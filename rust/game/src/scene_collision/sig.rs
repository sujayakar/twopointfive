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

/// (src/scene/collision.ts:98 noNumber)
pub fn no_number(ro: Option<V3>, rd: Option<V3>, tmax: Option<f64>) -> String { todo!() }

/// (src/scene/collision.ts:124 rowsHow)
pub fn rows_how(r: Option<crate::scene_collision::LampRows>) -> f64 { todo!() }

/// (src/scene/collision.ts:140 handedBack)
pub fn handed_back(row: &str, why: &str, a: &[f64], b: &[f64]) -> crate::Opaque { todo!() }

/// (src/scene/collision.ts:148 StaticCollision.constructor)
pub fn new(g: &mut crate::game::Game, boxes: &[BoxId], of: Option<&str>) -> crate::scene_collision::StaticCollision { todo!() }

/// (src/scene/collision.ts:169 StaticCollision.addStatic)
pub fn add_static(g: &mut crate::game::Game, b: BoxId, footprint: Option<BoxId>) -> f64 { todo!() }

/// (src/scene/collision.ts:177 StaticCollision.footCells)
pub fn foot_cells(g: &mut crate::game::Game, f: BoxId) -> [f64; 4] { todo!() }

/// (src/scene/collision.ts:186 StaticCollision.footEntries)
pub fn foot_entries(g: &mut crate::game::Game, f: BoxId) -> f64 { todo!() }

/// (src/scene/collision.ts:188 StaticCollision.truncateStatics)
pub fn truncate_statics(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/collision.ts:197 StaticCollision.tickOpen)
pub fn tick_open(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:200 StaticCollision.stand)
pub fn stand(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:204 StaticCollision.rulesWorld)
pub fn rules_world(g: &mut crate::game::Game, dyn_: &[BoxId]) { todo!() }

/// (src/scene/collision.ts:208 StaticCollision.rulesStatics)
pub fn rules_statics(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:212 StaticCollision.ruleOpen)
pub fn rule_open(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:218 StaticCollision.rule)
pub fn rule<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/collision.ts:224 StaticCollision.rulesPanes)
pub fn rules_panes(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/collision.ts:226 StaticCollision.fellWhy)
pub fn fell_why(g: &mut crate::game::Game) -> String { todo!() }

/// (src/scene/collision.ts:228 StaticCollision.footOf)
pub fn foot_of(g: &mut crate::game::Game, y0: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:230 StaticCollision.tickShut)
pub fn tick_shut(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:239 StaticCollision.window)
pub fn window<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/collision.ts:248 StaticCollision.ready)
pub fn ready(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:251 StaticCollision.lone)
pub fn lone(g: &mut crate::game::Game, must: Option<bool>) -> bool { todo!() }

/// (src/scene/collision.ts:257 StaticCollision.wrote)
pub fn wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:267 StaticCollision.aside)
pub fn aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::scene_collision::AsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/scene/collision.ts:287 StaticCollision.gather)
pub fn gather(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:291 StaticCollision.gatherAsk)
pub fn gather_ask(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:301 StaticCollision.gatherHere) TWIN → sim::collide::gather(w, x0, z0, x1, z1, y0, y1, cap) -> usize
pub fn gather_here(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:319 StaticCollision.circleTouches)
pub fn circle_touches(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:323 StaticCollision.circleTouchesAsk)
pub fn circle_touches_ask(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:333 StaticCollision.circleTouchesHere) TWIN → sim::collide::touches(w, x, z, r, y0, y1, cap) -> i32
pub fn circle_touches_here(g: &mut crate::game::Game, x: f64, z: f64, r: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:345 StaticCollision.collideCircle)
pub fn collide_circle(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:349 StaticCollision.collideCircleAsk)
pub fn collide_circle_ask(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:359 StaticCollision.collideCircleHere) TWIN → sim::collide::circle(w, px, pz, r, y0, y1, iterations) -> u32
pub fn collide_circle_here(g: &mut crate::game::Game, p: &mut V3, r: f64, y0: f64, y1: f64, iterations: f64) -> bool { todo!() }

/// (src/scene/collision.ts:401 StaticCollision.clampToWorld)
pub fn clamp_to_world(g: &mut crate::game::Game, p: &mut V3, r: f64) -> bool { todo!() }

/// (src/scene/collision.ts:409 StaticCollision.glassOpen)
pub fn glass_open(g: &mut crate::game::Game) -> Option<crate::scene_collision::PaneRule> { todo!() }

/// (src/scene/collision.ts:415 StaticCollision.glassRule)
pub fn glass_rule(g: &mut crate::game::Game, rule: Option<crate::scene_collision::PaneRule>, rows: Option<crate::scene_collision::PaneRows>) { todo!() }

/// (src/scene/collision.ts:416 StaticCollision.glassPanes)
pub fn glass_panes(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneSource> { todo!() }

/// (src/scene/collision.ts:424 StaticCollision.glassWrote)
pub fn glass_wrote(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:426 StaticCollision.glassViewed)
pub fn glass_viewed(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:433 StaticCollision.paneAsk)
pub fn pane_ask(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/scene/collision.ts:435 StaticCollision.paneFell)
pub fn pane_fell(g: &mut crate::game::Game, row: &str, ask: f64) { todo!() }

/// (src/scene/collision.ts:447 StaticCollision.segmentBlockedDynamic)
pub fn segment_blocked_dynamic(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:451 StaticCollision.segmentBlockedDynamicAsk)
pub fn segment_blocked_dynamic_ask(g: &mut crate::game::Game, a: V3, b: V3, need_flags: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:470 StaticCollision.thingsRay)
pub fn things_ray(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<Option<crate::scene_collision::ThingsRay>> { todo!() }

/// (src/scene/collision.ts:474 StaticCollision.thingsAsk)
pub fn things_ask(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<Option<crate::scene_collision::ThingsAsk>> { todo!() }

/// (src/scene/collision.ts:484 StaticCollision.thingsBoth)
pub fn things_both(g: &mut crate::game::Game, p: &crate::scene_box_mirror::Furniture, ro: V3, rd: V3, tmax: f64, t: Option<crate::scene_collision::ThingsBothT>) { todo!() }

/// (src/scene/collision.ts:491 StaticCollision.compared)
pub fn compared(g: &mut crate::game::Game, set: f64, ok: bool, what: &str, a: V3, b: V3, p: f64, q: f64) { todo!() }

/// (src/scene/collision.ts:498 StaticCollision.dynamicHit)
pub fn dynamic_hit(g: &mut crate::game::Game, a: V3, r0: f64, r1: f64, r2: f64, len: f64, need_flags: f64) -> bool { todo!() }

/// (src/scene/collision.ts:526 StaticCollision.segmentBlocked)
pub fn segment_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool { todo!() }

/// (src/scene/collision.ts:530 StaticCollision.segmentBlockedAsk)
pub fn segment_blocked_ask(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, through: Option<f64>, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool { todo!() }

/// (src/scene/collision.ts:557 StaticCollision.ownLines)
pub fn own_lines(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/scene/collision.ts:565 StaticCollision.sense)
pub fn sense(g: &mut crate::game::Game, row: &str) -> f64 { todo!() }

/// (src/scene/collision.ts:576 StaticCollision.sensePanes)
pub fn sense_panes(g: &mut crate::game::Game, row: &str) -> f64 { todo!() }

/// (src/scene/collision.ts:582 StaticCollision.eyeLine)
pub fn eye_line(g: &mut crate::game::Game, row: &str, through: f64) { todo!() }

/// (src/scene/collision.ts:587 StaticCollision.sightOpen)
pub fn sight_open(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/collision.ts:592 StaticCollision.sightEye)
pub fn sight_eye(g: &mut crate::game::Game, k: f64, pos: V3, look: V3, look_len: f64, cos_half: f64, near: f64, full: bool) { todo!() }

/// (src/scene/collision.ts:599 StaticCollision.eyesSee)
pub fn eyes_see(g: &mut crate::game::Game, order: f64, n: f64, want: f64, kind: f64, through: f64, a: V3, b: V3) -> f64 { todo!() }

/// (src/scene/collision.ts:610 StaticCollision.eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> crate::scene_collision::EyeContact { todo!() }

/// (src/scene/collision.ts:615 StaticCollision.eyeContactAsk)
pub fn eye_contact_ask(g: &mut crate::game::Game, e: crate::scene_collision::EyeContactAskE, chest: V3, head: V3, unseen: bool, smoked: bool, through: f64) -> crate::scene_collision::EyeContactAsk { todo!() }

/// (src/scene/collision.ts:634 StaticCollision.lampsBegin)
pub fn lamps_begin(g: &mut crate::game::Game, p: V3, shade: Option<&crate::scene_collision::LampsBeginShade>, k: Option<crate::scene_collision::SmokeNumbers>, rows: Option<crate::scene_collision::LampRows>) { todo!() }

/// (src/scene/collision.ts:641 StaticCollision.lampsOver)
pub fn lamps_over(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:643 StaticCollision.lampsPart)
pub fn lamps_part(g: &mut crate::game::Game, last: Option<bool>) { todo!() }

/// (src/scene/collision.ts:654 StaticCollision.lamp)
pub fn lamp(g: &mut crate::game::Game, pos: V3, range: f64, radius: f64, kind: f64, dir: V3, outer_deg: f64, inner_deg: f64, color: V3, intensity: f64, rows: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:664 StaticCollision.lampsOn)
pub fn lamps_on(g: &mut crate::game::Game, acc: &mut Vec<f64>) { todo!() }

/// (src/scene/collision.ts:672 StaticCollision.blobsBegin)
pub fn blobs_begin(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/collision.ts:674 StaticCollision.blobsFit)
pub fn blobs_fit(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/scene/collision.ts:675 StaticCollision.blob)
pub fn blob(g: &mut crate::game::Game, x: f64, y0: f64, z: f64, r2: f64, age: f64, mass: f64) { todo!() }

/// (src/scene/collision.ts:676 StaticCollision.blobsAlong)
pub fn blobs_along(g: &mut crate::game::Game, a: V3, b: V3, sigma: f64, spread: f64, widest: f64, rise: f64, rise_rate: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:682 StaticCollision.senseFor)
pub fn sense_for(g: &mut crate::game::Game, row: crate::scene_collision::SenseForRow) -> crate::scene_collision::SenseFor { todo!() }

/// (src/scene/collision.ts:694 StaticCollision.gridStep)
pub fn grid_step(g: &mut crate::game::Game, st: &mut crate::Opaque, pending: &[f64], n: f64, k: crate::scene_collision::GridStepK, glow: Option<crate::scene_collision::GridStepGlow>, wind: Option<&crate::scene_collision::GridWind>) -> bool { todo!() }

/// (src/scene/collision.ts:721 StaticCollision.segmentBlockedHere) TWIN → sim::lines::blocked(w, a0, a1, a2, b0, b1, b2, len, shrink, through, set, ask) -> i32; the named lines: sight_line, solid_line, leaf_line, man_hidden, leaves_blocked
pub fn segment_blocked_here(g: &mut crate::game::Game, a: V3, b: V3, shrink: f64, through: f64, panes: Option<&mut Vec<crate::scene_collision::SegmentBlockedPanesItem>>) -> bool { todo!() }

/// (src/scene/collision.ts:835 StaticCollision.solidBlocked)
pub fn solid_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:841 StaticCollision.shotBlocked)
pub fn shot_blocked(g: &mut crate::game::Game, a: V3, b: V3, shrink: Option<f64>, energy: Option<f64>) -> bool { todo!() }

/// (src/scene/collision.ts:861 StaticCollision.raycast)
pub fn raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> { todo!() }

/// (src/scene/collision.ts:866 StaticCollision.raycastAsk)
pub fn raycast_ask(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::RaycastAsk> { todo!() }

/// (src/scene/collision.ts:884 StaticCollision.raycastHere) TWIN → sim::lines::ray(w, a0, a1, a2, r0, r1, r2, tmax, set, ask, out) -> i32 (ray_but leaves one leaf out; ray_rule asks the pane's rule)
pub fn raycast_here(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::RaycastHere> { todo!() }

/// (src/scene/collision.ts:929 SoundPaths.constructor)
pub fn sound_paths_new(g: &mut crate::game::Game) -> crate::scene_collision::SoundPaths { todo!() }

/// (src/scene/collision.ts:930 SoundPaths.tick)
pub fn sound_paths_tick(g: &mut crate::game::Game, time: f64) { todo!() }

/// (src/scene/collision.ts:932 SoundPaths.invalidate)
pub fn sound_paths_invalidate(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:937 SoundPaths.propagate)
pub fn sound_paths_propagate(g: &mut crate::game::Game, src: V3, dst: V3, keep: Option<bool>) -> crate::scene_collision::Propagation { todo!() }

/// (src/scene/collision.ts:940 SoundPaths.propagateAsk)
pub fn sound_paths_propagate_ask(g: &mut crate::game::Game, src: V3, dst: V3, keep: bool) -> crate::scene_collision::Propagation { todo!() }

/// (src/scene/collision.ts:974 NavGrid.constructor)
pub fn nav_grid_new(g: &mut crate::game::Game) -> crate::scene_collision::NavGrid { todo!() }

/// (src/scene/collision.ts:991 NavGrid.rebake)
pub fn nav_grid_rebake(g: &mut crate::game::Game, x0: f64, z0: f64, x1: f64, z1: f64) -> Vec<f64> { todo!() }

/// (src/scene/collision.ts:1003 NavGrid.idx)
pub fn nav_grid_idx(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/scene/collision.ts:1006 NavGrid.cellBlockers)
pub fn nav_grid_cell_blockers(g: &mut crate::game::Game, wx: f64, wz: f64, out: &mut Vec<i32>) -> f64 { todo!() }

/// (src/scene/collision.ts:1007 NavGrid.isBlocked)
pub fn nav_grid_is_blocked(g: &mut crate::game::Game, wx: f64, wz: f64) -> bool { todo!() }

/// (src/scene/collision.ts:1021 NavGrid.findPath)
pub fn nav_grid_find_path(g: &mut crate::game::Game, a: V3, b: V3, max_iter: Option<f64>, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1028 NavGrid.there)
pub fn nav_grid_there(g: &mut crate::game::Game) { todo!() }

/// (src/scene/collision.ts:1032 NavGrid.findPathAsk)
pub fn nav_grid_find_path_ask(g: &mut crate::game::Game, a: V3, b: V3, max_iter: f64, penalty: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<Vec<V3>> { todo!() }

/// (src/scene/collision.ts:1052 NavGrid.priceThese)
pub fn nav_grid_price_these(g: &mut crate::game::Game, k: f64, penalty: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) { todo!() }

/// (src/scene/collision.ts:1065 NavGrid.nearestFree)
pub fn nav_grid_nearest_free(g: &mut crate::game::Game, gx: f64, gz: f64, wx: Option<f64>, wz: Option<f64>) -> Option<[f64; 2]> { todo!() }

/// (src/scene/collision.ts:1082 NavGrid.nearestFreePoint)
pub fn nav_grid_nearest_free_point(g: &mut crate::game::Game, wx: f64, wz: f64) -> Option<V3> { todo!() }

/// (src/scene/collision.ts:1089 NavGrid.walkable)
pub fn nav_grid_walkable(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/scene/collision.ts:1100 clampi)
pub fn clampi(x: f64, a: f64, b: f64) -> f64 { todo!() }


/// INVENTED: `g.col.glassOpen && g.col.glassOpen(bi, p)`, the rule glassbreak.ts:213 hangs. False with no rule hung; else glass_break_open_at(g, bi, p), and the burst's view `also` if GlassBreak keeps that field. NOT boxes.ts:1047's rule, which is glassBreakLive(g) && openAt (glassbreak.ts:214).
pub fn pane_open(g: &mut crate::game::Game, bi: f64, p: V3) -> bool { todo!() }
