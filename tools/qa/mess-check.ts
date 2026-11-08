// WHAT A BLAST AND A BROKEN THING LEAVE ON THE FLOOR (src/game/floormess.ts, destruction.ts CHAOS.mess — the maintainer, 10-14: "maybe we've just blown up too much
// stuff but the office floor looks too clean"). Headless: no picture here, so this check reads the MARKS the game lays — which, where, how wide, tagged as
// laid only for the picture — and nothing of how they look: that is the page (never seen on a GPU).
//   bun run tools/qa/mess-check.ts            a hand grenade and a 40 mm on the block's floor (the films, where they lie, that every one is a picture-only
//                                             mark); hard by a wall that stands no film reaches across it; a wall charge films both sides of the wall
//                                             it opened and one side of a whole wall; a second burst on the same spot lays no second film; a wall chewed
//                                             open by rifle fire leaves one at its foot; a chunk the pool takes leaves its mark where it lay; a ring
//                                             full of play's marks takes none and loses none; nothing at chaos 0 or under a show; the panel's reset
//   MESS_BREAK=1 …                            every floor stood at chaos 0, where no mess is laid: the lines that want one must fail
// 2026-11-07: the mess, the pall and the destruction switch are settled rules, and a write of the other side of any is refused. Gone with that: 'the stand';
// every 'without it none' half (the second floor with the rule off was the same floor); 'the one switch off'; the digest with the rule on and off; the
// recording from before the key and the page's ?mess=0. The counters are read as differences since the floor stood (they are the game's own and run on).
// STILL BY HAND: the ring is filled with play's marks by `decals.add` on the tools' copy of the game, which the Rust game never gets.
import { ROOT } from './headless.ts';
import { floor, wallSpot, charge, burst, openAt, thickOf, offOf, type Floor } from './pall-check.ts';
const { CHAOS, resetChaosKnobs } = await import(`${ROOT}/src/game/destruction.ts`);
const { MESS_STATS, MESS_LOOK, messWall } = await import(`${ROOT}/src/game/floormess.ts`);
const { LOOK_DECALS } = await import(`${ROOT}/src/game/lookDecals.ts`);
const { wallPt } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { DECAL_CAP } = await import(`${ROOT}/src/render/decals.ts`);
type Vec3 = [number, number, number];
const PLAYER_ID = 1, f2 = (x: number) => x.toFixed(2);
const BREAK = process.env.MESS_BREAK === '1';
/** the mess's counters now, and what they have gone up by since a reading (the game's own: they run on from floor to floor) */
const stats = () => ({ ...MESS_STATS }) as Record<string, number>;
const since = (a: Record<string, number>) => { const b = stats(), d: Record<string, number> = {}; for (const k of Object.keys(b)) d[k] = b[k] - (a[k] ?? 0); return d; };
const lookOf = (d: any) => d.kind === 'damage' ? Math.floor(d.param) : -1;
const films = (g: any): any[] => g.decals.list.filter((d: any) => lookOf(d) === MESS_LOOK.film);
const rubble = (g: any): any[] => g.decals.list.filter((d: any) => lookOf(d) === MESS_LOOK.grit || lookOf(d) === MESS_LOOK.splinter);
const picture = (ds: any[]) => ds.every(d => d.look === true && LOOK_DECALS.has(d));
const play = (g: any): number => g.decals.list.filter((d: any) => !LOOK_DECALS.has(d)).length;
/** the block's floor (pall-check's) and the mess's counters as it stood */
async function stand(opts: { chaos?: number } = {}): Promise<Floor & { s0: Record<string, number> }> { const S = await floor({ chaos: BREAK ? 0 : opts.chaos }); return { ...S, s0: stats() }; }
/** a wall of this floor no blast opens (not on the walls' own list), a clear lane off one face of it */
function hardWall(g: any): any {
  const soft = new Set(g.destr.walls.all().map((B: any) => B.id));
  for (let j = 0; j < g.col.boxes.length; j++) { const b = g.col.boxes[j]; if (soft.has(j) || (b.flags & 1) || b.h[1] < 1.2 || Math.min(b.h[0], b.h[2]) > 0.2 || Math.max(b.h[0], b.h[2]) < 1.5) continue;
    const ax = b.h[0] >= b.h[2];
    for (const side of [1, -1] as const) { const w = { index: j, box: b, alongX: ax, u: ax ? b.c[0] : b.c[2], side };
      if (!g.col.solidBlocked(wallPt(w, 0, 3.5, 1.7), wallPt(w, 0, 0.05, 1.7)) && !g.col.solidBlocked(wallPt(w, 0, 3.5, 0.3), wallPt(w, 0, 0.05, 0.3)) && g.col.solidBlocked(wallPt(w, 0, 0.3, 1.7), wallPt(w, 0, -thickOf(w) - 0.3, 1.7))) return w; } }
  return null;
}
/** of the floor films lying abreast of the wall (inside its own length): how many reach past its face, and the nearest any comes to it in its own reach */
function across(w: any, F: any[], under: number) {
  const off = offOf(w), half = w.alongX ? w.box.h[0] : w.box.h[2], along = (d: any) => Math.abs((w.alongX ? d.pos[0] - w.box.c[0] : d.pos[2] - w.box.c[2]));
  const abreast = F.filter(d => d.pos[1] < 0.1 && along(d) <= half);
  return { n: abreast.length, beyond: abreast.filter(d => off(d) <= 0).length, over: abreast.filter(d => off(d) > 0 && d.radius > off(d) + under + 0.02).length };
}

/** the films (floor ones) that reach THROUGH something standing: 24 looks out from each one's middle to just inside its own reach, at the foot and at the head
 *  of the band dust's way is read in — a film fitted to its room meets nothing on any of them */
function through(g: any, F: any[], K: any): number {
  let bad = 0;
  for (const d of F) { if (d.pos[1] > 0.05) continue; const r = d.radius - K.under - 0.03; let hit = false;
    for (let i = 0; i < 24 && !hit; i++) { const a = i * Math.PI / 12, px = d.pos[0] + Math.sin(a) * r, pz = d.pos[2] + Math.cos(a) * r;
      for (const y of [K.standY + 0.02, K.clearY - 0.02]) if (g.col.segmentBlocked([d.pos[0], y, d.pos[2]], [px, y, pz], 0, 0)) hit = true; }
    if (hit) bad++; }
  return bad;
}
/** a static with a flat top a film may lie on (a desk, a counter, a crate), open above: the first by the level's own order */
function topSpot(g: any, K: any): any {
  for (let j = 0; j < g.col.boxes.length; j++) { const b = g.col.boxes[j], y = b.c[1] + b.h[1];
    if (b.rot || (b.flags & 3) || b.emissive[0] + b.emissive[1] + b.emissive[2] > 0 || y < 0.4 || y > 1.2 || Math.min(b.h[0], b.h[2]) < 0.25) continue;
    if (g.col.solidBlocked([b.c[0], y + 0.05, b.c[2]], [b.c[0], K.clearY, b.c[2]])) continue;
    return { index: j, box: b, y }; }
  return null;
}

export async function messCheck(): Promise<boolean> {
  console.log(`\n=== mess: a blast and a broken thing leave the floor a mess${BREAK ? ' — MESS_BREAK: every floor at chaos 0' : ''} ===`);
  let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const rnd0 = Math.random, core0 = CHAOS.mess.burst.core;
  const log = console.log, quiet = async <T>(fn: () => Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  try {
    const K = CHAOS.mess;
    // A HAND GRENADE on open floor
    { const A = await quiet(() => stand()), gA = A.g, at = openAt(gA), p0 = play(gA); await quiet(async () => { burst(A, 'frag'); });
      const F = films(gA), st = since(A.s0), far = Math.max(0, ...F.map(d => Math.hypot(d.pos[0] - at[0], d.pos[2] - at[2]))), low = F.filter(d => d.pos[1] < 0.05), tops = F.filter(d => d.pos[1] >= K.top.lo && d.pos[1] <= K.top.hi);
      verdict(low.length >= 3 && low.length + tops.length === F.length && picture(F) && far <= (K.burst.core * 0.5 + K.burst.reach * 1.25) + 0.05 && F.every(d => d.radius >= K.minRadius && d.radius <= K.burst.core + 1e-6) && through(gA, F, K) === 0 && st.films === low.length && st.tops === tops.length,
        'a hand grenade on open floor: a heart and its lobes of settled dust lie round the burst, every one a mark laid only for the picture',
        `${low.length} on the floor, ${tops.length} on tops, the farthest ${f2(far)} m off, reach ${f2(Math.min(9, ...F.map(d => d.radius)))}‥${f2(Math.max(0, ...F.map(d => d.radius)))} m, picture-only ${picture(F)}, through something standing ${through(gA, F, K)}, play's marks ${p0} → ${play(gA)} · counters since the floor stood ${JSON.stringify(st)}`);
      // …and a second burst on the same spot lays no second film over the first
      const n1 = low.length, c1 = MESS_STATS.crowded; await quiet(async () => { burst(A, 'frag'); }); const n2 = films(gA).filter(d => d.pos[1] < 0.1).length;
      verdict(n1 > 0 && n2 === n1 && MESS_STATS.crowded > c1, 'a second grenade on the same spot lays no second film: the dust that lies there is the dust that lies there', `floor films ${n1} → ${n2} · left out as already there ${MESS_STATS.crowded - c1}`); }
    // A 40 MM (the frag's blast by its own radii) on open floor
    { const S = await quiet(() => stand()); await quiet(async () => { burst(S, '40 mm'); }); const F = films(S.g).filter(d => d.pos[1] < 0.1);
      verdict(F.length >= 3 && picture(F) && Math.max(0, ...F.map(d => d.radius)) <= K.burst.core * (2.2 / 2.5) + 0.01, 'a 40 mm on open floor leaves its own, a size under the hand grenade\'s', `${F.length} on the floor, the widest ${f2(Math.max(0, ...F.map(d => d.radius)))} m`); }
    // HARD BY A WALL THAT STANDS: a grenade lying against it, a 40 mm burst a hand off its face — no film across it
    { const S = await quiet(() => stand()), g = S.g, w = hardWall(g); if (w) await quiet(async () => { burst(S, 'frag', wallPt(w, 0, 0.06, 0.1)); });
      const a = w ? across(w, films(g), K.under) : { n: 0, beyond: 1, over: 1 };
      const S2 = await quiet(() => stand()), g2 = S2.g, w2 = hardWall(g2); if (w2) await quiet(async () => { burst(S2, '40 mm', wallPt(w2, 0, 0.15, 1.3)); });
      const b = w2 ? across(w2, films(g2), K.under) : { n: 0, beyond: 1, over: 1 };
      const t1 = through(g, films(g), K), t2 = through(g2, films(g2), K);
      verdict(!!w && a.n >= 2 && a.beyond === 0 && a.over === 0 && b.n >= 2 && b.beyond === 0 && b.over === 0 && t1 === 0 && t2 === 0, 'hard by a wall that stands: the films lie in the burst\'s own room, none beyond the wall, none reaching under it, none through anything standing',
        `wall #${w?.index} · grenade: ${a.n} abreast of it, ${a.beyond} beyond, ${a.over} reaching past its face · 40 mm: ${b.n}, ${b.beyond}, ${b.over} · through something standing ${t1} / ${t2} · left out ${MESS_STATS.shut}`); }
    // A WALL CHARGE: both sides of the wall it opened; and on a wall that IS whole (the entry point asked directly) the charge's side alone, none reaching across
    { const S = await quiet(() => stand()), g = S.g, w = wallSpot(g), C = await quiet(async () => charge(S)); const F = films(g).filter(d => d.pos[1] < 0.1), off = w ? offOf(w) : () => 0, T = w ? thickOf(w) : 0;
      const near = F.filter(d => off(d) > 0).length, far = F.filter(d => off(d) < -T).length;
      const H = await quiet(() => stand()), gh = H.g, wh = wallSpot(gh); if (wh) messWall(gh, wallPt(wh, 0, 0, 0), wh.alongX ? [0, 0, wh.side] : [wh.side, 0, 0], thickOf(wh), false);
      const h = wh ? across(wh, films(gh), K.under) : { n: 0, beyond: 1, over: 1 };
      const tc = through(g, films(g), K), th = through(gh, films(gh), K);
      verdict(!!C && C.hole && near >= 2 && far >= 2 && picture(F) && h.n >= 2 && h.beyond === 0 && h.over === 0 && tc === 0 && th === 0, 'a wall charge that opens its wall leaves a film on both sides of it; on a whole wall, the charge\'s side alone and none reaching under the wall',
        `opened: hole ${C?.hole}, ${near} on the charge's side, ${far} beyond · a whole wall: ${h.n} abreast, ${h.beyond} beyond, ${h.over} reaching past its face · through something standing ${tc} / ${th}`); }
    // A WALL CHEWED OPEN BY FIRE leaves a small film at its foot
    { const S = await quiet(() => stand()), g = S.g, w = wallSpot(g), walls = g.destr.walls, rifle = WEAPONS.fn2000 ?? Object.values(WEAPONS)[0]; let shots = 0, open = 0;
      if (w) { const face = wallPt(w, 0, 0.02, 1.05), dir: Vec3 = w.alongX ? [0, 0, -w.side] : [-w.side, 0, 0];
        await quiet(async () => { for (; shots < 600 && !(open = walls.breakableAt(w.index)?.open ?? 0); shots++) walls.round(PLAYER_ID, [face[0], 0.4 + (shots % 8) * 0.2, face[2]], dir, w.index, rifle, 0.5); S.step(30); }); }
      const F = films(g), face = w ? wallPt(w, 0, 0, 0) : [0, 0, 0], nearFoot = F.filter(d => Math.hypot(d.pos[0] - face[0], d.pos[2] - face[2]) < 1.6).length;
      verdict(!!w && open > 0 && nearFoot >= 1 && picture(F) && F.every(d => d.radius <= K.foot.radius + K.foot.perMetre * 6), 'a wall that gives under rifle fire leaves a small film of its own dust at its foot', `wall #${w?.index}: ${shots} rounds, ${open} cells open · ${nearFoot} films at its foot of ${F.length}, the widest ${f2(Math.max(0, ...F.map(d => d.radius)))} m`); }
    // THE TOPS STANDING IN A FILM take their own: a burst over a desk-high static; and six bursts and a charge pack into the decal buffer with nothing dropped
    { const S = await quiet(() => stand()), g = S.g, T = topSpot(g, K); if (T) await quiet(async () => { g.destr.blast([T.box.c[0], 0.2, T.box.c[2] + T.box.h[2] + 0.6], { who: PLAYER_ID, radius: 2.5 }); });
      const F = films(g), tops = F.filter(d => d.pos[1] > 0.05);
      // each film on a top lies at the height of a static that still stands inside its reach (the burst may have thrown the nearest one: no film where it stood)
      const real = tops.filter(d => g.col.boxes.some((b: any) => !(b.flags & 3) && !b.rot && Math.abs(b.c[1] + b.h[1] - d.pos[1]) < 0.03 && Math.hypot(b.c[0] - d.pos[0], b.c[2] - d.pos[2]) - Math.max(b.h[0], b.h[2]) <= d.radius)).length;
      verdict(!!T && tops.length >= 1 && real === tops.length && picture(tops) && tops.every(d => d.pos[1] >= K.top.lo && d.pos[1] <= K.top.hi) && since(S.s0).tops === tops.length, 'a burst beside a desk-high top: the tops standing in its films take films of their own, each at the height of a top that stands there',
        `beside static #${T?.index} (top at ${f2(T?.y ?? 0)} m): ${tops.length} on tops (at ${[...new Set(tops.map(d => f2(d.pos[1])))].join(', ')} m), ${real} at a standing top's height, ${F.length - tops.length} on the floor`);
      const S2 = await quiet(() => stand()), g2 = S2.g, at = openAt(g2); await quiet(async () => { for (let k = 0; k < 6; k++) g2.destr.blast([at[0] + (k % 3) * 1.4, 0.2, at[2] + Math.floor(k / 3) * 1.4], { who: PLAYER_ID, radius: 2.5 }); charge(S2); });
      g2.decals.build(); const st = g2.decals.stats, n = films(g2).length;
      verdict(n >= 8 && st.dropped === 0, 'six bursts and a wall charge in one room pack into the decal buffer with nothing dropped', `${n} films of ${st.count} marks drawn · ${st.listLen} list entries, the busiest square ${st.maxCell}, dropped ${st.dropped} · left out as already there ${MESS_STATS.crowded}`); }
    // A RING FULL OF PLAY'S OWN MARKS takes none of it and loses none
    { const S = await quiet(() => stand()), g = S.g, p = g.player.char.pos; for (let k = 0; g.decals.list.length < DECAL_CAP && k < DECAL_CAP * 2; k++) g.decals.add('bullet', [p[0] + 6 + (k % 32) * 0.1, 0, p[2] + 6 + Math.floor(k / 32) * 0.1], [0, 1, 0], 0.03);
      const n0 = play(g), len0 = g.decals.list.length; await quiet(async () => { g.destr.blast(openAt(g), { who: PLAYER_ID, radius: 2.5 }); });
      verdict(len0 === DECAL_CAP && n0 === DECAL_CAP && films(g).length === 0 && play(g) === DECAL_CAP && since(S.s0).full > 0, 'a ring full of the marks play laid takes no film and gives none of its own up for one', `${n0} of play's marks before, ${play(g)} after · films ${films(g).length} · turned away ${since(S.s0).full}`); }
    // NOTHING at chaos 0, nor under a show; the panel's reset
    { const S = await quiet(() => stand({ chaos: 0 })); await quiet(async () => { burst(S, 'frag'); }); const a = films(S.g).length;
      const S3 = await quiet(() => stand()); S3.g.destr.showChaos = 0.8; await quiet(async () => { burst(S3, 'frag'); }); const c = films(S3.g).length;
      verdict(a === 0 && c === 0, 'chaos 0, or a level script\'s show running on its own chaos (its cameras keep the picture they were set on): no mess', `chaos 0: ${a} · a show at 0.8: ${c}`);
      const c0 = CHAOS.mess.burst.core; CHAOS.mess.burst.core = 9; resetChaosKnobs(S3.g); const kept = [CHAOS.mess.on, CHAOS.mess.burst.core === c0];
      verdict(kept[0] === true && kept[1] === true, 'the panel\'s ↺ (resetChaosKnobs) puts the mess\'s numbers back and leaves its switch on', `on, the numbers back: ${kept}`); }
  } finally { console.log = log; CHAOS.mess.burst.core = core0; Math.random = rnd0; }
  console.log(`=== mess: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await messCheck()) ? 0 : 1);
