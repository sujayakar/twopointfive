// THE MARKS RUST KEEPS ARE THE MARKS THE COPY OF THE GAME LAID. The page that draws hands the GPU the buffer rust/game_draw marks.rs wrote; the page before it lays the same marks in TypeScript on
// its copy of the game (render/decals.ts, game/marks.ts, floormess.ts, wallwear.ts) from the tick's out events. Here one Rust game is stood under the headless host, a fight is played, and every word of
// the two buffers is compared. It goes with the copy.
//   tools/blitz/rust.sh rust bun run tools/draw/marks-check.ts [level[:query]…]     default: the levels below, each a room clear with four
//   MARKS_BREAK=1 …    the copy's first mark is a centimetre off: it must fail
// What the copy has lost since the tick left TypeScript is not asked of it: a print's drying and its look (the copy's stays as laid).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from '../qa/headless.ts';
import { RustHost } from '../blitz/rust-host.ts';
import { Section, Want } from '../../src/draw/sections';
import { Drawn } from '../../src/draw/drawn';
import { lookNumbers } from '../../src/draw/look';

const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { cutConfig } = await import(`${ROOT}/src/scene/cut.ts`), { roundConfig } = await import(`${ROOT}/src/scene/round.ts`), { lookConfig } = await import(`${ROOT}/src/render/frame.ts`), { lookWordOf, lookCoreOf } = await import(`${ROOT}/src/render/gbuffer.ts`), { COATFINISH, SOLIDGLASS } = await import(`${ROOT}/src/scene/stuff.ts`);
const { DRESS } = await import(`${ROOT}/src/game/dress.ts`), { MARKCUT, MARKSIT } = await import(`${ROOT}/src/render/decals.ts`);

const LEVELS = process.argv.slice(2).length ? process.argv.slice(2) : ['office', 'compound', 'glasshouse', 'mousehole'];
const BREAK = process.env.MARKS_BREAK === '1', DT = 1 / 60, TICKS = Number(process.env.MARKS_TICKS ?? 900);
const CELLS = 40 * 28, CELL0 = 4, LIST0 = CELL0 + CELLS, REC0 = LIST0 + 4096, RECORD = 12, KIND_PRINT = 6;
let fails = 0;
const line = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
const log = console.log, quiet = async <T>(f: () => T | Promise<T>): Promise<T> => { console.log = () => { }; try { return await f(); } finally { console.log = log; } };
const f32 = (w: number): number => { const b = new Uint32Array([w]); return new Float32Array(b.buffer)[0]; };
/** the same float, as near as a mark on a moving leaf is kept to its box (render/decals.ts `near`) */
const same = (a: number, b: number): boolean => a === b || Math.abs(f32(a) - f32(b)) <= 3e-5 * Math.max(1, Math.abs(f32(a)));

cutConfig.on = true; roundConfig.gpu = true; lookConfig.on = true; COATFINISH.on = true; SOLIDGLASS.on = true; MARKCUT.on = true; MARKSIT.on = true;

for (const L of LEVELS) { const [name, query] = L.split(':');
  const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', `${name}.json`), 'utf8'))), seed: 3 }));
  const why = (await quiet(() => startFromQuery(W.game, `?levelFile=${name}.json&${query ?? 'roomclear&squad4'}`)) as any)?.why; if (why) { line(false, `${name}: the start was refused`, why); continue; }
  const g = W.game, E = g.engine, world = E.world; const seen = new Map<string, [number, string]>(); let ticks = 0, most = 0, kinds = new Set<string>(), rode = 0;
  const say = (k: string, w: string) => { const e = seen.get(k); if (e) e[0]++; else seen.set(k, [1, w]); };
  let D: Drawn = null as never;
  DRESS.men = true; world.setStuffOf(lookWordOf(), lookCoreOf());
  for (let t = 0; t < TICKS; t++) {
    const at = g.player.char.pos;
    if (t === 60 || t === 300) g.dropThrowable('frag', [at[0] + 3, 0, at[2] + 1]);
    if (t === 180 || t === 500) g.dropThrowable('flash', [at[0] - 2, 0, at[2] + 2]);
    g.godMode = true;
    // the squad works the nearest door, and Sam empties his rifle at whatever stands in front of him
    { const O = g.orders, d = g.doors.list.reduce((a: any, b: any) => Math.hypot(b.pos[0] - at[0], b.pos[2] - at[2]) < Math.hypot(a.pos[0] - at[0], a.pos[2] - at[2]) ? b : a); try {
        if (t === 20) O.give('stack', O.targetAt(d.pos), 'all'); if (t === 200) O.give('go', O.targetAt(d.pos), 'all'); if (t === 500) O.give('stack', O.targetAt(d.pos), 'red'); if (t === 560) O.give('bangClear', O.targetAt(d.pos), 'red'); } catch { /* an order the floor has no use for */ } }
    if (t === 2) W.input.pressed.add('Digit2'), W.input.keys.add('Digit2'); if (t === 8) W.input.keys.delete('Digit2');
    W.input.buttons = 0;
    if (t > 70 && t % 9 === 0) { const a = t * 0.013; W.cursorAt([at[0] + Math.cos(a) * 3, 1 + (t % 5) * 0.2, at[2] + Math.sin(a) * 3]); W.input.clicked |= 1; W.input.buttons |= 1; }
    await quiet(() => W.step(DT));
    const H: any = RustHost.last; if (!H) { line(false, `${name}: no Rust game stands`); break; }
    if (t === 0) { const X = H.F.X; D = new Drawn(X, H.F.mem, Want.all); const v = lookNumbers(E.settings, world, !!g.ch.cast); v[6] = v[7] = 0;
      const a = X.face_alloc(v.length * 8) >>> 0; new Float64Array(H.F.mem.buffer, a, v.length).set(v); X.face_draw_look(H.h, a, v.length); X.face_free(a, v.length * 8); continue; }
    if (t % 11 && t > 20) continue;
    // the copy's buffer, of the boxes the tick left
    (g as any).drawMen([g.player.char, ...g.guards.map((x: any) => x.char)], false, false, true);
    world.glassFocus = [20, 0, 14]; world.upload();
    const C = g.decals; C.sync(world); if (BREAK && C.list.length) C.list[0].pos = [C.list[0].pos[0] + 0.01, C.list[0].pos[1], C.list[0].pos[2]];
    C.build(); const c: Uint32Array = C.words();
    // Rust's, of the same tick
    D.take(H.h); D.frame(H.h, 1, world.glassFocus);
    const u = D.u, w0 = D.at[Section.decals], n = c[0]; ticks++; most = Math.max(most, n);
    if (u[w0] !== n) { say('count', `tick ${t}: copy ${n} rust ${u[w0]}`); continue; }
    if (c[2] !== u[w0 + 2]) say('serial', `tick ${t}: copy ${c[2]} rust ${u[w0 + 2]}`);
    if (c[3] !== u[w0 + 3]) say('sit', `copy ${c[3]} rust ${u[w0 + 3]}`);
    for (let i = 0; i < n; i++) { const o = REC0 + i * RECORD, kind = c[o + 11] & 7, hidden = f32(c[o + 3]) === 0, attached = (c[o + 11] & 8) !== 0; kinds.add(`${kind}`); if (attached) rode++;
      for (let k = 0; k < RECORD; k++) { const x = c[o + k], y = u[w0 + o + k]; if (k === 7 && kind === KIND_PRINT) continue;
        if (k === 11) { const mask = (kind === KIND_PRINT ? 0xffff000f : 0xffffffff) & (hidden ? 0x0000ffff : 0xffffffff); if (((x ^ y) & mask) === 0) continue; }
        else if (same(x, y)) continue;
        say(`record.${k}`, `tick ${t}, mark ${i} of ${n} (kind ${kind}${attached ? ', rides' : ''}): copy ${k === 11 ? x : f32(x)} rust ${k === 11 ? y : f32(y)}`); } }
    for (let i = 0; i < CELLS; i++) if (c[CELL0 + i] !== u[w0 + CELL0 + i]) { say('cell', `tick ${t}, cell ${i}: copy ${c[CELL0 + i]} rust ${u[w0 + CELL0 + i]}`); }
    for (let i = 0; i < c[1]; i++) if (c[LIST0 + i] !== u[w0 + LIST0 + i]) { say('list', `tick ${t}, entry ${i}: copy ${c[LIST0 + i]} rust ${u[w0 + LIST0 + i]}`); break; }
  }
  line(seen.size === 0 && ticks > 0, `${name}: every word of the marks' buffer on ${ticks} ticks is the copy's (at most ${most} marks, of kinds ${[...kinds].sort().join(' ')}, ${rode} rows that ride)`, [...seen].map(([k, [c, w]]) => `${k} × ${c} (${w})`).join('\n    '));
}
console.log(fails ? `=== marks: ${fails} FAILED ===` : '=== marks: ALL PASS ===');
process.exit(fails ? 1 : 0);
