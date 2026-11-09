//! The types of `morale`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Training {
    #[default] None,
    Trained,
    Green,
    Difficulty,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Nerve {
    #[default] Steady,
    Normal,
    Brittle,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleAct {
    #[default] Down,
    Reload,
    Burst,
    None,
    Back,
    Hold,
    Stand,
    Flinch,
    Duck,
    Bait,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Mind {
    pub shock: f64,
    pub alone: bool,
    pub dark: bool,
    pub lit: f64,
    pub ask: f64,
    pub rung: f64,
    pub green: bool,
    pub leader_down: bool,
    pub fall: f64,
    pub lost: bool,
    pub by: Option<GuardId>,
    pub heard: f64,
    pub said: bool,
    pub broke_t: f64,
    pub act: crate::morale::MoraleAct,
    pub t: f64,
    pub hole: Option<V3>,
    pub goal: Option<V3>,
    pub rounds: f64,
    pub next: f64,
    pub calls: f64,
    pub call_t: f64,
    pub baits: f64,
    pub at_hole: bool,
    pub bark: f64,
    pub shout: bool,
    pub drive: bool,
    pub drive_t: f64,
    pub script: crate::game::GuardScript,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Blast {
    pub at: V3,
    pub t: f64,
    pub who: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LineKind {
    Clear,
    Out,
    Lost,
    Spent,
    Order,
    Again,
    Duck,
    Call,
    Okay,
}
impl LineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineKind::Clear => "clear",
            LineKind::Out => "out",
            LineKind::Lost => "lost",
            LineKind::Spent => "spent",
            LineKind::Order => "order",
            LineKind::Again => "again",
            LineKind::Duck => "duck",
            LineKind::Call => "call",
            LineKind::Okay => "okay",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct MoraleModMORALELOGItem {
    pub id: f64,
    pub t: f64,
    pub from: V3,
    pub at: V3,
    pub of: crate::morale::MoraleModMORALELOGItemOf,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleModMORALELOGItemOf {
    #[default] Dust,
    Bait,
}

#[derive(Clone, Debug, Default)]
pub struct LINESValue {
    pub say: Vec<String>,
    pub he: Option<Vec<String>>,
    pub they: Option<Vec<String>>,
    pub dark: Option<Vec<String>>,
    pub lit: Option<Vec<String>>,
    pub now: Option<Vec<String>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MoraleModMixAtValue {
    pub key: String,
    pub mix: crate::morale::MoraleModMixAtValueMix,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MoraleModMixAtValueMix {
    #[default] Hard,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MixNow {
    Hard,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TrainingOf {
    Steady,
    Green,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoundOf {
    Dust,
    Bait,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MoraleOf {
    pub drive: bool,
    pub heard: f64,
    pub t: f64,
    pub lost: bool,
    pub next: f64,
    pub hole: Option<V3>,
    pub act: crate::morale::MoraleAct,
    pub goal: Option<V3>,
    pub lit: f64,
    pub alone: bool,
    pub said: bool,
    pub ask: f64,
    pub by: Option<GuardId>,
    pub green: bool,
    pub dark: bool,
    pub fall: f64,
    pub drive_t: f64,
    pub calls: f64,
    pub rounds: f64,
    pub shout: bool,
    pub shock: f64,
    pub rung: f64,
    pub leader_down: bool,
    pub broke_t: f64,
    pub call_t: f64,
    pub baits: f64,
    pub at_hole: bool,
    pub bark: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct MoraleMod {
    /// (src/game/morale.ts:111 MORALE_LOG)
    pub morale_log: Vec<crate::morale::MoraleModMORALELOGItem>,
    /// (src/game/morale.ts:128 stateOf)
    pub state_of: JsMap<GuardId, crate::morale::Mind>,
    /// (src/game/morale.ts:132 blastsOf)
    pub blasts_of: Option<Vec<crate::morale::Blast>>,
    /// (src/game/morale.ts:219 mixAt)
    pub mix_at: Option<crate::morale::MoraleModMixAtValue>,
}
impl MoraleMod {
    pub fn blank() -> Self {
        MoraleMod {
            morale_log: Vec::new(),
            state_of: JsMap::new(),
            blasts_of: None,
            mix_at: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct MoraleStats {
    pub blasts: i32,
    pub fed: i32,
    pub ducks: i32,
    pub broke: i32,
    pub rounds: i32,
    pub calls: i32,
    pub shouts: i32,
    pub baits: i32,
    pub reloads: i32,
    pub orders: i32,
    pub falls: i32,
    pub flinches: i32,
}

/// (src/game/morale.ts:106 MORALE_AT: a table nothing writes)
pub fn morale_at() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("standard"), String::from("mixed")); t.set(String::from("hard"), String::from("hard")); t } }

/// (src/game/morale.ts:108 ALERT_AW)
pub const ALERT_AW: f64 = 0.85;

/// (src/game/morale.ts:142 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::morale::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("duck"), crate::morale::LINESValue { say: alloc::vec![String::from("down! get down!"), String::from("get down — stay down!"), String::from("heads down!")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("call"), crate::morale::LINESValue { say: Vec::new(), he: Some(alloc::vec![String::from("he's coming in — I've got nobody in here!"), String::from("anyone on this net? he's right on top of me!")]), they: Some(alloc::vec![String::from("they're coming in — I've got nobody in here!"), String::from("anyone on this net? they're right on top of me!")]), dark: Some(alloc::vec![String::from("can't see a thing — somebody get in here!")]), lit: Some(alloc::vec![String::from("right in my room — somebody get in here!")]), now: None }); t.set(String::from("again"), crate::morale::LINESValue { say: alloc::vec![String::from("is anybody there? answer me!"), String::from("nobody's coming. nobody's coming—"), String::from("please — somebody answer!")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("out"), crate::morale::LINESValue { say: alloc::vec![String::from("I'm out! I'm out!"), String::from("empty — I'm empty!"), String::from("I'm dry! changing!")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("spent"), crate::morale::LINESValue { say: alloc::vec![String::from("no — no, stay back!"), String::from("keep away from me!"), String::from("don't — don't come in!")], he: None, they: None, dark: None, lit: None, now: Some(alloc::vec![String::from("no — no, stay back!"), String::from("stay away from me!"), String::from("don't — don't come in!")]) }); t.set(String::from("clear"), crate::morale::LINESValue { say: alloc::vec![String::from("breach on my room. I'm alone in here.")], he: Some(alloc::vec![String::from("he's breached. I'm on my own in here."), String::from("he's through. holding it on my own.")]), they: Some(alloc::vec![String::from("they've breached. I'm on my own in here."), String::from("they're through. holding it on my own.")]), dark: None, lit: None, now: None }); t.set(String::from("lost"), crate::morale::LINESValue { say: alloc::vec![String::from("they got him! they got him!"), String::from("he's down. oh no, he's down."), String::from("we've lost him! what do we do?")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("order"), crate::morale::LINESValue { say: alloc::vec![String::from("get up. watch that hole. breathe."), String::from("on your feet. eyes on that hole."), String::from("up. gun on the hole. breathe.")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("okay"), crate::morale::LINESValue { say: alloc::vec![String::from("okay — okay."), String::from("yeah. yeah, I'm up."), String::from("I'm up. I'm up.")], he: None, they: None, dark: None, lit: None, now: None }); t } }

