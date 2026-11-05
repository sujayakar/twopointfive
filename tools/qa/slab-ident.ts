// A LEAF OFF ITS HINGES, THE TYPESCRIPT AGAINST THE RUST MODULE — every loose slab of a level stepped by src/game/doorblast.ts as this repo has always
// stepped it and by rust/sim/src/slabs.rs (the row RUST.slabs), from the same rows and the same boxes, thousands of times a level; everything the two
// leave must be the same to the bit: every column of every slab, the rows' two tables and their counts (src/sim/slabshome.ts SlabStore.bytes), the
// boxes the slabs are posed as, the noises in their order, the tallies and the log.
//   bun run tools/qa/slab-ident.ts [--n 4000] [--levels office.json,compound.json] [--seed 1]
// No game tick runs here: the tool opens the box world's copy as a tick would, moves the slabs' columns home (the game's SimState.settle), and between
// steps does what the game's rules do to a door — a blast that tears a leaf out from either face, hard or weakly, two at once; gunfire that takes one
// apart and leaves its lock rail lying; a hung leaf stood at another angle in a slab's way; every door hung again — and takes ticks of other lengths.
// Two ways of asking, turn about:
//   both   doorblast.ts stepBoth: the module's step, then the TypeScript's from the same row, compared in place (what collision.ts BOTH does in a
//          played mission: tools/qa/rust-check.ts --both)
//   twin   the step as the game takes it with the row on, then everything put back as it was and the step taken with the row off, and what each
//          left compared here
// A run is FAIR only if it met what the step does: slabs torn forward and ones that fall back through their doorway, two in the air at once, the slam,
// coming to rest, a slab moved clear of a static and one laid in its own doorway for want of a spot, a slab found in the middle of a wall, one six seconds
// in the air, a lock rail, doors hung again, a slab thrown at a pane and a pane that was open where a ray met it, and a
// listener that put the game's block out of the module while a noise was said.
// SLABS_BREAK=1 (or RUST_BREAK=1) is the red path: the module is handed a tick one unit in the last place too long, and the run must FAIL.
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1';   // (the rows and the game's own maths are the build's: no word raises them)
import { resolve } from 'path';
const { standUp } = await import('./headless.ts');   // (not a static import: those load before this file's first line, and the harness reads the stand as it loads)
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`), BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`);
const BX: any = await import(`${ROOT}/src/scene/boxes.ts`);
const SH: any = await import(`${ROOT}/src/sim/slabshome.ts`), DB: any = await import(`${ROOT}/src/game/doorblast.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 4000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.SLABS_BREAK === '1' || process.env.RUST_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,compound.json,consulate-gwot.json,mousehole.json,office.json:panes,compound.json:panes').split(',');   // (`:panes` = the level with three statics in ten made panes of glass, a quarter of them gone: rays end on panes all the time, open and shut)
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== slab-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
if (BREAK) { const io = SH.slabIo(), x = io.x, m = () => SH.slabMem(); io.x = { slabs_step: (g: number, w: number) => { m()[io.par + SH.SP.dt] *= 1 + 2 ** -52; return x.slabs_step(g, w); } }; }   // (the red path: a tick a hair too long for the module alone)
const is = Object.is;

const MET = { atPane: 0, torn: 0, back: 0, two: 0, landed: 0, rested: 0, nudged: 0, laid: 0, buried: 0, capped: 0, rails: 0, rehung: 0, open: 0, parked: 0, there: 0, rays: 0, gathers: 0 };
let bad = 0, steps = 0, differ = 0; const firsts: string[] = [];
// enough other games to put any game's block out of the module (a listener below binds them: whoever hears a noise may do as much)
const others: any[] = Array.from({ length: ST.simLayout().games + 1 }, () => new ST.SimState());
for (const entry of files) {
  const [file, mode] = entry.split(':');
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${entry.padEnd(22)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was stepped`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col, doors = g.doors, DL: any[] = doors.list, S = doors.slabs, wood = DL.filter(d => !d.def.steel);
  if (!X.EXACT.on || !SIM.RUST.slabs || !wood.length) { log(`${entry.padEnd(22)} FAIL — ${!wood.length ? 'the level has no leaf a blast can take' : 'the stand left the row or the game\'s own maths down'}: nothing was stepped`); bad++; continue; }
  const panes: any[] = col.boxes.filter((b: any) => (b.flags & BX.BoxFlag.Glass) && !(b.flags & BX.BoxFlag.NoShadow) && !b.rot);
  if (mode === 'panes') {   // (made before the first tick opens, so the module's copy has them as the boxes do)
    let q = 12345; for (const b of col.boxes) { q = (Math.imul(q, 1103515245) + 12345) >>> 0; if ((q >>> 16) % 10 < 3 && !(b.flags & BX.BoxFlag.NoShadow)) b.flags |= BX.BoxFlag.Glass; }
    // (a quarter of the made panes GONE — a made-up state of the glass, said twice as the game's own is: the rule a walk of this side asks, and the rows the
    //  module's copy holds, boxMirror.ts PaneSource. Where a pane is open piece by piece is tools/qa/glass-ident.ts's to hold.)
    const goneOf = (bi: number) => (((bi * 7919) >>> 2) & 3) === 0, rows: any[] = [];
    col.boxes.forEach((b: any, bi: number) => { if ((b.flags & BX.BoxFlag.Glass) && goneOf(bi)) rows.push({ bi, gone: true, run: false, sites: [], open: new Uint8Array(0), pending: [], dice: null }); });
    col.glassWalkable = null; const said = { each: (to: (r: any) => void) => { for (const r of rows) to(r); }, view: null }; col.glassRule((bi: number) => { const o = goneOf(bi); if (o) MET.open++; return o; }, () => said); }
  const walls: any[] = col.boxes.filter((b: any) => !(b.flags & BX.BoxFlag.NoShadow) && b.c[1] + b.h[1] > 1 && b.c[1] - b.h[1] < 0.3);
  const thick: any[] = [...walls].sort((a, b) => Math.min(b.h[0], b.h[2]) - Math.min(a.h[0], a.h[2])).slice(0, 8);   // (the widest of them: no clear spot within reach of their middles, so a slab found there is laid in its own doorway)
  const ids: number[] = [g.player.char.id, ...g.guards.map((x: any) => x.char.id)];
  const sent: any[] = []; let asked = 0;
  (doors as any).emit = (e: any) => { sent.push([DL.indexOf(e.door), e.sound, e.pos[0], e.pos[1], e.pos[2], e.who, e.level]);
    col.rulesWorld(col.dynamicBoxes);   // (…and WHOEVER HEARS ONE ASKS THE MODULE, as a sound's road or a line does in the game —
    if (++asked % 3 === 0) { const at = g.sim.where; for (const o of others) o.bind(); if (at >= 0 && g.sim.where < 0) MET.parked++; } };   //  and now and then so much of it that this game's block is put out of the module's memory)
  // everything a step of the slabs may write besides their rows: the leaves' boxes and handles, the tallies, the log
  const boxesOf = (): any[] => DL.flatMap(d => [d.box, ...d.handleBoxes]);
  const take = () => ({ cols: S.bytes() as Uint8Array, boxes: boxesOf().map(b => ({ c: b.c, cc: [...b.c], h: b.h, hh: [...b.h], yaw: b.yaw, flags: b.flags, rot: b.rot, rr: b.rot ? [...b.rot] : null, marks: b.marks })), stats: { ...DB.DOORBLAST_STATS }, log: DB.DOORBLAST_LOG.length, last: DB.DOORBLAST_LOG[DB.DOORBLAST_LOG.length - 1] });
  const putBack = (t: ReturnType<typeof take>) => { S.put(t.cols); boxesOf().forEach((b, i) => { const o = t.boxes[i]; b.c = o.c; for (let k = 0; k < 3; k++) { b.c[k] = o.cc[k]; } b.h = o.h; for (let k = 0; k < 3; k++) b.h[k] = o.hh[k]; b.yaw = o.yaw; b.flags = o.flags; b.rot = o.rot; if (o.rr) for (let k = 0; k < o.rr.length; k++) b.rot[k] = o.rr[k]; b.marks = o.marks; });
    Object.assign(DB.DOORBLAST_STATS, t.stats); DB.DOORBLAST_LOG.length = t.log; BM.leafWrote(); };
  const seen = () => JSON.stringify([boxesOf().map(b => [b.c, b.h, b.yaw, b.flags, b.rot ?? 0, b.marks === undefined ? 0 : 1]), DB.DOORBLAST_STATS, DB.DOORBLAST_LOG.length, DB.DOORBLAST_LOG[DB.DOORBLAST_LOG.length - 1] ?? '', sent], (_k, v) => typeof v === 'number' && (v !== v || is(v, -0)) ? String(is(v, -0) ? '-0' : v) : v);
  const was = { blast: DB.DOORBLAST.on, shot: DB.DOORSHOT.on }; DB.DOORBLAST.on = true; DB.DOORSHOT.on = true;
  let here = 0, levelBad = 0, twins = 0, why = '';
  const tear = () => { const hung = wood.filter(d => !d.off); if (!hung.length) return; const d = pick(hung), [hx, hz] = d.def.hinge, a = d.def.closedDir + d.angle, s = rnd() < 0.5 ? 1 : -1, t = rnd() * d.def.width, o = 0.2 + rnd() * 1.2;
    const from: Vec3 = [hx + Math.cos(a) * t - Math.sin(a) * o * s, 0.2 + rnd() * 1.2, hz + Math.sin(a) * t + Math.cos(a) * o * s], T0 = DB.DOORBLAST_STATS.back;
    try { if (DB.tearOff(g, d, from, pick(ids), rnd() < 0.3 ? rnd() * 0.3 : rnd())) { MET.torn++; if (DB.DOORBLAST_STATS.back > T0) MET.back++; } } catch (e) { why ||= String((e as Error).message); } };
  for (let r = 0; r < N; r++) {
    const DT = rnd() < 0.85 ? 1 / 60 : pick([1 / 30, 1 / 20, 1 / 120, 1 / 45]);
    hush(true); col.tickOpen(); let line = '';
    try {
      g.sim.settle(g);
      // the rules' own writes between steps
      const k = rnd(), off = wood.filter(d => d.off).length;
      if (S.fly < 2 && k < 0.1) { tear(); if (rnd() < 0.3) tear(); }
      else if (k < 0.115) { const hung = wood.filter(d => !d.off); if (hung.length) { const d = pick(hung), a = rnd() * 6.283; try { if (DB.splinter(g, d, [Math.cos(a), 0, Math.sin(a)], pick(ids), [d.def.hinge[0], 1, d.def.hinge[1]])) MET.rails++; } catch (e) { why ||= String((e as Error).message); } } }
      else if (k < 0.16) { const hung = DL.filter(d => !d.off); if (hung.length) { const d = pick(hung); d.angle = (rnd() - 0.5) * 3; d.vel = 0; d.latched = false; d.place(); BM.leafWrote(); } }   // (a hung leaf stood at another angle, in a slab's way or out of it)
      else if (k < 0.2 && panes.length && S.fly > 0 && col.glassOpen) {   // a slab in the air thrown at a pane of glass: its ray ends on the pane, and whether that is open is the host's to say
        const d = DL[S.doorAt(Math.floor(rnd() * S.fly))], G = pick(panes), thinX = G.h[0] < G.h[2], sg = rnd() < 0.5 ? 1 : -1, nx = thinX ? sg : 0, nz = thinX ? 0 : sg, o = Math.min(G.h[0], G.h[2]) + 0.9 + rnd() * 0.6, sl = S.load(S.rowOf(d.i));
        sl.p[0] = G.c[0] + nx * o; sl.p[2] = G.c[2] + nz * o; sl.v[0] = -nx * (3 + rnd() * 3); sl.v[2] = -nz * (3 + rnd() * 3); d.slab = sl; DB.poseSlab(d); MET.atPane++; }
      else if (k < 0.215 && walls.length && S.fly > 0) {   // a slab in the air found in the middle of a wall: nothing its rays or its settling were written for, and all of their ladder is climbed
        const d = DL[S.doorAt(Math.floor(rnd() * S.fly))], G = rnd() < 0.5 ? pick(thick) : pick(walls), sl = S.load(S.rowOf(d.i));
        sl.p[0] = G.c[0] + (rnd() - 0.5) * 0.2; sl.p[2] = G.c[2] + (rnd() - 0.5) * 0.2; if (rnd() < 0.5) { sl.tip = 1.5; sl.tipV = 3; } d.slab = sl; DB.poseSlab(d); MET.buried++; }
      else if (k < 0.225 && S.fly > 0) { const d = DL[S.doorAt(Math.floor(rnd() * S.fly))], sl = S.load(S.rowOf(d.i)); sl.t = 5.9 + rnd() * 0.2; d.slab = sl; MET.capped++; }   // …and one that has been six seconds in the air: the cap lands it
      else if (off && (k < 0.164 || off > wood.length * 0.7)) { for (const d of DL) if (d.off) { d.reset(); d.place(); MET.rehung++; } BM.leafWrote(); }
      if (S.fly >= 2) MET.two++;
      sent.length = 0; const T0 = { ...SH.SLABS_STEPPED }, A0 = { ...DB.DOORBLAST_STATS };
      if (r % 3 !== 2) { C.BOTH.on = true; SH.SLABS_STEPPED.first = ''; try { DB.doorSlabs(g, DT); } finally { C.BOTH.on = false; } if (SH.SLABS_STEPPED.differ > T0.differ) line = SH.SLABS_STEPPED.first; }
      else {   // the twin: the game's own road with the row on, then all of it put back and the same with the row off
        twins++;
        const t0 = take();
        DB.doorSlabs(g, DT);
        const c1 = S.bytes(), s1 = seen();
        putBack(t0); sent.length = 0; SIM.RUST.slabs = false;
        try { DB.doorSlabs(g, DT); } finally { SIM.RUST.slabs = true; }
        const out: string[] = (SH.slabDiff(S, S.bytes(), c1, 3) as string[]).map(t => `${t} (here | there)`), s2 = seen();
        if (s1 !== s2) { let i = 0; while (i < s1.length && s1[i] === s2[i]) i++; out.push(`the boxes, tallies, log or noises: …${s2.slice(Math.max(0, i - 60), i + 120)} here, …${s1.slice(Math.max(0, i - 60), i + 120)} there`); }
        line = out.length ? `the slabs' step (twin): ${out.join(' · ')}` : '';
      }
      const T1 = SH.SLABS_STEPPED, A1 = DB.DOORBLAST_STATS;
      if (T1.here > T0.here && T1.host === T0.host && r % 3 !== 2) here++;
      MET.landed += A1.landed - A0.landed; MET.nudged += A1.nudged - A0.nudged; MET.laid += A1.laid - A0.laid;
    } catch (e) { why ||= String((e as Error).stack ?? e).slice(0, 600); line ||= `threw: ${String((e as Error).message).slice(0, 200)}`; }
    finally { col.tickShut(); hush(false); }
    steps++; if (line) { differ++; levelBad++; if (firsts.length < 6) firsts.push(`${entry} step ${r}: ${line.slice(0, 900)}`); }
    g.time += DT;
  }
  DB.DOORBLAST.on = was.blast; DB.DOORSHOT.on = was.shot;
  if (levelBad || here > N / 10) bad++;
  log(`${entry.padEnd(22)} ${levelBad || here > N / 10 ? 'FAIL' : 'PASS'} — ${wood.length} leaves a blast can take, ${N} steps (${twins} of them the twin${here ? `; ${here} with a slab the module could not take` : ''}): ${levelBad} differ${why ? ` · a rule worked between steps threw: ${why.slice(0, 300)}` : ''}`);
}
{ const T = SH.SLABS_STEPPED; MET.there = T.there; MET.rays = T.rays; MET.gathers = T.gathers; MET.rested = T.rested; }
for (const f of firsts) log(`    ${f}`);
const thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0);
log(`slab-ident: ${steps} steps over ${files.length} levels, ${differ} differ · met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length && !BREAK) { log(`    NOT A FAIR RUN: never met ${thin.join(', ')}`); bad++; }
log(`=== slab-ident: ${bad ? 'FAIL' : 'PASS'}${BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(bad ? 1 : 0);
