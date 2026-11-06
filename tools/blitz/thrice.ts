// THE SAME GAME STOOD N TIMES IN ONE PROCESS (tools/blitz/PLAY.md, "A tool that stands many games"): a two-seat fight recorded once (tools/qa/seats-run.ts's
// stand and its scripted hands), then its records played again in fresh games of the same process, the game's digest read every second and held to the
// first game's. A tool that stands hundreds of games a process (the probe runner) needs every one of them to be the game a fresh process would have played.
//   tools/blitz/rust.sh rust bun run tools/blitz/thrice.ts [--games 3] [--secs 40] [--query 'roomclear&squad4'] [--seed 9] [--one]
//       [--ticks DIR]      a line a tick a game in DIR/<word>-g<n>.txt: the tick, the die's count, a hash of each part of rust/sim's block
//       [--dump DIR --at 1019,1020]   the whole state of every game as those ticks return, DIR/<word>-g<n>-t<k>.bin (tools/blitz/diff.ts holds two to each other)
//   --one: one seat, nobody at the keys (`W.step` and nothing else)
//   --weave: after the first game, the later games are stood TOGETHER and stepped in turn, a tick each (two games alive and stepping in one process)
//   exit 0: one digest at every second in every game · 1: a game parted from the first (the second it parted in is said)
import { mkdirSync, writeFileSync, appendFileSync } from 'node:fs';
const A = process.argv.slice(2), arg = (k: string, d: string) => { const i = A.indexOf(k); return i >= 0 && A[i + 1] !== undefined ? A[i + 1] : d; };
const GAMES = +arg('--games', '3'), SECS = +arg('--secs', '40'), QUERY = arg('--query', 'roomclear&squad4'), SEED = +arg('--seed', '9'), ONE = A.includes('--one');
const TICKS = arg('--ticks', ''), DUMP = arg('--dump', ''), AT = new Set(arg('--at', '').split(',').filter(Boolean).map(Number)), WORD = process.env.TPS_GAME ?? 'none';
const { standTwo, DT } = await import('../qa/seats-run.ts');
const { standUp, ROOT } = await import('../qa/headless.ts');
const { cmdText, cmdFromText } = await import(`${ROOT}/src/game/cmd.ts`);
const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
for (const d of [TICKS, DUMP]) if (d) mkdirSync(d, { recursive: true });

// (the block's parts, by the module's own map: the same reading of the TypeScript game's block and of a puppet's copy of the Rust one)
let parts: { name: string; off: number; len: number }[] | null = null, bridge: any = null;
if (TICKS || DUMP) { const RH = await import('./rust-host.ts'), F = RH.loadFace(), X = F.X, base = X.state_addr(0) >>> 0;
  parts = F.NAMES.map((name: string, i: number) => ({ name, off: (X.face_part_addr(0, i) >>> 0) - base, len: X.face_part_len(i) })).filter((p: any) => p.name !== 'poses');
  if (TICKS) writeFileSync(`${TICKS}/names.txt`, parts.map(p => p.name).join(',') + '\n');
  bridge = { exportGame: (await import('./export.ts')).exportGame, reg: (await import(`${ROOT}/src/game/knobs.ts`)).knobRegistry() }; }
function tickOut(n: number, k: number, g: any): void {
  if (TICKS && parts) { const w: Uint32Array = g.sim.w, b = new Uint8Array(w.buffer, w.byteOffset, w.byteLength); let l = `${k} ${g.dice?.n}`; for (const p of parts) l += ` ${Bun.hash(b.subarray(p.off, p.off + p.len)).toString(36)}`; appendFileSync(`${TICKS}/${WORD}-g${n}.txt`, l + '\n'); }
  if (DUMP && AT.has(k)) { const H = g.rustHost; let blob: Uint8Array; if (H) { H.F.X.face_export(H.h); blob = H.F.outView().slice(); } else { g.sim.settle(g); blob = bridge.exportGame(g, bridge.reg); } writeFileSync(`${DUMP}/${WORD}-g${n}-t${k}.bin`, blob); } }

const N = Math.round(SECS * 60); let records: string[][] | null = null; const words: string[][] = [];
const WEAVE = A.includes('--weave') && !ONE;
const said = (n: number, my: string[], g: any) => { const part = my.findIndex((w, k) => w !== words[0][k]);
  console.log(`game ${n}: ${N} ticks · starts on ${my[0]} · ends on ${my[my.length - 1]} · the die rolled ${g.dice?.n} times${n === 1 ? '' : part < 0 ? ' · THE SAME as the first at the start and at every second' : part === 0 ? ' · PARTS FROM THE FIRST BEFORE A TICK' : ` · PARTS FROM THE FIRST in second ${part}`}`); };
for (let n = 1; n <= (WEAVE ? 1 : GAMES); n++) {
  if (TICKS) writeFileSync(`${TICKS}/${WORD}-g${n}.txt`, '');
  const my: string[] = []; let g: any, tick: (i: number) => void;
  if (ONE) { Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed: SEED }); g = W.game; const r = startFromQuery(g, QUERY); if (r.why) throw new Error(`thrice: ${QUERY} refused: ${r.why}`); tick = () => W.step(DT); }
  else { const P = await standTwo(QUERY, SEED, null); g = P.g; const mine: string[][] = [];
    tick = (i: number) => { const cmds = records ? records[i].map(cmdFromText) : P.sample(i); if (!records) mine.push(cmds.map(cmdText)); P.tick(cmds); if (!records && i === N - 1) records = mine; }; }
  my.push(hashDigest(digestGame(g)));
  for (let i = 0; i < N; i++) { tick(i); tickOut(n, i + 1, g); if (i % 60 === 59) my.push(hashDigest(digestGame(g))); }
  words.push(my); said(n, my, g);
}
if (WEAVE) { const P: any[] = [], W: string[][] = [];
  for (let n = 2; n <= GAMES; n++) { if (TICKS) writeFileSync(`${TICKS}/${WORD}-g${n}.txt`, ''); const p = await standTwo(QUERY, SEED, null); P.push(p); W.push([hashDigest(digestGame(p.g))]); }
  for (let i = 0; i < N; i++) P.forEach((p, j) => { p.tick(records![i].map(cmdFromText)); tickOut(j + 2, i + 1, p.g); if (i % 60 === 59) W[j].push(hashDigest(digestGame(p.g))); });
  P.forEach((p, j) => { words.push(W[j]); said(j + 2, W[j], p.g); }); }
const bad = words.filter(w => w.join() !== words[0].join()).length;
console.log(bad ? `=== thrice (${WORD}): ${bad} of ${GAMES - 1} later games PLAYED ANOTHER GAME than the first ===` : `=== thrice (${WORD}): ${GAMES} games of one process, one digest at the start and at each of ${SECS} seconds: ${words[0][words[0].length - 1]} ===`);
process.exit(bad ? 1 : 0);
