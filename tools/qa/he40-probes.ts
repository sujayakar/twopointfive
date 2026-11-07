// The 'he40' family (game/he40.ts — the FN2000 launcher's 40 mm high-explosive round: the launcher's second load beside the ring airfoil), run through probes.ts:
//   bun run tools/qa/probes.ts he40 | he40-keys | he40-burst | he40-dud | he40-preview | he40-quiet
// PASS / FAIL each, all headless (the Game of headless.ts, its staging borrowed from fn2000-probes.ts). No picture and no sound here: the round in the air, the
// burst, the stand-in thump, the reticle's word and the dud on the floor are untested by eye and ear.
//   he40-keys     a fresh rifle has the RING up (never nothing); 2 again with the rifle up flips RING ⇄ 40 MM and says so both ways; the kit column's row 2
//                 carries 'RING ×5' and '40MM ×3', the one that is up lit; the right button is press-then-release: no arc at rest, the press frame holds the
//                 arc and sends nothing (and no plain checkpoint while it is held), the next frame sends one and the count drops; a press inside the tube's
//                 reload is refused and begins nothing; held, the arc stays up and nothing goes; Esc (Game.escape, and the key inside a frame) drops it
//                 unsent and the button still down begins nothing; letting go of a held one sends one; with none left it says so (and where the other load
//                 is); a checkpoint keeps the count and the load, a file from when 'nothing' could be selected (attIdx −1) loads with the RING up, and none
//                 is written while a round is in the air; your kit card has the row and it sets the count
//   he40-burst    a man 14 m down the corridor: the arc on the press frame reads armed, one right click, the round is there in under half a second and he is
//                 dead — by frag.ts's blast (one report in FRAG_LOG named '40 mm grenade', one 'bang' event flagged frag, ONE call into the destruction lane),
//                 no gunfire event, the tube's thump a 'prop' at Sam; Sam, 14 m off, is not in the report
//   he40-dud      the same man 5 m off: no blast at all (no report, no bang, no destruction call) — he folds and is alert, alive; a 'shell' item lies there and
//                 stays inert; a wall 3 m off: a dud and a clack nobody made; a wall 9 m off: the burst — and Sam, standing where he fired from, untouched
//   he40-preview  the previewed arc IS the flight (three arcs, within 6 cm), and 'armed' read off the preview agrees with what the round then does
//   he40-quiet    with the 40 mm never put on the button (the RING is the load a fresh rifle has up) nothing of it runs: no round in the air, the digest's rifle word unchanged ('… r5', no 'g'), the ring still
//                 flies its own table (launcher.ts ringSolve ≡ arcSolve by RING)
import { ROOT } from './headless.ts';
import { stage, stageRing, family, eventTap, listen, rmbTap } from './fn2000-probes.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { HE40, HE40_LOG, HE40_STATS, he40StatsReset, fireShell, shellSolve, arcArmed, arcLength, previewTooClose } = await import(`${ROOT}/src/game/he40.ts`);
const { RING, ringSolve, arcSolve } = await import(`${ROOT}/src/game/launcher.ts`);
const { FRAG, FRAG_LOG, fragStatsReset } = await import(`${ROOT}/src/game/frag.ts`);
const { LAUNCHER, LAUNCHER40 } = await import(`${ROOT}/src/game/weapons.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { aimGunOn } = await import(`${ROOT}/src/game/aimweight.ts`);   // (the player's aim has weight on the shipped stand: a tap waits for the gun to be on the cursor)

type Vec3 = [number, number, number];
const DT = 1 / 60;
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
/** count every call into the destruction lane a blast may make (the engine's entry point where the build has one, else the stun charge's hook) */
function destrTap(game: any) { const T = { n: 0 }; let depth = 0; for (const k of ['bang', 'blast']) { const f = game.destr[k]; if (typeof f === 'function') { const b = f.bind(game.destr); game.destr[k] = (...a: any[]) => { if (!depth) T.n++; depth++; try { return b(...a); } finally { depth--; } }; } } return T; }   // (calls from OUTSIDE the lane: the engine's blast() runs the lane's own bang() inside it)
function fresh() { he40StatsReset(); fragStatsReset(); }
/** the rifle is up (stage) with the RING on its button, as every fresh one: its digit once more = the 40 mm. True if that is what is up after it. */
function select40(W: any): boolean { const { input } = W; input.pressed.add('Digit2'); W.step(DT); return W.game.player.rifle.attachment === 'he40'; }


export async function he40Burst() {
  const F = family('he40-burst: a man 14 m off, the 40 mm up (2 again), one right click — the arc on the press, the round on the release — the round arms on the way and he dies by the frag\'s own blast, once');
  fresh(); const { W, A } = await stageRing(2302, 14, -Math.PI / 2); const { game, input } = W; const pl = game.player, rf = pl.rifle;
  const audio = listen(W); const tap = eventTap(game); const D = destrTap(game);
  select40(W); const chest = () => A.char.bones.chest ?? v3.add(A.char.pos, [0, 1.2, 0]);
  for (let f = 0; f < 3 || (f < 90 && !aimGunOn(game)); f++) { W.cursorAt(chest()); W.step(DT); }   // (three frames as ever where the aim has no weight; with it, until the swing onto him is over — a load let go of mid-swing waits for the gun, and this asks about the frame after the click)
  const snapped = game.aimGuard === A; const restArc = !!pl.throwPreview;
  const items0 = new Set(game.items.items); const shells0 = rf.shells;   // (the tube's count as staged: LAUNCHER40.rounds on the frozen stand, the kit card's one round on the page's)
  let pre: any = null, heldP: any = null, airP = -1;
  rmbTap(W, chest, () => { tap.pump(); pre = pl.throwPreview; heldP = pl.launchHeld; airP = game.shells.length; });   // (the press frame: the arc; the frame after: the round)
  const preArmed = pre ? arcArmed(pre) : false, preLen = pre ? arcLength(pre) : 0;
  tap.pump(); const inAir = game.shells.length;
  let resolvedAt = -1; for (let f = 0; f < 90; f++) { W.cursorAt(chest()); W.step(DT); tap.pump(); if (resolvedAt < 0 && !game.shells.length) resolvedAt = f; }
  const fails: string[] = [];
  if (rf.attachment !== 'he40') fails.push(`attachment ${rf.attachment}`);
  if (restArc) fails.push('an arc drawn before the button went down');
  if (!snapped) fails.push('the reticle did not snap to him'); if (!pre || pre.points.length < 2) fails.push('no arc on the press frame'); else if (!preArmed) fails.push(`the preview (${f2(preLen)} m) read too close to arm`);
  if (heldP !== 'he40' || airP !== 0) fails.push(`the press frame: launchHeld ${heldP}, ${airP} in the air (want 'he40' held, nothing sent yet)`);
  if (inAir !== 1 || shells0 < 1 || rf.shells !== shells0 - 1 || pl.launchHeld !== null) fails.push(`the frame after the click: ${inAir} in the air, ${rf.shells} left, launchHeld ${pl.launchHeld}`);
  if (resolvedAt < 0 || resolvedAt > 30) fails.push(`the round took ${resolvedAt} frames over 14 m (want under half a second at ${HE40.speed} m/s)`);
  if (A.state !== 'dead' || A.downKind !== 'shot' || A.downBy !== 'player') fails.push(`him: ${A.state} ${A.downKind}/${A.downBy} (want dead, shot, by the player)`);
  const rep = FRAG_LOG[0];
  if (FRAG_LOG.length !== 1 || !rep) fails.push(`${FRAG_LOG.length} blast reports (want 1)`);
  else { if (rep.who !== pl.char.id) fails.push(`the blast is ${rep.who}'s`); if (!rep.hits.some((h: any) => h.id === A.char.id && h.result === 'killed')) fails.push(`report ${JSON.stringify(rep.hits.map((h: any) => `${h.name} ${f2(h.d)} ${h.result}`))}`); if (rep.player) fails.push(`Sam is in the report: ${rep.player}`); }
  const bangs = tap.evs.filter((e: any) => e.kind === 'bang'), shots = tap.evs.filter((e: any) => e.kind === 'shot' || e.kind === 'guardShot'), props = tap.evs.filter((e: any) => e.kind === 'prop');
  if (bangs.length !== 1) fails.push(`${bangs.length} bang events (want 1)`);
  if (D.n !== 1) fails.push(`${D.n} calls into the destruction lane (want exactly 1: the blast's own)`);
  if (shots.length) fails.push(`${shots.length} gunfire event(s) off the launcher`);
  if (!props.some((e: any) => Math.abs(e.level - HE40.launchNoise) < 1e-6 && e.who === pl.char.id)) fails.push(`no launch noise at Sam (props ${JSON.stringify(props.map((e: any) => `${e.level}@${e.who}`))})`);
  if (!audio.some((a: any) => a.name === 'launcher')) fails.push('no launcher voice'); if (audio.some((a: any) => a.name === 'rifleSuppressed')) fails.push('a rifle report played');
  const end = HE40_LOG[0];
  if (!end || !end.armed || end.flown < HE40.armM || Math.abs(end.flown - 14) > 1.2) fails.push(`the round's end: ${end ? `${end.armed ? 'armed' : 'unarmed'} after ${f2(end.flown)} m, met ${end.met}` : 'none logged'}`);
  if (game.items.items.some((it: any) => !items0.has(it) && it.kind === 'shell')) fails.push('a dud lies there after a burst');
  if (pl.down) fails.push('Sam went down 14 m from it');
  if (!game.messages.some((m: any) => /40 mm grenade — 1 down/.test(m.text))) fails.push(`the log: "${game.messages.at(-1)?.text}"`);
  return F.done();
}

export async function he40Dud() {
  const F = family('he40-dud: short of its arming distance it is a slug — no blast, a clatter; past it, the burst; and the man who fired it from where he stands is outside his own round');
  { fresh(); const { W, A } = await stageRing(2303, 5, -Math.PI / 2); const { game, input } = W; const pl = game.player;
    const tap = eventTap(game); const D = destrTap(game);
    const is40 = select40(W); const chest = () => A.char.bones.chest ?? v3.add(A.char.pos, [0, 1.2, 0]);
    for (let f = 0; f < 3; f++) { W.cursorAt(chest()); W.step(DT); }
    const shortRest = previewTooClose(game); let preArmed = true, shortP = false, airP = -1;
    rmbTap(W, chest, () => { tap.pump(); preArmed = pl.throwPreview ? arcArmed(pl.throwPreview) : true; shortP = previewTooClose(game); airP = game.shells.length; });   // (the press frame's arc is the one the reticle words; the round goes on the next)
    tap.pump(); const shortSent = previewTooClose(game);
    let staggered = false; for (let f = 0; f < 60; f++) { W.step(DT); tap.pump(); staggered ||= A.staggerUntil > game.time; }
    const fails: string[] = [];
    if (!is40) fails.push(`2 again did not put the 40 mm up (${pl.rifle.attachment})`);
    if (preArmed) fails.push('the preview read armed at 5 m');
    if (shortRest || !shortP || shortSent) fails.push(`previewTooClose: ${shortRest} at rest, ${shortP} on the press frame, ${shortSent} on the frame the round left (want it true only while the 40 mm's arc is up: false, true, false)`);
    if (airP !== 0 || HE40_STATS.fired !== 1) fails.push(`${airP} in the air on the press frame, ${HE40_STATS.fired} fired in all (want 0, then exactly 1)`);
    if (FRAG_LOG.length || D.n || tap.evs.some((e: any) => e.kind === 'bang')) fails.push(`a blast happened: ${FRAG_LOG.length} report(s), ${D.n} destruction call(s)`);
    if (A.state === 'dead' || !A.char.alive) fails.push(`he is ${A.state} — a dud does no damage`);
    if (!staggered) fails.push('he did not fold'); { const props = tap.evs.filter((e: any) => e.kind === 'prop'); if (props.length !== 2) fails.push(`${props.length} noises for one round (want two: the tube, his fold): ${props.map((e: any) => e.level).join(', ')}`); } if (!(A.awareness >= 0.8 || A.state === 'alert')) fails.push(`his awareness ${f2(A.awareness)}, ${A.state} (want alert, or all but)`);
    const duds = game.items.items.filter((it: any) => it.kind === 'shell');
    if (duds.length !== 1 || v3.distXZ(duds[0].pos, A.char.pos) > 2.5) fails.push(`${duds.length} dud(s) on the floor${duds[0] ? `, ${f2(v3.distXZ(duds[0].pos, A.char.pos))} m from him` : ''}`);
    const end = HE40_LOG[0]; if (!end || end.armed || end.met !== 'man' || end.flown >= HE40.armM) fails.push(`the round's end: ${JSON.stringify(end && { armed: end.armed, met: end.met, flown: f2(end.flown) })}`);
    if (!game.messages.some((m: any) => /a dud/.test(m.text))) fails.push('the log does not say dud');
    for (let f = 0; f < 60 * 6; f++) W.step(DT);   // … and it stays a lump of aluminium
    if (FRAG_LOG.length || game.items.items.filter((it: any) => it.kind === 'shell').length !== 1) fails.push('the dud did not stay inert on the floor');
     }
  { fresh(); const W = await stage(2304, true, [16.0, 0, 11.4]); const { game } = W; const pl = game.player; const tap = eventTap(game); const D = destrTap(game);
    const from = v3.add(pl.char.pos, [0.4, 1.3, 0]); const fails: string[] = [];
    // the corridor's north wall three metres up the corridor: a dud
    fireShell(game, pl.char, from, [18.5, 1.2, 10.05]); for (let f = 0; f < 60; f++) { W.step(DT); tap.pump(); }
    const e1 = HE40_LOG[0]; const clack = tap.evs.find((e: any) => e.kind === 'prop' && e.who === 0 && Math.abs(e.level - HE40.dudNoise) < 1e-6);
    if (!e1 || e1.armed || e1.met !== 'static' || FRAG_LOG.length || D.n) fails.push(`near wall: ${JSON.stringify(e1 && { armed: e1.armed, met: e1.met, flown: f2(e1.flown) })}, ${FRAG_LOG.length} report(s)`);
    if (!clack) fails.push('near wall: no clack event with nobody behind it');
    if (!game.items.items.some((it: any) => it.kind === 'shell')) fails.push('near wall: no dud on the floor');
    // the same wall nine metres up it: the burst, and Sam where he stands is outside it
    tap.evs.length = 0; fireShell(game, pl.char, from, [24.9, 1.2, 10.05]); for (let f = 0; f < 60; f++) { W.step(DT); tap.pump(); }
    const e2 = HE40_LOG[1]; const rep = FRAG_LOG[0];
    if (!e2 || !e2.armed || e2.met !== 'static' || FRAG_LOG.length !== 1 || D.n !== 1) fails.push(`far wall: ${JSON.stringify(e2 && { armed: e2.armed, met: e2.met, flown: f2(e2.flown) })}, ${FRAG_LOG.length} report(s), ${D.n} destruction call(s)`);
    else { if (rep.player) fails.push(`far wall: Sam, ${f2(v3.dist(rep.at, pl.char.pos))} m from his own burst, is in the report (${rep.player})`); if (v3.dist(e2.at, [24.9, 1.2, 10.05]) > 0.6) fails.push(`far wall: burst ${f2(v3.dist(e2.at, [24.9, 1.2, 10.05]))} m from the mark`); if (game.col.nav.isBlocked(e2.at[0], e2.at[2]) && e2.at[2] < 10.05) fails.push('far wall: the burst point is inside the wall'); }
    if (pl.down) fails.push('Sam is down');
    F.verdict(!fails.length, `(b) a wall 3 m off: a dud, a ${HE40.dudNoise} clack with nobody behind it, the slug on the floor; the same wall 9 m off: one burst a hand's width off the plaster — and Sam, who has not moved, is not in its report (it arms at ${HE40.armM} m, the blast reaches ${FRAG.woundM})`, fails.join('; ') || `near: flown ${f2(e1.flown)} m · far: flown ${f2(e2.flown)} m, burst at (${f2(e2.at[0])}, ${f2(e2.at[1])}, ${f2(e2.at[2])})`); }
  return F.done();
}


export async function he40Quiet() {
  const F = family('he40-quiet: never put on the button, nothing of it runs — and the ring flies exactly as it did');
  fresh(); const W = await stage(2306, true, [16.0, 0, 11.4]); const { game, input } = W; const pl = game.player;
  const fails: string[] = [];
  const word = () => (digestGame(game).player as any).rifle as string;
  // (the stand deals the tube: the rifle's own three, or the page's one — and the word names the count only when it is not the rifle's three)
  const tube0 = pl.rifle.shells, g0 = tube0 === LAUNCHER40.rounds ? '' : ` g${tube0}`;
  const w0 = word(); if (!w0.endsWith(` r5${g0}`)) fails.push(`the digest's rifle word "${w0}" (want it to end " r5${g0}": no 40 mm term while the tube holds the rifle's own ${LAUNCHER40.rounds})`);
  // the RING is up on a fresh rifle (no second 2: that would flip to the 40 mm); three rings by the button — a one-frame click each, sent on the frame after; never the 40 mm
  if (pl.rifle.attachment !== 'ring') fails.push(`a fresh rifle has ${pl.rifle.attachment} up (want the RING)`);
  for (let k = 0; k < 3; k++) { for (let f = 0; f < 70; f++) { W.cursorAt([27 + k, 1.2, 11.4]); if (f === 2) input.clicked |= 4; W.step(DT); if (game.shells.length) fails.push('a 40 mm round in the air'); } }
  if (pl.rifle.rings !== LAUNCHER.rings - 3) fails.push(`rings ${pl.rifle.rings} after three`);
  if (HE40_STATS.fired || HE40_LOG.length || FRAG_LOG.length) fails.push(`he40 ran: ${JSON.stringify(HE40_STATS)}`);
  if (pl.rifle.shells !== tube0) fails.push(`${pl.rifle.shells} rounds left of ${tube0} with none fired`);
  const w1 = word(); if (!w1.endsWith(` r2${g0}`)) fails.push(`the digest's rifle word after three rings "${w1}"`);
  pl.rifle.shells = LAUNCHER40.rounds; const w3 = word(); if (!/ r2$/.test(w3)) fails.push(`with the rifle's own ${LAUNCHER40.rounds} in the tube the word is "${w3}" (want it to end " r2")`);
  pl.rifle.shells = 2; const w2 = word(); pl.rifle.shells = tube0; if (!/ r2 g2$/.test(w2)) fails.push(`with a round spent the word is "${w2}" (want " r2 g2")`);
  // the ring's solver is the shared one by the ring's own table, to the bit
  { const from = v3.add(pl.char.pos, [0.4, 1.3, 0]); for (const t of [[30, 1.3, 11.4], [22, 0.1, 11.6], [26, 2.7, 11.2]] as Vec3[]) { const a = ringSolve(game, from, t), b = arcSolve(game, from, t, RING); if (JSON.stringify(a) !== JSON.stringify(b)) fails.push('ringSolve differs from arcSolve by RING'); } }
  F.verdict(!fails.length, 'three rings by the button with the 40 mm never put on it: no round of it in the air, nothing logged, the tube as dealt; the digest\'s rifle word is the old one ("… r5", "… r2") and gains " g2" only once a round is spent; ringSolve is arcSolve by the ring\'s own table', fails.join('; ') || `"${w0}" → "${w1}"`);
  return F.done();
}
