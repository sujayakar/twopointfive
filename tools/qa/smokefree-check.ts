// THE SMOKE'S FREE LIST HANDS A BRICK OUT ONCE (src/smoke/smoke.wgsl ensureBrick): many requests at the last slots, with two bars (the view's and the haze's), taken in every order a seed deals, an atomic at
// a time, by a model of the pop. Afterwards every slot is on the list or in use, and never both. No GPU: the shader's own text is held to the model's by what it calls.
//   bun run tools/qa/smokefree-check.ts              exit 0 = passed
//   SMOKEFREE_BREAK=1 …                              the pop as it was (take one, look, put it back): must FAIL
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const BREAK = process.env.SMOKEFREE_BREAK === '1', SEEDS = 20, CAP = 64, WGSL = readFileSync(resolve(import.meta.dir, '../../src/smoke/smoke.wgsl'), 'utf8');
let red = 0; const check = (ok: boolean, what: string): void => { if (!ok) red++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); };
type Pool = { free: number; list: number[]; used: Map<number, number[]> };
const take = (P: Pool, id: number, n: number): void => { const slot = P.list[n - 1]; const got = P.used.get(slot); if (got) got.push(id); else P.used.set(slot, [id]); };
/** the pop as it is: a compare-and-swap, tried against the others' */
function* exchange(P: Pool, id: number, bar: number): Generator<void> {
  let n = P.free; yield;
  for (let go = 0; go < 256 && n > bar && n <= CAP; go++) { if (P.free === n) { P.free = n - 1; yield; take(P, id, n); return; } n = P.free; yield; }
}
/** …and as it was: take one, look at what was there, put it back where the bar is not met */
function* subtract(P: Pool, id: number, bar: number): Generator<void> {
  const n = P.free--; yield;
  if (n <= bar || n > CAP) { P.free++; yield; return; }
  take(P, id, n);
}
const deal = (seed: number): { wrong: number } => {
  let r = seed * 2654435761 + 1 >>> 0; const rnd = (): number => { r = Math.imul(r ^ r >>> 15, r | 1); r ^= r + Math.imul(r ^ r >>> 7, r | 61); return ((r ^ r >>> 14) >>> 0) / 4294967296; };
  const P: Pool = { free: 12, list: Array.from({ length: CAP }, (_, i) => i), used: new Map() };
  for (let i = 12; i < CAP; i++) P.used.set(i, [-1]);   // (the rest are out already)
  const threads = Array.from({ length: 200 }, (_, k) => (BREAK ? subtract : exchange)(P, k, rnd() < 0.5 ? 0 : 8));
  const live = [...threads]; while (live.length) { const i = Math.floor(rnd() * live.length); if (live[i].next().done) live.splice(i, 1); }
  // what stands: a slot handed to two places, one in use and on the list, one that is neither
  let wrong = 0; const free = new Set(P.list.slice(0, Math.max(0, P.free))); for (const who of P.used.values()) if (who.length > 1) wrong++;
  for (const s of P.used.keys()) if (free.has(s)) wrong++;
  for (let s = 0; s < CAP; s++) if (!P.used.has(s) && !free.has(s)) wrong++;
  return { wrong };
};
const all = Array.from({ length: SEEDS }, (_, i) => deal(i + 1)), bad = all.filter(x => x.wrong).length;
check(bad === 0, `${SEEDS} seeds of 200 requests with two bars at the last 12 slots of ${CAP}: ${bad} leave a brick in two places or off the list`);
check(/atomicCompareExchangeWeak\(&A\[C_FREE\]/.test(WGSL) && !/atomicSub\(&A\[C_FREE\]/.test(WGSL) && /for \(var go = 0u; go < POP_TRIES && n > keepFree && n <= CAP; go\+\+\)/.test(WGSL), 'the shader\'s pop is the exchange the model takes, and nothing takes one off the count and puts it back');
console.log(`=== smokefree: ${red ? `${red} FAILED` : 'ALL PASS'} ===`);
process.exit(red ? 1 : 0);
