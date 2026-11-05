// THE LAMPS' REACH MASKS ARE THE PICTURE'S TO KEEP (src/render/reachKeep.ts). Until 11-05 the masks, their wall raster and the queue that decides them again
// after a wall changes lived in the Game and were worked in its tick. This holds the move:
//   1  a stand with no picture has no keeper, and a game there does NO reach work at all: not at boot, not on a blast, not in the frames after it
//   2  the game is the same game with a keeper or without one (its digest after a blast), and the keeper's work moves no row of the state's hash
//   3  a snapshot put back with nobody telling the keeper: it reads the walls again by itself and every mask is the one a whole derivation gives on the
//      walls as they then stand, a directional light's too
//   4  the walls as authored again (the wall engine's reset): the masks the level booted with, at once, with nothing left waiting
//   5  a wide sun's mask is of its disc: asked twice for the same disc it is made once, and where the disc is another (the weather turned, a snapshot put
//      the old one back) the next step makes the mask again with nobody telling it
//   6  a step on the clock (`tickTimed`: the frame has the planners' pace) ends in the same masks as a step on the count, with the clock scripted: the
//      two mixed; a directional mask carried across steps; a second wall while it is in hand; the sun's disc turned while it is in hand; a wide low
//      sun let in through a gap cut in an outer wall, and a second gap cut while its job is in hand
//   bun run tools/qa/reachkeep-check.ts          exit 0 = all pass · 1 = a line failed
//   REACHKEEP_BREAK=1 = line 1 is asked of a stand WITH a keeper, in line 3 the keeper never sees a put-back, and in line 5 it takes no step after the
//   disc is made small, and in line 6 the clock's arm takes one step only: all four must FAIL
import { standUp, ROOT } from './headless.ts';
const BREAK = process.env.REACHKEEP_BREAK === '1';
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { REACH_WORK, reachMaskFor, reachLevel, buildWallRaster } = await import(`${ROOT}/src/render/reach.ts`);
const { takeSnapshot, restoreSnapshot, stateHash } = await import(`${ROOT}/src/game/rollback.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { PUT_BACKS } = await import(`${ROOT}/src/math/side.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
let fails = 0; const check = (ok: boolean, what: string, detail = ''): void => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? ` (${detail})` : ''}`); };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const DT = 1 / 60, quiet = <T>(f: () => T): T => { const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
const stand = async (reach: boolean): Promise<any> => { Math.random = seeded(7); DESTRUCTION.on = true; const W: any = await standUp(reach ? { reach: true, seed: 1 } : { seed: 1 }); W.game.godMode = true; W.game.aiEnabled = false; W.game.tune.chaos = 0.5; for (let i = 0; i < 3; i++) W.step(DT); return W; };
const wallsOf = (g: any): any[] => g.destr.walls.all().filter((w: any) => w.kind === 'wall' && Math.max(w.box.h[0], w.box.h[2]) >= 1.6);
const blast = (g: any, w: any): boolean => { const b = w.box, ax = b.h[0] >= b.h[2]; return !!g.destr.blast(ax ? [b.c[0], 0.3, b.c[2] + 0.3] : [b.c[0] + 0.3, 0.3, b.c[2]], { who: PLAYER_ID }).walls; };
const steps = (W: any, n: number): void => { for (let f = 0; f < n; f++) W.step(DT); };
/** every light's mask as the keeper holds it (a turning beacon's aside: its mask is of the frame it was decided on) */
const masks = (W: any): string[] => { const K = W.engine.reach; return W.engine.lights.lights.map((l: any) => { if (W.game.beacons.some((b: any) => b.light === l)) return 'beacon'; const m = l.kind === 2 ? K.dirOf(l) : K.of(l); return m ? Array.from(m as Uint32Array).join(',') : ''; }); };
/** how many masks are not what a whole derivation gives on the keeper's walls as they stand: a lamp's inside its range, a directional light's everywhere */
const stray = (W: any): { lamps: number; dirs: number; n: number; raster: boolean } => { const K = W.engine.reach, { solid: kept, interior, lamps, doors } = K.peek(); let bad = 0, dbad = 0, n = 0;
  const solid = buildWallRaster(W.engine.world.statics, doors) as Uint8Array, raster = kept.length === solid.length && solid.every((v, i) => v === kept[i]);   // (the walls as they stand NOW, read here: the keeper's own raster must be this one, and the masks are held against this one)
  for (const l of lamps as any[]) { if (W.game.beacons.some((b: any) => b.light === l)) continue; n++;
    const full = reachMaskFor(solid, l.pos, l.kind === 3 || l.kind === 1 ? l.dir : [0, 0, 0], l.radius, interior), has = K.of(l); let off = false;
    for (let ci = 0; ci < WORLD.gridW * WORLD.gridH && !off; ci++) { const x = WORLD.min[0] + (ci % WORLD.gridW + 0.5) * WORLD.gridCell, z = WORLD.min[2] + (Math.floor(ci / WORLD.gridW) + 0.5) * WORLD.gridCell;
      if (Math.hypot(x - l.pos[0], z - l.pos[2]) <= l.range + 1 && reachLevel(full, ci) !== reachLevel(has, ci)) off = true; }
    if (off) bad++; }
  for (const l of W.engine.lights.lights as any[]) if (l.kind === 2 && K.dirOf(l)) { n++; const inc = K.dirOf(l) as Uint32Array, whole = K.wholeOf(l) as Uint32Array; if (!whole || inc.some((v, i) => v !== whole[i])) dbad++; }   // (… over its disc, if its mask was made for one: the keeper's own whole derivation, nothing kept)
  return { lamps: bad, dirs: dbad, n, raster }; };

// 1 ---- no picture, no mask, no work
{ const w0 = { ...REACH_WORK }, A = await stand(BREAK), g = A.game, ws = wallsOf(g); let blown = 0; for (const w of ws) { if (blown >= 2) break; if (quiet(() => blast(g, w))) { blown++; steps(A, 60); } }
  const carried = (A.engine.lights.lights as any[]).filter(l => 'reach' in l || 'reachDir' in l).length, grew = (REACH_WORK.cells - w0.cells) + (REACH_WORK.steps - w0.steps) + (REACH_WORK.fields - w0.fields);
  check(A.engine.reach === undefined && typeof g.reachTick === 'undefined' && typeof g.buildDirReach === 'undefined' && carried === 0 && grew === 0 && blown === 2 && g.reachInterior instanceof Uint8Array,
    'a stand with no picture has no keeper, no light carries a mask, and a game stood up there, two walls blown and sixty frames after each, has decided no cell and stepped no sight line; it still knows which cells are under a roof',
    `${blown} walls blown · keeper ${A.engine.reach === undefined ? 'none' : 'THERE'} · cells, steps and fields of reach work since before the stand-up: ${grew}`); }

// 2 ---- the same game either way, and the keeper is nobody's state
const B = await stand(true), gB = B.game, K = B.engine.reach, boot = masks(B);
{ const A = await stand(false), wa = wallsOf(A.game), wb = wallsOf(gB); let ia = -1; for (let i = 0; i < wa.length && ia < 0; i++) if (quiet(() => blast(A.game, wa[i]))) ia = i;
  const okB = ia >= 0 && quiet(() => blast(gB, wb[ia]));
  const h0 = stateHash(gB), m0 = masks(B).join('|'); let ticks = 0; while (K.pending && ticks++ < 5000) K.tick(); const h1 = stateHash(gB), m1 = masks(B).join('|');
  steps(A, 120); steps(B, 120);
  const dA = JSON.stringify(digestGame(A.game, true)), dB = JSON.stringify(digestGame(gB, true));
  check(okB && dA === dB && h0 === h1 && m0 !== m1 && ticks > 0,
    'the game is the same game with a keeper or without (its digest 120 frames after a blown wall), and the keeper deciding its masks again moves no row of the state\'s hash',
    `digests ${dA === dB ? 'equal' : 'DIFFER'} · the hash before and after ${ticks} steps of the keeper ${h0 === h1 ? 'equal' : 'DIFFERS'} · masks ${m0 !== m1 ? 'changed' : 'DID NOT CHANGE'} in those steps`); }

// 3 ---- a snapshot put back, and nobody tells the keeper
{ const after1 = masks(B), s1 = takeSnapshot(gB), ws = wallsOf(gB); let second = false, tried = 0;
  for (const w of ws) { if (second || tried >= 12) break; tried++; if (!quiet(() => blast(gB, w))) continue; steps(B, 240); if (masks(B).join('|') !== after1.join('|')) second = true; }
  const after2 = masks(B), n2 = B.engine.world.statics.length, pb0 = PUT_BACKS.n;
  if (BREAK) Object.defineProperty(K, 'putBacks', { get: () => PUT_BACKS.n, set() { }, configurable: true });   // (the keeper never sees a put-back)
  restoreSnapshot(s1); const n1 = B.engine.world.statics.length; steps(B, 240); const back = masks(B), st = stray(B);
  const moved = back.filter((m, i) => m !== after2[i]).length, asAfter1 = back.filter((m, i) => m === after1[i]).length;
  check(second && PUT_BACKS.n === pb0 + 1 && n1 !== n2 && K.pending === 0 && moved > 0 && st.raster && st.lamps === 0 && st.dirs === 0,
    'a snapshot put back with nobody telling the keeper: it reads the walls again and every mask is what a whole derivation gives on the walls as they then stand, the directional light\'s too',
    `a second wall blown after the snapshot changed masks: ${second} · statics ${n2} then ${n1} · ${moved} masks moved back, ${asAfter1} of ${back.length} are the snapshot's moment's to the bit · the keeper's raster is the walls' as they stand: ${st.raster} · of ${st.n} compared ${st.lamps} lamps and ${st.dirs} directional differ from a whole derivation`); }

// 4 ---- the walls as authored again
{ gB.destr.walls.reset(); const now = masks(B), same = now.filter((m, i) => m === boot[i]).length, pend = K.pending; steps(B, 2); const later = masks(B).filter((m, i) => m === boot[i]).length;
  check(pend === 0 && same === now.length && later === now.length && now.length > 0,
    'the walls as authored again (the wall engine\'s reset): every mask is the one the level booted with at once, nothing waits, and two frames on they still are',
    `${same} of ${now.length} masks the boot's at once, ${later} two frames on · waiting: ${pend}`); }

// 5 ---- a wide sun's mask is of its disc
{ const moon = (B.engine.lights.lights as any[]).find(l => l.kind === 2 && K.dirOf(l)), r0 = moon?.radius, bits = (m: Uint32Array) => Array.from(m).join(',');
  if (!moon) check(false, 'a directional light with a mask to try the disc on', 'none on this level');
  else { moon.radius = 0.25; K.sun(moon, 0.25); const wide = K.dirOf(moon) as Uint32Array, c0 = REACH_WORK.cells; K.sun(moon, 0.25); const again = REACH_WORK.cells - c0, kept = K.dirOf(moon) === wide;
    moon.radius = 0.01; if (!BREAK) K.tick(); const narrow = K.dirOf(moon) as Uint32Array; moon.radius = 0.25; K.tick(); const back = K.dirOf(moon) as Uint32Array;
    let more = 0, less = 0; for (let ci = 0; ci < WORLD.gridW * WORLD.gridH; ci++) { const a = reachLevel(wide, ci) > 0, b = reachLevel(narrow, ci) > 0; if (a && !b) more++; if (b && !a) less++; }
    check(again === 0 && kept && narrow !== wide && less === 0 && more > 0 && back !== narrow && bits(back) === bits(wide),
      'a wide sun\'s mask is of its disc: asked twice for the same disc it is made once; with the light\'s disc made small (as a snapshot put back would) the next step makes the mask again, a part of the wide one\'s cells and no cell more; with the wide disc back, the wide mask again',
      `cells decided by the second ask: ${again} · the wide disc reaches ${more} cells the small one does not, the small one ${less} the wide does not · the wide mask again to the bit: ${bits(back) === bits(wide)}`);
    moon.radius = r0; K.dir(moon); } }

// 6 ---- a step on the clock gives the same masks as a step on the count
// (THE CLOCK IS SCRIPTED: every look at it is 0.7 ms on, so a step is three looks whatever the machine: no line here is the real clock's luck)
{ const C = await stand(true), g = C.game, KC = C.engine.reach, ws = wallsOf(g), bitsOf = (m: Uint32Array | null | undefined) => (m ? Array.from(m).join(',') : '');
  { let t = 0; KC.timeNow = () => (t += 0.7); }
  const inHand = (): boolean => (KC.peek().queue as any[]).some(q => q.dirJob);
  /** steps until nothing waits: all on the clock, or all on the count, or every third on the count (`mix`); `each` is called after every step */
  const drain = (how: 'clock' | 'count' | 'mix', each?: (i: number) => void): { ticks: number; carried: number } => { let ticks = 0, carried = 0;
    while (KC.pending && ticks < 20000) { if (how === 'count' || (how === 'mix' && ticks % 3 === 2)) KC.tick(); else KC.tickTimed(); ticks++; if (inHand()) carried++; each?.(ticks); if (BREAK && how === 'clock') break; } return { ticks, carried }; };
  const moon = (C.engine.lights.lights as any[]).find(l => l.kind === 2 && KC.dirOf(l)), before = masks(C).join('|'); let w: any = null, w2: any = null;
  for (const x of ws) { if (quiet(() => blast(g, x))) { w = x; break; } }
  // (a) the same wall blown three times on one stand: decided on the clock, on the count, and with the two mixed
  const T = drain('clock'), mT = masks(C).join('|'); g.destr.walls.reset(); const again = masks(C).join('|') === before;
  const okU = !!w && quiet(() => blast(g, w)); const U = drain('count'), mU = masks(C).join('|'); g.destr.walls.reset();
  const okM = !!w && quiet(() => blast(g, w)); const M = drain('mix'), mM = masks(C).join('|'), stA = stray(C);
  // (b) a second wall goes while the first's directional mask is in hand: the light is asked again, and the end is a whole derivation's
  let asked = false; g.destr.walls.reset(); const okB = !!w && quiet(() => blast(g, w));
  const B2 = drain('clock', () => { if (!asked && inHand()) { for (const x of ws) if (x !== w && quiet(() => blast(g, x))) { w2 = x; asked = true; break; } } }), stB = stray(C);
  // (c) the disc turns while the sun's mask is in hand (the cloud's work, or a put-back's): the mask at the end is the new disc's, whole
  g.destr.walls.reset(); let turned = false, discOk = false, wide = 0;
  if (moon) { const r0 = moon.radius; moon.radius = 0.25; KC.sun(moon, 0.25); wide = bitsOf(KC.dirOf(moon)).length; const okC = !!w && quiet(() => blast(g, w));
    drain('clock', () => { if (!turned && inHand()) { moon.radius = 0.02; turned = true; } });
    discOk = okC && turned && bitsOf(KC.dirOf(moon)) === bitsOf(KC.wholeOf(moon)); moon.radius = r0; KC.dir(moon); }
  // (d) A SUN THAT A HOLE LETS IN. A partition's hole changes no directional mask (the roof still stands between a room and the sky), so the three above
  // hold the lamps' work and only the SHAPE of a directional job. Here a wide, low sun shines at an outer wall and a gap is cut in that wall by hand (the
  // boxes edited in place, the keeper told the rect): its mask must change, on the count be the whole derivation's OVER ITS DISC, and with a second gap
  // cut while the job is in hand on the clock be the whole derivation's again (the job reads the statics as they stood when it began: it must begin again).
  g.destr.walls.reset(); let sunLine = 'no outer wall found to cut';
  let dOk = false; { const st = C.engine.world.statics as any[], inr = g.reachInterior as Uint8Array, cellOf = (x: number, z: number) => Math.floor(z - WORLD.min[2]) * WORLD.gridW + Math.floor(x - WORLD.min[0]);
    const tall = (b: any) => !b.rot && !b.yaw && b.h[0] < 0.2 && b.c[1] + b.h[1] > 2.4 && b.c[1] - b.h[1] < 0.3 && b.h[2] > 1.4;
    const east = st.filter(b => tall(b) && inr[cellOf(b.c[0] - 0.8, b.c[2])] && !inr[cellOf(b.c[0] + 0.8, b.c[2])]).sort((a, b) => b.h[2] - a.h[2])[0];
    const west = st.filter(b => tall(b) && inr[cellOf(b.c[0] + 0.8, b.c[2])] && !inr[cellOf(b.c[0] - 0.8, b.c[2])]).sort((a, b) => b.h[2] - a.h[2])[0];
    const wall = east ?? west, into = east ? -1 : 1;   // (which way the light must travel to shine into the building through this wall)
    if (wall) { const c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0, n0 = st.length; let pieces: [number, number][] = [[z0, z1]];
      const lay = (): void => { st.length = n0; pieces.forEach((pc, i) => { const c = [wall.c[0], wall.c[1], (pc[0] + pc[1]) / 2], h = [wall.h[0], wall.h[1], (pc[1] - pc[0]) / 2]; if (i === 0) { wall.c = c; wall.h = h; } else st.push({ ...wall, c, h }); }); };
      const G = 0.4, cut = (zc: number): void => { pieces = pieces.flatMap(pc => (zc > pc[0] + G + 0.1 && zc < pc[1] - G - 0.1 ? [[pc[0], zc - G], [zc + G, pc[1]]] as [number, number][] : [pc])); lay(); KC.changed([wall.c[0] - 0.2, zc - G, wall.c[0] + 0.2, zc + G]); };   // (a gap 0.8 m wide, floor to the wall's top)
      const mend = (): void => { pieces = [[z0, z1]]; lay(); KC.changed(null); };
      const len = Math.hypot(1, 0.2), sun: any = { name: 'check sun', group: 'check', kind: 2, pos: [0, 0, 0], dir: [into / len, -0.2 / len, 0], color: [1, 1, 1], intensity: 1, range: 1e6, innerDeg: 0, outerDeg: 0, radius: 0.25, volumetric: 0, owner: 0, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: 1 };
      C.engine.lights.lights.push(sun); KC.sun(sun, 0.25); const m0 = bitsOf(KC.dirOf(sun));
      cut(c0 - h0 / 2); drain('count'); const m1 = bitsOf(KC.dirOf(sun)), whole1 = bitsOf(KC.wholeOf(sun));
      mend(); const backTo0 = bitsOf(KC.dirOf(sun)) === m0;
      let second = false; cut(c0 - h0 / 2); const D = drain('clock', () => { if (!second && (KC.peek().queue as any[]).some(q => q.l === sun && q.dirJob)) { cut(c0 + h0 / 2); second = true; } });
      const m2 = bitsOf(KC.dirOf(sun)), whole2 = bitsOf(KC.wholeOf(sun)), stD = stray(C);
      mend(); C.engine.lights.lights.splice(C.engine.lights.lights.indexOf(sun), 1);
      dOk = m1 !== m0 && m1 === whole1 && backTo0 && second && m2 === whole2 && m2 !== m1 && stD.raster && stD.lamps === 0;
      sunLine = `a gap in the ${east ? 'east' : 'west'} wall lets the sun in: ${m1 !== m0} · on the count the whole derivation's over its disc: ${m1 === whole1} · the wall mended, the first mask again: ${backTo0} · a second gap with the job in hand (${D.ticks} steps): cut ${second}, the whole derivation's: ${m2 === whole2}, more than one gap's: ${m2 !== m1}`; } }
  check(!!w && again && okU && okM && mT !== before && mT === mU && mM === mU && T.ticks > 1 && T.carried > 0 && U.carried === 0 && stA.lamps === 0 && stA.dirs === 0
      && okB && asked && !!w2 && B2.carried > 0 && stB.raster && stB.lamps === 0 && stB.dirs === 0 && !!moon && wide > 0 && discOk && dOk,
    'a step on the clock ends in the same masks as a step on the count (the clock scripted, three looks a step): the same wall blown and decided on the clock, on the count and with the two mixed, a directional light\'s mask carried across steps on the clock and never on the count; a second wall blown while that mask is in hand, and the end is a whole derivation\'s; the sun\'s disc turned while its mask is in hand, and the mask at the end is the new disc\'s; a wide low sun let in through a gap cut in an outer wall is the whole derivation\'s over its disc on the count, and again with a second gap cut while its job is in hand on the clock',
    `clock ${T.ticks} steps (${T.carried} with a directional mask in hand) · count ${U.ticks} · mixed ${M.ticks}: masks equal ${mT === mU} and ${mM === mU} · a second wall mid-job: asked ${asked}, ${B2.ticks} steps, ${stB.lamps} lamps and ${stB.dirs} directional differ from a whole derivation · the disc turned mid-job: ${turned}, the new disc's mask whole: ${discOk} · ${sunLine}`); }

console.log(fails ? `=== reachkeep-check: ${fails} FAILED ===` : '=== reachkeep-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
