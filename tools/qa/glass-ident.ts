// WHERE A PANE IS OPEN, THE TYPESCRIPT AGAINST THE RUST MODULE — the game's own rule (src/game/glassbreak.ts: `openAt(bi, X) || also(bi, X)`, what
// StaticCollision.glassOpen answers) and rust/sim/src/glass.rs over the copy of every pane that is no longer whole (src/scene/boxMirror.ts), asked the
// same points; and a solid line and a ray through the same glass asked of both walks (collision.ts BOTH). Every answer must be the same.
//   [TPS_PROFILE=shipped] bun run tools/qa/glass-ident.ts [--n 1500] [--levels office.json,glasshouse.json] [--seed 1]
// No game tick runs here. The tool opens the box world's copy as a tick would and, between questions, does to the level's glass what a game does: rounds
// into panes of every kind (hard ones that punch, soft ones that only crack), pellets that widen a hole, blasts near them, the glass's own step (pieces
// queued to leave let go, toughened panes come down clump by clump, the support pass), a pane made another kind of glass, every pane made whole. The
// points are on and just off panes that are broken, panes that are whole and glass that is no sheet (a block of a level made solid glass for the run);
// each is asked with no view, as a blast that has just gone off sees glass ('blown'), as a man wary of a burst does ('wary') and as one counting on it
// does ('counted': frag.ts GlassView).
// AND FROM INSIDE THE GLASS'S OWN WORK: every fragment that lets go is handed on while the rule is half way through its writes (GlassBreak.emit: the
// page plays its crack there, which asks a sound's way, and the shards ask lines of their own), so the hook the fragment is handed to asks here too —
// the fragment's own centre and points round it, a line through it and the crack's way to a listener along the pane — of the rule and of the module.
// The stand is what it is asked on (TPS_PROFILE; rust-check runs both): toughened glass comes down in a wave on the frozen stand and drops after a
// beat on the page's. Glass breaking is made live here whatever the stand holds.
// A run is FAIR only if it met what the rule does: float glass with pieces out and with pieces queued to leave, cracks that have run to the frame,
// toughened glass part way down, panes gone, a laminated web, a pane with every site it can hold; points that are open and shut under each view,
// glass on its way out, a pane a burst would break whole and one it would break only near its nearest point; solid glass; lines and rays that passed
// a hole and that a pane stopped; fragments of float glass and clumps of toughened glass asked about as they let go; and a copy that held every pane
// as it stood (no stale row).
// GLASS_BREAK=1 (or RUST_BREAK=1) is the red path: the module is handed each pane with the word "its cracks have run" turned and its last clump's key
// halved, and the run must FAIL.
process.env.TPS_CPUSENSE ??= '1';   // (the rows and the game's own maths are the build's: no word raises them)
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
const BX: any = await import(`${ROOT}/src/scene/boxes.ts`), GB: any = await import(`${ROOT}/src/game/glassbreak.ts`), BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`), GLASS: any = await import(`${ROOT}/src/game/glass.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 1500), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.GLASS_BREAK === '1' || process.env.RUST_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,glasshouse.json,barracks.json,consulate.json').split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== glass-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };

const MET = { holed: 0, queued: 0, run: 0, dicing: 0, gone: 0, web: 0, full: 0, open: 0, shut: 0, blownOpen: 0, blownShut: 0, waryOpen: 0, waryShut: 0, countedOpen: 0, countedShut: 0,
  going: 0, givesWhole: 0, givesNear: 0, solid: 0, linePassed: 0, lineStopped: 0, rayPassed: 0, rayStopped: 0, midPiece: 0, midClump: 0, midHeard: 0, rewritten: 0 };
let bad = 0, asked = 0, differ = 0, stale = 0, lines = 0, wrong = 0; const firsts: string[] = [];
const VIEWS = ['none', 'blown', 'wary', 'counted'] as const;

for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); W.step(1 / 60); }
  catch (e) { hush(false); log(`${file.padEnd(18)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was asked`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col, gb = g.glassBreak, M = col.mirror, boxes: any[] = g.level.boxes;
  GLASS.GLASSFX.on = true; g.tune.glassBreak = true; if (!(g.tune.chaos > 0)) g.tune.chaos = 1;   // (glass breaking live, whatever the stand holds)
  { const b = boxes.find((b: any) => !(b.flags & (BX.BoxFlag.Glass | BX.BoxFlag.NoShadow)) && !b.rot && Math.min(b.h[0], b.h[2]) > 0.08 && b.c[1] + b.h[1] > 1 && b.c[1] - b.h[1] < 1); if (b) b.flags |= BX.BoxFlag.Glass; }   // (solid glass: a block of the level made glass before the copy is first made — the rule never opens it)
  const sheets: number[] = [], thick: number[] = [];
  boxes.forEach((b, i) => { if ((b.flags & BX.BoxFlag.Glass) && !(b.flags & BX.BoxFlag.NoShadow)) (GLASS.isSheet(b) ? sheets : thick).push(i); });
  if (!X.EXACT.on || !SIM.RUST.lines || !SIM.RUST.rays || !GB.glassBreakLive(g) || !sheets.length || col.glassPanes !== gb) {
    log(`${file.padEnd(18)} FAIL — ${!sheets.length ? 'the level has no sheet of glass' : !GB.glassBreakLive(g) ? 'glass breaking is not live on this stand' : 'the stand left a row or the game\'s own maths down, or the glass is not hung on the copy'}: nothing was asked`); bad++; continue; }
  let broke = false;
  const breakIt = () => { if (!BREAK || broke || !M.x) return; broke = true; const x = M.x, world_glass = x.world_glass, at = (t: number, by: number) => x.world_addr(M.slot, t) / by;   // (the red path: the rows are another pane's story)
    M.x = Object.create(x, { world_glass: { value: (w: number, n: number) => { const ww = SIM.u32(), m = SIM.f64(), A = BM.GLASS_AT, w0 = at(A.words, 4), d0 = at(A.doubles, 8); for (let r = 0; r < n; r++) { ww[w0 + r * A.GW + A.flags] ^= A.run; m[d0 + r * A.GD + A.lastKey] *= 0.5; } world_glass(w, n); } } }); };
  // a point of pane bi's own plane: u along it, v up, w off its face
  const onPane = (bi: number, u: number, v: number, w: number): Vec3 => { const b = boxes[bi], cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), wide = b.h[0] >= b.h[2], lx = wide ? u : w, lz = wide ? w : u; return [b.c[0] + cs * lx + sn * lz, b.c[1] + v, b.c[2] - sn * lx + cs * lz]; };
  const spot = (bi: number, over = 1.04): Vec3 => { const b = boxes[bi], hu = Math.max(b.h[0], b.h[2]); return onPane(bi, (rnd() * 2 - 1) * hu * over, (rnd() * 2 - 1) * b.h[1] * over, (rnd() - 0.5) * 0.02); };
  const ids: number[] = [g.player.char.id, ...g.guards.map((x: any) => x.char.id)];
  let levelBad = 0, why = '', rounds = 0, mid = '';
  // FROM INSIDE A RELEASE: the fragment's own centre and two points round it, a line through it, and the crack's way to a listener along the pane
  const shards = gb.onRelease; let inside = false;
  gb.onRelease = (f: any) => {
    if (!inside) { inside = true; try {
      col.rulesWorld(col.dynamicBoxes);
      if (col.paneAsk !== 1) mid ||= `inside a release the copy does not hold the glass (paneAsk ${col.paneAsk})`;
      else { f.kind === 'clump' || f.kind === 'sheet' ? MET.midClump++ : MET.midPiece++;
        for (let q = 0; q < 3; q++) { const P: Vec3 = q === 0 ? f.centre : spot(f.pane, 1.0), a = !!col.glassOpen(f.pane, P), b = M.x.glass_open(M.slot, f.pane, P[0], P[1], P[2]) === 1; asked++;
          if (a !== b) { wrong++; mid ||= `as a ${f.kind} of pane ${f.pane} (${f.type}, ${f.cause}) lets go, at [${P.join(', ')}]: the rule says ${a ? 'open' : 'shut'}, the module ${b ? 'open' : 'shut'}`; } }
        const P: Vec3 = f.centre, bx = boxes[f.pane], wide = bx.h[0] >= bx.h[2], cs = Math.cos(bx.yaw), sn = Math.sin(bx.yaw), nx = wide ? sn : cs, nz = wide ? cs : -sn, ux = wide ? cs : sn, uz = wide ? -sn : cs, th = (rnd() - 0.5) * 0.3;
        const A: Vec3 = [P[0] + nx * 0.6, P[1], P[2] + nz * 0.6], B: Vec3 = [P[0] - nx * 0.6, P[1], P[2] - nz * 0.6], L: Vec3 = [P[0] + (ux * Math.cos(th) + nx * Math.sin(th)) * 3, Math.max(P[1], 0.9), P[2] + (uz * Math.cos(th) + nz * Math.sin(th)) * 3];
        const d0 = C.BOTH.differ, s0 = C.BOTH.stale, was = C.BOTH.on; C.BOTH.on = true; C.BOTH.first = '';
        try { col.segmentBlocked(A, B, 0.02, 0); lines++; col.sound.invalidate(); col.sound.propagate(P, L, false); MET.midHeard++; } finally { C.BOTH.on = was; }   // (the line is compared; the sound's way is the module's alone and is asked here so that a sense asked half way through the rule's writes is ANSWERED — what it answers past a hole is tools/qa/sense-ident.ts's made-up glass to hold)
        if (C.BOTH.differ > d0 || C.BOTH.stale > s0) { stale += C.BOTH.stale - s0; mid ||= `as a ${f.kind} of pane ${f.pane} lets go: ${C.BOTH.first}`; } }
    } catch (e) { mid ||= `inside a release: ${String((e as Error).message).slice(0, 200)}`; } finally { inside = false; } }
    if (shards) shards(f);
  };
  const live = (): any[] => { const out: any[] = []; gb.each((p: any) => out.push(p)); return out; };
  const tally = () => { for (const st of gb.panes.values()) { if (st.gone) MET.gone++; else if (st.dice) MET.dicing++; else { if (st.nOpen > 0) MET.holed++; if (st.pending.length) MET.queued++; if (st.run) MET.run++; if (st.type === 'laminated' && st.sites.length) MET.web++; if (st.sites.length >= 8) MET.full++; } } };
  for (let r = 0; r < N; r++) {
    hush(true); col.tickOpen(); breakIt(); let line = '';
    try {
      // what a game does to glass between questions
      const k = rnd();
      if (k < 0.5) { const n = 1 + Math.floor(rnd() * 3), bi = rnd() < 0.5 && gb.panes.size ? pick([...gb.panes.keys()]) : pick(sheets);   // rounds, a burst of them into one pane
        for (let i = 0; i < n; i++) { const Xp = spot(bi, 0.98), a = rnd() * 6.283; gb.impact(bi, Xp, [Math.cos(a), 0, Math.sin(a)], rnd() < 0.3 ? 0.2 + rnd() * 0.5 : 0.8 + rnd() * 2.5, pick(ids), rnd() < 0.3 ? rnd() < 0.7 : undefined); } }
      else if (k < 0.56 && gb.panes.size) { const bi = pick([...gb.panes.keys()]) as number; for (let i = 0; i < 4; i++) gb.widen(bi, spot(bi, 0.98)); }   // a shell's pellets through a pane it has broken
      else if (k < 0.66) { const bi = pick(sheets), b = boxes[bi], off = 0.2 + rnd() * 2.2, sg = rnd() < 0.5 ? 1 : -1; gb.blast(onPane(bi, (rnd() * 2 - 1) * Math.max(b.h[0], b.h[2]), (rnd() - 0.5) * b.h[1], sg * off), { radius: 1.5 + rnd() * 2.5, power: 0.3 + rnd() * 1.2, who: pick(ids) }); }
      else if (k < 0.672 && gb.panes.size) { const bi = pick([...gb.panes.keys()]) as number; gb.setType(bi, pick(['annealed', 'tempered', 'laminated', null])); }   // a pane made another kind of glass (and whole)
      else if (k < 0.676) gb.reset();   // every pane whole again
      // the glass's own step, most rounds (a pane's pieces leave over the next frames; a toughened pane comes down in a wave)
      if (rnd() < 0.8) { const dt = rnd() < 0.8 ? 1 / 60 : pick([1 / 30, 0.2, 1]); g.time += dt; gb.update(dt); }
      tally(); rounds++; if (mid) { line ||= mid; mid = ''; }
      // the questions: the same points asked of the rule and of the module, under each view
      const L = live(), from: Vec3 = spot(pick(sheets), 3), size = { radius: 1 + rnd() * 3.5, power: 0.3 + rnd() * 1.3 };
      from[0] += (rnd() - 0.5) * 3; from[2] += (rnd() - 0.5) * 3;
      for (const view of VIEWS) {
        const ask = () => {
          col.rulesWorld(col.dynamicBoxes);   // (whoever asks next pays what the glass owes: the copy's panes are written again)
          if (col.paneAsk !== 1) { line ||= `the copy does not hold the glass (paneAsk ${col.paneAsk})`; return; }
          const V = gb.view; if (V && V.kind === BM.VIEW_GIVES) for (let i = 0; i < V.bi.length; i++) { if (V.at[i * 4 + 3] < 0) MET.givesWhole++; else MET.givesNear++; }
          for (let q = 0; q < 24; q++) {
            const bi = q < 14 && L.length ? pick(L).bi : q < 22 ? pick(sheets) : thick.length ? pick(thick) : pick(sheets), P = q % 8 === 7 ? spot(bi, 1.3) : spot(bi, 1.0);
            const a = !!col.glassOpen(bi, P), b = M.x.glass_open(M.slot, bi, P[0], P[1], P[2]) === 1; asked++;
            if (q >= 22 && thick.length) MET.solid++;
            if (view === 'none') a ? MET.open++ : MET.shut++; else if (view === 'blown') { a ? MET.blownOpen++ : MET.blownShut++; if (a && !gb.openAt(bi, P)) MET.going++; }
            else if (view === 'wary') a ? MET.waryOpen++ : MET.waryShut++; else a ? MET.countedOpen++ : MET.countedShut++;
            if (a !== b) wrong++; if (a !== b) line ||= `pane ${bi} (${gb.panes.get(bi)?.type ?? 'never hit'}) at [${P.join(', ')}] under view ${view}: the rule says ${a ? 'open' : 'shut'}, the module ${b ? 'open' : 'shut'}`;
            // a solid line and a ray through the same point, both walks compared (BOTH)
            if (q % 3 === 0) { const b0 = boxes[bi], wide = b0.h[0] >= b0.h[2], cs = Math.cos(b0.yaw), sn = Math.sin(b0.yaw), nx = wide ? sn : cs, nz = wide ? cs : -sn, d = 0.4 + rnd() * 1.2, sg = rnd() < 0.5 ? 1 : -1;
              const A: Vec3 = [P[0] + nx * d * sg + (rnd() - 0.5) * 0.3, P[1] + (rnd() - 0.5) * 0.3, P[2] + nz * d * sg + (rnd() - 0.5) * 0.3], B: Vec3 = [2 * P[0] - A[0], 2 * P[1] - A[1], 2 * P[2] - A[2]];
              const d0 = C.BOTH.differ, s0 = C.BOTH.stale; C.BOTH.on = true; C.BOTH.first = '';
              try { const hit = col.segmentBlocked(A, B, 0.02, 0); hit ? MET.lineStopped++ : MET.linePassed++; const len = Math.hypot(B[0] - A[0], B[1] - A[1], B[2] - A[2]), rd: Vec3 = [(B[0] - A[0]) / len, (B[1] - A[1]) / len, (B[2] - A[2]) / len];
                const h = col.raycast(A, rd, len); h && h.index === bi ? MET.rayStopped++ : MET.rayPassed++; lines += 2; }
              finally { C.BOTH.on = false; }
              if (C.BOTH.differ > d0 || C.BOTH.stale > s0) { stale += C.BOTH.stale - s0; line ||= `under view ${view}: ${C.BOTH.first}`; } }
          }
        };
        if (view === 'none') ask(); else gb.seeThrough(view, from, size, ask);
      }
      { const st = M.stalePanes(); if (st) { stale += st; line ||= `${st} pane row(s) of the copy are not the glass as it stands`; } else MET.rewritten++; }
    } catch (e) { why ||= String((e as Error).stack ?? e).slice(0, 700); line ||= `threw: ${String((e as Error).message).slice(0, 200)}`; }
    finally { col.tickShut(); hush(false); }
    if (line) { differ++; levelBad++; if (firsts.length < 6) firsts.push(`${file} round ${r}: ${line.slice(0, 900)}`); }
  }
  if (levelBad) bad++;
  log(`${file.padEnd(18)} ${levelBad ? 'FAIL' : 'PASS'} — ${sheets.length} sheets of glass and ${thick.length} of solid glass, toughened glass ${GB.GLASSREAL.drop ? 'dropping after a beat' : 'coming down in a wave'}, ${rounds} rounds: ${levelBad} differ${why ? ` · threw: ${why.slice(0, 400)}` : ''}`);
}
for (const f of firsts) log(`    ${f}`);
const thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0);
log(`glass-ident: ${asked} points and ${lines} lines and rays over ${files.length} levels: ${wrong} points answered differently, ${differ} rounds differ, ${stale} stale rows · met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length && !BREAK) { log(`    NOT A FAIR RUN: never met ${thin.join(', ')}`); bad++; }
log(`=== glass-ident (the ${process.env.TPS_PROFILE === 'shipped' ? 'page\'s' : 'frozen'} stand): ${bad ? 'FAIL' : 'PASS'}${BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(bad ? 1 : 0);
