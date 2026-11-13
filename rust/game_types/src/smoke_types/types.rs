//! The types of `smoke_types`.
use crate::{Words, bytes::ToBytes, V3};

/// what kind of source an emitter is, for the host's solver
#[derive(Clone, Copy, Debug, Default, Words)]
pub enum SmokeEmitterKind {
    #[default] Flash,
    Wisp,
    Canister,
    Burst,
    Cloud,
    Jet,
    Trail,
    Vent,
    Puff,
    Breath,
}
#[derive(Clone, Debug, Default)]
pub struct SmokeEmitter {
    /// an open fire's: the graphics preset that sheds a fire's smoke drops it
    pub fire: Option<bool>,
    pub pos: V3,
    pub dir: V3,
    pub speed: f64,
    pub radius: f64,
    pub density: f64,
    pub temperature: f64,
    pub ttl: f64,
    pub age: f64,
    pub track: Option<crate::effects::Track>,
    pub kind: crate::smoke_types::SmokeEmitterKind,
    pub mode: Option<crate::smoke_types::SmokeEmitterMode>,
    pub push: Option<f64>,
    pub expand: Option<f64>,
    pub attack: Option<f64>,
    pub jitter: Option<f64>,
    pub lattice: Option<crate::smoke_types::SmokeLattice>,
    pub confined: Option<bool>,
    pub prio: Option<f64>,
    pub min_voxels: Option<f64>,
    pub look: Option<bool>,
    /// the game's own smoke holds what it lets out (said as it goes out to the host: out_events)
    pub held: Option<bool>,
    pub track_vel: Option<bool>,
    pub tint: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SmokeEmitterMode {
    Burst,
    Jet,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SmokePush {
    pub pos: V3,
    pub dir: V3,
    pub speed: f64,
    pub radius: f64,
    pub lattice: Option<crate::smoke_types::SmokeLattice>,
    pub body: Option<f64>,
    pub line: Option<f64>,
    pub push: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct WindGpuSource {
    pub kind: crate::wind::WindSourceKind,
    pub pos: V3,
    pub dir: V3,
    pub speed: f64,
    pub reach: f64,
    pub tan_spread: f64,
    pub r0: f64,
    pub d_max: f64,
    pub floor_y: Option<f64>,
    pub field: Option<bool>,
    pub bound: crate::smoke_types::WindGpuSourceBound,
    pub kinds: Option<f64>,
    pub heat: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WindGpuSourceBound {
    pub c: V3,
    pub r: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CanisterParams {
    pub canister_duration: f64,
    pub canister_density: f64,
    pub canister_radius: f64,
    pub canister_temp: f64,
    pub canister_speed: f64,
}

pub const WIND_GPU_MAX: f64 = 8.0;

/// which lattice an emitter lands on; none = by its kind: the page's smoke (src/smoke/smoke.ts `levelOf`) puts a canister on the coarse one and everything else on the fine
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SmokeLattice {
    Fine,
    Coarse,
}
