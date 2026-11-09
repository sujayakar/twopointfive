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
pub struct RenderFrameModLookConfig {
    pub fire_snow: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RenderFrameMod {
    /// (src/render/frame.ts:351 lookConfig)
    pub look_config: crate::render_frame::RenderFrameModLookConfig,
}
impl RenderFrameMod {
    pub fn blank() -> Self {
        RenderFrameMod {
            look_config: crate::render_frame::RenderFrameModLookConfig { fire_snow: 1.0 },
        }
    }
}

