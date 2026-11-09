//! The types of `devices`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CameraDef2 {
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

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KeypadDef2 {
    pub name: String,
    pub pos: V3,
    pub yaw: f64,
    pub door: String,
    pub code: String,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LaserDef2 {
    pub name: String,
    pub from: V3,
    pub to: V3,
    pub group: Option<String>,
    pub mains: Option<bool>,
    pub alarm: Option<crate::devices::LaserDef2Alarm>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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

#[derive(Clone, Debug, ToBytes)]
pub struct CameraRt {
    pub def: crate::devices::CameraDef2,
    pub index: f64,
    pub target: FixtureId,
    pub cone: LightId,
    pub slot: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub st: crate::Opaque,
}
impl CameraRt {
    /// what a new one holds
    pub fn blank() -> Self {
        CameraRt {
            def: crate::devices::CameraDef2::default(),
            index: 0.0,
            target: FixtureId(0),
            cone: LightId(0),
            slot: 0.0,
            st: crate::Opaque,
        }
    }
}
impl Default for CameraRt { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes)]
pub struct LaserRt {
    pub def: crate::devices::LaserDef2,
    pub target: FixtureId,
    pub dir: V3,
    pub len: f64,
    pub slot: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub st: crate::Opaque,
}
impl LaserRt {
    /// what a new one holds
    pub fn blank() -> Self {
        LaserRt {
            def: crate::devices::LaserDef2::default(),
            target: FixtureId(0),
            dir: [0.0; 3],
            len: 0.0,
            slot: 0.0,
            st: crate::Opaque,
        }
    }
}
impl Default for LaserRt { fn default() -> Self { Self::blank() } }

#[derive(Clone, Debug, ToBytes, Default)]
pub struct KeypadRt {
    pub def: crate::devices::KeypadDef2,
    pub door: DoorId,
    pub n: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct DeviceLogRow {
}

#[derive(Clone, Debug, ToBytes)]
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
    /// what a new one holds
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

#[derive(Clone, Copy, Debug, Default)]
pub struct CONE {
    pub intensity: f64,
    pub color: V3,
    pub range_k: f64,
    pub volumetric: f64,
    pub outer_max_deg: f64,
    pub radius: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ERRAND {
    pub awareness: f64,
    pub react_t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct DevicesNoteR {
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ChronDeviceKind {
    CameraSpotted,
    CameraAlarm,
    LaserTripped,
    DeviceDown,
    CodeLearned,
    CodeEntered,
}

#[derive(Clone, Debug)]
pub enum ChronDeviceDataValue {
    Text(String),
    Num(f64),
    Bool(bool),
}

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

#[derive(Clone, Debug, Default)]
pub struct PadState {
    pub pad: crate::devices::KeypadRt,
    pub known: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SegSegClosest {
    pub dist: f64,
    pub on_b: V3,
}

pub fn led_idle() -> V3 { [1.6, 0.08, 0.05] }

pub fn led_hot() -> V3 { [14.0, 0.7, 0.3] }

pub fn cone() -> crate::devices::CONE { crate::devices::CONE { intensity: 1.1, color: [1.0, 0.12, 0.07], range_k: 0.7, volumetric: 1.0, outer_max_deg: 24.0, radius: 0.12 } }

pub const BEAM_MARGIN: f64 = 0.02;

pub const CODE_SECS: f64 = 1.2;

pub const PAD_REACH: f64 = 1.35;

pub const DENY_LEVEL: f64 = 0.05;

pub fn errand() -> crate::devices::ERRAND { crate::devices::ERRAND { awareness: 0.42, react_t: 1.0 } }

pub fn cam_col() -> V3 { [0.16, 0.17, 0.19] }

pub fn cam_dark() -> V3 { [0.05, 0.05, 0.06] }

pub fn pad_col() -> V3 { [0.22, 0.23, 0.25] }

pub fn emit_col() -> V3 { [0.14, 0.14, 0.16] }

pub fn green() -> V3 { [0.3, 5.0, 0.6] }

pub fn red() -> V3 { [5.0, 0.3, 0.15] }

pub fn amber() -> V3 { [5.0, 2.4, 0.2] }

