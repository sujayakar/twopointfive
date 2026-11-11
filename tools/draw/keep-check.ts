// A RUN IS KEPT AS IT IS PLAYED (src/tape/keep.ts over tape/keeper.ts and keepstore.ts), with no page and no module: a tape of hands grows, goes to the store in beats and comes back out of it
// byte for byte; killed between two beats it comes back up to the last; a piece lost ends it there; a visit removed under it is begun again whole; a hand-over file's lines give the same.
//   bun run tools/draw/keep-check.ts      exit 0 · 1
//   KEEP_BREAK=1 …      a piece says it is from a byte further on: what comes back must be cut short
import { HAND } from '../../src/draw/hand';
import { ReplayKeeper } from '../../src/tape/keeper';
import { bodyText, memoryStore } from '../../src/tape/keepstore';
import { parseLines } from '../../src/tape/kept';
import { HandTape, HandWalk } from '../../src/tape/handtape';
import { handSource, handsOf, keptHands } from '../../src/tape/keep';

const BREAK = process.env.KEEP_BREAK === '1'; let fails = 0;
const line = (ok: boolean, what: string, why = ''): void => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${why ? `\n    ${why}` : ''}`); };
const same = (a: Uint8Array | null, b: Uint8Array): boolean => !!a && a.length === b.length && a.every((v, i) => v === b[i]);
let n = 1; const roll = (): number => { n = Math.imul(n ^ n >>> 15, n | 1); n ^= n + Math.imul(n ^ n >>> 7, n | 61); return ((n ^ n >>> 14) >>> 0) / 4294967296; };
const hand = new Float64Array(HAND.count);
const begun = (): HandTape => { const t = new HandTape(HAND.count); t.start({ build: 'b', levelName: 'a level', level: new Uint8Array(5000).map(() => roll() * 256), words: '?seed=1', parts: [], seed: 1, seats: 1, flags: 1, kept: '' }); return t; };
const played = (t: HandTape, ticks: number): void => { for (let i = 0; i < ticks; i++) { for (let k = 0; k < HAND.count; k++) if (roll() < 0.7) hand[k] = roll() < 0.5 ? Math.fround(roll()) : roll(); t.step(1 / 60, hand, roll() < 0.05 ? 'KeyW\n' : '\n'); } };
const keeper = (t: HandTape, store: ReturnType<typeof memoryStore>, sid: string): ReplayKeeper => { const src = handSource(() => t);
  return new ReplayKeeper(BREAK ? { again: src.again, since: () => { const p = src.since(); return p && { ...p, line: (s, q, a) => p.line(s, q, a).replace(/"from":([1-9]\d*)/, (_m, f) => `"from":${Number(f) + 1}`) }; } } : src, { sid, store, played: () => t.ticks > 0, budget: async () => 1e9, head: () => ({ level: { file: 'office.json', scenario: 'a mission' }, hands: 1, url: '?x' }) }); };

const store = memoryStore(), sid = '20261108T120000-abc', T = begun(), K = keeper(T, store, sid);
K.beat(); await K.idle(); line((await store.rows()).length === 0, 'nothing is kept of a run nobody has played');
for (let i = 0; i < 6; i++) { played(T, 300); K.beat(); await K.idle(); }
const upTo = T.whole(); played(T, 120);
line(same(await keptHands(store, sid), upTo), 'a page killed between two beats has kept the run up to the last one', `${T.ticks} ticks played, ${T.ticks - 120} kept`);
T.again(); T.end('lost'); await K.leave();
const back = await keptHands(store, sid), row = (await store.rows())[0];
line(same(back, T.whole()), 'a run left is kept whole, byte for byte', `${(T.length / 1024).toFixed(0)} KB in ${row.pieces} pieces, ${(row.bytes / 1024).toFixed(0)} KB kept`);
line(row.hands === true && row.seconds === 32 && row.scenario === 'a mission' && row.left === true, 'its row says how long it is, what of, that it is a tape of hands and that its page has gone', JSON.stringify(row));
{ const W = new HandWalk(back ?? T.whole(), HAND.count); let ticks = 0, tries = 0, end = ''; for (let r = W.next(); r; r = W.next()) { if (r.kind === 'step') ticks++; else if (r.kind === 'try') tries++; else if (r.kind === 'end') end = r.why; } line(ticks === T.ticks && tries === 1 && end === 'lost' && T.began === T.ticks, 'it reads to its end, with its try and why it ended', `${ticks} ticks, ${tries} try, '${end}'`); }
{ const bodies = await store.bodies(sid), text = (await Promise.all(bodies.map(bodyText))).join(''), all = parseLines((await store.head(sid))! + text); line(all.bad === 0 && same(handsOf(all.lines), T.whole()), 'a hand-over file’s lines give the same tape');
  const lost = parseLines((await Promise.all(bodies.filter((_b, i) => i !== 3).map(bodyText))).join('')).lines, cut = handsOf(lost), W = new HandWalk(cut!, HAND.count); let ticks = 0; for (let r = W.next(); r; r = W.next()) if (r.kind === 'step') ticks++; line(ticks === 900, 'a piece that was lost ends it there', `${ticks} ticks`); }
{ const s2 = memoryStore(), t = begun(), k = keeper(t, s2, sid); played(t, 200); k.beat(); await k.idle(); await k.leave(); await s2.drop(sid); k.back(); await k.idle(); played(t, 100); k.beat(); await k.idle(); line(same(await keptHands(s2, sid), t.whole()), 'a visit removed while its page was away is kept again from the run’s first tick'); }
{ // a browser that refuses a piece, both times it is asked: the next takes it up from there
  const s3 = memoryStore(), put = s3.put; let no = 0; s3.put = async (r, q, b, h) => no-- > 0 ? false : put(r, q, b, h); const t = begun(), k = keeper(t, s3, sid);
  for (let i = 0; i < 5; i++) { if (i === 2) no = 2; played(t, 100); k.beat(); await k.idle(); } no = 0;
  line(k.stats.dropped === 1 && !k.broke && same(await keptHands(s3, sid), t.whole()), 'a piece the browser refused is kept with the next', JSON.stringify(k.stats)); }
{ const s4 = memoryStore(0), t = begun(), k = keeper(t, s4, sid); played(t, 100); await k.flush(); line(k.stats.pieces === 0 && k.stats.dropped === 1, 'a browser that keeps nothing is known by it at the first piece', JSON.stringify(k.stats)); }
{ const a = begun(), b = begun(); played(a, 50); played(b, 80); const l = (t: HandTape, s: string): string => handSource(() => t).since()!.line(s, 0, ''); line(same(handsOf(parseLines(l(a, 'one') + l(b, 'two')).lines), a.whole()), 'two visits’ lines are not laid into one tape'); }
line((await keptHands(store, 'nobody')) === null, 'a visit that is not kept has no tape');
console.log(fails ? `=== keep: ${fails} FAILED ===` : '=== keep: ALL PASS ===');
process.exit(fails ? 1 : 0);
