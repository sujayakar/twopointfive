// Scripted guards: the runner behind Guard.script (tour beats and, later, squad drills — stack on a door, pie the opening, enter to
// points of domination). While a guard has a script the AI state machine in guards.ts is bypassed and this moves him: nav-path toward
// `goal` at `speed`, hold `face` once planted (or all the way when strafing), point gun / torch / eyes at `aimAt`, feed the animator's
// tactical layers (stance, stack side, crouch, upper mode), keep the torch attached. It writes `arrived` back so the choreographer can
// sequence on it. Deliberately small: perception, barks and shooting stay with whoever holds the script (they call say()/fireWeapon).
// Below the runner: the first drill built on it — two men clearing a room (RoomClear), driven by a plan authored per room (level.ts, or the tour's own) —
// and below that the live use of it: the lockdown pair clearing the rooms nearest the last fix (Clearing), which also runs the scripted men's senses.
import { SideMap } from '../math/side';
import { Vec3, v3, wrapAngle, damp, clamp } from '../math/vec';
import type { Game, Guard, GuardScript } from './game';
import { isBreathing } from './game';
import type { Door } from './doors';
import type { RoomClearPlan } from '../scene/level';
import { KICK_IMPACT } from './character';
import './kitload';
import './teampace';
import './noshove';
import { kickDustFx } from './particleFx';
import { nearKnownBody } from './guards';
import './gunwall';
import { kickScuff } from './marks';
import './doorway';
import './silent';
import './handleside';
import { wg } from './words';   // (a leaf: the dialogue pass's one switch)
import { RUST_PASS, RUST_SURFACE } from './crossing';

/** Within this of the goal the nav path is dropped and he settles straight onto the exact spot (paths end on half-metre cell centres, and a
 *  choreographer's marks — a kicking distance off a door, a shoulder on a jamb — are not on cell centres). */
const SETTLE = 0.6;
const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** settle watchdog per guard: the goal being settled onto, the closest he has got to it, and how long since that last improved — a mark he cannot
 *  actually reach (authored inside furniture, or someone standing on it) makes him stop short and stand rather than tread on the spot forever */
const settleWatch = new SideMap<Guard, { goal: Vec3; best: number; t: number }>('squad.settleWatch');
/** a man the room fight walks on with his chest to a threat (GuardScript.over.walk) strafes his path, and a strafe has no stuck watchdog: where he was last
 *  seen to get somewhere, and when — wedged for over a second he walks his path the plain way (followPath's own side-step and re-path) for a while */
const walkWatch = new SideMap<Guard, { x: number; z: number; t: number }>('squad.walkWatch');



/** Release a scripted guard back to the AI in a sane state (stance dropped, on patrol unless told otherwise, nothing half-heard left to act on).
 *  A man who died under a script (killGuard leaves it on him) is only unhooked — the AI must never be handed a corpse as a patrol. */
export function endGuardScript(gd: Guard, state: 'patrol' | 'suspicious' | 'alert' | 'search' = 'patrol') { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('endGuardScript', undefined, [gd, state]); if (__r !== RUST_PASS) return __r as any; } }
  gd.script = null; if (gd.state === 'dead') return;
  gd.char.anim.stance = 'none'; gd.muzzleDown = false; gd.char.anim.crouchTarget = 0; gd.char.anim.lean = 0; gd.char.anim.reverse = false; gd.state = state; gd.path = []; gd.pathGoal = null;   // (the AI never plays the cycle backwards, so it never clears it either; the muzzle it works out afresh next frame)
  if (state === 'patrol') { gd.awareness = 0; gd.reactT = 0; gd.searchT = 0; gd.lastKnown = null; gd.bodyDuty = null; }   // (an awareness left over from the drill would tip a 'patrol' straight into 'huh?'; a calm man has no body on his mind)
}

/** Give a scripted guard new orders: patch his script and, when the goal moves, drop the old nav path (goTo keeps a path whose goal is within 0.75 m
 *  of the new one — fine for the AI's jittery fixes, wrong for marks), the stuck timer and the settle watch so the runner plans afresh. Creates the
 *  script if he has none — which takes him off the AI there and then: only order men you mean to choreograph, and endGuardScript them after. */
export function order(gd: Guard, patch: Partial<GuardScript>) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('order', undefined, [gd, patch]); if (__r !== RUST_PASS) return __r as any; } }
  const s = gd.script ?? (gd.script = { goal: null });
  if (patch.goal !== undefined && patch.goal !== s.goal) { gd.path = []; gd.pathGoal = null; gd.stuckT = 0; settleWatch.delete(gd); s.arrived = false; }   // (a new goal object = a new leg, even at the same spot; and he has not arrived at it until the runner says so — a stale true would fire whatever is sequenced on it a leg early)
  Object.assign(s, patch);
}

/** A muzzle sweep: eases an aim point through a polyline over `dur` seconds, starting from wherever the point was (no snap), easing in and
 *  out of every corner — which is how a sector is cleared: snap to the near corner, dwell a hair, run the wall to the next. Allocation-free
 *  per frame; `at` writes into the caller's Vec3 (normally the script's own aimAt). */
export class Sweep {
  private pts: Vec3[] = []; private t0 = 0; private dur = 1; active = false;
  start(from: Vec3, pts: readonly Vec3[], t0: number, dur: number) { this.pts = [v3.copy(from), ...pts.map(p => v3.copy(p))]; this.t0 = t0; this.dur = Math.max(0.01, dur); this.active = pts.length > 0; }
  /** the point for time t → out; returns false once the sweep has finished (out is left on the last corner) */
  at(t: number, out: Vec3): boolean {
    if (!this.active) return false;
    const n = this.pts.length - 1; const u = clamp((t - this.t0) / this.dur, 0, 1) * n;
    const i = Math.min(n - 1, Math.floor(u)); const f = u - i; const e = f * f * (3 - 2 * f);
    const a = this.pts[i], b = this.pts[i + 1];
    out[0] = a[0] + (b[0] - a[0]) * e; out[1] = a[1] + (b[1] - a[1]) * e; out[2] = a[2] + (b[2] - a[2]) * e;
    if (t - this.t0 >= this.dur) this.active = false;
    return this.active;
  }
}

// ================================================================ two-man room clear
/** The plan's shape (marks, facings, aim lists, lines) lives with the level data: scene/level.ts RoomClearPlan — re-exported here for the stagers. */
export type { RoomClearPlan };

/** Two scripted guards clearing a room like a trained pair, telegraphing every step out loud: stack (0–2.5 s), breach (the #2's squeeze, #1's
 *  kick — the real leaf, flung off the real hinge, banging off its stop — or, for staff at their own doors, the keyed shove that does the same
 *  without the boot), enter (split to their points of domination, muzzles sweeping their sectors corner to corner), search (one pushes deep, one
 *  checks low), resolve (call it clear, radio it in, weapons to low ready, rally on the door). tick(t) is driven by the caller's clock; every wait
 *  is capped so a blocked man delays a line, never the beat. Nobody in here perceives or shoots: the runner bypasses the AI — the tour keeps the
 *  player out of it, the live dispatcher below (Clearing) runs its own senses for the pair and breaks the drill off the moment they fire.
 *  `b` may be null: a man on his own runs the same drill as the point man with the cover's steps and lines left out.
 *  The doctrine is battle drill six's, as the breach showcase stages it (breach.ts): the man who boots the leaf steps back onto his own side while it
 *  swings rather than standing in the funnel (KICK_BACK); #2 never goes in on #1's heels (≥ 0.85 s); each man's muzzle checks the room's CENTRE first and
 *  is in his own corner inside a second of starting in (CENTRE_CHECK), then runs the rest of his sector; through the search they hold opposing corners
 *  (#1 deep along his wall, #2 on the door side); and where the room has another door, #2 comes up off his knee squared to it and holds it (nextDoorOf). */
export class RoomClear {
  phase: 'stack' | 'breach' | 'kick' | 'enter' | 'search' | 'resolve' = 'stack';
  /** shot-clock times the phases began (for the caller's camera) — NaN until reached; tEnter2 = when #2 started in */
  tKick = NaN; tEnter = NaN; tEnter2 = NaN; tSearch = NaN; tResolve = NaN;
  /** the plan's door (undefined if the level has no leaf of that name: the drill then stages nothing) */
  readonly door: Door | undefined;
  /** another door of this room for #2 to hold once he is back on his feet (the one nearest his point of domination, by the plan's bounds) — null: his wall, as authored (upAim) */
  readonly nextDoor: Door | null;
  /** live (the lockdown pair, squad.ts Clearing) rather than staged (the tour): start() takes the men where and as they stand, the squeeze waits for the stack to
   *  actually form (capped), the breach clock runs from the squeeze, #2 goes in on distance rather than the kick mark, and `complete` reports the rally */
  readonly live: boolean;
  private done = new Set<string>(); private flung = false; private bGo = false; private noMark = false;
  private tSet = NaN; private tGo = NaN; private tNow = 0; private aHome = false; private bHome = false;
  private aimA: Vec3 = [0, 0, 0]; private aimB: Vec3 = [0, 0, 0]; private swA = new Sweep(); private swB = new Sweep();
  /** the rest of each man's entry sweep, run once his centre check and corner are done (enterSweep) */
  private restA: Vec3[] | null = null; private restB: Vec3[] | null = null;
  private wpA: Vec3[] = []; private wpB: Vec3[] = [];   // waypoints still ahead of each man (script.goal is [0]; passed ones are shifted off)
  constructor(private g: Game, readonly plan: RoomClearPlan, readonly a: Guard, readonly b: Guard | null, opts: { live?: boolean } = {}) { this.door = g.doors.byName(plan.door); this.live = !!opts.live; this.nextDoor = nextDoorOf(g, plan); }

  /** Put the pair on their approach marks (staged) or take them as they come (live), pistols out (state 'search': drawn, weapon lights and lasers on,
   *  and calm enough that nothing they brush gets shoulder-bashed), and start them walking into the stack. Restartable: every run begins from scratch. */
  start() { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RoomClear.start', this, []); if (__r !== RUST_PASS) return __r as any; } }
    const P = this.plan;
    this.phase = 'stack'; this.tKick = this.tEnter = this.tEnter2 = this.tSearch = this.tResolve = NaN; this.done.clear(); this.flung = false; this.bGo = false; this.noMark = false; this.tSet = this.tGo = NaN; this.tNow = 0; this.aHome = this.bHome = false;
    this.aimA = v3.copy(P.stackAim[0]); this.aimB = v3.copy(P.stackAim[1]); this.swA = new Sweep(); this.swB = new Sweep(); this.restA = this.restB = null;
    this.men().forEach((gd, i) => {
      const c = gd.char;
      if (!this.live && P.from) {   // staged: stood on the approach marks facing the stack, at rest (a pose at the new spot before anyone reads his bones)
        c.pos = v3.copy(P.from[i]); c.vel = [0, 0, 0]; c.bodyYaw = c.aimYaw = Math.atan2(P.stack[i][0] - P.from[i][0], P.stack[i][2] - P.from[i][2]); c.aimPitch = 0; gd.speed = 0;
      }
      gd.state = 'search'; gd.awareness = 0; gd.lastKnown = null; gd.hold = false; gd.pinned = false; gd.reactT = 0; gd.searchT = 0; gd.script = null;
      c.anim.stance = i === 0 ? 'stack' : 'highReady'; if (!this.live) c.update(0);
    });
    this.route(this.a, this.wpA, [P.stack[0]], { speed: 1.6, face: P.stackFace[0], stance: 'stack', stackSide: P.stackSide, upper: 'none', aimAt: this.aimA, crouch: false });
    if (this.b) this.route(this.b, this.wpB, [P.stack[1]], { speed: 1.5, face: P.stackFace[1], stance: 'highReady', upper: 'none', aimAt: this.aimB, crouch: false });
  }

  /** Hand both men back to the AI (see endGuardScript). */
  end(state: 'patrol' | 'search' = 'patrol') { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RoomClear.end', this, [state]); if (__r !== RUST_PASS) return __r as any; } } endGuardScript(this.a, state); if (this.b) endGuardScript(this.b, state); }
  /** false once there is nothing left to drive: no such door, or a man reset / killed / taken off his script from under the drill */
  get intact(): boolean {
    const g = this.g, a = this.a, b = this.b;
    return !!this.door && g.guards.includes(a) && !!a.script && a.state !== 'dead' && (!b || (g.guards.includes(b) && !!b.script && b.state !== 'dead'));
  }
  /** rallied on the door after calling it — or long enough after that a man who cannot make his mark is not waited for (live sequencing) */
  get complete(): boolean { const tr = this.tNow - this.tResolve; return this.phase === 'resolve' && (tr > 7.5 || (tr > 2.8 && this.done.has('#formup') && this.aHome && this.bHome)); }

  private men(): Guard[] { return this.b ? [this.a, this.b] : [this.a]; }
  /** first time only: line keys as they are, other one-offs (signals, orders) under '#…' */
  private once(key: string): boolean { if (this.done.has(key)) return false; this.done.add(key); return true; }
  /** the plan's line for this step, once — or, with a colleague the floor already knows about lying within a few metres of the man saying it, the version of it that
   *  does not call the corner he lies in 'empty' (BODY_LINES: the room is still cleared and called, he is just not nothing — and a man out cold is said to be breathing) */
  private say(key: keyof RoomClearPlan['lines'], gd: Guard, radio = false) {
    if (!this.once(key)) return;
    const B = BODY_LINES[key] ? this.bodyHere(gd) : null;
    this.g.say(gd, B ? (isBreathing(B) ? wg(BODY_LINES_COLD_NOW[key] ?? BODY_LINES_COLD[key], BODY_LINES_COLD[key]) : wg(BODY_LINES_NOW[key] ?? BODY_LINES[key], BODY_LINES[key])) ?? this.plan.lines[key] : this.plan.lines[key], radio);
  }
  /** a found colleague lying in this room (inside its bounds when the plan has them) within a few metres of the man, in his plain sight — not one out in the corridor through the wall */
  private bodyHere(gd: Guard): Guard | null {
    const g = this.g, B = nearKnownBody(g, gd.char.pos, 3.5); if (!B) return null;
    const h = B.char.bones.hips ?? B.char.pos; if (this.plan.bounds && !inBounds([h[0], 0, h[2]], this.plan.bounds, 0.3)) return null;
    return g.col.segmentBlocked(gd.char.bones.head ?? v3.add(gd.char.pos, [0, 1.65, 0]), [h[0], h[1] + 0.3, h[2]]) ? null : B;
  }
  /** new waypoint list for a man (+ orders); advance() feeds them to his script one at a time */
  private route(gd: Guard, wps: Vec3[], pts: readonly Vec3[], patch: Partial<GuardScript> = {}) { wps.length = 0; for (const p of pts) wps.push(v3.copy(p)); order(gd, { ...patch, goal: wps[0] ?? null }); }
  /** pass intermediate waypoints on the fly (just before the runner would start settling onto them), keep the last as the goal; true once planted on the last */
  private advance(gd: Guard, wps: Vec3[]): boolean {
    while (wps.length > 1 && flat(gd.char.pos, wps[0]) < SETTLE + 0.1) { wps.shift(); order(gd, { goal: wps[0] }); }
    return wps.length <= 1 && !!gd.script?.arrived;
  }
  /** a point `dist` metres out along a facing from `p`, at height y — where a man holding that facing rests his eyes */
  private static ahead(p: Vec3, yaw: number, dist: number, y: number): Vec3 { return [p[0] + Math.sin(yaw) * dist, y, p[2] + Math.cos(yaw) * dist]; }
  /** #1 up and through the door, hooking to the open side: his muzzle checks the middle of the room, comes round into his corner, then runs the wall */
  private enter(t: number) {
    const P = this.plan; this.tEnter = t; this.phase = 'enter';
    this.route(this.a, this.wpA, P.entry[0], { speed: 2.3, face: P.podFace[0], strafe: false, stance: 'none', upper: 'aim', aimAt: this.aimA });   // (strafe off again: the step back off the boot side-stepped)
    this.restA = this.enterSweep(this.swA, this.aimA, P.entryAim[0], t);
  }
  /** A man's entry sweep, doctrine first: the room's CENTRE (what stands in the open kills you first), then the first corner of his authored list — his own —
   *  both inside CENTRE_CHECK.corner seconds of starting in; the rest of the list is handed back for the caller to run once this is done (tick: restSecs).
   *  A one-point list has nothing to hand back. */
  private enterSweep(sw: Sweep, aim: Vec3, list: readonly Vec3[], t: number): Vec3[] | null {
    const C = CENTRE_CHECK; const centre = roomCentre(this.plan);
    sw.start(aim, [centre, list[0]], t, C.corner);
    return list.length > 1 ? list.slice(1) : null;
  }
  /** The keyed breach ('open'): his hand on the lever — locked or not, staff have the key, so the bolt stays as it was and catches again whenever the leaf next
   *  latches — the leaf unlatched and shoved hard into the room off its real hinge (it bangs off its stop like the kicked one and coasts back a little), and he
   *  flows straight in behind it. A leaf already standing well open into the room is left alone: 'door's open', and in. */
  private shove(t: number) {
    const d = this.door!, P = this.plan, c = this.a.char; const s = Math.sign(P.fling) || -1;
    if (!d.off && this.g.doors.openFor(d, s) < 1.0) {
      d.latched = false; d.closing = false; d.vel = s * Math.max(Math.abs(d.vel), Math.abs(P.fling)); d.quietT = -1.0; d.creakArmed = false;
      d.lastWho = c.id;   // the bang off the stop is a guard's doing: his colleagues do not come running
      this.g.audio.play('doorBash', d.pos, 0.5, { rate: 1.15 });
    }
    this.tKick = t; this.flung = true; this.say('in1', this.a);
    this.enter(t);
  }

  tick(t: number, dt: number) { { const __s = RUST_SURFACE; if (__s) { const __r = __s.call('RoomClear.tick', this, [t, dt]); if (__r !== RUST_PASS) return __r as any; } }
    const g = this.g, P = this.plan, a = this.a, b = this.b, d = this.door;
    if (!d || !this.intact) return;   // no such door, or reset / shot out from under us: nothing more to stage
    this.tNow = t;
    let aIn = this.advance(a, this.wpA), bIn = b ? this.advance(b, this.wpB) : true;   // (re-routing a man below makes these stale for the rest of the tick: cleared there)
    if (this.swA.active) this.swA.at(t, this.aimA);
    if (this.swB.active) this.swB.at(t, this.aimB);
    const kick = (P.breach ?? 'kick') === 'kick';
    switch (this.phase) {
      case 'stack': {   // walking the last couple of metres into the stack: #1 bladed to the wall on the jamb, head and muzzle on the door; #2 tucked in behind covering high
        if (t > 0.6) this.say('stack1', a);
        let squeeze = false;
        if (!this.live) {   // staged: on the shot clock (they were stood two metres off the marks)
          if (b && t > 1.2 && t < 1.6 && bIn && this.once('#hold')) b.char.anim.signal('squeeze');   // fist up beside his head as he tucks in: set (skipped if he is late — the arm must be down again for the squeeze)
          if (b && t > 1.6) this.say('stack2', b);
          squeeze = t >= 2.5 && (!b || !b.char.anim.signalling || t > 2.9);
        } else {            // live: as they actually close up — set once both are planted (or STACK_CAP in: a man who cannot make his mark delays it, never stalls it), the squeeze a second after
          if (isNaN(this.tSet) && ((aIn && bIn) || t > STACK_CAP)) { this.tSet = t; if (b && bIn && this.once('#hold')) b.char.anim.signal('squeeze'); }
          if (b && (bIn || t > 2.2) && t > 1.0) this.say('stack2', b);
          squeeze = !isNaN(this.tSet) && t >= Math.max(2.5, this.tSet + 1.0) && (!b || !b.char.anim.signalling || t > this.tSet + 1.5);
        }
        if (squeeze) {   // the squeeze: #2 chops him forward, #1 steps off the wall square to the leaf at kicking distance — or, his own door standing open already, straight in
          if (b) { b.char.anim.signal('go'); this.say('go2', b); }
          this.tGo = t; this.phase = 'breach';
          this.noMark = !kick && !d.isClosed();   // his own door standing open already: no lever to work — a beat on the jamb for the word, then straight in from there
          if (!this.noMark) { this.route(a, this.wpA, [P.kickFrom], { speed: 1.2, face: P.kickFace, stance: 'highReady', upper: 'none' }); aIn = false; }
        }
        break;
      }
      case 'breach': {   // planted and squared up (or out of time): throw the kick — rooted from here on, chest square to the leaf like the player's kick — or work the lever and shove
        const c = a.char; const planted = aIn && flat(c.pos, P.kickFrom) < 0.07 && Math.abs(wrapAngle(c.bodyYaw - P.kickFace)) < 0.1;
        const t0 = this.live ? this.tGo : 2.5;   // (staged: the squeeze is at 2.5 on the shot clock, and these were tuned as 3.0 / 3.9)
        if (b) { this.aimB[0] = damp(this.aimB[0], d.frameCentre[0], 3, dt); this.aimB[1] = damp(this.aimB[1], 1.5, 3, dt); this.aimB[2] = damp(this.aimB[2], d.frameCentre[2], 3, dt); }   // #2's muzzle comes down onto the doorway itself
        if (this.noMark ? t > t0 + 0.45 : (planted && t > t0 + 0.5) || t > t0 + 1.4) {
          if (kick) { order(a, { goal: null, aimAt: null }); c.anim.kickDoor(); this.tKick = t; this.phase = 'kick'; }
          else { this.shove(t); aIn = false; }
        }
        break;
      }
      case 'kick': {   // the sole meets the leaf at KICK_IMPACT: unlatch it and throw it off the boot into the room — the door model swings it, bangs it off its stop and lets it rebound
        const an = a.char.anim, c = a.char;
        if (!this.flung && (an.kickImpact || an.kickTime >= KICK_IMPACT || !an.kicking)) {
          this.flung = true;
          const onIt = flat(c.pos, P.kickFrom) < 0.4 && Math.abs(wrapAngle(c.bodyYaw - P.kickFace)) < 0.35;   // (had he not made his mark in time, the boot cannot have met the leaf: it just comes unlatched with a push's worth, so the beat goes on without telekinesis)
          if (Math.abs(d.angle) < 1.4) { d.latched = false; d.closing = false; d.vel = onIt ? P.fling : P.fling * 0.32; d.sinceBash = onIt ? 0 : d.sinceBash; d.quietT = -0.5; d.creakArmed = false; }
          d.lastWho = c.id;   // whatever the leaf does next (the bang off the stop) is a guard's doing: his colleagues do not come running
          if (onIt && !d.off) { g.audio.play('doorBash', d.pos, 1.0, { rate: 0.9 }); kickScuff(g, d, c.pos); kickDustFx(g, d, c.pos); }   // his boot's scuff on the leaf, as the player's kick leaves (marks.ts) + the plaster and splinters off the jamb (GPU particles, nothing with them off)
          this.say('in1', a);
          // off the funnel while the leaf swings and the foot comes back under him: a step back toward his own stack mark (his side of the frame), chest kept on the doorway
          const k = Math.min(1, KICK_BACK / Math.max(1e-3, flat(P.kickFrom, P.stack[0])));
          this.route(a, this.wpA, [v3.lerp(P.kickFrom, P.stack[0], k)], { speed: 1.6, face: P.kickFace, strafe: true, stance: 'highReady', upper: 'none', aimAt: null }); aIn = false;
        }
        if (this.flung && (!an.kicking || t > this.tKick + 1.3)) { this.enter(t); aIn = false; }   // foot back under him: gun up and through the door
        break;
      }
      case 'enter': {
        const te = t - this.tEnter;
        // #2 a beat behind, never on #1's heels (staged: once #1 is off the kick mark; live — where there may have been no kick mark — once #1 is a length clear of him):
        // takes the doorway square, muzzle across the middle and into his corner, turns hard onto the door side and walks the leaf flat
        if (b && !this.bGo && (this.live ? te > 0.85 && flat(a.char.pos, b.char.pos) > 1.5 : te > 0.85 || flat(a.char.pos, P.kickFrom) > 1.6)) {
          this.bGo = true; this.tEnter2 = t;
          this.route(b, this.wpB, P.entry[1], { speed: 2.2, face: P.podFace[1], stance: 'none', upper: 'aim' }); bIn = false;
          this.restB = this.enterSweep(this.swB, this.aimB, P.entryAim[1], t);
        }
        if (this.restA && !this.swA.active) { this.swA.start(this.aimA, this.restA, t, restSecs(this.restA)); this.restA = null; }   // centre and corner done: the rest of his sector, the whole sweep still about three seconds
        if (b && this.restB && !this.swB.active) { this.swB.start(this.aimB, this.restB, t, restSecs(this.restB)); this.restB = null; }
        if (aIn && te > 1.5) this.say('side1', a);                       // each calls his side as he plants on it
        if (b && this.bGo && bIn && te > 1.5) this.say('side2', b);
        if (te > 3.4) {   // sectors covered from the points of domination: #1 pushes up his wall to the far corner, #2 gets his light under the table
          this.tSearch = t; this.phase = 'search'; this.restA = null;
          this.route(a, this.wpA, P.searchPath, { speed: 1.3, face: P.searchFace, upper: 'aim' }); aIn = false;
          this.swA.start(this.aimA, P.searchAimWalk, t, 3.0);
        }
        break;
      }
      case 'search': {
        const ts = t - this.tSearch;
        if (b && this.restB && !this.swB.active) { this.swB.start(this.aimB, this.restB, t, restSecs(this.restB)); this.restB = null; }   // (a #2 sent in late finishes his sector before he kneels)
        if (ts > 0.35 && b) this.say('push1', a);   // (his word to the #2 — a man alone just does it)
        if (b && ts > 0.5 && !this.restB && this.once('#kneel')) { order(b, { crouch: true }); this.swB.start(this.aimB, P.underAim, t, 3.2); }   // down on a knee, light under the table end to end
        if (b && ts > 0.9) this.say('under2', b);
        if (ts > 3.0 && (aIn || ts > 4.0) && this.once('#across')) this.swA.start(this.aimA, P.searchAimThere, t, 2.4);   // planted in the far corner: across the room from there
        if (b && ts > 3.4) this.say('nothing2', b);
        if (b && ts > 3.7 && this.once('#standup')) {   // back on his feet: onto the room's other door if it has one (squared to it: his to hold while #1 finishes), else up onto his wall
          const nd = this.nextDoor;
          if (nd) { const fc = nd.frameCentre; order(b, { crouch: false, face: Math.atan2(fc[0] - b.char.pos[0], fc[2] - b.char.pos[2]) }); this.swB.start(this.aimB, [[fc[0], 1.0, fc[2]]], t, 1.2); }
          else { order(b, { crouch: false }); this.swB.start(this.aimB, [P.upAim], t, 1.2); }
        }
        if (ts > 4.1 && (aIn || ts > 4.5)) this.say('corner1', a);
        if (ts > 4.7) { this.tResolve = t; this.phase = 'resolve'; }
        break;
      }
      case 'resolve': {   // call it, radio it in, muzzles down (the beams pool at their feet), rally on the door: #2 first out holding the doorway, #1 in behind him
        const tr = t - this.tResolve;
        this.say('empty2', b ?? a);
        if (tr > 0.9) this.say('radio1', a, true);
        if (tr > 1.0 && this.once('#lower')) { order(a, { stance: 'lowReady', upper: 'none' }); if (b) order(b, { stance: 'lowReady', upper: 'none', crouch: false }); }
        if (b && tr > 1.15 && this.once('#rally')) a.char.anim.signal('rally');
        if (tr > 1.3 && this.once('#formup')) {
          const [fa, fb] = P.formUp; const ea = fa[fa.length - 1], eb = fb[fb.length - 1];
          this.route(a, this.wpA, fa, { speed: 1.8, face: P.formFace[0], stance: 'lowReady', upper: 'none' }); aIn = false;
          this.swA.start(this.aimA, [RoomClear.ahead(ea, P.formFace[0], 3, 0.4)], t, 2.0);   // eyes ahead and low: three metres out along the facing each will hold at the door
          if (b) { this.route(b, this.wpB, fb, { speed: 1.2, face: P.formFace[1], stance: 'lowReady', upper: 'none' }); bIn = false; this.swB.start(this.aimB, [RoomClear.ahead(eb, P.formFace[1], 3, 0.4)], t, 1.5); }
        }
        break;
      }
    }
    this.aHome = aIn; this.bHome = bIn;
  }
}
/** live stack: seconds after start() by which the squeeze goes ahead whether or not both men made their marks (they start ≤ 2.5 m off them; two seconds is usual) */
const STACK_CAP = 6;
/** metres the man who booted the leaf steps back toward his own stack mark while it swings (off the funnel; capped at the mark itself) */
const KICK_BACK = 0.7;
/** the entry sweep's doctrine (RoomClear.enterSweep): the muzzle is across the room's centre and INTO his own corner `corner` seconds after he starts in (two eased
 *  legs: the centre at half that), then the rest of his authored sector in what is left of the old three-second sweep (`total`), no leg quicker than `minLeg` */
const CENTRE_CHECK = { corner: 0.95, total: 3.0, minLeg: 0.6 };
/** seconds for the rest of an entry sweep: what is left of the old three-second sweep after the centre check and the corner, but never under minLeg a leg */
const restSecs = (rest: readonly Vec3[]) => Math.max(CENTRE_CHECK.minLeg * rest.length, CENTRE_CHECK.total - CENTRE_CHECK.corner);
/** the middle of the room at muzzle height: the plan's bounds when it has them, else the centroid of everything both men's entry sweeps visit (the tour's plan has no bounds) */
function roomCentre(P: RoomClearPlan): Vec3 {
  const B = P.bounds; if (B) return [(B.x0 + B.x1) / 2, 1.0, (B.z0 + B.z1) / 2];
  const pts = [...P.entryAim[0], ...P.entryAim[1]]; let x = 0, z = 0; for (const p of pts) { x += p[0]; z += p[2]; }
  return [x / pts.length, 1.0, z / pts.length];
}
/** Another door of the room the plan clears, for #2 to hold: of the level's doors whose frame stands on the room's bounds (a hand's pad either way) other than the
 *  one they came through, the nearest to #2's point of domination — null when the plan has no bounds (the tour's) or the room no second door. */
function nextDoorOf(g: Game, P: RoomClearPlan): Door | null {
  const B = P.bounds; if (!B) return null;
  const pod = P.entry[1][P.entry[1].length - 1] ?? P.stack[1];
  let best: Door | null = null, bd = Infinity;
  for (const d of g.doors.list) {
    if (d.def.name === P.door) continue;
    const fc = d.frameCentre; if (fc[0] < B.x0 - 0.3 || fc[0] > B.x1 + 0.3 || fc[2] < B.z0 - 0.3 || fc[2] > B.z1 + 0.3) continue;
    const dd = flat(fc, pod); if (dd < bd) { bd = dd; best = d; }
  }
  return best;
}
/** what a man says instead of the plan's line when a found colleague lies within a few metres of him as he says it (RoomClear.say) — shot dead… */
const BODY_LINES: Partial<Record<keyof RoomClearPlan['lines'], string>> = {
  side1: "my side's clear. …he's right here.", side2: "door side clear — apart from him.",
  under2: 'checking under… just him.', nothing2: 'nothing else down here.', corner1: "corner's clear. only him.",
  empty2: "room's clear — nobody but him.",
};
/** (WORDS) the two tables' lines as they are said now, where one changed (read in RoomClear.say) */
const BODY_LINES_NOW: Partial<Record<keyof RoomClearPlan['lines'], string>> = { side2: 'door side clear — except for him.' };
const BODY_LINES_COLD_NOW: Partial<Record<keyof RoomClearPlan['lines'], string>> = { nothing2: 'nothing else down here. he took a hard hit.' };
/** …or put out cold and left breathing (isBreathing: the takedown) — the same steps, the words of men stepping round a colleague who will wake up sore */
const BODY_LINES_COLD: Partial<Record<keyof RoomClearPlan['lines'], string>> = {
  side1: "my side's clear. …he's here — still breathing.", side2: "door side clear — just him, out cold.",
  under2: 'checking under… just him. breathing.', nothing2: 'nothing else down here. he took a proper hit.', corner1: "corner's clear. only him.",
  empty2: "room's clear — nobody but him, and he's breathing.",
};



export const inBounds = (p: Vec3, B: RoomClearPlan['bounds'], pad = 0) => !!B && p[0] > B.x0 - pad && p[0] < B.x1 + pad && p[2] > B.z0 - pad && p[2] < B.z1 + pad;
export type ClearStage = 'muster' | 'approach' | 'drill' | 'done';
/** The lockdown pair clearing rooms (guards.ts planClears deals it when an alarm has come to nothing on a locked-down floor, or the panel asks): the rooms nearest
 *  the last fix, one after another — walk to the door, run the drill above live, 'moving to the …', the next — then both handed back calm to the paired patrol.
 *  A room may come with more than one plan (one per door it can be taken from): the variant whose stack is the shorter walk from where they stand is the one
 *  run. Standing in the very room, or when the walk would cut through it (an office reached by its side door from the one just cleared), they go out through
 *  their own door first and round; and the walk ends a length past the marks on the stack's own side, so they always turn into it in file, #1 leading.
 *  It owns the one thing the runner takes away from scripted men: their senses. Every frame both are run through perceive(), and anything real ends the
 *  choreography there and then, for both, with the right thing on their minds — a sighting or a bump or gunfire or a door going in: 'CONTACT!' toward it (the one
 *  who saw shoots, the other converges); a creak, footsteps, a light dying: 'huh?' and over they go; a colleague calling contact or a man down on the net: to
 *  that; the partner dropping (a shot they will have heard anyway — a silent takedown they notice as the talk stops): over to where he was; the mains going:
 *  edgy, and whoever setBlackout picked for the breaker keeps that errand. The floor standing down, the AI off, the player dead or a reset dissolve it quietly. */
export class Clearing {
  stage: ClearStage = 'muster';
  roomI = 0; drill: RoomClear | null = null;
  /** the variant of the current room being walked to / run (chosen at approach time) */
  cur: RoomClearPlan | null = null;
  private t = 0; private total = 0; private eta = 0; private tDrill = 0; private tNear = NaN; private aw = [0, 0];   // (aw: each man's awareness as his senses left it last frame)
  private wps: [Vec3[], Vec3[]] = [[], []];   // approach waypoints still ahead of each man (script.goal is [0])
  private readonly dark: boolean;
  constructor(private g: Game, readonly rooms: RoomClearPlan[][], readonly a: Guard, readonly b: Guard | null) {
    this.dark = g.blackout.active; this.cur = rooms[0]?.[0] ?? null;
  }
  private name(p: RoomClearPlan | null | undefined): string { return p?.room ?? p?.door ?? 'room'; }
  /** one line for the panel / debugging: which room, what stage (the drill's phase inside it), who, what is next */
  summary(): string {
    const r = this.cur; if (!r || this.stage === 'done') return '';
    const who = (gd: Guard) => this.g.level.routes[gd.routeI]?.name ?? `guard ${gd.char.id - 10}`;
    const next = this.rooms.slice(this.roomI + 1).map(v => this.name(v[0])).join(', ');
    return `clearing: ${this.name(r)}${this.stage !== 'muster' ? ` (by the ${r.door} door)` : ''} · ${this.stage === 'drill' && this.drill ? this.drill.phase : this.stage} · ${who(this.a)}${this.b ? ' + ' + who(this.b) : ' (alone)'}${next ? ' · then ' + next : ''} · ${Math.round(this.total)} s`;
  }
  /** hand whoever of ours is still scripted back to the AI calm (finished, given up, or the floor stood down under us) */
  cancel() { for (const gd of this.men()) if (gd.script && this.g.guards.includes(gd)) endGuardScript(gd, 'patrol'); this.stage = 'done'; this.drill = null; }
  private men(): Guard[] { return this.b ? [this.a, this.b] : [this.a]; }


}
