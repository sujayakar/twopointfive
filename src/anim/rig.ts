// ---------------------------------------------------------------------------
// Skeletal animation runtime.
//
// Consumes the packed rig produced by tools/extract-rig.mjs. Because the
// extractor resampled every clip onto a fixed frame rate, sampling here is an
// array index and a lerp — there is no per-channel keyframe search at runtime.
//
// Transforms are kept as (position, quaternion) rather than matrices. The rig
// has no animated scale, so every joint is a rigid transform, and composing
// those is both cheaper and immune to the shear that creeps into repeated
// matrix multiplication.
// ---------------------------------------------------------------------------

export interface BoneDef {
  name: string;
  parent: number;
  t: [number, number, number];
  r: [number, number, number, number];
  s: [number, number, number];
}

interface ClipHeader {
  name: string;
  frames: number;
  duration: number;
  loop: boolean;
  rotOffset: number;
  posOffset: number;
  posBones: number[];
}

interface RigHeader {
  fps: number;
  bones: BoneDef[];
  clips: ClipHeader[];
}

export interface Clip {
  name: string;
  frames: number;
  duration: number;
  loop: boolean;
  /** frames * boneCount * 4, normalised from int16. */
  rot: Float32Array;
  /** frames * posBones.length * 3. */
  pos: Float32Array;
  posBones: number[];
}

/** A pose: local rotation per bone, plus local translation per bone. */
export interface Pose {
  rot: Float32Array; // boneCount * 4
  pos: Float32Array; // boneCount * 3
}

export function makePose(boneCount: number): Pose {
  return { rot: new Float32Array(boneCount * 4), pos: new Float32Array(boneCount * 3) };
}

// --- quaternion helpers -----------------------------------------------------

export function quatSlerp(
  a: Float32Array, ai: number,
  b: Float32Array, bi: number,
  t: number,
  out: Float32Array, oi: number,
): void {
  let ax = a[ai], ay = a[ai + 1], az = a[ai + 2], aw = a[ai + 3];
  let bx = b[bi], by = b[bi + 1], bz = b[bi + 2], bw = b[bi + 3];
  let cos = ax * bx + ay * by + az * bz + aw * bw;
  // Take the shorter arc; without this, blends occasionally spin a joint the
  // long way round and the limb visibly snaps.
  if (cos < 0) { cos = -cos; bx = -bx; by = -by; bz = -bz; bw = -bw; }
  let s0: number, s1: number;
  if (cos > 0.9995) {
    s0 = 1 - t; s1 = t;
  } else {
    const theta = Math.acos(cos);
    const sin = Math.sin(theta);
    s0 = Math.sin((1 - t) * theta) / sin;
    s1 = Math.sin(t * theta) / sin;
  }
  let x = s0 * ax + s1 * bx, y = s0 * ay + s1 * by;
  let z = s0 * az + s1 * bz, w = s0 * aw + s1 * bw;
  const inv = 1 / (Math.hypot(x, y, z, w) || 1);
  out[oi] = x * inv; out[oi + 1] = y * inv; out[oi + 2] = z * inv; out[oi + 3] = w * inv;
}

/** out = a * b (apply b, then a). */
export function quatMul(
  a: Float32Array, ai: number, b: Float32Array, bi: number, out: Float32Array, oi: number,
): void {
  const ax = a[ai], ay = a[ai + 1], az = a[ai + 2], aw = a[ai + 3];
  const bx = b[bi], by = b[bi + 1], bz = b[bi + 2], bw = b[bi + 3];
  out[oi] = aw * bx + ax * bw + ay * bz - az * by;
  out[oi + 1] = aw * by - ax * bz + ay * bw + az * bx;
  out[oi + 2] = aw * bz + ax * by - ay * bx + az * bw;
  out[oi + 3] = aw * bw - ax * bx - ay * by - az * bz;
}

/** Rotates vector (vx,vy,vz) by quaternion q at qi, writing to out at oi. */
export function quatRotate(
  q: Float32Array, qi: number,
  vx: number, vy: number, vz: number,
  out: Float32Array, oi: number,
): void {
  const x = q[qi], y = q[qi + 1], z = q[qi + 2], w = q[qi + 3];
  const tx = 2 * (y * vz - z * vy);
  const ty = 2 * (z * vx - x * vz);
  const tz = 2 * (x * vy - y * vx);
  out[oi] = vx + w * tx + (y * tz - z * ty);
  out[oi + 1] = vy + w * ty + (z * tx - x * tz);
  out[oi + 2] = vz + w * tz + (x * ty - y * tx);
}

// --- rig --------------------------------------------------------------------

export class Rig {
  readonly bones: BoneDef[];
  readonly clips = new Map<string, Clip>();
  readonly boneIndex = new Map<string, number>();

  /** World-space rotation per bone, in rig space. */
  readonly worldRot: Float32Array;
  /** World-space position per bone, in rig space. */
  readonly worldPos: Float32Array;

  private constructor(header: RigHeader, bin: ArrayBuffer) {
    this.bones = header.bones;
    this.bones.forEach((b, i) => this.boneIndex.set(b.name, i));
    const n = this.bones.length;
    this.worldRot = new Float32Array(n * 4);
    this.worldPos = new Float32Array(n * 3);

    for (const c of header.clips) {
      // Copy rather than view: the packed offsets are only guaranteed 4-byte
      // aligned, and a misaligned typed-array view throws.
      const rotCount = c.frames * n * 4;
      const raw = new Int16Array(bin.slice(c.rotOffset, c.rotOffset + rotCount * 2));
      const rot = new Float32Array(rotCount);
      for (let i = 0; i < rotCount; i++) rot[i] = raw[i] / 32767;

      const posCount = c.frames * c.posBones.length * 3;
      const pos = new Float32Array(bin.slice(c.posOffset, c.posOffset + posCount * 4));

      this.clips.set(c.name, {
        name: c.name, frames: c.frames, duration: c.duration, loop: c.loop,
        rot, pos, posBones: c.posBones,
      });
    }
  }

  static async load(base = import.meta.env.BASE_URL ?? "/"): Promise<Rig> {
    const [header, bin] = await Promise.all([
      fetch(`${base}rig.json`).then((r) => {
        if (!r.ok) throw new Error(`rig.json ${r.status}`);
        return r.json() as Promise<RigHeader>;
      }),
      fetch(`${base}rig.bin`).then((r) => {
        if (!r.ok) throw new Error(`rig.bin ${r.status}`);
        return r.arrayBuffer();
      }),
    ]);
    return new Rig(header, bin);
  }

  get boneCount(): number {
    return this.bones.length;
  }

  clip(name: string): Clip {
    const c = this.clips.get(name);
    if (!c) throw new Error(`no such clip: ${name} (have ${[...this.clips.keys()].join(", ")})`);
    return c;
  }

  /** Samples a clip at `time` seconds into `out`. Wraps if the clip loops. */
  sample(clip: Clip, time: number, out: Pose): void {
    const n = this.boneCount;
    let t = clip.loop ? ((time % clip.duration) + clip.duration) % clip.duration
                      : Math.min(Math.max(time, 0), clip.duration);

    const fpos = clip.duration > 0 ? (t / clip.duration) * (clip.frames - 1) : 0;
    let f0 = Math.floor(fpos);
    let f1 = f0 + 1;
    const frac = fpos - f0;
    if (f1 >= clip.frames) f1 = clip.loop ? 0 : clip.frames - 1;
    if (f0 >= clip.frames) f0 = clip.frames - 1;

    const b0 = f0 * n * 4;
    const b1 = f1 * n * 4;
    for (let b = 0; b < n; b++) {
      quatSlerp(clip.rot, b0 + b * 4, clip.rot, b1 + b * 4, frac, out.rot, b * 4);
    }

    // Bones without a translation track keep their rest offset.
    for (let b = 0; b < n; b++) {
      const rest = this.bones[b].t;
      out.pos[b * 3] = rest[0];
      out.pos[b * 3 + 1] = rest[1];
      out.pos[b * 3 + 2] = rest[2];
    }
    const m = clip.posBones.length;
    if (m > 0) {
      const p0 = f0 * m * 3;
      const p1 = f1 * m * 3;
      for (let k = 0; k < m; k++) {
        const b = clip.posBones[k];
        for (let c = 0; c < 3; c++) {
          out.pos[b * 3 + c] =
            clip.pos[p0 + k * 3 + c] * (1 - frac) + clip.pos[p1 + k * 3 + c] * frac;
        }
      }
    }
  }

  /**
   * Blends b into a by `t`, optionally masked per bone.
   *
   * `mask` is how the upper body can hold an aim pose while the legs run a
   * locomotion cycle — without it, walking would swing the arms and the
   * flashlight would wander off the cursor.
   */
  blendInto(a: Pose, b: Pose, t: number, out: Pose, mask?: Float32Array): void {
    const n = this.boneCount;
    for (let i = 0; i < n; i++) {
      const w = mask ? t * mask[i] : t;
      if (w <= 0) {
        out.rot[i * 4] = a.rot[i * 4]; out.rot[i * 4 + 1] = a.rot[i * 4 + 1];
        out.rot[i * 4 + 2] = a.rot[i * 4 + 2]; out.rot[i * 4 + 3] = a.rot[i * 4 + 3];
        out.pos[i * 3] = a.pos[i * 3]; out.pos[i * 3 + 1] = a.pos[i * 3 + 1];
        out.pos[i * 3 + 2] = a.pos[i * 3 + 2];
        continue;
      }
      quatSlerp(a.rot, i * 4, b.rot, i * 4, w, out.rot, i * 4);
      for (let c = 0; c < 3; c++) {
        out.pos[i * 3 + c] = a.pos[i * 3 + c] * (1 - w) + b.pos[i * 3 + c] * w;
      }
    }
  }

  /**
   * Composes local transforms into rig-space world transforms.
   * Bones are ordered parents-before-children, so one forward pass suffices.
   */
  computeWorld(pose: Pose): void {
    const n = this.boneCount;
    const tmp = new Float32Array(3);
    for (let i = 0; i < n; i++) {
      const p = this.bones[i].parent;
      if (p < 0) {
        this.worldRot.set(pose.rot.subarray(i * 4, i * 4 + 4), i * 4);
        this.worldPos.set(pose.pos.subarray(i * 3, i * 3 + 3), i * 3);
      } else {
        quatMul(this.worldRot, p * 4, pose.rot, i * 4, this.worldRot, i * 4);
        quatRotate(
          this.worldRot, p * 4,
          pose.pos[i * 3], pose.pos[i * 3 + 1], pose.pos[i * 3 + 2],
          tmp, 0,
        );
        this.worldPos[i * 3] = this.worldPos[p * 3] + tmp[0];
        this.worldPos[i * 3 + 1] = this.worldPos[p * 3 + 1] + tmp[1];
        this.worldPos[i * 3 + 2] = this.worldPos[p * 3 + 2] + tmp[2];
      }
    }
  }

  /** Builds a per-bone weight mask from a set of root bone names (inclusive of descendants). */
  maskFrom(roots: string[]): Float32Array {
    const mask = new Float32Array(this.boneCount);
    const rootIdx = new Set(roots.map((r) => this.boneIndex.get(r)).filter((i) => i !== undefined));
    for (let i = 0; i < this.boneCount; i++) {
      let cur: number = i;
      while (cur >= 0) {
        if (rootIdx.has(cur)) { mask[i] = 1; break; }
        cur = this.bones[cur].parent;
      }
    }
    return mask;
  }
}
