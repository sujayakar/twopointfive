//! The types of `darkdoor`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DarkDoorAct {
    #[default] None,
    With,
    Balk,
    Solo,
    Come,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Fear {
    pub act: crate::darkdoor::DarkDoorAct,
    pub t: f64,
    pub door: f64,
    pub side: f64,
    pub spot: Option<V3>,
    pub at: bool,
    pub other: Option<GuardId>,
    pub called: bool,
    pub call_t: f64,
    pub tried: Vec<GuardId>,
    pub left: f64,
    pub told: bool,
    pub lag: f64,
    pub armed: f64,
    pub arm_t: f64,
    pub quits: Vec<crate::darkdoor::FearQuitsItem>,
    pub pass: Vec<crate::darkdoor::FearPassItem>,
    pub said: f64,
    pub idle: f64,
    pub ask: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FearQuitsItem {
    pub room: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FearPassItem {
    pub door: f64,
    pub until: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Read {
    pub t: f64,
    pub dark: bool,
    pub mean: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum LineKind {
    With,
    Copy,
    Alone,
    Self_,
    Call,
    Nobody,
}
impl LineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineKind::With => "with",
            LineKind::Copy => "copy",
            LineKind::Alone => "alone",
            LineKind::Self_ => "self",
            LineKind::Call => "call",
            LineKind::Nobody => "nobody",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LINESValue {
    pub say: Vec<String>,
    pub he: Option<Vec<String>>,
    pub they: Option<Vec<String>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FightWalk {
    Other,
    Seen,
    Chase,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DarkRoom {
    pub dark: bool,
    pub mean: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StandsLitR {
    pub mean: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorAhead {
    pub d: DoorId,
    pub side: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DarkdoorMod {
    /// (src/game/darkdoor.ts:76 stateOf)
    pub state_of: JsMap<GuardId, crate::darkdoor::Fear>,
    /// (src/game/darkdoor.ts:79 readsOf)
    pub reads_of: Option<JsMap<crate::Num, crate::darkdoor::Read>>,
    /// (src/game/darkdoor.ts:81 cellsOf)
    pub cells_of: Option<JsMap<crate::Num, Vec<f64>>>,
}
impl DarkdoorMod {
    pub fn blank() -> Self {
        DarkdoorMod {
            state_of: JsMap::new(),
            reads_of: None,
            cells_of: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DarkdoorStats {
    pub balks: i32,
    pub calls: i32,
    pub pairs: i32,
    pub quits: i32,
    pub solos: i32,
    pub drops: i32,
    pub asks: i32,
}

/// (src/game/darkdoor.ts:88 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::darkdoor::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("call"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("I'm not going in there alone. get up here."), String::from("not in there. not on my own — get up here!")], he: Some(alloc::vec![String::from("he could be anywhere in that. somebody get up here.")]), they: Some(alloc::vec![String::from("they could be anywhere in that. somebody get up here.")]) }); t.set(String::from("self"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("no. not in there. not on my own."), String::from("pitch black. no. not alone."), String::from("not a chance. not by myself.")], he: None, they: None }); t.set(String::from("copy"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("copy. coming."), String::from("on my way. stay put."), String::from("hold there — coming to you.")], he: None, they: None }); t.set(String::from("with"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("on you. get a light in there."), String::from("I'm here. lights on — go."), String::from("right behind you. go on.")], he: None, they: None }); t.set(String::from("nobody"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("nobody's coming."), String::from("no one. all right — leave it."), String::from("nobody. forget that room.")], he: None, they: None }); t.set(String::from("alone"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("fine. fine — I'll do it myself."), String::from("nobody ever comes. going in."), String::from("all right. slow. going in.")], he: None, they: None }); t } }
