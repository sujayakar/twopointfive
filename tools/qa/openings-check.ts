// DAYLIGHT AT OPENINGS (the look page's row "Daylight at openings", lookConfig.windows; src/scene/openings.ts; engine.ts lookOpeningLights keeps the lights,
// in a list of its own that the game never reads and no checkpoint holds: engine.ts dayLights, handed to the light upload).
// One rule finds every place a level is open to the sky by, from its boxes as they stand: where floor under a roof meets floor under the sky with no wall
// between, and wall on both hands. Held here to a room built by hand (a wide window, high glass, a porch; an awning, a canopy, glass between two rooms and glass
// out of doors, which are none), to the pass (its windows, a door as it swings, its bay),
// to the office's wide and high windows, to two levels with awnings, to the smoke lab's floor, and to how bright one is by the hour.
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
}

// ---- the office: windows wider than a doorway and strips of glass over head height (the first cut of this rule lost both)
{ const S = await stand('office'); S.shut(); const { O } = S.read(), wide = O.filter(o => o.full > 2.5), high = O.filter(o => o.pos[1] > 2.2);
  check(O.length >= 9 && wide.length >= 5 && high.length >= 2, 'the office has every window the glass-only rule found there (nine) and more: its long windows and its strips of high glass among them', `${O.length} with every door shut: ${wide.length} of more than 2.5 m², ${high.length} over head height`); }
// ---- awnings: a stall's roof on posts is no room
{ const out: string[] = []; let ok = true;
  for (const name of ['gwot-yard', 'tut-quiet']) { const S = await stand(name); S.shut(); const { O, all } = S.read(); out.push(`${name}: ${O.length} open with every door shut, ${all.length - O.length} doorways`); if (O.some(o => o.full > 3) || O.length > 1) ok = false; }
  check(ok, 'a yard of market stalls gets no light from their awnings (the first cut lit four of five there)', out.join(' · ')); }

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
  const { readdirSync } = await import('node:fs'); const { sample, swept } = await import('./sweep.ts'); const names = sample(readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).map(f => f.replace('.json', '')), 8);
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
  check(names.length >= swept(8, 40) && doors >= swept(20, 150) && openShut === 0 && lowWide === 0 && jambless === 0, 'to a portal every level\'s shut door is shut and its open door open: no leaf in its frame reads as a gap, a leaf swung wide leaves three fifths of its doorway or more, and every doorway found the boxes that end it',
    `${AJAR_BRK ? '(AJAR_BREAK) ' : ''}${names.length} levels, ${doors} openings a leaf hangs in · read open when shut: ${openShut} · under three fifths at 1.5 rad: ${lowWide} · with no box for a jamb: ${jambless}${worst.length ? ' · ' + worst.slice(0, 4).join('; ') : ''}`); 
  WAY.play = playWas; }
console.log(red ? '=== openings: FAILED ===' : '=== openings: ALL PASS ===');
process.exit(red ? 1 : 0);
