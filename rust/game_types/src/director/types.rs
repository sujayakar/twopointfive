//! The types of `director`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// which events a c.on takes: every distinct filter closure of levels/*.script.ts, with what it captured (director.ts EventFilter)
#[derive(Clone, Debug, ToBytes)]
pub enum EventFilter {
    InBox { x0: f64, x1: f64, z0: f64, z1: f64 },
    XAbove { x: f64 },
    XBelow { x: f64 },
    XZAbove { x: f64, z: f64 },
    OrderSignal,
    Heard { by: crate::director::UnitRef, run_level: f64, run_within: f64, walk_level: f64, walk_within: f64 },
}
/// a beat's generator as plain data, so a clone of Game holds where it stands (LiveBeat.it): one arm a beat, its resume point and locals, added by hand with the nine scripts (CHANGES-2 Open 1). Spent: not begun, or closed: it.return() is writing Spent, no script has a finally
#[derive(Clone, Debug, ToBytes, Default)]
pub enum BeatFrame {
    #[default] Spent,
    TutKitHold(crate::script_tut_kit::TutKitHoldFrame),
    TutKitGun(crate::script_tut_kit::TutKitGunFrame),
    TutKitBags(crate::script_tut_kit::TutKitBagsFrame),
    TutKitGap(crate::script_tut_kit::TutKitGapFrame),
    TutKitBlind(crate::script_tut_kit::TutKitBlindFrame),
    TutKitShop(crate::script_tut_kit::TutKitShopFrame),
    TutKitRoom(crate::script_tut_kit::TutKitRoomFrame),
    TutKitLoose(crate::script_tut_kit::TutKitLooseFrame),
    TutKitTrucks(crate::script_tut_kit::TutKitTrucksFrame),
    TutKitCall(crate::script_tut_kit::TutKitCallFrame),
    TutKitBox(crate::script_tut_kit::TutKitBoxFrame),
    TutKitDark(crate::script_tut_kit::TutKitDarkFrame),
    TutKitBunk(crate::script_tut_kit::TutKitBunkFrame),
    TutKitWall(crate::script_tut_kit::TutKitWallFrame),
    TutKitBunkroom(crate::script_tut_kit::TutKitBunkroomFrame),
    TutKitFuelroom(crate::script_tut_kit::TutKitFuelroomFrame),
    TutKitFree(crate::script_tut_kit::TutKitFreeFrame),
    TutKitSeen(crate::script_tut_kit::TutKitSeenFrame),
    TutKitYard(crate::script_tut_kit::TutKitYardFrame),
    TutPairGate(crate::script_tut_pair::TutPairGateFrame),
    TutPairPark(crate::script_tut_pair::TutPairParkFrame),
    TutPairYard(crate::script_tut_pair::TutPairYardFrame),
    TutQueueGun(crate::script_tut_queue::TutQueueGunFrame),
    TutQueueGates(crate::script_tut_queue::TutQueueGatesFrame),
    TutQuietLane(crate::script_tut_quiet::TutQuietLaneFrame),
    TutQuietNight(crate::script_tut_quiet::TutQuietNightFrame),
    TutQuietStore(crate::script_tut_quiet::TutQuietStoreFrame),
    TutRoadHold(crate::script_tut_road::TutRoadHoldFrame),
    TutRoadCease(crate::script_tut_road::TutRoadCeaseFrame),
    TutRoadRoad(crate::script_tut_road::TutRoadRoadFrame),
    TutRoadWay(crate::script_tut_road::TutRoadWayFrame),
    TutRoadHut(crate::script_tut_road::TutRoadHutFrame),
    TutRoadGun(crate::script_tut_road::TutRoadGunFrame),
    TutRoadBags(crate::script_tut_road::TutRoadBagsFrame),
    TutRoadEase(crate::script_tut_road::TutRoadEaseFrame),
    TutRoadGate(crate::script_tut_road::TutRoadGateFrame),
    TutRoadOpen(crate::script_tut_road::TutRoadOpenFrame),
    TutTeamHold(crate::script_tut_team::TutTeamHoldFrame),
    TutTeamCook(crate::script_tut_team::TutTeamCookFrame),
    TutTeamAmbush(crate::script_tut_team::TutTeamAmbushFrame),
    TutTeamPinned(crate::script_tut_team::TutTeamPinnedFrame),
    TutTeamSmoke(crate::script_tut_team::TutTeamSmokeFrame),
    TutTeamBreak(crate::script_tut_team::TutTeamBreakFrame),
    TutTeamOrdered(crate::script_tut_team::TutTeamOrderedFrame),
    TutTeamBlind(crate::script_tut_team::TutTeamBlindFrame),
    TutTeamLinger(crate::script_tut_team::TutTeamLingerFrame),
    TutTeamPush(crate::script_tut_team::TutTeamPushFrame),
    TutTeamIndoors(crate::script_tut_team::TutTeamIndoorsFrame),
    TutTeamHouse(crate::script_tut_team::TutTeamHouseFrame),
    TutTeamOut(crate::script_tut_team::TutTeamOutFrame),
}
/// what a c.until asks (CondArm2.test): Script, a test of the beat's own script, answered by beat_test with the beat's frame; Burst, the stimulus.burst's cond at director.ts:1012: no row of r.bursts with this serial and left > 0
#[derive(Clone, Debug, ToBytes)]
pub enum CondTest {
    Script { id: u32 },
    Burst { serial: u32 },
}
/// what a level script's define pass names and its beats read: one arm a script (scriptgen.cjs)
#[derive(Clone, Debug, ToBytes, Default)]
pub enum ScriptVars {
    #[default] None,
    TutKit(crate::script_tut_kit::TutKitVars),
    TutPair(crate::script_tut_pair::TutPairVars),
    TutQueue(crate::script_tut_queue::TutQueueVars),
    TutQuiet(crate::script_tut_quiet::TutQuietVars),
    TutRoad(crate::script_tut_road::TutRoadVars),
    TutTeam(crate::script_tut_team::TutTeamVars),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum EventKind {
    #[default] Shot,
    Bang,
    Kick,
    Door,
    LightOut,
    GuardShot,
    Step,
    Prop,
    Whistle,
    Glass,
}
impl EventKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventKind::Shot => "shot",
            EventKind::Bang => "bang",
            EventKind::Kick => "kick",
            EventKind::Door => "door",
            EventKind::LightOut => "lightOut",
            EventKind::GuardShot => "guardShot",
            EventKind::Step => "step",
            EventKind::Prop => "prop",
            EventKind::Whistle => "whistle",
            EventKind::Glass => "glass",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct Director {
    pub manifest: Option<crate::director::Manifest>,
    pub problems: Vec<crate::director::LintProblem>,
    pub armed: bool,
    pub log: Vec<crate::director::LogRow>,
    pub run: Option<DirectorRunId>,
    pub last_run: Option<crate::director::DirectorLastRun>,
    pub frame: Option<crate::director::DirectorFrame>,
    pub serial: f64,
    pub seen: JsSet<u32>,
    pub exiting: bool,
    pub due_keys: JsSet<String>,
    pub due_clicks: f64,
    pub shown: JsSet<GuardId>,
}
impl Director {
    /// what a new one holds
    pub fn blank() -> Self {
        Director {
            manifest: None,
            problems: Vec::new(),
            armed: false,
            log: Vec::new(),
            run: None,
            last_run: None,
            frame: None,
            serial: 0.0,
            seen: JsSet::new(),
            exiting: false,
            due_keys: JsSet::new(),
            due_clicks: 0.0,
            shown: JsSet::new(),
        }
    }
}
impl Default for Director { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Manifest {
    pub marks: JsMap<String, crate::director::ManifestMarksValue>,
    pub volumes: JsSet<String>,
    pub routes: JsSet<String>,
    pub doors: JsSet<String>,
    pub devices: JsSet<String>,
    pub plans: JsMap<String, crate::director::PlanKind>,
    pub loadouts: JsMap<String, crate::director::LoadoutSpec>,
    pub groups: JsSet<String>,
    pub variants: JsSet<String>,
    pub programs: JsMap<String, crate::director::ProgramDef>,
    pub expects: Vec<crate::director::ManifestExpectsItem>,
    pub systems: JsMap<&'static str, bool>,
    /// INVENTED: the locals of the script's define pass
    pub vars: crate::director::ScriptVars,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ManifestMarksValue {
    pub pos: Option<V3>,
    pub yaw: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum PlanKind {
    #[default] Breach,
    RoomClear,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LoadoutSpec {
    pub team: crate::teams::Team,
    pub weapon: Option<crate::weapons::WeaponId>,
    pub tint: Option<V3>,
    pub tint2: Option<V3>,
    pub light: Option<bool>,
    pub trained: Option<bool>,
    pub god: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ProgramDef {
    pub name: String,
    pub opts: crate::director::ProgramOpts,
    pub beats: JsMap<String, crate::director::ProgramDefBeatsValue>,
    pub triggers: Vec<crate::director::TriggerDef>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ProgramOpts {
    pub showcase: Option<bool>,
    pub end_veil: Option<f64>,
    pub end_hold: Option<f64>,
    pub fresh_encounter: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ProgramDefBeatsValue {
    pub fn_: crate::scene_level::LevelScriptFn,
    pub atomic: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct TriggerDef {
    pub kind: crate::director::TriggerDefKind,
    pub beat: String,
    pub secs: Option<f64>,
    pub event: Option<crate::director::EventKind>,
    pub name: Option<String>,
    pub volume: Option<String>,
    pub who: Option<crate::director::TriggerDefWho>,
    pub label: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum TriggerDefKind {
    #[default] Start,
    Enter,
    After,
    Event,
    Signal,
    When,
}
impl TriggerDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TriggerDefKind::Start => "start",
            TriggerDefKind::Enter => "enter",
            TriggerDefKind::After => "after",
            TriggerDefKind::Event => "event",
            TriggerDefKind::Signal => "signal",
            TriggerDefKind::When => "when",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub enum TriggerDefWho {
    Sam,
    AnyUnit,
    Unit(crate::director::UnitRef),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct UnitRef {
    pub id: f64,
    pub callsign: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ManifestExpectsItem {
    pub kind: crate::director::ManifestExpectsItemKind,
    pub program: Option<String>,
    pub max: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum ManifestExpectsItemKind {
    #[default] Order,
    Within,
    Never,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LintProblem {
    pub level: crate::director::LintProblemLevel,
    pub what: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LintProblemLevel {
    #[default] Error,
    Warn,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LogRow {
}

#[derive(Clone, Debug, ToBytes)]
pub enum Json {
    Text(String),
    Num(f64),
    Bool(bool),
    Array(Vec<Option<crate::director::Json>>),
    Arm4(JsMap<String, Option<crate::director::Json>>),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DirectorLastRun {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DirectorFrame {
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MarkRef {
    pub id: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorRef {
    pub id: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct VolumeRef {
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum VariantRefId {
    #[default] Daylight,
}
impl VariantRefId {
    pub const fn as_str(self) -> &'static str {
        match self {
            VariantRefId::Daylight => "daylight",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LoadoutRef {
    pub id: String,
}

#[derive(Clone, Debug, ToBytes)]
pub enum Place {
    List(V3),
    Mark(crate::director::MarkRef),
}

#[derive(Clone, Debug, Default)]
pub struct OrderPatch2 {
    pub goal: Option<Option<crate::director::Place>>,
    pub speed: Option<Option<f64>>,
    pub face: Option<Option<f64>>,
    pub strafe: Option<Option<bool>>,
    pub aim_at: Option<Option<crate::director::Place>>,
    pub stance: Option<Option<crate::character::Stance>>,
    pub stack_side: Option<Option<f64>>,
    pub crouch: Option<Option<bool>>,
    pub upper: Option<Option<crate::character::UpperMode>>,
    pub light: Option<Option<bool>>,
    pub hot: Option<Option<bool>>,
    pub flinch: Option<Option<bool>>,
    pub direct: Option<Option<bool>>,
    pub smoke: Option<Option<bool>>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EventView {
    pub kind: crate::director::EventViewKind,
    pub pos: V3,
    pub level: f64,
    pub name: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum EventViewKind {
    #[default] Shot,
    Bang,
    Kick,
    Door,
    LightOut,
    GuardShot,
    Step,
    Prop,
    Whistle,
    Glass,
    Signal,
}
impl EventViewKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventViewKind::Shot => "shot",
            EventViewKind::Bang => "bang",
            EventViewKind::Kick => "kick",
            EventViewKind::Door => "door",
            EventViewKind::LightOut => "lightOut",
            EventViewKind::GuardShot => "guardShot",
            EventViewKind::Step => "step",
            EventViewKind::Prop => "prop",
            EventViewKind::Whistle => "whistle",
            EventViewKind::Glass => "glass",
            EventViewKind::Signal => "signal",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub enum Cond {
    Arm0(crate::director::CondArm0),
    Arm1(crate::director::CondArm1),
    Arm2(crate::director::CondArm2),
    Arm3(crate::director::CondArm3),
    Arm4(crate::director::CondArm4),
    Arm5(crate::director::CondArm5),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CondArm0 {
    pub until: f64,
    pub label: Option<String>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CondArm1 {
    pub due: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct CondArm2 {
    pub test: crate::director::CondTest,
    pub label: String,
}
impl Default for CondArm2 {
    fn default() -> Self {
        CondArm2 {
            test: crate::director::CondTest::Script { id: 0 },
            label: String::new(),
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CondArm3 {
    pub kind: crate::director::CondArm3Kind,
    pub name: Option<String>,
    pub filter: Option<crate::director::EventFilter>,
    pub got: Option<crate::director::EventView>,
    pub tick0: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm3Kind {
    #[default] Shot,
    Bang,
    Kick,
    Door,
    LightOut,
    GuardShot,
    Step,
    Prop,
    Whistle,
    Glass,
    Signal,
}
impl CondArm3Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm3Kind::Shot => "shot",
            CondArm3Kind::Bang => "bang",
            CondArm3Kind::Kick => "kick",
            CondArm3Kind::Door => "door",
            CondArm3Kind::LightOut => "lightOut",
            CondArm3Kind::GuardShot => "guardShot",
            CondArm3Kind::Step => "step",
            CondArm3Kind::Prop => "prop",
            CondArm3Kind::Whistle => "whistle",
            CondArm3Kind::Glass => "glass",
            CondArm3Kind::Signal => "signal",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CondArm4 {
    pub name: String,
    pub state: crate::director::CondArm4State,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm4State {
    #[default] Done,
    Started,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CondArm5 {
    pub k: crate::director::CondArm5K,
    pub of: Vec<crate::director::Cond>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm5K {
    #[default] All,
    Any,
}
impl CondArm5K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm5K::All => "all",
            CondArm5K::Any => "any",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CondResult {
    pub index: f64,
    pub event: Option<crate::director::EventView>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum BeatState {
    #[default] Done,
    Failed,
    Running,
    Cancelled,
}

#[derive(Clone, Debug, ToBytes)]
pub struct CameraShotSpec {
    pub look_y: Option<f64>,
    pub track: Option<f64>,
    pub yaw: f64,
    pub yaw_to: Option<f64>,
    pub pitch: f64,
    pub dist: f64,
    pub dist_to: Option<f64>,
    pub dur: f64,
    /// INVENTED by a review: see the proposals
    pub look: crate::director::Place,
}
impl Default for CameraShotSpec {
    fn default() -> Self {
        CameraShotSpec {
            look_y: None,
            track: None,
            yaw: 0.0,
            yaw_to: None,
            pitch: 0.0,
            dist: 0.0,
            dist_to: None,
            dur: 0.0,
            look: crate::director::Place::List([0.0; 3]),
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LiveBeat {
    pub name: String,
    pub it: crate::director::BeatFrame,
    pub waiting: Option<crate::director::Cond>,
    pub result: crate::director::CondResult,
    pub t0: f64,
    pub step: f64,
    pub state: crate::director::BeatState,
    pub touched: JsSet<crate::Num>,
    pub pinned_camera: bool,
    pub flow: crate::director::LiveBeatFlow,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct LiveBeatFlow {
    pub finish: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct UnitRec {
    pub gd: GuardId,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Shot {
    pub spec: crate::director::CameraShotSpec,
    pub t: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShowState {
    pub saved: crate::director::ShowStateSaved,
    pub look: V3,
    pub shot: Option<crate::director::Shot>,
    pub fade: f64,
    pub fade_to: f64,
    pub fade_rate: f64,
    pub ending: Option<crate::director::ShowStateEnding>,
    pub benched: Option<Vec<GuardId>>,
    pub variant_on: bool,
    pub puppet: bool,
    pub weather_saved: Option<crate::director::ShowStateWeatherSaved>,
    pub handed_over: bool,
    pub chaos_set: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShowStateSaved {
    pub ai: bool,
    pub god: bool,
    pub invisible: bool,
    pub quiet: bool,
    pub touring: bool,
    pub cam: Option<crate::director::ShowStateSavedCam>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShowStateSavedCam {
    pub yaw: f64,
    pub pitch: f64,
    pub dist: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShowStateEnding {
    pub t: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShowStateWeatherSaved {
    pub storm: f64,
    pub rain: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Run {
    pub serial: f64,
    pub name: String,
    pub def: crate::director::ProgramDef,
    pub t: f64,
    pub dt: f64,
    pub tick_n: f64,
    pub dice: crate::dice::Die,
    pub beats: Vec<DirectorLiveBeatId>,
    pub fired: JsSet<crate::Num>,
    pub signals: Vec<crate::director::RunSignalsItem>,
    pub sig_seen: f64,
    pub flags: JsMap<String, Option<crate::director::Json>>,
    pub units: JsMap<crate::Num, crate::director::UnitRec>,
    pub bursts: Vec<crate::director::RunBurstsItem>,
    pub kicks: Vec<crate::director::RunKicksItem>,
    pub show: Option<crate::director::ShowState>,
    pub objective: Option<crate::director::RunObjective>,
    pub systems_saved: Option<JsMap<&'static str, bool>>,
    pub ended: bool,
    pub finishing: bool,
    pub failed_show: bool,
    /// INVENTED: the manifest's, for a beat's step to read
    pub vars: crate::director::ScriptVars,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RunSignalsItem {
    pub name: String,
}

#[derive(Clone, Debug, ToBytes)]
pub struct RunBurstsItem {
    pub u: crate::director::UnitRef,
    pub at: crate::director::RunBurstsItemAt,
    pub left: f64,
    pub cadence: f64,
    pub spread: f64,
    pub next: f64,
    pub fired: f64,
    /// INVENTED by a review: see the proposals
    pub serial: u32,
}
impl Default for RunBurstsItem {
    fn default() -> Self {
        RunBurstsItem {
            u: crate::director::UnitRef::default(),
            at: crate::director::RunBurstsItemAt::List([0.0; 3]),
            left: 0.0,
            cadence: 0.0,
            spread: 0.0,
            next: 0.0,
            fired: 0.0,
            serial: 0,
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub enum RunBurstsItemAt {
    List(V3),
    Unit(crate::director::UnitRef),
    Mark(crate::director::MarkRef),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RunKicksItem {
    pub door: DoorId,
    pub gd: GuardId,
    pub by: String,
    pub beat: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RunObjective {
    pub text: String,
    pub pos: Option<V3>,
}

#[derive(Clone, Debug, Default)]
pub struct LintManifestLevel {
    pub routes: Vec<RouteId>,
    pub breach: Option<crate::scene_level::BreachPlan>,
    pub devices: Option<Vec<crate::scene_level::DeviceDef>>,
    pub doors: Vec<crate::scene_level::LevelDoorDef>,
    pub lights: Vec<crate::scene_level::LightDef>,
    pub switchables: Vec<crate::scene_level::LevelSwitchablesItem>,
    pub clear_plans: Option<Vec<crate::scene_level::RoomClearPlan>>,
    pub daylight: Option<crate::scene_level::DaylightDef>,
    pub volumes: Option<Vec<crate::scene_level::VolumeDef>>,
}

#[derive(Clone, Debug, Default)]
pub struct DirectorObjective {
    pub text: String,
    pub pos: Option<V3>,
}

#[derive(Clone, Debug)]
pub enum DirectorSignalData {
    Text(String),
    Num(f64),
    Bool(bool),
    Array(Vec<Option<crate::director::Json>>),
    Arm4(JsMap<String, Option<crate::director::Json>>),
}

#[derive(Clone, Debug, Default)]
pub struct DirectorPushRow {
    pub t: Option<f64>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DirectorPushRowKind {
    #[default] Camera,
    Radio,
    Error,
    Spawn,
    Drill,
    Note,
    Program,
    Beat,
    Trigger,
    Yield,
    Stimulus,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DirectorStartOpts {
    pub seed: Option<f64>,
}

#[derive(Clone, Debug)]
pub enum DirectorInsideVolumeWho {
    Team,
    Sam,
    AnyUnit,
    Unit(crate::director::UnitRef),
}

#[derive(Clone, Debug, Default)]
pub struct SignalViewS {
    pub name: String,
}

/// (src/game/director.ts:335 TEAMS: a table nothing writes)
pub fn teams() -> Vec<String> { alloc::vec![String::from("guard"), String::from("operative"), String::from("hostile")] }

/// (src/game/director.ts:394 MAX_RESUMES)
pub const MAX_RESUMES: f64 = 64.0;

/// (src/game/director.ts:396 LOG_CAP)
pub const LOG_CAP: f64 = 2500.0;

/// (src/game/director.ts:396 MAX_WHEN)
pub const MAX_WHEN: f64 = 32.0;

/// (src/game/director.ts:396 MAX_PASSES)
pub const MAX_PASSES: f64 = 8.0;

