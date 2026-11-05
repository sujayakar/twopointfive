//! The types of `pairorders`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Job, src/game/pairorders.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub foe: GuardId,
    pub mode: crate::pairorders::JobMode,
    pub go: bool,
    pub set: bool,
    pub set_said: bool,
    pub t0: f64,
    pub born: f64,
    pub check_t: f64,
    pub wait_t: f64,
    pub heel_t: f64,
    pub going: bool,
    pub scuffled: bool,
    pub was_following: bool,
    pub handed: Option<bool>,
    pub watcher: Option<GuardId>,
    pub watch_t: f64,
    pub seen_t: f64,
    pub force: bool,
    pub cue_t: f64,
    pub cue_by: Option<GuardId>,
    pub cued: bool,
    pub back_said: Option<bool>,
    pub back_t: Option<f64>,
}
impl Job {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Job {
            foe: GuardId(0),
            mode: crate::pairorders::JobMode::Mark,
            go: false,
            set: false,
            set_said: false,
            t0: 0.0,
            born: 0.0,
            check_t: 0.0,
            wait_t: 0.0,
            heel_t: 0.0,
            going: false,
            scuffled: false,
            was_following: false,
            handed: None,
            watcher: None,
            watch_t: 0.0,
            seen_t: 0.0,
            force: false,
            cue_t: 0.0,
            cue_by: None,
            cued: false,
            back_said: None,
            back_t: None,
        }
    }
}
impl Default for Job { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JobMode {
    Mark,
    Now,
}
impl JobMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            JobMode::Mark => "mark",
            JobMode::Now => "now",
        }
    }
}

/// (TypeScript: PairOrders, src/game/pairorders.ts:143)
#[derive(Clone, Debug)]
pub struct PairOrders {
    pub jobs: JsMap<GuardId, crate::pairorders::Job>,
    pub habit: JsMap<GuardId, [f64; 2]>,
    pub glance: JsMap<GuardId, crate::pairorders::PairOrdersGlanceValue>,
    pub stood: JsMap<GuardId, f64>,
    pub swept: JsMap<GuardId, f64>,
    pub dark: JsMap<GuardId, crate::pairorders::PairOrdersDarkValue>,
    pub dark_memo: JsMap<String, crate::pairorders::PairOrdersDarkMemoValue>,
    pub ways: JsMap<GuardId, crate::pairorders::PairOrdersWaysValue>,
    pub seated: bool,
    pub go_seated: bool,
    pub hands_was: bool,
    pub lit_cells: JsMap<crate::Num, [f64; 2]>,
}
impl PairOrders {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        PairOrders {
            jobs: JsMap::new(),
            habit: JsMap::new(),
            glance: JsMap::new(),
            stood: JsMap::new(),
            swept: JsMap::new(),
            dark: JsMap::new(),
            dark_memo: JsMap::new(),
            ways: JsMap::new(),
            seated: false,
            go_seated: false,
            hands_was: false,
            lit_cells: JsMap::new(),
        }
    }
}
impl Default for PairOrders { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/pairorders.ts:147 PairOrders.glance, src/game/pairorders.ts:147)
#[derive(Clone, Copy, Debug)]
pub struct PairOrdersGlanceValue {
    pub on: bool,
    pub end_t: f64,
    pub h0: Option<[f64; 2]>,
}
impl PairOrdersGlanceValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairOrdersGlanceValue {
            on: false,
            end_t: 0.0,
            h0: None,
        }
    }
}
impl Default for PairOrdersGlanceValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairorders.ts:151 PairOrders.dark, src/game/pairorders.ts:151)
#[derive(Clone, Copy, Debug)]
pub struct PairOrdersDarkValue {
    pub check_t: f64,
    pub lit_t: f64,
    pub said_t: f64,
}
impl PairOrdersDarkValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairOrdersDarkValue {
            check_t: 0.0,
            lit_t: 0.0,
            said_t: 0.0,
        }
    }
}
impl Default for PairOrdersDarkValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairorders.ts:152 PairOrders.darkMemo, src/game/pairorders.ts:152)
#[derive(Clone, Copy, Debug)]
pub struct PairOrdersDarkMemoValue {
    pub t: f64,
    pub spot: Option<V3>,
}
impl PairOrdersDarkMemoValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairOrdersDarkMemoValue {
            t: 0.0,
            spot: None,
        }
    }
}
impl Default for PairOrdersDarkMemoValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairorders.ts:154 PairOrders.ways, src/game/pairorders.ts:154)
#[derive(Clone, Debug)]
pub struct PairOrdersWaysValue {
    pub to: V3,
    pub path: Vec<V3>,
    pub t: f64,
}
impl PairOrdersWaysValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairOrdersWaysValue {
            to: [0.0; 3],
            path: Vec::new(),
            t: 0.0,
        }
    }
}
impl Default for PairOrdersWaysValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairorders.ts:113 PAIR_LOG, src/game/pairorders.ts:113)
#[derive(Clone, Debug)]
pub struct PairordersModPAIRLOGItem {
    pub t: f64,
    pub what: String,
    /// INVENTED: g.serial() at the push: lesson.ts:113 holds a row as a cursor by identity
    pub serial: u32,
}
impl PairordersModPAIRLOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairordersModPAIRLOGItem {
            t: 0.0,
            what: String::new(),
            serial: 0,
        }
    }
}
impl Default for PairordersModPAIRLOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pairorders.ts:181 jobOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct PairOrdersJobOf {
    pub foe: GuardId,
    pub mode: crate::pairorders::PairOrdersJobOfMode,
    pub go: bool,
    pub set: bool,
    pub set_said: bool,
    pub t0: f64,
    pub born: f64,
    pub check_t: f64,
    pub wait_t: f64,
    pub heel_t: f64,
    pub going: bool,
    pub scuffled: bool,
    pub was_following: bool,
    pub handed: Option<bool>,
    pub watcher: Option<GuardId>,
    pub watch_t: f64,
    pub seen_t: f64,
    pub force: bool,
    pub cue_t: f64,
    pub cue_by: Option<GuardId>,
    pub cued: bool,
    pub back_said: Option<bool>,
    pub back_t: Option<f64>,
}
impl PairOrdersJobOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairOrdersJobOf {
            foe: GuardId(0),
            mode: crate::pairorders::PairOrdersJobOfMode::Mark,
            go: false,
            set: false,
            set_said: false,
            t0: 0.0,
            born: 0.0,
            check_t: 0.0,
            wait_t: 0.0,
            heel_t: 0.0,
            going: false,
            scuffled: false,
            was_following: false,
            handed: None,
            watcher: None,
            watch_t: 0.0,
            seen_t: 0.0,
            force: false,
            cue_t: 0.0,
            cue_by: None,
            cued: false,
            back_said: None,
            back_t: None,
        }
    }
}
impl Default for PairOrdersJobOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PairOrdersJobOfMode {
    Mark,
    Now,
}
impl PairOrdersJobOfMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            PairOrdersJobOfMode::Mark => "mark",
            PairOrdersJobOfMode::Now => "now",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PairOrdersMarkBy {
    Row,
    Hands,
    Api,
    F,
}
impl PairOrdersMarkBy {
    pub const fn as_str(self) -> &'static str {
        match self {
            PairOrdersMarkBy::Row => "row",
            PairOrdersMarkBy::Hands => "hands",
            PairOrdersMarkBy::Api => "api",
            PairOrdersMarkBy::F => "F",
        }
    }
}

/// (TypeScript: inline at src/game/pairorders.ts:493 stalk ->, src/game/pairorders.ts:493)
#[derive(Clone, Debug)]
pub struct PairOrdersStalk {
    pub path: Vec<V3>,
    pub exposed: bool,
    pub by: Option<GuardId>,
}
impl PairOrdersStalk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PairOrdersStalk {
            path: Vec::new(),
            exposed: false,
            by: None,
        }
    }
}
impl Default for PairOrdersStalk { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PairordersMod {
    /// (src/game/pairorders.ts:113 PAIR_LOG)
    pub pair_log: Vec<crate::pairorders::PairordersModPAIRLOGItem>,
}
impl PairordersMod {
    pub fn blank() -> Self {
        PairordersMod {
            pair_log: Vec::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PairordersStats {
    pub given: i32,
    pub marks: i32,
    pub taken: i32,
    pub blown: i32,
    pub called: i32,
    pub cancelled: i32,
    pub held: i32,
    pub dark: i32,
    pub lit: i32,
    pub watched: i32,
    pub anyway: i32,
}

/// (src/game/pairorders.ts:110 STEADY_BREAK)
pub const STEADY_BREAK: bool = false;

/// (src/game/pairorders.ts:123 ROWS: a table nothing writes)
pub fn rows() -> Vec<String> { alloc::vec![String::from("pairMark"), String::from("pairWatch"), String::from("pairTake"), String::from("pairDark"), String::from("pairGo")] }

/// (src/game/pairorders.ts:124 TAKES: a table nothing writes)
pub fn takes() -> Vec<String> { alloc::vec![String::from("pairMark"), String::from("pairTake")] }
