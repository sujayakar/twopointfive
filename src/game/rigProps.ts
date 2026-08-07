// Rig props: every number that places something ON the mannequin — the trifocal goggles on the head, the Five-seveN with its
// under-barrel rail light in the right hand, the guards' hand torch / weapon light — lives in RIG, and rigBoxes() / rigLightPose()
// turn it into the prop boxes and the rail light's pose. The character viewer (viewer.html → src/viewer.ts) drives the
// same functions with sliders bound to RIG and its "copy constants" button emits the `export const RIG` statement below verbatim
// (rigSource()), so tuning is: drag, copy, paste over the block, commit.
import { Vec3, v3, quat } from '../math/vec';
import { Box, BoxFlag, makeBox } from '../scene/boxes';
import { RtLight } from '../render/lights';
import { Character } from './character';

/** A stick-shaped part laid along a direction from an anchor point: it spans [l0, l1] metres along the direction; the box is
 *  `thick` (half width, sideways) × `tall` (half height). The box is fully oriented along the direction (Box.rot, see seg()). */
export interface Seg { l0: number; l1: number; thick: number; tall: number; adj?: Adj; }
/** A free adjustment on top of the structured numbers, Blender-style, in the node's PARENT frame (x = side, y = up, z = along —
 *  see nodeFrames below): `off` metres, `rot` degrees about those axes (applied x, then y, then z). Groups (pistol, goggles.on/off,
 *  torch, guardPistol) and parts (each Seg, the goggle housing / lens cluster) can carry one; absent = zero. This is what the
 *  viewer's gizmo edits. Props are traced boxes carrying a full 3D rotation (Box.rot), so groups turn rigidly. */
export interface Adj { off: Vec3; rot: Vec3; }

export interface Rig {
  goggles: {
    /** night vision ON: lens-cluster base point = head joint + f·fwd + up·[0,1,0] in the head frame chosen by followPitch (f = flattened head
     *  direction by default). The head joint sits at the top of the neck, so the eyes are ~10 cm up and ~10 cm forward of it… */
    on: { fwd: number; up: number };
    /** …and OFF the goggles are flipped up, parked on the forehead (~18 cm up) */
    off: { fwd: number; up: number };
    /** trifocal triangle around the base point: two lenses at ±lensSide sideways and lensDown, the third centred at lensUp */
    lensSide: number; lensDown: number; lensUp: number;
    /** one lens box: lensHalf across (square face) × lensThick along f (half extents) */
    lensHalf: number; lensThick: number;
    /** housing behind the lenses: centre sits `back` behind the base point along f */
    housing: { back: number; half: Vec3 };
    /** lens emission: down over the eyes they blaze, parked on the forehead they idle at a readable glow (screen glow + bloom only: props are
     *  raster-only boxes, no ray ever sees them, so this never lights the face) */
    glowOn: Vec3; glowOff: Vec3;
    /** free adjustments (viewer gizmo): the whole cluster per state, the housing, the lens triangle */
    onAdj?: Adj; offAdj?: Adj; housingAdj?: Adj; lensAdj?: Adj;
    /** 0: fwd/up/side are laid out in a yaw-only head frame (world up) — what the game shipped with; 1: they ride the skull's full rotation
     *  (rest→current delta, like headDir), so a pitched head — crouch, looking down the sights, a ragdolled body — keeps them on the face. The boxes
     *  themselves can only yaw either way. */
    followPitch: number;
  };
  /** the player's pistol, laid along char.gunDir from bones.handR (wrist) */
  pistol: {
    slide: Seg; suppressor: Seg;
    /** grip: a segment hung under the rear of the frame — its adj.rot[0] rakes it down-and-back from the bore axis (≈108°), adj.off puts its top under the slide */
    grip: Seg;
    /** rail light body: same axis, but its anchor is dropped `under` metres along char.gunUnder() (below the bore) */
    rail: Seg & { under: number };
    /** the rail light's lens (drawn lit only while the light is on) */
    lens: Seg & { glow: Vec3 };
    /** the spot light itself: pos = hand + gunDir·lightAhead + gunUnder·lightUnder, dir = gunDir (see rigLightPose) */
    lightUnder: number; lightAhead: number;
    adj?: Adj;   // whole pistol group (props + rail light) relative to the hand frame
  };
  /** guards on patrol carry a hand torch along their (AI-smoothed) beam direction; the spot light sits at its lens */
  torch: { body: Seg; lens: Seg; /** lens emissive = flashlight colour × lensGlow while lit */ lensGlow: number; /** light origin: hand + beamDir·lightAhead */ lightAhead: number; adj?: Adj };
  /** guards off patrol: drawn pistol (no can) with the weapon light glowing in the lens slot */
  guardPistol: { slide: Seg; grip: Seg; lens: Seg; lightAhead: number; adj?: Adj };
}

// ---- paste target: the viewer's "copy constants" button (rigSource()) emits exactly this statement, in this format ----
export const RIG: Rig = {
  goggles: {
    on: { fwd: 0.105, up: 0.1 },
    off: { fwd: 0.075, up: 0.185 },
    lensSide: 0.03,
    lensDown: -0.012,
    lensUp: 0.026,
    lensHalf: 0.013,
    lensThick: 0.008,
    housing: { back: 0.03, half: [0.055, 0.028, 0.03] },
    glowOn: [1.5, 14, 3],
    glowOff: [0.35, 3.2, 0.7],
    followPitch: 1,
    onAdj: { off: [-0.004, 0.025, 0.02], rot: [19.1, 0, 1.1] },
    offAdj: { off: [-0.006, 0.048, 0.032], rot: [-17.2, 0, 0.7] },
  },
  pistol: {
    slide: { l0: 0.02, l1: 0.21, thick: 0.016, tall: 0.026 },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017, adj: { off: [0, -0.024, 0.05], rot: [108, 0, 0] } },   // top under the slide behind the trigger, raked ~18° back from vertical
    suppressor: { l0: 0.21, l1: 0.36, thick: 0.019, tall: 0.019 },
    rail: { l0: 0.09, l1: 0.2, thick: 0.013, tall: 0.012, under: 0.034, adj: { off: [0, 0, 0.064], rot: [0, 0, 0] } },
    lens: { l0: 0.2, l1: 0.215, thick: 0.014, tall: 0.013, glow: [14, 14, 13] },
    lightUnder: 0.038,
    lightAhead: 0.2,
    adj: { off: [0.039, 0.08, 0.035], rot: [-1.3, 0, 0] },
  },
  torch: {
    body: { l0: -0.06, l1: 0.15, thick: 0.022, tall: 0.022 },
    lens: { l0: 0.15, l1: 0.18, thick: 0.026, tall: 0.026 },
    lensGlow: 40,
    lightAhead: 0.18,
    adj: { off: [0.076, 0.04, 0.092], rot: [0, 6.7, 0] },
  },
  guardPistol: {
    slide: { l0: 0.02, l1: 0.2, thick: 0.016, tall: 0.026, adj: { off: [0.028, -0.034, 0.073], rot: [11.2, 0, 0.5] } },
    grip: { l0: 0, l1: 0.095, thick: 0.014, tall: 0.017, adj: { off: [0.028, -0.06, 0.118], rot: [119, 0, 0.5] } },   // follows the tuned slide (its off/rot folded in); fine-tune in the viewer
    lens: { l0: 0.2, l1: 0.23, thick: 0.015, tall: 0.015, adj: { off: [0.029, -0.064, 0.023], rot: [11.8, 0, 0] } },
    lightAhead: 0.25,
    adj: { off: [0.031, 0.105, -0.05], rot: [-8.6, 0, 0] },
  },
};
// ---- end of paste target ----

/** albedos are not tunables (diffuse-only renderer: they just need to read as gunmetal / can / polymer / glass) */
const COL = {
  gun: [0.05, 0.05, 0.055] as Vec3, can: [0.09, 0.09, 0.095] as Vec3, torch: [0.16, 0.16, 0.17] as Vec3, lensBody: [0.3, 0.3, 0.3] as Vec3,
  gogglesLens: [0.02, 0.05, 0.03] as Vec3, gogglesHousing: [0.04, 0.045, 0.04] as Vec3,
};
const PROP_FLAGS = BoxFlag.Dynamic;                      // real geometry: the gun / torch body / goggle housing shadow and occlude like anything else (owner = the character, and shadow rays skip whatever the LIGHT is mounted on, so a weapon light is never blocked by its own gun)
const GLOW_FLAGS = BoxFlag.Dynamic | BoxFlag.NoShadow;   // lit lenses stay raster-only: centimetre emitters would only be firefly bait for the gather; their light is the analytic spot / a glow

type Tree = { [k: string]: number | number[] | Tree };
function assignTree(dst: Tree, src: Tree) {
  for (const k of Object.keys(src)) {
    const s = src[k], d = dst[k];
    if (typeof s === 'number') dst[k] = s;
    else if (Array.isArray(s)) { if (Array.isArray(d) && d.length === s.length) s.forEach((x, i) => { d[i] = x; }); else dst[k] = [...s]; }
    else assignTree((typeof d === 'object' && !Array.isArray(d) ? d : (dst[k] = {})) as Tree, s);
  }
}
export function cloneRig(r: Rig): Rig { const o = {} as Rig; assignTree(o as unknown as Tree, r as unknown as Tree); return o; }
/** Deep in-place copy (keeps the RIG object identity: sliders and rigBoxes() hold on to it). */
export function assignRig(dst: Rig, src: Rig) { assignTree(dst as unknown as Tree, src as unknown as Tree); }
/** Drop every free adjustment (viewer reset: RIG_DEFAULTS has none, and assignRig only copies keys that exist in the source). */
export function clearAdjs(rig: Rig) { const walk = (o: Record<string, unknown>) => { for (const k of Object.keys(o)) { if (/adj$/i.test(k)) delete o[k]; else if (o[k] && typeof o[k] === 'object' && !Array.isArray(o[k])) walk(o[k] as Record<string, unknown>); } }; walk(rig as unknown as Record<string, unknown>); }
/** The shipped numbers, captured at load — the viewer's "reset". */
export const RIG_DEFAULTS: Rig = cloneRig(RIG);

/** `export const RIG: Rig = {…};` as TypeScript source in the exact layout used above (group per block, member per line, leaves inline,
 *  numbers rounded to 3 decimals) so pasting it over the block is a clean, reviewable diff. */
export function rigSource(rig: Rig = RIG): string {
  const num = (n: number) => String(+n.toFixed(3));
  const zeroAdj = (v: unknown) => !!v && typeof v === 'object' && !Array.isArray(v) && 'off' in (v as object) && 'rot' in (v as object) && [...(v as Adj).off, ...(v as Adj).rot].every(x => Math.abs(x) < 5e-4);
  const keep = ([k, x]: [string, unknown]) => !(/adj$/i.test(k) && zeroAdj(x));   // an all-zero adjustment is the same as none: leave it out of the source
  const inline = (v: unknown): string => typeof v === 'number' ? num(v) : Array.isArray(v) ? `[${v.map(x => num(x as number)).join(', ')}]`
    : `{ ${Object.entries(v as object).filter(keep).map(([k, x]) => `${k}: ${inline(x)}`).join(', ')} }`;
  const lines = ['export const RIG: Rig = {'];
  for (const [group, members] of Object.entries(rig)) {
    lines.push(`  ${group}: {`);
    for (const [k, v] of Object.entries(members as object).filter(keep)) lines.push(`    ${k}: ${inline(v)},`);
    lines.push('  },');
  }
  lines.push('};');
  return lines.join('\n');
}

/** A right-handed frame: origin + side (x), up (y), along (z) unit axes. Every group / part is laid out in one, and its Adj acts in its parent's. */
export interface Frame { o: Vec3; x: Vec3; y: Vec3; z: Vec3; }
/** Frame from an origin, an 'along' axis and an up hint (Gram-Schmidt; falls back when the axis is (anti)parallel to the hint). */
export function frameFrom(o: Vec3, along: Vec3, upHint: Vec3 = [0, 1, 0]): Frame {
  const z = v3.normalize(along);
  let y = v3.sub(upHint, v3.scale(z, v3.dot(upHint, z))); if (v3.len(y) < 1e-3) y = v3.sub([1, 0, 0], v3.scale(z, z[0]));
  y = v3.normalize(y); const x = v3.cross(y, z);
  return { o: v3.copy(o), x, y, z };
}
function rotAxis(v: Vec3, axis: Vec3, ang: number): Vec3 { return Math.abs(ang) < 1e-6 ? v : quat.rotate(quat.axisAngle(axis, ang), v); }
/** Child frame = parent frame moved by adj.off (along the parent's axes) and turned by adj.rot (degrees about the parent's x, then y, then z). */
export function applyAdj(f: Frame, adj?: Adj): Frame {
  if (!adj) return { o: v3.copy(f.o), x: f.x, y: f.y, z: f.z };
  const o = v3.mad(v3.mad(v3.mad(f.o, f.x, adj.off[0]), f.y, adj.off[1]), f.z, adj.off[2]);
  const D = Math.PI / 180; let x = f.x, y = f.y, z = f.z;
  for (const [axis, a] of [[f.x, adj.rot[0] * D], [f.y, adj.rot[1] * D], [f.z, adj.rot[2] * D]] as [Vec3, number][]) { x = rotAxis(x, axis, a); y = rotAxis(y, axis, a); z = rotAxis(z, axis, a); }
  return { o, x, y, z };
}
/** What the viewer needs to draw a hierarchy + gizmo: one entry per node, in draw order. `parent` is the frame the node's Adj acts in
 *  (draw the handles with ITS axes), `frame` the node's own after the adjustment; boxes [boxStart, boxStart+boxCount) in `out` are its geometry. */
export interface RigNodeMeta { path: string; label: string; depth: number; parent: Frame; frame: Frame; boxStart: number; boxCount: number; getAdj: () => Adj; }
type MetaSink = RigNodeMeta[] | undefined;
const ZERO_ADJ = (): Adj => ({ off: [0, 0, 0], rot: [0, 0, 0] });
/** Lazily materialize the Adj on its owner (so untouched nodes never appear in rigSource output). */
function adjOf<T extends object>(owner: T, key: keyof T): () => Adj { return () => ((owner[key] as unknown as Adj | undefined) ?? ((owner[key] as unknown as Adj) = ZERO_ADJ())); }

/** One stick part as a yawed box laid along its frame's z from the frame origin (after the part's own Adj in the group frame `g`).
 *  Boxes only yaw, so the part follows the axis' heading and switches to an upright box when the axis is steep (relaxed carry, gun
 *  hanging down) instead of shearing. */
let curOwner = 0;   // character id stamped on the boxes rigBoxes() is emitting (set at its top)
function seg(out: Box[], g: Frame, s: Seg, albedo: Vec3, emissive: Vec3 = [0, 0, 0], meta?: MetaSink, path = '', label = '', depth = 0) {
  const f = applyAdj(g, s.adj); const dir = f.z;
  const mid = v3.mad(f.o, dir, (s.l0 + s.l1) / 2); const len = Math.max(0.001, (s.l1 - s.l0) / 2);   // (max: a slider dragged past l1 < l0 must not make an inside-out box)
  if (meta) meta.push({ path, label, depth, parent: g, frame: { ...f, o: mid }, boxStart: out.length, boxCount: 1, getAdj: adjOf(s, 'adj') });
  // fully oriented (raster-only props may tilt: Box.rot): thick across (frame x), tall along frame y, len along the axis
  const lit = emissive[0] + emissive[1] + emissive[2] > 0;
  out.push(makeBox({ c: mid, h: [s.thick, s.tall, len], yaw: Math.atan2(dir[0], dir[2]), rot: quat.fromBasis(f.x, f.y, f.z), albedo, emissive: v3.copy(emissive), flags: lit ? GLOW_FLAGS : PROP_FLAGS, owner: curOwner }));
}

/** Flattened head 'forward' (falls back to the body heading when the head looks nearly straight up/down) — the goggle strap follows the skull's yaw only. */
function headFlatForward(c: Character): Vec3 {
  const hd = c.headDir(); const flat = Math.hypot(hd[0], hd[2]);
  if (flat > 0.2) return [hd[0] / flat, 0, hd[2] / flat];
  const fw = c.forward(); const fl = Math.hypot(fw[0], fw[2]) || 1; return [fw[0] / fl, 0, fw[2] / fl];
}

/** The trifocal goggles: housing + three lenses in a tight triangle — down over the eyes (lit bright) when night vision is on, flipped up
 *  onto the forehead (dim) when off. The offsets live in a head frame that is either yaw-only (G.followPitch 0) or the skull's full
 *  rotation (1); the boxes' own orientation is the flattened head yaw in both cases (boxes only yaw). */
function gogglesBoxes(c: Character, nv: boolean, out: Box[], G: Rig['goggles'], meta?: MetaSink) {
  const head = c.bones.head; if (!head) return;
  const f = headFlatForward(c);
  let fwd: Vec3 = f, up: Vec3 = [0, 1, 0], side: Vec3 = [f[2], 0, -f[0]];        // yaw-only frame (side = quat.yaw · +X, the figure's left; the triangle is symmetric so the sign is moot)
  if (G.followPitch >= 0.5) { const q = c.headRot(); fwd = quat.rotate(q, [0, 0, 1]); up = quat.rotate(q, [0, 1, 0]); side = quat.rotate(q, [1, 0, 0]); }
  const headF: Frame = { o: v3.copy(head), x: side, y: up, z: fwd };            // the head frame: parent of the cluster
  const P = nv ? G.on : G.off;
  // cluster frame: base point (fwd/up numbers) then the state's free adjustment
  const cl = applyAdj({ ...headF, o: v3.mad(v3.mad(head, fwd, P.fwd), up, P.up) }, nv ? G.onAdj : G.offAdj);
  const path = nv ? 'goggles.on' : 'goggles.off';
  if (meta) meta.push({ path, label: nv ? 'goggles (down, NV on)' : 'goggles (up, NV off)', depth: 1, parent: headF, frame: cl, boxStart: out.length, boxCount: 4, getAdj: adjOf(G, nv ? 'onAdj' : 'offAdj') });
  const glow = nv ? G.glowOn : G.glowOff;
  const hs = applyAdj({ ...cl, o: v3.mad(cl.o, cl.z, -G.housing.back) }, G.housingAdj);
  if (meta) meta.push({ path: path + '.housing', label: 'housing', depth: 2, parent: cl, frame: hs, boxStart: out.length, boxCount: 1, getAdj: adjOf(G, 'housingAdj') });
  out.push(makeBox({ c: hs.o, h: v3.copy(G.housing.half), yaw: Math.atan2(hs.z[0], hs.z[2]), rot: quat.fromBasis(hs.x, hs.y, hs.z), albedo: COL.gogglesHousing, flags: PROP_FLAGS, owner: curOwner }));
  const ln = applyAdj(cl, G.lensAdj);
  if (meta) meta.push({ path: path + '.lenses', label: 'lens triangle', depth: 2, parent: cl, frame: ln, boxStart: out.length, boxCount: 3, getAdj: adjOf(G, 'lensAdj') });
  const yawL = Math.atan2(ln.z[0], ln.z[2]); const rotL = quat.fromBasis(ln.x, ln.y, ln.z);
  for (const [ox, oy] of [[-G.lensSide, G.lensDown], [G.lensSide, G.lensDown], [0, G.lensUp]] as [number, number][]) {
    out.push(makeBox({ c: v3.mad(v3.mad(ln.o, ln.x, ox), ln.y, oy), h: [G.lensHalf, G.lensHalf, G.lensThick], yaw: yawL, rot: rotL, albedo: COL.gogglesLens, emissive: v3.copy(glow), flags: GLOW_FLAGS, owner: curOwner }));
  }
}

/** The hand frame the pistol group lives in: origin at the wrist, z along the bore (char.gunDir), y up out of the top of the slide
 *  (−gunUnder), x sideways. */
function pistolFrame(c: Character, hand: Vec3, P: { adj?: Adj }): { handF: Frame; g: Frame } {
  const handF: Frame = frameFrom(hand, c.gunDir, v3.scale(c.gunUnder(), -1));
  return { handF, g: applyAdj(handF, P.adj) };
}

export interface RigOpts {
  /** wears the trifocal goggles at all (the player does, guards don't) */
  goggles: boolean;
  /** night vision on: goggles down over the eyes and blazing; off: parked on the forehead, idling */
  nv: boolean;
  /** 1 = pistol in the right hand; anything else = empty hand (canister slot) */
  slot: number;
  /** player: rail light on (lens lit) · guard: flashlight / weapon light currently lit */
  lightOn: boolean;
  /** guard kit instead of the player's: torch when `torch`, else a drawn pistol with a weapon-light lens; no can, no goggles */
  isGuard: boolean;
  /** guard is carrying the hand torch (patrol) rather than the drawn pistol */
  torch?: boolean;
  /** guard torch direction (the AI's smoothed beam; defaults to gunDir) */
  beamDir?: Vec3;
  /** guard light colour: lens glow = colour × RIG.torch.lensGlow while lit */
  lensColor?: Vec3;
  /** draw the sidearm holstered on the right thigh when it is not in the hand (player with a canister up; a guard on patrol carrying the torch). Off for a
   *  body whose gun has left him. */
  holster?: boolean;
  /** what an empty right hand holds when slot is 2 / 3: the canister body about to be thrown ('smoke' | 'flash'), nothing otherwise */
  held?: 'smoke' | 'flash' | null;
}

/** Held / worn props for one character as small boxes, appended to `out` (the frame's dynamic box list).
 *  Needs the character's bones, i.e. call after char.update(). */
export function rigBoxes(c: Character, o: RigOpts, out: Box[], rig: Rig = RIG, meta?: RigNodeMeta[]) {
  const hand = c.bones.handR; curOwner = c.id & 0xff;
  if (!o.isGuard) {
    if (hand && o.slot === 1) {
      const P = rig.pistol; const { handF, g } = pistolFrame(c, hand, P);
      if (meta) meta.push({ path: 'pistol', label: 'pistol (group + rail light)', depth: 1, parent: handF, frame: g, boxStart: out.length, boxCount: o.lightOn ? 5 : 4, getAdj: adjOf(P, 'adj') });
      seg(out, g, P.slide, COL.gun, undefined, meta, 'pistol.slide', 'slide + frame', 2);
      seg(out, g, P.grip, COL.gun, undefined, meta, 'pistol.grip', 'grip', 2);
      seg(out, g, P.suppressor, COL.can, undefined, meta, 'pistol.suppressor', 'suppressor', 2);
      const under: Frame = { ...g, o: v3.mad(g.o, g.y, -P.rail.under) };          // rail light under the slide; lit lens when on
      seg(out, under, P.rail, COL.torch, undefined, meta, 'pistol.rail', 'rail light body', 2);
      if (o.lightOn) seg(out, applyAdj(under, P.rail.adj), P.lens, COL.lensBody, P.lens.glow, meta, 'pistol.lens', 'rail light lens (lit)', 2);   // the lens rides on the rail body (its adj), then its own
    }
    if (o.slot !== 1) { if (o.holster) holsterBoxes(c, out, { slide: rig.pistol.slide, grip: rig.pistol.grip, suppressor: rig.pistol.suppressor }, meta); if (o.held) heldCanBoxes(c, o.held, out); }   // canister up: the Five-seveN rides the thigh, the can is in the fist
    if (o.goggles) gogglesBoxes(c, o.nv, out, rig.goggles, meta);
    return;
  }
  if (!hand) return;
  const lens: Vec3 = o.lightOn && o.lensColor ? v3.scale(o.lensColor, rig.torch.lensGlow) : [0, 0, 0];
  if (o.torch && c.alive) {                                                     // torch carried in the hand (the spot light sits at its lens)
    const handF = frameFrom(hand, o.beamDir ?? c.gunDir); const g = applyAdj(handF, rig.torch.adj);
    if (meta) meta.push({ path: 'torch', label: 'hand torch (group + light)', depth: 1, parent: handF, frame: g, boxStart: out.length, boxCount: 2, getAdj: adjOf(rig.torch, 'adj') });
    seg(out, g, rig.torch.body, COL.torch, undefined, meta, 'torch.body', 'body', 2);
    seg(out, g, rig.torch.lens, COL.lensBody, lens, meta, 'torch.lens', 'lens', 2);
    if (o.holster !== false) holsterBoxes(c, out, { slide: rig.guardPistol.slide, grip: rig.guardPistol.grip }, meta);   // torch in hand → the sidearm is on his thigh
  } else {                                                                      // drawn pistol with a weapon light (a body keeps the gun, unlit)
    const { handF, g } = pistolFrame(c, hand, rig.guardPistol);
    if (meta) meta.push({ path: 'guardPistol', label: 'guard pistol (group + light)', depth: 1, parent: handF, frame: g, boxStart: out.length, boxCount: c.alive ? 3 : 2, getAdj: adjOf(rig.guardPistol, 'adj') });
    seg(out, g, rig.guardPistol.slide, COL.gun, undefined, meta, 'guardPistol.slide', 'slide', 2);
    seg(out, g, rig.guardPistol.grip, COL.gun, undefined, meta, 'guardPistol.grip', 'grip', 2);
    if (c.alive) seg(out, g, rig.guardPistol.lens, COL.lensBody, lens, meta, 'guardPistol.lens', 'weapon light lens', 2);
  }
}

/** Sidearm in a thigh holster: barrel down the outside of the right thigh, grip up and to the rear (top of the slide facing forward). Same part
 *  boxes as the drawn gun (slide, grip, and the player's suppressor), just hung on the leg frame instead of the hand frame. */
export function holsterBoxes(c: Character, out: Box[], parts: { slide: Seg; grip: Seg; suppressor?: Seg }, meta?: MetaSink) {
  const th = c.bones.thighR, kn = c.bones.shinR, hp = c.bones.hips; if (!th || !kn || !hp) return;
  curOwner = c.id & 0xff;   // (also called on its own for a body — stamp the boxes with THIS character, not whoever rigBoxes drew last)
  const down = v3.normalize(v3.sub(kn, th)); const fwd = c.forward();
  let outward = v3.sub(th, hp); outward = v3.sub(outward, v3.scale(down, v3.dot(outward, down))); outward[1] = 0;   // away from the pelvis, level
  outward = v3.len(outward) > 1e-3 ? v3.normalize(outward) : v3.cross(fwd, [0, 1, 0]);
  const anchor = v3.mad(v3.mad(v3.mad(th, down, 0.06), outward, 0.115), fwd, 0.01);
  const g = frameFrom(anchor, down, fwd);   // z down the thigh, y = body forward → the grip (−y, raked toward −z) points back and up, as holstered
  const gg = applyAdj(g, HOLSTER_ADJ);
  if (meta) meta.push({ path: 'holster', label: 'holstered sidearm (thigh)', depth: 1, parent: g, frame: gg, boxStart: out.length, boxCount: parts.suppressor ? 3 : 2, getAdj: () => HOLSTER_ADJ });   // the group owns the boxes (the parts reuse the hand gun's Segs — nothing of their own to adjust here)
  seg(out, gg, { ...parts.slide, adj: undefined }, COL.gun);
  seg(out, gg, GRIP_HUNG(parts.grip), COL.gun);
  if (parts.suppressor) seg(out, gg, { ...parts.suppressor, adj: undefined }, COL.can);
}
const GRIP_HUNG = (grip: Seg): Seg => ({ ...grip, adj: { off: [0, -0.024, 0.05], rot: [108, 0, 0] } });
const HOLSTER_ADJ: Adj = { off: [0, 0, 0], rot: [0, 0, 0] };   // free adjustment for the viewer (one for both characters)

/** A canister in the right hand (slot 2 / 3 up): the throwable's own body — smoke can olive with a red band, stun can near-black with a pale band —
 *  standing in the fist, tilted a little forward. */
function heldCanBoxes(c: Character, kind: 'smoke' | 'flash', out: Box[]) {
  const hand = c.bones.handR; if (!hand) return;
  const f = c.forward(); const axis = v3.normalize([f[0] * 0.35, 1, f[2] * 0.35]);
  const g = frameFrom(v3.mad(hand, axis, -0.02), axis, f);
  const body: Vec3 = kind === 'flash' ? [0.07, 0.075, 0.08] : [0.25, 0.29, 0.2]; const band: Vec3 = kind === 'flash' ? [0.7, 0.7, 0.66] : [0.6, 0.08, 0.06];
  seg(out, g, { l0: -0.02, l1: 0.12, thick: 0.032, tall: 0.032 }, body);
  seg(out, g, { l0: 0.085, l1: 0.109, thick: 0.034, tall: 0.034 }, band);
}

/** The player's rail light: physically mounted under the barrel, so its position and direction ARE the gun's, aiming or not —
 *  lens `lightAhead` along the bore from the wrist and `lightUnder` below the bore axis. */
export function rigLightPose(c: Character, rig: Rig = RIG): { pos: Vec3; dir: Vec3 } {
  const { g } = pistolFrame(c, c.bones.handR ?? c.muzzle, rig.pistol);        // follows the pistol group's free adjustment…
  const r = applyAdj({ ...g, o: v3.mad(g.o, g.y, -rig.pistol.lightUnder) }, rig.pistol.rail.adj);   // …and the rail body's, so the beam leaves the drawn lens
  return { pos: v3.mad(r.o, r.z, rig.pistol.lightAhead), dir: r.z };
}

/** A guard's light origin along its beam: at the torch lens on patrol, at the weapon-light slot when the pistol is drawn. */
export function rigTorchPos(c: Character, dir: Vec3, torch: boolean, rig: Rig = RIG): Vec3 {
  const hand = c.bones.handR ?? c.muzzle;
  const g = torch ? applyAdj(applyAdj(frameFrom(hand, dir), rig.torch.adj), rig.torch.lens.adj) : applyAdj(pistolFrame(c, hand, rig.guardPistol).g, rig.guardPistol.lens.adj);   // follows the group's AND the lens part's free adjustment: the beam starts at the drawn lens
  return v3.mad(g.o, g.z, torch ? rig.torch.lightAhead : rig.guardPistol.lightAhead);
}

/** The player's rail light — a small, tight, cool-white weapon light. Declared once for game.ts and the viewer; parked and off until posed. */
export function railLightDef(owner: number, intensity: number): RtLight {
  return { name: 'pistol_light', group: 'player', kind: 1, pos: [0, -10, 0], dir: [0, 0, 1], color: [0.9, 0.96, 1.0], intensity, range: 18, innerDeg: 5, outerDeg: 15, radius: 0.035, volumetric: 1.0, owner, enabled: false, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: intensity };
}
/** A guard's torch / weapon light — warm and wider; the cone angles are game tunables. */
export function torchLightDef(name: string, owner: number, intensity: number, innerDeg: number, outerDeg: number): RtLight {
  return { name, group: 'flashlights', kind: 1, pos: [0, -10, 0], dir: [0, 0, 1], color: [1.0, 0.92, 0.78], intensity, range: 16, innerDeg, outerDeg, radius: 0.05, volumetric: 1.0, owner, enabled: true, fixtureBox: -1, flicker: 0, ttl: -1, age: 0, peakIntensity: intensity };
}

/** Relaxed torch carry (torch in the swinging hand): the beam follows the chest / aim heading, tilted by the aim pitch, and is smoothed
 *  from last frame's `prev` so the arm swing only bobs the origin, never the direction. Returns the new beam direction. */
export function torchCarryDir(c: Character, prev: Vec3): Vec3 {
  const f: Vec3 = [Math.sin(c.aimYaw), 0, Math.cos(c.aimYaw)];
  const want = v3.normalize([f[0] * 0.5 + c.chestDir[0] * 0.5, Math.tan(c.aimPitch), f[2] * 0.5 + c.chestDir[2] * 0.5]);
  return v3.normalize(v3.lerp(prev, want, 0.2));
}
