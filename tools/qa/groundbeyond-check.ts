// The look page's GROUND PAST THE LEVEL'S EDGE (src/scene/groundBeyond.ts; the address's `&beyond=1`, off unless asked), and THE RULE FOR A MAP'S EDGE that
// came with it (src/render/frame.ts MAP_EDGE, the prelude's pastMap and rimShare). On every level: the ground slab is found, one box is laid under it that
// reaches past the world on every side, in the slab's own paint and stuff, and no ray can meet it; nothing else of the level is touched, the level's
// collision does not file the box, and the box takes on the slab's look whenever the page asks. The edge rule's arithmetic is written again here and its
// lines in the three readers are held word for word, because the claim that nothing drawn on the map reads anything new rests on them.
// No GPU: what the picture makes of the box is the page's to show (never seen on a GPU). Nothing here reads the stand (host/profile.ts): one run is both.
//   bun run tools/qa/groundbeyond-check.ts   (exit 0 = all pass; GROUNDBEYOND_BREAK=1 lays the box as one the rays meet, and must fail twice: by its flag, and
//                                             by the collision that then files it under the world's cells)
import { readFileSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { parseLevelData } from '../../src/scene/levelData';
import { loadLevel } from '../../src/scene/levelLoad';
import { groundSlab, groundBeyond, groundBeyondFollow, BEYOND } from '../../src/scene/groundBeyond';
import { groundBeyondOn, lookPageWord } from '../../src/scene/look';
import { BoxFlag, makeBox, type Box } from '../../src/scene/boxes';
import { StaticCollision } from '../../src/scene/collision';
import { WORLD } from '../../src/scene/world';
import { scenePrelude, MAP_EDGE } from '../../src/render/frame';
let red = 0; const check = (ok: boolean, w: string, more = '') => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${w}${more ? `\n    ${more}` : ''}`); if (!ok) red++; };
const BREAK = process.env.GROUNDBEYOND_BREAK === '1', ROOT = resolve(import.meta.dir, '../..');
const Q = (s: string) => new URLSearchParams(s);
check(groundBeyondOn(Q('look&beyond=1')) && groundBeyondOn(Q('look=hours:1&beyond=1')) && !groundBeyondOn(Q('look')) && !groundBeyondOn(Q('look&beyond=0')) && !groundBeyondOn(Q('look&beyond')) && !groundBeyondOn(Q('beyond=1')) && !groundBeyondOn(Q('roomclear&squad4&beyond=1')) && lookPageWord('beyond', '1'),
  'the ground is laid past the edge only on the look page and only when the address says &beyond=1: the page as it opens, and every page that is not the look\'s, has the black it had; and the word is one of the look page\'s own, so the benchmark of a page opened with it measures the picture that page shows');
const COLLIDE = ['block', 'compound', 'depot', 'office', 'tut-road', 'white-roof'];   // (the six whose collision is stood up with the box in the list)
let reachPast = 0, reachWho = '';
{ const l0 = console.log, w0 = console.warn; let levels = 0, filed = 0, asked = 0; const none: string[] = [], bad: string[] = [], stuffs = new Map<string, number>(), out = new Int32Array(256);
  for (const name of readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint')).map(f => f.replace(/\.json$/, '')).sort()) {
    console.log = console.warn = () => { }; let L: { boxes: Box[] } | null = null; try { L = loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8'))); } catch { L = null; }
    console.log = l0; console.warn = w0; levels++; if (!L) { bad.push(`${name}: the level did not load`); continue; }
    for (const x of L.boxes) { if (x.flags & BoxFlag.NoPrimary) continue; const cs = Math.abs(Math.cos(x.yaw)), sn = Math.abs(Math.sin(x.yaw)), r = Math.hypot(x.h[0], x.h[1], x.h[2]), ex = x.rot ? r : cs * x.h[0] + sn * x.h[2], ez = x.rot ? r : sn * x.h[0] + cs * x.h[2];   // (a tilted box by the ball it turns in)   // (how far anything the file draws reaches past the map: the edge rule's near distance must cover it)
      const o = Math.max(WORLD.min[0] - (x.c[0] - ex), x.c[0] + ex - WORLD.max[0], WORLD.min[2] - (x.c[2] - ez), x.c[2] + ez - WORLD.max[2]); if (o > reachPast) { reachPast = o; reachWho = name; } }
    const before = L.boxes.slice(), was = before.map(b => JSON.stringify([b.c, b.h, b.flags, b.albedo])), g = groundSlab(L.boxes), pair = groundBeyond(L), b = pair?.box;
    if (b && BREAK) b.flags &= ~BoxFlag.NoShadow;
    if (b && COLLIDE.includes(name)) { asked++;   // (the collision, stood up with the box in the list as the page stands it up: is the box under any cell, and does a sweep under the floor return it)
      console.log = console.warn = () => { }; const col = new StaticCollision(L.boxes), at = L.boxes.length - 1; console.log = l0; console.warn = w0;
      if ((col as unknown as { cells: number[][] }).cells.some(c => c.includes(at))) filed++;
      for (const [x, z] of [[6, 6], [34, 6], [20, 14], [34, 22]]) { const n = col.gather(x - 1, z - 1, x + 1, z + 1, -1, -0.001, out); for (let k = 0; k < n; k++) if (out[k] === at) filed++; } }
    if (!g || !pair || !b) { none.push(name); if (L.boxes.length !== before.length) bad.push(`${name}: no slab, yet the level changed`); continue; }
    stuffs.set(String(g.stuff ?? 'plain'), (stuffs.get(String(g.stuff ?? 'plain')) ?? 0) + 1);
    const why: string[] = [];
    if (pair.slab !== g || L.boxes.length !== before.length + 1 || L.boxes[L.boxes.length - 1] !== b || before.some((x, i) => L!.boxes[i] !== x || JSON.stringify([x.c, x.h, x.flags, x.albedo]) !== was[i])) why.push('the level\'s own boxes moved');
    if (!(b.flags & BoxFlag.NoShadow) || b.flags & (BoxFlag.NoPrimary | BoxFlag.Dynamic) || (b.flags & BoxFlag.Gloss) !== (g.flags & BoxFlag.Gloss)) why.push('a ray could meet it, or it is not drawn, or its gloss is not the slab\'s');
    if (b.c[0] - b.h[0] > WORLD.min[0] - BEYOND.reach + 1e-6 || b.c[0] + b.h[0] < WORLD.max[0] + BEYOND.reach - 1e-6 || b.c[2] - b.h[2] > WORLD.min[2] - BEYOND.reach + 1e-6 || b.c[2] + b.h[2] < WORLD.max[2] + BEYOND.reach - 1e-6) why.push('it stops short of its reach');
    const top = b.c[1] + b.h[1], gtop = g.c[1] + g.h[1], plane = (y: number) => Math.floor(y * 100 + 0.31);   // (look.wgsl's `plane`: a face's pieces are dealt by its height in whole centimetres)
    if (!(top < gtop) || gtop - top >= 0.01 || Math.abs(gtop) > BEYOND.top || plane(top) !== plane(gtop)) why.push(`its top is ${(gtop - top).toFixed(4)} m under the slab's, or in another centimetre of the look's deal`);
    if (b.albedo === g.albedo || b.albedo.some((v, i) => v !== g.albedo[i]) || b.stuff !== g.stuff || b.rough !== g.rough || !!b.metal !== !!g.metal || b.marks || b.cut || b.rot || b.yaw !== 0 || b.owner !== 0) why.push('its paint is not the slab\'s own, or it carries more than paint');
    if (why.length) bad.push(`${name}: ${why.join('; ')}`); }
  check(levels >= 50 && bad.length === 0 && none.length === 0, `on every level the ground slab is found and laid once more a hair under itself (${BEYOND.drop * 1000} mm: still the ground to what asks by height, and the same centimetre to the look's deal of pieces), ${BEYOND.reach} m past the world on every side, in its own paint, gloss and stuff, as a box that is drawn and that no ray meets; the level's own boxes are the same objects with the same numbers`,
    `${levels} levels; ${none.length} with no ground slab${none.length ? `: ${none.join(', ')}` : ''}; ${bad.length} wrong${bad.length ? `: ${bad.slice(0, 4).join(' | ')}` : ''}; the slabs' stuff: ${[...stuffs].map(([k, n]) => `${k} ${n}`).join(', ')}`);
  check(asked === COLLIDE.length && filed === 0, 'the level\'s collision, built from the list with the box in it, has the box under no cell and returns it from no sweep under the floor: nothing that walks, falls, flies or is thrown can meet it', `${asked} levels' collisions (${COLLIDE.join(', ')}); the box found ${filed} times`); }
{ // what counts as the level's ground, on made-up lists
  const slab = (o: Partial<Box> = {}) => makeBox({ c: [20, -0.125, 14], h: [20, 0.125, 14], albedo: [0.3, 0.3, 0.3], ...o }), is = (b: Box) => groundSlab([b]) === b, no: string[] = [];
  const cases: [string, Box, boolean][] = [['the plain slab', slab(), true], ['a slab turned a quarter', slab({ h: [14, 0.125, 20], yaw: Math.PI / 2 }), true], ['a slab whose top is 4 cm down', slab({ c: [20, -0.165, 14] }), true],
    ['not drawn', slab({ flags: BoxFlag.NoPrimary }), false], ['met by no ray', slab({ flags: BoxFlag.NoShadow }), false], ['a moving box', slab({ flags: BoxFlag.Dynamic }), false], ['glass', slab({ flags: BoxFlag.Glass }), false], ['a stage wall', slab({ flags: BoxFlag.Cutaway }), false],
    ['tilted', slab({ rot: [0, 0, 0, 1] }), false], ['cut', slab({ cut: [{ n: [1, 0, 0], d: 0 }] as Box['cut'] }), false], ['turned off the axes', slab({ yaw: 0.3 }), false], ['a top 6 cm down', slab({ c: [20, -0.185, 14] }), false], ['a top 6 cm up', slab({ c: [20, -0.065, 14] }), false],
    ['a slab a metre short on one side', slab({ c: [19.5, -0.125, 14], h: [19.5, 0.125, 14] }), false], ['a quarter turn that leaves the world uncovered', slab({ yaw: Math.PI / 2 }), false]];
  for (const [w, b, want] of cases) if (is(b) !== want) no.push(w);
  const base = slab({ c: [20, -0.145, 14] }), sheet = slab({ c: [20, -0.01, 14], h: [20, 0.01, 14], albedo: [0.6, 0.5, 0.4] });
  if (groundSlab([base, sheet]) !== sheet || groundSlab([sheet, base]) !== sheet) no.push('of two, the sheet on top');
  check(no.length === 0, 'the level\'s ground is a plain box that is drawn and met by rays, square to the world, its top within 5 cm of the floor, covering the whole of the world\'s rectangle; of two such, the one on top', `${cases.length + 1} made-up lists${no.length ? `; wrong: ${no.join(', ')}` : ''}`); }
{ // a level with no slab the size of the world is left alone
  const deck = { boxes: [{ c: [20, -0.125, 14], h: [8, 0.125, 6], yaw: 0, albedo: [0.4, 0.4, 0.4], emissive: [0, 0, 0], flags: 0, owner: 0, cutHeight: 1e6 } as Box, { c: [20, 2, 14], h: [20, 0.1, 14], yaw: 0, albedo: [0.4, 0.4, 0.4], emissive: [0, 0, 0], flags: 0, owner: 0, cutHeight: 1e6 } as Box] };
  check(groundSlab(deck.boxes) === null && groundBeyond(deck) === null && deck.boxes.length === 2, 'a level whose floor is a deck smaller than the world, or whose only world-wide slab is a roof, has no ground to run on: nothing is laid'); }
{ // THE BOX LOOKS AS THE SLAB LOOKS NOW: the rain's two ways of wetting a slab (game/weather.ts soak: new paint, gloss, a finish; wetOnly: the render-only factor) and their drying, done by hand
  const g = makeBox({ c: [20, -0.125, 14], h: [20, 0.125, 14], albedo: [0.4, 0.3, 0.2], rough: 0.8 }), L = { boxes: [g] }, pair = groundBeyond(L)!, b = pair.box, no: string[] = [];
  const same = () => b.albedo !== g.albedo && b.albedo.every((v, i) => v === g.albedo[i]) && b.wetK === g.wetK && b.rough === g.rough && b.aniso === g.aniso && b.flags === (BoxFlag.NoShadow | (g.flags & BoxFlag.Gloss)) && ('wetK' in b) === (g.wetK !== undefined);
  if (!same()) no.push('as laid');
  const dry = { albedo: g.albedo, flags: g.flags, rough: g.rough };
  g.albedo = [0.2, 0.15, 0.104]; g.flags |= BoxFlag.Gloss; g.rough = 0.08; g.aniso = true;
  if (same()) no.push('it changed before the page asked'); groundBeyondFollow(pair); if (!same()) no.push('soaked');
  b.wetK = 0.5; b.rough = 0.3; b.flags |= BoxFlag.Dynamic; groundBeyondFollow(pair); if (!same()) no.push('written over by another hand');   // (the weather's sweep of open slabs writes the box's own look: the slab's wins)
  g.albedo = dry.albedo; g.flags = dry.flags; g.rough = dry.rough; delete g.aniso; groundBeyondFollow(pair); if (!same() || b.aniso !== undefined) no.push('dried');
  g.wetK = 0.5; g.flags |= BoxFlag.Gloss; groundBeyondFollow(pair); if (!same() || b.wetK !== 0.5) no.push('the render-only wet'); delete g.wetK; g.flags = dry.flags; groundBeyondFollow(pair); if (!same()) no.push('and its drying');
  check(no.length === 0 && L.boxes.length === 2, 'the laid box takes on the slab\'s look whenever the page asks, and only then: its paint by value, the render-only wet, its gloss, its roughness and its stretched lobe, soaked and dried both ways, over whatever else wrote the box; it stays a box no ray meets', no.length ? `wrong: ${no.join(', ')}` : '5 states'); }
{ // THE MAP'S EDGE: the prelude's two lines, their arithmetic written again, and the readers' lines word for word
  const rd = (f: string) => readFileSync(resolve(ROOT, f), 'utf8'), pre = scenePrelude(), f = (n: number) => (Number.isInteger(n) ? n.toFixed(1) : String(n));
  const lines = pre.includes('fn pastMap(xz: vec2f) -> f32 { return length(max(max(WORLD_MIN.xz - xz, xz - WORLD_MAX.xz), vec2f(0.0))); }') && pre.includes(`fn rimShare(past: f32) -> f32 { return 1.0 - smoothstep(${f(MAP_EDGE.near)}, ${f(MAP_EDGE.far)}, past); }`);
  const past = (x: number, z: number) => Math.hypot(Math.max(WORLD.min[0] - x, x - WORLD.max[0], 0), Math.max(WORLD.min[2] - z, z - WORLD.max[2], 0)), F = Math.fround;
  const share = (p: number) => { const t = Math.min(1, Math.max(0, F(F(p - MAP_EDGE.near) / F(MAP_EDGE.far - MAP_EDGE.near)))); return F(1 - F(F(t * t) * F(3 - F(2 * t)))); };   // (WGSL's smoothstep, in single floats)
  const on = [[0, 0], [40, 28], [20, 14], [0, 28], [39.99, 0.01]].every(([x, z]) => past(x, z) === 0 && share(past(x, z)) === 1), near = [[-0.3, 14], [40.46, 3], [20, 28 + MAP_EDGE.near], [-0.7 * MAP_EDGE.near, -0.7 * MAP_EDGE.near]].every(([x, z]) => past(x, z) > 0 && share(past(x, z)) === 1);
  const mid = share((MAP_EDGE.near + MAP_EDGE.far) / 2), far = [[-MAP_EDGE.far, 14], [20, 28 + MAP_EDGE.far], [200, -160]].every(([x, z]) => share(past(x, z)) === 0), round = Math.abs(past(-3, -4) - 5) < 1e-9 && share(past(-MAP_EDGE.near, -MAP_EDGE.near)) < 1; let mono = true; for (let p = 0, was = 1; p <= 6; p += 0.01) { const s = share(p); if (s > was || s < 0) mono = false; was = s; }
  check(lines && on && near && Math.abs(mid - 0.5) < 1e-6 && far && round && mono && reachPast <= MAP_EDGE.near * 0.5,
    `a map of the level is let go of past its edge by one rule (the prelude): the rim's answer times EXACTLY 1 on the map and for ${MAP_EDGE.near} m past it, falling to 0 at ${MAP_EDGE.far} m and never rising, by the true distance to the map's rectangle (round at a corner); and nothing a level FILE draws reaches half that near distance past the map (what the look page's trials and a man's reach add is not counted here), so no pixel of a level as filed reads anything new`,
    `the prelude's two lines ${lines} · on the map exactly 1 ${on} · a man's reach past it exactly 1 ${near} · halfway ${mid.toFixed(4)} · none from the far distance ${far} · round at a corner ${round} · never rising ${mono} · the furthest a file's drawn box reaches past the map: ${reachPast.toFixed(3)} m (${reachWho})`);
  const page = rd('src/main.ts'), comp = rd('src/render/shaders/composite.wgsl'), twin = rd('src/render/shaders/compact/composite.wgsl'), wet = rd('src/game/weather.ts'), look = rd('src/render/shaders/look.wgsl'), detail = rd('src/render/shaders/detail.wgsl'), snow = rd('src/render/snowField.ts');
  const iBoot = page.indexOf('async function bootLevel('), iLay = page.indexOf('if (groundBeyondOn(q)) { beyond = groundBeyond(level);'), iMark = page.indexOf('level.markTexel', iLay), iRet = page.indexOf('\n  return level;\n}', iBoot);
  const laid = iBoot > 0 && iLay > iBoot && iMark > iLay && iRet > iMark;   // (inside the stand-up, before the level's boxes are marked and handed on)
  const world = rd('src/scene/boxes.ts');
  const drawn = page.includes('  if (groundBeyondOn(q)) engine.world.beforePack = () => { if (beyond) groundBeyondFollow(beyond); };') && (page.match(/groundBeyondFollow\(beyond\)/g) ?? []).length === 1 && world.includes('  upload(between = false) {\n    this.beforePack?.();\n') && (world.match(/beforePack/g) ?? []).length === 2;
  const again = /level = buildBundledLevel\(\);[^\n]*\n    beyond = groundBeyondOn\(q\) \? groundBeyond\(level\) : null;[^\n]*\n    populateHost\(engine, level\);/.test(page);
  check(laid && drawn && again
    && comp.includes('color = skyRadiance(rdSky) * (1.0 - clamp(-rdSky.y * 8.0, 0.0, 1.0));') && !/BEYOND|groundBeyond|ground_beyond/.test(wet),
    'the page lays it inside the level\'s stand-up, before the boxes are marked and handed to the world, and again on the level it falls back to when an editor\'s level will not stand; it makes the box look as the slab does in ONE place, the head of the world\'s pack, so every picture the page draws (a frame, a still, a replay\'s) is packed with the slab\'s look as it is then, and no page that does not ask has the hook; the composite\'s own word for a ray that meets nothing under the horizon is still black; and the weather\'s text does not name the box', `laid in the stand-up ${laid} · followed at every pack, by one hook ${drawn} · laid again on the fallback ${again}`);
  const bare = (t: string) => t.replace(/^\s*\/\/[^\n]*\n/gm, '');   // (the kernel without its whole-line comments)
  const levels = 'fn lookPlaceLevels(P: vec3f, N: vec3f, skip: u32) -> vec2f {\n  let pick = lookPlaceField(P + N * 0.3); if (pick.x > -500.0 && pick.x - pick.y < 0.5) { return vec2f(exp2(pick.x), exp2(pick.z)); }\n  let here = lookPlaceHere(P + N * 0.3, skip);\n';
  const gain = 'fn lookShadeGain(P: vec3f, N: vec3f, Eown: f32, skip: u32) -> f32 {\n  if (!(frame.eyeCfg.w < 0.0)) { return 1.0; }\n'
    + '  let rim = rimShare(pastMap(P.xz));\n'
    + '  var lv = vec2f(select(frame.shadeCfg.y, exp2(placeWord(2)), placeWord(1) > 0.5 && placeWord(2) > -500.0));\n'
    + '  if (rim > 0.0) { let open = lv; lv = lookPlaceLevels(P, N, skip); if (rim < 1.0) { lv = exp2(mix(log2(max(open, vec2f(1e-9))), log2(max(lv, vec2f(1e-9))), rim)); } }\n'
    + '  return lookExpoGain(lv.x, Eown, lv.y);\n}\n';
  const kernel = (t: string) => { const b = bare(t).replace(/ +\/\/ \([^\n]*\n/g, '\n'); return b.includes(levels) && b.includes('  return vec2f(h, near);\n}\n') && b.includes(gain) && (t.match(/rimShare\(/g) ?? []).length === 1 && (t.match(/lookPlaceLevels\(/g) ?? []).length === 2 && (t.match(/return lookExpoGain\(/g) ?? []).length === 1; };   // (whole-line comments and the trailing ones out)
  check(kernel(comp) && kernel(twin)
    && look.includes('fn lookWayAt(xz: vec2f) -> vec2f { return textureSampleLevel(wearTex, linSamp, (xz - WORLD_MIN.xz) / (vec2f(f32(GRID_W), f32(GRID_H)) * GRID_CELL), 0.0).xy * rimShare(pastMap(xz)); }') && (look.match(/rimShare\(/g) ?? []).length === 1 && (look.match(/textureSampleLevel\(wearTex/g) ?? []).length === 1
    && detail.includes('textureSampleLevel(wearTex, linSamp, uv, 0.0).xy * rimShare(pastMap(wp.xz)), k, uneven);') && (detail.match(/rimShare\(/g) ?? []).length === 1
    && /snowPaintOf\(b\) === 'snow' && !\(b\.flags & BoxFlag\.NoShadow\)/.test(snow),
    'the three readers of a level map hold the rule as written (read as text): in both composite kernels the two levels a place is shown by are the levels they were unless the share is under 1, then let go of toward the open air\'s in stops (the field\'s level for it, the frame\'s own until the field is live), and from the far distance nothing is read at all, with one way out to the model; the look\'s one reader of the walked ways, and the detail layer\'s wear, are their reads times the share at the point read; and the snow\'s own ground is still the slab'); }
process.exit(red ? 1 : 0);
