#![allow(unused_variables)]
//! Every function and method of `render_frame`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/render/frame.ts:93 noWeather)
pub fn no_weather() -> crate::render_frame::WeatherFrame { todo!() }

/// (src/render/frame.ts:399 weatherVolWgsl)
pub fn weather_vol_wgsl(g: &mut crate::game::Game) -> String { todo!() }

/// (src/render/frame.ts:404 groundRayWgsl)
pub fn ground_ray_wgsl(g: &mut crate::game::Game) -> String { todo!() }

/// (src/render/frame.ts:411 roundRayWgsl)
pub fn round_ray_wgsl(g: &mut crate::game::Game, cut: bool) -> String { todo!() }

/// (src/render/frame.ts:415 commonSource)
pub fn common_source(g: &mut crate::game::Game) -> String { todo!() }

/// (src/render/frame.ts:418 scenePrelude)
pub fn scene_prelude(g: &mut crate::game::Game, cut: Option<bool>) -> String { todo!() }
