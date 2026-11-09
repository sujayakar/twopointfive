//! The types of `sim_lightshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Col {
    Kind,
    X,
    Hand,
    G,
    N,
    Rows,
    Beacons,
    Cameras,
    Lasers,
    Mains,
    Since,
    Until,
    RestoredAt,
    FixT,
    BlindUntil,
    Factor,
    Stagger,
    Base,
    Bits,
    Y,
    Z,
    Range,
    Radius,
    Dx,
    Dy,
    Dz,
    Outer,
    Inner,
    R,
    B,
    Intensity,
    Sx,
    Sy,
    Sz,
    SRadius,
    SIntensity,
    RowBase,
    RowBits,
    RowTarget,
    BeaconPhase,
    BeaconRow,
    BeaconTarget,
    CamYaw,
    CamPhase,
    CamSeen,
    CamReported,
    CamLx,
    CamLy,
    CamLz,
    CamRest,
    CamA,
    CamB,
    CamPeriod,
    CamPitch,
    CamPx,
    CamPy,
    CamPz,
    CamState,
    CamBits,
    CamTarget,
    CamRow,
    LasLast,
    LasCut,
    LasSense,
    LasTrips,
    LasBits,
    LasTarget,
}
impl Col {
    pub const fn as_str(self) -> &'static str {
        match self {
            Col::Kind => "kind",
            Col::X => "x",
            Col::Hand => "hand",
            Col::G => "g",
            Col::N => "n",
            Col::Rows => "rows",
            Col::Beacons => "beacons",
            Col::Cameras => "cameras",
            Col::Lasers => "lasers",
            Col::Mains => "mains",
            Col::Since => "since",
            Col::Until => "until",
            Col::RestoredAt => "restored_at",
            Col::FixT => "fix_t",
            Col::BlindUntil => "blind_until",
            Col::Factor => "factor",
            Col::Stagger => "stagger",
            Col::Base => "base",
            Col::Bits => "bits",
            Col::Y => "y",
            Col::Z => "z",
            Col::Range => "range",
            Col::Radius => "radius",
            Col::Dx => "dx",
            Col::Dy => "dy",
            Col::Dz => "dz",
            Col::Outer => "outer",
            Col::Inner => "inner",
            Col::R => "r",
            Col::B => "b",
            Col::Intensity => "intensity",
            Col::Sx => "sx",
            Col::Sy => "sy",
            Col::Sz => "sz",
            Col::SRadius => "s_radius",
            Col::SIntensity => "s_intensity",
            Col::RowBase => "row_base",
            Col::RowBits => "row_bits",
            Col::RowTarget => "row_target",
            Col::BeaconPhase => "beacon_phase",
            Col::BeaconRow => "beacon_row",
            Col::BeaconTarget => "beacon_target",
            Col::CamYaw => "cam_yaw",
            Col::CamPhase => "cam_phase",
            Col::CamSeen => "cam_seen",
            Col::CamReported => "cam_reported",
            Col::CamLx => "cam_lx",
            Col::CamLy => "cam_ly",
            Col::CamLz => "cam_lz",
            Col::CamRest => "cam_rest",
            Col::CamA => "cam_a",
            Col::CamB => "cam_b",
            Col::CamPeriod => "cam_period",
            Col::CamPitch => "cam_pitch",
            Col::CamPx => "cam_px",
            Col::CamPy => "cam_py",
            Col::CamPz => "cam_pz",
            Col::CamState => "cam_state",
            Col::CamBits => "cam_bits",
            Col::CamTarget => "cam_target",
            Col::CamRow => "cam_row",
            Col::LasLast => "las_last",
            Col::LasCut => "las_cut",
            Col::LasSense => "las_sense",
            Col::LasTrips => "las_trips",
            Col::LasBits => "las_bits",
            Col::LasTarget => "las_target",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CameraState {
    #[default] Alarm,
    Idle,
    Suspect,
}
impl CameraState {
    pub const fn as_str(self) -> &'static str {
        match self {
            CameraState::Alarm => "alarm",
            CameraState::Idle => "idle",
            CameraState::Suspect => "suspect",
        }
    }
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
impl FixtureDefKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            FixtureDefKind::Analytic => "analytic",
            FixtureDefKind::Emissive => "emissive",
            FixtureDefKind::Breaker => "breaker",
            FixtureDefKind::Device => "device",
        }
    }
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
    pub fluorescent_flicker: bool,
    pub mains: bool,
    pub interactive: bool,
    pub area_lights: Option<Vec<crate::sim_lightshome::FixtureAreaLightsItem>>,
    pub lens: Option<V3>,
    pub i: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub st: crate::Opaque,
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
            fluorescent_flicker: false,
            mains: false,
            interactive: false,
            area_lights: None,
            lens: None,
            i: 0.0,
            st: crate::Opaque,
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

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Mains {
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub st: crate::Opaque,
}
impl Mains {
    /// what a new one holds
    pub fn blank() -> Self {
        Mains {
            st: crate::Opaque,
        }
    }
}
impl Default for Mains { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug)]
pub struct CameraMachine {
    pub slot: f64,
    pub st: crate::Opaque,
}
impl CameraMachine {
    /// what a new one holds
    pub fn blank() -> Self {
        CameraMachine {
            slot: 0.0,
            st: crate::Opaque,
        }
    }
}
impl Default for CameraMachine { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug)]
pub struct LaserMachine {
    pub slot: f64,
    pub st: crate::Opaque,
}
impl LaserMachine {
    /// what a new one holds
    pub fn blank() -> Self {
        LaserMachine {
            slot: 0.0,
            st: crate::Opaque,
        }
    }
}
impl Default for LaserMachine { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct BeaconFact {
    pub light: LightId,
    pub phase: f64,
    pub target: FixtureId,
}

#[derive(Clone, Debug, ToBytes)]
pub struct LightHome {
    pub mains: crate::sim_lightshome::Mains,
    pub lamps: Vec<LightId>,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub st: crate::Opaque,
}
impl LightHome {
    /// what a new one holds
    pub fn blank() -> Self {
        LightHome {
            mains: crate::sim_lightshome::Mains::blank(),
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
pub struct T {
    pub on: f64,
    pub broken: f64,
    pub flicker: f64,
    pub mains: f64,
    pub hand: f64,
    pub authored: f64,
    pub emissive: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ROWBIT {
    pub on: f64,
    pub passing: f64,
    pub screen: f64,
    pub role: f64,
    pub own: f64,
    pub area: f64,
    pub beacon: f64,
    pub cone: f64,
    pub steady: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CB {
    pub sweep: f64,
    pub last: f64,
    pub saw: f64,
    pub dead: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LB {
    pub dead: f64,
    pub held: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MAINS {
    pub active: f64,
    pub permanent: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LIGHTIN {
    pub on: f64,
    pub break_: f64,
    pub mend: f64,
    pub toggle: f64,
    pub blind: f64,
    pub hand: f64,
    pub authored: f64,
    pub base: f64,
    pub put: f64,
    pub trip: f64,
    pub back: f64,
    pub mains: f64,
    pub fix: f64,
    pub fresh: f64,
    pub stagger: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LIGHTEV {
    pub ran_out: f64,
    pub hand: f64,
    pub fixed: f64,
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

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimLightshomeMod {
    /// (src/sim/lightshome.ts:22 C)
    pub c: JsMap<crate::sim_lightshome::Col, f64>,
    /// (src/sim/lightshome.ts:23 X)
    pub x: crate::sim_sim::SimExports,
    /// (src/sim/lightshome.ts:24 WORD_COLS)
    pub word_cols: JsSet<String>,
}
impl SimLightshomeMod {
    pub fn blank() -> Self {
        SimLightshomeMod {
            c: JsMap::new(),
            x: crate::sim_sim::SimExports::default(),
            word_cols: JsSet::new(),
        }
    }
}

/// (src/sim/lightshome.ts:16 LIGHT_COLS: a table nothing writes)
pub fn light_cols() -> [String; 67] { [String::from("n"), String::from("rows"), String::from("beacons"), String::from("cameras"), String::from("lasers"), String::from("mains"), String::from("since"), String::from("until"), String::from("restored_at"), String::from("fix_t"), String::from("blind_until"), String::from("factor"), String::from("stagger"), String::from("base"), String::from("hand"), String::from("bits"), String::from("x"), String::from("y"), String::from("z"), String::from("range"), String::from("radius"), String::from("kind"), String::from("dx"), String::from("dy"), String::from("dz"), String::from("outer"), String::from("inner"), String::from("r"), String::from("g"), String::from("b"), String::from("intensity"), String::from("sx"), String::from("sy"), String::from("sz"), String::from("s_radius"), String::from("s_intensity"), String::from("row_base"), String::from("row_bits"), String::from("row_target"), String::from("beacon_phase"), String::from("beacon_row"), String::from("beacon_target"), String::from("cam_yaw"), String::from("cam_phase"), String::from("cam_seen"), String::from("cam_reported"), String::from("cam_lx"), String::from("cam_ly"), String::from("cam_lz"), String::from("cam_rest"), String::from("cam_a"), String::from("cam_b"), String::from("cam_period"), String::from("cam_pitch"), String::from("cam_px"), String::from("cam_py"), String::from("cam_pz"), String::from("cam_state"), String::from("cam_bits"), String::from("cam_target"), String::from("cam_row"), String::from("las_last"), String::from("las_cut"), String::from("las_sense"), String::from("las_trips"), String::from("las_bits"), String::from("las_target")] }

/// (src/sim/lightshome.ts:26 LIGHT_CAPS: a table nothing writes)
pub fn light_caps() -> crate::sim_lightshome::LIGHTCAPS { crate::sim_lightshome::LIGHTCAPS { targets: 256.0, rows: 256.0, beacons: 32.0, cameras: 32.0, lasers: 32.0 } }

/// (src/sim/lightshome.ts:36 T: a table nothing writes)
pub fn t() -> crate::sim_lightshome::T { crate::sim_lightshome::T { on: 1.0, broken: 2.0, flicker: 4.0, mains: 8.0, hand: 16.0, authored: 32.0, emissive: 64.0 } }

/// (src/sim/lightshome.ts:37 ROW_BIT: a table nothing writes)
pub fn row_bit() -> crate::sim_lightshome::ROWBIT { crate::sim_lightshome::ROWBIT { on: 1.0, passing: 2.0, screen: 4.0, role: 24.0, own: 0.0, area: 8.0, beacon: 16.0, cone: 24.0, steady: 32.0 } }

/// (src/sim/lightshome.ts:38 CB: a table nothing writes)
pub fn cb() -> crate::sim_lightshome::CB { crate::sim_lightshome::CB { sweep: 1.0, last: 2.0, saw: 4.0, dead: 8.0 } }

/// (src/sim/lightshome.ts:39 LB: a table nothing writes)
pub fn lb() -> crate::sim_lightshome::LB { crate::sim_lightshome::LB { dead: 1.0, held: 2.0 } }

/// (src/sim/lightshome.ts:40 MAINS: a table nothing writes)
pub fn mains() -> crate::sim_lightshome::MAINS { crate::sim_lightshome::MAINS { active: 1.0, permanent: 2.0 } }

/// (src/sim/lightshome.ts:42 LIGHT_IN: a table nothing writes)
pub fn light_in() -> crate::sim_lightshome::LIGHTIN { crate::sim_lightshome::LIGHTIN { on: 0.0, break_: 1.0, mend: 2.0, toggle: 3.0, blind: 4.0, hand: 5.0, authored: 6.0, base: 7.0, put: 8.0, trip: 9.0, back: 10.0, mains: 11.0, fix: 12.0, fresh: 13.0, stagger: 14.0 } }

/// (src/sim/lightshome.ts:44 LIGHT_EV: a table nothing writes)
pub fn light_ev() -> crate::sim_lightshome::LIGHTEV { crate::sim_lightshome::LIGHTEV { ran_out: 1.0, hand: 2.0, fixed: 4.0 } }

/// (src/sim/lightshome.ts:46 CAM: a table nothing writes)
pub fn cam() -> crate::sim_lightshome::CAM { crate::sim_lightshome::CAM { ok: 1.0, down: 2.0, suspect: 4.0, alarm: 8.0 } }

/// (src/sim/lightshome.ts:47 LASER: a table nothing writes)
pub fn laser() -> crate::sim_lightshome::LASER { crate::sim_lightshome::LASER { ok: 1.0, down: 2.0, trip: 4.0 } }

/// (src/sim/lightshome.ts:49 CAMERA_STATES: a table nothing writes)
pub fn camera_states() -> [String; 3] { [String::from("idle"), String::from("suspect"), String::from("alarm")] }

/// (src/sim/lightshome.ts:274 LIGHTS_BIT)
pub const LIGHTS_BIT: f64 = 512.0;
