//! The types of `doze`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum DozeWhy {
    Light,
    Alarm,
    Noise,
    Taken,
    Time,
    Net,
    Bump,
}
impl DozeWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            DozeWhy::Light => "light",
            DozeWhy::Alarm => "alarm",
            DozeWhy::Noise => "noise",
            DozeWhy::Taken => "taken",
            DozeWhy::Time => "time",
            DozeWhy::Net => "net",
            DozeWhy::Bump => "bump",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default)]
pub enum Phase {
    #[default] Down,
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Seat {
    pub chair: PropId,
    pub at: V3,
    pub face: f64,
    pub mon: Option<FixtureId>,
    pub to: V3,
}

#[derive(Clone, Debug, Default)]
pub struct DozeSnap {
    pub men: Vec<crate::doze::DozeSnapMenItem>,
    pub said: Option<Vec<f64>>,
    pub left: Option<Vec<Vec<f64>>>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WakeWhy {
    Light,
    Alarm,
    Noise,
    Taken,
    Net,
    Bump,
}
impl WakeWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            WakeWhy::Light => "light",
            WakeWhy::Alarm => "alarm",
            WakeWhy::Noise => "noise",
            WakeWhy::Taken => "taken",
            WakeWhy::Net => "net",
            WakeWhy::Bump => "bump",
        }
    }
}

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

#[derive(Clone, Copy, Debug, Default)]
pub struct DozeOf {
    pub phase: crate::doze::Phase,
    pub why: Option<crate::doze::DozeWhy>,
    pub secs: f64,
    pub wp: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DozeMod {
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
    /// (src/game/doze.ts:117 LEFT)
    pub left: Option<JsMap<FixtureId, f64>>,
}
impl DozeMod {
    pub fn blank() -> Self {
        DozeMod {
            rec: JsMap::new(),
            said: None,
            pulls: None,
            pulls_m: -1.0,
            pulls_e: -1.0,
            left: None,
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

/// (src/game/doze.ts:331 WHOLE: a table nothing writes)
pub fn whole() -> [f64; 2] { [1.0, 1.0] }
