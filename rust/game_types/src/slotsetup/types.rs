//! The types of `slotsetup`.
use crate::{Words, bytes::ToBytes, js::*};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum SetupGun {
    #[default] Pistol,
    Rifle,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Words)]
pub enum SetupKind {
    #[default] Light,
    Load,
    Mode,
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

pub fn row_key() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("mode"), 1.0); t.set(String::from("load"), 2.0); t.set(String::from("light"), 3.0); t } }

pub fn slot_key(gun: crate::slotsetup::SetupGun) -> f64 {
    match gun {
        crate::slotsetup::SetupGun::Pistol => 1.0,
        crate::slotsetup::SetupGun::Rifle => 2.0,
    }
}

pub const KEYED: f64 = 0.1;

pub const OWN_SLACK: f64 = 0.05;

pub const STALE: f64 = 0.2;

