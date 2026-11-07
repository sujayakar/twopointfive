// 'disarm' — THE GRAB DISARMS HIM (HOLD.disarm, src/game/disarm.ts). The maintainer: "we should also make the grab super satisfying. cool animation where sam
// disarms him and satisfying sounds".
//   bun run tools/qa/disarm-check.ts        (also `probes.ts disarm`)
//   pose      both figures are told of the move while it runs (holdPose.strip, heldPose.strip) and of nothing once it is over; no box of Sam's is the knife
//             before HOLD.strip.knife.
//   ends      the hold let go while the gun is in Sam's hand: nothing is left carried a frame later, and the gun comes to rest on the floor.
// The key is settled on: the check asks the page's side alone and writes no switch. It used to hold three later rules off as well (HOLD.stripFirst,
// CLOSE.noKnife and the stalk, all settled on): the grab is asked as the page makes it. Until 2026-11-07 it also had `beats` (the gun out of his hand at
// HOLD.strip.take, let go at HOLD.strip.drop, the move over at HOLD.strip.home, held on the frame he is with the switch off): those are the moments of the
// catch-first order, which the disarm leading (HOLD.stripFirst) took away (on the page: out of his hand at 0.17 s, let go at 0.30 s, over at 0.85 s;
// tools/qa/stripfirst-check.ts `lead` asks the page's own). A build from before that rule: out by ruling, with the `off` part and the harness part.
// DISARM_BREAK=1 = the man is stood with empty hands (his gun never raised: the pistol man with his torch), so there is nothing to take: pose must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: nothing here says how the move LOOKS (tools/qa/close-stills.ts scenes disarm, disarm-rifle, disarm-gun draw it on the CPU) or sounds.
import { standUp, ROOT } from './headless.ts';

const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { startGrab, freeHeld } = await import(`${ROOT}/src/game/player.ts`);
const { inHand } = await import(`${ROOT}/src/game/guards.ts`);
const { handProps } = await import(`${ROOT}/src/game/combat.ts`);
const { KNIFE_TAG } = await import(`${ROOT}/src/game/rigProps.ts`);

const BREAK = process.env.DISARM_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, AT = [10.2, 0, 19.5];   // (the lobby's open floor)
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
/** the rifleman with his gun in his hands stood on the lobby's floor facing east, the rest parked, Sam with empty hands 0.8 m behind him */
async function world() {
  const W: any = await quiet(() => standUp({ seed: 11 } as any)), g = W.game;
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true; g.player.slot = 'pistol'; g.player.holstered = true;
  if (!BREAK) g.tune.carbineGuards = 'reyes'; for (let i = 0; i < 5; i++) W.step(DT);   // (a calm man with a pistol rig carries his torch: the rifleman has his gun in his hands)
  const A = g.guards.find((x: any) => x.callsign === 'Reyes'), rest = g.guards.filter((x: any) => x !== A);
  for (const [i, gd] of rest.entries()) { gd.char.pos = [37.5, 0, 4.5 + i * 1.5]; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.heardUpTo = 1e12; }
  A.char.pos = [...AT]; A.char.bodyYaw = A.char.aimYaw = EAST; A.char.vel = [0, 0, 0]; A.path = []; A.pathGoal = null; A.hold = true; A.pinned = true; A.awareness = 0; A.heardUpTo = 1e12;
  W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([AT[0] - 0.8, 0, AT[2]]);
  const step = (n = 1) => { for (let i = 0; i < n; i++) { W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); W.step(DT); g.player.visibility = 0.02; } };
  step(40);
  const gun = () => g.items.items.find((it: any) => it === A.dropped) ?? null;
  return { W, g, A, step, gun, armed: inHand(A) === 'weapon' };
}
/** the frame (from the grab's) on which he is `held`, and the move's own events, over three seconds */
function run(C: Awaited<ReturnType<typeof world>>) {
  const { g, A } = C; startGrab(g, A);
  const R = { first: inHand(A) === 'weapon' && !A.dropped, heldF: -1, takeT: -1, carriedT: -1, letT: -1, restT: -1, overT: -1, poseBoth: 0, poseOne: 0, poseAfter: 0, knifeEarly: 0, rest: [0, 0, 0] as number[] };
  for (let f = 1; f <= 180; f++) {
    C.step(); const t = f * DT, H = g.player.holding, it = C.gun(), an = g.player.char.anim, hn = A.char.anim;
    if (R.heldF < 0 && H?.phase === 'held') R.heldF = f;
    if (R.takeT < 0 && inHand(A) !== 'weapon') R.takeT = t;
    if (R.carriedT < 0 && it?.carried) R.carriedT = t;
    if (R.carriedT >= 0 && R.letT < 0 && it && !it.carried) R.letT = t;
    if (R.letT >= 0 && R.restT < 0 && it?.settled) { R.restT = t; R.rest = [...it.pos]; }
    if (R.overT < 0 && R.takeT >= 0 && H && !H.strip) R.overT = t;
    const a = !!an.holdPose?.strip, b = !!hn.heldPose?.strip;
    if (H?.strip) { if (a && b) R.poseBoth++; else R.poseOne++; } else if (a || b) R.poseAfter++;
    if (H?.strip && H.strip.t < HOLD.strip.knife) { const out: any[] = []; handProps(g, out); if (out.some(x => x.owner === (g.player.char.id & 0xff) && x.name === KNIFE_TAG)) R.knifeEarly++; }
  }
  return R;
}

async function pose() {
  console.log('\n=== pose: both figures are told, and no knife before its beat ===');
  const C = await world(), R = run(C);
  verdict(R.poseBoth > 30 && R.poseOne === 0 && R.poseAfter <= 1 && R.knifeEarly === 0, 'while the move runs both poses carry it, neither does a frame after it is over, and no box of Sam\'s is the knife before HOLD.strip.knife',
    `frames both told ${R.poseBoth} · only one ${R.poseOne} · told after it ${R.poseAfter} · frames a knife was drawn early ${R.knifeEarly}`);
}
async function ends() {
  console.log('\n=== ends: let go with the gun in his hand ===');
  const C = await world(), { g, A } = C; startGrab(g, A); let had = false;
  for (let f = 0; f < 60 && !had; f++) { C.step(); had = !!C.gun()?.carried; }
  freeHeld(g, 'cancel'); C.step(2); const still = g.items.items.some((it: any) => it.carried); let rest = false; for (let f = 0; f < 150 && !rest; f++) { C.step(); rest = !!C.gun()?.settled || (!C.gun() && inHand(A) === 'weapon'); }
  verdict(had && !still && rest, 'the hold let go while the gun is in Sam\'s hand: nothing is carried two frames later and the gun comes to rest (or is back in his hand)', `it was in Sam's hand ${had} · still carried after ${still} · came to rest ${rest}`);
}
/** the whole card (probes.ts 'disarm'): true = every check passed */
export async function disarmCheck(): Promise<boolean> {
  allPass = true;
  await pose(); await ends();
  console.log(`\n=== disarm: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit((await disarmCheck()) ? 0 : 1);
