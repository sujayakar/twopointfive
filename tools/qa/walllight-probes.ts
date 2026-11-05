// The 'light-wall' probe: A TEAMMATE'S LIGHT IS DARK INTO THE WALL AT HIS MUZZLE (guards.ts LIGHTS.wallOff — wallDark / wallDarkSeen). Run through probes.ts:
//   bun run tools/qa/probes.ts light-wall        (family: litman)
// PASS / FAIL each, headless (bang-tally.ts stage()). The squad is given the orders a player gives — fall in, stack on the nearest door, open and clear — with
// the light discipline rule held off and the team bare-eyed, so every weapon light burns as its unit's switch has it; every frame the probe reads each beam
// ITSELF (beam-reading.ts: after the step, ray by ray with the rule's line test, judged only where the beam is plain at a margin either side of the rule's
// two distances — its middle on something that stands and no ray clear; the ground is neither) and keeps its own clock:
//   · with the switch off the flaw is there: frames of a burning light whose whole beam has ended at arm's length for longer than the rule would allow
//   · with it on there is no such frame, and the rule did hold lights dark
//   · it burns again at once: no frame where the rule has him dark and any ray runs clear past the far distance, or the beam's middle is down on open ground
//   · the floor is not a wall, asked of beams laid by hand from where a man could stand, at the rule's two distances and the hand torch's two: a beam
//     plainly into a wall is a wall to the rule, a beam plainly open is not, and one whose middle is on a thing's FOOT is a wall only to a rule that held it
//     a moment ago — and of the open ones, the beams whose every ray would have ended inside the distance with the GROUND counted (a gun at low ready over
//     open floor: walls to the rule as it was) are counted, and must be there
//   · nobody but the team is asked: no hostile man's reading is ever dark
//   · off, nothing reads the rule's numbers: the same game frame for frame with them moved
//   · the stand's own word, and a recording carries the key
// What it cannot say: what any of it looks like (never seen on a GPU), or whether it helps a fight — that is playthrough.ts's tally.
// The probe's own red path: WALLLIGHT_BREAK=1 bun run tools/qa/probes.ts light-wall must FAIL (the rule answers "not dark" with its switch on).
// …and the floor's: WALLFLOOR_BREAK=1 must FAIL the line of beams laid by hand on both stands (the rule counts the ground as something solid, as it did
// before 11-04) — and 'it burns again at once' wherever a play has a man at low ready beside a wall (the page's stand).
import { ROOT } from './headless.ts';
import { stage } from './bang-tally.ts';
import { BEAM_MARGIN as MARGIN, plainWall, plainOpen, plainFoot, type Beam } from './beam-reading.ts';
const GDM: any = await import(`${ROOT}/src/game/guards.ts`);
const LTM: any = await import(`${ROOT}/src/game/lighting.ts`);
const NVM: any = await import(`${ROOT}/src/game/nvg.ts`).catch(() => null);
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const LIGHTS = GDM.LIGHTS;
const IDS: string[] = (SCENARIOS as any[]).map(e => e.id);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const doorsOf = (S: any): any[] => { const sam = S.g.player.char.pos; const c: { ent: any; dist: number }[] = []; for (const d of S.defs) for (const e of d.room.entries) if (!c.some(q => q.ent === e)) c.push({ ent: e, dist: flat(sam, e.at) }); c.sort((a, b) => a.dist - b.dist); return c.map(q => q.ent); };
function fnv() { let h = 0x811c9dc5 >>> 0; return { eat: (s: string) => { for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193) >>> 0; } }, hex: () => (h >>> 0).toString(16).padStart(8, '0') }; }
/** the game's beam geometry in this game (beam-reading.ts reads beams with it) */
const beamOf = (g: any): Beam => ({ fan: GDM.beamFan, ray: (o, r, m) => GDM.beamRayEnd(g, o, r, m) });
/** the probe's own reading of a man's beam as the step left it, from where his light will burn: plainly into a wall inside the rule's near distance ·
 *  plainly open at its far one */
const intoWall = (g: any, x: any): boolean => { const o = GDM.beamLens(g, x); return !!o && plainWall(beamOf(g), o, x.flashlight.dir, LIGHTS.wallM); };
const openBeam = (g: any, x: any): boolean => { const o = GDM.beamLens(g, x); return !!o && plainOpen(beamOf(g), o, x.flashlight.dir, LIGHTS.wallBackM); };
/** BEAMS LAID BY HAND from where a man could stand, round where the team stands (the walkable spots of a metre lattice three metres each way; a lens at the
 *  hip and at the shoulder; level, at low ready and steeper; eight headings), each asked at distance `m` of the rule (guards.ts beamWalledAt, as it starts
 *  and as it holds) and read plain or not: `wall` = plainly into a wall (`wallOpen`: no wall to the rule, starting or holding) · `open` = plainly open
 *  (`openWall`: a wall to the rule, either way) · `foot` = its middle on a thing's foot (`footOdd`: a wall to start on, or none to a rule that held it) ·
 *  `ground` = the open ones whose every ray, flown straight, ends inside m with the ground counted as something solid: the beams the rule called walls
 *  before the floor stopped being one */
function laid(g: any, m: number): { n: number; wall: number; wallOpen: number; open: number; openWall: number; foot: number; footOdd: number; ground: number } {
  const T = { n: 0, wall: 0, wallOpen: 0, open: 0, openWall: 0, foot: 0, footOdd: 0, ground: 0 }, seen = new Set<string>(), B = beamOf(g);
  for (const x of g.guards) { if (x.team !== 'operative' || !x.unit) continue; const p = x.char.pos;
    for (let i = -3; i <= 3; i++) for (let j = -3; j <= 3; j++) { const sx = Math.round(p[0]) + i, sz = Math.round(p[2]) + j, key = `${sx},${sz}`; if (seen.has(key) || g.col.nav.isBlocked(sx, sz)) continue; seen.add(key);
      for (const y of [0.9, 1.3]) for (const dy of [0, -0.65, -0.87]) for (let a = 0; a < 8; a++) {
        const hl = Math.sqrt(1 - dy * dy), o = [sx, y, sz], d = [Math.sin(a * Math.PI / 4) * hl, dy, Math.cos(a * Math.PI / 4) * hl], starts = !!GDM.beamWalledAt(g, o, d, m, false), holds = !!GDM.beamWalledAt(g, o, d, m, true); T.n++;
        if (plainWall(B, o, d, m)) { T.wall++; if (!starts || !holds) T.wallOpen++; }
        else if (plainFoot(B, o, d, m)) { T.foot++; if (starts || !holds) T.footOdd++; }
        else if (plainOpen(B, o, d, m)) { T.open++; if (starts || holds) T.openWall++;
          if (B.fan(d).every(r => g.col.segmentBlocked(o, [o[0] + r[0] * m, o[1] + r[1] * m, o[2] + r[2] * m]))) T.ground++; } } } }
  return T;
}
/** somebody who is not of his side stands in his beam inside `m` (+ `slack`): the rule's exemption, read by the probe at a margin either way */
const manIn = (g: any, x: any, m: number, cos: number): boolean => { const o = x.flashlight.pos, d = x.flashlight.dir, hl = Math.hypot(d[0], d[2]) || 1;
  return g.guards.some((y: any) => { if (y === x || y.state === 'dead' || y.team === 'operative') return false; const vx = y.char.posX - o[0], vz = y.char.posZ - o[2], l = Math.hypot(vx, vz); return l < m && (l < 0.3 || (vx * d[0] + vz * d[2]) / (l * hl) > cos); }); };

interface Tally { darkOnMan: number; manNear: number; litNear: number; lit: number; dark: number; darkClear: number; foeDark: number; foeNear: number; hash: string; frames: number }
/** one scripted stretch: fall in, stack on the nearest door, open and clear — the team bare-eyed and the discipline rule off, so the lights burn */
async function play(id: string, on: boolean, stood = false): Promise<Tally> {
  const S: any = await quiet(() => stage(id, 1)); const { g, step } = S, O = g.orders; LIGHTS.wallOff = on; if (!stood) { LIGHTS.on = false; if (NVM?.NVG) NVM.NVG.on = false; g.player.nv = false; }   // (stood: the light discipline rule and the goggles as the stand has them — a dark light in a dim place takes his goggles down, nvg.ts)
  const T: Tally = { darkOnMan: 0, manNear: 0, litNear: 0, lit: 0, dark: 0, darkClear: 0, foeDark: 0, foeNear: 0, hash: '', frames: 0 }, H = fnv(), near = new Map<any, number>();
  const frame = () => { step(); T.frames++; H.eat(JSON.stringify(digestGame(g, true)));
    for (const x of g.guards) { if (x.state === 'dead' || !x.flashlight) continue; const mate = x.team === 'operative' && !!x.unit; H.eat(`${x.flashlight.enabled ? 1 : 0}`);
      if (!mate) { if (GDM.wallDarkSeen(g, x)) T.foeDark++; if (x.flashlight.enabled && intoWall(g, x)) T.foeNear++; continue; }
      const walled = intoWall(g, x);   // (read once a man a frame)
      const n = walled && !manIn(g, x, LIGHTS.wallM + 1, 0.8) ? (near.get(x) ?? 0) + 1 : 0; near.set(x, n);   // (nobody anywhere near his beam: the frames the rule owes dark)
      if (walled && manIn(g, x, LIGHTS.wallM - MARGIN, 0.95)) { T.manNear++; if (GDM.wallDarkSeen(g, x)) T.darkOnMan++; }   // (a man plainly in a beam that ends at arm's length: the frames it must burn)
      if (x.flashlight.enabled) { T.lit++; if (n > (LIGHTS.wallSecs + 0.2) * 60) T.litNear++; }
      if (GDM.wallDarkSeen(g, x)) { T.dark++; if (openBeam(g, x)) T.darkClear++; } } };
  const sam = g.player.char.pos; await quiet(() => O.give('fallIn', O.targetAt([sam[0], 0, sam[2]]), 'all')); for (let f = 0; f < 120; f++) frame();
  let fc: any = null; for (const e of doorsOf(S)) { fc = e.door.frameCentre; if (!(await quiet(() => O.give('stack', O.targetAt(fc), 'all')))) break; } for (let f = 0; f < 60 * 8; f++) frame();
  if (fc) await quiet(() => O.give('openClear', O.targetAt(fc), 'all')); for (let f = 0; f < 60 * 15; f++) frame();
  T.hash = H.hex(); return T;
}

export async function lightWall(): Promise<boolean> {
  const F = family('light-wall: a teammate\'s weapon light is dark while its whole beam ends on something solid at arm\'s length, and burns again the frame it does not');
  const keep = { ...LIGHTS }, keepNv = NVM?.NVG ? NVM.NVG.on : undefined, back = () => { Object.assign(LIGHTS, keep); if (NVM?.NVG) NVM.NVG.on = keepNv; };
  try {
    let stood = 0, stoodOn = 0; const ids = IDS.filter(id => ['night-breach', 'rescue-registry', IDS[0]].includes(id)).slice(0, 3);
    const off: Tally[] = [], on: Tally[] = [], offStood: Tally[] = [], onStood: Tally[] = [];
    for (const id of ids) { stood++; await quiet(() => stage(id, 1)); if (LIGHTS.wallOff) stoodOn++; back(); off.push(await play(id, false)); back(); on.push(await play(id, true)); back(); offStood.push(await play(id, false, true)); back(); onStood.push(await play(id, true, true)); back(); }
    const sum = (A: Tally[], k: keyof Tally) => A.reduce((n, t) => n + (t[k] as number), 0), per = (k: keyof Tally) => ids.map((id, i) => `${id} ${off[i][k]}→${on[i][k]}`).join(' · ');
    F.verdict(sum(off, 'litNear') > 0, 'with the switch off the flaw is there: a light burns on with its whole beam ended at arm\'s length', `man-frames lit, the beam ended inside ${(LIGHTS.wallM - MARGIN).toFixed(2)} m for over ${(LIGHTS.wallSecs + 0.2).toFixed(1)} s: ${sum(off, 'litNear')} of ${sum(off, 'lit')} lit, over ${ids.length} scenarios (${per('litNear')})`);
    F.verdict(sum(on, 'litNear') === 0 && sum(on, 'dark') > 0, 'with it on there is no such frame, and the rule held lights dark', `man-frames lit into the wall ${sum(off, 'litNear')} → ${sum(on, 'litNear')} (want 0) · man-frames the rule had dark: ${sum(on, 'dark')} (${per('dark')})`);
    F.verdict(sum(onStood, 'litNear') === 0 && sum(onStood, 'darkClear') === 0, '… and the same with the light discipline rule and the goggles as the stand has them', `man-frames lit into the wall ${sum(offStood, 'litNear')} → ${sum(onStood, 'litNear')} (want 0) · lit at all ${sum(offStood, 'lit')} → ${sum(onStood, 'lit')} · the rule had dark ${sum(onStood, 'dark')}, with a ray clear ${sum(onStood, 'darkClear')} (want 0)`);
    F.verdict(sum(on, 'darkOnMan') + sum(onStood, 'darkOnMan') === 0, 'never dark on a man: with somebody who is not of his side plainly in the beam it burns, wall or no wall', `man-frames with a man in a beam that ends at arm's length: ${sum(on, 'manNear') + sum(onStood, 'manNear')} · of those the rule had dark: ${sum(on, 'darkOnMan') + sum(onStood, 'darkOnMan')} (want 0)`);
    F.verdict(sum(on, 'darkClear') === 0, 'it burns again at once: never dark with a ray running clear past the far distance, or with the beam\'s middle down on open ground', `man-frames the rule had dark with a ray clear to ${(LIGHTS.wallBackM + MARGIN).toFixed(2)} m or the beam's middle on the ground: ${sum(on, 'darkClear')} (want 0)`);
    F.verdict(sum(on, 'foeDark') === 0 && sum(off, 'foeDark') === 0 && sum(onStood, 'foeDark') === 0, 'no reading is kept for a man who is not Sam\'s (the rule is one line: his team and his unit)', `readings dark on such a man: ${sum(on, 'foeDark') + sum(onStood, 'foeDark')} · their own lights burning into something at arm's length, left as they are: ${sum(on, 'foeNear')} man-frames`);
    { LIGHTS.wallM = 5; LIGHTS.wallBackM = 6; LIGHTS.wallSecs = 0; const moved = await play(ids[0], false); back();
      F.verdict(moved.hash === off[0].hash && moved.frames === off[0].frames, 'off, nothing reads it: the same game frame for frame with its numbers moved', `${ids[0]}: ${off[0].hash} · numbers moved ${moved.hash} (${moved.frames} frames)`); }
    { const shipped = process.env.TPS_PROFILE === 'shipped', word = process.env.TPS_WALLLIGHT, want = word !== undefined ? word === '1' : shipped;
      F.verdict(stoodOn === (want ? stood : 0), 'the stand: on as the page opens, off on the frozen stand (TPS_WALLLIGHT has the say on either)', `${stoodOn} of ${stood} stagings stood up with it on · ${shipped ? 'the page\'s' : 'the frozen'} stand${word !== undefined ? `, TPS_WALLLIGHT=${word}` : ''}`);
      const J: any = await import(`${ROOT}/src/game/journal.ts`), S: any = await quiet(() => stage(ids[0], 1)), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(S.g)));
      { const TORCH: any = (await import(`${ROOT}/src/game/calmtorch.ts`)).TORCH, ms: number[] = [LIGHTS.wallM, LIGHTS.wallBackM, TORCH.wallM, TORCH.wallBackM];
        const L = ms.map(m => laid(S.g, m)), tot = (k: 'wallOpen' | 'openWall' | 'footOdd') => L.reduce((n, t) => n + t[k], 0), each = (k: 'wall' | 'open' | 'foot' | 'ground') => L.map(t => t[k]).join(' / ');
        F.verdict(L.every(t => t.wall > 0) && L[0].foot > 0 && L[0].ground > 0 && L[1].ground > 0 && tot('wallOpen') === 0 && tot('openWall') === 0 && tot('footOdd') === 0, 'the floor is not a wall, asked of beams laid by hand at the rule\'s two distances and the hand torch\'s two: a beam plainly into a wall is a wall to the rule, a beam plainly open is not however much ground it ends on, and a beam whose middle is on a thing\'s foot is one only to a rule that held it', `${ids[0]}: ${L[0].n} beams from where a man could stand, at ${ms.map(m => m.toFixed(1)).join(' / ')} m · plainly into a wall ${each('wall')}, no wall to the rule ${tot('wallOpen')} (want 0) · plainly open ${each('open')}, a wall to the rule ${tot('openWall')} (want 0) · its middle on a foot ${each('foot')}, a wall to start on or none to hold ${tot('footOdd')} (want 0) · of the open ones, every ray ended inside the distance with the ground counted: ${each('ground')}`); }
      LIGHTS.wallOff = true; const own = snap(), old = snap(), none = snap(); delete old.tables.LIGHTS.wallOff; delete none.tables.LIGHTS;
      LIGHTS.wallOff = false; J.applyKnobs(S.g, own); const carried = LIGHTS.wallOff; LIGHTS.wallOff = true; J.applyKnobs(S.g, old); const before = LIGHTS.wallOff; LIGHTS.wallOff = true; J.applyKnobs(S.g, none); const bare = LIGHTS.wallOff;
      F.verdict(carried === true && before === false && bare === false, 'a recording carries it: one made with it on replays with it on, one from before the key (or before the table was carried) with it off', `the key in the file, on: ${carried} · no key: ${before} · no LIGHTS table: ${bare}`); }
    return F.done();
  } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; }
  finally { back(); }
}
