// THE LEVEL'S OWN LOOK RUST SAYS IS THE ONE THE COPY OF THE GAME MADE: the traffic map (render/wear.ts buildWearMap), the door leaves as drawn (scene/leafPaint.ts paintLeaves: Rust's game has them from the stand's export here, and tools/blitz/start-levels.ts holds its own rule) and the dust's table (game/particleFx.ts DUST_LOOK), each
// asked of both on one Rust game stood under the headless host. It goes with the copy.
//   tools/blitz/rust.sh rust bun run tools/draw/look-check.ts [level…]     default: the levels below, each in the look's file where it has one
//   LOOK_BREAK=1 …    the copy's map is a byte off, and a leaf's paint a shade: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Section, DustLook, Mat, Want } from '../../src/draw/sections';
import { Drawn } from '../../src/draw/drawn';
import { lookNumbers } from '../../src/draw/look';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { lookConfig } = await import(`${ROOT}/src/render/frame.ts`), { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`), { roundConfig } = await import(`${ROOT}/src/scene/round.ts`), { lookWordOf, lookCoreOf } = await import(`${ROOT}/src/render/gbuffer.ts`), { COATFINISH, SOLIDGLASS } = await import(`${ROOT}/src/scene/stuff.ts`);
const { buildWearMap } = await import(`${ROOT}/src/render/wear.ts`), { LOOK } = await import(`${ROOT}/src/scene/look.ts`), { leafRowOf, paintLeaves } = await import(`${ROOT}/src/scene/leafPaint.ts`), { levelNameOf } = await import(`${ROOT}/src/scene/levelLoad.ts`), { DUST_LOOK } = await import(`${ROOT}/src/game/particleFx.ts`);

const LEVELS = process.argv.slice(2).length ? process.argv.slice(2) : ['office', 'compound-gwot', 'block-gwot', 'mousehole-gwot', 'glasshouse', 'killhouse-gwot', 'records-gwot'];
const BREAK = process.env.LOOK_BREAK === '1', DT = 1 / 60;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };

lookConfig.on = true; LOOK.on = true; cutConfig.on = true; roundConfig.gpu = true; COATFINISH.on = true; SOLIDGLASS.on = true;

for (const name of LEVELS) {
  const level = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8')));
  const W: any = await quiet(() => standUp({ level, seed: 3 }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&roomclear&squad4`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, E = g.engine, world = E.world; const said: string[] = [];
  world.setStuffOf(lookWordOf(), lookCoreOf());
  g.godMode = true; await quiet(() => W.step(DT));
  const H: any = RustHost.last; if (!H) { line(false, `${name}: no Rust game stands`); continue; }
  const X = H.F.X, D = new Drawn(X, H.F.mem, Want.all), v = lookNumbers(E.settings, world, !!g.ch.cast); v[6] = v[7] = 0;
  { const a = X.face_alloc(v.length * 8) >>> 0; new Float64Array(H.F.mem.buffer, a, v.length).set(v); X.face_draw_look(H.h, a, v.length); X.face_free(a, v.length * 8); }
  // the map
  const wm = buildWearMap(level, g.col.nav, true, true); if (BREAK) wm.data[1000] ^= 8;
  D.take(H.h); D.frame(H.h, 1, [20, 0, 14]);
  const bytes = new Uint8Array(D.u.buffer, D.at[Section.traffic] * 4, D.n[Section.traffic] * 4);
  if (bytes.length !== wm.data.length) said.push(`the map's size: copy ${wm.data.length} rust ${bytes.length}`);
  else { let off = 0, worst = 0, at = -1, sum = 0; for (let i = 0; i < bytes.length; i++) { const d = Math.abs(bytes[i] - wm.data[i]); sum += wm.data[i]; if (d) { off++; if (d > worst) { worst = d; at = i; } } }
    // (a texel's sum of float32 stamps in another order may round to the next byte)
    if (worst > 1) said.push(`the map: ${off} bytes off, the worst by ${worst} at ${at} (${at & 1 ? 'band' : 'traffic'}, texel ${at >> 1}: copy ${wm.data[at]} rust ${bytes[at]})`); else if (!sum) said.push('the copy\'s map is empty');
    else log(`    ${name}: map sum ${sum}, ${off} bytes a step off of ${bytes.length}`); }
  // the leaves' paint
  // (the leaves were painted as the game stood. This game's Rust is made FROM the stand's export, so its leaves are the copy's by the bytes handed over: what this holds is that
  // the stand painted them and that the rows draw the box. tools/blitz/start-levels.ts holds Rust's own rule, doors/paint.rs, to the copy's, level by level)
  const asked = g.doors.list.map((d: any) => ({ def: d.def, box: { ...d.box, albedo: [...d.box.albedo] } })), painted: number = paintLeaves(asked, world.statics, leafRowOf(levelNameOf(level)));
  asked.forEach((a: any, i: number) => { const b = g.doors.list[i].box; if (a.box.albedo.some((x: number, k: number) => x !== b.albedo[k]) || a.box.stuff !== b.stuff) said.push(`${a.def.name}: stood in ${b.albedo} ${b.stuff}, its paint is ${a.box.albedo} ${a.box.stuff}`); });
  if (BREAK && g.doors.list[0]) g.doors.list[0].box.albedo[0] += 0.1;
  world.glassFocus = [20, 0, 14]; world.upload();
  D.take(H.h); D.frame(H.h, 1, [20, 0, 14]);
  const mat = new Float32Array(world.matData.buffer), m0 = D.at[Section.mat]; let leaves = 0;
  for (const d of g.doors.list) { const i = world.all.indexOf(d.box); if (i < 0) continue; leaves++; for (const k of [0, 1, 2, 7, 12, 13, 14]) { const a = mat[i * Mat.row + k], b = D.f[m0 + i * Mat.row + k]; if (Math.abs(a - b) > 2e-5) { said.push(`door ${d.def.name} (box ${i}), word ${k}: copy ${a} rust ${b}`); break; } } }
  // the dust's table
  { const o = D.at[Section.dust], t: [string, number, number][] = [['opacity', DUST_LOOK.opacity, DustLook.opacity], ['growth', DUST_LOOK.growth, DustLook.growth], ['fadeIn', DUST_LOOK.fadeIn, DustLook.fadeIn], ['soft', DUST_LOOK.soft, DustLook.soft], ['wrap', DUST_LOOK.wrap, DustLook.wrap], ['ambient', DUST_LOOK.ambient, DustLook.ambient]];
    for (const [w, a, k] of t) if (Math.abs(a - D.f[o + k]) > 1e-6) said.push(`dust.${w}: copy ${a} rust ${D.f[o + k]}`); }
  line(said.length === 0, `${name}: the level's look is the copy's (${painted} of ${leaves} leaves painted)`, said.slice(0, 12).join('\n    '));
}
console.log(fails ? `=== look: ${fails} FAILED ===` : '=== look: ALL PASS ===');
process.exit(fails ? 1 : 0);
