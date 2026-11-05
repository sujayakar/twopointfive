// WHAT A MAN BELIEVES AND WHAT HE COVERS ARE TWO THINGS — the check behind src/game/belief.ts (BELIEF.on: on in the page, ?belief=0; off on the frozen stand,
// TPS_BELIEF=1).
//   bun run tools/qa/belief-check.ts [--verbose]      (also `probes.ts belief`, family roomfight; run it on BOTH stands: plain, and TPS_PROFILE=shipped in front)
// Headless, on the warehouse's room clear: one defender with a room of two doors or more, every other man parked blind and deaf. What he hears is a real report
// filed where the hearing model reads it. The check raises the rule itself on either stand and puts it back.
//   1  the door never     five seconds with a report heard and his watch made to move: nothing writes the point he watches into lastKnown
//   2  a report stays     a shot he hears is in his belief, near where it was fired, for as long as nothing newer says otherwise (it used to last one tick)
//   3  his gun            and through all of that his gun's point is what he covers, the same object as the room fight's watched point: no twitch toward the noise
//   4  no stamp           his watched door changing does not move his belief or stamp a fresh fix (frag.ts, picture.ts and the search read that stamp)
//   5  his post           a belief alone does not take him off his post
//   6  whose gun          gunPoint: his eyes this tick beat what he covers; last tick's eyes do not; a man let off his post has nothing to cover
//   7  the writers        nothing in src/game writes the watched point into lastKnown but cover(), and only belief.ts writes `covers`
//   8  off                the rule off: one field as it was, the door in lastKnown, `covers` never set, gunPoint is lastKnown
//   9  the stand          held off on the frozen stand, on as the page opens, FROZEN_HOLDS carries its row and its word
//  10  the journal        a recording from before the table replays with the rule off; one that carries it keeps its own word
//  11  one tick, two reports a blast or a mate's word placed this tick stands: what he hears of the same moment does not write over it
//  12  a late hearing    the same in the tick's real order: a word and its report filed together, the man's update a tick on. A report filed after it takes his belief
//  13  a led fix          a man he has just lost, the fix led to where he was going (guards.ts leadFix): still what his eyes gave him
//  14  a duty ended       what a man covers is said anew each tick: off the room fight's roster, nothing is left in the field and his gun goes on his belief
//  15  a blast's place    a blast placed by word and heard: the room fight's evidence takes the place of the blast, and a thirty second old noise does not pass for it
// BELIEF_BREAK=1 = the rule stays down where the check raises it, which puts the door back into his belief: lines 1, 2, 4, 11, 12 and 15 must fail.
// exit 0 = all hold · 1 = one does not. Nothing here is a picture: no gun has been looked at on a GPU by this check.
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
type Vec3 = [number, number, number];
const VERBOSE = process.argv.includes('--verbose');

export async function beliefCheck(): Promise<boolean> {
  const BREAK = process.env.BELIEF_BREAK === '1';
  const { standUp, ROOT } = await import('./headless.ts');
  const { BELIEF, cover, eyesFix, gunPoint, word } = await import(`${ROOT}/src/game/belief.ts`);
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
  const stood = { on: null as boolean | null };
  const was = { on: BELIEF.on, rally: ROOMFIGHT.rally, fight: ROOMFIGHT.on };
  const raise = (on: boolean) => { if (stood.on === null) stood.on = BELIEF.on; BELIEF.on = on && !BREAK; };

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
    { const { W, g, D, G } = await world(false); BELIEF.on = false; let inField = 0, nulls = 0, same = 0; const n = 120;
      for (let i = 0; i < n; i++) { W.step(DT); if (G.lastKnown === D.pt) inField++; if (G.covers === null) nulls++; if (gunPoint(g, G) === G.lastKnown) same++; }
      line(inField === n && nulls === n && same === n, `off: the door in his one field ${inField} of ${n} ticks, \`covers\` null ${nulls}, gunPoint is lastKnown ${same}`); }
    // ------------------------------------------------------------ 9 the stand
    { const word = process.env.TPS_BELIEF, shipped = process.env.TPS_PROFILE === 'shipped', want = word !== undefined ? word === '1' : shipped, row = FROZEN_HOLDS['BELIEF.on'];
      line(stood.on === want && /TPS_BELIEF=1/.test(row ?? ''), `the stand: ${want ? 'on, as the page opens' : 'held off, as every frozen digest was recorded'} (${stood.on}); FROZEN_HOLDS: ${row}`); }
    // ------------------------------------------------------------ 10 the journal
    { const W: any = await quiet(() => standUp()); const g = W.game, snap = () => JSON.parse(JSON.stringify(knobsSnapshot(g)));
      BELIEF.on = true; const old = snap(); delete old.tables.BELIEF; applyKnobs(g, old); const before = BELIEF.on;
      BELIEF.on = false; const own = snap(); own.tables.BELIEF.on = true; applyKnobs(g, own); const carried = BELIEF.on;
      line(before === false && carried === true, `the journal: a recording from before the table replays with the rule off (${before}) · one that carries it keeps its own word (${carried})`); }
    // ------------------------------------------------------------ 11 one tick, two reports
    { const { g, D, G } = await world(true); G.state = 'suspicious'; const X: Vec3 = [G.char.pos[0] + 1, 0, G.char.pos[2]]; word(g, G, X); G.heardUpTo = g.eventSeq; report(g, D); hearingCheck(g, G); const firstStood = !!G.lastKnown && flat(G.lastKnown, X) < 0.01;
      G.wordT = -100; G.heardUpTo = g.eventSeq; report(g, D); hearingCheck(g, G); const nextTook = !G.lastKnown || flat(G.lastKnown, X) > 0.01;
      line(firstStood && nextTook, `one tick, two reports: a word placed this tick stood against a report he heard in it (${firstStood}); with no word this tick the report takes his belief (${nextTook})`); }
    // ------------------------------------------------------------ 12 a late hearing
    { const { W, g, D, G } = await world(true); W.step(DT); G.state = 'suspicious'; G.heardUpTo = g.eventSeq; const X: Vec3 = [G.char.pos[0] + 1, 0, G.char.pos[2]]; word(g, G, X); report(g, D); W.step(DT); const stood = !!G.lastKnown && flat(G.lastKnown, X) < 0.01;
      W.step(DT); report(g, D); W.step(DT); const took = !G.lastKnown || flat(G.lastKnown, X) > 0.01;
      line(stood && took, `a late hearing: the report filed with the word, heard a tick on, left it standing (${stood}); one filed after it took his belief (${took})`); }
    // ------------------------------------------------------------ 13 a led fix
    { const { W, g, D, G } = await world(true); W.step(DT); const eye: Vec3 = [G.char.pos[0], 1.6, G.char.pos[2]]; let spot: Vec3 | null = null;
      for (let r = 2; r < 14 && !spot; r += 0.5) for (let a = 0; a < 16 && !spot; a++) { const p: Vec3 = [G.char.pos[0] + Math.cos(a / 16 * 6.283) * r, 0, G.char.pos[2] + Math.sin(a / 16 * 6.283) * r]; if (!g.col.nav.isBlocked(p[0], p[2]) && g.col.segmentBlocked(eye, [p[0], 1.6, p[2]]) && g.col.segmentBlocked(eye, [p[0], 1.1, p[2]])) spot = p; }
      if (!spot) throw new Error('no spot out of his sight');
      g.teleportPlayer([...spot]); g.player.char.update(0); G.foe = 'player'; const at: Vec3 = [spot[0], 0, spot[2]]; G.sightPos = [...at] as Vec3; G.sightVel = [3, 0, 0]; G.sightT = g.time; G.ledT = -100; G.lastSeenT = g.time; G.lastKnown = [...at] as Vec3; W.step(DT);
      const led = !!G.lastKnown && flat(G.lastKnown, at) > 0.05;
      line(led && eyesFix(G, D.pt) === G.lastKnown, `a led fix: his fix was led ${G.lastKnown ? flat(G.lastKnown, at).toFixed(2) : '-'} m from the spot he lost the man at, and it is still what his eyes gave him (${eyesFix(G, D.pt) === G.lastKnown})`); }
    // ------------------------------------------------------------ 14 a duty ended
    { const { W, g, D, G } = await world(true); W.step(DT); const had = G.covers === D.pt || BREAK; ROOMFIGHT.on = false; G.hold = true; G.pinned = true; G.lastKnown = [G.char.pos[0] + 3, 0, G.char.pos[2]]; const held = G.lastKnown; cover(G, [G.char.pos[0], 0, G.char.pos[2] + 3]); const said = BREAK ? null : G.covers; W.step(DT); const gun = gunPoint(g, G); ROOMFIGHT.on = was.fight;
      line(had && G.covers === null && (BREAK || (said !== null && gun === held)), `a duty ended: on the roster he covered his door (${had}); with no duty saying it again nothing is left in the field (${G.covers === null}) and his gun goes on his belief (${gun === held})`); }
    // ------------------------------------------------------------ 15 a blast's place
    { const { W, g, D, G } = await world(true); W.step(DT); G.state = 'suspicious'; G.heardUpTo = g.eventSeq; const E0 = D.room.entries[0], E1 = D.room.entries[1], at: Vec3 = [E0.at[0] + E0.out[0], 0, E0.at[2] + E0.out[1]]; D.noiseAt = [E1.at[0], 0, E1.at[2]]; D.noiseT = g.time - 30; D.ev.fill(0);
      word(g, G, at); g.eventSeq++; g.events.push({ kind: 'bang', pos: [...at], time: g.time, loud: true, level: 2, id: g.eventSeq }); W.step(DT); W.step(DT);
      line(!!D.noiseAt && flat(D.noiseAt, at) < 0.5 && D.ev[0] > D.ev[1], `a blast's place: the room fight has the noise ${D.noiseAt ? flat(D.noiseAt, at).toFixed(2) : '-'} m from the blast, its evidence on that door (${D.ev[0].toFixed(2)} against ${D.ev[1].toFixed(2)})`); }
  } catch (err) { line(false, `threw: ${(err as Error)?.stack ?? err}`); }
  finally { BELIEF.on = was.on; ROOMFIGHT.rally = was.rally; ROOMFIGHT.on = was.fight; }
  console.log(failed ? '=== belief-check: FAIL ===' : '=== belief-check: ALL PASS ===');
  return !failed;
}

if (import.meta.main) { const ok = await beliefCheck(); process.exit(ok ? 0 : 1); }
