//! Independent journal experiment. Timings include transaction and receipt, not only INSERT.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{error::Error, path::Path, time::Instant};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

enum Db {
    Sqlite(rusqlite::Connection),
    Turso(turso::Connection),
}
impl Db {
    async fn open(engine: &str, path: &Path) -> Result<Self> {
        let db = match engine {
            "sqlite" => Self::Sqlite(rusqlite::Connection::open(path)?),
            "turso" => Self::Turso(
                turso::Builder::new_local(path.to_str().ok_or("path")?)
                    .build()
                    .await?
                    .connect()?,
            ),
            _ => return Err("unknown engine".into()),
        };
        for sql in [
            "PRAGMA journal_mode=WAL",
            "PRAGMA synchronous=FULL",
            "PRAGMA foreign_keys=ON",
        ] {
            db.exec(sql, &[]).await?;
        }
        Ok(db)
    }
    async fn exec(&self, sql: &str, args: &[String]) -> Result<()> {
        match self {
            Self::Sqlite(c) => {
                let mut s = c.prepare(sql)?;
                let mut r = s.query(rusqlite::params_from_iter(args))?;
                while r.next()?.is_some() {}
            }
            Self::Turso(c) => {
                let mut r = c.query(sql, args.to_vec()).await?;
                while r.next().await?.is_some() {}
            }
        }
        Ok(())
    }
    async fn rows(&self, sql: &str, args: &[String]) -> Result<Vec<Vec<String>>> {
        let mut out = Vec::new();
        match self {
            Self::Sqlite(c) => {
                let mut s = c.prepare(sql)?;
                let count = s.column_count();
                let mut rows = s.query(rusqlite::params_from_iter(args))?;
                while let Some(row) = rows.next()? {
                    let mut cells = Vec::new();
                    for n in 0..count {
                        use rusqlite::types::ValueRef;
                        cells.push(match row.get_ref(n)? {
                            ValueRef::Text(v) => String::from_utf8(v.to_vec())?,
                            ValueRef::Integer(v) => v.to_string(),
                            ValueRef::Real(v) => v.to_string(),
                            ValueRef::Null => "null".into(),
                            _ => return Err("unexpected blob".into()),
                        });
                    }
                    out.push(cells);
                }
            }
            Self::Turso(c) => {
                let mut rows = c.query(sql, args.to_vec()).await?;
                while let Some(row) = rows.next().await? {
                    let mut cells = Vec::new();
                    for n in 0..row.column_count() {
                        cells.push(match row.get_value(n)? {
                            turso::Value::Text(v) => v,
                            turso::Value::Integer(v) => v.to_string(),
                            turso::Value::Real(v) => v.to_string(),
                            turso::Value::Null => "null".into(),
                            _ => return Err("unexpected blob".into()),
                        });
                    }
                    out.push(cells);
                }
            }
        }
        Ok(out)
    }
    async fn schema(&self) -> Result<()> {
        self.exec("BEGIN IMMEDIATE", &[]).await?;
        for sql in [
            "CREATE TABLE IF NOT EXISTS state (id INTEGER PRIMARY KEY CHECK(id=1), revision INTEGER NOT NULL CHECK(revision>=0), projection TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS events (seq INTEGER PRIMARY KEY, payload TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS receipts (op TEXT PRIMARY KEY, request TEXT NOT NULL, revision INTEGER NOT NULL)",
            "INSERT OR IGNORE INTO state VALUES(1,0,'{}')",
            "PRAGMA user_version=1",
        ] {
            self.exec(sql, &[]).await?;
        }
        self.exec("COMMIT", &[]).await
    }
    async fn append(&self, op: &Operation, abort: bool) -> Result<u64> {
        self.exec("BEGIN IMMEDIATE", &[]).await?;
        let result = self.in_transaction(op, abort).await;
        match result {
            Ok(v) => {
                self.exec("COMMIT", &[]).await?;
                Ok(v)
            }
            Err(e) => {
                self.exec("ROLLBACK", &[]).await?;
                Err(e)
            }
        }
    }
    async fn in_transaction(&self, op: &Operation, abort: bool) -> Result<u64> {
        let request = serde_json::to_string(op)?;
        if let Some(row) = self
            .rows(
                "SELECT request,revision FROM receipts WHERE op=?1",
                std::slice::from_ref(&op.key),
            )
            .await?
            .first()
        {
            return if row[0] == request {
                Ok(row[1].parse()?)
            } else {
                Err("operation conflict".into())
            };
        }
        let revision: u64 = self
            .rows("SELECT revision FROM state WHERE id=1", &[])
            .await?[0][0]
            .parse()?;
        if op.expected != revision {
            return Err("stale revision".into());
        }
        for (n, event) in op.events.iter().enumerate() {
            self.exec(
                "INSERT INTO events VALUES(?1,?2)",
                &[(revision + n as u64 + 1).to_string(), event.clone()],
            )
            .await?;
        }
        let next = revision + op.events.len() as u64;
        self.exec(
            "UPDATE state SET revision=?1,projection=?2 WHERE id=1",
            &[next.to_string(), op.projection.clone()],
        )
        .await?;
        if abort {
            return Err("injected transaction abort".into());
        }
        self.exec(
            "INSERT INTO receipts VALUES(?1,?2,?3)",
            &[op.key.clone(), request, next.to_string()],
        )
        .await?;
        Ok(next)
    }
}
#[derive(Serialize, Deserialize, Clone)]
struct Operation {
    key: String,
    expected: u64,
    events: Vec<String>,
    projection: String,
}
fn operation(n: u64, batch: u64) -> Operation {
    Operation {key:format!("op-{n}"),expected:n*batch,events:(0..batch).map(|i|format!("{{\"observation\":{},\"answer\":\"move\",\"rubric\":\"ownership-v1\",\"thinking\":\"{}\"}}",n*batch+i,"x".repeat(160))).collect(),projection:format!("{{\"revision\":{}}}",(n+1)*batch)}
}
async fn conformance(db: &Db) -> Result<Vec<String>> {
    db.schema().await?;
    let a = operation(0, 2);
    assert_eq!(db.append(&a, false).await?, 2);
    assert_eq!(db.append(&a, false).await?, 2);
    let mut conflict = a.clone();
    conflict.events[0].push('x');
    assert!(db.append(&conflict, false).await.is_err());
    let mut stale = operation(1, 2);
    stale.expected = 0;
    assert!(db.append(&stale, false).await.is_err());
    let b = operation(1, 2);
    assert!(db.append(&b, true).await.is_err());
    assert_eq!(
        db.rows("SELECT COUNT(*) FROM events", &[]).await?[0][0],
        "2"
    );
    assert_eq!(db.append(&b, false).await?, 4);
    assert!(
        db.exec("INSERT INTO events VALUES(1,'duplicate')", &[])
            .await
            .is_err()
    );
    assert!(db.exec("UPDATE state SET revision=-1", &[]).await.is_err());
    db.exec("BEGIN IMMEDIATE", &[]).await?;
    db.exec("CREATE TABLE failed_migration(x TEXT)", &[])
        .await?;
    db.exec("PRAGMA user_version=2", &[]).await?;
    db.exec("ROLLBACK", &[]).await?;
    assert!(
        db.exec("SELECT * FROM failed_migration", &[])
            .await
            .is_err()
    );
    assert_eq!(db.rows("PRAGMA user_version", &[]).await?[0][0], "1");
    db.exec("CREATE TABLE parent(id INTEGER PRIMARY KEY)", &[])
        .await?;
    db.exec("CREATE TABLE child(id INTEGER REFERENCES parent(id))", &[])
        .await?;
    assert!(db.exec("INSERT INTO child VALUES(99)", &[]).await.is_err());
    let replay = db
        .rows("SELECT payload FROM events ORDER BY seq", &[])
        .await?;
    assert_eq!(replay.len(), 4);
    Ok(vec![
        "atomic append".into(),
        "duplicate retry before CAS".into(),
        "payload conflict".into(),
        "stale revision".into(),
        "rollback".into(),
        "unique/check/FK constraints".into(),
        "aborted migration".into(),
        "ordered replay".into(),
    ])
}
fn percentile(xs: &mut [f64], p: f64) -> f64 {
    xs.sort_by(f64::total_cmp);
    xs[((xs.len() - 1) as f64 * p).round() as usize]
}
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).ok_or("mode")?;
    let engine = args.get(2).ok_or("engine")?;
    let path = Path::new(args.get(3).ok_or("path")?);
    let db = Db::open(engine, path).await?;
    match mode.as_str() {
        "check" => println!(
            "{}",
            serde_json::json!({"engine":engine,"passed":conformance(&db).await?,"version":db.rows("SELECT sqlite_version()",&[]).await?,"journal":db.rows("PRAGMA journal_mode",&[]).await?,"sync":db.rows("PRAGMA synchronous",&[]).await?})
        ),
        "bench" => {
            db.schema().await?;
            let count: u64 = args.get(4).ok_or("event count")?.parse()?;
            let batch: u64 = args.get(5).map(String::as_str).unwrap_or("10").parse()?;
            let mut times = Vec::new();
            let mut hash = Sha256::new();
            let start = Instant::now();
            for n in 0..count / batch {
                let op = operation(n, batch);
                hash.update(serde_json::to_vec(&op)?);
                let t = Instant::now();
                db.append(&op, false).await?;
                times.push(t.elapsed().as_secs_f64() * 1000.0);
            }
            let append_ms = start.elapsed().as_secs_f64() * 1000.;
            let t = Instant::now();
            let rows = db
                .rows("SELECT payload FROM events ORDER BY seq", &[])
                .await?;
            let replay_ms = t.elapsed().as_secs_f64() * 1000.;
            assert_eq!(rows.len() as u64, count);
            println!(
                "{}",
                serde_json::json!({"engine":engine,"events":count,"batch":batch,"samples":times.len(),"fixture_sha256":format!("{:x}",hash.finalize()),"append_total_ms":append_ms,"append_p50_ms":percentile(&mut times,0.5),"append_p95_ms":percentile(&mut times,0.95),"replay_ms":replay_ms,"samples_ms":times})
            );
        }
        "child" => {
            db.schema().await?;
            if args.get(4).map(String::as_str) == Some("uncommitted") {
                db.exec("BEGIN IMMEDIATE", &[]).await?;
                db.in_transaction(&operation(0, 2), false).await?;
                println!("UNCOMMITTED");
            } else {
                db.append(&operation(0, 2), false).await?;
                println!("ACK");
            }
            use std::io::Write;
            std::io::stdout().flush()?;
            std::future::pending::<()>().await;
        }
        "reopen" => println!(
            "{}",
            serde_json::json!({"events":db.rows("SELECT COUNT(*) FROM events",&[]).await?,"revision":db.rows("SELECT revision FROM state",&[]).await?,"receipts":db.rows("SELECT COUNT(*) FROM receipts",&[]).await?})
        ),
        _ => return Err("unknown mode".into()),
    }
    Ok(())
}
