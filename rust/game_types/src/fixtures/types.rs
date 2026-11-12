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
    /// those lamps as a light question reads them, in the same order: that of the engine's list, which a sum of floats over them is made of, so nothing sorts them
    #[bytes(skip)]
    pub rows: Vec<Lamp>,
}

/// A LAMP A LEVEL'S FIXTURE FEEDS (a fixture's own light, an area light of a flat fixture or a marking, a beacon's spot, a camera's cone), AS A LIGHT QUESTION READS IT. Lamps nobody's
/// fixture feeds stay the question's own (a man's torch, a flash), each with how many of these stand before it in the engine's list.
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct Lamp {
    pub pos: V3,
    pub range: f64,
    pub radius: f64,
    pub kind: f64,
    pub dir: V3,
    pub outer: f64,
    pub inner: f64,
    pub color: V3,
    pub intensity: f64,
    pub on: bool,
    /// its steady self: what who asks what the PLACE lights takes (a fire's, a bad tube's)
    pub steady: Option<Steady>,
    /// which questions leave it out (PASSING: one of what the place lights; SCREEN: a sleeper's face)
    pub leave: u32,
    /// what it is by day (rust/sim light.rs by_day)
    pub day: u32,
    /// what feeds it, an area light's own strength (a beacon's: its peak), and the fixture it follows
    pub feed: Feed,
    pub base: f64,
    pub target: FixtureId,
}
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct Steady { pub pos: V3, pub radius: f64, pub intensity: f64 }
pub const PASSING: u32 = 2;
pub const SCREEN: u32 = 4;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, ToBytes)]
pub enum Feed { #[default] Own, Area, Beacon, Cone }
/// lamps fixtures feed, at most (a level over it is refused at load, by name)
pub const LAMPS: usize = 256;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CameraState {
    #[default] Idle,
    Suspect,
    Alarm,
}
