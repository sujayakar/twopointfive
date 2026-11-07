//! THE BRING-UP'S OWN PART of render_frame: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A weather frame with nothing in it. the tick makes one where the host's engine has none (weather.ts:350 `??= noWeather()`).   (render/frame.ts noWeather)
pub fn no_weather() -> crate::render_frame::WeatherFrame {
    crate::render_frame::WeatherFrame { rain: 0.0, snow: None, shear_x: 0.0, shear_z: 0.0, wet: 0.0, flash: [0.0, 0.0, 0.0], rain_top: 4.5, interior: [0.0; 4], annex: [0.0; 4], roof_c: None, roof_d: None, fog: None, near_fade: [0.0; 4], phase: None }
}
