// Runtime analytic light list → GPU buffer (struct Light in common.wgsl, 64 B each).
import { Vec3, v3, DEG } from '../math/vec';
import { makeBuffer } from '../gpu/device';
import { LightDef } from '../scene/level';

export interface RtLight {
  name: string; group: string;
  kind: 0 | 1 | 2 | 3;        // point, spot, directional, area (one-sided disk: dir = normal, radius = disk radius)
  pos: Vec3; dir: Vec3;
  color: Vec3; intensity: number;
  range: number; innerDeg: number; outerDeg: number;
  radius: number;             // physical size (m): clamps 1/d^2 and shortens shadow rays
  volumetric: number;
  owner: number;              // character id whose proxies don't shadow this light (e.g. the guard holding it)
  enabled: boolean;
  fixtureBox: number;         // emissive glow-card box index or -1
  flicker: number;            // 0 = steady; >0 = flicker amount (runtime modulation)
  /** transient lights (muzzle flashes) expire when ttl hits 0 */
  ttl: number; age: number; peakIntensity: number;
  /** wide, soft emitter (ceiling panels, screens, street lamps): its own denoise class in the direct pass. Explicit, not radius-derived. */
  broad?: boolean;
}

export const MAX_LIGHTS = 64;

export function lightFromDef(d: LightDef): RtLight {
  return {
    name: d.name, group: d.group, kind: d.kind === 'point' ? 0 : d.kind === 'spot' ? 1 : 2,
    pos: v3.copy(d.pos), dir: v3.normalize(d.dir), color: v3.copy(d.color), intensity: d.intensity,
    range: d.range, innerDeg: d.innerDeg, outerDeg: d.outerDeg, radius: d.kind === 'dir' ? (d.radius ?? 0) : (d.radius ?? 0.05),
    volumetric: d.volumetric, owner: 0, enabled: d.enabled, fixtureBox: d.fixtureBox ?? -1, flicker: 0, ttl: -1, age: 0, peakIntensity: d.intensity, broad: !!d.broad,
  };
}

export class LightSet {
  lights: RtLight[] = [];
  buf: GPUBuffer;
  private data = new ArrayBuffer(MAX_LIGHTS * 64);
  private f32 = new Float32Array(this.data); private u32 = new Uint32Array(this.data);
  /** number of lights uploaded last frame (enabled only) */
  activeCount = 0;
  /** indices (into `lights`) of the uploaded lights, in GPU order */
  activeIdx: number[] = [];

  constructor(private device: GPUDevice) {
    this.buf = makeBuffer(device, MAX_LIGHTS * 64, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST, 'lights');
  }

  add(l: RtLight): RtLight { this.lights.push(l); return l; }
  remove(l: RtLight) { const i = this.lights.indexOf(l); if (i >= 0) this.lights.splice(i, 1); }

  /** Spawn a short-lived point light (muzzle flash). */
  flash(pos: Vec3, color: Vec3, intensity: number, range: number, ttl: number, owner: number): RtLight {
    return this.add({ name: 'flash', group: 'transient', kind: 0, pos: v3.copy(pos), dir: [0, -1, 0], color, intensity, range, innerDeg: 0, outerDeg: 180, radius: 0.08, volumetric: 1.0, owner, enabled: true, fixtureBox: -1, flicker: 0, ttl, age: 0, peakIntensity: intensity });
  }

  update(dt: number, time: number) {
    for (let i = this.lights.length - 1; i >= 0; i--) {
      const l = this.lights[i];
      if (l.ttl >= 0) {
        // a flash shorter than a frame must still deliver its energy: intensity for THIS frame = the (1-x)^2.2 envelope averaged
        // over [age, age+dt] (F = its antiderivative), evaluated BEFORE advancing age; retired only once a frame has shown it
        if (l.age >= l.ttl) { this.lights.splice(i, 1); continue; }
        const x0 = l.age / l.ttl, x1 = Math.min(1, (l.age + Math.max(dt, 1e-4)) / l.ttl);
        const F = (x: number) => -Math.pow(1 - x, 3.2) / 3.2;
        l.intensity = l.peakIntensity * (F(x1) - F(x0)) / Math.max(x1 - x0, 1e-6);
        l.age += dt;
      }
    }
    void time;
  }

  upload(time: number) {
    const f = this.f32, u = this.u32; let n = 0; this.activeIdx.length = 0;
    for (let i = 0; i < this.lights.length && n < MAX_LIGHTS; i++) {
      const l = this.lights[i]; if (!l.enabled || l.intensity <= 0) continue;
      let inten = l.intensity;
      if (l.flicker > 0) {
        const s = Math.sin(time * 61 + i * 3.1) * Math.sin(time * 17.3 + i) + Math.sin(time * 7.1 + i * 0.7) * 0.5;
        inten *= 1 - l.flicker * (s > 0.55 ? 0.85 : s > 0.3 ? 0.3 : 0.0);
      }
      const o = n * 16;
      // kind low byte + flags: 256 = transient (ttl light: excluded from accumulated signals), 512 = broad emitter class (wide penumbrae → its own denoise)
      f[o] = l.pos[0]; f[o + 1] = l.pos[1]; f[o + 2] = l.pos[2]; u[o + 3] = l.kind | (l.ttl >= 0 ? 256 : 0) | (l.broad ? 512 : 0);
      f[o + 4] = l.dir[0]; f[o + 5] = l.dir[1]; f[o + 6] = l.dir[2]; f[o + 7] = l.range;
      f[o + 8] = l.color[0] * inten; f[o + 9] = l.color[1] * inten; f[o + 10] = l.color[2] * inten; f[o + 11] = Math.cos(l.outerDeg * DEG);
      f[o + 12] = Math.cos(Math.min(l.innerDeg, l.outerDeg - 0.5) * DEG); f[o + 13] = l.volumetric; u[o + 14] = l.owner >>> 0; f[o + 15] = l.radius;
      this.activeIdx.push(i); n++;
    }
    this.activeCount = n;
    this.device.queue.writeBuffer(this.buf, 0, this.data, 0, Math.max(64, n * 64));
    return n;
  }
}
