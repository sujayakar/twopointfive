//! The types of `kitanim`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BeatRow {
    pub dur: f64,
    pub out: f64,
    pub in_: f64,
    pub rack: Option<f64>,
    pub keep: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum ReloadKind {
    #[default] Dry,
    Empty,
    Tactical,
}
impl ReloadKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ReloadKind::Dry => "dry",
            ReloadKind::Empty => "empty",
            ReloadKind::Tactical => "tactical",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitanimModKITANIMLOGItem {
    pub t: f64,
    pub what: crate::kitanim::KitanimModKITANIMLOGItemWhat,
    pub from: Option<crate::character::KitItem>,
    pub to: Option<crate::character::KitItem>,
    pub kind: Option<crate::kitanim::ReloadKind>,
    pub secs: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum KitanimModKITANIMLOGItemWhat {
    #[default] Reload,
    Swap,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NoteRow {
    pub kind: Option<crate::kitanim::ReloadKind>,
    pub from: Option<crate::character::KitItem>,
    pub to: Option<crate::character::KitItem>,
    pub what: crate::kitanim::NoteRowWhat,
    pub secs: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum NoteRowWhat {
    #[default] Reload,
    Swap,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitShowV {
    pub pistol_in_hand: bool,
    pub rifle: Option<crate::kitanim::KitShowVRifle>,
    pub held: Option<crate::game::ThrowKind>,
    pub knife: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitShowVRifle {
    Hand,
    Slung,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitReload {
    pub kind: crate::kitanim::ReloadKind,
    pub row: crate::weapons::WeaponDefReload,
    pub beat: crate::character::ReloadBeat,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct KitanimMod {
    /// (src/game/kitanim.ts:86 KITANIM_LOG)
    pub kitanim_log: Vec<crate::kitanim::KitanimModKITANIMLOGItem>,
}
impl KitanimMod {
    pub fn blank() -> Self {
        KitanimMod {
            kitanim_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct KitanimStats {
    pub swaps: i32,
    pub draws: JsMap<crate::character::KitItem, i32>,
    pub stows: JsMap<crate::character::KitItem, i32>,
    pub reloads: JsMap<crate::kitanim::ReloadKind, i32>,
    pub close_frames: i32,
    pub nv: JsMap<bool, i32>,
    pub owed: i32,
}

