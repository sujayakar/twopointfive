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

#[derive(Clone, Debug, Default)]
pub struct ROLE {
    pub fiveseven: Option<String>,
    pub guard9mm: Option<String>,
    pub aks74u: Option<String>,
    pub ak74m: Option<String>,
    pub akms: Option<String>,
    pub fn2000: Option<String>,
    pub m870: Option<String>,
    pub mk12: Option<String>,
    pub mp7: Option<String>,
    pub mk46: Option<String>,
    pub m1014: Option<String>,
    pub hk45: Option<String>,
    pub g18: Option<String>,
    pub pkm: Option<String>,
    pub mk18bare: Option<String>,
    pub g19bare: Option<String>,
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

/// (src/game/gunroles.ts:69 ROLE: a table nothing writes)
pub fn role() -> crate::gunroles::ROLE { crate::gunroles::ROLE { fiveseven: None, guard9mm: None, aks74u: None, ak74m: None, akms: None, fn2000: None, m870: None, mk12: Some(String::from("reach")), mp7: Some(String::from("room")), mk46: Some(String::from("belt")), m1014: Some(String::from("room")), hk45: None, g18: None, pkm: Some(String::from("belt")), mk18bare: None, g19bare: None } }

/// (src/game/gunroles.ts:71 RANK: a table nothing writes)
pub fn rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("room"), 0.0); t.set(String::from("rifle"), 1.0); t.set(String::from("pistol"), 1.0); t.set(String::from("reach"), 2.0); t.set(String::from("belt"), 2.0); t } }
