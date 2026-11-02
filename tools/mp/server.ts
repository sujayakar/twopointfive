// THE GAME SERVER (bun): one process, one game, stepped with authority over WebSockets. The core is src/net/server.ts; this file is the process round it — the
// environment, the stood game (tools/mp/stand.ts), the sockets, the health page, the heartbeat, the exit. It knows nothing of where it runs: a control plane
// starts it in a sandbox, a dev server starts it as a child, a person starts it by hand.
//   MP_SETUP='{"search":"?roomclear&squad4","seed":7}' MP_SEATKEYS=k0 bun run tools/mp/server.ts
// THE ENVIRONMENT (the control plane's contract):
//   MP_PORT      the port to listen on (default 5300) · MP_HOST the address (default 127.0.0.1: loopback only; a sandbox behind a tunnel says 0.0.0.0)
//   MP_GAME      the game's id (default 'local')
//   MP_SETUP     the setup as JSON ({search, seed, tables?}: stand.ts Setup), or {recipe} where whoever made the game already has one
//   MP_SEATKEYS  one key a seat, comma separated: a player proves his seat with it. As many as the stood game has seats
//   MP_CONTROL   a URL to POST heartbeats to (none: nobody is told) · MP_SECRET what proves a heartbeat is this server's
//   MP_IDLE_S    seconds with nobody connected before it ends itself (default 60) · MP_MAX_S its longest life (default 3600) · MP_GRACE_S how long it runs
//                on after the mission's end (default 15)
//   MP_BUILD     the build's name (default: the checkout's commit, or none with uncommitted changes): a server that has one lets in no client of another
// IT ANSWERS: a WebSocket upgrade on any path (the wire: src/net/wire.ts) · GET /health → {game, build, state, tick, seats, watchers, slips, lost, lines}.
// IT TELLS THE CONTROL PLANE: POST {game, secret, …health} every 10 s and at once on a change of state or of who is sitting.
// IT EXITS 0 when the core is done (idle, ended, too old) after a last heartbeat; 1 when it could not stand its game; 2 on a bad environment.
import { resolve } from 'node:path';
import type { Recipe } from '../../src/net/wire.ts';
import type { Conn } from '../../src/net/server.ts';
import type { PlayerCmd } from '../../src/game/cmd.ts';
import { standRecipe, buildName, type Setup } from './stand.ts';

const ROOT = resolve(import.meta.dir, '../..'), E = process.env;
const say = (s: string) => console.log(`[mp ${new Date().toISOString().slice(11, 23)}] ${s}`);
const num = (k: string, d: number): number => { const v = E[k]; if (v === undefined || v === '') return d; const n = Number(v); if (!Number.isFinite(n) || n < 0) { console.error(`server.ts: ${k}=${v} is not a number`); process.exit(2); } return n; };
const port = num('MP_PORT', 5300), hostname = E.MP_HOST ?? '127.0.0.1', game = E.MP_GAME ?? 'local';
const build = E.MP_BUILD ?? buildName();

let asked: { recipe?: Recipe } & Partial<Setup>;
try { asked = JSON.parse(E.MP_SETUP ?? ''); } catch { console.error('server.ts: MP_SETUP is not JSON (want {"search":"?roomclear&squad4","seed":7} or {"recipe":{…}})'); process.exit(2); }

/** the setup made into a recipe in a process of its own (tools/mp/setup.ts: the game it stands for it is thrown away) */
async function recipeOfSetup(setup: Setup): Promise<Recipe> {
  const p = Bun.spawn([process.execPath, 'run', resolve(import.meta.dir, 'setup.ts'), JSON.stringify(setup)], { stdout: 'pipe', stderr: 'pipe', cwd: ROOT, env: { ...E } });
  const [out, err, code] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text(), p.exited]);
  const line = out.split('\n').find(l => l.startsWith('MPSETUP '));
  if (code !== 0 || !line) throw new Error(`the setup could not be made into a recipe: ${err.trim().split('\n').pop() || `exit ${code}`}`);
  return JSON.parse(line.slice(8)) as Recipe;
}

let stood, recipe: Recipe;
try {
  recipe = asked.recipe ?? await recipeOfSetup({ search: String(asked.search ?? ''), seed: Number(asked.seed), ...(asked.tables ? { tables: asked.tables } : {}) });
  stood = await standRecipe(recipe);
} catch (e) { console.error(`server.ts: ${(e as Error).message}`); process.exit(1); }
for (const w of stood.warnings) say(`note: ${w}`);
const seatsN = stood.W.game.seats.length as number, seatKeys = (E.MP_SEATKEYS ?? '').split(',').filter(Boolean);
if (seatKeys.length !== seatsN || new Set(seatKeys).size !== seatsN) { console.error(`server.ts: MP_SEATKEYS wants ${seatsN} different key${seatsN === 1 ? '' : 's'}, one a seat (got ${seatKeys.length})`); process.exit(2); }

const { NetServer } = await import(`${ROOT}/src/net/server.ts`), { NET } = await import(`${ROOT}/src/net/wire.ts`);
const core = new NetServer({ now: () => performance.now(), step: (cmds: PlayerCmd[]) => stood.step(cmds), word: () => stood.word(), ended: () => stood.ended() },
  { game, build, recipe, seatKeys, idleS: num('MP_IDLE_S', 60), maxS: num('MP_MAX_S', 3600), graceS: num('MP_GRACE_S', 15) });

// ---- the heartbeat
const control = E.MP_CONTROL ?? '', secret = E.MP_SECRET ?? '';
let beating: Promise<void> = Promise.resolve();
function beat(): void {
  if (!control) return;
  const body = JSON.stringify({ secret, ...core.health() });
  beating = beating.then(() => fetch(control, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body, signal: AbortSignal.timeout(5000) }).then(() => undefined, () => undefined));
}
let saidState = '';
core.onChange = () => { const h = core.health(), k = `${h.state} ${h.seats.join()} ${h.lost} ${h.done ?? ''}`; if (k !== saidState) { saidState = k; say(`${h.state} · tick ${h.tick} · seats ${h.seats.map((s: boolean) => s ? 'in' : 'empty').join(', ')}${h.outcome ? ` · ${h.outcome.end} (${h.outcome.why})` : ''}${h.lost ? ` · ${h.lost} client${h.lost === 1 ? '' : 's'} parted from this game` : ''}${h.done ? ` · done: ${h.done}` : ''}`); beat(); } };

// ---- the sockets
type WS = { send(text: string): number; close(code?: number, reason?: string): unknown; data: { conn: Conn } };
const server = Bun.serve({
  port, hostname,
  fetch(req: Request, srv: { upgrade(req: Request, o: { data: unknown }): boolean }) {
    const url = new URL(req.url);
    if (url.pathname === '/health') return new Response(JSON.stringify({ ...core.health(), lines: core.lines() }), { headers: { 'Content-Type': 'application/json', 'Cache-Control': 'no-store', 'Access-Control-Allow-Origin': '*' } });
    if (srv.upgrade(req, { data: {} })) return undefined as unknown as Response;
    return new Response('a game server: connect a WebSocket, or GET /health', { status: 426 });
  },
  websocket: {
    maxPayloadLength: 64 * 1024,   // (a client's record is a few hundred bytes: nothing longer is read)
    open(ws: WS) { ws.data.conn = { send: text => ws.send(text), close: why => { ws.close(1000, why.slice(0, 100)); } }; },
    drain(ws: WS) { core.drained(ws.data.conn); },
    message(ws: WS, msg: string | Uint8Array) { core.message(ws.data.conn, typeof msg === 'string' ? msg : new TextDecoder().decode(msg)); },
    close(ws: WS) { core.closed(ws.data.conn); },
  },
});
say(`game ${game} · build ${build || '(none)'} · ${recipe.scenario} ${recipe.search} · seed ${recipe.seed} · ${seatsN} seat${seatsN === 1 ? '' : 's'} · ws://${hostname}:${server.port} · ${NET.hz} Hz`);
beat();

const pulse = setInterval(beat, 10_000);
const clock = setInterval(async () => {
  try { core.advance(); } catch (e) { say(`the game threw at tick ${core.tick}: ${(e as Error).stack ?? e}`); clearInterval(clock); clearInterval(pulse); await beating; process.exit(1); }
  if (!core.done) return;
  clearInterval(clock); clearInterval(pulse);
  await beating; server.stop(true);
  process.exit(0);
}, 2);
for (const sig of ['SIGINT', 'SIGTERM'] as const) process.on(sig, () => { say(`${sig}: stopping`); server.stop(true); process.exit(0); });
