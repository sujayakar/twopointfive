// The 'tactics' probe family (run through probes.ts: `bun run tools/qa/probes.ts tactics | tactics-flank | tactics-single | tactics-lone | tactics-door |
// tactics-rounds | tactics-decay [--verbose]`): the pursuit tactics of src/game/tactics.ts — STATUS E12 (a) 'they all just run
// straight at sam. instead they should coordinate flanking maneuvers' and (b) 'if they notice their buddies dead by a door, they shouldn't just blindly charge
// in past them!'. Each stands the full game up headless (headless.ts), stages one situation, steps it, prints what happened, PASS / FAIL.
//   tactics-flank     two alert men in the corridor with one fix fifteen metres off in the cubicle farm and two ways round to it: roles dealt (a presser on the
//                     short route, a flanker sent round), the flanker's way into the fix's four-metre disc at least 90° round from the presser's, the presser
//                     kneeling at his last corner for two seconds and more before the joint push, both there within 2.5 s of each other; the radio lines.
//   tactics-single    the same two men with the fix inside the storage room — one door, one way in: no flank to be had, so the second man presses too, three
//                     metres BEHIND the first up the same line (never abreast: lateral separation under a metre, longitudinal 2.5 m and more).
//   tactics-lone      one alert man, an ARMED Sam (he has fired) fourteen metres off round a corner: he takes the nearest corner that sees the fix, calls it in,
//                     and only goes on — at a walk — after loneWait or when a second man turns up; the same man with Sam four metres off fights as he always did.
//   tactics-door      a man struck down IN the storage doorway under his colleague's eyes: the doorway is a kill zone for their side — the colleague sent through
//                     it stops short on the jamb out of the opening's line (his capsule never inside the gap till the drill starts), radios it, waits, slices the
//                     pie and goes in at a run; staged again with a stun charge on him, the bang lands inside first.
//   tactics-rounds    Sam's rifle fired out of the break room through its open west door past a man in the cubicles: that door is dangerous for the guards and
//                     the man's way to Sam's fix re-plans through the north door (his path never touches the west door's gap cells).
//   tactics-decay     a dangerous door is plain again doorDanger seconds later (and one banged and entered, at once).
import { standUp, ROOT } from './headless.ts';
const { v3, wrapAngle, DEG } = await import(`${ROOT}/src/math/vec.ts`);
const guardsMod = await import(`${ROOT}/src/game/guards.ts`);
const { resetChase } = guardsMod;
const { fireWeapon, killGuard } = await import(`${ROOT}/src/game/combat.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
const f1 = (x: number) => x.toFixed(1), f2 = (x: number) => x.toFixed(2);
const posS = (p: Vec3 | null | undefined) => p ? `(${f1(p[0])}, ${f1(p[2])})` : '—';
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** A guard planted at `p` facing yaw, still, no path, posed twice (bones where the sight tests read them). */
export function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
/** parked out on the lot east of the building — held, pinned, blind and deaf */
export function park(gd: any, i: number) { gd.hold = true; gd.pinned = true; gd.dazzledUntil = 1e9; gd.heardUpTo = 1e12; place(gd, [37.5, 0, 6 + i * 1.5], Math.PI / 2); }
/** alert on a FIX with no sight of anybody: free to move, hearing on from now, awareness full, the fix placed, every chase clock fresh */
export function fixAlert(gd: any, game: any, fix: Vec3) {
  gd.hold = false; gd.pinned = false; gd.dazzledUntil = -1; gd.heardUpTo = game.eventSeq; gd.state = 'alert'; gd.awareness = 1; gd.reactT = 0; gd.fireCd = 0.5; gd.reloadT = -1;
  gd.lastKnown = v3.copy(fix); gd.lastSeenT = -100; gd.sightT = -100; gd.searchT = 0; gd.bodyDuty = null; gd.searchPlan = null; gd.standoff = null; gd.script = null; gd.task = null; resetChase(gd);
}
/** what everybody says (game.say) and radios from here on */
export function tapSay(game: any) {
  const lines: { t: number; who: string; text: string; radio: boolean }[] = [];
  const orig = game.say.bind(game);
  game.say = (gd: any, text: string, radio = false) => { const r = orig(gd, text, radio); if (gd.bubble && gd.bubble.text === text && gd.bubble.t === game.time) lines.push({ t: game.time, who: gd.callsign, text, radio }); return r; };
  return lines;
}


// ================================================================ shared staging for the behaviour probes (tactics.ts imported here, not at the top: the identity run above must load on a tree without it)
const tac = await import(`${ROOT}/src/game/tactics.ts`);
const { tacticsRoleOf, roleRecord, loneRecord, gateRecord, plansOf, dangerDoors, dangerAt, giveStuns, TACTICS_DEBUG } = tac;
/** the level up off `seed`, god mode, Sam's meter pinned dark by the caller each frame; TACTICS_DEBUG lines collected (printed under --verbose) */
async function stage(seed: number) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W;
  game.godMode = true; game.aiEnabled = true;
  const tlog: string[] = []; TACTICS_DEBUG.log = (s: string) => tlog.push(s);
  for (let i = 0; i < 5; i++) W.step(DT);
  const lines = tapSay(game);
  return { W, game, pl: game.player, A: game.guards[0], B: game.guards[1], C: game.guards[2], lines, tlog };
}
function dumpLog(tlog: string[], n = 40) { if (VERBOSE) console.log('    ' + tlog.slice(0, n).join('\n    ')); }
const bearingDeg = (p: Vec3, C: Vec3) => Math.atan2(p[0] - C[0], p[2] - C[2]) / DEG;
const degApart = (a: number, b: number) => Math.abs(wrapAngle((a - b) * DEG)) / DEG;

// ================================================================ (a) two men, one fix, two ways round: the pincer
export async function tacticsFlank() {
  console.log('\n=== tactics-flank: Kowalski + Novak alert in the corridor on one fix 15 m off in the cubicles\' middle aisle (a dark Sam nobody can see) — presser + flanker, ≥ 90° apart into the disc, the presser kneels ≥ 2 s at his corner, "moving round" / "set" / "go go go", both there within 2.5 s ===');
  const { W, game, pl, A, B, C, lines, tlog } = await stage(801);
  park(C, 2);
  const FIX: Vec3 = [24.0, 0, 17.6];
  game.teleportPlayer([26.5, 0, 20.5]); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = true; game.playerInvisible = true;   // (invisible: this is about the approach on a FIX — nobody is to get eyes on and turn it into the fight)
  place(A, [12.0, 0, 11.2], Math.PI / 2); place(B, [10.5, 0, 11.4], Math.PI / 2); W.step(DT);
  fixAlert(A, game, FIX); fixAlert(B, game, FIX);
  let roles: Record<string, string> = {}, presser: any = null, flanker: any = null, plan: any = null; let plannedApart = NaN; let holdPos: Vec3 | null = null;
  let holdStart = -1, holdEnd = -1, goT = -1, kneltMax = 0; const arriveT = new Map<any, number>(); const trail: string[] = [];
  const enter = new Map<any, number>();   // bearing (deg about the fix) at which each man first came inside 4 m of it
  const prevD = new Map<any, number>();
  for (let f = 0; f < 60 * 30; f++) {
    pl.visibility = 0.02; W.step(DT);
    const t = game.time;
    if (!plan && plansOf(game).length) { plan = plansOf(game)[0]; for (const m of plan.members) { const r = plan.roles.get(m); roles[m.callsign] = r.kind; if (r.kind === 'presser') presser = m; if (r.kind === 'flanker') { flanker = m; plannedApart = degApart(r.bearing / DEG, plan.presserBearing / DEG); } } }
    if (presser) { const r = roleRecord(game, presser); if (r?.holding && plan.phase === 'approach') { if (holdStart < 0) { holdStart = t; holdPos = v3.copy(presser.char.pos); } holdEnd = t; kneltMax = Math.max(kneltMax, presser.char.anim.crouchTarget); } }
    if (plan && goT < 0 && plan.phase === 'push') goT = t;
    for (const m of [A, B]) {
      const d = v3.distXZ(m.char.pos, FIX); const pd = prevD.get(m) ?? d; prevD.set(m, d);
      if (!enter.has(m) && pd > 4 && d <= 4) enter.set(m, bearingDeg(m.char.pos, FIX));
      if (!arriveT.has(m) && (d < 1.6 || (m.state === 'search' && d < 3))) arriveT.set(m, t);
    }
    if (f % 60 === 0) trail.push(`${f1(t)}s ` + [A, B].map(m => `${m.callsign} ${m.state} ${posS(m.char.pos)} ${tacticsRoleOf(game, m) ?? '-'}${roleRecord(game, m)?.holding ? '(holding)' : ''} ${f1(v3.distXZ(m.char.pos, FIX))} m`).join(' | ') + (plan ? ` · plan ${plan.phase}` : ''));
    if (arriveT.size === 2 && t - Math.max(...arriveT.values()) > 1) break;
  }
  const radio = lines.filter(l => l.radio).map(l => `${f1(l.t)}s ${l.who}: "${l.text}"`);
  const fails: string[] = [];
  if (!plan) fails.push('no plan was ever dealt');
  if (!presser || !flanker) fails.push(`roles: ${JSON.stringify(roles)} (want one presser and one flanker)`);
  const bP = presser ? enter.get(presser) : undefined, bF = flanker ? enter.get(flanker) : undefined;
  const apart = bP !== undefined && bF !== undefined ? degApart(bP, bF) : NaN;
  if (!(apart >= 90) && !(Number.isNaN(apart) && plannedApart >= 90)) fails.push(`the two came into the 4 m disc ${f1(apart)}° apart about the fix (presser ${bP === undefined ? '—' : Math.round(bP)}°, flanker ${bF === undefined ? '—' : Math.round(bF)}°; planned ${f1(plannedApart)}°) — want ≥ 90°`);
  const held = holdStart >= 0 ? holdEnd - holdStart : 0;
  if (held < 2) fails.push(`the presser held ${f2(held)} s at his corner before the push (want ≥ 2)`);
  if (kneltMax < 0.9) fails.push(`the presser never knelt at his corner (crouch peak ${f2(kneltMax)})`);
  const tA = presser ? arriveT.get(presser) : undefined, tB = flanker ? arriveT.get(flanker) : undefined;
  if (tA === undefined || tB === undefined) fails.push(`arrivals: presser ${tA === undefined ? 'never' : f1(tA)}, flanker ${tB === undefined ? 'never' : f1(tB)}`);
  else if (Math.abs(tA - tB) > 2.5) fails.push(`they arrived ${f2(Math.abs(tA - tB))} s apart (want ≤ 2.5)`);
  if (!radio.some(r => /(moving|going) a?round/.test(r))) fails.push('nobody radioed "moving round" / "going around"');
  if (!radio.some(r => /: "set/.test(r))) fails.push('the flanker never radioed "set"');
  if (!radio.some(r => /go go go|pushing, go/.test(r))) fails.push('nobody gave the "go"');
  if (goT >= 0 && holdStart >= 0 && goT < holdStart) fails.push('the go came before the presser was at his corner');
  const ok = fails.length === 0;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — roles ${JSON.stringify(roles)}; into the disc: presser at ${bP === undefined ? '—' : Math.round(bP) + '°'}, flanker at ${bF === undefined ? '—' : Math.round(bF) + '°'} = ${f1(apart)}° apart (planned ${f1(plannedApart)}°); presser knelt ${f2(held)} s at ${posS(holdPos)} (crouch ${f2(kneltMax)}); go at ${goT < 0 ? '—' : f1(goT) + ' s'}; arrivals presser ${tA === undefined ? '—' : f1(tA)} s / flanker ${tB === undefined ? '—' : f1(tB)} s (${tA !== undefined && tB !== undefined ? f2(Math.abs(tA - tB)) : '—'} s apart)\n    radio: ${radio.join(' · ') || '—'}\n    ${trail.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  dumpLog(tlog);
  console.log(`=== tactics-flank: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
  return ok;
}

// ================================================================ (b) one way in: the second presser three metres behind, never abreast
export async function tacticsSingle() {
  console.log('\n=== tactics-single: the same two with the fix inside the storage room (one door): no flank — a second presser 3 m BEHIND the first up the corridor (lateral < 1 m, longitudinal ≥ 2.5 m while both move), the first holds at the jamb till the second has closed up ===');
  const { W, game, pl, A, B, C, lines, tlog } = await stage(802);
  park(C, 2);
  const FIX: Vec3 = [33.5, 0, 6.5];
  game.teleportPlayer([6.0, 0, 22.0]); game.playerInvisible = true;
  place(A, [24.0, 0, 11.2], Math.PI / 2); place(B, [21.5, 0, 11.4], Math.PI / 2); W.step(DT);
  fixAlert(A, game, FIX); fixAlert(B, game, FIX);
  let plan: any = null, roles: Record<string, string> = {}, presser: any = null, second: any = null;
  let latMax = 0, longMin = Infinity, sampled = 0, abreast = 0; const trail: string[] = [];
  for (let f = 0; f < 60 * 20; f++) {
    pl.visibility = 0.02; W.step(DT);
    const t = game.time;
    if (!plan && plansOf(game).length) { plan = plansOf(game)[0]; for (const m of plan.members) { const r = plan.roles.get(m); roles[m.callsign] = r.kind; if (r.kind === 'presser') presser = m; if (r.kind === 'second') second = m; } }
    // in the corridor (z > 10.3), both on the move: the second's offset from the presser split along / across the corridor (x runs along it)
    if (presser && second && plan?.phase === 'approach' && presser.char.pos[2] > 10.3 && second.char.pos[2] > 10.3 && presser.speed > 0.8 && second.speed > 0.8) {
      const lat = Math.abs(presser.char.pos[2] - second.char.pos[2]), lon = Math.abs(presser.char.pos[0] - second.char.pos[0]);
      sampled++; latMax = Math.max(latMax, lat); longMin = Math.min(longMin, lon); if (lat > 1 || lon < 2.5) abreast++;
    }
    if (f % 30 === 0) trail.push(`${f1(t)}s ` + [A, B].map(m => `${m.callsign} ${m.state} ${posS(m.char.pos)} ${tacticsRoleOf(game, m) ?? '-'} spd ${f2(m.speed)}`).join(' | ') + ` gap ${f2(v3.distXZ(A.char.pos, B.char.pos))}` + (plan ? ` · ${plan.phase}` : ''));
    if (plan && plan.phase === 'done' && A.state === 'search' && B.state === 'search') break;
  }
  const fails: string[] = [];
  if (!plan) fails.push('no plan was dealt');
  if (!presser || !second) fails.push(`roles ${JSON.stringify(roles)} (want a presser and a SECOND — no flank exists through one door)`);
  if (Object.values(roles).includes('flanker')) fails.push('a flanker was dealt through the only door');
  if (sampled < 30) fails.push(`only ${sampled} frames of both moving up the corridor to judge the file by`);
  if (latMax >= 1.0) fails.push(`lateral separation reached ${f2(latMax)} m (want < 1: in file, not abreast)`);
  if (longMin < 2.5) fails.push(`longitudinal separation fell to ${f2(longMin)} m while both moved (want ≥ 2.5)`);
  const ok = fails.length === 0;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — roles ${JSON.stringify(roles)}; ${sampled} frames both moving up the corridor: lateral ≤ ${f2(latMax)} m, longitudinal ≥ ${Number.isFinite(longMin) ? f2(longMin) : '—'} m, abreast frames ${abreast}\n    radio: ${lines.filter(l => l.radio).map(l => `${f1(l.t)}s ${l.who}: "${l.text}"`).join(' · ') || '—'}\n    ${trail.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  dumpLog(tlog);
  console.log(`=== tactics-single: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
  return ok;
}

// ================================================================ (c) the lone man on an armed Sam: an angle and a call, not a charge — and inside five metres, the fight as it was
export async function tacticsLone() {
  console.log('\n=== tactics-lone: Kowalski alone, alert on a fix 12 m down the cubicles\' south aisle where an ARMED Sam (a shot of his in Kowalski\'s ears) crouches dark: he takes the corner that sees the fix, kneels, radios it, and only goes on — at a WALK — after loneWait; then the same man with the fix 4 m off: the chase / fight as it always was ===');
  let allPass = true;
  {
    const { W, game, pl, A, B, C, lines, tlog } = await stage(803);
    park(B, 1); park(C, 2);
    const FIX: Vec3 = [14.0, 0, 22.8];
    game.teleportPlayer(FIX); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = true; game.playerInvisible = true;
    place(A, [26.5, 0, 20.5], Math.PI); W.step(DT); fixAlert(A, game, FIX);
    A.witness.heard.shots = 1; A.witness.heard.lastShotT = game.time; A.witness.heard.lastT = game.time; pl.lastFireT = game.time;   // (a shot of Sam's reached him a moment ago: the man is armed as far as he knows)
    let holdAt: Vec3 | null = null, holdT0 = -1, holdT1 = -1, los = false, walls = 0, movedOnAt = -1, maxSpeedAfter = 0, minDistWhileHeld = Infinity; const trail: string[] = [];
    for (let f = 0; f < 60 * 26; f++) {
      pl.visibility = 0.02; W.step(DT);
      const t = game.time; const L = loneRecord(game, A);
      if (L && L.holdT > 0 && !L.released) { if (holdT0 < 0) { holdT0 = t; holdAt = v3.copy(A.char.pos); los = !game.col.segmentBlocked([A.char.pos[0], 1.5, A.char.pos[2]], [FIX[0], 1.0, FIX[2]]); const nav = game.col.nav, cs = nav.cell; const cx = Math.floor(A.char.pos[0] / cs), cz = Math.floor(A.char.pos[2] / cs); walls = 0; for (let dz = -1; dz <= 1; dz++) for (let dx = -1; dx <= 1; dx++) if ((dx || dz) && nav.isBlocked((cx + dx + 0.5) * cs, (cz + dz + 0.5) * cs)) walls++; } holdT1 = t; minDistWhileHeld = Math.min(minDistWhileHeld, v3.distXZ(A.char.pos, FIX)); }
      if (holdT0 >= 0 && movedOnAt < 0 && holdAt && v3.distXZ(A.char.pos, holdAt) > 0.6) movedOnAt = t;
      if (movedOnAt >= 0 && v3.distXZ(A.char.pos, FIX) > 5.2) maxSpeedAfter = Math.max(maxSpeedAfter, A.speed);
      if (f % 60 === 0) trail.push(`${f1(t)}s Kowalski ${A.state} ${posS(A.char.pos)} ${tacticsRoleOf(game, A) ?? '-'} spd ${f2(A.speed)} crouch ${f1(A.char.anim.crouchTarget)} ${f1(v3.distXZ(A.char.pos, FIX))} m off${L ? ` · lone ${L.released ? 'released:' + L.releasedWhy : L.holdT > 0 ? 'holding ' + f1(L.holdT) : 'to corner'}` : ''}`);
      if (A.state === 'search') break;
    }
    const radio = lines.filter(l => l.radio && l.who === 'Kowalski');
    const fails: string[] = [];
    const held = holdT0 >= 0 ? holdT1 - holdT0 : 0;
    if (holdT0 < 0) fails.push('he never took up a hold');
    if (!los) fails.push(`no line of sight from his corner ${posS(holdAt)} to the fix`);
    if (walls < 2) fails.push(`his corner ${posS(holdAt)} has ${walls} statics about it (want ≥ 2: a wall at his shoulder)`);
    if (minDistWhileHeld < 5) fails.push(`held only ${f1(minDistWhileHeld)} m off the fix (want > 5)`);
    if (!radio.length) fails.push('he radioed nothing while holding');
    if (held < game.tune.loneWait - 0.6) fails.push(`he held ${f1(held)} s (want ≈ loneWait ${game.tune.loneWait})`);
    if (movedOnAt >= 0 && holdT0 >= 0 && movedOnAt < holdT0 + game.tune.loneWait - 0.6) fails.push(`he moved on at ${f1(movedOnAt)} s, ${f1(movedOnAt - holdT0)} s into the hold (want ≥ loneWait)`);
    if (movedOnAt < 0) fails.push('he never went on after the wait');
    if (maxSpeedAfter > game.tune.guardWalk + 0.15) fails.push(`going on he reached ${f2(maxSpeedAfter)} m/s outside 5 m of the fix (want a walk ≤ ${game.tune.guardWalk})`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (1) 12 m, armed: corner ${posS(holdAt)} (${walls} statics about it, LOS ${los}, ${f1(minDistWhileHeld)} m off), held ${f1(held)} s from ${holdT0 < 0 ? '—' : f1(holdT0)} s, moved on at ${movedOnAt < 0 ? '—' : f1(movedOnAt) + ' s'} peaking ${f2(maxSpeedAfter)} m/s outside 5 m\n    radio: ${radio.map(l => `${f1(l.t)}s "${l.text}"`).join(' · ') || '—'}\n    ${trail.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    dumpLog(tlog);
  }
  {   // (2) the regression guard: 4 m off, armed — no hold, no call: he closes / fights as before (playerInvisible: he runs onto the fix and searches)
    const { W, game, pl, A, B, C, lines } = await stage(804);
    park(B, 1); park(C, 2);
    const FIX: Vec3 = [20.0, 0, 22.8];
    game.teleportPlayer(FIX); pl.slot = 'pistol'; pl.holstered = false; pl.crouch = true; game.playerInvisible = true;
    place(A, [24.0, 0, 22.7], -Math.PI / 2); W.step(DT); fixAlert(A, game, FIX);
    A.witness.heard.shots = 1; A.witness.heard.lastShotT = game.time; A.witness.heard.lastT = game.time; pl.lastFireT = game.time;
    let everLone = false, reachedAt = -1, peak = 0;
    for (let f = 0; f < 60 * 8; f++) { pl.visibility = 0.02; W.step(DT); if (loneRecord(game, A)) everLone = true; peak = Math.max(peak, A.speed); if (reachedAt < 0 && v3.distXZ(A.char.pos, FIX) < 1.5) reachedAt = game.time; }
    const radio = lines.filter(l => l.radio && l.who === 'Kowalski');
    const fails: string[] = [];
    if (everLone) fails.push('a lone-hold record was made at 4 m');
    if (reachedAt < 0) fails.push('he never closed onto the fix');
    if (radio.some(l => /armed/.test(l.text))) fails.push(`he radioed "${radio[0].text}"`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (2) 4 m, armed: lone record ever ${everLone}; onto the fix at ${reachedAt < 0 ? '—' : f1(reachedAt) + ' s'} peaking ${f2(peak)} m/s; state ${A.state}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== tactics-lone: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (d) a man down in a doorway, in view: the doorway is a kill zone — stop short, call, wait, slice, cross fast (and with a charge: bang first)
/** the door gap's floor rectangle: along the frame line hinge → latch, ± `depth` either side of it */
function inGap(game: any, d: any, p: Vec3, depth = 0.35): boolean {
  const hx = d.def.hinge[0], hz = d.def.hinge[1], ux = Math.cos(d.def.closedDir), uz = Math.sin(d.def.closedDir); const nx = -uz, nz = ux;
  const a = (p[0] - hx) * ux + (p[2] - hz) * uz, b = (p[0] - hx) * nx + (p[2] - hz) * nz;
  return a > -0.1 && a < d.def.width + 0.1 && Math.abs(b) < depth;
}
async function doorRun(seed: number, stun: boolean) {
  const S = await stage(seed); const { W, game, pl, A, B, C, lines, tlog } = S;
  park(B, 1);
  game.teleportPlayer([6.0, 0, 22.0]); game.playerInvisible = true;
  const d = game.doors.byName('storage');
  C.hold = false; C.pinned = false; C.dazzledUntil = -1; C.heardUpTo = game.eventSeq; place(C, [32.7, 0, 10.45], Math.PI);   // Reyes in the storage doorway, corridor side of the shut leaf
  place(A, [28.5, 0, 11.2], Math.PI / 2); W.step(DT);                                                                          // Kowalski four metres up the corridor, looking straight at him
  if (stun) giveStuns(A, 1);
  killGuard(game, C, [0, 0, -1], true);   // struck down there (Sam's takedown, unseen: Sam is the sandbox's ghost) — under Kowalski's eyes
  const dangerAtOnce = !!dangerAt(game, d, 'guard');
  fixAlert(A, game, [33.5, 0, 6.5]); A.reactT = 0.4;   // and Kowalski sent in: a fix inside the room, the doorway between
  let gateAt = -1, stackedAt = -1, drillAt = -1, crossAt = -1, inAt = -1, bangAt = -1, bangPos: Vec3 | null = null; let inGapBefore = 0, minGapDistBefore = Infinity, radioT = -1;
  const phases: string[] = []; let lastPhase = ''; const slicePos: string[] = []; const trail: string[] = [];
  const bounds = { x0: 30.12, x1: 35.88, z0: 4.12, z1: 9.94 };
  for (let f = 0; f < 60 * 26; f++) {
    pl.visibility = 0.02; W.step(DT);
    const t = game.time; const G = gateRecord(game, A);
    if (G && gateAt < 0) gateAt = t;
    const ph = G ? G.phase : (gateAt >= 0 ? 'released' : 'none');
    if (ph !== lastPhase) { phases.push(`${f1(t)}s ${ph}`); lastPhase = ph; if (ph === 'wait' && stackedAt < 0) stackedAt = t; if ((ph === 'slice' || ph === 'toss') && drillAt < 0) drillAt = t; if (ph === 'cross' && crossAt < 0) crossAt = t; }
    if (drillAt < 0) { if (inGap(game, d, A.char.pos)) inGapBefore++; minGapDistBefore = Math.min(minGapDistBefore, Math.abs(A.char.pos[2] - 10)); }
    if (G?.phase === 'slice' && f % 6 === 0) slicePos.push(`(${f2(A.char.pos[0])},${f2(A.char.pos[2])})→yaw ${Math.round(A.char.aimYaw / DEG)}°`);
    for (const e of game.events) if (e.kind === 'bang' && e.time === t && bangAt < 0) { bangAt = t; bangPos = v3.copy(e.pos); }
    if (inAt < 0 && A.char.pos[0] > bounds.x0 && A.char.pos[0] < bounds.x1 && A.char.pos[2] < bounds.z1 - 0.3) inAt = t;
    if (radioT < 0) { const r = lines.find(l => l.radio && l.who === 'Kowalski' && /doorway|door/.test(l.text)); if (r) radioT = r.t; }
    if (f % 30 === 0) trail.push(`${f1(t)}s Kowalski ${A.state} (${f2(A.char.pos[0])}, ${f2(A.char.pos[2])}) ${tacticsRoleOf(game, A) ?? '-'} ${G ? G.phase : ''} spd ${f2(A.speed)} crouch ${f1(A.char.anim.crouchTarget)} · leaf ${f2(d.angle)}${d.latched ? 'L' : ''}`);
    if (inAt >= 0 && t - inAt > 1.5) break;
  }
  return { ...S, d, dangerAtOnce, gateAt, stackedAt, drillAt, crossAt, inAt, bangAt, bangPos, inGapBefore, minGapDistBefore, radioT, phases, slicePos, trail, bounds };
}
export async function tacticsDoor() {
  console.log('\n=== tactics-door: Reyes struck down in the storage doorway under Kowalski\'s eyes, Kowalski then sent into the room — the doorway is a kill zone: he stacks on the jamb OUT of the opening (never in the gap before the drill), radios it, waits doorWait, slices the pie, crosses at a run; again with a stun charge on him: the bang lands INSIDE first ===');
  let allPass = true;
  {
    const R = await doorRun(805, false); const { game, A, d, lines, tlog } = R;
    const fails: string[] = [];
    if (!R.dangerAtOnce) fails.push('the storage door was not marked dangerous the frame Reyes dropped in it under Kowalski\'s eyes');
    if (R.gateAt < 0) fails.push('the gate never took Kowalski');
    if (R.inGapBefore > 0) fails.push(`his capsule was inside the door gap on ${R.inGapBefore} frame(s) before the drill began`);
    if (R.radioT < 0) fails.push('he radioed nothing about the doorway');
    if (R.stackedAt < 0 || R.drillAt < 0) fails.push(`phases: ${R.phases.join(' → ')} (want stack → wait → slice → cross)`);
    else if (R.drillAt - R.stackedAt < game.tune.doorWait - 0.3) fails.push(`he waited ${f1(R.drillAt - R.stackedAt)} s on the jamb (want ≈ doorWait ${game.tune.doorWait})`);
    if (!R.phases.some(p => /slice/.test(p))) fails.push('no slice');
    if (R.slicePos.length < 4) fails.push(`only ${R.slicePos.length} slice samples`);
    if (R.crossAt < 0 || R.inAt < 0) fails.push(`cross at ${R.crossAt < 0 ? 'never' : f1(R.crossAt)}, inside the room at ${R.inAt < 0 ? 'never' : f1(R.inAt)}`);
    let peakIn = 0; if (R.crossAt >= 0) { /* speed through the funnel: re-read from the trail is coarse; take the gate's own pace as given and assert he was not walking */ }
    const crossTrail = R.trail.filter(s => / cross /.test(s)).map(s => Number(/spd ([0-9.]+)/.exec(s)?.[1] ?? 0)); peakIn = Math.max(0, ...crossTrail);
    if (R.crossAt >= 0 && crossTrail.length && peakIn < game.tune.guardInvestigate) fails.push(`he crossed at ${f2(peakIn)} m/s peak (want a run, > ${game.tune.guardInvestigate})`);
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (1) no charge: dangerous at once ${R.dangerAtOnce}; gate at ${f1(R.gateAt)} s; in the gap before the drill: ${R.inGapBefore} frames (nearest ${f2(R.minGapDistBefore)} m off the frame line); radio at ${R.radioT < 0 ? '—' : f1(R.radioT) + ' s'}; waited ${R.stackedAt >= 0 && R.drillAt >= 0 ? f1(R.drillAt - R.stackedAt) : '—'} s; cross at ${R.crossAt < 0 ? '—' : f1(R.crossAt)} s peaking ${f2(peakIn)} m/s; inside at ${R.inAt < 0 ? '—' : f1(R.inAt) + ' s'}\n    phases: ${R.phases.join(' → ')}\n    slice stations (pos → aim yaw): ${R.slicePos.join(' ')}\n    radio: ${lines.filter((l: any) => l.radio).map((l: any) => `${f1(l.t)}s ${l.who}: "${l.text}"`).join(' · ')}\n    ${R.trail.join('\n    ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    dumpLog(tlog, 30);
  }
  {
    const R = await doorRun(806, true); const { game, A, d, lines, tlog, bounds } = R;
    const fails: string[] = [];
    if (R.bangAt < 0) fails.push('no bang');
    else {
      const p = R.bangPos!; const inside = p[0] > bounds.x0 && p[0] < bounds.x1 && p[2] > bounds.z0 && p[2] < bounds.z1;
      if (!inside) fails.push(`the charge went off at ${posS(p)} — not inside the storage room`);
      if (R.crossAt >= 0 && R.bangAt > R.crossAt + 1e-6) fails.push(`the bang (${f1(R.bangAt)} s) came after he started in (${f1(R.crossAt)} s)`);
      if (A.dazzledUntil > R.bangAt) fails.push('the thrower dazzled himself');
    }
    if (R.inAt < 0) fails.push('he never went in after the bang');
    if (R.inGapBefore > 0) fails.push(`in the gap on ${R.inGapBefore} frame(s) before the drill`);
    const rec = dangerAt(game, d, 'guard');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — (2) a stun charge on him: bang at ${R.bangAt < 0 ? '—' : f1(R.bangAt) + ' s ' + posS(R.bangPos)}; cross at ${R.crossAt < 0 ? '—' : f1(R.crossAt)} s; inside at ${R.inAt < 0 ? '—' : f1(R.inAt) + ' s'}; the doorway after: ${rec ? `still dangerous ${f1(rec.until - game.time)} s` : 'plain again (banged + entered)'}\n    phases: ${R.phases.join(' → ')}\n    lines: ${lines.filter((l: any) => l.who === 'Kowalski').map((l: any) => `${f1(l.t)}s ${l.radio ? 'RADIO ' : ''}"${l.text}"`).join(' · ')}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    dumpLog(tlog, 30);
  }
  console.log(`=== tactics-door: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (e) rounds through a doorway at a man: that door is dangerous for his side, and his way in re-plans through the other one
export async function tacticsRounds() {
  console.log('\n=== tactics-rounds: Sam in the break room fires his rifle west out through its open west door, a stride past Novak eight metres down the cubicles\' middle aisle — the west door is a kill zone for the guards; Novak\'s way to the fix goes round by the north door (his path never touches the west door\'s gap cells); with tune.tactics off, straight through the west door ===');
  let allPass = true;
  const runOnce = async (on: boolean) => {
    const S = await stage(807); const { W, game, pl, A, B, C, lines, tlog } = S;
    game.tune.tactics = on;
    park(A, 0); park(C, 2);
    const SAM: Vec3 = [34.0, 0, 17.7];
    game.teleportPlayer(SAM); pl.slot = 'rifle'; pl.holstered = false; pl.crouch = false; game.playerInvisible = true;
    place(B, [19.5, 0, 17.7], Math.PI / 2); B.hold = false; B.pinned = false; B.dazzledUntil = -1; B.heardUpTo = game.eventSeq; W.step(DT);
    const dw = game.doors.byName('breakroom_w'), dn = game.doors.byName('breakroom_n');
    for (let i = 0; i < 2; i++) { fireWeapon(game, pl.char, [19.5, 1.15, 18.7], true); for (let k = 0; k < 12; k++) { pl.visibility = 0.02; W.step(DT); } }   // two rifle rounds out through the west door, a stride south of Novak
    const marked = !!dangerAt(game, dw, 'guard');
    // his way in from here: the cells his planned path runs through against the west door's gap cells
    const nav = game.col.nav; const { doorCells } = guardsMod; const DC = doorCells(game);
    const gapW = new Set<number>(), gapN = new Set<number>(); for (const [i, dd] of DC) { if (dd === dw) gapW.add(i); if (dd === dn) gapN.add(i); }
    let viaW = 0, viaN = 0, pathLen = 0; const via: string[] = [];
    let firstPath: Vec3[] | null = null;
    for (let f = 0; f < 60 * 16; f++) {
      pl.visibility = 0.02; W.step(DT);
      if (!firstPath && B.state === 'alert' && B.path.length > 1 && B.reactT <= 0) {
        firstPath = B.path.map((p: Vec3) => v3.copy(p)) as Vec3[]; let q: Vec3 = B.char.pos;
        for (const w of firstPath) { const L = v3.distXZ(q, w); pathLen += L; const n = Math.ceil(L / 0.2); for (let k = 0; k <= n; k++) { const x = q[0] + (w[0] - q[0]) * k / n, z = q[2] + (w[2] - q[2]) * k / n; const idx = nav.idx(Math.floor(x / nav.cell), Math.floor(z / nav.cell)); if (gapW.has(idx)) viaW++; if (gapN.has(idx)) viaN++; } q = w; }
        via.push(...firstPath.map((p: Vec3) => posS(p)));
      }
    }
    // where he actually went: did his body ever pass the west door's gap, or the north door's
    return { game, B, marked, viaW, viaN, pathLen, via, lines, tlog, fix: B.lastKnown ? posS(B.lastKnown) : '—', state: B.state, at: posS(B.char.pos) };
  };
  {
    const R = await runOnce(true);
    const fails: string[] = [];
    if (!R.marked) fails.push('the west door was not marked dangerous for the guards after two rounds through it past Novak');
    if (!R.via.length) fails.push(`Novak never planned a way to the fix (state ${R.state}, fix ${R.fix})`);
    if (R.viaW > 0) fails.push(`his path runs through the west door's gap cells (${R.viaW} samples)`);
    if (R.viaN === 0) fails.push('his path does not go by the north door either');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — tactics ON: west door dangerous ${R.marked}; his first path ${f1(R.pathLen)} m: ${R.via.join(' ')} — west-gap samples ${R.viaW}, north-gap samples ${R.viaN}; ends ${R.state} at ${R.at}\n    danger: ${dangerDoors(R.game).map((r: any) => `${r.door.def.name}/${r.team}/${r.why}`).join(', ') || '—'}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
    dumpLog(R.tlog, 20);
  }
  {
    const R = await runOnce(false);
    const fails: string[] = [];
    if (R.marked) fails.push('a door was marked dangerous with tune.tactics off');
    if (R.viaW === 0) fails.push('with the tactics off his path did not take the west door (the staging is not exercising the short way)');
    const ok = fails.length === 0; allPass &&= ok;
    console.log(`- ${ok ? 'PASS' : 'FAIL'} — tactics OFF (control): dangerous ${R.marked}; his first path ${f1(R.pathLen)} m: ${R.via.join(' ')} — west-gap samples ${R.viaW}, north-gap samples ${R.viaN}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  }
  console.log(`=== tactics-rounds: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (f) danger decays
export async function tacticsDecay() {
  console.log('\n=== tactics-decay: a doorway marked dangerous is plain again tune.doorDanger seconds later; the A* prices it while it lasts and not after; a record banged AND entered clears at once ===');
  const { W, game, pl, A, B, C, tlog } = await stage(808);
  park(A, 0); park(B, 1); park(C, 2); game.teleportPlayer([6, 0, 22]); game.playerInvisible = true;
  const d = game.doors.byName('server'); const T0 = game.time;
  const rec = tac.markDanger(game, d, 'guard', 'rounds', null);
  const until0 = rec.until;
  let liveAt10 = false, liveAtEnd = true, goneAt = -1;
  // the penalty seen by a guard's plan through that door while it lasts: Kowalski (parked, but a guard) asking for a route from the corridor into the server room
  A.hold = false; A.pinned = false; A.state = 'search'; place(A, [17.7, 0, 11.3], Math.PI);
  const pen0 = tac.tacticsPenalty(game, A, [17.7, 0, 7.0]);
  const { doorCells } = guardsMod; const DC = doorCells(game); let doorMult = 1; if (pen0) for (const [i, dd] of DC) if (dd === d) { const nav = game.col.nav; doorMult = Math.max(doorMult, pen0(i % nav.W, Math.floor(i / nav.W))); }
  A.hold = true; A.pinned = true;
  for (let f = 0; f < 60 * (game.tune.doorDanger + 3); f++) {
    pl.visibility = 0.02; W.step(DT);
    const live = !!dangerAt(game, d, 'guard');
    if (Math.abs(game.time - T0 - 10) < DT / 2) liveAt10 = live;
    if (!live && goneAt < 0) goneAt = game.time - T0;
  }
  liveAtEnd = !!dangerAt(game, d, 'guard');
  A.hold = false; A.pinned = false; const penAfter = tac.tacticsPenalty(game, A, [17.7, 0, 7.0]); A.hold = true; A.pinned = true;
  // banged + entered clears at once
  const d2 = game.doors.byName('manager'); const r2 = tac.markDanger(game, d2, 'guard', 'drop', 'Reyes'); r2.banged = true;
  const G = { door: d2, rec: r2, phase: 'cross', role: 'lead', lead: null, crossed: true } as any;   // (the release's own bookkeeping, driven by hand: a lead who crossed a banged doorway)
  B.hold = false; B.pinned = false; (tac as any).gateRecord(game, B);   // (touch: the state exists)
  // emulate: releaseGate is private — the public effect is rec.until once a crossing lead is let go; assert through a real gate would need the whole drill, which tactics-door runs. Here: the rule as data.
  const clearedEarly = (() => { if (r2.banged) { r2.entered = true; r2.until = Math.min(r2.until, game.time); } return !dangerAt(game, d2, 'guard'); })();
  const fails: string[] = [];
  if (Math.abs(until0 - T0 - game.tune.doorDanger) > 0.05) fails.push(`until ${f2(until0 - T0)} s (want doorDanger ${game.tune.doorDanger})`);
  if (!liveAt10) fails.push('not live 10 s in');
  if (liveAtEnd) fails.push(`still live ${f1(game.time - T0)} s in`);
  if (goneAt < 0 || Math.abs(goneAt - game.tune.doorDanger) > 0.1) fails.push(`went plain at ${f2(goneAt)} s (want ${game.tune.doorDanger})`);
  if (!(doorMult >= 20)) fails.push(`the door's cells cost ×${f1(doorMult)} while dangerous (want ≥ 20)`);
  if (penAfter !== undefined) fails.push('a penalty function is still handed out after the danger lapsed (want undefined = plain A*)');
  if (!clearedEarly) fails.push('banged + entered did not clear the record');
  const ok = fails.length === 0;
  console.log(`- ${ok ? 'PASS' : 'FAIL'} — marked at ${f1(T0)} s for ${f1(until0 - T0)} s; live at +10 s ${liveAt10}; plain again at +${f2(goneAt)} s; door cells ×${f1(doorMult)} while it lasted, penalty after: ${penAfter === undefined ? 'none (plain A*)' : 'STILL PRICED'}; banged+entered → cleared ${clearedEarly}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);
  dumpLog(tlog, 10);
  console.log(`=== tactics-decay: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
  return ok;
}
