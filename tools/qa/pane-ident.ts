// WHAT A PANE IS TO PLAY, THE MODULE AGAINST ITS TWIN — every call of the glass rule (rust/sim/src/panes.rs, pen.rs) run both ways in one run: the twin (src/game/glassbreakTwin.ts, glasspenTwin.ts) first and the
// module after, each from the state the other had, and what each answered, what each let go, and every pane's whole state compared to the bit.
//   [TPS_PROFILE=shipped] bun run tools/qa/pane-ident.ts [--n 1200] [--levels office.json,glasshouse.json] [--seed 1] [--missions room-clear-four:1,...]
// Two kinds of run. THE HAND: no game tick; between questions the tool does to the level's glass what a game does — rounds of every energy with and without the caller's word on them, a shell's pellets
// that widen, blasts at every strength near every kind, the glass's own step, walkers into what is left, a pane made another glass, every pane whole. THE GAME: a played level with the
// hook on (`--missions`), so what breaks is what a game breaks. Glass breaking and rounds through glass are made live here whatever the stand holds.
// A run is FAIR only if it reached every name below (MET); PANES_BREAK=1 is the red path: the module is spoiled at every name in turn and the run must FAIL on every one.
process.env.TPS_CPUSENSE ??= '1';
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), BX: any = await import(`${ROOT}/src/scene/boxes.ts`), GB: any = await import(`${ROOT}/src/game/glassbreak.ts`), GLASS: any = await import(`${ROOT}/src/game/glass.ts`);
const FALL: any = await import(`${ROOT}/src/game/falling.ts`), MODEL: any = await import(`${ROOT}/src/game/shardModel.ts`), GP: any = await import(`${ROOT}/src/game/glasspen.ts`), SH: any = await import(`${ROOT}/src/game/shards.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 1200), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.PANES_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,glasshouse.json,barracks.json,consulate.json').split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };

/** the names the run must reach, a call or a thing the rule does */
const MET: Record<string, number> = { impact: 0, widen: 0, blast: 0, update: 0, annealed: 0, tempered: 0, laminated: 0, runByFire: 0, runByOne: 0, diced: 0, sheetOut: 0, toothOut: 0, heard: 0, raked: 0, supportFell: 0, punched: 0, softCrack: 0, wasOpen: 0, noRoom: 0, clump: 0, block: 0, dagger: 0, crater: 0, viewBlown: 0, viewWary: 0, viewCounted: 0, tollPass: 0, tollSpent: 0, crossHole: 0, crossInto: 0, pathOne: 0, pathTwo: 0, shotAllowed: 0, shotRefused: 0, armPast: 0, armHole: 0, landed: 0 };
let calls = 0, differ = 0; const firsts: string[] = [];
const say = (what: string) => { differ++; if (firsts.length < 8) firsts.push(what); };
const same = (a: unknown, b: unknown): boolean => {
  if (typeof a === 'number' && typeof b === 'number') return Object.is(a, b);
  if (a === null || b === null || typeof a !== 'object' || typeof b !== 'object') return a === b;
  if (Array.isArray(a) || ArrayBuffer.isView(a)) { const x = a as unknown as ArrayLike<unknown>, y = b as unknown as ArrayLike<unknown>; if (x.length !== y.length) return false; for (let i = 0; i < x.length; i++) if (!same(x[i], y[i])) return false; return true; }
  const ka = Object.keys(a as object), ko = a as Record<string, unknown>, kb = b as Record<string, unknown>; for (const k of ka) if (!same(ko[k], kb[k])) return false; return true;
};
const FRAG_KEYS = ['pane', 'type', 'kind', 'outline', 'centre', 'normal', 'thickness', 'area', 'velocity', 'spin', 'grain', 'cause', 'who', 'seed', 'key', 'heard'];
const PANE_KEYS = ['bi', 'key', 'type', 'uAxis', 'hu', 'hv', 'ht', 'exterior', 'run', 'nOpen', 'hits', 'energy', 'damage', 'nextThink', 'gone', 'lastDir', 'lastWho', 'lastUV', 'soundAt', 'walkable'];
const SITE_KEYS = ['u', 'v', 'rot', 'n', 'seed', 'rho', 'rz', 'e', 'merge', 'hits'];
const hook = (g: any) => (what: string, twin: any, answers: [unknown, unknown], tf: any[], rf: any[]) => {
  const gb = g.glassBreak; calls++; MET[what] = (MET[what] ?? 0) + 1;
  if (!same(answers[0], answers[1])) say(`${what}: answered ${JSON.stringify(answers[0])} by the twin, ${JSON.stringify(answers[1])} by the module`);
  if (tf.length !== rf.length) say(`${what}: ${tf.length} fragments by the twin, ${rf.length} by the module`);
  else for (let i = 0; i < tf.length; i++) for (const k of FRAG_KEYS) if (!same(tf[i][k], rf[i][k])) { say(`${what}: fragment ${i} (${tf[i].kind}, ${tf[i].cause}, pane ${tf[i].pane}): ${k} is ${JSON.stringify(tf[i][k])} by the twin, ${JSON.stringify(rf[i][k])} by the module`); break; }
  for (const f of rf) { MET[f.kind === 'clump' ? (f.area > 0.05 ? 'block' : 'clump') : f.kind === 'sheet' ? 'sheetOut' : f.kind === 'dagger' ? 'dagger' : f.grain > 0 ? 'crater' : 'punched']++; if (f.heard) MET.heard++; if (f.cause === 'rake') MET.raked++; if (f.cause === 'support') MET.supportFell++; if (f.landed) MET.landed++;
    if (f.fall) { const t = FALL.fallOf(g.col, f.centre, f.velocity, MODEL.SHARD.gravity, 0.8); if (!same(t.t, f.fall.t) || !same(t.pos, f.fall.pos)) say(`${what}: a ${f.kind} of pane ${f.pane} comes down at ${JSON.stringify(t)} by the host's fall, ${JSON.stringify(f.fall)} by the module`); } }
  const a = [...twin.panes.values()] as any[], b = [...gb.panes.values()] as any[];
  if (a.length !== b.length) { say(`${what}: ${a.length} panes touched by the twin, ${b.length} by the module`); return; }
  for (let i = 0; i < a.length; i++) {
    const p = a[i], q = b[i]; let line = '';
    for (const k of PANE_KEYS) if (!same(p[k], q[k])) { line = `${k} is ${JSON.stringify(p[k])} by the twin, ${JSON.stringify(q[k])} by the module`; break; }
    if (!line && p.sites.length !== q.sites.length) line = `${p.sites.length} sites by the twin, ${q.sites.length} by the module`;
    if (!line) for (let s = 0; s < p.sites.length && !line; s++) for (const k of SITE_KEYS) if (!same(p.sites[s][k], q.sites[s][k])) { line = `site ${s}: ${k} is ${JSON.stringify(p.sites[s][k])} by the twin, ${JSON.stringify(q.sites[s][k])} by the module`; break; }
    if (!line && !same(p.open, q.open)) line = 'the pieces out differ';
    if (!line) { if (p.pending.length !== q.pending.length) line = `${p.pending.length} queued by the twin, ${q.pending.length} by the module`; else for (let j = 0; j < p.pending.length && !line; j++) { const u = p.pending[j], v = q.pending[j]; if (u.fid !== v.fid || !Object.is(u.at, v.at) || u.cause !== v.cause || !same(u.from, v.from) || !Object.is(u.k, v.k)) line = `queue place ${j}: ${JSON.stringify(u)} by the twin, ${JSON.stringify(v)} by the module`; } }
    if (!line && (!!p.dice !== !!q.dice)) line = 'dicing by one and not the other';
    if (!line && p.dice) for (const k of ['u', 'v', 'cols', 'rows', 'seed', 'hu', 'hv', 't0', 'next', 'out']) if (!same(p.dice[k], q.dice[k])) { line = `dice ${k} is ${JSON.stringify(p.dice[k])} by the twin, ${JSON.stringify(q.dice[k])} by the module`; break; }
    if (!line && !Object.is(twin.openFraction(p.bi), gb.openFraction(p.bi))) line = `open share ${twin.openFraction(p.bi)} by the twin, ${gb.openFraction(p.bi)} by the module`;
    if (!line && p.type === 'annealed' && p.sites.length && !p.dice) { const h = twin.heldSet(p), j = gb.heldSet(q); if (h.size !== j.size || [...h].some((x: number) => !j.has(x))) line = `held pieces ${h.size} by the twin, ${j.size} by the module`; }
    if (line) { say(`${what}: pane ${p.bi} (${p.type}): ${line}`); break; }
    if (p.run) { if (what === 'blast') MET.runByFire += 0; }
  }
  if (!same(twin.openSig(), gb.openSig())) say(`${what}: the open signature differs`);
};

for (const file of files) {
  hush(true); let W: any;
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); W.step(1 / 60); }
  catch (e) { hush(false); log(`${file.padEnd(18)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 120)})`); differ++; continue; }
  hush(false);
  const g = W.game, col = g.col, gb = g.glassBreak, boxes: any[] = g.level.boxes;
  GLASS.GLASSFX.on = true; g.tune.glassBreak = true; if (!(g.tune.chaos > 0)) g.tune.chaos = 1; GP.GLASSPEN.on = true; g.tune.glassShards = true;
  GB.PANES.both = hook(g);
  const sheets: number[] = []; boxes.forEach((b, i) => { if (GLASS.isSheet(b) && !(b.flags & BX.BoxFlag.NoShadow)) sheets.push(i); });
  if (!sheets.length) { log(`${file.padEnd(18)} no sheet of glass: nothing asked`); GB.PANES.both = null; continue; }
  const onPane = (bi: number, u: number, v: number, w: number): Vec3 => { const b = boxes[bi], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), wide = b.h[0] >= b.h[2], lx = wide ? u : w, lz = wide ? w : u; return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz]; };
  const spot = (bi: number, over = 0.98): Vec3 => { const b = boxes[bi], hu = Math.max(b.h[0], b.h[2]); return onPane(bi, (rnd() * 2 - 1) * hu * over, (rnd() * 2 - 1) * b.h[1] * over, (rnd() - 0.5) * 0.01); };
  const man = g.player, ids: number[] = [man.char.id, ...g.guards.map((x: any) => x.char.id)];
  for (let r = 0; r < N; r++) {
    hush(true); col.tickOpen();
    try {
      const k = rnd(), touched = [...gb.panes.keys()] as number[];
      if (k < 0.45) { const n = 1 + Math.floor(rnd() * 3), bi = rnd() < 0.55 && touched.length ? pick(touched) : pick(sheets);   // rounds, a burst into one pane
        for (let i = 0; i < n; i++) { const X = spot(bi), a = rnd() * 6.283, soft = rnd() < 0.25, e = soft ? 0.1 + rnd() * 0.35 : rnd() < 0.15 ? 2.5 + rnd() : 0.5 + rnd() * 1.8, th = rnd() < 0.35 ? rnd() < 0.7 : undefined; const res = gb.impact(bi, X, rnd() < 0.05 ? null : [Math.cos(a), 0, Math.sin(a)], e, pick(ids), th);
          if (res.wasOpen) MET.wasOpen++; if (res.type && MET[res.type] !== undefined) MET[res.type]++; if (soft && !res.released) MET.softCrack++; if (res.diced) MET.diced++; if (e >= 2.5 && gb.panes.get(bi)?.run) MET.runByOne++; if (e < 2.5 && gb.panes.get(bi)?.run) MET.runByFire++; if (gb.panes.get(bi)?.sites.length >= 8) MET.noRoom++; } }
      else if (k < 0.52 && touched.length) { const bi = pick(touched); for (let i = 0; i < 4; i++) gb.widen(bi, spot(bi)); }
      else if (k < 0.64) { const bi = pick(sheets), b = boxes[bi], off = 0.1 + rnd() * 2.2, sg = rnd() < 0.5 ? 1 : -1; const at = onPane(bi, (rnd() * 2 - 1) * Math.max(b.h[0], b.h[2]), (rnd() - 0.5) * b.h[1], sg * off), o = { radius: 1.5 + rnd() * 2.5, power: 0.3 + rnd() * 1.2, who: pick(ids) }; gb.blast(at, o); if (rnd() < 0.3) gb.blast([at[0], Math.max(0.3, at[1]), at[2]], { radius: 2.6, power: 0.6, who: o.who }); }
      else if (k < 0.70 && touched.length) { const bi = pick(touched), b = boxes[bi], w = rnd() < 0.5 ? 0.3 : -0.3, wide = b.h[0] >= b.h[2], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), nx = wide ? sn : cs, nz = wide ? cs : -sn, P = onPane(bi, (rnd() - 0.5) * Math.max(b.h[0], b.h[2]), 0, w); man.char.pos = [P[0], Math.min(P[1], b.c[1]), P[2]]; const sign = w > 0 ? -1 : 1; man.char.vel = [nx * sign * 2.2, 0, nz * sign * 2.2]; }   // a man walks at what is left
      else if (k < 0.712 && touched.length) gb.setType(pick(touched), pick(['annealed', 'tempered', 'laminated', null]));
      else if (k < 0.716) gb.reset();
      if (rnd() < 0.85) { const dt = rnd() < 0.8 ? 1 / 60 : pick([1 / 30, 0.2, 1]); g.time += dt; gb.update(dt); }
      // a burst's three views, the same points asked of the twin and of the module
      if (r % 3 === 0) { const L = [...gb.panes.keys()] as number[], from = spot(pick(sheets), 3); from[0] += (rnd() - 0.5) * 3; from[2] += (rnd() - 0.5) * 3; const size = { radius: 1 + rnd() * 3.5, power: 0.3 + rnd() * 1.3 };
        for (const view of ['blown', 'wary', 'counted'] as const) gb.seeThrough(view, from, size, () => { for (let q = 0; q < 12; q++) { const bi = q < 8 && L.length ? pick(L) : pick(sheets), P = spot(bi, q % 5 === 4 ? 1.3 : 1), a = !!col.glassOpen(bi, P), t = gb.twin.ruleOpen(bi, P); MET[view === 'blown' ? 'viewBlown' : view === 'wary' ? 'viewWary' : 'viewCounted']++; if (a !== t) say(`view ${view}: pane ${bi} at [${P.join(', ')}]: open ${t} by the twin, ${a} by the module`); } });
      }
    } catch (e) { hush(false); say(`${file} round ${r}: threw ${String((e as Error).stack ?? e).slice(0, 500)}`); hush(true); }
    finally { col.tickShut(); hush(false); }
  }
  GB.PANES.both = null;
  log(`${file.padEnd(18)} ${sheets.length} sheets, toughened glass ${GB.GLASSREAL.drop ? 'dropping after a beat' : 'coming down in a wave'}, ${N} rounds · ${differ} differ so far`);
}
for (const f of firsts) log(`    ${f}`);
const thin = Object.keys(MET).filter(k => MET[k] === 0);
log(`pane-ident: ${calls} calls both ways, ${differ} differ · met: ${Object.keys(MET).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length) log(`    never met: ${thin.join(', ')}`);
log(`=== pane-ident (the ${process.env.TPS_PROFILE === 'shipped' ? 'page\'s' : 'frozen'} stand): ${differ ? 'FAIL' : 'PASS'} ===`);
process.exit(differ ? 1 : 0);
