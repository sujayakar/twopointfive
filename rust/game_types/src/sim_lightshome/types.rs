//! The types of `sim_lightshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CameraState {
    #[default] Alarm,
    Idle,
    Suspect,
}

#[derive(Clone, Debug, Default)]
pub struct FixtureDef {
    pub name: String,
    pub group: String,
    pub kind: crate::sim_lightshome::FixtureDefKind,
    pub pos: V3,
    pub light: Option<LightId>,
    pub boxes: Vec<f64>,
    pub base_emissive: V3,
    pub base_intensity: f64,
    pub on: bool,
    pub broken: bool,
    pub disabled_until: f64,
    pub fluorescent_flicker: bool,
    pub factor: f64,
    pub mains: bool,
    pub interactive: bool,
    pub stagger: f64,
    pub area_lights: Option<Vec<crate::sim_lightshome::FixtureDefAreaLightsItem>>,
    pub lens: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum FixtureDefKind {
    #[default] Analytic,
    Emissive,
    Breaker,
    Device,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FixtureDefAreaLightsItem {
    pub light: LightId,
    pub base: f64,
}

#[derive(Clone, Debug, ToBytes)]
pub struct Fixture {
    pub name: String,
    pub group: String,
    pub kind: crate::sim_lightshome::FixtureKind,
    pub pos: V3,
    pub light: Option<LightId>,
    pub boxes: Vec<f64>,
    pub base_emissive: V3,
    pub mains: bool,
    pub interactive: bool,
    pub area_lights: Option<Vec<crate::sim_lightshome::FixtureAreaLightsItem>>,
    pub lens: Option<V3>,
    pub i: f64,
}
impl Fixture {
    /// what a new one holds
    pub fn blank() -> Self {
        Fixture {
            name: String::new(),
            group: String::new(),
            kind: crate::sim_lightshome::FixtureKind::Analytic,
            pos: [0.0; 3],
            light: None,
            boxes: Vec::new(),
            base_emissive: [0.0; 3],
            mains: false,
            interactive: false,
            area_lights: None,
            lens: None,
            i: 0.0,
        }
    }
}
impl Default for Fixture { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum FixtureKind {
    Analytic,
    Emissive,
    Breaker,
    Device,
}
impl FixtureKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            FixtureKind::Analytic => "analytic",
            FixtureKind::Emissive => "emissive",
            FixtureKind::Breaker => "breaker",
            FixtureKind::Device => "device",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FixtureAreaLightsItem {
    pub light: LightId,
    pub base: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BeaconFact {
    pub light: LightId,
    pub phase: f64,
    pub target: FixtureId,
}

#[derive(Clone, Debug, ToBytes)]
pub struct LightHome {
    pub lamps: Vec<LightId>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub st: crate::Opaque,
}
impl LightHome {
    /// what a new one holds
    pub fn blank() -> Self {
        LightHome {
            lamps: Vec::new(),
            st: crate::Opaque,
        }
    }
}
impl Default for LightHome { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct LIGHTCAPS {
    pub targets: f64,
    pub rows: f64,
    pub beacons: f64,
    pub cameras: f64,
    pub lasers: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ROWBIT {
    pub passing: f64,
    pub screen: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LIGHTIN {
    pub on: f64,
    pub break_: f64,
    pub mend: f64,
    pub hand: f64,
    pub authored: f64,
    pub base: f64,
    pub fix: f64,
    pub fresh: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CAM {
    pub ok: f64,
    pub down: f64,
    pub suspect: f64,
    pub alarm: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LASER {
    pub ok: f64,
    pub down: f64,
    pub trip: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LightHomeStandConesItem {
    pub light: LightId,
    pub target: FixtureId,
    pub slot: f64,
}

pub fn light_cols() -> [String; 67] { [String::from("n"), String::from("rows"), String::from("beacons"), String::from("cameras"), String::from("lasers"), String::from("mains"), String::from("since"), String::from("until"), String::from("restored_at"), String::from("fix_t"), String::from("blind_until"), String::from("factor"), String::from("stagger"), String::from("base"), String::from("hand"), String::from("bits"), String::from("x"), String::from("y"), String::from("z"), String::from("range"), String::from("radius"), String::from("kind"), String::from("dx"), String::from("dy"), String::from("dz"), String::from("outer"), String::from("inner"), String::from("r"), String::from("g"), String::from("b"), String::from("intensity"), String::from("sx"), String::from("sy"), String::from("sz"), String::from("s_radius"), String::from("s_intensity"), String::from("row_base"), String::from("row_bits"), String::from("row_target"), String::from("beacon_phase"), String::from("beacon_row"), String::from("beacon_target"), String::from("cam_yaw"), String::from("cam_phase"), String::from("cam_seen"), String::from("cam_reported"), String::from("cam_lx"), String::from("cam_ly"), String::from("cam_lz"), String::from("cam_rest"), String::from("cam_a"), String::from("cam_b"), String::from("cam_period"), String::from("cam_pitch"), String::from("cam_px"), String::from("cam_py"), String::from("cam_pz"), String::from("cam_state"), String::from("cam_bits"), String::from("cam_target"), String::from("cam_row"), String::from("las_last"), String::from("las_cut"), String::from("las_sense"), String::from("las_trips"), String::from("las_bits"), String::from("las_target")] }

pub fn light_caps() -> crate::sim_lightshome::LIGHTCAPS { crate::sim_lightshome::LIGHTCAPS { targets: 256.0, rows: 256.0, beacons: 32.0, cameras: 32.0, lasers: 32.0 } }

pub fn row_bit() -> crate::sim_lightshome::ROWBIT { crate::sim_lightshome::ROWBIT { passing: 2.0, screen: 4.0 } }

pub fn light_in() -> crate::sim_lightshome::LIGHTIN { crate::sim_lightshome::LIGHTIN { on: 0.0, break_: 1.0, mend: 2.0, hand: 5.0, authored: 6.0, base: 7.0, fix: 12.0, fresh: 13.0 } }

pub fn cam() -> crate::sim_lightshome::CAM { crate::sim_lightshome::CAM { ok: 1.0, down: 2.0, suspect: 4.0, alarm: 8.0 } }

pub fn laser() -> crate::sim_lightshome::LASER { crate::sim_lightshome::LASER { ok: 1.0, down: 2.0, trip: 4.0 } }

pub const LIGHTS_BIT: f64 = 512.0;
