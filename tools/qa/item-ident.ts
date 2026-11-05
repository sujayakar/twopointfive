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
// Levels: `:panes` = three statics in ten made panes of glass with a quarter of them gone (a ray that meets a pane where it is open flies on, in the
// module as here); `:snow` = a level of lying snow with the game's own rule over it (snowBog.ts, forced on whatever the stand: the module asks the same ground of its copy);
// `:many` = the pool driven past the rule's thirty-two with cans on their fuses (it grows, a row a can, and is stepped in the module all the same:
// nothing may differ there either).
// A run is FAIR only if it met every kind of thing the step does. ITEMS_BREAK=1 (or RUST_BREAK=1) is the red path: the module is handed a tick one
// unit in the last place too long, and the run must FAIL.
process.env.TPS_PROFILE ??= 'shipped'; process.env.TPS_CPUSENSE ??= '1';   // (the rows and the game's own maths are the build's: no word raises them)
import { resolve } from 'path';
const { standUp, hold } = await import('./headless.ts');   // (not a static import: those load before this file's first line, and the harness reads the stand as it loads)
hold({ rows: { lines: false, rays: false, circles: false, fog: false, props: false } });   // (this check's own arm: the TypeScript's step is all TypeScript, its rays collision.ts's own walk, so the box world's rows and the furniture's are held down for both arms)
const ROOT = resolve(import.meta.dir, '../..');
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), X: any = await import(`${ROOT}/src/math/exact.ts`), C: any = await import(`${ROOT}/src/scene/collision.ts`), BM: any = await import(`${ROOT}/src/scene/boxMirror.ts`);
const BX: any = await import(`${ROOT}/src/scene/boxes.ts`);
const IH: any = await import(`${ROOT}/src/sim/itemshome.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`);
const SB: any = await import(`${ROOT}/src/game/snowBog.ts`), WK: any = await import(`${ROOT}/src/game/weatherKinds.ts`);
type Vec3 = [number, number, number];

const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 4000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.ITEMS_BREAK === '1' || process.env.RUST_BREAK === '1', SNOW_BREAK = process.env.ITEMS_BREAK === 'snow';
const files = (flag('--levels') ?? 'office.json,compound.json,consulate-gwot.json,mousehole.json,office.json:panes,tut-road.json:snow,tut-quiet.json:snow,mousehole.json:many').split(',');
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const pick = <T,>(a: readonly T[]): T => a[Math.floor(rnd() * a.length)];
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== item-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
if (BREAK) { const io = IH.itemIo(), x = io.x, m = () => IH.itemMem(); io.x = { items_update: (g: number, w: number) => { m()[io.par + IH.IP.dt] *= 1 + 2 ** -52; return x.items_update(g, w); } }; }   // (the red path: a tick a hair too long for the module alone)
const is = Object.is;
const hex = (b: Uint8Array) => { let h = 2166136261; for (let i = 0; i < b.length; i++) { h ^= b[i]; h = Math.imul(h, 16777619); } return (h >>> 0).toString(36) + ':' + b.length; };

const MET = { thrown: 0, there: 0, atPane: 0, open: 0, snow: 0, snowDeep: 0, snowPart: 0, snowNone: 0, snowUp: 0, snowBank: 0, snowRoof: 0, roofs: 0, impulses: 0, bounces: 0, gone: 0, onThing: 0, dropped: 0, inHand: 0, stood: 0, givenUp: 0, takenOut: 0, grown: 0, meddled: 0, thingRays: 0, rays: 0, rested: 0, substeps: 0, furnitureRays: 0, furnitureHits: 0, parked: 0, shoved: 0, levelBack: 0 };
let bad = 0, steps = 0, differ = 0; const firsts: string[] = [];
// enough other games to put any game's block out of the module (a callback below binds them: whoever hears a bounce may do as much)
const others: any[] = Array.from({ length: ST.simLayout().games + 1 }, () => new ST.SimState());
const KINDS: string[] = [...IH.ITEM_KINDS];
const STAND_SNOW = [WK.SNOW.on, SB.SNOWBOG.on];
for (const entry of files) {
  const [file, mode] = entry.split(':');
  WK.SNOW.on = STAND_SNOW[0]; SB.SNOWBOG.on = STAND_SNOW[1];   // (the stand's own, whatever the entry before this one forced)
  if (mode === 'snow') { WK.SNOW.on = true; SB.SNOWBOG.on = true; }   // (a snow level is born in its snow: before the game is made)
  let W: any; hush(true);
  try { const data = JSON.parse(await Bun.file(resolve(ROOT, 'levels', file)).text()); W = await standUp({ uploadEvery: 1000, ...(file === 'office.json' ? {} : { level: loadLevel(data) }) }); }
  catch (e) { hush(false); log(`${entry.padEnd(22)} FAIL — not stood up (${String((e as Error).message).split('\n')[0].slice(0, 90)}): nothing was stepped`); bad++; continue; }
  hush(false);
  const g = W.game, col = g.col, T = g.items, P = g.props, S = T.store, L: any[] = T.items, DL: any[] = g.doors.list;
  if (!X.EXACT.on || !SIM.RUST.items || !P.all.length) { log(`${entry.padEnd(22)} FAIL — ${!P.all.length ? 'the level has no furniture' : 'the stand left the row or the game\'s own maths down'}: nothing was stepped`); bad++; continue; }
  if (mode === 'panes') {   // (made before the first tick opens, so the module's copy has them as the boxes do)
    let q = 12345; for (const b of col.boxes) { q = (Math.imul(q, 1103515245) + 12345) >>> 0; if ((q >>> 16) % 10 < 3 && !(b.flags & BX.BoxFlag.NoShadow)) b.flags |= BX.BoxFlag.Glass; }
    // (a quarter of the made panes GONE — a made-up state of the glass, said twice as the game's own is: the rule a walk of this side asks, and the rows the
    //  module's copy holds, boxMirror.ts PaneSource. Where a pane is open piece by piece is tools/qa/pane-ident.ts's to hold.)
    const goneOf = (bi: number) => (((bi * 7919) >>> 2) & 3) === 0, rows: any[] = [];
    col.boxes.forEach((b: any, bi: number) => { if ((b.flags & BX.BoxFlag.Glass) && goneOf(bi)) rows.push({ bi, gone: true, run: false, sites: [], open: new Uint8Array(0), pending: [], dice: null }); });
    col.glassWalkable = null; const said = { each: (to: (r: any) => void) => { for (const r of rows) to(r); }, view: null }; col.glassRule((bi: number) => { const o = goneOf(bi); if (o) MET.open++; return o; }, () => said); }
  // (LYING SNOW, THE GAME'S OWN: the level's lays and banks and its roofs, by snowBog.ts's rule on this side and by the same ground in the module's copy
  //  on the other. What this side's rule answered is only counted here.)
  let snowy: any[] = [];
  if (mode === 'snow') {
    WK.SNOW.on = true; SB.SNOWBOG.on = true; const real = T.bog, K = real.kinds() as Uint8Array;
    if (!real.may() || !K.some((k: number) => k !== 0)) { log(`${entry.padEnd(22)} FAIL — the level has no lying snow on this stand: nothing was stepped`); bad++; continue; }
    snowy = col.boxes.filter((_b: any, i: number) => K[i] !== 0);
    // (FOUR ROOFS in force, where the level has one: three made up over lays of its snow — the weather's own list, so this side's rule and the walk's
    //  hand-over read the same four)
    { const lays = col.boxes.filter((_b: any, i: number) => K[i] !== 0 && K[i] !== SB.SNOW_KIND.bank), W0 = g.weather; W0.roofsNow();
      const made = [0.2, 0.5, 0.8].map(f => lays[Math.floor(f * lays.length)]).filter(Boolean).map((b: any) => { const r = Math.max(b.h[0], b.h[2]) * 0.5 + 0.5; return [b.c[0] - r, b.c[2] - r, b.c[0] + r, b.c[2] + r]; });
      W0.roofs = [...W0.roofs.slice(0, 1), ...made].slice(0, IH.ITEM_ROOFS); MET.roofs = Math.max(MET.roofs, W0.roofs.length); }
    // (ITEMS_BREAK=snow, the snow's own red path: the module is told another snow than the level's — trodden for bare, nothing for a bank — and must differ)
    const told = SNOW_BREAK ? K.map((k: number) => k === SB.SNOW_KIND.bare ? SB.SNOW_KIND.packed : k === SB.SNOW_KIND.packed ? SB.SNOW_KIND.bare : k === SB.SNOW_KIND.bank ? SB.SNOW_KIND.none : k) : null;
    T.bog = { may: () => real.may(), kinds: () => told ?? real.kinds(), roofs: () => real.roofs(), at: (x: number, y: number, z: number) => { const v = real.at(x, y, z); MET.snow++;
      if (!(y < SB.SNOWBOG.top)) { MET.snowUp++; if (v > 0) MET.snowBank++; } else if (g.weather.underRoof(x, z)) MET.snowRoof++; else if (v >= 1) MET.snowDeep++; else if (v > 0) MET.snowPart++; else MET.snowNone++; return v; } };
  }
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
    if (snowy.length && rnd() < 0.6) { const G = pick(snowy), th = rnd() * 6.283, o = Math.max(G.h[0], G.h[2]) * rnd() * 1.3; return [G.c[0] + Math.sin(th) * o, G.c[1] + G.h[1] + 0.2 + rnd() * 1.6, G.c[2] + Math.cos(th) * o]; }   // (over a lay or a bank of snow, and a little off it)
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
      // (forty live cans at once: the rule gives none of them up, so the pool GROWS past its thirty-two — and is stepped in the module all the same)
      if (mode === 'many' && r === Math.floor(N / 3)) { for (let i = 0; i < 40; i++) toss(true); if (L.length > IH.ITEM_CAP && L.length <= IH.ITEM_ROWS) MET.grown++; }
      if (flying < 3 && k < 0.5 || k < 0.08) { toss(); if (rnd() < 0.4) toss(); if (L.length >= IH.ITEM_CAP) MET.givenUp++; }
      else if (k < 0.11 && L.length) { const it = pick(L); it.carried = true; it.pos = near(); it.vel = [0, 0, 0]; MET.inHand++; }   // (taken in a hand: placed by whoever carries it)
      else if (k < 0.14) { for (const it of L) if (it.carried) { it.carried = rnd() < 0.5 ? false : undefined; it.settled = false; it.vel = [(rnd() - 0.5) * 3, -rnd() * 2, (rnd() - 0.5) * 3]; } }   // (…and let go)
      else if (k < 0.17 && L.length) { const it = pick(L); if (it.settled) { it.pos = near(); it.vel = [(rnd() - 0.5) * 10, rnd() * 4, (rnd() - 0.5) * 10]; it.heading = Math.atan2(it.vel[0], it.vel[2]); it.bounces = 0; it.settled = false; MET.stood++; } }   // (a can passed on from where it lay: doortools.ts — what it lay on is not forgotten)
      else if (k < 0.19 && L.length) {   // the list changed from outside the pool, as the game's rules change it: an item spliced out, the oldest shifted off, two put the other way round, all of it emptied — and, once out, one put back in
        const w = rnd(), i = Math.floor(rnd() * L.length);
        if (w < 0.5) out.push(L.splice(i, 1)[0]); else if (w < 0.65) out.push(L.shift()); else if (w < 0.8 && L.length > 1) { const j = (i + 1) % L.length, t = L[i]; L[i] = L[j]; L[j] = t; } else if (w < 0.85) { out.push(...L); L.length = 0; } else if (out.length && L.length < IH.ITEM_ROWS) L.splice(Math.min(i, L.length), 0, out.pop());   // (put back only where the pool has a row for it: a list longer than its columns is a fault, sim/itemshome.ts)
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
      if (T1.there === T0.there && n0 > 0 && r % 3 !== 2) here++;
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
const handedBack = IH.ITEMS_STEPPED.host;   // (rows the module handed back for this side to step: nothing of play is such a row any more — a pane, lying snow and a ray of no length are all asked there)
if (handedBack > 0 && !BREAK && !SNOW_BREAK) { log(`    THE MODULE HANDED ${handedBack} ROW(S) BACK: every row of these levels is its own to step`); bad++; }
{ const T = IH.ITEMS_STEPPED; MET.there = T.there; MET.rays = T.rays; MET.thingRays = T.thingRays; MET.rested = T.rested; MET.substeps = T.substeps; }
for (const f of firsts) log(`    ${f}`);
const thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0);
log(`item-ident: ${steps} steps over ${files.length} levels, ${differ} differ · met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}`);
if (thin.length && !BREAK && !SNOW_BREAK) { log(`    NOT A FAIR RUN: never met ${thin.join(', ')}`); bad++; }
log(`=== item-ident: ${bad ? 'FAIL' : 'PASS'}${BREAK || SNOW_BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(bad ? 1 : 0);
