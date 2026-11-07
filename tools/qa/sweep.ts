// A SWEEP IS ASKED IN A SAMPLE. A check that drives every static, every door or every seed takes minutes, so nobody runs it and a red in it goes unread. Its
// cases are handed through `sample`, which takes `n` of them, spread evenly from the first to the last (the same ones on every run), unless the run says
// QA_SWEEP=all, which takes every one. The floors a sweep keeps on how many cases it ran (ranAtLeast) are said per mode with `swept`.
//   for (const w of sample(walls, 12)) …
//   ranAtLeast('walls driven into', runs, swept(120, 1500))
const ALL = ((v: string | undefined): boolean => { if (v === undefined || v === '') return false; if (v === 'all') return true; throw new Error(`QA_SWEEP=${v}: want all (every case of every sweep) or nothing (a sample)`); })(process.env.QA_SWEEP);
/** `n` of the cases, spread evenly, the first and the last among them; all of them where there are no more than `n`, or under QA_SWEEP=all */
export function sample<A extends readonly any[]>(cases: A, n: number): A[number][] {
  if (ALL || cases.length <= n) return [...cases];
  if (n === 1) return [cases[0]];
  return Array.from({ length: n }, (_, i) => cases[Math.round(i * (cases.length - 1) / (n - 1))]); }
/** a count that is one thing in a sampled run and another in a whole one */
export const swept = (sampled: number, whole: number): number => ALL ? whole : sampled;
/** for loops inside loops: `const turn = everyNth(37); … if (!turn()) continue;` asks every 37th case met (take an `n` that shares no factor with the inner
 *  loops' lengths, so that every axis is walked), all of them under QA_SWEEP=all */
export function everyNth(n: number): () => boolean { let k = 0; return () => ALL || k++ % n === 0; }
