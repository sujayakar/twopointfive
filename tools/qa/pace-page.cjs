// THE PAGE'S PACE ON A LIVE FRAME LOOP (src/host/pace.ts, main.ts frame()) — the real page in headless Chromium on the software WebGPU adapter, NOT paused and
// not driven: its own frame callbacks run its own loop, which is the one place the pacer acts (every other page tool opens the page `?paused` and steps it with
// __runFrames, which never meets the pacer: tools/qa/pace-check.ts plays the pacer's rules on stamps, this asks that the page built round them stays alive).
//   1. the page is what a person opens: no driven browser, the frame loop not paused
//   2. the rules are on as the page ships them: the even pace, giving way, and the wait for the GPU at the page's number
//   3. it comes ready, and over WATCH_S seconds after that (up to three times that, until two were worked) frames keep being worked, frames are seen with
//      the GPU, and never more of them than the rule allows
//   4. A TAB THAT SLEEPS: the browser hides and freezes the page for SLEEP_S seconds with frames in flight and wakes it. The page's own `freeze` and
//      `resume` events say that it slept and that it worked no frame asleep; awake it draws on, with no more frames with the GPU than the rule allows
//   5. no page error
//   PORT=5199 node tools/qa/pace-page.cjs        env: WATCH_S (240) · SLEEP_S (15) · QUALITY (verylow) · CHROMIUM (/usr/bin/chromium)
//   PACE_PAGE_BREAK=1 …   the page is opened with ?flight=0 (nothing waits for the GPU, as it was): lines 2 and 3 must FAIL
// THE BROWSER IS SPOKEN TO OVER ITS DEBUGGING PORT, with no driver library, and that is the point of line 4: under the driver the other page tools use a page
// is held focused and visible for good (its focus emulation), and none of hiding the tab, minimising the window or the protocol's freeze puts it to sleep
// there: frame callbacks run on and no `freeze` event comes (tried on a plain page, 2026-11-03). Here the page is kept in front by the same emulation
// while it is watched, and the sleep is: the emulation off, Page.setWebLifecycleState frozen (the page reads hidden, its frame callbacks stop), and back.
// What this is NOT: a pace. A frame takes seconds on the software adapter and the first ones after ready minutes (flight on or off), longer than the ten
// seconds the pacer waits for an answer, so here the wait mostly ends in a frame given up and the page handing over the next as it always did; whether
// frames come EVENLY on a real GPU is nobody's to say from here. It is a check that a page which waits on the GPU's word boots, keeps drawing and wakes.
// Nine minutes to half an hour. exit 0 = every line passed · 1 = one failed · 2 = the browser or the page could not be opened
const { spawn } = require('child_process'), fs = require('fs'), os = require('os'), path = require('path');
const PORT = process.env.PORT || 5199, BREAK = process.env.PACE_PAGE_BREAK === '1', WATCH_S = Number(process.env.WATCH_S || 240), SLEEP_S = Number(process.env.SLEEP_S || 15), QUALITY = process.env.QUALITY || 'verylow';
const FLAGS = ['--headless=new', '--remote-debugging-port=0', '--no-first-run', '--window-size=640,360', '--enable-unsafe-webgpu', '--use-gl=angle', '--use-angle=swiftshader', '--enable-features=Vulkan', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--disable-blink-features=AutomationControlled'];
const URL = `http://localhost:${PORT}/?cloud=0&mute&quality=${QUALITY}${BREAK ? '&flight=0' : ''}`;
const T0 = Date.now(), log = (...a) => console.log(`  [${((Date.now() - T0) / 1000).toFixed(0)}s]`, ...a), sleep = ms => new Promise(r => setTimeout(r, ms));
let red = 0, shut = () => { }; const line = (ok, what) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) red++; };
const NOISE = /Failed to load resource/;

(async () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'pace-page-')), browser = spawn(process.env.CHROMIUM || '/usr/bin/chromium', [...FLAGS, `--user-data-dir=${dir}`, 'about:blank'], { stdio: 'ignore' });
  shut = () => browser.kill();
  const end = async code => { browser.kill(); await sleep(500); fs.rmSync(dir, { recursive: true, force: true }); process.exit(code); };
  // (port 0 = the browser picks one and writes it down in its profile)
  let at = ''; for (let i = 0; i < 100 && !at; i++) { await sleep(200); try { const [p, w] = fs.readFileSync(path.join(dir, 'DevToolsActivePort'), 'utf8').split('\n'); if (w) at = `ws://127.0.0.1:${p}${w}`; } catch { } }
  if (!at) { console.error('the browser did not open its debugging port'); await end(2); }
  const ws = new WebSocket(at); await new Promise((ok, no) => { ws.addEventListener('open', ok); ws.addEventListener('error', no); });
  let id = 0; const waits = new Map(), errs = [];
  ws.addEventListener('message', m => { const o = JSON.parse(m.data);
    if (o.id && waits.has(o.id)) { waits.get(o.id)(o); waits.delete(o.id); }
    else if (o.method === 'Runtime.exceptionThrown') errs.push(`page error: ${String(o.params.exceptionDetails.exception?.description ?? o.params.exceptionDetails.text).slice(0, 200)}`);
    else if (o.method === 'Runtime.consoleAPICalled' && o.params.type === 'error') { const t = o.params.args.map(a => a.value ?? a.description ?? '').join(' ').slice(0, 200); if (!NOISE.test(t)) errs.push(t); } });
  const send = (method, params = {}, sessionId) => new Promise((ok, no) => { const i = ++id; waits.set(i, o => o.error ? no(new Error(`${method}: ${o.error.message}`)) : ok(o.result)); ws.send(JSON.stringify({ id: i, method, params, sessionId })); });
  const { targetId } = await send('Target.createTarget', { url: 'about:blank' }), { sessionId } = await send('Target.attachToTarget', { targetId, flatten: true }), page = (m, p) => send(m, p, sessionId);
  await page('Runtime.enable'); await page('Page.enable'); await page('Emulation.setFocusEmulationEnabled', { enabled: true });
  await page('Page.addScriptToEvaluateOnNewDocument', { source: 'window.__testVisible = true;' });   // (the quality ladder's reading of a headless page; nothing of the pacer's)
  // (the page a person opens is a shell with the game in a frame of the same origin: the hooks are the frame's, and the shell can reach them)
  const G = `(document.getElementById('kept-page')?.contentWindow ?? window)`, ask = async x => { const r = await page('Runtime.evaluate', { expression: `(() => { const g = ${G}; return ${x}; })()`, returnByValue: true }); if (r.exceptionDetails) throw new Error(r.exceptionDetails.exception?.description ?? r.exceptionDetails.text); return r.result.value; };
  log(URL); await page('Page.navigate', { url: URL });
  let ready = false; for (let i = 0; i < 1800 && !ready; i++) { await sleep(500); ready = await ask(`!!(g.__boot && g.__boot.marks && g.__boot.marks.interactive && g.__pace)`).catch(() => false); }
  if (!ready) { line(false, 'the page never came ready'); await end(1); }
  log('ready');
  const read = () => ask(`(p => ({ on: p.on, giveway: p.giveway, flight: p.flight, flying: p.flying, held: p.held, lost: p.lost, gave: p.gave, worked: p.worked, attended: p.attended }))(g.__pace())`);
  const who = await ask(`({ driven: g.navigator.webdriver === true, paused: g.__pauseLoop === true })`);
  line(!who.driven && !who.paused, `the page is what a person opens: no driven browser (${!who.driven}), its frame loop not paused (${!who.paused})`);
  const p0 = await read();
  line(p0.on === true && p0.giveway === true && p0.flight === 2 && p0.attended === true, `the rules are on as the page ships them: the even pace ${p0.on}, giving way ${p0.giveway}, at most ${p0.flight} frames with the GPU when another is begun; somebody is at the window (${p0.attended})`);
  // the watch (frames come very unevenly on this adapter, minutes between two just after ready: it runs on, to three times its length, until two were worked)
  let most = 0, last = p0, watched = 0; for (; watched < WATCH_S || (watched < 3 * WATCH_S && last.worked - p0.worked < 2); watched++) { await sleep(1000); last = await read().catch(() => last); if (last.flying > most) most = last.flying; if (watched % 60 === 59) log(JSON.stringify(last)); }
  line(last.worked - p0.worked >= 2 && most >= 1 && most <= 2, `over ${watched} s after it came ready: ${last.worked - p0.worked} more frames worked, ${most} with the GPU at the most (never over two), ${last.held - p0.held} display frames held for it, ${last.lost} given up unanswered`);
  // a tab that sleeps, with frames in flight (the page writes down its own freeze and resume, and the frames it had worked at each)
  { await ask(`(() => { const z = g.__slept = { froze: 0, woke: 0, at: -1, back: -1 }; g.document.addEventListener('freeze', () => { z.froze++; z.at = g.__pace().worked; }); g.document.addEventListener('resume', () => { z.woke++; z.back = g.__pace().worked; }); return 1; })()`);
    const before = await read();
    await page('Emulation.setFocusEmulationEnabled', { enabled: false }); await page('Page.setWebLifecycleState', { state: 'frozen' }); await sleep(SLEEP_S * 1000);
    const asleep = await ask(`({ ...g.__slept, hidden: g.document.visibilityState === 'hidden' })`);
    await page('Page.setWebLifecycleState', { state: 'active' }); await page('Emulation.setFocusEmulationEnabled', { enabled: true });
    const woke = await read(), z = await ask(`g.__slept`); let after = woke, most2 = woke.flying;
    for (let k = 0; k < 3 * WATCH_S && after.worked - woke.worked < 2; k++) { await sleep(1000); after = await read().catch(() => after); if (after.flying > most2) most2 = after.flying; }
    line(asleep.froze === 1 && asleep.woke === 0 && asleep.hidden && z.woke === 1 && z.back === z.at && after.worked - woke.worked >= 2 && most2 <= 2, `a tab that sleeps: hidden (${asleep.hidden}) and frozen (${z.froze}) for ${SLEEP_S} s with ${before.flying} frame(s) with the GPU, it worked none asleep (${z.at} worked at the freeze, ${z.back} at the resume); awake (${z.woke}) with ${woke.flying} in flight it worked ${after.worked - woke.worked} more (${most2} with the GPU at the most; ${after.lost - before.lost} given up over the sleep)`); }
  line(errs.length === 0, `no page error${errs.length ? `: ${errs.slice(0, 3).join(' | ')}` : ''}`);
  console.log(red ? `=== pace-page: FAILED (${red}) ===` : '=== pace-page: ALL PASS ===');
  ws.close(); await end(red ? 1 : 0);
})().catch(e => { console.error(e); shut(); process.exit(2); });
