#![allow(unused_variables)]
//! Every function and method of `scene_collision`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/scene/collision.ts:15 lineBox)
pub fn line_box(g: &mut crate::game::Game, a: V3, rd: V3, b: BoxId) -> Option<crate::scene_collision::LineBox> { todo!() }

/// (src/scene/collision.ts:54 segClear)
pub fn seg_clear(g: &mut crate::game::Game, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, len: f64, b: BoxId, pad: f64) -> bool { todo!() }

/// (src/scene/collision.ts:65 rayBoxHits)
pub fn ray_box_hits(g: &mut crate::game::Game, a0: f64, a1: f64, a2: f64, r0: f64, r1: f64, r2: f64, b: BoxId, tmax: f64) -> bool { todo!() }

/// (src/scene/collision.ts:88 footBand)
pub fn foot_band(g: &mut crate::game::Game, y0: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:97 noNumber)
pub fn no_number(ro: Option<V3>, rd: Option<V3>, tmax: Option<f64>) -> crate::Opaque { todo!() }

/// (src/scene/collision.ts:125 senseName)
pub fn sense_name(bit: f64) -> String { todo!() }

/// (src/scene/collision.ts:133 StaticCollision.constructor)
pub fn new(g: &mut crate::game::Game, boxes: &[BoxId], of: Option<&str>) -> crate::scene_collision::StaticCollision { todo!() }

/// (src/scene/collision.ts:154 StaticCollision.addStatic)
pub fn add_static(g: &mut crate::game::Game, b: BoxId, footprint: Option<BoxId>) -> f64 { todo!() }

/// (src/scene/collision.ts:162 StaticCollision.footCells)
pub fn foot_cells(g: &mut crate::game::Game, f: BoxId) -> [f64; 4] { todo!() }

/// (src/scene/collision.ts:171 StaticCollision.footEntries)
pub fn foot_entries(g: &mut crate::game::Game, f: BoxId) -> f64 { todo!() }

/// (src/scene/collision.ts:173 StaticCollision.truncateStatics)
pub fn truncate_statics(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/collision.ts:183 StaticCollision.tickOpen)
pub fn tick_open(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:185 StaticCollision.stand)
pub fn stand(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:189 StaticCollision.rulesWorld)
pub fn rules_world(g: &mut crate::game::Game, dyn_: &[BoxId]) { todo!() }

/// (src/scene/collision.ts:193 StaticCollision.rulesStatics)
pub fn rules_statics(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:196 StaticCollision.rulesPanes)
pub fn rules_panes(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/collision.ts:198 StaticCollision.fellWhy)
pub fn fell_why(g: &mut crate::game::Game) -> String { todo!() }

/// (src/scene/collision.ts:200 StaticCollision.footOf)
pub fn foot_of(g: &mut crate::game::Game, y0: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:202 StaticCollision.tickShut)
pub fn tick_shut(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:211 StaticCollision.window)
pub fn window<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/collision.ts:220 StaticCollision.ready)
pub fn ready(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:223 StaticCollision.lone)
pub fn lone(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/collision.ts:229 StaticCollision.wrote)
pub fn wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:239 StaticCollision.aside)
pub fn aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_collision::AsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/collision.ts:259 StaticCollision.gather)
pub fn gather(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:263 StaticCollision.gatherAsk)
pub fn gather_ask(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:273 StaticCollision.gatherHere) TWIN → sim::collide::gather(w, x0, z0, x1, z1, y0, y1, cap) -> usize
pub fn gather_here(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:291 StaticCollision.circleTouches)
pub fn circle_touches(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:295 StaticCollision.circleTouchesAsk)
pub fn circle_touches_ask(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:305 StaticCollision.circleTouchesHere) TWIN → sim::collide::touches(w, x, z, r, y0, y1, cap) -> i32
pub fn circle_touches_here(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:317 StaticCollision.collideCircle)
pub fn collide_circle(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:321 StaticCollision.collideCircleAsk)
pub fn collide_circle_ask(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:331 StaticCollision.collideCircleHere) TWIN → sim::collide::circle(w, px, pz, r, y0, y1, iterations) -> u32
pub fn collide_circle_here(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: f64) -> bool { todo!() }

/// (src/scene/collision.ts:373 StaticCollision.clampToWorld)
pub fn clamp_to_world(g: &mut crate::game::Game, p: &mut V3, r: f64) -> bool { todo!() }

/// (src/scene/collision.ts:381 StaticCollision.glassOpen)
pub fn glass_open(g: &mut crate::game::Game) -> Option<crate::FnField> { todo!() }

/// (src/scene/collision.ts:387 StaticCollision.glassRule)
pub fn glass_rule(g: &mut crate::game::Game, rule: Option<&mut dyn FnMut(&mut crate::game::Game, f64, V3) -> bool>, rows: Option<&mut dyn FnMut(&mut crate::game::Game) -> crate::scene_box_mirror::PaneSource>) { todo!() }

/// (src/scene/collision.ts:388 StaticCollision.glassPanes)
pub fn glass_panes(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneSource> { todo!() }

/// (src/scene/collision.ts:393 StaticCollision.glassBegin)
pub fn glass_begin(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:394 StaticCollision.glassEnd)
pub fn glass_end(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:396 StaticCollision.glassWrote)
pub fn glass_wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:398 StaticCollision.glassViewed)
pub fn glass_viewed(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:405 StaticCollision.paneAsk)
pub fn pane_ask(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/collision.ts:407 StaticCollision.paneFell)
pub fn pane_fell(g: &mut crate::game::Game, row: &str, ask: f64) { todo!() }

/// (src/scene/collision.ts:419 StaticCollision.segmentBlockedDynamic)
pub fn segment_blocked_dynamic(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:423 StaticCollision.segmentBlockedDynamicAsk)
pub fn segment_blocked_dynamic_ask(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:442 StaticCollision.thingsRay)
pub fn things_ray(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::ThingsRay> { todo!() }

/// (src/scene/collision.ts:446 StaticCollision.thingsAsk)
pub fn things_ask(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::ThingsAsk> { todo!() }

/// (src/scene/collision.ts:456 StaticCollision.thingsBoth)
pub fn things_both(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64, t: Option<crate::scene_collision::ThingsBothT>) { todo!() }

/// (src/scene/collision.ts:463 StaticCollision.compared)
pub fn compared(g: &mut crate::game::Game, set: f64, ok: bool, what: &str, a: V3, b: V3, p: f64, q: f64) { todo!() }

/// (src/scene/collision.ts:470 StaticCollision.dynamicHit)
pub fn dynamic_hit(g: &mut crate::game::Game, a: V3, r0: f64, r1: f64, r2: f64, len: f64, need_flags: f64) -> bool { todo!() }

/// (src/scene/collision.ts:498 StaticCollision.segmentBlocked)
pub fn segment_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool { todo!() }

/// (src/scene/collision.ts:502 StaticCollision.segmentBlockedAsk)
pub fn segment_blocked_ask(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedAskPanesItem>>) -> bool { todo!() }

/// (src/scene/collision.ts:529 StaticCollision.ownLines)
pub fn own_lines(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/collision.ts:533 StaticCollision.sightOpen)
pub fn sight_open(g: &mut crate::game::Game, n: f64) -> bool { todo!() }

/// (src/scene/collision.ts:536 StaticCollision.sightEye)
pub fn sight_eye(g: &mut crate::game::Game, k: f64, pos: V3, look: V3, look_len: f64, cos_half: f64, near: f64, full: bool) { todo!() }

/// (src/scene/collision.ts:543 StaticCollision.eyesSee)
pub fn eyes_see(g: &mut crate::game::Game, order: f64, n: f64, want: f64, kind: f64, through: f64, a: V3, b: V3) -> f64 { todo!() }

/// (src/scene/collision.ts:556 StaticCollision.senseSet)
pub fn sense_set(g: &mut crate::game::Game, bit: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:559 StaticCollision.senseCompared)
pub fn sense_compared(g: &mut crate::game::Game, set: f64, ok: bool, what: &str, a: V3, b: V3) { todo!() }

/// (src/scene/collision.ts:562 StaticCollision.eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> Option<crate::scene_collision::EyeContact> { todo!() }

/// (src/scene/collision.ts:567 StaticCollision.eyeContactAsk)
pub fn eye_contact_ask(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactAskE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> Option<crate::scene_collision::EyeContactAsk> { todo!() }

/// (src/scene/collision.ts:583 StaticCollision.lampsBegin)
pub fn lamps_begin(g: &mut crate::game::Game, p: V3, shade: Option<&crate::scene_collision::LampsBeginShade>, k: Option<crate::scene_collision::SmokeNumbers>) -> bool { todo!() }

/// (src/scene/collision.ts:590 StaticCollision.lampsOver)
pub fn lamps_over(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:592 StaticCollision.lampsPart)
pub fn lamps_part(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/collision.ts:600 StaticCollision.lamp)
pub fn lamp(g: &mut crate::game::Game, pos: V3, range: f64, radius: f64, kind: f64, dir: V3, outer_deg: f64, inner_deg: f64, color: V3, intensity: f64) -> bool { todo!() }

/// (src/scene/collision.ts:610 StaticCollision.lampsOn)
pub fn lamps_on(g: &mut crate::game::Game, acc: &mut Vec<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:617 StaticCollision.blobsBegin)
pub fn blobs_begin(g: &mut crate::game::Game, n: f64) -> bool { todo!() }

/// (src/scene/collision.ts:620 StaticCollision.blobsFit)
pub fn blobs_fit(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/collision.ts:621 StaticCollision.blob)
pub fn blob(g: &mut crate::game::Game, x: f64, y0: f64, z: f64, r2: f64, age: f64, mass: f64) { todo!() }

/// (src/scene/collision.ts:622 StaticCollision.blobsAlong)
pub fn blobs_along(g: &mut crate::game::Game, a: V3, b: V3, sigma: f64, spread: f64, widest: f64, rise: f64, rise_rate: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:637 StaticCollision.gridStep)
pub fn grid_step(g: &mut crate::game::Game, st: &mut crate::Opaque, pending: &[f64], n: f64, k: crate::scene_collision::GridStepK, glow: Option<crate::scene_collision::GridStepGlow>, wind: Option<&crate::scene_collision::GridWind>) -> bool { todo!() }

/// (src/scene/collision.ts:663 StaticCollision.segmentBlockedHere) TWIN → sim::lines::blocked(w, a0, a1, a2, b0, b1, b2, len, shrink, through, set, ask) -> i32; the named lines: sight_line, solid_line, leaf_line, man_hidden, leaves_blocked
pub fn segment_blocked_here(g: &mut crate::game::Game, a: V3, b: V3, shrink: f64, through: f64, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedHerePanesItem>>) -> bool { todo!() }

/// (src/scene/collision.ts:777 StaticCollision.solidBlocked)
pub fn solid_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:783 StaticCollision.shotBlocked)
pub fn shot_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:803 StaticCollision.raycast)
pub fn raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> { todo!() }

/// (src/scene/collision.ts:808 StaticCollision.raycastAsk)
pub fn raycast_ask(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::RaycastAsk> { todo!() }

/// (src/scene/collision.ts:826 StaticCollision.raycastHere) TWIN → sim::lines::ray(w, a0, a1, a2, r0, r1, r2, tmax, set, ask, out) -> i32 (ray_but leaves one leaf out; ray_rule asks the pane's rule)
pub fn raycast_here(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::RaycastHere> { todo!() }

/// (src/scene/collision.ts:871 SoundPaths.constructor)
pub fn sound_paths_new(g: &mut crate::game::Game) -> crate::scene_collision::SoundPaths { todo!() }

/// (src/scene/collision.ts:872 SoundPaths.tick)
pub fn sound_paths_tick(g: &mut crate::game::Game, time: f64) { todo!() }

/// (src/scene/collision.ts:874 SoundPaths.invalidate)
pub fn sound_paths_invalidate(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:876 SoundPaths.propagate)
pub fn sound_paths_propagate(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation { todo!() }

/// (src/scene/collision.ts:879 SoundPaths.propagateAsk)
pub fn sound_paths_propagate_ask(g: &mut crate::game::Game, src: V3, dst: V3, keep: bool) -> crate::scene_collision::Propagation { todo!() }

/// (src/scene/collision.ts:889 SoundPaths.propagateThere)
pub fn sound_paths_propagate_there(g: &mut crate::game::Game, src: V3, dst: V3, keep: bool, write: bool) -> Option<crate::scene_collision::Propagation> { todo!() }

/// (src/scene/collision.ts:914 SoundPaths.propagateHere)
pub fn sound_paths_propagate_here(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation { todo!() }

/// (src/scene/collision.ts:964 NavGrid.constructor)
pub fn nav_grid_new(g: &mut crate::game::Game) -> crate::scene_collision::NavGrid { todo!() }

/// (src/scene/collision.ts:981 NavGrid.rebake)
pub fn nav_grid_rebake(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64) -> Vec<f64> { todo!() }

/// (src/scene/collision.ts:993 NavGrid.idx)
pub fn nav_grid_idx(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:996 NavGrid.cellBlockers)
pub fn nav_grid_cell_blockers(g: &mut crate::game::Game, wx: f64, wz: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:997 NavGrid.isBlocked)
pub fn nav_grid_is_blocked(g: &mut crate::game::Game, wx: f64, wz: f64) -> bool { todo!() }

/// (src/scene/collision.ts:1001 NavGrid.findPathAside)
pub fn nav_grid_find_path_aside(g: &mut crate::game::Game, own: &mut crate::scene_collision::NavGridFindPathAsideOwn, a: V3, b: V3, max_iter: Option<f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1003 NavGrid.aside)
pub fn nav_grid_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_collision::NavGridAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/collision.ts:1014 NavGrid.findPath)
pub fn nav_grid_find_path(g: &mut crate::game::Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1018 NavGrid.findPathAsk)
pub fn nav_grid_find_path_ask(g: &mut crate::game::Game, a: V3, b: V3, max_iter: f64, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1045 NavGrid.there)
pub fn nav_grid_there(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/collision.ts:1052 NavGrid.findPathThere)
pub fn nav_grid_find_path_there(g: &mut crate::game::Game, a: V3, b: V3, max_iter: f64, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1072 NavGrid.priceThese)
pub fn nav_grid_price_these(g: &mut crate::game::Game, k: f64, penalty: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) { todo!() }

/// (src/scene/collision.ts:1078 NavGrid.findPathHere)
pub fn nav_grid_find_path_here(g: &mut crate::game::Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1133 NavGrid.dearAlong)
pub fn nav_grid_dear_along(g: &mut crate::game::Game, a: V3, b: V3, penalty: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) -> f64 { todo!() }

/// (src/scene/collision.ts:1150 NavGrid.nearestFree)
pub fn nav_grid_nearest_free(g: &mut crate::game::Game, gx: f64, gz: f64, wx: Option<f64>, wz: Option<f64>) -> Option<[f64; 2]> { todo!() }

/// (src/scene/collision.ts:1167 NavGrid.nearestFreePoint)
pub fn nav_grid_nearest_free_point(g: &mut crate::game::Game, wx: f64, wz: f64) -> Option<V3> { todo!() }

/// (src/scene/collision.ts:1174 NavGrid.walkable)
pub fn nav_grid_walkable(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/scene/collision.ts:1185 clampi)
pub fn clampi(x: f64, a: f64, b: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:1187 same3)
pub fn same3(a: V3, b: V3) -> bool { todo!() }

/// (src/scene/collision.ts:1189 navScratch)
pub fn nav_scratch(n: f64) -> crate::scene_collision::NavScratch { todo!() }

/// (src/scene/collision.ts:1191 countTrue)
pub fn count_true(flags: &[bool], i: f64, j: f64) -> f64 { todo!() }
