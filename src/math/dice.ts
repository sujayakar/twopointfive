// Dice shared by the simulation's seams. mulberry32 is the one seeded die the QA harness, the checkpoints' identity claim and the session journal all mean by
// 'seeded' (tools/qa seeds Math.random with the same function). A die the session JOURNAL installs as Math.random for the simulation (game/journal.ts) carries the
// die it displaced on `.sys`, and the systems that are NOT the simulation — the mixer's rate jitter and noise beds (audio/), the smoke solver's per-frame emitter
// jitter (smoke/smoke.ts) — draw through sysRandom(): under a recording they leave the seeded stream to the game alone (a replay has no mixer or solver to draw),
// while a plainly seeded Math.random (the capture recipes' determinism checklist, the harness) still reaches them exactly as it always did.
/** mulberry32: a small, fast, seedable uniform [0, 1) generator */
export function mulberry32(a: number): () => number { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** a die a rollback snapshot can hold (game/rollback.ts): its whole state is the word `dice`, on an object, and nothing is captured — `mulberry32(seed)` keeps the same
 *  word inside a closure, where no snapshot can read it or put it back, so a die that LIVES IN GAME STATE from tick to tick is one of these (a die made and spent
 *  inside one call may stay a mulberry32). `roll(dieOf(seed))` gives the sequence `mulberry32(seed)` gives, draw for draw. */
export interface Die { dice: number }
export const dieOf = (seed: number): Die => ({ dice: seed | 0 });
export function roll(d: Die): number { const a = d.dice = (d.dice + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }
/** the same draw as a method of the record that carries the word: `rec.rnd = rollOn; rec.rnd()` — one function for every record, so a record put into another game
 *  rolls its own die there (a closure over the record would go on rolling the first game's) */
export function rollOn(this: Die): number { return roll(this); }
/** a small stable string hash (FNV-1a, 32 bit): the same string gives the same number on every machine — what things DEALT by name, never rolled, are dealt off */
export function hashStr(s: string): number { let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; } return h >>> 0; }
/** a die standing in for Math.random on the simulation's behalf, remembering the one it displaced */
export type SimDie = (() => number) & { sys?: () => number };
/** a draw for something that is not the simulation: the displaced system die under a recording, plain Math.random otherwise */
export function sysRandom(): number { const r = Math.random as SimDie; return (r.sys ?? r)(); }

// THE SIMULATION'S OWN DIE. Every draw the game makes goes through simRandom() below (no file under src/game calls Math.random for one any more: tools/qa/dice-check.ts
// holds that). A Game stood up with a seed (host/standup.ts StandOpts.seed — the page always is) OWNS a die: one mulberry32 word on the Game, so the stream is the
// game's alone (nothing else in the page can draw from it), a checkpoint carries where it stands (savestate.ts `dice`), and two machines given the same seed and the
// same inputs roll the same numbers — what a server and a client predicting off it need. A Game stood up without one (the QA harness, whose tools seed Math.random)
// draws Math.random exactly as it always did, number for number.
// ONE RULE keeps every older tool and recipe working: a die of the game's own is used only while Math.random is still the function it was when that die was made. The
// moment somebody puts a die of THEIR OWN in Math.random's place — a probe's seed, a capture recipe's, the session journal's recording die above, the civilians'
// private construction die — the game draws theirs, as it always has, and goes back to its own when they put Math.random back.
/** a game's own die: `s` = the mulberry32 word (the whole state), `n` = draws taken off it, `seed` = what it started from, `sys` = Math.random as it was when the die was made.
 *  WHERE THE WORD AND THE COUNT LIVE: in the two fields here — or in a `home` somebody gave the die (src/sim/dicehome.ts: while the row RUST.dice is on, the
 *  game's block of the Rust module's memory, where a draw is rolled by rust/sim/src/dice.rs). `s` and `n` read and write whichever it is, so nobody who asks a
 *  die where it stands has to know; and this file names no home, so the mixer and the menus that import it for sysRandom() bring none along. */
export interface DiceHome { s: number; n: number; roll(): number }
export class SimDice {
  /** where the word and the count are while they are not `s0` and `n0` (null: they are) */
  home: DiceHome | null = null;
  s0: number; n0 = 0;
  seed: number; sys: () => number;
  /** the keyed draws made at this clock: how many of each key (simDraw) */
  at = 0; drawn = new Map<bigint, number>();
  constructor(seed: number) { this.s0 = seed | 0; this.seed = seed >>> 0; this.sys = Math.random; }
  get s(): number { const h = this.home; return h ? h.s : this.s0; }
  set s(v: number) { const h = this.home; if (h) h.s = v; else this.s0 = v | 0; }
  get n(): number { const h = this.home; return h ? h.n : this.n0; }
  set n(v: number) { const h = this.home; if (h) h.n = v; else this.n0 = v; }
}
let cur: SimDice | null = null;
/** a die for a Game, at the start of `seed`'s stream (the same numbers mulberry32(seed) gives) */
export function newDice(seed: number): SimDice { return new SimDice(seed); }
/** make `d` the die simRandom() draws (null: none — Math.random); returns the one it replaced. Game.update's first statement, the stand-up and a load call it */
export function useDice(d: SimDice | null): SimDice | null { const p = cur; cur = d; return p; }
/** the die in use (null: the game draws Math.random) — false also while somebody else's die stands in Math.random's place */
export function diceInUse(): SimDice | null { return cur && Math.random === cur.sys ? cur : null; }
/** a draw for the SIMULATION, uniform in [0, 1): the Game's own die when it has one, Math.random otherwise (see the rule above) */
export function simRandom(): number {
  const d = cur; if (!d || Math.random !== d.sys) return Math.random();
  const h = d.home; if (h !== null) return h.roll();   // (the die is somewhere else — the game's block of the Rust module: rolled there, the same draw)
  d.n0++; const a = d.s0 = (d.s0 + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
}

// THE KEYED DRAW (rust/game_types/src/dice.rs `keyed`, the same function to the bit: tools/qa/dice-check.ts and that file's own test hold both to one number).
// A fixed function of the game's seed, the SITE that asks, the man it is about (0: nobody, else his body's id + 1), the clock, and how many times this site has drawn
// for him at this clock: no draw moves another. The game rolls in Rust; this side rolls what STANDING A GAME UP rolls, under the Rust site's own name.
const W64 = (1n << 64n) - 1n, BITS = new DataView(new ArrayBuffer(8));
function stir(x: bigint): bigint { x ^= x >> 30n; x = x * 0xbf58476d1ce4e5b9n & W64; x ^= x >> 27n; x = x * 0x94d049bb133111ebn & W64; return x ^ (x >> 31n); }
/** who asks, about whom, as one word */
export function drawKey(site: string, who: number): bigint {
  let h = 0xcbf29ce484222325n; for (let i = 0; i < site.length; i++) { h ^= BigInt(site.charCodeAt(i)); h = h * 0x100000001b3n & W64; }
  return stir(h ^ stir(BigInt(who) ^ 0x9e3779b97f4a7c15n));
}
export function keyed(seed: number, key: bigint, time: number, nth: number): number {
  BITS.setFloat64(0, time);
  const x = stir(stir(stir(BigInt(seed >>> 0) ^ 0x2545f4914f6cdd1dn) ^ key) ^ stir(BITS.getBigUint64(0)) ^ ((BigInt(nth) << 1n | 1n) * 0xd6e8feb86659fd93n & W64));
  return Number(stir(x) >> 11n) / 9007199254740992;
}
/** a keyed draw for the simulation. The game's die is still rolled once a draw (its count says how many draws were made); with none in force the roll is the draw. */
export function simDraw(site: string, who = 0, time = 0): number {
  const d = diceInUse(), old = simRandom(); if (!d) return old;
  if (d.at !== time) { d.at = time; d.drawn.clear(); }
  const key = drawKey(site, who), nth = d.drawn.get(key) ?? 0; d.drawn.set(key, nth + 1);
  return keyed(d.seed, key, time, nth);
}
