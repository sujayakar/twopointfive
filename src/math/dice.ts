// Dice. mulberry32 is the seeded die. The systems that are not the game — the mixer's rate jitter and noise beds (audio/), the smoke solver's per-frame emitter
// jitter (smoke/smoke.ts) — draw through sysRandom().
/** mulberry32: a small, fast, seedable uniform [0, 1) generator */
export function mulberry32(a: number): () => number { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** a die whose whole state is the word `dice`, on an object: `roll` on it gives the sequence `mulberry32` gives, draw for draw */
export interface Die { dice: number }
export function roll(d: Die): number { const a = d.dice = (d.dice + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }
/** a small stable string hash (FNV-1a, 32 bit): the same string gives the same number on every machine — what things DEALT by name, never rolled, are dealt off */
export function hashStr(s: string): number { let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; } return h >>> 0; }
/** a die standing in for Math.random, remembering the one it displaced */
export type SimDie = (() => number) & { sys?: () => number };
/** a draw for something that is not the game: the displaced die, if one stands in, else plain Math.random */
export function sysRandom(): number { const r = Math.random as SimDie; return (r.sys ?? r)(); }

// THE KEYED DRAW (rust/game_types/src/dice.rs `keyed`, the same function to the bit: that file's own test holds it to one number).
// A fixed function of the game's seed, the SITE that asks, the man it is about (0: nobody, else his body's id + 1), the clock, and how many times this site has drawn
// for him at this clock: no draw moves another. The game rolls in Rust; this side rolls what STANDING A GAME UP rolls, under the Rust site's own name.
const W64 = (1n << 64n) - 1n, BITS = new DataView(new ArrayBuffer(8));
function stir(x: bigint): bigint { x ^= x >> 30n; x = x * 0xbf58476d1ce4e5b9n & W64; x ^= x >> 27n; x = x * 0x94d049bb133111ebn & W64; return x ^ (x >> 31n); }
export function keyed(seed: number, key: bigint, time: number, nth: number): number {
  BITS.setFloat64(0, time);
  const x = stir(stir(stir(BigInt(seed >>> 0) ^ 0x2545f4914f6cdd1dn) ^ key) ^ stir(BITS.getBigUint64(0)) ^ ((BigInt(nth) << 1n | 1n) * 0xd6e8feb86659fd93n & W64));
  return Number(stir(x) >> 11n) / 9007199254740992;
}
