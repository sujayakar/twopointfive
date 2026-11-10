import type { RoomSpec } from '../world/room';
import { colorBox, furnace, plates, shapes } from './simple';
import { levelsRoom } from './levels';
import { smokeWindow } from './air';
import { thinRoom } from './thin';
import { terrace } from './terrace';
import { ladder, stoneYard } from './stones';
import { door, fence, lampHall, stripFloor, turntable } from './lit';

export const ROOMS: RoomSpec[] = [shapes, furnace, colorBox, plates, lampHall, fence, door, stripFloor, turntable, levelsRoom, stoneYard, ladder, smokeWindow, thinRoom, terrace];
export const roomNamed = (id: string): RoomSpec => ROOMS.find((r) => r.id === id) ?? ROOMS[0]!;
