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
   * The smoke canister: a strong sustained source that follows its canister.
   * Barely warm and heavy, so it billows out and pools along the floor rather
   * than shooting for the ceiling.
   */
  canisterCloud(follow: () => Vec3, seconds = 8): void {
    this.spawn({
      pos: follow(),
      radius: 0.7,
      vel: { x: 0, y: 0.35, z: 0 },
      push: 2,
      density: 120,
      temp: 1.6,
      life: seconds,
      attack: 0.4,
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

  /** Ages sources and packs the live ones for the solver. */
  update(dt: number): void {
    let n = 0;
    const out = this.packed;
    for (let i = this.list.length - 1; i >= 0; i--) {
      const s = this.list[i];
      const life = s.life ?? Infinity;
      if (s.age >= life) { this.list.splice(i, 1); continue; }
      let env = 1;
      if (Number.isFinite(life)) {
        const attack = s.attack ?? Math.min(0.05, life * 0.2);
        const t = s.age / life;
        env = Math.min(1, s.age / Math.max(attack, 1e-3));
        // Fade over the last quarter so an emitter stops without a step.
        if (t > 0.75) env *= (1 - t) / 0.25;
      }
      let density = s.density * env;
      if (s.instantAmount !== undefined) {
        // Delivered whole in this one frame: rate = amount / dt. The attack
        // envelope must not apply — it is 0 at age 0, and since only `density`
        // was written past it, an instant source's temperature was silently
        // dropped and every puff came out cold whatever it asked for.
        density = s.instantAmount / Math.max(dt, 1e-3);
        env = 1;
        s.age = life; // gone after this pack
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
        out[o + 10] = 0;
        out[o + 11] = 0;
        n++;
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
