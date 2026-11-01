// ONE INDEX OF EVERY SOLID THING, ONE SET OF RINGS FOR EVERY BURST — the check behind THINGS (src/game/things.ts).
//   bun run tools/qa/things-check.ts [--verbose]        (also `probes.ts things`; it raises the switches it needs itself, so it reads the same on both stands)
//   bun run tools/qa/things-check.ts --census [level.json …]   = the index's counts for a level file (default: every level), nothing checked
// On the pass (levels/pass.json: a walled yard under snow with huts in it — the level the flaw was seen on, everything in its yard untouched by a blast):
//  1  EVERY authored box has a row, and every row a material, a size, a hold and an engine
//  2  WHAT NO ENGINE BREAKS IS NAMED: every row that is structure or kept says why, in one of the index's own words
//  3  the old count: with the switch off the furniture engine lists nothing outdoors; with it on, more things than before and things in the yard
//  4  every thing the index gives the furniture engine is in one of its groups (bar the few that rest on a small thing of the lane's own)
//  5  A BURST IN THE YARD: a hand grenade beside a wooden thing outdoors breaks it — with the switch off it stands
//  6  ONE MODEL: where it stood a line is open that was shut, the walk grid was re-baked, the level's change feed has its row and the defenders' reader took it
//  7  THE RINGS, one size a burst: a hand grenade is 0.6 of a wall charge, a door charge 0.7; the furniture's throw ring is the ring (a thing just inside it
//     comes apart, just outside it stands)
//  8  STONE BREAKS IN THE CONTACT RING ALONE: a concrete thing in the yard stands a grenade a pace off and breaks under one lying against it
//  9  THE SHAKE RING: past the throw ring a wall charge breaks nothing that stands; a small thing resting on something falls off it
// 10  the level put back: after a reset every box is as authored
// 11  the walls read the same table: a grenade that would open a partition opens nothing of it once the shatter ring is drawn in short of it
// 12  EVERY THING BREAKS (THINGS.every; the kit's showroom yard, gwot-yard.json): a heavy thing of steel with fittings (a lorry) loses its fittings to a
//     grenade a metre and a half off and keeps its heavy parts, and loses those too to one lying against them
// 13  …a heavy thing that is not steel or stone (a stall's counter) stands a grenade just outside the shatter ring and breaks just inside it
// 14  …a post that stands free is a thing and goes with what it carries; a post in a wall (a door's jamb) is still the building's
// 15  WHAT STANDS SHOWS THE HIT, EACH STUFF ITS OWN WAY (THINGS.deform, with cut boxes on as the page has them): a grenade beside a steel thing and beside a
//     stone thing that no engine breaks leaves both standing and MARKED, the steel with creases (a dent), the stone with a corner off (a chip) and its
//     grey inside showing; a wooden or a soft thing that held is split or torn; every marked box keeps over four fifths of itself, no floor is
//     marked and a long thing hit mid-length is not cut (a mark lies by the burst or is not made); a checkpoint's slice files the marks, a reset takes
//     them off, the slice re-applied cuts the same planes again, and a row for a box that takes no mark today refuses nothing and cuts nothing
// 16  …A HEAVY THING BREAKS WHERE IT IS HIT (a floor with a piece whose heavy steel parts stand a little apart): a grenade against one heavy part takes that
//     part; the next heavy part of the same piece, past the contact ring and inside the shatter ring, stands and is dented
// 17  …STEEL IS NEVER SAWN: of the pieces a wrecked lorry is thrown as, every steel one of any size is folded or dented — none is a plain block
// DEFORM_BREAK=1 = THINGS.deform is off under lines 15 to 17, which must then fail.
// EVERY_BREAK=1 = THINGS.every is off under lines 12 to 14, which must then fail.
// APART_BREAK=1 = THINGS.apart is off under line 18, which must then fail.
// INSIDE_BREAK=1 = THINGS.inside is off under line 19, which must then fail.
// RUBBLE_BREAK=1 = THINGS.rubble is off under line 20, which must then fail.
// HUNG_BREAK=1 = THINGS.hung is off under line 21, which must then fail.
// THINGS_BREAK=1 = the index is switched off under lines 5 to 9, which must then fail. RINGS_BREAK=1 = the rings alone are switched off under the lines that
// are theirs (the throw ring, the shake ring, the walls' table), which must then fail.   exit 0 = every line holds · 1 = one does not
// Nothing here looks at a picture: never seen on a GPU.
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
type Vec3 = [number, number, number];
const DT = 1 / 60;

async function mods() {
  const { standUp } = await import('./headless.ts');
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { DESTRUCTION, FURNBLAST } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const T = await import(`${ROOT}/src/game/things.ts`);
  const F = await import(`${ROOT}/src/game/furnblast.ts`);
  const { WALLREACT_STATS } = await import(`${ROOT}/src/game/wallreact.ts`);
  const { BoxFlag, makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
  const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
  const { WALLS } = await import(`${ROOT}/src/game/wallbreak.ts`);
  const CUT = await import(`${ROOT}/src/scene/cut.ts`), BT = await import(`${ROOT}/src/game/breakTable.ts`);
  const DB = await import(`${ROOT}/src/game/doorblast.ts`);
  return { CUT, BT, DB, WALLS, makeBox, standUp, parseLevelData, loadLevel, DESTRUCTION, FURNBLAST, T, F, WALLREACT_STATS, BoxFlag, WORLD };
}
type M = Awaited<ReturnType<typeof mods>>;
async function quiet<T>(fn: () => Promise<T> | T): Promise<T> { const l = console.log, w = console.warn, e = console.error; console.log = console.warn = console.error = () => {}; try { return await fn(); } finally { console.log = l; console.warn = w; console.error = e; } }

async function stage(m: M, file: string, on: boolean, rings = on) {
  m.DESTRUCTION.on = true; m.FURNBLAST.on = true;
  for (const k of Object.keys(m.F.FURN_STATS)) (m.F.FURN_STATS as any)[k] = 0;
  // (the level as the page has it on either stand: its dressing and its lived-in rooms in; the switches after the stand-up, which pins them to its own stand)
  const W: any = await quiet(() => m.standUp({ uploadEvery: 1, level: m.loadLevel(m.parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')), { dressing: true, lived: true }), seed: 1 }));
  m.T.THINGS.on = on; m.T.THINGS.rings.on = rings;
  const g = W.game; g.tune.chaos = 0.5; g.godMode = true; g.aiEnabled = false; g.playerInvisible = true;
  for (let f = 0; f < 5; f++) W.step(DT);
  return { W, g };
}
const run = (S: { W: any }, n: number) => { for (let f = 0; f < n; f++) S.W.step(DT); };

export async function thingsCensus(files: string[]): Promise<void> {
  const m = await mods(); const was = [m.T.THINGS.on, m.T.THINGS.rings.on, m.DESTRUCTION.on];
  if (!files.length) files = readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).sort();
  console.log('level · boxes · by engine · what no engine breaks, by why');
  for (const f of files) {
    try { const S = await stage(m, f, true); const C = S.g.destr.things.census(); const inv = S.g.destr.furn.inventory();
      const row = (o: Record<string, number>) => Object.entries(o).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${v}`).join(', ');
      console.log(`${f} · ${C.n} · ${row(C.via)} · furniture groups ${inv.groups}\n    stays: ${row(C.why)}\n    of the things: ${row(C.mat)} · ${row(C.size)} · ${row(C.hold)}`);
    } catch (e) { console.log(`${f} · not stood up (${String((e as Error).message ?? e).slice(0, 80)})`); }
  }
  [m.T.THINGS.on, m.T.THINGS.rings.on, m.DESTRUCTION.on] = was;
}

export async function thingsCheck(): Promise<boolean> {
  const m = await mods(); const was = [m.T.THINGS.on, m.T.THINGS.rings.on, m.DESTRUCTION.on, m.FURNBLAST.on], ring0 = { ...m.T.THINGS.rings };
  try { return await thingsLines(m); } finally { [m.T.THINGS.on, m.T.THINGS.rings.on, m.DESTRUCTION.on, m.FURNBLAST.on] = was; Object.assign(m.T.THINGS.rings, ring0, { on: was[1] }); }   // (whatever a line throws, the next probe of the family finds the switches as they were)
}
async function thingsLines(m: M): Promise<boolean> {
  const BREAK = process.env.THINGS_BREAK === '1', RB = process.env.RINGS_BREAK === '1';
  let ok = true; const line = (pass: boolean, what: string, detail = '') => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); if (!pass) ok = false; };
  console.log('\n=== things: one index of every solid thing, one set of rings for every burst (the pass) ===');
  const FILE = 'pass.json'; const f2 = (x: number) => x.toFixed(2);
  const roofed = (g: any, x: number, z: number) => { const I = g.reachInterior; if (!I) return true; const cx = Math.floor(x - m.WORLD.min[0]), cz = Math.floor(z - m.WORLD.min[2]); return cx >= 0 && cz >= 0 && cx < m.WORLD.gridW && cz < m.WORLD.gridH && I[cz * m.WORLD.gridW + cx] === 1; };
  const outdoors = (g: any, G: any) => !roofed(g, (G.a.x0 + G.a.x1) / 2, (G.a.z0 + G.a.z1) / 2);
  const standing = (g: any, i: number) => !(g.col.boxes[i].flags & m.BoxFlag.NoPrimary);
  const whole = (g: any, G: any) => G.members.every((i: number) => standing(g, i));
  /** a point `d` m off the bounds of `G` on a side the burst has a line from, 0.3 m up; null = no such side */
  const beside = (g: any, G: any, d: number): Vec3 | null => {
    const a = G.a, cx = (a.x0 + a.x1) / 2, cz = (a.z0 + a.z1) / 2, top: Vec3 = [cx, a.y1 + 0.05, cz];
    for (const p of [[a.x0 - d, 0.3, cz], [a.x1 + d, 0.3, cz], [cx, 0.3, a.z0 - d], [cx, 0.3, a.z1 + d]] as Vec3[]) {
      if (p[0] < m.WORLD.min[0] + 0.5 || p[0] > m.WORLD.max[0] - 0.5 || p[2] < m.WORLD.min[2] + 0.5 || p[2] > m.WORLD.max[2] - 0.5 || roofed(g, p[0], p[2])) continue;
      const eye: Vec3 = [p[0], 0.6, p[2]]; if (!g.col.solidBlocked(eye, top)) return p;
    }
    return null;
  };
  const frag = (g: any, at: Vec3) => g.destr.blast(at, { who: 1, radius: 2.5, power: 1 });
  const matOf = (g: any, G: any) => { const T = g.destr.things.all(); let big = G.members[0], v = -1; for (const i of G.members) { const b = g.col.boxes[i], vol = b.h[0] * b.h[1] * b.h[2]; if (T[i].hold !== 'rests' && vol > v) { v = vol; big = i; } } return T[big].mat as string; };

  // ---- 1 to 4: the index itself
  const OFF = await stage(m, FILE, false); const invOff = OFF.g.destr.furn.inventory(); const outOff = invOff.list.filter((G: any) => outdoors(OFF.g, G)).length;
  const ON = await stage(m, FILE, true); const g = ON.g, T: any[] = g.destr.things.all(), inv = g.destr.furn.inventory(), C = g.destr.things.census();
  const MATS = ['wood', 'board', 'metal', 'hard', 'soft', 'stone', 'glass', 'snow'], SIZES = ['structure', 'heavy', 'furniture', 'small'], HOLDS = ['stands', 'rests', 'hangs', 'lies'], VIAS = ['walls', 'cover', 'furniture', 'item', 'glass', 'kept', 'structure', 'unseen'];
  const bad = T.filter(t => !MATS.includes(t.mat) || !SIZES.includes(t.size) || !HOLDS.includes(t.hold) || !VIAS.includes(t.via));
  line(T.length === g.destr.walls.authored && T.every((t, i) => t.i === i) && !bad.length, 'every authored box has a row: a material, a size, a hold and an engine', `${T.length} rows for ${g.destr.walls.authored} authored boxes · by engine: ${Object.entries(C.via).map(([k, v]) => `${k} ${v}`).join(', ')}${bad.length ? ` · ${bad.length} rows with a word the index does not have` : ''}`);
  const WHY = ['lying snow', 'a finish', 'full height', 'overhead', 'a post', 'a window\'s frame', 'the building', 'outside the level', 'carries what stays', 'too heavy to move', 'light', 'the breaker', 'the mission\'s thing'];
  const unnamed = T.filter(t => (t.via === 'structure' || t.via === 'kept') && !WHY.includes(t.why));
  const build = ['lying snow', 'a finish', 'full height', 'overhead', 'a post', 'a window\'s frame', 'the building', 'outside the level'].reduce((s, k) => s + (C.why[k] ?? 0), 0), stays = T.filter(t => t.via === 'structure' || t.via === 'kept').length;
  line(!unnamed.length, 'what no engine breaks is named, every row of it', `${stays} rows: ${Object.entries(C.why).map(([k, v]) => `${k} ${v}`).join(', ')} — the building itself ${build}, things that stay ${stays - build}`);
  const members = new Set<number>(); for (const G of inv.list) for (const i of G.members) members.add(i);
  const outOn = inv.list.filter((G: any) => outdoors(g, G));
  line(outOff === 0 && outOn.length > 0 && inv.members > invOff.members, 'switch off: the furniture engine lists nothing outdoors. On: more things, and things in the yard', `off ${invOff.groups} pieces of ${invOff.members} boxes, ${outOff} outdoors · on ${inv.groups} pieces of ${inv.members} boxes, ${outOn.length} outdoors`);
  const lost = T.filter(t => t.via === 'furniture' && !members.has(t.i));
  line(lost.length <= 4, 'every thing the index gives the furniture engine is in one of its pieces', `${lost.length} are not${lost.length ? ` (${lost.slice(0, 4).map(t => `box ${t.i} ${t.mat} ${t.hold}`).join(', ')})` : ''}`);

  // ---- 5, 6: a burst in the yard
  const pickWood = (gg: any) => gg.destr.furn.inventory().list.filter((G: any) => outdoors(gg, G) && matOf(gg, G) === 'wood' && G.a.y1 >= 0.3).map((G: any) => ({ G, at: beside(gg, G, 0.4) })).find((x: any) => x.at);
  const wood = pickWood(g);
  if (!wood) line(false, 'a wooden thing stands in the yard with open ground beside it', 'none found');
  else {
    const { G, at } = wood as { G: any; at: Vec3 };
    { const S = await stage(m, FILE, false); frag(S.g, at); run(S, 120);
      const same = G.members.every((i: number) => standing(S.g, i));
      line(same, 'switch off: a hand grenade beside the wooden thing in the yard leaves it standing, as it did', `piece ${G.id} (${G.members.length} boxes) ${f2(G.a.x1 - G.a.x0)} × ${f2(G.a.z1 - G.a.z0)} m, top ${f2(G.a.y1)} · burst 0.4 m off it at (${f2(at[0])}, ${f2(at[2])})`); }
    const S = await stage(m, FILE, !BREAK); const gg = S.g;
    const a = G.a, alongX = a.x1 - a.x0 >= a.z1 - a.z0, y = Math.min(0.5, a.y1 * 0.5), cx = (a.x0 + a.x1) / 2, cz = (a.z0 + a.z1) / 2;
    const p: Vec3 = alongX ? [cx, y, a.z0 - 0.3] : [a.x0 - 0.3, y, cz], q: Vec3 = alongX ? [cx, y, a.z1 + 0.3] : [a.x1 + 0.3, y, cz];
    const shut0 = gg.col.solidBlocked(p, q); const nav0 = gg.col.nav.blocked.reduce((s: number, v: number) => s + (v ? 1 : 0), 0); const rows0 = gg.destr.walls.changes.length, asked0 = m.WALLREACT_STATS.rows;
    gg.aiEnabled = true; frag(gg, at); run(S, 120);
    const gone = G.members.filter((i: number) => !standing(gg, i)).length, shut1 = gg.col.solidBlocked(p, q), nav1 = gg.col.nav.blocked.reduce((s: number, v: number) => s + (v ? 1 : 0), 0);
    const rows = gg.destr.walls.changes.slice(rows0).filter((c: any) => c.kind === 'cover' && c.cause === 'blast'), mine = rows.find((c: any) => c.id === G.id);
    const fin = gg.destr.debris.list.every((c: any) => [...c.box.c, ...c.v, ...c.w].every(Number.isFinite));
    line(gone > 0 && fin && gg.destr.debris.list.length > 0, 'switch on: the same grenade breaks it into pieces', `${gone} of its ${G.members.length} boxes gone · ${gg.destr.debris.list.length} pieces on the ground, every number finite ${fin} · ${gg.destr.furn.summary()}`);
    line(shut0 && !shut1 && !!mine && mine.sight === true && m.WALLREACT_STATS.rows > asked0 && nav1 <= nav0, 'one model: the line across it is open, the walk grid re-baked, the change feed has its row and the defenders\' reader took it', `line shut before ${shut0}, after ${shut1} · walk grid ${nav0} → ${nav1} blocked cells · feed rows ${rows.length}${mine ? ` (piece ${mine.id}: sight ${mine.sight}, ${mine.opened.length} cells opened)` : ' — none for this piece'} · rows the defenders' reader took ${m.WALLREACT_STATS.rows - asked0}`);
  }

  // ---- 7: the rings
  { const K = m.T.THINGS.rings, sFrag = m.T.burstSize({ radius: 2.5, power: 1 }), sWall = m.T.burstSize({ chaos: false }), sDoor = m.T.burstSize({ chaos: false, radius: 1.0 }), Q = m.T.ringsOf(sFrag);
    line(Math.abs(sFrag - K.frag * 2.5 / m.WALLS.blast.radius) < 1e-9 && Math.abs(sWall - 1) < 1e-9 && Math.abs(sDoor - K.door) < 1e-9 && Q.contact < Q.shatter && Q.shatter < Q.throw && Q.throw < Q.shake && Q.shake < Q.mark, 'one size a burst, its breaking radius over a wall charge\'s: a hand grenade, a door charge and a wall charge, five rings each', `sizes: grenade ${f2(sFrag)}, door charge ${f2(sDoor)}, wall charge ${f2(sWall)} · a grenade's rings: contact ${f2(Q.contact)}, shatter ${f2(Q.shatter)}, throw ${f2(Q.throw)}, shake ${f2(Q.shake)}, mark ${f2(Q.mark)} m`);
    const tryAt = async (d: number) => { const S = await stage(m, FILE, !BREAK, !BREAK && !RB); const w = pickWood(S.g) as any; if (!w) return null; const at = beside(S.g, w.G, d); if (!at) return null; frag(S.g, at); run(S, 60); const TT: any[] = S.g.destr.things.all(), body = w.G.members.filter((i: number) => TT[i].hold !== 'rests');   // (what stands, not what lies on it: past the throw ring the snow on a log is the shake ring's)
      return { gone: body.filter((i: number) => !standing(S.g, i)).length, n: body.length }; };
    const inside = await tryAt(Q.throw - 0.3), outside = await tryAt(Q.throw + 0.3);
    line(!!inside && !!outside && inside.gone > 0 && outside.gone === 0, 'the furniture\'s reach IS the throw ring: a thing just inside it comes apart, just outside it stands', inside && outside ? `a grenade ${f2(Q.throw - 0.3)} m off: ${inside.gone} of the ${inside.n} boxes that stand gone · ${f2(Q.throw + 0.3)} m off: ${outside.gone} gone` : 'no open ground at those distances'); }

  // ---- 8: stone
  { const pick = (gg: any, d: number) => gg.destr.furn.inventory().list.filter((G: any) => outdoors(gg, G) && matOf(gg, G) === 'stone').map((G: any) => ({ G, at: beside(gg, G, d) })).find((x: any) => x.at) as { G: any; at: Vec3 } | undefined;
    const far = m.T.ringsOf(m.T.burstSize({ radius: 2.5, power: 1 })).contact + 0.4; const A = await stage(m, FILE, !BREAK), a = pick(A.g, far); const B = await stage(m, FILE, !BREAK), b = pick(B.g, 0.15);
    if (!a || !b) line(false, 'a concrete thing stands in the yard with open ground beside it', 'none found');
    else { frag(A.g, a.at); run(A, 60); const TA: any[] = A.g.destr.things.all(), held = a.G.members.filter((i: number) => TA[i].hold !== 'rests').every((i: number) => standing(A.g, i)), heldN = m.F.FURN_STATS.held; frag(B.g, b.at); run(B, 60); const gone = b.G.members.filter((i: number) => !standing(B.g, i)).length;
      line(held && heldN > 0 && gone > 0, 'stone breaks in the contact ring alone: it stands a grenade just outside it and breaks under one lying against it', `piece ${a.G.id}: a grenade ${f2(far)} m off, all of it that stands is whole ${held} (held by the stone rule ${heldN}) · piece ${b.G.id}: a grenade 0.15 m off, ${gone} of ${b.G.members.length} boxes gone`); } }

  // ---- 9: the shake ring
  { const S = await stage(m, FILE, !BREAK, !BREAK && !RB), gg = S.g, TT: any[] = gg.destr.things.all(), Q = m.T.ringsOf(1);
    let hit: { G: any; at: Vec3; small: number[] } | null = null;
    for (const G of gg.destr.furn.inventory().list) {
      const lit = (i: number) => { const b = gg.col.boxes[i]; return b.emissive[0] + b.emissive[1] + b.emissive[2] > 0 || gg.targets.some((t: any) => t.boxes.includes(i)); };
      const small = G.members.filter((i: number) => TT[i].size === 'small' && TT[i].hold === 'rests' && !gg.destr.itemOf(i) && !lit(i) && !G.members.some((j: number) => TT[j].on === i)); if (!small.length) continue;
      for (const d of [Q.throw + 0.6, Q.throw + 1.2]) { const at = beside(gg, G, d); if (at) { hit = { G, at, small }; break; } }
      if (hit) break;
    }
    if (!hit) line(BREAK ? false : true, 'the shake ring: no small thing rests on a thing with open ground a shake ring away on this level (nothing asked)', 'none found');
    else { gg.destr.furnBlast(hit.at, 'wall', 1); run(S, 60);
      // (a lamp on a piece the inner rings BROKE goes out with it, as a desk lamp always did: asked of the pieces past the throw ring alone)
      const off = (a: any) => Math.hypot(Math.max(a.x0 - hit!.at[0], 0, hit!.at[0] - a.x1), Math.max(a.z0 - hit!.at[2], 0, hit!.at[2] - a.z1)), isLit = (i: number) => { const b = gg.col.boxes[i]; return b.emissive[0] + b.emissive[1] + b.emissive[2] > 0 || gg.targets.some((t: any) => t.boxes.includes(i)); };
      const lamps = gg.destr.furn.inventory().list.filter((G: any) => off(G.a) > Q.throw).flatMap((G: any) => G.members).filter((i: number) => isLit(i) && !standing(gg, i)).length;
      const fell = hit.small.filter(i => !standing(gg, i)).length, rest = hit.G.members.filter((i: number) => !hit!.small.includes(i)), stood = rest.filter((i: number) => standing(gg, i)).length;
      line(fell > 0 && stood === rest.length && lamps === 0, 'the shake ring: past the throw ring a wall charge breaks nothing that stands, a small thing resting on it falls off, and no light goes out', `piece ${hit.G.id}: ${fell} of ${hit.small.length} small resting things fell · ${stood} of its ${rest.length} other boxes stand · shaken ${m.F.FURN_STATS.shaken} · lights' boxes taken past the throw ring ${lamps}`); } }

  // ---- 11: the walls read the same table
  { const S = await stage(m, FILE, !BREAK, !BREAK && !RB), gg = S.g, K = m.T.THINGS.rings, sh0 = K.shatter, ct0 = K.contact; let found: { at: Vec3; n: number; d: number } | null = null;
    for (const t of gg.destr.things.all() as any[]) { if (t.via !== 'walls') continue; const a = t.a, cx = (a.x0 + a.x1) / 2, cz = (a.z0 + a.z1) / 2;
      for (const d of [1.3, 1.0]) { for (const at of [[a.x0 - d, 0.3, cz], [a.x1 + d, 0.3, cz], [cx, 0.3, a.z0 - d], [cx, 0.3, a.z1 + d]] as Vec3[]) { if (gg.col.nav.isBlocked(at[0], at[2])) continue; const n = gg.destr.wallsFor(at, { radius: 2.5, power: 1 }).length; if (n > 0) { found = { at, n, d }; break; } } if (found) break; }
      if (found) break; }
    if (!found) line(false, 'a partition a grenade a pace off would open stands on this level', 'none found');
    else { let short = -1; try { K.shatter = 0.5; K.contact = 0.3; short = gg.destr.wallsFor([found.at[0] + 1e-4, found.at[1], found.at[2]], { radius: 2.5, power: 1 }).length; } finally { K.shatter = sh0; K.contact = ct0; }
      line(found.n > 0 && short === 0, 'the walls read the same table: a grenade that would open a partition opens nothing of it once the shatter ring is drawn in short of it', `a grenade ${f2(found.d)} m off a partition: ${found.n} opening(s) by the table as it is (shatter ${f2(m.T.ringsOf(m.T.burstSize({ radius: 2.5, power: 1 })).shatter)} m) · ${short} with the shatter ring at ${f2(0.5 * m.T.burstSize({ radius: 2.5, power: 1 }))} m`); } }

  // ---- 10: reset
  { const S = await stage(m, FILE, true), gg = S.g; const flags = () => gg.col.boxes.slice(0, gg.destr.walls.authored).map((b: any) => b.flags).join(','); const f0 = flags();
    const w = pickWood(gg) as any; if (w) { frag(gg, w.at); run(S, 30); } const f1 = flags(); gg.destr.reset(); const f2_ = flags();
    line(f1 !== f0 && f2_ === f0, 'the level put back: after a reset every box is as authored', `changed by the burst ${f1 !== f0} · as authored after the reset ${f2_ === f0}`); }

  // ---- 12 to 14: every thing breaks (the kit's showroom yard: a lorry, a stall, door frames)
  { const YARD = 'gwot-yard.json', EB = process.env.EVERY_BREAK === '1', every0 = m.T.THINGS.every; m.T.THINGS.every = !EB;
    try {
      const up = (gg: any, ids: number[]) => ids.filter(i => !(gg.col.boxes[i].flags & m.BoxFlag.NoPrimary)).length;
      // (a point `d` m off a box of rows `a`, on the first side with open floor: no body's footprint within 0.25 m of it)
      const off = (gg: any, a: any, d: number): Vec3 | null => { const all = gg.destr.things.all().filter((t: any) => t.via !== 'unseen' && !t.ghost && t.a.y1 > 0.3 && t.a.y0 < 1.5);
        for (const p of [[(a.x0 + a.x1) / 2, 0.3, a.z0 - d], [(a.x0 + a.x1) / 2, 0.3, a.z1 + d], [a.x0 - d, 0.3, (a.z0 + a.z1) / 2], [a.x1 + d, 0.3, (a.z0 + a.z1) / 2]] as Vec3[])
          if (!all.some((t: any) => p[0] > t.a.x0 - 0.25 && p[0] < t.a.x1 + 0.25 && p[2] > t.a.z0 - 0.25 && p[2] < t.a.z1 + 0.25)) return p;
        return null; };
      const pick = (gg: any, hard: boolean) => gg.destr.furn.inventory().list.find((G: any) => G.core && G.core.hard === hard && (!hard || G.core.parts.length < G.members.length));
      // 12: steel
      { const A = await stage(m, YARD, true), GA = pick(A.g, true), Q = m.T.ringsOf(m.T.burstSize({ radius: 2.5 }));
        if (!GA) line(false, 'a heavy thing of steel loses its fittings to a grenade a metre and a half off and keeps its heavy parts; lying against them, it loses those too', 'no piece with a heavy steel core and fittings in the yard');
        else { const light = GA.members.filter((i: number) => !GA.core.parts.includes(i)), far = off(A.g, GA.core.a, 1.5), near = off(A.g, GA.core.a, 0.3);
          const l0 = up(A.g, light), h0 = up(A.g, GA.core.heavy); if (far) frag(A.g, far); run(A, 30); const l1 = up(A.g, light), h1 = up(A.g, GA.core.heavy);
          const B = await stage(m, YARD, true), GB = pick(B.g, true); if (near) frag(B.g, near); run(B, 30); const h2 = up(B.g, GB.core.heavy);
          line(!!far && !!near && l1 < l0 && h1 === h0 && h2 < h0, 'a heavy thing of steel loses its fittings to a grenade a metre and a half off and keeps its heavy parts; lying against them, it loses those too', `piece ${GA.id}: ${GA.members.length} parts, ${GA.core.heavy.length} heavy, ${GA.core.parts.length} in its core · the contact ring ${f2(Q.contact)} m · a metre and a half off: fittings standing ${l0} → ${l1}, heavy parts ${h0} → ${h1} · against it: heavy parts ${h0} → ${h2}`); } }
      // 13: not steel
      { const A = await stage(m, YARD, true), GA = pick(A.g, false), Q = m.T.ringsOf(m.T.burstSize({ radius: 2.5 }));
        if (!GA) line(false, 'a heavy thing that is not steel or stone stands a grenade just outside the shatter ring and breaks just inside it', 'no such piece in the yard');
        else { const out = off(A.g, GA.core.a, Q.shatter + 0.25), inn = off(A.g, GA.core.a, Q.shatter - 0.25); const h0 = up(A.g, GA.core.heavy);
          if (out) frag(A.g, out); run(A, 30); const h1 = up(A.g, GA.core.heavy);
          const B = await stage(m, YARD, true), GB = pick(B.g, false); if (inn) frag(B.g, inn); run(B, 30); const h2 = up(B.g, GB.core.heavy);
          line(!!out && !!inn && h1 === h0 && h2 < h0, 'a heavy thing that is not steel or stone stands a grenade just outside the shatter ring and breaks just inside it', `piece ${GA.id} (${A.g.destr.things.at(GA.core.heavy[0]).mat}) · the shatter ring ${f2(Q.shatter)} m · just outside: heavy parts ${h0} → ${h1} · just inside: ${h0} → ${h2}`); } }
      // 14: posts
      { const A = await stage(m, YARD, true), all = A.g.destr.things.all(), K = m.T.THINGS, post = (t: any) => { const b = A.g.col.boxes[t.i]; return Math.max(t.a.x1 - t.a.x0, t.a.z1 - t.a.z0) <= K.post.thick && t.a.y1 - t.a.y0 >= K.post.tall && t.a.y1 <= K.maxTop && !t.ghost && !(b.flags & m.BoxFlag.NoPrimary); };
        const free = all.filter((t: any) => post(t) && t.via === 'furniture'), kept = all.filter((t: any) => post(t) && t.via === 'structure');
        const jamb = kept.filter((t: any) => t.why === 'a post' || t.why === 'carries what stays').length;
        const p = free[0], at = p ? off(A.g, p.a, 0.4) : null; if (at) frag(A.g, at); run(A, 30); const gone = p ? !!(A.g.col.boxes[p.i].flags & m.BoxFlag.NoPrimary) : false;
        line(free.length > 0 && jamb > 0 && jamb === kept.length && gone, 'a post that stands free is a thing and goes under a grenade beside it; a post in a wall is still the building\'s', `posts no taller than a thing: ${free.length} free, ${kept.length} kept (${jamb} of them a post in a wall or carrying what stays) · the free post ${p ? p.i : 'none'} gone ${gone}`); }
    } finally { m.T.THINGS.every = every0; }
  }
  // ---- 15 to 17: what stands shows the hit; a heavy thing breaks where it is hit; steel is never sawn
  { const YARD = 'gwot-yard.json', DB = process.env.DEFORM_BREAK === '1', K = m.T.THINGS, d0 = K.deform.on, e0 = K.every, c0 = m.CUT.cutConfig.on; K.deform.on = !DB; K.every = true; m.CUT.cutConfig.on = true;
    try {
      const gone = (gg: any, i: number) => !!(gg.col.boxes[i].flags & m.BoxFlag.NoPrimary);
      // (a clear spot `d` off the thing: beside the middle of a side, or — `ends` — beside a side 0.3 m in from its end, where a long thing takes its mark)
      const off = (gg: any, a: any, d: number, ends = false): Vec3 | null => { const all = gg.destr.things.all().filter((t: any) => t.via !== 'unseen' && !t.ghost && t.a.y1 > 0.3 && t.a.y0 < 1.5);
        const mx = (a.x0 + a.x1) / 2, mz = (a.z0 + a.z1) / 2, e = 0.3, xs = ends ? [Math.min(a.x0 + e, mx), Math.max(a.x1 - e, mx)] : [mx], zs = ends ? [Math.min(a.z0 + e, mz), Math.max(a.z1 - e, mz)] : [mz];
        for (const p of [...xs.flatMap(x => [[x, 0.3, a.z0 - d], [x, 0.3, a.z1 + d]]), ...zs.flatMap(z => [[a.x0 - d, 0.3, z], [a.x1 + d, 0.3, z]])] as Vec3[])
          if (!all.some((t: any) => p[0] > t.a.x0 - 0.25 && p[0] < t.a.x1 + 0.25 && p[2] > t.a.z0 - 0.25 && p[2] < t.a.z1 + 0.25)) return p;
        return null; };
      // 15: the marks
      { const A = await stage(m, YARD, true), g = A.g, all = g.destr.things.all(), sc = g.destr.scars; const got: Record<string, string> = {};
        const big = (t: any) => { const h = g.col.boxes[t.i].h; return Math.min(h[0], h[1], h[2]) * 2 >= 0.2; };
        for (const kind of ['dent', 'chip'] as const) for (const t of all.filter((x: any) => sc.kindOf(x) === kind && x.via === 'structure' && !x.ghost && !g.col.boxes[x.i].rot && big(x) && (x.why === 'full height' || x.why === 'the building')).slice(0, 8)) {
          if (got[kind]) break; const p = off(g, t.a, 0.5, true); if (!p) continue; frag(g, p); run(A, 6); const b = g.col.boxes[t.i];
          if (b.cut?.length && !gone(g, t.i)) got[kind] = `#${t.i} (${t.why}) ${b.cut.length} planes${b.cutCore ? ', its inside showing' : ''}`; }
        // (wood and soft stuff break in the shatter ring: the one that HOLDS is a heavy one, a little outside it)
        { const Q = m.T.ringsOf(m.T.burstSize({ radius: 2.5 })), GS = g.destr.furn.inventory().list.find((G: any) => G.core && !G.core.hard);
          for (const ends of [true, false]) { const p = GS && !got.soft ? off(g, GS.core.a, Q.shatter + 0.3, ends) : null; if (!p) continue;   // (by its end, then by its middle: whichever the burst has a line from)
            frag(g, p); run(A, 6); const i = GS.core.heavy.find((j: number) => g.col.boxes[j].cut?.length && !gone(g, j)); if (i !== undefined) got.soft = `#${i} (${all[i].mat}, ${sc.kindOf(all[i])}) ${g.col.boxes[i].cut.length} planes`; } }
        // (how much of a box its cut leaves, on a 12³ grid of its own frame: a mark is a mark, never a third of the thing)
        const kept = (b: any) => { let n = 0, N = 0; for (let x = 0; x < 12; x++) for (let y = 0; y < 12; y++) for (let z = 0; z < 12; z++) { const q = [((x + 0.5) / 6 - 1) * b.h[0], ((y + 0.5) / 6 - 1) * b.h[1], ((z + 0.5) / 6 - 1) * b.h[2]]; N++; if (b.cut.every((c: any) => c.n[0] * q[0] + c.n[1] * q[1] + c.n[2] * q[2] <= c.d)) n++; } return n / N; };
        // (a long thing hit mid-length shows nothing — no edge or corner of it is near the burst — and a floor is never marked)
        const SPAN = K.deform.span, longOne = all.find((x: any) => { const b = g.col.boxes[x.i]; return sc.kindOf(x) && x.via === 'structure' && !x.ghost && !b.rot && !b.cut && Math.max(b.h[0], b.h[2]) > SPAN + 0.6 && Math.min(b.h[0], b.h[1], b.h[2]) * 2 >= 0.2 && x.a.y1 >= 1 && !!off(g, x.a, 0.5); });
        let mid = 'none on this level'; if (longOne) { frag(g, off(g, longOne.a, 0.5)!); run(A, 6); mid = `#${longOne.i} ${g.col.boxes[longOne.i].cut?.length ? 'CUT' : 'not cut'}`; }
        const L = sc.list(), kinds = new Set(L.map((x: any) => x.kind)), least = Math.min(1, ...L.map((x: any) => kept(g.col.boxes[x.i]))), floors = L.filter((x: any) => all[x.i].a.y1 < 0.3 || g.col.boxes[x.i].h[1] * 2 < 0.25).length;
        const bad = L.filter((x: any) => { const b = g.col.boxes[x.i], t = all[x.i]; return gone(g, x.i) && t.via !== 'furniture' || !b.cut || b.cut.length > 8 || kept(b) < 0.8; });
        const chipCore = L.some((x: any) => x.kind === 'chip' && !!g.col.boxes[x.i].cutCore), dentCore = L.some((x: any) => x.kind === 'dent' && !!g.col.boxes[x.i].cutCore);
        const snap = g.destr.furn.snapshot(), sound = g.destr.furn.check(snap) === null, cuts = () => JSON.stringify(sc.list().map((x: any) => [x.i, g.col.boxes[x.i].cut]));
        // (a row for a box that takes no mark today — a floor here; in play a marked wall the wall engine has since taken on — refuses nothing, cuts nothing, and is filed again)
        const floor = all.find((x: any) => x.via === 'structure' && x.a.y1 < 0.3 && !g.col.boxes[x.i].cut), odd = floor && snap ? { ...snap, d: [...(snap.d ?? []), [floor.i, 100, 100, 50]] } : null;
        const c1 = cuts(), n1 = L.length; await quiet(async () => { g.destr.reset(); }); const left = all.filter((t: any) => g.col.boxes[t.i].cut).length;
        const oddOk = !!odd && g.destr.furn.check(odd) === null; await quiet(async () => { g.destr.furn.restore(odd ?? snap); }); const c2 = cuts(), kept2 = g.destr.furn.snapshot()?.d?.length ?? 0, floorCut = !!floor && !!g.col.boxes[floor.i].cut;
        line(!!got.dent && !!got.chip && !!got.soft && (kinds.has('split') || kinds.has('tear')) && !bad.length && floors === 0 && !/CUT/.test(mid) && chipCore && !dentCore && !!snap?.d?.length && sound && left === 0 && c1 === c2 && n1 > 0 && oddOk && !floorCut && kept2 === (odd?.d.length ?? -1),
          'what stands shows the hit, each stuff its own way: steel dented and stone chipped where they stand, wood split or soft stuff torn, each mark by the burst and none over a fifth of its thing, no floor marked and a long thing hit mid-length left alone; a checkpoint files the marks, a reset takes them off and the slice cuts the same planes again, a row for a thing that takes no mark today refusing nothing',
          `steel ${got.dent ?? 'NOT MARKED'} · stone ${got.chip ?? 'NOT MARKED'} · a heavy thing of wood or soft stuff just outside the shatter ring ${got.soft ?? 'NOT MARKED'} · ${n1} things marked, kinds ${[...kinds].join(' ')} · the least left of a marked box ${least.toFixed(2)} · cut too deep or gone ${bad.length} · floors marked ${floors} · a long thing hit mid-length: ${mid} · stone shows its inside ${chipCore}, steel its own paint ${!dentCore} · the slice files ${snap?.d?.length ?? 0} marks, sound ${sound} · after a reset ${left} boxes cut · re-applied the same ${c1 === c2} · with a floor's row added: accepted ${oddOk}, the floor cut ${floorCut}, filed again ${kept2}`); }
      // 16: where it is hit (the first of these floors with a piece whose heavy steel parts stand a little apart: one against the grenade, one past its contact ring and well inside its throw ring)
      { let done = false;
        for (const file of ['office.json', 'registry.json', 'warehouse.json', 'mousehole.json', 'tut-road.json']) {
          const A = await stage(m, file, true), g = A.g, T = g.destr.things.all(), Q = m.T.ringsOf(m.T.burstSize({ radius: 2.5 }));
          const dist = (p: number[], a: any) => Math.hypot(Math.max(a.x0 - p[0], 0, p[0] - a.x1), Math.max(a.z0 - p[2], 0, p[2] - a.z1));
          let best: { p: Vec3; h: number; far: number; o: number; id: number } | null = null;
          for (const G of g.destr.furn.inventory().list) { if (!G.core?.hard || G.core.heavy.length < 2) continue; const hv = G.core.heavy.map((i: number) => T[i]);
            for (const h of hv) { const p = off(g, h.a, 0.25); if (!p) continue;
              for (const q of hv) { const d = q === h ? 0 : dist(p, q.a); if (d > Q.contact + 0.3 && d < Q.shatter && !G.core.parts.some((k: number) => k !== q.i && T[q.i].on === k && k === h.i) && (!best || d < best.o)) best = { p, h: h.i, far: q.i, o: d, id: G.id }; } } }
          if (!best) continue; done = true; frag(g, best.p); run(A, 30); const fb = g.col.boxes[best.far];
          line(gone(g, best.h) && !gone(g, best.far) && !!fb.cut?.length, 'a heavy thing breaks where it is hit: a grenade against one heavy steel part takes it, and the next heavy part of the same piece, past the contact ring, stands and is dented',
            `${file} piece ${best.id}: the grenade 0.25 m off part #${best.h}, ${f2(best.o)} m from part #${best.far} (the contact ring ${f2(Q.contact)} m, the shatter ring ${f2(Q.shatter)} m) · the near part gone ${gone(g, best.h)} · the next part gone ${gone(g, best.far)}, its dent ${fb.cut?.length ?? 0} planes`);
          break; }
        if (!done) line(false, 'a heavy thing breaks where it is hit: a grenade against one heavy steel part takes it, and the next heavy part of the same piece, past the contact ring, stands and is dented', 'no piece with two heavy steel parts a little apart on any of the floors tried'); }
      // 17: steel is never sawn
      { const A = await stage(m, YARD, true), g = A.g, GA = g.destr.furn.inventory().list.find((G: any) => G.core && G.core.hard && G.core.parts.length < G.members.length), near = GA ? off(g, GA.core.a, 0.3) : null;
        const s0 = m.BT.BREAK_STATS.sheets + m.BT.BREAK_STATS.bent; if (near) frag(g, near); run(A, 4);
        const steel = g.destr.debris.list.filter((c: any) => c.mat?.mat === 'metal' && Math.max(c.box.h[0], c.box.h[1], c.box.h[2]) >= 0.2), plain = steel.filter((c: any) => !c.box.cut), shaped = m.BT.BREAK_STATS.sheets + m.BT.BREAK_STATS.bent - s0;
        line(!!near && steel.length >= 4 && plain.length === 0 && shaped >= steel.length, 'steel is never sawn: of the pieces a wrecked lorry is thrown as, every steel one of any size is folded or dented',
          `piece ${GA?.id}: ${steel.length} steel pieces 0.4 m or longer on the floor, ${plain.length} of them plain blocks · ${shaped} folded or dented by the table`); }
    } finally { K.deform.on = d0; K.every = e0; m.CUT.cutConfig.on = c0; }
  }

  // 18: broken pieces lie apart (THINGS.apart): equal slabs laid through each other rest with their tops at different heights — pool pieces and fallen doors
  { const YARD = 'gwot-yard.json', AB = process.env.APART_BREAK === '1', K = m.T.THINGS, a0 = K.apart.on;
    try {
      const lay = async (apart: boolean) => { K.apart.on = apart; const A = await stage(m, YARD, true), g = A.g, D = g.destr.debris, p = g.player.char.pos;
        const drop = (n: number, x: number, z: number, t: number) => { const own: any[] = []; for (let i = 0; i < n; i++) { const c = D.spawnRaw('chunk', m.makeBox({ c: [x + (i % 3) * 0.05, 0.3, z + (i % 4) * 0.05], h: [0.45, t, 0.1], yaw: i * 0.5, albedo: [0.5, 0.1, 0.1] }), [0, 0, 0], [0, 0, 0]); if (c) own.push(c); } return own; };
        const own = drop(12, p[0] + 1.2, p[2], 0.02);
        // two doors of one thickness laid flat on one spot, as a checkpoint lays a door that was blown off, and pieces as thick as a door on the floor beside them
        const doors = g.doors.list.filter((d: any) => d.thick === g.doors.list[0].thick).slice(0, 2), T = doors[0].thick, bits = drop(11, p[0] + 1.2, p[2] + 1.2, T / 2);   // (on the bare floor beside: a piece whose middle is over a door comes to rest on the door)
        for (const d of doors) m.DB.slabRestore(d, { x: p[0] + 3, z: p[2], a: 0.3, side: 1, who: -1 });
        run(A, 240); const ys = own.map(c => c.box.c[1]), rest = own.filter(c => c.rest).length + bits.filter(c => c.rest).length, flat = own.filter(c => Math.abs(c.box.c[1] - ys[0]) < 0.02).length;
        let near = Infinity; for (let i = 0; i < 11; i++) for (let j = i + 1; j < 11; j++) near = Math.min(near, Math.abs(ys[i] - ys[j]));   // (the first eleven: one whole turn of the levels)
        const top = (b: any) => b.c[1] + b.h[1], dt = doors.map((d: any) => top(d.box)); let bitNear = Infinity; for (const c of bits) for (const y of dt) bitNear = Math.min(bitNear, Math.abs(top(c.box) - y));
        return { n: own.length + bits.length, rest, flat, near, span: Math.max(...ys) - Math.min(...ys), same: Math.abs(ys[0] - ys[11]), doors: doors.length, doorGap: Math.abs(dt[0] - dt[1]), bitNear, doorUp: Math.max(...dt) - T }; };
      const off = await lay(false), on = await lay(!AB), S = K.apart.step, N = K.apart.levels, mm = (x: number) => (x * 1000).toFixed(2);
      line(off.n === 23 && off.rest === 23 && off.span < 1e-6 && off.doors === 2 && off.doorGap < 1e-6, 'the case: twelve slabs of one thickness dropped through each other all rest with their tops in one plane, and so do two doors laid on one spot, while pieces do not lie apart',
        `${off.rest} of ${off.n} pieces at rest · the twelve ${mm(off.span)} mm apart in height at most · the two doors' tops ${mm(off.doorGap)} mm apart`);
      line(on.n === 23 && on.rest === 23 && on.flat === 12 && on.near >= S * 0.99 && on.span <= S * (N - 1) + 1e-6 && on.same < 1e-6, 'broken pieces lie apart: the same twelve rest flat, no two of eleven in a row nearer than a step in height, none more than the top level up',
        `${on.rest} of ${on.n} at rest, ${on.flat} of the twelve flat on the floor · the nearest two ${mm(on.near)} mm apart (a step ${mm(S)} mm) · lowest to highest ${mm(on.span)} mm (the top level ${mm(S * (N - 1))} mm) · the first and the twelfth share a level, ${mm(on.same)} mm apart`);
      line(on.doors === 2 && on.doorGap >= S * 0.99 && on.bitNear >= S * 0.49 && on.bitNear < S && on.doorUp <= 0.005 + S * N, 'fallen doors lie apart too: two doors of one thickness laid on one spot have their tops a step apart, and no piece as thick as a door lying on the same floor shares a top with either',
        `the two doors' tops ${mm(on.doorGap)} mm apart · the nearest of eleven door-thick pieces to a door's top ${mm(on.bitNear)} mm (half a step ${mm(S / 2)} mm) · the higher door's underside ${mm(on.doorUp)} mm over the floor`);
    } finally { K.apart.on = a0; }
  }

  // 19: what is loose stays in the level (THINGS.inside): thrown at the world's edge, a pool piece, a prop and a door off its hinges all end inside it
  { const YARD = 'gwot-yard.json', IB = process.env.INSIDE_BREAK === '1', K = m.T.THINGS, i0 = K.inside.on, W = m.WORLD, mid = [(W.min[0] + W.max[0]) / 2, 0, (W.min[2] + W.max[2]) / 2];
    try {
      // (each of the four edges: its middle, and the way out)
      const edges: [number, number, number, number][] = [[W.max[0], mid[2], 1, 0], [W.min[0], mid[2], -1, 0], [mid[0], W.max[2], 0, 1], [mid[0], W.min[2], 0, -1]];
      const out = (x: number, z: number, r = 0) => x - r < W.min[0] - 1e-6 || x + r > W.max[0] + 1e-6 || z - r < W.min[2] - 1e-6 || z + r > W.max[2] + 1e-6;
      const fling = async (inside: boolean) => { K.inside.on = inside; const A = await stage(m, YARD, true), g = A.g, D = g.destr.debris;
        const bits: any[] = []; for (const [x, z, dx, dz] of edges) for (let i = 0; i < 3; i++) { const c = D.spawnRaw('chunk', m.makeBox({ c: [x - dx * 0.6 + dz * (i - 1), 3.4, z - dz * 0.6 + dx * (i - 1)], h: [0.2, 0.05, 0.12], yaw: i, albedo: [0.4, 0.4, 0.4] }), [dx * 7, 1, dz * 7], [1, 2, 0]); if (c) bits.push(c); }
        const props = g.props.props.slice(0, 4); props.forEach((q: any, i: number) => { const [x, z, dx, dz] = edges[i]; q.x = x - dx * (q.radius + 0.05); q.z = z - dz * (q.radius + 0.05); q.vx = dx * 5; q.vz = dz * 5; q.w = 0; q.asleep = false; q.idle = 0; q.dirty = true; });
        const doors = g.doors.list.slice(0, 2); doors.forEach((d: any, i: number) => { const [x, z, dx, dz] = edges[i + 2]; m.DB.slabRestore(d, { x: x - dx * 1.4, z: z - dz * 1.4, a: 0.3, side: 1, who: -1 }); d.slab.rest = false; d.slab.v = [dx * 7, 0, dz * 7]; });
        run(A, 420);
        return { n: [bits.length, props.length, doors.length], rest: bits.filter(c => c.rest).length + doors.filter((d: any) => d.slab.rest).length + props.filter((q: any) => q.asleep).length, kept: bits.filter(c => D.list.includes(c)).length,
          bits: bits.filter(c => out(c.box.c[0], c.box.c[2])).length, props: props.filter((q: any) => out(q.x, q.z, q.radius - 1e-3)).length, doors: doors.filter((d: any) => out(d.slab.p[0], d.slab.p[2])).length }; };
      const off = await fling(false), on = await fling(!IB);
      line(off.bits >= 6 && off.props >= 1 && off.doors >= 1, 'the case: thrown at the edge of the world while nothing holds them, pieces, props and doors off their hinges end past it',
        `of ${off.n[0]} pieces ${off.bits} end with their middle past the edge · of ${off.n[1]} props ${off.props} reach past it · of ${off.n[2]} doors ${off.doors} lie with their middle past it`);
      line(on.bits === 0 && on.props === 0 && on.doors === 0 && on.kept === on.n[0] && on.rest === on.n[0] + on.n[1] + on.n[2], 'what is loose stays in the level: the same throws all end inside the edge, every piece still in the pool, everything at rest',
        `past the edge: ${on.bits} of ${on.n[0]} pieces, ${on.props} of ${on.n[1]} props, ${on.doors} of ${on.n[2]} doors · ${on.kept} pieces still in the pool · ${on.rest} of ${on.n[0] + on.n[1] + on.n[2]} at rest`);
    } finally { K.inside.on = i0; }
  }

  // 20: old rubble stays (THINGS.rubble): ONE list for what moves and what lies still. Waves of half the tier's figure are dropped on open ground and left to
  //     come to rest; then a blast goes over the heap; then more waves, to the larger bound
  { const LAB = 'break-lab.json', RB = process.env.RUBBLE_BREAK === '1', K = m.T.THINGS, r0 = K.rubble.on, k0 = K.rubble.keep;
    try {
      const heap = async (keep: boolean) => { K.rubble.on = keep; K.rubble.keep = 3; const A = await stage(m, LAB, true), g = A.g, D = g.destr.debris, cap = D.cap(), half = cap >> 1;
        let peak = 0; const settle = (n: number) => { for (let f = 0; f < n; f++) { A.W.step(DT); peak = Math.max(peak, D.moving); } };
        const wave = (k: number) => { const out: any[] = []; for (let i = 0; i < half; i++) { const c = D.spawnRaw('chunk', m.makeBox({ c: [18.2 + (i % 8) * 0.42 + k * 0.05, 0.3, 8.6 + ((i / 8) | 0) * 0.42 + k * 0.04], h: [0.12, 0.03, 0.08], yaw: i + k, albedo: [0.4, 0.4, 0.4] }), [0, 0, 0], [0, 0.5, 0]); if (c) out.push(c); } settle(150); return out; };
        const first = wave(0); let made = first.length; for (let k = 1; k < 5; k++) made += wave(k).length;
        const lying = D.list.length, firstKept = first.filter(c => D.list.includes(c)).length, atRest = D.list.filter((c: any) => c.rest).length, ev0 = D.stats.evicted, ref0 = D.stats.refused;
        // (a blast over the whole heap: strong enough to wake every piece it reaches)
        const at = [18.0, 0.1, 8.4], woke = D.impulse({ pos: at, radius: 9, strength: 30, up: 0.6, bias: [0, 0, 0] }), flying = D.moving;
        // (…from a corner of it: the pieces it sets going are the ones nearest the blast, not the oldest)
        const dist = (c: any) => Math.hypot(c.box.c[0] - at[0], c.box.c[2] - at[2]), far = Math.max(0, ...D.list.filter((c: any) => !c.rest).map(dist)), near = Math.min(99, ...D.list.filter((c: any) => c.rest).map(dist)), nearest = far <= near + 0.05; settle(360);
        const afterBlast = D.list.length, restAfter = D.list.filter((c: any) => c.rest).length;
        // (…and on to the larger bound: three times the tier's figure here)
        if (keep) for (let k = 5; k < 8; k++) wave(k);
        return { cap, half, made, lying, firstKept, atRest, ev0, ref0, woke, flying, afterBlast, restAfter, nearest, far, near, peak, end: D.list.length, room: D.room(), evicted: D.stats.evicted }; };
      const off = await heap(false), on = await heap(!RB);
      line(off.lying <= off.cap && off.firstKept === 0 && off.ev0 > 0, 'the case: with the rule off the tier\'s figure bounds every piece, so the first pieces down crumble for later ones',
        `${off.made} pieces dropped in waves of ${off.half} · ${off.lying} in the pool of ${off.cap} · of the first wave ${off.firstKept} left · ${off.ev0} crumbled for a slot`);
      line(on.lying === on.made && on.made === 5 * on.half && on.firstKept === on.half && on.atRest === on.lying && on.ev0 === 0 && on.ref0 === 0, 'old rubble stays: the same waves all lie in the one list, the first wave among them, none crumbled and none refused',
        `${on.lying} of ${on.made} in the list (the tier's figure is ${on.cap}) · of the first wave ${on.firstKept} of ${on.half} · ${on.atRest} at rest · crumbled ${on.ev0}, refused ${on.ref0}`);
      line(on.woke > 0 && on.flying > 0 && on.flying <= on.cap && on.peak <= on.cap && on.nearest && on.afterBlast === on.lying && on.restAfter === on.lying, 'a blast at a corner of the heap sets going no more than the tier\'s figure, the nearest pieces and not the oldest, and every piece comes to rest again in the list',
        `a blast moved ${on.woke}: ${on.flying} in motion right after (most at any frame ${on.peak}, the figure ${on.cap}) · the farthest one moved ${on.far.toFixed(2)} m off, the nearest one left ${on.near.toFixed(2)} m · ${on.afterBlast} in the list after, ${on.restAfter} at rest`);
      line(on.end === on.room && on.room === 3 * on.cap && on.evicted === on.lying + 3 * on.half - on.room, 'past the larger bound the oldest piece at rest crumbles for a new one, by the pool\'s one rule',
        `three more waves: ${on.end} in the list of ${on.room} · ${on.evicted} crumbled`);
    } finally { K.rubble.on = r0; K.rubble.keep = k0; }
  }

  // 21: a hung shelf goes with its uprights (THINGS.hung): the racks of block-squads, each a pair of steel uprights with a shelf 13 cm over the floor
  { const BLOCK = 'block-squads.json', HB = process.env.HUNG_BREAK === '1', K = m.T.THINGS, h0 = K.hung.on;
    try {
      const racks = async (hung: boolean) => { K.hung.on = hung; const A = await stage(m, BLOCK, true), g = A.g, st = g.col.boxes, T = g.destr.things;
        // (the low shelves: flat steel, its foot over the floor's touch and under a quarter metre, nothing under it)
        const low = T.all().filter((t: any) => !t.ghost && t.a.y1 - t.a.y0 < K.flatM && t.a.y0 > K.touch && t.a.y0 <= 0.25 && t.a.x1 - t.a.x0 > 1 && t.mat === 'metal');
        const asFurniture = low.filter((t: any) => t.via === 'furniture').length;
        for (const t of low) { g.destr.blast([(t.a.x0 + t.a.x1) / 2, 0.4, t.a.z1 + 0.5], { who: 0, radius: 2.6 }); run(A, 30); g.destr.blast([(t.a.x0 + t.a.x1) / 2, 0.4, t.a.z0 - 0.5], { who: 0, radius: 2.6 }); run(A, 30); }
        const up = (i: number) => !(st[i].flags & m.BoxFlag.NoPrimary);
        // (left in the air: it still stands and no standing box touches it)
        const e = 0.06, hangs = low.filter((t: any) => up(t.i) && !st.some((b: any, j: number) => { if (j === t.i || !up(j) || (b.flags & m.BoxFlag.Dynamic)) return false; const x0 = b.c[0] - b.h[0], x1 = b.c[0] + b.h[0], y0 = b.c[1] - b.h[1], y1 = b.c[1] + b.h[1], z0 = b.c[2] - b.h[2], z1 = b.c[2] + b.h[2]; return x0 <= t.a.x1 + e && x1 >= t.a.x0 - e && y0 <= t.a.y1 + e && y1 >= t.a.y0 - e && z0 <= t.a.z1 + e && z1 >= t.a.z0 - e; })).length;
        return { n: low.length, asFurniture, gone: low.filter((t: any) => !up(t.i)).length, hangs }; };
      const off = await racks(false), on = await racks(!HB);
      line(off.n >= 3 && off.asFurniture === 0 && off.hangs >= 1, 'the case: with the rule off a rack\'s lowest shelf is the building\'s finish, and a blast either side of the rack leaves it in the air',
        `${off.n} low shelves · ${off.asFurniture} are furniture · after two blasts each: ${off.gone} gone, ${off.hangs} left touching nothing`);
      line(on.n === off.n && on.asFurniture === on.n && on.hangs === 0 && on.gone >= 1, 'a hung shelf goes with its uprights: each is furniture, and none is left in the air',
        `${on.n} low shelves · ${on.asFurniture} are furniture · after two blasts each: ${on.gone} gone, ${on.hangs} left touching nothing`);
    } finally { K.hung.on = h0; }
  }

  console.log(`=== things: ${ok ? 'ALL PASS' : 'FAIL'} ===`);
  return ok;
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  if (args.includes('--census')) { await thingsCensus(args.filter(a => !a.startsWith('--'))); process.exit(0); }
  process.exit(await thingsCheck() ? 0 : 1);
}
