// THE PAGE SCRIPTS: the real page in headless Chromium on the software WebGPU adapter, one script at a time (a page takes the whole box), against one dev server started here.
//   bun run tools/qa/pages.ts [group | name …] [--list]     groups: boot · play · menus · coop · saves · cloud · replay · all (every group but tools) · tools (named one by one: never run by a group)
//   env: QA_PAGE_CAP (minutes a script may take, default 10; its whole process group is stopped there) · the script's own words pass through (SHOTS=0, JOBS=…)
// A .ts row is run by bun with a fresh file and its own --port. A script reads PORT, PLAYWRIGHT_MODULE, CHROMIUM and CHROME from here. Exit 0 every script green · 1 one red (a FAIL line, a non-zero exit or the cap) · 2 an unknown name.
import { existsSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve } from 'node:path';

export type PageGroup = 'boot' | 'play' | 'menus' | 'coop' | 'saves' | 'cloud' | 'replay' | 'tools';
/** script, group, what it guards in one line, minutes when read (four cores, no GPU; 0 = not read: a tool, or the stand-in backend's packages were not to be had) */
export type PageRow = readonly [script: string, group: PageGroup, guards: string, minutes: number];
export const PAGES: readonly PageRow[] = [
  ['nogpu-page.cjs', 'boot', "a start with no GPU: the red card, its lines and its Copy button", 2.1],
  ['firstframe-page.cjs', 'boot', "the first played frame is an ordinary one: the game is warmed before the loop", 3.0],
  ['keeppage-page.cjs', 'boot', "the kept page: the game in a frame on a lease, a second mission compiles nothing again", 3.1],
  ['loadbrief-page.cjs', 'boot', "a mission's own address as a visitor opens it: the brief, then the block goes live", 5.6],
  ['wgsl-compile.cjs', 'boot', "every shader text compiles", 0.1],
  ['tut-road-page.cjs', 'play', "the first tutorial mission with real keys, first line to debrief", 2.4],
  ['tut-team-page.cjs', 'play', "the squad tutorial mission with real keys", 2.4],
  ['tut-pair-page.cjs', 'play', "the pair tutorial mission with real keys", 7.2],
  ['tut-kit-page.cjs', 'play', "the kit tutorial mission with real keys", 3.6],
  ['tut-queue-page.cjs', 'play', "the fourth tutorial mission with real keys", 1.9],
  ['tut-quiet-page.cjs', 'play', "the stealth tutorial's stretch with the launcher, real keys", 1.3],
  ['samefight-page.cjs', 'play', "same seed, same inputs: the page against the harness to the last bit", 2.7],
  ['cleanrestart-page.cjs', 'play', "a restart on the page: a lamp shot out comes back, the frames after it as the first", 4.1],
  ['deathrecap-page.cjs', 'play', "every death explains itself: the down card's plan, who fired, it plays", 3.3],
  ['runbooks-page.cjs', 'play', "a run's page is nobody's try: the end card and the books", 5.1],
  ['panel-keys.cjs', 'play', "the debugger's key and Tab, pressed for real", 1.5],
  ['startpage-look.cjs', 'menus', "every mission on the start page goes through its loadout to Deploy", 0.9],
  ['cards-page.ts', 'menus', "the start page walked: every mission, row and card written down as the page shows them", 0.7],
  ['kitdense-page.cjs', 'menus', "the dense kit card: tiles, pop-ups, fits at four window sizes", 0.5],
  ['kitguide-page.cjs', 'menus', "the kit step's brief and the tutorial's walkthrough", 0.8],
  ['kitpaste-page.cjs', 'menus', "copy and paste for loadouts", 0.2],
  ['keybinds-page.cjs', 'menus', "the player's own keys on Options > Controls", 0.2],
  ['textsize-page.cjs', 'menus', "the game's own text size, the screens at each size", 1.5],
  ['mpclient-page.cjs', 'coop', "the page as a client of a real game server", 4.5],
  ['lobby-page.cjs', 'coop', "host or join a co-op game, two windows", 0],
  ['savetimeline-page.cjs', 'saves', "the save timeline: F5, the strip, a click goes back", 5.8],
  ['savedoor-page.cjs', 'saves', "a saved file goes to the backend, no download", 0],
  ['cloud-page.cjs', 'cloud', "the hosted page's backend: tape, visit, settings, the link", 0],
  ['cloudfatal-page.cjs', 'cloud', "a red card tells the backend", 0],
  ['cloudnote-page.cjs', 'cloud', "the notice line on a card that is not the title", 0.1],
  ['engine-replay-page.cjs', 'replay', "a played run is on the shelf with its run tape, and plays to its end as recorded", 7.9],
  ['replaykeep-page.cjs', 'replay', "every run kept, one handed over", 6.1],
  ['replaysend-page.cjs', 'replay', "the map viewer's Send", 0.4],
  ['bench-page.cjs', 'tools', "the benchmark run end to end", 0],
  ['perf-sweep-page.cjs', 'tools', "the F8 sweep run and its dump", 0],
  ['framecap-page.cjs', 'tools', "an F6 frame capture taken and read back", 0],
  ['pricerun-page.cjs', 'tools', "the pricing run", 0],
  ['cpuprofile-page.cjs', 'tools', "where a frame's CPU goes", 0],
  ['draw-equal.cjs', 'tools', "the picture-equality rig", 0],
  ['kernel-stack.cjs', 'tools', "how near a kernel is to the software adapter's limit", 0],
  ['level-cost-page.cjs', 'tools', "what the look costs on one level", 0],
  ['look-sheet.cjs', 'tools', "the look page's picture at every hour, a contact sheet", 0],
  ['page-look.cjs', 'tools', "a scenario played a little: stills, errors", 0],
  ['shader-count-page.cjs', 'tools', "how much a page compiles", 0],
  ['shader-serial-page.cjs', 'tools', "what each program costs to compile alone", 0],
  ['warmboot-page.cjs', 'tools', "a second visit's boot, on a GPU", 0],
  ['weather-look.cjs', 'tools', "a still per level, weather and view", 0],
];
/** these start their own dev server (it wants a backend's words) on the PORT they are handed */
const OWN_SERVER = new Set(['lobby-page.cjs']);

const ROOT = resolve(import.meta.dir, '../..'), GROUPS = new Set<string>([...PAGES.map(r => r[1])]);
const freePort = (): number => { const s = (Bun as any).serve({ hostname: '127.0.0.1', port: 0, fetch: () => new Response() }), p: number = s.port; s.stop(true); return p; };
const short = (script: string): string => script.replace(/\.(cjs|ts)$/, '');
const mins = (ms: number): string => (ms / 60000).toFixed(1);

/** every process under `pid` by parent (Playwright starts its browser in a process group of its own) */
function under(pid: number): number[] {
  const rows = new TextDecoder().decode(Bun.spawnSync(['ps', '-A', '-o', 'pid=,ppid=']).stdout).trim().split('\n').map(l => l.trim().split(/\s+/).map(Number));
  const found = [pid]; for (const p of found) for (const [c, q] of rows) if (q === p) found.push(c);
  return found;
}
const stop = (pid: number, sig: 'SIGTERM' | 'SIGKILL'): void => { try { process.kill(pid, sig); } catch { /* gone already */ } };
/** asked to end first, so that what it says as it ends is said and Playwright shuts its browser; three seconds later all of it is killed */
function stopAll(pid: number): void { const all = under(pid); stop(-pid, 'SIGTERM'); setTimeout(() => { stop(-pid, 'SIGKILL'); for (const p of all) stop(p, 'SIGKILL'); }, 3000); }

async function echo(from: ReadableStream<Uint8Array>, to: { write(s: string): unknown }, fails: string[]): Promise<void> {
  const dec = new TextDecoder(); let tail = '';
  for await (const chunk of from) { const s = dec.decode(chunk, { stream: true }); to.write(s); tail += s; const lines = tail.split('\n'); tail = lines.pop() ?? ''; for (const l of lines) if (/^\s*(- )?FAIL\b/.test(l)) fails.push(l.trim()); }
}

interface Read { script: string; ms: number; ok: boolean; fails: string[]; code: number; capped: boolean }
async function runOne(script: string, port: number, capMs: number): Promise<Read> {
  const env: Record<string, string | undefined> = { ...process.env, PORT: String(OWN_SERVER.has(script) ? freePort() : port) };
  const pw = '/opt/node-tools/node_modules'; if (existsSync(pw)) { env.NODE_PATH = pw; env.PLAYWRIGHT_MODULE ??= `${pw}/playwright`; }
  if (script.startsWith('tut-')) env.SHOTS ??= '0';   // (their pictures: a minute each here, for a person who asks)
  if (existsSync('/usr/bin/chromium')) { env.CHROMIUM ??= '/usr/bin/chromium'; env.CHROME ??= '/usr/bin/chromium'; }
  const t0 = performance.now(), c = Bun.spawn(script.endsWith('.ts') ? ['bun', 'run', resolve(import.meta.dir, script), resolve(mkdtempSync(resolve(tmpdir(), 'page-')), 'out.txt'), '--port', String(freePort())] : ['node', resolve(import.meta.dir, script)], { cwd: ROOT, detached: true, stdout: 'pipe', stderr: 'pipe', env }), fails: string[] = [];
  let capped = false; const timer = setTimeout(() => { capped = true; stopAll(c.pid); }, capMs);
  LIVE.add(c.pid);
  const gone = c.exited.then(code => { stop(-c.pid, 'SIGKILL'); return code; });   // (nothing it started outlives it, so the pipes close)
  const [code] = await Promise.all([gone, echo(c.stdout, process.stdout, fails), echo(c.stderr, process.stderr, fails)]);
  clearTimeout(timer); LIVE.delete(c.pid);
  return { script, ms: performance.now() - t0, ok: code === 0 && !capped && !fails.length, fails, code, capped };
}
const LIVE = new Set<number>();

if (import.meta.main) {
  const args = process.argv.slice(2), words = args.filter(a => !a.startsWith('--'));
  if (args.includes('--list') || !words.length) {
    for (const g of [...GROUPS]) { const rows = PAGES.filter(r => r[1] === g); console.log(`${g}  (${rows.reduce((n, r) => n + r[3], 0).toFixed(0)} min)`); for (const [s, , what, m] of rows) console.log(`  ${short(s).padEnd(24)} ${String(m).padStart(4)}  ${what}`); }
    process.exit(0);
  }
  const unknown = words.filter(w => w !== 'all' && !GROUPS.has(w) && !PAGES.some(r => r[0] === w || short(r[0]) === w));
  if (unknown.length) { console.error(`pages: no such group or script: ${unknown.join(', ')}`); process.exit(2); }
  const chosen = PAGES.filter(([s, g]) => words.includes(s) || words.includes(short(s)) || (g !== 'tools' && (words.includes(g) || words.includes('all'))));
  const capMs = Number(process.env.QA_PAGE_CAP || 10) * 60000, port = freePort();
  const server = Bun.spawn(['bun', 'run', 'tools/serve.ts'], { cwd: ROOT, stdout: 'ignore', stderr: 'ignore', env: { ...process.env, PORT: String(port) } });
  for (const sig of ['SIGINT', 'SIGTERM'] as const) process.on(sig, () => { for (const p of LIVE) for (const q of under(p)) stop(q, 'SIGKILL'); server.kill(); process.exit(1); });
  for (let i = 0; ; i++) { try { if ((await fetch(`http://127.0.0.1:${port}/qa/commit`)).ok) break; } catch { /* not up yet */ } if (i > 100) { console.error('pages: the dev server did not come up'); server.kill(); process.exit(1); } await Bun.sleep(100); }
  const reads: Read[] = [];
  for (const [script] of chosen) {
    console.log(`\n# ${script}`);
    const r = await runOne(script, port, capMs); reads.push(r);
    console.log(`# ${script}: ${r.ok ? 'green' : 'RED'} · ${mins(r.ms)} min${r.capped ? ` · stopped at the cap of ${mins(capMs)} min` : r.ok ? '' : ` · exit ${r.code}, ${r.fails.length} FAIL lines`}`);
    if (r.capped) await Bun.sleep(3500);   // (the cap's kill, before the next script starts)
  }
  server.kill();
  const red = reads.filter(r => !r.ok), total = reads.reduce((n, r) => n + r.ms, 0);
  console.log('');
  for (const r of reads) console.log(`${r.ok ? 'green' : 'RED  '} ${mins(r.ms).padStart(5)}  ${r.script}`);
  for (const r of red) for (const f of r.fails.slice(0, 8)) console.log(`  ${r.script}: ${f.slice(0, 200)}`);
  console.log(`=== pages: ${reads.length - red.length} of ${reads.length} green, ${mins(total)} min${red.length ? `; RED: ${red.map(r => r.script).join(', ')}` : ''} ===`);
  process.exit(red.length ? 1 : 0);
}
