#![allow(unused_variables)]
//! Every function and method of `sim_lightshome`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/sim/lightshome.ts:27 columns)
pub fn columns(g: &mut crate::game::Game) { todo!() }

/// (src/sim/lightshome.ts:69 Fixture.constructor)
pub fn fixture_new(g: &mut crate::game::Game, i: f64, st: crate::Opaque, def: &crate::sim_lightshome::FixtureDef) -> FixtureId { todo!() }

/// (src/sim/lightshome.ts:76 Fixture.on)
pub fn fixture_on(g: &mut crate::game::Game, id: FixtureId) -> bool { todo!() }

/// (src/sim/lightshome.ts:77 Fixture.broken)
pub fn fixture_broken(g: &mut crate::game::Game, id: FixtureId) -> bool { todo!() }

/// (src/sim/lightshome.ts:79 Fixture.disabledUntil)
pub fn fixture_disabled_until(g: &mut crate::game::Game, id: FixtureId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:81 Fixture.factor)
pub fn fixture_factor(g: &mut crate::game::Game, id: FixtureId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:83 Fixture.handOff)
pub fn fixture_hand_off(g: &mut crate::game::Game, id: FixtureId) -> Option<f64> { todo!() }

/// (src/sim/lightshome.ts:85 Fixture.authored)
pub fn fixture_authored(g: &mut crate::game::Game, id: FixtureId) -> bool { todo!() }

/// (src/sim/lightshome.ts:86 Fixture.baseIntensity)
pub fn fixture_base_intensity(g: &mut crate::game::Game, id: FixtureId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:87 Fixture.stagger)
pub fn fixture_stagger(g: &mut crate::game::Game, id: FixtureId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:91 Mains.constructor)
pub fn mains_new(g: &mut crate::game::Game, st: crate::Opaque) -> crate::sim_lightshome::Mains { todo!() }

/// (src/sim/lightshome.ts:92 Mains.active)
pub fn mains_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/sim/lightshome.ts:93 Mains.permanent)
pub fn mains_permanent(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/sim/lightshome.ts:94 Mains.until)
pub fn mains_until(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/lightshome.ts:95 Mains.since)
pub fn mains_since(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/lightshome.ts:96 Mains.restoredAt)
pub fn mains_restored_at(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/lightshome.ts:98 Mains.fixT)
pub fn mains_fix_t(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/sim/lightshome.ts:104 CameraMachine.constructor)
pub fn camera_machine_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque) -> CameraRtId { todo!() }

/// (src/sim/lightshome.ts:106 CameraMachine.yaw)
pub fn camera_machine_yaw(g: &mut crate::game::Game, id: CameraRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:107 CameraMachine.phase)
pub fn camera_machine_phase(g: &mut crate::game::Game, id: CameraRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:109 CameraMachine.seen)
pub fn camera_machine_seen(g: &mut crate::game::Game, id: CameraRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:111 CameraMachine.state)
pub fn camera_machine_state(g: &mut crate::game::Game, id: CameraRtId) -> crate::sim_lightshome::CameraState { todo!() }

/// (src/sim/lightshome.ts:112 CameraMachine.reportedAt)
pub fn camera_machine_reported_at(g: &mut crate::game::Game, id: CameraRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:114 CameraMachine.lastSeen)
pub fn camera_machine_last_seen(g: &mut crate::game::Game, id: CameraRtId) -> Option<V3> { todo!() }

/// (src/sim/lightshome.ts:115 CameraMachine.sawThisFrame)
pub fn camera_machine_saw_this_frame(g: &mut crate::game::Game, id: CameraRtId) -> bool { todo!() }

/// (src/sim/lightshome.ts:117 CameraMachine.deadCalled)
pub fn camera_machine_dead_called(g: &mut crate::game::Game, id: CameraRtId) -> bool { todo!() }

/// (src/sim/lightshome.ts:119 CameraMachine.put)
pub fn camera_machine_put(g: &mut crate::game::Game, id: CameraRtId, yaw: f64, phase: f64, seen: f64, state: crate::sim_lightshome::CameraState, reported_at: f64, last_seen: Option<V3>, dead_called: bool) { todo!() }

/// (src/sim/lightshome.ts:128 LaserMachine.constructor)
pub fn laser_machine_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque) -> LaserRtId { todo!() }

/// (src/sim/lightshome.ts:129 LaserMachine.lastTripAt)
pub fn laser_machine_last_trip_at(g: &mut crate::game::Game, id: LaserRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:130 LaserMachine.trips)
pub fn laser_machine_trips(g: &mut crate::game::Game, id: LaserRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:132 LaserMachine.cut)
pub fn laser_machine_cut(g: &mut crate::game::Game, id: LaserRtId) -> f64 { todo!() }

/// (src/sim/lightshome.ts:135 LaserMachine.deadCalled)
pub fn laser_machine_dead_called(g: &mut crate::game::Game, id: LaserRtId) -> bool { todo!() }

/// (src/sim/lightshome.ts:137 LaserMachine.held)
pub fn laser_machine_held(g: &mut crate::game::Game, id: LaserRtId) -> bool { todo!() }

/// (src/sim/lightshome.ts:139 LaserMachine.put)
pub fn laser_machine_put(g: &mut crate::game::Game, id: LaserRtId, last_trip_at: f64, trips: f64, dead_called: bool, held: bool) { todo!() }

/// (src/sim/lightshome.ts:151 LightHome.constructor)
pub fn light_home_new(g: &mut crate::game::Game, st: crate::Opaque) -> crate::sim_lightshome::LightHome { todo!() }

/// (src/sim/lightshome.ts:155 LightHome.add)
pub fn light_home_add(g: &mut crate::game::Game, def: &crate::sim_lightshome::FixtureDef) -> FixtureId { todo!() }

/// (src/sim/lightshome.ts:164 LightHome.camera)
pub fn light_home_camera(g: &mut crate::game::Game, target: FixtureId, rest: f64, sweep: Option<[f64; 2]>, period: f64, pitch: f64, pos: V3) -> f64 { todo!() }

/// (src/sim/lightshome.ts:173 LightHome.laser)
pub fn light_home_laser(g: &mut crate::game::Game, target: FixtureId, len: f64) -> f64 { todo!() }

/// (src/sim/lightshome.ts:180 LightHome.stand)
pub fn light_home_stand(g: &mut crate::game::Game, list: &[LightId], targets: &[FixtureId], beacons: &[crate::sim_lightshome::BeaconFact], cones: &[crate::sim_lightshome::LightHomeStandConesItem]) { todo!() }

/// (src/sim/lightshome.ts:203 LightHome.feed)
pub fn light_home_feed(g: &mut crate::game::Game, l: LightId) { todo!() }

/// (src/sim/lightshome.ts:214 LightHome.feedBurning)
pub fn light_home_feed_burning(g: &mut crate::game::Game, l: LightId) { todo!() }

/// (src/sim/lightshome.ts:220 LightHome.in)
pub fn light_home_in(g: &mut crate::game::Game, kind: f64, t: Option<FixtureId>, a: Option<f64>, b: Option<f64>, c_2: Option<f64>, d: Option<f64>) -> f64 { todo!() }

/// (src/sim/lightshome.ts:230 LightHome.flip)
pub fn light_home_flip(g: &mut crate::game::Game, targets: &[FixtureId], on: bool, time: f64, strike: bool) { todo!() }

/// (src/sim/lightshome.ts:236 LightHome.step)
pub fn light_home_step(g: &mut crate::game::Game, time: f64, dt: f64, near: bool, fumble: f64, emissive_scale: f64) -> f64 { todo!() }

/// (src/sim/lightshome.ts:240 LightHome.paint)
pub fn light_home_paint(g: &mut crate::game::Game) { todo!() }

/// (src/sim/lightshome.ts:250 LightHome.cameraPan)
pub fn light_home_camera_pan(g: &mut crate::game::Game, slot: f64, cone: Option<LightId>, dt: f64, live: bool, cones: bool) -> f64 { todo!() }

/// (src/sim/lightshome.ts:256 LightHome.cameraSee)
pub fn light_home_camera_see(g: &mut crate::game::Game, slot: f64, time: f64, dt: f64, visible: bool, gain: f64, p: V3, suspicion: f64, alarm: f64) -> f64 { todo!() }

/// (src/sim/lightshome.ts:260 LightHome.laserOpen)
pub fn light_home_laser_open(g: &mut crate::game::Game, slot: f64) -> f64 { todo!() }

/// (src/sim/lightshome.ts:264 LightHome.laserClose)
pub fn light_home_laser_close(g: &mut crate::game::Game, slot: f64, time: f64, live: bool, cut: f64, sense: f64, held: bool) -> f64 { todo!() }
