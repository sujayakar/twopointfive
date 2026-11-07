//! The types of `gunroles`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GunRole {
    Pistol,
    Rifle,
    Room,
    Reach,
    Belt,
}
impl GunRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            GunRole::Pistol => "pistol",
            GunRole::Rifle => "rifle",
            GunRole::Room => "room",
            GunRole::Reach => "reach",
            GunRole::Belt => "belt",
        }
    }
}

/// (TypeScript: DealMan, src/game/gunroles.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct DealMan {
    pub gd: GuardId,
    pub watch: V3,
    pub hostage: Option<bool>,
}
impl DealMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DealMan {
            gd: GuardId(0),
            watch: [0.0; 3],
            hostage: None,
        }
    }
}
impl Default for DealMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gunroles.ts:69 ROLE, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
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
impl ROLE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ROLE {
            fiveseven: None,
            guard9mm: None,
            aks74u: None,
            ak74m: None,
            akms: None,
            fn2000: None,
            m870: None,
            mk12: None,
            mp7: None,
            mk46: None,
            m1014: None,
            hk45: None,
            g18: None,
            pkm: None,
            mk18bare: None,
            g19bare: None,
        }
    }
}
impl Default for ROLE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UseOnKey {
    Post,
    Entry,
    Lanes,
    Strings,
}
impl UseOnKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            UseOnKey::Post => "post",
            UseOnKey::Entry => "entry",
            UseOnKey::Lanes => "lanes",
            UseOnKey::Strings => "strings",
        }
    }
}

/// (TypeScript: inline at src/game/gunroles.ts:100 gunDeal ->, src/game/gunroles.ts:100)
#[derive(Clone, Copy, Debug)]
pub struct GunDealItem {
    pub gd: GuardId,
    pub gun: crate::weapons::WeaponId,
    pub line: f64,
}
impl GunDealItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GunDealItem {
            gd: GuardId(0),
            gun: crate::weapons::WeaponId::Fiveseven,
            line: 0.0,
        }
    }
}
impl Default for GunDealItem { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct GunrolesStats {
    pub dealt: i32,
    pub belt: i32,
    pub pistol: i32,
    pub akms: i32,
    pub first: i32,
    pub last: i32,
    pub filed: i32,
    pub posts: i32,
}

/// (src/game/gunroles.ts:63 BREAK)
pub const BREAK: bool = false;

/// (src/game/gunroles.ts:69 ROLE: a table nothing writes)
pub fn role() -> crate::gunroles::ROLE { crate::gunroles::ROLE { fiveseven: None, guard9mm: None, aks74u: None, ak74m: None, akms: None, fn2000: None, m870: None, mk12: Some(String::from("reach")), mp7: Some(String::from("room")), mk46: Some(String::from("belt")), m1014: Some(String::from("room")), hk45: None, g18: None, pkm: Some(String::from("belt")), mk18bare: None, g19bare: None } }

/// (src/game/gunroles.ts:71 RANK: a table nothing writes)
pub fn rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("room"), 0.0); t.set(String::from("rifle"), 1.0); t.set(String::from("pistol"), 1.0); t.set(String::from("reach"), 2.0); t.set(String::from("belt"), 2.0); t } }
