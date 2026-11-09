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
/// e.near's first argument, DoorRef | Place (director.ts:102, :1092)
#[derive(Clone, Debug)]
pub enum NearWhat {
    Door { d: crate::director::DoorRef },
    Place { p: crate::director::Place },
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
impl PlanKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlanKind::Breach => "breach",
            PlanKind::RoomClear => "roomClear",
        }
    }
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
    pub lower_third: Option<bool>,
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
    pub once: Option<bool>,
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
    pub beats: Option<Vec<String>>,
    pub a: Option<String>,
    pub b: Option<String>,
    pub max: Option<f64>,
    pub what: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum ManifestExpectsItemKind {
    #[default] Order,
    Within,
    Never,
}
impl ManifestExpectsItemKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManifestExpectsItemKind::Order => "order",
            ManifestExpectsItemKind::Within => "within",
            ManifestExpectsItemKind::Never => "never",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ManifestSystems {
    pub civilians: Option<bool>,
    pub needs: Option<bool>,
    pub qrf: Option<bool>,
    pub autosave: Option<bool>,
    pub their_frags: Option<bool>,
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
impl LintProblemLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            LintProblemLevel::Error => "error",
            LintProblemLevel::Warn => "warn",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LogRow {
    pub t: f64,
    pub run: f64,
    pub program: String,
    pub beat: String,
    pub kind: crate::director::LogRowKind,
    pub what: String,
    pub data: Option<crate::director::Json>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LogRowKind {
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
impl LogRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LogRowKind::Camera => "camera",
            LogRowKind::Radio => "radio",
            LogRowKind::Error => "error",
            LogRowKind::Spawn => "spawn",
            LogRowKind::Drill => "drill",
            LogRowKind::Note => "note",
            LogRowKind::Program => "program",
            LogRowKind::Beat => "beat",
            LogRowKind::Trigger => "trigger",
            LogRowKind::Yield => "yield",
            LogRowKind::Stimulus => "stimulus",
        }
    }
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
    pub serial: f64,
    pub name: String,
    pub t: f64,
    pub finished: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DirectorFrame {
    pub look: Option<V3>,
    pub caption: String,
    pub sub: String,
    pub fade: f64,
    pub lower_third: bool,
    pub program: String,
    pub handed_over: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct MarkRef {
    pub kind: crate::director::MarkRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum MarkRefKind {
    #[default] Mark,
}
impl MarkRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            MarkRefKind::Mark => "mark",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DoorRef {
    pub kind: crate::director::DoorRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum DoorRefKind {
    #[default] Door,
}
impl DoorRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DoorRefKind::Door => "door",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct VolumeRef {
    pub kind: crate::director::VolumeRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum VolumeRefKind {
    #[default] Volume,
}
impl VolumeRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VolumeRefKind::Volume => "volume",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RouteRef {
    pub kind: crate::director::RouteRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RouteRefKind {
    #[default] Route,
}
impl RouteRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RouteRefKind::Route => "route",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GroupRef {
    pub kind: crate::director::GroupRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum GroupRefKind {
    #[default] Group,
}
impl GroupRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            GroupRefKind::Group => "group",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VariantRef {
    pub kind: crate::director::VariantRefKind,
    pub id: crate::director::VariantRefId,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum VariantRefKind {
    #[default] Variant,
}
impl VariantRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VariantRefKind::Variant => "variant",
        }
    }
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
    pub kind: crate::director::LoadoutRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum LoadoutRefKind {
    #[default] Loadout,
}
impl LoadoutRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LoadoutRefKind::Loadout => "loadout",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DeviceRef {
    pub kind: crate::director::DeviceRefKind,
    pub id: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DeviceRefKind {
    #[default] Device,
}
impl DeviceRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DeviceRefKind::Device => "device",
        }
    }
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
    /// INVENTED: true only from a caller that passes a goal it keeps and moves (orders.ts:1519, qrf.ts:828, :851): that one compares by value. goal Some(Some(_)) from anyone else is a new leg, even at the same spot (squad.ts:146)
    pub held: bool,
}

#[derive(Clone, Debug)]
pub enum OrderPatch2AimAt {
    List(V3),
    Unit(crate::director::UnitRef),
    Mark(crate::director::MarkRef),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct EventView {
    pub kind: crate::director::EventViewKind,
    pub pos: V3,
    pub level: f64,
    pub loud: bool,
    pub who: f64,
    pub name: Option<String>,
    pub data: Option<crate::director::Json>,
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
    pub k: crate::director::CondArm0K,
    pub until: f64,
    pub label: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm0K {
    #[default] Time,
}
impl CondArm0K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm0K::Time => "time",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CondArm1 {
    pub k: crate::director::CondArm1K,
    pub due: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm1K {
    #[default] Frames,
}
impl CondArm1K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm1K::Frames => "frames",
        }
    }
}

#[derive(Clone, Debug, ToBytes)]
pub struct CondArm2 {
    pub k: crate::director::CondArm2K,
    pub test: crate::director::CondTest,
    pub label: String,
}
impl Default for CondArm2 {
    fn default() -> Self {
        CondArm2 {
            k: crate::director::CondArm2K::Pred,
            test: crate::director::CondTest::Script { id: 0 },
            label: String::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum CondArm2K {
    Pred,
}
impl CondArm2K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm2K::Pred => "pred",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CondArm3 {
    pub k: crate::director::CondArm3K,
    pub kind: crate::director::CondArm3Kind,
    pub name: Option<String>,
    pub filter: Option<crate::director::EventFilter>,
    pub got: Option<crate::director::EventView>,
    pub tick0: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm3K {
    #[default] Event,
}
impl CondArm3K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm3K::Event => "event",
        }
    }
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
    pub k: crate::director::CondArm4K,
    pub name: String,
    pub state: crate::director::CondArm4State,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm4K {
    #[default] Beat,
}
impl CondArm4K {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm4K::Beat => "beat",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CondArm4State {
    #[default] Done,
    Started,
}
impl CondArm4State {
    pub const fn as_str(self) -> &'static str {
        match self {
            CondArm4State::Done => "done",
            CondArm4State::Started => "started",
        }
    }
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
impl BeatState {
    pub const fn as_str(self) -> &'static str {
        match self {
            BeatState::Done => "done",
            BeatState::Failed => "failed",
            BeatState::Running => "running",
            BeatState::Cancelled => "cancelled",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Query {
    pub t: f64,
    pub time: f64,
    pub escalation: f64,
    pub mains_on: bool,
    pub daylight: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DrillRef {
    pub kind: crate::director::DrillRefKind,
    pub name: String,
    pub beat: String,
    pub phase: String,
    pub t: f64,
    pub done: bool,
    pub aborted: bool,
    pub cleared: bool,
    pub ops: Vec<crate::director::UnitRef>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum DrillRefKind {
    #[default] Drill,
}
impl DrillRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DrillRefKind::Drill => "drill",
        }
    }
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
    pub title: Option<String>,
    pub chapter: Option<f64>,
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
            title: None,
            chapter: None,
            look: crate::director::Place::List([0.0; 3]),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BeatCtx {
}

#[derive(Clone, Debug, Default)]
pub struct PuppetSpec {
    pub goal: Option<crate::director::Place>,
    pub aim: Option<crate::director::Place>,
    pub crouch: Option<bool>,
    pub walk: Option<f64>,
    pub fixtures: Option<bool>,
    pub throw_at: Option<crate::director::Place>,
}

#[derive(Clone, Debug, Default)]
pub struct SamView {
    pub pos: V3,
    pub yaw: f64,
    pub speed: f64,
    pub crouch: bool,
    pub holstered: bool,
    pub slot: String,
    pub nv: bool,
    pub hover: Option<crate::director::SamViewHover>,
    pub rows: Vec<String>,
    pub holding: Option<crate::director::SamViewHolding>,
    pub dragging: Option<String>,
    pub takedown: bool,
    pub easing: Option<String>,
    pub picking: Option<String>,
    pub coding: Option<String>,
    pub drive_t: f64,
    pub pulled: bool,
    pub stage: String,
    pub codes: Vec<String>,
    pub keys: Vec<String>,
    pub aim_target: Option<String>,
    pub aim_guard: Option<String>,
    pub ocp: f64,
    pub canisters: f64,
    pub flashbangs: f64,
    pub door_push: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SamViewHover {
    pub kind: String,
    pub verb: Option<String>,
    pub in_reach: bool,
    pub who: Option<String>,
    pub running: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SamViewHolding {
    pub who: String,
    pub phase: String,
    pub variant: String,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProgramApi {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DirectorApi {
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
    pub ref_: crate::director::UnitRef,
    pub gd: GuardId,
    pub loadout: String,
    pub by: String,
}

#[derive(Clone, Debug)]
pub enum DrillImpl {
    Breach(crate::director::DrillImplBreach),
    Clear(crate::director::DrillImplClear),
    Hold,
    Exchange(crate::director::DrillImplExchange),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DrillImplBreach {
    pub forwarded: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DrillImplClear {
    pub d: RoomClearId,
    pub clock: f64,
    pub last_phase: String,
}

#[derive(Clone, Debug, Default)]
pub struct DrillImplExchange {
    pub e: crate::guards::Engagement,
}

#[derive(Clone, Debug)]
pub struct DrillRun {
    pub ref_: crate::director::DrillRef,
    pub impl_: crate::director::DrillImpl,
    pub beat: String,
    pub owner: String,
    pub ops: Vec<crate::director::UnitRef>,
    pub men: Vec<GuardId>,
    pub times: JsMap<String, f64>,
    pub t: f64,
    pub phase: String,
    pub done: bool,
    pub aborted: bool,
    pub cleared: bool,
}
impl Default for DrillRun {
    fn default() -> Self {
        DrillRun {
            ref_: crate::director::DrillRef::default(),
            impl_: crate::director::DrillImpl::Breach(crate::director::DrillImplBreach::default()),
            beat: String::new(),
            owner: String::new(),
            ops: Vec::new(),
            men: Vec::new(),
            times: JsMap::new(),
            t: 0.0,
            phase: String::new(),
            done: false,
            aborted: false,
            cleared: false,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Shot {
    pub spec: crate::director::CameraShotSpec,
    pub t: f64,
    pub title: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShowState {
    pub saved: crate::director::ShowStateSaved,
    pub look: V3,
    pub shot: Option<crate::director::Shot>,
    pub shot_i: f64,
    pub chapter_t0: f64,
    pub chapters: Vec<crate::director::ShowStateChaptersItem>,
    pub caption: String,
    pub sub: String,
    pub fade: f64,
    pub fade_to: f64,
    pub fade_rate: f64,
    pub ending: Option<crate::director::ShowStateEnding>,
    pub benched: Option<Vec<GuardId>>,
    pub variant_on: bool,
    pub puppet: bool,
    pub weather_saved: Option<crate::director::ShowStateWeatherSaved>,
    pub mission_on: bool,
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ShowStateChaptersItem {
    pub title: String,
    pub dur: f64,
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
    pub seed: f64,
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
    pub t: f64,
    pub data: Option<crate::director::Json>,
}

#[derive(Clone, Debug, ToBytes)]
pub struct RunBurstsItem {
    pub u: crate::director::UnitRef,
    pub at: crate::director::RunBurstsItemAt,
    pub left: f64,
    pub cadence: f64,
    pub spread: f64,
    pub next: f64,
    pub cond: crate::director::RunBurstsItemCond,
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
            cond: crate::director::RunBurstsItemCond::default(),
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
pub struct RunBurstsItemCond {
    pub k: crate::director::RunBurstsItemCondK,
    pub label: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum RunBurstsItemCondK {
    #[default] Pred,
}
impl RunBurstsItemCondK {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunBurstsItemCondK::Pred => "pred",
        }
    }
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RunSystemsSaved {
    pub civilians: Option<bool>,
    pub needs: Option<bool>,
    pub qrf: Option<bool>,
    pub autosave: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct BeatFailure {
    pub name: String,
    pub message: String,
    pub stack: Option<String>,
    pub cause: Option<crate::Opaque>,
}
impl BeatFailure {
    /// what a new one holds
    pub fn blank() -> Self {
        BeatFailure {
            name: String::new(),
            message: String::new(),
            stack: None,
            cause: None,
        }
    }
}
impl Default for BeatFailure { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct UTIL {
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
pub struct DirectorChapters {
    pub list: Vec<crate::director::DirectorChaptersListItem>,
    pub i: f64,
    pub frac: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DirectorChaptersListItem {
    pub title: String,
    pub dur: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DirectorPushRow {
    pub kind: crate::director::DirectorPushRowKind,
    pub what: String,
    pub data: Option<crate::director::Json>,
    pub beat: String,
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
impl DirectorPushRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            DirectorPushRowKind::Camera => "camera",
            DirectorPushRowKind::Radio => "radio",
            DirectorPushRowKind::Error => "error",
            DirectorPushRowKind::Spawn => "spawn",
            DirectorPushRowKind::Drill => "drill",
            DirectorPushRowKind::Note => "note",
            DirectorPushRowKind::Program => "program",
            DirectorPushRowKind::Beat => "beat",
            DirectorPushRowKind::Trigger => "trigger",
            DirectorPushRowKind::Yield => "yield",
            DirectorPushRowKind::Stimulus => "stimulus",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DirectorStartOpts {
    pub seed: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct PlanRef {
    pub kind: crate::director::PlanRefKind,
    pub id: String,
    pub plan: crate::director::PlanKind,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum PlanRefKind {
    #[default] Plan,
}
impl PlanRefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlanRefKind::Plan => "plan",
        }
    }
}

#[derive(Clone, Debug)]
pub enum DirectorPlanOf {
    BreachPlan(crate::scene_level::BreachPlan),
    RoomClearPlan(crate::scene_level::RoomClearPlan),
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
    pub t: f64,
    pub data: Option<crate::director::Json>,
}

/// (src/game/director.ts:330 UTIL: a table nothing writes)
pub fn util() -> crate::director::UTIL { crate::director::UTIL {  } }

/// (src/game/director.ts:335 TEAMS: a table nothing writes)
pub fn teams() -> Vec<String> { alloc::vec![String::from("guard"), String::from("operative"), String::from("hostile")] }

/// (src/game/director.ts:377 SHOWCASE_VERBS: a table nothing writes)
pub fn showcase_verbs() -> [String; 16] { [String::from("stage"), String::from("puppet"), String::from("press"), String::from("click"), String::from("camera"), String::from("caption"), String::from("chapters"), String::from("fade"), String::from("variant"), String::from("shift"), String::from("select"), String::from("view"), String::from("weather"), String::from("mission"), String::from("handover"), String::from("chaos")] }

/// (src/game/director.ts:394 MAX_RESUMES)
pub const MAX_RESUMES: f64 = 64.0;

/// (src/game/director.ts:396 LOG_CAP)
pub const LOG_CAP: f64 = 2500.0;

/// (src/game/director.ts:396 MAX_WHEN)
pub const MAX_WHEN: f64 = 32.0;

/// (src/game/director.ts:396 MAX_PASSES)
pub const MAX_PASSES: f64 = 8.0;

