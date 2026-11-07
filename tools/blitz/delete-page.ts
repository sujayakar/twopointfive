// THE PAGE UNDER THE RUST GAME, DRIVEN (tools/blitz/DELETION.md): headless Chromium on the software adapter loads the dev server's pages and plays them by the
// page's own hooks (`__ready`, `__runFrames`, `__menu`, `__scenarios`, `__orders`: src/main.ts). It is the deletion's check that the page still boots and
// plays, and, on the census's tree, the census of what the page runs as TypeScript and how often a frame. NEVER SEEN ON A GPU, AND NOT DRAWN: the frames it steps are undrawn (the software adapter needs minutes for some drawn frames),
// so nothing here says anything of the picture; what is held is that the page reaches play, the Rust game steps, and no error is said.
//   PORT=5260 bun run tools/serve.ts &                                   the tree's dev server
//   bun run tools/blitz/delete-page.ts --at http://localhost:5260 [--word rust=1] [--frames 3600] [--census ran.jsonl] [--only solo,squad,title,pages] [--squad room-clear-four]
// Exit 0: every flow reached play and no page said an error · 1: one did not.
import { appendFileSync } from 'node:fs';
import type { Cdp as CdpT } from '../qa/cdp.ts';

const A = process.argv.slice(2), opt = (n: string, d?: string) => { const i = A.indexOf(n); return i < 0 ? d : A[i + 1]; };
const AT = opt('--at', 'http://localhost:5260')!, WORD = opt('--word', 'rust=1')!, FRAMES = Number(opt('--frames', '3600')), CENSUS = opt('--census'), ONLY = new Set((opt('--only', 'solo,squad,title,pages')!).split(',')), SQUAD = opt('--squad', 'room-clear-four')!;
process.env.CHROME ??= '/usr/bin/chromium';   // (read by cdp.ts as it loads)
const { Cdp } = await import('../qa/cdp.ts'); type Cdp = CdpT;
const FLAGS = ['--no-sandbox', '--use-gl=angle', '--use-angle=swiftshader', '--use-vulkan=swiftshader', '--ignore-gpu-blocklist', '--disable-gpu-watchdog', '--enable-features=Vulkan'];
const COUNTER = `window.__tpsN = new Map(); window.__tpsRan = (id) => { window.__tpsN.set(id, (window.__tpsN.get(id) || 0) + 1); };`;
const word = (q: string) => [q, WORD].filter(Boolean).join('&');
let bad = 0; const say = (s: string) => console.log(s);

async function withPage<T>(port: number, f: (c: Cdp) => Promise<T>): Promise<T> { const c = await Cdp.launch({ port, width: 1280, height: 800, dpr: 1, extraArgs: FLAGS }); try { if (CENSUS) await c.send('Page.addScriptToEvaluateOnNewDocument', { source: COUNTER }); return await f(c); } finally { await c.close(); } }
/** the page's own errors, but for the ones a headless browser makes of itself */
const errorsOf = (c: Cdp) => c.errors.filter(e => !/favicon|ERR_FILE_NOT_FOUND.*\.map|AudioContext|autoplay/i.test(e));
async function census(c: Cdp, what: string, frames: number): Promise<void> { if (!CENSUS) return; const ran = await c.eval<[number, number][]>('[...(window.__tpsN ?? [])]'); appendFileSync(CENSUS, JSON.stringify({ page: true, argv: what, frames, ran }) + '\n'); await c.eval('window.__tpsN && window.__tpsN.clear()'); }
const state = `(() => { const g = window.__game, r = window.__rust; return { time: g ? +g.time.toFixed(2) : null, rust: r ? { on: r.on, ticks: r.ticks, starts: r.starts } : null, player: g ? { down: !!g.player?.down, x: +g.player.char.pos[0].toFixed(2), z: +g.player.char.pos[2].toFixed(2) } : null, guards: g ? g.guards.length : null, fatal: document.querySelector('.fatal, #fatal')?.textContent?.slice(0, 300) ?? null }; })()`;
/** frames in bursts, each with the keys its row holds; a row may also run a line of the page's own */
async function play(c: Cdp, rows: { n: number; keys?: string[]; js?: string }[]): Promise<number> { let done = 0;
  for (const r of rows) { if (r.js) await c.eval(`(async () => { ${r.js} })()`, { timeoutMs: 60000 }); for (let left = r.n; left > 0; left -= 30) { const k = Math.min(30, left); await c.eval(`window.__runFrames(${k}, 1 / 60, ${JSON.stringify(r.keys ?? [])}, 'none')`, { timeoutMs: 600000 }); /* (UNDRAWN: one drawn frame of the solo night takes minutes on the software adapter, with or without the Rust game) */ done += k; } note(`${done} frames`); }
  return done; }
function verdict(name: string, c: Cdp, before: any, after: any, frames: number): void { const errs = errorsOf(c), stepped = after.time !== null && before.time !== null && after.time > before.time, rustOk = !WORD || (after.rust?.on && after.rust.ticks > 0);
  const ok = stepped && rustOk && !errs.length && !after.fatal; if (!ok) bad++;
  say(`${ok ? 'PASS' : 'FAIL'}  ${name}: ${frames} frames, the game's clock ${before.time} → ${after.time} s${WORD ? `, the Rust game ${after.rust?.on ? `on, ${after.rust.ticks} ticks, ${after.rust.starts} starts` : 'NOT ON'}` : ''}, ${errs.length} errors${after.fatal ? `, the fatal card: ${after.fatal}` : ''}`);
  for (const e of errs.slice(0, 8)) say(`      ${e.slice(0, 400)}`); }
const T0 = Date.now(), note = (m: string) => process.stderr.write(`  [${((Date.now() - T0) / 1000).toFixed(0)} s] ${m}\n`);
const ready = async (c: Cdp) => { await c.eval(`window.__ready.then(() => { window.__testVisible = true; return 1; })`, { timeoutMs: 1200000 }); note('the page is ready'); };

// ---- the solo night, from the start card: walk, crouch, run, shoot, the kit's keys, a throw, the objectives, the debugger
if (ONLY.has('solo')) await withPage(9361, async c => { const url = `${AT}/?${word('paused&quality=verylow&mute')}`; await c.open(url, 120000); await ready(c);
  await c.eval(`window.__menu.close()`); const before = await c.eval(state);
  const n = await play(c, [{ n: 240, keys: ['KeyW'] }, { n: 180, keys: ['KeyD'] }, { n: 180, keys: ['KeyS'] }, { n: 180, keys: ['KeyA'] }, { n: 120, keys: ['KeyC'] }, { n: 180, keys: ['KeyW', 'ShiftLeft'] },
    { n: 60, js: `window.__mouse(700, 300); window.__click(0);` }, { n: 60, js: `window.__click(0);` }, { n: 120, keys: ['Digit1'] }, { n: 60, js: `window.__click(0);` }, { n: 60, js: `window.__click(2);` },
    { n: 60, keys: ['Digit3'] }, { n: 240, js: `window.__click(0);` }, { n: 60, keys: ['Digit4'] }, { n: 240, js: `window.__click(0);` }, { n: 60, keys: ['Digit2'] }, { n: 60, keys: ['KeyR'] }, { n: 60, keys: ['Tab'] }, { n: 60, keys: ['KeyE'] }, { n: 60, keys: ['Space'] },
    { n: 60, keys: ['Backslash'] }, { n: 60, keys: ['Backslash'] }, { n: Math.max(60, FRAMES - 2520), keys: ['KeyW'] }]);
  const after = await c.eval(state); verdict('the solo night', c, before, after, n); await census(c, `the solo night ${url}`, n); });

// ---- a squad mission, by its own address: the squad's keys, the orders menu and a pick of it
if (ONLY.has('squad')) await withPage(9362, async c => { await c.open(`${AT}/?${word('paused&quality=verylow&mute')}`, 120000); await ready(c);
  const to = await c.eval<string | null>(`(() => { const e = window.__scenarios.all.find(x => x.id === ${JSON.stringify(SQUAD)}); if (!e) return null; window.__scenarios.pick(e.id); return location.href; })()`);
  if (to === null) { bad++; say(`FAIL  the squad mission: no mission '${SQUAD}'`); return; }
  await new Promise(r => setTimeout(r, 1500)); for (let i = 0; i < 600; i++) { try { if (await c.eval('document.readyState === "complete" && !!window.__ready', { timeoutMs: 2000 })) break; } catch { /* the page is changing */ } await new Promise(r => setTimeout(r, 200)); }
  await ready(c); const url = await c.eval<string>('location.href'); const before = await c.eval(state);
  const n = await play(c, [{ n: 240, keys: ['KeyW'] }, { n: 60, keys: ['KeyQ'] }, { n: 240 }, { n: 60, keys: ['KeyF'] }, { n: 300 }, { n: 60, js: `window.__mouse(640, 360); window.__orders.open(); ` }, { n: 60, js: `window.__orders.pick(0);` }, { n: 300 },
    { n: 60, keys: ['KeyX'] }, { n: 60, keys: ['Escape'] }, { n: 60, keys: ['KeyT'] }, { n: 60, js: `window.__click(0);` }, { n: 240, keys: ['KeyW'] }, { n: 60, keys: ['KeyQ'] }, { n: Math.max(60, FRAMES - 1860) }]);
  const after = await c.eval(state); verdict(`the squad mission ${SQUAD}`, c, before, after, n); await census(c, `the squad mission ${url}`, n); });

// ---- the page as a player opens it: the title, Play, the starting page's two steps, Deploy, and the live loop for a while
if (ONLY.has('title')) await withPage(9363, async c => { const url = `${AT}/?${word('quality=verylow&mute&keeppage=0')}`; await c.open(url, 120000); await ready(c);
  const key = (code: string) => c.eval(`(() => { for (const t of ['keydown', 'keyup']) window.dispatchEvent(new KeyboardEvent(t, { code: ${JSON.stringify(code)}, key: ${JSON.stringify(code === 'Enter' ? 'Enter' : code)}, bubbles: true })); })()`);
  const menu0 = await c.eval<string>(`String(window.__menu?.mode ?? '')`);
  for (let i = 0; i < 4; i++) { await key('Enter'); await new Promise(r => setTimeout(r, 2500)); }
  const before = await c.eval(state); await new Promise(r => setTimeout(r, 20000)); const after = await c.eval(state);
  say(`      the title: the menu opened on '${menu0}', and is '${await c.eval<string>(`String(window.__menu?.mode ?? '') + (window.__menu?.isOpen ? ' (open)' : ' (closed)')`)}' after Enter four times`);
  verdict('the title, Play, Deploy, and 20 s of the live loop', c, before, after, Math.round(((after.time ?? 0) - (before.time ?? 0)) * 60)); await census(c, `the title to play ${url}`, Math.max(1, Math.round(((after.time ?? 0) - (before.time ?? 0)) * 60))); });

// ---- the other pages: each loads, and says no error in its first seconds
if (ONLY.has('pages')) for (const [i, p] of ['workbench.html', 'viewer.html', 'editor.html', 'replay.html', 'sounds.html', 'music.html'].entries()) await withPage(9364 + i, async c => { const url = `${AT}/${p}`;
  try { await c.open(url, 120000); await new Promise(r => setTimeout(r, 8000)); } catch (e) { c.errors.push(String(e)); }
  const errs = errorsOf(c); if (errs.length) bad++; say(`${errs.length ? 'FAIL' : 'PASS'}  /${p}: loaded, ${errs.length} errors in 8 s`); for (const e of errs.slice(0, 6)) say(`      ${e.slice(0, 400)}`); await census(c, `the page ${url}`, 0); });

say(bad ? `${bad} flows FAILED` : 'every flow passed (software adapter: never seen on a GPU)');
process.exit(bad ? 1 : 0);
