// WHAT RUST DRAWS IS WHAT THE COPY OF THE GAME DREW. The page that draws hands the GPU the rows rust/game_draw wrote; the page before it packs the same tables from
// its TypeScript copy of the game (scene/boxes.ts, render/lights.ts, anim/characters.ts). Here one Rust game is stood under the headless host, both packers run
// on the same tick, and every word is compared. This check goes with the copy: it is how the two are held together while both stand.
//   tools/blitz/rust.sh rust bun run tools/draw/rows-check.ts [level[:query]…]     default: the levels below, each a room clear with four
//   ROWS_BREAK=1 …    Rust's rows are of half a tick ago: it must fail
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Section, Want, Head, Geo, Mat, Cut, Word, Row4, Light, Man, Inst, Bones, Cap, STUFFS } from '../../src/draw/sections';
import { Drawn } from '../../src/draw/drawn';
import { lookNumbers } from '../../src/draw/look';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`), { lookConfig } = await import(`${ROOT}/src/render/frame.ts`), { lookWordOf, lookCoreOf } = await import(`${ROOT}/src/render/gbuffer.ts`), { COATFINISH, SOLIDGLASS } = await import(`${ROOT}/src/scene/stuff.ts`);
const { DRESS } = await import(`${ROOT}/src/game/dress.ts`);

const LEVELS = process.argv.slice(2).length ? process.argv.slice(2) : ['office', 'compound', 'pass', 'glasshouse', 'depot', 'mousehole'];
const BREAK = process.env.ROWS_BREAK === '1', DT = 1 / 60;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
const f32 = (w: number): number => { const b = new Uint32Array([w]); return new Float32Array(b.buffer)[0]; };
/** words that are not the same, by which of a row's words; floats within a last place are the same */
function differ(a: Uint32Array, at: number, b: Uint32Array, bt: number, rows: number, stride: number, floats: (k: number) => boolean, names: (k: number) => string, seen: Map<string, [number, string]>, tag: string, mask: (k: number) => number = () => 0xffffffff): number {
  let n = 0;
  for (let i = 0; i < rows; i++) for (let k = 0; k < stride; k++) { const x = a[at + i * stride + k], y = b[bt + i * stride + k]; if (x === y || (x & mask(k)) === (y & mask(k))) continue;
    if (floats(k)) { const p = f32(x), q = f32(y); if (Math.abs(p - q) <= 2e-6 * Math.max(1, Math.abs(p), Math.abs(q))) continue; }
    n++; const name = `${tag}.${names(k)}`, e = seen.get(name); if (e) e[0]++; else seen.set(name, [1, `row ${i}: copy ${floats(k) ? f32(x) : x} rust ${floats(k) ? f32(y) : y}`]); }
  return n;
}
const GEO = ['cx', 'cy', 'cz', 'turn', 'hx', 'hy', 'hz', 'flags'], MAT = ['r', 'g', 'b', 'cutHeight', 'er', 'eg', 'eb', 'finish', 'qx', 'qy', 'qz', 'qw', 'r2', 'g2', 'b2', 'gain'];

// (the picture's rules as a page has them)
cutConfig.on = true; lookConfig.on = true; COATFINISH.on = true; SOLIDGLASS.on = true;

for (const L of LEVELS) { const [name, query] = L.split(':');
  const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))), seed: 3 }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&${query ?? 'roomclear&squad4'}`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, E = g.engine, world = E.world; const seen = new Map<string, [number, string]>(); let ticks = 0, boxes = 0, lights = 0, cuts = 0, panes = 0, marked = 0;
  let D: Drawn = null as never;
  DRESS.men = true; DRESS.wetGround = true; world.setStuffOf(lookWordOf(), lookCoreOf());
  for (let t = 0; t < 420; t++) {
    if (t === 60 || t === 200) g.dropThrowable('frag', [g.player.char.pos[0] + 3, 0, g.player.char.pos[2] + 1]);
    g.godMode = true;
    await quiet(() => W.step(DT));
    const H: any = RustHost.last; if (!H) { line(false, `${name}: no Rust game stands`); break; }
    if (t === 0) { const X = H.F.X; D = new Drawn(X, H.F.mem, Want.all); const v = lookNumbers(E.settings, world, !!g.ch.cast); v[6] = v[7] = 0;   // (no line of light is taken out of the boxes)
      const at = X.face_alloc(v.length * 8) >>> 0; new Float64Array(H.F.mem.buffer, at, v.length).set(v); X.face_draw_look(H.h, at, v.length); X.face_free(at, v.length * 8); continue; }
    D.take(H.h); D.frame(H.h, 1, [20, 0, 14]);   // (a frame each tick, as a page draws: the motion words are of the last)
    if (t % 7 && t > 20) continue;
    // the copy's pack
    (g as any).wetGround.follow(g);   // (what the page's dress step does: the rain's dark on the ground)
    (g as any).drawMen([g.player.char, ...g.guards.map((x: any) => x.char)], false, false, true);
    world.glassFocus = [20, 0, 14];
    world.upload();
    // Rust's, of the same tick
    D.take(H.h); D.frame(H.h, BREAK ? 0.5 : 1, world.glassFocus);
    const u = D.u, h = D.at[Section.head], n = u[h + Head.count], st = u[h + Head.statics]; ticks++; boxes = Math.max(boxes, n);
    const geo = new Uint32Array(world.geoData), mat = new Uint32Array(world.matData.buffer);
    if (st !== world.statics.length || n !== world.all.length) { const e = seen.get('count'); if (e) e[0]++; else seen.set('count', [1, `tick ${t}: copy ${world.statics.length} + ${world.dynamics.length}, rust ${st} + ${n - st}`]); if (st !== world.statics.length) continue; }
    const same = Math.min(n, world.all.length);
    // (a frame's own boxes are compared where the two lists have them in one place: the order is the tick's)
    differ(geo, 0, u, D.at[Section.geo], same, Geo.row, k => k !== 3 && k !== 7, k => GEO[k], seen, 'geo', k => k === 7 ? 0x3ffff : 0xffffffff);
    differ(mat, 0, u, D.at[Section.mat], same, Mat.row, () => true, k => MAT[k], seen, 'mat');
    if (n === world.all.length) {
      { const CT = new Uint32Array(world.cutData.buffer); for (let i = 0; i < n; i++) { const b = world.all[i]; if (b.round === true || b.cut?.length > 0) { cuts++; differ(CT, i * Cut.row, u, D.at[Section.cut] + i * Cut.row, 1, Cut.row, () => true, k => `w${k}`, seen, 'cut'); } } }
      { const I = world.infoData as Uint32Array, keep = (k: number) => k === 20 || k === 21 || (k >= 24 && k < 88); for (let k = 0; k < I.length; k++) if (!keep(k)) differ(I, k, u, D.at[Section.info] + k, 1, 1, () => k > 23 && k < 252 || k > 255, () => `word ${k}`, seen, 'info'); }
      { const B = new Uint32Array((world as any).gbData.buffer); differ(B, 0, u, D.at[Section.panes], Math.min(B.length / 4, D.n[Section.panes]), Row4.row, () => true, k => `w${k}`, seen, 'panes'); for (let k = 0; k < B.length; k += 4) if (B[k]) { panes++; break; } }
      const cells = world.cellStart as Uint32Array, vis = world.visible as Uint32Array;
      const vn = u[h + Head.visible] + u[h + Head.visibleCut], tn = world.visibleCount + world.visibleCutCount;
      if (vn !== tn) { const e = seen.get('visible.count'); if (e) e[0]++; else seen.set('visible.count', [1, `copy ${world.visibleCount} + ${world.visibleCutCount}, rust ${u[h + Head.visible]} + ${u[h + Head.visibleCut]}`]); }
      else if (!u[h + Head.orderStale]) { let bad = 0, first = '';   // (the order is made again a few calm frames after a wall changed: the copy's, here, at once)
        for (let i = 0; i < vn; i++) { const x = vis[i], y = u[D.at[Section.visible] + i]; if (x !== y) { bad++; first ||= `at ${i}: copy ${x} rust ${y}`; } } if (bad) { const e = seen.get('visible.order'); if (e) e[0] += bad; else seen.set('visible.order', [bad, first]); } }
      differ(cells, 0, u, D.at[Section.cells], Math.min(cells.length, D.n[Section.cells]), Word.row, () => false, () => 'word', seen, 'grid.cells');
      if (world.itemCount === u[h + Head.items]) differ(world.items, 0, u, D.at[Section.items], 2 * world.itemCount, Word.row, () => false, () => 'word', seen, 'grid.items');
      else { const e = seen.get('grid.items'); if (e) e[0]++; else seen.set('grid.items', [1, `copy ${world.itemCount} rust ${u[h + Head.items]}`]); }
    }
    // a box's marks, by what its list says: the two tables hold the same lists in the order each met them (a list's first row rides in the box's word)
    { const MD = world.markData as Float32Array, m0 = D.at[Section.marks], bare = (v: number) => v & 128 ? v - 128 : v;   /* (a mark that is a light is known by its object in the copy, which no bridge carries to a game stood from it) */
      for (let i = 0; i < same; i++) { const a = geo[i * 8 + 7] >>> 18, b = u[D.at[Section.geo] + i * 8 + 7] >>> 18; if (!a && !b) continue; marked++;
        let bad = !a !== !b ? 'one has none' : ''; for (let r = 0, last = false; !bad && !last; ) { const c = (a - 1 + r) * 4, d = m0 + (b - 1 + r) * 4, meta = MD[c + 4], rows = 8 + Math.ceil(MD[c + 2] * MD[c + 3] / 48); last = (meta & 32) !== 0;
          if (bare(meta) !== bare(D.f[d + 4])) bad = `meta ${meta} ${D.f[d + 4]}`; for (let k = 0; k < rows * 4 && !bad; k++) if (k !== 4 && Math.abs(MD[c + k] - D.f[d + k]) > 2e-6 * Math.max(1, Math.abs(MD[c + k]))) bad = `word ${k}: ${MD[c + k]} ${D.f[d + k]}`; r += rows; }
        if (bad) { const e = seen.get('marks'); if (e) e[0]++; else seen.set('marks', [1, `box ${i}: ${bad}`]); } } }
    // the lights, as objects (their slots are the renderer's)
    { const LL = E.lights.lights; let k = 0; lights = Math.max(lights, LL.length);
      const rows: number[] = []; for (let i = 0; i < D.n[Section.lights]; i++) { const o = D.at[Section.lights] + i * Light.row; if (!(u[o + Light.says] & 16)) rows.push(o); }
      if (rows.length !== LL.length) { const e = seen.get('lights.count'); if (e) e[0]++; else seen.set('lights.count', [1, `tick ${t}: copy ${LL.length} rust ${rows.length}`]); }
      else for (const o of rows) { const l = LL[k++]; const pairs: [string, number, number][] = [['kind', l.kind, u[o + Light.kind]], ['x', l.pos[0], D.f[o + Light.pos]], ['z', l.pos[2], D.f[o + Light.pos + 2]], ['dx', l.dir[0], D.f[o + Light.dir]], ['color', l.color[1], D.f[o + Light.color + 1]], ['intensity', l.ttl < 0 ? l.intensity : 0, l.ttl < 0 ? D.f[o + Light.intensity] : 0], ['range', l.range, D.f[o + Light.range]], ['radius', l.radius, D.f[o + Light.radius]], ['inner', l.innerDeg, D.f[o + Light.inner]], ['outer', l.outerDeg, D.f[o + Light.outer]], ['volumetric', l.volumetric, D.f[o + Light.volumetric]], ['owner', l.owner, u[o + Light.owner]], ['enabled', l.enabled ? 1 : 0, u[o + Light.says] & 1], ['mark', l.markRef ?? -1, u[o + Light.mark] | 0], ['fog', E.lights.fogClass(l), D.f[o + Light.fog]], ['ghost', l.fogGhost ?? 0, D.f[o + Light.ghost]], ['ttl', l.ttl, D.f[o + Light.ttl]], ['flicker', l.flicker, D.f[o + Light.flicker]]];
        for (const [w, a, b] of pairs) if (Math.abs(a - b) > 2e-6 * Math.max(1, Math.abs(a))) { const e = seen.get(`lights.${w}`); if (e) e[0]++; else seen.set(`lights.${w}`, [1, `tick ${t}, light ${k - 1} ${l.name}: copy ${a} rust ${b}`]); } } }
    // the men handed to the skinning
    { const C = g.chars, nm = D.n[Section.men]; if (C && C.instanceCount !== undefined && C.instanceCount !== nm) { const e = seen.get('men.count'); if (e) e[0]++; else seen.set('men.count', [1, `tick ${t}: copy ${C.instanceCount} rust ${nm}`]); }
      else if (C) { const I = C.instU32 as Uint32Array, J = new Uint32Array((C.staged as Float32Array).buffer);
        differ(I, 0, u, D.at[Section.insts], nm, Inst.row, k => ![3, 7, 22].includes(k), k => `w${k}`, seen, 'men.inst');
        differ(J, 0, u, D.at[Section.bones], nm, Bones.row, () => true, k => `bone`, seen, 'men.bones');
        const caps = new Uint32Array(world.capsuleData.buffer); differ(caps, 0, u, D.at[Section.caps], Math.min(world.charCount, nm) * 15, Cap.row, k => k < 7, k => k < 7 ? 'end' : 'misc', seen, 'men.caps'); } }
  }
  const off = [...seen].filter(([k]) => k !== 'men.caps.misc');   // (a proxy's colour is the page's: src/draw/men.ts)
  line(off.length === 0 && ticks > 0, `${name}: every word of ${ticks} ticks is the copy's (at most ${boxes} boxes, ${lights} lights; ${cuts} cut rows, ${marked} marked boxes, panes broken on ${panes} ticks)`, off.map(([k, [c, w]]) => `${k} × ${c} (${w})`).join('\n    '));
}
console.log(fails ? `=== rows: ${fails} FAILED ===` : '=== rows: ALL PASS ===');
process.exit(fails ? 1 : 0);
