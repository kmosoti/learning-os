//! Native conformance adapter. The running personal PWA never sends its evidence here.
use learningos_application::*;
use learningos_domain::*;
use rusqlite::{Connection, OptionalExtension, params};
use std::{path::Path, sync::Mutex};
pub struct SqliteJournal {
    connection: Mutex<Connection>,
}
fn storage(e: impl ToString) -> AppError {
    AppError::Storage(e.to_string())
}
impl SqliteJournal {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let mut c = Connection::open(path).map_err(storage)?;
        c.pragma_update(None, "foreign_keys", "ON")
            .map_err(storage)?;
        c.pragma_update(None, "journal_mode", "WAL")
            .map_err(storage)?;
        c.pragma_update(None, "synchronous", "FULL")
            .map_err(storage)?;
        let version: u32 = c
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .map_err(storage)?;
        if version > 1 {
            return Err(storage(
                "journal schema is newer than this application; export with the newer version",
            ));
        }
        let tx = c.transaction().map_err(storage)?;
        tx.execute_batch("CREATE TABLE IF NOT EXISTS operations(id TEXT PRIMARY KEY NOT NULL, request TEXT NOT NULL, revision INTEGER NOT NULL CHECK(revision>0)); CREATE TABLE IF NOT EXISTS events(sequence INTEGER PRIMARY KEY CHECK(sequence>0), id TEXT UNIQUE NOT NULL, operation TEXT NOT NULL REFERENCES operations(id), payload TEXT NOT NULL); CREATE TABLE IF NOT EXISTS snapshot(id INTEGER PRIMARY KEY CHECK(id=1),revision INTEGER NOT NULL CHECK(revision>=0),payload TEXT NOT NULL); PRAGMA user_version=1;").map_err(storage)?;
        tx.commit().map_err(storage)?;
        let journal = Self {
            connection: Mutex::new(c),
        };
        let backup = journal.backup_sync()?;
        let state = backup.validate()?;
        let c = journal.connection.lock().map_err(storage)?;
        c.execute("INSERT INTO snapshot(id,revision,payload) VALUES(1,?1,?2) ON CONFLICT(id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload",params![state.revision as u32,serde_json::to_string(&state).map_err(storage)?]).map_err(storage)?;
        drop(c);
        Ok(journal)
    }
    fn backup_sync(&self) -> Result<Backup, AppError> {
        let mut connection = self.connection.lock().map_err(storage)?;
        let c = connection.transaction().map_err(storage)?;
        let events: Vec<Event> = c
            .prepare("SELECT payload FROM events ORDER BY sequence")
            .map_err(storage)?
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(storage)?
            .map(|r| serde_json::from_str(&r.map_err(storage)?).map_err(storage))
            .collect::<Result<_, _>>()?;
        let receipts: Vec<Receipt> = c
            .prepare("SELECT request,revision FROM operations ORDER BY revision")
            .map_err(storage)?
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, u64::from(r.get::<_, u32>(1)?)))
            })
            .map_err(storage)?
            .map(|r| {
                let (request, revision) = r.map_err(storage)?;
                Ok(Receipt {
                    request: serde_json::from_str(&request).map_err(storage)?,
                    revision,
                })
            })
            .collect::<Result<_, AppError>>()?;
        c.commit().map_err(storage)?;
        let event_digest = digest(&serde_json::to_string(&events).map_err(storage)?);
        Ok(Backup {
            schema: 1,
            fixture: FIXTURE_VERSION.into(),
            fixture_digest: fixture_digest(),
            events,
            receipts,
            event_digest,
        })
    }
    pub fn backup(&self) -> Result<Backup, AppError> {
        let backup = self.backup_sync()?;
        backup.validate()?;
        Ok(backup)
    }
    pub fn import(&self, backup: &Backup) -> Result<(), AppError> {
        let state = backup.validate()?;
        let mut c = self.connection.lock().map_err(storage)?;
        let tx = c
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(storage)?;
        let count: u32 = tx
            .query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))
            .map_err(storage)?;
        if count != 0 {
            return Err(AppError::Backup("import requires an empty journal".into()));
        }
        for receipt in &backup.receipts {
            tx.execute(
                "INSERT INTO operations VALUES(?1,?2,?3)",
                params![
                    receipt.request.id,
                    serde_json::to_string(&receipt.request).map_err(storage)?,
                    receipt.revision as u32
                ],
            )
            .map_err(storage)?;
        }
        for (index, event) in backup.events.iter().enumerate() {
            let op = backup
                .receipts
                .iter()
                .find(|r| r.revision > index as u64)
                .ok_or_else(|| storage("unbound event"))?;
            tx.execute(
                "INSERT INTO events VALUES(?1,?2,?3,?4)",
                params![
                    index as u32 + 1,
                    event.id,
                    op.request.id,
                    serde_json::to_string(event).map_err(storage)?
                ],
            )
            .map_err(storage)?;
        }
        tx.execute("INSERT INTO snapshot VALUES(1,?1,?2) ON CONFLICT(id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload",params![state.revision as u32,serde_json::to_string(&state).map_err(storage)?]).map_err(storage)?;
        tx.commit().map_err(storage)
    }
}
impl Journal for SqliteJournal {
    async fn events(&self) -> Result<Vec<Event>, AppError> {
        Ok(self.backup_sync()?.events)
    }
    async fn session(&self) -> Result<Session, AppError> {
        let c = self.connection.lock().map_err(storage)?;
        let text: String = c
            .query_row("SELECT payload FROM snapshot WHERE id=1", [], |r| r.get(0))
            .map_err(storage)?;
        serde_json::from_str(&text).map_err(storage)
    }
    async fn receipt(&self, id: &str) -> Result<Option<Receipt>, AppError> {
        let c = self.connection.lock().map_err(storage)?;
        let row = c
            .query_row(
                "SELECT request,revision FROM operations WHERE id=?1",
                [id],
                |r| Ok((r.get::<_, String>(0)?, u64::from(r.get::<_, u32>(1)?))),
            )
            .optional()
            .map_err(storage)?;
        row.map(|(request, revision)| {
            Ok(Receipt {
                request: serde_json::from_str(&request).map_err(storage)?,
                revision,
            })
        })
        .transpose()
    }
    async fn commit(&self, commit: &Commit) -> Result<Receipt, AppError> {
        let mut c = self.connection.lock().map_err(storage)?;
        let tx = c
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(storage)?;
        let duplicate = tx
            .query_row(
                "SELECT request,revision FROM operations WHERE id=?1",
                [&commit.request.id],
                |r| Ok((r.get::<_, String>(0)?, u64::from(r.get::<_, u32>(1)?))),
            )
            .optional()
            .map_err(storage)?;
        if let Some((json, revision)) = duplicate {
            let request: Request = serde_json::from_str(&json).map_err(storage)?;
            return if request == commit.request {
                Ok(Receipt { request, revision })
            } else {
                Err(AppError::Conflict)
            };
        }
        let (revision, text): (u32, String) = tx
            .query_row(
                "SELECT revision,payload FROM snapshot WHERE id=1",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .map_err(storage)?;
        if u64::from(revision) != commit.request.expected {
            return Err(AppError::Stale);
        }
        let state: Session = serde_json::from_str(&text).map_err(storage)?;
        let expected = decide(
            &state,
            &commit.request.command,
            &commit.request.id,
            commit.request.at_ms,
        )?;
        if expected != commit.events {
            return Err(storage("unbound commit"));
        }
        let mut next = state;
        for event in &expected {
            next = apply(&next, event)?;
        }
        if next != commit.projection
            || commit.receipt.request != commit.request
            || commit.receipt.revision != next.revision
        {
            return Err(storage("invalid commit projection/receipt"));
        }
        tx.execute(
            "INSERT INTO operations VALUES(?1,?2,?3)",
            params![
                commit.request.id,
                serde_json::to_string(&commit.request).map_err(storage)?,
                next.revision as u32
            ],
        )
        .map_err(storage)?;
        for (index, event) in commit.events.iter().enumerate() {
            tx.execute(
                "INSERT INTO events VALUES(?1,?2,?3,?4)",
                params![
                    revision + index as u32 + 1,
                    event.id,
                    commit.request.id,
                    serde_json::to_string(event).map_err(storage)?
                ],
            )
            .map_err(storage)?;
        }
        tx.execute(
            "UPDATE snapshot SET revision=?1,payload=?2 WHERE id=1",
            params![
                next.revision as u32,
                serde_json::to_string(&next).map_err(storage)?
            ],
        )
        .map_err(storage)?;
        tx.commit().map_err(storage)?;
        Ok(commit.receipt.clone())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn req(id: &str, expected: u64, command: Command) -> Request {
        Request {
            schema: 1,
            id: id.into(),
            expected,
            at_ms: 100,
            command,
        }
    }
    #[tokio::test]
    async fn receipt_lookup_precedes_cas_and_export_import_preserves_retry() {
        let db = SqliteJournal::open(":memory:").unwrap();
        let r = req(
            "goal",
            0,
            Command::ChooseGoal {
                goal: "Understand borrowing".into(),
            },
        );
        let receipt = execute(&db, r.clone()).await.unwrap();
        execute(&db, req("plan", 1, Command::ConfirmPlan))
            .await
            .unwrap();
        assert_eq!(execute(&db, r.clone()).await.unwrap(), receipt);
        let mut conflicting = r.clone();
        conflicting.at_ms += 1;
        assert_eq!(
            execute(&db, conflicting).await.unwrap_err(),
            AppError::Conflict
        );
        assert_eq!(
            execute(&db, req("stale", 0, Command::ConfirmPlan))
                .await
                .unwrap_err(),
            AppError::Stale
        );
        let backup = db.backup().unwrap();
        let copy = SqliteJournal::open(":memory:").unwrap();
        copy.import(&backup).unwrap();
        assert_eq!(copy.backup().unwrap(), backup);
        assert_eq!(execute(&copy, r).await.unwrap(), receipt);
    }
    #[tokio::test]
    async fn concurrent_connections_cannot_both_use_one_revision() {
        let path =
            std::env::temp_dir().join(format!("learningos-cas-{}.sqlite3", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let one = SqliteJournal::open(&path).unwrap();
        let two = SqliteJournal::open(&path).unwrap();
        execute(
            &one,
            req(
                "one",
                0,
                Command::ChooseGoal {
                    goal: "Rust".into(),
                },
            ),
        )
        .await
        .unwrap();
        assert_eq!(
            execute(
                &two,
                req(
                    "two",
                    0,
                    Command::ChooseGoal {
                        goal: "Rust".into()
                    }
                )
            )
            .await
            .unwrap_err(),
            AppError::Stale
        );
        drop((one, two));
        let reopened = SqliteJournal::open(&path).unwrap();
        assert_eq!(reopened.session().await.unwrap().revision, 1);
        drop(reopened);
        let _ = std::fs::remove_file(&path);
    }
    #[tokio::test]
    async fn invalid_batches_leave_no_receipt_or_event() {
        let db = SqliteJournal::open(":memory:").unwrap();
        assert!(
            execute(&db, req("closed", 0, Command::OpenCheck))
                .await
                .is_err()
        );
        assert!(db.backup().unwrap().events.is_empty());
        assert!(db.receipt("closed").await.unwrap().is_none());
    }
    #[tokio::test]
    async fn mid_transaction_failure_rolls_back_operation_event_and_snapshot() {
        let db = SqliteJournal::open(":memory:").unwrap();
        db.connection.lock().unwrap().execute_batch("CREATE TRIGGER fail_event BEFORE INSERT ON events BEGIN SELECT RAISE(ABORT,'injected event failure'); END;").unwrap();
        assert!(
            execute(
                &db,
                req(
                    "aborted",
                    0,
                    Command::ChooseGoal {
                        goal: "Rust".into()
                    }
                )
            )
            .await
            .is_err()
        );
        assert_eq!(db.session().await.unwrap().revision, 0);
        assert!(db.receipt("aborted").await.unwrap().is_none());
        assert!(db.backup().unwrap().events.is_empty());
    }
}
