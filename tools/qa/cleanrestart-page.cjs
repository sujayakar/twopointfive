// A RESTART ON THE REAL PAGE: headless Chromium on the software WebGPU adapter (tools/qa/page-look.cjs's recipe), the solo night as the page opens it,
// seeded. Everything is done by what a player can do. The level's light factors are read on its first frames with the game's clock beside each; the man
// draws his pistol and SHOOTS A LAMP OUT; two seconds pass; the page's own restart key (Shift+Enter) is pressed and the frames after it are read again and
// three of them drawn.
//   PORT=5199 node tools/qa/cleanrestart-page.cjs [outDir]     (a dev server on that port: PORT=5199 bun run tools/serve.ts)     env: SEED (default 7)
//   PASS = a lamp read broken before the key · after it the game's clock is back at the start, no fixture is broken, the mains are as a fresh boot's, no
//          beacon turns, nothing is said of the mains or of a restart · every light's factor on each frame after the restart is the one the fresh boot
//          had AT THE SAME GAME TIME (the dying tubes apart: their dropouts follow the absolute clock) · the camera stands by the man as a fresh boot's
//          does · three frames draw · no page error
// The frames are matched by the game's clock and not by their count: the frame the restart is made on shows the start as it stands (time 0) and the
// first tick comes on the next, so a count of frames is one off against a boot. The wreck is the player's own shot: a check that breaks a fixture or pulls
// the mains by a call on `window.__game` moves only the page's copy of the game where the simulation is not the page's, and the restart is then judged
// against a level that was never wrecked. No counter of a module's own is read: what a restart did is read off the game.
// exit 0 = pass · 1 = otherwise. The verdict is game state only: nothing here judges a picture (the software adapter is not a GPU, and how the first
// frames after a restart LOOK has not been seen on one). With an outDir it keeps a still of the frame before the restart and of the third frame after
// it, for a person to look at, never for the verdict.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const fs = require('fs'), path = require('path');
const PORT = process.env.PORT || 5199, QUALITY = process.env.QUALITY || 'verylow', SEED = process.env.SEED || 7, outDir = process.argv[2] || '';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
if (outDir) fs.mkdirSync(outDir, { recursive: true });

// in the page: the game stepped with no frame drawn
const STEP = (n) => { const e = window.__engine, real = e.render; e.render = () => { e.world.upload(); }; try { for (let i = 0; i < n; i++) window.__runFrames(1, 1 / 60); } finally { e.render = real; } };
// in the page: n frames stepped one at a time, each one's light factors kept (-2 for a dying tube: its dropouts follow the absolute clock, lighting.ts
// updateTargets — not a restart's to match)
const FRAMES = (n) => { const e = window.__engine, real = e.render, out = []; e.render = () => { e.world.upload(); };
  try { for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 60); out.push({ t: Math.round(window.__game.time * 60), f: window.__game.targets.map(t => t.fluorescentFlicker ? -2 : +t.factor.toFixed(5)) }); } } finally { e.render = real; } return out; };
// in the page: draw n frames one at a time, read the last one back as a PNG data URL (page-look.cjs DRAW)
const DRAW = async (n) => {
  const dev = window.__engine.device;
  const yieldMacro = () => new Promise(res => { const ch = new MessageChannel(); ch.port1.onmessage = () => res(); ch.port2.postMessage(0); });
  for (let i = 0; i < n; i++) { window.__runFrames(1, 1 / 600); await dev.queue.onSubmittedWorkDone(); await yieldMacro(); }
  window.__engine.render(window.__cam, window.__game.time, 1 / 60, false); const fr = await window.__engine.readbackFrame(); if (!fr) return null;
  if (fr.bgra) for (let i = 0; i < fr.data.length; i += 4) { const b = fr.data[i]; fr.data[i] = fr.data[i + 2]; fr.data[i + 2] = b; }
  for (let i = 0; i < fr.data.length; i += 4) fr.data[i + 3] = 255;
  const c2 = document.createElement('canvas'); c2.width = fr.w; c2.height = fr.h;
  c2.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(fr.data.buffer), fr.w, fr.h), 0, 0);
  return c2.toDataURL('image/png');
};
const SET = () => { const g = window.__game; window.__menu?.close?.(); if (window.__smoke) window.__smoke.params.enabled = false; g.godMode = true; g.aiEnabled = false;   // (the page's own switches, as its panels write them)
  return { targets: g.targets.length, dying: g.targets.filter(t => t.fluorescentFlicker).length }; };
// in the page: how far the camera's look-at is from the man (it leads his aim: never zero)
const CAM = () => { const p = window.__game.player.char.pos; return +Math.hypot(window.__cam.target[0] - p[0], window.__cam.target[2] - p[2]).toFixed(3); };
// in the page: THE PLAYER'S OWN SHOT. The pistol's key, then the cursor laid on the nearest lamps in turn and the trigger held, through the page's own
// Input, until a fixture reads broken (a lamp behind a wall takes none: the next is tried)
const SHOOT = () => { const g = window.__game, I = window.__input, cam = window.__cam, cv = document.getElementById('view'), e = window.__engine, real = e.render; e.render = () => { e.world.upload(); };
  const run = n => { for (let i = 0; i < n; i++) window.__runFrames(1, 1 / 60); }, broken = () => g.targets.filter(t => t.broken).length;
  try { I.pressed.add('Digit1'); run(40);
    const p = g.player.char.pos, lamps = g.targets.filter(t => t.pos && t.kind === 'emissive' && !t.broken && !t.fluorescentFlicker).sort((a, b) => Math.hypot(a.pos[0] - p[0], a.pos[2] - p[2]) - Math.hypot(b.pos[0] - p[0], b.pos[2] - p[2])); let tried = 0, at = null;
    for (const L of lamps.slice(0, 5)) { tried++;
      for (let i = 0; i < 90 && !broken(); i++) { const [sx, sy, front] = cam.project([L.pos[0], L.pos[1], L.pos[2]], cv.clientWidth, cv.clientHeight); if (front) { I.mouseX = sx; I.mouseY = sy; } I.buttons |= 1; if (i % 14 === 0) I.clicked |= 1; run(1); }
      I.buttons &= ~1; run(1); if (broken()) { at = L.name ?? String(L.i); break; } }
    return { shot: broken(), lamp: at, tried, holstered: !!g.player.holstered, lightsShot: g.chronicle?.counts?.lightsShot ?? null };
  } finally { e.render = real; } };
const RESTART = () => { const i = window.__input; i.keys.add('ShiftLeft'); i.pressed.add('Enter'); };
const AFTER = () => { const g = window.__game, p = g.player.char.pos; window.__input.keys.delete('ShiftLeft');
  return { time: +g.time.toFixed(3), mains: g.blackout.toJSON(), beacons: g.beacons.filter(b => b.target.on).length, broken: g.targets.filter(t => t.broken).length,
    said: g.messages.map(m => m.text).filter(t => /mains|guards reset|encounter restarted|power/i.test(t)), cam: +Math.hypot(window.__cam.target[0] - p[0], window.__cam.target[2] - p[2]).toFixed(3),
    fromSpawn: +Math.hypot(p[0] - g.level.playerSpawn[0], p[2] - g.level.playerSpawn[2]).toFixed(2), fatal: (document.getElementById('fatal') || {}).textContent || '' }; };

(async () => {
  let red = 0; const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const page = await (await browser.newContext({ viewport: { width: 1024, height: 576 }, deviceScaleFactor: 1 })).newPage();
  const errs = []; page.on('pageerror', e => errs.push(String(e).slice(0, 300))); page.on('console', m => { if (m.type() === 'error' && !/Failed to load resource/.test(m.text())) errs.push(m.text().slice(0, 300)); });
  const url = `http://localhost:${PORT}/?solo&paused&quality=${QUALITY}&mute&seed=${SEED}`; log(url);
  const r = {}, still = async (tag) => { if (!outDir) return; const u = await page.evaluate(DRAW, 1); if (u) fs.writeFileSync(path.join(outDir, `cleanrestart-${tag}.png`), Buffer.from(u.split(',')[1], 'base64')); };
  let boot = [], after = [];
  try {
    await page.goto(url, { timeout: 0 }); await page.evaluate(async () => { await window.__ready; });
    Object.assign(r, await page.evaluate(SET));
    boot = await page.evaluate(FRAMES, 8); r.camBoot = await page.evaluate(CAM); log(`booted, ${r.targets} lights`);
    Object.assign(r, await page.evaluate(SHOOT)); log(`shot: ${r.shot} fixture(s) broken (${r.lamp}), ${r.tried} lamp(s) tried`);
    await page.evaluate(STEP, 120); r.brokenBefore = (await page.evaluate(AFTER)).broken; r.timeBefore = (await page.evaluate(AFTER)).time;
    await still('before');
    await page.evaluate(RESTART); after = await page.evaluate(FRAMES, 6); Object.assign(r, await page.evaluate(AFTER));
    r.drawn = !!(await page.evaluate(DRAW, 3)); await still('third-frame'); r.fatal = (await page.evaluate(AFTER)).fatal;
  } catch (e) { r.err = String(e).slice(0, 300); }
  await browser.close();
  // every frame after the restart against the fresh boot's frame of the SAME game time
  const at = new Map(boot.map(b => [b.t, b.f])); let off = 0, worst = 0, matched = 0; const firstOff = [];
  for (const a of after) { const b = at.get(a.t); if (!b) continue; matched++; for (let i = 0; i < b.length; i++) { const d = Math.abs(b[i] - (a.f[i] ?? -1)); if (d > 1e-4) { off++; if (firstOff.length < 4) firstOff.push(`tick ${a.t} light ${i}: ${b[i]} at boot, ${a.f[i]} after`); } worst = Math.max(worst, d); } }
  r.clockAfter = after.map(a => a.t); r.framesMatched = matched; r.factorsOff = off; r.worst = +worst.toFixed(4);
  const line = (ok, what, detail) => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  line(!r.err && !r.fatal, 'the page boots the solo night and the run reaches its end', `${r.err || 'no error'} · fatal "${r.fatal || ''}" · ${r.targets} lights, ${r.dying} dying tube(s)`);
  line(r.shot >= 1 && r.brokenBefore >= 1 && !r.holstered, 'the player draws his pistol and shoots a lamp out: a fixture reads broken two seconds later', `broken after the shots ${r.shot} (${r.lamp}, ${r.tried} lamp(s) tried) · broken before the key ${r.brokenBefore} at ${r.timeBefore} s · the game's count of lights shot ${r.lightsShot}`);
  line(after.length === 6 && after[0].t <= 1 && r.time < 0.2 && r.fromSpawn < 0.5, 'Shift+Enter restarts the mission: the game\'s clock is back at the start and the man at his spawn', `the clock on the six frames after the key (ticks): ${r.clockAfter} · ${r.time} s now · ${r.fromSpawn} m from the spawn`);
  line(r.broken === 0 && r.beacons === 0 && r.said.length === 0 && r.mains && r.mains.active === false && r.mains.restoredAt === -100, 'the level is a fresh one: no fixture broken, the mains as a boot has them, no beacon turning, nothing said of it', `broken ${r.broken} · beacons ${r.beacons} · mains ${JSON.stringify(r.mains)} · said ${JSON.stringify(r.said)}`);
  line(matched >= 4 && off === 0, 'every light (the dying tubes apart) is at the factor a fresh boot had at the same game time, on every frame after the restart', `${matched} frame(s) matched by the clock · ${off} factor(s) off, worst ${r.worst}${firstOff.length ? ' · ' + firstOff.join(' | ') : ''}`);
  line(Math.abs(r.cam - r.camBoot) < 0.25, 'the camera stands by the man as on a fresh boot', `${r.cam} m from him now, ${r.camBoot} m at the boot`);
  line(!!r.drawn, 'three frames after the restart draw', `drawn ${r.drawn}`);
  line(errs.length === 0, 'no page error', errs.slice(0, 4).join(' | ') || 'none');
  console.log(`=== cleanrestart-page: ${red ? 'FAILED' : 'ALL PASS'} ===`);
  process.exit(red ? 1 : 0);
})();
