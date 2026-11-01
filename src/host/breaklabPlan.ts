// THE BREAK LAB'S FLOOR, AS A TABLE — what stands where on levels/break-lab.json (tools/levels/break-lab.ts lays the floor out from this) and what the lab's
// panel says of each (src/host/breaklab.ts: a button an exhibit that walks you to it, and one that sets its burst off for you). Four columns, one a material,
// west to east: steel, stone, wood, soft stuff; three rows a column, north to south: small things, a heavy thing, a thing that stands and takes the
// mark (the soft column's first is a hut instead: a wall and a door for the charges). Every exhibit is eight metres or more from the next: a grenade's
// outermost ring is 6.8 m, so one grenade is one exhibit's. Data only: no game module is imported here.
export type LabStuff = 'steel' | 'stone' | 'wood' | 'soft' | 'walls';
export type LabBurst = 'grenade' | 'door charge' | 'wall charge';
export interface LabExhibit {
  id: string; stuff: LabStuff; /** the button's word */ name: string;
  /** its middle on the floor */ x: number; z: number;
  /** where its burst lies (m), and which burst */ hit: [number, number]; burst: LabBurst;
  /** where to stand for it, when the usual place (below) is taken by a neighbour */ stand?: [number, number];
  /** what to look for, in a line or two (the panel's text once you are there) */ look: string;
  /** what its burst must do (tools/qa/breaklab-check.ts): at least this many of its own boxes gone (0 = none may go), at least this many things marked where
   *  they stand, a wall holed, a door off. Whatever it wants, no box nearer another exhibit may go. */
  want: { lost?: number; marked?: number; wall?: boolean; door?: boolean };
}
export const LAB_LEVEL = 'break-lab.json';
/** the columns' x and the rows' z */
export const LAB_COLS = { steel: 5.5, stone: 15, wood: 24.5, soft: 34 } as const, LAB_ROWS = [6, 13.5, 21] as const;
const C = LAB_COLS, R = LAB_ROWS;
export const LAB_EXHIBITS: LabExhibit[] = [
  { id: 'steel-small', stuff: 'steel', name: 'trunk and drums', x: C.steel, z: R[0], hit: [C.steel, R[0] + 0.75], burst: 'grenade', want: { lost: 3 },
    look: 'Steel folds. The trunk and the drums come off as bent plates, never as square blocks.' },
  { id: 'steel-heavy', stuff: 'steel', name: 'pickup', x: C.steel, z: R[1], hit: [C.steel, R[1] + 2.3], stand: [C.steel - 3.2, R[1] + 3.2], burst: 'grenade', want: { lost: 4, marked: 1 },
    look: 'A grenade a few steps from a pickup. Its wheels and fittings are torn off, and the body stands with dents pushed into its near side. Then click a grenade against the body: the whole truck comes apart as folded plates.' },
  { id: 'steel-stands', stuff: 'steel', name: 'container', x: C.steel, z: R[2], hit: [C.steel + 1.5, R[2] + 2.6], burst: 'grenade', want: { lost: 0, marked: 1 },
    look: 'Too big for a grenade to break. It stands, and the corner by the blast is dented in.' },
  { id: 'stone-small', stuff: 'stone', name: 'blocks and sandbags', x: C.stone, z: R[0], hit: [C.stone, R[0] + 0.8], burst: 'grenade', want: { lost: 3 },
    look: 'Stone crumbles. The blocks and the sandbags go to rough lumps: nothing bends and nothing splinters.' },
  { id: 'stone-heavy', stuff: 'stone', name: 'road barriers', x: C.stone, z: R[1], hit: [C.stone, R[1] + 0.55], burst: 'grenade', want: { lost: 2 },
    look: 'Stone breaks only when the blast lies against it. This grenade does, so the barriers go to lumps. Put it all back and click a grenade two steps away: they stand, with chips off the near corners.' },
  { id: 'stone-stands', stuff: 'stone', name: 'blast wall', x: C.stone, z: R[2], hit: [C.stone + 1.0, R[2] + 0.7], burst: 'grenade', want: { lost: 0, marked: 1 },
    look: 'The blast wall stands. Look for the chips knocked off the corners nearest the blast.' },
  { id: 'wood-small', stuff: 'wood', name: 'table and crates', x: C.wood, z: R[0], hit: [C.wood + 0.4, R[0] - 0.8], burst: 'grenade', want: { lost: 5 },
    look: 'Wood splinters. The table and the crates snap into long boards with broken ends.' },
  { id: 'wood-heavy', stuff: 'wood', name: 'market stall', x: C.wood, z: R[1], hit: [C.wood - 1.6, R[1]], burst: 'grenade', want: { lost: 8 },
    look: 'The stall\'s posts and counter snap into splintered lengths and its awning is torn off. Put it all back and click a grenade a few steps away: the goods fall off it and the counter stands, marked on its near side.' },
  { id: 'wood-stands', stuff: 'wood', name: 'rope bed and wood pile', x: C.wood, z: R[2], hit: [C.wood, R[2] + 0.9], burst: 'grenade', want: { lost: 5 },
    look: 'The rope bed\'s frame snaps and the wood pile comes apart as split logs.' },
  { id: 'hut-wall', stuff: 'walls', name: 'hut wall', x: C.soft, z: R[0], hit: [35.2, 8.7], burst: 'wall charge', want: { wall: true },
    look: 'A wall charge on the hut\'s wall. The wall is holed from the floor up where the charge sits. Inside, the things by the hole break and the things across the room stand.' },
  { id: 'hut-door', stuff: 'walls', name: 'hut door', x: C.soft, z: R[0], hit: [32.5, 8.7], burst: 'door charge', want: { door: true },
    look: 'A door charge on the hut\'s door. The leaf comes off its hinges and is thrown into the room.' },
  { id: 'soft-small', stuff: 'soft', name: 'sacks', x: C.soft, z: R[1], hit: [C.soft, R[1] - 0.9], burst: 'grenade', want: { lost: 6 },
    look: 'Soft stuff is torn and thrown. The sacks fly as limp pieces: nothing snaps and nothing folds.' },
  { id: 'soft-heavy', stuff: 'soft', name: 'cushions and a cot', x: C.soft, z: R[2], hit: [C.soft, R[2] + 0.2], burst: 'grenade', want: { lost: 5 },
    look: 'The cushions are torn and thrown, and the cot\'s steel frame folds under its blanket.' },
];
/** where to stand to watch one: south of it, where the camera looks from */
export const labStand = (e: LabExhibit): [number, number, number] => e.stand ? [e.stand[0], 0, e.stand[1]] : [e.x, 0, Math.min(25.5, Math.max(e.z, e.hit[1]) + 3.4)];
