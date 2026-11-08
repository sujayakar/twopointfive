// The 'particles' probe family (EXPERIMENT E4 — GPU particles: render/particles.ts + game/particleFx.ts), run through probes.ts:
//   bun run tools/qa/probes.ts particles | particles-queue | particles-sites | particles-off | particles-determinism [--verbose]
// Headless = the Proxy device: no kernel runs, nothing is drawn. What IS asserted is every CPU-side promise the picture rests on:
//   (queue)        the emit queue → uniform block: records packed at the documented offsets, births counted, kinds dealt from the right ring in ring order, the
//                  record cap and the birth cap drop-and-count (no silent loss), a lapped ring advances its tail (oldest-die, the window never exceeds the ring),
//                  windows shrink again once every particle of an emit has outlived its longest life, clear() empties both, prepare(off) drops everything;
//   (sites)        with a Particles attached and the setting on, AT CHAOS 0 (the family's own hold, stage() below: the page's 0.5 adds destruction's burst,
//                  wall wear's flakes and a dropped tile's dust at the same points, which are their own families'), each scripted event queues what its
//                  site says and nothing else — but that a round which goes on through a thin wall is counted where it struck (its far face and what it
//                  meets beyond are the thin walls'). PARTSITES_BREAK=1 = each round's site read a metre off; =chaos = the hold dropped (0.5): a Five-seveN round into the
//                  corridor's plaster (chips + dust by the drywall recipe, one brass case, the muzzle's powder sparks — and the CPU spark pool stays EMPTY), an
//                  FN2000 round (rifle scaling, its own case out of the front), a round into a server rack (sparks: metal), the stun charge (PFX.bangSparks fragments
//                  + the dust ring + the puff on the GPU, exactly the trail-carrying dozen in the CPU pool), a kicked door (dust + splinters at the jamb), the ring's
//                  clack, a tracer's ember; and the case is HEARD landing exactly once per round, ~0.5 s later, never as a hearing event;
//   (off)          settings.gpuParticles = false with the same events: zero GPU emits, no casingTink, and the CPU pool holds what it always held (20 powder sparks
//                  a shot, 72 fragments a bang) — today's game exactly;
//   (determinism)  the same scripted minute twice → the identical emit stream (kinds, counts, positions, seeds), so a ?paused capture repeats.
// The WGSL itself is compile-checked through the page (__wgsl with the scene prelude — CLAUDE.md), not here.
import { standUp, ROOT, fake } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { WALLPEN_STATS } = await import(`${ROOT}/src/game/wallpen.ts`);   // (did a round go on through the wall it struck: the thin walls' own count)
const { Particles, RINGS, MAX_EMITS, MAX_NEW_PER_FRAME, KIND_RING, MAX_QUEUE, BLOCK_HDR_BYTES, REC_BYTES } = await import(`${ROOT}/src/render/particles.ts`);
const H = BLOCK_HDR_BYTES / 4, RW = REC_BYTES / 4;   // the uniform block's header words / an emit record's words (records at BLOCK_HDR_BYTES + REC_BYTES·r)
const { fireWeapon, detonateStun } = await import(`${ROOT}/src/game/combat.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const { pendingTinks, PFX } = await import(`${ROOT}/src/game/particleFx.ts`);
const fxm = await import(`${ROOT}/src/game/effects.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const mkVerdict = () => { const st = { all: true }; return { st, verdict: (ok: boolean, label: string, detail: string) => { st.all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); } }; };

/** the harness Game with a Particles attached to its engine stand-in (what game.ts does on a real Engine) and the setting as asked */
/** THE FAMILY'S OWN HOLD: chaos 0. Every line here is what particleFx queues at a site, by its own recipe. The harness stood every game at chaos 0 until a
 *  tool with no word stood the page's (src/host/standup.ts: 'the destruction probes raise it themselves'); at the page's 0.5 a round's strike also carries
 *  destruction.ts's burst (by the heat of the rounds before it) and wallwear.ts's flakes, and a stun drops a ceiling tile's dust — same points, other
 *  modules, their own families (loudchaos, wallwear). PARTSITES_BREAK=chaos, the red path of the hold: 0.5 on either stand. */
const BREAK = process.env.PARTSITES_BREAK ?? '', CHAOS_HELD = BREAK === 'chaos' ? 0.5 : 0;
async function stage(seed: number, on = true) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game, engine } = W; game.tune.chaos = CHAOS_HELD;
  engine.particles = new Particles(fake('device'), fake('sceneLayout'));
  engine.settings.gpuParticles = on;
  game.godMode = true; game.player.visibility = 0.02;
  for (let i = 0; i < 5; i++) W.step(DT);
  game.guards.forEach((gd: any, i: number) => { gd.hold = true; gd.pinned = true; gd.char.pos = [37.5, 0, 6 + i * 1.5]; gd.heardUpTo = 1e12; gd.dazzledUntil = 1e9; gd.flashlight.enabled = false; });
  game.teleportPlayer([21.5, 0, 11.5]); game.player.slot = 'pistol'; game.player.holstered = false; game.player.crouch = false; game.player.pistol.lightOn = false;
  W.step(DT); W.step(DT);
  const P = engine.particles;
  /** what has been queued since the last flush, then flush it through prepare (as engine.render would) */
  const flush = (dt = DT) => { const q = P.pending.map((e: any) => ({ kind: e.kind, count: e.count, pos: e.pos.map((x: number) => +x.toFixed(3)), seed: e.seed, shape: e.o.shape ?? 'cone', color: e.o.color, owner: e.o.owner ?? 0 })); P.prepare(dt, engine.settings.gpuParticles); return q; };
  return { W, game, engine, P, flush };
}
const WALL: Vec3 = [21.5, 1.2, 10.0];       // the corridor's north wall in front of him: plaster ('wall' in the palette → drywall recipe)
const countKinds = (q: any[]) => { const c: Record<string, number> = { spark: 0, debris: 0, dust: 0, casing: 0 }; for (const e of q) c[e.kind] += e.count; return c; };
/** A ROUND'S CHIPS AND DUST WHERE IT STRUCK, and whether it went on: the emits within 0.3 m of the point his round first strikes, on his side of that face
 *  (the recipe's sit 0.015 and 0.07 m off it), and the things the round crossed by the thin walls' own count. A round that stops in the wall owes the whole
 *  queue to that site — 'and nothing else', as these lines always read it; one that goes on through also chips the far face and what it meets beyond, which
 *  wallpen-check and decals-sites hold, and is counted at its strike alone. PARTSITES_BREAK=1, the red path: the site is read a metre along the wall. */
function shotAt(game: any, to: Vec3, fire: () => void, flush: () => any[]): { q: any[]; all: Record<string, number>; site: Record<string, number> | null; through: boolean } {
  const from = [...game.player.char.muzzle] as Vec3, d = v3.normalize(v3.sub(to, from)), h = game.col.raycast(from, d, 40);
  const c0 = WALLPEN_STATS.crossed; fire(); const crossed = WALLPEN_STATS.crossed - c0;
  const q = flush(), all = countKinds(q); if (!h) return { q, all, site: null, through: crossed > 0 };
  const p = v3.mad(from, d, h.t), c: Vec3 = [p[0] + (BREAK === '1' ? 1 : 0), p[1], p[2]], n = h.n;
  const near = q.filter((e: any) => (e.kind === 'debris' || e.kind === 'dust') && Math.hypot(e.pos[0] - c[0], e.pos[1] - c[1], e.pos[2] - c[2]) <= 0.3 && (e.pos[0] - c[0]) * n[0] + (e.pos[1] - c[1]) * n[1] + (e.pos[2] - c[2]) * n[2] > -1e-3);
  return { q, all, site: countKinds(near), through: crossed > 0 };
}
/** the line's own clause: the recipe at the strike, and the whole queue no more than that where the round stopped there */
const recipeAt = (s: ReturnType<typeof shotAt>, chips: number, dust: number): string | null => !s.site ? 'THE ROUND STRUCK NOTHING' : s.site.debris !== chips || s.site.dust !== dust ? `where the round struck: debris ${s.site.debris} dust ${s.site.dust} (want ${chips} / ${dust}; the whole queue ${s.all.debris} / ${s.all.dust})`
  : !s.through && (s.all.debris !== chips || s.all.dust !== dust) ? `the round stopped in the wall and the queue holds more than its strike: debris ${s.all.debris} dust ${s.all.dust} (want ${chips} / ${dust} and nothing else)` : null;

// ================================================================ queue
export async function particlesQueue() {
  console.log('\n=== particles-queue: the emit queue → the uniform block, the two rings and their windows, the caps, expiry, clear ===');
  const { st, verdict } = mkVerdict();
  const P = new Particles(fake('device'), fake('sceneLayout'));
  const bu: Uint32Array = (P as any).bu, bf: Float32Array = (P as any).bf;
  { // packing + rings
    P.emit('spark', [1, 2, 3], [0, 1, 0], 10, { speed: 5, life: 0.5, size: 0.003, color: [1, 0.5, 0.25] });
    P.emit('casing', [4, 5, 6], [1, 0, 0], 1, { speed: 2, life: 45, size: 0.009, owner: 7, color: [0.72, 0.5, 0.16] });
    P.emit('dust', [7, 8, 9], [0, 0, 1], 5, { shape: 'ring', radius: 0.3, speed: 3, life: 1.5, size: 0.1 });
    const pend = P.pending.length;
    P.prepare(DT, true);
    const fails: string[] = [];
    if (pend !== 3) fails.push(`pending ${pend} ≠ 3`);
    if (bu[0] !== 3 || bu[1] !== 16) fails.push(`header numRecs/totalNew = ${bu[0]}/${bu[1]} (want 3/16)`);
    // record 0 at word H: pos, count, dir, speed, first, kind word, base slot
    if (bf[H] !== 1 || bf[H + 1] !== 2 || bf[H + 2] !== 3 || bu[H + 3] !== 10) fails.push(`rec0 pos/count = ${bf[H]},${bf[H + 1]},${bf[H + 2]} / ${bu[H + 3]}`);
    if (bu[H + 11] !== 0 || bu[H + RW + 11] !== 10 || bu[H + 2 * RW + 11] !== 11) fails.push(`first indices ${bu[H + 11]}, ${bu[H + RW + 11]}, ${bu[H + 2 * RW + 11]} (want 0, 10, 11)`);
    const k0 = bu[H + 24], k1 = bu[H + RW + 24], k2 = bu[H + 2 * RW + 24];
    if ((k0 & 15) !== 0 || ((k0 >> 16) & 1) !== 0) fails.push(`rec0 kind/ring ${k0 & 15}/${(k0 >> 16) & 1} (spark → 0 / ring 0)`);
    if ((k1 & 15) !== 3 || ((k1 >> 16) & 1) !== 1 || ((k1 >> 8) & 255) !== 7) fails.push(`rec1 kind/ring/owner ${k1 & 15}/${(k1 >> 16) & 1}/${(k1 >> 8) & 255} (casing → 3 / ring 1 / owner 7)`);
    if ((k2 & 15) !== 2 || ((k2 >> 16) & 1) !== 0 || bu[H + 2 * RW + 15] !== 2) fails.push(`rec2 kind/ring/shape ${k2 & 15}/${(k2 >> 16) & 1}/${bu[H + 2 * RW + 15]} (dust → 2 / ring 0 / shape ring=2)`);
    if (bu[H + 26] !== 0 || bu[H + 2 * RW + 26] !== 10 || bu[H + RW + 26] !== 0) fails.push(`slot bases ${bu[H + 26]}, ${bu[H + 2 * RW + 26]}, ${bu[H + RW + 26]} (want spark 0, dust 10 in ring 0; casing 0 in ring 1)`);
    const win = P.windows;
    if (win[0][3] !== 15 || win[1][3] !== 1 || win[0][2] !== 0 || win[1][0] !== RINGS[1].start) fails.push(`windows ${JSON.stringify(win)} (want ring0 live 15 from 0, ring1 live 1 at ${RINGS[1].start})`);
    if ((bu[4 + 3] !== 15) || bu[8 + 3] !== 1 || bu[8] !== RINGS[1].start) fails.push(`window words in the block: ${[...bu.slice(4, 12)].join(',')}`);
    const col = bu[H + 27]; if ((col & 255) !== 255 || ((col >> 8) & 255) !== 128 || ((col >> 16) & 255) !== 64) fails.push(`rec0 colour rgba8 ${col.toString(16)} (want ff,80,40)`);
    verdict(fails.length === 0, `three emits (10 sparks, 1 casing, 5 ring dust) → one block: header 3 records / 16 births, records at ${BLOCK_HDR_BYTES} + ${REC_BYTES}·r with pos / count / first / kind|owner<<8|ring<<16 / shape / slot base / rgba8, sparks + dust dealt ring 0 slots 0‥14 in order, the casing ring 1 slot 0, both windows sized to match`,
      fails.length ? fails.join('; ') : `block words: hdr [${[...bu.slice(0, 4)].join(',')}] win [${[...bu.slice(4, 12)].join(',')}]; rec kinds ${k0 & 15},${k1 & 15},${k2 & 15}; windows ${JSON.stringify(win)}`);
  }
  { // caps (renderer v3 Phase 0: the frame's record cap and birth cap DEFER instead of drop — what does not fit waits in the queue for the next frames, long-lived kinds
    // dealt before sparks once anything has to wait, a spark record shed after SPARK_MAX_WAIT frames of waiting; the queue itself is bounded at MAX_QUEUE, shedding
    // the oldest waiting spark record first)
    const opts = { speed: 1, life: 0.2, size: 0.002 };
    const P2 = new Particles(fake('device'), fake('sceneLayout')); let acc = 0;
    for (let i = 0; i < MAX_EMITS + 5; i++) acc += P2.emit('spark', [0, 0, 0], [0, 1, 0], 1, opts);
    P2.prepare(DT, true); const rec1 = P2.stats.records, wait1 = P2.pending.length; P2.prepare(DT, true); const rec2 = P2.stats.records, wait2 = P2.pending.length;
    const P3 = new Particles(fake('device'), fake('sceneLayout'));
    P3.emit('spark', [0, 0, 0], [0, 1, 0], MAX_NEW_PER_FRAME - 10, opts); P3.emit('spark', [0, 0, 0], [0, 1, 0], 50, opts); P3.emit('dust', [0, 0, 0], [0, 1, 0], 5, { speed: 1, life: 0.2, size: 0.1 });
    P3.prepare(DT, true); const born3 = P3.stats.born, drop3 = P3.stats.droppedBirths, wait3 = P3.pending.length; P3.prepare(DT, true); const born3b = P3.stats.born;
    // the per-frame cap handed in by the engine (settings.particleBirths): 200 + 200 sparks + a case at cap 384 → the case and the first 200 now, the second 200 next frame, in that order
    const P6 = new Particles(fake('device'), fake('sceneLayout'));
    P6.emit('spark', [1, 0, 0], [0, 1, 0], 200, opts); P6.emit('spark', [2, 0, 0], [0, 1, 0], 200, opts); P6.emit('casing', [3, 0, 0], [0, 1, 0], 1, { speed: 1, life: 45, size: 0.009 });
    P6.prepare(DT, true, 384); const born6 = P6.stats.born, recs6 = P6.stats.records, wait6 = P6.pending.length, waitPos6 = P6.pending[0]?.pos[0]; P6.prepare(DT, true, 384); const born6b = P6.stats.born;
    // a record bigger than the whole cap is born alone rather than never; a spark record that cannot get in for SPARK_MAX_WAIT + 1 frames is shed and counted
    const P7 = new Particles(fake('device'), fake('sceneLayout'));
    P7.emit('dust', [0, 0, 0], [0, 1, 0], 50, { speed: 1, life: 1, size: 0.1 }); P7.emit('spark', [0, 0, 0], [0, 1, 0], 160, opts);
    P7.prepare(DT, true, 100); const born7a = P7.stats.born; P7.prepare(DT, true, 100); const born7b = P7.stats.born;
    // starvation: a 50-spark record behind 100 dust EVERY frame at cap 100 never finds room — after 2 frames of waiting it is dealt past the cap (once a frame; the ceiling holds)
    const P8 = new Particles(fake('device'), fake('sceneLayout')); P8.emit('spark', [0, 0, 0], [0, 1, 0], 50, opts);
    let rescuedAt = -1, rescuedBorn = 0; for (let f = 0; f < 8 && rescuedAt < 0; f++) { P8.emit('dust', [0, 0, 0], [0, 1, 0], 100, { speed: 1, life: 1, size: 0.1 }); P8.prepare(DT, true, 100); if (P8.stats.born > 100) { rescuedAt = f; rescuedBorn = P8.stats.born; } }
    // the queue's own bound: full (MAX_QUEUE) → the oldest waiting SPARK record is shed to make room; nothing else is ever shed — a full queue of chips refuses the newcomer instead
    const P9 = new Particles(fake('device'), fake('sceneLayout')); P9.emit('spark', [9, 0, 0], [0, 1, 0], 7, opts); for (let i = 1; i < MAX_QUEUE; i++) P9.emit('debris', [0, 0, 0], [0, 1, 0], 1, { speed: 1, life: 10, size: 0.01 });
    const full = P9.pending.length; const acceptedOver = P9.emit('debris', [1, 0, 0], [0, 1, 0], 1, { speed: 1, life: 10, size: 0.01 }); const shed9 = P9.shedSparks, stillSpark = P9.pending.some((r: any) => r.kind === 'spark');
    const refused = P9.emit('debris', [2, 0, 0], [0, 1, 0], 1, { speed: 1, life: 10, size: 0.01 }); const droppedRecs9 = P9.stats.droppedRecs;
    const fails: string[] = [];
    if (acc !== MAX_EMITS + 5 || rec1 !== MAX_EMITS || wait1 !== 5 || rec2 !== 5 || wait2 !== 0 || P2.stats.droppedRecs !== 0) fails.push(`record cap: accepted ${acc}, frame 1 records ${rec1} / waiting ${wait1}, frame 2 records ${rec2} / waiting ${wait2}, refused ${P2.stats.droppedRecs} (want ${MAX_EMITS + 5}; ${MAX_EMITS} / 5; 5 / 0; 0)`);
    if (born3 !== MAX_NEW_PER_FRAME - 10 + 5 || drop3 !== 0 || wait3 !== 1 || born3b !== 50) fails.push(`birth ceiling: born ${born3}, dropped ${drop3}, waiting ${wait3}, next frame born ${born3b} (want ${MAX_NEW_PER_FRAME - 5}: the dust dealt first, the 50 that would cross ${MAX_NEW_PER_FRAME} wait — not dropped — and go next frame)`);
    if (born6 !== 201 || recs6 !== 2 || wait6 !== 1 || waitPos6 !== 2 || born6b !== 200) fails.push(`cap 384: born ${born6} in ${recs6} records, waiting ${wait6} (the record at x=${waitPos6}), next frame ${born6b} (want 201 in 2 — the case + the FIRST 200 — the second 200 waiting, then 200)`);
    if (born7a !== 50 || born7b !== 160) fails.push(`oversize record: frame 1 born ${born7a}, frame 2 born ${born7b} (want the 50 dust, then the 160 sparks alone though 160 > cap 100)`);
    if (rescuedAt !== 2 || rescuedBorn !== 150 || P8.shedSparks !== 0) fails.push(`starved spark record: dealt past the cap at frame ${rescuedAt} with ${rescuedBorn} born, shed ${P8.shedSparks} (want frame 2, 150, 0)`);
    if (full !== MAX_QUEUE || acceptedOver !== 1 || shed9 !== 7 || stillSpark || refused !== 0 || droppedRecs9 !== 1) fails.push(`queue bound: ${full} waiting (want ${MAX_QUEUE}); over it a chip is accepted ${acceptedOver} by shedding the spark record (shed ${shed9}, spark still queued ${stillSpark}); with no spark left the next is refused (${refused}) and counted (${droppedRecs9}) (want 1, 7, false, 0, 1)`);
    verdict(fails.length === 0, `caps defer, never silently lose: the ${MAX_EMITS + 1}th‥ record of a frame waits for the next; a record that would cross the frame's birth cap waits (long-lived kinds dealt first once anything waits, arrival order otherwise untouched); an oversize record is born alone; a spark record starved 2 frames behind steady traffic goes past the cap; a full queue (${MAX_QUEUE}) sheds its oldest spark record, else refuses`, fails.length ? fails.join('; ') : `records ${rec1}+${rec2}; ceiling ${born3}+${born3b}; cap 384 → ${born6}+${born6b}; oversize ${born7a}+${born7b}; starved → frame ${rescuedAt} (${rescuedBorn}); queue ${full} shed ${shed9} refused ${droppedRecs9}`);
  }
  { // ring wrap (oldest-die) + expiry + clear + off
    const P4 = new Particles(fake('device'), fake('sceneLayout')); const size1 = RINGS[1].size;
    for (let f = 0; f < 3; f++) { P4.emit('casing', [0, 0, 0], [0, 1, 0], 4000, { speed: 1, life: 45, size: 0.009 }); P4.prepare(DT, true); }
    const winWrap = P4.windows[1].slice(); const wrapOk = winWrap[3] === size1 && winWrap[2] === (12000 - size1) % size1;
    // expiry: short sparks (life 0.3, lifeJit default 0.35 → max 0.405 + dt + 0.1) gone from the window after ~0.55 s of prepare() time; a later long emit keeps its own
    const P5 = new Particles(fake('device'), fake('sceneLayout'));
    P5.emit('spark', [0, 0, 0], [0, 1, 0], 100, { speed: 1, life: 0.3, size: 0.002 }); P5.prepare(DT, true);
    const live0 = P5.windows[0][3];
    let tGone = -1; for (let f = 1; f < 120; f++) { P5.prepare(DT, true); if (P5.windows[0][3] === 0 && tGone < 0) tGone = f * DT; }
    P5.emit('spark', [0, 0, 0], [0, 1, 0], 30, { speed: 1, life: 0.3, size: 0.002 }); P5.prepare(DT, true); const tailAfter = P5.windows[0][2], liveAfter = P5.windows[0][3];
    // clear + off
    P5.emit('debris', [0, 0, 0], [0, 1, 0], 10, { speed: 1, life: 10, size: 0.01 }); P5.prepare(DT, true); const liveBeforeClear = P5.liveSlots; P5.clear(); P5.prepare(DT, true); const liveAfterClear = P5.liveSlots;
    P5.emit('debris', [0, 0, 0], [0, 1, 0], 10, { speed: 1, life: 10, size: 0.01 }); P5.prepare(DT, false); const offLive = P5.liveSlots, offPending = P5.pending.length, offBorn = P5.stats.born;
    const fails: string[] = [];
    if (!wrapOk) fails.push(`wrap: window ${JSON.stringify(winWrap)} after 12000 casings into a ${size1} ring (want live ${size1}, tail ${(12000 - size1) % size1})`);
    if (live0 !== 100 || !(tGone > 0.45 && tGone < 0.7)) fails.push(`expiry: live ${live0} at birth, window empty at ${tGone.toFixed(2)} s (want 100, 0.45‥0.7 s)`);
    if (tailAfter !== 100 || liveAfter !== 30) fails.push(`after expiry a fresh emit starts at the old head: tail ${tailAfter} live ${liveAfter} (want 100 / 30)`);
    if (!(liveBeforeClear > 0 && liveAfterClear === 0)) fails.push(`clear: live ${liveBeforeClear} → ${liveAfterClear}`);
    if (offLive !== 0 || offPending !== 0 || offBorn !== 0) fails.push(`prepare(off): live ${offLive}, pending ${offPending}, born ${offBorn} (want 0, 0, 0)`);
    verdict(fails.length === 0, 'rings: a lapped LONG ring keeps exactly its size live with the tail moved up (the oldest overwritten); a SHORT emit leaves the window once its longest life is past and the next emit starts where the head was; clear() empties; prepare(off) drops the queue and shows nothing',
      fails.length ? fails.join('; ') : `wrap ${JSON.stringify(winWrap)}; 100 sparks gone at ${tGone.toFixed(2)} s; then tail ${tailAfter} live ${liveAfter}; clear ${liveBeforeClear}→${liveAfterClear}; off ${offLive}/${offPending}/${offBorn}`);
    if (KIND_RING.spark !== 0 || KIND_RING.dust !== 0 || KIND_RING.debris !== 1 || KIND_RING.casing !== 1) { st.all = false; console.log('- FAIL — KIND_RING is not spark/dust → 0, debris/casing → 1'); }
  }
  console.log(`=== particles-queue: ${st.all ? 'ALL PASS' : 'FAILURES above'} ===`);
  if (!st.all) process.exitCode = 1;
}

// ================================================================ sites
export async function particlesSites(verbose = process.argv.includes('--verbose')) {
  console.log('\n=== particles-sites: each effect site queues what it says (GPU on, a Particles attached to the harness engine) — impact by material, casings + the tink, muzzle powder, the bang, the kick, the clack, the ember; the CPU spark pool idle throughout ===');
  const { st, verdict } = mkVerdict();
  const { W, game, engine, P, flush } = await stage(8101, true);
  const pl = game.player; const audio: any[] = []; W.audioLog = audio;
  const seenEv = new Set<any>(); const evKinds = () => { const out: string[] = []; for (const e of game.events) { if (seenEv.has(e)) continue; seenEv.add(e); out.push(e.kind); } return out; };
  { // (a) a Five-seveN round into the plaster wall
    flush(); evKinds(); audio.length = 0;
    const shot = shotAt(game, WALL, () => fireWeapon(game, pl.char, WALL, true, { weapon: WEAPONS.fiveseven }), flush);
    const q = shot.q, c = shot.all; const cpuLive0 = game.sparks.live; W.step(DT); const cpuLive1 = game.sparks.live;
    const cas = q.find((e: any) => e.kind === 'casing'); const tinks = pendingTinks(game).length;
    const fails: string[] = [];
    { const bad = recipeAt(shot, 10, 6); if (bad) fails.push(`impact, the drywall recipe — ${bad}`); }
    if (c.spark !== fxm.shot.sparkCount) fails.push(`muzzle powder: ${c.spark} GPU sparks (want shot.sparkCount ${fxm.shot.sparkCount}, no impact sparks off plaster)`);
    if (c.casing !== 1 || !cas || cas.owner !== PLAYER_ID) fails.push(`casing: ${c.casing} (owner ${cas?.owner}) — want 1 owned by the player`);
    if (cpuLive0 !== 0 || cpuLive1 !== 0) fails.push(`CPU spark pool live ${cpuLive0}/${cpuLive1} (want 0: the powder went to the GPU)`);
    if (tinks !== 1) fails.push(`tinks pending ${tinks} (want 1)`);
    // the tink lands ~0.5 s on, once, as audio only
    let tinkAt = -1, tinkN = 0; const ev: string[] = [];
    for (let f = 0; f < 60; f++) { W.step(DT); flush(); ev.push(...evKinds()); for (const a of audio) if (a.name === 'casingTink') { tinkN++; if (tinkAt < 0) tinkAt = f * DT; } audio.length = 0; }
    if (tinkN !== 1 || !(tinkAt > 0.4 && tinkAt < 0.62)) fails.push(`casingTink played ${tinkN}× at ${tinkAt.toFixed(2)} s (want once, ≈ 0.5 s)`);
    if (ev.some(k => k !== 'shot')) fails.push(`hearing events after the shot: ${ev.join(',')} (want none but the shot's own)`);
    if (verbose) console.log(q);
  }
  { // (b) an FN2000 round: rifle scaling + its own case
    flush();
    const shot = shotAt(game, WALL, () => fireWeapon(game, pl.char, WALL, true, { weapon: WEAPONS.fn2000 }), flush);
    const q = shot.q, c = shot.all; const cas = q.find((e: any) => e.kind === 'casing');
    const fails: string[] = [];
    { const bad = recipeAt(shot, Math.round(10 * PFX.impactScaleRifle), Math.round(6 * PFX.impactScaleRifle)); if (bad) fails.push(bad); }
    if (!cas || c.casing !== 1) fails.push('no case');
    else { const port = cas.pos, mz = pl.char.muzzle; const d = Math.hypot(port[0] - mz[0], port[1] - mz[1], port[2] - mz[2]); if (!(d < 0.3)) fails.push(`FN2000 case born ${d.toFixed(2)} m from the muzzle (forward ejection: want < 0.3)`); }
    for (let f = 0; f < 40; f++) { W.step(DT); flush(); }
    verdict(fails.length === 0, 'FN2000 into the same wall: the drywall counts × impactScaleRifle, and its case leaves by the muzzle (forward ejection), not the wrist', fails.length ? fails.join('; ') : `at the strike ${JSON.stringify(shot.site)}${shot.through ? ' (it went on through the wall)' : ''} · the whole queue ${JSON.stringify(c)}, case ${JSON.stringify(cas?.pos)} vs muzzle ${JSON.stringify(pl.char.muzzle.map((x: number) => +x.toFixed(2)))}`);
  }
  { // (c) a round into a server rack (black → metal → sparks)
    game.teleportPlayer([18.6, 0, 9.2]); pl.char.bodyYaw = pl.char.aimYaw = Math.PI; W.step(DT); W.step(DT); flush();
    const rack: Vec3 = [18.6, 1.1, 7.25];
    fireWeapon(game, pl.char, rack, true, { weapon: WEAPONS.fiveseven });
    const q = flush(); const c = countKinds(q);
    const st0 = game.col.raycast(pl.char.muzzle, v3.normalize(v3.sub(rack, pl.char.muzzle)), 10); const alb = st0 && st0.index < 1e6 ? game.col.boxes[st0.index].albedo : null;
    const fails: string[] = [];
    if (!(c.spark >= fxm.shot.sparkCount + 14)) fails.push(`sparks ${c.spark} (want powder ${fxm.shot.sparkCount} + ≥ 14 off the metal; hit albedo ${JSON.stringify(alb)})`);
    if (c.dust !== 0) fails.push(`dust ${c.dust} off metal (want 0)`);
    for (let f = 0; f < 40; f++) { W.step(DT); flush(); }
    verdict(fails.length === 0, 'a round into a server rack front (palette black → metal): impact sparks on top of the powder, a couple of dark chips, no dust', fails.length ? fails.join('; ') : `emits ${JSON.stringify(c)}; hit albedo ${JSON.stringify(alb)}`);
  }
  { // (d) the stun charge
    flush(); game.sparks.clear();
    const it = game.items.spawn('flash', [20, 0.25, 11.2], [0, 0, 0], { fuse: 9, heading: 0 });
    detonateStun(game, it);
    const q = flush(); const c = countKinds(q); const cpu = game.sparks.live === 0 ? (() => { game.sparks.update(0, null); return game.sparks.live; })() : game.sparks.live;
    let cpuAlive = 0; (game.sparks as any).pool.forEach((s: any) => { if (s.alive) cpuAlive++; });
    const wantCpu = Math.min(fxm.bang.sparkCount, fxm.flags.trails ? fxm.bang.trailCount : 0);
    const fails: string[] = [];
    if (c.spark !== PFX.bangSparks) fails.push(`GPU sparks ${c.spark} (want bangSparks ${PFX.bangSparks})`);
    if (c.dust !== PFX.bangRing + PFX.bangPuff) fails.push(`GPU dust ${c.dust} (want ring ${PFX.bangRing} + puff ${PFX.bangPuff})`);
    if (cpuAlive !== wantCpu) fails.push(`CPU sparks alive ${cpuAlive} (want ${wantCpu}: only the trail carriers)`);
    if (!q.some((e: any) => e.kind === 'dust' && e.shape === 'ring')) fails.push('no ring-shaped dust emit');
    for (let f = 0; f < 30; f++) { W.step(DT); flush(); }
    verdict(fails.length === 0, `the stun charge: ${PFX.bangSparks} fragments + a ${PFX.bangRing}-puff dust ring + ${PFX.bangPuff} over the can on the GPU; the CPU pool flies exactly the ${wantCpu} that carry smoke trails`, fails.length ? fails.join('; ') : `emits ${JSON.stringify(c)}, cpu ${cpuAlive} (${cpu} live)`);
  }
  { // (e) a kicked door, (f) the ring's clack, (g) a tracer ember through the sink
    flush();
    const d = game.doors.list.find((x: any) => !x.def.steel && !x.def.exterior) ?? game.doors.list[0];
    game.doors.kickIn(d, [d.frameCentre[0], 0, d.frameCentre[2] + 1], PLAYER_ID);
    const qk = flush(); const ck = countKinds(qk);
    // clack: a ring straight into the wall beside him
    const { fireRing } = await import(`${ROOT}/src/game/launcher.ts`);
    game.teleportPlayer([21.5, 0, 11.5]); W.step(DT); flush();
    fireRing(game, pl.char, [21.5, 1.3, 11.0], [21.5, 1.3, 9.0]);
    let cs = 0; for (let f = 0; f < 30; f++) { W.step(DT); cs += countKinds(flush()).spark; }
    // ember: the tracers' sink with GPU on
    const before = game.sparks.live; (game.tracers as any).sparks.emitCone([21, 1, 11], [0, 0, 1], { count: 3, speed: 5, cone: 0.9, life: 0.3 }, 1); const qe = flush(); game.sparks.update(DT, null);
    const fails: string[] = [];
    const lockBits = 'lockSplinters' in PFX ? PFX.lockSplinters + PFX.lockSplintersBack + PFX.lockScrews : 0;   // (the wind lane's fixture deaths: the torn keep's splinters ride the same 'kick' event — game.ts onDoorEvent → lockSplinterFx)
    if (ck.dust !== PFX.kickDust + 5 || ck.debris !== PFX.kickSplinters + lockBits) fails.push(`kick: dust ${ck.dust} debris ${ck.debris} (want ${PFX.kickDust + 5} / ${PFX.kickSplinters} + ${lockBits} off the keep)`);
    if (cs !== PFX.clackSparks) fails.push(`clack sparks ${cs} within 0.5 s (want ${PFX.clackSparks}: one clack off the wall)`);
    if (countKinds(qe).spark !== 3 || game.sparks.live !== before) fails.push(`ember: GPU ${countKinds(qe).spark}, CPU pool ${before} → ${game.sparks.live} (want 3 on the GPU, pool untouched)`);
    verdict(fails.length === 0, `door '${d.def.name}' kicked in: ${PFX.kickDust} + 5 puffs and ${PFX.kickSplinters} splinters at the jamb; a ring into the wall: ${PFX.clackSparks} sparks off the clack; a tracer ember: its 3 sparks on the GPU and the CPU pool untouched`, fails.length ? fails.join('; ') : `kick ${JSON.stringify(ck)}, clack ${cs}, ember ${countKinds(qe).spark}`);
  }
  { // stats sanity: nothing dropped through all that
    const s = P.stats; const want = fxm.shot.sparkCount * 3 + PFX.bangSparks + PFX.bangRing; const ok = s.droppedRecs === 0 && s.droppedBirths === 0 && s.emitted > want;
    verdict(ok, `through all of it: no record or birth dropped, > ${want} particles emitted (three shots' powder + the bang at least)`, `emitted ${s.emitted}, dropped ${s.droppedRecs} rec / ${s.droppedBirths} births`);
  }
  console.log(`=== particles-sites: ${st.all ? 'ALL PASS' : 'FAILURES above'} ===`);
  if (!st.all) process.exitCode = 1;
}


