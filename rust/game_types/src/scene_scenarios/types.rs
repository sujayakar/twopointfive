//! The types of `scene_scenarios`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum RouteOpeningSide {
    #[default] Left,
    Right,
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
    pub done: crate::lesson::LessonTest,
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
            done: crate::lesson::LessonTest::Any { of: Vec::new() },   // (any of nothing: no deed passes it)
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
    pub lesson: Option<alloc::sync::Arc<crate::scene_scenarios::LessonData>>,   // (shared: a tick holds the list beside `g` with no copy of it)
    pub par: Option<crate::scene_scenarios::ParData>,
    pub issue: Option<crate::scene_scenarios::IssueData>,
    pub response: Option<crate::scene_scenarios::ResponseData>,
    pub door_names: Option<Vec<crate::scene_scenarios::LevelScenariosDoorNamesItem>>,
    pub walked: Option<Vec<crate::scene_ways::WayHint>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LevelScenariosDoorNamesItem {
    pub door: String,
    pub name: String,
}

/// One row of the mission list: which kind, how many teammates, on which level file (the rows: scene_scenario_list). Types and words: the address that is a row is `scenario_query`.
#[derive(Clone, Debug, Default)]
pub struct ScenarioEntry {
    pub id: String,
    pub title: String,
    pub kind: crate::scene_scenarios::MissionKind,
    /// teammates: the pair or the four (the rescue is the pair's as built); a quiet mission's is nobody or one partner
    pub squad: f64,
    /// which heading of the list it is under (none: by what it is)
    pub group: Option<crate::scene_scenarios::MissionGroup>,
    /// it deploys at its own size and no other: the line every other size shows greyed. A design call, not an inability: the level still stages any size from an address
    pub only: Option<String>,
    /// a file under levels/ (none: the office)
    pub level: Option<String>,
    /// the hour and the weather a visit to it is always started in: they win over the address's, and the sheet draws no row for them. None: the visitor's pick, else the level's own. A weather is one a
    /// level can be started in, never snow (a level built for snow keeps its own)
    pub hour: Option<crate::hours::HourId>,
    pub weather: Option<crate::scene_scenarios::ScenarioEntryWeather>,
    /// the room clear's placement seed, a quiet mission's deal pinned (none: dealt on every start)
    pub seed: Option<f64>,
    /// the one line under the title: the premise
    pub blurb: Option<String>,
    /// a lesson's opening card: where we are and why, and what it teaches, a short line a thing, five at most (the things, not their keys)
    pub scene: Option<String>,
    pub teaches: Option<Vec<String>>,
    /// it runs, and nobody has balanced it
    pub untuned: Option<bool>,
    /// cut from the list: the row stays in its table, so its id, its address, its level file and every check that names it hold; no list draws it, and a cut lesson is nobody's next
    pub off_list: Option<bool>,
    /// said of it and read by no rule: where it is, which way it leans, how many hold it as it is staged
    pub place: Option<String>,
    pub lean: Option<crate::ui_missions::Lean>,
    pub defenders: Option<f64>,
    /// the ways in, named: the level's own team start first, then each of its `scenarios.roomClear.starts` in the file's order (none: one way in)
    pub ways: Option<Vec<String>>,
    /// the level's file names defender squads, so Hard is worded as theirs (none: one pool)
    pub squads: Option<bool>,
}

/// what kind of mission it is. A word for lists, cards and addresses: a rule asks the mission that is up what it brings (`g.staged`)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum MissionKind {
    #[default] Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum MissionGroup {
    Clock,
    Tutorial,
    Raids,
    Strongholds,
    #[word("quiet-alone")] QuietAlone,
    #[word("quiet-two")] QuietTwo,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum ScenarioEntryWeather {
    Clear,
    Overcast,
    Rain,
    Downpour,
    Fog,
    Dust,
}

/// the most men a room clear stages in squads
pub const SQUAD_MAX_MEN: f64 = 11.0;
