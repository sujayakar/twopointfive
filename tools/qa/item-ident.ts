// THE THROWN THINGS' STEP, THE TYPESCRIPT AGAINST THE RUST MODULE — every can, grenade, magazine and dropped gun of a pool stepped by
// src/game/throwables.ts Throwables.update as this repo steps it (`stepRow`) and by rust/sim/src/items.rs (the row RUST.items), from the same state,
// thousands of times a level; everything the two leave must be the same to the bit: every column of every row (src/sim/itemshome.ts ItemStore.bytes),
// what the step listed for after it and so what was said and done (each bounce's callback with its speed, place and face; each share of momentum the
// furniture was handed, by the furniture's own columns; each item whose time was up, and what it was as it left), and the pool's list.
// AND THE FURNITURE'S RAY (src/game/props.ts PropSystem.raycast against rust/sim/src/lines.rs ray_things over the box world's copy of the furniture's
// parts): asked both ways at every step, and by every ray a step of the pool asks.
//   bun run tools/qa/item-ident.ts [--n 4000] [--levels office.json,compound.json:panes] [--seed 1]
// No game tick runs here: the tool opens the box world's copy as a tick would, moves the pool's columns home (the game's SimState.settle), and steps the
// pool itself. Between steps it does what the game's rules do to a pool and round it — things thrown and dropped of every kind, at walls, at furniture,
// onto chairs, at panes of glass, down a hand's length; a thing put in a hand, let go, stood up again where it lay, its time run out, taken out of the
// pool; furniture shoved from under what lies on it, taken out of the level, the level put back; doors stood at other angles — and takes ticks of
// other lengths. Two ways of asking, turn about:
//   both   Throwables.walkBoth (collision.ts BOTH): the module's walk with nothing of it kept, then the TypeScript's over the same rows from the same
//          columns, compared in place (what a played mission does under tools/qa/rust-check.ts --both)
//   twin   the step as the game takes it with the row on (the module's walk, its events said), then everything put back as it was and the step taken
//          with the row off, and what each left compared here
// Levels: `:panes` = three statics in ten made panes of glass with a quarter of their cells fallen out (a ray that ends on a pane is the host's to
// judge: the module hands that row back); `:snow` = a made-up map of lying snow under the pool (a bounce on ground that may bog is the host's too);
// `:many` = the pool driven past the module's thirty-two rows with cans on their fuses (its columns grow and stay in TypeScript: nothing may differ
// there either, and nothing is the module's).
// A run is FAIR only if it met every kind of thing the step does. ITEMS_BREAK=1 (or RUST_BREAK=1) is the red path: the module is handed a tick one
// unit in the last place too long, and the run must FAIL.
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1'; process.env.TPS_EXACT = '1'; process.env.TPS_RUSTITEMS = '1';   // (the game's own maths: the step runs in the module under no other)
process.env.TPS_RUSTLINES = '0'; process.env.TPS_RUSTRAYS = '0'; process.env.TPS_RUSTCIRCLES = '0'; process.env.TPS_RUSTFOG = '0';   // (the TypeScript's arm is all TypeScript, whatever the stand has on: its rays are collision.ts's own walk)
import { resolve } from 'path';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`), BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`);
const BX: any = await import(`${ROOT}/src/scene/boxes.ts`);
const IH: any = await import(`${ROOT}/src/sim/itemshome.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 4000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.ITEMS_BREAK === '1' || process.env.RUST_BREAK === '1';
const files = (flag('--levels') ?? 'office.json,compound.json,consulate-gwot.json,mousehole.json,office.json:panes,compound.json:snow,mousehole.json:many').split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== item-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
if (BREAK) { const io = IH.itemIo(), x = io.x, m = () => IH.itemMem(); io.x = { items_update: (g: number, w: number) => { m()[io.par + IH.IP.dt] *= 1 + 2 ** -52; return x.items_update(g, w); } }; }   // (the red path: a tick a hair too long for the module alone)
const is = Object.is;
const hex = (b: Uint8Array) => { let h = 2166136261; for (let i = 0; i < b.length; i++) { h ^= b[i]; h = Math.imul(h, 16777619); } return (h >>> 0).toString(36) + ':' + b.length; };

const MET = { thrown: 0, there: 0, host: 0, atPane: 0, open: 0, snow: 0, impulses: 0, bounces: 0, gone: 0, onThing: 0, dropped: 0, inHand: 0, stood: 0, givenUp: 0, takenOut: 0, grown: 0, meddled: 0, thingRays: 0, rays: 0, rested: 0, substeps: 0, furnitureRays: 0, furnitureHits: 0, parked: 0, shoved: 0, levelBack: 0 };
let bad = 0, steps = 0, differ = 0; const firsts: string[] = [];
// enough other games to put any game's block out of the module (a callback below binds them: whoever hears a bounce may do as much)
const others: any[] = Array.from({ length: ST.simLayout().games + 1 }, () => new ST.SimState());
const KINDS: string[] = [...IH.ITEM_KINDS];
for (const entry of files) {
  const [file, mode] = entry.split(':');
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${entry.padEnd(22)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was stepped`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col, T = g.items, P = g.props, S = T.store, L: any[] = T.items, DL: any[] = g.doors.list;
  if (!X.EXACT.on || !SIM.RUST.items || !P.all.length) { log(`${entry.padEnd(22)} FAIL — ${!P.all.length ? 'the level has no furniture' : 'the stand left the row or the game\'s own maths down'}: nothing was stepped`); bad++; continue; }
  if (mode === 'panes') {   // (made before the first tick opens, so the module's copy has them as the boxes do)
    let q = 12345; for (const b of col.boxes) { q = (Math.imul(q, 1103515245) + 12345) >>> 0; if ((q >>> 16) % 10 < 3 && !(b.flags & BX.BoxFlag.NoShadow)) b.flags |= BX.BoxFlag.Glass; }
    col.glassWalkable = null; col.glassOpen = (bi: number, p: number[]) => { const o = ((bi * 7919 + Math.floor(p[0] * 3) * 31 + Math.floor(p[1] * 3) * 17 + Math.floor(p[2] * 3)) & 3) === 0; if (o) MET.open++; return o; }; }
  // (lying snow, made up: boggy in a chequer of two-metre squares — the rule's own arithmetic is the game's, the map is this tool's)
  if (mode === 'snow') T.bog = { may: () => true, at: (x: number, _y: number, z: number) => { MET.snow++; return ((Math.floor(x / 2) + Math.floor(z / 2)) & 1) ? 0.25 + 0.5 * (((Math.floor(x * 3) * 31 + Math.floor(z * 3) * 17) & 7) / 7) : 0; } };
  const panes: any[] = col.boxes.filter((b: any) => (b.flags & BX.BoxFlag.Glass) && !(b.flags & BX.BoxFlag.NoShadow) && !b.rot);
  const walls: any[] = col.boxes.filter((b: any) => !(b.flags & BX.BoxFlag.NoShadow) && b.c[1] + b.h[1] > 1 && b.c[1] - b.h[1] < 0.3);
  const ids: number[] = [g.player.char.id, ...g.guards.map((x: any) => x.char.id)];
  const sent: any[] = []; let asked = 0;
  g.audio.play = (name: string, p: number[] | null, gain: number) => { sent.push(['play', name, p ? [...p] : null, gain]); return null; };
  { const give = P.applyImpulse.bind(P); P.applyImpulse = (p: any, at: number[], J: number[], who: number, thud?: boolean) => { MET.impulses++; sent.push(['impulse', p.id, [...at], [...J], who]); return give(p, at, J, who, thud); }; }
  // a bounce's callback, as the game's are: it reads the item, keeps a count on it, says something, and may ask the module anything meanwhile
  const onBounce = (sp: number, p: number[], gg: any, it: any, n: number[]) => {
    sent.push(['bounce', it.serial, it.kind, sp, [...p], [...n], it.clatters ?? -1]); it.clatters = (it.clatters ?? 0) + 1;
    if (sp > 1) gg.events.push({ kind: 'prop', pos: [...p], time: gg.time, loud: false, level: Math.min(0.5, 0.2 + sp * 0.08), who: 0 });
    col.rulesWorld(col.dynamicBoxes);
    // …and now and then it WRITES THE POOL (by what the bounce was, so that the two arms of a twin do the same): another item stood up where it lay, the
    // oldest taken out of the list. Said when the whole pool has been stepped, on either side, it cannot reach into the step — which is what this holds.
    { const q = Math.floor(sp * 1000); if (q % 7 === 0 && L.length) { const o = L[it.serial % L.length]; if (o.settled) { o.settled = false; o.vel = [n[0] * 2, 1.5, n[2] * 2]; MET.meddled++; } } else if (q % 23 === 0 && L.length > 4) { L.shift(); MET.meddled++; } }
    if (++asked % 5 === 0) { const at = g.sim.where; for (const o of others) o.bind(); if (at >= 0 && g.sim.where < 0) MET.parked++; }
  };
  // a place to throw from: beside a thing of furniture, a wall, a pane, or over the middle of a room
  const near = (): Vec3 => { const k = rnd();
    if (k < 0.45) { const p = pick(P.all as any[]), th = rnd() * 6.283, o = rnd() * (p.radius + 1.2); return [p.x + Math.sin(th) * o, 0.2 + rnd() * 1.6, p.z + Math.cos(th) * o]; }
    if (k < 0.6 && panes.length) { const G = pick(panes), thinX = G.h[0] < G.h[2], sg = rnd() < 0.5 ? 1 : -1, o = Math.min(G.h[0], G.h[2]) + 0.3 + rnd() * 1.2; MET.atPane++; return [G.c[0] + (thinX ? sg * o : (rnd() - 0.5) * G.h[0]), Math.max(0.2, G.c[1] + (rnd() - 0.5) * G.h[1]), G.c[2] + (thinX ? (rnd() - 0.5) * G.h[2] : sg * o)]; }
    if (k < 0.8 && walls.length) { const G = pick(walls), th = rnd() * 6.283, o = Math.max(G.h[0], G.h[2]) * rnd() + 0.4; return [G.c[0] + Math.sin(th) * o, 0.2 + rnd() * 1.6, G.c[2] + Math.cos(th) * o]; }
    const c = g.player.char.pos; return [c[0] + (rnd() - 0.5) * 8, 0.3 + rnd() * 1.5, c[2] + (rnd() - 0.5) * 8]; };
  const toss = (live = false) => {
    const kind = live ? pick(['smoke', 'flash', 'frag']) : pick(KINDS), from = near(), k = rnd();
    // thrown hard and flat, lobbed, dropped from a hand, set down on what is under it, or aimed at the nearest thing of furniture
    let v: Vec3 = k < 0.35 ? [(rnd() - 0.5) * 16, rnd() * 5, (rnd() - 0.5) * 16] : k < 0.6 ? [(rnd() - 0.5) * 6, 2 + rnd() * 5, (rnd() - 0.5) * 6] : k < 0.8 ? [(rnd() - 0.5) * 0.6, -0.2, (rnd() - 0.5) * 0.6] : [0, -1 - rnd() * 3, 0];
    if (k >= 0.8) { const p = pick(P.all as any[]); from[0] = p.x + (rnd() - 0.5) * p.radius; from[2] = p.z + (rnd() - 0.5) * p.radius; from[1] = 0.9 + rnd(); }   // (down onto a chair's seat, a carton's top)
    hush(true); let it: any; try { it = T.spawn(kind, from, v, { who: pick(ids), ...(live ? { fuse: 60, life: 60 } : rnd() < 0.2 ? { life: 0.05 + rnd() * 0.4 } : {}), ...(rnd() < 0.3 ? { fill: Math.floor(rnd() * 8) } : {}) }); } finally { hush(false); }
    it.onBounce = rnd() < 0.9 ? onBounce : undefined; MET.thrown++; return it; };
  // everything a step of the pool may write besides its rows: the list, what each item keeps of its own, the furniture's columns, the game's events
  // (taken after the pool's own `sync`, so the list is the rows; put back by this tool's own hand: the list, each item standing in its row again — one
  //  that left meanwhile kept its numbers as it left, which are not the ones it had — and the pool's own note of the list)
  const take = () => { (T as any).sync(); return { cols: S.bytes() as Uint8Array, list: [...L], own: L.map((it: any) => it.clatters), props: P.store.bytes() as Uint8Array, events: g.events.length }; };
  const putBack = (t: ReturnType<typeof take>) => { S.put(t.cols); L.length = 0; t.list.forEach((it: any, k: number) => { it.stand(k); it.clatters = t.own[k]; L.push(it); }); (T as any).rows = [...L]; P.store.put(t.props); g.events.length = t.events; };
  const leftOf = (was: any[]) => was.filter(it => !L.includes(it)).map(it => [it.serial, it.kind, it.pos, it.vel, it.age, it.life, it.settled, it.bounces, it.yaw, it.tumble]);
  const seen = (t: ReturnType<typeof take>) => JSON.stringify([L.map((it: any) => [it.serial, it.row, it.clatters ?? -1]), leftOf(t.list), hex(P.store.bytes()), g.events.slice(t.events), sent], (_k, v) => typeof v === 'number' && (v !== v || is(v, -0)) ? String(is(v, -0) ? '-0' : v) : v);
  let here = 0, levelBad = 0, twins = 0, why = ''; const out: any[] = [];   // (out: items taken out of the list, some to be put back)
  for (let r = 0; r < N; r++) {
    const DT = rnd() < 0.85 ? 1 / 60 : pick([1 / 30, 1 / 20, 1 / 120, 1 / 45]);
    hush(true); col.tickOpen(); let line = '';
    try {
      g.sim.settle(g);
      // the rules' own writes between steps
      const k = rnd(), flying = L.filter(it => !it.settled).length;
      if (mode === 'many' && r === Math.floor(N / 3)) { const cap0 = S.cap; for (let i = 0; i < 40; i++) toss(true); if (S.cap > cap0) MET.grown++; }
      if (flying < 3 && k < 0.5 || k < 0.08) { toss(); if (rnd() < 0.4) toss(); if (L.length >= IH.ITEM_CAP) MET.givenUp++; }
      else if (k < 0.11 && L.length) { const it = pick(L); it.carried = true; it.pos = near(); it.vel = [0, 0, 0]; MET.inHand++; }   // (taken in a hand: placed by whoever carries it)
      else if (k < 0.14) { for (const it of L) if (it.carried) { it.carried = rnd() < 0.5 ? false : undefined; it.settled = false; it.vel = [(rnd() - 0.5) * 3, -rnd() * 2, (rnd() - 0.5) * 3]; } }   // (…and let go)
      else if (k < 0.17 && L.length) { const it = pick(L); if (it.settled) { it.pos = near(); it.vel = [(rnd() - 0.5) * 10, rnd() * 4, (rnd() - 0.5) * 10]; it.heading = Math.atan2(it.vel[0], it.vel[2]); it.bounces = 0; it.settled = false; MET.stood++; } }   // (a can passed on from where it lay: doortools.ts — what it lay on is not forgotten)
      else if (k < 0.19 && L.length) {   // the list changed from outside the pool, as the game's rules change it: an item spliced out, the oldest shifted off, two put the other way round, all of it emptied — and, once out, one put back in
        const w = rnd(), i = Math.floor(rnd() * L.length);
        if (w < 0.5) out.push(L.splice(i, 1)[0]); else if (w < 0.65) out.push(L.shift()); else if (w < 0.8 && L.length > 1) { const j = (i + 1) % L.length, t = L[i]; L[i] = L[j]; L[j] = t; } else if (w < 0.85) { out.push(...L); L.length = 0; } else if (out.length) L.splice(Math.min(i, L.length), 0, out.pop());
        MET.takenOut++; }
      else if (k < 0.27) { const on = L.filter(it => it.settled && it.on), p: any = on.length && rnd() < 0.7 ? pick(on).on : pick(P.props as any[]);   // a thing of furniture shoved (from under what lies on it, more often than not)
        if (p) { const far = rnd() < 0.5; p.x += (rnd() - 0.5) * (far ? 1.6 : 0.05); p.z += (rnd() - 0.5) * (far ? 1.6 : 0.05); if (rnd() < 0.5) p.yaw += (rnd() - 0.5) * 1.2; p.place(); MET.shoved++; } }
      else if (k < 0.275 && P.props.length > 2) { const p = pick(P.props as any[]); P.remove(p); if (rnd() < 0.6) for (const it of L) if (it.on === p) { it.on = null; it.settled = false; it.vel = [0, 0, 0]; } }   // (a thing a blast took: furnblast.ts — and, now and then, with what lay on it left believing it there)
      else if (k < 0.28 && P.props.length !== P.all.length) { P.reset(); MET.levelBack++; }
      else if (k < 0.31) { const hung = DL.filter(d => !d.off); if (hung.length) { const d = pick(hung); d.angle = (rnd() - 0.5) * 3; d.vel = 0; d.latched = false; d.place(); BM.leafWrote(); } }
      // THE FURNITURE'S RAY, both ways: a handful of rays from round a thing, some straight down onto it
      { const p = pick(P.all as any[]), B0 = { ...C.BOTH }; C.BOTH.on = true; C.BOTH.first = '';
        try { for (let i = 0; i < 6; i++) { const th = rnd() * 6.283, o = rnd() * (p.radius + 1.5), ro: Vec3 = [p.x + Math.sin(th) * o, rnd() * 1.8, p.z + Math.cos(th) * o];
          const down = rnd() < 0.25, a = rnd() * 6.283, e = (rnd() - 0.5) * 1.2, rd: Vec3 = down ? [0, -1, 0] : [Math.sin(a) * Math.cos(e), Math.sin(e), Math.cos(a) * Math.cos(e)];
          if (P.raycast(ro, rd, 0.1 + rnd() * rnd() * 6)) MET.furnitureHits++; } }
        finally { C.BOTH.on = false; }
        MET.furnitureRays += C.BOTH.things - B0.things;
        if (C.BOTH.differ > B0.differ || C.BOTH.stale > B0.stale) line = C.BOTH.first; }
      sent.length = 0; const T0 = { ...IH.ITEMS_STEPPED }, onBefore = L.filter(it => it.settled && it.on).length, n0 = L.length;
      if (line) { /* the ray already differed: the step is not asked on top of it */ }
      else if (r % 3 !== 2) { C.BOTH.on = true; IH.ITEMS_STEPPED.first = ''; const d0 = C.BOTH.differ, s0 = C.BOTH.stale; C.BOTH.first = ''; try { T.update(DT, g); } finally { C.BOTH.on = false; }
        if (IH.ITEMS_STEPPED.differ > T0.differ) line = IH.ITEMS_STEPPED.first; else if (C.BOTH.differ > d0 || C.BOTH.stale > s0) line = C.BOTH.first; }
      else {   // the twin: the game's own road with the row on, then all of it put back and the same with the row off
        twins++;
        const t0 = take();
        T.update(DT, g);
        const c1 = S.bytes(), s1 = seen(t0);
        putBack(t0); sent.length = 0; SIM.RUST.items = false;
        try { T.update(DT, g); } finally { SIM.RUST.items = true; }
        const out: string[] = (IH.itemDiff(S, S.bytes(), c1, 3) as string[]).map(t => `${t} (here | there)`), s2 = seen(t0);
        if (s1 !== s2) { let i = 0; while (i < s1.length && s1[i] === s2[i]) i++; out.push(`the list, what left, the furniture, the events or what was said: …${s2.slice(Math.max(0, i - 60), i + 140)} here, …${s1.slice(Math.max(0, i - 60), i + 140)} there`); }
        line = out.length ? `the pool's step (twin): ${out.join(' · ')}` : '';
      }
      const T1 = IH.ITEMS_STEPPED;
      if (T1.there === T0.there && n0 > 0 && r % 3 !== 2 && S.cap === IH.ITEM_CAP) here++;
      MET.bounces += sent.filter(e => e[0] === 'bounce').length; MET.gone += Math.max(0, n0 - L.length);
      const onNow = L.filter(it => it.settled && it.on).length; if (onNow > onBefore) MET.onThing += onNow - onBefore; else if (onNow < onBefore) MET.dropped += onBefore - onNow;
    } catch (e) { why ||= String((e as Error).stack ?? e).slice(0, 600); line ||= `threw: ${String((e as Error).message).slice(0, 200)}`; }
    finally { col.tickShut(); hush(false); }
    steps++; if (line) { differ++; levelBad++; if (firsts.length < 6) firsts.push(`${entry} step ${r}: ${line.slice(0, 900)}`); }
    g.time += DT;
  }
  if (levelBad || here > N / 10) bad++;
  log(`${entry.padEnd(22)} ${levelBad || here > N / 10 ? 'FAIL' : 'PASS'} — ${P.all.length} things of furniture, ${N} steps (${twins} of them the twin${here ? `; ${here} with a pool the module could not walk` : ''}): ${levelBad} differ${why ? ` · a rule worked between steps threw: ${why.slice(0, 300)}` : ''}`);
}
{ const T = IH.ITEMS_STEPPED; MET.there = T.there; MET.host = T.host; MET.rays = T.rays; MET.thingRays = T.thingRays; MET.rested = T.rested; MET.substeps = T.substeps; }
for (const f of firsts) log(`    ${f}`);
const thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0);
log(`item-ident: ${steps} steps over ${files.length} levels, ${differ} differ · met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length && !BREAK) { log(`    NOT A FAIR RUN: never met ${thin.join(', ')}`); bad++; }
log(`=== item-ident: ${bad ? 'FAIL' : 'PASS'}${BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(bad ? 1 : 0);
