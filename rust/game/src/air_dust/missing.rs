//! Functions of `air_dust` that a caller names and nobody has written: a call says so by name.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/airDust.ts:144 AirDust.snapshot)
pub fn air_dust_snapshot(g: &mut crate::game::Game) -> Option<crate::air_dust::AirDustSnap> { crate::no_body("air_dust::air_dust_snapshot") }
/// (src/game/airDust.ts:154 AirDust.restore)
pub fn air_dust_restore(g: &mut crate::game::Game, sn: Option<&crate::air_dust::AirDustSnap>) { crate::no_body("air_dust::air_dust_restore") }
