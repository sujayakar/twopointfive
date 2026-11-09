//! The types of `render_frame`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WeatherFrame {
    pub rain: f64,
    pub snow: Option<f64>,
    pub shear_x: f64,
    pub shear_z: f64,
    pub wet: f64,
    pub flash: V3,
    pub rain_top: f64,
    pub interior: [f64; 4],
    pub annex: [f64; 4],
    pub roof_c: Option<f64>,
    pub roof_d: Option<f64>,
    pub fog: Option<f64>,
    pub near_fade: [f64; 4],
    pub phase: Option<[f64; 4]>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModReachBinding {
    pub storage: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModMeshBinding {
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModGlassBreakConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModWeatherKindConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModSnowConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModSnowLookConfig {
    pub surface: bool,
    pub wisps: bool,
    pub tracks: bool,
    pub glow: bool,
    pub sparkle: bool,
    pub grains: bool,
    pub under: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModWeatherEdgeConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModWeatherVolConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModRainShadowConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModRainScaleConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModRainWallsConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModBounceWallsConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModLookConfig {
    pub kept: f64,
    pub pressed: f64,
    pub bounce: f64,
    pub air: f64,
    pub sky: f64,
    pub shade: f64,
    pub lamps: f64,
    pub exposure: Option<crate::FnField>,
    pub puffs: f64,
    pub veil: f64,
    pub snow: Option<crate::FnField>,
    pub eyes: f64,
    pub eye_at: Option<crate::FnField>,
    pub door_gaps: Option<crate::FnField>,
    pub night_sky: Option<crate::FnField>,
    pub fire_smoke: f64,
    pub fire_fit: f64,
    pub long_foot: f64,
    pub walked: f64,
    pub fire_snow: f64,
    pub rooms: f64,
    pub steady: f64,
    pub room_bright: f64,
    pub order: f64,
    pub dark: f64,
    pub windows: f64,
    pub glare: f64,
    pub grade_exp: f64,
    pub grade_c: f64,
    pub grade_h: f64,
    pub grade_s: f64,
    pub grade_sat: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModReflWallsConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModSharpCapConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModSunGlintConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModGlintPickConfig {
    pub on: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderFrameModHighlightConfig {
    pub reach: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderFrameMod {
    /// (src/render/frame.ts:264 reachBinding)
    pub reach_binding: crate::render_frame::RenderFrameModReachBinding,
    /// (src/render/frame.ts:269 meshBinding)
    pub mesh_binding: crate::render_frame::RenderFrameModMeshBinding,
    /// (src/render/frame.ts:280 glassBreakConfig)
    pub glass_break_config: crate::render_frame::RenderFrameModGlassBreakConfig,
    /// (src/render/frame.ts:284 weatherKindConfig)
    pub weather_kind_config: crate::render_frame::RenderFrameModWeatherKindConfig,
    /// (src/render/frame.ts:289 snowConfig)
    pub snow_config: crate::render_frame::RenderFrameModSnowConfig,
    /// (src/render/frame.ts:294 snowLookConfig)
    pub snow_look_config: crate::render_frame::RenderFrameModSnowLookConfig,
    /// (src/render/frame.ts:298 weatherEdgeConfig)
    pub weather_edge_config: crate::render_frame::RenderFrameModWeatherEdgeConfig,
    /// (src/render/frame.ts:303 weatherVolConfig)
    pub weather_vol_config: crate::render_frame::RenderFrameModWeatherVolConfig,
    /// (src/render/frame.ts:309 rainShadowConfig)
    pub rain_shadow_config: crate::render_frame::RenderFrameModRainShadowConfig,
    /// (src/render/frame.ts:317 rainScaleConfig)
    pub rain_scale_config: crate::render_frame::RenderFrameModRainScaleConfig,
    /// (src/render/frame.ts:323 rainWallsConfig)
    pub rain_walls_config: crate::render_frame::RenderFrameModRainWallsConfig,
    /// (src/render/frame.ts:332 bounceWallsConfig)
    pub bounce_walls_config: crate::render_frame::RenderFrameModBounceWallsConfig,
    /// (src/render/frame.ts:351 lookConfig)
    pub look_config: crate::render_frame::RenderFrameModLookConfig,
    /// (src/render/frame.ts:364 reflWallsConfig)
    pub refl_walls_config: crate::render_frame::RenderFrameModReflWallsConfig,
    /// (src/render/frame.ts:368 sharpCapConfig)
    pub sharp_cap_config: crate::render_frame::RenderFrameModSharpCapConfig,
    /// (src/render/frame.ts:382 sunGlintConfig)
    pub sun_glint_config: crate::render_frame::RenderFrameModSunGlintConfig,
    /// (src/render/frame.ts:390 glintPickConfig)
    pub glint_pick_config: crate::render_frame::RenderFrameModGlintPickConfig,
    /// (src/render/frame.ts:395 highlightConfig)
    pub highlight_config: crate::render_frame::RenderFrameModHighlightConfig,
}
impl RenderFrameMod {
    pub fn blank() -> Self {
        RenderFrameMod {
            reach_binding: crate::render_frame::RenderFrameModReachBinding { storage: true },
            mesh_binding: crate::render_frame::RenderFrameModMeshBinding { enabled: true },
            glass_break_config: crate::render_frame::RenderFrameModGlassBreakConfig { on: false },
            weather_kind_config: crate::render_frame::RenderFrameModWeatherKindConfig { on: false },
            snow_config: crate::render_frame::RenderFrameModSnowConfig { on: false },
            snow_look_config: crate::render_frame::RenderFrameModSnowLookConfig { surface: false, wisps: false, tracks: false, glow: false, sparkle: false, grains: false, under: false },
            weather_edge_config: crate::render_frame::RenderFrameModWeatherEdgeConfig { on: true },
            weather_vol_config: crate::render_frame::RenderFrameModWeatherVolConfig { on: true },
            rain_shadow_config: crate::render_frame::RenderFrameModRainShadowConfig { on: true },
            rain_scale_config: crate::render_frame::RenderFrameModRainScaleConfig { on: true },
            rain_walls_config: crate::render_frame::RenderFrameModRainWallsConfig { on: true },
            bounce_walls_config: crate::render_frame::RenderFrameModBounceWallsConfig { on: true },
            look_config: crate::render_frame::RenderFrameModLookConfig { kept: 0.0, pressed: 1.0, bounce: 0.7, air: 1.0, sky: 1.0, shade: 0.7, lamps: 1.0, exposure: None, puffs: 1.0, veil: 1.0, snow: None, eyes: 1.0, eye_at: None, door_gaps: None, night_sky: None, fire_smoke: 1.0, fire_fit: 1.0, long_foot: 1.0, walked: 1.0, fire_snow: 1.0, rooms: 1.0, steady: 1.0, room_bright: 1.0, order: 1.0, dark: 1.0, windows: 1.0, glare: 0.1, grade_exp: 1.0, grade_c: 1.35, grade_h: 0.0, grade_s: 0.0, grade_sat: 1.0 },
            refl_walls_config: crate::render_frame::RenderFrameModReflWallsConfig { on: true },
            sharp_cap_config: crate::render_frame::RenderFrameModSharpCapConfig { on: true },
            sun_glint_config: crate::render_frame::RenderFrameModSunGlintConfig { on: true },
            glint_pick_config: crate::render_frame::RenderFrameModGlintPickConfig { on: false },
            highlight_config: crate::render_frame::RenderFrameModHighlightConfig { reach: 1.0 },
        }
    }
}

/// (src/render/frame.ts:5 sdfRigSrc: a table nothing writes)
pub fn sdf_rig_src() -> String { todo!() }
