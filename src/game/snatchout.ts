// HOW A SNATCH ENDS (SNATCHOUT; snatch.ts asks here while the switch is up and the level's block names a van). The maintainer, having played it: "i was
// expecting to have to grab him and exfil him though (while my squad handles holding off the response force), not needing to clear the structure. like maybe we
// need to get him to a van near our infil".
//
//   WHAT WAS THERE   the house taken with him alive and in hand was the win, and he was held where he gave up: nowhere to take him.
//   THE RULE         tied, he is LED: a row on him (Space, 'on your feet — with me') and he gets up and follows whoever said it, on the civilians' own legs
//                    (civilians.ts hostageLead: the walk a rescue's hostages have, his wrists still behind him); the same row then reads 'stay here'. The
//                    level's block names the team's VAN by its way in (scenarios.snatch.van): the leader inside `sendM` of its open doors with him close
//                    behind and he walks the last steps himself, and the moment he is at them he is IN (off the floor) and the mission is WON — men of
//                    theirs alive in the house or not. In Sam's arm, or out cold in his hands, he is in once Sam stands at the doors.
//   LOST is as it was: out by a way of his own, or dead by anybody's round or grenade. Tied is still for good.
//   THE SQUAD (`hold`)  nobody is asked: from the word, the teammates who were with the leader cover the walk out BY BOUNDS — one pair plants (a hold that
//                    names nothing to watch, so cover.ts deals each man one of the doors and gaps a man could come at them by, no two the same), never in
//                    a doorway, on the leader's way out or on a mate; left `trailM` behind, it comes up at the squad's own pace while the pair that was
//                    with the leader plants in its place. While one planted man fights, or one has just gone down, the other's gun goes to where that is.
//                    A man the player has given an order since is the player's: the order stands and this lets go of him (the Assignment is not ours
//                    any more). Nobody in a fight is called off it.
//
// Off (the module's default, the frozen stand) = the build as it was. On in the page (host/profile.ts pageSwitches; ?nosnatchout, ?nosnatchhold) and on the
// shipped stand; TPS_SNATCHOUT=1 raises it on a frozen run. UNTUNED by play.
// Per-player state (habit: 'which player, not the player'): the one who gave the word is kept as a character, never read off Game.player after the press.
import type { Vec3 } from '../math/vec';
import { v3 } from '../math/vec';
import type { Game, Guard, Player } from './game';
import type { Character } from './character';
import type { Snatch } from './snatch';
import type { Assignment } from './orders';
import { hostageLead, civilianLedTo } from './civilians';
import { RUST_PASS, RUST_SURFACE } from './crossing';

export const SNATCHOUT = {
  on: false,
  /** the squad covers the walk out by bounds, unasked (false: every man does what he was last told) */
  hold: true,
  /** the row on him: its marker shows inside `markerM`, the word can be given inside `reachM` */
  markerM: 2.4, reachM: 2.2,
  /** led, he stops this far behind whoever leads him */
  gapM: 1.3,
  /** the leader this near the van's doors with him inside `sendHimM` of them: he walks the last steps himself · he is IN this near the spot at the doors */
  sendM: 3.5, sendHimM: 8, inM: 1.1,
  /** in Sam's arm or in his hands: Sam this near the doors and he is in */
  carryM: 2.2,
  /** the bounds: a pair left holding comes up once the leader is this far off · a pair coming up has arrived this near him */
  trailM: 12, closeM: 5,
  /** a planted man's gun stays on a mate's fight, or on where the round that put one down came from, this long after the last of it */
  turnSecs: 4,
  /** where a man plants: this clear of the leader's way out · of a doorway's or a gap's width · and no farther than this from where he stood */
  laneM: 1.0, offM: 0.7, spotM: 3.4,
};

/** the player a row is built for and a press is read from (one today: the place a second would be asked for) */
const who = (g: Game): Player => g.player;

export interface SnatchOutSnap { lead: number | null; led: boolean; sent: boolean; inAt: number }

export class SnatchOut {
  /** whom he follows (null: he is down where he was tied, or put) */
  lead: Character | null = null;
  /** the word has been given at least once since the staging · he has been sent the last steps to the van's doors */
  led = false; sent = false;
  /** the game time he was put in the van (−1: he is not in it) */
  inAt = -1;
  /** the men planted by the bounds, each with the order it gave him (his order being another object = the player's order has him: let go) */
  private jobs = new Map<Guard, Assignment>();
  /** the pair coming up (null: nobody is) · he is being moved (the bounds are running) · bounds made, for the tally */
  private coming: string | null = null; private moving = false; bounds = 0;
  /** every man's order as this last saw it (another object = somebody else has ordered him since) · the men so ordered: theirs, until he is next moved off */
  private last = new Map<Guard, Assignment | null>(); private told = new Set<Guard>();
  /** where a planted man's fight is, or where the round that put one down came from: the rest of the planted men turn onto it until `until` · what each
   *  turned man's order had him on before */
  private hot: { at: Vec3; until: number } | null = null; private turned = new Map<Guard, { watch: Vec3 | null; face: number | null }>();
  constructor(private g: Game, private S: Snatch) {}

  /** a fresh staging (snatch.ts stage / drop): nobody led, nobody planted */
  reset() { this.lead = null; this.led = false; this.sent = false; this.inAt = -1; this.jobs.clear(); this.last.clear(); this.told.clear(); this.coming = null; this.moving = false; this.bounds = 0; this.hot = null; this.turned.clear(); }
  /** the spot at the van's open doors (null: the switch is off, or this level's block names no van) */
  get van(): Vec3 | null { const N = this.S.block; return SNATCHOUT.on && N?.van ? N.van.pos : null; }
  /** this level's snatch ends in the van (asked before he is staged too: qrf.ts) */
  get on(): boolean { return !!this.van; }
  /** …and it is running */
  get ruled(): boolean { return !!this.van && this.S.active; }
  /** he can be given the word: tied, on his feet or down, in nobody's arm */
  private get free(): boolean { const S = this.S, gd = S.man; return this.ruled && !!gd && !S.end && S.tied && gd.state !== 'dead' && !gd.held; }

  /** the word, for a caller that is not the key (a script, the console): he follows `by`, or (null) stays where he stands. False = refused */
  word(by: Character | null): boolean { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('SnatchOut.word', this, [by]); if (__r !== RUST_PASS) return __r as any; } }
    if (!this.free) return false;
    const g = this.g, gd = this.S.man!;
    hostageLead(g, gd, by, SNATCHOUT.gapM);
    this.lead = by; this.sent = false; this.moving = false;   // (each word is a fresh start for the bounds: a man ordered since the last one is asked again)
    if (by) this.led = true;
    this.S.note(by ? 'lead' : 'stay');
    return true;
  }

  /** the debugger panel's switch thrown mid-run: with the rule off nobody is led and nobody planted (what is on the floor is the old snatch's from there) */
  switched() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('SnatchOut.switched', this, []); if (__r !== RUST_PASS) return __r as any; } } if (SNATCHOUT.on) return; const gd = this.S.man; if (gd && this.lead) hostageLead(this.g, gd, null, SNATCHOUT.gapM); this.lead = null; this.sent = false; this.letGo(); }

  // ---------------------------------------------------------------- the squad, by bounds
  private fighting(gd: Guard): boolean { return (!gd.script && gd.state === 'alert') || !!this.g.fight?.engaged(gd); }
  /** an order of this module's, and the object it is from then on (what `last` and `jobs` know him by) */
  private mine(gd: Guard, spec: Parameters<Game['orders']['assign']>[1]): Assignment { const O = this.g.orders; O.assign(gd, spec); const A = O.of(gd)!; this.last.set(gd, A); return A; }
  private drop(gd: Guard) { const A = this.jobs.get(gd), was = this.turned.get(gd); if (A && was) { A.watch = was.watch; A.face = was.face; } this.jobs.delete(gd); this.turned.delete(gd); }
  private rejoin(gd: Guard) {
    const O = this.g.orders, A = this.jobs.get(gd); this.drop(gd);
    if (A && O.of(gd) === A && gd.state !== 'dead') this.mine(gd, { order: 'fallIn', post: null, follow: 9, face: null, stance: 'lowReady', upper: 'none', going: 'moving up', there: 'with you' });
  }
  /** it is over, or nobody is moving him: every man still ours goes back to the file (snatch.ts calls this at either end: nothing here ticks after it) */
  letGo() { for (const gd of [...this.jobs.keys()]) this.rejoin(gd); this.coming = null; this.hot = null; this.moving = false; }
  /** a pair looks after itself: while one planted man fights, or one has just gone down, the others' guns go to where that is — and back to their own
   *  openings `turnSecs` after the last of it (a man in his own fight is the fight's) */
  private turn() {
    const g = this.g, K = SNATCHOUT;
    for (const gd of this.jobs.keys()) { const t = g.fight?.opOf(gd)?.tgt; if (t && t.state !== 'dead') this.hot = { at: v3.copy(t.char.pos), until: g.time + K.turnSecs }; }
    if (this.hot && g.time >= this.hot.until) this.hot = null;
    const at = this.hot?.at ?? null;
    for (const [gd, A] of this.jobs) {
      if (this.fighting(gd)) continue;
      const was = this.turned.get(gd);
      if (at) { if (!was) this.turned.set(gd, { watch: A.watch, face: A.face }); A.watch = [at[0], 1.1, at[2]]; A.face = v3.yawTo(A.post ?? gd.char.pos, at); }
      else if (was) { A.watch = was.watch; A.face = was.face; this.turned.delete(gd); }
    }
  }

  /** snatch.ts objective(): the row once he is in hand — where to take him (null: not this rule's to say) */
  objective(): { text: string; pos: Vec3 | null } | null {
    const S = this.S, gd = S.man; if (!this.ruled || !gd || S.end || !S.inHand) return null;
    const moving = !!this.lead || !!gd.held || who(this.g).dragging === gd;
    return { text: `objective ▸ get ${gd.civ!.name} to ${S.block!.van!.name}`, pos: moving ? v3.copy(this.van!) : [gd.char.pos[0], 0, gd.char.pos[2]] };
  }

  /** the state a save would hold (habit: new state the game depends on has its pair beside it; snatch.ts carries it in its own slice — which nothing loads
   *  today: a checkpoint is refused with a squad on the floor). The bounds are not carried: a load is every man back in the file, and the next half second plants again */
  snapshot(): SnatchOutSnap { return { lead: this.lead ? this.lead.id : null, led: this.led, sent: this.sent, inAt: this.inAt }; }
  restore(s: SnatchOutSnap | undefined) {
    this.reset(); if (!s || !this.ruled) return;
    const g = this.g, pl = who(g), gd = this.S.man!;
    const by = s.lead === null ? null : pl.char.id === s.lead ? pl.char : g.guards.find(x => x.char.id === s.lead)?.char ?? null;
    if (by && this.free) { hostageLead(g, gd, by, SNATCHOUT.gapM); this.lead = by; }
    this.led = s.led; this.inAt = s.inAt;
    if (s.sent && this.lead) { this.sent = true; civilianLedTo(gd, this.van); }
  }
}
