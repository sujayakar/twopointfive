// 'stalkwide' — ARMS OUT (STALK.wide, src/game/stalk.ts; fed by close.ts, drawn by character.ts creepLayer under CREEP.wide). The maintainer, of the creep's
// raised hands: "having the hands up just doesn't look cool"; closing on a man from behind in his crouch, Sam's arms are out to each side, low, with open hands.
//   bun run tools/qa/stalkwide-check.ts        (also `probes.ts stalkwide`)
// The key is settled on, and so are the rules round it: the check asks the page's side alone and writes no switch. It used to hold the stalk's heft off
// (STALK.heft, settled on, tools/qa/stalkheft-check.ts): the arms are read with it now. The halves against the key down (his hands where the key down has
// them before he sinks and once the man is gone, a good way off the key down's in the creep, held within a frame of the key down), the part `upright`
// (the key changes nothing upright: a comparison alone) and the harness part went with that.
//   out       hands empty, crouched, the click held on a man three metres ahead: under two metres each hand is a little outside its own shoulder and
//             never a wing's reach out, well under it, the left leading, and no higher than his own neck.
//   waits     the click let go, he stops behind the man: two seconds on his arms are still out, the left leading.
//   grab      the key takes the man out of the stance: he is `held`.
// STALKWIDE_BREAK=1 = the click is never held, so he never closes on the man: out and grab must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: nothing here says how the stance LOOKS (tools/qa/close-stills.ts scene stalk-rifle draws it on the CPU).
import { standUp, ROOT } from './headless.ts';


const BREAK = process.env.STALKWIDE_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, AT = [10.2, 0, 19.5];   // (the lobby's open floor)
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
const f2 = (x: number) => x.toFixed(2);
/** the lobby's floor, the rifleman facing east with Sam 3.2 m behind him, hands empty, the rest parked */
async function world(crouch = true) {
  const W: any = await quiet(() => standUp({ seed: 11 } as any)), g = W.game;
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true; g.player.slot = 'pistol'; g.player.holstered = true;
  g.tune.carbineGuards = 'reyes'; for (let i = 0; i < 5; i++) W.step(DT);
  const A = g.guards.find((x: any) => x.callsign === 'Reyes'), rest = g.guards.filter((x: any) => x !== A);
  for (const [i, gd] of rest.entries()) { gd.char.pos = [37.5, 0, 4.5 + i * 1.5]; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.heardUpTo = 1e12; }
  A.char.pos = [...AT]; A.char.bodyYaw = A.char.aimYaw = EAST; A.char.vel = [0, 0, 0]; A.path = []; A.pathGoal = null; A.hold = true; A.pinned = true; A.awareness = 0; A.heardUpTo = 1e12;
  W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([AT[0] - 3.2, 0, AT[2]]); g.player.crouch = crouch;
  const step = (n = 1) => { for (let i = 0; i < n; i++) { A.char.vel = [0, 0, 0]; W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); W.step(DT); g.player.visibility = 0.02; } };
  step(40);
  const c = g.player.char, gap = () => Math.hypot(c.pos[0] - A.char.pos[0], c.pos[2] - A.char.pos[2]);
  const spread = () => { const b = c.bones; return Math.hypot(b.handL[0] - b.handR[0], b.handL[1] - b.handR[1], b.handL[2] - b.handR[2]); };
  const over = () => { const b = c.bones, n = b.neck ?? b.head; return Math.max(b.handL[1], b.handR[1]) - n[1]; };   // (how far the higher hand is above his own neck)
  /** the arms' shape against his shoulders: the most a hand is outside its own shoulder, the least a hand is under it, and how far the left is ahead of the right (m) */
  const shape = () => { const b = c.bones, sL = b.upperArmL, sR = b.upperArmR; let lx = sL[0] - sR[0], lz = sL[2] - sR[2]; const L = Math.hypot(lx, lz) || 1; lx /= L; lz /= L;   // (toward his left)
    const fx = -lz, fz = lx;   // (his forward: the shoulder line turned a quarter)
    const outL = (b.handL[0] - sL[0]) * lx + (b.handL[2] - sL[2]) * lz, outR = -((b.handR[0] - sR[0]) * lx + (b.handR[2] - sR[2]) * lz);
    const lead = (b.handL[0] - b.handR[0]) * fx + (b.handL[2] - b.handR[2]) * fz;
    const fwdL = (b.handL[0] - sL[0]) * fx + (b.handL[2] - sL[2]) * fz, fwdR = (b.handR[0] - sR[0]) * fx + (b.handR[2] - sR[2]) * fz;
    return { out: Math.max(outL, outR), under: Math.min(sL[1] - b.handL[1], sR[1] - b.handR[1]), lead, v: [outL, sL[1] - b.handL[1], fwdL, outR, sR[1] - b.handR[1], fwdR] }; };
  return { W, g, A, step, gap, spread, over, shape };
}
/** the click held from 3.2 m: the widest his hands are before he sinks (over 2.7 m), the mean of it under 1.9 m, and the highest a hand comes against his neck; he is left 1.5 m behind the man with the click let go */
async function creep(crouch = true) {
  const C = await world(crouch), { W } = C; if (!BREAK) { W.input.buttons |= 1; W.input.clicked |= 1; } let far = 0, near = 0, n = 0, high = -9, out = -9, under = 9, lead = 0, at16: number[] | null = null;
  for (let i = 0; i < (BREAK ? 120 : 900) && C.gap() > 1.5; i++) { C.step(); const d = C.gap(); if (d > 2.7) far = Math.max(far, C.spread()); if (d < 1.9) { near += C.spread(); n++; high = Math.max(high, C.over()); const S = C.shape(); out = Math.max(out, S.out); under = Math.min(under, S.under); lead += S.lead; if (!at16 && d < 1.6) at16 = S.v; } }
  W.input.buttons &= ~1; C.step(30);
  return { C, far, near: n ? near / n : 0, high, out, under, lead: n ? lead / n : 0, at16: at16 ?? [0, 0, 0, 0, 0, 0] };
}
async function out() {
  console.log('\n=== out: his arms come out as he sinks ===');
  const a = await creep();
  verdict(a.high < 0 && a.out > 0 && a.out < 0.25 && a.under > 0.2 && a.lead > 0.05,
    'hands empty, crouched, the click held on a man three metres ahead: under two metres the footage\'s creep: each hand a little outside its own shoulder and never a wing\'s reach out, well under it, the left leading, no hand above his own neck',
    `his hands apart before he sinks ${f2(a.far)} m · under two metres: ${f2(a.near)} m apart, the furthest out of its shoulder ${f2(a.out)} m, the least under it ${f2(a.under)} m, the left ahead by ${f2(a.lead)} m · the higher hand against his neck ${f2(a.high)} m`);
  console.log('\n=== waits: they stay out while he waits behind the man ===');
  a.C.step(120); const wait = a.C.spread(), still = Math.hypot(a.C.g.player.char.vel[0], a.C.g.player.char.vel[2]), held = a.C.shape();
  verdict(still < 0.02 && held.lead > 0.05 && held.out < 0.25,
    'the click let go, he stops behind the man and two seconds on his arms are still out, the left leading',
    `stopped (${f2(still)} m/s), two seconds on his hands ${f2(wait)} m apart, the left ahead by ${f2(held.lead)} m, the furthest out ${f2(held.out)} m`);
}
async function grab() {
  console.log('\n=== grab: the key takes the man out of the stance ===');
  const { C } = await creep(), { W, g, A } = C, c = g.player.char; if (!BREAK) { W.input.buttons |= 1; W.input.clicked |= 1; }
  for (let n = 0; n < (BREAK ? 120 : 900); n++) { C.step(); if (n > 30 && Math.hypot(c.vel[0], c.vel[2]) < 0.02 && C.gap() < 0.95) break; }
  C.step(30); const sp = C.spread(); g.selectVerb('guard', 'grab'); W.input.pressed.add('Space'); let held = -1;
  for (let f = 1; f <= 90 && held < 0; f++) { C.step(); if (g.player.holding?.phase === 'held') held = f; }
  const who = g.player.holding?.g === A;
  verdict(held > 0 && who, 'the click brings him in behind the rifleman, arms out, and the key takes him: he is held',
    `held on frame ${held}, the rifleman ${who} · in reach his hands were ${f2(sp)} m apart`);
}

/** the whole card (probes.ts 'stalkwide'): true = every check passed */
export async function stalkwideCheck(): Promise<boolean> {
  allPass = true;
  await out(); await grab();
  console.log(`\n=== stalkwide: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit((await stalkwideCheck()) ? 0 : 1);
