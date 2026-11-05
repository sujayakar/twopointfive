//! The types of `render_frame`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WeatherFrame, src/render/frame.ts:66)
#[derive(Clone, Copy, Debug)]
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
impl WeatherFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherFrame {
            rain: 0.0,
            snow: None,
            shear_x: 0.0,
            shear_z: 0.0,
            wet: 0.0,
            flash: [0.0; 3],
            rain_top: 0.0,
            interior: [0.0; 4],
            annex: [0.0; 4],
            roof_c: None,
            roof_d: None,
            fog: None,
            near_fade: [0.0; 4],
            phase: None,
        }
    }
}
impl Default for WeatherFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:264 reachBinding, src/render/frame.ts:264)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModReachBinding {
    pub storage: bool,
}
impl RenderFrameModReachBinding {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModReachBinding {
            storage: false,
        }
    }
}
impl Default for RenderFrameModReachBinding { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:269 meshBinding, src/render/frame.ts:269)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModMeshBinding {
    pub enabled: bool,
}
impl RenderFrameModMeshBinding {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModMeshBinding {
            enabled: false,
        }
    }
}
impl Default for RenderFrameModMeshBinding { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:275 traceConfig, src/render/frame.ts:275)
#[derive(Clone, Debug)]
pub struct RenderFrameModTraceConfig {
    pub variant: f64,
    pub common: Option<String>,
}
impl RenderFrameModTraceConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModTraceConfig {
            variant: 0.0,
            common: None,
        }
    }
}
impl Default for RenderFrameModTraceConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:280 glassBreakConfig, src/render/frame.ts:280)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModGlassBreakConfig {
    pub on: bool,
}
impl RenderFrameModGlassBreakConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModGlassBreakConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModGlassBreakConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:284 weatherKindConfig, src/render/frame.ts:284)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModWeatherKindConfig {
    pub on: bool,
}
impl RenderFrameModWeatherKindConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModWeatherKindConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModWeatherKindConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:289 snowConfig, src/render/frame.ts:289)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModSnowConfig {
    pub on: bool,
}
impl RenderFrameModSnowConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModSnowConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModSnowConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:294 snowLookConfig, src/render/frame.ts:294)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModSnowLookConfig {
    pub surface: bool,
    pub wisps: bool,
    pub tracks: bool,
    pub glow: bool,
    pub sparkle: bool,
    pub grains: bool,
    pub under: bool,
}
impl RenderFrameModSnowLookConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModSnowLookConfig {
            surface: false,
            wisps: false,
            tracks: false,
            glow: false,
            sparkle: false,
            grains: false,
            under: false,
        }
    }
}
impl Default for RenderFrameModSnowLookConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:298 weatherEdgeConfig, src/render/frame.ts:298)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModWeatherEdgeConfig {
    pub on: bool,
}
impl RenderFrameModWeatherEdgeConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModWeatherEdgeConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModWeatherEdgeConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:303 weatherVolConfig, src/render/frame.ts:303)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModWeatherVolConfig {
    pub on: bool,
}
impl RenderFrameModWeatherVolConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModWeatherVolConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModWeatherVolConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:309 rainShadowConfig, src/render/frame.ts:309)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModRainShadowConfig {
    pub on: bool,
}
impl RenderFrameModRainShadowConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModRainShadowConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModRainShadowConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:317 rainScaleConfig, src/render/frame.ts:317)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModRainScaleConfig {
    pub on: bool,
}
impl RenderFrameModRainScaleConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModRainScaleConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModRainScaleConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:323 rainWallsConfig, src/render/frame.ts:323)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModRainWallsConfig {
    pub on: bool,
}
impl RenderFrameModRainWallsConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModRainWallsConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModRainWallsConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:332 bounceWallsConfig, src/render/frame.ts:332)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModBounceWallsConfig {
    pub on: bool,
}
impl RenderFrameModBounceWallsConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModBounceWallsConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModBounceWallsConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:340 wallSidesConfig, src/render/frame.ts:340)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModWallSidesConfig {
    pub on: bool,
}
impl RenderFrameModWallSidesConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModWallSidesConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModWallSidesConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:343 smokeLabConfig, src/render/frame.ts:343)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModSmokeLabConfig {
    pub on: bool,
}
impl RenderFrameModSmokeLabConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModSmokeLabConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModSmokeLabConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:351 lookConfig, src/render/frame.ts:351)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModLookConfig {
    pub on: bool,
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
    pub stir: f64,
    pub grade_exp: f64,
    pub grade_c: f64,
    pub grade_h: f64,
    pub grade_s: f64,
    pub grade_sat: f64,
}
impl RenderFrameModLookConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModLookConfig {
            on: false,
            kept: 0.0,
            pressed: 0.0,
            bounce: 0.0,
            air: 0.0,
            sky: 0.0,
            shade: 0.0,
            lamps: 0.0,
            exposure: None,
            puffs: 0.0,
            veil: 0.0,
            snow: None,
            eyes: 0.0,
            eye_at: None,
            door_gaps: None,
            night_sky: None,
            fire_smoke: 0.0,
            fire_fit: 0.0,
            long_foot: 0.0,
            walked: 0.0,
            fire_snow: 0.0,
            rooms: 0.0,
            steady: 0.0,
            room_bright: 0.0,
            order: 0.0,
            dark: 0.0,
            windows: 0.0,
            glare: 0.0,
            stir: 0.0,
            grade_exp: 0.0,
            grade_c: 0.0,
            grade_h: 0.0,
            grade_s: 0.0,
            grade_sat: 0.0,
        }
    }
}
impl Default for RenderFrameModLookConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:364 reflWallsConfig, src/render/frame.ts:364)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModReflWallsConfig {
    pub on: bool,
}
impl RenderFrameModReflWallsConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModReflWallsConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModReflWallsConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:368 sharpCapConfig, src/render/frame.ts:368)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModSharpCapConfig {
    pub on: bool,
}
impl RenderFrameModSharpCapConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModSharpCapConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModSharpCapConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:382 sunGlintConfig, src/render/frame.ts:382)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModSunGlintConfig {
    pub on: bool,
}
impl RenderFrameModSunGlintConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModSunGlintConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModSunGlintConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:390 glintPickConfig, src/render/frame.ts:390)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModGlintPickConfig {
    pub on: bool,
}
impl RenderFrameModGlintPickConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModGlintPickConfig {
            on: false,
        }
    }
}
impl Default for RenderFrameModGlintPickConfig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/frame.ts:395 highlightConfig, src/render/frame.ts:395)
#[derive(Clone, Copy, Debug)]
pub struct RenderFrameModHighlightConfig {
    pub reach: f64,
}
impl RenderFrameModHighlightConfig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RenderFrameModHighlightConfig {
            reach: 0.0,
        }
    }
}
impl Default for RenderFrameModHighlightConfig { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderFrameMod {
    /// (src/render/frame.ts:264 reachBinding)
    pub reach_binding: crate::render_frame::RenderFrameModReachBinding,
    /// (src/render/frame.ts:269 meshBinding)
    pub mesh_binding: crate::render_frame::RenderFrameModMeshBinding,
    /// (src/render/frame.ts:275 traceConfig)
    pub trace_config: crate::render_frame::RenderFrameModTraceConfig,
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
    /// (src/render/frame.ts:340 wallSidesConfig)
    pub wall_sides_config: crate::render_frame::RenderFrameModWallSidesConfig,
    /// (src/render/frame.ts:343 smokeLabConfig)
    pub smoke_lab_config: crate::render_frame::RenderFrameModSmokeLabConfig,
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
            trace_config: crate::render_frame::RenderFrameModTraceConfig { variant: 0.0, common: None },
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
            wall_sides_config: crate::render_frame::RenderFrameModWallSidesConfig { on: false },
            smoke_lab_config: crate::render_frame::RenderFrameModSmokeLabConfig { on: false },
            look_config: crate::render_frame::RenderFrameModLookConfig { on: false, kept: 0.0, pressed: 1.0, bounce: 0.7, air: 1.0, sky: 1.0, shade: 0.7, lamps: 1.0, exposure: None, puffs: 1.0, veil: 1.0, snow: None, eyes: 1.0, eye_at: None, door_gaps: None, night_sky: None, fire_smoke: 1.0, fire_fit: 1.0, long_foot: 1.0, walked: 1.0, fire_snow: 1.0, rooms: 1.0, steady: 1.0, room_bright: 1.0, order: 1.0, dark: 1.0, windows: 1.0, glare: 0.1, stir: 1.0, grade_exp: 1.0, grade_c: 1.35, grade_h: 0.0, grade_s: 0.0, grade_sat: 1.0 },
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
