// THE LOOK PROTOTYPE's LOOSE SNOW IN THE AIR, the frame's side (the look panel's "Blowing snow" and "Snow puffs" rows, render/frame.ts lookConfig.veil / .puffs /
// .snow; the stuff itself is render/powderField.ts, carried by the game's own air; drawn by shaders/composite.wgsl lookPowder on the pixel under it). The
// maintainer, 10-28, of the first try (a pattern slid along one wind vector): "doesn't look like it reacts to walls, and it seems to imply a wind velocity that
// doesn't match the wind velocity for the falling snow". So the frame says nothing of any wind now: only WHERE the powder's field lies over the world and where
// the sun is. Picture only: nothing in the game reads it. The check: tools/qa/lookveil-check.ts.

/** this frame's two fields, or null (no field: the frame's fields stay zeros and the shader skips every statement of it). `world` = the floor plan the powder's
 *  field covers (m: its corner and its size); `amount` 0‥1 = how much of the powder is drawn; the sun (or the moon) = the first directional light that is on.
 *  veilA = (1 ∕ the plan's width, 1 ∕ its depth, amount, what that sun puts on level ground as luminance), veilB = (the plan's corner x z, the way to the sun along
 *  the ground per metre of height x z; 0 = no sun: no lit side, no shade under the powder). */
export function lookVeilFrame(world: { x: number; z: number; w: number; d: number } | null, amount: number, lights: readonly { kind: number; enabled: boolean; transient?: boolean; intensity: number; dir: ArrayLike<number>; color: ArrayLike<number> }[]): [[number, number, number, number], [number, number, number, number]] | null {
  if (!world || !(world.w > 0) || !(world.d > 0)) return null;
  const k = Math.min(1, amount); if (!(k > 0)) return null;
  const sun = lights.find(l => l.kind === 2 && l.enabled && !l.transient && l.intensity > 0);
  const h = sun && sun.dir[1] < -0.05 ? Math.max(-sun.dir[1], 0.25) : 0;
  // (w: what that sun puts on level ground, as luminance: the shader takes the powder's shade and its sunny side out of that share of a pixel's direct light
  // only, so a lamp's pool and a fire's are not shaded as if the moon had lit them)
  const sunE = h ? sun!.intensity * -sun!.dir[1] * (0.2126 * sun!.color[0] + 0.7152 * sun!.color[1] + 0.0722 * sun!.color[2]) : 0;
  return [[1 / world.w, 1 / world.d, k, sunE], [world.x, world.z, h ? -sun!.dir[0] / h : 0, h ? -sun!.dir[2] / h : 0]];
}
