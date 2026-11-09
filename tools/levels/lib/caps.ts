// WHAT A LEVEL MAY HAVE AT MOST: the rows the game's own lists have room for (rust/sim: doors.rs, props.rs, world.rs, lights.rs, which hold themselves to these numbers
// in rust/sim/tests/level_caps.rs). A level file of more is refused (levelData.ts).
export const DOOR_CAP = 64;
/** loose things of furniture, and the boxes all of them are made of */
export const PROP_CAP = 128, PROP_PARTS = 512;
export const LIGHT_CAPS = { targets: 256, rows: 256, beacons: 32, cameras: 32, lasers: 32 } as const;
