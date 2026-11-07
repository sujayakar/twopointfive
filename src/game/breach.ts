// The breach DRILL: two operatives with FN2000s run up at the low ready, stack on a room's door, boot it, put a stun charge in the instant the leaf clears, go in
// on the bang — #1 through the fatal funnel and a stride and a half straight in, a check across the middle and the man standing in it dropped, then his corner; #2
// on his heels the other way — end in an L owning the room and its next door, and stack on that door. Both operatives are scripted men (squad.ts's runner behind
// Guard.script, the drill below sequencing them the way RoomClear sequences the lockdown pair — battle drill six, the same doctrine RoomClear's live men keep).
//
// It began as the choreography of a daytime showcase (the office's level script, program 'breach', with a `Breach` facade here that main.ts and the probes drove).
// The two showcases were removed; what stays is BreachDrill — the one piece of choreography with fourteen clocks, kept statement for statement — which the night
// breach scenario runs LIVE on Sam's orders (game/nightbreach.ts, BreachLive) and a level script may still drive as a Director drill (game/director.ts D.drill).
// Its men are spawned UNITS (guards.ts spawnUnit: an operative's round finds a hostile through the ordinary fireWeapon and he goes down to the hit; the drill only
// aims and pulls the trigger on cue, and drops a man itself only if both rounds of his pair found furniture — `guaranteeDowns`, logged when it fires).
import './teampace';   // (read inside tick only)
import { Vec3, v3 } from '../math/vec';
import type { Game, Guard } from './game';
import type { BreachPlan } from '../scene/level';
import type { Item } from './throwables';
import { order, endGuardScript, Sweep } from './squad';
import './character';
import { killGuard } from './combat';
import './particleFx';
import './glasspen';

/** the staged exchange: seconds a man's muzzle rests on his target before the first round, between the two rounds of the controlled pair, and before he takes the next man */
const PAIR = 0.16;   // (SETTLE 0.34 → 0.14, SHIFT 0.4 → 0.3, 08-13: 'take the shot immediately when passing through the door … no lingering in the threshold with the laser on the enemy')

// ================================================================ the drill
type Phase = 'wait' | 'approach' | 'stack' | 'breach' | 'kick' | 'bang' | 'enter' | 'clear' | 'restack' | 'done';
/** one operative's way through the room (BreachFlow runs on this): waiting for his go, through the funnel, the centre check, on to his corner, holding his sector */
type FlowSt = 'wait' | 'into' | 'centre' | 'corner' | 'hold';
/** What a LIVE drill reads every frame (BreachDrill.live; game/nightbreach.ts owns the object and writes it): how many orders Sam has given (1 = stack up, 2 = go),
 *  whether the pair's weapon lights come on as they enter, whether the boot is heard by the men inside (a 'kick' hearing event at the leaf), and whether #2 has a
 *  stun charge to put in (false = boot and go: no can, no bang, #1 starts through the door on the beat the charge would have left the hand). */
export interface BreachLive { orders: number; lights: boolean; loudBoot: boolean; stun: boolean;
  /** seconds the stack stands set before the point man may step out on 'go' (absent = SET_HOLD, the show's beat; game/rescue.ts shortens it once the clock is running) */
  setHold?: number; }
interface OpFlow {
  st: FlowSt; t0: number; wps: Vec3[]; sweep: Sweep; aim: Vec3;
  /** the staged exchange: hostile indices still to engage in this leg, the one his muzzle is on (−1 none) and since when, rounds of the pair fired, earliest next trigger / pick-up */
  q: number[]; on: number; since: number; n: number; next: number;
  said: boolean; inAt: number; cornerAt: number; holdAt: number;
}
/** Two operatives taking a held room through one door and flowing through it: run in → stack → #1 steps out and boots the leaf off his left leg → steps back to
 *  his own side while #2, tight on the jamb, chops the charge in the moment the latch edge clears → the bang → #1 through the funnel, centre check, the man in it,
 *  his corner; #2 on his heels, the same the other way, ending squared to the next door → the calls → both stack on that door → cut. Sequenced on its own clock
 *  like squad.ts RoomClear, every wait capped so a man who cannot make a mark costs a line, never the show. `beats` logs what happened when, in order, for the probes. */
export class BreachDrill {
  phase: Phase = 'wait';
  tStack = NaN; tSet = NaN; tGo = NaN; tKick = NaN; tFlung = NaN; tToss = NaN; tLand = NaN; tBang = NaN; tEnter = NaN; tCorner = NaN; tClear = NaN; tNext = NaN; tStacked2 = NaN; tDone = NaN;
  beats: { t: number; what: string }[] = [];
  rounds = [0, 0];
  /** the drill's own clock (seconds since start): the camera programme reads 'how long since tEnter / tClear' off it */
  t = 0;
  private done = new Set<string>();
  private aimA: Vec3; private aimB: Vec3;
  private wpA: Vec3[] = []; private wpB: Vec3[] = [];
  private can: Item | null = null; private chopped = false; private downLogged: boolean[];
  readonly flows: [OpFlow, OpFlow];
  /** guaranteeDowns (default true = the show as it always ran): a hostile still standing after both rounds of his pair, or after 14 s of entry, is put down by hand so the
   *  calls are true — logged as 'guaranteed:k' when it fires (it never has: the rounds find their men); false = only real rounds drop anybody */
  readonly guaranteeDowns: boolean;
  /** LIVE PLAY (game/nightbreach.ts — null = the show's drill, statement for statement as it was): the pair wait on the approach marks for `orders` ≥ 1 ('stack up')
   *  and on the stack for `orders` ≥ 2 ('go') instead of the show's clocks; the men inside are NOT the drill's to script (start and bang leave them alone: they are
   *  pinned AI men who shoot back); a man down ends nothing while the other can go on (he finishes his flow with every living man inside on his list); the staged
   *  exchange asks for a line before every pick AND every round; the boot is heard (a 'kick' event); the room is called only when everybody inside is down — no 14 s
   *  fallback — and the drill ends there, on the hold, with no restack. */
  readonly live: BreachLive | null;
  /** live: the drill clock when order one came (the approach's cap counts from it, as the show's does from HOLD_OPEN) */
  tRun = NaN;
  constructor(private g: Game, readonly plan: BreachPlan, readonly a: Guard, readonly b: Guard, readonly hostiles: Guard[], opts: { guaranteeDowns?: boolean; live?: BreachLive } = {}) {
    this.aimA = v3.copy(plan.stackAim[0]); this.aimB = v3.copy(plan.stackAim[1]);
    const mk = (aim: Vec3): OpFlow => ({ st: 'wait', t0: 0, wps: [], sweep: new Sweep(), aim, q: [], on: -1, since: 0, n: 0, next: 0, said: false, inAt: NaN, cornerAt: NaN, holdAt: NaN });
    this.flows = [mk(this.aimA), mk(this.aimB)];
    this.downLogged = hostiles.map(() => false);
    this.guaranteeDowns = opts.guaranteeDowns ?? true;
    this.live = opts.live ?? null;
  }
  get door() { return this.g.doors.byName(this.plan.door); }
  get nextDoor() { return this.g.doors.byName(this.plan.next.door); }
  private beat(what: string) { this.beats.push({ t: this.t, what }); }

  /** Stand everybody on his mark — the pair on the approach marks at the low ready facing down the aisle, the opposition inside on theirs, muzzles on the door —
   *  the leaf pulled to and latched (it is authored standing open): something to breach. */
  start() {
    const P = this.plan;
    [this.a, this.b].forEach((gd, i) => {
      const c = gd.char; const from = P.from?.[i] ?? P.stack[i];
      c.pos = v3.copy(from); c.vel = [0, 0, 0]; c.bodyYaw = c.aimYaw = v3.yawTo(from, P.stack[i]); c.aimPitch = 0; gd.speed = 0;
      gd.state = 'patrol'; gd.awareness = 0; gd.lastKnown = null; gd.hold = false; gd.pinned = false; gd.reactT = 0; gd.script = null;   // ('patrol', not the drill's 'search': a hunting man's laser is on — combat.ts handProps — and two lasers waving down a sunlit office is not the approach; they go to 'search' on the bang, and the lasers cut the room's haze where they belong)
      gd.weapon.rearm(); c.update(0);
      order(gd, { goal: null, face: c.bodyYaw, stance: 'lowReady', upper: 'none', aimAt: null, light: false, crouch: false });   // stood at the low ready facing the way they will go, until the word (tick: 'wait')
    });
    if (!this.live) P.hostiles.forEach((h, i) => { const gd = this.hostiles[i]; if (!gd) return; const c = gd.char; c.pos = v3.copy(h.pos); c.vel = [0, 0, 0]; c.bodyYaw = c.aimYaw = h.yaw; gd.state = 'patrol'; gd.awareness = 0; c.update(0);
      order(gd, { goal: null, face: h.yaw, aimAt: v3.copy(h.aimAt), stance: 'none', upper: 'aim', crouch: !!h.crouch, light: false }); });
    const d = this.door; if (d) { d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; }
    const dn = this.nextDoor; if (dn) { dn.reset(); dn.angle = 0; dn.vel = 0; dn.latched = true; dn.closing = false; }   // the next threshold shut too: a door to stack on, not a hole in the wall
    this.beat('start');
  }
  /** everybody off his script (the cast is about to be dismissed anyway; a stop mid-show must not leave the runner holding men that no longer exist) */
  end() { for (const gd of [this.a, this.b, ...this.hostiles]) if (gd.script) endGuardScript(gd, 'patrol'); }



  /** p inside the room's authored bounds by at least `m` (plan.bounds: the walls' inner faces) */
  private insideRoom(p: Vec3, m: number): boolean { const B = this.plan.bounds; if (!B) return true; return p[0] > B.x0 + m && p[0] < B.x1 - m && p[2] > B.z0 + m && p[2] < B.z1 - m; }   // (no bounds authored: the frame test is the funnel's own arrival)
  /** both men holding their sectors and every hostile down (a beat after the last of those) — or 14 s of entry, when whoever is somehow still standing is put down so the calls are true */
  private roomClear(te: number): boolean {
    const own = !!this.live && this.g.fight.on;   // (the room fight: a defender who has got OUT of the room is not in it — the room is called on the men left inside)
    const H = this.hostiles, allDown = H.every(h => h.state === 'dead' || h.fled || (own && !this.insideRoom(h.char.pos, -0.3)));
    const up = (i: 0 | 1) => (i === 0 ? this.a : this.b).state !== 'dead';
    const holding = this.live ? (!up(0) || this.flows[0].st === 'hold') && (!up(1) || this.flows[1].st === 'hold') : this.flows[0].st === 'hold' && this.flows[1].st === 'hold';
    if (holding && allDown) { if (isNaN(this.tAllDone)) this.tAllDone = this.t; return this.t > this.tAllDone + 0.5; }
    if (this.live) return false;   // live: nobody calls a room with a man still up in it, whatever the clock says
    if (te > 14) { H.forEach((h, k) => { if (h.state !== 'dead' && this.guaranteeDowns && !h.fled) { this.beat(`guaranteed:${k}`); killGuard(this.g, h, [1, 0, 0], false, 'shot', 'guard', 'hostile down'); } }); return true; }
    return false;
  }
  private tAllDone = NaN;

}
