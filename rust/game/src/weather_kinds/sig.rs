#![allow(unused_variables)]
//! Every function and method of `weather_kinds`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/weatherKinds.ts:50 isWeatherKind)
pub fn is_weather_kind(s: Option<&str>) -> bool { todo!() }

/// (src/game/weatherKinds.ts:108 snowLevel)
pub fn snow_level(g: &mut crate::game::Game, name: Option<&str>) -> bool { todo!() }

/// (src/game/weatherKinds.ts:110 snowLevelOfQuery)
pub fn snow_level_of_query(g: &mut crate::game::Game, q: crate::Opaque) -> bool { todo!() }

/// (src/game/weatherKinds.ts:112 isAnyKind)
pub fn is_any_kind(g: &mut crate::game::Game, s: Option<crate::weather_kinds::WeatherKind>) -> bool { todo!() }

/// (src/game/weatherKinds.ts:113 weatherTitle)
pub fn weather_title(g: &mut crate::game::Game, k: Option<crate::weather_kinds::WeatherKind>) -> String { todo!() }

/// (src/game/weatherKinds.ts:114 weatherWords)
pub fn weather_words(g: &mut crate::game::Game, k: Option<crate::weather_kinds::WeatherKind>) -> String { todo!() }

/// (src/game/weatherKinds.ts:117 q)
pub fn q(x: f64, step: f64, max: f64) -> f64 { todo!() }

/// (src/game/weatherKinds.ts:118 packFog)
pub fn pack_fog(f: Option<crate::weather_kinds::FogSpec>) -> f64 { todo!() }

/// (src/game/weatherKinds.ts:126 fogGlowWord)
pub fn fog_glow_word(w: f64, k: f64) -> f64 { todo!() }

/// (src/game/weatherKinds.ts:131 unpackFog)
pub fn unpack_fog(w: f64) -> Option<crate::weather_kinds::FogSpec> { todo!() }

/// (src/game/weatherKinds.ts:136 fogTau)
pub fn fog_tau(f: crate::weather_kinds::FogSpec, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/weatherKinds.ts:145 packRect)
pub fn pack_rect(r: Option<[f64; 4]>) -> f64 { todo!() }

/// (src/game/weatherKinds.ts:151 unpackRect)
pub fn unpack_rect(w: f64) -> [f64; 4] { todo!() }

/// (src/game/weatherKinds.ts:152 inRect)
pub fn in_rect(r: &[f64], x: f64, z: f64) -> bool { todo!() }

/// (src/game/weatherKinds.ts:174 weatherOfSearch)
pub fn weather_of_search(search: &crate::weather_kinds::WeatherOfSearchSearch) -> Option<crate::weather_kinds::WeatherKind> { todo!() }

/// (src/game/weatherKinds.ts:179 initWeather)
pub fn init_weather(g: &mut crate::game::Game, qs: crate::Opaque) -> Option<crate::weather_kinds::WeatherKind> { todo!() }

/// (src/game/weatherKinds.ts:186 weatherSearch)
pub fn weather_search(search: &str, kind: Option<crate::weather_kinds::WeatherKind>) -> String { todo!() }
