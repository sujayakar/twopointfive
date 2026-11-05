// THE SENSES, THE TYPESCRIPT AGAINST THE RUST MODULE — what a man sees, how lit he is, what the smoke hides, how a sound reaches him and how he would walk
// there, each asked of the module (src/sim/sim.ts RUST.sight / light / smoke / sound / paths: rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs) and of
// the TypeScript it stands in for, over random questions on every level: every number of every answer must be the same to the bit. And what the TEAM sees:
// the fog of war's sight pass (RUST.fog, rust/sim/src/fog.rs against fogofwar.ts seesHere), which no mission asks with anything but eyes that see all round.
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
// SoundPaths.propagate, nothing remembered between questions) · a way from anywhere to anywhere, under a few different caps on the search (NavGrid.findPath)
// · one to seven of a team's eyes, each with a wedge and a near field of its own or the team's (all round), handed over as a pass hands them (fogofwar.ts
// handEyes), on a man (his chest and head) or a body, tried in an order and until so many have a line (fogofwar.ts sees).
// The last lines: `SENSE-IDENT {…}` is each row's count of differing answers for a caller to read; exit 3 = the red path left a row green (named).
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.TPS_EXACT = '1';   // (the game's own maths: the rows answer under no other)
import { readdirSync } from 'fs';
import { resolve } from 'path';
const { standUp } = await import('./headless.ts');   // (not a static import: those load before this file's first line, and the harness reads the stand as it loads)
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
type Sense = keyof typeof import('../../src/scene/boxMirror.ts').SENSES; const SENSES = Object.keys(BM.SENSES) as Sense[];   // (the senses are boxMirror.ts's list: one added there and not asked below does not typecheck)
type Kind = Sense | 'fog'; const KINDS: Kind[] = [...SENSES, 'fog'];   // (…and the fog's pass, a row of its own with a flag of its own on the copy: `sight` there, `senses` the five)
const none = (): Record<Kind, number> => Object.fromEntries(KINDS.map(k => [k, 0])) as Record<Kind, number>;
for (const k of KINDS) process.env['TPS_RUST' + k.toUpperCase()] = '1';   // (the stand's own words: every level is stood up, and asked, with the six rows raised)
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
  const g = W.game, col = g.col as any, M = col.mirror, boxes = col.boxes as any[], fog = g.fogOfWar as any;
  let fogThere = 0; { const see = col.eyesSee; col.eyesSee = function (this: unknown, ...a: unknown[]) { const r = see.apply(col, a); if (r >= 0) fogThere++; return r; }; }   // (the men the module answered for: collision.ts counts the five senses, not the fog)
  const asked = (k: Kind): number => k === 'fog' ? fogThere : C.SENSED[k];
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
    fog: () => {
      // a team's eyes round a man: most within a room or two of him, half of them looking his way give or take, each with the team's field (absent: all
      // round, as every mission has it) or a wedge and a near field of its own (the Hard cone, the wand's lens) — the cone's edge and the near field's are asked
      const n = 1 + Math.floor(rnd() * 7), chest = pick(), eyes: any[] = [], idx: number[] = [];
      for (let k = 0; k < n; k++) { const pos = rnd() < 0.7 ? near(chest, 0.2 + rnd() * 18) : pick(), e: any = { id: k, pos, at: pos, look: rnd() < 0.03 ? [0, 1, 0] : unit() };
        if (rnd() < 0.5) { const d = [chest[0] - pos[0], chest[2] - pos[2]], l = Math.hypot(d[0], d[1]) || 1, t = (rnd() - 0.5) * 2.4, c = Math.cos(t), s = Math.sin(t); e.look = [(d[0] * c - d[1] * s) / l, (rnd() - 0.5) * 0.3, (d[0] * s + d[1] * c) / l]; }
        if (rnd() < 0.7) e.fov = [360, 359, 200, 120, 90, 60, 30][Math.floor(rnd() * 7)]; if (rnd() < 0.5) e.near = [0, 0.5, 2, 6][Math.floor(rnd() * 4)];
        eyes.push(e); idx.splice(Math.floor(rnd() * (idx.length + 1)), 0, k); }
      const order = idx.reduce((o, i, k) => o | (i << (4 * k)), 0), want = [1, 1, 2, n][Math.floor(rnd() * 4)], body = rnd() < 0.25;
      const pc: any = { alive: !body && rnd() > 0.1, pos: [chest[0], Math.max(0, chest[1] - 1.1), chest[2]], bones: rnd() < 0.9 ? { head: [chest[0] + (rnd() - 0.5) * 0.2, chest[1] + 0.5, chest[2] + (rnd() - 0.5) * 0.2], hips: [chest[0], chest[1] - 0.3, chest[2]] } : {} };
      return { q: `${n} eyes ${order.toString(16)} want ${want} on ${body ? 'a body' : pc.alive ? 'a man' : 'a man down'} at ${chest} (eye 0 at ${eyes[0].pos} look ${eyes[0].look} fov ${eyes[0].fov} near ${eyes[0].near})`,
        run: () => { fog.handEyes(eyes); return fog.sees(eyes, order, want, { char: pc }, body ? null : chest); } };
    },
  };
  // a share of the statics is re-shaped half way: in place, and said (or, on the red path, not)
  const movers = boxes.filter(b => !b.rot && b.h[1] > 0.4 && !(b.flags & (BoxFlag.NoShadow | BoxFlag.Glass)) && rnd() < 0.15);
  const diff = none(), there = none(); let first = '';
  col.sound.invalidate(); col.tickOpen();
  for (const k of SENSES) C.SENSED[k] = 0;
  const per = Math.ceil(N / KINDS.length); let regrid = 0;
  for (let i = 0; i < per; i++) {
    if (i === (per >> 1)) { for (const b of movers) { b.c[0] += (rnd() - 0.5) * 1.5; b.c[2] += (rnd() - 0.5) * 1.5; b.h[1] *= 0.3 + rnd() * 0.7; } if (!UNSAID) col.wrote();
      // the floor's grid follows the boxes, as it does where a wall is broken; on the red path the module keeps the grid it had (the hand-over is cut)
      if (UNSAID) { const nav = col.nav; nav.there = (m: any) => nav.blocked.length <= m.navCap; }
      const ch = col.nav.rebake(0, 0, x1, z1); regrid += ch.length >> 1; }
    for (const k of KINDS) {
      const Q = ask[k](), n0 = asked(k);
      const r = flat(Q.run()); if (asked(k) > n0) there[k]++;
      // (here: the same function with EVERY row of the copy down for the one call — the senses' own and the lines, rays and circles their TypeScript walks
      //  with: a stand has those up too, and a "TypeScript's answer" that asked the module its lines would be the module's answer twice)
      const rows = M.rowSenses, sight = M.rowSight, ln = M.rowLines, ry = M.rowRays, ci = M.rowCircles;
      M.senses = 0; M.rowSenses = 0; M.sight = M.rowSight = false; M.lines = M.rowLines = false; M.rays = M.rowRays = false; M.circles = M.rowCircles = false; const h = flat(Q.run());
      M.rowSenses = rows; M.rowSight = sight; M.rowLines = ln; M.rowRays = ry; M.rowCircles = ci; { const up = !(M.owes || !M.live); M.senses = up ? rows : 0; M.sight = up && sight; M.lines = up && ln; M.rays = up && ry; M.circles = up && ci; }
      if (!same(r, h)) { diff[k]++; first ||= `${k}: ${Q.q}: here ${JSON.stringify(h).slice(0, 200)}, the module ${JSON.stringify(r).slice(0, 200)}`; }
    }
  }
  let bench = '';
  if (argv.includes('--bench') && !UNSAID) {   // the same questions through both, a block each way, three times over
    const parts: string[] = [];
    for (const k of KINDS) { const qs = Array.from({ length: 3000 }, () => ask[k]().run); let tThere = 0, tHere = 0;
      for (let rep = 0; rep < 3; rep++) { col.tickOpen(); let t = performance.now(); for (const q of qs) q(); tThere += performance.now() - t; M.senses = 0; M.rowSenses = 0; M.lines = M.rowLines = false; M.rays = M.rowRays = false; M.circles = M.rowCircles = false; t = performance.now(); for (const q of qs) q(); tHere += performance.now() - t; }
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
log('SENSE-IDENT ' + JSON.stringify(ever));
log(bad ? `=== sense-ident: ${bad} level(s) FAIL${UNSAID ? blind.length ? ` — AND THE RED PATH IS NOT RED FOR ${blind.join(', ')}` : ' — every sense differed somewhere, as the red path must' : ''} ===` : '=== sense-ident: ALL PASS ===');
process.exit(blind.length ? 3 : bad ? 1 : 0);
