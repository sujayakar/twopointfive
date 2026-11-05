//! The types of `journal`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: SessionRecorder, src/game/journal.ts:701)
#[derive(Clone, Debug)]
pub struct SessionRecorder {
    pub state: crate::journal::JournalState,
    pub why: String,
    pub frame: f64,
    pub opts: crate::journal::RecordOpts,
    pub pending_left: f64,
    pub save: Option<crate::savestate::SaveState>,
    pub env: Option<crate::journal::EnvSnap>,
    pub knobs: Option<crate::journal::KnobsSnap>,
    pub seed: f64,
    pub t0: f64,
    pub started_at: String,
    pub die: Option<crate::FnField>,
    pub dts: Vec<[f64; 2]>,
    pub views: Vec<Vec<f64>>,
    pub cams: Vec<Vec<f64>>,
    pub inputs: Vec<(f64, crate::journal::InputRec)>,
    pub digests: Vec<(f64, JsMap<String, crate::Opaque>)>,
    pub last_view: Vec<f64>,
    pub last_cam: Vec<f64>,
    pub last_keys: String,
    pub last_buttons: f64,
    pub last_mouse: Vec<f64>,
    pub in_update: bool,
    pub slots: Option<crate::journal::SessionRecorderSlots>,
    pub slot_own: JsSet<String>,
    pub serial: f64,
    pub bytes: f64,
    pub stop_after_next: Option<String>,
    pub last: Option<crate::journal::SessionFile>,
    pub on_stop: Option<crate::FnField>,
}
impl SessionRecorder {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SessionRecorder {
            state: crate::journal::JournalState::Idle,
            why: String::from(""),
            frame: 0.0,
            opts: crate::journal::RecordOpts { seed: None, note: None, commit: None, user_agent: None, hud: None },
            pending_left: 0.0,
            save: None,
            env: None,
            knobs: None,
            seed: 0.0,
            t0: 0.0,
            started_at: String::from(""),
            die: None,
            dts: Vec::new(),
            views: Vec::new(),
            cams: Vec::new(),
            inputs: Vec::new(),
            digests: Vec::new(),
            last_view: alloc::vec![f64::NAN, f64::NAN, f64::NAN, f64::NAN],
            last_cam: alloc::vec![f64::NAN; 8],
            last_keys: String::from("\n"),
            last_buttons: f64::NAN,
            last_mouse: alloc::vec![f64::NAN, f64::NAN],
            in_update: false,
            slots: None,
            slot_own: JsSet::new(),
            serial: 0.0,
            bytes: 0.0,
            stop_after_next: None,
            last: None,
            on_stop: None,
        }
    }
}
impl Default for SessionRecorder { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum JournalState {
    Idle,
    Pending,
    Recording,
}
impl JournalState {
    pub const fn as_str(self) -> &'static str {
        match self {
            JournalState::Idle => "idle",
            JournalState::Pending => "pending",
            JournalState::Recording => "recording",
        }
    }
}

/// (TypeScript: RecordOpts, src/game/journal.ts:698)
#[derive(Clone, Debug)]
pub struct RecordOpts {
    pub seed: Option<f64>,
    pub note: Option<String>,
    pub commit: Option<String>,
    pub user_agent: Option<String>,
    pub hud: Option<bool>,
}
impl RecordOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RecordOpts {
            seed: None,
            note: None,
            commit: None,
            user_agent: None,
            hud: None,
        }
    }
}
impl Default for RecordOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: EnvSnap, src/game/journal.ts:212)
#[derive(Clone, Debug)]
pub struct EnvSnap {
    pub debris_cap: f64,
    pub emissive_scale: f64,
    pub flashlight_radius: f64,
    pub markings: bool,
    pub particles_on: bool,
    pub hud: bool,
    pub glass_fracture: Option<bool>,
    pub cut_boxes: Option<bool>,
    pub canister: Option<crate::journal::EnvSnapCanister>,
    pub stagger: JsMap<String, f64>,
    pub destruction: Option<crate::journal::EnvSnapDestruction>,
    pub lived: Option<bool>,
}
impl EnvSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EnvSnap {
            debris_cap: 0.0,
            emissive_scale: 0.0,
            flashlight_radius: 0.0,
            markings: false,
            particles_on: false,
            hud: false,
            glass_fracture: None,
            cut_boxes: None,
            canister: None,
            stagger: JsMap::new(),
            destruction: None,
            lived: None,
        }
    }
}
impl Default for EnvSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/journal.ts:219 EnvSnap.canister, src/game/journal.ts:219)
#[derive(Clone, Copy, Debug)]
pub struct EnvSnapCanister {
    pub canister_duration: f64,
    pub canister_density: f64,
    pub canister_radius: f64,
    pub canister_temp: f64,
    pub canister_speed: f64,
}
impl EnvSnapCanister {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EnvSnapCanister {
            canister_duration: 0.0,
            canister_density: 0.0,
            canister_radius: 0.0,
            canister_temp: 0.0,
            canister_speed: 0.0,
        }
    }
}
impl Default for EnvSnapCanister { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/journal.ts:221 EnvSnap.destruction, src/game/journal.ts:221)
#[derive(Clone, Copy, Debug)]
pub struct EnvSnapDestruction {
    pub on: bool,
}
impl EnvSnapDestruction {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EnvSnapDestruction {
            on: false,
        }
    }
}
impl Default for EnvSnapDestruction { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KnobsSnap, src/game/journal.ts:224)
#[derive(Clone, Debug)]
pub struct KnobsSnap {
    pub tune: JsMap<String, crate::Opaque>,
    pub flags: crate::journal::KnobsSnapFlags,
    pub tables: JsMap<String, crate::Opaque>,
}
impl KnobsSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnobsSnap {
            tune: JsMap::new(),
            flags: crate::journal::KnobsSnapFlags::zeroed(),
            tables: JsMap::new(),
        }
    }
}
impl Default for KnobsSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/journal.ts:224 KnobsSnap.flags, src/game/journal.ts:224)
#[derive(Clone, Copy, Debug)]
pub struct KnobsSnapFlags {
    pub god_mode: bool,
    pub ai_enabled: bool,
    pub infinite_ammo: bool,
    pub player_invisible: bool,
    pub fn2000: bool,
    pub autosave: bool,
    pub fog: Option<bool>,
    pub stalk_ambient: Option<bool>,
    pub round_cap: Option<f64>,
    pub behind_sam: Option<bool>,
    pub toss_mate: Option<bool>,
}
impl KnobsSnapFlags {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnobsSnapFlags {
            god_mode: false,
            ai_enabled: false,
            infinite_ammo: false,
            player_invisible: false,
            fn2000: false,
            autosave: false,
            fog: None,
            stalk_ambient: None,
            round_cap: None,
            behind_sam: None,
            toss_mate: None,
        }
    }
}
impl Default for KnobsSnapFlags { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: InputRec, src/game/journal.ts:210)
#[derive(Clone, Debug)]
pub struct InputRec {
    pub k: Option<Vec<String>>,
    pub p: Option<Vec<String>>,
    pub c: Option<f64>,
    pub b: Option<f64>,
    pub m: Option<[f64; 2]>,
    pub w: Option<f64>,
}
impl InputRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        InputRec {
            k: None,
            p: None,
            c: None,
            b: None,
            m: None,
            w: None,
        }
    }
}
impl Default for InputRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/journal.ts:712 SessionRecorder.slots, src/game/journal.ts:712 SessionRecorder.slots)
#[derive(Clone, Debug)]
pub struct SessionRecorderSlots {
    pub index: Option<String>,
    pub last_auto: Option<crate::journal::SessionRecorderSlotsLastAuto>,
    pub loads: Vec<(f64, String, String)>,
}
impl SessionRecorderSlots {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SessionRecorderSlots {
            index: None,
            last_auto: None,
            loads: Vec::new(),
        }
    }
}
impl Default for SessionRecorderSlots { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/savestate.ts:873 SessionRecorderSlots.lastAuto, src/game/savestate.ts:873)
#[derive(Clone, Debug)]
pub struct SessionRecorderSlotsLastAuto {
    pub key: String,
    pub t: f64,
}
impl SessionRecorderSlotsLastAuto {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SessionRecorderSlotsLastAuto {
            key: String::new(),
            t: 0.0,
        }
    }
}
impl Default for SessionRecorderSlotsLastAuto { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SessionFile, src/game/journal.ts:226)
#[derive(Clone, Debug)]
pub struct SessionFile {
    pub v: f64,
    pub game: crate::journal::SessionFileGame,
    pub recorded_at: String,
    pub commit: Option<String>,
    pub note: Option<String>,
    pub seed: f64,
    pub level: crate::savestate::SaveStateLevel,
    pub save: crate::savestate::SaveState,
    pub env: crate::journal::EnvSnap,
    pub knobs: crate::journal::KnobsSnap,
    pub frames: f64,
    pub dt: Vec<[f64; 2]>,
    pub view: Vec<Vec<f64>>,
    pub cam: Vec<Vec<f64>>,
    pub input: Vec<(f64, crate::journal::InputRec)>,
    pub sensors: crate::sensors::SensorJournal,
    pub slots: Option<crate::journal::SessionFileSlots>,
    pub digests: Vec<(f64, JsMap<String, crate::Opaque>)>,
    pub meta: crate::journal::SessionFileMeta,
}
impl SessionFile {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SessionFile {
            v: 0.0,
            game: crate::journal::SessionFileGame::Twopointsix,
            recorded_at: String::new(),
            commit: None,
            note: None,
            seed: 0.0,
            level: crate::savestate::SaveStateLevel::zeroed(),
            save: crate::savestate::SaveState::zeroed(),
            env: crate::journal::EnvSnap::zeroed(),
            knobs: crate::journal::KnobsSnap::zeroed(),
            frames: 0.0,
            dt: Vec::new(),
            view: Vec::new(),
            cam: Vec::new(),
            input: Vec::new(),
            sensors: crate::sensors::SensorJournal::zeroed(),
            slots: None,
            digests: Vec::new(),
            meta: crate::journal::SessionFileMeta::zeroed(),
        }
    }
}
impl Default for SessionFile { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SessionFileGame {
    Twopointsix,
}
impl SessionFileGame {
    pub const fn as_str(self) -> &'static str {
        match self {
            SessionFileGame::Twopointsix => "twopointsix",
        }
    }
}

/// (TypeScript: inline at src/game/journal.ts:241 SessionFile.slots, src/game/journal.ts:241 SessionFile.slots)
#[derive(Clone, Debug)]
pub struct SessionFileSlots {
    pub index: Option<String>,
    pub last_auto: Option<crate::journal::SessionRecorderSlotsLastAuto>,
    pub loads: Vec<(f64, String, String)>,
}
impl SessionFileSlots {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SessionFileSlots {
            index: None,
            last_auto: None,
            loads: Vec::new(),
        }
    }
}
impl Default for SessionFileSlots { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/journal.ts:244 SessionFile.meta, src/game/journal.ts:244)
#[derive(Clone, Debug)]
pub struct SessionFileMeta {
    pub seconds: f64,
    pub stopped: String,
    pub bytes_per_frame: Option<f64>,
    pub user_agent: Option<String>,
    pub between: Option<bool>,
    pub lost: Option<bool>,
}
impl SessionFileMeta {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SessionFileMeta {
            seconds: 0.0,
            stopped: String::new(),
            bytes_per_frame: None,
            user_agent: None,
            between: None,
            lost: None,
        }
    }
}
impl Default for SessionFileMeta { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: View, src/game/journal.ts:247)
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub client_width: f64,
    pub client_height: f64,
    pub width: f64,
    pub height: f64,
}
impl View {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        View {
            client_width: 0.0,
            client_height: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}
impl Default for View { fn default() -> Self { Self::zeroed() } }

/// (src/game/journal.ts:187 JOURNAL_VERSION)
pub const JOURNAL_VERSION: f64 = 1.0;

/// (src/game/journal.ts:191 JOURNAL_VERSION_SLOTS)
pub const JOURNAL_VERSION_SLOTS: f64 = 2.0;

/// (src/game/journal.ts:193 JOURNAL_DIGEST_EVERY)
pub const JOURNAL_DIGEST_EVERY: f64 = 120.0;

/// (src/game/journal.ts:195 JOURNAL_PENDING_FRAMES)
pub const JOURNAL_PENDING_FRAMES: f64 = 480.0;

/// (src/game/journal.ts:197 JOURNAL_MAX_FRAMES)
pub const JOURNAL_MAX_FRAMES: f64 = 72000.0;
