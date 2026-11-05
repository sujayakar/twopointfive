#![allow(unused_variables)]
//! Every function and method of `props`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/props.ts:57 freshTrig)
pub fn fresh_trig(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:63 clampRow)
pub fn clamp_row(d: &mut Vec<f64>, vx: f64, vz: f64, ww: f64, r: f64) { todo!() }

/// (src/game/props.ts:68 turnRow)
pub fn turn_row(d: &mut Vec<f64>, yaw: f64, cs: f64, sn: f64, r: f64, y: f64) { todo!() }

/// (src/game/props.ts:78 Prop.f)
pub fn prop_f(g: &mut crate::game::Game, id: PropId, o: f64) -> f64 { todo!() }

/// (src/game/props.ts:79 Prop.pf)
pub fn prop_pf(g: &mut crate::game::Game, id: PropId, o: f64, v: f64) { todo!() }

/// (src/game/props.ts:80 Prop.bit)
pub fn prop_bit(g: &mut crate::game::Game, id: PropId, m: f64) -> bool { todo!() }

/// (src/game/props.ts:81 Prop.setBit)
pub fn prop_set_bit(g: &mut crate::game::Game, id: PropId, m: f64, v: bool) { todo!() }

/// (src/game/props.ts:84 Prop.x)
pub fn prop_x(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:84 Prop.x)
pub fn prop_set_x(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:85 Prop.z)
pub fn prop_z(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:85 Prop.z)
pub fn prop_set_z(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:87 Prop.yaw)
pub fn prop_yaw(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:88 Prop.yaw)
pub fn prop_set_yaw(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:89 Prop.vx)
pub fn prop_vx(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:89 Prop.vx)
pub fn prop_set_vx(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:90 Prop.vz)
pub fn prop_vz(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:90 Prop.vz)
pub fn prop_set_vz(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:91 Prop.w)
pub fn prop_w(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:91 Prop.w)
pub fn prop_set_w(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:95 Prop.asleep)
pub fn prop_asleep(g: &mut crate::game::Game, id: PropId) -> bool { todo!() }

/// (src/game/props.ts:95 Prop.asleep)
pub fn prop_set_asleep(g: &mut crate::game::Game, id: PropId, v: bool) { todo!() }

/// (src/game/props.ts:96 Prop.idle)
pub fn prop_idle(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:96 Prop.idle)
pub fn prop_set_idle(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:97 Prop.dirty)
pub fn prop_dirty(g: &mut crate::game::Game, id: PropId) -> bool { todo!() }

/// (src/game/props.ts:97 Prop.dirty)
pub fn prop_set_dirty(g: &mut crate::game::Game, id: PropId, v: bool) { todo!() }

/// (src/game/props.ts:99 Prop.rolling)
pub fn prop_rolling(g: &mut crate::game::Game, id: PropId) -> bool { todo!() }

/// (src/game/props.ts:99 Prop.rolling)
pub fn prop_set_rolling(g: &mut crate::game::Game, id: PropId, v: bool) { todo!() }

/// (src/game/props.ts:101 Prop.lastWho)
pub fn prop_last_who(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:101 Prop.lastWho)
pub fn prop_set_last_who(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:102 Prop.noiseT)
pub fn prop_noise_t(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:102 Prop.noiseT)
pub fn prop_set_noise_t(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:103 Prop.pushT)
pub fn prop_push_t(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:103 Prop.pushT)
pub fn prop_set_push_t(g: &mut crate::game::Game, id: PropId, v: f64) { todo!() }

/// (src/game/props.ts:106 Prop.constructor)
pub fn prop_new(g: &mut crate::game::Game, def: &crate::scene_level::PropDef, id_2: f64) -> PropId { todo!() }

/// (src/game/props.ts:117 Prop.cs)
pub fn prop_cs(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:117 Prop.sn)
pub fn prop_sn(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:119 Prop.place)
pub fn prop_place(g: &mut crate::game::Game, id: PropId) { todo!() }

/// (src/game/props.ts:128 Prop.wake)
pub fn prop_wake(g: &mut crate::game::Game, id: PropId) { todo!() }

/// (src/game/props.ts:129 Prop.speed)
pub fn prop_speed(g: &mut crate::game::Game, id: PropId) -> f64 { todo!() }

/// (src/game/props.ts:175 PropSystem.time)
pub fn prop_system_time(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/props.ts:175 PropSystem.time)
pub fn prop_system_set_time(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/props.ts:185 PropSystem.say)
pub fn prop_system_say(g: &mut crate::game::Game, e: crate::props::PropEvent) { todo!() }

/// (src/game/props.ts:194 PropSystem.constructor)
pub fn prop_system_new(g: &mut crate::game::Game, defs: &[crate::scene_level::PropDef], emit: &mut dyn FnMut(&mut crate::game::Game, crate::props::PropEvent), st: Option<crate::Opaque>) -> crate::props::PropSystem { todo!() }

/// (src/game/props.ts:202 PropSystem.propOf)
pub fn prop_system_prop_of(g: &mut crate::game::Game, b: BoxId) -> Option<PropId> { todo!() }

/// (src/game/props.ts:203 PropSystem.boxes)
pub fn prop_system_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/props.ts:206 PropSystem.remove)
pub fn prop_system_remove(g: &mut crate::game::Game, p: PropId) { todo!() }

/// (src/game/props.ts:208 PropSystem.reset)
pub fn prop_system_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:213 PropSystem.scatter)
pub fn prop_system_scatter(g: &mut crate::game::Game, strength: Option<f64>) { todo!() }

/// (src/game/props.ts:220 PropSystem.settle)
pub fn prop_system_settle(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:232 PropSystem.settleHere) TWIN → sim::props::settle(g, w) -> u32
pub fn prop_system_settle_here(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:235 PropSystem.settleWorld)
pub fn prop_system_settle_world(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:241 PropSystem.settleThere)
pub fn prop_system_settle_there(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:263 PropSystem.raycast)
pub fn prop_system_raycast(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::props::PropSystemRaycast> { todo!() }

/// (src/game/props.ts:271 PropSystem.raycastHere) TWIN → sim::lines::ray_things(w, a0, a1, a2, r0, r1, r2, tmax, out: &mut [f64]) -> i32
pub fn prop_system_raycast_here(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::props::PropSystemRaycastHere> { todo!() }

/// (src/game/props.ts:285 PropSystem.applyImpulse)
pub fn prop_system_apply_impulse(g: &mut crate::game::Game, p: PropId, point: V3, j: V3, who: f64, thud: Option<bool>) { todo!() }

/// (src/game/props.ts:293 PropSystem.clampVel)
pub fn prop_system_clamp_vel(g: &mut crate::game::Game, p: PropId) { todo!() }

/// (src/game/props.ts:300 PropSystem.noise)
pub fn prop_system_noise(g: &mut crate::game::Game, p: PropId, sound: crate::props::PropSystemNoiseSound, speed: f64, who: f64, level: f64) { todo!() }

/// (src/game/props.ts:309 PropSystem.update)
pub fn prop_system_update(g: &mut crate::game::Game, dt: f64, contacts: &mut Vec<crate::props::PropContact>, leaves: &[BoxId]) { todo!() }

/// (src/game/props.ts:324 PropSystem.updateHere) TWIN → sim::props::update(g, w) -> u32 (the events it listed)
pub fn prop_system_update_here(g: &mut crate::game::Game, dt: f64, contacts: &[crate::props::PropContact], leaves: &[BoxId]) { todo!() }

/// (src/game/props.ts:337 PropSystem.there)
pub fn prop_system_there(g: &mut crate::game::Game, contacts: &[crate::props::PropContact], leaves: &[BoxId]) -> Option<crate::props::PropSystemThere> { todo!() }

/// (src/game/props.ts:351 PropSystem.facts)
pub fn prop_system_facts(g: &mut crate::game::Game) { todo!() }

/// (src/game/props.ts:359 PropSystem.handOver)
pub fn prop_system_hand_over(g: &mut crate::game::Game, dt: f64, contacts: &[crate::props::PropContact], t: &crate::props::PropSystemHandOverT) -> f64 { todo!() }

/// (src/game/props.ts:377 PropSystem.takeBack)
pub fn prop_system_take_back(g: &mut crate::game::Game, contacts: &mut Vec<crate::props::PropContact>, ev: f64) { todo!() }

/// (src/game/props.ts:386 PropSystem.heard)
pub fn prop_system_heard(g: &mut crate::game::Game, e: &[f64], o: f64) -> crate::props::PropEvent { todo!() }
