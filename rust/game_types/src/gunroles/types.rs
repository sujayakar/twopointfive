//! The types of `gunroles`.
#![allow(unused_imports)]
use crate::{Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum GunRole {
    Pistol,
    Rifle,
    Room,
    Reach,
    Belt,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DealMan {
    pub gd: GuardId,
    pub watch: V3,
    pub hostage: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GunDealItem {
    pub gd: GuardId,
    pub gun: crate::weapons::WeaponId,
    pub line: f64,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GunrolesStats {
    pub dealt: i32,
    pub belt: i32,
    pub pistol: i32,
    pub akms: i32,
    pub first: i32,
    pub last: i32,
    pub posts: i32,
}

