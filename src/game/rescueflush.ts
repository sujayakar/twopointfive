// THE MEN LEFT ALIVE ON A RESCUE COME FOR THE HOSTAGES (RESCUEFLUSH; rescue.ts tick() / captor() / clock() ask here while the switch is up). The
// maintainer, 10-18: "remaining enemies should try to flush you out and/or kill the hostage when loud".
//
//   WHAT WAS THERE   the captor's clock passed to a man of theirs only if he happened to be standing in the hostage room; once the room was the team's, the
//                    men alive elsewhere (the door man in the corridor, a roamer who had hunted the noise and lost it, Hard's man on his crossfire) stayed
//                    where the fight had left them. Nobody came back for the hostages and nobody pushed the team off them.
//   THE RULE, once the team has been LOUD (a shot, a bang, a boot of its own: rescue.ts noiseAt) and the ROOM HAS FALLEN (every man posted in it is down):
//     THEY COME   `afterSecs` later the nearest man of theirs outside the room says so on their net and goes for the hostages; every `gapSecs` the next
//                 one follows — the first by the room's main door, the second by its side door where the level has one (two ways at once: the flush), and
//                 so on in turn. Each is let go the way a roamer is let go to the noise (the fight AI's own hunt of his `lastKnown`, the room fight's
//                 'free'): his eyes, his grenades and his pursuit drills are his own, he stops and fights whoever he meets, and the walk is priced and
//                 gated by the covered lanes like any other. The lanes only CHOOSE his door (`lanesChoose`; lanes.ts laneCovers asked of the floor
//                 outside each door: his turn's door in their line of fire and the other not, he takes the other); nobody is kept back for being
//                 warned off — where he is sent is where the team's guns are, so a man kept back for a lane at those guns never came at all.
//                 Stopped short of a covered corridor on his way he is that rule's (he looks, goes round or throws as its men do), and while it
//                 holds him there no word takes him off it for what lies at that corridor's guns (laneBars).
//     AND AGAIN   a man who has come to rest with nobody in his sights (the room fight planted him again) is sent again `againSecs` after his last word.
//     THEY'RE GONE   the first of them to stand in the hostage room and find no hostage in it says so on the net, and from then on they go where a man
//                 taking people out of the building goes: the team's last loud thing and the way out (the team's own way in), in turn.
//     THE GUN ON HER   a man of theirs with a clear line to a living hostage inside `reachM` HAS her, as the man standing in the room always did
//                 (rescue.ts captor()): the clock is his. One who came for her is `killSecs` from turning his gun on her, then RESCUE.aimSecs from the
//                 round — the same act, broken the same ways (shot, blinded, staggered, grabbed, or one of the team standing in the line). So a
//                 hostage on her feet behind the team is safe only while nobody of theirs is left a line to her.
//   Never: a response from outside (another rule's), more health, a man who knows where the team is without having heard or seen it (the three places
//   he is sent are the room he was guarding, a noise his side heard, and the door the team came in by).
//
// Off (the module's default, the frozen stand) = the build as it was. On in the page (host/profile.ts pageSwitches; ?norescueflush) and on the shipped
// stand; TPS_RESCUEFLUSH=1 raises it on a frozen run. UNTUNED by play.
import { senseTool } from './sense';
import type { Vec3 } from '../math/vec';
import { v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Rescue } from './rescue';
import './squad';
import './guards';
import './lanes';
import { hasHim, OWN } from './owner';
import { RUST_PASS, RUST_SURFACE } from './crossing';

export const RESCUEFLUSH = {
  on: false,
  /** seconds from the room falling to the first man moving on it · between one man's word and the next man's · before a man who has come to rest is sent again */
  afterSecs: 4, gapSecs: 2.5, againSecs: 8,
  /** a man with a clear line to a living hostage inside this many metres has her · one who came for her is this long from turning his gun on her */
  reachM: 10, killSecs: 3,
  /** a man with one of the team in his sights this recently is fighting: not sent anywhere */
  busySecs: 2,
  /** smoke between him and her thinner than this hides her from him (the bar guards.ts puts between two AI men) */
  smokeK: 0.3,
  /** a man sent round has reached a mark of his way inside this many metres of it: on to the next, and from the side door in */
  atDoorM: 1.8,
};

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
type Goal = 'room' | 'side' | 'noise' | 'out';
interface Sent { at: number; n: number; goal: Goal; via: Vec3[] }

export class RescueFlush {
  /** when the room fell with the team loud (NaN: not yet) · when the next man is due · when one of them found the room empty (NaN: nobody has) · men sent so far */
  fellAt = NaN; nextAt = NaN; goneAt = NaN; count = 0;
  /** every man sent, his last word's time, how many times, where to, and the marks still ahead of him on a way round (`via`: the first is where he walks now) */
  readonly sent = new Map<Guard, Sent>();
  /** the men a covered lane has turned to the other door, each logged once ('flush:lane:<name>') */
  private readonly barred = new Set<Guard>();
  constructor(private g: Game, private R: Rescue) {}

  reset() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RescueFlush.reset', this, []); if (__r !== RUST_PASS) return __r as any; } } this.fellAt = NaN; this.nextAt = NaN; this.goneAt = NaN; this.count = 0; this.sent.clear(); this.barred.clear(); }
  /** the rule is up on this run */
  private get live(): boolean { const R = this.R, g = this.g; return RESCUEFLUSH.on && R.active && !R.result && g.aiEnabled && g.fight.on; }
  private alive(): Guard[] { return this.R.hostages.filter(h => h.state !== 'dead'); }

  /** HIS LINE ON HER: inside `reachM`, nothing that stops a round between his eyes and her chest (a pane that would stop it is a wall to this: collision.ts
   *  shotBlocked), and no smoke of the team's hiding her (the Rust game's: sense.ts senseTool).
   *  The distance, or Infinity. His facing and the light on her are not asked: he came knowing where she was kept. */
  private line(by: Guard, v: Guard): number {
    const g = this.g, K = RESCUEFLUSH, d = flat(v.char.pos, by.char.pos); if (d >= K.reachM) return Infinity;
    const e: Vec3 = [by.char.pos[0], 1.5, by.char.pos[2]], c = v.char.bones.chest ?? v3.add(v.char.pos, [0, 0.6, 0]);
    if (g.col.shotBlocked(e, c)) return Infinity;
    return senseTool(g, 1, [...e, ...c])[0] < K.smokeK ? Infinity : d;
  }
  /** THE GUN ON HER: the man of theirs, outside the hostage room, with a clear line to a living hostage inside `reachM` — the nearest to one (rescue.ts
   *  captor(), after the room's own men; `keep`: the man who has the clock keeps it while this still holds for him). Null with the switch off. */
  holder(keep: Guard | null): Guard | null {
    if (!this.live || isNaN(this.fellAt)) return null;   // (while the room's own men stand, the clock is theirs as it always was)
    const g = this.g, alive = this.alive(); if (!alive.length) return null;
    const has = (h: Guard): number => {
      if (h.state === 'dead' || h.held || g.time < h.dazzledUntil || hasHim(h, OWN.runner) || hasHim(h, OWN.morale)) return Infinity;
      let best = Infinity; for (const v of alive) best = Math.min(best, this.line(h, v));
      return best;
    };
    if (keep && has(keep) < Infinity) return keep;
    let who: Guard | null = null, bd = Infinity;
    for (const h of this.R.hostiles) { const d = has(h); if (d < bd) { bd = d; who = h; } }
    return who;
  }


}
