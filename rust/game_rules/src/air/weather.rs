pub use ::game_types::weather::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};


/// `g.level.boxes` as IDs: that list and `g.col.boxes` are one array in the TypeScript, statics appended later included. `level.boxes` is not a field in Rust.
fn level_boxes(g: &Game) -> Vec<BoxId> { g.col.boxes.clone() }

/// Paints a box's row.
fn paint(g: &mut Game, b: BoxId, albedo: V3, flags: f64, rough: Option<f64>, aniso: Option<bool>) {
    let mut x = g.tab.boxes.edit(b);
    x.albedo = albedo; x.flags = flags; x.rough = rough; x.aniso = aniso;
}

/// `WEATHER_PRESETS[kind]`.
fn preset(g: &Game, kind: crate::weather_kinds::WeatherKind) -> &crate::weather_kinds::WeatherPreset { g.m.weather_kinds.weather_presets.get(kind).unwrap() }

// ---- the file's free functions

/// A LEVEL'S ROOFS: the ceiling slabs among these boxes (a centre and a half size each) as footprints, biggest first (equal ones in the order they came), at most four.
pub fn roofs_among(boxes: impl Iterator<Item = (V3, V3)>) -> Vec<[f64; 4]> {
    let ceiling = crate::scene_world::world().ceiling_y;
    let mut out: Vec<(f64, [f64; 4])> = Vec::new();
    for (c, h) in boxes {
        let (y0, y1) = (c[1] - h[1], c[1] + h[1]);
        if !(y0 >= ceiling - 0.25 && y1 <= ceiling + 0.45) || h[0] < 0.45 || h[2] < 0.45 { continue; }
        let r = [c[0] - h[0] - ROOF_PAD, c[2] - h[2] - ROOF_PAD, c[0] + h[0] + ROOF_PAD, c[2] + h[2] + ROOF_PAD];
        let area = (r[2] - r[0]) * (r[3] - r[1]);
        let at = out.iter().rposition(|q| !(q.0 < area)).map_or(0, |k| k + 1);
        out.insert(at, (area, r));
        out.truncate(::sim::weather::ROOFS);
    }
    out.into_iter().map(|q| q.1).collect()
}
/// The level's own.
fn roofs_of(g: &crate::game::Game, boxes: &[BoxId]) -> Vec<[f64; 4]> { roofs_among(boxes.iter().map(|&b| { let x = &g.tab.boxes[b]; (x.c, x.h) })) }

/// A level that carries its own places: the office's footprints are not its building's.
fn own_floor(g: &crate::game::Game) -> bool {
    g.level.own_places()
}

/// The mean of the stroke's envelope over [a, b].
fn stroke_mean(s: &crate::weather::Stroke, a: f64, b: f64) -> f64 {
    let mut e = 0.0;
    if b <= a {
        for p in &s.pulses { if a >= p.t0 { e += p.amp * m::exp(-(a - p.t0) / p.tau); } }
        return e;
    }
    for p in &s.pulses {
        let lo = m::max(a, p.t0);
        if b <= lo { continue; }
        e += p.amp * p.tau * (m::exp(-(lo - p.t0) / p.tau) - m::exp(-(b - p.t0) / p.tau));
    }
    e / (b - a)
}

// ---- the storm's numbers: the speed of sound, a stroke's own peak, the mean seconds between strokes, the lot's soak; a roof's overhang
const SPEED_OF_SOUND: f64 = 343.0;
const FLASH_PEAK: f64 = 18.0;
const FLASH_MEAN_GAP: f64 = 11.0;
const WET_SECS: f64 = 20.0;
const ROOF_PAD: f64 = 0.06;
/// thunder on its way at once. A stroke comes at least four seconds after the last and its sound is at most thirteen on its way, so play has four at most; a hand on
/// the panel's button may ask more, and the oldest then goes unheard
const THUNDERS: usize = 16;

/// Seconds to the next stroke; the first one of a night comes sooner.
fn gap(w: &mut Weather, first: bool) -> f64 {
    if first { return 6.0 + 6.0 * w.die.roll(); }
    let mean = v3::clamp(FLASH_MEAN_GAP / m::max(w.storm, 0.05), 8.0, 25.0);
    mean * (0.5 + w.die.roll())
}
fn thunder_push(w: &mut Weather, t: Thunder) {
    if w.thunders.len() >= THUNDERS { w.thunders.remove(0); }
    w.thunders.push(t);
}
/// is (x, z) under a footprint in force
fn under(w: &Weather, x: f64, z: f64) -> bool { ::sim::weather::under_any(&w.roof_rects, x, z) }
/// the rain's level at a listener, what the mixer plays there and what drowns small noises
fn rain_bed(g: &Game, x: f64, z: f64) -> f64 {
    let w = &g.weather;
    if w.rain_rate > 0.001 { (0.35 + 0.65 * w.rain_rate) * (if under(w, x, z) { g.k.wx_sim.roof_bed } else { 1.0 }) } else { 0.0 }
}
/// The fog's optical depth along a → b, before any roof.
pub(super) fn fog_tau(sigma: f64, h: f64, a: V3, b: V3) -> f64 {
    let l = m::hyp3(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if !(l > 0.0) { return 0.0; }
    if !(h > 0.0) { return sigma * l; }
    let ya = m::max(a[1], 0.0) / h;
    let ks = (m::max(b[1], 0.0) - m::max(a[1], 0.0)) / h;
    sigma * m::exp(-ya) * l * (if m::abs(ks) < 1e-3 { 1.0 - 0.5 * ks } else { (1.0 - m::exp(-ks)) / ks })
}

// ---- class Weather

/// The footprints in force: their one writer.
fn weather_set_roofs(g: &mut crate::game::Game, r: &[[f64; 4]]) {
    if r.len() > ::sim::weather::ROOFS { panic!("weather: {} roofs, and the thrown things' walk holds {}", r.len(), ::sim::weather::ROOFS); }
    g.weather.roof_rects = r.to_vec();
}

/// The fog's word, and the numbers it holds.
fn weather_set_fog(g: &mut crate::game::Game, word: f64) {
    g.weather.fog_word = word;
    g.weather.fog_spec = crate::weather_kinds::unpack_fog(word);
}

/// Cloud cover 0..1 for whoever owns the sun and the moon to read.
pub(super) fn weather_cloud(g: &crate::game::Game) -> f64 {
    if !g.weather.on { return 0.0; }
    match g.weather.kind_now { Some(k) => preset(g, k).cloud, None => legacy().cloud }
}

/// The weather a new game is born in. Runs on `g.weather` (every method's path) and returns what the caller stores there.
pub fn weather_new(g: &mut crate::game::Game) -> crate::weather::Weather {
    g.weather = Weather::blank();
    let l = legacy();
    g.weather.storm = l.storm;
    g.weather.rain_rate = l.rain;
    g.weather.die = crate::dice::Die(20260809);
    weather_set_roofs(g, &[interior(), annex()]);
    let stood = g.stood;
    let born = match weather_level_kind(g) {
        Some(k) => Some(k),
        None => if stood.weather == Some(crate::weather_kinds::WeatherKind::Snow) { None } else { stood.weather },
    };
    if born.is_none() && own_floor(g) { weather_own_night(g); }
    let wet = if g.weather.rain_rate > 0.0 { 1.0 } else { 0.0 };
    g.weather.wet = wet;
    if born.is_some() { set_kind_at(g, born, false); } else { sync_roofs(g, false, false); }
    g.weather.clone()
}

/// A level built for snow is born in the kind 'snow' and stays in it; None = any other level.
fn weather_level_kind(g: &crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> {
    let name = crate::scene_level_load::level_name_of(g);
    let stood = g.stood;
    if crate::weather_kinds::snow_level(g, name.as_deref()) && (!stood.locked || stood.weather == Some(crate::weather_kinds::WeatherKind::Snow)) { Some(crate::weather_kinds::WeatherKind::Snow) } else { None }
}

/// No weather picked on a floor that is none of the office's: its own night is dry and still, under its own roofs.
fn weather_own_night(g: &mut crate::game::Game) {
    g.weather.storm = 0.0;
    g.weather.rain_rate = 0.0;
    g.weather.wind = [0.0, 0.0];
    weather_set_fog(g, 0.0);
    let roofs = weather_own_roofs(g);
    weather_set_roofs(g, &roofs);
}

/// The level's own storm is in force, cut by the level's own roofs (WX_OWN.roofs).
pub(super) fn weather_own_storm(g: &crate::game::Game) -> bool { g.weather.own }

/// Do the office's two rectangles fit this level's roofs (asked once).
fn weather_office_fit(g: &mut crate::game::Game) -> bool {
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

/// Two footprints within 0.3 m in every number.
fn near(a: &[f64; 4], b: &[f64; 4]) -> bool {
    a.iter().enumerate().all(|(i, v)| m::abs(*v - b[i]) < 0.3)
}

/// The footprints follow the rule in force.
fn weather_sync_roofs(g: &mut crate::game::Game, tell_wind: Option<bool>) { sync_roofs(g, tell_wind.unwrap_or(false), true) }

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

/// `roofs` are the level's own and in force.
pub fn weather_level_roofs(g: &crate::game::Game) -> bool { g.weather.kind_now.is_some() || g.weather.own }

/// The level's own ceiling slabs as the no-rain footprints (roofs 0 / 1 exact, 2 / 3 as the Frame packs them).
fn weather_own_roofs(g: &mut crate::game::Game) -> Vec<[f64; 4]> {
    let boxes = level_boxes(g);
    roofs_of(g, &boxes).into_iter().enumerate().map(|(i, r)| if i < 2 { r } else { crate::weather_kinds::unpack_rect(crate::weather_kinds::pack_rect(Some(r))) }).collect()
}

/// Put the level in a weather (None = back as it has always been). False = refused.
pub fn weather_set_kind_2(g: &mut crate::game::Game, kind: Option<crate::weather_kinds::WeatherKind>) -> bool { set_kind_at(g, kind, true) }

/// setKind, with `built` false while the constructor runs: `g.wind` and `g.airDust` are built after this object.
fn set_kind_at(g: &mut Game, kind: Option<crate::weather_kinds::WeatherKind>, built: bool) -> bool {
    let own = weather_level_kind(g);
    if own.is_some() && kind != own { return false; }   // a level built for snow stays in it
    if g.stood.locked && kind != g.stood.weather { return false; }
    weather_set_wet_2(g, false);   // (what is wet depends on the kind)
    g.weather.kind_now = kind;
    g.weather.own = false;
    g.weather.snow_rate = 0.0;
    if kind.is_some() && !g.weather.on { g.weather.on = true; }   // (a picked weather IS on)
    weather_end_flash(g);
    g.weather.thunders.clear();
    g.weather.next_flash_at = -1.0;
    g.weather.thunder_until = -1e9;
    if let Some(k) = kind {
        let (storm, rain, wind, snow, fog) = { let p = preset(g, k); (p.storm, p.rain, p.wind, p.snow, p.fog) };
        g.weather.storm = storm;
        g.weather.rain_rate = rain;
        g.weather.wind = [wind[0], wind[1]];
        g.weather.snow_rate = snow.unwrap_or(0.0);
        weather_set_fog(g, crate::weather_kinds::pack_fog(fog));
        let roofs = weather_own_roofs(g);
        weather_set_roofs(g, &roofs);
    } else if own_floor(g) {
        weather_own_night(g);
    } else {
        let l = legacy();
        g.weather.storm = l.storm;
        g.weather.rain_rate = l.rain;
        g.weather.wind = [l.wind[0], l.wind[1]];
        weather_set_fog(g, 0.0);
        weather_set_roofs(g, &[interior(), annex()]);
    }
    let wet = if g.weather.on && g.weather.rain_rate > 0.0 { 1.0 } else { 0.0 };
    g.weather.wet = wet;
    sync_roofs(g, true, built);
    if built { crate::air_dust::air_dust_reseed(g); }
    crate::hours::hour_weather_changed(g);
    true
}

/// The footprints in force now, as underRoof reads them.
pub fn weather_roofs_now(g: &mut crate::game::Game) -> Vec<[f64; 4]> {
    weather_sync_roofs(g, None);
    g.weather.roof_rects.clone()
}

/// Is (x, z) under one of the footprints the rain is cut by.
pub fn weather_under_roof(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    weather_sync_roofs(g, None);
    under(&g.weather, x, z)
}

/// HEARING: the factor (>= 1) a sound's path is stretched by for a listener at `ear`.
pub fn weather_hear_mask(g: &mut crate::game::Game, ear: V3) -> f64 {
    let w = &g.weather;
    if w.kind_now.is_none() { return 1.0; }
    let mut k = 1.0 + g.k.wx_sim.rain_k * rain_bed(g, ear[0], ear[2]);
    if g.time < w.thunder_until { k += g.k.wx_sim.thunder_k * (if under(w, ear[0], ear[2]) { 0.6 } else { 1.0 }); }
    k
}

/// SIGHT: the fog's transmittance along a to b, the share of it standing there taken along the line (the dust where it actually is while the field moves, else the
/// roof's leak at four points); exactly 1 with no kind picked or no fog in it.
pub fn weather_air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let (Some(f), Some(_)) = (g.weather.fog_spec, g.weather.kind_now) else { return 1.0 };
    crate::air_dust::air_dust_ready(g);   // (the field stands before it is read: it is built when first asked)
    let air = &::sim::weather::of_read(g.sim).air;
    let mut leak = 0.0;
    if air.on != 0.0 {
        let n = m::min(24.0, m::max(4.0, m::ceil(m::hyp2(b[0] - a[0], b[2] - a[2]) / 0.75)));
        let mut i = 0.0;
        while i < n {
            let t = (i + 0.5) / n;
            leak += air.share_at(a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t);
            i += 1.0;
        }
        return m::exp(-fog_tau(f.sigma, f.h, a, b) * leak / n);
    }
    for i in 0..4 {
        let t = (i as f64 + 0.5) / 4.0;
        leak += if under(&g.weather, a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t) { f.leak } else { 1.0 };
    }
    m::exp(-fog_tau(f.sigma, f.h, a, b) * leak / 4.0)
}

/// Everything the storm's next minutes hang on; a stroke in progress is not carried. hours.ts holds a level's own storm in one, so it is a rule's call and not only a checkpoint's.
pub(super) fn weather_snapshot(g: &crate::game::Game) -> crate::weather::WeatherSnap {
    let kind = g.weather.kind_now;
    WeatherSnap {
        kind,
        thunder_until: if kind.is_some() { Some(g.weather.thunder_until) } else { None },
        storm: g.weather.storm,
        rain_rate: g.weather.rain_rate,
        wind: [g.weather.wind[0], g.weather.wind[1]],
        wet_gloss: g.weather.wet_gloss,
        wet: g.weather.wet,
        next_flash_at: g.weather.next_flash_at,
        thunders: g.weather.thunders.clone(),
        last_bark_t: g.weather.last_bark_t,
        rng: g.weather.die.0 as f64,
        stats: WeatherSnapStats { flashes: g.weather.stats.flashes, thunders: g.weather.stats.thunders, barks: g.weather.stats.barks },
    }
}

/// `Number.isFinite(x) ? x : d`.
fn finite_or(x: f64, d: f64) -> f64 { if x.is_finite() { x } else { d } }

/// The storm as a snapshot had it; on / off stays the session's.
pub(super) fn weather_restore(g: &mut crate::game::Game, s: &crate::weather::WeatherSnap) {
    weather_end_flash(g);
    let k = if crate::weather_kinds::is_any_kind(g, s.kind) { s.kind } else { None };
    if k != g.weather.kind_now { weather_set_kind_2(g, k); }   // the kind is the file's: the run it holds was played under it
    let own = weather_level_kind(g);
    if own.is_some() && k != own { return; }   // made in other weather on a level that keeps its own: none of its rain, wind or wet comes in
    g.weather.thunder_until = finite_or(s.thunder_until.unwrap_or(-1e9), -1e9);
    let storm = finite_or(s.storm, g.weather.storm);
    g.weather.storm = storm;
    let rain = finite_or(s.rain_rate, g.weather.rain_rate);
    g.weather.rain_rate = rain;
    g.weather.wind = [finite_or(s.wind[0], g.weather.wind[0]), finite_or(s.wind[1], g.weather.wind[1])];
    g.weather.wet_gloss = s.wet_gloss;
    let wet = finite_or(s.wet, g.weather.wet);
    g.weather.wet = wet;
    g.weather.next_flash_at = finite_or(s.next_flash_at, -1.0);
    let thunders: Vec<Thunder> = s.thunders.iter().map(|t| Thunder { at: finite_or(t.at, 0.0), dist_km: finite_or(t.dist_km, 1.0), bearing: finite_or(t.bearing, 0.0), loud: finite_or(t.loud, 0.5) }).collect();
    g.weather.thunders.clear();
    for t in thunders { thunder_push(&mut g.weather, t); }
    g.weather.last_bark_t = finite_or(s.last_bark_t, -1e9);
    if s.rng.is_finite() { g.weather.die = crate::dice::Die(js::to_u32(s.rng)); }
    let w = &mut g.weather;
    w.stats.flashes = js::to_i32(s.stats.flashes) as f64;
    w.stats.thunders = js::to_i32(s.stats.thunders) as f64;
    w.stats.barks = js::to_i32(s.stats.barks) as f64;
    weather_sync_roofs(g, None);   // a replay's knobs are put in after this object was made: the footprints follow the rule now in force
}

/// Switch the storm on or off; a picked weather stays on.
pub fn weather_set_enabled_2(g: &mut crate::game::Game, on: bool) {
    if on == g.weather.on { return; }
    if !on && g.weather.kind_now.is_some() { return; }
    g.weather.on = on;
    if !on { weather_teardown(g); } else { let wet = if g.weather.rain_rate > 0.0 { 1.0 } else { 0.0 }; g.weather.wet = wet; }
}

/// Fire a stroke now. The Flash returned is a copy; the live one is `g.weather.flash`.
pub fn weather_strike(g: &mut crate::game::Game, opts: Option<crate::weather::WeatherStrikeOpts>) -> Option<crate::weather::Flash> {
    let opts = opts.unwrap_or_default();
    if !g.weather.on { return None; }
    if g.weather.flash.is_some() { weather_end_flash(g); }
    // (what was given is a number, what was not is thrown for, in the order the rule has always thrown them)
    let (time, w) = (g.time, &mut g.weather);
    let bearing = opts.bearing.unwrap_or_else(|| stroke_bearing(w.die.roll()));
    let elev = opts.elev_deg.map_or_else(|| stroke_elev(w.die.roll()), |d| d * m::PI / 180.0);
    let dist = match opts.dist_km { Some(d) => d, None => 0.3 + m::pow(w.die.roll(), 1.3) * 3.7 * (1.15 - 0.5 * w.storm) };
    let peak = opts.peak.unwrap_or_else(|| FLASH_PEAK * (0.55 + 0.9 * w.die.roll()) * v3::clamp(1.25 - dist * 0.18, 0.45, 1.2));
    // two return strokes, sometimes a third flicker
    let amp2 = 0.35 + 0.45 * w.die.roll();
    let mut stroke = Stroke { pulses: vec![StrokePulsesItem { t0: 0.0, amp: 1.0, tau: 0.022 }, StrokePulsesItem { t0: 0.085, amp: amp2, tau: 0.05 }] };
    if w.die.roll() < 0.3 {
        let t0 = 0.2 + 0.05 * w.die.roll();
        stroke.pulses.push(StrokePulsesItem { t0, amp: 0.25 + 0.25 * w.die.roll(), tau: 0.03 });
    }
    thunder_push(w, Thunder { at: time + dist * 1000.0 / SPEED_OF_SOUND, dist_km: dist, bearing, loud: v3::clamp(1.15 - dist * 0.22, 0.25, 1.0) });
    let dir = dir_of(bearing, elev);
    let light = g.tab.light.push(crate::render_lights::RtLight {
        name: String::from("lightning"), group: String::from("weather"), kind: 2.0, pos: [0.0, 0.0, 0.0], dir, color: v3::copy(flash_color()), intensity: 0.0, range: 1e6, inner_deg: 0.0, outer_deg: 0.0,
        radius: STROKE_RADIUS, volumetric: 1.3, owner: 0.0, enabled: true, fixture_box: -1.0, flicker: 0.0, ttl: -1.0, age: 0.0, peak_intensity: peak, transient: Some(true),
        ..crate::render_lights::RtLight::default()   // (every field the literal leaves absent)
    });
    crate::render_lights::light_set_add(g, light);
    g.weather.flash = Some(Flash { light, stroke, t: 0.0, peak });
    g.weather.stats.flashes += 1.0;
    weather_apply_flash(g, 0.0, 1.0 / 60.0);   // lit from this very frame
    g.weather.flash.clone()
}

/// where a stroke stands, of its first two throws
fn stroke_bearing(throw: f64) -> f64 { throw * m::PI * 2.0 }
fn stroke_elev(throw: f64) -> f64 { (24.0 + throw * 30.0) * m::PI / 180.0 }
/// the throws a thunder takes as it comes due: its pitch and what somebody says of it
const THUNDER_THROWS: u32 = 4;
/// how wide a stroke is as a lamp
pub const STROKE_RADIUS: f64 = 0.006;
/// THE NEXT STROKE'S WAY, before it comes: what the picture works out ahead of a light that stands for an instant. Read off the storm's own die, which the thunders due
/// before the stroke throw first. None with no stroke on its way. One that is asked for by hand is not foreseen, and comes as it comes.
pub fn weather_next_way(g: &Game) -> Option<V3> {
    let w = &g.weather;
    if !w.on || !(w.storm > 0.01) || w.next_flash_at < 0.0 { return None; }
    let ticks = |at: f64| m::max(m::ceil((at - g.time) * 60.0), 1.0);
    let stroke = ticks(w.next_flash_at);
    let before = w.thunders.iter().filter(|t| ticks(t.at) < stroke).count() as u32 * THUNDER_THROWS;
    Some(dir_of(stroke_bearing(w.die.ahead(before)), stroke_elev(w.die.ahead(before + 1))))
}

/// Once a frame from Game.update: the clock, the strokes, the flash lamp, the thunder due. The frame's numbers are the picture's.
pub fn weather_update(g: &mut crate::game::Game, dt: f64) {
    weather_sync_roofs(g, None);
    // THE FRAME'S NUMBERS.
    if g.engine.weather.is_none() { g.engine.weather = Some(crate::render_frame::no_weather()); }
    if !g.weather.on { zero(g.engine.weather.as_mut().unwrap()); g.weather.rain_now = 0.0; return; }
    let raining = g.weather.rain_rate > 0.001;
    let w = &mut g.weather;
    w.wet = v3::clamp(w.wet + (if raining { dt } else { -dt }) / WET_SECS, 0.0, 1.0);
    let soaked = w.wet > 0.02;
    weather_set_wet_2(g, soaked);
    // ---- strokes
    let (time, w) = (g.time, &mut g.weather);
    if w.next_flash_at < 0.0 { w.next_flash_at = time + gap(w, true); }
    if w.storm > 0.01 && time >= w.next_flash_at {
        weather_strike(g, None);
        g.weather.next_flash_at = g.time + gap(&mut g.weather, false);
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
    for i in (0..g.weather.thunders.len()).rev() {
        let w = &mut g.weather;
        if !(g.time >= w.thunders[i].at) { continue; }
        let th = w.thunders.remove(i);
        // (its clap may drown the small noises, and four dice are thrown whoever is listening)
        let pitch = w.die.roll();
        if w.kind_now.is_some() && th.loud > g.k.wx_sim.thunder_loud { w.thunder_until = g.time + g.k.wx_sim.thunder_secs; }
        let remark = [w.die.roll(), w.die.roll(), w.die.roll()];
        weather_play_thunder(g, th, pitch, remark);
    }
    // ---- the rain bed follows the rate and where the listener stands
    let seat = crate::game::game_host_seat(g);
    let man = crate::cmd::seat_man(g, seat);
    let pp = g.ch[g.pl[man].ch].pos;
    weather_sync_roofs(g, None);
    let bed = rain_bed(g, pp[0], pp[2]);
    crate::audio_iface::game_audio_like_rain(g, bed);
    // ---- what the frame draws
    let fl = match g.weather.flash.as_ref().map(|f| f.light) { Some(l) => g.tab.light[l].intensity * SKY_FLASH_K, None => 0.0 };
    let (rain, snow, wind, wet, kind, own) = (if raining { g.weather.rain_rate } else { 0.0 }, g.weather.snow_rate, g.weather.wind, g.weather.wet, g.weather.kind_now.is_some(), g.weather.own);
    g.weather.rain_now = rain;
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

/// The stroke's light over [t, t + dt].
fn weather_apply_flash(g: &mut crate::game::Game, t: f64, dt: f64) {
    let f = g.weather.flash.as_ref().unwrap();
    let intensity = f.peak * stroke_mean(&f.stroke, t, t + m::max(dt, 1e-4));
    let enabled = intensity > 1e-3 * f.peak;
    let light = f.light;
    let l = &mut g.tab.light[light];
    l.intensity = intensity;
    l.enabled = enabled;
}

/// The stroke's light leaves the set.
fn weather_end_flash(g: &mut crate::game::Game) {
    let light = match g.weather.flash.as_ref() { Some(f) => f.light, None => return };
    crate::render_lights::light_set_remove(g, light);
    g.weather.flash = None;
}

/// A thunder that came due: the sound of it, and somebody's remark.
fn weather_play_thunder(g: &mut crate::game::Game, th: crate::weather::Thunder, pitch: f64, remark: [f64; 3]) {
    // near: the crack and the clap on top of the roll; far: the roll alone, darker and slower
    let near = th.dist_km < 1.3;
    let name = if near { "thunderNear" } else { "thunderFar" };
    let gain = (if near { 1.0 } else { 0.85 }) * th.loud * (0.6 + 0.4 * g.weather.storm);
    let lowpass = v3::clamp(9000.0 * m::exp(-th.dist_km * 0.75), 500.0, 9000.0);
    let rate = v3::clamp(1.02 - th.dist_km * 0.06, 0.78, 1.02) * (0.96 + 0.08 * pitch);
    // (which ear it is in: the shown seat's, and no number of the game's)
    let yaw = g.tab.camera[g.tab.cmd_seat[crate::game::game_host_seat(g)].cam].yaw;
    let (cy, sy) = (m::cos(yaw), m::sin(yaw));
    let bx = m::sin(th.bearing);
    let bz = -m::cos(th.bearing);
    let right = bx * cy - bz * sy;
    let opts = crate::audio_iface::PlayOpts::Thunder { lowpass, rate, pan: v3::clamp(right * 0.6, -0.6, 0.6), wet: if near { 0.35 } else { 0.55 } };
    crate::audio_iface::game_audio_like_play(g, name, None, Some(gain), Some(&opts));
    g.weather.stats.thunders += 1.0;
    // the three dice for a remark are thrown on EVERY thunder, whoever is on screen: same seed, same storm
    let [r_bark, r_who, r_line] = remark;
    if th.loud > 0.45 && g.time - g.weather.last_bark_t > BARK_COOLDOWN && r_bark < BARK_CHANCE {
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
                if !crate::scene_collision::solid_blocked(g, [p[0], 1.9, p[2]], [p[0], 10.9, p[2]], None) { line = THUNDER_OUT; }   // he is out in it
            }
            crate::game::game_say(g, gd, line, None, None);
            g.weather.stats.barks += 1.0;
            let at = g.time;
            g.weather.last_bark_t = at;
        }
    }
}

/// The wet lot: the asphalt slab darkened and glossy, or everything put back. Idempotent both ways.
fn weather_set_wet_2(g: &mut crate::game::Game, on: bool) {
    if on == g.weather.wet_on { return; }
    g.weather.wet_on = on;
    if on {
        // the lot's ground: the one big slab whose top is the y = 0 the cars stand on (40 x 28 m: the interior carpet base is 32 x 20 and must never match)
        let mut asphalt = None;
        for i in 0..g.engine.world.statics.len() {
            let b = g.engine.world.statics[i];
            let (c, h) = (g.tab.boxes[b].c, g.tab.boxes[b].h);
            if c[1] < 0.0 && c[1] + h[1] > -0.01 && h[0] > 18.0 && h[2] > 12.0 { asphalt = Some(b); break; }
        }
        if let Some(b) = asphalt { soak(g, b); }
        g.weather.lot = asphalt;
    } else {
        let touched = core::mem::take(&mut g.weather.touched);
        for t in &touched { paint(g, t.box_, t.albedo, t.flags, t.rough, t.aniso); }
        g.weather.lot = None;
    }
}

/// Darkens a box and remembers what to put back.
fn soak(g: &mut Game, b: BoxId) {
    let (albedo, flags, rough, aniso) = { let x = &g.tab.boxes[b]; (x.albedo, x.flags, x.rough, x.aniso) };
    g.weather.touched.push(WeatherTouchedItem { box_: b, albedo: v3::copy(albedo), flags, rough, aniso });
    let gloss = crate::scene_boxes::box_flag().gloss;
    paint(g, b, [albedo[0] * WET_ALBEDO, albedo[1] * WET_ALBEDO, albedo[2] * WET_ALBEDO * 1.04], (js::to_i32(flags) | js::to_i32(gloss)) as f64, Some(WET_ROUGH), Some(true));
}

/// Everything off and put back.
fn weather_teardown(g: &mut crate::game::Game) {
    weather_end_flash(g);
    g.weather.thunders.clear();
    g.weather.next_flash_at = -1.0;
    weather_set_wet_2(g, false);
    g.weather.wet = 0.0;
    g.weather.rain_now = 0.0;
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

/// The direction a stroke's light travels, for a stroke seen at this bearing and elevation.
fn dir_of(bearing: f64, elev: f64) -> V3 {
    let to: V3 = [m::sin(bearing) * m::cos(elev), m::sin(elev), -m::cos(bearing) * m::cos(elev)];
    [-to[0], -to[1], -to[2]]
}

/// The weather's frame with everything off.
fn zero(f: &mut crate::render_frame::WeatherFrame) {
    f.rain = 0.0;
    f.snow = Some(0.0);
    f.wet = 0.0;
    f.flash = [0.0, 0.0, 0.0];
    f.shear_x = 0.0;
    f.shear_z = 0.0;
    f.roof_c = Some(0.0);
    f.roof_d = Some(0.0);
    f.fog = Some(0.0);
}

/// Hearing through the weather: a sound's way with its length stretched by the noise at the listener.
pub fn hear_path(g: &mut crate::game::Game, src: V3, ear: V3, keep: Option<bool>) -> crate::scene_collision::Propagation {
    let p = crate::scene_collision::sound_paths_propagate(g, src, ear, Some(keep.unwrap_or(true)));
    let k = weather_hear_mask(g, ear);
    if k == 1.0 { p } else { crate::scene_collision::Propagation { path_len: p.path_len * k, ..p } }
}

/// The length a loud sound's path is taken at: the weather's stretch taken back out.
pub fn loud_len(g: &mut crate::game::Game, path_len: f64, ear: V3) -> f64 {
    let k = weather_hear_mask(g, ear);
    if k == 1.0 { path_len } else { path_len / k }
}

/// A stroke of lightning under a picked weather, added to the six faces of the light meter's stand-in.
pub fn stroke_light(g: &mut crate::game::Game, p: V3, acc: &mut [f64; 6]) {
    let l = if g.weather.kind_now.is_some() { g.weather.flash.as_ref().map(|f| f.light) } else { None };
    let Some(l) = l else { return };
    let (enabled, color, intensity, dir) = { let r = &g.tab.light[l]; (r.enabled, r.color, r.intensity, r.dir) };
    if !enabled { return; }
    let i = (0.2126 * color[0] + 0.7152 * color[1] + 0.0722 * color[2]) * intensity;
    if !(i > 0.002) { return; }
    let (tx, ty, tz) = (-dir[0], -dir[1], -dir[2]);
    let t = if ty > 0.05 { m::min(60.0, m::max(0.0, (crate::scene_world::world().ceiling_y + 6.0 - p[1]) / ty)) } else { 60.0 };
    if t > 0.05 && crate::scene_collision::segment_blocked(g, p, [p[0] + tx * t, p[1] + ty * t, p[2] + tz * t], None, None, None) { return; }
    if tx > 0.0 { acc[0] += i * tx; } else { acc[1] -= i * tx; }
    if ty > 0.0 { acc[2] += i * ty; } else { acc[3] -= i * ty; }
    if tz > 0.0 { acc[4] += i * tz; } else { acc[5] -= i * tz; }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    /// `dirOf`'s doubles from the TypeScript, under bun with EXACT on.
    #[test]
    fn dir_of_has_the_typescript_bits() {
        let rows: [(u64, u64, [u64; 3]); 5] = [
            (0x3fe6666666666666, 0x3fd999999999999a, [0xbfe2fcd60b856028, 0xbfd8ec3ae92b676b, 0x3fe68afcebc95d21]),
            (0x4002666666666666, 0x3ff199999999999a, [0xbfd5a5df199aaf8d, 0xbfec84c54c2d6338, 0xbfd3579379410117]),
            (0x0000000000000000, 0x3fe0000000000000, [0x8000000000000000, 0xbfdeaee8744b05f0, 0x3fec1528065b7d50]),
            (0xbffe666666666666, 0x3fa999999999999a, [0x3fee3e66f827a4f7, 0xbfa996dea2ff643c, 0xbfd4aa281baef78e]),
            (0x3ff921fb54442d18, 0x3feccccccccccccd, [0xbfe3e43a9692e21d, 0xbfe91103985da841, 0x3c85f10d9d62342f]),
        ];
        for (b, e, want) in rows {
            let d = dir_of(f64::from_bits(b), f64::from_bits(e));
            assert_eq!([d[0].to_bits(), d[1].to_bits(), d[2].to_bits()], want);
        }
    }
}
