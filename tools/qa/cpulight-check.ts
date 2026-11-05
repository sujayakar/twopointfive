// THE LIGHT MODEL (src/game/sense.ts SENSE.bounce; rust/sim/src/glow.rs, light.rs; guards.ts lightEstimateAt) — held on the office, headless, through the
// game's own tick. EXIT 0 = every line passes · 1 = one fails.
//   bun run tools/qa/cpulight-check.ts          (TPS_PROFILE=shipped in front for the other stand: the check holds on both)
//   stands    the switch up over a grid sweeps the plan (lamps' light and bounce in its metres) and the estimate has the bounce in it; a new
//             encounter starts a new grid; down, the
//             columns stay zeros and the estimate is the one it always was, to the bit; up with no grid, the same. The numbers this file and the game
//             read the state by (sense.ts GRID) are the Rust sources' own
//   rest      with the lamps and the doors still, the bounce comes to rest, and where it rests is the rule's own fixed point worked out HERE from the
//             columns (albedo of what a metre keeps and what its open sides hand it), and never more than a closed room of the brightest metre holds
//   rooms     light comes round (metres no lamp reaches have bounce, by an open way) and a wall holds it (a metre with no open way has its own and
//             nobody else's); and a point reads the nearest metre whose middle it SEES, never one behind a wall
//   lamp      every lamp a switch holds put out: within two steps no metre holds their light (the plan is swept WHOLE when the lamps change, not a
//             slice at a time) and half a second on the bounce has gone with it
//   door      a SHUT door: no open way joins the metre before it and the metre beyond. Opened, within a sweep, one does
//   shadow    a canister between a man and the lamps: the direct light on him falls; the smoke cleared, it is the plain estimate again
//   put back  a game put back from a snapshot steps to the same state as the game that was never put back, a door swinging and smoke out on the way
//   cost      milliseconds a tick with the model up and down (said, with a loose bar)
// CPULIGHT_BREAK=1 is the red path: the field is swept with no leaves, so a shut door shuts nothing in, and `door` must FAIL.
process.env.TPS_CPUSENSE = '1'; process.env.TPS_SMOKEGRID = '1'; process.env.TPS_CPULIGHT = '1';
import { resolve } from 'path';
import { readFileSync } from 'fs';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const S: any = await import(`${ROOT}/src/game/sense.ts`), R: any = await import(`${ROOT}/src/game/rollback.ts`), SIM: any = await import(`${ROOT}/src/sim/sim.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`), GD: any = await import(`${ROOT}/src/game/guards.ts`);
type Vec3 = [number, number, number];
const BREAK = process.env.CPULIGHT_BREAK === '1';
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const verdict = (ok: boolean, what: string, detail: string) => { if (!ok) bad++; log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
{ const no = SIM.simWhyNot(); if (no) { log(`=== cpulight-check: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }

const L = S.GRID, K = S.SENSE.glow;
const wordOf = (st: Float64Array) => { const u = new Uint32Array(st.buffer, st.byteOffset, st.length * 2); let h = 0x811c9dc5; for (let i = 0; i < u.length; i++) { h ^= u[i]; h = Math.imul(h, 0x01000193); } return (h >>> 0).toString(36); };
/** a still scene: nobody walks, opens a door or shoots a lamp out while the field is read */
async function stand(bounce = true): Promise<any> { hush(true); process.env.TPS_CPULIGHT = bounce ? '1' : '0'; try { const W = await standUp({ uploadEvery: 1000, seed: 1 }); const g = W.game; g.aiEnabled = false; g.playerInvisible = true; g.godMode = true; for (let i = 0; i < 3; i++) W.step(1 / 60); return W; } finally { process.env.TPS_CPULIGHT = '1'; hush(false); } }
const run = (W: any, secs: number, each?: (tick: number) => void) => { hush(true); try { const n = Math.round(secs * 60); for (let i = 0; i < n; i++) { each?.(i); W.step(1 / 60); } } finally { hush(false); } };
/** the field of a game: its plan in metres and its four columns */
const fieldOf = (g: any) => { const G = g.cpuSmoke.grid, nx = G.at(L.nbx) * L.side, nz = G.at(L.nbz) * L.side; return { G, nx, nz, n: nx * nz, D: (i: number) => G.at(L.glowD + i) as number, B: (i: number) => G.at(L.glowB + i) as number, X: (i: number) => G.at(L.glowX + i) as number, Z: (i: number) => G.at(L.glowZ + i) as number }; };   // (each a read of the game's block as it stands: sense.ts SmokeGrid.at)
/** the ways out of metre i: [neighbour, open] toward +x, −x, +z, −z, where there is a neighbour */
const waysOf = (F: ReturnType<typeof fieldOf>, i: number): [number, number][] => { const ix = i % F.nx, iz = (i - ix) / F.nx, out: [number, number][] = [];
  if (ix + 1 < F.nx) out.push([i + 1, F.X(i)]); if (ix > 0) out.push([i - 1, F.X(i - 1)]); if (iz + 1 < F.nz) out.push([i + F.nx, F.Z(i)]); if (iz > 0) out.push([i - F.nx, F.Z(i - F.nx)]); return out; };
const plain = (g: any, P: Vec3) => GD.lightEstimateAt(g, P, undefined, false, null), model = (g: any, P: Vec3) => GD.lightEstimateAt(g, P);

// ---- stands
{
  const W = await stand(), g = W.game; run(W, 2 * K.sweep * L.step + 1); const G = g.cpuSmoke.grid, F = fieldOf(g);
  let lit = 0, glow = 0; for (let i = 0; i < F.n; i++) { if (F.D(i) > 0) lit++; if (F.B(i) > 0) glow++; }
  const free = (p: Vec3) => !g.col.nav.isBlocked(p[0], p[2]); const pts: Vec3[] = []; for (let z = 1.3; z < F.nz && pts.length < 300; z += 1.7) for (let x = 1.3; x < F.nx; x += 1.7) if (free([x, 0, z])) pts.push([x, 1.2, z]);
  let more = 0, less = 0; for (const p of pts) { const a = plain(g, p), b = model(g, p); if (b > a) more++; else if (b < a) less++; }
  // the Rust sources' own numbers
  const rs = (f: string) => readFileSync(`${ROOT}/rust/sim/src/${f}`, 'utf8'), num = (src: string, name: string) => { const m = new RegExp(`const ${name}: (?:usize|f64) = ([0-9.]+);`).exec(src); return m ? +m[1] : NaN; };
  const bricks = rs('bricks.rs'), glw = rs('glow.rs'), part = ST.simLayout().parts.find((q: any) => q.name === 'smoke');   // (the grid's part of a game's block: sim/state.ts)
  const pairs: [string, number, number][] = [['glowMax', L.glowMax, num(bricks, 'MAX_GLOW')], ['chest', L.chest, num(glw, 'CHEST')], ['side', L.side, num(bricks, 'BX') * num(bricks, 'CELL')], ['glowD', L.glowD, L.head + L.emitters * L.emit + L.bricks * 2],
    ['glowB', L.glowB, L.glowD + L.glowMax], ['glowX', L.glowX, L.glowD + 2 * L.glowMax], ['glowZ', L.glowZ, L.glowD + 3 * L.glowMax], ['glowLamps', L.glowLamps, L.glowD + 4 * L.glowMax], ['nbx', L.nbx, num(bricks, 'H_NBX')], ['nbz', L.nbz, num(bricks, 'H_NBZ')], ['rho', L.rho, L.glowLamps + 1], ['the part', part ? part.len : NaN, 8 + (L.rho + L.bricks * L.cells) * 8]];
  const off = pairs.filter(p => p[1] !== p[2]).map(p => `${p[0]} ${p[1]} ≠ ${p[2]}`);
  const up = G.stands && F.n > 0 && F.n <= L.glowMax && lit > F.n / 10 && glow > lit && more > pts.length / 2 && less === 0;
  // a new encounter takes a new grid: its steps counted from none and its beat from the first tick, so a file loaded into this game and into a fresh one go on alike
  const STEPS = num(bricks, 'H_STEPS');
  const had = F.G.at(STEPS) as number; hush(true); g.restartEncounter(); W.step(1 / 60); hush(false); const N = g.cpuSmoke.grid;
  const anew = !!N && had > 0 && N.at(STEPS) === 0 && Math.abs(N.clock - 1 / 60) < 1e-12 && N.at(L.glowLamps) === 0;
  // down: a fresh game with the switch down never writes the columns, and its estimate is the plain one
  const W2 = await stand(false), g2 = W2.game; run(W2, 4); const F2 = fieldOf(g2); let written = 0; for (let i = L.glowD; i < L.rho; i++) if (F2.G.at(i) !== 0) written++;
  let same = 0; for (const p of pts) if (Object.is(plain(g2, p), model(g2, p))) same++;
  // up with no grid: nothing to stand on
  const W3 = await stand(), g3 = W3.game; S.SENSE.grid = false; run(W3, 0.2); let bare = 0; for (const p of pts) if (Object.is(plain(g3, p), model(g3, p))) bare++; S.SENSE.grid = true;
  verdict(up && anew && written === 0 && same === pts.length && bare === pts.length && !off.length, 'stands: the switch up sweeps the plan and the estimate has the bounce in it; down, or with no grid, it is the estimate it always was',
    `up: ${F.nx} by ${F.nz} metres, lamps' light in ${lit}, bounce in ${glow} · ${more} of ${pts.length} points brighter by the model, ${less} darker · a new encounter: ${anew ? 'a new grid, no step taken yet' : 'THE OLD GRID GOES ON'} · down: ${written} numbers written in the columns, ${same} of ${pts.length} points the plain estimate to the bit · up with no grid: ${bare} of ${pts.length} · ${pairs.length} numbers shared with the Rust${off.length ? ` · DIFFER: ${off.join(', ')}` : ''} · the stand ${process.env.TPS_PROFILE ?? 'frozen'}`);
}

// ---- rest, rooms: one still game
{
  const W = await stand(), g = W.game, col = g.col;
  // (the office has lamps that sweep: the field follows them a sweep behind and never rests. This line is about the rule, so they are put out)
  const lamps: any[] = g.engine.lights.lights, where = () => lamps.map(l => JSON.stringify([l.pos, l.dir, l.intensity])), w0 = where(); run(W, 0.5); const w1 = where(); let out = 0;
  for (let i = 0; i < lamps.length; i++) if (w0[i] !== w1[i] && lamps[i].enabled && !lamps[i].owner) { lamps[i].enabled = false; out++; }
  run(W, 12); const F = fieldOf(g);
  // the rule, worked out here from the columns
  let worst = 0, top = 0, bmax = 0; for (let i = 0; i < F.n; i++) { top = Math.max(top, F.D(i)); bmax = Math.max(bmax, F.B(i)); }
  for (let i = 0; i < F.n; i++) { let kept = 1, got = 0; for (const [j, o] of waysOf(F, i)) { kept -= K.share * o; got += o * (F.D(j) + F.B(j)); } const want = K.albedo * (kept * (F.D(i) + F.B(i)) + K.share * got); worst = Math.max(worst, Math.abs(want - F.B(i))); }
  const before = wordOf(F.G.row().slice(L.glowB, L.glowB + F.n)); run(W, 2); const after = wordOf(F.G.row().slice(L.glowB, L.glowB + F.n));
  const bound = K.albedo / (1 - K.albedo) * top;
  verdict(worst <= 1e-9 * Math.max(bmax, 1e-9) + 1e-15 && bmax <= bound * (1 + 1e-9) && bmax > 0, 'rest: the bounce comes to rest at the rule\'s own fixed point, under what a closed room of the brightest metre holds',
    `${out} sweeping lamps put out · after 12 s: the rule worked out here differs from the column by at most ${worst.toExponential(2)} (the brightest bounce ${bmax.toFixed(4)}, the brightest lamps' light ${top.toFixed(4)}, a closed room of it ${bound.toFixed(4)}) · two seconds on the bounce is ${before === after ? 'the same to the bit' : 'still moving in its last bits'}`);
  // rooms
  let round = 0, alone = 0, aloneOff = 0, shut = 0; const iso = K.albedo / (1 - K.albedo);
  for (let i = 0; i < F.n; i++) { const ways = waysOf(F, i), open = ways.filter(w => w[1] > 0).length;
    if (F.D(i) === 0 && F.B(i) > 0 && open > 0) round++;
    if (open === 0) { shut++; if (Math.abs(F.B(i) - iso * F.D(i)) <= 1e-9 * Math.max(F.D(i), 1e-9)) alone++; else aloneOff++; } }
  // a point reads a metre it sees: over the whole floor, a point's reading is its own metre's where it sees that metre's middle; else the first it sees of
  // the three beside it on its own side of the middle (across the nearer edge, the other, the corner), worked out here; else nothing
  const G = g.cpuSmoke.grid; let asked = 0, own = 0, beside = 0, none = 0, wrong = 0;
  for (let z = 0.15; z < F.nz; z += 0.35) for (let x = 0.15; x < F.nx; x += 0.35) { if (col.nav.isBlocked(x, z)) continue; const P: Vec3 = [x, 1.4, z]; asked++;
    const v = G.glowAt(col, P), ix = Math.floor(x), iz = Math.floor(z), fx = x - ix - 0.5, fz = z - iz - 0.5, sx = fx < 0 ? -1 : 1, sz = fz < 0 ? -1 : 1;
    const seen = (jx: number, jz: number) => jx >= 0 && jz >= 0 && jx < F.nx && jz < F.nz && !col.segmentBlocked(P, [jx + 0.5, L.chest, jz + 0.5]);
    const order: [number, number][] = Math.abs(fx) >= Math.abs(fz) ? [[ix, iz], [ix + sx, iz], [ix, iz + sz], [ix + sx, iz + sz]] : [[ix, iz], [ix, iz + sz], [ix + sx, iz], [ix + sx, iz + sz]];
    const k = order.findIndex(([jx, jz]) => seen(jx, jz)), want = k < 0 ? 0 : F.B(order[k][1] * F.nx + order[k][0]);
    if (k === 0) own++; else if (k > 0) beside++; else none++;
    if (!Object.is(v, want)) wrong++; }
  verdict(round > 0 && shut > 0 && aloneOff === 0 && wrong === 0 && own > asked / 2 && beside > 0, 'rooms: light comes round by an open way, a wall holds it, and a point reads a metre whose middle it sees',
    `${round} metres no lamp reaches have bounce by an open way · ${shut} metres have no open way, ${alone} of them hold their own bounce alone${aloneOff ? `, ${aloneOff} DO NOT` : ''} · ${asked} points of the floor: ${own} read their own metre, ${beside} one beside it that they see, ${none} nothing (no middle of the four in sight)${wrong ? ` · ${wrong} READ A METRE THEY DO NOT SEE` : ''}`);
}

// ---- lamp
{
  const W = await stand(), g = W.game; run(W, 6); const F = fieldOf(g); let d0 = 0, b0 = 0; for (let i = 0; i < F.n; i++) { d0 = Math.max(d0, F.D(i)); b0 = Math.max(b0, F.B(i)); }
  // (by the game's own switch where a lamp is in a group, as a breaker does it)
  let out = 0; const groups = new Set<string>(); for (const l of g.engine.lights.lights) if (l.enabled && !l.owner) { if (l.group) groups.add(l.group); else { l.enabled = false; g.lightHome.feed(l); } out++; }   // (a lamp put out by hand is fed to its row, where it has one: a light question reads the rows)
  for (const name of groups) g.setGroup(name, false);
  const t0 = performance.now(); run(W, 0.25); const ms = performance.now() - t0; let d1 = 0, lit = 0; for (let i = 0; i < F.n; i++) { d1 = Math.max(d1, F.D(i)); if (F.D(i) > 0) lit++; }
  run(W, 0.5); let b1 = 0; for (let i = 0; i < F.n; i++) b1 = Math.max(b1, F.B(i));
  verdict(d0 > 0 && b0 > 0 && out > 0 && d1 < d0 * 0.01 && b1 < b0 * 0.01, 'lamp: the lamps put out, no metre keeps their light and the bounce goes with it',
    `${out} lamps put out · the brightest lamps' light ${d0.toFixed(4)} before, a quarter of a second on ${d1.toFixed(4)} in ${lit} metres (what the lamps no switch holds still give; a slice a step would have left the rest of the plan lit for three seconds) (those fifteen ticks, the whole plan swept in one of them: ${ms.toFixed(1)} ms) · the brightest bounce ${b0.toFixed(4)} before, ${b1.toExponential(1)} three quarters of a second on`);
}

// ---- door, shadow, put back: one scene
{
  const W = await stand(), g = W.game, col = g.col, nav = col.nav;
  const free = (p: Vec3) => !nav.isBlocked(p[0], p[2]);
  let pick: any = null, n: Vec3 = [0, 0, 1];
  for (const d of g.doors.list) { if (d.angle !== 0 || !d.latched || !d.box) continue; const yaw = d.box.yaw, v: Vec3 = [Math.sin(yaw), 0, Math.cos(yaw)]; const c = d.box.c;
    if ([-1.55, -1, -0.55, 0.55, 1, 1.55].every(k => free([c[0] + v[0] * k, 0, c[2] + v[2] * k]))) { pick = d; n = v; break; } }
  if (!pick) verdict(false, 'door: a shut door shuts the light in, and lets it by opened', 'no shut door of the office has open floor a metre either side of it');
  else {
    const c = pick.box.c as Vec3, at = (k: number, y: number): Vec3 => [c[0] + n[0] * k, y, c[2] + n[2] * k];
    if (BREAK) col.mirror.leaves = () => -1;   // (the red path: the field is swept with no leaves)
    const hold = () => { pick.angle = 0; pick.vel = 0; pick.latched = true; }, ajar = () => { pick.latched = false; pick.vel = 0; pick.angle = 1.4; };
    /** can the light get from the metre before the door to the metre beyond it by open ways, without going more than three metres from the door */
    const joined = () => { const F = fieldOf(g), idx = (p: Vec3) => Math.floor(p[2]) * F.nx + Math.floor(p[0]), a = idx(at(0.8, 0)), b = idx(at(-0.8, 0)); if (a === b) return null;
      const near = (i: number) => Math.hypot(i % F.nx + 0.5 - c[0], Math.floor(i / F.nx) + 0.5 - c[2]) <= 3; const seen = new Set([a]), todo = [a];
      while (todo.length) { const i = todo.pop()!; if (i === b) return true; for (const [j, o] of waysOf(F, i)) if (o > 0 && near(j) && !seen.has(j)) { seen.add(j); todo.push(j); } } return false; };
    run(W, 2 * K.sweep * L.step + 1, hold); const shutJ = joined(), F0 = fieldOf(g), far0 = F0.B(Math.floor(at(-0.8, 0)[2]) * F0.nx + Math.floor(at(-0.8, 0)[0])), near0 = F0.B(Math.floor(at(0.8, 0)[2]) * F0.nx + Math.floor(at(0.8, 0)[0]));
    run(W, K.sweep * L.step + 0.5, ajar); const openJ = joined(), F1 = fieldOf(g), far1 = F1.B(Math.floor(at(-0.8, 0)[2]) * F1.nx + Math.floor(at(-0.8, 0)[0])), near1 = F1.B(Math.floor(at(0.8, 0)[2]) * F1.nx + Math.floor(at(0.8, 0)[0]));
    verdict(shutJ === false && openJ === true, 'door: a shut door shuts the light in, and lets it by opened',
      shutJ === null ? 'THE DOOR STANDS INSIDE ONE METRE: nothing to ask' : `shut: ${shutJ ? 'AN OPEN WAY JOINS the metre before the door and the metre beyond' : 'no open way joins the two metres'} (bounce ${near0.toFixed(5)} before it, ${far0.toFixed(5)} beyond) · opened, a sweep on: ${openJ ? 'one does' : 'STILL NONE'} (${near1.toFixed(5)} and ${far1.toFixed(5)})${BREAK ? ' · THE FIELD WAS SWEPT WITH NO LEAVES' : ''}`);

    // ---- put back   (the same seconds again from the snapshot: the door swinging open, a canister out)
    const canister = (pos: Vec3) => g.smoke.emit({ pos, dir: [0, 1, 0], speed: 0, radius: 0.5, density: 3, temperature: 0, ttl: 30, age: 0, kind: 'canister' });
    const script = (tick: number) => { if (tick === 0) canister(at(1, 0.2)); pick.latched = false; pick.vel = 0; pick.angle = Math.min(1.4, tick * 0.02); };
    const from = R.takeSnapshot(g); run(W, 6, script); const wordA = wordOf(g.cpuSmoke.grid.row());
    hush(true); R.restoreSnapshot(from); hush(false); run(W, 6, script); const wordB = wordOf(g.cpuSmoke.grid.row());
    verdict(wordA === wordB, 'put back: a game put back from a snapshot steps to the same state as the one that never was', `six seconds straight on, a door swinging and a canister out: ${wordA} · put back to where they began and stepped again: ${wordB}`);

    // ---- shadow   (the canister of the last line is a metre before the door: the light on a point in its smoke)
    const G = g.cpuSmoke.grid; let P: Vec3 | null = null, best = 0;
    for (const k of [0.6, 1, 1.4, 1.8, 2.2]) for (const y of [1.2, 1.6]) { const p = at(k, y), v = plain(g, p); if (v > best && G.along(p, [p[0], 2.6, p[2]]) < 0.9) { best = v; P = p; } }
    if (!P) verdict(false, 'shadow: smoke between a man and the lamps dims the light on him', `no lit point in the smoke before the door (the brightest of ten reads ${best.toFixed(5)})`);
    else {
      const direct = () => model(g, P!) - G.glowAt(col, P!), a = plain(g, P), b = direct();
      hush(true); g.smoke.clearAll?.(); hush(false); run(W, 0.3, ajar); const a2 = plain(g, P), b2 = direct();
      verdict(a > 0 && b < a * 0.98 && Math.abs(b2 - a2) <= 1e-12 * Math.max(a2, 1e-9), 'shadow: smoke between a man and the lamps dims the light on him, and cleared it does not',
        `at (${P.map(v => v.toFixed(1)).join(', ')}): the lamps' light ${a.toFixed(5)} by the plain estimate, ${b.toFixed(5)} through the smoke (${(100 * b / a).toFixed(1)} %) · the smoke cleared: ${a2.toFixed(5)} and ${b2.toFixed(5)}`);
    }
  }
}

// ---- cost
{
  const time = async (on: boolean) => { const W = await stand(on); run(W, 4); const t0 = performance.now(); run(W, 10); return (performance.now() - t0) / 600; };
  const down = await time(false), up = await time(true), down2 = await time(false);
  const base = Math.min(down, down2);
  verdict(up < base * 1.6 + 0.3, 'cost: a tick with the model up', `${up.toFixed(3)} ms a tick up · ${base.toFixed(3)} ms down (the quieter of two runs) · ten seconds of the still office each, one seed`);
}
log(bad ? `\n=== cpulight-check: ${bad} FAILED${BREAK ? ' (the red path: `door` must be one of them)' : ''} ===` : '\n=== cpulight-check: ALL PASS ===');
process.exit(bad ? 1 : 0);
