//! The types of `root_settings`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RenderSettings, src/settings.ts:7)
#[derive(Clone, Copy, Debug)]
pub struct RenderSettings {
    pub sharp_decouple: bool,
    pub fire_smoke: bool,
    pub exposure: f64,
    pub debug_view: f64,
    pub haze_density: f64,
    pub indirect_scale: f64,
    pub emissive_scale: f64,
    pub sky_zenith: V3,
    pub sky_horizon: V3,
    pub nv_gain: f64,
    pub flashlight_radius: f64,
    pub decals: bool,
    pub mesh_shadows: bool,
    pub char_mats: bool,
    pub markings: bool,
    pub marking_texel: f64,
    pub rt_mesh_hits: bool,
    pub sdf_rig_smooth: bool,
}
impl RenderSettings {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderSettings {
            sharp_decouple: false,
            fire_smoke: false,
            exposure: 0.0,
            debug_view: 0.0,
            haze_density: 0.0,
            indirect_scale: 0.0,
            emissive_scale: 0.0,
            sky_zenith: [0.0; 3],
            sky_horizon: [0.0; 3],
            nv_gain: 0.0,
            flashlight_radius: 0.0,
            decals: false,
            mesh_shadows: false,
            char_mats: false,
            markings: false,
            marking_texel: 0.0,
            rt_mesh_hits: false,
            sdf_rig_smooth: false,
        }
    }
}
impl Default for RenderSettings { fn default() -> Self { Self::zeroed() } }
