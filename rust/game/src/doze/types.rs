//! The types of `doze`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DozeWhy {
    Light,
    Alarm,
    Noise,
    Taken,
    Time,
    Bump,
    Net,
}
impl DozeWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            DozeWhy::Light => "light",
            DozeWhy::Alarm => "alarm",
            DozeWhy::Noise => "noise",
            DozeWhy::Taken => "taken",
            DozeWhy::Time => "time",
            DozeWhy::Bump => "bump",
            DozeWhy::Net => "net",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Down,
    Work,
    Up,
    Spent,
    Going,
    No,
    Asleep,
    Stir,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Down => "down",
            Phase::Work => "work",
            Phase::Up => "up",
            Phase::Spent => "spent",
            Phase::Going => "going",
            Phase::No => "no",
            Phase::Asleep => "asleep",
            Phase::Stir => "stir",
        }
    }
}

/// (TypeScript: Doze, src/game/doze.ts:81)
#[derive(Clone, Copy, Debug)]
pub struct Doze {
    pub phase: crate::doze::Phase,
    pub wp: f64,
    pub chair: Option<PropId>,
    pub mon: Option<FixtureId>,
    pub mon_was: bool,
    pub say: f64,
    pub to: V3,
    pub at: V3,
    pub face: f64,
    pub cx: f64,
    pub cz: f64,
    pub cyaw: f64,
    pub t: f64,
    pub seen: f64,
    pub until: f64,
    pub lit: f64,
    pub ask: f64,
    pub push: f64,
    pub why: Option<crate::doze::DozeWhy>,
    pub woke_at: f64,
}
impl Doze {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Doze {
            phase: crate::doze::Phase::Down,
            wp: 0.0,
            chair: None,
            mon: None,
            mon_was: false,
            say: 0.0,
            to: [0.0; 3],
            at: [0.0; 3],
            face: 0.0,
            cx: 0.0,
            cz: 0.0,
            cyaw: 0.0,
            t: 0.0,
            seen: 0.0,
            until: 0.0,
            lit: 0.0,
            ask: 0.0,
            push: 0.0,
            why: None,
            woke_at: 0.0,
        }
    }
}
impl Default for Doze { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Seat, src/game/doze.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct Seat {
    pub chair: PropId,
    pub at: V3,
    pub face: f64,
    pub mon: Option<FixtureId>,
    pub to: V3,
}
impl Seat {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Seat {
            chair: PropId(0),
            at: [0.0; 3],
            face: 0.0,
            mon: None,
            to: [0.0; 3],
        }
    }
}
impl Default for Seat { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DozeSnap, src/game/doze.ts:355)
#[derive(Clone, Debug)]
pub struct DozeSnap {
    pub men: Vec<crate::doze::DozeSnapMenItem>,
    pub said: Option<Vec<f64>>,
    pub left: Option<Vec<Vec<f64>>>,
}
impl DozeSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DozeSnap {
            men: Vec::new(),
            said: None,
            left: None,
        }
    }
}
impl Default for DozeSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doze.ts:355 DozeSnap.men, src/game/doze.ts:355)
#[derive(Clone, Debug)]
pub struct DozeSnapMenItem {
    pub i: f64,
    pub phase: crate::doze::Phase,
    pub wp: f64,
    pub chair: f64,
    pub mon: Option<f64>,
    pub mon_was: Option<bool>,
    pub say: Option<f64>,
    pub to: Option<Vec<f64>>,
    pub at: Vec<f64>,
    pub face: f64,
    pub cx: f64,
    pub cz: f64,
    pub cyaw: f64,
    pub t: f64,
    pub until: f64,
    pub lit: f64,
    pub ask: Option<f64>,
    pub why: Option<crate::doze::DozeWhy>,
    pub woke_at: f64,
}
impl DozeSnapMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DozeSnapMenItem {
            i: 0.0,
            phase: crate::doze::Phase::Down,
            wp: 0.0,
            chair: 0.0,
            mon: None,
            mon_was: None,
            say: None,
            to: None,
            at: Vec::new(),
            face: 0.0,
            cx: 0.0,
            cz: 0.0,
            cyaw: 0.0,
            t: 0.0,
            until: 0.0,
            lit: 0.0,
            ask: None,
            why: None,
            woke_at: 0.0,
        }
    }
}
impl Default for DozeSnapMenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/doze.ts:97 DOZE_LOG, src/game/doze.ts:97)
#[derive(Clone, Debug)]
pub struct DozeModDOZELOGItem {
    pub t: f64,
    pub who: String,
    pub why: crate::doze::DozeWhy,
    pub secs: f64,
}
impl DozeModDOZELOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DozeModDOZELOGItem {
            t: 0.0,
            who: String::new(),
            why: crate::doze::DozeWhy::Light,
            secs: 0.0,
        }
    }
}
impl Default for DozeModDOZELOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WakeWhy {
    Light,
    Alarm,
    Noise,
    Taken,
    Bump,
    Net,
}
impl WakeWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            WakeWhy::Light => "light",
            WakeWhy::Alarm => "alarm",
            WakeWhy::Noise => "noise",
            WakeWhy::Taken => "taken",
            WakeWhy::Bump => "bump",
            WakeWhy::Net => "net",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DozeAt {
    Screen,
    Asleep,
}
impl DozeAt {
    pub const fn as_str(self) -> &'static str {
        match self {
            DozeAt::Screen => "screen",
            DozeAt::Asleep => "asleep",
        }
    }
}

/// (TypeScript: inline at src/game/doze.ts:344 dozeOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct DozeOf {
    pub phase: crate::doze::Phase,
    pub why: Option<crate::doze::DozeWhy>,
    pub secs: f64,
    pub wp: f64,
}
impl DozeOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DozeOf {
            phase: crate::doze::Phase::Down,
            why: None,
            secs: 0.0,
            wp: 0.0,
        }
    }
}
impl Default for DozeOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct DozeMod {
    /// (src/game/doze.ts:82 GAMES)
    pub games: Option<JsMap<GuardId, crate::doze::Doze>>,
    /// (src/game/doze.ts:83 REC)
    pub rec: JsMap<GuardId, crate::doze::Doze>,
    /// (src/game/doze.ts:84 SAID)
    pub said: Option<Vec<f64>>,
    /// (src/game/doze.ts:87 PULLS)
    pub pulls: Option<Vec<[f64; 2]>>,
    /// (src/game/doze.ts:87 PULLS_M)
    pub pulls_m: f64,
    /// (src/game/doze.ts:87 PULLS_E)
    pub pulls_e: f64,
    /// (src/game/doze.ts:97 DOZE_LOG)
    pub doze_log: Vec<crate::doze::DozeModDOZELOGItem>,
    /// (src/game/doze.ts:117 LEFT)
    pub left: Option<JsMap<FixtureId, f64>>,
    /// (src/game/doze.ts:334 EARS)
    pub ears: [f64; 2],
}
impl DozeMod {
    pub fn blank() -> Self {
        DozeMod {
            games: None,
            rec: JsMap::new(),
            said: None,
            pulls: None,
            pulls_m: -1.0,
            pulls_e: -1.0,
            doze_log: Vec::new(),
            left: None,
            ears: [1.0, 1.0],
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DozeStats {
    pub sat: i32,
    pub slept: i32,
    pub worked: i32,
    pub lines: i32,
    pub rose: i32,
    pub woke: [i32; 6],
}

/// (src/game/doze.ts:85 TOUCH: a table nothing writes)
pub fn touch() -> Vec<i32> { alloc::vec![0; 96] }

/// (src/game/doze.ts:129 SCREEN_LINES: a table nothing writes)
pub fn screen_lines() -> Vec<String> { alloc::vec![String::from("password expired. of course it has."), String::from("reply all. he replied all."), String::from("…and it's frozen. wonderful."), String::from("who formats a duty roster like this."), String::from("no. no, I saved that."), String::from("page two of two. there is no page two."), String::from("the overtime form wants a form."), String::from("mandatory fun. who writes these."), String::from("loading… still loading."), String::from("printer offline. it's right there."), String::from("one more hand. then the rounds."), String::from("unread, unread, unread. all him.")] }

/// (src/game/doze.ts:334 WHOLE: a table nothing writes)
pub fn whole() -> [f64; 2] { [1.0, 1.0] }
