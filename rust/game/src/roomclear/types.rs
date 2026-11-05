//! The types of `roomclear`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Placement, src/game/roomclear.ts:90)
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

/// (TypeScript: RoomState, src/game/roomclear.ts:228)
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

/// (TypeScript: RoomClearScenario, src/game/roomclear.ts:230)
#[derive(Clone, Debug)]
pub struct RoomClearScenario {
    pub knobs: crate::knobs::RoomclearK,
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
            knobs: crate::knobs::RoomclearK { seed: 0.0, reroll: true, announce: true, pair_only: false, fresh: false, way: 0.0 },
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
            layout: crate::nightbreach::NightBreachLayout::zeroed(),
        }
    }
}
impl Default for RoomClearScenario { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/roomclear.ts:259 RoomClearScenario.log, src/game/roomclear.ts:259)
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

/// (TypeScript: inline at src/game/roomclear.ts:264 RoomClearScenario.obj, src/game/roomclear.ts:264)
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

/// (TypeScript: inline at src/game/roomclear.ts:94 placeDefenders(S), src/game/roomclear.ts:94)
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

/// (TypeScript: inline at src/game/roomclear.ts:113 checkStructure(S), src/game/roomclear.ts:113)
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

/// (TypeScript: inline at src/game/roomclear.ts:159 checkSquads(S), src/game/roomclear.ts:159)
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

/// (TypeScript: inline at src/game/roomclear.ts:196 propGap(p), src/game/roomclear.ts:196)
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

/// (TypeScript: inline at src/game/roomclear.ts:386 team ->, src/game/roomclear.ts:386)
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

/// (TypeScript: inline at src/game/roomclear.ts:391 objective ->, src/game/roomclear.ts:391)
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

/// (src/game/roomclear.ts:87 STRUCTURE: a table nothing writes)
pub fn structure() -> crate::scene_scenarios::StructureData { crate::scene_scenarios::StructureData { name: String::from("east wing"), team: None, rooms: alloc::vec![crate::scene_scenarios::StructureRoom { id: String::from("breakroom"), name: String::from("break room"), bounds: crate::scene_scenarios::Rect { x0: 28.12, x1: 35.88, z0: 12.26, z1: 23.88 }, doors: alloc::vec![crate::scene_scenarios::StructureDoor { id: String::from("breakroom_w"), from: String::from("cubicles") }, crate::scene_scenarios::StructureDoor { id: String::from("breakroom_n"), from: String::from("corridor") }, crate::scene_scenarios::StructureDoor { id: String::from("breakroom_ext"), from: String::from("south lot") }, crate::scene_scenarios::StructureDoor { id: String::from("fire_exit"), from: String::from("east side") }], garrison: 2.0, marks: alloc::vec![crate::scene_scenarios::StructureMark { pos: [32.3, 0.0, 17.95], watch: String::from("breakroom_w"), aim_at: [28.1, 1.1, 17.6] }, crate::scene_scenarios::StructureMark { pos: [33.6, 0.0, 15.4], watch: String::from("breakroom_n"), aim_at: [30.6, 1.1, 12.36] }, crate::scene_scenarios::StructureMark { pos: [31.9, 0.0, 14.9], watch: String::from("breakroom_w"), aim_at: [28.1, 1.1, 17.6] }, crate::scene_scenarios::StructureMark { pos: [33.4, 0.0, 21.7], watch: String::from("breakroom_w"), aim_at: [28.1, 1.1, 17.6] }, crate::scene_scenarios::StructureMark { pos: [29.4, 0.0, 21.9], watch: String::from("breakroom_n"), aim_at: [30.6, 1.1, 12.36] }, crate::scene_scenarios::StructureMark { pos: [33.9, 0.0, 19.2], watch: String::from("breakroom_ext"), aim_at: [31.6, 1.1, 23.84] }] }, crate::scene_scenarios::StructureRoom { id: String::from("storage"), name: String::from("storage"), bounds: crate::scene_scenarios::Rect { x0: 30.12, x1: 35.88, z0: 4.12, z1: 9.94 }, doors: alloc::vec![crate::scene_scenarios::StructureDoor { id: String::from("storage"), from: String::from("corridor") }], garrison: 1.0, marks: alloc::vec![crate::scene_scenarios::StructureMark { pos: [31.8, 0.0, 5.3], watch: String::from("storage"), aim_at: [32.6, 1.1, 9.84] }, crate::scene_scenarios::StructureMark { pos: [35.1, 0.0, 8.8], watch: String::from("storage"), aim_at: [32.6, 1.1, 9.5] }, crate::scene_scenarios::StructureMark { pos: [31.0, 0.0, 8.8], watch: String::from("storage"), aim_at: [32.6, 1.1, 9.5] }] }, crate::scene_scenarios::StructureRoom { id: String::from("manager"), name: String::from("manager's office"), bounds: crate::scene_scenarios::Rect { x0: 22.12, x1: 29.94, z0: 4.12, z1: 9.94 }, doors: alloc::vec![crate::scene_scenarios::StructureDoor { id: String::from("manager"), from: String::from("corridor") }, crate::scene_scenarios::StructureDoor { id: String::from("server_manager"), from: String::from("server room") }], garrison: 1.0, marks: alloc::vec![crate::scene_scenarios::StructureMark { pos: [28.2, 0.0, 8.1], watch: String::from("manager"), aim_at: [25.6, 1.1, 9.84] }, crate::scene_scenarios::StructureMark { pos: [23.4, 0.0, 6.3], watch: String::from("manager"), aim_at: [25.6, 1.1, 9.84] }, crate::scene_scenarios::StructureMark { pos: [28.6, 0.0, 6.5], watch: String::from("server_manager"), aim_at: [22.16, 1.1, 8.55] }] }, crate::scene_scenarios::StructureRoom { id: String::from("server"), name: String::from("server room"), bounds: crate::scene_scenarios::Rect { x0: 14.12, x1: 21.88, z0: 4.12, z1: 9.94 }, doors: alloc::vec![crate::scene_scenarios::StructureDoor { id: String::from("server"), from: String::from("corridor") }, crate::scene_scenarios::StructureDoor { id: String::from("server_manager"), from: String::from("manager's office") }], garrison: 1.0, marks: alloc::vec![crate::scene_scenarios::StructureMark { pos: [15.2, 0.0, 8.4], watch: String::from("server"), aim_at: [17.6, 1.1, 9.84] }, crate::scene_scenarios::StructureMark { pos: [20.6, 0.0, 7.9], watch: String::from("server"), aim_at: [17.6, 1.1, 9.84] }, crate::scene_scenarios::StructureMark { pos: [16.0, 0.0, 7.8], watch: String::from("server_manager"), aim_at: [21.84, 1.1, 8.55] }] }], starts: None, max_men: None, way_out: None, dig_in: None, wall_charges: None, squads: None } }

/// (src/game/roomclear.ts:107 MARK_CLEAR_M)
pub const MARK_CLEAR_M: f64 = 0.45;

/// (src/game/roomclear.ts:213 CLEAR_SAY_SECS)
pub const CLEAR_SAY_SECS: f64 = 0.6;
