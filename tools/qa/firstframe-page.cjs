// THE FIRST PLAYED FRAME IS AN ORDINARY ONE ON EVERY ROAD TO IT: a page with no held card (a benchmark's address, a page opened with ?mute) stands the Rust game and warms it before the loop's first frame, so no frame of play holds the whole start.
//   node tools/qa/firstframe-page.cjs     env: PORT (the dev server's, default 5199) · PLAYWRIGHT_MODULE, CHROMIUM (where they are, if not found by name) · FIRSTFRAME_BREAK=1 opens the page with ?rustwarm=0, and the check must fail
// On the software adapter: its pipelines take minutes under load, and every ms here is the CPU's. Exit 0 pass · 1 a line failed.
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const PORT = process.env.PORT || 5199, BREAK = process.env.FIRSTFRAME_BREAK === '1', Q = process.argv[2] || 'nightbreach', FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--no-sandbox'];
let red = 0; const line = (ok, what) => { console.log(`${ok ? 'PASS' : 'FAIL'}: ${what}`); if (!ok) red++; };
(async () => { const b = await chromium.launch({ headless: true, ...(process.env.CHROMIUM ? { executablePath: process.env.CHROMIUM } : {}), args: FLAGS }), p = await b.newPage({ viewport: { width: 320, height: 180 } });
  // (the tick as the page's own loop runs it: timed from the page's first line, so no tick goes untimed)
  await p.addInitScript(() => { window.__T = []; const t = setInterval(() => { const g = window.__game, r = window.__rust; if (!g || !r || !g.stepSeats || g.__timed) return; g.__timed = true; clearInterval(t); window.__atHook = { starts: r.starts, ticks: r.ticks }; const ss = g.stepSeats; g.stepSeats = function (...a) { const t0 = performance.now(), s0 = r.starts, R = ss.apply(this, a); window.__T.push({ ms: performance.now() - t0, started: r.starts - s0 }); return R; }; }, 0); });
  await p.goto(`http://localhost:${PORT}/?${Q}&keeppage=0&quality=low&mute${BREAK ? '&rustwarm=0' : ''}`, { timeout: 180000 });
  await p.waitForFunction(() => window.__started === true, null, { timeout: 900000 });
  await p.waitForFunction(() => (window.__T?.length ?? 0) >= 12, null, { timeout: 300000 });
  const R = await p.evaluate(() => ({ T: window.__T.slice(0, 12), hook: window.__atHook, warm: window.__rust.warm, starts: window.__rust.starts, parts: window.__rust.startParts, startMs: window.__rust.startMs }));
  console.log(`the start ${R.startMs.toFixed(0)} ms: ${Object.entries(R.parts).map(([k, v]) => `${k} ${v.toFixed(0)}`).join(' · ')}; warmed ${R.warm.n} (${R.warm.applyMs.toFixed(0)} ms of applies, ${R.warm.tickMs.toFixed(0)} ms of ticks, the first ${R.warm.firstTickMs.toFixed(0)}, the last ${R.warm.lastTickMs.toFixed(0)})`);
  console.log(`first ticks, ms: ${R.T.map(x => x.ms.toFixed(1) + (x.started ? '*' : '')).join(' ')}`);
  const sorted = R.T.map(x => x.ms).sort((a, b) => a - b), median = sorted[sorted.length >> 1], first = R.T[0];
  line(R.warm.n === 1, 'the game was warmed before the first frame');
  line(!R.T.some(x => x.started), 'no tick of play holds a start');
  line(first.ms < Math.max(R.startMs * 0.5, median * 6), `the first tick is no start (${first.ms.toFixed(1)} ms, the start ${R.startMs.toFixed(0)}, the median ${median.toFixed(1)})`);
  await b.close(); console.log(`=== firstframe-page: ${red ? `${red} lines fail` : 'all pass'} ===`); process.exit(red ? 1 : 0); })().catch(e => { console.error(e); process.exit(1); });
