// THE SIMULATION IN RUST, FROM TYPESCRIPT — the one place the Rust module (rust/sim, built to WebAssembly: src/sim/built.ts) is loaded and named.
//
// WHAT THIS IS FOR: the game's update is moving out of TypeScript into Rust a system at a time (docs/agents/rust.md has the order, the rules and what holds
// each step to the TypeScript it replaces). Everything that calls into the module goes through `sim()`; everything that says WHICH systems run there is a row
// of RUST below.
//
// ONE WAY IN, EVERYWHERE. The module's bytes are text in built.ts, so the page, the game server, bun and node load them as they load any source file; and it
// is compiled and started in one synchronous step the first time it is asked for, so nothing that uses it has to be written around a promise. (Chrome allows a
// synchronous compile on the page's own thread up to 8 MB; tools/sim/build.ts is red long before that.)
//
// MEMORY: the module owns one linear memory. A typed array over it dies when the memory GROWS — never keep one across a call that can allocate; ask `f64()`
// / `u8()` again (they hand back the same array until the buffer has changed).
//
// (Imports built.ts, the module starter and src/math alone: like src/math, this is below the game.)
import { SIM_WASM, SIM_BYTES } from './built';
import { startModule } from './load';
import { mathsFrom, EXACT, type Maths } from '../math/exact';
import { kahanN } from '../math/hyp';

/** the version of the exports this file was written against (rust/sim/src/lib.rs ABI) */
export const SIM_ABI = 30;

/** the module's exports, by name (rust/sim/src/lib.rs: plain C functions, `<module>_<thing>`) */
export interface SimExports {
  readonly memory: WebAssembly.Memory;
  sim_abi(): number;
  /** where the scratch is: a few doubles a call's own parameters cannot carry go through it (write, call, read; nothing is kept there between calls) */
  sim_scratch(): number;
  sim_scratch_len(): number;
  // the game's own maths (rust/sim/src/maths.rs — src/math/exact.ts to the bit)
  maths_sin(x: number): number; maths_cos(x: number): number; maths_tan(x: number): number;
  maths_asin(x: number): number; maths_acos(x: number): number; maths_atan(x: number): number; maths_atan2(y: number, x: number): number;
  maths_exp(x: number): number; maths_log(x: number): number; maths_log2(x: number): number; maths_log10(x: number): number;
  maths_pow(x: number, y: number): number; maths_tanh(x: number): number;
  maths_hypot2(a: number, b: number): number; maths_hypot3(a: number, b: number, c: number): number; maths_hypot4(a: number, b: number, c: number, d: number): number;
  /** the length of the first n doubles of the scratch */
  maths_hypot_n(n: number): number;
  // the box world's copy (rust/sim/src/world.rs — written by src/scene/boxMirror.ts, which names the tables)
  world_addr(w: number, table: number): number; world_cap(table: number): number;
  world_shape(w: number, gw: number, gh: number, cs: number, boxes: number, tilted: number): void; world_leaves(w: number, set: number, n: number): void; world_things(w: number, n: number): void;
  // the line tests through it (rust/sim/src/lines.rs — src/scene/collision.ts segmentBlocked / segmentBlockedDynamic past their length)
  lines_blocked(w: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, len: number, shrink: number, through: number, set: number, ask: number): number;
  lines_leaves(w: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, len: number, need: number, set: number): number;
  /** collision.ts raycast: −1 nothing met · −2 the host answers (a ray that is no number) · else the static's index, or 1e6 + the leaf's; t and the normal → the scratch. `ask` 1 = a pane met where it is open is passed (glass.rs) */
  lines_ray(w: number, a0: number, a1: number, a2: number, r0: number, r1: number, r2: number, tmax: number, set: number, ask: number): number;
  world_glass(w: number, n: number): void; world_view(w: number, kind: number, n: number): void; glass_open(w: number, bi: number, x: number, y: number, z: number): number;
  // a circle against it (rust/sim/src/collide.rs — src/scene/collision.ts collideCircle / circleTouches / gather)
  collide_circle(w: number, px: number, pz: number, r: number, y0: number, y1: number, iterations: number): number;
  collide_touches(w: number, x: number, z: number, r: number, y0: number, y1: number, cap: number): number;
  collide_gather(w: number, x0: number, z0: number, x1: number, z1: number, y0: number, y1: number, cap: number): number;
  // the fog of war's sight pass (rust/sim/src/fog.rs — src/game/fogofwar.ts through collision.ts sightOpen / eyesSee)
  fog_eyes(w: number): number;
  fog_see(w: number, set: number, order: number, n: number, want: number, kind: number, through: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number): number;
  // posing (rust/sim/src/skeleton.rs — src/anim/skeleton.ts). A question says WHERE ITS POSE IS: g = −1 → place `at` of the module's bench (the host wrote it
  // there and reads the answer back) · else game g's pool (rust/sim/src/poses.rs), the pose starting at single `at` of that part. 1 = done, 0 = not a pose.
  pose_addr(table: number): number; pose_rig(nodes: number, order: number): number;
  pose_blend(g: number, a: number, b: number, w: number, how: number, out: number, n: number): number;
  pose_run(g: number, at: number, root: number): number; pose_parent_rot(g: number, at: number, i: number): number;
  pose_turn(g: number, at: number, nodes: number, i: number, how: number): number;
  pose_ik(g: number, at: number, a: number, b: number, c: number, flags: number): number;
  knob_addr(table: number): number; knob_len(table: number): number;
  layer_mv_trunk(g: number, at: number): number; layer_long_body(g: number, at: number): number; layer_life(g: number, at: number): number; layer_mv_pre(g: number, at: number): number;
  layer_stance_body(g: number, at: number): number; layer_stand_pose(g: number, at: number, atB: number, atC: number): number; layer_crouch_pose(g: number, at: number, atB: number, atC: number): number; layer_soft_knees(g: number, at: number): number; layer_run_arms(g: number, at: number): number; layer_run_plant(g: number, at: number): number; layer_sneak_body(g: number, at: number): number; layer_steady(g: number, at: number, atC: number): number; layer_creep_body(g: number, at: number): number; layer_kick_body(g: number, at: number, atC: number): number; layer_toes_up(g: number, at: number): number; layer_wall_body(g: number, at: number, atC: number, atB: number, atRest: number): number; layer_watch_slip(g: number, at: number): number; layer_watch_look(g: number, at: number): number; layer_mv_post_body(g: number, at: number): number; layer_turn_weigh(): number; layer_lean_body(g: number, at: number): number; layer_gaze_keep(g: number, at: number): number; layer_sneak_arms(g: number, at: number, atB: number): number; layer_carry_arms(g: number, at: number, atB: number, atAim: number): number; layer_wall_arms(g: number, at: number, atB: number, atRest: number): number; layer_act_pose(g: number, at: number, atB: number, atRest: number): number; layer_wound_body(g: number, at: number, atB: number, atRest: number): number; layer_wound_limbs(g: number, at: number, atB: number, atRest: number): number; layer_mv_post_arms(g: number, at: number, atB: number, atRest: number): number; layer_watch_arm(g: number, at: number, atB: number, atRest: number): number; layer_creep_arms(g: number, at: number, atB: number, atRest: number): number; layer_clutch_arm(g: number, at: number, atB: number, atRest: number): number; layer_bound_arms(g: number, at: number, atB: number, atRest: number): number; layer_reload_arm(g: number, at: number, atB: number, atRest: number): number; layer_pistol_reload_arm(g: number, at: number, atB: number, atRest: number): number; layer_hands_pass(g: number, at: number): number; layer_throw_layer(g: number, at: number, atB: number, atRest: number): number; layer_big_layer(g: number, at: number, atB: number, atBuf: number, atRest: number): number; layer_hold_layer(g: number, at: number, atB: number, atC: number, atAim: number, atRest: number): number; layer_held_layer(g: number, at: number, atB: number, atRest: number): number; layer_catch_arm(g: number, at: number): number; layer_door_layer(g: number, at: number, atB: number, atRest: number): number; layer_work_layer(g: number, at: number, atB: number, atRest: number): number; layer_ready_layer(g: number, at: number, atB: number, atRest: number): number; layer_kit_layer(g: number, at: number, atB: number, atRest: number): number; layer_taken_layer(g: number, at: number): number; layer_blow_legs(g: number, at: number): number; layer_blow_trunk(g: number, at: number): number; layer_blow_arms(g: number, at: number, atB: number, atRest: number): number; layer_knife_pose(g: number, at: number, atB: number, atRest: number): number; layer_cable_pose(g: number, at: number, atB: number, atRest: number): number; layer_stealth_layer(g: number, at: number, atB: number, atRest: number): number; layer_swap_pose(g: number, at: number, atB: number, atRest: number, atSw: number): number; layer_lockpick_layer(g: number, at: number, atB: number, atRest: number): number; layer_haul_layer(g: number, at: number, atB: number, atRest: number): number; layer_torch_up(g: number, at: number, atB: number): number; layer_radio_pose(g: number, at: number, atB: number, atRest: number): number; layer_smoke_pose(g: number, at: number, atB: number, atRest: number): number; layer_nv_pose(g: number, at: number, atB: number, atRest: number): number; layer_signal_layer(g: number, at: number, atB: number, atRest: number): number; layer_tell_layer(g: number, at: number, atB: number, atRest: number): number; layer_long_arms(g: number, at: number, atB: number, atAim: number, atRest: number): number;
  /** skeleton.ts sampleClip: clip `clip` (its number from clip_end) at time t into the pose, whose own count of nodes is `nodes` */
  pose_sample(g: number, at: number, nodes: number, clip: number, t: number, how: number): number;
  // the clips (rust/sim/src/clips.rs): a character file's animations, copied in once. clip_begin (0 = no room) · clip_track for each track → where its times
  // are written, an index into the singles, its values after them (−1 read unsigned = refused) · clip_end → the clip's number (−1 = refused, nothing kept)
  clip_begin(duration: number, tracks: number): number; clip_track(node: number, path: number, step: number, keys: number): number; clip_end(): number;
  /** room for a pose of `nodes` nodes in game g's pool, zeros: where it starts (−1 here = the pool is full) */
  pose_new(g: number, nodes: number): number;
  /** game g's pool empty again */
  pose_clear(g: number): void;
  // the senses (rust/sim/src/sight.rs, light.rs, smoke.rs, hear.rs, nav.rs — asked through src/scene/collision.ts, THE SENSES IN THE MODULE). 0xffffffff (−1 here) = ask the TypeScript.
  /** guards.ts eyeContact: 1 visible + 2 in the cone; the distance → scratch 0 */
  sight_contact(w: number, set: number, through: number, q: number, e0: number, e1: number, e2: number, l0: number, l1: number, l2: number, range: number, halfDeg: number, nearM: number, nearDeg: number, vHalfDeg: number, c0: number, c1: number, c2: number, h0: number, h1: number, h2: number): number;
  light_lamps(): number; light_cap(): number;
  /** guards.ts lightEstimateAt's loop over the lamps written: the six faces → scratch 0‥5 */
  light_estimate(w: number, set: number, n: number, p0: number, p1: number, p2: number, sigma: number, g: number, stamp: number, stands: number, more: number, nb: number, spread: number, widest: number, rise: number, riseRate: number): number;
  smoke_blobs(): number; smoke_cap(): number;
  /** sense.ts CpuSmoke.trans over the blobs written: the transmittance → scratch 0 */
  smoke_trans(w: number, set: number, n: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, sigma: number, spread: number, widest: number, rise: number, riseRate: number): number;
  nav_addr(table: number): number; nav_cap(table: number): number;
  /** collision.ts NavGrid.findPath with no penalty: the waypoints written (x z each) · −1 no way · −2 ask the TypeScript */
  nav_path(w: number, set: number, ask: number, gw: number, gh: number, cell: number, a0: number, a2: number, b0: number, b2: number, maxIter: number): number;
  /** …with a penalty: as nav_path, or −3 = `nav_need()` cells (nav_addr(2)) want a price (nav_addr(3)) before `nav_resume()`; `nav_drop()` lets a waiting search go */
  nav_priced(w: number, set: number, ask: number, gw: number, gh: number, cell: number, a0: number, a2: number, b0: number, b2: number, maxIter: number): number;
  nav_need(): number; nav_resume(): number; nav_drop(): void;
  /** collision.ts SoundPaths.propagate up to its memory of routed answers; the straight distance → scratch 0 */
  hear_direct(w: number, set: number, ask: number, s0: number, s1: number, s2: number, d0: number, d1: number, d2: number): number;
  /** …and the way round: the path's length and the point it comes through → scratch 0‥3 */
  hear_route(w: number, set: number, ask: number, seal: number, gw: number, gh: number, cell: number, s0: number, s1: number, s2: number, d0: number, d1: number, d2: number, straight: number): number;
  // a game's state in the module (rust/sim/src/state.rs — a block a game, bound to slot g by src/sim/state.ts)
  state_addr(g: number): number; state_len(): number; state_games(): number; state_layout(): number; state_parts(): number; state_part(i: number, what: number): number;
  // a man's columns: the module's own word for their layout, and each column's place (rust/sim/src/men.rs — src/sim/men/store.ts holds them to its own)
  men_schema(): number; men_col(i: number): number;
  /** every number of row `row` of game g's men as the module reads it, folded: what tools/qa/men-check.ts holds to the accessors */
  men_fold(g: number, row: number): number;
  // the game's own die, in its block (rust/sim/src/dice.rs — src/math/dice.ts simRandom)
  dice_roll(g: number): number;
  // the game's own smoke on the coarse grid (rust/sim/src/bricks.rs; game/sense.ts SmokeGrid): its state is a part of the game's block (state.ts), so a call
  // names the block's slot `g` (`st.bind()`), with the block's `stamp` where it reads the tables made from the state, and a step the box world and its copy
  grid_reset(g: number, home: number): void; grid_clear(g: number): void; grid_dirty(w: number, x0: number, z0: number, x1: number, z1: number): void;
  grid_step(g: number, stamp: number, w: number, copy: number, set: number, n: number, emits: number, decay: number, spread: number, rise: number, riseRate: number, gone: number, lamps: number, albedo: number, share: number, passes: number, sweep: number, windX: number, windZ: number, windLow: number, windGain: number, windFullAt: number, windKeep: number): number;
  /** where the wind's share of each square metre of the plan is written before a step that has wind, and how many doubles it holds (bricks.rs WIND) */
  grid_wind(): number; grid_wind_cap(): number;
  grid_trans(g: number, stamp: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number, sigma: number): number;
  // the doors' step (rust/sim/src/doors.rs — src/game/doors.ts Doors.update, over the hand-over src/sim/doorshome.ts names)
  doors_addr(table: number): number; doors_cap(table: number): number;
  /** every leaf of the game in slot g stepped and placed, over world w: the noises listed */
  doors_update(g: number, w: number): number;
  // the furniture men shove (rust/sim/src/props.rs — src/game/props.ts PropSystem.update, over the hand-over src/sim/propshome.ts names)
  props_addr(table: number): number; props_cap(table: number): number;
  /** every thing of the game in slot g stepped, over world w: the noises listed */
  props_update(g: number, w: number): number;
  props_settle(g: number, w: number): number;
  // a leaf off its hinges, in the air (rust/sim/src/slabs.rs — src/game/doorblast.ts stepSlab, over the hand-over src/sim/slabshome.ts names)
  slabs_addr(table: number): number; slabs_cap(table: number): number;
  /** one slab of the game in slot g stepped, over world w: 0 = stepped · 1 = the host's to take (nothing written) · 2 = no such step */
  slabs_step(g: number, w: number): number;
  // what is thrown or dropped (rust/sim/src/items.rs — src/game/throwables.ts Throwables.update, over the hand-over src/sim/itemshome.ts names)
  items_addr(table: number): number; items_cap(table: number): number;
  /** the pool of the game in slot g walked from the hand-over's row down, over world w: 0 = every row taken · 1 = the hand-over's row is the host's · 2 = no such walk */
  items_update(g: number, w: number): number;
  /** the furniture's ray (props.ts PropSystem.raycast) over world w's copy of its parts: −1, or which thing of the level's list; t, the normal and the part → scratch 0 … 4 */
  lines_ray_things(w: number, a0: number, a1: number, a2: number, r0: number, r1: number, r2: number, tmax: number): number;
  // what falls and what it leaves (rust/sim/src/falls.rs — src/sim/fallshome.ts FallStore, whose rules game/shards.ts calls)
  falls_schedule(g: number, list: number, t: number, x: number, y: number, z: number, name: number, gain: number, rate: number, amount: number, who: number, event: number, level: number, dust: number, cap: number): number;
  falls_deposit(g: number, x: number, z: number, amount: number): void;
  /** the landings due: how many, each 10 doubles of the scratch */
  falls_due(g: number, list: number, time: number, level: number, gap: number): number;
  /** the level a footfall is filed at; how much glass was underfoot → scratch 0 */
  falls_step(g: number, x: number, z: number, level: number, crunchFull: number, grind: number, gain: number, add: number): number;
  falls_heard(g: number, list: number, time: number, gap: number): number;
  falls_clear(g: number, list: number, all: number, events: number): void; falls_cap(what: number): number;
  // the weather that play reads (rust/sim/src/wind.rs, weather.rs, air.rs, snow.rs — src/sim/weatherhome.ts is the host's half)
  wind_shape(i: number): number; wind_noise(seed: number, x: number): number;
  /** wind.ts jetVel, stormVel, stormVelRoofs (the roofs at scratch 0): the air → scratch 0‥2 */
  wind_jet(o0: number, o1: number, o2: number, d0: number, d1: number, d2: number, speed: number, reach: number, tanSpread: number, r0: number, dMax: number, floorY: number, p0: number, p1: number, p2: number): void;
  wind_storm(w0: number, w1: number, w2: number, i0: number, i1: number, i2: number, i3: number, a0: number, a1: number, a2: number, a3: number, p0: number, p1: number, p2: number): void;
  wind_storm_roofs(w0: number, w1: number, w2: number, n: number, p0: number, p1: number, p2: number): void;
  weather_col(i: number): number; weather_cap(i: number): number;
  weather_under_roof(g: number, x: number, z: number): number; weather_rain_bed(g: number, x: number, z: number): number; weather_hear_mask(g: number, time: number, x: number, z: number): number;
  weather_fog_tau(sigma: number, h: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number): number;
  weather_air_trans(g: number, a0: number, a1: number, a2: number, b0: number, b1: number, b2: number): number;
  weather_roofs_begin(): void; weather_roofs_box(cx: number, cy: number, cz: number, hx: number, hy: number, hz: number, ceiling: number): void; weather_roofs_end(): number; weather_roofs_seen(): number;
  weather_gap(g: number, first: number): number; weather_stroke_due(g: number, time: number): number; weather_strike(g: number, time: number, bearing: number, elevDeg: number, distKm: number, peak: number): void;
  weather_thunder_add(g: number, at: number, dist: number, bearing: number, loud: number): void; weather_thunder_due(g: number, i: number, time: number): number;
  weather_stroke_mean(n: number, a: number, b: number): number; weather_lot_step(g: number, dt: number): number;
  wet_open_sky(g: number, w: number, set: number, ask: number, time: number, x: number, z: number, head: number): number;
  wet_step(g: number, w: number, set: number, ask: number, time: number, dt: number, x: number, z: number, head: number): void;
  snow_bog_at(g: number, w: number, x: number, y: number, z: number): number;
  air_clear(g: number, stamp: number): void; air_begin(g: number, stamp: number, on: number): void; air_seed(g: number, stamp: number, leak: number): void; air_settle(g: number, stamp: number, w: number, set: number, ask: number): void; air_update(g: number, stamp: number, w: number, set: number, ask: number, dt: number): void; air_reread(g: number, stamp: number, w: number, set: number): void; air_table(g: number, stamp: number, which: number): number; weather_read(i: number): number;
}
/** every export this file expects, for the check that the built module and this face name the same things (tools/qa/rust-check.ts) */
export const SIM_EXPORTS: readonly (keyof SimExports)[] = ['memory', 'sim_abi', 'sim_scratch', 'sim_scratch_len', 'maths_sin', 'maths_cos', 'maths_tan', 'maths_asin', 'maths_acos', 'maths_atan', 'maths_atan2',
  'maths_exp', 'maths_log', 'maths_log2', 'maths_log10', 'maths_pow', 'maths_tanh', 'maths_hypot2', 'maths_hypot3', 'maths_hypot4', 'maths_hypot_n',
  'world_addr', 'world_cap', 'world_shape', 'world_leaves', 'world_things', 'world_glass', 'world_view', 'glass_open', 'lines_blocked', 'lines_leaves', 'lines_ray', 'lines_ray_things', 'collide_circle', 'collide_touches', 'collide_gather', 'fog_eyes', 'fog_see',
  'pose_addr', 'pose_rig', 'pose_blend', 'pose_run', 'pose_parent_rot', 'pose_turn', 'pose_ik', 'pose_sample', 'clip_begin', 'clip_track', 'clip_end', 'knob_addr', 'knob_len', 'layer_mv_trunk', 'layer_long_body', 'layer_life', 'layer_mv_pre', 'layer_stance_body', 'layer_stand_pose', 'layer_crouch_pose', 'layer_soft_knees', 'layer_run_arms', 'layer_run_plant', 'layer_sneak_body', 'layer_steady', 'layer_creep_body', 'layer_kick_body', 'layer_toes_up', 'layer_wall_body', 'layer_watch_slip', 'layer_watch_look', 'layer_mv_post_body', 'layer_turn_weigh', 'layer_lean_body', 'layer_gaze_keep', 'layer_sneak_arms', 'layer_carry_arms', 'layer_wall_arms', 'layer_act_pose', 'layer_wound_body', 'layer_wound_limbs', 'layer_long_arms', 'layer_mv_post_arms', 'layer_watch_arm', 'layer_creep_arms', 'layer_clutch_arm', 'layer_bound_arms', 'layer_reload_arm', 'layer_pistol_reload_arm', 'layer_hands_pass', 'layer_throw_layer', 'layer_big_layer', 'layer_hold_layer', 'layer_held_layer', 'layer_catch_arm', 'layer_door_layer', 'layer_work_layer', 'layer_ready_layer', 'layer_kit_layer', 'layer_taken_layer', 'layer_blow_legs', 'layer_blow_trunk', 'layer_blow_arms', 'layer_knife_pose', 'layer_cable_pose', 'layer_stealth_layer', 'layer_swap_pose', 'layer_lockpick_layer', 'layer_haul_layer', 'layer_torch_up', 'layer_radio_pose', 'layer_smoke_pose', 'layer_nv_pose', 'layer_signal_layer', 'layer_tell_layer',
  'sight_contact', 'light_lamps', 'light_cap', 'light_estimate', 'smoke_blobs', 'smoke_cap', 'smoke_trans', 'nav_addr', 'nav_cap', 'nav_path', 'nav_priced', 'nav_need', 'nav_resume', 'nav_drop', 'hear_direct', 'hear_route',
  'state_addr', 'state_len', 'state_games', 'state_layout', 'state_parts', 'state_part', 'dice_roll', 'men_schema', 'men_col', 'men_fold',
  'grid_reset', 'grid_clear', 'grid_dirty', 'grid_step', 'grid_wind', 'grid_wind_cap', 'grid_trans',
  'pose_new', 'pose_clear', 'doors_addr', 'doors_cap', 'doors_update', 'props_addr', 'props_cap', 'props_update', 'props_settle', 'slabs_addr', 'slabs_cap', 'slabs_step', 'items_addr', 'items_cap', 'items_update',
  'falls_schedule', 'falls_deposit', 'falls_due', 'falls_step', 'falls_heard', 'falls_clear', 'falls_cap',
  'wind_shape', 'wind_noise', 'wind_jet', 'wind_storm', 'wind_storm_roofs', 'weather_col', 'weather_cap', 'weather_under_roof', 'weather_rain_bed', 'weather_hear_mask', 'weather_fog_tau', 'weather_air_trans', 'weather_roofs_begin', 'weather_roofs_box', 'weather_roofs_end', 'weather_roofs_seen', 'weather_gap', 'weather_stroke_due', 'weather_strike', 'weather_thunder_add', 'weather_thunder_due', 'weather_stroke_mean', 'weather_lot_step', 'wet_open_sky', 'wet_step', 'snow_bog_at', 'air_clear', 'air_begin', 'air_seed', 'air_settle', 'air_update', 'air_reread', 'air_table', 'weather_read'];

/** A RUNTIME WITH NO WEBASSEMBLY (a browser whose policy turns a site's JIT off has none) DOES NOT PLAY: the simulation is the module's and nothing else
 *  works it out. The page shows these words in place of the game, before anything is built (main.ts); they are the maintainer's, as he chose them. */
export const NO_WASM_WORDS = "This game needs WebAssembly, and it's turned off in this browser. Try another browser.";
export class NoWasmError extends Error { constructor(why: string) { super(why); this.name = 'NoWasmError'; } }
export const noWasm = (): boolean => typeof (globalThis as { WebAssembly?: unknown }).WebAssembly === 'undefined';
let X: SimExports | null = null, why: string | null = null;
/** how long the one compile-and-start took, in milliseconds (0 until it has happened): the page's boot line and the check print it */
export let simBootMs = 0;
function start(): SimExports {
  const M = startModule(SIM_WASM), x = M.exports as unknown as SimExports;   // (load.ts: nothing is handed in — the module imports nothing)
  if (x.sim_abi() !== SIM_ABI) throw new Error(`the Rust simulation is version ${x.sim_abi()} and src/sim/sim.ts was written against ${SIM_ABI}: bun run tools/sim/build.ts`);
  simBootMs = M.ms;
  return x;
}
/** WHY THIS RUNTIME CANNOT RUN THE MODULE ('' = it can, and it is started). Asked once and kept: the module is started here, so a runtime that has no
 *  WebAssembly (a browser whose policy turns its JIT off has none), one that will not compile it (a page whose policy forbids it) and a built.ts of another
 *  version are all the one answer. Such a page does not play: the simulation is the module's alone. With no WebAssembly it shows the card (NO_WASM_WORDS);
 *  any other reason is a fault of the build and is shown as one. */
export function simWhyNot(): string {
  if (why === null) { try { X = start(); why = ''; } catch (e) { why = String((e as Error)?.message ?? e); } }
  return why;
}
/** the module, started on the first call (throws what simWhyNot says where it cannot run) */
export function sim(): SimExports {
  const no = simWhyNot(); if (no) throw new Error(no);
  return X!;
}

// ---- the module's memory
/** is `b` the module's memory (false where the module was never started: asking does not start it) — what tells a typed array over it from one of the game's own */
export function simOwns(b: ArrayBufferLike): boolean { return X !== null && f64().buffer === b; }
// THE VIEWS OVER THE MODULE'S MEMORY are made once and kept. A memory that grew would leave them over a buffer the engine has DETACHED, and a view over
// a detached buffer has length 0: that is the test an ask makes (a property of the array, read in place), where it was a call into the engine for
// `memory.buffer` — asked a thousand times a tick by the senses (a lamp, an eye, a blob each write through one). Nothing in the module allocates, so in
// play the memory never does grow (the one memory.grow in its bytes is the standard library's allocator, which rust/prim links and nothing calls).
let F64: Float64Array | null = null, U8: Uint8Array | null = null, U32: Uint32Array | null = null, I32: Int32Array | null = null, F32: Float32Array | null = null;
function views(): Float64Array { const b = sim().memory.buffer; U8 = new Uint8Array(b); U32 = new Uint32Array(b); I32 = new Int32Array(b); F32 = new Float32Array(b); return F64 = new Float64Array(b); }
/** the module's memory as doubles (index = address / 8). Ask again after a call into the module that could allocate: none can today, and a view kept over a
 *  memory that grew would read nothing and write nowhere without a word. */
export function f64(): Float64Array { const v = F64; return v !== null && v.length !== 0 ? v : views(); }
/** the module's memory as bytes (as f64: not to be kept across a call that can allocate) */
export function u8(): Uint8Array { f64(); return U8!; }
/** the module's memory as 32-bit words, unsigned and signed (index = address / 4; as f64: not to be kept across a call that can allocate) */
export function u32(): Uint32Array { f64(); return U32!; }
export function i32(): Int32Array { f64(); return I32!; }
/** the module's memory as singles (index = address / 4): a stored pose (as f64: not to be kept across a call that can allocate) */
export function f32(): Float32Array { f64(); return F32!; }

// ---- the game's own maths, worked out in Rust
let rustMaths: Maths | null = null;
/** the fourteen and the three lengths as the module works them out: THE set RUST.maths puts in Math's place (and the one tools/qa/rust-check.ts folds) */
export function mathsOfRust(): Maths {
  if (rustMaths) return rustMaths;
  const x = sim(), at = x.sim_scratch() / 8, cap = x.sim_scratch_len();
  const hypot = function (): number {
    const n = arguments.length;
    if (n === 2) return x.maths_hypot2(+arguments[0], +arguments[1]);
    if (n === 3) return x.maths_hypot3(+arguments[0], +arguments[1], +arguments[2]);
    if (n === 4) return x.maths_hypot4(+arguments[0], +arguments[1], +arguments[2], +arguments[3]);
    if (n > cap) return kahanN(arguments as unknown as ArrayLike<number>);   // (more terms than the scratch holds: the same sum, here)
    const m = f64(); for (let i = 0; i < n; i++) m[at + i] = +arguments[i];
    return x.maths_hypot_n(n);
  } as (...v: number[]) => number;
  return rustMaths = { sin: x.maths_sin, cos: x.maths_cos, tan: x.maths_tan, asin: x.maths_asin, acos: x.maths_acos, atan: x.maths_atan, atan2: x.maths_atan2, exp: x.maths_exp, log: x.maths_log,
    log2: x.maths_log2, log10: x.maths_log10, pow: x.maths_pow, tanh: x.maths_tanh, hypot, hyp2: x.maths_hypot2, hyp3: x.maths_hypot3, hyp4: x.maths_hypot4 };
}

// ---- WHICH SYSTEMS RUN IN RUST
// (every row but the maths is UP as this module loads: that is how the game is played, on every page and in every tool, whoever stands it up and whether
//  or not anyone calls rowsUp. A runtime with no WebAssembly does not play: the page shows its card, main.ts)
let mathsOn = false, linesOn = true, circlesOn = true, fogOn = true, posesOn = true, diceOn = true;
let sightOn = true, lightOn = true, smokeOn = true, soundOn = true, pathsOn = true, walkOn = true;
let doorsOn = true, fallsOn = true, propsOn = true, slabsOn = true, itemsOn = true, raysOn = true;
/** A row a system: on = that system of the update runs in the Rust module, off = in TypeScript as it always did. THE TWO ARE THE SAME GAME TO THE BIT — that is
 *  what lets a system move at all, and tools/qa/rust-check.ts holds each row to it (whole missions played both ways, every tick's raw bits). So a row is never
 *  a rule of play: a recording made with one on replays with it off, and the frozen stand's fixtures do not care. A row is ON in the page once its system is
 *  proven and faster there; until then it is a thing a check raises.
 *
 *  maths   the game's own maths (src/math/exact.ts, when EXACT.on has put them in Math's place) from Rust instead: every sin, cos, atan2, exp … and every
 *          length of math/hyp.ts the update asks for crosses into the module. OFF in the page and on both stands (no word raises it: a check holds it up in its own process, tools/qa/headless.ts hold): a
 *          call from TypeScript into the module costs more than the dozen operations it saves (bun: sin 27 ns here, 44 ns there; V8 about level), so this row
 *          is the proof that the two arithmetics are one — and the Rust systems that follow call the Rust maths with no crossing.
 *  lines   the box world's line tests (src/scene/collision.ts segmentBlocked, shotBlocked through it, segmentBlockedDynamic: every line of sight, every
 *          straight run of a sound, every reach and line of fire) from the module's copy of the box world (src/scene/boxMirror.ts) — INSIDE A TICK: there the
 *          copy is proven against the boxes as the tick opens; a question asked outside one (the page's own, a tool's) is answered in TypeScript as it always
 *          was.
 *  rays    the box world's ray (collision.ts raycast: the nearest thing along a ray, how far and by which face — a round's flight, a thrown can's bounce,
 *          a torn leaf's slide) from the same copy, inside a tick. A ROW OF ITS OWN, though it is lines.rs that answers: the lines' row can be on in the
 *          page while this one waits its turn. A rule stepped in the module (the slabs', the thrown things') asks its rays there whatever this row says.
 *  circles a circle against the box world (collision.ts collideCircle, circleTouches, gather: a body's footprint pushed out of the statics, what a prop or a
 *          rag doll rests against) from the same copy, inside a tick.
 *  fog     the fog of war's sight pass (game/fogofwar.ts update: which of the team's eyes has a line to each man and each body): the loop over the eyes and
 *          its lines in the module, a man a question, inside a tick and where the game's own maths stand (EXACT.on: the lengths are worked out there).
 *  poses   the poses a character keeps (src/anim/skeleton.ts Pose, PoseSet) live in its game's block of the module's memory (rust/sim/src/poses.rs, a tenant:
 *          game/posehome.ts), and the posing functions (a clip sampled, a pose blended, run forward, a node turned, the two-bone solve) are asked of the module, of a kept
 *          pose where it lives and of a function's scratch pose by a copy onto the bench: A PROOF, not a speed-up (a call a question costs more than the
 *          question), until the animator's layers that ask them are Rust too. Where the game's own maths stand.
 *  dice    the game's own die (src/math/dice.ts SimDice: the word every draw of the simulation comes off, and the count of draws) kept in the game's block of
 *          the module's memory (src/sim/state.ts) and rolled there. THE FIRST STATE OF THE GAME THAT LIVES IN THE MODULE: it is what the rollback snapshot,
 *          the saves in memory and the state's word were taught to carry a block for. A proof like `maths` (a draw from TypeScript then costs a call), and the
 *          die has to be there before any rule that rolls it is. A game takes the row as its next tick opens.
 *  walk    the animator's layers (game/character.ts CharacterAnimator: what makes a blend of clips a man who leans into his walk, breathes, keeps his soles
 *          where he put them, flinches, holds a wall), a layer at a time (rust/sim/src/layers.rs): each asked once on the pose where it lives, so it needs
 *          `poses` at home and the game's own maths; the TypeScript method stays behind the row as `<name>Own`. Not yet: the two holds, the big tools, the throw, and the update itself.
 *
 *  THE SENSES, a row each (asked through src/scene/collision.ts; every one inside a tick and where the game's own maths stand, since each works its lengths out
 *  in the module):
 *  sight   an eye on a man (game/guards.ts eyeContact: the cone, then the lines to his chest and his head) as one question.
 *  light   the light on a point (guards.ts lightEstimateAt: every lamp in reach, each shadowed by a line) as one question a point, the host writing the
 *          lamps that may be asked.
 *  smoke   the game's own smoke along a line (game/sense.ts CpuSmoke.trans, what SENSE.cpu sees through) as one question a line.
 *  sound   how a sound reaches a listener (scene/collision.ts SoundPaths.propagate: straight, over the top, or round by the floor).
 *  paths   a way across the floor (collision.ts NavGrid.findPath: the A* and its smoothing; with a penalty the search waits for the caller's prices, nav.rs find_priced).
 *
 *  THE THINGS of a level, a row a family (each keeps its state in the game's block from the moment the game is made — sim/state.ts — and, while its row
 *  is on, its step runs there and WRITES the box world's rows: the Box objects the picture draws follow, scene/boxMirror.ts `follow`):
 *  doors   the doors' step (game/doors.ts Doors.update: what was written onto each leaf, the men and bodies in its swing, its swing, stops, closer, latch and
 *          lock's clocks, and where its box stands) from rust/sim/src/doors.rs, the leaves' own numbers at home in the block (sim/doorshome.ts). The state is
 *          at home on either maths; the step runs there inside a tick, where the game's own maths stand and the box world's copy does.
 *  falls   what falls and what it leaves (sim/fallshome.ts FallStore: the landings the simulation has scheduled for things that let go, and the litter map of
 *          broken glass a footfall is louder on) kept in the game's block and worked there (rust/sim/src/falls.rs). Either maths.
 *  props   the furniture men shove (game/props.ts PropSystem.update: each chair, carton, bin and plant coasting, pushed, knocking and coming to rest, and
 *          the scrapes and knocks of it) from rust/sim/src/props.rs, each thing's own numbers at home in the block (sim/propshome.ts). It writes NO row of
 *          the box world (furniture is not in it): the boxes the picture draws are placed from the columns, as they were. The state is at home on either
 *          maths; the step runs there inside a tick, where the game's own maths stand and the box world's copy does.
 *  slabs   a leaf off its hinges, in the air (game/doorblast.ts stepSlab: the loose slab flying, toppling, stopped by what its rays find, slamming flat
 *          and coming to rest clear of every static) from rust/sim/src/slabs.rs, each slab's own numbers at home in the block (sim/slabshome.ts). It
 *          writes no row of the box world: the host poses the slab's box from the columns after each slab's step, as it did. A pane its ray meets is judged
 *          there (rust/sim/src/glass.rs); nothing of play is handed back.
 *  items   what is thrown or dropped (game/throwables.ts Throwables.update: a can, a grenade, a magazine, a gun out of a dying hand — ageing, falling,
 *          bouncing off walls, leaves and furniture, coming to rest, dropping off a chair that moved) from rust/sim/src/items.rs, each thing's own
 *          numbers at home in the block (sim/itemshome.ts). The pool is walked in one call; what a thing does to anything else (a bounce said, a share
 *          of momentum for a chair, its time up) is listed and done by the host after. A pane a ray meets and lying snow under a bounce are asked
 *          there (glass.rs; items.rs bog_at over the level's snow); nothing of play is handed back. */
export const RUST = {
  get maths(): boolean { return mathsOn; },
  set maths(v: boolean) { const on = !!v && !simWhyNot(); if (on === mathsOn) return; mathsFrom(on ? mathsOfRust() : null); mathsOn = on; },   // (a runtime that cannot run the module keeps the row off: the same game either way)
  get lines(): boolean { return linesOn; },
  set lines(v: boolean) { linesOn = !!v && !simWhyNot(); },
  get circles(): boolean { return circlesOn; },
  set circles(v: boolean) { circlesOn = !!v && !simWhyNot(); },
  get fog(): boolean { return fogOn; },
  set fog(v: boolean) { fogOn = !!v && !simWhyNot(); },
  get poses(): boolean { return posesOn; },
  set poses(v: boolean) { posesOn = !!v && !simWhyNot(); },
  get dice(): boolean { return diceOn; },
  set dice(v: boolean) { diceOn = !!v && !simWhyNot(); },
  get sight(): boolean { return sightOn; },
  set sight(v: boolean) { sightOn = !!v && !simWhyNot(); },
  get light(): boolean { return lightOn; },
  set light(v: boolean) { lightOn = !!v && !simWhyNot(); },
  get smoke(): boolean { return smokeOn; },
  set smoke(v: boolean) { smokeOn = !!v && !simWhyNot(); },
  get sound(): boolean { return soundOn; },
  set sound(v: boolean) { soundOn = !!v && !simWhyNot(); },
  get paths(): boolean { return pathsOn; },
  set paths(v: boolean) { pathsOn = !!v && !simWhyNot(); },
  get doors(): boolean { return doorsOn; },
  set doors(v: boolean) { doorsOn = !!v && !simWhyNot(); },
  get falls(): boolean { return fallsOn; },
  set falls(v: boolean) { fallsOn = !!v && !simWhyNot(); },
  get walk(): boolean { return walkOn; },
  set walk(v: boolean) { walkOn = !!v && !simWhyNot(); },
  get props(): boolean { return propsOn; },
  set props(v: boolean) { propsOn = !!v && !simWhyNot(); },
  get slabs(): boolean { return slabsOn; },
  set slabs(v: boolean) { slabsOn = !!v && !simWhyNot(); },
  get items(): boolean { return itemsOn; },
  set items(v: boolean) { itemsOn = !!v && !simWhyNot(); },
  get rays(): boolean { return raysOn; },
  set rays(v: boolean) { raysOn = !!v && !simWhyNot(); },
};
/** A QUESTION THE TYPESCRIPT ANSWERED WHILE ITS ROW WAS UP, COUNTED (`?fell` on the page, `TPS_FELL=1` on the harness; `=throw` stops at the first).
 *  A row that is on sends its questions to the module only where the module can answer: inside a tick with the game's copy standing, under a cap, with a
 *  shape it knows. Everywhere else the TypeScript still answers, which is why it cannot be deleted yet. This is the list of those places, by row and by
 *  reason, as a run met them: zero for a row is what lets its TypeScript go. Off (the default) it costs one test at a place that was falling back anyway.
 *  A tool of the build's, never a rule of play: it counts and changes no answer. */
/** who asked: the first `n` callers on the stack from frame `skip` on (of the next `span` frames, if given), outside the files `not` names — for the
 *  count below and for the box world's count of questions asked alone (scene/collision.ts) */
export function callers(skip: number, n: number, not: RegExp, span = Infinity): string {
  return (new Error().stack ?? '').split('\n').slice(skip, skip + span).map(l => l.trim().replace(/^at /, '').replace(/\(?[^()\s]*\/(src|tools)\//, '($1/').replace(/:\d+\)?$/, '')).filter(l => !not.test(l)).slice(0, n).join(' < ');
}
export const FELL = { on: false, hard: false, who: 0, n: {} as Record<string, number>, red: false, reds: {} as Record<string, number> };
export function fell(row: string, why: string): void {
  let k = row + ' · ' + why;
  if (FELL.who) {   // (`=who`: …and who asked — the first two callers outside the box world's own files, `=who6` the first six, so the count says which code still asks outside a tick)
    k += ' · asked by ' + callers(3, FELL.who, /scene\/(collision|boxMirror)\.ts/);
  }
  FELL.n[k] = (FELL.n[k] ?? 0) + 1;
  if (FELL.hard) throw new Error(`rust: the TypeScript answered for the row "${row}": ${why}`);
}
/** THE ROWS THAT ARE ONE WAY IN: no run of the gate reaches their TypeScript (the box world's lines and circles, the fog's pass, the posing
 *  functions, the animator's layers). A count on one of them is a way back in that somebody opened, so the tool that counted says so and ends red (headless.ts; the page's
 *  check reads `fellSealed`). A family adds its row here in the step that closes its last fallback, and its TypeScript can go in the step after.
 *  `rays` joined when a pane's open question moved to the module (rust/sim/src/glass.rs): until then a ray whose nearest thing was a pane the host
 *  said was open there flew on in the TypeScript walk, some five hundred a mission with glass in it.
 *  THE SENSES', all five: `sight`, `light` and `smoke` (every caller's sight line passes glass, and a level's lamps and the game's blobs are fewer
 *  than a question carries), and `sound` and `paths`, whose lines came to a pane and handed the question back, for the ray's reason, until the
 *  panes were in the module's copy (`collision.ts paneAsk`). Nothing of the five is counted over the missions on either stand.
 *  THE RULES OF THE THINGS joined when their last ways back were closed (a pane and lying snow asked in the module, a ray of no length, the landing a
 *  checkpoint files, a pool that grew, a level too big for a pool stopped as it loads): the doors, what falls, the furniture, a leaf off its hinges, the
 *  thrown things — and RULES_ROW, the copy of the box world refused to any of them. */
export const RULES_ROW = 'a rule\'s step (doors, furniture, leaves, thrown things)';
/** A TWIN'S BODY REACHED WITH NO CHOICE MADE: a rule of the things answered by its TypeScript where nothing asks whether the module could (so no `ruleFell`
 *  says it). Counted under this name, a row of its own, wherever one is known, until the place has a choice like any other; SEALED, so a run that
 *  reaches one ends red. KNOWN TODAY: none (the two that were — the furniture's settle as a level is stood up, a ray into the furniture — each have a
 *  choice now, and their way back is the furniture's own row). */
export const TWIN_ROW = "a twin's body, no choice made";
export const SEALED: readonly string[] = ['lines', 'circles', 'fog', 'poses', 'rays', 'walk', 'sight', 'light', 'smoke', 'sound', 'paths', 'doors', 'falls', 'props', 'slabs', 'items', RULES_ROW, TWIN_ROW];
/** A RULE OF `row` WAS STEPPED IN TYPESCRIPT WHILE THE ROW IS UP (a family's fallback site says so, with the reason in plain words): counted, except
 *  where the row is idle by its own rule — one that works lengths and angles out answers only under the game's own maths (OWN_MATHS, below) */
export function ruleFell(row: string, why: string): null { if (FELL.on && (EXACT.on || !OWN_MATHS.includes(row))) fell(row, why); return null; }
/** THE SEAL'S RED PATH FOR A RULE'S ROW (`FELL.red`, raised by a tool under FELL_BREAK=1 and by nothing else): the first step of each row, and every
 *  hundredth after, is sent to its TypeScript and counted, so a run with the break up reaches every sealed rule's row that stepped in it, by name. The
 *  TypeScript's step is the same game; the count says a way back was taken, and the tool must end red. → true: this step is this side's. */
export function ruleRed(row: string): boolean {
  if (!FELL.red) return false;
  const n = FELL.reds[row] = (FELL.reds[row] ?? 0) + 1; if (n % 100 !== 1) return false;
  fell(row, 'the red path: a step sent to its TypeScript'); return true;
}
/** the sealed rows the count has met */
export const fellSealed = (): string[] => SEALED.filter(r => Object.keys(FELL.n).some(k => k.startsWith(r + ' · ')));
/** the count's switches from a word (`1`, `throw`, `who`, `who6`; `0` = off): the harness's TPS_FELL, the page's ?fell */
export function fellWord(w: string): void { if (w === '0') { FELL.on = false; return; }   // (`0` is off, as every word of this family reads it)
  FELL.on = true; FELL.hard = w === 'throw'; const m = /^who(\d*)$/.exec(w); FELL.who = m ? Number(m[1] || 2) : 0; }
/** the count so far, the busiest first, as one line a row and reason */
export const fellLines = (): string[] => Object.entries(FELL.n).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${v} × ${k}`);
/** the rows that work their lengths and angles out in the module, and so answer only where the game's own maths stand in Math's place (EXACT.on) */
const OWN_MATHS: readonly string[] = ['maths', 'fog', 'poses', 'sight', 'light', 'smoke', 'sound', 'paths', 'doors', 'walk', 'props', 'slabs', 'items'];
/** THE ROWS THAT ARE ON WHERE THE GAME IS PLAYED (the page and both of the harness's stands, and no word of an address or of the environment moves
 *  one: rowsUp below): EVERY row but the maths. Each is the same game to the bit in the module, by its differential and by the missions under both
 *  engines (tools/qa/rust-check.ts). They are not all cheaper there yet: the box world's lines, the furniture and the poses with the walk's layers are; a
 *  row whose callers are still TypeScript pays a crossing a question (the die, a door, a sound's path, a route) and is on all the same, because the
 *  TypeScript behind it is going and the game must be played the way it will be kept (docs/agents/rust.md, "Rows that are on", has every number).
 *  The maths stay TypeScript (math/exact.ts): a sine a crossing is the dearest crossing there is, and the module works its own out inside itself.
 *  The rows that work lengths and angles out (OWN_MATHS) answer only under the game's own maths, which is the only maths the game is played on. A
 *  runtime with no WebAssembly does not play: the page says so on a card (main.ts, NO_WASM_WORDS) and builds no game. */
export const RUST_ON: readonly (keyof typeof RUST)[] = ['lines', 'circles', 'fog', 'poses', 'dice', 'sight', 'light', 'smoke', 'sound', 'paths', 'doors', 'falls', 'walk', 'props', 'slabs', 'items', 'rays'];
/** is `row` one of them */
export const rustOn = (row: string): boolean => (RUST_ON as readonly string[]).includes(row);
/** EVERY ROW AS THE GAME IS PLAYED (RUST_ON up, the maths down): what the page's boot and the harness's stand-up write, and nothing else writes a row in
 *  play. A row is NOT a switch: no word of an address, of a tool's environment or of a recording moves one. The table stays a table only so that a
 *  check that still holds the two languages against each other can put its own row down inside its own process, until that family's TypeScript is gone. */
export function rowsUp(): void { for (const r of rustRows()) rustRow(r, rustOn(r)); }
/** THE GAME'S OWN MATHS, ON (math/exact.ts): what the page's boot writes beside the rows. The harness writes its own (standup.ts pinMaths), so that a
 *  check's own arm can take them out for a red path; nothing else in play writes them */
export function mathsUp(): void { EXACT.on = true; }
/** THE WORDS THAT WERE THE ROWS' SWITCHES AND THE MATHS', RETIRED: a tool or an address that still says one is told so, by name (a word that is silently
 *  nothing is a run that is not what its caller thinks it is). Two lists, since they are two places: a tool's environment, and a page's address (the
 *  rows' names are spelt out so that no later word that merely begins `rust` is taken for one) */
export function retiredEnvWord(w: string): boolean { return /^(TPS_RUST[A-Z]*|TPS_EXACT)$/.test(w); }
export function retiredAddressWord(w: string): boolean { return w === 'rust' || w === 'noexact' || (w.startsWith('rust') && rustRows().includes(w.slice(4))); }
/** the rows by name, for what treats them all alike (a check's own arm, tools/qa/headless.ts hold; the checks that walk every row) */
export const rustRows = (): string[] => Object.keys(RUST);
export function rustRow(row: string, on: boolean): void { if (rustRows().includes(row)) (RUST as Record<string, boolean>)[row] = on; }
/** WHAT EACH ROW HAS RUNNING IN THE MODULE NOW: a row can be on and idle (the maths cross only while the game's own maths stand in Math's place) */
function running(): string[] { const R = RUST as Record<string, boolean>; return rustRows().filter(r => R[r]).map(r => EXACT.on || !OWN_MATHS.includes(r) ? r : `${r} (idle: the game's own maths are off)`); }
/** one line for the page's console: the module started (or why not), and which rows are on. Starts the module — a page that cannot is known at boot, not at
 *  the first system that needs it. */
export function simSay(): string {
  const no = simWhyNot(); if (no) return `rust simulation: NOT RUNNING — ${no}; the game does not play here`;
  const on = running();
  return `rust simulation: ${SIM_BYTES} bytes, started in ${simBootMs.toFixed(1)} ms · in Rust: ${on.length ? on.join(' ') : 'nothing yet (rows: ' + Object.keys(RUST).join(' ') + ')'}`;
}
