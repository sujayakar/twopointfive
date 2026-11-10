// A ROOM is a small scene about one question. It is written as code: pieces, lamps, a sky, a camera,
// a few sliders, and (if things move) where each object stands at a given time.

import { NO_TURN, add, normalize, turnBoth, turnVec, type Quat, type Vec3 } from './vec';

export type Shape = 'box' | 'ball' | 'cylinder';

export interface Material {
  matte: Vec3;
  /** what comes through to the other side: thin things only */
  through?: Vec3;
  roughness?: number;
  metal?: number;
  pattern?: 'plain' | 'checker' | 'strips';
  /** the second color or roughness of a pattern */
  matte2?: Vec3;
  roughness2?: number;
  /** meters a square, or strips a room */
  patternSize?: number;
}

export interface PieceGiven {
  shape?: Shape;
  center: Vec3;
  /** half of its width, height and depth, meters */
  half: Vec3;
  turn?: Quat;
  material: number;
  lamp?: number;
  object?: number;
  /** cut planes in the piece's own frame: normal, and how far out. What is past them is gone */
  planes?: [Vec3, number][];
}

export interface LampGiven {
  kind: 'ball' | 'panel' | 'sun';
  /** light it sends from each bit of it, the same at any size */
  radiance: Vec3;
  center?: Vec3;
  radius?: number;
  /** a panel's two half edges; it shines along their cross product */
  halfU?: Vec3;
  halfV?: Vec3;
  /** the way to the sun */
  toward?: Vec3;
  /** switched, swung or carried: it gets its own signal and a short history */
  changing?: boolean;
}

export interface Place {
  position: Vec3;
  turn: Quat;
}

export interface RoomSlider {
  key: string;
  label: string;
  min: number;
  max: number;
  step: number;
  value: number;
  about: string;
}

export interface Camera {
  /** what it looks at, how far off, and from which way (radians) */
  target: Vec3;
  distance: number;
  yaw: number;
  pitch: number;
  /** degrees, top to bottom */
  lens: number;
}

export interface RoomSpec {
  id: string;
  name: string;
  question: string;
  camera: Camera;
  exposure: number;
  sky?: { zenith: Vec3; horizon: Vec3; ground: Vec3 };
  sliders?: RoomSlider[];
  build: (b: Builder, sliders: Record<string, number>) => void;
  /** where each object stands `seconds` after the start */
  move?: (seconds: number, sliders: Record<string, number>) => Map<number, Place>;
  /** does it ever move by itself? */
  moves?: boolean;
}

export class Builder {
  materials: Material[] = [];
  pieces: PieceGiven[] = [];
  lamps: LampGiven[] = [];
  openings: { center: Vec3; halfU: Vec3; halfV: Vec3 }[] = [];
  /** object 0 is the room itself and never moves */
  objects = 1;

  material(m: Material): number {
    this.materials.push(m);
    return this.materials.length - 1;
  }
  /** a door or window that wide rays may be aimed through */
  opening(center: Vec3, halfU: Vec3, halfV: Vec3): void {
    this.openings.push({ center, halfU, halfV });
  }
  /** things that move together */
  object(): number {
    return this.objects++;
  }
  box(center: Vec3, half: Vec3, material: number, more: Partial<PieceGiven> = {}): void {
    this.pieces.push({ shape: 'box', center, half, material, ...more });
  }
  ball(center: Vec3, radius: number | Vec3, material: number, more: Partial<PieceGiven> = {}): void {
    const half: Vec3 = typeof radius === 'number' ? [radius, radius, radius] : radius;
    this.pieces.push({ shape: 'ball', center, half, material, ...more });
  }
  cylinder(center: Vec3, radius: number, halfHeight: number, material: number, more: Partial<PieceGiven> = {}): void {
    this.pieces.push({ shape: 'cylinder', center, half: [radius, halfHeight, radius], material, ...more });
  }
  /** a lamp you can see: a ball that shines */
  ballLamp(center: Vec3, radius: number, radiance: Vec3, more: Partial<LampGiven> & { object?: number } = {}): number {
    const { object, ...lamp } = more;
    this.lamps.push({ kind: 'ball', center, radius, radiance, ...lamp });
    const index = this.lamps.length - 1;
    this.ball(center, radius, this.shell(), { lamp: index, object });
    return index;
  }
  /** a lamp you can see: a thin plate that shines along halfU × halfV */
  panelLamp(center: Vec3, halfU: Vec3, halfV: Vec3, radiance: Vec3, more: Partial<LampGiven> & { object?: number } = {}): number {
    const { object, ...lamp } = more;
    this.lamps.push({ kind: 'panel', center, halfU, halfV, radiance, ...lamp });
    const index = this.lamps.length - 1;
    const u = Math.hypot(...halfU);
    const v = Math.hypot(...halfV);
    const THIN = 0.01;
    const normal = normalize([
      halfU[1] * halfV[2] - halfU[2] * halfV[1],
      halfU[2] * halfV[0] - halfU[0] * halfV[2],
      halfU[0] * halfV[1] - halfU[1] * halfV[0],
    ]);
    const turn = turnFromAxes(normalize(halfU), normal);
    const middle: Vec3 = [center[0] - normal[0] * THIN, center[1] - normal[1] * THIN, center[2] - normal[2] * THIN];
    this.pieces.push({ shape: 'box', center: middle, half: [u, THIN, v], turn, material: this.shell(), lamp: index, object });
    return index;
  }
  sun(toward: Vec3, radiance: Vec3): number {
    this.lamps.push({ kind: 'sun', toward: normalize(toward), radiance });
    return this.lamps.length - 1;
  }
  private shellIndex = -1;
  private shell(): number {
    if (this.shellIndex < 0) this.shellIndex = this.material({ matte: [0.02, 0.02, 0.02], roughness: 1 });
    return this.shellIndex;
  }
}

/** the turn that takes x to `x` and y to `y` */
export function turnFromAxes(x: Vec3, y: Vec3): Quat {
  const z: Vec3 = [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]];
  const trace = x[0] + y[1] + z[2];
  if (trace > 0) {
    const s = Math.sqrt(trace + 1) * 2;
    return [(y[2] - z[1]) / s, (z[0] - x[2]) / s, (x[1] - y[0]) / s, s / 4];
  }
  if (x[0] > y[1] && x[0] > z[2]) {
    const s = Math.sqrt(1 + x[0] - y[1] - z[2]) * 2;
    return [s / 4, (y[0] + x[1]) / s, (z[0] + x[2]) / s, (y[2] - z[1]) / s];
  }
  if (y[1] > z[2]) {
    const s = Math.sqrt(1 + y[1] - x[0] - z[2]) * 2;
    return [(y[0] + x[1]) / s, s / 4, (z[1] + y[2]) / s, (z[0] - x[2]) / s];
  }
  const s = Math.sqrt(1 + z[2] - x[0] - y[1]) * 2;
  return [(z[0] + x[2]) / s, (z[1] + y[2]) / s, s / 4, (x[1] - y[0]) / s];
}

/** a piece moved by its object */
export function placed(piece: PieceGiven, at: Place): PieceGiven {
  return {
    ...piece,
    center: add(at.position, turnVec(at.turn, piece.center)),
    turn: turnBoth(at.turn, piece.turn ?? NO_TURN),
  };
}
