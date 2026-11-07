#![allow(unused_variables)]
//! Every function and method of `hours`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/hours.ts:80 hourLabel)
pub fn hour_label(id_2: crate::hours::HourId) -> String { todo!() }

/// (src/game/hours.ts:81 hourClock)
pub fn hour_clock(id_2: crate::hours::HourId) -> String { todo!() }

/// (src/game/hours.ts:82 hourWords)
pub fn hour_words(id_2: crate::hours::HourId) -> String { todo!() }

/// (src/game/hours.ts:88 hourLive)
pub fn hour_live(g: &mut crate::game::Game) -> crate::hours::HourId { todo!() }

/// (src/game/hours.ts:92 hourIdOf)
pub fn hour_id_of(v: Option<&str>) -> crate::hours::HourId { todo!() }

/// (src/game/hours.ts:101 hourFromQuery)
pub fn hour_from_query(g: &mut crate::game::Game, q: crate::Opaque) { todo!() }

/// (src/game/hours.ts:147 hourOf)
pub fn hour_of(g: &mut crate::game::Game) -> crate::hours::HourId { todo!() }

/// (src/game/hours.ts:151 exposureOf)
pub fn exposure_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/hours.ts:156 hourSun)
pub fn hour_sun(g: &mut crate::game::Game) -> Option<LightId> { todo!() }

/// (src/game/hours.ts:157 isHourSun)
pub fn is_hour_sun(g: &mut crate::game::Game, l: LightId) -> bool { todo!() }

/// (src/game/hours.ts:161 inEstimate)
pub fn in_estimate(g: &mut crate::game::Game, l: LightId) -> bool { todo!() }

/// (src/game/hours.ts:164 skyLampOf)
pub fn sky_lamp_of(g: &mut crate::game::Game) -> Option<LightId> { todo!() }

/// (src/game/hours.ts:171 sunStrength)
pub fn sun_strength(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/hours.ts:173 sunGiven)
pub fn sun_given(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/hours.ts:177 setSunStrength)
pub fn set_sun_strength(g: &mut crate::game::Game, k: f64, given: Option<bool>) { todo!() }

/// (src/game/hours.ts:201 cloudSun)
pub fn cloud_sun(c_2: f64) -> f64 { todo!() }

/// (src/game/hours.ts:203 cloudSoft)
pub fn cloud_soft(c_2: f64) -> f64 { todo!() }

/// (src/game/hours.ts:207 greyed)
pub fn greyed(c_2: V3, w: f64, k: Option<f64>) -> V3 { todo!() }

/// (src/game/hours.ts:209 skyShare)
pub fn sky_share(d: &crate::hours::HourDef) -> f64 { todo!() }

/// (src/game/hours.ts:216 cloudLight)
pub fn cloud_light(d: &crate::hours::HourDef, c_2: f64) -> [f64; 2] { todo!() }

/// (src/game/hours.ts:225 cloudTotal)
pub fn cloud_total(d: &crate::hours::HourDef, c_2: f64) -> f64 { todo!() }

/// (src/game/hours.ts:226 underWeather)
pub fn under_weather(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:243 hourWeatherChanged)
pub fn hour_weather_changed(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:245 sunToCloud)
pub fn sun_to_cloud(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:247 lum)
pub fn lum(c_2: V3) -> f64 { todo!() }

/// (src/game/hours.ts:249 toSunOf)
pub fn to_sun_of(d: &crate::hours::HourDef) -> V3 { todo!() }

/// (src/game/hours.ts:252 applyHour)
pub fn apply_hour(g: &mut crate::game::Game, id_2: crate::hours::HourId) -> bool { todo!() }

/// (src/game/hours.ts:261 putIn)
pub fn put_in(g: &mut crate::game::Game, def: &crate::hours::HourDef, from: Option<&crate::hours::HourState>) { todo!() }

/// (src/game/hours.ts:279 takeOut)
pub fn take_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:290 lampsOut)
pub fn lamps_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:296 hourAfterReset)
pub fn hour_after_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:298 hourWorldChanged)
pub fn hour_world_changed(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:301 castersOf)
pub fn casters_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/hours.ts:319 skyLum)
pub fn sky_lum(g: &mut crate::game::Game, y: f64, sky: Option<crate::hours::SkyStateDome>) -> f64 { todo!() }

/// (src/game/hours.ts:323 boxOf)
pub fn box_of(g: &mut crate::game::Game) -> crate::hours::SkyStateBox { todo!() }

/// (src/game/hours.ts:334 reachOf)
pub fn reach_of(b: crate::hours::SkyStateBox, p: V3, d: V3) -> f64 { todo!() }

/// (src/game/hours.ts:343 staticBlocked)
pub fn static_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/hours.ts:349 openRays)
pub fn open_rays(g: &mut crate::game::Game, c_2: V3) -> Vec<crate::hours::Open> { todo!() }

/// (src/game/hours.ts:366 hourLight)
pub fn hour_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) { todo!() }

/// (src/game/hours.ts:377 moonLight)
pub fn moon_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) { todo!() }

/// (src/game/hours.ts:389 moonForget)
pub fn moon_forget(g: &mut crate::game::Game) { todo!() }

/// (src/game/hours.ts:391 skyLight)
pub fn sky_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) { todo!() }

/// (src/game/hours.ts:420 hourLightAt)
pub fn hour_light_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/hours.ts:423 hourSummary)
pub fn hour_summary(g: &mut crate::game::Game) -> String { todo!() }
