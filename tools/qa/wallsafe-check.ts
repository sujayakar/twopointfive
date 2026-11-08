// THE WALL CHARGE'S SAFETY RADIUS — the check behind WALLCHARGE.safe (src/game/wallcharge.ts safeRadius; a settled rule: every tool stands the game with it on).
//   bun run tools/qa/wallsafe-check.ts [--scn room-clear-four,block-four] [--verbose]        (also `probes.ts wallsafe`)
// On the office and on a level with outer walls, a squad of four, the destruction switch raised as the wall charge's own probes raise it:
//   (1) the level's own geometry says which side of a wall is indoors: an outer wall worked from outside is not, a partition (or an outer wall from inside) is;
//       the radius is larger than it was outdoors and larger again indoors;
//   (2) the stand-off: every mark is out of the blast — at the radius or more from the charge (4 m outdoors, 6 m indoors), or with something solid between it and
//       the charge — and nearer marks than the old 2.6 m are never dealt; a place whose floor has no such marks falls back to the old ones;
//   (3) the blast, staged: on the charge's side a man inside the reach with a line to it is flash-blind, longer the nearer he stood; one just past it is not; a
//       man 4 m off is caught indoors and clear outdoors; Sam past the reach feels nothing, Sam inside it is rocked (no round for a beat, his aim open, still on
//       his feet and unhurt); the trigger's words name who is inside before it goes;
//   (4) a played order indoors: the squad sets it, stands off out of the blast, F — nobody of theirs is caught; and in a place with no room the man who calls it
//       says so once, the note warns, and they are caught (the same place with the radius asked for more floor than the level has);
//   (6) Mousehole, a charge set from inside the ward (WALLCHARGE.safeBack): the marks by the walk back to the charge, and the men off their marks on the blast.
//   (8) the far side of the warning: a teammate beyond the wall where it will open is named and caught, one past the stun's reach is neither, Sam there is named
//       and blinded (the probe `wallkeep`'s second half until 2026-11-07; its first half, a paired run of the kept walks with the keep held off, went: it counted
//       calls through wraps on the tools' copy of the game and held the keep off by a handle on it, neither of which reaches the Rust game).
//   (7) a mark is only ever on floor the squad holds: never in a room nobody of theirs has stood in, defenders left where the level put them.
// 2026-11-07: the radius, the walk back to the hole (safeBack), the player's stun rule and the charge's kill at its foot are settled rules, and a write of
// the other side of any of them is refused. Gone with that: (5) 'the switch off: the build as it was'; in (6) the marks 'by the straight line' and the play 'as
// it was' that the walk-back marks were laid beside (each line keeps its own numbers: the cap, the blast's share, the phase); the stun rule held off for Sam's
// first blast and the kill rule held off for the whole check (both writes were refused, so the lines have run on the page's side since the fold: who the
// charge kills among the men (3) stages at it is the kill rule's, and a line of (3) that cannot hold beside it is to be deleted, not mended).
// 2026-11-07 (later): the three blasts of (3) are on a charge Sam's own hands set (tools/qa/charge-rig.ts), each set before the men are stood around it; until
// then its row was pushed onto the tools' copy of the game, which the Rust game never got. What was said is read from the game's own state (heard.ts).
// WALLSAFE_BREAK=1 = the charges of (3) and (8) are set and never set off: their blast lines must fail.   exit 0 = all hold · 1 = one does not
import { behindWall, notesOf } from './squadwords';
import { resolve } from 'node:path';
import { samSets } from './charge-rig.ts';
import { sample } from './sweep.ts';
const ROOT = resolve(import.meta.dir, '../..');
type Vec3 = [number, number, number];

/** the two levels asked when none is named: each has a room to play the charge in from a fresh start */
const FRESH = sample(['room-clear-four', 'block-four'], 1);
export async function wallsafeCheck(scns = FRESH): Promise<boolean> {
  const { rig, outerSites, partition } = (await import('./outerwall-probes.ts')) as any;
  const { WALLCHARGE, WALL_ORDER, wallTarget, chargeIndoors, safeRadius, wallPt, wallOff } = await import(`${ROOT}/src/game/wallcharge.ts`);
  const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), f2 = (x: number) => x.toFixed(2), at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
  const was = { rnd: Math.random, sw: DESTRUCTION.on, wc: JSON.parse(JSON.stringify(WALLCHARGE)) };
  console.log('\n=== wallsafe: the wall charge\'s safety radius, larger and larger again indoors ===');
  try {
    for (const scn of scns) {
      console.log(`  — ${scn}`);
      const R = await rig(scn, 5), g = R.game, C = R.C, col = g.col; let pl = g.player; R.kit();   // (`pl` is taken again after every restart: a restart makes the player's man anew, and the man held from before it is no longer the game's)
      const src = (W: any): Vec3 => wallPt(W, 0, 0.5, 1.1), lined = (W: any, q: number[]) => !col.solidBlocked(src(W), [q[0], 1.1, q[2]]);
      /** a free place on the charge's side `d` metres from it with a line to it (along the wall either way, straight back, or between), not within 0.8 m of `not` */
      const place = (W: any, d: number, not: number[][] = []): Vec3 | null => { const k = Math.SQRT1_2 * d;
        for (const q of [wallPt(W, d, 0.55), wallPt(W, -d, 0.55), wallPt(W, 0, d), wallPt(W, k, k), wallPt(W, -k, k)] as Vec3[]) { const e = flat(q, wallPt(W, 0, 0)), r: Vec3 = [q[0] * d / e + wallPt(W, 0, 0)[0] * (1 - d / e), 0, q[2] * d / e + wallPt(W, 0, 0)[2] * (1 - d / e)];
          if (R.free(r) && wallOff(W, r) > 0.3 && lined(W, r) && !not.some(n => flat(n, r) < 0.8)) return r; }
        return null; };
      const outs = outerSites(R, 'outside', 2).map((s: any) => s.W), P0 = partition(R), ins = [...(P0 ? [P0.W] : []), ...outerSites(R, 'inside', 2).map((s: any) => s.W)];
      // (room for the whole staging of (3), as (3) deals it — each place clear of the ones before it: three places that each stand alone are not four that stand together)
      const stage = (W: any): (Vec3 | null)[] => { const r = safeRadius(g, W), pA = place(W, 0.6 * r.reach), pB = pA && place(W, r.reach + 0.5, [pA]), pM = pB && place(W, 4.0, [pA!, pB]), pS = pM && place(W, r.reach + 0.6, [pA!, pB!, pM]); return [pA, pB, pM, pS]; };
      const roomy = (W: any) => stage(W).every(p => !!p);
      const Wout = outs.find(roomy) ?? outs[0], Win = ins.find(roomy) ?? ins[0];
      if (!Wout || !Win) { line(false, `${scn}: an outer wall to work from outside and a wall to work from indoors`, `outside ${outs.length} · indoors ${ins.length}`); continue; }
      // (1) indoors or not, and the radii
      const ro = safeRadius(g, Wout), ri = safeRadius(g, Win);
      line(!chargeIndoors(g, Wout) && chargeIndoors(g, Win) && ro.standoff > WALLCHARGE.standoff[0] && ro.reach > DOORTOOLS.backblastM && ri.standoff > ro.standoff && ri.reach > ro.reach && ro.standoff > ro.reach && ri.standoff > ri.reach,
        'the level\'s own geometry tells the two apart, and the radius is larger than it was outdoors and larger again indoors',
        `outer wall ${Wout.index} from outside: indoors ${chargeIndoors(g, Wout)}, stand-off ${ro.standoff} m, reach ${ro.reach} m · wall ${Win.index}${Win.outer ? ' (outer, from inside)' : ' (partition)'}: indoors ${chargeIndoors(g, Win)}, stand-off ${ri.standoff} m, reach ${ri.reach} m · it was ${WALLCHARGE.standoff[0]} m and ${DOORTOOLS.backblastM} m`);
      // (3) the blast, staged on each of the two walls: three men by distance, Sam outside the reach and then inside it
      // (EACH BLAST IS ON A CHARGE SAM'S OWN HANDS SET, on a whole wall: until 2026-11-07 its row was pushed onto the tools' copy of the game by hand, three times on
      //  the one wall, a write the Rust game never got. The floor is put back as authored before each (restartEncounter), everybody is moved off, Sam sets it,
      //  and only then are the men and he stood where the line wants them; it goes off through the module's own trigger. The wall is the first with room to
      //  stage the line that takes his row: what is measured from is where the charge settled. THE TWO BLASTS WITH SAM INSIDE THE REACH ARE THE SQUAD'S CHARGE,
      //  set on the wall's own order and set off on the squad's key: his own charge is never set off with him in its blast (ownblow.ts), and read on the Rust
      //  game its blast, forced by the trigger with him inside, does nothing to him.)
      const far = (gd: any, i: number) => R.put(gd, [gd.char.pos[0] + 400 + i, 0, gd.char.pos[2]]);
      const blow = (c: any) => { if (process.env.WALLSAFE_BREAK !== '1') C.blow(c); };
      const caught: Record<string, boolean> = {};
      for (const [name, all] of [['indoors', ins]] as [string, any[]][]) {
        const cands = all.filter(roomy); let asked: any = null;
        const set = (): any => { g.restartEncounter(); R.step(5); R.kit();
          g.guards.forEach((gd: any, i: number) => { if (gd.state !== 'dead') far(gd, i); }); R.step(1); let why = cands.length ? '' : 'no wall of this kind has room to stage the line';
          for (const W0 of cands) { const s = samSets(R.W, W0, R.each); if (s.charge) { asked = W0; return s.charge; } why = `wall ${W0.index}: ${s.why}`; }
          line(false, `${name}: a charge set by Sam's own row on a wall with room to stage the line`, why); return null; };
        let c = set(); if (!c) continue;
        const W = c.wall, r = safeRadius(g, W), dA = 0.6 * r.reach, dB = r.reach + 0.5, c0 = wallPt(W, 0, 0);
        const [pA, pB, pM, pS] = stage(W).every(p => !!p) ? stage(W) : stage(asked);   // (the places are dealt off where the charge settled; where a slide along the wall leaves one of them no room, off the place it was asked at, which had it)
        if (!pA || !pB || !pM || !pS) { line(false, `${name}: room to stage three men and Sam with a line to the charge on wall ${W.index}`, `places at ${f2(dA)} ${f2(dB)} 4.00 ${f2(r.reach + 0.6)} m: ${[pA, pB, pM, pS].map(p => p ? 'yes' : 'NO').join(' ')}`); continue; }
        let [A, B, M] = R.up;
        R.put(A, pA); R.put(B, pB); R.put(M, pM); for (const gd of [A, B, M]) gd.dazzledUntil = -1; g.teleportPlayer(pS); pl.fireCd = 0; pl.sup.v = 0; R.step(1);
        const warn1: string[] = C.tooClose(c); blow(c); const dz = (gd: any) => gd.dazzledUntil - g.time, sam1 = { cd: pl.fireCd, sup: pl.sup.v, share: C.lastBlow?.sam ?? -1 };
        caught[name] = dz(M) > 0;
        line(dz(A) > 0 && dz(B) <= 0 && warn1.includes(A.callsign) && !warn1.includes(B.callsign) && !warn1.includes('you') && sam1.cd <= 0 && sam1.sup === 0 && sam1.share === 0,
          `${name}: a man inside the reach with a line to the charge is flash-blind, one just past it is not, and Sam past it feels nothing; the trigger's words named the man inside`,
          `wall ${W.index}, reach ${r.reach} m · ${A.callsign} at ${f2(flat(pA, c0))} m blind ${f2(Math.max(0, dz(A)))} s · ${B.callsign} at ${f2(flat(pB, c0))} m blind ${f2(Math.max(0, dz(B)))} s · Sam at ${f2(flat(pS, c0))} m: no-fire ${f2(Math.max(0, sam1.cd))} s, aim ${f2(sam1.sup)} · warned: ${warn1.join(', ') || 'nobody'}`);
        // … Sam inside it; a second man nearer the wall than the first is blind longer
        /** the squad's own charge on that wall: the floor put back, the men gathered at it, the wall's order to all, armed (null, and a line, where it does not come about) */
        const squadSets = (): any => { g.restartEncounter(); R.step(5); pl = g.player; R.kit(); for (const h of R.hostiles) if (h.state !== 'dead') far(h, 0);
          R.gather(wallPt(W, 0, 2.4) as Vec3, 40); const tAt = wallPt(W, 0, 0) as Vec3, n0 = C.ops.length, why = R.O.give(WALL_ORDER, wallTarget(g, tAt, { kind: 'floor', at: R.O.freeSpot(tAt) ?? tAt }), 'all'), op = C.ops[C.ops.length - 1];
          if (why || C.ops.length === n0 || !op) { line(false, `${name}: the squad takes the wall's order on wall ${W.index}`, `refused: ${why}`); return null; }
          if (!(R.until(() => op.phase === 'armed' || !C.ops.includes(op), 90) && op.phase === 'armed')) { line(false, `${name}: the squad's charge on wall ${W.index} is armed`, `phase '${op.phase}'${op.ended ? `, ended '${op.ended}'` : ''}`); return null; }
          return C.set[C.set.length - 1] ?? null; };
        const key = () => { if (process.env.WALLSAFE_BREAK !== '1') R.press('KeyF'); R.step(1); };   // (the squad's key, and a step on: what the blast did to the player is read off the game once it has stepped)
        c = squadSets(); if (!c) continue; [A, B, M] = R.up;
        // (his place is worked out on the floor as it stands now: the blast above opened walls that squadSets has put back, and one between him and the charge is a wall that shields him)
        const pS2 = place(W, 0.5 * r.reach, [pA, pB, pM]), pN = pS2 && place(W, 0.3 * r.reach, [pA, pB, pM, pS2]);
        // (with the wall standing this floor has no place for him on the charge's side: being rocked inside the reach, and shielded by a wall the charge does not take, is the Rust game's native test)
        if (!pS2 || !pN) { console.log(`    ${name}: no room for Sam inside the reach on this floor: at ${f2(0.5 * r.reach)} m ${pS2 ? 'yes' : 'no'} · a man at ${f2(0.3 * r.reach)} m ${pN ? 'yes' : 'no'}`); continue; }
        R.put(A, pA); for (const gd of [A, B, M]) gd.dazzledUntil = -1; g.teleportPlayer(pS2); pl.fireCd = 0; pl.sup.v = 0; pl.dazzledUntil = -1; R.step(1);
        const warn2: string[] = C.tooClose(c), note = notesOf(g).join(' / '), hits1 = pl.hitsLeft; key();
        // ('the nearer man is blind the longer' went 2026-11-07: a man that near the charge is killed by it — the charge's kill at its foot, a settled rule this check used to hold off)
        line(warn2.includes('you') && /TOO CLOSE: you/.test(note) && pl.fireCd >= WALLCHARGE.samSecs[1] - 0.05 && pl.fireCd <= WALLCHARGE.samSecs[0] && pl.sup.v >= WALLCHARGE.samSup && (C.lastBlow?.sam ?? 0) > 0 && !pl.down && pl.hitsLeft === hits1 && dz(A) > 0,
          `${name}: Sam inside the reach of the squad's charge is rocked — no round for a beat, his aim open — and is neither hit nor down; the note said so before it went`,
          `Sam at ${f2(flat(pS2, c0))} m: no-fire ${f2(pl.fireCd)} s, aim ${f2(pl.sup.v)}, share ${f2(C.lastBlow?.sam ?? 0)}, hits left ${pl.hitsLeft}/${hits1}, down ${pl.down} · ${A.callsign} at ${f2(flat(pA, c0))} m ${f2(Math.max(0, dz(A)))} s · note: '${note}'`);
      }
    }
  } finally { Object.assign(WALLCHARGE, was.wc); DESTRUCTION.on = was.sw; Math.random = was.rnd; }
  console.log(`=== wallsafe: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) { const i = process.argv.indexOf('--scn'); process.exit((await wallsafeCheck(i >= 0 ? process.argv[i + 1].split(',') : undefined)) ? 0 : 1); }
