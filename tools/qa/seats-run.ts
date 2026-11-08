// A TWO-SEAT FIGHT AS RECORDS (src/game/seats.ts; the Rust port's two-seat bring-up reads this): a game of two stood up headless, both seats driven by scripted
// hands, every tick stepped by `Game.stepSeats(dt, [record, record])` and nothing else — so the fight IS its records, and any stepper handed the same stand
// and the same records (the TypeScript tick again, the Rust `game_step`) must land on the same state words.
//   bun run tools/qa/seats-run.ts                       the check: record 40 s, play the records again in a fresh game, the state's word the same every
//                                                       second; exit 0 = pass. The page's stand; TPS_PROFILE=frozen is the old one
//   SEATSRUN_BREAK=1 bun run tools/qa/seats-run.ts      the red path: the replay hands each seat the OTHER seat's records; `replay` must FAIL
//   bun run tools/qa/seats-run.ts --record f.json [--query 'roomclear&squad4'] [--seed 9] [--secs 40] [--level compound]      write a recording
//   bun run tools/qa/seats-run.ts --replay f.json       play a recording in a fresh game on this build and stand; exit 1 where a word parts
// For a harness: `standTwo` stands the game, `hands` gives the scripted players, `sample(i)` is tick i's two records (as they cross the wire: cmd.ts cmdText
// and back), `tick(cmds)` is one step. A recording is { query, seed, level, stand, seats, dt, start, records[tick][seat] as text, words[second] }.
// WHAT IT IS NOT: a mission played well. The hands are seats-check's (each man walks at the nearest enemy and fires, a key of the game's every second and a
// half); no playthrough mission stands two seats, and the scripted commander of tools/qa/playthrough.ts plays one seat. The mission kinds a game of two
// takes are the room clear and the night breach (seats.ts SEAT_KINDS). A `levelFile=` word in the query does not change the floor here: pass `level`.
// The die is the game's own (`seed`), and Math.random is seeded before the stand as seats-check does: the same call makes the same recording, to the byte.
import { readFileSync, writeFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { sampleCmd, cmdText, cmdFromText } = await import(`${ROOT}/src/game/cmd.ts`);
const { Input } = await import(`${ROOT}/src/host/input.ts`);
const { FollowCamera } = await import(`${ROOT}/src/host/camera.ts`);
const { stateHash } = await import(`${ROOT}/src/game/rollback.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { standOf } = await import(`${ROOT}/src/host/standEnv.ts`);

export const DT = 1 / 60;
export interface Recording { what: 'seats-run 1'; query: string; seed: number; level: string | null; stand: 'frozen' | 'shipped'; seats: 2; dt: '1/60'; start: string; records: string[][]; words: string[] }
export interface Two { W: any; g: any; /** tick i's two records, off the scripted hands */ sample(i: number): any[]; /** one step on two records */ tick(cmds: any[]): void; word(): string }
const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const KEYS = ['KeyG', 'KeyF', 'KeyE', 'KeyQ', 'KeyB', 'KeyC', 'KeyR', 'KeyV', 'KeyT', 'KeyH', 'KeyN', 'KeyX', 'KeyZ', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'Space', 'Tab'];

/** a game of two on `query` (the room clear with four by default), its die `seed`, its floor `level` (a name under levels/, or the page's own). `idle`: the
 *  hands do nothing (no key, no button, the cursor where it lies): for a driver that gives its orders between ticks and wants the players out of the way */
export async function standTwo(query = 'roomclear&squad4', seed = 9, level: string | null = null, idle = false, dress = false): Promise<Two> {
  Math.random = mulberry32(0xc0de);
  const W: any = await standUp({ seed, seats: 2, address: query, ...(dress ? { dress: true } : {}), ...(level ? { level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${level}.json`, 'utf8'))) } : {}) }), g = W.game;
  const r = W.start(); if (r.why) throw new Error(`seats-run: ${query} refused: ${r.why}`);
  const v0 = g.seats[0].view, v1 = g.seats[1].view, I1: any = Input.detached(), cam1: any = new FollowCamera(); cam1.aspect = W.cam.aspect; cam1.target = [...v1.player.char.pos]; cam1.rebuild();
  const hands: any[] = [{ v: v0, I: W.input, cam: W.cam, held: null }, { v: v1, I: I1, cam: cam1, held: null }];
  /** the hands for tick i: walk at the nearest enemy, the cursor on him, a burst every four seconds, a key of the game's every second and a half */
  const drive = (i: number) => { if (!idle) hands.forEach((h, hi) => { const I = h.I; for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k);
    const p = h.v.player.char.pos, foe = g.guards.filter((x: any) => x.state !== 'dead' && x.team !== 'operative').sort((a: any, b: any) => flat(a.char.pos, p) - flat(b.char.pos, p))[0];
    const t = foe ? foe.char.pos : [p[0] + Math.sin(i / 200 + hi) * 8, 0, p[2] + Math.cos(i / 200 + hi) * 8], dx = t[0] - p[0], dz = t[2] - p[2], d = Math.hypot(dx, dz) || 1;
    const { fwd, right } = h.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / d, b = (dx * right[0] + dz * right[2]) / d, T = 0.3827;
    if (d > 3) { if (a > T) I.keys.add('KeyW'); if (a < -T) I.keys.add('KeyS'); if (b > T) I.keys.add('KeyD'); if (b < -T) I.keys.add('KeyA'); }
    const [sx, sy, front] = h.cam.project([t[0], 1.2, t[2]], W.canvas.clientWidth, W.canvas.clientHeight); if (front) { I.mouseX = sx; I.mouseY = sy; }
    const ph = (i + hi * 37) % 240; if (ph > 120 && ph < 200) { I.buttons |= 1; if (ph === 121) I.clicked |= 1; } else I.buttons &= ~1;
    if ((i + hi * 53) % 90 === 0) { const k = KEYS[Math.floor((i + hi * 53) / 90) % KEYS.length]; I.keys.add(k); I.pressed.add(k); h.held = k; } else if ((i + hi * 53) % 90 === 6 && h.held) { I.keys.delete(h.held); h.held = null; } }); };
  return { W, g,
    sample(i: number) { drive(i); return hands.map(h => cmdFromText(cmdText(sampleCmd(h.v, h.I, h.cam, W.canvas)))); },
    /** the tick and what a headless host does round it (headless.ts step, less the sampling): the lamps, the boxes, each seat's lens and the end of its input's frame */
    tick(cmds: any[]) { g.stepSeats(DT, cmds); g.engine.lights.update(DT, g.time); g.engine.world.dynamics = g.dynBoxes; g.engine.world.upload();
      for (const h of hands) { h.cam.update(DT, h.v.followPos(), h.v.aimPoint); h.I.endFrame(); } },
    // (THE WORD. `stateHash` walks the JavaScript objects of the game, which a game that is the puppet of another stepper cannot equal field for field;
    // under the harness's word TPS_GAME (tools/blitz/PLAY.md: `ts`, `rust`, `audit`) the word is the game's digest, the same reading on every side)
    word: () => process.env.TPS_GAME ? hashDigest(digestGame(g)) : stateHash(g, false, true) };
}
const standName = (): 'frozen' | 'shipped' => standOf(process.env.TPS_PROFILE);   // (standEnv.ts: no word is the page's stand; TPS_PROFILE=frozen the old one)
/** play `secs` of the scripted fight and keep its records */
export async function record(query = 'roomclear&squad4', seed = 9, secs = 40, level: string | null = null): Promise<Recording> {
  const P = await standTwo(query, seed, level), R: Recording = { what: 'seats-run 1', query, seed, level, stand: standName(), seats: 2, dt: '1/60', start: P.word(), records: [], words: [] };
  for (let i = 0; i < Math.round(secs * 60); i++) { const cmds = P.sample(i); R.records.push(cmds.map(cmdText)); P.tick(cmds); if (i % 60 === 59) R.words.push(P.word()); }
  return R;
}
/** play a recording's records in a fresh game: the second its word first parts from the recording's (-1: never), and the words it made. `swap`: the red path */
export async function replay(R: Recording, swap = false): Promise<{ part: number; start: string; words: string[] }> {
  const P = await standTwo(R.query, R.seed, R.level), start = P.word(), words: string[] = [];
  for (let i = 0; i < R.records.length; i++) { const c = R.records[i].map(cmdFromText); P.tick(swap ? [c[1], c[0]] : c); if (i % 60 === 59) words.push(P.word()); }
  let part = start === R.start ? -1 : 0; for (let k = 0; part < 0 && k < Math.max(words.length, R.words.length); k++) if (words[k] !== R.words[k]) part = k + 1;
  return { part, start, words };
}

if (import.meta.main) {
  const A = process.argv.slice(2), arg = (k: string, d: string | null = null) => { const i = A.indexOf(k); return i >= 0 && A[i + 1] !== undefined ? A[i + 1] : d; };
  const query = arg('--query', 'roomclear&squad4')!, seed = Number(arg('--seed', '9')), secs = Number(arg('--secs', '40')), level = arg('--level');
  if (arg('--record')) {
    const R = await record(query, seed, secs, level); writeFileSync(arg('--record')!, JSON.stringify(R));
    console.log(`seats-run: ${R.records.length} ticks of ${query}${level ? ` on ${level}` : ''}, die ${seed}, the ${R.stand === 'shipped' ? "page's" : 'old frozen'} stand → ${arg('--record')} · starts on ${R.start} · ends on ${R.words[R.words.length - 1] ?? R.start}`);
  } else if (arg('--replay')) {
    const R = JSON.parse(readFileSync(arg('--replay')!, 'utf8')) as Recording;
    if (R.what !== 'seats-run 1' || R.stand !== standName()) { console.log(`seats-run: ${arg('--replay')} is ${R.what === 'seats-run 1' ? `the ${R.stand} stand's, and this run is the ${standName()} stand's` : 'not a recording of this tool'}`); process.exit(2); }
    const got = await replay(R);
    console.log(got.part < 0 ? `seats-run: ${R.records.length} ticks replayed, the state's word the same at the start and at each of ${R.words.length} seconds · ends on ${got.words[got.words.length - 1] ?? got.start}`
      : got.part === 0 ? `seats-run: THE STAND PARTS before a tick: ${got.start} | ${R.start} (another build, another stand, or another floor)` : `seats-run: THE GAMES PART in second ${got.part}: ${got.words[got.part - 1]} | ${R.words[got.part - 1]}`);
    process.exit(got.part < 0 ? 0 : 1);
  } else {
    const BREAK = process.env.SEATSRUN_BREAK === '1'; let fails = 0;
    const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
    console.log(`=== seats-run: a two-seat fight is its records · ${standName() === 'shipped' ? "the page's" : 'the old frozen'} stand${BREAK ? ' · SEATSRUN_BREAK: the replay hands each seat the other seat\'s records' : ''} ===`);
    const R = await record(query, seed, secs, level);
    const moved = new Set(R.words).size, differ = R.records.filter(t => t[0] !== t[1]).length;
    verdict(R.records.length === Math.round(secs * 60) && R.words.length === Math.round(secs) && moved > secs / 2 && differ > R.records.length / 2, `record: ${secs} s of ${query} from two seats' records`,
      `${R.records.length} ticks · ${moved} different words in ${R.words.length} seconds · the two seats' records differ on ${differ} ticks · about ${Math.round(JSON.stringify(R).length / 1024)} KB as text · starts on ${R.start}, ends on ${R.words[R.words.length - 1]}`);
    const got = await replay(R, BREAK);
    verdict(got.part < 0, 'replay: the records played again in a fresh game are the same game', got.part < 0 ? `the state's word the same at the start and at each of ${R.words.length} seconds`
      : got.part === 0 ? `THE STAND PARTS before a tick: ${got.start} | ${R.start}` : `THE GAMES PART in second ${got.part}: ${got.words[got.part - 1]} | ${R.words[got.part - 1]}`);
    const again = await record(query, seed, secs, level);
    verdict(JSON.stringify(again) === JSON.stringify(R), 'twice: the same call makes the same recording, to the byte', `${JSON.stringify(again).length} bytes | ${JSON.stringify(R).length} bytes${JSON.stringify(again) === JSON.stringify(R) ? '' : ` · first tick apart ${again.records.findIndex((t, i) => t.join() !== R.records[i]?.join())}`}`);
    console.log(fails ? `=== seats-run: ${fails} FAILED ===` : '=== seats-run: ALL PASS ===');
    process.exit(fails ? 1 : 0);
  }
}
