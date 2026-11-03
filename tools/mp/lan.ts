// A CO-OP GAME ON YOUR OWN NETWORK, by hand, until the page has its host and join screen. It starts ONE game server (tools/mp/server.ts) on this machine,
// listening on every interface, for a squad mission of two players, and prints the two addresses to open: one for this machine, one for the other.
//   bun run tools/mp/lan.ts [--mission '?roomclear&squad4'] [--seed 7] [--port 5300] [--dev http://localhost:5173]
// EACH MACHINE RUNS ITS OWN PAGE (`bun run dev`, the same commit on both: the server turns a page of another build away). A page must come from localhost,
// because a browser gives WebGPU only to a secure address and another machine's plain http address is not one; only the game's socket crosses the network.
// The server ends itself when the mission is over, a quarter of an hour after the last player left, or four hours after it started. Ctrl+C stops it.
// To make the connection worse on purpose: the debugger's Network page on either machine (docs/agents/net.md).
import { resolve } from 'node:path';
import { networkInterfaces } from 'node:os';

const ROOT = resolve(import.meta.dir, '../..');
const arg = (k: string, d: string): string => { const i = process.argv.indexOf(k); return i > 0 ? process.argv[i + 1] ?? d : d; };
const mission = arg('--mission', '?roomclear&squad4'), seed = Number(arg('--seed', '7')), port = Number(arg('--port', '5300')), dev = arg('--dev', 'http://localhost:5173').replace(/\/+$/, '');
if (!mission.startsWith('?') || !Number.isInteger(seed) || !Number.isInteger(port)) { console.error('lan.ts: --mission is a page address like \'?roomclear&squad4\', --seed and --port are whole numbers'); process.exit(2); }
const key = (): string => Array.from(crypto.getRandomValues(new Uint8Array(6)), b => b.toString(16).padStart(2, '0')).join('');
const keys = [key(), key()], game = 'lan-' + key().slice(0, 6);
const scenario = /[?&]seats=/.test(mission) ? mission : `${mission}&seats=2`;
/** this machine's addresses on its networks (IPv4, not the loopback) */
const mine = Object.values(networkInterfaces()).flatMap(l => l ?? []).filter(a => a.family === 'IPv4' && !a.internal).map(a => a.address);

const srv = Bun.spawn([process.execPath, 'run', 'tools/mp/server.ts'], { cwd: ROOT, stdout: 'inherit', stderr: 'inherit',
  env: { ...process.env, MP_HOST: '0.0.0.0', MP_PORT: String(port), MP_GAME: game, MP_SETUP: JSON.stringify({ scenario, seed, seatKeys: keys }), MP_IDLE_S: process.env.MP_IDLE_S ?? '900', MP_GRACE_S: process.env.MP_GRACE_S ?? '60', MP_MAX_S: process.env.MP_MAX_S ?? '14400' } });
const stop = (): void => { try { srv.kill(); } catch { /* gone */ } };
process.on('SIGINT', stop); process.on('SIGTERM', stop);

// (up = THIS game answers on the port: another server left on it answers /health too, with its own game's name)
let up = false, other = '';
for (let i = 0; i < 300 && !up && srv.exitCode === null; i++) {
  try { const r = await fetch(`http://127.0.0.1:${port}/health`); if (r.ok) { const h = await r.json() as { game?: string }; if (h.game === game) up = true; else other = String(h.game ?? '?'); } } catch { /* not yet */ }
  if (!up) await Bun.sleep(200);
}
if (!up) { console.error(other ? `lan.ts: port ${port} already has a game server on it (game ${other}). Stop that one, or pass --port with another number.` : `lan.ts: the game server did not come up on port ${port} (its own lines above say why)`); stop(); process.exit(1); }
const page = (host: string, k: string): string => `${dev}/?net=${encodeURIComponent(`ws://${host}:${port}`)}&netgame=${game}&netkey=${k}`;
console.log(`\nA co-op game of ${scenario} is waiting on this machine. It starts when both players are in.\n`);
console.log(`  Open on THIS machine:\n    ${page('127.0.0.1', keys[0])}\n`);
if (mine.length) for (const a of mine) console.log(`  Open on the OTHER machine${mine.length > 1 ? ` (if it reaches this one at ${a})` : ''}:\n    ${page(a, keys[1])}\n`);
else console.log(`  Open on the OTHER machine, with this machine's address on your network in place of ADDRESS:\n    ${page('ADDRESS', keys[1])}\n`);
console.log(`Both machines run \`bun run dev\` on the same commit. If the other machine can't connect, let this one accept connections on port ${port}.`);
console.log('The debugger has a Network page on each to make the connection worse on purpose. Ctrl+C here stops the game.\n');
process.exit(await srv.exited);
