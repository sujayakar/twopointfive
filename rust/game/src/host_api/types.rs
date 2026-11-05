//! The types of `host_api`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: HostApi, src/host/api.ts:40)
#[derive(Clone, Debug)]
pub struct HostApi {
    pub world: crate::scene_boxes::BoxWorld,
    pub lights: crate::render_lights::LightSet,
    pub settings: crate::root_settings::RenderSettings,
    pub sharp_on: Option<bool>,
    pub glass_fracture: Option<bool>,
    pub effective_scale: f64,
    pub view_focus: Option<V3>,
    pub fires: Option<Vec<crate::render_passes::FireRecord>>,
    pub fire_look: Option<Vec<f64>>,
    pub host_books: Option<crate::host_api::HostBooks>,
    pub skin: Option<crate::render_gbuffer::SkinnedDrawable>,
    pub gbuffer: Option<crate::host_api::HostApiGbuffer>,
    pub device: crate::Opaque,
    pub scene_layout: crate::Opaque,
}
impl HostApi {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HostApi {
            world: crate::scene_boxes::BoxWorld::blank(),
            lights: crate::render_lights::LightSet::blank(),
            settings: crate::root_settings::RenderSettings::zeroed(),
            sharp_on: None,
            glass_fracture: None,
            effective_scale: 0.0,
            view_focus: None,
            fires: None,
            fire_look: None,
            host_books: None,
            skin: None,
            gbuffer: None,
            device: crate::Opaque,
            scene_layout: crate::Opaque,
        }
    }
}
impl Default for HostApi { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: HostBooks, src/host/api.ts:39)
#[derive(Clone, Debug)]
pub struct HostBooks {
    pub fire: Option<crate::fire::FireShown>,
    pub smoke_break: Option<crate::smokebreak::SmokeShown>,
    pub breath: Option<crate::snow_breath::BreathShown>,
    pub snow: Option<crate::snow_fx::SnowState>,
    pub snow_blast: Option<crate::snow_blast::SnowBlastShown>,
    pub rain: Option<crate::rain_fx::RainState>,
}
impl HostBooks {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HostBooks {
            fire: None,
            smoke_break: None,
            breath: None,
            snow: None,
            snow_blast: None,
            rain: None,
        }
    }
}
impl Default for HostBooks { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/host/api.ts:89 HostApi.gbuffer, src/host/api.ts:89)
#[derive(Clone, Copy, Debug)]
pub struct HostApiGbuffer {
}
impl HostApiGbuffer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HostApiGbuffer {
        }
    }
}
impl Default for HostApiGbuffer { fn default() -> Self { Self::zeroed() } }
