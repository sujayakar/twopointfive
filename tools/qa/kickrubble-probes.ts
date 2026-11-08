// The 'kickrubble' probe family — MEN KICK RUBBLE ASIDE AS THEY WALK (src/game/debris.ts feet(), behind destructionSwitch.ts RUBBLE.on), run through probes.ts:
//   bun run tools/qa/probes.ts kickrubble | kickrubble-walk | kickrubble-quiet | kickrubble-offman | kickrubble-wall | kickrubble-left | kickrubble-edge | kickrubble-pool [--verbose]
// Headless = the Proxy device; what is asserted is every CPU-side promise, in the office's corridor with one of the shift walking it under orders:
//   (walk)    the pieces lying in his path are shoved aside as he walks through and come to rest again; a piece off his path keeps its pose to the bit; the same
//             walk twice lays the same litter (deterministic); Sam, driven by the keys, shoves them too;
//   (quiet)   the shove is SILENT to the simulation: no hearing event of any kind is filed, no landing is counted, the break table is told nothing (nothing snaps
//             underfoot), no furniture is pushed, the impulses' hash stream is not advanced, no piece is spawned — and the game's digest after the walk equals
//             the digest of the same walk with the switch off (the pieces are not in the digest: what it says is that nothing ELSE moved);
//   (offman)  a plank that came to rest through a man standing still is pushed off him, and he has not moved;
//   (wall)    a piece pinned between a standing man and a wall is never pushed through the wall, and is given up on (bounded tries);
//   (left)    what a boot leaves alone: a piece lying on a desk top beside him (standing in reach of it, and walking past it), a slab too heavy to shift, a glass
//             plate; and a slider never leaves the floor it lies on — pushed toward the edge of a desk top (the floor rule widened for the test) it stops on it;
//   (edge)    the fragments a blown wall sheds at its foot (game/edgeLook.ts — picture only, not pool pieces) are shoved by the same feet: a walker through
//             them moves them, they come to rest on the floor outside every wall, nothing is heard, the pool is not touched — its shove count neither (the heap's kicks
//             are on the look's own books); kicked flakes that overlap do not end with their tops in one plane; a shadow caster slid to a beam casts none; Sam
//             standing in them parts them;
//   (pool)    a piece sliding off a boot is still evictable and is not 'moving' for the mayhem rule; a blast's impulse takes it over (it flies, lands and sounds).
// 2026-11-07: the probe 'off' (the switch off: the same walk moves nothing) is deleted, a settled rule's off side. STILL TO RESTAGE: the family lays its litter
// with RUBBLE.on written off and raises it for the walk, and 'quiet' lays its walk beside the switch-off walk. Those writes are refused now (the rule is
// settled on), so the pieces are laid under live feet. Not restaged here: on the Rust game no piece is made headless and every probe throws before a line.
import { standUp, ROOT } from './headless.ts';

const { RUBBLE, DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { makeBox } = await import(`${ROOT}/src/scene/boxes.ts`);
const { order } = await import(`${ROOT}/src/game/squad.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => x.toFixed(2);
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { const ok = all; console.log(`=== ${title.split(':')[0]}: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`); if (!ok) process.exitCode = 1; return ok; } };
}
async function quiet<T>(fn: () => Promise<T> | T): Promise<T> { if (VERBOSE) return await fn(); const l = console.log, w = console.warn, e = console.error; console.log = console.warn = console.error = () => {}; try { return await fn(); } finally { console.log = l; console.warn = w; console.error = e; } }

/** the office's corridor: Kowalski stands at its west end; the litter is laid east of him along his line */
interface Stage { W: any; game: any; D: any; gd: any; p0: Vec3; dir: [number, number]; free: number; events: any[]; seen: Set<any>; kicked: number; landedHook: number }
async function stage(on: boolean, seed = 7107): Promise<Stage> {
  Math.random = seeded(seed); RUBBLE.on = on;
  const W: any = await quiet(() => standUp({ uploadEvery: 1, dress: true })); const game = W.game; game.godMode = true; game.playerInvisible = true; game.aiEnabled = true;
  const gd = game.guards[0]; gd.hold = true; const p0: Vec3 = [gd.char.pos[0], 0, gd.char.pos[2]];
  let dir: [number, number] = [1, 0], free = 0; for (const d of [[1, 0], [-1, 0], [0, 1], [0, -1]] as [number, number][]) { const h = game.col.raycast([p0[0], 0.5, p0[2]], [d[0], 0, d[1]], 12); const t = h ? h.t : 12; if (t > free) { free = t; dir = d; } }
  const D = game.destr.debris; const S: Stage = { W, game, D, gd, p0, dir, free, events: [], seen: new Set(game.events), kicked: 0, landedHook: 0 };
  // spies: every hearing event filed from here on, and every word to the break table
  const k0 = D.breaks.kicked.bind(D.breaks), l0 = D.breaks.landed.bind(D.breaks);
  D.breaks.kicked = (c: any, k: number) => { S.kicked++; return k0(c, k); }; D.breaks.landed = (c: any, sp: number) => { S.landedHook++; return l0(c, sp); };
  return S;
}
// (the game replaces `events` with a filtered copy every frame and an event lives half a second: every OBJECT not seen before is a new one)
const run = (S: Stage, n: number) => { for (let f = 0; f < n; f++) { S.W.step(DT); for (const e of S.game.events) if (!S.seen.has(e)) { S.seen.add(e); S.events.push(e); } } };
/** a point `t` m down his line and `off` m to its left */
const at = (S: Stage, t: number, off: number, y = 0.25): Vec3 => [S.p0[0] + S.dir[0] * t - S.dir[1] * off, y, S.p0[2] + S.dir[1] * t + S.dir[0] * off];
const piece = (S: Stage, c: Vec3, h: Vec3, yaw: number) => S.D.spawnRaw('chunk', makeBox({ c, h, yaw, albedo: [0.7, 0.7, 0.68] }), [0, 0, 0], [0, 0, 0]);
const PLANK: Vec3 = [0.3, 0.012, 0.06], LUMP: Vec3 = [0.09, 0.05, 0.07];
/** six pieces in his path (planks and lumps, a little either side of his line) and one well off it, let fall and settle while he is held */
function litter(S: Stage, far = 0.95): { path: any[]; aside: any } {
  const path: any[] = []; for (let i = 0; i < 6; i++) path.push(piece(S, at(S, 1.5 + i * 0.5, (i % 3 - 1) * 0.18), i % 2 ? PLANK : LUMP, i * 0.7));
  const aside = piece(S, at(S, 2.75, far), LUMP, 0.4); run(S, 120); return { path, aside };
}
const pose = (c: any) => `${c.box.c.map(String).join(',')}|${(c.box.rot ?? [c.box.yaw]).map(String).join(',')}`;   // (every digit: 'to the bit')
const walk = (S: Stage, frames = 600) => { S.gd.hold = false; order(S.gd, { goal: at(S, Math.min(S.free - 0.6, 5.5), 0, 0) as any }); run(S, frames); };
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** how far a piece's middle lies from his line of march (the corridor's axis through p0) */
const offLine = (S: Stage, c: any) => Math.abs(-(c.box.c[0] - S.p0[0]) * S.dir[1] + (c.box.c[2] - S.p0[2]) * S.dir[0]);

async function walkOnce(on: boolean) {
  const S = await stage(on); const L = litter(S); const start = L.path.map(c => [...c.box.c] as Vec3), aside0 = pose(L.aside), settled = L.path.every(c => c.rest) && L.aside.rest;
  const spawned0 = S.D.stats.spawned, landed0 = S.D.stats.landed, imp0 = S.D.impulses, props0 = S.game.props.props.map((p: any) => `${p.x},${p.z},${p.yaw}`).join(' '); S.events.length = 0; S.kicked = 0; S.landedHook = 0;
  const g0: Vec3 = [...S.gd.char.pos] as Vec3; walk(S);
  return { S, L, start, aside0, settled, spawned0, landed0, imp0, props0, walked: flat(g0, S.gd.char.pos) };
}

/** A man walking the corridor shoves the pieces in his path aside; a piece off it is not touched; the same walk twice lays the same litter; Sam's feet do it too. */
export async function kickWalk(): Promise<boolean> {
  const Fm = family('kickrubble-walk: a walking man shoves the pieces in his path aside');
  const A = await walkOnce(true); const { S, L } = A; const moved = L.path.map((c, i) => flat(c.box.c, A.start[i]));
  Fm.verdict(A.settled && A.walked > 4, 'the stage: seven pieces at rest, and he walks the corridor through them', `settled ${A.settled} · he walked ${f2(A.walked)} m at ≤ ${f2(S.game.tune.guardWalk ?? 0)} m/s`);
  Fm.verdict(moved.every(d => d > 0.08) && Math.max(...moved) < 2.5, 'every piece in his path was shoved (and none was thrown)', `moved ${moved.map(f2).join(' · ')} m · shoves: ${JSON.stringify(S.D.feetStats)}`);
  Fm.verdict(L.path.every(c => c.rest && !c.foot), 'they all lie at rest again', `resting ${L.path.filter(c => c.rest).length}/6 · still flagged as a boot's ${L.path.filter(c => c.foot).length}`);
  const off = L.path.map(c => offLine(S, c)); const mean = off.reduce((a, b) => a + b, 0) / off.length;
  Fm.verdict(mean > 0.3 && off.every(d => d > 0.1), 'they were swept ASIDE, not dribbled down the corridor', `off his line by ${off.map(f2).join(' · ')} m, mean ${f2(mean)} (they were laid 0 or 0.18 m off it: mean 0.12)`);
  Fm.verdict(pose(L.aside) === A.aside0, 'the piece 0.95 m off his line was not touched', `pose ${pose(L.aside) === A.aside0 ? 'bit-equal' : 'CHANGED'}`);
  Fm.verdict(S.D.list.every((c: any) => [...c.box.c, ...c.v, ...c.w].every(Number.isFinite)) && L.path.every(c => c.box.c[1] > 0 && c.box.c[1] < 0.2), 'nothing NaN, nothing sunk or left hanging', `heights ${L.path.map(c => c.box.c[1].toFixed(3)).join(' · ')}`);
  const B = await walkOnce(true);
  Fm.verdict(L.path.map(pose).join(' ') === B.L.path.map(pose).join(' '), 'the same walk twice lays the same litter', `second run: ${JSON.stringify(B.S.D.feetStats)}`);
  { // Sam, on the keys, walking east through three pieces
    const P = await stage(true); const g = P.game; const from = at(P, 0.8, 0.45, 0); g.teleportPlayer(from); const pc = [piece(P, at(P, 1.8, 0.45), LUMP, 0.2), piece(P, at(P, 2.4, 0.5), PLANK, 1.1), piece(P, at(P, 3.0, 0.4), LUMP, 0.9)]; run(P, 120);
    const s0 = pc.map(c => [...c.box.c] as Vec3); const ev0 = P.events.length; const key = P.dir[0] > 0 ? 'KeyD' : P.dir[0] < 0 ? 'KeyA' : P.dir[1] > 0 ? 'KeyS' : 'KeyW';
    const p0 = [...g.player.char.pos] as Vec3; for (let f = 0; f < 240; f++) { P.W.input.keys.add(key); run(P, 1); } P.W.input.keys.delete(key); run(P, 120);
    const went = flat(p0, g.player.char.pos); const mv = pc.map((c, i) => flat(c.box.c, s0[i]));
    Fm.verdict(went > 1.5 && mv.filter(d => d > 0.08).length >= 2 && pc.every(c => c.rest), 'Sam, walked on the keys, shoves them too', went > 1.5 ? `he went ${f2(went)} m · moved ${mv.map(f2).join(' · ')} m · ${JSON.stringify(P.D.feetStats)} · 'prop' events ${P.events.slice(ev0).filter((e: any) => e.kind === 'prop').length}` : `NOT ASSERTED: the keys moved him ${f2(went)} m only (the camera's basis is not the corridor's)`);
  }
  return Fm.done();
}

/** The shove is silent to the simulation. */
export async function kickQuiet(): Promise<boolean> {
  const Fm = family('kickrubble-quiet: a boot\'s shove files no hearing event, snaps nothing, pushes no furniture — the digest is the switch-off walk\'s');
  const A = await walkOnce(true); const { S } = A; const shoves = S.D.feetStats.kicked + S.D.feetStats.offMan;
  const kinds = new Map<string, number>(); for (const e of S.events) kinds.set(e.kind, (kinds.get(e.kind) ?? 0) + 1);
  const O = await walkOnce(false); const okinds = new Map<string, number>(); for (const e of O.S.events) okinds.set(e.kind, (okinds.get(e.kind) ?? 0) + 1);
  const show = (m: Map<string, number>) => [...m].map(([k, n]) => `${k} ${n}`).join(', ') || 'none';
  Fm.verdict(shoves >= 6 && !S.events.some((e: any) => e.kind === 'prop') && show(kinds) === show(okinds), 'no hearing event comes of it', `${shoves} shoves · events during the walk: ${show(kinds)} (switch off: ${show(okinds)})`);
  Fm.verdict(S.D.stats.landed === A.landed0 && S.landedHook === 0, 'no landing is counted or sounded', `landings ${S.D.stats.landed - A.landed0} · break table told of a landing ${S.landedHook}×`);
  Fm.verdict(S.kicked === 0 && S.D.stats.spawned === A.spawned0, 'the break table is told of no kick: nothing snaps underfoot, no piece is born', `breaks.kicked calls ${S.kicked} · pieces spawned ${S.D.stats.spawned - A.spawned0}`);
  Fm.verdict(S.D.impulses === A.imp0, 'the impulses\' hash stream is not advanced', `impulses ${A.imp0} → ${S.D.impulses} · the feet's own count ${S.D.shoves}`);
  const propsOf = (G: any) => G.props.props.map((p: any) => `${p.x},${p.z},${p.yaw}`).join(' ');
  Fm.verdict(propsOf(S.game) === propsOf(O.S.game), 'no furniture was pushed by a piece', `${S.game.props.props.length} props: poses bit-equal to the switch-off walk's`);
  const da = JSON.stringify(digestGame(S.game)), db = JSON.stringify(digestGame(O.S.game));
  Fm.verdict(da === db, 'the game\'s digest after the walk equals the switch-off walk\'s', da === db ? `${da.length} chars, equal` : `DIFFERS at char ${[...da].findIndex((ch, i) => ch !== db[i])}: …${da.slice(Math.max(0, [...da].findIndex((ch, i) => ch !== db[i]) - 60), [...da].findIndex((ch, i) => ch !== db[i]) + 60)}…`);
  return Fm.done();
}

/** A plank at rest through a standing man is pushed off him. */
export async function kickOffMan(): Promise<boolean> {
  const Fm = family('kickrubble-offman: a piece that came to rest in a man is pushed off him');
  const S = await stage(true); const g = S.game; const here: Vec3 = [...S.gd.char.pos] as Vec3;
  RUBBLE.on = false; const plank = piece(S, [here[0] + 0.05, 0.3, here[2] + 0.03], PLANK, 0.5), lump = piece(S, [here[0] - 0.1, 0.3, here[2] - 0.12], LUMP, 1.2); run(S, 120);   // (they fall and come to rest THROUGH him, as a blast's pieces do)
  const lay = plank.rest && lump.rest && flat(plank.box.c, here) < 0.15 && flat(lump.box.c, here) < 0.25; RUBBLE.on = true; S.events.length = 0; const landed0 = S.D.stats.landed; run(S, 360);
  const clear = (c: any) => flat(c.box.c, S.gd.char.pos);
  /** the nearest he stands to any point of a piece's long axis (11 stations end to end), less its half width: how far its footprint is from his middle */
  const edge = (c: any, h: Vec3) => { const q = c.box.rot, yaw = q ? 2 * Math.atan2(q[1], q[3]) : c.box.yaw; const ax = [Math.cos(yaw), -Math.sin(yaw)]; let m = 9; for (let i = -5; i <= 5; i++) m = Math.min(m, Math.hypot(c.box.c[0] + ax[0] * h[0] * i / 5 - S.gd.char.pos[0], c.box.c[2] + ax[1] * h[0] * i / 5 - S.gd.char.pos[2])); return m - h[2]; };
  const need = RUBBLE.radius - RUBBLE.inside - 0.02;
  Fm.verdict(edge(lump, LUMP) >= need && edge(plank, PLANK) >= need, 'no part of either lies in him any more', `nearest part of the plank ${f2(edge(plank, PLANK))} m from his middle, of the lump ${f2(edge(lump, LUMP))} m · his circle is ${RUBBLE.radius} m (a piece may rest ${RUBBLE.inside} m into it)`);
  Fm.verdict(flat(here, S.gd.char.pos) < 1e-6 && S.events.length === 0 && S.D.stats.landed === landed0, 'he did not move, and nobody heard it', `he moved ${flat(here, S.gd.char.pos).toFixed(4)} m · hearing events ${S.events.length} · landings ${S.D.stats.landed - landed0}`);
  void g;
  return Fm.done();
}

/** A piece pinned between a man and a wall is not pushed through it and is given up on. */
export async function kickWall(): Promise<boolean> {
  const Fm = family('kickrubble-wall: a piece pinned against a wall is never pushed through it, and is given up on');
  const S = await stage(true); const g = S.game;
  // the corridor's nearer side wall from where he stands
  let wd: [number, number] = [0, 1], wt = 99; for (const d of [[S.dir[1], S.dir[0]], [-S.dir[1], -S.dir[0]]] as [number, number][]) { const h = g.col.raycast([S.p0[0], 0.3, S.p0[2]], [d[0], 0, d[1]], 8); if (h && h.t < wt) { wt = h.t; wd = d; } }
  const wallAt = (p: Vec3) => (p[0] - S.p0[0]) * wd[0] + (p[2] - S.p0[2]) * wd[1];   // metres toward the wall from his first place; the wall's face is at wt
  const stand: Vec3 = [S.p0[0] + wd[0] * (wt - 0.34), 0, S.p0[2] + wd[1] * (wt - 0.34)]; S.gd.char.pos[0] = stand[0]; S.gd.char.pos[2] = stand[2];
  const c = piece(S, [S.p0[0] + wd[0] * (wt - 0.1), 0.2, S.p0[2] + wd[1] * (wt - 0.1)], LUMP, 0); run(S, 600);
  Fm.verdict(wt < 8 && wallAt(c.box.c) < wt - 0.02, 'the piece is still on his side of the wall', `wall face ${f2(wt)} m off · the piece's middle at ${f2(wallAt(c.box.c))} m · ${JSON.stringify(S.D.feetStats)}`);
  const tries = S.D.shoves;
  Fm.verdict(S.D.feetStats.gaveUp >= 1 && tries <= Math.ceil(10 / (RUBBLE.giveUp * RUBBLE.cooldown + RUBBLE.giveUpSecs) + 1) * RUBBLE.giveUp, 'the tries are bounded: it is given up on, then tried again only after a wait', `${tries} tries in 10 s (uncapped: ${Math.round(10 / RUBBLE.cooldown)}) · given up ${S.D.feetStats.gaveUp}× · walled ${S.D.feetStats.walled}`);
  Fm.verdict([...c.box.c, ...c.v, ...c.w].every(Number.isFinite) && c.box.c[1] < 0.2, 'nothing NaN, it lies on the floor', `at ${c.box.c.map(f2).join(', ')} · rest ${c.rest}`);
  return Fm.done();
}

/** What a boot leaves alone, and the floor a slider never leaves. */
export async function kickLeft(): Promise<boolean> {
  const Fm = family('kickrubble-left: a boot leaves alone what is not at his feet, what is too heavy, and glass; a slider never leaves its floor');
  { // a piece on a desk top: he stands in reach of it, then walks past it
    const S = await stage(true); const g = S.game; RUBBLE.on = false;
    // a low static top (0.35‥1.0 m) with open floor beside it: the first the level has
    const topAt = (x: number, z: number) => { const h = g.col.raycast([x, 1.4, z], [0, -1, 0], 1.6); return h ? 1.4 - h.t : -1; };
    let spot: { x: number; z: number; top: number; sx: number; sz: number } | null = null;
    for (let z = 5; z < 23 && !spot; z += 0.25) for (let x = 5; x < 35 && !spot; x += 0.25) { const t = topAt(x, z); if (t < 0.35 || t > 1.0 || Math.abs(topAt(x + 0.1, z) - t) > 0.01 || Math.abs(topAt(x - 0.1, z) - t) > 0.01 || Math.abs(topAt(x, z + 0.1) - t) > 0.01 || Math.abs(topAt(x, z - 0.1) - t) > 0.01) continue;
      for (const [sx, sz] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { let open = true; for (const r of [0.4, 0.6, 0.9, 1.4, 1.9]) if (Math.abs(topAt(x + sx * r, z + sz * r)) > 0.02 || g.props.raycast([x + sx * r, 1.4, z + sz * r], [0, -1, 0], 1.39)) open = false; if (open && !g.props.raycast([x, 1.4, z], [0, -1, 0], 1.39)) { spot = { x, z, top: t, sx, sz }; break; } } }
    if (!spot) { Fm.verdict(false, 'the stage: a low top with open floor beside it', 'none found in the office'); return Fm.done(); }
    // the top's edge on its open side; a plank lying ACROSS it, its middle on the top, 14 cm of it out over the floor where he stands
    let ed = 0; while (ed < 2 && Math.abs(topAt(spot.x + spot.sx * (ed + 0.02), spot.z + spot.sz * (ed + 0.02)) - spot.top) < 0.01) ed += 0.02;
    const SH: Vec3 = [0.2, 0.012, 0.05]; const yaw = Math.atan2(-spot.sz, spot.sx);
    const lay = async (T: Stage) => { RUBBLE.on = false; const c = piece(T, [spot!.x + spot!.sx * (ed - 0.06), spot!.top + 0.1, spot!.z + spot!.sz * (ed - 0.06)], SH, yaw); run(T, 150);
      T.gd.hold = true; T.gd.char.pos[0] = spot!.x + spot!.sx * (ed + 0.3); T.gd.char.pos[2] = spot!.z + spot!.sz * (ed + 0.3); run(T, 2);
      const tip = [c.box.c[0] + spot!.sx * SH[0], c.box.c[2] + spot!.sz * SH[0]]; const into = RUBBLE.radius - Math.hypot(tip[0] - T.gd.char.pos[0], tip[1] - T.gd.char.pos[2]);
      return { c, on: c.rest && Math.abs(c.box.c[1] - spot!.top - SH[1]) < 0.01, into, p0: pose(c) }; };
    // the twin with the floor rule widened: there the same man, standing the same, DOES push it — he is in reach of it in plan, so the rule is what holds
    const T2 = await stage(true); const floor0 = RUBBLE.floor; let twin = 0, twinInto = 0;
    try { const B = await lay(T2); RUBBLE.floor = 5; RUBBLE.on = true; run(T2, 120); twin = T2.D.shoves; twinInto = B.into; } finally { RUBBLE.floor = floor0; }
    const A = await lay(S); const c = A.c; RUBBLE.on = true; S.events.length = 0; const l0 = S.D.stats.landed; run(S, 240);
    Fm.verdict(A.on && A.into > RUBBLE.inside && twin > 0 && pose(c) === A.p0 && S.D.shoves === 0, 'a plank lying on a desk top and out over its edge INTO the man standing there is not at his feet', `top ${f2(spot.top)} m at (${f2(spot.x)}, ${f2(spot.z)}), edge ${f2(ed)} m on · it reaches ${f2(A.into)} m into his circle in plan (pushed off past ${RUBBLE.inside}) · pose ${pose(c) === A.p0 ? 'bit-equal' : 'CHANGED'}, shoves ${S.D.shoves} · the twin with the floor rule widened (in ${f2(twinInto)} m): ${twin} shoves`);
    S.gd.char.pos[0] = spot.x + spot.sx * (ed + 0.3) - spot.sz * 1.2; S.gd.char.pos[2] = spot.z + spot.sz * (ed + 0.3) + spot.sx * 1.2; S.gd.hold = false; const from = [...S.gd.char.pos] as Vec3;
    order(S.gd, { goal: [spot.x + spot.sx * (ed + 0.3) + spot.sz * 1.2, 0, spot.z + spot.sz * (ed + 0.3) - spot.sx * 1.2] as any }); run(S, 360);
    Fm.verdict(flat(from, S.gd.char.pos) > 1.5 && pose(c) === A.p0 && S.D.shoves === 0 && S.D.stats.landed === l0 && !S.events.some((e: any) => e.kind === 'prop'), '…nor of the man walking through its overhang: it does not slide off, fall and bring the shift', `he walked ${f2(flat(from, S.gd.char.pos))} m along the top's edge, 0.3 m off it · pose ${pose(c) === A.p0 ? 'bit-equal' : 'CHANGED'} · shoves ${S.D.shoves} · landings +${S.D.stats.landed - l0}`);
    // a slider never leaves its floor: slid by hand (the mark and a boot's speed set on it) off each of the top's four sides, it stops on the top
    RUBBLE.on = false; S.gd.hold = true; let stayed = 0; const outs: string[] = [];
    for (const [dx, dz] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) { S.D.clear(); const k = piece(S, [spot.x, spot.top + 0.1, spot.z], [0.07, 0.012, 0.05], 0.3); run(S, 120); const y0 = k.box.c[1], at0 = [...k.box.c] as Vec3; const ld = S.D.stats.landed; S.events.length = 0;
      k.rest = false; k.stillT = 0; k.foot = true; k.v = [dx * RUBBLE.vMax, 0, dz * RUBBLE.vMax]; run(S, 180);
      const ok = !S.events.some((e: any) => e.kind === 'prop') && k.rest && Math.abs(k.box.c[1] - y0) < 0.004 && Math.abs(topAt(k.box.c[0], k.box.c[2]) - spot.top) < 0.01 && S.D.stats.landed === ld; stayed += ok ? 1 : 0; outs.push(`${f2(flat(at0, k.box.c))} m`); }
  }
  { // in his path: a slab too heavy for a boot, and a glass plate
    const S = await stage(true); const slab = piece(S, at(S, 2.0, 0.05, 0.3), [0.4, 0.22, 0.2], 0.2), plate = piece(S, at(S, 3.2, -0.05, 0.1), [0.12, 0.004, 0.09], 0.6), lump = piece(S, at(S, 4.0, 0, 0.2), LUMP, 0.1); run(S, 150);
    plate.glass = { pane: -1 }; const p = [pose(slab), pose(plate)], l0 = [...lump.box.c] as Vec3; walk(S);
    Fm.verdict(slab.rest && pose(slab) === p[0] && pose(plate) === p[1] && flat(lump.box.c, l0) > 0.08, 'he walks through a slab too heavy to shift and over a glass plate; the lump beyond them is kicked', `slab ${(0.4 * 0.22 * 0.2 * 8 * 600).toFixed(0)} kg (a boot shifts ≤ ${RUBBLE.heavy}) ${pose(slab) === p[0] ? 'untouched' : 'MOVED'} · plate ${pose(plate) === p[1] ? 'untouched' : 'MOVED'} · lump moved ${f2(flat(lump.box.c, l0))} m · ${JSON.stringify(S.D.feetStats)}`);
  }
  return Fm.done();
}


