import { IDENTITY3, Vec3, v3 } from "../core/math";

// ---------------------------------------------------------------------------
// Scene primitives.
//
// The whole world is oriented boxes. That is not a shortcut for its own sake:
// the reference art (cubicle walls, desks, crates, carpet slabs) is genuinely
// box-shaped, and a ray-OBB slab test is ~5-10x cheaper than a triangle
// intersection while being exact. A BVH over a few thousand boxes traverses
// fast enough to path trace in real time.
// ---------------------------------------------------------------------------

export interface Material {
  albedo: Vec3;
  roughness: number;
  /** Radiance emitted per unit area, in the same linear units as light intensity. */
  emissive: Vec3;
  metallic: number;
}

export interface Box {
  center: Vec3;
  half: Vec3;
  /** Row-major 3x3 rotation. Rows are the box's local axes in world space. */
  rot: [Vec3, Vec3, Vec3];
  material: number;
  /** Extra FLAG_* bits OR'd into the packed flags word. */
  flags?: number;
}

export const LIGHT_SPHERE = 0;
export const LIGHT_SPOT = 1;

export interface Light {
  pos: Vec3;
  kind: number;
  dir: Vec3;
  /** Emitter radius. Drives penumbra size — this is what makes shadows soft. */
  radius: number;
  color: Vec3;
  intensity: number;
  cosInner: number;
  cosOuter: number;
  /**
   * World half-extents of the emitting box, for slab emitters.
   *
   * When set, NEE samples this box instead of a sphere of `radius`. Absent or
   * zero falls back to the sphere, which is what point-like and directional
   * lights want.
   */
  halfExtents?: Vec3;
}

export const BOX_STRIDE_F32 = 20; // 80 bytes
export const MATERIAL_STRIDE_F32 = 8; // 32 bytes
export const LIGHT_STRIDE_F32 = 20; // 80 bytes

export class SceneBuilder {
  boxes: Box[] = [];
  materials: Material[] = [];
  lights: Light[] = [];

  material(
    albedo: Vec3,
    roughness: number,
    metallic = 0,
    emissive: Vec3 = v3(0, 0, 0),
  ): number {
    this.materials.push({ albedo, roughness, metallic, emissive });
    return this.materials.length - 1;
  }

  box(
    center: Vec3,
    half: Vec3,
    material: number,
    rot: [Vec3, Vec3, Vec3] = IDENTITY3,
    flags = 0,
  ): Box {
    const b: Box = { center, half, rot, material, flags };
    this.boxes.push(b);
    return b;
  }

  light(l: Light): Light {
    this.lights.push(l);
    return l;
  }

  /**
   * An emissive box that also registers as a sampleable light.
   *
   * `radiance` is how bright the fixture looks to the camera. `intensity` is how
   * much it actually lights the room, in the same inverse-square units as the
   * flashlight. These are deliberately independent.
   *
   * Deriving intensity from the emitter's area (radiance * r^2) is physically
   * correct and visually useless here: an exit sign is ~12cm across, so r^2
   * lands it near 0.01 against a flashlight at 240, and it contributes nothing
   * you can see. Real practicals are dim but not invisible, and the fixture
   * being small should not mean the room stays black.
   */
  areaLight(
    center: Vec3,
    half: Vec3,
    color: Vec3,
    radiance: number,
    intensity: number,
    rot = IDENTITY3,
  ): number {
    const mat = this.material(v3(0, 0, 0), 1, 0, {
      x: color.x * radiance,
      y: color.y * radiance,
      z: color.z * radiance,
    });
    this.box(center, half, mat, rot);
    // NEE samples the emitting box itself, not a sphere around it.
    //
    // This used to derive a sphere radius from the two largest half-extents,
    // which on a thin fixture protrudes straight through its own housing: the
    // stray samples shadow-test against the metal or the ceiling and come back
    // black. Measured by review at ~29% of fluorescent samples and ~44% of
    // rack-LED samples wasted — a shadow ray spent per sample, for nothing, and
    // ReSTIR then reuses the resulting noise.
    //
    // World extents rather than local: every emitter in this level is a slab
    // rotated about Y, so the thin axis is preserved exactly. A slab tilted off
    // Y would get a conservative box and could still stray slightly.
    const b = boxBounds({ center, half, rot, material: mat });
    const world = v3(
      (b.max.x - b.min.x) * 0.5,
      (b.max.y - b.min.y) * 0.5,
      (b.max.z - b.min.z) * 0.5,
    );
    // radius stays as the penumbra hint for anything that ignores halfExtents.
    const e = [half.x, half.y, half.z].sort((a, b2) => b2 - a);
    this.light({
      pos: center,
      kind: LIGHT_SPHERE,
      dir: v3(0, -1, 0),
      radius: Math.max(0.06, (e[0] + e[1]) * 0.5),
      color,
      intensity,
      cosInner: -1,
      cosOuter: -1,
      halfExtents: world,
    });
    return mat;
  }
}

export interface PackedScene {
  boxes: Float32Array;
  materials: Float32Array;
  lights: Float32Array;
  boxCount: number;
  materialCount: number;
  lightCount: number;
}

export function packMaterials(materials: Material[]): Float32Array<ArrayBuffer> {
  const out = new Float32Array(Math.max(1, materials.length) * MATERIAL_STRIDE_F32);
  materials.forEach((m, i) => {
    const o = i * MATERIAL_STRIDE_F32;
    out[o + 0] = m.albedo.x;
    out[o + 1] = m.albedo.y;
    out[o + 2] = m.albedo.z;
    out[o + 3] = m.roughness;
    out[o + 4] = m.emissive.x;
    out[o + 5] = m.emissive.y;
    out[o + 6] = m.emissive.z;
    out[o + 7] = m.metallic;
  });
  return out;
}

export function packLights(lights: Light[]): Float32Array<ArrayBuffer> {
  const out = new Float32Array(Math.max(1, lights.length) * LIGHT_STRIDE_F32);
  lights.forEach((l, i) => {
    const o = i * LIGHT_STRIDE_F32;
    out[o + 0] = l.pos.x;
    out[o + 1] = l.pos.y;
    out[o + 2] = l.pos.z;
    new Uint32Array(out.buffer, (o + 3) * 4, 1)[0] = l.kind;
    out[o + 4] = l.dir.x;
    out[o + 5] = l.dir.y;
    out[o + 6] = l.dir.z;
    out[o + 7] = l.radius;
    out[o + 8] = l.color.x;
    out[o + 9] = l.color.y;
    out[o + 10] = l.color.z;
    out[o + 11] = l.intensity;
    out[o + 12] = l.cosInner;
    out[o + 13] = l.cosOuter;
    out[o + 16] = l.halfExtents ? l.halfExtents.x : 0;
    out[o + 17] = l.halfExtents ? l.halfExtents.y : 0;
    out[o + 18] = l.halfExtents ? l.halfExtents.z : 0;
  });
  return out;
}

export const FLAG_EMISSIVE = 1;
/** Invisible to camera rays; still occludes and bounces light. */
export const FLAG_NO_CAMERA = 2;
/**
 * Rotation is identity, so the intersection test can skip the change of basis.
 * 43% of this level's boxes qualify (walls, floor, ceiling, carpet slabs).
 *
 * Kept because it is free and correct, but be aware it measures at only ~0.4%
 * (16.68 -> 16.61 ms), which is inside run-to-run noise. The intersection test
 * is not where this renderer spends its time.
 */
export const FLAG_AXIS_ALIGNED = 4;

function isIdentityRotation(rot: [Vec3, Vec3, Vec3]): boolean {
  const e = 1e-6;
  return (
    Math.abs(rot[0].x - 1) < e && Math.abs(rot[0].y) < e && Math.abs(rot[0].z) < e &&
    Math.abs(rot[1].x) < e && Math.abs(rot[1].y - 1) < e && Math.abs(rot[1].z) < e &&
    Math.abs(rot[2].x) < e && Math.abs(rot[2].y) < e && Math.abs(rot[2].z - 1) < e
  );
}

export function boxFlags(b: Box, m: Material | undefined): number {
  let f = b.flags ?? 0;
  if (m) {
    const e = m.emissive;
    if (e.x > 0 || e.y > 0 || e.z > 0) f |= FLAG_EMISSIVE;
  }
  if (isIdentityRotation(b.rot)) f |= FLAG_AXIS_ALIGNED;
  return f;
}

/** Packs boxes in BVH primitive order so leaves reference a contiguous range. */
export function packBoxes(
  boxes: Box[],
  order: Uint32Array,
  materials: Material[],
): Float32Array<ArrayBuffer> {
  const out = new Float32Array(Math.max(1, boxes.length) * BOX_STRIDE_F32);
  const u32 = new Uint32Array(out.buffer);
  for (let i = 0; i < order.length; i++) {
    const b = boxes[order[i]];
    const o = i * BOX_STRIDE_F32;
    out[o + 0] = b.center.x;
    out[o + 1] = b.center.y;
    out[o + 2] = b.center.z;
    u32[o + 3] = b.material;
    out[o + 4] = b.half.x;
    out[o + 5] = b.half.y;
    out[o + 6] = b.half.z;
    u32[o + 7] = boxFlags(b, materials[b.material]);
    for (let r = 0; r < 3; r++) {
      const row = b.rot[r];
      out[o + 8 + r * 4 + 0] = row.x;
      out[o + 8 + r * 4 + 1] = row.y;
      out[o + 8 + r * 4 + 2] = row.z;
      out[o + 8 + r * 4 + 3] = 0;
    }
  }
  return out;
}

/** World-space AABB of an oriented box. */
export function boxBounds(b: Box): { min: Vec3; max: Vec3 } {
  // Each world axis extent is the L1 norm of the corresponding rotation column
  // weighted by the half extents.
  const ex =
    Math.abs(b.rot[0].x) * b.half.x + Math.abs(b.rot[1].x) * b.half.y + Math.abs(b.rot[2].x) * b.half.z;
  const ey =
    Math.abs(b.rot[0].y) * b.half.x + Math.abs(b.rot[1].y) * b.half.y + Math.abs(b.rot[2].y) * b.half.z;
  const ez =
    Math.abs(b.rot[0].z) * b.half.x + Math.abs(b.rot[1].z) * b.half.y + Math.abs(b.rot[2].z) * b.half.z;
  return {
    min: v3(b.center.x - ex, b.center.y - ey, b.center.z - ez),
    max: v3(b.center.x + ex, b.center.y + ey, b.center.z + ez),
  };
}
