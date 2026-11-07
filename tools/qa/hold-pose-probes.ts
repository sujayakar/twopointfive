// The 'hold-pose' probe family (run through probes.ts: `bun run tools/qa/probes.ts hold-pose | hold-knife | hold-struggle`): what the hold LOOKS like, as far as
// a headless run can tell — the combat knife's boxes in Sam's fist for exactly the life of a hands-empty hold (rigProps.knifeBoxes off combat.ts handProps: there
// while Player.holding is the arm variant, gone the frame it ends however it ends, never in the pistol variant, never on anybody else, Sam's owner byte on every
// one), and the held man's fight through the choke (character.ts heldLayer, HOLD.struggle: head and hands moving frame to frame early, spent by the end, the two
// roots never moving for it, the choke still landing on HOLD.secs, no NaN in any bone) plus his breathing and tugging while merely held. Each stands the full game
// up headless (headless.ts), stages the corridor man with Sam crouched behind him, grabs him (player.ts startGrab — the press itself is grab-gate's business) and
// drives the rest through the real Space on his row.
import { standUp, ROOT } from './headless.ts';
const IK = 'Space';
const { v3, quat } = await import(`${ROOT}/src/math/vec.ts`);
const { startGrab, freeHeld } = await import(`${ROOT}/src/game/player.ts`);
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { CLOSE } = await import(`${ROOT}/src/game/close.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
/** hold-knife is a check of the KNIFE: the page has none since the no-knife rule (CLOSE.noKnife, on there; `?noknife=0` gives it back), and a stand-up on the
 *  shipped stand writes the rule on — so while that probe runs, every stage writes the rule: down (the knife is back), or up under HOLDKNIFE_BREAK=1 (with
 *  CLOSE.on, which the rule needs: no knife on either stand, and (a) and (c) must FAIL). null = as the stand has it (every other probe here) */
let knifeRule: boolean | null = null;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => x.toFixed(2), mm = (x: number) => (x * 1000).toFixed(1);
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
function park(gd: any, i: number) { gd.hold = true; gd.pinned = true; gd.heardUpTo = 1e12; place(gd, [37.5, 0, 6 + i * 1.5], Math.PI / 2); }
/** the corridor man (A) planted at (14, 11.2) facing east, calm; B and C parked deaf on the lot; Sam crouched 0.8 m behind him with the pistol HOLSTERED (→ the arm
 *  hold and its knife) unless `gun` (→ drawn: the pistol-to-the-head hold, no knife); `drawn`: A carries his own pistol drawn (an edgy floor) instead of the torch */
async function stage(seed: number, gun = false, drawn = false) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W; if (knifeRule !== null) { CLOSE.noKnife = knifeRule; if (knifeRule) CLOSE.on = true; }
  game.godMode = true; game.player.visibility = 0.02;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = game.guards; park(B, 1); park(C, 2);
  const at: Vec3 = [14, 0, 11.2], yaw = Math.PI / 2;
  place(A, at, yaw); A.wp = 1; A.hold = true;
  if (drawn) { A.state = 'suspicious'; A.awareness = 0.4; A.lastKnown = [at[0] + 4, 0, at[2]]; A.reactT = 0; A.drawn = true; }
  game.teleportPlayer([at[0] - Math.sin(yaw) * 0.8, 0, at[2] - Math.cos(yaw) * 0.8]); game.player.crouch = true; game.player.slot = 'pistol'; game.player.holstered = !gun;
  W.cam.yaw = yaw + Math.PI; W.cam.rebuild(); W.step(DT); game.player.visibility = 0.02; A.hold = false;
  return { W, game, A, B, C };
}
/** the cursor 6 m dead ahead of Sam at chest height: updateHold turns the pair toward the cursor, so this keeps it facing where it faces (his row is THE row while
 *  he holds somebody, wherever the cursor is — Space still lands on it) */
function ahead(W: any, game: any) { const c = game.player.char; W.cursorAt([c.pos[0] + Math.sin(c.bodyYaw) * 6, 1.0, c.pos[2] + Math.cos(c.bodyYaw) * 6]); }
/** grab A and let the hold settle 1.5 s, facing held */
function grabSettle(W: any, game: any, A: any) { startGrab(game, A); for (let i = 0; i < 90; i++) { ahead(W, game); W.step(DT); } }
function nanBones(...cs: any[]): number { let n = 0; for (const c of cs) for (const k in c.bones) for (const x of c.bones[k]) if (!Number.isFinite(x)) n++; return n; }


// ================================================================ (2) the fight: he struggles through the choke — a lot early, spent late — pose-only, on time, finite; and is no statue while merely held
export async function holdStruggle() {
  console.log('\n=== hold-struggle: choked, the held man thrashes (head / hands moving frame to frame early, spent by the end), the roots never move for it, the choke lands on time; held, he breathes and tugs ===');
  let allPass = true;
  /** per-frame travel (m) of the man's head joint and two hands over a window, and the roots' total drift */
  const track = (W: any, game: any, A: any, frames: number, keys?: () => void) => {
    const c = A.char, pc = game.player.char; let prevH = v3.copy(c.bones.head), prevL = v3.copy(c.bones.handL), prevR = v3.copy(c.bones.handR);
    const p0 = v3.copy(pc.pos), g0 = v3.copy(c.pos); const dH: number[] = [], dL: number[] = [], dR: number[] = [], dLat: number[] = []; let nan = 0, rootP = 0, rootG = 0, station = 0, minY = Infinity, maxY = -Infinity;
    for (let i = 0; i < frames; i++) {
      ahead(W, game); keys?.(); W.step(DT);
      if (!game.player.holding) break;
      nan += nanBones(c, pc);
      dH.push(v3.dist(c.bones.head, prevH)); dL.push(v3.dist(c.bones.handL, prevL)); dR.push(v3.dist(c.bones.handR, prevR));
      { const fw = pc.forward(); dLat.push(Math.abs((c.bones.head[0] - prevH[0]) * fw[2] - (c.bones.head[2] - prevH[2]) * fw[0])); }   // the side-to-side share of the head's travel (across the pair's facing): the thrash, not the loll forward or the sag down
      prevH = v3.copy(c.bones.head); prevL = v3.copy(c.bones.handL); prevR = v3.copy(c.bones.handR);
      rootP = Math.max(rootP, v3.distXZ(pc.pos, p0)); rootG = Math.max(rootG, v3.distXZ(c.pos, g0));
      const f = pc.forward(); station = Math.max(station, Math.hypot(c.pos[0] - (pc.pos[0] + f[0] * HOLD.dist), c.pos[2] - (pc.pos[2] + f[2] * HOLD.dist)));   // (his root off its station HOLD.dist dead ahead of Sam — what a pose leaking into the roots would show as, turn or no turn)
      minY = Math.min(minY, c.bones.head[1]); maxY = Math.max(maxY, c.bones.head[1]);
    }
    const mean = (a: number[]) => a.reduce((x, y) => x + y, 0) / Math.max(1, a.length);
    return { head: mean(dH), lat: mean(dLat), handL: mean(dL), handR: mean(dR), headMax: Math.max(0, ...dH), nan, rootP, rootG: Math.max(rootG, station), n: dH.length, headPP: maxY - minY };
  };
  // ---- (a) merely held, 4 s: not a statue (breathing, the odd tug) but nothing like a fight; roots still
  {
    const S = await stage(81); const { W, game, A } = S;
    grabSettle(W, game, A);
    const held = track(W, game, A, 240);
    const fails: string[] = [];
    if (game.player.holding?.phase !== 'held') fails.push(`phase ${game.player.holding?.phase} after 5.5 s of nothing (want held)`);
    if (held.nan) fails.push(`${held.nan} non-finite bone coordinates`);
    if (held.head < 0.00003) fails.push(`head all but frozen while held: ${mm(held.head)} mm/frame mean`);
    if (held.head > 0.0025) fails.push(`head thrashing while merely held: ${mm(held.head)} mm/frame mean (that is the choke's)`);
    if (held.rootP > 0.002 || held.rootG > 0.002) fails.push(`roots drifted: Sam ${mm(held.rootP)} mm, him ${mm(held.rootG)} mm`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (a) held 4 s: mean travel/frame head ${mm(held.head)} mm (peak ${mm(held.headMax)}), hands L ${mm(held.handL)} R ${mm(held.handR)} mm; head height range ${mm(held.headPP)} mm; roots Sam ${mm(held.rootP)} him ${mm(held.rootG)} mm; NaN ${held.nan}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (b) the choke all the way: early window (0.3–1.1 s in) vs late (the last half second before he goes); on time; roots still; then the ragdoll takes him
  
  // ---- (c) eased off at 1.2 s (a second tap): the fight dies down with the eased choke (no snap), he lives, the hold goes on
  {
    const S = await stage(83); const { W, game, A } = S;
    grabSettle(W, game, A);
    let chosen = false; const down = () => { if (!chosen) { chosen = true; game.selectVerb('held', 'choke'); W.input.pressed.add(IK); } };   // tapped once
    track(W, game, A, Math.round((HOLD.secs.chokeHold + 0.4) / DT), down);
    const during = track(W, game, A, Math.round(0.8 / DT), down);
    W.input.pressed.add(IK);   // the stopping tap (read in the first frame below)
    const justAfter = track(W, game, A, Math.round(0.25 / DT));
    for (let i = 0; i < 45; i++) W.step(DT);
    const settled = track(W, game, A, Math.round(1.0 / DT));
    const fails: string[] = [];
    if (game.player.holding?.phase !== 'held' || A.state === 'dead') fails.push(`after easing off: phase ${game.player.holding?.phase}, A ${A.state} (want held, alive)`);
    if (during.head < 0.002) fails.push(`no fight during: head ${mm(during.head)} mm/frame`);
    if (settled.lat > during.lat * 0.35) fails.push(`the fight went on after easing off: ${mm(settled.lat)} mm/frame side to side a second later vs ${mm(during.lat)} during`);
    if (justAfter.headMax > 0.03) fails.push(`a snap on release: head moved ${mm(justAfter.headMax)} mm in one frame`);
    if (during.nan + justAfter.nan + settled.nan) fails.push('non-finite bones');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (c) eased off at 1.2 s: head ${mm(during.head)} mm/frame during (${mm(during.lat)} side to side) → ${mm(justAfter.head)} just after (peak frame ${mm(justAfter.headMax)}) → ${mm(settled.head)} (${mm(settled.lat)}) a second on; phase ${game.player.holding?.phase}, A ${A.state}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== hold-struggle: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}
