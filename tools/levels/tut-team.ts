// THE SQUAD'S TUTORIAL MISSION, ON THE GROUND (levels/tut-team.json; `bun run tools/levels/export.ts tut-team`): the tunnel station of tools/levels/tut-sky.ts,
// box for box, with the player standing on the road behind his own truck. What is different is the mission in the file, nothing on the floor:
//   · HE IS ONE OF FIVE. He starts third in the file behind the north pickup's tail, in the truck's shadow from all three posts of the roadblock.
//   · ONE ORDER, ONE WORD. The squad's key tapped on the ground, on a door and on himself, and the go key. No ring, no wall charge, no mains box, no goggles:
//     those are the next mission's. So the store keeps its lock and nobody is in it, and the bunkhouse is a room taken by its door with the lamps on.
//   · THE BUNKHOUSE HOLDS TWO, not three: it is met as a plain open and clear here, where the mission from above met it in the dark with two of its three
//     already down in the yard.
// THE SENTENCE NOBODY COULD SAY OF ANOTHER LEVEL is the station's own (the road ends in a hole in the mountain with a gun truck's lamps in front of it, and the
// only way off the road is the station's gate). What this mission adds to it: the player is behind the truck with them when the first rounds come in.
// FIVE BEATS, one new thing at a time (game/rails.ts locks the rest; levels/tut-team.script.ts stages the road):
//   the road      pinned behind the two pickups. Zoom out until all four are in the picture; the squad throws its smoke; the squad's key on the yard moves
//                 them through the gate, and he goes with them.
//   the gate      the roadblock comes down the road for the gate, and five rifles hold it.
//   indoors       the squad's key on a door stacks them, the go key sends them in. Twice: the tea room, the kitchen.
//   follow me     the squad's key on himself calls them to him, and he leads them across the yard.
//   the last room the same two keys on the bunkhouse's door; then he calls them to him once more and walks them back to the trucks.
// Nobody of the squad is lost (rails.respawn). He can be: a checkpoint stands at each beat's first step.
import type { Vec3 } from '../../src/math/vec';
import type { LessonData, LevelScenarios, Rect, StructureData } from '../../src/scene/scenarios';
import { build as buildStation, SCENARIOS as STATION, MATERIALS as STATION_MATERIALS, SKY_ROOMS as R, YARD_IN, YARD_MARK, TEA_DOOR, PASS_DOOR, BUNK_DOOR, TRUCKS_MARK, ROAD_BACK } from './tut-sky';

export const NAME = 'tut-team';
export const MATERIALS = STATION_MATERIALS;
export const NOTES = 'The squad\'s tutorial mission on the ground: the tunnel station of tut-sky, box for box, with the player on the road behind his own truck and a mission of one order and one word. Regenerate: bun run tools/levels/export.ts tut-team';

const HALF = Math.PI / 2;
/** where he stands when the first rounds come in: third in the file behind the north pickup's tail, a step behind his second man (tut-sky's OPS) */
export const TEAM_SAM: Vec3 = [4.55, 0, 22.5];
// (the yard inside the gate, the three doors a stack forms on and the road by the trucks are the station's own marks: tools/levels/tut-sky.ts)
/** the road by the trucks as the squad is counted back on it: men who follow him stop a few steps behind, and the last of a file stands in the lane along
 *  the yard's wall (a file that stopped on the station's own line, half a step short, kept the last step open) */
const ROAD_HOME: Rect = { x0: 1.1, x1: 12.5, z0: 20.1, z1: 27.0 };
/** inside the gate, where the line that asks him through it points */
const GATE_MARK: Vec3 = [14.5, 0.02, 18.9];
/** the yard's east end, between the plough lorry and the bunkhouse's wall: where he leads them, short of the door */
const BUNK_SIDE: Rect = { x0: 24.6, x1: 28.9, z0: 12.0, z1: 19.7 }, YARD_EAST: Rect = { x0: 19.0, x1: 28.9, z0: 11.7, z1: 19.8 }, BUNK_MARK: Vec3 = [26.2, 0.02, 17.4];   // (a few steps short of the door, so that the squad's key on himself afterwards is on himself and not on the door beside him)

/** THE STATION AS THIS MISSION HOLDS IT: five men in three rooms. The store is no room of the fight (locked, empty, and no order here opens it) */
function structure(): StructureData {
  const S = structuredClone(STATION.roomClear!) as StructureData;
  S.maxMen = 5; delete S.wallCharges;
  S.team = { ...S.team!, sam: TEAM_SAM, samYaw: HALF };
  S.rooms = S.rooms.filter(r => r.id !== 'store');
  for (const r of S.rooms) {
    r.doors = r.doors.filter(d => d.id !== 'store');
    if (r.id === 'bunk') { r.garrison = 2; r.marks = r.marks!.slice(0, 2); }
  }
  S.squads = S.squads!.map(q => q.id === 'lines' ? { ...q, rooms: ['bunk'] } : q);
  return S;
}

/** THE STEPS. A line is a verb and a key. One new action a step; done by the deed. What he learnt alone is his from the first frame */
const LESSON: LessonData = {
  title: 'Lead the squad',
  // (`rows`: the interaction key is the next mission's, and with a list no row is drawn while that key is locked. Without one the corner of the screen
  //  offered "push it shut" at every door for a key that did nothing)
  // (`orders`: the squad's key gives these and nothing else. Without the list a tap on the mains box by the kitchen's door sends a man to throw it, and
  //  the station is dark for the rest of a mission that has no goggles)
  rails: { allow: ['move', 'sprint', 'crouch', 'wall', 'aim', 'fire', 'alt', 'reload', 'slot:rifle', 'camera'], respawn: 5, lasers: false, orders: ['move', 'stack', 'go', 'fallIn'], rows: ['push', 'pull'] },
  steps: [
    // ---- the road ----
    { id: 'zoom', line: 'Hold [Ctrl] and scroll to zoom out until you can see all four', allow: ['zoom'], cue: 'ambush', checkpoint: 'the road', done: { is: 'view', zoomed: 1.2 } },
    { id: 'smoke', line: 'They\'re throwing smoke so the roadblock can\'t see them move', cue: 'smoke', done: { is: 'cue', name: 'smoke' } },
    { id: 'move', line: 'Press [B] on the yard to move them in', after: { line: 'They\'re using the smoke to get to the yard' }, allow: ['order'], at: YARD_MARK, done: { is: 'squad', area: YARD_IN }, hold: 0.6 },
    { id: 'with', line: 'Go through the gate with them', at: GATE_MARK, done: { is: 'in', area: YARD_IN } },
    // ---- the gate ----
    { id: 'gate', line: 'Hold the gate with them. The enemies from the roadblock are coming', cue: 'push', checkpoint: 'the gate', at: [14.5, 0.02, 19.9], done: { is: 'cue', name: 'roadblock' } },
    // ---- indoors ----
    { id: 'stack', line: 'Press [B] on the door to stack them up', after: { line: 'They\'re stacking up on the door' }, explain: { quiet: true, title: 'Doors', text: 'The way into a room is usually its door. Point at a door and press [B] to stack the squad up beside it.\nThey\'ll wait there until you say go.' }, cue: 'indoors', checkpoint: 'the yard', at: TEA_DOOR, done: { is: 'any', of: [{ is: 'stackOn', door: 'tea', set: true }, { is: 'taken', room: 'tea' }, { is: 'squad', area: R.tea.bounds, n: 3 }] } },
    { id: 'go', line: 'Press [F] to send them in', after: { word: 'They\'re going in' }, allow: ['word'], at: TEA_DOOR, done: { is: 'any', of: [{ is: 'taken', room: 'tea' }, { is: 'all', of: [{ is: 'word' }, { is: 'squad', area: R.tea.bounds, n: 3 }] }] }, hold: 1.5 },
    { id: 'kitchen', line: 'Press [B] on the kitchen door, then [F]', after: { line: 'Press [F] to send them in', word: 'They\'re taking the kitchen' }, checkpoint: 'the tea room', at: PASS_DOOR, done: { is: 'any', of: [{ is: 'taken', room: 'kitchen' }, { is: 'all', of: [{ is: 'word' }, { is: 'squad', area: R.kitchen.bounds, n: 3 }] }] }, hold: 1.5 },
    // ---- follow me ----
    { id: 'fallin', line: 'Point at yourself and press [B] to call them to you', after: { line: 'They\'re falling in behind you' }, explain: { quiet: true, title: 'Follow me', text: 'Point at yourself and press [B], and the squad will fall in behind you.\nThey\'ll follow you until you give them another order.' }, cue: 'house', checkpoint: 'the kitchen', done: { is: 'following' }, hold: 0.6 },
    { id: 'lead', line: 'Lead them across the yard to the last building', at: BUNK_MARK, done: { is: 'all', of: [{ is: 'in', area: BUNK_SIDE }, { is: 'squad', area: YARD_EAST, n: 3 }] } },
    // ---- the last room ----
    { id: 'bunk', line: 'Press [B] on the last door, then [F]', after: { line: 'Press [F] to send them in', word: 'They\'re taking the last room' }, at: BUNK_DOOR, done: { is: 'any', of: [{ is: 'taken', room: 'bunk' }, { is: 'all', of: [{ is: 'word' }, { is: 'squad', area: R.bunk.bounds, n: 3 }] }] }, hold: 1.5 },
    // (the trucks are out of the picture from the bunkhouse: the way back is the order he has just learnt, given once more, and his own feet)
    { id: 'out', line: 'Call them to you again and lead them back to the trucks', cue: 'out', at: TRUCKS_MARK, done: { is: 'all', of: [{ is: 'in', area: ROAD_BACK }, { is: 'squad', area: ROAD_HOME, n: 3 }] }, hold: 1 },
  ],
};

export function build() {
  const out = buildStation();
  const scenarios: LevelScenarios = { ...STATION, roomClear: structure(), lesson: LESSON };
  return { ...out, level: { ...out.level, playerSpawn: TEAM_SAM }, scenarios };
}
