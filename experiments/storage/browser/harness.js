// Measurement orchestrator only. The stable browser adapter is implemented in Rust/WASM.
import init, {
  BrowserJournal,
} from "./pkg/learningos_browser_journal_experiment.js";
await init();
const assert = (yes, message) => {
  if (!yes) throw new Error(message);
};
const op = (n, batch = 2) => ({
  key: `op-${n}`,
  expected: n * batch,
  events: Array.from(
    { length: batch },
    (_, i) =>
      `{"observation":${n * batch + i},"answer":"move","rubric":"ownership-v1","thinking":"${"x".repeat(160)}"}`,
  ),
  projection: `{"revision":${(n + 1) * batch}}`,
});
async function open(engine, name) {
  if (engine === "idb") {
    const db = await BrowserJournal.open(name);
    return {
      append: async (o, abort = false) =>
        Number(await db.append(JSON.stringify(o), abort)),
      state: async () => JSON.parse(await db.state()),
      events: async () => JSON.parse(await db.events()),
      close: () => db.close(),
    };
  }
  const { connect } = await import(
    "./node_modules/@tursodatabase/database-wasm/bundle/main.es.js"
  );
  const db = await connect(name + ".db");
  await db.exec(
    "PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; PRAGMA foreign_keys=ON",
  );
  await db.exec(
    "CREATE TABLE IF NOT EXISTS state(id INTEGER PRIMARY KEY CHECK(id=1),revision INTEGER NOT NULL CHECK(revision>=0),projection TEXT NOT NULL); CREATE TABLE IF NOT EXISTS events(seq INTEGER PRIMARY KEY,payload TEXT NOT NULL); CREATE TABLE IF NOT EXISTS receipts(op TEXT PRIMARY KEY,request TEXT NOT NULL,revision INTEGER NOT NULL); INSERT OR IGNORE INTO state VALUES(1,0,'{}'); PRAGMA user_version=1",
  );
  return {
    append: async (o, abort = false) => {
      await db.exec("BEGIN IMMEDIATE");
      try {
        const canonical = JSON.stringify(o),
          duplicate = await db
            .prepare("SELECT request,revision FROM receipts WHERE op=?1")
            .get(o.key);
        if (duplicate) {
          if (duplicate.request !== canonical)
            throw new Error("operation conflict");
          await db.exec("COMMIT");
          return duplicate.revision;
        }
        const { revision } = await db
          .prepare("SELECT revision FROM state WHERE id=1")
          .get();
        if (revision !== o.expected) throw new Error("stale revision");
        const insert = db.prepare("INSERT INTO events VALUES(?1,?2)");
        for (let i = 0; i < o.events.length; i++)
          await insert.run(revision + i + 1, o.events[i]);
        const next = revision + o.events.length;
        await db
          .prepare("UPDATE state SET revision=?1,projection=?2 WHERE id=1")
          .run(next, o.projection);
        if (abort) throw new Error("injected transaction abort");
        await db
          .prepare("INSERT INTO receipts VALUES(?1,?2,?3)")
          .run(o.key, canonical, next);
        await db.exec("COMMIT");
        return next;
      } catch (e) {
        await db.exec("ROLLBACK");
        throw e;
      }
    },
    state: () =>
      db.prepare("SELECT revision,projection FROM state WHERE id=1").get(),
    events: async () =>
      (await db.prepare("SELECT payload FROM events ORDER BY seq").all()).map(
        (x) => x.payload,
      ),
    close: () => db.close(),
  };
}
window.storageOpen = open;
window.operation = op;
window.check = async (engine) => {
  const name = `${engine}-check-${crypto.randomUUID()}`,
    db = await open(engine, name),
    passed = [];
  const reject = async (f, label) => {
    let rejected = false;
    try {
      await f();
    } catch {
      rejected = true;
    }
    assert(rejected, label);
    passed.push(label);
  };
  assert((await db.append(op(0))) === 2, "append");
  passed.push("atomic append");
  assert((await db.append(op(0))) === 2, "retry");
  passed.push("retry before CAS");
  await reject(
    () => db.append({ ...op(0), projection: "different" }),
    "payload conflict",
  );
  await reject(() => db.append({ ...op(1), expected: 0 }), "stale revision");
  await reject(() => db.append(op(1), true), "abort");
  assert(
    JSON.stringify(await db.events()) === JSON.stringify(op(0).events),
    "rollback/replay mismatch",
  );
  assert((await db.state()).revision === 2, "projection changed on abort");
  passed.push("rollback");
  await db.close();
  const reopened = await open(engine, name);
  assert((await reopened.state()).revision === 2, "reopen");
  assert((await reopened.append(op(0))) === 2, "reopened retry");
  passed.push("reopen and lost ACK");
  await reopened.close();
  return passed;
};
window.bench = async (engine, count, batch = 10) => {
  const name = `${engine}-bench-${crypto.randomUUID()}`,
    init = performance.now(),
    db = await open(engine, name),
    open_ms = performance.now() - init,
    times = [],
    fixture = [];
  const start = performance.now();
  for (let n = 0; n < count / batch; n++) {
    const o = op(n, batch);
    fixture.push(JSON.stringify(o));
    const t = performance.now();
    await db.append(o);
    times.push(performance.now() - t);
  }
  const total = performance.now() - start,
    t = performance.now(),
    events = await db.events(),
    replay_ms = performance.now() - t;
  assert(events.length === count, "replay count");
  await db.close();
  const hash = Array.from(
    new Uint8Array(
      await crypto.subtle.digest(
        "SHA-256",
        new TextEncoder().encode(fixture.join("")),
      ),
    ),
  )
    .map((x) => x.toString(16).padStart(2, "0"))
    .join("");
  const ordered = [...times].sort((a, b) => a - b),
    p = (q) => ordered[Math.round((times.length - 1) * q)];
  return {
    engine,
    events: count,
    batch,
    samples: times.length,
    fixture_sha256: hash,
    open_ms,
    append_total_ms: total,
    append_p50_ms: p(0.5),
    append_p95_ms: p(0.95),
    replay_ms,
    samples_ms: times,
  };
};
window.ready = true;
