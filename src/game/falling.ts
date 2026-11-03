// WHERE AND WHEN A THING THAT IS LET GO COMES DOWN — the one answer of the simulation's for everything that falls and is heard or leaves something where it
// lands: a fragment of glass (shards.ts), a piece a broken thing throws (debris.ts thrown). No chunk, plate or particle is read: the answer is the same on a
// machine that draws none of them.
import type { Vec3 } from '../math/vec';

/** the statics' ray, as StaticCollision answers it */
export interface FallWorld { raycast(ro: Vec3, rd: Vec3, tmax: number): { t: number } | null }

/** Where and when something let go at `c` with velocity `v` reaches what is under it (statics only; the world's floor if nothing is), under `gravity` (m/s²).
 *  It gets `reachK` of the way a body in a vacuum would sideways in the time a fall to the floor takes, stopped short of the first thing in its way (a
 *  wall: nothing lands through one) — and what is under it is asked THERE (not under where it let go: a window's sill is not where its glass lands). */
export function fallOf(col: FallWorld, c: Vec3, v: Vec3, gravity: number, reachK: number): { t: number; pos: Vec3 } {
  const vy = -v[1];   // downward speed
  const tOf = (h: number) => (-vy + Math.sqrt(vy * vy + 2 * gravity * h)) / gravity;
  let reach = Math.hypot(v[0], v[2]) * tOf(Math.max(0.02, c[1])) * reachK; const d: Vec3 = reach > 1e-4 ? [v[0] * reachK * tOf(Math.max(0.02, c[1])) / reach, 0, v[2] * reachK * tOf(Math.max(0.02, c[1])) / reach] : [0, 0, 0];
  if (reach > 0.05) { const w = col.raycast([c[0], c[1], c[2]], d, reach + 0.05); if (w) reach = Math.max(0, w.t - 0.05); }
  const x = c[0] + d[0] * reach, z = c[2] + d[2] * reach;
  const hit = col.raycast([x, c[1], z], [0, -1, 0], c[1] + 0.5);
  const h = Math.max(0.02, hit ? hit.t : c[1]);
  return { t: tOf(h), pos: [x, c[1] - h, z] };
}

/** how many chords a thrown thing's arc is followed in */
const CHORDS = 6;
/** THE SAME QUESTION ASKED ALONG THE ARC, for a thing THROWN (a piece of something that broke, at metres a second: it clears a desk it would meet in a
 *  straight line at the height it left from, and meets a wall higher up than it started). The flight to the floor's level is walked in CHORDS straight
 *  chords, each asked of the statics; where one is cut the thing stops going sideways a hand short of what it met and drops from there onto what is under
 *  it. → when and where it comes to lie. (fallOf above is a fragment of glass's answer and stays as it was: one ray at the height it let go from.) */
export function flightOf(col: FallWorld, c: Vec3, v: Vec3, gravity: number): { t: number; pos: Vec3 } {
  const up = v[1], T = (up + Math.sqrt(up * up + 2 * gravity * Math.max(0.02, c[1]))) / gravity;   // the time to the floor's level, nothing in the way
  let px = c[0], py = c[1], pz = c[2], at = 0;
  for (let k = 1; k <= CHORDS; k++) {
    const tk = T * k / CHORDS, qx = c[0] + v[0] * tk, qy = Math.max(0.02, c[1] + up * tk - 0.5 * gravity * tk * tk), qz = c[2] + v[2] * tk;
    const dx = qx - px, dy = qy - py, dz = qz - pz, len = Math.sqrt(dx * dx + dy * dy + dz * dz);
    if (len > 1e-6) {
      const hit = col.raycast([px, py, pz], [dx / len, dy / len, dz / len], len + 0.05);
      if (hit) {   // met something: it stops there and drops (from rest: what it had left went into what it met)
        const s = Math.max(0, hit.t - 0.05) / len, x = px + dx * s, y = py + dy * s, z = pz + dz * s, under = col.raycast([x, y, z], [0, -1, 0], y + 0.5), h = Math.max(0, under ? under.t : y);
        return { t: at + (tk - at) * Math.min(1, hit.t / len) + Math.sqrt(2 * h / gravity), pos: [x, y - h, z] };
      }
    }
    px = qx; py = qy; pz = qz; at = tk;
  }
  return { t: T, pos: [px, 0, pz] };
}
