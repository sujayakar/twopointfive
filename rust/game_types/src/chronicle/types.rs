//! The types of `chronicle`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, crosses::Crosses, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ChronEvent {
    pub t: f64,
    pub kind: crate::chronicle::ChronKind,
    pub room: String,
    pub pos: V3,
    pub data: JsMap<String, Option<crate::chronicle::ChronEventDataValue>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words, Crosses)]
pub enum ChronKind {
    #[default] Down,
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

#[derive(Clone, Debug, ToBytes)]
pub enum ChronEventDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ChronCounts {
    pub seen_up: f64,
    pub esc1: f64,
    pub esc2: f64,
    pub shots_near_men: f64,
    pub lights_shot: f64,
    pub ocps: f64,
    pub kicked: f64,
    pub picked: f64,
    pub intel: f64,
    pub thrown: f64,
    pub grabs: f64,
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

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Chronicle {
    pub events: Vec<crate::chronicle::ChronEvent>,
    pub dropped: f64,
    #[is(crate::chronicle::ChronCounts { seen_up: 0.0, esc1: 0.0, esc2: 0.0, shots_near_men: 0.0, lights_shot: 0.0, ocps: 0.0, kicked: 0.0, picked: 0.0, intel: 0.0, thrown: 0.0, grabs: 0.0, civ_seen: 0.0, civ_reported: 0.0, civ_complied: 0.0, civ_ko: 0.0, civ_killed: 0.0, devices: 0.0, hits: 0.0, down: false, down_by: String::from(""), saves: 0.0, loads: 0.0 })] pub counts: crate::chronicle::ChronCounts,
    pub questioned: JsSet<String>,
    #[is(alloc::vec![0.0; 12000])] pub path: Vec<f32>,
    pub path_n: f64,
    #[is(0.5)] pub path_every: f64,
    pub path_next: f64,
    pub heard_of: JsMap<u32, usize>,
    pub heard_no_row: JsSet<u32>,
}

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

#[derive(Clone, Debug, ToBytes, Default, Crosses)]
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

#[derive(Clone, Debug, ToBytes, Default, Crosses)]
pub struct ScoreLine {
    pub label: String,
    pub pts: f64,
}

#[derive(Clone, Debug, ToBytes, Default, Crosses)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub on: bool,
    pub hint: String,
}

#[derive(Clone, Debug, ToBytes, Default, Crosses)]
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

#[derive(Clone, Debug, Default)]
pub struct ROOMSItem {
    pub name: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

#[derive(Clone, Debug, Default)]
pub struct BANDSItem {
    pub min: f64,
    pub word: String,
    pub blurb: String,
}

#[derive(Clone, Debug)]
pub enum ChronicleRecentKey {
    Text(String),
    Num(f64),
}

#[derive(Clone, Debug)]
pub enum ChronicleDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub enum ChronicleNDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

#[derive(Clone, Debug, Default)]
pub struct FixtureNameTg {
    pub name: String,
    pub group: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronDoorKind {
    DoorKicked,
    LockPicked,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum ChronCheckpointWhat {
    Save,
    Load,
}

#[derive(Clone, Debug, Default)]
pub struct StealthScore {
    pub score: f64,
    pub lines: Vec<crate::chronicle::ScoreLine>,
}

#[derive(Clone, Debug, Default)]
pub struct RatingOf {
    pub word: String,
    pub blurb: String,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ChronicleMod {
    #[is({ let mut s = JsSet::new(); for k in [crate::chronicle::ChronKind::Heard, crate::chronicle::ChronKind::Shot, crate::chronicle::ChronKind::Radio, crate::chronicle::ChronKind::Lost, crate::chronicle::ChronKind::Calm, crate::chronicle::ChronKind::Contact, crate::chronicle::ChronKind::NeedBreak] { s.add(k); } s })] pub chatter: JsSet<crate::chronicle::ChronKind>,
}

pub fn rooms() -> Vec<crate::chronicle::ROOMSItem> { alloc::vec![crate::chronicle::ROOMSItem { name: String::from("corridor"), x0: 4.0, x1: 36.0, z0: 10.0, z1: 12.2 }, crate::chronicle::ROOMSItem { name: String::from("conference room"), x0: 4.0, x1: 14.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("server room"), x0: 14.0, x1: 22.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("manager's office"), x0: 22.0, x1: 30.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("storage"), x0: 30.0, x1: 36.0, z0: 4.0, z1: 10.0 }, crate::chronicle::ROOMSItem { name: String::from("lobby"), x0: 4.0, x1: 12.0, z0: 12.2, z1: 24.0 }, crate::chronicle::ROOMSItem { name: String::from("cubicles"), x0: 12.0, x1: 28.0, z0: 12.2, z1: 24.0 }, crate::chronicle::ROOMSItem { name: String::from("break room"), x0: 28.0, x1: 36.0, z0: 12.2, z1: 24.0 }] }

pub const MAX_EVENTS: f64 = 700.0;

pub const SOFT_EVENTS: f64 = 520.0;

pub const PATH_CAP: f64 = 4000.0;

pub const HEARD_MERGE_SECS: f64 = 8.0;

pub const SHOT_MERGE_SECS: f64 = 2.5;

pub const SHOT_NEAR_M: f64 = 3.0;

pub fn rank() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("pricks up"), 0.0); t.set(String::from("investigates"), 1.0); t.set(String::from("alerted"), 2.0); t } }

pub fn bands() -> Vec<crate::chronicle::BANDSItem> { alloc::vec![crate::chronicle::BANDSItem { min: 90.0, word: String::from("GHOST"), blurb: String::from("in and out, and the night none the wiser") }, crate::chronicle::BANDSItem { min: 70.0, word: String::from("SHADOW"), blurb: String::from("they know something happened — not much more") }, crate::chronicle::BANDSItem { min: 40.0, word: String::from("OPERATIVE"), blurb: String::from("the job got done; the floor will be talking about it") }, crate::chronicle::BANDSItem { min: 0.0, word: String::from("ASSAULT"), blurb: String::from("loud, seen, and paid for") }] }

pub fn stage_name() -> Vec<String> { alloc::vec![String::from("calm"), String::from("heightened"), String::from("lockdown")] }

pub fn cause() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("sight"), String::from("eyes on you")); t.set(String::from("sound"), String::from("heard it")); t.set(String::from("radio"), String::from("over the net")); t.set(String::from("body"), String::from("a body")); t.set(String::from("held"), String::from("a man held")); t.set(String::from("search"), String::from("the search found nobody")); t.set(String::from("missing"), String::from("a man missing")); t.set(String::from("hostage"), String::from("a man held")); t.set(String::from("device"), String::from("security had you on camera")); t.set(String::from("civilian"), String::from("a civilian told them")); t.set(String::from("time"), String::from("the clock ran out")); t } }

pub fn civ_cause() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("sam"), String::from("saw you")); t.set(String::from("gun"), String::from("saw you, gun in hand")); t.set(String::from("body"), String::from("found a body")); t.set(String::from("gunfire"), String::from("heard the shooting")); t.set(String::from("shot"), String::from("heard a muffled shot")); t.set(String::from("bang"), String::from("heard a stun charge")); t.set(String::from("scream"), String::from("heard the screaming")); t.set(String::from("noise"), String::from("a crash beside her")); t.set(String::from("grabbed"), String::from("you had her by the throat")); t.set(String::from("panel"), String::from("panicked")); t } }

pub fn need_words() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("coffee"), String::from("a coffee at the break-room counter")); t.set(String::from("smoke"), String::from("a smoke outside the fire exit")); t.set(String::from("water"), String::from("water at the cooler")); t.set(String::from("stretch"), String::from("a stretch round the lobby")); t } }

pub fn need_short() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("coffee"), String::from("a coffee")); t.set(String::from("smoke"), String::from("a smoke")); t.set(String::from("water"), String::from("water")); t.set(String::from("stretch"), String::from("a stretch")); t } }

pub fn intel() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("door"), String::from("the keyed door (and his key)")); t.set(String::from("code"), String::from("a door code")); t.set(String::from("routes"), String::from("a colleague's route")); t.set(String::from("rollcall"), String::from("when the radio check is due")); t.set(String::from("rollcallLive"), String::from("that a radio check is on now")); t.set(String::from("checkOn"), String::from("that a man is out looking for a colleague")); t.set(String::from("clearing"), String::from("which room the pair is clearing")); t.set(String::from("blackout"), String::from("who is on the breaker")); t.set(String::from("lockdownPosts"), String::from("where the lockdown posts are")); t.set(String::from("missing"), String::from("that a man is missed")); t.set(String::from("drive"), String::from("that the rack is being looked at")); t } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words, Crosses)]
pub enum DebriefState {
    #[default] Down,
    Done,
    Sofar,
}
