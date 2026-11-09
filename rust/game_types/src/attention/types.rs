//! The types of `attention`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DrawKind {
    #[default] Door,
    Light,
    Noise,
    Whistle,
    Lent,
}
impl DrawKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DrawKind::Door => "door",
            DrawKind::Light => "light",
            DrawKind::Noise => "noise",
            DrawKind::Whistle => "whistle",
            DrawKind::Lent => "lent",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum HoldMode {
    #[default] Go,
    Look,
    Lent,
}
impl HoldMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            HoldMode::Go => "go",
            HoldMode::Look => "look",
            HoldMode::Lent => "lent",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Hold {
    pub at: V3,
    pub kind: crate::attention::DrawKind,
    pub mode: crate::attention::HoldMode,
    pub t0: f64,
    pub secs: f64,
    pub until: f64,
    pub full: f64,
    pub glimpse: f64,
    pub told: bool,
    pub nth: i32,
    pub ears: f64,
    pub noises: f64,
    pub yaw: Option<f64>,
    /// INVENTED: dealt when a hold starts; a new hold of the same man counts as new
    pub serial: u32,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Mem {
    pub hold: Option<crate::attention::Hold>,
    pub cand: Option<crate::attention::MemCand>,
    pub lent: Option<crate::attention::MemLent>,
    pub takes: Vec<f64>,
    pub tick: f64,
    pub told_at: f64,
    pub line: i32,
    pub heard_t: f64,
    pub lent_off: f64,
    pub owed: f64,
    pub series: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MemCand {
    pub at: V3,
    pub kind: crate::attention::DrawKind,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MemLent {
    pub at: V3,
    pub t: f64,
    pub ears: f64,
    pub noises: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct AttentionModATTENTIONLOGItem {
    pub what: String,
    pub secs: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LogH {
    pub secs: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AttentionLendO {
    pub ears: Option<f64>,
    pub noises: Option<f64>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct AttentionMod {
    pub mem_of: JsMap<GuardId, crate::attention::Mem>,
    pub attention_log: Vec<crate::attention::AttentionModATTENTIONLOGItem>,
}
impl AttentionMod {
    pub fn blank() -> Self {
        AttentionMod {
            mem_of: JsMap::new(),
            attention_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct AttentionStats {
    pub taken: i32,
    pub looks: i32,
    pub lent: i32,
    pub released: i32,
    pub broken: i32,
    pub spent: i32,
    pub worn: i32,
}

pub const STALE: f64 = 0.25;

pub fn wary() -> Vec<String> { alloc::vec![String::from("again?"), String::from("…that's twice.")] }

pub fn wary_foe() -> Vec<String> { alloc::vec![String::from("again. stay sharp."), String::from("twice now.")] }

pub fn spent() -> Vec<String> { alloc::vec![String::from("no. somebody's doing that."), String::from("that's no accident. come out.")] }

pub fn spent_foe() -> Vec<String> { alloc::vec![String::from("that's bait. search the place."), String::from("someone's playing us. sweep it.")] }

pub fn broke() -> Vec<String> { alloc::vec![String::from("—wait."), String::from("what—")] }

pub fn done_foe() -> Vec<String> { alloc::vec![String::from("…nothing."), String::from("nothing there.")] }
