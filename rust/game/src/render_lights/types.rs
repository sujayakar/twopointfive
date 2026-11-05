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

/// (src/render/lights.ts:81 MAX_LIGHTS)
pub const MAX_LIGHTS: f64 = 64.0;

/// (src/render/lights.ts:88 REACH_BYTES: a table nothing writes)
pub fn reach_bytes() -> f64 { todo!() }

/// (src/render/lights.ts:90 REACH_VEC4S: a table nothing writes)
pub fn reach_vec4_s() -> f64 { todo!() }
