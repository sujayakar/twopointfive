#![allow(unused_variables)]
//! Every function and method of `gunwall`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/gunwall.ts:57 st)
pub fn st<'g>(g: &'g mut crate::game::Game, c_2: CharId) -> &'g mut crate::gunwall::St { todo!() }

/// (src/game/gunwall.ts:73 fightOn)
pub fn fight_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunwall.ts:83 showing)
pub fn showing(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunwall.ts:86 frame)
pub fn frame(g: &mut crate::game::Game, c_2: CharId, yaw: f64) -> crate::gunwall::Frame { todo!() }

/// (src/game/gunwall.ts:91 at)
pub fn at(o: V3, f: crate::gunwall::AtF, fwd: f64, right: f64, y: f64) -> V3 { todo!() }

/// (src/game/gunwall.ts:94 track)
pub fn track(g: &mut crate::game::Game, c_2: CharId) { todo!() }

/// (src/game/gunwall.ts:102 closing)
pub fn closing(s: crate::gunwall::St, f: V3) -> f64 { todo!() }

/// (src/game/gunwall.ts:108 raisedLine)
pub fn raised_line(g: &mut crate::game::Game, c_2: CharId, up: crate::gunwall::GunUp, yaw: f64, o: Option<V3>) -> crate::gunwall::RaisedLine { todo!() }

/// (src/game/gunwall.ts:121 gunClearFrom)
pub fn gun_clear_from(g: &mut crate::game::Game, c_2: CharId, x: f64, z: f64, to: V3, ahead: f64) -> bool { todo!() }

/// (src/game/gunwall.ts:133 lowReach)
pub fn low_reach(g: &mut crate::game::Game, c_2: CharId) -> f64 { todo!() }

/// (src/game/gunwall.ts:138 panel)
pub fn panel(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/gunwall.ts:140 woodBetween)
pub fn wood_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/gunwall.ts:160 walledNow)
pub fn walled_now(g: &mut crate::game::Game, c_2: CharId, up: crate::gunwall::GunUp, to: Option<V3>, dt: f64, hot: bool, deep: bool) -> bool { todo!() }

/// (src/game/gunwall.ts:199 breachAim)
pub fn breach_aim(g: &mut crate::game::Game, c_2: CharId, b: crate::weapons::PelletSpecBreach) -> Option<V3> { todo!() }

/// (src/game/gunwall.ts:211 gunWalled)
pub fn gun_walled(g: &mut crate::game::Game, c_2: CharId, up: Option<crate::gunwall::GunUp>, to: Option<V3>, dt: f64, hot: Option<bool>, deep: Option<bool>) -> bool { todo!() }

/// (src/game/gunwall.ts:223 gunWallIs)
pub fn gun_wall_is(g: &mut crate::game::Game, c_2: CharId) -> bool { todo!() }

/// (src/game/gunwall.ts:230 gunWallCheck)
pub fn gun_wall_check(g: &mut crate::game::Game, gd: GuardId, up: Option<crate::gunwall::GunUp>, to: Option<V3>, dt: f64, hot: Option<bool>) -> bool { todo!() }

/// (src/game/gunwall.ts:243 gunWallPose)
pub fn gun_wall_pose(g: &mut crate::game::Game, c_2: CharId) { todo!() }

/// (src/game/gunwall.ts:269 gunWallStance)
pub fn gun_wall_stance(g: &mut crate::game::Game, c_2: CharId, set: bool) { todo!() }

/// (src/game/gunwall.ts:288 gunWallFresh)
pub fn gun_wall_fresh(g: &mut crate::game::Game) { todo!() }
