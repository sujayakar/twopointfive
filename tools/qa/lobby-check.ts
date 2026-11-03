// HOST OR JOIN A CO-OP GAME — the lobby, checked with no page (src/net/lobby.ts; the screen is src/ui/coop.ts, driven by tools/qa/lobby-page.cjs):
//   bun run tools/qa/lobby-check.ts          NEEDS the backend's own packages: `npm ci` in cloud/ once
// Three things stand here, on this machine, and nothing hosted is touched:
//   the backend   a STAND-IN (cloud/test/standin.ts): the real functions and schema in the service's test library
//   the runner    the REAL local runner (tools/mp/runner.ts): what `bun run dev` is to the local backend
//   the server    the REAL game server (tools/mp/server.ts), started by that runner for the game the lobby made
//   words      a typed code (and the control plane's own letters for one), the mission's address as a game's setup, the addresses of the door and of the
//              game, the refusals as the player is told them — also when a deployed backend keeps a refusal's words in its data
//   settings   the host's difficulty is the server's: the recipe stood from a Hard address differs from a Standard one's in the tables Hard writes
//   host       one tab makes a game of a squad mission and comes through the door: it holds the first seat and is told its server's address; the server
//              stood the mission with TWO seats and waits
//   join       a second tab comes through the same door by the code: the second seat, its own key, the same server; the first tab again is the first seat
//              again; a third tab is told the game has two players; a code nobody has is told so
//   fight      a scripted client in each seat, on the address and key its tab was told: both walk the same ticks and match every state word
// The red path: LOBBY_BREAK=1 (the host's game is made for one player) must FAIL `host`, `join` and `fight`.
// NOT HERE: a browser (lobby-page.cjs), the service itself, a sandbox. EXIT 0 all pass · 1 a failure · 2 the backend's packages are not installed.
import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { wire } from '../../src/cloud/wire.ts';
import { localRunner } from '../mp/runner.ts';
import { COOP, CODE, Lobby, codeTyped, codeWhole, coopScenario, joinLink, netSearch, lobbyWhy, gameWhy, freshId, freshSeed, type GameView } from '../../src/net/lobby.ts';

const ROOT = resolve(import.meta.dir, '../..');
if (!existsSync(`${ROOT}/cloud/node_modules/convex-test`) || !existsSync(`${ROOT}/cloud/node_modules/modal`)) { console.error('lobby-check: the backend\'s packages are not installed — run `npm ci` in cloud/ first'); process.exit(2); }
const { standIn } = await import(`${ROOT}/cloud/test/standin.ts`);
// (the control plane's own numbers, read as the stand-in is: by a path the tools' typecheck does not follow, so that program needs none of the backend's packages)
const { MP } = await import(`${ROOT}/cloud/convex/games.ts`) as { MP: { codeLen: number; codeLetters: string } };
const BREAK = process.env.LOBBY_BREAK === '1';
if (BREAK) COOP.seats = 1;

let fails = 0;
const check = (ok: boolean, text: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${text}`); if (!ok) fails++; };
const why = async (p: Promise<unknown>): Promise<string> => { try { await p; return ''; } catch (e) { return String((e as Error)?.message ?? e); } };
const BUILD = 'c0ffee123456', RKEY = 'runner-key-of-the-check-0123456789';

console.log('words');
check(codeTyped(' bq-7zk w!x ') === 'BQZKW' && codeWhole('BQZKW') && !codeWhole('BQZK') && !codeWhole('BAZKW') && codeTyped('aeiou12345') === '' && CODE.len === MP.codeLen && CODE.letters === MP.codeLetters, 'a typed code keeps its letters, in capitals, up to a code\'s length; a short one or one with a vowel is no code; the letters and the length are the control plane\'s own');
{ const s = coopScenario('?roomclear&levelFile=barracks.json&squad4&hour=day&loadout=barracks', [['fog', 'thin'], ['hour', 'night']]), q = new URLSearchParams(s);
  check(q.get('squad') === String(COOP.squad) && q.get('seats') === String(COOP.seats) && !q.has('squad4') && !q.has('loadout') && q.get('levelFile') === 'barracks.json' && q.get('hour') === 'day' && q.get('fog') === 'thin' && /^\?roomclear&/.test(s),
    `the mission's address as a game's setup: the four, two seats, the kept lever on it, what the address said left alone (${s})`); }
{ const n = new URLSearchParams(netSearch('?quality=low&mute', { address: 'ws://localhost:5301', game: 'g1', seatKey: 'k', code: 'BQZKW' }));
  check(n.get('net') === 'ws://localhost:5301' && n.get('netgame') === 'g1' && n.get('netkey') === 'k' && n.get('join') === 'BQZKW' && n.get('quality') === 'low' && joinLink('http://localhost:5173', '/', 'BQZKW') === 'http://localhost:5173/?join=BQZKW',
    'the game\'s address keeps the visit\'s words and carries the server, the game, the seat\'s key and the code; the link is the page and the code'); }
{ const R = (m: string) => ({ message: m, refused: true });
  check(/two players/.test(lobbyWhy(R('games:join: full: every seat is taken'), 'join')) && /busy/.test(lobbyWhy(R('games:make: full: every game server is taken'), 'host')) && /Reload/.test(lobbyWhy(R('x: build: another'), 'join')) && /no game with that code/.test(lobbyWhy(R('x: code: no game'), 'join')) && /reach/.test(lobbyWhy({ message: 'fetch failed', refused: false }, 'join'))
    && /over/.test(gameWhy({ state: 'ended', seat: 0 } as GameView)) && /didn’t start/.test(gameWhy({ state: 'failed', seat: 0 } as GameView)) && gameWhy({ state: 'waiting', seat: 1 } as GameView) === '' && /two players/.test(gameWhy({ state: 'waiting', seat: -1 } as GameView)),
    'every refusal has its own words for the player'); }
{ // (a deployed backend's refusal: the message says nothing, the words are the error's data — src/cloud/wire.ts folds them in)
  const W = wire('http://backend.invalid', { fetch: (async () => new Response(JSON.stringify({ status: 'error', errorMessage: 'Server Error', errorData: 'full: every seat is taken' }))) as unknown as typeof fetch });
  const e = await W.mutation('games:join', {}).then(() => null, (x: unknown) => x);
  check(/two players/.test(lobbyWhy(e, 'join')), `a refusal whose words are in its data is still told in its own words ("${lobbyWhy(e, 'join')}")`); }
check(/^[a-z0-9]{40}$/.test(freshId()) && freshId() !== freshId() && Number.isInteger(freshSeed()) && freshSeed() >= 0 && freshSeed() <= 0xFFFFFFFF, 'a tab\'s id is one the control plane takes, and a new game\'s die one the server takes');

console.log('settings');
{ const recipe = async (search: string): Promise<{ tables: Record<string, unknown> } | null> => { const p = Bun.spawn(['bun', 'run', 'tools/mp/setup.ts', JSON.stringify({ search, seed: 7 })], { cwd: ROOT, stdout: 'pipe', stderr: 'ignore' }); const l = (await new Response(p.stdout).text()).split('\n').find(x => x.startsWith('MPSETUP ')); return l ? JSON.parse(l.slice(8)) as never : null; };
  const at = (d: string) => coopScenario(`?roomclear&difficulty=${d}`), [std, hard, bare] = await Promise.all([recipe(at('standard')), recipe(at('hard')), recipe(coopScenario('?roomclear'))]);
  const moved = std && hard ? Object.keys(std.tables).filter(k => JSON.stringify(std.tables[k]) !== JSON.stringify(hard.tables[k])) : [];
  check(!!std && !!hard && !!bare && moved.length > 0 && JSON.stringify(bare.tables) === JSON.stringify(std.tables), `the host's difficulty is the server's: a Hard address stands a recipe that differs from a Standard one's in ${moved.join(', ') || 'nothing'}, and an address that names none is Standard`); }

// ---- the flow: a stand-in backend, the real runner, the real server
const S = standIn();
const admin = wire(S.url, { key: S.key });
const runner = localRunner({ key: RKEY, build: BUILD });
const RS = Bun.serve({ port: 0, hostname: '127.0.0.1', fetch: async (req: Request) => (await runner.handle(req)) ?? new Response('not found', { status: 404 }) });
await admin.mutation('runners:register', { name: 'check', url: `http://127.0.0.1:${RS.port}/mp/runner`, key: RKEY, build: BUILD, cap: 2 });
const cron = setInterval(() => void admin.mutation('games:reap', {}).catch(() => { /* asked again */ }), 500);
const tab = () => new Lobby(wire(S.url), freshId(), BUILD);
const health = async (address: string): Promise<{ state: string; tick: number; seats: boolean[] } | null> => { try { return await (await fetch(address.replace(/^ws/, 'http') + '/health')).json() as never; } catch { return null; } };

try {
  console.log('host');
  const A = tab(), B = tab(), states: string[] = [];
  const made = await A.make(coopScenario('?roomclear&squad4'), 7);
  check(codeWhole(made.code) && made.seat === 0 && made.state === 'starting' && made.seats.length === COOP.seats, `a game is made: code ${made.code}, the host in the first seat of ${made.seats.length}, its server asked for`);
  const a = await A.seat(made.code, v => { if (states[states.length - 1] !== v.state) states.push(v.state); });
  const h0 = await health(a.address);
  check(a.seat === 0 && /^ws:\/\/localhost:\d+$/.test(a.address) && !!a.seatKey && !!a.game && states[0] === 'starting' && !!h0 && h0.state === 'waiting' && h0.seats.length === 2 && h0.tick === 0,
    `the host comes through the door: ${states.join(' → ')}, then its server's address; the server stood the mission with ${h0?.seats.length ?? '?'} seats and waits (${h0?.state}, tick ${h0?.tick})`);

  console.log('join');
  const bWhy = await why(B.seat(made.code.toLowerCase())), b = bWhy ? null : await B.join(made.code);
  check(!!b && b.seat === 1 && b.address === a.address && b.game === a.game && !!b.seatKey && b.seatKey !== a.seatKey, `a second tab comes through by the code (typed small): ${b ? `seat ${b.seat}, its own key, the same server` : bWhy}`);
  const again = await A.join(made.code);
  check(again.seat === 0 && again.seatKey === a.seatKey, 'the first tab, loaded again, is the first seat again');
  const third = await why(tab().seat(made.code)), none = await why(tab().seat('ZZZZZ'));
  check(/already has two players/.test(third) && /no game with that code/.test(none), `a third tab is told "${third}"; a code nobody has, "${none}"`);

  console.log('fight');
  const bot = (v: GameView) => { const p = Bun.spawn(['bun', 'run', 'tools/mp/bot.ts', '--url', v.address, '--game', v.game, '--key', v.seatKey, '--build', BUILD, '--ticks', '240', '--timeout', '120'], { cwd: ROOT, stdout: 'pipe', stderr: 'ignore' });
    return new Response(p.stdout).text().then(t => { const l = t.split('\n').find(x => x.startsWith('MPBOT ')); return l ? JSON.parse(l.slice(6)) as { seat: number; confirmed: number; wordsOk: number; bad: unknown; word: string } : null; }); };
  const [ba, bb] = b ? await Promise.all([bot(a), bot(b)]) : [null, null];
  check(!!ba && !!bb && ba.seat === 0 && bb.seat === 1 && ba.confirmed >= 240 && bb.confirmed >= 240 && !ba.bad && !bb.bad && ba.wordsOk >= 8 && bb.wordsOk >= 8,
    `a client in each seat, on the address and key its tab was told: ${ba && bb ? `seats ${ba.seat} and ${bb.seat} walked ${ba.confirmed} and ${bb.confirmed} ticks and matched ${ba.wordsOk} and ${bb.wordsOk} state words` : 'they did not both play'}`);
} catch (e) { check(false, `the flow stopped: ${String((e as Error)?.message ?? e)}`); }

clearInterval(cron); runner.stopAll(); RS.stop(true); S.stop();
console.log(fails ? `lobby-check: ${fails} FAILED${BREAK ? ' (LOBBY_BREAK=1: host, join and fight must)' : ''}` : 'lobby-check: all pass');
process.exit(fails ? 1 : 0);
