//! The types of `attention`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DrawKind {
    Door,
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HoldMode {
    Go,
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

/// (TypeScript: Hold, src/game/attention.ts:73)
#[derive(Clone, Copy, Debug)]
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
    pub nth: f64,
    pub ears: f64,
    pub noises: f64,
    pub yaw: Option<f64>,
    /// INVENTED: dealt when a hold starts; a new hold of the same man counts as new
    pub serial: u32,
}
impl Hold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hold {
            at: [0.0; 3],
            kind: crate::attention::DrawKind::Door,
            mode: crate::attention::HoldMode::Go,
            t0: 0.0,
            secs: 0.0,
            until: 0.0,
            full: 0.0,
            glimpse: 0.0,
            told: false,
            nth: 0.0,
            ears: 0.0,
            noises: 0.0,
            yaw: None,
            serial: 0,
        }
    }
}
impl Default for Hold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Mem, src/game/attention.ts:86)
#[derive(Clone, Debug)]
pub struct Mem {
    pub hold: Option<crate::attention::Hold>,
    pub cand: Option<crate::attention::MemCand>,
    pub lent: Option<crate::attention::MemLent>,
    pub takes: Vec<f64>,
    pub tick: f64,
    pub told_at: f64,
    pub line: f64,
    pub heard_t: f64,
    pub lent_off: f64,
    pub owed: f64,
    pub series: bool,
}
impl Mem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mem {
            hold: None,
            cand: None,
            lent: None,
            takes: Vec::new(),
            tick: 0.0,
            told_at: 0.0,
            line: 0.0,
            heard_t: 0.0,
            lent_off: 0.0,
            owed: 0.0,
            series: false,
        }
    }
}
impl Default for Mem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:89 Mem.cand, src/game/attention.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct MemCand {
    pub at: V3,
    pub kind: crate::attention::DrawKind,
    pub t: f64,
}
impl MemCand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MemCand {
            at: [0.0; 3],
            kind: crate::attention::DrawKind::Door,
            t: 0.0,
        }
    }
}
impl Default for MemCand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:91 Mem.lent, src/game/attention.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct MemLent {
    pub at: V3,
    pub t: f64,
    pub ears: f64,
    pub noises: f64,
}
impl MemLent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MemLent {
            at: [0.0; 3],
            t: 0.0,
            ears: 0.0,
            noises: 0.0,
        }
    }
}
impl Default for MemLent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AttentionMark, src/game/attention.ts:288)
#[derive(Clone, Copy, Debug)]
pub struct AttentionMark {
    pub gd: GuardId,
    pub from: V3,
    pub at: V3,
    pub left: f64,
    pub mode: crate::attention::HoldMode,
    pub kind: crate::attention::DrawKind,
    pub ending: bool,
    pub worn: bool,
    pub walking: bool,
}
impl AttentionMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionMark {
            gd: GuardId(0),
            from: [0.0; 3],
            at: [0.0; 3],
            left: 0.0,
            mode: crate::attention::HoldMode::Go,
            kind: crate::attention::DrawKind::Door,
            ending: false,
            worn: false,
            walking: false,
        }
    }
}
impl Default for AttentionMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AttentionWayKept, src/game/attention.ts:307)
#[derive(Clone, Debug)]
pub struct AttentionWayKept {
    pub key: String,
    pub t: f64,
    pub from: V3,
    pub pts: Option<Vec<V3>>,
}
impl AttentionWayKept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionWayKept {
            key: String::new(),
            t: 0.0,
            from: [0.0; 3],
            pts: None,
        }
    }
}
impl Default for AttentionWayKept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AttentionWayStore, src/game/attention.ts:308)
#[derive(Clone, Copy, Debug)]
pub struct AttentionWayStore {
}
impl AttentionWayStore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionWayStore {
        }
    }
}
impl Default for AttentionWayStore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:331 AttentionSnap, src/game/attention.ts:331)
#[derive(Clone, Debug)]
pub struct AttentionSnapItem {
    pub hold: Option<crate::attention::AttentionSnapItemHold>,
    pub takes: Vec<f64>,
    pub told: f64,
    pub line: Option<f64>,
    pub heard: Option<f64>,
    pub lent_off: Option<f64>,
    pub series: Option<bool>,
    pub lent: Option<crate::attention::AttentionSnapItemLent>,
    pub tick: Option<f64>,
}
impl AttentionSnapItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionSnapItem {
            hold: None,
            takes: Vec::new(),
            told: 0.0,
            line: None,
            heard: None,
            lent_off: None,
            series: None,
            lent: None,
            tick: None,
        }
    }
}
impl Default for AttentionSnapItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:331 AttentionSnapItem.hold, src/game/attention.ts:331 AttentionSnapItem.hold)
#[derive(Clone, Copy, Debug)]
pub struct AttentionSnapItemHold {
    pub kind: crate::attention::DrawKind,
    pub at: V3,
    pub yaw: Option<f64>,
    pub ears: f64,
    pub told: bool,
    pub mode: crate::attention::HoldMode,
    pub secs: f64,
    pub full: f64,
    pub glimpse: f64,
    pub nth: f64,
    pub noises: f64,
    pub age: f64,
    pub left: f64,
}
impl AttentionSnapItemHold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionSnapItemHold {
            kind: crate::attention::DrawKind::Door,
            at: [0.0; 3],
            yaw: None,
            ears: 0.0,
            told: false,
            mode: crate::attention::HoldMode::Go,
            secs: 0.0,
            full: 0.0,
            glimpse: 0.0,
            nth: 0.0,
            noises: 0.0,
            age: 0.0,
            left: 0.0,
        }
    }
}
impl Default for AttentionSnapItemHold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:331 AttentionSnapItem.lent, src/game/attention.ts:331)
#[derive(Clone, Copy, Debug)]
pub struct AttentionSnapItemLent {
    pub at: V3,
    pub age: f64,
    pub ears: f64,
    pub noises: f64,
}
impl AttentionSnapItemLent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionSnapItemLent {
            at: [0.0; 3],
            age: 0.0,
            ears: 0.0,
            noises: 0.0,
        }
    }
}
impl Default for AttentionSnapItemLent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:105 ATTENTION_LOG, src/game/attention.ts:105)
#[derive(Clone, Debug)]
pub struct AttentionModATTENTIONLOGItem {
    pub t: f64,
    pub who: String,
    pub what: String,
    pub kind: crate::attention::DrawKind,
    pub secs: f64,
    pub nth: f64,
}
impl AttentionModATTENTIONLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionModATTENTIONLOGItem {
            t: 0.0,
            who: String::new(),
            what: String::new(),
            kind: crate::attention::DrawKind::Door,
            secs: 0.0,
            nth: 0.0,
        }
    }
}
impl Default for AttentionModATTENTIONLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:106 log(H), src/game/attention.ts:106)
#[derive(Clone, Copy, Debug)]
pub struct LogH {
    pub kind: crate::attention::DrawKind,
    pub secs: f64,
    pub nth: f64,
}
impl LogH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LogH {
            kind: crate::attention::DrawKind::Door,
            secs: 0.0,
            nth: 0.0,
        }
    }
}
impl Default for LogH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:120 attentionOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct AttentionOf {
    pub at: V3,
    pub kind: crate::attention::DrawKind,
    pub mode: crate::attention::HoldMode,
    pub t0: f64,
    pub secs: f64,
    pub until: f64,
    pub full: f64,
    pub glimpse: f64,
    pub told: bool,
    pub nth: f64,
    pub ears: f64,
    pub noises: f64,
    pub yaw: Option<f64>,
}
impl AttentionOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionOf {
            at: [0.0; 3],
            kind: crate::attention::DrawKind::Door,
            mode: crate::attention::HoldMode::Go,
            t0: 0.0,
            secs: 0.0,
            until: 0.0,
            full: 0.0,
            glimpse: 0.0,
            told: false,
            nth: 0.0,
            ears: 0.0,
            noises: 0.0,
            yaw: None,
        }
    }
}
impl Default for AttentionOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/attention.ts:186 attentionLend(o), src/game/attention.ts:186)
#[derive(Clone, Copy, Debug)]
pub struct AttentionLendO {
    pub ears: Option<f64>,
    pub noises: Option<f64>,
}
impl AttentionLendO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AttentionLendO {
            ears: None,
            noises: None,
        }
    }
}
impl Default for AttentionLendO { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct AttentionMod {
    /// (src/game/attention.ts:99 memOf)
    pub mem_of: JsMap<GuardId, crate::attention::Mem>,
    /// (src/game/attention.ts:105 ATTENTION_LOG)
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

/// (src/game/attention.ts:102 STALE)
pub const STALE: f64 = 0.25;

/// (src/game/attention.ts:111 WARY: a table nothing writes)
pub fn wary() -> Vec<String> { alloc::vec![String::from("again?"), String::from("…that's twice.")] }

/// (src/game/attention.ts:111 WARY_FOE: a table nothing writes)
pub fn wary_foe() -> Vec<String> { alloc::vec![String::from("again. stay sharp."), String::from("twice now.")] }

/// (src/game/attention.ts:112 SPENT: a table nothing writes)
pub fn spent() -> Vec<String> { alloc::vec![String::from("no. somebody's doing that."), String::from("that's no accident. come out.")] }

/// (src/game/attention.ts:112 SPENT_FOE: a table nothing writes)
pub fn spent_foe() -> Vec<String> { alloc::vec![String::from("that's bait. search the place."), String::from("someone's playing us. sweep it.")] }

/// (src/game/attention.ts:113 BROKE: a table nothing writes)
pub fn broke() -> Vec<String> { alloc::vec![String::from("—wait."), String::from("what—")] }

/// (src/game/attention.ts:114 DONE_FOE: a table nothing writes)
pub fn done_foe() -> Vec<String> { alloc::vec![String::from("…nothing."), String::from("nothing there.")] }
