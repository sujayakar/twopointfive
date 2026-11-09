//! The types of `render_sdfrig`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum RigSlotName {
    #[default] Chest,
    UpperArmR,
    UpperArmL,
    LowerTorso,
    Shoulders,
    Pelvis,
    NeckHead,
    ForearmL,
    ForearmR,
    ThighL,
    ShinL,
    SoleL,
    ThighR,
    ShinR,
    SoleR,
}
impl RigSlotName {
    pub const fn as_str(self) -> &'static str {
        match self {
            RigSlotName::Chest => "chest",
            RigSlotName::UpperArmR => "upperArmR",
            RigSlotName::UpperArmL => "upperArmL",
            RigSlotName::LowerTorso => "lowerTorso",
            RigSlotName::Shoulders => "shoulders",
            RigSlotName::Pelvis => "pelvis",
            RigSlotName::NeckHead => "neckHead",
            RigSlotName::ForearmL => "forearmL",
            RigSlotName::ForearmR => "forearmR",
            RigSlotName::ThighL => "thighL",
            RigSlotName::ShinL => "shinL",
            RigSlotName::SoleL => "soleL",
            RigSlotName::ThighR => "thighR",
            RigSlotName::ShinR => "shinR",
            RigSlotName::SoleR => "soleR",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigSlotFit {
    pub name: crate::render_sdfrig::RigSlotName,
    pub r0: f64,
    pub r1: f64,
    pub solved: [f64; 2],
    pub n: f64,
    pub rms: f64,
    pub max_abs: f64,
    pub cover: f64,
    pub len: f64,
    pub hand: f64,
    pub overridden: bool,
}

#[derive(Clone, Debug, Default)]
pub struct RigFit {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub slots: Vec<crate::render_sdfrig::RigSlotFit>,
    pub verts: f64,
    pub ms: f64,
    pub by_owner: Option<JsMap<crate::Num, crate::render_sdfrig::RigFitByOwnerValue>>,
    pub pad: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct RigFitByOwnerValue {
    pub r0: Vec<f32>,
    pub r1: Vec<f32>,
    pub pad: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct MeshRows {
    pub segs: Vec<[V3; 2]>,
    pub cand: Vec<crate::render_sdfrig::MeshRowsCandItem>,
    pub ends: Vec<u8>,
    pub home: Vec<i8>,
}

#[derive(Clone, Debug, Default)]
pub struct MeshRowsCandItem {
    pub v: Vec<i32>,
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    pub rho: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIGOVERRIDES {
    pub chest: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub upper_arm_r: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub upper_arm_l: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub lower_torso: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub shoulders: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub pelvis: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub neck_head: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub forearm_l: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub forearm_r: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub thigh_l: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub shin_l: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub sole_l: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub thigh_r: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub shin_r: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
    pub sole_r: Option<crate::render_sdfrig::RIGOVERRIDESChest>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIGOVERRIDESChest {
    pub r0: Option<f64>,
    pub r1: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct SLOTSItem {
    pub name: String,
    pub seg: crate::FnField,
    pub joints: Vec<String>,
    pub u_range: [f64; 2],
    pub after: Option<Vec<String>>,
    pub rho_max: Option<f64>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderSdfrigMod {
    /// (src/render/sdfrig.ts:109 cache)
    pub cache: JsMap<AnimGltfMeshDataId, JsMap<crate::Num, crate::render_sdfrig::RigFit>>,
    /// (src/render/sdfrig.ts:110 padV)
    pub pad_v: Vec<i32>,
    /// (src/render/sdfrig.ts:110 padD)
    pub pad_d: Vec<f64>,
    /// (src/render/sdfrig.ts:190 rowsCache)
    pub rows_cache: JsMap<AnimGltfMeshDataId, crate::render_sdfrig::MeshRows>,
}
impl RenderSdfrigMod {
    pub fn blank() -> Self {
        RenderSdfrigMod {
            cache: JsMap::new(),
            pad_v: alloc::vec![0; 0],
            pad_d: alloc::vec![0.0; 0],
            rows_cache: JsMap::new(),
        }
    }
}

/// (src/render/sdfrig.ts:16 RIG_SLOTS)
pub const RIG_SLOTS: f64 = 15.0;

/// (src/render/sdfrig.ts:17 RIG_SLOT_NAMES: a table nothing writes)
pub fn rig_slot_names() -> [String; 15] { [String::from("lowerTorso"), String::from("chest"), String::from("shoulders"), String::from("pelvis"), String::from("neckHead"), String::from("upperArmL"), String::from("forearmL"), String::from("upperArmR"), String::from("forearmR"), String::from("thighL"), String::from("shinL"), String::from("soleL"), String::from("thighR"), String::from("shinR"), String::from("soleR")] }

/// (src/render/sdfrig.ts:20 HAND_RADII: a table nothing writes)
pub fn hand_radii() -> Vec<f64> { alloc::vec![0.15, 0.16, 0.07, 0.11, 0.105, 0.052, 0.042, 0.052, 0.042, 0.075, 0.058, 0.033, 0.075, 0.058, 0.033] }

/// (src/render/sdfrig.ts:24 RIG_K: a table nothing writes)
pub fn rig_k() -> Vec<f64> { alloc::vec![0.0, 0.06, 0.05, 0.06, 0.03, 0.02, 0.03, 0.02, 0.03, 0.04, 0.04, 0.03, 0.04, 0.04, 0.03] }

/// (src/render/sdfrig.ts:25 FOLD_ORDER: a table nothing writes)
pub fn fold_order() -> Vec<f64> { alloc::vec![0.0, 3.0, 1.0, 2.0, 4.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 5.0, 6.0, 7.0, 8.0] }

/// (src/render/sdfrig.ts:27 RIG_KPAD: a table nothing writes)
pub fn rig_kpad() -> f64 { todo!() }

/// (src/render/sdfrig.ts:29 RB_QUANT)
pub const RB_QUANT: f64 = 0.001;

/// (src/render/sdfrig.ts:31 RIG_OVERRIDES: a table nothing writes)
pub fn rig_overrides() -> crate::render_sdfrig::RIGOVERRIDES { crate::render_sdfrig::RIGOVERRIDES { chest: None, upper_arm_r: None, upper_arm_l: None, lower_torso: None, shoulders: None, pelvis: None, neck_head: None, forearm_l: None, forearm_r: None, thigh_l: None, shin_l: None, sole_l: Some(crate::render_sdfrig::RIGOVERRIDESChest { r0: Some(0.034), r1: Some(0.034) }), thigh_r: None, shin_r: None, sole_r: Some(crate::render_sdfrig::RIGOVERRIDESChest { r0: Some(0.034), r1: Some(0.034) }) } }

/// (src/render/sdfrig.ts:71 SLOTS: a table nothing writes)
pub fn slots() -> Vec<crate::render_sdfrig::SLOTSItem> { todo!() }

/// (src/render/sdfrig.ts:114 RIG_PAD_MAX)
pub const RIG_PAD_MAX: f64 = 0.3;
