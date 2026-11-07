// THE WALL CHARGE'S SAFETY RADIUS — the check behind WALLCHARGE.safe (src/game/wallcharge.ts safeRadius; a settled rule: every tool stands the game with it on).
//   bun run tools/qa/wallsafe-check.ts [--scn room-clear-four,block-squads] [--verbose]        (also `probes.ts wallsafe`)
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
import { resolve } from 'node:path';
import { samSets } from './charge-rig.ts';
const ROOT = resolve(import.meta.dir, '../..');
type Vec3 = [number, number, number];

/** the two levels asked when none is named: each has a room to play the charge in from a fresh start */
const FRESH = ['room-clear-four', 'block-squads'];
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
      const outs = outerSites(R, 'outside', 6).map((s: any) => s.W), P0 = partition(R), ins = [...(P0 ? [P0.W] : []), ...outerSites(R, 'inside', 6).map((s: any) => s.W)];
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
      // (2) the marks, over every site found
      { let n = 0, bad = 0, fell = 0, least = { out: Infinity, in: Infinity }, hidden = 0; const worst: string[] = [];
        for (const W of [...outs, ...ins]) { const r = safeRadius(g, W), c = wallPt(W, 0, 0), spot = wallPt(W, 0, WALLCHARGE.setOff), safe = C.safePosts?.(W, 4, spot) ?? null, posts: Vec3[] | null = C.standoffPosts(W, 4); if (!posts) continue; n++;
          if (!safe) { fell++; if (JSON.stringify(posts) !== JSON.stringify(C.standoffPosts(W, 4, undefined, true))) { bad++; worst.push(`wall ${W.index}: no safe marks, and the fallback is not the old marks`); } continue; }
          for (const q of posts) { const d = flat(q, c), seen = lined(W, q); if (!seen) hidden++; else least[r.indoors ? 'in' : 'out'] = Math.min(least[r.indoors ? 'in' : 'out'], d);
            if (!g.cleared.knownAt(q) && g.cleared.roomAt(q) !== g.cleared.roomAt(spot)) { bad++; worst.push(`wall ${W.index}: a mark at ${at(q)} in a room nobody of the squad has stood in`); }
            if (!seen && !col.segmentBlocked(src(W), [q[0], 1.1, q[2]]) && d < r.reach) { bad++; worst.push(`wall ${W.index}: a mark at ${at(q)} with only glass between it and the charge`); }
            if (C.blastShare(W, q, undefined, false) > 0 /* a door leaf is no cover: the blast flings it */ || (seen && d < r.standoff - 0.05) || d < WALLCHARGE.standoff[0] - 0.05) { bad++; worst.push(`wall ${W.index}: a mark ${f2(d)} m from the charge, ${seen ? 'in its line' : 'out of its line'} (stand-off ${r.standoff})`); } } }
        line(n >= 2 && bad === 0 && least.out >= 4 - 0.05 && least.in >= 6 - 0.05, 'every stand-off mark is out of the blast: at the radius or more (4 m outdoors, 6 m indoors), or with something solid between it and the charge (a door leaf and a pane of glass do not count), and on floor the squad holds',
          `${n} places · nearest mark with a line to the charge: ${f2(least.out)} m outdoors, ${isFinite(least.in) ? `${f2(least.in)} m` : 'none (all out of its line)'} indoors · ${hidden} marks out of its line · ${fell} places with no room fell back to the old marks${worst.length ? ` · ${worst.slice(0, 3).join(' · ')}` : ''}`); }
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
      for (const [name, all] of [['outdoors', outs], ['indoors', ins]] as [string, any[]][]) {
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
        const pS2 = place(W, 0.5 * r.reach, [pA, pB, pM]), pN = pS2 && place(W, 0.3 * r.reach, [pA, pB, pM, pS2]);
        if (!pS2 || !pN) { line(false, `${name}: room for Sam inside the reach`, `at ${f2(0.5 * r.reach)} m: ${pS2 ? 'yes' : 'NO'} · a man at ${f2(0.3 * r.reach)} m: ${pN ? 'yes' : 'NO'}`); continue; }
        /** the squad's own charge on that wall: the floor put back, the men gathered at it, the wall's order to all, armed (null, and a line, where it does not come about) */
        const squadSets = (): any => { g.restartEncounter(); R.step(5); pl = g.player; R.kit(); for (const h of R.hostiles) if (h.state !== 'dead') far(h, 0);
          R.gather(wallPt(W, 0, 2.4) as Vec3, 40); const tAt = wallPt(W, 0, 0) as Vec3, n0 = C.ops.length, why = R.O.give(WALL_ORDER, wallTarget(g, tAt, { kind: 'floor', at: R.O.freeSpot(tAt) ?? tAt }), 'all'), op = C.ops[C.ops.length - 1];
          if (why || C.ops.length === n0 || !op) { line(false, `${name}: the squad takes the wall's order on wall ${W.index}`, `refused: ${why}`); return null; }
          if (!(R.until(() => op.phase === 'armed' || !C.ops.includes(op), 90) && op.phase === 'armed')) { line(false, `${name}: the squad's charge on wall ${W.index} is armed`, `phase '${op.phase}'${op.ended ? `, ended '${op.ended}'` : ''}`); return null; }
          return C.set[C.set.length - 1] ?? null; };
        const key = () => { if (process.env.WALLSAFE_BREAK !== '1') R.press('KeyF'); R.step(1); };   // (the squad's key, and a step on: what the blast did to the player is read off the game once it has stepped)
        c = squadSets(); if (!c) continue; [A, B, M] = R.up;
        R.put(A, pA); for (const gd of [A, B, M]) gd.dazzledUntil = -1; g.teleportPlayer(pS2); pl.fireCd = 0; pl.sup.v = 0; pl.dazzledUntil = -1; R.step(1);
        const warn2: string[] = C.tooClose(c), note = (C as any).hudNotes().map((n: any) => n.text).join(' / '), hits1 = pl.hitsLeft; key();
        // ('the nearer man is blind the longer' went 2026-11-07: a man that near the charge is killed by it — the charge's kill at its foot, a settled rule this check used to hold off)
        line(warn2.includes('you') && /TOO CLOSE: you/.test(note) && pl.fireCd >= WALLCHARGE.samSecs[1] - 0.05 && pl.fireCd <= WALLCHARGE.samSecs[0] && pl.sup.v >= WALLCHARGE.samSup && (C.lastBlow?.sam ?? 0) > 0 && !pl.down && pl.hitsLeft === hits1 && dz(A) > 0,
          `${name}: Sam inside the reach of the squad's charge is rocked — no round for a beat, his aim open — and is neither hit nor down; the note said so before it went`,
          `Sam at ${f2(flat(pS2, c0))} m: no-fire ${f2(pl.fireCd)} s, aim ${f2(pl.sup.v)}, share ${f2(C.lastBlow?.sam ?? 0)}, hits left ${pl.hitsLeft}/${hits1}, down ${pl.down} · ${A.callsign} at ${f2(flat(pA, c0))} m ${f2(Math.max(0, dz(A)))} s · note: '${note}'`);
        // … and it blinds him as it blinds a teammate (samstun.ts, the player's stun rule)
        c = squadSets(); if (!c) continue;
        g.teleportPlayer(pS2); pl.fireCd = 0; pl.sup.v = 0; pl.dazzledUntil = -1; R.step(1); key();
        { const dS = flat(pS2, c0), owed = WALLCHARGE.safeSecs[0] + (WALLCHARGE.safeSecs[1] - WALLCHARGE.safeSecs[0]) * Math.min(1, dS / r.reach), got = Math.max(0, pl.dazzledUntil - g.time) + 1 / 60;   // (the one step since the blast put back)
          line(Math.abs(got - owed) < 0.05, `${name}: Sam inside the reach is blind the seconds a teammate standing there gets (the player's stun rule)`, `Sam at ${f2(dS)} m: blind ${f2(got)} s · a teammate there is owed ${f2(owed)} s`); pl.dazzledUntil = -1; }
      }
      line(caught.indoors === true && caught.outdoors === false, 'indoors reaches further than outdoors: a man 4 m from the charge with a line to it is caught indoors and clear outdoors', `outdoors ${caught.outdoors ? 'caught' : 'clear'} · indoors ${caught.indoors ? 'caught' : 'clear'}`);
    }
    // (4) played: the order indoors, in a place with room and in one without
    for (const want of ['room', 'tight'] as const) {
      let done = false;
      // (`taken`, the second try of 'room' only: a level whose every room is too small to get clear in from a FRESH start — the squad holds nothing but the floor
      //  it stands on — is asked again as it is in play, with the rooms behind the squad taken: every room known, the defenders off the floor. NEVER for the two
      //  levels this check runs by default: they have such a place from a fresh start, and one gone missing there is the rule broken, not the staging)
      for (const taken of want === 'room' ? [false, true] : [false]) for (const scn of scns) { if (done) break; if (taken && FRESH.includes(scn)) continue;
        const R = await rig(scn, 7), g = R.game, C = R.C; R.kit();
        if (taken) { const nav = g.col.nav; for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]); R.step(2);
          for (let z = 0; z < nav.H; z++) for (let x = 0; x < nav.W; x++) g.cleared.mark([(x + 0.5) * nav.cell, 0, (z + 0.5) * nav.cell]); }
        if (want === 'tight') { WALLCHARGE.safeStandoff = [40, 40]; WALLCHARGE.safeHideM = 0; }   // (no room: the same place, the radius asked for more floor than the level has and no corner looked for)
        const P0 = partition(R), sites = [...(P0 ? [P0] : []), ...outerSites(R, 'inside', 6)].filter((s: any) => chargeIndoors(g, s.W) && !!C.safePosts(s.W, 4, wallPt(s.W, 0, WALLCHARGE.setOff)) === (want === 'room'));
        const P = sites[0]; if (!P) continue;
        R.gather(P.near); for (const h of R.hostiles) if (h.state !== 'dead' && flat(h.char.pos, wallPt(P.W, 0, 0)) < DOORTOOLS.stunM + 1) R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]); R.step(2);
        const tAt: Vec3 = [P.p[0], 0, P.p[2]], why = g.orders.give(WALL_ORDER, wallTarget(g, tAt, { kind: 'floor', at: g.orders.freeSpot(tAt) ?? tAt }), 'all'), op = C.ops[0]; if (!op) { line(false, `played (${want}, ${scn}): the order is taken`, `refused: ${why}`); done = true; break; }
        const W = op.wall, r = safeRadius(g, W), c0 = wallPt(W, 0, 0), armed = R.until(() => op.phase === 'armed', 90), men: any[] = [...op.men];
        const share = men.map(m => ({ m, d: flat(m.char.pos, c0), k: C.blastShare(W, m.char.pos) })), words = R.spoke.filter((s: any) => /your call/.test(s.text)).map((s: any) => s.text), tightWords = words.filter((t: string) => /can't get clear/.test(t));
        // (Sam steps out of it before his word: along the wall past the radius, or wherever the squad's own marks are)
        const sam = [8, -8, 7, -7, 6.5, -6.5].map(d => wallPt(W, d, 0.9) as Vec3).find(q => R.free(q) && C.blastShare(W, q) === 0) ?? men[0].char.pos; g.teleportPlayer([sam[0], 0, sam[2]]); R.step(2);
        const note = (C as any).hudNotes().map((n: any) => n.text).join(' / '), b0 = C.stats.backblast; R.press('KeyF'); const back = C.stats.backblast - b0, blind = men.filter(m => m.dazzledUntil > g.time);
        const tell = `${scn}${taken ? ' (no room from a fresh start: staged with the building taken)' : ''}, wall ${W.index} at ${at(c0)} (reach ${r.reach} m): ${share.map(x => `${x.m.callsign} ${f2(x.d)} m${x.k > 0 ? ' IN IT' : ''}`).join(' · ')} · said: ${words.map((t: string) => `'${t}'`).join(' ') || 'nothing'} · note: '${note}' · ${back} caught`;
        if (want === 'room') line(armed && !op.close && share.every(x => x.k === 0) && words.length === 1 && !tightWords.length && !/CLOSE/.test(note) && back === 0 && !blind.length && (C.lastBlow?.indoors ?? false),
          'played indoors: the squad sets it, stands off out of the blast, and at Sam\'s F nobody of theirs is caught', tell);
        else line(armed && !!op.close && share.some(x => x.k > 0) && words.length === 1 && tightWords.length === 1 && /TOO CLOSE/.test(note) && back > 0,
          'played in a place with no room to get clear: the order is still taken, the man who calls it says so once, the note names who is inside, and they are caught', tell);
        done = true;
      }
      if (!done) line(false, `played (${want}): a place to play it`, 'none found');
    }
    // (6) a mark the men cannot walk to is no mark: the block's mess wall, where the floor straight back from the charge is behind a locked door — the
    //     marks along the wall are dealt, never the old ones with 'no room' (the scripted commander's wall: all four were caught there before this was asked)
    if (scns.includes('block-squads')) { const R = await rig('block-squads', 1), g = R.game, C = R.C; R.kit();
      for (const h of R.hostiles) R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]); R.gather([8.3, 0, 13.7]);
      const why = C.order([11.0, 0, 11.3], 'all'), op = C.ops[0], posts: [any, Vec3][] = op ? [...op.post] : [], W = op?.wall;
      line(!!op && !op.close && posts.length >= 2 && posts.every(([m, q]) => !g.orders.reach(m, q) && C.blastShare(W, q) === 0),
        'a place behind a locked door is passed over: every dealt mark is one its man can walk to, out of the blast', op ? `block-squads, wall ${W.index}: ${posts.map(([m, q]) => `${m.callsign} ${at(q)} ${f2(flat(q, wallPt(W, 0, 0)))} m`).join(' · ')} · close ${op.close}` : `refused: ${why}`); }
    // (7) ours: the block's corridor wall with the mess's defenders left where they stand — the nearest place out of the blast's line is their room, and nobody is sent into it
    if (scns.includes('block-squads')) { const R = await rig('block-squads', 5), g = R.game, C = R.C; R.kit(); R.gather([11.8, 0, 13.2]); R.step(2);
      const why = C.order([11.8, 0, 12.1], 'all'), op = C.ops[0], posts: [any, Vec3][] = op ? [...op.post] : [], W = op?.wall, room0 = W ? g.cleared.roomAt(wallPt(W, 0, WALLCHARGE.setOff)) : 0;
      const theirs = new Set(R.hostiles.filter((h: any) => h.state !== 'dead').map((h: any) => g.cleared.roomAt(h.char.pos))); theirs.delete(0); theirs.delete(room0);
      const inTheirs = posts.filter(([, q]) => theirs.has(g.cleared.roomAt(q))), unknown = posts.filter(([, q]) => !g.cleared.knownAt(q) && g.cleared.roomAt(q) !== room0);
      line(!!op && posts.length >= 2 && !inTheirs.length && !unknown.length, 'no mark is in a room the squad has not been in: with the defenders in place next door, the squad stands off on its own floor',
        op ? `block-squads, wall ${W.index}: ${posts.map(([m, q]) => `${m.callsign} ${at(q)} room ${g.cleared.roomAt(q)}${theirs.has(g.cleared.roomAt(q)) ? ' THEIRS' : ''}`).join(' · ')} · defenders' rooms ${[...theirs].join(', ') || 'none'}` : `refused: ${why}`); }
    // (6) BACK AT THE HOLE (WALLCHARGE.safeBack). Mousehole's ward is 3.5 m wide: under the indoor radii no place in it is clear of a charge on its partition, so the
    //     squad stands off out through the hole it came in by. Dealt by the straight line and up to safeWalkM of walk, that was marks on another side of the building
    //     and a man back at the new hole eight seconds after the blast (the maintainer's replay); and the men stood `afterBlow` out on marks that far back.
    if (scns === FRESH || scns.includes('mousehole-four')) { console.log('  — mousehole-four, a charge set from inside the ward');
      const R = await rig('mousehole-four', 1), g = R.game, C = R.C, O = g.orders; C.setCarried(5);
      const give = (p: Vec3) => O.give(WALL_ORDER, wallTarget(g, p, { kind: 'floor', at: O.freeSpot(p) ?? p }), 'all'), cap = WALLCHARGE.safeBackM;
      let op: any = null; for (const x of [12, 12.6, 11.4, 13.2, 10.8, 13.8, 10.2]) { give([x, 0, 7.0]); op = C.ops[0]; if (op) break; }
      const in1 = !!op && R.until(() => op.phase === 'armed' || !C.ops.includes(op), 90) && op.phase === 'armed';
      if (in1) { const W = op.wall, q = [20, -20, 8, -8, 6, -6].map(d => wallPt(W, d, 3) as Vec3).find(p => R.free(p) && C.blastShare(W, p) === 0) ?? g.player.char.pos; g.teleportPlayer([q[0], 0, q[2]]); R.step(2); C.blow(C.nearest()); R.until(() => !C.ops.includes(op), 30); }
      for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
      const inWard = in1 && R.gather([12, 0, 11], 40); R.step(60);
      if (!inWard) line(false, 'the ward: a way in blown through its yard wall and the four gathered inside', `the first charge ${in1 ? 'blown' : 'never armed'}`);
      else {
        type Deal = { close: boolean; walks: number[]; share: number[] };
        const marks = (z: number): Deal | null => { let o: any = null; for (const dz of [0, 0.6, -0.6]) { give([15.0, 0, z + dz]); o = C.ops[0]; if (o) break; } if (!o) return null;
          const post: [any, Vec3][] = [...o.post], res = { close: !!o.close, walks: post.map(([, q]) => R.pathLen(q, o.spot) as number), share: post.map(([, q]) => C.blastShare(o.wall, q, undefined, false) as number) }; C.end(o, 'asked'); R.step(2); return res; };
        const far = (d: Deal | null) => d ? Math.max(...d.walks) : NaN, say = (d: Deal | null) => d ? `${d.close ? "'can't get clear'" : 'stood off'}, walks back ${d.walks.map(f2).join(' ')} m` : 'refused';
        const b = marks(8.5), d = marks(13.5);
        line(!!b && !b.close && far(b) <= cap + 0.5 && b.share.every(k => k === 0),
          `the marks are dealt by the walk back: no man is stood off more than ${cap} m of walk from the charge, and every mark is still out of the blast`, `low on the ward's partition · ${say(b)}`);
        line(!!d && (d.close || far(d) <= cap + 0.5),
          'where the floor has not four such places the squad stays in the room and the man who calls it says so, in place of a mark on another side of the building', `high on the partition · ${say(d)}`);
        const play = (z: number) => { R.gather([12, 0, 11], 40); R.step(60); let o: any = null; for (const dz of [0, 0.6, -0.6]) { give([15.0, 0, z + dz]); o = C.ops[0]; if (o) break; }
          const armed = !!o && R.until(() => o.phase === 'armed' || !C.ops.includes(o), 90) && o.phase === 'armed', caught = armed ? o.men.filter((m: any) => C.blastShare(o.wall, m.char.pos, undefined, false) > 0).length : -1;
          if (armed) { C.blow(C.nearest()); R.step(18); } const res = { armed, caught, phase: armed ? o.phase as string : '', hole: armed && !!o.hole, near: armed && o.hole ? Math.min(...o.men.map((m: any) => flat(m.char.pos, wallPt(o.wall, o.hole.du, 0)))) : NaN };
          if (armed) R.until(() => !C.ops.includes(o), 30); return res; };
        const on = play(9), told = (r: typeof on) => r.armed ? `${r.phase === 'wait' ? 'still standing on their marks' : r.phase === 'enter' ? 'on their way' : `'${r.phase}'`}${r.hole ? '' : ' (no hole)'} · caught in it ${r.caught} · nearest ${f2(r.near)} m from the hole` : 'the charge never armed';
        line(on.armed && on.hole && on.caught === 0 && on.phase === 'enter',
          `stood off clear of it, the men start for the hole on the blast: the ${WALLCHARGE.afterBlow} s beat is the time before anybody is AT the hole, and their walk is longer than that`, `0.3 s after the blast · ${told(on)}`);
      }
    }
    // (8) THE FAR SIDE OF THE WARNING. The trigger's words named who was inside the radius on the charge's own side only. On a wall with free floor beyond it:
    //     a teammate who has walked round to stand beyond the wall where it will open is named ('behind the wall: …'), and the blast then does catch him; one
    //     beyond the wall but past the stun's reach is not named and not caught; Sam there is named and blinded. Each on a charge Sam's own hands set, Sam's own
    //     line in a game of its own on a whole wall. (Until 2026-11-07 the probe `wallkeep`, on charges pushed onto the tools' copy of the game by hand.)
    { const thickOf = (W: any) => 2 * (W.alongX ? W.box.h[2] : W.box.h[0]), K = DOORTOOLS;
      /** a game on `scn` with a charge of Sam's own on a wall that has free floor beyond it inside and past the stun's reach, everybody moved off first */
      const stageOn = async (scn: string) => { const R = await rig(scn, 5), g = R.game, C = R.C, col = g.col; R.kit();
        const P0 = partition(R), sites = [...(P0 ? [P0] : []), ...outerSites(R, 'inside', 6)]; let why = 'no wall with free floor beyond it inside and past the stun\'s reach';
        g.guards.forEach((gd: any, i: number) => { if (gd.state !== 'dead') R.put(gd, [gd.char.pos[0] + 400 + i, 0, gd.char.pos[2]]); }); R.step(1);
        for (const P of sites) { const W0 = P.W, T = thickOf(W0), c0 = wallPt(W0, 0, 0), inn = wallPt(W0, 0, -(T + 0.45), 1.1);
          const lined = (q: number[]) => !col.solidBlocked(inn, [q[0], 1.1, q[2]]);
          const beyond = (lo: number, hi: number, not: number[][] = []): Vec3 | null => { for (const du of [0, 0.6, -0.6, 1.2, -1.2, 1.8, -1.8]) for (let off = 0.8; off <= 3.2; off += 0.4) { const q = wallPt(W0, du, -(T + off)) as Vec3, d = flat(q, c0); if (d >= lo && d <= hi && R.free(q) && lined(q) && !not.some(n => flat(n, q) < 0.8)) return q; } return null; };
          const past = (): Vec3 | null => { for (const du of [0, 2, -2, 4, -4, 6, -6]) for (let off = 1.0; off <= 9; off += 0.5) { const q = wallPt(W0, du, -(T + off)) as Vec3; if (flat(q, c0) > K.stunM + 0.4 && R.free(q)) return q; } return null; };
          const own = [6.5, -6.5, 7.5, -7.5, 8.5, -8.5].map(d => wallPt(W0, d, 0.9) as Vec3).find(q => R.free(q) && C.blastShare(W0, q) === 0);
          const pA = beyond(1.0, K.stunM - 0.6), pS = pA && beyond(1.0, K.stunM - 0.6, [pA]), pB = past();
          if (!pA || !pS || !pB || !own) continue;
          const set = samSets(R.W, W0, R.each); if (!set.charge) { why = `wall ${W0.index}: ${set.why}`; continue; }
          return { R, c: set.charge, c0, pA, pS, pB, own, why: '' }; }
        return { R, c: null as any, c0: [0, 0, 0] as Vec3, pA: null, pS: null, pB: null, own: null, why }; };
      const note = (C: any) => C.hudNotes().map((n: any) => n.text).join(' / ');
      let staged = false;
      for (const scn of scns) { if (staged) break;
        const S1 = await stageOn(scn); if (!S1.c) { console.log(`    (${scn}: the far side is not staged here — ${S1.why})`); continue; }
        staged = true;
        { const { R, c, c0 } = S1, pA = S1.pA!, pB = S1.pB!, own = S1.own!, g = R.game, C = R.C, pl = g.player, [A, B] = R.up;
          // a teammate beyond the wall inside the stun's reach, one beyond it past the reach; Sam clear on the charge's own side
          R.put(A, pA); R.put(B, pB); for (const gd of [A, B]) gd.dazzledUntil = -1; g.teleportPlayer(own); pl.dazzledUntil = -1; R.step(1);
          const near1: string[] = C.tooClose(c), far1: string[] = C.behind(c), n1 = note(C);
          if (process.env.WALLSAFE_BREAK !== '1') C.blow(c); const hole = !!C.holeAt(c.wall), caughtA = A.state === 'dead' || A.dazzledUntil > g.time, caughtB = B.state === 'dead' || B.dazzledUntil > g.time;
          line(hole && far1.includes(A.callsign) && !far1.includes(B.callsign) && !far1.includes('you') && !near1.length && new RegExp(`BEHIND THE WALL: ${A.callsign}$`).test(n1) && !/TOO CLOSE/.test(n1) && caughtA && !caughtB,
            'a teammate who walked round to stand beyond the wall where it will open is named before it goes, and is caught when it does; one beyond it past the stun\'s reach is neither',
            `${scn}, wall ${c.wall.index} at ${at(c0)} (stun reach ${K.stunM} m) · ${A.callsign} ${f2(flat(pA, c0))} m beyond: ${caughtA ? 'caught' : 'CLEAR'} · ${B.callsign} ${f2(flat(pB, c0))} m beyond: ${caughtB ? 'CAUGHT' : 'clear'} · note: '${n1}' · hole ${hole}`); }
        // Sam beyond the wall: a game of its own, a whole wall
        { const S2 = await stageOn(scn);
          if (!S2.c) line(false, 'a charge of Sam\'s own on a whole wall in a second game, for his own line', S2.why);
          else { const { R, c, c0 } = S2, pS = S2.pS!, g = R.game, C = R.C, pl = g.player;
            g.teleportPlayer(pS); pl.dazzledUntil = -1; R.step(1);
            const farS: string[] = C.behind(c), nS = note(C); if (process.env.WALLSAFE_BREAK !== '1') C.blow(c); const blind = pl.dazzledUntil > g.time;
            line(farS.includes('you') && /BEHIND THE WALL: you/.test(nS) && blind, 'Sam beyond the wall: he is named, and blinded when it goes',
              `Sam ${f2(flat(pS, c0))} m beyond wall ${c.wall.index} · named ${farS.join(', ') || 'nobody'} · blind ${f2(Math.max(0, pl.dazzledUntil - g.time))} s · note: '${nS}'`); } }
      }
      if (!staged) line(false, 'a wall to stage the far side on', 'none found on any level asked');
    }
  } finally { Object.assign(WALLCHARGE, was.wc); DESTRUCTION.on = was.sw; Math.random = was.rnd; }
  console.log(`=== wallsafe: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) { const i = process.argv.indexOf('--scn'); process.exit((await wallsafeCheck(i >= 0 ? process.argv[i + 1].split(',') : undefined)) ? 0 : 1); }
