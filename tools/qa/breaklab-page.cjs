// THE BREAK LAB ON THE REAL PAGE (src/host/breaklab.ts, src/game/things.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's
// recipe), the dev server's /breaklab as a person opens it, and the lab's own buttons pressed in the panel.
//   PORT=5199 node tools/qa/breaklab-page.cjs        (a dev server on that port: PORT=5199 bun run tools/serve.ts)
//   on    /breaklab: both switches read on, the lab's system is in the panel, a grenade beside a thing out of doors breaks it, the level goes back
//   off   /breaklab?things=0: the index reads off, the same grenade in the same place breaks nothing out of doors (the build as it was)
// exit 0 = both, with no page error · 1 = otherwise. Game logic and the panel's rows only: nothing here looks at a picture (a frame is drawn on the software
// adapter to see that drawing the level with things broken throws no error).
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);

const STAGE = async (place) => {
  await window.__ready;
  const g = window.__game, T = window.__knobs.registry().THINGS, D = g.destr;
  const btn = t => [...document.querySelectorAll('button')].find(b => b.textContent.trim() === t);
  const lab = [...document.querySelectorAll('*')].some(n => n.children.length === 0 && /Bursts at the cursor/.test(n.textContent || ''));
  window.__runFrames(4, 1 / 60);
  // (the place: with the index on, the first furniture thing with open sky over it — no box above its top — that stands; the 'off' page is handed the same place)
  let at = place;
  if (!at) {
    // (a Thing's `a` is its bounds: x0 x1 y0 y1 z0 z1)
    const all = D.things.all().filter(t => t.via !== 'unseen'), roofed = t => { const x = (t.a.x0 + t.a.x1) / 2, z = (t.a.z0 + t.a.z1) / 2; return all.some(o => o !== t && o.a.y0 >= t.a.y1 - 0.01 && o.a.y0 > 1.9 && o.a.x0 <= x && o.a.x1 >= x && o.a.z0 <= z && o.a.z1 >= z); };
    const t = all.find(t => t.via === 'furniture' && t.hold === 'stands' && t.mat === 'wood' && !t.ghost && !roofed(t)) || all.find(t => t.via === 'furniture' && t.hold === 'stands' && !t.ghost && !roofed(t));
    if (!t) return { err: 'no furniture thing stands out of doors on this level' };
    at = [(t.a.x0 + t.a.x1) / 2, 0.3, t.a.z1 + 0.3];
  }
  const hidden = () => { const B = g.col.boxes; let n = 0; for (const b of B) if (b.flags & 3) n++; return n; };   // (NoPrimary | NoShadow: how a broken thing is put away)
  const h0 = hidden(), census = D.things.census();
  g.aimPoint = [at[0], 0, at[2]];
  const b = btn('grenade'); if (!b) return { err: 'the lab has no grenade button', lab, on: T.on, rings: T.rings.on };
  b.click(); window.__runFrames(20, 1 / 60);
  const h1 = hidden(), said = [...document.querySelectorAll('*')].map(n => n.children.length === 0 ? (n.textContent || '') : '').find(s => /^grenade: \d+ parts broke/.test(s)) || '';
  let shot = h1; const back = btn('put it all back'); if (back) { back.click(); window.__runFrames(4, 1 / 60); shot = hidden(); }
  return { lab, on: T.on, rings: T.rings.on, at, broke: h1 - h0, back: shot - h0, said: said.split('\n')[0], n: census.n, furniture: census.via.furniture || 0, god: g.godMode, unseen: g.playerInvisible };
};

(async () => {
  let red = 0, place = null; const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  for (const job of [{ name: 'on', query: '' }, { name: 'off', query: 'things=0&' }]) {
    const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 })).newPage();
    const errs = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300))); page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
    page.on('response', r => { if (r.status() >= 400) log(`  (${r.status()} ${r.url()})`); });
    const url = `http://localhost:${PORT}/breaklab?${job.query}paused&quality=${QUALITY}&mute`; log(`${job.name}: ${url}`);
    let r; try { await page.goto(url); r = await page.evaluate(STAGE, place); } catch (e) { r = { err: String(e).slice(0, 300) }; }
    if (!r.err && job.name === 'on') place = r.at;
    await page.close();
    if (r.err) { red++; console.log(`- FAIL — ${job.name}: ${r.err}${errs.length ? ` · ${errs[0]}` : ''}`); continue; }
    const detail = `index ${r.on} · rings ${r.rings} · lab in the panel ${r.lab} · ${r.n} boxes, furniture ${r.furniture} · at ${r.at.map(v => v.toFixed(1))} · put away by the grenade ${r.broke} · still away after 'put it all back' ${r.back} · "${r.said}" · god ${r.god}, unseen ${r.unseen} · page errors ${errs.length}${errs.length ? ': ' + errs[0] : ''}`;
    const staged = r.lab && !errs.length && r.god && r.unseen && r.back === 0;
    const ok = job.name === 'on' ? staged && r.on === true && r.rings === true && r.broke > 0 : staged && r.on === false && r.broke === 0;
    if (!ok) red++;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${job.name === 'on' ? '/breaklab as it opens: both switches on, the lab open in the panel, a grenade beside a thing out of doors breaks it, and the level goes back' : '/breaklab?things=0: the index is off, and the same grenade in the same place breaks nothing (the build as it was)'} (${detail})`);
  }
  await browser.close();
  console.log(`=== breaklab-page: ${red ? 'FAILURES above' : 'ALL PASS'} ===`);
  process.exit(red ? 1 : 0);
})();
