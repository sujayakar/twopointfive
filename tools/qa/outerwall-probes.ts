// The 'outerwall' probes (WALLCHARGE.outer — src/game/wallcharge.ts outerWall, src/game/wallbreak.ts adopt: a wall charge on a building's outer wall), their own program:
//   bun run tools/qa/outerwall-probes.ts [all | sites | squad | sam | hard | rooms | reset | live] [--scn block-four,compound-four,warehouse-four] [--verbose]
// Headless = the Proxy device: nothing is drawn. Each family runs on every scenario of --scn. The probes raise the destruction's switch, the wall charge's and
// WALLCHARGE.outer themselves, at the page's chaos 0.5.
// 2026-11-07: charges on outer walls are a settled rule. The lines that asked the build without it went: the families 'inside' (a partition's run with the rule on = with it
// off, digest for digest) and 'off' (the outer wall refused in the old words), the sites' line with the rule off, the rooms' partition laid beside its off run, and
// the note of a checkpoint loaded with the rule off. Sam alone no longer has his own charges written off: he stands as the page stands him.
//   (sites)   the level's outer walls as the module reads them: structure with the open sky over one face (a building's shell) or both (a yard wall), which of them
//             have a place a squad can work from outside; nothing is the engine's until a charge goes (the engine's own list is the partitions', on or off);
//   (squad)   the order from OUTSIDE by the partition's own presses (X, 5, 3): a man sets it, the rest stand off, nothing until Sam's F; then a hole a man walks
//             through, the squad unhurt, every man in through it, the order ends 'in';
//   (sam)     with no squad: the Space row at the foot of an outer wall reads 'wall · outer wall', setSecs on it, set off by his own action from out of its
//             blast, and he walks in through the hole;
//   (hard)    an outer wall is opened by the charge set on it and by nothing else: rounds and a charge-sized blast with no charge on it leave it whole and
//             un-adopted; after the breach, more of the same widen nothing;
//   (live)    both sides awake through a breach: nothing throws, nobody ends in a wall, the change goes out on the engine's feed;
//   (reset)   a restart after a breach: the wall is whole and structure again (the engine has let it go); a checkpoint taken after a breach, loaded: the hole is there.
import { menuKey } from './squadwords';
import { standUp, ROOT } from './headless.ts';
import { samBlows, samBreach } from './charge-rig.ts';
import { listen } from './heard.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { swept } from './sweep.ts';
const WC = await import(`${ROOT}/src/game/wallcharge.ts`);
const { WALLCHARGE, WALLCHARGE_DEFAULTS, wallAt, wallPt, wallOff, outerWall, skyFaces, WALL_ORDER } = WC;
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
/** how an order ended: the game files an ended order on its `done` list as a record of its own, so the row a check held from before is not the one that says so */
const endOf = (C: any, op: any): string | undefined => op.ended ?? [...(C.done ?? [])].reverse().find((d: any) => d.t0 === op.t0 && d.wall?.index === op.wall?.index)?.ended;

type Vec3 = [number, number, number];
const DT = 1 / 60;
const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const VERBOSE = argv.includes('--verbose');
const SCNS = (flag('--scn') ?? 'block-four,compound-four,warehouse-four').split(',');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
type Verdict = (ok: boolean, label: string, detail: string) => void;
let CHECKS = 0, FAILS = 0;
function family(title: string): { verdict: Verdict; note: (s: string) => void; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; CHECKS++; if (!ok) FAILS++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, note: s => console.log(`  (note) ${s}`),
    done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const DT_DEFAULTS = JSON.parse(JSON.stringify(DOORTOOLS));
/** the wall charge's table as the stand has it (the page's values: the module's own defaults are the build before each settled row, and laying them back was a refused write a row) */
const WALLCHARGE_STOOD = JSON.parse(JSON.stringify(WALLCHARGE));
const restoreKnobs = () => { Object.assign(WALLCHARGE, JSON.parse(JSON.stringify(WALLCHARGE_STOOD))); Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); };
const entryOf = (id: string): any => SCENARIOS.find((e: any) => e.id === id);
const levelOfFile = (file: string): any => loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (VERBOSE) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };

/** One game on a scenario's level (the wall charge probes' rig). `squad`: the scenario started — false: the level with Sam alone. `still`: the other side deaf,
 *  blind and held where it stands. */
async function rig(scn: string, seed: number, o: { squad?: boolean; still?: boolean; chaos?: number } = {}) {
  restoreKnobs(); Math.random = seeded(0x9137 + seed * 7919); DESTRUCTION.on = true; WALLCHARGE.on = true;
  const E = entryOf(scn); if (!E) throw new Error(`no scenario '${scn}'`);
  const W: any = await quiet(() => standUp(E.level ? { level: levelOfFile(E.level), dress: true } : { dress: true })   /* (the wall's pieces are the debris pool's, a dressing row: asked for) */); const game = W.game, input = W.input;
  // (until 2026-11-07 the rig wrote OWNBLOW.on off here, for lines that press the squad's key on a charge set by hand: a settled rule, the write was refused)
  game.godMode = true; game.tune.chaos = o.chaos ?? 0.5;
  if (o.squad ?? true) { const { why } = await quiet(() => startFromQuery(game, new URLSearchParams(scenarioQuery(E)))); if (why) throw new Error(`start refused — ${why}`); }
  const ears = listen(game);   // (what the log showed and what the men said, read from the game's own state each step: heard.ts. Until 2026-11-07 two wraps on the tools' copy, which the Rust tick never calls)
  const O = game.orders, D = game.doorTools, C = game.wallCharge;
  // (A MAN ALONE: the door tools' bag is put as a man alone had it, and nothing is carried. Until 2026-11-07 his own charges' switch was written off here, a settled
  // row: the write was refused, so these lines have run on the page's side since the fold.)
  if (!(o.squad ?? true)) { D.alone(DOORTOOLS.charges); C.setCarried(null); }
  const still = () => { for (const gd of game.guards) if (gd.team !== 'operative') { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; if (gd.flashlight) gd.flashlight.enabled = false; } };
  const R = { W, game, input, O, D, C, scn,
    /** what the log has shown and what the men have said, read now (heard.ts) */
    get said(): string[] { ears.poll(); return ears.said; }, get spoke(): { who: string; text: string; radio: boolean }[] { ears.poll(); return ears.spoke; },
    get team(): any[] { return O.squad(); }, get up(): any[] { return O.squad().filter((m: any) => m.state !== 'dead'); },
    get hostiles(): any[] { return game.guards.filter((x: any) => x.team !== 'operative' && x.team !== 'civilian'); },
    /** what the rig does before every step (handed to charge-rig.ts, which steps the stand itself) */
    each() { ears.poll(); game.player.visibility = 0.05; if (o.still ?? true) still(); },
    step(n = 1) { for (let i = 0; i < n; i++) { R.each(); W.step(DT); ears.poll(); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    press(code: string) { input.pressed.add(code); R.step(); },
    pick(p: number[], ...n: number[]) { W.cursorAt([p[0], p[1], p[2]]); menuKey(O); R.step(); R.step(20); for (const k of n) { input.pressed.add(`Digit${k}`); R.step(); R.step(6); } },
    beats(op: any): string { return (op?.beats ?? []).map((b: any) => `${f2(b.t)} ${b.what}`).join(' | '); },
    free(p: number[]) { return !game.col.nav.isBlocked(p[0], p[2]); },
    pathLen(a: number[], b: number[]): number { const path = game.col.nav.findPath(a, b); if (!path) return Infinity; let L = 0, q = a; for (const w of path) { L += flat(q, w); q = w; } return L + flat(q, b); },
    gather(p: number[], secs = 75) { game.teleportPlayer([p[0], 0, p[2]]); R.step(3); O.give('fallIn', O.targetAt(game.player.char.pos), 'all'); return R.until(() => R.up.every((m: any) => flat(m.char.pos, p) < 6 && O.of(m)?.arrived), secs); },
    /** the cursor held on p while the camera settles on Sam (a long teleport leaves it catching up for a second) */
    aim(p: number[], n = 90) { for (let i = 0; i < n; i++) { W.cursorAt([p[0], p[1], p[2]]); R.step(); } },
    walk(p: number[], secs = 8, there: () => boolean = () => flat(game.player.char.pos, p) <= 0.3) { const I = input;
      for (let i = 0; i < secs * 60 && !there(); i++) { for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k); const pos = game.player.char.pos, dx = p[0] - pos[0], dz = p[2] - pos[2], d = Math.hypot(dx, dz), { fwd, right } = W.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / d, b = (dx * right[0] + dz * right[2]) / d, T = 0.3827;
        if (a > T) I.keys.add('KeyW'); if (a < -T) I.keys.add('KeyS'); if (b > T) I.keys.add('KeyD'); if (b < -T) I.keys.add('KeyA'); R.step(); }
      for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k); R.step(); return there(); },
    /** the shipped stand's kit rule (WALLCHARGE.carried: a wall charge is brought, not paid out of the bag): two brought, so every family here has one to set */
    kit() { if (C.carried !== null) C.setCarried(2); },
    /** one wall's worth has gone since (bag, carried) was read — out of the bag on the frozen stand, off the carried count on the shipped one */
    spent(c0: number, k0: number | null) { return k0 !== null ? C.carried === k0 - 1 && D.charges === c0 : D.charges === c0 - WALLCHARGE.cost; },
    put(gd: any, p: number[]) { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.path = []; gd.pathGoal = null; gd.speed = 0; },
  };
  R.kit(); R.step(20);
  return R;
}
type Rig = Awaited<ReturnType<typeof rig>>;

interface Site { p: Vec3; top: Vec3; W: any; near: Vec3; far: Vec3; walk: number; sky: number; from: 'outside' | 'inside' | 'yard' }
const skyOver = (g: any, p: number[]) => !g.col.solidBlocked([p[0], 1.5, p[2]], [p[0], 12, p[2]]);
/** Places on the level's outer walls a charge can go with room to work: floor on both faces, floor along the near face for the stand-off, no door within 2.2 m —
 *  nearest Sam first by the walk, one a wall. `from`: which side the squad works it from ('outside' = the sky over the near side and a roof over the far one). */
function outerSites(R: Rig, from: Site['from'] | 'any' = 'outside', n = 4, deep = 2.6): Site[] {
  const g = R.game, out: Site[] = [], sam = g.player.char.pos, boxes = g.col.boxes, N = Math.min(g.destr.walls.authored, boxes.length);
  for (let i = 0; i < N; i++) { if (!outerWall(g, i)) continue; const b = boxes[i], alongX = b.h[0] >= b.h[2], L = alongX ? b.h[0] : b.h[2], T = alongX ? b.h[2] : b.h[0]; if (L < 1.6) continue;
    const sky = skyFaces(g, i);
    for (let u = -L + 1.2; u <= L - 1.2 + 1e-6; u += 0.6) for (const side of [1, -1]) {
      const mid: Vec3 = alongX ? [b.c[0] + u, 1.05, b.c[2]] : [b.c[0], 1.05, b.c[2] + u], nrm: Vec3 = alongX ? [0, 0, side] : [side, 0, 0];
      const off = (d: number): Vec3 => [mid[0] + nrm[0] * (d + Math.sign(d) * T), 0, mid[2] + nrm[2] * (d + Math.sign(d) * T)];
      if (![0.6, 1.2, deep].every(d => R.free(off(d))) || ![0.6, 1.2].every(d => R.free(off(-d)))) continue;
      if (g.col.solidBlocked([off(deep)[0], 0.5, off(deep)[2]], [off(0.3)[0], 0.5, off(0.3)[2]])) continue;
      if (g.doors.list.some((d: any) => flat(d.frameCentre, mid) < 2.2)) continue;
      const kind: Site['from'] = sky === 2 ? 'yard' : skyOver(g, off(1.2)) ? 'outside' : 'inside'; if (from !== 'any' && kind !== from) continue;
      const near = off(deep), W = wallAt(g, mid, near); if (!W || !W.ok || !W.outer || W.index !== i) continue;
      if (!R.C.standoffPosts(W, 4)) continue;
      const walk = R.pathLen(sam, near); if (!isFinite(walk)) continue;
      out.push({ p: mid, top: [mid[0], b.c[1] + b.h[1], mid[2]], W, near, far: off(-1.2), walk, sky, from: kind }); break; } }
  const seen = new Set<number>(); return out.sort((a, b) => a.walk - b.walk).filter(x => !seen.has(x.W.index) && !!seen.add(x.W.index)).slice(0, n);
}
/** the engine's partitions with room to work (the wall charge probes' own search, cut down) */
function partition(R: Rig, deep = 2.6): { p: Vec3; W: any; near: Vec3 } | null {
  const g = R.game, sam = g.player.char.pos; let best: { p: Vec3; W: any; near: Vec3; walk: number } | null = null;
  for (const w of g.destr.walls.all()) { if (w.kind !== 'wall') continue; const b = w.box, alongX = b.h[0] >= b.h[2], L = alongX ? b.h[0] : b.h[2]; if (L < 1.6) continue;
    for (let u = -L + 1.2; u <= L - 1.2 + 1e-6; u += 0.6) for (const side of [1, -1]) {
      const mid: Vec3 = alongX ? [b.c[0] + u, 1.05, b.c[2]] : [b.c[0], 1.05, b.c[2] + u], nrm: Vec3 = alongX ? [0, 0, side] : [side, 0, 0], off = (d: number): Vec3 => [mid[0] + nrm[0] * d, 0, mid[2] + nrm[2] * d];
      if (![0.6, 1.2, deep].every(d => R.free(off(d)) && R.free(off(-d))) || g.doors.list.some((d: any) => flat(d.frameCentre, mid) < 2.2)) continue;
      const near = off(deep), W = wallAt(g, mid, near); if (!W || !W.ok || W.outer || W.index !== w.id || !R.C.standoffPosts(W, 4)) continue;
      const walk = R.pathLen(sam, near); if (isFinite(walk) && (!best || walk < best.walk)) best = { p: mid, W, near, walk }; break; } }
  return best;
}
const thickOf = (W: any) => 2 * (W.alongX ? W.box.h[2] : W.box.h[0]);


// ================================================================ squad
async function probeSquad(scn: string): Promise<boolean> {
  const F = family(`outerwall-squad: from outside — set, stand off, Sam's F, a way in, the squad in through it (${scn})`);
  const R = await rig(scn, 5); const { O, D, C, game } = R, col = game.col;
  const P = outerSites(R, 'outside')[0] ?? outerSites(R, 'yard')[0]; if (!P) { F.verdict(false, 'an outer wall with room to work from outside', 'none found'); return F.done(); }
  F.note(`${P.from === 'yard' ? 'a yard wall' : 'a building\'s outer wall, worked from outside'}: wall ${P.W.index} at ${at(P.p)}, ${f2(thickOf(P.W))} m thick`);
  const gathered = R.gather(P.near); if (!gathered) F.note(`the squad did not all reach ${at(P.near)} in time — going on with who is there`);
  const W0 = P.W, T = thickOf(W0), a: Vec3 = wallPt(W0, 0, 1.2), b: Vec3 = wallPt(W0, 0, -(T + 1.2)), before = R.pathLen(a, b);
  for (const h of R.hostiles) if (h.state !== 'dead' && flat(h.char.pos, wallPt(W0, 0, 0)) < DOORTOOLS.stunM + 1) R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  R.step(2);
  const t = O.targetAt(P.p), c0 = D.charges, k0 = C.carried; R.pick(P.top, 5, 3); let op = C.ops[0], by = 'X, 5, 3';
  if (!op) { const why = C.order(P.p, 'all'); op = C.ops[0]; by = `the order given by code (the presses found no wall under the cursor: ${why ?? R.said.slice(-1)[0] ?? ''})`; }
  F.verdict(!!t.wall && t.wall.ok && t.wall.outer && !!op && op.phase === 'place' && op.men.length === R.up.length && by === 'X, 5, 3', 'the order by X, 5, 3 on the outer wall: every man on it, one of them to the wall', `${op ? `${op.men.length} men · ${op.lead.callsign} sets it at ${at(op.spot)} · by ${by}` : `no order — ${R.said.slice(-2).join(' / ')} · ${R.spoke.slice(-1)[0]?.text ?? ''}`}`);
  if (!op) return F.done();
  const W = op.wall, cp: Vec3 = wallPt(W, 0, 0, 1.05);
  const setOk = R.until(() => C.set.length === 1, 45), lead = op.lead;
  const spentOk = R.spent(c0, k0), setAt = at(lead.char.pos), word = R.until(() => R.spoke.some(s => s.who === lead.callsign && s.text === 'charge set.'), 4);   // (wallcharge.ts WALLKILL: his word comes once he is clear of the charge — with the rule off, on the frame it is set and nothing is stepped here)
  F.verdict(setOk && spentOk && word && !game.destr.walls.breakableAt(W.index), 'a man sets it on the wall: the charge out of the bag, \'charge set.\' — and the wall is still structure (nothing is the engine\'s yet)',
    `${lead.callsign} at ${setAt} · charges ${c0} → ${D.charges}${k0 !== null ? ` · carried ${k0} → ${C.carried}` : ''} · engine: ${game.destr.walls.breakableAt(W.index) ? 'ADOPTED EARLY' : 'not its wall yet'}`);
  const armed = R.until(() => op.phase === 'armed', 45);
  const dists = R.up.map((m: any) => ({ m, d: flat(m.char.pos, cp), off: wallOff(W, m.char.pos) }));
  F.verdict(armed && dists.every(x => x.d >= WALLCHARGE.standoff[WALLCHARGE.standoff.length - 1] - 0.35 && x.off > 0), 'the squad stands off on the charge\'s side, two metres or more from it', dists.map(x => `${x.m.callsign} ${f2(x.d)} m`).join(' · '));
  const upBefore = R.up.length, statics0 = col.boxes.length, deb0 = new Set(game.destr.debris.list), ev0 = game.eventSeq ?? 0;
  R.press('KeyF'); const tBlow = game.time;
  const thrown = game.destr.debris.list.filter((c: any) => !deb0.has(c)), flying = thrown.filter((c: any) => Math.hypot(c.v[0], c.v[2]) > 0.3), inward = flying.filter((c: any) => (W.alongX ? c.v[2] : c.v[0]) * W.side < 0);
  const loud = game.events.filter((e: any) => (e.seq ?? 1e9) > ev0 && (e.kind === 'bang' || e.kind === 'door') && flat(e.pos, cp) < 2);
  const hole = op.hole, after = R.pathLen(a, b), q: Vec3 = wallPt(W, hole ? hole.du : 0, -T / 2), q0: Vec3 = [...q] as Vec3, pushed = col.collideCircle(q, 0.3, 0.25, 1.5) ? flat(q, q0) : 0, B = game.destr.walls.breakableAt(W.index);
  F.verdict(!!hole && hole.half * 2 >= 1.2 && after < 4.5 && after <= before && pushed < 0.01 && C.set.length === 0 && C.stats.holes === 1, 'F: a way in — a hole a man walks through, the route between the two faces a few metres, a man\'s circle stands in the gap',
    `hole ${hole ? f2(hole.half * 2) : '—'} m wide, ${hole ? f2(hole.du) : '—'} m along from the charge · route ${f2(before)} → ${f2(after)} m · circle pushed ${f2(pushed)} m · ${R.said.filter(s => /^charge blown/.test(s)).slice(-1)[0] ?? ''}`);
  F.verdict(!!B?.hard && B.open > 0 && B.open < B.cells && col.boxes.length > statics0 && !col.solidBlocked(wallPt(W, hole?.du ?? 0, 0.3, 1.9), wallPt(W, hole?.du ?? 0, -(T + 0.3), 1.9)),
    'the engine has taken the wall on: its pieces stand where it stood, the breach open head-high, the rest of the wall standing', `${B ? `${B.open} of ${B.cells} cells open · hard ${!!B.hard}` : 'NOT ADOPTED'} · statics ${statics0} → ${col.boxes.length}`);
  F.verdict(R.up.length === upBefore && R.up.every((m: any) => m.dazzledUntil <= game.time) && C.stats.backblast === 0, 'the squad stood off is unhurt', `${R.up.length}/${upBefore} up · backblast ${C.stats.backblast}`);
  const men = [...op.men]; const ended = R.until(() => !C.ops.includes(op), WALLCHARGE.enterCap + 6);
  const through = men.map((m: any) => ({ m, x: op.crossed.get(m), off: wallOff(W, m.char.pos), A: O.of(m) }));
  F.verdict(ended && through.every(x => x.m.state === 'dead' || (x.x !== undefined && hole && Math.abs(x.x - hole.du) <= hole.half + 0.3 && x.off < -T)),
    'the squad goes in THROUGH THE HOLE: every man crosses the wall\'s line inside the gap and ends past the far face', `${through.map(x => `${x.m.callsign} crossed at ${x.x === undefined ? '—' : f2(x.x)} m, ${f2(-x.off - T)} m in`).join(' · ')} · ended '${endOf(C, op)}' ${f2(game.time - tBlow)} s after the blast`);
  if (VERBOSE) console.log(`    beats: ${R.beats(op)}`);
  return F.done();
}

// ================================================================ sam
async function probeSam(scn: string): Promise<boolean> {
  const F = family(`outerwall-sam: with no squad — the Space row at the foot of an outer wall, setSecs on it, his own action, and in (${scn})`);
  const R = await rig(scn, 7, { squad: false }); const { D, C, game, input } = R;
  const P = outerSites(R, 'outside')[0] ?? outerSites(R, 'yard')[0]; if (!P) { F.verdict(false, 'an outer wall with floor both sides', 'none found'); return F.done(); }
  const W0 = P.W, T = thickOf(W0), foot = wallPt(W0, 0, 0.55), a: Vec3 = wallPt(W0, 0, 1.2), b: Vec3 = wallPt(W0, 0, -(T + 1.2)), before = R.pathLen(a, b);
  const rowNow = () => game.nearby.find((r: any) => r.kind === 'wall' && r.verb === 'charge'), marker = () => game.interactables.find((r: any) => r.kind === 'wall');
  game.teleportPlayer(foot); R.aim(P.top);
  const row = rowNow(), mk = marker();
  F.verdict(!!row && /^Space {2}breach the wall · uses both charges/.test(row.line2) && row.inReach && (mk?.line1 ?? row.line1) === 'wall · outer wall', 'at its foot: the Space row \'breach the wall · uses both charges\', the wall named \'wall · outer wall\'', `row "${row?.line1} — ${row?.line2}" · ${game.nearby.map((r: any) => `${r.kind}:${r.verb ?? '-'}`).join(' ')}`);
  const c0 = D.charges, k0 = C.carried; game.selectVerb('wall', 'charge'); R.press('Space'); const t0 = game.time;
  const done = R.until(() => C.set.length === 1, DOORTOOLS.setSecs + 1);
  F.verdict(done && Math.abs(game.time - t0 - DOORTOOLS.setSecs) < 0.2 && R.spent(c0, k0) && input.keys.size === 0, 'stood on it for setSecs with no key held: the charge is on the wall, out of the bag', `set after ${f2(game.time - t0)} s (setSecs ${DOORTOOLS.setSecs}) · charges ${D.charges} · "${R.said.slice(-1)[0]}"`);
  if (!done) return F.done();
  R.step(60 * 3); F.verdict(C.set.length === 1 && !C.holeAt(C.set[0].wall) && R.pathLen(a, b) === before && !game.destr.walls.breakableAt(W0.index), 'three seconds on: nothing has happened, the wall is still structure', `set ${C.set.length} · route ${f2(R.pathLen(a, b))} m`);
  const Wc = C.set[0].wall; samBlows(R.W, C.set[0], R.each);   // (his own charge goes off from his own action, out of its blast: the squad's key is not its key — ownblow.ts. Until 2026-11-07 this pressed F, with that rule written off)
  const hole = C.holeAt(Wc), after = R.pathLen(a, b);
  F.verdict(!!hole && hole.half * 2 >= 1.2 && after < 4.5 && C.set.length === 0 && C.ops.length === 0 && !!game.destr.walls.breakableAt(W0.index)?.hard, 'set off by his own action from out of its blast: a hole a man fits through', `hole ${hole ? f2(hole.half * 2) : '—'} m · route ${f2(before)} → ${f2(after)} m · "${R.said.filter(s => /^charge blown/.test(s)).slice(-1)[0]}"`);
  if (hole) { const near = wallPt(Wc, hole.du, 0.9), far = wallPt(Wc, hole.du, -(T + 0.9)); game.teleportPlayer(near); R.step(3); const ok = R.walk(far, 8, () => wallOff(Wc, game.player.char.pos) < -(T + 0.4)), pos = game.player.char.pos;
    F.verdict(ok && wallOff(Wc, pos) < -T, 'Sam walks in through it on his own feet', `from ${at(near)} to ${at(pos)} (${f2(-wallOff(Wc, pos) - T)} m past the far face)`);
    // the breach is still that wall to the module: no second charge where it is open, a place beside it still takes one
    game.teleportPlayer(wallPt(Wc, hole.du, 0.55)); const again = wallAt(game, wallPt(Wc, hole.du, 0, 1.05), game.player.char.pos);
    F.verdict(!!again && again.index === W0.index && again.ok && !!again.outer && C.openAt({ ...again, u: Wc.u + hole.du }), 'the holed wall is still that outer wall to the module, open where the breach is', `wallAt → wall ${again?.index} ok ${again?.ok} outer ${!!again?.outer}`); }
  // from inside, outward (the module asks no more of a wall than which one it is): the same wall kind on another building's wall
  const Q = outerSites(R, 'inside').find(s => s.W.index !== W0.index);
  if (!Q) F.note('no place on another outer wall with room to work from inside on this level — the inside-out blast is not checked here');
  else { D.alone(DOORTOOLS.charges); C.setCarried(null); game.teleportPlayer(wallPt(Q.W, 0, 0.55)); R.aim(Q.top); const ok = game.selectVerb('wall', 'charge'); R.press('Space'); const set = R.until(() => C.set.length === 1, DOORTOOLS.setSecs + 1);
    const Wq = C.set[0]?.wall ?? Q.W; if (C.set[0]) samBlows(R.W, C.set[0], R.each); const h2 = C.holeAt(Wq);
    F.verdict(ok && set && !!h2 && h2.half * 2 >= 1.0 /* (holeAt's width is twice the way through's distance to the gap's nearer end: lived-in furniture at a face — the block's wall 28 — moves the way through off the gap's middle, 1.13 m of the same 1.5 m hole) */, 'from INSIDE, outward: the same row, the same hole (the charge asks which wall, not which face)', `wall ${Q.W.index} at ${at(Q.p)} · row ${ok} · set ${set} · hole ${h2 ? f2(h2.half * 2) : '—'} m`); }
  return F.done();
}

// ================================================================ rooms
/** The room fight's rooms round a breach: a hole in a building's SHELL is a way in, not a merger with the yard — the room behind it keeps its floor and its doors (wallbreak.ts
 *  shellGaps, roomfight.ts roomAt) — while a hole in a partition joins its two rooms exactly as it did. */
async function probeRooms(scn: string): Promise<boolean> {
  const F = family(`outerwall-rooms: a breached shell does not pour the room into the yard (${scn})`);
  const shape = async (which: 'outer' | 'partition') => {
    const R = await rig(scn, 31, { still: false }); const g = R.game, fight = g.fight;
    const site = which === 'outer' ? outerSites(R, 'outside')[0] : partition(R); if (!site || !fight?.roomAt) return null;
    R.step(60); const far = wallPt(site.W, 0, -(thickOf(site.W) + 1.0)), near = wallPt(site.W, 0, 1.0);
    const of = (p: Vec3) => { const r = fight.roomAt(p); return r ? { id: r.id, cells: r.cells.size, doors: r.entries.length } : null; };
    const b = { far: of(far), near: of(near) };
    const br = samBreach(R.W, site.W, R.each); R.step(60);   // (set by Sam's own hands and set off by his own action: a charge pushed onto the tools' copy by hand never reaches the game)
    return { W: site.W.index, hole: !!br.hole && !!R.C.holeAt(br.wall), before: b, after: { far: of(far), near: of(near) } };
  };
  const say = (r: any) => (r ? `${r.cells} cells / ${r.doors} doors` : 'no room');
  const O = await shape('outer');
  if (!O) F.note('no building\'s outer wall with a walk to it on this map, or no room fight: skipped');
  else if (!O.before.far) F.note(`wall ${O.W}: nothing the room fight calls a room behind it — skipped`);
  else {
    const b = O.before.far, a = O.after.far;
    F.verdict(!O.after.near || !a || O.after.near.id !== a.id, 'outside the hole is not that room', `outside: ${say(O.after.near)}${O.after.near ? ` (room ${O.after.near.id})` : ''} · inside: room ${a?.id}`);
  }
  return F.done();
}

// ================================================================ hard
async function probeHard(scn: string): Promise<boolean> {
  const F = family(`outerwall-hard: an outer wall is opened by the charge set on it and by nothing else (${scn})`);
  const R = await rig(scn, 9, { squad: false }); const { C, game } = R, walls = game.destr.walls, col = game.col;
  const P = outerSites(R, 'outside')[0] ?? outerSites(R, 'yard')[0]; if (!P) { F.verdict(false, 'an outer wall', 'none found'); return F.done(); }
  const W = P.W, T = thickOf(W), face: Vec3 = wallPt(W, 0, 0.02, 1.05), n0 = walls.all().length, s0 = col.boxes.length, dir: Vec3 = W.alongX ? [0, 0, -W.side] : [-W.side, 0, 0];
  const rifle = WEAPONS.fn2000 ?? Object.values(WEAPONS)[0];
  for (let k = 0; k < 40; k++) walls.round(PLAYER_ID, [face[0], 0.4 + (k % 8) * 0.2, face[2]], dir, W.index, rifle, 0.5);
  game.destr.blast(face, { who: PLAYER_ID, chaos: false }); R.step(5);
  F.verdict(walls.all().length === n0 && col.boxes.length === s0 && !walls.breakableAt(W.index) && !C.holeAt(W), 'forty rounds into it and a charge-sized blast at its face with no charge set: whole, and still not the engine\'s', `breakables ${n0} → ${walls.all().length} · statics ${s0} → ${col.boxes.length} · hole ${C.holeAt(W) ? 'OPEN' : 'none'}`);
  // the breach, by Sam's own charge
  game.teleportPlayer(wallPt(W, 0, 0.55)); R.aim(P.top); game.selectVerb('wall', 'charge'); R.press('Space'); R.until(() => C.set.length === 1, DOORTOOLS.setSecs + 1);
  const Wc = C.set[0]?.wall ?? W; if (C.set[0]) samBlows(R.W, C.set[0], R.each); R.step(5);
  const B1 = walls.breakableAt(W.index), h1 = C.holeAt(Wc);
  F.verdict(!!B1?.hard && !!h1 && walls.all().length === n0 + 1, 'the charge set on it opens it: one more breakable in the engine\'s list, a hard one', `${B1 ? `${B1.open} of ${B1.cells} cells open` : 'not adopted'} · hole ${h1 ? f2(h1.half * 2) : '—'} m · breakables ${walls.all().length}`);
  if (!B1 || !h1) return F.done();
  // more of the same after it: nothing widens
  const beside: Vec3 = wallPt(Wc, h1.du + h1.half + 0.5, 0.02, 1.05);
  const piece = col.boxes.findIndex((b: any, i: number) => i >= walls.authored && walls.breakableAt(i)?.id === B1.id && Math.abs(b.c[0] - beside[0]) <= b.h[0] + 0.05 && Math.abs(b.c[2] - beside[2]) <= b.h[2] + 0.05);   // (the standing piece of THIS wall the rounds land in: −1 would make the forty rounds no test at all)
  for (let k = 0; k < 40; k++) walls.round(PLAYER_ID, [beside[0], 0.4 + (k % 8) * 0.2, beside[2]], dir, piece, rifle, 0.5);
  game.destr.blast(beside, { who: PLAYER_ID, chaos: false }); game.destr.blast(beside, { who: PLAYER_ID }); R.step(5);
  const B2 = walls.breakableAt(W.index), h2 = C.holeAt(Wc);
  F.verdict(piece >= 0 && !!B2 && B2.open === B1.open && !!h2 && Math.abs(h2.half - h1.half) < 1e-6, 'after the breach, forty rounds and two more blasts beside the hole (one a breaching charge\'s, one a grenade\'s) widen nothing', `piece ${piece} · cells open ${B1.open} → ${B2?.open} · hole ${f2(h1.half * 2)} → ${h2 ? f2(h2.half * 2) : '—'} m`);
  // a second charge beside the first hole opens its own stretch (the wall is still chargeable)
  { const u2 = h1.du + (h1.du >= 0 ? -1 : 1) * (h1.half + 1.6), w2 = wallAt(game, wallPt(Wc, u2, 0, 1.05), wallPt(Wc, u2, 1.5));
    if (!w2 || w2.index !== W.index || !w2.ok) F.note('no room for a second charge further along this wall');
    else { samBreach(R.W, w2, R.each); R.step(5); const B3 = walls.breakableAt(W.index);
      F.verdict(!!B3 && B3.open > B2!.open, 'a second charge set further along the same wall opens its own stretch', `cells open ${B2!.open} → ${B3?.open}`); } }
  return F.done();
}

// ================================================================ reset
async function probeReset(scn: string): Promise<boolean> {
  const F = family(`outerwall-reset: a restart puts the wall back; a checkpoint keeps the breach (${scn})`);
  const R = await rig(scn, 13, { squad: false }); const { D, C, game } = R, walls = game.destr.walls, col = game.col;
  const P = outerSites(R, 'outside')[0] ?? outerSites(R, 'yard')[0]; if (!P) { F.verdict(false, 'an outer wall', 'none found'); return F.done(); }
  const n0 = walls.all().length, s0 = col.boxes.length;
  const breach = () => { game.teleportPlayer(wallPt(P.W, 0, 0.55)); R.aim(P.top); game.selectVerb('wall', 'charge'); R.press('Space'); if (!R.until(() => C.set.length === 1, DOORTOOLS.setSecs + 1)) return null;
    const Wc = C.set[0].wall; samBlows(R.W, C.set[0], R.each); R.step(40); return Wc; };
  const W1 = breach(), open1 = !!W1 && !!C.holeAt(W1);
  game.restartEncounter(); R.step(5); const full = D.charges === DOORTOOLS.charges; R.kit();
  const W2 = breach(), h2 = W2 ? C.holeAt(W2) : null, B2 = walls.breakableAt(P.W.index);
  let s: any = null, err = ''; try { s = R.W.snapshot('outerwall', true); } catch (e) { err = String(e); }
  if (!s) { F.note(`a checkpoint was refused here (${err.split('\n')[0]}) — the load half is not checked on this level`); return F.done(); }
  R.step(20); R.W.restore(s); R.step(5);
  const h3 = W2 ? C.holeAt(W2) : null, B3 = walls.breakableAt(P.W.index);
  F.verdict(!!h2 && !!h3 && Math.abs(h3.half - h2.half) < 1e-6 && !!B3?.hard && B3.open === B2?.open && C.set.length === 0, 'a checkpoint taken after a breach, loaded (a load is a restart first: the wall let go, then taken on again from the file): the hole is there, cell for cell',
    `hole ${h2 ? f2(h2.half * 2) : '—'} → ${h3 ? f2(h3.half * 2) : '—'} m · cells open ${B2?.open} → ${B3?.open} · hard ${!!B3?.hard}`);
  return F.done();
}


const FAMILIES: Record<string, (scn: string) => Promise<boolean>> = { squad: probeSquad, sam: probeSam, hard: probeHard, rooms: probeRooms, reset: probeReset };
export { rig, outerSites, partition };
/** probes.ts's door in (family 'outerwall'): one family on one map, the switches, the knobs and the dice put back as they were found */
export async function outerwallFamily(name: string, scn = 'block-four'): Promise<boolean> {
  const rnd = Math.random, sw = DESTRUCTION.on, wc = WALLCHARGE.on;
  try { return await FAMILIES[name](scn); } finally { restoreKnobs(); Math.random = rnd; DESTRUCTION.on = sw; WALLCHARGE.on = wc; }
}

if (import.meta.main) {
  const names = argv.filter((a, i) => !a.startsWith('--') && argv[i - 1] !== '--scn'); const want = !names.length || names.includes('all') ? Object.keys(FAMILIES) : names;
  for (const n of want) if (!FAMILIES[n]) { console.error(`unknown family '${n}' — ${Object.keys(FAMILIES).join(', ')}`); process.exit(2); }
  const failed: string[] = [];
  for (const n of want) for (const scn of SCNS) { try { if (!(await FAMILIES[n](scn))) failed.push(`${n}(${scn})`); } catch (e) { console.log(`- FAIL — ${n} on ${scn} threw\n    ${e instanceof Error ? e.stack : e}`); FAILS++; failed.push(`${n}(${scn})`); } finally { restoreKnobs(); DESTRUCTION.on = false; } }
  console.log(failed.length ? `\n=== outerwall-probes: FAILED — ${failed.join(', ')} (${FAILS} of ${CHECKS} checks) ===` : `\n=== outerwall-probes: ALL PASS (${CHECKS} checks) ===`);
  process.exit(failed.length ? 1 : 0);
}
