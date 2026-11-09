//! The types of `scene_scenarios`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a lesson step's done-test, a variant for each `is` of LESSON_TESTS (scenarios.ts LessonTest, lesson.ts). Unknown: an `is` no row has, or a record that would throw; it never passes (lesson.ts passes)
#[derive(Clone, Debug, ToBytes)]
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
    View { turned: Option<f64>, zoomed: Option<f64> },
    Squad { area: crate::scene_scenarios::Rect, n: Option<f64> },
    Stacks { doors: Vec<String>, n: Option<f64>, set: bool },
    Watching { at: V3, r: Option<f64>, n: Option<f64> },
    All { of: Vec<crate::scene_scenarios::LessonTestKind> },
    Any { of: Vec<crate::scene_scenarios::LessonTestKind> },
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Rect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct TeamStart {
    pub sam: V3,
    pub sam_yaw: f64,
    pub ops: Option<Vec<V3>>,
    pub ops_yaw: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ScenarioMark {
    pub pos: V3,
    pub aim_at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NightBreachData {
    pub team: crate::scene_scenarios::TeamStart,
    pub room: Option<String>,
    pub lights: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StructureDoor {
    pub id: String,
    pub from: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StructureMark {
    pub pos: V3,
    pub watch: String,
    pub aim_at: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct StructureRoom {
    pub id: String,
    pub name: String,
    pub bounds: crate::scene_scenarios::Rect,
    pub doors: Vec<crate::scene_scenarios::StructureDoor>,
    pub garrison: f64,
    pub marks: Vec<crate::scene_scenarios::StructureMark>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct SquadArrive {
    pub door: String,
    pub spawn: Vec<V3>,
    pub men: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RouteOpeningKind {
    #[default] Door,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RouteOpeningSide {
    #[default] Left,
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueFlowData {
    pub corridor: crate::scene_scenarios::Rect,
    pub regions: Vec<crate::scene_scenarios::RescueFlowDataRegionsItem>,
    pub route: Vec<crate::scene_scenarios::RouteOpening>,
    pub hold: crate::scene_scenarios::RescueFlowDataHold,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueFlowDataRegionsItem {
    pub id: String,
    pub door: Option<String>,
    pub rect: crate::scene_scenarios::Rect,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueFlowDataHold {
    pub ahead: f64,
    pub x_max: f64,
    pub z: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueDataSide {
    pub door: String,
    pub hold: V3,
    pub from: Option<crate::scene_scenarios::RescueDataSideFrom>,
    pub pair: Option<bool>,
    pub stack: Option<Vec<V3>>,
    pub standoff: Option<Vec<V3>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RescueDataSideFrom {
    pub door: String,
    pub bounds: Option<crate::scene_scenarios::Rect>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueDataHostagesItem {
    pub pos: V3,
    pub face: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueDataDoorMan {
    pub inside: crate::scene_scenarios::ScenarioMark,
    pub outside: crate::scene_scenarios::ScenarioMark,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RescueDataCast {
    pub roamers: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlaceArea {
    pub name: String,
    pub bounds: crate::scene_scenarios::Rect,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct PlacesData {
    pub areas: Vec<crate::scene_scenarios::PlaceArea>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietStart {
    pub sam: V3,
    pub sam_yaw: f64,
    pub mate: Option<V3>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietExit {
    pub name: String,
    pub door: String,
    pub x: f64,
    pub z: f64,
    pub out: [f64; 2],
    pub half_w: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietStand {
    pub name: String,
    pub pos: V3,
    pub face: f64,
    pub loop_: Option<Vec<V3>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct QuietDataNeeds {
    pub coffee: Option<crate::scene_scenarios::QuietStand>,
    pub smoke: Option<crate::scene_scenarios::QuietStand>,
    pub water: Option<crate::scene_scenarios::QuietStand>,
    pub stretch: Option<crate::scene_scenarios::QuietStand>,
}

#[derive(Clone, Debug, ToBytes)]
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
impl Default for LessonStep {
    fn default() -> Self {
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LessonStepEye {
    pub at: V3,
    pub yaw: f64,
    pub fov: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LessonStepExplain {
    pub title: Option<String>,
    pub text: String,
    pub quiet: Option<bool>,
    pub now: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LessonStepOnWay {
    pub nv: Option<bool>,
    pub run: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LessonStepAfter {
    pub line: Option<String>,
    pub on: Option<Vec<String>>,
    pub word: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RailsData {
    pub allow: Vec<String>,
    pub respawn: Option<f64>,
    pub fog: Option<bool>,
    pub waits: Option<crate::scene_scenarios::RailsDataWaits>,
    pub lasers: Option<bool>,
    pub orders: Option<Vec<String>>,
    pub flat_ring: Option<bool>,
    pub rows: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RailsDataWaits {
    pub low: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LessonData {
    pub title: String,
    pub steps: Vec<crate::scene_scenarios::LessonStep>,
    pub rails: Option<crate::scene_scenarios::RailsData>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ParData {
    pub gold: f64,
    pub silver: f64,
    pub bronze: f64,
    pub loss_secs: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct IssueData {
    pub members: JsMap<String, crate::scene_scenarios::IssueCard>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ResponsePost {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub sweep: [V3; 2],
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ResponseDataSweepItem {
    pub name: String,
    pub pos: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ResponseDataRaid {
    pub room: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelScenarios {
    pub night_breach: Option<crate::scene_scenarios::NightBreachData>,
    pub room_clear: Option<crate::scene_scenarios::StructureData>,
    pub rescue: Option<crate::scene_scenarios::RescueData>,
    pub places: Option<crate::scene_scenarios::PlacesData>,
    pub quiet: Option<crate::scene_scenarios::QuietData>,
    pub lesson: Option<crate::scene_scenarios::LessonData>,
    pub par: Option<crate::scene_scenarios::ParData>,
    pub issue: Option<crate::scene_scenarios::IssueData>,
    pub kit_view: Option<crate::scene_scenarios::KitViewData>,
    pub response: Option<crate::scene_scenarios::ResponseData>,
    pub door_names: Option<Vec<crate::scene_scenarios::LevelScenariosDoorNamesItem>>,
    pub walked: Option<Vec<crate::scene_ways::WayHint>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitViewData {
    pub ways: Vec<crate::scene_scenarios::KitViewWay>,
    pub sets: Option<JsMap<String, crate::scene_scenarios::KitViewSet>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitViewWayPin {
    pub yaw: f64,
    pub pitch: f64,
    pub dist: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitViewLightOver {
    pub porch: Option<crate::scene_scenarios::KitViewLightOverPorch>,
    pub fill: Option<crate::scene_scenarios::KitViewLightOverFill>,
    pub room_lights: Option<f64>,
    pub moon_scale: Option<f64>,
    pub sky_scale: Option<f64>,
    pub sam: Option<crate::scene_scenarios::KitViewLightOverSam>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitViewLightOverPorchFixture {
    pub at: V3,
    pub half: V3,
    pub glow: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitViewLightOverSam {
    pub on: Option<bool>,
    pub intensity: Option<f64>,
    pub inner_deg: Option<f64>,
    pub outer_deg: Option<f64>,
    pub volumetric: Option<f64>,
    pub aim: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KitViewWayWall {
    pub face: Option<f64>,
    pub thickness: Option<f64>,
    pub top: Option<f64>,
}

#[derive(Clone, Debug, ToBytes)]
pub enum KitViewWayPane {
    Bool(bool),
    Arm1(crate::scene_scenarios::KitViewWayPaneArm1),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitViewWayPaneArm1 {
    pub box_: String,
    pub stand: Option<[f64; 3]>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitViewWayAdd {
    pub boxes: Option<Vec<crate::scene_level_data::BoxData>>,
    pub lamps: Option<Vec<crate::scene_level_data::LightData>>,
    pub laid: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KitViewSet {
    pub boxes: Vec<crate::scene_level_data::BoxData>,
    pub doors: Option<Vec<crate::scene_level_data::DoorData>>,
    pub lights: Option<Vec<crate::scene_level_data::LightData>>,
    pub ref_: [f64; 2],
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelScenariosDoorNamesItem {
    pub door: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ScenarioEntry {
    pub id: String,
    pub title: String,
    pub kind: crate::scene_scenarios::ScenarioEntryKind,
    pub squad: f64,
    pub group: Option<crate::scene_scenarios::MissionGroup>,
    pub only: Option<String>,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ScenarioEntryKind {
    #[default] Nightbreach,
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
