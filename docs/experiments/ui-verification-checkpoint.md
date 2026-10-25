# Independent UI verification checkpoint

2026-10-25 · Refs #51, #52 · **Incomplete research checkpoint; no mechanism winner.**

The owner requested a bounded stopping point and progress commit/push. Full
performance campaigns were deliberately not started. The implementation and
research remain reviewable, unmerged work.

## Observed results

Independent Playwright 1.56.1 / axe 4.11.1 checks used Chrome **155.0.8059.79** on
the pre-review Leptos server at `127.0.0.1:3000`, with fresh synthetic profiles.
[Raw results and selected screenshots](artifacts/independent-checkpoint/) retain
failures. The initial running executable and assets are identified in
`initial-runtime-pin.json`; the exact source snapshot for that older binary was
not retained, so these are exploratory results, not final-revision acceptance.

- At 1440×900, 390×844 and 360×740, graph, prediction, teaching, closed check,
  open check and evidence passed root-overflow checks and reported zero axe
  violations. Dialog keyboard containment/restoration, preserved reasoning,
  plain malicious reasoning, offline evidence reload, 200% root text and 200%
  **CSS zoom** also passed. Aria snapshots are retained. Axe marked contrast as
  incomplete; manual rendered contrast review remains open. CSS zoom does not
  establish browser-menu zoom or soft-keyboard behavior.
- The 1280×720 run stopped at an immediate focus assertion before asynchronous
  restoration completed. The harness now waits for restored focus; the full
  viewport case has not been rerun. Its captured states remain evidence of the
  attempt, not a completed pass. The implementation agent subsequently reported
  related Sources/focus failures in its own suite; that correction is not verified
  by this checkpoint.
- A focused rerun passed incorrect-answer repair → hinted correct answer →
  unaided retry → changed examples → delayed-retrieval recommendation, followed
  by append-only retraction. Quota injection retained the draft and acknowledged
  no write; a future database version failed closed. Failure after initial store
  creation rolled the database migration back and a subsequent reload recovered.
- SSR withheld the explanation/check content. Source Markdown containing markup
  was served as `text/plain` with `nosniff`; strict CSP blocked an injected inline
  script. This is not acceptance of arbitrary-source ingestion, which is outside
  the current slice.
- Three real Chrome-parent **SIGKILL** schedules after a prediction receipt
  reopened the same profile with byte-equivalent backup values, including all
  events and receipts: **3/3 passed, zero acknowledged observations lost** in
  those schedules. This does not simulate hardware power loss or browser eviction.
- The first service-worker failure test timed out: aborting `/sw.js` through the
  request router did not establish reliable interception. That failed run is
  retained. Explicitly rejecting `ServiceWorkerContainer.register` then passed:
  the UI reported unavailable caching while retaining access to the local journal.

The retained initial Composite UI Toolkit report covers 360/768/1280 px, normally
and with its documented text-spacing override: six automated cases passed. Its
isolated loopback proxy permits only that exact style hash; production CSP remains
unchanged. This initial-page check does not satisfy the outstanding manual contrast
or complete final workflow gates.

## Assistive technology and remaining gates

Orca **50.3**, AT-SPI **2.60.7**, Playwright Firefox **142**, and private Xvfb were
attempted with isolated D-Bus/XDG state. The registry started, but Orca exited
before speech output with XInputExtension `BadWindow`, resource `0x1`. Starting
Firefox before Orca produced the same failure. Exact stderr is retained. Browser
keyboard/aria checks are **not** a screen-reader pass. Real Android installation,
offline use, performance, soft keyboard and TalkBack exercise remain owner-run
release gates. Safari is unmeasured.

## Prepared but unfinished

Locked harnesses cover coherent versus split islands, 30 cold/warm/offline runs per
candidate and profile, actual graph frame gaps/paint traces, bounded application
archive costs, installability diagnostics, and waiting service-worker upgrades.
These campaigns have **no samples/results at this checkpoint**. Final review1
candidate bundles were produced by the implementation agent; `artifact-pin.json`
records their bytes plus the current source-content digest independently of later
branch rebasing. No hydration boundary or graph renderer has been selected from
unmeasured performance.

Both final candidates still need complete Firefox/workflow/accessibility/offline
gates, 1280 focus retest, graph interaction/list parity, manual contrast, actual
browser zoom, reduced-height keyboard checks, duplicate-tab identity checks and
upgrade/installability checks. Prepared measurement code also needs a small smoke
run and worker-throttling audit before a full campaign. The follow-on product
milestone remains one local Markdown source → immutable spans → reviewed assertions
→ a second offline lesson.

## Checkpoint verification

`npm run check --prefix experiments/islands` checks every JavaScript harness.
`python3 -m py_compile experiments/islands/at-session.py experiments/islands/pin.py`
passed. The focused browser commands and their raw outputs are represented by
`focused-results.jsonl`, `worker-rejection-results.jsonl` and `sigkill-results.json`.
No hosted CI or physical-device result is claimed here.
