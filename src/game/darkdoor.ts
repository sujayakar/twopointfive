// NOT IN THERE ALONE — THE DARK DOORWAY (the maintainer, 10-13, of the morale work: "especially for stealth sections that aren't perfect ghost where enemies just
// don't [know] what's going on, more poorly trained ones could get really scared and refuse to go into dark rooms" — and "a lot of the value from this comes with
// good telegraphing": he SAYS it, then he DOES it, with a beat the player can act on).
//
//   · WHO BALKS: a GREEN man (greenMan: morale.ts trainingOf) who is ALARMED (suspicious, searching or alert: he knows something is
//     out there — a calm man on his own beat walks his dark rooms as he always has), who is ALONE (no living mate inside `pairM` with a line to him), who stands
//     in the LIGHT (his own place reads `darkAt` on the meter, or `litBy` brighter than the room ahead: a man already in the dark has no lit side of a door to
//     stop on — and in a blackout nobody balks at all), and whose walk's next doorway, inside `stopM` ahead, stands OPEN onto a room that is DARK (a shut leaf
//     he opens as he always did: he cannot know what is behind it — then he balks, and steps back). His own torch does not exempt him. A steady man, a man with
//     a mate at his shoulder, a lit room: all walk in exactly as they did. Never a choreographed man, a trained one (the response team), a man of the hostage
//     rescue's cast, a man morale.ts has in a sequence, a man in the standoff over a held colleague, a man on an errand he was SENT on (the breaker, a silent
//     colleague's route, a body he was called to). IN A FIGHT only a chase to a last-known fix is stopped: not a man with a sighting newer than `seenSecs`, not
//     a walk to cover, not a man closing on a foe who is down.
//   · THE DARK ROOM: the room on the far face of the doorway (cleared.ts's rooms: a flood from the door's floor; open floor is nobody's room and never counts),
//     `samples` of its floor cells read for the PLACE's steady light (calmtorch.ts placeLight: nothing anybody carries, no flash) — dark when `darkShare` of
//     them read under `darkAt` on the meter. Read once per `askSecs` a room, whoever asks; the readings ride a checkpoint.
//   · THE BALK: he stops a stride short of the frame (`standM`), off the middle of the doorway toward his own jamb (`sideM`) — not in the opening, with a view
//     through it — and SAYS it as he stops. With a mate to call the line IS the call: spoken if that man is inside `earM`, on the net with the handset's chirp
//     otherwise; the mate answers after `copySecs`. With nobody to call it is a different line, said to himself. Then he SWEEPS: his aim, and his light with it,
//     goes slowly across the opening and back (`sweepSecs` a pass) for as long as he waits. The mate is the free man of his side with the shortest WALK to him
//     inside `mateM` (of the `mateTry` nearest; asked again every `callSecs` while nobody is coming): he walks to him (the floor's own 'suspicious' walk to a
//     fix, the fix kept on the caller — what guards.ts sendCheckOn does to a man, less the errand). When that mate, or any man free to come, is inside `pairM`
//     with a line to him, the mate says so and they go in together: the first man's walk resumes and the mate's fix is the first man's goal for `withSecs`. A
//     mate who dies, is taken by something else, or covers no ground for `lagSecs` is dropped and the next man called.
//   · NEVER A SAFE ROOM FOR GOOD: nobody inside `waitSecs` and he says so and GIVES THE ROOM UP: every spot of his search in it is struck off, and the leg is
//     dropped by the walk's own rule for a leg that goes nowhere (a suspicion's fix is where he stands; a chase is dropped to a search of where he stands —
//     never with a foe in his eyes: the wait's clock stands while he has one). The `fedUpN`th time he is sent at that ROOM inside `fedUpSecs` he NERVES
//     HIMSELF: says it, stands `nerveSecs`, and goes in alone at `slowK` of his pace, light first. A door he has gone through, with a mate or alone, does not
//     stop him again for `againSecs`. An enemy seen INSIDE the room while he waits does not take him in: he fires from where he stands by the floor's own alert
//     rules, untouched.
//   · A man called, and a pair going in, are never balked: two men sent to look are not alone.
// SHOW, NOT TELL: nothing is drawn or written for the player but the men themselves. No die at all, no wall clock, no graphics setting: the light is the CPU's
// steady estimate (the same number on the page and in the harness); the clocks are seconds COUNTED and ride a checkpoint (darkDoorSnapshot / Restore).
// It only ever acts with MORALE.on as well. The switch is off in the module and on the frozen stand: every seam returns on its first line, no man is met.
// Nothing here touches health, aim, skill or a reaction beat.
// Type-only game imports but for the functions below, used inside calls only (src/game is one import cycle).
import { SideMap, GameSideMap } from '../math/side';
import type { Game, Guard } from './game';
import { Vec3 } from '../math/vec';
import './guards';
import './calmtorch';
import './hours';
import { MORALE } from './morale';
import './teams';
import './runner';   // (asked inside a tick only: false with its switch off)
import { hasHim, OWN } from './owner';
import { crossToRust } from './crossing';

/** the switch and its numbers: off in the module, raised by the page (host/profile.ts pageSwitches; ?nodarkdoor = every man walks into the dark as he did) and by
 *  the shipped stand; the harness's frozen stand leaves it off (TPS_DARKDOOR=1 turns it on — with TPS_MORALE=1: it acts only under MORALE.on). `who` = who is
 *  green: 'rest' (morale.ts trainingOf: the knob, the squad's key, the difficulty's mix) · 'all' · 'none'. Metres, seconds and shares as the
 *  head of the file names them; `openRad` = how far a leaf must stand off its frame to be an open door, `armSecs` how long a shut door he
 *  reached in the light stays his to balk at once it opens; `keepAw` = the awareness a waiting man's suspicion is
 *  held at (it must not drain away under him at the frame); `keepSecs` = how long a balk outlasts his walk not asking (a reaction beat, a clear shot), and
 *  `resumeSecs` how long after that the same door takes the same wait up again without a word. UNTUNED by eye (nobody has seen it on a GPU, nobody has played it). */
export const DARKDOOR = { on: false, who: 'rest' as 'rest' | 'all' | 'none', stopM: 2.5, pairM: 4, darkAt: 0.2, darkShare: 0.7, litBy: 0.1, samples: 40, askSecs: 0.5, openRad: 0.35, armSecs: 3,
  standM: 1.3, sideM: 0.3, sweepSecs: 3.2, earM: 12, mateM: 40, mateTry: 3, callSecs: 2, copySecs: 1.2, lagSecs: 5, withSecs: 10, keepAw: 0.3, keepSecs: 2, resumeSecs: 12, seenSecs: 2,
  waitSecs: 20, againSecs: 45, fedUpSecs: 300, fedUpN: 2, nerveSecs: 1.5, slowK: 0.5 };
export const DARKDOOR_STATS = { balks: 0, calls: 0, pairs: 0, quits: 0, solos: 0, drops: 0, asks: 0 };

/** what the rule has a man doing: nothing · stopped at a frame · going in alone, slowly · called, walking to the man who balked · going in with him */
export type DarkDoorAct = 'none' | 'balk' | 'solo' | 'come' | 'with';
const ACTS: DarkDoorAct[] = ['none', 'balk', 'solo', 'come', 'with'];
/** per man. The act and its seconds `t` (a balk's: seconds WAITED); the door (its index in the level's list), the
 *  face he is on and where he stands by it (`at`: he is there); the other man of the pair (the mate he called / the man who called him). A balker's: has the
 *  call been said, seconds since he last looked for a man to call, the men dropped on the way to him, when the balk was last let go unresolved (`left`). A
 *  shut door he is opening onto a dark room, noted while he still stood in the light, and when (`armed`, `armT`: he is owed the balk when it stands open). A
 *  mate's: has he answered, the place he last made half a metre of ground from (`spot`) and the seconds since (`lag`). The rooms he gave up and when; the doors that will not
 *  stop him and until when; lines said (deals the next); seconds since his walk last had the gate hold him, and since he last asked the room and for a mate. */
interface Fear { act: DarkDoorAct; t: number; door: number; side: 1 | -1; spot: Vec3 | null; at: boolean; other: Guard | null;
  called: boolean; callT: number; tried: Guard[]; left: number; told: boolean; lag: number;
  armed: number; armT: number; quits: { room: number; t: number }[]; pass: { door: number; until: number }[]; said: number; idle: number; ask: number; }
const stateOf = new SideMap<Guard, Fear>('darkdoor.stateOf');
interface Read { t: number; dark: boolean; share: number; mean: number }
/** a room's reading, kept `askSecs`: by game and room number */
const readsOf = new GameSideMap<Game, Map<number, Read>>('darkdoor.readsOf');
/** a level's rooms as floor cells to read, dealt once a nav grid: room number → cell centres (x, z pairs) */
const cellsOf = new SideMap<object, Map<number, number[]>>('darkdoor.cellsOf');

/** THE LINES (placeholders in the register the advisor's round set — the writing pass is the maintainer's). `say` = the lines that name nobody; `he` / `they`
 *  as morale.ts has them (words.ts: a scenario's defender under the pass is up against a team). call: as he stops, to the mate he calls. self: as he stops,
 *  nobody to call. copy: the mate, setting off. with: the mate, at his shoulder. nobody: the wait is over and no one came. alone: sent at the room he gave
 *  up — said, then he goes in. */
type LineKind = 'call' | 'self' | 'copy' | 'with' | 'nobody' | 'alone';
const LINES: Record<LineKind, { say: string[]; he?: string[]; they?: string[] }> = {
  call: { say: ['I\'m not going in there alone. get up here.', 'not in there. not on my own — get up here!'], he: ['he could be anywhere in that. somebody get up here.'], they: ['they could be anywhere in that. somebody get up here.'] },
  self: { say: ['no. not in there. not on my own.', 'pitch black. no. not alone.', 'not a chance. not by myself.'] },
  copy: { say: ['copy. coming.', 'on my way. stay put.', 'hold there — coming to you.'] },
  with: { say: ['on you. get a light in there.', 'I\'m here. lights on — go.', 'right behind you. go on.'] },
  nobody: { say: ['nobody\'s coming.', 'no one. all right — leave it.', 'nobody. forget that room.'] },
  alone: { say: ['fine. fine — I\'ll do it myself.', 'nobody ever comes. going in.', 'all right. slow. going in.'] },
};
/** this file's lines — every kind's, or one kind's (the check: a line said came from the right pool; the words lint's bubble length and blocklist) */
export function darkDoorLines(kind?: LineKind): string[] { return (kind ? [LINES[kind]] : Object.values(LINES)).flatMap(l => [...l.say, ...(l.he ?? []), ...(l.they ?? [])]); }

const rescued = (g: Game, gd: Guard) => !!g.rescue?.active && g.rescue.hostiles.includes(gd);
/** nobody's but his own: no choreography, no order to stand, no standoff, no errand, no body, not morale.ts's, not the rescue's */
const own = (g: Game, gd: Guard) => !gd.script && !gd.hold && !gd.pinned && !gd.standoff && !gd.task && !gd.bodyDuty && !rescued(g, gd) && !hasHim(gd, OWN.morale) && !hasHim(gd, OWN.lastMan);   // (…nor the last man at his act, netquiet.ts: his hold wins — never stopped at a doorway, never called to one)

/** IS THE ROOM DARK — the one place this file reads a room's light: `samples` of its floor cells, evenly through its list, each the place's own steady light
 *  on the meter's scale (calmtorch.ts placeLight: lightEstimateAt with every carried lamp and every transient light left out); dark = `darkShare` of them
 *  under `darkAt`; `mean` = their mean. By day a cell under the bar that the hour's daylight reaches (calmtorch.ts workLit: the estimate has no sun in it) is
 *  not dark: it counts at the bar. Kept `askSecs`. Open floor (room 0) is never dark. */
export function darkRoom(g: Game, room: number): Readonly<Read> { return crossToRust('darkRoom', undefined, [g, room]) as any; }



/** QA: what the rule has of a man (null = never met) */
export function darkDoorOf(gd: Guard): Readonly<Fear> | null { return stateOf.get(gd) ?? null; }
/** QA: is he green as the rule deals it */
export function darkDoorGreen(g: Game, gd: Guard): boolean { return crossToRust('darkDoorGreen', undefined, [g, gd]) as any; }
/** 'dark door ▸ Novak balk 6.2 s at breakroom_w (Reyes coming) · Reyes come' — every man the rule has; '' with either switch off */
export function darkDoorSummary(g: Game): string {
  if (!DARKDOOR.on || !MORALE.on) return '';
  const men = g.guards.filter(gd => { const S = stateOf.get(gd); return !!S && S.act !== 'none'; }).map(gd => { const S = stateOf.get(gd)!, d = g.doors.list[S.door];
    return `${gd.callsign} ${S.act}${S.act === 'balk' || S.act === 'solo' ? ` ${S.t.toFixed(1)} s at ${d?.def.name ?? '?'}` : ''}${S.other ? ` (${S.act === 'balk' ? `${S.other.callsign} coming` : `with ${S.other.callsign}`})` : ''}`; });
  return 'dark door ▸ ' + (men.join(' · ') || 'nobody stopped');
}

// ---------------------------------------------------------------- checkpoints (game/savestate.ts): each man by roster index — green or not, his act, door, place
// and clocks, the rooms he gave up, the doors open to him, the men he dropped — the rooms' readings still in hand (a file from before morale.ts's training carries a green flag a man and a `dealt` count: read past, never kept)
export interface DarkDoorSnap { men: (number[] | null)[]; quits: (number[] | null)[]; pass: (number[] | null)[]; tried: (number[] | null)[]; reads: number[][]; dealt?: number }
/** nothing (no key in the file) with either switch off or before the rule has met anybody */
export function darkDoorSnapshot(g: Game): DarkDoorSnap | undefined {
  if (!DARKDOOR.on || !MORALE.on) return undefined;
  const of = (gd: Guard) => stateOf.get(gd); if (!g.guards.some(gd => of(gd))) return undefined;
  const fin = (x: number, d: number) => Number.isFinite(x) ? x : d;
  return { men: g.guards.map(gd => { const S = of(gd); return S ? [-1, ACTS.indexOf(S.act), S.t, S.door, S.side, S.spot ? S.spot[0] : NaN, S.spot ? S.spot[2] : NaN, S.at ? 1 : 0, S.other ? g.guards.indexOf(S.other) : -1,
      S.called ? 1 : 0, S.callT, fin(S.left, -1e9), S.told ? 1 : 0, S.lag, S.said, S.idle, S.ask, S.armed, fin(S.armT, -1e9)] : null; }),
    quits: g.guards.map(gd => { const S = of(gd); return S ? S.quits.flatMap(q => [q.room, q.t]) : null; }), pass: g.guards.map(gd => { const S = of(gd); return S ? S.pass.flatMap(p => [p.door, p.until]) : null; }),
    tried: g.guards.map(gd => { const S = of(gd); return S ? S.tried.map(o => g.guards.indexOf(o)) : null; }),
    reads: [...(readsOf.get(g) ?? [])].map(([room, r]) => [room, r.t, r.dark ? 1 : 0, r.share, r.mean]) };
}
/** …onto the fresh roster (a file without the slice: every man is met afresh — dealt on the first tick, nobody stopped, no room read) */
export function darkDoorRestore(g: Game, s: DarkDoorSnap | null | undefined) {
  const f = (x: unknown, d = 0) => typeof x === 'number' && Number.isFinite(x) ? x : d;
  const R = new Map<number, Read>(); for (const r of s?.reads ?? []) R.set(f(r[0]), { t: f(r[1]), dark: !!r[2], share: f(r[3]), mean: f(r[4], 1) });
  if (R.size) readsOf.set(g, R); else readsOf.delete(g);
  g.guards.forEach((gd, i) => { const M = s?.men?.[i], Q = s?.quits?.[i] ?? [], P = s?.pass?.[i] ?? [], T = s?.tried?.[i] ?? [];
    if (!M) { stateOf.delete(gd); return; }
    const quits: Fear['quits'] = []; for (let k = 0; k + 1 < Q.length; k += 2) quits.push({ room: f(Q[k]), t: f(Q[k + 1]) });
    const pass: Fear['pass'] = []; for (let k = 0; k + 1 < P.length; k += 2) pass.push({ door: f(P[k], -1), until: f(P[k + 1]) });
    stateOf.set(gd, { act: ACTS[f(M[1])] ?? 'none', t: Math.max(0, f(M[2])), door: f(M[3], -1), side: M[4] === -1 ? -1 : 1, spot: Number.isFinite(M[5]) && Number.isFinite(M[6]) ? [M[5], 0, M[6]] : null, at: !!M[7],
      other: g.guards[f(M[8], -1)] ?? null, called: !!M[9], callT: f(M[10]), left: f(M[11], -1e9), told: !!M[12], lag: f(M[13]), said: f(M[14]), idle: f(M[15]), ask: f(M[16]), armed: f(M[17], -1), armT: f(M[18], -1e9),
      quits, pass, tried: T.map(k => g.guards[k]).filter((o): o is Guard => !!o) }); });
}
