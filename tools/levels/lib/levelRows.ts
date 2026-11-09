// THE LEVEL FORMAT'S PLAIN RECORDS: a box, a door, a light, as a level file and a kit view's set write them (the file's whole shape: tools/levels/lib/levelData.ts, which takes them from here).
import type { Quat, Vec3 } from '../../../src/math/vec';
import type { MarkingDef } from '../../../src/scene/markings';
import type { ShapeForm } from '../../../src/scene/shapes';

/** Box flags by NAME as they appear in a level file. Only the authored subset: Dynamic / Emissive / AreaLit / Rot are set by the runtime (props, upload,
 *  rust/game_rules/src/tick/game area lights, Box.rot presence) and SoundSeal is a CPU-only door-leaf state — none of them is ever authored, and the validator rejects them. */
export const AUTHORED_FLAGS = ['NoPrimary', 'NoShadow', 'Cutaway', 'Gloss', 'Glass'] as const;   // (Glass: experiment E3 — a pane; boxes.ts BoxFlag.Glass)
export type FlagName = typeof AUTHORED_FLAGS[number];
/** One static box (scene/boxes.ts Box, minus everything the runtime owns: owner, the Emissive / AreaLit / Rot / Dynamic bits). */
export interface BoxData {
  id: string;
  /** FloorData.id; absent = floors[0] */
  floor?: string;
  /** editor grouping only (the generator's section: 'lobby', 'walls.interior', 'doorframe.server', …) — no runtime meaning */
  group?: string;
  /** Box.name: a tag the game may look up ('breaker_panel') */
  name?: string;
  c: Vec3; h: Vec3;
  /** rotation about +Y, radians; absent = 0 */
  yaw?: number;
  /** full orientation (unit quaternion x y z w) for the few tilted things (the bike's tubes); absent = yaw only */
  rot?: Quat;
  /** A ROUND THING (scene/round.ts): to every ray, eye, foot and to the picture this box is the cylinder about its own y, radius h[0] (which equals h[2]), half
   *  height h[1]. Not a shape: a shape is cut from a box that still walks as the plain box. Says no yaw, rot, flags or shape. */
  round?: true;
  /** exactly one of material (a key of LevelData.materials) / albedo (inline linear rgb) */
  material?: string; albedo?: Vec3;
  /** emitted radiance (linear HDR); absent = none. Fixtures that switch (panels, screens, signs) are ALSO listed in `switchables` with their lit value. */
  emissive?: Vec3;
  /** the finish (see MaterialData): overrides the material's / gives an inline-albedo one-off one; absent = the material's, else matte / dielectric */
  rough?: number; metal?: boolean;
  /** Box.stuff: what the box is made of, where its material's row does not say it (scene/stuff.ts: a material of the look, 'plain', 'waits', 'glass' or
   *  'rubber'); absent = what its material's row says, and plain for a box with an inline albedo. Nothing is read off the colour. The picture's alone (the
   *  look, and what a round's hit looks like: rust/game_rules/src/breakage/particle_fx.rs HITSTUFF): no rule of the game reads it. */
  stuff?: string;
  /** Box.does: what the box does, where its material's row does not say it (scene/materials.ts); absent = what its material's row says, and the default
   *  row for a box with an inline albedo. 'unnamed' (the default row) is written only over a row that says something else. */
  does?: string;
  flags?: FlagName[];
  /** Box.cutHeight (viewer stage back wall only); absent = never cut */
  cutHeight?: number;
  /** MARKINGS on the box's faces (scene/markings.ts MarkingDef: face, kind, rect from the face centre, res, palette a/b/c, emit, rough, p, id) — the pixel-art
   *  signifiers (key caps, tape, seams, the vending window, text bars, LEDs, glyphs); ≤ MAX_MARKS_PER_BOX; absent = none */
  markings?: MarkingDef[];
  /** A SHAPE (scene/shapes.ts — read its head): how this box stands when shapes are on, said in the box's OWN frame so that it moves, turns and copies with
   *  the box. `round` 1 | 3 = a twelve-sided prism about the box's own x or z; `arc` { axis, rise } = its top a round about its own x or z; `cut` = planes
   *  [nx, ny, nz, d], each keeping n · p ≤ d (8 at most, 5 beside an arc, whose own facets give way to them); `old` = not a fresh break (the picture's word: the cut faces take the box's own
   *  paint and lying snow); `with` = pieces of trim that are there only when shapes are on, drawn and nothing else. The box keeps its centre and half size: a shape is cut FROM the plain
   *  box. With shapes off the box is the plain box above and nothing here is read. Not on a box with `rot`; no form on a box with `markings` or the Glass
   *  flag (their readers — the mark painter, the pane engine — read the whole box). */
  shape?: ShapeData;
}

/** one piece of a shape (BoxData.shape.with): TRIM, drawn and nothing else — `off` (its centre from the box's, in the box's own frame), half size, yaw from
 *  the box's own, paint (exactly one of material / albedo), finish — and a form. It says no flags: a piece always stands NoShadow */
export interface ShapePieceData extends ShapeForm { off: Vec3; h: Vec3; yaw?: number; /** at most one of material / albedo; neither = the box's own paint, finish and word */ material?: string; albedo?: Vec3; rough?: number; metal?: boolean; /** as BoxData.does */ does?: string; /** as BoxData.stuff: what the piece is made of where its paint's row does not say it */ stuff?: string }
export interface ShapeData extends ShapeForm { with?: ShapePieceData[] }
/** An analytic light (tools/levels/lib/level.ts LightDef, every field the renderer reads). `id` is the LightDef.name the game keys targets / panels on. */
export interface LightData {
  id: string;
  kind: 'point' | 'spot' | 'dir';
  pos: Vec3; dir: Vec3; color: Vec3;
  /** point/spot: candela-ish; dir: irradiance on a surface square to it */
  intensity: number; range: number; innerDeg: number; outerDeg: number;
  /** light-target group for the Lights panel / OCP / daylight ('exterior', 'manager', …) */
  group: string;
  enabled: boolean;
  /** haze scattering multiplier for beams */
  volumetric: number;
  /** emitter size (m) → penumbra; for 'dir' the angular radius (rad). absent = the renderer's default (0.05 / 0) */
  radius?: number;
  /** fed from the building breaker; absent = true (false: street lighting, moon, sun) */
  mains?: boolean;
  /** wide soft emitter → the direct pass's 'broad' denoise class */
  broad?: boolean;
  /** BoxData.id of the emissive glow card that represents the fixture (goes dark with the light) */
  fixture?: string;
}
/** A hinged leaf in a doorway gap (rust/game_rules/src/fittings/doors DoorDef minus the runtime albedo). The FRAME (jambs, head, stops) is ordinary boxes, group 'doorframe.<id>'. */
export interface DoorData { id: string; floor?: string; hinge: [number, number]; closedDir: number; width: number; angle?: number; exterior?: boolean; closer?: boolean; minAngle?: number; maxAngle?: number; locked?: boolean; /** which key turns it; absent = the floor key */ lock?: string; keySide?: 1 | -1; /** the side the leaf opens into, +1 = the CCW side of the closed leaf (rust/game_rules/src/fittings/doors DoorDef.swing); absent = dealt from the level: into the room it serves (rust/game_rules/src/fittings/doorswing.rs) */ swing?: 1 | -1; /** a steel fire door: a boot only rings off it, no pick opens it (rust/game_rules/src/fittings/doors DoorDef.steel — the QRF's west stair door) */ steel?: boolean; }
