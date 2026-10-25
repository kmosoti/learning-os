// Browser platform glue; no application logic, telemetry, forced takeover or remote cache.
const CACHE = 'learningos-shell-@CACHE_VERSION@';
const ASSETS = ['/', '/session/ownership', '/app.css', '/bootstrap.js', '/manifest.webmanifest', '/icon.svg', '/icon-192.png', '/icon-512.png', '/fonts/cmunbx.ttf', '/fonts/source-sans-3-latin-400-normal.woff2', '/fonts/source-sans-3-latin-600-normal.woff2', '/fonts/jetbrains-mono-latin-400-normal.woff2', '/pkg/learningos_web.js', '/pkg/learningos_web_bg.wasm', '/api/v1/lesson'];
self.addEventListener('install', event => event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(ASSETS.map(url => new Request(url, { cache: 'reload' }))))));
self.addEventListener('activate', event => event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key.startsWith('learningos-shell-') && key !== CACHE).map(key => caches.delete(key))))));
self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET' || new URL(event.request.url).origin !== self.location.origin) return;
  event.respondWith(caches.match(event.request).then(cached => cached || fetch(event.request).catch(error => {
    if (event.request.mode === 'navigate') return caches.match('/');
    throw error;
  })));
});
