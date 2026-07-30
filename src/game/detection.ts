import { Vec3, clamp, v3 } from "../core/math";
import { Guard, Guards } from "./guards";
import { NavGrid } from "./nav";
import { Player } from "./player";
import { Raycaster } from "./raycast";
import { TORCH_TINT } from "../scene/scene";

// ---------------------------------------------------------------------------
// Guard perception and behaviour.
//
// Detection is the lighting you are looking at, not a parallel system with its
// own opinion: the signal a guard scores is built from the same light-meter
// value the HUD shows (the GPU probe at the player's chest), the same beam the
// tracer draws (the view cone is the torch's cone axis), and the same static
// geometry the raycaster shoots bullets through (line of sight). A clear ray is
// necessary but never sufficient: eyes are on the player only while the signal
// is non-zero. If the meter says you are dark and the beam is not on you, no
// guard can see you — by construction, not by tuning. What a guard *does* in
// the dark is chase the last lit position and sweep his torch across it, and
// the torch is what finds you.
//
// This file is the brain. It reads the world at a fixed CPU cadence, keeps a
// suspicion accumulator per guard, and steers each Guard body through its
// route / hold / nav modes. The body owns pose, torch and weapon.
// ---------------------------------------------------------------------------

/**
 * Every number the behaviour hangs off, live on the tweak panel exactly like
 * movementTuning. Distances in metres, angles in degrees, rates per second.
 */
export const detectionTuning = {
  /** Perception ticks per second per guard. Vision is cheap; 15 is plenty. */
  perceptionHz: 15,
  /**
   * Half-angle of the view cone about the beam axis. The beam is the eye: a
   * little past the torch's 30° outer cone to take in the spill around it.
   */
  fovDeg: 38,
  viewRange: 22,
  eyeHeight: 1.6,
  /**
   * What the guard's line of sight aims at on the player. Standing, the head
   * clears the 1.35 m cubicle partitions; crouched, the target and the light
   * probe drop together below their top edge — the eye and the meter must
   * measure the same body, or the meter promises safety the eye ignores.
   */
  targetStand: 1.5,
  targetCrouch: 0.85,
  /**
   * Where the GPU light probe sits on the player. Chest, and it follows the
   * stance — the meter reads the light on the body that is actually there.
   */
  probeStand: 1.15,
  probeCrouch: 0.85,
  /** LOS stops this far short of the target so a body against a wall is not
   * occluded by the wall it is leaning on. */
  losMargin: 0.15,
  /**
   * Light level below which the eye contributes nothing. Pinned to the HUD's
   * HIDDEN band edge (visibility.ts) so the meter reading HIDDEN and no eye
   * scoring you are the same statement.
   */
  litMin: 0.25,
  /** Suspicion per second for a fully lit target at point-blank range. */
  seenRate: 1.2,
  /** A target inside this guard's own beam, close, detects fast regardless
   * of the ambient it is standing in. */
  beamRange: 12,
  beamRate: 2.5,
  /** Suspicion decay per second while nothing is stimulating this guard. */
  decayRate: 0.06,
  /** Band thresholds on the suspicion accumulator. */
  suspiciousAt: 0.25,
  suspiciousExit: 0.12,
  alertAt: 0.65,
  /** Below this, an alert guard with no line of sight gives up and searches. */
  alertExit: 0.35,
  // -- hearing ---------------------------------------------------------------
  gunshotRange: 34,
  /** Sprinting is loud; walking and crouch-walking are silent. */
  footstepRange: 8,
  /** Suspicion per second from footsteps at point blank; falls off linearly. */
  footstepRate: 0.55,
  thudRange: 4.5,
  thudSuspicion: 0.4,
  /** A guard going alert shouts; colleagues in this radius come running. */
  calloutRange: 18,
  // -- behaviour -------------------------------------------------------------
  /** Torch sweep either side of the stimulus bearing while suspicious. */
  sweepDeg: 50,
  sweepPeriod: 2.6,
  /** Seconds spent sweeping a searched spot before returning to the route. */
  searchTime: 8,
  /** Metres per second when closing on a last-known position. */
  alertSpeed: 2.6,
  // -- weapon ------------------------------------------------------------------
  fireRange: 24,
  /** Seconds of continuous line of sight before the first shot: the beat that
   * lets a spotted player break contact instead of dying to a reflex. */
  fireReaction: 0.7,
  fireInterval: 0.55,
  /** Cone half-angle the shots scatter over. */
  fireSpreadDeg: 3.5,
};

/** Beam tints. Cool torch cooling → amber → red as the guard's attention hardens. */
const TINT_CALM: [number, number, number] = TORCH_TINT;
const TINT_SUSPICIOUS: [number, number, number] = [1.0, 0.62, 0.28];
const TINT_ALERT: [number, number, number] = [1.0, 0.18, 0.06];

const DEG = Math.PI / 180;

/** Player capsule for guard bullets — the same shape the player's shots use on guards. */
const PLAYER_RADIUS = 0.32;

/**
 * xorshift32, seeded. Fire cadence and spread jitter come from here rather
 * than Math.random, so a scenario that pins the world produces the same shots
 * on every run.
 */
function seededRandom(seed: number): () => number {
  let s = seed >>> 0 || 1;
  return () => {
    s ^= s << 13; s >>>= 0;
    s ^= s >>> 17;
    s ^= s << 5; s >>>= 0;
    return s / 4294967296;
  };
}

export type NoiseKind = "gunshot" | "thud" | "callout";

/** HUD readout: the loudest guard, described. */
export interface DetectionSummary {
  /** Highest suspicion across live guards, 0..1. */
  level: number;
  label: "HIDDEN" | "SUSPICIOUS" | "SEEN" | "HUNTED";
  /** True while any guard currently has eyes on the player. Drives the vignette. */
  seen: boolean;
}

export class Detection {
  readonly tuning = detectionTuning;
  private rng = seededRandom(0xdec0de);
  /** Set for one frame when a guard's bullet connects. main.ts owns the death. */
  playerHit = false;
  /** True once any guard has gone alert since the last reset — the GHOST test. */
  everAlerted = false;

  constructor(
    private readonly guards: Guards,
    private readonly raycaster: Raycaster,
    private readonly nav: NavGrid,
  ) {
    // Stagger the perception cadence so N guards do not all raycast on the
    // same frame.
    const period = 1 / detectionTuning.perceptionHz;
    guards.all.forEach((g, i) => {
      g.perceptTimer = -(i / Math.max(guards.count, 1)) * period;
    });
  }

  reset(): void {
    this.rng = seededRandom(0xdec0de);
    this.playerHit = false;
    this.everAlerted = false;
    const period = 1 / detectionTuning.perceptionHz;
    this.guards.all.forEach((g, i) => {
      g.perceptTimer = -(i / Math.max(this.guards.count, 1)) * period;
    });
  }

  /**
   * A sound heard at `origin`, pointing attention at `at` (the same point,
   * except for a callout: the shout comes from the guard, the finger points
   * at the intruder). Returns how many guards it moved.
   *
   * Broadcast rather than traced: sound goes around corners, and working out
   * who has line of sight would be both wrong and more expensive.
   */
  noise(at: Vec3, kind: NoiseKind, source?: Guard, origin: Vec3 = at): number {
    const T = detectionTuning;
    const range = kind === "gunshot" ? T.gunshotRange
      : kind === "thud" ? T.thudRange
      : T.calloutRange;
    let n = 0;
    for (const g of this.guards.all) {
      if (g === source || g.dead) continue;
      const dx = origin.x - g.pos.x, dz = origin.z - g.pos.z;
      if (dx * dx + dz * dz > range * range) continue;
      // A guard staring at the player already has better information than an
      // echo; do not drag its attention off to a stale point.
      const engaged = g.state === "alert" && g.sees;
      if (kind === "thud") {
        g.suspicion = clamp(g.suspicion + T.thudSuspicion, 0, 1);
      } else {
        g.suspicion = Math.max(g.suspicion, T.alertAt + 0.02);
      }
      // A shout is second-hand knowledge: whoever hears it runs to the spot
      // but does not shout again. Without this the callouts relay guard to
      // guard until one gunshot has the whole building hunting.
      if (kind === "callout") g.calloutDone = true;
      if (!engaged) g.stimulus = v3(at.x, 0, at.z);
      n++;
    }
    return n;
  }

  update(dt: number, player: Player, lit: number): void {
    this.playerHit = false;
    const T = detectionTuning;
    const period = 1 / T.perceptionHz;

    // Sprinting footsteps: continuous rather than an event, since the sound is
    // continuous. Loudness falls off with distance to the runner.
    const speed = Math.hypot(player.velX, player.velZ);
    const sprinting = player.sprinting && speed > 1.0 && !player.dead;

    for (const g of this.guards.all) {
      if (g.dead || g.carried) continue;

      if (sprinting) {
        const dx = player.pos.x - g.pos.x, dz = player.pos.z - g.pos.z;
        const d = Math.hypot(dx, dz);
        if (d < T.footstepRange) {
          g.suspicion = clamp(g.suspicion + T.footstepRate * (1 - d / T.footstepRange) * dt, 0, 1);
          if (!(g.state === "alert" && g.sees)) g.stimulus = v3(player.pos.x, 0, player.pos.z);
        }
      }

      g.perceptTimer += dt;
      if (g.perceptTimer >= period) {
        const dtP = g.perceptTimer;
        g.perceptTimer = 0;
        this.perceive(g, player, lit, dtP);
        this.transition(g, player, dtP);
      }
      g.stateTime += dt;
      this.steer(g, player, dt);
    }
  }

  /** Vision + hearing decay, one tick. Writes signal/suspicion onto the guard. */
  private perceive(g: Guard, player: Player, lit: number, dtP: number): void {
    const T = detectionTuning;
    let signal = 0;
    let hasLOS = false;
    let inBeam = false;

    if (!player.dead) {
      const eyeY = g.pos.y + T.eyeHeight;
      const targetY = player.pos.y + (player.crouching ? T.targetCrouch : T.targetStand);
      const dx = player.pos.x - g.pos.x;
      const dy = targetY - eyeY;
      const dz = player.pos.z - g.pos.z;
      const distXZ = Math.hypot(dx, dz);
      g.distToPlayer = distXZ;
      if (distXZ <= T.viewRange) {
        const d3 = Math.max(Math.hypot(dx, dy, dz), 1e-6);
        const dir = g.light.dir; // the beam axis IS the eye
        const cosA = (dx * dir.x + dy * dir.y + dz * dir.z) / d3;
        if (cosA >= Math.cos(T.fovDeg * DEG)) {
          const eye = v3(g.pos.x, eyeY, g.pos.z);
          const target = v3(player.pos.x, targetY, player.pos.z);
          // Static geometry only — bodies do not hide you, so the missing
          // dynamic boxes are correct here, not an omission.
          if (!this.raycaster.blocked(eye, target, T.losMargin)) {
            hasLOS = true;
            // Own-beam term: inside this guard's torch cone at close range you
            // are found fast whatever the ambient light says, because the light
            // finding you is his.
            const cosOuter = g.light.cosOuter, cosInner = g.light.cosInner;
            const beamAtten = clamp((cosA - cosOuter) / Math.max(cosInner - cosOuter, 1e-4), 0, 1);
            const beamF = beamAtten * clamp(1 - distXZ / T.beamRange, 0, 1);
            inBeam = beamF > 0.25;
            // Ambient term: light and closeness multiply. A dim figure across the
            // room is not a sighting; the same figure at arm's length is.
            const litF = clamp((lit - T.litMin) / (1 - T.litMin), 0, 1);
            const distF = clamp(1 - distXZ / T.viewRange, 0, 1);
            signal = litF * distF * T.seenRate + beamF * T.beamRate;
          }
        }
      }
    } else {
      g.distToPlayer = Infinity;
    }

    g.hasLOS = hasLOS;
    g.inBeam = inBeam;
    g.signal = signal;
    // Eyes on the player only while light is doing the seeing. A clear ray to
    // a body in the dark carries nothing: no rise, no position, and the trail
    // goes cold like any other lost contact.
    g.sees = hasLOS && signal > 1e-3;
    if (g.sees) {
      g.suspicion = clamp(g.suspicion + signal * dtP, 0, 1);
      g.stimulus = v3(player.pos.x, 0, player.pos.z);
    } else {
      g.suspicion = clamp(g.suspicion - T.decayRate * dtP, 0, 1);
    }

    if (g.state !== "alert") this.checkBodies(g);
  }

  /**
   * A guard whose eye reaches a downed, un-carried colleague treats it as
   * proof there is an intruder: straight to alert, searching from the body.
   * Bodies never leave the dynamic box list, so the only new work is one ray.
   */
  private checkBodies(g: Guard): void {
    const T = detectionTuning;
    const eyeY = g.pos.y + T.eyeHeight;
    for (const b of this.guards.all) {
      if (!b.dead || b.carried || b.reported) continue;
      const dx = b.pos.x - g.pos.x, dz = b.pos.z - g.pos.z;
      const dy = 0.3 - eyeY;
      const distXZ = Math.hypot(dx, dz);
      if (distXZ > T.viewRange) continue;
      const d3 = Math.max(Math.hypot(dx, dy, dz), 1e-6);
      const dir = g.light.dir;
      const cosA = (dx * dir.x + dy * dir.y + dz * dir.z) / d3;
      if (cosA < Math.cos(T.fovDeg * DEG)) continue;
      const eye = v3(g.pos.x, eyeY, g.pos.z);
      if (this.raycaster.blocked(eye, v3(b.pos.x, 0.3, b.pos.z), T.losMargin)) continue;
      b.reported = true;
      g.suspicion = Math.max(g.suspicion, T.alertAt + 0.02);
      g.stimulus = v3(b.pos.x, 0, b.pos.z);
      return;
    }
  }

  /** State machine, evaluated on the perception cadence. */
  private transition(g: Guard, player: Player, dtP: number): void {
    const T = detectionTuning;
    const s = g.suspicion;
    switch (g.state) {
      case "patrol":
        if (s >= T.alertAt) this.enter(g, "alert", player);
        else if (s >= T.suspiciousAt) this.enter(g, "suspicious", player);
        break;
      case "suspicious":
        if (s >= T.alertAt) this.enter(g, "alert", player);
        else if (s <= T.suspiciousExit && g.stateTime > 1.5) this.enter(g, "patrol", player);
        break;
      case "alert":
        // Real elapsed time, not the nominal tick: a slow frame stretches the
        // period, and the reaction delay is a promise in seconds.
        if (g.sees) g.aimTime += dtP;
        else g.aimTime = 0;
        // Lost them and the trail has gone cold: search where they last were.
        if (!g.sees && (s < T.alertExit || (g.mode === "nav" && g.arrived))) {
          this.enter(g, "search", player);
        }
        break;
      case "search":
        if (s >= T.alertAt && g.sees) this.enter(g, "alert", player);
        else if (g.arrived && g.stateTime > T.searchTime) {
          // Giving up is a conclusion: the swept spot was empty, so what is
          // left of the suspicion goes with it rather than tipping the guard
          // straight back into standing and staring at nothing.
          g.suspicion = Math.min(g.suspicion, T.suspiciousExit);
          this.enter(g, "patrol", player);
        }
        break;
    }
  }

  private enter(g: Guard, state: Guard["state"], player: Player): void {
    g.state = state;
    g.stateTime = 0;
    switch (state) {
      case "patrol": {
        // Walk back to the loop the long way round the furniture rather than
        // teleporting: path to the nearest point on the route, then carry on.
        g.aimTime = 0;
        const back = g.nearestRoutePoint(g.pos.x, g.pos.z);
        const path = this.nav.findPath(g.pos.x, g.pos.z, back.x, back.z);
        if (path && path.length > 1) {
          path[path.length - 1] = [back.x, back.z];
          g.follow(path, 1.35);
          g.rejoinTravelled = back.travelled;
        } else {
          g.resumeRoute(back.travelled);
        }
        break;
      }
      case "suspicious":
        g.hold(g.yaw);
        break;
      case "alert": {
        g.aimTime = 0;
        g.arrived = false;
        g.repathTimer = 0;
        this.everAlerted = true;
        if (!g.stimulus) g.stimulus = v3(player.pos.x, 0, player.pos.z);
        // Call it out. Everyone in earshot converges on the same point, so
        // being seen by one guard turns into being hunted by three.
        if (!g.calloutDone) {
          g.calloutDone = true;
          this.noise(g.stimulus, "callout", g, g.pos);
        }
        break;
      }
      case "search":
        g.calloutDone = false;
        g.arrived = false;
        if (g.stimulus) {
          const path = this.nav.findPath(g.pos.x, g.pos.z, g.stimulus.x, g.stimulus.z);
          if (path && path.length > 0) g.follow(path, 1.6);
          else g.arrived = true;
        } else {
          g.arrived = true;
        }
        break;
    }
  }

  /** Per-frame steering for the current state: facing, sweeps, paths, firing. */
  private steer(g: Guard, player: Player, dt: number): void {
    const T = detectionTuning;
    switch (g.state) {
      case "patrol": {
        this.setTint(g, TINT_CALM);
        // Finishing an excursion: hop back onto the loop where the path ends.
        if (g.mode === "nav" && g.arrived) {
          g.resumeRoute(g.rejoinTravelled);
          g.calloutDone = false;
        }
        break;
      }
      case "suspicious": {
        this.setTint(g, TINT_SUSPICIOUS);
        if (g.stimulus) {
          const base = Math.atan2(g.stimulus.x - g.pos.x, g.stimulus.z - g.pos.z);
          // Sweep the beam either side of the noise: the eye is the torch, so
          // this is a search you can watch — and dodge.
          const sweep = Math.sin((g.stateTime / T.sweepPeriod) * Math.PI * 2) * T.sweepDeg * DEG;
          g.hold(base + sweep);
        } else {
          g.hold(g.yaw);
        }
        break;
      }
      case "alert": {
        this.setTint(g, TINT_ALERT);
        if (g.sees && !player.dead) {
          // Face the player and fire. The torch pins them; the reaction delay
          // is the moment they have to break contact.
          const to = Math.atan2(player.pos.x - g.pos.x, player.pos.z - g.pos.z);
          g.hold(to);
          this.tryFire(g, player, dt);
        } else {
          this.pursue(g, dt);
        }
        break;
      }
      case "search": {
        this.setTint(g, TINT_ALERT);
        if (g.mode === "nav" && !g.arrived) break;
        // At the spot: a slow wide sweep until the timer sends the guard back.
        const sweep = Math.sin((g.stateTime / (T.sweepPeriod * 1.4)) * Math.PI * 2) * 80 * DEG;
        const base = g.stimulus
          ? Math.atan2(g.stimulus.x - g.pos.x, g.stimulus.z - g.pos.z)
          : g.yaw;
        g.hold(base + sweep);
        break;
      }
    }
  }

  /** Alert without eyes on the player: run to where they were last known. */
  private pursue(g: Guard, dt: number): void {
    if (!g.stimulus) { g.hold(g.yaw); return; }
    g.repathTimer -= dt;
    // The timer paces every replan, including the unreachable-goal case:
    // findPath returning null must cost one A* per repath period, not one
    // per frame.
    const stale = g.mode === "nav" && g.arrived && !this.nearStimulus(g);
    if (g.repathTimer > 0 && !stale) return;
    g.repathTimer = 0.8;
    const path = this.nav.findPath(g.pos.x, g.pos.z, g.stimulus.x, g.stimulus.z);
    if (path && path.length > 0) g.follow(path, detectionTuning.alertSpeed);
    else g.hold(Math.atan2(g.stimulus.x - g.pos.x, g.stimulus.z - g.pos.z));
  }

  private nearStimulus(g: Guard): boolean {
    if (!g.stimulus) return true;
    return Math.hypot(g.stimulus.x - g.pos.x, g.stimulus.z - g.pos.z) < 1.5;
  }

  private setTint(g: Guard, t: [number, number, number]): void {
    g.torchTarget.x = t[0];
    g.torchTarget.y = t[1];
    g.torchTarget.z = t[2];
  }

  private tryFire(g: Guard, player: Player, dt: number): void {
    const T = detectionTuning;
    if (g.aimTime < T.fireReaction) return;
    if (g.distToPlayer > T.fireRange) return;
    g.fireTimer -= dt;
    if (g.fireTimer > 0) return;
    if (!g.fire()) return;
    g.fireTimer = T.fireInterval * (0.85 + 0.3 * this.rng());
    // The report is heard whether or not the round connects.
    this.noise(g.pos, "gunshot", g);
    if (this.resolveShot(g, player)) this.playerHit = true;
  }

  /**
   * One round from the guard's muzzle toward the player's chest, scattered by
   * a seeded spread and stopped by the same static geometry the player's own
   * bullets use. Returns true on a hit.
   */
  private resolveShot(g: Guard, player: Player): boolean {
    const T = detectionTuning;
    const m = g.muzzle();
    const tx = player.pos.x, tz = player.pos.z;
    const ty = player.pos.y + (player.crouching ? 0.75 : 1.15);
    let dx = tx - m.pos.x, dy = ty - m.pos.y, dz = tz - m.pos.z;
    const dist = Math.hypot(dx, dy, dz);
    if (dist < 1e-4) return true;
    dx /= dist; dy /= dist; dz /= dist;
    // Yaw the shot by the spread; a real pistol at 10 m does not group inside
    // a torso every round, and neither should a guard.
    const spread = (this.rng() * 2 - 1) * T.fireSpreadDeg * DEG;
    const cs = Math.cos(spread), sn = Math.sin(spread);
    const dir = v3(dx * cs - dz * sn, dy + (this.rng() * 2 - 1) * T.fireSpreadDeg * DEG * 0.5, dx * sn + dz * cs);
    const dl = Math.hypot(dir.x, dir.y, dir.z);
    dir.x /= dl; dir.y /= dl; dir.z /= dl;

    const wall = this.raycaster.raycast(m.pos, dir, dist + 1.0);
    const reach = wall ? wall.t : dist + 1.0;
    return hitCapsule(player.pos, player.crouching, m.pos, dir, reach) !== null;
  }

  /** The HUD's one-line answer: how much trouble is the player in. */
  summary(): DetectionSummary {
    const T = detectionTuning;
    let level = 0;
    let anyAlert = false;
    let seen = false;
    let anySuspicious = false;
    for (const g of this.guards.all) {
      if (g.dead) continue;
      level = Math.max(level, g.suspicion);
      if (g.state === "alert" || g.state === "search") anyAlert = true;
      if (g.state === "alert" && g.sees) seen = true;
      if (g.state === "suspicious" || g.suspicion >= T.suspiciousAt) anySuspicious = true;
    }
    const label: DetectionSummary["label"] = seen ? "SEEN"
      : anyAlert ? "HUNTED"
      : anySuspicious ? "SUSPICIOUS"
      : "HIDDEN";
    return { level, label, seen };
  }

  /** Debug/scenario view: one plain object per guard, safe to JSON. */
  snapshot(): Array<Record<string, unknown>> {
    return this.guards.all.map((g, i) => ({
      index: i,
      dead: g.dead,
      state: g.state,
      mode: g.mode,
      suspicion: +g.suspicion.toFixed(3),
      signal: +g.signal.toFixed(3),
      hasLOS: g.hasLOS,
      sees: g.sees,
      inBeam: g.inBeam,
      dist: +g.distToPlayer.toFixed(2),
      pos: [+g.pos.x.toFixed(2), +g.pos.z.toFixed(2)],
      yaw: +g.yaw.toFixed(3),
      stimulus: g.stimulus ? [+g.stimulus.x.toFixed(2), +g.stimulus.z.toFixed(2)] : null,
      torch: [+g.light.color.x.toFixed(2), +g.light.color.y.toFixed(2), +g.light.color.z.toFixed(2)],
    }));
  }
}

/**
 * Ray vs the player's standing capsule (a vertical cylinder about the spine),
 * mirroring Guard.hitScan so both sides shoot at the same kind of target.
 */
export function hitCapsule(
  feet: Vec3, crouching: boolean, origin: Vec3, dir: Vec3, tmax: number,
): number | null {
  const R = PLAYER_RADIUS;
  const LO = 0.15, HI = crouching ? 1.05 : 1.75;
  const ox = origin.x - feet.x;
  const oz = origin.z - feet.z;
  const a = dir.x * dir.x + dir.z * dir.z;
  if (a < 1e-9) return null;
  const b = 2 * (ox * dir.x + oz * dir.z);
  const c = ox * ox + oz * oz - R * R;
  const disc = b * b - 4 * a * c;
  if (disc < 0) return null;
  const t = (-b - Math.sqrt(disc)) / (2 * a);
  if (t < 0 || t > tmax) return null;
  const y = origin.y + dir.y * t;
  if (y < feet.y + LO || y > feet.y + HI) return null;
  return t;
}
