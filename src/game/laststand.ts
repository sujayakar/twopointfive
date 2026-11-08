// THE LAST DEFENDERS (the maintainer, 10-08, playing the compound: "it also took a while to find the last two stragglers. ready or not has this problem too for
// the "bring order to chaos" objective. curious how we can make this more fun while still having the peril of barricaded defenders"). Two switches, both
// `regroup` ships OFF, `callOut` ON (10-08: telling only, so the run is the run with both off) — with both off nothing here runs, no die is drawn and the room fight is run for run what it was (the seams: `lastStandTick` at the tail of
// RoomFight.tick, `lastStandStop` in stop(), one clause each in rally(), lost(), the dig-in's gate / wedge rule / stand-down, the squads' home(), the press's
// consider(), and the room clear's row, which the Rust game words).
//
//   WHEN. A room clear (not the hostage rescue), once `few` (3) or fewer of its defenders are up and no more than a third of the staging (the compound's
//     eleven: three; the block's eight: two; the office's five: the last man). Until that frame a run with
//     either switch on IS the run with both off: nothing is drawn or written before it, so a tally pairs by seed up to the endgame.
//   THE RADIO CHECK (both switches). The survivors do not know they are the last by telepathy: one of them who is fit and has had nobody of the team in his
//     sight these `quiet` seconds asks the net who is still up. Whoever is fit and calm answers on his own beat; a man in a fight does not answer and is not
//     counted. What the check closes with — who answered — is ALL the survivors know of each other. It is asked again every `every` seconds (`again` once they
//     have made their stand): a man who was busy the first time answers the next.
//   `callOut` — CALL OUT (telling only: nothing a man's AI, the dice of the simulation or a digest reads is written). Every answer says where the man is by
//     the level's own rooms ('still up — in the kitchen'), and the one who asked says where he is. A man nobody answers knows he is the last, and says that
//     and his room ('…if anyone can hear me, I'm in the office!'). Their net is on the HUD's radio strip (cues.ts), so the player hears where to go — said by
//     men to each other, never to the team. (`regroup` says the same places: the one who picks the room has to know where they are. `callOut` is that
//     telling half alone.)
//   `places` — THE LINE IS TRUE OF WHERE HE STANDS (on; off = the lines as they were, word for word — `callOut` stays telling only either way). A man in floor
//     nobody holds — the compound's front hall, a corridor, a yard — is 'in the front hall' by the level's own name for it (dialogue.ts placeIn: the structure's
//     rooms, then `scenarios.places`), not 'outside' the nearest held room through a solid wall; a man on a room's threshold is in that room; and the HUD's
//     marker goes to the place he named when it is no room of the list. A man who has left the place he named says so once he has stopped ('moved — I'm in the
//     corridor now'); one who has gone after the team says he is out of it, and the marker comes off. Nobody is pinned to the room he named: the line follows
//     the man.
//   `regroup` — REGROUP AND DIG IN LOUDLY. At the close of a check the one who asked picks the room to make the stand in, by situation and never a die: a
//     room one of the answering men is holding, or the post one of them was staged on (the team not known to be in it, never entered) — the one MOST of
//     them are in or can reach, then the one with the most doors already wedged. A man can come if
//     he holds a post, nobody else has him (a squad about to give its word or with him out on a leg, the press, a door he is already working on), a squad
//     whose men stay in its rooms keeps to those, and his way there is at most `maxM` and does not pass within `clearM` of where the team was last seen or
//     heard; ties go to the most men already there, the farthest from the team, the least walking. He ORDERS it on the net, naming the room ('everybody to
//     the office — we hold it there!'), the men who come say so and run (pressure.ts `run` + `boots`: heard, and stopped by the room fight's own rules — an
//     enemy seen close and he fights where he is), a man with no way says that and where he digs in instead. In the room every man digs in through digin.ts —
//     furniture hauled and wedged under the doors (scrapes and thuds, any door of that room, squads' barricade lists or not), a spot off the door's line, set
//     on it and STEADY when it is booted — and they stay set: no standing down after the dig-in's `holdSecs`, no giving the room up through a door (cover
//     inside it still), no rally to a noise next door. When they are set one of them says so; from then one of them shores the wedge up every `thump` seconds
//     (a thud at the door, heard like any shoved furniture) and the check goes out with the room's name. A lone last man digs in where he stands;
//     a man out hunting (the room fight's 'free') is nobody's to order: he answers, and is counted, and is taken in once he holds a post again.
//     A man who arrives is the stand room's man for the room clear (roomclear.ts `rehome`): the room he left can clear behind him.
//   THE HUD. Under either switch the objective's marker sits on the first room still held that their net has NAMED, and the list brackets those rooms — what
//     the player has been told, not where the men truly are.
//   The die here is its own (mulberry32 off the room clear's seed and attempt): never Math.random. (digin.ts draws its own beats from Math.random as it always
//   has — once `regroup` has ordered a dig-in the run is a different run; `callOut` alone never is.)
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import { type Die } from '../math/dice';
import type { Game, Guard } from './game';
import type { RoomFight } from './roomfight';
import './pressure';
import './cues';
import './dialogue';
import './digin';
import './netquiet';
import './enemysquads';
import { claimMan, releaseMan, ownerWitness, OWN } from './owner';

export const LASTSTAND = {
  /** REGROUP AND DIG IN LOUDLY (shipped off): the last defenders gather in one room on a spoken order, wedge it and hold it to the end */
  regroup: false,
  /** CALL OUT (shipped ON since 10-08; telling only): the last defenders name their rooms on their net. `?callout=0`, the panel's row or `LASTSTAND=nocallout` in the harness's environment = off */
  callOut: true,
  /** THE LINE IS TRUE OF WHERE HE STANDS (telling only; rides on either switch above): a man is named by the level's own word for the floor he is on — a room
   *  of the structure, or a place nobody holds (dialogue.ts placeIn) — the marker goes there, and he says again when he has moved. Off = the lines as they were */
  places: true,
  /** `places`: he has been out of the place he named this long (s), and has stood still this long (s), before he says where he is now; no man says it more
   *  often than every `movedGap` s */
  movedSecs: 3, stillSecs: 2, movedGap: 8,
  /** 'most are down': this many defenders or fewer are up — and never more than a third of the staging (eleven: three; eight: two; five: the last man) */
  few: 3,
  /** nobody of the team in his sight these seconds: a man may ask, answer, be ordered */
  quiet: 3,
  /** seconds from 'most are down' to the first radio check [min, max] */
  first: [3, 6] as [number, number],
  /** seconds between checks, and between them once the stand is made */
  every: 22, again: 30,
  /** an answer's beat after the check [min, max], and between two answers (their net's strip takes a line of a kind every three seconds) */
  answer: [1.2, 2.2] as [number, number], between: [3.2, 3.9] as [number, number],
  /** the one who asked listens this long after the last answer is due [min, max] (what he says then is a line of the answers' kind: past the strip's three seconds) */
  listen: [3.2, 3.8] as [number, number],
  /** a man comes if his way is at most this long (m) and does not pass this near where the team was last seen or heard (m) these `knownSecs` */
  maxM: 38, clearM: 4, knownSecs: 30,
  /** his beat between the order and setting off [min, max]; the way is run in at most this many legs (the room fight's move is ten seconds long) */
  go: [1.3, 2] as [number, number], legs: 5,
  /** his beat between the order and starting on the room's doors [min, max] */
  dig: [0.6, 1.6] as [number, number],
  /** once set: seconds between two thuds at a wedged door [min, max], and how loud (a hauled chair's scrape is 0.5) */
  thump: [9, 14] as [number, number], thumpLevel: 0.6,
};
/** `?callout` (and `=0`) sets the switch for the visit; the harness sets it by `LASTSTAND=callout`
 *  in the environment (tools/qa/playthrough.ts's --set reaches a fixed list of tables) */
try { const q = new URLSearchParams(location.search); if (q.has('callout')) LASTSTAND.callOut = q.get('callout') !== '0'; if (q.has('calloutplaces')) LASTSTAND.places = q.get('calloutplaces') !== '0'; } catch { /* no page (the harness) */ }
try { const e = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.LASTSTAND; if (e) { const w = e.toLowerCase().split(/[\s,+]+/); if (w.includes('regroup')) LASTSTAND.regroup = true; if (w.includes('callout')) LASTSTAND.callOut = true; if (w.includes('nocallout')) LASTSTAND.callOut = false; if (w.includes('places')) LASTSTAND.places = true; if (w.includes('noplaces')) LASTSTAND.places = false; } } catch { /* no process (the page) */ }

type Def = RoomFight['defs'][number];
type Room = NonNullable<Def['room']>;
type LineKind = 'check1' | 'checkN' | 'checkStand' | 'checkLoneAt' | 'ansAt' | 'ansFreeAt' | 'fewAt' | 'lastAt' | 'lastGo'
  | 'movedAt' | 'movedFree'
  | 'order' | 'orderFrom' | 'orderStay' | 'orderShut' | 'orderOut' | 'go' | 'cant' | 'in' | 'short' | 'set' | 'setOpen' | 'setLone' | 'setLoneOpen';

interface Mover { D: Def; phase: 'beat' | 'run'; goAt: number; spot: Vec3; tries: number; stepT: number; t0: number; /** he has said where he is going already */ quiet: boolean }
interface Stand {
  /** the room they hold (null: nobody could reach anybody — each man digs in where he is) */ room: Room | null; id: string | null; the: string; anchor: Vec3; t: number;
  /** in the room · told to hold where they are · on their way */ in: Set<Def>; out: Set<Def>; movers: Mover[];
  setSaid: boolean; setT: number; thumpAt: number; lookT: number; orders: Map<Def, number>; homed: Set<Def>;
}
interface Check { caller: Def; t: number; closeAt: number; due: { D: Def; at: number }[]; got: Def[] }
export interface LastStandSaid { t: number; who: string; kind: string; text: string; /** where he stood as he said it (x, z): for the checks — nothing in the game reads it */ at: [number, number] }
const newStats = () => ({
  /** the endgame started (game clock; NaN = it never did) */ t0: NaN, /** defenders up then */ up0: 0,
  checks: 0, answers: 0, /** men up who did not answer, summed over the checks */ silent: 0, lastMan: 0,
  /** stands made · of them in one room with two or more · men sent (= arrived + stopped short + met on the way + down on the way + called off + still on their way) · no way */
  stands: 0, gathered: 0, sent: 0, arrived: 0, short: 0, met: 0, diedOnWay: 0, /** called off before he set out */ dropped: 0, /** sent without a 'moving' line (he has said where he goes, or goes back in) */ quiet: 0, noWay: 0,
  /** `places`: 'moved' lines said */ moved: 0,
  digOrders: 0, /** game clock of the order and of 'we're set' (NaN = never) */ standT: NaN, setT: NaN, /** men in the stand room when it was said set */ setMen: 0, thumps: 0, lines: 0,
});
interface Later { at: number; t0: number; gd: Guard; kind: LineKind; text: string; cls: number; then?: () => void; /** `places`: the line's wording and what fills it, asked again as it is said */ tpl: string; redo?: () => { vars: Record<string, string>; then?: () => void } }
interface State {
  die: Die; nextAt: number; check: Check | null; known: Set<Def>; named: Map<Def, string>; stand: Stand | null;
  /** `places`: the place each of them last named that is no room of the structure, and what every named man said last (to say again when it is no longer true) */
  spots: Map<Def, Spot>; at: Map<Def, Said>; loneSaid: boolean; fewN: number; told: Set<Def>;
  said: LastStandSaid[]; stats: ReturnType<typeof newStats>; stale: boolean;
  /** the post each of them was staged on (room id), and the staging it was read off (seed:attempt) */ post: Map<Guard, string>; key: string;
  /** the lines: the next variant of each kind (two men never say the same words back to back), when a line of each class may next go out, and the lines waiting for that */
  rot: Map<LineKind, number>; clsAt: number[]; later: Later[];
}
/** a named place that is no room of the structure: its name as the list shows it ('front hall'), and where the marker goes (the middle of its floor) */
interface Spot { name: string; pos: Vec3 }
/** what a man last named: `key` the room's id or the place's name, `the` as he said it; since when he has been somewhere else (NaN: he has not), where and
 *  since when he has stood still, when he last said he moved */
interface Said { key: string; the: string; offT: number; stillAt: Vec3; stillT: number; saidT: number }
const STATES = new SideMap<Game, State>('laststand.STATES');
ownerWitness<Game>('stand', (gd, g) => { if (!LASTSTAND.regroup) return false; const S = STATES.get(g)?.stand; return !!S && ofStand(S, gd); });


/** the strip takes one line of a class every few seconds and DROPS the rest (cues.ts `gaps`, whoever spoke last — the squads' and the dig-in's lines too): a
 *  line that would land inside that waits its turn here, and is said by the same man if he is still fit to say it. `then` = what the line makes true for the
 *  HUD (the room it names): it happens when the line is said, never for a line that was not */
const fill = (tpl: string, vars: Record<string, string>) => tpl.replace(/\{(\w+)\}/g, (_m, k: string) => vars[k] ?? '');
const name = (st: State, D: Def, id: string | null) => { if (id) st.named.set(D, id); else st.named.delete(D); st.spots.delete(D); st.at.delete(D); };




/** WHO THE STAND HAS (src/game/owner.ts): in the room, on his way to it, or told to hold where he is. The dig-in runs for him on any level, wedges any door of his room and does not stand down; the squads do not send him, the press does not take him */
const ofStand = (S: Stand, gd: Guard) => { for (const D of S.in) if (D.gd === gd) return true; for (const D of S.out) if (D.gd === gd) return true; return S.movers.some(m => m.D.gd === gd); };
function standSay(g: Game): void { const S = STATES.get(g)?.stand; for (const gd of g.guards) { if (S && ofStand(S, gd)) claimMan(gd, OWN.stand, g.time); else releaseMan(gd, OWN.stand, g.time); } }
/** the fight is over (RoomFight.stop): everything forgotten but the counts and the lines, which stay for whoever reads them after the run */
export function lastStandStop(g: Game) { const st = STATES.get(g); if (!st) return; st.check = null; st.stand = null; standSay(g); st.known.clear(); st.named.clear(); st.spots.clear(); st.at.clear(); st.later.length = 0; st.stale = true; }
export function lastStandStats(g: Game): Record<string, number> { const s = STATES.get(g)?.stats ?? newStats(); const r: Record<string, number> = {}; for (const [k, v] of Object.entries(s)) r[k] = Number.isFinite(v) ? +v.toFixed(2) : -1; return r; }
/** the endgame as it went, for the tallies and the probes: the counts, the lines their net said and where the stand is */
export function lastStandReport(g: Game): { on: string; stats: Record<string, number>; room: string | null; in: string[]; out: string[]; moving: string[]; known: string[]; named: string[]; said: LastStandSaid[] } {
  const st = STATES.get(g), S = st?.stand ?? null, cs = (a: Iterable<Def>) => [...a].map(D => D.gd.callsign);
  return { on: [LASTSTAND.regroup ? 'regroup' : '', LASTSTAND.callOut ? 'callOut' : ''].filter(Boolean).join('+') || 'off', stats: lastStandStats(g), room: S?.id ?? null,
    in: S ? cs(S.in) : [], out: S ? cs(S.out) : [], moving: S ? S.movers.map(m => m.D.gd.callsign) : [], known: st ? cs(st.known) : [], named: st ? [...new Set([...st.named.values(), ...[...st.spots.values()].map(x => x.name)])] : [], said: st ? st.said.slice() : [] };
}
export function lastStandSummary(g: Game): string {
  const st = STATES.get(g); if (!LASTSTAND.regroup && !LASTSTAND.callOut) return 'off'; if (!st || st.stale) return 'on — waiting for the last few';
  const S = st.stand, T = st.stats;
  return `${T.checks} check${T.checks === 1 ? '' : 's'} · ${st.known.size} known up` + (S ? ` · stand ${S.room ? `in ${S.the}` : 'each where he is'}: ${S.in.size} in, ${S.movers.length} coming, ${S.out.size} out${S.setSaid ? ' · set' : ''}` : '');
}
