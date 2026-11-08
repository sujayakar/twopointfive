// A LONE DEFENDER RADIOS FOR A BUDDY (the maintainer, 10-07: "and then if someone is alone in a room, they could radio for a buddy"). Behind `BUDDY.on`
// (shipped OFF — see the measured note below; off: nothing here runs, no die is drawn, and the room fight is roomfight.ts run for run as it was — its seams are `buddyTick` at the tail of tick(),
// `buddyStop` in stop() and one clause in rally()).
//
//   THE CALL. A scenario defender holding a room with nobody of his side left in it — his room mates down, or gone to a fight elsewhere (`staged`: also a man
//     who was posted alone, once the floor has gone loud in his own ears) — and nobody of the team in his sight these `quiet` seconds, says so on their net
//     after his own beat — and not while his own room is being hit (`calm`): 'I'm on my own in here — somebody get to me!'. That is `g.say(…, radio)`, the line every other defender call is ('<callsign>
//     (radio): …' in the log): the player hears of a room held by one man.
//   THE ANSWER. The nearest FREE colleague by the way a man walks answers after a beat of his own ('on my way to you — hold on!') and comes: a man holding a
//     post with nobody in his sight, whom nobody else has (a squad about to give its word, a squad whose men stay in its rooms, the harder defenders' press,
//     the rescue's men with the hostages), inside `maxM` of path, on a way that does not pass where the team was last seen or heard (`clearM`). Who is free is
//     situation, not dice: a man alone himself may come (`loners`: the two pair up in the caller's room, and — `emptyRoom`, shipped on — so may a squad's last man in one of its
//     rooms or a man set on a door he has wedged: his room is left empty behind him), one of a room that keeps two men holding it behind him may come, one of a PAIR stays (`fromPair` off: he would
//     leave his own mate calling). Nobody free: the call stands `again` seconds and the first man to come free answers it.
//   THE WALK is pressure.ts's leg (`run`: a script with his own senses under it — roomfight.ts tickMove: an enemy seen close stops him to fight), his boots
//     filed for the team's ears (`boots`), run again in legs when the room fight's ten-second move plants him short. The caller down or gone before he is
//     there: 'he's not answering — going back!', and back to the post he left.
//   MEANWHILE the caller waits for him: he does not walk off to a fight next door by himself (roomfight.ts rally reads `OWN.buddy`), nor does the pair for
//     `again` seconds after it is made. That is the stock rally only: his squad's word (enemysquads.ts) or a press (pressure.ts) still takes him, and the
//     man coming finds him where he stops, if there is still a way that is not past the team.
//   THERE he is the room fight's holder of that room ('with you — two on this room!'), told what the caller knows of its doors (the caller's evidence is his),
//     put on a door the caller is not watching.
//
// What the counts mean (`buddyStats`, tools/qa/playthrough.ts files them per run): `calls` = the line said · `answered` = a colleague set off · `arrived` =
// he planted in the caller's room with the caller up · `metOnWay` = the team stopped him first · `turnedBack` = the caller was down or gone before he got
// there · `diedOnWay` · `noOne` = a call nobody could answer inside `again` · `secs` / `metres` = call to arrival, and the way walked, summed over arrivals.
// (src/game is one import cycle: nothing below is built from another game module's exports at module load.)
import { SideMap } from '../math/side';
import { Vec3 } from '../math/vec';
import type { Game } from './game';
import type { RoomFight } from './roomfight';
import { pressKeepsHome } from './pressure';
import { DIGIN, digInSet } from './digin';
import { releaseMan, hasHim, ownerWitness, OWN } from './owner';

export const BUDDY = {
  /** the switch: off = none of this file runs. SHIPPED OFF, the maintainer's call: on, a scripted team keeps clearly MORE men on the compound and the barracks
   *  and somewhat more on the block (seeds the knobs never saw) — a man who answers a call holds a room where, left alone, he went to the fight. Numbers:
   *  rebuild/telegraphing/buddy.md */
  on: false,
  /** a man POSTED alone calls too, once a loud thing has reached his own ears inside `loudSecs` (false = only a man whose room mates are down or gone) */
  staged: false, loudSecs: 30,
  /** s alone before he calls · s since the team was last in his sight before he may (a man in a fight does not radio for company) */
  beat: [2, 4.5] as [number, number], quiet: 3,
  /** s since a loud thing last reached his ears from INSIDE his room or from one of its doors (inside `calmM` of it) before he may call: a room being hit is a
   *  fight to come to — the rally's business — not a man asking for company (0 = he calls whatever is at his door) */
  calm: 6, calmM: 4,
  /** the colleague's beat between the call and setting off */
  answer: [1, 2.5] as [number, number],
  /** the way he will walk at most (m) · he takes no way that passes this near where the team was last seen or heard (0 = any way) */
  maxM: 30, clearM: 3,
  /** who may answer: a man alone in his own room (the two pair up in the caller's) · one of a pair (he leaves his mate alone) — one of three or more always may */
  loners: true, fromPair: false,
  /** a squad's last man in one of its rooms, or a man set on a door he has wedged, may answer as well: his room is left empty behind him and the two hold the
   *  caller's (off = those two stay: on a squads level nearly every lone man is one, and 192 of 255 calls on the warehouse then went unanswered) */
  emptyRoom: true,
  /** s a call nobody could answer stands before he may call again · calls a man makes at most */
  again: 20, calls: 2,
};

type Def = RoomFight['defs'][number];
interface Man { had: boolean; callAt: number; calledT: number; calls: number; open: boolean; lookT: number; /** when his last errand as somebody's buddy ended */ doneT: number; /** the pair just made stays put until then */ keepT: number }
interface Errand { caller: Def; buddy: Def; phase: 'beat' | 'run' | 'back'; goAt: number; spot: Vec3; home: Vec3; tries: number; stepT: number; t0: number; L: number }
const newStats = () => ({ calls: 0, answered: 0, arrived: 0, metOnWay: 0, turnedBack: 0, diedOnWay: 0, short: 0, noOne: 0, secs: 0, metres: 0 });
interface State { men: Map<Def, Man>; errands: Errand[]; stats: ReturnType<typeof newStats>; stale: boolean }
const STATES = new SideMap<Game, State>('buddy.STATES');
ownerWitness<Game>('buddy', (gd, g) => BUDDY.on && !!STATES.get(g)?.errands.some(e => e.caller.gd === gd));

const alive = (D: Def) => D.gd.state !== 'dead';
/** his own wits about him, and nobody else's man: alive, holding a post, not in Sam's arm, not blinded or reeling, not pinned to his room by a stager (the
 *  rescue's men with the hostages: their rally clock is parked at Infinity) */
const own = (g: Game, D: Def) => alive(D) && D.mode === 'hold' && !D.gd.held && g.time >= D.gd.dazzledUntil && g.time >= D.gd.staggerUntil && D.rallyAt !== Infinity;
/** the men of his side holding the room he holds, himself not counted (a man off on the fight AI's hunt has left it) */
const mates = (F: RoomFight, D: Def) => F.defs.filter(X => X !== D && alive(X) && X.mode !== 'free' && !!D.room && X.room === D.room);

/** may this man answer a call: his own man, nobody in his sight, on nobody else's errand (`mine` = the errand he is on his beat for), and not the one who
 *  would leave a mate alone — a room keeps two men FIRMLY holding it (not walking a leg, not about to go to somebody themselves) behind the one who goes */
function free(g: Game, F: RoomFight, st: State, X: Def, mine: Errand | null = null): boolean {
  const gd = X.gd;
  if (!own(g, X) || g.time - gd.lastSeenT < BUDDY.quiet || st.errands.some(e => e !== mine && (e.buddy === X || e.caller === X))) return false;
  if (g.time - (st.men.get(X)?.doneT ?? -100) < BUDDY.again) return false;   // (he has only just come to somebody: not off again on the next call)
  if (hasHim(X.gd, OWN.press) || pressKeepsHome(g, X) || gd.staysInRooms || hasHim(gd, OWN.squadWord) || hasHim(gd, OWN.stand)) return false;   // (laststand.ts, default off: a man of the last stand answers nobody's call)
  const set = digInSet(g, gd);
  if (set && (DIGIN.stay || DIGIN.reachM > 0)) return false;                  // (digin.ts's own rules for who holds his room, when they are on, keep him)
  const all = mates(F, X), firm = all.filter(m => m.mode === 'hold' && !st.errands.some(e => e.buddy === m)).length;
  if (firm >= 2) return true;
  if (all.length) return BUDDY.fromPair && firm >= 1;
  return BUDDY.loners && (BUDDY.emptyRoom || (!hasHim(gd, OWN.squad) && !set));   // (alone himself: the two pair up in the caller's room; with `emptyRoom` off a squad's last man in a room does not leave it empty, nor does a man set on a wedged door)
}



/** …and for a moment after, the two have just paired up: he waits, and no rally clock is left running to send him the frame he is let go. A time, so asked as it is */
export function buddyPaired(g: Game, D: Def): boolean { return BUDDY.on && g.time < (STATES.get(g)?.men.get(D)?.keepT ?? -100); }
/** the fight is over (RoomFight.stop): every call forgotten. The counts stay for whoever reads them after the run; a new fight starts them again */
export function buddyStop(g: Game) { const st = STATES.get(g); if (!st) return; st.men.clear(); st.errands = []; st.stale = true; for (const gd of g.guards) releaseMan(gd, OWN.buddy, g.time); }
/** the counts of this run (tools/qa/playthrough.ts files them per run) */
export function buddyStats(g: Game): Record<string, number> { const s = STATES.get(g)?.stats ?? newStats(); return { ...s, secs: +s.secs.toFixed(1), metres: +s.metres.toFixed(1) }; }
export function buddySummary(g: Game): string {
  if (!BUDDY.on) return 'off';
  const st = STATES.get(g); if (!st) return 'on · nothing yet';
  const s = st.stats;
  return `${s.calls} call${s.calls === 1 ? '' : 's'} · ${s.answered} answered · ${s.arrived} arrived · ${st.errands.length} on the way`;
}
