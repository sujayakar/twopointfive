// The 'nvg' family: THE TEAM'S NIGHT VISION (the maintainer, 10-06: "the squad members should have night vision and use it accordingly. this could either be
// following your lead or knowing when they haven't been compromised and flashlight would ruin that" · "but the lights make sense too, especially when things
// are loud" · "ready or not is a good base case here"). The rule is game/nvg.ts; this family is its check. Run through probes.ts:
//   bun run tools/qa/probes.ts nvg | nvg-quiet | nvg-lead | nvg-loud | nvg-sight | nvg-fight
// PASS / FAIL each, all headless (the Game of headless.ts stood up on each row's own level; nothing is drawn). THE SCENARIOS are scene/scenarioList.ts
// SCENARIOS, row by row, each started as a click on its row starts it (bang-tally.ts stage()).
//   nvg-quiet   NOT COMPROMISED, IN THE DARK: every scenario with the floor emptied of hostiles (nothing but the rule decides), the team told to fall in and then
//               sent to stack on a held room's door — the walk af55d89 lit ("a pair sent down a dark corridor"). The team is 'dark' the whole way, no weapon
//               light burns but a man's whom the tube shows nothing (no light of the place at all somewhere he is looking: nvg.ts blackAt — the log line says
//               so), and every unlit man in a dim place has his goggles down (a moonlit lot is not dim: bare eyes, no light).
//   nvg-lead    FOLLOWING SAM'S LEAD, both ways. His goggles down and his light off on a floor that has just gone LOUD: the team stays dark all the same, not
//               one light, black places included. His own weapon light on, on a quiet floor: theirs are free again (the light rule's picture),
//               and dark again a moment after he puts it out.
//   nvg-loud    ITS OWN JUDGEMENT. A charge goes off in the team's ears: 'compromised' inside a frame, the walk is lit again by the light rule, a lit man's
//               goggles come up. NVG.quietSecs of quiet later the team goes dark again and one of them says so. An alarm on the floor does the same as the
//               charge.
//   nvg-sight   WHAT THE GOGGLES DO: the tube's curve is the picture's own (post.wgsl's lines are still the ones nvg.ts copies), monotone, nothing from
//               nothing; a man with his goggles up gains nothing.
//   nvg-fight   END TO END: Sam's goggles down (the team dark whatever happens), the team sent through a held door with the room's men alive, every
//               scenario. Not one weapon light burns from start to finish, and the team still takes targets and drops men.
// The rule and its two halves (NVG.on, lead, own) are settled on: no line asks one off and nothing here writes one. Until 2026-11-07 every walk was also
// made with the switch off ("the old picture", "as with the switch off"), with `lead` and `own` off; nvg-sight held every hostile to the same eye with the
// goggles off (seen by the tube only, never worse, nothing gained on a black man, a hostile's eye the same either way); nvg-fight counted the targets taken
// "through the tube alone" against the switch off; nvg-rig counted the goggles' boxes as the boxes with the rule on less those with it off; nvg-off asked
// the module absent, a flip in play and the panel's switch rows; and `digest` printed the switch-off hashes for a checkout of the base. All of that was the
// off side: out, with the probes nvg-rig and nvg-off. (Lost with no keeper: what the goggles add to a sighting, and their boxes on the man.)
// The family's own red path: NVG_BREAK=1 bun run tools/qa/probes.ts nvg-quiet must FAIL (Sam's own weapon light is burning under the probe: the team's
// lights are free, and no floor is walked dark).
import { readFileSync } from 'node:fs';
import { ROOT } from './headless.ts';
import { stage, setKnobs } from './bang-tally.ts';
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const CBM: any = await import(`${ROOT}/src/game/combat.ts`);
const ENVM: any = await import(`${ROOT}/src/game/env.ts`).catch(() => null);   // (the game's own tube gain: null in a checkout from before the file)
const NVM: any = await import(`${ROOT}/src/game/nvg.ts`).catch(() => null);   // (null in a checkout from before the module: only `digest` runs there)
const NVG: any = NVM?.NVG ?? { on: false };

const VERBOSE = process.argv.includes('--verbose');
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
// (bang-tally.ts stage() puts every knob table back to the build's own before each staging: an override goes through its setKnobs — and is taken off the
//  table here as well, for the game already standing)
const reset = () => { setKnobs({}); };
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (VERBOSE) return await fn(); const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => x.toFixed(2);
const pct = (a: number, b: number) => b ? `${(100 * a / b).toFixed(0)} %` : '-';
const IDS: string[] = (SCENARIOS as any[]).map(e => e.id);

// ---------------------------------------------------------------- the staging the first three probes share
/** what the floor is made to be before the walk: `loud` = a charge let off twelve metres from Sam (in every teammate's ears), `nv` = Sam's goggles down,
 *  `light` = Sam's pistol up with its light burning, `alarm` = the floor's alarm up a step */
interface Prep { loud?: boolean; nv?: boolean; light?: boolean; alarm?: boolean }
interface Walk {
  team: number; frames: number; modes: Map<string, number>;
  /** man-frames on the way to a post (further than the light rule's onM from it) · of them: light on · goggles down */
  walk: number; lit: number; gog: number;
  /** man-frames with a light on that the rule did not call black · man-frames unlit in a place the probe's own reading calls dim, and of them goggles down ·
   *  the longest run of frames one man had his light on AND his goggles down */
  litNotBlack: number; dimUnlit: number; dimUnlitGog: number; both: number;
  log: string[]; said: string[]; door: string;
}
const samLight = (g: any, on: boolean) => { const pl = g.player; if (on) { pl.slot = 'pistol'; pl.holstered = false; } pl.pistol.lightOn = on; };
const bang = (g: any) => { const p = g.player.char.pos; g.events.push({ kind: 'bang', pos: [p[0] + 12, 1, p[2]], time: g.time, loud: true }); };
const modeOf = (g: any): string => { const st = NVM.nvgState(g); return st.mode === 'off' ? 'off' : `${st.mode}/${st.why}`; };
/** a scenario with every hostile taken off the floor and the team fallen in on Sam — nothing but the orders and the rule decides what follows */
async function emptied(scn: string) {
  const S: any = await quiet(() => stage(scn, 1)); const { g, step } = S; const O = g.orders;
  const cands: { ent: any; dist: number }[] = []; for (const d of S.defs) for (const e of d.room.entries) if (!cands.some(c => c.ent === e)) cands.push({ ent: e, dist: flat(g.player.char.pos, e.at) });
  cands.sort((a, b) => a.dist - b.dist);
  for (const x of g.guards) if (x.team === 'hostile' && x.state !== 'dead') await quiet(() => CBM.killGuard(g, x, [0, 0, 1], true));
  const team: any[] = O.squad().filter((x: any) => x.state !== 'dead'); const sam = g.player.char.pos;
  step(60 * 6);   // (the hand-off's own noise is out of everybody's ears)
  await quiet(() => O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all')); step(60 * 3);
  return { ...S, O, team, cands, sam };
}
/** the walk: the team sent to stack on the nearest held room's door it can be sent to, watched until every man is there (and three seconds on the stack) */
async function walk(X: any, prep: Prep): Promise<Walk | null> {
  const { g, step, O, team } = X; NVM.NVG_LOG.length = 0;
  if (prep.nv) g.player.nv = true;
  if (prep.light) samLight(g, true);
  if (prep.alarm) await quiet(() => GDM.raiseEscalation(g, 1));
  if (prep.loud) bang(g);
  step(prep.light ? 30 : 3);   // (the pistol comes up before its light is one)
  const R: Walk = { team: team.length, frames: 0, modes: new Map(), walk: 0, lit: 0, gog: 0, litNotBlack: 0, dimUnlit: 0, dimUnlitGog: 0, both: 0, log: [], said: [], door: '' };
  const feet = (x: any) => g.visibilityOf(NVM.nvgPlaceLight(g, [x.char.pos[0], 1.2, x.char.pos[2]]));   // (the place's own light under him, the moon included: the rule's own reading, taken afresh)
  const run = new Map<any, number>(), dimRun = new Map<any, number>();
  const watch = () => {
    R.frames++; const m = modeOf(g); R.modes.set(m, (R.modes.get(m) ?? 0) + 1); const st = NVM.nvgState(g);
    for (const x of team) { if (x.state === 'dead') continue;
      const lit = !!x.flashlight?.enabled, gog = NVM.nvgDown(g, x), A = O.of(x), black = !!st.men.find((q: any) => q.gd === x)?.black;
      if (A?.post && flat(x.char.pos, A.post) >= GDM.LIGHTS.onM + 0.4) { R.walk++; if (lit) R.lit++; if (gog) R.gog++; }
      if (lit && !black) R.litNotBlack++;
      const b = lit && gog ? (run.get(x) ?? 0) + 1 : 0; run.set(x, b); R.both = Math.max(R.both, b);
      const d = !lit && feet(x) < NVG.darkAt * 0.8 ? (dimRun.get(x) ?? 0) + 1 : 0; dimRun.set(x, d);
      if (d > 60 * (NVG.downSecs + 0.5) + 2) { R.dimUnlit++; if (gog) R.dimUnlitGog++; }   // (past the half second his reading of the place may be old and the moment the flip takes)
    }
  };
  let why: string | void = 'no door';
  for (const c of X.cands) { why = await quiet(() => O.give('stack', O.targetAt(c.ent.door.frameCentre), 'all')); if (!why) { R.door = c.ent.door.def.name; break; } }
  if (why) return null;
  for (let f = 0; f < 60 * 30; f++) { step(); watch(); if (team.every((x: any) => x.state === 'dead' || O.of(x)?.arrived)) break; }
  for (let f = 0; f < 180; f++) { step(); watch(); }
  R.log = NVM.NVG_LOG.map((n: any) => `${n.who} ${n.what}`);
  return R;
}
/** Sam's light on and the team sent to a door: the first man seen walking with his light on and his goggles up, on that frame (null: nobody in 20 s) */
async function litWalker(X: any): Promise<any | null> {
  const { g, step, O, team } = X; samLight(g, true); step(30);
  for (const c of X.cands) if (!(await quiet(() => O.give('stack', O.targetAt(c.ent.door.frameCentre), 'all')))) break;
  for (let f = 0; f < 60 * 20; f++) { step(); const m = team.find((x: any) => x.state !== 'dead' && x.flashlight?.enabled && !NVM.nvgDown(g, x)); if (m) return m; }
  return null;
}
const modes = (R: Walk) => [...R.modes].map(([m, n]) => `${m} ${pct(n, R.frames)}`).join(', ');
const only = (R: Walk, m: string, slack = 3) => R.frames - (R.modes.get(m) ?? 0) <= slack;

// ---------------------------------------------------------------- nvg-quiet
export const nvgQuiet = guarded(async () => {
  const F = family('nvg-quiet: not compromised and in the dark, the team walks on its goggles — no light but where the tube has nothing');
  if (process.env.NVG_BREAK) console.log("  (NVG_BREAK: Sam's weapon light is burning under this probe — it must FAIL)");
  let cleanDark = 0;
  for (const id of IDS) {
    const on = await walk(await emptied(id), process.env.NVG_BREAK ? { light: true } : {});
    if (!on) { F.verdict(false, `${id}: a door to walk to`, 'no held room\'s door took the stack order'); continue; }
    const tubeless = on.log.filter(l => l.includes('too dark for the tube')).length;
    if (on.lit === 0 && on.dimUnlit > 0) cleanDark++;
    F.verdict(on.walk > 60 && only(on, 'dark/not compromised') && on.litNotBlack === 0 && on.dimUnlitGog >= 0.97 * on.dimUnlit && on.both <= 60 * NVG.upSecs + 12,
      `${id}: dark to the door (${on.door})`,
      `${on.team} men · ${modes(on)} · walking man-frames ${on.walk}: lit ${pct(on.lit, on.walk)}, goggles down ${pct(on.gog, on.walk)} · lit where the tube had something: ${on.litNotBlack} · unlit in a dim place: goggles down ${pct(on.dimUnlitGog, on.dimUnlit)} of ${on.dimUnlit}${tubeless ? ` · ${tubeless}× 'too dark for the tube' (no light of the place there at all)` : ''}`);
  }
  F.verdict(cleanDark >= 4, 'not vacuous: most floors are walked with no light at all and the goggles down',
    `${cleanDark} of ${IDS.length} scenarios with not one lit man-frame on the walk and goggled man-frames in the dim`);
  return F.done();
});

// ---------------------------------------------------------------- nvg-lead
export const nvgLead = guarded(async () => {
  const F = family('nvg-lead: Sam\'s goggles keep the team dark through anything; Sam\'s light frees theirs');
  let litFree = 0;
  for (const id of IDS) {
    const dark = await walk(await emptied(id), { loud: true, nv: true });
    const X = await emptied(id); const free = await walk(X, { light: true }); const samLit = !!X.g.player.light.enabled;
    if (!dark || !free) { F.verdict(false, `${id}: a door to walk to`, 'no held room\'s door took the stack order'); continue; }
    F.verdict(dark.walk > 60 && only(dark, 'dark/your goggles') && dark.lit === 0 && dark.litNotBlack === 0 && dark.dimUnlitGog >= 0.97 * dark.dimUnlit,
      `${id}: his goggles down, a charge just gone off — dark all the same`,
      `${modes(dark)} · walking man-frames ${dark.walk}: lit ${dark.lit}, goggles down ${pct(dark.gog, dark.walk)} · unlit in a dim place: goggles down ${pct(dark.dimUnlitGog, dark.dimUnlit)} of ${dark.dimUnlit}`);
    if (free.lit > 0) litFree++;
    F.verdict(samLit && only(free, 'free/your light') && free.both <= 60 * NVG.upSecs + 12,
      `${id}: his light on — theirs are the light rule's again`,
      `Sam's light ${samLit ? 'burning' : 'NOT burning (the probe could not light it)'} · ${modes(free)} · walking man-frames ${free.walk}: lit ${free.lit} · longest light-and-goggles overlap ${free.both} frames`);
    // …and out again: dark a moment after
    samLight(X.g, false); X.step(Math.round(60 * (NVG.leadTail + 1.5)));
    const after = modeOf(X.g), lit = X.team.filter((x: any) => x.flashlight?.enabled && !NVM.nvgState(X.g).men.find((q: any) => q.gd === x)?.black).length;
    F.verdict(after === 'dark/not compromised' && lit === 0, `${id}: his light out — dark again`, `${f2(NVG.leadTail + 1.5)} s after: ${after} · lights burning where the tube has something: ${lit}`);
  }
  F.verdict(litFree >= 4, 'not vacuous: with his light on the walks are lit', `${litFree} of ${IDS.length} scenarios with lit man-frames on the walk`);
  return F.done();
});

// ---------------------------------------------------------------- nvg-loud
export const nvgLoud = guarded(async () => {
  const F = family('nvg-loud: once the floor is loud the lights are the light rule\'s again; quiet long enough, the team goes dark and says so');
  let lit = 0;
  for (const id of IDS) {
    const X = await emptied(id); const r = await walk(X, { loud: true });
    if (!r) { F.verdict(false, `${id}: a door to walk to`, 'no held room\'s door took the stack order'); continue; }
    if (r.lit > 0) lit++;
    F.verdict(only(r, 'free/compromised') && r.both <= 60 * NVG.upSecs + 12,
      `${id}: a charge in their ears — compromised, lights as the light rule has them`,
      `${modes(r)} · walking man-frames ${r.walk}: lit ${r.lit}, goggles down ${pct(r.gog, r.walk)} · longest light-and-goggles overlap ${r.both} frames`);
    // quiet again: NVG.quietSecs after the charge the team is dark, and somebody said so
    const { g, step, team } = X; let said = '', darkAt = NaN; const t0 = g.time, calls = new Set<string>();
    for (let f = 0; f < 60 * (NVG.quietSecs + 5); f++) { step(); for (const x of team) if (x.bubble?.text === 'going dark.' && g.time - x.bubble.t < 0.05) { said = x.callsign; calls.add(`${x.callsign}@${x.bubble.t}`); } if (isNaN(darkAt) && modeOf(g) === 'dark/not compromised') darkAt = g.time; }
    const burning = team.filter((x: any) => x.flashlight?.enabled && !NVM.nvgState(g).men.find((q: any) => q.gd === x)?.black).length;
    F.verdict(!isNaN(darkAt) && calls.size === 1 && burning === 0 && modeOf(g) === 'dark/not compromised', `${id}: ${NVG.quietSecs} s of quiet — dark again, and called once`,
      `dark ${isNaN(darkAt) ? 'never' : `${f2(darkAt - t0)} s after the walk`} · 'going dark.' from ${said || 'nobody'}${calls.size > 1 ? ` (${calls.size} CALLS)` : ''} · lights burning where the tube has something: ${burning}`);
  }
  F.verdict(lit >= 4, 'not vacuous: loud, the walks are lit', `${lit} of ${IDS.length} scenarios with lit man-frames on the walk`);
  { const r = await walk(await emptied(IDS[0]), { alarm: true });
    F.verdict(!!r && only(r, 'free/compromised'), 'an alarm on the floor compromises like a charge', r ? modes(r) : 'no walk'); }
  return F.done();
});

// ---------------------------------------------------------------- nvg-sight
export const nvgSight = guarded(async () => {
  const F = family('nvg-sight: the tube is the picture\'s own, and a man with his goggles up has a bare eye');
  // the tube's curve is the picture's
  { const src = readFileSync(`${ROOT}/src/render/shaders/post.wgsl`, 'utf8');
    const same = src.includes('var sig = log(1.0 + lum * g) / log(1.0 + g);') && src.includes('sig = sig + n * 0.16 * (1.0 - sig * 0.85);') && src.includes('color * frame.exposure');
    F.verdict(same, 'post.wgsl still has the tube nvg.ts copies', same ? 'the gain curve, the noise line and the exposure in front of them are where they were' : 'the shader\'s night-vision lines changed: nvg.ts tubeVis must follow them'); }
  const S: any = await quiet(() => stage(IDS[0], 1)); const g = S.g;
  { const Es = [0, 0.001, 0.003, 0.01, 0.03, 0.1, 0.3, 1, 3, 10, 50]; const vs = Es.map(E => NVM.tubeVis(g, E)); let mono = true; for (let i = 1; i < vs.length; i++) if (vs[i] < vs[i - 1] - 1e-9) mono = false;
    const floorE = g.tune.visFloor ?? 0.03, gain0 = ENVM.ENV.tubeGain; ENVM.ENV.tubeGain = gain0 / 8; const lowGain = NVM.tubeVis(g, 0.01); ENVM.ENV.tubeGain = gain0;   // (the game's own gain, src/game/env.ts: the picture's settings.nvGain is no longer what a man's tube is worked out at)
    F.verdict(mono && vs[0] === 0 && vs.every(v => v >= 0 && v <= 1) && NVM.tubeVis(g, floorE) > 0.4 && g.visibilityOf(floorE) < 0.01 && lowGain < NVM.tubeVis(g, 0.01),
      'the tube: nothing from nothing, monotone, and the bare eye\'s zero is plain to it',
      `E → through the tube: ${Es.map((E, i) => `${E}→${f2(vs[i])}`).join(' · ')} · at the meter's zero (E ${floorE}): bare ${f2(g.visibilityOf(floorE))}, tube ${f2(NVM.tubeVis(g, floorE))} · an eighth of the game's own tube gain (ENV.tubeGain): 0.01→${f2(lowGain)}`); }
  // goggles up = a bare eye
  { const X = await emptied(IDS[0]); const up = await litWalker(X); const G = X.g; const h = G.guards.find((x: any) => x.team === 'hostile');
    if (!up || !h) F.verdict(false, 'goggles up: a bare eye', 'no teammate walked with his light on and his goggles up');
    else { const o = { ...h, irradiance: 0.03, visibility: 0 }; F.verdict(NVM.nvgSight(G, up, o) === 0 && NVM.tubeVis(G, 0.03) > 0.4, 'goggles up: a bare eye', `${up.callsign}, goggles up: the tube's term on a man at E 0.03 is ${f2(NVM.nvgSight(G, up, o))} (down it would be ${f2(NVM.tubeVis(G, 0.03))})`); } }
  return F.done();
});

// ---------------------------------------------------------------- nvg-fight
export const nvgFight = guarded(async () => {
  const F = family('nvg-fight: Sam\'s goggles down, a held room taken with not one light on');
  let takenTotal = 0, downTotal = 0;
  for (const id of IDS) {
    const S: any = await quiet(() => stage(id, 1)); const { g, step } = S; const O = g.orders, D = g.doorTools;
    g.player.nv = true; const team: any[] = O.squad().filter((x: any) => x.state !== 'dead'); const sam = g.player.char.pos;
    const cands: { ent: any; dist: number }[] = []; for (const d of S.defs) for (const e of d.room.entries) if (!cands.some(c => c.ent === e)) cands.push({ ent: e, dist: flat(sam, e.at) });
    cands.sort((a, b) => a.dist - b.dist);
    const before = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').length;
    let lit = 0, taken = 0, frames = 0, notDark = 0; const last = new Map<any, any>();
    // (the rule is asked while a man of the team stands: with every teammate down there is no team to be dark, and the mode says so)
    const watch = () => { frames++; if (team.some(x => x.state !== 'dead') && modeOf(g) !== 'dark/your goggles') notDark++;
      for (const x of team) if (x.state !== 'dead' && x.flashlight?.enabled) lit++;
      for (const P of g.fight.ops) { const h = P.tgt; if (h && h !== last.get(P.gd) && h.state !== 'dead') taken++;
        last.set(P.gd, h); } };
    await quiet(() => O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all')); for (let f = 0; f < 120; f++) { step(); watch(); }
    let why: string | void = 'no door', fc: any = null;
    for (const c of cands) { fc = c.ent.door.frameCentre; why = await quiet(() => O.give('stack', O.targetAt(fc), 'all')); if (!why) break; }
    if (why) { F.verdict(false, `${id}: a door to stack on`, String(why)); continue; }
    for (let f = 0; f < 60 * 30; f++) { step(); watch(); if (team.every(x => x.state === 'dead' || O.of(x)?.arrived)) break; }
    for (let f = 0; f < 120; f++) { step(); watch(); }
    const sent = await quiet(() => O.give('openClear', O.targetAt(fc), 'all')); let op: any = null, endT = NaN;
    for (let f = 0; f < 60 * 60; f++) { step(); watch(); op ??= D.ops[0] ?? null; if (op && !D.ops.includes(op) && isNaN(endT)) endT = g.time; if (!isNaN(endT) && g.time - endT > 8) break; }
    const alive = team.filter(x => x.state !== 'dead').length, down = before - g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').length;
    takenTotal += taken; downTotal += down;
    F.verdict(lit === 0 && notDark <= 3, `${id}: dark from the first step to the last`,
      `${frames} frames · lit man-frames ${lit} · frames not 'dark/your goggles' ${notDark}${sent ? ` · the entry was refused (${sent}): the walk and the wait only` : ''} · targets taken ${taken} · hostiles down ${down} of ${before} · team standing ${alive} of ${team.length}`);
  }
  F.verdict(takenTotal >= 3 && downTotal >= 3, 'not vacuous: men are found and dropped with no light burning',
    `${takenTotal} targets taken over ${IDS.length} scenarios · ${downTotal} hostiles down`);
  return F.done();
});
