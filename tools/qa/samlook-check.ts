// 'samlook' — SAM WATCHES THE MAN WHO MATTERS, AND GIVES A MAN ROOM AS HE PASSES (SAMLOOK, src/game/samlook.ts; the poses: character.ts WATCH). The
// maintainer's words: "another thing CT did really well is having sam look at enemies! curious how that might fit with our control scheme" and "i'm also
// curious if we can make the experience of 'slithering' through guards also feel just as amazing as approaching them for a grab".
//   bun run tools/qa/samlook-check.ts        (also `probes.ts samlook`; run it on BOTH stands: TPS_PROFILE=shipped in front for the second)
//   look      a man three metres off, forty degrees to one side of where he aims: inside a second his head is on the man (the cursor has not moved and
//             neither has his aim); with nobody near his head is on his aim's line.
//   noticed   of two men in front of him the nearer is his — until the further one has him in view with his notice risen: then that one is.
//   sticky    two men at one distance do not trade him back and forth: the man he has is kept against one a little nearer, and given up for one a good
//             deal nearer.
//   nobody    a man behind him, a man past the range, and any man while a gun is in the aim: his head is his aim's.
//   stalk     the man a held click is creeping up on is the man he watches.
// Both keys are settled on and the check writes neither: the slip, between, off and harness cards read the moment against a key down, and went with that. The fog's
// half of `nobody` asked through a stand-in the Rust game never calls, and went too.
// THE HARNESS HAS NO GPU: nothing here says how the turned head, the lean or the low arm LOOK.
import { standUp, ROOT } from './headless.ts';

const { SAMLOOK, samLookMan } = await import(`${ROOT}/src/game/samlook.ts`);
const { STALK, stalkOn } = await import(`${ROOT}/src/game/stalk.ts`);
const { CLOSE } = await import(`${ROOT}/src/game/close.ts`);
const { quat } = await import(`${ROOT}/src/math/vec.ts`);

const DT = 1 / 60, SAM = [8.5, 0, 17.0], R2D = 180 / Math.PI;   // (the lobby's open floor; he aims up it, +z)
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
const f1 = (n: number) => n.toFixed(1), f2 = (n: number) => n.toFixed(2);
const wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));
/** Sam on the lobby's floor aiming up it with empty hands; men[i] = where a man stands off him [to +x, up the floor] — the rest of the shift parked */
async function world(men: number[][], o: { crouch?: boolean } = {}) {
  const W: any = await quiet(() => standUp({ seed: 11 } as any)), g = W.game;
  g.godMode = true; g.aiEnabled = false; g.playerInvisible = true; g.player.slot = 'pistol'; g.player.holstered = true;
  const M: any[] = [];
  for (const [i, gd] of g.guards.entries()) { gd.path = []; gd.pathGoal = null; gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; gd.awareness = 0; gd.char.vel = [0, 0, 0];
    if (i < men.length) { gd.char.pos = [SAM[0] + men[i][0], 0, SAM[2] + men[i][1]]; gd.char.bodyYaw = gd.char.aimYaw = 0; M.push(gd); } else gd.char.pos = [37.5, 0, 4.5 + i * 1.5]; }
  W.cam.yaw = 0; W.cam.rebuild?.(); g.teleportPlayer([...SAM]); g.player.crouch = o.crouch ?? true; const c = g.player.char; c.bodyYaw = c.aimYaw = 0;
  const C = { W, g, pl: g.player, c, M,
    step: (n = 1) => { for (let i = 0; i < n; i++) { W.cursorAt([c.pos[0], 1, c.pos[2] + 6]); W.step(DT); g.player.visibility = 0.02; } },
    /** where his head looks, ° off +z (his aim's line here) */
    gaze: () => { const d = quat.rotate(quat.mul(quat.yaw(c.bodyYaw), c.skel.globalRot[c.anim.nodes.head]), c.anim.gazeLocal); return Math.atan2(d[0], d[2]) * R2D; },
    bearing: (gd: any) => Math.atan2(gd.char.pos[0] - c.pos[0], gd.char.pos[2] - c.pos[2]) * R2D,
    clear: (gd: any) => !g.col.solidBlocked([c.pos[0], 1.2, c.pos[2]], [gd.char.pos[0], 1.4, gd.char.pos[2]]),
    who: () => { const m = samLookMan(g); return m ? M.indexOf(m) : -1; } };
  C.step(40); return C;
}

async function lookAndNoticed() {
  console.log('\n=== look: his head goes to a man off to one side of his aim ===');
  const A = await world([[1.95, 2.3]]), b = A.bearing(A.M[0]), at0 = A.gaze(), aim0 = A.c.aimYaw; A.step(60); const at = A.gaze(), aim = A.c.aimYaw * R2D;
  const N = await world([]); N.step(60); const none = N.gaze();
  verdict(A.clear(A.M[0]) && A.who() === 0 && Math.abs(wrap((at - b) / R2D)) * R2D < 12 && Math.abs(b) > 30 && Math.abs(aim) < 3 && Math.abs((A.c.aimYaw - aim0) * R2D) < 1 && Math.abs(none) < 10 && N.who() === -1,
    'a man three metres off and forty degrees to one side: inside a second and a half his head is on him, his aim has not moved, and with nobody near his head is on his aim\'s line',
    `the man is ${f1(b)}° off his aim, in the clear ${A.clear(A.M[0])} · his head ${f1(at0)}° after two thirds of a second, ${f1(at)}° a second on · his aim ${f1(aim)}° · nobody near: his head ${f1(none)}°, watching ${N.who() < 0 ? 'nobody' : 'someone'}`);

  console.log('\n=== noticed: the man who has begun to see him before a nearer one who has not ===');
  const T = await world([[-1.2, 2.2], [2.0, 4.0]]), calm = T.who(), d0 = Math.hypot(1.2, 2.2), d1 = Math.hypot(2.0, 4.0);
  const hot = T.M[1]; for (let i = 0; i < 40; i++) { hot.awareness = 0.3; hot.sightT = T.g.time; T.step(1); } const seen = T.who(), at1 = T.gaze(), b1 = T.bearing(hot);
  for (let i = 0; i < 60; i++) { hot.awareness = 0; T.step(1); } const back = T.who();
  verdict(T.clear(T.M[0]) && T.clear(T.M[1]) && calm === 0 && seen === 1 && back === 0 && Math.abs(at1 - b1) < 14,
    'of two men in front of him the nearer is the one he watches; when the further has him in view with his notice risen he watches that one, his head on him; when that man has lost him he watches the nearer again',
    `the men are ${f2(d0)} m and ${f2(d1)} m off · both calm: he watches the ${calm === 0 ? 'nearer' : calm === 1 ? 'further' : 'neither'} · the further noticing: the ${seen === 0 ? 'nearer' : seen === 1 ? 'further' : 'neither'}, his head ${f1(at1)}° (that man is at ${f1(b1)}°) · calm again: the ${back === 0 ? 'nearer' : back === 1 ? 'further' : 'neither'}`);
}
async function stickyAndNobody() {
  console.log('\n=== sticky: two men at one distance do not trade him back and forth ===');
  const S = await world([[-1.5, 3.0], [1.5, 3.0]]), first = S.who(), other = 1 - first, o = S.M[other];
  if (first >= 0) { o.char.pos[2] -= 0.4; } S.step(30); const little = S.who();
  if (first >= 0) { o.char.pos[2] -= 0.9; } S.step(30); const lot = S.who();
  verdict(first >= 0 && little === first && lot === other && SAMLOOK.swapM > 0.4 && SAMLOOK.swapM < 1.2,
    'with two men at the same distance he watches one; the other coming 0.4 m nearer does not take him, and coming a further 0.9 m nearer does',
    `he watches man ${first} · the other 0.4 m nearer: man ${little} · 1.3 m nearer: man ${lot} (the man he has counts as ${f2(SAMLOOK.swapM)} m nearer)`);

  console.log('\n=== nobody: behind him, too far, or with a gun in the aim ===');
  const B = await world([[0.6, -2.4]]); B.step(30); const behind = B.who(), bg = B.gaze();
  const F = await world([[0.5, 5.2]]); const nearW = F.who(), reach = SAMLOOK.nearM; SAMLOOK.nearM = 4; F.step(20); const far = F.who(); SAMLOOK.nearM = reach;   // (the floor is not as long as the range: the range brought in under him instead)
  const G = await world([[1.2, 2.6]]); const before = G.who();
  G.pl.holstered = false; G.pl.pistol.lightOn = true; for (let i = 0; i < 60; i++) { G.W.cursorAt([G.c.pos[0], 1, G.c.pos[2] + 6]); G.W.step(DT); } const upper = G.c.anim.upper, gun = G.who(), gg = G.gaze();
  verdict(behind === -1 && Math.abs(bg) < 10 && nearW === 0 && before === 0 && far === -1 && upper === 'aim' && gun === -1 && Math.abs(gg) < 20,
    'a man two and a half metres behind him is not watched and his head stays on his aim; a man past the range is not watched; with the pistol up and its light on (the aim) nobody is watched and his head is the gun\'s',
    `behind: ${behind < 0 ? 'nobody' : 'watched'}, his head ${f1(bg)}° · inside the range ${nearW === 0 ? 'watched' : 'nobody'}, past it ${far < 0 ? 'nobody' : 'watched'} · in the open ${before === 0 ? 'watched' : 'nobody'} · the gun up (${upper}): ${gun < 0 ? 'nobody' : 'watched'}, his head ${f1(gg)}°`);
}
async function stalkCreep() {
  console.log('\n=== stalk: the man a held click creeps up on is the man he watches ===');
  const K = await world([[1.5, 1.5], [0, 2.8]]); STALK.only = true; const near = K.who(), far = K.M[1];
  for (let i = 0; i < 40; i++) { K.W.cursorAt([far.char.pos[0], 1.2, far.char.pos[2]]); if (i === 0) K.W.input.clicked |= 1; K.W.input.buttons |= 1; K.W.step(DT); K.g.player.visibility = 0.02; }
  const on = stalkOn(K.g), w = samLookMan(K.g); K.W.input.buttons &= ~1;
  verdict(near === 0 && on === far && w === far, 'with a nearer man off to one side, a held click on the back of the further one begins the creep, and the man crept up on is the man he watches',
    `before the click he watches the ${near === 0 ? 'nearer' : 'other'} · the creep is on ${on === far ? 'the further man' : on ? 'another' : 'nobody'} · he watches ${w === far ? 'the further man' : w ? 'another' : 'nobody'}`);

}
/** the whole card (probes.ts 'samlook'): true = every check passed. The switches are put back as they were found. */
export async function samlookCheck(): Promise<boolean> {
  const was = { ...SAMLOOK }, stalk = { ...STALK }, close = JSON.parse(JSON.stringify(CLOSE)); allPass = true;
  try { await lookAndNoticed(); await stickyAndNobody(); await stalkCreep(); }
  finally { Object.assign(SAMLOOK, was); Object.assign(STALK, stalk); Object.assign(CLOSE, close); }
  console.log(`\n=== samlook: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit((await samlookCheck()) ? 0 : 1);
