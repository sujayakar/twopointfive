//! The types of `slotsetup`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SetupGun {
    Pistol,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SetupKind {
    Light,
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

/// (TypeScript: SetupRow, src/game/slotsetup.ts:74)
#[derive(Clone, Debug)]
pub struct SetupRow {
    pub key: f64,
    pub kind: crate::slotsetup::SetupKind,
    pub opts: Vec<crate::slotsetup::SetupRowOptsItem>,
}
impl SetupRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SetupRow {
            key: 0.0,
            kind: crate::slotsetup::SetupKind::Light,
            opts: Vec::new(),
        }
    }
}
impl Default for SetupRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/slotsetup.ts:74 SetupRow.opts, src/game/slotsetup.ts:74)
#[derive(Clone, Debug)]
pub struct SetupRowOptsItem {
    pub id: String,
    pub on: bool,
    pub left: Option<f64>,
}
impl SetupRowOptsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SetupRowOptsItem {
            id: String::new(),
            on: false,
            left: None,
        }
    }
}
impl Default for SetupRowOptsItem { fn default() -> Self { Self::zeroed() } }

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

/// (src/game/slotsetup.ts:85 BREAK)
pub const BREAK: bool = false;
