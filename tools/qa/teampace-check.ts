// TEAMMATES ARE SLOWER THAN THE PLAYER, AND RUN ONLY FOR A REASON — the check behind src/game/teampace.ts (TEAMPACE.on: on in the page, ?teampace=0; off on the
// frozen stand, TPS_TEAMPACE=1).
//   bun run tools/qa/teampace-check.ts [--verbose]      (also `probes.ts teampace`, family squadorders)
// Headless; nothing here looks at a picture. room-clear-four with the other side put far off the floor; the player is the real player under real keys. One stand:
//   1. AN ORDERED WALK: the four sent to a spot a long walk off: no man's legs over the player's walk × his card; the player, setting off five seconds after them down
//      the same way, comes up on their last man;
//   2. FOLLOWING AT A SPRINT: fallen in there, he sprints back: every follower is at a run with him, gun low while he runs, down to a walk inside a second and a half of
//      his stopping, never into the player's back, in place soon after; each run written down as 'follow'.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { TEAMPACE, TEAMPACE_STATS } = await import(`${ROOT}/src/game/teampace.ts`);
const { loadPaceOn } = await import(`${ROOT}/src/game/kitload.ts`);
const VERBOSE = process.argv.includes('--verbose'), SCN = 'room-clear-four';
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
/** the scenario stood up with the other side far off the floor, the four fallen in on the player where the scenario put him */
async function stand() {
  const R = await rig(SCN, 5), g = R.game;
  const marks = R.hostiles.filter((h: any) => h.state !== 'dead').map((h: any) => [...h.char.pos]);
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const home = [...g.player.char.pos]; R.gather(home, 20); R.step(10);
  // a long walk off: the spot one of their men stood on, by the length of the way to it
  // … that 'move there' takes (no locked door on the way): asked, then the four called back
  const cands = marks.map((m: number[]) => ({ m, L: R.pathLen(home, m) as number })).filter((c: any) => Number.isFinite(c.L) && c.L >= 14).sort((a: any, b: any) => Math.abs(a.L - 20) - Math.abs(b.L - 20));
  let far: number[] | null = null, len = 0;
  for (const c of cands) { if (R.O.give('move', R.O.targetAt(c.m), 'all')) continue; far = c.m; len = c.L; break; }
  if (!far) throw new Error('teampace: no spot a long walk from the start that the squad can be sent to');
  R.gather(home, 20); R.step(10);
  const way = (a: number[], b: number[]): number[][] => [...(g.col.nav.findPath(a, b) ?? []), b];
  /** the player walked (or run, `shift`) down the way to `to` under his own keys; `each` is called every frame; ends there, or after `secs` */
  const go = (to: number[], shift: boolean, secs: number, each: () => void) => { const I = R.input, pts = way([...g.player.char.pos], to); let i = 0;
    for (let f = 0; f < secs * 60 && i < pts.length; f++) { const pos = g.player.char.pos; while (i < pts.length && flat(pos, pts[i]) < (i === pts.length - 1 ? 0.4 : 0.7)) i++; if (i >= pts.length) break;
      for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k); const p = pts[i], dx = p[0] - pos[0], dz = p[2] - pos[2], d = Math.hypot(dx, dz) || 1, { fwd, right } = R.W.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / d, b = (dx * right[0] + dz * right[2]) / d, T = 0.3827;
      if (a > T) I.keys.add('KeyW'); if (a < -T) I.keys.add('KeyS'); if (b > T) I.keys.add('KeyD'); if (b < -T) I.keys.add('KeyA'); if (shift) I.keys.add('ShiftLeft'); else I.keys.delete('ShiftLeft');
      R.step(); each(); }
    for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft']) I.keys.delete(k); R.step(); each(); };
  const card = (m: any) => loadPaceOn(g, m) as number;
  return { R, g, home, far, len, go, card, men: () => R.up as any[] };
}
type Stand = Awaited<ReturnType<typeof stand>>;

export const teampaceCheck = guarded(async () => {
  const F = family('teampace: teammates are slower than the player, and run only for a reason');
  const keep = { rnd: Math.random };
  try {
    const S = await stand(), { R, g } = S, men = S.men(), sam = g.player.char, walk = g.tune.playerWalk as number;
    // ---- 1: the ordered walk and the race
    { const why = R.O.give('move', R.O.targetAt(S.far), 'all'); if (why) throw new Error(`teampace: 'move there' refused — ${why}`);
      let top = 0, over = 0; const cap = (m: any) => walk * S.card(m) + 0.03;
      const watch = () => { for (const m of men) { top = Math.max(top, m.speed); if (m.speed > cap(m)) over++; } };
      for (let f = 0; f < 300; f++) { R.step(); watch(); }   // their five seconds' start
      // (the REAR man: the file fills the passage, so the player catches a team by coming up behind its last man)
      const rear = () => Math.max(...men.map(m => R.pathLen([...m.char.pos], S.far) as number)), mine = () => R.pathLen([...g.player.char.pos], S.far) as number;
      const gap0 = mine() - rear(), t0 = g.time; let gap1 = gap0, t1 = t0, caught = false, arrived = false;
      S.go(S.far, false, 60, () => { watch(); if (caught || arrived) return; gap1 = mine() - rear(); t1 = g.time; caught = gap1 <= 1.0; arrived = men.some(m => R.O.of(m)?.arrived); });
      const rate = (gap0 - gap1) / Math.max(1e-6, t1 - t0);
      F.verdict(top > 1.0 && over === 0, "an ordered walk: no man's legs over the player's own walk × his card", `top ${f2(top)} m/s against a walk of ${f2(walk)} (cards ×${men.map(S.card).map(f2).join(' ×')}), frames over ${over} · the way ${f2(S.len)} m`);
      F.verdict(rate > 0.12 && gap1 < gap0, 'the player, setting off five seconds behind an ordered team, comes up on its last man at a walk', `${f2(gap0)} m behind its last man → ${f2(gap1)} m in ${f2(t1 - t0)} s, ${f2(rate)} m/s gained`); }
    // ---- 2: fallen in, a sprint back the way he came
    { const dSam = () => Math.max(...men.map(m => flat(m.char.pos, sam.pos)));
      R.gather([...sam.pos], 20); R.step(10);
      const w0 = TEAMPACE_STATS.follow as number, tops = men.map(() => 0); let runF = 0, lowF = 0, far = 0, samTop = 0;
      S.go(S.home, true, 30, () => { samTop = Math.max(samTop, Math.hypot(sam.vel[0], sam.vel[2])); far = Math.max(far, dSam()); men.forEach((m, i) => { tops[i] = Math.max(tops[i], m.speed); if (m.speed >= TEAMPACE.runAt) { runF++; if (m.char.anim.upper === 'none') lowF++; } }); });
      let down = -1, near = 9; for (let f = 0; f < 360; f++) { R.step(); near = Math.min(near, ...men.map(m => flat(m.char.pos, sam.pos))); if (down < 0 && men.every(m => m.speed < TEAMPACE.runAt)) down = f / 60; }
      const inPlace = R.until(() => men.every(m => R.O.of(m)?.arrived), 12), runs = (TEAMPACE_STATS.follow as number) - w0;
      F.verdict(tops.every((t: number) => t >= 4), 'following at a sprint: every follower runs with him', `tops ${tops.map(f2).join(' ')} m/s, the player ${f2(samTop)}, the last man at most ${f2(far)} m off`);
      F.verdict(runF > 0 && lowF === runF && down >= 0 && down <= 1.5 && near > 0.4 && inPlace && runs >= men.length,
        'at a run the gun is low; down to a walk inside a second and a half of his stopping, never into his back, in place after; each run written down',
        `${lowF} of ${runF} running frames gun low · walking ${f2(down)} s after he stops · nearest ${f2(near)} m · in place ${inPlace} · runs written: follow ${runs}`); }
  } finally { Math.random = keep.rnd; }
  return F.done();
});

if (import.meta.main) process.exit((await teampaceCheck()) ? 0 : 1);
