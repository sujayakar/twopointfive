// 'close' — closer than ever (src/game/close.ts; hooked into player.ts buildInteractables / startTakedown / startGrab, game.ts's click site and seat tick, guards.ts sightGain,
// character.ts creep / ready):   bun run tools/qa/probes.ts close | close-<name> [--verbose]     (`bun run tools/qa/close-probes.ts` runs the family alone: exit 0 / 1)
// The ask the family holds line by line: "the animation changes when you enter the mode, the character changes when you're close enough to do a melee attack … melee attacks
// work from all angles, grabbing only works from behind, and then the sneaking animation is designed so it can be paused frame by frame in this mode (pausing movement sticks
// in the current position, not reverting back to an idle state)" · "sam doesn't pull out his knife when he's within melee radius, just reaches for it" · "lethal vs. nonlethal
// melee kills like splinter cell".
// All headless, on the lobby's open floor (nothing within 2 m at shin, chest or head height — checked by ray at every staging), against the office's south outer wall from the
// strip outside it, and in the break room with its lights off. The mode and its rows are settled: no line asks one off and nothing here writes one (until 2026-11-07 a line or a half
// in most probes asked CLOSE.on, mode, ready, side, front, dark, lethal or alertStrike off, or the marks on; those are out, with the probe close-harness).
// And no line asks THE KNIFE: no-knife is settled on (CLOSE.noKnife: the left click is no blow and no row names a blade), so until 2026-11-07's second cut the
// lines that asked the left click's kill (close-melee's sweep, the knife drawn in the lethal blow, an alert man, under a show; close-seat; close-duo) and the rows
// worded 'LMB  knife — lethal' (close-rows behind, a side, the front, the pistol drawn; close-melee's row words) asked a build from before that rule: out by ruling.
//   close-mode        THE MODE: on by distance at 30° steps round a standing man (off at 3.1 m coming in, on at 2.9, still on at 3.5 going out, off at 3.7); off through a wall;
//                     off for every suppressing state (a show, down, a man in the arm, a body in tow, the kick, the lock, the rifle up, sprinting, the wall);
//                     anim.creep is the mode on the same tick, creepAt his chest; the nearer of two men
//   close-tell        THE TELL: at 30° steps at 0.8 m, hands empty — anim.ready 'grab' in the rear arc, 'strike' everywhere else, never null; the hand on the hilt
//                     (anim.reachShown ≥ 0.9 within 0.5 s, under 0.1 within 0.6 s of stepping out) and the knife NEVER out (anim.knifeShown false throughout); the pistol up:
//                     'grabGun' behind, nothing elsewhere, no reach; behind a chair: 'seat'
//   close-melee       MELEE FROM EVERY ANGLE: at 30° steps through the real click — RIGHT = the knockout ('struck', breathing, knockouts +1, a 'ko' row, blows.knockout +1); the
//                     fall's hearing level by the way in, Sam never through him and no further than his station; the knife never shown; an ALERT man
//                     facing Sam; a civilian (never a kill); the pistol up (no blow)
//   close-grab        GRABBING ONLY FROM BEHIND: at 30° steps (and each side of the arc's edge) the grab row and Space — a hold starts only in the rear arc, elsewhere no row and
//                     nothing starts; from straight behind 'held' on HOLD.secs.grab; a seated man: hauled out from the rear arc at 1.15 m, never from his front
//   close-held        THE HELD SNEAK: Sam crouched on a puppet goal past a man, stopped four times in one walk — from the frame his root is at rest the phase does not move
//                     (1e-6) and feet / toes / hands stay within 2 cm for 80 frames; each stop stands in the stride it stopped in (the four differ); on resuming
//                     the phase goes on from the held value, no frame's step above the walk's own
//   close-dark        THE DARK: closeNearK's table (the light on Sam × patrol / search / alert × 0.5 / 1.0 m) and guards.ts sightGain under it; played in the break
//                     room, visibility fed from the CPU light estimate as the game's meter would — across a man's front at 1.2 m (his torch out): unnoticed in the
//                     dark, noticed with the room lit; through his burning torch: noticed (the light term is the build's); past his side with the torch burning:
//                     unnoticed; and the exhibit — three men, torches burning, Sam threading between them
//   close-windows     the table: a standing staff man's 1440 bearings each in EXACTLY one window (by the interface's own cosine AND by closeAt), 'behind' = the set
//                     cos(b) < −0.25 to the float and the only window that grabs, left / right mirror images; a seated man = the same rear arc with the seat's
//                     reach + ONE front strike window; a civilian = one rear window; the table is a registered knob table a recording carries
//   close-offer       closeAt from staged spots: 180° / 120° / ±77° / 30° / 0° at 0.7 m and 1.5 m — kind, inReach, grab, strike, can, why; sprinting; an alert man
//                     (strike only); a wall between
//   close-rows        the rows a real frame builds for an alert man in reach (the other rows' wording named the knife: out)
//   close-marks       closeMarks is off on the page (CLOSE.marks, settled off): nothing is handed to the overlay. (Until 2026-11-07 the probe then turned the row on
//                     and asked the picture: four windows, the precision claim on a 4 cm grid, a wall's cut. The page has no such picture: those lines are out.)
//   close-seat        closeSeat: down inside a second and a half, the sitting clip, kept on his mark; up the frame he is anything but calm; refusals; the right button and
//                     Space on a seated man from beyond the standing reach; a restart leaves nobody seated
//   close-duo         the two-person game (pairup.ts PAIRUP.on, one partner, silent, 'fall in' — the pair family's own stand, pairup-probes.ts `rig`): the
//                     partner is never a subject of any of it; on a defender the mode, the tell, the right button from behind / a side / the front, Space from behind
//                     only and the held gait; the partner's 'take him · on your mark' afterwards; the order menu keeps the right button;
//                     the dark rule is Sam's alone (a partner in a defender's near field is seen as ever) + what the partner does when Sam crosses the dark, as observed
// THE HARNESS HAS NO GPU AND NO DOM: the picture (the sheathed knife, the stance as drawn, the floor marks) is not measured here — what is measured is what close.ts tells the
// animator, what the animator reports back (creepShown / reachShown / knifeShown, the bones), and the game's own books.
import { standUp, ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)
import { tapVerb } from './verbs.ts';
const IK = 'Space';
const { v3, wrapAngle } = await import(`${ROOT}/src/math/vec.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { CLOSE, CLOSE_STATS, closeWindows, closeAt, closeMarks, closeSummary, closeSeat, closeSeated, CLOSE_DEFAULTS, closeMode, closeNearK, closeSubject } = await import(`${ROOT}/src/game/close.ts`);
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { TWOGEARS } = await import(`${ROOT}/src/game/player.ts`);
const { KITANIM } = await import(`${ROOT}/src/game/kitanim.ts`);
const { isBreathing } = await import(`${ROOT}/src/game/game.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
type Vec3 = [number, number, number];
const DT = 1 / 60, DEG = Math.PI / 180, VERBOSE = process.argv.includes('--verbose');
const f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3), f1 = (x: number) => x.toFixed(1), dg = (r: number) => (r / DEG).toFixed(1) + '°';
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const bad = (fails: string[]) => fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : '';
/** the lobby's open floor (the man stands here facing east); the strip south of the office by its outer wall (the wall's outside face found by ray in wallFace) */
const LOBBY: Vec3 = [9.5, 0, 19.5], EAST = Math.PI / 2, STRIP_X = 12;
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
/** parked out on the lot east of the building, on the scripted hold, deaf */
function park(gd: any, i: number) { gd.hold = true; place(gd, [37.5, 0, 6 + i * 1.5], EAST); gd.heardUpTo = 1e12; }
/** the nearest thing a ray meets from `p` at shin, chest and head height, 24 ways round (m; 9 = nothing inside 4 m) */
function clearance(g: any, p: Vec3): number { let m = 9; for (let i = 0; i < 24; i++) { const a = i / 24 * Math.PI * 2, d: Vec3 = [Math.sin(a), 0, Math.cos(a)]; for (const h of [0.3, 1.0, 1.6]) { const r = g.col.raycast([p[0], h, p[2]], d, 4); if (r) m = Math.min(m, r.t); } } return m; }
/** a spot off a man: bearing `bDeg` off HIS facing (0 = dead ahead of him, + = toward his left), `d` out, centre to centre */
function off(gd: any, bDeg: number, d: number): Vec3 { const a = gd.char.bodyYaw + bDeg * DEG; return [gd.char.pos[0] + Math.sin(a) * d, 0, gd.char.pos[2] + Math.cos(a) * d]; }
const bearingOf = (gp: ArrayLike<number>, yaw: number, p: ArrayLike<number>) => wrapAngle(Math.atan2(p[0] - gp[0], p[2] - gp[2]) - yaw);
const distXZ = (a: ArrayLike<number>, b: ArrayLike<number>) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** The one staging: A (Kowalski) planted at `at` facing `yaw` on the scripted hold (he walks nowhere), calm; B and C parked deaf on the lot; Sam's hands empty, dark,
 *  unhurtable. `ai` false = nobody perceives anything (the geometry / wording probes: Sam stands in front of the man for as long as the check takes). */
async function stage(seed: number, o: { ai?: boolean; at?: Vec3; yaw?: number; holdA?: boolean; open?: boolean } = {}) {
  Math.random = mulberry32(seed);
  const W: any = await standUp(); const g = W.game;
  g.godMode = true; g.player.visibility = 0.02; g.player.slot = 'pistol'; g.player.holstered = true; if (o.ai === false) g.aiEnabled = false;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = g.guards; const at = o.at ?? LOBBY;
  park(B, 0); park(C, 1); place(A, at, o.yaw ?? EAST); A.wp = 1; if (o.holdA !== false) A.hold = true;
  if (o.open !== false) { const c = clearance(g, at); if (c < 2) throw new Error(`close: the staging spot (${f2(at[0])}, ${f2(at[2])}) has something ${f2(c)} m off — the level moved under the probe`); }
  W.cam.yaw = 0; W.cam.rebuild?.();
  return { W, g, A, B, C, pl: g.player };
}
/** Sam put on a spot off the man and one settled frame run (the rows built, the ready pose told); → where he actually stands off him */
function samAt(S: any, bDeg: number, d: number, frames = 1, who: any = S.A) { const { W, g } = S, A = who; g.teleportPlayer(off(A, bDeg, d)); g.player.crouch = false; for (let i = 0; i < frames; i++) { W.step(DT); g.player.visibility = 0.02; } const p = g.player.char.pos; return { b: bearingOf(A.char.pos, A.char.bodyYaw, p), d: distXZ(p, A.char.pos) }; }
const rowsOf = (g: any, A: any) => g.nearby.filter((x: any) => x.kind === 'guard' && x.guard === A);
const markerOf = (g: any, A: any) => g.interactables.find((x: any) => x.kind === 'guard' && x.guard === A);
/** sit A down where he stands and run the frames it takes; → frames (−1 = never inside `max`) */
function seatDown(S: any, max = 150): number { const { W, g, A } = S; if (!closeSeat(g, A)) return -1; for (let i = 1; i <= max; i++) { W.step(DT); g.player.visibility = 0.02; if (closeSeated(g, A)) return i; } return -1; }
const { sightGain, lightEstimateAt } = await import(`${ROOT}/src/game/guards.ts`);
const { fmtEvent } = await import(`${ROOT}/src/game/chronicle.ts`);
/** what the animator agent still owes character.ts when a check that reads it fails */
const WAIT = 'WAITING ON THE ANIMATOR (character.ts)';
/** the bearings the ask is held at: every 30° round him */
const ROUND = [-150, -120, -90, -60, -30, 0, 30, 60, 90, 120, 150, 180];
/** the window a bearing is in on a standing man, by the table's own rule (rear = cos < −0.25, front inside sideFromDeg) */
const kindAt = (bDeg: number) => Math.cos(bDeg * DEG) < -0.25 ? 'behind' : Math.abs(bDeg) < CLOSE.sideFromDeg ? 'front' : 'side';
/** the world line out of the lobby spot with the longest clear run (240°: 5.7 m) — the far rings of close-mode are walked on it with the MAN turned */
const LINE = 240 * DEG;
const far = (S: any, frames = 1) => { S.g.teleportPlayer([LOBBY[0], 0, LOBBY[2] - 6]); for (let i = 0; i < frames; i++) { S.W.step(DT); S.g.player.visibility = 0.02; } };
/** Sam's meter fed as game.ts feeds it from the GPU's answer (visibilityOf of the brighter of chest and head, eased 6/s) — from the CPU's direct-light estimate */
function feedMeter(g: any) { const pl = g.player, c = pl.char; const pc = c.bones.chest ?? [c.pos[0], 1.0, c.pos[2]], ph = c.bones.head ?? [c.pos[0], 1.4, c.pos[2]]; const E = Math.max(lightEstimateAt(g, pc), lightEstimateAt(g, ph)); pl.visibility += (g.visibilityOf(E) - pl.visibility) * Math.min(1, DT * 6); }

// ================================================================ (1) the table
export async function closeWindowsProbe(): Promise<boolean> {
  console.log('\n=== close-windows: the windows round a man — every bearing in exactly one, behind = the build\'s own cosine, the sides mirrored, a seated man, a civilian, the knob table ===');
  allPass = true; const S = await stage(1101, { ai: false }), { W, g, A } = S;
  g.teleportPlayer([LOBBY[0], 0, LOBBY[2] - 6]); W.step(DT);
  const N = 1440, bs: number[] = []; for (let i = 0; i < N; i++) bs.push((-180 + (i + 0.5) * 360 / N) * DEG);   // (a quarter of a degree apart, offset half a step: no sample ON an edge)
  const inside = (w: any, b: number) => Math.cos(b - w.mid) > w.cosHalf;   // (the interface's own words: every bearing b with cos(b − mid) > cosHalf)
  const sweep = () => { const wins = closeWindows(g, A); let gap = 0, over = 0, differ = 0, behindWrong = 0, mirror = 0; const n: Record<string, number> = {};
    for (const b of bs) { const hit = wins.filter((w: any) => inside(w, b)); if (!hit.length) gap++; if (hit.length > 1) over++;
      const o = closeAt(g, A, off(A, b / DEG, 0.5)); const id = o.win?.id ?? 'none'; n[id] = (n[id] ?? 0) + 1; if (hit.length === 1 && hit[0].id !== o.win?.id) differ++;
      if ((id === 'behind') !== (Math.cos(b) < -0.25)) behindWrong++;
      const m = closeAt(g, A, off(A, -b / DEG, 0.5)).win?.id ?? 'none', want = id === 'left' ? 'right' : id === 'right' ? 'left' : id; if (m !== want) mirror++; }
    return { wins, gap, over, differ, behindWrong, mirror, n }; };
  { const r = sweep(), w = r.wins, L = w.find((x: any) => x.id === 'left'), R = w.find((x: any) => x.id === 'right'), fails: string[] = [];
    if (w.length !== 4) fails.push(`${w.length} windows (want behind, left, right, front)`);
    if (r.gap || r.over) fails.push(`${r.gap} bearings in no window, ${r.over} in more than one`); if (r.differ) fails.push(`${r.differ} bearings where closeAt names another window than the arc that holds them`);
    if (r.behindWrong) fails.push(`${r.behindWrong} bearings where 'behind' is not cos(b) < −0.25`); if (r.mirror) fails.push(`${r.mirror} bearings whose mirror image is not in the mirror window`);
    if (!L || !R || Math.abs(L.mid + R.mid) > 1e-12 || L.half !== R.half || L.reach !== R.reach || L.grab !== R.grab || L.strike !== R.strike) fails.push('left and right are not mirror images in the table');
    { const B0 = w.find((x: any) => x.id === 'behind'), F0 = w.find((x: any) => x.id === 'front'); if (!B0?.grab || !B0?.strike) fails.push('the rear window does not carry the grab and the strike'); if (L?.grab || R?.grab || !L?.strike || !R?.strike) fails.push('a side window carries the grab (or no strike)'); if (F0?.grab || !F0?.strike) fails.push('the front window carries the grab (or no strike)'); }
    if ((r.n.left ?? 0) !== (r.n.right ?? 0)) fails.push(`left holds ${r.n.left} bearings, right ${r.n.right}`);
    const wantBehind = bs.filter(b => Math.cos(b) < -0.25).length;
    verdict(!fails.length, 'standing staff man: 1440 bearings, each in exactly one window; behind is exactly cos(b) < −0.25 and ALONE carries the grab; the sides and the front carry the strike; left and right mirror each other',
      `${w.map((x: any) => `${x.id}/${x.kind} mid ${dg(x.mid)} ±${dg(x.half)} reach ${x.reach} ${x.grab ? 'grab ' : ''}${x.strike ? 'strike' : ''}`).join(' · ')}\n    bearings: behind ${r.n.behind} (cos < −0.25: ${wantBehind}) · left ${r.n.left} · right ${r.n.right} · front ${r.n.front} · none ${r.n.none ?? 0} · gaps ${r.gap} · overlaps ${r.over}${bad(fails)}`); }
  { g.aiEnabled = true; const n = seatDown(S); g.aiEnabled = false; const w = n < 0 ? [] : closeWindows(g, A), fails: string[] = []; const rearFrom = Math.acos(-0.25); let wrong = 0, none = 0;
    if (n < 0) fails.push('he never sat down'); else {
      if (w.length !== 2 || w[0].id !== 'behind' || w[0].kind !== 'seat' || w[1].id !== 'front' || w[1].kind !== 'front') fails.push(`windows ${w.map((x: any) => `${x.id}/${x.kind}`).join(', ')} (want behind/seat + front/front)`);
      if (Math.abs((Math.PI - w[0]?.half) - rearFrom) > 1e-9 || Math.abs(w[1]?.half - rearFrom) > 1e-9 || Math.abs(w[1]?.mid) > 1e-12) fails.push(`the rear begins ${dg(Math.PI - w[0]?.half)} off his facing, the front is ±${dg(w[1]?.half)} (want ${dg(rearFrom)} both: the standing man's own rear arc)`);
      if (w[0]?.reach !== CLOSE.reach.seat || w[1]?.reach !== CLOSE.reach.seatFront || !w[0]?.grab || !w[0]?.strike || w[1]?.grab || !w[1]?.strike) fails.push('reach / grab / strike of the seated windows');
      for (const b of bs) { const o = closeAt(g, A, off(A, b / DEG, 0.5)); if (!o.win) none++; else if ((o.win.kind === 'seat') !== (Math.cos(b) < -0.25)) wrong++; }
      if (wrong || none) fails.push(`${none} bearings in no window, ${wrong} on the wrong side of cos(b) = −0.25`); }
    verdict(!fails.length, 'seated (closeSeat): the same rear arc with the seat\'s longer reach (grab + strike), and ONE front window over everything forward of it (strike only) — every bearing in one of them',
      `sat in ${n} frames · ${w.map((x: any) => `${x.id}/${x.kind} mid ${dg(x.mid)} ±${dg(x.half)} reach ${x.reach} ${x.grab ? 'grab ' : ''}${x.strike ? 'strike' : ''}`).join(' · ')} · gaps ${none} · wrong side ${wrong}${bad(fails)}`); }
  { const cvs = g.civilians ?? []; if (!cvs.length) console.log('  (a civilian: none on this stand — skipped)'); else { const rows: string[] = [], fails: string[] = [];
      for (const cv of cvs) { const w = closeWindows(g, cv); rows.push(`${cv.callsign} (${cv.civ.state}${closeSeated(g, cv) ? ', seated' : ''}): ${w.map((x: any) => `${x.id}/${x.kind} ±${dg(x.half)} reach ${x.reach}`).join(' · ')}`);
        if (w.length !== 1 || w[0].id !== 'behind' || Math.abs(w[0].mid - Math.PI) > 1e-12 || !w[0].grab) fails.push(`${cv.callsign}: ${w.length} windows`);
        else if (w[0].kind === 'behind' && cv.civ.state !== 'cower' && Math.abs(Math.cos(w[0].half) - 0.25) > 1e-12) fails.push(`${cv.callsign}: a standing civilian's rear is ±${dg(w[0].half)}, not the build's`); }
      verdict(!fails.length, 'a civilian: one rear window (the build\'s own arc standing, the wider one at a desk)', rows.join('\n    ') + bad(fails)); } }
  { const K = await import(`${ROOT}/src/game/knobs.ts`), reg = K.knobRegistry(), inJ = K.JOURNAL_KNOBS.includes('CLOSE'), same = reg.CLOSE === CLOSE; let back = false;
    { const c = K.knobsCopy(), was = CLOSE.strikeDist; CLOSE.strikeDist = was + 1; (CLOSE.reach as any).behind += 1; K.knobsBack(c); back = CLOSE.strikeDist === was && CLOSE.reach.behind === (JSON.parse(JSON.stringify(CLOSE)).reach.behind) && CLOSE.reach.behind < 1.5; }
    verdict(same && inJ && back, 'knobRegistry().CLOSE is the table, JOURNAL_KNOBS carries it, and knobsCopy / knobsBack restore it (nested rows too)', `registry is the table: ${same} · in JOURNAL_KNOBS: ${inJ} · a changed strikeDist / reach.behind put back: ${back}`); }
  return allPass;
}

// ================================================================ (2) the offer
export async function closeOffer(): Promise<boolean> {
  console.log('\n=== close-offer: closeAt from staged spots — the window, the reach, what is on offer and the row\'s word for why not ===');
  allPass = true; const S = await stage(1102, { ai: false }), { W, g, A, pl } = S;
  g.teleportPlayer([LOBBY[0], 0, LOBBY[2] - 6]); W.step(DT);
  const R = CLOSE.reach;
  const table: [number, string, string, number, boolean][] = [[180, 'behind', 'behind', R.behind, true], [120, 'behind', 'behind', R.behind, true], [77, 'left', 'side', R.side, false], [-77, 'right', 'side', R.side, false], [30, 'front', 'front', R.front, false], [0, 'front', 'front', R.front, false]];
  { const rows: string[] = [], fails: string[] = [];
    for (const [b, id, kind, reach, grab] of table) for (const d of [0.7, 1.5]) { const o = closeAt(g, A, off(A, b, d)), near = d < reach;
      const got = `${o.win?.id}/${o.win?.kind} reach ${o.inReach} clear ${o.clear} grab ${o.grab} strike ${o.strike} can ${o.can} why '${o.why}'`; rows.push(`${String(b).padStart(4)}° ${d} m: ${got}`);
      const ok = o.win?.id === id && o.win?.kind === kind && o.inReach === near && o.clear === near && o.grab === (near && grab) && o.strike === near && o.can === near && o.why === (near ? '' : 'get closer') && Math.abs(o.dist - d) < 1e-9 && Math.abs(wrapAngle(o.bearing - b * DEG)) < 1e-9;
      if (!ok) fails.push(`${b}° ${d} m: ${got} (want ${id}/${kind}, ${near ? `in reach, ${grab ? 'grab + ' : ''}strike` : 'out of reach, \'get closer\''})`); }
    verdict(!fails.length, '180° / 120° / ±77° / 30° / 0° at 0.7 m and 1.5 m: behind gives the grab and the strike, the sides and the front the strike alone, 1.5 m nothing (\'get closer\')', rows.join('\n    ') + bad(fails)); }
  { pl.sprinting = true; const fails: string[] = [], seen: string[] = []; for (const [b] of table) { const o = closeAt(g, A, off(A, b, 0.7)); seen.push(o.why); if (o.can || o.inReach || o.why !== 'stop sprinting') fails.push(`${b}°: can ${o.can} why '${o.why}'`); } pl.sprinting = false;
    verdict(!fails.length, 'sprinting: nothing from any window, \'stop sprinting\'', `why: ${[...new Set(seen)].join(' | ')}${bad(fails)}`); }
  { A.state = 'alert'; const fails: string[] = [], rows: string[] = [];
    for (const [b, id] of table) { const o = closeAt(g, A, off(A, b, 0.7)); rows.push(`${b}° grab ${o.grab} strike ${o.strike}`); if (o.win?.id !== id || o.grab || !o.strike || !o.can) fails.push(`${b}°: grab ${o.grab} strike ${o.strike} can ${o.can}`); }
    A.state = 'patrol';
    verdict(!fails.length, 'an ALERT man: no grab from anywhere, the strike from every window', `${rows.join(' · ')}${bad(fails)}`); }
  { // a wall between: the south outer wall, the man half a metre off its outside face with his back to it; the spot behind him inside his reach is beyond the wall
    const hit = g.col.raycast([STRIP_X, 1.0, 2], [0, 0, 1], 6), back = g.col.raycast([STRIP_X, 1.0, 8], [0, 0, -1], 6); if (!hit || !back) throw new Error('close-offer: the south wall is not where the probe expects it');
    const near = 2 + hit.t, far = 8 - back.t; place(A, [STRIP_X, 0, near - 0.5], Math.PI);   // (facing south: the wall is at his back)
    const d = (far - near) + 0.5 + 0.12, through = closeAt(g, A, off(A, 180, d)), control = closeAt(g, A, off(A, 0, Math.min(d, R.front - 0.02))), side = closeAt(g, A, off(A, 90, 0.7)); const fails: string[] = [];
    if (!(d < R.behind)) fails.push(`staging: the far side of the wall is ${f2(d)} m off him, not inside the rear reach ${R.behind}`);
    if (through.win?.id !== 'behind' || !through.inReach || through.clear || through.can || through.why !== 'get closer') fails.push(`through the wall: reach ${through.inReach} clear ${through.clear} can ${through.can} why '${through.why}'`);
    if (!control.clear || !control.can || !side.clear || !side.can) fails.push(`his own side of the wall: front clear ${control.clear} can ${control.can} · side clear ${side.clear} can ${side.can}`);
    verdict(!fails.length, 'a wall between (he stands 0.5 m off the outer wall, Sam\'s spot inside his rear reach on its far side): in reach, NOT clear, nothing on offer; his own side is clear',
      `wall z ${f2(near)} … ${f2(far)} · the spot ${f2(d)} m dead astern of him: reach ${through.inReach} clear ${through.clear} can ${through.can} '${through.why}' · in front ${f2(control.dist)} m: clear ${control.clear} · beside 0.70 m: clear ${side.clear}${bad(fails)}`); }
  return allPass;
}

// ================================================================ (3) the rows
export async function closeRowsProbe(): Promise<boolean> {
  console.log('\n=== close-rows: what a real frame puts on the panel for an alert man ===');
  allPass = true; const S = await stage(1103, { ai: false }), { g, A } = S;
  const show = (r: any) => r ? `[${r.verb ?? '—'}] "${r.line2}" reach ${r.inReach} close ${r.close}` : 'no row';
  { A.state = 'alert'; samAt(S, 0, 0.7); const mk = markerOf(g, A), r = rowsOf(g, A).find((x: any) => x.inReach), fails: string[] = []; if (!/guard · alert/.test(mk?.line1 ?? '') || !r || r.verb) fails.push(`marker "${mk?.line1}" · ${show(r)}`); A.state = 'patrol';
    verdict(!fails.length, 'an alert man in reach: the marker says alert, the one line is the blow\'s (no Space row)', `marker "${mk?.line1}" · ${show(r)}${bad(fails)}`); }
  return allPass;
}
// ================================================================ (4) the mode
export async function closeModeProbe(): Promise<boolean> {
  console.log('\n=== close-mode: "the animation changes when you enter the mode" — on by distance from every side, off through a wall and for every state that has his body, the nearer of two men ===');
  allPass = true; const S = await stage(1111, { ai: false }), { W, g, A, B, pl } = S, an = pl.char.anim;
  const lineP = (a: number, d: number): Vec3 => [LOBBY[0] + Math.sin(a) * d, 0, LOBBY[2] + Math.cos(a) * d];
  const exact = (p: Vec3) => { g.teleportPlayer(p); return distXZ(pl.char.pos, p) < 1e-6; };
  const lineClear = (a: number, ds: number[]) => ds.every(d => { const p = lineP(a, d); return exact(p) && !g.col.solidBlocked([LOBBY[0], 1, LOBBY[2]], [p[0], 1, p[2]]); });
  if (!lineClear(LINE, [2.9, 3.1, 3.5, 3.7, 5.5])) throw new Error('close-mode: the lobby\'s 240° line is no longer clear to 5.5 m — the level moved under the probe');
  const RING: [number, boolean][] = [[3.1, false], [2.9, true], [3.5, true], [3.7, false]];
  const look = () => { const m = closeMode(g); return { d: distXZ(pl.char.pos, A.char.pos), on: m === A, other: !!m && m !== A, creep: !!an.creep, at: an.creepAt as Vec3 | null }; };
  const judge = (what: string, d: number, want: boolean, fails: string[]) => { const r = look();
    if (Math.abs(r.d - d) > 0.01) fails.push(`staging ${what} ${d} m: Sam is ${f2(r.d)} m off him`);
    if (r.on !== want || r.other) fails.push(`${what} ${d} m: mode ${r.on ? 'on' : 'off'} (want ${want ? 'on' : 'off'})`);
    if (r.creep !== r.on) fails.push(`${what} ${d} m: anim.creep ${r.creep} with the mode ${r.on ? 'on' : 'off'} on the same tick`);
    if (r.on && (!r.at || distXZ(r.at, A.char.pos) > 0.5 || r.at[1] < 0.8)) fails.push(`${what} ${d} m: creepAt ${r.at ? r.at.map(f2).join(', ') : 'null'} is not his chest`);
    if (!r.on && r.at) fails.push(`${what} ${d} m: creepAt still set out of the mode`); return r.on; };
  { const fails: string[] = [], rows: string[] = [];
    for (const b of ROUND) { place(A, LOBBY, LINE - b * DEG); const got: string[] = [];   // (the man turned: Sam on the one long clear line is at bearing b off his facing)
      g.teleportPlayer(lineP(LINE, 5.5)); pl.crouch = false; W.step(DT); judge(`${b}°`, 5.5, false, fails);
      for (const [d, want] of RING) { g.teleportPlayer(lineP(LINE, d)); W.step(DT); const bb = bearingOf(A.char.pos, A.char.bodyYaw, pl.char.pos); if (Math.abs(wrapAngle(bb - b * DEG)) > 0.5 * DEG) fails.push(`staging: Sam at ${dg(bb)} off his facing (want ${b}°)`); got.push(`${d} ${judge(`${b}°`, d, want, fails) ? 'on' : 'off'}`); }
      rows.push(`${String(b).padStart(4)}°: ${got.join(' → ')}`); }
    // …and with the man left facing east, Sam really going round him on every world line the lobby leaves clear to 3.7 m
    place(A, LOBBY, EAST); const walked: string[] = [];
    for (let i = 0; i < 12; i++) { const a = i * 30 * DEG; if (!lineClear(a, [2.9, 3.1, 3.5, 3.7])) continue; far(S); judge(`world ${i * 30}°`, 6, false, fails); const b = wrapAngle(a - EAST);
      for (const [d, want] of RING) { g.teleportPlayer(lineP(a, d)); W.step(DT); judge(`world ${i * 30}° (his ${dg(b)})`, d, want, fails); } walked.push(`${i * 30}° (his ${dg(b)})`); }
    if (walked.length < 3) fails.push(`staging: only ${walked.length} world lines out of the lobby spot are clear to 3.7 m`);
    verdict(!fails.length, `every 30° round him: off at 5.5 and 3.1 m coming in, ON at 2.9 (modeM ${CLOSE.modeM}), still on at 3.5 going out, OFF at 3.7 (modeOutM ${CLOSE.modeOutM}); anim.creep = the mode on the same tick, creepAt = his chest`,
      `${rows.join('\n    ')}\n    Sam going round a man who stands still, on the world lines clear to 3.7 m: ${walked.join(' · ')} — the same four answers on each${bad(fails)}`); }
  { // the eased weight the animator reports
    const fails: string[] = []; if (!('creepShown' in an)) fails.push(`${WAIT}: anim.creepShown is not there`); else {
      far(S, 90); const cold = an.creepShown; samAt(S, 180, 2.0); let up = -1, dn = -1; for (let f = 1; f <= 90 && up < 0; f++) { W.step(DT); if (an.creepShown > 0.9) up = f; }
      far(S); for (let f = 1; f <= 90 && dn < 0; f++) { W.step(DT); if (an.creepShown < 0.05) dn = f; }
      if (cold > 0.01) fails.push(`creepShown ${f2(cold)} with nobody near`); if (up < 0 || up > 60) fails.push(`creepShown past 0.9 after ${up} frames in the mode (want ≤ 60)`); if (dn < 0 || dn > 70) fails.push(`creepShown under 0.05 after ${dn} frames out of it (want ≤ 70)`);
      verdict(!fails.length, 'the carriage as the animator reports it (anim.creepShown): nothing with nobody near, past 0.9 inside a second in the mode, under 0.05 inside 70 frames out of it', `cold ${f3(cold)} · up at frame ${up} · down at frame ${dn}${bad(fails)}`); }
    if (fails.length && !('creepShown' in an)) verdict(false, 'anim.creepShown', bad(fails)); }
  { // through a wall: the man on the strip half a metre off the outer wall with his back to it, Sam half a metre inside the building
    const fails: string[] = []; let done = '';
    for (const x of [STRIP_X, STRIP_X + 1, STRIP_X - 1, STRIP_X + 2, STRIP_X - 2, STRIP_X + 3]) { const hit = g.col.raycast([x, 1.0, 2], [0, 0, 1], 6), back = g.col.raycast([x, 1.0, 8], [0, 0, -1], 6); if (!hit || !back) continue;
      const near = 2 + hit.t, farZ = 8 - back.t, man: Vec3 = [x, 0, near - 0.5], inside: Vec3 = [x, 0, farZ + 0.5], D = distXZ(man, inside), own: Vec3 = [x, 0, man[2] - D];
      if (farZ - near > 0.6 || !exact(inside) || !exact(own)) continue;
      far(S); place(A, man, Math.PI); g.teleportPlayer(inside); W.step(DT); const through = look(), blocked = g.col.solidBlocked([man[0], 1, man[2]], [inside[0], 1, inside[2]]);
      g.teleportPlayer(own); W.step(DT); const same = look();
      if (!blocked) fails.push('staging: nothing solid between the two spots'); if (through.on || through.creep) fails.push(`through the wall at ${f2(D)} m: mode ${through.on ? 'on' : 'off'}, creep ${through.creep}`); if (!same.on || !same.creep) fails.push(`his own side at ${f2(D)} m: mode ${same.on ? 'on' : 'off'}`);
      done = `x ${x}: wall z ${f2(near)} … ${f2(farZ)} · Sam ${f2(D)} m off him beyond it: ${through.on ? 'ON' : 'off'} · ${f2(D)} m off him on his own side: ${same.on ? 'on' : 'OFF'}`; break; }
    if (!done) fails.push('staging: no spot along the south wall where Sam stands half a metre inside it');
    verdict(!fails.length, 'a wall between (inside modeM, nothing clear between): not in the mode; the same distance on his own side: in it', done + bad(fails)); place(A, LOBBY, EAST); }
  { // the nearer of two men
    const fails: string[] = []; far(S); place(A, LOBBY, EAST); place(B, lineP(LINE, 3.4), EAST);
    g.teleportPlayer(lineP(LINE, 2.0)); W.step(DT); const m1 = closeMode(g), s1 = closeSummary(g); g.teleportPlayer(lineP(LINE, 1.2)); W.step(DT); const m2 = closeMode(g), s2 = closeSummary(g);
    if (m1 !== B) fails.push(`2.0 m from ${A.callsign}, 1.4 m from ${B.callsign}: the mode is on ${m1?.callsign ?? 'nobody'}`); if (m2 !== A) fails.push(`1.2 m from ${A.callsign}, 2.2 m from ${B.callsign}: the mode is on ${m2?.callsign ?? 'nobody'}`);
    const at = an.creepAt as Vec3 | null; if (!at || distXZ(at, A.char.pos) > 0.5) fails.push('creepAt is not on the nearer man');
    verdict(!fails.length, 'two men inside modeM: the mode is on the nearer one, and changes hands as Sam moves between them', `${s1}\n    ${s2}${bad(fails)}`); park(B, 0); }
  { // played, not asked: the rifle brought up for real, and a man taken in the arm
    const fails: string[] = []; far(S); samAt(S, 180, 2.0, 3); const on0 = look().on; pl.slot = 'rifle'; pl.holstered = false; for (let i = 0; i < 3; i++) W.step(DT); const rifle = { up: pl.rifleUp, on: look().on, creep: an.creep };
    pl.slot = 'pistol'; pl.holstered = true; for (let i = 0; i < 3; i++) W.step(DT); const on1 = look().on;
    samAt(S, 180, 0.8); const tapped = tapVerb(W, 'guard', 'grab'); W.step(DT); const held = { holding: !!pl.holding, mode: closeMode(g), creep: an.creep, ready: an.ready };
    if (!on0 || !on1) fails.push(`staging: mode ${on0} before the rifle, ${on1} after it`); if (!rifle.up || rifle.on || rifle.creep) fails.push(`rifle up ${rifle.up}: mode ${rifle.on ? 'on' : 'off'}, creep ${rifle.creep}`);
    if (!tapped || !held.holding) fails.push('staging: the grab took nobody'); else if (held.mode || held.creep || held.ready) fails.push(`with him in the arm: mode ${held.mode?.callsign ?? 'off'}, creep ${held.creep}, ready ${held.ready}`);
    verdict(!fails.length, 'through real frames: the rifle brought up → out of the mode, slung again → back in; a man taken in the arm → out of the mode, no tell', `rifle up: mode ${rifle.on ? 'on' : 'off'} · slung: ${on1 ? 'on' : 'off'} · holding: mode ${held.mode ? 'on' : 'off'} creep ${held.creep} ready ${held.ready}${bad(fails)}`); }
  return allPass;
}

// ================================================================ (5) the tell
export async function closeTell(): Promise<boolean> {
  console.log('\n=== close-tell: "the character changes when you\'re close enough to do a melee attack" — the hand goes to the knife at every bearing in reach; "sam doesn\'t pull out his knife … just reaches for it" ===');
  allPass = true; const S = await stage(1112, { ai: false }), { W, g, A, pl } = S, an = pl.char.anim; const hasReach = 'reachShown' in an;
  const IN = 30, OUT = 36;   // (0.5 s · 0.6 s)
  { const rows: string[] = [], fKind: string[] = [], fReach: string[] = [], fKnife: string[] = [];
    for (const b of ROUND) { const want = kindAt(b) === 'behind' ? 'grab' : 'strike'; pl.holstered = true; far(S, 50); const cold = { ready: an.ready, reach: hasReach ? an.reachShown : 0, knife: an.knifeShown };
      if (cold.ready || cold.reach > 0.1 || cold.knife) fKind.push(`${b}°: 6 m off him the pose is not down (ready ${cold.ready}, reach ${f2(cold.reach)}, knife ${cold.knife})`);
      const at = samAt(S, b, 0.8); let nul = 0, wrong = 0, up = -1, knife = 0, far0 = 0;
      for (let f = 1; f <= IN; f++) { if (f > 1) { W.step(DT); pl.visibility = 0.02; } if (an.ready == null) nul++; else if (an.ready !== want) wrong++; 
        if (an.ready && (!an.readyAt || distXZ(an.readyAt, A.char.pos) > 0.5)) far0++; if (hasReach && up < 0 && an.reachShown >= 0.9) up = f; if (an.knifeShown) knife++; }
      const reach = closeAt(g, A, pl.char.pos).win?.reach ?? 1; const out = samAt(S, b, reach + 0.15); const gone = an.ready; let down = -1;
      for (let f = 1; f <= OUT; f++) { if (f > 1) { W.step(DT); pl.visibility = 0.02; } if (an.ready != null) wrong++; if (hasReach && down < 0 && an.reachShown < 0.1) down = f; if (an.knifeShown) knife++; }
      rows.push(`${String(b).padStart(4)}° (${kindAt(b)}) ${f2(at.d)} m: ${want}${nul || wrong ? ' ✗' : ''} · hand on the hilt at frame ${hasReach ? up : '—'} · ${f2(out.d)} m: ${gone ?? 'null'} · off it at frame ${hasReach ? down : '—'} · knife out ${knife} f`);
      if (nul) fKind.push(`${b}°: anim.ready null on ${nul} of ${IN} frames in reach`); if (wrong) fKind.push(`${b}°: the wrong pose on ${wrong} frames (want ${want} in reach, null ${f2(reach + 0.15)} m out)`); if (far0) fKind.push(`${b}°: readyAt is not on the man`);
      if (hasReach && (up < 0 || down < 0)) fReach.push(`${b}°: reachShown ${up < 0 ? `never ≥ 0.9 inside ${IN} frames` : ''}${down < 0 ? ` not under 0.1 inside ${OUT} frames of stepping out` : ''}`);
      if (knife) fKnife.push(`${b}°: anim.knifeShown true on ${knife} frames of the tell`); }
    verdict(!fKnife.length, 'the knife is NOT out in the tell: anim.knifeShown false on every frame at every bearing', fKnife.length ? `${WAIT} — the draw still runs with the ready pose (KNIFE_DRAW's clock goes out with any live pose that has a knife fist)${bad(fKnife)}` : 'never true'); }
  { const fails: string[] = [], rows: string[] = []; pl.holstered = false;
    for (const b of ROUND) { const want = kindAt(b) === 'behind' ? 'grabGun' : null; far(S, 50); samAt(S, b, 0.8); let wrong = 0, reach = 0, knife = 0;
      for (let f = 1; f <= IN; f++) { if (f > 1) W.step(DT); if ((an.ready ?? null) !== want) wrong++; if (hasReach) reach = Math.max(reach, an.reachShown); if (an.knifeShown) knife++; }
      rows.push(`${b}° ${an.ready ?? 'null'}`); if (!pl.pistolUp) fails.push('staging: the pistol is not up'); if (wrong) fails.push(`${b}°: ${wrong} frames not ${want}`); if (reach > 0.1) fails.push(`${b}°: reachShown ${f2(reach)} with the pistol up`); if (knife) fails.push(`${b}°: knifeShown on ${knife} frames`); }
    pl.holstered = true;
    verdict(!fails.length, 'the pistol up: \'grabGun\' behind him (the free hand only), nothing from a side or the front; no hand on the hilt, no knife', rows.join(' · ') + bad(fails)); }
  { const S2 = await stage(1112, {}), g2 = S2.g, an2 = g2.player.char.anim, n = seatDown(S2); S2.pl.holstered = true; samAt(S2, 180, 1.0, 2); const k = an2.ready, fails: string[] = []; let up = -1, knife = 0;
    for (let f = 1; f <= IN; f++) { S2.W.step(DT); g2.player.visibility = 0.02; if (hasReach && up < 0 && an2.reachShown >= 0.9) up = f; if (an2.knifeShown) knife++; }
    if (n < 0) fails.push('he never sat down'); if (k !== 'seat') fails.push(`anim.ready ${k}`); if (hasReach && up < 0) fails.push('reachShown never ≥ 0.9'); if (knife) fails.push(`knifeShown on ${knife} frames (${WAIT})`);
    samAt(S2, 0, 0.8, 2); const front = an2.ready; if (closeSeated(g2, S2.A) && front !== 'strike') fails.push(`in front of the chair: ${front}`);
     }
  return allPass;
}

// ================================================================ (6) melee
/** One blow through the real click (button 1 = the left, 4 = the right): Sam on his spot (one settled frame — the row), the click, 100 frames. `pre` runs between the two. */
function blow(S: any, bDeg: number, d: number, button: 1 | 4, who: any = S.A, pre?: () => void) {
  const { W, g, pl } = S, A = who, an = pl.char.anim; const at = samAt(S, bDeg, d, 1, A); pre?.();
  const p0 = v3.copy(A.char.pos) as Vec3, found = g.nearby.find((x: any) => x.kind === 'guard' && x.inReach && x.guard === A), row = found && { ...found };   // (the copy's rows are written again in place: what the row said is kept here)
  const st0 = { ...g.mission.stats }, bl0 = { ...CLOSE_STATS.blows }, takes0 = { ...CLOSE_STATS.takes }, ev0 = g.chronicle.events.length, seen = new Set<any>(), ev: { level: number; f: number }[] = [], sam0 = v3.copy(pl.char.pos) as Vec3;
  W.input.clicked |= button; W.step(DT); pl.visibility = 0.02;
  const td0 = pl.takedown ? { kind: (pl.takedown.kind ?? 'behind') as string, lethal: !!pl.takedown.lethal, alert: !!pl.takedown.alert } : null;
  let minD = distXZ(pl.char.pos, p0), deadAt = -1, f = 0, knife0 = -1, knife1 = -1, yawErr = NaN;
  const look = () => { minD = Math.min(minD, distXZ(pl.char.pos, p0)); for (const e of g.events) if (e.kind === 'prop' && e.who === PLAYER_ID && !seen.has(e)) { seen.add(e); ev.push({ level: e.level, f }); }
    if (an.knifeShown) { if (knife0 < 0) knife0 = f; knife1 = f; }
    if (deadAt < 0 && A.state === 'dead') { deadAt = f; yawErr = wrapAngle(Math.atan2(p0[0] - pl.char.pos[0], p0[2] - pl.char.pos[2]) - pl.char.bodyYaw); } };
  look(); for (f = 1; f <= 100; f++) { W.step(DT); pl.visibility = 0.02; look(); }
  const takes: Record<string, number> = {}; for (const k of Object.keys(takes0)) takes[k] = (CLOSE_STATS.takes as any)[k] - (takes0 as any)[k];
  const rows = g.chronicle.events.slice(ev0).map((e: any) => ({ kind: e.kind as string, weapon: e.data?.weapon as string | undefined, text: fmtEvent(e) as string }));
  return { at, row, td0, p0, minD, deadAt, yawErr, slid: distXZ(pl.char.pos, sam0), endD: distXZ(pl.char.pos, p0), ev, takes, rows, knife0, knife1,
    kills: g.mission.stats.kills - st0.kills, kos: g.mission.stats.knockouts - st0.knockouts, knifeBlows: CLOSE_STATS.blows.knife - bl0.knife, koBlows: CLOSE_STATS.blows.knockout - bl0.knockout,
    dead: A.state === 'dead', breathing: isBreathing(A), down: A.downKind as string | null, over: !pl.takedown };
}
/** what a landed blow must leave: `lethal` = the knife's books, else the knockout's; `kind` = the way in the table gives for where Sam stood */
function judgeBlow(r: any, lethal: boolean, kind: string, o: { books?: boolean, takes?: boolean } = {}): string[] {
  const fails: string[] = [], station = kind === 'behind' ? 0.55 : kind === 'seat' ? CLOSE.seatDist : CLOSE.strikeDist;
  if (!r.row) fails.push('no row in reach before the click'); else if (r.row.close !== kind) fails.push(`the row in reach says ${r.row.close} (want ${kind})`);
  if (!r.td0) { fails.push('the click started no blow'); return fails; }
  if (r.td0.kind !== kind) fails.push(`the blow is a '${r.td0.kind}' one (want ${kind})`); if (r.td0.lethal !== lethal) fails.push(`takedown.lethal ${r.td0.lethal}`);
  if (!r.dead) { fails.push('he is NOT down'); return fails; }
  if (r.deadAt < 12 || r.deadAt > 24) fails.push(`it landed on frame ${r.deadAt} (the cross lands at 0.28 s = 17)`);
  if (lethal) { if (r.down !== 'shot' || r.breathing) fails.push(`the knife left him '${r.down}', breathing ${r.breathing} (want 'shot', not breathing)`); if (r.knifeBlows !== 1 || r.koBlows) fails.push(`blows: knife +${r.knifeBlows}, knockout +${r.koBlows}`); }
  else { if (r.down !== 'struck' || !r.breathing) fails.push(`the knockout left him '${r.down}', breathing ${r.breathing} (want 'struck', breathing)`); if (r.koBlows !== 1 || r.knifeBlows) fails.push(`blows: knife +${r.knifeBlows}, knockout +${r.koBlows}`); }
  if (o.books !== false) { if (lethal) { if (r.kills !== 1 || r.kos) fails.push(`the card: kills +${r.kills}, knockouts +${r.kos}`); if (!r.rows.some((e: any) => e.kind === 'kill' && e.weapon === 'knife' && /killed — the knife/.test(e.text))) fails.push(`no 'kill' row by the knife (${r.rows.map((e: any) => e.kind).join(', ') || 'no rows'})`); }
    else { if (r.kos !== 1 || r.kills) fails.push(`the card: kills +${r.kills}, knockouts +${r.kos}`); if (!r.rows.some((e: any) => e.kind === 'ko')) fails.push(`no 'ko' row (${r.rows.map((e: any) => e.kind).join(', ') || 'no rows'})`); if (r.rows.some((e: any) => e.kind === 'kill')) fails.push('a \'kill\' row for a knockout'); } }
  for (const k of Object.keys(r.takes)) if (r.takes[k] !== (k === kind && o.takes !== false ? 1 : 0)) fails.push(`CLOSE_STATS.takes.${k} moved by ${r.takes[k]}`);
  const want = kind === 'behind' ? 0.3 : CLOSE.noise[kind] + (r.td0.alert ? CLOSE.alertNoise : 0), fall = r.ev.filter((e: any) => e.f === r.deadAt); if (!fall.some((e: any) => Math.abs(e.level - want) < 1e-9)) fails.push(`the fall's 'prop' event: [${fall.map((e: any) => e.level).join(', ')}] on the landing frame (want ${want})`);
  if (r.minD < 0.25) fails.push(`Sam came within ${f2(r.minD)} m of the man's centre`); if (!(Math.abs(r.yawErr) < 20 * DEG)) fails.push(`he faced ${dg(r.yawErr)} off the man at the blow`);
  if (kind === 'behind') { if (Math.abs(r.endD - station) > 0.15) fails.push(`Sam ends ${f2(r.endD)} m from where the man stood (want ${station} ± 0.15)`); }
  else if (r.slid > Math.abs(r.at.d - station) + 0.1) fails.push(`Sam moved ${f2(r.slid)} m (a strike from where he stands: the ${f2(Math.abs(r.at.d - station))} m in to ${station} m and no further)`);
  if (!r.over) fails.push('the blow never let go of Sam');
  return fails;
}
const blowLine = (r: any) => `${dg(r.at.b).padStart(7)} ${f2(r.at.d)} m ${(r.td0?.kind ?? 'no blow').padEnd(6)} → ${r.dead ? `${r.down}${r.breathing ? ', breathing' : ', dead'} at frame ${r.deadAt}` : 'not down'} · fall [${r.ev.map((e: any) => e.level).join(', ')}] · card k+${r.kills} ko+${r.kos} · ${r.rows.filter((e: any) => /kill|ko/i.test(e.kind)).map((e: any) => e.text.replace(/^.*· /, '')).join(' ; ') || 'no row'} · Sam ${f2(r.endD)} m off, moved ${f2(r.slid)} · knife out ${r.knife0 < 0 ? 'never' : `frames ${r.knife0}‥${r.knife1}`}`;
export async function closeMelee(): Promise<boolean> {
  console.log('\n=== close-melee: "melee attacks work from all angles" — the right click is the knockout, at every 30° ===');
  allPass = true;
  const sweep: any[] = [];
  { const rows: string[] = [], fails: string[] = [];
    for (const b of ROUND) { const S = await stage(1104, {}), r = blow(S, b, 0.75, 4), kind = kindAt(b); sweep.push(r); rows.push(blowLine(r));
      for (const x of judgeBlow(r, false, kind)) fails.push(`${b}°: ${x}`); }
     }
  { const fk: string[] = [];
    for (const r of sweep) if (r.knife0 >= 0) fk.push(`${dg(r.at.b)}: out on frames ${r.knife0}‥${r.knife1} of a knockout`);
    verdict(!fk.length, 'the knockout never shows the knife', fk.length ? bad(fk) : 'never out at any bearing'); }
  { const S = await stage(1104, { ai: false }); S.A.state = 'alert'; S.A.awareness = 1; const r = blow(S, 0, 0.75, 4), fails = judgeBlow(r, false, 'front');
    if (!r.td0?.alert) fails.push('staging: the blow\'s own note does not say he was alert');
    verdict(!fails.length, `an ALERT man facing Sam, the right click: struck all the same, and louder (${CLOSE.noise.front} + ${CLOSE.alertNoise})`, blowLine(r) + bad(fails)); }
  { const fails: string[] = [], rows: string[] = []; let who = '';
    for (const button of [4] as const) { const S = await stage(1104, {}), cv = (S.g.civilians ?? []).find((x: any) => x.civ && !closeSeated(S.g, x) && x.state !== 'dead'); if (!cv) { fails.push('staging: no standing civilian on this stand'); break; } who = cv.callsign;
      const r = blow(S, 180, 0.8, button, cv); rows.push(`right: ${r.td0 ? `lethal ${r.td0.lethal}` : 'no blow'} → ${r.dead ? `${r.down}, breathing ${r.breathing}` : 'not down'} · kills +${r.kills} · blows knife +${r.knifeBlows} knockout +${r.koBlows} · rows ${r.rows.map((e: any) => e.kind).join(', ')}`);
      if (Math.abs(r.at.d - 0.8) > 0.05) fails.push(`staging: Sam ${f2(r.at.d)} m behind her`); if (!r.td0 || r.td0.lethal) fails.push(`button ${button}: ${r.td0 ? 'a LETHAL blow' : 'no blow'}`); if (!r.dead || r.down !== 'struck' || !r.breathing) fails.push(`button ${button}: she is ${r.dead ? `'${r.down}', breathing ${r.breathing}` : 'not down'}`);
      if (r.kills || r.knifeBlows || r.rows.some((e: any) => e.kind === 'kill')) fails.push(`button ${button}: a kill on the books`); if (r.koBlows !== 1) fails.push(`button ${button}: blows.knockout +${r.koBlows}`); }
    verdict(!fails.length, 'a CIVILIAN from behind: the right click puts her out cold — never a kill', `${who}: ${rows.join('\n    ')}${bad(fails)}`); }
  { const S = await stage(1104, { ai: false }), { pl, A } = S; pl.holstered = false; const fails: string[] = [], rows: string[] = [];
    for (const [b, button] of [[180, 4], [0, 4], [180, 1], [0, 1]] as const) { const r = blow(S, b, 0.75, button); rows.push(`${b}° ${button === 1 ? 'left' : 'right'}: ${r.td0 ? 'a blow' : 'no blow'}`);
      if (!pl.pistolUp) fails.push('staging: the pistol is not up'); if (r.td0 || r.knifeBlows || r.koBlows || Object.values(r.takes).some(n => n)) fails.push(`${b}° button ${button}: a blow with the pistol up`); if (A.downKind === 'struck') fails.push(`${b}° button ${button}: he was struck`); }
    verdict(!fails.length, 'the pistol up: neither button starts a blow (the left click is a round, as ever)', rows.join(' · ') + bad(fails)); }
  return allPass;
}

// ================================================================ (7) the grab
/** Space on his grab row; the frames until 'held' */
function grabRun(S: any, bDeg: number, d: number) {
  const { W, g, A, pl } = S; const at = samAt(S, bDeg, d); const found = rowsOf(g, A).find((x: any) => x.verb === 'grab'), row = found && { ...found }, g0 = { ...CLOSE_STATS.grabs };
  const tapped = tapVerb(W, 'guard', 'grab'); const H = pl.holding; const tr: number[] = []; let heldAt = -1;
  if (H) { tr.push(distXZ(pl.char.pos, A.char.pos)); for (let f = 1; f <= 150; f++) { W.step(DT); pl.visibility = 0.02; if (!pl.holding) break; tr.push(distXZ(pl.char.pos, A.char.pos)); if (pl.holding.phase === 'held') { heldAt = f; break; } } }
  const grabs: Record<string, number> = {}; for (const k of Object.keys(g0)) grabs[k] = (CLOSE_STATS.grabs as any)[k] - (g0 as any)[k];
  return { at, row, tapped, H: !!H, heldAt, tr, variant: H?.variant, grabs, held: !!A.held };
}
/** Space where there should be no grab: a tap, then held half a second and let go. → what started */
function spaceNothing(S: any) { const { W, g, A, pl } = S; const sel = g.selectVerb('guard', 'grab'); W.input.pressed.add(IK); W.step(DT); const tap = !!pl.holding || !!pl.takedown;
  W.input.keys.add(IK); W.input.pressed.add(IK); for (let i = 0; i < 30; i++) W.step(DT); W.input.keys.delete(IK); for (let i = 0; i < 5; i++) W.step(DT); if (pl.sel?.open) { W.input.pressed.add('Escape'); W.step(DT); }
  return { sel, started: tap || !!pl.holding || !!pl.takedown || !!A.held || A.state === 'dead' }; }
export async function closeGrab(): Promise<boolean> {
  console.log('\n=== close-grab: "grabbing only works from behind" — the grab row and Space at every 30° (and each side of the arc\'s edge); a seated man ===');
  allPass = true; const EDGE = Math.acos(-0.25) / DEG;   // 104.48°
  { const S = await stage(1105, { ai: false }), { g, A } = S, fails: string[] = [], rows: string[] = [];
    for (const b of [...ROUND, 103, -103]) if (kindAt(b) !== 'behind') { const at = samAt(S, b, 0.8), rs = rowsOf(g, A), gr = rs.find((x: any) => x.verb === 'grab'), st = rs.find((x: any) => !x.verb && x.inReach), n = spaceNothing(S);
      rows.push(`${b}° ${f2(at.d)} m: ${gr ? 'A GRAB ROW' : 'no grab row'}${n.started ? ' · SOMETHING STARTED' : ''}`);
      if (gr) fails.push(`${b}°: a grab row "${gr.line2}"`); if (n.sel) fails.push(`${b}°: selectVerb(guard, grab) found a row`); if (n.started) fails.push(`${b}°: Space started something (holding ${!!S.pl.holding}, takedown ${!!S.pl.takedown}, held ${!!A.held}, ${A.state})`);
      if (!st || !/the grab is from behind$/.test(st.line2)) fails.push(`${b}°: the one row in reach reads "${st?.line2}"`); }
    verdict(!fails.length, `outside the rear arc (every 30° of it, and 103° each hand — the arc begins at ${EDGE.toFixed(2)}°): no grab row, and Space — tapped, then held half a second — starts nothing; the row says the grab is from behind`, rows.join(' · ') + bad(fails)); }
  { const fails: string[] = [], rows: string[] = [];
    for (const b of [...ROUND, 106, -106]) if (kindAt(b) === 'behind') { const S = await stage(1105, {}), r = grabRun(S, b, 0.8); rows.push(`${b}°: ${r.H ? `held at frame ${r.heldAt}` : 'NOTHING'}`);
      if (!r.row || !/^Space\s+grab him\s+·/.test(r.row.line2) || r.row.close !== 'behind') fails.push(`${b}°: row "${r.row?.line2}" close ${r.row?.close}`); if (!r.tapped || !r.H || !r.held) fails.push(`${b}°: Space took nobody`); else if (r.heldAt < 0 || r.heldAt > 60) fails.push(`${b}°: 'held' at frame ${r.heldAt}`);
      for (const k of Object.keys(r.grabs)) if (r.grabs[k] !== (k === 'behind' && r.H ? 1 : 0)) fails.push(`${b}°: CLOSE_STATS.grabs.${k} moved by ${r.grabs[k]}`); if (S.pl.takedown || S.A.state === 'dead') fails.push(`${b}°: Space struck him`); }
    verdict(!fails.length, 'inside the rear arc (every 30° of it, and 106° each hand): the grab row, Space takes him — \'held\' inside a second, grabs.behind +1', rows.join(' · ') + bad(fails)); }
  { const on = grabRun(await stage(1105, {}), 180, 0.8), want = Math.round(HOLD.secs.grab / DT), fails: string[] = [];
    if (!on.H) fails.push('the tap took nobody'); if (Math.abs(on.heldAt - want) > 1) fails.push(`'held' at frame ${on.heldAt} (HOLD.secs.grab ${HOLD.secs.grab} s = ${want})`);
    verdict(!fails.length, 'from straight behind: \'held\' on HOLD.secs.grab', `held at frame ${on.heldAt} (${HOLD.secs.grab} s = ${want})${bad(fails)}`); }
  { const fails: string[] = [], rows: string[] = [];
    for (const b of [180, 120, -120]) { const S = await stage(1105, {}), n = seatDown(S), r = grabRun(S, b, 1.15); rows.push(`${b}° ${f2(r.at.d)} m: ${r.H ? `held at frame ${r.heldAt}` : 'NOTHING'}`);
      if (n < 0) fails.push(`${b}°: he never sat down`); if (!(r.at.d > CLOSE.reach.behind && r.at.d < CLOSE.reach.seat)) fails.push(`staging ${b}°: ${f2(r.at.d)} m is not between the standing reach and the seat's`);
      if (!r.row || !/haul him out of the chair/.test(r.row.line2) || r.row.close !== 'seat') fails.push(`${b}°: row "${r.row?.line2}" close ${r.row?.close}`); if (!r.H || r.heldAt < 0) fails.push(`${b}°: not hauled out into the hold`); if (r.grabs.seat !== (r.H ? 1 : 0)) fails.push(`${b}°: grabs.seat +${r.grabs.seat}`);
      if (closeSeated(S.g, S.A) || S.A.char.anim.act?.clip === 'Sitting_Idle_Loop') fails.push(`${b}°: he is still sitting`); }
    const S = await stage(1105, {}), { g, A } = S; S.g.aiEnabled = true; const n = seatDown(S); g.aiEnabled = false; if (n < 0) fails.push('front: he never sat down');
    for (const b of [0, 60, -90, 100]) { const at = samAt(S, b, 0.8), gr = rowsOf(g, A).find((x: any) => x.verb === 'grab'), o = closeAt(g, A, S.pl.char.pos), s = spaceNothing(S); rows.push(`front ${b}°: ${gr ? 'A GRAB ROW' : 'no grab row'} (${o.win?.id}/${o.win?.kind}, strike ${o.strike})`);
      if (gr || s.sel || s.started || o.grab) fails.push(`seated, ${b}°: grab row ${!!gr}, offer.grab ${o.grab}, Space started ${s.started}`); if (o.win?.kind !== 'front' || !o.strike) fails.push(`seated, ${b}° ${f2(at.d)} m: the window is ${o.win?.kind}, strike ${o.strike}`); if (!closeSeated(g, A)) fails.push(`seated, ${b}°: he is no longer sitting`); }
    verdict(!fails.length, 'a SEATED man: hauled out of the chair from the rear arc at 1.15 m (beyond the standing reach) at 180° and ±120°; from his front (0°, 60°, −90°, 100°) no grab row, Space starts nothing, the strike is on', rows.join('\n    ') + bad(fails)); }
  return allPass;
}

// ================================================================ (8) the held sneak
const HELD_KEYS = ['footL', 'footR', 'toeL', 'toeR', 'handL', 'handR'];
/** Sam crouched on a puppet goal east past the man (1.2 m off his left side: outside every window's reach — no tell in the arms), the goal dropped at four
 *  places and put back. Each stop: the frame the ROOT comes to rest, then the phase and the bones (off the root) 0 / 10 / 30 / 90 frames on. */
async function heldWalk() {
  const S = await stage(1113, { ai: false }), { W, g, A, pl } = S, c = pl.char, an = c.anim;
  const Z = LOBBY[2] + 1.2, from: Vec3 = [LOBBY[0] - 1.9, 0, Z], goal: Vec3 = [LOBBY[0] + 1.8, 0, Z], STOPS = [-1.2, -0.6, 0, 0.6].map(x => LOBBY[0] + x);
  g.teleportPlayer(from); const exact = distXZ(c.pos, from) < 1e-6; c.bodyYaw = c.aimYaw = EAST; pl.crouch = true; an.crouchTarget = 1;
  g.puppet = { goal: null, aim: [goal[0] + 2, 1, Z], crouch: true };
  const snap = () => ({ ph: an.phase as number, rel: HELD_KEYS.map(k => [c.bones[k][0] - c.pos[0], c.bones[k][1], c.bones[k][2] - c.pos[2]]) });
  let frames = 0, inMode = 0, ready = 0;
  const step = () => { const p = an.phase; W.step(DT); pl.visibility = 0.02; frames++; if (closeMode(g) === A && an.creep) inMode++; if (an.ready) ready++; let d = an.phase - p; if (d < -0.5) d += 1; return d as number; };
  const stops: any[] = []; let ref = 0;
  const walk = (until: (n: number) => boolean) => { const adv: number[] = []; g.puppet.goal = goal; for (let n = 0; n < 400 && !until(n); n++) adv.push(step()); return adv; };
  for (const sx of STOPS) { const adv = walk(() => c.pos[0] >= sx); if (!stops.length) ref = Math.max(...adv.slice(20)); else stops[stops.length - 1].resume = adv.slice(0, 30);
    g.puppet.goal = null; const ph0 = an.phase, x0 = c.pos[0]; let rest = -1; for (let i = 1; i <= 150 && rest < 0; i++) { const q = v3.copy(c.pos); step(); if (distXZ(q, c.pos) < 1e-9) rest = i; }
    const coast = c.pos[0] - x0; let coastPh = an.phase - ph0; if (coastPh < -0.5) coastPh += 1; const s0 = snap(); for (let i = 0; i < 10; i++) step(); const s10 = snap(); for (let i = 0; i < 20; i++) step(); const s30 = snap(); for (let i = 0; i < 60; i++) step(); const s90 = snap();
    stops.push({ sx, x: c.pos[0], rest, coast, coastPh, s0, s10, s30, s90, resume: [] as number[] }); }
  stops[stops.length - 1].resume = walk(n => n >= 30);
  g.puppet = null;
  return { stops, ref, frames, inMode, ready, exact, crouched: pl.crouch };
}
const boneGap = (a: number[][], b: number[][], n = a.length) => { let m = 0; for (let i = 0; i < n; i++) m = Math.max(m, Math.hypot(a[i][0] - b[i][0], a[i][1] - b[i][1], a[i][2] - b[i][2])); return m; };
export async function closeHeld(): Promise<boolean> {
  console.log('\n=== close-held: "pausing movement sticks in the current position, not reverting back to an idle state" — the sneak stopped four times in one walk past a man ===');
  allPass = true; const on = await heldWalk();
  const spread = (r: any) => { let m = 0; for (const a of r.stops) for (const b of r.stops) m = Math.max(m, boneGap(a.s90.rel, b.s90.rel, 4)); return m; };   // (feet and toes only)
  { const fails: string[] = []; if (!on.exact || !on.crouched) fails.push('staging: Sam is not on his start, crouched'); if (on.stops.length !== 4 || on.stops.some((s: any) => s.rest < 0)) fails.push(`staging: ${on.stops.filter((s: any) => s.rest >= 0).length} of 4 stops came to rest`);
    if (on.inMode !== on.frames) fails.push(`in the mode (closeMode on the man, anim.creep) on ${on.inMode} of ${on.frames} frames`); if (on.ready) fails.push(`a tell pose on ${on.ready} frames (the walk passes outside every reach)`);
    verdict(!fails.length, 'the walk: crouched on a puppet goal 1.2 m past his left side, in the mode on every frame, no tell in the arms; four stops, each coming to rest', `${on.frames} frames · stops at x ${on.stops.map((s: any) => f2(s.x)).join(', ')} (the man at ${LOBBY[0]}) · the root at rest ${on.stops.map((s: any) => s.rest).join(' / ')} frames after the goal is dropped, having coasted ${on.stops.map((s: any) => (s.coast * 100).toFixed(1)).join(' / ')} cm (the controller's own run-out: the stride moves on with that ground — phase +${on.stops.map((s: any) => f3(s.coastPh)).join(' / ')} — and stops with it)${bad(fails)}`); }
  { const fails: string[] = [], rows: string[] = [];
    for (const s of on.stops) { const d = Math.max(Math.abs(s.s10.ph - s.s0.ph), Math.abs(s.s30.ph - s.s0.ph), Math.abs(s.s90.ph - s.s0.ph)); rows.push(`x ${f2(s.x)}: phase ${s.s0.ph.toFixed(6)} → ${s.s30.ph.toFixed(6)} → ${s.s90.ph.toFixed(6)}`); if (!(d <= 1e-6)) fails.push(`x ${f2(s.x)}: the phase moved ${d.toExponential(2)} while he stood`); }
    verdict(!fails.length, 'stopped: anim.phase unchanged to 1e-6 from the frame the root is at rest through 10, 30 and 90 frames on', rows.join(' · ') + bad(fails)); }
  { const fails: string[] = [], rows: string[] = [];
    const handsM = KITANIM.on ? 0.05 : 0.02;   // (kit handling on, kitanim.ts: his knife hand follows how near the man is and the wrists are alive on the layer's clock — the hands get 5 cm, feet and toes stay at 2)
    for (const s of on.stops) { const a = boneGap(s.s30.rel, s.s10.rel), b = boneGap(s.s90.rel, s.s10.rel), fa = boneGap(s.s30.rel, s.s10.rel, 4), fb = boneGap(s.s90.rel, s.s10.rel, 4); rows.push(`x ${f2(s.x)}: ${(a * 100).toFixed(2)} / ${(b * 100).toFixed(2)} cm`);
      if (fa > 0.02 || fb > 0.02) fails.push(`x ${f2(s.x)}: a foot / toe ${(Math.max(fa, fb) * 100).toFixed(1)} cm from where it was 10 frames after the stop`); else if (a > handsM || b > handsM) fails.push(`x ${f2(s.x)}: a hand ${(Math.max(a, b) * 100).toFixed(1)} cm from where it was 10 frames after the stop`); }
    verdict(!fails.length, `stopped: both feet, both toes and both hands within 2 cm${KITANIM.on ? ' (the hands 5 cm: kit handling on)' : ''} (30 and 90 frames on) of where they were 10 frames after the stop — breathing apart, nothing eases anywhere`, rows.join(' · ') + bad(fails)); }
  { const sOn = spread(on), fails: string[] = [];
    if (!(sOn >= 0.25)) fails.push(`the four held stances differ by at most ${(sOn * 100).toFixed(1)} cm in the feet (want ≥ 25: each stop in the stride it stopped in)`);
    verdict(!fails.length, 'NOT the idle: the four stops stand in four different strides (feet / toes ≥ 25 cm apart between stops)', `the stops' feet differ by up to ${(sOn * 100).toFixed(1)} cm${bad(fails)}`); }
  { const fails: string[] = [], rows: string[] = [];
    for (const s of on.stops) { const mx = Math.max(...s.resume), mn = Math.min(...s.resume), sum = s.resume.reduce((a: number, b: number) => a + b, 0); rows.push(`x ${f2(s.x)}: first step +${s.resume[0].toFixed(4)}, largest +${mx.toFixed(4)}, 30 frames +${f3(sum)}`);
      if (s.resume.length < 30) fails.push(`x ${f2(s.x)}: only ${s.resume.length} frames walked on`); if (mx > on.ref * 1.1 + 1e-6) fails.push(`x ${f2(s.x)}: a frame's phase step of ${mx.toFixed(4)} on resuming (the walk's own largest: ${on.ref.toFixed(4)})`); if (mn < -1e-9) fails.push(`x ${f2(s.x)}: the phase went back ${mn.toExponential(2)}`); if (!(sum > 0.02)) fails.push(`x ${f2(s.x)}: the stride did not move on (${f3(sum)} in 30 frames)`); }
    verdict(!fails.length, 'resumed: the phase goes on FROM the held value — no frame\'s step above the steady walk\'s own (× 1.1), never back, and the stride does move on', `the walk in: at most +${on.ref.toFixed(4)} a frame · ${rows.join(' · ')}${bad(fails)}`); }
  console.log(`  (the numbers used, off character.ts creepGait / CREEP: the held gait's share comes in over gaitIn 0.2 s and under it the phase is advanced by the ground the ROOT covered (crV) — so 'stopped' is taken from the first frame the root moves less than 1e-9 m, not from the frame the goal is dropped; bones are read off the root, in the world's axes)`);
  return allPass;
}

// ================================================================ (9) the dark
/** One man on a mark in the break room, Sam crouch-walking a 4 m line past him on a puppet goal with his meter fed from the CPU light estimate. `path` 'front' = across
 *  his front 1.2 m ahead of him, 'side' = down his right side 1.2 m off, front to back. `torch` false = his torch put out every frame (the room's own light alone). */
async function darkCross(o: { lights: boolean; path: 'front' | 'side'; torch: boolean }) {
  Math.random = mulberry32(11);
  const W: any = await standUp(); const g = W.game;
  g.restartEncounter(); g.playerInvisible = false; g.godMode = true; g.setGroup('breakroom', o.lights);
  const pl = g.player, c = pl.char; pl.slot = 'pistol'; pl.holstered = true; pl.pistol.lightOn = false; pl.nvOn = false;
  { const dw = g.doors.byName('breakroom_w'); if (dw) { dw.angle = 0; dw.vel = 0; dw.latched = true; dw.closing = false; } }   // the room's west door shut for the walk: the shipped stand's one-way leaves (doors.ts oneWay) open this one INTO the room, across Sam's start on the front path — teleported into its sweep he knocks it shut, and that slam (a 'door' event, 0.58, two metres off) is what the man then hears. Shut on both stands, so both walk the same room
  const [A, B, C] = g.guards; for (const [i, gd] of [B, C].entries()) { gd.hold = true; gd.pinned = true; gd.char.pos = [-40, 0, -40 - i]; gd.heardUpTo = 1e12; }
  const mx = 31.2, mz = 16.0, yaw = 0, d = 1.2; const put = () => { A.char.pos[0] = mx; A.char.pos[2] = mz; A.char.bodyYaw = A.char.aimYaw = yaw; if (!o.torch) A.flashlight.enabled = false; };
  A.hold = true; A.pinned = true; A.state = 'patrol'; A.awareness = 0; A.wait = 1e6; put();
  const fx = Math.sin(yaw), fz = Math.cos(yaw), rx = Math.cos(yaw), rz = -Math.sin(yaw);
  const P = (s: number): Vec3 => o.path === 'side' ? [mx + rx * d - fx * s, 0, mz + rz * d - fz * s] : [mx + fx * d + rx * s, 0, mz + fz * d + rz * s];
  const from = P(-2), to = P(2); g.teleportPlayer(from); const exact = distXZ(c.pos, from) < 1e-6; pl.crouch = true; c.anim.crouchTarget = 1; pl.pace = 0.7;
  g.puppet = { goal: null, aim: [to[0], 1, to[2]], crouch: true };
  for (let i = 0; i < 30; i++) { put(); feedMeter(g); W.step(DT); }
  g.puppet.goal = to; let maxAw = 0, minD = 9, maxVis = 0, seen = '', f = 0, arrived = false, inCone = 0; const dp0 = CLOSE_STATS.darkPasses;
  for (; f < 900 && !arrived; f++) { put(); feedMeter(g); W.step(DT); const dd = distXZ(c.pos, A.char.pos); minD = Math.min(minD, dd); maxAw = Math.max(maxAw, A.awareness); if (dd < 1.8) { maxVis = Math.max(maxVis, pl.visibility); inCone++; }
    if (!seen && A.state !== 'patrol') seen = `${A.state} at frame ${f}`; arrived = distXZ(c.pos, to) < 0.15; }
  g.puppet = null;
  return { exact, arrived, f, maxAw, minD, maxVis, seen, near: inCone, passes: CLOSE_STATS.darkPasses - dp0, state: A.state as string };
}
/** THE EXHIBIT (the lead's staging, lifted): three men on marks in the dark break room, torches burning, each looking his own way; Sam threads between them on a
 *  puppet path, freezes mid-stride while one of them turns his way, goes on when he has turned back. Their senses are live. */
const SLITHER = { men: [{ x: 30.5, z: 16.4, yaw: 0 }, { x: 31.9, z: 15.2, yaw: 180 }, { x: 33.3, z: 16.5, yaw: 90 }], from: [28.6, 13.0], yaw0: 60,
  path: [[30.3, 15.3], [31.2, 15.8], [32.05, 16.3], [32.75, 15.75], [33.6, 15.0], [34.4, 13.6]], pace: 0.7, pause: { leg: 2, k: 0.45, frames: 100, man: 1, turnDeg: -38, lead: 20, hold: 40 } };
async function slither() {
  Math.random = mulberry32(11);
  const W: any = await standUp(); const g = W.game, K = SLITHER, P = K.pause;
  Math.random = mulberry32(11); g.restartEncounter(); g.playerInvisible = false; g.godMode = true; g.puppet = null; g.setGroup('breakroom', false);
  const men = g.guards.slice(0, K.men.length); men.forEach((gd: any, k: number) => { const m = K.men[k]; gd.char.pos = [m.x, 0, m.z]; gd.char.bodyYaw = gd.char.aimYaw = m.yaw * DEG; gd.hold = true; gd.pinned = true; gd.state = 'patrol'; gd.awareness = 0; gd.wait = 1e6; });
  const pl = g.player, c = pl.char; pl.slot = 'pistol'; pl.holstered = true; pl.pistol.lightOn = false; pl.nvOn = false;
  g.teleportPlayer([K.from[0], 0, K.from[1]]); c.bodyYaw = c.aimYaw = K.yaw0 * DEG; pl.crouch = true; c.anim.crouchTarget = 1; pl.pace = K.pace; g.puppet = { goal: null, aim: null, crouch: true };
  for (let i = 0; i < 30; i++) { feedMeter(g); W.step(DT); }
  let leg = 0, st = 0, t0 = 0, f = 0, done = false, seen = '', modeF = 0, heldF = 0, maxVis = 0; const maxAw = men.map(() => 0), minD = men.map(() => 9), an = c.anim; let phHeld = NaN, phMoved = 0;
  for (; f < 3600 && !done; f++) { const p = c.pos;
    men.forEach((gd: any, k: number) => { const m = K.men[k]; gd.char.pos[0] = m.x; gd.char.pos[2] = m.z; let yaw = m.yaw;
      if (k === P.man && st >= 1) { const t = f - t0, u = t < P.lead ? t / P.lead : t < P.lead + P.hold ? 1 : Math.max(0, 1 - (t - P.lead - P.hold) / P.lead); yaw += P.turnDeg * (u * u * (3 - 2 * u)); }
      gd.char.bodyYaw = gd.char.aimYaw = yaw * DEG; maxAw[k] = Math.max(maxAw[k], gd.awareness); if (!seen && gd.state !== 'patrol') seen = `${gd.callsign} ${gd.state} at frame ${f}`; });
    if (leg >= K.path.length) { g.puppet.goal = null; if (st !== 9) { st = 9; t0 = f; } done = f - t0 > 50; }
    else { const a = leg === 0 ? K.from : K.path[leg - 1], b = K.path[leg], L = Math.hypot(b[0] - a[0], b[1] - a[1]), k = ((p[0] - a[0]) * (b[0] - a[0]) + (p[2] - a[1]) * (b[1] - a[1])) / (L * L);
      if (st === 0 && leg === P.leg && k >= P.k) { st = 1; t0 = f; } if (st === 1 && f - t0 >= P.frames) st = 2;
      const nxt = K.path[leg + 1] || b; g.puppet.aim = [nxt[0], 1.0, nxt[1]]; g.puppet.goal = st === 1 ? null : [b[0], 0, b[1]]; if (st !== 1 && Math.hypot(p[0] - b[0], p[2] - b[1]) < 0.16) leg++; }
    feedMeter(g); const q = v3.copy(c.pos); W.step(DT);
    men.forEach((gd: any, k: number) => { minD[k] = Math.min(minD[k], distXZ(c.pos, gd.char.pos)); }); if (Math.min(...men.map((gd: any) => distXZ(c.pos, gd.char.pos))) < 3) maxVis = Math.max(maxVis, pl.visibility);
    if (an.creep) modeF++; if (st === 1 && distXZ(q, c.pos) < 1e-9) { heldF++; if (Number.isNaN(phHeld)) phHeld = an.phase; else phMoved = Math.max(phMoved, Math.abs(an.phase - phHeld)); } }
  g.puppet = null;
  return { done, f, seen, maxAw, minD, maxVis, modeF, heldF, phMoved, names: men.map((gd: any) => gd.callsign as string) };
}
export async function closeDark(): Promise<boolean> {
  console.log('\n=== close-dark: the one rule of the men\'s eyes — the near-sight floor goes by the light on Sam; the table, sightGain, and played in the break room with its lights off ===');
  allPass = true;
  { const S = await stage(1114, { ai: false }), { g, A, pl } = S, fails: string[] = [], rows: string[] = []; const ss = (v: number) => { const t = Math.max(0, Math.min(1, (v - CLOSE.darkAt) / (CLOSE.litAt - CLOSE.darkAt))); return t * t * (3 - 2 * t); };
    for (const state of ['patrol', 'suspicious', 'search', 'alert']) for (const dist of [0.5, 1.0]) { const got: string[] = [];
      for (const vis of [0, 0.1, 0.2, 0.3, 1]) { A.state = state; pl.visibility = vis; const k = closeNearK(g, A, dist), want = state === 'alert' || state === 'search' || dist < CLOSE.touchM ? 1 : ss(vis); got.push(String(+k.toFixed(3))); if (Math.abs(k - want) > 1e-12) fails.push(`${state} ${dist} m, light ${vis}: ${k} (want ${want})`); }
      rows.push(`${state.padEnd(10)} ${dist} m: ${got.join(' / ')}`); }
    verdict(!fails.length, `closeNearK by the light on Sam (0 / 0.1 / 0.2 / 0.3 / 1): nothing under darkAt ${CLOSE.darkAt}, all of it from litAt ${CLOSE.litAt}; always 1 for a man searching or alert, inside touchM ${CLOSE.touchM}`, rows.join('\n    ') + bad(fails));
    const f2s: string[] = []; pl.visibility = 0; A.state = 'patrol'; A.smokeTrans = 1; const g0 = sightGain(g, A, 1.0), gTouch = sightGain(g, A, 0.5), gFar = sightGain(g, A, 2.5); pl.visibility = 1; const gLit = sightGain(g, A, 1.0); pl.visibility = 0; A.state = 'search'; const gHunt = sightGain(g, A, 1.0); A.state = 'patrol'; pl.visibility = 0.02;
    if (g0 !== 0) f2s.push(`pitch dark, 1 m: ${g0} a second (want 0)`); if (!(gTouch >= 3.5)) f2s.push(`0.5 m: ${gTouch}`); if (gFar !== 0) f2s.push(`2.5 m in the dark: ${gFar}`); if (!(gLit >= 3.5)) f2s.push(`lit: ${gLit}`); if (!(gHunt >= 3.5 * 2.2 - 1e-9)) f2s.push(`a searching man: ${gHunt}`);
    verdict(!f2s.length, 'guards.ts sightGain under it, Sam in pitch dark: 0 a second at 1 m, the build\'s floor at 0.5 m, for a searching man, and for Sam lit', `dark 1 m: ${g0} · dark 0.5 m: ${gTouch} · dark 2.5 m: ${gFar} · lit 1 m: ${f2(gLit)} · searching, dark 1 m: ${f2(gHunt)}${bad(f2s)}`); }
  const line = (r: any) => `${r.f} frames · nearest ${f2(r.minD)} m, ${r.near} frames inside 1.8 m · the light on Sam there at most ${f3(r.maxVis)} · his awareness at most ${f2(r.maxAw)} · ${r.seen || 'never anything but patrol'} · dark passes ${r.passes}`;
  const staged = (r: any, fails: string[]) => { if (!r.exact || !r.arrived) fails.push(`staging: Sam ${r.exact ? '' : 'not on his start'} ${r.arrived ? '' : 'never reached the far end'}`); if (Math.abs(r.minD - 1.2) > 0.08) fails.push(`staging: he passed ${f2(r.minD)} m off the man (want 1.2)`); return fails; };
  { const a = await darkCross({ lights: false, path: 'front', torch: false }), fails = staged(a, []);
    if (!(a.maxAw < 0.3) || a.state !== 'patrol') fails.push(`he noticed: awareness ${f2(a.maxAw)}, ${a.seen}`); if (!(a.maxVis < CLOSE.darkAt)) fails.push(`staging: the light on Sam reached ${f3(a.maxVis)} (not dark)`); if (!a.passes) fails.push('closeNearK never passed anything (darkPasses 0)');
    verdict(!fails.length, 'the room dark, his torch out: Sam crouch-walks across his front at 1.2 m — awareness stays under 0.3, he never leaves his patrol', line(a) + bad(fails)); }
  { const a = await darkCross({ lights: true, path: 'front', torch: false }), fails = staged(a, []);
    if (a.state === 'patrol') fails.push(`the room lit: he noticed nothing (awareness ${f2(a.maxAw)})`); if (!(a.maxVis >= CLOSE.litAt)) fails.push(`staging: the light on Sam only reached ${f3(a.maxVis)} with the room's lights on`);
    verdict(!fails.length, 'the room LIT: he is onto Sam', line(a) + bad(fails)); }
  { const a = await darkCross({ lights: false, path: 'front', torch: true }), fails = staged(a, []); if (a.state === 'patrol') fails.push(`he noticed nothing walking through his beam (awareness ${f2(a.maxAw)})`); if (!(a.maxVis >= CLOSE.litAt)) fails.push(`staging: his torch put only ${f3(a.maxVis)} on Sam`);
    verdict(!fails.length, 'the room dark, but his torch BURNING and Sam walking through its beam: seen — the light term is the build\'s, the rule gives nothing to a lit man', line(a) + bad(fails)); }
  { const a = await darkCross({ lights: false, path: 'side', torch: true }), fails = staged(a, []);
    if (!(a.maxAw < 0.3) || a.state !== 'patrol') fails.push(`he noticed (awareness ${f2(a.maxAw)}, ${a.seen})`);
    verdict(!fails.length, 'his torch burning, Sam down his side 1.2 m off, out of the beam: unnoticed', line(a) + bad(fails)); }
  { const a = await slither(), fails: string[] = []; const say = (r: any) => `${r.done ? 'through' : 'TIMED OUT'} in ${r.f} frames · ${r.seen || 'nobody noticed'} · awareness at most ${r.maxAw.map(f2).join(' / ')} (${r.names.join(', ')}) · nearest ${r.minD.map(f2).join(' / ')} m · the light on Sam inside 3 m at most ${f3(r.maxVis)} · in the mode ${r.modeF} frames · stood frozen ${r.heldF} frames, the stride held to ${r.phMoved.toExponential(1)}`;
    if (!a.done) fails.push('Sam never got through'); if (a.seen || a.maxAw.some((x: number) => x >= 0.3)) fails.push(`${a.seen || 'awareness past 0.3'}`); if (a.minD.filter((d: number) => d < 1.8).length < 2) fails.push(`staging: Sam passed inside 1.8 m of ${a.minD.filter((d: number) => d < 1.8).length} men`);
    // (the freeze is a hundred frames from the frame he is told to stop: his root coasts to rest for about seventy of them from the old crouch-walk and two more from
    //  the sneak's pace of two gears — player.ts TWOGEARS — so with that key up the bar on frames AT REST is 25; with it down it is the 30 it was. With the key up the staging's own slow pace is nobody's either: he threads them at the sneak's pace, which is what the page does)
    if (a.heldF < (TWOGEARS.on ? 25 : 30) || a.phMoved > 1e-6) fails.push(`the freeze: ${a.heldF} frames at rest, the phase moved ${a.phMoved.toExponential(1)}`);
    verdict(!fails.length, 'THE EXHIBIT — three men in the dark break room, torches burning, Sam threading between them inside arm\'s reach and freezing mid-stride while one turns: nobody notices', `${say(a)}${bad(fails)}`); }
  return allPass;
}

// ================================================================ (6) the picture
export async function closeMarksProbe(): Promise<boolean> {
  console.log('\n=== close-marks: the windows as floor polygons are off on the page — nothing is handed to the overlay ===');
  allPass = true; const S = await stage(1106, { ai: false }), { g } = S;
  { samAt(S, 180, 1.5); const shipped = closeMarks(g).length; verdict(CLOSE_DEFAULTS.marks === false && CLOSE.marks === false && shipped === 0, 'the floor marks are OFF (CLOSE.marks false): nothing is handed to the overlay', `CLOSE_DEFAULTS.marks ${CLOSE_DEFAULTS.marks} · CLOSE.marks ${CLOSE.marks} · ${shipped} marks 1.5 m behind him`); }
  return allPass;
}

// ================================================================ (8) the seat
export async function closeSeatProbe(): Promise<boolean> {
  console.log('\n=== close-seat: a calm staff man sat down on a mark — the clip, the hold flag, up the moment he is not calm; the click and Space over the chair; a restart ===');
  allPass = true;
  { const S = await stage(1109, { holdA: false }), { W, g, A, B, pl } = S; g.teleportPlayer([LOBBY[0], 0, LOBBY[2] - 7]); W.step(DT);
    const mark = v3.copy(A.char.pos) as Vec3, holdWas = A.hold, seats0 = CLOSE_STATS.seats, n = seatDown(S), fails: string[] = []; let drift = 0;
    const clip = A.char.anim.act?.clip, shown = A.char.anim.actShown; for (let i = 0; i < 300; i++) { W.step(DT); pl.visibility = 0.02; drift = Math.max(drift, distXZ(A.char.pos, mark)); if (!closeSeated(g, A)) { fails.push(`up again on frame ${i} of the 300 (${A.state}, awareness ${f2(A.awareness)})`); break; } }
    if (n < 0 || n > 90) fails.push(`seated after ${n} frames (want ≤ 90)`); if (clip !== 'Sitting_Idle_Loop') fails.push(`act clip ${clip}`); if (A.hold !== true) fails.push('the hold flag is not up while seated'); if (drift > 0.45) fails.push(`he drifted ${f2(drift)} m off the mark`);
    if (CLOSE_STATS.seats !== seats0 + 1 || !/seated 1/.test(closeSummary(g))) fails.push(`tally: seats +${CLOSE_STATS.seats - seats0}, ${closeSummary(g)}`);
    verdict(!fails.length, 'closeSeat on a calm man in the open: seated inside 90 frames, the sitting clip, on the hold, within 0.45 m of his mark for 300 frames', `seated in ${n} frames (act shown ${f2(shown)}) · clip ${clip} · hold ${holdWas} → ${A.hold} · furthest ${f2(drift)} m from the mark (the seat is ${0.33} m ahead of it) · ${closeSummary(g)}${bad(fails)}`);
    const un0 = CLOSE_STATS.unseats; A.state = 'suspicious'; A.awareness = 0.6; W.step(DT); const f2s: string[] = [];
    if (closeSeated(g, A) || !/seated 0/.test(closeSummary(g))) f2s.push('still seated the frame after'); if (A.hold !== holdWas) f2s.push(`hold ${A.hold} (it was ${holdWas})`); if (A.char.anim.act) f2s.push(`act still ${A.char.anim.act.clip}`); if (CLOSE_STATS.unseats !== un0 + 1) f2s.push('unseats did not count it');
    verdict(!f2s.length, 'roused (suspicious, awareness 0.6): up on the next frame — off the seat list, the hold flag as it was, the clip gone', `seated ${closeSeated(g, A)} · hold ${A.hold} · act ${A.char.anim.act?.clip ?? 'none'} · state ${A.state}${bad(f2s)}`);
    place(B, [LOBBY[0] + 1.5, 0, LOBBY[2] - 1.5], EAST); B.state = 'alert'; const rAlert = closeSeat(g, B), cv = (g.civilians ?? [])[0], rCiv = cv ? closeSeat(g, cv) : null, f3s: string[] = [];
    if (rAlert) f3s.push('an alert man was sat down'); if (rCiv) f3s.push('a civilian was sat down'); if (!/seated 0/.test(closeSummary(g))) f3s.push(closeSummary(g)); if (!cv) console.log('  (no civilian on this stand: that refusal not asked)');
    verdict(!f3s.length, 'closeSeat refuses an alert man and a civilian', `alert → ${rAlert} · civilian (${cv?.callsign ?? '—'}) → ${rCiv}${bad(f3s)}`); }
  { const S = await stage(1109, {}), n = seatDown(S), r = blow(S, 180, 1.12, 4), fails = n < 0 ? ['he never sat down'] : judgeBlow(r, false, 'seat');
    if (!(r.at.d > CLOSE.reach.behind && r.at.d < CLOSE.reach.seat)) fails.push(`staging: ${f2(r.at.d)} m`);
     }
  { const S = await stage(1109, {}), { g, A, pl } = S, n = seatDown(S), r = grabRun(S, 180, 1.12), fails: string[] = [];
    if (n < 0) fails.push('he never sat down'); if (!r.row || !/haul him out of the chair/.test(r.row.line2) || r.row.close !== 'seat') fails.push(`row: "${r.row?.line2}" close ${r.row?.close}`);
    if (!r.H || !pl.holding) fails.push('Space took nobody'); if (closeSeated(g, A) || !/seated 0/.test(closeSummary(g))) fails.push('he is still seated'); if (A.char.anim.act?.clip === 'Sitting_Idle_Loop') fails.push('the sitting clip is still on him');
    if (r.heldAt < 0) fails.push('never \'held\'');
    verdict(!fails.length, 'Space on a seated man from 1.12 m behind: hauled out of the chair into the hold — pl.holding set, no longer seated', `row "${r.row?.line2}" · held at frame ${r.heldAt} · ${closeSummary(g)}${bad(fails)}`); }
  { const S = await stage(1109, {}), { W, g } = S, n = seatDown(S), before = closeSummary(g); g.restartEncounter(); const after = closeSummary(g); for (let i = 0; i < 30; i++) W.step(DT); const later = closeSummary(g), fails: string[] = [];
    if (n < 0 || !/seated 1/.test(before)) fails.push(`staging: ${before}`); if (!/seated 0/.test(after) || !/seated 0/.test(later)) fails.push(`after the restart: ${after} · 30 frames on: ${later}`); if (g.guards.some((x: any) => x.char.anim.act?.clip === 'Sitting_Idle_Loop' || closeSeated(g, x))) fails.push('a man of the new shift is sitting');
    verdict(!fails.length, 'restartEncounter leaves nobody seated', `before: ${before}\n    after:  ${after}${bad(fails)}`); }
  return allPass;
}

// ================================================================ (14) the two-person game
/** THE DUO STAND (lifted from the pair's own family — pairup-probes.ts `rig`: the room clear on the office with ONE teammate, PAIRUP.on set before the Game is made,
 *  the squad told silent): Sam's hands empty, every defender but Tango 1 and 2 quietly off the floor. `room` = the break room with its
 *  lights off and Tango 1 held on the dark-crossing's mark facing north-of-screen (+z), calm whatever he sees (`calm`), Sam 3.1 m behind him, the partner at Sam's
 *  shoulder on 'fall in'; else the pair family's own aisle marks. */
const BR = { man: [31.2, 0, 16.0] as Vec3, yaw: 0, sam: [31.2, 0, 12.9] as Vec3, mate: [30.2, 0, 13.0] as Vec3 };
async function duoStage(o: { calm?: boolean; room?: boolean } = {}) {
  const { rig } = await import('./pairup-probes.ts');
  const R: any = await rig({ up: true });
  const g = R.game, pl = g.player, m = R.mate; pl.slot = 'pistol'; pl.holstered = true;
  const T1 = R.tango(1), T2 = R.tango(2); R.only(T1, T2);
  let mateMode = 0, orders = new Set<string>(); R.post = () => { if (closeMode(g) === m) mateMode++; const A = R.O.of(m); orders.add(A ? `${A.order}${A.post === null ? '' : '@post'}` : 'none'); };
  if (o.room !== false) { g.setGroup('breakroom', false); R.pin(T1, { at: BR.man, yaw: BR.yaw, ...(o.calm === false ? {} : { state: 'patrol', aw: 0, eyesOn: null }) }); T1.flashlight.enabled = false; R.pin(T2, { at: [20, 0, 11], yaw: EAST });
    g.teleportPlayer(BR.sam); R.put(m, BR.mate, 0); R.step(5); const why = R.O.give('fallIn', R.O.targetAt(pl.char.pos), 'all'); if (why) throw new Error(`close-duo: 'fall in' refused — ${why}`); R.step(60); orders.clear(); }
  /** Sam on a spot off Tango 1 (bearing off his facing, metres), `frames` settled */
  const at = (bDeg: number, d: number, frames = 2, who: any = T1) => { g.teleportPlayer(off(who, bDeg, d)); for (let i = 0; i < frames; i++) R.step(); return { b: bearingOf(who.char.pos, who.char.bodyYaw, pl.char.pos), d: distXZ(pl.char.pos, who.char.pos) }; };
  const order = () => { const A = R.O.of(m); return A ? `${A.order}${A.post === null ? '' : '@post'}` : 'none'; };
  return { R, g, pl, m, T1, T2, at, order, orders, mateMode: () => mateMode };
}
export async function closeDuo(): Promise<boolean> {
  console.log('\n=== close-duo: "make sure all the sneaking stuff works in the duo mode" — the pair on (PAIRUP.on, one partner, silent, \'fall in\'): the partner is nobody\'s subject, Sam\'s set works on a defender, the partner\'s own orders still work ===');
  allPass = true; const K = await import(`${ROOT}/src/game/knobs.ts`), keep = K.knobsCopy(), rnd = Math.random;
  try {
  { // (1) the partner is never a subject
    const S = await duoStage({ room: false }), { R, g, pl, m, T1, T2 } = S, an = pl.char.anim, fails: string[] = [], rows: string[] = [];
    R.pin(T1, { at: [22, 0, 17.6], yaw: EAST }); R.pin(T2, { at: [20, 0, 11], yaw: EAST }); g.teleportPlayer([11, 0, 18.5]); R.put(m, [13, 0, 17.6], EAST); R.step(10); R.O.give('fallIn', R.O.targetAt(pl.char.pos), 'all'); R.step(30);
    if (!R.P.up || m.team !== 'operative') fails.push(`staging: the pair is ${R.P.up ? 'up' : 'NOT up'}, the partner's team ${m.team}`);
    for (const b of [180, 90, 0]) for (const button of [1, 4] as const) { R.put(m, [13, 0, 17.6], EAST); const w = S.at(b, 0.8, 2, m), bl0 = { ...CLOSE_STATS.blows }; let td = 0;
      const subj = closeSubject(g, m), row = g.nearby.some((x: any) => x.kind === 'guard' && x.guard === m) || g.interactables.some((x: any) => x.kind === 'guard' && x.guard === m), ready = an.ready ?? null, mode = closeMode(g);
      R.W.input.clicked |= button; for (let i = 0; i < 40; i++) { R.step(); if (pl.takedown) td++; }
      rows.push(`${b}° ${f2(w.d)} m, ${button === 1 ? 'left' : 'right'}: subject ${subj} · row ${row} · tell ${ready} · mode ${mode?.callsign ?? 'off'} · blow ${td ? 'YES' : 'none'}`);
      if (w.d > 1.0) fails.push(`staging ${b}°: Sam ${f2(w.d)} m off him`); if (subj) fails.push(`${b}°: closeSubject true for the partner`); if (row) fails.push(`${b}°: a guard row / marker on the partner`); if (ready) fails.push(`${b}°: the tell '${ready}' with only the partner in reach`); if (mode) fails.push(`${b}°: the mode is on ${mode.callsign}`);
      if (td || m.state === 'dead' || m.downKind || CLOSE_STATS.blows.knife !== bl0.knife || CLOSE_STATS.blows.knockout !== bl0.knockout) fails.push(`${b}° button ${button}: a blow on the partner (takedown ${td} frames, ${m.state}, ${m.downKind})`); }
    if (S.mateMode()) fails.push(`closeMode returned the partner on ${S.mateMode()} frames`);
    verdict(!fails.length, 'the PARTNER, Sam at arm\'s length behind him, beside him and in front of him, each button clicked: never a subject — closeSubject false, no row, no tell, no mode, no blow', `${m.callsign}: ${rows.join('\n    ')}${bad(fails)}`); }
  { // (2) Sam's set on a defender, the partner at his shoulder
    const fails: string[] = [], rows: string[] = [];
    for (const [b, button] of [[180, 4], [-90, 4], [0, 4]] as const) { const S = await duoStage(), { R, g, pl, m, T1 } = S, an = pl.char.anim, lethal: boolean = false, kind = kindAt(b), what = `${b}° right`;
      const off0 = closeMode(g), d0 = distXZ(pl.char.pos, T1.char.pos); R.walk([BR.sam[0], 0, 13.6], 1.0, 10); const on1 = closeMode(g), d1 = distXZ(pl.char.pos, T1.char.pos), creep = an.creep, mateD = distXZ(m.char.pos, pl.char.pos);
      const w = S.at(b, 0.75), tell = an.ready, want = kind === 'behind' ? 'grab' : 'strike', row = g.nearby.find((x: any) => x.kind === 'guard' && x.inReach && x.guard === T1);
      const st0 = { ...g.mission.stats }, bl0 = { ...CLOSE_STATS.blows }; R.W.input.clicked |= button; R.step(); const td = pl.takedown ? { lethal: !!pl.takedown.lethal, kind: pl.takedown.kind ?? 'behind' } : null; let deadAt = -1; for (let f = 1; f <= 100; f++) { R.step(); if (deadAt < 0 && T1.state === 'dead') deadAt = f; }
      rows.push(`${what.padEnd(11)}: mode off at ${f2(d0)} m, on at ${f2(d1)} m (partner ${f2(mateD)} m behind Sam) · tell ${tell} · ${td ? `${td.kind}${td.lethal ? ', lethal' : ''}` : 'no blow'} → ${T1.state === 'dead' ? `${T1.downKind}${isBreathing(T1) ? ', breathing' : ', dead'} at frame ${deadAt}` : 'NOT DOWN'} · partner: ${[...S.orders].join(', ')} · ${R.status()}`);
      if (off0 || d0 < CLOSE.modeM) fails.push(`${what}: staging — mode ${off0?.callsign ?? 'off'} at ${f2(d0)} m`); if (on1 !== T1 || !creep || !(d1 < CLOSE.modeM)) fails.push(`${what}: at ${f2(d1)} m the mode is ${on1?.callsign ?? 'off'}, creep ${creep}`); if (mateD > 2.5) fails.push(`${what}: staging — the partner is ${f2(mateD)} m from Sam`);
      if (tell !== want) fails.push(`${what}: the tell is ${tell} (want ${want})`); if (!row || row.close !== kind || (kind === 'behind') !== (row.verb === 'grab')) fails.push(`${what}: row "${row?.line2}" close ${row?.close}`);
      if (!td || td.lethal !== lethal || td.kind !== kind) fails.push(`${what}: ${td ? `a '${td.kind}' blow, lethal ${td.lethal}` : 'no blow'}`);
      if (T1.state !== 'dead' || T1.downKind !== (lethal ? 'shot' : 'struck') || isBreathing(T1) === lethal) fails.push(`${what}: he is ${T1.state === 'dead' ? `'${T1.downKind}', breathing ${isBreathing(T1)}` : 'not down'}`);
      if (CLOSE_STATS.blows.knife - bl0.knife !== (lethal ? 1 : 0) || CLOSE_STATS.blows.knockout - bl0.knockout !== (lethal ? 0 : 1)) fails.push(`${what}: blows knife +${CLOSE_STATS.blows.knife - bl0.knife}, knockout +${CLOSE_STATS.blows.knockout - bl0.knockout}`);
      if (g.mission.stats.kills - st0.kills !== (lethal ? 1 : 0)) fails.push(`${what}: the card's kills +${g.mission.stats.kills - st0.kills}`);
      if ([...S.orders].some(x => x !== 'fallIn') || m.state === 'dead' || S.mateMode()) fails.push(`${what}: the partner — orders ${[...S.orders].join(', ')}, ${m.state}, in Sam's mode ${S.mateMode()} frames`); }
    verdict(!fails.length, 'a DEFENDER in the dark break room, the partner following at Sam\'s shoulder: the mode on inside modeM; the tell \'grab\' behind / \'strike\' at a side and in front; the right click the knockout (breathing), from behind, a side and the front; the partner on \'fall in\' on every frame of it', rows.join('\n    ') + bad(fails)); }
  { // (2, 4) the grab, the held gait, the menu's right button
    const S = await duoStage(), { R, g, pl, m, T1 } = S, an = pl.char.anim, fails: string[] = [];
    // the held gait: crouched, east along z 14.8 (1.2 m behind his back — past the rear reach: no tell in the arms), by the keys (a puppet would switch the squad's orders off)
    g.teleportPlayer([29.8, 0, 14.8]); pl.crouch = true; R.W.cam.yaw = 0; R.W.cam.rebuild?.(); R.step(20); const x0 = pl.char.pos[0]; let inMode = 0, tell = 0, n = 0; const tick = () => { R.step(); pl.crouch = true; n++; if (closeMode(g) === T1 && an.creep) inMode++; if (an.ready) tell++; };
    R.W.input.keys.add('KeyD'); for (let i = 0; i < 70; i++) tick(); R.W.input.keys.delete('KeyD'); const went = pl.char.pos[0] - x0;
    let rest = -1; for (let i = 1; i <= 150 && rest < 0; i++) { const q = v3.copy(pl.char.pos); tick(); if (distXZ(q, pl.char.pos) < 1e-9) rest = i; }
    const rel = () => HELD_KEYS.map(k => [pl.char.bones[k][0] - pl.char.pos[0], pl.char.bones[k][1], pl.char.bones[k][2] - pl.char.pos[2]]); for (let i = 0; i < 10; i++) tick(); const ph0 = an.phase, r0 = rel(); for (let i = 0; i < 80; i++) tick(); const ph1 = an.phase, moved = boneGap(rel(), r0);
    R.W.input.keys.add('KeyD'); let adv = 0; for (let i = 0; i < 30; i++) { const p = an.phase; tick(); let d = an.phase - p; if (d < -0.5) d += 1; adv += d; } R.W.input.keys.delete('KeyD'); for (let i = 0; i < 90; i++) tick();
    if (went < 0.8 || rest < 0) fails.push(`staging: he walked ${f2(went)} m, at rest after ${rest} frames`); if (inMode !== n || tell) fails.push(`in the mode on ${inMode} of ${n} frames, a tell on ${tell}`); if (Math.abs(ph1 - ph0) > 1e-6 || moved > 0.02) fails.push(`stopped: the phase moved ${Math.abs(ph1 - ph0).toExponential(1)}, a foot / hand ${(moved * 100).toFixed(1)} cm, over 80 frames`); if (!(adv > 0.02)) fails.push(`resumed: the stride moved on ${f3(adv)} in 30 frames`);
    // the grab: only from behind
    const f2s: string[] = []; pl.crouch = false; S.at(90, 0.8); const sideSel = g.selectVerb('guard', 'grab'); R.W.input.pressed.add(IK); R.step(); const sideHeld = !!pl.holding || !!pl.takedown; S.at(0, 0.8); const frontSel = g.selectVerb('guard', 'grab'); R.W.input.pressed.add(IK); R.step(); const frontHeld = !!pl.holding || !!pl.takedown;
    // the menu's right button, a defender in reach of Sam's empty hands
    S.at(180, 0.8); const f4: string[] = [], inReach = g.nearby.some((x: any) => x.kind === 'guard' && x.inReach && x.guard === T1), opened = R.O.open(R.O.targetAt([pl.char.pos[0] - 1.2, 0, pl.char.pos[2] - 0.6])), up = !!R.O.menu, bl0 = { ...CLOSE_STATS.blows };
    R.W.input.clicked |= 4; R.step(); const menuAfter = !!R.O.menu, tdMenu = !!pl.takedown; for (let i = 0; i < 30; i++) R.step(); const stillUp = T1.state !== 'dead' && !pl.takedown && CLOSE_STATS.blows.knockout === bl0.knockout;
    if (!inReach || !opened || !up) f4.push(`staging: in reach ${inReach}, the menu opened ${opened} / up ${up}`); if (menuAfter) f4.push('the right click left the menu up'); if (tdMenu || !stillUp) f4.push(`the right click with the menu up started a knockout (takedown ${tdMenu}, he is ${T1.state})`);
    verdict(!f4.length, 'the order menu keeps the right button: the menu open, a defender in reach of Sam\'s empty hands — the right click puts the menu away and starts NO knockout', `in reach ${inReach} · menu up ${up} → ${menuAfter ? 'still up' : 'gone'} · takedown that frame ${tdMenu} · he is ${T1.state} 30 frames on${bad(f4)}`);
    S.at(180, 0.8); const sel = g.selectVerb('guard', 'grab'); R.W.input.pressed.add(IK); R.step(); const H = !!pl.holding; const held = R.until(() => pl.holding?.phase === 'held', 1.5); R.step(60);
    if (sideSel || sideHeld || frontSel || frontHeld) f2s.push(`a side: row ${sideSel}, started ${sideHeld} · the front: row ${frontSel}, started ${frontHeld}`); if (!sel || !H || !held || !T1.held) f2s.push(`behind: row ${sel}, holding ${H}, 'held' ${held}`); if (m.state === 'dead' || S.order() !== 'fallIn') f2s.push(`the partner: ${m.state}, ${S.order()}`);
    verdict(!f2s.length, 'Space beside a partner: no grab row and nothing started at the defender\'s side or front; from behind he is taken into the hold — the partner still on \'fall in\'', `side: row ${sideSel} · front: row ${frontSel} · behind: ${H ? `held ${held}` : 'NOTHING'} · partner ${[...S.orders].join(', ')} (${R.status()})${bad(f2s)}`); }
  { // (3) the partner's own quiet order afterwards
    const run = async () => { const S = await duoStage({ room: false }), { R, g, pl, m, T1, T2 } = S;
      R.pin(T1, { at: [22, 0, 17.6], yaw: EAST }); R.pin(T2, { at: LOBBY, yaw: EAST, state: 'patrol', aw: 0, eyesOn: null }); R.put(m, [9.5, 0, 22.0], 0); R.step(5);
      S.at(180, 0.8, 2, T2); R.W.input.clicked |= 4; R.step(); const td = !!pl.takedown; for (let i = 0; i < 90; i++) R.step(); const first = { td, state: T2.state as string, down: T2.downKind as string | null, breathing: isBreathing(T2) };
      g.teleportPlayer([11, 0, 18.5]); R.put(m, [13, 0, 17.6], EAST); R.step(10); const fall = R.O.give('fallIn', R.O.targetAt(pl.char.pos), 'all'); R.step(30); const before = S.order();
      const f0 = R.frames, why = R.O.give('pairMark', R.foe(T1), 'all'), set = R.until(() => !!R.P.jobOf(m)?.set, 30), setAt = R.frames - f0, sent = set && R.P.mark('api'), down = R.until(() => T1.state === 'dead', 15), downAt = R.frames - f0; R.step(30);
      return { first, fall, before, why, set, setAt, sent, down, downAt, kind: T1.downKind as string | null, breathing: isBreathing(T1), after: S.order(), status: R.status() as string, mate: m.state as string, line: R.lines(/he's down/).slice(-1)[0] ?? '(nothing)' }; };
    const on = await run(), fails: string[] = []; const say = (r: any) => `Sam's own first: ${r.first.down}${r.first.breathing ? ', breathing' : ''} · 'fall in' → ${r.before} · 'take him · on your mark' ${r.why ? `REFUSED: ${r.why}` : 'given'} · set after ${(r.setAt / 60).toFixed(1)} s · the mark ${r.sent} · down ${r.down ? `after ${(r.downAt / 60).toFixed(1)} s: ${r.kind}${r.breathing ? ', breathing' : ''}` : 'NEVER'} · then ${r.after} (${r.status}) · "${r.line}"`;
    { const r = on; if (r.first.state !== 'dead' || r.first.down !== 'struck') fails.push(`staging — Sam's knockout of the first man left him ${r.first.state} / ${r.first.down}`); if (r.fall || r.before !== 'fallIn') fails.push(`'fall in' after it → ${r.fall ?? r.before}`); if (r.why || !r.set || !r.sent || !r.down) fails.push(`the order — refused ${r.why}, set ${r.set}, sent ${r.sent}, down ${r.down}`); if (r.mate === 'dead') fails.push('the partner is down'); }
    verdict(!fails.length, 'after Sam\'s own knockout of one man: \'take him · on your mark\' on a second, then the mark — he goes down by the partner\'s hands', `${say(on)}${bad(fails)}`); }
  { // (5) the dark rule is Sam's alone
    const SPOT: Vec3 = [BR.man[0], 0, BR.man[2] + 1.2], fails: string[] = [];
    const alone = async (who: 'partner' | 'sam') => { const S = await duoStage({ calm: false, room: false }), { R, g, pl, m, T1, T2 } = S; g.setGroup('breakroom', false); R.pin(T1, { at: BR.man, yaw: BR.yaw }); T1.flashlight.enabled = false; R.pin(T2, { at: [20, 0, 11], yaw: EAST });
      if (who === 'partner') g.teleportPlayer([11, 0, 18.5]); else R.put(m, [11, 0, 18.5], 0); R.O.give('hold', R.O.targetAt(m.char.pos), 'all'); const dp = CLOSE_STATS.darkPasses; let maxAw = 0, first = '';
      for (let f = 0; f < 180; f++) { T1.flashlight.enabled = false; if (who === 'partner') { R.put(m, SPOT, Math.PI); m.char.vel[0] = m.char.vel[2] = 0; } else { g.teleportPlayer(SPOT); pl.crouch = true; } R.step(); maxAw = Math.max(maxAw, T1.awareness); if (!first && T1.state !== 'patrol') first = `${T1.state} at frame ${f}`; }
      return { maxAw, first, passes: CLOSE_STATS.darkPasses - dp, foe: T1.foe === m ? 'the partner' : T1.foe === 'player' ? 'Sam' : String(T1.foe), lit: R.lit(SPOT) as number, samD: distXZ(pl.char.pos, T1.char.pos) }; };
    const p = await alone('partner'), s = await alone('sam');
    if (!(p.lit < CLOSE.darkAt)) fails.push(`staging: the spot reads ${f3(p.lit)} on the meter (not dark)`); if (p.passes) fails.push(`CLOSE_STATS.darkPasses moved by ${p.passes} with only the partner inside 1.8 m of the man (Sam ${f1(p.samD)} m off)`); if (!p.first || p.foe !== 'the partner') fails.push(`the partner 1.2 m in front of him in the dark: ${p.first || `never roused (awareness ${f2(p.maxAw)})`}, his eyes on ${p.foe}`);
    if (!s.passes || s.first || s.maxAw >= 0.3) fails.push(`Sam on the same spot: dark passes ${s.passes}, ${s.first || 'calm'}, awareness ${f2(s.maxAw)}`);
    verdict(!fails.length, 'the dark rule is SAM\'s alone (by decision): the partner stood 1.2 m in front of a defender in the dark is seen by the build\'s near-sight floor and closeNearK is never asked (darkPasses +0); Sam on the same spot is passed', `the spot's meter ${f3(p.lit)} · the partner there, Sam ${f1(p.samD)} m away: ${p.first || 'calm'}, his eyes on ${p.foe}, dark passes +${p.passes} · Sam there, the partner away: ${s.first || 'calm'} (awareness ${f2(s.maxAw)}), dark passes +${s.passes}${bad(fails)}`);
  }
  } finally { K.knobsBack(keep); Math.random = rnd; }
  return allPass;
}

/** the whole card (`bun run tools/qa/close-probes.ts`): true = every probe passed */
export const CLOSE_PROBES = [['close-mode', closeModeProbe], ['close-tell', closeTell], ['close-melee', closeMelee], ['close-grab', closeGrab], ['close-held', closeHeld], ['close-dark', closeDark], ['close-windows', closeWindowsProbe], ['close-offer', closeOffer], ['close-rows', closeRowsProbe], ['close-marks', closeMarksProbe], ['close-seat', closeSeatProbe], ['close-duo', closeDuo]] as const;
async function closeAll(want: readonly string[]): Promise<boolean | null> {
  const t0 = performance.now(), timed = Object.fromEntries(CLOSE_PROBES.map(([name, fn]) => [name, async (): Promise<boolean> => { const t = performance.now(); try { return await fn(); } finally { if (VERBOSE) console.log(`  (${name}: ${((performance.now() - t) / 1000).toFixed(1)} s)`); } }]));
  const ok = await asProbes(timed, want);
  console.log(`\n=== close: ${ok === null ? 'NOTHING RAN' : ok ? 'ALL PASS' : 'FAILURES above'} === (${((performance.now() - t0) / 1000).toFixed(1)} s)`); return ok;
}
if (import.meta.main) process.exit(exitCode(await closeAll(wanted())));
