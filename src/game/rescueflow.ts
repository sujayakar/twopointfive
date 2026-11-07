// The hostage rescue's DYNAMIC TEMPO (rescue.ts owns one of these; ?rescue): what the pair do about the openings Sam runs past, without being told, and what the
// player is shown of each. The 'fast rescue' answer page's 3.6–3.8 and 3.11: you pass, they pass, and the last man picks up the opening.
//
//   THE ROUTE      the corridor to the hostage room and the openings onto it, west to east — LEVEL DATA (scene/scenarios.ts RescueFlowData: Level.scenarios.rescue.flow;
//                  the office's, levels/office.scenarios.ts: the conference room's glass and door, the cubicles' west doorway, the server room's door, the cubicles'
//                  east doorway), checked headless by check(): where a man stands to cover each, and the point his gun is on. A level with no `flow` = none of this runs.
//   WHAT IS KNOWN  per opening, sensed a few times a second from the TEAM's own eyes (Sam's and every living, undazzled operative's — sight lines through the
//                  collision's own test: glass passes, a shut leaf does not): `looked` = most of what can be seen of the room from the corridor at that opening
//                  has been seen inside the last lookSecs; `hot` = one of theirs has been seen in there (or has fired from in there) and not since seen
//                  elsewhere or dead — let go when the room has since been looked through without him, or after hotSecs of nothing; `cleared` = one of the
//                  team has been in it, most of its floor has been seen, nobody of theirs known in it; `held` = a man is posted on it (the flow's man on HIS
//                  opening, or one of Sam's menu orders); else `unknown` (a shut door reads 'shut'). The floor is sampled at a crouched man's chest (0.9 m: a
//                  cubicle partition hides it), and who of theirs is SEEN is the fog of war's answer while the fog is up — what the player is shown is what is known.
//   THE FLOW       only once the captor's clock runs, only men who are FALLING IN (a man on any other menu order is Sam's: the menu overrides, and 'fall in'
//                  hands him back). Sam WALKS the corridor past an opening, west to east → the LAST man in the file takes it: runs to its mark, gun on it, says so ('glass left.',
//                  'opening right.', 'door left.'), gives it its look (longer while one of theirs is known in it: 'contact left — holding.'), and catches up — the
//                  other man stays on Sam. Openings Sam passes while that man is busy queue behind him: he leapfrogs them in order ('wait one.' when Sam has
//                  got far ahead), so a sprint arrives a man short. Sam turns off the corridor INTO a side room with both men free → the last man holds the
//                  corridor at that opening ('corridor.') until Sam is back out. Coming onto the corridor from a side room, or into the hostage room by its
//                  side door, passes nothing: only what he has walked past is behind him.
//   THE PICTURE    a strip on the floor across each opening, in the corridor, coloured by what is known (amber unknown · pale looked · red hot · blue held ·
//                  green cleared: one-frame NoShadow boxes — no ray, no light meter, no guard ever sees them), the same words on the objective line for the
//                  openings behind Sam, and each man's job in the squad block ('covering cubicles west').
//
// Not built (the answer page has them): the wedge, a 'looked' that decays into a timer on the strip, the guns of the men WITH Sam turning to the nearest unknown.
// Nothing here runs unless the rescue is up; with RESCUE.flow and RESCUE.marks off it does nothing at all.
import { SideSet } from '../math/side';
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { Rescue } from './rescue';
import type { Assignment } from './orders';
import './squad';
import './pairup';
import './silent';
import { makeBox, BoxFlag } from '../scene/boxes';
import './dialogue';   // (called inside the hold's line only: nothing of dialogue.ts is touched at load)
import type { RouteOpening, RescueFlowData, Rect } from '../scene/scenarios';
import { RUST_PASS, RUST_SURFACE } from './crossing';

export type OpenState = 'unknown' | 'looked' | 'hot' | 'held' | 'cleared';
/** THE ROUTE is level data (scene/scenarios.ts RescueFlowData — Level.scenarios.rescue.flow; the office's: levels/office.scenarios.ts): the corridor, the rooms
 *  off it and the openings onto it with their cover marks. A level whose rescue data has no `flow` runs without any of this (tick() returns on its first line). */
export type { RouteOpening } from '../scene/scenarios';
type RegionId = string;

/** the flow's numbers (plain literals: Tab ▸ Sandbox ▸ Hostage rescue carries the two switches, RESCUE.flow / RESCUE.marks; these are __rescue.flow.knobs) */
export const FLOW = {
  /** Sam this far short of an opening's middle = he is passing it; back this far west of it = it is ahead of him again */
  abreast: 0.6, unpass: 2.0,
  /** the cover is let go once Sam is this far past it (and the look is done), or after maxCover on the mark whatever Sam does; a man who cannot get onto his
   *  mark gives it up after giveUp; an opening nobody was free to take is forgotten after wantSecs */
  past: 1.5, maxCover: 2.5, giveUp: 7, wantSecs: 6,
  /** a hot opening is held this long after the last sight or sound of the man in it; the room stops being hot hotSecs after it (nobody has seen or heard
   *  him: it is unknown again), or as soon as hotEmpty of its floor has been seen since without him */
  hotHold: 6, hotSecs: 20, hotEmpty: 0.6,
  /** 'looked' = this share of what can be seen through the opening, seen inside lookSecs; 'cleared' = this share of the room's whole floor, with a man in it */
  lookShare: 0.6, lookSecs: 10, clearShare: 0.7, clearSecs: 25,
  /** the team's eyes: reach, how often each room is looked over (one room a call, in turn), and the height of the floor samples (a crouched man's chest) */
  eyeM: 16, senseHz: 6, sampleY: 0.9,
  /** Sam turning into a side room: the corridor is taken by a man who comes free within this long of it */
  holdSecs: 3,
  /** 'wait one.' when the man working the openings is this far behind Sam with more to do */
  waitM: 9,
  /** the strips' glow (× the state's colour) */
  glow: 1,
};

const COLOUR: Record<OpenState, Vec3> = { unknown: [1.0, 0.55, 0.08], looked: [0.6, 0.62, 0.5], hot: [2.6, 0.12, 0.05], held: [0.12, 0.5, 1.6], cleared: [0.1, 1.0, 0.3] };
const NO_ROUTE: RouteOpening[] = [];

interface Region { id: RegionId; rect: Rect; pts: Vec3[]; seenT: Float64Array; cleared: boolean; hot: Map<Guard, { t: number; at: Vec3 }>; wasIn: number }
interface Known { o: RouteOpening; region: Region; lookable: number[]; builtOpen: boolean; share: number; state: OpenState; held: Guard | null; hot: boolean }
interface Job { kind: 'cover' | 'corridor'; o: RouteOpening; A: Assignment; t0: number; onT: number; chain: RouteOpening[]; saidHot: boolean; saidWait: boolean; leap: boolean }

export class RescueFlow {
  readonly knobs = FLOW;
  /** the level's route data (null = this level's rescue has no flow) and its openings */
  private get F(): RescueFlowData | null { return this.R.data?.flow ?? null; }
  get route(): RouteOpening[] { return this.F?.route ?? NO_ROUTE; }
  known: Known[] = [];
  /** who the flow has on a job right now */
  jobs = new Map<Guard, Job>();
  /** openings Sam has come past (ids), and the ones still waiting for a man (id → when) */
  passed = new Set<string>(); private want = new Map<string, number>();
  stats = { takes: 0, chained: 0, corridor: 0, overridden: 0, hotHolds: 0, unmanned: 0 };
  private regions = new Map<RegionId, Region>();
  /** the assignments the flow itself handed out (its covers, and the fall-in it sends a man back with): what tells the flow's man from one Sam posted */
  private mine = new SideSet<Assignment>();
  private senseAt = 0; private turn = 0; private heard = new SideSet<object>(); private built = false;
  private room: Rect = { x0: 0, x1: 0, z0: 0, z1: 0 };
  /** where Sam last stood ON the corridor (a pass is a walk along it, not a place), and the side room he has just turned into off it */
  private corr: { x: number; t: number } | null = null;
  private cameIn: { region: RegionId; o: RouteOpening; t: number } | null = null;
  constructor(private g: Game, private R: Rescue) { }

  /** a fresh staging: nothing known, nobody on a job */
  reset() {
    this.jobs.clear(); this.passed.clear(); this.want.clear(); this.senseAt = 0; this.turn = 0; this.built = false; this.known = []; this.regions.clear(); this.corr = null; this.cameIn = null;
    for (const k of Object.keys(this.stats) as (keyof RescueFlow['stats'])[]) this.stats[k] = 0;
  }
  /** is this assignment one the flow gave (rescue.ts: such a man is free for F; a cover Sam gave himself is not) */
  gave(A: Assignment | null): boolean { return !!A && this.mine.has(A); }
  /** F has taken these men: their jobs end where they stand */
  release(men: Guard[]) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RescueFlow.release', this, [men]); if (__r !== RUST_PASS) return __r as any; } } for (const gd of men) this.jobs.delete(gd); }
  jobOf(gd: Guard): { kind: 'cover' | 'corridor'; opening: string; queued: string[] } | null { const J = this.jobs.get(gd); return J ? { kind: J.kind, opening: J.o.id, queued: J.chain.map(o => o.id) } : null; }

  private open(o: RouteOpening): boolean { const d = o.door ? this.g.doors.byName(o.door) : null; return !d || Math.abs(d.angle) > 0.5; }



  /** nothing for a man to do at it: the room behind it is cleared, or a man Sam posted has it already */
  private settled(o: RouteOpening): boolean {
    const k = this.known.find(x => x.o === o); if (!k) return false;
    return k.region.cleared || (!!k.held && !this.jobs.has(k.held));
  }
  /** the men the flow may send: alive, falling in (unasked, or because Sam said so), not F's, not in a fight, not blind — the LAST in the file first */
  private free(): Guard[] {
    const g = this.g, out: [Guard, Assignment][] = [];
    for (const gd of this.R.ops) {
      if (gd.state === 'dead' || this.jobs.has(gd) || this.R.owns(gd) || g.time < gd.dazzledUntil) continue;
      const A = g.orders.of(gd); if (!A || A.order !== 'fallIn') continue;
      if ((!gd.script && gd.state === 'alert') || g.fight.engaged(gd)) continue;
      out.push([gd, A]);
    }
    return out.sort((a, b) => b[1].follow - a[1].follow).map(x => x[0]);
  }
  private take(gd: Guard, o: RouteOpening, chain: RouteOpening[], saidWait: boolean, leap: boolean) {
    const g = this.g, door = o.door ? g.doors.byName(o.door) ?? null : null;
    g.orders.newGroup();   // (his own: a 'set.' Sam's last order is waiting to call does not wait on this man's mark)
    g.orders.assign(gd, { order: 'cover', post: v3.copy(o.mark), face: v3.yawTo(o.mark, o.watch), watch: v3.copy(o.watch), door, stance: 'none', upper: 'aim', move: { stance: 'none', upper: 'aim' }, going: `moving · ${o.name}`, there: `covering ${o.name}` });
    const A = g.orders.of(gd)!; this.mine.add(A);
    this.jobs.set(gd, { kind: 'cover', o, A, t0: g.time, onT: NaN, chain, saidHot: false, saidWait, leap });
    this.stats.takes++; g.say(gd, o.call); this.R.note(`flow:take:${o.id}:${gd.callsign}`);
  }

  /** the strips: one flat glowing bar on the corridor floor across each opening, in its state's colour (a hot one beats) */
  private boxes() {
    const g = this.g;
    for (const k of this.known) {
      const o = k.o, c = COLOUR[k.state], beat = k.state === 'hot' ? 0.65 + 0.35 * Math.sin(g.time * 9) : 1, e = FLOW.glow * beat;
      g.fx.push({ box: makeBox({ c: [o.at[0], 0.012, o.at[2] + (o.north ? 0.36 : -0.3)], h: [o.half, 0.008, 0.05], albedo: [0.03, 0.03, 0.03], emissive: [c[0] * e, c[1] * e, c[2] * e], flags: BoxFlag.Dynamic | BoxFlag.NoShadow }), ttl: 1e-3 });
    }
  }
  private word(k: Known): string {
    const shut = k.o.kind === 'door' && !this.open(k.o);
    switch (k.state) {
      case 'hot': return k.held ? 'HOT, held' : 'HOT';
      case 'held': return 'held';
      case 'cleared': return 'clear';
      case 'looked': return 'looked';
      default: return shut ? 'shut' : 'unknown';
    }
  }
  /** the objective line's tail: the openings behind Sam and what is known of each, and any hot one still ahead of him ('' with neither) */
  line(): string {
    const row = (k: Known) => `${k.o.short} ${this.word(k)}`;
    const behind = this.known.filter(k => this.passed.has(k.o.id)).map(row), ahead = this.known.filter(k => !this.passed.has(k.o.id) && k.state === 'hot').map(row);
    return [behind.length ? `behind you ▸ ${behind.join(' · ')}` : '', ahead.length ? `ahead ▸ ${ahead.join(' · ')}` : ''].filter(x => x).join(' · ');
  }
  summary(): string {
    if (!this.known.length) return 'flow: not staged';
    const jobs = [...this.jobs].map(([gd, J]) => `${gd.callsign} ${J.kind === 'corridor' ? 'corridor' : J.o.short}${J.chain.length ? ` +${J.chain.length}` : ''}`).join(', ');
    return `openings ${this.known.map(k => `${k.o.short} ${this.word(k)}${this.passed.has(k.o.id) ? '' : ' (ahead)'}`).join(' · ')}${jobs ? ` · on it: ${jobs}` : ''} · ${this.stats.takes} covers, ${this.stats.corridor} corridor holds, ${this.stats.overridden} overridden`;
  }
}
