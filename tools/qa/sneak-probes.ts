// The 'sneak' family (game/sneak.ts — how ONE teammate follows while the squad is silent and nobody has noticed it). Headless: the whole Game over the fake
// device, the room clear on the office with the PAIR (Sam and Blue One), Sam on his own feet and keys (tools/qa/pairsneak.ts's scripted player and watcher).
//
//   sneak-low      Sam crouched and creeping: the partner is low for the walk, never over SNEAK.lowPace (nor silent's pace — he never sprints); Sam upright:
//                  he is upright. His goggles are down in the dark and his weapon light is out, by the rules those already had (nvg.ts, silent.ts).
//   sneak-steps    he walks Sam's own steps: the furthest his feet are from Sam's trail, down the dark side of a held room
//   sneak-waits    a defender LOOKING at a lit doorway Sam has slipped through: the partner stops short of it, low, where that man has no lit line to him, and
//                  says so over the net (and again after a few seconds, with what would bring him on); the man's eyes never have him; the man turned away,
//                  he crosses and is back at Sam's shoulder.
//   sneak-light    the same wait, ended the other way: the light put out, he comes on with the man still looking
//   sneak-fallin   told to hold in the corridor and then to fall in with Sam already through: he comes by the steps Sam took, and waits on the same eyes
//   sneak-stand    Sam stopped a stride inside a doorway whose light a defender has a line to: the partner's place is NOT in that light
//   sneak-aside    Sam at an open door at the end of a corridor that runs straight at it (every step of the trail is in the doorway's line): the partner steps
//   sneak-door     Sam at a SHUT door with a man beyond it: the partner is out of the doorway's line before the leaf moves, and still when it does
//   sneak-post     sent to a mark (a move / cover, the rescue flow's cover) while Sam is low: he goes there low at the low pace; a stack's mark is not touched
//                  ASIDE, off the trail, out of that line — and the man inside who turns to the door does not have him
//   sneak-scope    the sneak is the silent, unnoticed squad's: with a defender hunting, the follower is the build's own (upright) — and a man
//                  told to HOLD is never touched
//   sneak-off      the switch off (SNEAK.on is a run-time row: down wherever a quiet mission has not raised it): the numbers moved about change nothing (the
//                  same scripted walk digests the same), no line is said, nothing is counted
// The halves of the sneak (crouch, post, trail, look, stand, side, door …) are settled on, and so are the two later rules an "off" arm used to hold off with
// the switch (unseen.ts's UNSEEN.on and doorway.ts's DOORMANNERS.on): nothing here writes any of them. Until 2026-11-07 most probes also played an arm "off:
// the build before" (before all three rules): upright whatever Sam does, no more walk-up-and-stop than before, no further off the trail than before, walking
// into the light, waiting in the door's line, the posted walk upright, a loud squad upright beside a low Sam (that one is unseen.ts's on the page), and
// sneak-door's arm with SNEAK.door off. Those lines asked a build from before settled rules: out by ruling.
// The men's gaze is written as their yaw every frame (`stare`), and the probes that ask of the door at the corridor's end start in the corridor (`putInCorridor`).
import { ROOT } from './headless.ts';
import { Run, squadUp } from './pairsneak.ts';
const SNM: any = await import(`${ROOT}/src/game/sneak.ts`);
const SLM: any = await import(`${ROOT}/src/game/silent.ts`);
const NVM: any = await import(`${ROOT}/src/game/nvg.ts`);
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
const DT = 1 / 60;
const SNEAK: any = SNM.SNEAK, SILENT: any = SLM.SILENT;

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const clone = (o: any) => JSON.parse(JSON.stringify(o));
const DEFAULTS = { SNEAK: clone(SNEAK), SILENT: clone(SILENT) };
const reset = () => { Object.assign(SNEAK, clone(DEFAULTS.SNEAK)); Object.assign(SILENT, clone(DEFAULTS.SILENT)); };   // (the tables as the import found them: the page's values, and the run-time rows a run raised put back down)
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
const at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;

/** the pair stood up on the office's room clear (placement 10: two men in the break room, three behind locked doors), squadded up and silent */
async function rig(dice: number, sneak: boolean, set: Record<string, unknown> = {}): Promise<Run> {
  reset(); const R = await Run.start('roomclear=10&squad=1', dice, { sneak, lean: sneak, set: Object.keys(set).length ? { SNEAK: set } : undefined });
  R.stopOnNotice = false; squadUp(R); if (sneak) stareEachFrame(R); return R;
}
/** the gazes written every frame (see `stare`), on a lean run */
const stareEachFrame = (R: Run) => { R.beforeFrame = () => { for (const h of R.hostiles) { const t = gaze.get(h.char); if (!t || h.state === 'dead') continue; const y = Math.atan2(t[0] - h.char.pos[0], t[2] - h.char.pos[2]); h.char.aimYaw = h.char.bodyYaw = y; if ('headYaw' in h.char) h.char.headYaw = y; } }; };
/** from now on, what the partner is seen for: the one tally of the watcher's that these probes read, asked of the two men in the break room alone (the other
 *  three are behind locked doors). Returns the tally as it stands, to read a difference from. */
const watchEyes = (R: Run): number => {
  R.watch = () => { const m = R.mate; if (!m) return;
    for (const h of R.hostiles.slice(0, 2)) { if (h.state === 'dead') continue; const v = GDM.canSeeChar(R.g, h, m.char, false); if (v?.visible && GDM.sightGainOn(R.g, h, m, v.dist) > 0.05) R.mateSeenS += DT; } };
  return R.mateSeenS; };
/** what a walk made of the partner: seconds moving low / upright, his top speed, the most his feet were off Sam's trail, his pace against Sam's while both moved */
function walkStats(R: Run, fn: () => void) {
  const m = R.mate, trail: number[][] = []; let low = 0, up = 0, top = 0, off = 0, stops = 0, wasMoving = false, lightOn = 0, gogglesDown = 0, n = 0;
  const step0 = R.step.bind(R);
  (R as any).step = (k = 1) => { for (let i = 0; i < k; i++) { step0(1); n++;
    const sp = Math.hypot(m.char.vel[0], m.char.vel[2]); top = Math.max(top, sp);
    if (!trail.length || flat(trail[trail.length - 1], R.pos) > 0.1) trail.push([R.pos[0], 0, R.pos[2]]);
    const moving = sp > 0.3; if (moving) { if (m.char.anim.crouch > 0.5) low += 1 / 60; else up += 1 / 60; let d = 1e9; for (const q of trail) d = Math.min(d, flat(q, m.char.pos)); if (trail.length > 20) off = Math.max(off, d); }
    if (wasMoving && !moving && R.g.orders.samV > 0.5) stops++; wasMoving = moving;
    if (m.flashlight?.enabled) lightOn++; if (NVM.nvgDown(R.g, m)) gogglesDown++; } };
  try { fn(); } finally { (R as any).step = step0; }
  return { low, up, top, off, stops, lightOn: lightOn / 60, goggles: n ? gogglesDown / n : 0 };
}
/** Sam and the partner put a few strides short of `to` in the corridor, in place of the walk there from the lobby (a quarter of what the probes that ask about
 *  the door at its end cost): the trail the partner follows starts here */
const putInCorridor = (R: Run, x: number) => { R.g.teleportPlayer(R.free([x, 0, 11.2])); const c = R.mate.char, p = R.free([x - 1.2, 0, 11.2]); c.pos[0] = p[0]; c.pos[2] = p[2]; c.vel[0] = c.vel[2] = 0; if (R.mate.script) R.mate.script.path = undefined; R.step(2); };
const DARK_WALK = (R: Run) => { R.go(R.free([24.6, 0, 11.2]), 60); };   // (the corridor east, from the lobby: twenty metres with nobody's eyes on it)

export const sneakLow = guarded(async () => {
  const F = family('sneak-low: low when Sam is low, at his pace; upright when he is; never over a walk');
  const runs: Record<string, ReturnType<typeof walkStats>> = {};
  for (const [name, crouch] of [['on, Sam low', true], ['on, Sam upright', false]] as const) {
    const R = await rig(3, true); R.crouch(crouch); runs[name] = walkStats(R, () => DARK_WALK(R)); }
  const a = runs['on, Sam low'], b = runs['on, Sam upright'];
  const words = (s: ReturnType<typeof walkStats>) => `low ${f2(s.low)} s, upright ${f2(s.up)} s, top ${f2(s.top)} m/s, stopped ${s.stops}× while Sam walked, light on ${f2(s.lightOn)} s, goggles down ${Math.round(100 * s.goggles)} % of the time`;
  F.verdict(a.low > 5 && a.up < 0.5, 'Sam crouched: the partner makes the walk low', words(a));
  F.verdict(a.top <= SNEAK.lowPace + 0.06, `… and never over the low pace (${SNEAK.lowPace} m/s)`, `top ${f2(a.top)} m/s`);
  F.verdict(b.up > 5 && b.low < 0.5 && b.top <= SILENT.pace + 0.06, `Sam upright: the partner is upright, never over silent's walk (${SILENT.pace} m/s)`, words(b));
  F.verdict(a.lightOn === 0 && b.lightOn === 0, 'his weapon light stays out (silent.ts)', `on for ${f2(a.lightOn)} s / ${f2(b.lightOn)} s`);
  F.verdict(!NVM.NVG.on || a.goggles > 0.5, "his goggles are down in the dark by nvg.ts's own rule", `down ${Math.round(100 * a.goggles)} % of the low walk (NVG.on ${NVM.NVG.on})`);
  return F.done();
});

/** into the break room behind its two men, down the dark east side */
const EAST_SIDE = (R: Run) => { R.crouch(true); R.go(R.free([24.6, 0, 11.2]), 60); R.gather(); R.toDoor('breakroom_n'); R.ocp('panel_corridor_20'); R.gather(); R.ease('breakroom_n'); R.holster(true); R.through('breakroom_n', 1.0);
  for (const p of [[34.3, 0, 13.6], [34.3, 0, 17.0], [34.3, 0, 21.6]]) R.go(R.free(p), 30); };
/** a defender's GAZE pinned on a point: his yaw written every frame before the step, as the spotted family's staging writes it (which is all the eye test and the
 *  partner's read take of where he looks — the men's own idle glancing is dice; a probe that says 'looking at the door' has to mean it). His feet, his senses and
 *  his awareness stay his own. (Until 2026-11-07 this replaced `chestDir` and `headDir` on the copy, which the Rust game neither reads nor is sent: no man's
 *  eyes moved.) */
const gaze = new WeakMap<object, number[]>();
const stare = (_R: Run, h: any, to: number[]) => { if (process.env.STARE_BREAK !== '1') gaze.set(h.char, [to[0], 0, to[2]]); };
/** wait `secs` and say how far off `to` the men's bodies stood at the worst, in radians, the first second, which is the turn, aside: a wait 'under eyes' is one the
 *  men really looked through. `STARE_BREAK=1` pins nobody's gaze: the staged lines of sneak-waits must fail */
function waitUnder(R: Run, secs: number, to: (h: any) => number[]): number {
  let worst = 0; for (let i = 0; i < Math.round(secs * 60); i++) { R.step(); if (i < 60) continue;
    for (const h of R.hostiles.slice(0, 2)) { if (h.state === 'dead') continue; const t = to(h), e = Math.atan2(t[0] - h.char.pos[0], t[2] - h.char.pos[2]) - h.char.bodyYaw; worst = Math.max(worst, Math.abs(Math.atan2(Math.sin(e), Math.cos(e)))); } }
  return worst; }

export const sneakSteps = guarded(async () => {
  const F = family("sneak-steps: he walks Sam's own steps");
  const R = await rig(5, true); for (const h of R.hostiles) stare(R, h, [h.char.pos[0] - 6, 0, h.char.pos[2] + 1]);   // (both men looking west, away from the east side: nobody waits on anybody)
  const off = walkStats(R, () => EAST_SIDE(R)).off;
  F.verdict(off <= 0.4, "his feet stay within 0.4 m of Sam's trail", `furthest off the trail: ${f2(off)} m`);
  return F.done();
});

/** the lit doorway: the corridor panel left on; the near man stood square to the door STARE_M inside it (a line through it to the lit corridor), looking AWAY
 *  while Sam eases it and slips through, then turned to stare at it — with the partner still on the corridor's side */
async function doorway(dice: number, sneak: boolean, set: Record<string, unknown> = {}, closer = false) {
  const R = await rig(dice, sneak, set), T1 = R.hostiles[0], d = R.door('breakroom_n'), fc = d.frameCentre;
  if (!closer) d.def.closer = false;   // (the leaf stays where Sam eased it: the read of the doorway is the man's line, not the closer's timing — sneak-fallin leaves it on)
  const spot = R.free([fc[0], 0, fc[2] + STARE_M]); T1.char.pos[0] = spot[0]; T1.char.pos[2] = spot[2];
  const look = (to: (h: any) => number[]) => { for (const h of R.hostiles) stare(R, h, to(h)); };
  look(h => [h.char.pos[0] + 1, 0, h.char.pos[2] + 8]);
  R.crouch(true); putInCorridor(R, 21); R.go(R.free([24.6, 0, 11.2]), 60); R.gather(); R.toDoor('breakroom_n'); R.gather();
  R.ease('breakroom_n'); R.holster(true); R.through('breakroom_n', 1.0);
  look(() => [fc[0], 0, fc[2]]); const seen0 = watchEyes(R), mateAt = [...R.mate.char.pos], cues0 = SNM.SNEAK_STATS.cues, cross0 = SNM.SNEAK_STATS.crossings;
  R.go(R.free([32.8, 0, 13.5]), 10);
  return { R, T1, fc, seen0, mateAt, look, cues0, cross0 };
}
/** how far inside the door the staring man stands: near enough that nothing in the room stands between his eyes and a crouched man in the doorway */
const STARE_M = 3.0;
const sideOf = (R: Run, name: string, p: number[]) => { const d = R.door(name), c = d.frameCentre, th = d.def.closedDir; return (p[0] - c[0]) * -Math.sin(th) + (p[2] - c[2]) * Math.cos(th); };

export const sneakWaits = guarded(async () => {
  const F = family('sneak-waits: he stops short of a lit doorway a defender is looking at, says so, and crosses when the eyes are off it');
  { const { R, T1, fc, seen0, mateAt, look, cues0, cross0 } = await doorway(2, true);
    const off = waitUnder(R, SNEAK.nagSecs + 3, () => [fc[0], 0, fc[2]]);
    F.verdict(off < 0.4, 'staged: both men look at the lit doorway the whole wait', `${(off * 180 / Math.PI).toFixed(0)}° off it at the worst`);
    const m = R.mate, word = SNM.sneakWord(R.g, m), mySide = Math.sign(sideOf(R, 'breakroom_n', R.pos)), his = sideOf(R, 'breakroom_n', m.char.pos) * mySide;
    F.verdict(word === 'waiting on eyes' && Math.hypot(m.char.vel[0], m.char.vel[2]) < 0.05, 'the man looking at the lit doorway: the partner waits, still', `after ${SNEAK.nagSecs + 3} s: '${word}', at ${at(m.char.pos)} (he was at ${at(mateAt)} when the man turned), ${f2(flat(m.char.pos, fc))} m from the frame, Sam at ${at(R.pos)}, T1 at ${at(T1.char.pos)}`);
    F.verdict(his < -0.3, "… on the corridor's side of the door, not in it", `${f2(-his)} m short of the frame's line`);
    F.verdict(m.char.anim.crouch > 0.5, '… low', `crouch ${f2(m.char.anim.crouch)}`);
    F.verdict(R.mateSeenS - seen0 < 0.35, "… and the man's eyes barely have him (the moment he turned, at most)", `eyes on the partner since the man turned: ${f2(R.mateSeenS - seen0)} s; T1 awareness ${f2(T1.awareness)}`);
    F.verdict(SNM.SNEAK_STATS.cues - cues0 >= 1 && /still on it/.test(m.bubble?.text ?? ''), `he says so over the net — and what he is waiting for after ${SNEAK.nagSecs} s (never chattier than a line every ${SNEAK.cueGap} s but for that one)`, `lines since the man turned: ${SNM.SNEAK_STATS.cues - cues0} · his bubble now: ${m.bubble ? `“${m.bubble.text}”${m.bubble.radio ? ' (radio)' : ''}` : '—'}`);
    // the eyes come off it
    look(h => [h.char.pos[0] + 1, 0, h.char.pos[2] + 8]); const back = waitUnder(R, 1.2, h => [h.char.pos[0] + 1, 0, h.char.pos[2] + 8]);
    F.verdict(back < 0.4, 'staged: both men look the other way', `${(back * 180 / Math.PI).toFixed(0)}° off it at the worst`);
    const seen1 = R.mateSeenS, ok = R.until(() => flat(m.char.pos, R.pos) < 2.6, 12);
    F.verdict(ok, 'the man turned away: he crosses and is back at Sam’s shoulder', `${ok ? 'rejoined' : 'NOT back'}: ${f2(flat(m.char.pos, R.pos))} m from Sam at ${at(m.char.pos)}; eyes on him in the crossing ${f2(R.mateSeenS - seen1)} s; T1 awareness ${f2(T1.awareness)}`);
    F.verdict(SNM.SNEAK_STATS.crossings - cross0 >= 1, '… counted as a crossing', `crossings since the man first turned: ${SNM.SNEAK_STATS.crossings - cross0}`); }
  return F.done();
});

export const sneakLight = guarded(async () => {
  const F = family('sneak-light: the light put out, he comes on with the man still looking');
  const { R, T1 } = await doorway(4, true); R.wait(4); const m = R.mate;
  F.verdict(SNM.sneakWord(R.g, m) === 'waiting on eyes', 'he is waiting on the lit doorway', `'${SNM.sneakWord(R.g, m)}' at ${at(m.char.pos)}`);
  const t = R.g.targets.find((x: any) => x.name === 'panel_corridor_20'); R.g.lightHome.stage(t, null, null, R.g.time + 30);   // (staged: the panel out for the length of the probe — Sam's OCP holds it nine seconds)
  const seen0 = R.mateSeenS, ok = R.until(() => flat(m.char.pos, R.pos) < 2.6, 10);
  F.verdict(ok && R.mateSeenS - seen0 < 0.05, 'the panel out: he crosses in the dark, unseen, the man still looking at the door', `${ok ? 'rejoined' : 'NOT back'} (${f2(flat(m.char.pos, R.pos))} m from Sam); eyes on him ${f2(R.mateSeenS - seen0)} s; T1 awareness ${f2(T1.awareness)}`);
  return F.done();
});

/** told to HOLD in the corridor, Sam on through the lit doorway alone, then FALL IN with a defender looking at it: he comes by Sam's steps and waits the same */
export const sneakFallIn = guarded(async () => {
  const F = family("sneak-fallin: told to fall in from a hold, he comes on by the steps Sam took meanwhile — and waits on the same eyes, through a door that shuts");
  const R = await rig(8, true), T1 = R.hostiles[0], d = R.door('breakroom_n'), fc = d.frameCentre, m = R.mate;
  const spot = R.free([fc[0], 0, fc[2] + STARE_M]); T1.char.pos[0] = spot[0]; T1.char.pos[2] = spot[2];
  const look = (to: (h: any) => number[]) => { for (const h of R.hostiles) stare(R, h, to(h)); };
  look(h => [h.char.pos[0] + 1, 0, h.char.pos[2] + 8]);
  R.crouch(true); putInCorridor(R, 23); R.go(R.free([24.6, 0, 11.2]), 60); R.gather();
  const why = R.g.orders.give('hold', { kind: 'none', at: [R.pos[0], 0, R.pos[2]] }), held = [...m.char.pos];
  R.toDoor('breakroom_n'); R.ease('breakroom_n'); R.holster(true); R.through('breakroom_n', 1.0); R.go(R.free([32.8, 0, 13.5]), 10);
  F.verdict(!why && flat(m.char.pos, held) < 1.5, 'he held in the corridor while Sam went through', why ? `hold refused — ${why}` : `${f2(flat(m.char.pos, held))} m from where he was told, ${f2(flat(m.char.pos, R.pos))} m from Sam`);
  // the door held open and the man staring at it: fall in
  d.def.closer = false; if (d.isClosed()) { R.go(R.free([31.0, 0, 13.2]), 6); R.ease('breakroom_n'); R.go(R.free([32.8, 0, 13.5]), 6); }
  look(() => [fc[0], 0, fc[2]]); const seen0 = watchEyes(R);
  const why2 = R.g.orders.give('fallIn', { kind: 'none', at: [R.pos[0], 0, R.pos[2]] }); R.wait(6);
  const mySide = Math.sign(sideOf(R, 'breakroom_n', R.pos)), side = () => sideOf(R, 'breakroom_n', m.char.pos) * mySide;
  F.verdict(!why2 && flat(m.char.pos, held) > 2 && side() < -0.3 && SNM.sneakWord(R.g, m) === 'waiting on eyes', 'fall in: he comes up the corridor and waits short of the watched doorway', why2 ? `fall in refused — ${why2}` : `'${SNM.sneakWord(R.g, m)}' at ${at(m.char.pos)}, ${f2(flat(m.char.pos, held))} m from his hold, ${f2(-side())} m short of the frame's line; eyes on him ${f2(R.mateSeenS - seen0)} s; door ${d.isClosed() ? 'shut' : 'open'}`);
  // the door let go: it swings to between them — he does not take the shut door for the man having looked away
  // (where leaves stay as they are left — the page's stand — no closer does it: the leaf is pulled to, as a hand lets it go shut)
  d.def.closer = true; d.quietT = 3; if (!R.g.doors.hasCloser(d)) R.g.doors.pullTo(d, d.lastWho); R.until(() => d.isClosed(), 6); const shutAt = R.t; R.wait(4);
  F.verdict(d.isClosed() && side() < -0.3 && SNM.sneakWord(R.g, m) === 'waiting on eyes', 'the door swung shut between them: four seconds on he is still waiting (he remembers who was looking)', `door ${d.isClosed() ? 'shut' : 'OPEN'} at ${f2(shutAt)} s; '${SNM.sneakWord(R.g, m)}' at ${at(m.char.pos)}`);
  look(h => [h.char.pos[0] + 1, 0, h.char.pos[2] + 8]);
  const ok = R.until(() => flat(m.char.pos, R.pos) < 2.6, SNEAK.memory + 8);
  F.verdict(ok && T1.state === 'patrol', `… and comes on once that has gone stale (${SNEAK.memory} s): through the door and at Sam’s shoulder, nobody the wiser`, `${ok ? `rejoined ${f2(R.t - shutAt)} s after it shut` : 'NOT back'} (${f2(flat(m.char.pos, R.pos))} m from Sam at ${at(m.char.pos)}); eyes on him since the order ${f2(R.mateSeenS - seen0)} s; T1 ${T1.state}, awareness ${f2(T1.awareness)}`);
  return F.done();
});

export const sneakStand = guarded(async () => {
  const F = family('sneak-stand: Sam stopped just inside a lit doorway — the partner does not stand in the light');
  let a = { lit: 0, line: false, d: 0, p: [0, 0, 0] as number[] };
  {
    const R = await rig(6, true), d = R.door('breakroom_n');
    R.crouch(true); R.go(R.free([24.6, 0, 11.2]), 60); R.gather(); R.toDoor('breakroom_n'); R.gather();
    for (const h of R.hostiles) stare(R, h, [h.char.pos[0] + 1, 0, h.char.pos[2] + 8]);   // (looking away south: nobody WATCHES the doorway — but the near man has a line to it)
    R.ease('breakroom_n'); R.holster(true); R.through('breakroom_n', 1.3); R.wait(7);
    const m = R.mate, p = m.char.pos, low = m.char.anim.crouch > 0.5;
    const lit = m.visibility, pc = m.char.bones.chest, ph = m.char.bones.head;   // (his own meter and his own bones: what the eye test reads)
    const line = R.hostiles.some((h: any) => { const e = h.char.bones.head; return flat(e, p) < 16 && !(R.g.col.segmentBlocked(e, pc) && R.g.col.segmentBlocked(e, ph)); }); void low;
    a = { lit, line, d: flat(p, R.pos), p: [...p] }; void d;
  }
  const w = (x: typeof a) => `at ${at(x.p)}, ${f2(x.d)} m from Sam, lit ${Math.round(100 * x.lit)} %, a defender's line to him: ${x.line ? 'yes' : 'no'}`;
  F.verdict(!(a.lit >= SNEAK.litAt && a.line), 'where he stands is not lit with a line from a defender', w(a));
  F.verdict(a.d < 4.5, '… and he is still close', `${f2(a.d)} m from Sam`);
  return F.done();
});

/** the corridor that runs straight at a door (the break room's west door): every step Sam took to it is in the doorway's line */
export const sneakAside = guarded(async () => {
  const F = family("sneak-aside: Sam at an open door at the end of a straight corridor — the partner steps aside, out of the door's line");
  let a: { lit: number; line: boolean; funnel: boolean; d: number; p: number[]; seen: number; aw: number; asides: number };
  {
    const R = await rig(6, true), d = R.door('breakroom_w'), fc = d.frameCentre, T1 = R.hostiles[0]; d.def.closer = false; const asides0 = SNM.SNEAK_STATS.asides;
    const th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th), in0 = Math.sign((T1.char.pos[0] - fc[0]) * nx + (T1.char.pos[2] - fc[2]) * nz) || 1;
    const spot = R.free([fc[0] + nx * in0 * 4.3, 0, fc[2] + nz * in0 * 4.3 + 0.3]); T1.char.pos[0] = spot[0]; T1.char.pos[2] = spot[2];   // (square to the door, 4.3 m inside it: the night breach's near man)
    const look = (to: (h: any) => number[]) => { for (const h of R.hostiles) stare(R, h, to(h)); };
    look(h => [h.char.pos[0] + nx * in0 * 8, 0, h.char.pos[2] + nz * in0 * 8]);   // (their backs to the door)
    R.crouch(true); R.go(R.free([fc[0] - nx * in0 * 5.5, 0, fc[2] - nz * in0 * 5.5]), 60); R.gather(); R.toDoor('breakroom_w'); R.ease('breakroom_w'); R.wait(4);
    const m = R.mate;
    look(() => [fc[0], 0, fc[2]]); const seen0 = watchEyes(R); R.wait(4);
    const p = m.char.pos, e = T1.char.bones.head, line = flat(e, p) < 16 && !(R.g.col.segmentBlocked(e, m.char.bones.chest) && R.g.col.segmentBlocked(e, m.char.bones.head));
    const A = d.def.hinge, B = [A[0] + Math.cos(th) * d.def.width, A[1] + Math.sin(th) * d.def.width], o = (px: number, pz: number, qx: number, qz: number, rx: number, rz: number) => (qx - px) * (rz - pz) - (qz - pz) * (rx - px);
    const funnel = o(e[0], e[2], p[0], p[2], A[0], A[1]) * o(e[0], e[2], p[0], p[2], B[0], B[1]) <= 0 && o(A[0], A[1], B[0], B[1], e[0], e[2]) * o(A[0], A[1], B[0], B[1], p[0], p[2]) <= 0;
    a = { lit: m.visibility, line, funnel, d: flat(p, R.pos), p: [...p], seen: R.mateSeenS - seen0, aw: T1.awareness, asides: SNM.SNEAK_STATS.asides - asides0 };
  }
  const w = (x: typeof a) => `at ${at(x.p)}, ${f2(x.d)} m from Sam, lit ${Math.round(100 * x.lit)} %, the man's line to him: ${x.line ? 'yes' : 'no'}, in the open door's funnel: ${x.funnel ? 'yes' : 'no'}; the man's eyes on him after he turned to the door: ${f2(x.seen)} s, awareness ${f2(x.aw)}`;
  F.verdict(!a.funnel && !a.line, "he has stepped out of the door's line", w(a));
  F.verdict(a.seen < 0.1 && a.aw < 0.05, "… and the man who turns to the door does not see him", `eyes on him ${f2(a.seen)} s, awareness ${f2(a.aw)}`);
  F.verdict(a.d < 5 && a.asides >= 1, '… still close, and counted as a step aside', `${f2(a.d)} m from Sam; steps aside ${a.asides}`);
  return F.done();
});

/** the same corridor and door, SHUT, Sam stood at it about to open it: the partner is out of the doorway's line before the leaf moves */
export const sneakDoor = guarded(async () => {
  const F = family('sneak-door: Sam at a shut door with a man beyond it — the partner is out of the doorway\'s line BEFORE it opens, and still when it does');
  let a: { lit: number; funnel: boolean; d: number; p: number[]; v: number; seen: number; aw: number; moved: number };
  {
    const R = await rig(6, true), d = R.door('breakroom_w'), fc = d.frameCentre, T1 = R.hostiles[0]; d.def.closer = false;
    const th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th), in0 = Math.sign((T1.char.pos[0] - fc[0]) * nx + (T1.char.pos[2] - fc[2]) * nz) || 1;
    const spot = R.free([fc[0] + nx * in0 * 4.3, 0, fc[2] + nz * in0 * 4.3 + 0.3]); T1.char.pos[0] = spot[0]; T1.char.pos[2] = spot[2];
    const look = (to: (h: any) => number[]) => { for (const h of R.hostiles) stare(R, h, to(h)); };
    look(h => [h.char.pos[0] + nx * in0 * 8, 0, h.char.pos[2] + nz * in0 * 8]);   // (their backs to the door)
    R.crouch(true); R.go(R.free([fc[0] - nx * in0 * 5.5, 0, fc[2] - nz * in0 * 5.5]), 60); R.gather(); R.toDoor('breakroom_w'); R.wait(5);   // (the leaf untouched: five seconds at it, as long as a pick)
    const m = R.mate, p0 = [...m.char.pos], e = T1.char.pos, lit = m.visibility, v = Math.hypot(m.char.vel[0], m.char.vel[2]);
    const A = d.def.hinge, B = [A[0] + Math.cos(th) * d.def.width, A[1] + Math.sin(th) * d.def.width], o = (px: number, pz: number, qx: number, qz: number, rx: number, rz: number) => (qx - px) * (rz - pz) - (qz - pz) * (rx - px);
    const funnel = o(e[0], e[2], p0[0], p0[2], A[0], A[1]) * o(e[0], e[2], p0[0], p0[2], B[0], B[1]) <= 0 && o(A[0], A[1], B[0], B[1], e[0], e[2]) * o(A[0], A[1], B[0], B[1], p0[0], p0[2]) <= 0;
    look(() => [fc[0], 0, fc[2]]); const seen0 = watchEyes(R); R.ease('breakroom_w'); R.wait(1.5);   // (the man turned to the door as it opens)
    a = { lit, funnel, d: flat(p0, R.pos), p: p0, v, seen: R.mateSeenS - seen0, aw: T1.awareness, moved: flat(p0, m.char.pos) };
  }
  const w = (x: typeof a) => `at ${at(x.p)}, ${f2(x.d)} m from Sam, lit ${Math.round(100 * x.lit)} %, in the shut door's line: ${x.funnel ? 'yes' : 'no'}, moving ${f2(x.v)} m/s; once it opens: eyes on him ${f2(x.seen)} s in 1.5 s, the man's awareness ${f2(x.aw)}, he moved ${f2(x.moved)} m`;
  F.verdict(!a.funnel && a.v < 0.1, "out of the door's line and still before the leaf moves", w(a));
  F.verdict(a.seen < 0.1, '… and the man looking at the door as it opens does not see him', `eyes on him ${f2(a.seen)} s, awareness ${f2(a.aw)}`);
  F.verdict(a.d < 5, '… still close to Sam', `${f2(a.d)} m`);
  return F.done();
});

export const sneakPost = guarded(async () => {
  const F = family('sneak-post: sent to a mark while Sam is low — he goes there low, at the low pace (a plain posted walk is upright, gun up)');
  const walk = async (crouch: boolean, order = 'move') => {
    const R = await rig(5, true); R.crouch(crouch); putInCorridor(R, 12); R.wait(1);
    const m = R.mate, to = R.free([20, 0, 11.2]);
    R.g.orders.assign(m, { order, post: [to[0], 0, to[2]], face: null, watch: null, stance: 'none', upper: 'aim', move: { stance: 'none', upper: 'aim' }, going: 'moving', there: 'set' });
    let low = 0, up = 0, top = 0;
    for (let i = 0; i < 480 && flat(m.char.pos, to) > 0.5; i++) { R.step(1); const sp = Math.hypot(m.char.vel[0], m.char.vel[2]); top = Math.max(top, sp); if (sp > 0.3) { if (m.char.anim.crouch > 0.5) low += 1 / 60; else up += 1 / 60; } }
    R.wait(1);
    return { low, up, top, d: flat(m.char.pos, to), kneels: m.char.anim.crouch > 0.5 };
  };
  const words = (s: Awaited<ReturnType<typeof walk>>) => `low ${f2(s.low)} s, upright ${f2(s.up)} s, top ${f2(s.top)} m/s, ${f2(s.d)} m from the mark, ${s.kneels ? 'low' : 'upright'} on it`;
  const on = await walk(true), up = await walk(false), stack = await walk(true, 'stack');
  F.verdict(on.low > 3 && on.up < 0.5 && on.kneels, 'Sam low: he walks to the mark low and is low on it', words(on));
  F.verdict(on.top <= SNEAK.lowPace + 0.06 && on.d < 0.6, `… at no more than the low pace (${SNEAK.lowPace} m/s), and he gets there`, words(on));
  F.verdict(up.up > 3 && up.low < 0.3, 'Sam upright: he walks to the mark upright', words(up));
  F.verdict(stack.up > 3 && stack.low < 0.3, "Sam low, a stack's mark: its carriage is the door's (upright)", words(stack));
  return F.done();
});

export const sneakScope = guarded(async () => {
  const F = family("sneak-scope: the silent, unnoticed squad's — never with a defender hunting, never a man told to hold");
  { const R = await rig(7, true); R.crouch(true); R.go(R.free([14, 0, 11.2]), 30); const m = R.mate;
    F.verdict(SNM.sneaking(R.g, m) && m.char.anim.crouch > 0.5, 'silent and unnoticed: he sneaks', `sneaking ${SNM.sneaking(R.g, m)}, crouch ${f2(m.char.anim.crouch)}`);
    R.hostiles[0].state = 'alert'; R.go(R.free([17, 0, 11.2]), 20);
    F.verdict(!SNM.sneaking(R.g, m) && m.char.anim.crouch < 0.5, "a defender hunting: the follower is the build's own (upright)", `sneaking ${SNM.sneaking(R.g, m)}, crouch ${f2(m.char.anim.crouch)}`); }
  { const R = await rig(7, true); R.crouch(true); R.go(R.free([14, 0, 11.2]), 30); const m = R.mate, p0 = [...m.char.pos];
    const why = R.g.orders.give('hold', { kind: 'none', at: [R.pos[0], 0, R.pos[2]] }); R.go(R.free([20, 0, 11.2]), 20);
    F.verdict(!why && flat(m.char.pos, p0) < 1.5 && m.char.anim.crouch < 0.5, 'told to hold: he holds, upright, as the build before (the orders part owns a holding man)', `${why ? `hold refused — ${why}` : `moved ${f2(flat(m.char.pos, p0))} m, crouch ${f2(m.char.anim.crouch)}`}`); }
  return F.done();
});


if (import.meta.main) {
  const all: Record<string, () => Promise<boolean>> = { 'sneak-low': sneakLow, 'sneak-steps': sneakSteps, 'sneak-waits': sneakWaits, 'sneak-light': sneakLight, 'sneak-fallin': sneakFallIn, 'sneak-stand': sneakStand, 'sneak-aside': sneakAside, 'sneak-door': sneakDoor, 'sneak-post': sneakPost, 'sneak-scope': sneakScope,  };
  const want = process.argv.slice(2); let ok = true;
  for (const [n, fn] of Object.entries(all)) if (!want.length || want.includes(n)) ok = (await fn()) && ok;
  process.exit(ok ? 0 : 1);
}
