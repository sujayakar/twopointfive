//! The types of `gunslots`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GunSlot {
    Primary,
    Secondary,
}
impl GunSlot {
    pub const fn as_str(self) -> &'static str {
        match self {
            GunSlot::Primary => "primary",
            GunSlot::Secondary => "secondary",
        }
    }
}

/// (TypeScript: GunPick, src/game/gunslots.ts:44)
#[derive(Clone, Debug)]
pub struct GunPick {
    pub id: String,
    pub slot: crate::gunslots::GunSlot,
    pub can: Option<crate::weapons::WeaponId>,
    pub bare: Option<crate::weapons::WeaponId>,
    pub bare_was: Option<crate::weapons::WeaponId>,
}
impl GunPick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunPick {
            id: String::new(),
            slot: crate::gunslots::GunSlot::Primary,
            can: None,
            bare: None,
            bare_was: None,
        }
    }
}
impl Default for GunPick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SideHung {
    Carbine,
    Sc20k,
}
impl SideHung {
    pub const fn as_str(self) -> &'static str {
        match self {
            SideHung::Carbine => "carbine",
            SideHung::Sc20k => "sc20k",
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GunslotsStats {
    pub staged: i32,
    pub out: i32,
    pub back: i32,
}

/// (src/game/gunslots.ts:32 BREAK)
pub const BREAK: bool = false;

/// (src/game/gunslots.ts:39 SEEN_SECS)
pub const SEEN_SECS: f64 = 0.5;

/// (src/game/gunslots.ts:46 PICKS: a table nothing writes)
pub fn picks() -> Vec<crate::gunslots::GunPick> { alloc::vec![crate::gunslots::GunPick { id: String::from("mk18"), slot: crate::gunslots::GunSlot::Primary, can: Some(crate::weapons::WeaponId::Fn2000), bare: Some(crate::weapons::WeaponId::Mk18bare), bare_was: Some(crate::weapons::WeaponId::Aks74u) }, crate::gunslots::GunPick { id: String::from("mk12"), slot: crate::gunslots::GunSlot::Primary, can: Some(crate::weapons::WeaponId::Mk12), bare: None, bare_was: None }, crate::gunslots::GunPick { id: String::from("mp7"), slot: crate::gunslots::GunSlot::Primary, can: Some(crate::weapons::WeaponId::Mp7), bare: None, bare_was: None }, crate::gunslots::GunPick { id: String::from("mk46"), slot: crate::gunslots::GunSlot::Primary, can: None, bare: Some(crate::weapons::WeaponId::Mk46), bare_was: None }, crate::gunslots::GunPick { id: String::from("m1014"), slot: crate::gunslots::GunSlot::Primary, can: None, bare: Some(crate::weapons::WeaponId::M1014), bare_was: None }, crate::gunslots::GunPick { id: String::from("g19"), slot: crate::gunslots::GunSlot::Secondary, can: Some(crate::weapons::WeaponId::Fiveseven), bare: Some(crate::weapons::WeaponId::G19bare), bare_was: Some(crate::weapons::WeaponId::Guard9mm) }, crate::gunslots::GunPick { id: String::from("hk45"), slot: crate::gunslots::GunSlot::Secondary, can: Some(crate::weapons::WeaponId::Hk45), bare: None, bare_was: None }, crate::gunslots::GunPick { id: String::from("g18"), slot: crate::gunslots::GunSlot::Secondary, can: None, bare: Some(crate::weapons::WeaponId::G18), bare_was: None }] }

/// (src/game/gunslots.ts:57 NO_PRIMARY)
pub const NO_PRIMARY: &str = "none";
