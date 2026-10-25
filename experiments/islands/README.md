# Independent UI experiment harness

Issue #51 progress checkpoint, 2026-10-25. These scripts exercise the actual Leptos
app. They do not create another UI implementation. Read the
[checkpoint report](../../docs/experiments/ui-verification-checkpoint.md) before
interpreting prepared scripts as completed experiments.

```sh
npm ci --ignore-scripts --prefix experiments/islands
npm run check --prefix experiments/islands
python3 -m py_compile experiments/islands/at-session.py experiments/islands/pin.py
URL=http://127.0.0.1:3000 OUT=/tmp/learningos-independent node experiments/islands/integration.mjs
URL=http://127.0.0.1:3000 OUT=/tmp/learningos-crash node experiments/islands/crash.mjs
```

Node 22+, system Chrome at `/usr/bin/google-chrome`, and Playwright's Firefox are
needed. Dependencies are locked. `BROWSER=chromium|firefox` and `CASE_FILTER` narrow
the independent integration cases. Output directories are explicit; use fresh
directories to avoid appending to previous evidence. Crash checks create synthetic
profiles, kill only their own Chrome parent process, reopen those profiles, then
remove them. No real learner profile is used.

Prepared experiments, **not executed at this checkpoint**:

```sh
CANDIDATES='{"coherent":"http://127.0.0.1:4302","split":"http://127.0.0.1:4303"}' RUNS=30 OUT=/tmp/islands-hydration node experiments/islands/measure.mjs
EXPERIMENT=graph RUNS=10 OUT=/tmp/islands-graph node experiments/islands/measure.mjs
URL=http://127.0.0.1:4302 FIXTURE=/tmp/learningos-maximal-backup.json RUNS=30 OUT=/tmp/islands-port node experiments/islands/bounded-port.mjs
OLD_URL=http://127.0.0.1:4302 NEW_URL=http://127.0.0.1:4303 OUT=/tmp/islands-upgrade node experiments/islands/upgrade.mjs
URL=http://127.0.0.1:4302 OUT=/tmp/islands-install node experiments/islands/installability.mjs
```

Freeze both candidate bundles, pin their source/asset hashes using `pin.py`, then
ensure no compilation or competing benchmarks run during measurements. Hydration
uses a fresh context for each candidate/run: uncached navigation, one committed
transition, service-worker-ready reload, then offline reload. It records page
resource transfer sizes, readiness, buffered LCP/CLS/event entries, heap metrics,
and retained phase. The mobile CDP throttle applies to the page target; service
worker target throttling must be audited before publishing network-budget claims.
Do not label warm service-worker-cache reloads as warm HTTP-cache measurements.

Graph runs record Rust call cost, requestAnimationFrame gaps, semantic-list hashes,
and CDP metrics. Run zero additionally records paint traces and must be excluded
from uninstrumented timing aggregates. Heap metrics do not establish total browser
or WASM resident memory. Scripted interaction timing is a lab proxy, not field INP.
The bounded-port experiment seeds a valid synthetic 9,990-event archive; seeding is
excluded from actual protocol append/retry/replay/backup timings.

`spacing-preview.mjs` adds only the toolkit's exact documented spacing-style hash
to a loopback test proxy; production CSP is unchanged. Run the installed toolkit
against that proxy to test spacing. Its initial-page automated matrix does not
replace workflow checks, manual contrast review, or real assistive technology.

`at-session.py` starts official AT-SPI/Orca processes inside a caller-provided
private `dbus-run-session`; the checkpoint used isolated Xvfb `:99` and isolated
XDG directories. It never replaces the user's screen reader. This setup currently
fails before Orca speech output; its browser aria snapshots are separate evidence.
