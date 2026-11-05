// TWO PLAYERS IN ONE GAME (src/game/seats.ts; `standUp({ seats: 2 })`): is the second seat a second player — his own man, his own pair, his own record — in a
// squad mission played from two seats' records, and is a game of one seat the game it always was?
//   bun run tools/qa/seats-check.ts                  every line below; exit 0 = all pass, 1 = one failed (named). Run it on both stands (TPS_PROFILE=shipped).
//   SEATS_BREAK=1 bun run tools/qa/seats-check.ts    the red path: every seat's view answers the FIRST seat's record (SEAT_ASKS.shared) — one player in two bodies;
//                                                    `men`, `walk`, `orders`, `fog`, `host` and `down` must FAIL
// The mission is the room clear with the four (the page's `?roomclear&squad4`: fog of war up), played under SEAT_ASKS.strict: the Game's own field of a seat's
// name (g.player, g.orders …), asked anywhere in a game of two, THROWS — so every line below also says no code of the tick asked "the player" of nobody's.
// WHAT IS HELD:
//   one       a game of one seat: its seat's view IS the Game, the seat owns nothing apart, and `player` is the plain field it always was (the run digests of
//             one seat against the build before this file are tools/qa/playthrough.ts's, PT_DIGEST=1: not asked here)
//   men       two seats: two men with two ids standing apart on the floor, the four dealt two a seat, each seat's order book holding its own two and no other
//   walk      each man walks by his own record: one east, one west, at once
//   orders    a seat's order reaches its own pair and nobody else's: seat 1 taps the squad key on a door with his cursor (keys and cursor in HIS record) and his
//             two have that door; seat 0's two are as they were — then the other way round
//   fog       the fog of war is the team's: its eyes are both men's and all four teammates'
//   down      one player down is not the end: the mission runs on, the other man still walks, the downed man's pair still stands; both down ends it
//   host      outside a tick the Game's own `player` (what a HUD, a camera, a panel ask) is the seat this host shows, Game.localSeat; inside one it throws
//   fight     both players walk at the nearest enemy and fire for 40 s of the room clear and 30 s of the night breach, a key of the game's pressed every
//             second and a half: no throw — a firefight asks the Game for no "the player" either — and it was a fight: somebody went down
//   tables    over all of the above (math/side.ts watchViewKeys): a named table keyed by the game was asked under a seat's VIEW only if it is listed as a
//             seat's own (seats.ts SEAT_TABLES), and no listed one was asked under the Game itself — a world table read through a view finds no row, a
//             seat's read through the Game finds the first player's, and neither throws
//   word      `?seats=2` in an address is two seats (seatsOfSearch: what standGame reads), no word is one
//   rollback  a moment mid-fight is taken (rollback.ts takeSnapshot), ninety ticks are played from both seats' kept records, the moment is put back: the state is
//             the moment's to the bit, and the same records play the same ninety ticks, bit for bit — and moving seat 1's man moves the state's word
//   twice     the whole script played again in a fresh game ends on the same state, to the bit
//   ahead     a look ahead (walkahead.ts walkAhead: one seat's man walked through records the game has not had, on a copy of him) lands where the whole game
//             then puts him from the same records, tick by tick and to the bit (a walk, a sprint, a crouch walk, a coast, from a seat marked remote; and again from
//             the middle of the sprint); the game's state word, the seat and the records are as they were — it writes nothing; no
//             record answers where he stands, and so does a man who is down. Prints what it costs beside a whole tick
//   twin      a firefight played twice in fresh games — plain, and with a look ahead for both seats before every tick (the last eight records, throws and
//             rounds in flight among them) — is the same game: the state's word the same every second and at the end
//   refuses   a mission written round one commander (the hostage rescue) refuses a game of two, in words
import { standUp, ROOT } from './headless.ts';
const { SEAT_ASKS, SEAT_TABLES, men, seatsOfSearch } = await import(`${ROOT}/src/game/seats.ts`);
const { VIEW_KEYS, watchViewKeys } = await import(`${ROOT}/src/math/side.ts`);
const { sampleCmd, cmdText, cmdFromText } = await import(`${ROOT}/src/game/cmd.ts`);
const { Input } = await import(`${ROOT}/src/game/input.ts`);
const { FollowCamera } = await import(`${ROOT}/src/game/camera.ts`);
const { startFromQuery, startScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { takeSnapshot, restoreSnapshot, stateHash, stateDiff } = await import(`${ROOT}/src/game/rollback.ts`);
const { walkAhead } = await import(`${ROOT}/src/game/walkahead.ts`);
const { WALL_STATS } = await import(`${ROOT}/src/game/wallbreak.ts`);
const { hitPlayer } = await import(`${ROOT}/src/game/combat.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const BREAK = process.env.SEATS_BREAK === '1', DT = 1 / 60, shipped = process.env.TPS_PROFILE === 'shipped';
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
console.log(`=== seats-check: two players in one game (src/game/seats.ts) · ${shipped ? 'shipped' : 'frozen'} stand${BREAK ? ' · SEATS_BREAK: every view answers the first seat' : ''} ===`);
const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const at = (p: ArrayLike<number>) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;

// ---- one
{
  const W: any = await standUp({ seed: 9 }), g = W.game, d = Object.getOwnPropertyDescriptor(g, 'player');
  verdict(g.seats.length === 1 && g.seats[0].view === g && g.seats[0].own === null && !!d && 'value' in d && g.localSeat === g.seats[0] && g.seat === g.seats[0],
    'one: a game of one seat is the Game itself', `seats ${g.seats.length} · its view is the Game ${g.seats[0].view === g} · owns nothing apart ${g.seats[0].own === null} · 'player' a plain field ${!!d && 'value' in d}`);
}

/** a player's hands: an input and a lens of his own, sampled into a record each tick */
interface Hand { v: any; I: any; cam: any; goal: number[] | null }
interface Play { W: any; g: any; hands: Hand[]; step(): void; run(secs: number): void; cmds(): any[] }
async function stand(seats = 2): Promise<Play> {
  Math.random = mulberry32(0xc0de);
  const W: any = await standUp({ seed: 9, seats }), g = W.game;
  const r = startFromQuery(g, 'roomclear&squad4'); if (r.why) throw new Error(r.why);
  const v0 = g.seats[0].view, v1 = g.seats[1].view;
  const I1: any = Input.detached(); I1.mouseX = 800; I1.mouseY = 500;
  const cam1: any = new FollowCamera(); cam1.aspect = W.cam.aspect; cam1.target = [...v1.player.char.pos]; cam1.rebuild();
  const hands: Hand[] = [{ v: v0, I: W.input, cam: W.cam, goal: null }, { v: v1, I: I1, cam: cam1, goal: null }];
  const drive = (h: Hand) => { const I = h.I; for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k); if (!h.goal) return;
    const p = h.v.player.char.pos, dx = h.goal[0] - p[0], dz = h.goal[2] - p[2], d = Math.hypot(dx, dz); if (d < 0.25) return;
    const { fwd, right } = h.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / d, b = (dx * right[0] + dz * right[2]) / d, T = 0.3827;
    if (a > T) I.keys.add('KeyW'); if (a < -T) I.keys.add('KeyS'); if (b > T) I.keys.add('KeyD'); if (b < -T) I.keys.add('KeyA'); };
  const step = () => { for (const h of hands) drive(h); g.seats[1].hand(sampleCmd(v1, I1, cam1, W.canvas)); W.step(DT); cam1.update(DT, v1.followPos(), v1.aimPoint); I1.endFrame(); };
  const cmds = () => hands.map(h => cmdFromText(cmdText(sampleCmd(h.v, h.I, h.cam, W.canvas))));
  return { W, g, hands, step, run: (secs: number) => { for (let i = 0; i < Math.round(secs * 60); i++) step(); }, cmds };
}
const ops = (g: any): any[] => g.guards.filter((x: any) => x.unit && x.team === 'operative');
/** the whole script: returns what each line needs; `quiet` = no verdicts (the second playing, for `twice`) */
async function play(quiet: boolean): Promise<any> {
  const say = quiet ? (_ok: boolean, _l: string, _d: string) => {} : verdict;
  const P = await stand(), { g, hands, run, step } = P, [h0, h1] = hands, v0 = h0.v, v1 = h1.v;
  run(1.5);
  // ---- men
  { const a = v0.player, b = v1.player, T = ops(g), b0 = v0.orders.squad(), b1 = v1.orders.squad(), inLevel = (p: number[]) => Number.isFinite(p[0]) && Number.isFinite(p[2]);
    const dealt = T.length === 4 && b0.length === 2 && b1.length === 2 && b0.every((m: any) => !b1.includes(m)) && T.every((m: any) => b0.includes(m) || b1.includes(m));
    say(g.seats.length === 2 && a !== b && a.char.id !== b.char.id && inLevel(a.char.pos) && inLevel(b.char.pos) && flat(a.char.pos, b.char.pos) > 0.5 && dealt && men(g).length === 2,
      'men: two men, and the four dealt two a seat', `ids ${a.char.id} and ${b.char.id} · ${at(a.char.pos)} and ${at(b.char.pos)}, ${flat(a.char.pos, b.char.pos).toFixed(2)} m apart · seat 0 has ${b0.map((m: any) => m.callsign).join(' + ') || 'nobody'} · seat 1 has ${b1.map((m: any) => m.callsign).join(' + ') || 'nobody'}`); }
  // ---- walk
  { const p0 = [...v0.player.char.pos], p1 = [...v1.player.char.pos]; h0.goal = [p0[0] + 4, 0, p0[2]]; h1.goal = [p1[0] - 4, 0, p1[2]]; run(2); h0.goal = h1.goal = null; run(0.3);
    const d0 = v0.player.char.pos[0] - p0[0], d1 = v1.player.char.pos[0] - p1[0];
    say(d0 > 1 && d1 < -1, 'walk: each man walks by his own record', `seat 0's went ${d0.toFixed(2)} m east · seat 1's went ${(-d1).toFixed(2)} m west`); }
  // ---- orders
  { const key: string = CMDRING.on ? CMDRING.key : ORDERS.quickKey;
    /** the seat taps the squad's key with his cursor on a door: nearest shut doors tried in turn until his pair has an order on one */
    const tap = (h: Hand): any => { const mine = h.v.orders.squad(), p = h.v.player.char.pos;
      const doors = g.doors.list.filter((d: any) => d.isClosed()).sort((a: any, b: any) => flat(a.frameCentre, p) - flat(b.frameCentre, p)).slice(0, 5);
      for (const d of doors) { const before = mine.map((m: any) => h.v.orders.of(m));
        const [sx, sy, front] = h.cam.project(d.frameCentre, P.W.canvas.clientWidth, P.W.canvas.clientHeight); if (!front) continue; h.I.mouseX = sx; h.I.mouseY = sy;
        step(); h.I.keys.add(key); h.I.pressed.add(key); step(); step(); h.I.keys.delete(key); step(); run(0.2);
        if (mine.some((m: any, i: number) => { const A = h.v.orders.of(m); return A && A !== before[i]; })) return d; }
      return null; };
    const was0 = v0.orders.squad().map((m: any) => v0.orders.of(m)), d1 = tap(h1), got1 = v1.orders.squad().map((m: any) => v1.orders.of(m)?.order ?? '-');
    const kept0 = v0.orders.squad().every((m: any, i: number) => v0.orders.of(m) === was0[i]);
    const was1 = v1.orders.squad().map((m: any) => v1.orders.of(m)), d0 = tap(h0), got0 = v0.orders.squad().map((m: any) => v0.orders.of(m)?.order ?? '-');
    const kept1 = v1.orders.squad().every((m: any, i: number) => v1.orders.of(m) === was1[i]);
    say(!!d1 && !!d0 && kept0 && kept1 && v0.orders !== v1.orders, "orders: a seat's order reaches its own pair and nobody else's",
      `seat 1 tapped [${key}] on ${d1?.def?.name ?? 'no door that took it'}: his two ${got1.join(' / ')} · seat 0's two untouched ${kept0} — seat 0 tapped on ${d0?.def?.name ?? 'no door that took it'}: his two ${got0.join(' / ')} · seat 1's two untouched ${kept1}`); }
  // ---- fog
  { run(0.2); const ids = new Set<number>(g.fogOfWar.eyes.map((e: any) => e.id)), want = [v0.player.char.id, v1.player.char.id, ...ops(g).filter((m: any) => m.state !== 'dead').map((m: any) => m.char.id)];
    say(g.fogOfWar.active && want.every(id => ids.has(id)) && new Set(want).size === want.length, "fog: the fog of war's eyes are both men's and every teammate's",
      `fog on ${g.fogOfWar.active} · eyes ${[...ids].join(', ')} · wanted ${want.join(', ')}`); }
  // ---- rollback
  { const tick = (c: any[]) => { g.stepSeats(DT, c); g.engine.lights.update(DT, g.time); g.engine.world.dynamics = g.dynBoxes; g.engine.world.upload(); };
    h0.goal = [v0.player.char.pos[0], 0, v0.player.char.pos[2] + 3]; h1.goal = [v1.player.char.pos[0], 0, v1.player.char.pos[2] - 3]; run(0.5);
    const S = takeSnapshot(g), hash0 = stateHash(g), N = 90, kept: any[][] = [], ref: string[] = [];
    for (let i = 0; i < N; i++) { const c = P.cmds(); kept.push(c.map(x => cmdFromText(cmdText(x)))); tick(c); ref.push(stateHash(g)); }
    restoreSnapshot(S); const back = stateHash(g) === hash0;
    let same = -1; for (let i = 0; i < N; i++) { tick(kept[i]); if (stateHash(g) !== ref[i]) { same = i; break; } }
    const pl = v1.player.char; pl.pos[0] += 0.01; const moved = stateHash(g) !== ref[N - 1]; pl.pos[0] -= 0.01; const again = stateHash(g) === ref[N - 1];
    h0.goal = h1.goal = null;
    say(back && same < 0 && moved && again, 'rollback: a moment of a game of two is put back to the bit, and the same records play the same ticks',
      `put back to the bit ${back} · ${N} ticks from the kept records ${same < 0 ? 'the same, bit for bit' : `DIFFER at tick ${same}`} · seat 1's man a centimetre over moves the state's word ${moved}, and back ${again}`); }
  // ---- ahead
  { run(0.3); const seat = g.seats[1], man = v1.player.char, N = 46, now = performance.now.bind(performance), cv = P.W.canvas;
    { const w = v1.player.wall; if (w) { h1.goal = [man.pos[0] + w.n[0] * 1.5, 0, man.pos[2] + w.n[2] * 1.5]; run(1.2); h1.goal = null; run(0.3); } }   // (the page's stand: the walk before this left him with his back on a wall — a hold is not a walk of his own, so he steps off it first)
    // the records a client would have sent and not had answered: a walk, a sprint, C and a crouch walk the other way, Shift in the crouch, a coast — from a seat the
    // game does not trust (`remote`: with the record check on, each view is worked out again from its pose, about where the man then stands)
    const held = (i: number): string[] => i < 12 ? ['KeyD'] : i < 26 ? ['KeyD', 'KeyW', 'ShiftLeft'] : i < 38 ? ['KeyA', 'KeyS'] : i < 42 ? ['KeyA', 'KeyS', 'ShiftLeft'] : [];
    const recs: any[] = [], I = h1.I, moves = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', 'KeyC'];
    for (let i = 0; i < N; i++) { for (const k of moves) I.keys.delete(k); for (const k of held(i)) I.keys.add(k); if (i === 26) { I.keys.add('KeyC'); I.pressed.add('KeyC'); } recs.push(cmdFromText(cmdText(sampleCmd(v1, I, h1.cam, cv)))); I.pressed.delete('KeyC'); }
    for (const k of moves) I.keys.delete(k);
    const was = seat.remote; seat.remote = true;
    const p0 = [...man.pos], text0 = recs.map(cmdText).join('|'), hash0 = stateHash(g), had = seat.cmd, path: number[][] = [];
    const none = walkAhead(seat, [], DT), stays = flat(none.pos, p0) === 0 && none.pos !== man.pos;
    let t = now(); const A = walkAhead(seat, recs, DT, path); const aheadMs = now() - t;
    t = now(); for (let i = 0; i < 20; i++) walkAhead(seat, recs.slice(0, 8), DT); const eightMs = (now() - t) / 20;
    const pure = stateHash(g) === hash0 && seat.cmd === had && recs.map(cmdText).join('|') === text0 && flat(man.pos, p0) === 0;
    const still = cmdFromText(cmdText(sampleCmd(v0, h0.I, h0.cam, cv)));
    const tick = (c: any[]) => { g.stepSeats(DT, c); g.engine.lights.update(DT, g.time); g.engine.world.dynamics = g.dynBoxes; g.engine.world.upload(); };
    let worst = 0, off = -1, ran = false, low = false, went = 0, B: any = null, pureB = true; const pathB: number[][] = [], MID = 20; t = now();
    for (let i = 0; i < N; i++) {
      if (i === MID) { const h = stateHash(g); B = walkAhead(seat, recs.slice(MID), DT, pathB); pureB = stateHash(g) === h && v1.player.sprinting; }   // (a second look, begun in the middle of the sprint: what the game last did with him is a run)
      tick([still, recs[i]]); if (i >= MID) { const q = pathB[i - MID], e = Math.hypot(q[0] - man.pos[0], q[1] - man.pos[1], q[2] - man.pos[2]); if (e > 0 && off < 0) off = i; worst = Math.max(worst, e); }
      const d = Math.hypot(path[i][0] - man.pos[0], path[i][1] - man.pos[1], path[i][2] - man.pos[2]); if (d > 0 && off < 0) off = i; worst = Math.max(worst, d); went = Math.max(went, flat(man.pos, p0)); ran ||= v1.player.sprinting; low ||= v1.player.crouch; }
    const fullMs = (now() - t) / N; seat.remote = was;
    const yaw = Math.max(Math.abs(A.aimYaw - man.aimYaw), Math.abs(B.aimYaw - man.aimYaw)), hips = Math.max(Math.abs(A.bodyYaw - man.bodyYaw), Math.abs(B.bodyYaw - man.bodyYaw)), same = worst === 0 && yaw === 0 && hips === 0 && A.crouch === v1.player.crouch && B.crouch === A.crouch && A.vel[0] === man.vel[0] && A.vel[2] === man.vel[2] && B.vel[0] === man.vel[0] && B.vel[2] === man.vel[2];
    say(went > 0.3 && ran && low && A.free && B.free && same && path.length === N && pure && pureB && stays, 'ahead: a look ahead lands where the whole game puts the man, to the bit, and writes nothing',
      `${N} records ahead (a walk, a sprint, a crouch walk back, a coast), and the last ${N - MID} again from the middle of the sprint${v1.player.rifleUp ? ', the rifle in his hands' : ''}: the whole game from the same records took him ${went.toFixed(3)} m out · he ran ${ran}, he crouched ${low} · apart at the worst tick ${worst === 0 ? '0' : `${(worst * 1000).toExponential(2)} mm (first at record ${off})`} · his aim ${yaw === 0 ? '0' : yaw.toExponential(2)} rad off, his hips ${hips === 0 ? '0' : hips.toExponential(2)} · the game's word, the seat and the records as they were ${pure} · no record answers where he stands ${stays}`
      + `\n    cost here: ${N} records ahead ${aheadMs.toFixed(2)} ms · 8 records ${eightMs.toFixed(3)} ms · a whole tick ${fullMs.toFixed(2)} ms`); }
  // ---- host
  { const was = g.localSeat; let inTick = 'no throw';
    g.localSeat = g.seats[1]; const his = g.player === v1.player && g.followPos() === v1.player.char.pos && g.orders === v1.orders;
    g.localSeat = g.seats[0]; const mine = g.player === v0.player && g.orders === v0.orders; g.localSeat = was;
    g.ticking = true; try { void g.player; } catch { inTick = 'throws'; } g.ticking = false;
    say(his && mine && inTick === 'throws' && v0.player !== v1.player, "host: outside a tick the Game's own player is the seat this host shows; inside one there is none",
      `shown seat 1: the Game's player, camera and order book are his ${his} · shown seat 0: his ${mine} · asked inside a tick: ${inTick}`); }
  // ---- down
  { run(0.2); const end0 = String(g.roomClear.state), stage0 = String(g.mission.stage);
    for (let i = 0; i < 8 && !v1.player.down; i++) hitPlayer(v1, [1, 0, 0]);
    const dn = walkAhead(g.seats[1], [cmdFromText(cmdText(sampleCmd(v1, h1.I, h1.cam, P.W.canvas)))], DT), lies = !!v1.player.down && !dn.free && flat(dn.pos, v1.player.char.pos) === 0;
    const down1 = !!v1.player.down, up0 = !v0.player.down, p = [...v0.player.char.pos], pair = v1.orders.squad().filter((m: any) => m.state !== 'dead').length;
    h0.goal = [p[0] - 2, 0, p[2] + 1]; run(2.5); h0.goal = null;
    const on = String(g.roomClear.state) === 'live' && g.mission.stage !== 'failed', walked = flat(v0.player.char.pos, p), pairAfter = v1.orders.squad().filter((m: any) => m.state !== 'dead').length;
    for (let i = 0; i < 8 && !v0.player.down; i++) hitPlayer(v0, [1, 0, 0]);
    run(0.5); const over = String(g.roomClear.state) === 'wiped' || g.mission.stage === 'failed';
    say(end0 === 'live' && down1 && up0 && on && walked > 0.5 && pairAfter === pair && over && lies, 'down: one player down is not the end, both down is',
      `a look ahead answers the downed man where he lies ${lies} · before: room clear ${end0}, mission ${stage0} · seat 1's man down ${down1}, seat 0's up ${up0} · 2.5 s on: room clear ${on ? 'live' : 'ENDED'}, seat 0's man walked ${walked.toFixed(2)} m, the downed man's pair ${pairAfter} of ${pair} still up · both down: ${over ? `ended (${g.roomClear.state})` : 'STILL LIVE'}`); }
  return g;
}

SEAT_ASKS.strict = true; SEAT_ASKS.shared = BREAK; watchViewKeys();
let first: any = null;
try { first = await play(false); } catch (e) { verdict(false, 'the script ran to its end', String((e as Error)?.stack ?? e).split('\n').slice(0, 6).join('\n    ')); }
// ---- twice   (the state's own word, rollback.ts stateHash with `own`: the one that stands between two games of one process)
if (first) { let second: any = null, why = ''; try { second = await play(true); } catch (e) { why = String(e); }
  const a = stateHash(first, false, true), b = second ? stateHash(second, false, true) : why;
  verdict(a === b, 'twice: the same script in a fresh game ends on the same state, to the bit', `${a} | ${b}${second && a !== b ? ` · ${(stateDiff(first, second, 6, false, true) as string[]).join(' | ')}` : ''}`); }
// ---- fight   (a fresh game each: both men walk at the nearest enemy and fire, a key of the game's pressed every second and a half — under `strict`, so one ask
//               of the Game's own player anywhere in the tick ends it)
async function fight(query: string, secs: number, ahead = false): Promise<{ ticks: number; down: number; hurt: number; err: string; words: string[] }> {
  for (const k of Object.keys(WALL_STATS)) if (typeof WALL_STATS[k] === 'number') WALL_STATS[k] = 0;   // (a tally of the process's: coverfire.ts keeps where it stood when a game began, in the game's state — two games of one process start level only from a level tally)
  Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed: 9, seats: 2 }), g = W.game; const r = startFromQuery(g, query); if (r.why) return { ticks: 0, down: 0, hurt: 0, err: r.why, words: [] };
  const v0 = g.seats[0].view, v1 = g.seats[1].view, I1: any = Input.detached(), cam1: any = new FollowCamera(); cam1.aspect = W.cam.aspect; cam1.target = [...v1.player.char.pos]; cam1.rebuild();
  const hands: any[] = [{ v: v0, I: W.input, cam: W.cam, held: null }, { v: v1, I: I1, cam: cam1, held: null }];
  const KEYS = ['KeyG', 'KeyF', 'KeyE', 'KeyQ', 'KeyB', 'KeyC', 'KeyR', 'KeyV', 'KeyT', 'KeyH', 'KeyN', 'KeyX', 'KeyZ', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'Space', 'Tab'];
  const foes0 = g.guards.filter((x: any) => x.state !== 'dead' && !(x.unit && x.team === 'operative')).length; let i = 0, err = ''; const words: string[] = [], late: any[][] = [[], []];
  for (; i < Math.round(secs * 60) && !err; i++) {
    hands.forEach((h, hi) => { const I = h.I; for (const k of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) I.keys.delete(k);
      const p = h.v.player.char.pos, foe = g.guards.filter((x: any) => x.state !== 'dead' && x.team !== 'operative').sort((a: any, b: any) => flat(a.char.pos, p) - flat(b.char.pos, p))[0];
      const t = foe ? foe.char.pos : [p[0] + Math.sin(i / 200 + hi) * 8, 0, p[2] + Math.cos(i / 200 + hi) * 8], dx = t[0] - p[0], dz = t[2] - p[2], d = Math.hypot(dx, dz) || 1;
      const { fwd, right } = h.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / d, b = (dx * right[0] + dz * right[2]) / d, T = 0.3827;
      if (d > 3) { if (a > T) I.keys.add('KeyW'); if (a < -T) I.keys.add('KeyS'); if (b > T) I.keys.add('KeyD'); if (b < -T) I.keys.add('KeyA'); }
      const [sx, sy, front] = h.cam.project([t[0], 1.2, t[2]], W.canvas.clientWidth, W.canvas.clientHeight); if (front) { I.mouseX = sx; I.mouseY = sy; }
      const ph = (i + hi * 37) % 240; if (ph > 120 && ph < 200) { I.buttons |= 1; if (ph === 121) I.clicked |= 1; } else I.buttons &= ~1;
      if ((i + hi * 53) % 90 === 0) { const k = KEYS[Math.floor((i + hi * 53) / 90) % KEYS.length]; I.keys.add(k); I.pressed.add(k); h.held = k; } else if ((i + hi * 53) % 90 === 6 && h.held) { I.keys.delete(h.held); h.held = null; } });
    if (ahead) hands.forEach((h, hi) => { const q = late[hi]; q.push(cmdFromText(cmdText(sampleCmd(h.v, h.I, h.cam, W.canvas)))); if (q.length > 8) q.shift(); walkAhead(g.seats[hi], q, DT); });
    try { g.seats[1].hand(sampleCmd(v1, I1, cam1, W.canvas)); W.step(DT); if (i % 60 === 59) words.push(stateHash(g, false, true)); cam1.update(DT, v1.followPos(), v1.aimPoint); I1.endFrame(); } catch (e) { err = String((e as Error)?.stack ?? e).split('\n').slice(0, 5).join(' | '); }
  }
  return { ticks: i, hurt: g.guards.filter((x: any) => x.state === 'dead').length + men(g).filter((m: any) => m.down).length, down: foes0 - g.guards.filter((x: any) => x.state !== 'dead' && !(x.unit && x.team === 'operative')).length, err, words };
}
for (const [q, secs] of [['roomclear&squad4', 40], ['nightbreach', 30]] as const) {
  let F = { ticks: 0, down: 0, hurt: 0, err: 'did not run', words: [] as string[] }; try { F = await fight(q, secs); } catch (e) { F.err = String(e); }
  verdict(!F.err && F.ticks === secs * 60 && (BREAK || F.hurt > 0), `fight (${q}): ${secs} s of both players in a firefight asks the Game for no "the player"`, F.err ? `after ${F.ticks} ticks: ${F.err}` : `${F.ticks} ticks · ${F.hurt} men down in it, ${F.down} of them the other side's`);
  // ---- twin   (the same fight again in a fresh game, a look ahead for both seats before every tick)
  { let T = { ticks: 0, down: 0, hurt: 0, err: 'did not run', words: [] as string[] }; try { T = await fight(q, secs, true); } catch (e) { T.err = String(e); }
    let part = -1; for (let k = 0; k < Math.max(F.words.length, T.words.length); k++) if (F.words[k] !== T.words[k]) { part = k; break; }
    verdict(!F.err && !T.err && F.words.length === secs && part < 0, `twin (${q}): the fight with a look ahead for both seats every tick is the same game`,
      T.err ? `after ${T.ticks} ticks: ${T.err}` : part < 0 ? `${T.words.length} seconds, the state's word the same at each · it ends on ${T.words[T.words.length - 1]}` : `THE GAMES PART in second ${part + 1}: ${F.words[part]} | ${T.words[part]}`); }
}
// ---- tables   (over everything above: the script, the firefights)
{ const seen = [...VIEW_KEYS.seen].sort(), odd = seen.filter((n: string) => !SEAT_TABLES.includes(n)), raw = [...VIEW_KEYS.raw].filter((n: string) => SEAT_TABLES.includes(n)).sort();
  verdict(odd.length === 0 && raw.length === 0 && seen.length > 0, "tables: a table keyed by the game is a seat's (asked under a view) or the world's (asked under the Game), never both",
    `${seen.length} asked under a view, each listed as a seat's: ${seen.join(', ') || 'none'}${odd.length ? ` · NOT A SEAT'S, ASKED UNDER A VIEW: ${odd.join(', ')}` : ''}${raw.length ? ` · A SEAT'S, ASKED UNDER THE GAME: ${raw.join(', ')}` : ''} · ${VIEW_KEYS.raw.size} asked under the Game`); }
// ---- word
{ const got = [seatsOfSearch('?roomclear&squad4&seats=2'), seatsOfSearch('?roomclear'), seatsOfSearch('?seats=9'), seatsOfSearch('?seats=0'), seatsOfSearch('?xseats=2'), seatsOfSearch(null)];
  verdict(got.join(',') === '2,1,2,1,1,1', 'word: the address says how many seats (?seats=2), and no word is one', `seats=2 → ${got[0]} · none → ${got[1]} · seats=9 → ${got[2]} · seats=0 → ${got[3]} · another word → ${got[4]} · no address → ${got[5]}`); }
// ---- refuses
{ Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed: 9, seats: 2 }); const why = startScenario(W.game, { kind: 'rescue', squad: 2 } as any);
  let ran = true; try { for (let i = 0; i < 30; i++) W.step(DT); } catch { ran = false; }
  verdict(typeof why === 'string' && why.length > 0 && ran && !W.game.rescue.active, 'refuses: a mission written round one commander refuses a game of two', `hostage rescue: ${why ?? 'STARTED'} · the floor still ticks ${ran}`); }

console.log(fails ? `=== seats-check: ${fails} FAILED ===` : '=== seats-check: ALL PASS ===');
process.exit(fails ? 1 : 0);
