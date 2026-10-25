// Fault injection and multi-tab applicability. Uses persistent profiles on the repo filesystem.
import { chromium, firefox } from "@playwright/test";
import { createServer } from "node:http";
import { readFile, stat, mkdir, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { join, extname, resolve } from "node:path";
import { spawn } from "node:child_process";
const root = fileURLToPath(new URL(".", import.meta.url));
const server = createServer(async (req, res) => {
  try {
    const path = resolve(root, "." + decodeURIComponent(req.url.split("?")[0]));
    if (path !== resolve(root) && !path.startsWith(root))
      throw Error("outside root");
    const file = (await stat(path)).isDirectory()
      ? join(path, "index.html")
      : path;
    res.writeHead(200, {
      "Content-Type":
        {
          ".html": "text/html",
          ".js": "text/javascript",
          ".wasm": "application/wasm",
        }[extname(file)] || "application/octet-stream",
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp",
    });
    res.end(await readFile(file));
  } catch {
    res.writeHead(404);
    res.end();
  }
});
await new Promise((r) => server.listen(4174, "127.0.0.1", r));
const origin = "http://127.0.0.1:4174";
const results = {
  at: new Date().toISOString(),
  conditions:
    "Persistent profiles on repository btrfs filesystem; process kill is SIGKILL to owned Chrome process group; not a power-cut test.",
  tests: [],
  failures: [],
};
const contexts = [];
const ownedProcesses = [];
const output = resolve(
  root,
  "../../../docs/experiments/results/storage-browser-lifecycle.json",
);
const save = () => writeFile(output, JSON.stringify(results, null, 2) + "\n");
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const assert = (yes, message) => {
  if (!yes) throw Error(message);
};
const pageReady = async (context) => {
  const page = await context.newPage();
  await page.goto(origin);
  await page.waitForFunction(() => window.ready);
  return page;
};
try {
  for (const kind of ["chromium", "firefox"]) {
    const profile = resolve(
      root,
      `../../../.local/lifecycle-${kind}-${Date.now()}`,
    );
    const context = await (
      kind === "chromium" ? chromium : firefox
    ).launchPersistentContext(
      profile,
      kind === "chromium"
        ? { executablePath: "/usr/bin/google-chrome", headless: true }
        : { headless: true },
    );
    contexts.push(context);
    const page = await pageReady(context),
      second = await pageReady(context);
    for (const engine of ["idb", "turso"]) {
      const name = `${engine}-tabs-${Date.now()}`;
      try {
        await page.evaluate(
          async ({ engine, name }) => {
            window.db = await window.storageOpen(engine, name);
          },
          { engine, name },
        );
        await second.evaluate(
          async ({ engine, name }) => {
            window.db = await window.storageOpen(engine, name);
          },
          { engine, name },
        );
        const outcomes = await Promise.all([
          page.evaluate(async () => {
            try {
              return {
                revision: await window.db.append({
                  ...window.operation(0),
                  key: "tab-a",
                }),
              };
            } catch (e) {
              return { error: String(e) };
            }
          }),
          second.evaluate(async () => {
            try {
              return {
                revision: await window.db.append({
                  ...window.operation(0),
                  key: "tab-b",
                }),
              };
            } catch (e) {
              return { error: String(e) };
            }
          }),
        ]);
        assert(
          outcomes.filter((x) => x.revision === 2).length === 1,
          "exactly one writer must commit",
        );
        assert(
          outcomes.filter((x) => x.error?.includes("stale revision")).length ===
            1,
          "other writer must see stale revision",
        );
        const events = await page.evaluate(() => window.db.events());
        assert(events.length === 2, "lost update");
        results.tests.push({
          browser: kind,
          engine,
          gate: "two-tab concurrent CAS",
          outcomes,
        });
      } catch (e) {
        results.failures.push({
          browser: kind,
          engine,
          gate: "two-tab concurrent CAS",
          error: String(e),
        });
      }
      await page.evaluate(async () => {
        if (window.db) {
          await window.db.close();
          window.db = null;
        }
      });
      await second.evaluate(async () => {
        if (window.db) {
          await window.db.close();
          window.db = null;
        }
      });
      await save();
    }
    // Fault the Rust adapter's migration lifecycle through the public browser API.
    const migrations = await page.evaluate(async () => {
      const name = `migration-${crypto.randomUUID()}`,
        journal = await window.storageOpen("idb", name);
      await journal.append(window.operation(0));
      await journal.close();
      const req = indexedDB.open(name, 2);
      req.onupgradeneeded = () => {
        req.result.createObjectStore("must-not-survive");
        req.transaction.abort();
      };
      await new Promise((resolve, reject) => {
        req.onerror = () => resolve();
        req.onsuccess = () => reject(Error("aborted migration succeeded"));
      });
      const reopened = await window.storageOpen("idb", name);
      const before = await reopened.events();
      await reopened.close();
      if (before.length !== 2) throw Error("migration lost events");
      const hold = await new Promise((resolve, reject) => {
        const r = indexedDB.open(name, 1);
        r.onsuccess = () => resolve(r.result);
        r.onerror = () => reject(r.error);
      });
      let blocked = false;
      const upgrade = indexedDB.open(name, 2);
      upgrade.onblocked = () => {
        blocked = true;
        hold.close();
      };
      upgrade.onupgradeneeded = () =>
        upgrade.result.createObjectStore("future");
      const future = await new Promise((resolve, reject) => {
        upgrade.onsuccess = () => resolve(upgrade.result);
        upgrade.onerror = () => reject(upgrade.error);
      });
      future.close();
      let rejected = false;
      try {
        await window.storageOpen("idb", name);
      } catch (e) {
        rejected = String(e).includes("VersionError");
      }
      if (!blocked || !rejected)
        throw Error("blocked/unsupported version gate");
      return {
        abortedMigrationPreservedEvents: before.length,
        blockedUpgradeObserved: blocked,
        futureVersionRejected: rejected,
      };
    });
    results.tests.push({
      browser: kind,
      engine: "idb",
      gate: "migration/blocked/future version",
      ...migrations,
    });
    await save();
    if (kind === "chromium") {
      const quotaContext = await chromium.launchPersistentContext(
        resolve(root, `../../../.local/quota-${Date.now()}`),
        { executablePath: "/usr/bin/google-chrome", headless: true },
      );
      contexts.push(quotaContext);
      const quotaPage = await pageReady(quotaContext);
      const protocolSession = await quotaContext.newCDPSession(quotaPage);
      // Official Chrome DevTools Storage.overrideQuotaForOrigin contract inspected before use.
      await protocolSession.send("Storage.overrideQuotaForOrigin", {
        origin,
        quotaSize: 1024,
      });
      const quotaOverride = await protocolSession.send(
        "Storage.getUsageAndQuota",
        { origin },
      );
      const quota = await quotaPage.evaluate(async () => {
        let acknowledged = false,
          error = "";
        const db = await window.storageOpen(
          "idb",
          `quota-${crypto.randomUUID()}`,
        );
        const events = Array.from({ length: 100 }, () =>
          btoa(
            String.fromCharCode(
              ...crypto.getRandomValues(new Uint8Array(45000)),
            ),
          ),
        );
        try {
          await db.append({ ...window.operation(0, 100), events });
          acknowledged = true;
        } catch (e) {
          error = String(e);
        }
        const state = await db.state(),
          persisted = await db.events();
        await db.close();
        return {
          acknowledged,
          error,
          revision: state.revision,
          events: persisted.length,
        };
      });
      assert(
        !quota.acknowledged &&
          quota.error.includes("QuotaExceededError") &&
          quota.revision === 0 &&
          quota.events === 0,
        "quota must roll back, without ACK: " + JSON.stringify(quota),
      );
      results.tests.push({
        browser: kind,
        engine: "idb",
        gate: "quota failure",
        quotaOverride,
        ...quota,
      });
      await protocolSession.send("Storage.overrideQuotaForOrigin", { origin });
      await quotaContext.close();
      // Kill the candidate's I/O workers; a bounded rejection is needed to recover a draft.
      const workerPage = await context.newPage();
      await workerPage.addInitScript(() => {
        const Original = window.Worker;
        window.workers = [];
        window.Worker = class extends Original {
          constructor(...args) {
            super(...args);
            window.workers.push(this);
          }
        };
      });
      await workerPage.goto(origin);
      await workerPage.waitForFunction(() => window.ready);
      const worker = await workerPage.evaluate(async () => {
        const db = await window.storageOpen(
          "turso",
          `worker-${crypto.randomUUID()}`,
        );
        const workers = window.workers.length;
        window.workers.forEach((w) => w.terminate());
        return Promise.race([
          db.append(window.operation(0)).then(
            () => ({ workers, acknowledged: true }),
            (e) => ({ workers, acknowledged: false, error: String(e) }),
          ),
          new Promise((resolve) =>
            setTimeout(
              () => resolve({ workers, acknowledged: false, timeout: true }),
              3000,
            ),
          ),
        ]);
      });
      (worker.timeout || worker.acknowledged
        ? results.failures
        : results.tests
      ).push({
        browser: kind,
        engine: "turso",
        gate: "worker death bounded failure",
        ...worker,
      });
      await save();
    }
    await context.close();
  }
  // Launch Chrome ourselves to identify exactly the process group being killed.
  for (const engine of ["idb", "turso"]) {
    for (let schedule = 0; schedule < 3; schedule++) {
      const profile = resolve(
        root,
        `../../../.local/crash-${engine}-${Date.now()}`,
      );
      await mkdir(profile, { recursive: true });
      const start = async () => {
        const process = spawn(
          "/usr/bin/google-chrome",
          [
            "--headless=new",
            "--no-first-run",
            "--no-default-browser-check",
            "--disable-background-networking",
            "--remote-debugging-port=0",
            `--user-data-dir=${profile}`,
            "about:blank",
          ],
          { detached: true, stdio: "ignore" },
        );
        ownedProcesses.push(process);
        for (let i = 0; i < 100; i++) {
          try {
            const port = (
              await readFile(join(profile, "DevToolsActivePort"), "utf8")
            ).split("\n")[0];
            const endpoint = `http://127.0.0.1:${port}`;
            const response = await fetch(endpoint + "/json/version");
            if (response.ok) {
              const browser = await chromium.connectOverCDP(endpoint);
              return { process, browser, context: browser.contexts()[0] };
            }
          } catch {}
          await sleep(100);
        }
        throw Error("Chrome debug port not ready");
      };
      const first = await start();
      const page = await pageReady(first.context);
      const acknowledged = await page.evaluate(async (engine) => {
        const db = await window.storageOpen(engine, "crash-journal");
        window.crashDb = db;
        return db.append(window.operation(0));
      }, engine);
      assert(acknowledged === 2, "save ACK before kill");
      process.kill(-first.process.pid, "SIGKILL");
      await new Promise((r) => first.process.once("exit", r));
      await sleep(200);
      const second = await start();
      const reopenedPage = await pageReady(second.context);
      const restored = await reopenedPage.evaluate(async (engine) => {
        const db = await window.storageOpen(engine, "crash-journal");
        const events = await db.events(),
          state = await db.state(),
          retry = await db.append(window.operation(0));
        await db.close();
        return { events, state, retry };
      }, engine);
      assert(
        restored.events.length === 2 &&
          restored.state.revision === 2 &&
          restored.retry === 2,
        "acknowledged evidence lost or duplicated after kill",
      );
      results.tests.push({
        browser: "chromium",
        engine,
        gate: "SIGKILL-after-ACK",
        schedule,
        events: restored.events.length,
        revision: restored.state.revision,
        duplicateRetry: restored.retry,
      });
      await second.browser.close();
      await save();
    }
  }
} catch (e) {
  results.failures.push({ gate: "harness", error: String(e) });
  console.error(e);
} finally {
  for (const context of contexts) await context.close().catch(() => {});
  for (const child of ownedProcesses) {
    try {
      process.kill(-child.pid, "SIGKILL");
    } catch {}
  }
  await save();
  server.close();
  console.log(
    JSON.stringify(
      { tests: results.tests.length, failures: results.failures },
      null,
      2,
    ),
  );
}

// Candidate applicability failures are reportable outcomes; baseline/harness failures fail the run.
if (results.failures.some((f) => f.engine !== "turso")) process.exitCode = 1;
