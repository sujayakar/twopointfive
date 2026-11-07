// The 'pellets' family (`bun run tools/qa/probes.ts pellets | pellets-arm | pellets-rule | pellets-energy | pellets-red`):
// A LOAD OF SHOT AND A PANE (src/game/glasspen.ts 'the shell rule', shotgun.ts, glassbreak.ts widen; rebuild/gear-and-glass/pellets.md) — a shell's nine pellets
// leave one honest set of holes in glass. All headless, on the real game stood up as the page has it, the fracture model and the shards raised for each game
// by name (Game.tune), PASS / FAIL each with the numbers. THE GLASS LOOK AND THE ROUNDS THROUGH GLASS ARE SETTLED (GLASSFX.on, GLASSPEN.on: constants of the
// game), so since 2026-11-07 nothing here writes them, and `pellets-identity` is deleted: its whole question was the run with the glass parts off (the words
// frozen on 0ee0094, the look's switch alone moving nothing).
//   pellets-arm        the muzzle pressed through a pane (shoulder one side, muzzle the other): one shell files ONE crossing at the arm point — one blow in the
//                      fracture model, one hole remembered, one exit jet, one 'glass' event — and all nine pellets fly on from the muzzle (the man beyond drops);
//                      a laminated sheet across the arm takes the shell as one mark and lets nothing by
//   pellets-rule       one shell at a plain (annealed), a toughened (tempered) and a laminated pane from 1, 4 and 8 m, many shells each on a mended pane: every
//                      pellet that met the glass is in the shell's record, through or stopped; the record obeys the rule (a mark is never on another mark of
//                      its shell, a joiner always is); the fracture model was given exactly the marks, the holes and the jets exactly the marks that went
//                      through, one hearing event a pane a shell. A shell with pellets BOTH through and stopped in one pane (toughened glass raked at its
//                      limit) is held to the same rule. Prints the table the page quotes: what a shell does to each glass at each range
//   pellets-energy     GLASSPEN.energy.m870 is written down and means what the page says: one plain or toughened sheet square-on, never a second standing
//                      one, never laminated; other rows untouched; a man behind plain glass is dropped by a shell, a man behind the laminated sidelight is not
//   pellets-red        the red paths: with the rule switched off (GLASSPEN.shellR 0) the arm check DOES go red (nine blows at one point); the record
//                      checker flags a doctored record; with the m870 entry taken out the energy check goes red
import { standUp, ROOT, fake } from './headless.ts';
import { swept } from './sweep.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { glassPanes, isSheet, GLASS, resetGlass } = await import(`${ROOT}/src/game/glass.ts`);
const PEN = await import(`${ROOT}/src/game/glasspen.ts`);
const { GLASSPEN, GLASSPEN_STATS, GLASSPEN_HOOKS, paneToll, paneType, roundEnergy, holeCount, resetGlassPen, shotPasses } = PEN;
const { fireWeapon, armThrough } = await import(`${ROOT}/src/game/combat.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { SHOTGUN_LAST, SHOTGUN_STATS } = await import(`${ROOT}/src/game/shotgun.ts`);
const PM = await import(`${ROOT}/src/render/particles.ts`);

type Vec3 = [number, number, number];
type PaneKind = 'annealed' | 'tempered' | 'laminated';
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x)), f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const PEN_SHIPPED = JSON.parse(JSON.stringify(GLASSPEN));
/** every probe leaves the numbers of the rounds-through-glass table as the module ships them, pass, fail or throw */
const guarded = (fn: () => Promise<boolean>) => async () => { const hook = GLASSPEN_HOOKS.crossed, stats = { ...SHOTGUN_STATS }; try { return await fn(); } finally { Object.assign(SHOTGUN_STATS, stats); for (const k of Object.keys(GLASSPEN)) if (!(k in PEN_SHIPPED)) delete (GLASSPEN as any)[k]; Object.assign(GLASSPEN, JSON.parse(JSON.stringify(PEN_SHIPPED))); GLASSPEN_HOOKS.crossed = hook; } };

interface Pane { bi: number; b: any; kind: PaneKind; /** unit, off the face the shot comes at */ out: Vec3; /** the point of the sheet shot at */ p: Vec3; /** clear metres off that face */ room: number; /** that point is chest high (a man can stand at it), not up in a clerestory */ chest: boolean }
/** One office under a seed, the fracture model and the shards as asked (on by default), Sam unseen, everybody parked well away, AI off. */
async function stage(seed: number, parts: { brk?: boolean; shards?: boolean } = {}) {
  Math.random = seeded(seed);
  const W: any = await standUp(); const { game, engine } = W;
  engine.particles = new PM.Particles(fake('device'), fake('sceneLayout')); engine.settings.gpuParticles = true;
  // (what a probe needs of the game's own dials goes on here, by name)
  game.tune.glassBreak = parts.brk ?? true; game.tune.glassShards = parts.shards ?? true; game.tune.chaos = 0.5;
  game.godMode = true; game.aiEnabled = false; game.player.visibility = 0.02;
  for (let i = 0; i < 5; i++) W.step(DT);
  const park = () => game.guards.forEach((gd: any, i: number) => { gd.hold = true; gd.pinned = true; gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.heardUpTo = 1e12; gd.dazzledUntil = 1e9; gd.flashlight.enabled = false; gd.char.update(0); });
  park();
  const P = engine.particles, seen = new WeakSet<any>(), events: any[] = [];
  const step = (n = 1) => { for (let i = 0; i < n; i++) { game.player.visibility = 0.02; W.step(DT); P.pending.length = 0; P.pendingImpulses.length = 0; for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); events.push(e); } } };
  const jets: any[] = []; const hook = GLASSPEN_HOOKS.crossed; GLASSPEN_HOOKS.crossed = (g: any, x: any, who: number) => { jets.push(x); hook?.(g, x, who); };
  const gb = game.glassBreak;
  /** every pane whole again, nothing remembered */
  const mend = () => { gb.reset(); resetGlassPen(game); resetGlass(game); game.decals.clear(); jets.length = 0; };
  /** ONE PULL from a muzzle put at `from`, aimed at `to`, the shoulder a forearm behind it on the line (or where `shoulder` says): the player's own round, no spread */
  const fire = (from: Vec3, to: Vec3, gun = 'm870', shoulder?: Vec3) => {
    const c = game.player.char, m0 = c.muzzle, a0 = c.bones.upperArmR, d = v3.normalize(v3.sub(to, from));
    c.muzzle = [...from] as Vec3; c.bones.upperArmR = shoulder ?? v3.mad(from, d, -0.3);
    try { fireWeapon(game, c, to, true, { weapon: (WEAPONS as any)[gun] }); } finally { c.muzzle = m0; c.bones.upperArmR = a0; }
    step(1);
  };
  /** a sheet of `kind` with `need` m of clear floor off one face — a chest-high one before a clerestory, then the most room; null if the level has none */
  const pane = (kind: PaneKind, need = 0, skip: number[] = []): Pane | null => {
    let best: Pane | null = null;
    for (const bi of glassPanes(game) as number[]) {
      const b = game.level.boxes[bi]; if (skip.includes(bi) || !isSheet(b) || paneType(game, b) !== kind) continue;
      const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), n: Vec3 = b.h[0] < b.h[2] ? [cs, 0, -sn] : [sn, 0, cs];
      const chest = b.c[1] - b.h[1] < 1.1 && b.c[1] + b.h[1] > 1.6; const p: Vec3 = [b.c[0], chest ? 1.35 : b.c[1], b.c[2]];
      for (const s of [1, -1]) {
        const out = v3.scale(n, s) as Vec3; let room = 0;
        for (const D of [9, 8.9, 6, 4.9, 3, 2, 1.9]) { const from = v3.mad(p, out, D) as Vec3; const h = game.col.raycast(from, v3.scale(out, -1), D + 1); if (h && h.index === bi && Math.abs(h.t - (D - Math.min(b.h[0], b.h[2]))) < 0.02) { room = D; break; } }
        if (room >= need + 0.85 && (!best || (chest && !best.chest) || (chest === best.chest && room > best.room))) best = { bi, b, kind, out, p, room, chest };
      }
    }
    return best;
  };
  /** a parked man stood `back` m behind the pane on the shot's line, facing it (he notices nothing) */
  const behind = (X: Pane, back: number, i = 0): any => {
    const gd = game.guards[i]; const at = v3.mad([X.p[0], 0, X.p[2]], X.out, -back) as Vec3;
    gd.char.pos = at; gd.char.vel = [0, 0, 0]; gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(X.out[0], X.out[2]); gd.char.update(0); gd.char.update(DT);
    return gd;
  };
  return { W, game, engine, gb, step, fire, pane, behind, mend, park, events, jets };
}
type Stage = Awaited<ReturnType<typeof stage>>;

/** what ONE shell did to pane X (read straight after the pull; `settle` frames later for the pane's end state) */
interface ShellOut {
  /** the shell's own record on this pane: [x, y, z, through, mark] per pellet that met it (null: this build keeps none) */ rec: number[] | null;
  met: number; through: number; stopped: number; marks: number; joined: number; marksThrough: number; /** pellets that stopped on a mark of their own shell (their energy is filed on it, nothing else) */ joinedStopped: number;
  /** pellets that flew more than a metre from the muzzle before they ended */ flew: number;
  /** blows the fracture model was given on this pane · crack stars drawn on it · holes remembered · exit jets · 'glass' hearing events · crack voices */ blows: number; stars: number; holes: number; jets: number; events: number;
  /** pellets that ended in this pane (shotgun.ts SHOTGUN_LAST.ends) · that ended beyond its far face */ endIn: number; endBeyond: number;
  energy: number; run: boolean; diced: boolean; open: number;
}
function shellAt(S: Stage, X: Pane, from: Vec3, to: Vec3, settle = 0, shoulder?: Vec3): ShellOut {
  const { game, gb } = S; const st0 = gb.panes.get(X.bi); const starsOn = () => game.decals.list.filter((d: any) => d.kind === 'glassHit' && d.pane === X.bi).length; const s0 = starsOn(); const b0 = st0?.hits ?? 0, h0 = holeCount(game, X.bi), j0 = S.jets.filter((x: any) => x.bi === X.bi).length, half0 = Math.min(X.b.h[0], X.b.h[2]), evOn = () => S.events.filter((e: any) => e.kind === 'glass' && e.level === GLASS.hearLevel && Math.abs(v3.dot(v3.sub(e.pos, X.b.c), X.out)) < half0 + 0.02 && Math.hypot(e.pos[0] - X.b.c[0], e.pos[2] - X.b.c[2]) < Math.max(X.b.h[0], X.b.h[2]) + 0.05).length, e0 = evOn();   // (the HIT events of THIS pane: a wide pattern also finds the sheet next door, and a pane coming down is its own, louder event)
  S.fire(from, to, 'm870', shoulder);
  const st = gb.panes.get(X.bi); const rec: number[] | null = SHOTGUN_LAST.glass ? (SHOTGUN_LAST.glass.get(X.bi) ?? []) : null;
  const rows = rec ? rec.length / 5 : 0; let through = 0, marks = 0, marksThrough = 0, joinedStopped = 0;
  for (let i = 0; rec && i < rec.length; i += 5) { if (rec[i + 3]) through++; if (rec[i + 4]) { marks++; if (rec[i + 3]) marksThrough++; } else if (!rec[i + 3]) joinedStopped++; }
  const half = Math.min(X.b.h[0], X.b.h[2]); const ends: any[] = SHOTGUN_LAST.ends;
  const endIn = ends.filter(e => e.index === X.bi).length, endBeyond = ends.filter(e => e.index !== X.bi && v3.dot(v3.sub(e.at, X.p), X.out) < -half - 1e-4).length;
  const out: ShellOut = { rec, met: rows, through, stopped: rows - through, marks, joined: rows - marks, marksThrough, joinedStopped, flew: ends.filter(e => e.t > 1).length,
    blows: (st?.hits ?? 0) - b0, stars: starsOn() - s0, holes: holeCount(game, X.bi) - h0, jets: S.jets.filter((x: any) => x.bi === X.bi).length - j0, events: evOn() - e0,
    endIn, endBeyond, energy: 0, run: false, diced: false, open: 0 };
  if (settle > 0) S.step(settle);
  const info = gb.paneInfo(X.bi), st2 = gb.panes.get(X.bi);
  out.energy = st2?.energy ?? 0; out.run = !!info?.run; out.diced = !!st2?.dice || !!info?.gone; out.open = info?.openFraction ?? 0;
  return out;
}
/** THE RULE, checked on a shell's record by this file's own reading of it (null = it holds; else the first row that breaks it): in order, a pellet is a MARK iff no
 *  earlier mark of the record lies within r of it that it may join (a pellet that went through joins only a mark that went through) */
function recordBreaks(rec: number[], r: number): string | null {
  for (let i = 0; i < rec.length; i += 5) {
    let near = false;
    for (let k = 0; k < i; k += 5) { if (!rec[k + 4] || (rec[i + 3] && !rec[k + 3])) continue; if (Math.hypot(rec[k] - rec[i], rec[k + 1] - rec[i + 1], rec[k + 2] - rec[i + 2]) < r) { near = true; break; } }
    if (near === !!rec[i + 4]) return `pellet row ${i / 5}: ${rec[i + 4] ? 'filed a mark' : 'joined'} with ${near ? 'a' : 'no'} mark of its shell within ${r} m`;
  }
  return null;
}


// ================================================================ pellets-rule
const RANGES = [0.3, 1, 4, 8], SHELLS = swept(2, 40);
/** the rows an ordinary run asks: the ones the lines below read (every kind at every range under QA_SWEEP=all) */
const ASKED = (kind: string, D: number) => swept(0, 1) === 1 || (kind === 'annealed' && D !== 8) || (kind === 'laminated' && D === 1);
interface Row { kind: PaneKind; D: number; n: number; met: number; through: number; stopped: number; marks: number; joined: number; energy: number; failed: number; open: number; beyond: number; failed2: number; bad: string[] }
/** `SHELLS` shells at one pane from D metres, square on, the pane mended between shells (and a second shell on the same pane for 'after two') */
async function rangeRows(seed: number): Promise<Row[]> {
  const S = await stage(seed); const rows: Row[] = [];
  for (const kind of ['annealed', 'tempered', 'laminated'] as PaneKind[]) for (const D of RANGES) {
    if (!ASKED(kind, D)) continue;
    const X = S.pane(kind, D); if (!X) { console.log(`    (no ${kind} sheet with ${D} m of clear floor off a face: row skipped)`); continue; }
    const R: Row = { kind, D, n: SHELLS, met: 0, through: 0, stopped: 0, marks: 0, joined: 0, energy: 0, failed: 0, open: 0, beyond: 0, failed2: 0, bad: [] };
    const from = v3.mad(X.p, X.out, D) as Vec3, to = v3.mad(X.p, X.out, -6) as Vec3;
    for (let k = 0; k < SHELLS; k++) {
      S.mend(); S.park();
      const o = shellAt(S, X, from, to, 150);
      R.met += o.met; R.through += o.through; R.stopped += o.stopped; R.marks += o.marks; R.joined += o.joined; R.energy += o.energy; R.open += o.open; R.beyond += o.endBeyond; if (o.run || o.diced) R.failed++;
      // the wiring, shell by shell
      if (!o.rec) { if (!R.bad.length) R.bad.push('no record of the shell on the glass'); }
      else {
        const why = recordBreaks(o.rec, GLASSPEN.shellR); if (why) R.bad.push(`shell ${k}: ${why}`);
        if (o.blows !== o.marks + o.joinedStopped) R.bad.push(`shell ${k}: ${o.blows} blows filed for ${o.marks} marks and ${o.joinedStopped} pellets stopped on one`);
        if (o.stars !== o.marks) R.bad.push(`shell ${k}: ${o.stars} crack stars drawn for ${o.marks} marks`);
        if (o.holes !== o.marksThrough || o.jets !== o.marksThrough) R.bad.push(`shell ${k}: ${o.holes} holes / ${o.jets} jets for ${o.marksThrough} marks that went through`);
        if (o.stopped !== o.endIn) R.bad.push(`shell ${k}: ${o.endIn} pellets ended in the pane, ${o.stopped} in the record as stopped`);
        if (o.through > o.endBeyond) R.bad.push(`shell ${k}: ${o.endBeyond} pellets ended beyond the pane, ${o.through} in the record as through`);   // (fewer is honest: a pellet that left by the hole an earlier one opened met no glass)
        if (o.events !== (o.marks > 0 ? 1 : 0)) R.bad.push(`shell ${k}: ${o.events} 'glass' events for one shell on one pane`);
      }
      if (swept(0, 1)) { const o2 = shellAt(S, X, from, to, 150); if (o2.run || o2.diced) R.failed2++; }
    }
    rows.push(R);
  }
  return rows;
}

// ================================================================ pellets-energy
export const pelletsEnergy = guarded(async () => {
  const { verdict, done } = family('pellets-energy: the M870 has its own through-glass value, and it means what the page says');
  const e = GLASSPEN.energy.m870;
  verdict(typeof e === 'number' && roundEnergy(WEAPONS.m870) === e && e < GLASSPEN.energy.guard9mm && e > 0, 'GLASSPEN.energy has an m870 entry, under the 9 mm sidearm\'s, and it is what a pellet leaves the muzzle with', `m870 ${e} · 9 mm ${GLASSPEN.energy.guard9mm} · roundEnergy(m870) ${roundEnergy(WEAPONS.m870)}`);
  verdict(GLASSPEN.energy.fn2000 === 1.0 && GLASSPEN.energy.aks74u === 0.9 && GLASSPEN.energy.fiveseven === 0.6 && GLASSPEN.energy.guard9mm === 0.5 && GLASSPEN.cost.annealed === 0.10 && GLASSPEN.cost.tempered === 0.16 && GLASSPEN.cost.laminated === 0.42, 'no other gun\'s energy and no pane\'s cost was moved', JSON.stringify({ energy: GLASSPEN.energy, cost: GLASSPEN.cost }));
  const S = await stage(51); const en = e ?? GLASSPEN.defaultEnergy;
  const toll = (kind: PaneKind, deg = 0, eIn = en, nth = 0) => { const X = S.pane(kind); if (!X) return undefined; const a = deg * Math.PI / 180, tan: Vec3 = [X.out[2], 0, -X.out[0]]; const d = v3.normalize(v3.add(v3.scale(X.out, -Math.cos(a)), v3.scale(tan, Math.sin(a)))) as Vec3; return paneToll(S.game, X.bi, v3.mad(X.p, X.out, Math.min(X.b.h[0], X.b.h[2])), d, eIn, nth); };
  const an = toll('annealed'), te = toll('tempered'), la = toll('laminated');
  verdict(!!an && !!te && la === null, 'square on: one sheet of plain glass — yes; one of toughened — yes; laminated — no, the pellet is spent in it', `plain ${an ? `through, ${f2(an.eOut)} left` : an === null ? 'spent' : 'no such pane'} · toughened ${te ? `through, ${f2(te.eOut)} left` : te === null ? 'spent' : 'no such pane'} · laminated ${la ? `through, ${f2(la.eOut)} left` : la === null ? 'spent' : 'no such pane'}`);
  const an2 = an ? toll('annealed', 0, an.eOut, 1) : undefined, te2 = te ? toll('annealed', 0, te.eOut, 1) : undefined;
  verdict(an2 === null && te2 === null, 'never a second standing pane: what one sheet leaves a pellet is under what the next asks', `after plain glass ${an ? f2(an.eOut) : '-'} → a second plain pane: ${an2 === null ? 'spent' : 'through'} · after toughened ${te ? f2(te.eOut) : '-'} → a plain pane: ${te2 === null ? 'spent' : 'through'}`);
  const lim = (kind: PaneKind) => { let last = -1; for (let d = 0; d <= 85; d++) { if (toll(kind, d)) last = d; else break; } return last; };
  const la0 = lim('annealed'), lt0 = lim('tempered');
  verdict(la0 >= 45 && la0 <= 65 && lt0 >= 15 && lt0 <= 40, 'on the slant: plain glass lets a pellet through to about 56° off its normal, toughened to about 27° — raked past that, the sheet eats the shell', `plain: through to ${la0}° · toughened: through to ${lt0}°`);
  const rifle = { an: paneToll(S.game, S.pane('annealed')!.bi, S.pane('annealed')!.p, v3.scale(S.pane('annealed')!.out, -1), 1.0, 0), la: S.pane('laminated') ? paneToll(S.game, S.pane('laminated')!.bi, S.pane('laminated')!.p, v3.scale(S.pane('laminated')!.out, -1), 1.0, 0) : undefined };
  verdict(!!rifle.an && Math.abs(rifle.an.eOut - 0.9) < 1e-9 && (rifle.la === undefined || (!!rifle.la && Math.abs(rifle.la.eOut - 0.58) < 1e-9)), 'a rifle round pays what it paid (0.10 in plain glass, 0.42 in laminated)', `rifle through plain: ${rifle.an ? f2(rifle.an.eOut) : 'spent'} left · through laminated: ${rifle.la ? f2(rifle.la.eOut) : rifle.la === null ? 'spent' : 'no such pane'}`);
  { const X = S.pane('annealed')!; const pass = shotPasses(S.game, v3.mad(X.p, X.out, 2), v3.scale(X.out, -1), [{ bi: X.bi, t: 2 - 0.006 }], en); const L = S.pane('laminated'); const passL = L ? shotPasses(S.game, v3.mad(L.p, L.out, 2), v3.scale(L.out, -1), [{ bi: L.bi, t: 2 - 0.006 }], en) : false;
    verdict(pass && !passL, 'the line-of-fire question a shotgun man asks reads the same number: a shot through a plain pane is open to him, through the laminated sidelight it is not', `plain ${pass} · laminated ${passL}`); }
  // real men: behind plain glass a shell from 3 m drops him; behind the laminated sidelight he is untouched
  { const X = S.pane('annealed', 3); let down = false, o: ShellOut | null = null;
    if (X) { S.mend(); const man = S.behind(X, 1.5); S.step(2); o = shellAt(S, X, v3.mad(X.p, X.out, 3) as Vec3, v3.mad(X.p, X.out, -6) as Vec3); down = man.state === 'dead'; }
    verdict(!!X && down, 'a man a stride and a half behind a plain pane, one shell from 3 m: down (a pellet that is through is a round — no hit points, and none added)', o ? `through ${o.through} of ${o.met} on the pane · he is ${down ? 'down' : 'standing'}` : 'no plain pane with 3 m of floor');
    const S2 = await stage(52); const L = S2.pane('laminated', 2); let up = true, o2: ShellOut | null = null;
    if (L) { const man = S2.behind(L, 1.2); S2.step(2); o2 = shellAt(S2, L, v3.mad(L.p, L.out, 2) as Vec3, v3.mad(L.p, L.out, -6) as Vec3); up = man.state !== 'dead' && !(man.staggerUntil > S2.game.time); }
    if (L) verdict(up && o2!.through === 0, 'a man behind the laminated sidelight, one shell from 2 m: untouched', `through ${o2!.through} of ${o2!.met} on the pane · he is ${up ? 'standing' : 'hit'}`);
    else console.log('    (no laminated sheet with 2 m of floor off a face: not run)'); }
  return done();
});


if (import.meta.main && process.argv.includes('--table')) { (GLASSPEN as any).shellR ??= 0; if (process.env.SHELLR) GLASSPEN.shellR = +process.env.SHELLR; if (process.env.M870E) GLASSPEN.energy.m870 = +process.env.M870E; const rows = await rangeRows(41); for (const r of rows) console.log(JSON.stringify({ ...r, bad: r.bad.length })); process.exit(0); }
