// A DOOR CHARGE ORDER CARRIED OUT OR ANSWERED, ON THE REAL PAGE (src/game/doortools.ts DOORTOOLS.chargeAnswer; the headless lines are
// tools/qa/chargeanswer-check.ts): headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe), the pair's room clear with every
// defender off the floor but the farthest, the order given through the page's own `__orders.give` on the farthest door that takes it and that the
// picture does not show open when it is put open.
//   PORT=5231 node tools/qa/chargeanswer-page.cjs [outDir]        (a dev server on that port: PORT=5231 bun run tools/serve.ts)
//   on    the page as it opens: the switch is on; the leaf swung open and shut again unseen, the charge still goes on and is armed; the leaf then
//         opened under the set charge, the setter's bubble says so and both men hold on the door
//   off   the same page under ?chargeanswer=0: the switch off, the same swing ends the order with the men left on its assignments, as it was
// exit 0 = all of it, with no page error · 1 = otherwise. A still of the men holding is written to outDir when one is given.
const { chromium } = (() => { for (const m of ['/opt/node-tools/node_modules/playwright', 'playwright', 'playwright-core']) { try { return require(m); } catch { } } throw new Error('no playwright to drive the page with'); })();
const fs = require('fs'), path = require('path');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', OUT = process.argv[2] || '';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
if (OUT) fs.mkdirSync(OUT, { recursive: true });
(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : { channel: 'chromium' }), args: FLAGS }); let failed = false;
  const line = (ok, label, detail) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  for (const [name, q, want] of [['on', '', true], ['off', '&chargeanswer=0', false]]) {
    const page = await browser.newPage({ viewport: { width: 1280, height: 720 }, deviceScaleFactor: 1 }); const errors = [];
    page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errors.push(m.text().slice(0, 300)); });
    page.on('pageerror', x => errors.push('pageerror: ' + String(x && x.stack || x).slice(0, 400)));
    await page.goto(`http://localhost:${PORT}/?paused&quality=${QUALITY}&mute&roomclear&freshstart=0&nocutboxes${q}`, { timeout: 600000 });
    const fatal = await page.evaluate(async () => { await window.__ready; window.__testVisible = true; return document.getElementById('fatal')?.textContent || ''; });
    if (fatal) errors.push('fatal: ' + fatal);
    const R = await page.evaluate(async () => {
      // (the software adapter draws a frame in seconds: the game is STEPPED with the renderer stubbed to the world's upload)
      const step = (n) => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { window.__runFrames(n, 1 / 60, []); } finally { e.render = real; } };
      const g = window.__game, D = window.__doortools, O = window.__orders, F = g.fogOfWar; window.__menu.close(); window.__smoke.params.enabled = false; g.godMode = true;
      step(30);
      const flat = (a, b) => Math.hypot(a[0] - b[0], a[2] - b[2]), team = g.guards.filter(x => x.unit && x.team === 'operative' && x.state !== 'dead'), home = team[0].char.pos;
      const out = { knob: window.__knobs.registry().DOORTOOLS.chargeAnswer, fog: !!F.active, men: team.length };
      const foes = g.guards.map((x, i) => [x, i]).filter(([x]) => x.team === 'hostile' && x.state !== 'dead').sort((a, b) => flat(b[0].char.pos, home) - flat(a[0].char.pos, home));
      for (const [, i] of foes.slice(1)) window.__killGuard(i, 'struck');
      const put = (d, open) => { if (open) { d.latched = false; d.angle = (d.swing || 1) * 1.4; } else { d.angle = 0; d.latched = true; } d.vel = 0; d.closing = false; };
      const isOpen = d => !d.latched && Math.abs(d.angle) >= 0.6;
      const side = d => { const th = d.def.closedDir; return Math.cos(th) * (home[2] - d.def.hinge[1]) - Math.sin(th) * (home[0] - d.def.hinge[0]) >= 0 ? 1 : -1; };
      const doors = g.doors.list.filter(d => !d.def.steel && !d.off && !isOpen(d)).sort((a, b) => flat(b.frameCentre, home) - flat(a.frameCentre, home)); let d = null;
      for (const x of doors) { const was = [x.angle, x.latched]; put(x, true); step(3); const held = F.doorHeld(x); x.angle = was[0]; x.latched = was[1]; step(3);
        if (held && !O.give('charge', { kind: 'door', at: [x.frameCentre[0], 0, x.frameCentre[2]], door: x, side: side(x) }, 'all')) { d = x; break; } }
      if (!d) return { ...out, why: 'no door that takes the order unseen' };
      const op = D.ops[D.ops.length - 1], asg = () => team.filter(m => m.state !== 'dead').map(m => { const A = O.of(m); return A ? `${A.order}: ${A.arrived ? A.there : A.going}` : 'none'; });
      out.door = d.def.name; step(30); put(d, true); let shown = false;
      for (let f = 0; f < 150; f++) { step(1); if (isOpen(d) && !F.doorHeld(d)) shown = true; }
      out.afterSwing = D.ops.includes(op) ? `still ${op.phase}` : `ended: ${op.ended}`; out.shown = shown; put(d, false);
      for (let f = 0; f < 60 * 80 && D.ops.includes(op) && op.phase !== 'armed'; f++) step(1);
      out.then = D.ops.includes(op) ? `still ${op.phase}` : `ended: ${op.ended}`; out.set = D.set.filter(c => c.door === d).length; out.left = asg();
      if (out.then === 'still armed') { step(40); put(d, true); step(4); out.bubble = op.lead.bubble ? op.lead.bubble.text : ''; out.gone = D.ops.includes(op) ? `still ${op.phase}` : `ended: ${op.ended}`; step(120); out.hold = asg(); put(d, false); }
      window.__caCam = op.lead.char.pos;
      return out; });
    if (OUT && want) { await page.evaluate(async () => { const dev = window.__engine.device, tick = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
        for (let i = 0; i < 3; i++) { window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone(); await tick(); } }).catch(() => { });
      await page.screenshot({ path: path.join(OUT, 'chargeanswer-hold.png'), timeout: 120000 }).catch(e => console.log(`    (no still: ${String(e).slice(0, 120)})`)); }
    const tail = `${JSON.stringify(R)}${errors.length ? ' · ERRORS ' + errors.join(' | ') : ''}`;
    if (want) line(R.knob === true && R.fog && !R.shown && R.afterSwing === 'still place' && R.then === 'still armed' && R.set === 1 && /charge is off/.test(R.bubble || '') && R.gone === 'ended: the charge is gone' && (R.hold || []).length > 0 && R.hold.every(x => x.startsWith('hold: ')) && !errors.length,
      'the page as it opens: a leaf swung open and shut again unseen does not end the order, the charge is armed; opened under the set charge, the setter says so and the men hold on the door', tail);
    else line(R.knob === false && R.afterSwing === 'ended: the door opened' && (R.left || []).some(x => x.startsWith('charge: ')) && !errors.length,
      'under ?chargeanswer=0: the switch off, the same swing ends the order and leaves the men on its assignments, as it was', tail);
    await page.close();
  }
  await browser.close();
  console.log(failed ? '=== chargeanswer-page: FAILED ===' : '=== chargeanswer-page: ALL PASS ===');
  process.exit(failed ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
