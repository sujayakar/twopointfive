//! The types of `gunreload`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum FeedKind {
    #[default] Mag,
    Shells,
    Belt,
}
impl FeedKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            FeedKind::Mag => "mag",
            FeedKind::Shells => "shells",
            FeedKind::Belt => "belt",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FeedBeat {
    pub dur: f64,
    pub out: f64,
    pub in_: f64,
    pub rack: f64,
    pub pull: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FeedCue {
    pub t: f64,
    pub sfx: String,
    pub gain: f64,
    pub rate: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct FeedKey {
    pub t: f64,
    pub at: crate::gunreload::FeedKeyAt,
    pub how: crate::gunreload::FeedKeyHow,
    pub grip: f64,
}
impl Default for FeedKey {
    fn default() -> Self {
        FeedKey {
            t: 0.0,
            at: crate::gunreload::FeedKeyAt::List([0.0; 3]),
            how: crate::gunreload::FeedKeyHow::Mag,
            grip: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FeedKeyAt {
    List(V3),
    Pouch,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FeedKeyHow {
    Mag,
    Top,
    Pouch,
}
impl FeedKeyHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            FeedKeyHow::Mag => "mag",
            FeedKeyHow::Top => "top",
            FeedKeyHow::Pouch => "pouch",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FeedMarks {
    pub mag: V3,
    pub strip: V3,
    pub handle: V3,
    pub pull: V3,
    pub hf: V3,
    pub hb: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShellRow {
    pub dur: f64,
    pub out: Option<f64>,
    pub in_: Option<f64>,
    pub rack: Option<f64>,
    pub shell: Option<crate::gunreload::ShellRowShell>,
    pub exact: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ShellRowShell {
    pub lead: f64,
    pub each: f64,
    pub tail: f64,
    pub bolt: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FEEDValue {
    pub kind: crate::gunreload::FeedKind,
    pub hand: V3,
    pub col: V3,
    pub drop: Option<f64>,
    pub marks: Option<crate::gunreload::FeedMarks>,
    pub port_first: Option<bool>,
    pub pump: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BELT {
    pub cover: V3,
    pub lifted: V3,
    pub box_: V3,
    pub strip: V3,
    pub tray: V3,
    pub handle: V3,
    pub pull: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TUBE {
    pub saddle: V3,
    pub port: V3,
    pub home: V3,
    pub eject: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BeltTimes {
    pub up: f64,
    pub off: f64,
    pub pouch: f64,
    pub on: f64,
    pub lay: f64,
    pub down: f64,
    pub charge: f64,
}

#[derive(Clone, Debug)]
pub enum FeedPath {
    Arm0(crate::gunreload::FeedPathArm0),
    Arm1(crate::gunreload::FeedPathArm1),
}

#[derive(Clone, Debug, Default)]
pub struct FeedPathArm0 {
    pub keys: Vec<crate::gunreload::FeedKey>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FeedPathArm1 {
    pub marks: Option<crate::gunreload::FeedMarks>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FeedPlanShells {
    pub n: f64,
    pub pull: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FeedPlan {
    pub cues: Option<Vec<crate::gunreload::FeedCue>>,
    pub times: Vec<f64>,
    pub drop: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct ShellPlan {
    pub row: crate::gunreload::ShellRow,
    pub times: Vec<f64>,
    pub n: f64,
    pub port: bool,
    pub pull: bool,
    pub idle: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReloadShow {
    pub no_mag: bool,
    pub cover: f64,
}

/// (src/game/gunreload.ts:47 GROUP: a table nothing writes)
pub fn group() -> V3 { [0.06, 0.08, 0.09] }

/// (src/game/gunreload.ts:52 FEED: a table nothing writes)
pub fn feed() -> JsMap<String, crate::gunreload::FEEDValue> { { let mut t = JsMap::new(); t.set(String::from("mk12"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Mag, hand: [0.012, 0.055, 0.03], col: [0.03, 0.03, 0.033], drop: Some(6.0), marks: None, port_first: None, pump: None }); t.set(String::from("mp7"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Mag, hand: [0.008, 0.095, 0.013], col: [0.03, 0.03, 0.033], drop: Some(7.0), marks: Some(crate::gunreload::FeedMarks { mag: [0.0 + 0.06 + 0.025, -0.135 + 0.08 + -0.15, -0.045 + 0.09 + -0.05], strip: [0.0 + 0.06 + 0.03, -0.135 + 0.08 + -0.24, -0.045 + 0.09 + -0.07], handle: [0.05, 0.09, -0.02], pull: [0.05, 0.09, -0.1], hf: [-0.7, 0.6, 0.1], hb: [0.5, -0.5, 0.6] }), port_first: None, pump: None }); t.set(String::from("mk46"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Belt, hand: [0.035, 0.045, 0.06], col: [0.16, 0.17, 0.1], drop: Some(5.0), marks: None, port_first: None, pump: None }); t.set(String::from("m1014"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Shells, hand: [0.01, 0.035, 0.01], col: [0.5, 0.05, 0.04], drop: None, marks: None, port_first: Some(true), pump: None }); t.set(String::from("m870"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Shells, hand: [0.01, 0.035, 0.01], col: [0.5, 0.05, 0.04], drop: None, marks: None, port_first: None, pump: Some(true) }); t.set(String::from("hk45"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Mag, hand: [0.009, 0.055, 0.018], col: [0.05, 0.05, 0.055], drop: Some(0.0), marks: None, port_first: None, pump: None }); t.set(String::from("g18"), crate::gunreload::FEEDValue { kind: crate::gunreload::FeedKind::Mag, hand: [0.008, 0.11, 0.016], col: [0.05, 0.05, 0.055], drop: Some(4.0), marks: None, port_first: None, pump: None }); t } }

/// (src/game/gunreload.ts:65 BELT: a table nothing writes)
pub fn belt() -> crate::gunreload::BELT { crate::gunreload::BELT { cover: [0.0 + 0.06 + 0.07, 0.04 + 0.08 + 0.03, 0.07 + 0.09 + -0.03], lifted: [0.0 + 0.06 + 0.07, 0.04 + 0.08 + 0.1, 0.07 + 0.09 + -0.08], box_: [0.02 + 0.06 + 0.03, -0.085 + 0.08 + -0.13, 0.06 + 0.09 + -0.04], strip: [0.02 + 0.06 + 0.04, -0.085 + 0.08 + -0.225, 0.06 + 0.09 + -0.03], tray: [0.0 + 0.06 + 0.06, 0.04 + 0.08 + 0.01, 0.07 + 0.09 + 0.0], handle: [0.04, -0.06, 0.1], pull: [0.04, -0.06, 0.0] } }

/// (src/game/gunreload.ts:68 TUBE: a table nothing writes)
pub fn tube() -> crate::gunreload::TUBE { crate::gunreload::TUBE { saddle: [0.024 + 0.06 + 0.08, 0.0 + 0.08 + -0.03, 0.0 + 0.09 + -0.02], port: [0.0 + 0.06 + 0.03, -0.047 + 0.08 + -0.11, 0.06 + 0.09 + -0.04], home: [0.0 + 0.06 + 0.03, -0.047 + 0.08 + -0.09, 0.06 + 0.09 + -0.015], eject: [-0.02 + 0.06 + 0.04, 0.005 + 0.08 + -0.15, 0.03 + 0.09 + 0.0] } }
