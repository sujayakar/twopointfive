#![allow(unused_variables)]
//! Every function and method of `weather`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/weather.ts:65 roofsOf)
pub fn roofs_of(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:78 openGround)
pub fn open_ground(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { todo!() }

/// (src/game/weather.ts:87 skySpots)
pub fn sky_spots(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]], enough: f64, step: f64) -> f64 { todo!() }

/// (src/game/weather.ts:97 skyOver)
pub fn sky_over(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { todo!() }

/// (src/game/weather.ts:100 skyOverMost)
pub fn sky_over_most(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { todo!() }

/// (src/game/weather.ts:103 ownFloor)
pub fn own_floor(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:131 rng32)
pub fn rng32(seed: f64) -> crate::weather::WeatherDice { todo!() }

/// (src/game/weather.ts:138 strokeMean)
pub fn stroke_mean(s: &crate::weather::Stroke, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/weather.ts:144 strokeAt)
pub fn stroke_at(s: &crate::weather::Stroke, t: f64) -> f64 { todo!() }

/// (src/game/weather.ts:151 Weather.enabled)
pub fn weather_enabled(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:152 Weather.enabled)
pub fn weather_set_enabled(g: &mut crate::game::Game, v: bool) { todo!() }

/// (src/game/weather.ts:168 Weather.cloud)
pub fn weather_cloud(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:186 Weather.rnd)
pub fn weather_rnd(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:192 Weather.dryPaint)
pub fn weather_dry_paint(g: &mut crate::game::Game, b: BoxId) -> Option<[f64; 3]> { todo!() }

/// (src/game/weather.ts:195 Weather.constructor)
pub fn weather_new(g: &mut crate::game::Game) -> crate::weather::Weather { todo!() }

/// (src/game/weather.ts:198 Weather.levelKind)
pub fn weather_level_kind(g: &mut crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> { todo!() }

/// (src/game/weather.ts:204 Weather.ownNight)
pub fn weather_own_night(g: &mut crate::game::Game) { todo!() }

/// (src/game/weather.ts:210 Weather.ownStorm)
pub fn weather_own_storm(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:213 Weather.officeFit)
pub fn weather_office_fit(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:222 Weather.syncRoofs)
pub fn weather_sync_roofs(g: &mut crate::game::Game, tell_wind: Option<bool>) { todo!() }

/// (src/game/weather.ts:233 Weather.levelRoofs)
pub fn weather_level_roofs(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:235 Weather.ownRoofs)
pub fn weather_own_roofs(g: &mut crate::game::Game) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:239 Weather.setKind)
pub fn weather_set_kind(g: &mut crate::game::Game, kind: Option<crate::weather_kinds::WeatherKind>) -> bool { todo!() }

/// (src/game/weather.ts:262 Weather.refuses)
pub fn weather_refuses(g: &mut crate::game::Game, s: Option<&crate::weather::WeatherSnap>) -> Option<String> { todo!() }

/// (src/game/weather.ts:268 Weather.roofsNow)
pub fn weather_roofs_now(g: &mut crate::game::Game) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:270 Weather.underRoof)
pub fn weather_under_roof(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/weather.ts:272 Weather.rainBed)
pub fn weather_rain_bed(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/game/weather.ts:273 Weather.simLive)
pub fn weather_sim_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:275 Weather.hearMask)
pub fn weather_hear_mask(g: &mut crate::game::Game, ear: V3) -> f64 { todo!() }

/// (src/game/weather.ts:283 Weather.airTrans)
pub fn weather_air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/weather.ts:293 Weather.seed)
pub fn weather_seed(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/weather.ts:296 Weather.snapshot)
pub fn weather_snapshot(g: &mut crate::game::Game) -> crate::weather::WeatherSnap { todo!() }

/// (src/game/weather.ts:297 Weather.restore)
pub fn weather_restore(g: &mut crate::game::Game, s: &crate::weather::WeatherSnap) { todo!() }

/// (src/game/weather.ts:310 Weather.setEnabled)
pub fn weather_set_enabled_2(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/weather.ts:318 Weather.strike)
pub fn weather_strike(g: &mut crate::game::Game, opts: Option<crate::weather::WeatherStrikeOpts>) -> Option<crate::weather::Flash> { todo!() }

/// (src/game/weather.ts:347 Weather.update)
pub fn weather_update(g: &mut crate::game::Game, dt: f64, cam_yaw: Option<f64>) { todo!() }

/// (src/game/weather.ts:385 Weather.gap)
pub fn weather_gap(g: &mut crate::game::Game, first: bool) -> f64 { todo!() }

/// (src/game/weather.ts:390 Weather.applyFlash)
pub fn weather_apply_flash(g: &mut crate::game::Game, t: f64, dt: f64) { todo!() }

/// (src/game/weather.ts:393 Weather.endFlash)
pub fn weather_end_flash(g: &mut crate::game::Game) { todo!() }

/// (src/game/weather.ts:395 Weather.playThunder)
pub fn weather_play_thunder(g: &mut crate::game::Game, th: crate::weather::Thunder) { todo!() }

/// (src/game/weather.ts:427 Weather.groundWet)
pub fn weather_ground_wet(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:430 Weather.setWet)
pub fn weather_set_wet(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/weather.ts:458 Weather.teardown)
pub fn weather_teardown(g: &mut crate::game::Game) { todo!() }

/// (src/game/weather.ts:465 Weather.summary)
pub fn weather_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/weather.ts:473 dirOf)
pub fn dir_of(bearing: f64, elev: f64) -> V3 { todo!() }

/// (src/game/weather.ts:477 zero)
pub fn zero(f: &mut crate::render_frame::WeatherFrame) { todo!() }

/// (src/game/weather.ts:481 hearPath)
pub fn hear_path(g: &mut crate::game::Game, src: V3, ear: V3, keep: Option<bool>) -> crate::scene_collision::Propagation { todo!() }

/// (src/game/weather.ts:487 loudLen)
pub fn loud_len(g: &mut crate::game::Game, path_len: f64, ear: V3) -> f64 { todo!() }

/// (src/game/weather.ts:492 strokeLight)
pub fn stroke_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) { todo!() }

/// (src/game/weather.ts:502 airTrans)
pub fn air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }
