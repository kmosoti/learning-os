import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { readFile } from 'node:fs/promises';
async function ready(page) { await expect(page.locator('#main')).toHaveAttribute('data-ready', 'true'); }
async function phase(page, phase) { await expect(page.locator('#main')).toHaveAttribute('data-phase', phase); }
async function start(page) { await page.goto('/session/ownership'); await ready(page); await page.getByRole('button', { name: 'Start learning →', exact: true }).click(); await phase(page, 'Plan'); await page.getByRole('button', { name: 'Make a prediction →' }).click(); await phase(page, 'Predict'); }
async function reachClosedCheck(page) { await start(page); await page.getByLabel("I don't know yet", { exact: true }).check(); await page.locator('#reasoning').pressSequentially('Before reading: uncertain'); await expect(page.locator('#reasoning')).toBeFocused(); await page.getByRole('button', { name: 'Save prediction →' }).click(); await phase(page, 'Teach'); await page.locator('#reading').pressSequentially('Reading: ownership moves; <script> stays plain text.'); await expect(page.locator('#reading')).toBeFocused(); await page.getByRole('button', { name: 'I’m ready to check →' }).click(); await phase(page, 'CheckClosed'); }
async function backup(page) { return page.evaluate(async () => JSON.parse(await (await import('/pkg/learningos_web.js')).local_backup_json())); }

test('complete loop, separate thoughts, exact receipt retry, offline hard reload and correction', async ({ page, context }) => {
  const errors=[];page.on('pageerror',error=>errors.push(String(error)));
  await reachClosedCheck(page);
  await expect(page.locator('pre')).toHaveCount(0);
  await expect(page.locator('body')).not.toContainText('rejected with E0382');
  await page.getByRole('button', { name: 'Open check →' }).click(); await phase(page, 'Check');
  await page.getByLabel('It does not compile', { exact: true }).check();
  await page.locator('#reasoning').pressSequentially('Answer: name was moved into next.');
  await page.locator('#confidence').fill('72');
  await page.getByRole('button', { name: 'Commit answer →' }).click(); await phase(page, 'Evidence');
  const before=await backup(page);
  const observed=before.events.filter(e=>e.kind.Observed).map(e=>e.kind.Observed);
  expect(observed).toHaveLength(2);
  expect(observed[0].kind).toBe('Prediction');expect(observed[0].reasoning).toBe('Before reading: uncertain');
  expect(observed[1].kind).toBe('Check');expect(observed[1].reading_thoughts).toBe('Reading: ownership moves; <script> stays plain text.');
  expect(observed[1].reasoning).toBe('Answer: name was moved into next.');expect(observed[1].confidence).toBe(72);
  const receipt=before.receipts.at(-1);
  expect(await page.evaluate(async request=>JSON.parse(await (await import('/pkg/learningos_web.js')).local_command_json(JSON.stringify(request))), receipt.request)).toEqual(receipt);
  expect((await backup(page)).events).toEqual(before.events);
  await page.evaluate(async()=>{await navigator.serviceWorker.ready;});
  await page.reload();await ready(page);await phase(page,'Evidence');
  await context.setOffline(true); await page.reload(); await ready(page); await phase(page, 'Evidence');
  expect((await backup(page)).event_digest).toBe(before.event_digest);
  await page.locator('.evidence-row').filter({hasText:'Check · move'}).getByRole('button', { name: 'Correct my answer', exact: true }).click();
  await page.getByRole('button', { name: 'It compiles', exact: true }).click();await phase(page,'Evidence');
  await expect.poll(async()=> (await backup(page)).events.length).toBe(before.events.length+3);
  const corrected=await backup(page);expect(corrected.events.at(-2).kind.Observed.replaces).toBe(observed[1].id);
  await page.reload(); await ready(page); expect((await backup(page)).event_digest).toBe(corrected.event_digest);
  expect(errors).toEqual([]);
});

test('export/import retains event digest and receipts; clearing requires explicit text', async ({ page }) => {
  await start(page); await page.getByLabel('It compiles', { exact: true }).check();await page.getByRole('button',{name:'Save prediction →'}).click();await phase(page,'Teach');
  const original=await backup(page);
  await page.getByRole('button',{name:'Activity & local data',exact:true}).click();
  const downloadPromise=page.waitForEvent('download');await page.getByRole('button',{name:'Export evidence backup',exact:true}).click();const download=await downloadPromise;const exported=await readFile(await download.path(),'utf8');expect(JSON.parse(exported)).toEqual(original);
  await page.getByText('Clear local learning data',{exact:true}).click();
  await expect(page.getByRole('button',{name:'Clear this browser’s learning data',exact:true})).toBeDisabled();
  await page.locator('#clear').fill('CLEAR');await page.getByRole('button',{name:'Clear this browser’s learning data',exact:true}).click();await phase(page,'Goal');
  await page.getByRole('button',{name:'Activity & local data',exact:true}).click();await page.locator('#backup').fill(exported);await page.getByRole('button',{name:'Validate and import into an empty journal',exact:true}).click();await phase(page,'Teach');
  expect(await backup(page)).toEqual(original);
  const first=original.receipts[0];expect(await page.evaluate(async request=>JSON.parse(await (await import('/pkg/learningos_web.js')).local_command_json(JSON.stringify(request))), first.request)).toEqual(first);
});

test('two tabs preserve their own drafts and only one writer wins a revision',async({page,context})=>{
  await start(page);await page.locator('#reasoning').fill('First tab keeps this draft');
  const other=await context.newPage();await other.goto('/session/ownership');await ready(other);await phase(other,'Predict');await other.locator('#reasoning').fill('Second tab has a different draft');
  await expect.poll(()=>page.evaluate(async()=>JSON.parse(await(await import('/pkg/learningos_web.js')).local_draft_json()).reasoning)).toBe('First tab keeps this draft');
  await expect.poll(()=>other.evaluate(async()=>JSON.parse(await(await import('/pkg/learningos_web.js')).local_draft_json()).reasoning)).toBe('Second tab has a different draft');
  await page.getByLabel('It compiles',{exact:true}).check();await other.getByLabel("I don't know yet",{exact:true}).check();
  await Promise.all([page.getByRole('button',{name:'Save prediction →'}).click(),other.getByRole('button',{name:'Save prediction →'}).click()]);
  await expect.poll(async()=>Number(await page.locator('#main').getAttribute('data-phase')==='Teach')+Number(await other.locator('#main').getAttribute('data-phase')==='Teach')).toBe(1);
  const loser=(await page.locator('#main').getAttribute('data-phase'))==='Teach'?other:page;await expect(loser.getByRole('alert')).toContainText('behind another save');await expect(loser.locator('#reasoning')).not.toHaveValue('');
  await loser.getByRole('button',{name:'Reload local state'}).click();await phase(loser,'Teach');await loser.reload();await ready(loser);await phase(loser,'Teach');
});

test('native and WASM replay projections are byte-equivalent JSON values',async({page})=>{
  const golden=JSON.parse(await readFile(new URL('../../artifacts/v2/golden-replay.json',import.meta.url),'utf8'));
  await page.goto('/');await ready(page);const actual=await page.evaluate(async events=>JSON.parse((await import('/pkg/learningos_web.js')).replay_json(JSON.stringify(events))),golden.events);expect(actual).toEqual(golden.session);
});

for(const [width,height]of [[1440,900],[1280,720],[390,844],[360,740]])test(`viewport ${width}x${height}: keyboard, source dialog, focus, axe and reflow`,async({page},info)=>{
  await page.setViewportSize({width,height});await page.goto('/');await ready(page);await page.screenshot({path:info.outputPath(`graph-${width}x${height}.png`),fullPage:true});
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  await page.getByRole('button',{name:'Start learning →',exact:true}).click();await phase(page,'Plan');await page.getByRole('button',{name:'Make a prediction →'}).click();await phase(page,'Predict');
  await page.getByRole('button',{name:'Sources',exact:true}).click();await expect(page.getByRole('dialog')).toBeVisible();await page.keyboard.press('Escape');await expect(page.getByRole('dialog')).toHaveCount(0);await expect(page.getByRole('button',{name:'Sources',exact:true})).toBeFocused();
  await page.screenshot({path:info.outputPath(`prediction-${width}x${height}.png`),fullPage:true});
  const results=await new AxeBuilder({page}).withTags(['wcag2a','wcag2aa','wcag21aa']).analyze();expect(results.violations).toEqual([]);
  await page.emulateMedia({reducedMotion:'reduce'});await page.evaluate(()=>{const root=document.documentElement;root.style.fontSize=`${parseFloat(getComputedStyle(root).fontSize)*2}px`;});expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
});

test('hint/source consultation retains typed draft and source-assisted checks never count as unaided',async({page})=>{
  await reachClosedCheck(page);await page.getByRole('button',{name:'Open check →'}).click();await phase(page,'Check');
  await page.getByLabel('It does not compile',{exact:true}).check();await page.locator('#reasoning').fill('Keep this reasoning when asking for help.');
  await page.getByRole('button',{name:'Hint 0/2',exact:true}).click();await expect(page.getByRole('button',{name:'Hint 1/2',exact:true})).toBeVisible();await expect(page.locator('#reasoning')).toHaveValue('Keep this reasoning when asking for help.');
  await page.getByRole('button',{name:'Sources',exact:true}).click();await expect(page.getByRole('dialog')).toBeVisible();await page.keyboard.press('Escape');await expect(page.getByRole('dialog')).toHaveCount(0);await expect(page.locator('#reasoning')).toHaveValue('Keep this reasoning when asking for help.');
  await page.getByRole('button',{name:'Commit answer →'}).click();await phase(page,'Evidence');const evidence=await backup(page);expect(evidence.events.filter(e=>e.kind.Observed).at(-1).kind.Observed.source_consulted).toBe(true);await page.getByRole('button',{name:'Choose the next step →'}).click();await phase(page,'NextAction');await expect(page.getByRole('heading',{name:'Try once without a hint'})).toBeVisible();
});

test('seeded graph camera, search, source inspection and evidence overlay work',async({page})=>{
  await page.goto('/');await ready(page);const graph=page.locator('.interactive-graph');const initial=await graph.getAttribute('viewBox');await page.getByRole('button',{name:'Zoom in',exact:true}).click();expect(await graph.getAttribute('viewBox')).not.toBe(initial);await graph.focus();await page.keyboard.press('ArrowRight');await page.keyboard.press('0');expect(await graph.getAttribute('viewBox')).toBe(initial);
  await page.getByLabel('Search graph assertions').fill('Mutable');await page.getByText('Explore the graph as a list',{exact:true}).click();await page.getByRole('button',{name:'Mutable access ↗',exact:true}).click();await expect(page.getByRole('heading',{name:'Mutable access under explicit conditions',exact:true})).toBeVisible();await expect(page.locator('.graph-inspector')).toContainText('E0596');await page.getByRole('button',{name:'Evidence overlay: off',exact:true}).click();await expect(page.locator('.graph-evidence')).toContainText('0 prediction(s)');await expect(page.locator('.graph-evidence')).toContainText('Retention and broader transfer remain untested');
});

test('malformed existing local records fail closed instead of appearing empty',async({page})=>{
  await page.goto('/');await ready(page);
  await page.evaluate(()=>new Promise((resolve,reject)=>{const open=indexedDB.open('learningos-v2-evidence-1',1);open.onerror=()=>reject(open.error);open.onsuccess=()=>{const db=open.result;const tx=db.transaction('local','readwrite');tx.objectStore('local').put({not:'a JSON string'},'journal');tx.oncomplete=()=>{db.close();resolve();};tx.onabort=()=>reject(tx.error);};}));
  await page.reload();await expect(page.getByRole('alert')).toContainText('Damaged local record journal');await expect(page.locator('#main')).toHaveAttribute('data-ready','false');
});

test('reading appearance persists offline without changing evidence or the current draft', async ({page,context}) => {
  await start(page);
  await page.locator('#reasoning').fill('Keep my reasoning while I compare themes.');
  await expect.poll(()=>page.evaluate(async()=>JSON.parse(await(await import('/pkg/learningos_web.js')).local_draft_json()).reasoning)).toBe('Keep my reasoning while I compare themes.');
  const before=await backup(page);
  for(const [width,height,size] of [[1440,900,'20'],[360,740,'24']]) {
    await page.setViewportSize({width,height});
    await page.getByLabel('Text size',{exact:true}).selectOption(size);
    for(const theme of ['dark','contrast','paper']) {
      await page.getByLabel('Color scheme',{exact:true}).selectOption(theme);
      await expect(page.locator('html')).toHaveAttribute('data-theme',theme);
      await expect(page.locator('#reasoning')).toHaveValue('Keep my reasoning while I compare themes.');
      expect(await backup(page)).toEqual(before);
      expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
      const result=await new AxeBuilder({page}).withTags(['wcag2a','wcag2aa','wcag21aa']).analyze();
      expect(result.violations).toEqual([]);
      await page.evaluate(()=>window.scrollTo(0,0));
      await page.screenshot({path:test.info().outputPath(`appearance-${theme}-${width}.png`),fullPage:true});
    }
  }
  await page.getByLabel('Text size',{exact:true}).selectOption('22');
  await page.getByLabel('Color scheme',{exact:true}).selectOption('dark');
  await page.evaluate(async()=>{await navigator.serviceWorker.ready;});
  await page.reload();await ready(page);
  await context.setOffline(true);await page.reload();await ready(page);await phase(page,'Predict');
  await expect(page.getByLabel('Color scheme',{exact:true})).toHaveValue('dark');
  await expect(page.getByLabel('Text size',{exact:true})).toHaveValue('22');
  await expect(page.locator('html')).toHaveAttribute('data-theme','dark');
  await expect(page.locator('html')).toHaveAttribute('data-text-size','22');
  await expect(page.locator('#reasoning')).toHaveValue('Keep my reasoning while I compare themes.');
  expect(await backup(page)).toEqual(before);
});
