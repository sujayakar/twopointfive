#![allow(unused_variables)]
//! Every function and method of `weather`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/weather.ts:66 roofsOf)
pub fn roofs_of(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:75 openGround)
pub fn open_ground(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { todo!() }

/// (src/game/weather.ts:84 skySpots)
pub fn sky_spots(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]], enough: f64, step: f64) -> f64 { todo!() }

/// (src/game/weather.ts:94 skyOver)
pub fn sky_over(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { todo!() }

/// (src/game/weather.ts:97 skyOverMost)
pub fn sky_over_most(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { todo!() }

/// (src/game/weather.ts:100 ownFloor)
pub fn own_floor(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:126 strokeMean)
pub fn stroke_mean(s: &crate::weather::Stroke, a: f64, b: f64) -> f64 { todo!() }

/// (src/game/weather.ts:137 Weather.enabled)
pub fn weather_enabled(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:138 Weather.enabled)
pub fn weather_set_enabled(g: &mut crate::game::Game, v: bool) { todo!() }

/// (src/game/weather.ts:141 Weather.kind)
pub fn weather_kind(g: &mut crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> { todo!() }

/// (src/game/weather.ts:142 Weather.kind)
pub fn weather_set_kind(g: &mut crate::game::Game, k: Option<crate::weather_kinds::WeatherKind>) { todo!() }

/// (src/game/weather.ts:146 Weather.roofs)
pub fn weather_roofs(g: &mut crate::game::Game) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:147 Weather.roofs)
pub fn weather_set_roofs(g: &mut crate::game::Game, r: &[[f64; 4]]) { todo!() }

/// (src/game/weather.ts:154 Weather.fog)
pub fn weather_fog(g: &mut crate::game::Game) -> Option<crate::weather_kinds::FogSpec> { todo!() }

/// (src/game/weather.ts:157 Weather.setFog)
pub fn weather_set_fog(g: &mut crate::game::Game, word: f64) { todo!() }

/// (src/game/weather.ts:162 Weather.thunderUntil)
pub fn weather_thunder_until(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:163 Weather.thunderUntil)
pub fn weather_set_thunder_until(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:166 Weather.cloud)
pub fn weather_cloud(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:168 Weather.storm)
pub fn weather_storm(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:169 Weather.storm)
pub fn weather_set_storm(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:171 Weather.rainRate)
pub fn weather_rain_rate(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:172 Weather.rainRate)
pub fn weather_set_rain_rate(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:179 Weather.wet)
pub fn weather_wet(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:180 Weather.wet)
pub fn weather_set_wet(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:184 Weather.thunders)
pub fn weather_thunders(g: &mut crate::game::Game) -> Vec<crate::weather::Thunder> { todo!() }

/// (src/game/weather.ts:185 Weather.thundersAre)
pub fn weather_set_thunders_are(g: &mut crate::game::Game, l: &[crate::weather::Thunder]) { todo!() }

/// (src/game/weather.ts:187 Weather.thunder)
pub fn weather_thunder(g: &mut crate::game::Game, t: crate::weather::Thunder) { todo!() }

/// (src/game/weather.ts:188 Weather.nextFlashAt)
pub fn weather_next_flash_at(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:189 Weather.nextFlashAt)
pub fn weather_set_next_flash_at(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:193 Weather.die)
pub fn weather_die(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:194 Weather.die)
pub fn weather_set_die(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:195 Weather.lastBarkT)
pub fn weather_last_bark_t(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/weather.ts:196 Weather.lastBarkT)
pub fn weather_set_last_bark_t(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:201 Weather.dryPaint)
pub fn weather_dry_paint(g: &mut crate::game::Game, b: BoxId) -> Option<[f64; 3]> { todo!() }

/// (src/game/weather.ts:204 Weather.constructor)
pub fn weather_new(g: &mut crate::game::Game) -> crate::weather::Weather { todo!() }

/// (src/game/weather.ts:209 Weather.levelKind)
pub fn weather_level_kind(g: &mut crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> { todo!() }

/// (src/game/weather.ts:215 Weather.ownNight)
pub fn weather_own_night(g: &mut crate::game::Game) { todo!() }

/// (src/game/weather.ts:221 Weather.ownStorm)
pub fn weather_own_storm(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:224 Weather.officeFit)
pub fn weather_office_fit(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:233 Weather.syncRoofs)
pub fn weather_sync_roofs(g: &mut crate::game::Game, tell_wind: Option<bool>) { todo!() }

/// (src/game/weather.ts:244 Weather.levelRoofs)
pub fn weather_level_roofs(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:246 Weather.ownRoofs)
pub fn weather_own_roofs(g: &mut crate::game::Game) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:250 Weather.setKind)
pub fn weather_set_kind_2(g: &mut crate::game::Game, kind: Option<crate::weather_kinds::WeatherKind>) -> bool { todo!() }

/// (src/game/weather.ts:273 Weather.refuses)
pub fn weather_refuses(g: &mut crate::game::Game, s: Option<&crate::weather::WeatherSnap>) -> Option<String> { todo!() }

/// (src/game/weather.ts:279 Weather.roofsNow)
pub fn weather_roofs_now(g: &mut crate::game::Game) -> Vec<[f64; 4]> { todo!() }

/// (src/game/weather.ts:281 Weather.underRoof)
pub fn weather_under_roof(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/weather.ts:287 Weather.rainBed)
pub fn weather_rain_bed(g: &mut crate::game::Game, x: f64, z: f64) -> f64 { todo!() }

/// (src/game/weather.ts:294 Weather.hearMask)
pub fn weather_hear_mask(g: &mut crate::game::Game, ear: V3) -> f64 { todo!() }

/// (src/game/weather.ts:301 Weather.airTrans)
pub fn weather_air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/weather.ts:307 Weather.seed)
pub fn weather_seed(g: &mut crate::game::Game, n: f64) { todo!() }

/// (src/game/weather.ts:310 Weather.snapshot)
pub fn weather_snapshot(g: &mut crate::game::Game) -> crate::weather::WeatherSnap { todo!() }

/// (src/game/weather.ts:311 Weather.restore)
pub fn weather_restore(g: &mut crate::game::Game, s: &crate::weather::WeatherSnap) { todo!() }

/// (src/game/weather.ts:324 Weather.setEnabled)
pub fn weather_set_enabled_2(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/weather.ts:332 Weather.strike)
pub fn weather_strike(g: &mut crate::game::Game, opts: Option<crate::weather::WeatherStrikeOpts>) -> Option<crate::weather::Flash> { todo!() }

/// (src/game/weather.ts:361 Weather.thunderDue)
pub fn weather_thunder_due(g: &mut crate::game::Game, i: f64) -> Option<Vec<f64>> { todo!() }

/// (src/game/weather.ts:371 Weather.update)
pub fn weather_update(g: &mut crate::game::Game, dt: f64, cam_yaw: Option<f64>) { todo!() }

/// (src/game/weather.ts:411 Weather.gap)
pub fn weather_gap(g: &mut crate::game::Game, first: bool) -> f64 { todo!() }

/// (src/game/weather.ts:416 Weather.rainNow)
pub fn weather_set_rain_now(g: &mut crate::game::Game, v: f64) { todo!() }

/// (src/game/weather.ts:417 Weather.applyFlash)
pub fn weather_apply_flash(g: &mut crate::game::Game, t: f64, dt: f64) { todo!() }

/// (src/game/weather.ts:420 Weather.endFlash)
pub fn weather_end_flash(g: &mut crate::game::Game) { todo!() }

/// (src/game/weather.ts:423 Weather.playThunder)
pub fn weather_play_thunder(g: &mut crate::game::Game, th: crate::weather::Thunder, dice: &[f64]) { todo!() }

/// (src/game/weather.ts:454 Weather.groundWet)
pub fn weather_ground_wet(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/weather.ts:457 Weather.setWet)
pub fn weather_set_wet_2(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/weather.ts:485 Weather.teardown)
pub fn weather_teardown(g: &mut crate::game::Game) { todo!() }

/// (src/game/weather.ts:492 Weather.summary)
pub fn weather_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/weather.ts:500 dirOf)
pub fn dir_of(bearing: f64, elev: f64) -> V3 { todo!() }

/// (src/game/weather.ts:504 zero)
pub fn zero(f: &mut crate::render_frame::WeatherFrame) { todo!() }

/// (src/game/weather.ts:508 hearPath)
pub fn hear_path(g: &mut crate::game::Game, src: V3, ear: V3, keep: Option<bool>) -> crate::scene_collision::Propagation { todo!() }

/// (src/game/weather.ts:514 loudLen)
pub fn loud_len(g: &mut crate::game::Game, path_len: f64, ear: V3) -> f64 { todo!() }

/// (src/game/weather.ts:519 strokeLight)
pub fn stroke_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) { todo!() }

/// (src/game/weather.ts:529 airTrans)
pub fn air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }
