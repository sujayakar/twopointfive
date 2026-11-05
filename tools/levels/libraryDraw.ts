// A PICTURE OF A LIBRARY PIECE, drawn from its boxes (tools/levels/library.ts): a small software rasteriser, because the catalogue is written where there is no
// GPU and has to be the same file on every machine. It is a DRAWING OF THE BOXES, not the game's picture: flat paint, the markings on the faces, one fixed light,
// no shadow, no material maps. What a piece looks like in the game is the showroom's business (levels/gwot-yard.json on the page).
// The view is the play camera's kind of view: from above and a little to one side, parallel (no perspective), so a face maps to the screen by one 2 × 2 matrix
// and a pixel finds its place on the face, and its marking cell, by the inverse.
import type { Vec3 } from '../../src/math/vec';
import { BoxFlag } from '../../src/scene/boxes';
import { FACE_AXES, MARK_FACES, MARKING_TEXEL, texelOf, resolveAlbedo, type MarkingDef } from '../../src/scene/markings';

/** a box as the drawing needs it (a static of the level, or a part of a loose thing put where it stands) */
export interface DrawBox { c: Vec3; h: Vec3; yaw: number; albedo: Vec3; flags: number; emissive?: Vec3; marks?: MarkingDef[] }

const TURN = 32 * Math.PI / 180, TILT = 52 * Math.PI / 180;       // the camera: turned off the piece's own axes, looking down
const cT = Math.cos(TURN), sT = Math.sin(TURN), cP = Math.cos(TILT), sP = Math.sin(TILT);
/** world → (screen x, screen y down, nearness) */
const project = (p: Vec3): Vec3 => { const xr = p[0] * cT - p[2] * sT, zr = p[0] * sT + p[2] * cT; return [xr, zr * sP - p[1] * cP, zr * cP + p[1] * sP]; };
const TO_EYE: Vec3 = [sT * cP, sP, cT * cP];
const LIGHT: Vec3 = (() => { const l: Vec3 = [-0.45, 0.8, 0.4], n = Math.hypot(...l); return [l[0] / n, l[1] / n, l[2] / n]; })();
const BACKDROP: Vec3 = [0.2, 0.17, 0.13];                         // packed earth, lifted: white plastic and black rubber both stand off it
const EXPOSURE = 1.2;
const toByte = (lin: number) => Math.round(255 * Math.pow(Math.min(1, Math.max(0, lin * EXPOSURE)), 1 / 2.2));

/** draw the boxes into w × h pixels (drawn at twice that and averaged down); returns rgba (tools/qa/png.ts encodePng makes the file) */
export function drawBoxes(boxes: readonly DrawBox[], w = 240, h = 180, texel = MARKING_TEXEL): Uint8Array {
  const W = w * 2, H = h * 2, col = new Float32Array(W * H * 3), near = new Float32Array(W * H).fill(-Infinity);
  for (let i = 0; i < W * H; i++) { col[i * 3] = BACKDROP[0]; col[i * 3 + 1] = BACKDROP[1]; col[i * 3 + 2] = BACKDROP[2]; }
  const shown = boxes.filter(b => !(b.flags & BoxFlag.NoPrimary));
  if (!shown.length) return downsample(col, W, H);
  // every visible face as a centre and two half edges, in the world
  type Face = { b: DrawBox; f: number; c: Vec3; u: Vec3; v: Vec3; hu: number; hv: number; lit: number };
  const faces: Face[] = [];
  for (const b of shown) {
    const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), toWorld = (l: Vec3): Vec3 => [cs * l[0] + sn * l[2], l[1], -sn * l[0] + cs * l[2]];
    FACE_AXES.forEach((ax, f) => {
      const n = toWorld(ax.n); if (n[0] * TO_EYE[0] + n[1] * TO_EYE[1] + n[2] * TO_EYE[2] <= 1e-6) return;
      const pick = (a: Vec3) => Math.abs(a[0]) * b.h[0] + Math.abs(a[1]) * b.h[1] + Math.abs(a[2]) * b.h[2], hn = pick(ax.n);
      faces.push({ b, f, c: [b.c[0] + n[0] * hn, b.c[1] + n[1] * hn, b.c[2] + n[2] * hn], u: toWorld(ax.u), v: toWorld(ax.v), hu: pick(ax.u), hv: pick(ax.v),
        lit: 0.5 + 0.5 * Math.max(0, n[0] * LIGHT[0] + n[1] * LIGHT[1] + n[2] * LIGHT[2]) });
    });
  }
  // fit the lot in the frame
  let x0 = Infinity, x1 = -Infinity, y0 = Infinity, y1 = -Infinity;
  for (const F of faces) for (const su of [-1, 1]) for (const sv of [-1, 1]) {
    const p = project([F.c[0] + su * F.u[0] * F.hu + sv * F.v[0] * F.hv, F.c[1] + su * F.u[1] * F.hu + sv * F.v[1] * F.hv, F.c[2] + su * F.u[2] * F.hu + sv * F.v[2] * F.hv]);
    x0 = Math.min(x0, p[0]); x1 = Math.max(x1, p[0]); y0 = Math.min(y0, p[1]); y1 = Math.max(y1, p[1]);
  }
  const scale = Math.min(W * 0.88 / Math.max(x1 - x0, 0.3), H * 0.86 / Math.max(y1 - y0, 0.3)), ox = W / 2 - (x0 + x1) / 2 * scale, oy = H / 2 - (y0 + y1) / 2 * scale;
  for (const F of faces) {
    const pc = project(F.c), pu = project([F.u[0] * F.hu, F.u[1] * F.hu, F.u[2] * F.hu]), pv = project([F.v[0] * F.hv, F.v[1] * F.hv, F.v[2] * F.hv]);
    const cx = pc[0] * scale + ox, cy = pc[1] * scale + oy, ux = pu[0] * scale, uy = pu[1] * scale, vx = pv[0] * scale, vy = pv[1] * scale, det = ux * vy - uy * vx;
    if (Math.abs(det) < 1e-9) continue;
    const ex = Math.abs(ux) + Math.abs(vx), ey = Math.abs(uy) + Math.abs(vy);
    const marks = (F.b.marks ?? []).filter(m => MARK_FACES.indexOf(m.face) === F.f);
    const glow = F.b.emissive && (F.b.emissive[0] + F.b.emissive[1] + F.b.emissive[2] > 0) ? F.b.emissive : null, gmax = glow ? Math.max(...glow) : 1;
    for (let py = Math.max(0, Math.floor(cy - ey)); py <= Math.min(H - 1, Math.ceil(cy + ey)); py++) for (let px = Math.max(0, Math.floor(cx - ex)); px <= Math.min(W - 1, Math.ceil(cx + ex)); px++) {
      const dx = px + 0.5 - cx, dy = py + 0.5 - cy, a = (dx * vy - dy * vx) / det, bb = (ux * dy - uy * dx) / det;
      if (a < -1 || a > 1 || bb < -1 || bb > 1) continue;
      const d = pc[2] + a * pu[2] + bb * pv[2], i = py * W + px; if (d <= near[i]) continue; near[i] = d;
      let paint: Vec3 = F.b.albedo;
      for (const m of marks) {
        const t = texelOf(m, texel), ci = Math.floor(a * F.hu / t - m.at[0]), cj = Math.floor(bb * F.hv / t - m.at[1]);
        if (ci < 0 || cj < 0 || ci >= m.size[0] || cj >= m.size[1]) continue;
        const ch = m.cells[m.size[1] - 1 - cj]?.charCodeAt(ci) ?? 46; if (ch === 46) continue;
        paint = resolveAlbedo(m.pal[ch - 97], F.b.albedo);
      }
      const k = glow ? 1 / EXPOSURE : F.lit, src = glow ? [glow[0] / gmax, glow[1] / gmax, glow[2] / gmax] : paint;
      col[i * 3] = src[0] * k; col[i * 3 + 1] = src[1] * k; col[i * 3 + 2] = src[2] * k;
    }
  }
  return downsample(col, W, H);
}
function downsample(col: Float32Array, W: number, H: number): Uint8Array {
  const w = W / 2, h = H / 2, out = new Uint8Array(w * h * 4);
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) for (let c = 0; c < 4; c++) {
    if (c === 3) { out[(y * w + x) * 4 + 3] = 255; continue; }
    const at = (yy: number, xx: number) => col[(yy * W + xx) * 3 + c];
    out[(y * w + x) * 4 + c] = toByte((at(2 * y, 2 * x) + at(2 * y, 2 * x + 1) + at(2 * y + 1, 2 * x) + at(2 * y + 1, 2 * x + 1)) / 4);
  }
  return out;
}
