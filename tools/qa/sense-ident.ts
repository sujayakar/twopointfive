// THE SENSES, THE TYPESCRIPT AGAINST THE RUST MODULE — what a man sees, how lit he is, what the smoke hides, how a sound reaches him and how he would walk
// there, each asked of the module (src/sim/sim.ts RUST.sight / light / smoke / sound / paths: rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs) and of
// the TypeScript it stands in for, over random questions on every level: every number of every answer must be the same to the bit.
//   bun run tools/qa/sense-ident.ts [--n 20000] [--levels office.json,warehouse.json] [--seed 1] [--bench]
// No tick runs here, so the tool opens the box world's copy as a tick would (StaticCollision.tickOpen) with the five rows raised, and the game's own maths put
// in Math's place (the module works its lengths out itself: under an engine's maths these rows answer nothing). "There" is the game's own function with the
// row up; "here" is the same function with the copy's rows put down for the one call. Doors are swung to random angles first; half way through, a share of
// the statics is re-shaped in place and SAID (col.wrote), as a blast says it.
// The floor's grid is baked again over the re-shaped boxes, as it is where a wall is broken. A level that does not stand up is a FAIL, not a level left out.
// RUST_BREAK=1 is the red path: the re-shape goes unsaid and the floor's new grid is not handed over, the module answers from a copy that is no longer the
// boxes, and the run must FAIL (every sense must differ somewhere: the last line says which did not).
// What a question is: an eye anywhere with any cone on a chest and a head anywhere (guards.ts eyeContact; some lenses, some hidden or smoked, some eyes that
// pass the thin statics too) · the light on a point, the lamps of the level and a dozen more of every kind put in for the run (lightEstimateAt, steady or not,
// some with an owner skipped) · the smoke of two dozen blobs along a line (sense.ts CpuSmoke.along) · a sound from anywhere to anywhere (collision.ts
// SoundPaths.propagate, nothing remembered between questions) · a way from anywhere to anywhere, under a few different caps on the search (NavGrid.findPath).
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.TPS_EXACT = '1';   // (the game's own maths: the rows answer under no other)
import { readdirSync } from 'fs';
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
const G: any = await import(`${ROOT}/src/game/guards.ts`), F: any = await import(`${ROOT}/src/game/fogofwar.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 20000), SEED = Number(flag('--seed') ?? 1), UNSAID = process.env.RUST_BREAK === '1';
const files = (flag('--levels') ?? readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).join(',')).split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== sense-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`);
type Kind = keyof typeof import('../../src/scene/boxMirror.ts').SENSES; const KINDS = Object.keys(BM.SENSES) as Kind[];   // (the senses are boxMirror.ts's list: one added there and not asked below does not typecheck)
const none = (): Record<Kind, number> => Object.fromEntries(KINDS.map(k => [k, 0])) as Record<Kind, number>;
for (const k of KINDS) process.env['TPS_RUST' + k.toUpperCase()] = '1';   // (the stand's own words: every level is stood up, and asked, with the five rows raised)
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
/** an answer as the numbers it is made of (a missing answer is its own number) */
const flat = (v: unknown, out: number[] = []): number[] => { if (v === null || v === undefined) out.push(-12345.5); else if (typeof v === 'number') out.push(v); else if (typeof v === 'boolean') out.push(v ? 1 : 0); else if (Array.isArray(v)) { out.push(v.length); for (const e of v) flat(e, out); } else for (const k of Object.keys(v as object).sort()) flat((v as Record<string, unknown>)[k], out); return out; };
const same = (a: number[], b: number[]) => a.length === b.length && a.every((x, i) => Object.is(x, b[i]));
let bad = 0; const ever = none();
for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${file.padEnd(20)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was asked`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col as any, M = col.mirror, boxes = col.boxes as any[];
  for (const d of g.doors.list) { if (rnd() < 0.6) { d.angle = (rnd() * 2 - 1) * 1.9; d.latched = false; } } hush(true); try { for (let i = 0; i < 3; i++) W.step(1 / 60); } catch { /* a level that needs more to run a frame still has its statics and leaves */ } hush(false);
  if (!X.EXACT.on || KINDS.some(k => !SIM.RUST[k])) { log(`${file.padEnd(20)} FAIL — the stand left a row down (${KINDS.filter(k => !SIM.RUST[k]).join(', ') || 'the game\'s own maths'}): nothing was asked`); bad++; continue; }
  const x1 = col.W * col.cs, z1 = col.H * col.cs;
  const anywhere = (): Vec3 => [-1 + rnd() * (x1 + 2), rnd() < 0.7 ? [0.15, 0.45, 0.9, 1.1, 1.6, 2.0, 2.6][Math.floor(rnd() * 7)] : -0.5 + rnd() * 4.5, -1 + rnd() * (z1 + 2)];
  const onBox = (): Vec3 => { const b = boxes[Math.floor(rnd() * boxes.length)]; return [b.c[0] + (rnd() * 2 - 1) * (b.h[0] + 0.4), b.c[1] + (rnd() * 2 - 1) * (b.h[1] + 0.2), b.c[2] + (rnd() * 2 - 1) * (b.h[2] + 0.4)]; };
  const onLeaf = (): Vec3 => { const dyn = col.dynamicBoxes; if (!dyn.length) return anywhere(); const b = dyn[Math.floor(rnd() * dyn.length)]; return [b.c[0] + (rnd() * 2 - 1) * 1.5, b.c[1] + (rnd() * 2 - 1) * 1.2, b.c[2] + (rnd() * 2 - 1) * 1.5]; };
  const pick = (): Vec3 => { const r = rnd(); return r < 0.5 ? anywhere() : r < 0.8 ? onBox() : onLeaf(); };
  const near = (a: Vec3, L: number): Vec3 => { const th = rnd() * 6.283; return [a[0] + Math.sin(th) * L, a[1] + (rnd() - 0.5) * 1.2, a[2] + Math.cos(th) * L]; };
  const unit = (): Vec3 => { const th = rnd() * 6.283, up = rnd() < 0.7 ? 0 : (rnd() - 0.5) * 1.6, c = Math.sqrt(Math.max(0, 1 - up * up)); return [Math.sin(th) * c, up, Math.cos(th) * c]; };
  // lamps of every kind beside the level's own, some of them somebody's
  const L = g.engine.lights.lights as any[], own = L.length;
  for (let i = 0; i < 14; i++) { const kind = [0, 1, 1, 3, 3, 2][Math.floor(rnd() * 6)];
    L.push({ kind, pos: pick(), dir: unit(), color: [rnd(), rnd(), rnd()], intensity: rnd() < 0.1 ? 0 : 5 + rnd() * 400, range: 3 + rnd() * 25, innerDeg: 5 + rnd() * 40, outerDeg: 10 + rnd() * 50, radius: rnd() < 0.5 ? 0.05 : rnd() * 0.6, owner: rnd() < 0.3 ? 1 + Math.floor(rnd() * 3) : 0, enabled: rnd() > 0.1, ttl: rnd() < 0.2 ? 0.5 : -1, steady: rnd() < 0.3 ? { intensity: 50 + rnd() * 100, pos: pick(), radius: 0.2 } : undefined }); }
  // the game's own smoke: blobs let out here and there, some old, some still filling
  const smoke = g.cpuSmoke; smoke.clear();
  for (let i = 0; i < 24; i++) { const p = pick(); smoke.emit({ pos: [p[0], Math.max(0.1, p[1]), p[2]], dir: [0, 1, 0], speed: 0, radius: 0.2 + rnd() * 0.8, density: 0.5 + rnd() * 6, temperature: 0, ttl: 2 + rnd() * 20, age: 0, kind: 'canister' }); if (rnd() < 0.5) smoke.step(0.3 + rnd() * 4); }
  smoke.step(0.5);
  const blobs = smoke.blobs.map((b: any): Vec3 => [b.x, b.y0 + 0.5, b.z]);
  const ask: Record<Kind, () => { q: string; run: () => unknown }> = {
    sight: () => {
      const eye = pick(), E: any = { pos: eye, look: unit(), range: 4 + rnd() * 40, halfDeg: 20 + rnd() * 160, nearM: rnd() * 4, nearDeg: 60 + rnd() * 120 }; if (rnd() < 0.3) E.vHalfDeg = 10 + rnd() * 60;
      const chest = rnd() < 0.5 ? near(eye, 0.5 + rnd() * 20) : pick(), pc = { bones: { chest, head: [chest[0] + (rnd() - 0.5) * 0.2, chest[1] + 0.5, chest[2] + (rnd() - 0.5) * 0.2] }, pos: chest, alive: rnd() > 0.1 };
      if (rnd() < 0.6) { const d: Vec3 = [chest[0] - eye[0], 0, chest[2] - eye[2]], l = Math.hypot(d[0], d[2]) || 1; E.look = [d[0] / l, (rnd() - 0.5) * 0.3, d[2] / l]; }   // (most eyes look about at him: the lines are what is asked)
      const blind = rnd() < 0.05, isPlayer = rnd() < 0.3, st = rnd(), through = rnd() < 0.7 ? BoxFlag.Glass : rnd() < 0.7 ? F.fogThrough() | BoxFlag.Thin : 0;
      return { q: `eye ${eye} look ${E.look} on ${chest} through ${through}`, run: () => G.eyeContact(g, E, pc, isPlayer, blind, st, through) };
    },
    light: () => {
      const r = rnd(), P = r < 0.4 ? near(L[Math.floor(rnd() * L.length)].pos, 0.2 + rnd() * 8) : pick(), steady = rnd() < 0.3, skip = rnd() < 0.3 ? (o: number) => o === 2 : undefined;
      return { q: `light on ${P} steady ${steady}`, run: () => G.lightEstimateAt(g, P, skip, steady) };
    },
    smoke: () => {
      const a = rnd() < 0.6 && blobs.length ? near(blobs[Math.floor(rnd() * blobs.length)], rnd() * 6) : pick(), b = rnd() < 0.6 ? near(a, 0.5 + rnd() * 15) : pick();
      return { q: `smoke ${a} → ${b}`, run: () => smoke.along(col, a, b) };
    },
    sound: () => {
      const src = pick(), dst = rnd() < 0.7 ? near(src, 0.5 + rnd() * 25) : pick();
      return { q: `sound ${src} → ${dst}`, run: () => col.sound.propagate(src, dst, false) };
    },
    paths: () => {
      const a = pick(), b = rnd() < 0.6 ? near(a, 1 + rnd() * 20) : pick(), cap = [6000, 6000, 6000, 400, 40, 3][Math.floor(rnd() * 6)];
      return { q: `way ${a} → ${b} within ${cap}`, run: () => col.nav.findPath(a, b, cap) };
    },
  };
  // a share of the statics is re-shaped half way: in place, and said (or, on the red path, not)
  const movers = boxes.filter(b => !b.rot && b.h[1] > 0.4 && !(b.flags & (BoxFlag.NoShadow | BoxFlag.Glass)) && rnd() < 0.15);
  const diff = none(), there = none(); let first = '';
  col.sound.invalidate(); col.tickOpen();
  for (const k of KINDS) C.SENSED[k] = 0;
  const per = Math.ceil(N / KINDS.length); let regrid = 0;
  for (let i = 0; i < per; i++) {
    if (i === (per >> 1)) { for (const b of movers) { b.c[0] += (rnd() - 0.5) * 1.5; b.c[2] += (rnd() - 0.5) * 1.5; b.h[1] *= 0.3 + rnd() * 0.7; } if (!UNSAID) col.wrote();
      // the floor's grid follows the boxes, as it does where a wall is broken; on the red path the module keeps the grid it had (the hand-over is cut)
      if (UNSAID) { const nav = col.nav; nav.there = (m: any) => nav.blocked.length <= m.navCap; }
      const ch = col.nav.rebake(0, 0, x1, z1); regrid += ch.length >> 1; }
    for (const k of KINDS) {
      const Q = ask[k](), n0 = C.SENSED[k];
      const r = flat(Q.run()); if (C.SENSED[k] > n0) there[k]++;
      const rows = M.rowSenses; M.senses = 0; M.rowSenses = 0; const h = flat(Q.run()); M.rowSenses = rows; M.senses = M.owes || !M.live ? 0 : rows;   // (here: the same function with the copy's rows down for the one call)
      if (!same(r, h)) { diff[k]++; first ||= `${k}: ${Q.q}: here ${JSON.stringify(h).slice(0, 200)}, the module ${JSON.stringify(r).slice(0, 200)}`; }
    }
  }
  let bench = '';
  if (argv.includes('--bench') && !UNSAID) {   // the same questions through both, a block each way, three times over
    const parts: string[] = [];
    for (const k of KINDS) { const qs = Array.from({ length: 3000 }, () => ask[k]().run); let tThere = 0, tHere = 0;
      for (let rep = 0; rep < 3; rep++) { col.tickOpen(); let t = performance.now(); for (const q of qs) q(); tThere += performance.now() - t; M.senses = 0; M.rowSenses = 0; t = performance.now(); for (const q of qs) q(); tHere += performance.now() - t; }
      parts.push(`${k} ${(tHere / 9).toFixed(2)} → ${(tThere / 9).toFixed(2)} µs`); }
    bench = `\n${''.padEnd(20)} bench (a question, TypeScript → the module): ${parts.join(' · ')}`;
  }
  col.tickShut(); L.length = own;
  const total = KINDS.reduce((n, k) => n + diff[k], 0), thin = KINDS.filter(k => there[k] < per * 0.5);   // (a sound that meets an open pane, a line too long to be one: the host's own — but not most of them)
  const fail = total > 0 || (!UNSAID && thin.length > 0);
  log(`${file.padEnd(20)} ${per} questions a sense (to the Rust module: ${KINDS.map(k => `${k} ${there[k]}`).join(', ')}) · ${boxes.length} statics, ${movers.length} re-shaped${UNSAID ? ' UNSAID' : ''}, ${regrid} cells of the floor's grid changed · ${col.dynamicBoxes.length} leaves · ${own} + 14 lamps · ${smoke.blobs.length} blobs · ${fail ? `FAIL — ${total ? `${KINDS.filter(k => diff[k]).map(k => `${k} ${diff[k]}`).join(', ')} differ, first: ${first}` : `too few questions went to the module (${thin.join(', ')})`}` : 'PASS — every answer the same to the bit'}${bench}`);
  if (fail) bad++; for (const k of KINDS) ever[k] += diff[k];
}
const blind = UNSAID ? KINDS.filter(k => !ever[k]) : [];   // (the red path must turn every sense red on some level)
log(bad ? `=== sense-ident: ${bad} level(s) FAIL${UNSAID ? blind.length ? ` — AND THE RED PATH IS NOT RED FOR ${blind.join(', ')}` : ' — every sense differed somewhere, as the red path must' : ''} ===` : '=== sense-ident: ALL PASS ===');
process.exit(blind.length ? 3 : bad ? 1 : 0);
