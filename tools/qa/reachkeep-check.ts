// THE LAMPS' REACH MASKS ARE THE PICTURE'S TO KEEP (src/render/reachKeep.ts). Until 11-05 the masks, their wall raster and the queue that decides them again
// after a wall changes lived in the Game and were worked in its tick. This holds the move:
//   1  a stand with no picture has no keeper, and a game there does NO reach work at all: not at boot, not on a blast, not in the frames after it
//   2  the game is the same game with a keeper or without one (its digest after a blast), and the keeper's work moves no row of the state's hash
//   3  a snapshot put back with nobody telling the keeper: it reads the walls again by itself and every mask is the one a whole derivation gives on the
//      walls as they then stand, a directional light's too
//   4  the walls as authored again (the wall engine's reset): the masks the level booted with, at once, with nothing left waiting
//   5  a wide sun's mask is of its disc: asked twice for the same disc it is made once, and where the disc is another (the weather turned, a snapshot put
//      the old one back) the next step makes the mask again with nobody telling it
//   bun run tools/qa/reachkeep-check.ts          exit 0 = all pass · 1 = a line failed
//   REACHKEEP_BREAK=1 = line 1 is asked of a stand WITH a keeper, in line 3 the keeper never sees a put-back, and in line 5 it takes no step after the
//   disc is made small: all three must FAIL
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
  for (const l of W.engine.lights.lights as any[]) if (l.kind === 2 && K.dirOf(l)) { n++; const inc = K.dirOf(l) as Uint32Array; K.dir(l); const whole = K.dirOf(l) as Uint32Array; if (inc.some((v, i) => v !== whole[i])) dbad++; }
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

console.log(fails ? `=== reachkeep-check: ${fails} FAILED ===` : '=== reachkeep-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
