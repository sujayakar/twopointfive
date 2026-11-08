// DAYLIGHT AT OPENINGS (the look page's row "Daylight at openings", lookConfig.windows; src/scene/openings.ts; engine.ts lookOpeningLights keeps the lights,
// in a list of its own that the game never reads and no checkpoint holds: engine.ts dayLights, handed to the light upload).
// One rule finds every place a level is open to the sky by, from its boxes as they stand: where floor under a roof meets floor under the sky with no wall
// between, and wall on both hands. Held here to a room built by hand (a wide window, high glass, a porch; an awning, a canopy, glass between two rooms and glass
// out of doors, which are none), to the pass (its windows, a door as it swings, its bay, a hole a real breaching charge blows, the engine's lights as a door
// opens and shuts), to the office's wide and high windows, to two levels with awnings, to the smoke lab's floor, and to how bright one is by the hour.
// No GPU: the picture is never SEEN here.
//   bun run tools/qa/openings-check.ts                    exit 0 = every line passed, 1 = one failed (run it on both stands: TPS_PROFILE=shipped in front)
//   OPENINGS_BREAK=1 bun run tools/qa/openings-check.ts   glass is taken for wall: must FAIL
//   PLANPACE_BREAK=1  …                                    the openings are found again with no pace (whole on one frame, as ?planpace=old): must FAIL
//   MASKCLOCK_BREAK=1 …                                    on the clock the keeper takes no guest (the openings' masks are nobody's): must FAIL
//   MASKOWED_BREAK=1  …                                    the openings are told the walls changed somewhere else: must FAIL
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { placeMask, PLACE_W, PLACE_RES } = await import(`${ROOT}/src/scene/placePlan.ts`);
const { planOpenings, openAreas, openingLooks, LOOK_OPENINGS } = await import(`${ROOT}/src/scene/openings.ts`);
const { WAY } = await import(`${ROOT}/src/scene/wayThrough.ts`);
const { lookConfig } = await import(`${ROOT}/src/render/frame.ts`);
const { lightClass, lightDayK } = await import(`${ROOT}/src/render/photometry.ts`);
const { reachMaskFor, buildWallRaster } = await import(`${ROOT}/src/render/reach.ts`);
const { HOURS } = await import(`${ROOT}/src/game/hours.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
const { Steps } = await import(`${ROOT}/src/render/steps.ts`);
const { ReachKeep } = await import(`${ROOT}/src/render/reachKeep.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const BRK = process.env.OPENINGS_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== openings: where a level is open to the sky${BRK ? ' (glass taken for wall: must fail)' : ''} ===`);
type O = { pos: number[]; dir: number[]; ajar: number; mid: [number, number, number]; half: [number, number]; full: number; radius: number };
const say = (o: O) => `(${o.pos[0].toFixed(2)}, ${o.pos[2].toFixed(2)}) ${o.ajar.toFixed(2)} m²`;
const stand = async (name: string, reach = false) => { const l = console.log, w = console.warn; console.log = console.warn = () => { };
  const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))), seed: 1, reach }); for (let i = 0; i < 10; i++) W.step(1 / 60); console.log = l; console.warn = w;
  const g = W.game, world = g.engine.world; g.godMode = true; g.aiEnabled = false;
  const gaps = () => g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
  const hang = (n: string, a: number) => { const d = g.doors.byName(n); d.latched = a === 0; d.vel = 0; d.angle = a; };
  const shut = () => { for (const d of g.doors.list) hang(d.def.name, 0); };
  const read = (): { m: Uint8Array; all: O[]; O: O[] } => { for (let i = 0; i < 2; i++) W.step(1 / 60); const m = placeMask(world.statics, world.dynamics, undefined, gaps()) as Uint8Array;
    const st = BRK ? world.statics.map((b: any) => (b.flags & BoxFlag.Glass) ? { ...b, flags: b.flags & ~BoxFlag.Glass } : b) : world.statics;
    const all = planOpenings(st) as O[]; openAreas(all, world.dynamics);   // (every opening the walls leave, and what is open of each now)
    return { m, all, O: all.filter(o => o.ajar > 0) }; };   // (what is open at all: what the engine lights)
  return { W, g, world, hang, shut, read }; };
const cellOf = (x: number, z: number) => Math.floor((z - WORLD.rcMin[2]) * PLACE_RES) * PLACE_W + Math.floor((x - WORLD.rcMin[0]) * PLACE_RES);
const roofed = (m: Uint8Array, x: number, z: number) => m[cellOf(x, z)] !== 255 && (m[cellOf(x, z)] & 4) === 0, sky = (m: Uint8Array, x: number, z: number) => m[cellOf(x, z)] !== 255 && (m[cellOf(x, z)] & 4) !== 0;
/** an opening's light hangs under the roof and faces into it, and the open air is behind it */
const inward = (m: Uint8Array, o: O) => roofed(m, o.pos[0] + o.dir[0] * 0.7, o.pos[2] + o.dir[2] * 0.7) && sky(m, o.pos[0] - o.dir[0] * 0.9, o.pos[2] - o.dir[2] * 0.9);

// ---- a hole a breaching charge blows in the store's outer wall: the game's own blast, on the stand the page opens on (the frozen stand holds a charge on an
// outer wall off, so from there this line is asked of a second run of this file on the other stand)
const hole = async () => { const was = DESTRUCTION.on; DESTRUCTION.on = true;
  try { const S = await stand('pass'), { g, W } = S; S.shut(); const before = S.read(), walls = g.destr.walls, boxes = g.col.boxes; let wi = -1;
    for (let i = 0; i < walls.authored; i++) { const b = boxes[i]; if (Math.abs(b.c[0] - 34) < 0.2 && b.h[0] < 0.2 && b.h[2] > 1 && b.h[1] > 1 && Math.abs(b.c[2] - 8) < b.h[2]) wi = i; }
    const took = wi >= 0 && walls.adopt(wi); if (took) g.destr.blast([34.35, 1.0, 8.0], { who: g.player.char.id, chaos: false, breach: wi }); for (let f = 0; f < 6; f++) W.step(1 / 60);
    const after = S.read(), fresh = after.O.filter(o => !before.O.some(w => Math.hypot(w.pos[0] - o.pos[0], w.pos[2] - o.pos[2]) < 0.3)), h = fresh[0];
    check(!!took && fresh.length === 1 && Math.abs(h.pos[0] - 34) < 0.4 && Math.abs(h.pos[2] - 8) < 1 && h.dir[0] === -1 && h.ajar > 1.5 && inward(after.m, h) && before.m[cellOf(34, h.pos[2])] === 255 && after.m[cellOf(34, h.pos[2])] !== 255,
      'a hole a breaching charge blows in the store\'s outer wall is an opening, facing into the store, and the floor plan has the wall open there', `${before.O.length} openings before the charge, ${after.O.length} after · the hole: ${fresh.map(say).join(' · ') || 'none'}`);
  } finally { DESTRUCTION.on = was; } };
if (process.argv.includes('--hole')) { await hole(); process.exit(red ? 1 : 0); }

// ---- a room built by hand: what is an opening and what is not
{ const G = BoxFlag.Glass, box = (c: number[], h: number[], flags = 0) => ({ c, h, yaw: 0, flags }), T = 0.12, B: any[] = [];
  const wallZ = (x: number, z0: number, z1: number) => B.push(box([x, 1.5, (z0 + z1) / 2], [T, 1.5, (z1 - z0) / 2])), wallX = (z: number, x0: number, x1: number) => B.push(box([(x0 + x1) / 2, 1.5, z], [(x1 - x0) / 2, 1.5, T]));
  const roof = (x0: number, z0: number, x1: number, z1: number, y = 3) => B.push(box([(x0 + x1) / 2, y + 0.1, (z0 + z1) / 2], [(x1 - x0) / 2, 0.1, (z1 - z0) / 2]));
  const post = (x: number, z: number, y = 3) => B.push(box([x, y / 2, z], [0.05, y / 2, 0.05]));
  // room A, 6 by 6 m: a window 4 m wide in its far wall (sill, glass, lintel), a strip of high glass in its west wall, a pane in the wall it shares with room C
  wallZ(16, 9.88, 16.12); roof(9.9, 9.9, 16.1, 16.1);
  wallX(16, 9.88, 11); wallX(16, 15, 16.12); B.push(box([13, 0.45, 16], [2, 0.45, T]), box([13, 2.45, 16], [2, 0.55, T]), box([13, 1.4, 16], [2, 0.5, 0.006], G));
  wallZ(10, 9.88, 12); wallZ(10, 14, 16.12); B.push(box([10, 1.1, 13], [T, 1.1, 1]), box([10, 2.85, 13], [T, 0.15, 1]), box([10, 2.45, 13], [0.006, 0.25, 1], G));
  wallX(10, 9.88, 12); wallX(10, 14, 16.12); B.push(box([13, 0.45, 10], [1, 0.45, T]), box([13, 2.45, 10], [1, 0.55, T]), box([13, 1.4, 10], [1, 0.5, 0.006], G));
  wallZ(10, 3.88, 9.88); wallZ(16, 3.88, 9.88); wallX(4, 9.88, 16.12); roof(9.9, 3.9, 16.1, 9.9);   // room C, shut
  // a porch: three walls and a roof, open on its fourth side · an awning on four posts · a canopy on two posts against room A's wall · a pane out of doors
  wallZ(20, 9.88, 14); wallZ(24, 9.88, 14); wallX(10, 19.88, 24.12); roof(19.9, 9.9, 24.1, 14);
  roof(26.75, 17.3, 29.25, 18.7, 2.15); post(26.8, 17.35, 2.15); post(29.2, 17.35, 2.15); post(26.8, 18.65, 2.15); post(29.2, 18.65, 2.15);
  roof(16.1, 11.5, 18.9, 14.5); post(18.85, 11.55); post(18.85, 14.45);
  B.push(box([6, 1.4, 20], [0.5, 0.5, 0.006], G));
  // a shed whose walls stop 10 cm under its roof (a tin roof on battens), one doorway in its far wall
  { const w = (c: number[], h: number[]) => B.push(box(c, h)); w([3, 1.45, 12], [T, 1.45, 2.12]); w([7, 1.45, 12], [T, 1.45, 2.12]); w([5, 1.45, 10], [2.12, 1.45, T]);
    w([3.75, 1.45, 14], [0.87, 1.45, T]); w([6.25, 1.45, 14], [0.87, 1.45, T]); w([5, 2.45, 14], [0.5, 0.45, T]); roof(2.9, 9.9, 7.1, 14.1); }
  const st = BRK ? B.map(b => (b.flags & G) ? { ...b, flags: 0 } : b) : B, O = planOpenings(st) as O[];
  const win = O.filter(o => o.dir[2] === -1 && Math.abs(o.pos[2] - 16) < 0.4 && o.pos[0] > 11 && o.pos[0] < 15), high = O.filter(o => o.dir[0] === 1 && Math.abs(o.pos[0] - 10) < 0.4 && Math.abs(o.pos[2] - 13) < 0.2), porch = O.filter(o => o.dir[2] === -1 && Math.abs(o.pos[2] - 14) < 0.4 && o.pos[0] > 20 && o.pos[0] < 24);
  const sum = (l: O[]) => l.reduce((a, o) => a + o.full, 0);
  check(win.length === 2 && Math.abs(sum(win) - 4.0) < 0.3 && win.every(o => Math.abs(o.pos[1] - 1.4) < 0.06) && Math.abs(win[0].pos[0] - win[1].pos[0]) > 1.5, `a window 4 m wide is an opening however wide, lit as two lights side by side (longer than ${LOOK_OPENINGS.span} m and than twice its height)`, win.map(say).join(' · '));
  check(high.length === 1 && Math.abs(high[0].full - 1.0) < 0.15 && high[0].pos[1] > 2.2, 'a strip of glass over a door\'s height is an opening (the plan here is cut from the floor to the ceiling, not at a man\'s height)', high.map(o => `${say(o)} at ${o.pos[1].toFixed(2)} m up`).join(' · '));
  check(porch.length === 1 && porch[0].full > 9 && porch[0].full < 12, 'a porch\'s open side, between its two walls, is an opening', porch.map(say).join(' · '));
  { const winOf = (extra: any[]) => (planOpenings([...st, ...extra]) as O[]).filter(o => o.dir[2] === -1 && Math.abs(o.pos[2] - 16) < 0.4 && o.pos[0] > 11 && o.pos[0] < 15);
    const cup = winOf([box([12, 1.0, 15.5], [0.5, 1.0, 0.2])]), bags = winOf([box([14.5, 0.7, 16.4], [0.5, 0.7, 0.2])]);
    check(Math.abs(sum(cup) - sum(win)) < 0.01 && Math.abs(sum(win) - sum(bags) - 0.5) < 0.12, 'a cupboard in the room, in front of that window, takes nothing of it (it stands in front of the light and throws its own shadow); sandbags against the wall outside take what they cover',
      `${sum(win).toFixed(2)} m² · with the cupboard ${sum(cup).toFixed(2)} · with the sandbags ${sum(bags).toFixed(2)}`); }
  const shed = O.filter(o => o.pos[0] > 2.5 && o.pos[0] < 7.5 && o.pos[2] > 9.5 && o.pos[2] < 14.5);
  check(shed.length === 1 && shed[0].dir[2] === -1 && Math.abs(shed[0].pos[0] - 5) < 0.3 && Math.abs(shed[0].full - 1.55) < 0.2, 'a shed whose walls stop a hand under its roof is open by its doorway alone (a gap a hand high under the eaves is no opening, and the walls still count as walls on both hands)', shed.map(say).join(' · ') || 'none');
  { const leaf = { c: [5, 0.95, 14], h: [0.38, 0.95, 0.02], yaw: 0, flags: BoxFlag.Dynamic }, o2 = planOpenings(st) as O[]; openAreas(o2, [leaf]); const d = o2.filter(o => o.pos[0] > 2.5 && o.pos[0] < 7.5 && o.pos[2] > 9.5 && o.pos[2] < 14.5);
    check(d.length === 1 && d[0].ajar === 0 && d[0].full > 1.3, 'a shut leaf a hand shorter than its doorway is a shut door (the strip over its head is a slit, as it is in a wall)', d.map(o => `${o.ajar.toFixed(2)} of ${o.full.toFixed(2)} m² open`).join(' · ')); }
  check(O.length === win.length + high.length + porch.length + shed.length, 'and nothing else is: not a stall\'s awning on four posts, a canopy on posts against a wall, glass between two rooms or glass out of doors', O.map(say).join(' · ')); }

// ---- the pass
{ const S = await stand('pass', true), { g } = S; S.shut();   // (… with the picture's keeper of the game's own lamps' masks stood up, render/reachKeep.ts: line 5 holds that the openings' lights are in none of its lists)
  const { m, O, all } = S.read(), glazed = O.filter(o => Math.abs(o.pos[1] - 1.35) < 0.08 && o.full < 2), sides = O.filter(o => !glazed.includes(o));
  // 1. its windows: the four the glass-only rule found, where it hung them (17 cm inside the glass: 5 cm off the wall's inside face)
  { const tea = glazed.filter(o => Math.abs(o.pos[2] - 10.83) < 0.02 && o.pos[0] > 12 && o.pos[0] < 22 && o.dir[2] === -1), kit = glazed.filter(o => Math.abs(o.pos[2] - 10.83) < 0.02 && o.pos[0] > 22 && o.pos[0] < 28 && o.dir[2] === -1);
    check(glazed.length === 4 && sides.length === 3 && sides.every(o => o.dir[0] === -1 && o.pos[0] < 12.2 && o.full > 6) && O.every(o => inward(m, o)) && all.length - O.length === 4 && all.filter(o => !O.includes(o)).every(o => o.ajar === 0 && Math.abs(o.full - 1.7) < 0.1),
      'with every door shut the pass is open to the sky by its four windows and by the open sides of its bay and its lean-to; its four doors to the yard are doorways with nothing open; the lorry\'s bed under its canvas is none', O.map(say).join(' · '));
    check(tea.length === 2 && kit.length === 1 && tea.every(o => o.ajar <= 1.8 * 0.95 + 0.01 && o.ajar > 1.3 && o.ajar === o.full) && kit[0].ajar <= 1.2 * 0.95 + 0.01 && kit[0].ajar > 0.85 && glazed.every(o => 4 * o.half[0] * o.half[1] >= o.full - 1e-6 && 4 * o.half[0] * o.half[1] < o.full * 1.3),
      'the tea room has two and the kitchen one, each measured as what is open of its glass (the frame\'s bars are not), in a rectangle that holds it with little to spare', `tea ${tea.map(o => o.ajar.toFixed(2)).join(' and ')} of 1.71 m² of glass · kitchen ${kit[0]?.ajar.toFixed(2)} of 1.14`); }
  // 2. doors: one at a time, wide open
  { const out: string[] = [], inn: string[] = [], bad: string[] = [];
    for (const d of g.doors.list) { const n = d.def.name; S.shut(); S.hang(n, 1.5); const R = S.read(), fresh = R.O.filter(o => !O.some(w => Math.hypot(w.pos[0] - o.pos[0], w.pos[2] - o.pos[2]) < 0.3));
      const [hx, hz] = d.def.hinge, a = d.def.closedDir, mx = hx + Math.cos(a) * d.def.width * 0.5, mz = hz + Math.sin(a) * d.def.width * 0.5;
      if (fresh.length === 0 && R.O.length === O.length) inn.push(n);
      else if (fresh.length === 1 && R.O.length === O.length + 1 && R.all.length === all.length && Math.hypot(fresh[0].pos[0] - mx, fresh[0].pos[2] - mz) < 0.5 && fresh[0].ajar > 1.4 && fresh[0].ajar < 1.9 && Math.abs(fresh[0].pos[1] - 1.0) < 0.1 && inward(R.m, fresh[0])) out.push(`${n} ${fresh[0].ajar.toFixed(2)}`);
      else bad.push(`${n}: ${R.O.length} openings, new ${fresh.map(say).join(' · ') || 'none'}`); }
    S.shut(); const back = S.read().O;
    check(bad.length === 0 && out.length >= 3 && inn.length >= 1 && back.length === O.length, 'a door to the open air, standing open, is an opening the size of its doorway, facing into its room; a door between two rooms is none; shut, it is none again',
      `to the air: ${out.join(' · ')} m² · between rooms: ${inn.join(', ')}${bad.length ? ` — WRONG: ${bad.join('; ')}` : ''}`);
    // … and as it swings: what is open grows with the leaf from the first hand's width, and the light hangs where it hung
    const name = out[0]?.split(' ')[0] ?? g.doors.list[0].def.name, door = (R: { all: O[] }) => R.all.filter(o => !O.some(w => Math.hypot(w.pos[0] - o.pos[0], w.pos[2] - o.pos[2]) < 0.3) && Math.abs(o.pos[1] - 1.0) < 0.1);
    const at = (a: number) => { S.shut(); S.hang(name, a); const R = S.read(), d = door(R); return { area: d.reduce((s, o) => s + o.ajar, 0), pos: JSON.stringify(R.all.map(o => o.pos)) }; };
    const sw = [0, 0.3, 0.8, 1.2, 1.5].map(at);
    check(sw[0].area === 0 && sw[1].area > 0 && sw[1].area < 0.2 * sw[4].area && sw.every((v, i) => i === 0 || v.area >= sw[i - 1].area - 0.02) && sw[2].area > 0.4 && sw[2].area < sw[4].area - 0.2 && sw[4].area > 1.4 && sw.every(v => v.pos === sw[0].pos),
      `the ${name} door shut is no opening, a hand\'s width ajar it is one by that hand\'s width, it grows as the leaf swings to nearly its whole doorway, and no light moves while it does`, sw.map((v, i) => `${[0, 0.3, 0.8, 1.2, 1.5][i]} rad ${v.area.toFixed(2)}`).join(' · ') + ' m²'); S.shut(); }
  // 3. the light can reach its own room (render/reach.ts: the mask the engine decides for it, from the rectangle's middle and half width on the engine's
  //    own raster of the walls: engine.ts lookOpeningLights)
  { const R = S.read(); let dark = 0, asked = 0; const solid = buildWallRaster(S.world.statics, g.doors.list.map((d: any) => ({ hinge: d.def.hinge, closedDir: d.def.closedDir, width: d.def.width })));
    for (const o of R.all) { const mask = reachMaskFor(solid, o.mid, o.dir, o.half[0], null) as Uint32Array; asked++;
      const cx = Math.floor((o.pos[0] + o.dir[0] * 1.5 - WORLD.min[0]) / WORLD.gridCell), cz = Math.floor((o.pos[2] + o.dir[2] * 1.5 - WORLD.min[2]) / WORLD.gridCell), ci = cz * WORLD.gridW + cx;
      if (((mask[ci >> 4] >>> ((ci & 15) * 2)) & 3) === 0) dark++; }
    check(asked >= 8 && dark === 0, 'each opening\'s light reaches the floor a step inside it by the lamps\' own reach masks (it does not hang inside a wall)', `${asked} asked, ${dark} that reach nothing there`); }
  // 4. pure: the same boxes give the same openings in the same order
  { const a = S.read().all, b = S.read().all; check(JSON.stringify(a) === JSON.stringify(b), 'the same level gives the same openings in the same order'); }
  // 5. the engine's lights (engine.ts lookOpeningLights), under the grey sky. THE PICTURE'S ALONE: they are the engine's own list, and the list the game
  // reads, its reach lists, the budget a new man's torch is counted against and the state a checkpoint holds are as they were with the row off
  // (the harness's engine is a stand-in with the real light list and world: the real method is run on one that borrows both)
  { const { Engine, openMaskOwed } = await import(`${ROOT}/src/engine.ts`), { REACH_PACE, REACH_WORK } = await import(`${ROOT}/src/render/reach.ts`), { spawnUnit } = await import(`${ROOT}/src/game/guards.ts`), { stateHash } = await import(`${ROOT}/src/game/rollback.ts`);
    const LS = g.engine.lights;
    const KEEPS: any[] = [];   // (every borrowed engine's keeper of its own lights' masks: render/reachKeep.ts)
    const mk = () => Object.assign(Object.create(Engine.prototype), { lights: LS, world: g.engine.world, settings: { skyZenith: HOURS.overcast.sky.zenith, skyHorizon: HOURS.overcast.sky.horizon, exposure: HOURS.overcast.exposure }, viewFocus: [20, 0, 10], openRecs: [], dayLights: [], openLevel: null, openSerial: -1, openTick: 0, openSeen: NaN, openPlanned: NaN, openWait: 0, openLeaves: NaN, openWalls: null, openJob: new Steps(), reach: KEEPS[KEEPS.push(new ReachKeep(g.engine.world, LS)) - 1] }) as any;   // (… and a keeper of its own for the masks of its own lights: render/reachKeep.ts)
    const E = mk(), M = (l: any) => { if (l) for (const k of KEEPS) { const m = k.of(l); if (m) return m; } return undefined; };   // (a day light's mask: the engine's keeper holds it, no light carries one)
    const was = { windows: lookConfig.windows, rooms: lookConfig.rooms, eye: lookConfig.eyeAt, gaps: lookConfig.doorGaps, lit: LOOK_OPENINGS.lit };
    const quiet = <T>(f: () => T): T => { const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
    try { lookConfig.windows = 1; lookConfig.rooms = 1;
      let eye: [number, number] = [20, 10]; lookConfig.eyeAt = () => eye;
      lookConfig.doorGaps = () => g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
      // (a drawn frame: the engine's pass alone. The game is NOT stepped between them: behind the start card and on a paused page it is not either)
      const frames = (n: number) => quiet(() => { for (let i = 0; i < n; i++) E.lookOpeningLights(); });
      const day = () => E.dayLights as any[], lit = () => day().filter(l => l.enabled), game = () => (LS.lights as any[]).map(l => l.name).join(','), reachN = () => { const P = (g.engine as any).reach.peek(); return P.lamps.length + P.queue.length; };   // (the game's own lamps' masks are the stand's keeper's: the openings' lights must be in neither of its lists)
      const before = { list: game(), reach: reachN(), steady: (LS.lights as any[]).filter(l => l.ttl < 0).length, hash: quiet(() => stateHash(g)) };
      S.shut(); const s0 = REACH_WORK.steps; frames(1); const first = lit().length, firstMasks = day().filter(l => M(l)).length, s1 = REACH_WORK.steps; let most = 0, n = 1;
      while (day().some(l => !M(l)) && n < 400) { const c = REACH_WORK.steps; frames(1); most = Math.max(most, REACH_WORK.steps - c); n++; }
      const lit0 = lit(), used0 = day().slice(), sAll = REACH_WORK.steps; frames(5); const quietAfter = REACH_WORK.steps === sAll, hash1 = quiet(() => stateHash(g));
      const tea = g.doors.byName('tea'), [hx, hz] = tea.def.hinge, mx = hx + Math.cos(tea.def.closedDir) * tea.def.width * 0.5, mz = hz + Math.sin(tea.def.closedDir) * tea.def.width * 0.5, doorL = used0.find(l => Math.hypot(l.pos[0] - mx, l.pos[2] - mz) < 0.5);
      S.hang('tea', 1.5); quiet(() => S.W.step(1 / 60)); frames(1); const open = !!doorL?.enabled, I = doorL?.intensity ?? 0, mask = M(doorL), range1 = doorL?.range, pos1 = JSON.stringify(doorL?.pos);
      S.hang('tea', 0); quiet(() => S.W.step(1 / 60)); frames(1); const shutOut = !!doorL && !doorL.enabled && doorL.range === range1 && M(doorL) === mask && JSON.stringify(doorL.pos) === pos1 && REACH_WORK.steps === sAll;
      check(used0.length === all.length && lit0.length === O.length && first === O.length && firstMasks === all.length && most <= REACH_PACE.steps * 1.6 && quietAfter && !!doorL && !lit0.includes(doorL) && open && I > 0.3 && !!mask && shutOut,
        `the engine keeps one light an opening: on a level\'s first frame EVERY opening has its reach mask whole, the shut ones too (since 11-05: a door opened on the next frame finds its mask made), and those open are lit; after it none is decided but a frame\'s share of the work at a time (the game\'s own pace for a lamp\'s), with the game standing still as it does behind the start card; a door\'s comes on as it opens and goes out as it shuts, and neither moves it, its range or its mask`,
        `${used0.length} openings · ${first} lit and ${firstMasks} masked on the first frame (${s1 - s0} sight-line steps) · all ${used0.length} masks whole ${n} frames on, ${most} steps in the longest of them (the pace is ${REACH_PACE.steps}) and none after · the tea door open: ${I.toFixed(2)}`);
      // DAYLIGHT COUNTED ONCE (render/lights.ts LIGHT_PORTAL; tools/qa/portal-check.ts asks what the light is): each light is the whole rectangle its
      // walls leave with the sky's reference divided back out, lit under a night's sky too. The only way since 11-04 (the disks and their row are gone)
      { const lumOf = (c: number[]) => 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2], daySky = E.settings, recs = E.openRecs as any[], sunL = (LS.lights as any[]).find(l => l.kind === 2 && l.enabled && !l.transient && l.intensity > 0), refLum = lumOf(openingLooks(daySky.skyZenith, daySky.skyHorizon, lookConfig.sky, sunL));
        frames(1); const P = lit(), masked = day().every(l => !!M(l));
        // a door's leaf: lit from the first hand's width, as bright to a pass whose ray stops short as what is open, the whole rectangle written beside it
        const shareOf = (l: any) => l.portal.share as number, leafAt = (a: number) => { S.hang('tea', a); quiet(() => S.W.step(1 / 60)); frames(1); return { on: !!doorL.enabled, I: doorL.intensity as number, E: doorL.intensity * lumOf(doorL.color), share: doorL.enabled && doorL.portal ? shareOf(doorL) : 0 }; };
        const ajar = leafAt(0.3), half = leafAt(0.7), wide = leafAt(1.5), shut = leafAt(0), oT = recs.find(r => r.l === doorL).o, rectT = 4 * oT.half[0] * oT.half[1];
        const leaf = ajar.on && half.on && wide.on && !shut.on && ajar.I > 0 && ajar.I < 0.2 * wide.I && half.I > ajar.I && half.I < wide.I && [ajar, half, wide].every(x => Math.abs(x.E - refLum * x.share * rectT) < 1e-6) && ajar.share < half.share && half.share < wide.share && wide.share <= 1;
        const shaped = P.every(l => { const o = recs.find(r => r.l === l).o; return !!l.portal && Math.abs(l.portal.halfWide - o.half[0]) < 1e-9 && Math.abs(l.radius - Math.hypot(o.half[0], o.half[1])) < 1e-9 && o.half[0] > 0 && o.half[1] > 0 && Math.abs(l.pos[0] - o.mid[0]) + Math.abs(l.pos[1] - o.mid[1]) + Math.abs(l.pos[2] - o.mid[2]) < 1e-9
          && Math.abs(l.intensity * lumOf(l.color) - refLum * l.portal.share * 4 * o.half[0] * o.half[1]) < 1e-6 && l.portal.share > 0 && l.portal.share <= 1 && 4 * o.half[0] * o.half[1] >= o.full - 1e-6; });
        E.settings = { skyZenith: [0.012, 0.016, 0.03], skyHorizon: [0.02, 0.022, 0.03], exposure: 1.9 }; frames(1); const night = lit().length;
        const sunWas = sunL ? sunL.intensity : 0; if (sunL) sunL.intensity = 0;   // (… and no sun or moon on the ground outside: an opening looks out on that too)
        E.settings = { skyZenith: [0.0004, 0.0005, 0.001], skyHorizon: [0.0006, 0.0007, 0.001], exposure: 1.9 }; frames(1); const black = lit().length; if (sunL) sunL.intensity = sunWas;   // (a sky the picture would show nothing of)
        E.settings = { skyZenith: [0.012, 0.016, 0.03], skyHorizon: [0.02, 0.022, 0.03], exposure: 1.9 };
        E.settings = daySky; frames(1); const back = lit().length;
        check(P.length === O.length && shaped && masked && leaf && night === O.length && black === 0 && back === O.length,
          'each opening is a portal: the rectangle that holds all its walls leave open, as bright as what is open of it with that share of the rectangle written beside it; a door\'s is lit from the first hand\'s width between its leaf and the jamb and grows with the gap, with no step a disk\'s quarter of a square metre makes; lit under a night\'s sky as by day and out only where the picture would show nothing of what it looks out on',
          `${P.length} portals of ${O.length} open · every record a rectangle no smaller than its opening: ${shaped} · every one with its reach mask: ${masked} · the tea door at 0.3, 0.7 and 1.5 rad: ${ajar.I.toFixed(3)}, ${half.I.toFixed(3)}, ${wide.I.toFixed(3)} (a fifth of wide open or less at 0.3, and beside each the share of its rectangle that is: ${leaf}), out when shut: ${!shut.on} · lit under a night's sky: ${night}, ${black} under a black one, ${back} by day again`); }
      // the game cannot tell: its light list, its reach lists and its count of steady lights are what they were, and a man can still be spawned
      { let spawned = false, why = ''; try { quiet(() => { const u = spawnUnit(g, { team: 'hostile', pos: [30, 0, 20], yaw: 0 } as any); spawned = !!u; }); } catch (e) { why = String(e).slice(0, 120); }
        const steady = (LS.lights as any[]).filter(l => l.ttl < 0).length;
        check(hash1 === before.hash && game().startsWith(before.list) && reachN() === before.reach && steady - before.steady <= 2 && !(LS.lights as any[]).some(l => l.group === 'sky' && l.kind === 3) && spawned,
          'the game cannot tell the row is on: its state is the same to the bit with every opening lit and masked (a checkpoint holds none of it), none of these lights is in the list it reads (its light estimates, the fog\'s lamps, the count a new man\'s torch is held to) or in its reach lists, and a man is still spawned', `the state\'s hash ${before.hash} before the first frame and ${hash1} after the last mask · ${before.steady} steady lights in the game\'s list with the row off, ${steady} with it on and a man spawned (his own), ${day().length} in the picture\'s own list${why ? ` — ${why}` : ''}`); }
      // more open than are lit at once: the nearest to Sam, wherever the camera looks
      { LOOK_OPENINGS.lit = 3; const set = () => lit().map(l => `${l.pos[0].toFixed(0)},${l.pos[2].toFixed(0)}`).join(' ');
        eye = [14, 12]; E.viewFocus = [14, 0, 12]; frames(1); const a = set(); E.viewFocus = [34, 0, 8]; frames(1); const b = set(); eye = [34, 8]; frames(1); const c = set(); LOOK_OPENINGS.lit = was.lit; eye = [20, 10]; frames(1);
        check(a.split(' ').length === 3 && a === b && c !== a && c.split(' ').length === 3 && lit().length === O.length, 'when more are open than are lit at once, the nearest to Sam are lit and the camera has no say', `three by Sam at the tea house: ${a} · the camera moved to the store: ${b} · Sam moved there: ${c}`); }
      // a wall changes: found again once it has stood still, each light kept where its opening still is, lit by its old mask until the new one is whole
      { const st = g.engine.world.statics as any[], wall = st.find(b => !b.rot && b.h[0] < 0.2 && b.h[1] > 1.2 && b.h[2] > 1.6 && Math.abs(b.c[0] - 34) < 0.2 && Math.abs(b.c[2] - 8) < b.h[2] - 1.2), keepL = lit0[0], oldMask = M(keepL);
        const c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0, bump = () => { (g.engine.world as any).geomSerial = ((g.engine.world as any).geomSerial ?? 0) + 1; };
        // (a hole 1.5 m wide in the middle of the store's outer wall: the wall is cut short of it and a second piece stands past it)
        wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; st.push({ ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] }); bump();
        // (and what the frames of a blast may cost: no jump field made again where the walls only opened, and no frame over its share of sight-line steps,
        //  the hole's own first mask included. Until 11-05 that mask was decided whole on the frame the walls were read, this hole's 327 thousand steps, and the field made beside it)
        const fields0 = REACH_WORK.fields; let most = 0; const one = () => { const s0 = REACH_WORK.steps; frames(1); most = Math.max(most, REACH_WORK.steps - s0); };
        one(); const waits = day().length === used0.length; for (let i = 0; i < LOOK_OPENINGS.settle + 1; i++) one();
        const holeL = day().find(l => !used0.includes(l)), kept = day().includes(keepL) && keepL.enabled && !!M(keepL), dark = !!holeL && (!holeL.enabled || !!M(holeL)); let k = 0; while (!(holeL?.enabled && M(holeL)) && k++ < 400) one(); const late = k; k = 0; while (day().some(l => !M(l)) && k++ < 400) one(); for (let i = 0; i < 80; i++) one();
        const fieldsMade = REACH_WORK.fields - fields0, noField = fieldsMade === 0, paced = most <= REACH_PACE.steps * 1.6;   // (1.6: a share and the cell that crossed it, as the pace's own line above allows)
        const litHole = !!holeL?.enabled && !!M(holeL), fresh = M(keepL) !== oldMask, at = holeL ? `at (${holeL.pos[0].toFixed(2)}, ${holeL.pos[2].toFixed(2)}) ${holeL.intensity.toFixed(2)}` : 'none';
        st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); frames(LOOK_OPENINGS.settle + 2); const gone = !!holeL && !day().includes(holeL) && day().length === used0.length;
        check(waits && !!holeL && kept && dark && litHole && fresh && gone && noField && paced, `a wall holed is a new opening ${LOOK_OPENINGS.settle} frames after it has stood still (never on the frame of the blast): its light comes on with its mask whole, a few frames on, the others stay lit by their old masks until theirs are decided again; no frame of it spends more than its share of sight-line steps and no jump field is made again; the wall put back, its light is gone`,
          `the hole\'s light ${at} · not on the first frame: ${waits} · a window\'s light kept and lit throughout: ${kept} · its mask decided again: ${fresh} · lit: ${litHole}, ${late} frames after the frames that found it · the most steps in a frame ${most} (a share is ${REACH_PACE.steps}) · jump fields made for the hole ${fieldsMade} · gone with the wall back: ${gone}`); }
      // WHAT IS LIT STAYS LIT, NO JOB IS LOST, AND A FIELD IS NOT HANDED ON PAST A WALL THAT CLOSED (11-05, from a review of the paced masks: each of these
      // was wrong in the first cut of them, and none was asked)
      { const st = g.engine.world.statics as any[], recs = () => E.openRecs as any[], bump = () => { (g.engine.world as any).geomSerial = ((g.engine.world as any).geomSerial ?? 0) + 1; };
        const settle = () => { let k = 0; do frames(1); while (recs().some(r => openMaskOwed(r)) && k++ < 900); frames(LOOK_OPENINGS.settle + 2); k = 0; while (recs().some(r => openMaskOwed(r)) && k++ < 900) frames(1); };
        const wall = st.find(b => !b.rot && b.h[0] < 0.2 && b.h[1] > 1.2 && b.h[2] > 1.6 && Math.abs(b.c[0] - 34) < 0.2 && Math.abs(b.c[2] - 8) < b.h[2] - 1.2), c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0, f: string[] = [], said: string[] = [];
        settle(); const known = new Set(day());
        // a hole, and a static that is no wall nudged every five frames from the frame it is found (a crate shot to bits, a pane gone: the openings are found again each time)
        wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; const second = { ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] }; st.push(second); bump();
        const crate = st.find(b => b !== wall && b !== second && b.h[1] < 0.6 && b.c[1] < 1.2), y0 = crate.c[1]; let flip = false, litAt = -1;
        for (let k = 1; k <= 200 && litAt < 0; k++) { if (k >= 6 && (k - 6) % 5 === 0) { flip = !flip; crate.c[1] = y0 + (flip ? 0.02 : 0); bump(); } frames(1); const h = day().find(l => !known.has(l)); if (h?.enabled && M(h)) litAt = k; }
        crate.c[1] = y0; bump(); settle();
        if (!(litAt > 0 && litAt <= 20)) f.push(`with the openings found again every five frames the hole's light ${litAt < 0 ? 'never came on in 200 frames' : `came on at frame ${litAt}`} (a job in hand is kept where no wall moved)`); else said.push(`found again every five frames, the hole lit at frame ${litAt}`);
        // a second hole beside the first: one opening of the two, its middle 0.75 m off, so a new record. Nothing that was lit goes dark
        const n1 = lit().length, a2 = c0 + 0.75 + 1.5; second.c = [second.c[0], second.c[1], (a2 + z1) / 2]; second.h = [second.h[0], second.h[1], (z1 - a2) / 2]; bump();
        let fewest = n1; for (let k = 0; k < 40; k++) { frames(1); fewest = Math.min(fewest, lit().length); } settle();
        if (fewest < n1) f.push(`a second hole beside the first: ${n1} lit before it, ${fewest} on some frame after (the widened opening takes the old one's mask until its own is whole)`); else said.push(`a hole widened: never fewer than the ${n1} lit`);
        // a box set down in open ground beside it: cells CLOSE, so the field is not handed on, and every mask is the one a fresh raster gives
        const doors = g.doors.list.map((d: any) => ({ hinge: d.def.hinge, closedDir: d.def.closedDir, width: d.def.width })), before = buildWallRaster(st, doors), fields1 = REACH_WORK.fields;
        const box = { ...wall, c: [wall.c[0] - 1.6, wall.c[1], c0], h: [0.5, wall.h[1], 0.5] }; st.push(box); bump(); settle();
        const after = buildWallRaster(st, doors); let closedCells = 0; for (let i = 0; i < after.length; i++) if (after[i] && !before[i]) closedCells++;
        let differ = 0; for (const r of recs()) { const m = reachMaskFor(after, r.o.mid, r.o.dir, r.o.half[0], null) as Uint32Array; if (m.length !== r.mask.length || m.some((v: number, i: number) => v !== r.mask[i])) differ++; }
        if (!(closedCells > 0)) f.push('the box set down closed no cell of the raster (the check asks nothing)'); else if (differ || REACH_WORK.fields === fields1) f.push(`a box set down (${closedCells} sub-cells closed): ${differ} of ${recs().length} masks are not what a fresh raster gives, ${REACH_WORK.fields - fields1} jump fields made`); else said.push(`a box set down closed ${closedCells} sub-cells: a field made, all ${recs().length} masks as a fresh raster gives them`);
        st.pop(); bump(); settle();
        // the wall put back whole, as a restart puts it (the same boxes, in place): nothing lit goes dark on the way, and every mask is a fresh raster's
        const n2 = lit().length; st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); let fewest2 = n2; for (let k = 0; k < 40; k++) { frames(1); fewest2 = Math.min(fewest2, lit().length); } settle();
        const whole = buildWallRaster(st, doors); let differ2 = 0; for (const r of recs()) { const m = reachMaskFor(whole, r.o.mid, r.o.dir, r.o.half[0], null) as Uint32Array; if (m.some((v: number, i: number) => v !== r.mask[i])) differ2++; }
        if (fewest2 < lit().length || differ2) f.push(`the wall put back: ${fewest2} lit on some frame where ${lit().length} are lit after, ${differ2} masks not a fresh raster's`); else said.push(`the wall put back: never fewer lit than the ${lit().length} after, every mask fresh`);
        // the masks' first frame gives EVERY opening its mask, the shut and the far too (a door swung or a step taken on the next frame finds it made)
        { LOOK_OPENINGS.lit = 3; const E3 = mk(); quiet(() => { E3.lookOpeningLights(); E3.lookOpeningLights(); }); const R3 = E3.openRecs as any[], none3 = R3.filter(r => !r.mask).length; LOOK_OPENINGS.lit = was.lit;
          if (!R3.length || none3) f.push(`a level's first frames: ${none3} of ${R3.length} openings with no mask`); else said.push(`a level's first frame: all ${R3.length} masked with three lit`);
          // and a second level on the same engine starts over (its statics are other boxes): whole at once again
          const s0 = st[0]; st[0] = { ...s0 }; quiet(() => { E3.lookOpeningLights(); E3.lookOpeningLights(); }); const R4 = E3.openRecs as any[]; if (!R4.length || R4.some(r => !r.mask)) f.push(`a new level on the same engine: ${R4.filter(r => !r.mask).length} of ${R4.length} openings with no mask after its first frames`); st[0] = s0; }
        // ?reachpace=old (REACH_PACE.whole): a hole's first mask whole at once, as it was
        { REACH_PACE.whole = true; const known2 = new Set(day()); wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; st.push({ ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] }); bump();
          let found = -1, litOn = -1; for (let k = 1; k <= 60 && litOn < 0; k++) { frames(1); const h = day().find(l => !known2.has(l)); if (h && found < 0) found = k; if (h?.enabled && M(h)) litOn = k; }
          REACH_PACE.whole = false; st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); settle();
          if (!(found > 0 && litOn === found)) f.push(`with the old pace asked for, the hole was found on frame ${found} and lit on frame ${litOn} (want the same frame)`); else said.push('the old pace asked for: the hole lit on the frame it is found'); }
        check(f.length === 0, 'what is lit stays lit and no job is lost: a hole\'s light comes on though the openings are found again every five frames; a hole widened, and the wall put back, never show fewer lit; a box set down makes the field fresh and every mask a fresh raster\'s; the first frame masks every opening, on a second level too; the old pace asked for is the old pace', f.join('; ') || said.join(' · ')); }
      // THE OPENINGS ARE FOUND AGAIN A LITTLE A FRAME (render/steps.ts, render/planPace.ts): the plan, the walls' raster and its scan were 9 to 15 ms of one
      // frame in a browser, cold each time a wall changed. Three stops a frame here, which no clock can change. PLANPACE_BREAK=1 gives no pace: must FAIL.
      { const PB = process.env.PLANPACE_BREAK === '1', bit = (): any => PB ? undefined : { left: 3 }, st = g.engine.world.statics as any[], bump = () => { (g.engine.world as any).geomSerial = ((g.engine.world as any).geomSerial ?? 0) + 1; }, f: string[] = [], said: string[] = [];
        const E5 = mk(), recs = () => E5.openRecs as any[], go = (pace = bit()) => quiet(() => E5.lookOpeningLights(pace));
        const sig = (os: any[]) => os.map(o => `${o.pos.map((v: number) => v.toFixed(3)).join(',')}/${o.full.toFixed(4)}`).sort().join(' '), planNow = () => sig(planOpenings(st)), shown = () => sig(recs().map(r => r.o)), litN = () => (E5.dayLights as any[]).filter(l => l.enabled).length;
        const rest = () => { let k = 0; do go(); while ((E5.openJob.busy || E5.openPlanned !== E5.openSeen || recs().some(r => openMaskOwed(r))) && k++ < 3000); return k < 3000; };
        go(); if (!recs().length || shown() !== planNow() || E5.openJob.busy) f.push('a level\'s first frame: its openings were not all found on it');
        if (!rest()) f.push('never at rest (the level as it stands)');
        const wall = st.find(b => !b.rot && b.h[0] < 0.2 && b.h[1] > 1.2 && b.h[2] > 1.6 && Math.abs(b.c[0] - 34) < 0.2 && Math.abs(b.c[2] - 8) < b.h[2] - 1.2), c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0;
        // (a) a hole: found over several frames with the old plan serving and nothing going dark, and what lands is the plan made at once
        const before = shown(), n0 = litN(); wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; const second = { ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] }; st.push(second); bump();
        let began = -1, landed = -1, fewest = n0, neither = 0; for (let k = 1; k <= 600 && landed < 0; k++) { go(); if (began < 0 && E5.openJob.busy) began = k; fewest = Math.min(fewest, litN()); const s = shown(); if (s !== before) { if (s === planNow()) landed = k; else neither++; } }
        const took = landed - began;
        if (landed < 0) f.push('a hole: never found in 600 frames'); else if (!(began > LOOK_OPENINGS.settle && took >= 2)) f.push(`a hole: begun on frame ${began}, landed on frame ${landed} (want it begun once the walls have stood still and landed frames later)`); else said.push(`a hole found over ${took + 1} frames of three stops, from frame ${began}`);
        if (neither) f.push(`${neither} frame(s) showed a plan that is neither the old one nor the new`); if (fewest < n0) f.push(`${n0} lit before the hole, ${fewest} on some frame while it was being found`);
        if (!rest()) f.push('never at rest (the hole)');
        // (b) one begun is finished on the level as it took it, whatever moves after; then found again for what moved
        { const a2 = c0 + 0.75 + 1.5, was = shown(); second.c = [second.c[0], second.c[1], (a2 + z1) / 2]; second.h = [second.h[0], second.h[1], (z1 - a2) / 2]; bump();
          let k = 0; while (!E5.openJob.busy && k++ < 60) go(); go(); const wantA = planNow();
          const a3 = a2 + 1; second.c[2] = (a3 + z1) / 2; second.h[2] = (z1 - a3) / 2; bump(); const wantAB = planNow();   // (the hole a metre wider still, under the finding, the box's own numbers changed in place as the game changes them)
          k = 0; while (shown() === was && k++ < 600) go();
          if (wantA === wantAB) f.push('the hole made wider changed no opening (this part asks nothing)');
          if (shown() !== wantA) f.push('boxes that moved while the openings were being found are in the plan that landed, or it is of neither level');
          if (!rest() || shown() !== wantAB) f.push('the openings were not found again for what moved meanwhile'); else said.push('one begun lands the level as it took it, then the level as it is');
}
        // (c) a static that is no wall, moved every six frames (a crate shot to bits): the openings are still found, where a finding dropped at every change never would be
        { const crate = st.find(b => b !== wall && b !== second && b.h[1] < 0.6 && b.c[1] < 1.2), y0 = crate.c[1], was = shown(); second.c = [second.c[0], second.c[1], (c0 + 0.75 + z1) / 2]; second.h = [second.h[0], second.h[1], (z1 - c0 - 0.75) / 2]; bump();
          let at = -1, flip = false; for (let k = 1; k <= 600 && at < 0; k++) { if (k % 6 === 0) { flip = !flip; crate.c[1] = y0 + (flip ? 0.02 : 0); bump(); } go(); if (shown() !== was && shown() === planNow()) at = k; }
          crate.c[1] = y0; bump();
          if (at < 0) f.push('with a crate moved every six frames the openings were never found again in 600 frames'); else said.push(`a crate moved every six frames: found again by frame ${at}`);
          if (!rest()) f.push('never at rest (the crate still)'); }
        // (d) a finding takes a step every frame whatever is left of the frame's allowance (nothing the other planners do can hold it up), and no more than one
        if (!PB) { const was = shown(); st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); let b0 = -1, at = -1; for (let k = 1; k <= 900 && at < 0; k++) { go(k % 2 ? { left: 0 } : { left: Infinity, ms: 0 }); if (b0 < 0 && E5.openJob.busy) b0 = k; if (shown() !== was) at = k; }
          if (at < 0 || shown() !== planNow()) f.push('with nothing left of any frame\'s allowance the openings were never found again'); else if (at - b0 < 2 * took) f.push(`with nothing left of any frame's allowance a finding took ${at - b0 + 1} frames (three stops a frame took ${took + 1}: a step a frame should take near three times that)`); else said.push(`a step a frame with nothing left: ${at - b0 + 1} frames`);
          if (!rest()) f.push('never at rest (the wall back, a step a frame)');
          // (e) a new level drops what is in hand and has its own openings whole on its first frame; what was in hand never lands on it
          wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; st.push(second); second.c[2] = (c0 + 0.75 + z1) / 2; second.h[2] = (z1 - c0 - 0.75) / 2; bump();
          let k = 0; while (!E5.openJob.busy && k++ < 60) go(); const inHand = E5.openJob.busy, holed = planNow();
          st.pop(); wall.c[2] = c0; wall.h[2] = h0; const s0 = st[0]; st[0] = { ...s0 }; bump(); go(); const whole = planNow(); let wrong = shown() !== whole || E5.openJob.busy ? 1 : 0; for (let n = 0; n < 60; n++) { go(); if (shown() !== whole) wrong++; }
          if (!inHand) f.push('no finding was in hand to change the level under'); if (holed === whole) f.push('the new level has the old one\'s openings (this part asks nothing)');
          if (wrong) f.push(`a new level while the openings were being found: on ${wrong} of its first 61 frames its openings were not its own`);
          st[0] = s0; bump(); go();
          // … and with the row put off, what is in hand is given up with the lights
          wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; st.push(second); bump(); k = 0; while (!E5.openJob.busy && k++ < 60) go(); const inHand2 = E5.openJob.busy, w0 = lookConfig.windows; lookConfig.windows = 0; go(); if (!inHand2) f.push('no finding was in hand to put the row off under'); if (E5.openJob.busy || recs().length) f.push('the row put off: a finding is still in hand, or lights are'); lookConfig.windows = w0;
          st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); go(); if (!rest()) f.push('never at rest (the row on again)');
          // … and a finding that throws is not begun again for the same boxes: the plan in use serves (the key was kept before the work, once)
          { const real = E5.openReplan, was = shown(); E5.openReplan = function* () { yield; throw new Error('thrown'); }; wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; st.push(second); bump();
            let threw = 0; for (let n = 0; n < 40; n++) { try { go(); } catch { threw++; } } if (threw !== 1 || E5.openJob.busy || shown() !== was) f.push(`a finding that throws: thrown ${threw} time(s) in 40 frames (want once), ${E5.openJob.busy ? 'still in hand' : 'not in hand'}`);
            E5.openReplan = real; st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); go(); } }
        else { st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); }
        if (!rest() || shown() !== planNow()) f.push('never at rest (the wall back)');
        check(f.length === 0, 'the openings are found again a little a frame once the walls have stood still: the old plan serves and nothing goes dark, what lands is the plan made at once, one begun lands the level as it took it whatever moves after, a crate moved every six frames cannot stop it, and it takes a step every frame whatever the frame has left; whole on a level\'s first frame, and a new level never gets the old one\'s', f.join('; ') || said.join(' · ')); }
      // ONE ALLOWANCE OF TIME A FRAME FOR ALL MASK WORK, AND A MASK DECIDED AGAIN ONLY BEHIND WHAT CHANGED (engine.ts lightMasks, openMaskSome;
      // render/reachKeep.ts KeepGuest, ROUND): on the clock the openings' masks are the keeper's guests, worked in its one allowance with the lamps',
      // the classes that owe work leading in turn; and a change of the walls has an opening's mask decided again behind that change only, as a lamp's
      // is. Every frame here is the engine's own lightMasks, as render calls it. The keeper's clock is scripted (every look at it is `tick` ms later:
      // at 0.7 a step is three slices of sight-line steps, at 2.5 it is the one unit of work its leader does) and the planners are given a count of
      // stops beside a clock that never runs out, so nothing here is the machine's. What a frame's guests and the lamps each spent is read off the
      // keeper (its lamps' share is wrapped; a guest's steps are the rest).
      //   MASKCLOCK_BREAK=1 = the keeper never takes a guest (the openings' masks are nobody's on the clock): must FAIL
      //   MASKOWED_BREAK=1  = the openings are told the walls changed somewhere else (a mask keeps cells it should decide again): must FAIL
      { const MB = process.env.MASKCLOCK_BREAK === '1', OB = process.env.MASKOWED_BREAK === '1', st = g.engine.world.statics as any[], bump = () => { (g.engine.world as any).geomSerial = ((g.engine.world as any).geomSerial ?? 0) + 1; }, f: string[] = [], said: string[] = [];
        const doors = g.doors.list.map((d: any) => ({ hinge: d.def.hinge, closedDir: d.def.closedDir, width: d.def.width }));
        const lampsLeft = (K7: any) => { const P = K7.peek(); return P.queue.length + P.held.length + P.now.length; };
        interface Stand { E7: any; K7: any; spy: { lamps: number; lampsRan: boolean; guests: { first: boolean; rank: number }[]; asked: number; calls: number; firsts: number; others: number; guestSteps: number }; tick: number; /** the keeper is handed no guest (it has no time for them: what they are owed piles up) */ freeze: boolean; /** the finding does not say where the walls changed (every mask is owed all of it: the rule until 11-05) */ noRect: boolean; /** the most sight-line steps of a clocked frame that neither the lamps' share nor a guest's took */ outside: number; /** an opening the finding is to give somewhere else (a plan that moves one where no wall of the raster changed) */ move: ((o: any) => any) | null; /** the most sight-line steps one share of a guest's mask took, and the most shares the guests were given in one frame */ most: number; calls: number }
        const mk7 = (boot = true): Stand => { const E7 = mk(), K7 = E7.reach, S: Stand = { E7, K7, spy: { lamps: 0, lampsRan: false, guests: [], asked: 0, calls: 0, firsts: 0, others: 0, guestSteps: 0 }, tick: 0.7, freeze: false, noRect: false, outside: 0, move: null, most: 0, calls: 0 }; if (boot) quiet(() => K7.boot(doors, g.reachInterior));
          const L = K7.lamps.bind(K7); K7.lamps = (...a: any[]) => { const s0 = REACH_WORK.steps; S.spy.lampsRan = true; try { return L(...a); } finally { S.spy.lamps += REACH_WORK.steps - s0; } };
          const T = K7.tickTimed.bind(K7); K7.tickTimed = (n?: number, gs?: any[]) => { S.spy.guests = (gs ?? []).map(x => ({ first: x.first, rank: x.rank }));
            for (const x of gs ?? []) { const some = x.some; x.some = (steps: number) => { const s0 = REACH_WORK.steps; S.spy.calls++; if (x.first) S.spy.firsts++; else S.spy.others++; try { return some(steps); } finally { const d = REACH_WORK.steps - s0; S.spy.guestSteps += d; S.most = Math.max(S.most, d); } }; }
            return MB || S.freeze ? T(n) : T(n, gs); };
          let t = 0; K7.timeNow = () => { S.spy.asked++; return (t += S.tick); };
          const R = E7.openReplan.bind(E7); E7.openReplan = function* (): any { const r = yield* R(); return { ...r, plan: S.move ? r.plan.map(S.move) : r.plan, rect: S.noRect ? null : OB && r.rect ? [0, 0, 0.01, 0.01] : r.rect }; };
          quiet(() => { E7.lightMasks({ left: Infinity }); E7.lightMasks({ left: Infinity }); }); return S; };
        // one frame on the clock, and what it spent: the lamps' sight-line steps, the guests', how many guests were handed over and how many of them `first`,
        // and whose mask a share was worked of (a guest whose light is out, one whose old mask serves)
        const frame = (S: Stand) => quiet(() => { const s0 = REACH_WORK.steps, sp = S.spy; sp.lamps = 0; sp.lampsRan = false; sp.guests = []; sp.calls = sp.firsts = sp.others = sp.guestSteps = 0; S.E7.lightMasks({ left: 3, ms: 1e9 }); const all = REACH_WORK.steps - s0; S.calls = Math.max(S.calls, sp.calls); S.outside = Math.max(S.outside, all - sp.lamps - sp.guestSteps);
          return { all, lamps: sp.lamps, lampsRan: sp.lampsRan, guests: sp.guestSteps, n: sp.guests.length, firsts: sp.guests.filter(x => x.first).length, ledF: sp.firsts > 0, ledO: sp.others > 0 }; });
        const owed = (E7: any) => (E7.openRecs as any[]).filter(r => openMaskOwed(r)).length, planned = (S: Stand) => !S.E7.openJob.busy && S.E7.openPlanned === S.E7.openSeen, busy = (S: Stand) => S.E7.openJob.busy || S.E7.openPlanned !== S.E7.openSeen || owed(S.E7) > 0 || S.K7.pending > 0;
        const rest = (S: Stand, what: string, go: (S: Stand) => unknown = frame) => { let k = 0; while (k++ < 4000 && (k < LOOK_OPENINGS.settle + 12 || busy(S))) go(S); if (k >= 4000) f.push(`${what}: the masks never came to rest`); return k; };
        // frames until the finding of the openings that a change of the walls began has landed
        const landed = (S: Stand) => { let k = 0; for (; k < LOOK_OPENINGS.settle + 2; k++) frame(S); while (k++ < 400 && !planned(S)) frame(S); };
        // (what the upload would send: the keeper's row for every opening's light and every lamp)
        const key = (r: any) => `${r.o.pos.map((v: number) => v.toFixed(2)).join(',')}`;
        const masksOf = (S: Stand) => [...(S.E7.openRecs as any[]).map(r => `${key(r)}:${Array.from((S.K7.row(r.l, false) ?? []) as Uint32Array).join('.')}`).sort(), ...(S.K7.peek().lamps as any[]).map((l, i) => `lamp${i}:${Array.from((S.K7.of(l) ?? []) as Uint32Array).join('.')}`)].join('|');
        // openings whose mask is not the one decided WHOLE, from nothing, on the walls as they stand (and is the one the keeper's row holds)
        const stale = (S: Stand) => { const now = buildWallRaster(st, doors); let n = 0; for (const r of S.E7.openRecs as any[]) { const m = reachMaskFor(now, r.o.mid, r.o.dir, r.o.half[0], null) as Uint32Array, row = S.K7.row(r.l, false) as Uint32Array | undefined; if (!r.mask || r.mask !== row || m.some((v: number, i: number) => v !== r.mask[i])) n++; } return n; };
        // the store's outer wall, holed: cut short of the gap, a second piece past it
        const wall = st.find(b => !b.rot && b.h[0] < 0.2 && b.h[1] > 1.2 && b.h[2] > 1.6 && Math.abs(b.c[0] - 34) < 0.2 && Math.abs(b.c[2] - 8) < b.h[2] - 1.2), c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0;
        const second = { ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] };
        const gap = (S: Stand, half: number, mid = c0) => { wall.c[2] = (z0 + mid - half) / 2; wall.h[2] = (mid - half - z0) / 2; second.c[2] = (mid + half + z1) / 2; second.h[2] = (z1 - mid - half) / 2; if (!st.includes(second)) st.push(second); bump(); S.K7.changed([wall.c[0] - 0.3, mid - half, wall.c[0] + 0.3, mid + half]); };
        const mend = (S: Stand) => { const i = st.indexOf(second); if (i >= 0) st.splice(i, 1); wall.c[2] = c0; wall.h[2] = h0; bump(); S.K7.changed([wall.c[0] - 0.3, z0, wall.c[0] + 0.3, z1]); };
        // ANOTHER wall, well away from the hole and the nearest such to a lamp, cut short at its far end by `by` metres (a second charge somewhere else):
        // walls that really change for the lamps near it, and, where the engine is told (`tell`), for every opening
        const lampsAll = (LS.lights as any[]).filter(l => l.kind !== 2 && l.enabled && !l.transient && !l.portal);
        const other = st.filter(b => b !== wall && !b.rot && b.h[1] > 1.2 && b.h[0] < 0.2 && b.h[2] > 1.2 && Math.hypot(b.c[0] - wall.c[0], b.c[2] - c0) > 8)
          .map(b => ({ b, d: Math.min(...lampsAll.map(l => Math.hypot(l.pos[0] - b.c[0], l.pos[2] - b.c[2]))) })).sort((a, b) => a.d - b.d)[0]?.b, oc = other ? other.c[2] : 0, oh = other ? other.h[2] : 0;
        const cut = (S: Stand, by: number, tell: boolean, hold = -1) => { other.c[2] = oc - by / 2; other.h[2] = oh - by / 2; if (tell) bump(); S.K7.changed([other.c[0] - 0.3, oc + oh - 1.2, other.c[0] + 0.3, oc + oh], hold); };
        const hole = (S: Stand, mid = c0) => (S.E7.openRecs as any[]).find(r => Math.abs(r.o.pos[0] - wall.c[0]) < 1.5 && Math.abs(r.o.pos[2] - mid) < 0.5 && r.o.ajar > 0);
        if (!other || !lampsAll.length) f.push('no second wall near a lamp to cut (this block asks nothing)');
        else {
          // ON THE COUNT, to rest: the masks to compare with, and the count asks no clock
          const A = mk7(); A.spy.asked = 0; const count = (S: Stand) => quiet(() => S.E7.lightMasks({ left: 3 }));
          gap(A, 0.75); rest(A, 'on the count', count); cut(A, 0.6, true); rest(A, 'on the count', count); const want = masksOf(A), staleA = stale(A), askedA = A.spy.asked;
          cut(A, 0, true); mend(A); rest(A, 'on the count', count);
          if (askedA) f.push(`a frame with no clock in its pace asked the keeper's clock ${askedA} times`); if (staleA) f.push(`on the count ${staleA} openings' masks at rest are not the ones decided whole`);
          // (a) ON THE CLOCK, A HOLE, AND A SECOND WALL CUT WHILE IT IS DARK. The hole has no light until its mask is whole, so it is `first` and leads
          //     most steps; the lamps the second cut asks are decided in the same steps; every other opening owes a mask behind each change only
          const S = mk7(); for (let k = 0; k < 4; k++) frame(S); gap(S, 0.75);
          let n = 0; while (n++ < 200 && !(hole(S) && !hole(S).mask && !S.E7.openJob.busy)) frame(S); const seenDark = !!hole(S) && !hole(S).mask;
          // … which of its guests leads: only one whose light is OUT for want of the mask and would be lit with it
          let probes = '';
          if (seenDark) { const one = frame(S); if (one.firsts !== 1) f.push(`a hole with no mask: ${one.firsts} of ${one.n} guests lead as lights that are out (want the hole alone)`);
            LOOK_OPENINGS.lit = 3; eye = [14, 12]; const far = frame(S); LOOK_OPENINGS.lit = was.lit; eye = [20, 10];
            const h = hole(S), aj = h.o.ajar; h.o.ajar = 0; const shut = frame(S); h.o.ajar = aj;
            const ex = S.E7.settings.exposure; S.E7.settings.exposure = 0; const unseen = frame(S); S.E7.settings.exposure = ex;
            for (const [name, p] of [['beyond the lit few', far], ['shut', shut], ['under a sky the picture does not show', unseen]] as const) if (p.firsts !== 0 || p.n < 1) f.push(`a hole with no mask, ${name}: ${p.firsts} of ${p.n} guests lead as lights that are out (want none: its light would not be lit with the mask)`);
            probes = `leads alone of ${one.n} guests, and not when far, shut or unseen`; }
          // (the second cut is noted to be read six steps on, as a blast's is: six frames, whoever leads them)
          cut(S, 0.6, true, 6); let dark = 0, lampsWhileDark = 0, lampsOwed = 0, othersWhileDark = 0, heldFor = -1; n = 0;
          for (; n < 4000 && (n < LOOK_OPENINGS.settle + 12 || busy(S)); n++) { const h = hole(S), isDark = !!h && !h.mask, lb = lampsLeft(S.K7), olds = (S.E7.openRecs as any[]).filter(r => r.mask && openMaskOwed(r)).length, r = frame(S);
            if (heldFor < 0 && S.K7.peek().held.length === 0) heldFor = n + 1;
            if (isDark) { dark++; if (lb > 0) lampsOwed++; if (r.lamps > 0) lampsWhileDark++; if (olds > 0 && r.guests > 0 && (S.E7.openRecs as any[]).filter(q => q.mask && openMaskOwed(q)).length < olds) othersWhileDark++; } }
          if (n >= 4000) f.push('on the clock the masks never came to rest');
          if (!seenDark || !(dark > 0)) f.push('no frame had an opening with no light waiting (this part asks nothing)');
          else if (!(lampsOwed > 0)) f.push('the lamps owed nothing while the hole waited (this part asks nothing)');
          else if (!(lampsWhileDark > 0)) f.push(`while the hole waited for its mask (${dark} frames, the lamps owing work on ${lampsOwed}) no sight line was spent on a lamp's mask: the hole's class led every step`);
          if (!(heldFor >= 6 && heldFor <= 12)) f.push(`a note of the walls to be read six steps on was read after ${heldFor} frames with the hole's mask leading (want six, and the lamps' turn)`);
          const staleS = stale(S); if (staleS) f.push(`on the clock ${staleS} openings' masks at rest are not the ones decided whole on the walls as they stand`);
          if (masksOf(S) !== want) f.push('the masks at rest are not the ones the count makes');
          if (!f.length) said.push(`a hole (${probes}): ${dark} frames without its light, the lamps' masks worked on ${lampsWhileDark} of them and other openings' finished on ${othersWhileDark}, the walls' note read ${heldFor} frames on, at rest after ${n} with the count's masks (${want.split('|').length}), each opening's the one decided whole`);
          // (b) EVERY LAMP ASKED EVERY FRAME for all of its mask (the keeper's own asking, of the whole level: its queue is never empty, and a mask
          //     asked again while it is being decided is begun again), a step being ONE unit of its leader's work: the openings whose old mask serves
          //     are decided all the same, in their turn, and the lamps are worked on in theirs
          { S.tick = 2.5; gap(S, 1.5); let k = 0, olds0 = -1, at = -1, lampFrames = 0, idle = 0; const ALL = [-1e4, -1e4, 1e4, 1e4], askAll = () => { for (const l of S.K7.peek().lamps as any[]) S.K7.ask(l, ALL, [0, 0, 1e6, 1e6]); };
            for (; k < 2400 && at < 0; k++) { askAll(); if (S.K7.peek().queue.length === 0) idle++; const r = frame(S); if (r.lamps > 0) lampFrames++; const o = owed(S.E7); if (olds0 < 0 && !S.E7.openJob.busy && S.E7.openPlanned === S.E7.openSeen && o > 0) olds0 = o; if (olds0 > 0 && o === 0) at = k; }
            if (!(olds0 > 0) || idle > 0) f.push(`with every lamp asked every frame: ${olds0} openings owed a mask, the lamps owed nothing on ${idle} frames (this part asks nothing)`); else if (at < 0) f.push(`with every lamp asked every frame the openings' ${olds0} owed masks were never decided in 2400 frames`); else if (!(lampFrames * 4 >= at)) f.push(`with every lamp asked every frame the lamps' masks were worked on ${lampFrames} of ${at} frames (want their turn: a quarter at the least)`); else said.push(`every lamp asked every frame, a step one unit of work: the openings' ${olds0} owed masks whole by frame ${at}, the lamps worked on ${lampFrames} of them`);
            S.tick = 0.7; rest(S, 'after the lamps were asked every frame'); }
          // (b2) ALL THREE OWE AT ONCE, for as long as a hole is dark: its own mask, every lamp asked every frame, and the second wall cut again
          //      every thirty frames so that every other opening keeps owing. A step being one unit of its leader's work, the frames go round: the
          //      hole's mask two of four, the lamps' one, the others' one
          { mend(S); rest(S, 'the wall mended'); S.tick = 2.5; gap(S, 0.75); landed(S); const ALL = [-1e4, -1e4, 1e4, 1e4]; let dark = 0, fF = 0, fL = 0, fO = 0, two = 0, k = 0;
            for (; k < 1500 && hole(S) && !hole(S).mask; k++) { for (const l of S.K7.peek().lamps as any[]) S.K7.ask(l, ALL, [0, 0, 1e6, 1e6]); if (k % 30 === 0) cut(S, k % 60 ? 0.8 : 0.6, true);
              const r = frame(S); dark++; if (r.ledF) fF++; if (r.ledO) fO++; if (r.lampsRan) fL++; if (+r.ledF + +r.ledO + +r.lampsRan !== 1) two++; }   // (the lamps' unit may be a directional light's cells, which are rays and no sight-line steps: their share RAN)
            if (!(dark >= 40)) f.push(`all three owing: the hole was dark for ${dark} frames (this part asks nothing)`);
            else if (!(two === 0 && Math.abs(fF * 2 - dark) <= 4 && Math.abs(fL * 4 - dark) <= 8 && Math.abs(fO * 4 - dark) <= 8)) f.push(`all three owing for ${dark} frames, a step one unit of work: the hole's mask was worked on ${fF}, the lamps' on ${fL}, the other openings' on ${fO}, and ${two} frames were not one class's alone (want two, one and one of four, one class a frame)`);
            else said.push(`all three owing for ${dark} frames: the hole's mask worked on ${fF}, the lamps' on ${fL}, the other openings' on ${fO}`);
            S.tick = 0.7; cut(S, 0.6, true); rest(S, 'after all three owed'); const st2 = stale(S); if (st2) f.push(`after all three owed: ${st2} masks at rest not the ones decided whole`); }
          // (c) A HOLE FOUND AGAIN A LITTLE WAY OFF, time after time (it slides along the wall every eight frames, twenty times): the mask begun for
          //     it is finished as begun and serves as an old one, so it has its light while it is still sliding
          { mend(S); rest(S, 'the wall mended'); let k = 0, mid = c0, slid = 0, litAt = -1, litSlid = -1; gap(S, 0.75, mid);
            for (; k < 2000 && (slid < 20 || k < 8 * 21); k++) { if (k > 0 && k % 8 === 0 && slid < 20) { mid += 0.12; slid++; gap(S, 0.75, mid); } frame(S); const h = hole(S, mid); if (litAt < 0 && h?.mask) { litAt = k; litSlid = slid; } }
            const end = rest(S, 'the hole slid'), staleC = stale(S);
            if (litAt < 0 || !(litSlid < 20)) f.push(`a hole that slid along its wall every eight frames ${litAt < 0 ? 'had no mask while it slid' : `had its first mask at frame ${litAt}, after its last move`} (a mask begun is finished as begun)`);
            else if (staleC) f.push(`the hole come to rest: ${staleC} openings' masks are not the ones decided whole where they stand`); else said.push(`a hole slid along its wall twenty times: lit by frame ${litAt} (${litSlid} moves in), and at rest ${end} frames on every mask is the one decided whole where it stands`);
            mend(S); cut(S, 0, true); rest(S, 'the walls mended'); }
          // (c2) A HOLE MOVED ONCE, 12 cm along its wall: at rest with its mask, and again while its first mask is in the making. A mask decided at
          //      one place is never kept, nor decided again in part, for another: at rest each is the one decided whole where its opening stands
          { gap(S, 0.75); rest(S, 'a hole'); const m0 = hole(S)?.mask; gap(S, 0.75, c0 + 0.12); rest(S, 'the hole moved'); const h1 = hole(S, c0 + 0.12), stale1 = stale(S), again = !!h1?.mask && h1.mask !== m0;
            mend(S); rest(S, 'the wall mended'); gap(S, 0.75); let n = 0; while (n++ < 200 && !hole(S)?.job) frame(S); const inHand = !!hole(S)?.job && !hole(S).mask; gap(S, 0.75, c0 + 0.12);
            let landedOff = false; for (n = 0; n < 400 && !landedOff; n++) { frame(S); const h = hole(S, c0 + 0.12) ?? hole(S); if (h?.mask) landedOff = true; } rest(S, 'the hole moved with its mask in the making'); const stale2 = stale(S);
            if (!again || stale1) f.push(`a hole moved 12 cm at rest: its mask ${again ? 'was' : 'was not'} decided again, ${stale1} masks not the ones decided whole where they stand`);
            else if (!inHand || !landedOff || stale2) f.push(`a hole moved 12 cm with its first mask in the making (in hand: ${inHand}, landed: ${landedOff}): ${stale2} masks at rest not the ones decided whole where they stand`);
            else said.push('a hole moved 12 cm, at rest and with its first mask in the making: every mask at rest the one decided whole where it stands');
            mend(S); rest(S, 'the wall mended'); }
          // (c3) AN OPENING FOUND SOMEWHERE ELSE WHERE NO WALL OF THE RASTER CHANGED (its sill or its head moved: the finding reads heights the raster
          //      does not): 25 cm along its wall here, by a finding that is made to say so, begun by a crate nudged. Nothing is owed it for the walls,
          //      and its mask is decided again all the same, whole, for where it stands: one at rest, and one whose first mask is in the making
          { const crate = st.find(b => b !== wall && b !== second && b.h[1] < 0.6 && b.c[1] < 1.2), y0 = crate.c[1], nudge = () => { crate.c[1] = crate.c[1] === y0 ? y0 + 0.02 : y0; bump(); };
            const along = (o: any) => ({ ...o, pos: [o.pos[0] - o.dir[2] * 0.25, o.pos[1], o.pos[2] + o.dir[0] * 0.25], mid: [o.mid[0] - o.dir[2] * 0.25, o.mid[1], o.mid[2] + o.dir[0] * 0.25] });
            const only = (at: number[]) => (o: any) => Math.hypot(o.pos[0] - at[0], o.pos[2] - at[2]) < 0.01 ? along(o) : o;
            const w = (S.E7.openRecs as any[]).find(r => r.o.ajar > 0 && r.mask), m0 = w.mask, l0 = w.l; S.move = only(w.o.pos); nudge(); rest(S, 'a window found a little way off');
            const w1 = (S.E7.openRecs as any[]).find(r => r.l === l0), again = !!w1?.mask && w1.mask !== m0, stale1 = stale(S); S.move = null; nudge(); rest(S, 'the window back');
            gap(S, 0.75); let n = 0; while (n++ < 200 && !hole(S)?.job) frame(S); const inHand = !!hole(S)?.job && !hole(S).mask, hl = hole(S)?.l; if (inHand) S.move = only(hole(S).o.pos); nudge(); rest(S, 'a hole found a little way off');
            const h1 = (S.E7.openRecs as any[]).find(r => r.l === hl), stale2 = stale(S); S.move = null; nudge(); rest(S, 'the hole back'); const stale3 = stale(S);
            if (!again || stale1) f.push(`a window found 25 cm off where no wall changed: its mask ${again ? 'was' : 'was not'} decided again, ${stale1} masks not the ones decided whole where they stand`);
            else if (!inHand || !h1?.mask || stale2 || stale3) f.push(`a hole found 25 cm off with its first mask in the making (in hand: ${inHand}): ${stale2} masks at rest not the ones decided whole where they stand, ${stale3} with it back`);
            else said.push('an opening found 25 cm off where no wall changed, at rest and with its first mask in the making: decided again whole where it stands');
            // (c4) AND WALLS THAT CHANGE IN ITS VIEW WHILE ITS FIRST MASK IS IN THE MAKING (a box set down inside the hole): that mask is finished on
            //      the walls it began with, and decided again behind the box
            mend(S); rest(S, 'the wall mended'); gap(S, 0.75); n = 0; while (n++ < 200 && !hole(S)?.job) frame(S); const inHand2 = !!hole(S)?.job && !hole(S).mask;
            const box = { ...wall, c: [wall.c[0] - 1.6, wall.c[1], c0], h: [0.5, wall.h[1], 0.5] }; st.push(box); bump(); S.K7.changed([box.c[0] - 0.5, c0 - 0.5, box.c[0] + 0.5, c0 + 0.5]); rest(S, 'a box set down'); const stale4 = stale(S);
            st.splice(st.indexOf(box), 1); bump(); S.K7.changed([box.c[0] - 0.5, c0 - 0.5, box.c[0] + 0.5, c0 + 0.5]); mend(S); rest(S, 'the walls mended');
            if (!inHand2 || stale4) f.push(`a box set down in a hole's view with its first mask in the making (in hand: ${inHand2}): ${stale4} masks at rest not the ones decided whole on the walls as they stand`); else said.push('a box set down in its view meanwhile: decided again behind it'); }
          // (c5) A MASK TAKEN OVER IS OWED WHOLE. The keeper has no time for guests (none is handed to it) while: a box is set down in the hole's
          //      view (the hole's mask is owed that rect), and then the hole is found as ANOTHER opening over the old one (the same rectangle, its
          //      weight 0.7 m off, past what counts as the same opening: a new record that takes the old one's mask). Decided again all the same
          { gap(S, 0.75); rest(S, 'a hole'); S.freeze = true; const box = { ...wall, c: [wall.c[0] - 1.6, wall.c[1], c0], h: [0.5, wall.h[1], 0.5] }, rect = [box.c[0] - 0.5, c0 - 0.5, box.c[0] + 0.5, c0 + 0.5];
            const crate = st.find(b => b !== wall && b !== second && b.h[1] < 0.6 && b.c[1] < 1.2), y0 = crate.c[1];
            st.push(box); bump(); S.K7.changed(rect); landed(S); const h0 = hole(S), owes1 = !!h0 && h0.owed !== 'all' && h0.owed.length === 1, l0 = h0?.l;
            S.move = (o: any) => h0 && Math.hypot(o.pos[0] - h0.o.pos[0], o.pos[2] - h0.o.pos[2]) < 0.01 ? { ...o, pos: [o.pos[0], o.pos[1], o.pos[2] + 0.7] } : o; crate.c[1] = y0 + 0.02; bump(); landed(S);
            const h1 = (S.E7.openRecs as any[]).find(r => r.l === l0), took = !!h1 && h1.mask === h0?.mask && h1.owed === 'all' /* (found again as itself it keeps its one rect: only a takeover is owed all) */; S.freeze = false; rest(S, 'a hole taken over'); const stale5 = stale(S);
            S.move = null; crate.c[1] = y0; st.splice(st.indexOf(box), 1); bump(); S.K7.changed(rect); mend(S); rest(S, 'the walls mended');
            if (!owes1 || !took) f.push(`a hole taken over with a rect owed: owed one rect ${owes1}, a new record with the old mask ${took} (this part asks nothing)`); else if (stale5) f.push(`a hole taken over by a new record while its mask was owed a rect: ${stale5} masks at rest not the ones decided whole`); else said.push('a hole taken over while its mask was owed a rect: decided again');
            // (c6) AND MORE RECTS THAN ARE KEPT ONE BY ONE: the second wall cut eight times, 15 cm more each time and then that last cut made again
            //      (a rect inside one owed adds nothing), then a box set down in the hole's view, none of it worked meanwhile. The ninth is not
            //      dropped: the mask is owed all of it
            gap(S, 0.75); rest(S, 'a hole'); S.freeze = true; for (let k = 1; k <= 8; k++) { cut(S, 0.15 * k, true); landed(S); } cut(S, 1.05, true); landed(S); cut(S, 1.2, true); landed(S); const eight = hole(S)?.owed === 'all' ? -1 : hole(S)?.owed.length;
            st.push(box); bump(); S.K7.changed(rect); landed(S); const all = hole(S)?.owed === 'all'; S.freeze = false; rest(S, 'nine changes owed'); const stale6 = stale(S);
            st.splice(st.indexOf(box), 1); bump(); S.K7.changed(rect); cut(S, 0, true); mend(S); rest(S, 'the walls mended');
            if (eight !== 8 || !all) f.push(`nine changes with no mask worked: the hole owed ${eight} rects after eight cuts and one made again (want 8), all of it after the ninth: ${all}`); else if (stale6) f.push(`nine changes owed: ${stale6} masks at rest not the ones decided whole`); else said.push('nine changes owed: all of it, decided whole'); }
          // (by the scripted clock a step is late at its third look, and every share of a mask is followed by a look: three shares a frame, whoever worked;
          //  a share is 4000 sight-line steps and the cell it ends in, which is a few thousand where the lines are long)
          if (!(S.calls <= 3 && S.most <= 8000 && S.outside === 0)) f.push(`the guests were given ${S.calls} shares of a mask in one frame, one share took ${S.most} sight-line steps, and ${S.outside} steps of a frame were neither the lamps' nor a guest's (three shares a frame by the scripted clock, each 4000 and the cell it ends in, and no mask decided beside the keeper's step)`); else said.push(`never more than ${S.calls} shares of a guest's mask in a frame, the longest ${S.most} sight-line steps, none beside the keeper's step`);
          // (d) A KEEPER THAT WAS NEVER BOOTED (a page with no lamps' masks of its own) still works its guests: the hole has its light
          { const U = mk7(false); for (let k = 0; k < 4; k++) frame(U); gap(U, 0.75); const k = rest(U, 'a keeper never booted'), h = hole(U), staleU = stale(U); mend(U); rest(U, 'a keeper never booted');
            if (!h?.mask || staleU || !(U.calls <= 3 && U.most <= 8000)) f.push(`a keeper never booted: the hole ${h?.mask ? 'has' : 'has not'} its mask, ${staleU} masks not the ones decided whole, ${U.calls} shares in a frame, the longest ${U.most}`); else said.push(`a keeper never booted: the hole masked, at rest after ${k}`); }
          // (e) THE ROW OFF (no daylight at openings: nothing to be a guest), the lamps' masks are still decided on the clock, by the keeper's own step
          { const R = mk7(); for (let k = 0; k < 4; k++) frame(R); lookConfig.windows = 0; frame(R); cut(R, 0.6, true); let k = 0, worked = 0, owedAt = 0; for (; k < 600 && (k < 3 || R.K7.pending > 0); k++) { owedAt = Math.max(owedAt, lampsLeft(R.K7)); if (frame(R).lamps > 0) worked++; }
            const left = R.K7.pending; cut(R, 0, true); for (let j = 0; j < 600 && (j < 3 || R.K7.pending > 0); j++) frame(R); lookConfig.windows = 1;
            if (!(owedAt > 0)) f.push('the row off: the cut asked no lamp (this part asks nothing)'); else if (left > 0 || !(worked > 0)) f.push(`the row off: the lamps' masks were ${worked > 0 ? 'worked on' : 'never worked on'} and ${left} were still owed after 600 frames (the keeper is stepped where the openings did not step it)`); else said.push(`the row off: the lamps' masks decided in ${k} frames`); }
          // (e2) "LIGHT STAYS IN ITS ROOM" OFF WHILE THE WALLS CHANGE (no mask is worked or used), then on again: every mask the one decided whole
          { const J = mk7(); for (let k = 0; k < 4; k++) frame(J); gap(J, 0.75); rest(J, 'a hole'); lookConfig.rooms = 0; frame(J); const box = { ...wall, c: [wall.c[0] - 1.6, wall.c[1], c0], h: [0.5, wall.h[1], 0.5] }, rect = [box.c[0] - 0.5, c0 - 0.5, box.c[0] + 0.5, c0 + 0.5];
            st.push(box); bump(); J.K7.changed(rect); landed(J); lookConfig.rooms = 1; rest(J, 'the rooms row on again'); const staleJ = stale(J); st.splice(st.indexOf(box), 1); bump(); J.K7.changed(rect); mend(J); rest(J, 'the walls mended');
            if (staleJ) f.push(`the rooms row off while a box was set down, then on: ${staleJ} masks at rest not the ones decided whole`); else said.push('the rooms row off while the walls changed, then on: every mask the one decided whole'); }
          // (e3) A CLOCK THAT DOES NOT RUN (a browser that holds its clock to tens of milliseconds): a step is still no more than four of the count's
          //      shares of sight-line steps, and the masks come to rest
          { const C = mk7(); for (let k = 0; k < 4; k++) frame(C); C.tick = 0; gap(C, 0.75); let most = 0, k = 0; for (; k < 4000 && (k < LOOK_OPENINGS.settle + 12 || busy(C)); k++) most = Math.max(most, frame(C).all); const staleC = stale(C); mend(C); rest(C, 'a clock that does not run');
            if (k >= 4000 || staleC || !(most <= 4 * REACH_PACE.steps + 8000)) f.push(`a clock that does not run: at rest after ${k} frames, ${staleC} masks not the ones decided whole, the most in one frame ${most} sight-line steps (four of the count's shares of ${REACH_PACE.steps} at the most)`); else said.push(`a clock that does not run: no frame over ${most} sight-line steps`); }
          // (f) WHAT IT SAVES. The same hole with the finding made not to say where the walls changed, so that every mask is owed all of it (the rule
          //     until 11-05): the sight-line steps the openings' masks took, and the same masks
          { const run = (old: boolean) => { const N = mk7(); N.noRect = old; for (let k = 0; k < 4; k++) frame(N); let steps = 0; const go = (X: Stand) => { steps += frame(X).guests; };
              gap(N, 0.75); rest(N, old ? 'every mask whole' : 'behind the change', go); const got = { steps, masks: masksOf(N), stale: stale(N) }; mend(N); rest(N, old ? 'every mask whole' : 'behind the change'); return got; };
            const now = run(false), old = run(true);
            if (old.stale || now.stale || old.masks !== now.masks) f.push(`a hole, every mask whole again and behind the change only: ${old.stale} and ${now.stale} masks not the ones decided whole, the same masks: ${old.masks === now.masks}`);
            else if (!(now.steps * 2 <= old.steps)) f.push(`a hole cost the openings' masks ${now.steps} sight-line steps, and ${old.steps} with every mask whole again (want half at the most: only the hole's own is decided whole)`);
            else said.push(`a hole's daylight masks: ${now.steps} sight-line steps, ${old.steps} with every mask whole again, the same masks`); }
        }
        check(f.length === 0, 'on the clock a frame\'s mask work is one allowance of time and a mask is decided again only behind what changed: the openings\' masks are the keeper\'s guests, a hole with no light leads but stops nobody, lamps asked every frame do not stop the openings\' nor they the lamps\', a mask begun is finished though its hole moves, what a mask is owed is never lost, the keeper is stepped with the row off, a clock that does not run is still a bounded step, and every mask at rest is the one decided whole on the walls as they stand', f.join('; ') || said.join(' · ')); }
      // THE ENGINE'S OWN WORDS ON THE PACE (engine.ts framePace, startPlansOver; nothing here calls render): a page's first frames have every plan at once, then
      // the frame's allowance and the draw order's wait; a restart or a save put back gives up what is in hand and has the frames at once again
      { const { PLAN_PACE } = await import(`${ROOT}/src/render/planPace.ts`), { WallSides } = await import(`${ROOT}/src/render/wallSides.ts`), was = PLAN_PACE.on, first = LOOK_OPENINGS.settle + 2, f: string[] = [];
        const EP = Object.assign(mk(), { planFresh: first, drawWait: { calm: 0, still: 0 }, wallSides: new WallSides() }), atOnce = (p: any) => p.wait === null && p.plan.left === Infinity && p.plan.ms === undefined;
        PLAN_PACE.on = true; let n = 0; while (n < 40 && atOnce(EP.framePace())) n++;
        if (n !== first) f.push(`a page's first frames: ${n} at once (want ${first})`);
        const p1 = EP.framePace(); if (atOnce(p1) || p1.plan.ms !== PLAN_PACE.ms || p1.wait !== EP.drawWait || p1.wait.calm !== PLAN_PACE.calm) f.push('after them: not the frame\'s allowance and the order\'s wait');
        EP.openJob.begin((function* (): Generator<void, any, void> { yield; return null; })()); EP.startPlansOver(); n = 0; while (n < 40 && atOnce(EP.framePace())) n++;
        if (EP.openJob.busy || EP.wallSides.busy || n !== first) f.push(`the level put back to another state: ${EP.openJob.busy ? 'a finding still in hand, ' : ''}${n} frames at once (want ${first})`);
        PLAN_PACE.on = false; if (!atOnce(EP.framePace())) f.push('the pace off: a frame not at once'); PLAN_PACE.on = was;
        // … and the switch is one the stands, the registry and a recording know (host/profile.ts pageSwitches and FROZEN_HOLDS, host/standEnv.ts, game/knobs.ts,
        // game/journal.ts applyKnobs): down on the frozen stand, up on the page's, or as its own word says; a recording with no table for it replays with it down
        { const PR = await import(`${ROOT}/src/host/profile.ts`), SE = await import(`${ROOT}/src/host/standEnv.ts`), KN = await import(`${ROOT}/src/game/knobs.ts`), J = await import(`${ROOT}/src/game/journal.ts`);
          const shipped = (process.env.TPS_PROFILE ?? '') === 'shipped', said = process.env.TPS_PLANPACE, want = said !== undefined ? said === '1' : shipped;
          if (was !== want) f.push(`this stand (${shipped ? 'the page\'s' : 'frozen'}) stood the pace ${was ? 'on' : 'off'} (want ${want ? 'on' : 'off'})`);
          if (PR.FROZEN_HOLDS['PLAN_PACE.on'] !== 'TPS_PLANPACE=1 (a picture only)' || !SE.STAND_ENV.includes('TPS_PLANPACE') || !KN.JOURNAL_KNOBS.includes('PLAN_PACE') || KN.knobRegistry().PLAN_PACE !== PLAN_PACE) f.push('the row, the stand\'s word, the recording\'s list or the registry lacks it');
          const snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(g))); PLAN_PACE.on = true; const old = snap(); delete old.tables.PLAN_PACE; J.applyKnobs(g, old); const before = PLAN_PACE.on; PLAN_PACE.on = true; const rec = snap(); PLAN_PACE.on = false; J.applyKnobs(g, rec); const kept = PLAN_PACE.on; PLAN_PACE.on = was;
          if (before !== false || kept !== true) f.push(`a recording from before the table replays with the pace ${before ? 'on' : 'off'} (want off), one made with it on with it ${kept ? 'on' : 'off'} (want on)`); }
        // … and so is the reach masks' pace (render/reach.ts REACH_PACE, `?reachpace=old`): the same on both stands, so no row, but in the registry and on a
        // recording. BOTH WORDS ARE READ WITH THE PAGE'S OTHER SWITCHES (host/profile.ts pageSwitches, which a server's stand runs on a recipe's address) and
        // main.ts reads neither by itself. Asked in a process of its own: pageSwitches writes every switch of the page
        { const KN = await import(`${ROOT}/src/game/knobs.ts`), J = await import(`${ROOT}/src/game/journal.ts`), wasW = REACH_PACE.whole;
          if (wasW !== false || !KN.JOURNAL_KNOBS.includes('REACH_PACE') || KN.knobRegistry().REACH_PACE !== REACH_PACE) f.push('the reach masks\' pace: up on this stand with no word, or not in the registry or the recording\'s list');
          const snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(g))); REACH_PACE.whole = true; const rec = snap(), old = snap(); delete old.tables.REACH_PACE; J.applyKnobs(g, old); const before = REACH_PACE.whole; J.applyKnobs(g, rec); const kept = REACH_PACE.whole; REACH_PACE.whole = wasW;
          if (before !== false || kept !== true) f.push(`the reach masks' pace: a recording with no table replays with it ${before ? 'whole' : 'in shares'} (want shares), one made whole with it ${kept ? 'whole' : 'in shares'} (want whole)`);
          const ask = `const P = await import('${ROOT}/src/host/profile.ts'), A = await import('${ROOT}/src/render/planPace.ts'), R = await import('${ROOT}/src/render/reach.ts'); const out = []; for (const q of ['', 'planpace=old', 'reachpace=old', 'planpace=old&reachpace=old']) { P.pageSwitches(new URLSearchParams(q)); out.push((A.PLAN_PACE.on ? 1 : 0) + '' + (R.REACH_PACE.whole ? 1 : 0)); } console.log('WORDS ' + out.join(' '));`;
          const r = Bun.spawnSync(['bun', '-e', ask], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env }), got = (/WORDS (.*)/.exec(r.stdout.toString()) ?? [])[1] ?? `no answer (${r.stderr.toString().slice(0, 120)})`;
          if (got !== '10 00 11 01') f.push(`the page's switches read with '', planpace=old, reachpace=old, both (the planners' pace, then the masks' whole): ${got} (want 10 00 11 01)`);
          if (/q\.get\('(planpace|reachpace)'\)/.test(readFileSync(`${ROOT}/src/main.ts`, 'utf8'))) f.push('main.ts reads a pace\'s word by itself'); }
        check(f.length === 0, 'the engine gives every plan at once on a page\'s first frames and after a restart or a save put back (what is in hand given up), and the frame\'s allowance and the draw order\'s wait otherwise; at once always with the pace off; the planners\' pace and the reach masks\' are switches the stands, the registry and a recording know, read with the page\'s other switches', f.join('; ') || `${first} frames at once, then ${PLAN_PACE.ms} ms a frame and ${PLAN_PACE.calm} still frames for the order`); }
      // the rooms row put off and on, and put on for the first time in the middle of a level: no frame with the rooms dark
      { const n0 = lit().length; lookConfig.rooms = 0; frames(1); const off = lit().length, bare = lit().every(l => !M(l)); lookConfig.rooms = 1; frames(1); const on = lit().length, masked = lit().every(l => !!M(l));
        lookConfig.rooms = 0; const E2 = mk(); quiet(() => { E2.lookOpeningLights(); E2.lookOpeningLights(); }); const a2 = (E2.dayLights as any[]).filter(l => l.enabled).length; lookConfig.rooms = 1; quiet(() => E2.lookOpeningLights()); const b2 = (E2.dayLights as any[]).filter(l => l.enabled && M(l)).length;
        check(n0 === O.length && off === n0 && bare && on === n0 && masked && a2 === n0 && b2 === n0, 'with "Light stays in its room" put off the lights are lit with no mask, and put on again, or on for the first time in the middle of a level, every one is lit on that same frame with its mask', `${n0} lit · the row off ${off} · on again ${on} · a level begun with the row off ${a2}, the row put on ${b2}`); }
      lookConfig.windows = 0; frames(1); check(day().length === 0, 'the row at 0 takes every one of them out');
    } finally { lookConfig.windows = was.windows; lookConfig.rooms = was.rooms; lookConfig.eyeAt = was.eye; lookConfig.doorGaps = was.gaps; LOOK_OPENINGS.lit = was.lit; } }
}

// ---- the office: windows wider than a doorway and strips of glass over head height (the first cut of this rule lost both)
{ const S = await stand('office'); S.shut(); const { O } = S.read(), wide = O.filter(o => o.full > 2.5), high = O.filter(o => o.pos[1] > 2.2);
  check(O.length >= 9 && wide.length >= 5 && high.length >= 2, 'the office has every window the glass-only rule found there (nine) and more: its long windows and its strips of high glass among them', `${O.length} with every door shut: ${wide.length} of more than 2.5 m², ${high.length} over head height`); }
// ---- awnings: a stall's roof on posts is no room
{ const out: string[] = []; let ok = true;
  for (const name of ['gwot-yard', 'tut-quiet']) { const S = await stand(name); S.shut(); const { O, all } = S.read(); out.push(`${name}: ${O.length} open with every door shut, ${all.length - O.length} doorways`); if (O.some(o => o.full > 3) || O.length > 1) ok = false; }
  check(ok, 'a yard of market stalls gets no light from their awnings (the first cut lit four of five there)', out.join(' · ')); }

if (process.env.TPS_PROFILE === 'shipped') await hole();
else { const r = Bun.spawnSync(['bun', 'run', import.meta.path, '--hole'], { cwd: ROOT, env: { ...process.env, TPS_PROFILE: 'shipped' }, stdout: 'pipe', stderr: 'pipe' }), out = r.stdout.toString().split('\n').filter(l => /^- (PASS|FAIL)|^    /.test(l)).join('\n');
  console.log(out || '- FAIL — the hole line gave nothing'); if (r.exitCode !== 0 || !out) red = true; }

// ---- the vault's floor: a window hole with no glass
{ const S = await stand('vault'); S.shut(); const a = S.read(), hole = a.O.find(o => o.full < 2);
  check(a.O.length === 1 && !!hole && Math.abs(hole.ajar - 0.8) < 0.06 && Math.abs(hole.pos[0] - 17.4) < 0.15 && Math.abs(hole.pos[1] - 1.5) < 0.06 && hole.dir[0] === -1 && inward(a.m, hole), 'a window hole with no glass is an opening by the same rule, measured as what is open of it', a.O.map(say).join(' · ')); }

// ---- how bright, by the hour
{ const sunOf = (h: any) => ({ intensity: h.sun.intensity, dir: [0, -Math.sin(h.sun.elevDeg * Math.PI / 180), 0], color: h.sun.color }), lum = (r: readonly number[]) => 0.2126 * r[0] + 0.7152 * r[1] + 0.0722 * r[2];
  const of = (id: string) => lum(openingLooks(HOURS[id].sky.zenith, HOURS[id].sky.horizon, 1, sunOf(HOURS[id])));
  const L = JSON.parse(readFileSync(`${ROOT}/levels/pass.json`, 'utf8')), night = lum(openingLooks(L.sky.zenith, L.sky.horizon, 1, { intensity: 0.3, dir: [0, -0.6, 0], color: [0.6, 0.7, 1] }));   // (the level's own night, under its moon)
  const grey = of('overcast'), noon = of('midday'), dusk = of('dusk');
  check(grey > 0.3 && grey < 1.2 && noon > grey && dusk > 0.15 && night > 0 && night < 0.2 * dusk, 'an opening is as bright as what it looks out on, at every hour: a night\'s sky is a sky, and a small one', `${grey.toFixed(2)} under the grey sky, ${noon.toFixed(2)} at midday, ${dusk.toFixed(2)} at dusk, ${night.toFixed(4)} at night`);
  const l = { name: 'opening0', group: 'sky', kind: 3, ttl: -1, intensity: 0.6, peakIntensity: 0.6 };
  check(lightClass(l) === 'none' && lightDayK(l, 0.05, 1) === 1 && lightClass({ ...l, group: 'interior' }) === 'face' && lightDayK({ ...l, group: 'interior' }, 0.0002, 1) < 0.02, 'an opening\'s light is the hour\'s own: the day\'s rescale of lamps leaves it as it is (a lamp of the same strength is taken down)'); }

// EVERY LEVEL'S DOORS, TO A PORTAL (scene/openings.ts ajarOf: what is open of an opening is the narrowest way through it past what hangs in it). A shut
// leaf must read shut whatever the wall's thickness and wherever its jambs stand on the samples (one that read open would light a room through a shut
// door for every pass whose ray stops short of it), a leaf swung wide must leave most of its doorway, and every such opening must have found the box
// that ends it on each hand.   AJAR_BREAK=1 bun run tools/qa/openings-check.ts = no play allowed a shut leaf: this line must FAIL
{ const AJAR_BRK = process.env.AJAR_BREAK === '1', playWas = WAY.play; if (AJAR_BRK) WAY.play = 0;   // (AJAR_BREAK=1: a shut leaf is allowed no play in its frame: must FAIL)
  const { readdirSync } = await import('node:fs'); const names = readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).map(f => f.replace('.json', ''));
  let doors = 0, openShut = 0, lowWide = 0, jambless = 0; const worst: string[] = [];
  for (const name of names) { const S = await stand(name), { g, world } = S, set = (a: number) => { for (const d of g.doors.list) S.hang(d.def.name, a); for (let i = 0; i < 2; i++) { S.W.step(1 / 60); for (const d of g.doors.list) { d.vel = 0; d.angle = a; d.latched = a === 0; } } };
    const l = console.log, w = console.warn; console.log = console.warn = () => { };
    try { set(0); const plan = planOpenings(world.statics) as any[]; openAreas(plan, world.dynamics); const shut = plan.map(o => o.ajar as number);
      set(1.5); openAreas(plan, world.dynamics);
      plan.forEach((o, i) => { if (shut[i] >= o.full - 1e-6 && o.ajar >= o.full - 1e-6) return; doors++;   // (nothing hangs in it)
        if (!o.cut.jambs[0] || !o.cut.jambs[1]) jambless++;
        if (shut[i] > 0) { openShut++; worst.push(`${name} (${o.pos[0].toFixed(1)}, ${o.pos[2].toFixed(1)}) reads ${shut[i].toFixed(3)} m² open when shut`); }
        if (o.ajar < 0.6 * o.full) { lowWide++; worst.push(`${name} (${o.pos[0].toFixed(1)}, ${o.pos[2].toFixed(1)}) reads ${o.ajar.toFixed(2)} of ${o.full.toFixed(2)} m² at 1.5 rad`); } });
    } finally { console.log = l; console.warn = w; } }
  check(names.length >= 40 && doors >= 150 && openShut === 0 && lowWide === 0 && jambless === 0, 'to a portal every level\'s shut door is shut and its open door open: no leaf in its frame reads as a gap, a leaf swung wide leaves three fifths of its doorway or more, and every doorway found the boxes that end it',
    `${AJAR_BRK ? '(AJAR_BREAK) ' : ''}${names.length} levels, ${doors} openings a leaf hangs in · read open when shut: ${openShut} · under three fifths at 1.5 rad: ${lowWide} · with no box for a jamb: ${jambless}${worst.length ? ' · ' + worst.slice(0, 4).join('; ') : ''}`); 
  WAY.play = playWas; }
console.log(red ? '=== openings: FAILED ===' : '=== openings: ALL PASS ===');
process.exit(red ? 1 : 0);
