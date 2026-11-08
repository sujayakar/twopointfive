// The 'irlaser' family: THE TEAM'S LASERS ARE INFRARED BEHIND GOGGLES (the maintainer, 10-06: "and then we could also model IR lasers (assuming enemies don't
// have night vision)" · "ready or not is a good base case here"). The rule is game/irlaser.ts; this family is its check. Run through probes.ts:
//   bun run tools/qa/probes.ts irlaser | ir-sam | ir-unseen | ir-off
// PASS / FAIL each, all headless (the Game of headless.ts stood up on each scenario's own level, started as a click on its row starts it — bang-tally.ts
// stage()). What is asked is WHICH BOXES the lasers' draw hands the renderer and whether anything in the game goes differently for them. Nothing is drawn: how
// an infrared line LOOKS through the tube is not judged here, and the GPU's light meter is not run (headless it is the CPU stand-in) — what is held instead is
// the mechanism the meter rests on: a laser is boxes flagged NoShadow (in no ray's grid) and never a light (the light list is hashed frame for frame).
//   ir-sam     SAM'S OWN: a scenario's team on the floor and the gun in his hands — a line and a dot on his bore in the infrared with his goggles down, in step
//              with the picture's swap to the tube frame for frame, both ways; nothing with goggles up, holstered, sprinting, down, with IR.sam or IR.on off,
//              or in a game with no team (the office at night, alone).
//   ir-unseen  NOBODY READS ONE. Every scenario played through a held door from the same seed five times: Sam's goggles down with IR on (infrared lines in
//              the frames' boxes), with IR off (none), on again (the control); Sam's goggles up with IR on (a goggled man's laser in NO frame's boxes) and
//              off. Hashed every frame: the whole game's digest, every man's awareness and light, every camera's integrator, the light list. On ≡ off both
//              ways — no enemy's awareness and no light is any different for an infrared laser or for none — while the counts say it is not vacuous
//              (frames an infrared beam lay ON a living enemy). In these plays Sam is the sandbox's invisible man and no camera has him, so then, closely:
//              Sam plainly lit and visible, his goggles down, 3.5 m in front of an enemy with his laser on (the enemy's awareness RISES — identically with
//              the laser on, off and on again). CAMERAS do not run while a scenario's machine has the floor (devices.ts sensing), and only a scenario
//              has a team: with a team, Sam lit in a lens fills nothing, laser or not; alone in the office the lens fills and no laser exists — no laser
//              and a running camera ever share a frame today (if that changes: a lens reads Sam's meter, never a box). What no scenario has is a RED teammate laser to compare (a scenario's teammate never shows one): the red line's own
//              invisibility to every reader is the same mechanism (NoShadow boxes, no light) and is held by ir-draw's 'every box NoShadow'.
//   ir-off     THE SWITCH: IR.on false is the build before — every teammate's laser the old red draw field for field whatever his goggles and the picture
//              do, Sam with none; flipped in play it takes hold inside a frame. (The frame-for-frame identity
//              against the merged base is `bun run tools/qa/irlaser-probes.ts digest`: one hash per scenario of a scripted minute with the switch off, the
//              frames' boxes included; the same file copied into a checkout of the base must print the same lines.)
import { ROOT } from './headless.ts';
import { stage } from './bang-tally.ts';
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { standUp } = await import('./headless.ts');
const CBM: any = await import(`${ROOT}/src/game/combat.ts`);
const NVM: any = await import(`${ROOT}/src/game/nvg.ts`);
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const RFM: any = await import(`${ROOT}/src/game/roomfight.ts`);
const DTM: any = await import(`${ROOT}/src/game/doortools.ts`);
const IRM: any = await import(`${ROOT}/src/game/irlaser.ts`).catch(() => null);
const CUM: any = await import(`${ROOT}/src/game/cues.ts`).catch(() => null);   // (the defenders' cues, a later default-on switch that rewords lines the digest carries: off for `digest`, where the checkout has it)   // (null in a checkout from before the module: only `digest` runs there)
const BDM: any = await import(`${ROOT}/src/game/buddy.ts`).catch(() => null);   // (the lone defender's radio call: null in a checkout from before it)
const DWM: any = await import(`${ROOT}/src/game/doorway.ts`).catch(() => null);   // (doorway manners: a later default-on switch, pinned off in `digest` where the checkout has it)
const RWM: any = await import(`${ROOT}/src/game/rearwatch.ts`).catch(() => null);   // (the stack's rear man on the door behind: a later default-on switch, pinned off in `digest` where the checkout has it)
const DGM: any = await import(`${ROOT}/src/game/digin.ts`).catch(() => null);   // (the dig-in: `digest` pins its later default-on brace rule off where the checkout has it)
const SLM: any = await import(`${ROOT}/src/game/silent.ts`).catch(() => null);   // (the squad's posture: a later default-on switch, pinned off in `digest` where the checkout has it)
const ILM: any = await import(`${ROOT}/src/game/irlight.ts`).catch(() => null);   // (the infrared light on the goggles, a later default-on switch: pinned off in `digest` where the checkout has it)
const IR: any = IRM?.IR ?? { on: false };
const LASER = CBM.LASER;

const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
const BREAK = !!process.env.IR_BREAK;
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const IR_DEFAULTS = JSON.parse(JSON.stringify(IR));
if ('teamOff' in IR_DEFAULTS) IR_DEFAULTS.teamOff = false;   // (the team's lasers put off, a later default-on switch: every probe here but ir-team reads the lines DRAWN — held off after each staging, which puts the stand's own back)
const teamDrawn = () => { if ('teamOff' in IR) IR.teamOff = false; };
const reset = () => { Object.assign(IR, JSON.parse(JSON.stringify(IR_DEFAULTS))); LASER.enemy = false; };
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (VERBOSE) return await fn(); const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const IDS: string[] = (SCENARIOS as any[]).map(e => e.id);

// ---------------------------------------------------------------- what a laser's boxes look like
const thin = (b: any) => !!b.emissive && b.h[0] < 0.02 && b.h[1] < 0.02;
/** an infrared laser's line or dot: no hue, no albedo (combat.ts beamBoxes' `ir`) */
const isIr = (b: any) => thin(b) && b.emissive[0] > 0 && b.emissive[0] === b.emissive[1] && b.emissive[1] === b.emissive[2] && b.albedo[0] === 0 && b.albedo[1] === 0;
/** a red laser's line or dot (a weapon's or a tripwire's) */
const isRed = (b: any) => thin(b) && b.emissive[0] > 0.1 && b.emissive[1] < b.emissive[0] * 0.1 && (b.albedo[0] === 0.02 || b.albedo[0] === 0.05);
const noShadow = (b: any) => !!(b.flags & BoxFlag.NoShadow);
/** a box on this character's bore (his laser, not a tripwire across the corridor or another man's) */
const onBore = (c: any, b: any) => { const m = c.muzzle, d = c.gunDir, r = v3.sub(b.c, m), t = v3.dot(r, d); return t > 0 && v3.len(v3.sub(r, v3.scale(d, t))) < 0.05; };
const draw = (g: any, gd: any) => { const out: any[] = []; CBM.laserBoxes(g, gd, out); return out; };
const same = (a: any, b: any) => JSON.stringify(a) === JSON.stringify(b);
/** lay a man's laser by hand: a muzzle half a metre off his chest and a unit bore (what the draw reads; nothing is stepped in between) */
const WAYS: number[][] = []; for (let k = 0; k < 8; k++) WAYS.push([Math.sin(k * Math.PI / 4 + 0.2), 0, Math.cos(k * Math.PI / 4 + 0.2)]);
const lay = (gd: any, dir: number[]) => { const c = gd.char, ch = c.bones.chest ?? [c.pos[0], 1.2, c.pos[2]]; c.muzzle = [ch[0] + dir[0] * 0.5, ch[1], ch[2] + dir[2] * 0.5]; c.gunDir = [...dir]; };
/** the picture: the tube's (as the shader has it once the goggles are down) or the bare eye's — written straight, for a draw asked between frames */
const picture = (g: any, tube: boolean) => { g.player.nv = tube; g.player.nvAmount = BREAK ? 1 : tube ? 1 : 0; };
const mates = (g: any): any[] => g.orders.squad().filter((x: any) => x.state !== 'dead');
const doorsOf = (S: any): any[] => { const sam = S.g.player.char.pos; const c: { ent: any; dist: number }[] = []; for (const d of S.defs) for (const e of d.room.entries) if (!c.some(q => q.ent === e)) c.push({ ent: e, dist: flat(sam, e.at) }); c.sort((a, b) => a.dist - b.dist); return c.map(q => q.ent); };
/** a scenario as its row starts it, the team fallen in on Sam and four seconds on: every man's goggles are where his place puts them */
async function settled(id: string) {
  const S: any = await quiet(() => stage(id, 1)); const { g, step } = S; teamDrawn();
  await quiet(() => g.orders.give('fallIn', g.orders.targetAt(g.player.char.pos), 'all')); step(240);
  return S;
}
function fnv() { let h = 0x811c9dc5 >>> 0; return { eat: (s: string) => { for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; } }, hex: () => (h >>> 0).toString(16).padStart(8, '0') }; }

// ---------------------------------------------------------------- ir-sam
/** Sam's laser among a frame's boxes: infrared boxes on his own bore */
const samIr = (g: any) => g.dynBoxes.filter((b: any) => isIr(b) && onBore(g.player.char, b));
export const irSam = guarded(async () => {
  const F = family('ir-sam: Sam\'s own gun carries one — infrared with his goggles down, nothing with them up');
  for (const id of [IDS[0], IDS[IDS.length - 1]]) {
    const S = await settled(id); const { g, step } = S; const pl = g.player;
    const hp = () => { const out: any[] = []; CBM.handProps(g, out); return out; };
    // goggles up: nothing of his
    const upNow = samIr(g).length;
    // goggles down, through Game.update: in step with the picture's swap
    let early = 0, late = 0, shown = 0, f0 = -1; pl.nv = true;
    for (let f = 0; f < 60; f++) { step(); const n = samIr(g).length, tube = pl.nvAmount > 0.5; if (n > 0 && !tube) early++; if (n === 0 && tube) late++; if (n > 0) { shown++; if (f0 < 0) f0 = f; } }
    const boxes = samIr(g), line = boxes.find((b: any) => b.h[2] > 0.02), dot = boxes.find((b: any) => b.h[2] <= 0.02);
    const from = v3.mad(pl.char.muzzle, pl.char.gunDir, 0.02), startsAtMuzzle = !!line && v3.len(v3.sub(v3.mad(line.c, pl.char.gunDir, -line.h[2]), from)) < 0.03;   // (dynBoxes are this frame's: the muzzle is this frame's too)
    const allNoShadow = boxes.every(noShadow), gun = pl.rifleUp ? 'rifle' : pl.pistolUp ? 'pistol' : 'nothing';
    const lightsOn = g.engine.lights.lights.length;
    const back = samIr(g).length;
    pl.holstered = true; step(3); const holst = samIr(g).length; pl.holstered = false; step(30); const drawn = samIr(g).length;
    // goggles up again: gone the frame the key is pressed
    let afterUp = 0; pl.nv = false; for (let f = 0; f < 30; f++) { step(); afterUp += samIr(g).length; }
    F.verdict(upNow === 0 && early === 0 && late === 0 && shown > 30 && !!line && startsAtMuzzle && allNoShadow && back > 0 && holst === 0 && drawn > 0 && afterUp === 0, `${id}: the ${gun} in his hands`,
      `goggles up: ${upNow} infrared boxes on his bore · N pressed: first shown ${f0 + 1} frames on, frames shown before the picture was the tube's ${early}, not shown once it was ${late} (want 0, 0) · then a line ${line ? (2 * line.h[2]).toFixed(2) + ' m from the muzzle' : 'MISSING'}${dot ? ' and a dot' : ', no dot (nothing inside 30 m)'}, all NoShadow ${allNoShadow} · back on ${back} · holstered ${holst}, drawn again ${drawn} · N again: infrared boxes over the next half second ${afterUp} (want 0) · lights in the list throughout: ${lightsOn}`);
    // sprinting / down
    pl.nv = true; step(30); const before = samIr(g).length; const his = () => hp().filter((q: any) => isIr(q) && onBore(pl.char, q)).length;   // (his own: the teammates' lines are in the same frame)
    const hpN = his(); pl.sprinting = true; const sprint = his(); pl.sprinting = false; pl.down = true; const dn = his(); pl.down = false;
    F.verdict(before > 0 && hpN > 0 && sprint === 0 && dn === 0, `${id}: not while he sprints, not from a downed man`, `gun up ${hpN} · sprinting ${sprint} · down ${dn}`);
    // the pistol: only once it is pointed — at the relaxed carry the bore is not where he aims
    pl.slot = 'pistol'; pl.holstered = false; pl.pistol.lightOn = false; step(90); const carry = pl.char.anim.upper, relaxed = samIr(g).length;
    pl.pistol.lightOn = true; step(60); const upper = pl.char.anim.upper, pointed = samIr(g).length; pl.pistol.lightOn = false;
    F.verdict(carry !== 'aim' && relaxed === 0 && upper === 'aim' && pointed > 0, `${id}: the pistol — no line from the relaxed carry, one once the gun is up`, `carried '${carry}': ${relaxed} infrared boxes on his bore (want 0) · brought up (behind its light) '${upper}': ${pointed}`);
  }
  // no team, no device: the office at night alone
  { const W: any = await quiet(() => standUp()); const g = W.game; g.godMode = true; g.playerInvisible = true; const pl = g.player;
    pl.slot = 'pistol'; pl.holstered = false; pl.nv = true; for (let i = 0; i < 60; i++) W.step(DT);
    const n = g.dynBoxes.filter(isIr).length, st = IRM.irState(g);
    F.verdict(n === 0 && !st.sam && st.men.length === 0 && pl.nvAmount > 0.5, 'the office at night with no team: goggles down, pistol up — no laser (the solo game as it was)', `infrared boxes ${n} · irState sam ${st.sam}, men ${st.men.length} · "${IRM.irSummary(g)}"`); }
  return F.done();
});

// ---------------------------------------------------------------- ir-unseen
interface Play { hash: string; frames: number; irFrames: number; irMateFrames: number; redMateFrames: number; onFoe: number; hunting: number; gogHunting: number; camSeen: number; awMax: number; lightsMin: number; lightsMax: number; unshadowed: number; shots: number; down: number; raw: any }
/** a scenario played through a held door from its own seed: fall in, stack, open and clear, the room's men alive — every frame hashed and its boxes counted */
async function play(id: string, nv: boolean, irOn: boolean): Promise<Play | null> {
  const S: any = await quiet(() => stage(id, 1)); const { g, step } = S; const O = g.orders; IR.on = irOn; teamDrawn(); g.player.nv = nv;
  const team = mates(g), foes = g.guards.filter((x: any) => x.team === 'hostile'); const H = fnv();
  const R: Play = { hash: '', frames: 0, irFrames: 0, irMateFrames: 0, redMateFrames: 0, onFoe: 0, hunting: 0, gogHunting: 0, camSeen: 0, awMax: 0, lightsMin: Infinity, lightsMax: 0, unshadowed: 0, shots: 0, down: 0, raw: null };
  const shots0 = foes.reduce((n: number, x: any) => n + x.weapon.shotsFired, 0);
  const frame = () => { step(); R.frames++;
    H.eat(JSON.stringify(digestGame(g, true)));
    for (const x of g.guards) H.eat(`${x.callsign}:${x.state}:${x.awareness.toFixed(5)}:${x.flashlight?.enabled ? 1 : 0}:${x.char.pos[0].toFixed(4)},${x.char.pos[2].toFixed(4)},${x.char.aimYaw.toFixed(4)}:${(x.irradiance ?? 0).toFixed(5)}:${(x.visibility ?? 0).toFixed(5)}|`);
    for (const c of g.devices?.cameras ?? []) { H.eat(`cam:${c.seen.toFixed(5)}:${c.state}|`); R.camSeen = Math.max(R.camSeen, c.seen); }
    const L = g.engine.lights.lights; R.lightsMin = Math.min(R.lightsMin, L.length); R.lightsMax = Math.max(R.lightsMax, L.length);
    for (const l of L) H.eat(`${l.name}:${l.kind}:${l.enabled ? 1 : 0}:${l.intensity.toFixed(4)}:${l.pos[0].toFixed(3)},${l.pos[1].toFixed(3)},${l.pos[2].toFixed(3)}|`);
    H.eat(`sam:${g.player.visibility.toFixed(5)}:${(g.player.irradiance ?? 0).toFixed(5)}|`);
    for (const x of foes) if (x.state !== 'dead') R.awMax = Math.max(R.awMax, x.awareness);
    const lasers = g.dynBoxes.filter((b: any) => isIr(b) || isRed(b)); R.unshadowed += lasers.filter((b: any) => !noShadow(b)).length;
    if (lasers.some(isIr)) R.irFrames++;
    let irMate = false, redMate = false;
    for (const x of team) { if (x.state === 'dead') continue; const gog = NVM.nvgDown(g, x), armed = gog ? x.char.anim.upper === 'aim' && !x.muzzleDown && !x.held : (x.state === 'alert' || x.state === 'search') && !x.muzzleDown && !x.held; if (!armed) continue; R.hunting++;   // (the frames his laser is on with IR on — asked of the man, not of the switch: the IR-off runs count the same frames)
      if (gog) R.gogHunting++;
      if (lasers.some((b: any) => isIr(b) && onBore(x.char, b))) irMate = true; if (lasers.some((b: any) => isRed(b) && onBore(x.char, b))) redMate = true;
      if (gog) { const reach = CBM.laserReach(g, x.char, x); if (reach) for (const h of foes) if (h.state !== 'dead' && Math.abs(h.char.rayHit(reach.from, reach.dir, reach.L + 0.05) - reach.L) < 0.02) { R.onFoe++; break; } } }
    if (irMate) R.irMateFrames++; if (redMate) R.redMateFrames++;
  };
  const sam = g.player.char.pos; await quiet(() => O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all')); for (let f = 0; f < 120; f++) frame();
  let fc: any = null, why: string | void = 'no door';
  for (const e of doorsOf(S)) { fc = e.door.frameCentre; why = await quiet(() => O.give('stack', O.targetAt(fc), 'all')); if (!why) break; }
  if (why) return null;
  for (let f = 0; f < 60 * 14; f++) frame();
  await quiet(() => O.give('openClear', O.targetAt(fc), 'all')); for (let f = 0; f < 60 * 30; f++) frame();
  R.hash = H.hex(); R.shots = foes.reduce((n: number, x: any) => n + x.weapon.shotsFired, 0) - shots0; R.down = foes.filter((x: any) => x.state === 'dead').length; R.raw = digestGame(g, true);
  reset(); return R;
}

// ---------------------------------------------------------------- ir-off

// ---------------------------------------------------------------- the team's guns show no laser line (IR.teamOff)

// ---------------------------------------------------------------- digest (the cross-commit identity: run this file's `digest` here and in a checkout of the merged base)
/** one hash per scenario of a scripted minute with IR.on false (or with no irlaser.ts at all): fall in, stack, open and clear with the room's men alive, Sam's
 *  goggles down for the middle of it — the game's digest, every man's place and light, and EVERY BOX the frame hands the renderer */
export async function digestLines(): Promise<string[]> {
  const out: string[] = [], sights0 = RFM.ROOMFIGHT.sights, buddy0 = DTM.DOORTOOLS.buddy, other0 = (DTM.DOORTOOLS as any).otherDoorM, cues0 = CUM?.CUES.on, call0 = BDM?.BUDDY.on, door0 = DWM?.DOORMANNERS.on;
  if (DWM) DWM.DOORMANNERS.on = false;   // (before the first staging: stage() steps frames of its own)
  const CVM: any = await import(`${ROOT}/src/game/cover.ts`).catch(() => null), keep0 = CVM?.COVER.backKeep; if (CVM && 'backKeep' in CVM.COVER) CVM.COVER.backKeep = false;   // (a later default-on rule of the cover's: off for the whole digest, put back at its end)
  const brace0 = DGM?.DIGIN.braceInRoom, fbIn0 = RFM.ROOMFIGHT.fallBackIn; if (DGM && 'braceInRoom' in DGM.DIGIN) DGM.DIGIN.braceInRoom = false;   // (the brace goal rule: a later default-on rule, off for the whole digest, put back at its end)
  for (const id of IDS) {
    const S: any = await quiet(() => stage(id, 1)); const { g, step } = S; const O = g.orders; IR.on = false; teamDrawn(); if (ILM) ILM.IRLIGHT.on = false; if (CUM) CUM.CUES.on = false; if ('litMan' in GDM.LIGHTS) GDM.LIGHTS.litMan = false; if ('wallOff' in GDM.LIGHTS) GDM.LIGHTS.wallOff = false; if ('sights' in RFM.ROOMFIGHT) RFM.ROOMFIGHT.sights = false; if ('workOut' in O.knobs) O.knobs.workOut = false; if ('hallFile' in O.knobs) O.knobs.hallFile = false; if ('secondFloor' in O.knobs) O.knobs.secondFloor = false; if ('cleared' in O.knobs) Object.assign(O.knobs, { cleared: false, knownFace: false, ownDoor: false }); if ('roundCap' in O.knobs) O.knobs.roundCap = 0; if ('fileHold' in O.knobs) O.knobs.fileHold = false; if ('asideClear' in O.knobs) O.knobs.asideClear = false; if ('fileQueue' in O.knobs) O.knobs.fileQueue = false; if ('propRound' in O.knobs) O.knobs.propRound = false; if ('backEnd' in O.knobs) O.knobs.backEnd = false; if ('inWay' in O.knobs) O.knobs.inWay = false; if ('walkTries' in O.knobs) O.knobs.walkTries = false; if ('sameWay' in O.knobs) O.knobs.sameWay = false; if ('trailDoors' in O.knobs) O.knobs.trailDoors = false; if ('buddy' in DTM.DOORTOOLS) DTM.DOORTOOLS.buddy = false; if ('otherDoorM' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).otherDoorM = 0; if ('ownCan' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).ownCan = false; if ('behindSam' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).behindSam = false; if ('tossMate' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).tossMate = false; if ('haltBack' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).haltBack = false; if ('thrHand' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).thrHand = false; for (const k of ['thrNoCan', 'thrFragHold', 'fragHold', 'thrDoorsLeaf']) if (k in DTM.DOORTOOLS) (DTM.DOORTOOLS as any)[k] = false; if (BDM) BDM.BUDDY.on = false; if (SLM) SLM.SILENT.on = false; if (DWM) DWM.DOORMANNERS.on = false; if (RWM) { RWM.REARWATCH.on = false; RWM.REARWATCH.room = false; } if ('fallBackIn' in RFM.ROOMFIGHT) RFM.ROOMFIGHT.fallBackIn = false; if ('coverKeep' in RFM.ROOMFIGHT) RFM.ROOMFIGHT.coverKeep = false; if ('doorShots' in RFM.ROOMFIGHT) RFM.ROOMFIGHT.doorShots = false; const H = fnv(); let lasers = 0;   // (litMan, sights: later default-on rules, off where the checkout has them so this stays the base's digest; stage() puts them back)
    const frame = () => { step(); H.eat(JSON.stringify(digestGame(g, true)));
      for (const x of g.guards) H.eat(`${x.flashlight?.enabled ? 1 : 0}${x.state[0]}${x.char.pos[0].toFixed(4)},${x.char.pos[2].toFixed(4)},${x.char.aimYaw.toFixed(4)}`);
      for (const b of g.dynBoxes) { H.eat(`${b.c[0].toFixed(4)},${b.c[1].toFixed(4)},${b.c[2].toFixed(4)};${b.h[0].toFixed(4)},${b.h[1].toFixed(4)},${b.h[2].toFixed(4)};${b.flags};${b.emissive ? b.emissive.map((e: number) => e.toFixed(3)).join(',') : ''}|`); if (isRed(b)) lasers++; } };
    const sam = g.player.char.pos; await quiet(() => O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all')); for (let f = 0; f < 120; f++) frame();
    let fc: any = null; for (const e of doorsOf(S)) { fc = e.door.frameCentre; if (!(await quiet(() => O.give('stack', O.targetAt(fc), 'all')))) break; }
    for (let f = 0; f < 60 * 8; f++) frame();
    g.player.nv = true; for (let f = 0; f < 60 * 7; f++) frame();
    await quiet(() => O.give('openClear', O.targetAt(fc), 'all')); for (let f = 0; f < 60 * 20; f++) frame();
    g.player.nv = false; for (let f = 0; f < 60 * 20; f++) frame();
    out.push(`${id}: ${H.hex()} · ${g.guards.filter((x: any) => x.state !== 'dead').length} of ${g.guards.length} standing at ${g.time.toFixed(2)} s · red laser boxes hashed ${lasers}`);
    reset();
  }
  if ('sights' in RFM.ROOMFIGHT) RFM.ROOMFIGHT.sights = sights0; if ('buddy' in DTM.DOORTOOLS) DTM.DOORTOOLS.buddy = buddy0; if ('otherDoorM' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).otherDoorM = other0; if ('ownCan' in DTM.DOORTOOLS) (DTM.DOORTOOLS as any).ownCan = true; if (CUM) CUM.CUES.on = cues0; if (BDM) BDM.BUDDY.on = call0; if (DWM) DWM.DOORMANNERS.on = door0;
  if (DGM && 'braceInRoom' in DGM.DIGIN) DGM.DIGIN.braceInRoom = brace0; if ('fallBackIn' in RFM.ROOMFIGHT) RFM.ROOMFIGHT.fallBackIn = fbIn0;
  if (CVM && 'backKeep' in CVM.COVER) CVM.COVER.backKeep = keep0;
  return out;
}
if (import.meta.main && process.argv[2] === 'digest') { for (const l of await digestLines()) console.log(l); process.exit(0); }
