import { Box, FLAG_EMISSIVE, Material, boxFlags } from "./scene";

// ---------------------------------------------------------------------------
// Radiosity patches.
//
// Diffuse-only materials and static box geometry make classical radiosity
// exact here — not an approximation being tolerated but the analytically
// right answer. The static faces are diced into patches of roughly uniform
// area; a one-time GPU bake computes the patch-to-patch form factors with
// visibility, plus per-patch visibility of every static light and of the sky
// dome. Each frame the renderer injects direct light into the patches (from
// the baked tables and the torch depth maps — zero rays), runs a couple of
// warm-started Jacobi iterations, and the trace pass reads the gathered
// irradiance back per pixel. The result is noise-free, infinite-bounce
// indirect light that follows the flashlight around the room.
//
// Only static, non-emissive boxes get patches. Emissive boxes are light
// housings: their albedo is zero, so they could never re-radiate, and their
// lens boxes sit on top of the very lights being injected. Dynamic geometry
// (characters, doors) neither carries patches nor occludes the baked
// transfer — bodies are small, and the error is the price of a static bake.
// ---------------------------------------------------------------------------

/** f32 lanes per patch: [pos, area], [normal, matIdx], [tanU, halfU], [tanV, halfV]. */
export const PATCH_STRIDE_F32 = 16;
/** u32 lanes per face-table entry: base, gridW, gridH, pad. */
export const FACE_STRIDE_U32 = 4;
/** Sentinel base for faces with no patches. */
export const FACE_NONE = 0xffffffff;

/**
 * Hard cap on patch count: the dense form-factor matrix is N^2 * 4 bytes,
 * and buildPatchCdf in radiosity.wgsl scans it in one 256-thread workgroup
 * of <= 16-element chunks (256 * 16 = 4096).
 */
const MAX_PATCHES = 4096;
/** Faces smaller than this are furniture trim — not worth a matrix row. */
const MIN_FACE_AREA = 0.15;
/**
 * Up-facing surfaces get patches this much smaller: floors are where the
 * beams land and where the bounce is actually read, walls and ceilings
 * mostly re-radiate at a distance.
 */
const UP_REFINE = 0.65;

export interface RadiosityData {
  patchCount: number;
  /** Metres; chosen adaptively so patchCount lands under MAX_PATCHES. */
  patchSize: number;
  patches: Float32Array<ArrayBuffer>;
  /** Indexed by (bvhOrderBoxIndex * 6 + face). */
  faceTable: Uint32Array<ArrayBuffer>;
}

interface FaceAxes {
  /** Local unit axis index (0/1/2) of the face normal, and its sign. */
  axis: number;
  sign: number;
  /** Local axis indices spanning the face. */
  uAxis: number;
  vAxis: number;
}

/**
 * Face conventions, shared with the WGSL lookup in pathtrace.wgsl. Face ids:
 * 0:+X 1:-X 2:+Y 3:-Y 4:+Z 5:-Z in the box's local frame; U/V span axes are
 * fixed per face so CPU bake positions and GPU lookups agree exactly.
 */
const FACES: FaceAxes[] = [
  { axis: 0, sign: 1, uAxis: 2, vAxis: 1 },
  { axis: 0, sign: -1, uAxis: 2, vAxis: 1 },
  { axis: 1, sign: 1, uAxis: 0, vAxis: 2 },
  { axis: 1, sign: -1, uAxis: 0, vAxis: 2 },
  { axis: 2, sign: 1, uAxis: 0, vAxis: 1 },
  { axis: 2, sign: -1, uAxis: 0, vAxis: 1 },
];

function half(b: Box, axis: number): number {
  return axis === 0 ? b.half.x : axis === 1 ? b.half.y : b.half.z;
}

/** Local axis direction in world space. Rot rows are the local axes. */
function localAxisWorld(b: Box, axis: number): [number, number, number] {
  const r = b.rot[axis];
  return [r.x, r.y, r.z];
}

/** Face centre and outward normal in world space. */
function faceFrame(b: Box, f: FaceAxes): { c: [number, number, number]; n: [number, number, number] } {
  const nAxis = localAxisWorld(b, f.axis);
  const hn = half(b, f.axis) * f.sign;
  return {
    c: [b.center.x + hn * nAxis[0], b.center.y + hn * nAxis[1], b.center.z + hn * nAxis[2]],
    n: [f.sign * nAxis[0], f.sign * nAxis[1], f.sign * nAxis[2]],
  };
}

function insideAnyBox(
  boxes: Box[], selfIdx: number, x: number, y: number, z: number,
): boolean {
  for (let i = 0; i < boxes.length; i++) {
    if (i === selfIdx) continue;
    const b = boxes[i];
    const dx = x - b.center.x, dy = y - b.center.y, dz = z - b.center.z;
    let inside = true;
    for (let a = 0; a < 3 && inside; a++) {
      const r = b.rot[a];
      const l = dx * r.x + dy * r.y + dz * r.z;
      const h = a === 0 ? b.half.x : a === 1 ? b.half.y : b.half.z;
      inside = Math.abs(l) < h - 0.005;
    }
    if (inside) return true;
  }
  return false;
}

interface FaceJob {
  boxOrderIdx: number;
  face: number;
  sizeFactor: number;
  area: number;
}

export function buildPatches(
  boxes: Box[],
  order: Uint32Array,
  materials: Material[],
): RadiosityData {
  // Decide once which faces deserve patches; sizing and building share it.
  // Culled: emissive boxes (light housings — zero albedo, they cannot
  // re-radiate), faces buried inside another box (a wall's back pressed
  // against the next wall: five probe points all land inside a neighbour),
  // and ground-level undersides (visible to nothing, lit by nothing).
  const jobs: FaceJob[] = [];
  let weightedArea = 0;
  for (let i = 0; i < order.length; i++) {
    const bi = order[i];
    const b = boxes[bi];
    if ((boxFlags(b, materials[b.material]) & FLAG_EMISSIVE) !== 0) continue;
    for (let f = 0; f < 6; f++) {
      const fa = FACES[f];
      const hu = half(b, fa.uAxis);
      const hv = half(b, fa.vAxis);
      const area = 4 * hu * hv;
      if (area < MIN_FACE_AREA) continue;

      const { c, n } = faceFrame(b, fa);
      if (n[1] < -0.5 && c[1] < 0.02) continue;

      const uAxis = localAxisWorld(b, fa.uAxis);
      const vAxis = localAxisWorld(b, fa.vAxis);
      let buried = true;
      for (let s = 0; s < 5 && buried; s++) {
        const su = s === 0 ? 0 : (s < 3 ? -0.5 : 0.5) * hu;
        const sv = s === 0 ? 0 : (s % 2 === 1 ? -0.5 : 0.5) * hv;
        buried = insideAnyBox(
          boxes, bi,
          c[0] + su * uAxis[0] + sv * vAxis[0] + n[0] * 0.06,
          c[1] + su * uAxis[1] + sv * vAxis[1] + n[1] * 0.06,
          c[2] + su * uAxis[2] + sv * vAxis[2] + n[2] * 0.06,
        );
      }
      if (buried) continue;

      const sizeFactor = n[1] > 0.5 ? UP_REFINE : 1;
      jobs.push({ boxOrderIdx: i, face: f, sizeFactor, area });
      weightedArea += area / (sizeFactor * sizeFactor);
    }
  }

  let patchSize = Math.max(0.4, Math.sqrt(weightedArea / (MAX_PATCHES * 0.85)));
  for (let attempt = 0; attempt < 6; attempt++) {
    const built = tryBuild(boxes, order, jobs, patchSize);
    if (built) return built;
    patchSize *= 1.18;
  }
  throw new Error("radiosity: patch budget not reachable");
}

function tryBuild(
  boxes: Box[],
  order: Uint32Array,
  jobs: FaceJob[],
  patchSize: number,
): RadiosityData | null {
  const faceTable = new Uint32Array(order.length * 6 * FACE_STRIDE_U32).fill(FACE_NONE);
  const patches: number[] = [];
  let count = 0;

  for (const job of jobs) {
    const b = boxes[order[job.boxOrderIdx]];
    const fa = FACES[job.face];
    const hu = half(b, fa.uAxis);
    const hv = half(b, fa.vAxis);
    const size = patchSize * job.sizeFactor;

    const gridW = Math.max(1, Math.round((2 * hu) / size));
    const gridH = Math.max(1, Math.round((2 * hv) / size));
    if (count + gridW * gridH > MAX_PATCHES) return null;

    const e = (job.boxOrderIdx * 6 + job.face) * FACE_STRIDE_U32;
    faceTable[e + 0] = count;
    faceTable[e + 1] = gridW;
    faceTable[e + 2] = gridH;
    faceTable[e + 3] = 0;

    const nAxis = localAxisWorld(b, fa.axis);
    const uAxis = localAxisWorld(b, fa.uAxis);
    const vAxis = localAxisWorld(b, fa.vAxis);
    const cellU = (2 * hu) / gridW;
    const cellV = (2 * hv) / gridH;
    const hn = half(b, fa.axis);
    const cellArea = cellU * cellV;

    for (let gv = 0; gv < gridH; gv++) {
      for (let gu = 0; gu < gridW; gu++) {
        const lu = -hu + (gu + 0.5) * cellU;
        const lv = -hv + (gv + 0.5) * cellV;
        const ln = fa.sign * hn;
        const px = b.center.x + lu * uAxis[0] + lv * vAxis[0] + ln * nAxis[0];
        const py = b.center.y + lu * uAxis[1] + lv * vAxis[1] + ln * nAxis[1];
        const pz = b.center.z + lu * uAxis[2] + lv * vAxis[2] + ln * nAxis[2];
        patches.push(
          px, py, pz, cellArea,
          fa.sign * nAxis[0], fa.sign * nAxis[1], fa.sign * nAxis[2], b.material,
          uAxis[0], uAxis[1], uAxis[2], cellU * 0.5,
          vAxis[0], vAxis[1], vAxis[2], cellV * 0.5,
        );
        count++;
      }
    }
  }

  return {
    patchCount: count,
    patchSize,
    patches: new Float32Array(patches),
    faceTable,
  };
}
