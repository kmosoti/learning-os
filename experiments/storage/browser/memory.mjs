// Snapshot RSS is summed per browser process (shared pages may be counted twice), not PSS or peak.
import { chromium } from "@playwright/test";
import { createServer } from "node:http";
import { readFile, stat, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { join, extname, resolve } from "node:path";
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
const results = {
  at: new Date().toISOString(),
  boundary:
    "Persistent Chrome profile, fresh browser per engine. Snapshot after 10k events plus replay. Sum of process RSS, not peak/PSS; shared memory may double count. This probe does not contribute latency samples.",
  runs: [],
};
try {
  for (const engine of ["idb", "turso"]) {
    const context = await chromium.launchPersistentContext(
      resolve(root, `../../../.local/memory-${engine}-${Date.now()}`),
      { executablePath: "/usr/bin/google-chrome", headless: true },
    );
    try {
      const page = await context.newPage();
      await page.goto("http://127.0.0.1:4174");
      await page.waitForFunction(() => window.ready);
      const cdp = await context.browser().newBrowserCDPSession();
      const snapshot = async () => {
        const { processInfo } = await cdp.send("SystemInfo.getProcessInfo");
        const processes = [];
        for (const p of processInfo) {
          try {
            const status = await readFile(`/proc/${p.id}/status`, "utf8");
            processes.push({
              type: p.type,
              rss_kib: Number(status.match(/^VmRSS:\s+(\d+)/m)?.[1] || 0),
            });
          } catch {}
        }
        return {
          processes,
          total_rss_kib: processes.reduce((sum, p) => sum + p.rss_kib, 0),
        };
      };
      const before = await snapshot();
      await page.evaluate((engine) => window.bench(engine, 10000), engine);
      const after = await snapshot();
      results.runs.push({
        engine,
        browser: await page.evaluate(() => navigator.userAgent),
        before,
        after,
      });
    } finally {
      await context.close();
    }
  }
} finally {
  server.close();
  await writeFile(
    resolve(
      root,
      "../../../docs/experiments/results/storage-browser-memory.json",
    ),
    JSON.stringify(results, null, 2) + "\n",
  );
  console.log(JSON.stringify(results, null, 2));
}
