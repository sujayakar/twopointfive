// QUIET ORDERS FOR ONE PARTNER (two person stealth): what Sam can tell the one AI teammate while the pair is still unseen, in the squad order menu's own grammar
// (orders.ts: X / the middle button opens the menu on what the cursor is on, a digit 1‥5 picks, B gives the target's first row at once, F is the word).
//
//   THE SWITCH. `PAIR.on`, OFF by default (`?pairorders` for the visit, Tab ▸ Sandbox ▸ Team feel; the harness leaves it off). Off — or with a squad of more
//   than `PAIR.maxSquad` men, or none — nothing here is on the menu, a person is not a target, no built-in row is touched and the two seams in the room fight
//   return on their first line: the game as it was (probes.ts pair-identity; the playthrough digests against the base).
//
//   A PERSON IS A TARGET. With the cursor on a defender the team can see (not one the fog of war hides), the menu opens ON HIM, as it does on a door:
//     1  take him · on your mark   the partner works round to the man's BACK unseen (stalk: the floor in front of every defender priced all but shut), stops a
//                                  few paces short, says 'set.' and WAITS. The MARK is F — or Sam's own hands going onto a man (his takedown, his grab:
//                                  `markOnHands`), so two men go down on one beat. On the mark he closes the last paces and puts the man out cold, quietly.
//                                  F before he is set: he goes as soon as he can get there.
//     2  watch him                 where he stands, gun on the man, no round while the man has not noticed anybody
//     3  fall in                   the built-in row ('on me')
//     4  hold position             the built-in row ('wait here')
//     5  take him now              the same job with no waiting: the partner picks his own moment (on the heels of the man's glance)
//   and on FLOOR, while the squad is silent (silent.ts: with no silent order in the build there is no such row):
//     1  move there · hold in the dark   in place of 'move there': the dark floor at, or nearest, the point — crouched, gun low. No dark floor within `darkM`:
//                                  refused, 'nowhere dark there.' (`darkRefuse` off: he goes anyway and says it is lit)
//   AN ORDER THAT CANNOT BE DONE IS REFUSED WITH A CUE — greyed on the menu with the reason, and said by the man when given by B: no way to the man ("can't get
//   there."), no floor at his back ('no room at his back.'), the man already down, the man already roused ("he's fighting. no hands."). A job that goes wrong
//   under way ends in words too: seen ("he's seen me!" — he stands with his gun on the man and the fight is the fight layer's), no way round unseen for
//   `stalkWait` s ('no way to his back unseen.'), not done in `takeSecs` ("can't get to his back."). Any other order cancels the job where he stands.
//
//   NOT HERE (other parts of the goal): how the partner moves and follows while nobody is alerted — his pace, his crouch, how well he sneaks — and what he does
//   once a guard is suspicious. The stalk's own pace and crouch are knobs here until that part gives him a sneaking grade to read.
//
// No module-level table is built from another game module's exports (src/game is one import cycle): PAIR is a plain literal, the rows are made in seat().
import { SideMap, rollbackRoot } from '../math/side';
import type { Game, Guard } from './game';
import { v3, type Vec3 } from '../math/vec';
import './combat';
import { lightEstimateAt, canSeeChar, sightGainOn, pointInView, BODY_LIGHT, GUARD_EYE } from './guards';
import { SILENT, squadSilent } from './silent';
import { pairUp } from './pairup';
import './sneak';
import { spottedGripped } from './spotted';
import { HOSTILE } from './teams';
import './takedown';   // the partner's takedown as a move, and the man it lands on (takedown.ts): poses only
import { type SquadOrders } from './orders';
import { RUST_PASS, RUST_SURFACE, crossToRust } from './crossing';

export const PAIR = {
  /** THE SWITCH: the quiet orders for one partner (off: the game as it was) */
  on: false,
  /** the biggest squad (men besides Sam) the rows are offered to: these are a PAIR's orders */
  maxSquad: 1,
  /** a defender this near (m) the point under the cursor is the target (his body under the cursor always is) */
  pickM: 0.9,
  /** on your mark: how far behind the man (m) the partner waits · his hands reach this far (m) and take this long (s) · the noise they make (a 'prop' event; 0: none) */
  stageM: 1.2, reach: 1.05, secs: 0.6, scuffle: 0,
  /** a man with the partner's hands on him sees nobody for as long as they are (as one in Sam's arm does not): false = he could turn his head inside those
   *  `secs`, see the man at his back at arm's length and go ALERT before he went down — one takedown in a handful, played */
  grip: true,
  /** … nor does he find a BODY while they are on him: two men who face each other across a lit road, taken on one beat, otherwise fail by a tenth of a second —
   *  the first to drop is found, and radioed, by the second with the partner's hands already on him (the border crossing's lane, traced). The hold is `gripped()`'s:
   *  while the partner's job on him stands, at most `secs` + 0.5 s from the hands going on (0.6 s when the takedown lands; up to 1.1 s when it does not), and it
   *  ends with the job. It is asked inside guards.ts bodySeen, so every caller takes it — enemysquads.ts's two as well (a squad man in those hands). false = as it was */
  gripBody: true,
  /** his pace working round to the man's back, and over the last paces once sent (m/s; the silent posture's own cap still applies) · on a knee on the way */
  pace: 1.1, goPace: 1.6, 
  /** the way to that back is read this often (s); with every way across somebody's eyes he waits this long (s) before he says there is none */
  stalkEvery: 0.3, stalkWait: 20,
  /** A SENTRY WHO SWEEPS: a standing man's head turning to and fro takes his near field (±100° of where he looks) round with it, so the floor at his elbow
   *  that is out of his eye this second is in it the next. The way to a back treats floor inside `ringM` of such a man as watched wherever his sweep has
   *  reached lately — all but a wedge astern of the way he mostly faces — so the search goes round it, the same way round every time it is asked (off: priced only
   *  where he looks now and mostly looks, and two near-equal ways round him trade places as his head turns) */
  sweep: true, ringM: 2.4,
  /** HE DOES NOT TURN ABOUT ON HIS WAY (the maintainer, 10-22, the pair lesson: "partner seems to be very paranoid about always turning around to check the
   *  other guy"). Two things turned him (the lesson's scripted run, sampled): the way to a man's back handed out with a corner he had already passed first —
   *  the way he is kept on picks its nearest CORNER, and with that a pace behind him he turned right round to walk back to it and round again the next time
   *  the way was read; and each time the way read watched he parked and at once squared to the man, sixty to a hundred degrees, and squared back to walk a
   *  third of a second later. So: on the kept way a corner he is already past along its own leg is dropped, where the way on from the next is as plain and
   *  as cheap a walk from where he stands (priced as any kept way is); and parked on a watched way he keeps the heading he has for `steadyFaceSecs` — only a wait that lasts turns him to watch
   *  his man. The page: on (`?pairsteady=0`); false here and on the frozen harness stand (`TPS_PAIRSTEADY=1`; a FROZEN_HOLDS row). false = as it was */
  steadyFaceSecs: 1.2,
  /** floor inside this (m) of a defender, in front of him and in his sight, is not walked (just past GUARD_EYE.nearM: inside that an eye has ±100°, and at
   *  2.6 two of 37 un-staged takes on The block and the depot walked the partner into it and were seen) */
  frontM: 3.6,
  /** … nor is LIT floor (brighter than `coneLit` on the meter's scale, the CPU's steady direct estimate) inside `coneM` of a defender, within 65° of where he
   *  looks and in his line: a man taken there is taken in plain view of his mate. 0 m = only the near field counts (un-staged on The block a man was then taken 5 m in front of his mate, on lit floor, and the floor woke) */
  coneM: 9, coneLit: 0.3,
  /** HE'S WATCHED: at the mark (and at his own moment, 'take him now') the partner asks whether another defender would SEE it — by the defenders' own tests:
   *  eyes on the partner at the man's back (canSeeChar) with light enough on him to start that man inside `watchSecs` (sightGainOn × secs ≥ `watchAw`), or the
   *  floor the man will lie on in his open view with the bodies-by-light bar met by the light now on him. Then he does not go: he says who, and waits for
   *  that man to look away — or for a second mark ('going anyway.'). Sam's own hands as the mark are never held up by it. Off: the mark is the mark. */
  watched: true, watchSecs: 1.2, watchAw: 0.25,
  /** … and a watcher who stands where he stood and whose sweep will bring his eyes back is not 'looking away': the partner stays held ('he'll look back.')
   *  until that man walks off, turns for good, or a second mark sends him (off: he goes the moment the watcher's head is turned) */
  lookBack: true,
  /** 'take him now': he holds at this ring (m) for the man to glance away and back, and goes within `heelFresh` s of the glance ending — or after `heelWait` s of none */
  heelRing: 2.9, heelFresh: 1.2, heelWait: 22,
  /** a job not done this long (s) after it was given (after the mark, for one that waited) is called off by the man himself */
  takeSecs: 75,
  /** Sam's own hands going onto a man (his takedown, his grab) are the mark too */
  markOnHands: true,
  /** hold in the dark: floor is DARK at or under this on the light meter's 0‥1 scale (the place's steady light, direct only) · how far (m) from the point he
   *  may look for it · none: refuse (off: go anyway, and say so) · on a knee there · the height (m) the light is read at */
  darkVis: 0.3, darkM: 1.5, darkH: 0.9,
  /** … and once there he says so when the place has been lit this long (s), not more often than every `litEvery` s */
  litSecs: 2, litEvery: 20,
};
export const PAIR_STATS = { given: 0, marks: 0, taken: 0, blown: 0, called: 0, cancelled: 0, held: 0, dark: 0, lit: 0, watched: 0, anyway: 0 };
/** what happened when (the probes read it; capped): 'mark:<man> on <foe>', 'now:…', 'set:…', 'go:<by>', 'watched:<foe> by <who>', 'clear:…', 'anyway:…', 'hands:…', 'down:…', 'blown:…', 'off:<why>', 'cancel:…', 'dark:…', 'lit:…' */
export const PAIR_LOG: { t: number; what: string }[] = [];
rollbackRoot('pairorders.PAIR_LOG', PAIR_LOG);   // (not a log alone: a lesson's `pair` step counts the rows filed since it began — lesson.ts)
/** ?pairorders (=0: off) and the harness's PAIRORDERS — read at load, and again once the Game has read the stored team feel back (the flag is this visit's word) */
function flags() {
  try { const q = new URLSearchParams(location.search); if (q.has('pairorders')) PAIR.on = q.get('pairorders') !== '0'; } catch { /* no page (the harness) */ }
  try { const e = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.PAIRORDERS; if (e) PAIR.on = e !== '0' && e.toLowerCase() !== 'off'; } catch { /* no process (the page) */ }
}
flags();

const flat = v3.distXZ, yawTo = v3.yawTo;
const nameOf = (h: Guard) => h.callsign || 'that man';

interface Job {
  foe: Guard; mode: 'mark' | 'now';
  /** the mark has been given (mode 'now': from the start) · he is on his staging spot · 'set.' has been said */
  go: boolean; set: boolean; setSaid: boolean;
  /** when his hands went on (−1: not yet) · when the job (or its last leg) began · when the way was last read · since when there has been none · since when he has waited at the ring · he is past it */
  t0: number; born: number; checkT: number; waitT: number; heelT: number; going: boolean; scuffled: boolean;
  /** the mark is given but another defender would see it (who, since when) · a second mark, or Sam's own hands: he goes whoever is looking */
  /** he was at Sam's shoulder (on 'fall in') when the order came: with the man down he falls in again (pairup.ts 'rejoin') */
  wasFollowing: boolean;
  /** the man roused with this one at his back and spotted.ts's hands took it on (pairup.ts 'hands'): the job waits on them */
  handed?: boolean;
  watcher: Guard | null; watchT: number; seenT: number; force: boolean; cueT: number; cueBy: Guard | null; cued: boolean; backSaid?: boolean; backT?: number;   // (cue*: the 'hold … / clear.' pair is SAID at most once per 8 s per watcher — a watcher whose glance comes and goes is not narrated turn by turn; the status line always shows it)
}

export class PairOrders {
  private jobs = new Map<Guard, Job>();
  /** where each defender MOSTLY looks (his facing eased over ~6 s: the back a takedown goes for) and whether he is glancing off it right now */
  private habit = new SideMap<Guard, [number, number]>();
  private glance = new SideMap<Guard, { on: boolean; endT: number; h0?: [number, number] }>();
  /** seconds a defender has stood where he is (0 while he walks) · how far (rad) his look has lately swung off the way he mostly faces, standing */
  private stood = new SideMap<Guard, number>();
  private swept = new SideMap<Guard, number>();
  private dark = new SideMap<Guard, { checkT: number; litT: number; saidT: number }>();
  private darkMemo = new Map<string, { t: number; spot: Vec3 | null }>();
  /** the way each man is on to a back, under pairup.ts oneEye (stalk keeps it while it reads unseen) */
  private ways = new SideMap<Guard, { to: Vec3; path: Vec3[]; t: number }>();
  private goSeated = false; private handsWas = false;
  constructor(private g: Game) {
    const S = g.orders; flags();
    S.people = {};   // (a man of the other side is offered: the Rust game's question of a point and of the cursor asks)
  }

  // ------------------------------------------------------------ who, what
  /** the rows are on the menu: the switch, a squad on the floor, and not more of it than a pair's orders are for */
  get up(): boolean { if (!PAIR.on) return false; const S = this.g.orders; if (!S.active) return false; const n = S.squad().length; return n >= 1 && n <= PAIR.maxSquad; }
  /** the squad is quiet enough for hands (silent — or the silent order not in the build at all) */
  get quiet(): boolean { return !SILENT.on || squadSilent(this.g); }
  /** a man an order can be about: standing, on the other side from Sam, not in Sam's arm, and seen (the fog of war has not got him) */
  isFoe(h: Guard): boolean { return h.state !== 'dead' && h.team !== 'operative' && HOSTILE.player[h.team] && !h.held && !this.g.fogOfWar.hidden(h); }
  /** the men waiting on the mark */
  waiting(): [Guard, Job][] { return [...this.jobs].filter(([gd, J]) => !J.handed && ((J.mode === 'mark' && !J.go) || (!!J.watcher && !J.force && this.g.time - Math.max(J.seenT, J.backT ?? -1e9) < 1)) && gd.state !== 'dead'); }
  jobOf(gd: Guard): Readonly<Job> | null { return this.jobs.get(gd) ?? null; }
  /** (bounded: hands that stopped being tended — their man handed to the fight AI mid-struggle — do not blind the man for ever) */
  gripped(h: Guard): boolean { for (const J of this.jobs.values()) if (J.foe === h && J.t0 >= 0 && this.g.time - J.t0 < PAIR.secs + 0.5 && this.g.time >= J.t0) return true; return false; }
  private note(what: string) { PAIR_LOG.push({ t: this.g.time, what }); if (PAIR_LOG.length > 200) PAIR_LOG.splice(0, 50); }


  /** THE MARK: every man waiting on it goes for his man (false: nobody was waiting) */
  mark(by: 'F' | 'hands' | 'row' | 'api' = 'api'): boolean { return crossToRust('PairOrders.mark', this, [by]) as any; }
  /** WHO WOULD SEE IT (PAIR.watched): the first other defender who, by the defenders' own tests, has eyes on `gd` where he stands at the man's back with light
   *  enough on him to start that man inside `watchSecs`, or whose open view holds the floor the man will lie on with the bodies-by-light bar met by the light
   *  now on him (guards.ts canSeeChar · sightGainOn · pointInView · BODY_LIGHT — nothing of this module's own). A quiet pair of hands makes no sound: there is
   *  nothing to hear (PAIR.scuffle 0). */
  watcher(foe: Guard, gd: Guard): Guard | null { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('PairOrders.watcher', this, [foe, gd]); if (__r !== RUST_PASS) return __r as any; } }
    const g = this.g, hp = foe.char.bones.hips ?? foe.char.pos, at: Vec3 = [hp[0], 0.35, hp[2]];
    for (const o of g.guards) {
      if (o === foe || o.state === 'dead' || o.held || o.team === 'operative' || !HOSTILE.player[o.team]) continue;
      const s = canSeeChar(g, o, gd.char, false);
      if (s.visible && sightGainOn(g, o, gd, s.dist) * PAIR.watchSecs >= PAIR.watchAw) return o;
      if (o.team !== foe.team || !pointInView(g, o, at)) continue;   // (bodyInView: a man finds a body of his own side)
      if (this.bodyLit(o, foe, at)) return o;
    }
    return null;
  }
  /** the bodies-by-light bar from `o`'s eye, met by the light now on the man (his meter where he has one, the floor's own estimate where he does not: staff
   *  carry no meter. Smoke between them is not asked: he errs to holding) */
  private bodyLit(o: Guard, foe: Guard, at: Vec3): boolean { const B = BODY_LIGHT;
    const eye = o.char.bones.head ?? [o.char.pos[0], o.char.pos[1] + 1.65, o.char.pos[2]], d = Math.hypot(eye[0] - at[0], eye[1] - at[1], eye[2] - at[2]);   // (eye to the body, as bodyLit measures it)
    const bar = !this.g.tune.bodyLight || d <= B.nearM ? -1 : B.litNear + (B.lit - B.litNear) * Math.min(1, (d - B.nearM) / Math.max(0.1, B.farM - B.nearM));
    return Math.max(foe.visibility, this.litAt(at)) >= bar; }
  /** HE'LL LOOK BACK (PAIR.lookBack): the man who was watching has only turned his head — he is up, calm, standing where he stood, and what he would see
   *  when it comes back is still there to see: inside what his eyes sweep (the way he mostly faces, widened by how far his look has lately swung, his own
   *  reach, a line to it) is the floor the body would lie on with the bodies-by-light bar met, or the partner with light enough on him — the watcher's own
   *  two tests, with the sweep where the open view was. A lamp put out, or the partner gone from his line, ends the hold */
  private looksBack(w: Guard, foe: Guard, gd: Guard): boolean {
    if (!PAIR.lookBack || w.state === 'dead' || w.held || (this.stood.get(w) ?? 0) < 0.5) return false;
    const H = this.habit.get(w), L = H ? Math.hypot(H[0], H[1]) : 0; if (!H || L < 0.3) return false;
    const eye = w.char.bones.head ?? [w.char.pos[0], w.char.pos[1] + 1.65, w.char.pos[2]], half = GUARD_EYE.halfDeg * Math.PI / 180 + (this.swept.get(w) ?? 0);
    const swept = (x: number, y: number, z: number): number => { const dx = x - eye[0], dz = z - eye[2], d = Math.hypot(dx, dz); if (d > GUARD_EYE.range || d < 0.05) return -1;
      return Math.acos(Math.max(-1, Math.min(1, (dx * H[0] + dz * H[1]) / (d * L)))) < half && !this.g.col.segmentBlocked([eye[0], eye[1], eye[2]], [x, y, z]) ? d : -1; };
    const hp = foe.char.bones.hips ?? foe.char.pos, at: Vec3 = [hp[0], 0.35, hp[2]];
    if (w.team === foe.team && swept(at[0], at[1], at[2]) >= 0 && this.bodyLit(w, foe, at)) return true;
    const c = gd.char.pos, d = swept(c[0], c[1] + 1.0, c[2]);
    return d >= 0 && sightGainOn(this.g, w, gd, d) * PAIR.watchSecs >= PAIR.watchAw;
  }
  /** would the mark, given now, find this man held up by a watcher (he is near enough for the question, and nothing has forced him) */
  private held(J: Job, gd: Guard): boolean { return PAIR.watched && !J.force && J.t0 < 0 && (!!J.watcher || flat(gd.char.pos, J.foe.char.pos) < PAIR.heelRing) && (!!this.watcher(J.foe, gd) || (!!J.watcher && this.looksBack(J.watcher, J.foe, gd))); }   // (once held he stays held while the watcher looks, wherever the man walks)
  /** off the job, onto a plain hold where he stands — with his gun on `aim` when the job ended in the man being roused */
  private stand(S: SquadOrders, gd: Guard, words: string, aim: Guard | null) {
    const p = gd.char.pos, here: Vec3 = [p[0], 0, p[2]]; this.jobs.delete(gd);
    S.assign(gd, aim ? { order: 'hold', post: here, face: yawTo(p, aim.char.pos), watch: [aim.char.pos[0], 1.1, aim.char.pos[2]], stance: 'none', upper: 'aim', going: words, there: words }
      : { order: 'hold', post: here, face: gd.char.bodyYaw, stance: 'lowReady', upper: 'none', going: words, there: words });
  }

  // ------------------------------------------------------------ taking a man
  private yawOf(foe: Guard): number { const H = this.habit.get(foe); return H && Math.hypot(H[0], H[1]) > 0.3 ? Math.atan2(H[0], H[1]) : foe.char.bodyYaw; }
  private back(foe: Guard, m: number): Vec3 { const c = foe.char.pos, y = this.yawOf(foe); return [c[0] - Math.sin(y) * m, 0, c[2] - Math.cos(y) * m]; }
  /** is `p` floor at `foe`'s back with nothing solid between it and him */
  private backOpen(foe: Guard, p: Vec3): boolean { const c = foe.char.pos; return !this.g.col.solidBlocked([p[0], 1.0, p[2]], [c[0], 1.0, c[2]]); }
  /** where the hands go on from: a short pace behind the way he MOSTLY faces (a glance over his shoulder does not send the partner round him) — as far behind
   *  him as there is floor */
  behind(foe: Guard): Vec3 { for (const m of [0.8, 0.6, 0.45]) { const p = this.back(foe, m); if (this.backOpen(foe, p)) return p; } return this.back(foe, 0.45); }
  /** where a man waits on the mark: `stageM` behind him, nearer where the floor is short — a spot he can stand on with nothing between it and the man's back */
  stage(foe: Guard): Vec3 { const nav = this.g.col.nav;
    for (const k of [1, 0.8, 0.6]) { const p = this.back(foe, PAIR.stageM * k); if (!nav.isBlocked(p[0], p[2]) && this.backOpen(foe, p)) return p; }
    return this.behind(foe); }

  // ------------------------------------------------------------ holding in the dark
  /** how lit a spot of floor is, 0‥1 on the light meter's own scale: the place's steady lamps only (nobody's torch, no flash), direct light only — the CPU's
   *  estimate (guards.ts lightEstimateAt: no bounce) */
  /** litAt per nav cell, kept a few seconds (the stalk asks it of every cell it prices; the steady lamps change only when one is shot or switched) */
  private litCells = new Map<number, [number, number]>();
  litAt(p: Vec3): number { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('PairOrders.litAt', this, [p]); if (__r !== RUST_PASS) return __r as any; } } const g = this.g; return g.visibilityOf(lightEstimateAt(g, [p[0], PAIR.darkH, p[2]], () => true, true)); }

  summary(): string {
    if (!PAIR.on) return 'off';
    if (!this.up) return `on · not offered (a squad of 1 to ${PAIR.maxSquad} on the floor has them)`;
    const J = [...this.jobs].map(([gd, j]) => `${gd.callsign} ${j.mode === 'mark' ? (j.go ? 'going for' : j.set ? 'SET on' : 'moving up on') : 'taking'} ${nameOf(j.foe)}`);
    return `on · ${this.quiet ? 'quiet' : 'loud'} · ${J.join(' · ') || 'no job'} · given ${PAIR_STATS.given} · marks ${PAIR_STATS.marks} · taken ${PAIR_STATS.taken} · blown ${PAIR_STATS.blown}`;
  }
}

/** guards.ts's eyes: the partner's hands are on this man — he sees nobody until they are off (PAIR.grip) */
export function pairGripped(g: Game, h: Guard): boolean { return PAIR.grip && ((PAIR.on && g.pair.gripped(h)) || (pairUp('hands') && spottedGripped(g, h))); }   // (… or spotted.ts's hands, under pairup.ts's 'hands': one takedown, one rule for what the man in it sees)
