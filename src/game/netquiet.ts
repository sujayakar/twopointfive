// THE NET GOES QUIET — a slice of the enemy's morale (morale.ts; the maintainer, 10-13: "especially for stealth sections that aren't perfect ghost where
// enemies just don't [know] what's going on, more poorly trained ones could get really scared" · "a lot of the value from this comes with good telegraphing" ·
// "we don't want them to feel dumb" · "we should never make it adding HP etc." · "we should generally show, not tell". Approved as: "Dread from unanswered
// radio checks and bodies found. Heard in the calls getting worse, seen in beams whipping to noises. Done-test: the last man never dies standing upright
// without firing.")
//
// It needs MORALE on to do anything: every feed here is morale.ts's `moraleShock` (its cap, its nerve), and every behaviour keys off that file's rungs.
//   · FEEDS (shock, × `steadyK` for a steady man — trainingOf, nothing of this file's):
//       (enemysquads.ts's two reach this file through netquietfeed.ts, a leaf: that file sits too early in the import cycle to import this one)
//       a mate who does not answer a radio check — `unanswered` to the man who asked, `heardIt` to every man who heard the check on the net (the squad's
//         check: enemysquads.ts radioCheck · the night staff's roll call: guards.ts updateRollcall · the last defenders' own check: laststand.ts tickCheck — a
//         man too busy to answer is a man who did not answer, and so is a dead one);
//       a body found — `body` to the finder, `bodyTold` to the men he tells (enemysquads.ts event 'found' · guards.ts, the staff's body in view);
//       a mate seen to fall — `fallUnseen` with no enemy in the watcher's view, `fallSeen` with one (combat.ts killGuard, beside moraleManDown: a green man who
//         has just taken that file's leaderShock for the same fall takes only what this one adds over it).
//       ONE DEAD MAN IS ONE SHOCK to one man: the fall he saw, the check nobody answered and the body found pay the largest of the three, not their sum.
//   · THE CALLS GET WORSE, said by the man who asked, on the net, `askBeat` after his check's own line: the first silence a flat repeat of the name (the
//     name the squad's own ask uses); with two checks unanswered "why's nobody answering?"; "somebody talk to me!" only with nobody of his side left on the
//     WHOLE net — every squad's men, not the one he asked on. The finder of a body nobody heard go: "he's down. nobody heard a thing?". The tell first, then
//     the act.
//   · BEAMS WHIP TO NOISES: a jumpy or broken man who HEARS an enemy's noise (his own ears counted it: hearingCheck's gates, nothing of this file's) swings
//     to it `whipShift` after — not after the room fight's own 0.4‥0.9 s —, at `whipTurn` rad/s (broken: `whipTurnBroken`), body and all, and at where HIS
//     EARS put it: the source through open air, the doorway it came round otherwise (the source itself only when it is near that place and in plain view).
//     His fix is never truer than his hearing. `whipSecs` on it (one target `whipHold` at the least; a second noise does not send his eyes back to the door
//     and out again), then the man he was. His lamp follows his aim (guards.ts). A man holding a room has it as what he watches meanwhile (roomfight.ts
//     applyWatch → netQuietLook), so whoever then shows somewhere else catches him turned away: the beat the player can use. Nothing here is asked while he
//     has a man in his eyes.
//   · THE LAST MAN — as far as HE knows it: every man of his squad down and each of them known to him (asked for and silent, seen to fall, found, told of on
//     the net), nobody of his side at his elbow; or nobody of his side left at all AND a check gone unanswered in his hearing (if nobody has asked, he asks
//     the net himself once he has been roused `calmSecs`: "all posts, answer up" — and it is the silence that has him act). At his post, he says his line —
//     his squad's ("nobody left in here but me") or, with nobody on the net, the last one —, and `lastBeat` later stops guarding his room upright: he backs,
//     facing them, to the spot in it where every entry sits inside one `arcDeg` arc (else the narrowest arc there is), goes LOW (half down: `crouch`), on his
//     gun (roused, as any defender who has taken a new post is: roomfight.ts plant), and holds the doors in his EYES — the floor's eye sits round from a
//     man's gun, so his gun rests off the arc's middle by what that takes; no wandering glance. He does not rally to a fight next door meanwhile
//     (roomfight.ts rally, `OWN.lastMan`), nor is he his squad's to send (enemysquads.ts home). `alertSecs` of nothing seen or heard and
//     he is no longer 'alert' — wary (`wary`), still low on the spot; anything he hears has him on his gun again. A living mate at his side for `calmSecs`
//     and he is up, the act dropped, his owner's again. A man morale.ts's own sequence has (`OWN.morale`) is that file's.
//   · A MAN WHO HAS RUN (runner.ts `OWN.runner`: the shout, the drop, the run, gone) is nobody's mate on the net: he hears no check, answers none (asked for by name
//     he is a silence like a dead man's), is not whipped, is never the last man — and to the man he left he is one more gone, once that man knows it. The last
//     man at his act is not stopped at a dark doorway nor called to one (darkdoor.ts, `OWN.lastMan`): his hold wins.
//   No health, aim, skill or reaction table is read or written: where he stands, how low, where he looks — and, low on his spot, the floor's own alert /
//   wary lines (rouse, ease, wary: the only writes to a man's state in this file).
// SHOW, NOT TELL: nothing is drawn or written for the player but the men themselves. One die (simRandom, only for which line of a pool), no wall clock, no
// graphics setting. The switch is off in the module and on the frozen stand: every seam returns on its first line, no man is met, no die is drawn.
// Type-only game imports but for the functions below, used inside calls only (src/game is one import cycle).
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';
import type { RoomFight } from './roomfight';
import { Vec3, clamp, DEG } from '../math/vec';
import { MORALE, moraleShock, trainingOf, sawLeaderFall } from './morale';
import './enemysquads';
import { canSee, canSeeChar } from './guards';
import { isHostile } from './teams';
import './squad';
import { NETFEED, netFeedTake } from './netquietfeed';
import { wg } from './words';   // (a leaf: the dialogue pass's one switch)
import { claimMan, releaseMan, hasHim, ownerWitness, OWN } from './owner';
import { RUST_PASS, RUST_SURFACE, crossToRust, goneWithTick } from './crossing';

type Def = RoomFight['defs'][number];

/** the switch and its numbers: off in the module, raised by the page (host/profile.ts pageSwitches; ?nonetquiet) and by the shipped stand; the harness's frozen
 *  stand leaves it off (TPS_NETQUIET=1 turns it on — with TPS_MORALE=1, without which it does nothing). The shocks are morale.ts dread, 0‥1, before its nerve
 *  and cap: `unanswered` / `heardIt` a mate who does not answer (the asker / a man who heard the check), `body` / `bodyTold` a body found (the finder / the men
 *  told), `fallSeen` / `fallUnseen` a mate seen to fall with / without an enemy in view, all × `steadyK` for a steady man — and ONE dead man is one shock to
 *  one man: the largest of the three, not their sum. Seconds: `askBeat` / `bodyBeat` from the event to the line, `barkSecs` the least between two of one man's
 *  lines from this file (`sayHold` how long a line owed waits for that, or for a man to be out of his eyes, before it is dropped), `shooterSecs` how fresh a sighting makes the shooter 'in view', `whipShift` / `whipSecs` the whip's start and length, `whipHold` the
 *  least a whip stays on one noise, `lastBeat` from the last man's line to his first step, `calmSecs` with nobody in his eyes before he starts (and roused
 *  before he asks the net himself), `alertSecs` with nothing seen or heard before a last man low on his spot stops being 'alert' (he keeps the spot and the
 *  crouch), `giveUp` how long he waits for the room fight to hand him back before the act is dropped. `whipTurn` / `whipTurnBroken` rad/s; metres: `whipM`
 *  the furthest noise he whips to, `whipNearM` how near the place his EARS put a noise its true source must be (with a clear line to it) for him to look at the
 *  source itself, `bodyM` how far he knows a man lying in his view, `mateM` a living mate this near and he is not alone; `arcDeg` the arc every entry should
 *  sit in; `eyeDeg` the half-cone of a man's eyes (the floor's own number — guards.ts GUARD_EYE.halfDeg; the check holds
 *  the two equal) and `eyeSlack` the margin inside it he keeps his doors; `crouch` how far down he goes, 0‥1 — HALF: measured on
 *  the floor's eye, a man fully down on his gun (1) looks 48° round from his aim through a cone 26° either way in plan, so past the near field he loses the
 *  very man he aims at; at 0.5 it is 42° and 47°, at 0.4 41° and 49°: what he aims at stays in his eyes, with a margin;
 *  `wary` the awareness he does not fall under, low on his spot, once he is no longer alert (the floor's 'suspicious' line is 0.3; 0 = he calms right down, as
 *  a man his squad has calmed does); `backSpeed` m/s; `maxM` the furthest he walks for the spot; `doorM` the nearest he sits to a door; `eyeM` his eye that far down; `stayM` how much
 *  better than where he stands a spot must score to be walked to. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it). */
export const NETQUIET = { on: false, unanswered: 0.12, heardIt: 0.06, body: 0.2, bodyTold: 0.08, fallSeen: 0.08, fallUnseen: 0.22, steadyK: 0.6,
  askBeat: 1.4, bodyBeat: 2, barkSecs: 3, sayHold: 4, shooterSecs: 1, bodyM: 10,
  whipShift: 0.12, whipSecs: 1.5, whipHold: 0.8, whipTurn: 10, whipTurnBroken: 14, whipM: 15, whipNearM: 4,
  lastBeat: 1.5, calmSecs: 2, alertSecs: 15, giveUp: 8, mateM: 5, arcDeg: 40, eyeDeg: 58, eyeSlack: 4, crouch: 0.4, wary: 0.35, backSpeed: 2, maxM: 9, doorM: 2, eyeM: 1.3, stayM: 1 };
export const NETQUIET_STATS = { asks: 0, quiet: 0, bodies: 0, falls: 0, unseen: 0, lines: 0, whips: 0, last: 0, backs: 0, low: 0, selfAsks: 0, released: 0, eased: 0,
  squadAsks: 0, squadBodies: 0, standChecks: 0, staffAsks: 0, staffBodies: 0 };   // (…and by seam: enemysquads.ts's two, laststand.ts's, guards.ts's two)
/** every line this file has had a man say (the newest 64): who, when, which kind — what the check reads */
export const NETQUIET_LOG: { id: number; t: number; kind: LineKind; text: string; radio: boolean }[] = [];
/** the check's breakage (tools/qa/netquiet-check.ts, NETQUIET_BREAK=1): every feed returns on its first line (nobody asked for, no body, no fall — so no
 *  shock, no line, no man who knows he is alone), the last man's spot is never looked for and the room fight's watch seam (netQuietLook) hands back nothing —
 *  never set by the game */
export const NETQUIET_QA = { broken: false };

/** THE LINES (placeholders in the defenders' register — the writing pass is the maintainer's). name: the first silence, the man's name again, flat. why: the
 *  second. last: nobody left on the whole net to hear him. body: the finder, of a man nobody heard go. alone: the last of his squad, every mate of it gone
 *  quiet, other squads still on the net. ask: a roused man nobody has asked anything calls the net himself. `{name}` = the man asked for, as the squad's own
 *  ask names him (enemysquads.ts mate). */
type LineKind = 'name' | 'why' | 'last' | 'body' | 'alone' | 'ask';
const KINDS: LineKind[] = ['name', 'why', 'last', 'body', 'alone', 'ask'];
const LINES: Record<LineKind, string[]> = {
  name: ['{name}. … {name}.', '{name}? … {name}.', '{name}. {name}, answer up.'],
  why: ['why\'s nobody answering?', 'where is everybody? answer me.', 'that\'s another of you quiet. come on.'],
  last: ['somebody talk to me!', 'anybody. anybody at all, talk to me!', 'is anyone left on this net?'],
  body: ['he\'s down. nobody heard a thing?', 'man down here. not a sound out of him.', 'he\'s down, and nobody heard it?'],
  alone: ['that\'s my whole squad quiet. it\'s just me in here.', 'nobody left in here but me.', 'it\'s gone quiet in here. i\'m on my own.'],
  ask: ['all posts, answer up.', 'anyone on this net, come in.', 'who\'s still up? sound off.'],
};
/** (WORDS) a line as it is said now, by the line it stands for (read where the line is picked) */
const LINES_NOW: Record<string, string> = { 'that\'s another of you quiet. come on.': 'that\'s another one of you not answering. come on.', 'that\'s my whole squad quiet. it\'s just me in here.': 'my whole squad\'s quiet. it\'s just me in here.' };
/** this file's lines — every kind's, or one kind's, with a line's wording of now beside it (the check: a line said came from the right pool; bubble length and the originality blocklist) */
export function netQuietLines(kind?: LineKind): string[] { const L = (kind ? [LINES[kind]] : Object.values(LINES)).flat(); return [...L, ...wg(L.filter(t => LINES_NOW[t]).map(t => LINES_NOW[t]), [])]; }

/** the last man's act: nothing · his line is out and the beat runs · backing to the spot · low on it · said, and no room of his to work (nothing more) */
const NONE = 0, SAID = 1, LOW = 3, DONE = 4;
/** per man. `quiet` = checks nobody answered him (`asked`: the men they were for — a man counts once); `known` = the mates he knows are gone without asking:
 *  seen to fall, found, or told of on the net; `paid` = the largest shock of this file's he has taken for each dead man (one death, one shock). The line owed
 *  (`say`: its kind's index, −1 none; seconds to it; the name in it); seconds since his last line and its kind; `bodyT` since he last found a body; `roused`
 *  seconds he has been off his calm. The whip: when his ears last counted something (`heardT`, Witness.lastT) and how much (`cs` small noises, `cl` loud
 *  ones), where he last had his fix (`fx`,`fz`: a man who holds no room), the noise (`whipAt`), seconds to the swing, left of it and on this target. The last
 *  man: the act and its seconds, how long a living mate has been with him again (`mateT`), how much he knows and since when (`news`, `newsT`), how far round from his gun his EYES sit, low on his gun (`eye`, rad) and how wide they are in plan (`eyeHalf`, rad), the spot, the arc's middle (`yaw`) and width, what he watches from it (`look`) and whether he is down on it this frame (`low`). */
interface Mind { quiet: number; asked: Set<number>; known: Set<number>; paid: Map<number, number>; say: number; sayT: number; sayName: string; bark: number; lastKind: number; bodyT: number; roused: number;
  heardT: number; cs: number; cl: number; fx: number; fz: number; whipAt: Vec3 | null; whipIn: number; whipT: number; whipHeld: number;
  last: number; t: number; mateT: number; news: number; newsT: number; eye: number; eyeHalf: number; spot: Vec3 | null; yaw: number; arc: number; look: Vec3 | null; low: boolean; }
const stateOf = new SideMap<Guard, Mind>('netquiet.stateOf');
ownerWitness('lastMan', gd => { const N = stateOf.get(gd); return live() && !!N && mine(N); });

const live = () => NETQUIET.on && MORALE.on;
/** …and the feeds reach somebody (the check's breakage cuts them) */
const fed = () => live() && !NETQUIET_QA.broken;
const loudOf = (gd: Guard) => { const H = gd.witness.heard; return H.shots + H.kick + H.bang; };
/** a man this file is about at all (morale.ts's own men): a living defender or a man of the night staff, not one of the hostage rescue's cast */
const side = (g: Game, gd: Guard) => !gd.civ && (gd.team === 'hostile' || gd.team === 'guard') && !(g.rescue?.active && g.rescue.hostiles.includes(gd));
const ours = (g: Game, gd: Guard) => gd.state !== 'dead' && side(g, gd);
/** …on the net: alive, not in Sam's arm — and not a man runner.ts has (he has said he is done, dropped it, run: he hears no check and answers none) */
const onNet = (g: Game, gd: Guard) => ours(g, gd) && !gd.held && !hasHim(gd, OWN.runner);
function mind(gd: Guard): Mind {
  let N = stateOf.get(gd);
  if (!N) stateOf.set(gd, N = { quiet: 0, asked: new Set(), known: new Set(), paid: new Map(), say: -1, sayT: 0, sayName: '', bark: NETQUIET.barkSecs, lastKind: -1, bodyT: 60, roused: 0,
    heardT: gd.witness.heard.lastT, cs: gd.witness.heard.small, cl: loudOf(gd), fx: NaN, fz: NaN, whipAt: null, whipIn: 0, whipT: 0, whipHeld: 0,
    last: NONE, t: 0, mateT: 0, news: 0, newsT: 60, eye: 0, eyeHalf: NETQUIET.eyeDeg * DEG, spot: null, yaw: 0, arc: 0, look: null, low: false });
  return N;
}
/** a shock of this file's on one man: a steady man takes `steadyK` of it (morale.ts scales it by the nerve and caps it) */
function shock(g: Game, gd: Guard, amount: number) { if (amount > 0) moraleShock(g, gd, amount * (trainingOf(g, gd) === 'steady' ? NETQUIET.steadyK : 1)); }
/** …for one dead (or silent) man: what this one adds over the most he has already taken for that same man — the fall he saw, the check nobody answered and
 *  the body found are one death, counted once, at the largest of the three */
function pay(g: Game, gd: Guard, N: Mind, id: number, amount: number) { const had = N.paid.get(id) ?? 0; if (amount > had) { N.paid.set(id, amount); shock(g, gd, amount - had); } }

/** A RADIO CHECK'S SILENCE (enemysquads.ts radioCheck, guards.ts updateRollcall — as the asker asks for `who` by name): `men` = the net he asked on */
export function netQuietAsked(g: Game, asker: Guard | null, who: Guard, men: readonly Guard[]): void { return crossToRust('netQuietAsked', undefined, [g, asker, who, men]) as any; }
/** THE LAST DEFENDERS' CHECK CLOSES (laststand.ts tickCheck): `silent` did not answer `asker` — the dead among them too: it is the dead who do not answer —,
 *  `heard` did */
export function netQuietClosed(g: Game, asker: Guard, silent: Guard[], heard: Guard[]): void { return crossToRust('netQuietClosed', undefined, [g, asker, silent, heard]) as any; }
/** A BODY FOUND (enemysquads.ts event 'found', guards.ts the staff's body in view): the finder, and the men of `men` he tells */
export function netQuietBody(g: Game, finder: Guard | null, men: readonly Guard[]): void { return crossToRust('netQuietBody', undefined, [g, finder, men]) as any; }
/** has he an enemy in his eyes right now, or had one inside `shooterSecs` — whichever enemy: Sam, a teammate of his, any hostile unit */
function foeInView(g: Game, gd: Guard): boolean {
  if (g.time - gd.lastSeenT < NETQUIET.shooterSecs) return true;
  if (isHostile(gd.team, 'player') && !g.player.down && !g.playerInvisible && canSee(g, gd).visible) return true;
  for (const o of g.guards) if (o !== gd && o.state !== 'dead' && isHostile(gd.team, o.team) && canSeeChar(g, gd, o.char, false).visible) return true;
  return false;
}
/** A MAN GOES DOWN (combat.ts killGuard, beside moraleManDown, the victim not yet 'dead'): every mate with the fall in his eyes takes `fallUnseen` — or
 *  `fallSeen`, with the enemy who did it (any enemy) in view — and knows that man is gone without asking for him. A green man who took morale.ts's
 *  leaderShock for this same fall takes only the rest (and nothing more for that man later unless it is larger than that). */
export function netQuietFall(g: Game, victim: Guard) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('netQuietFall', undefined, [g, victim]); if (__r !== RUST_PASS) return __r as any; } }
  if (!fed() || !ours(g, victim)) return;
  const K = NETQUIET, id = victim.char.id;
  for (const o of g.guards) {
    if (o === victim || !onNet(g, o) || o.team !== victim.team || g.time < o.dazzledUntil) continue;
    if (!canSeeChar(g, o, victim.char, false).visible) continue;
    const N = mind(o), seen = foeInView(g, o); N.known.add(id);
    if (sawLeaderFall(o) === 0) N.paid.set(id, Math.max(N.paid.get(id) ?? 0, MORALE.leaderShock));
    pay(g, o, N, id, seen ? K.fallSeen : K.fallUnseen); NETQUIET_STATS.falls++; if (!seen) NETQUIET_STATS.unseen++;
  }
}



// ---------------------------------------------------------------- the tick
/** may this file act on him this frame: the AI on, no show on the floor, free of Sam's arm, himself (not dazzled, not doubled over), not morale.ts's */
const free = (g: Game, gd: Guard) => g.aiEnabled && !g.touring && !g.quietUtility && !gd.held && g.time >= gd.dazzledUntil && g.time >= gd.staggerUntil && !hasHim(gd, OWN.morale) && !hasHim(gd, OWN.runner);
/** the last man at his act, and this file's to keep where he is: his line said, backing, or low — not a man who has waited `giveUp` past his beat for the
 *  fight to hand him back (he is the fight's and his squad's again until he holds) */
const mine = (N: Mind) => !NETQUIET_QA.broken && N.last >= SAID && N.last <= LOW && !(N.last === SAID && N.t > NETQUIET.lastBeat + NETQUIET.giveUp);
/** at his post: holding his room, on nobody's scripted leg */
const posted = (gd: Guard, D: Def) => D.mode === 'hold' && gd.hold && !gd.script;
/** ONCE A FRAME, right after morale.ts's tick (game.ts update; the rungs are this frame's) and before anybody moves: the lines owed are said, the noises the
 *  men's ears counted last frame are read for a whip, the last man is taken a step through his act. A return on its first line with either switch off. */
export function netQuietTick(g: Game, dt: number): void { return crossToRust('netQuietTick', undefined, [g, dt]) as any; }
/** what he watches from the spot: the middle of the arc his doors sit in — moved round, by no more than it takes, until every door of it is inside his EYES
 *  (`eye`, `eyeHalf`: they sit round from his gun, toward his chest, and narrow — measured on a crouched man: 45° round, 26° either way; with his gun on the
 *  middle of a 34° arc neither door was in them past the near field, and a man walked 2.4 m into the room unseen). So his gun rests off the middle, toward
 *  his off side, and his eyes hold the doors; an arc wider than his eyes (doors on three walls) has their middle on its middle and its ends outside them. */
function watch(N: Mind) {
  const s = N.spot!; if (N.arc > Math.PI) { N.look = null; return; }
  const room = Math.max(0, N.eyeHalf - NETQUIET.eyeSlack * DEG - N.arc / 2), y = N.yaw - Math.sign(N.eye) * Math.max(0, Math.abs(N.eye) - room);
  N.look = [s[0] + Math.sin(y) * 6, 0, s[2] + Math.cos(y) * 6];
}

// ---------------------------------------------------------------- the seams that pose him
/** WHAT A MAN HOLDING A ROOM WATCHES (roomfight.ts applyWatch, after it has written `pt`): the last man's arc, low on his spot; a whip's noise. Untouched
 *  otherwise — and on its first line with the switch off. */
export function netQuietLook(gd: Guard, pt: Vec3): void { return goneWithTick('netQuietLook') as any; }
/** HOW HE STANDS AND WHERE HIS GUN GOES (guards.ts updateGuard's pose, beside tactics.ts's look): a last man low on his spot is crouched; a whipping man with
 *  nobody in his eyes has the noise as his aim, and comes round to it — aim and body — at the whip's rate, not the calm one. Null = nothing of this file's
 *  (and on its first line with the switch off). `eyesOnFoe`: he sees the man he is fighting — the fight's aim stands. */
export function netQuietPose(g: Game, gd: Guard, dt: number, eyesOnFoe: boolean): { aim: Vec3 | null; crouch: number } | null { return crossToRust('netQuietPose', undefined, [g, gd, dt, eyesOnFoe]) as any; }

/** QA: what the rule has of a man (null = never met) */
export function netQuietOf(gd: Guard): Readonly<Mind> | null { return stateOf.get(gd) ?? null; }
/** 'net ▸ Tango 2 quiet ×2 · Tango 4 LAST low (arc 31°)' — every living man with anything on him; '' with either switch off */
export function netQuietSummary(g: Game): string {
  if (!live()) return '';
  const acts = ['', 'LAST said', 'LAST backing', 'LAST low', 'LAST'];
  const men = g.guards.filter(gd => ours(g, gd) && stateOf.has(gd)).map(gd => [gd, stateOf.get(gd)!] as const).filter(([, N]) => N.quiet > 0 || N.last !== NONE || !!N.whipAt);
  return 'net ▸ ' + (men.map(([gd, N]) => `${gd.callsign}${N.quiet ? ` quiet ×${N.quiet}` : ''}${N.last ? ` ${acts[N.last]}${N.last === LOW ? ` (arc ${Math.round(N.arc / DEG)}°)` : ''}` : ''}${N.whipAt ? ' whip' : ''}`).join(' · ') || 'every man answered');
}

// ---------------------------------------------------------------- the visit's choice (the page: host/profile.ts pageSwitches; the debugger's Enemy nerve section)
const VISIT = 'twopointsix.netquiet.visit';
/** a choice made by hand in the debugger stands for the visit (the tab's own store), as morale.ts's does */
export function netQuietKeep() { try { globalThis.sessionStorage?.setItem(VISIT, JSON.stringify({ on: NETQUIET.on })); } catch { /* no store */ } }
/** THE PAGE'S RULE for the switch: the visit's kept choice, and over it the address — `?nonetquiet`. No flag and nothing kept (the shipped stand): on. */
export function netQuietFromQuery(q: URLSearchParams) {
  let kept: { on?: unknown } = {};
  try { kept = JSON.parse(globalThis.sessionStorage?.getItem(VISIT) ?? '{}') ?? {}; } catch { /* no store */ }
  NETQUIET.on = !q.has('nonetquiet') && kept.on !== false;
}

// ---------------------------------------------------------------- checkpoints (game/savestate.ts): each man's count, line owed, whip and act, by roster index
export interface NetQuietSnap { men: (number[] | null)[]; pts: (number[] | null)[]; names: (string | null)[]; asked: (number[] | null)[]; known?: (number[] | null)[]; paid?: (number[] | null)[] }
const n3 = (p: Vec3 | null) => p ? [p[0], p[1], p[2]] : [NaN, NaN, NaN];
/** nothing (no key in the file) with either switch off or before the rule has met anybody */
export function netQuietSnapshot(g: Game): NetQuietSnap | undefined {
  if (!live()) return undefined;
  const Ns = g.guards.map(gd => stateOf.get(gd) ?? null); if (!Ns.some(N => N)) return undefined;
  return { men: Ns.map(N => N ? [N.quiet, N.say, N.sayT, N.bark, N.bodyT, N.heardT, N.whipIn, N.whipT, N.last, N.t, N.lastKind, N.yaw, N.arc, N.low ? 1 : 0, N.cs, N.cl, N.fx, N.fz, N.whipHeld, N.roused, N.eye, N.eyeHalf, N.news, N.newsT] : null),
    pts: Ns.map(N => N ? [...n3(N.whipAt), ...n3(N.spot), ...n3(N.look)] : null), names: Ns.map(N => N ? N.sayName : null), asked: Ns.map(N => N ? [...N.asked] : null),
    known: Ns.map(N => N ? [...N.known] : null), paid: Ns.map(N => N ? [...N.paid].flat() : null) };
}
/** …onto the fresh roster (a file without the slice: every man is met afresh on his first tick). The squads' book and the record of who holds are set HERE too: the room fight's tick — the squads' with it — runs before this file's on the first frame after a load. */
export function netQuietRestore(g: Game, s: NetQuietSnap | null | undefined) {
  const f = (x: unknown, d = 0) => typeof x === 'number' && Number.isFinite(x) ? x : d, ids = (a: unknown) => (Array.isArray(a) ? a : []).filter((x): x is number => typeof x === 'number');
  netFeedTake(g); NETFEED.live = live();   // (nothing filed for a roster that is gone is read by this one)
  g.guards.forEach((gd, i) => { const M = s?.men?.[i], P = s?.pts?.[i];
    if (!M) { stateOf.delete(gd); releaseMan(gd, OWN.lastMan, g.time); return; }
    const p3 = (o: number): Vec3 | null => P && Number.isFinite(P[o]) ? [P[o], P[o + 1], P[o + 2]] : null, H = gd.witness.heard;
    const paid = new Map<number, number>(), pp = ids(s?.paid?.[i]); for (let k = 0; k + 1 < pp.length; k += 2) paid.set(pp[k], pp[k + 1]);
    const last = clamp(Math.round(f(M[8])), NONE, DONE);
    stateOf.set(gd, { quiet: Math.max(0, f(M[0])), asked: new Set(ids(s?.asked?.[i])), known: new Set(ids(s?.known?.[i])), paid, say: clamp(Math.round(f(M[1], -1)), -1, KINDS.length - 1), sayT: f(M[2]), sayName: String(s?.names?.[i] ?? ''), bark: f(M[3], NETQUIET.barkSecs), bodyT: f(M[4], 60),
      heardT: f(M[5], H.lastT), whipAt: p3(0), whipIn: f(M[6]), whipT: f(M[7]), last, t: Math.max(0, f(M[9])), mateT: 0, news: f(M[22]), newsT: f(M[23], 60), eye: f(M[20]), eyeHalf: f(M[21], NETQUIET.eyeDeg * DEG), lastKind: clamp(Math.round(f(M[10], -1)), -1, KINDS.length - 1), yaw: f(M[11]), arc: f(M[12]), low: !!M[13],
      cs: f(M[14], H.small), cl: f(M[15], loudOf(gd)), fx: f(M[16], NaN), fz: f(M[17], NaN), whipHeld: f(M[18]), roused: f(M[19]), spot: p3(3), look: p3(6) });
    if (live() && mine(stateOf.get(gd)!)) claimMan(gd, OWN.lastMan, g.time); else releaseMan(gd, OWN.lastMan, g.time); });
}
