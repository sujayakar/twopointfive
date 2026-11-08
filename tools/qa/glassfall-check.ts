// BROKEN GLASS: LITTER AND HEARING BY THE FALL (src/game/shards.ts SHARDS.byFall; docs/agents/destruction.md) — is what broken glass does to the GAME
// (the glass it leaves on the floor, which makes a footfall louder, and the landings the floor hears) decided by the simulation alone, and by nothing a
// machine draws?
//   bun run tools/qa/glassfall-check.ts                       (also `bun run tools/qa/probes.ts glassfall`, and under `glass` and `all`). The rule is settled:
//                                                             nothing here writes it or asks the game without it (until 2026-11-07: `sees`, the tallies, and the break word)
//   GLASSFALL_BREAK=1 bun run tools/qa/glassfall-check.ts     the red path: `pool` is laid against ANOTHER seed's volley (a
//                                                             game that differs) — `pool` must then FAIL
// THE SCENE is the shards probes' seeded volley: thirty rounds through the office's conference glazing over five seconds, one toughened sheet struck so
// that it pours as dice, a bang beside it, and then a man walking the far side over whatever came down (his footfalls' levels are the game's answer).
// WHAT IS HELD:
//   lays     glass is laid where fragments fall, the floor is told, and a footfall on it is louder than one off it
//   pool     THE RULE: the volley played with the debris pool at its two tiers' sizes (64, 32) and at 4, with the particle budget cut to an eighth, and
//            with the pool refusing every plate, is ONE GAME: the same hearing events at the same ticks, the same footfall levels, the same litter in the
//            same cells, the same landings waiting, the same digest and the same raw bits of everything but the debris pool after every tick
//   module   the volley with RUST.falls on is the volley with it off after every tick, and the rules were the module's
//   snapshot with the row on, a snapshot taken mid-volley (landings waiting, glass on the floor) and put back plays the next 260 ticks to the
//            same state's word after each (SIMSTATE_BREAK=1 — a put back that writes no block — must fail it)
// Not game code; nothing here needs a GPU, and nothing here has been seen on one.
import { standUp, ROOT, fake } from './headless.ts';
import { CONF_FAR, floatPartition } from './office-pane.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
const GBM: any = await import(`${ROOT}/src/game/glassbreak.ts`), SM: any = await import(`${ROOT}/src/game/shards.ts`);
const DG: any = await import(`${ROOT}/src/game/digest.ts`), FL: any = await import(`${ROOT}/src/game/floor.ts`);
const S: any = await import(`${ROOT}/src/sim/sim.ts`), FH: any = await import(`${ROOT}/src/sim/fallshome.ts`);
const { SHARDS, SHARD_STATS } = SM, { GB_STATS } = GBM;
type Vec3 = [number, number, number];
const DT = 1 / 60, BREAK = process.env.GLASSFALL_BREAK === '1';
if (process.env.SIMSTATE_BREAK === '1') { const P = (await import(`${ROOT}/src/sim/state.ts`) as any).SimState.prototype, put = P.put; P.put = function (this: unknown, c: unknown[]) { if (c.length === 0) put.call(this, c); }; }   // (tools/qa/simstate-check.ts's red path: a take put back writes nothing)
const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
const VOLLEY: Vec3[] = [[7.4, 1.3, 10], [7.42, 1.31, 10], [6.3, 1.0, 10], [7.9, 0.7, 10], [8.8, 1.5, 10], [6.6, 1.6, 10], [7.1, 0.5, 10], [8.9, 1.52, 10], [9.3, 0.9, 10], [7.6, 1.9, 10]];
const SAM: Vec3 = [8.3, 0, 11.6];
interface Opts { liveCap?: number; refuse?: boolean; row?: boolean }
interface Rec { bare: number[]; partsAt: Record<string, number>[]; events: string[]; treads: string[]; litter: string[]; waiting: number; digest: string; bits: number[]; stats: any; plates: number; refused: number; there: number; m2: number; cells: number; treadSum: number }
/** the volley's scene stood up, and frame `f` of it (the rounds, the struck sheet, the bang, the walker) as a function a check can call again */
async function stage(seed: number, o: Opts) {
  Math.random = seeded(seed);
  const W: any = await standUp(), game = W.game, engine = W.engine; floatPartition(game);
  S.RUST.falls = !!o.row; if (o.liveCap) SHARDS.liveCap = o.liveCap;   // (a stand-up puts every switch where its stand has it: set after)
  engine.particles = new Particles(fake('device'), fake('sceneLayout')); engine.settings.gpuParticles = true;
  if (o.refuse) game.destr.debris.spawnRaw = () => null;
  game.godMode = true; game.player.visibility = 0.02; game.tune.chaos = 0.5; game.tune.glassBreak = true; game.tune.glassShards = true;
  for (const k of Object.keys(GB_STATS)) GB_STATS[k] = 0; for (const k of Object.keys(SHARD_STATS)) SHARD_STATS[k] = 0;
  for (let i = 0; i < 5; i++) W.step(DT);
  game.guards.forEach((gd: any, i: number) => { gd.hold = true; gd.pinned = true; gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); gd.heardUpTo = 1e12; gd.dazzledUntil = 1e9; gd.flashlight.enabled = false; });
  game.teleportPlayer(SAM); game.player.char.bodyYaw = game.player.char.aimYaw = Math.PI; game.player.slot = 'rifle'; game.player.holstered = false; game.player.crouch = false;
  W.step(DT); W.step(DT);
  game.glassBreak.setType(CONF_FAR, 'tempered');
  const jit = seeded(seed * 7919 + 3), aim = VOLLEY.map((p): Vec3 => seed === 1 ? p : [p[0] + (jit() - 0.5) * 0.5, p[1] + (jit() - 0.5) * 0.3, p[2]]);   // (seed 1 is the shards probes' own volley; another seed shoots about it)
  const spot = (k: number): Vec3 => [7.5 + (k % 5) * 0.4 - 0.8, 0, 9.6], bare = [0, 1, 2, 3, 4].map(k => FL.tread(game, spot(k), 0.3, true) as number);   // (the walker's five spots before any glass lies there: what a footfall is on the bare floor)
  const treads: string[] = []; const T = { sum: 0 };
  const frame = (f: number) => {
    if (f < 300 && f % 10 === 0) fireWeapon(game, game.player.char, aim[(f / 10) % aim.length], true, { weapon: WEAPONS.fn2000 });
    if (f === 150) { const c = game.level.boxes[CONF_FAR].c as Vec3; game.glassBreak.hit(CONF_FAR, [c[0], c[1], c[2]], [0, 0, -1], 1); }
    if (f === 330) game.events.push({ kind: 'bang', pos: [10.2, 0.4, 10.9], time: game.time, loud: true, who: 0 });
    if (f >= 400 && f % 20 === 0) { const l = FL.tread(game, spot(f / 20), 0.3, true); treads.push(`${f}:${l}`); T.sum += l; }
    W.step(DT);
  };
  return { W, game, frame, treads, T, bare };
}
/** the volley, played: what the GAME keeps of it (and a little of what is drawn, for the tallies) */
async function volley(seed: number, o: Opts): Promise<Rec> {
  const was = { liveCap: SHARDS.liveCap, row: S.RUST.falls }, there0 = FH.FALLS.there;
  hush(true);
  try {
    const { game, frame, treads, T, bare } = await stage(seed, o);
    const partsAt: Record<string, number>[] = [], events: string[] = [], bits: number[] = [], seen = new WeakSet<object>();
    for (let f = 0; f < 600; f++) {
      frame(f);
      for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'prop') events.push(`${f}:${e.level}:${e.pos.join(',')}:${e.who}`); }
      const P = DG.rawBitsParts(game); let h = 0x811c9dc5; for (const k of Object.keys(P)) if (k !== 'debris') h = Math.imul(h ^ P[k], 0x01000193) >>> 0; bits.push(h); partsAt.push(P);
    }
    const cells: [number, number, number][] = game.falls.cells();
    return { bare, partsAt, events, treads, litter: cells.map(c => c.join(':')), waiting: game.falls.waiting, digest: DG.hashDigest(DG.digestGame(game)), bits, stats: { ...SHARD_STATS }, plates: SHARD_STATS.plates, refused: SHARD_STATS.platesRefused,
      there: FH.FALLS.there - there0, m2: cells.reduce((a, c) => a + c[2], 0), cells: cells.length, treadSum: T.sum };
  } finally { hush(false); SHARDS.liveCap = was.liveCap; S.RUST.falls = was.row; }
}
/** where two plays of the volley part as GAMES ('' = they do not) */
function parts(a: Rec, b: Rec): string {
  const list = (k: 'events' | 'treads' | 'litter') => { const x = a[k], y = b[k]; if (x.length === y.length && x.every((v, i) => v === y[i])) return ''; let i = 0; while (i < x.length && i < y.length && x[i] === y[i]) i++; return `${k} (${x.length} | ${y.length}) from #${i}: ${x[i] ?? '∅'} | ${y[i] ?? '∅'}`; };
  const tick = a.bits.findIndex((w, i) => w !== b.bits[i]);
  return [list('events'), list('treads'), list('litter'), a.waiting !== b.waiting ? `landings waiting ${a.waiting} | ${b.waiting}` : '', a.digest !== b.digest ? `digest ${a.digest} | ${b.digest}` : '', tick >= 0 ? `raw bits (the debris pool left out) from tick ${tick}: ${Object.keys(a.partsAt[tick]).filter(k => k !== 'debris' && a.partsAt[tick][k] !== b.partsAt[tick][k]).join(', ')}` : ''].filter(Boolean).join(' · ');
}
/** every line above (tools/qa/probes.ts `glassfall`): true = all pass */
export async function glassfallCheck(): Promise<boolean> {
  fails = 0;
  console.log(`broken glass by the fall${BREAK ? ' · GLASSFALL_BREAK=1 (the red path)' : ''}`);
  const POOLS: [string, Opts][] = [['an eighth of the particle budget', { liveCap: 500 }], ['every plate refused', { refuse: true }]];

  // ---- lays
  const on = await volley(1, {}), other = BREAK ? await volley(2, {}) : on;   // (the break word: the pools are laid against another seed's volley)
  { const lv = on.treads.map(t => Number(t.split(':')[1])), loud = lv.filter((l, i) => l > on.bare[i % 5]).length;
    verdict(on.stats.fragments > 100 && on.cells > 5 && on.events.length > 3 && loud > 0 && on.plates > 10, 'lays: glass is laid where fragments fall, the floor is told, and a footfall on it is louder than one off it',
      `${on.stats.fragments} fragments (${on.plates} plates in the pool, ${on.refused} refused) · glass in ${on.cells} cells, ${(on.m2 * 1e4).toFixed(0)} cm² · the floor told ${on.events.length} times · ${loud} of ${lv.length} footfalls on the glazing's far side louder than the same footfall on the bare floor (${on.bare.map(b => b.toFixed(3)).join(' ')} bare; the loudest on glass ${Math.max(...lv).toFixed(3)})`); }

  // ---- pool
  { const lines: string[] = []; let same = 0;
    for (const [name, o] of POOLS) { const r = await volley(1, { ...o }), d = parts(other, r); if (!d) same++; lines.push(`${name} (${r.plates} plates, ${r.refused} refused): ${d ? 'ANOTHER GAME — ' + d.slice(0, 260) : 'the same game'}`); }
    verdict(same === POOLS.length, `pool: the volley is one game whatever the debris pool holds and the particle budget lets through${BREAK ? ' (LAID AGAINST ANOTHER SEED: the red path)' : ''}`, `against the pool at 64 (${other.plates} plates) · ${lines.join('\n    ')}`); }

  // ---- module
  if (!BREAK) { const r = await volley(1, { row: true }), d = parts(on, r);
    verdict(!d && r.there > 50 && on.there === 0, 'module: the volley with RUST.falls on is the volley with it off, and the rules were the module\'s', `${d || 'the same game after every tick'} · ${r.there} rules taken in the module (${on.there} without the row)`); }

  // ---- snapshot: the falls in the module, taken and put back while glass is in the air and on the floor
  if (!BREAK) { const RB: any = await import(`${ROOT}/src/game/rollback.ts`), was = { row: S.RUST.falls }; hush(true); let line = '', ok = false;
    try { const { game, frame } = await stage(1, { row: true }); let f = 0; for (; f < 200; f++) frame(f);
      const snap = RB.takeSnapshot(game), h0 = RB.stateHash(game), waiting = game.falls.waiting, cells = game.falls.cells().length, home = !!game.falls.home, first: string[] = [];
      for (let k = 0; k < 260; k++) { frame(f + k); first.push(RB.stateHash(game)); }
      const after = game.falls.cells().length;
      RB.restoreSnapshot(snap); const stood = RB.stateHash(game) === h0 && game.falls.waiting === waiting && game.falls.cells().length === cells && !!game.falls.home;
      let part = -1; for (let k = 0; k < 260; k++) { frame(f + k); if (part < 0 && RB.stateHash(game) !== first[k]) part = k; }
      ok = stood && part < 0 && home && waiting > 0 && cells > 0 && after > cells && snap.simViews === 0;
      line = `taken at tick 200 with ${waiting} landings waiting and glass in ${cells} cells (${after} cells 260 ticks on), the store at home in the block: ${home} · put back: the state's word, the landings and the glass as taken: ${stood} · ${part < 0 ? '260 ticks, the same word after each' : 'the word parts at tick ' + part}`;
    } finally { hush(false); S.RUST.falls = was.row; }
    verdict(ok, 'snapshot: with the falls in the module, a snapshot taken while glass is falling and put back plays on as it did', line); }

  console.log(fails ? `\n${fails} FAILED` : '\nall pass');
  return fails === 0;
}
if (import.meta.main) process.exit(await glassfallCheck() ? 0 : 1);
