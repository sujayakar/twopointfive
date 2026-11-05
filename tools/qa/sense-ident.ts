// THE SENSES OF THE RUST MODULE, HELD — what a man sees, how lit he is, what the smoke hides, how a sound reaches him, how he would walk there and what a
// team's eyes see (rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs, fog.rs, asked through src/scene/collision.ts). Every answer the module gives
// is shown to a watcher (collision.ts SENSE_WATCH) and folded into a word under the name of its kind of question (tools/qa/ident-pin.ts).
//   bun run tools/qa/sense-ident.ts [--n 20000] [--levels office.json,warehouse.json] [--seed 1]       random questions over whole levels (the page's stand)
//   bun run tools/qa/sense-ident.ts --made [--n 21000] [--seed 1] [--record]                             …over a box world of the tool's own (the frozen stand)
//   … --against <root>                                                                                  the same run by another checkout's tool: every word the same
// NOTHING IN TYPESCRIPT ANSWERS A SENSE (collision.ts THE SENSES), so there is nothing to compare an answer to: the module is held to ITS OWN EARLIER
// ANSWERS, the four ways of tools/qa/ident-pin.ts — the words, the red paths by a child that plays the same run plain, the pin, and `--against`. A
// question the module will not take is thrown by name, and a run that threw is a FAIL.
// OVER LEVELS no tick runs, so the tool opens the box world's copy as a tick would (StaticCollision.tickOpen). Doors are swung to random angles first; half
// way through, a share of the statics is re-shaped in place and SAID (col.wrote), as a blast says it, and the floor's grid is baked again over them. A level
// that does not stand up is a FAIL, not a level left out. A level's questions come off a die of its own (the seed and the level's name), and so does its
// game, so a level added or taken away moves no other level's words and a level's run is the same run every time.
// THE MADE-UP WORLD (`--made`) is what the PIN is taken over: some two hundred statics of every kind a line can meet (walls turned any way, furniture,
// panes of glass, thin things, tilted and round and cut boxes, decals), leaves, lamps of every kind (more in reach of a point than one part of a light
// question carries), blobs of smoke and a team's eyes, all dealt from the seed and from nothing else: no level's edit moves a word of it, and the smoke's
// numbers are the tool's own for the run (a tuning of the game's moves none). Its words on the frozen stand are held to tools/qa/fixtures/sense-words.json;
// `--record` writes that file. ITS FIRST RECORDING was taken on the last tree where the TypeScript stood, from a run in which every one of the 21,000
// questions was also asked of it and the two agreed to the bit: that is all the pin ever knew of the TypeScript.
// What a question is: an eye anywhere with any cone on a chest and a head anywhere (guards.ts eyeContact; some lenses, some hidden or smoked, some eyes that
// pass the thin statics too) · the light on a point, lamps of every kind (lightEstimateAt, steady or not, some with an owner skipped) · the smoke of two
// dozen blobs along a line (sense.ts CpuSmoke.along) · a sound from anywhere to anywhere (collision.ts SoundPaths.propagate, nothing remembered between
// questions) · a way from anywhere to anywhere, under a few different caps on the search, half of them with a price a cell (NavGrid.findPath; the cells
// priced, in the order they were asked for, are part of the answer) · one to seven of a team's eyes, each with a wedge and a near field of its own or the
// team's, handed over as a pass hands them (fogofwar.ts handEyes), on a man or a body, tried in an order and until so many have a line (fogofwar.ts sees).
// THE LAMPS IN PARTS, AND THE SMOKE'S SHADOW (a block of its own after a level's questions): some six hundred lamps in reach of a few points, asked in
// parts (collision.ts lamp, light.rs); then the same lamps through a field of smoke: its grid able to answer, no grid standing for it, and a grid holding
// more smoke than bricks — in the last two the blobs answer, inside the lamp loop (smoke.rs).
// THE RED PATHS, one at a time. Under one the tool plays the same run again, plain, in a child, and holds its own words to the child's:
//   RUST_BREAK=1        the re-shape goes unsaid and the floor's new grid is not handed over: the module answers from a copy that is no longer the boxes,
//                       and every kind of question's word must move on some level (every word of the made-up world).
//   PRICED_BREAK=1      the host prices the cells the module asks for last first: the way is the same and the order is not. The paths' words must move,
//                       and no other.
//   LAMPS_BREAK=1       every part of a light question begins its sum again: the words of the lamps' block must move, and no kind's of the questions.
//   SHADE_BREAK=1       the module is handed no blobs: the words of the two cases where the grid cannot answer must move, and nothing else.
// Exit 0 = every verdict passed · 1 = a verdict failed (under a red path: as it must) · 3 = a red path that moved the wrong words, or none (named).
const ARGV = process.argv.slice(2), MADE = ARGV.includes('--made');
process.env.TPS_PROFILE ??= MADE ? 'frozen' : 'shipped'; if (!MADE) process.env.TPS_CPUSENSE ??= '1';   // (the rows and the game's own maths are the build's: no word raises them)
import { readdirSync } from 'fs';
import { resolve } from 'path';
const { standUp } = await import('./headless.ts');   // (not a static import: those load before this file's first line, and the harness reads the stand as it loads)
const PIN: typeof import('./ident-pin.ts') = await import('./ident-pin.ts');
const ROOT = resolve(import.meta.dir, '../..');
const { BoxFlag, makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
const G: any = await import(`${ROOT}/src/game/guards.ts`), F: any = await import(`${ROOT}/src/game/fogofwar.ts`), SE: any = await import(`${ROOT}/src/game/sense.ts`);
const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
type Vec3 = [number, number, number];

const flag = (n: string) => { const i = ARGV.indexOf(n); return i >= 0 ? ARGV[i + 1] : undefined; };
const N = Number(flag('--n') ?? (MADE ? 21000 : 20000)), SEED = Number(flag('--seed') ?? 1), UNSAID = process.env.RUST_BREAK === '1', BACKWARD = process.env.PRICED_BREAK === '1', AGAIN = process.env.LAMPS_BREAK === '1', UNSHADED = process.env.SHADE_BREAK === '1';
const files = (flag('--levels') ?? readdirSync(resolve(ROOT, 'levels')).filter(f => f.endsWith('.json') && !f.endsWith('.lint.json')).join(',')).split(',');
/** a die of its own from a seed (the one the tool has always rolled) */
const die = (seed: number) => { let s = seed >>> 0; return () => { s = (s + 0x6D2B79F5) >>> 0; let t = s; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; };
/** …and the seed of a name's own die */
const seedOf = (name: string, salt: number) => { let h = (2166136261 ^ Math.imul(SEED + salt, 0x9e3779b1)) >>> 0; for (let i = 0; i < name.length; i++) h = Math.imul(h ^ name.charCodeAt(i), 16777619) >>> 0; return h; };
const BREAK_WORD = UNSAID ? 'RUST_BREAK' : BACKWARD ? 'PRICED_BREAK' : AGAIN ? 'LAMPS_BREAK' : UNSHADED ? 'SHADE_BREAK' : 'RUST_BREAK';
if (+UNSAID + +BACKWARD + +AGAIN + +UNSHADED > 1) { console.log('=== sense-ident: RUST_BREAK, PRICED_BREAK, LAMPS_BREAK and SHADE_BREAK are four red paths: one at a time ==='); process.exit(2); }
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== sense-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`);
/** the kinds of question (collision.ts SENSE_WATCH's rows) */
const KINDS = ['sight', 'light', 'smoke', 'sound', 'paths', 'fog'] as const; type Kind = (typeof KINDS)[number];
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const blind: string[] = []; const verdict = (ok: boolean, label: string, detail: string) => { log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${detail}`); if (!ok) bad++; };

// ---- THE WORDS: every answer the module gives, under the name of its kind of question and of where it was asked (`WHERE`: a level, or the made-up
// world). A light question of the lamps' block is folded under that block's own name (`SUB`), apart from the questions' own.
const FOLD = new PIN.Fold(); let WHERE = '', SUB: string | null = null;
const lane = (row: string) => FOLD.call(`${row === 'light' && SUB ? SUB : row} · ${WHERE}`);
/** THE MODULE'S OWN COUNT of the questions it answered, a kind (collision.ts SENSED counts five where the answer is read; the fog's pass is counted here,
 *  where its answer comes back): a question of the tool's that the game answered some other way — a grid, a cache, an early return — is not in it */
let fogThere = 0; { const P = C.StaticCollision.prototype, see = P.eyesSee; P.eyesSee = function (this: unknown, ...a: unknown[]) { const r = see.apply(this, a); fogThere++; return r; }; }
const there = (): Record<string, number> => ({ ...C.SENSED, fog: fogThere });
/** an answer as the numbers it is made of (a missing answer is its own number) */
const flat = (v: unknown, out: number[] = []): number[] => { if (v === null || v === undefined) out.push(-12345.5); else if (typeof v === 'number') out.push(v); else if (typeof v === 'boolean') out.push(v ? 1 : 0); else if (Array.isArray(v)) { out.push(v.length); for (const e of v) flat(e, out); } else for (const k of Object.keys(v as object).sort()) flat((v as Record<string, unknown>)[k], out); return out; };
/** …and what the question HANDS THE GAME, as its numbers, under the same name: the host's half of a sense (a sound's loudness and apparent place made of
 *  what the lines said, an eye's verdict read off the answer's bits, a way's waypoints) is held with the module's */
const handed = (k: string, a: unknown): number[] => { const r = flat(a); FOLD.doubles(FOLD.call(`${k} · ${WHERE}`), r); return r; };
C.SENSE_WATCH.see = (row: string, v: ArrayLike<number>) => { if (WHERE) FOLD.doubles(lane(row), v); };   // (only the tool's own questions: what a game's own ticks ask as a level is stood up is no part of the run)
/** a price a cell for a search: plain floor mostly, some dear, some all but a wall, some under 1, in blocks of a cell or of four by four — a function of the
 *  cell alone, as every penalty of the game's is */
const priceOf = (salt: number, shift: number) => (ix: number, iz: number) => { let h = Math.imul((ix >> shift) + 1, 73856093) ^ Math.imul((iz >> shift) + 1, 19349663) ^ salt; h = Math.imul(h ^ (h >>> 15), 0x2c1b3c6d); h = (h ^ (h >>> 12)) >>> 0; const r = h % 16; return r < 9 ? 1 : r < 12 ? 6 : r < 14 ? 20 : r === 14 ? 0.5 : 200; };
/** one question: what it is in words, how it is asked, and what of its answer is folded beside what the watcher was shown */
interface Ask { q: string; run: () => unknown; fold?: (a: unknown) => void }
/** a way with a price: the answer is the way AND the cells the search asked the price of, in the order it first asked for each (nav.rs find_priced:
 *  the module comes to need the cells the search reads, when it reads them) */
const priced = (col: any, a: Vec3, b: Vec3, cap: number, salt: number, shift: number): Ask => { const price = priceOf(salt, shift), W = col.nav.W;
  return { q: `way ${a} → ${b} within ${cap}, priced (salt ${salt}, blocks of ${1 << shift})`,
    run: () => { const order: number[] = [], met = new Set<number>();
      const way = col.nav.findPath(a, b, cap, (ix: number, iz: number) => { const i = iz * W + ix; if (!met.has(i)) { met.add(i); order.push(i); } return price(ix, iz); });
      return [way, order]; },
    fold: (ans: unknown) => FOLD.doubles(lane('paths'), (ans as [unknown, number[]])[1]) }; };
/** the order's red path: the host prices the cells the module asks for last first */
const backward = (col: any) => { col.nav.priceThese = (m: any, k: number, pen: (ix: number, iz: number) => number) => { const w = col.nav.W; C.SENSE_WATCH.see?.('paths', SIM.u32().slice(m.navWant, m.navWant + k)); for (let i = k - 1; i >= 0; i--) { const c = SIM.u32()[m.navWant + i]; SIM.f64()[m.navPrice + c] = pen(c % w, (c / w) | 0); } }; };

const unable: Record<string, boolean> = { 'a grid that answers': false, 'no grid standing': false, 'more smoke than bricks': false };
/** the first line of what was thrown, and where from */
const thrown = (e: unknown) => String((e as Error).stack ?? (e as Error).message).split('\n').slice(0, 3).join(' / ').slice(0, 500);

// ================================================================ THE MADE-UP WORLD
async function made(): Promise<void> {
  hush(true); let W: any; try { W = await standUp({ uploadEvery: 1000, seed: 1 }); } finally { hush(false); }
  const g = W.game, fog = g.fogOfWar as any, rnd = die(seedOf('the made-up world', 0));
  if (!X.EXACT.on) { verdict(false, 'made-up world', 'the stand left the game\'s own maths down: nothing was asked'); return; }
  const x1 = WORLD.gridW * WORLD.gridCell, z1 = WORLD.gridH * WORLD.gridCell, TAU = Math.PI * 2;
  const at = (y: number): Vec3 => [1 + rnd() * (x1 - 2), y, 1 + rnd() * (z1 - 2)];
  const turn = () => rnd() < 0.6 ? Math.floor(rnd() * 4) * Math.PI / 2 : rnd() * TAU;
  const unit = (): Vec3 => { const th = rnd() * TAU, up = rnd() < 0.7 ? 0 : (rnd() - 0.5) * 1.6, c = Math.sqrt(Math.max(0, 1 - up * up)); return [Math.sin(th) * c, up, Math.cos(th) * c]; };
  // ---- the box world: every kind of static a line, a circle or a search can meet
  const boxes: any[] = [];
  for (let i = 0; i < 70; i++) { const hy = 1.2 + rnd() * 0.4, r = rnd(); boxes.push(makeBox({ c: at(hy), h: [0.5 + rnd() * 3, hy, 0.05 + rnd() * 0.1], yaw: turn(), flags: r < 0.12 ? BoxFlag.Glass : r < 0.2 ? BoxFlag.Thin : 0 })); }   // walls, panes, thin things
  for (let i = 0; i < 60; i++) { const h: Vec3 = [0.2 + rnd() * 0.8, 0.15 + rnd() * 0.7, 0.2 + rnd() * 0.8]; boxes.push(makeBox({ c: at(h[1]), h, yaw: rnd() * TAU })); }   // furniture, on the floor
  for (let i = 0; i < 10; i++) boxes.push(makeBox({ c: at(2.1 + rnd() * 0.6), h: [0.3 + rnd() * 2, 0.08 + rnd() * 0.15, 0.1 + rnd() * 0.3], yaw: turn() }));   // overhead
  for (let i = 0; i < 12; i++) { const ax = unit(), l = Math.hypot(ax[0], ax[1], ax[2]) || 1, a = 0.2 + rnd(), s = Math.sin(a / 2) / l; boxes.push(makeBox({ c: at(0.4 + rnd() * 1.2), h: [0.2 + rnd() * 0.9, 0.1 + rnd() * 0.6, 0.2 + rnd() * 0.9], yaw: 0, rot: [ax[0] * s, ax[1] * s, ax[2] * s, Math.cos(a / 2)] })); }   // tilted
  for (let i = 0; i < 12; i++) { const r = 0.15 + rnd() * 0.6, hy = 0.3 + rnd() * 1.2; boxes.push(makeBox({ c: at(hy), h: [r, hy, r], yaw: 0, round: true })); }   // round
  for (let i = 0; i < 12; i++) { const hy = 0.4 + rnd() * 1.0, cut = Array.from({ length: 1 + Math.floor(rnd() * 3) }, () => { const n = unit(), l = Math.hypot(n[0], n[1], n[2]) || 1; return { n: [n[0] / l, n[1] / l, n[2] / l] as Vec3, d: 0.05 + rnd() * 0.4 }; });
    boxes.push(makeBox({ c: at(hy), h: [0.4 + rnd() * 1.2, hy, 0.2 + rnd() * 0.6], yaw: turn(), cut, shaped: true })); }   // cut to a shape (a shape's record is met whatever the cut-pieces switch says)
  for (let i = 0; i < 6; i++) boxes.push(makeBox({ c: at(1 + rnd()), h: [0.5, 0.5, 0.02], yaw: turn(), flags: BoxFlag.NoShadow }));   // decals: no line meets one
  for (let i = 0; i < 24; i++) boxes.push(makeBox({ c: at(1.25), h: [0.6 + rnd() * 2.4, 1.25, 0.03], yaw: turn(), flags: BoxFlag.Glass }));   // sheets of glass, floor to head height
  const leaves: any[] = []; for (let i = 0; i < 14; i++) leaves.push(makeBox({ c: at(1.0), h: [0.45, 1.0, 0.025], yaw: rnd() * TAU, flags: BoxFlag.Dynamic | (rnd() < 0.4 ? BoxFlag.SoundSeal : 0) }));
  const col: any = new C.StaticCollision(boxes); col.dynamicBoxes = leaves;
  const anywhere = (): Vec3 => [-1 + rnd() * (x1 + 2), rnd() < 0.7 ? [0.15, 0.45, 0.9, 1.1, 1.6, 2.0, 2.6][Math.floor(rnd() * 7)] : -0.5 + rnd() * 4.5, -1 + rnd() * (z1 + 2)];
  const onBox = (): Vec3 => { const b = boxes[Math.floor(rnd() * boxes.length)]; return [b.c[0] + (rnd() * 2 - 1) * (b.h[0] + 0.4), b.c[1] + (rnd() * 2 - 1) * (b.h[1] + 0.2), b.c[2] + (rnd() * 2 - 1) * (b.h[2] + 0.4)]; };
  const onLeaf = (): Vec3 => { const b = leaves[Math.floor(rnd() * leaves.length)]; return [b.c[0] + (rnd() * 2 - 1) * 1.5, b.c[1] + (rnd() * 2 - 1) * 1.2, b.c[2] + (rnd() * 2 - 1) * 1.5]; };
  const pick = (): Vec3 => { const r = rnd(); return r < 0.5 ? anywhere() : r < 0.8 ? onBox() : onLeaf(); };
  const near = (a: Vec3, L: number): Vec3 => { const th = rnd() * TAU; return [a[0] + Math.sin(th) * L, a[1] + (rnd() - 0.5) * 1.2, a[2] + Math.cos(th) * L]; };
  // ---- the lamps: every kind, some somebody's, some out, some passing, some with a steady self; most reach the whole floor, so a point has more in
  // reach than one part of a question carries
  const lamps: any[] = [];
  for (let i = 0; i < 460; i++) { const kind = [0, 0, 1, 1, 3, 3, 2][Math.floor(rnd() * 7)], wide = i < 420;
    lamps.push({ kind, pos: pick(), dir: unit(), color: [rnd(), rnd(), rnd()], intensity: rnd() < 0.05 ? 0 : 5 + rnd() * 400, range: wide ? 55 + rnd() * 10 : 3 + rnd() * 20, innerDeg: 5 + rnd() * 40, outerDeg: 10 + rnd() * 70, radius: rnd() < 0.5 ? 0.05 : rnd() * 0.6,
      owner: rnd() < 0.2 ? 1 + Math.floor(rnd() * 3) : 0, enabled: rnd() > 0.05, ttl: rnd() < 0.1 ? 0.5 : -1, steady: rnd() < 0.2 ? { intensity: 50 + rnd() * 100, pos: pick(), radius: 0.2 } : undefined }); }
  // ---- the smoke: the tool's own blobs under the tool's own numbers (a tuning of the game's moves no word here)
  const blobOf = () => { const p = pick(); return { x: p[0], y0: Math.max(0.1, p[1]), z: p[2], r2: 0.05 + rnd() * 0.9, rate: 0, ttl: 30, age: rnd() * 20, mass: rnd() < 0.1 ? 0 : 0.5 + rnd() * 40 }; };
  const smoke = new SE.CpuSmoke(); smoke.blobs = Array.from({ length: 24 }, blobOf);
  const spots = smoke.blobs.map((b: any): Vec3 => [b.x, b.y0 + 0.5, b.z]);
  // (…and a field for the light: smoke let out, and no grid standing for it — the blobs dim each lamp, inside the lamp loop)
  const field: any = new SE.CpuSmoke(); field.grid = SE.SmokeGrid.make();
  if (!field.grid) { verdict(false, 'made-up world', 'no smoke grid of the tool\'s own could be made: nothing was asked'); return; }
  field.emit({ pos: at(0.2), dir: [0, 1, 0], speed: 0, radius: 0.5, density: 6, temperature: 0, ttl: 60, age: 0, kind: 'canister' }); field.blobs = Array.from({ length: 30 }, blobOf);
  const K0 = { ...SE.SENSE.smoke }, list = g.engine.lights.lights as any[], had = list.slice(), col0 = g.col;
  Object.assign(SE.SENSE.smoke, { sigma: 6, spread: 0.2, widest: 3.5, rise: 1, riseRate: 0.3 });
  list.length = 0; for (const l of lamps) list.push(l); g.col = col;
  const KM = ['sight', 'light', 'shaded', 'smoke', 'sound', 'paths', 'fog'] as const; let regrid = 0, moved = 0;
  const ask: Record<(typeof KM)[number], () => Ask> = {
    sight: () => {
      const eye = pick(), E: any = { pos: eye, look: unit(), range: 4 + rnd() * 40, halfDeg: 20 + rnd() * 160, nearM: rnd() * 4, nearDeg: 60 + rnd() * 120 }; if (rnd() < 0.3) E.vHalfDeg = 10 + rnd() * 60;
      const chest = rnd() < 0.5 ? near(eye, 0.5 + rnd() * 20) : pick(), pc = { bones: { chest, head: [chest[0] + (rnd() - 0.5) * 0.2, chest[1] + 0.5, chest[2] + (rnd() - 0.5) * 0.2] }, pos: chest, alive: rnd() > 0.1 };
      if (rnd() < 0.6) { const d: Vec3 = [chest[0] - eye[0], 0, chest[2] - eye[2]], l = Math.hypot(d[0], d[2]) || 1; E.look = [d[0] / l, (rnd() - 0.5) * 0.3, d[2] / l]; }
      const blind = rnd() < 0.05, st = rnd(), through = rnd() < 0.7 ? BoxFlag.Glass : BoxFlag.Glass | BoxFlag.Thin;   // (never Sam: what hides him is the game's, read after the module has answered)
      return { q: `eye ${eye} look ${E.look} on ${chest} through ${through}`, run: () => G.eyeContact(g, E, pc, false, blind, st, through) };
    },
    light: () => {
      const P = rnd() < 0.4 ? near(lamps[Math.floor(rnd() * lamps.length)].pos, 0.2 + rnd() * 8) : pick(), steady = rnd() < 0.3, skip = rnd() < 0.3 ? (o: number) => o === 2 : undefined;
      return { q: `light on ${P} steady ${steady}`, run: () => G.lightEstimateAt(g, P, skip, steady, null, false) };
    },
    shaded: () => {
      const b = field.blobs[Math.floor(rnd() * field.blobs.length)], P = rnd() < 0.6 ? near([b.x, 1, b.z], rnd() * 5) : pick();
      return { q: `light on ${P} through the field's blobs`, run: () => { SUB = 'shaded'; try { return G.lightEstimateAt(g, P, undefined, false, field, false); } finally { SUB = null; } } };
    },
    smoke: () => {
      const a = rnd() < 0.6 ? near(spots[Math.floor(rnd() * spots.length)], rnd() * 6) : pick(), b = rnd() < 0.6 ? near(a, 0.5 + rnd() * 15) : pick();
      return { q: `smoke ${a} → ${b}`, run: () => smoke.along(col, a, b) };
    },
    sound: () => {
      const src = pick(), dst = rnd() < 0.7 ? near(src, 0.5 + rnd() * 25) : pick();
      return { q: `sound ${src} → ${dst}`, run: () => col.sound.propagate(src, dst, false) };
    },
    paths: () => {
      const a = pick(), b = rnd() < 0.6 ? near(a, 1 + rnd() * 20) : pick(), cap = [6000, 6000, 20000, 400, 40, 3][Math.floor(rnd() * 6)];
      if (rnd() < 0.5) return { q: `way ${a} → ${b} within ${cap}`, run: () => col.nav.findPath(a, b, cap) };
      return priced(col, a, b, cap, Math.floor(rnd() * 0x7fffffff), rnd() < 0.5 ? 0 : 2);
    },
    fog: () => {
      // (every eye with a wedge and a near field of its own: the team's are knobs of the game's)
      const n = 1 + Math.floor(rnd() * 7), chest = pick(), eyes: any[] = [], idx: number[] = [];
      for (let k = 0; k < n; k++) { const pos = rnd() < 0.7 ? near(chest, 0.2 + rnd() * 18) : pick(), e: any = { id: k, pos, at: pos, look: rnd() < 0.03 ? [0, 1, 0] : unit() };
        if (rnd() < 0.5) { const d = [chest[0] - pos[0], chest[2] - pos[2]], l = Math.hypot(d[0], d[1]) || 1, t = (rnd() - 0.5) * 2.4, c = Math.cos(t), s = Math.sin(t); e.look = [(d[0] * c - d[1] * s) / l, (rnd() - 0.5) * 0.3, (d[0] * s + d[1] * c) / l]; }
        e.fov = [360, 360, 360, 359, 200, 120, 90, 60, 30][Math.floor(rnd() * 9)]; e.near = [0, 0.5, 2, 6][Math.floor(rnd() * 4)];
        eyes.push(e); idx.splice(Math.floor(rnd() * (idx.length + 1)), 0, k); }
      const order = idx.reduce((o, i, k) => o | (i << (4 * k)), 0), want = [1, 1, 2, n][Math.floor(rnd() * 4)], body = rnd() < 0.25;
      const pc: any = { alive: !body && rnd() > 0.1, pos: [chest[0], Math.max(0, chest[1] - 1.1), chest[2]], bones: rnd() < 0.9 ? { head: [chest[0] + (rnd() - 0.5) * 0.2, chest[1] + 0.5, chest[2] + (rnd() - 0.5) * 0.2], hips: [chest[0], chest[1] - 0.3, chest[2]] } : {} };
      return { q: `${n} eyes ${order.toString(16)} want ${want} on ${body ? 'a body' : pc.alive ? 'a man' : 'a man down'} at ${chest}`, run: () => { fog.handEyes(eyes); return fog.sees(eyes, order, want, { char: pc }, body ? null : chest); } };
    },
  };
  const per = Math.ceil(N / KM.length), movers = boxes.filter(b => !b.rot && b.h[1] > 0.4 && !(b.flags & (BoxFlag.NoShadow | BoxFlag.Glass)) && rnd() < 0.15);
  // (THE GLASS'S STATE, made up and said twice as the game's own is — the rule a walk of this side asks and the rows the module's copy holds, boxMirror.ts
  //  PaneSource: half of the glass gone whole. Where a pane is open piece by piece is tools/qa/glass-ident.ts's to hold.)
  const goneOf = (bi: number) => (((bi * 7919) >>> 2) & 1) === 0, paneRows: any[] = []; let gone = 0;
  boxes.forEach((b: any, bi: number) => { if ((b.flags & BoxFlag.Glass) && goneOf(bi)) paneRows.push({ bi, gone: true, run: false, sites: [], open: new Uint8Array(0), pending: [], dice: null }); });
  const paneSaid = { each: (to: (r: any) => void) => { for (const r of paneRows) to(r); }, view: null };
  if (BACKWARD) backward(col);
  WHERE = 'made'; let threw = '', asking = ''; const parts0 = C.LAMPS.parts, t0 = there();
  try {
    col.sound.invalidate(); col.tickOpen();
    for (let i = 0; i < per; i++) {
      // a quarter of the way: half the glass is gone (the rule and its rows are hung). A third of the way: the leaves swing. Half way: a share of the statics is re-shaped in place and SAID (or, on the red path, not), and the floor's grid follows
      if (i === Math.floor(per / 3)) { for (const b of leaves) if (rnd() < 0.5) { b.yaw = rnd() * TAU; b.c[0] += (rnd() - 0.5) * 0.6; b.c[2] += (rnd() - 0.5) * 0.6; moved++; } BM.leafWrote(); }
      if (i === Math.floor(per / 4)) { gone = paneRows.length; col.glassRule(goneOf, () => paneSaid); }
      if (i === (per >> 1)) { for (const b of movers) { b.c[0] += (rnd() - 0.5) * 1.5; b.c[2] += (rnd() - 0.5) * 1.5; b.h[1] *= 0.3 + rnd() * 0.7; } if (!UNSAID) col.wrote();
        if (UNSAID) col.nav.there = () => {};
        regrid += col.nav.rebake(0, 0, x1, z1).length >> 1; }
      for (const k of KM) { const Q = ask[k](); asking = `${k}: ${Q.q}`; const a = Q.run(); Q.fold?.(a); handed(k, a); }
    }
  } catch (e) { threw = `${thrown(e)} — asking ${asking.slice(0, 300)}`; }
  finally { col.tickShut(); g.col = col0; list.length = 0; for (const l of had) list.push(l); Object.assign(SE.SENSE.smoke, K0); field.grid.free(); WHERE = ''; }
  const words = FOLD.words(), run = `made-up world · seed ${SEED} · ${per} questions a kind`, end = `${boxes.length} statics, ${movers.length} re-shaped, ${regrid} cells of the floor's grid changed, ${leaves.length} leaves (${moved} swung), ${gone} panes gone a quarter of the way in, ${lamps.length} lamps, ${smoke.blobs.length} and ${field.blobs.length} blobs`;
  const t1 = there(), by = (k: string) => t1[k] - t0[k], want: Record<string, number> = { sight: per, light: 2 * per, smoke: per, sound: per, paths: per, fog: per };   // (`light` and `shaded` are both the light's)
  const thin = Object.keys(want).filter(k => by(k) < want[k]);   // (by the module's own count: a kind it answered less often was answered some other way)
  log(`the made-up world: ${per} questions a kind · ${end} · ${C.LAMPS.parts - parts0} parts of light questions sent ahead`);
  if (threw) verdict(false, 'asked: every question of the made-up world was answered', `it threw: ${threw}`);
  verdict(!thin.length, 'reach: the module counts an answer for every question of every kind', `${Object.keys(want).map(k => `${k} ${by(k)}`).join(' · ')}${thin.length ? ` — TOO FEW of ${thin.join(', ')}` : ''}`);
  if (BACKWARD || AGAIN || UNSHADED) return;   // (red paths of the levels' run: nothing more is held here under one)
  const held = PIN.hold({ root: ROOT, tool: 'tools/qa/sense-ident.ts', fixture: 'sense-words.json', run, runArgs: ['--made', '--n', String(N), '--seed', String(SEED)], end, words, breakWord: 'RUST_BREAK',
    unfit: threw ? 'the run threw' : thin.length ? 'a kind of question did not reach the module' : '', verdict, say: log });
  if (UNSAID) blind.push(...held.stayed);   // (every word of the made-up world must move with the re-shape unsaid)
  if (held.record && !bad) held.record();
}

// ================================================================ OVER LEVELS
async function levels(): Promise<void> {
for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, seed: seedOf(file, 3), ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }   // (the game on a die of its own, as the page's is: the lamps men carry and the doors they have passed are then the same from run to run)
  catch (e) { hush(false); log(`${file.padEnd(20)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was asked`); bad++; continue; }
  hush(false);
  const rnd = die(seedOf(file, 1));
  const g = W.game, col = g.col as any, boxes = col.boxes as any[], fog = g.fogOfWar as any;
  for (const d of g.doors.list) { if (rnd() < 0.6) { d.angle = (rnd() * 2 - 1) * 1.9; d.latched = false; } } hush(true); try { for (let i = 0; i < 3; i++) W.step(1 / 60); } catch { /* a level that needs more to run a frame still has its statics and leaves */ } hush(false);
  if (!X.EXACT.on) { log(`${file.padEnd(20)} FAIL — the stand left the game's own maths down: nothing was asked`); bad++; continue; }
  WHERE = file.replace(/\.json$/, '');
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
  const ask: Record<Kind, () => Ask> = {
    sight: () => {
      const eye = pick(), E: any = { pos: eye, look: unit(), range: 4 + rnd() * 40, halfDeg: 20 + rnd() * 160, nearM: rnd() * 4, nearDeg: 60 + rnd() * 120 }; if (rnd() < 0.3) E.vHalfDeg = 10 + rnd() * 60;
      const chest = rnd() < 0.5 ? near(eye, 0.5 + rnd() * 20) : pick(), pc = { bones: { chest, head: [chest[0] + (rnd() - 0.5) * 0.2, chest[1] + 0.5, chest[2] + (rnd() - 0.5) * 0.2] }, pos: chest, alive: rnd() > 0.1 };
      if (rnd() < 0.6) { const d: Vec3 = [chest[0] - eye[0], 0, chest[2] - eye[2]], l = Math.hypot(d[0], d[2]) || 1; E.look = [d[0] / l, (rnd() - 0.5) * 0.3, d[2] / l]; }   // (most eyes look about at him: the lines are what is asked)
      // (every eye of the game's passes glass: a line that stops at it is no question of an eye's, and the module is not asked one)
      const blind = rnd() < 0.05, isPlayer = rnd() < 0.3, st = rnd(), through = rnd() < 0.7 ? BoxFlag.Glass : rnd() < 0.7 ? F.fogThrough() | BoxFlag.Thin : BoxFlag.Glass | BoxFlag.Thin;
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
      if (rnd() < 0.5) return { q: `way ${a} → ${b} within ${cap}`, run: () => col.nav.findPath(a, b, cap) };
      return priced(col, a, b, cap, Math.floor(rnd() * 0x7fffffff), rnd() < 0.5 ? 0 : 2);   // …half of them WITH A PENALTY
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
  if (BACKWARD) backward(col);
  let threw = '', asking = '', lampsSay = '', lampsOk = true; const t0 = there();
  const per = Math.ceil(N / KINDS.length); let regrid = 0;
  try {
  col.sound.invalidate(); col.tickOpen();
  for (let i = 0; i < per; i++) {
    if (i === (per >> 1)) { for (const b of movers) { b.c[0] += (rnd() - 0.5) * 1.5; b.c[2] += (rnd() - 0.5) * 1.5; b.h[1] *= 0.3 + rnd() * 0.7; } if (!UNSAID) col.wrote();
      // the floor's grid follows the boxes, as it does where a wall is broken; on the red path the module keeps the grid it had (the hand-over is cut)
      if (UNSAID) col.nav.there = () => {};
      const ch = col.nav.rebake(0, 0, x1, z1); regrid += ch.length >> 1; }
    for (const k of KINDS) { const Q = ask[k](); asking = `${k}: ${Q.q}`; const a = Q.run(); Q.fold?.(a); handed(k, a); }
  }
  // THE LAMPS IN PARTS, AND THE SMOKE'S SHADOW (the head of this file)
  asking = 'the lamps in parts';
  if (!UNSAID) {   // (not with the re-shape unsaid: the block's own verdicts are of a copy that is the boxes)
    // (the middle of it all is OPEN FLOOR: a place a man can stand with two metres of floor to each side, so the smoke a case lets out there is on the
    //  plan the grid stands on and round the points asked. A middle dealt anywhere fell off the plan's edge or in a pocket on some level and seed: the
    //  grid answered clear air honestly, the case dimmed no point and proved nothing of the shadow)
    const nav = col.nav, free = (x: number, z: number) => !nav.isBlocked(x, z), open: Vec3[] = [], floor: Vec3[] = [];
    for (let z = 0.75; z < nav.H * nav.cell; z += 1.5) for (let x = 0.75; x < nav.W * nav.cell; x += 1.5) if (free(x, z)) { floor.push([x, 0.9, z]); if (free(x + 2, z) && free(x - 2, z) && free(x, z + 2) && free(x, z - 2)) open.push([x, 0.9, z]); }
    const from = open.length ? open : floor, centre: Vec3 = from.length ? from[Math.floor(rnd() * from.length)] : pick();
    const at = L.length, MANY = 2 * BM.FACE.MAX_LAMPS + 88, to = (p: Vec3, c: Vec3): Vec3 => { const d = [c[0] - p[0], c[1] - p[1], c[2] - p[2]], l = Math.hypot(d[0], d[1], d[2]) || 1; return [d[0] / l, d[1] / l, d[2] / l]; };
    for (let i = 0; i < MANY; i++) { const c = centre, pos = near(c, 0.5 + rnd() * 5), kind = [0, 1, 3][Math.floor(rnd() * 3)];   // (every one in reach of every point asked, and lit toward the middle of them, give or take: a spot and a panel look somewhere)
      L.push({ kind, pos, dir: rnd() < 0.8 ? to(pos, c) : unit(), color: [rnd(), rnd(), rnd()], intensity: 5 + rnd() * 200, range: 14 + rnd() * 10, innerDeg: 20 + rnd() * 40, outerDeg: 65 + rnd() * 20, radius: rnd() < 0.5 ? 0.05 : rnd() * 0.4, owner: 0, enabled: true, ttl: -1 }); }
    const part0 = col.lampsPart, begin0 = col.lampsBegin;
    if (AGAIN) col.lampsPart = function (this: any) { this.lampQ.parts = 0; part0.call(this); };   // (the parts' red path: no part is a later part)
    if (UNSHADED) col.lampsBegin = function (this: any, P: Vec3, shade: any, K: any) { begin0.call(this, P, shade ? { grid: shade.grid, blobs: [] } : null, K); };   // (the shadow's red path: the module is handed no blobs)
    const canister = (f: any, p: Vec3, ttl: number) => f.emit({ pos: [p[0], 0.2, p[2]], dir: [0, 1, 0], speed: 0, radius: 0.5, density: 6, temperature: 0, ttl, age: 0, kind: 'canister' });
    const field = (): any => { const f = new SE.CpuSmoke(); f.grid = SE.SmokeGrid.make(); return f.grid ? f : null; };
    /** the light on each of `pts` under the field `f` (null: no smoke), folded under `name`: how many the module answered and in how many parts, and the answers */
    const asks = (name: string, pts: Vec3[], f: any) => { const n0 = C.SENSED.light, p0 = C.LAMPS.parts, got: number[] = []; SUB = name;
      try { for (const P of pts) got.push(G.lightEstimateAt(g, P, undefined, false, f, false)); } finally { SUB = null; }
      FOLD.doubles(FOLD.call(`${name} · ${WHERE}`), got);   // (…and what the game is handed for each: the meter's one number off the six faces)
      return { n: pts.length, there: C.SENSED.light - n0, parts: C.LAMPS.parts - p0, got }; };
    const round = (n: number): Vec3[] => Array.from({ length: n }, () => near(centre, rnd() * 3));
    const plain = asks('lamps in parts', round(40), null), cases: string[] = [`${plain.n} points under ${at + MANY} lamps: ${plain.there} to the module in ${plain.there + plain.parts} parts`];
    if (plain.there < plain.n || plain.parts < 2 * plain.n) { lampsOk = false; cases.push('THE LAMPS WERE NOT ASKED IN PARTS (three a point at least)'); }
    // …through smoke: a grid that answers · no grid standing (never stepped: sense.ts SmokeGrid.along says −1) · more smoke than bricks (a canister on every free cell of a three-metre lattice).
    // The smoke of a case is let out at the middle and a metre and a half to three sides of it, and those four places are the first four points asked
    const shaded = (name: string, make: (f: any) => void, want: (f: any) => boolean, spots: Vec3[] = [centre, ...[0, 2.1, 4.2].map((th): Vec3 => [centre[0] + Math.sin(th) * 1.5, centre[1], centre[2] + Math.cos(th) * 1.5])]): void => { const f = field(); if (!f) { lampsOk = false; cases.push(`${name}: no grid of the tool's own`); return; }
      try { for (const p of spots) canister(f, p, 60); make(f); const is = want(f), pts = [...spots, ...round(24 - spots.length)], a = asks(`lamps, ${name}`, pts, f), clear = asks('lamps, clear air', pts, null), dimmed = a.got.filter((v, i) => v < clear.got[i]).length;
        if (a.there < a.n || (is && !dimmed)) lampsOk = false; if (is) unable[name] = true;
        cases.push(`${name}${is ? '' : ' (NOT SO ON THIS LEVEL)'}: ${f.blobs.length} blobs · ${a.n} points, ${a.there} to the module, ${dimmed} dimmed by the smoke${is && !dimmed ? ' — NONE' : ''}`); }
      finally { f.grid.free(); } };
    const far: Vec3 = [centre[0] + 3, 1.2, centre[2]];
    shaded('a grid that answers', f => { for (let i = 0; i < 40; i++) { f.step(0.1); f.grid.step(col, 0.1); } }, f => f.grid.stands && f.grid.along(centre, far) !== -1);
    shaded('no grid standing', f => { f.step(4); }, f => !f.grid.stands);
    shaded('more smoke than bricks', f => { const cells: Vec3[] = []; for (let z = 1.5; z < nav.H * nav.cell; z += 3) for (let x = 1.5; x < nav.W * nav.cell; x += 3) if (free(x, z)) cells.push([x, 0.2, z]);
      const d2 = (p: Vec3) => (p[0] - centre[0]) ** 2 + (p[2] - centre[2]) ** 2; cells.sort((a, b) => d2(a) - d2(b) || a[0] - b[0] || a[2] - b[2]);   // (after the case's own four, which the cap keeps: the cells nearest the points asked — the smoke that outruns the bricks is the smoke on their lines)
      for (const p of cells.slice(0, 56)) canister(f, p, 60); for (let i = 0; i < 300; i++) { f.step(0.1); f.grid.step(col, 0.1); } }, f => f.grid.stands && f.grid.along(centre, far) === -1);
    col.lampsPart = part0; col.lampsBegin = begin0; L.length = at;
    lampsSay = `\n${''.padEnd(20)} lamps in parts: ${cases.join(' · ')}`;
  }
  } catch (e) { threw = `${thrown(e)} — asking ${asking.slice(0, 300)}`; }
  finally { col.tickShut(); L.length = own; }
  const t1 = there(), thin = KINDS.filter(k => t1[k] - t0[k] < per);   // (by the module's own count: a kind it answered less often was answered some other way)
  WHERE = '';
  const fail = !!threw || thin.length > 0 || !lampsOk;
  log(`${file.padEnd(20)} ${per} questions a kind · ${boxes.length} statics, ${movers.length} re-shaped${UNSAID ? ' UNSAID' : ''}, ${regrid} cells of the floor's grid changed · ${col.dynamicBoxes.length} leaves · ${own} + 14 lamps · ${smoke.blobs.length} blobs · ${fail ? `FAIL — ${threw ? `it threw: ${threw}` : thin.length ? `not every question went to the module (${thin.join(', ')})` : 'the lamps in parts were not asked as they must be (below)'}` : 'PASS — every question answered by the module'}${lampsSay}`);
  if (fail) bad++;
}
}

if (MADE) await made(); else await levels();
if (!MADE) {
  const unreached = Object.keys(unable).filter(k => !unable[k]);   // (a way the grid cannot answer that no level reached proves nothing of it)
  if (!UNSAID && unreached.length) { log(`    never reached on any level asked: ${unreached.join(', ')}`); bad++; }
  // THE WORDS of this run over levels: held to nothing by themselves on the page's stand (a run's course there moves with every level's edit), to the
  // same run played plain by a child under a red path, and to another checkout's run of the same tool with `--against <root>`.
  const held = PIN.hold({ root: ROOT, tool: 'tools/qa/sense-ident.ts', fixture: 'sense-words.json', run: `levels ${files.join(',')} · seed ${SEED} · ${N} questions a level`, runArgs: ['--n', String(N), '--seed', String(SEED), '--levels', files.join(',')],
    end: `${files.length} levels`, words: FOLD.words(), breakWord: BREAK_WORD, unfit: 'a recording is the made-up world\'s (--made): a level\'s edit moves a level\'s words', verdict, say: log });
  // WHAT A RED PATH MUST MOVE, AND MUST NOT: by the names of the words that moved against the plain run
  const off = held.off, kindOf = (n: string) => n.split(' · ')[0], is = (n: string, k: string) => kindOf(n) === k, lamp = (n: string) => kindOf(n).startsWith('lamps');
  if (UNSAID) blind.push(...KINDS.filter(k => !off.some(n => is(n, k))));   // (every kind of question, on some level)
  else if (BACKWARD) { if (!off.some(n => is(n, 'paths'))) blind.push('the paths (no word of theirs moved)'); blind.push(...off.filter(n => !is(n, 'paths'))); }
  else if (AGAIN) { if (!off.some(n => is(n, 'lamps in parts'))) blind.push('the lamps in parts (no word of theirs moved)'); blind.push(...off.filter(n => !lamp(n))); }
  else if (UNSHADED) { const two = ['lamps, no grid standing', 'lamps, more smoke than bricks']; for (const t of two) if (!off.some(n => is(n, t))) blind.push(`${t} (no word of it moved)`); blind.push(...off.filter(n => !two.includes(kindOf(n)))); }
}
const red = UNSAID || BACKWARD || AGAIN || UNSHADED;
const redSay = !red ? '' : blind.length ? `THE RED PATH (${BREAK_WORD}=1) IS WRONG ABOUT ${blind.slice(0, 12).join(', ')}` : `${BREAK_WORD}=1 moved the words it must and no other`;
log(bad || blind.length ? `=== sense-ident: ${bad} ${MADE ? 'verdict(s)' : 'level(s) or verdict(s)'} FAIL${redSay ? ' — ' + redSay : ''} ===` : '=== sense-ident: ALL PASS ===');
process.exit(blind.length ? 3 : bad ? 1 : 0);
