// 'runarms' — A RUN CARRIES ITS ARMS CLOSE (RUNARMS, src/game/character.ts runArms). The maintainer: "sprint looks dainty when viewed from the back, maybe
// should have arms closer in?" The run clips' elbows and hands stand wide of the shoulders and the sprint's hands hardly travel.
//   bun run tools/qa/runarms-check.ts        (also `probes.ts runarms`; run it on BOTH stands: TPS_PROFILE=shipped in front for the second)
//   arms      the pistol on his thigh and the sprint held: over a second of the run, read in the shoulder line's own frame, neither hand and neither elbow
//             comes further out from its shoulder than a hand's width and a half, none comes in past the shoulder by more than three fingers, and each
//             hand travels further fore and aft than the clip's did.
//   slowed    the same sprint at 3.5 m/s, the pace a heavy kit leaves him (part jog, part sprint): the arms are in as close. The rule is the run's, by
//             the run's share of the pose, so the jog a guard chases at is carried the same way; the longer pump alone is the sprint's.
//   off       the key down: the clip's arms, wide of the shoulders, as they were.
//   others    the same sprint with the pistol in both hands (RUNREADY up, as the page has it), and with the rifle: both elbows and both hands are where
//             they were with the key down, to the millimetre. (A thing in ONE pumping fist, a can or the pistol with RUNREADY down, comes in with the arm.)
//   harness   the key is a row of the two stands' list with its word, off on the frozen stand and on in the page (`?runarms=0` puts it off), rides a
//             recording's tables, and a recording from before the table replays with it off.
// RUNARMS_BREAK=1 = the key left down where `arms` asks for it up: `arms` must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: the numbers say where the arms are, not how the run LOOKS (tools/qa/pose-strip.ts sprint-bare draws it from behind).
import { standUp, ROOT } from './headless.ts';

const { RUNARMS, RUNREADY } = await import(`${ROOT}/src/game/character.ts`);
const PR = await import(`${ROOT}/src/host/profile.ts`);
const JR = await import(`${ROOT}/src/game/journal.ts`);
const { STAND_ENV } = await import(`${ROOT}/src/host/standEnv.ts`);

const BREAK = process.env.RUNARMS_BREAK === '1', DT = 1 / 60, FROM = [5.2, 0, 19.5];   // (the lobby's open floor, the long way east)
let allPass = true, found = false;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
const cm = (n: number) => (n * 100).toFixed(0);
/** Sam alone on the lobby's floor. `go(keys, secs)` holds keys and reads every frame's arms in the shoulder line's frame (to his left along the two shoulder
 *  joints, level; up; forward): the furthest out a hand and an elbow come, each hand's fore-aft travel, and where both hands are, frame by frame (`trace`) */
async function world(on: boolean, gun: 'pistol' | 'holstered' | 'rifle') {
  const W: any = await quiet(() => standUp({ seed: 11 } as any)), g = W.game; RUNARMS.on = on;
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true; const pl = g.player;
  if (gun === 'rifle') { if (!pl.hasRifle) return null; pl.slot = 'rifle'; pl.holstered = false; } else { pl.slot = 'pistol'; pl.holstered = gun === 'holstered'; }
  for (const [i, gd] of g.guards.entries()) { gd.char.pos = [37.5, 0, 4.5 + i * 1.5]; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.heardUpTo = 1e12; }
  W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([...FROM]); pl.crouch = false; const c = pl.char, a = c.anim, nd = a.nodes, fk = a.fk;
  const C = { W, g, pl, c,
    go: (keys: string[], secs: number) => { for (const k of keys) W.input.keys.add(k);
      const M = { hand: 0, elbow: 0, inn: 9, fLo: [9, 9], fHi: [-9, -9], n: 0, sprint: 0, v: 0, trace: [] as number[] };
      for (let i = 0; i < Math.round(secs / DT); i++) { W.cursorAt([c.pos[0] + 6, 1, c.pos[2]]); W.step(DT); M.n++; if (pl.sprinting) M.sprint++;
        fk.run(a.pose); const A = fk.pos[nd.upperArmL], B = fk.pos[nd.upperArmR]; let lx = A[0] - B[0], lz = A[2] - B[2]; const ll = Math.hypot(lx, lz) || 1; lx /= ll; lz /= ll;
        for (const [k, s, el, hd, sg] of [[0, A, nd.forearmL, nd.handL, 1], [1, B, nd.forearmR, nd.handR, -1]] as [number, number[], number, number, number][]) {
          const e = fk.pos[el], h = fk.pos[hd], out = (p: number[]) => ((p[0] - s[0]) * lx + (p[2] - s[2]) * lz) * sg, f = -(h[0] - s[0]) * lz + (h[2] - s[2]) * lx;
          M.hand = Math.max(M.hand, out(h)); M.elbow = Math.max(M.elbow, out(e)); M.inn = Math.min(M.inn, out(h), out(e)); M.fLo[k] = Math.min(M.fLo[k], f); M.fHi[k] = Math.max(M.fHi[k], f); M.trace.push(h[0], h[1], h[2], e[0], e[1], e[2]); } }
      for (const k of keys) W.input.keys.delete(k); M.v = Math.hypot(c.vel[0], c.vel[2]); return M; } };
  for (let i = 0; i < 30; i++) { W.cursorAt([c.pos[0] + 6, 1, c.pos[2]]); W.step(DT); } return C;
}
const RUN = ['KeyD', 'ShiftLeft'];
const travel = (M: any) => Math.min(M.fHi[0] - M.fLo[0], M.fHi[1] - M.fLo[1]);
const say = (M: any) => `sprinting on ${M.sprint} of ${M.n} frames at ${M.v.toFixed(1)} m/s · a hand at most ${cm(M.hand)} cm out from its shoulder, an elbow ${cm(M.elbow)}, the furthest in ${cm(M.inn)} · each hand travels at least ${cm(travel(M))} cm fore and aft`;
const apart = (a: number[], b: number[]) => { let m = 0; for (let i = 0; i < Math.min(a.length, b.length); i++) m = Math.max(m, Math.abs(a[i] - b[i])); return a.length === b.length ? m : Infinity; };

async function arms() {
  console.log('\n=== arms: a sprint with nothing drawn carries its arms close ===');
  const A = (await world(!BREAK, 'holstered'))!; A.go(RUN, 1.0); const run = A.go(RUN, 1.0);
  const O = (await world(false, 'holstered'))!; O.go(RUN, 1.0); const old = O.go(RUN, 1.0);
  verdict(run.sprint === run.n && run.v > 4 && run.hand < 0.18 && run.elbow < 0.18 && run.inn > -0.05 && travel(run) > 0.33 && travel(run) > travel(old) * 1.4,
    'over a second of the sprint neither hand and neither elbow comes further out from its shoulder than a hand and a half, none comes in past it by more than three fingers, and each hand travels further fore and aft than the clip\'s',
    `${say(run)}\n    with the key down: ${say(old)}`);
  console.log('\n=== slowed: a sprint a heavy kit slows is in as close ===');
  // (Sam walks at 1.6 m/s or sprints: the run he holds below a full sprint is the one a heavy kit gives him, 3.5 m/s, a blend of the jog's clip and the sprint's.)
  A.g.tune.playerSprint = O.g.tune.playerSprint = 3.5; RUNARMS.on = !BREAK; A.go(RUN, 1.0); const slow = A.go(RUN, 1.0); RUNARMS.on = false; O.go(RUN, 1.0); const slowOld = O.go(RUN, 1.0);   // (the key is one table: it is set for the world that runs)
  verdict(slow.sprint === slow.n && slow.v > 3 && slow.v < 4 && slow.hand < 0.18 && slow.elbow < 0.18 && slow.inn > -0.05 && slowOld.hand > 0.24 && travel(slow) > travel(slowOld),
    'sprinting at the pace a heavy kit leaves him, neither hand and neither elbow comes further out than a hand and a half, and the hands still travel further than the clips\'',
    `${say(slow)}\n    with the key down: ${say(slowOld)}`);
  console.log('\n=== off: the key down, the clip\'s arms ===');
  verdict(old.sprint === old.n && old.hand > 0.27 && travel(old) < 0.3, 'with the key down the same sprint swings its hands wide of the shoulders over a short travel, as it did', say(old));
}
async function others() {
  console.log('\n=== others: a pistol in his hands and the rifle are as they were ===');
  const rows: string[] = [], ready = RUNREADY.on; let ok = true; try {   // (the pistol's sprint as the page has it, both hands on it: with RUNREADY down the gun rides a bare fist, and a bare arm is this key's)
  for (const gun of ['pistol', 'rifle'] as const) {
    const U = await world(true, gun); RUNREADY.on = true; if (!U) { rows.push(`the ${gun}: this stand's kit has none (not asked)`); continue; }
    U.go(RUN, 1.0); const u = U.go(RUN, 1.0);   // (the key is one module table: a world is run before the next is stood up)
    const D = (await world(false, gun))!; RUNREADY.on = true; D.go(RUN, 1.0); const d = D.go(RUN, 1.0), mm = apart(u.trace, d.trace) * 1000;
    ok &&= u.sprint === u.n && mm < 1; rows.push(`the ${gun}: sprinting on ${u.sprint} of ${u.n} frames, elbows and hands at most ${mm.toFixed(2)} mm from where they are with the key down`);
  } } finally { RUNREADY.on = ready; }
  verdict(ok, 'sprinting with the pistol in both hands, and with the rifle, both elbows and both hands are where they are with the key down', rows.join('\n    '));
}
async function harness() {
  console.log('\n=== harness: off on the frozen stand, on in the page, and in a recording ===');
  const shipped = process.env.TPS_PROFILE === 'shipped', word = process.env.TPS_RUNARMS, want = word !== undefined ? word === '1' : shipped;
  RUNARMS.on = shipped ? found : !want;   // (the frozen stand's pin writes the key at every stand-up; the page's stand has it from the page's boot, read once when the harness loads)
  const W: any = await quiet(() => standUp()), stood = RUNARMS.on, foundOk = !shipped || word !== undefined || found === want;
  const snap = JSON.parse(JSON.stringify(JR.knobsSnapshot(W.game))), had = typeof snap.tables.RUNARMS?.on === 'boolean';
  RUNARMS.on = true; const snapUp = JSON.parse(JSON.stringify(JR.knobsSnapshot(W.game))); RUNARMS.on = false; JR.applyKnobs(W.game, snapUp); const kept = RUNARMS.on;
  delete snap.tables.RUNARMS; RUNARMS.on = true; JR.applyKnobs(W.game, snap); const old = RUNARMS.on;
  const tables = JR.knobsSnapshot(W.game);   // (the page's boot writes every page switch: the tables are put back as they stood once it has been read)
  const read = (q: string) => { RUNARMS.on = !RUNARMS.on; PR.pageSwitches(new URLSearchParams(q)); return RUNARMS.on; }, plainQ = read(''), offQ = read('runarms=0');
  JR.applyKnobs(W.game, tables);
  verdict((STAND_ENV as string[]).includes('TPS_RUNARMS') && PR.FROZEN_HOLDS['RUNARMS.on'] === 'TPS_RUNARMS=1' && stood === want && foundOk && had && kept === true && old === false && plainQ === true && offQ === false,
    'the key is a row of the two stands\' list with its word, stands up on the shipped stand and down on the frozen one, rides a recording\'s tables, a recording from before the table replays with it down, and the page\'s boot writes it on for a plain address and off for ?runarms=0',
    `STAND_ENV ${(STAND_ENV as string[]).includes('TPS_RUNARMS')} · FROZEN_HOLDS ${PR.FROZEN_HOLDS['RUNARMS.on']} · found ${found}, stood up ${stood}, wanted ${want} · in the tables ${had} · recorded up ${kept} · an old recording ${old} · plain ${plainQ} · ?runarms=0 ${offQ}`);
}

/** the whole card (probes.ts 'runarms'): true = every check passed. The switch is put back as it was found. */
export async function runarmsCheck(): Promise<boolean> {
  const was = { ...RUNARMS }; allPass = true; found = RUNARMS.on;
  try { await arms(); await others(); Object.assign(RUNARMS, was); await harness(); }
  finally { Object.assign(RUNARMS, was); if (process.env.TPS_PROFILE !== 'shipped' && process.env.TPS_RUNARMS === undefined) RUNARMS.on = false; }   // (the frozen stand's own value, whatever a check before this one in the same process left up)
  console.log(`\n=== runarms: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit((await runarmsCheck()) ? 0 : 1);
