// WHAT A MAN BELIEVES AND WHAT HE COVERS ARE TWO THINGS — the check behind src/game/belief.ts (BELIEF.on: on in the page, ?belief=0; off on the frozen stand,
// TPS_BELIEF=1) and the rule that a man put on a post gives up the walk to a door he meant to boot (DOORFORCE.postEnds: ?postends=0; TPS_POSTENDS=1).
//   bun run tools/qa/belief-check.ts [--verbose]      (also `probes.ts belief`, family roomfight; run it on BOTH stands: plain, and TPS_PROFILE=shipped in front)
// Headless, on the warehouse's room clear: one defender with a room of two doors or more, every other man parked blind and deaf. What he hears is a real report
// filed where the hearing model reads it. The check raises the rules itself on either stand and puts them back.
//   1  the door never     five seconds with a report heard and his watch made to move: nothing writes the point he watches into lastKnown
//   2  a report stays     a shot he hears is in his belief, near where it was fired, for as long as nothing newer says otherwise (it used to last one tick)
//   3  his gun            and through all of that his gun's point is what he covers, the same object as the room fight's watched point: no twitch toward the noise
//   4  no stamp           his watched door changing does not move his belief or stamp a fresh fix (frag.ts, picture.ts and the search read that stamp)
//   5  his post           a belief alone does not take him off his post
//   6  whose gun          gunPoint: his eyes this tick beat what he covers; last tick's eyes do not; a man let off his post has nothing to cover
//   7  the writers        nothing in src/game writes the watched point into lastKnown but cover(), and only belief.ts writes `covers`
//   8  off                the rule off: one field as it was, the door in lastKnown, `covers` never set, gunPoint is lastKnown
//   9  the boot's walk    a man walking to boot a door who is put on a post stops: the walk is over, no step taken after it (with the rule off he creeps on)
//  10  the stand          held off on the frozen stand, on as the page opens, FROZEN_HOLDS carries both rows and their words
//  11  the journal        a recording from before the table replays with the rules off; one that carries them keeps its own word
//  12  one tick, two reports a blast or a mate's word placed this tick stands: what he hears of the same moment does not write over it
// BELIEF_BREAK=1 = the rules stay down where the check raises them, which puts the door back into his belief: lines 1, 2, 4, 9 and 12 must fail.
// exit 0 = all hold · 1 = one does not. Nothing here is a picture: no gun has been looked at on a GPU by this check.
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
type Vec3 = [number, number, number];
const VERBOSE = process.argv.includes('--verbose');

export async function beliefCheck(): Promise<boolean> {
  const BREAK = process.env.BELIEF_BREAK === '1';
  const { standUp, ROOT } = await import('./headless.ts');
  const { BELIEF, gunPoint, word } = await import(`${ROOT}/src/game/belief.ts`);
  const { DOORFORCE, DOORFORCE_STATS, doorForceOn } = await import(`${ROOT}/src/game/doorforce.ts`);
  const { ROOMFIGHT } = await import(`${ROOT}/src/game/roomfight.ts`);
  const { hearingCheck } = await import(`${ROOT}/src/game/guards.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { knobsSnapshot, applyKnobs } = await import(`${ROOT}/src/game/journal.ts`);
  const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
  const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
  const DT = 1 / 60;
  let failed = false; const line = (ok: boolean, s: string) => { if (!ok) failed = true; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}`); };
  const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; if (!VERBOSE) console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
  const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const stood = { on: null as boolean | null, post: null as boolean | null };
  const was = { on: BELIEF.on, post: DOORFORCE.postEnds, door: DOORFORCE.on, kick: DOORFORCE.kickSecs, noShot: DOORFORCE.kickSecsNoShot, rally: ROOMFIGHT.rally };
  const raise = (on: boolean) => { if (stood.on === null) { stood.on = BELIEF.on; stood.post = DOORFORCE.postEnds; } BELIEF.on = on && !BREAK; DOORFORCE.postEnds = on && !BREAK; };

  /** the warehouse's room clear stood up: `D` a defender with a room of two doors or more, holding, roused; every other man parked blind and deaf, away from it */
  const world = async (on: boolean) => {
    const W: any = await quiet(async () => { const w: any = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'warehouse.json'), 'utf8'))) }); await startFromQuery(w.game, new URLSearchParams('?levelFile=warehouse.json&roomclear=1&squad4')); return w; });
    const g = W.game; raise(on); g.godMode = true; g.playerInvisible = true; g.aiEnabled = true; g.tune.tactics = false; ROOMFIGHT.rally = false;
    for (let i = 0; i < 3; i++) W.step(DT);
    const D = g.fight.defs.find((X: any) => X.room && X.room.entries.length >= 2 && X.mode === 'hold' && X.gd.state !== 'dead');
    if (!D) throw new Error('no defender with a room of two doors');
    g.guards.forEach((o: any, i: number) => { if (o === D.gd) return; o.pinned = true; o.hold = true; o.dazzledUntil = 1e9; o.heardUpTo = 1e12; o.char.pos = [1 + i * 0.1, 0, 1]; o.char.vel = [0, 0, 0]; o.path = []; o.pathGoal = null; o.script = null; });
    const G = D.gd; G.state = 'alert'; G.awareness = 0.95; G.reactT = 0; G.lastKnown = null; G.lastSeenT = -100; G.sightT = -100; G.fixT = -100; G.fixRef = null; G.heardUpTo = g.eventSeq; G.dazzledUntil = -1; G.covers = null;
    return { W, g, D, G };
  };
  /** a report he hears: from just outside his room's first door, somebody else's gun */
  const report = (g: any, D: any) => { const E = D.room.entries[0], at: Vec3 = [E.at[0] + E.out[0] * 2, 0, E.at[2] + E.out[1] * 2]; g.eventSeq++; g.events.push({ kind: 'shot', pos: at, time: g.time, loud: true, level: 2, id: g.eventSeq, who: g.player.char.id, weapon: g.player.weapon?.def?.id }); return at; };
  /** counts what is written into a man's lastKnown while `fn` runs, the watched point apart */
  const trap = (D: any, fn: () => void) => { const G = D.gd; let v = G.lastKnown, door = 0, all = 0; Object.defineProperty(G, 'lastKnown', { configurable: true, enumerable: true, get: () => v, set: (x: any) => { all++; if (x === D.pt) door++; v = x; } }); try { fn(); } finally { delete G.lastKnown; G.lastKnown = v; } return { door, all }; };

  try {
    // ------------------------------------------------------------ 1, 2, 3 a report and a moving watch
    { const { W, g, D, G } = await world(true); const at = report(g, D); let stay = 0, near = Infinity, gun = 0, ticks = 0, heard = false; const moved0: number[] = [...G.char.pos];
      const T = trap(D, () => { for (let i = 0; i < 300; i++) { if (i === 120) D.ev[1] = 3; W.step(DT); const lk = G.lastKnown; if (i > 3) { ticks++; if (lk && lk !== D.pt) { stay++; near = Math.min(near, flat(lk, at)); heard = true; } if (gunPoint(g, G) === D.pt) gun++; } } });
      line(T.door === 0, `the door never: ${T.door} of ${T.all} writes into his belief in five seconds were the point he watches`);
      line(heard && stay === ticks && near < 8, `a report stays: in his belief ${stay} of ${ticks} ticks after he heard it, ${near.toFixed(1)} m from where it was fired`);
      line(gun === ticks, `his gun: its point was what he covers ${gun} of ${ticks} ticks`);
      line(flat(moved0, G.char.pos) < 0.3, `his post: ${flat(moved0, G.char.pos).toFixed(2)} m from where he stood after five seconds with a report in his belief`); }
    // ------------------------------------------------------------ 4 no stamp
    { const { W, g, D, G } = await world(true); G.lastKnown = [G.char.pos[0], 0, G.char.pos[2]]; const held = G.lastKnown; W.step(DT); const t0 = G.fixT, w0 = D.watch; D.ev.fill(0); const to = D.watch === 0 ? 1 : 0; let swapped = false;
      for (let i = 0; i < 300 && !swapped; i++) { D.ev[to] = 3; W.step(DT); swapped = D.watch === to; }
      line(swapped && G.lastKnown === held && G.fixT === t0, `no stamp: his watch went from entry ${w0} to ${D.watch}${swapped ? '' : ' (it never moved)'}; his belief ${G.lastKnown === held ? 'stood' : 'moved'}, the stamp on his fix ${G.fixT === t0 ? 'stood' : 'moved'}`); }
    // ------------------------------------------------------------ 6 whose gun
    { const { W, g, D, G } = await world(true); W.step(DT); const seen: Vec3 = [G.char.pos[0] + 5, 0, G.char.pos[2]]; G.lastKnown = seen; G.sightPos = [seen[0], 0, seen[2]]; G.sightT = g.time; const cv = G.covers ?? D.pt; G.covers = cv; G.pinned = true; G.hold = true;
      const a = gunPoint(g, G) === seen; G.sightT = g.time - 0.02; const b = gunPoint(g, G) === cv; G.pinned = false; const c = gunPoint(g, G) === seen; G.pinned = true; G.hold = false; const d = gunPoint(g, G) === seen; G.hold = true;
      line(a && b && c && d, `whose gun: eyes this tick → the man (${a}); eyes last tick → what he covers (${b}); let off his post → what he believes (${c}, ${d})`); }
    // ------------------------------------------------------------ 7 the writers
    { const dir = resolve(ROOT, 'src/game'), bad: string[] = []; const allowed = new Set(['belief.ts']);
      for (const f of readdirSync(dir)) { if (!f.endsWith('.ts')) continue; const t = readFileSync(resolve(dir, f), 'utf8'); if (/lastKnown\s*=\s*(D|F)\.pt\b/.test(t) && f !== 'belief.ts') bad.push(f); if (/\.covers\s*=[^=]/.test(t) && !allowed.has(f)) bad.push(f); }
      line(!bad.length, `the writers: nothing writes the watched point into lastKnown by hand and only ${[...allowed].join(', ')} writes \`covers\`${bad.length ? `; found in ${bad.join(', ')}` : ''}`); }
    // ------------------------------------------------------------ 8 off
    { const { W, g, D, G } = await world(false); BELIEF.on = false; DOORFORCE.postEnds = false; let inField = 0, nulls = 0, same = 0; const n = 120;
      for (let i = 0; i < n; i++) { W.step(DT); if (G.lastKnown === D.pt) inField++; if (G.covers === null) nulls++; if (gunPoint(g, G) === G.lastKnown) same++; }
      line(inField === n && nulls === n && same === n, `off: the door in his one field ${inField} of ${n} ticks, \`covers\` null ${nulls}, gunPoint is lastKnown ${same}`); }
    // ------------------------------------------------------------ 9 the boot's walk: an alert free man on the office, sure of a shut door; put on a post on his way
    { const run = async (on: boolean) => {
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
        if (!walking) return { walking, after: 0, kick: null as string | null };
        G.hold = true; G.pinned = true;   // what a rule that stops him short does (roomfight.ts laneStop → plant)
        const p0 = [...c.pos]; for (let i = 0; i < 60; i++) W.step(DT);
        return { walking, after: flat(p0, c.pos), kick: doorForceOn(g, G)?.kick ?? null };
      };
      const a = await run(true);
      line(a.walking && a.kick === null && a.after < 0.25, `the boot's walk: walking to boot it (${a.walking}), put on a post: the walk ${a.kick === null ? 'is over' : 'goes on, ' + a.kick}, ${a.after.toFixed(2)} m in the second after`); }
    // ------------------------------------------------------------ 10 the stand
    { const word = process.env.TPS_BELIEF, wordP = process.env.TPS_POSTENDS, shipped = process.env.TPS_PROFILE === 'shipped', want = word !== undefined ? word === '1' : shipped, wantP = wordP !== undefined ? wordP === '1' : shipped;
      const rowA = FROZEN_HOLDS['BELIEF.on'], rowB = FROZEN_HOLDS['DOORFORCE.postEnds'];
      line(stood.on === want && stood.post === wantP && /TPS_BELIEF=1/.test(rowA ?? '') && /TPS_POSTENDS=1/.test(rowB ?? ''), `the stand: ${want ? 'on, as the page opens' : 'held off, as every frozen digest was recorded'} (BELIEF ${stood.on}, postEnds ${stood.post}); FROZEN_HOLDS: ${rowA} · ${rowB}`); }
    // ------------------------------------------------------------ 11 the journal
    { const W: any = await quiet(() => standUp()); const g = W.game, snap = () => JSON.parse(JSON.stringify(knobsSnapshot(g)));
      BELIEF.on = true; DOORFORCE.postEnds = true; const old = snap(); delete old.tables.BELIEF; delete old.tables.DOORFORCE.postEnds; applyKnobs(g, old); const before = [BELIEF.on, DOORFORCE.postEnds];
      BELIEF.on = false; DOORFORCE.postEnds = false; const own = snap(); own.tables.BELIEF.on = true; own.tables.DOORFORCE.postEnds = true; applyKnobs(g, own); const carried = [BELIEF.on, DOORFORCE.postEnds];
      line(before[0] === false && before[1] === false && carried[0] === true && carried[1] === true, `the journal: a recording from before the tables replays with the rules off (${before}) · one that carries them keeps its own word (${carried})`); }
    // ------------------------------------------------------------ 12 one tick, two reports
    { const { g, D, G } = await world(true); G.state = 'suspicious'; const X: Vec3 = [G.char.pos[0] + 1, 0, G.char.pos[2]]; word(g, G, X); G.heardUpTo = g.eventSeq; report(g, D); hearingCheck(g, G); const firstStood = G.lastKnown === X;
      G.wordT = -100; G.heardUpTo = g.eventSeq; report(g, D); hearingCheck(g, G); const nextTook = G.lastKnown !== X;
      line(firstStood && nextTook, `one tick, two reports: a word placed this tick stood against a report he heard in it (${firstStood}); with no word this tick the report takes his belief (${nextTook})`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { BELIEF.on = was.on; DOORFORCE.postEnds = was.post; DOORFORCE.on = was.door; DOORFORCE.kickSecs = was.kick; DOORFORCE.kickSecsNoShot = was.noShot; ROOMFIGHT.rally = was.rally; }
  console.log(failed ? '=== belief-check: FAIL ===' : '=== belief-check: ALL PASS ===');
  return !failed;
}

if (import.meta.main) { const ok = await beliefCheck(); process.exit(ok ? 0 : 1); }
