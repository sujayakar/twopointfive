//! The types of `slotsetup`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum SetupGun {
    #[default] Pistol,
    Rifle,
}
impl SetupGun {
    pub const fn as_str(self) -> &'static str {
        match self {
            SetupGun::Pistol => "pistol",
            SetupGun::Rifle => "rifle",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SetupKind {
    #[default] Light,
    Load,
    Mode,
}
impl SetupKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            SetupKind::Light => "light",
            SetupKind::Load => "load",
            SetupKind::Mode => "mode",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SetupRow {
    pub key: f64,
    pub kind: crate::slotsetup::SetupKind,
    pub opts: Vec<crate::slotsetup::SetupRowOptsItem>,
}

#[derive(Clone, Debug, Default)]
pub struct SetupRowOptsItem {
    pub id: String,
    pub on: bool,
    pub left: Option<f64>,
}

/// (src/game/slotsetup.ts:76 ROW_KEY: a table nothing writes)
pub fn row_key() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("mode"), 1.0); t.set(String::from("load"), 2.0); t.set(String::from("light"), 3.0); t } }

/// (src/game/slotsetup.ts:77 SLOT_KEY: a table nothing writes)
pub fn slot_key() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("pistol"), 1.0); t.set(String::from("rifle"), 2.0); t } }

/// (src/game/slotsetup.ts:79 KEYED)
pub const KEYED: f64 = 0.1;

/// (src/game/slotsetup.ts:81 OWN_SLACK)
pub const OWN_SLACK: f64 = 0.05;

/// (src/game/slotsetup.ts:83 STALE)
pub const STALE: f64 = 0.2;

