// Hand-authored office level: boxes only. Units: meters. +X east, +Z south, +Y up. Floor top at y=0.
import { Vec3, DEG, quat } from '../math/vec';
import { Box, BoxFlag, makeBox } from './boxes';
import { WORLD } from './world';

export interface LightDef {
  kind: 'point' | 'spot' | 'dir';
  pos: Vec3; dir: Vec3;
  color: Vec3;         // linear rgb, multiplied by intensity
  intensity: number;   // point/spot: candela-ish; dir: irradiance (lux-ish)
  range: number;
  innerDeg: number; outerDeg: number;
  name: string;
  group: string;       // for UI toggles / OCP ("ceiling", "exterior", "desk", ...)
  mains?: boolean;     // fed from the building breaker (default true) — false for street lighting etc.
  broad?: boolean;     // wide soft emitter → the direct pass's 'broad' denoise class (street lamps; area lights derived from fixtures set it themselves)
  fixtureBox?: number; // index of the emissive box representing the fixture (glow card)
  enabled: boolean;
  volumetric: number;  // 0..1 haze scattering multiplier for beams
  radius?: number;     // physical emitter size (m) → penumbra width; for 'dir' lights the angular radius in radians
}

export interface PatrolRoute { name: string; points: Vec3[]; wait: number[]; }
/** A corridor junction one man can hold in a lockdown (game/guards.ts): planted at `pos` facing `yaw` (Character convention: forward = [sin yaw, 0, cos yaw]), sweeping. */
export interface Chokepoint { name: string; pos: Vec3; yaw: number; }
/** The geometry and the talk for two men clearing one room (game/squad.ts RoomClear runs it: the tour stages the conference room with its own plan in demo.ts,
 *  the live AI takes `Level.clearPlans` in a lockdown). #1 is the point man: he stacks on the jamb, breaches and takes the open side of the room deep; #2 covers
 *  high behind him, follows him in a beat later onto the door side (the dead space behind an inward-opening leaf is his), then drops to a knee and puts his
 *  light under the furniture while #1 pushes up his wall. Every position must be reachable in a straight, clear leg from the one before it (the nav grid knows
 *  nothing of chairs); faces are body yaws (Character.forward = [sin, 0, cos]); aim lists are corners the muzzle visits in order. */
export interface RoomClearPlan {
  door: string;                                                        // Doors.byName — staged: the caller shuts and latches it before start(); live: taken as found (open = no breach, just the shove and in)
  room?: string;                                                       // what they call it on the net ('server room'): the live dispatcher's own lines
  breach?: 'kick' | 'open';                                            // a shut leaf: the boot (staged, or a door nobody has a key to) or — the default is 'kick' only because the tour's plan predates the field — unlatch it keyed and shove it hard into the room ('open': what staff do to their own doors)
  bounds?: { x0: number; x1: number; z0: number; z1: number };         // the room's floor inside its walls (plan validation)
  from?: [Vec3, Vec3];                                                 // staged only: where start() stands the pair before they walk into the stack (live, they come as they are)
  stack: [Vec3, Vec3]; stackFace: [number, number]; stackSide: -1 | 1; stackAim: [Vec3, Vec3];
  kickFrom: Vec3; kickFace: number; fling: number;                    // #1's breaching mark (Doors.kickSpot: square to the leaf, 0.76 m off it, by the lock), and the signed leaf velocity that throws it INTO the room (kick or shove)
  entry: [Vec3[], Vec3[]]; podFace: [number, number]; entryAim: [Vec3[], Vec3[]];   // waypoints through the doorway to each point of domination (the last one), the facing held there, the sector swept getting there
  searchPath: Vec3[]; searchFace: number; searchAimWalk: Vec3[]; searchAimThere: Vec3[];   // #1: up his wall to the far corner, checking ahead as he goes, then across the room from there
  underAim: Vec3[]; upAim: Vec3;                                       // #2: on a knee at his point, light under the table end to end, then back up onto his wall
  formUp: [Vec3[], Vec3[]]; formFace: [number, number];               // waypoints back to the door (last = where each waits), facing there
  lines: { stack1: string; stack2: string; go2: string; in1: string; side2: string; side1: string; push1: string; under2: string; nothing2: string; corner1: string; empty2: string; radio1: string };
}

/** locked: a keyed lever set, cylinder on the corridor face (doors.ts DoorDef.locked — guards carry keys; the player picks it or kicks it in; the room side always opens) */
export interface LevelDoorDef { name: string; hinge: [number, number]; closedDir: number; width: number; angle?: number; exterior?: boolean; closer?: boolean; minAngle?: number; maxAngle?: number; locked?: boolean; }
export type PropKind = 'chair' | 'cardboard' | 'bin' | 'plant';
/** One box of a loose prop, in the prop's local frame: `off` from the prop origin (on the floor, y up), `yaw` relative to the prop's heading (frameAt / Box.yaw convention). */
export interface PropPart { off: Vec3; h: Vec3; yaw: number; albedo: Vec3; }
/** Loose furniture handed to the physics layer (src/game/props.ts) instead of `boxes`: a rigid group of parts standing on the floor at (x, z) with
 *  heading `yaw`, its collision footprint (half extents of the local XZ rectangle about the origin), overall height and mass (kg). */
export interface PropDef { kind: PropKind; name: string; x: number; z: number; yaw: number; parts: PropPart[]; half: [number, number]; height: number; mass: number; }
export interface Level {
  boxes: Box[];
  /** Chairs, floor-standing cardboard boxes, bins, potted plants. Deliberately NOT in `boxes` (so the static GPU list, StaticCollision and the nav bake
   *  never see them and nothing is baked twice); the game's PropSystem simulates them and emits their boxes as dynamics every frame. */
  props: PropDef[];
  lights: LightDef[];
  /** Emissive ceiling panels etc. that the OCP / UI can switch: name -> box indices */
  switchables: { name: string; boxes: number[]; group: string; on: boolean; baseEmissive: Vec3; flicker?: boolean; mains?: boolean }[];
  playerSpawn: Vec3;
  routes: PatrolRoute[];
  /** lockdown posts (optional: the viewer's stage has none) */
  chokepoints?: Chokepoint[];
  /** rooms the lockdown pair knows how to clear (game/guards.ts planClears picks the nearest to the last fix; optional like the posts) */
  clearPlans?: RoomClearPlan[];
  doors: LevelDoorDef[];
  breaker: { pos: Vec3; boxes: number[] };
  beacons: { pos: Vec3; box: number }[];
  sky: { zenith: Vec3; horizon: Vec3 };
  /** The one mission thread's geometry (game.ts Mission): the room to reach (floor rectangle) and the door the HUD points at for it, the rack whose drive
   *  gets pulled (its index n names the switchables `rack${n}_leds` / `rack${n}_glow` that go dark; `front` is the marker point on its LED face), the door
   *  to leave by and the x past which you are outside it. */
  mission: { serverRoom: { x0: number; x1: number; z0: number; z1: number }; entryDoor: string; rack: { index: number; front: Vec3; halfW: number }; exfilDoor: string; exfilX: number; exfilZ: number };
}

const H = WORLD.ceilingY;

// palette (linear)
const C = {
  carpet: [0.13, 0.14, 0.17] as Vec3, carpetBlue: [0.09, 0.11, 0.2] as Vec3, carpetRed: [0.22, 0.07, 0.07] as Vec3,
  lino: [0.33, 0.33, 0.31] as Vec3, stone: [0.38, 0.37, 0.35] as Vec3, asphalt: [0.09, 0.09, 0.1] as Vec3, grass: [0.1, 0.16, 0.08] as Vec3,
  wall: [0.66, 0.64, 0.6] as Vec3, wallDark: [0.36, 0.38, 0.4] as Vec3, concrete: [0.42, 0.41, 0.4] as Vec3, ceiling: [0.72, 0.72, 0.7] as Vec3,
  partRed: [0.5, 0.13, 0.1] as Vec3, partBlue: [0.13, 0.17, 0.42] as Vec3, partTan: [0.55, 0.47, 0.35] as Vec3,
  wood: [0.52, 0.4, 0.27] as Vec3, woodDark: [0.25, 0.18, 0.12] as Vec3, metal: [0.3, 0.31, 0.33] as Vec3, black: [0.04, 0.04, 0.045] as Vec3,
  chair: [0.14, 0.2, 0.45] as Vec3, cardboard: [0.58, 0.44, 0.26] as Vec3, white: [0.8, 0.8, 0.78] as Vec3, plant: [0.12, 0.3, 0.1] as Vec3,
  glassFrame: [0.2, 0.22, 0.25] as Vec3, screenOff: [0.02, 0.02, 0.025] as Vec3, red: [0.55, 0.08, 0.06] as Vec3,
};

export function buildLevel(): Level {
  const boxes: Box[] = [];
  const lights: LightDef[] = [];
  const switchables: Level['switchables'] = [];
  const add = (b: Partial<Box> & { c: Vec3; h: Vec3 }) => { boxes.push(makeBox(b)); return boxes.length - 1; };
  /** axis-aligned box from min/max corners */
  const slab = (x0: number, y0: number, z0: number, x1: number, y1: number, z1: number, albedo: Vec3, extra: Partial<Box> = {}) =>
    add({ c: [(x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2], h: [Math.abs(x1 - x0) / 2, Math.abs(y1 - y0) / 2, Math.abs(z1 - z0) / 2], albedo, ...extra });

  type Opening = { a: number; b: number; y0?: number; y1?: number; mullions?: number };
  /** Wall running along X at fixed z (thickness t), from x0..x1, with openings (doors: y0=0,y1=2.2; windows: y0=0.9,y1=2.4). */
  function wallX(z: number, x0: number, x1: number, t: number, albedo: Vec3, openings: Opening[] = [], extra: Partial<Box> = {}, height = H) {
    const ops = [...openings].sort((p, q) => p.a - q.a); let cur = x0;
    for (const o of ops) {
      if (o.a > cur) slab(cur, 0, z - t / 2, o.a, height, z + t / 2, albedo, extra);
      const y0 = o.y0 ?? 0, y1 = o.y1 ?? 2.2;
      if (y0 > 0) slab(o.a, 0, z - t / 2, o.b, y0, z + t / 2, albedo, extra);
      if (y1 < height) slab(o.a, y1, z - t / 2, o.b, height, z + t / 2, albedo, extra);
      if (o.mullions) { const n = o.mullions; for (let i = 1; i < n; i++) { const mx = o.a + (o.b - o.a) * i / n; slab(mx - 0.04, y0, z - t * 0.4, mx + 0.04, y1, z + t * 0.4, C.glassFrame, extra); } }
      cur = o.b;
    }
    if (cur < x1) slab(cur, 0, z - t / 2, x1, height, z + t / 2, albedo, extra);
  }
  /** Wall running along Z at fixed x. */
  function wallZ(x: number, z0: number, z1: number, t: number, albedo: Vec3, openings: Opening[] = [], extra: Partial<Box> = {}, height = H) {
    const ops = [...openings].sort((p, q) => p.a - q.a); let cur = z0;
    for (const o of ops) {
      if (o.a > cur) slab(x - t / 2, 0, cur, x + t / 2, height, o.a, albedo, extra);
      const y0 = o.y0 ?? 0, y1 = o.y1 ?? 2.2;
      if (y0 > 0) slab(x - t / 2, 0, o.a, x + t / 2, y0, o.b, albedo, extra);
      if (y1 < height) slab(x - t / 2, y1, o.a, x + t / 2, height, o.b, albedo, extra);
      if (o.mullions) { const n = o.mullions; for (let i = 1; i < n; i++) { const mz = o.a + (o.b - o.a) * i / n; slab(x - t * 0.4, y0, mz - 0.04, x + t * 0.4, y1, mz + 0.04, C.glassFrame, extra); } }
      cur = o.b;
    }
    if (cur < z1) slab(x - t / 2, 0, cur, x + t / 2, height, z1, albedo, extra);
  }
  // ---- props ----
  /** Local→world frame of a yawed prop at (x,z): box local +X → world (cos yaw, −sin yaw), local +Z → (sin yaw, cos yaw). */
  const frameAt = (x: number, z: number, yaw: number) => { const cs = Math.cos(yaw), sn = Math.sin(yaw); return (lx: number, ly: number, lz: number): Vec3 => [x + cs * lx + sn * lz, ly, z - sn * lx + cs * lz]; };
  function desk(x: number, z: number, w: number, d: number, yaw = 0, albedo = C.wood) {
    // top + two side panels (as yawed boxes around center)
    const at = frameAt(x, z, yaw);
    add({ c: at(0, 0.735, 0), h: [w / 2, 0.025, d / 2], yaw, albedo });
    add({ c: at(-w / 2 + 0.03, 0.36, 0), h: [0.025, 0.36, d / 2 - 0.02], yaw, albedo });
    add({ c: at(w / 2 - 0.03, 0.36, 0), h: [0.025, 0.36, d / 2 - 0.02], yaw, albedo });
    add({ c: at(0, 0.45, -d / 2 + 0.03), h: [w / 2 - 0.05, 0.25, 0.012], yaw, albedo }); // modesty panel
  }
  function monitor(x: number, z: number, yaw: number, on: boolean, baseY = 0.76, keyboard = true) {   // baseY = the surface it stands on (desk 0.76, reception counter 1.15); keyboard: false for a second screen
    const at = frameAt(x, z, yaw);
    add({ c: at(0, baseY + 0.03, 0), h: [0.1, 0.03, 0.08], yaw, albedo: C.black });
    add({ c: at(0, baseY + 0.17, 0), h: [0.02, 0.12, 0.02], yaw, albedo: C.black });
    add({ c: at(0, baseY + 0.32, 0), h: [0.27, 0.17, 0.02], yaw, albedo: C.black });
    const scr = add({ c: at(0, baseY + 0.32, 0.022), h: [0.25, 0.15, 0.004], yaw, albedo: C.screenOff, emissive: on ? [0.9, 1.3, 2.2] : [0, 0, 0] });
    if (on) switchables.push({ name: `monitor${switchables.length}`, boxes: [scr], group: 'monitors', on: true, baseEmissive: [0.9, 1.3, 2.2] });
    if (keyboard) add({ c: at(0, baseY + 0.01, 0.28), h: [0.2, 0.012, 0.07], yaw, albedo: [0.08, 0.08, 0.09] });
  }
  // ---- loose props (pushed around by the physics layer) ----
  // Chairs, floor-standing cardboard boxes / stacks, bins and potted plants go to `props`, NOT `boxes`: the game's PropSystem owns them, moves them and
  // emits their boxes as dynamics every frame. Keeping them out of `boxes` (rather than leaving them in and flagging them) means the static GPU list,
  // StaticCollision and the nav bake are prop-free with no special cases and nothing is baked twice; the box indices other systems hold (switchables,
  // LightDef.fixtureBox, breaker, beacons — and main.ts's world.statics[i] ≡ boxes[i]) stay consistent because every one of them is captured from the
  // same add() sequence in this run and nothing outside buildLevel hardcodes an index. Desk-top and shelf cartons stay static (they are not on the floor).
  const props: PropDef[] = [];
  /** Fit the collision footprint (bounding rectangle of the parts in the prop frame) and height, and re-centre the prop origin on that rectangle so the
   *  body turns about — and collides symmetrically around — the middle of the group (matters for a stack whose top carton overhangs the base). */
  const fitProp = (p: PropDef) => {
    let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity, height = 0.1;
    for (const q of p.parts) {
      const cs = Math.abs(Math.cos(q.yaw)), sn = Math.abs(Math.sin(q.yaw)); const ex = cs * q.h[0] + sn * q.h[2], ez = sn * q.h[0] + cs * q.h[2];
      x0 = Math.min(x0, q.off[0] - ex); x1 = Math.max(x1, q.off[0] + ex); z0 = Math.min(z0, q.off[2] - ez); z1 = Math.max(z1, q.off[2] + ez); height = Math.max(height, q.off[1] + q.h[1]);
    }
    const mx = (x0 + x1) / 2, mz = (z0 + z1) / 2;
    if (Math.abs(mx) > 1e-6 || Math.abs(mz) > 1e-6) { const cs = Math.cos(p.yaw), sn = Math.sin(p.yaw); p.x += cs * mx + sn * mz; p.z += -sn * mx + cs * mz; for (const q of p.parts) q.off = [q.off[0] - mx, q.off[1], q.off[2] - mz]; }
    p.half = [Math.max(0.05, (x1 - x0) / 2), Math.max(0.05, (z1 - z0) / 2)]; p.height = height;
  };
  function prop(kind: PropKind, x: number, z: number, yaw: number, mass: number, parts: PropPart[], half?: [number, number]) {
    const p: PropDef = { kind, name: `${kind}${props.length}`, x, z, yaw, parts, half: [0, 0], height: 0, mass };
    fitProp(p); if (half) p.half = half; props.push(p); return p;
  }
  function chair(x: number, z: number, yaw: number, albedo = C.chair) {   // office chair on casters: seat, back, gas lift, star base
    prop('chair', x, z, yaw, 12, [
      { off: [0, 0.46, 0], h: [0.24, 0.04, 0.24], yaw: 0, albedo }, { off: [0, 0.8, 0.22], h: [0.23, 0.3, 0.035], yaw: 0, albedo },
      { off: [0, 0.22, 0], h: [0.04, 0.22, 0.04], yaw: 0, albedo: C.black }, { off: [0, 0.03, 0], h: [0.25, 0.03, 0.05], yaw: 0, albedo: C.black }, { off: [0, 0.03, 0], h: [0.05, 0.03, 0.25], yaw: 0, albedo: C.black },
    ]);
  }
  function cardboardBox(x: number, z: number, s: number, yaw: number, y = 0) {
    if (y > 0) {   // sitting on an earlier floor carton: join that prop's rigid group (expressed in its frame) so the stack moves as one
      for (let i = props.length - 1; i >= 0; i--) {
        const p = props[i]; if (p.kind !== 'cardboard') continue;
        const cs = Math.cos(p.yaw), sn = Math.sin(p.yaw), dx = x - p.x, dz = z - p.z; const lx = cs * dx - sn * dz, lz = sn * dx + cs * dz;   // world → prop local (inverse of frameAt)
        if (Math.abs(lx) > p.half[0] || Math.abs(lz) > p.half[1]) continue;
        p.parts.push({ off: [lx, y + s / 2, lz], h: [s / 2, s / 2, s / 2], yaw: yaw - p.yaw, albedo: C.cardboard }); p.mass += 1 + 22 * s ** 3; fitProp(p); return;
      }
    }
    prop('cardboard', x, z, yaw, 1 + 22 * s ** 3, [{ off: [0, y + s / 2, 0], h: [s / 2, s / 2, s / 2], yaw: 0, albedo: C.cardboard }]);   // half-full moving carton: ~5 kg at 55 cm
  }
  // --- set dressing helpers (all boxes, all diffuse) ---
  const rug = (x: number, z: number, w: number, d: number, albedo: Vec3, yaw = 0) => add({ c: [x, 0.018, z], h: [w / 2, 0.006, d / 2], yaw, albedo, flags: BoxFlag.NoShadow });
  function plant(x: number, z: number, s = 1) {   // pot + two rotated foliage masses as one rigid group; footprint = the foliage's flat sides, not its rotated corners
    prop('plant', x, z, 0, 14 * s ** 3, [
      { off: [0, 0.22 * s, 0], h: [0.2 * s, 0.22 * s, 0.2 * s], yaw: 0, albedo: [0.36, 0.3, 0.26] },
      { off: [0, 0.7 * s, 0], h: [0.26 * s, 0.28 * s, 0.26 * s], yaw: 0.6, albedo: C.plant },
      { off: [0.04 * s, 1.08 * s, -0.03 * s], h: [0.19 * s, 0.18 * s, 0.19 * s], yaw: 0.2, albedo: [C.plant[0] * 1.15, C.plant[1] * 1.15, C.plant[2] * 1.1] },
    ], [0.26 * s, 0.26 * s]);
  }
  const bin = (x: number, z: number) => prop('bin', x, z, 0.3, 2.5, [{ off: [0, 0.2, 0], h: [0.13, 0.2, 0.13], yaw: 0, albedo: [0.12, 0.12, 0.13] }]);
  function cabinet(x: number, z: number, yaw: number) {   // filing cabinet, drawer fronts on local -Z
    const at = frameAt(x, z, yaw);
    add({ c: [x, 0.66, z], h: [0.23, 0.66, 0.3], yaw, albedo: [0.58, 0.58, 0.56] });
    for (let k = 0; k < 3; k++) add({ c: at(0, 0.3 + k * 0.4, -0.305), h: [0.19, 0.012, 0.006], yaw, albedo: [0.25, 0.25, 0.26], flags: BoxFlag.NoShadow });
  }
  const papers = (x: number, y: number, z: number, yaw = 0) => { add({ c: [x, y + 0.02, z], h: [0.11, 0.02, 0.15], yaw, albedo: [0.86, 0.86, 0.82], flags: BoxFlag.NoShadow }); add({ c: [x + 0.03, y + 0.05, z - 0.02], h: [0.1, 0.012, 0.14], yaw: yaw + 0.25, albedo: [0.9, 0.88, 0.8], flags: BoxFlag.NoShadow }); };
  const wallArt = (x: number, y: number, z: number, w: number, h: number, albedo: Vec3, alongX: boolean) => add({ c: [x, y, z], h: alongX ? [w / 2, h / 2, 0.015] : [0.015, h / 2, w / 2], albedo, flags: BoxFlag.NoShadow });
  function bench(x: number, z: number, yaw: number) {
    const at = frameAt(x, z, yaw);
    add({ c: [x, 0.42, z], h: [0.7, 0.03, 0.2], yaw, albedo: C.woodDark });
    for (const sx of [-0.6, 0.6]) add({ c: at(sx, 0.2, 0), h: [0.03, 0.2, 0.18], yaw, albedo: C.metal });
  }
  function car(x: number, z: number, yaw: number, body: Vec3) {   // local +X forward
    const at = frameAt(x, z, yaw);
    add({ c: [x, 0.62, z], h: [2.2, 0.34, 0.86], yaw, albedo: body });                                   // lower body
    add({ c: at(-0.25, 1.2, 0), h: [1.25, 0.27, 0.8], yaw, albedo: [0.07, 0.08, 0.1] });                 // greenhouse (dark glass)
    add({ c: at(-0.25, 1.49, 0), h: [1.15, 0.03, 0.76], yaw, albedo: body });                             // roof
    for (const [lx, lz] of [[1.35, 0.8], [1.35, -0.8], [-1.4, 0.8], [-1.4, -0.8]]) add({ c: at(lx, 0.3, lz), h: [0.32, 0.3, 0.12], yaw, albedo: [0.04, 0.04, 0.045] });   // wheels
    for (const lz of [0.62, -0.62]) { add({ c: at(2.2, 0.7, lz), h: [0.02, 0.07, 0.14], yaw, albedo: [0.8, 0.8, 0.75], flags: BoxFlag.NoShadow }); add({ c: at(-2.2, 0.75, lz), h: [0.02, 0.06, 0.16], yaw, albedo: [0.3, 0.02, 0.02], emissive: [0.5, 0.03, 0.02], flags: BoxFlag.NoShadow }); }   // head / tail lights
  }

  function shelf(x: number, z: number, w: number, d: number, yaw: number, levels = 4, hgt = 2.0) {
    const at = frameAt(x, z, yaw);
    add({ c: at(-w / 2 + 0.02, hgt / 2, 0), h: [0.02, hgt / 2, d / 2], yaw, albedo: C.metal });
    add({ c: at(w / 2 - 0.02, hgt / 2, 0), h: [0.02, hgt / 2, d / 2], yaw, albedo: C.metal });
    for (let i = 0; i < levels; i++) { const y = 0.15 + (hgt - 0.2) * i / (levels - 1); add({ c: at(0, y, 0), h: [w / 2, 0.015, d / 2], yaw, albedo: C.metal }); if (i < levels - 1 && ((i * 7 + Math.floor(x * 3)) % 3 !== 0)) add({ c: at((i % 2 ? -0.15 : 0.12) * w, y + 0.17, 0), h: [w * 0.22, 0.15, d * 0.4], yaw, albedo: C.cardboard }); }
  }
  function ceilingPanel(x: number, z: number, on: boolean, group: string, flicker = false, w = 1.2, d = 0.6, radiance: Vec3 = [11, 10.5, 9.5]) {
    const bi = add({ c: [x, H - 0.02, z], h: [w / 2, 0.02, d / 2], albedo: C.white, emissive: on ? radiance : [0, 0, 0], flags: BoxFlag.NoPrimary });
    switchables.push({ name: `panel_${group}_${switchables.length}`, boxes: [bi], group, on, baseEmissive: radiance, flicker });
    return bi;
  }

  // ---------------- ground & structure ----------------
  // exterior ground (asphalt south/east parking, grass north/west)
  slab(0, -0.25, 0, 40, 0, 28, C.asphalt);
  slab(0, 0, 0, 40, 0.01, 3.2, C.grass, { flags: BoxFlag.NoShadow }); // thin grass tint strips (decal-like, no rays)
  slab(0, 0, 3.2, 3.2, 0.01, 28, C.grass, { flags: BoxFlag.NoShadow });
  // interior floor finishes (thin decals over the base slab; NoShadow so rays ignore them, base slab catches rays)
  const floor = (x0: number, z0: number, x1: number, z1: number, alb: Vec3) => slab(x0, 0, z0, x1, 0.004, z1, alb);   // 3 mm proud of the base: below every shadow-ray origin (1.5 cm), so a finish edge can never draw a hairline shadow under grazing light
  slab(4, -0.1, 4, 36, 0.001, 24, C.carpet); // whole interior carpet base (thin top at y=0)
  floor(4, 10, 36, 12.2, C.lino);            // corridor
  floor(4, 16, 12, 24, C.stone);             // lobby
  floor(14, 4, 22, 10, [0.2, 0.21, 0.24]);   // server room raised floor
  floor(28, 12.2, 36, 24, C.lino);           // break room
  // rugs in cubicle farm (like the reference shots)
  floor(15, 14, 19, 17, C.carpetBlue); floor(21, 18.5, 25, 21.5, C.carpetRed); floor(14.5, 19.5, 18, 22.5, C.carpetRed);

  // ceiling over building footprint (light-only; hidden from camera)
  slab(3.9, H, 3.9, 36.1, H + 0.2, 24.1, C.ceiling, { flags: BoxFlag.NoPrimary });

  // exterior walls (t=0.24), all full height: the camera orbits (Q/E) to look round them
  const ext = C.concrete;
  // south wall z=24: lobby windows x 5..11 (3 mullions), cubicle windows 14..18, 20..24, break room door 31..32.2 + window 33..35
  wallX(24, 3.88, 36.12, 0.24, ext, [
    { a: 5, b: 11, y0: 0.8, y1: 2.5, mullions: 4 }, { a: 14, b: 18, y0: 0.9, y1: 2.4, mullions: 3 }, { a: 20, b: 24, y0: 0.9, y1: 2.4, mullions: 3 },
    { a: 31, b: 32.2, y0: 0, y1: 2.2 }, { a: 33, b: 35.2, y0: 0.9, y1: 2.4, mullions: 2 },
  ]);
  // north wall z=4: a few high windows in offices
  wallX(4, 3.88, 36.12, 0.24, ext, [{ a: 24, b: 28, y0: 2.05, y1: 2.6, mullions: 3 }, { a: 6, b: 12, y0: 2.05, y1: 2.6, mullions: 4 }]);   // manager's office + conference room: clerestory strips (a cabinet / the whiteboard stand against the solid wall below; a full window behind the cabinet read as light bleeding round it)
  // west wall x=4: lobby glazing z 17..23, conference window z 5..9
  wallZ(4, 4, 24, 0.24, ext, [{ a: 17, b: 23, y0: 0.5, y1: 2.6, mullions: 4 }, { a: 5, b: 9, y0: 0.9, y1: 2.4, mullions: 3 }]);
  // east wall x=36: fire exit door z 20..21.2 (open), storage window
  wallZ(36, 4, 24, 0.24, ext, [{ a: 20, b: 21.2, y0: 0, y1: 2.2 }, { a: 6, b: 8, y0: 1.0, y1: 2.3, mullions: 2 }]);

  // ---------------- interior partitions (t=0.12) ----------------
  const iw = C.wall;
  // corridor z=10..12.2 spans x 4..36. North side wall z=10 with doors to: conference (x 9..10.2), server (17..18.2), manager (25..26.2), storage (32..33.2)
  wallX(10, 4.12, 35.88, 0.12, iw, [{ a: 6, b: 9.0, y0: 0.0, y1: H - 0.01, mullions: 3 }, { a: 9.0, b: 10.2 }, { a: 10.2, b: 13.5, y0: 0.0, y1: H - 0.01, mullions: 3 }, { a: 17, b: 18.2 }, { a: 25, b: 26.2 }, { a: 32, b: 33.2 }]);   // glazing either side of a clear door gap (no mullion in the doorway)
  // conference glass wall: the opening above (6..13.5 full height with mullions) acts as glazing frames; add a low sill + header band
  slab(6, 0, 9.94, 9.0, 0.5, 10.06, C.wallDark); slab(10.2, 0, 9.94, 13.5, 0.5, 10.06, C.wallDark); // sill (door gap at 9..10.2)
  slab(6, 2.5, 9.94, 9.0, H, 10.06, iw); slab(10.2, 2.5, 9.94, 13.5, H, 10.06, iw); // headers over the glazing (the door gap's lintel comes from wallX)
  // south side of corridor z=12.2: lobby opening (x 6..9 wide arch), cubicle farm openings (15..16.4, 22..23.4), break room door (30..31.2)
  wallX(12.2, 4.12, 35.88, 0.12, iw, [{ a: 6, b: 9, y0: 0, y1: 2.5 }, { a: 15, b: 16.4 }, { a: 22, b: 23.4 }, { a: 30, b: 31.2 }]);
  // room dividers north of corridor: conference | server x=14 ; server | manager x=22 ; manager | storage x=30
  wallZ(14, 4.12, 9.94, 0.12, iw); wallZ(22, 4.12, 9.94, 0.12, iw, [{ a: 8, b: 9.1 }]); wallZ(30, 4.12, 9.94, 0.12, iw);
  // south: lobby | cubicles x=12 (z 12.2..24) with opening 13..14.4 ; cubicles | break room x=28 with door 17..18.2
  wallZ(12, 12.26, 23.88, 0.12, iw, [{ a: 18, b: 19.6, y0: 0, y1: 2.4 }]);
  wallZ(28, 12.26, 23.88, 0.12, iw, [{ a: 17, b: 18.2 }]);

  // ---------------- lobby (x4..12, z12.2..24) ----------------
  // reception desk (L-shape), couch, low table, plants
  add({ c: [8.2, 0.55, 15.2], h: [1.6, 0.55, 0.35], albedo: C.woodDark }); add({ c: [8.2, 1.12, 15.0], h: [1.7, 0.03, 0.45], albedo: C.stone });
  add({ c: [9.95, 0.55, 16.1], h: [0.35, 0.55, 0.6], albedo: C.woodDark });
  monitor(7.6, 15.1, Math.PI, true, 1.15); chair(8.0, 14.2, Math.PI);   // monitor on the counter top (not in it), chair facing it
  add({ c: [6.0, 0.22, 21.5], h: [1.1, 0.22, 0.45], albedo: [0.3, 0.12, 0.1] }); add({ c: [6.0, 0.6, 21.9], h: [1.1, 0.35, 0.12], albedo: [0.3, 0.12, 0.1] }); // couch
  add({ c: [6.0, 0.2, 20.2], h: [0.6, 0.2, 0.35], albedo: C.woodDark }); // low table
  add({ c: [10.8, 0.3, 22.8], h: [0.3, 0.3, 0.3], albedo: C.stone }); add({ c: [10.8, 0.95, 22.8], h: [0.35, 0.4, 0.35], albedo: C.plant });
  add({ c: [4.9, 0.3, 13.3], h: [0.3, 0.3, 0.3], albedo: C.stone }); add({ c: [4.9, 0.95, 13.3], h: [0.3, 0.4, 0.3], albedo: C.plant });
  rug(6.2, 20.9, 3.4, 2.6, [0.34, 0.1, 0.09]);                                     // lounge rug under couch + table
  papers(6.2, 0.4, 20.25, 0.4); add({ c: [5.75, 0.43, 20.1], h: [0.05, 0.03, 0.05], albedo: C.white });   // magazines + cup on the low table
  rug(8.3, 13.9, 2.6, 1.5, [0.16, 0.17, 0.2]);                                      // mat in front of reception
  wallArt(11.92, 1.55, 14.2, 1.1, 0.7, [0.55, 0.32, 0.14], false); wallArt(11.92, 1.55, 21.6, 0.9, 1.1, [0.14, 0.3, 0.42], false);   // canvases on the cubicle partition wall
  plant(11.4, 12.8, 1.1); bin(9.6, 13.9);
  // company logo wall panel (emissive backlit strip, dim)
  const logo = add({ c: [8.2, 2.08, 12.33], h: [1.2, 0.22, 0.02], albedo: C.white, emissive: [0.9, 0.95, 1.1] });   // above head height (it hangs over the arch)
  switchables.push({ name: 'lobby_logo', boxes: [logo], group: 'lobby', on: true, baseEmissive: [0.9, 0.95, 1.1] });
  ceilingPanel(8, 18.5, false, 'lobby'); ceilingPanel(8, 21.5, false, 'lobby');
  // set dressing — the camera reads the north partition and the cubicle wall, so the company sign goes on the blank run east of the arch (flat coloured shapes on a
  // dark panel, under the lit strip's height and clear of it) and a coat stand with an umbrella stand beside it against the cubicle wall between the canvas and
  // the opening; the counter gets a phone, the sign-in book and a bell, the low table one more magazine, and a planter trough sits under the west glazing in the
  // corner behind the couch, where the moon comes in through both windows. Floor solids keep to the walls: the spawn, the patrol's diagonal through the arch and
  // its legs, Sam's crouch by the pier and the soak's corner spot are untouched, and the west wall keeps its one clean twenty-metre face from the corner pot to
  // the couch (the wall-snap probe slides all of it); one nav cell behind the couch and three by the cubicle wall go, none of them on anyone's way.
  add({ c: [8.45, 1.175, 14.85], h: [0.09, 0.025, 0.11], albedo: C.black }); add({ c: [8.39, 1.22, 14.85], h: [0.03, 0.02, 0.1], albedo: [0.07, 0.07, 0.08] });   // desk phone and handset, the sitter's side of the screen
  add({ c: [9.25, 1.158, 15.18], h: [0.17, 0.008, 0.125], albedo: [0.32, 0.08, 0.07] }); add({ c: [9.25, 1.171, 15.18], h: [0.155, 0.005, 0.112], albedo: [0.9, 0.88, 0.8], flags: BoxFlag.NoShadow });   // the sign-in book open on the visitors' edge…
  add({ c: [9.49, 1.157, 15.02], h: [0.07, 0.006, 0.006], yaw: 0.5, albedo: C.black, flags: BoxFlag.NoShadow });                                                    // …its pen…
  add({ c: [8.85, 1.156, 15.3], h: [0.05, 0.006, 0.05], albedo: C.black }); add({ c: [8.85, 1.19, 15.3], h: [0.036, 0.028, 0.036], yaw: 0.78, albedo: [0.72, 0.56, 0.24] });   // …and the bell
  add({ c: [6.55, 0.408, 20.12], h: [0.1, 0.008, 0.14], yaw: -0.3, albedo: [0.16, 0.28, 0.5], flags: BoxFlag.NoShadow });                                             // a magazine at the table's other end
  add({ c: [10.5, 1.55, 12.287], h: [0.9, 0.4, 0.025], albedo: [0.1, 0.13, 0.2] });                                                                                    // sign panel on the north wall (x 9.6‥11.4, under 1.95)…
  add({ c: [9.95, 1.57, 12.3155], h: [0.17, 0.17, 0.0025], albedo: [0.85, 0.42, 0.12], flags: BoxFlag.NoShadow });                                                      // …the mark: a square…
  add({ c: [9.95, 1.57, 12.32], h: [0.075, 0.075, 0.0015], albedo: [0.1, 0.13, 0.2], flags: BoxFlag.NoShadow });                                                         // …with its centre punched out…
  add({ c: [10.78, 1.65, 12.3155], h: [0.46, 0.045, 0.0025], albedo: C.white, flags: BoxFlag.NoShadow }); add({ c: [10.66, 1.5, 12.3155], h: [0.34, 0.028, 0.0025], albedo: [0.6, 0.62, 0.66], flags: BoxFlag.NoShadow });   // …and two lines of 'name'
  add({ c: [4.36, 0.2, 22.4], h: [0.2, 0.2, 0.35], albedo: [0.2, 0.2, 0.21] }); add({ c: [4.36, 0.406, 22.4], h: [0.17, 0.007, 0.31], albedo: [0.12, 0.09, 0.06], flags: BoxFlag.NoShadow });   // planter trough under the west glazing behind the couch (z 22.05‥22.75) and its soil…
  add({ c: [4.36, 0.62, 22.2], h: [0.17, 0.21, 0.17], yaw: 0.5, albedo: C.plant }); add({ c: [4.37, 0.68, 22.58], h: [0.18, 0.27, 0.18], yaw: 0.2, albedo: [C.plant[0] * 1.15, C.plant[1] * 1.15, C.plant[2] * 1.1] });   // …two clumps
  add({ c: [11.6, 0.015, 16.5], h: [0.17, 0.015, 0.17], albedo: C.black }); add({ c: [11.6, 0.89, 16.5], h: [0.02, 0.86, 0.02], albedo: C.woodDark });               // coat stand against the cubicle wall: base, pole, pegs, a coat
  add({ c: [11.6, 1.69, 16.5], h: [0.14, 0.012, 0.012], albedo: C.woodDark }); add({ c: [11.6, 1.69, 16.5], h: [0.012, 0.012, 0.14], albedo: C.woodDark });
  add({ c: [11.54, 1.3, 16.58], h: [0.11, 0.32, 0.07], yaw: -0.4, albedo: [0.3, 0.24, 0.15] });
  add({ c: [11.62, 0.25, 17.15], h: [0.11, 0.25, 0.11], albedo: [0.22, 0.23, 0.25] });                                                                                 // umbrella stand beside it, one handle showing
  add({ c: [11.6, 0.63, 17.12], h: [0.012, 0.13, 0.012], albedo: C.black, flags: BoxFlag.NoShadow }); add({ c: [11.6, 0.77, 17.155], h: [0.012, 0.012, 0.047], albedo: C.black, flags: BoxFlag.NoShadow });

  // ---------------- cubicle farm (x12..28, z12.2..24) ----------------
  // clusters of 4 desks with 1.4 m partitions
  function cubicleCluster(cx: number, cz: number, colorA: Vec3, colorB: Vec3, seed: number) {
    // central spine partition along X and cross partition along Z
    add({ c: [cx, 0.7, cz], h: [2.4, 0.7, 0.04], albedo: colorA });
    add({ c: [cx, 0.7, cz], h: [0.04, 0.7, 1.6], albedo: colorB });
    // end caps
    add({ c: [cx - 2.4, 0.7, cz], h: [0.04, 0.7, 0.9], albedo: colorA }); add({ c: [cx + 2.4, 0.7, cz], h: [0.04, 0.7, 0.9], albedo: colorA });
    // conventional cubicle cells: the desk runs along the spine partition with the screen against it, the worker faces the
    // partition and the chair sits behind them toward the cell's open (aisle) side — flipped from the first layout, which
    // parked the chairs in the spine and read as desks facing into the aisle
    const spots: [number, number, number][] = [[-1.2, -0.55, Math.PI], [1.2, -0.55, Math.PI], [-1.2, 0.55, 0], [1.2, 0.55, 0]];
    spots.forEach(([dx, dz, yaw], i) => {
      desk(cx + dx, cz + dz, 1.6, 0.8, yaw);
      const out = Math.sign(dz);                                   // +1 south cells, -1 north cells (away from the spine)
      const on = ((seed + i) % 3) === 0; monitor(cx + dx, cz + dz - out * 0.2, yaw, on);             // screen 20 cm from the spine, facing out into the cell (monitor() faces local +Z, like the desk)
      const jitter = ((seed * 31 + i * 17) % 10) / 10 - 0.5;
      chair(cx + dx + jitter * 0.3, cz + dz + out * 0.78, yaw + jitter * 0.6);                     // in the cell opening, facing in toward the desk and screen
      if ((seed + i) % 4 === 1) add({ c: [cx + dx + 0.5, 0.98, cz + dz + out * 0.1], h: [0.12, 0.25, 0.15], yaw: 0.2, albedo: C.cardboard }); // box on desk
    });
  }
  cubicleCluster(16.5, 15.2, C.partRed, C.partTan, 1);
  cubicleCluster(23.0, 15.2, C.partBlue, C.partTan, 2);
  cubicleCluster(16.5, 20.6, C.partTan, C.partRed, 3);
  cubicleCluster(23.0, 20.6, C.partRed, C.partBlue, 4);
  // scattered moving boxes (like reference #3)
  cardboardBox(12.45, 13.4, 0.55, 0.3); cardboardBox(12.5, 14.1, 0.45, -0.4); cardboardBox(12.48, 13.6, 0.4, 0.9, 0.55);   // against the partition wall so the NW aisle stays a guard-width clear
  cardboardBox(19.8, 17.9, 0.6, 0.1); cardboardBox(20.5, 22.3, 0.5, 0.7); cardboardBox(26.9, 13.0, 0.6, -0.2); cardboardBox(26.8, 13.1, 0.42, 0.5, 0.6);
  cardboardBox(19.6, 12.65, 0.5, 1.2);   // (these two sit just off the cubicle patrol line: now that cartons are pushable rather than nav-blocking, a carton on the line would get shoved — audibly — every lap)
  // printer station against west partition
  add({ c: [12.5, 0.45, 16.0], h: [0.35, 0.45, 0.45], albedo: C.white }); add({ c: [12.5, 0.95, 16.0], h: [0.3, 0.06, 0.35], albedo: [0.15, 0.15, 0.16] });
  const printerLed = add({ c: [12.86, 0.85, 16.2], h: [0.005, 0.01, 0.03], albedo: C.black, emissive: [0.2, 6, 0.4], flags: BoxFlag.NoShadow });
  // water cooler
  add({ c: [27.5, 0.5, 23.4], h: [0.18, 0.5, 0.18], albedo: C.white }); add({ c: [27.5, 1.2, 23.4], h: [0.15, 0.2, 0.15], albedo: [0.3, 0.45, 0.6] });
  cabinet(24.6, 12.62, Math.PI); cabinet(25.12, 12.62, Math.PI); cabinet(27.55, 15.4, Math.PI / 2);   // filing cabinets (drawers into the room)
  papers(15.4, 0.76, 14.5, 0.3); papers(24.1, 0.76, 21.4, -0.2); papers(17.6, 0.76, 21.3, 0.9); papers(22.0, 0.76, 14.4, 0.1);
  bin(14.6, 16.3); bin(21.2, 21.7); bin(25.0, 16.2);
  wallArt(19.0, 1.6, 23.85, 1.6, 0.9, [0.82, 0.82, 0.8], true);                     // whiteboard on the south wall pier between the two window bands (x 18..20)
  rug(19.9, 18.0, 2.2, 5.6, [0.2, 0.2, 0.23]);                                       // runner down the middle aisle
  plant(13.0, 23.3); plant(27.4, 12.9, 0.9);
  // set dressing — a handful of the sixteen desks get something, all on the desk tops (a second screen angled at its worker, a box fan, a jacket over a chair back,
  // a photo frame, a pot plant, a run of binders — spread so no trace-grid cell passes the level's busiest); the printer grows a document feeder and an output
  // tray on its south face with a blue recycling box past it, hard against the west partition like the cartons north of it; a clock on the corridor partition
  // between the two openings; a wayfinding placard hung inside each opening (thin, no rays, hem at 2.32 over the door heads). Every aisle, the patrol's eight
  // legs, the tour's firefight route and posts, the takedown strip and the smoke crossing are exactly as they were: no nav cell changes, and the one new prop
  // stands 0.58 m clear of the west-aisle patrol line.
  monitor(17.16, 14.83, Math.PI - 0.35, false, 0.76, false);                                                          // second screen on the north-east desk of the red cluster, turned toward the chair
  add({ c: [15.92, 0.775, 15.6], h: [0.06, 0.015, 0.05], albedo: [0.8, 0.8, 0.78] }); add({ c: [15.92, 0.86, 15.6], h: [0.012, 0.07, 0.012], albedo: [0.8, 0.8, 0.78] });   // box fan on the desk across the spine from it: base, neck…
  add({ c: [15.92, 1.03, 15.63], h: [0.1, 0.1, 0.025], albedo: [0.8, 0.8, 0.78] }); add({ c: [15.92, 1.03, 15.66], h: [0.03, 0.03, 0.008], albedo: [0.25, 0.25, 0.27], flags: BoxFlag.NoShadow });   // …cage and hub, facing the chair
  { const ch = props.find(p => p.kind === 'chair' && Math.hypot(p.x - 21.83, p.z - 16.53) < 0.08);                     // a jacket hung over the back of the blue cluster's south-west chair (its back is to the aisle)
    if (ch) { ch.parts.push({ off: [0, 0.87, 0.277], h: [0.21, 0.24, 0.02], yaw: 0, albedo: [0.1, 0.11, 0.17] }, { off: [0, 1.117, 0.24], h: [0.2, 0.017, 0.05], yaw: 0, albedo: [0.1, 0.11, 0.17] }); ch.mass += 1; fitProp(ch); } }
  add({ c: [14.72, 0.835, 20.3], h: [0.065, 0.075, 0.008], yaw: 0.35, albedo: [0.2, 0.16, 0.12] });                   // photo frame on the tan cluster's north-west desk, facing its chair (the camera gets its back, as it would)…
  add({ c: [14.717, 0.84, 20.291], h: [0.05, 0.055, 0.0015], yaw: 0.35, albedo: [0.55, 0.62, 0.72], flags: BoxFlag.NoShadow });   // …the photo on its north face
  add({ c: [22.42, 0.82, 20.3], h: [0.06, 0.06, 0.06], albedo: [0.45, 0.28, 0.2] }); add({ c: [22.42, 0.97, 20.3], h: [0.1, 0.09, 0.1], yaw: 0.5, albedo: [0.16, 0.36, 0.14] });   // pot plant on the south-east cluster's north-west desk
  for (const [k, alb] of [[0, [0.15, 0.25, 0.5]], [1, [0.5, 0.12, 0.1]], [2, [0.2, 0.2, 0.22]]] as [number, Vec3][]) add({ c: [18.22 + k * 0.07, 0.92, 20.22], h: [0.03, 0.16, 0.12], albedo: alb });   // three binders stood at the east end of the tan cluster's north-east desk (the trace grid's lighter side of that block)
  add({ c: [12.45, 1.05, 15.86], h: [0.22, 0.04, 0.13], albedo: [0.24, 0.24, 0.26] });                                // printer: document feeder on the lid…
  add({ c: [12.55, 0.7, 16.53], h: [0.16, 0.012, 0.08], albedo: [0.24, 0.24, 0.26] });                                // …output tray on the south face…
  prop('cardboard', 12.42, 16.95, 0.1, 4, [{ off: [0, 0.2, 0], h: [0.2, 0.2, 0.15], yaw: 0, albedo: [0.14, 0.25, 0.55] }, { off: [0.02, 0.412, 0], h: [0.15, 0.012, 0.11], yaw: 0.3, albedo: [0.88, 0.88, 0.84] }]);   // …and the recycling box beyond it, a sheaf on top (loose, like the cartons)
  add({ c: [19.2, 2.05, 12.28], h: [0.16, 0.16, 0.02], albedo: [0.16, 0.16, 0.18] }); add({ c: [19.2, 2.05, 12.3015], h: [0.13, 0.13, 0.0015], albedo: [0.88, 0.88, 0.85], flags: BoxFlag.NoShadow });   // clock on the corridor partition: rim, face…
  add({ c: [19.2, 1.995, 12.3045], h: [0.006, 0.055, 0.0012], albedo: C.black, flags: BoxFlag.NoShadow }); add({ c: [19.245, 2.05, 12.3045], h: [0.045, 0.007, 0.0012], albedo: C.black, flags: BoxFlag.NoShadow });   // …hands at half past three (it is slow)
  for (const x of [15.7, 22.7]) {                                                                                        // wayfinding placards inside the two openings, lettered both faces
    add({ c: [x, 2.45, 13.1], h: [0.5, 0.13, 0.01], albedo: [0.12, 0.2, 0.34], flags: BoxFlag.NoShadow });
    for (const dz of [0.0115, -0.0115]) { add({ c: [x - 0.08, 2.47, 13.1 + dz], h: [0.33, 0.028, 0.0015], albedo: C.white, flags: BoxFlag.NoShadow }); add({ c: [x + 0.36, 2.45, 13.1 + dz], h: [0.06, 0.06, 0.0015], albedo: [0.85, 0.42, 0.12], flags: BoxFlag.NoShadow }); }
  }
  // ceiling panels: grid over the farm, only two on (one flickering)
  ceilingPanel(16.5, 15.2, false, 'cubicles'); ceilingPanel(23.0, 15.2, true, 'cubicles');
  ceilingPanel(16.5, 20.6, true, 'cubicles', true); ceilingPanel(23.0, 20.6, false, 'cubicles');
  ceilingPanel(19.8, 18.0, false, 'cubicles');

  // ---------------- corridor (z10..12.2) ----------------
  // exit signs (green) at both ends, fire extinguisher box (red), notice board
  const exitW = add({ c: [4.25, 2.45, 11.1], h: [0.02, 0.12, 0.3], albedo: C.white, emissive: [0.3, 3.2, 0.6] });
  const exitE = add({ c: [35.75, 2.45, 11.1], h: [0.02, 0.12, 0.3], albedo: C.white, emissive: [0.3, 3.2, 0.6] });
  switchables.push({ name: 'exit_signs', boxes: [exitW, exitE], group: 'corridor', on: true, baseEmissive: [0.3, 3.2, 0.6], mains: false });   // battery-backed
  add({ c: [20.0, 1.1, 12.08], h: [0.15, 0.3, 0.06], albedo: C.red });
  add({ c: [28.5, 1.5, 10.1], h: [0.9, 0.45, 0.02], albedo: [0.45, 0.35, 0.2] });
  bench(26.6, 11.86, 0); plant(4.65, 10.55, 0.9); plant(35.4, 11.7, 0.9); bin(21.3, 11.95);   // bench on the blank wall run between the east cubicle opening and the break-room door
  wallArt(11.5, 1.6, 12.12, 1.3, 0.75, [0.16, 0.28, 0.4], true); wallArt(15.6, 1.6, 10.09, 1.0, 0.7, [0.5, 0.42, 0.2], true);
  rug(20.0, 11.1, 30.0, 1.1, [0.24, 0.22, 0.2]);                                     // long runner
  ceilingPanel(8, 11.1, false, 'corridor', false, 1.2, 0.3); ceilingPanel(14, 11.1, true, 'corridor', false, 1.2, 0.3, [7, 6.8, 6.4]);
  ceilingPanel(20, 11.1, false, 'corridor', false, 1.2, 0.3); ceilingPanel(26, 11.1, false, 'corridor', false, 1.2, 0.3);
  ceilingPanel(32, 11.1, true, 'corridor', false, 1.2, 0.3, [7, 6.8, 6.4]);
  // set dressing on the north wall (the face the camera reads the whole game): an extinguisher hung west of the server-room door — 12 cm deep, so the corridor keeps
  // its width, and west of x 16.5 so it stands clear of that leaf flung into the corridor, of the stack marks east of the door and of the cubicle-door post — framed
  // prints on the long blank runs between the doors, and name plates by the server, manager and storage frames (all flush decals)
  add({ c: [16.42, 1.0, 10.12], h: [0.07, 0.24, 0.06], albedo: C.red }); add({ c: [16.42, 1.28, 10.115], h: [0.03, 0.04, 0.03], albedo: C.black });   // extinguisher body + valve
  wallArt(16.42, 1.56, 10.075, 0.2, 0.25, [0.6, 0.08, 0.06], true);                                                                                    // its sign
  wallArt(20.4, 1.6, 10.075, 0.9, 0.65, [0.42, 0.3, 0.16], true); wallArt(22.7, 1.6, 10.075, 0.7, 0.9, [0.15, 0.25, 0.35], true); wallArt(30.5, 1.6, 10.075, 0.8, 0.6, [0.5, 0.42, 0.22], true);
  for (const x of [18.5, 26.5, 33.5]) add({ c: [x, 1.55, 10.07], h: [0.09, 0.05, 0.01], albedo: [0.15, 0.15, 0.17], flags: BoxFlag.NoShadow });

  // ---------------- conference room (x4..14, z4..10) ----------------
  add({ c: [9.0, 0.72, 7.0], h: [2.2, 0.04, 0.8], albedo: C.woodDark }); add({ c: [8.0, 0.35, 7.0], h: [0.25, 0.35, 0.3], albedo: C.black }); add({ c: [10.0, 0.35, 7.0], h: [0.25, 0.35, 0.3], albedo: C.black });
  for (let i = 0; i < 4; i++) { chair(7.2 + i * 1.2, 5.75, Math.PI + (i - 1.5) * 0.1); chair(7.2 + i * 1.2, 8.25, (i * 0.13) % 0.4 - 0.2); }
  chair(6.2, 7.0, Math.PI / 2); chair(11.8, 7.0, -Math.PI / 2);
  // wall TV (east wall x=14) — standby: dim; UI can set "presentation" bright
  add({ c: [13.9, 1.5, 7.0], h: [0.04, 0.5, 0.9], albedo: C.black });
  const tv = add({ c: [13.85, 1.5, 7.0], h: [0.005, 0.45, 0.82], albedo: C.screenOff, emissive: [0.05, 0.08, 0.25] });
  switchables.push({ name: 'conference_tv', boxes: [tv], group: 'conference', on: true, baseEmissive: [2.2, 2.6, 3.6] });
  // whiteboard on north wall, credenza under it, plant in the corner, papers on the table
  add({ c: [9.0, 1.45, 4.18], h: [1.5, 0.55, 0.02], albedo: [0.85, 0.85, 0.84] });
  add({ c: [9.0, 0.38, 4.5], h: [1.4, 0.38, 0.25], albedo: C.woodDark }); plant(4.7, 4.7); plant(13.3, 9.3, 0.8);
  papers(8.4, 0.76, 6.8, 0.3); papers(10.1, 0.76, 7.3, -0.4); rug(9.0, 7.0, 5.4, 3.4, [0.17, 0.19, 0.24]);
  ceilingPanel(7.5, 7.0, false, 'conference'); ceilingPanel(10.5, 7.0, false, 'conference');
  // set dressing — all of it on the north wall the camera reads (the whiteboard's tray, markers, notes and strokes; a clock east of it under the clerestory sill;
  // a water jug and glasses on the credenza), the middle of the table (the conference phone, between the two paper stacks) and the glazing header (blind
  // cassettes room side, the west third let a little way down — its hem at 1.78, over every eye line). Nothing new touches the floor: the pair's marks and legs,
  // the parked third man's corner and every chair keep their room. No projector: the ceiling is NoPrimary because the camera looks through it, so a box hung
  // from it is either a slab floating over the table or pure ray cost.
  add({ c: [9.0, 0.885, 4.225], h: [0.62, 0.012, 0.035], albedo: C.metal, flags: BoxFlag.NoShadow });                                                     // marker tray under the whiteboard (its face is z 4.20)…
  add({ c: [8.7, 0.905, 4.235], h: [0.05, 0.008, 0.008], albedo: [0.62, 0.1, 0.08], flags: BoxFlag.NoShadow }); add({ c: [9.35, 0.905, 4.22], h: [0.05, 0.008, 0.008], yaw: 0.2, albedo: [0.12, 0.2, 0.55], flags: BoxFlag.NoShadow });   // …two markers on it
  for (const [x, y, alb] of [[7.85, 1.75, [0.86, 0.76, 0.22]], [8.07, 1.53, [0.85, 0.42, 0.52]], [10.15, 1.7, [0.42, 0.7, 0.36]], [10.22, 1.26, [0.32, 0.5, 0.8]]] as [number, number, Vec3][])
    add({ c: [x, y, 4.2025], h: [0.038, 0.038, 0.0015], albedo: alb, flags: BoxFlag.NoShadow });                                                          // four notes stuck to the board…
  for (const [x, y, hw, hh] of [[9.0, 1.63, 0.42, 0.005], [8.95, 1.3, 0.3, 0.005], [9.42, 1.465, 0.005, 0.16]] as [number, number, number, number][])
    add({ c: [x, y, 4.2025], h: [hw, hh, 0.0015], albedo: [0.14, 0.18, 0.3], flags: BoxFlag.NoShadow });                                                 // …and the strokes of somebody's diagram
  add({ c: [11.3, 1.62, 4.14], h: [0.16, 0.16, 0.02], albedo: [0.16, 0.16, 0.18] }); add({ c: [11.3, 1.62, 4.1615], h: [0.13, 0.13, 0.0015], albedo: [0.88, 0.88, 0.85], flags: BoxFlag.NoShadow });   // wall clock: rim, face…
  add({ c: [11.345, 1.62, 4.1645], h: [0.045, 0.008, 0.0012], albedo: C.black, flags: BoxFlag.NoShadow }); add({ c: [11.3, 1.677, 4.1645], h: [0.006, 0.057, 0.0012], albedo: C.black, flags: BoxFlag.NoShadow });   // …and hands at three o'clock
  add({ c: [8.05, 0.766, 4.5], h: [0.2, 0.006, 0.14], albedo: [0.14, 0.14, 0.15] });                                                                       // tray on the credenza…
  add({ c: [7.95, 0.872, 4.52], h: [0.05, 0.1, 0.05], albedo: [0.5, 0.58, 0.66] }); add({ c: [7.95, 0.987, 4.52], h: [0.036, 0.015, 0.036], albedo: [0.5, 0.58, 0.66] });   // …water jug and its neck…
  for (const [dx, dz] of [[0.1, -0.06], [0.17, 0.02], [0.09, 0.07]]) add({ c: [8.0 + dx, 0.812, 4.5 + dz], h: [0.024, 0.04, 0.024], albedo: [0.72, 0.75, 0.8] });   // …three glasses
  add({ c: [9.05, 0.775, 7.02], h: [0.11, 0.015, 0.11], albedo: [0.09, 0.09, 0.1] }); add({ c: [9.05, 0.775, 7.02], h: [0.11, 0.015, 0.11], yaw: Math.PI / 4, albedo: [0.09, 0.09, 0.1] });   // conference phone: an octagonal puck…
  add({ c: [9.05, 0.7915, 7.06], h: [0.05, 0.002, 0.03], albedo: [0.3, 0.31, 0.34], flags: BoxFlag.NoShadow });                                            // …with its keypad
  add({ c: [7.5, 2.46, 9.9], h: [1.46, 0.04, 0.035], albedo: [0.78, 0.77, 0.72] }); add({ c: [11.85, 2.46, 9.9], h: [1.6, 0.04, 0.035], albedo: [0.78, 0.77, 0.72] });   // blind cassettes under the glazing header, both runs…
  add({ c: [6.76, 2.1, 9.905], h: [0.7, 0.32, 0.006], albedo: [0.82, 0.8, 0.74] });                                                                        // …the west third a little way down

  // ---------------- server room (x14..22, z4..10) ----------------
  const DRIVE_RACK = 5; let rackFront: Vec3 = [19.55, 1.1, 7.13];   // the mission's rack (r 1, i 1): east bank, second from the middle gap — steady LEDs (not in the flicker set), its LED face toward the door
  for (let r = 0; r < 2; r++) for (let i = 0; i < 4; i++) {
    const x = 15.2 + i * 0.75 + (r ? 3.6 : 0), z = 6.6;
    add({ c: [x, 1.05, z], h: [0.33, 1.05, 0.5], albedo: [0.06, 0.06, 0.07] });
    if (r * 4 + i === DRIVE_RACK) rackFront = [x, 1.1, z + 0.53];      // marker point: on the LED face, chest height
    // status LEDs on the south face (toward the door): short horizontal runs of three green LEDs every 4U down the LEFT
    // edge of the front — a handful per rack, not every slot (which slots varies per rack) — plus, aligned with that LED
    // column, an invisible thin vertical emitter (NoPrimary + it becomes an analytic area light) so the racks actually throw a
    // faint green glow onto the floor and each other where the LEDs are, without 40 tiny lights
    const green: Vec3 = [0.25, 6.0, 0.7]; const amber: Vec3 = [6.0, 2.2, 0.2];
    const leds: number[] = []; let lo = 10, hi = -10;
    for (let u = 0; u < 10; u++) {                                   // 4U pitch ≈ 0.178 m from y = 0.32
      if (((u * 7 + i * 3 + r * 5) % 5) > 1) continue;              // ~40 % of the slots are populated on this rack
      const y = 0.32 + u * 0.178; lo = Math.min(lo, y); hi = Math.max(hi, y);
      for (let k = 0; k < 3; k++) leds.push(add({ c: [x - 0.25 + k * 0.028, y, z + 0.504], h: [0.008, 0.008, 0.003], albedo: C.black, emissive: (u + i) % 7 === 3 && k === 2 ? amber : green, flags: BoxFlag.NoShadow }));   // raster-only pinpricks (one amber fault light here and there)
    }
    if (leds.length) {
      switchables.push({ name: `rack${r * 4 + i}_leds`, boxes: leds, group: 'server_leds', on: true, baseEmissive: green, flicker: (i + r) % 3 === 0, mains: false });   // on the UPS
      const strip = add({ c: [x - 0.222, (lo + hi) / 2, z + 0.506], h: [0.03, (hi - lo) / 2 + 0.06, 0.003], albedo: C.black, emissive: [0.2, 1.9, 0.45], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });   // the glow source: never drawn, lights like the LED column would
      switchables.push({ name: `rack${r * 4 + i}_glow`, boxes: [strip], group: 'server_glow', on: true, baseEmissive: [0.2, 1.9, 0.45], flicker: (i + r) % 3 === 0, mains: false });
    }
  }
  add({ c: [21.4, 0.6, 5.0], h: [0.4, 0.6, 0.5], albedo: C.metal }); // UPS (north end, clear of the door to the manager's office)
  ceilingPanel(18.0, 7.6, true, 'server', false, 1.2, 0.6, [4.5, 6.0, 9.0]); // cold blue-ish panel
  // set dressing — read from the south / south-west camera, so it goes on the north and east walls, over the racks and in the near-back strip; the front aisle keeps
  // every clear-plan mark, the corridor patrol's look-in (17.6, 8.5), the tour's rack-front strip (x 16.4‥19.6, z ≈ 7.8) and the rack-check spot exactly as they were.
  // The one new solid thing on walkable floor is the crash cart, parked in the dead pocket off the east bank's end (the 0.5 m gap there was never passable), 1.4 m
  // from #1's point of domination and north of the side door's full swing.
  { const tray: Vec3 = [0.16, 0.17, 0.19], lip: Vec3 = [0.24, 0.25, 0.27];
    add({ c: [18.1, 2.43, 6.6], h: [3.55, 0.015, 0.16], albedo: tray });                                            // cable tray along both banks (x 14.55‥21.65), a hand above the rack tops — clear of the ceiling panel's footprint, so it shadows nothing on the aisle
    for (const dz of [-0.16, 0.16]) add({ c: [18.1, 2.475, 6.6 + dz], h: [3.55, 0.03, 0.012], albedo: lip });      // its side lips
    add({ c: [21.4, 2.43, 5.35], h: [0.12, 0.015, 1.09], albedo: tray });                                            // branch north over the UPS…
    add({ c: [21.4, 1.83, 4.17], h: [0.09, 0.6, 0.045], albedo: C.black });                                           // …and the cable riser down the north wall into it
    for (const x of [15.95, 17.45, 19.55, 21.05]) add({ c: [x, 2.27, 6.87], h: [0.045, 0.17, 0.045], albedo: C.black, flags: BoxFlag.NoShadow });   // drops from the tray into four of the racks (raster only: nothing up there to shadow)
    const cx = 21.63, cz = 7.47;                                                                                       // crash cart: end frames, low shelf with a spare unit, top with a dark screen facing the aisle
    for (const dz of [-0.265, 0.265]) add({ c: [cx, 0.47, cz + dz], h: [0.22, 0.47, 0.012], albedo: C.metal });
    add({ c: [cx, 0.2, cz], h: [0.21, 0.012, 0.25], albedo: C.metal }); add({ c: [cx + 0.02, 0.33, cz], h: [0.16, 0.11, 0.2], albedo: [0.06, 0.06, 0.07] });
    add({ c: [cx, 0.955, cz], h: [0.23, 0.015, 0.28], albedo: [0.2, 0.21, 0.23] }); add({ c: [cx - 0.1, 0.98, cz], h: [0.07, 0.01, 0.19], albedo: [0.08, 0.08, 0.09] });
    add({ c: [cx + 0.11, 1.16, cz], h: [0.03, 0.17, 0.25], albedo: C.black }); add({ c: [cx + 0.076, 1.16, cz], h: [0.004, 0.15, 0.23], albedo: C.screenOff, flags: BoxFlag.NoShadow });
    wallArt(14.075, 1.5, 8.2, 1.4, 0.9, [0.86, 0.86, 0.85], false);                                                    // whiteboard on the west wall with the rack map: two banks as bars, the marked rack ringed, a marker tray (#2's light lands on it in a clear)
    for (const y of [1.63, 1.37]) add({ c: [14.094, y, 7.95], h: [0.004, 0.05, 0.32], albedo: [0.28, 0.3, 0.34], flags: BoxFlag.NoShadow });
    add({ c: [14.099, 1.37, 8.03], h: [0.004, 0.065, 0.05], albedo: [0.72, 0.12, 0.08], flags: BoxFlag.NoShadow });
    add({ c: [14.09, 1.02, 8.2], h: [0.03, 0.012, 0.5], albedo: C.metal, flags: BoxFlag.NoShadow });
    add({ c: [21.25, 1.02, 5.504], h: [0.13, 0.07, 0.004], albedo: C.black, flags: BoxFlag.NoShadow });               // UPS front display with two steady status LEDs (raster pinpricks like the racks', a touch dimmer; on the UPS, so no switchable)
    add({ c: [21.19, 1.13, 5.508], h: [0.008, 0.008, 0.006], albedo: C.black, emissive: [0.2, 4.5, 0.5], flags: BoxFlag.NoShadow });
    add({ c: [21.31, 1.13, 5.508], h: [0.008, 0.008, 0.006], albedo: C.black, emissive: [4.5, 1.6, 0.15], flags: BoxFlag.NoShadow });
    add({ c: [18.1, 1.55, 4.2], h: [0.3, 0.3, 0.08], albedo: [0.5, 0.51, 0.5] });                                       // wall-mount patch enclosure on the back wall (where #1's light comes through the gap between the banks)…
    add({ c: [18.34, 2.42, 4.15], h: [0.025, 0.57, 0.025], albedo: C.metal, flags: BoxFlag.NoShadow });                // …its conduit up into the ceiling (from the enclosure's top at 1.85)
    add({ c: [15.4, 0.62, 4.3], h: [0.13, 0.62, 0.13], albedo: C.red }); add({ c: [15.4, 1.3, 4.3], h: [0.06, 0.06, 0.06], albedo: C.black });   // clean-agent cylinder and valve head against the back wall…
    add({ c: [15.4, 2.16, 4.15], h: [0.025, 0.81, 0.025], albedo: [0.5, 0.1, 0.08], flags: BoxFlag.NoShadow });        // …its discharge pipe up the wall
    add({ c: [20.7, 0.011, 8.52], h: [0.13, 0.009, 1.42], albedo: [0.06, 0.06, 0.065], flags: BoxFlag.NoShadow });     // cable cover across the aisle from the east bank to the south wall: a 2 cm decal like the rugs (no rays, no collision — nobody trips)…
    add({ c: [20.7, 0.022, 8.52], h: [0.03, 0.002, 1.42], albedo: [0.6, 0.48, 0.1], flags: BoxFlag.NoShadow });        // …with its yellow centre line
    wallArt(21.925, 1.55, 7.35, 0.32, 0.42, [0.72, 0.58, 0.1], false);                                                 // electrical-hazard placard on the east wall above the cart
    const exitS = add({ c: [17.6, 2.47, 9.915], h: [0.3, 0.12, 0.02], albedo: C.white, emissive: [0.3, 3.2, 0.6] });    // exit sign over the door on the room side: joins the corridor pair's battery-backed switchable (same group, same feed), so it becomes one more dim area light
    switchables.find(s => s.name === 'exit_signs')?.boxes.push(exitS);
  }

  // ---------------- manager office (x22..30, z4..10) ----------------
  desk(26.5, 6.3, 1.8, 0.9, 0, C.woodDark); monitor(26.5, 6.1, 0, false); chair(26.5, 5.3, Math.PI * 0.95, [0.1, 0.1, 0.11]);
  chair(25.7, 7.6, 0.3); chair(27.4, 7.7, -0.25);
  shelf(29.6, 6.5, 1.6, 0.35, Math.PI / 2, 5, 2.1);
  add({ c: [23.0, 0.35, 5.0], h: [0.4, 0.35, 0.4], albedo: [0.35, 0.25, 0.15] }); add({ c: [23.0, 1.0, 5.0], h: [0.3, 0.35, 0.3], albedo: C.plant });
  rug(26.3, 6.9, 3.4, 2.6, [0.28, 0.14, 0.1]); papers(26.0, 0.76, 6.4, 0.2); bin(27.7, 6.9);
  wallArt(22.09, 1.6, 6.6, 1.2, 0.8, [0.6, 0.55, 0.4], false); plant(29.4, 9.4, 0.9);
  add({ c: [24.7, 0.3, 4.75], h: [0.9, 0.3, 0.45], albedo: [0.22, 0.2, 0.19] }); add({ c: [24.7, 0.62, 4.3], h: [0.9, 0.32, 0.12], albedo: [0.22, 0.2, 0.19] });   // low sofa under the north window (keeps the side door to the server room clear)
  // desk lamp: small fixture boxes + warm spot light
  add({ c: [27.2, 0.78, 6.0], h: [0.08, 0.02, 0.08], albedo: C.black }); add({ c: [27.2, 0.98, 6.0], h: [0.015, 0.2, 0.015], albedo: C.black });
  const lampHead = add({ c: [27.12, 1.2, 6.08], h: [0.07, 0.05, 0.07], albedo: C.black, emissive: [0, 0, 0] });
  lights.push({ kind: 'spot', pos: [27.1, 1.16, 6.1], dir: [-0.35, -0.9, 0.25], color: [1.0, 0.75, 0.45], intensity: 9, range: 6, innerDeg: 30, outerDeg: 62, name: 'desk_lamp', group: 'manager', enabled: true, volumetric: 0.25, fixtureBox: lampHead, radius: 0.06 });   // a desk lamp, not a searchlight: ~1/8 of a guard torch, wide soft cone 40 cm over the blotter
  ceilingPanel(26, 7, false, 'manager');
  // set dressing — his wall is the north one (the face the camera reads): a credenza under the clerestory with a frame and a plant on it, prints over it and over
  // the sofa, the coat stand in the corner behind his chair (#1's light finds it from the far end of his search leg, 0.7 m short of the credenza); books among the
  // cartons on the east shelves, his certificate beside them, a briefcase against the desk's end panel, the phone. New solids only line the north wall and the
  // desk's east end: the door side, the kneehole #2 lights and every mark keep their room.
  add({ c: [28.1, 0.375, 4.36], h: [0.9, 0.375, 0.22], albedo: C.woodDark });                                  // credenza (x 27.2‥29.0, out to z 4.58)…
  add({ c: [27.62, 0.87, 4.3], h: [0.09, 0.12, 0.02], albedo: [0.66, 0.56, 0.3] });                              // …a framed something on it…
  add({ c: [28.72, 0.83, 4.36], h: [0.08, 0.08, 0.08], albedo: [0.36, 0.3, 0.26] }); add({ c: [28.72, 1.03, 4.36], h: [0.13, 0.12, 0.13], yaw: 0.5, albedo: C.plant });   // …and a pot plant
  wallArt(27.55, 1.5, 4.135, 0.62, 0.82, [0.2, 0.28, 0.42], true); wallArt(28.5, 1.55, 4.135, 0.8, 0.6, [0.62, 0.5, 0.3], true);   // prints over the credenza (under the clerestory sill at 2.05)…
  wallArt(24.7, 1.5, 4.135, 1.3, 0.7, [0.5, 0.36, 0.2], true);                                                  // …and a wide one over the sofa
  add({ c: [29.55, 0.015, 4.5], h: [0.17, 0.015, 0.17], albedo: C.black });                                    // coat stand in the north-east corner: base, pole, pegs, and a coat hanging off it
  add({ c: [29.55, 0.89, 4.5], h: [0.02, 0.86, 0.02], albedo: C.woodDark });
  add({ c: [29.55, 1.69, 4.5], h: [0.14, 0.012, 0.012], albedo: C.woodDark }); add({ c: [29.55, 1.69, 4.5], h: [0.012, 0.012, 0.14], albedo: C.woodDark });
  add({ c: [29.46, 1.3, 4.57], h: [0.11, 0.32, 0.07], yaw: 0.3, albedo: [0.12, 0.12, 0.14] });
  add({ c: [29.6, 0.76, 6.05], h: [0.11, 0.12, 0.25], albedo: [0.5, 0.45, 0.3] });                              // books and binders on the east shelves, in the gaps the cartons leave (levels 1, 2, 2, 3)
  add({ c: [29.6, 1.245, 6.05], h: [0.12, 0.13, 0.3], albedo: [0.16, 0.22, 0.34] }); add({ c: [29.6, 1.235, 6.85], h: [0.11, 0.12, 0.33], albedo: [0.4, 0.12, 0.1] });
  add({ c: [29.6, 1.72, 7.0], h: [0.12, 0.13, 0.25], albedo: [0.14, 0.26, 0.22] });
  wallArt(29.925, 1.55, 8.25, 0.5, 0.4, [0.85, 0.82, 0.7], false);                                             // his certificate on the east wall, south of the shelves
  add({ c: [27.45, 0.17, 6.35], h: [0.05, 0.17, 0.22], albedo: [0.1, 0.07, 0.05] }); add({ c: [27.45, 0.36, 6.35], h: [0.012, 0.02, 0.06], albedo: C.black });   // briefcase stood against the desk's east panel (the bin is round the corner)
  add({ c: [25.95, 0.785, 6.05], h: [0.09, 0.025, 0.11], albedo: C.black }); add({ c: [25.89, 0.83, 6.05], h: [0.03, 0.02, 0.1], albedo: [0.07, 0.07, 0.08] });   // desk phone and handset

  // ---------------- storage (x30..36, z4..10) ----------------
  shelf(31.0, 6.8, 2.6, 0.5, Math.PI / 2, 4, 2.2); shelf(35.2, 6.8, 2.6, 0.5, Math.PI / 2, 4, 2.2);
  cardboardBox(33.0, 5.2, 0.7, 0.2); cardboardBox(33.6, 5.9, 0.5, -0.3); cardboardBox(33.1, 5.3, 0.45, 0.6, 0.7); cardboardBox(32.6, 8.6, 0.6, 0.1);
  // mains breaker panel by the door (OCP it for a temporary blackout, shoot it for a permanent one)
  const brk = add({ c: [34.3, 1.35, 9.9], h: [0.28, 0.36, 0.04], albedo: [0.42, 0.44, 0.45], name: 'breaker_panel' });
  const brkLed = add({ c: [34.12, 1.62, 9.85], h: [0.015, 0.015, 0.01], albedo: C.black, emissive: [0.3, 4, 0.6], flags: BoxFlag.NoShadow });
  add({ c: [34.3, 2.3, 9.9], h: [0.03, 0.6, 0.03], albedo: [0.3, 0.3, 0.32], flags: BoxFlag.NoShadow }); // conduit up to the ceiling
  // bare bulb (point light) hanging
  add({ c: [33, 2.7, 7], h: [0.005, 0.3, 0.005], albedo: C.black, flags: BoxFlag.NoShadow });
  const bulb = add({ c: [33, 2.36, 7], h: [0.04, 0.05, 0.04], albedo: C.white, emissive: [30, 22, 12], flags: BoxFlag.NoShadow });
  lights.push({ kind: 'point', pos: [33, 2.33, 7], dir: [0, -1, 0], color: [1.0, 0.7, 0.42], intensity: 28, range: 7, innerDeg: 0, outerDeg: 180, name: 'storage_bulb', group: 'storage', enabled: true, volumetric: 0.25, fixtureBox: bulb, radius: 0.06 });
  // set dressing — the janitor's end of the building: a third shelf run and a hand truck on the back wall (the face the camera reads; #2's light goes up it in a clear),
  // a hi-vis vest and the stock clipboard hung beside them, a mop bucket in the dead pocket south of the west shelf, keep-clear hatching on the floor under the
  // breaker. Nothing new stands in the aisle: every clear mark, the breaker errand's spot and the door's swing keep their clearances; the shelf costs four nav
  // cells against the north wall that only ever led to the wall.
  shelf(34.65, 4.37, 1.5, 0.45, 0, 4, 2.0);                                                              // north-wall run, east half (clear of the floor cartons at x ≈ 33 and of the east run's end)
  cardboardBox(34.3, 5.05, 0.5, 0.3); cardboardBox(34.33, 5.02, 0.34, -0.5, 0.5);                        // a carton and a half in the corner under it (loose, like the others; nobody's path)
  { const hx = 32.15, blue: Vec3 = [0.15, 0.2, 0.38];                                                     // hand truck stood against the back wall: rails, cross bar, handle, toe plate, wheels
    for (const dx of [-0.17, 0.17]) add({ c: [hx + dx, 0.66, 4.15], h: [0.015, 0.64, 0.015], albedo: blue });
    add({ c: [hx, 0.75, 4.15], h: [0.17, 0.015, 0.012], albedo: blue }); add({ c: [hx, 1.29, 4.16], h: [0.2, 0.02, 0.02], albedo: blue });
    add({ c: [hx, 0.01, 4.3], h: [0.2, 0.01, 0.15], albedo: C.metal, flags: BoxFlag.NoShadow });
    for (const dx of [-0.23, 0.23]) add({ c: [hx + dx, 0.12, 4.27], h: [0.03, 0.12, 0.1], albedo: C.black });   // (24 cm wheels: under the nav band, so the cells in front stay free)
    add({ c: [31.3, 1.72, 4.135], h: [0.36, 0.02, 0.015], albedo: C.woodDark, flags: BoxFlag.NoShadow });     // hook rail…
    add({ c: [31.16, 1.36, 4.14], h: [0.19, 0.32, 0.02], albedo: [0.6, 0.72, 0.12], flags: BoxFlag.NoShadow }); // …a hi-vis vest hanging from it…
    add({ c: [31.56, 1.42, 4.135], h: [0.11, 0.15, 0.01], albedo: [0.42, 0.32, 0.2], flags: BoxFlag.NoShadow }); add({ c: [31.56, 1.4, 4.147], h: [0.09, 0.12, 0.003], albedo: [0.88, 0.88, 0.84], flags: BoxFlag.NoShadow });   // …and the stock clipboard
    add({ c: [30.42, 0.2, 9.5], h: [0.2, 0.2, 0.15], yaw: 0.15, albedo: [0.55, 0.45, 0.1] });                 // mop bucket, wringer, and the mop stood in it (south-west pocket, well outside the leaf's arc)
    add({ c: [30.49, 0.47, 9.5], h: [0.1, 0.07, 0.14], yaw: 0.15, albedo: [0.4, 0.32, 0.08] });
    add({ c: [30.31, 0.86, 9.43], h: [0.014, 0.62, 0.014], albedo: C.wood });
    rug(34.3, 9.5, 0.84, 0.8, [0.68, 0.55, 0.08]);                                                             // keep-clear square under the breaker panel…
    for (const dx of [-0.18, 0, 0.18]) add({ c: [34.3 + dx, 0.027, 9.5], h: [0.035, 0.003, 0.3], yaw: Math.PI / 4, albedo: C.black, flags: BoxFlag.NoShadow });   // …hatched
  }

  // ---------------- break room (x28..36, z12.2..24) ----------------
  add({ c: [35.5, 0.45, 15.0], h: [0.35, 0.45, 2.2], albedo: C.white }); add({ c: [35.5, 0.92, 15.0], h: [0.38, 0.03, 2.25], albedo: [0.2, 0.2, 0.22] }); // counter
  add({ c: [35.4, 1.0, 18.2], h: [0.4, 1.0, 0.45], albedo: [0.75, 0.75, 0.74] }); // fridge
  add({ c: [31.5, 0.72, 19.5], h: [0.9, 0.03, 0.9], albedo: C.white }); add({ c: [31.5, 0.35, 19.5], h: [0.08, 0.35, 0.08], albedo: C.metal }); // table
  chair(30.4, 19.5, Math.PI / 2, [0.5, 0.5, 0.48]); chair(32.6, 19.5, -Math.PI / 2, [0.5, 0.5, 0.48]); chair(31.5, 20.6, 0, [0.5, 0.5, 0.48]); chair(31.5, 18.4, Math.PI, [0.5, 0.5, 0.48]);
  // vending machine with glowing front (faces west into the room)
  // (kept west of x=30 so the north door's approach stays clear for the nav grid)
  add({ c: [28.62, 0.95, 12.9], h: [0.5, 0.95, 0.45], albedo: [0.08, 0.08, 0.1] });
  const vend = add({ c: [28.62, 1.05, 13.36], h: [0.42, 0.65, 0.01], albedo: C.screenOff, emissive: [2.6, 0.5, 0.35] });
  switchables.push({ name: 'vending', boxes: [vend], group: 'breakroom', on: true, baseEmissive: [2.6, 0.5, 0.35] });
  const vend2 = add({ c: [29.55, 0.95, 12.9], h: [0.42, 0.95, 0.45], albedo: [0.1, 0.1, 0.3] });
  const vendB = add({ c: [29.55, 1.1, 13.36], h: [0.36, 0.55, 0.01], albedo: C.screenOff, emissive: [0.4, 0.9, 2.8] });
  switchables.push({ name: 'vending2', boxes: [vendB], group: 'breakroom', on: true, baseEmissive: [0.4, 0.9, 2.8] });
  void vend2;
  switchables.push({ name: 'printer_led', boxes: [printerLed], group: 'server_leds', on: true, baseEmissive: [0.2, 6, 0.4], mains: true });
  add({ c: [33.5, 1.3, 12.32], h: [0.6, 0.4, 0.02], albedo: [0.2, 0.35, 0.3] }); // notice board
  ceilingPanel(31.5, 19.5, false, 'breakroom'); ceilingPanel(33.5, 15.0, false, 'breakroom');
  bin(29.2, 23.4); wallArt(29.5, 1.7, 23.85, 1.4, 0.8, [0.75, 0.62, 0.25], true); rug(31.5, 19.5, 3.0, 3.0, [0.26, 0.27, 0.22]);   // art on the solid pier west of the exterior door
  // microwave with clock display
  add({ c: [35.5, 1.1, 13.4], h: [0.25, 0.15, 0.2], albedo: [0.15, 0.15, 0.16] });
  const mwClock = add({ c: [35.24, 1.12, 13.3], h: [0.005, 0.03, 0.06], albedo: C.black, emissive: [0.3, 2.5, 0.5], flags: BoxFlag.NoShadow });
  switchables.push({ name: 'microwave_clock', boxes: [mwClock], group: 'server_leds', on: true, baseEmissive: [0.3, 2.5, 0.5], mains: true });   // ('server_leds' = the non-interactive pinprick group) so the blackout takes it down with everything else
  // set dressing — the kitchen end reads on the east wall (counter kit, wall cupboards, the fridge's door furniture, a calendar) and the north wall (a bracket TV,
  // a first-aid box, sheets pinned to the notice board); the floor keeps its one table: both clear plans, the patrol's three points, the smoke beat's two watchers
  // and the fire-exit approach criss-cross the rest of it. The only new things standing on the floor are a bin tucked between counter and fridge and a mat decal.
  { const coffee = add({ c: [35.58, 1.15, 14.3], h: [0.15, 0.2, 0.17], albedo: [0.1, 0.1, 0.11] });          // coffee machine: body, hopper lid, carafe on the plate, one green 'ready' LED (in the microwave clock's switchable: mains-fed, dark in a blackout, no light of its own)
    add({ c: [35.58, 1.38, 14.3], h: [0.1, 0.03, 0.1], albedo: [0.25, 0.25, 0.27] }); add({ c: [35.38, 1.03, 14.3], h: [0.06, 0.08, 0.06], albedo: [0.05, 0.05, 0.06] }); void coffee;
    const led = add({ c: [35.426, 1.27, 14.4], h: [0.004, 0.008, 0.008], albedo: C.black, emissive: [0.3, 2.5, 0.5], flags: BoxFlag.NoShadow });
    switchables.find(s => s.name === 'microwave_clock')?.boxes.push(led);
    add({ c: [35.47, 0.953, 15.7], h: [0.22, 0.002, 0.3], albedo: [0.14, 0.15, 0.17], flags: BoxFlag.NoShadow });   // sink inset in the counter top, tap behind it
    add({ c: [35.78, 1.06, 15.7], h: [0.02, 0.11, 0.02], albedo: C.metal }); add({ c: [35.7, 1.165, 15.7], h: [0.09, 0.015, 0.015], albedo: C.metal });
    add({ c: [35.35, 0.99, 16.15], h: [0.035, 0.04, 0.035], albedo: C.white }); add({ c: [35.42, 0.99, 16.9], h: [0.035, 0.04, 0.035], albedo: [0.5, 0.1, 0.08] });   // two mugs left out
    add({ c: [35.71, 1.85, 14.9], h: [0.17, 0.32, 1.9], albedo: [0.78, 0.78, 0.76] });                            // wall cupboards over the counter (from 1.53 m: above the collision band, and the counter keeps everyone off that wall anyway)…
    for (const z of [14.27, 15.53]) add({ c: [35.537, 1.85, z], h: [0.002, 0.3, 0.006], albedo: [0.18, 0.18, 0.2], flags: BoxFlag.NoShadow });   // …door gaps
    add({ c: [34.997, 1.32, 18.2], h: [0.002, 0.006, 0.43], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow });  // fridge: freezer / fridge door split and a handle
    add({ c: [34.975, 1.0, 17.88], h: [0.015, 0.18, 0.015], albedo: C.metal });
    wallArt(35.865, 1.6, 19.35, 0.35, 0.45, [0.86, 0.85, 0.8], false);                                            // calendar on the east wall past the fridge
    for (const [x, y, w, h, alb] of [[33.1, 1.45, 0.09, 0.12, [0.88, 0.88, 0.85]], [33.45, 1.52, 0.1, 0.07, [0.8, 0.7, 0.2]], [33.82, 1.4, 0.08, 0.11, [0.55, 0.65, 0.8]], [33.38, 1.12, 0.12, 0.08, [0.8, 0.5, 0.5]]] as [number, number, number, number, Vec3][])
      add({ c: [x, y, 12.343], h: [w / 2, h / 2, 0.002], albedo: alb, flags: BoxFlag.NoShadow });                  // sheets pinned to the notice board
    add({ c: [35.0, 2.15, 12.295], h: [0.42, 0.25, 0.035], albedo: C.black }); add({ c: [35.0, 2.15, 12.332], h: [0.39, 0.22, 0.002], albedo: C.screenOff, flags: BoxFlag.NoShadow });   // bracket TV high in the north-east corner, off
    add({ c: [32.2, 1.45, 12.295], h: [0.13, 0.11, 0.035], albedo: C.white }); add({ c: [32.2, 1.45, 12.332], h: [0.05, 0.05, 0.002], albedo: [0.1, 0.45, 0.2], flags: BoxFlag.NoShadow });   // first-aid box
    rug(34.75, 15.0, 0.7, 2.4, [0.1, 0.1, 0.11]);                                                                 // anti-fatigue mat along the counter
    papers(31.15, 0.75, 19.85, 0.5); add({ c: [31.95, 0.79, 19.15], h: [0.035, 0.04, 0.035], albedo: C.white });  // magazines and a mug on the table
    bin(35.45, 17.5);                                                                                              // second bin, in the gap between the counter's end and the fridge
  }

  // ---------------- exterior ----------------
  // parked van south-east, dumpster east, fence posts, street lamp poles
  add({ c: [30.5, 0.95, 26.4], h: [2.2, 0.95, 0.95], yaw: 0.05, albedo: [0.55, 0.55, 0.52] }); add({ c: [31.95, 1.45, 26.33], h: [0.68, 0.3, 0.965], yaw: 0.05, albedo: [0.1, 0.12, 0.15] });   // van body + dark glazing band round the cab, on the body's own (yawed) axis: below the roof line and 1.5 cm proud of both flanks, so no two faces are coplanar (they z-fought)
  add({ c: [38.2, 0.65, 14.0], h: [0.8, 0.65, 1.2], albedo: [0.1, 0.25, 0.15] });
  for (let i = 0; i < 9; i++) add({ c: [1.0, 0.6, 4 + i * 2.6], h: [0.05, 0.6, 0.05], albedo: C.woodDark });
  car(11.5, 26.4, Math.PI + 0.06, [0.42, 0.08, 0.07]); car(21.0, 26.95, -0.04, [0.5, 0.52, 0.55]);     // parked along the south lot
  for (let i = 0; i < 7; i++) { if (i === 6) continue; add({ c: [6 + i * 4.2, 0.4, 24.75], h: [0.08, 0.4, 0.08], albedo: [0.7, 0.65, 0.2] }); }   // bollards along the facade (none in the break-room door's swing at x≈31.2)
  add({ c: [37.0, 0.45, 9.0], h: [0.45, 0.45, 0.4], albedo: [0.62, 0.63, 0.62] }); add({ c: [37.0, 0.45, 10.2], h: [0.45, 0.45, 0.4], albedo: [0.62, 0.63, 0.62] });   // AC condensers
  for (let i = 0; i < 4; i++) add({ c: [24.9 + i * 0.5, 0.4, 24.92], h: [0.03, 0.4, 0.3], albedo: C.metal });   // bike rack hoops (clear of the bollard at x = 27, and short of the z≈25.75 patrol line the nav grid routes along — at z 25.1 their ends snagged a capsule the grid thought was clear)
  // set dressing — the lot gets its markings: worn bay lines square to the facade framing the two cars and the van (decals), and precast wheel stops on the kerb
  // side of four bays (12 cm: under every collision band, but traced — the sodium lamp lays their shadows on the asphalt); a bike left leaning across the hoop
  // ends (its plane 4 cm south of them, bars kept short of the z 25.75 lane, so no cell changes there); a pay-and-display machine in the dead corner of the
  // walkway west of the first bollard with its P plate on the wall (a dim raster-only screen, street-fed like the tail lights); the dumpster's lid, one half thrown
  // back, bags to the rim; bulkhead housings over the break-room door and by the fire exit (housings only — no new light); and a stack of pallets against the east
  // wall two metres north of the fire exit: clear of that leaf's whole outward sweep (z ≥ 19.6), of the exfil strip beyond the door and of the Doors beat inside it.
  for (const x of [8.8, 14.2, 18.4, 23.6, 28.0, 33.4]) add({ c: [x, 0.005, 26.65], h: [0.05, 0.005, 1.25], albedo: [0.62, 0.62, 0.58], flags: BoxFlag.NoShadow });   // bay lines, z 25.4‥27.9
  for (const x of [11.5, 16.3, 25.8, 30.7]) add({ c: [x, 0.06, 27.82], h: [0.9, 0.06, 0.07], albedo: [0.6, 0.6, 0.56] });                                            // wheel stops (the grey car stands on where its own would be)
  { const bz = 25.26, tyre: Vec3 = [0.05, 0.05, 0.055], frame: Vec3 = [0.1, 0.4, 0.44];                                                                                // the bike: two wheels (a square and its diamond each), top tube, down tube, fork, seat tube, bars
    const tilt = (deg: number) => quat.axisAngle([0, 0, 1], deg * DEG);
    for (const wx of [25.02, 26.06]) { add({ c: [wx, 0.33, bz], h: [0.23, 0.23, 0.011], albedo: tyre }); add({ c: [wx, 0.33, bz], h: [0.23, 0.23, 0.011], rot: tilt(45), albedo: tyre }); }   // (the diamond's lower corner is what stands on the asphalt)
    add({ c: [25.52, 0.79, bz], h: [0.2, 0.013, 0.013], albedo: frame });
    add({ c: [25.47, 0.54, bz], h: [0.283, 0.014, 0.014], rot: tilt(-58), albedo: frame });
    add({ c: [25.17, 0.55, bz], h: [0.29, 0.012, 0.012], rot: tilt(59), albedo: frame });
    add({ c: [25.68, 0.58, bz], h: [0.286, 0.013, 0.013], rot: tilt(78), albedo: frame });
    add({ c: [25.31, 0.84, bz], h: [0.03, 0.02, 0.13], albedo: C.black });
  }
  add({ c: [4.55, 0.75, 24.34], h: [0.22, 0.75, 0.2], albedo: [0.14, 0.16, 0.22] }); add({ c: [4.55, 1.56, 24.36], h: [0.25, 0.06, 0.23], albedo: C.black });          // pay-and-display machine: body, hood…
  add({ c: [4.55, 1.2, 24.5425], h: [0.12, 0.08, 0.0025], albedo: C.screenOff, emissive: [0.12, 0.3, 0.55], flags: BoxFlag.NoShadow });                                  // …screen…
  add({ c: [4.55, 0.9, 24.5425], h: [0.2, 0.04, 0.0025], albedo: [0.8, 0.6, 0.1], flags: BoxFlag.NoShadow });                                                              // …instruction band…
  add({ c: [4.55, 2.0, 24.135], h: [0.2, 0.2, 0.015], albedo: [0.14, 0.3, 0.7], flags: BoxFlag.NoShadow });                                                                 // …and the P plate on the wall over it
  add({ c: [38.2, 1.32, 13.4], h: [0.83, 0.02, 0.61], albedo: [0.08, 0.2, 0.12] }); add({ c: [39.01, 1.91, 14.61], h: [0.02, 0.61, 0.6], albedo: [0.08, 0.2, 0.12] });   // dumpster: north half-lid down, south half thrown back upright on the rim…
  add({ c: [38.2, 1.3015, 14.6], h: [0.74, 0.0015, 0.54], albedo: [0.02, 0.02, 0.025], flags: BoxFlag.NoShadow }); add({ c: [38.0, 1.4, 14.75], h: [0.22, 0.1, 0.26], yaw: 0.4, albedo: [0.06, 0.06, 0.07] });   // …the open half dark to the rim, a bag on top
  add({ c: [31.6, 2.52, 24.17], h: [0.12, 0.07, 0.05], albedo: [0.16, 0.17, 0.18] }); add({ c: [31.6, 2.49, 24.226], h: [0.09, 0.04, 0.006], albedo: [0.55, 0.5, 0.4], flags: BoxFlag.NoShadow });     // bulkhead light over the break-room door (housing, lens — unlit)…
  add({ c: [36.17, 2.52, 19.5], h: [0.05, 0.07, 0.12], albedo: [0.16, 0.17, 0.18] }); add({ c: [36.226, 2.49, 19.5], h: [0.006, 0.04, 0.09], albedo: [0.55, 0.5, 0.4], flags: BoxFlag.NoShadow });     // …and one north of the fire exit
  add({ c: [36.72, 0.29, 17.5], h: [0.58, 0.29, 0.5], albedo: [0.4, 0.3, 0.19] });                                                                                          // pallet stack (x 36.14‥37.30, z 17‥18): four pallets as one block…
  for (let i = 0; i < 5; i++) add({ c: [36.2 + i * 0.26, 0.591, 17.5], h: [0.05, 0.011, 0.5], albedo: [0.5, 0.39, 0.25] });                                               // …and the top one's deck boards
  // the lot's two open edges get a concrete upstand (6 cm × 32 cm, flush with the slab's rim): the world's edge there becomes real geometry instead of only the
  // player's world clamp. That clamp and his static push-out are applied in turn (player.ts), and where a parked vehicle stands within a body width of the rim —
  // the van's south face 0.5‥0.8 m off it, the red car's 0.6 — a man sprinting along the rim was fed between the two, up to 30 cm inside the bodywork (the soak's
  // 'in-geometry' at (32.9, 27.6); the same numbers on main). With the rim in the collision grid the push-out sees both faces at once and turns him out at the
  // corner. It costs the rim row of nav cells, where nothing is ever authored or rolled (a fix on a man standing at the kerb resolves to the nearest free cell, as at
  // any wall); the grey car's tyres stand 1 cm off it, the dumpster keeps 0.94 m to the east one.
  slab(3.2, 0, 27.94, 40, 0.32, 28, [0.5, 0.5, 0.47]); slab(39.94, 0, 3.2, 40, 0.32, 27.94, [0.5, 0.5, 0.47]);
  // street lamps (sodium) — poles outside RC slab height is fine
  const lampAt = (x: number, z: number, name: string, dir: Vec3, on = true) => {
    // pole + arm + cobra-head housing over the emitting lens; exterior and taller than any wall
    const yawL = Math.atan2(dir[0], dir[2]);
    add({ c: [x, 2.6, z], h: [0.06, 2.6, 0.06], albedo: C.metal });
    add({ c: [x + dir[0] * 0.28, 5.24, z + dir[2] * 0.28], h: [0.04, 0.04, 0.32], yaw: yawL, albedo: C.metal });                       // arm
    add({ c: [x + dir[0] * 0.55, 5.22, z + dir[2] * 0.55], h: [0.17, 0.05, 0.27], yaw: yawL, albedo: [0.16, 0.17, 0.18] });             // slim cobra-head housing: covers the lens from above
    const head = add({ c: [x + dir[0] * 0.55, 5.15, z + dir[2] * 0.55], h: [0.13, 0.025, 0.22], yaw: yawL, albedo: C.metal, emissive: on ? [40, 24, 7] : [0, 0, 0], flags: BoxFlag.NoShadow });   // lens
    lights.push({ kind: 'spot', pos: [x + dir[0] * 0.55, 5.05, z + dir[2] * 0.55], dir: [dir[0] * 0.35, -1, dir[2] * 0.35], color: [1.0, 0.62, 0.22], intensity: 260, range: 22, innerDeg: 35, outerDeg: 65, name, group: 'exterior', enabled: on, volumetric: 0.12, fixtureBox: head, radius: 0.18, mains: false, broad: true });   // street supply, not the building's
  };
  lampAt(16.0, 27.0, 'street_lamp_s', [0, 0, -1]);
  lampAt(1.2, 20.0, 'street_lamp_w', [1, 0, 0], false);   // the lamp outside the lobby glazing where you come in: dark to start with (someone took care of it on the way in) — switch it on from the Lights panel
  lampAt(38.8, 22.0, 'street_lamp_e', [-1, 0, 0]);
  // moonlight (cool, dim) from south-west, low elevation → long window shadows into lobby/cubicles
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: normalize3([0.45, -0.5, -0.74]), color: [0.55, 0.7, 1.0], intensity: 0.9, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0.05, radius: 0.009, mains: false });   // radius = angular radius (rad): about twice the real moon — mullion shadows soften a little with distance without smearing

  // emergency beacons (rotating amber-red spots during a blackout): ceiling-hung base + dome; the dome is a switchable emissive (off)
  const beacons: Level['beacons'] = [];
  const beacon = (x: number, z: number) => {
    // (NoPrimary like the rest of the ceiling: the top-down camera looks through the ceiling plane, so ceiling fixtures would read as floating blobs)
    add({ c: [x, H - 0.06, z], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });            // base
    const dome = add({ c: [x, H - 0.19, z], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: BoxFlag.NoShadow | BoxFlag.NoPrimary });
    switchables.push({ name: `beacon${beacons.length}`, boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });
    beacons.push({ pos: [x, H - 0.3, z], box: dome });
  };
  beacon(11.0, 11.1); beacon(27.0, 11.1);      // corridor
  beacon(8.0, 18.0);                            // lobby
  beacon(20.0, 17.6);                           // cubicle floor
  beacon(32.0, 17.5);                           // break room

  // door frames: jambs + head slightly proud of the wall around every doorway that gets a leaf (see `doors` below), and the frame's stops: doors.ts hangs the leaf
  // 2 cm off its hinge line (the knuckle clearance) and ends it exactly on the far jamb's face, which left a 2 cm sight slot up the hinge side of every shut door
  // (guards' eye rays — segmentBlocked over statics + leaves — went clean through it: a hundred-odd fan segments per door in the headless probe) and a hairline
  // at the latch seam. Two full-height fillers in the leaf's own plane close both: hinge side from 1.2 cm inside the jamb to 4 mm into the leaf's edge, latch
  // side 4 mm into its tip — thinner than the leaf (±2 cm inside its ±2.5), so nothing is coplanar with it shut and what the swinging leaf passes through at
  // the root is millimetres, inside the frame; 2.4 cm proud of the hinge jamb, well inside the wall's thickness: no nav cell, kick / pick spot or swing changes.
  // locks: the server room (the job's way in — pick it or kick it), the storage room (the breaker) and the manager's office are keyed on the corridor face;
  // the manager's leaf is authored ajar with no closer, so its lock only bites once somebody pulls it to (and the office stays the quiet way round, through
  // the unlocked server_manager door)
  const doorDefs: LevelDoorDef[] = [
    { name: 'conference', hinge: [9.02, 10.0], closedDir: 0, width: 1.16, angle: -1.25, closer: false },
    { name: 'server', hinge: [17.02, 10.0], closedDir: 0, width: 1.16, locked: true },
    { name: 'manager', hinge: [25.02, 10.0], closedDir: 0, width: 1.16, angle: 0.7, closer: false, locked: true },
    { name: 'storage', hinge: [32.02, 10.0], closedDir: 0, width: 1.16, locked: true },
    { name: 'breakroom_n', hinge: [30.02, 12.2], closedDir: 0, width: 1.16, maxAngle: 1.55 },
    { name: 'server_manager', hinge: [22.0, 8.02], closedDir: Math.PI / 2, width: 1.06 },
    { name: 'breakroom_w', hinge: [28.0, 17.02], closedDir: Math.PI / 2, width: 1.16, angle: 1.35, closer: false },
    { name: 'breakroom_ext', hinge: [31.02, 24.0], closedDir: 0, width: 1.16, exterior: true, maxAngle: 3.05 },   // folds (almost) flat against the outside wall: open at the usual 110° it lay across the 1.4 m alley to the van and wedged anyone walking it
    { name: 'fire_exit', hinge: [36.0, 20.02], closedDir: Math.PI / 2, width: 1.16, exterior: true },
  ];
  for (const d of doorDefs) {
    const t = d.exterior ? 0.24 : 0.12; const gap = d.width + 0.04; const alongX = Math.abs(Math.sin(d.closedDir)) < 0.5;
    const a = alongX ? d.hinge[0] - 0.02 : d.hinge[1] - 0.02; const w = alongX ? d.hinge[1] : d.hinge[0];   // gap start along the wall, wall line coordinate
    const frameCol: Vec3 = d.exterior ? [0.2, 0.21, 0.23] : [0.3, 0.24, 0.17];
    const jamb = (u: number) => alongX ? add({ c: [u, 1.1, w], h: [0.03, 1.1, t / 2 + 0.012], albedo: frameCol }) : add({ c: [w, 1.1, u], h: [t / 2 + 0.012, 1.1, 0.03], albedo: frameCol });
    jamb(a - 0.01); jamb(a + gap + 0.01);
    if (alongX) add({ c: [a + gap / 2, 2.23, w], h: [gap / 2 + 0.04, 0.035, t / 2 + 0.012], albedo: frameCol }); else add({ c: [w, 2.23, a + gap / 2], h: [t / 2 + 0.012, 0.035, gap / 2 + 0.04], albedo: frameCol });
    const stop = (u0: number, u1: number) => { const m = (u0 + u1) / 2, hu = (u1 - u0) / 2; return alongX ? add({ c: [d.hinge[0] + m, 1.1, w], h: [hu, 1.1, 0.02], albedo: frameCol }) : add({ c: [w, 1.1, d.hinge[1] + m], h: [0.02, 1.1, hu], albedo: frameCol }); };   // (u along the closed leaf from the hinge line)
    stop(-0.012, 0.024); stop(d.width - 0.004, d.width + 0.012);
  }

  const routes: PatrolRoute[] = [
    { name: 'corridor', points: [[6, 0, 11.1], [34, 0, 11.1], [34, 0, 11.4], [18.5, 0, 11.2], [17.6, 0, 8.5], [17.6, 0, 11.0], [6, 0, 11.3]], wait: [2.5, 3, 0, 0, 3.5, 0, 0] },
    { name: 'cubicles', points: [[13.6, 0, 13.2], [19.8, 0, 13.4], [19.9, 0, 17.0], [26.3, 0, 18.0], [26.4, 0, 23.0], [19.8, 0, 23.2], [19.7, 0, 18.9], [13.5, 0, 18.0]], wait: [2, 0, 1.5, 0, 2.5, 0, 1, 0] },   // the two mid-aisle pauses stop just short of the carton at (19.8, 17.9) — where the nav grid used to halt the guard when it was a static — instead of shoving it back and forth every lap
    { name: 'lobby_break', points: [[30.6, 0, 16], [33.5, 0, 21], [30.5, 0, 13.0], [14, 0, 11.0], [10.5, 0, 13.5], [10.6, 0, 20.5], [6.5, 0, 17.5], [10.4, 0, 13.4], [14.2, 0, 11.3], [30.6, 0, 11.4]], wait: [0, 3, 0, 0, 1.5, 2.5, 3, 0, 0, 0] },
  ];
  // lockdown posts (game/guards.ts holds the corridor junction nearest the last alarm with the spare man): each is held from an end wall or the north wall, so the
  // man has the length of the corridor in front of him, the openings nearest him inside his cone, and stands clear of the patrol line (z 11.1‥11.4) and the swing
  // of every leaf — west end (back to the end wall under the exit sign: the lobby arch a step ahead on his right, the conference door on his left), the north wall
  // between the conference glazing and the west cubicle opening (that opening ahead-right, the server-room door ahead-left), east end (back to the fire-exit end:
  // the storage door — the breaker — ahead-right, the break-room door ahead-left)
  const chokepoints: Chokepoint[] = [
    { name: 'lobby arch', pos: [5.2, 0, 11.2], yaw: Math.PI / 2 },
    { name: 'cubicle door', pos: [13.6, 0, 10.5], yaw: Math.PI / 2 },
    { name: 'east end', pos: [35.0, 0, 10.8], yaw: -Math.PI / 2 },
  ];

  // room clearing in a lockdown (game/guards.ts planClears → squad.ts Clearing / RoomClear): one plan per room the pair can take from the corridor. Every door on
  // the corridor's north wall hinges on its west jamb and swings north into its room, so the pair stack on the east (latch) jamb — #1 bladed to the wall half a
  // metre off it, #2 tucked in 0.85 m behind — #1 breaches by the lock and hooks east along the near wall, #2 takes the west, door side a beat later; the break
  // room is taken through its corridor door the same way mirrored (that wall is on their left). Staff carry keys, so every breach here is 'open': the leaf comes
  // unlatched and gets shoved off its real hinge into the room (`fling` is that shove; the tour's conference plan in demo.ts kicks instead). Marks were checked
  // headlessly against this file — free nav cells, ≥ 0.5 m off every settled prop footprint, straight walkable legs, the flung leaf's arc clear of the props —
  // and are listed per room below; y in the aim lists is where the muzzle (and its light) rests: ~1 m on walls and doors, ~0.3 m under furniture.
  const clearPlans: RoomClearPlan[] = [
    { // conference room (x 4‥14, z 4‥10): the tour's own marks (demo.ts CONFERENCE_CLEAR) — the table-and-chairs block leaves clean lanes only along the south glass
      // and up the east side, so #1 dominates from the south-east corner and pushes up the TV wall while #2 holds the door side and clears the west half by light
      door: 'conference', room: 'conference room', breach: 'open', bounds: { x0: 4.12, x1: 13.94, z0: 4.12, z1: 9.94 },
      stack: [[10.55, 0, 10.5], [11.4, 0, 10.62]], stackFace: [-1.92, -1.9], stackSide: 1, stackAim: [[9.65, 1.05, 10.02], [9.7, 1.95, 10.0]],
      kickFrom: [10.0, 0, 10.76], kickFace: Math.PI, fling: -7,
      entry: [[[9.8, 0, 9.55], [12.45, 0, 9.15]], [[9.75, 0, 10.75], [9.7, 0, 9.6], [9.3, 0, 8.95]]], podFace: [-2.36, -2.6],
      entryAim: [[[13.5, 0.8, 9.5], [13.5, 1.0, 4.6], [9.0, 1.1, 4.4]], [[9.6, 1.1, 9.9], [4.5, 0.8, 9.5], [4.7, 0.9, 4.9]]],
      searchPath: [[12.75, 0, 7.25], [12.75, 0, 5.25]], searchFace: -1.25,
      searchAimWalk: [[13.4, 0.5, 4.6], [13.4, 0.5, 4.6], [10.8, 0.35, 6.9]], searchAimThere: [[7.2, 0.35, 7.1], [4.6, 1.0, 4.6], [8.0, 1.3, 4.3]],
      underAim: [[9.2, 0.32, 7.4], [7.0, 0.32, 7.0], [10.6, 0.32, 7.2]], upAim: [4.4, 1.1, 7.5],
      formUp: [[[12.75, 0, 8.75], [10.75, 0, 9.25]], [[9.8, 0, 9.4]]], formFace: [-0.99, -0.12],
      lines: { stack1: 'stack up — conference room.', stack2: 'on you.', go2: 'set… go!', in1: 'going in!', side1: 'right side clear.', side2: 'door side clear.',
        push1: 'pushing up the right — get your light under that table.', under2: 'checking under…', nothing2: 'nothing under there.', corner1: 'far corner clear.',
        empty2: "room's clear — he's not in here.", radio1: 'negative contact, conference room.' } },
    { // server room (x 14‥22, z 4‥10): the two rack banks wall the back strip off completely (the gaps are a hand too narrow to pass), so it is cleared from the front
      // aisle — #1 hooks east to the side door's corner and then walks the rack fronts to the gap between the banks to put his light through it onto the back
      // wall; #2 takes the dead corner behind the leaf (it stands north-west off the west jamb once shoved) and checks it low
      door: 'server', room: 'server room', breach: 'open', bounds: { x0: 14.12, x1: 21.88, z0: 4.12, z1: 9.94 },
      stack: [[18.55, 0, 10.5], [19.4, 0, 10.62]], stackFace: [-1.92, -1.9], stackSide: 1, stackAim: [[17.75, 1.05, 10.02], [17.7, 1.95, 10.0]],
      kickFrom: [18.0, 0, 10.76], kickFace: Math.PI, fling: -7,
      entry: [[[17.7, 0, 9.5], [21.2, 0, 9.2]], [[17.65, 0, 10.75], [17.6, 0, 9.45], [16.3, 0, 8.3], [15.0, 0, 8.4]]], podFace: [-2.2, 2.36],
      entryAim: [[[21.6, 0.8, 9.6], [21.9, 1.0, 8.5], [15.2, 1.1, 7.15]], [[17.6, 1.1, 9.9], [14.4, 0.8, 9.6], [14.4, 0.9, 7.3]]],
      searchPath: [[19.75, 0, 8.25], [18.25, 0, 7.75]], searchFace: Math.PI,
      searchAimWalk: [[17.2, 0.5, 7.2], [17.2, 0.5, 7.2], [18.1, 0.7, 6.4]], searchAimThere: [[18.1, 0.7, 4.4], [17.5, 0.9, 4.4], [18.7, 0.9, 4.4], [18.1, 0.3, 5.2]],
      underAim: [[14.6, 0.3, 9.6], [16.6, 0.3, 9.7], [14.6, 0.35, 7.4]], upAim: [14.3, 1.2, 8.4],
      formUp: [[[19.75, 0, 8.75], [18.6, 0, 9.3]], [[16.3, 0, 8.2], [17.7, 0, 8.6], [17.65, 0, 9.4]]], formFace: [-0.96, 0],
      lines: { stack1: 'stack up — server room.', stack2: 'on you.', go2: 'set… go!', in1: 'door — going in!', side1: 'right side clear — side door\'s shut.', side2: 'door side clear.',
        push1: 'walking the racks — check behind that door.', under2: 'behind the door…', nothing2: 'nothing. corner\'s empty.', corner1: 'back row\'s clear — nobody behind the racks.',
        empty2: "server room's clear.", radio1: 'negative contact, server room.' } },
    { // manager's office (x 22‥30, z 4‥10): desk and chairs fill the middle, so #1 takes the south-east corner and pushes up the east strip past the shelves to come at
      // the desk from behind, #2 takes the door side (the side door to the server room at his shoulder) and kneels to light the kneehole and under the sofa
      door: 'manager', room: "manager's office", breach: 'open', bounds: { x0: 22.12, x1: 29.94, z0: 4.12, z1: 9.94 },
      stack: [[26.55, 0, 10.5], [27.4, 0, 10.62]], stackFace: [-1.92, -1.9], stackSide: 1, stackAim: [[25.75, 1.05, 10.02], [25.7, 1.95, 10.0]],
      kickFrom: [26.0, 0, 10.76], kickFace: Math.PI, fling: -6,
      entry: [[[25.7, 0, 9.5], [28.5, 0, 9.15]], [[25.65, 0, 10.75], [25.6, 0, 9.45], [24.3, 0, 8.35], [23.1, 0, 8.6]]], podFace: [-2.36, 2.36],
      entryAim: [[[29.5, 0.8, 9.6], [29.5, 1.0, 5.0], [26.5, 0.9, 5.4]], [[25.6, 1.1, 9.9], [22.4, 0.8, 9.6], [22.15, 1.0, 8.5], [22.5, 0.9, 4.6]]],
      searchPath: [[28.25, 0, 7.75], [28.25, 0, 5.25]], searchFace: -0.87,
      searchAimWalk: [[29.3, 0.5, 4.5], [29.3, 0.5, 4.5], [26.5, 0.35, 5.2]], searchAimThere: [[26.4, 0.3, 5.35], [24.7, 0.45, 5.3], [22.5, 1.0, 4.5]],
      underAim: [[26.0, 0.3, 6.6], [27.2, 0.3, 6.6], [24.7, 0.2, 5.45]], upAim: [22.3, 1.1, 6.5],
      formUp: [[[28.25, 0, 8.6], [26.75, 0, 9.3]], [[24.4, 0, 8.3], [25.9, 0, 8.55], [25.75, 0, 9.45]]], formFace: [-1.0, 0],
      lines: { stack1: "stack up — manager's office.", stack2: 'on you.', go2: 'set… go!', in1: 'going in!', side1: 'right side clear.', side2: 'door side clear — side door\'s shut.',
        push1: 'pushing up the right — light under that desk.', under2: 'checking under the desk…', nothing2: 'nothing under it. sofa\'s clear.', corner1: 'behind the desk — clear.',
        empty2: 'office is clear.', radio1: "negative contact, manager's office." } },
    { // storage (x 30‥36, z 4‥10): one aisle between two shelf runs, cartons at the far end and by the door — a gentle shove (the leaf stops short of the carton
      // behind it), #1 up the east side to the breaker corner then the length of the aisle to the stack of boxes, #2 just inside covering the west bay, then low
      door: 'storage', room: 'storage room', breach: 'open', bounds: { x0: 30.12, x1: 35.88, z0: 4.12, z1: 9.94 },
      stack: [[33.5, 0, 10.5], [34.25, 0, 10.62]], stackFace: [-1.92, -1.9], stackSide: 1, stackAim: [[32.75, 1.05, 10.02], [32.7, 1.95, 10.0]],
      kickFrom: [33.0, 0, 10.76], kickFace: Math.PI, fling: -2.35,
      entry: [[[32.85, 0, 9.5], [34.0, 0, 8.75]], [[32.75, 0, 10.75], [32.8, 0, 9.55], [33.4, 0, 9.15]]], podFace: [-2.9, -2.5],
      entryAim: [[[35.3, 0.8, 9.5], [35.0, 1.0, 6.8], [33.0, 0.9, 4.4]], [[32.8, 1.1, 9.9], [31.0, 0.9, 8.8], [30.7, 1.0, 5.0]]],
      searchPath: [[33.75, 0, 7.25], [32.25, 0, 6.25]], searchFace: 2.5,
      searchAimWalk: [[33.0, 0.5, 4.5], [33.0, 0.5, 4.5], [33.1, 0.6, 5.3]], searchAimThere: [[34.4, 0.5, 4.5], [30.6, 0.8, 4.6], [30.9, 1.2, 7.9]],
      underAim: [[32.6, 0.3, 8.4], [31.4, 0.25, 6.8], [33.2, 0.3, 5.6]], upAim: [31.3, 1.3, 5.0],
      formUp: [[[33.75, 0, 8.0], [33.9, 0, 9.3]], [[32.85, 0, 9.6]]], formFace: [-1.02, 0],
      lines: { stack1: 'stack up — storage.', stack2: 'on you.', go2: 'set… go!', in1: 'going in!', side1: 'right side clear.', side2: 'door side — clear.',
        push1: 'checking the back — watch the left bay.', under2: 'looking low…', nothing2: 'nothing down there.', corner1: "back's clear — just boxes.",
        empty2: 'storage is clear.', radio1: 'negative contact, storage room.' } },
    { // break room (x 28‥36, z 12‥24) through its corridor door (hinged west, opens south into the room, folding toward the vending machines): stacked on the corridor's
      // south wall east of it — the wall on their LEFT — #1 hooks east along the north wall to the counter's corner and later walks the counter down to the fridge end
      // to look across and under the table; #2 turns west along the machines to the west wall and kneels to light under the table from there
      door: 'breakroom_n', room: 'break room', breach: 'open', bounds: { x0: 28.12, x1: 35.88, z0: 12.26, z1: 23.88 },
      stack: [[31.6, 0, 11.72], [32.45, 0, 11.6]], stackFace: [-1.25, -1.3], stackSide: -1, stackAim: [[31.0, 1.05, 12.18], [30.9, 1.95, 12.2]],
      kickFrom: [31.0, 0, 11.44], kickFace: 0, fling: 6.5,
      entry: [[[30.75, 0, 12.75], [34.2, 0, 13.3]], [[30.75, 0, 11.45], [30.7, 0, 12.9], [29.3, 0, 14.2], [28.9, 0, 15.0]]], podFace: [-0.29, 0.785],
      entryAim: [[[34.9, 0.8, 12.6], [35.2, 1.0, 18.0], [31.5, 0.9, 19.5]], [[30.7, 1.1, 12.4], [28.3, 0.9, 13.8], [28.2, 1.0, 17.5]]],
      searchPath: [[33.75, 0, 16.25], [33.75, 0, 20.75]], searchFace: -2.08,
      searchAimWalk: [[35.3, 0.5, 21.5], [35.3, 0.5, 21.5], [32.6, 0.35, 19.6]], searchAimThere: [[30.4, 0.8, 19.5], [28.6, 0.6, 23.3], [31.6, 1.0, 23.8], [28.2, 1.1, 17.6]],
      underAim: [[31.5, 0.32, 19.3], [30.3, 0.32, 20.5], [32.7, 0.32, 18.5]], upAim: [28.2, 1.1, 17.6],
      formUp: [[[33.0, 0, 14.5], [31.4, 0, 13.1]], [[30.1, 0, 13.9], [30.65, 0, 12.75]]], formFace: [-2.48, Math.PI],
      lines: { stack1: 'stack up — break room.', stack2: 'on you.', go2: 'set… go!', in1: 'going in!', side1: 'left side clear.', side2: 'door side clear.',
        push1: 'pushing down the left — get your light under the table.', under2: 'checking under the table…', nothing2: 'nothing under it.', corner1: "back corner clear — outside door's shut.",
        empty2: "break room's clear.", radio1: 'negative contact, break room.' } },
    { // break room again, from the cubicle side through its west door (hinged on the north jamb, latch to the south, shoved east into the room where it lies along the
      // north side of the doorway): the variant the pair takes when that is the shorter walk (squad.ts Clearing) — stacked south of the door on the partition, wall on
      // their right; #1 hooks right down the west wall to the south-west corner and later walks the south wall to the far end; #2 rounds the leaf's tip onto the north
      // side and kneels there, between the machines and the table
      door: 'breakroom_w', room: 'break room', breach: 'open', bounds: { x0: 28.12, x1: 35.88, z0: 12.26, z1: 23.88 },
      stack: [[27.4, 0, 18.7], [27.35, 0, 19.55]], stackFace: [2.43, 2.5], stackSide: 1, stackAim: [[27.98, 1.05, 17.9], [28.0, 1.95, 17.6]],
      kickFrom: [27.24, 0, 18.04], kickFace: Math.PI / 2, fling: -6.5,
      entry: [[[28.5, 0, 17.75], [28.9, 0, 21.5]], [[27.3, 0, 17.75], [28.6, 0, 17.7], [29.7, 0, 17.35], [29.6, 0, 15.6], [29.3, 0, 14.6]]], podFace: [2.3, 1.75],
      entryAim: [[[28.4, 0.8, 23.4], [31.6, 1.0, 23.7], [31.5, 0.9, 19.5]], [[28.3, 1.1, 17.5], [28.6, 0.9, 13.8], [30.6, 1.0, 12.5], [35.2, 1.0, 13.4]]],
      searchPath: [[31.25, 0, 22.25], [33.75, 0, 21.25]], searchFace: -2.23,
      searchAimWalk: [[35.3, 0.5, 22.8], [35.3, 0.5, 22.8], [34.9, 0.5, 18.4]], searchAimThere: [[30.5, 0.8, 19.4], [35.3, 1.0, 15.0], [35.2, 0.9, 12.9], [30.6, 1.1, 12.5]],
      underAim: [[31.5, 0.32, 19.4], [32.6, 0.32, 18.6], [30.4, 0.32, 20.4]], upAim: [30.6, 1.1, 12.5],
      formUp: [[[30.0, 0, 21.6], [28.9, 0, 18.9]], [[29.7, 0, 16.2], [29.4, 0, 17.45], [28.6, 0, 17.75]]], formFace: [-2.68, -Math.PI / 2],
      lines: { stack1: 'stack up — break room, side door.', stack2: 'on you.', go2: 'set… go!', in1: 'going in!', side1: 'right side clear.', side2: 'door side clear — machines, north door.',
        push1: 'taking the back wall — get your light under the table.', under2: 'checking under the table…', nothing2: 'nothing under it.', corner1: "far end clear — outside door's shut.",
        empty2: "break room's clear.", radio1: 'negative contact, break room.' } },
  ];

  return {
    boxes, props, lights, switchables,
    doors: doorDefs,   // hinged leaves for the doorway gaps (closedDir 0: leaf runs +X from the hinge along a z-wall; π/2: runs +Z along an x-wall)
    breaker: { pos: [34.3, 1.35, 9.8], boxes: [brk, brkLed] },
    beacons,
    playerSpawn: [5.1, 0, 15.2],
    routes, chokepoints, clearPlans,
    sky: { zenith: [0.015, 0.025, 0.05], horizon: [0.04, 0.05, 0.08] },
    mission: { serverRoom: { x0: 14.12, x1: 21.88, z0: 4.12, z1: 9.94 }, entryDoor: 'server', rack: { index: DRIVE_RACK, front: rackFront, halfW: 0.33 }, exfilDoor: 'fire_exit', exfilX: 36.5, exfilZ: 20.6 },   // room = inside its partitions; exfilX = clear of the east wall (x 36 ± 0.12): past it you are in the lot
  };
}

function normalize3(v: Vec3): Vec3 { const l = Math.hypot(v[0], v[1], v[2]); return [v[0] / l, v[1] / l, v[2] / l]; }
export { DEG };
