//! The types of `chronicle`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronKind {
    Down,
    Shot,
    Blackout,
    Heard,
    Objective,
    Grab,
    Ocp,
    Thrown,
    Radio,
    Spotted,
    Contact,
    Lost,
    Calm,
    Alarm,
    LightShot,
    Interrogated,
    KeyTaken,
    Ko,
    Kill,
    BodyFound,
    DoorKicked,
    LockPicked,
    Hit,
    NeedBreak,
    CivilianSpotted,
    CivilianReported,
    CivilianComplied,
    CivilianKO,
    CivilianKilled,
    CameraSpotted,
    CameraAlarm,
    LaserTripped,
    DeviceDown,
    CodeLearned,
    CodeEntered,
    Checkpoint,
}
impl ChronKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChronKind::Down => "down",
            ChronKind::Shot => "shot",
            ChronKind::Blackout => "blackout",
            ChronKind::Heard => "heard",
            ChronKind::Objective => "objective",
            ChronKind::Grab => "grab",
            ChronKind::Ocp => "ocp",
            ChronKind::Thrown => "thrown",
            ChronKind::Radio => "radio",
            ChronKind::Spotted => "spotted",
            ChronKind::Contact => "contact",
            ChronKind::Lost => "lost",
            ChronKind::Calm => "calm",
            ChronKind::Alarm => "alarm",
            ChronKind::LightShot => "lightShot",
            ChronKind::Interrogated => "interrogated",
            ChronKind::KeyTaken => "keyTaken",
            ChronKind::Ko => "ko",
            ChronKind::Kill => "kill",
            ChronKind::BodyFound => "bodyFound",
            ChronKind::DoorKicked => "doorKicked",
            ChronKind::LockPicked => "lockPicked",
            ChronKind::Hit => "hit",
            ChronKind::NeedBreak => "needBreak",
            ChronKind::CivilianSpotted => "civilianSpotted",
            ChronKind::CivilianReported => "civilianReported",
            ChronKind::CivilianComplied => "civilianComplied",
            ChronKind::CivilianKO => "civilianKO",
            ChronKind::CivilianKilled => "civilianKilled",
            ChronKind::CameraSpotted => "cameraSpotted",
            ChronKind::CameraAlarm => "cameraAlarm",
            ChronKind::LaserTripped => "laserTripped",
            ChronKind::DeviceDown => "deviceDown",
            ChronKind::CodeLearned => "codeLearned",
            ChronKind::CodeEntered => "codeEntered",
            ChronKind::Checkpoint => "checkpoint",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: ChronEvent, src/game/chronicle.ts:85)
#[derive(Clone, Debug)]
pub struct ChronEvent {
    pub t: f64,
    pub kind: crate::chronicle::ChronKind,
    pub room: String,
    pub pos: V3,
    pub data: JsMap<String, Option<crate::chronicle::ChronEventDataValue>>,
}
impl ChronEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChronEvent {
            t: 0.0,
            kind: crate::chronicle::ChronKind::Down,
            room: String::new(),
            pos: [0.0; 3],
            data: JsMap::new(),
        }
    }
}
impl Default for ChronEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronEventDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronCountKey {
    Heard,
    SeenUp,
    Esc1,
    Esc2,
    ShotsNearMen,
    LightsShot,
    Ocps,
    Blackouts,
    Kicked,
    Picked,
    Intel,
    Thrown,
    Grabs,
    Contacts,
    Radio,
    CivSeen,
    CivReported,
    CivComplied,
    CivKO,
    CivKilled,
    Devices,
    Hits,
    Saves,
    Loads,
}
impl ChronCountKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChronCountKey::Heard => "heard",
            ChronCountKey::SeenUp => "seenUp",
            ChronCountKey::Esc1 => "esc1",
            ChronCountKey::Esc2 => "esc2",
            ChronCountKey::ShotsNearMen => "shotsNearMen",
            ChronCountKey::LightsShot => "lightsShot",
            ChronCountKey::Ocps => "ocps",
            ChronCountKey::Blackouts => "blackouts",
            ChronCountKey::Kicked => "kicked",
            ChronCountKey::Picked => "picked",
            ChronCountKey::Intel => "intel",
            ChronCountKey::Thrown => "thrown",
            ChronCountKey::Grabs => "grabs",
            ChronCountKey::Contacts => "contacts",
            ChronCountKey::Radio => "radio",
            ChronCountKey::CivSeen => "civSeen",
            ChronCountKey::CivReported => "civReported",
            ChronCountKey::CivComplied => "civComplied",
            ChronCountKey::CivKO => "civKO",
            ChronCountKey::CivKilled => "civKilled",
            ChronCountKey::Devices => "devices",
            ChronCountKey::Hits => "hits",
            ChronCountKey::Saves => "saves",
            ChronCountKey::Loads => "loads",
        }
    }
}

/// (TypeScript: ChronCounts, src/game/chronicle.ts:105)
#[derive(Clone, Debug)]
pub struct ChronCounts {
    pub seen_up: f64,
    pub esc1: f64,
    pub esc2: f64,
    pub shots_near_men: f64,
    pub lights_shot: f64,
    pub ocps: f64,
    pub blackouts: f64,
    pub kicked: f64,
    pub picked: f64,
    pub intel: f64,
    pub thrown: f64,
    pub grabs: f64,
    pub contacts: f64,
    pub heard: f64,
    pub radio: f64,
    pub civ_seen: f64,
    pub civ_reported: f64,
    pub civ_complied: f64,
    pub civ_ko: f64,
    pub civ_killed: f64,
    pub devices: f64,
    pub hits: f64,
    pub down: bool,
    pub down_by: String,
    pub saves: f64,
    pub loads: f64,
}
impl ChronCounts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ChronCounts {
            seen_up: 0.0,
            esc1: 0.0,
            esc2: 0.0,
            shots_near_men: 0.0,
            lights_shot: 0.0,
            ocps: 0.0,
            blackouts: 0.0,
            kicked: 0.0,
            picked: 0.0,
            intel: 0.0,
            thrown: 0.0,
            grabs: 0.0,
            contacts: 0.0,
            heard: 0.0,
            radio: 0.0,
            civ_seen: 0.0,
            civ_reported: 0.0,
            civ_complied: 0.0,
            civ_ko: 0.0,
            civ_killed: 0.0,
            devices: 0.0,
            hits: 0.0,
            down: false,
            down_by: String::new(),
            saves: 0.0,
            loads: 0.0,
        }
    }
}
impl Default for ChronCounts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Chronicle, src/game/chronicle.ts:118)
#[derive(Clone, Debug)]
pub struct Chronicle {
    pub events: Vec<crate::chronicle::ChronEvent>,
    pub dropped: f64,
    pub counts: crate::chronicle::ChronCounts,
    pub questioned: JsSet<String>,
    pub path: Vec<f32>,
    pub path_n: f64,
    pub path_every: f64,
    pub path_next: f64,
    pub heard_of: JsMap<u32, usize>,
    pub heard_no_row: JsSet<u32>,
}
impl Chronicle {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Chronicle {
            events: Vec::new(),
            dropped: 0.0,
            counts: crate::chronicle::ChronCounts { seen_up: 0.0, esc1: 0.0, esc2: 0.0, shots_near_men: 0.0, lights_shot: 0.0, ocps: 0.0, blackouts: 0.0, kicked: 0.0, picked: 0.0, intel: 0.0, thrown: 0.0, grabs: 0.0, contacts: 0.0, heard: 0.0, radio: 0.0, civ_seen: 0.0, civ_reported: 0.0, civ_complied: 0.0, civ_ko: 0.0, civ_killed: 0.0, devices: 0.0, hits: 0.0, down: false, down_by: String::from(""), saves: 0.0, loads: 0.0 },
            questioned: JsSet::new(),
            path: alloc::vec![0.0; 12000],
            path_n: 0.0,
            path_every: 0.5,
            path_next: 0.0,
            heard_of: JsMap::new(),
            heard_no_row: JsSet::new(),
        }
    }
}
impl Default for Chronicle { fn default() -> Self { Self::blank() } }

/// (TypeScript: RunCounts, src/game/chronicle.ts:325)
#[derive(Clone, Debug)]
pub struct RunCounts {
    pub elapsed: f64,
    pub stage: String,
    pub done: bool,
    pub failed: bool,
    pub sandbox: bool,
    pub blackout: bool,
    pub seen: f64,
    pub seen_up: f64,
    pub alarms: f64,
    pub esc1: f64,
    pub esc2: f64,
    pub bodies: f64,
    pub kills: f64,
    pub kos: f64,
    pub shots: f64,
    pub shots_near_men: f64,
    pub lights_shot: f64,
    pub ocps: f64,
    pub kicked: f64,
    pub picked: f64,
    pub keys: f64,
    pub intel: f64,
    pub questioned: f64,
    pub roster: f64,
    pub hits: f64,
    pub down: bool,
    pub loads: f64,
    pub thrown: f64,
    pub grabs: f64,
    pub dropped: f64,
    pub devices: f64,
    pub civ_seen: f64,
    pub civ_reported: f64,
    pub civ_complied: f64,
    pub civ_ko: f64,
    pub civ_killed: f64,
}
impl RunCounts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RunCounts {
            elapsed: 0.0,
            stage: String::new(),
            done: false,
            failed: false,
            sandbox: false,
            blackout: false,
            seen: 0.0,
            seen_up: 0.0,
            alarms: 0.0,
            esc1: 0.0,
            esc2: 0.0,
            bodies: 0.0,
            kills: 0.0,
            kos: 0.0,
            shots: 0.0,
            shots_near_men: 0.0,
            lights_shot: 0.0,
            ocps: 0.0,
            kicked: 0.0,
            picked: 0.0,
            keys: 0.0,
            intel: 0.0,
            questioned: 0.0,
            roster: 0.0,
            hits: 0.0,
            down: false,
            loads: 0.0,
            thrown: 0.0,
            grabs: 0.0,
            dropped: 0.0,
            devices: 0.0,
            civ_seen: 0.0,
            civ_reported: 0.0,
            civ_complied: 0.0,
            civ_ko: 0.0,
            civ_killed: 0.0,
        }
    }
}
impl Default for RunCounts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ScoreLine, src/game/chronicle.ts:364)
#[derive(Clone, Debug)]
pub struct ScoreLine {
    pub label: String,
    pub pts: f64,
}
impl ScoreLine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ScoreLine {
            label: String::new(),
            pts: 0.0,
        }
    }
}
impl Default for ScoreLine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Badge, src/game/chronicle.ts:403)
#[derive(Clone, Debug)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub on: bool,
    pub hint: String,
}
impl Badge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Badge {
            id: String::new(),
            name: String::new(),
            on: false,
            hint: String::new(),
        }
    }
}
impl Default for Badge { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Debrief, src/game/chronicle.ts:425)
#[derive(Clone, Debug)]
pub struct Debrief {
    pub counts: crate::chronicle::RunCounts,
    pub score: f64,
    pub lines: Vec<crate::chronicle::ScoreLine>,
    pub rating: String,
    pub blurb: String,
    pub badges: Vec<crate::chronicle::Badge>,
    pub summary: String,
    pub state: crate::chronicle::DebriefState,
    pub scored: bool,
    pub grade: String,
}
impl Debrief {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Debrief {
            counts: crate::chronicle::RunCounts::zeroed(),
            score: 0.0,
            lines: Vec::new(),
            rating: String::new(),
            blurb: String::new(),
            badges: Vec::new(),
            summary: String::new(),
            state: crate::chronicle::DebriefState::Down,
            scored: false,
            grade: String::new(),
        }
    }
}
impl Default for Debrief { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DebriefState {
    Down,
    Done,
    Sofar,
}
impl DebriefState {
    pub const fn as_str(self) -> &'static str {
        match self {
            DebriefState::Down => "down",
            DebriefState::Done => "done",
            DebriefState::Sofar => "sofar",
        }
    }
}

/// (TypeScript: inline at src/game/chronicle.ts:27 ROOMS, src/game/chronicle.ts:27)
#[derive(Clone, Debug)]
pub struct ROOMSItem {
    pub name: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl ROOMSItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ROOMSItem {
            name: String::new(),
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for ROOMSItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/chronicle.ts:392 BANDS, src/game/chronicle.ts:392)
#[derive(Clone, Debug)]
pub struct BANDSItem {
    pub min: f64,
    pub word: String,
    pub blurb: String,
}
impl BANDSItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BANDSItem {
            min: 0.0,
            word: String::new(),
            blurb: String::new(),
        }
    }
}
impl Default for BANDSItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/chronicle.ts:507 PIN, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct PIN {
    pub down: Option<crate::chronicle::PINDown>,
    pub shot: Option<crate::chronicle::PINDown>,
    pub blackout: Option<crate::chronicle::PINDown>,
    pub heard: Option<crate::chronicle::PINDown>,
    pub objective: Option<crate::chronicle::PINDown>,
    pub grab: Option<crate::chronicle::PINDown>,
    pub ocp: Option<crate::chronicle::PINDown>,
    pub thrown: Option<crate::chronicle::PINDown>,
    pub radio: Option<crate::chronicle::PINDown>,
    pub spotted: Option<crate::chronicle::PINDown>,
    pub contact: Option<crate::chronicle::PINDown>,
    pub lost: Option<crate::chronicle::PINDown>,
    pub calm: Option<crate::chronicle::PINDown>,
    pub alarm: Option<crate::chronicle::PINDown>,
    pub light_shot: Option<crate::chronicle::PINDown>,
    pub interrogated: Option<crate::chronicle::PINDown>,
    pub key_taken: Option<crate::chronicle::PINDown>,
    pub ko: Option<crate::chronicle::PINDown>,
    pub kill: Option<crate::chronicle::PINDown>,
    pub body_found: Option<crate::chronicle::PINDown>,
    pub door_kicked: Option<crate::chronicle::PINDown>,
    pub lock_picked: Option<crate::chronicle::PINDown>,
    pub hit: Option<crate::chronicle::PINDown>,
    pub need_break: Option<crate::chronicle::PINDown>,
    pub civilian_spotted: Option<crate::chronicle::PINDown>,
    pub civilian_reported: Option<crate::chronicle::PINDown>,
    pub civilian_complied: Option<crate::chronicle::PINDown>,
    pub civilian_ko: Option<crate::chronicle::PINDown>,
    pub civilian_killed: Option<crate::chronicle::PINDown>,
    pub camera_spotted: Option<crate::chronicle::PINDown>,
    pub camera_alarm: Option<crate::chronicle::PINDown>,
    pub laser_tripped: Option<crate::chronicle::PINDown>,
    pub device_down: Option<crate::chronicle::PINDown>,
    pub code_learned: Option<crate::chronicle::PINDown>,
    pub code_entered: Option<crate::chronicle::PINDown>,
    pub checkpoint: Option<crate::chronicle::PINDown>,
}
impl PIN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PIN {
            down: None,
            shot: None,
            blackout: None,
            heard: None,
            objective: None,
            grab: None,
            ocp: None,
            thrown: None,
            radio: None,
            spotted: None,
            contact: None,
            lost: None,
            calm: None,
            alarm: None,
            light_shot: None,
            interrogated: None,
            key_taken: None,
            ko: None,
            kill: None,
            body_found: None,
            door_kicked: None,
            lock_picked: None,
            hit: None,
            need_break: None,
            civilian_spotted: None,
            civilian_reported: None,
            civilian_complied: None,
            civilian_ko: None,
            civilian_killed: None,
            camera_spotted: None,
            camera_alarm: None,
            laser_tripped: None,
            device_down: None,
            code_learned: None,
            code_entered: None,
            checkpoint: None,
        }
    }
}
impl Default for PIN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at PIN.down, src/game/chronicle.ts:507)
#[derive(Clone, Debug)]
pub struct PINDown {
    pub glyph: String,
    pub cls: String,
    pub label: String,
}
impl PINDown {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PINDown {
            glyph: String::new(),
            cls: String::new(),
            label: String::new(),
        }
    }
}
impl Default for PINDown { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/chronicle.ts:38 roomsOf ->, src/game/chronicle.ts:38)
#[derive(Clone, Debug)]
pub struct RoomsOfItem {
    pub name: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}
impl RoomsOfItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomsOfItem {
            name: String::new(),
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
        }
    }
}
impl Default for RoomsOfItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum IsChatterDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronicleAddDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronicleRecentKey {
    Text(String),
    Num(f64),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronicleDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronicleNDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: inline at src/game/chronicle.ts:276 fixtureName(tg), src/game/chronicle.ts:276)
#[derive(Clone, Debug)]
pub struct FixtureNameTg {
    pub name: String,
    pub group: String,
}
impl FixtureNameTg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixtureNameTg {
            name: String::new(),
            group: String::new(),
        }
    }
}
impl Default for FixtureNameTg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronDoorKind {
    DoorKicked,
    LockPicked,
}
impl ChronDoorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChronDoorKind::DoorKicked => "doorKicked",
            ChronDoorKind::LockPicked => "lockPicked",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronCheckpointWhat {
    Save,
    Load,
}
impl ChronCheckpointWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChronCheckpointWhat::Save => "save",
            ChronCheckpointWhat::Load => "load",
        }
    }
}

/// (TypeScript: inline at src/game/chronicle.ts:366 stealthScore ->, src/game/chronicle.ts:366)
#[derive(Clone, Debug)]
pub struct StealthScore {
    pub score: f64,
    pub lines: Vec<crate::chronicle::ScoreLine>,
}
impl StealthScore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StealthScore {
            score: 0.0,
            lines: Vec::new(),
        }
    }
}
impl Default for StealthScore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/chronicle.ts:398 ratingOf ->, src/game/chronicle.ts:398)
#[derive(Clone, Debug)]
pub struct RatingOf {
    pub word: String,
    pub blurb: String,
}
impl RatingOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RatingOf {
            word: String::new(),
            blurb: String::new(),
        }
    }
}
impl Default for RatingOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SummaryLineState {
    Down,
    Done,
    Sofar,
}
impl SummaryLineState {
    pub const fn as_str(self) -> &'static str {
        match self {
            SummaryLineState::Down => "down",
            SummaryLineState::Done => "done",
            SummaryLineState::Sofar => "sofar",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ChronicleMod {
    /// (src/game/chronicle.ts:90 CHATTER)
    pub chatter: JsSet<crate::chronicle::ChronKind>,
}
impl ChronicleMod {
    pub fn blank() -> Self {
        ChronicleMod {
            chatter: { let mut s = JsSet::new(); for k in [crate::chronicle::ChronKind::Heard, crate::chronicle::ChronKind::Shot, crate::chronicle::ChronKind::Radio, crate::chronicle::ChronKind::Lost, crate::chronicle::ChronKind::Calm, crate::chronicle::ChronKind::Contact, crate::chronicle::ChronKind::NeedBreak] { s.add(k); } s },
        }
    }
}

/// (src/game/chronicle.ts:27 ROOMS: a table nothing writes)
pub fn rooms() -> Vec<crate::chronicle::ROOMSItem> { alloc::vec![crate::chronicle::ROOMSItem { name: String::from("corridor"), x0: 4.0, x1: 36.0, z0: 10.0, z1: 12.2 }, crate::chronicle::ROOMSItem { name: String::from("conference room"), x0: 4.0, x1: 14.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("server room"), x0: 14.0, x1: 22.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("manager's office"), x0: 22.0, x1: 30.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("storage"), x0: 30.0, x1: 36.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("lobby"), x0: 4.0, x1: 12.0, z0: 12.2, z1: 24.0 }, crate::chronicle::ROOMSItem { name: String::from("cubicles"), x0: 12.0, x1: 28.0, z0: 12.2, z1: 24.0 }, crate::chronicle::ROOMSItem { name: String::from("break room"), x0: 28.0, x1: 36.0, z0: 12.2, z1: 24.0 }] }

/// (src/game/chronicle.ts:89 MAX_EVENTS)
pub const MAX_EVENTS: f64 = 700.0;

/// (src/game/chronicle.ts:89 SOFT_EVENTS)
pub const SOFT_EVENTS: f64 = 520.0;

/// (src/game/chronicle.ts:97 PATH_CAP)
pub const PATH_CAP: f64 = 4000.0;

/// (src/game/chronicle.ts:97 PATH_SECS)
pub const PATH_SECS: f64 = 0.5;

/// (src/game/chronicle.ts:100 HEARD_MERGE_SECS)
pub const HEARD_MERGE_SECS: f64 = 8.0;

/// (src/game/chronicle.ts:100 SHOT_MERGE_SECS)
pub const SHOT_MERGE_SECS: f64 = 2.5;

/// (src/game/chronicle.ts:102 SHOT_NEAR_M)
pub const SHOT_NEAR_M: f64 = 3.0;

/// (src/game/chronicle.ts:220 RANK: a table nothing writes)
pub fn rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("pricks up"), 0.0); t.set(String::from("investigates"), 1.0); t.set(String::from("alerted"), 2.0); t } }

/// (src/game/chronicle.ts:392 BANDS: a table nothing writes)
pub fn bands() -> Vec<crate::chronicle::BANDSItem> { alloc::vec![crate::chronicle::BANDSItem { min: 90.0, word: String::from("GHOST"), blurb: String::from("in and out, and the night none the wiser") }, crate::chronicle::BANDSItem { min: 70.0, word: String::from("SHADOW"), blurb: String::from("they know something happened — not much more") }, crate::chronicle::BANDSItem { min: 40.0, word: String::from("OPERATIVE"), blurb: String::from("the job got done; the floor will be talking about it") }, crate::chronicle::BANDSItem { min: 0.0, word: String::from("ASSAULT"), blurb: String::from("loud, seen, and paid for") }] }

/// (src/game/chronicle.ts:447 STAGE_NAME: a table nothing writes)
pub fn stage_name() -> Vec<String> { alloc::vec![String::from("calm"), String::from("heightened"), String::from("lockdown")] }

/// (src/game/chronicle.ts:448 CAUSE: a table nothing writes)
pub fn cause() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("sight"), String::from("eyes on you")); t.set(String::from("sound"), String::from("heard it")); t.set(String::from("radio"), String::from("over the net")); t.set(String::from("body"), String::from("a body")); t.set(String::from("held"), String::from("a man held")); t.set(String::from("search"), String::from("the search found nobody")); t.set(String::from("missing"), String::from("a man missing")); t.set(String::from("hostage"), String::from("a man held")); t.set(String::from("device"), String::from("security had you on camera")); t.set(String::from("civilian"), String::from("a civilian told them")); t.set(String::from("time"), String::from("the clock ran out")); t } }

/// (src/game/chronicle.ts:450 CIV_CAUSE: a table nothing writes)
pub fn civ_cause() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("sam"), String::from("saw you")); t.set(String::from("gun"), String::from("saw you, gun in hand")); t.set(String::from("body"), String::from("found a body")); t.set(String::from("gunfire"), String::from("heard the shooting")); t.set(String::from("shot"), String::from("heard a muffled shot")); t.set(String::from("bang"), String::from("heard a stun charge")); t.set(String::from("scream"), String::from("heard the screaming")); t.set(String::from("noise"), String::from("a crash beside her")); t.set(String::from("grabbed"), String::from("you had her by the throat")); t.set(String::from("panel"), String::from("panicked")); t } }

/// (src/game/chronicle.ts:452 NEED_WORDS: a table nothing writes)
pub fn need_words() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("coffee"), String::from("a coffee at the break-room counter")); t.set(String::from("smoke"), String::from("a smoke outside the fire exit")); t.set(String::from("water"), String::from("water at the cooler")); t.set(String::from("stretch"), String::from("a stretch round the lobby")); t } }

/// (src/game/chronicle.ts:454 NEED_SHORT: a table nothing writes)
pub fn need_short() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("coffee"), String::from("a coffee")); t.set(String::from("smoke"), String::from("a smoke")); t.set(String::from("water"), String::from("water")); t.set(String::from("stretch"), String::from("a stretch")); t } }

/// (src/game/chronicle.ts:504 INTEL: a table nothing writes)
pub fn intel() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("door"), String::from("the keyed door (and his key)")); t.set(String::from("code"), String::from("a door code")); t.set(String::from("routes"), String::from("a colleague's route")); t.set(String::from("rollcall"), String::from("when the radio check is due")); t.set(String::from("rollcallLive"), String::from("that a radio check is on now")); t.set(String::from("checkOn"), String::from("that a man is out looking for a colleague")); t.set(String::from("clearing"), String::from("which room the pair is clearing")); t.set(String::from("blackout"), String::from("who is on the breaker")); t.set(String::from("lockdownPosts"), String::from("where the lockdown posts are")); t.set(String::from("missing"), String::from("that a man is missed")); t.set(String::from("drive"), String::from("that the rack is being looked at")); t } }

/// (src/game/chronicle.ts:507 PIN: a table nothing writes)
pub fn pin() -> crate::chronicle::PIN { crate::chronicle::PIN { down: Some(crate::chronicle::PINDown { glyph: String::from("✚"), cls: String::from("down"), label: String::from("you went down") }), shot: None, blackout: Some(crate::chronicle::PINDown { glyph: String::from("✦"), cls: String::from("light"), label: String::from("OCP / light") }), heard: None, objective: None, grab: None, ocp: Some(crate::chronicle::PINDown { glyph: String::from("✦"), cls: String::from("light"), label: String::from("OCP / light") }), thrown: None, radio: None, spotted: Some(crate::chronicle::PINDown { glyph: String::from("◉"), cls: String::from("seen"), label: String::from("spotted") }), contact: None, lost: None, calm: None, alarm: Some(crate::chronicle::PINDown { glyph: String::from("!"), cls: String::from("alarm"), label: String::from("alarm up") }), light_shot: Some(crate::chronicle::PINDown { glyph: String::from("✦"), cls: String::from("light"), label: String::from("OCP / light") }), interrogated: None, key_taken: None, ko: Some(crate::chronicle::PINDown { glyph: String::from("z"), cls: String::from("ko"), label: String::from("knockout") }), kill: Some(crate::chronicle::PINDown { glyph: String::from("✕"), cls: String::from("kill"), label: String::from("kill") }), body_found: Some(crate::chronicle::PINDown { glyph: String::from("◇"), cls: String::from("body"), label: String::from("body found") }), door_kicked: Some(crate::chronicle::PINDown { glyph: String::from("▮"), cls: String::from("door"), label: String::from("door kicked / picked") }), lock_picked: Some(crate::chronicle::PINDown { glyph: String::from("▯"), cls: String::from("door"), label: String::from("door kicked / picked") }), hit: None, need_break: None, civilian_spotted: Some(crate::chronicle::PINDown { glyph: String::from("◉"), cls: String::from("seen"), label: String::from("a civilian saw / told") }), civilian_reported: Some(crate::chronicle::PINDown { glyph: String::from("☎"), cls: String::from("alarm"), label: String::from("a civilian saw / told") }), civilian_complied: None, civilian_ko: Some(crate::chronicle::PINDown { glyph: String::from("z"), cls: String::from("ko"), label: String::from("knockout") }), civilian_killed: Some(crate::chronicle::PINDown { glyph: String::from("✕"), cls: String::from("kill"), label: String::from("kill") }), camera_spotted: Some(crate::chronicle::PINDown { glyph: String::from("◎"), cls: String::from("seen"), label: String::from("camera / beam") }), camera_alarm: Some(crate::chronicle::PINDown { glyph: String::from("◎"), cls: String::from("alarm"), label: String::from("camera / beam") }), laser_tripped: Some(crate::chronicle::PINDown { glyph: String::from("≋"), cls: String::from("seen"), label: String::from("camera / beam") }), device_down: None, code_learned: None, code_entered: None, checkpoint: None } }
