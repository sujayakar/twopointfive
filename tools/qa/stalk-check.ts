// 'stalk' — THE STALK (STALK, src/game/stalk.ts). The maintainer's design for the last metres to a man's back: "when you're close enough to an enemy … you can
// click and hold on an enemy and sam will creep toward him · click, let go to stop, and click again to continue · the game also controls the trajectory and speed
// and makes things smooth. it ends with you grabbing him · you get kicked out of the mode if he gets too far away, turns around, goes through a door".
//   bun run tools/qa/stalk-check.ts        (also `probes.ts stalk`; run it on BOTH stands: TPS_PROFILE=shipped in front for the second)
// The rule is settled on, and on the page the click only brings him in and the row's key is the grab: the cards of the hold that ends in the grab by itself (grab,
// still, steer, let, armed, out, tell, stepin, shape, legs, catch, lands), the comparisons with a key down (off, the low creep's against the pass before) and the
// harness card asked a build from before that and went.
//   key       the held click brings him in, in reach, and stops; he keeps station on a man walking away; the row's key is the grab, the stalk's own step in.
//   low       closing he sinks and no arm comes up; let go, the pose stands still part way.
//   shipped   what ends it with the button down, the button let go in reach, the wider way out, a man walked up on standing.
//   corner    his back on a wall, hands empty, the button held (STALK.corner): a man walking past is grabbed once he is past, by the same step in; a man facing him
//             is not taken.
// STALK_BREAK=1 = the key, the low creep and the corner left down where they are asked for up: each must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: nothing here says how it looks or feels. tools/qa/stalk-page.cjs drives the real page on the software adapter.
import { standUp, ROOT } from './headless.ts';

const { STALK, STALK_STATS, stalkOn, stalkPolish } = await import(`${ROOT}/src/game/stalk.ts`);
const { CLOSE } = await import(`${ROOT}/src/game/close.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);

type P3 = number[];
const BREAK = process.env.STALK_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, SAM: P3 = [6.2, 0, 19.5], SEEDS = [4411, 1, 2];   // (the lobby's open floor, some 15 m of it east of him)
const f2 = (n: number) => n.toFixed(2);
const f1 = (n: number) => n.toFixed(1);
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const up = () => !BREAK;

interface Ctx { W: any; g: any; A: any; walk: number; f: number; top: number; jerk: number; step(): void; press(): void; lift(): void; held(): boolean; gap(): number; astern(): number }
/** the lobby with the floor's men parked far off, one man `gap` m east of Sam facing east (walking away at `walk` m/s, moved by the check: the AI is off), Sam
 *  crouched with empty hands `bearing` degrees off the man's facing (180 = dead astern) */
async function world(on: boolean, seed: number, gap: number, walk: number, bearing = 180, shift: [number, number] = [0, 0]): Promise<Ctx> {
  Math.random = seeded(seed); const W: any = await quiet(() => standUp({ seed } as any)), g = W.game; CLOSE.on = true; STALK.on = on; stalkPolish(true); STALK.key = false; STALK.low = false; STALK.orbit = false;   // (STALK.orbit is down too: these cards are of a move key that ends the creep — stalkorbit-check has the keys that go round him. the frozen stand's pins write all of them at every stand-up: the polish is down there, and this check is of the stalk with it. The tenth pass's two keys are DOWN for every card but their own, `key` and `low`: the cards before them are of the hold that ends in the grab by itself and of the arm that comes up)
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true; g.player.slot = 'pistol'; g.player.holstered = true;
  const [A, ...rest] = g.guards; for (const [i, gd] of rest.entries()) { gd.char.pos = [37.5, 0, 4.5 + i * 1.5]; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.heardUpTo = 1e12; }
  const a = EAST + bearing * Math.PI / 180, at: P3 = [SAM[0] + shift[0] + gap, 0, SAM[2] + shift[1]];   // (shift: the whole scene moved that far [x, z] across the lobby)
  A.char.pos = [...at]; A.char.bodyYaw = A.char.aimYaw = EAST; A.char.vel = [walk, 0, 0]; A.path = []; A.pathGoal = null; A.hold = true; A.pinned = true; A.awareness = 0; A.heardUpTo = 1e12;
  W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([at[0] + Math.sin(a) * gap, 0, at[2] + Math.cos(a) * gap]); g.player.crouch = true;
  const pl = g.player.char; let v0 = 0;
  const C: Ctx = { W, g, A, walk, f: 0, top: 0, jerk: 0,
    step: () => { if (C.walk > 0 && !A.held) { A.char.pos[0] += C.walk * DT; A.char.vel = [C.walk, 0, 0]; } else A.char.vel = [0, 0, 0];
      W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); W.step(DT); g.player.visibility = 0.02; C.f++;
      const v = Math.hypot(pl.vel[0], pl.vel[2]); C.top = Math.max(C.top, v); if (!g.player.holding) C.jerk = Math.max(C.jerk, Math.abs(v - v0)); v0 = v; },   // (the grab's own frame is the grab's: he and the man stop together)
    press: () => { W.input.buttons |= 1; W.input.clicked |= 1; }, lift: () => { W.input.buttons &= ~1; },
    held: () => g.player.holding?.g === A, gap: () => Math.hypot(pl.pos[0] - A.char.pos[0], pl.pos[2] - A.char.pos[2]),
    astern: () => { const b = Math.atan2(pl.pos[0] - A.char.pos[0], pl.pos[2] - A.char.pos[2]) - A.char.bodyYaw; return Math.abs(Math.atan2(Math.sin(b - Math.PI), Math.cos(b - Math.PI))) * 180 / Math.PI; } };
  for (let i = 0; i < 20; i++) { const w = C.walk; C.walk = 0; C.step(); C.walk = w; }   // (settle: he is crouched and still before the press)
  C.top = 0; C.jerk = 0; return C;
}
/** Sam put on the nearest wall of the lobby with his hands empty (the wall hold, [Q]) and the man set walking past him along it, a metre out: the context, the
 *  wall's normal and the way he walks along it (he starts `back` m short of level with Sam) — or null if no wall took him */
async function onWall(seed: number, back: number): Promise<{ C: Ctx; n: P3; t: P3 } | null> {
  const C = await world(true, seed, 6, 0), g = C.g, pl = g.player, W = C.W; let hit: { t: number; d: P3 } | null = null;
  for (let k = 0; k < 16; k++) { const a = k * Math.PI / 8, d: P3 = [Math.sin(a), 0, Math.cos(a)], h = g.col.raycast([SAM[0], 1.0, SAM[2]], d, 14); if (h && (!hit || h.t < hit.t)) hit = { t: h.t, d }; }
  if (!hit) return null;
  const foot: P3 = [SAM[0] + hit.d[0] * (hit.t - 0.6), 0, SAM[2] + hit.d[2] * (hit.t - 0.6)]; g.teleportPlayer(foot); pl.crouch = false;
  const step = (n: number) => { for (let i = 0; i < n; i++) { W.cursorAt([foot[0] + hit!.d[0] * 2, 1.2, foot[2] + hit!.d[2] * 2]); W.step(DT); pl.visibility = 0.02; } };
  step(10); W.input.pressed.add('KeyQ'); step(1); step(50); if (!pl.wall) return null;
  const n: P3 = [pl.wall.n[0], 0, pl.wall.n[2]], t0: P3 = [n[2], 0, -n[0]], p = pl.char.pos, A = C.A;
  // the side he comes from: the one with a clear run along the wall past Sam (the lobby's furniture stands on one of them)
  const run = (sg: number) => { const from: P3 = [p[0] + n[0] * 0.75 - t0[0] * back * sg, 0.5, p[2] + n[2] * 0.75 - t0[2] * back * sg]; return g.col.raycast(from, [t0[0] * sg, 0, t0[2] * sg], back + 1.6)?.t ?? 99; };
  const sg = run(1) >= run(-1) ? 1 : -1, t: P3 = [t0[0] * sg, 0, t0[2] * sg];
  A.char.pos = [p[0] + n[0] * 0.75 - t[0] * back, 0, p[2] + n[2] * 0.75 - t[2] * back]; A.char.bodyYaw = A.char.aimYaw = Math.atan2(t[0], t[2]);
  return { C, n, t };
}
async function corner() {
  console.log('\n=== corner: with his back on a wall the held button lies in wait, and the man who comes past is grabbed as he passes ===');
  const pass = async (hold: boolean, on: boolean) => {
    const S = await onWall(SEEDS[0], 2.6); if (!S) return null; const { C, t } = S, g = C.g, pl = g.player, A = C.A; STALK.corner = on; const c0 = STALK_STATS.corner;
    CLOSE.noKnife = true;   // (the wait's pose is a bare-handed one: the frozen stand has the knife, and every stand-up writes this again)
    let at = { deg: -1, d: -1 }, early = false, from = false, coil = 0, low = 0, r0 = -1, r1 = 0;   // (the wait's pose: how far he is sunk — asked, and shown —, and the lead hand's reach at the start and at its most)
    for (let i = 0; i < 5 * 60 && !C.held(); i++) {
      A.char.pos[0] += t[0] * 1.15 * DT; A.char.pos[2] += t[2] * 1.15 * DT; A.char.vel = [t[0] * 1.15, 0, t[2] * 1.15];
      const b = Math.atan2(pl.char.pos[0] - A.char.pos[0], pl.char.pos[2] - A.char.pos[2]) - A.char.bodyYaw, deg = Math.abs(Math.atan2(Math.sin(b), Math.cos(b))) * 180 / Math.PI, d = C.gap();
      if (hold) { C.W.input.buttons |= 1; if (i === 0) C.W.input.clicked |= 1; }
      C.W.step(DT); pl.visibility = 0.02;
      if (C.held()) { at = { deg, d }; early = deg < STALK.cornerDeg - 3; from = !!pl.holding?.from; }
      else { const an = pl.char.anim; coil = Math.max(coil, an.coil); if (i > 30) low = Math.max(low, an.crouch); if (an.ready === 'grab' && an.coil > 0) { if (r0 < 0) r0 = an.readyAmt; r1 = Math.max(r1, an.readyAmt); } }
    }
    return { held: C.held(), at, early, from, wall: !!pl.wall, n: STALK_STATS.corner - c0, coil, low, r0, r1 };
  };
  const A = await pass(true, up());
  if (!A) { verdict(false, 'the lobby has a wall Sam can put his back on', 'no wall took him: the check could not stand its scene'); return; }
  verdict(A.held && !A.early && A.at.d < STALK.cornerM + 0.05 && A.from && !A.wall && A.n === 1, 'on a wall with empty hands and the button held from the start, a man walking past a pace out is in his arm — not before Sam is STALK.cornerDeg round from his nose (he is past), inside the corner\'s reach, off the wall by the step in',
    `in his arm ${A.held}${A.held ? ` with Sam ${A.at.deg.toFixed(0)}° round from his nose, ${f2(A.at.d)} m off` : ''} · the step in ${A.from} · still on the wall ${A.wall} · corner grabs ${A.n}`);
  const S = await onWall(SEEDS[0], 1.0);   // a man who stands a pace short of him along the wall, facing him
  if (S) { const { C } = S, pl = C.g.player.char; S.C.A.char.bodyYaw = S.C.A.char.aimYaw = Math.atan2(pl.pos[0] - C.A.char.pos[0], pl.pos[2] - C.A.char.pos[2]); STALK.corner = true;
    for (let i = 0; i < 120; i++) { C.W.input.buttons |= 1; C.W.step(DT); C.g.player.visibility = 0.02; }
    verdict(!C.held(), 'a man who stands a pace off FACING him is not taken, the button held two seconds', `in his arm ${C.held()} · ${f2(C.gap())} m off`); }
}
/** THE KEY FINISHES IT (STALK.key, the tenth pass): the held click only brings him in. The grab is the row's key, and that grab is the stalk's. */
async function keyCard() {
  console.log('\n=== key: the held click brings him in and stops, and the row\'s key is the grab ===');
  const grabRow = (C: Ctx) => C.g.nearby.some((r: any) => r.kind === 'guard' && r.verb === 'grab' && r.guard === C.A);
  const take = (C: Ctx) => { const g = C.g, ok = g.selectVerb('guard', 'grab'); C.W.input.pressed.add('Space'); let n = 0; while (!C.held() && n++ < 40) C.step(); return ok; };
  const run = async (key: boolean, walk: number, secs: number, gap = 2.6) => {
    const C = await world(true, SEEDS[0], gap, walk); CLOSE.noKnife = true; STALK.key = key; const g0 = STALK_STATS.grabs, pl = C.g.player.char;
    C.press(); let alone = false, far = 0, near = 9, slow = 9, drift = 0;
    for (let i = 0; i < secs * 60; i++) { C.step(); if (C.held()) { alone = true; break; }
      if (i >= secs * 60 - 90) { far = Math.max(far, C.gap()); near = Math.min(near, C.gap()); slow = Math.min(slow, Math.abs(Math.hypot(pl.vel[0], pl.vel[2]) - walk)); drift = Math.max(drift, Math.abs(Math.hypot(pl.vel[0], pl.vel[2]) - walk)); } }
    const row = !alone && grabRow(C), on = !alone && stalkOn(C.g) === C.A, astern = alone ? 0 : C.astern();
    let took = false, from = false, hit = false, grabs = STALK_STATS.grabs - g0;
    if (!alone) { took = take(C) && C.held(); from = !!C.g.player.holding?.from; grabs = STALK_STATS.grabs - g0; for (let i = 0; i < 60 && C.held(); i++) { C.step(); hit ||= C.g.player.holding?.hitAt !== undefined; } }
    return { alone, far, near, drift, row, on, astern, took, from, hit, grabs, held: C.held() };
  };
  const A = await run(up(), 0, 7), W = await run(up(), 0.9, 4, 1.4);   // (from 1.4 m, for four seconds: further along, the lobby's floor has things in his way)
  verdict(!A.alone && A.far < STALK.grabM && A.near > STALK.keyM - 0.02 && A.drift < 0.03 && A.row && A.on && A.astern < 25,
    'from 2.6 m behind a man who stands, the button held for seven seconds with no key: nobody is grabbed. He ends still, inside the grab\'s reach and behind him, the stalk still his, with the row\'s grab offered',
    `grabbed by itself ${A.alone} · over the last second and a half: ${f2(A.near)}‥${f2(A.far)} m from him (the grab's reach ${f2(STALK.grabM)}), the most he moved ${A.drift.toFixed(3)} m/s · ${f1(A.astern)}° off dead astern · the stalk on him ${A.on} · the row offered ${A.row}`);
  verdict(A.took && A.from && A.hit && A.grabs === 1,
    'the row\'s key then: he is in the arm, and the grab is the stalk\'s own (it steps in from where he stood, it lands, and it is counted once)',
    `in his arm ${A.took} · a step in from where he stood ${A.from} · it landed ${A.hit} · the stalk's grabs ${A.grabs}`);
  verdict(!W.alone && W.far < STALK.grabM && W.near > STALK.keyM - 0.05 && W.drift < 0.1 && W.row && W.took && W.from && W.grabs === 1,
    '1.4 m behind a man walking away at 0.9 m/s, the button held for four seconds: nobody is grabbed, he keeps station in reach at the man\'s own pace, and the key takes him',
    `grabbed by itself ${W.alone} · over the last second and a half: ${f2(W.near)}‥${f2(W.far)} m, his speed within ${f2(W.drift)} m/s of the man's · the row offered ${W.row} · the key: in his arm ${W.took}, the stalk's ${W.from}, counted ${W.grabs}`);
  // by his own keys: no click at all. Stood in reach behind him, the key's grab is the stalk's too
  const K = await (async () => { const C = await world(true, SEEDS[0], 0.85, 0); CLOSE.noKnife = true; STALK.key = up(); for (let i = 0; i < 10; i++) C.step(); const g0 = STALK_STATS.grabs, row = grabRow(C), took = take(C) && C.held(); return { row, took, from: !!C.g.player.holding?.from, grabs: STALK_STATS.grabs - g0 }; })();
  verdict(K.row && K.took && K.from && K.grabs === 1, 'walked in on his own keys (no click at all) and stood in reach behind him, the key\'s grab is the same one: the stalk\'s step in, counted once',
    `the row offered ${K.row} · in his arm ${K.took} · the stalk's ${K.from} · counted ${K.grabs}`);
}

/** AS THE PAGE SHIPS IT (the tenth pass's two keys both up): what ends a stalk, and what is drawn and offered, with the key finishing the grab from the low creep —
 *  the cards before this one run with both keys down (they read the passes before). */
async function shippedCard() {
  console.log('\n=== shipped: with the key and the low creep both up — what ends it, the wider way out, the line that says what the click does, and a man walked up on standing ===');
  const mk = async (gap: number) => { const C = await world(true, SEEDS[0], gap, 0); CLOSE.ready = true; CLOSE.noKnife = true; STALK.key = up(); STALK.low = up(); return C; };
  const inReach = async () => { const C = await mk(2.6); C.press(); for (let i = 0; i < 7 * 60; i++) C.step(); return C; };
  const speed = (C: Ctx) => Math.hypot(C.g.player.char.vel[0], C.g.player.char.vel[2]);
  // he turns on Sam while Sam keeps station in reach
  { const C = await inReach(), on0 = stalkOn(C.g) === C.A && !C.held(); C.A.char.bodyYaw = C.A.char.aimYaw = -EAST; let grabbed = false, again = false; C.step(); const on1 = stalkOn(C.g); for (let i = 0; i < 120; i++) { C.step(); grabbed ||= C.held(); again ||= !!stalkOn(C.g); }
    verdict(on0 && on1 === null && !grabbed && !again, 'keeping station in reach behind him with the button down, the man turns to face Sam: the stalk is over on that frame, and two seconds on nobody is grabbed and nothing is stalked again',
      `waiting in reach before ${on0} · the frame after, ${on1 ? 'still stalking' : 'nothing stalked'} · grabbed ${grabbed}, stalking again ${again}`); }
  // let go in reach, and pressed again
  { const C = await inReach(), on0 = stalkOn(C.g) === C.A; C.lift(); for (let i = 0; i < 30; i++) C.step(); const on1 = stalkOn(C.g), v = speed(C), held1 = C.held(); C.press(); for (let i = 0; i < 20; i++) C.step(); const on2 = stalkOn(C.g) === C.A, held2 = C.held();
    verdict(on0 && on1 === null && v < 0.02 && !held1 && on2 && !held2, 'the button let go while he waits in reach: nothing is stalked, he stands still and nobody is grabbed. Pressed on him again from there: the stalk is his again, and still nobody is grabbed without the key',
      `waiting ${on0} · let go: ${on1 ? 'still stalking' : 'nothing stalked'}, ${f2(v)} m/s, in his arm ${held1} · pressed again: stalking ${on2}, in his arm ${held2}`); }
  // the wider way out: begun from 3.8 m, the man is put 4.6 m off (inside lowOutM) and then 5.3 m off (outside it)
  { const C = await mk(3.8); C.press(); for (let i = 0; i < 6; i++) C.step(); const on0 = stalkOn(C.g) === C.A; C.A.char.pos[0] += 4.6 - C.gap(); C.step(); C.step(); const on1 = stalkOn(C.g) === C.A; C.A.char.pos[0] += STALK.lowOutM + 0.3 - C.gap(); C.step(); C.step(); const on2 = stalkOn(C.g);
    verdict(on0 && on1 && on2 === null, `begun from 3.8 m, the man suddenly 4.6 m off is still stalked (the wider way out, ${f1(STALK.lowOutM)} m) and ${f1(STALK.lowOutM + 0.3)} m off he is not`,
      `begun ${on0} · 4.6 m off: stalking ${on1} · ${f1(STALK.lowOutM + 0.3)} m off: ${on2 ? 'still stalking' : 'nothing stalked'}`); }
  // a man walked up on STANDING is walked up on standing: the low creep is out of his crouch, and nothing forces one
  { const C = await mk(2.0), pl = C.g.player, an = pl.char.anim; pl.crouch = false; let low = 0; for (let i = 0; i < 90; i++) { C.step(); low = Math.max(low, an.low); } const head = pl.char.bones.head[1];
    pl.crouch = true; for (let i = 0; i < 90; i++) C.step(); const headC = pl.char.bones.head[1];
    verdict(low === 0 && head > 1.3 && headC < head - 0.3 && an.low > 0, 'stood up 2.0 m behind him with no button: for a second and a half nothing of the low creep is fed and he is drawn standing (head above 1.3 m); crouched there, he is lower by 30 cm or more and the creep\'s sink is fed',
      `standing: the most of the sink fed ${f2(low)}, head ${f2(head)} m · crouched: head ${f2(headC)} m, the sink fed ${f2(an.low)}`); }
}

/** LOWER, HANDS DOWN (STALK.low, the tenth pass): closing, he sinks and slows and no arm comes up; the grab is the rise. */
async function lowCard() {
  console.log('\n=== low: closing, he sinks lower and slower with his hands down, the pose holds when the button is let go, and the grab springs up out of it ===');
  const run = async (low: boolean, pause: boolean) => {
    const C = await world(true, SEEDS[0], 2.9, 0), pl = C.g.player.char, an = pl.anim; CLOSE.ready = true; CLOSE.noKnife = true; STALK.key = true; STALK.low = low;
    const hand = () => Math.max(pl.bones.handL[1], pl.bones.handR[1]) - pl.bones.chest[1];
    const far = { hips: pl.bones.hips[1], hand: hand() }; let top = -9, back = 0, last = far.hips, t = Infinity, drift = 0, reach = 0, rise = 0;
    C.press();
    for (let i = 0; i < 12 * 60; i++) {
      if (pause && C.gap() < 1.6) { C.lift(); for (let k = 0; k < 30; k++) C.step(); const h0 = pl.bones.hips[1], a0 = hand(); for (let k = 0; k < 40; k++) { C.step(); drift = Math.max(drift, Math.abs(pl.bones.hips[1] - h0), Math.abs(hand() - a0)); } break; }
      C.step(); top = Math.max(top, hand()); reach = Math.max(reach, an.ready ? an.readyAmt : 0); rise = Math.max(rise, an.rise); back = Math.max(back, pl.bones.hips[1] - last); last = pl.bones.hips[1];
      if (C.gap() < STALK.grabM && Math.hypot(pl.vel[0], pl.vel[2]) < 0.02) { t = i / 60; break; }
    }
    for (let k = 0; k < 30 && !pause; k++) C.step();   // (stood in reach: the pose he waits in)
    const near = { hips: pl.bones.hips[1], hand: hand(), head: pl.bones.head[1] };
    let spring = 0, held = false;
    if (!pause) { C.g.selectVerb('guard', 'grab'); C.W.input.pressed.add('Space'); for (let i = 0; i < 40; i++) { C.step(); held ||= C.held(); spring = Math.max(spring, pl.bones.hips[1] - near.hips); } }
    return { far, near, top, back, t, drift, reach, rise, spring, held, gap: C.gap() };
  };
  const A = await run(up(), false), P = await run(up(), true);
  verdict(isFinite(A.t) && A.far.hips - A.near.hips >= 0.025 && A.back < 0.004 && A.top < 0.02 && A.reach === 0 && A.rise === 0,
    'from 2.9 m behind a man who stands, the button held: his hips only ever go down as he closes, 2.5 cm or more by the time he is in reach, neither hand ever comes up to his chest, and nothing of the old reach or rise plays',
    `hips ${f2(A.far.hips)} m at 2.9 m, ${f2(A.near.hips)} in reach (${f1((A.far.hips - A.near.hips) * 100)} cm lower) · the most they came back up in a frame ${(A.back * 1000).toFixed(1)} mm · the higher hand, at its highest, ${f1(A.top * 100)} cm against his chest · the old reach ${f2(A.reach)}, rise ${f2(A.rise)}`);
  verdict(P.drift < 0.006 && P.near.hips < P.far.hips - 0.005 && P.near.hips > A.near.hips + 0.005, 'let go 1.6 m out, the pose is a frame of the move, paused: part of the way down, and hips and hands do not move for two thirds of a second', `hips ${f2(P.near.hips)} m (${f2(P.far.hips)} far off, ${f2(A.near.hips)} in reach) · the most either moved while he stood ${(P.drift * 1000).toFixed(1)} mm`);
}
/** the whole card (probes.ts 'stalk'): true = every check passed. The switches are put back as they were found. */
export async function stalkCheck(): Promise<boolean> {
  const was = { ...STALK }, close = JSON.parse(JSON.stringify(CLOSE)), rnd = Math.random; allPass = true; const hold = knobsCopy(['HOLD']);
  const only = process.env.STALK_ONLY?.split(',').filter(Boolean);   // (STALK_ONLY=key,low: those cards alone, for a quick look while tuning. Never a verdict: that is the whole card, and a run of some of it says PARTIAL and fails)
  const cards: Record<string, () => Promise<void>> = { key: keyCard, low: lowCard, shipped: shippedCard, corner };
  const unknown = only?.filter(n => !(n in cards)) ?? [];
  if (unknown.length) { console.log(`STALK_ONLY names no card: ${unknown.join(', ')} (the cards: ${Object.keys(cards).join(', ')})`); return false; }
  try {
    for (const [name, card] of Object.entries(cards)) if (!only || only.includes(name)) await card();
  }
  finally { Object.assign(STALK, was); Object.assign(CLOSE, close); Math.random = rnd; knobsBack(hold); }   // (the hold's table too, as it was found: world() raises its share of the polish)
  console.log(`\n=== stalk: ${only ? `PARTIAL (${only.join(', ')}): ${allPass ? 'those pass' : 'FAILURES above'} — not a verdict` : allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass && !only;
}
if (import.meta.main) process.exit((await stalkCheck()) ? 0 : 1);
