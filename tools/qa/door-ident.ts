// THE DOORS' STEP, THE TYPESCRIPT AGAINST THE RUST MODULE — every leaf of a level stepped by src/game/doors.ts Doors.update as this repo has always stepped it
// and by rust/sim/src/doors.rs (the row RUST.doors), from the same state and the same men, thousands of times a level; everything the two leave must be the
// same to the bit: every column of every door (src/sim/doorshome.ts DC), where each man ends and what a leaf did to him, the noises in their order, each
// leaf's box and handles, the tallies.
//   bun run tools/qa/door-ident.ts [--n 6000] [--levels office.json,barracks.json] [--seed 1]
// No game tick runs here: the tool opens the box world's copy as a tick would, moves the leaves' columns home (the game's SimState.settle), and steps the
// doors itself over men it makes up — walkers, runners, crouched men, keyed staff, a player's man who is solid to a pulled leaf, limp bodies — who walk at
// the leaves for a while and are then made anew. Between steps it works the doors as the game's own rules do (a hand, a pull, a boot, a pick, a wedge of a
// shove written straight onto the leaf, an angle eased, a lock thrown) and turns the doors' switches over (one-way leaves, doors that stay, the throw, the
// soft start, exits that open out). Two ways of asking, turn about:
//   both   Doors.updateBoth: the module's step with nothing of it kept, then the TypeScript's from the same columns, compared in place (what collision.ts
//          BOTH does in a played mission: tools/qa/rust-check.ts --both)
//   twin   the step as the game takes it with the row on (the module's, taken back into the game: the boxes follow their rows, the noises are heard), then
//          everything put back as it was and the TypeScript's own step taken, and what each left compared here
// A run is FAIR only if it met every kind of thing a leaf does: each noise of a step, a man struck, carried, blocked by a lock, a leaf bashed and ripped,
// a man gone round one, the player's man moved by a pull and stopped on. A level that does not stand up is a FAIL, not a level left out.
// DOORS_BREAK=1 (or RUST_BREAK=1) is the red path: the module is handed a tick one unit in the last place too long, and the run must FAIL.
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.TPS_EXACT = '1'; process.env.TPS_RUSTDOORS = '1';   // (the game's own maths: the step runs in the module under no other)
import { readdirSync } from 'fs';
import { resolve } from 'path';
const { standUp } = await import('./headless.ts');   // (not a static import: those load before this file's first line, and the harness reads the stand as it loads)
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
const D: any = await import(`${ROOT}/src/game/doors.ts`), DH: any = await import(`${ROOT}/src/sim/doorshome.ts`), BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 6000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.DOORS_BREAK === '1' || process.env.RUST_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,barracks.json,compound.json,warehouse.json').split(',');
void readdirSync;
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== door-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
if (BREAK) { const P = D.Doors.prototype, hand = P.handOver; P.handOver = function (this: unknown, dt: number, c: unknown, slot: number) { return hand.call(this, dt * (1 + 2 ** -52), c, slot); }; }   // (the red path: a tick a hair too long for the module alone)
const is = Object.is;

interface Man { pos: Vec3; vel: Vec3; left: number; base: any }
const MET = { asked: 0, heldAsked: 0, creak: 0, bang: 0, latch: 0, bash: 0, rattle: 0, struck: 0, moved: 0, blocked: 0, bashed: 0, ripped: 0, rounds: 0, carried: 0, stopped: 0, held: 0, reached: 0, drives: 0, turned: 0 };
let bad = 0, steps = 0, differ = 0, guards = 0; const firsts: string[] = [];   // (guards: the men the levels stood up — a held man is one of them)
for (const file of files) {
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${file.padEnd(20)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was stepped`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col, doors = g.doors, L: any[] = doors.list, n = L.length;
  if (!X.EXACT.on || !SIM.RUST.doors || !n) { log(`${file.padEnd(20)} FAIL — ${!n ? 'the level has no door' : 'the stand left the row or the game\'s own maths down'}: nothing was stepped`); bad++; continue; }
  const ids: number[] = [g.player.char.id, ...g.guards.map((x: any) => x.char.id), ...(g.civilians ?? []).map((x: any) => x.char.id)];
  const guardIds: number[] = g.guards.map((x: any) => x.char.id); guards += guardIds.length;
  // a man made up near a door: on one face of it, walking at it or along it
  const man = (): Man => {
    const d = pick(L), [hx, hz] = d.def.hinge, w = d.def.width, a = d.def.closedDir + d.angle, ux = Math.cos(a), uz = Math.sin(a), side = rnd() < 0.5 ? 1 : -1;
    const along = rnd() < 0.8 ? rnd() * (w + 0.3) : -0.4 + rnd() * (w + 1.2), off = 0.15 + rnd() * rnd() * 1.6;
    const pos: Vec3 = [hx + ux * along - uz * off * side, rnd() < 0.9 ? 0 : rnd() * 3, hz + uz * along + ux * off * side];
    const sp = pick([0, 0.4, 1.2, 1.6, 2.6, 4.5, 6]), th = rnd() < 0.7 ? Math.atan2(-ux * side, uz * side) + (rnd() - 0.5) * 1.2 : rnd() * 6.283;   // (mostly AT the leaf)
    const vel: Vec3 = [Math.sin(th) * sp, 0, Math.cos(th) * sp];
    const kind = rnd(), who = kind < 0.25 ? ids[0] : kind < 0.85 ? (guardIds.length && rnd() < 0.5 ? pick(guardIds) : pick(ids)) : pick([33, 35, 41, 47, -1]);
    const base: any = { radius: pick([0.34, 0.34, 0.3, 0.42]), bash: sp > 2.2 && rnd() < 0.8, who, quiet: sp < 1 && rnd() < 0.5 };
    if (kind < 0.25) { base.solid = rnd() < 0.8; if (rnd() < 0.3) base.keys = new Set([pick(L).lockId]); if (base.bash && rnd() < 0.7) base.run = { bash: 5 + rnd() * 4, rip: 3 + rnd() * 4, ripLevel: 0.4 + rnd() * 0.5 }; }
    else if (kind < 0.85) { if (rnd() < 0.75) base.key = true; if (rnd() < 0.4) base.hand = pick([0.05, 0.3, 0.8, 2]); if (rnd() < 0.5) base.round = true; }
    if (rnd() < 0.12) { base.weak = true; base.quiet = true; base.bash = false; base.radius = 0.12 + rnd() * 0.2; delete base.key; delete base.solid; }
    return { pos, vel, left: 5 + Math.floor(rnd() * 50), base };
  };
  const men: Man[] = []; for (let i = 0; i < 14; i++) men.push(man());
  const kept = new Map<Man, any>();   // (a man's contact is one object from step to step, as the game keeps a player's: what a step leaves on it the next one finds)
  const contactsOf = (): any[] => men.map(m => { let c = kept.get(m); if (!c) { c = { ...m.base, pos: m.pos, ...(rnd() < 0.85 || m.base.solid ? { vel: m.vel } : {}) }; kept.set(m, c); } return c; });
  const from = (d: any): Vec3 => { const [hx, hz] = d.def.hinge, a = d.def.closedDir + d.angle, s = rnd() < 0.5 ? 1 : -1, t = rnd() * d.def.width, o = 0.3 + rnd(); return [hx + Math.cos(a) * t - Math.sin(a) * o * s, 0, hz + Math.sin(a) * t + Math.cos(a) * o * s]; };
  const sent: any[] = []; const emit = (e: any) => { sent.push({ door: L.indexOf(e.door), sound: e.sound, who: e.who, level: e.level, x: e.pos[0], z: e.pos[2] }); if (e.sound in MET) MET[e.sound as 'creak']++; if (!process.env.DOORS_DEAF) { col.rulesWorld(col.dynamicBoxes); MET.asked++; } };   // (…and WHOEVER HEARS ONE ASKS THE MODULE, as a sound's road or a line does in the game: the copy of the leaves is made again from the boxes as they stand, mid-step. The boxes must still end where the step placed them.)   // (the step's own five: a boot's, a pick's and a key's noises are the verbs', made between steps)
  (doors as any).emit = emit;
  const cols = (): Float64Array => { const S = doors.store, d = S.d, out = new Float64Array(DH.DOOR_COLS * n); for (let k = 0; k < DH.DOOR_COLS; k++) for (let i = 0; i < n; i++) out[k * n + i] = d[S.base + k * S.stride + i]; return out; };
  const put = (v: Float64Array) => { const S = doors.store, d = S.d; for (let k = 0; k < DH.DOOR_COLS; k++) for (let i = 0; i < n; i++) d[S.base + k * S.stride + i] = v[k * n + i]; };
  const boxes = () => L.map(d => ({ c: [...d.box.c], yaw: d.box.yaw, flags: d.box.flags, h: d.handleBoxes.map((h: any) => [...h.c, h.yaw]), kept: d.keptAt }));
  const flatC = (cs: any[]) => cs.map(c => [c.pos[0], c.pos[2], L.indexOf(c.blockedBy), c.run ? L.indexOf(c.bashed) : -9, c.run ? L.indexOf(c.ripped) : -9, c.bashed ? c.bashedAt : -9, c.struck ? L.indexOf(c.struck.door) : -1, c.struck ? c.struck.by : 0,
    c.struck ? ['open', 'slam', 'boot'].indexOf(c.struck.tier) : 0, c.struck?.floor ? 1 : 0, c.struck ? c.struck.dir[0] : 0, c.struck ? c.struck.dir[1] : 0, c.moved ? 1 : 0, c.man === undefined ? -7 : c.man]);
  const DT = 1 / 60; let here = 0, levelBad = 0, twins = 0;
  const T = D.DOORTHROW, WAY = D.DOORWAY, STAY = D.DOORSTAY, was = { thr: T.on, soft: T.soft, ow: WAY.oneWay, eo: WAY.exitsOut, lw: WAY.leafWay, stay: STAY.on };
  for (let r = 0; r < N; r++) {
    if (r % 400 === 0) { T.on = rnd() < 0.8; T.soft = rnd() < 0.7; WAY.oneWay = rnd() < 0.8; WAY.exitsOut = rnd() < 0.3; WAY.leafWay = rnd() < 0.3; STAY.on = rnd() < 0.7;
      for (const gd of g.guards) { gd.staggerUntil = rnd() < 0.2 ? g.time + 5 : rnd() < 0.2 ? g.time - 1 : -1; gd.held = rnd() < 0.4 ? (gd.held || {}) : null; } }   // (a man in somebody's hold: Doors.man says MAN_HELD of him, and an opening leaf does not stand him up)
    // the men walk; one whose walk is over is made anew
    for (let i = 0; i < men.length; i++) { const m = men[i]; if (--m.left <= 0) { kept.delete(m); men[i] = man(); continue; } m.pos[0] += m.vel[0] * DT; m.pos[2] += m.vel[2] * DT; }
    // the rules' own writes between steps
    if (rnd() < 0.12) { const d = pick(L), who = pick(ids), p = from(d), k = rnd();
      if (k < 0.2) doors.use(d, p, who, rnd() < 0.3 ? new Set([d.lockId]) : undefined); else if (k < 0.35) doors.kickIn(d, p, who, rnd() < 0.2); else if (k < 0.5) doors.pull(d, who, rnd() < 0.4); else if (k < 0.6) doors.pullTo(d, who);
      else if (k < 0.7) doors.crack(d, p, who, DT, undefined); else if (k < 0.78) doors.pick(d, who, DT * 30); else if (k < 0.84) doors.rattle(d, who);
      else if (k < 0.9) { d.vel = (rnd() < 0.5 ? 1 : -1) * (2 + rnd() * 6); d.latched = false; d.lastWho = who; } else if (k < 0.94) { d.angle = (rnd() * 2 - 1) * 1.8; d.latched = false; }
      else if (k < 0.97) { d.locked = !d.locked; } else { d.reset(); } }
    hush(true); col.tickOpen(); let line = '';
    try {
      g.sim.settle(g);
      const cs = contactsOf(), M = col.rulesWorld(col.dynamicBoxes);
      if (!M || !doors.store.home) { hush(false); here++; doors.update(DT, cs); continue; }
      sent.length = 0;
      if (r % 3 !== 2) { line = doors.updateBoth(DT, cs, M); { const io = DH.doorIo(), m = DH.doorMem(); MET.turned += m[io.par + DH.DP.turned]; } }
      else {   // the twin: the game's own road (the module's step taken back), then all of it put back and this file's step
        twins++;
        const c0 = cols(), b0 = boxes(), p0 = cs.map(c => [c.pos[0], c.pos[1], c.pos[2]]), k0 = cs.map(c => ({ ...c })), tally = { ...doors.solids }, rounds = doors.rounds, turned0 = DH.DOORS_STEPPED.turned;
        doors.update(DT, cs); MET.turned += DH.DOORS_STEPPED.turned - turned0;
        const c1 = cols(), b1 = boxes(), f1 = flatC(cs), e1 = sent.slice(), t1 = { ...doors.solids }, r1 = doors.rounds;
        put(c0); L.forEach((d, i) => { d.box.c = b0[i].c; d.box.yaw = b0[i].yaw; d.box.flags = b0[i].flags; d.handleBoxes.forEach((h: any, j: number) => { h.c = b0[i].h[j].slice(0, 3); h.yaw = b0[i].h[j][3]; }); d.keptPos(b0[i].kept); }); BM.leafWrote();
        cs.forEach((c, i) => { c.pos[0] = p0[i][0]; c.pos[1] = p0[i][1]; c.pos[2] = p0[i][2]; for (const k of ['blockedBy', 'bashed', 'ripped', 'bashedAt', 'struck', 'moved', 'man']) { if (k in k0[i]) c[k] = (k0[i] as any)[k]; else delete c[k]; } });
        Object.assign(doors.solids, tally); doors.rounds = rounds; sent.length = 0;
        doors.updateHere(DT, cs);
        const c2 = cols(), b2 = boxes(), f2 = flatC(cs), names = Object.keys(DH.DC), out: string[] = [];
        for (let k = 0; k < DH.DOOR_COLS && out.length < 3; k++) for (let i = 0; i < n; i++) if (!is(c1[k * n + i], c2[k * n + i])) { out.push(`${L[i].def.name}.${names[k]} ${c2[k * n + i]} here, ${c1[k * n + i]} there`); break; }
        for (let i = 0; i < n && out.length < 3; i++) { const a = b1[i], b = b2[i]; if (!a.c.every((v: number, j: number) => is(v, b.c[j])) || !is(a.yaw, b.yaw) || a.flags !== b.flags || !a.h.every((h: number[], j: number) => h.every((v, q) => is(v, b.h[j][q]))) || !a.kept.every((v: number, j: number) => is(v, b.kept[j]))) out.push(`${L[i].def.name}'s box, handles or kept centre: ${JSON.stringify(b)} here, ${JSON.stringify(a)} there`); }
        for (let i = 0; i < f1.length && out.length < 3; i++) if (!f1[i].every((v, j) => is(v, f2[i][j]))) out.push(`contact ${i} (who ${cs[i].who}): [${f2[i]}] here, [${f1[i]}] there`);
        if (JSON.stringify(e1) !== JSON.stringify(sent)) out.push(`noises ${JSON.stringify(sent).slice(0, 200)} here, ${JSON.stringify(e1).slice(0, 200)} there`);
        if (JSON.stringify(t1) !== JSON.stringify(doors.solids) || r1 !== doors.rounds) out.push('the tallies differ');
        line = out.length ? `the doors' step (twin): ${out.join(' · ')}` : '';
      }
      for (const c of cs) if (c.man !== undefined && (c.man & D.MAN_HELD)) MET.heldAsked++;   // (a man in somebody's hold whom the step ASKED the game about: the throw's own question of a man at a driven leaf — an opening leaf does not stand a held man up, so none is ever `struck` by one)
      for (const c of cs) { if (c.struck) MET.struck++; if (c.moved) MET.moved++; if (c.blockedBy) MET.blocked++; if (c.bashed) MET.bashed++; if (c.ripped) MET.ripped++; }
      for (const d of L) if (d.drive) MET.drives++;
    } finally { col.tickShut(); hush(false); }
    steps++; if (line) { differ++; levelBad++; if (firsts.length < 6) firsts.push(`${file} step ${r}: ${line.slice(0, 700)}`); }
    g.time += DT;
  }
  T.on = was.thr; T.soft = was.soft; WAY.oneWay = was.ow; WAY.exitsOut = was.eo; WAY.leafWay = was.lw; STAY.on = was.stay;
  MET.rounds += doors.rounds; MET.carried += doors.solids.carried; MET.stopped += doors.solids.stopped; MET.held += doors.solids.held; MET.reached += doors.solids.reached;
  if (levelBad || here > N / 10) bad++;
  log(`${file.padEnd(20)} ${levelBad || here > N / 10 ? 'FAIL' : 'PASS'} — ${n} doors, ${N} steps (${twins} of them the twin${here ? `; ${here} the module could not take` : ''}): ${levelBad} differ`);
}
for (const f of firsts) log(`    ${f}`);
const thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0 && !(k === 'heldAsked' && guards === 0));   // (levels that stand up no guards have nobody to hold: nothing is asked of them)
log(`door-ident: ${steps} steps over ${files.length} levels, ${differ} differ · met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length && !BREAK) { log(`    NOT A FAIR RUN: never met ${thin.join(', ')}`); bad++; }
log(`=== door-ident: ${bad ? 'FAIL' : 'PASS'}${BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(bad ? 1 : 0);
