// What a scene says of itself beyond its pieces: where its rooms are, how its floors are cut, where a visit starts. Shaders get it as constants.
import type { V3 } from '../math';
export type Room = [number, number, number, number, number, number];
export interface Place {
  name: string; title: string;
  /** indoors: boxes (x0 y0 z0 x1 y1 z1) that rain and wind do not reach; what the section hides goes with the rooms it cuts */ rooms: Room[];
  /** the floor of what grows out of doors above a room (a section under it hides it) */ openFloor: number;
  /** the middle of what the sun's map of leaves covers, and how wide */ sunAt: V3; sunSize: number;
  start: { hour: number; look: V3; yaw: number; pitch: number; dist: number; level: number; wet: number; rain: number; steam: number };
  /** each level's floor, for what is put where the pointer is */ floors: number[]; pan: [number, number];
  /** the tour's legs */ legs: { name: string; look: V3; dist: number; level: number }[];
  /** lamp groups with a switch of their own; and the scene in the dark */ groups: string[]; quiet: { hour: number; groups: Record<string, number> };
  /** how much of the wind reaches what hangs */ still?: number;
  fires: V3[]; smokes: V3[]; judged: [string, Room][];
}
const f = (x: number) => x.toFixed(4);
export const placeCode = (p: Place) => `
const OPEN_FLOOR = ${f(p.openFloor)}; const SUN_AT = vec3f(${p.sunAt.map(f).join(', ')}); const SUN_WIDE = ${f(p.sunSize)};
const ROOMS = ${p.rooms.length}u; const ROOM_LO = array<vec3f, ${p.rooms.length}>(${p.rooms.map(r => `vec3f(${r.slice(0, 3).map(f).join(', ')})`).join(', ')}); const ROOM_HI = array<vec3f, ${p.rooms.length}>(${p.rooms.map(r => `vec3f(${r.slice(3).map(f).join(', ')})`).join(', ')});
/** under the open sky? */
fn openAir(x: vec3f) -> bool { for (var i = 0u; i < ROOMS; i++) { if (all(x > ROOM_LO[i]) && all(x < ROOM_HI[i])) { return false; } } return true; }
/** does the section (at height cut) hide what is at x: over a room that it cuts? */
fn overCut(x: vec3f, cut: f32) -> bool { for (var i = 0u; i < ROOMS; i++) { if (ROOM_HI[i].y + 0.4 > cut && all(x.xz > ROOM_LO[i].xz - 0.3) && all(x.xz < ROOM_HI[i].xz + 0.3)) { return true; } } return false; }
`;
