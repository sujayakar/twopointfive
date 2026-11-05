//! The types of `nightbreach`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: BlindBurst, src/game/nightbreach.ts:107)
#[derive(Clone, Copy, Debug)]
pub struct BlindBurst {
    pub until: f64,
    pub n: f64,
    pub next: f64,
}
impl BlindBurst {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlindBurst {
            until: 0.0,
            n: 0.0,
            next: 0.0,
        }
    }
}
impl Default for BlindBurst { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NightBreachMark, src/game/nightbreach.ts:128)
#[derive(Clone, Debug)]
pub struct NightBreachMark {
    pub pos: V3,
    pub aim_at: V3,
    pub name: Option<String>,
    pub drill: Option<bool>,
}
impl NightBreachMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachMark {
            pos: [0.0; 3],
            aim_at: [0.0; 3],
            name: None,
            drill: None,
        }
    }
}
impl Default for NightBreachMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NightBreachLayout, src/game/nightbreach.ts:132)
#[derive(Clone, Debug)]
pub struct NightBreachLayout {
    pub name: String,
}
impl NightBreachLayout {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachLayout {
            name: String::new(),
        }
    }
}
impl Default for NightBreachLayout { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NightBreachState {
    Clear,
    Set,
    Failed,
    Off,
    Going,
    Staged,
    Stacking,
    Inside,
    Recalled,
}
impl NightBreachState {
    pub const fn as_str(self) -> &'static str {
        match self {
            NightBreachState::Clear => "clear",
            NightBreachState::Set => "set",
            NightBreachState::Failed => "failed",
            NightBreachState::Off => "off",
            NightBreachState::Going => "going",
            NightBreachState::Staged => "staged",
            NightBreachState::Stacking => "stacking",
            NightBreachState::Inside => "inside",
            NightBreachState::Recalled => "recalled",
        }
    }
}

/// (TypeScript: NightBreach, src/game/nightbreach.ts:145)
#[derive(Clone, Debug)]
pub struct NightBreach {
    pub active: bool,
    pub drill: Option<crate::breach::BreachDrill>,
    pub ops: Vec<GuardId>,
    pub hostiles: Vec<GuardId>,
    pub layout: Option<crate::nightbreach::NightBreachLayout>,
    pub marks: Vec<crate::nightbreach::NightBreachMark>,
    pub staged: f64,
    pub log: Vec<crate::nightbreach::NightBreachLogItem>,
    pub no_drill: bool,
    pub bang_taken: bool,
    pub bang_by: Option<SeatId>,
    pub blind: bool,
    pub bursts: Vec<crate::nightbreach::BlindBurst>,
    pub blind_rounds: f64,
    pub benched: Option<Vec<GuardId>>,
    pub beats_seen: f64,
    pub said: JsSet<String>,
    pub cut: bool,
    pub failure: Option<String>,
    pub obj_of: Option<crate::nightbreach::NightBreachObjOf>,
}
impl NightBreach {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        NightBreach {
            active: false,
            drill: None,
            ops: Vec::new(),
            hostiles: Vec::new(),
            layout: None,
            marks: Vec::new(),
            staged: 0.0,
            log: Vec::new(),
            no_drill: false,
            bang_taken: false,
            bang_by: None,
            blind: false,
            bursts: Vec::new(),
            blind_rounds: 0.0,
            benched: None,
            beats_seen: 0.0,
            said: JsSet::new(),
            cut: false,
            failure: None,
            obj_of: None,
        }
    }
}
impl Default for NightBreach { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/nightbreach.ts:160 NightBreach.log, src/game/nightbreach.ts:160)
#[derive(Clone, Debug)]
pub struct NightBreachLogItem {
    pub t: f64,
    pub what: String,
}
impl NightBreachLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for NightBreachLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nightbreach.ts:386 NightBreach.objOf, src/game/nightbreach.ts:386)
#[derive(Clone, Debug)]
pub struct NightBreachObjOf {
    pub taken: bool,
    pub o: crate::nightbreach::NightBreachObjOfO,
}
impl NightBreachObjOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachObjOf {
            taken: false,
            o: crate::nightbreach::NightBreachObjOfO::zeroed(),
        }
    }
}
impl Default for NightBreachObjOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nightbreach.ts:386 NightBreachObjOf.o, src/game/nightbreach.ts:386)
#[derive(Clone, Debug)]
pub struct NightBreachObjOfO {
    pub text: String,
    pub pos: Option<V3>,
}
impl NightBreachObjOfO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachObjOfO {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for NightBreachObjOfO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nightbreach.ts:88 SQUAD4, src/game/nightbreach.ts:88)
#[derive(Clone, Debug)]
pub struct SQUAD4Item {
    pub name: String,
    pub pair: String,
    pub tint: V3,
    pub tint2: V3,
}
impl SQUAD4Item {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SQUAD4Item {
            name: String::new(),
            pair: String::new(),
            tint: [0.0; 3],
            tint2: [0.0; 3],
        }
    }
}
impl Default for SQUAD4Item { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/nightbreach.ts:104 BLIND, src/game/nightbreach.ts:104)
#[derive(Clone, Copy, Debug)]
pub struct BLIND {
    pub after: f64,
    pub stagger: f64,
    pub rounds: f64,
    pub every: f64,
    pub spread: f64,
    pub low_gun: f64,
}
impl BLIND {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BLIND {
            after: 0.0,
            stagger: 0.0,
            rounds: 0.0,
            every: 0.0,
            spread: 0.0,
            low_gun: 0.0,
        }
    }
}
impl Default for BLIND { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NightBreachOrder {
    Stack,
    Go,
}
impl NightBreachOrder {
    pub const fn as_str(self) -> &'static str {
        match self {
            NightBreachOrder::Stack => "stack",
            NightBreachOrder::Go => "go",
        }
    }
}

/// (TypeScript: inline at src/game/nightbreach.ts:378 objective ->, src/game/nightbreach.ts:378)
#[derive(Clone, Debug)]
pub struct NightBreachObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl NightBreachObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for NightBreachObjective { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct NightbreachMod {
    /// (src/game/nightbreach.ts:140 applied)
    pub applied: Option<crate::scene_scenarios::TeamStart>,
}
impl NightbreachMod {
    pub fn blank() -> Self {
        NightbreachMod {
            applied: Some(crate::scene_scenarios::TeamStart { sam: [9.6, 0.0, 20.3], sam_yaw: 1.5707963267948966, ops: None, ops_yaw: None }),
        }
    }
}

/// (src/game/nightbreach.ts:88 SQUAD4: a table nothing writes)
pub fn squad4() -> Vec<crate::nightbreach::SQUAD4Item> { alloc::vec![crate::nightbreach::SQUAD4Item { name: String::from("Red One"), pair: String::from("red"), tint: [0.2, 0.085, 0.075], tint2: [0.55, 0.07, 0.05] }, crate::nightbreach::SQUAD4Item { name: String::from("Red Two"), pair: String::from("red"), tint: [0.2, 0.085, 0.075], tint2: [0.55, 0.07, 0.05] }, crate::nightbreach::SQUAD4Item { name: String::from("Blue One"), pair: String::from("blue"), tint: [0.06, 0.075, 0.13], tint2: [0.07, 0.2, 0.6] }, crate::nightbreach::SQUAD4Item { name: String::from("Blue Two"), pair: String::from("blue"), tint: [0.06, 0.075, 0.13], tint2: [0.07, 0.2, 0.6] }] }

/// (src/game/nightbreach.ts:96 SQUAD4_BUDGET)
pub const SQUAD4_BUDGET: f64 = 10.0;

/// (src/game/nightbreach.ts:98 NONE: a table nothing writes)
pub fn none() -> Vec<GuardId> { Vec::new() }

/// (src/game/nightbreach.ts:99 NUMBER: a table nothing writes)
pub fn number() -> Vec<String> { alloc::vec![String::from("no"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six")] }

/// (src/game/nightbreach.ts:104 BLIND: a table nothing writes)
pub fn blind() -> crate::nightbreach::BLIND { crate::nightbreach::BLIND { after: 0.3, stagger: 0.07, rounds: 12.0, every: 0.1, spread: 0.3, low_gun: 0.21 } }
