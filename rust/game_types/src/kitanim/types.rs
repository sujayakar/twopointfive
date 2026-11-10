//! The types of `kitanim`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BeatRow {
    pub dur: f64,
    pub out: f64,
    pub in_: f64,
    pub rack: Option<f64>,
    pub keep: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum ReloadKind {
    #[default] Dry,
    Empty,
    Tactical,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitShowV {
    pub pistol_in_hand: bool,
    pub rifle: Option<crate::rig_props::RifleAt>,
    pub held: Option<crate::game::ThrowKind>,
    pub knife: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitReload {
    pub kind: crate::kitanim::ReloadKind,
    pub row: crate::weapons::WeaponDefReload,
    pub beat: crate::character::ReloadBeat,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct KitanimStats {
    pub draws: JsMap<crate::character::KitItem, i32>,
    pub stows: JsMap<crate::character::KitItem, i32>,
    pub reloads: JsMap<crate::kitanim::ReloadKind, i32>,
    pub nv: JsMap<bool, i32>,
}

