// THE DEBRIS POOL IS DRESSING (src/game/destructionSwitch.ts DEBRIS.dress; src/game/debris.ts thrown / landings; docs/agents/destruction.md) — is what a
// thing that breaks does to the GAME (the floor hearing its pieces come down) decided by the simulation alone, and by nothing a machine draws?
//   bun run tools/qa/debrisfall-check.ts                       the frozen stand (also `bun run tools/qa/probes.ts debrisfall`) (the rule is raised by the check itself, as TPS_DEBRISFALL=1 raises it)
//   TPS_PROFILE=shipped bun run tools/qa/debrisfall-check.ts   the shipped stand
//   bun run tools/qa/debrisfall-check.ts --seeds 8             more seeds for the tallies
//   DEBRISFALL_BREAK=1 bun run tools/qa/debrisfall-check.ts    the red path: `pool` is asked with the rule OFF (each chunk the pool holds tells the floor
//                                                              itself, as it was) — `pool` must then FAIL
// THE SCENE is the furniture probes' room, the warehouse's office as the stand has it: a wall charge on the office wall from the corridor, forty rounds
// through the hole into the room over three seconds (clutter knocked off, the ceiling shaken), four ceiling tiles let go over the corridor and the
// room (a tile lands hard enough to break again, with a puff of dust), a second charge on the same wall, and five seconds more for everything to come down. Nobody stands on that level here (a bare level has no men): what is compared is everything the men would be told — the
// hearing events — and every bit of the game; how men TAKE the change is the play tallies' to say (docs/agents/destruction.md).
// WHAT IS HELD:
//   tells    with the rule on, pieces are thrown, the simulation puts a landing in the falls for each that comes down hard enough, and the floor is told
//            of exactly those (no chunk tells it); and where the men are told is where the player's chunk knocks: each landing against its own chunk's
//            first hard touch in the pool at 64, half of them inside LAND_NEAR metres and LAND_SOON seconds
//   again    a checkpoint saved before the first charge and loaded after the scene (the clock goes back four hundred ticks): the same charge again
//            tells the floor as it did the first time
//   pool     THE RULE: the scene played with the pool at its two tiers' sizes (64, 32), at 4, and with the dressing row down (dress.ts DRESS.debris: no
//            pool at all) is ONE GAME: the same hearing events at the same ticks, the same dust let into the smoke the senses read, the same landings
//            waiting, the same digest of what was blown and the same raw bits after every tick
//   turned   a piece is what it was THROWN as: the first charge, then the switch turned ten ticks on with its pieces in the air — on to off, the floor is
//            told what it is told with the switch left on (a dressing chunk landing after tells nothing more); off to on, what it is told with it left
//            off (a chunk that is the game's still tells of itself)
//   sees     the same asked with the rule OFF differs somewhere (the check can see a pool that reaches the game: what the red path leans on)
//   module   with the rule on, the scene with RUST.falls on is the scene with it off after every tick, and the rules were the module's
//   snapshot with the rule and the row on, a snapshot taken while the second charge's pieces are in the air (landings waiting) and put back plays the next 200 ticks to the
//            same state's word after each (SIMSTATE_BREAK=1 — a put back that writes no block — must fail it)
//   tallies  per seed, the rule off (at each tier's pool) against on: pieces asked for, chunks the pool took, times the floor was told of a thing landing
//            — printed for docs/agents/destruction.md; judged only for being a game (pieces thrown and heard both ways)
// Not game code; nothing here needs a GPU, and nothing here has been seen or heard on one.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT, fake } from './headless.ts';
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { Particles } = await import(`${ROOT}/src/render/particles.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { DESTRUCTION, FURNBLAST, DEBRIS } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { DRESS } = await import(`${ROOT}/src/game/dress.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const DG: any = await import(`${ROOT}/src/game/digest.ts`), SV: any = await import(`${ROOT}/src/game/savestate.ts`);
const S: any = await import(`${ROOT}/src/sim/sim.ts`), FH: any = await import(`${ROOT}/src/sim/fallshome.ts`);
type Vec3 = [number, number, number];
const DT = 1 / 60, TICKS = 600, LAND_NEAR = 1.0, LAND_SOON = 0.35, BREAK = process.env.DEBRISFALL_BREAK === '1';
if (process.env.SIMSTATE_BREAK === '1') { const P = (await import(`${ROOT}/src/sim/state.ts`) as any).SimState.prototype, put = P.put; P.put = function (this: unknown, c: unknown[]) { if (c.length === 0) put.call(this, c); }; }   // (tools/qa/simstate-check.ts's red path: a take put back writes nothing)
const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const SEEDS = Number(flag('--seeds') ?? 4), FAR = argv.includes('--far');   // (--far: every piece whose chunk knocked more than 1.5 m from its landing, as it happens)
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const log = console.log, warn = console.warn, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.warn = on ? () => {} : warn; console.error = on ? () => {} : err; };
/** the warehouse's office (tools/qa/furnblast-probes.ts OFFICE): the wall between the corridor and it runs along z = 17 */
const CHARGE: Vec3 = [8.0, 0.9, 16.9], CHARGE2: Vec3 = [9.6, 0.9, 16.9], SAM: Vec3 = [8.0, 0, 14.6];
const TILES: Vec3[] = [[6.4, 0, 15.2], [9.4, 0, 15.4], [7.2, 0, 18.6], [9.0, 0, 19.4]];
interface Opts { dress: boolean; cap?: number; noPool?: boolean; row?: boolean }
interface Rec { smokes: string[]; tiles: number; apart: number[]; late: number[]; events: string[]; waiting: number; digest: string; dg: unknown; bits: number[]; partsAt: Record<string, number>[]; asked: number; took: number; refused: number; told: number; chunkLandings: number; there: number; peakWaiting: number }
/** the scene stood up, and frame `f` of it as a function a check can call again */
async function stage(seed: number, o: Opts) {
  Math.random = seeded(seed);
  const W: any = await standUp({ uploadEvery: 1, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'warehouse.json'), 'utf8')), {}) }), game = W.game, engine = W.engine;
  DESTRUCTION.on = true; FURNBLAST.on = true; S.RUST.falls = !!o.row; DEBRIS.dress = o.dress; DRESS.debris = !o.noPool;   // (a stand-up puts every switch where its stand has it: set after)
  engine.particles = new Particles(fake('device'), fake('sceneLayout')); engine.settings.gpuParticles = true; if (o.cap) engine.settings.debrisCap = o.cap;
  game.godMode = true; game.tune.chaos = 0.5;
  for (let i = 0; i < 5; i++) W.step(DT);
  game.teleportPlayer(SAM); game.player.char.bodyYaw = game.player.char.aimYaw = 0; game.player.slot = 'rifle'; game.player.holstered = false; game.player.crouch = false;
  W.step(DT); W.step(DT);
  const jit = seeded(seed * 7919 + 3), aim: Vec3[] = []; for (let k = 0; k < 40; k++) aim.push([8.0 + (jit() - 0.5) * 3.2, 0.5 + jit() * 2.2, 17.6 + jit() * 2.6]);
  const T = { tiles: 0 };
  const frame = (f: number) => {
    if (f === 20) game.destr.blast(CHARGE, { who: PLAYER_ID, chaos: false });
    if (f >= 60 && f < 260 && f % 5 === 0) fireWeapon(game, game.player.char, aim[(f - 60) / 5], true, { weapon: WEAPONS.fn2000 });
    if (f >= 40 && f < 56 && f % 4 === 0 && game.destr.tileDrop(TILES[(f - 40) / 4], 0.5, true, PLAYER_ID, false)) T.tiles++;
    if (f === 300) game.destr.blast(CHARGE2, { who: PLAYER_ID, chaos: false });
    W.step(DT);
  };
  return { W, game, frame, T };
}
/** the scene, played: what the GAME keeps of it (and a little of the pool, for the tallies) */
async function play(seed: number, o: Opts): Promise<Rec> {
  const was = { dress: DEBRIS.dress, row: S.RUST.falls, pool: DRESS.debris, on: DESTRUCTION.on, furn: FURNBLAST.on }, there0 = FH.FALLS.there;
  hush(true);
  try {
    const { game, frame, T } = await stage(seed, o), D = game.destr.debris, spawn = D.spawn.bind(D), F = game.falls, sched = F.schedule.bind(F);
    // each piece asked for, its landing as the simulation put it in the falls (if it did), and the chunk the pool made of it (if it did): for `tells`
    const pairs: { t: number; x: number; z: number; c: any; from: string }[] = []; let last: number[] | null = null, tick = 0;
    F.schedule = (...a: any[]) => { if (a[12] === FH.FALL_PIECES) last = [a[0], a[1], a[3]]; return sched(...a); };
    let asked = 0, peak = 0; D.spawn = (...a: unknown[]) => { asked++; last = null; const c = spawn(...a); if (last && c) pairs.push({ t: last[0], x: last[1], z: last[2], c, from: `${a[0]} from ${(a[1] as any).c.map((x: number) => x.toFixed(2))} at ${(a[2] as number[]).map(x => x.toFixed(1))} m/s` }); return c; };
    const smokes: string[] = [], emit = game.smoke.emit.bind(game.smoke); game.smoke.emit = (e: any) => { smokes.push(`${tick}:${e.kind}:${e.pos.map((x: number) => x.toFixed(6)).join(',')}`); return emit(e); };
    const apart: number[] = [], late: number[] = [], landed = new Set<object>();
    const partsAt: Record<string, number>[] = [], events: string[] = [], bits: number[] = [], seen = new WeakSet<object>();
    for (let f = 0; f < TICKS; f++) {
      tick = f; frame(f);
      for (const p of pairs) if (p.c.sounded && !landed.has(p)) { landed.add(p); const far = Math.hypot(p.c.box.c[0] - p.x, p.c.box.c[2] - p.z); apart.push(far); late.push(game.time - p.t); if (FAR && far > 1.5) err(`  far ${far.toFixed(2)} m, ${(game.time - p.t).toFixed(2)} s: ${p.from} · the landing at ${p.x.toFixed(2)}, ${p.z.toFixed(2)} · the chunk knocked at ${p.c.box.c.map((x: number) => x.toFixed(2))}`); }
      for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'prop') events.push(`${f}:${e.level}:${e.pos.map((x: number) => x.toFixed(6)).join(',')}:${e.who}`); }
      const P = DG.rawBitsParts(game); let h = 0x811c9dc5; for (const k of Object.keys(P)) if (k !== 'debris') h = Math.imul(h ^ P[k], 0x01000193) >>> 0; bits.push(h); partsAt.push(P);
      peak = Math.max(peak, game.falls.waitingIn(FH.FALL_PIECES));
    }
    const dg = DG.digestGame(game); delete dg.debris;
    return { smokes, tiles: T.tiles, apart, late, events, waiting: game.falls.waitingIn(FH.FALL_PIECES), digest: DG.hashDigest(dg), dg, bits, partsAt, asked, took: D.stats.spawned, refused: D.stats.refused, told: D.stats.told, chunkLandings: D.stats.landed,
      there: FH.FALLS.there - there0, peakWaiting: peak };
  } finally { hush(false); DEBRIS.dress = was.dress; S.RUST.falls = was.row; DRESS.debris = was.pool; DESTRUCTION.on = was.on; FURNBLAST.on = was.furn; }
}
/** where two plays of the scene part as GAMES ('' = they do not) */
function parts(a: Rec, b: Rec): string {
  const list = (k: 'events' | 'smokes') => { const x = a[k], y = b[k]; if (x.length === y.length && x.every((v, i) => v === y[i])) return ''; let i = 0; while (i < x.length && i < y.length && x[i] === y[i]) i++; return `${k} (${x.length} | ${y.length}) from #${i}: ${x[i] ?? '∅'} | ${y[i] ?? '∅'}`; };
  const tick = a.bits.findIndex((w, i) => w !== b.bits[i]);
  return [list('events'), list('smokes'), a.waiting !== b.waiting ? `landings waiting ${a.waiting} | ${b.waiting}` : '', a.digest !== b.digest ? `digest: ${DG.diffDigest(a.dg, b.dg, 3).join('; ')}` : '', tick >= 0 ? `raw bits from tick ${tick}: ${Object.keys(a.partsAt[tick]).filter(k => k !== 'debris' && a.partsAt[tick][k] !== b.partsAt[tick][k]).join(', ')}` : ''].filter(Boolean).join(' · ');
}
/** every line above (tools/qa/probes.ts `debrisfall`): true = all pass */
export async function debrisfallCheck(): Promise<boolean> {
  fails = 0;
  console.log(`the debris pool as dressing · profile ${process.env.TPS_PROFILE ?? 'frozen'}${BREAK ? ' · DEBRISFALL_BREAK=1 (the red path)' : ''}`);
  const POOLS: [string, Opts][] = [['the pool at 32', { dress: true, cap: 32 }], ['the pool at 4', { dress: true, cap: 4 }], ['no pool (the dressing row down)', { dress: true, noPool: true }]];

  // ---- tells
  const on = await play(1, { dress: true, cap: 64 }), off = await play(1, { dress: false, cap: 64 });
  const med = (v: number[]) => v.length ? [...v].sort((a, b) => a - b)[v.length >> 1] : NaN, worst = (v: number[]) => v.reduce((m, x) => Math.max(m, Math.abs(x)), 0), lateAbs = on.late.map(Math.abs);
  verdict(on.asked > 20 && on.told > 10 && on.took > 10 && on.events.length >= on.told - on.waiting && on.events.length > 10 && off.told === 0 && off.peakWaiting === 0 && on.tiles >= 3 && on.apart.length > 10 && med(on.apart) < LAND_NEAR && med(lateAbs) < LAND_SOON,
    `tells: with the rule on, pieces are thrown, the simulation puts a landing in the falls for each that comes down hard enough, the floor is told of those, and half of them are within ${LAND_NEAR} m and ${LAND_SOON} s of their own chunk's knock`,
    `${on.asked} pieces asked for, ${on.took} taken by the pool (${on.refused} refused) · ${on.told} landings put in the falls (at most ${on.peakWaiting} waiting at once, ${on.waiting} at the end) · the floor told ${on.events.length} times of a thing landing (pieces and what the men knocked) · chunks that landed in the pool, for the ear: ${on.chunkLandings} · with the rule off: ${off.told} landings in the falls, the floor told ${off.events.length} times\n    ${on.tiles} ceiling tiles let go, ${on.smokes.length} puffs into the smoke · a landing against its own chunk's first hard touch (${on.apart.length} pieces whose chunk the pool at 64 took and landed): ${med(on.apart).toFixed(2)} m apart at the median, ${worst(on.apart).toFixed(2)} m at the worst; the chunk's knock ${med(on.late).toFixed(2)} s after the landing at the median, ${worst(on.late).toFixed(2)} s off at the worst`);

  // ---- again: the clock put back by a load
  if (!BREAK) { const was = { dress: DEBRIS.dress, row: S.RUST.falls, on: DESTRUCTION.on, furn: FURNBLAST.on }; hush(true); let line = '', ok = false;
    try { const { game, frame } = await stage(1, { dress: true, cap: 64 }), seen = new WeakSet<object>(), count = () => { let n = 0; for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'prop') n++; } return n; };
      for (let f = 0; f < 10; f++) frame(f); count();
      const save = SV.saveState(game, { force: true }), t0 = game.time; let first = 0, again = 0;
      for (let f = 10; f < 400; f++) { frame(f); first += count(); }
      const why = SV.canLoad(game, save); if (!why) SV.loadState(game, save); const back = game.time; count();
      for (let f = 10; f < 400; f++) { frame(f); again += count(); }
      ok = !why && back < t0 + 1 && first > 10 && again === first;
      line = why ? `the checkpoint would not load: ${why}` : `saved at ${t0.toFixed(2)} s, loaded at ${(t0 + 390 * DT).toFixed(2)} s (the clock back at ${back.toFixed(2)} s) · the floor told ${first} times the first time through, ${again} the second`;
    } finally { hush(false); DEBRIS.dress = was.dress; S.RUST.falls = was.row; DESTRUCTION.on = was.on; FURNBLAST.on = was.furn; }
    verdict(ok, 'again: a checkpoint loaded after the scene puts the clock back, and the same charges tell the floor as they did the first time', line); }

  // ---- pool (and, with the rule off, sees)
  { const lines: string[] = []; let same = 0;
    for (const [name, o] of POOLS) { const r = await play(1, { ...o, dress: !BREAK }), base = BREAK ? off : on, d = parts(base, r); if (!d) same++; lines.push(`${name} (${r.took} chunks, ${r.refused} refused): ${d ? 'ANOTHER GAME — ' + d.slice(0, 260) : 'the same game'}`); }
    verdict(same === POOLS.length, `pool: the scene is one game whatever the debris pool holds, and with no pool${BREAK ? ' (ASKED WITH THE RULE OFF: the red path)' : ''}`, `against the pool at 64 (${(BREAK ? off : on).took} chunks) · ${lines.join('\n    ')}`); }
  if (!BREAK) { const lines: string[] = []; let differ = 0;
    for (const [name, o] of POOLS.slice(0, 2)) { const r = await play(1, { ...o, dress: false }), d = parts(off, r); if (d) differ++; lines.push(`${name}: ${d ? d.slice(0, 200) : 'the same game'}`); }
    verdict(differ >= 1, 'sees: with the rule off the same plays are other games (the pool reaches the game there, and this check sees it)', lines.join('\n    ')); }

  // ---- turned: the switch turned with pieces in the air
  if (!BREAK) { const was = { dress: DEBRIS.dress, row: S.RUST.falls, on: DESTRUCTION.on, furn: FURNBLAST.on };
    const air = async (from: boolean, turn: boolean): Promise<string[]> => { hush(true); const ev: string[] = [];
      try { const { W, game, frame } = await stage(1, { dress: from, cap: 64 }), seen = new WeakSet<object>();
        for (let f = 0; f < 200; f++) { if (f === 30 && turn) DEBRIS.dress = !from; if (f < 30) frame(f); else W.step(DT);   // (past tick 30 nothing new is thrown: only what the first charge threw comes down)
          for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'prop') ev.push(`${f}:${e.level}:${e.pos.map((x: number) => x.toFixed(6)).join(',')}:${e.who}`); } }
      } finally { hush(false); DEBRIS.dress = was.dress; S.RUST.falls = was.row; DESTRUCTION.on = was.on; FURNBLAST.on = was.furn; }
      return ev; };
    const onRef = await air(true, false), onOff = await air(true, true), offRef = await air(false, false), offOn = await air(false, true), same = (a: string[], b: string[]) => a.length === b.length && a.every((v, i) => v === b[i]), late = (a: string[]) => a.filter(e => Number(e.split(':')[0]) > 30).length;
    verdict(same(onRef, onOff) && same(offRef, offOn) && late(onRef) > 3 && late(offRef) > 3, 'turned: a piece in the air when the switch turns lands as it was thrown — the floor is told what it would have been told',
      `on, then off at tick 30: the floor told ${onOff.length} times (${onRef.length} with the switch left on, ${late(onRef)} of them after the turn) · off, then on: ${offOn.length} (${offRef.length} with it left off, ${late(offRef)} after)`); }

  // ---- module
  if (!BREAK) { const r = await play(1, { dress: true, cap: 64, row: true }), d = parts(on, r);
    verdict(!d && r.there > 20 && on.there === 0, 'module: with the rule on, the scene with RUST.falls on is the scene with it off, and the rules were the module\'s', `${d || 'the same game after every tick'} · ${r.there} rules taken in the module (${on.there} without the row)`); }

  // ---- snapshot: the pieces' landings in the module, taken and put back while pieces are in the air
  if (!BREAK) { const RB: any = await import(`${ROOT}/src/game/rollback.ts`), was = { dress: DEBRIS.dress, row: S.RUST.falls, on: DESTRUCTION.on, furn: FURNBLAST.on }; hush(true); let line = '', ok = false;
    try { const { game, frame } = await stage(1, { dress: true, cap: 64, row: true }); let f = 0; for (; f < 303 || (f < 360 && game.falls.waitingIn(FH.FALL_PIECES) < 3); f++) frame(f);   // (the second charge's pieces: the level's own autosave — whose slot a put back does not take off the shelf, tools/qa/rollback-check.ts — is behind it by then)
      const snap = RB.takeSnapshot(game), h0 = RB.stateHash(game), waiting = game.falls.waitingIn(FH.FALL_PIECES), home = !!game.falls.home, first: string[] = [], at = f;
      for (let k = 0; k < 200; k++) { frame(f + k); first.push(RB.stateHash(game)); }
      RB.restoreSnapshot(snap); const stood = RB.stateHash(game) === h0 && game.falls.waitingIn(FH.FALL_PIECES) === waiting && !!game.falls.home;
      let part = -1; for (let k = 0; k < 200; k++) { frame(f + k); if (part < 0 && RB.stateHash(game) !== first[k]) part = k; }
      ok = stood && part < 0 && home && waiting > 0 && snap.simViews === 0;
      line = `taken at tick ${at} with ${waiting} landings of pieces waiting, the store at home in the block: ${home} · put back: the state's word and the landings as taken: ${stood} · ${part < 0 ? '200 ticks, the same word after each' : 'the word parts at tick ' + part}`;
    } finally { hush(false); DEBRIS.dress = was.dress; S.RUST.falls = was.row; DESTRUCTION.on = was.on; FURNBLAST.on = was.furn; }
    verdict(ok, 'snapshot: with the falls in the module, a snapshot taken while pieces are in the air and put back plays on as it did', line); }

  // ---- tallies
  if (!BREAK) { const lines: string[] = []; let ok = true;
    for (let s = 1; s <= SEEDS; s++) { const a = s === 1 ? off : await play(s, { dress: false, cap: 64 }), b = s === 1 ? on : await play(s, { dress: true, cap: 64 }), c = await play(s, { dress: false, cap: 32 });
      if (!(a.asked > 0 && b.asked > 0 && a.events.length > 0 && b.events.length > 0)) ok = false;
      const row = (r: Rec) => `${r.asked} pieces asked for, ${r.took} chunks (${r.refused} refused), the floor told ${r.events.length} times`;
      lines.push(`seed ${s}: off at 64: ${row(a)} · off at 32: ${row(c)} · on: ${row(b)}`); }
    verdict(ok, `tallies: ${SEEDS} seeds, the rule off (at each tier's pool) against on (for the area file; judged only for pieces thrown and heard both ways)`, lines.join('\n    ')); }
  console.log(fails ? `\n${fails} FAILED` : '\nall pass');
  return fails === 0;
}
if (import.meta.main) process.exit(await debrisfallCheck() ? 0 : 1);
