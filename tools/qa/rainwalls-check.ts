// RAIN SHOWS IN FRONT OF A WALL'S OUTER FACE AND OVER ITS TOP — the check behind the scene prelude's RAIN_WALLS (src/render/frame.ts rainWallsConfig; on,
// ?norainwalls at boot = as it was; picture only: no game table, nothing for a recording to carry).
//   bun run tools/qa/checks.ts rainwalls        RAINWALLS_BREAK=1 = the rule off and the same lines asked: must FAIL
// the maintainer, 10-19, a depot's yard in the rain: "rain isn't showing up on top of walls properly". The rain is hidden from a pixel whose surface is indoors,
// and indoors was the roofs' bare rectangles: a level's own roof is its ceiling slab grown by centimetres, over the walls, so every outer wall's face, the door
// in it and the wall's top showed no rain and the streaks over the yard stopped dead at the wall behind them. The rule now asked: is the point UNDER the roofs,
// 0.2 m or more inside them on every side (common.wgsl wxUnderRoof).
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. The rule is written again below (`under`) and the shader's own function is held to it BY ITS TEXT, with the
// two places that ask it and the ring's own gate; the copy is then asked at a wall under a roof written here, grown over it as a level's is. A change to those
// shader lines is a red line here, not a silent pass. What the picture looks like is the page's to show.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT, main } from './here.ts';
export async function rainWallsCheck(): Promise<boolean> {
  const BRK = process.env.RAINWALLS_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== rainwalls: rain shows in front of a wall's outer face and over its top${BRK ? ' (the rule off: must fail)' : ''} ===`);
  const read = (f: string) => readFileSync(resolve(ROOT, 'src/render/shaders', f), 'utf8');
  const common = read('common.wgsl'), vol = read('volumetrics.wgsl'), draw = read('particles_draw.wgsl'), pc = read('particles_common.wgsl');
  const IN = Number(/const WX_WALL_IN: f32 = ([0-9.]+);/.exec(common)?.[1]);
  const body = (/fn wxUnderRoof\(p: vec2f\) -> bool \{\n([\s\S]*?)\n\}/.exec(common)?.[1] ?? '').replace(/\s+/g, ' ').trim();
  const WANT = 'if (!wxRoofed(p)) { return false; } if (!lv(WX_KINDS)) { return true; } if (frame.cardCfg.y == 0u) { return true; } let d = WX_WALL_IN; return wxRoofed(p + vec2f(d, d)) && wxRoofed(p + vec2f(-d, d)) && wxRoofed(p + vec2f(d, -d)) && wxRoofed(p - vec2f(d, d));';
  /** the shader's rule again: wxRoofed (the bare rectangles) and wxUnderRoof (own = the level's own roofs in force: the packed words are not 0) */
  const roofed = (R: number[][], x: number, z: number) => R.some(r => x >= r[0] && z >= r[1] && x <= r[2] && z <= r[3]);
  const under = (R: number[][], x: number, z: number, own = true) => roofed(R, x, z) && (!own || (roofed(R, x + IN, z + IN) && roofed(R, x - IN, z + IN) && roofed(R, x + IN, z - IN) && roofed(R, x - IN, z - IN)));
  const RULE = !BRK, shown = (R: number[][], x: number, z: number) => !roofed(R, x, z) || (RULE && !under(R, x, z));   // the two gates' own expression
  // (1) a wall, 24 cm thick, its outer face at x = 4, under a roof grown 7 cm past that face as a level's own roof is
  { const R = [[3.93, 2, 12, 10]]; let face = 0, inner = 0, n = 0; for (let z = 3; z < 9; z += 0.25) { n++; if (shown(R, 4, z)) face++; if (!shown(R, 4.24, z)) inner++; }
    verdict(face === n && inner === n, 'rain shows in front of the outer face of a wall under a roof grown over it, and not in front of its inner face',
      `outer face shown ${face} of ${n}, inner face dry ${inner} of ${n}`); }
  // (2) where two slabs of one building meet, and the office's own two rectangles
  { const two = [[4, 4, 12, 10.93], [4, 10.81, 12, 20]]; let seam = 0, n = 0; for (let x = 4.5; x < 11.6; x += 0.25) for (let z = 10.5; z < 11.3; z += 0.05) { n++; if (!shown(two, x, z)) seam++; }
    const office = [[4, 4, 36, 24], [1.1, 9.33, 4, 12.87]]; let same = 0, m = 0; for (let x = 0.05; x < 40; x += 0.37) for (let z = 0.05; z < 28; z += 0.37) { m++; if (under(office, x, z, false) === roofed(office, x, z)) same++; }
    verdict(seam === n && same === m, 'the strip where two roof slabs of one building meet is under the roof, and on the office\'s own two rectangles (no kind in force) the rule is the bare rectangle',
      `two slabs overlapping 12 cm: ${seam} of ${n} points across the seam dry · the office's rectangles: ${same} of ${m} points as the bare test says`); }
  // (3) who asks, and the switch
  { const F = await import(`${ROOT}/src/render/frame.ts`) as any, was = F.rainWallsConfig.on; let onT = '', offT = '';
    try { F.rainWallsConfig.on = true; onT = F.scenePrelude(); F.rainWallsConfig.on = false; offT = F.scenePrelude(); } finally { F.rainWallsConfig.on = was; }
    const main = readFileSync(`${ROOT}/src/host/pictureopens.ts`, 'utf8');
    const asked = /if \(depth >= 1\.0 \|\| outdoorsXZ\(Pend\.xz\) \|\| \(RAIN_WALLS && !wxUnderRoof\(Pend\.xz\)\)\) \{/.test(vol) && /else if \(RAIN_WALLS && rainOverWall\(Pend\)\) \{ \}[^\n]*\n\s*else if \(!rainOutdoors\(Pend\.xz\)\) \{ discard; \}/.test(draw) && /fn rainOverWall\(P: vec3f\) -> bool \{\s*let p = P\.xz;\s*return !wxUnderRoof\(p\) && /.test(pc);
    const rings = /if \(depth < 1\.0 && outdoorsXZ\(Pend\.xz\)\) \{ let sp = splashAt\(Pend\);/.test(vol);
    verdict(was === true && body === WANT && IN === 0.2 && asked && rings && /const RAIN_WALLS: bool = true;/.test(onT) && /const RAIN_WALLS: bool = false;/.test(offT) && /q\.has\('norainwalls'\)\) rainWallsConfig\.on = false;/.test(main),
      'the shader\'s rule is the one asked here; the lattice rain and the particle rain both ask it and a ring is drawn on open ground only; the prelude carries the constant, on as the page boots, and ?norainwalls lowers it before the shaders are built',
      `wxUnderRoof's text as held ${body === WANT} · drawn in ${IN} m · both gates ask ${asked} · rings behind the old test ${rings} · on at import ${was} · in the prelude on ${/const RAIN_WALLS: bool = true;/.test(onT)}, off ${/const RAIN_WALLS: bool = false;/.test(offT)} · the page's flag ${/norainwalls/.test(main)}`); }
  console.log(`=== rainwalls: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) await main([rainWallsCheck]);
