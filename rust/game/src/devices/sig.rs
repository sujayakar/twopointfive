#![allow(unused_variables)]
//! Every function and method of `devices`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/devices.ts:65 CameraRt.constructor)
pub fn camera_rt_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque, def: &crate::devices::CameraDef2, index: f64, target: FixtureId, cone: LightId, smoke_key: &str) -> CameraRtId { todo!() }

/// (src/game/devices.ts:69 LaserRt.constructor)
pub fn laser_rt_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque, def: &crate::devices::LaserDef2, index: f64, target: FixtureId, dir: V3, len: f64) -> LaserRtId { todo!() }

/// (src/game/devices.ts:84 Devices.note)
pub fn devices_note(g: &mut crate::game::Game, r: &crate::devices::DevicesNoteR) { todo!() }

/// (src/game/devices.ts:85 Devices.constructor)
pub fn devices_new(g: &mut crate::game::Game) -> crate::devices::Devices { todo!() }

/// (src/game/devices.ts:110 Devices.padOf)
pub fn devices_pad_of(g: &mut crate::game::Game, d: DoorId) -> Option<crate::devices::KeypadRt> { todo!() }

/// (src/game/devices.ts:112 Devices.overhearSaid)
pub fn devices_overhear_said(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool { todo!() }

/// (src/game/devices.ts:114 Devices.padNear)
pub fn devices_pad_near(g: &mut crate::game::Game, p: V3) -> Option<crate::devices::KeypadRt> { todo!() }

/// (src/game/devices.ts:118 Devices.snapshot)
pub fn devices_snapshot(g: &mut crate::game::Game) -> crate::devices::DevicesSnap { todo!() }

/// (src/game/devices.ts:122 Devices.restore)
pub fn devices_restore(g: &mut crate::game::Game, s: &crate::devices::DevicesSnap) { todo!() }

/// (src/game/devices.ts:130 Devices.reset)
pub fn devices_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:136 Devices.summary)
pub fn devices_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/devices.ts:143 alive)
pub fn alive(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/devices.ts:145 sensing)
pub fn sensing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/devices.ts:151 lensGain)
pub fn lens_gain(g: &mut crate::game::Game, dist: f64, range: f64, smoke_trans: f64) -> f64 { todo!() }

/// (src/game/devices.ts:156 lensDir)
pub fn lens_dir(yaw: f64, pitch: f64) -> V3 { todo!() }

/// (src/game/devices.ts:158 sweepYaw)
pub fn sweep_yaw(d: &crate::devices::CameraDef2, phase: f64) -> f64 { todo!() }

/// (src/game/devices.ts:163 updateDevices)
pub fn update_devices(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/devices.ts:171 updateCamera)
pub fn update_camera(g: &mut crate::game::Game, c_2: CameraRtId, dt: f64, live: bool) { todo!() }

/// (src/game/devices.ts:190 cameraSuspect)
pub fn camera_suspect(g: &mut crate::game::Game, c_2: CameraRtId) { todo!() }

/// (src/game/devices.ts:196 cameraAlarm)
pub fn camera_alarm(g: &mut crate::game::Game, c_2: CameraRtId) { todo!() }

/// (src/game/devices.ts:206 updateLaser)
pub fn update_laser(g: &mut crate::game::Game, l: LaserRtId, dt: f64, live: bool) { todo!() }

/// (src/game/devices.ts:237 bodyBreaks)
pub fn body_breaks(g: &mut crate::game::Game, c_2: CharId, a: V3, b: V3) -> Option<V3> { todo!() }

/// (src/game/devices.ts:243 laserTrip)
pub fn laser_trip(g: &mut crate::game::Game, l: LaserRtId, who: &str, at: V3) { todo!() }

/// (src/game/devices.ts:263 deviceDown)
pub fn device_down(g: &mut crate::game::Game, name: &str, spoken: &str, pos: V3) { todo!() }

/// (src/game/devices.ts:273 sendToLook)
pub fn send_to_look(g: &mut crate::game::Game, pos: V3, what: &str) -> Option<GuardId> { todo!() }

/// (src/game/devices.ts:285 control)
pub fn control(g: &mut crate::game::Game, text: &str) { todo!() }

/// (src/game/devices.ts:287 chronDevice)
pub fn chron_device(g: &mut crate::game::Game, kind: crate::devices::ChronDeviceKind, data: &JsMap<String, Option<crate::devices::ChronDeviceDataValue>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> { todo!() }

/// (src/game/devices.ts:294 learnCode)
pub fn learn_code(g: &mut crate::game::Game, lock_id: &str, how: crate::devices::LearnCodeHow, from: Option<GuardId>) -> bool { todo!() }

/// (src/game/devices.ts:306 overhear)
pub fn overhear(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool { todo!() }

/// (src/game/devices.ts:314 spell)
pub fn spell(code: &str) -> String { todo!() }

/// (src/game/devices.ts:316 padDoorName)
pub fn pad_door_name(g: &mut crate::game::Game, k: &crate::devices::KeypadRt) -> String { todo!() }

/// (src/game/devices.ts:319 padState)
pub fn pad_state(g: &mut crate::game::Game, d: DoorId, from: V3) -> Option<crate::devices::PadState> { todo!() }

/// (src/game/devices.ts:327 startCoding)
pub fn start_coding(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/devices.ts:335 denyPad)
pub fn deny_pad(g: &mut crate::game::Game, d: DoorId) { todo!() }

/// (src/game/devices.ts:342 stopCoding)
pub fn stop_coding(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:345 updateCoding)
pub fn update_coding(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/devices.ts:367 deviceBoxes)
pub fn device_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/devices.ts:372 deviceBoxesRaw)
pub fn device_boxes_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/devices.ts:408 loadDeviceTune)
pub fn load_device_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:415 saveDeviceTune)
pub fn save_device_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:416 resetDeviceTune)
pub fn reset_device_tune(g: &mut crate::game::Game) { todo!() }

/// (src/game/devices.ts:420 segSegClosest)
pub fn seg_seg_closest(p0: V3, p1: V3, b0: V3, b1: V3) -> crate::devices::SegSegClosest { todo!() }
