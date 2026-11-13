// THE GAME SERVER (bun): one process, one game, stepped with authority over WebSockets. The core is src/net/server.ts; this file is the process round it — the
// environment, the stood game (stand.ts), the sockets, the health page, the heartbeat, the exit. It knows nothing of where it runs: a control plane
// starts it in a sandbox, a dev server starts it as a child, a person starts it by hand.
//   MP_SETUP='{"scenario":"?roomclear&squad4","seed":7,"seatKeys":["k0"]}' bun run tools/mp/server.ts
// THE ENVIRONMENT (the control plane's contract):
//   MP_PORT      the port to listen on (default 5300) · MP_HOST the address (default 127.0.0.1: loopback only; a sandbox behind a tunnel says 0.0.0.0)
//   MP_GAME      the game's id (default 'local')
//   MP_SETUP     THE SETUP RECORD as JSON, the one a host's page hands the control plane and the control plane hands on:
//                  scenario  the mission's page address ("?roomclear&squad4", "?rescue&levelFile=consulate.json")       seed  the die, a whole number
//                  knobs     {kept}: what the host's machine keeps, as its one text (none: nothing)     seats one entry a seat
//                  seatKeys  one key a seat: a player proves his seat with it. As many as the stood game has seats      build, game  as MP_BUILD, MP_GAME
//                or {start, seatKeys} where whoever made the game already has one
//   MP_SEATKEYS  the seat keys, comma separated, where the record carries none
//   MP_CONTROL   a URL to POST heartbeats to (none: nobody is told) · MP_SECRET what proves a heartbeat is this server's
//   MP_IDLE_S    seconds with nobody connected before it ends itself (default 60) · MP_MAX_S its longest life (default 3600) · MP_GRACE_S how long it runs
//                on after the mission's end (default 15)
//   MP_BUILD     what its game is made of, as one word (default: the record's, else this checkout's, tools/blitz/made.ts; empty: none): a server that has
//                one lets in no client whose game is made of something else
// IT ANSWERS: a WebSocket upgrade on any path (the wire: src/net/wire.ts) · GET /health → {game, build, state, tick, seats, watchers, slips, lost, lines}.
// IT TELLS THE CONTROL PLANE: POST {game, secret, …health} every 10 s and at once on a change of state or of who is sitting; a server that is done says
// state 'ended' whatever it was doing. An answer of 410 = the control plane has no such game any more: the server stops. An answer's
// `seatKeys` are the seats' keys now (a seat that became another player's has another); a hello whose key no seat has is asked about with a heartbeat first.
// IT EXITS 0 when the core is done (idle, ended, too old, told its game is gone) after a last heartbeat; 1 when it could not stand its game; 2 on a bad environment.
import type { NetStart } from '../../src/net/start.ts';
import { NetServer, type Conn } from '../../src/net/server.ts';
import { NET } from '../../src/net/wire.ts';
import { gameMade } from '../blitz/made.ts';
import { freshStart, standStart, type Stood } from './stand.ts';

const E = process.env;
const say = (s: string) => console.log(`[mp ${new Date().toISOString().slice(11, 23)}] ${s}`);
const num = (k: string, d: number): number => { const v = E[k]; if (v === undefined || v === '') return d; const n = Number(v); if (!Number.isFinite(n) || n < 0) { console.error(`server.ts: ${k}=${v} is not a number`); process.exit(2); } return n; };
const port = num('MP_PORT', 5300), hostname = E.MP_HOST ?? '127.0.0.1';

/** the setup record (see the head) */
let asked: { scenario?: unknown; seed?: unknown; knobs?: unknown; start?: NetStart; seatKeys?: unknown; build?: unknown; game?: unknown };
try { asked = JSON.parse(E.MP_SETUP ?? ''); if (!asked || typeof asked !== 'object') throw new Error('not a record'); } catch { console.error('server.ts: MP_SETUP is not a setup record as JSON (want {"scenario":"?roomclear&squad4","seed":7,"seatKeys":["k0"]})'); process.exit(2); }
const game = E.MP_GAME ?? (typeof asked.game === 'string' ? asked.game : 'local'), build = E.MP_BUILD ?? (typeof asked.build === 'string' ? asked.build : gameMade());

let stood: Stood, start: NetStart;
try {
  const knobs: unknown = asked.knobs ?? {}; if (typeof knobs !== 'object' || knobs === null || Array.isArray(knobs)) throw new Error('the setup\'s knobs are not an object');
  const { kept, ...other } = knobs as { kept?: unknown };
  if (Object.keys(other).length) throw new Error(`a host's settings are handed over as what it keeps, not as ${Object.keys(other).join(', ').slice(0, 80)}`);
  if (kept !== undefined && typeof kept !== 'string') throw new Error('what the setup says the host keeps is not a text');
  start = asked.start ?? freshStart({ search: String(asked.scenario ?? ''), seed: Number(asked.seed), kept });
  stood = await standStart(start);
} catch (e) { console.error(`server.ts: ${(e as Error).message}`); process.exit(1); }
const seatsN = stood.seats;
const seatKeys: string[] = Array.isArray(asked.seatKeys) ? asked.seatKeys.filter((k): k is string => typeof k === 'string' && k !== '') : (E.MP_SEATKEYS ?? '').split(',').filter(Boolean);
if (seatKeys.length !== seatsN || new Set(seatKeys).size !== seatsN) { console.error(`server.ts: the stood game has ${seatsN} seat${seatsN === 1 ? '' : 's'} and wants as many different seat keys (the record's seatKeys, or MP_SEATKEYS): got ${seatKeys.length}`); process.exit(2); }

const core = new NetServer({ now: () => performance.now(), step: cmds => stood.step(cmds), word: () => stood.word(), ended: () => stood.ended(), ...(E.MP_CONTROL ? { ask: () => beat() } : {}) },
  { game, build, start, seatKeys, idleS: num('MP_IDLE_S', 60), maxS: num('MP_MAX_S', 3600), graceS: num('MP_GRACE_S', 15) });

// ---- the heartbeat
const control = E.MP_CONTROL ?? '', secret = E.MP_SECRET ?? '';
let beating: Promise<void> = Promise.resolve(), next: Promise<void> | null = null;
/** one waits at a time: it says what is so when it goes. Settled once it is answered, or is not */
function beat(): Promise<void> {
  if (!control) return Promise.resolve();
  return next ??= beating = beating.then(() => { next = null; return sendBeat(); });
}
async function sendBeat(): Promise<void> {
  // (to a control plane 'ended' means THIS SERVER IS DONE — it sweeps the game and its runner on the word. A mission that is over while the grace still runs is
  //  still up for its players' end screen: it reads 'playing', its outcome beside it, until the core is done. /health keeps the core's own word)
  const h = core.health(), body = JSON.stringify({ secret, ...h, state: h.done ? 'ended' : h.state === 'ended' ? 'playing' : h.state });
  try {
    const r = await fetch(control, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body, signal: AbortSignal.timeout(5000) });
    if (r.status === 410) return core.stop('the control plane has no such game');
    const seatKeys = r.ok ? (await r.json() as { seatKeys?: unknown }).seatKeys : null;
    if (Array.isArray(seatKeys) && seatKeys.every(k => typeof k === 'string' && k)) core.rekey(seatKeys);
  } catch { /* the control plane is away */ }
}
let saidState = '';
core.onChange = () => { const h = core.health(), k = `${h.state} ${h.seats.join()} ${h.lost} ${h.done ?? ''}`; if (k !== saidState) { saidState = k; say(`${h.state} · tick ${h.tick} · seats ${h.seats.map(s => s ? 'in' : 'empty').join(', ')}${h.outcome ? ` · ${h.outcome.end} (${h.outcome.why})` : ''}${h.lost ? ` · ${h.lost} client${h.lost === 1 ? '' : 's'} parted from this game` : ''}${h.done ? ` · done: ${h.done}` : ''}`); void beat(); } };

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
say(`game ${game} · build ${build || '(none)'} · ${start.file} ${start.words} · seed ${start.seed} · ${seatsN} seat${seatsN === 1 ? '' : 's'} · ws://${hostname}:${server.port} · ${NET.hz} Hz`);
void beat();

const pulse = setInterval(() => void beat(), 10_000);
const clock = setInterval(async () => {
  try { core.advance(); } catch (e) { say(`the game threw at tick ${core.tick}: ${(e as Error).stack ?? e}`); clearInterval(clock); clearInterval(pulse); core.stop('the game threw'); await last(); process.exit(1); }   // (its clients are told, and the control plane hears 'ended' now, not by a silence)
  if (!core.done) return;
  clearInterval(clock); clearInterval(pulse);
  await beating; server.stop(true);
  process.exit(0);
}, 2);
/** the last heartbeat, waited for at most three seconds */
const last = (): Promise<unknown> => Promise.race([beating, new Promise(r => setTimeout(r, 3000))]);
for (const sig of ['SIGINT', 'SIGTERM'] as const) process.on(sig, () => { say(`${sig}: stopping`); clearInterval(clock); clearInterval(pulse); core.stop(`the server was stopped (${sig})`); void last().then(() => { server.stop(true); process.exit(0); }); });   // (a bye to its clients and a last 'ended' to the control plane, then out)
