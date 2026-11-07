//! The types of `roomclear`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Placement, src/game/roomclear.ts:89)
#[derive(Clone, Debug)]
pub struct Placement {
    pub room: String,
    pub mark: f64,
    pub pos: V3,
    pub watch: String,
    pub aim_at: V3,
    pub name: String,
}
impl Placement {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Placement {
            room: String::new(),
            mark: 0.0,
            pos: [0.0; 3],
            watch: String::new(),
            aim_at: [0.0; 3],
            name: String::new(),
        }
    }
}
impl Default for Placement { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomClearEnd {
    Taken,
    Wiped,
}
impl RoomClearEnd {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearEnd::Taken => "taken",
            RoomClearEnd::Wiped => "wiped",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoomClearState {
    Off,
    Taken,
    Wiped,
    Live,
}
impl RoomClearState {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoomClearState::Off => "off",
            RoomClearState::Taken => "taken",
            RoomClearState::Wiped => "wiped",
            RoomClearState::Live => "live",
        }
    }
}

/// (TypeScript: RoomState, src/game/roomclear.ts:227)
#[derive(Clone, Debug)]
pub struct RoomState {
    pub def: crate::scene_scenarios::StructureRoom,
    pub defenders: Vec<GuardId>,
    pub cleared: bool,
    pub cleared_at: f64,
}
impl RoomState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomState {
            def: crate::scene_scenarios::StructureRoom::zeroed(),
            defenders: Vec::new(),
            cleared: false,
            cleared_at: 0.0,
        }
    }
}
impl Default for RoomState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomClearScenario, src/game/roomclear.ts:229)
#[derive(Clone, Debug)]
pub struct RoomClearScenario {
    pub rooms: Vec<crate::roomclear::RoomState>,
    pub end: Option<crate::roomclear::RoomClearEnd>,
    pub end_at: f64,
    pub end_why: String,
    pub end_say: String,
    pub placement: Vec<crate::roomclear::Placement>,
    pub seed: f64,
    pub attempt: f64,
    pub fresh: f64,
    pub way: f64,
    pub log: Vec<crate::roomclear::RoomClearScenarioLogItem>,
    pub pair_said: bool,
    pub entry_said: bool,
    pub team_said: bool,
    pub clear_due: Vec<crate::scene_scenarios::StructureRoom>,
    pub clear_at: f64,
    pub obj: Option<crate::roomclear::RoomClearScenarioObj>,
    pub layout: crate::nightbreach::NightBreachLayout,
}
impl RoomClearScenario {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        RoomClearScenario {
            rooms: Vec::new(),
            end: None,
            end_at: f64::NAN,
            end_why: String::from(""),
            end_say: String::from("team wiped"),
            placement: Vec::new(),
            seed: 0.0,
            attempt: 0.0,
            fresh: 0.0,
            way: 0.0,
            log: Vec::new(),
            pair_said: false,
            entry_said: false,
            team_said: false,
            clear_due: Vec::new(),
            clear_at: f64::NAN,
            obj: None,
            layout: crate::nightbreach::NightBreachLayout { name: String::from("room clear") },
        }
    }
}
impl Default for RoomClearScenario { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/roomclear.ts:258 RoomClearScenario.log, src/game/roomclear.ts:258)
#[derive(Clone, Debug)]
pub struct RoomClearScenarioLogItem {
    pub t: f64,
    pub what: String,
}
impl RoomClearScenarioLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomClearScenarioLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for RoomClearScenarioLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:263 RoomClearScenario.obj, src/game/roomclear.ts:263)
#[derive(Clone, Debug)]
pub struct RoomClearScenarioObj {
    pub text: String,
    pub pos: Option<V3>,
    pub ticks: bool,
    pub hint: String,
}
impl RoomClearScenarioObj {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomClearScenarioObj {
            text: String::new(),
            pos: None,
            ticks: false,
            hint: String::new(),
        }
    }
}
impl Default for RoomClearScenarioObj { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:93 placeDefenders(S), src/game/roomclear.ts:93)
#[derive(Clone, Debug)]
pub struct PlaceDefendersS {
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
}
impl PlaceDefendersS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlaceDefendersS {
            rooms: Vec::new(),
        }
    }
}
impl Default for PlaceDefendersS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:112 checkStructure(S), src/game/roomclear.ts:112)
#[derive(Clone, Debug)]
pub struct CheckStructureS {
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
    pub team: Option<crate::scene_scenarios::TeamStart>,
    pub starts: Option<Vec<crate::scene_scenarios::TeamStart>>,
    pub max_men: Option<f64>,
    pub squads: Option<Vec<crate::scene_scenarios::SquadData>>,
}
impl CheckStructureS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckStructureS {
            rooms: Vec::new(),
            team: None,
            starts: None,
            max_men: None,
            squads: None,
        }
    }
}
impl Default for CheckStructureS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:158 checkSquads(S), src/game/roomclear.ts:158)
#[derive(Clone, Debug)]
pub struct CheckSquadsS {
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
    pub max_men: Option<f64>,
    pub squads: Option<Vec<crate::scene_scenarios::SquadData>>,
}
impl CheckSquadsS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CheckSquadsS {
            rooms: Vec::new(),
            max_men: None,
            squads: None,
        }
    }
}
impl Default for CheckSquadsS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:195 propGap(p), src/game/roomclear.ts:195)
#[derive(Clone, Copy, Debug)]
pub struct PropGapP {
    pub home: [f64; 3],
    pub hx: f64,
    pub hz: f64,
}
impl PropGapP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PropGapP {
            home: [0.0; 3],
            hx: 0.0,
            hz: 0.0,
        }
    }
}
impl Default for PropGapP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:385 team ->, src/game/roomclear.ts:385)
#[derive(Clone, Copy, Debug)]
pub struct RoomClearScenarioTeam {
    pub sam: bool,
    pub ops: f64,
}
impl RoomClearScenarioTeam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomClearScenarioTeam {
            sam: false,
            ops: 0.0,
        }
    }
}
impl Default for RoomClearScenarioTeam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/roomclear.ts:390 objective ->, src/game/roomclear.ts:390)
#[derive(Clone, Debug)]
pub struct RoomClearScenarioObjective {
    pub text: String,
    pub pos: Option<V3>,
}
impl RoomClearScenarioObjective {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomClearScenarioObjective {
            text: String::new(),
            pos: None,
        }
    }
}
impl Default for RoomClearScenarioObjective { fn default() -> Self { Self::zeroed() } }

/// (src/game/roomclear.ts:106 MARK_CLEAR_M)
pub const MARK_CLEAR_M: f64 = 0.45;

/// (src/game/roomclear.ts:212 CLEAR_SAY_SECS)
pub const CLEAR_SAY_SECS: f64 = 0.6;
