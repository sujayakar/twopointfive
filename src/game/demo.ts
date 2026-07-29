import { Vec3, v3 } from "../core/math";
import { Equipment } from "./equipment";
import { Guards } from "./guards";
import { Player } from "./player";

// ---------------------------------------------------------------------------
// Scripted run, for recording.
//
// The arc is dark -> light -> kill the light -> see in the dark you made ->
// silent kill -> loud kills -> leave. Every beat is motivated by stealth
// rather than by having something to show off, which is also what makes it
// show the right things: the flashlight beat exists because you cannot see,
// the night vision beat exists because you just removed the only light, and
// the muzzle flashes exist because the silent option ran out.
//
// It drives the player's *inputs*, not its results, so a recorded run goes
// through the same acceleration, wall tucking, aim smoothing and animation as
// a played one. A separate cutscene path would be free to drift away from how
// the game actually looks, which would make the recording a lie.
// ---------------------------------------------------------------------------

/** How close counts as having arrived at a waypoint. */
const ARRIVE = 0.55;

export interface DemoDeps {
  player: Player;
  guards: Guards;
  equipment: Equipment;
  /** Fires the OCP at a world point, exactly as the trigger would. */
  fireOCP: (at: Vec3) => void;
  /**
   * The E key: take down, pick up, or drop, whichever is possible. The demo
   * calls the same handler the key does rather than a second implementation.
   */
  interact: () => void;
  setNightVision: (on: boolean) => void;
  /**
   * Whether a straight walk between two points is unobstructed.
   *
   * The demo has no pathfinding and should not grow any. What it needs instead
   * is to never *choose* a target it cannot walk to: the first cut steered at
   * whichever guard was nearest by straight-line distance, which was sometimes
   * one standing behind a wall, and the player spent the beat pressed against
   * it. Filtering candidates by this is the whole of the navigation.
   */
  clearPath: (from: Vec3, to: Vec3) => boolean;
}

interface Beat {
  name: string;
  /** Seconds. The beat also ends early once `until` returns true. */
  seconds: number;
  enter?: (d: DemoDeps) => void;
  update?: (d: DemoDeps, t: number, dt: number) => void;
  until?: (d: DemoDeps) => boolean;
}

/**
 * Waypoints, in the order the run visits them.
 *
 * Authored against the level's real geometry, and measured rather than
 * guessed. The east corridor at z ~= 1 is walkable from x = -13 to x = 22; the
 * first cut ran west instead, straight into the wall at x = -14, and the player
 * spent two beats pressed against it going nowhere.
 *
 * The lamp is the one at (1, 3.9): close to the corridor, unobstructed from
 * it, and comfortably inside the OCP's 22m reach from where the run stops.
 */
const APPROACH = v3(-6, 0, 1);
const FIRING_POINT = v3(-1.5, 0, 1);
const LIGHT_AT = v3(1, 2.1, 3.9);
const DRAG_TO = v3(-8.5, 0, 1);
/** Where the run expects to find the guard it takes down. */
const TAKEDOWN_SPOT = v3(-3, 0, 0.8);
/** Somewhere for the held guard to be looking: away, down the corridor. */
const LOOKING_AWAY = v3(14, 1.2, 1);

/**
 * Puts the mark where the script expects it and keeps it there.
 *
 * Snapping once at the start is not enough — the patrol simply walks off
 * during the four beats before it is needed, and by the takedown it was 24m
 * away. Holding it is also the better shot: a guard standing still with his
 * back turned is what the player is supposed to be looking for.
 */
function holdMark(d: DemoDeps): void {
  const g = d.guards.all[0];
  if (!g || g.dead) return;
  g.hear(LOOKING_AWAY);
}
const EXIT = v3(22, 0, 2);

/** Nearest live guard the player could actually walk to. */
function reachableGuard(d: DemoDeps, range: number) {
  let best = null;
  let bestD = range;
  for (const g of d.guards.all) {
    if (g.dead || g.carried) continue;
    const dist = Math.hypot(g.pos.x - d.player.pos.x, g.pos.z - d.player.pos.z);
    if (dist >= bestD) continue;
    if (!d.clearPath(d.player.pos, g.pos)) continue;
    best = g;
    bestD = dist;
  }
  return best;
}

function steer(player: Player, to: Vec3): boolean {
  const dx = to.x - player.pos.x;
  const dz = to.z - player.pos.z;
  const d = Math.hypot(dx, dz);
  if (d < ARRIVE) {
    player.scriptMove = null;
    return true;
  }
  player.scriptMove = { x: dx / d, z: dz / d };
  return false;
}

/** Look at a world point, which is also what turns the character and torch. */
function look(player: Player, at: Vec3): void {
  player.scriptAim = { x: at.x, z: at.z };
}

const BEATS: Beat[] = [
  {
    // Opens lit and moving, so the light gauge has somewhere to fall from.
    // Starting hidden would make the meter look broken rather than meaningful.
    name: "exposed",
    seconds: 6,
    enter: (d) => {
      d.player.flashlightOn = false;
      d.equipment.select(1);
    },
    update: (d) => {
      look(d.player, APPROACH);
      steer(d.player, APPROACH);
    },
  },
  {
    // The torch beat. A cone of volumetric light with the bounce off the far
    // wall doing the rest of the work.
    name: "torch",
    seconds: 6,
    enter: (d) => { d.player.flashlightOn = true; },
    update: (d) => {
      look(d.player, LIGHT_AT);
      steer(d.player, FIRING_POINT);
    },
  },
  {
    // Kill the light. The OCP is silent and temporary, which is what makes it
    // the setup for a takedown rather than a replacement for one.
    name: "ocp",
    seconds: 4,
    enter: (d) => {
      d.equipment.select(2);
      look(d.player, LIGHT_AT);
      d.fireOCP(LIGHT_AT);
    },
    update: (d) => { d.player.scriptMove = null; },
  },
  {
    // Torch off, tube on. The room is dark because of something the player
    // did, and this is the answer to it.
    name: "nightvision",
    seconds: 5,
    enter: (d) => {
      d.player.flashlightOn = false;
      d.setNightVision(true);
      d.equipment.select(0);
      d.guards.all[0]?.snapNearest(TAKEDOWN_SPOT.x, TAKEDOWN_SPOT.z);
    },
    update: (d) => {
      holdMark(d);
      const g = d.guards.all[0];
      if (g) look(d.player, g.pos);
      d.player.scriptMove = null;
    },
  },
  {
    name: "takedown",
    seconds: 9,
    update: (d, t) => {
      holdMark(d);
      const g = d.guards.all[0];
      if (!g || g.dead) return;
      look(d.player, g.pos);
      if (!steer(d.player, g.pos)) return;
      if (t > 0.4) d.interact();
    },
    until: (d) => d.player.carrying,
  },
  {
    // Drag the body out of the light. Equipment stows itself while carrying.
    name: "drag",
    seconds: 6,
    update: (d) => {
      look(d.player, DRAG_TO);
      steer(d.player, DRAG_TO);
    },
  },
  {
    // The silent option has run out. Tube off — the flashes would white it
    // out — and the pistol comes up.
    name: "shoot",
    seconds: 11,
    enter: (d) => {
      d.interact();
      d.setNightVision(false);
      d.player.flashlightOn = true;
      d.equipment.select(1);
    },
    update: (d, t) => {
      d.player.scriptMove = null;
      const g = reachableGuard(d, 40);
      if (!g) return;
      look(d.player, g.pos);
      // Spaced out, so each flash is its own event and the guards have time to
      // turn toward the last one.
      if (t % 1.6 < 1 / 60) d.player.scriptFire = true;
    },
  },
  {
    name: "exit",
    seconds: 9,
    enter: (d) => { d.player.sprinting = true; },
    update: (d) => {
      look(d.player, EXIT);
      steer(d.player, EXIT);
    },
  },
];

/**
 * Plays the scripted run.
 *
 * Advanced from the frame loop rather than from a timer, so it stays in step
 * with the render even when frames are long — a recording that drifts against
 * its own animation is worse than no recording.
 */
export class Demo {
  private beat = 0;
  private t = 0;
  running = false;

  constructor(private deps: DemoDeps) {}

  get label(): string {
    return this.running ? BEATS[this.beat].name : "";
  }

  start(): void {
    this.beat = 0;
    this.t = 0;
    this.running = true;
    BEATS[0].enter?.(this.deps);
  }

  stop(): void {
    this.running = false;
    const p = this.deps.player;
    p.scriptMove = null;
    p.scriptAim = null;
    p.scriptFire = false;
    this.deps.setNightVision(false);
  }

  update(dt: number): void {
    if (!this.running) return;
    const b = BEATS[this.beat];
    b.update?.(this.deps, this.t, dt);
    this.t += dt;
    if (this.t < b.seconds && !b.until?.(this.deps)) return;

    this.beat++;
    this.t = 0;
    if (this.beat >= BEATS.length) {
      this.stop();
      return;
    }
    BEATS[this.beat].enter?.(this.deps);
  }
}
