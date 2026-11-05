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
const BARS: any = await import(`${ROOT}/src/game/winbars.ts`), FALL: any = await import(`${ROOT}/src/game/falling.ts`), MODEL: any = await import(`${ROOT}/src/game/shardModel.ts`), GP: any = await import(`${ROOT}/src/game/glasspen.ts`), SH: any = await import(`${ROOT}/src/game/shards.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 1200), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.PANES_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,glasshouse.json,barracks.json,consulate.json').split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };

/** the names the run must reach, a call or a thing the rule does */
const NAMES: Record<string, number> = { barsBlast: 0, barsTaken: 0, impact: 0, widen: 0, blast: 0, update: 0, annealed: 0, tempered: 0, laminated: 0, runByFire: 0, runByOne: 0, diced: 0, sheetOut: 0, toothOut: 0, heard: 0, raked: 0, supportFell: 0, punched: 0, softCrack: 0, wasOpen: 0, noRoom: 0, clump: 0, block: 0, dagger: 0, crater: 0, viewBlown: 0, viewWary: 0, viewCounted: 0, tollPass: 0, tollSpent: 0, crossHole: 0, crossInto: 0, pathOne: 0, pathTwo: 0, shotAllowed: 0, shotRefused: 0, armPast: 0, armHole: 0, landed: 0 };
/** PANES_BREAK: the name being spoiled, and whether it has just been reached */
let target = '', armed = false;
const MET: Record<string, number> = new Proxy(NAMES, { set(t, k: string, v) { t[k] = v; if (k === target) armed = true; return true; } });
const ANSWERS = new Set(['impact', 'widen', 'blast', 'update', 'barsBlast', 'barsTaken', 'heard', 'toll', 'cross', 'path', 'arm', 'tollPass', 'tollSpent', 'crossHole', 'crossInto', 'pathOne', 'pathTwo', 'shotAllowed', 'shotRefused', 'armPast', 'armHole']);
const PIECES = new Set(['sheetOut', 'toothOut', 'raked', 'supportFell', 'punched', 'clump', 'block', 'dagger', 'crater', 'landed']);
/** a reached name spoils what the module said or holds: its answer, a piece it let go, or (a thing a pane's state came to) the state itself, in the block */
function spoil(g: any, answers: [unknown, unknown], rf: any[]): void {
  armed = false;
  if (ANSWERS.has(target)) { answers[1] = '(spoiled)'; return; }
  if (PIECES.has(target) && rf.length) { const f = rf.find(x => x.fall) ?? rf[0]; if (target === 'landed' && f.fall) f.fall.t += 1e-9; else f.seed++; return; }
  const x = SIM.sim(), s = g.sim.bind(); if (x.panes_row_n(s) === 0) { answers[1] = '(spoiled)'; return; }
  new Float64Array(x.memory.buffer)[(x.panes_row_addr(s, 0) + x.panes_field(8)) / 8] += 1e-6;   // (a row's energy)
}
let calls = 0, differ = 0, standFail = 0; const firsts: string[] = [];
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
const boxes0 = (g: any): number => Math.min(g.col.boxes.length, g.level.boxes.length);
const smallOut = new Set<number>();   // (panes that have lost a small piece to a blow: what fell by support after it stood on it)
const hook = (what: string, twin: any, answers: [unknown, unknown], tf: any[], rf: any[]) => {
  const g = twin.g, gb = g.glassBreak; calls++; MET[what] = (MET[what] ?? 0) + 1; const said = answers[1] as any;
  if (what === 'barsBlast' && said.length) MET.barsTaken++;
  if (what === 'toll') MET[said ? 'tollPass' : 'tollSpent']++;
  if (what === 'cross' && said) { if (said.hole) MET.crossHole++; if (said.into >= 0) MET.crossInto++; }
  if (what === 'path' && said) MET[said.cross.length > 1 ? 'pathTwo' : 'pathOne']++;
  if (what === 'arm' && said) MET[said.cross.length ? 'armPast' : 'armHole']++;
  for (const f of rf) { if (f.cause === 'support' && smallOut.has(f.pane)) MET.toothOut++; else if (f.cause !== 'support' && f.area < GB.GLASSBREAK.toothArea) smallOut.add(f.pane); MET[f.kind === 'clump' ? (f.area > 0.05 ? 'block' : 'clump') : f.kind === 'sheet' ? 'sheetOut' : f.kind === 'dagger' ? 'dagger' : f.grain > 0 ? 'crater' : 'punched']++; if (f.heard) MET.heard++; if (f.cause === 'rake') MET.raked++; if (f.cause === 'support') MET.supportFell++; if (f.landed) MET.landed++; }
  if (armed) spoil(g, answers, rf);
  if (what === 'barsBlast') { const gone: number[] = BARS.barsGone(g), hid: number[] = []; g.col.boxes.forEach((b: any, i: number) => { if ((b.flags & (BX.BoxFlag.Bar | BX.BoxFlag.NoShadow)) === (BX.BoxFlag.Bar | BX.BoxFlag.NoShadow) && i < boxes0(g)) hid.push(i); }); if (!same(hid, gone)) say(`the bars gone: ${JSON.stringify(gone)} in the block, ${JSON.stringify(hid)} hidden`); }
  if (!same(answers[0], answers[1])) say(`${what}: answered ${JSON.stringify(answers[0])} by the twin, ${JSON.stringify(answers[1])} by the module`);
  if (tf.length !== rf.length) say(`${what}: ${tf.length} fragments by the twin, ${rf.length} by the module`);
  else for (let i = 0; i < tf.length; i++) for (const k of FRAG_KEYS) if (!same(tf[i][k], rf[i][k])) { say(`${what}: fragment ${i} (${tf[i].kind}, ${tf[i].cause}, pane ${tf[i].pane}): ${k} is ${JSON.stringify(tf[i][k])} by the twin, ${JSON.stringify(rf[i][k])} by the module`); break; }
  for (const f of rf) {
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

/** stand a level up and put it through `n` rounds of the hand, until `done` says enough (or `shut`: the level stays up for the next call) */
async function hand(file: string, n: number, done: () => boolean = () => false): Promise<void> {
  hush(true); let W: any;
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); const glass = data.boxes.find((b: any) => b.flags?.includes('Glass')), slab = data.boxes[0];
    // (a tee of two panes on the first floor's slab: nothing on the list has a pane running on through another's far face, and that is what a round born in a pane is)
    if (glass) data.boxes.push({ ...glass, id: 'box_tee_a', c: [slab.c[0], 1.2, slab.c[2]], h: [1, 1, 0.006], yaw: 0 }, { ...glass, id: 'box_tee_b', c: [slab.c[0] + 1.006, 1.2, slab.c[2]], h: [0.006, 1, 1], yaw: 0 });
    W = await standUp({ uploadEvery: 1, level: loadLevel(data) }); W.step(1 / 60); }
  catch (e) { hush(false); log(`${file.padEnd(18)} FAIL — not stood up (${String((e as Error).stack).split('\n').slice(0, 12).join(' | ').slice(0, 1500)})`); differ++; standFail++; return; }
  hush(false);
  const g = W.game, col = g.col, gb = g.glassBreak, boxes: any[] = g.level.boxes;
  GLASS.GLASSFX.on = true; g.tune.glassBreak = true; if (!(g.tune.chaos > 0)) g.tune.chaos = 1; GP.GLASSPEN.on = true; g.tune.glassShards = true; BARS.WINBARS.on = true;
  GB.PANES.both = hook;
  const sheets: number[] = []; boxes.forEach((b, i) => { if (GLASS.isSheet(b) && !(b.flags & BX.BoxFlag.NoShadow)) sheets.push(i); });
  if (!sheets.length) { log(`${file.padEnd(18)} no sheet of glass: nothing asked`); GB.PANES.both = null; return; }
  const onPane = (bi: number, u: number, v: number, w: number): Vec3 => { const b = boxes[bi], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), wide = b.h[0] >= b.h[2], lx = wide ? u : w, lz = wide ? w : u; return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz]; };
  const spot = (bi: number, over = 0.98): Vec3 => { const b = boxes[bi], hu = Math.max(b.h[0], b.h[2]); return onPane(bi, (rnd() * 2 - 1) * hu * over, (rnd() * 2 - 1) * b.h[1] * over, (rnd() - 0.5) * 0.01); };
  const man = g.player, ids: number[] = [man.char.id, ...g.guards.map((x: any) => x.char.id)];
  let lastRound: { bi: number; P: Vec3 } | null = null; const tee = boxes.findIndex(b => b.h[0] === 1 && b.h[1] === 1 && b.h[2] === 0.006 && GLASS.isSheet(b));
  for (let r = 0; r < n && !done(); r++) {
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
      if (r % 8 === 3) { const up: number[] = BARS.barsStanding(g), gone: number[] = BARS.barsGone(g), L = up.length ? up : gone; if (L.length) { const b = boxes[pick(L)], at: Vec3 = [b.c[0] + (rnd() - 0.5) * 6, 0.2 + rnd() * 2, b.c[2] + (rnd() - 0.5) * 6]; BARS.barsBlast(g, { at, radius: 0.5 + rnd() * 3, who: pick(ids) }); } if (rnd() < 0.1) BARS.resetBars(g); }   // a charge among a window's bars
      // a round's way through glass: from a stride or a room off, the toll, the crossing, the whole path, the line of fire and a muzzle held past the pane
      if (r % 2 === 0) { const onTee: boolean = tee >= 0 && rnd() < 0.12, was: { bi: number; P: Vec3 } | null = lastRound as { bi: number; P: Vec3 } | null, again: boolean = !onTee && was !== null && rnd() < 0.3, bi: number = onTee ? tee : again ? was!.bi : rnd() < 0.6 && touched.length ? pick(touched) : pick(sheets), b = boxes[bi], side = rnd() < 0.5 ? 1 : -1, off = 0.3 + rnd() * rnd() * 7, skew = rnd() < 0.3 ? 2.5 : 0.4, edge = onTee ? 1 : !again && rnd() < 0.25 ? (rnd() < 0.5 ? 1 : -1) : 0, P: Vec3 = again ? was!.P : edge ? onPane(bi, edge * (Math.max(b.h[0], b.h[2]) - rnd() * 0.004), (rnd() * 2 - 1) * b.h[1] * 0.8, side * Math.min(b.h[0], b.h[2])) : spot(bi), wide = b.h[0] >= b.h[2], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), nx = wide ? sn : cs, nz = wide ? cs : -sn, tx = wide ? cs : sn, tz = wide ? -sn : cs, sk = edge ? edge * (0.5 + rnd() * 1.5) : (rnd() - 0.5) * 2 * skew, up = (rnd() - 0.5) * 0.5;
        let d: Vec3 = [-side * nx + sk * tx, up, -side * nz + sk * tz]; const dl = Math.hypot(d[0], d[1], d[2]); d = [d[0] / dl, d[1] / dl, d[2] / dl];
        const from: Vec3 = [P[0] - d[0] * off, P[1] - d[1] * off, P[2] - d[2] * off], e = pick([0.3, 0.5, 0.6, 0.9, 1, 1.4, 2.2]), who = pick(ids);
        lastRound = { bi, P };
        GP.paneToll(g, bi, P, d, e, Math.floor(rnd() * 3)); GP.paneCrossing(g, bi, from, d, off, e, Math.floor(rnd() * 3));
        const path = GP.roundPath(g, from, d, 60, e);
        const list = path && path.cross.length ? path.cross.map((x: any) => ({ bi: x.bi, t: x.t })) : [{ bi, t: off }];
        GP.shotPasses(g, from, d, list, e);
        if (path && rnd() < 0.5) GP.commitCrossings(g, path.cross, path.len, who, rnd() < 0.3 ? { glass: new Map() } : undefined);
        if (rnd() < 0.25) { const arm: Vec3 = [P[0] - d[0] * 0.35, P[1] - d[1] * 0.35, P[2] - d[2] * 0.35], mz: Vec3 = [P[0] + d[0] * 0.35, P[1] + d[1] * 0.35, P[2] + d[2] * 0.35]; GP.armThroughGlass(g, { id: who, pos: arm, bones: { upperArmR: arm }, muzzle: mz }, d, e); } }
      if (rnd() < 0.85) { const dt = rnd() < 0.8 ? 1 / 60 : pick([1 / 30, 0.2, 1]); g.time += dt; gb.update(dt); }
      // a burst's three views, the same points asked of the twin and of the module
      if (r % 3 === 0) { const L = [...gb.panes.keys()] as number[], from = spot(pick(sheets), 3); from[0] += (rnd() - 0.5) * 3; from[2] += (rnd() - 0.5) * 3; const size = { radius: 1 + rnd() * 3.5, power: 0.3 + rnd() * 1.3 };
        for (const view of ['blown', 'wary', 'counted'] as const) gb.seeThrough(view, from, size, () => { for (let q = 0; q < 12; q++) { const bi = q < 8 && L.length ? pick(L) : pick(sheets), P = spot(bi, q % 5 === 4 ? 1.3 : 1), t = gb.twin.ruleOpen(bi, P); let a = !!col.glassOpen(bi, P); MET[view === 'blown' ? 'viewBlown' : view === 'wary' ? 'viewWary' : 'viewCounted']++; if (armed && target.startsWith('view')) { armed = false; a = !a; } if (a !== t) say(`view ${view}: pane ${bi} at [${P.join(', ')}]: open ${t} by the twin, ${a} by the module`); } });
      }
    } catch (e) { hush(false); say(`${file} round ${r}: threw ${String((e as Error).stack ?? e).slice(0, 500)}`); hush(true); }
    finally { col.tickShut(); hush(false); }
  }
  GB.PANES.both = null;
  if (!target) log(`${file.padEnd(18)} ${sheets.length} sheets, toughened glass ${GB.GLASSREAL.drop ? 'dropping after a beat' : 'coming down in a wave'}, ${n} rounds · ${differ} differ so far`);
}

if (BREAK) {   // THE RED PATH: every name in turn, spoiled the moment it is reached; the run must see every one
  const blind: string[] = [], wide = ['office.json', 'glasshouse.json', 'barracks.json', 'consulate.json'];
  for (const name of Object.keys(NAMES)) {
    target = name; armed = false; differ = 0; firsts.length = 0;
    for (const file of name.startsWith('bars') ? ['barracks.json'] : wide) { if (differ) break; await hand(file, N, () => differ > 0); }
    if (standFail) { log(`${name}: a level did not stand`); process.exit(2); }
    log(`${name.padEnd(14)} ${differ ? 'red' : NAMES[name] ? 'NOT SEEN' : 'NOT REACHED'}${firsts[0] ? ` · ${firsts[0].slice(0, 110)}` : ''}`);
    if (!differ) blind.push(name);
  }
  log(blind.length ? `pane-ident: PANES_BREAK left these green: ${blind.join(', ')}` : `pane-ident: PANES_BREAK is red at all ${Object.keys(NAMES).length} names`);
  process.exit(blind.length ? 0 : 1);
}
for (const file of files) await hand(file, N);
const played = flag('--missions');
if (played) {   // THE GAME: what breaks is what a game breaks, the hook on from before the level stands
  const PT: any = await import('./playthrough.ts'); GB.PANES.both = hook;
  for (const m of played.split(',')) { const [id, seed] = m.split(':'), c0 = calls; hush(true); try { await PT.play(id, Number(seed ?? 1), Number(flag('--cap') ?? 300)); } catch (e) { hush(false); say(`${m}: threw ${String((e as Error).stack ?? e).slice(0, 400)}`); } hush(false); log(`${m.padEnd(22)} ${calls - c0} calls both ways · ${differ} differ so far`); }
  GB.PANES.both = null;
}
for (const f of firsts) log(`    ${f}`);
const thin = Object.keys(MET).filter(k => MET[k] === 0);
log(`pane-ident: ${calls} calls both ways, ${differ} differ · met: ${Object.keys(MET).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length) log(`    never met: ${thin.join(', ')}`);
log(`=== pane-ident (the ${process.env.TPS_PROFILE === 'shipped' ? 'page\'s' : 'frozen'} stand): ${differ ? 'FAIL' : 'PASS'} ===`);
process.exit(differ ? 1 : 0);
