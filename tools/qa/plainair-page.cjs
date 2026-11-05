// PLAIN AIR (src/gpu/device.ts PLAIN_AIR), on the real page in headless Chromium, software WebGPU adapter: a load under `?plainair=1` must hand the browser the plain
// volumetric program (shaders/vol_plain.wgsl) and no text of the real march (the pass's own program, its froxel twin, its dust twin, its K = 0 twin), come ready with
// every program made and nothing refused, say so in the warning the records keep, and still DRAW smoke: a canister's plume shows in the volumetric pass's own picture.
//   node tools/qa/plainair-page.cjs        env: PORT (the dev server's, default 5199) · PLAINAIR_SMOKE_S (seconds the canister is let grow, default 1.5)
//                                          PLAINAIR_SHOTS (a folder: the two pictures after the canister are written there as PNGs)
// What this is NOT: a driver that stops on the real program (whether leaving it out gets such a machine into the game is its own records' to say), a note written by
// a lost device (tools/qa/gpulost-check.ts has the rule), or the look of the plain plume beside the real one (stills by hand; never seen on a GPU).
// Exit 0 = every check passed, 1 = one failed or the page did not get that far.   PLAINAIR_BREAK=1: the page runs under ?plainair=0 — must FAIL.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, BREAK = process.env.PLAINAIR_BREAK === '1', GROW_S = Number(process.env.PLAINAIR_SMOKE_S || 1.5), SHOTS = process.env.PLAINAIR_SHOTS || '';
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const check = (ok, what) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const URL = `http://localhost:${PORT}/?paused&quality=verylow&mute&quiet&keeppage=0&settle=0&cloud=0&plainair=${BREAK ? 0 : 1}`;
const REAL = ['volumetrics', 'volumetricsF', 'volumetricsDust', 'volumetricsK0'];

(async () => {
  const browser = await chromium.launch({ headless: true, channel: 'chromium', args: FLAGS });
  const ctx = await browser.newContext({ viewport: { width: 320, height: 180 }, deviceScaleFactor: 1 });
  // every shader module and pipeline the page asks the browser for, by label (beneath the page's own wrappers: they call these)
  await ctx.addInitScript(() => {
    window.__testVisible = true; window.__asked = { modules: [], pipes: [] };
    const P = GPUDevice.prototype, lab = d => String(d?.label ?? '?');
    const mod = P.createShaderModule; P.createShaderModule = function (d) { window.__asked.modules.push(lab(d)); return mod.call(this, d); };
    for (const k of ['createComputePipeline', 'createComputePipelineAsync', 'createRenderPipeline', 'createRenderPipelineAsync']) { const was = P[k]; P[k] = function (d) { window.__asked.pipes.push(lab(d)); return was.call(this, d); }; }
  });
  const page = await ctx.newPage(); const said = [];
  page.on('console', m => said.push({ type: m.type(), text: m.text().slice(0, 400) }));
  log(URL); await page.goto(URL, { waitUntil: 'commit' });
  const ready = await page.waitForFunction(() => !!window.__ready, null, { timeout: 600000 }).then(() => page.evaluate(() => Promise.race([window.__ready.then(() => true), new Promise(r => setTimeout(() => r(false), 900000))]))).catch(() => false);
  if (!ready) { check(false, 'the page never came ready'); await browser.close(); process.exit(1); }
  const boot = await page.evaluate(() => ({ p: window.__boot.pipelines, asked: window.__asked }));
  log(`ready: ${boot.p.done} of ${boot.p.total} programs, ${boot.p.failed} failed; ${boot.asked.modules.length} modules asked for`);
  check(boot.p.failed === 0 && boot.p.done === boot.p.total, `the load came ready with every program of its start made (${boot.p.done} of ${boot.p.total}, ${boot.p.failed} failed)`);
  const plain = boot.asked.modules.filter(m => m === 'volumetricsPlain').length, real = boot.asked.modules.filter(m => REAL.includes(m));
  check(plain >= 1, `the browser was handed the plain program's text (${plain} module${plain === 1 ? '' : 's'} named volumetricsPlain)`);
  check(real.length === 0, `and no text of the real march (${real.join(', ') || 'none'})`);
  check(boot.asked.pipes.includes('vol') && !boot.asked.pipes.some(l => l === 'volFroxel' || l === 'volDust' || l === 'volK0'), `the pass has its program under its own label and none of its twins was asked for (${boot.asked.pipes.filter(l => /^vol/.test(l)).join(', ')})`);
  const warn = said.find(s => /smoke and fog are drawn the plain way on this load/.test(s.text));
  check(!!warn && warn.type === 'warning', `it prints the warning the records keep (${warn ? warn.text.slice(0, 160) : 'none'})`);
  if (red) { console.log('(the smoke is not looked at: the load is not in plain air)'); await browser.close(); console.log(`${red} FAILED`); process.exit(1); }
  // smoke is drawn: the final picture and the volumetric pass's own (debug view 5), twice before a canister (what the picture moves by itself) and once after it has grown
  const snap = name => page.evaluate(async (name) => {
    const E = window.__engine, S = E.settings, was = S.debugView, out = {};
    for (const view of [0, 5]) { S.debugView = view; E.render(window.__cam, window.__game.time, 1 / 60, false); await E.device.queue.onSubmittedWorkDone(); const fr = await E.readbackFrame(); out[view] = fr ? Array.from(fr.data) : null; if (fr) { out.w = fr.w; out.h = fr.h; out.bgra = fr.bgra; } }
    S.debugView = was; (window.__snaps ??= {})[name] = out; return !!out[0] && !!out[5];
  }, name);
  const moved = (a, b) => page.evaluate(([a, b]) => { const A = window.__snaps[a], B = window.__snaps[b], r = {}; for (const v of [0, 5]) { let n = 0; const x = A[v], y = B[v]; for (let i = 0; i < x.length; i += 4) if (Math.abs(x[i] - y[i]) + Math.abs(x[i + 1] - y[i + 1]) + Math.abs(x[i + 2] - y[i + 2]) > 36) n++; r[v] = n; } r.of = A[0].length / 4; return r; }, [a, b]);
  await page.evaluate(async () => {
    if (window.__menu && window.__menu.mode !== 'closed') window.__menu.close();
    const g = window.__game, pl = g.player.char.pos; window.__engine.passes.grain = 0; g.godMode = true; g.aiEnabled = false;
    window.__can = [pl[0] + 1.5, 0, pl[2]];
    window.__stage({ look: [window.__can[0], 0.6, window.__can[2]], yaw: 0.4, pitch: 0.95, dist: 9 }, { park: true, guards: [], player: { pos: [pl[0], 0, pl[2]], yaw: 0, holstered: true } });
    for (let i = 0; i < 12; i++) { window.__runFrames(1, 1 / 60, []); await window.__engine.device.queue.onSubmittedWorkDone(); }
  });
  const ok1 = await snap('a'); await page.evaluate(async () => { for (let i = 0; i < 6; i++) { window.__runFrames(1, 1 / 60, []); await window.__engine.device.queue.onSubmittedWorkDone(); } }); const ok2 = await snap('b');
  await page.evaluate(async (frames) => { window.__smoke.spawnCanister(window.__can); for (let i = 0; i < frames; i++) { window.__runFrames(1, 1 / 60, []); if ((i & 3) === 3) await window.__engine.device.queue.onSubmittedWorkDone(); } await window.__engine.device.queue.onSubmittedWorkDone(); }, Math.round(GROW_S * 60));
  const ok3 = await snap('c');
  if (SHOTS && ok3) for (const v of [0, 5]) {   // (the picture and the volumetric one after the canister, for a person to look at)
    const b64 = await page.evaluate(v => { const S = window.__snaps.c, d = Uint8ClampedArray.from(S[v]), c = document.createElement('canvas'); if (S.bgra) for (let i = 0; i < d.length; i += 4) { const b = d[i]; d[i] = d[i + 2]; d[i + 2] = b; } c.width = S.w; c.height = S.h; c.getContext('2d').putImageData(new ImageData(d, S.w, S.h), 0, 0); return c.toDataURL('image/png').split(',')[1]; }, v);
    require('fs').writeFileSync(`${SHOTS}/plainair-${BREAK ? 'real' : 'plain'}-view${v}.png`, Buffer.from(b64, 'base64'));
  }
  const still = ok1 && ok2 ? await moved('a', 'b') : null, smoke = ok1 && ok3 ? await moved('a', 'c') : null;
  log(`texels that moved, of ${still ? still.of : '?'}: with nothing thrown ${still ? `${still[0]} in the picture, ${still[5]} in the volumetric one` : '?'}; after ${GROW_S} s of a canister ${smoke ? `${smoke[0]}, ${smoke[5]}` : '?'}`);
  check(!!still && !!smoke && Math.min(smoke[0] - still[0], smoke[5] - still[5]) > smoke.of * 0.004, `a canister's plume shows in the picture AND in the volumetric pass's own (${still && smoke ? `${smoke[0] - still[0]} and ${smoke[5] - still[5]} more texels moved than with nothing thrown` : 'no picture read'})`);
  const bad = said.filter(s => /WebGPU error|GPU device lost|\[pipeline\]|Invalid (ShaderModule|ComputePipeline|BindGroup)/.test(s.text));
  check(bad.length === 0, `no GPU error was printed (${bad.length ? bad[0].text.slice(0, 200) : 'none'})`);
  check(!(await page.evaluate(() => { const f = document.getElementById('fatal'); return !!f && f.style.display !== 'none' && !!f.textContent; })), 'and no red card is up');
  await browser.close();
  console.log(red ? `${red} FAILED` : 'ALL PASS');
  process.exit(red ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
