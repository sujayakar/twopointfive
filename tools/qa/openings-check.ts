// DAYLIGHT AT OPENINGS (the look page's row "Daylight at openings", lookConfig.windows; src/scene/openings.ts; engine.ts lookOpeningLights keeps the lights,
// in a list of its own that the game never reads and no checkpoint holds: engine.ts dayLights, handed to the light upload).
// One rule finds every place a level is open to the sky by, from its boxes as they stand: where floor under a roof meets floor under the sky with no wall
// between, and wall on both hands. Held here to a room built by hand (a wide window, high glass, a porch; an awning, a canopy, glass between two rooms and glass
// out of doors, which are none), to the pass (its windows, a door as it swings, its bay, a hole a real breaching charge blows, the engine's lights as a door
// opens and shuts), to the office's wide and high windows, to two levels with awnings, to the smoke lab's floor, and to how bright one is by the hour.
// No GPU: the picture is never SEEN here.
//   bun run tools/qa/openings-check.ts                    exit 0 = every line passed, 1 = one failed (run it on both stands: TPS_PROFILE=shipped in front)
//   OPENINGS_BREAK=1 bun run tools/qa/openings-check.ts   glass is taken for wall: must FAIL
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
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const BRK = process.env.OPENINGS_BREAK === '1'; let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== openings: where a level is open to the sky${BRK ? ' (glass taken for wall: must fail)' : ''} ===`);
type O = { pos: number[]; dir: number[]; ajar: number; mid: [number, number, number]; half: [number, number]; full: number; radius: number };
const say = (o: O) => `(${o.pos[0].toFixed(2)}, ${o.pos[2].toFixed(2)}) ${o.ajar.toFixed(2)} m²`;
const stand = async (name: string) => { const l = console.log, w = console.warn; console.log = console.warn = () => { };
  const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 10; i++) W.step(1 / 60); console.log = l; console.warn = w;
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
{ const S = await stand('pass'), { g } = S; S.shut();
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
  { const { Engine } = await import(`${ROOT}/src/engine.ts`), { REACH_PACE, REACH_WORK } = await import(`${ROOT}/src/render/reach.ts`), { spawnUnit } = await import(`${ROOT}/src/game/guards.ts`), { stateHash } = await import(`${ROOT}/src/game/rollback.ts`);
    const LS = g.engine.lights;
    const mk = () => Object.assign(Object.create(Engine.prototype), { lights: LS, world: g.engine.world, settings: { skyZenith: HOURS.overcast.sky.zenith, skyHorizon: HOURS.overcast.sky.horizon, exposure: HOURS.overcast.exposure }, viewFocus: [20, 0, 10], openRecs: [], dayLights: [], openLevel: null, openSerial: -1, openTick: 0, openSeen: NaN, openPlanned: NaN, openWait: 0, openLeaves: NaN, openWalls: null }) as any;
    const E = mk();
    const was = { on: lookConfig.on, windows: lookConfig.windows, rooms: lookConfig.rooms, eye: lookConfig.eyeAt, gaps: lookConfig.doorGaps, lit: LOOK_OPENINGS.lit };
    const quiet = <T>(f: () => T): T => { const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
    try { lookConfig.on = true; lookConfig.windows = 1; lookConfig.rooms = 1;
      let eye: [number, number] = [20, 10]; lookConfig.eyeAt = () => eye;
      lookConfig.doorGaps = () => g.doors.list.map((d: any) => { const [hx, hz] = d.def.hinge, a = d.def.closedDir, wd = d.def.width; return [hx, hz, hx + Math.cos(a) * wd, hz + Math.sin(a) * wd]; });
      // (a drawn frame: the engine's pass alone. The game is NOT stepped between them: behind the start card and on a paused page it is not either)
      const frames = (n: number) => quiet(() => { for (let i = 0; i < n; i++) E.lookOpeningLights(); });
      const day = () => E.dayLights as any[], lit = () => day().filter(l => l.enabled), game = () => (LS.lights as any[]).map(l => l.name).join(','), reachN = () => (g as any).reachLights.length + (g as any).reachQueue.length;
      const before = { list: game(), reach: reachN(), steady: (LS.lights as any[]).filter(l => l.ttl < 0).length, hash: quiet(() => stateHash(g)) };
      S.shut(); const s0 = REACH_WORK.steps; frames(1); const first = lit().length, firstMasks = day().filter(l => l.reach).length, s1 = REACH_WORK.steps; let most = 0, n = 1;
      while (day().some(l => !l.reach) && n < 400) { const c = REACH_WORK.steps; frames(1); most = Math.max(most, REACH_WORK.steps - c); n++; }
      const lit0 = lit(), used0 = day().slice(), sAll = REACH_WORK.steps; frames(5); const quietAfter = REACH_WORK.steps === sAll, hash1 = quiet(() => stateHash(g));
      const tea = g.doors.byName('tea'), [hx, hz] = tea.def.hinge, mx = hx + Math.cos(tea.def.closedDir) * tea.def.width * 0.5, mz = hz + Math.sin(tea.def.closedDir) * tea.def.width * 0.5, doorL = used0.find(l => Math.hypot(l.pos[0] - mx, l.pos[2] - mz) < 0.5);
      S.hang('tea', 1.5); quiet(() => S.W.step(1 / 60)); frames(1); const open = !!doorL?.enabled, I = doorL?.intensity ?? 0, mask = doorL?.reach, range1 = doorL?.range, pos1 = JSON.stringify(doorL?.pos);
      S.hang('tea', 0); quiet(() => S.W.step(1 / 60)); frames(1); const shutOut = !!doorL && !doorL.enabled && doorL.range === range1 && doorL.reach === mask && JSON.stringify(doorL.pos) === pos1 && REACH_WORK.steps === sAll;
      check(used0.length === all.length && lit0.length === O.length && first === O.length && firstMasks === all.length && most <= REACH_PACE.steps * 1.6 && quietAfter && !!doorL && !lit0.includes(doorL) && open && I > 0.3 && !!mask && shutOut,
        `the engine keeps one light an opening: on a level\'s first frame EVERY opening has its reach mask whole, the shut ones too (since 11-05: a door opened on the next frame finds its mask made), and those open are lit; after it none is decided but a frame\'s share of the work at a time (the game\'s own pace for a lamp\'s), with the game standing still as it does behind the start card; a door\'s comes on as it opens and goes out as it shuts, and neither moves it, its range or its mask`,
        `${used0.length} openings · ${first} lit and ${firstMasks} masked on the first frame (${s1 - s0} sight-line steps) · all ${used0.length} masks whole ${n} frames on, ${most} steps in the longest of them (the pace is ${REACH_PACE.steps}) and none after · the tea door open: ${I.toFixed(2)}`);
      // DAYLIGHT COUNTED ONCE (render/lights.ts LIGHT_PORTAL; tools/qa/portal-check.ts asks what the light is): each light is the whole rectangle its
      // walls leave with the sky's reference divided back out, lit under a night's sky too. The only way since 11-04 (the disks and their row are gone)
      { const lumOf = (c: number[]) => 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2], daySky = E.settings, recs = E.openRecs as any[], sunL = (LS.lights as any[]).find(l => l.kind === 2 && l.enabled && !l.transient && l.intensity > 0), refLum = lumOf(openingLooks(daySky.skyZenith, daySky.skyHorizon, lookConfig.sky, sunL));
        frames(1); const P = lit(), masked = day().every(l => !!l.reach);
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
      { const st = g.engine.world.statics as any[], wall = st.find(b => !b.rot && b.h[0] < 0.2 && b.h[1] > 1.2 && b.h[2] > 1.6 && Math.abs(b.c[0] - 34) < 0.2 && Math.abs(b.c[2] - 8) < b.h[2] - 1.2), keepL = lit0[0], oldMask = keepL.reach;
        const c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0, bump = () => { (g.engine.world as any).geomSerial = ((g.engine.world as any).geomSerial ?? 0) + 1; };
        // (a hole 1.5 m wide in the middle of the store's outer wall: the wall is cut short of it and a second piece stands past it)
        wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; st.push({ ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] }); bump();
        // (and what the frames of a blast may cost: no jump field made again where the walls only opened, and no frame over its share of sight-line steps,
        //  the hole's own first mask included. Until 11-05 that mask was decided whole on the frame the walls were read, this hole's 327 thousand steps, and the field made beside it)
        const fields0 = REACH_WORK.fields; let most = 0; const one = () => { const s0 = REACH_WORK.steps; frames(1); most = Math.max(most, REACH_WORK.steps - s0); };
        one(); const waits = day().length === used0.length; for (let i = 0; i < LOOK_OPENINGS.settle + 1; i++) one();
        const holeL = day().find(l => !used0.includes(l)), kept = day().includes(keepL) && keepL.enabled && !!keepL.reach, dark = !!holeL && (!holeL.enabled || !!holeL.reach); let k = 0; while (!(holeL?.enabled && holeL.reach) && k++ < 400) one(); const late = k; k = 0; while (day().some(l => !l.reach) && k++ < 400) one(); for (let i = 0; i < 80; i++) one();
        const fieldsMade = REACH_WORK.fields - fields0, noField = fieldsMade === 0, paced = most <= REACH_PACE.steps * 1.6;   // (1.6: a share and the cell that crossed it, as the pace's own line above allows)
        const litHole = !!holeL?.enabled && !!holeL.reach, fresh = keepL.reach !== oldMask, at = holeL ? `at (${holeL.pos[0].toFixed(2)}, ${holeL.pos[2].toFixed(2)}) ${holeL.intensity.toFixed(2)}` : 'none';
        st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); frames(LOOK_OPENINGS.settle + 2); const gone = !!holeL && !day().includes(holeL) && day().length === used0.length;
        check(waits && !!holeL && kept && dark && litHole && fresh && gone && noField && paced, `a wall holed is a new opening ${LOOK_OPENINGS.settle} frames after it has stood still (never on the frame of the blast): its light comes on with its mask whole, a few frames on, the others stay lit by their old masks until theirs are decided again; no frame of it spends more than its share of sight-line steps and no jump field is made again; the wall put back, its light is gone`,
          `the hole\'s light ${at} · not on the first frame: ${waits} · a window\'s light kept and lit throughout: ${kept} · its mask decided again: ${fresh} · lit: ${litHole}, ${late} frames after the frames that found it · the most steps in a frame ${most} (a share is ${REACH_PACE.steps}) · jump fields made for the hole ${fieldsMade} · gone with the wall back: ${gone}`); }
      // WHAT IS LIT STAYS LIT, NO JOB IS LOST, AND A FIELD IS NOT HANDED ON PAST A WALL THAT CLOSED (11-05, from a review of the paced masks: each of these
      // was wrong in the first cut of them, and none was asked)
      { const st = g.engine.world.statics as any[], recs = () => E.openRecs as any[], bump = () => { (g.engine.world as any).geomSerial = ((g.engine.world as any).geomSerial ?? 0) + 1; };
        const settle = () => { let k = 0; do frames(1); while (recs().some(r => !r.mask || r.old || r.job) && k++ < 900); frames(LOOK_OPENINGS.settle + 2); k = 0; while (recs().some(r => !r.mask || r.old || r.job) && k++ < 900) frames(1); };
        const wall = st.find(b => !b.rot && b.h[0] < 0.2 && b.h[1] > 1.2 && b.h[2] > 1.6 && Math.abs(b.c[0] - 34) < 0.2 && Math.abs(b.c[2] - 8) < b.h[2] - 1.2), c0 = wall.c[2], h0 = wall.h[2], z0 = c0 - h0, z1 = c0 + h0, f: string[] = [], said: string[] = [];
        settle(); const known = new Set(day());
        // a hole, and a static that is no wall nudged every five frames from the frame it is found (a crate shot to bits, a pane gone: the openings are found again each time)
        wall.c[2] = (z0 + c0 - 0.75) / 2; wall.h[2] = (c0 - 0.75 - z0) / 2; const second = { ...wall, c: [wall.c[0], wall.c[1], (c0 + 0.75 + z1) / 2], h: [wall.h[0], wall.h[1], (z1 - c0 - 0.75) / 2] }; st.push(second); bump();
        const crate = st.find(b => b !== wall && b !== second && b.h[1] < 0.6 && b.c[1] < 1.2), y0 = crate.c[1]; let flip = false, litAt = -1;
        for (let k = 1; k <= 200 && litAt < 0; k++) { if (k >= 6 && (k - 6) % 5 === 0) { flip = !flip; crate.c[1] = y0 + (flip ? 0.02 : 0); bump(); } frames(1); const h = day().find(l => !known.has(l)); if (h?.enabled && h.reach) litAt = k; }
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
          let found = -1, litOn = -1; for (let k = 1; k <= 60 && litOn < 0; k++) { frames(1); const h = day().find(l => !known2.has(l)); if (h && found < 0) found = k; if (h?.enabled && h.reach) litOn = k; }
          REACH_PACE.whole = false; st.pop(); wall.c[2] = c0; wall.h[2] = h0; bump(); settle();
          if (!(found > 0 && litOn === found)) f.push(`with the old pace asked for, the hole was found on frame ${found} and lit on frame ${litOn} (want the same frame)`); else said.push('the old pace asked for: the hole lit on the frame it is found'); }
        check(f.length === 0, 'what is lit stays lit and no job is lost: a hole\'s light comes on though the openings are found again every five frames; a hole widened, and the wall put back, never show fewer lit; a box set down makes the field fresh and every mask a fresh raster\'s; the first frame masks every opening, on a second level too; the old pace asked for is the old pace', f.join('; ') || said.join(' · ')); }
      // the rooms row put off and on, and put on for the first time in the middle of a level: no frame with the rooms dark
      { const n0 = lit().length; lookConfig.rooms = 0; frames(1); const off = lit().length, bare = lit().every(l => !l.reach); lookConfig.rooms = 1; frames(1); const on = lit().length, masked = lit().every(l => !!l.reach);
        lookConfig.rooms = 0; const E2 = mk(); quiet(() => { E2.lookOpeningLights(); E2.lookOpeningLights(); }); const a2 = (E2.dayLights as any[]).filter(l => l.enabled).length; lookConfig.rooms = 1; quiet(() => E2.lookOpeningLights()); const b2 = (E2.dayLights as any[]).filter(l => l.enabled && l.reach).length;
        check(n0 === O.length && off === n0 && bare && on === n0 && masked && a2 === n0 && b2 === n0, 'with "Light stays in its room" put off the lights are lit with no mask, and put on again, or on for the first time in the middle of a level, every one is lit on that same frame with its mask', `${n0} lit · the row off ${off} · on again ${on} · a level begun with the row off ${a2}, the row put on ${b2}`); }
      lookConfig.windows = 0; frames(1); check(day().length === 0, 'the row at 0 takes every one of them out');
    } finally { lookConfig.on = was.on; lookConfig.windows = was.windows; lookConfig.rooms = was.rooms; lookConfig.eyeAt = was.eye; lookConfig.doorGaps = was.gaps; LOOK_OPENINGS.lit = was.lit; } }
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

// ---- the smoke lab's floor: a window hole with no glass, a passage's mouth
{ const S = await stand('smoke-lab'); S.shut(); const a = S.read();
  for (const d of S.g.doors.list) S.hang(d.def.name, 1.5); const b = S.read();
  const hole = a.O.find(o => o.full < 2), mouth = b.O.filter(o => Math.abs(o.pos[2] - 22) < 0.5 && o.pos[0] > 6 && o.pos[0] < 8.6);
  check(a.O.length === 2 && !!hole && Math.abs(hole.ajar - 0.8) < 0.06 && Math.abs(hole.pos[0] - 16) < 0.1 && Math.abs(hole.pos[1] - 1.5) < 0.06 && hole.dir[2] === -1 && inward(a.m, hole), 'a window hole with no glass is an opening by the same rule, measured as the hole (0.8 by 1.0 m, its middle 1.5 m up)', a.O.map(say).join(' · '));
  check(b.O.length === 6 && mouth.length === 1 && mouth[0].dir[2] === -1 && Math.abs(mouth[0].full - 7.2) < 0.4 && b.O.every(o => inward(b.m, o)), 'with every door open the lab has six (the hole, four doors to the yard and the tunnel\'s mouth between its two walls); the door between its rooms is none', b.O.map(say).join(' · ')); }

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
