// THE FURNITURE'S STEP, THE TYPESCRIPT AGAINST THE RUST MODULE — every chair, carton, bin and plant of a level stepped by src/game/props.ts PropSystem.update
// as this repo has always stepped it and by rust/sim/src/props.rs (the row RUST.props), from the same state and the same men, thousands of times a level;
// everything the two leave must be the same to the bit: every column of every thing, the rows' two tables and the system's clock (src/sim/propshome.ts PropStore.bytes), where each man
// ends, the noises in their order, and the boxes the picture draws.
//   bun run tools/qa/prop-ident.ts [--n 6000] [--levels office.json,compound.json] [--seed 1]
// No game tick runs here: the tool opens the box world's copy as a tick would, moves the things' columns home (the game's SimState.settle), and steps the
// furniture itself over men it makes up — walkers, runners, men easing past — who walk at the furniture for a while and are then made anew. Between steps
// it does to the furniture what the game's own rules do (a round into a carton, a blast's shove, a thing taken out of the level, the level put back, a chair
// stood before a door frame with a man shoving it through) and works the doors (a boot, a hand, a leaf torn off) so that leaves sweep it; and it turns the
// switches over (castors, the friction, the system frozen) and takes ticks of other lengths (one substep to four). Two ways of asking, turn about:
//   both   PropSystem.updateBoth: the module's step with nothing of it kept, then the TypeScript's from the same columns, compared in place (what
//          collision.ts BOTH does in a played mission: tools/qa/rust-check.ts --both)
//   twin   the step as the game takes it with the row on (the module's, taken back into the game: the men moved, the noises heard, the boxes placed), then
//          everything put back as it was and the TypeScript's own step taken, and what each left compared here
// THE LEVEL'S SETTLE is held too (PropSystem.settle: the twelve passes that resolve a level's furniture out of its authoring overlaps as the level is
// stood up, and make where it comes to stand home — props.rs settle): for each level a second set of its furniture is made over the same statics with the
// both-ways switch up, so the module's passes and the TypeScript's start from the same columns and every column they leave is compared. AND EVERY LEVEL
// FILE'S (levels/*.json, each as it loads and as it loads lived in): its statics and its furniture alone, no game stood up, the same way.
// A RAY INTO THE FURNITURE is held too (PropSystem.raycast, which the module answers — collision.ts thingsRay, lines.rs ray_things — against
// PropSystem.raycastHere, this file's walk): four rays a step from beside a thing, at it or past it, short and long, two asked OUTSIDE the tick straight
// after the rules' own writes (a thing shoved, taken out, put back, stood somewhere else and not yet placed: the copy is proven for the one question and
// must have the furniture as it stands) and two inside it after the step; which thing, which of its boxes, how far along and the face's normal must be
// the same to the bit.
// A run is FAIR only if it met every kind of thing the step does: each noise, a leaf's sweep, a frame's guide, things knocking each other and the statics,
// a man held back, a tick of more than one substep, a thing out of the level, the system frozen, a leaf off its hinges — a settle that moved a thing
// and pushed one out of a static — and rays that met a thing, rays that met none, and rays asked outside a tick. A level that does not stand up is a
// FAIL, not a level left out.
// PROPS_BREAK=1 (or RUST_BREAK=1) is the red path: the module is handed a tick one unit in the last place too long, every thing a hair wider for its
// settle, and every ray from a hair to one side, and the run must FAIL (the steps differ, and the settles and the rays: `settles differ` and `rays
// differ` must not be 0).
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1';   // (the rows and the game's own maths are the build's: no word raises them)
import { resolve } from 'path';
const { standUp } = await import('./headless.ts');   // (not a static import: those load before this file's first line, and the harness reads the stand as it loads)
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`);
const PR: any = await import(`${ROOT}/src/game/props.ts`), PH: any = await import(`${ROOT}/src/sim/propshome.ts`), DB: any = await import(`${ROOT}/src/game/doorblast.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 6000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.PROPS_BREAK === '1' || process.env.RUST_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,compound.json,consulate-gwot.json,mousehole.json,compound.json:bare').split(',');   // (`:bare` = the level with its doors taken out: no leaf in the box world at all)
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== prop-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
if (BREAK) { const P = PR.PropSystem.prototype, hand = P.handOver; P.handOver = function (this: unknown, dt: number, c: unknown, T: unknown) { return hand.call(this, dt * (1 + 2 ** -52), c, T); };   // (the red path: a tick a hair too long for the module alone)
  const there = P.settleThere, facts = P.facts; let settling = false;   // (…and for its settle every thing a hair wider than the level says)
  P.settleThere = function (this: unknown, M: unknown) { settling = true; try { return there.call(this, M); } finally { settling = false; } };
  P.facts = function (this: any) { facts.call(this); if (!settling) return; const io = PH.propIo(), m = PH.propMem(); for (let r = 0; r < this.props.length; r++) m[io.def + PH.PD.hx * PH.PROP_CAP + r] *= 1 + 2 ** -52; };
  const SC = C.StaticCollision.prototype, ray = SC.thingsRay; SC.thingsRay = function (this: unknown, F: unknown, ro: Vec3, rd: Vec3, tmax: number) { return ray.call(this, F, [ro[0] * (1 + 2 ** -52), ro[1], ro[2]], rd, tmax); }; }   // (…and the module's ray from a hair to one side)
const is = Object.is;

interface Man { pos: Vec3; vel: Vec3; left: number; who: number; quiet: boolean }
const MET = { asked: 0, scrape: 0, thud: 0, rollStart: 0, rollStop: 0, swept: 0, guided: 0, pairs: 0, walls: 0, held: 0, substeps: 0, gone: 0, frozen: 0, offLeaf: 0, rounds: 0, parked: 0, bare: 0, settles: 0, settleMoved: 0, settleWalls: 0, rays: 0, rayHits: 0, rayMisses: 0, raysAlone: 0, rayMovedUnder: 0 };
let bad = 0, steps = 0, differ = 0, settlesDiffer = 0, raysDiffer = 0, why: string | undefined; const firsts: string[] = [];
// enough other games to put any game's block out of the module (a listener below binds them: whoever hears a noise may do as much)
const others: any[] = Array.from({ length: ST.simLayout().games + 1 }, () => new ST.SimState());
for (const entry of files) {
  const [file, mode] = entry.split(':'), bare = mode === 'bare';
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${entry.padEnd(22)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was stepped`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col, P = g.props, doors = g.doors, DL: any[] = doors.list, S = P.store, n: number = S.total;
  if (bare) { DL.length = 0; col.dynamicBoxes.length = 0; }
  if (!X.EXACT.on || !SIM.RUST.props || !n) { log(`${entry.padEnd(22)} FAIL — ${!n ? 'the level has no furniture' : 'the stand left the row or the game\'s own maths down'}: nothing was stepped`); bad++; continue; }
  // THE LEVEL'S SETTLE, BOTH WAYS: the level's furniture made a second time over the same statics (a block of its own), the both-ways switch up as it is made
  { const T = PH.PROPS_STEPPED, t0 = { ...T }, was = C.BOTH.on; let threw = ''; C.BOTH.on = true; hush(true);
    try { new PR.PropSystem(g.level.props, col, () => {}); } catch (e) { threw = String((e as Error).message).split('\n')[0].slice(0, 200); } finally { C.BOTH.on = was; hush(false); }
    const asked = T.settled - t0.settled, off = T.differ - t0.differ, here = T.settledHere - t0.settledHere;
    MET.settles += asked; MET.settleMoved += T.settleMoved - t0.settleMoved; MET.settleWalls += T.settleWalls - t0.settleWalls;
    if (off || threw || here || asked !== 1) { settlesDiffer++; if (firsts.length < 6) firsts.push(`${file} settle: ${threw ? 'threw: ' + threw : here ? 'the module could not take it' : asked !== 1 ? `taken both ways ${asked} times, not once` : String(T.first).slice(0, 700)}`); T.first = ''; T.differ = t0.differ; C.BOTH.first = ''; } }
  const ids: number[] = [g.player.char.id, ...g.guards.map((x: any) => x.char.id)];
  // a man made up near a thing, walking at it or past it
  const man = (): Man => {
    const p = pick(P.all as any[]), th = rnd() * 6.283, off = p.radius + 0.25 + rnd() * rnd() * 1.4;
    const pos: Vec3 = [p.x + Math.sin(th) * off, 0, p.z + Math.cos(th) * off];
    const sp = pick([0, 0.4, 1.2, 1.6, 2.6, 4.5, 6]), at = rnd() < 0.75 ? th + Math.PI + (rnd() - 0.5) * 0.9 : rnd() * 6.283;   // (mostly AT the thing)
    return { pos, vel: [Math.sin(at) * sp, 0, Math.cos(at) * sp], left: 8 + Math.floor(rnd() * 70), who: rnd() < 0.2 ? ids[0] : pick(ids), quiet: sp < 1.3 && rnd() < 0.5 };
  };
  const men: Man[] = []; for (let i = 0; i < 12; i++) men.push(man());
  const contactsOf = (): any[] => men.map(m => ({ pos: m.pos, vel: m.vel, radius: 0.3, who: m.who, mass: 75, quiet: m.quiet }));
  const sent: any[] = [];
  (P as any).emit = (e: any) => { sent.push([e.prop.id, e.sound, e.roll ?? '', e.pos[0], e.pos[1], e.pos[2], e.who, e.level, e.speed, e.rolling]);
    if (e.sound === 'roll') MET[e.roll === 'start' ? 'rollStart' : 'rollStop']++; else MET[e.sound as 'scrape']++;
    col.rulesWorld(col.dynamicBoxes); MET.asked++;   // (…and WHOEVER HEARS ONE ASKS THE MODULE, as a sound's road or a line does in the game —
    if (MET.asked % 4 === 0) { const at = g.sim.where; for (const o of others) o.bind(); if (at >= 0 && g.sim.where < 0) MET.parked++; } };   //  and now and then so much of it that this game's block is put out of the module's memory under the step's feet)
  const cols = (): Uint8Array => S.bytes(), put = (v: Uint8Array) => S.put(v);   // (every column, the head and the two tables, as bytes)
  const boxes = (): number[] => { const out: number[] = []; for (const p of P.all) for (const b of p.boxes) out.push(b.c[0], b.c[1], b.c[2], b.yaw); return out; };
  const unbox = (v: number[]) => { let o = 0; for (const p of P.all) for (const b of p.boxes) { b.c[0] = v[o++]; b.c[1] = v[o++]; b.c[2] = v[o++]; b.yaw = v[o++]; } };
  const leavesOf = (): any[] => DL.some(d => d.off) ? DL.filter(d => !d.off).map(d => d.box) : col.dynamicBoxes;   // (as game.ts updateProps hands them)
  // A RAY INTO THE FURNITURE, the game's own road (the module's) against this file's walk: from beside a thing, at it or past it
  const ray = (alone: boolean): void => {
    const b0 = pick(pick(P.all as any[]).boxes as any[]), p = { x: b0.c[0] as number, z: b0.c[2] as number }, th = rnd() * 6.283, far = 0.3 + rnd() * rnd() * 5, ro: Vec3 = [p.x + Math.sin(th) * far, 0.05 + rnd() * 1.7, p.z + Math.cos(th) * far];   // (aimed where a thing's BOXES are: where it stands by its columns may be somewhere else for a moment)
    const to: Vec3 = rnd() < 0.8 ? [p.x + (rnd() - 0.5) * 0.7, rnd() * 1.2, p.z + (rnd() - 0.5) * 0.7] : [ro[0] + rnd() - 0.5, ro[1] + (rnd() - 0.5) * (rnd() < 0.2 ? 4 : 0.4), ro[2] + rnd() - 0.5];
    const dx = to[0] - ro[0], dy = to[1] - ro[1], dz = to[2] - ro[2], l = Math.hypot(dx, dy, dz) || 1, rd: Vec3 = rnd() < 0.03 ? [0, -1, 0] : [dx / l, dy / l, dz / l], tmax = pick([0.4, 1.5, 4, 12, 60, 1e9]);
    const a = P.raycast(ro, rd, tmax), b = P.raycastHere(ro, rd, tmax);
    MET.rays++; if (alone) MET.raysAlone++; if (b) MET.rayHits++; else MET.rayMisses++;
    const same = a === null || b === null ? a === b : a.prop === b.prop && a.box === b.box && is(a.t, b.t) && is(a.n[0], b.n[0]) && is(a.n[1], b.n[1]) && is(a.n[2], b.n[2]);
    if (!same) { raysDiffer++; if (firsts.length < 6) firsts.push(`${file} a ray${alone ? ' outside the tick' : ''} [${ro.join(', ')}] → [${rd.join(', ')}] (${tmax}): ${a ? `thing ${a.prop.id} at ${a.t}` : 'nothing'} from the module, ${b ? `thing ${b.prop.id} at ${b.t}` : 'nothing'} here`); }
  };
  const CAS = PR.CASTOR, was = { cas: CAS.on, fr: P.frictionScale };
  let here = 0, levelBad = 0, twins = 0, frozenLeft = 0;
  for (let r = 0; r < N; r++) {
    if (r % 300 === 0) { CAS.on = rnd() < 0.7; P.frictionScale = pick([1, 1, 0.4, 2.2]); }
    const DT = rnd() < 0.85 ? 1 / 60 : pick([1 / 30, 1 / 20, 1 / 120, 1 / 45]);
    // the men walk; one whose walk is over is made anew
    for (let i = 0; i < men.length; i++) { const m = men[i]; if (--m.left <= 0) { men[i] = man(); continue; } m.pos[0] += m.vel[0] * DT; m.pos[2] += m.vel[2] * DT; }
    // the rules' own writes between steps
    const k = rnd();
    if (k < 0.05) { const p = pick(P.props as any[]); if (p) { const a = rnd() * 6.283, J = 20 + rnd() * 400; P.applyImpulse(p, [p.x + (rnd() - 0.5) * 0.4, 0.5, p.z + (rnd() - 0.5) * 0.4], [Math.cos(a) * J, 0, Math.sin(a) * J], rnd() < 0.8 ? pick(ids) : 0, rnd() < 0.5); MET.rounds++; } }
    else if (k < 0.058 && P.props.length > 4) { P.remove(pick(P.props as any[])); MET.gone++; }
    else if (k < 0.061) P.reset();
    else if (k < 0.066) { P.enabled = false; frozenLeft = 3 + Math.floor(rnd() * 6); }
    else if (k < 0.1 && DL.length) { const d = pick(DL), [hx, hz] = d.def.hinge, a = d.def.closedDir + d.angle, s = rnd() < 0.5 ? 1 : -1, t = rnd() * d.def.width, o = 0.3 + rnd(), from: Vec3 = [hx + Math.cos(a) * t - Math.sin(a) * o * s, 0, hz + Math.sin(a) * t + Math.cos(a) * o * s], q = rnd();
      hush(true); try { if (q < 0.45) doors.kickIn(d, from, pick(ids), false); else if (q < 0.8) doors.use(d, from, pick(ids)); else if (q < 0.9) { d.vel = (rnd() < 0.5 ? 1 : -1) * (2 + rnd() * 6); d.latched = false; } else if (q < 0.96) { const on = DB.DOORBLAST.on; DB.DOORBLAST.on = true; try { DB.tearOff(g, d, from, pick(ids)); } finally { DB.DOORBLAST.on = on; } } else d.reset(); } catch (e) { why ??= String((e as Error).message).split('\n')[0]; } finally { hush(false); } }
    else if (k < 0.14 && P.doorways.length) {   // a chair stood before a door frame, off its middle, its leaf swung clear, and a man behind shoving it through
      const f = Math.floor(rnd() * P.doorways.length), F = P.doorways[f], p = pick((P.props as any[]).filter(x => x.def.kind === 'chair' || x.radius < F.half)), side = rnd() < 0.5 ? 1 : -1, nx = -F.uz * side, nz = F.ux * side;
      if (p) { const a = (rnd() < 0.5 ? 1 : -1) * (F.half - p.radius * (0.5 + rnd() * 0.5)), m = men[Math.floor(rnd() * men.length)], d = DL[f];
        if (d && !d.off) { d.angle = (rnd() < 0.5 ? 1 : -1) * 1.5; d.vel = 0; d.latched = false; }
        p.x = F.cx + nx * (0.5 + rnd() * 0.2) + F.ux * a; p.z = F.cz + nz * (0.5 + rnd() * 0.2) + F.uz * a; p.yaw = rnd() * 6.283; p.vx = p.vz = p.w = 0; p.wake(); p.dirty = true;
        m.pos[0] = p.x + nx * (p.radius + 0.32); m.pos[2] = p.z + nz * (p.radius + 0.32); m.vel[0] = -nx * 1.6; m.vel[2] = -nz * 1.6; m.left = 60; m.quiet = false; } }
    if (!P.enabled) { MET.frozen++; if (--frozenLeft <= 0) P.enabled = true; }
    ray(true); ray(true);   // (outside the tick, straight after the rules' writes above)
    hush(true); col.tickOpen(); let line = '';
    try {
      g.sim.settle(g);
      doors.update(DT, []);   // (the leaves swing as the doors' own step swings them: this file's, the doors' row being down here)
      const cs = contactsOf(), leaves = leavesOf(); if (leaves !== col.dynamicBoxes) MET.offLeaf++;
      sent.length = 0; const T0 = { ...PH.PROPS_STEPPED };
      if (r % 3 !== 2) { C.BOTH.on = true; PH.PROPS_STEPPED.first = ''; try { P.update(DT, cs, leaves); } finally { C.BOTH.on = false; } if (PH.PROPS_STEPPED.differ > T0.differ) line = PH.PROPS_STEPPED.first; }
      else {   // the twin: the game's own road (the module's step taken back), then all of it put back and this file's step
        twins++;
        const c0 = cols(), b0 = boxes(), p0 = cs.map(c => [c.pos[0], c.pos[2]]);
        P.update(DT, cs, leaves);
        const c1 = cols(), b1 = boxes(), f1 = cs.map(c => [c.pos[0], c.pos[2]]), e1 = JSON.stringify(sent);
        put(c0); unbox(b0); cs.forEach((c, i) => { c.pos[0] = p0[i][0]; c.pos[2] = p0[i][1]; }); sent.length = 0;
        P.updateHere(DT, cs, leaves); for (const p of P.props) if (p.dirty) p.place();
        const c2 = cols(), b2 = boxes(), out: string[] = (PH.propDiff(S, c2, c1, 3) as string[]).map(t => `${t} (here | there)`);
        for (let i = 0; i < b1.length && out.length < 3; i++) if (!is(b1[i], b2[i])) out.push(`a box of a thing: ${b2[i]} here, ${b1[i]} there`);
        for (let i = 0; i < f1.length && out.length < 3; i++) if (!is(f1[i][0], cs[i].pos[0]) || !is(f1[i][1], cs[i].pos[2])) out.push(`man ${i} ends at ${cs[i].pos[0]}, ${cs[i].pos[2]} here, ${f1[i]} there`);
        if (e1 !== JSON.stringify(sent)) out.push(`noises ${JSON.stringify(sent).slice(0, 200)} here, ${e1.slice(0, 200)} there`);
        line = out.length ? `the furniture's step (twin): ${out.join(' · ')}` : '';
      }
      if (PH.PROPS_STEPPED.here > T0.here) here++; else if (bare) MET.bare++;
      ray(false);   // (inside the tick, the furniture as the step left it …)
      if (rnd() < 0.3) { const p = pick(P.props as any[]); if (p) { if (rnd() < 0.5) p.x += (rnd() - 0.5) * 1.2; else p.z += (rnd() - 0.5) * 1.2; p.dirty = true; MET.rayMovedUnder++; } }   // (… and with a thing stood somewhere else by a rule and not yet placed: its boxes are where they were, its place is not)
      ray(false);
    } finally { col.tickShut(); hush(false); }
    steps++; if (line) { differ++; levelBad++; if (firsts.length < 6) firsts.push(`${file} step ${r}: ${line.slice(0, 700)}`); }
    g.time += DT;
  }
  CAS.on = was.cas; P.frictionScale = was.fr; P.enabled = true;
  if (levelBad || here > N / 10) bad++;
  log(`${entry.padEnd(22)} ${levelBad || here > N / 10 ? 'FAIL' : 'PASS'} — ${n} things, ${N} steps (${twins} of them the twin${here ? `; ${here} the module could not take` : ''}): ${levelBad} differ`);
}
{ const T = PH.PROPS_STEPPED; MET.swept = T.swept; MET.guided = T.guided; MET.pairs = T.pairs; MET.walls = T.walls; MET.held = T.held; MET.substeps = Math.max(0, T.substeps - T.there - T.asked); }   // (substeps past the first of a tick)
for (const f of firsts) log(`    ${f}`);
const thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0);
// EVERY LEVEL FILE'S SETTLE, BOTH WAYS: the level's statics and its furniture alone (a block of its own), as it loads and as it loads lived in
let every = 0, everyBad = 0, bareLevels = 0;
{ const { readdirSync } = await import('fs'), T = PH.PROPS_STEPPED, t00 = { ...T };
  for (const file of readdirSync(resolve(ROOT, 'levels')).filter((f: string) => f.endsWith('.json') && !f.endsWith('.lint.json')).sort()) for (const lived of [false, true]) {
    const t0 = { ...T }, was = C.BOTH.on; let threw = '', n = 0; C.BOTH.on = true; hush(true);
    try { const level = loadLevel(JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()), { validate: false, lived }); n = level.props.length; if (n) new PR.PropSystem(level.props, new C.StaticCollision(level.boxes), () => {}); }
    catch (e) { threw = String((e as Error).message).split('\n')[0].slice(0, 200); } finally { C.BOTH.on = was; hush(false); }
    if (!n && !threw) { bareLevels++; continue; }
    every++; const asked = T.settled - t0.settled, off = T.differ - t0.differ, here = T.settledHere - t0.settledHere;
    if (off || threw || here || asked !== 1) { everyBad++; if (firsts.length < 6) firsts.push(`${file}${lived ? ' (lived in)' : ''} settle: ${threw ? 'threw: ' + threw : here ? 'the module could not take it' : asked !== 1 ? `taken both ways ${asked} times, not once` : String(T.first).slice(0, 700)}`); T.first = ''; C.BOTH.first = ''; } }
  MET.settles += T.settled - t00.settled; MET.settleMoved += T.settleMoved - t00.settleMoved; MET.settleWalls += T.settleWalls - t00.settleWalls;
  log(`every level's settle  ${everyBad || every < 40 ? 'FAIL' : 'PASS'} — ${every} levels with furniture (as they load, and lived in; ${bareLevels} with none), each settled both ways: ${everyBad} differ`);
  if (everyBad || every < 40) bad++; }
if (settlesDiffer || raysDiffer) bad++;
log(`prop-ident: ${steps} steps over ${files.length} levels, ${differ} differ · ${files.length + every} settles, ${settlesDiffer + everyBad} settles differ · ${MET.rays} rays, ${raysDiffer} rays differ · met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length && !BREAK) { log(`    NOT A FAIR RUN: never met ${thin.join(', ')}${why ? ` (a rule worked between steps threw: ${why.slice(0, 120)})` : ''}`); bad++; }
log(`=== prop-ident: ${bad ? 'FAIL' : 'PASS'}${BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(bad ? 1 : 0);
