//! The types of `fixtures`.
use crate::{Blank, bytes::ToBytes, Words, ids::*, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words, Default)]
pub enum FixtureKind {
    #[default] Analytic,
    Emissive,
    Breaker,
    Device,
}

/// an area light a flat fixture feeds, and how strong it is whole
#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FixtureLight {
    pub light: LightId,
    pub base: f64,
}

/// A FIXTURE: what the level says of it, and how it stands
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Fixture {
    pub name: String,
    pub group: String,
    pub kind: FixtureKind,
    pub pos: V3,
    pub light: Option<LightId>,
    pub area_lights: Option<Vec<FixtureLight>>,
    /// the boxes that glow with it
    pub boxes: Vec<f64>,
    pub base_emissive: V3,
    pub base_intensity: f64,
    /// a vehicle's: where its lens is
    pub lens: Option<V3>,
    /// fed from the mains, and how long after them it strikes
    pub mains: bool,
    pub stagger: f64,
    pub interactive: bool,
    /// a bad tube
    pub flicker: bool,
    /// on in the level as authored
    pub authored: bool,
    pub on: bool,
    pub broken: bool,
    /// an OCP's blind runs to here, and a strike's restrike from here (−1: neither)
    #[is(-1.0)] pub blind_until: f64,
    /// the game time a hand put it out at its wall switch
    pub hand_off: Option<f64>,
    /// what it burns at this tick, 0 to 1
    pub factor: f64,
}

/// THE MAINS
#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Mains {
    /// they are out
    pub active: bool,
    /// …for good
    pub permanent: bool,
    #[is(-1.0)] pub until: f64,
    #[is(-1.0)] pub since: f64,
    #[is(-100.0)] pub restored_at: f64,
    /// how long a man of theirs has been at the panel
    pub fix_t: f64,
}

/// the mains, and which lamp each row a light question reads is
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightHome {
    pub mains: Mains,
    pub lamps: Vec<LightId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CameraState {
    #[default] Idle,
    Suspect,
    Alarm,
}
