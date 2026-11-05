// WHAT A PANE IS TO PLAY, THE MODULE AGAINST ITS OWN EARLIER ANSWERS — every call of the glass rule (rust/sim/src/panes.rs, pen.rs) folded into a word a name (tools/qa/ident-pin.ts): what each answered, what each let go, and every pane's whole state after it, to the bit.
//   [TPS_PROFILE=shipped] bun run tools/qa/pane-ident.ts [--n 300] [--levels office.json,glasshouse.json] [--seed 1] [--missions room-clear-four:1,...]
// No game tick in the hand: between questions the tool does to the level's glass what a game does — rounds of every energy with and without the caller's word on them, a shell's pellets
// that widen, blasts at every strength near every kind, the glass's own step, walkers into what is left, a pane made another glass, every pane whole. Glass breaking and rounds through glass are made live here whatever the stand holds, and the three newer
// rules (toughened glass that drops, the pane's place, the fall's seam) are held the other way from the stand's on every other level, so both stands reach the rule either way.
// A run is FAIR only if it reached every name below (MET). THE WORDS: held to tools/qa/fixtures/pane-words.json on the frozen stand (--record writes it; the first recording was taken on the last tree where the TypeScript twin stood, with every call of this same run asked of both and none differing).
// PANES_BREAK=1 is the red path: what the module says is bent at every name, and every name's word must move against the same run played plain by a child (exit 1 = red, as it must be).
process.env.TPS_CPUSENSE ??= '1';
import { resolve } from 'path';
import { standUp } from './headless.ts';
import { Fold, hold } from './ident-pin.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), BX: any = await import(`${ROOT}/src/scene/boxes.ts`), GB: any = await import(`${ROOT}/src/game/glassbreak.ts`), GLASS: any = await import(`${ROOT}/src/game/glass.ts`);
const KN: any = await import(`${ROOT}/src/game/rustknobs.ts`), BARS: any = await import(`${ROOT}/src/game/winbars.ts`), FALL: any = await import(`${ROOT}/src/game/falling.ts`), MODEL: any = await import(`${ROOT}/src/game/shardModel.ts`), GP: any = await import(`${ROOT}/src/game/glasspen.ts`), SH: any = await import(`${ROOT}/src/game/shards.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2), args0 = (n: string) => argv.includes(n); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 300), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.PANES_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,glasshouse.json,barracks.json,consulate.json').split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };

/** the names the run must reach, a call or a thing the rule does */
const NAMES: Record<string, number> = { barsBlast: 0, barsTaken: 0, impact: 0, widen: 0, blast: 0, update: 0, annealed: 0, tempered: 0, laminated: 0, runByFire: 0, runByOne: 0, diced: 0, sheetOut: 0, toothOut: 0, heard: 0, raked: 0, supportFell: 0, punched: 0, softCrack: 0, wasOpen: 0, noRoom: 0, clump: 0, block: 0, dagger: 0, crater: 0, viewBlown: 0, viewWary: 0, viewCounted: 0, tollPass: 0, tollSpent: 0, crossHole: 0, crossInto: 0, pathOne: 0, pathTwo: 0, shotAllowed: 0, shotRefused: 0, armPast: 0, armHole: 0, landed: 0 };
const F = new Fold(); let current: number[] = [];
const MET: Record<string, number> = new Proxy(NAMES, { set(t, k: string, v) { t[k] = v; const e = F.call(k); F.doubles(e, current); return true; } });
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
const boxes0 = (g: any): number => Math.min(g.col.boxes.length, g.level.boxes.length);
const smallOut = new Set<number>();   // (panes that have lost a small piece to a blow: what fell by support after it stood on it)
/** every number in a thing, in order (a word as its letters) */
const numbersOf = (x: unknown, out: number[] = []): number[] => {
  if (typeof x === 'number') out.push(x); else if (typeof x === 'boolean') out.push(x ? 1 : 0); else if (x === null || x === undefined) out.push(-1);
  else if (typeof x === 'string') for (let i = 0; i < x.length; i++) out.push(x.charCodeAt(i));
  else if (Array.isArray(x) || ArrayBuffer.isView(x)) for (const v of x as unknown as number[]) numbersOf(v, out);
  else if (x instanceof Set) for (const v of x) numbersOf(v, out); else if (typeof x === 'object') for (const k of Object.keys(x as object)) numbersOf((x as Record<string, unknown>)[k], out);
  return out;
};
/** what a pane's state comes to, as numbers */
const paneNumbers = (p: any, out: number[]): void => { for (const k of PANE_KEYS) numbersOf(p[k], out); for (const t of p.sites) for (const k of SITE_KEYS) numbersOf(t[k], out); numbersOf(p.open, out); for (const q of p.pending) numbersOf([q.fid, q.at, q.cause], out); if (p.dice) numbersOf([p.dice.seed, p.dice.next, p.dice.out], out); };
/** a call of the rule has answered: its names are counted, and what it said and left folded under each */
function said(g: any, what: string, answer: unknown, rf: any[]): void {
  const gb = g.glassBreak; current = numbersOf(answer);
  for (const f of rf) for (const k of FRAG_KEYS) numbersOf(f[k], current);
  for (const p of gb.panes.values()) paneNumbers(p, current);
  numbersOf(BARS.barsGone(g), current); current.push(gb.openSig());
  if (BREAK) { current.push(1); const x = SIM.sim(), s = g.sim.bind(); if (x.panes_row_n(s) > 0) g.sim.d[(x.panes_row_addr(s, 0) + x.panes_field(8)) / 8] += 1e-6; }
  calls++; MET[what] = (MET[what] ?? 0) + 1; const a = answer as any;
  if (what === 'barsBlast' && a.length) MET.barsTaken++;
  if (what === 'toll') MET[a ? 'tollPass' : 'tollSpent']++;
  if (what === 'cross' && a) { if (a.hole) MET.crossHole++; if (a.into >= 0) MET.crossInto++; }
  if (what === 'path' && a) MET[a.cross.length > 1 ? 'pathTwo' : 'pathOne']++;
  if (what === 'arm' && a) MET[a.panes ? 'armPast' : 'armHole']++;
  for (const f of rf) { if (f.cause === 'support' && smallOut.has(f.pane)) MET.toothOut++; else if (f.cause !== 'support' && f.area < GB.GLASSBREAK.toothArea) smallOut.add(f.pane); MET[f.kind === 'clump' ? (f.area > 0.05 ? 'block' : 'clump') : f.kind === 'sheet' ? 'sheetOut' : f.kind === 'dagger' ? 'dagger' : f.grain > 0 ? 'crater' : 'punched']++; if (f.heard) MET.heard++; if (f.cause === 'rake') MET.raked++; if (f.cause === 'support') MET.supportFell++; if (f.landed) MET.landed++; }
}
/** the game's own glass, watched: each call of the rule says what it said, each piece it let go is noted */
function watch(g: any): void {
  const gb = g.glassBreak; let out: any[] = []; const letGo = gb.letGo.bind(gb); gb.letGo = (f: any, x: any) => { out.push(f); return letGo(f, x); };
  for (const m of ['impact', 'widen', 'blast', 'update']) { const fn = gb[m].bind(gb); gb[m] = (...a: unknown[]) => { out = []; const r = fn(...a); said(g, m, r, out); return r; }; }
}
/** stand a level up and put it through `n` rounds of the hand */
async function hand(file: string, n: number): Promise<void> {
  hush(true); let W: any;
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); const glass = data.boxes.find((b: any) => b.flags?.includes('Glass')), slab = data.boxes[0];
    // (a tee of two panes on the first floor's slab: nothing on the list has a pane running on through another's far face, and that is what a round born in a pane is)
    if (glass) data.boxes.push({ ...glass, id: 'box_tee_a', c: [slab.c[0], 1.2, slab.c[2]], h: [1, 1, 0.006], yaw: 0 }, { ...glass, id: 'box_tee_b', c: [slab.c[0] + 1.006, 1.2, slab.c[2]], h: [0.006, 1, 1], yaw: 0 });
    W = await standUp({ uploadEvery: 1, level: loadLevel(data) }); W.step(1 / 60); }
  catch (e) { hush(false); log(`${file.padEnd(18)} FAIL — not stood up (${String((e as Error).stack).split('\n').slice(0, 12).join(' | ').slice(0, 1500)})`); differ++; return; }
  hush(false);
  const g = W.game, col = g.col, gb = g.glassBreak, boxes: any[] = g.level.boxes;
  GLASS.GLASSFX.on = true; g.tune.glassBreak = true; if (!(g.tune.chaos > 0)) g.tune.chaos = 1; GP.GLASSPEN.on = true; g.tune.glassShards = true; BARS.WINBARS.on = true;
  { const flip = files.indexOf(file) % 2 === 1, on = process.env.TPS_PROFILE === 'shipped' ? !flip : flip; GB.GLASSREAL.drop = GB.GLASSREAL.place = SH.SHARDS.byFall = on; KN.knobsToRust(); g.shards.update(0); }   // (the three newer rules, every other level the other way from the stand's: the rule as it stands on either, on the one stand)
  watch(g);
  const ask = <T,>(what: string, fn: () => T): T => { const r = fn(); said(g, what, r, []); return r; };
  const sheets: number[] = []; boxes.forEach((b, i) => { if (GLASS.isSheet(b) && !(b.flags & BX.BoxFlag.NoShadow)) sheets.push(i); });
  if (!sheets.length) { log(`${file.padEnd(18)} no sheet of glass: nothing asked`); return; }
  const onPane = (bi: number, u: number, v: number, w: number): Vec3 => { const b = boxes[bi], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), wide = b.h[0] >= b.h[2], lx = wide ? u : w, lz = wide ? w : u; return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz]; };
  const spot = (bi: number, over = 0.98): Vec3 => { const b = boxes[bi], hu = Math.max(b.h[0], b.h[2]); return onPane(bi, (rnd() * 2 - 1) * hu * over, (rnd() * 2 - 1) * b.h[1] * over, (rnd() - 0.5) * 0.01); };
  const man = g.player, ids: number[] = [man.char.id, ...g.guards.map((x: any) => x.char.id)];
  let lastRound: { bi: number; P: Vec3 } | null = null; const tee = boxes.findIndex(b => b.h[0] === 1 && b.h[1] === 1 && b.h[2] === 0.006 && GLASS.isSheet(b));
  for (let r = 0; r < n; r++) {
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
      if (r % 8 === 3) { const up: number[] = BARS.barsStanding(g), gone: number[] = BARS.barsGone(g), L = up.length ? up : gone; if (L.length) { const b = boxes[pick(L)], at: Vec3 = [b.c[0] + (rnd() - 0.5) * 6, 0.2 + rnd() * 2, b.c[2] + (rnd() - 0.5) * 6]; const gone0 = BARS.barsGone(g).length; BARS.barsBlast(g, { at, radius: 0.5 + rnd() * 3, who: pick(ids) }); said(g, 'barsBlast', Array(BARS.barsGone(g).length - gone0).fill(0), []); } if (rnd() < 0.1) BARS.resetBars(g); }   // a charge among a window's bars
      // a round's way through glass: from a stride or a room off, the toll, the crossing, the whole path, the line of fire and a muzzle held past the pane
      if (r % 2 === 0) { const onTee: boolean = tee >= 0 && rnd() < 0.12, was: { bi: number; P: Vec3 } | null = lastRound as { bi: number; P: Vec3 } | null, again: boolean = !onTee && was !== null && rnd() < 0.3, bi: number = onTee ? tee : again ? was!.bi : rnd() < 0.6 && touched.length ? pick(touched) : pick(sheets), b = boxes[bi], side = rnd() < 0.5 ? 1 : -1, off = 0.3 + rnd() * rnd() * 7, skew = rnd() < 0.3 ? 2.5 : 0.4, edge = onTee ? 1 : !again && rnd() < 0.25 ? (rnd() < 0.5 ? 1 : -1) : 0, P: Vec3 = again ? was!.P : edge ? onPane(bi, edge * (Math.max(b.h[0], b.h[2]) - rnd() * 0.004), (rnd() * 2 - 1) * b.h[1] * 0.8, side * Math.min(b.h[0], b.h[2])) : spot(bi), wide = b.h[0] >= b.h[2], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), nx = wide ? sn : cs, nz = wide ? cs : -sn, tx = wide ? cs : sn, tz = wide ? -sn : cs, sk = edge ? edge * (0.5 + rnd() * 1.5) : (rnd() - 0.5) * 2 * skew, up = (rnd() - 0.5) * 0.5;
        let d: Vec3 = [-side * nx + sk * tx, up, -side * nz + sk * tz]; const dl = Math.hypot(d[0], d[1], d[2]); d = [d[0] / dl, d[1] / dl, d[2] / dl];
        const from: Vec3 = [P[0] - d[0] * off, P[1] - d[1] * off, P[2] - d[2] * off], e = pick([0.3, 0.5, 0.6, 0.9, 1, 1.4, 2.2]), who = pick(ids);
        lastRound = { bi, P };
        const nth = Math.floor(rnd() * 3), nth2 = Math.floor(rnd() * 3); ask('toll', () => GP.paneToll(g, bi, P, d, e, nth)); ask('cross', () => GP.paneCrossing(g, bi, from, d, off, e, nth2));
        const path = ask('path', () => GP.roundPath(g, from, d, 60, e));
        const list = path && path.cross.length ? path.cross.map((x: any) => ({ bi: x.bi, t: x.t })) : [{ bi, t: off }];
        { const ok = GP.shotPasses(g, from, d, list, e); said(g, ok ? 'shotAllowed' : 'shotRefused', ok, []); }
        if (path && rnd() < 0.5) GP.commitCrossings(g, path.cross, path.len, who, rnd() < 0.3 ? { glass: new Map() } : undefined);
        if (rnd() < 0.25) { const arm: Vec3 = [P[0] - d[0] * 0.35, P[1] - d[1] * 0.35, P[2] - d[2] * 0.35], mz: Vec3 = [P[0] + d[0] * 0.35, P[1] + d[1] * 0.35, P[2] + d[2] * 0.35]; ask('arm', () => GP.armThroughGlass(g, { id: who, pos: arm, bones: { upperArmR: arm }, muzzle: mz }, d, e)); } }
      if (rnd() < 0.85) { const dt = rnd() < 0.8 ? 1 / 60 : pick([1 / 30, 0.2, 1]); g.time += dt; gb.update(dt); }
      // a burst's three views
      if (r % 3 === 0) { const L = [...gb.panes.keys()] as number[], from = spot(pick(sheets), 3); from[0] += (rnd() - 0.5) * 3; from[2] += (rnd() - 0.5) * 3; const size = { radius: 1 + rnd() * 3.5, power: 0.3 + rnd() * 1.3 };
        for (const view of ['blown', 'wary', 'counted'] as const) gb.seeThrough(view, from, size, () => { for (let q = 0; q < 12; q++) { const bi = q < 8 && L.length ? pick(L) : pick(sheets), P = spot(bi, q % 5 === 4 ? 1.3 : 1), a = !!col.glassOpen(bi, P); current = [a ? 1 : 0, bi]; if (BREAK) current.push(1); MET[view === 'blown' ? 'viewBlown' : view === 'wary' ? 'viewWary' : 'viewCounted']++; } });
      }
    } catch (e) { hush(false); say(`${file} round ${r}: threw ${String((e as Error).stack ?? e).slice(0, 500)}`); hush(true); }
    finally { col.tickShut(); hush(false); }
  }
  log(`${file.padEnd(18)} ${sheets.length} sheets, toughened glass ${GB.GLASSREAL.drop ? 'dropping after a beat' : 'coming down in a wave'}, ${n} rounds · ${differ} differ so far`);
}

for (const file of files) await hand(file, N);
const played = flag('--missions');
if (played) {   // THE GAME: what breaks is what a game breaks; a played run must not throw
  const PT: any = await import('./playthrough.ts');
  for (const m of played.split(',')) { const [id, seed] = m.split(':'); hush(true); try { await PT.play(id, Number(seed ?? 1), Number(flag('--cap') ?? 300)); } catch (e) { hush(false); say(`${m}: threw ${String((e as Error).stack ?? e).slice(0, 400)}`); } hush(false); log(`${m.padEnd(22)} played`); }
}
for (const f of firsts) log(`    ${f}`);
const thin = Object.keys(NAMES).filter(k => NAMES[k] === 0);
let failed = differ > 0; const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) failed = true; log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? ` · ${detail}` : ''}`); };
if (!BREAK) { log(`pane-ident: ${calls} calls, ${differ} went wrong · met: ${Object.keys(NAMES).map(k => `${k} ${NAMES[k]}`).join(', ')}`); if (!played) verdict(thin.length === 0 || !!flag('--levels'), 'reach: the run reaches every name', thin.length ? `NOT REACHED: ${thin.join(', ')}` : `${Object.keys(NAMES).length} names`); }
const pinned = !flag('--levels') && !flag('--missions');
if (pinned) {
  const words = F.words(), H = hold({ root: ROOT, tool: 'tools/qa/pane-ident.ts', fixture: 'pane-words.json', run: `${files.join(',')} n ${N} seed ${SEED}`, runArgs: [...(flag('--n') ? ['--n', String(N)] : []), ...(flag('--seed') ? ['--seed', String(SEED)] : [])], end: `${calls} calls`, words, breakWord: 'PANES_BREAK', unfit: differ ? 'calls went wrong' : thin.length ? `the run does not reach ${thin.join(', ')}` : '', verdict: BREAK ? (ok, label, detail) => { log(`- ${ok ? 'NOT RED' : 'RED'} — ${label} · ${detail.slice(0, 400)}`); if (!ok) failed = true; } : verdict, say: log });
  if (args0('--words')) process.exit(0);
  if (H.record && !failed) H.record();
}
log(`=== pane-ident (the ${process.env.TPS_PROFILE === 'shipped' ? 'page\'s' : 'frozen'} stand)${BREAK ? ', PANES_BREAK' : ''}: ${failed ? 'FAIL' : 'PASS'} ===`);
process.exit(failed ? 1 : 0);
