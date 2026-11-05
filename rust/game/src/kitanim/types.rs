//! The types of `kitanim`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: BeatRow, src/game/kitanim.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct BeatRow {
    pub dur: f64,
    pub out: f64,
    pub in_: f64,
    pub rack: Option<f64>,
    pub keep: Option<f64>,
}
impl BeatRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BeatRow {
            dur: 0.0,
            out: 0.0,
            in_: 0.0,
            rack: None,
            keep: None,
        }
    }
}
impl Default for BeatRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ReloadKind {
    Dry,
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

/// (TypeScript: inline at src/game/kitanim.ts:86 KITANIM_LOG, src/game/kitanim.ts:86)
#[derive(Clone, Copy, Debug)]
pub struct KitanimModKITANIMLOGItem {
    pub t: f64,
    pub what: crate::kitanim::KitanimModKITANIMLOGItemWhat,
    pub from: Option<crate::character::KitItem>,
    pub to: Option<crate::character::KitItem>,
    pub kind: Option<crate::kitanim::ReloadKind>,
    pub secs: f64,
}
impl KitanimModKITANIMLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitanimModKITANIMLOGItem {
            t: 0.0,
            what: crate::kitanim::KitanimModKITANIMLOGItemWhat::Reload,
            from: None,
            to: None,
            kind: None,
            secs: 0.0,
        }
    }
}
impl Default for KitanimModKITANIMLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitanimModKITANIMLOGItemWhat {
    Reload,
    Swap,
}
impl KitanimModKITANIMLOGItemWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitanimModKITANIMLOGItemWhat::Reload => "reload",
            KitanimModKITANIMLOGItemWhat::Swap => "swap",
        }
    }
}

/// (TypeScript: inline at src/game/kitanim.ts:87 note(row), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Copy, Debug)]
pub struct NoteRow {
    pub kind: Option<crate::kitanim::ReloadKind>,
    pub from: Option<crate::character::KitItem>,
    pub to: Option<crate::character::KitItem>,
    pub what: crate::kitanim::NoteRowWhat,
    pub secs: f64,
}
impl NoteRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NoteRow {
            kind: None,
            from: None,
            to: None,
            what: crate::kitanim::NoteRowWhat::Reload,
            secs: 0.0,
        }
    }
}
impl Default for NoteRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NoteRowWhat {
    Reload,
    Swap,
}
impl NoteRowWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            NoteRowWhat::Reload => "reload",
            NoteRowWhat::Swap => "swap",
        }
    }
}

/// (TypeScript: inline at src/game/kitanim.ts:225 kitLockMax(KIT), src/game/kitanim.ts:225)
#[derive(Clone, Debug)]
pub struct KitLockMaxKIT {
    pub stow: JsMap<crate::character::KitItem, f64>,
    pub reach: JsMap<crate::character::KitItem, f64>,
    pub draw: JsMap<crate::character::KitItem, f64>,
    pub far: f64,
    pub swap_k: f64,
    pub front: Option<crate::kitanim::KitLockMaxKITFront>,
}
impl KitLockMaxKIT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitLockMaxKIT {
            stow: JsMap::new(),
            reach: JsMap::new(),
            draw: JsMap::new(),
            far: 0.0,
            swap_k: 0.0,
            front: None,
        }
    }
}
impl Default for KitLockMaxKIT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitanim.ts:225 KitLockMaxKIT.front, src/game/kitanim.ts:225)
#[derive(Clone, Copy, Debug)]
pub struct KitLockMaxKITFront {
    pub stow: f64,
    pub reach: f64,
    pub draw: f64,
}
impl KitLockMaxKITFront {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitLockMaxKITFront {
            stow: 0.0,
            reach: 0.0,
            draw: 0.0,
        }
    }
}
impl Default for KitLockMaxKITFront { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/kitanim.ts:240 kitShow(v), src/game/kitanim.ts:240)
#[derive(Clone, Copy, Debug)]
pub struct KitShowV {
    pub pistol_in_hand: bool,
    pub rifle: Option<crate::kitanim::KitShowVRifle>,
    pub held: Option<crate::game::ThrowKind>,
    pub knife: bool,
}
impl KitShowV {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitShowV {
            pistol_in_hand: false,
            rifle: None,
            held: None,
            knife: false,
        }
    }
}
impl Default for KitShowV { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitShowVRifle {
    Hand,
    Slung,
}
impl KitShowVRifle {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitShowVRifle::Hand => "hand",
            KitShowVRifle::Slung => "slung",
        }
    }
}

/// (TypeScript: inline at src/game/kitanim.ts:268 kitReload ->, src/game/kitanim.ts:268)
#[derive(Clone, Copy, Debug)]
pub struct KitReload {
    pub kind: crate::kitanim::ReloadKind,
    pub row: crate::weapons::WeaponDefReload,
    pub beat: crate::character::ReloadBeat,
}
impl KitReload {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitReload {
            kind: crate::kitanim::ReloadKind::Dry,
            row: crate::weapons::WeaponDefReload::zeroed(),
            beat: crate::character::ReloadBeat::zeroed(),
        }
    }
}
impl Default for KitReload { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/game/kitanim.ts:83 KITANIM_DEFAULTS: a table nothing writes)
pub fn kitanim_defaults() -> crate::knobs::KitanimK { crate::knobs::page_kitanim() }
