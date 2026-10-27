// TUT-ROAD — the tutorial's first mission, "The pass road, alone": one man, his rifle and nothing else, walked up ONE ROAD. A fuel convoy was stopped on a
// mountain road in the snow, at night: its last truck stands across the road with its lamps on, its escort burns beside it, and its tanker has gone through
// the road post's corner and the rail. The road is cut into the mountain, so there is rock on one side of it and the drop on the other, and the only way
// along it is forward: the truck, the post's wall and its gateway, the hut whose door is locked, the hut's side door behind the tanker, the roadblock's
// sandbags under a machine gun, the roadblock's wall, the nest (the convoy's radio is in the green trunk behind the gun), the upper gate.
//
// THE SENTENCE NOBODY COULD SAY OF ANOTHER LEVEL: a mountain road with one way along it, where every wreck and wall on the way up is the cover for the next key.
// WHAT IT TEACHES, and nothing else: his body (walk, sprint, crouch, a wall and the lean round its end) and his gun (bring it up, aim, fire, full auto, a
// door shot off its hinges, the reload, a man shot through a plank wall). No grenade, no charge, no squad, nothing to sneak past.
// THE HAND PLACEMENT: the fights are the level's script (levels/tut-road.script.ts). Its six men are held on their marks until their beat, so the first of
// every list here is the only deal there is. The sentry stands in the gateway's line and nowhere near cover; the gun is in the far corner under the rock
// and everything it can reach from there is either hosed (the open snow before it) or swept at chest height (the lane behind the sandbags); the last
// three wait outside the upper gate, out of every line through it, and come in by it.
// THE CAMERA looks from the south: the rock is the far side of every picture and the drop is the near one, so no wall ever stands between the view and
// the man. Every wall he takes is taken from the side the view sees.
//
//        x 0     5       11        17      22.4     27.4  29.8  32      37   40
//   z 0   rock  rock  rock  rock  rock  rock  rock  rock  rock  rock  rock  rock
//   z 9.5 ========================+--hut--+============== nest =========+
//          cab|                  w| stove |               ▒ gun radio ▒ |  lane
//          VAN| →sprint→   WALL  ‖D      ¦m|   (the gun's open snow) ▒▒▒▒     g← the last three
//          tail            gate  ‖+--d----+wind                             g
//   z 15.5     ·                 sentry   |wall▒▒▒sandbags▒▒▒▒████wall████  ·
//         START  wreck(fire)      drum   T|  → crouched, behind them →  lean↗  pickup, lamps
//   z 19.7 ----rail----------rail------  T  ------rail-----------------rail----
//   z 21    the drop: a berm, pines below     (the tanker's cab is through the rail, burning)
//
//   bun run tools/levels/export.ts tut-road [--check]      → levels/tut-road.json
//   bun run tools/qa/lesson-check.ts tut-road              the lesson block against the built floor
//   bun run tools/qa/lesson-run.ts tut-road                the scripted player through every step to the debrief
//   bun run tools/qa/tut-road-check.ts                     the lesson played as said, and failed the two ways its lines warn of
//   page: ?levelFile=tut-road.json&quiet
import type { Vec3 } from '../../src/math/vec';
import type { Level, LevelDoorDef, PatrolRoute } from '../../src/scene/level';
import type { LevelScenarios, LessonData, LessonStep, QuietData, Rect, IssueData } from '../../src/scene/scenarios';
import type { SurfacesData } from '../../src/scene/levelData';
import { officeKit, normalize3 as norm, type GeneratorTrace } from '../../src/scene/officeGen';
import { gwotKit, gwotOfOffice, GWOT_PALETTE, GWOT_MATERIALS, GW_SHELL } from '../../src/scene/gwotKit';
import { SNOW_PALETTE, SNOW_FINISH, SNOW_MATERIALS, type SnowMat } from '../../src/scene/gwotPalette';
import { WORLD } from '../../src/scene/world';
import { BoxFlag } from '../../src/scene/boxes';
import { findCarLamps } from '../../src/game/carlamps';
import { LAST_OUT, LAST_MEN, HUT_NOOK, HUT_MARK, TRUCK_FLANK, HUT_WALL, HUT_END, BAGS_TOP, WALL_FACE } from '../../levels/tut-road.script';

export const NAME = 'tut-road';
export const NOTES = 'The tutorial\'s first mission: one man and his rifle on a snowed-in mountain road at night, walked along one line (the stopped truck, the road post\'s wall and gateway, the hut, the sandbags under a machine gun, the roadblock\'s wall, the upper gate). Its fights are staged by levels/tut-road.script.ts. The snow is in the file. Regenerate: bun run tools/levels/export.ts tut-road';
export const MATERIALS = { ...GWOT_MATERIALS, ...SNOW_MATERIALS };

const HALF = Math.PI / 2;
/** THE ROAD between the rock's foot and the rail: the only floor there is */
export const ROAD = { z0: 9.5, z1: 19.7 };
/** the road post: a wall across the road at each end of it, the lower one with the gateway he leans into, the upper one with the gate the mission ends at */
export const POST = { x0: 11, x1: 37 };
/** the lower wall's gateway, a truck wide: its near jamb is the corner he leans round */
export const GATEWAY = { a: 13.65, b: 16.85 };
/** the hut, against the rock: its wooden door on the road's side (locked), its side door behind the tanker (no leaf) */
export const HUT: Rect = { x0: 17, x1: 22.4, z0: 9.7, z1: 13.8 };
export const HUT_DOOR_Z = 12.75, SIDE_DOOR_X = 21.25;
/** the line of low cover and the wall after it: the hut's wind wall, the roadblock's sandbags, the roadblock's wall, one after the other on one line */
export const COVER_Z = 15.5, BAGS = { x0: 22.5, x1: 27.4 }, RB_WALL = { x0: 27.4, x1: 29.8 };
/** the upper gate: a man's width, on a walk-grid cell */
export const GATE_Z = 14.25;

// ---- the marks the level, its script and its scripted player all read ----
/** where he starts: on the road at the truck's open back doors, in the light of its load bay, facing up the truck. Well inside the world's edge, and the
 *  first walk is UP the screen (a mark toward the camera from him sits under the big line) */
export const START: Vec3 = [4.45, 0, 18.3];
/** the convoy's last truck, across the road with its nose to the rock: where it stands, and behind its cab (step 'move') */
export const VAN_AT = { x: 4.6, z: 14.4, yaw: HALF + 0.12 };
export const VAN: Vec3 = [3.1, 0, 12.6];
/** the post's wall beside the gateway (the sprint's end, the wall he takes): the rock's side of the gateway. A rifle is carried on the right, so with his
 *  back on this wall the gateway is on his left and the gun comes out into it; from the far jamb it would be behind the wall and stay at the low ready */
export const WALL: Vec3 = [10.35, 0, 11.9];
/** the gateway's near corner: the end of that wall, which he leans round */
export const CORNER: Vec3 = [10.9, 1.2, GATEWAY.a];
/** the sentry, in the gateway's line, by his fire */
export const SENTRY: Vec3 = [15.6, 0, 15.1];
/** where he stands square on to the hut's door to shoot it out (a round that clips a jamb is not counted) */
export const DOOR_FROM: Vec3 = [15.2, 0, HUT_DOOR_Z];
/** the hut's plank wall (its back room is boarded off: a rifle round goes through it), and the man behind it. The rounds for the door go on through the
 *  leaf and the way through at the boards' end, into the back room's other corner: he stands two long paces off their line */
export const PLANKS = { x: 20.2, z0: 9.82, z1: 12.2 }, HUT_MAN: Vec3 = HUT_MARK;
/** the stub of wall in the hut's back room that he waits behind until his line: it stops every round that comes in by the door, its leaf, the boards or the window */
export const NOOK_WALL = { x: 21.1, z1: 11.3 };
/** the trunk the radio is in: in the gun's nest, under the rock (the mission's one thing to take, and the last thing but the walk out) */
export const TRUNK: Vec3 = [32.7, 0, 10.45];
/** the nest's floor: between its two walls of sandbags, the rock and the upper wall */
export const NEST: Rect = { x0: 30.7, x1: 36.6, z0: 9.7, z1: 12.2 };
/** outside the hut's side door, in the lee of its wind wall: out of everything the gun can reach (steps 'arch', 'crouch') */
export const JAMB: Vec3 = [21.5, 0, 14.75];
/** the lane behind the sandbags, and its far end (step 'low') */
export const LANE_Z = 16.7, BAGS_END: Vec3 = [26.7, 0, LANE_Z];
/** against the roadblock's wall (step 'cover'), and the end of it he leans round */
export const COVER: Vec3 = [28.6, 0, 16.2], CORNER_E: Vec3 = [RB_WALL.x1, 1.2, COVER_Z + 0.1];
/** the man on the gun, in his nest under the rock: more behind the roadblock's wall than beside it, as a man leaning round its end sees him (a man
 *  who can already be seen from the wall has nobody to lean out for: game/wallsnap.ts wsSeenDeg) */
export const GUNNER: Vec3 = [31.6, 0, 11.2];
/** THE LAST THREE wait outside the upper gate and come in by it: their marks are the script's (levels/tut-road.script.ts LAST_OUT, LAST_MEN) */
export { LAST_OUT, LAST_MEN };
/** the way out: the upper gate, and on up the road */
export const EXIT = { name: 'the upper gate', door: 'gate', x: POST.x1 + 0.8, z: GATE_Z, out: [1, 0] as [number, number] };

/** THE TUTORIAL FRAME'S KEYS, a step each: `allow` turns actions on for the rest of the mission (from the first step he can walk: `rails.allow`), `cue` goes to the level's script as the step comes up ('lesson:<cue>'), `checkpoint` names the one save of a beat. Two checkpoints,
 *  each where he has just finished something and nobody alive can reach him: the hut (the sentry and the man behind the boards are down, the door is
 *  out) and the gun (the nest is out). THE PISTOL comes on with the gunner's line and no line names it: 91 rifle rounds can be emptied on full auto, and a
 *  man with a dry rifle at the last fight has its digit lit on his kit strip */
export const FRAME: Record<string, { allow?: string[]; deny?: string[]; cue?: string; checkpoint?: string }> = {
  move: { cue: 'road' }, turn: { allow: ['camera', 'zoom'] }, ready: { allow: ['slot:rifle'] }, sprint: { allow: ['sprint'] }, wall: { allow: ['wall'], cue: 'cease' }, peek: { allow: ['aim'] }, fire: { allow: ['fire'] },
  auto: { allow: ['mode'] }, door: { allow: ['reload'] }, reload: { cue: 'way' }, thin: { cue: 'hut' }, take: { allow: ['use'] },
  arch: { cue: 'gun', checkpoint: 'the hut' }, crouch: { allow: ['crouch'], cue: 'bags' }, cover: { cue: 'ease' }, gunner: { allow: ['slot:pistol'] },
  watch: { cue: 'gate', checkpoint: 'the gun' }, radio: { cue: 'open' },
};

/** HIS KIT (game/kit.ts kitIssue): the rifle and its magazines. Nothing to throw, nothing to set, nothing under the barrel: every one of those is a later
 *  mission's first line */
export const ISSUE: IssueData = { members: { '0': { weapon: 'rifle', smoke: 0, bangs: 0, frags: 0, shells: 0, charges: 0 } } };

export function build(): { level: Level; scenarios: LevelScenarios; groups: string[]; surfaces: SurfacesData } {
  const trace: GeneratorTrace = { groups: [] };
  const K = officeKit(trace), G = gwotKit(gwotOfOffice(K)), P = GWOT_PALETTE, H = WORLD.ceilingY, M = G.M, t = GW_SHELL / 2;
  const { grp, add, slab, doorFrame, lights, switchables } = K;
  const DECAL = BoxFlag.NoShadow;
  type Mat = { albedo: Vec3; rough: number };
  const S = Object.fromEntries((Object.keys(SNOW_PALETTE) as SnowMat[]).map(n => [n, { albedo: SNOW_PALETTE[n], rough: SNOW_FINISH[n].rough }])) as Record<SnowMat, Mat>;
  /** a vehicle's own frame: local +X its nose */
  const at = (v: { x: number; z: number; yaw: number }, lx: number, y: number, lz: number): Vec3 => { const cs = Math.cos(v.yaw), sn = Math.sin(v.yaw); return [v.x + cs * lx + sn * lz, y, v.z - sn * lx + cs * lz]; };
  const doors: LevelDoorDef[] = [];
  const zones: SurfacesData['zones'] = [];

  // ================================================================ SNOW: the pieces ====
  // (the pass's and the tunnel station's own, tools/levels/pass.ts: ground and cleared earth are thin solid slabs; paths, ruts and slush are decals; a
  //  DRIFT tops out under 0.2 m and is waded; a BANK stands over it and stops feet; a CAP is a decal on the top of a thing)
  const lay = (x0: number, z0: number, x1: number, z1: number, m: Mat, y = 0.004) => add({ c: [(x0 + x1) / 2, y / 2, (z0 + z1) / 2], h: [Math.abs(x1 - x0) / 2, y / 2, Math.abs(z1 - z0) / 2], ...m });
  /** ground cleared to earth inside a ring of grey slush; its footsteps are earth's */
  const cleared = (id: string, x0: number, z0: number, x1: number, z1: number, ring = 0.4) => {
    lay(x0 - ring, z0 - ring, x1 + ring, z1 + ring, S.snowOld, 0.004); lay(x0, z0, x1, z1, M.dust, 0.006);
    zones.push({ id, mat: 'asphalt', x0: x0 - ring, z0: z0 - ring, x1: x1 + ring, z1: z1 + ring });
  };
  /** ground melted round a fire: the same two layers, each four slabs through one centre (an octagon to the eye), a little unequal and off the walls' lines */
  const melted = (id: string, x: number, z: number, r: number, ring = 0.35) => {
    const T = Math.tan(Math.PI / 8), oct = (R: number, m: Mat, y: number, turn: number, f: number[]) => { for (let k = 0; k < 4; k++) add({ c: [x, y / 2, z], h: [R * f[k], y / 2, R * T], yaw: turn + k * Math.PI / 4, ...m }); };
    oct(r + ring, S.snowOld, 0.004, 0.2, [1, 0.92, 1.06, 0.9]); oct(r, M.dust, 0.006, 0.5, [0.94, 1.05, 0.9, 1]);
    zones.push({ id, mat: 'asphalt', x0: x - r - ring * 0.6, z0: z - r - ring * 0.6, x1: x + r + ring * 0.6, z1: z + r + ring * 0.6 });
  };
  /** DECAL: a flat mark at the floor */
  const flat = (x: number, z: number, w: number, d: number, m: Mat, yaw = 0, y = 0.012) => add({ c: [x, y, z], h: [w / 2, 0.003, d / 2], yaw, ...m, flags: DECAL });
  let pathN = 0;
  /** DECAL: a trodden path through the points; its footsteps are the hard ground's */
  const path = (pts: [number, number][], w = 0.8, m: Mat = S.snowPacked) => {
    for (let i = 0; i + 1 < pts.length; i++) {
      const [x0, z0] = pts[i], [x1, z1] = pts[i + 1], L = Math.hypot(x1 - x0, z1 - z0), yaw = Math.atan2(-(z1 - z0), x1 - x0);
      flat((x0 + x1) / 2, (z0 + z1) / 2, L + (i + 2 < pts.length ? w * 0.5 : 0), w, m, yaw, 0.016);
      for (let s = 0; s <= L; s += 0.5) { const x = x0 + (x1 - x0) * s / L, z = z0 + (z1 - z0) * s / L; zones.push({ id: `path_${pathN++}`, mat: 'asphalt', x0: x - w / 2, z0: z - w / 2, x1: x + w / 2, z1: z + w / 2 }); }
    }
  };
  /** DECAL: two wheel ruts through the points (an end on the map's edge stops there) */
  const ruts = (pts: [number, number][], track = 1.96) => {
    for (let i = 0; i + 1 < pts.length; i++) {
      const [x0, z0] = pts[i], [x1, z1] = pts[i + 1], L = Math.hypot(x1 - x0, z1 - z0), yaw = Math.atan2(-(z1 - z0), x1 - x0), b = G.piece((x0 + x1) / 2, (z0 + z1) / 2, yaw);
      const lap = (x: number) => x <= 0 || x >= 40 ? 0 : 0.12, e0 = lap(x0), e1 = lap(x1);
      for (const s of [-track / 2, track / 2]) b((e1 - e0) / 2, s, 0.009, 0.015, L / 2 + (e0 + e1) / 2, 0.17, S.snowOld, { flags: DECAL });
    }
  };
  /** a DRIFT (waded: nothing of it over 0.19 m). `yaw` turns local +Z to point AWAY from the thing it lies against (0 = toward +z, π/2 = +x, π = −z, −π/2 = −x) */
  const drift = (x: number, z: number, len: number, depth: number, yaw = 0, top = 0.17) => {
    const b = G.piece(x, z, yaw);
    b(0, 0, 0, top * 0.5, len / 2, depth / 2, S.snow); b(len * 0.03, -depth * 0.22, 0, top, len * 0.41, depth * 0.28, S.snow);
  };
  /** a BANK (stops feet). Same frame as drift(); three steps down from `top` */
  const bank = (x: number, z: number, len: number, depth: number, yaw = 0, top = 0.42) => {
    const b = G.piece(x, z, yaw);
    b(0, 0, 0, 0.16, len / 2, depth / 2, S.snow); b(-len * 0.02, -depth * 0.15, 0, top * 0.62, len * 0.47, depth * 0.35, S.snow); b(len * 0.03, -depth * 0.31, 0, top, len * 0.42, depth * 0.19, S.snow);
  };
  /** DECAL: a cap of snow on top of a thing whose top is at `y` */
  const cap = (x: number, z: number, w: number, d: number, y: number, yaw = 0, th = 0.05) => G.piece(x, z, yaw)(0, 0, y, y + th, w / 2, d / 2, S.snow, { flags: DECAL });
  /** caps along a wall's coping (its top at 2.47 m), in lengths with wind-scoured gaps between them */
  const copingCaps = (axis: 'x' | 'z', line: number, a: number, b: number) => {
    const lens = [2.6, 1.4, 3.1, 1.8, 2.2, 1.2]; let u = a + 0.06, k = Math.round(a * 5 + line) % lens.length;
    while (u < b - 0.4) { const l = Math.min(lens[k++ % lens.length], b - 0.06 - u), m = u + l / 2; if (axis === 'x') cap(m, line, l, 0.2, 2.47, 0, 0.05 + (k % 3) * 0.012); else cap(line, m, 0.2, l, 2.47, 0, 0.05 + (k % 3) * 0.012); u += l + 0.16 + (k % 2) * 0.12; }
  };
  /** THE MOUNTAIN: a block of the cut rock, `h` tall, snow lying on its top (set in from its edges: the rock shows as a dark rim) */
  const rock = (x0: number, z0: number, w: number, d: number, h: number, yaw = 0) => {
    const ex = (Math.abs(w * Math.cos(yaw)) + Math.abs(d * Math.sin(yaw))) / 2 + 0.03, ez = (Math.abs(w * Math.sin(yaw)) + Math.abs(d * Math.cos(yaw))) / 2 + 0.03;
    const x = Math.min(WORLD.max[0] - ex, Math.max(WORLD.min[0] + ex, x0)), z = Math.min(WORLD.max[2] - ez, Math.max(WORLD.min[2] + ez, z0));
    add({ c: [x, h / 2, z], h: [w / 2, h / 2, d / 2], yaw, ...M.block, albedo: [0.2, 0.21, 0.24] });
    cap(x - 0.04, z - 0.03, Math.max(0.2, w - 0.3), Math.max(0.2, d - 0.26), h, yaw, 0.07);
  };
  /** a PINE on the slope below the road: a trunk and three tiers, each two boards crossed (from over it, where the camera is, a tier is a star and not a
   *  box), a dab of snow where each tier meets the trunk: the boughs' ends stay dark */
  const pine = (x: number, z: number, h: number, turn = 0) => {
    const green = { albedo: [0.05, 0.085, 0.07] as Vec3, rough: 1 };
    add({ c: [x, h * 0.2, z], h: [0.09, h * 0.2, 0.09], ...M.woodGrey });
    for (const [k, r] of [[0, 0.95], [1, 0.66], [2, 0.36]] as const) { const y0 = h * (0.26 + k * 0.24), y1 = y0 + h * 0.26, w = r * (0.5 + h * 0.16), yaw = turn + k * 0.4;
      for (const q of [0, Math.PI / 4]) add({ c: [x, (y0 + y1) / 2 - q * 0.02, z], h: [w * 0.85, (y1 - y0) / 2 - q * 0.02, w * 0.85], yaw: yaw + q, ...green });
      cap(x - 0.03, z - 0.02, w * 0.7, w * 0.6, y1, yaw + 0.3, 0.04); }
  };

  // ---- the ground: snow to the edge of the map; the road ploughed and driven; the dark below the rail ----
  grp('ground');
  slab(0, -0.25, 0, 40, 0, 28, S.snow.albedo, { rough: S.snow.rough });
  lay(0, ROAD.z0 + 1.0, 40, ROAD.z1 - 0.9, S.snowPacked);                                      // the road, as the plough and the convoy left it
  lay(0, 21.3, 40, 28, { albedo: [0.035, 0.045, 0.06], rough: 1 }, 0.003);                     // the slope below the road, in the mountain's own shadow
  grp('floors');
  G.ground(HUT.x0, HUT.z0, HUT.x1, HUT.z1, 'slab', 0.006);
  zones.push({ id: 'hut', mat: 'stone', x0: HUT.x0, z0: HUT.z0, x1: HUT.x1, z1: HUT.z1 });
  grp('ceiling');
  slab(HUT.x0 - 0.1, H, HUT.z0 - 0.1, HUT.x1 + 0.1, H + 0.2, HUT.z1 + 0.1, P.render, { flags: BoxFlag.NoPrimary, rough: 0.95 });

  // ---- THE MOUNTAIN: the cut the road runs under, higher behind; stones off it on the road's edge ----
  grp('rock');
  for (const [x, z, w, d, h, yaw] of [
    [2.2, 7.9, 4.6, 3.2, 3.6, 0.05], [6.9, 8.0, 5.0, 3.0, 4.3, -0.04], [12.0, 7.9, 5.4, 3.2, 3.8, 0.03], [16.6, 8.2, 4.0, 2.6, 4.6, 0],
    [19.9, 8.3, 3.4, 2.4, 4.9, 0], [23.4, 8.2, 3.8, 2.6, 4.2, 0], [27.6, 7.9, 4.8, 3.2, 4.7, 0.04], [32.4, 8.0, 5.0, 3.0, 3.9, -0.03], [37.4, 7.9, 5.2, 3.2, 4.5, 0.04],
    [3.0, 4.6, 6.0, 3.6, 5.6, -0.03], [9.2, 4.4, 6.6, 3.8, 6.4, 0.04], [15.8, 4.6, 6.8, 3.6, 5.9, -0.02], [22.6, 4.4, 7.0, 3.8, 6.8, 0.03], [29.6, 4.6, 7.2, 3.6, 6.1, -0.04], [36.6, 4.5, 6.8, 3.8, 6.6, 0.02],
    [4.4, 1.4, 8.8, 2.8, 7.6, 0.02], [13.4, 1.4, 9.2, 2.8, 7.75, -0.02], [22.8, 1.4, 9.6, 2.8, 7.5, 0.02], [32.4, 1.4, 9.8, 2.8, 7.85, -0.01], [38.9, 1.4, 2.2, 2.8, 7.4, 0],
  ] as const) rock(x, z, w, d, h, yaw);
  for (const [x, z, w, d, h, yaw] of [[1.2, 10.3, 0.8, 0.55, 0.6, 0.4], [7.6, 10.1, 0.6, 0.5, 0.45, -0.3], [13.4, 10.05, 0.7, 0.5, 0.5, 0.6], [25.2, 10.2, 0.9, 0.6, 0.7, 0.2], [29.4, 10.0, 0.6, 0.45, 0.4, 0.7], [36.2, 10.1, 0.7, 0.5, 0.5, -0.5]] as const) rock(x, z, w, d, h, yaw);

  // ---- THE DROP: the rail, the plough's berm past it, the pines on the slope, the poles the mains come up the road on ----
  grp('drop');
  const TANKER = { x: 19.1, z: 17.55, yaw: -HALF };                                             // (nose toward +z: over the edge)
  for (let x = 1.2; x < 40; x += 2.4) if (x < 17.4 || x > 20.9) add({ c: [x, 0.36, ROAD.z1 + 0.05], h: [0.05, 0.36, 0.05], ...M.galv });
  for (const [x0, x1, z, yaw] of [[0, 17.5, ROAD.z1, 0], [15.9, 17.7, ROAD.z1 + 0.34, -0.36], [20.7, 22.4, ROAD.z1 + 0.3, 0.34], [20.8, 40, ROAD.z1, 0]] as const) add({ c: [(x0 + x1) / 2, 0.6, z], h: [(x1 - x0) / 2, 0.11, 0.02], yaw, ...M.galv });   // (two lengths torn out and bent over the edge where the tanker went through)
  G.piece(8.6, 20.55, 0)(0, 0, 0, 0.34, 8.6, 0.5, S.snow); G.piece(30.6, 20.55, 0)(0, 0, 0, 0.34, 9.4, 0.5, S.snow);   // what the plough throws over the rail (no floor out there to be sent to)
  // (their heads are under the ceiling's height: they stand on the slope below the road, and nothing out here is a roof)
  for (const [x, z, h, turn] of [[2.6, 23.2, 2.3, 0.2], [6.8, 25.4, 2.7, 0.5], [10.9, 22.8, 2.0, 0.1], [14.6, 24.9, 2.6, 0.7], [23.2, 23.4, 2.2, 0.3], [26.9, 25.6, 2.75, 0.6], [31.2, 22.9, 2.3, 0.9], [35.4, 24.6, 2.6, 0.4], [38.6, 22.7, 1.9, 0.2], [18.4, 26.3, 2.75, 0.8]] as const) pine(x, z, h, turn);
  // snow lying on the slope in the lee of each stand of trees, and stones through it: the dark below the rail is a hillside, not a sheet
  for (const [x, z, w, d, yaw] of [[4.6, 24.2, 3.2, 1.6, 0.2], [12.6, 23.9, 2.6, 1.4, -0.3], [16.6, 26.6, 3.4, 1.2, 0.1], [21.2, 24.6, 2.2, 1.5, 0.4], [25.0, 23.2, 2.8, 1.1, -0.2], [29.4, 25.4, 3.0, 1.6, 0.3], [33.4, 23.6, 2.4, 1.3, -0.4], [37.2, 25.9, 2.8, 1.5, 0.2], [9.0, 26.6, 2.6, 1.2, -0.1]] as const) flat(x, z, w, d, S.snowOld, yaw, 0.01);
  for (const [x, z, w, d, h, yaw] of [[8.4, 23.4, 0.9, 0.6, 0.5, 0.4], [20.2, 26.0, 1.1, 0.8, 0.7, -0.3], [28.2, 23.6, 0.8, 0.6, 0.45, 0.7], [36.6, 23.2, 1.0, 0.7, 0.6, 0.2]] as const) rock(x, z, w, d, h, yaw);
  for (const x of [2.2, 13.0, 24.6, 35.4]) { add({ c: [x, 1.66, 20.3], h: [0.07, 1.66, 0.07], ...M.woodGrey }); add({ c: [x, 3.2, 20.3], h: [0.05, 0.05, 0.5], ...M.woodGrey }); }
  G.wire(2.2, 20.3, 13.0, 20.3, 3.22); G.wire(13.0, 20.3, 24.6, 20.3, 3.22); G.wire(24.6, 20.3, 35.4, 20.3, 3.22); G.wire(35.4, 20.3, 40, 20.3, 3.22); G.wire(13.0, 20.3, POST.x0, GATEWAY.b + 0.2, 2.9);

  // ---- THE ROAD POST: a wall across the road below and above, the hut against the rock between them ----
  grp('walls.post');
  G.yardWallZ(POST.x0, ROAD.z0 - 0.1, GATEWAY.a, 'block'); G.yardWallZ(POST.x0, GATEWAY.b, ROAD.z1 + 0.1, 'block');     // the lower wall: the gateway is a truck wide and has no leaf
  G.pier(POST.x0, GATEWAY.b + 0.1, 2.4, 'block');                                              // (a pier on the far jamb only: the corner he leans round is bare)
  // the gateway's pole, swung open and back against the wall on its far side: a counterweight, a red and white pole, its rest
  add({ c: [POST.x0 + 0.34, 0.55, GATEWAY.b + 0.2], h: [0.09, 0.55, 0.09], ...M.galv }); add({ c: [POST.x0 + 0.34, 1.08, GATEWAY.b + 1.5], h: [0.04, 0.04, 1.5], ...M.paintCream });
  for (const k of [0, 1, 2]) add({ c: [POST.x0 + 0.34, 1.08, GATEWAY.b + 0.6 + k * 0.9], h: [0.043, 0.043, 0.2], ...M.truckRed, flags: DECAL });
  const upper = [{ a: GATE_Z - 0.6, b: GATE_Z + 0.6, y0: 0, y1: 2.2 }];
  G.yardWallZ(POST.x1, ROAD.z0 - 0.1, ROAD.z1 + 0.1, 'block', upper);                          // the upper wall: the trucks' gate in it is shut, the man's gate beside it stands open
  G.gate(POST.x1 - 0.15, 17.2, 'z', 3.2, -1, 'paintGreen', 0); G.gate(POST.x1 + 0.15, 17.2, 'z', 3.2, 1, 'paintGreen', 0);   // (steel leaves on each face of the wall: nobody walks, sees or shoots through)
  doors.push({ name: 'gate', hinge: [POST.x1, GATE_Z + 0.58], closedDir: -HALF, width: 1.16, exterior: true, angle: -1.45, swing: -1, closer: false });   // (hung to open INTO the post: the lane outside is no room its men can shut, and the three who come in by it push it wider, not shut)
  // the lane the road leaves the map by: walled on both sides, so nobody walks round the post and nobody comes to it but by the gate
  G.yardWallX(11.4, POST.x1 + 0.12, 40, 'block'); G.yardWallX(17.3, POST.x1 + 0.12, 40, 'block');
  grp('walls.hut');
  // THE HUT. Its door on the road is wooden and locked, and it opens TOWARD the road: a sprint into a locked door is a kick, and a frame holds a boot from
  // the face its door opens toward (doors.ts DOORWAY.frameHolds), so the run a new player tries first stops on it and the trigger is the way through. A
  // window beside it, lit: plain glass on a sill, which a burst takes with the door
  G.wallX(HUT.z0, HUT.x0 - t, HUT.x1 + t, 'render');
  G.wallX(HUT.z1, HUT.x0 - t, HUT.x1 + t, 'render', [{ a: SIDE_DOOR_X - 0.6, b: SIDE_DOOR_X + 0.6 }]);          // its side door: a doorway, no leaf (the light of the room falls out of it onto the snow)
  G.wallZ(HUT.x0, HUT.z0 + t, HUT.z1 - t, 'render', [{ a: 10.0, b: 10.8, y0: 0.9, y1: 1.9, glass: 'whole' }, { a: HUT_DOOR_Z - 0.6, b: HUT_DOOR_Z + 0.6 }]);
  G.wallZ(HUT.x1, HUT.z0 + t, HUT.z1 - t, 'render');
  G.wallZ(NOOK_WALL.x, HUT.z0 + t, NOOK_WALL.z1, 'render');                                      // the back room's nook: the stove's old flue wall, and where its man waits
  doors.push({ name: 'hut', hinge: [HUT.x0, HUT_DOOR_Z - 0.58], closedDir: HALF, width: 1.16, exterior: true, locked: true, swing: 1, closer: false });
  G.dado('z', HUT.x0, 11.0, HUT_DOOR_Z - 0.6, -1, 'paintGreen');   // the roads department's green, either side of the door
  // the wind wall: the hut's end wall run on past its corner, to keep the pass's wind off the side door. It is what stands between that door and the gun
  G.yardWallZ(HUT.x1, HUT.z1 + t, COVER_Z, 'block');
  grp('walls.roadblock');
  G.sandbags((BAGS.x0 + BAGS.x1) / 2, COVER_Z, BAGS.x1 - BAGS.x0, 0, 6);                       // chest high: a crouched man behind them is out of the gun's sight
  G.yardWallX(COVER_Z, RB_WALL.x0, RB_WALL.x1, 'block');                                       // the roadblock's wall: over a man, bare at its far end
  add({ c: [28.6, 1.5, COVER_Z + t + 0.012], h: [0.42, 0.3, 0.008], ...M.paintBlue, flags: DECAL }); add({ c: [28.6, 1.5, COVER_Z + t + 0.022], h: [0.3, 0.06, 0.004], ...M.paintCream, flags: DECAL });   // the post's board on it

  // ---- THE CONVOY, as it was stopped ----
  grp('convoy.truck');
  // its last truck, across the road with its nose to the rock: his first cover, and what the sentry's rounds chew. Everything on it is on, as its driver
  // left it: two head lamps (on the rock), the tail lamps, the lamp in the load bay over its open back doors, which is the light he starts in. Each lens is
  // a glowing sliver a hair off the truck's own, so a round at a lamp meets the sliver first and puts it out (carlamps.ts finds the lenses here)
  const van0 = K.boxes.length; G.van(VAN_AT.x, VAN_AT.z, VAN_AT.yaw);
  { const lenses = findCarLamps(K.boxes.slice(van0)).map(l => ({ what: l.what, b: K.boxes[van0 + l.box] })), heads = lenses.filter(l => l.what === 'head lamp').map(l => l.b), tails = lenses.filter(l => l.what === 'tail lamp').map(l => l.b);
    if (heads.length !== 2 || tails.length !== 2) throw new Error(`tut-road: the truck's lamps were not found (${heads.length} head, ${tails.length} tail): the kit's lenses have moved`);
    const mid = [(heads[0].c[0] + heads[1].c[0]) / 2, (heads[0].c[2] + heads[1].c[2]) / 2], l = Math.hypot(mid[0] - VAN_AT.x, mid[1] - VAN_AT.z), nx = (mid[0] - VAN_AT.x) / l, nz = (mid[1] - VAN_AT.z) / l;
    heads.forEach((b, k) => {
      const fb = add({ c: [b.c[0] + nx * 0.04, b.c[1], b.c[2] + nz * 0.04], h: [0.008, b.h[1] + 0.006, b.h[2] + 0.006], yaw: b.yaw, ...M.plasticWhite, emissive: [34, 31, 24], flags: DECAL });
      lights.push({ kind: 'spot', pos: [b.c[0] + nx * 0.08, b.c[1], b.c[2] + nz * 0.08], dir: norm([nx, -0.1, nz]), color: [1.0, 0.93, 0.78], intensity: 110, range: 12, innerDeg: 12, outerDeg: 30, name: `truck_lamp_${k}`, group: 'exterior', enabled: true, volumetric: 0.35, fixtureBox: fb, radius: 0.06, mains: false });
    });
    tails.forEach((b, k) => {
      const fb = add({ c: [b.c[0] - nx * 0.03, b.c[1], b.c[2] - nz * 0.03], h: [0.006, b.h[1] + 0.006, b.h[2] + 0.006], yaw: b.yaw, albedo: [0.5, 0.05, 0.04], rough: 0.4, emissive: [7, 0.4, 0.25], flags: DECAL });
      lights.push({ kind: 'point', pos: [b.c[0] - nx * 0.1, b.c[1], b.c[2] - nz * 0.1], dir: [0, -1, 0], color: [1.0, 0.12, 0.08], intensity: 1.6, range: 3, innerDeg: 0, outerDeg: 180, name: `truck_tail_${k}`, group: 'exterior', enabled: true, volumetric: 0, fixtureBox: fb, radius: 0.04, mains: false });
    });
    const bay: Vec3 = [VAN_AT.x - nx * 2.3, 1.84, VAN_AT.z - nz * 2.3];
    const bayLens = add({ c: bay, h: [0.07, 0.012, 0.07], ...M.plasticWhite, emissive: [22, 19, 13], flags: DECAL });
    lights.push({ kind: 'point', pos: [bay[0], bay[1] - 0.05, bay[2]], dir: [0, -1, 0], color: [1.0, 0.86, 0.62], intensity: 16, range: 6, innerDeg: 0, outerDeg: 180, name: 'truck_bay', group: 'exterior', enabled: true, volumetric: 0.2, fixtureBox: bayLens, radius: 0.05, mains: false });
  }
  { const c = at(VAN_AT, -0.5, 0, 0); cap(c[0], c[2], 2.2, 1.5, 1.95, VAN_AT.yaw, 0.03); }      // a dusting on its box: it has not stood long
  G.crates(6.4, 17.9, 0.5, 2); G.jerryCan(3.4, 17.4, 0.7, 'jerryYellow'); G.jerryCan(5.9, 18.6, -0.4); add({ c: [3.0, 0.07, 16.2], h: [0.3, 0.07, 0.22], yaw: 0.6, ...M.olive });   // what came off it when it stopped
  grp('convoy.escort');
  // its escort, off the road's line and burning: the fire lights the wall he runs to. (A light named 'brazier…' is an open fire: game/fire.ts gives it a
  // flame, embers, smoke and a flicker. The glowing plate is its mouth)
  const ESCORT = { x: 8.5, z: 17.5, yaw: 0.5 };
  G.pickup(ESCORT.x, ESCORT.z, ESCORT.yaw, 'truckTan', { load: 'none' });
  /** an open fire: the glowing plate it burns on, and its light */
  const fire = (name: string, c: Vec3, half: number, pos: Vec3, intensity: number, range: number, yaw = 0) => {
    const fb = add({ c, h: [half, 0.006, half], yaw, ...M.rust, emissive: [11, 3.8, 0.9], flags: DECAL });
    lights.push({ kind: 'point', pos, dir: [0, -1, 0], color: [1.0, 0.5, 0.2], intensity, range, innerDeg: 0, outerDeg: 180, name, group: 'exterior', enabled: true, volumetric: 0.25, fixtureBox: fb, radius: 0.06, mains: false });
  };
  { const c = at(ESCORT, -1.4, 0.84, 0), p = at(ESCORT, -1.4, 1.3, 0); fire('brazier_escort', c, 0.5, p, 13, 8.5, ESCORT.yaw); }
  grp('convoy.tanker');
  // THE TANKER: it took the post's corner and the rail, and stands across the whole road from the hut's wall to over the edge, its cab burning out past the
  // rail. Nobody passes it on the road: the way on is through the hut. A skirt to the ground along its length (nothing is seen or shot under it)
  { const b = G.piece(TANKER.x, TANKER.z, TANKER.yaw), tank = M.paintCream;
    for (const lx of [-3.0, -2.1, 0.7]) for (const s of [-1, 1]) b(lx, s * 0.98, 0, 0.92, 0.44, 0.15, M.rubber);
    b(-1.1, 0, 0.3, 1.0, 2.5, 1.02, { ...M.iron, rough: 0.85 });                                                    // the frame and its lockers
    b(-1.1, 0, 1.0, 2.45, 2.5, 1.15, tank); b(-1.1, 0, 2.45, 2.7, 2.4, 0.7, tank);              // the tank, its crown
    for (const s of [-1, 1]) b(-1.1, s * 1.155, 1.5, 1.82, 2.5, 0.004, M.truckRed, { flags: DECAL });   // the band along it
    for (const lx of [-2.6, -1.1, 0.4]) b(lx, 0, 2.7, 2.8, 0.26, 0.26, M.galv);                 // its hatches
    b(-3.62, 0, 0.5, 1.6, 0.03, 0.9, M.iron);                                                   // the ladder and valves at its back, against the hut's wall
    b(2.5, 0, 0.5, 1.55, 0.95, 1.05, M.plasticBlack); b(2.4, 0, 1.55, 2.2, 0.7, 1.0, M.glassDark); b(2.4, 0, 2.2, 2.3, 0.75, 1.02, M.plasticBlack);   // the cab, burnt black
    const c = at(TANKER, 2.5, 2.306, 0), p = at(TANKER, 2.5, 2.85, 0); fire('brazier_tanker', c, 0.55, p, 15, 9.5); }   // (it burns through the cab's roof: the light is over it, clear of every box)
  G.patch(18.6, 15.2, 2.6, 1.6, 'oil', 0.3, 0.019); G.patch(17.2, 16.4, 1.5, 2.2, 'oil', -0.2, 0.0198);   // what it is losing, on the snow

  // ---- THE FORECOURT inside the lower wall: the sentry's fire, what the post keeps by its gate ----
  grp('forecourt');
  G.drum(14.5, 16.7, 'rust'); fire('brazier_post', [14.5, 0.884, 16.7], 0.19, [14.5, 1.05, 16.7], 9, 6.5);
  G.plasticChair(13.6, 17.3, 0.9); G.crates(12.0, 18.9, 0, 2); G.tyres(15.9, 18.7, 3); G.drums(12.4, 10.4, 3); G.woodPile(14.2, 10.15, 0, 1.4);
  G.generator(16.1, 10.35, 0);                                                               // the post's generator, under the hut's wall
  G.wire(16.5, 10.35, HUT.x0, 10.2, 2.4);
  add({ c: [11.9, 0.9, 13.2], h: [0.03, 0.9, 0.03], ...M.galv }); add({ c: [11.9, 1.75, 13.18], h: [0.3, 0.3, 0.02], ...M.truckRed }); add({ c: [11.9, 1.75, 13.155], h: [0.2, 0.05, 0.004], ...M.paintCream, flags: DECAL });   // the sign inside the gateway: stop

  // ---- THE HUT, inside: where the post's two men sit out a watch ----
  grp('hut.room');
  // THE PLANK WALL: the back room, where the watch sleeps, is boarded off from the front one: boards on a rail, a way through at its end with no door.
  // Wood this thin does not stop a rifle round (game/wallpen.ts), and the man behind it is the lesson of that
  add({ c: [PLANKS.x, 1.15, (PLANKS.z0 + PLANKS.z1) / 2], h: [0.03, 1.15, (PLANKS.z1 - PLANKS.z0) / 2], ...M.ply });
  for (const y of [0.5, 1.6]) add({ c: [PLANKS.x - 0.036, y, (PLANKS.z0 + PLANKS.z1) / 2], h: [0.006, 0.05, (PLANKS.z1 - PLANKS.z0) / 2], ...M.woodGrey, flags: DECAL });   // its two rails, on the front room's face
  G.table(19.1, 10.25, 1.4, 0.66, 0, 'ply'); add({ c: [18.8, 0.87, 10.2], h: [0.2, 0.12, 0.13], ...M.olive }); add({ c: [19.4, 0.79, 10.3], h: [0.08, 0.04, 0.06], ...M.plasticBlack });   // the table, the set's charger, a handset
  G.plasticChair(19.1, 11.0, 0.1, 'plasticBlue'); G.stove(17.6, 11.35);
  G.rug(18.7, 12.2, 2.0, 1.3, 'felt');
  G.jerryCan(22.1, 13.35, 0.3, 'plasticBlue'); G.crates(20.65, 10.2, 0.1, 1);             // the back room: a store behind the boards, a nook behind the old flue wall
  G.wallHanging('x', HUT.z0, 20.75, 1, 'baluch', 0.7, 0.8);
  flat(17.5, HUT_DOOR_Z, 0.5, 0.8, S.snowOld, 0.07); flat(SIDE_DOOR_X, 13.4, 0.8, 0.5, S.snowOld, -0.06);   // slush tracked in at each door (nothing white under a roof)

  // ---- THE ROADBLOCK above the hut: the gun's nest under the rock, what stands behind the wall, the truck by the upper gate ----
  grp('roadblock');
  G.sandbags(30.4, 11.0, 2.6, HALF, 4); G.sandbags(31.8, 12.5, 3.0, 0, 4);                     // the nest: two low walls, the gun between them
  G.trunk(TRUNK[0], TRUNK[2], Math.PI, 'paintGreen');                                         // the radio is in it (the mission's one thing to take)
  add({ c: [31.2, 0.1, 10.4], h: [0.2, 0.1, 0.13], yaw: 0.3, ...M.olive }); G.crates(35.9, 10.3, 0.1, 2, 'coyote');   // the gun's ammunition, the roadblock's stores
  G.jersey(30.3, 19.15, 1, 0.12); G.drums(25.2, 18.9, 2); G.tyres(23.4, 18.8, 2); G.drumDown(27.9, 18.2, 0.4);   // the lane's far side, along the rail
  const TRUCK = { x: 34.4, z: 17.75, yaw: Math.PI };                                           // the roadblock's own truck, nose down the road, its lamps on
  G.pickup(TRUCK.x, TRUCK.z, TRUCK.yaw, 'truckRed', { load: 'crates' });
  for (const [k, lz] of [[0, -0.66], [1, 0.66]] as const) {
    const lens = add({ c: at(TRUCK, 2.49, 0.69, lz), h: [0.02, 0.06, 0.13], yaw: TRUCK.yaw, ...M.plasticWhite, emissive: [26, 24, 20], flags: DECAL });
    const d = at({ x: 0, z: 0, yaw: TRUCK.yaw }, 1, 0, 0);
    lights.push({ kind: 'spot', pos: at(TRUCK, 2.6, 0.69, lz), dir: norm([d[0], -0.035, d[2]]), color: [1.0, 0.93, 0.8], intensity: 110, range: 22, innerDeg: 9, outerDeg: 24,
      name: `block_lamp_${k}`, group: 'exterior', enabled: true, volumetric: 0.35, fixtureBox: lens, radius: 0.07, mains: false });
  }

  // ================================================================ SNOW: laid by hand ====
  grp('snow.cleared');
  cleared('gateway', POST.x0 - 0.9, GATEWAY.a + 0.2, POST.x0 + 1.1, GATEWAY.b - 0.2, 0.45);
  cleared('hut_door', 15.9, HUT_DOOR_Z - 0.7, HUT.x0 - t, HUT_DOOR_Z + 0.7, 0.35);
  cleared('side_door', SIDE_DOOR_X - 0.75, HUT.z1 + t, SIDE_DOOR_X + 0.75, 14.9, 0.35);
  cleared('upper_gate', 35.9, GATE_Z - 0.7, POST.x1 - t, GATE_Z + 0.7, 0.4);
  melted('fire_post', 14.5, 16.7, 0.55, 0.35); melted('fire_escort', ESCORT.x, ESCORT.z, 1.7, 0.45); melted('fire_tanker', 19.1, 18.9, 1.5, 0.5);
  grp('snow.paths');
  path([[4.7, 18.4], [3.5, 16.2], [3.1, 12.9]], 0.8);                                           // round the truck, by whoever got out of it
  path([[POST.x0 + 1.0, 15.2], [13.6, 14.2], [15.9, HUT_DOOR_Z]], 0.9);                               // the gateway → the hut's door
  path([[POST.x0 + 1.0, 15.6], [13.6, 16.4]], 0.8);                                             // the gateway → the fire
  path([[21.5, 14.9], [21.6, 16.6], [24.2, 16.8], [27.2, 16.7], [29.4, 16.5]], 0.9);            // the side door → along behind the sandbags → the wall
  path([[30.2, 15.9], [32.6, 13.6], [33.9, 13.0], [34.0, 11.4]], 0.8); path([[33.9, 13.0], [35.9, 14.2]], 0.8);   // the wall → the nest → the gate
  grp('snow.ruts');
  ruts([[0.5, 13.25], [3.2, 13.6]], 1.7); ruts([[6.4, 15.6], [POST.x0, 15.25]], 1.7); ruts([[POST.x0, 15.25], [16.8, 15.6]], 1.7);   // up the road and in through the gateway, as far as the tanker
  ruts([[22.8, 13.2], [31.6, 14.0], [POST.x1, 16.9]], 1.7);                                     // above the hut, to the trucks' gate
  grp('snow.caps');
  copingCaps('z', POST.x0, ROAD.z0, GATEWAY.a - 0.1); copingCaps('z', POST.x0, GATEWAY.b + 0.4, ROAD.z1); copingCaps('z', POST.x1, ROAD.z0, GATE_Z - 0.8); copingCaps('z', POST.x1, GATE_Z + 0.8, ROAD.z1);
  copingCaps('x', COVER_Z, RB_WALL.x0 + 0.1, RB_WALL.x1 - 0.1); copingCaps('z', HUT.x1, HUT.z1 + 0.2, COVER_Z);
  cap((BAGS.x0 + BAGS.x1) / 2, COVER_Z, BAGS.x1 - BAGS.x0 - 0.3, 0.34, 1.2, 0, 0.05);           // the sandbags
  cap(30.4, 11.0, 0.34, 2.3, 0.8, 0, 0.04); cap(31.8, 12.5, 2.7, 0.34, 0.8, 0, 0.04);           // the nest's
  cap(12.4, 10.4, 0.9, 0.4, 0.88, 0, 0.05); cap(14.2, 10.15, 1.2, 0.3, 0.42, 0, 0.06); cap(15.9, 18.7, 0.44, 0.44, 0.6, 0.3, 0.05); cap(12.0, 18.9, 0.7, 0.4, 0.9, 0, 0.05);
  cap(30.9, 18.6, 1.86, 0.2, 0.81, 0.12, 0.05); cap(25.2, 18.9, 0.8, 0.36, 0.88, 0, 0.05);
  { const c = at(TRUCK, 0.36, 0, 0); cap(c[0], c[2], 1.0, 1.3, 1.58, TRUCK.yaw, 0.03); }
  grp('snow.banks');
  // (they stop feet: against the rock and the rail, out of every route, none within 1.4 m of a door or on a mark)
  bank(5.2, ROAD.z1 - 0.36, 9.6, 0.62, Math.PI, 0.5); bank(14.2, ROAD.z1 - 0.36, 5.4, 0.62, Math.PI, 0.46); bank(26.0, ROAD.z1 - 0.36, 6.4, 0.62, Math.PI, 0.5); bank(33.0, ROAD.z1 - 0.3, 3.6, 0.5, Math.PI, 0.4);   // the plough's bank along the rail
  bank(3.4, ROAD.z0 + 0.32, 5.8, 0.6, 0, 0.46); bank(8.9, ROAD.z0 + 0.3, 3.2, 0.56, 0, 0.4); bank(26.0, ROAD.z0 + 0.34, 5.8, 0.64, 0, 0.5);   // packed along the rock's foot
  grp('snow.drifts');
  // (waded: wind-laid snow, 0.19 m and under, on the side of each thing the pass's wind comes from)
  drift(POST.x0 - 0.42, 11.4, 3.4, 0.6, -HALF, 0.16); drift(POST.x0 - 0.42, 18.3, 2.4, 0.6, -HALF, 0.17);   // the lower wall, outside
  drift(POST.x1 - 0.42, 11.2, 2.6, 0.6, -HALF, 0.16);                                          // the upper wall, inside
  drift(HUT.x0 - 0.4, 10.3, 1.0, 0.5, -HALF, 0.14); drift(HUT.x0 - 0.4, 13.2, 0.9, 0.5, -HALF, 0.14);   // the hut, under its window and past its door
  drift(24.9, COVER_Z - 0.5, 4.4, 0.5, Math.PI, 0.15); drift(28.6, COVER_Z - 0.42, 2.2, 0.5, Math.PI, 0.15);   // the sandbags and the wall, on the gun's side
  drift(30.06, 11.0, 2.2, 0.44, -HALF, 0.14);                                                  // the nest
   drift(35.0, ROAD.z0 + 0.34, 3.4, 0.6, 0, 0.15);
  // sastrugi: long low ridges the wind has cut across the open snow, all on one bearing
  for (const [x, z, len, d, h] of [[1.6, 15.8, 2.2, 0.5, 0.07], [7.4, 12.2, 1.8, 0.4, 0.06], [9.0, 14.6, 1.4, 0.36, 0.06], [12.8, 12.4, 1.6, 0.4, 0.06], [24.2, 12.0, 2.2, 0.45, 0.08], [26.8, 13.6, 1.6, 0.4, 0.06],
    [28.6, 11.8, 2.0, 0.45, 0.07], [32.4, 15.2, 1.4, 0.36, 0.06], [24.6, 18.0, 1.6, 0.4, 0.06], [29.2, 17.9, 1.8, 0.4, 0.07], [35.6, 12.0, 1.2, 0.36, 0.06],
    [5.0, 20.6, 2.4, 0.45, 0.07], [12.6, 20.6, 2.0, 0.45, 0.07], [26.4, 20.6, 2.2, 0.45, 0.08], [33.8, 20.6, 1.8, 0.4, 0.07]] as const)
    add({ c: [x, h / 2, z], h: [len / 2, h / 2, d / 2], yaw: -0.62 + ((x * 3 + z) % 1) * 0.16, ...S.snow });

  // ---- light: a thin moon on snow, and warm things that burn. Three fires (the escort, the sentry's drum, the tanker's cab), two trucks' lamps, the
  //  post's sodium lamps on its mains, the hut's bulb through its window and its doors, lanterns ----
  grp('lights');
  /** a sodium lamp on a wall or a post, thrown down and out along `dir`: on the mains, group 'exterior' (the photocell puts it out at midday) */
  const sodium = (name: string, head: Vec3, dir: Vec3, intensity = 60, range = 8) => {
    const fb = add({ c: head, h: [0.12, 0.06, 0.12], ...M.plasticBlack, emissive: [30, 20, 9], flags: DECAL });
    lights.push({ kind: 'spot', pos: [head[0] + dir[0] * 0.05, head[1] - 0.08, head[2] + dir[2] * 0.05], dir: norm(dir), color: [1.0, 0.72, 0.42], intensity, range, innerDeg: 36, outerDeg: 72,
      name, group: 'exterior', enabled: true, volumetric: 0.15, fixtureBox: fb, radius: 0.12, broad: true });
  };
  add({ c: [POST.x0 + 0.05, 2.5, GATEWAY.b + 0.1], h: [0.13, 0.015, 0.015], ...M.galv }); sodium('lamp_gateway', [POST.x0 + 0.2, 2.44, GATEWAY.b - 0.1], [0.4, -0.86, -0.3], 70);   // over the gateway, an arm off its far pier: it lights the sentry
  sodium('lamp_hut', [HUT.x0 - 0.14, 2.56, HUT_DOOR_Z], [-0.2, -0.97, 0], 15, 5);              // over the hut's door: a small one, straight down (the gateway's lamp is this court's key)
  sodium('lamp_gate', [POST.x1 - 0.2, 2.44, GATE_Z + 0.9], [-0.5, -0.86, -0.1], 80, 9);        // over the upper gate: the last three stand in it
  G.bulb(18.6, 11.4, 'bulb_hut', 'hut');
  G.lantern(20.45, 10.9, 'lantern_hut', 'hut');                                               // the back room's: the man behind the planks stands in it
  { const fb = add({ c: [17.82, 0.3, 11.35], h: [0.012, 0.05, 0.07], ...M.rust, emissive: [11, 3.8, 0.9], flags: DECAL });   // the stove's door: embers
    lights.push({ kind: 'point', pos: [17.95, 0.32, 11.35], dir: [0, -1, 0], color: [1.0, 0.5, 0.2], intensity: 3.0, range: 4.5, innerDeg: 0, outerDeg: 180, name: 'stove_hut', group: 'hut', enabled: true, volumetric: 0.25, fixtureBox: fb, radius: 0.06, mains: false }); }
  G.lantern(POST.x0 - 0.45, 10.9, 'lantern_wall', 'exterior');                                 // at the lower wall's foot: the wall he takes is a lit wall
  G.lantern(22.05, 14.2, 'lantern_side', 'exterior');                                          // in the wind wall's corner, by the side door
  G.lantern(30.95, 10.05, 'lantern_nest', 'exterior', 0);                                        // in the nest: the gun's crew is lit by it
  // THE MOON IS THIN, as the pass's is and for its reason: snow is the brightest ground paint in the game. It stands over the drop, so the rock is lit and
  // every wall across the road throws its shadow up the road
  lights.push({ kind: 'dir', pos: [0, 0, 0], dir: norm([0.5, -0.5, -0.7]), color: [0.5, 0.68, 1.0], intensity: 0.06, range: 1e6, innerDeg: 0, outerDeg: 0, name: 'moon', group: 'exterior', enabled: true, volumetric: 0, radius: 0.009, mains: false });

  // ---- the breaker and the beacon every level carries (the hut's back wall; nobody is sent to either) ----
  grp('breaker');
  const brk = add({ c: [17.75, 1.35, HUT.z0 + t + 0.04], h: [0.28, 0.36, 0.04], albedo: [0.42, 0.44, 0.45], name: 'breaker_panel' });
  const led = add({ c: [17.57, 1.62, HUT.z0 + t + 0.09], h: [0.015, 0.015, 0.015], albedo: P.plasticBlack, rough: 0.5, emissive: [0.3, 4, 0.6], flags: DECAL });
  grp('beacons');
  add({ c: [HUT.x0 - 0.2, 2.85, 13.0], h: [0.07, 0.06, 0.07], albedo: [0.2, 0.2, 0.22], flags: DECAL });
  const dome = add({ c: [HUT.x0 - 0.2, 2.72, 13.0], h: [0.055, 0.07, 0.055], albedo: [0.35, 0.08, 0.05], flags: DECAL });
  switchables.push({ name: 'beacon0', boxes: [dome], group: 'emergency', on: false, baseEmissive: [9, 1.2, 0.35], mains: false });

  // ---- six men, each on his mark until the script calls his beat (levels/tut-road.script.ts names them by roster order) ----
  const post = (name: string, p: Vec3, weapon?: PatrolRoute['weapon'], dx = 0.3): PatrolRoute => ({ name, points: [p, [p[0] + dx, 0, p[2] + 0.2]], wait: [600, 600], ...(weapon ? { weapon } : {}) });
  const routes: PatrolRoute[] = [post('sentry', SENTRY), post('hut', HUT_NOOK, undefined, 0), post('gunner', GUNNER, 'pkm'), post('bend', LAST_OUT[0]), post('trucks', LAST_OUT[1]), post('tunnel', LAST_OUT[2])];

  const front: Vec3 = [TRUNK[0], 0.45, TRUNK[2] + 0.44];
  const quiet: QuietData = {
    thing: 'the radio',
    starts: [{ name: 'the road', sam: START, samYaw: Math.PI }],
    spots: [{ place: 'the gun', area: { ...NEST }, door: 'hut', holder: 'the green trunk', rackIndex: 0, front, halfW: 0.3 }],
    exits: [EXIT],
    patrols: [routes.map(r => r.name)],
    radioCheck: false,
  };

  const DOWN = (n: number) => ({ is: 'men', down: n } as const);
  const lesson: LessonData = {
    title: 'The pass road',
    rails: { allow: ['move'], rows: ['drive'] },   // (the interaction key's one row here is the radio: no pick, no boot, no dead man to drag)
    steps: ([
      // THE ROAD
      { id: 'move', line: 'Use [W] [A] [S] [D] to move, and get behind the truck', done: { is: 'near', at: VAN, r: 1.1 }, at: VAN },
      // (the view is his from here: behind the truck with rounds going into its far side, the first thing worth turning it for is whoever is firing them)
      { id: 'turn', line: 'Hold [Ctrl] and move the mouse to turn the view and look up the road', done: { is: 'view', turned: 40 } },
      { id: 'ready', line: 'Press [2] to bring up your rifle', done: { is: 'slot', slot: 'rifle', drawn: true } },
      // (it goes by for a man who comes in at a sprint, and for one who let go of the key short of the wall and walked the rest: he is not left standing at
      //  the wall with a line that wants him to run at it again)
      { id: 'sprint', line: 'Hold [Shift] to sprint to the wall by the gate', done: { is: 'any', of: [{ is: 'all', of: [{ is: 'near', at: WALL, r: 3.5 }, { is: 'flag', flag: 'sprint' }] }, { is: 'near', at: WALL, r: 0.9 }] }, at: WALL },
      // (the mark is a pace short of the wall and points into it)
      { id: 'wall', line: 'Walk into the wall to snap to it for cover', done: { is: 'flag', flag: 'wall' }, at: [WALL[0] - 0.35, 0.05, WALL[2]], toward: [POST.x0, 0.05, WALL[2]],
        explain: { title: 'Taking cover on a wall', text: 'Walk into a wall and you\'ll snap to it, with your back against it and out of the line of fire. Use the move keys to slide along it. When you reach its end, aim past the corner and you\'ll lean out to shoot.' } },
      // (the peek is the lesson, the shot its reward. The trigger is still locked here, so the lean is the only way on: a man who walks into the gateway
      //  instead is shot at by nobody, the sentry's fire having stopped at the line before)
      { id: 'peek', line: 'Slide along to the gate, then aim past the corner to lean out', done: { is: 'flag', flag: 'lean' }, at: CORNER },
      // (`settle`, here and on every step that ends in a shot or a man going down: the next line or card comes up once the moment has played out, not over it)
      { id: 'fire', line: 'Left-click to shoot the sentry', done: DOWN(1), settle: 1.5, at: [SENTRY[0], 1.2, SENTRY[2]] },
      // THE FORECOURT: the gun's own setup, met in a quiet moment (the sentry is down and nobody else can reach him). One line, two presses: the rifle's
      // key again brings its setup up, and the first row of it is the selector. Where the gun has no setup (the switch off) the selector has its own key
      { id: 'auto', line: 'Press [Z] to switch to full auto', setupLine: 'Press [2] again and then [1] to switch to full auto', done: { is: 'flag', flag: 'auto' },
        explain: { title: 'Setting up your gun', text: 'When you\'re holding a gun, press its key again to open its setup menu above the kit strip. You can switch between single shots and full auto there.' } },
      // OFF THE WALL: a hold lets go when he moves square away from it, and no other key of his does it. The way on is round the wall's end, which is ALONG
      // the wall and INTO it: a hand that only knows 'walk' pushes at the corner and stays where it is (found on the real page, with real keys)
      { id: 'off', line: 'Move away from the wall to step off it', done: { is: 'flag', flag: 'wall', value: false } },
      // THE HUT'S DOOR: full auto has a use at once, and the reload comes after a magazine has really been fired. The leaf goes on the twelfth rifle round
      // (game/doorblast.ts DOORSHOT); where that rule is off, or his rounds find the jambs, the twentieth round he fires on this line ends it and the
      // level's script throws the door open as the next line comes up (cue 'way'), so the way on never hangs on a switch. The reload's line waits for the
      // gun to be FULL again, and a gun he ran dry has already started its own reload (game/autoreload.ts): the line is then being done, and goes by when it is
      { id: 'inner', line: 'Go through the gate to the hut\'s wooden door', done: { is: 'near', at: [HUT.x0 - 0.9, 0, HUT_DOOR_Z], r: 1.5 }, at: [HUT.x0, 1.1, HUT_DOOR_Z] },
      { id: 'door', line: 'Hold the left mouse button to shoot the wooden door until it breaks', done: { is: 'any', of: [{ is: 'door', door: 'hut', state: 'open' }, { is: 'did', what: 'event:shot', n: 20 }] }, settle: 1, at: [HUT.x0, 1.1, HUT_DOOR_Z] },
      { id: 'reload', line: 'Press [R] to reload', done: { is: 'loaded' }, settle: 0.8 },
      // THE PLANK WALL: a man in the back room, behind boards, firing blind through them at the wall over the front room. Nothing says where he is but
      // his own rounds coming through the wood and the mark that rides on him; the answer is the one the line gives. (A man who would rather walk round
      // the boards finds him too: either way he is down)
      { id: 'thin', line: 'Shoot the enemy through the wooden wall', done: DOWN(2), settle: 1.2, atMan: 'hut',
        explain: { title: 'Shooting through thin walls', text: 'A rifle round will go through wood, plaster and doors. An enemy behind them isn\'t safe from you, and you aren\'t safe from him. Concrete, brick and sandbags will stop it.' } },
      // THE GUN: he comes out of the side door into the wind wall's lee as the gun opens on the tanker (a second there: the gun is firing before the next line, whenever he came)
      { id: 'arch', line: 'Go out the side door and stop behind the wall', done: { is: 'near', at: JAMB, r: 1.0 }, hold: 1, at: JAMB },
      { id: 'crouch', line: 'Press [C] to crouch, and the shots will go over you', done: { is: 'all', of: [{ is: 'flag', flag: 'crouch' }, { is: 'near', at: JAMB, r: 1.9 }] }, at: JAMB },
      { id: 'low', line: 'Stay low and follow the sandbags', done: { is: 'all', of: [{ is: 'flag', flag: 'crouch' }, { is: 'near', at: BAGS_END, r: 1.1 }] }, at: BAGS_END },
      // THE ROADBLOCK'S WALL: the wall and the lean again, with nobody saying how, on the man who has been shooting at him
      { id: 'cover', line: 'Walk into the concrete wall to take cover', done: { is: 'all', of: [{ is: 'flag', flag: 'wall' }, { is: 'near', at: COVER, r: 2.2 }] }, at: [COVER[0], 0.05, COVER[2]], toward: [COVER[0], 0.05, COVER_Z] },
      { id: 'gunner', line: 'Slide to the corner, lean out and shoot the gunner', done: DOWN(3), settle: 1.5, at: [GUNNER[0], 1.2, GUNNER[2]] },
      // THE UPPER GATE: three more come in by it, on a line that only asks him to look (the frame draws a line with no key and a cue for its end as
      // something to watch); the script ends that line once they stand in the open, so the trigger is asked for with men in front of it
      { id: 'watch', line: 'Watch the upper gate for more enemies', done: { is: 'any', of: [{ is: 'cue', name: 'gate' }, DOWN(6)] } },
      { id: 'last', line: 'Hold the left mouse button and shoot them all', done: DOWN(6), settle: 1.5, at: [LAST_MEN[1][0], 1.2, LAST_MEN[1][2]] },
      // THE RADIO: the one thing the mission has him take, where the gun was, with nobody left on the road; then the walk out is the end
      { id: 'radio', line: 'Go to the green trunk behind the machine gun', done: { is: 'near', at: [front[0], 0, front[2] + 0.5], r: 1.2 }, at: front },
      { id: 'take', line: 'Press [Space] to take the radio', done: { is: 'stage', stage: 'exfil' }, at: front },
      { id: 'out', line: 'Go out through the upper gate', done: { is: 'stage', stage: 'done' }, at: [EXIT.x, 0, EXIT.z] },
    ] as LessonStep[]).map(T => ({ ...T, ...FRAME[T.id] })),
  };

  for (const d of doors) doorFrame(d);
  const level: Level = {
    boxes: K.boxes, props: K.props, lights, switchables, doors, devices: [],
    breaker: { pos: [17.75, 1.35, HUT.z0 + t + 0.18], boxes: [brk, led] }, beacons: [{ pos: [HUT.x0 - 0.2, 2.61, 13.0], box: dome }],
    playerSpawn: START, routes, chokepoints: [], clearPlans: [],
    sky: { zenith: [0.012, 0.02, 0.042], horizon: [0.04, 0.05, 0.075] },
    // (the script's eyes: a play program may ask of the player only whether he stands in a volume)
    volumes: [
      // the road between the truck and the sentry (the gateway's mouth and the ground inside it too: a man who sprints to the wall and carries on round its
      // end is in the same line), and round the truck's nose: the sentry's rounds cross it, so he holds his fire while the player is in either
      { name: 'road_east', kind: 'trigger', c: [10.2, 1, 14.6], h: [4.5, 1, 5.0] }, { name: 'road_nose', kind: 'trigger', c: [5.0, 1, 10.95], h: [0.7, 1, 1.05] },
      // the front room's far side, under the wall the man behind the planks is shooting: he holds his fire while the player stands in it
      { name: 'hut_north', kind: 'trigger', c: [18.65, 1, 10.65], h: [1.55, 1, 0.85] },
      // the gun's open snow, on its side of the sandbags and the roadblock's wall: a man who walks out into it is what the gun shoots at
      { name: 'field', kind: 'trigger', c: [26.55, 1, 12.4], h: [3.95, 1, 2.75] },
      // everything above the roadblock's wall, a pace past its bare end (a man leaning round that end is not in it): the gun shoots at a man who walks
      // out into it, and the last three hold their fire for one (they are shooting at the wall he should be behind)
      { name: 'open_east', kind: 'trigger', c: [33.7, 1, 14.6], h: [3.2, 1, 5.0] },
    ],
    mission: { serverRoom: { ...NEST }, entryDoor: 'hut', rack: { index: 0, front, halfW: 0.3 }, exfilDoor: 'gate', exfilX: EXIT.x, exfilZ: EXIT.z, exfilOut: EXIT.out },
  };
  // what is underfoot (game/floor.ts): fresh snow is 'grass' (the soft, quiet one); the road, the trodden paths and cleared earth are 'asphalt'; the hut's slab is 'stone'
  const surfaces: SurfacesData = { building: { x0: 0, x1: 40, z0: 0, z1: 28 }, grassEdge: 0, inside: 'grass', outside: 'grass', verge: 'grass',
    zones: [{ id: 'hut_felt', mat: 'carpet', x0: 17.7, z0: 11.55, x1: 19.7, z1: 12.85 }, ...zones, { id: 'road', mat: 'asphalt', x0: 0, z0: ROAD.z0 + 1.0, x1: 40, z1: ROAD.z1 - 0.9 }] };
  // THE SCRIPT'S FIRE POINTS, held to this floor: its rounds are real, and a wall moved here with the points left where they were is a round into a place a
  // line asks him to stand in. Each is asked where it should be: on the truck's flank, on the hut's far wall, on the hut's end, over the sandbags, on the
  // roadblock wall's far face
  { const bad: string[] = [], on = (what: string, ok: boolean) => { if (!ok) bad.push(what); };
    for (const p of TRUCK_FLANK) on(`the truck's flank (${p})`, Math.abs(p[0] - (VAN_AT.x + 1.0)) < 0.3 && p[2] > VAN_AT.z - 1.8 && p[2] < VAN_AT.z);
    for (const p of HUT_WALL) on(`the hut's far wall (${p})`, Math.abs(p[2] - (HUT.z0 + 0.13)) < 0.05 && p[0] > HUT.x0 + 1 && p[0] < PLANKS.x - 0.4 && p[1] > 2.1);
    for (const p of HUT_END) on(`the hut's end (${p})`, Math.abs(p[0] - (HUT.x1 + 0.13)) < 0.05 && p[2] > HUT.z0 && p[2] < COVER_Z);
    for (const p of BAGS_TOP) on(`the sandbags' top (${p})`, p[2] === COVER_Z && p[0] > BAGS.x0 && p[0] < BAGS.x1 && p[1] > 1.3 && p[1] < 1.5);
    for (const p of WALL_FACE) on(`the roadblock wall's far face (${p})`, Math.abs(p[2] - (COVER_Z - 0.13)) < 0.05 && p[0] > RB_WALL.x0 + 0.3 && p[0] < RB_WALL.x1 - 0.6);
    on('the nook is behind its wall', HUT_NOOK[0] > NOOK_WALL.x + 0.4 && HUT_NOOK[0] < HUT.x1 - 0.4 && HUT_NOOK[2] < NOOK_WALL.z1 - 0.6);
    on('his mark is behind the boards', HUT_MARK[0] > PLANKS.x + 0.4 && HUT_MARK[2] < PLANKS.z1 - 0.55 && HUT_MARK[2] > NOOK_WALL.z1 + 0.1 && Math.hypot(HUT_MARK[0] - (NOOK_WALL.x - 0.12), HUT_MARK[2] - NOOK_WALL.z1) > 0.3);
    if (bad.length) throw new Error(`tut-road: the script's places (levels/tut-road.script.ts) are off this floor: ${bad.join(' · ')}`); }
  const scenarios: LevelScenarios = { quiet, lesson, issue: ISSUE, doorNames: [{ door: 'hut', name: 'hut door' }, { door: 'gate', name: 'upper gate' }],
    places: { areas: [{ name: 'the road', bounds: { x0: 0, x1: POST.x0, z0: ROAD.z0, z1: ROAD.z1 } }, { name: 'the gateway', bounds: { x0: POST.x0, x1: HUT.x0, z0: ROAD.z0, z1: ROAD.z1 } },
      { name: 'the roadblock', bounds: { x0: HUT.x1, x1: POST.x1, z0: ROAD.z0, z1: ROAD.z1 } }] } };
  return { level, scenarios, groups: trace.groups, surfaces };
}
