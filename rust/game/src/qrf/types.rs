//! The types of `qrf`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// one row of the response team's knobs as this machine kept them (qrf.ts saveQrfTune, read back by storedTune)
#[derive(Clone, Debug)]
pub enum StoredTuneRow {
    Bool { v: bool },
    Num { v: f64 },
}

/// (TypeScript: Marks, src/game/qrf.ts:114)
#[derive(Clone, Debug)]
pub struct Marks {
    pub door: Option<String>,
    pub name: String,
    pub muster: Vec<V3>,
    pub entry: Vec<V3>,
    pub entry_face: Vec<f64>,
    pub coming: String,
    pub clear: Option<String>,
    pub stairs: bool,
    pub posts: Vec<crate::scene_scenarios::ResponsePost>,
    pub outside: Vec<V3>,
    pub site: String,
    pub at: String,
    pub team: String,
    pub tag: String,
}
impl Marks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Marks {
            door: None,
            name: String::new(),
            muster: Vec::new(),
            entry: Vec::new(),
            entry_face: Vec::new(),
            coming: String::new(),
            clear: None,
            stairs: false,
            posts: Vec::new(),
            outside: Vec::new(),
            site: String::new(),
            at: String::new(),
            team: String::new(),
            tag: String::new(),
        }
    }
}
impl Default for Marks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum QrfState {
    Idle,
    Requested,
    Inbound,
    Arriving,
    Deployed,
}
impl QrfState {
    pub const fn as_str(self) -> &'static str {
        match self {
            QrfState::Idle => "idle",
            QrfState::Requested => "requested",
            QrfState::Inbound => "inbound",
            QrfState::Arriving => "arriving",
            QrfState::Deployed => "deployed",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum QrfReason {
    Gunfire,
    Panel,
    Silent,
    Bodies,
}
impl QrfReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            QrfReason::Gunfire => "gunfire",
            QrfReason::Panel => "panel",
            QrfReason::Silent => "silent",
            QrfReason::Bodies => "bodies",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Bang,
    Fan,
    Breach,
    Hold,
    Move,
    Stack,
    Called,
    Drill,
    Fight,
    Flow,
    Landing,
    File,
    Regroup,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Bang => "bang",
            Phase::Fan => "fan",
            Phase::Breach => "breach",
            Phase::Hold => "hold",
            Phase::Move => "move",
            Phase::Stack => "stack",
            Phase::Called => "called",
            Phase::Drill => "drill",
            Phase::Fight => "fight",
            Phase::Flow => "flow",
            Phase::Landing => "landing",
            Phase::File => "file",
            Phase::Regroup => "regroup",
        }
    }
}

/// (TypeScript: Target, src/game/qrf.ts:197)
#[derive(Clone, Debug)]
pub struct Target {
    pub kind: crate::qrf::TargetKind,
    pub plan: Option<crate::scene_level::RoomClearPlan>,
    pub room: String,
    pub pos: V3,
    pub word: Option<f64>,
}
impl Target {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Target {
            kind: crate::qrf::TargetKind::Room,
            plan: None,
            room: String::new(),
            pos: [0.0; 3],
            word: None,
        }
    }
}
impl Default for Target { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TargetKind {
    Room,
    Point,
}
impl TargetKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TargetKind::Room => "room",
            TargetKind::Point => "point",
        }
    }
}

/// (TypeScript: Perimeter, src/game/qrf.ts:199)
#[derive(Clone, Debug)]
pub struct Perimeter {
    pub state: crate::qrf::PerimeterState,
    pub called_at: f64,
    pub eta_at: f64,
    pub set_at: f64,
    pub men: Vec<crate::qrf::PerimeterMenItem>,
    pub spawned: f64,
    pub peeled: f64,
}
impl Perimeter {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Perimeter {
            state: crate::qrf::PerimeterState::Set,
            called_at: 0.0,
            eta_at: 0.0,
            set_at: 0.0,
            men: Vec::new(),
            spawned: 0.0,
            peeled: 0.0,
        }
    }
}
impl Default for Perimeter { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PerimeterState {
    Set,
    Called,
}
impl PerimeterState {
    pub const fn as_str(self) -> &'static str {
        match self {
            PerimeterState::Set => "set",
            PerimeterState::Called => "called",
        }
    }
}

/// (TypeScript: inline at src/game/qrf.ts:199 Perimeter.men, src/game/qrf.ts:199)
#[derive(Clone, Debug)]
pub struct PerimeterMenItem {
    pub gd: GuardId,
    pub post: crate::scene_scenarios::ResponsePost,
    pub arrived_at: f64,
    pub fighting: bool,
}
impl PerimeterMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PerimeterMenItem {
            gd: GuardId(0),
            post: crate::scene_scenarios::ResponsePost::zeroed(),
            arrived_at: 0.0,
            fighting: false,
        }
    }
}
impl Default for PerimeterMenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Qrf, src/game/qrf.ts:202)
#[derive(Clone, Debug)]
pub struct Qrf {
    pub state: crate::qrf::QrfState,
    pub reason: Option<crate::qrf::QrfReason>,
    pub requested_at: f64,
    pub arrive_at: f64,
    pub deployed_at: f64,
    pub answered: bool,
    pub said30: bool,
    pub said10: bool,
    pub refused_at: f64,
    pub everyone_said: bool,
    pub men: Vec<GuardId>,
    pub perim: Option<crate::qrf::Perimeter>,
    pub bangs: f64,
    pub lost: f64,
    pub got_sam: bool,
    pub sam_was_down: bool,
    pub loud_t: f64,
    pub raid_t: f64,
    pub heard_at: JsMap<GuardId, f64>,
    pub shots_since: Vec<f64>,
    pub heard_events: JsSet<u32>,
    pub silent_at: f64,
    pub silent_said: bool,
    pub silent_spent: bool,
    pub log: Vec<crate::qrf::QrfLogItem>,
    pub phase: crate::qrf::Phase,
    pub phase_t: f64,
    pub target: Option<crate::qrf::Target>,
    pub target_at: f64,
    pub fix: Option<V3>,
    pub fix_t: f64,
    pub cue_rush: bool,
    pub cleared: JsMap<String, f64>,
    pub wps: JsMap<GuardId, Vec<V3>>,
    pub trail: Vec<V3>,
    pub station_t: f64,
    pub point_id: f64,
    pub move_speed: f64,
    pub door_pulled: bool,
    pub filed: f64,
    pub flowed: f64,
    pub can: Option<ItemId>,
    pub can_land_t: f64,
    pub stack_marks: Vec<V3>,
    pub shoved_at: f64,
    pub seen_ids: JsSet<crate::Num>,
    pub aim_pts: JsMap<GuardId, V3>,
    pub stations: JsMap<GuardId, V3>,
    pub cast: Option<crate::squadcast::SquadCast>,
    pub op: Option<DoorOpId>,
    pub op_beat: f64,
    pub drills: f64,
    pub drills_in: f64,
    pub spent: Option<crate::qrf::QrfSpent>,
    pub swept_t: f64,
    pub way: f64,
    pub size: f64,
    pub marks: Option<crate::qrf::Marks>,
    pub marks_on: bool,
    pub marks_way: f64,
    pub word: JsMap<crate::qrf::QrfWordKey, crate::qrf::QrfWordValue>,
    pub word_t: f64,
}
impl Qrf {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Qrf {
            state: crate::qrf::QrfState::Idle,
            reason: None,
            requested_at: -1.0,
            arrive_at: -1.0,
            deployed_at: -1.0,
            answered: false,
            said30: false,
            said10: false,
            refused_at: -100.0,
            everyone_said: false,
            men: Vec::new(),
            perim: None,
            bangs: 0.0,
            lost: 0.0,
            got_sam: false,
            sam_was_down: false,
            loud_t: -1000000000.0,
            raid_t: -1000000000.0,
            heard_at: JsMap::new(),
            shots_since: Vec::new(),
            heard_events: JsSet::new(),
            silent_at: -1.0,
            silent_said: false,
            silent_spent: false,
            log: Vec::new(),
            phase: crate::qrf::Phase::Landing,
            phase_t: 0.0,
            target: None,
            target_at: -100.0,
            fix: None,
            fix_t: -100.0,
            cue_rush: false,
            cleared: JsMap::new(),
            wps: JsMap::new(),
            trail: Vec::new(),
            station_t: 0.0,
            point_id: -1.0,
            move_speed: 2.15,
            door_pulled: false,
            filed: 0.0,
            flowed: 0.0,
            can: None,
            can_land_t: f64::NAN,
            stack_marks: Vec::new(),
            shoved_at: f64::NAN,
            seen_ids: JsSet::new(),
            aim_pts: JsMap::new(),
            stations: JsMap::new(),
            cast: None,
            op: None,
            op_beat: 0.0,
            drills: 0.0,
            drills_in: 0.0,
            spent: None,
            swept_t: -1000000000.0,
            way: 0.0,
            size: -1.0,
            marks: None,
            marks_on: false,
            marks_way: -1.0,
            word: JsMap::new(),
            word_t: -1000000000.0,
        }
    }
}
impl Default for Qrf { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/qrf.ts:222 Qrf.log, src/game/qrf.ts:222)
#[derive(Clone, Debug)]
pub struct QrfLogItem {
    pub t: f64,
    pub what: String,
}
impl QrfLogItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QrfLogItem {
            t: 0.0,
            what: String::new(),
        }
    }
}
impl Default for QrfLogItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:239 Qrf.spent, src/game/qrf.ts:239)
#[derive(Clone, Debug)]
pub struct QrfSpent {
    pub room: String,
    pub fix_t: f64,
}
impl QrfSpent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QrfSpent {
            room: String::new(),
            fix_t: 0.0,
        }
    }
}
impl Default for QrfSpent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum QrfWordKey {
    Guard(GuardId),
    Player,
}

/// (TypeScript: inline at src/game/qrf.ts:247 Qrf.word, src/game/qrf.ts:247)
#[derive(Clone, Debug)]
pub struct QrfWordValue {
    pub at: V3,
    pub t: f64,
    pub how: String,
}
impl QrfWordValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QrfWordValue {
            at: [0.0; 3],
            t: 0.0,
            how: String::new(),
        }
    }
}
impl Default for QrfWordValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ResponseWordSnap, src/game/qrf.ts:363)
#[derive(Clone, Debug)]
pub struct ResponseWordSnap {
    pub who: f64,
    pub at: V3,
    pub age: f64,
    pub how: String,
}
impl ResponseWordSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseWordSnap {
            who: 0.0,
            at: [0.0; 3],
            age: 0.0,
            how: String::new(),
        }
    }
}
impl Default for ResponseWordSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:70 QRF_MARKS, src/game/qrf.ts:70)
#[derive(Clone, Debug)]
pub struct QRFMARKS {
    pub door: String,
    pub landing: Vec<V3>,
    pub entry: Vec<V3>,
    pub entry_face: Vec<f64>,
    pub posts: Vec<crate::qrf::QRFMARKSPostsItem>,
    pub outside_spawn: Vec<V3>,
}
impl QRFMARKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QRFMARKS {
            door: String::new(),
            landing: Vec::new(),
            entry: Vec::new(),
            entry_face: Vec::new(),
            posts: Vec::new(),
            outside_spawn: Vec::new(),
        }
    }
}
impl Default for QRFMARKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:80 QRFMARKS.posts, src/game/qrf.ts:81)
#[derive(Clone, Debug)]
pub struct QRFMARKSPostsItem {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub sweep: Vec<V3>,
}
impl QRFMARKSPostsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QRFMARKSPostsItem {
            name: String::new(),
            pos: [0.0; 3],
            yaw: 0.0,
            sweep: Vec::new(),
        }
    }
}
impl Default for QRFMARKSPostsItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:279 QRF_TUNE_DEFAULTS, src/game/qrf.ts:279)
#[derive(Clone, Copy, Debug)]
pub struct QRFTUNEDEFAULTS {
    pub qrf: bool,
    pub qrf_eta: f64,
    pub qrf_size: f64,
    pub perimeter: bool,
    pub perimeter_eta: f64,
    pub perimeter_size: f64,
    pub qrf_silent: bool,
    pub qrf_squad: bool,
}
impl QRFTUNEDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        QRFTUNEDEFAULTS {
            qrf: false,
            qrf_eta: 0.0,
            qrf_size: 0.0,
            perimeter: false,
            perimeter_eta: 0.0,
            perimeter_size: 0.0,
            qrf_silent: false,
            qrf_squad: false,
        }
    }
}
impl Default for QRFTUNEDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SideOfTeam {
    Guard,
    Hostile,
}
impl SideOfTeam {
    pub const fn as_str(self) -> &'static str {
        match self {
            SideOfTeam::Guard => "guard",
            SideOfTeam::Hostile => "hostile",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum BudgetOf {
    Arm0(crate::qrf::BudgetOfArm0),
    Arm1(crate::qrf::BudgetOfArm1),
}

/// (TypeScript: inline at src/game/qrf.ts:155 budgetOf ->, src/game/qrf.ts:155)
#[derive(Clone, Copy, Debug)]
pub struct BudgetOfArm0 {
    pub budget: f64,
}
impl BudgetOfArm0 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BudgetOfArm0 {
            budget: 0.0,
        }
    }
}
impl Default for BudgetOfArm0 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:155 budgetOf ->, src/game/qrf.ts:155)
#[derive(Clone, Copy, Debug)]
pub struct BudgetOfArm1 {
    pub budget: Option<()>,
}
impl BudgetOfArm1 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BudgetOfArm1 {
            budget: None,
        }
    }
}
impl Default for BudgetOfArm1 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:320 bestFix ->, src/game/qrf.ts:320)
#[derive(Clone, Copy, Debug)]
pub struct BestFix {
    pub pos: V3,
    pub t: f64,
}
impl BestFix {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BestFix {
            pos: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for BestFix { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:349 netWord ->, src/game/qrf.ts:349)
#[derive(Clone, Copy, Debug)]
pub struct NetWord {
    pub pos: V3,
    pub t: f64,
    pub n: f64,
}
impl NetWord {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NetWord {
            pos: [0.0; 3],
            t: 0.0,
            n: 0.0,
        }
    }
}
impl Default for NetWord { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:356 fixOf ->, src/game/qrf.ts:356)
#[derive(Clone, Copy, Debug)]
pub struct FixOf {
    pub pos: V3,
    pub t: f64,
}
impl FixOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixOf {
            pos: [0.0; 3],
            t: 0.0,
        }
    }
}
impl Default for FixOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:426 responseWords ->, src/game/qrf.ts:426)
#[derive(Clone, Debug)]
pub struct ResponseWords {
    pub site: String,
    pub at: String,
    pub team: String,
}
impl ResponseWords {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ResponseWords {
            site: String::new(),
            at: String::new(),
            team: String::new(),
        }
    }
}
impl Default for ResponseWords { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:431 dealt ->, src/game/qrf.ts:431)
#[derive(Clone, Copy, Debug)]
pub struct Dealt {
    pub way: f64,
    pub men: f64,
    pub eta: f64,
}
impl Dealt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Dealt {
            way: 0.0,
            men: 0.0,
            eta: 0.0,
        }
    }
}
impl Default for Dealt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:779 sweepPlaces ->, src/game/qrf.ts:779)
#[derive(Clone, Debug)]
pub struct SweepPlacesItem {
    pub name: String,
    pub pos: V3,
}
impl SweepPlacesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SweepPlacesItem {
            name: String::new(),
            pos: [0.0; 3],
        }
    }
}
impl Default for SweepPlacesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:986 flowLegs ->, src/game/qrf.ts:986)
#[derive(Clone, Debug)]
pub struct FlowLegs {
    pub pts: Vec<V3>,
    pub face: f64,
    pub aim: V3,
}
impl FlowLegs {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FlowLegs {
            pts: Vec::new(),
            face: 0.0,
            aim: [0.0; 3],
        }
    }
}
impl Default for FlowLegs { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/qrf.ts:1110 route(patch), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
pub struct RoutePatch {
    pub goal: Option<V3>,
    pub speed: Option<f64>,
    pub face: Option<f64>,
    pub strafe: Option<bool>,
    pub aim_at: Option<V3>,
    pub stance: Option<crate::character::Stance>,
    pub stack_side: Option<f64>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub upper: Option<crate::character::UpperMode>,
    pub light: Option<bool>,
    pub arrived: Option<bool>,
    pub hot: Option<bool>,
    pub flinch: Option<bool>,
    pub direct: Option<bool>,
    pub smoke: Option<bool>,
    pub over: Option<crate::game::GuardScriptOver>,
}
impl RoutePatch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoutePatch {
            goal: None,
            speed: None,
            face: None,
            strafe: None,
            aim_at: None,
            stance: None,
            stack_side: None,
            crouch: None,
            lean: None,
            upper: None,
            light: None,
            arrived: None,
            hot: None,
            flinch: None,
            direct: None,
            smoke: None,
            over: None,
        }
    }
}
impl Default for RoutePatch { fn default() -> Self { Self::zeroed() } }

/// (src/game/qrf.ts:70 QRF_MARKS: a table nothing writes)
pub fn qrf_marks() -> crate::qrf::QRFMARKS { crate::qrf::QRFMARKS { door: String::from("stairwell"), landing: alloc::vec![[3.1, 0.0, 11.25], [2.25, 0.0, 11.25], [3.1, 0.0, 10.3], [2.25, 0.0, 10.3]], entry: alloc::vec![[6.7, 0.0, 11.2], [5.9, 0.0, 10.55], [5.9, 0.0, 11.8], [5.4, 0.0, 11.25]], entry_face: alloc::vec![1.5707963267948966, 2.4, 0.75, 1.5707963267948966], posts: alloc::vec![crate::qrf::QRFMARKSPostsItem { name: String::from("fire exit"), pos: [38.9, 0.0, 24.2], yaw: -2.48, sweep: alloc::vec![[36.15, 1.0, 19.9], [36.15, 1.0, 21.5]] }, crate::qrf::QRFMARKSPostsItem { name: String::from("break-room door"), pos: [27.4, 0.0, 25.7], yaw: 1.95, sweep: alloc::vec![[30.9, 1.0, 24.15], [32.4, 1.0, 24.15]] }, crate::qrf::QRFMARKSPostsItem { name: String::from("lobby front"), pos: [2.3, 0.0, 20.6], yaw: 1.5707963267948966, sweep: alloc::vec![[3.9, 1.0, 17.6], [3.9, 1.0, 22.4]] }, crate::qrf::QRFMARKSPostsItem { name: String::from("south lot"), pos: [17.4, 0.0, 26.4], yaw: 3.141592653589793, sweep: alloc::vec![[14.5, 1.2, 24.15], [23.5, 1.2, 24.15]] }], outside_spawn: alloc::vec![[38.6, 0.0, 26.6], [37.4, 0.0, 26.9], [38.8, 0.0, 25.4], [37.2, 0.0, 25.6]] } }

/// (src/game/qrf.ts:162 CHAR_BUDGET)
pub const CHAR_BUDGET: f64 = 8.0;

/// (src/game/qrf.ts:164 LOUD_MEMORY_SECS)
pub const LOUD_MEMORY_SECS: f64 = 180.0;

/// (src/game/qrf.ts:167 SILENT_CHECK_SECS)
pub const SILENT_CHECK_SECS: f64 = 6.0;

/// (src/game/qrf.ts:167 SILENT_GO_SECS)
pub const SILENT_GO_SECS: f64 = 14.0;

/// (src/game/qrf.ts:170 ANSWER_SECS)
pub const ANSWER_SECS: f64 = 1.6;

/// (src/game/qrf.ts:170 CALL30)
pub const CALL30: f64 = 30.0;

/// (src/game/qrf.ts:170 CALL10)
pub const CALL10: f64 = 10.0;

/// (src/game/qrf.ts:172 FILE_GAP)
pub const FILE_GAP: f64 = 0.55;

/// (src/game/qrf.ts:172 ENTRY_HOLD)
pub const ENTRY_HOLD: f64 = 1.6;

/// (src/game/qrf.ts:175 MOVE_SPEED)
pub const MOVE_SPEED: f64 = 2.15;

/// (src/game/qrf.ts:175 RUSH_SPEED)
pub const RUSH_SPEED: f64 = 2.6;

/// (src/game/qrf.ts:175 FILE_SPACING)
pub const FILE_SPACING: f64 = 1.3;

/// (src/game/qrf.ts:175 FILE_OFFSET)
pub const FILE_OFFSET: f64 = 0.42;

/// (src/game/qrf.ts:175 STATION_SECS)
pub const STATION_SECS: f64 = 0.35;

/// (src/game/qrf.ts:177 RETARGET_M)
pub const RETARGET_M: f64 = 4.0;

/// (src/game/qrf.ts:177 RETARGET_SECS)
pub const RETARGET_SECS: f64 = 20.0;

/// (src/game/qrf.ts:177 RECLEAR_SECS)
pub const RECLEAR_SECS: f64 = 90.0;

/// (src/game/qrf.ts:180 STACK_CAP)
pub const STACK_CAP: f64 = 5.5;

/// (src/game/qrf.ts:180 BANG_DEPTH)
pub const BANG_DEPTH: f64 = 2.3;

/// (src/game/qrf.ts:180 BANG_FUSE)
pub const BANG_FUSE: f64 = 0.7;

/// (src/game/qrf.ts:180 FLOW_GAP)
pub const FLOW_GAP: f64 = 0.42;

/// (src/game/qrf.ts:180 FLOW_CAP)
pub const FLOW_CAP: f64 = 6.5;

/// (src/game/qrf.ts:180 HOLD360_SECS)
pub const HOLD360_SECS: f64 = 1.6;

/// (src/game/qrf.ts:183 DRILL_CAP)
pub const DRILL_CAP: f64 = 120.0;

/// (src/game/qrf.ts:185 REGROUP_SECS)
pub const REGROUP_SECS: f64 = 8.0;

/// (src/game/qrf.ts:185 HUNT_AWARENESS)
pub const HUNT_AWARENESS: f64 = 0.7;

/// (src/game/qrf.ts:188 GUNFIGHT_SHOTS)
pub const GUNFIGHT_SHOTS: f64 = 3.0;

/// (src/game/qrf.ts:188 GUNFIGHT_WINDOW)
pub const GUNFIGHT_WINDOW: f64 = 20.0;

/// (src/game/qrf.ts:188 SETTLE_IN_SECS)
pub const SETTLE_IN_SECS: f64 = 8.0;

/// (src/game/qrf.ts:188 DOWN_FOR_PERIMETER)
pub const DOWN_FOR_PERIMETER: f64 = 3.0;

/// (src/game/qrf.ts:190 WALKOUT_ENGAGE_M)
pub const WALKOUT_ENGAGE_M: f64 = 6.0;

/// (src/game/qrf.ts:192 POST_SPEED)
pub const POST_SPEED: f64 = 2.6;

/// (src/game/qrf.ts:192 SWEEP_SECS)
pub const SWEEP_SECS: f64 = 5.0;

/// (src/game/qrf.ts:260 TUNE_KEY)
pub const TUNE_KEY: &str = "twopointsix.qrf";

/// (src/game/qrf.ts:279 QRF_TUNE_DEFAULTS: a table nothing writes)
pub fn qrf_tune_defaults() -> crate::qrf::QRFTUNEDEFAULTS { crate::qrf::QRFTUNEDEFAULTS { qrf: true, qrf_eta: 75.0, qrf_size: 4.0, perimeter: true, perimeter_eta: 45.0, perimeter_size: 2.0, qrf_silent: true, qrf_squad: true } }
