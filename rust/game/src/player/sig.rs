#![allow(unused_variables)]
//! Every function and method of `player`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/player.ts:108 samBeat)
pub fn sam_beat(chars: f64) -> f64 { todo!() }

/// (src/game/player.ts:152 playerBodyR)
pub fn player_body_r(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/player.ts:154 settleSpot)
pub fn settle_spot(g: &mut crate::game::Game, p: &mut V3) { todo!() }

/// (src/game/player.ts:155 bodyR)
pub fn body_r(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/player.ts:174 collidePlayer)
pub fn collide_player(g: &mut crate::game::Game, y1: f64) { todo!() }

/// (src/game/player.ts:177 collideMan)
pub fn collide_man(g: &mut crate::game::Game, pl: &mut crate::player::CollideManPl, y1: f64) { todo!() }

/// (src/game/player.ts:189 canStand)
pub fn can_stand(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:191 canStandAt)
pub fn can_stand_at(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/player.ts:198 settlePlayer)
pub fn settle_player(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:217 crouchPace)
pub fn crouch_pace(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/player.ts:222 wheelPace)
pub fn wheel_pace(g: &mut crate::game::Game, wheel: f64) { todo!() }

/// (src/game/player.ts:248 footfallDue)
pub fn footfall_due(g: &mut crate::game::Game, dt: f64, phase: f64, speed: f64) -> f64 { todo!() }

/// (src/game/player.ts:263 itemName)
pub fn item_name(g: &mut crate::game::Game, pl: PlayerId) -> String { todo!() }

/// (src/game/player.ts:266 nextLoad)
pub fn next_load(g: &mut crate::game::Game, rf: FirearmId) -> String { todo!() }

/// (src/game/player.ts:267 rifleName)
pub fn rifle_name(g: &mut crate::game::Game, pl: PlayerId) -> String { todo!() }

/// (src/game/player.ts:271 setHolstered)
pub fn set_holstered(g: &mut crate::game::Game, on: bool, quiet: Option<bool>) { todo!() }

/// (src/game/player.ts:280 stowHolster)
pub fn stow_holster(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/player.ts:282 cancelReloads)
pub fn cancel_reloads(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/player.ts:292 manAim)
pub fn man_aim(g: &mut crate::game::Game, gd: GuardId, h: f64) -> V3 { todo!() }

/// (src/game/player.ts:305 levelOn)
pub fn level_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:311 freeAim)
pub fn free_aim(g: &mut crate::game::Game, keep: Option<f64>) -> V3 { todo!() }

/// (src/game/player.ts:320 poseGunLight)
pub fn pose_gun_light(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:330 armRifle)
pub fn arm_rifle(g: &mut crate::game::Game, in_hands: bool, carriage: Option<crate::character::LongStance>) { todo!() }

/// (src/game/player.ts:342 shellBreak)
pub fn shell_break(g: &mut crate::game::Game) -> Option<crate::player::ShellBreak> { todo!() }

/// (src/game/player.ts:356 reloadInHand)
pub fn reload_in_hand(g: &mut crate::game::Game, sprinting: Option<bool>, bolt_only: Option<bool>) -> bool { todo!() }

/// (src/game/player.ts:399 legsOwn)
pub fn legs_own(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:406 freePace)
pub fn free_pace(g: &mut crate::game::Game, m: crate::player::Legs, ml: f64) -> f64 { todo!() }

/// (src/game/player.ts:419 keysWay)
pub fn keys_way(fwd: V3, right: V3, w: bool, s: bool, d: bool, a: bool) -> [f64; 2] { todo!() }

/// (src/game/player.ts:428 crouchKey)
pub fn crouch_key(g: &mut crate::game::Game, m: &mut crate::player::Legs) -> bool { todo!() }

/// (src/game/player.ts:430 sprintStance)
pub fn sprint_stance(g: &mut crate::game::Game, m: &mut crate::player::Legs, want_sprint: bool, stalk: bool) { todo!() }

/// (src/game/player.ts:439 strideStep)
pub fn stride_step(g: &mut crate::game::Game, m: &mut crate::player::Legs, mx: f64, mz: f64, ml: f64, tap: Option<crate::player::StrideStepTap>, stalk: Option<crate::player::StrideStepStalk>, dt: f64, now: Option<f64>) { todo!() }

/// (src/game/player.ts:454 gunRaised)
pub fn gun_raised(g: &mut crate::game::Game, pl: PlayerId, now: Option<f64>, m: Option<crate::player::GunRaisedM>) -> bool { todo!() }

/// (src/game/player.ts:456 leafFacing)
pub fn leaf_facing(dp: Option<&crate::player::LeafFacingDp>, aiming: bool) -> Option<f64> { todo!() }

/// (src/game/player.ts:458 sprintFacing)
pub fn sprint_facing(g: &mut crate::game::Game, m: &mut crate::player::Legs, dt: f64, now: Option<f64>) -> Option<f64> { todo!() }

/// (src/game/player.ts:465 hipsTurn)
pub fn hips_turn(m: &mut crate::player::Legs, speed: f64, aiming: bool, orbit: bool, into_door: Option<f64>, by_hand: bool, dt: f64) { todo!() }

/// (src/game/player.ts:484 updatePlayer)
pub fn update_player(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId) { todo!() }

/// (src/game/player.ts:839 rifleSpread)
pub fn rifle_spread(g: &mut crate::game::Game, speed: f64) -> f64 { todo!() }

/// (src/game/player.ts:857 keepOffGuards)
pub fn keep_off_guards(g: &mut crate::game::Game, dt: f64, y1: f64, m: Option<&mut crate::player::Legs>) { todo!() }

/// (src/game/player.ts:959 faceAt)
pub fn face_at(g: &mut crate::game::Game, n: V3, d: f64, t: V3, u: f64) -> Option<[f64; 2]> { todo!() }

/// (src/game/player.ts:973 traceFace)
pub fn trace_face(g: &mut crate::game::Game, n: V3, d: f64, t: V3, u0: f64, look: Option<f64>) -> Option<[f64; 2]> { todo!() }

/// (src/game/player.ts:985 endOpen)
pub fn end_open(g: &mut crate::game::Game, n: V3, d: f64, t: V3, u_end: f64, s: f64) -> bool { todo!() }

/// (src/game/player.ts:997 clearOfProps)
pub fn clear_of_props(g: &mut crate::game::Game, q: &mut V3, r: f64) -> bool { todo!() }

/// (src/game/player.ts:1021 findWall)
pub fn find_wall(g: &mut crate::game::Game, quick: Option<bool>, why: Option<&mut crate::player::FindWallWhy>, only: Option<V3>) -> Option<crate::player::FindWall> { todo!() }

/// (src/game/player.ts:1064 canWallSnap)
pub fn can_wall_snap(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:1074 wallSnap)
pub fn wall_snap(g: &mut crate::game::Game, cam: CameraId, why: Option<&mut crate::player::WallSnapWhy>, only: Option<V3>) -> bool { todo!() }

/// (src/game/player.ts:1103 releaseWall)
pub fn release_wall(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1116 wallStepOff)
pub fn wall_step_off(g: &mut crate::game::Game, mx: f64, mz: f64, ml: f64, how: crate::player::WallStepOffHow) { todo!() }

/// (src/game/player.ts:1127 wallVoice)
pub fn wall_voice(g: &mut crate::game::Game, w: crate::player::WallHold, on: bool) { todo!() }

/// (src/game/player.ts:1136 wallSnapLater)
pub fn wall_snap_later(g: &mut crate::game::Game, ml: f64, why: crate::player::WallSnapLaterWhy) -> bool { todo!() }

/// (src/game/player.ts:1140 wallSnapPending)
pub fn wall_snap_pending(g: &mut crate::game::Game, dt: f64, cam: CameraId, want_sprint: bool) { todo!() }

/// (src/game/player.ts:1168 pressFace)
pub fn press_face(g: &mut crate::game::Game, mx: f64, mz: f64) -> Option<crate::player::PressFace> { todo!() }

/// (src/game/player.ts:1178 moveKeysDown)
pub fn move_keys_down(g: &mut crate::game::Game, input: InputId) -> f64 { todo!() }

/// (src/game/player.ts:1182 wallRanOn)
pub fn wall_ran_on(g: &mut crate::game::Game, pl: PlayerId, input: InputId) -> bool { todo!() }

/// (src/game/player.ts:1187 wallPress)
pub fn wall_press(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId, mx: f64, mz: f64, ml: f64, want_sprint: bool) { todo!() }

/// (src/game/player.ts:1264 wallStacks)
pub fn wall_stacks(g: &mut crate::game::Game, w: crate::player::WallHold) -> bool { todo!() }

/// (src/game/player.ts:1268 wallTakeYaw)
pub fn wall_take_yaw(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/player.ts:1278 throwRest)
pub fn throw_rest(g: &mut crate::game::Game) -> Option<crate::player::ThrowRest> { todo!() }

/// (src/game/player.ts:1282 launchCancel)
pub fn launch_cancel(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:1288 stackInWay)
pub fn stack_in_way(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:1296 gunShut)
pub fn gun_shut(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/player.ts:1303 stackUp)
pub fn stack_up(g: &mut crate::game::Game, dt: f64, input: InputId) -> bool { todo!() }

/// (src/game/player.ts:1327 stackLean)
pub fn stack_lean(g: &mut crate::game::Game, dt: f64, cam: CameraId, pl: PlayerId, dir: f64) -> f64 { todo!() }

/// (src/game/player.ts:1368 wallSnapPose)
pub fn wall_snap_pose(g: &mut crate::game::Game, dt: f64, aiming: bool) { todo!() }

/// (src/game/player.ts:1391 wallSnapCam)
pub fn wall_snap_cam(g: &mut crate::game::Game, cam: CameraId, pl: PlayerId) { todo!() }

/// (src/game/player.ts:1413 moveOnWall)
pub fn move_on_wall(g: &mut crate::game::Game, dt: f64, cam: CameraId, mx: f64, mz: f64, ml: f64) -> bool { todo!() }

/// (src/game/player.ts:1504 startTakedown)
pub fn start_takedown(g: &mut crate::game::Game, gd: GuardId, way: Option<crate::close::CloseKind>, lethal: Option<bool>, loud: Option<bool>) { todo!() }

/// (src/game/player.ts:1522 updateTakedown)
pub fn update_takedown(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/player.ts:1555 grabVariant)
pub fn grab_variant(g: &mut crate::game::Game) -> crate::character::HoldVariant { todo!() }

/// (src/game/player.ts:1562 startGrab)
pub fn start_grab(g: &mut crate::game::Game, gd: GuardId, way: Option<crate::close::CloseKind>) { todo!() }

/// (src/game/player.ts:1605 nvKey)
pub fn nv_key(g: &mut crate::game::Game, input: Option<InputId>, dt: f64) { todo!() }

/// (src/game/player.ts:1627 updateHold)
pub fn update_hold(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId) { todo!() }

/// (src/game/player.ts:1758 holdTail)
pub fn hold_tail(g: &mut crate::game::Game, dt: f64, speed: f64, own: Option<f64>) { todo!() }

/// (src/game/player.ts:1788 setPhase)
pub fn set_phase(g: &mut crate::game::Game, phase: crate::character::HoldPhase) { todo!() }

/// (src/game/player.ts:1796 easeOffHold)
pub fn ease_off_hold(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1805 cancelAsk)
pub fn cancel_ask(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1813 sayAfter)
pub fn say_after(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1826 swapHold)
pub fn swap_hold(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1847 startChoke)
pub fn start_choke(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1855 startWhip)
pub fn start_whip(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1861 killSecs)
pub fn kill_secs(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/player.ts:1863 killCut)
pub fn kill_cut(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/player.ts:1865 holdKilling)
pub fn hold_killing(h: &crate::player::Hold) -> bool { todo!() }

/// (src/game/player.ts:1868 startKill)
pub fn start_kill(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1877 placeHeld)
pub fn place_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1890 keepHeldOffGuards)
pub fn keep_held_off_guards(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/player.ts:1909 heldTalk)
pub fn held_talk(g: &mut crate::game::Game, h: &crate::player::Hold) -> crate::player::HeldTalk { todo!() }

/// (src/game/player.ts:1922 askHeld)
pub fn ask_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1937 answerHeld)
pub fn answer_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1948 takeKey)
pub fn take_key(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) { todo!() }

/// (src/game/player.ts:1962 settleHeld)
pub fn settle_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1972 chokeOut)
pub fn choke_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1985 killOut)
pub fn kill_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:1999 whipOut)
pub fn whip_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:2014 handBack)
pub fn hand_back(g: &mut crate::game::Game, gd: GuardId, stagger: f64) { todo!() }

/// (src/game/player.ts:2031 freeHeld)
pub fn free_held(g: &mut crate::game::Game, why: crate::player::FreeHeldWhy) { todo!() }

/// (src/game/player.ts:2039 dropHold)
pub fn drop_hold(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:2048 startPicking)
pub fn start_picking(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/player.ts:2058 stopPicking)
pub fn stop_picking(g: &mut crate::game::Game, text: Option<&str>) { todo!() }

/// (src/game/player.ts:2067 updatePicking)
pub fn update_picking(g: &mut crate::game::Game, dt: f64, input: InputId) { todo!() }

/// (src/game/player.ts:2089 startWand)
pub fn start_wand(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/player.ts:2099 stopWand)
pub fn stop_wand(g: &mut crate::game::Game, text: Option<&str>) { todo!() }

/// (src/game/player.ts:2105 updateWanding)
pub fn update_wanding(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/player.ts:2130 updateCodingPlant)
pub fn update_coding_plant(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/player.ts:2142 updateWorkPlant)
pub fn update_work_plant(g: &mut crate::game::Game, dt: f64, w: &mut crate::handwork::SamWork) { todo!() }

/// (src/game/player.ts:2155 startKick)
pub fn start_kick(g: &mut crate::game::Game, d: DoorId, run: Option<bool>, tool: Option<crate::player::StartKickTool>) { todo!() }

/// (src/game/player.ts:2173 sideKickSpot)
pub fn side_kick_spot(g: &mut crate::game::Game, d: DoorId, from: V3) -> Option<crate::player::SideKickSpot> { todo!() }

/// (src/game/player.ts:2184 updateKick)
pub fn update_kick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/player.ts:2211 bodyDist)
pub fn body_dist(g: &mut crate::game::Game, gd: GuardId, within: Option<f64>) -> f64 { todo!() }

/// (src/game/player.ts:2219 throwOrigin)
pub fn throw_origin(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/player.ts:2221 doThrow)
pub fn do_throw(g: &mut crate::game::Game, sol: &crate::throwables::ThrowSolution, kind: crate::game::ThrowKind) { todo!() }

/// (src/game/player.ts:2230 aimEase)
pub fn aim_ease(yaw: f64, to_aim: V3, dt: f64) -> f64 { todo!() }

/// (src/game/player.ts:2232 aimLevel)
pub fn aim_level(c_2: crate::player::AimLevelC) -> V3 { todo!() }

/// (src/game/player.ts:2235 cursorHit)
pub fn cursor_hit(g: &mut crate::game::Game, ray: crate::player::CursorHitRay, own: f64) -> Option<crate::scene_boxes::BoxWorldRaycast> { todo!() }

/// (src/game/player.ts:2236 cursorPoint)
pub fn cursor_point(g: &mut crate::game::Game, cam: CameraId, u: f64, v: f64, ray: crate::player::CursorPointRay, hit: Option<crate::player::CursorPointHit>) -> Option<V3> { todo!() }

/// (src/game/player.ts:2239 updateAim)
pub fn update_aim(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) { todo!() }

/// (src/game/player.ts:2308 buildInteractables)
pub fn build_interactables(g: &mut crate::game::Game, cam: CameraId, mx: f64, my: f64, w: f64, h: f64) { todo!() }

/// (src/game/player.ts:2540 rowKey)
pub fn row_key(g: &mut crate::game::Game, r: &crate::game::Interactable) -> String { todo!() }

/// (src/game/player.ts:2542 pickHover)
pub fn pick_hover(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:2549 cancelRow)
pub fn cancel_row(g: &mut crate::game::Game) -> crate::game::Interactable { todo!() }

/// (src/game/player.ts:2556 thingDist)
pub fn thing_dist(g: &mut crate::game::Game, it: &crate::game::Interactable, p: V3) -> f64 { todo!() }

/// (src/game/player.ts:2570 throwCount)
pub fn throw_count(g: &mut crate::game::Game, kind: crate::game::ThrowKind) -> f64 { todo!() }

/// (src/game/player.ts:2572 canBack)
pub fn can_back(g: &mut crate::game::Game, pl: PlayerId, kind: crate::game::ThrowKind) { todo!() }

/// (src/game/player.ts:2575 throwOff)
pub fn throw_off(g: &mut crate::game::Game, pl: PlayerId) { todo!() }

/// (src/game/player.ts:2578 toggleDrag)
pub fn toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) { todo!() }

/// (src/game/player.ts:2590 releaseBody)
pub fn release_body(g: &mut crate::game::Game, laid: Option<bool>) { todo!() }

/// (src/game/player.ts:2601 cancelActions)
pub fn cancel_actions(g: &mut crate::game::Game) { todo!() }

/// (src/game/player.ts:2617 teleportPlayer)
pub fn teleport_player(g: &mut crate::game::Game, p: V3) { todo!() }
