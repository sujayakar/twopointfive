// THE LIGHTS GO OUT AND SOMEBODY GOES TO PUT THEM BACK ON (defenders answer the plan the team chose: the answer to the mains being cut).
//
// WHAT WAS THERE: lighting.ts setBlackout hands the walk to the breaker to the nearest calm man who can leave where he stands. A mission's defenders (the room
// fight's hostile units: a room clear, a rescue, the night breach) are every one of them held on a mark, so with workthings.ts's `posted` rule nobody is handed
// it and nobody says it: the team cuts the mains and the garrison stands in the dark for as long as the team cares to take.
//
// THE RULE, for a room fight's hostile defenders, once per blackout:
//   · WHO NOTICES: a living defender, his wits about him, with a mains-fed fixture that was burning in his plain view inside `seeM` (a clear line from his eye to
//     where it hangs) — or whose floor goes dark: the floor he stands on, watches (his aim and either edge of his look, out to `lookM`) or has about him
//     (`aboutM` every way) read `litAt` or over on the meter's scale by the place's own light at the trip and under `dropK` of that `darkAfter` seconds on (a
//     lamp over a glass roof, a fitting round a corner: what he loses is the lit floor, not the bulb). Nobody loses any: nothing is said and nobody goes — a
//     garrison in rooms with no light of the mains, or in daylight with no fitting in view, has nothing to miss.
//   · THE WORD COMES FIRST. `think` seconds after the trip (by the id of the first man to notice: no dice) one of them says on their net what they do, and
//     nobody moves for `beat` seconds after it. The net is what the team's strip shows (cues.ts), the walk files its boots for the team's ears (pressure.ts
//     boots), and the men who go burn their weapon lights for as long as the mains are down (guards.ts poseFlashlight: `powerOutLit`) — heard and seen
//     before it lands; what lands is the lights coming back on.
//   · WHO GOES: only men who are FREE (`kept` below — calm, holding a post the room fight gave them, nobody else's: no dig-in, squad's word, lane, press, buddy,
//     last stand, captor, script, Sam's arm). The breaker in a defender's own room: the man of that room nearest it, alone, and he never leaves the room.
//     Elsewhere: the `men` nearest it by the way they would walk, no further than `maxM`, A MAN LEFT IN EVERY ROOM THAT HAD ONE while that can be done; no
//     man to spare anywhere and `lone`: the nearest goes by himself and his room stands empty behind him (that is the price of the lights, and the team's
//     opening). A squad told to stay in its rooms (`mayLeave: false`: it answers no other squad's FIGHT) still sends for the lights with `stayers`.
//   · THE WALK is the room fight's own 'move' leg at the pace of a man going to look (his own senses under it: a man of the team seen close stops him to
//     fight, and he is the room fight's from there). The first man puts his hand to the lever (lighting.ts re-arms it under any man of their side who stands
//     at it: its own fumble, its own "breaker's back in"); the second stops a few paces short with a line to him and watches the way they came. The mains
//     back — by his hand, by the trip's own clock, by anybody — every man out walks back to the post he left and holds it as he held it.
//   · THE GARRISON ALREADY KNOWS (a man of them alert or searching, or with gunfire, a boot or a charge in his ears — or they are holding hostages: rescue.ts
//     has every one of them learn of the team by the dark itself): nobody walks out into a fight to mend a
//     fuse — "that's them — stay on your doors!", and THE WORD is out (below). The same the `parties`+1-th time the lights go in one fight, and after a
//     party that lost a man: a fault does not happen twice, and nobody is fed to it again.
//   · A BREAKER SHOT TO SCRAP: the man who gets there looks at it for `lookSecs`, says so, and the word is out; they go back.
//   · THE WORD (`powerOutWord`): their side holds the dark for the team's doing. darkfoe.ts takes it as the word a mate's lamp is — a man of theirs whose view
//     is dark lights his own — for as long as the mains stay down. Nothing else reads it.
//   · A MAN OF THE PARTY GOES DOWN OR IS TAKEN: nobody is sent again this fight. Nobody is told by magic either: a mate out with him who has the place he
//     fell in plain view (or who gets to his own place by the lever and finds nobody at it) says so ("he's down at the breaker — that's them!"), the word is out and he runs for his post; a fall nobody saw is only MISSED —
//     `overdue` seconds past the walk's own length with the mains still down and no word from the party, "nothing from the breaker — that's them!".
//   · THE FIRST MAN CANNOT STEP OFF when the beat is up (dazzled, taken, another rule's by then): the second goes for the lever in his place; neither can:
//     nobody goes, and the walk is not counted. A lever that will not take a hand in `leverSecs` is given up. The lights back and gone again inside a walk:
//     the party already out is the answer (a man on his way home turns back). The switch thrown off under a walk: the men out run back to their posts.
//   · FIGHT WHILE THEY ARE OUT (a man of them roused, or the walker's own ears count a shot, a boot or a charge): a man with his hand already at the lever
//     finishes; anybody else goes back to his post at a run and is the room fight's man there.
// Nothing here gives anybody better eyes, aim or health, and no man is added. What a defender acts on is a fixture he saw die, his side's net, and his own
// senses on the walk — never where the team is.
// DYNAMIC: the way to the breaker is asked of the walk grid as it stands when the word is given and again on every leg, so a wall blown open or a door
// wedged shut changes the way they come; a breaker no man can reach inside `maxM` sends nobody.
// No dice, no camera, no graphics setting, no wall clock. State per Game (powerOutSnapshot / powerOutRestore beside it, carried by savestate.ts; the timeline
// is the check's and the debugger's and is not kept). Type-only game imports but for functions used inside calls (src/game is one import cycle).
import { SideMap } from '../math/side';
import type { Game, Guard } from './game';
import type { RoomFight } from './roomfight';
import { Vec3, v3 } from '../math/vec';
import { isHostile } from './teams';
import { order } from './squad';
import { run } from './pressure';
import './workthings';
import './lighting';
import { lightEstimateAt } from './guards';
import { carriesDarkLamp } from './darkfoe';
import './digin';
import './gunroles';
import { claimMan, releaseMan, ownerWitness, OWN } from './owner';
import './pressure';
import './buddy';
import { goneWithTick } from './crossing';

/** the switch and its numbers: off in the module, raised by the page (host/profile.ts pageSwitches; ?nopowerout = the garrison stands in the dark, as it did)
 *  and by the shipped stand; the harness's frozen stand leaves it off (TPS_POWEROUT=1 turns it on). UNTUNED by play: nobody has played against it. */
export const POWEROUT = {
  on: false,
  /** m: a burning mains fixture in a man's plain view this near is one he saw die */
  seeM: 14,
  /** s between the trip and their word on the net (by id) · s between the word and the first step */
  think: [3, 6] as [number, number], beat: 2,
  /** … or the floor he stands on, watches (out to `lookM`, every `stepM`) or has about him (`aboutM` every way) read `litAt` or over by the place's light and falls under `dropK` of it, read `darkAfter` s after the trip */
  lookM: 12, aboutM: 4, stepM: 2, fanRad: 0.3, litAt: 0.3, dropK: 0.5, darkAfter: 0.3,
  /** men who go · the longest way they will walk for it, m · a garrison with nobody to spare sends its nearest man alone */
  men: 2, maxM: 60, 
  /** times a garrison walks to the breaker in one fight (the next time the lights go it is no fault) */
  parties: 1,
  /** s a man looks at a breaker shot to scrap before he says so · legs a walk is tried in before it is given up */
  lookSecs: 1.5, tries: 8,
  /** s the first man stands at a lever that will not take his hand before he gives it up · s past the walk's own length before a party that lost a man and
   *  sent no word is missed ("nothing from the breaker") · the nearest men, in a straight line, whose way to the lever is worked out */
  leverSecs: 8, overdue: 15, ask: 6,
  /** the second man: how far short of the lever he stops, m (nearest free floor in that band with a line to it) */
  coverM: [1.6, 3.2] as [number, number],
};

type Def = RoomFight['defs'][number];
/** how a man held his post before he left it (enemysquads.ts's looker keeps the same) */
export interface Calm { awareness: number; hold: boolean; pinned: boolean; station: Vec3; home: Vec3; room: Def['room']; ev: number[]; watch: number; want: number; pt: Vec3 }
type Phase = 'wait' | 'out' | 'there' | 'back';
interface Man { gd: Guard; role: 'lever' | 'cover'; phase: Phase; goal: Vec3; from: Vec3; watch: Vec3 | null; tries: number; stepT: number; heard0: number; thereT: number; calm: Calm | null }
export interface PowerOutEvent { t: number; what: 'noticed' | 'said' | 'go' | 'there' | 'wrecked' | 'restored' | 'home' | 'abort' | 'lost' | 'overdue' | 'stay' | 'nobody'; who: string; why: string }
interface State {
  /** the blackout this is the answer to (Game.blackout.since) · when their word is due · when the men step off (−1 = no party) */
  since: number; decideAt: number; goAt: number;
  /** the walk's length as their word gave it, m · when a party that lost a man and has sent no word is missed (−1 = nobody is waited for) */
  wayM: number; overdueAt: number;
  /** where a man of the party went down, until a mate out with him has seen it */
  fell: Vec3 | null;
  /** men whose floor was lit at the trip (index in Game.guards, the reading) and when it is read again (−1 = read, or no need) */
  seen: { i: number; lit: number }[]; lookAt: number;
  men: Man[]; decided: boolean; word: boolean;
  /** kept over the whole fight: walks to the breaker so far, and a party that lost a man */
  parties: number; lost: boolean;
  events: PowerOutEvent[];
}
const STATES = new SideMap<Game, State>('powerout.STATES');
ownerWitness<Game>('breaker', (gd, g) => POWEROUT.on && !!STATES.get(g)?.men.some(m => m.gd === gd));

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const frac = (x: number) => x - Math.floor(x);
/** his id's share of 0‥1 (no dice) */
const idShare = (gd: Guard) => frac(gd.char.id * 0.6180339887);
const alive = (gd: Guard) => gd.state !== 'dead' && gd.char.alive && !gd.fled;
const fit = (g: Game, gd: Guard) => alive(gd) && !gd.held && g.time >= gd.dazzledUntil && g.time >= gd.staggerUntil;
const defOf = (g: Game, gd: Guard): Def | undefined => g.fight.defs.find(D => D.gd === gd);
/** the rule's men: the room fight's defenders on the other side from the player, in live play */
function ruled(g: Game): Def[] {
  if (!POWEROUT.on || g.touring || g.director.showing || g.quietUtility || !g.level.breaker) return [];
  return g.fight.defs.filter(D => isHostile(D.gd.team, 'player') && !!D.gd.unit);
}
function event(g: Game, st: State, what: PowerOutEvent['what'], gd: Guard | null, why: string) {
  st.events.push({ t: g.time, what, who: gd ? gd.callsign : '-', why }); if (st.events.length > 120) st.events.splice(0, 40);
  const L = g.fight.log; L.push({ t: g.time, who: gd ? gd.callsign : '-', what: `[powerout] ${what}: ${why}` }); if (L.length > 600) L.splice(0, 200);
}

/** a mains-fed fixture that was burning, in his plain view inside `seeM` */
function sawThemDie(g: Game, gd: Guard): boolean {
  const c = gd.char, eye: Vec3 = [c.pos[0], 1.5, c.pos[2]];
  for (const t of g.targets) {
    if (!t.mains || !t.on || t.broken || !(t.factor > 0.3) || t.kind === 'breaker' || t.kind === 'device') continue;   // (factor: last frame's supply — a fitting the OCP has knocked out is no light to see die)
    if (flat(t.pos, c.pos) > POWEROUT.seeM) continue;
    if (!g.col.segmentBlocked(eye, [t.pos[0], Math.max(1.2, t.pos[1] - 0.15), t.pos[2]])) return true;
  }
  return false;
}
/** the brightest floor he stands on, watches or has about him, by the place's own light (guards.ts lightEstimateAt, the CPU meter both the page and the harness run: nobody's
 *  carried lamp, no flash, no beacon) */
function floorWatched(g: Game, gd: Guard): number {
  const K = POWEROUT, c = gd.char, eye: Vec3 = [c.pos[0], 1.5, c.pos[2]], at = (x: number, z: number) => g.visibilityOf(lightEstimateAt(g, [x, 1.2, z], carried, true));
  let best = at(c.pos[0], c.pos[2]);
  for (const off of [0, -K.fanRad, K.fanRad]) { const fx = Math.sin(c.aimYaw + off), fz = Math.cos(c.aimYaw + off);
    for (let r = K.stepM; r <= K.lookM + 1e-6; r += K.stepM) { const x = eye[0] + fx * r, z = eye[2] + fz * r;
      if (g.col.segmentBlocked(eye, [x, 1.5, z])) break;
      best = Math.max(best, at(x, z)); } }
  // (… and the floor about him, whichever way he faces: a room going dark is not missed for looking at its door)
  for (let k = 0; k < 8; k++) { const fx = Math.sin(k * Math.PI / 4), fz = Math.cos(k * Math.PI / 4);
    for (let r = K.stepM; r <= K.aboutM + 1e-6; r += K.stepM) { const x = eye[0] + fx * r, z = eye[2] + fz * r;
      if (g.col.segmentBlocked(eye, [x, 1.5, z])) break;
      best = Math.max(best, at(x, z)); } }
  return best;
}
const carried = () => true;
/** a leg of the walk: pressure.ts's scripted leg at the pace of a man going to look (the tick below ends it itself, before the room fight's plant would) */
function walk(g: Game, D: Def, to: Vec3) { run(g, D, to, null); order(D.gd, { speed: g.tune.guardInvestigate }); }
/** a kept post as a checkpoint carries it (its room by id) and back */
export type CalmSnap = Omit<Calm, 'room'> & { room: number };
export const calmSave = (c: Calm): CalmSnap => ({ ...c, station: v3.copy(c.station), home: v3.copy(c.home), ev: [...c.ev], pt: v3.copy(c.pt), room: c.room ? c.room.id : -1 });
export const calmLoad = (g: Game, c: CalmSnap): Calm => ({ ...c, station: v3.copy(c.station), home: v3.copy(c.home), ev: [...c.ev], pt: v3.copy(c.pt), room: g.fight.defs.map(D => D.room).find(R => R?.id === c.room) ?? null });

/** lighting.ts setBlackout, on a trip that found the mains up: the answer's clock starts (a return on its first line with the switch off or no defenders) */
export function powerOut(g: Game) { decideOut(g); breakerSay(g); }
function decideOut(g: Game) {
  const defs = ruled(g); if (!defs.length) return;
  const prev = STATES.get(g), first = defs.find(D => fit(g, D.gd) && sawThemDie(g, D.gd));
  const out = prev?.men.length ? prev : null;   // (a party of the last blackout still out — the lights back and gone again inside its walk: it is this one's answer too)
  const st: State = { since: g.time, decideAt: Infinity, goAt: out ? out.goAt : -1, wayM: out ? out.wayM : 0, overdueAt: out ? out.overdueAt : -1, fell: out ? out.fell : null, seen: [], lookAt: -1, men: prev?.men ?? [], decided: !!out,
    word: prev?.word ?? false, parties: prev?.parties ?? 0, lost: prev?.lost ?? false, events: prev?.events ?? [] };   // (their word once out stays out: a later dark is the team's doing to them from its first frame)
  STATES.set(g, st);
  if (out) {
    for (const M of st.men) { const D = defOf(g, M.gd); if (M.phase !== 'back' || !D || D.mode !== 'move') continue;
      M.phase = 'out'; M.tries = 0; walk(g, D, M.goal); event(g, st, 'go', M.gd, 'the lights go again on his way home: he turns back for the lever'); }
    return;
  }
  if (first) return noticed(g, st, first.gd, 'the lights die in his view');
  // (the lights are still burning this frame — lighting.ts updateTargets takes them down on the next: what each man's floor reads now is read again in the dark)
  for (const D of defs) { if (!fit(g, D.gd)) continue; const lit = floorWatched(g, D.gd); if (lit >= POWEROUT.litAt) st.seen.push({ i: g.guards.indexOf(D.gd), lit }); }
  if (st.seen.length) st.lookAt = g.time + POWEROUT.darkAfter; else nobody(g, st);
}
function noticed(g: Game, st: State, gd: Guard, how: string) {
  st.seen = []; st.lookAt = -1;
  st.decideAt = st.since + POWEROUT.think[0] + (POWEROUT.think[1] - POWEROUT.think[0]) * idShare(gd);
  event(g, st, 'noticed', gd, `${how}: their word is due ${(st.decideAt - g.time).toFixed(1)} s on`);
}
function nobody(g: Game, st: State) {
  st.seen = []; st.lookAt = -1; st.decided = true;
  event(g, st, 'nobody', null, 'no defender had a burning mains fixture in his view or lost the light on his floor: nobody knows the power is out');
}
/** lighting.ts setBlackout: is the walk to the breaker this rule's to hand out (so the stock errand passes this man over)? */
export function powerOutRules(g: Game, gd: Guard): boolean { return POWEROUT.on && !!gd.unit && ruled(g).some(D => D.gd === gd); }
/** guards.ts poseFlashlight: a man out for the lights burns his own while the mains are down */
export function powerOutLit(g: Game, gd: Guard): boolean {
  if (!POWEROUT.on || !g.blackout.active) return false;
  const M = STATES.get(g)?.men.find(m => m.gd === gd); return !!M && M.phase !== 'wait' && carriesDarkLamp(gd);   // (a man the mission dealt no lamp at all has none to burn: darkfoe.ts `carry`)
}
/** darkfoe.ts hasWord: their side holds this dark for the team's doing */
export function powerOutWord(g: Game, gd: Guard): boolean { return goneWithTick('powerOutWord') as any; }
/** is he out for the lights (or about to step off)? */
/** what was said and done, oldest first (the check and the debugger read it; nothing in the game does) */
export function powerOutTimeline(g: Game): PowerOutEvent[] { return STATES.get(g)?.events ?? []; }
export function powerOutReset(g: Game) { STATES.delete(g); breakerSay(g); }
/** WHO THE BREAKER'S PARTY HAS (src/game/owner.ts): out for the lights, or waiting to be sent */
function breakerSay(g: Game): void { const men = POWEROUT.on ? STATES.get(g)?.men : undefined; for (const gd of g.guards) { if (men?.some(m => m.gd === gd)) claimMan(gd, OWN.breaker, g.time); else releaseMan(gd, OWN.breaker, g.time); } }



// ---------------------------------------------------------------- the state's pair (new module state the game depends on rides a snapshot)
export type PowerOutSnap = { since: number; decideAt: number; goAt: number; wayM: number; overdueAt: number; fell: Vec3 | null; seen: { i: number; lit: number }[]; lookAt: number; decided: boolean; word: boolean; parties: number; lost: boolean;
  men: { i: number; role: Man['role']; phase: Phase; goal: Vec3; from: Vec3; watch: Vec3 | null; tries: number; stepT: number; heard0: number; thereT: number; calm: CalmSnap | null }[] };
export function powerOutSnapshot(g: Game): PowerOutSnap | undefined {
  const st = STATES.get(g); if (!POWEROUT.on || !st) return undefined;
  return { since: st.since, decideAt: st.decideAt, goAt: st.goAt, wayM: st.wayM, overdueAt: st.overdueAt, fell: st.fell ? v3.copy(st.fell) : null, seen: st.seen.map(x => ({ ...x })), lookAt: st.lookAt, decided: st.decided, word: st.word, parties: st.parties, lost: st.lost,   // (the timeline is the check's and the debugger's: not kept)
    men: st.men.map(M => ({ i: g.guards.indexOf(M.gd), role: M.role, phase: M.phase, goal: v3.copy(M.goal), from: v3.copy(M.from), watch: M.watch ? v3.copy(M.watch) : null, tries: M.tries, stepT: M.stepT, heard0: M.heard0, thereT: M.thereT,
      calm: M.calm ? calmSave(M.calm) : null })) };
}
export function powerOutRestore(g: Game, s: PowerOutSnap | null | undefined) {
  if (!s) { STATES.delete(g); breakerSay(g); return; }
  const men: Man[] = [];
  for (const m of s.men) { const gd = g.guards[m.i]; if (!gd) continue;
    men.push({ gd, role: m.role, phase: m.phase, goal: v3.copy(m.goal), from: v3.copy(m.from), watch: m.watch ? v3.copy(m.watch) : null, tries: m.tries, stepT: m.stepT, heard0: m.heard0, thereT: m.thereT,
      calm: m.calm ? calmLoad(g, m.calm) : null }); }
  STATES.set(g, { since: s.since, decideAt: s.decideAt ?? Infinity, goAt: s.goAt, wayM: s.wayM, overdueAt: s.overdueAt, fell: s.fell ? v3.copy(s.fell) : null, seen: s.seen.map(x => ({ ...x })), lookAt: s.lookAt, men, decided: s.decided, word: s.word, parties: s.parties, lost: s.lost, events: [] });   // (decideAt: Infinity is null in a file)
  breakerSay(g);
}
