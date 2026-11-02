// TRY THE CONTROL PLANE, with no page. What a lobby screen will do, as lines of text: a game is made as one browser and joined as another, and its state
// is printed as it changes until it is over (a server nobody connects to ends itself after a minute).
//   bun run dev   and, in another terminal,   bun run tools/mp/runner-try.ts [http://localhost:5173]      this machine: the dev server's backend and build
//   bun run tools/mp/runner-try.ts --cloud https://<name>.convex.cloud --build <commit>                   a hosted backend: it starts a REAL server there
import { wire } from '../../src/cloud/wire.ts';

const arg = (k: string): string => { const i = process.argv.indexOf(k); return i > 0 ? process.argv[i + 1] ?? '' : ''; };
const dev = (process.argv[2] && !process.argv[2].startsWith('--') ? process.argv[2] : 'http://localhost:5173').replace(/\/+$/, '');
const ask = async (path: string): Promise<string> => { try { const r = await fetch(dev + path); return r.ok ? (await r.text()).trim() : ''; } catch { return ''; } };
const url = arg('--cloud') || await ask('/qa/cloud'), build = arg('--build') || await ask('/qa/commit');
if (!url) { console.error(`${dev} runs no backend (yet). Start it with \`bun run dev\` and wait for its "local backend: up at …" line.`); process.exit(1); }
const W = wire(url), id = (): string => Array.from(crypto.getRandomValues(new Uint8Array(20)), b => b.toString(16).padStart(2, '0')).join('');
const host = id(), guest = id();
interface View { code: string; state: string; why: string; seat: number; seatKey: string; address: string; connected: boolean[] }
try {
  const g = await W.mutation<View>('games:make', { pid: host, build, setup: { scenario: 'try', seed: 1, knobs: {}, seats: [{}, {}] } });
  console.log(`made game ${g.code} on build '${build}' (backend ${url}): ${g.state}`);
  const j = await W.mutation<View>('games:join', { pid: guest, code: g.code, build });
  console.log(`a second browser joined: seat ${j.seat + 1} of 2`);
  for (let last = '', t0 = Date.now(); ;) {
    const v = await W.query<View | null>('games:get', { pid: host, code: g.code });
    if (!v) break;
    const line = `${v.state}${v.why ? ` (${v.why})` : ''}${v.address ? ` at ${v.address}` : ''}, connected: ${v.connected.map(c => (c ? 'yes' : 'no')).join(' ')}`;
    if (line !== last) console.log(`${((Date.now() - t0) / 1000).toFixed(0).padStart(4)} s  ${last = line}`);
    if (v.state === 'ended' || v.state === 'failed') break;
    await Bun.sleep(500);
  }
} catch (e) { console.error(String((e as Error)?.message ?? e)); process.exit(1); }
