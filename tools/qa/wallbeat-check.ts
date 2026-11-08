// A WALL HOLD COUNTS AS SNEAKING ONCE HIS BACK HAS BEEN ON IT A BEAT (UNSEEN.wallBeat, settled on; not written here since 2026-11-07, nor is the UNSEEN table
// laid again or HOLDFIRE.on put off. Out with the rule's off side: (1) the stand's word, (2) 'rule off, he stays', (4) 'rule off, he brushes it … the
// fault', the 'rule off it always was' clause of (6), (7) 'off, nothing reads its number', (8) the recording and the page's flag).
// Found by the squad-sneak probes on the page's stand: an upright walk that ran into a wall took the wall hold (a move key held into a wall takes it with no key of its own, wallsnap.ts
// `press`), a wall hold was sneaking from its first step, and the squad dropped into a sneak for the brush and `linger` after it.
// Staged: the office as a room clear with four under 'fall in', the hold-without-a-key raised by hand. Sam is walked upright (the sneak probes' own walker: the
// move keys toward a goal) into the first wall a straight walk finds, and either stays on it or is turned about a quarter second after his back lands.
//   (3) he stays: not until his back has been on it `wallSecs`, and from then;
//   (5) he brushes it: the squad never sneaks;
//   (6) the test itself: low on a wall is sneaking at once; a hold that keeps no clock counts at once; the wheel down counts whatever the wall says;
//
//   bun run tools/qa/wallbeat-check.ts            (also `probes.ts wall-beat`, family unseen)
//   WALLBEAT_BREAK=1 …                            (the rule's own body left out, the same lines asked: must fail — the game's own hook, in unseen.ts)
// What it cannot say: one wall, one walk, keys held by code; whether 0.8 s is the right beat for a hand on the keys is a thing to play. Never seen on a GPU, nobody has played it.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
import { Run } from './pairsneak.ts';
const U: any = await import(`${ROOT}/src/game/unseen.ts`), { UNSEEN } = U;
const { WALLSNAP } = (await import(`${ROOT}/src/game/wallsnap.ts`)) as any;
const { knobsCopy, knobsBack } = (await import(`${ROOT}/src/game/knobs.ts`)) as any;

const DT = 1 / 60, QUERY = 'roomclear=3&squad4', WAYS = [[1, 0], [0, 1], [-1, 0], [0, -1]];
async function quiet<T>(fn: () => T | Promise<T>): Promise<T> { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } }

interface Play { why: string; pre: number; first: number; firstHit: number; frames: number; held: number; maxHit: number; series: string; g: any }
/** the office, the four fallen in, Sam walked upright into a wall: `stay` on it 2.5 s, or pushed off a quarter second after his back lands and watched 2.5 s more */
async function play(how: 'stay' | 'brush', o: { secs?: number } = {}): Promise<Play> {
  const P: Play = { why: '', pre: 0, first: NaN, firstHit: NaN, frames: 0, held: 0, maxHit: 0, series: '', g: null };
  const R: any = await quiet(() => Run.start(QUERY, 1, { hard: true, lean: true })); R.stopOnNotice = false; R.cap = 600; const g = R.g, pl = R.pl, c = pl.char; P.g = g;
  if (o.secs !== undefined) UNSEEN.wallSecs = o.secs;
  WALLSNAP.on = true; WALLSNAP.press = true;
  const why = g.orders.give('fallIn', { kind: 'none', at: [R.pos[0], 0, R.pos[2]] }); if (why) { P.why = `fall in refused: ${why}`; return P; }
  const sneaking = () => U.unseenWord(g) === 'sneaking';
  try {
    await quiet(() => R.step(60));
    if (pl.crouch || pl.wall || sneaking()) { P.why = `not an upright walk to begin with (crouch ${pl.crouch}, wall ${!!pl.wall}, the squad ${U.unseenWord(g) ?? 'loud'})`; return P; }
    let dir: number[] | null = null;
    await quiet(() => { for (const d of WAYS) { if (pl.wall) break; R.goal = [c.pos[0] + d[0] * 12, 0, c.pos[2] + d[1] * 12]; for (let i = 0; i < 60 * 6 && !pl.wall; i++) { R.step(); if (sneaking() && !pl.wall) P.pre++; } if (pl.wall) dir = d; } });
    if (!pl.wall || !dir) { P.why = 'no way he walked took him onto a wall'; return P; }
    const t0 = g.time; let off = false; const bits: string[] = [], away: number[] = dir;
    await quiet(() => {
      for (let i = 0; i < Math.round(2.5 / DT) + (how === 'brush' ? 60 : 0); i++) {
        const hit = pl.wall?.hitT ?? 0; if (pl.wall) { P.held++; P.maxHit = Math.max(P.maxHit, hit); }
        const s = sneaking(); bits.push(s ? '1' : '0'); if (s) { P.frames++; if (isNaN(P.first)) { P.first = g.time - t0; P.firstHit = hit; } }
        if (how === 'brush' && !off && pl.wall && hit >= 0.25) { off = true; R.goal = [c.pos[0] - away[0] * 12, 0, c.pos[2] - away[1] * 12]; }
        R.step();
      }
    });
    P.series = bits.join('');
  } catch (e) { P.why = `the run stopped: ${(e as Error).message}`; }
  return P;
}
const f2 = (n: number) => Number.isNaN(n) ? '—' : n.toFixed(2);
const show = (R: Play) => R.why || `on the wall ${R.held} steps, his back on it ${f2(R.maxHit)} s at most · the squad sneaking ${R.frames} steps${R.frames ? `, first ${f2(R.first)} s after the hold began with his back ${f2(R.firstHit)} s on it` : ''} · ${R.pre} before the wall`;

export const wallBeatCheck = guarded(async () => {
  const F = family('wall-beat: a wall hold counts as sneaking once his back has been on it a beat');
  const knobs0 = knobsCopy(), rnd0 = Math.random;
  try {
    const stay1 = await play('stay'), brush1 = await play('brush'), S = UNSEEN.wallSecs;
    F.verdict(!stay1.why && stay1.pre === 0 && stay1.firstHit >= S - 1e-6 && stay1.firstHit <= S + 0.05 && stay1.maxHit > S + 0.5, `he stays on the wall: the squad is not sneaking until his back has been on it ${S} s, and is from then`, show(stay1));
    F.verdict(!brush1.why && brush1.pre === 0 && brush1.frames === 0 && brush1.maxHit < S && brush1.maxHit >= 0.25, 'he brushes it and pushes off: the squad never sneaks', show(brush1));
    { const man = (o: object) => ({ down: false, sprinting: false, dash: false, crouch: false, pace: 1, wall: null, ...o });
      const low = U.samSneaks(man({ crouch: true, wall: { hitT: 0 } })), bare = U.samSneaks(man({ wall: {} })), slow = U.samSneaks(man({ pace: 0.35, wall: { hitT: 0 } })), fresh = U.samSneaks(man({ wall: { hitT: 0.1 } })), sat = U.samSneaks(man({ wall: { hitT: S } }));
      F.verdict(low && bare && slow && !fresh && sat, 'the test itself: low on a wall is sneaking at once, so is a hold that keeps no clock, so is the wheel down; upright with his back just landed is not, with the beat spent it is',
        `low ${low} · no clock ${bare} · wheel down ${slow} · back just landed ${fresh} · beat spent ${sat}`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }
  return F.done();
});

if (import.meta.main) process.exit((await wallBeatCheck()) ? 0 : 1);
