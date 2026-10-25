import { chromium, firefox } from "@playwright/test";
import { createServer } from "node:http";
import { readFile, stat, mkdir, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { join, extname, resolve } from "node:path";
import os from "node:os";
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
  cpu: os.cpus()[0].model,
  os: os.release(),
  memory: os.totalmem(),
  runs: [],
  failures: [],
};
const output = resolve(
  root,
  process.env.STORAGE_CHECK_ONLY === "1"
    ? "../../../docs/experiments/results/storage-browser-conformance.json"
    : "../../../docs/experiments/results/storage-browser.json",
);
await mkdir(resolve(output, ".."), { recursive: true });
try {
  for (const kind of ["chromium", "firefox"]) {
    for (const engine of ["idb", "turso"]) {
      let context;
      const profile = resolve(
        root,
        `../../../.local/storage-${kind}-${engine}-${Date.now()}`,
      );
      try {
        context = await (
          kind === "chromium" ? chromium : firefox
        ).launchPersistentContext(
          profile,
          kind === "chromium"
            ? { executablePath: "/usr/bin/google-chrome", headless: true }
            : { headless: true },
        );
      } catch (e) {
        results.failures.push({
          browser: kind,
          gate: "launch",
          error: String(e),
        });
        continue;
      }
      const page = await context.newPage();
      page.setDefaultTimeout(180000);
      const errors = [];
      page.on("pageerror", (e) => errors.push(String(e)));
      const version = await page.evaluate(() => navigator.userAgent);
      try {
        await page.goto("http://127.0.0.1:4174");
        await page.waitForFunction(() => window.ready);
        const passed = await page.evaluate((e) => window.check(e), engine);
        results.runs.push({ browser: kind, version, engine, passed });
        console.log(kind, engine, "conformance", passed);
        if (kind === "chromium" && process.env.STORAGE_CHECK_ONLY !== "1")
          for (const count of [10000, 100000]) {
            const result = await page.evaluate(
              ({ engine, count }) => window.bench(engine, count),
              { engine, count },
            );
            results.runs.push({ browser: kind, version, ...result });
            console.log(kind, engine, count, result.append_p95_ms);
            await writeFile(output, JSON.stringify(results, null, 2));
          }
      } catch (e) {
        results.failures.push({
          browser: kind,
          engine,
          error: String(e),
          pageErrors: errors,
        });
        console.log(kind, engine, "FAILED", String(e));
      }
      await context.close();
      await writeFile(output, JSON.stringify(results, null, 2));
    }
  }
} finally {
  server.close();
  await writeFile(output, JSON.stringify(results, null, 2));
}

if (results.failures.some((f) => f.engine !== "turso")) process.exitCode = 1;
