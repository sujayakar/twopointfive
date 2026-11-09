#![allow(unused_imports, unused_variables)]
//! THE BRING-UP'S OWN PART of render_frame: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)

pub use ::game_types::render_frame::types;
pub use types::*;

/// A weather frame with nothing in it. the tick makes one where the host's engine has none (weather.ts:350 `??= noWeather()`).   (render/frame.ts noWeather)
pub(in crate::air_and_light) fn no_weather() -> crate::render_frame::WeatherFrame {
    crate::render_frame::WeatherFrame { rain: 0.0, snow: None, shear_x: 0.0, shear_z: 0.0, wet: 0.0, flash: [0.0, 0.0, 0.0], rain_top: 4.5, interior: [0.0; 4], annex: [0.0; 4], roof_c: None, roof_d: None, fog: None, near_fade: [0.0; 4], phase: None }
}
