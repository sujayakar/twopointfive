#![allow(unused_variables)]
//! Every function and method of `devices`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/devices.ts:64 CameraRt.constructor)
pub fn camera_rt_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque, def: &crate::devices::CameraDef2, index: f64, target: FixtureId, cone: Option<LightId>, smoke_key: &str) -> CameraRtId { todo!() }

/// (src/game/devices.ts:68 LaserRt.constructor)
pub fn laser_rt_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque, def: &crate::devices::LaserDef2, index: f64, target: FixtureId, dir: V3, len: f64) -> LaserRtId { todo!() }

/// (src/game/devices.ts:83 Devices.note)
pub fn devices_note(g: &mut crate::game::Game, r: &crate::devices::DevicesNoteR) { todo!() }

/// (src/game/devices.ts:84 Devices.constructor)
pub fn devices_new(g: &mut crate::game::Game) -> crate::devices::Devices { todo!() }

/// (src/game/devices.ts:109 Devices.padOf)
pub fn devices_pad_of(g: &mut crate::game::Game, d: DoorId) -> Option<crate::devices::KeypadRt> { todo!() }

/// (src/game/devices.ts:111 Devices.overhearSaid)
pub fn devices_overhear_said(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool { todo!() }

/// (src/game/devices.ts:113 Devices.padNear)
pub fn devices_pad_near(g: &mut crate::game::Game, p: V3) -> Option<crate::devices::KeypadRt> { todo!() }

/// (src/game/devices.ts:117 Devices.snapshot)
pub fn devices_snapshot(g: &mut crate::game::Game) -> crate::devices::DevicesSnap { todo!() }

/// (src/game/devices.ts:121 Devices.restore)
pub fn devices_restore(g: &mut crate::game::Game, s: &crate::devices::DevicesSnap) { todo!() }

/// (src/game/devices.ts:129 Devices.reset)
pub fn devices_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:135 Devices.summary)
pub fn devices_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/devices.ts:142 alive)
pub fn alive(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/devices.ts:144 sensing)
pub fn sensing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/devices.ts:150 lensGain)
pub fn lens_gain(g: &mut crate::game::Game, dist: f64, range: f64, smoke_trans: f64) -> f64 { todo!() }

/// (src/game/devices.ts:155 lensDir)
pub fn lens_dir(yaw: f64, pitch: f64) -> V3 { todo!() }

/// (src/game/devices.ts:157 sweepYaw)
pub fn sweep_yaw(d: &crate::devices::CameraDef2, phase: f64) -> f64 { todo!() }

/// (src/game/devices.ts:162 updateDevices)
pub fn update_devices(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/devices.ts:170 updateCamera)
pub fn update_camera(g: &mut crate::game::Game, c_2: CameraRtId, dt: f64, live: bool) { todo!() }

/// (src/game/devices.ts:189 cameraSuspect)
pub fn camera_suspect(g: &mut crate::game::Game, c_2: CameraRtId) { todo!() }

/// (src/game/devices.ts:195 cameraAlarm)
pub fn camera_alarm(g: &mut crate::game::Game, c_2: CameraRtId) { todo!() }

/// (src/game/devices.ts:205 updateLaser)
pub fn update_laser(g: &mut crate::game::Game, l: LaserRtId, dt: f64, live: bool) { todo!() }

/// (src/game/devices.ts:236 bodyBreaks)
pub fn body_breaks(g: &mut crate::game::Game, c_2: CharId, a: V3, b: V3) -> Option<V3> { todo!() }

/// (src/game/devices.ts:242 laserTrip)
pub fn laser_trip(g: &mut crate::game::Game, l: LaserRtId, who: &str, at: V3) { todo!() }

/// (src/game/devices.ts:262 deviceDown)
pub fn device_down(g: &mut crate::game::Game, name: &str, spoken: &str, pos: V3) { todo!() }

/// (src/game/devices.ts:272 sendToLook)
pub fn send_to_look(g: &mut crate::game::Game, pos: V3, what: &str) -> Option<GuardId> { todo!() }

/// (src/game/devices.ts:284 control)
pub fn control(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/devices.ts:286 chronDevice)
pub fn chron_device(g: &mut crate::game::Game, kind: crate::devices::ChronDeviceKind, data: &JsMap<String, Option<crate::devices::ChronDeviceDataValue>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/devices.ts:293 learnCode)
pub fn learn_code(g: &mut crate::game::Game, lock_id: &str, how: crate::devices::LearnCodeHow, from: Option<GuardId>) -> bool { todo!() }

/// (src/game/devices.ts:305 overhear)
pub fn overhear(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool { todo!() }

/// (src/game/devices.ts:313 spell)
pub fn spell(code: &str) -> String { todo!() }

/// (src/game/devices.ts:315 padDoorName)
pub fn pad_door_name(g: &mut crate::game::Game, k: &crate::devices::KeypadRt) -> String { todo!() }

/// (src/game/devices.ts:318 padState)
pub fn pad_state(g: &mut crate::game::Game, d: DoorId, from: V3) -> Option<crate::devices::PadState> { todo!() }

/// (src/game/devices.ts:326 startCoding)
pub fn start_coding(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/devices.ts:334 denyPad)
pub fn deny_pad(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/devices.ts:341 stopCoding)
pub fn stop_coding(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:344 updateCoding)
pub fn update_coding(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/devices.ts:366 deviceBoxes)
pub fn device_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/devices.ts:371 deviceBoxesRaw)
pub fn device_boxes_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/devices.ts:407 loadDeviceTune)
pub fn load_device_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:414 saveDeviceTune)
pub fn save_device_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:415 resetDeviceTune)
pub fn reset_device_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:419 segSegClosest)
pub fn seg_seg_closest(p0: V3, p1: V3, b0: V3, b1: V3) -> crate::devices::SegSegClosest { todo!() }
