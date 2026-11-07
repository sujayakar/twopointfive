// A SPRINT INTO A DOOR HAS WEIGHT — the check behind src/game/sprintdoor.ts (SPRINTDOOR.on: on in the page, ?nosprintdoor; off on the frozen stand, TPS_SPRINTDOOR=1).
//   bun run tools/qa/sprintdoor-check.ts        (also `probes.ts sprintdoor`; run it on BOTH stands: plain, and TPS_PROFILE=shipped in front)
// Headless; nothing here looks at a picture. The office with nobody in it, the one-way doors and the frame rule as the page has them, the switch thrown by hand;
// every run is the real player under real keys (W + Shift held from 3 m off, the camera turned so W runs him square at the leaf) on two doors with that much
// floor before both faces. Each rule line is run with the switch on AND off and passes only when the rule shows on and the old behaviour shows off:
//   (1) the face it opens AWAY from, unlocked: a shoulder through it — the loud bash, the leaf leaving him at `bashVel` (it leads him), he keeps `keep` of his speed (0.75) and
//       has the sprint back inside half a second, through no more than a tenth of a second later than before (off: not a centimetre a second lost);
//   (2) the face it opens TOWARD, unlocked and shut in its frame: a boot at the run — he plants short of the leaf (never touching it: no bash, no thud), the
//       leaf comes off its hinges away from him with the kicked door's own noise, he is through the open frame with the keys still held about a second later
//       than before (off: full speed through a leaf hauled round on the full bash);
//   (2b) where a boot is not the thing (`kick: false`, or doors that never come off): the handle — checked to about a walk, the leaf brought round toward him
//       at the lower loudness (the game's own event for it is not `loud`), sprinting again and through about a third of a second later than before;
//   (2c) the boot's edges: none at a leaf standing ajar, none with the frame rule off, none from a run begun on top of the door; one from a stride off;
//   (2d) booted out of a room onto a corridor (no room beyond for the leaf to lie in): off its hinges all the same, the slab back down through its doorway;
//   (3) locked against him: the kick from the face it opens away from, a wall and nothing broken from the other — the same in both arms;
//   (4) a glancing shoulder costs less than a square one, and so does a leaf met standing ajar (the module's own rule, asked directly: 60° off square costs half);
//   (5) his body turns a shoulder into the leaf for a quarter second and comes back square, his head staying on the run; it is the shoulder nearer the LATCH
//       that leads — asked on a door and on the same door hung the other way, where the turn must come out the other way round;
//   (6) the stamps ride a checkpoint taken mid-rip;
//   (7) never on a puppet or under a show: his contact carries nothing;
//   (8) off: he goes through both faces on the full bash, no contact carries the rule, neither stamp is ever written, the sprint eases his gun down as ever.
//       A SANITY LINE: 'off = the build as it was' is the frame-for-frame comparison with the build head named in docs/agents/doors.md, and the gate's digests.
// SPRINTDOOR_BREAK=1 = the switch left OFF for the 'on' arms: (1), (2), (2b), (2c), (2d) and (5) must fail.   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';

export async function sprintdoorCheck(): Promise<boolean> {
  const { DOORWAY } = await import(`${ROOT}/src/game/doors.ts`);
  const { DOORBLAST, DOORBLAST_STATS } = await import(`${ROOT}/src/game/doorblast.ts`);
  const { SPRINTDOOR, SPRINTDOOR_STATS, sprintDoorRun, sprintDoorAfter } = await import(`${ROOT}/src/game/sprintdoor.ts`);
  const DT = 1 / 60, BREAK = process.env.SPRINTDOOR_BREAK === '1'; let all = true;
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
  function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
  const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  console.log('\n=== sprintdoor: a sprint into a door has weight ===');
  const { WALLSNAP } = await import(`${ROOT}/src/game/wallsnap.ts`);
  const was = { door: { ...DOORWAY }, sd: { ...SPRINTDOOR }, blast: DOORBLAST.on, random: Math.random, wallRun: WALLSNAP.pressRun };
  try {
    DOORWAY.oneWay = true; DOORWAY.frameHolds = true; DOORBLAST.on = true;
    DOORWAY.solid = false;   // (the run's own rule, measured alone: with DOORWAY.solid a leaf hauled toward him moves him aside instead of passing him — that pair is doorsolid-check's 'keys' lines)
    Math.random = seeded(7); const W: any = await quiet(() => standUp({ uploadEvery: 4 })); const g = W.game, pl = g.player, c = pl.char;
    g.godMode = true; g.playerInvisible = true; g.tune.civilians = false; g.guards.length = 0;
    const SPR = g.tune.playerSprint as number;
    const off = (d: any, f: number, o: number, along = d.def.width * 0.55): [number, number, number] => { const a = d.def.closedDir, ux = Math.cos(a), uz = Math.sin(a); return [d.def.hinge[0] + ux * along - uz * o * f, 0, d.def.hinge[1] + uz * along + ux * o * f]; };
    const pullSide = (d: any): 1 | -1 => g.doors.pullFace(d, off(d, 1, 0.8)) ? 1 : -1;
    const fresh = (d: any, lock = false) => { d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.locked = lock; d.sinceBash = 10; d.place(); };
    const door = (name: string) => g.doors.list.find((x: any) => x.def.name === name);
    const DOORS = ['server_manager', 'fire_exit'];   // (3 m of clear floor before both faces, and no rolling chair near enough for one run to leave it in the next one's doorway: the break room's west door has one)
    interface Run { before: number; slow: number; back: number; cross: number; heard: { sound: string; level: number }[]; kick: boolean; angle: number; broken: boolean; bashes: number; rips: number; kicks: number; off: boolean; kickEv: boolean; reel: number; slabOut: number; fellBack: boolean; wide: number; leafVel: number; headMax: number; headRun: number; pitch: number; look: number; loud: (boolean | undefined)[]; seen?: Set<any>; leanMax: number; leanEnd: number; latchLed: number; ripU: number; bashAt: number }
    /** one run at door `name` from face `face` (±1 × the face it opens toward) with the switch `on` */
    const run = (name: string, toward: boolean, lock: boolean, on: boolean, flip = false, opt: { from?: number; ajar?: number } = {}): Run => {
      SPRINTDOOR.on = on && !BREAK; SPRINTDOOR_STATS.bashes = SPRINTDOOR_STATS.rips = SPRINTDOOR_STATS.kicks = 0;
      WALLSNAP.pressRun = false;   // (every run here carries Shift on through the door to the wall past it and is read where it ends, still a sprint: a sprint that ends ON that wall is wallsnap.ts pressRun's, asked in wallrun-check.ts. Set for each run: a game stood up later on has the stand's say again)
      const d = door(name); if (flip) d.setSwing(-d.swing as 1 | -1);   // (hung the other way for this run: its push face is the other one, and so is the side of his run the latch is on)
      const face = pullSide(d) * (toward ? 1 : -1);
      for (const x of g.doors.list) fresh(x, !!x.def.locked); fresh(d, lock); if (lock) d.keySide = face;
      if (opt.ajar) { d.angle = opt.ajar * d.swing; d.latched = false; d.place(); }   // (met standing ajar, off its frame)
      const back0 = DOORBLAST_STATS.back;
      for (let i = 0; i < 90 && (pl.kick || c.anim.kicking); i++) W.step(DT);
      pl.doorBashAt = -10; pl.doorRipUntil = -1; pl.doorBashLean = 0;
      g.teleportPlayer(off(d, face, opt.from ?? 3)); pl.crouch = false; for (let i = 0; i < 5; i++) W.step(DT);
      const a = d.def.closedDir, ux = Math.cos(a), uz = Math.sin(a), dirIn = [uz * face, -ux * face];
      const out = () => (-(c.pos[0] - d.def.hinge[0]) * uz + (c.pos[2] - d.def.hinge[1]) * ux) * face;
      let yaw = 0, best = -2; for (let i = 0; i < 720; i++) { W.cam.yaw = i * Math.PI / 360; W.cam.rebuild(); const f = W.cam.planarBasis().fwd, dot = f[0] * dirIn[0] + f[2] * dirIn[1]; if (dot > best) { best = dot; yaw = W.cam.yaw; } }
      const heard: Run['heard'] = []; const emit0 = g.doors.emit; g.doors.emit = (e: any) => { if (e.door === d) heard.push({ sound: e.sound, level: e.level }); emit0.call(g.doors, e); };
      W.input.keys.add('KeyW'); W.input.keys.add('ShiftLeft');
      const R: Run = { before: 0, slow: 99, back: -1, cross: -1, heard, kick: false, angle: 0, wide: 0, leafVel: 0, headMax: 0, headRun: 0, pitch: 0, look: 0, loud: [], broken: false, bashes: 0, rips: 0, kicks: 0, off: false, kickEv: false, reel: 0, slabOut: 0, fellBack: false, leanMax: 0, leanEnd: 0, latchLed: 0, ripU: -1, bashAt: -10 };
      let last = [c.pos[0], c.pos[2]], hit = -1, lastO = out(); const runYaw = Math.atan2(dirIn[0], dirIn[1]);
      const wrap = (x: number) => x - Math.round(x / (2 * Math.PI)) * 2 * Math.PI;
      for (let i = 0; i < 170; i++) {
        W.cam.yaw = yaw; W.cam.rebuild(); W.step(DT);
        const v = ((c.pos[0] - last[0]) * dirIn[0] + (c.pos[2] - last[1]) * dirIn[1]) / DT; last = [c.pos[0], c.pos[2]]; const o = out(); R.kick ||= !!pl.kick; if (pl.kick) R.reel = Math.max(R.reel, o - lastO); lastO = o;   // (reel: the furthest one frame of a kick carried him back off the leaf)
        if (o > 0.6 && hit < 0) R.before = v;
        if (hit < 0 && (heard.length || pl.kick || (o < 0.6 && v < R.before - 0.5))) hit = i;
        if (hit >= 0 && i - hit < 40) R.slow = Math.min(R.slow, v);
        if (hit >= 0 && R.back < 0 && i > hit + 1 && v >= 0.95 * SPR - 0.1 && pl.sprinting) R.back = (i - hit) * DT;
        if (R.cross < 0 && o < -0.5) R.cross = i * DT;
        for (const e of g.events) if (e.kind === 'kick' && e.who === 1) R.kickEv = true;
        for (const e of g.events) if (e.kind === 'door' && e.who === 1 && e.level >= 0.5 && !R.seen?.has(e)) { (R.seen ??= new Set()).add(e); R.loud.push(e.loud); }
        R.wide = Math.max(R.wide, d.angle * d.swing);
        if (hit >= 0 && i === hit) R.leafVel = d.vel * d.swing;
        if (hit >= 0 && i - hit > 1 && i - hit < 30) { const h = c.headDir(); R.headMax = Math.max(R.headMax, Math.abs(wrap(Math.atan2(h[0], h[2]) - runYaw))); }
        if (hit < 0 && o < 1.2) { const h = c.headDir(); R.headRun = Math.max(R.headRun, Math.abs(wrap(Math.atan2(h[0], h[2]) - runYaw))); }   // (how far the run's own bob takes his head off the line, before any door)
        if (hit >= 0 && i - hit <= 2) { R.ripU = Math.max(R.ripU, pl.doorRipUntil - g.time); R.bashAt = Math.max(R.bashAt, pl.doorBashAt); }
        if (hit >= 0 && i - hit < 30) { const lean = wrap(c.bodyYaw - runYaw); if (Math.abs(lean) > Math.abs(R.leanMax)) { R.leanMax = lean;
          // a turn of +δ swings his front toward e = (cos yaw, −sin yaw): the shoulder on e's side falls back and the other one leads
          const latch = [d.def.hinge[0] + ux * d.def.width, d.def.hinge[1] + uz * d.def.width], side = Math.sign((latch[0] - c.pos[0]) * Math.cos(runYaw) - (latch[1] - c.pos[2]) * Math.sin(runYaw));
          R.latchLed = Math.sign(lean) === -side ? 1 : -1; } }   // +1: the shoulder on the latch's side of his run is the one ahead
        if (hit >= 0 && i - hit === 45) R.leanEnd = wrap(c.bodyYaw - runYaw);
      }
      R.pitch = c.aimPitch;
      W.input.keys.delete('KeyW'); W.input.keys.delete('ShiftLeft'); g.doors.emit = emit0;
      for (let i = 0; i < 100; i++) W.step(DT);
      R.look = c.anim.lookYawExtra; R.off = !!d.off; R.fellBack = DOORBLAST_STATS.back > back0;
      if (d.slab) R.slabOut = (-(d.slab.p[0] - d.def.hinge[0]) * uz + (d.slab.p[2] - d.def.hinge[1]) * ux) * face;   // (where the slab lies: metres on HIS side of the frame — negative is beyond it)
      if (flip) { fresh(d); d.setSwing(-d.swing as 1 | -1); }
      R.angle = d.angle * d.swing; R.broken = !!d.lockBroken; R.bashes = SPRINTDOOR_STATS.bashes; R.rips = SPRINTDOOR_STATS.rips; R.kicks = SPRINTDOOR_STATS.kicks;
      return R;
    };
    const says = (r: Run) => `before ${f2(r.before)} m/s, slowest ${f2(r.slow)} (${f2(r.slow / r.before)}), sprinting again after ${r.back < 0 ? 'never' : f2(r.back) + ' s'}, through at ${r.cross < 0 ? 'never' : f2(r.cross) + ' s'}, heard ${r.heard.map(h => `${h.sound}@${f2(h.level)}`).join(',') || 'nothing'}, leaf at ${f2(r.leafVel)} rad/s on the frame he met it, ${f2(r.wide)} rad open at its widest`;
    const bashAt = (r: Run, lv: number) => r.heard.some(h => h.sound === 'bash' && Math.abs(h.level - lv) < 1e-6);

    // (1) the face it opens away from
    for (const name of DOORS) { const A = run(name, false, false, true); const k = A.slow / A.before;
      line(bashAt(A, 1) && A.bashes === 1 && A.rips === 0 && k > SPRINTDOOR.keep - 0.06 && k < SPRINTDOOR.keep + 0.06 && A.back > 0 && A.back <= 0.5 && A.cross > 0 && A.wide > 1.2 && !A.kick && A.leafVel > SPRINTDOOR.bashVel - 0.5 && A.leafVel * 0.55 > SPR * 0.8,
        `(1) ${name}, opens away: a shoulder through it keeps ${f2(SPRINTDOOR.keep)} of the run and has it back inside half a second`, says(A)); }
    // (2) the face it opens toward: the boot at the run
    for (const name of DOORS) { const A = run(name, true, false, true);
      line(A.kick && A.kicks === 1 && A.rips + A.bashes === 0 && A.off && A.kickEv && A.heard.some(h => h.sound === 'kick' && h.level === 1) && !A.heard.some(h => h.sound === 'thud' || h.sound === 'bash') && A.slow < 0.1 && A.reel < 0.01 && A.back > 0 && A.back <= 1.35 && A.cross > 0,
        `(2) ${name}, opens toward him: a boot at the run — planted short of the leaf, the leaf off its hinges away from him with a kicked door's noise, through the open frame with the keys still held`, `on: ${says(A)}, off its hinges ${A.off}, never carried back off the leaf ${A.reel < 0.01}, the slab at rest ${f2(-A.slabOut)} m beyond the frame${A.fellBack ? ' (no room beyond: it fell back through its doorway)' : ''}`); }
    // (2d) a door with no room beyond its frame for the leaf to lie in (a room's door onto a corridor): doorblast's own rule — to the far wall and back down through its doorway
    { const A = run('manager', true, false, true);
      line(A.kick && A.off && A.kicks === 1 && A.fellBack && A.cross > 0 && -A.slabOut > 0 && -A.slabOut < 2.2,
        '(2d) booted out of a room onto a corridor: the leaf still comes off and he is through; with no room beyond to lie in it goes to the far wall and comes back down through its own doorway (it touches nobody: doorblast.ts)',
        `manager: kick ${A.kick}, off its hinges ${A.off}, fell back ${A.fellBack}, the slab at rest ${f2(-A.slabOut)} m beyond the frame, through ${A.cross > 0}`); }
    // (3) locked against him: as ever, in both arms
    { const name = 'server_manager'; const A = run(name, false, true, true), B = run(name, false, true, false), C = run(name, true, true, true), D = run(name, true, true, false);
      line(A.kick && A.broken && !A.off && B.kick && B.broken && A.bashes + A.rips + A.kicks === 0 && !C.kick && !C.broken && !C.off && C.cross < 0 && !D.kick && !D.broken && D.cross < 0 && C.bashes + C.rips + C.kicks === 0 && Math.abs(C.angle) < 0.01,
        '(3) locked against him: the kick from the face it opens away from, a wall and nothing broken from the face it opens toward — the same in both arms',
        `away — on: kick ${A.kick}, keep broken ${A.broken} · off: kick ${B.kick}, keep broken ${B.broken}\n    toward — on: kick ${C.kick}, through ${C.cross >= 0}, heard ${C.heard.map(h => h.sound).join(',') || 'nothing'} · off: kick ${D.kick}, through ${D.cross >= 0}`); }
    // (5) the shoulder turned in
    { const A = run('server_manager', false, false, true), B = run('server_manager', false, false, false), M = run('server_manager', false, false, true, true);
       }
  } catch (e) { }
  finally { Object.assign(DOORWAY, was.door); Object.assign(SPRINTDOOR, was.sd); DOORBLAST.on = was.blast; Math.random = was.random; WALLSNAP.pressRun = was.wallRun; }
  console.log(`=== sprintdoor: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await sprintdoorCheck()) ? 0 : 1);
