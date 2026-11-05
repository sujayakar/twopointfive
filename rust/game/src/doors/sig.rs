#![allow(unused_variables)]
//! Every function and method of `doors`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/doors.ts:256 SlabRow.constructor)
pub fn slab_row_new(g: &mut crate::game::Game, door: f64) -> SlabRowId { todo!() }

/// (src/game/doors.ts:257 SlabRow.g)
pub fn slab_row_g(g: &mut crate::game::Game, id: SlabRowId, k: crate::doors::SlabRowGK) -> f64 { todo!() }

/// (src/game/doors.ts:258 SlabRow.f)
pub fn slab_row_f(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:259 SlabRow.p)
pub fn slab_row_p(g: &mut crate::game::Game, id: SlabRowId) -> [f64; 3] { todo!() }

/// (src/game/doors.ts:260 SlabRow.v)
pub fn slab_row_v(g: &mut crate::game::Game, id: SlabRowId) -> [f64; 3] { todo!() }

/// (src/game/doors.ts:261 SlabRow.a)
pub fn slab_row_a(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:262 SlabRow.spin)
pub fn slab_row_spin(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:263 SlabRow.tip)
pub fn slab_row_tip(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:264 SlabRow.tipV)
pub fn slab_row_tip_v(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:265 SlabRow.t)
pub fn slab_row_t(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:266 SlabRow.ang)
pub fn slab_row_ang(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:267 SlabRow.wedged)
pub fn slab_row_wedged(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:268 SlabRow.side)
pub fn slab_row_side(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:269 SlabRow.rest)
pub fn slab_row_rest(g: &mut crate::game::Game, id: SlabRowId) -> bool { todo!() }

/// (src/game/doors.ts:270 SlabRow.landed)
pub fn slab_row_landed(g: &mut crate::game::Game, id: SlabRowId) -> bool { todo!() }

/// (src/game/doors.ts:271 SlabRow.who)
pub fn slab_row_who(g: &mut crate::game::Game, id: SlabRowId) -> f64 { todo!() }

/// (src/game/doors.ts:272 SlabRow.bits)
pub fn slab_row_bits(g: &mut crate::game::Game, id: SlabRowId) -> Option<[f64; 4]> { todo!() }

/// (src/game/doors.ts:277 DriveCols.constructor)
pub fn drive_cols_new(g: &mut crate::game::Game, i: f64) -> DriveColsId { todo!() }

/// (src/game/doors.ts:278 DriveCols.g)
pub fn drive_cols_g(g: &mut crate::game::Game, id: DriveColsId, k: f64) -> f64 { todo!() }

/// (src/game/doors.ts:279 DriveCols.p)
pub fn drive_cols_p(g: &mut crate::game::Game, id: DriveColsId, k: f64, v: f64) { todo!() }

/// (src/game/doors.ts:280 DriveCols.who)
pub fn drive_cols_who(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:280 DriveCols.who)
pub fn drive_cols_set_who(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:281 DriveCols.dir)
pub fn drive_cols_dir(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:281 DriveCols.dir)
pub fn drive_cols_set_dir(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:282 DriveCols.kind)
pub fn drive_cols_kind(g: &mut crate::game::Game, id: DriveColsId) -> crate::doors::DriveColsKind { todo!() }

/// (src/game/doors.ts:282 DriveCols.kind)
pub fn drive_cols_set_kind(g: &mut crate::game::Game, id: DriveColsId, v: crate::doors::DriveColsSetKindV) { todo!() }

/// (src/game/doors.ts:283 DriveCols.t)
pub fn drive_cols_t(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:283 DriveCols.t)
pub fn drive_cols_set_t(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:284 DriveCols.from)
pub fn drive_cols_from(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:284 DriveCols.from)
pub fn drive_cols_set_from(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:285 DriveCols.quiet)
pub fn drive_cols_quiet(g: &mut crate::game::Game, id: DriveColsId) -> bool { todo!() }

/// (src/game/doors.ts:285 DriveCols.quiet)
pub fn drive_cols_set_quiet(g: &mut crate::game::Game, id: DriveColsId, v: bool) { todo!() }

/// (src/game/doors.ts:286 DriveCols.side)
pub fn drive_cols_side(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:286 DriveCols.side)
pub fn drive_cols_set_side(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:287 DriveCols.rate)
pub fn drive_cols_rate(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:287 DriveCols.rate)
pub fn drive_cols_set_rate(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:288 DriveCols.px)
pub fn drive_cols_px(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:288 DriveCols.px)
pub fn drive_cols_set_px(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:289 DriveCols.pz)
pub fn drive_cols_pz(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:289 DriveCols.pz)
pub fn drive_cols_set_pz(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:290 DriveCols.hit)
pub fn drive_cols_hit(g: &mut crate::game::Game, id: DriveColsId) -> f64 { todo!() }

/// (src/game/doors.ts:290 DriveCols.hit)
pub fn drive_cols_set_hit(g: &mut crate::game::Game, id: DriveColsId, v: f64) { todo!() }

/// (src/game/doors.ts:296 driveOf)
pub fn drive_of(g: &mut crate::game::Game, d: DoorId) -> Option<crate::doors::DoorDrive> { todo!() }

/// (src/game/doors.ts:303 Door.g)
pub fn door_g(g: &mut crate::game::Game, id: DoorId, k: f64) -> f64 { todo!() }

/// (src/game/doors.ts:304 Door.p)
pub fn door_p(g: &mut crate::game::Game, id: DoorId, k: f64, v: f64) { todo!() }

/// (src/game/doors.ts:305 Door.angle)
pub fn door_angle(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:305 Door.angle)
pub fn door_set_angle(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:306 Door.vel)
pub fn door_vel(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:306 Door.vel)
pub fn door_set_vel(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:307 Door.quietT)
pub fn door_quiet_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:307 Door.quietT)
pub fn door_set_quiet_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:308 Door.latched)
pub fn door_latched(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:308 Door.latched)
pub fn door_set_latched(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:309 Door.lastWho)
pub fn door_last_who(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:309 Door.lastWho)
pub fn door_set_last_who(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:310 Door.sinceBash)
pub fn door_since_bash(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:310 Door.sinceBash)
pub fn door_set_since_bash(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:311 Door.closing)
pub fn door_closing(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:311 Door.closing)
pub fn door_set_closing(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:315 Door.locked)
pub fn door_locked(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:315 Door.locked)
pub fn door_set_locked(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:316 Door.lockBroken)
pub fn door_lock_broken(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:316 Door.lockBroken)
pub fn door_set_lock_broken(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:317 Door.noticed)
pub fn door_noticed(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:317 Door.noticed)
pub fn door_set_noticed(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:318 Door.noticedT)
pub fn door_noticed_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:318 Door.noticedT)
pub fn door_set_noticed_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:319 Door.pick)
pub fn door_pick(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:319 Door.pick)
pub fn door_set_pick(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:320 Door.picking)
pub fn door_picking(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:320 Door.picking)
pub fn door_set_picking(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:321 Door.pickIdle)
pub fn door_pick_idle(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:321 Door.pickIdle)
pub fn door_set_pick_idle(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:322 Door.pickClock)
pub fn door_pick_clock(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:322 Door.pickClock)
pub fn door_set_pick_clock(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:323 Door.pickN)
pub fn door_pick_n(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:323 Door.pickN)
pub fn door_set_pick_n(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:324 Door.rattleT)
pub fn door_rattle_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:324 Door.rattleT)
pub fn door_set_rattle_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:326 Door.guardT)
pub fn door_guard_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:326 Door.guardT)
pub fn door_set_guard_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:329 Door.passers)
pub fn door_passers(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:329 Door.passers)
pub fn door_set_passers(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:339 Door.swing)
pub fn door_swing(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:340 Door.dealt)
pub fn door_dealt(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:340 Door.dealt)
pub fn door_set_dealt(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:341 Door.outward)
pub fn door_outward(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:341 Door.outward)
pub fn door_set_outward(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:343 Door.clearway)
pub fn door_clearway(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:343 Door.clearway)
pub fn door_set_clearway(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:345 Door.minA)
pub fn door_min_a(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:346 Door.maxA)
pub fn door_max_a(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:349 Door.pullT)
pub fn door_pull_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:349 Door.pullT)
pub fn door_set_pull_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:350 Door.seenVel)
pub fn door_seen_vel(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:357 Door.swept)
pub fn door_swept(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:357 Door.swept)
pub fn door_set_swept(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:360 Door.drive)
pub fn door_drive(g: &mut crate::game::Game, id: DoorId) -> Option<crate::doors::DoorDrive> { todo!() }

/// (src/game/doors.ts:361 Door.drive)
pub fn door_set_drive(g: &mut crate::game::Game, id: DoorId, v: Option<crate::doors::DoorDrive>) { todo!() }

/// (src/game/doors.ts:369 Door.restsTo)
pub fn door_rests_to(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:371 Door.solidT)
pub fn door_solid_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:371 Door.solidT)
pub fn door_set_solid_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:372 Door.pressT)
pub fn door_press_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:372 Door.pressT)
pub fn door_set_press_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:373 Door.openT)
pub fn door_open_t(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:373 Door.openT)
pub fn door_set_open_t(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:374 Door.mirrored)
pub fn door_mirrored(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:374 Door.mirrored)
pub fn door_set_mirrored(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:380 Door.off)
pub fn door_off(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:380 Door.off)
pub fn door_set_off(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:383 Door.slab)
pub fn door_slab(g: &mut crate::game::Game, id: DoorId) -> Option<crate::doors::SlabSeen> { todo!() }

/// (src/game/doors.ts:384 Door.slab)
pub fn door_set_slab(g: &mut crate::game::Game, id: DoorId, s: Option<crate::doors::SlabSeen>) { todo!() }

/// (src/game/doors.ts:391 Door.shot)
pub fn door_shot(g: &mut crate::game::Game, id: DoorId) -> f64 { todo!() }

/// (src/game/doors.ts:391 Door.shot)
pub fn door_set_shot(g: &mut crate::game::Game, id: DoorId, v: f64) { todo!() }

/// (src/game/doors.ts:394 Door.creakArmed)
pub fn door_creak_armed(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:394 Door.creakArmed)
pub fn door_set_creak_armed(g: &mut crate::game::Game, id: DoorId, v: bool) { todo!() }

/// (src/game/doors.ts:395 Door.constructor)
pub fn door_new(g: &mut crate::game::Game, def: &crate::doors::DoorDef, index: Option<f64>) -> DoorId { todo!() }

/// (src/game/doors.ts:418 Door.pos)
pub fn door_pos(g: &mut crate::game::Game, id: DoorId) -> V3 { todo!() }

/// (src/game/doors.ts:428 Door.keptPos)
pub fn door_kept_pos(g: &mut crate::game::Game, id: DoorId, p: V3) { todo!() }

/// (src/game/doors.ts:430 Door.keptAt)
pub fn door_kept_at(g: &mut crate::game::Game, id: DoorId) -> V3 { todo!() }

/// (src/game/doors.ts:433 Door.posAside)
pub fn door_pos_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, doors: &mut Vec<DoorId>, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/doors.ts:440 Door.isClosed)
pub fn door_is_closed(g: &mut crate::game::Game, id: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:441 Door.dir)
pub fn door_dir(g: &mut crate::game::Game, id: DoorId) -> [f64; 2] { todo!() }

/// (src/game/doors.ts:443 Door.reset)
pub fn door_reset(g: &mut crate::game::Game, id: DoorId) { todo!() }

/// (src/game/doors.ts:453 Door.setSwing)
pub fn door_set_swing(g: &mut crate::game::Game, id: DoorId, s: f64, outward: Option<f64>, clearway: Option<f64>) { todo!() }

/// (src/game/doors.ts:456 Door.keep)
pub fn door_keep(g: &mut crate::game::Game, id: DoorId) { todo!() }

/// (src/game/doors.ts:458 Door.sided)
pub fn door_sided(g: &mut crate::game::Game, id: DoorId, a: f64) -> f64 { todo!() }

/// (src/game/doors.ts:459 Door.place)
pub fn door_place(g: &mut crate::game::Game, id: DoorId) { todo!() }

/// (src/game/doors.ts:472 Door.poseAt)
pub fn door_pose_at(g: &mut crate::game::Game, id: DoorId, angle: f64) -> crate::doors::DoorPoseAt { todo!() }

/// (src/game/doors.ts:536 Doors.constructor)
pub fn doors_new(g: &mut crate::game::Game, defs: &[crate::doors::DoorDef], emit: &mut dyn FnMut(&mut crate::game::Game, crate::doors::DoorEvent), st: Option<crate::Opaque>) -> crate::doors::Doors { todo!() }

/// (src/game/doors.ts:538 Doors.byName)
pub fn doors_by_name(g: &mut crate::game::Game, n: &str) -> Option<DoorId> { todo!() }

/// (src/game/doors.ts:542 Doors.pullFace)
pub fn doors_pull_face(g: &mut crate::game::Game, d: DoorId, from: V3) -> bool { todo!() }

/// (src/game/doors.ts:544 Doors.frameRule)
pub fn doors_frame_rule(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doors.ts:547 Doors.bootHolds)
pub fn doors_boot_holds(g: &mut crate::game::Game, d: DoorId, from: V3) -> bool { todo!() }

/// (src/game/doors.ts:551 Doors.wedgeHolds)
pub fn doors_wedge_holds(g: &mut crate::game::Game, d: DoorId, from: V3) -> bool { todo!() }

/// (src/game/doors.ts:554 Doors.swingOf)
pub fn doors_swing_of(g: &mut crate::game::Game, d: DoorId) -> crate::doors::DoorSwing { todo!() }

/// (src/game/doors.ts:566 Doors.staged)
pub fn doors_staged(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doors.ts:568 Doors.stay)
pub fn doors_stay(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doors.ts:572 Doors.hasCloser)
pub fn doors_has_closer(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/doors.ts:575 Doors.side)
pub fn doors_side(g: &mut crate::game::Game, d: DoorId, from: V3) -> f64 { todo!() }

/// (src/game/doors.ts:578 Doors.lockedOut)
pub fn doors_locked_out(g: &mut crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool { todo!() }

/// (src/game/doors.ts:581 Doors.keyOpens)
pub fn doors_key_opens(g: &mut crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool { todo!() }

/// (src/game/doors.ts:592 Doors.throws)
pub fn doors_throws(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/doors.ts:594 Doors.opening)
pub fn doors_opening(g: &mut crate::game::Game, d: DoorId, dir: f64) -> bool { todo!() }

/// (src/game/doors.ts:596 Doors.driven)
pub fn doors_driven(g: &mut crate::game::Game, d: DoorId, who: f64, dir: f64, kind: crate::doors::DoorsDrivenKind, quiet: bool, rate: f64) { todo!() }

/// (src/game/doors.ts:636 Doors.update)
pub fn doors_update(g: &mut crate::game::Game, dt: f64, contacts: &mut Vec<crate::doors::DoorContact>) { todo!() }

/// (src/game/doors.ts:650 Doors.there)
pub fn doors_there(g: &mut crate::game::Game, contacts: &[crate::doors::DoorContact]) { todo!() }

/// (src/game/doors.ts:664 Doors.handOver)
pub fn doors_hand_over(g: &mut crate::game::Game, dt: f64, contacts: &[crate::doors::DoorContact], slot: f64) -> f64 { todo!() }

/// (src/game/doors.ts:693 Doors.takeBack)
pub fn doors_take_back(g: &mut crate::game::Game, contacts: &mut Vec<crate::doors::DoorContact>, ev: f64) { todo!() }

/// (src/game/doors.ts:721 Doors.follow)
pub fn doors_follow(g: &mut crate::game::Game, i: f64) { todo!() }

/// (src/game/doors.ts:725 Doors.handles)
pub fn doors_handles(g: &mut crate::game::Game, d: DoorId, yaw: f64, x1: f64, z1: f64, x2: f64, z2: f64) { todo!() }

/// (src/game/doors.ts:761 Doors.updateHere) TWIN → sim::doors::update(g, w) -> u32 (the noises it listed)
pub fn doors_update_here(g: &mut crate::game::Game, dt: f64, contacts: &[crate::doors::DoorContact]) { todo!() }

/// (src/game/doors.ts:1013 Doors.crack)
pub fn doors_crack(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, dt: f64, keys: Option<&JsSet<String>>, rate: Option<f64>, max_angle: Option<f64>) { todo!() }

/// (src/game/doors.ts:1025 Doors.use)
pub fn doors_use(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, keys: Option<&JsSet<String>>) { todo!() }

/// (src/game/doors.ts:1044 Doors.openFor)
pub fn doors_open_for(g: &mut crate::game::Game, d: DoorId, s: f64) -> f64 { todo!() }

/// (src/game/doors.ts:1045 Doors.pullTo)
pub fn doors_pull_to(g: &mut crate::game::Game, d: DoorId, who: f64) { todo!() }

/// (src/game/doors.ts:1048 Doors.pull)
pub fn doors_pull(g: &mut crate::game::Game, d: DoorId, who: f64, quiet: Option<bool>) { todo!() }

/// (src/game/doors.ts:1064 Doors.rattle)
pub fn doors_rattle(g: &mut crate::game::Game, d: DoorId, who: f64, level: Option<f64>) { todo!() }

/// (src/game/doors.ts:1071 Doors.workSpot)
pub fn doors_work_spot(g: &mut crate::game::Game, d: DoorId, from: V3, stand: Option<f64>, along: Option<f64>) -> crate::doors::DoorsWorkSpot { todo!() }

/// (src/game/doors.ts:1077 Doors.kickSpot)
pub fn doors_kick_spot(g: &mut crate::game::Game, d: DoorId, from: V3) -> crate::doors::DoorsKickSpot { todo!() }

/// (src/game/doors.ts:1084 Doors.kickSpotSide)
pub fn doors_kick_spot_side(g: &mut crate::game::Game, d: DoorId, from: V3) -> crate::doors::DoorsKickSpotSide { todo!() }

/// (src/game/doors.ts:1091 Doors.frameCoords)
pub fn doors_frame_coords(g: &mut crate::game::Game, d: DoorId, p: V3) -> crate::doors::DoorsFrameCoords { todo!() }

/// (src/game/doors.ts:1099 Doors.pick)
pub fn doors_pick(g: &mut crate::game::Game, d: DoorId, who: f64, dt: f64) -> bool { todo!() }

/// (src/game/doors.ts:1121 Doors.kickIn)
pub fn doors_kick_in(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, lock: Option<bool>) -> bool { todo!() }

/// (src/game/doors.ts:1140 Doors.blast)
pub fn doors_blast(g: &mut crate::game::Game, at: V3, tear_m: f64, who: f64) -> Vec<DoorId> { todo!() }

/// (src/game/doors.ts:1144 Doors.sound)
pub fn doors_sound(g: &mut crate::game::Game, d: DoorId, sound: crate::doors::DoorSound, pos: V3, who: f64, level: f64) { todo!() }

/// (src/game/doors.ts:1147 Doors.onTorn)
pub fn doors_on_torn(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, DoorId, f64, crate::doors::DoorsOnTornFnCause)) -> crate::FnField { todo!() }

/// (src/game/doors.ts:1148 Doors.torn)
pub fn doors_torn(g: &mut crate::game::Game, d: DoorId, who: f64, cause: Option<crate::doors::DoorsTornCause>) { todo!() }

/// (src/game/doors.ts:1153 Doors.threadPath)
pub fn doors_thread_path(g: &mut crate::game::Game, path: &[V3], start: V3, is_free: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool>) -> Vec<V3> { todo!() }

/// (src/game/doors.ts:1185 Doors.boxes)
pub fn doors_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/doors.ts:1188 Doors.leafBoxes)
pub fn doors_leaf_boxes(g: &mut crate::game::Game) -> Vec<BoxId> { todo!() }

/// (src/game/doors.ts:1190 Doors.posAside)
pub fn doors_pos_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }
