// Dice shared by the simulation's seams. mulberry32 is the one seeded die the QA harness, the checkpoints' identity claim and the session journal all mean by
// 'seeded' (tools/qa seeds Math.random with the same function). A die the session JOURNAL installs as Math.random for the simulation (rust/game_rules/src/journal) carries the
// die it displaced on `.sys`, and the systems that are NOT the simulation — the mixer's rate jitter and noise beds (audio/), the smoke solver's per-frame emitter
// jitter (smoke/smoke.ts) — draw through sysRandom(): under a recording they leave the seeded stream to the game alone (a replay has no mixer or solver to draw),
// while a plainly seeded Math.random (the capture recipes' determinism checklist, the harness) still reaches them exactly as it always did.
/** mulberry32: a small, fast, seedable uniform [0, 1) generator */
export function mulberry32(a: number): () => number { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** a die a rollback snapshot can hold (rust/game_rules/src/rollback): its whole state is the word `dice`, on an object, and nothing is captured — `mulberry32(seed)` keeps the same
 *  word inside a closure, where no snapshot can read it or put it back, so a die that LIVES IN GAME STATE from tick to tick is one of these (a die made and spent
 *  inside one call may stay a mulberry32). `roll(dieOf(seed))` gives the sequence `mulberry32(seed)` gives, draw for draw. */
export interface Die { dice: number }
export function roll(d: Die): number { const a = d.dice = (d.dice + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }
/** a small stable string hash (FNV-1a, 32 bit): the same string gives the same number on every machine — what things DEALT by name, never rolled, are dealt off */
export function hashStr(s: string): number { let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; } return h >>> 0; }
/** a die standing in for Math.random on the simulation's behalf, remembering the one it displaced */
export type SimDie = (() => number) & { sys?: () => number };
/** a draw for something that is not the simulation: the displaced system die under a recording, plain Math.random otherwise */
export function sysRandom(): number { const r = Math.random as SimDie; return (r.sys ?? r)(); }

// THE SIMULATION'S OWN DIE. Every draw the game makes goes through simRandom() below (no file under src/game calls Math.random for one any more: tools/qa/dice-check.ts
// holds that). A Game stood up with a seed (src/host/drawpage.ts StandOpts.seed — the page always is) OWNS a die: one mulberry32 word on the Game, so the stream is the
// game's alone (nothing else in the page can draw from it), a checkpoint carries where it stands (savestate.ts `dice`), and two machines given the same seed and the
// same inputs roll the same numbers — what a server and a client predicting off it need. A Game stood up without one (the QA harness, whose tools seed Math.random)
// draws Math.random exactly as it always did, number for number.
// ONE RULE keeps every older tool and recipe working: a die of the game's own is used only while Math.random is still the function it was when that die was made. The
// moment somebody puts a die of THEIR OWN in Math.random's place — a probe's seed, a capture recipe's, the session journal's recording die above, the civilians'
// private construction die — the game draws theirs, as it always has, and goes back to its own when they put Math.random back.

// THE KEYED DRAW (rust/game_types/src/dice.rs `keyed`, the same function to the bit: tools/qa/dice-check.ts and that file's own test hold both to one number).
// A fixed function of the game's seed, the SITE that asks, the man it is about (0: nobody, else his body's id + 1), the clock, and how many times this site has drawn
// for him at this clock: no draw moves another. The game rolls in Rust; this side rolls what STANDING A GAME UP rolls, under the Rust site's own name.
const W64 = (1n << 64n) - 1n, BITS = new DataView(new ArrayBuffer(8));
function stir(x: bigint): bigint { x ^= x >> 30n; x = x * 0xbf58476d1ce4e5b9n & W64; x ^= x >> 27n; x = x * 0x94d049bb133111ebn & W64; return x ^ (x >> 31n); }
export function keyed(seed: number, key: bigint, time: number, nth: number): number {
  BITS.setFloat64(0, time);
  const x = stir(stir(stir(BigInt(seed >>> 0) ^ 0x2545f4914f6cdd1dn) ^ key) ^ stir(BITS.getBigUint64(0)) ^ ((BigInt(nth) << 1n | 1n) * 0xd6e8feb86659fd93n & W64));
  return Number(stir(x) >> 11n) / 9007199254740992;
}
