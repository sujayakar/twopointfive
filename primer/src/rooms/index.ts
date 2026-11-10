import type { RoomSpec } from '../world/room';
import { colorBox, furnace, plates, shapes } from './simple';
import { door, fence, lampHall, stripFloor, turntable } from './lit';

export const ROOMS: RoomSpec[] = [shapes, furnace, colorBox, plates, lampHall, fence, door, stripFloor, turntable];
export const roomNamed = (id: string): RoomSpec => ROOMS.find((r) => r.id === id) ?? ROOMS[0]!;
