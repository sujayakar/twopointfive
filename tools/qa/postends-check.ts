// A MAN PUT ON A POST GIVES UP HIS WALK TO A DOOR — the check behind DOORFORCE.postEnds (src/game/doorforce.ts): on in the page, ?postends=0; off on the frozen stand, TPS_POSTENDS=1.
//   bun run tools/qa/postends-check.ts      (also `probes.ts postends`, family doors; run it on BOTH stands: plain, and TPS_PROFILE=shipped in front)
// Headless, on the office: an alert free man, sure of a shut door, walking to boot it, and put on a post on his way, as a rule that stops him short does (lanes.ts plantShort).
//   1  the walk ends      the walk is over and he takes no step in the second after; he still believes in the door, so a posted man can fire on it
//   2  off                the rule off: the walk goes on under the post, as it was
//   3  the stand          held off on the frozen stand, on as the page opens, FROZEN_HOLDS carries its row and its word
//   4  the journal        a recording from before the key replays with the rule off; one that carries it keeps its own word
// POSTENDS_BREAK=1 = the rule stays down where the check raises it: line 1 must fail.
// exit 0 = all hold · 1 = one does not.
type Vec3 = [number, number, number];

export async function postendsCheck(): Promise<boolean> {
  const BREAK = process.env.POSTENDS_BREAK === '1';
  const { standUp, ROOT } = await import('./headless.ts');
  const { DOORFORCE, DOORFORCE_STATS, doorForceOn } = await import(`${ROOT}/src/game/doorforce.ts`);
  const { knobsSnapshot, applyKnobs } = await import(`${ROOT}/src/game/journal.ts`);
  const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
  const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
  const DT = 1 / 60;
  let failed = false; const line = (ok: boolean, s: string) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const stood = DOORFORCE.postEnds as boolean, was = { post: DOORFORCE.postEnds, door: DOORFORCE.on, kick: DOORFORCE.kickSecs, noShot: DOORFORCE.kickSecsNoShot };
  const raise = (on: boolean) => { DOORFORCE.postEnds = on && !BREAK; };

  const run = async (on: boolean) => {
    const W: any = await quiet(() => standUp()); const g = W.game; raise(on); DOORFORCE.on = true; DOORFORCE.kickSecs = 0.2; DOORFORCE.kickSecsNoShot = 0.2; g.godMode = true; g.aiEnabled = true; g.playerInvisible = false; g.tune.tactics = false; DOORFORCE_STATS.stands = 0;
    const G = g.guards[0], free = (p: Vec3) => !g.col.nav.isBlocked(p[0], p[2]); let spot: { me: Vec3; him: Vec3 } | null = null;
    for (const d of g.doors.list) { if (d.off || Math.abs(d.angle) > 0.02 || d.def.steel) continue; const cd = d.def.closedDir, ux = Math.cos(cd), uz = Math.sin(cd), mx = d.def.hinge[0] + ux * d.def.width * 0.5, mz = d.def.hinge[1] + uz * d.def.width * 0.5;
      const at = (s: number, off: number): Vec3 => [mx - uz * s * off, 0, mz + ux * s * off], sPull = g.doors.pullFace(d, at(1, 1)) ? 1 : -1, me = at(-sPull, 3), him = at(sPull, 0.9);
      const clear = (a: Vec3, b: Vec3) => { const dd = v3.sub([b[0], 1.2, b[2]], [a[0], 1.2, a[2]]), L = v3.len(dd); const h = g.col.raycast([a[0], 1.2, a[2]], v3.scale(dd, 1 / L), L - 0.25); return !!h && h.index >= 1e6 && g.col.dynamicBoxes[h.index - 1e6] === d.box; };   // (the leaf the first thing in that line)
      if (free(me) && free(him) && free(at(-sPull, 0.9)) && clear(me, him)) { spot = { me, him }; break; } }
    if (!spot) throw new Error('no door on the office to stand at');
    g.guards.forEach((o: any, i: number) => { if (o === G) return; o.hold = true; o.pinned = true; o.dazzledUntil = 1e9; o.heardUpTo = 1e12; o.char.pos = [37.5, 0, 25 + i * 1.2]; o.char.vel = [0, 0, 0]; o.path = []; o.pathGoal = null; });
    g.teleportPlayer([...spot.him]); const c = G.char; c.pos = [...spot.me]; c.vel = [0, 0, 0]; const yaw = Math.atan2(spot.him[0] - spot.me[0], spot.him[2] - spot.me[2]); c.bodyYaw = c.aimYaw = yaw; G.hold = false; G.pinned = false; G.state = 'alert'; G.awareness = 1; G.foe = 'player'; G.reactT = 0; G.fireCd = 0.3; G.reloadT = -1; G.lastSeenT = -100; G.sightT = -100; G.lastKnown = [...spot.him]; G.path = []; G.pathGoal = null; G.heardUpTo = g.eventSeq; G.dazzledUntil = -1; G.script = null; G.task = null; G.bodyDuty = null; G.flashlight.enabled = false; c.update(0);
    // (the player treading behind the leaf at his walk's level until the man is sure of it: kept where he stands till then, as doorforce-check.ts line 5 has him)
    let walking = false, next = 0; const t0 = g.time;
    for (let i = 0; i < 1200 && !walking; i++) { if (g.time - t0 >= next) { next += 0.3; g.events.push({ kind: 'step', pos: [...spot.him], time: g.time, loud: false, level: 0.26 }); } W.step(DT); if (DOORFORCE_STATS.stands === 0) { c.pos = [...spot.me]; c.vel = [0, 0, 0]; G.path = []; G.pathGoal = null; } walking = doorForceOn(g, G)?.kick === 'walk'; }
    if (!walking) return { walking, after: 0, kick: null as string | null, on: false };
    G.hold = true; G.pinned = true;   // what a rule that stops him short does (roomfight.ts laneStop → plant)
    const p0 = [...c.pos]; for (let i = 0; i < 60; i++) W.step(DT);
    return { walking, after: flat(p0, c.pos), kick: doorForceOn(g, G)?.kick ?? null, on: !!doorForceOn(g, G) };
  };
  try {
    const a = await run(true);
    line(a.walking && a.kick === null && a.on && a.after < 0.25, `the walk ends: walking to boot it (${a.walking}), put on a post: the walk ${a.kick === null ? 'is over' : 'goes on, ' + a.kick}, ${a.after.toFixed(2)} m in the second after; he still believes in the door (${a.on})`);
    const b = await run(false);
    line(b.walking && b.kick === 'walk', `off: walking to boot it (${b.walking}), put on a post: the walk ${b.kick === 'walk' ? 'goes on' : 'is over'}, ${b.after.toFixed(2)} m in the second after`);
    { const word = process.env.TPS_POSTENDS, shipped = process.env.TPS_PROFILE === 'shipped', want = word !== undefined ? word === '1' : shipped, row = FROZEN_HOLDS['DOORFORCE.postEnds'];
      line(stood === want && /TPS_POSTENDS=1/.test(row ?? ''), `the stand: ${want ? 'on, as the page opens' : 'held off, as every frozen digest was recorded'} (${stood}); FROZEN_HOLDS: ${row}`); }
    { const W: any = await quiet(() => standUp()); const g = W.game, snap = () => JSON.parse(JSON.stringify(knobsSnapshot(g)));
      DOORFORCE.postEnds = true; const old = snap(); delete old.tables.DOORFORCE.postEnds; applyKnobs(g, old); const before = DOORFORCE.postEnds;
      DOORFORCE.postEnds = false; const own = snap(); own.tables.DOORFORCE.postEnds = true; applyKnobs(g, own); const carried = DOORFORCE.postEnds;
      line(before === false && carried === true, `the journal: a recording from before the key replays with the rule off (${before}) · one that carries it keeps its own word (${carried})`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { DOORFORCE.postEnds = was.post; DOORFORCE.on = was.door; DOORFORCE.kickSecs = was.kick; DOORFORCE.kickSecsNoShot = was.noShot; }
  console.log(failed ? '=== postends-check: FAIL ===' : '=== postends-check: ALL PASS ===');
  return !failed;
}

if (import.meta.main) { const ok = await postendsCheck(); process.exit(ok ? 0 : 1); }
