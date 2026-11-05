//! The types of `render_lights`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: LightSet, src/render/lights.ts:123)
#[derive(Clone, Debug)]
pub struct LightSet {
    pub lights: Vec<LightId>,
    pub buf: crate::Opaque,
    pub reach_buf: crate::Opaque,
    pub reach_data: Vec<u32>,
    pub ir: Vec<LightId>,
    pub device: crate::Opaque,
    pub fog_team: crate::render_lights::LightSetFogTeam,
}
impl LightSet {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        LightSet {
            lights: Vec::new(),
            buf: crate::Opaque,
            reach_buf: crate::Opaque,
            reach_data: Vec::new(),
            ir: Vec::new(),
            device: crate::Opaque,
            fog_team: crate::render_lights::LightSetFogTeam { on: false, owners: JsSet::new(), lights: JsSet::new() },
        }
    }
}
impl Default for LightSet { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/render/lights.ts:198 LightSet.fogTeam, src/render/lights.ts:198)
#[derive(Clone, Debug)]
pub struct LightSetFogTeam {
    pub on: bool,
    pub owners: JsSet<crate::Num>,
    pub lights: JsSet<LightId>,
}
impl LightSetFogTeam {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LightSetFogTeam {
            on: false,
            owners: JsSet::new(),
            lights: JsSet::new(),
        }
    }
}
impl Default for LightSetFogTeam { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RtLight, src/render/lights.ts:8)
#[derive(Clone, Debug)]
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
    pub row: Option<f64>,
    pub ttl: f64,
    pub age: f64,
    pub peak_intensity: f64,
    pub steady: Option<crate::render_lights::RtLightSteady>,
    pub transient: Option<bool>,
    pub reach: Option<Vec<u32>>,
    pub reach_dir: Option<Vec<u32>>,
    pub slot: Option<f64>,
    pub fog: Option<f64>,
    pub fog_ghost: Option<f64>,
    pub fog_by: Option<f64>,
    pub portal: Option<crate::render_lights::RtLightPortal>,
}
impl RtLight {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RtLight {
            name: String::new(),
            group: String::new(),
            kind: 0.0,
            pos: [0.0; 3],
            dir: [0.0; 3],
            color: [0.0; 3],
            intensity: 0.0,
            range: 0.0,
            inner_deg: 0.0,
            outer_deg: 0.0,
            radius: 0.0,
            volumetric: 0.0,
            owner: 0.0,
            enabled: false,
            fixture_box: 0.0,
            row: None,
            ttl: 0.0,
            age: 0.0,
            peak_intensity: 0.0,
            steady: None,
            transient: None,
            reach: None,
            reach_dir: None,
            slot: None,
            fog: None,
            fog_ghost: None,
            fog_by: None,
            portal: None,
        }
    }
}
impl Default for RtLight { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/lights.ts:26 RtLight.steady, src/render/lights.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct RtLightSteady {
    pub intensity: f64,
    pub pos: V3,
    pub radius: f64,
    pub color: Option<V3>,
    pub range: Option<f64>,
    pub out: Option<bool>,
}
impl RtLightSteady {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RtLightSteady {
            intensity: 0.0,
            pos: [0.0; 3],
            radius: 0.0,
            color: None,
            range: None,
            out: None,
        }
    }
}
impl Default for RtLightSteady { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/lights.ts:65 RtLight.portal, src/render/lights.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct RtLightPortal {
    pub half_wide: f64,
    pub share: f64,
    pub deep: Option<f64>,
    pub rims: Option<f64>,
}
impl RtLightPortal {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RtLightPortal {
            half_wide: 0.0,
            share: 0.0,
            deep: None,
            rims: None,
        }
    }
}
impl Default for RtLightPortal { fn default() -> Self { Self::zeroed() } }

/// (src/render/lights.ts:81 MAX_LIGHTS)
pub const MAX_LIGHTS: f64 = 64.0;

/// (src/render/lights.ts:88 REACH_BYTES: a table nothing writes)
pub fn reach_bytes() -> f64 { todo!() }

/// (src/render/lights.ts:90 REACH_VEC4S: a table nothing writes)
pub fn reach_vec4_s() -> f64 { todo!() }
