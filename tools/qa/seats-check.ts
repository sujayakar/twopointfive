// TWO PLAYERS IN ONE GAME (src/game/seats.ts; `standUp({ seats: 2 })`): is the second seat a second player — his own man, his own pair, his own record — in a
// squad mission played from two seats' records, and is a game of one seat the game it always was? The game that steps is the Rust game (the harness
// stands it: tools/blitz/PLAY.md), and every tick here is `Game.stepSeats(dt, [record, record])`, as a game server's and a client's is.
//   bun run tools/qa/seats-check.ts                  every line below; exit 0 = all pass, 1 = one failed (named)
//   SEATS_BREAK=1 bun run tools/qa/seats-check.ts    the red path: the second seat's tick runs on the FIRST seat's record — one player's hands in two
//                                                    bodies; `walk`, `orders` and `ahead` must FAIL
// The mission is the room clear with the four (the page's `?roomclear&squad4`: fog of war up).
// THE WORD a line holds a game to (`word` below) is the game's own: its die's count and the hash of every part of its block, what a networked game's
// machines compare (src/net/step.ts netWord). It sees where every body, door and thing is, not a seat's aim or a man's mind (docs/agents/net.md).
// WHAT IS HELD:
//   one       a game of one seat: its seat's view IS the Game, the seat owns nothing apart, and `player` is the plain field it always was
//   men       two seats: two men with two ids standing apart on the floor, the four dealt two a seat, each seat's order book holding its own two and no other
//   walk      each man walks by his own record: one east, one west, at once
//   orders    a seat's order reaches its own pair and nobody else's: seat 1 taps the squad key on a door with his cursor (keys and cursor in HIS record) and his
//             two have that door; seat 0's two are as they were — then the other way round
//   fog       the fog of war is the team's: its eyes are both men's and all four teammates'
//   down      one player down is not the end: the mission runs on, the other man still walks, the downed man's pair still stands; both down ends it.
//             RED TODAY: the tools cannot hit the second seat's man (the line says why)
//   host      outside a tick the Game's own `player` (what a HUD, a camera, a panel ask) is the seat this host shows, Game.localSeat; inside one it throws
//   fight     both players walk at the nearest enemy and fire for 40 s of the room clear and 30 s of the night breach, a key of the game's pressed every
//             second and a half: it runs to its end and it was a fight (somebody went down)
//   long      the same hands for 150 s of the room clear on another die, and on the compound
//   tables    over all of the above (math/side.ts watchViewKeys): a named table of the picture's side keyed by the game was asked under a seat's VIEW only
//             if it is listed as a seat's own (seats.ts SEAT_TABLES), and no listed one was asked under the Game itself
//   word      `?seats=2` in an address is two seats (seatsOfSearch: what standGame reads), no word is one
//   rollback  a moment mid-fight is taken (savestate.ts SNAPS.take: the Rust game kept), ninety ticks are played from both seats' kept records, the moment
//             is put back: the word is the moment's, and the same records play the same ninety ticks, word for word
//   twice     the whole script played again in a fresh game ends on the same word
//   ahead     a look ahead (walkahead.ts walkAhead: one seat's man walked through records the game has not had, on a copy of him) lands where the whole game
//             then puts him from the same records, tick by tick and to the bit (a walk, a sprint, a crouch walk, a coast, from a seat marked remote; and again from
//             the middle of the sprint); the game's word, the seat and the records are as they were; no record answers where he stands, and so does a
//             man who is down. Prints what it costs beside a whole tick
//   twin      a firefight played twice, each in a fresh process — plain, and with a look ahead for both seats before every tick (the last eight records, throws and
//             rounds in flight among them) — is the same game: the word the same every second and at the end
//   refuses   a mission written round one commander (the hostage rescue) refuses a game of two, in words
// NOT HELD HERE ANY MORE (2026-11-07): the count of asks of the Game for "the player" inside a tick (the TypeScript tick's, gone with it). The three
// kinds of fault a game of two had (docs/agents/net.md, the known seat faults) are the Rust game's to hold: tools/blitz/coop-check.ts.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
const { SEAT_ASKS, SEAT_TABLES, men, seatsOfSearch } = await import(`${ROOT}/src/game/seats.ts`);
const { VIEW_KEYS, watchViewKeys } = await import(`${ROOT}/src/math/side.ts`);
const { sampleCmd, cmdText, cmdFromText } = await import(`${ROOT}/src/game/cmd.ts`);
const { Input } = await import(`${ROOT}/src/game/input.ts`);
const { FollowCamera } = await import(`${ROOT}/src/game/camera.ts`);
const { startFromQuery, startScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { SNAPS } = await import(`${ROOT}/src/game/savestate.ts`);
const CROSSING = await import(`${ROOT}/src/game/crossing.ts`);
const { walkAhead } = await import(`${ROOT}/src/game/walkahead.ts`);
const { WALL_STATS } = await import(`${ROOT}/src/game/wallbreak.ts`);
const { hitPlayer } = await import(`${ROOT}/src/game/combat.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const BREAK = process.env.SEATS_BREAK === '1', DT = 1 / 60;
/** the game's own word for where it stands (the head says what it sees) */
const word = (g: any): string => { const W = g.stateWords?.(); if (!W) throw new Error('seats-check: the game stood here answers no state words (no Rust game steps it)'); return [W.die, ...Array.from(W.parts as ArrayLike<number>)].map(x => x.toString(36)).join('.'); };
/** one tick on both seats' records, and what a headless host does round it (tools/qa/seats-run.ts tick) */
const tick = (g: any, c: any[]): void => { g.stepSeats(DT, BREAK ? [c[0], c[0]] : c); g.engine.lights.update(DT, g.time); g.engine.world.dynamics = g.dynBoxes; g.engine.world.upload(); };
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
console.log(`=== seats-check: two players in one game (src/game/seats.ts)${BREAK ? ' · SEATS_BREAK: the second seat runs on the first seat\'s record' : ''} ===`);
const flat = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const at = (p: ArrayLike<number>) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;

// ---- a fight in a process of its own (`--fight '<json>'`): what `fight`, `twin` and `long` below hold to each other. A game stood after another in one
// process does not start as the first did (found 2026-11-07 with tools/mp/words-diff.ts: the modules holdfire, nvg, kit and guards carry something of the
// last game into the next one's start), so two games that are to be the same game are each stood in a fresh process.
if (process.argv[2] === '--fight') { const [q, secs, ahead, seed, level] = JSON.parse(process.argv[3]); const log = console.log; console.log = () => {};
  let F: unknown; try { F = await fight(q, secs, ahead, seed, level); } catch (e) { F = { ticks: 0, down: 0, hurt: 0, err: String((e as Error)?.stack ?? e).split('\n').slice(0, 4).join(' | '), words: [] }; }
  log('SEATSFIGHT ' + JSON.stringify(F)); process.exit(0); }

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
  const step = () => { for (const h of hands) drive(h); tick(g, hands.map(h => sampleCmd(h.v, h.I, h.cam, W.canvas))); for (const h of hands) { h.cam.update(DT, h.v.followPos(), h.v.aimPoint); h.I.endFrame(); } };
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
  { h0.goal = [v0.player.char.pos[0], 0, v0.player.char.pos[2] + 3]; h1.goal = [v1.player.char.pos[0], 0, v1.player.char.pos[2] - 3]; run(0.5);
    const S = SNAPS.take(g), w0 = word(g), N = 90, kept: any[][] = [], ref: string[] = [];
    for (let i = 0; i < N; i++) { const c = P.cmds(); kept.push(c.map(x => cmdFromText(cmdText(x)))); tick(g, c); ref.push(word(g)); }
    const went = ref[N - 1] !== w0;
    SNAPS.put(S); const back = word(g) === w0;
    let same = -1; for (let i = 0; i < N; i++) { tick(g, kept[i]); if (word(g) !== ref[i]) { same = i; break; } }
    h0.goal = h1.goal = null;
    say(back && same < 0 && went, 'rollback: a moment of a game of two is put back, and the same records play the same ticks',
      `the word moved over ${N} ticks ${went} · put back to the moment's word ${back} · ${N} ticks from the kept records ${same < 0 ? 'the same, word for word' : `DIFFER at tick ${same}`}`); }
  // ---- host
  { const was = g.localSeat; let inTick = 'no throw';
    g.localSeat = g.seats[1]; const his = g.player === v1.player && g.followPos() === v1.player.char.pos && g.orders === v1.orders;
    g.localSeat = g.seats[0]; const mine = g.player === v0.player && g.orders === v0.orders; g.localSeat = was;
    g.ticking = true; try { void g.player; } catch { inTick = 'throws'; } g.ticking = false;
    say(his && mine && inTick === 'throws' && v0.player !== v1.player, "host: outside a tick the Game's own player is the seat this host shows; inside one there is none",
      `shown seat 1: the Game's player, camera and order book are his ${his} · shown seat 0: his ${mine} · asked inside a tick: ${inTick}`); }
  // ---- down
  { run(0.2); const end0 = String(g.roomClear.state), stage0 = String(g.mission.stage);
    // (RED TODAY, 2026-11-07, and the tools' call surface's to mend: a rule a tool calls with a SEAT'S VIEW crosses to the Rust game as the seat that
    //  stands on the Game, so the hit meant for seat 1's man lands on seat 0's. The line is right as written for a surface that runs the rule as the seat
    //  whose view it is handed)
    for (let i = 0; i < 8 && !v1.player.down; i++) hitPlayer(v1, [1, 0, 0]);
    const wrongMan = !v1.player.down && !!v0.player.down;
    const dn = walkAhead(g.seats[1], [cmdFromText(cmdText(sampleCmd(v1, h1.I, h1.cam, P.W.canvas)))], DT), lies = !!v1.player.down && !dn.free && flat(dn.pos, v1.player.char.pos) === 0;
    const down1 = !!v1.player.down, up0 = !v0.player.down, p = [...v0.player.char.pos], pair = v1.orders.squad().filter((m: any) => m.state !== 'dead').length;
    h0.goal = [p[0] - 2, 0, p[2] + 1]; run(2.5); h0.goal = null;
    const on = String(g.roomClear.state) === 'live' && g.mission.stage !== 'failed', walked = flat(v0.player.char.pos, p), pairAfter = v1.orders.squad().filter((m: any) => m.state !== 'dead').length;
    for (let i = 0; i < 8 && !v0.player.down; i++) hitPlayer(v0, [1, 0, 0]);
    run(0.5); const over = String(g.roomClear.state) === 'wiped' || g.mission.stage === 'failed';
    say(end0 === 'live' && down1 && up0 && on && walked > 0.5 && pairAfter === pair && over && lies, 'down: one player down is not the end, both down is',
      `${wrongMan ? "THE HIT HANDED SEAT 1'S VIEW DOWNED SEAT 0'S MAN (a tool's call does not cross as the seat whose view it is handed) · " : ''}a look ahead answers the downed man where he lies ${lies} · before: room clear ${end0}, mission ${stage0} · seat 1's man down ${down1}, seat 0's up ${up0} · 2.5 s on: room clear ${on ? 'live' : 'ENDED'}, seat 0's man walked ${walked.toFixed(2)} m, the downed man's pair ${pairAfter} of ${pair} still up · both down: ${over ? `ended (${g.roomClear.state})` : 'STILL LIVE'}`); }
  return word(g);
}

SEAT_ASKS.strict = true; watchViewKeys();   // (strict: the TypeScript accessors of a seat's fields throw inside a tick: the `host` line's)
let first: string | null = null;
try { first = await play(false); } catch (e) { verdict(false, 'the script ran to its end', String((e as Error)?.stack ?? e).split('\n').slice(0, 6).join('\n    ')); }
// ---- twice
if (first !== null) { let second = ''; try { second = await play(true); } catch (e) { second = String(e); }
  verdict(first === second, 'twice: the same script in a fresh game ends on the same word', first === second ? `${first.slice(0, 60)}…` : `${first.slice(0, 80)} | ${second.slice(0, 80)}`); }

// ---- ahead   (a game of two with NO teammates on the floor: a look ahead holds everybody else where the tick has them, so a teammate who steps into his
// way is the game's to say and no look's. Both looks are held: the Rust rule a tool's call crosses to, and the TypeScript body the page runs on its copy)
{ Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed: 9, seats: 2 }), g = W.game; const r = startFromQuery(g, 'roomclear&squad=0');
  if (r.why) verdict(false, 'ahead: a look ahead lands where the whole game puts the man, to the bit, and writes nothing', `the game without teammates did not start: ${r.why}`);
  else {
    for (const st of g.seats) st.remote = true;   // (as a networked game's seats are from its start: each view is worked out again from its pose)
    const v0 = g.seats[0].view, v1 = g.seats[1].view, I1: any = Input.detached(), cam1: any = new FollowCamera(); cam1.aspect = W.cam.aspect; cam1.target = [...v1.player.char.pos]; cam1.rebuild(); I1.mouseX = 800; I1.mouseY = 500;
    const hands = [{ v: v0, I: W.input, cam: W.cam }, { v: v1, I: I1, cam: cam1 }], cv = W.canvas, now = performance.now.bind(performance);
    const lens = () => { for (const h of hands) { h.cam.update(DT, h.v.followPos(), h.v.aimPoint); h.I.endFrame(); } };
    for (let i = 0; i < 110; i++) { tick(g, hands.map(h => cmdFromText(cmdText(sampleCmd(h.v, h.I, h.cam, cv))))); lens(); }
    const seat = g.seats[1], man = v1.player.char, N = 46, MID = 20, moves = ['KeyW', 'KeyA', 'KeyS', 'KeyD', 'ShiftLeft', 'KeyC'];
    // the records a client would have sent and not had answered: a walk, a sprint, C and a crouch walk the other way, Shift in the crouch, a coast
    const held = (i: number): string[] => i < 12 ? ['KeyD'] : i < 26 ? ['KeyD', 'KeyW', 'ShiftLeft'] : i < 38 ? ['KeyA', 'KeyS'] : i < 42 ? ['KeyA', 'KeyS', 'ShiftLeft'] : [];
    const recs: any[] = []; for (let i = 0; i < N; i++) { for (const k of moves) I1.keys.delete(k); for (const k of held(i)) I1.keys.add(k); if (i === 26) { I1.keys.add('KeyC'); I1.pressed.add('KeyC'); } recs.push(cmdFromText(cmdText(sampleCmd(v1, I1, cam1, cv)))); I1.pressed.delete('KeyC'); }
    for (const k of moves) I1.keys.delete(k);
    const still = cmdFromText(cmdText(sampleCmd(v0, W.input, W.cam, cv)));
    /** the look by the TypeScript body, as a page runs it (no call surface there: crossing.ts) */
    const body = (cmds: any[], path?: number[][]): any => { const S = CROSSING.RUST_SURFACE; CROSSING.setSurface(null); try { return walkAhead(seat, cmds, DT, path as never); } finally { CROSSING.setSurface(S); } };
    const p0 = [...man.pos], text0 = recs.map(cmdText).join('|'), w0 = word(g), pathR: number[][] = [], pathT: number[][] = [];
    const none = walkAhead(seat, [], DT), stays = flat(none.pos, p0) === 0 && none.pos !== man.pos;
    let t = now(); const A = walkAhead(seat, recs, DT, pathR as never); const crossedMs = now() - t;
    t = now(); const T = body(recs, pathT); const bodyMs = now() - t;
    const pure = word(g) === w0 && recs.map(cmdText).join('|') === text0 && flat(man.pos, p0) === 0;
    let worstR = 0, worstT = 0, off = -1, ran = false, low = false, went = 0, B: any = null, Bt: any = null, pureB = true; const pathB: number[][] = [], pathBt: number[][] = []; t = now();
    for (let i = 0; i < N; i++) {
      if (i === MID) { const w = word(g); B = walkAhead(seat, recs.slice(MID), DT, pathB as never); Bt = body(recs.slice(MID), pathBt); pureB = word(g) === w && v1.player.sprinting; }   // (a second look, begun in the middle of the sprint)
      tick(g, [still, recs[i]]); lens();
      const e = (q: number[]) => Math.hypot(q[0] - man.pos[0], q[1] - man.pos[1], q[2] - man.pos[2]);
      const dR = Math.max(e(pathR[i]), i >= MID ? e(pathB[i - MID]) : 0), dT = Math.max(e(pathT[i]), i >= MID ? e(pathBt[i - MID]) : 0);
      if ((dR > 0 || dT > 0) && off < 0) off = i; worstR = Math.max(worstR, dR); worstT = Math.max(worstT, dT); went = Math.max(went, flat(man.pos, p0)); ran ||= v1.player.sprinting; low ||= v1.player.crouch; }
    const fullMs = (now() - t) / N;
    const end = (L: any) => Math.abs(L.aimYaw - man.aimYaw) === 0 && Math.abs(L.bodyYaw - man.bodyYaw) === 0 && L.crouch === v1.player.crouch && L.vel[0] === man.vel[0] && L.vel[2] === man.vel[2];
    const same = worstR === 0 && worstT === 0 && end(A) && end(T) && end(B) && end(Bt);
    verdict(went > 0.3 && ran && low && A.free && T.free && B.free && Bt.free && same && pathR.length === N && pathT.length === N && pure && pureB && stays, 'ahead: a look ahead lands where the whole game puts the man, to the bit, and writes nothing',
      `${N} records ahead (a walk, a sprint, a crouch walk back, a coast), and the last ${N - MID} again from the middle of the sprint, each by the Rust rule and by the TypeScript body: the whole game from the same records took him ${went.toFixed(3)} m out · he ran ${ran}, he crouched ${low} · apart at the worst tick: the Rust rule ${(worstR * 1000).toExponential(2)} mm, the TypeScript body ${(worstT * 1000).toExponential(2)} mm${off >= 0 ? ` (first at record ${off})` : ''} · his aim, his hips, his crouch and his speed at the end as each look said ${end(A) && end(T) && end(B) && end(Bt)} · the game's word and the records as they were ${pure}, and from the middle ${pureB} · no record answers where he stands ${stays}`
      + `\n    cost here: ${N} records ahead ${crossedMs.toFixed(2)} ms crossed to the Rust rule, ${bodyMs.toFixed(2)} ms by the TypeScript body · a whole tick ${fullMs.toFixed(2)} ms`); } }

// ---- the fights: both players walk at the nearest enemy and fire, a key of the game's every second and a half (tools/qa/seats-run.ts has the same hands)
async function fight(query: string, secs: number, ahead = false, seed = 9, level: string | null = null): Promise<{ ticks: number; down: number; hurt: number; err: string; words: string[] }> {
  for (const k of Object.keys(WALL_STATS)) if (typeof WALL_STATS[k] === 'number') WALL_STATS[k] = 0;   // (a tally of the process's: coverfire.ts keeps where it stood when a game began, in the game's state — two games of one process start level only from a level tally)
  Math.random = mulberry32(0xc0de); const W: any = await standUp({ seed, seats: 2, ...(level ? { level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${level}.json`, 'utf8'))) } : {}) }), g = W.game; const r = startFromQuery(g, query); if (r.why) return { ticks: 0, down: 0, hurt: 0, err: r.why, words: [] };
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
    try { const cmds = hands.map(h => cmdFromText(cmdText(sampleCmd(h.v, h.I, h.cam, W.canvas))));
      if (ahead) hands.forEach((_h, hi) => { const q = late[hi]; q.push(cmds[hi]); if (q.length > 8) q.shift(); walkAhead(g.seats[hi], q, DT); });
      tick(g, cmds); if (i % 60 === 59) words.push(word(g)); for (const h of hands) { h.cam.update(DT, h.v.followPos(), h.v.aimPoint); h.I.endFrame(); } } catch (e) { err = String((e as Error)?.stack ?? e).split('\n').slice(0, 5).join(' | '); }
  }
  return { ticks: i, hurt: g.guards.filter((x: any) => x.state === 'dead').length + men(g).filter((m: any) => m.down).length, down: foes0 - g.guards.filter((x: any) => x.state !== 'dead' && !(x.unit && x.team === 'operative')).length, err, words };
}
/** a fight's line: it ran to its end and it was a fight */
function fightLine(name: string, q: string, secs: number, F: { ticks: number; down: number; hurt: number; err: string }) {
  verdict(!F.err && F.ticks === secs * 60 && (BREAK || F.hurt > 0), `${name} (${q}): ${secs} s of both players in a firefight runs to its end`,
    F.err ? `after ${F.ticks} ticks: ${F.err}` : `${F.ticks} ticks · ${F.hurt} men down in it, ${F.down} of them the other side's`);
}
const NONE = () => ({ ticks: 0, down: 0, hurt: 0, err: 'did not run', words: [] as string[] });
/** a fight in a fresh process (the head of this file's `--fight`) */
const apart = (q: string, secs: number, ahead = false, seed = 9, level: string | null = null): ReturnType<typeof NONE> => {
  const r = Bun.spawnSync([process.execPath, 'run', import.meta.path, '--fight', JSON.stringify([q, secs, ahead, seed, level])], { env: process.env, stdout: 'pipe', stderr: 'pipe' });
  const out = new TextDecoder().decode(r.stdout), l = out.split('\n').find(x => x.startsWith('SEATSFIGHT '));
  return l ? JSON.parse(l.slice(11)) : { ...NONE(), err: `the fight's process said nothing (exit ${r.exitCode}): ${new TextDecoder().decode(r.stderr).trim().split('\n').slice(-2).join(' | ').slice(0, 300)}` }; };
for (const [q, secs] of [['roomclear&squad4', 40], ['nightbreach', 30]] as const) {
  const F = apart(q, secs);
  fightLine('fight', q, secs, F);
  // ---- twin   (the same fight again in a fresh process, a look ahead for both seats before every tick)
  { const T = apart(q, secs, true);
    let part = -1; for (let k = 0; k < Math.max(F.words.length, T.words.length); k++) if (F.words[k] !== T.words[k]) { part = k; break; }
    verdict(!F.err && !T.err && F.words.length === secs && part < 0, `twin (${q}): the fight with a look ahead for both seats every tick is the same game`,
      T.err ? `after ${T.ticks} ticks: ${T.err}` : part < 0 ? `${T.words.length} seconds, the game's word the same at each` : `THE GAMES PART in second ${part + 1}`); }
}
// ---- long   (two and a half minutes each, on another die and on another floor)
for (const [q, secs, seed, level] of [['roomclear&squad4', 150, 5, null], ['roomclear&squad4', 150, 3, 'compound']] as const) {
  const F = apart(q, secs, false, seed, level);
  fightLine('long', `${q}, ${level ?? 'the page\'s own floor'}, die ${seed}`, secs, F);
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
