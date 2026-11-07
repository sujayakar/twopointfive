// DROPS IT AND RUNS — THE SECOND SLICE OF THE ENEMY'S MORALE (the maintainer, 10-13: "let's not add surrendering to our mechanics. instead, enemies drop their
// weapons and run" — and, as of morale.ts, "a lot of the value from this comes with good telegraphing": he SAYS it, then he DOES it, with a beat the player can
// act on). Nobody ever puts his hands up.
//   · WHO: a man morale.ts has BROKEN (its rung 2), who is ALONE, who is DRY — that file's bait burst has run his magazine out and he is mid-change, or his gun is
//     empty by its own count — and who has an enemy plainly in his eyes inside `closeM`. Never on dread alone: the player causes it on purpose — bait the burst,
//     step in during the dry window. Never a civilian, an operative or the player; never a choreographed man, a man of the hostage rescue's cast or of the
//     response team (morale.ts's own list, and qrf.ts owns). OR, with rounds still in his gun: a broken GREEN man who SAW HIS LEADER FALL (morale.ts
//     sawLeaderFall) with an enemy plainly in his eyes inside `closeM` — the same shout, the same beat, the same run. `runnerBreak` starts the same sequence directly (a later slice: a broken man whose leader is
//     killed in his view).
//   · SAY, THEN DO: he shouts it — spoken, not on the net — and holds, low, gun down, for `beatSecs`; then the weapon leaves his hand and lies on the floor where
//     he dropped it (combat.ts dropFromHand: the item a dying hand lets go of), and he runs.
//   · UNARMED FROM THEN ON, and FLED (`gd.fled`): his gun's own count is kept at nothing and no magazine change is ever his again, so no fire site has a
//     round of his; guards.ts inHand says his hand is empty, so killGuard has nothing more to drop; he is drawn with empty hands (combat.ts guardProps: the
//     rifleman's sidearm stays on his thigh, never drawn — with `anyway` it is on the floor beside the rifle and the thigh is empty; the pistol man's holster
//     is empty); his light went with the gun. He perceives nothing and fights
//     nobody: guards.ts runs this file's GuardScript every frame from the shout on, the room fight's defender tick yields (`OWN.runner`), and morale.ts's own
//     sequence is let go of (moraleRelease) and kept let go of.
//   · WHERE HE RUNS, at the guards' own chase pace (Game.tune.guardChase) and no faster: to a living armed MATE first — the nearest BY THE FLOOR, inside
//     `mateM` of path — and inside `tellM` of him with a clear line he shouts it and the mates in earshot (`toldM`) are TOLD through the game's own records:
//     roomfight.ts told (what a loud noise out that way does to a man on a post: the entry toward it gains a point of evidence, so his watch turns to it; the
//     post's noise record reads the place, which the squads' ears and the dig-in read) and digin.ts digInOrder (the last stand's order: he starts on his room
//     after `tellBeat`; it runs for a man wherever DIGIN.on or a squad is staged); a mate with no post of the room fight's (the night staff) has the place put
//     in his head at `tellAware`, the floor's own rules from there. Then he KEEPS GOING, out of the building: the nearest floor he can reach that lies outside
//     the structure's rooms under open sky (roomfight.ts outOfBuilding's test, `clearM` clear of the walls), for choice one no enemy has a line to — on a
//     level with no structure, the rim of the walkable map (`edgeM`). With no mate that near he tells nobody and goes straight out. The way is planned round
//     the enemies he knows of (`shyM`) and looked at again every `replanSecs`: an enemy who steps into the `aheadM` in front of him sends him another way (or
//     to the corner). An enemy at arm's length never holds him where he stands: he may always step AWAY. He waits on no door drill, no lane, nobody's
//     manners in a doorway (guards.ts followPath: those three gates pass a fled man) — a leaf that opens toward him he still has to pull.
//   · WHO HE KNOWS OF (picture.ts PICTURE.run; off = as it was: every living enemy where he truly stands, through any wall): the way, the floor he may
//     reach, the outside he picks, the corner and the wall he turns his face to are all reckoned from HIS OWN PICTURE — the men he has seen, heard fire or
//     been told of, each where he believes him to be, a place he has since seen empty or an idea gone stale left out. A man he knows nothing of bars no
//     way of his until he sees him: then the look ahead sends him another way, as above. What the WORLD decides stays the truth: he is taken off the
//     floor only when no enemy truly has a line to him, and is low while one has — and stopped outside with a man's eyes on him he looks about him and
//     has that man (a line runs both ways), so he goes on to floor that man cannot see. From the drop on he is off his side's net in the picture too
//     (picture.ts onNet), as netquiet.ts and enemysquads.ts have him.
//   · HE RUNS FOR IT NO MATTER WHAT (`anyway`; off = as it was. The maintainer: 'if a guy gives up he should drop his gun (including pistol) and just run
//     for it no matter what'): the way round every enemy he knows of is still his first choice. But the floor he may reach is walled off round every one
//     of them alike — a man in his eyes, a sound he placed in a doorway, a mate's word about a man since shot — so one idea on the only door between him
//     and the outside sent him to the corner, for good (from a recorded game on the block: the fire exit two shut doors away). So when that leaves him NO
//     way out he looks once more with nobody as a wall and runs the way it finds (`Run.bold`): the enemies still dear to the path (route's own cost) and
//     still minded in the pick of the outside floor, and nobody stepping into his way a reason to think again. Only a floor that gives him no way out at
//     all — a locked door — leaves him the corner, and he never settles there: every `retrySecs` he looks again, with no `retries` to run out of. And BOTH guns go: a rifleman's sidearm is put on the floor beside his rifle (`Run.bare`; combat.ts
//     guardKit draws his thigh empty, alive or shot later).
//   · NOBODY'S ENEMY ON THE BOOKS (`nobody`; off = as it was): the readers outside this file that walk the roster ask `runnerNobody` (from the drop: he
//     counts for nothing — fogofwar.ts's 'contact' and 'steps' calls, doortools.ts's 'room clear', morale.ts and
//     darkdoor.ts's company) or `runnerMute` (from the shout: he speaks for nobody — cues.ts, suppression.ts's barks).
//   · GONE: out, and no enemy with a line to him for `goneSecs`, a spawned unit is taken out of the world (guards.ts despawnUnit). A man of the level's own
//     roster is never removed (a checkpoint is poured over that roster): he stays low where he got to. With no way out he goes to the reachable corner
//     farthest from every enemy and stays low there, his face to the wall, silent, still fled — and looks again for a way out every `retrySecs`, until
//     `retries` runs at a way that would not let him through (a door that will not open) have come to nothing: then he stays.
//   · COUNTING: a fled man is nobody's living hostile — every ending that counts them asks `gd.fled`, a scenario's "my cast was taken away" test passes
//     over him, his squad counts him out (enemysquads.ts alive); the team's eyes pass over him (roomfight.ts tickOp, guards.ts acquireFoe, the breach's
//     target queue). He is nobody's calm colleague either (Guard.hold + pinned: no chat, no need, no greeting, no errand is his). A grenade he was winding up
//     stays on his belt (frag.ts). The player may shoot him: it is a kill like any other, and nothing here scores or answers it.
// SHOW, NOT TELL: nothing is drawn or written for the player but the man himself. One die (simRandom), no wall clock, no graphics setting; the clocks are seconds
// COUNTED and ride a checkpoint with the rest (runnerSnapshot / Restore).
// The switch is off in the module and on the frozen stand, and acts only with MORALE.on: every seam returns on its first line, no man is met, no die is drawn.
// Type-only game imports but for the functions below, used inside calls only (src/game is one import cycle).
import './seats';
import { SideMap } from '../math/side';
import { claimMan, releaseMan, ownerWitness, hasHim, OWN } from './owner';
import type { Game, Guard, GuardScript } from './game';
import type { Character } from './character';
import { Vec3, v3 } from '../math/vec';
import { simRandom } from '../math/dice';
import './guards';
import './combat';
import { isHostile } from './teams';
import { MORALE, moraleOf, rungOf, moraleRelease, sawLeaderFall } from './morale';
import './digin';
import { w } from './words';
import { PICTURE, ideasOf, type Who } from './picture';
import { RUST_PASS, RUST_SURFACE } from './crossing';

/** the switch and its numbers: off in the module, raised by the page (host/profile.ts pageSwitches; ?norunner = a broken man stays and changes his magazine, as
 *  he did) and by the shipped stand; the harness's frozen stand leaves it off (TPS_RUNNER=1 turns it on — and it needs TPS_MORALE=1 beside it). Metres:
 *  `closeM` the enemy in his eyes, `tellM` the mate he runs in on, `toldM` the mates who hear it, `shyM` how wide of an enemy his way is planned, `mateM` the
 *  longest way to a mate worth running, `clearM` how far clear of the building's walls counts as out, `edgeM` the rim of a map with no building, `cornerM`
 *  how far the corner may be. Seconds: `beatSecs` shout to drop, `tellBeat` the told men's own beat, `goneSecs` out and unseen, `mateSecs` the most he spends
 *  getting to a mate, `stallSecs` going nowhere before he thinks again, `retrySecs` between a
 *  cornered man's looks for a way out that has come free (`retries` = the runs at a way that would not let him through before he stays where he is for
 *  good), `replanSecs` between his looks at the `aheadM` of his way in front of him for an enemy who has stepped into it. `mateM` is by the FLOOR (the
 *  path's own length). `askCells` / `askWide` = how many outside cells (a metre's grid) are asked for one no enemy has a line to — on the first look, and
 *  on the wider looks of a man who is out with eyes still on him. `tellAware` = the awareness a told man of no post is left at. `anyway` (off in the
 *  module, raised by the page; ?runanyway=0): with no way out round the enemies he knows of he runs for the outside regardless, and a rifleman's sidearm
 *  is dropped with his rifle. UNTUNED by eye (nobody
 *  has seen it on a GPU, nobody has played it). */
export const RUNNER = { on: false, closeM: 5, outSecs: 0.8, beatSecs: 0.6, tellM: 4, toldM: 8, tellBeat: 0.8, tellAware: 0.6, shyM: 2, mateM: 20, mateSecs: 10, clearM: 1.5, edgeM: 2, cornerM: 25, goneSecs: 2, stallSecs: 3, retrySecs: 2, retries: 3, replanSecs: 0.5, aheadM: 3, askCells: 60, askWide: 240, anyway: false, nobody: false };
export const RUNNER_STATS = { said: 0, dropped: 0, toMates: 0, told: 0, out: 0, gone: 0, low: 0, bold: 0 };

/** where a man's run stands: the shout's beat (gun still in his hand) · to a mate · out of the building · low (the corner, or a roster man who got out) · gone */
export type RunPhase = 'say' | 'mates' | 'out' | 'low' | 'gone';
const PHASES: RunPhase[] = ['say', 'mates', 'out', 'low', 'gone'];
/** per man: the phase and its seconds `t`, where the enemy stood when he broke (`from`: what he tells them), where
 *  he is going, the mate he runs to, how many men he told, seconds out with no enemy's line on him, the stall watch (`at` / `stall`), whether he is out.
 *  `tries` = runs at a way out that would not let him through, `plan` = seconds since he last looked at the way ahead, `face` = where a man low in his
 *  corner looks (the wall: away from the nearest enemy). `bold` = the way he is on was found with nobody as a wall (`anyway`), `bare` = his sidearm went to the floor too. `drive` = this file's script has
 *  him this frame (stamped `driveT`). */
interface Run { phase: RunPhase; t: number; from: Vec3; goal: Vec3 | null; mate: Guard | null; told: number; unseen: number; at: Vec3; stall: number; isOut: boolean;
  tries: number; plan: number; face: number | null; bold: boolean; bare: boolean; drive: boolean; driveT: number; script: GuardScript; }
const stateOf = new SideMap<Guard, Run>('runner.stateOf');
ownerWitness('runner', gd => RUNNER.on && stateOf.has(gd));

/** THE LINES (placeholders in the register of the defenders' own — the writing pass is the maintainer's). done: shouted at the enemy in front of him, the
 *  beat before the gun goes down. mates: shouted to his own as he runs in on them — `he` as the night staff have it (their intruder is one man), `they` a
 *  scenario's defender under the dialogue pass (words.ts). */
type LineKind = 'done' | 'mates';
const LINES: Record<LineKind, { say: string[]; he?: string[]; they?: string[] }> = {
  done: { say: ['no. no, I\'m done. I\'m done!', 'that\'s it — I\'m done! I\'m done!', 'no more! I\'m finished here!'] },
  mates: { say: ['don\'t shoot — it\'s me! it\'s me!'], he: ['don\'t shoot, it\'s me! he\'s right behind me!', 'it\'s me! hold your fire — he\'s coming!'],
    they: ['don\'t shoot, it\'s me! they\'re right behind me!', 'it\'s me! hold your fire — they\'re coming!'] },
};
/** this file's lines — every kind's, or one kind's (the check: a line said came from the right pool; the words lint's bubble length and blocklist) */
export function runnerLines(kind?: LineKind): string[] { return (kind ? [LINES[kind]] : Object.values(LINES)).flatMap(l => [...l.say, ...(l.he ?? []), ...(l.they ?? [])]); }
/** one line of a kind from this man, shouted (never on the net: the player hears it where he stands) */
function bark(g: Game, gd: Guard, kind: LineKind) {
  const L = LINES[kind], pool = [...L.say, ...((gd.team !== 'guard' ? w(L.they, L.he) : L.he) ?? [])];   // (words.ts: 'they' is a scenario's defender under the pass — off, every man's intruder is 'he', as it was)
  g.say(gd, pool[Math.floor(simRandom() * pool.length) % pool.length], false, true);
}

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const headOf = (c: Character): Vec3 => c.bones.head ?? [c.pos[0], c.pos[1] + 1.6, c.pos[2]];
/** a man this file is about at all (morale.ts's own): a living defender or a man of the night staff — never a civilian, an operative, a body */
const ours = (gd: Guard) => gd.state !== 'dead' && !gd.civ && (gd.team === 'hostile' || gd.team === 'guard');
/** nobody's to take: the hostage rescue's cast (rescue.ts) and the response team (qrf.ts) */
const spoken = (g: Game, gd: Guard) => (!!g.rescue?.active && g.rescue.hostiles.includes(gd)) || !!g.qrf?.owns(gd);
/** may the sequence start on him this frame (morale.ts reacts): the AI on, no show on the floor, free of Sam's arm, nobody's choreography but a room fight's walk */
function reacts(g: Game, gd: Guard): boolean {
  if (!g.aiEnabled || g.touring || g.quietUtility || gd.held) return false;
  return !gd.script || !!g.fight?.defs.some(D => D.gd === gd);
}
/** an enemy as a run reckons with him: where he is on the floor, the eye over it, and who he is */
interface Foe { pos: Vec3; eye: Vec3; who: Who }
const foeAt = (c: Character, who: Who): Foe => ({ pos: c.pos, eye: headOf(c), who });
/** his enemies on the floor: the player (the one there is, asked as a side) and every living man of a side hostile to his */
function foes(g: Game, gd: Guard): Foe[] {
  const out: Foe[] = []; if (isHostile(gd.team, 'player') && !g.player.down) out.push(foeAt(g.player.char, 'player'));
  for (const o of g.guards) if (o.state !== 'dead' && isHostile(gd.team, o.team)) out.push(foeAt(o.char, o));
  return out;
}
/** is a run planned from the man's own picture (PICTURE.run; a show keeps no pictures) */
const byPicture = (g: Game) => PICTURE.on && PICTURE.run && !g.touring;
/** THE ENEMIES A RUN IS PLANNED ROUND: as it was, every one of them where he truly stands (`foes`); with PICTURE.run, the men HE knows of, each where he
 *  believes him to be — every idea his picture still holds (it forgets at `forgetSecs`: a man low in his corner with his face to the wall does not take
 *  the man who cornered him for gone the moment the idea is no longer fresh) but a place he has since seen empty */
function known(g: Game, gd: Guard): Foe[] {
  if (!byPicture(g)) return foes(g, gd);
  const out: Foe[] = []; for (const I of ideasOf(g, gd)) if (!I.lost && g.time - I.t < PICTURE.forgetSecs) out.push({ pos: I.at, eye: [I.at[0], PICTURE.eyeH, I.at[2]], who: I.who });
  return out;
}
/** …OR THE MAN HIS LEADER'S FALL LEFT (morale.ts): broken, GREEN (the training he broke with), and he SAW his leader go down (sawLeaderFall: seconds since,
 *  Infinity for a man who never did) — rounds in his gun or not, mates about him or not. The enemy in his eyes is the caller's to ask, as above */
function lost(g: Game, gd: Guard): boolean {
  if (!ours(gd) || spoken(g, gd) || !reacts(g, gd) || rungOf(gd) !== 2) return false;
  return !!moraleOf(gd)?.green && Number.isFinite(sawLeaderFall(gd));
}
/** the shout: from here his body is this file's, and morale.ts's sequence is over */
function start(g: Game, gd: Guard, from: Vec3) {
  const c = gd.char;
  claimMan(gd, OWN.runner, g.time); stateOf.set(gd, { phase: 'say', t: 0, from: [from[0], 0, from[2]], goal: null, mate: null, told: 0, unseen: 0, at: v3.copy(c.pos), stall: 0, isOut: false, tries: 0, plan: 0, face: null, bold: false, bare: false, drive: false, driveT: -1, script: { goal: null } });
  moraleRelease(g, gd); gd.reloadT = -1; c.anim.cancelReload(); gd.weapon.burst = null;   // (the magazine change he was in ends with it: the count and the clip both)
  bark(g, gd, 'done'); RUNNER_STATS.said++;
}
/** A LATER SLICE'S SEAM: start the sequence on this man directly (a broken man whose leader is killed in his view), whatever his magazine holds and whoever
 *  is near. False = the switch is off, he is not a man of this file's, or it already has him */
export function runnerBreak(g: Game, gd: Guard): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('runnerBreak', undefined, [g, gd]); if (__r !== RUST_PASS) return __r as any; } }
  if (!RUNNER.on || !MORALE.on || !ours(gd) || spoken(g, gd) || !reacts(g, gd) || stateOf.has(gd)) return false;
  let from: Vec3 = gd.lastKnown ?? gd.char.pos, bd = Infinity;
  for (const e of known(g, gd)) { const d = flat(e.pos, gd.char.pos); if (d < bd) { bd = d; from = e.pos; } }
  start(g, gd, from); return true;
}

/** the rectangle round every room of the level's structure [x0, x1, z0, z1], worked out once a level (null = the level has no structure) */
const rects = new SideMap<object, [number, number, number, number]>('runner.rects');



/** …AND HIS SIDEARM WITH IT (`anyway`): a rifleman who put the pistol on his thigh down beside his rifle — combat.ts guardKit draws the thigh empty */
export function runnerBare(gd: Guard): boolean { return RUNNER.on && !!stateOf.get(gd)?.bare; }
/** A MAN WHO HAS RUN IS NOBODY'S ENEMY ON THE BOOKS (`nobody`; off = as it was, false here). His own script runs and nothing else of his does, but every
 *  module that walks the roster from outside still met a calm, living man in state 'patrol': a teammate called 'contact' on him, a room was not called clear
 *  with him in it, a mate beside him was 'not alone', his own side's cues picked him to speak. Two questions for those readers —
 *  COUNTS FOR NOTHING, from the drop: nobody's contact, nobody's company, no reason a floor is not clear */
export function runnerNobody(gd: Guard): boolean { return RUNNER.nobody && gd.fled; }
/** …and SPEAKS FOR NOBODY, from the shout: no cue of his side's, no bark under fire (his own lines are this file's) */
export function runnerMute(gd: Guard): boolean { return RUNNER.nobody && hasHim(gd, OWN.runner); }
/** QA: what the rule has of a man (null = never taken) */
export function runnerOf(gd: Guard): Readonly<Omit<Run, 'script'>> | null { return stateOf.get(gd) ?? null; }
/** 'runner ▸ Tango 2 out (told 2) · Tango 5 say' — every man it has on the floor; '' with the switch off */
export function runnerSummary(g: Game): string {
  if (!RUNNER.on) return '';
  const men = g.guards.filter(gd => stateOf.has(gd)).map(gd => { const S = stateOf.get(gd)!; return `${gd.callsign} ${gd.state === 'dead' ? 'shot' : S.phase}${S.told ? ` (told ${S.told})` : ''}`; });
  return 'runner ▸ ' + (men.join(' · ') || 'nobody has run') + (RUNNER_STATS.gone ? ` · ${RUNNER_STATS.gone} gone` : '');
}

// ---------------------------------------------------------------- checkpoints (game/savestate.ts): each roster man's run by roster index (a spawned unit is never in a checkpoint)
export interface RunnerSnap { men: (number[] | null)[] }
/** nothing (no key in the file) with the switch off or before anybody has run */
export function runnerSnapshot(g: Game): RunnerSnap | undefined {
  if (!RUNNER.on) return undefined;
  const men = g.guards.map(gd => { const S = stateOf.get(gd); return S ? [PHASES.indexOf(S.phase), S.t, gd.fled ? 1 : 0, S.told, S.unseen, S.isOut ? 1 : 0, S.mate ? g.guards.indexOf(S.mate) : -1, S.from[0], S.from[1], S.from[2], ...(S.goal ?? [NaN, NaN, NaN]), S.tries, S.bold ? 1 : 0, S.bare ? 1 : 0] : null; });
  return men.some(m => m) ? { men } : undefined;
}
/** …onto the fresh roster (a file without the slice: nobody has run). A man mid-run thinks again from where he stands on his first frame */
export function runnerRestore(g: Game, s: RunnerSnap | null | undefined) {
  const f = (x: unknown, d = 0) => typeof x === 'number' && Number.isFinite(x) ? x : d;
  g.guards.forEach((gd, i) => { const M = s?.men?.[i];
    if (!M) { stateOf.delete(gd); gd.fled = false; releaseMan(gd, OWN.runner, g.time); return; }
    const goal: Vec3 | null = Number.isFinite(M[10]) ? [M[10], f(M[11]), f(M[12])] : null;
    gd.fled = !!M[2]; claimMan(gd, OWN.runner, g.time, gd.fled ? 1 : 0); stateOf.set(gd, { phase: PHASES[f(M[0])] ?? 'low', t: Math.max(0, f(M[1])), from: [f(M[7]), f(M[8]), f(M[9])], goal, mate: g.guards[f(M[6], -1)] ?? null, told: f(M[3]), unseen: Math.max(0, f(M[4])), at: v3.copy(gd.char.pos), stall: 0, isOut: !!M[5],
      tries: Math.max(0, f(M[13])), plan: 0, face: null, bold: !!M[14], bare: !!M[15], drive: false, driveT: -1, script: { goal: null } }); });
}
