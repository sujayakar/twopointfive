// The 'gunhold' probe family (run through probes.ts: `bun run tools/qa/probes.ts gunhold | gunhold-variant | gunhold-fire | gunhold-whip | gunhold-dialogue`):
// slice 4 of docs/internal/grab-interrogate-design.md — the pistol-to-the-head variant of the hold. What was in the hand at the grab fixes it (player.ts
// grabVariant: the Five-seveN drawn → 'gun', holstered / a canister up → 'arm'); in the gun variant the pistol stays drawn up beside his head (character.ts
// holdLayer, HOLD.gun), a click fires one round past him at what the reticle has (HOLD_FIRE_CD apart, HOLD_SPREAD wide, never into him), HOLD Space is the
// pistol-whip (lands at HOLD.secs.whip: 'struck', breathing, a 0.35 thud) instead of the choke, E still lets him go. Plus the dialogue follow-up: a held man's
// clock is the hold's own and an aborted whip / choke rattles him. Each stands the full game up headless (headless.ts), stages one situation, drives it through
// the REAL input path (input.keys / pressed / clicked, the hover row), prints what happened, PASS / FAIL.
import { standUp, ROOT } from './headless.ts';
const IK = 'Space';
const { v3, quat } = await import(`${ROOT}/src/math/vec.ts`);
const playerMod = await import(`${ROOT}/src/game/player.ts`);
const { startGrab, buildInteractables, GRAB_HOLD_SECS, HOLD_FIRE_CD } = playerMod;
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { isBreathing } = await import(`${ROOT}/src/game/game.ts`);
const { handProps, fireWeapon, bodyHit } = await import(`${ROOT}/src/game/combat.ts`);
const Dlg = await import(`${ROOT}/src/game/dialogue.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
/** parked out on the lot east of the building, held (the scripted flag), facing east — deaf too unless said otherwise */
function park(gd: any, i: number, deaf = true) { gd.hold = true; place(gd, [37.5, 0, 6 + i * 1.5], Math.PI / 2); if (deaf) gd.heardUpTo = 1e12; }
function tap(game: any) {
  const msgs: { t: number; text: string }[] = [];
  const origMsg = game.msg.bind(game); game.msg = (text: string) => { msgs.push({ t: game.time, text }); return origMsg(text); };
  return { msgs };
}
/** hold `code` down for `secs` of frames (keys + the press edge on the first), then let it up; returns frames stepped */
function holdKey(W: any, code: string, secs: number, each?: () => void): number { const n = Math.max(1, Math.round(secs / DT)); W.input.keys.add(code); W.input.pressed.add(code); for (let i = 0; i < n; i++) { W.step(DT); each?.(); } W.input.keys.delete(code); return n; }
function tapKey(W: any, code: string) { W.input.pressed.add(code); }
function click(W: any) { W.input.clicked |= 1; }
const f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3);
const vecS = (p: number[]) => `[${p.map(x => f2(x)).join(', ')}]`;

/** The staging: A (the corridor man, Kowalski — steady) planted at `at` facing `yaw`, calm, free; B and C parked deaf on the lot (unless `keep`); Sam crouched dark
 *  0.8 m behind A with the pistol DRAWN or HOLSTERED as asked; one settled frame. */
async function stage(seed: number, drawn: boolean, at: Vec3 = [14, 0, 11.2], yaw = Math.PI / 2, opts: { parkOthers?: boolean; lightOn?: boolean } = {}) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W;
  game.player.visibility = 0.02; game.godMode = true;
  game.player.slot = 1; game.player.holstered = !drawn; game.player.pistol.lightOn = !!opts.lightOn;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = game.guards;
  place(A, at, yaw); A.wp = 1;
  if (opts.parkOthers !== false) { park(B, 0); park(C, 1); }
  const behind: Vec3 = [at[0] - Math.sin(yaw) * 0.8, 0, at[2] - Math.cos(yaw) * 0.8];
  game.teleportPlayer(behind); game.player.crouch = true; W.cam.yaw = 0; W.cam.rebuild?.(); W.step(DT); game.player.visibility = 0.02;
  return { W, game, A, B, C, ...tap(game) };
}
/** grab A through the real press: the cursor on him, Space held past GRAB_HOLD_SECS; returns the frame the hold began (−1 never) */
function grabViaHold(S: any, secs = GRAB_HOLD_SECS + 0.15): number {
  const { W, game, A } = S; let at = -1;
  W.cursorAt(v3.add(A.char.pos, [0, 1.35, 0]));
  holdKey(W, IK, secs, () => { if (at < 0 && game.player.holding) at = Math.round(game.time / DT); });
  return at;
}
/** step until the hold proper has begun (phase 'held'), at most 2 s; then one more frame so the Space that grabbed him is up (fHeld clears) */
function untilHeld(S: any): boolean { const { W, game } = S; for (let i = 0; i < 120; i++) { if (game.player.holding?.phase === 'held') break; W.step(DT); } W.step(DT); return game.player.holding?.phase === 'held'; }
/** the pistol's boxes in Sam's hand this frame: handProps' output within `r` of his right wrist and above shoulder height (the gun up beside the man's head) — the
 *  stick parts (slide 9.5 cm, can 7.5, rail 5.5, grip 4.75 half-lengths; the goggles by his own head are all under 3) and, separately, a lit lens by them */
function pistolBoxesNearHand(game: any, r = 0.5): { n: number; lit: number } {
  const out: any[] = []; handProps(game, out); const h = game.player.char.bones.handR; let n = 0, lit = 0;
  const near = out.filter(b => b.owner === (game.player.char.id & 0xff) && v3.dist(b.c, h) < r && b.c[1] > 1.25);
  for (const b of near) if (b.h[2] >= 0.04) n++;
  for (const b of near) if (b.h[2] < 0.01 && b.emissive[0] + b.emissive[1] + b.emissive[2] > 0 && near.some(s => s.h[2] >= 0.05 && v3.dist(s.c, b.c) < 0.12)) lit++;   // (the rail light's lens: a thin lit box on the rail body, not a goggle lens up on his forehead)
  return { n, lit };
}
/** the visual can's mouth and the drawn gun's rear, off the same numbers rigProps lays the boxes with (RIG.pistol: group off [0.039, 0.08, 0.035], slide from 0.02, can to 0.36) */
function gunEnds(c: any): { tip: Vec3; rear: Vec3 } {
  const hand = c.bones.handR, g = c.gunDir; const up = v3.scale(c.gunUnder(), -1); const side = v3.cross(up, g);
  const at = (a: number): Vec3 => v3.mad(v3.mad(v3.mad(hand, g, 0.035 + a), up, 0.08), side, 0.039) as Vec3;
  return { tip: at(0.36), rear: at(0.02) };
}
function segDist(p: number[], a: number[], b: number[]) { const ab = v3.sub(b, a); const t = Math.max(0, Math.min(1, v3.dot(v3.sub(p, a), ab) / v3.dot(ab, ab))); return v3.dist(p, v3.mad(a, ab, t)); }
/** the held man's skull centre (~10 cm up the head bone from its joint) */
function skullOf(c: any): Vec3 { return v3.mad(c.bones.head, quat.rotate(c.headRot(), [0, 1, 0]), 0.10) as Vec3; }

// ================================================================ (1) the variant follows the hand: drawn → 'gun' (pistol drawn up beside his head, its light with it, the gun row), holstered → 'arm' (as it was)
export async function gunholdVariant() {
  console.log('\n=== gunhold-variant: what is in the hand at the grab fixes the hold — drawn: the gun at his head (prop drawn, rail light live, whip / fire on the row); holstered: the arm, its choke, the pistol on the thigh ===');
  let allPass = true;
  // ---- (a) drawn, rail light on
  {
    const S = await stage(401, true, undefined, undefined, { lightOn: true }); const { W, game, A, msgs } = S; const { cam, canvas } = W;
    buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
    const rowBefore = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === A)?.line2 ?? '';
    const at = grabViaHold(S); const heldOk = untilHeld(S);
    for (let i = 0; i < 60; i++) W.step(DT);   // a second for the layer's springs
    const H = game.player.holding, pl = game.player, c = pl.char;
    const props = pistolBoxesNearHand(game);
    const row = game.hover?.line2 ?? '', label = game.hover?.line1 ?? '';
    const light0 = pl.light.enabled, lightPos0 = v3.copy(pl.light.pos);
    // pose numbers, in the game: the can vs his skull, the muzzle vs his head, Sam's head off the pair's axis
    const sk = skullOf(A.char), ends = gunEnds(c); const canToSkull = segDist(sk, ends.rear, ends.tip);
    const muzzleToHead = v3.dist(c.muzzle, A.char.bones.head), muzzleToSkull = v3.dist(c.muzzle, sk);
    const fw = c.forward(), left: Vec3 = [fw[2], 0, -fw[0]]; const headOff = v3.dot(v3.sub(c.bones.head, c.pos), left), hisHeadOff = v3.dot(v3.sub(A.char.bones.head, c.pos), left);
    const wristUp = c.bones.handR[1], wristToHisShoulder = v3.dist(c.bones.handR, A.char.bones.upperArmR);
    const hisHands = [A.char.bones.handL[1] - A.char.pos[1], A.char.bones.handR[1] - A.char.pos[1]];
    // L: the light goes dark, the hold goes on
    tapKey(W, 'KeyL'); W.step(DT); const light1 = pl.light.enabled; tapKey(W, 'KeyL'); W.step(DT); const light2 = pl.light.enabled;
    const fails: string[] = [];
    if (at < 0 || !heldOk) fails.push(`staging: grab at frame ${at}, held ${heldOk}`);
    if (!/pistol to his head/.test(rowBefore)) fails.push(`the guard's row before the grab: "${rowBefore}" (want it to say the grab puts the pistol to his head)`);
    if (H?.variant !== 'gun' || A.held?.variant !== 'gun') fails.push(`variant ${H?.variant} / ${A.held?.variant} (want gun / gun)`);
    if (c.anim.hideHeldItem) fails.push('hideHeldItem is up: the pistol went to the thigh');
    if (props.n < 3) fails.push(`only ${props.n} pistol boxes by his raised hand (want slide + grip + can ≥ 3)`);
    if (!light0) fails.push('rail light not lit in the gun hold with L on'); else if (v3.dist(lightPos0, c.bones.handR) > 0.4) fails.push(`rail light posed ${f2(v3.dist(lightPos0, c.bones.handR))} m from the gun hand`);
    if (props.lit < 1 && light0) fails.push('no lit lens box on the drawn pistol');
    if (light1 !== false || light2 !== true) fails.push(`L in the hold: ${light0} → ${light1} → ${light2} (want on → off → on)`);
    if (!/pistol-whip/.test(row) || !/LMB\s+fire past him/.test(row) || !/E\s+let him go/.test(row) || !/^Space\s+“/.test(row)) fails.push(`held row: "${row}"`);
    if (!/pistol at his head/.test(label)) fails.push(`held label: "${label}"`);
    if (!msgs.some(m => /pistol at his head/.test(m.text) && /^got /.test(m.text))) fails.push(`grab log line does not name the variant: ${msgs.map(m => m.text).join(' | ')}`);
    if (canToSkull < 0.11 || canToSkull > 0.2) fails.push(`the can's line runs ${f3(canToSkull)} m from his skull centre (want 0.11–0.20: along the temple, not in it, not off in the air)`);
    if (ends.tip[1] < 1.45 || ends.tip[1] > 1.75) fails.push(`the can's mouth at height ${f2(ends.tip[1])} (want head height 1.45–1.75)`);
    if (muzzleToHead > 0.35) fails.push(`shot muzzle ${f2(muzzleToHead)} m from his head joint (want ≤ 0.35)`);
    if (headOff < 0.02) fails.push(`Sam's head ${f3(headOff)} m to the LEFT of the pair's axis (want out to the left, ≥ 0.02)`);
    if (wristToHisShoulder < 0.09) fails.push(`Sam's gun wrist ${f3(wristToHisShoulder)} m from the man's shoulder joint (inside his deltoid)`);
    if (hisHands[0] < 1.3 || hisHands[1] < 1.3) fails.push(`his hands at ${vecS(hisHands)} up (want half raised, ≥ 1.3)`);
    if (!game.player.holding || A.state === 'dead') fails.push('the hold ended');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (a) DRAWN: variant ${H?.variant}/${A.held?.variant}; guard row before: "${rowBefore}"; held row: "${label}" / "${row}"\n    pistol boxes by the raised hand: ${props.n} (${props.lit} lit); hideHeldItem ${c.anim.hideHeldItem}; rail light ${light0} at ${f2(v3.dist(lightPos0, c.bones.handR))} m off the wrist, L → ${light1} → ${light2}\n    pose: can line → his skull centre ${f3(canToSkull)} m, can mouth ${vecS(ends.tip)} (his skull ${vecS(sk)}), shot muzzle → head joint ${f2(muzzleToHead)} / skull ${f2(muzzleToSkull)} m; Sam's head ${f3(headOff)} m left of the axis (his ${f3(hisHeadOff)}); gun wrist y ${f2(wristUp)}, ${f3(wristToHisShoulder)} m off his shoulder joint; his hands ${vecS(hisHands)} up\n    log: ${msgs.map(m => m.text).join(' | ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (b) holstered: the arm variant, unchanged — pistol on the thigh, the choke on HOLD Space, L dead
  {
    const S = await stage(402, false, undefined, undefined, { lightOn: true }); const { W, game, A, msgs } = S; const { cam, canvas } = W;
    buildInteractables(game, cam, 0, 0, canvas.clientWidth, canvas.clientHeight);
    const rowBefore = game.interactables.find((x: any) => x.kind === 'guard' && x.guard === A)?.line2 ?? '';
    grabViaHold(S); const heldOk = untilHeld(S); for (let i = 0; i < 60; i++) W.step(DT);
    const H = game.player.holding, c = game.player.char; const props = pistolBoxesNearHand(game); const row = game.hover?.line2 ?? '';
    const lightOn = game.player.light.enabled;
    click(W); W.step(DT); const shots = game.mission.stats.shots;
    holdKey(W, IK, 1.0); const phaseDuring = A.held?.phase; W.step(DT); W.step(DT); const phaseAfter = game.player.holding?.phase;
    const fails: string[] = [];
    if (!heldOk) fails.push('staging: never held');
    if (/pistol/.test(rowBefore)) fails.push(`guard row holstered: "${rowBefore}" (should not promise the pistol)`);
    if (H?.variant !== 'arm' || A.held?.variant !== 'arm') fails.push(`variant ${H?.variant} / ${A.held?.variant} (want arm / arm)`);
    if (!c.anim.hideHeldItem) fails.push('hideHeldItem down in the arm hold');
    if (props.n > 0) fails.push(`${props.n} pistol boxes up by the hand in the arm hold`);
    if (lightOn) fails.push('rail light lit in the arm hold');
    if (shots > 0) fails.push('a click fired in the arm hold');
    if (!/choke him out/.test(row) || /pistol-whip|LMB/.test(row)) fails.push(`held row: "${row}"`);
    if (phaseDuring !== 'choke' || phaseAfter !== 'held' || A.state === 'dead') fails.push(`1 s of Space: ${phaseDuring} → ${phaseAfter}, A ${A.state} (want choke → held, alive)`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (b) HOLSTERED: variant ${H?.variant}/${A.held?.variant}; guard row before: "${rowBefore}"; held row "${row.split('  ·  ').slice(1).join(' · ')}"; pistol boxes up ${props.n}, hideHeldItem ${c.anim.hideHeldItem}, light ${lightOn}, click → shots ${shots}; 1 s Space: ${phaseDuring} → ${phaseAfter}\n    log: ${msgs.map(m => m.text).join(' | ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (c) drawn but a canister up (slot 2): the arm variant too
  {
    const S = await stage(403, true); const { game, A } = S; game.player.slot = 2; game.player.holstered = false;
    grabViaHold(S); untilHeld(S);
    const fails: string[] = []; if (game.player.holding?.variant !== 'arm') fails.push(`slot 2 grab: variant ${game.player.holding?.variant} (want arm)`); if (!game.player.char.anim.hideHeldItem) { for (let i = 0; i < 30; i++) S.W.step(DT); if (!game.player.char.anim.hideHeldItem) fails.push('the can stayed in the fist'); }
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (c) a smoke can up: variant ${game.player.holding?.variant}, held item hidden ${game.player.char.anim.hideHeldItem} (A ${A.state})${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== gunhold-variant: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (2) firing past him: clicks → single rounds HOLD_FIRE_CD apart from beside his head, never into him, into the man 6 m ahead
export async function gunholdFire() {
  console.log('\n=== gunhold-fire: clicks in the gun hold — one round each, ≥ 0.7 s apart, leaving from beside his head, the held man never hit, the man 6 m ahead hit ===');
  let allPass = true;
  for (const [label, tgtOff, seed] of [['target dead ahead 6 m', [6, 0], 501], ['target 6 m ahead and 0.8 m to the left (across his face from the muzzle at his right jaw)', [6, 0.8], 502]] as [string, [number, number], number][]) {
    const S = await stage(seed, true, [14, 0, 11.2], Math.PI / 2, { parkOthers: false }); const { W, game, A, B, C, msgs } = S;
    park(C, 1);
    // the target: B planted ahead of the pair (east = +x; 'left' of a pair facing east = north = −z), his back to them, deaf and rooted so five rounds find him standing (the first may drop him)
    const T = B; T.hold = true; T.pinned = true; T.heardUpTo = 1e12; place(T, [14 + 0.27 + tgtOff[0], 0, 11.2 - tgtOff[1]], Math.PI / 2); T.state = 'patrol'; T.awareness = 0;
    grabViaHold(S); const heldOk = untilHeld(S); for (let i = 0; i < 40; i++) W.step(DT);
    const pl = game.player, c = pl.char;
    // the cursor on the target's chest; keep clicking every frame for 3.4 s
    const shots: { t: number; from: Vec3; muzzleToHead: number; muzzleToSkull: number; gunOut: number; aimGuard: string }[] = []; let lastFire = pl.lastFireT; let heldHit = false, tDown = -1; let fxSeen = game.fx.length;
    let maxFlinch = 0; const headRest = v3.sub(A.char.bones.head, A.char.pos);
    for (let i = 0; i < Math.round(3.4 / DT); i++) {
      const chest = T.char.bones.chest ?? v3.add(T.char.pos, [0, 1.2, 0]); W.cursorAt(T.state === 'dead' ? v3.add(T.char.pos, [0, 1.2, 0]) as Vec3 : chest);
      click(W);
      const headBefore = v3.copy(A.char.bones.head), skBefore = skullOf(A.char);
      W.step(DT);
      if (pl.lastFireT !== lastFire) {   // a round left this frame: its flash card sits 5 cm down the bore from the muzzle it left
        lastFire = pl.lastFireT;
        const card = game.fx.slice(fxSeen).find((e: any) => Math.abs(e.box.h[0] - 0.018) < 1e-4) ?? game.fx[game.fx.length - 1];
        const from = card ? v3.copy(card.box.c) : v3.copy(c.muzzle);
        shots.push({ t: game.time, from, muzzleToHead: v3.dist(from, headBefore), muzzleToSkull: v3.dist(from, skBefore), gunOut: pl.holding?.gunOut ?? -1, aimGuard: game.aimGuard ? game.aimGuard.callsign : game.aimTarget ? 'fixture' : 'point' });
      }
      fxSeen = game.fx.length;
      if (A.state === 'dead') heldHit = true;
      if (tDown < 0 && T.state === 'dead') tDown = game.time;
      maxFlinch = Math.max(maxFlinch, v3.dist(v3.sub(A.char.bones.head, A.char.pos), headRest));
      if (!pl.holding) break;
    }
    const gaps = shots.slice(1).map((s, i) => s.t - shots[i].t); const minGap = gaps.length ? Math.min(...gaps) : Infinity;
    for (let i = 0; i < 90; i++) W.step(DT);   // let the gun settle back on him
    const gunOutAfter = pl.holding?.gunOut ?? -1;
    const fails: string[] = [];
    if (!heldOk) fails.push('staging: never held');
    if (shots.length < 4 || shots.length > 6) fails.push(`${shots.length} rounds in 3.4 s of clicking (want 4–6 at ${HOLD_FIRE_CD} s)`);
    if (minGap < HOLD_FIRE_CD - 1e-6) fails.push(`two rounds ${f3(minGap)} s apart (want ≥ ${HOLD_FIRE_CD})`);
    if (heldHit || A.state === 'dead') fails.push(`the held man went down (${A.downKind}/${A.downBy})`);
    if (tDown < 0) fails.push(`the target 6 m ahead was never hit (${T.state})`);
    if (shots.some(s => s.muzzleToHead > 0.35)) fails.push(`a round left ${f2(Math.max(...shots.map(s => s.muzzleToHead)))} m from his head joint (want ≤ 0.35: from beside his head)`);
    if (shots.some(s => s.gunOut < 0.8)) fails.push(`a round left with the gun only ${f2(Math.min(...shots.map(s => s.gunOut)))} presented (want ≥ 0.8)`);
    if (shots.some(s => s.aimGuard === A.callsign)) fails.push('the auto-aim snapped to the man in the arm');
    if (game.mission.stats.shots !== shots.length) fails.push(`card shots ${game.mission.stats.shots} vs ${shots.length} seen`);
    if (!pl.holding) fails.push('the hold ended under fire');
    if (gunOutAfter > 0.05) fails.push(`gun still ${f2(gunOutAfter)} presented 1.5 s after the last round`);
    if (maxFlinch < 0.01) fails.push('the held man never flinched');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}: ${shots.length} rounds at ${shots.map(s => f2(s.t - (shots[0]?.t ?? 0))).join(' / ')} s (min gap ${gaps.length ? f3(minGap) : '—'}); left from ${shots.map(s => f2(s.muzzleToHead)).join(' / ')} m off his head joint (${shots.map(s => f2(s.muzzleToSkull)).join(' / ')} off the skull), gun out ${shots.map(s => f2(s.gunOut)).join('/')}, aimed at ${[...new Set(shots.map(s => s.aimGuard))].join('/')}; held man ${A.state}, target ${T.state}${tDown >= 0 ? ` (down at ${f2(tDown - (shots[0]?.t ?? 0))} s after the first)` : ''}; his biggest flinch ${f3(maxFlinch)} m; gun back to ${f2(gunOutAfter)} after\n    log: ${msgs.filter(m => !/^got /.test(m.text)).map(m => m.text).join(' | ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- the skip itself, point blank: at rest the muzzle sits INSIDE his body cylinder (beside his jaw, < 0.28 m off his axis) — bodyHit's contact rule would put
  //      Sam's own round in him at t 0; fireWeapon must not even ask (the round goes on to the wall ahead)
  {
    const S = await stage(504, true); const { W, game, A } = S;
    grabViaHold(S); untilHeld(S); for (let i = 0; i < 40; i++) W.step(DT);
    const c = game.player.char, hc = A.char; const hips = hc.bones.hips ?? hc.pos;
    const offAxis = Math.hypot(c.muzzle[0] - hips[0], c.muzzle[2] - hips[2]);
    const ahead: Vec3 = v3.mad(v3.add(c.pos, [0, 1.4, 0]), c.forward(), 8) as Vec3;
    const wouldHit = bodyHit(hc, v3.copy(c.muzzle), v3.normalize(v3.sub(ahead, c.muzzle)), 60, true);   // what the loop would get for him if it asked
    fireWeapon(game, c, ahead, true);
    for (let i = 0; i < 3; i++) W.step(DT);
    const fails: string[] = [];
    if (offAxis > 0.28) fails.push(`staging: muzzle ${f3(offAxis)} m off his axis at rest — not inside his cylinder, the contact rule is not what is tested`);
    if (wouldHit < 0) fails.push('staging: bodyHit says the geometry would not have hit him anyway');
    if (A.state === 'dead') fails.push(`he took Sam's round (${A.downKind}/${A.downBy})`);
    if (!game.player.holding) fails.push('the hold ended');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — point blank: muzzle ${f3(offAxis)} m off his axis (cylinder 0.28), bodyHit alone says t ${f2(wouldHit)} (a hit); fireWeapon past him → he is ${A.state}, hold ${!!game.player.holding}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- an empty gun: clicks dry-fire, nothing leaves, the row says so
  {
    const S = await stage(503, true); const { W, game, A, msgs } = S;
    game.player.pistol.mag = 0; game.player.pistol.chamber = 0; game.player.pistol.spare = [];
    grabViaHold(S); untilHeld(S); for (let i = 0; i < 30; i++) W.step(DT);
    const row = game.hover?.line2 ?? '';
    for (let i = 0; i < 60; i++) { click(W); W.step(DT); }
    const fails: string[] = [];
    if (game.mission.stats.shots > 0) fails.push('an empty gun fired'); if (!/empty/.test(row)) fails.push(`row does not say empty: "${row}"`); if (!msgs.some(m => /click — empty/.test(m.text))) fails.push('no dry-fire line'); if (!game.player.holding || A.state === 'dead') fails.push('hold ended');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — empty gun: shots ${game.mission.stats.shots}, row "${row.split('  ·  ')[2] ?? row}", dry-fire lines ${msgs.filter(m => /click — empty/.test(m.text)).length}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== gunhold-fire: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (3) the pistol-whip: HOLD Space → 'whip' at 0.35 s, down at 0.95 s struck / breathing, louder than the choke; Space up at 0.5 s → still held, rattled
export async function gunholdWhip() {
  console.log('\n=== gunhold-whip: Space held in the gun hold — the whip starts at 0.35 s and lands at +0.6 s (struck, breathing, a 0.35 thud, the ragdoll); let go at 0.5 s and he is still held ===');
  let allPass = true;
  // ---- (a) all the way
  {
    const S = await stage(601, true); const { W, game, A, B, msgs } = S;
    grabViaHold(S); untilHeld(S); W.step(DT);
    let whipAt = -1, deadAt = -1; const t0 = game.time; const levels: number[] = []; let evSeen = 0;
    const headTrace: string[] = []; const skull0 = skullOf(A.char); let hitDist = Infinity;
    W.input.keys.add(IK); W.input.pressed.add(IK);
    for (let i = 0; i < 60 * 3 && deadAt < 0; i++) {
      W.step(DT);
      for (const e of game.events.slice(0)) { if ((e as any).__seen) continue; (e as any).__seen = true; if (e.kind === 'prop' && e.who === game.player.char.id) levels.push(e.level ?? 0); }
      void evSeen;
      const H = game.player.holding;
      if (whipAt < 0 && H?.phase === 'whip') whipAt = game.time - t0;
      if (H?.phase === 'whip') { hitDist = Math.min(hitDist, v3.dist(game.player.char.bones.handR, skullOf(A.char))); if (Math.round(H.t * 60) % 6 === 0) headTrace.push(`t${f2(H.t)} hand→skull ${f2(v3.dist(game.player.char.bones.handR, skullOf(A.char)))}`); }
      if (deadAt < 0 && A.state === 'dead') deadAt = game.time - t0;
    }
    const heldStill = !!game.player.holding, fHeld = game.player.fHeld, hide0 = game.player.char.anim.hideHeldItem;
    for (let i = 0; i < 30; i++) W.step(DT);   // Space still down half a second: nothing must come of it (spent)
    W.input.keys.delete(IK);
    let sleptAt = -1, darkAt = -1; const torchLit0 = A.dropped?.kind;
    for (let i = 0; i < 60 * 8; i++) { W.step(DT); if (sleptAt < 0 && A.char.ragdoll?.sleeping) sleptAt = game.time - t0 - deadAt; if (darkAt < 0 && !A.flashlight.enabled) darkAt = game.time - t0 - deadAt; }
    const st = game.mission.stats; const hips = A.char.bones.hips; const skull1 = skullOf(A.char);
    const fw = game.player.char.forward(), left: Vec3 = [fw[2], 0, -fw[0]]; const fell = v3.sub(A.char.pos, game.player.char.pos); const fellFwd = v3.dot(fell, fw), fellLeft = v3.dot(fell, left);
    const fails: string[] = [];
    if (whipAt < 0 || Math.abs(whipAt - HOLD.secs.chokeHold) > 0.05) fails.push(`whip began at +${f2(whipAt)} s of Space (want ${HOLD.secs.chokeHold})`);
    if (deadAt < 0 || deadAt < 0.9 - 1e-6 || deadAt > 1.1) fails.push(`went down at +${f2(deadAt)} s (want 0.9–1.1: ${HOLD.secs.chokeHold} + ${HOLD.secs.whip})`);
    if (A.downKind !== 'struck' || A.downBy !== 'player' || !isBreathing(A)) fails.push(`A ${A.state}/${A.downKind}/${A.downBy} breathing ${isBreathing(A)} (want dead/struck/player, breathing)`);
    if (heldStill || A.held) fails.push('hold not cleared on the frame he went');
    if (!fHeld) fails.push('the Space that whipped him was not marked spent (fHeld)');
    if (hide0) fails.push('the pistol left the hand at the strike (hideHeldItem)');
    if (game.player.takedown || game.player.holding) fails.push('the still-held Space started something else');
    const maxLevel = levels.length ? Math.max(...levels) : 0; if (maxLevel < 0.34) fails.push(`loudest player 'prop' event ${f2(maxLevel)} (want the whip's 0.35 — louder than the choke's 0.15 / the fold's 0.3)`);
    if (!A.char.ragdoll) fails.push('no ragdoll'); if (sleptAt < 0) fails.push('ragdoll never slept in 8 s');
    if (!hips || hips[1] > 0.35) fails.push(`his hips at y ${hips ? f2(hips[1]) : '?'} (want on the floor)`);
    if (hitDist > 0.24) fails.push(`the gun hand never came within ${f2(hitDist)} m of his skull centre (want the frame on the bone: ≤ 0.24)`);
    if (st.knockouts !== 1 || st.kills !== 0) fails.push(`stats ko ${st.knockouts} kills ${st.kills} (want 1 / 0)`);
    if (!msgs.some(m => /pistol-whipped/.test(m.text))) fails.push(`no 'pistol-whipped' log line: ${msgs.map(m => m.text).join(' | ')}`);
    if (torchLit0 !== 'torch') fails.push(`dropped ${torchLit0 ?? 'nothing'} (want his torch)`); if (darkAt < 0 || darkAt > 1.2) fails.push(`torch dark at ${darkAt < 0 ? 'never' : '+' + f2(darkAt) + ' s'}`);
    if (game.player.char.anim.holdPose) fails.push("Sam's holdPose still set");
    // somebody finds him: out cold
    game.teleportPlayer([37.5, 0, 26.5]); game.player.crouch = false;
    B.hold = false; B.heardUpTo = game.eventSeq; place(B, [A.char.pos[0] + 5, 0, 11.2], -Math.PI / 2); B.state = 'patrol'; B.awareness = 0; B.wp = 3;
    let foundAt = -1; for (let f = 0; f < 60 * 30; f++) { W.step(DT); if (foundAt < 0 && A.found) foundAt = game.time; }
    if (foundAt < 0) fails.push('B never found him'); if (!B.witness.sawBody?.breathing) fails.push(`B.witness.sawBody ${JSON.stringify(B.witness.sawBody)} (want breathing)`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (a) whip through: began +${f2(whipAt)} s, down +${f2(deadAt)} s as ${A.downKind}/${A.downBy} breathing ${isBreathing(A)}; loudest thud ${f2(maxLevel)} (events ${levels.map(l => f2(l)).join('/')}); gun hand → skull ${f2(hitDist)} m at closest; dropped ${A.dropped?.kind ?? '—'} dark +${darkAt >= 0 ? f2(darkAt) : '—'} s; ragdoll asleep +${sleptAt >= 0 ? f2(sleptAt) : '—'} s, hips y ${hips ? f2(hips[1]) : '?'}, fell ${f2(fellFwd)} m ahead / ${f2(fellLeft)} m left of Sam (skull moved ${vecS(v3.sub(skull1, skull0))}); card ko ${st.knockouts} kills ${st.kills}; found ${foundAt >= 0 ? 'yes, breathing ' + !!B.witness.sawBody?.breathing : 'never'}\n    ${headTrace.join(' · ')}\n    log: ${msgs.filter(m => !/^got |^teleported/.test(m.text)).map(m => m.text).join(' | ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (b) let go at 0.5 s: back to held, alive, rattled (talk.abortT); then a full one still lands
  {
    const S = await stage(602, true); const { W, game, A, msgs } = S;
    grabViaHold(S); untilHeld(S); W.step(DT);
    const ag0 = Dlg.agitationOf(game, A);
    holdKey(W, IK, 0.5);
    const phaseDuring = A.held?.phase; W.step(DT); W.step(DT);
    const phaseAfter = game.player.holding?.phase; const ag1 = Dlg.agitationOf(game, A); const why = (Dlg.gather(game, A).why as string[]).join(', ');
    const fails: string[] = [];
    if (phaseDuring !== 'whip') fails.push(`0.5 s of Space: phase ${phaseDuring} (want whip)`);
    if (phaseAfter !== 'held' || A.state === 'dead') fails.push(`after letting go: phase ${phaseAfter}, A ${A.state} (want held, alive)`);
    if (!msgs.some(m => /goes back to/.test(m.text))) fails.push('no checked-it line');
    if (A.talk.abortT < 0) fails.push('talk.abortT not stamped');
    if (Math.abs(ag1 - ag0 - 0.2) > 0.03) fails.push(`agitation ${f2(ag0)} → ${f2(ag1)} (want +0.20 for the aborted whip)`);
    for (let i = 0; i < 40; i++) W.step(DT);
    const gunOutBack = game.player.holding?.gunOut ?? -1;
    holdKey(W, IK, HOLD.secs.chokeHold + HOLD.secs.whip + 0.1);
    if (A.state !== 'dead' || A.downKind !== 'struck') fails.push(`the second, full whip: A ${A.state}/${A.downKind}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (b) let go at 0.5 s: phase ${phaseDuring} → ${phaseAfter}, alive; agitation ${f2(ag0)} → ${f2(ag1)} [${why}]; gun back on him (out ${f2(gunOutBack)}); the full one then: ${A.state}/${A.downKind}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  // ---- (c) at 20 Hz the strike still lands inside the window
  {
    Math.random = seeded(603);
    const W = await standUp(); const { game } = W; const dt = 1 / 20;
    game.player.visibility = 0.02; game.godMode = true; game.player.holstered = false;
    for (let i = 0; i < 3; i++) W.step(dt);
    const [A, B, C] = game.guards; place(A, [14, 0, 11.2], Math.PI / 2); A.wp = 1; park(B, 0); park(C, 1);
    game.teleportPlayer([13.2, 0, 11.2]); W.step(dt);
    startGrab(game, A); for (let i = 0; i < 20; i++) W.step(dt);
    W.cursorAt(v3.add(A.char.pos, [0, 1.6, 0])); W.step(dt);
    const t0 = game.time; let deadAt = -1; W.input.keys.add(IK); W.input.pressed.add(IK);
    for (let i = 0; i < 60 && deadAt < 0; i++) { W.step(dt); if (A.state === 'dead') deadAt = game.time - t0; }
    W.input.keys.delete(IK); for (let i = 0; i < 40; i++) W.step(dt);
    const fails: string[] = []; if (deadAt < 0.9 - 1e-6 || deadAt > 1.15) fails.push(`down at +${f2(deadAt)} s at 20 Hz`); if (A.downKind !== 'struck') fails.push(`A ${A.state}/${A.downKind}`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (c) 20 Hz: down at +${f2(deadAt)} s as ${A.downKind}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== gunhold-whip: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (4) the dialogue follow-up: the held clock is the hold's (never re-stamped by presses, never lapsing), an aborted choke rattles him +0.2, the gun at his head +0.1
export async function gunholdDialogue() {
  console.log('\n=== gunhold-dialogue: heldSinceOf = Guard.held.since through presses and past the sandbox lapse; +0.2 after an aborted choke; +0.1 for the gun at his head ===');
  let allPass = true;
  {
    const S = await stage(701, false); const { W, game, A } = S;   // the arm variant: the choke is the thing aborted here (the whip's abort is gunhold-whip (b))
    grabViaHold(S); untilHeld(S); W.step(DT);
    const since0 = A.held?.since ?? -9; const rec: string[] = [];
    const chk = (label: string) => { const hs = Dlg.heldSinceOf(game, A); rec.push(`${label}: t ${f2(game.time)} held.since ${f2(A.held?.since ?? -1)} talk.heldSince ${f2(A.talk.heldSince)} heldSinceOf ${f2(hs)} lastT ${f2(A.talk.lastT)}`); return hs; };
    const fails: string[] = [];
    if (Math.abs(chk('held') - since0) > 1e-6) fails.push('heldSinceOf ≠ held.since at the start');
    if (Math.abs(A.talk.heldSince - since0) > 1e-6) fails.push(`talk.heldSince ${f2(A.talk.heldSince)} vs held.since ${f2(since0)}: the two clocks disagree`);
    // three questions a couple of seconds apart
    for (let q = 0; q < 3; q++) { tapKey(W, IK); W.step(DT); for (let i = 0; i < 420; i++) W.step(DT); if (Math.abs(chk(`after press ${q + 1}`) - since0) > 1e-6) fails.push(`press ${q + 1} moved the held clock`); }   // (7 s apart: well past his longest answer)
    if (A.talk.stage < 3) fails.push(`staging: only ${A.talk.stage} exchanges went through`);
    // wait past the sandbox lapse (30 s) without a press, then press again: still the hold's clock, and interrogate() must not re-stamp talk.heldSince
    for (let i = 0; i < Math.round((Dlg.HOLD_LAPSE_SECS + 6) / DT); i++) W.step(DT);
    if (Math.abs(chk('36 s of silence later') - since0) > 1e-6) fails.push('the held clock lapsed after 30 s without a press');
    tapKey(W, IK); W.step(DT); W.step(DT);
    if (Math.abs(chk('after a late press') - since0) > 1e-6 || Math.abs(A.talk.heldSince - since0) > 1e-6) fails.push(`a press after the lapse re-stamped the clock (talk.heldSince ${f2(A.talk.heldSince)})`);
    const heldSecs = game.time - since0; const heldTerm = (Dlg.gather(game, A).why as string[]).find(w => /^held /.test(w)) ?? 'held —';
    // the abort term: ease off a choke at 1.5 s
    for (let i = 0; i < 90; i++) W.step(DT);
    const ag0 = Dlg.agitationOf(game, A); const why0 = (Dlg.gather(game, A).why as string[]).join(', ');
    holdKey(W, IK, 1.5); W.step(DT); W.step(DT);
    const ag1 = Dlg.agitationOf(game, A); const why1 = (Dlg.gather(game, A).why as string[]).join(', ');
    const termSum = (why: string) => why.split(', ').reduce((acc, w) => acc + (parseFloat(w.split(' ').pop() ?? '0') || 0), 0);   // the terms before the 0‥1 clamp (a steady man held 40 s sits below zero: the clamp would eat the difference)
    const raw0 = termSum(why0), raw1 = termSum(why1);
    if (game.player.holding?.phase !== 'held' || A.state === 'dead') fails.push(`after the aborted choke: ${game.player.holding?.phase}, A ${A.state}`);
    if (A.talk.abortT < 0) fails.push('abortT not stamped by the aborted choke');
    if (Math.abs(raw1 - raw0 - 0.2) > 0.03 || ag1 < ag0) fails.push(`agitation terms ${f3(raw0)} → ${f3(raw1)} (clamped ${f3(ag0)} → ${f3(ag1)}) across the aborted choke (want +0.20)`);
    // let him go: the term outlives the hold for ABORT_RATTLE_SECS, then goes; the held clock reads -1 free
    tapKey(W, 'KeyE'); for (let i = 0; i < 60; i++) W.step(DT);
    const hsFree = Dlg.heldSinceOf(game, A); const whyFree = (Dlg.gather(game, A).why as string[]).join(', ');
    if (hsFree !== -1) fails.push(`heldSinceOf ${f2(hsFree)} once let go (want −1)`);
    if (!/aborted \+0\.20/.test(whyFree)) fails.push(`the abort term is gone the second he is free: [${whyFree}]`);
    for (let i = 0; i < Math.round((Dlg.ABORT_RATTLE_SECS + 2) / DT); i++) W.step(DT);
    const whyLater = (Dlg.gather(game, A).why as string[]).join(', ');
    if (/aborted/.test(whyLater)) fails.push(`the abort term still on him ${Dlg.ABORT_RATTLE_SECS} s after release: [${whyLater}]`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — arm hold ${f2(heldSecs)} s, ${A.talk.presses} presses: the clock stayed ${f2(since0)} throughout (${heldTerm}); aborted choke: terms ${f3(raw0)} → ${f3(raw1)} [${why1}] (clamped ${f2(ag0)} → ${f2(ag1)}); free: heldSinceOf ${hsFree}, [${whyFree}] → a minute on [${whyLater}]\n    ${rec.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  {   // the gun term: the same man grabbed drawn vs holstered, everything else equal
    const cell = async (drawn: boolean) => { const S = await stage(702, drawn); grabViaHold(S); untilHeld(S); for (let i = 0; i < 10; i++) S.W.step(DT); return { a: Dlg.agitationOf(S.game, S.A), why: (Dlg.gather(S.game, S.A).why as string[]).join(', ') }; };
    const arm = await cell(false), gun = await cell(true);
    const fails: string[] = []; if (Math.abs(gun.a - arm.a - 0.1) > 0.02) fails.push(`gun ${f3(gun.a)} vs arm ${f3(arm.a)} (want +0.10)`); if (!/gunToHead \+0\.10/.test(gun.why)) fails.push(`no gunToHead term: [${gun.why}]`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — the gun at his head: agitation arm ${f3(arm.a)} [${arm.why}] vs gun ${f3(gun.a)} [${gun.why}]${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== gunhold-dialogue: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}
