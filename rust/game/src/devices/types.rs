//! The types of `devices`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/devices.ts:39 CameraDef, src/scene/level.ts:129)
#[derive(Clone, Debug)]
pub struct CameraDef2 {
    pub kind: crate::devices::CameraDef2Kind,
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub pitch: Option<f64>,
    pub sweep: Option<[f64; 2]>,
    pub sweep_period: Option<f64>,
    pub fov_deg: Option<f64>,
    pub range: Option<f64>,
    pub group: Option<String>,
    pub mains: Option<bool>,
}
impl CameraDef2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CameraDef2 {
            kind: crate::devices::CameraDef2Kind::Camera,
            name: String::new(),
            pos: [0.0; 3],
            yaw: 0.0,
            pitch: None,
            sweep: None,
            sweep_period: None,
            fov_deg: None,
            range: None,
            group: None,
            mains: None,
        }
    }
}
impl Default for CameraDef2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CameraDef2Kind {
    Camera,
}
impl CameraDef2Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CameraDef2Kind::Camera => "camera",
        }
    }
}

/// (TypeScript: inline at src/game/devices.ts:40 KeypadDef, src/scene/level.ts:130)
#[derive(Clone, Debug)]
pub struct KeypadDef2 {
    pub kind: crate::devices::KeypadDef2Kind,
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub door: String,
    pub code: String,
}
impl KeypadDef2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeypadDef2 {
            kind: crate::devices::KeypadDef2Kind::Keypad,
            name: String::new(),
            pos: [0.0; 3],
            yaw: 0.0,
            door: String::new(),
            code: String::new(),
        }
    }
}
impl Default for KeypadDef2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KeypadDef2Kind {
    Keypad,
}
impl KeypadDef2Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            KeypadDef2Kind::Keypad => "keypad",
        }
    }
}

/// (TypeScript: inline at src/game/devices.ts:41 LaserDef, src/scene/level.ts:131)
#[derive(Clone, Debug)]
pub struct LaserDef2 {
    pub kind: crate::devices::LaserDef2Kind,
    pub name: String,
    pub from: V3,
    pub to: V3,
    pub group: Option<String>,
    pub mains: Option<bool>,
    pub alarm: Option<crate::devices::LaserDef2Alarm>,
}
impl LaserDef2 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LaserDef2 {
            kind: crate::devices::LaserDef2Kind::Laser,
            name: String::new(),
            from: [0.0; 3],
            to: [0.0; 3],
            group: None,
            mains: None,
            alarm: None,
        }
    }
}
impl Default for LaserDef2 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaserDef2Kind {
    Laser,
}
impl LaserDef2Kind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaserDef2Kind::Laser => "laser",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LaserDef2Alarm {
    Alarm,
    Suspicion,
}
impl LaserDef2Alarm {
    pub const fn as_str(self) -> &'static str {
        match self {
            LaserDef2Alarm::Alarm => "alarm",
            LaserDef2Alarm::Suspicion => "suspicion",
        }
    }
}

/// (TypeScript: CameraRt, src/game/devices.ts:61)
#[derive(Clone, Debug)]
pub struct CameraRt {
    pub smoke_trans: f64,
    pub def: crate::devices::CameraDef2,
    pub index: f64,
    pub target: FixtureId,
    pub cone: Option<LightId>,
    pub smoke_key: String,
    pub slot: f64,
    pub st: crate::Opaque,
}
impl CameraRt {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        CameraRt {
            smoke_trans: 1.0,
            def: crate::devices::CameraDef2::zeroed(),
            index: 0.0,
            target: FixtureId(0),
            cone: None,
            smoke_key: String::new(),
            slot: 0.0,
            st: crate::Opaque,
        }
    }
}
impl Default for CameraRt { fn default() -> Self { Self::blank() } }

/// (TypeScript: LaserRt, src/game/devices.ts:67)
#[derive(Clone, Debug)]
pub struct LaserRt {
    pub def: crate::devices::LaserDef2,
    pub index: f64,
    pub target: FixtureId,
    pub dir: V3,
    pub len: f64,
    pub slot: f64,
    pub st: crate::Opaque,
}
impl LaserRt {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        LaserRt {
            def: crate::devices::LaserDef2::zeroed(),
            index: 0.0,
            target: FixtureId(0),
            dir: [0.0; 3],
            len: 0.0,
            slot: 0.0,
            st: crate::Opaque,
        }
    }
}
impl Default for LaserRt { fn default() -> Self { Self::blank() } }

/// (TypeScript: KeypadRt, src/game/devices.ts:70)
#[derive(Clone, Debug)]
pub struct KeypadRt {
    pub def: crate::devices::KeypadDef2,
    pub door: DoorId,
    pub n: V3,
}
impl KeypadRt {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KeypadRt {
            def: crate::devices::KeypadDef2::zeroed(),
            door: DoorId(0),
            n: [0.0; 3],
        }
    }
}
impl Default for KeypadRt { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: DeviceLogRow, src/game/devices.ts:72)
#[derive(Clone, Debug)]
pub struct DeviceLogRow {
    pub seq: f64,
    pub t: f64,
    pub device: String,
    pub what: crate::devices::DeviceLogRowWhat,
    pub where_: String,
    pub sent: Option<String>,
}
impl DeviceLogRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DeviceLogRow {
            seq: 0.0,
            t: 0.0,
            device: String::new(),
            what: crate::devices::DeviceLogRowWhat::Dead,
            where_: String::new(),
            sent: None,
        }
    }
}
impl Default for DeviceLogRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DeviceLogRowWhat {
    Dead,
    Code,
    Alarm,
    Trip,
    Suspicion,
    Deny,
    Overheard,
}
impl DeviceLogRowWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DeviceLogRowWhat::Dead => "dead",
            DeviceLogRowWhat::Code => "code",
            DeviceLogRowWhat::Alarm => "alarm",
            DeviceLogRowWhat::Trip => "trip",
            DeviceLogRowWhat::Suspicion => "suspicion",
            DeviceLogRowWhat::Deny => "deny",
            DeviceLogRowWhat::Overheard => "overheard",
        }
    }
}

/// (TypeScript: DevicesSnap, src/game/devices.ts:74)
#[derive(Clone, Debug)]
pub struct DevicesSnap {
    pub cameras: Vec<crate::devices::DevicesSnapCamerasItem>,
    pub lasers: Vec<crate::devices::DevicesSnapLasersItem>,
    pub log: Vec<crate::devices::DeviceLogRow>,
    pub radio_count: f64,
    pub seq: f64,
}
impl DevicesSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DevicesSnap {
            cameras: Vec::new(),
            lasers: Vec::new(),
            log: Vec::new(),
            radio_count: 0.0,
            seq: 0.0,
        }
    }
}
impl Default for DevicesSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:74 DevicesSnap.cameras, src/game/devices.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct DevicesSnapCamerasItem {
    pub yaw: f64,
    pub phase: f64,
    pub seen: f64,
    pub state: crate::sim_lightshome::CameraState,
    pub reported_at: f64,
    pub last_seen: Option<V3>,
    pub dead_called: bool,
}
impl DevicesSnapCamerasItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DevicesSnapCamerasItem {
            yaw: 0.0,
            phase: 0.0,
            seen: 0.0,
            state: crate::sim_lightshome::CameraState::Alarm,
            reported_at: 0.0,
            last_seen: None,
            dead_called: false,
        }
    }
}
impl Default for DevicesSnapCamerasItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:74 DevicesSnap.lasers, src/game/devices.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct DevicesSnapLasersItem {
    pub last_trip_at: f64,
    pub trips: f64,
    pub dead_called: bool,
    pub held: bool,
}
impl DevicesSnapLasersItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DevicesSnapLasersItem {
            last_trip_at: 0.0,
            trips: 0.0,
            dead_called: false,
            held: false,
        }
    }
}
impl Default for DevicesSnapLasersItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Devices, src/game/devices.ts:76)
#[derive(Clone, Debug)]
pub struct Devices {
    pub cameras: Vec<CameraRtId>,
    pub lasers: Vec<LaserRtId>,
    pub keypads: Vec<crate::devices::KeypadRt>,
    pub pad_by_door: JsMap<DoorId, crate::devices::KeypadRt>,
    pub log: Vec<crate::devices::DeviceLogRow>,
    pub seq: f64,
    pub radio_count: f64,
}
impl Devices {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Devices {
            cameras: Vec::new(),
            lasers: Vec::new(),
            keypads: Vec::new(),
            pad_by_door: JsMap::new(),
            log: Vec::new(),
            seq: 0.0,
            radio_count: 0.0,
        }
    }
}
impl Default for Devices { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/devices.ts:47 CONE, src/game/devices.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct CONE {
    pub intensity: f64,
    pub color: V3,
    pub range_k: f64,
    pub volumetric: f64,
    pub outer_max_deg: f64,
    pub radius: f64,
}
impl CONE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CONE {
            intensity: 0.0,
            color: [0.0; 3],
            range_k: 0.0,
            volumetric: 0.0,
            outer_max_deg: 0.0,
            radius: 0.0,
        }
    }
}
impl Default for CONE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:57 ERRAND, src/game/devices.ts:57)
#[derive(Clone, Copy, Debug)]
pub struct ERRAND {
    pub awareness: f64,
    pub react_t: f64,
}
impl ERRAND {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ERRAND {
            awareness: 0.0,
            react_t: 0.0,
        }
    }
}
impl Default for ERRAND { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:406 DEVICE_TUNE_DEFAULTS, src/game/devices.ts:406)
#[derive(Clone, Copy, Debug)]
pub struct DEVICETUNEDEFAULTS {
    pub devices: bool,
    pub lasers: bool,
    pub camera_suspicion_secs: f64,
    pub camera_alarm_secs: f64,
    pub device_cones: bool,
}
impl DEVICETUNEDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DEVICETUNEDEFAULTS {
            devices: false,
            lasers: false,
            camera_suspicion_secs: 0.0,
            camera_alarm_secs: 0.0,
            device_cones: false,
        }
    }
}
impl Default for DEVICETUNEDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:83 note(r), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1604)
#[derive(Clone, Debug)]
pub struct DevicesNoteR {
    pub device: String,
    pub what: crate::devices::DevicesNoteRWhat,
    pub sent: Option<String>,
    pub where_: String,
}
impl DevicesNoteR {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DevicesNoteR {
            device: String::new(),
            what: crate::devices::DevicesNoteRWhat::Dead,
            sent: None,
            where_: String::new(),
        }
    }
}
impl Default for DevicesNoteR { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DevicesNoteRWhat {
    Dead,
    Code,
    Alarm,
    Trip,
    Suspicion,
    Deny,
    Overheard,
}
impl DevicesNoteRWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            DevicesNoteRWhat::Dead => "dead",
            DevicesNoteRWhat::Code => "code",
            DevicesNoteRWhat::Alarm => "alarm",
            DevicesNoteRWhat::Trip => "trip",
            DevicesNoteRWhat::Suspicion => "suspicion",
            DevicesNoteRWhat::Deny => "deny",
            DevicesNoteRWhat::Overheard => "overheard",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronDeviceKind {
    CameraSpotted,
    CameraAlarm,
    LaserTripped,
    DeviceDown,
    CodeLearned,
    CodeEntered,
}
impl ChronDeviceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ChronDeviceKind::CameraSpotted => "cameraSpotted",
            ChronDeviceKind::CameraAlarm => "cameraAlarm",
            ChronDeviceKind::LaserTripped => "laserTripped",
            ChronDeviceKind::DeviceDown => "deviceDown",
            ChronDeviceKind::CodeLearned => "codeLearned",
            ChronDeviceKind::CodeEntered => "codeEntered",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum ChronDeviceDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LearnCodeHow {
    Told,
    Overheard,
}
impl LearnCodeHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            LearnCodeHow::Told => "told",
            LearnCodeHow::Overheard => "overheard",
        }
    }
}

/// (TypeScript: inline at src/game/devices.ts:318 padState ->, src/game/devices.ts:318)
#[derive(Clone, Debug)]
pub struct PadState {
    pub pad: crate::devices::KeypadRt,
    pub known: bool,
}
impl PadState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PadState {
            pad: crate::devices::KeypadRt::zeroed(),
            known: false,
        }
    }
}
impl Default for PadState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/devices.ts:419 segSegClosest ->, src/game/devices.ts:419)
#[derive(Clone, Copy, Debug)]
pub struct SegSegClosest {
    pub dist: f64,
    pub on_b: V3,
}
impl SegSegClosest {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SegSegClosest {
            dist: 0.0,
            on_b: [0.0; 3],
        }
    }
}
impl Default for SegSegClosest { fn default() -> Self { Self::zeroed() } }

/// (src/game/devices.ts:45 LED_IDLE: a table nothing writes)
pub fn led_idle() -> V3 { [1.6, 0.08, 0.05] }

/// (src/game/devices.ts:45 LED_HOT: a table nothing writes)
pub fn led_hot() -> V3 { [14.0, 0.7, 0.3] }

/// (src/game/devices.ts:47 CONE: a table nothing writes)
pub fn cone() -> crate::devices::CONE { crate::devices::CONE { intensity: 1.1, color: [1.0, 0.12, 0.07], range_k: 0.7, volumetric: 1.0, outer_max_deg: 24.0, radius: 0.12 } }

/// (src/game/devices.ts:49 BEAM_MARGIN)
pub const BEAM_MARGIN: f64 = 0.02;

/// (src/game/devices.ts:51 CODE_SECS)
pub const CODE_SECS: f64 = 1.2;

/// (src/game/devices.ts:52 PAD_REACH)
pub const PAD_REACH: f64 = 1.35;

/// (src/game/devices.ts:52 DENY_LEVEL)
pub const DENY_LEVEL: f64 = 0.05;

/// (src/game/devices.ts:57 ERRAND: a table nothing writes)
pub fn errand() -> crate::devices::ERRAND { crate::devices::ERRAND { awareness: 0.42, react_t: 1.0 } }

/// (src/game/devices.ts:364 CAM_COL: a table nothing writes)
pub fn cam_col() -> V3 { [0.16, 0.17, 0.19] }

/// (src/game/devices.ts:364 CAM_DARK: a table nothing writes)
pub fn cam_dark() -> V3 { [0.05, 0.05, 0.06] }

/// (src/game/devices.ts:364 PAD_COL: a table nothing writes)
pub fn pad_col() -> V3 { [0.22, 0.23, 0.25] }

/// (src/game/devices.ts:364 EMIT_COL: a table nothing writes)
pub fn emit_col() -> V3 { [0.14, 0.14, 0.16] }

/// (src/game/devices.ts:365 GREEN: a table nothing writes)
pub fn green() -> V3 { [0.3, 5.0, 0.6] }

/// (src/game/devices.ts:365 RED: a table nothing writes)
pub fn red() -> V3 { [5.0, 0.3, 0.15] }

/// (src/game/devices.ts:365 AMBER: a table nothing writes)
pub fn amber() -> V3 { [5.0, 2.4, 0.2] }

/// (src/game/devices.ts:405 LS_KEY)
pub const LS_KEY: &str = "twopointsix.devices";

/// (src/game/devices.ts:406 DEVICE_TUNE_DEFAULTS: a table nothing writes)
pub fn device_tune_defaults() -> crate::devices::DEVICETUNEDEFAULTS { crate::devices::DEVICETUNEDEFAULTS { devices: true, lasers: true, camera_suspicion_secs: 0.8, camera_alarm_secs: 2.0, device_cones: true } }
