// Platform glue only: generated WASM module and Leptos island discovery.
import init, * as app from '/pkg/learningos_web.js';
await init({ module_or_path: '/pkg/learningos_web_bg.wasm' });
app.hydrate();
for (const island of document.querySelectorAll('leptos-island')) {
  const hydrate = app[island.dataset.component];
  if (hydrate) await hydrate(island);
}
