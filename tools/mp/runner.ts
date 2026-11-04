// THE LOCAL RUNNER: game servers as child processes of the dev server, behind the same three calls every runner answers (cloud/convex/runners.ts says
// them: POST start · stop · list · ready, `Authorization: Bearer <key>`). `bun run dev` then plays the whole of co-op's control plane on one machine:
// the page asks the local backend for a game, the backend asks THIS for a server, this starts `tools/mp/server.ts`, the server sends its heartbeats to
// the backend, and the backend's reaper stops it through this again. Nothing hosted is touched.
//   localRunner(…)   the runner itself: start / stop / list, and `handle(req)` for a server's fetch (null = not its path)
//   devRunner(…)     the dev server's side (tools/serve.ts): the runner, and its registration with the backend on this machine once that answers
// Words of the environment (all optional):
//   MP_SERVER=tools/mp/runner-standin.ts   what a server is (default tools/mp/server.ts). The stand-in answers /health and sends heartbeats and plays nothing
//   MP_CAP=2                               how many servers this machine may carry at once
//   MP_CLOUD_KEY=…                         the admin key of a backend that is already up (TPS_DEV_CLOUD) and is not the one `convex dev` started here
// A server's build is the tree it is started from, which is the tree the page was served from: the same build with nothing to check.
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { LOOPBACK } from '../../src/cloud/addr.ts';
import { wire } from '../../src/cloud/wire.ts';
import { localKey } from '../cloud/local.ts';

const ROOT = resolve(import.meta.dir, '../..');

/** what a runner is asked to start (cloud/convex/runners.ts) */
export interface RunnerStart { game: string; build: string; setup: string; control: string; owner: string; secret: string; idleS: number; maxS: number }
export interface RunnerServer { id: string; game: string; owner: string; address: string }
interface Child { exited: Promise<number>; kill(signal?: 'SIGKILL'): void }
export interface LocalRunnerOpts {
  /** what a call must show */
  key: string;
  /** the build this tree is: a start for another is refused */
  build: string;
  /** the server's file, from the repo's root */
  entry?: string;
  /** where a child's output goes ('inherit' on the dev server: its lines are the server's log) */
  output?: 'inherit' | 'ignore';
  /** how long a start waits for the server to answer /health, ms */
  healthMs?: number;
  say?: (line: string) => void;
}

/** a port nothing listens on now (the system's choice, taken and let go: the server that is handed it binds it a moment later) */
const freePort = (): number => { const s = Bun.serve({ hostname: '127.0.0.1', port: 0, fetch: () => new Response('') }); const p = Number(s.port); s.stop(true); return p; };
const json = (status: number, body: unknown): Response => new Response(JSON.stringify(body), { status, headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store' } });
const isStart = (b: any): b is RunnerStart => !!b && ['game', 'build', 'setup', 'control', 'owner', 'secret'].every(k => typeof b[k] === 'string') && Number.isFinite(b.idleS) && Number.isFinite(b.maxS);

export function localRunner(o: LocalRunnerOpts) {
  const say = o.say ?? (() => { /* */ }), healthMs = o.healthMs ?? 15000;
  const servers = new Map<string, RunnerServer & { child: Child }>();
  let n = 0;

  async function start(b: RunnerStart): Promise<RunnerServer> {
    const entry = o.entry ?? 'tools/mp/server.ts';
    if (!existsSync(resolve(ROOT, entry))) throw new Error(`no server to start: ${entry} is not in this tree (MP_SERVER names another)`);
    const port = freePort(), id = `local-${process.pid}-${++n}`;
    const child = Bun.spawn([process.execPath, 'run', entry], {
      cwd: ROOT, stdin: 'ignore', stdout: o.output ?? 'ignore', stderr: o.output ?? 'ignore',
      env: { ...process.env, MP_PORT: String(port), MP_GAME: b.game, MP_SETUP: b.setup, MP_CONTROL: b.control, MP_SECRET: b.secret, MP_IDLE_S: String(b.idleS), MP_MAX_S: String(b.maxS) },
    });
    const info = { id, game: b.game, owner: b.owner, address: `ws://localhost:${port}`, child };
    servers.set(id, info);
    let gone = false;
    void child.exited.then(code => { gone = true; servers.delete(id); say(`server ${id} of game ${b.game} stopped (exit ${code})`); });
    for (const until = Date.now() + healthMs; Date.now() < until && !gone;) {
      try { if ((await fetch(`http://127.0.0.1:${port}/health`, { signal: AbortSignal.timeout(500) })).ok) { say(`server ${id} of game ${b.game} answers at ${info.address}`); return pub(info); } } catch { /* not up yet */ }
      await Bun.sleep(50);
    }
    stop(id);
    throw new Error(gone ? 'the server stopped before it answered' : `the server did not answer /health in ${healthMs} ms`);
  }
  const pub = ({ id, game, owner, address }: RunnerServer): RunnerServer => ({ id, game, owner, address });
  /** stopping one that is gone is not an error. A server that does not go when asked is killed */
  function stop(id: string): void {
    const s = servers.get(id); if (!s) return;
    try { s.child.kill(); } catch { /* gone */ }
    const hard = setTimeout(() => { try { s.child.kill('SIGKILL'); } catch { /* gone */ } }, 3000);
    void s.child.exited.then(() => clearTimeout(hard));
  }
  const list = (): RunnerServer[] => [...servers.values()].map(pub);
  const stopAll = (): void => { for (const id of [...servers.keys()]) stop(id); };

  /** `base` = the path the runner answers under */
  async function handle(req: Request, base = '/mp/runner'): Promise<Response | null> {
    const path = new URL(req.url).pathname;
    if (path !== base && !path.startsWith(base + '/')) return null;
    // (a page of any site can send this machine a request: the key is what makes a call the backend's)
    if (req.headers.get('authorization') !== `Bearer ${o.key}`) return json(401, { why: 'not the runner\'s key' });
    const what = path.slice(base.length + 1);
    if (req.method !== 'POST') return json(404, { why: 'not a call a runner answers' });
    let b: any; try { b = await req.json(); } catch { return json(400, { why: 'not JSON' }); }
    if (what === 'list') return json(200, { servers: list().filter(s => typeof b?.owner !== 'string' || s.owner === b.owner) });
    if (what === 'ready') return json(200, { ok: true });   // (this tree is the build: nothing to fetch)
    if (what === 'stop') { if (typeof b?.id !== 'string') return json(400, { why: 'no id' }); stop(b.id); return json(200, { ok: true }); }
    if (what !== 'start') return json(404, { why: 'not a call a runner answers' });
    if (!isStart(b)) return json(400, { why: 'not a start' });
    if (b.build !== o.build) return json(409, { why: `this runner's servers are build '${o.build}'` });
    try { const s = await start(b); return json(200, { id: s.id, address: s.address }); } catch (e) { return json(500, { why: String((e as Error)?.message ?? e) }); }
  }
  return { start, stop, list, stopAll, handle };
}
export type LocalRunner = ReturnType<typeof localRunner>;

export interface DevRunner { handle(req: Request): Promise<Response | null>; stop(): void }

/** THE DEV SERVER'S RUNNER. `cloud()` = the backend the dev server hands its pages ('' = none yet); `origin` = where the dev server answers; `build` =
 *  the commit it serves. Registers with a backend ON THIS MACHINE only, and again if that backend comes back at another address: a hosted backend has
 *  its own runner and cannot reach this one. `key` and `register` are the check's */
export function devRunner(d: { cloud: () => string; origin: string; build: string; env?: Record<string, string | undefined>; adminKey?: (url: string) => string | undefined; pollMs?: number }): DevRunner {
  const env = d.env ?? process.env, say = (s: string): void => console.log(`game servers: ${s}`);
  const key = Array.from(crypto.getRandomValues(new Uint8Array(24)), b => b.toString(16).padStart(2, '0')).join('');
  const runner = localRunner({ key, build: d.build, entry: env.MP_SERVER, output: 'inherit', say });
  const adminKey = d.adminKey ?? ((url: string) => { if (env.MP_CLOUD_KEY) return env.MP_CLOUD_KEY; const k = localKey(); return k && k.url === url ? k.key : undefined; });
  const cap = Number(env.MP_CAP ?? 2);
  // (the backend's functions load one package since co-op: a checkout set up before that pushes nothing new until its packages are fetched again)
  if (!existsSync(resolve(ROOT, 'cloud/node_modules/modal'))) say('the backend\'s packages have changed: run `cd cloud && npm ci` and start the dev server again, or the local backend keeps its old functions and no game can be started.');
  let at = '', told = '';
  const timer = setInterval(() => void (async () => {
    const url = d.cloud();
    if (!url || url === at || !LOOPBACK.test(url)) return;
    const admin = adminKey(url);
    if (!admin) { if (told !== url) say(`the backend at ${url} is not the one started here and MP_CLOUD_KEY is not set: no game can be started on this machine.`); told = url; return; }
    try {
      await wire(url, { key: admin }).mutation('runners:register', { name: 'local', url: `${d.origin}/mp/runner`, key, build: d.build, cap });
      at = url; say(`this machine runs the game servers of ${url} (at most ${cap} at once; a server is ${env.MP_SERVER ?? 'tools/mp/server.ts'}).`);
    } catch (e) { if (told !== url) say(`could not register with ${url}: ${String((e as Error)?.message ?? e)}`); told = url; }
  })(), d.pollMs ?? 1000);
  return { handle: req => runner.handle(req), stop: () => { clearInterval(timer); runner.stopAll(); } };
}
