//! The types of `shotgun`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a man this shell puts down, settled once the whole pattern has flown: combat.ts fireWeaponRaw's two blocks, run by fire_shell
#[derive(Clone, Debug)]
pub enum ShellLate {
    ManDown { hit_guard: GuardId, sg: Option<GuardId>, shooter: CharId, is_player: bool, ff_ours: bool, hostage_hit: bool, muzzle: V3, dir: V3, t_hit: f64, hit_p: V3 },
    SamHit { hit_view: PlayerId, sg: Option<GuardId>, dir: V3, hit_p: V3 },
}

#[derive(Clone, Debug, Default)]
pub struct Shell {
    pub def: crate::weapons::WeaponDef,
    pub i: f64,
    pub dir: Option<V3>,
    pub men: JsMap<CharId, crate::shotgun::ShellMenValue>,
    pub ends: Vec<crate::shotgun::ShellEndsItem>,
    pub glass: JsMap<crate::Num, Vec<f64>>,
    pub late: Vec<crate::shotgun::ShellLate>,
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShotgunModSHOTGUNLAST {
    pub ends: Vec<crate::shotgun::ShellEndsItem>,
    pub glass: JsMap<crate::Num, Vec<f64>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BreachSpotClose {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreachSpot {
    pub pos: V3,
    pub yaw: f64,
    pub at: V3,
    pub beside: bool,
    pub own: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ShotgunMod {
    #[is(crate::shotgun::ShotgunModSHOTGUNLAST { ends: Vec::new(), glass: JsMap::new() })] pub shotgun_last: crate::shotgun::ShotgunModSHOTGUNLAST,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ShotgunStats {
    pub shells: i32,
    pub pellets: i32,
    pub in_men: i32,
    pub dropped: i32,
    pub peppered: i32,
    pub locks: i32,
}

pub const LOCK_PROUD: f64 = 0.06;
