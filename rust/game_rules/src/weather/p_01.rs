//! src/game/weather.ts lines 1 to 496
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- this file's reach of the module's table and of a box

/// The weather's table in the game's block (rust/sim/src/weather.rs). Take it for one read, write or call: every accessor below is this.
fn wx(g: &Game) -> &'static mut ::sim::weather::Weather { ::sim::weather::of(g.sim) }
/// The same table, to read.
fn wx_read(g: &Game) -> &'static ::sim::weather::Weather { ::sim::weather::of_read(g.sim) }

/// `WX_KNOBS.here()`: the module's knobs as they stand, unless this tick has handed the list. rustknobs.ts:59 sets it and it needs `g`, which the skeleton's `here()` has not, so this is p_air_dust_01's line (notes, Skeleton; all the knobs, where the TypeScript writes the weather's).
fn knobs_here(g: &mut Game) {
    if !g.m.rustknobs.rust_knobs.tick { crate::rustknobs::knobs_to_rust(g); }
}

/// `console.warn` as the weather stands up (after-snapshot.md: a line, no refusal). The skeleton has no console, so it says nothing yet (notes, Skeleton).
fn seam_console_warn(_line: String) {}

/// A box's row (Box.* in the TypeScript): the one seam left. Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows; where the table comes back this is the one line `&mut g.<table>[b]`.
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// `g.level.boxes` as IDs: that list and `g.col.boxes` are one array in the TypeScript (game.ts:1219), statics appended later included. `level.boxes` is not a field in Rust.
fn level_boxes(g: &Game) -> Vec<BoxId> { g.col.boxes.clone() }

/// Paints a box's row.
fn paint(g: &mut Game, b: BoxId, albedo: V3, flags: f64, rough: Option<f64>, aniso: Option<bool>) {
    let x = seam_box(g, b);
    x.albedo = albedo; x.flags = flags; x.rough = rough; x.aniso = aniso;
}

/// `WEATHER_PRESETS[kind]`.
fn preset(g: &Game, kind: crate::weather_kinds::WeatherKind) -> &crate::weather_kinds::WeatherPreset { g.m.weather_kinds.weather_presets.get(kind).unwrap() }

// ---- the file's free functions

/// The level's roofs as footprints: its ceiling slabs, biggest first, at most MAX_ROOFS. (weather.ts roofsOf)
pub fn roofs_of(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<[f64; 4]> {
    let ceiling_y = crate::scene_world::world().ceiling_y;
    ::sim::weather::roofs_begin();
    for &b in boxes {
        let (c, h) = (seam_box(g, b).c, seam_box(g, b).h);
        ::sim::weather::roofs_box(c[0], c[1], c[2], h[0], h[1], h[2], ceiling_y);
    }
    let mut v = [0.0; ::sim::weather::ROOFS * 4];
    let n = ::sim::weather::roofs_end(&mut v);
    let mut out: Vec<[f64; 4]> = Vec::new();
    for i in 0..n { out.push([v[i * 4], v[i * 4 + 1], v[i * 4 + 2], v[i * 4 + 3]]); }
    out
}

/// A level that carries its own places: the office's footprints are not its building's. (weather.ts ownFloor)
pub fn own_floor(g: &crate::game::Game) -> bool {
    match &g.level.scenarios { Some(s) => s.quiet.is_some(), None => false }
}

/// Mean of the stroke's envelope over [a, b]: the module's, its pulses handed over as three doubles each. (weather.ts strokeMean)
pub fn stroke_mean(s: &crate::weather::Stroke, a: f64, b: f64) -> f64 {
    let n = s.pulses.len();
    let mut v: Vec<f64> = Vec::with_capacity(n * 3);
    for p in &s.pulses { v.push(p.t0); v.push(p.amp); v.push(p.tau); }
    ::sim::weather::stroke_mean(&v, n, a, b)
}

// ---- class Weather

/// Master switch, read. (weather.ts Weather.enabled)
pub fn weather_enabled(g: &crate::game::Game) -> bool { g.weather.on }

/// The weather the level was started in. (weather.ts Weather.kind)
pub fn weather_kind(g: &crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> { g.weather.kind_now }

/// Sets the kind and the module's `live` with it. Not setKind, which is `weather_set_kind_2`. (weather.ts Weather.kind)
pub fn weather_set_kind(g: &mut crate::game::Game, k: Option<crate::weather_kinds::WeatherKind>) {
    g.weather.kind_now = k;
    wx(g).live = if k.is_some() { 1.0 } else { 0.0 };
}

/// The no-rain footprints in force. (weather.ts Weather.roofs)
pub fn weather_roofs(g: &crate::game::Game) -> Vec<[f64; 4]> { g.weather.roof_rects.clone() }

/// The footprints, into the table the rules read: its one writer. (weather.ts Weather.roofs)
pub fn weather_set_roofs(g: &mut crate::game::Game, r: &[[f64; 4]]) {
    let n = r.len();
    if n > ::sim::weather::ROOFS { panic!("weather: {} roofs, and the table holds {}", n, ::sim::weather::ROOFS); }
    let w = wx(g);
    for i in 0..n { for k in 0..4 { w.roofs[i * 4 + k] = r[i][k]; } }
    w.roofs_n = n as f64;
    g.weather.roof_rects = r.to_vec();
}

/// The fog in the air, as the renderer's 32 bits decode it. (weather.ts Weather.fog)
pub fn weather_fog(g: &crate::game::Game) -> Option<crate::weather_kinds::FogSpec> { g.weather.fog_spec }

/// The fog's word, and what an eye reads of it into the module's table. (weather.ts Weather.setFog)
pub fn weather_set_fog(g: &mut crate::game::Game, word: f64) {
    let f = crate::weather_kinds::unpack_fog(word);
    g.weather.fog_word = word;
    g.weather.fog_spec = f;
    let w = wx(g);
    w.fog_on = if f.is_some() { 1.0 } else { 0.0 };
    w.fog_sigma = match f { Some(f) => f.sigma, None => 0.0 };
    w.fog_h = match f { Some(f) => f.h, None => 0.0 };
    w.fog_leak = match f { Some(f) => f.leak, None => 0.0 };
}

/// Game time until which a thunderclap is drowning the floor's small noises. (weather.ts Weather.thunderUntil)
pub fn weather_thunder_until(g: &crate::game::Game) -> f64 { wx_read(g).thunder_until }

/// (weather.ts Weather.thunderUntil)
pub fn weather_set_thunder_until(g: &mut crate::game::Game, v: f64) { wx(g).thunder_until = v; }

/// Cloud cover 0..1 for whoever owns the sun and the moon to read. (weather.ts Weather.cloud)
pub fn weather_cloud(g: &crate::game::Game) -> f64 {
    if !g.weather.on { return 0.0; }
    match g.weather.kind_now { Some(k) => preset(g, k).cloud, None => legacy().cloud }
}

/// Storm intensity 0..1. (weather.ts Weather.storm)
pub fn weather_storm(g: &crate::game::Game) -> f64 { wx_read(g).storm }

/// (weather.ts Weather.storm)
pub fn weather_set_storm(g: &mut crate::game::Game, v: f64) { wx(g).storm = v; }

/// Rain rate 0..1. (weather.ts Weather.rainRate)
pub fn weather_rain_rate(g: &crate::game::Game) -> f64 { wx_read(g).rain_rate }

/// (weather.ts Weather.rainRate)
pub fn weather_set_rain_rate(g: &mut crate::game::Game, v: f64) { wx(g).rain_rate = v; }

/// Current wetness 0..1. (weather.ts Weather.wet)
pub fn weather_wet(g: &crate::game::Game) -> f64 { wx_read(g).wet }

/// Sets the wetness. Not setWet, which is `weather_set_wet_2`. (weather.ts Weather.wet)
pub fn weather_set_wet(g: &mut crate::game::Game, v: f64) { wx(g).wet = v; }

/// The thunder on its way, the module's list as it stands. (weather.ts Weather.thunders)
pub fn weather_thunders(g: &crate::game::Game) -> Vec<crate::weather::Thunder> {
    let w = wx_read(g);
    let mut out = Vec::new();
    let mut i = 0usize;
    while (i as f64) < w.thunders_n {
        out.push(Thunder { at: w.thunders[i * 4], dist_km: w.thunders[i * 4 + 1], bearing: w.thunders[i * 4 + 2], loud: w.thunders[i * 4 + 3] });
        i += 1;
    }
    out
}

/// The module's list made this one. (weather.ts Weather.thundersAre)
pub fn weather_set_thunders_are(g: &mut crate::game::Game, l: &[crate::weather::Thunder]) {
    let w = wx(g);
    w.thunders_n = 0.0;
    for t in l { ::sim::weather::thunder_push(w, t.at, t.dist_km, t.bearing, t.loud); }
}

/// When the next stroke is due (game time). (weather.ts Weather.nextFlashAt)
pub fn weather_next_flash_at(g: &crate::game::Game) -> f64 { wx_read(g).next_flash_at }

/// (weather.ts Weather.nextFlashAt)
pub fn weather_set_next_flash_at(g: &mut crate::game::Game, v: f64) { wx(g).next_flash_at = v; }

/// The storm's own die: a word of the module's table. (weather.ts Weather.die)
pub fn weather_die(g: &crate::game::Game) -> f64 { wx_read(g).dice as f64 }

/// A store into the word: the Uint32Array's own conversion. (weather.ts Weather.die)
pub fn weather_set_die(g: &mut crate::game::Game, v: f64) { wx(g).dice = js::to_u32(v); }

/// When the last remark was made. (weather.ts Weather.lastBarkT)
pub fn weather_last_bark_t(g: &crate::game::Game) -> f64 { wx_read(g).last_bark_t }

/// (weather.ts Weather.lastBarkT)
pub fn weather_set_last_bark_t(g: &mut crate::game::Game, v: f64) { wx(g).last_bark_t = v; }

/// The weather a new game is born in. Runs on `g.weather` (every method's path) and returns what the caller stores there. (weather.ts Weather.constructor)
pub fn weather_new(g: &mut crate::game::Game) -> crate::weather::Weather {
    g.weather = Weather::blank();
    // g.sim.keep(WX_BIT, 'weather'): the slot's block stands already, so only the bit and its one check are left (as p_sim_lightshome_01's LightHome)
    let bit = crate::sim_weatherhome::WX_BIT as u32;
    let head = &mut ::sim::state::state(g.sim).head;
    if head.homes & bit != 0 { panic!("two stores of the weather over one game's block: a part of a block is one store's"); }
    head.homes |= bit;
    {
        let ceiling_y = crate::scene_world::world().ceiling_y;
        let mut items: Vec<crate::sim_weatherhome::RoofsCountedBoxesItem> = Vec::new();
        for b in level_boxes(g) {
            let (c, h) = (seam_box(g, b).c, seam_box(g, b).h);
            items.push(crate::sim_weatherhome::RoofsCountedBoxesItem { c: c.to_vec(), h: h.to_vec() });
        }
        let (has, most) = (crate::sim_weatherhome::roofs_counted(&items, ceiling_y), ::sim::weather::ROOFS as f64);
        if has > most { seam_console_warn(format!("the level has {} ceiling slabs and the weather's table holds {}: the biggest {} are its roofs, and rain and wind pass the rest", has, most, most)); }
    }
    let l = legacy();
    weather_set_storm(g, l.storm);
    weather_set_rain_rate(g, l.rain);
    weather_set_thunder_until(g, -1e9);
    weather_set_next_flash_at(g, -1.0);
    weather_set_last_bark_t(g, -1e9);
    weather_set_die(g, 20260809.0);
    weather_set_roofs(g, &[interior(), annex()]);
    let stood = g.stood;
    let born = match weather_level_kind(g) {
        Some(k) => Some(k),
        None => if stood.weather == Some(crate::weather_kinds::WeatherKind::Snow) { None } else { stood.weather },
    };
    if born.is_none() && own_floor(g) { weather_own_night(g); }
    let wet = if weather_rain_rate(g) > 0.0 { 1.0 } else { 0.0 };
    weather_set_wet(g, wet);
    if born.is_some() { set_kind_at(g, born, false); } else { sync_roofs(g, false, false); }
    g.weather.clone()
}

/// A level built for snow is born in the kind 'snow' and stays in it; None = any other level. (weather.ts Weather.levelKind)
pub fn weather_level_kind(g: &crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> {
    let name = crate::scene_level_load::level_name_of(g);
    let stood = g.stood;
    if crate::weather_kinds::snow_level(g, name.as_deref()) && (!stood.locked || stood.weather == Some(crate::weather_kinds::WeatherKind::Snow)) { Some(crate::weather_kinds::WeatherKind::Snow) } else { None }
}

/// No weather picked on a floor that is none of the office's: its own night is dry and still, under its own roofs. (weather.ts Weather.ownNight)
pub fn weather_own_night(g: &mut crate::game::Game) {
    weather_set_storm(g, 0.0);
    weather_set_rain_rate(g, 0.0);
    g.weather.wind = [0.0, 0.0];
    weather_set_fog(g, 0.0);
    let roofs = weather_own_roofs(g);
    weather_set_roofs(g, &roofs);
}

/// The level's own storm is in force, cut by the level's own roofs (WX_OWN.roofs). (weather.ts Weather.ownStorm)
pub fn weather_own_storm(g: &crate::game::Game) -> bool { g.weather.own }

/// Do the office's two rectangles fit this level's roofs (asked once). (weather.ts Weather.officeFit)
pub fn weather_office_fit(g: &mut crate::game::Game) -> bool {
    if let Some(f) = g.weather.fits { return f; }
    let boxes = level_boxes(g);
    let fits = if boxes.is_empty() {
        true   // (no level at all, a bare stage, is nobody's building: as it was)
    } else {
        let r = roofs_of(g, &boxes);
        r.len() == 2 && near(&r[0], &interior()) && near(&r[1], &annex())
    };
    g.weather.fits = Some(fits);
    fits
}

/// Two footprints within 0.3 m in every number. (weather.ts Weather.officeFit near)
fn near(a: &[f64; 4], b: &[f64; 4]) -> bool {
    a.iter().enumerate().all(|(i, v)| m::abs(*v - b[i]) < 0.3)
}

/// The footprints follow the rule in force. (weather.ts Weather.syncRoofs)
pub fn weather_sync_roofs(g: &mut crate::game::Game, tell_wind: Option<bool>) { sync_roofs(g, tell_wind.unwrap_or(false), true) }

/// syncRoofs, with `built` false while the constructor runs: `g.wind` is built after this object and has nothing to be told yet.
fn sync_roofs(g: &mut Game, tell_wind: bool, built: bool) {
    let own = g.weather.kind_now.is_none() && !own_floor(g) && !weather_office_fit(g);
    if own == g.weather.own && !tell_wind { return; }
    if own != g.weather.own {
        g.weather.own = own;
        weather_set_wet_2(g, false);   // (what is wet hangs on the roofs: put the ground back, the next update wets it by the new rule)
        if g.weather.kind_now.is_none() {
            let roofs = if own { weather_own_roofs(g) } else { vec![interior(), annex()] };
            weather_set_roofs(g, &roofs);
        }
    }
    if !built { return; }
    let roofs: Option<Vec<[f64; 4]>> = if g.weather.kind_now.is_some() || own { Some(g.weather.roof_rects.clone()) } else { None };
    crate::wind::wind_reroof(g, roofs.as_deref(), Some(own));
}

/// `roofs` are the level's own and in force. (weather.ts Weather.levelRoofs)
pub fn weather_level_roofs(g: &crate::game::Game) -> bool { g.weather.kind_now.is_some() || g.weather.own }

/// The level's own ceiling slabs as the no-rain footprints (roofs 0 / 1 exact, 2 / 3 as the Frame packs them). (weather.ts Weather.ownRoofs)
pub fn weather_own_roofs(g: &mut crate::game::Game) -> Vec<[f64; 4]> {
    let boxes = level_boxes(g);
    roofs_of(g, &boxes).into_iter().enumerate().map(|(i, r)| if i < 2 { r } else { crate::weather_kinds::unpack_rect(crate::weather_kinds::pack_rect(Some(r))) }).collect()
}

/// Put the level in a weather (None = back as it has always been). False = refused. (weather.ts Weather.setKind)
pub fn weather_set_kind_2(g: &mut crate::game::Game, kind: Option<crate::weather_kinds::WeatherKind>) -> bool { set_kind_at(g, kind, true) }

/// setKind, with `built` false while the constructor runs: `g.wind` and `g.airDust` are built after this object.
fn set_kind_at(g: &mut Game, kind: Option<crate::weather_kinds::WeatherKind>, built: bool) -> bool {
    let own = weather_level_kind(g);
    if own.is_some() && kind != own { return false; }   // a level built for snow stays in it
    if g.stood.locked && kind != g.stood.weather { return false; }
    weather_set_wet_2(g, false);   // (what is wet depends on the kind)
    weather_set_kind(g, kind);
    g.weather.own = false;
    g.weather.snow_rate = 0.0;
    if kind.is_some() && !g.weather.on { g.weather.on = true; }   // (a picked weather IS on)
    weather_end_flash(g);
    weather_set_thunders_are(g, &[]);
    weather_set_next_flash_at(g, -1.0);
    weather_set_thunder_until(g, -1e9);
    if let Some(k) = kind {
        let (storm, rain, wind, snow, fog) = { let p = preset(g, k); (p.storm, p.rain, p.wind, p.snow, p.fog) };
        weather_set_storm(g, storm);
        weather_set_rain_rate(g, rain);
        g.weather.wind = [wind[0], wind[1]];
        g.weather.snow_rate = snow.unwrap_or(0.0);
        weather_set_fog(g, crate::weather_kinds::pack_fog(fog));
        let roofs = weather_own_roofs(g);
        weather_set_roofs(g, &roofs);
    } else if own_floor(g) {
        weather_own_night(g);
    } else {
        let l = legacy();
        weather_set_storm(g, l.storm);
        weather_set_rain_rate(g, l.rain);
        g.weather.wind = [l.wind[0], l.wind[1]];
        weather_set_fog(g, 0.0);
        weather_set_roofs(g, &[interior(), annex()]);
    }
    let wet = if g.weather.on && weather_rain_rate(g) > 0.0 { 1.0 } else { 0.0 };
    weather_set_wet(g, wet);
    sync_roofs(g, true, built);
    if built { crate::air_dust::air_dust_reseed(g); }
    crate::hours::hour_weather_changed(g);
    true
}

/// The footprints in force now, as underRoof reads them. (weather.ts Weather.roofsNow)
pub fn weather_roofs_now(g: &mut crate::game::Game) -> Vec<[f64; 4]> {
    weather_sync_roofs(g, None);
    g.weather.roof_rects.clone()
}

/// Is (x, z) under one of the footprints the rain is cut by. (weather.ts Weather.underRoof)
pub fn weather_under_roof(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    weather_sync_roofs(g, None);
    ::sim::weather::under_roof(wx_read(g), x, z)
}

/// The rain bed's level at a listener (0..1). (weather.ts Weather.rainBed)
pub fn weather_rain_bed(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    weather_sync_roofs(g, None);
    knobs_here(g);
    ::sim::weather::rain_bed(wx_read(g), x, z)
}

/// HEARING: the factor (>= 1) a sound's path is stretched by for a listener at `ear`. (weather.ts Weather.hearMask)
pub fn weather_hear_mask(g: &mut crate::game::Game, ear: V3) -> f64 {
    if g.weather.kind_now.is_some() { knobs_here(g); }   // (no kind picked reads no knob: exactly 1)
    ::sim::weather::hear_mask(wx_read(g), g.time, ear[0], ear[2])
}

/// SIGHT: the fog's transmittance along a to b; exactly 1 with no kind picked or no fog in it. (weather.ts Weather.airTrans)
pub fn weather_air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    if g.weather.fog_spec.is_some() && g.weather.kind_now.is_some() { crate::air_dust::air_dust_ready(g); }   // (the field stands before it is read: it is built when first asked)
    ::sim::weather::air_trans(wx_read(g), a, b)
}

/// The weather's dice, reseeded. (weather.ts Weather.seed)
pub fn weather_seed(g: &mut crate::game::Game, n: f64) {
    weather_set_die(g, js::to_u32(n) as f64);
    weather_set_next_flash_at(g, -1.0);
}

/// Everything the storm's next minutes hang on; a stroke in progress is not carried. hours.ts holds a level's own storm in one, so it is a rule's call and not only a checkpoint's. (weather.ts Weather.snapshot)
pub fn weather_snapshot(g: &crate::game::Game) -> crate::weather::WeatherSnap {
    let kind = g.weather.kind_now;
    WeatherSnap {
        kind,
        thunder_until: if kind.is_some() { Some(weather_thunder_until(g)) } else { None },
        on: g.weather.on,
        storm: weather_storm(g),
        rain_rate: weather_rain_rate(g),
        wind: [g.weather.wind[0], g.weather.wind[1]],
        wet_gloss: g.weather.wet_gloss,
        wet: weather_wet(g),
        next_flash_at: weather_next_flash_at(g),
        thunders: weather_thunders(g),
        last_bark_t: weather_last_bark_t(g),
        rng: weather_die(g),
        stats: WeatherSnapStats { flashes: g.weather.stats.flashes, thunders: g.weather.stats.thunders, barks: g.weather.stats.barks },
    }
}

/// `Number.isFinite(x) ? x : d`. (weather.ts Weather.restore f)
fn finite_or(x: f64, d: f64) -> f64 { if x.is_finite() { x } else { d } }

/// The storm as a snapshot had it; on / off stays the session's. (weather.ts Weather.restore)
pub fn weather_restore(g: &mut crate::game::Game, s: &crate::weather::WeatherSnap) {
    weather_end_flash(g);
    let k = if crate::weather_kinds::is_any_kind(g, s.kind) { s.kind } else { None };
    if k != g.weather.kind_now { weather_set_kind_2(g, k); }   // the kind is the file's: the run it holds was played under it
    let own = weather_level_kind(g);
    if own.is_some() && k != own { return; }   // made in other weather on a level that keeps its own: none of its rain, wind or wet comes in
    weather_set_thunder_until(g, finite_or(s.thunder_until.unwrap_or(-1e9), -1e9));
    let storm = finite_or(s.storm, weather_storm(g));
    weather_set_storm(g, storm);
    let rain = finite_or(s.rain_rate, weather_rain_rate(g));
    weather_set_rain_rate(g, rain);
    g.weather.wind = [finite_or(s.wind[0], g.weather.wind[0]), finite_or(s.wind[1], g.weather.wind[1])];
    g.weather.wet_gloss = s.wet_gloss;
    let wet = finite_or(s.wet, weather_wet(g));
    weather_set_wet(g, wet);
    weather_set_next_flash_at(g, finite_or(s.next_flash_at, -1.0));
    let thunders: Vec<Thunder> = s.thunders.iter().map(|t| Thunder { at: finite_or(t.at, 0.0), dist_km: finite_or(t.dist_km, 1.0), bearing: finite_or(t.bearing, 0.0), loud: finite_or(t.loud, 0.5) }).collect();
    weather_set_thunders_are(g, &thunders);
    weather_set_last_bark_t(g, finite_or(s.last_bark_t, -1e9));
    if s.rng.is_finite() { weather_set_die(g, js::to_u32(s.rng) as f64); }
    let w = &mut g.weather;
    w.stats.flashes = js::to_i32(s.stats.flashes) as f64;
    w.stats.thunders = js::to_i32(s.stats.thunders) as f64;
    w.stats.barks = js::to_i32(s.stats.barks) as f64;
    weather_sync_roofs(g, None);   // a replay's knobs are put in after this object was made: the footprints follow the rule now in force
}

/// Switch the storm on or off; a picked weather stays on. (weather.ts Weather.setEnabled)
pub fn weather_set_enabled_2(g: &mut crate::game::Game, on: bool) {
    if on == g.weather.on { return; }
    if !on && g.weather.kind_now.is_some() { return; }
    g.weather.on = on;
    if !on { weather_teardown(g); } else { let wet = if weather_rain_rate(g) > 0.0 { 1.0 } else { 0.0 }; weather_set_wet(g, wet); }
}

/// Fire a stroke now. The Flash returned is a copy; the live one is `g.weather.flash`. (weather.ts Weather.strike)
pub fn weather_strike(g: &mut crate::game::Game, opts: Option<crate::weather::WeatherStrikeOpts>) -> Option<crate::weather::Flash> {
    let opts = opts.unwrap_or_default();
    if !g.weather.on || g.weather.suspended { return None; }
    if g.weather.flash.is_some() { weather_end_flash(g); }
    // the dice and the thunder are the module's. What was given is a number, what was not is thrown for
    let mut said = [0.0; 8];
    ::sim::weather::strike(wx(g), g.time, opts.bearing.unwrap_or(f64::NAN), opts.elev_deg.unwrap_or(f64::NAN), opts.dist_km.unwrap_or(f64::NAN), opts.peak.unwrap_or(f64::NAN), &mut said);
    let (bearing, elev, dist_km, peak) = (said[0], said[1], said[2], said[3]);
    // two return strokes, sometimes a third flicker
    let mut stroke = Stroke { pulses: vec![StrokePulsesItem { t0: 0.0, amp: 1.0, tau: 0.022 }, StrokePulsesItem { t0: 0.085, amp: said[4], tau: 0.05 }] };
    if said[5] != 0.0 { stroke.pulses.push(StrokePulsesItem { t0: said[6], amp: said[7], tau: 0.03 }); }
    let dir = dir_of(bearing, elev);
    let light = g.tab.light.push(crate::render_lights::RtLight {
        name: String::from("lightning"), group: String::from("weather"), kind: 2.0, pos: [0.0, 0.0, 0.0], dir, color: v3::copy(flash_color()), intensity: 0.0, range: 1e6, inner_deg: 0.0, outer_deg: 0.0,
        radius: 0.006, volumetric: 1.3, owner: 0.0, enabled: true, fixture_box: -1.0, flicker: 0.0, ttl: -1.0, age: 0.0, peak_intensity: peak, transient: Some(true),
        ..crate::render_lights::RtLight::zeroed()   // (every field the literal leaves absent)
    });
    crate::render_lights::light_set_add(g, light);
    g.weather.flash = Some(Flash { light, stroke, t: 0.0, peak, dir, bearing, dist_km });
    g.weather.stats.flashes += 1.0;
    weather_apply_flash(g, 0.0, 1.0 / 60.0);   // lit from this very frame
    g.weather.flash.clone()
}

/// One thunder, asked: due and gone from the list (its numbers and four dice), or None. (weather.ts Weather.thunderDue)
pub fn weather_thunder_due(g: &mut crate::game::Game, i: f64) -> Option<Vec<f64>> {
    knobs_here(g);
    let mut said = [0.0; 8];
    if !::sim::weather::thunder_due(wx(g), i as usize, g.time, &mut said) { return None; }
    Some(said.to_vec())
}

/// Once a frame from Game.update: the clock, the strokes, the flash lamp, the thunder due. The frame's numbers are the picture's (notes). (weather.ts Weather.update)
pub fn weather_update(g: &mut crate::game::Game, dt: f64, cam_yaw: Option<f64>) {
    g.weather.cam_yaw = cam_yaw.unwrap_or(0.0);
    weather_sync_roofs(g, None);
    // THE FRAME'S NUMBERS (weather.ts:350, :351, :356, :373-381). The part left them out as the picture's, but skeleton 5.2's bridge carries `g.engine.weather`
    // as a thing the tick reads (wet.ts rainNow reads `.rain` to say who is rained on), so the tick writes it as the TypeScript does.
    if g.engine.weather.is_none() { g.engine.weather = Some(crate::render_frame::no_weather()); }
    if !g.weather.on { zero(g.engine.weather.as_mut().unwrap()); weather_set_rain_now(g, 0.0); return; }
    // day: the storm holds still and shows nothing; the night gets it back as it was
    let day = crate::lighting::is_daylight(g) && g.weather.kind_now.is_none();
    if day != g.weather.suspended {
        g.weather.suspended = day;
        if day {
            if g.weather.flash.is_some() { weather_end_flash(g); }
            weather_set_thunders_are(g, &[]);
            weather_set_wet_2(g, false);
            crate::audio_iface::game_audio_like_rain(g, 0.0);
        }
        weather_set_next_flash_at(g, -1.0);
    }
    if g.weather.suspended { zero(g.engine.weather.as_mut().unwrap()); weather_set_rain_now(g, 0.0); return; }
    let raining = weather_rain_rate(g) > 0.001;
    let soaked = ::sim::weather::lot_step(wx(g), dt);
    weather_set_wet_2(g, soaked);
    // ---- strokes
    if ::sim::weather::stroke_due(wx(g), g.time) {
        weather_strike(g, None);
        let at = g.time + weather_gap(g, false);
        weather_set_next_flash_at(g, at);
    }
    // the stroke in progress: this frame shows the envelope over [t, t + dt], then the clock moves on
    if let Some(t) = g.weather.flash.as_ref().map(|f| f.t) {
        if t >= FLASH_SECS {
            weather_end_flash(g);
        } else {
            weather_apply_flash(g, t, dt);
            if let Some(f) = g.weather.flash.as_mut() { f.t += dt; }
        }
    }
    // ---- thunder
    let mut i = wx_read(g).thunders_n as i64 - 1;
    while i >= 0 {
        if let Some(said) = weather_thunder_due(g, i as f64) {
            weather_play_thunder(g, Thunder { at: said[7], dist_km: said[0], bearing: said[1], loud: said[2] }, &said);
        }
        i -= 1;
    }
    // ---- the rain bed follows the rate and where the listener stands
    let seat = crate::game::game_host_seat(g);
    let man = crate::cmd::seat_man(g, seat);
    let pp = g.ch[g.pl[man].ch].pos;
    let bed = weather_rain_bed(g, pp[0], pp[2]);
    crate::audio_iface::game_audio_like_rain(g, bed);
    // ---- what the frame draws (weather.ts:373-381)
    let fl = match g.weather.flash.as_ref().map(|f| f.light) { Some(l) => g.tab.light[l].intensity * SKY_FLASH_K, None => 0.0 };
    let (rain, snow, wind, wet, kind, own) = (if raining { weather_rain_rate(g) } else { 0.0 }, g.weather.snow_rate, g.weather.wind, weather_wet(g), g.weather.kind_now.is_some(), g.weather.own);
    weather_set_rain_now(g, rain);
    let roofs = g.weather.roof_rects.clone();
    let fog = if kind { crate::weather_kinds::fog_glow_word(g.weather.fog_word, g.weather.fog_glow) } else { 0.0 };
    let fc = flash_color();
    let frame = g.engine.weather.as_mut().unwrap();
    frame.rain = rain;
    frame.snow = Some(snow);
    frame.shear_x = -wind[0] / RAIN_V; frame.shear_z = -wind[1] / RAIN_V;
    frame.wet = wet;
    frame.flash = [fc[0] * fl, fc[1] * fl, fc[2] * fl];
    frame.rain_top = RAIN_TOP;
    if !kind && !own { frame.interior = interior(); frame.annex = annex(); frame.roof_c = Some(0.0); frame.roof_d = Some(0.0); frame.fog = Some(0.0); }
    else {
        frame.interior = roofs.get(0).copied().unwrap_or(no_rect()); frame.annex = roofs.get(1).copied().unwrap_or(no_rect());
        frame.roof_c = Some(js::or(crate::weather_kinds::pack_rect(roofs.get(2).copied()), 1.0)); frame.roof_d = Some(crate::weather_kinds::pack_rect(roofs.get(3).copied())); frame.fog = Some(fog);
    }
}

/// Seconds to the next stroke; the first one of a night comes sooner. (weather.ts Weather.gap)
pub fn weather_gap(g: &mut crate::game::Game, first: bool) -> f64 { ::sim::weather::gap(wx(g), first) }

/// The rain falling on the game this tick, the module's table: what the frame draws, and what soaks a man. (weather.ts Weather.rainNow)
pub fn weather_set_rain_now(g: &mut crate::game::Game, v: f64) { wx(g).rain_now = v; }

/// The stroke's light over [t, t + dt]. (weather.ts Weather.applyFlash)
pub fn weather_apply_flash(g: &mut crate::game::Game, t: f64, dt: f64) {
    let f = g.weather.flash.as_ref().unwrap();
    let intensity = f.peak * stroke_mean(&f.stroke, t, t + m::max(dt, 1e-4));
    let enabled = intensity > 1e-3 * f.peak;
    let light = f.light;
    let l = &mut g.tab.light[light];
    l.intensity = intensity;
    l.enabled = enabled;
}

/// The stroke's light leaves the set. (weather.ts Weather.endFlash)
pub fn weather_end_flash(g: &mut crate::game::Game) {
    let light = match g.weather.flash.as_ref() { Some(f) => f.light, None => return };
    crate::render_lights::light_set_remove(g, light);
    g.weather.flash = None;
}

/// A thunder that came due (`dice`: the module threw its four and its clap drowned the small noises): the sound of it, and somebody's remark. (weather.ts Weather.playThunder)
pub fn weather_play_thunder(g: &mut crate::game::Game, th: crate::weather::Thunder, dice: &[f64]) {
    // near: the crack and the clap on top of the roll; far: the roll alone, darker and slower
    let near = th.dist_km < 1.3;
    let name = if near { "thunderNear" } else { "thunderFar" };
    let gain = (if near { 1.0 } else { 0.85 }) * th.loud * (0.6 + 0.4 * weather_storm(g));
    let lowpass = v3::clamp(9000.0 * m::exp(-th.dist_km * 0.75), 500.0, 9000.0);
    let rate = v3::clamp(1.02 - th.dist_km * 0.06, 0.78, 1.02) * (0.96 + 0.08 * dice[3]);
    let cy = m::cos(g.weather.cam_yaw);
    let sy = m::sin(g.weather.cam_yaw);
    let bx = m::sin(th.bearing);
    let bz = -m::cos(th.bearing);
    let right = bx * cy - bz * sy;
    let opts = crate::audio_iface::PlayOpts::Thunder { lowpass, rate, pan: v3::clamp(right * 0.6, -0.6, 0.6), wet: if near { 0.35 } else { 0.55 } };
    crate::audio_iface::game_audio_like_play(g, name, None, Some(gain), Some(&opts));
    g.weather.stats.thunders += 1.0;
    g.weather.stats.last_thunder = Some(WeatherStatsLastThunder { at: th.at, dist_km: th.dist_km, bearing: th.bearing, loud: th.loud, played_at: g.time, name: String::from(name), gain });
    // the three dice for a remark are thrown on EVERY thunder, whoever is on screen: same seed, same storm
    let (r_bark, r_who, r_line) = (dice[4], dice[5], dice[6]);
    if th.loud > 0.45 && g.time - weather_last_bark_t(g) > BARK_COOLDOWN && r_bark < BARK_CHANCE {
        let mut cands: Vec<GuardId> = Vec::new();
        for gd in g.guards.clone() {   // (a copy: on_screen does not change the list)
            let man = &g.gd[gd];
            if !(man.state == crate::game::GuardState::Patrol && man.held.is_none() && man.script.is_none() && man.unit.is_none() && man.awareness < 0.2) { continue; }
            let c = man.ch;
            let head = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(h) => *h, None => v3::add(g.ch[c].pos, [0.0, 1.7, 0.0]) };
            if crate::guards::on_screen(g, head) { cands.push(gd); }
        }
        if !cands.is_empty() {
            let n = cands.len() as f64;
            let gd = cands[js::rem(m::floor(r_who * n), n) as usize];
            let lines = thunder_lines();
            let mut line: &str = &lines[js::rem(g.weather.stats.barks + m::floor(r_line * 2.0), lines.len() as f64) as usize];
            if line.contains("not out in that") {
                let p = g.ch[g.gd[gd].ch].pos;
                if !crate::scene_collision::solid_blocked(g, [p[0], 1.9, p[2]], [p[0], 10.9, p[2]], None) { line = crate::words::wg(g, &THUNDER_OUT_GUIDE, &THUNDER_OUT); }   // he is out in it
            }
            crate::game::game_say(g, gd, line, None, None, None);
            g.weather.stats.barks += 1.0;
            let at = g.time;
            weather_set_last_bark_t(g, at);
        }
    }
}

/// The wet lot: the asphalt slab darkened and glossy, or everything put back. Idempotent both ways. (weather.ts Weather.setWet)
pub fn weather_set_wet_2(g: &mut crate::game::Game, on: bool) {
    if on == g.weather.wet_on { return; }
    g.weather.wet_on = on;
    if on {
        // the lot's ground: the one big slab whose top is the y = 0 the cars stand on (40 x 28 m: the interior carpet base is 32 x 20 and must never match)
        let mut asphalt = None;
        for i in 0..g.engine.world.statics.len() {
            let b = g.engine.world.statics[i];
            let (c, h) = (seam_box(g, b).c, seam_box(g, b).h);
            if c[1] < 0.0 && c[1] + h[1] > -0.01 && h[0] > 18.0 && h[2] > 12.0 { asphalt = Some(b); break; }
        }
        if let Some(b) = asphalt { soak(g, b); }
        g.weather.lot = asphalt;
        if g.weather.wet_gloss && g.weather.gloss_was.is_none() {
            g.weather.gloss_was = Some(g.engine.settings.gloss);
            g.engine.settings.gloss = true;
        }
    } else {
        let touched = core::mem::take(&mut g.weather.touched);
        for t in &touched { paint(g, t.box_, t.albedo, t.flags, t.rough, t.aniso); }
        g.weather.lot = None;
        if let Some(was) = g.weather.gloss_was {
            if g.engine.settings.gloss { g.engine.settings.gloss = was; }   // (only if it is still what we set: a gloss switched off by hand stays off)
            g.weather.gloss_was = None;
        }
    }
    // the rest of the wet ground is the picture's and follows in this very call
    crate::wet_ground::wet_ground_follow(g);
}

/// Darkens a box and remembers what to put back. (weather.ts Weather.setWet soak)
fn soak(g: &mut Game, b: BoxId) {
    let (albedo, flags, rough, aniso) = { let x = seam_box(g, b); (x.albedo, x.flags, x.rough, x.aniso) };
    g.weather.touched.push(WeatherTouchedItem { box_: b, albedo: v3::copy(albedo), flags, rough, aniso });
    let gloss = crate::scene_boxes::box_flag().gloss;
    paint(g, b, [albedo[0] * WET_ALBEDO, albedo[1] * WET_ALBEDO, albedo[2] * WET_ALBEDO * 1.04], (js::to_i32(flags) | js::to_i32(gloss)) as f64, Some(WET_ROUGH), Some(true));
}

/// Everything off and put back. (weather.ts Weather.teardown)
pub fn weather_teardown(g: &mut crate::game::Game) {
    weather_end_flash(g);
    weather_set_thunders_are(g, &[]);
    weather_set_next_flash_at(g, -1.0);
    weather_set_wet_2(g, false);
    weather_set_wet(g, 0.0);
    weather_set_rain_now(g, 0.0);
    if let Some(frame) = g.engine.weather.as_mut() { zero(frame); }
    crate::audio_iface::game_audio_like_rain(g, 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bits from the TypeScript's own strokeMean, run under bun with EXACT.on (first pass): the same bits must come off the module's. The dice are the module's now, and tested there.
    fn stroke() -> Stroke {
        Stroke { pulses: vec![StrokePulsesItem { t0: 0.0, amp: 1.0, tau: 0.022 }, StrokePulsesItem { t0: 0.085, amp: 0.61, tau: 0.05 }, StrokePulsesItem { t0: 0.23, amp: 0.3, tau: 0.03 }] }
    }

    #[test]
    fn the_envelope_is_the_typescripts_bits() {
        let s = stroke();
        for (i, (a, b)) in SPANS.iter().enumerate() {
            assert_eq!(stroke_mean(&s, *a, *b).to_bits(), WANT_MEAN[i], "mean {}", i);
        }
    }

    const SPANS: [(f64, f64); 5] = [(0.0, 0.016666666666666666), (0.08, 0.1), (0.2, 0.2), (0.25, 0.3), (0.5, 0.25)];
    const WANT_MEAN: [u64; 5] = [0x3fe670152ad1625c, 0x3fda675a22e0a5c0, 0x3faf5edb84c60892, 0x3fb6d4ef3206a694, 0x3f28b8fb354e6075];
}
