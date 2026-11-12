//! The types of `render_lights`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct LightSet {
    pub lights: Vec<LightId>,
    pub ir: Vec<LightId>,
    #[is(crate::render_lights::LightSetFogTeam { on: false, owners: JsSet::new(), lights: JsSet::new() })] pub fog_team: crate::render_lights::LightSetFogTeam,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct LightSetFogTeam {
    pub on: bool,
    pub owners: JsSet<crate::Num>,
    pub lights: JsSet<LightId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RtLight {
    pub name: String,
    pub group: String,
    pub kind: f64,
    pub pos: V3,
    pub dir: V3,
    pub color: V3,
    pub intensity: f64,
    pub range: f64,
    pub inner_deg: f64,
    pub outer_deg: f64,
    pub radius: f64,
    pub volumetric: f64,
    pub owner: f64,
    pub enabled: bool,
    pub fixture_box: f64,
    pub flicker: f64,
    pub row: Option<f64>,
    pub ttl: f64,
    pub age: f64,
    pub peak_intensity: f64,
    pub steady: Option<crate::render_lights::RtLightSteady>,
    pub transient: Option<bool>,
    pub broad: bool,
    pub mark_ref: Option<f64>,
    pub card: bool,
    pub slot: Option<f64>,
    pub fog: Option<f64>,
    pub fog_ghost: Option<f64>,
    pub fog_by: Option<f64>,
    pub portal: Option<crate::render_lights::RtLightPortal>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RtLightSteady {
    pub intensity: f64,
    pub pos: V3,
    pub radius: f64,
    pub color: Option<V3>,
    pub range: Option<f64>,
    pub out: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RtLightPortal {
    pub half_wide: f64,
    pub share: f64,
}

pub const MAX_LIGHTS: f64 = 64.0;

