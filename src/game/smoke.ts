import { Vec3 } from "../core/math";
import { FLUID_MAX_SOURCES, FLUID_SOURCE_STRIDE } from "../engine/fluid";

// ---------------------------------------------------------------------------
// Smoke sources.
//
// Nothing here is smoke — these are the *emitters* the fluid simulation splats
// into its density/temperature/velocity fields each step (see fluid.wgsl's
// Source struct). The medium itself lives on the GPU: it advects, curls,
// pools and thins on its own once emitted, so every effect below is only a
// question of where, how much, how hot and how hard it pushes:
//
//   - a muzzle burst is small, hot and thrown down the barrel;
//   - an impact puff is cool and thrown off the surface;
//   - a shot-out fixture smoulders — a thin warm trickle for ~20 s;
//   - always-on wisps (coffee steam, a server rack's exhaust) prove the medium
//     is alive before anyone fires;
//   - the smoke canister is a strong sustained emitter that follows its rolling
//     canister.
//
// Density is the tracer's dimensionless density unit (sigma_t = 0.05/m per
// unit at the default `volumetric`), so a source's `density` rate times its
// dwell time is roughly the optical thickness it lays down.
// ---------------------------------------------------------------------------

export interface SmokeSourceSpec {
  pos: Vec3;
  /** Soft-sphere radius, metres. */
  radius: number;
  /** Velocity the medium is dragged toward inside the sphere, m/s. */
  vel?: Vec3;
  /** Drag rate, 1/s: how hard the sphere imposes `vel` on the flow. */
  push?: number;
  /** Density added per second at the core. */
  density: number;
  /** Temperature added per second at the core (buoyancy: hot rises). */
  temp?: number;
  /**
   * Expansion rate at the core, 1/s — how hard this source blows the air
   * apart. See the `expand` field in fluid.wgsl's Source for why this is not
   * simply a radial `vel`: the pressure projection would delete that.
   *
   * As an order of magnitude, div(v) ~ 3v/r, so a 0.5 m burst pushing out at
   * 10 m/s is about 60.
   */
  expand?: number;
  /** Seconds the source emits; Infinity for a permanent wisp. */
  life?: number;
  /** Seconds the emission ramps up from nothing. */
  attack?: number;
  /** Moving emitter: re-read every frame (a rolling canister). */
  follow?: () => Vec3;
  /** Survives reset() — a fixture of the level, not an event. */
  permanent?: boolean;
}

interface Live extends SmokeSourceSpec {
  age: number;
  /** One-frame delivery of a fixed density amount (a debug/legacy puff). */
  instantAmount?: number;
}

export class Smoke {
  private list: Live[] = [];
  /** Packed Source structs, uploaded to the solver every frame. */
  readonly packed = new Float32Array(FLUID_MAX_SOURCES * FLUID_SOURCE_STRIDE);
  /** Live packed sources this frame. */
  count = 0;
  /**
   * Debug: while set, sources age but pack nothing — the solver sees zero
   * emitters. The mass-drift / decay measurements need a field with no
   * inflow, and the always-on wisps otherwise have no off switch.
   */
  silenced = false;

  /** A pistol shot: a hot fistful of gas driven along the barrel. */
  muzzle(pos: Vec3, dir: Vec3): void {
    this.spawn({
      pos: { x: pos.x + dir.x * 0.35, y: pos.y + dir.y * 0.35, z: pos.z + dir.z * 0.35 },
      radius: 0.42,
      vel: { x: dir.x * 4.5, y: dir.y * 4.5 + 0.6, z: dir.z * 4.5 },
      push: 60,
      density: 70,
      temp: 25,
      life: 0.14,
    });
  }

  /** A round biting a wall: cool dust, thrown off the surface. */
  impact(at: Vec3, away: Vec3): void {
    this.spawn({
      pos: at,
      radius: 0.32,
      vel: { x: away.x * 1.2, y: away.y * 1.2 + 0.4, z: away.z * 1.2 },
      push: 20,
      density: 55,
      temp: 2,
      life: 0.22,
    });
  }

  /**
   * A shot-out light: the fixture smoulders. Slow, thin, warm — it hugs the
   * ceiling it hangs from and creeps outward for the whole ~20 s.
   */
  smolder(at: Vec3, seconds = 20): void {
    this.spawn({
      pos: { x: at.x, y: at.y - 0.15, z: at.z },
      radius: 0.35,
      vel: { x: 0, y: 0.25, z: 0 },
      push: 3,
      density: 4.5,
      temp: 4,
      life: seconds,
      attack: 1.5,
    });
  }

  /** Coffee steam: a thread of warm haze that rises and thins to nothing. */
  coffee(pos: Vec3): void {
    this.spawn({
      pos, radius: 0.16, vel: { x: 0, y: 0.35, z: 0 }, push: 6,
      density: 6, temp: 14, life: Infinity, permanent: true,
    });
  }

  /** Server-rack exhaust: a lazy warm updraught venting from the cabinet top. */
  serverExhaust(pos: Vec3): void {
    this.spawn({
      pos, radius: 0.4, vel: { x: 0, y: 0.5, z: 0 }, push: 4,
      density: 3.5, temp: 12, life: Infinity, permanent: true,
    });
  }

  /**
   * A flashbang's smoke: a hard expanding burst, then the column it leaves.
   *
   * Two sources rather than one because they are two different events. The
   * burst is over in a quarter second and its job is `expand` — it blows a
   * hole in the air, and the projection turns that into an outward push that
   * keeps moving after the source is gone. The column is what you actually
   * look at for the next several seconds, and it is buoyant rather than
   * violent. One source cannot be both without the tail inheriting the
   * burst's expansion and the room quietly gaining volume for six seconds.
   *
   * The bang itself is a light, not smoke — hang it on the transient list
   * (see flashes.ts) at the same position.
   */
  flashbang(at: Vec3): void {
    this.spawn({
      pos: at, radius: 0.6,
      // Dense because it expands. Once the advection stopped manufacturing
      // mass under divergence, `expand` became what it should always have
      // been — a redistribution, not a multiplier — so the same 90/s that
      // used to read as a wall of smoke now spreads to a peak of 0.73 and
      // vanishes. Measured: 450/s holds a visible core across the burst.
      density: 450, temp: 30, expand: 70,
      push: 0, life: 0.25, attack: 0.02,
    });
    this.spawn({
      pos: at, radius: 0.4,
      vel: { x: 0, y: 1.2, z: 0 }, push: 5,
      density: 26, temp: 7, life: 6, attack: 0.3,
    });
  }

  /**
   * The smoke canister: a cold, dense, directional vent.
   *
   * Reference is a smoke grenade lying on tarmac — a jet that leaves the can
   * sideways under pressure, hugs the ground and piles up, not a plume that
   * rises. Every number here follows from the solver's own force terms
   * (fluid.wgsl's `forces`: v.y += (buoyancy*temp - weight*dens)*dt):
   *
   * - `temp: 0`, not 1.6. Any temperature at all puts buoyancy*temp against
   *   weight*dens, and buoyancy (1.4/unit) beats weight (0.045/unit) unless
   *   the density is ~30x the temperature. At temp 1.6 the old preset was
   *   fighting its own weight term and drifting upward.
   * - `weight` then acts alone: 0.045 * 120 = 5.4 m/s^2 downward at the core,
   *   which is what pins it to the floor.
   * - `push`/`vel` are the jet. push is a drag RATE (1/s): at 2 the medium
   *   relaxed toward `vel` over half a second, i.e. barely. 45 imposes it
   *   inside a frame, which is what a pressurised vent does.
   * - No `expand`: a vent is momentum, not detonation. Expansion is the
   *   flashbang's mechanism and it thins the cloud (see flashbang above),
   *   which is the opposite of what a concealment device wants.
   *
   * @param axis unit vector the can is venting along, in world space.
   */
  canisterCloud(follow: () => Vec3, seconds = 30, axis?: Vec3): void {
    const a = axis ?? { x: 0, y: 0, z: 1 };
    const speed = 9;
    this.spawn({
      pos: follow(),
      radius: 0.55,
      // A little upward bias so the jet clears the ground it is lying on and
      // then settles, rather than scrubbing along the floor from the first cell.
      vel: { x: a.x * speed, y: a.y * speed + 0.4, z: a.z * speed },
      push: 45,
      density: 200,
      temp: 0,
      life: seconds,
      attack: 0.15,
      follow,
    });
  }

  /**
   * Debug: an instant blob (the retired __smokeTest, now delivered through
   * the simulation). `amount` is peak density; the sim carries it from here.
   *
   * Cold, deliberately: no temperature, so buoyancy has nothing to act on and
   * the solver's weight term is the only force on it. A puff therefore sinks
   * and pools, which is the canister's regime — for a rising plume, spawn a
   * sustained source with `temp` (see `smolder`).
   */
  puff(x: number, y: number, z: number, radius: number, amount: number): void {
    this.spawn({
      pos: { x, y, z }, radius,
      density: 0, life: 0.02, instantAmount: amount,
    });
  }

  /** Adds an emitter; the named presets above are all wrappers over this. */
  spawn(spec: SmokeSourceSpec & { instantAmount?: number }): void {
    const src: Live = { ...spec, age: 0 };
    if (this.list.length < FLUID_MAX_SOURCES) {
      this.list.push(src);
      return;
    }
    // Oldest-fraction transient loses its slot; permanent wisps never do.
    let worst = -1, worstFrac = -1;
    for (let i = 0; i < this.list.length; i++) {
      const s = this.list[i];
      if (s.permanent) continue;
      const frac = s.age / Math.max(s.life ?? 1, 1e-3);
      if (frac > worstFrac) { worstFrac = frac; worst = i; }
    }
    if (worst >= 0) this.list[worst] = src;
  }

  /**
   * Ages sources and packs the live ones for the solver.
   *
   * An instant source is delivered whole on one frame, so it is only aged out
   * once it has actually been packed into a frame the solver will step. All
   * three ways a frame can inject nothing are checked: `simulating` false (the
   * renderer skips the fluid step entirely), dt = 0 (a frozen clock — the
   * renderer skips it then too), and `silenced`. Any of them would otherwise
   * consume the puff without injecting it, and clearing the condition
   * afterwards could not bring it back.
   *
   * @param simulating whether the renderer will step the solver this frame
   *                   (`settings.fluidSim`). Only gates instant delivery —
   *                   sustained sources still age, so a solver toggled off and
   *                   on does not resurrect expired emitters.
   */
  update(dt: number, simulating = true): void {
    let n = 0;
    const out = this.packed;
    const canDeliver = simulating && dt > 0 && !this.silenced;
    for (let i = this.list.length - 1; i >= 0; i--) {
      const s = this.list[i];
      const life = s.life ?? Infinity;
      const amount = s.instantAmount;
      if (amount === undefined && s.age >= life) { this.list.splice(i, 1); continue; }
      // An undelivered instant source waits instead of ageing, so it cannot be
      // spent on a frame that injects nothing.
      if (amount !== undefined && !canDeliver) continue;
      let env = 1;
      if (Number.isFinite(life)) {
        const attack = s.attack ?? Math.min(0.05, life * 0.2);
        const t = s.age / life;
        env = Math.min(1, s.age / Math.max(attack, 1e-3));
        // Fade over the last quarter so an emitter stops without a step.
        if (t > 0.75) env *= (1 - t) / 0.25;
      }
      let density = s.density * env;
      if (amount !== undefined) {
        // Delivered whole in this one frame: rate = amount / dt. The attack
        // envelope must not apply — it is 0 at age 0, and since only `density`
        // was written past it, an instant source's temperature was silently
        // dropped and every puff came out cold whatever it asked for.
        density = amount / Math.max(dt, 1e-3);
        env = 1;
      }
      if (n < FLUID_MAX_SOURCES && !this.silenced) {
        const p = s.follow ? s.follow() : s.pos;
        const o = n * FLUID_SOURCE_STRIDE;
        out[o] = p.x; out[o + 1] = p.y; out[o + 2] = p.z;
        out[o + 3] = s.radius;
        out[o + 4] = s.vel?.x ?? 0;
        out[o + 5] = s.vel?.y ?? 0;
        out[o + 6] = s.vel?.z ?? 0;
        out[o + 7] = s.push ?? 0;
        out[o + 8] = density;
        out[o + 9] = (s.temp ?? 0) * env;
        // Envelope-scaled like the other rates, so a burst that ramps in does
        // not shove the air before it has any smoke to shove.
        out[o + 10] = (s.expand ?? 0) * env;
        out[o + 11] = 0;
        n++;
        if (amount !== undefined) {
          // Spent: an ordinary expired source now, evicted on the next update.
          s.instantAmount = undefined;
          s.age = life;
        }
      } else if (amount !== undefined) {
        // Pack table full — retry next frame rather than vanish.
        continue;
      }
      s.age += dt;
    }
    this.count = n;
  }

  /**
   * Level restart: transients go, the fixtures' wisps stay. `all` drops the
   * permanent wisps too — a scenario that needs a room with no emitters.
   */
  reset(all = false): void {
    this.list = all ? [] : this.list.filter((s) => s.permanent);
    this.count = 0;
  }
}
