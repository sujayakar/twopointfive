// A STAND-IN GAME SERVER: what the control plane starts where the real one (tools/mp/server.ts) is not wanted — its own check (tools/qa/mp-control-check.ts)
// and a dev server started with MP_SERVER=tools/mp/runner-standin.ts. It keeps the server's side of the contract with the control plane and plays nothing:
//   started with MP_PORT · MP_GAME · MP_SETUP (the setup record as JSON, `seatKeys` in it) · MP_CONTROL · MP_SECRET · MP_IDLE_S · MP_MAX_S
//   GET /health answers `{ game, tick, seats: [connected…], state }`; a WebSocket's first message `{ seat, key }` takes that seat if the key is the seat's
//   a heartbeat `{ game, secret, state, tick, seats, outcome? }` to MP_CONTROL every MP_BEAT_S (10) and at once when its state changes
//   it ends itself (a last heartbeat 'ended', exit 0): nobody connected for MP_IDLE_S, MP_MAX_S since its start, or its heartbeat answered 410
// The check's own words, for a server that does NOT keep the contract: STANDIN_STUBBORN=1 never ends itself · STANDIN_MUTE_S=<n> sends no heartbeat once it is n seconds old (0 = never sends one).
const E = process.env, num = (k: string, d: number): number => { const n = Number(E[k]); return Number.isFinite(n) && n > 0 ? n : d; };
const game = E.MP_GAME ?? '', control = E.MP_CONTROL ?? '', secret = E.MP_SECRET ?? '';
const setup = JSON.parse(E.MP_SETUP || '{}') as { seatKeys?: string[] };
const keys = setup.seatKeys ?? [], idleS = num('MP_IDLE_S', 60), maxS = num('MP_MAX_S', 3600), beatS = num('MP_BEAT_S', 10);
const stubborn = E.STANDIN_STUBBORN === '1', muteS = E.STANDIN_MUTE_S === undefined ? Infinity : Number(E.STANDIN_MUTE_S);
const seats = keys.map(() => false), born = Date.now();
let state: 'waiting' | 'playing' | 'ended' = 'waiting', emptySince = born, over = false;
const tick = (): number => (state === 'waiting' ? 0 : Math.floor((Date.now() - born) * 0.06));

async function beat(): Promise<void> {
  if (!control || Date.now() - born >= muteS * 1000) return;
  try {
    const r = await fetch(control, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ game, secret, state, tick: tick(), seats, ...(state === 'ended' ? { outcome: { by: 'stand-in' } } : {}) }), signal: AbortSignal.timeout(5000) });
    if (r.status === 410 && !stubborn) void end();
  } catch { /* the control plane is away: it will stop this server, or the clocks below will */ }
}
async function end(): Promise<void> {
  if (over) return; over = true; state = 'ended';
  await beat();
  server.stop(true); process.exit(0);
}
function settle(): void {
  const next = seats.length > 0 && seats.every(Boolean) ? 'playing' : 'waiting';
  if (!seats.some(Boolean)) emptySince = Math.min(emptySince, Date.now()); else emptySince = Infinity;
  if (next !== state && state !== 'ended') { state = next; void beat(); }
}

const server = Bun.serve<{ seat: number }, never>({
  port: Number(E.MP_PORT ?? 0), hostname: '127.0.0.1',
  fetch(req, srv) {
    const path = new URL(req.url).pathname;
    if (path === '/health') return Response.json({ game, tick: tick(), seats, state });
    return srv.upgrade(req, { data: { seat: -1 } }) ? undefined : new Response('a game server: WebSocket only', { status: 426 });
  },
  websocket: {
    message(ws, raw) {
      if (ws.data.seat >= 0) return;
      let m: { seat?: number; key?: string }; try { m = JSON.parse(String(raw)); } catch { return ws.close(1008, 'not JSON'); }
      const s = m.seat ?? -1;
      if (!Number.isInteger(s) || s < 0 || s >= keys.length || m.key !== keys[s] || seats[s]) return ws.close(1008, 'not a free seat\'s key');
      ws.data.seat = s; seats[s] = true; ws.send(JSON.stringify({ seat: s })); settle();
    },
    close(ws) { const s = ws.data.seat; if (s >= 0) { seats[s] = false; if (!seats.some(Boolean)) emptySince = Date.now(); settle(); } },
  },
});
void beat();
setInterval(() => void beat(), beatS * 1000);
setInterval(() => { if (!stubborn && (Date.now() - emptySince > idleS * 1000 || Date.now() - born > maxS * 1000)) void end(); }, 100);
for (const sig of ['SIGINT', 'SIGTERM'] as const) process.on(sig, () => process.exit(0));
