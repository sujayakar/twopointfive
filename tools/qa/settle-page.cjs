// THE START WAITS FOR THE PICTURE ON THE REAL PAGE (src/ui/settle.ts, main.ts briefSettle): headless Chromium on the software WebGPU adapter, in a browser
// that does not say a script drives it (the switch is off on a driven page), so the game is in the kept page's frame as a visitor's is.
//   PORT=5199 node tools/qa/settle-page.cjs      (a dev server on that port: PORT=5199 bun run tools/serve.ts)
// One mission's address with `?settle=2&settlewait=3600` (two drawn frames are the picture's turn and the programs get an hour: the software adapter takes
// minutes over a frame and over the second set) and a pinned preset:
//   · the level stands (`__boot.marks.interactive`): the brief is up, its block NOT live, its line says what it waits for, the game's clock still; START
//     pressed there is kept (the block sits pressed in) and starts nothing
//   · `__boot.marks.settled` comes with the second set built (`__cut.programs.state` ready) and no pipeline out: nothing is compiling when the run goes
//     live; the kept press starts it then (the brief gone)
// exit 0 = every line, with no page error · 1 = otherwise. SETTLE_BREAK=1 opens the address under `?settle=0` (live the moment the level stands): must FAIL.
// The compile queue is off on this adapter, so its rush is not seen here (tools/qa/pipequeue-check.ts has the queue's own logic). The DOM, the page's own
// marks and the software adapter's compiles: nothing here has been seen on a GPU, and no picture is looked at.
const { chromium } = require('/opt/node-tools/node_modules/playwright');
const PORT = process.env.PORT || 5199, BREAK = process.env.SETTLE_BREAK === '1', WAIT = (Number(process.env.WAIT) || 3600) * 1000;
const FLAGS = ['--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--disable-blink-features=AutomationControlled'];
const T0 = Date.now(), log = (...a) => console.log(`[${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a);
let red = 0; const line = (ok, what) => { log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const ADDRESS = `http://localhost:${PORT}/?levelFile=depot.json&roomclear&squad4&quality=verylow&seed=7&gpuretry=0&cloud=0&${BREAK ? 'settle=0' : 'settle=2&settlewait=3600'}`;
const sleep = ms => new Promise(r => setTimeout(r, ms));

/** in the game's own window: what the brief, the boot's marks and the compiles say */
const look = () => { const B = document.getElementById('brief'), P = window.__cut?.programs, m = window.__boot.marks;
  return { up: !!B && !B.classList.contains('out'), live: !!B && B.classList.contains('live'), armed: !!B && B.classList.contains('armed'), status: B?.querySelector('.bstatus')?.textContent ?? '',
    time: window.__game?.time ?? -1, cut: P?.state ?? '?', interactive: m.interactive ?? null, later: m.laterReady ?? null, settled: m.settled ?? null, drawn: window.__behind?.drawn ?? -1 }; };

(async () => {
  const b = await chromium.launch({ args: FLAGS });
  const page = await (await b.newContext({ viewport: { width: 1280, height: 720 } })).newPage();
  const errs = [], err = t => { errs.push(t.slice(0, 300)); log('page error:', t.slice(0, 300)); };
  page.on('pageerror', e => err(String(e))); page.on('console', m => { if (m.type() === 'error') err(m.text()); });
  await page.goto(ADDRESS);
  /** the window the game boots in: the kept page's frame, or the tab itself (the shell that keeps the frame has a `__boot` of its own and no game) */
  const game = async (fn) => { for (const f of page.frames()) { try { if (await f.evaluate(() => { try { return !!window.__boot?.marks && !document.getElementById('kept-page'); } catch { return false; } })) return await f.evaluate(fn); } catch { /* a frame on its way */ } } return null; };
  /** what the page's own boot line says, whichever window it is in (the brief's, else the start card's): a slow adapter's wait is not a silent one */
  const says = async () => { for (const f of page.frames()) { try { const t = await f.evaluate(() => (document.querySelector('#brief .bstatus') ?? document.querySelector('#boot-status'))?.textContent ?? ''); if (t) return t; } catch { /* a frame on its way */ } } return ''; };
  const until = async (key, what) => { const t0 = Date.now(); let told = t0; for (;;) { const v = await game(look); if (v && v[key] !== null) return v; if (Date.now() - t0 > WAIT) { line(false, `${what}: not in ${WAIT / 1000} s`); return null; }
    if (Date.now() - told > 120000) { told = Date.now(); log(`waiting for ${what}: "${await says()}"${v ? ` · cut ${v.cut}, drawn ${v.drawn}` : ''}`); } await sleep(1000); } };

  const a = await until('interactive', 'the level stands');
  if (a) {
    log('at the level standing:', JSON.stringify(a));
    line(a.up && !a.live && a.settled === null && /compiling shaders \d+ \/ \d+|getting the picture ready/.test(a.status), `the level stands and the brief's block is not live; its line says what it waits for ("${a.status}")`);
    await game(() => document.querySelector('#brief .bgo')?.click());
    await sleep(1500);
    const p = await game(look);
    line(!!p && p.up && p.armed && p.settled === null && p.time === a.time, `START pressed while it waits is kept and starts nothing: the game's clock has not moved (${p?.time})`);
    const s = await until('settled', 'the start goes live');
    if (s) {
      log('at the start going live:', JSON.stringify(s));
      line(s.cut === 'ready' && s.later !== null && s.later <= s.settled && s.settled > s.interactive, `live only with every program in: the second set is ${s.cut}, the last program was in ${s.later !== null ? ((s.settled - s.later) / 1000).toFixed(1) : '?'} s before, ${((s.settled - s.interactive) / 1000).toFixed(1)} s after the level stood`);
      line(s.drawn >= 2, `the picture had its frames behind the brief (${s.drawn} drawn)`);
      await sleep(1500);
      const g = await game(look);
      line(!!g && !g.up, 'the press that was kept started the run: the brief is gone');
    }
  }
  line(errs.length === 0, `no page error (${errs.length}${errs.length ? ': ' + errs[0] : ''})`);
  await b.close();
  log(`=== settle-page: ${red ? `${red} FAILED` : 'ALL PASS'} ===`);
  process.exit(red ? 1 : 0);
})();
