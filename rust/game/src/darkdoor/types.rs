//! The types of `darkdoor`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: DarkDoorSnap, src/game/darkdoor.ts:361)
#[derive(Clone, Debug)]
pub struct DarkDoorSnap {
    pub men: Vec<Option<Vec<f64>>>,
    pub quits: Vec<Option<Vec<f64>>>,
    pub pass: Vec<Option<Vec<f64>>>,
    pub tried: Vec<Option<Vec<f64>>>,
    pub reads: Vec<Vec<f64>>,
    pub dealt: Option<f64>,
}
impl DarkDoorSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkDoorSnap {
            men: Vec::new(),
            quits: Vec::new(),
            pass: Vec::new(),
            tried: Vec::new(),
            reads: Vec::new(),
            dealt: None,
        }
    }
}
impl Default for DarkDoorSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DarkDoorAct {
    None,
    With,
    Balk,
    Solo,
    Come,
}
impl DarkDoorAct {
    pub const fn as_str(self) -> &'static str {
        match self {
            DarkDoorAct::None => "none",
            DarkDoorAct::With => "with",
            DarkDoorAct::Balk => "balk",
            DarkDoorAct::Solo => "solo",
            DarkDoorAct::Come => "come",
        }
    }
}

/// (TypeScript: Fear, src/game/darkdoor.ts:73)
#[derive(Clone, Debug)]
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
impl Fear {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Fear {
            act: crate::darkdoor::DarkDoorAct::None,
            t: 0.0,
            door: 0.0,
            side: 0.0,
            spot: None,
            at: false,
            other: None,
            called: false,
            call_t: 0.0,
            tried: Vec::new(),
            left: 0.0,
            told: false,
            lag: 0.0,
            armed: 0.0,
            arm_t: 0.0,
            quits: Vec::new(),
            pass: Vec::new(),
            said: 0.0,
            idle: 0.0,
            ask: 0.0,
        }
    }
}
impl Default for Fear { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkdoor.ts:75 Fear.quits, src/game/darkdoor.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct FearQuitsItem {
    pub room: f64,
    pub t: f64,
}
impl FearQuitsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FearQuitsItem {
            room: 0.0,
            t: 0.0,
        }
    }
}
impl Default for FearQuitsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkdoor.ts:75 Fear.pass, src/game/darkdoor.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct FearPassItem {
    pub door: f64,
    pub until: f64,
}
impl FearPassItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FearPassItem {
            door: 0.0,
            until: 0.0,
        }
    }
}
impl Default for FearPassItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Read, src/game/darkdoor.ts:77)
#[derive(Clone, Copy, Debug)]
pub struct Read {
    pub t: f64,
    pub dark: bool,
    pub share: f64,
    pub mean: f64,
}
impl Read {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Read {
            t: 0.0,
            dark: false,
            share: 0.0,
            mean: 0.0,
        }
    }
}
impl Default for Read { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: inline at src/game/darkdoor.ts:88 LINES, src/game/darkdoor.ts:88)
#[derive(Clone, Debug)]
pub struct LINESValue {
    pub say: Vec<String>,
    pub he: Option<Vec<String>>,
    pub they: Option<Vec<String>>,
}
impl LINESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESValue {
            say: Vec::new(),
            he: None,
            they: None,
        }
    }
}
impl Default for LINESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FightWalk {
    Other,
    Seen,
    Chase,
}
impl FightWalk {
    pub const fn as_str(self) -> &'static str {
        match self {
            FightWalk::Other => "other",
            FightWalk::Seen => "seen",
            FightWalk::Chase => "chase",
        }
    }
}

/// (TypeScript: inline at src/game/darkdoor.ts:173 darkRoom ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct DarkRoom {
    pub t: f64,
    pub dark: bool,
    pub share: f64,
    pub mean: f64,
}
impl DarkRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkRoom {
            t: 0.0,
            dark: false,
            share: 0.0,
            mean: 0.0,
        }
    }
}
impl Default for DarkRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkdoor.ts:185 standsLit(R), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct StandsLitR {
    pub t: f64,
    pub dark: bool,
    pub share: f64,
    pub mean: f64,
}
impl StandsLitR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StandsLitR {
            t: 0.0,
            dark: false,
            share: 0.0,
            mean: 0.0,
        }
    }
}
impl Default for StandsLitR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkdoor.ts:192 doorAhead ->, src/game/darkdoor.ts:192)
#[derive(Clone, Copy, Debug)]
pub struct DoorAhead {
    pub d: DoorId,
    pub side: f64,
    pub at: V3,
    pub dist: f64,
}
impl DoorAhead {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorAhead {
            d: DoorId(0),
            side: 0.0,
            at: [0.0; 3],
            dist: 0.0,
        }
    }
}
impl Default for DoorAhead { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/darkdoor.ts:348 darkDoorOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Debug)]
pub struct DarkDoorOf {
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
impl DarkDoorOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DarkDoorOf {
            act: crate::darkdoor::DarkDoorAct::None,
            t: 0.0,
            door: 0.0,
            side: 0.0,
            spot: None,
            at: false,
            other: None,
            called: false,
            call_t: 0.0,
            tried: Vec::new(),
            left: 0.0,
            told: false,
            lag: 0.0,
            armed: 0.0,
            arm_t: 0.0,
            quits: Vec::new(),
            pass: Vec::new(),
            said: 0.0,
            idle: 0.0,
            ask: 0.0,
        }
    }
}
impl Default for DarkDoorOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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

/// (src/game/darkdoor.ts:66 ACTS: a table nothing writes)
pub fn acts() -> Vec<String> { alloc::vec![String::from("none"), String::from("balk"), String::from("solo"), String::from("come"), String::from("with")] }

/// (src/game/darkdoor.ts:88 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::darkdoor::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("call"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("I'm not going in there alone. get up here."), String::from("not in there. not on my own — get up here!")], he: Some(alloc::vec![String::from("he could be anywhere in that. somebody get up here.")]), they: Some(alloc::vec![String::from("they could be anywhere in that. somebody get up here.")]) }); t.set(String::from("self"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("no. not in there. not on my own."), String::from("pitch black. no. not alone."), String::from("not a chance. not by myself.")], he: None, they: None }); t.set(String::from("copy"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("copy. coming."), String::from("on my way. stay put."), String::from("hold there — coming to you.")], he: None, they: None }); t.set(String::from("with"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("on you. get a light in there."), String::from("I'm here. lights on — go."), String::from("right behind you. go on.")], he: None, they: None }); t.set(String::from("nobody"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("nobody's coming."), String::from("no one. all right — leave it."), String::from("nobody. forget that room.")], he: None, they: None }); t.set(String::from("alone"), crate::darkdoor::LINESValue { say: alloc::vec![String::from("fine. fine — I'll do it myself."), String::from("nobody ever comes. going in."), String::from("all right. slow. going in.")], he: None, they: None }); t } }
