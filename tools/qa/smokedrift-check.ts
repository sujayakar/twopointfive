// THE COARSE GRID'S SMOKE DRIFTS WITH THE WIND (src/game/sense.ts SENSE.drift; rust/sim/src/bricks.rs 3b; src/game/wind.ts stormDrift) — headless,
// through the game's own tick and its own emit. On the office's stormy night:
//   switch    down, a canister under the open sky steps to the smoke it stepped to before there was a drift (said: its word); up, to another
//   drift     up, the smoke of a canister under the open sky is carried DOWNWIND: its middle goes along the weather's vector at a fair part of the
//             wind's speed and hardly across it; down, it stays over the canister
//   roof      a canister in a room: the same smoke with the switch up as with it down, cell for cell (no wind under a roof)
//   calm      …and so is one under the open sky when the weather's wind is nothing (no statement of the drift is reached). THE WEATHER'S OWN SWITCH
//             IS NOT ASKED (`?noweather`, the panel: the session's, which no snapshot or recording carries): with it off the game's smoke drifts
//             as with it on
//   mass      THE BOOK BALANCES with the drift: what the cells hold + what went with the bricks given back and OVER THE PLAN'S EDGE + what was refused is
//             what was let out, to a part in a thousand million, for a canister the wind carries off the level; and some of it did go over the edge
//   put back  the game put back from a snapshot of three seconds earlier drifts to the same smoke, cell for cell, and so does a fresh game handed it
// …and on a level with open ground (levels/gwot-yard.json, in a picked weather):
//   bearing   A STRONG WIND IS NOT BENT: under the dust's wind, too fast for one pass of a step along x, the smoke's middle goes off on the wind's own
//             bearing (each axis held back by itself, it went off 20° to the side), and as far in the time as the wind's speed says
//   low       IT STAYS LOW: of one of the game's canisters in the rain's wind next to none is above a standing man's eyes (1.5 m), where in still
//             air a good part is (the picture's wind pulls the smoke's whole velocity toward its own, the rise too, and its smoke out of doors
//             stays under head height)
//   back      THE WIND THE OTHER WAY (no weather of the game's blows toward −x or −z; a hand on the panel can): the smoke goes with it, a canister by
//             the plan's near corner is carried over the near edges, and the book balances
//   bricks    how many bricks a canister of the game's takes in the rain's wind, and three (said, not judged: the grid has 160, and says so itself
//             when they are not enough — smokegrid-check.ts `full`)
//   bun run tools/qa/smokedrift-check.ts          (TPS_PROFILE=shipped in front for the other stand: the check holds on both)
// SMOKEDRIFT_BREAK=1 is the red path: the game is told a wind of nothing, so `switch`, `drift`, `calm` (its half that says the weather's wind moves
// the smoke whatever the session's switch), `low`, `mass`, `bearing` and `back` must FAIL and the others pass.
// EXIT 0 = every line passes · 1 = a line fails (under the break: every line that must fail did, and none that must pass failed) · 3 = THE RED PATH IS
// NOT RED for a line it must reach, or reached one it must not. Never seen on a GPU: nothing here is drawn, and the picture's own smoke is not this
// (how far the two are apart is tools/qa/modelerr.ts's to say, from a report made on a GPU: its row `adrift`).
process.env.TPS_CPUSENSE = '1'; process.env.TPS_SMOKEGRID = '1';
import { resolve } from 'path';
import { readFileSync } from 'fs';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const S: any = await import(`${ROOT}/src/game/sense.ts`), R: any = await import(`${ROOT}/src/game/rollback.ts`), SIM: any = await import(`${ROOT}/src/sim/sim.ts`);
const LV: any = await import(`${ROOT}/src/scene/levelLoad.ts`), LD: any = await import(`${ROOT}/src/scene/levelData.ts`);
type Vec3 = [number, number, number];
const WORD = process.env.SMOKEDRIFT_BREAK ?? '', BREAK = WORD === '1';
if (WORD && !BREAK) { console.error(`SMOKEDRIFT_BREAK is 1 (not '${WORD}')`); process.exit(2); }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const failed = new Set<string>(); const verdict = (ok: boolean, what: string, detail: string) => { if (!ok) { bad++; failed.add(what.split(':')[0]); } log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
{ const no = SIM.simWhyNot(); if (no) { log(`=== smokedrift-check: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }

const L = S.GRID, HEAD = L.head, EMITS = L.emitters * L.emit, RHO = L.rho, VOL = L.vol;
const massOf = (st: Float64Array) => { let m = 0; for (let i = RHO; i < st.length; i++) m += st[i]; return m * VOL; };
const wordOf = (st: Float64Array) => { const u = new Uint32Array(st.buffer, st.byteOffset, st.length * 2); let h = 0x811c9dc5; for (let i = 0; i < u.length; i++) { h ^= u[i]; h = Math.imul(h, 0x01000193); } return (h >>> 0).toString(36); };
/** where the smoke's middle is in plan, what share of it is above 1.5 m, and how much there is (a brick is 4 × 8 × 4 half-metre cells, x fastest then z then y: bricks.rs cell_at) */
const middle = (st: Float64Array) => { let m = 0, x = 0, z = 0, up = 0;
  for (let b = 0; b < L.bricks; b++) { const bx = st[HEAD + EMITS + b * 2] - 1, bz = st[HEAD + EMITS + b * 2 + 1] - 1; if (bx < 0 || RHO + (b + 1) * L.cells > st.length) continue;
    for (let c = 0; c < L.cells; c++) { const v = st[RHO + b * L.cells + c]; if (!v) continue; m += v; if (c >> 4 >= 3) up += v; x += v * (bx * 2 + (c % 4 + 0.5) * 0.5); z += v * (bz * 2 + ((c >> 2) % 4 + 0.5) * 0.5); } }
  return { x: m ? x / m : 0, z: m ? z / m : 0, high: m ? up / m : 0, held: m * VOL }; };

/** a stand of the office, or of a level file under a picked weather. The red path tells every game a wind of nothing */
async function stand(file?: string, kind?: string): Promise<any> { hush(true); try {
  const level = file ? LV.loadLevel(LD.parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8'))) : undefined;
  const W = await standUp({ uploadEvery: 1000, ...(level ? { level } : {}) }), g = W.game; g.aiEnabled = false; g.playerInvisible = true; g.godMode = true;
  if (kind) g.weather.setKind(kind);
  for (let i = 0; i < 3; i++) W.step(1 / 60);
  if (BREAK) g.wind.stormDrift = () => ({ x: 0, z: 0, low: 1, gain: 0, fullAt: 1, share: () => 1 });
  return W; } finally { hush(false); } }
const run = (W: any, secs: number) => { hush(true); try { const n = Math.round(secs * 60); for (let i = 0; i < n; i++) W.step(1 / 60); } finally { hush(false); } };
const canister = (g: any, pos: Vec3, ttl: number) => g.smoke.emit({ pos, dir: [0, 1, 0], speed: 0, radius: 0.5, density: 3, temperature: 0, ttl, age: 0, kind: 'canister' });
/** what a check keeps of a grid: read NOW and kept as numbers (the row itself is a view of the game's block, which moves when another game steps) */
interface Read { word: string; x: number; z: number; high: number; held: number; out: number; given: number; lost: number; off: number; bricks: number; over: boolean }
const read = (g: any): Read => { const st: Float64Array = g.cpuSmoke.grid.row(), m = middle(st), out = st[L.out], given = st[L.given], lost = st[L.lost], held = massOf(st);
  let bricks = 0; for (let b = 0; b < L.bricks; b++) if (st[HEAD + EMITS + b * 2]) bricks++;
  return { word: wordOf(st), x: m.x, z: m.z, high: m.high, held, out, given, lost, off: Math.abs(held + given + lost - out) / Math.max(out, 1e-300), bricks, over: st[7] !== 0 }; };
/** a canister at `at` for `ttl` seconds in a fresh game, played `secs` seconds with the switch as told: what its grid holds then, and the game */
async function play(at: Vec3, drift: boolean, secs: number, ttl = 6, weather: 'on' | 'off' | 'still' = 'on'): Promise<{ W: any; g: any; r: Read }> {
  const W = await stand(), g = W.game;   // (a stand puts the switch where its word says: it is raised for this play alone, after)
  S.SENSE.drift = drift;
  try { if (weather === 'off') g.weather.enabled = false; if (weather === 'still') g.weather.wind = [0, 0]; canister(g, at, ttl); run(W, secs); return { W, g, r: read(g) }; } finally { S.SENSE.drift = false; }
}
// the places: the weather's own vector, a spot under the open sky with the level's width downwind of it, a room, and a spot by the plan's downwind edge
const W0 = await stand(), g0 = W0.game, nav = g0.col.nav, wind = g0.weather.wind as [number, number], speed = Math.hypot(wind[0], wind[1]), along: [number, number] = [wind[0] / (speed || 1), wind[1] / (speed || 1)];
const mx = nav.W * nav.cell, mz = nav.H * nav.cell;
const clear = (x: number, z: number, r: number, share: (s: number) => boolean) => { for (let dz = -r; dz <= r; dz++) for (let dx = -r; dx <= r; dx++) { const px = x + dx, pz = z + dz; if (px < 0 || pz < 0 || px >= mx || pz >= mz || nav.isBlocked(px, pz) || !share(g0.wind.stormShare(px, pz))) return false; } return true; };
const find = (ok: (x: number, z: number) => boolean): Vec3 | null => { for (let z = 1.5; z < mz - 1; z += 1) for (let x = 1.5; x < mx - 1; x += 1) if (ok(x, z)) return [x, 0.2, z]; return null; };
// (open sky: a metre clear round it and three metres of open floor downwind · a room: two metres round it under a roof · the edge: as the first, with
//  the plan's edge within four metres downwind)
const downwind = (x: number, z: number, d: number) => { for (let k = 1; k <= d; k++) { const px = x + along[0] * k, pz = z + along[1] * k; if (px < 0 || pz < 0 || px >= mx || pz >= mz) return k; if (nav.isBlocked(px, pz) || g0.wind.stormShare(px, pz) < 1) return -1; } return 0; };
const SKY = find((x, z) => clear(x, z, 1, s => s >= 1) && downwind(x, z, 3) === 0), ROOM = find((x, z) => clear(x, z, 2, s => s === 0)), EDGE = find((x, z) => clear(x, z, 1, s => s >= 1) && downwind(x, z, 4) > 0);
if (!SKY || !ROOM || !EDGE || !(speed > 0.3)) { log(`=== smokedrift-check: the office's night has ${speed > 0.3 ? 'no' : 'a wind of ' + speed.toFixed(2) + ' m/s and'} ${!SKY ? 'open sky with three metres downwind' : !ROOM ? 'room' : 'open spot by the downwind edge'} — FAIL ===`); process.exit(1); }
const at3 = (p: Vec3) => `(${p[0]}, ${p[2]})`;

// ---- switch, drift
let a0 = '';
{
  const a = (await play(SKY, false, 8)).r, b = (await play(SKY, true, 8)).r; a0 = a.word;
  verdict(a.word !== b.word, 'switch: down, the smoke under the open sky is the still air\'s; up, it is another', `a canister at ${at3(SKY)} for 6 s, read at 8 s · down ${a.word} · up ${b.word} (the weather's wind ${wind[0]}, ${wind[1]} m/s)`);
  const dOff = [(a.x - SKY[0]), (a.z - SKY[2])], dOn = [(b.x - SKY[0]), (b.z - SKY[2])];
  const alongOn = dOn[0] * along[0] + dOn[1] * along[1], acrossOn = Math.abs(dOn[0] * along[1] - dOn[1] * along[0]), still = Math.hypot(dOff[0], dOff[1]);
  // (a canister lets smoke out for six of the eight seconds, so its smoke is on average five seconds old: at the wind's whole speed the middle would be
  //  5 × speed downwind. The wind is slower near the ground, walls stand in the way and what passes the plan's edge is gone: at least a quarter of
  //  that and never more than all of it, and no more than a metre across — where in still air the middle stays within 0.6 m of the canister)
  const far = 5 * speed;
  verdict(alongOn > far / 4 && alongOn < far * 1.05 && acrossOn < 1 && still < 0.6, 'drift: up, the smoke is carried downwind; down, it stays over the canister',
    `up: its middle is ${alongOn.toFixed(2)} m downwind of the canister and ${acrossOn.toFixed(2)} m across (the wind's whole speed would be ${far.toFixed(2)} m), ${b.held.toFixed(3)} held · down: ${still.toFixed(2)} m from it, ${a.held.toFixed(3)} held`);
}
// ---- roof, calm
{
  const a = (await play(ROOM, false, 8)).r, b = (await play(ROOM, true, 8)).r;
  verdict(a.word === b.word, 'roof: a canister in a room steps to the same smoke with the switch up as down', `at ${at3(ROOM)}, read at 8 s · down ${a.word} · up ${b.word}`);
  const c = (await play(SKY, false, 8, 6, 'still')).r, d = (await play(SKY, true, 8, 6, 'still')).r, on = (await play(SKY, true, 8)).r, off = (await play(SKY, true, 8, 6, 'off')).r;
  verdict(c.word === d.word && c.word === a0 && on.word === off.word && on.word !== c.word, 'calm: with no wind in the weather the switch changes nothing, and the weather\'s own on / off is not asked',
    `a wind of nothing, at ${at3(SKY)} · down ${c.word} · up ${d.word} · the weather's wind, the switch up: with the weather on ${on.word}, switched off in the session ${off.word}`);
}
// ---- mass, put back
{
  const { W, g, r } = await play(EDGE, true, 14), still = (await play(EDGE, false, 14)).r, g0v = still.given / still.out;
  verdict(r.off < 1e-9 && r.lost === 0 && r.given / r.out > g0v + 0.05, 'mass: with the drift the book balances, what went over the plan\'s edge in it',
    `a canister at ${at3(EDGE)}, read at 14 s: the book is off by ${r.off.toExponential(1)} of what was let out · ${(100 * r.given / r.out).toFixed(1)}% went with bricks given back and over the edge (in still air ${(100 * g0v).toFixed(1)}%) · ${(100 * r.held / r.out).toFixed(1)}% held · refused ${r.lost}`);
  // put back: from here, three seconds on, against the same three seconds played again from the snapshot
  S.SENSE.drift = true;
  try { canister(g, SKY, 4); run(W, 1); const snap = R.takeSnapshot(g); run(W, 3); const a = read(g).word; hush(true); R.restoreSnapshot(snap); hush(false); run(W, 3); const b = read(g).word;
    // …and the same snapshot put into a FRESH game, which has let no smoke out itself: it drifts there as it did here
    const W2 = await stand(), g2 = W2.game; S.SENSE.drift = true; hush(true); R.restoreSnapshot(snap, g2); hush(false); run(W2, 3); const c = g2.cpuSmoke.grid ? read(g2).word : 'no grid';
    verdict(a === b && a === c, 'put back: the game put back from a snapshot drifts to the same smoke, cell for cell, and so does a fresh game handed it', `straight ${a} · put back and played again ${b} · in a fresh game ${c}`); } finally { S.SENSE.drift = false; }
}
// ---- the yard: bearing, back, bricks
{
  const YARD = 'gwot-yard.json', deg = (x: number, z: number) => Math.atan2(z, x) * 180 / Math.PI;
  /** a stand of the yard in weather `kind` (its wind, or `blow`), the switch up for as long as `fn` plays it */
  const yard = async (kind: string, blow: [number, number] | null, fn: (W: any, g: any, open: (x: number, z: number) => boolean) => void) => {
    const W = await stand(YARD, kind), g = W.game, n = g.col.nav; if (blow) g.weather.wind = [blow[0], blow[1]];
    S.SENSE.drift = true; try { fn(W, g, (x, z) => x > 0 && z > 0 && x < n.W * n.cell && z < n.H * n.cell && !n.isBlocked(x, z) && g.wind.stormShare(x, z) >= 1); } finally { S.SENSE.drift = false; } };
  /** the open spot with the most open ground downwind of it: `far` metres along the wind `w` and `side` metres to either side */
  const roomiest = (g: any, open: (x: number, z: number) => boolean, w: readonly number[], far: number, side: number): Vec3 | null => { const n = g.col.nav, l = Math.hypot(w[0], w[1]), ux = w[0] / l, uz = w[1] / l; let best: Vec3 | null = null, most = -1;
    for (let z = 1.5; z < n.H * n.cell - 1; z++) for (let x = 1.5; x < n.W * n.cell - 1; x++) { if (!open(x, z)) continue; let k = 0; for (let c = -side; c <= side; c++) for (let a = -1; a <= far; a++) if (open(x + a * ux - c * uz, z + a * uz + c * ux)) k++; if (k > most) { most = k; best = [x, 0.2, z]; } }
    return best; };
  // bearing: the dust's wind is past one pass of a step along x. Read after three seconds of a canister that runs for two
  await yard('dust', null, (W, g, open) => {
    const w = g.weather.wind as [number, number], sp = Math.hypot(w[0], w[1]), at = roomiest(g, open, w, 9, 3);
    if (!at) { verdict(false, 'bearing: a strong wind carries the smoke on its own bearing', 'the yard has no open ground'); return; }
    canister(g, at, 2); run(W, 3); const r = read(g), dx = r.x - at[0], dz = r.z - at[2], off = Math.abs(deg(dx, dz) - deg(w[0], w[1])), went = Math.hypot(dx, dz);
    // (smoke let out over two seconds and read at three is 2 s old on average; the storm's field is 0.55 to 0.79 of the wind below 1.3 m, where a
    //  canister lets out: between a third of the wind's whole way and all of it)
    verdict(sp * 0.1 / 0.5 > 0.25 && off < 8 && went > 2 * sp / 3 && went < 2 * sp, 'bearing: a strong wind carries the smoke on its own bearing',
      `the dust's wind ${w[0]}, ${w[1]} m/s (bearing ${deg(w[0], w[1]).toFixed(1)}°; ${(sp * 0.1 / 0.5).toFixed(2)} of a cell a step, past the ${0.25} a pass carries) · a canister at ${at3(at)}, read at 3 s: its middle went ${went.toFixed(2)} m on ${deg(dx, dz).toFixed(1)}° (${off.toFixed(1)}° off; the wind's whole way would be ${(2 * sp).toFixed(2)} m) · ${r.held.toFixed(3)} held`); });
  // low: one of the game's own canisters (a fifth of a metre wide, let out at the ground) in the rain's wind, and the same in still air
  { const high: number[] = [];
    for (const up of [true, false]) await yard('rain', null, (W, g, open) => { S.SENSE.drift = up; const at = roomiest(g, open, g.weather.wind, 9, 3); if (!at) return;
      g.smoke.emit({ pos: [at[0], 0.1, at[2]], dir: [0, 1, 0], speed: 1.9, radius: 0.2, density: 95, temperature: 1.6, ttl: 32, age: 0, kind: 'canister' }); run(W, 8); high.push(read(g).high); });
    verdict(high.length === 2 && high[0] < 0.05 && high[1] > 0.15, 'low: up, the smoke under the open sky stays under a standing man\'s eyes; down, a good part rises over them',
      high.length === 2 ? `one of the game's canisters on the yard, 8 s on: ${(100 * high[0]).toFixed(1)}% of its smoke is above 1.5 m with the switch up, ${(100 * high[1]).toFixed(1)}% with it down` : 'the yard has no open ground'); }
  // back: a wind toward −x and −z, and a canister by the near corner
  await yard('rain', [-1.1, -0.45], (W, g, open) => {
    const far = roomiest(g, open, [-1.1, -0.45], 9, 3); let near: Vec3 | null = null;
    for (let d = 1.5; d < 12 && !near; d++) for (let x = 1.5; x <= d && !near; x++) if (open(x, d) && open(x - 1, d - 1)) near = [x, 0.2, d];
    if (!far || !near) { verdict(false, 'back: a wind toward −x and −z carries the smoke its way, over the near edges', 'the yard has no open ground for it'); return; }
    canister(g, far, 6); run(W, 8); const a = read(g), dx = a.x - far[0], dz = a.z - far[2], along = (dx * -1.1 + dz * -0.45) / Math.hypot(1.1, 0.45);
    g.cpuSmoke.grid.clear(); g.smoke.clearAll?.(); canister(g, near, 6); run(W, 20); const b = read(g);
    verdict(along > 1.2 && a.off < 1e-9 && b.off < 1e-9 && b.given / b.out > 0.3 && b.lost === 0, 'back: a wind toward −x and −z carries the smoke its way, over the near edges',
      `wind −1.1, −0.45 m/s · a canister at ${at3(far)}, read at 8 s: its middle is ${along.toFixed(2)} m along the wind (${dx.toFixed(2)}, ${dz.toFixed(2)}), the book off by ${a.off.toExponential(1)} · one at ${at3(near)} by the near corner, read at 20 s: ${(100 * b.given / b.out).toFixed(1)}% of what it let out went over the edge and with bricks given back, the book off by ${b.off.toExponential(1)}`); });
  // bricks: the game's own canister (smoke/params: density 95, a fifth of a metre, 32 s) in the rain's wind, one and then three, said
  if (!BREAK) for (const cans of [1, 3]) await yard('rain', null, (W, g, open) => {
    const n = g.col.nav, w = g.weather.wind as [number, number], spots: Vec3[] = [];
    for (let z = 1.5; z < n.H * n.cell - 1; z++) for (let x = 1.5; x < n.W * n.cell - 1; x++) if (open(x, z)) spots.push([x, 0.2, z]);
    spots.sort((p, q) => (p[0] * w[0] + p[2] * w[1]) - (q[0] * w[0] + q[2] * w[1])); const used: Vec3[] = [];
    for (const p of spots) { if (used.length >= cans) break; if (used.every(u => Math.hypot(u[0] - p[0], u[2] - p[2]) > 4)) used.push(p); }
    for (const p of used) g.smoke.emit({ pos: p, dir: [0, 1, 0], speed: 1.9, radius: 0.2, density: 95, temperature: 1.6, ttl: 32, age: 0, kind: 'canister' });
    let peak = 0, over = 0, reads = 0; for (let t = 0; t < 45; t++) { run(W, 1); const r = read(g); reads++; peak = Math.max(peak, r.bricks); if (r.over) over++; }
    log(`- bricks: ${cans} of the game's canisters at the yard's upwind end in the rain's wind (${w[0]}, ${w[1]} m/s), 45 s: ${peak} of ${L.bricks} bricks at most · the grid said it could not answer at ${over} of ${reads} reads (said, not judged)`); });
}
if (BREAK) {
  const must = ['switch', 'drift', 'calm', 'low', 'mass', 'bearing', 'back'], green = must.filter(m => !failed.has(m)), red = [...failed].filter(f => !must.includes(f));
  if (green.length || red.length) { log(`\n=== smokedrift-check: THE RED PATH IS NOT RED FOR ${green.join(', ') || 'nothing'}${red.length ? ` · AND IS RED WHERE IT MUST NOT BE: ${red.join(', ')}` : ''} (SMOKEDRIFT_BREAK=1) ===`); process.exit(3); }
  log(`\n=== smokedrift-check: the red path is red for ${must.join(', ')}, ${bad} FAILED (SMOKEDRIFT_BREAK=1) ===`); process.exit(1);
}
log(bad ? `\n=== smokedrift-check: ${bad} FAILED ===` : '\n=== smokedrift-check: ALL PASS ===');
process.exit(bad ? 1 : 0);
