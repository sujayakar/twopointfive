//! The types of `scene_scenarios`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a lesson step's done-test, a variant for each `is` of LESSON_TESTS (scenarios.ts LessonTest, lesson.ts). Unknown: an `is` no row has, or a record that would throw; it never passes (lesson.ts passes)
#[derive(Clone, Debug)]
pub enum LessonTestKind {
    Unknown { is: String },
    In { area: crate::scene_scenarios::Rect },
    Near { at: V3, r: Option<f64> },
    Flag { flag: String, value: Option<bool> },
    Slot { slot: String, drawn: Option<bool> },
    Pace { below: Option<f64>, above: Option<f64> },
    Meter { which: String, below: Option<f64>, above: Option<f64> },
    Stage { stage: String },
    Door { door: String, state: String },
    Target { name: String, on: bool },
    Men { down: Option<f64> },
    Did { what: String, on: Option<String>, n: Option<f64> },
    Downed { how: String, n: Option<f64> },
    Body { area: crate::scene_scenarios::Rect },
    Calm { route: Option<String> },
    Loaded,
    Reload,
    Launcher { on: String, fired: bool },
    Burst { n: Option<f64> },
    Threw { kind: Option<String>, quick: bool, area: Option<crate::scene_scenarios::Rect> },
    OrderTo { who: String },
    StackOn { door: String, who: Option<String>, set: bool },
    PairsOn { doors: Option<f64> },
    Word { doors: Option<f64> },
    Taken { room: String },
    Following,
    Mate { area: Option<crate::scene_scenarios::Rect>, at: Option<V3>, r: Option<f64>, by: Option<f64>, away: Option<f64> },
    Pair { what: String, n: Option<f64> },
    Cue { name: String },
    View { turned: Option<f64>, zoomed: Option<f64>, panned: Option<f64> },
    Squad { area: crate::scene_scenarios::Rect, n: Option<f64> },
    Stacks { doors: Vec<String>, n: Option<f64>, set: bool },
    Watching { at: V3, r: Option<f64>, n: Option<f64> },
    All { of: Vec<crate::scene_scenarios::LessonTestKind> },
    Any { of: Vec<crate::scene_scenarios::LessonTestKind> },
}

/// (TypeScript: Rect, src/scene/scenarios.ts:12)
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl Rect {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rect {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for Rect { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: TeamStart, src/scene/scenarios.ts:16)
#[derive(Clone, Debug)]
pub struct TeamStart {
    pub sam: V3,
    pub sam_yaw: f64,
    pub ops: Option<Vec<V3>>,
    pub ops_yaw: Option<f64>,
}
impl TeamStart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeamStart {
            sam: [0.0; 3],
            sam_yaw: 0.0,
            ops: None,
            ops_yaw: None,
        }
    }
}
impl Default for TeamStart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ScenarioMark, src/scene/scenarios.ts:18)
#[derive(Clone, Copy, Debug)]
pub struct ScenarioMark {
    pub pos: V3,
    pub aim_at: V3,
}
impl ScenarioMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScenarioMark {
            pos: [0.0; 3],
            aim_at: [0.0; 3],
        }
    }
}
impl Default for ScenarioMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: NightBreachData, src/scene/scenarios.ts:23)
#[derive(Clone, Debug)]
pub struct NightBreachData {
    pub team: crate::scene_scenarios::TeamStart,
    pub room: Option<String>,
    pub lights: Option<String>,
}
impl NightBreachData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NightBreachData {
            team: crate::scene_scenarios::TeamStart::zeroed(),
            room: None,
            lights: None,
        }
    }
}
impl Default for NightBreachData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StructureDoor, src/scene/scenarios.ts:27)
#[derive(Clone, Debug)]
pub struct StructureDoor {
    pub id: String,
    pub from: String,
}
impl StructureDoor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StructureDoor {
            id: String::new(),
            from: String::new(),
        }
    }
}
impl Default for StructureDoor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StructureMark, src/scene/scenarios.ts:29)
#[derive(Clone, Debug)]
pub struct StructureMark {
    pub pos: V3,
    pub watch: String,
    pub aim_at: V3,
}
impl StructureMark {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StructureMark {
            pos: [0.0; 3],
            watch: String::new(),
            aim_at: [0.0; 3],
        }
    }
}
impl Default for StructureMark { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StructureRoom, src/scene/scenarios.ts:30)
#[derive(Clone, Debug)]
pub struct StructureRoom {
    pub id: String,
    pub name: String,
    pub bounds: crate::scene_scenarios::Rect,
    pub doors: Vec<crate::scene_scenarios::StructureDoor>,
    pub garrison: f64,
    pub marks: Vec<crate::scene_scenarios::StructureMark>,
}
impl StructureRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StructureRoom {
            id: String::new(),
            name: String::new(),
            bounds: crate::scene_scenarios::Rect::zeroed(),
            doors: Vec::new(),
            garrison: 0.0,
            marks: Vec::new(),
        }
    }
}
impl Default for StructureRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StructureData, src/scene/scenarios.ts:40)
#[derive(Clone, Debug)]
pub struct StructureData {
    pub name: String,
    pub team: Option<crate::scene_scenarios::TeamStart>,
    pub rooms: Vec<crate::scene_scenarios::StructureRoom>,
    pub starts: Option<Vec<crate::scene_scenarios::TeamStart>>,
    pub max_men: Option<f64>,
    pub way_out: Option<bool>,
    pub dig_in: Option<bool>,
    pub wall_charges: Option<f64>,
    pub squads: Option<Vec<crate::scene_scenarios::SquadData>>,
}
impl StructureData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StructureData {
            name: String::new(),
            team: None,
            rooms: Vec::new(),
            starts: None,
            max_men: None,
            way_out: None,
            dig_in: None,
            wall_charges: None,
            squads: None,
        }
    }
}
impl Default for StructureData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SquadData, src/scene/scenarios.ts:70)
#[derive(Clone, Debug)]
pub struct SquadData {
    pub id: String,
    pub name: Option<String>,
    pub rooms: Option<Vec<String>>,
    pub rally: Option<V3>,
    pub barricade: Option<Vec<String>>,
    pub may_leave: Option<bool>,
    pub training: Option<crate::scene_scenarios::SquadDataTraining>,
    pub arrive: Option<crate::scene_scenarios::SquadArrive>,
}
impl SquadData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadData {
            id: String::new(),
            name: None,
            rooms: None,
            rally: None,
            barricade: None,
            may_leave: None,
            training: None,
            arrive: None,
        }
    }
}
impl Default for SquadData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SquadDataTraining {
    Trained,
    Green,
    Mixed,
}
impl SquadDataTraining {
    pub const fn as_str(self) -> &'static str {
        match self {
            SquadDataTraining::Trained => "trained",
            SquadDataTraining::Green => "green",
            SquadDataTraining::Mixed => "mixed",
        }
    }
}

/// (TypeScript: SquadArrive, src/scene/scenarios.ts:64)
#[derive(Clone, Debug)]
pub struct SquadArrive {
    pub door: String,
    pub spawn: Vec<V3>,
    pub men: f64,
}
impl SquadArrive {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SquadArrive {
            door: String::new(),
            spawn: Vec::new(),
            men: 0.0,
        }
    }
}
impl Default for SquadArrive { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RouteOpening, src/scene/scenarios.ts:99)
#[derive(Clone, Debug)]
pub struct RouteOpening {
    pub id: String,
    pub name: String,
    pub short: String,
    pub region: String,
    pub kind: crate::scene_scenarios::RouteOpeningKind,
    pub door: Option<String>,
    pub at: V3,
    pub mark: V3,
    pub watch: V3,
    pub half: f64,
    pub north: bool,
    pub call: String,
    pub side: crate::scene_scenarios::RouteOpeningSide,
    pub look: f64,
}
impl RouteOpening {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RouteOpening {
            id: String::new(),
            name: String::new(),
            short: String::new(),
            region: String::new(),
            kind: crate::scene_scenarios::RouteOpeningKind::Door,
            door: None,
            at: [0.0; 3],
            mark: [0.0; 3],
            watch: [0.0; 3],
            half: 0.0,
            north: false,
            call: String::new(),
            side: crate::scene_scenarios::RouteOpeningSide::Left,
            look: 0.0,
        }
    }
}
impl Default for RouteOpening { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RouteOpeningKind {
    Door,
    Glass,
    Doorway,
}
impl RouteOpeningKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RouteOpeningKind::Door => "door",
            RouteOpeningKind::Glass => "glass",
            RouteOpeningKind::Doorway => "doorway",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RouteOpeningSide {
    Left,
    Right,
}
impl RouteOpeningSide {
    pub const fn as_str(self) -> &'static str {
        match self {
            RouteOpeningSide::Left => "left",
            RouteOpeningSide::Right => "right",
        }
    }
}

/// (TypeScript: RescueFlowData, src/scene/scenarios.ts:109)
#[derive(Clone, Debug)]
pub struct RescueFlowData {
    pub corridor: crate::scene_scenarios::Rect,
    pub regions: Vec<crate::scene_scenarios::RescueFlowDataRegionsItem>,
    pub route: Vec<crate::scene_scenarios::RouteOpening>,
    pub hold: crate::scene_scenarios::RescueFlowDataHold,
}
impl RescueFlowData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueFlowData {
            corridor: crate::scene_scenarios::Rect::zeroed(),
            regions: Vec::new(),
            route: Vec::new(),
            hold: crate::scene_scenarios::RescueFlowDataHold::zeroed(),
        }
    }
}
impl Default for RescueFlowData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:112 RescueFlowData.regions, src/scene/scenarios.ts:112)
#[derive(Clone, Debug)]
pub struct RescueFlowDataRegionsItem {
    pub id: String,
    pub door: Option<String>,
    pub rect: crate::scene_scenarios::Rect,
}
impl RescueFlowDataRegionsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueFlowDataRegionsItem {
            id: String::new(),
            door: None,
            rect: crate::scene_scenarios::Rect::zeroed(),
        }
    }
}
impl Default for RescueFlowDataRegionsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:115 RescueFlowData.hold, src/scene/scenarios.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct RescueFlowDataHold {
    pub ahead: f64,
    pub x_max: f64,
    pub z: f64,
}
impl RescueFlowDataHold {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueFlowDataHold {
            ahead: 0.0,
            x_max: 0.0,
            z: 0.0,
        }
    }
}
impl Default for RescueFlowDataHold { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RescueData, src/scene/scenarios.ts:117)
#[derive(Clone, Debug)]
pub struct RescueData {
    pub team: crate::scene_scenarios::TeamStart,
    pub room: crate::scene_scenarios::RescueDataRoom,
    pub side: Option<crate::scene_scenarios::RescueDataSide>,
    pub hostages: Vec<crate::scene_scenarios::RescueDataHostagesItem>,
    pub captor: crate::scene_scenarios::ScenarioMark,
    pub door_man: crate::scene_scenarios::RescueDataDoorMan,
    pub roamers: Vec<crate::scene_scenarios::ScenarioMark>,
    pub guards: Option<Vec<crate::scene_scenarios::ScenarioMark>>,
    pub cast: Option<crate::scene_scenarios::RescueDataCast>,
    pub flow: Option<crate::scene_scenarios::RescueFlowData>,
}
impl RescueData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueData {
            team: crate::scene_scenarios::TeamStart::zeroed(),
            room: crate::scene_scenarios::RescueDataRoom::zeroed(),
            side: None,
            hostages: Vec::new(),
            captor: crate::scene_scenarios::ScenarioMark::zeroed(),
            door_man: crate::scene_scenarios::RescueDataDoorMan::zeroed(),
            roamers: Vec::new(),
            guards: None,
            cast: None,
            flow: None,
        }
    }
}
impl Default for RescueData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:120 RescueData.room, src/scene/scenarios.ts:120)
#[derive(Clone, Debug)]
pub struct RescueDataRoom {
    pub name: String,
    pub where_: Option<String>,
    pub door: String,
    pub bounds: Option<crate::scene_scenarios::Rect>,
    pub lights: Option<String>,
    pub door_aim: V3,
    pub outside: Option<V3>,
    pub hold: Option<V3>,
}
impl RescueDataRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueDataRoom {
            name: String::new(),
            where_: None,
            door: String::new(),
            bounds: None,
            lights: None,
            door_aim: [0.0; 3],
            outside: None,
            hold: None,
        }
    }
}
impl Default for RescueDataRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:136 RescueData.side, src/scene/scenarios.ts:136)
#[derive(Clone, Debug)]
pub struct RescueDataSide {
    pub door: String,
    pub hold: V3,
    pub from: Option<crate::scene_scenarios::RescueDataSideFrom>,
    pub pair: Option<bool>,
    pub stack: Option<Vec<V3>>,
    pub standoff: Option<Vec<V3>>,
}
impl RescueDataSide {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueDataSide {
            door: String::new(),
            hold: [0.0; 3],
            from: None,
            pair: None,
            stack: None,
            standoff: None,
        }
    }
}
impl Default for RescueDataSide { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:141 RescueDataSide.from, src/scene/scenarios.ts:141)
#[derive(Clone, Debug)]
pub struct RescueDataSideFrom {
    pub door: String,
    pub bounds: Option<crate::scene_scenarios::Rect>,
}
impl RescueDataSideFrom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueDataSideFrom {
            door: String::new(),
            bounds: None,
        }
    }
}
impl Default for RescueDataSideFrom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:148 RescueData.hostages, src/scene/scenarios.ts:148)
#[derive(Clone, Copy, Debug)]
pub struct RescueDataHostagesItem {
    pub pos: V3,
    pub face: f64,
}
impl RescueDataHostagesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueDataHostagesItem {
            pos: [0.0; 3],
            face: 0.0,
        }
    }
}
impl Default for RescueDataHostagesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:151 RescueData.doorMan, src/scene/scenarios.ts:151)
#[derive(Clone, Copy, Debug)]
pub struct RescueDataDoorMan {
    pub inside: crate::scene_scenarios::ScenarioMark,
    pub outside: crate::scene_scenarios::ScenarioMark,
}
impl RescueDataDoorMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueDataDoorMan {
            inside: crate::scene_scenarios::ScenarioMark::zeroed(),
            outside: crate::scene_scenarios::ScenarioMark::zeroed(),
        }
    }
}
impl Default for RescueDataDoorMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:158 RescueData.cast, src/scene/scenarios.ts:158)
#[derive(Clone, Copy, Debug)]
pub struct RescueDataCast {
    pub roamers: f64,
}
impl RescueDataCast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RescueDataCast {
            roamers: 0.0,
        }
    }
}
impl Default for RescueDataCast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PlaceArea, src/scene/scenarios.ts:167)
#[derive(Clone, Debug)]
pub struct PlaceArea {
    pub name: String,
    pub bounds: crate::scene_scenarios::Rect,
}
impl PlaceArea {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlaceArea {
            name: String::new(),
            bounds: crate::scene_scenarios::Rect::zeroed(),
        }
    }
}
impl Default for PlaceArea { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PlacesData, src/scene/scenarios.ts:168)
#[derive(Clone, Debug)]
pub struct PlacesData {
    pub areas: Vec<crate::scene_scenarios::PlaceArea>,
}
impl PlacesData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlacesData {
            areas: Vec::new(),
        }
    }
}
impl Default for PlacesData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietStart, src/scene/scenarios.ts:174)
#[derive(Clone, Debug)]
pub struct QuietStart {
    pub sam: V3,
    pub sam_yaw: f64,
    pub mate: Option<V3>,
    pub name: Option<String>,
}
impl QuietStart {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietStart {
            sam: [0.0; 3],
            sam_yaw: 0.0,
            mate: None,
            name: None,
        }
    }
}
impl Default for QuietStart { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietSpot, src/scene/scenarios.ts:181)
#[derive(Clone, Debug)]
pub struct QuietSpot {
    pub place: String,
    pub area: crate::scene_scenarios::Rect,
    pub door: String,
    pub holder: String,
    pub rack_index: f64,
    pub front: V3,
    pub half_w: f64,
    pub wired: Option<bool>,
}
impl QuietSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietSpot {
            place: String::new(),
            area: crate::scene_scenarios::Rect::zeroed(),
            door: String::new(),
            holder: String::new(),
            rack_index: 0.0,
            front: [0.0; 3],
            half_w: 0.0,
            wired: None,
        }
    }
}
impl Default for QuietSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietExit, src/scene/scenarios.ts:197)
#[derive(Clone, Debug)]
pub struct QuietExit {
    pub name: String,
    pub door: String,
    pub x: f64,
    pub z: f64,
    pub out: [f64; 2],
    pub half_w: Option<f64>,
}
impl QuietExit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietExit {
            name: String::new(),
            door: String::new(),
            x: 0.0,
            z: 0.0,
            out: [0.0; 2],
            half_w: None,
        }
    }
}
impl Default for QuietExit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietStand, src/scene/scenarios.ts:199)
#[derive(Clone, Debug)]
pub struct QuietStand {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub loop_: Option<Vec<V3>>,
}
impl QuietStand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietStand {
            name: String::new(),
            pos: [0.0; 3],
            face: 0.0,
            loop_: None,
        }
    }
}
impl Default for QuietStand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: QuietData, src/scene/scenarios.ts:200)
#[derive(Clone, Debug)]
pub struct QuietData {
    pub thing: Option<String>,
    pub starts: Vec<crate::scene_scenarios::QuietStart>,
    pub spots: Vec<crate::scene_scenarios::QuietSpot>,
    pub exits: Vec<crate::scene_scenarios::QuietExit>,
    pub patrols: Option<Vec<Vec<String>>>,
    pub needs: Option<crate::scene_scenarios::QuietDataNeeds>,
    pub phones: Option<Vec<crate::scene_scenarios::QuietStand>>,
    pub wayouts: Option<Vec<crate::scene_scenarios::QuietStand>>,
    pub radio_check: Option<bool>,
}
impl QuietData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietData {
            thing: None,
            starts: Vec::new(),
            spots: Vec::new(),
            exits: Vec::new(),
            patrols: None,
            needs: None,
            phones: None,
            wayouts: None,
            radio_check: None,
        }
    }
}
impl Default for QuietData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:210 QuietData.needs, src/scene/scenarios.ts:210)
#[derive(Clone, Debug)]
pub struct QuietDataNeeds {
    pub coffee: Option<crate::scene_scenarios::QuietStand>,
    pub smoke: Option<crate::scene_scenarios::QuietStand>,
    pub water: Option<crate::scene_scenarios::QuietStand>,
    pub stretch: Option<crate::scene_scenarios::QuietStand>,
}
impl QuietDataNeeds {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QuietDataNeeds {
            coffee: None,
            smoke: None,
            water: None,
            stretch: None,
        }
    }
}
impl Default for QuietDataNeeds { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LessonStep, src/scene/scenarios.ts:226)
#[derive(Clone, Debug)]
pub struct LessonStep {
    pub id: String,
    pub line: String,
    pub setup_line: Option<String>,
    pub ring_line: Option<String>,
    pub blow_line: Option<String>,
    pub menu_line: Option<String>,
    pub page_line: Option<String>,
    pub flat_line: Option<String>,
    pub done: crate::scene_scenarios::LessonTestKind,
    pub hold: Option<f64>,
    pub settle: Option<f64>,
    pub at: Option<V3>,
    pub at_man: Option<String>,
    pub allow: Option<Vec<String>>,
    pub deny: Option<Vec<String>>,
    pub cue: Option<String>,
    pub checkpoint: Option<String>,
    pub fog: Option<bool>,
    pub no_tap: Option<String>,
    pub eye: Option<crate::scene_scenarios::LessonStepEye>,
    pub explain: Option<crate::scene_scenarios::LessonStepExplain>,
    pub toward: Option<V3>,
    pub on_way: Option<crate::scene_scenarios::LessonStepOnWay>,
    pub after: Option<crate::scene_scenarios::LessonStepAfter>,
}
impl LessonStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonStep {
            id: String::new(),
            line: String::new(),
            setup_line: None,
            ring_line: None,
            blow_line: None,
            menu_line: None,
            page_line: None,
            flat_line: None,
            done: crate::scene_scenarios::LessonTestKind::Unknown { is: String::new() },
            hold: None,
            settle: None,
            at: None,
            at_man: None,
            allow: None,
            deny: None,
            cue: None,
            checkpoint: None,
            fog: None,
            no_tap: None,
            eye: None,
            explain: None,
            toward: None,
            on_way: None,
            after: None,
        }
    }
}
impl Default for LessonStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:270 LessonStep.eye, src/scene/scenarios.ts:270)
#[derive(Clone, Copy, Debug)]
pub struct LessonStepEye {
    pub at: V3,
    pub yaw: f64,
    pub fov: Option<f64>,
}
impl LessonStepEye {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonStepEye {
            at: [0.0; 3],
            yaw: 0.0,
            fov: None,
        }
    }
}
impl Default for LessonStepEye { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:276 LessonStep.explain, src/scene/scenarios.ts:276)
#[derive(Clone, Debug)]
pub struct LessonStepExplain {
    pub title: Option<String>,
    pub text: String,
    pub quiet: Option<bool>,
    pub now: Option<bool>,
}
impl LessonStepExplain {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonStepExplain {
            title: None,
            text: String::new(),
            quiet: None,
            now: None,
        }
    }
}
impl Default for LessonStepExplain { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:282 LessonStep.onWay, src/scene/scenarios.ts:282)
#[derive(Clone, Copy, Debug)]
pub struct LessonStepOnWay {
    pub nv: Option<bool>,
    pub run: Option<bool>,
}
impl LessonStepOnWay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonStepOnWay {
            nv: None,
            run: None,
        }
    }
}
impl Default for LessonStepOnWay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:285 LessonStep.after, src/scene/scenarios.ts:285)
#[derive(Clone, Debug)]
pub struct LessonStepAfter {
    pub line: Option<String>,
    pub on: Option<Vec<String>>,
    pub word: Option<String>,
}
impl LessonStepAfter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonStepAfter {
            line: None,
            on: None,
            word: None,
        }
    }
}
impl Default for LessonStepAfter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RailsData, src/scene/scenarios.ts:288)
#[derive(Clone, Debug)]
pub struct RailsData {
    pub allow: Vec<String>,
    pub respawn: Option<f64>,
    pub fog: Option<bool>,
    pub waits: Option<crate::scene_scenarios::RailsDataWaits>,
    pub sky: Option<crate::scene_scenarios::RailsDataSky>,
    pub lasers: Option<bool>,
    pub orders: Option<Vec<String>>,
    pub flat_ring: Option<bool>,
    pub rows: Option<Vec<String>>,
}
impl RailsData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsData {
            allow: Vec::new(),
            respawn: None,
            fog: None,
            waits: None,
            sky: None,
            lasers: None,
            orders: None,
            flat_ring: None,
            rows: None,
        }
    }
}
impl Default for RailsData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:298 RailsData.waits, src/scene/scenarios.ts:298)
#[derive(Clone, Copy, Debug)]
pub struct RailsDataWaits {
    pub low: Option<bool>,
}
impl RailsDataWaits {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsDataWaits {
            low: None,
        }
    }
}
impl Default for RailsDataWaits { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:303 RailsData.sky, src/scene/scenarios.ts:303)
#[derive(Clone, Copy, Debug)]
pub struct RailsDataSky {
    pub park: V3,
    pub reach: Option<f64>,
    pub eye: Option<crate::scene_scenarios::RailsDataSkyEye>,
    pub low: Option<bool>,
}
impl RailsDataSky {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsDataSky {
            park: [0.0; 3],
            reach: None,
            eye: None,
            low: None,
        }
    }
}
impl Default for RailsDataSky { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:303 RailsDataSky.eye, src/scene/scenarios.ts:303)
#[derive(Clone, Copy, Debug)]
pub struct RailsDataSkyEye {
    pub at: V3,
    pub yaw: f64,
    pub fov: Option<f64>,
}
impl RailsDataSkyEye {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RailsDataSkyEye {
            at: [0.0; 3],
            yaw: 0.0,
            fov: None,
        }
    }
}
impl Default for RailsDataSkyEye { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LessonData, src/scene/scenarios.ts:319)
#[derive(Clone, Debug)]
pub struct LessonData {
    pub title: String,
    pub steps: Vec<crate::scene_scenarios::LessonStep>,
    pub rails: Option<crate::scene_scenarios::RailsData>,
}
impl LessonData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LessonData {
            title: String::new(),
            steps: Vec::new(),
            rails: None,
        }
    }
}
impl Default for LessonData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ParData, src/scene/scenarios.ts:330)
#[derive(Clone, Copy, Debug)]
pub struct ParData {
    pub gold: f64,
    pub silver: f64,
    pub bronze: f64,
    pub loss_secs: f64,
}
impl ParData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParData {
            gold: 0.0,
            silver: 0.0,
            bronze: 0.0,
            loss_secs: 0.0,
        }
    }
}
impl Default for ParData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: IssueCard, src/scene/scenarios.ts:335)
#[derive(Clone, Copy, Debug)]
pub struct IssueCard {
    pub weapon: Option<crate::kit::KitWeapon>,
    pub suppressed: Option<bool>,
    pub light: Option<bool>,
    pub nvg: Option<bool>,
    pub bangs: Option<f64>,
    pub charges: Option<f64>,
    pub smoke: Option<f64>,
    pub frags: Option<f64>,
    pub walls: Option<f64>,
    pub shells: Option<f64>,
    pub rings: Option<f64>,
    pub loaded: Option<crate::scene_scenarios::IssueCardLoaded>,
}
impl IssueCard {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IssueCard {
            weapon: None,
            suppressed: None,
            light: None,
            nvg: None,
            bangs: None,
            charges: None,
            smoke: None,
            frags: None,
            walls: None,
            shells: None,
            rings: None,
            loaded: None,
        }
    }
}
impl Default for IssueCard { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IssueCardLoaded {
    Ring,
    He40,
}
impl IssueCardLoaded {
    pub const fn as_str(self) -> &'static str {
        match self {
            IssueCardLoaded::Ring => "ring",
            IssueCardLoaded::He40 => "he40",
        }
    }
}

/// (TypeScript: IssueData, src/scene/scenarios.ts:342)
#[derive(Clone, Debug)]
pub struct IssueData {
    pub members: JsMap<String, crate::scene_scenarios::IssueCard>,
}
impl IssueData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IssueData {
            members: JsMap::new(),
        }
    }
}
impl Default for IssueData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SnatchData, src/scene/scenarios.ts:346)
#[derive(Clone, Debug)]
pub struct SnatchData {
    pub man: crate::scene_scenarios::SnatchDataMan,
    pub exits: Vec<crate::scene_scenarios::SnatchDataExitsItem>,
    pub van: Option<crate::scene_scenarios::SnatchDataVan>,
}
impl SnatchData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchData {
            man: crate::scene_scenarios::SnatchDataMan::zeroed(),
            exits: Vec::new(),
            van: None,
        }
    }
}
impl Default for SnatchData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:348 SnatchData.man, src/scene/scenarios.ts:348)
#[derive(Clone, Debug)]
pub struct SnatchDataMan {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub tint: V3,
    pub tint2: Option<V3>,
    pub points: Option<Vec<V3>>,
    pub wait: Option<Vec<f64>>,
}
impl SnatchDataMan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchDataMan {
            name: String::new(),
            pos: [0.0; 3],
            face: 0.0,
            tint: [0.0; 3],
            tint2: None,
            points: None,
            wait: None,
        }
    }
}
impl Default for SnatchDataMan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:350 SnatchData.exits, src/scene/scenarios.ts:350)
#[derive(Clone, Debug)]
pub struct SnatchDataExitsItem {
    pub name: String,
    pub pos: V3,
}
impl SnatchDataExitsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchDataExitsItem {
            name: String::new(),
            pos: [0.0; 3],
        }
    }
}
impl Default for SnatchDataExitsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:354 SnatchData.van, src/scene/scenarios.ts:354)
#[derive(Clone, Debug)]
pub struct SnatchDataVan {
    pub name: String,
    pub pos: V3,
}
impl SnatchDataVan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SnatchDataVan {
            name: String::new(),
            pos: [0.0; 3],
        }
    }
}
impl Default for SnatchDataVan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DefuseDevice, src/scene/scenarios.ts:360)
#[derive(Clone, Debug)]
pub struct DefuseDevice {
    pub id: String,
    pub name: String,
    pub pos: V3,
    pub n: [f64; 2],
}
impl DefuseDevice {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseDevice {
            id: String::new(),
            name: String::new(),
            pos: [0.0; 3],
            n: [0.0; 2],
        }
    }
}
impl Default for DefuseDevice { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DefuseData, src/scene/scenarios.ts:369)
#[derive(Clone, Debug)]
pub struct DefuseData {
    pub secs: f64,
    pub work_secs: Option<f64>,
    pub devices: Vec<crate::scene_scenarios::DefuseDevice>,
    pub blast: Option<crate::scene_scenarios::DefuseDataBlast>,
}
impl DefuseData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseData {
            secs: 0.0,
            work_secs: None,
            devices: Vec::new(),
            blast: None,
        }
    }
}
impl Default for DefuseData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:376 DefuseData.blast, src/scene/scenarios.ts:376)
#[derive(Clone, Copy, Debug)]
pub struct DefuseDataBlast {
    pub lethal_m: Option<f64>,
    pub wound_m: Option<f64>,
    pub radius: Option<f64>,
    pub power: Option<f64>,
}
impl DefuseDataBlast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DefuseDataBlast {
            lethal_m: None,
            wound_m: None,
            radius: None,
            power: None,
        }
    }
}
impl Default for DefuseDataBlast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ResponseWay, src/scene/scenarios.ts:486)
#[derive(Clone, Debug)]
pub struct ResponseWay {
    pub name: String,
    pub door: Option<String>,
    pub muster: Vec<V3>,
    pub entry: Vec<V3>,
    pub entry_face: Vec<f64>,
    pub coming: Option<String>,
    pub clear: Option<String>,
    pub stairs: Option<bool>,
}
impl ResponseWay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseWay {
            name: String::new(),
            door: None,
            muster: Vec::new(),
            entry: Vec::new(),
            entry_face: Vec::new(),
            coming: None,
            clear: None,
            stairs: None,
        }
    }
}
impl Default for ResponseWay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ResponsePost, src/scene/scenarios.ts:503)
#[derive(Clone, Debug)]
pub struct ResponsePost {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub sweep: [V3; 2],
}
impl ResponsePost {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponsePost {
            name: String::new(),
            pos: [0.0; 3],
            yaw: 0.0,
            sweep: [[0.0; 3]; 2],
        }
    }
}
impl Default for ResponsePost { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ResponseData, src/scene/scenarios.ts:504)
#[derive(Clone, Debug)]
pub struct ResponseData {
    pub why: String,
    pub site: String,
    pub at: String,
    pub team: String,
    pub tag: Option<String>,
    pub ways: Vec<crate::scene_scenarios::ResponseWay>,
    pub men: Option<[f64; 2]>,
    pub eta: Option<[f64; 2]>,
    pub posts: Option<Vec<crate::scene_scenarios::ResponsePost>>,
    pub outside: Option<Vec<V3>>,
    pub sweep: Option<Vec<crate::scene_scenarios::ResponseDataSweepItem>>,
    pub raid: Option<crate::scene_scenarios::ResponseDataRaid>,
}
impl ResponseData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseData {
            why: String::new(),
            site: String::new(),
            at: String::new(),
            team: String::new(),
            tag: None,
            ways: Vec::new(),
            men: None,
            eta: None,
            posts: None,
            outside: None,
            sweep: None,
            raid: None,
        }
    }
}
impl Default for ResponseData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:522 ResponseData.sweep, src/scene/scenarios.ts:522)
#[derive(Clone, Debug)]
pub struct ResponseDataSweepItem {
    pub name: String,
    pub pos: V3,
}
impl ResponseDataSweepItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseDataSweepItem {
            name: String::new(),
            pos: [0.0; 3],
        }
    }
}
impl Default for ResponseDataSweepItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:527 ResponseData.raid, src/scene/scenarios.ts:527)
#[derive(Clone, Debug)]
pub struct ResponseDataRaid {
    pub room: Option<String>,
}
impl ResponseDataRaid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseDataRaid {
            room: None,
        }
    }
}
impl Default for ResponseDataRaid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LevelScenarios, src/scene/scenarios.ts:529)
#[derive(Clone, Debug)]
pub struct LevelScenarios {
    pub night_breach: Option<crate::scene_scenarios::NightBreachData>,
    pub room_clear: Option<crate::scene_scenarios::StructureData>,
    pub rescue: Option<crate::scene_scenarios::RescueData>,
    pub places: Option<crate::scene_scenarios::PlacesData>,
    pub quiet: Option<crate::scene_scenarios::QuietData>,
    pub lesson: Option<crate::scene_scenarios::LessonData>,
    pub par: Option<crate::scene_scenarios::ParData>,
    pub snatch: Option<crate::scene_scenarios::SnatchData>,
    pub defuse: Option<crate::scene_scenarios::DefuseData>,
    pub issue: Option<crate::scene_scenarios::IssueData>,
    pub kit_view: Option<crate::scene_scenarios::KitViewData>,
    pub response: Option<crate::scene_scenarios::ResponseData>,
    pub door_names: Option<Vec<crate::scene_scenarios::LevelScenariosDoorNamesItem>>,
    pub walked: Option<Vec<crate::scene_ways::WayHint>>,
}
impl LevelScenarios {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelScenarios {
            night_breach: None,
            room_clear: None,
            rescue: None,
            places: None,
            quiet: None,
            lesson: None,
            par: None,
            snatch: None,
            defuse: None,
            issue: None,
            kit_view: None,
            response: None,
            door_names: None,
            walked: None,
        }
    }
}
impl Default for LevelScenarios { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KitViewData, src/scene/scenarios.ts:480)
#[derive(Clone, Debug)]
pub struct KitViewData {
    pub ways: Vec<crate::scene_scenarios::KitViewWay>,
    pub sets: Option<JsMap<String, crate::scene_scenarios::KitViewSet>>,
}
impl KitViewData {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewData {
            ways: Vec::new(),
            sets: None,
        }
    }
}
impl Default for KitViewData { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KitViewWay, src/scene/scenarios.ts:414)
#[derive(Clone, Debug)]
pub struct KitViewWay {
    pub door: Option<String>,
    pub side: Option<f64>,
    pub pin: Option<crate::scene_scenarios::KitViewWayPin>,
    pub slots: Option<Vec<Option<[f64; 3]>>>,
    pub light: Option<crate::scene_scenarios::KitViewLightOver>,
    pub hide: Option<Vec<String>>,
    pub hide_paint: Option<Vec<String>>,
    pub hide_more: Option<Vec<String>>,
    pub room_lights: Option<f64>,
    pub span: Option<[[f64; 2]; 2]>,
    pub kind: Option<crate::scene_scenarios::KitViewWayKind>,
    pub set: Option<String>,
    pub ref_: Option<[f64; 2]>,
    pub hand: Option<crate::scene_scenarios::KitViewWayHand>,
    pub slot_gap: Option<f64>,
    pub wall: Option<crate::scene_scenarios::KitViewWayWall>,
    pub lane: Option<f64>,
    pub out: Option<f64>,
    pub inside: Option<f64>,
    pub pane: Option<crate::scene_scenarios::KitViewWayPane>,
    pub show: Option<Vec<String>>,
    pub indoors: Option<bool>,
    pub keep_lights: Option<Vec<String>>,
    pub day_lights: Option<Vec<String>>,
    pub lit_inside: Option<bool>,
    pub add: Option<crate::scene_scenarios::KitViewWayAdd>,
}
impl KitViewWay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewWay {
            door: None,
            side: None,
            pin: None,
            slots: None,
            light: None,
            hide: None,
            hide_paint: None,
            hide_more: None,
            room_lights: None,
            span: None,
            kind: None,
            set: None,
            ref_: None,
            hand: None,
            slot_gap: None,
            wall: None,
            lane: None,
            out: None,
            inside: None,
            pane: None,
            show: None,
            indoors: None,
            keep_lights: None,
            day_lights: None,
            lit_inside: None,
            add: None,
        }
    }
}
impl Default for KitViewWay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:420 KitViewWay.pin, src/scene/scenarios.ts:420)
#[derive(Clone, Copy, Debug)]
pub struct KitViewWayPin {
    pub yaw: f64,
    pub pitch: f64,
    pub dist: Option<f64>,
}
impl KitViewWayPin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewWayPin {
            yaw: 0.0,
            pitch: 0.0,
            dist: None,
        }
    }
}
impl Default for KitViewWayPin { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KitViewLightOver, src/scene/scenarios.ts:413)
#[derive(Clone, Copy, Debug)]
pub struct KitViewLightOver {
    pub porch: Option<crate::scene_scenarios::KitViewLightOverPorch>,
    pub fill: Option<crate::scene_scenarios::KitViewLightOverFill>,
    pub room_lights: Option<f64>,
    pub moon_scale: Option<f64>,
    pub sky_scale: Option<f64>,
    pub sam: Option<crate::scene_scenarios::KitViewLightOverSam>,
}
impl KitViewLightOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewLightOver {
            porch: None,
            fill: None,
            room_lights: None,
            moon_scale: None,
            sky_scale: None,
            sam: None,
        }
    }
}
impl Default for KitViewLightOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:413 KitViewLightOver.porch, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct KitViewLightOverPorch {
    pub kind: Option<f64>,
    pub offset: Option<V3>,
    pub aim: Option<V3>,
    pub color: Option<V3>,
    pub intensity: Option<f64>,
    pub range: Option<f64>,
    pub inner_deg: Option<f64>,
    pub outer_deg: Option<f64>,
    pub radius: Option<f64>,
    pub volumetric: Option<f64>,
    pub broad: Option<bool>,
    pub fixture: Option<crate::scene_scenarios::KitViewLightOverPorchFixture>,
    pub day: Option<f64>,
}
impl KitViewLightOverPorch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewLightOverPorch {
            kind: None,
            offset: None,
            aim: None,
            color: None,
            intensity: None,
            range: None,
            inner_deg: None,
            outer_deg: None,
            radius: None,
            volumetric: None,
            broad: None,
            fixture: None,
            day: None,
        }
    }
}
impl Default for KitViewLightOverPorch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:399 KitViewLightOverPorch.fixture, src/scene/scenarios.ts:399)
#[derive(Clone, Copy, Debug)]
pub struct KitViewLightOverPorchFixture {
    pub at: V3,
    pub half: V3,
    pub glow: f64,
}
impl KitViewLightOverPorchFixture {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewLightOverPorchFixture {
            at: [0.0; 3],
            half: [0.0; 3],
            glow: 0.0,
        }
    }
}
impl Default for KitViewLightOverPorchFixture { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:413 KitViewLightOver.fill, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct KitViewLightOverFill {
    pub kind: Option<f64>,
    pub offset: Option<V3>,
    pub aim: Option<V3>,
    pub color: Option<V3>,
    pub intensity: Option<f64>,
    pub range: Option<f64>,
    pub inner_deg: Option<f64>,
    pub outer_deg: Option<f64>,
    pub radius: Option<f64>,
    pub volumetric: Option<f64>,
    pub broad: Option<bool>,
}
impl KitViewLightOverFill {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewLightOverFill {
            kind: None,
            offset: None,
            aim: None,
            color: None,
            intensity: None,
            range: None,
            inner_deg: None,
            outer_deg: None,
            radius: None,
            volumetric: None,
            broad: None,
        }
    }
}
impl Default for KitViewLightOverFill { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:413 KitViewLightOver.sam, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct KitViewLightOverSam {
    pub on: Option<bool>,
    pub intensity: Option<f64>,
    pub inner_deg: Option<f64>,
    pub outer_deg: Option<f64>,
    pub volumetric: Option<f64>,
    pub aim: Option<V3>,
}
impl KitViewLightOverSam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewLightOverSam {
            on: None,
            intensity: None,
            inner_deg: None,
            outer_deg: None,
            volumetric: None,
            aim: None,
        }
    }
}
impl Default for KitViewLightOverSam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitViewWayKind {
    Door,
    Wall,
    Gap,
}
impl KitViewWayKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitViewWayKind::Door => "door",
            KitViewWayKind::Wall => "wall",
            KitViewWayKind::Gap => "gap",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitViewWayHand {
    Left,
    Right,
}
impl KitViewWayHand {
    pub const fn as_str(self) -> &'static str {
        match self {
            KitViewWayHand::Left => "left",
            KitViewWayHand::Right => "right",
        }
    }
}

/// (TypeScript: inline at src/scene/scenarios.ts:448 KitViewWay.wall, src/scene/scenarios.ts:448)
#[derive(Clone, Copy, Debug)]
pub struct KitViewWayWall {
    pub face: Option<f64>,
    pub thickness: Option<f64>,
    pub top: Option<f64>,
}
impl KitViewWayWall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewWayWall {
            face: None,
            thickness: None,
            top: None,
        }
    }
}
impl Default for KitViewWayWall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum KitViewWayPane {
    Bool(bool),
    Arm1(crate::scene_scenarios::KitViewWayPaneArm1),
}

/// (TypeScript: inline at src/scene/scenarios.ts:450 KitViewWay.pane, src/scene/scenarios.ts:450)
#[derive(Clone, Debug)]
pub struct KitViewWayPaneArm1 {
    pub box_: String,
    pub stand: Option<[f64; 3]>,
}
impl KitViewWayPaneArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewWayPaneArm1 {
            box_: String::new(),
            stand: None,
        }
    }
}
impl Default for KitViewWayPaneArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:472 KitViewWay.add, src/scene/scenarios.ts:472)
#[derive(Clone, Debug)]
pub struct KitViewWayAdd {
    pub boxes: Option<Vec<crate::scene_level_data::BoxData>>,
    pub lamps: Option<Vec<crate::scene_level_data::LightData>>,
    pub laid: Option<bool>,
}
impl KitViewWayAdd {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewWayAdd {
            boxes: None,
            lamps: None,
            laid: None,
        }
    }
}
impl Default for KitViewWayAdd { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KitViewSet, src/scene/scenarios.ts:479)
#[derive(Clone, Debug)]
pub struct KitViewSet {
    pub boxes: Vec<crate::scene_level_data::BoxData>,
    pub doors: Option<Vec<crate::scene_level_data::DoorData>>,
    pub lights: Option<Vec<crate::scene_level_data::LightData>>,
    pub ref_: [f64; 2],
}
impl KitViewSet {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitViewSet {
            boxes: Vec::new(),
            doors: None,
            lights: None,
            ref_: [0.0; 2],
        }
    }
}
impl Default for KitViewSet { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/scenarios.ts:535 LevelScenarios.doorNames, src/scene/scenarios.ts:535)
#[derive(Clone, Debug)]
pub struct LevelScenariosDoorNamesItem {
    pub door: String,
    pub name: String,
}
impl LevelScenariosDoorNamesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LevelScenariosDoorNamesItem {
            door: String::new(),
            name: String::new(),
        }
    }
}
impl Default for LevelScenariosDoorNamesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ScenarioEntry, src/scene/scenarios.ts:561)
#[derive(Clone, Debug)]
pub struct ScenarioEntry {
    pub id: String,
    pub title: String,
    pub kind: crate::scene_scenarios::ScenarioEntryKind,
    pub squad: f64,
    pub group: Option<crate::scene_scenarios::MissionGroup>,
    pub only: Option<String>,
    pub packs: Option<crate::scene_scenarios::ScenarioEntryPacks>,
    pub level: Option<String>,
    pub hour: Option<crate::hours::HourId>,
    pub weather: Option<crate::scene_scenarios::ScenarioEntryWeather>,
    pub seed: Option<f64>,
    pub blurb: Option<String>,
    pub scene: Option<String>,
    pub teaches: Option<Vec<String>>,
    pub untuned: Option<bool>,
    pub off_list: Option<bool>,
    pub place: Option<String>,
    pub lean: Option<crate::ui_missions::Lean>,
    pub defenders: Option<f64>,
    pub ways: Option<Vec<String>>,
    pub squads: Option<bool>,
}
impl ScenarioEntry {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScenarioEntry {
            id: String::new(),
            title: String::new(),
            kind: crate::scene_scenarios::ScenarioEntryKind::Nightbreach,
            squad: 0.0,
            group: None,
            only: None,
            packs: None,
            level: None,
            hour: None,
            weather: None,
            seed: None,
            blurb: None,
            scene: None,
            teaches: None,
            untuned: None,
            off_list: None,
            place: None,
            lean: None,
            defenders: None,
            ways: None,
            squads: None,
        }
    }
}
impl Default for ScenarioEntry { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ScenarioEntryKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl ScenarioEntryKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ScenarioEntryKind::Nightbreach => "nightbreach",
            ScenarioEntryKind::Roomclear => "roomclear",
            ScenarioEntryKind::Rescue => "rescue",
            ScenarioEntryKind::Quiet => "quiet",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MissionGroup {
    Clock,
    Tutorial,
    Raids,
    Strongholds,
    QuietAlone,
    QuietTwo,
}
impl MissionGroup {
    pub const fn as_str(self) -> &'static str {
        match self {
            MissionGroup::Clock => "clock",
            MissionGroup::Tutorial => "tutorial",
            MissionGroup::Raids => "raids",
            MissionGroup::Strongholds => "strongholds",
            MissionGroup::QuietAlone => "quiet-alone",
            MissionGroup::QuietTwo => "quiet-two",
        }
    }
}

/// (TypeScript: inline at src/scene/scenarios.ts:574 ScenarioEntry.packs, src/scene/scenarios.ts:574)
#[derive(Clone, Copy, Debug)]
pub struct ScenarioEntryPacks {
    pub frags: Option<f64>,
}
impl ScenarioEntryPacks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScenarioEntryPacks {
            frags: None,
        }
    }
}
impl Default for ScenarioEntryPacks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ScenarioEntryWeather {
    Clear,
    Overcast,
    Rain,
    Downpour,
    Fog,
    Dust,
}
impl ScenarioEntryWeather {
    pub const fn as_str(self) -> &'static str {
        match self {
            ScenarioEntryWeather::Clear => "clear",
            ScenarioEntryWeather::Overcast => "overcast",
            ScenarioEntryWeather::Rain => "rain",
            ScenarioEntryWeather::Downpour => "downpour",
            ScenarioEntryWeather::Fog => "fog",
            ScenarioEntryWeather::Dust => "dust",
        }
    }
}

/// (src/scene/scenarios.ts:88 SQUAD_MAX_MEN)
pub const SQUAD_MAX_MEN: f64 = 11.0;
