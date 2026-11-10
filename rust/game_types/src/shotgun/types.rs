//! The types of `shotgun`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a round in a man; a shell's are settled once the whole pattern has flown
#[derive(Clone, Copy, Debug)]
pub struct RoundIn {
    pub c: CharId,
    pub shooter: CharId,
    /// he was the man in a player's arm
    pub shield: bool,
    pub dir: V3,
    pub hit_p: V3,
}

#[derive(Clone, Debug, Default)]
pub struct Shell {
    pub def: crate::weapons::WeaponDef,
    pub i: f64,
    pub dir: Option<V3>,
    pub men: JsMap<CharId, crate::shotgun::ShellMenValue>,
    pub ends: Vec<crate::shotgun::ShellEndsItem>,
    pub glass: JsMap<crate::Num, Vec<f64>>,
    pub late: Vec<crate::shotgun::RoundIn>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShellMenValue {
    pub n: f64,
    pub near: f64,
    pub dir: V3,
    pub down: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShellEndsItem {
    pub index: f64,
    pub t: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreachSpot {
    pub pos: V3,
    pub yaw: f64,
    pub at: V3,
    pub beside: bool,
    pub own: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ShotgunStats {
    pub in_men: i32,
    pub dropped: i32,
    pub peppered: i32,
}

pub const LOCK_PROUD: f64 = 0.06;
