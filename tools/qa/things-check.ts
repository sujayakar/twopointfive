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
  const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
  const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
  const { WALLS } = await import(`${ROOT}/src/game/wallbreak.ts`);
  return { WALLS, standUp, parseLevelData, loadLevel, DESTRUCTION, FURNBLAST, T, F, WALLREACT_STATS, BoxFlag, WORLD };
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

  console.log(`=== things: ${ok ? 'ALL PASS' : 'FAIL'} ===`);
  return ok;
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  if (args.includes('--census')) { await thingsCensus(args.filter(a => !a.startsWith('--'))); process.exit(0); }
  process.exit(await thingsCheck() ? 0 : 1);
}
