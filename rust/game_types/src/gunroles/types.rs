//! The types of `gunroles`.
use crate::{Words, ids::*, V3};

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

