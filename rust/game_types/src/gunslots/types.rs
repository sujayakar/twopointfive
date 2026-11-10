//! The types of `gunslots`.
#![allow(unused_imports)]
use crate::{Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum GunSlot {
    #[default] Primary,
    Secondary,
}

#[derive(Clone, Debug, Default)]
pub struct GunPick {
    pub id: String,
    pub slot: crate::gunslots::GunSlot,
    pub can: Option<crate::weapons::WeaponId>,
    pub bare: Option<crate::weapons::WeaponId>,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GunslotsStats {
    pub out: i32,
    pub back: i32,
}

pub const SEEN_SECS: f64 = 0.5;

pub fn picks() -> Vec<crate::gunslots::GunPick> { alloc::vec![crate::gunslots::GunPick { id: String::from("mk18"), slot: crate::gunslots::GunSlot::Primary, can: Some(crate::weapons::WeaponId::Fn2000), bare: Some(crate::weapons::WeaponId::Mk18bare) }, crate::gunslots::GunPick { id: String::from("mk12"), slot: crate::gunslots::GunSlot::Primary, can: Some(crate::weapons::WeaponId::Mk12), bare: None }, crate::gunslots::GunPick { id: String::from("mp7"), slot: crate::gunslots::GunSlot::Primary, can: Some(crate::weapons::WeaponId::Mp7), bare: None }, crate::gunslots::GunPick { id: String::from("mk46"), slot: crate::gunslots::GunSlot::Primary, can: None, bare: Some(crate::weapons::WeaponId::Mk46) }, crate::gunslots::GunPick { id: String::from("m1014"), slot: crate::gunslots::GunSlot::Primary, can: None, bare: Some(crate::weapons::WeaponId::M1014) }, crate::gunslots::GunPick { id: String::from("g19"), slot: crate::gunslots::GunSlot::Secondary, can: Some(crate::weapons::WeaponId::Fiveseven), bare: Some(crate::weapons::WeaponId::G19bare) }, crate::gunslots::GunPick { id: String::from("hk45"), slot: crate::gunslots::GunSlot::Secondary, can: Some(crate::weapons::WeaponId::Hk45), bare: None }, crate::gunslots::GunPick { id: String::from("g18"), slot: crate::gunslots::GunSlot::Secondary, can: None, bare: Some(crate::weapons::WeaponId::G18) }] }

pub const NO_PRIMARY: &str = "none";
