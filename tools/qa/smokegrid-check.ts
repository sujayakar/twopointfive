// THE GAME'S OWN SMOKE ON THE COARSE GRID (src/game/sense.ts SENSE.grid, SmokeGrid; rust/sim/src/bricks.rs) — held on the office, headless, through the
// game's own tick and its own emit:
//   stands    with the switch up a grid is made and steps; with it down there is none (the blobs, as the build was). The numbers this side reads the
//             state by (sense.ts GRID) are the Rust source's own
//   mass      THE BOOK BALANCES: what the cells hold + what thin bricks held when they were given back + what was refused for want of a brick is what
//             was let out, to a part in a thousand million — a strong canister and one a tenth as strong, with no decay and with the game's own, read
//             early and four minutes on. Passing smoke from cell to cell makes none and loses none; what goes with a brick is said, not lost
//   full      more canisters than there are bricks for: the book says smoke was refused, the grid says it cannot answer, and the blobs do
//   door      a canister a metre inside a SHUT door: the metre beyond the door stays clear and the metre before it goes dark. The door opened, the smoke
//             goes through
//   block     THE STATE IS A PART OF THE GAME'S BLOCK IN THE MODULE (src/sim/state.ts): a pool whose first word is the row up to the end of the highest
//             brick that stands, so a snapshot of a game with a canister out carries a tenth of the part; the game keeps no array of it; and with the
//             switch down the game has no such part at home
//   put back  the game put back from a snapshot of three seconds earlier plays to the same state, cell for cell (the tables the module keeps beside the
//             state are made again from the block that was put back, and every face is asked again)
//   another game  …and the same snapshot put into a FRESH game steps to the same smoke in that game's own block
//   faces     …and so does one whose box world's copy is another copy before EVERY tick: the faces the grid keeps between steps are at every step what
//             asking them all afresh gives, with a door swinging and a wall moved and said on the way
//   parked    …and one stepped turn about with more games than the module has slots, so that its block is parked and brought back all the way: the
//             tables follow the block, not the slot
//   cost      what a step of the grid costs here (said, not judged)
//   bun run tools/qa/smokegrid-check.ts          (TPS_PROFILE=shipped in front for the other stand: the check holds on both)
// SMOKEGRID_BREAK=1 is the red path, two breaks at once, each also a word of its own:
//   =leaves   the grid is asked with no leaves, so a shut door holds nothing: `door` must FAIL, and nothing else does
//   =stamp    every game's block says one stamp whatever is done to it, so the tables the module keeps beside the state are made once and never again:
//             `put back` and `parked` must FAIL (and `faces`, which starts from a put-back), and `door` must PASS
// EXIT 0 = every line passes · 1 = a line fails (under a break word: every line that must fail did, and none that must pass failed) · 3 = THE RED PATH IS
// NOT RED for a line it must reach, or reached one it must not. Never seen on a GPU: nothing here is drawn, and the picture's own smoke is not this.
process.env.TPS_CPUSENSE = '1'; process.env.TPS_SMOKEGRID = '1';
import { resolve } from 'path';
import { readFileSync } from 'fs';
import { standUp } from './headless.ts';
const ROOT = resolve(import.meta.dir, '../..');
const S: any = await import(`${ROOT}/src/game/sense.ts`), R: any = await import(`${ROOT}/src/game/rollback.ts`), SIM: any = await import(`${ROOT}/src/sim/sim.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`);
type Vec3 = [number, number, number];
const WORD = process.env.SMOKEGRID_BREAK ?? '', NOLEAVES = WORD === '1' || WORD === 'leaves', PIN = WORD === '1' || WORD === 'stamp', BREAK = NOLEAVES || PIN;
if (WORD && !BREAK) { console.error(`SMOKEGRID_BREAK is 1, leaves or stamp (not '${WORD}')`); process.exit(2); }
const log = console.log, err = console.error; const hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let bad = 0; const failed = new Set<string>(); const verdict = (ok: boolean, what: string, detail: string) => { if (!ok) { bad++; failed.add(what.split(':')[0]); } log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
{ const no = SIM.simWhyNot(); if (no) { log(`=== smokegrid-check: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }

// the state row's layout (sense.ts GRID, held to bricks.rs below): a head, the emitters, two keys a brick, the light model's columns, and last the cells.
// A row here is the game's block read NOW, as far as it is in use (sense.ts SmokeGrid.row): never kept across a step.
const L = S.GRID, HEAD = L.head, EMITS = L.emitters * L.emit, KEYS = L.bricks * 2, RHO = L.rho, VOL = L.vol;
const massOf = (st: Float64Array) => { let m = 0; for (let i = RHO; i < st.length; i++) m += st[i]; return m * VOL; };   // (the cells, after the light model's columns: cpulight-check.ts holds those)
const bricksOf = (st: Float64Array) => { let n = 0; for (let b = 0; b < L.bricks; b++) if (st[HEAD + EMITS + b * 2]) n++; return n; };
const wordOf = (st: Float64Array) => { const u = new Uint32Array(st.buffer, st.byteOffset, st.length * 2); let h = 0x811c9dc5; for (let i = 0; i < u.length; i++) { h ^= u[i]; h = Math.imul(h, 0x01000193); } return (h >>> 0).toString(36); };

async function stand(): Promise<any> { hush(true); try { const W = await standUp({ uploadEvery: 1000 }); for (let i = 0; i < 3; i++) W.step(1 / 60); return W; } finally { hush(false); } }
const run = (W: any, secs: number, each?: (tick: number) => void) => { hush(true); try { const n = Math.round(secs * 60); for (let i = 0; i < n; i++) { each?.(i); W.step(1 / 60); } } finally { hush(false); } };
const canister = (g: any, pos: Vec3, ttl: number, density = 3) => g.smoke.emit({ pos, dir: [0, 1, 0], speed: 0, radius: 0.5, density, temperature: 0, ttl, age: 0, kind: 'canister' });
/** the red path's two breaks, on a game: its grid is asked with no leaves · its block says one stamp whatever is done to it (src/sim/state.ts SimState.stamp) */
const broken = (g: any) => { if (NOLEAVES) g.col.mirror.leaves = () => -1; if (PIN) Object.defineProperty(g.sim, 'stamp', { get: () => 1, set: () => {}, configurable: true }); };
const RATE = 3 * 5.568327996831708 * 0.125;   // (what that canister lets out a second: density × π^1.5 × radius³)

// ---- stands
{
  const W = await stand(), g = W.game; run(W, 0.3); const G = g.cpuSmoke.grid;   // (a step is a tenth of a second: the first comes six ticks in)
  const part = ST.simLayout().parts.find((q: any) => q.name === 'smoke'), len0 = G ? G.row().length : -1;
  const up = !!G && G.stands === true && G.sim === g.sim && L.glowD === HEAD + EMITS + KEYS && RHO === L.glowLamps + 1 && len0 === RHO && !!part && part.pool && part.len === 8 + (RHO + L.bricks * L.cells) * 8;   // (the light model's four columns, then the cells; with no smoke out the row in use ends where the cells start)
  S.SENSE.grid = false; run(W, 0.1); const down = g.cpuSmoke.grid === null && (!g.sim.open || g.sim.w[ST.simPart(ST.P_SMOKE) >>> 2] === 0); S.SENSE.grid = true;
  // sense.ts GRID against bricks.rs, name for name
  const src = readFileSync(resolve(ROOT, 'rust/sim/src/bricks.rs'), 'utf8'), there = (n: string) => Number(new RegExp(`const ${n}: \\w+ = ([\\d.]+);`).exec(src)?.[1] ?? NaN);
  const pairs: [string, number, number][] = [['step', L.step, there('SUB')], ['head', L.head, there('HEAD')], ['emitters', L.emitters, there('MAX_EMIT')], ['emit', L.emit, there('EMIT')], ['bricks', L.bricks, there('MAX_BRICKS')],
    ['cells', L.cells, there('BX') * there('BX') * there('BY')], ['vol', L.vol, there('CELL') ** 3], ['out', L.out, there('H_OUT')], ['given', L.given, there('H_GIVEN')], ['lost', L.lost, there('H_LOST')]];
  const off = pairs.filter(q => q[1] !== q[2]);
  verdict(up && down && !off.length, 'stands: the switch up makes a grid that steps, and down there is none', `up: a grid ${G ? `in the game's block, ${len0} doubles of its row in use, standing ${G.stands}` : 'was not made'} · down: ${down ? 'none, and no part of the block at home (the blobs answer)' : 'A GRID IS STILL THERE'} · ${pairs.length} numbers shared with bricks.rs${off.length ? ` — NOT THE SAME: ${off.map(q => `${q[0]} ${q[1]} here, ${q[2]} there`).join(', ')}` : ', each the same'} · the stand ${process.env.TPS_PROFILE ?? 'frozen'}`);
}

// ---- mass
{
  const K = S.SENSE.smoke, decay = K.decay, said: string[] = []; let ok = true, gave = 0;
  for (const [what, density, d] of [['a canister, no decay', 3, 0], ['a tenth of one, no decay', 0.3, 0], ['a canister, the game\'s decay', 3, decay], ['a tenth of one, the game\'s decay', 0.3, decay]] as [string, number, number][]) {
    K.decay = d;
    try {
      const W = await stand(), g = W.game, p = g.player.char.pos as Vec3;
      canister(g, [p[0] + 1, 0.2, p[2]], 6, density);
      const read = () => { const st = g.cpuSmoke.grid.row(), out = st[L.out], held = massOf(st), given = st[L.given], lost = st[L.lost]; return { out, held, given, lost, off: Math.abs(held + given + lost - out) / Math.max(out, 1e-300), bricks: bricksOf(st) }; };
      run(W, 14); const early = read(); run(W, 226); const late = read();
      // (with no decay, all that was ever let out is still in the book: density × π^1.5 × radius³ a second, for six seconds)
      const let6 = density * 5.568327996831708 * 0.125 * 6, whole = d !== 0 || Math.abs(early.out - let6) / let6 < 1e-9;
      ok = ok && early.off < 1e-9 && late.off < 1e-9 && whole && early.bricks > 2 && early.lost === 0 && late.lost === 0; gave = Math.max(gave, d === 0 ? late.given / late.out : 0);
      said.push(`${what}: at 14 s the book is off by ${early.off.toExponential(1)} of what was let out (${early.bricks} bricks, ${(100 * early.given / early.out).toFixed(2)}% given back), at 240 s by ${late.off.toExponential(1)} (${late.bricks} bricks${d === 0 ? `, ${(100 * late.given / late.out).toFixed(2)}% given back` : ''})${whole ? '' : ' · NOT ALL THAT WAS LET OUT IS IN THE BOOK'}`);
    } finally { K.decay = decay; }
  }
  verdict(ok, 'mass: what the cells hold, what went with the bricks given back and what was refused is what was let out', said.join('\n    '));
}

// ---- full
{
  const W = await stand(), g = W.game, nav = g.col.nav, C = g.cpuSmoke;
  // a canister on every free cell of a three-metre lattice over the level, each for a minute
  let n = 0; for (let z = 1.5; z < nav.H * nav.cell; z += 3) for (let x = 1.5; x < nav.W * nav.cell; x += 3) if (!nav.isBlocked(x, z) && n < 60) { canister(g, [x, 0.2, z], 60); n++; }
  run(W, 30);
  const G = C.grid, bricks = bricksOf(G.row()), a: Vec3 = [1.5, 1, 1.5], b: Vec3 = [nav.W * nav.cell - 1.5, 1, nav.H * nav.cell - 1.5];
  const grid = G.along(a, b), both = C.along(g.col, a, b), blobs = C.trans(a, b, (p: Vec3, q: Vec3) => g.col.segmentBlocked(p, q));
  // …and one more where no brick is: it can let nothing out, and the book says how much that was
  const st0 = G.row(), has = (x: number, z: number) => { const st = st0; for (let k = 0; k < L.bricks; k++) if (st[HEAD + EMITS + k * 2] === Math.floor(x / 2) + 1 && st[HEAD + EMITS + k * 2 + 1] === Math.floor(z / 2) + 1) return true; return false; };
  let spot: Vec3 | null = null; for (let z = 1; z < nav.H * nav.cell && !spot; z += 2) for (let x = 1; x < nav.W * nav.cell; x += 2) if (!nav.isBlocked(x, z) && !has(x, z)) { spot = [x, 0.2, z]; break; }
  if (spot) canister(g, spot, 5); run(W, 1);
  const st = G.row(), lost = st[L.lost], off = Math.abs(massOf(st) + st[L.given] + lost - st[L.out]) / st[L.out];
  verdict(bricks === L.bricks && grid === -1 && Object.is(both, blobs) && !!spot && lost > 0 && off < 1e-9, 'full: more smoke than bricks is said, and the blobs answer while it is so',
    `${n} canisters · ${bricks} of ${L.bricks} bricks · the grid ${grid === -1 ? 'says it cannot answer' : `ANSWERS ${grid}`} · the game's own smoke along a line is the blobs' (${blobs.toExponential(3)}): ${Object.is(both, blobs)} · one more where no brick is: ${spot ? `${lost.toFixed(3)} refused, in the book (off by ${off.toExponential(1)})` : 'NO FREE FLOOR WITHOUT A BRICK'}`);
}

// ---- door, block, put back, another game, faces, parked: one scene
{
  const W = await stand(), g = W.game, col = g.col, nav = col.nav;
  const free = (p: Vec3) => !nav.isBlocked(p[0], p[2]);
  // a shut door with open floor a metre either side of it
  let pick: any = null, n: Vec3 = [0, 0, 1];
  for (const d of g.doors.list) { if (d.angle !== 0 || !d.latched || !d.box) continue; const yaw = d.box.yaw, v: Vec3 = [Math.sin(yaw), 0, Math.cos(yaw)]; const c = d.box.c;
    if ([-1.55, -1, -0.55, 0.55, 1, 1.55].every(k => free([c[0] + v[0] * k, 0, c[2] + v[2] * k]))) { pick = d; n = v; break; } }
  if (!pick) verdict(false, 'door: a shut door holds the smoke, and lets it through opened', 'no shut door of the office has open floor a metre either side of it');
  else {
    const c = pick.box.c as Vec3, at = (k: number, y: number): Vec3 => [c[0] + n[0] * k, y, c[2] + n[2] * k];
    const near = () => g.cpuSmoke.along(col, at(0.55, 1), at(1.55, 1)), far = () => g.cpuSmoke.along(col, at(-0.55, 1), at(-1.55, 1));
    broken(g);   // (the red path: asked with no leaves · its block says one stamp from here on)
    const hold = () => { pick.angle = 0; pick.vel = 0; pick.latched = true; };
    canister(g, at(1, 0.2), 40); run(W, 6, hold);
    const n1 = near(), f1 = far();
    // the snapshot the lines below put back, and put into other games
    const snap = R.takeSnapshot(g);
    // THE NINE SECONDS EVERY LINE BELOW PLAYS, in whichever game: the door swings open, and a wall near the smoke is moved and said a second in (the
    // tallest plain static within three metres of the canister)
    const wallsOf = (game: any) => (game.col.boxes as any[]).filter(b => !b.rot && b.h[1] > 1 && Math.hypot(b.c[0] - c[0], b.c[2] - c[2]) < 3);
    const scriptOn = (game: any) => { const door = game.doors.list[g.doors.list.indexOf(pick)], walls = wallsOf(game);
      return (tick: number) => { door.latched = false; door.vel = 0; door.angle = Math.min(1.4, tick * 0.02); if (tick === 60 && walls.length) { const b = walls[0]; b.c[0] += 0.7; b.c[2] += 0.7; game.col.wrote(); } }; };
    const walls = wallsOf(g), script = scriptOn(g);
    run(W, 9, script);
    const n2 = near(), f2 = far(), G = g.cpuSmoke.grid, wordA = wordOf(G.row()), massA = massOf(G.row());
    const held = f1 > 0.99 && n1 < 0.2, through = f2 < 0.95;   // (the grid only spreads: nine seconds through a doorway is a haze beyond it, not a wall)
    verdict(held && through, 'door: a shut door holds the smoke, and lets it through opened', `shut, after 6 s: the metre before the door lets ${n1.toFixed(4)} through and the metre beyond it ${f1.toFixed(4)} · opened, 9 s on: ${n2.toFixed(4)} and ${f2.toFixed(4)} · ${bricksOf(G.row())} bricks${NOLEAVES ? ' · ASKED WITH NO LEAVES' : ''}`);
    // the block: what a snapshot of this game carries of the grid, and that the game keeps no array of it
    { const rowNow = G.row().length, live = 8 + rowNow * 8, whole = 8 + (RHO + L.bricks * L.cells) * 8, top = (rowNow - RHO) / L.cells;
      const own = Object.values(G).filter((v: any) => ArrayBuffer.isView(v)).length, views = snap.simViews ?? 0;
      let highest = 0; { const st = G.row(); for (let b = 0; b < L.bricks; b++) if (st[HEAD + EMITS + b * 2]) highest = b + 1; }
      verdict(G.sim === g.sim && g.sim.w[ST.simPart(ST.P_SMOKE) >>> 2] === live && top === highest && highest > 0 && live < whole / 2 && own === 0 && views === 0, 'block: the state is a part of the game\'s block, in use up to its highest brick, and the game keeps no array of it',
        `the row in use ends with brick ${top} of ${L.bricks} (the highest that stands: ${highest}) · ${live} of the part's ${whole} bytes are carried · typed arrays kept on the grid's object: ${own} · views over the module's memory the snapshot met: ${views}`); }
    // put back, and played again
    hush(true); R.restoreSnapshot(snap); hush(false); run(W, 9, script);
    const wordB = wordOf(g.cpuSmoke.grid.row());
    verdict(wordB === wordA, 'put back: the game put back from a snapshot plays to the same smoke, cell for cell', `straight ${wordA} · put back and played again ${wordB} · ${massA.toFixed(4)} held${PIN ? ' · THE TABLES WERE NOT MADE AGAIN' : ''}`);
    // …the same snapshot put into a fresh game, which steps on in its own block (under the leaves' break it too is asked with none; its stamp is its own)
    { const W2 = await stand(), g2 = W2.game; if (NOLEAVES) g2.col.mirror.leaves = () => -1; hush(true); R.restoreSnapshot(snap, g2); hush(false);
      run(W2, 9, scriptOn(g2));
      const G2 = g2.cpuSmoke.grid, wordD = G2 ? wordOf(G2.row()) : 'no grid';
      verdict(wordD === wordA && !!G2 && G2.sim === g2.sim && G2 !== G, 'another game: the snapshot put into a fresh game steps to the same smoke in that game\'s own block', `straight ${wordA} · in the fresh game ${wordD} · its grid is of its own block: ${!!G2 && G2.sim === g2.sim}`); }
    // …and with the box world's copy another copy before every tick
    hush(true); R.restoreSnapshot(snap); hush(false); run(W, 9, tick => { script(tick); col.mirror.gridDrop(); });
    const wordC = wordOf(g.cpuSmoke.grid.row());
    verdict(wordC === wordA && walls.length > 0, 'faces: the faces kept between steps are what asking them all afresh gives, a door swinging and a wall moved on the way', `kept ${wordA} · asked afresh every tick ${wordC} · ${walls.length ? 'a wall moved and said one second in' : 'NO WALL NEAR TO MOVE'}`);
    // …and a game that is never put back while it plays, stepped turn about with more games than the module has slots: its block is parked and brought
    // back all the way, other blocks stand in its slot meanwhile, and the tables follow the block. (Another fresh game handed the snapshot, so that
    // under the stamp's break this line is red by the parking and not by the put-back the lines above play from.)
    { const slots = ST.simLayout().games, others: any[] = []; for (let i = 0; i < slots; i++) { const o = await stand(); broken(o.game); canister(o.game, [o.game.player.char.pos[0] + 1 + i * 0.5, 0.2, o.game.player.char.pos[2]], 20); others.push(o); }
      const W5 = await stand(), g5 = W5.game; hush(true); R.restoreSnapshot(snap, g5); hush(false); broken(g5);
      for (let i = 0; i < 3; i++) for (const o of others) o.step(1 / 60);   // (the others have stood in every slot before it takes its first step)
      const p0 = ST.SIM_STATE_TALLY.parked, play = scriptOn(g5);
      run(W5, 9, tick => { play(tick); if (tick % 7 === 0) for (const o of others) o.step(1 / 60); });
      const wordE = wordOf(g5.cpuSmoke.grid.row()), parked = ST.SIM_STATE_TALLY.parked - p0;
      verdict(wordE === wordA && parked > 50, 'parked: stepped turn about with more games than the module has slots, the grid steps as one that never moved', `alone ${wordA} · with ${others.length} other games over ${slots} slots ${wordE} · ${parked} blocks parked on the way${PIN ? ' · THE TABLES WERE NOT MADE AGAIN' : ''}`); }
    // cost
    { hush(true); R.restoreSnapshot(snap); hush(false); const G2 = g.cpuSmoke.grid; let ms = 0, n = 0; hush(true); for (let i = 0; i < 600; i++) { const s0 = G2.at(1), t = performance.now(); W.step(1 / 60); const dt = performance.now() - t; if (G2.at(1) !== s0) { ms += dt; n++; } } hush(false); const burning = bricksOf(G2.row());
      let plain = 0, m = 0; hush(true); S.SENSE.grid = false; for (let i = 0; i < 300; i++) { const t = performance.now(); W.step(1 / 60); plain += performance.now() - t; m++; } S.SENSE.grid = true; hush(false);
      log(`- cost: a tick that steps the grid ${(ms / Math.max(1, n)).toFixed(3)} ms, a tick with the switch down ${(plain / m).toFixed(3)} ms (${n} stepping ticks, ${burning} bricks; this machine, said and not judged)`); }
  }
}
// THE RED PATH'S OWN VERDICT: each break must reach its lines, and only the lines it should
if (BREAK) {
  const must = [...(NOLEAVES ? ['door'] : []), ...(PIN ? ['put back', 'parked'] : [])], mustNot = WORD === 'stamp' ? ['door', 'another game'] : WORD === 'leaves' ? ['block', 'put back', 'another game', 'faces', 'parked'] : [];
  const green = must.filter(m => !failed.has(m)), red = mustNot.filter(m => failed.has(m)), always = ['stands', 'mass', 'full'].filter(m => failed.has(m));
  if (green.length || red.length || always.length) { log(`\n=== smokegrid-check: THE RED PATH IS NOT RED FOR ${green.join(', ') || 'nothing'}${red.length || always.length ? ` · AND IS RED WHERE IT MUST NOT BE: ${[...red, ...always].join(', ')}` : ''} (SMOKEGRID_BREAK=${WORD}) ===`); process.exit(3); }
  log(`\n=== smokegrid-check: the red path is red for ${must.join(', ')}${[...failed].filter(f => !must.includes(f)).length ? ` (and ${[...failed].filter(f => !must.includes(f)).join(', ')})` : ''}, ${bad} FAILED (SMOKEGRID_BREAK=${WORD}) ===`); process.exit(1);
}
log(bad ? `\n=== smokegrid-check: ${bad} FAILED ===` : '\n=== smokegrid-check: ALL PASS ===');
process.exit(bad ? 1 : 0);
