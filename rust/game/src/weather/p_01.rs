//! src/game/weather.ts lines 1 to 469
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- seams: what skeleton 3 still lacks (each has its line in notes.md)

/// Makes the `RtLight` literal of strike(), the TypeScript's own fields. No type or table behind a `LightId`; `light_from_def` cannot say `peakIntensity` or `transient`.
#[allow(clippy::too_many_arguments)]
fn seam_light_new(g: &mut Game, name: &str, group: &str, kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner_deg: f64, outer_deg: f64, radius: f64, volumetric: f64, owner: f64, enabled: bool, fixture_box: f64, flicker: f64, ttl: f64, age: f64, peak_intensity: f64, transient: bool) -> LightId { todo!("needs: a new light from RtLight's fields (weather.ts:330)") }
/// `light.intensity = i; light.enabled = on`.
fn seam_light_set(g: &mut Game, l: LightId, intensity: f64, enabled: bool) { todo!("needs: writing a light's intensity and enabled by LightId (weather.ts:391)") }
/// `light.intensity`, for the panel's line.
fn seam_light_intensity(g: &mut Game, l: LightId) -> f64 { todo!("needs: reading a light's intensity by LightId (weather.ts:467)") }
/// `g.audio.play(name, null, gain, { lowpass, rate, pan, wet })`: game_audio_like_play's opts hold `crate::Opaque`, which carries no number.
fn seam_audio_play(g: &mut Game, name: &str, gain: f64, lowpass: f64, rate: f64, pan: f64, wet: f64) { todo!("needs: game_audio_like_play's opts to carry lowpass, rate, pan, wet (weather.ts:406)") }

// ---- this file's reach of a box

/// A static box by its ID: the rows of `g.col.boxes` (properties.txt's `g.none[id]` does not exist; see notes).
#[inline]
fn bx(g: &Game, b: BoxId) -> &crate::scene_boxes::Box { &g.col.boxes[b] }

/// `g.level.boxes` as IDs: that list and `g.col.boxes` are one array in the TypeScript (game.ts:1219), and a level's box is the row of its place in the list.
fn level_boxes(g: &Game) -> Vec<BoxId> {
    let n = if g.col.boxes.len() < g.level.boxes.len() { g.col.boxes.len() } else { g.level.boxes.len() };
    (0..n).map(|i| BoxId(i as u32)).collect()
}

/// Paints a box: its row, and the level's copy where it is a level's box (`level.boxes` and `col.boxes` are one array in the TypeScript).
fn paint(g: &mut Game, b: BoxId, albedo: V3, flags: f64, rough: Option<f64>, aniso: Option<bool>) {
    let x = &mut g.col.boxes[b];
    x.albedo = albedo; x.flags = flags; x.rough = rough; x.aniso = aniso;
    if let Some(x) = g.level.boxes.get_mut(b.0 as usize) {
        x.albedo = albedo; x.flags = flags; x.rough = rough; x.aniso = aniso;
    }
}

/// `WEATHER_PRESETS[kind]`.
fn preset(g: &Game, kind: crate::weather_kinds::WeatherKind) -> &crate::weather_kinds::WeatherPreset { g.m.weather_kinds.weather_presets.get(kind).unwrap() }

/// `x.toFixed(d)` for the panel's line. (A tie on an exact binary value rounds to even here, up there: nothing reads this line but a person.)
fn fixed(x: f64, d: usize) -> String { format!("{:.*}", d, x) }

// ---- the file's free functions

/// The level's roofs as footprints: its ceiling slabs, biggest first, at most MAX_ROOFS. (weather.ts roofsOf)
pub fn roofs_of(g: &mut crate::game::Game, boxes: &[BoxId]) -> Vec<[f64; 4]> {
    let ceiling_y = crate::scene_world::world().ceiling_y;
    let mut out: Vec<[f64; 4]> = Vec::new();
    for &b in boxes {
        let (c, h) = (bx(g, b).c, bx(g, b).h);
        let y0 = c[1] - h[1];
        let y1 = c[1] + h[1];
        if !(y0 >= ceiling_y - 0.25 && y1 <= ceiling_y + 0.45) || h[0] < 0.45 || h[2] < 0.45 { continue; }
        out.push([c[0] - h[0] - ROOF_PAD, c[2] - h[2] - ROOF_PAD, c[0] + h[0] + ROOF_PAD, c[2] + h[2] + ROOF_PAD]);
    }
    let area = |r: &[f64; 4]| (r[2] - r[0]) * (r[3] - r[1]);
    js::sort_by(&mut out, |a, b| area(b) - area(a));
    out.truncate(crate::weather_kinds::MAX_ROOFS as usize);
    out
}

/// A ground slab a picked rain wets: the ground's top, drawn, not small, and some of it under open sky. (weather.ts openGround)
pub fn open_ground(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool {
    let (c, h, flags) = (bx(g, b).c, bx(g, b).h, bx(g, b).flags);
    let top = c[1] + h[1];
    let f = crate::scene_boxes::box_flag();
    let skip = js::to_i32(f.no_primary) | js::to_i32(f.glass) | js::to_i32(f.dynamic);
    if !(top > -0.011 && top < 0.003) || h[0] * h[2] < 4.0 || (js::to_i32(flags) & skip) != 0 { return false; }
    sky_over(g, b, roofs)
}

/// Of 25 spots over the box's rectangle, how many have no roof over them, counted no further than `enough`. (weather.ts skySpots)
pub fn sky_spots(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]], enough: f64, step: f64) -> f64 {
    let (c, h, yaw) = (bx(g, b).c, bx(g, b).h, bx(g, b).yaw);
    let cs = m::cos(yaw);
    let sn = m::sin(yaw);
    let mut n = 0.0;
    for i in 0..5 {
        for j in 0..5 {
            let lx = h[0] * (i as f64 - 2.0) * step;
            let lz = h[2] * (j as f64 - 2.0) * step;
            let x = c[0] + cs * lx + sn * lz;
            let z = c[2] - sn * lx + cs * lz;
            if !roofs.iter().any(|r| crate::weather_kinds::in_rect(r, x, z)) {
                n += 1.0;
                if n >= enough { return n; }
            }
        }
    }
    n
}

/// Rain falls on SOME of it: a ground slab's test. (weather.ts skyOver)
pub fn sky_over(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { sky_spots(g, b, roofs, 1.0, 0.45) >= 1.0 }

/// Rain falls on MOST of it: the test of a thing laid for one place. (weather.ts skyOverMost)
pub fn sky_over_most(g: &mut crate::game::Game, b: BoxId, roofs: &[[f64; 4]]) -> bool { sky_spots(g, b, roofs, 13.0, 0.4) >= 13.0 }

/// A level that carries its own places: the office's footprints are not its building's. (weather.ts ownFloor)
pub fn own_floor(g: &mut crate::game::Game) -> bool {
    match &g.level.scenarios { Some(s) => s.quiet.is_some(), None => false }
}

/// mulberry32: the weather's own dice. (weather.ts rng32)
pub fn rng32(seed: f64) -> crate::weather::WeatherDice {
    WeatherDice { a: js::to_u32(seed) as f64 }
}

/// Mean of the stroke's envelope over [a, b]. (weather.ts strokeMean)
pub fn stroke_mean(s: &crate::weather::Stroke, a: f64, b: f64) -> f64 {
    if b <= a { return stroke_at(s, a); }
    let mut e = 0.0;
    for p in &s.pulses {
        let lo = m::max(a, p.t0);
        if b <= lo { continue; }
        e += p.amp * p.tau * (m::exp(-(lo - p.t0) / p.tau) - m::exp(-(b - p.t0) / p.tau));
    }
    e / (b - a)
}

/// The stroke's envelope at `t`. (weather.ts strokeAt)
pub fn stroke_at(s: &crate::weather::Stroke, t: f64) -> f64 {
    let mut e = 0.0;
    for p in &s.pulses {
        if t >= p.t0 { e += p.amp * m::exp(-(t - p.t0) / p.tau); }
    }
    e
}

// ---- class Weather

/// Master switch, read. (weather.ts Weather.enabled)
pub fn weather_enabled(g: &mut crate::game::Game) -> bool { g.weather.on }

/// Master switch, set: tears down exactly as setEnabled(false) does. (weather.ts Weather.enabled)
pub fn weather_set_enabled(g: &mut crate::game::Game, v: bool) { weather_set_enabled_2(g, v) }

/// Cloud cover 0..1 for whoever owns the sun and the moon to read. (weather.ts Weather.cloud)
pub fn weather_cloud(g: &mut crate::game::Game) -> f64 {
    if !g.weather.on { return 0.0; }
    match g.weather.kind { Some(k) => preset(g, k).cloud, None => legacy().cloud }
}

/// A draw of the weather's own die. (weather.ts Weather.rnd)
pub fn weather_rnd(g: &mut crate::game::Game) -> f64 { g.weather.dice.roll() }

/// The paint a box had before the wet lot darkened it, if it is one of those. (weather.ts Weather.dryPaint)
pub fn weather_dry_paint(g: &mut crate::game::Game, b: BoxId) -> Option<[f64; 3]> {
    for t in &g.weather.touched {
        if t.box_ == b { return Some(t.albedo); }
    }
    None
}

/// The weather a new game is born in. Runs on `g.weather` (every method's path) and returns what the caller stores there. (weather.ts Weather.constructor)
pub fn weather_new(g: &mut crate::game::Game) -> crate::weather::Weather {
    g.weather = Weather::blank();
    let boot = g.m.weather_kinds.weather_boot;
    let born = match weather_level_kind(g) {
        Some(k) => Some(k),
        None => if boot.kind == Some(crate::weather_kinds::WeatherKind::Snow) { None } else { boot.kind },
    };
    if born.is_none() && own_floor(g) { weather_own_night(g); }
    g.weather.wet = if g.weather.rain_rate > 0.0 { 1.0 } else { 0.0 };
    if born.is_some() { set_kind_at(g, born, false); } else { sync_roofs(g, false, false); }
    g.weather.clone()
}

/// A level built for snow is born in the kind 'snow' and stays in it; None = any other level. (weather.ts Weather.levelKind)
pub fn weather_level_kind(g: &mut crate::game::Game) -> Option<crate::weather_kinds::WeatherKind> {
    let name = crate::scene_level_load::level_name_of(g);
    let boot = g.m.weather_kinds.weather_boot;
    if crate::weather_kinds::snow_level(g, name.as_deref()) && (!boot.locked || boot.kind == Some(crate::weather_kinds::WeatherKind::Snow)) { Some(crate::weather_kinds::WeatherKind::Snow) } else { None }
}

/// No weather picked on a floor that is none of the office's: its own night is dry and still, under its own roofs. (weather.ts Weather.ownNight)
pub fn weather_own_night(g: &mut crate::game::Game) {
    g.weather.storm = 0.0;
    g.weather.rain_rate = 0.0;
    g.weather.wind = [0.0, 0.0];
    g.weather.fog_word = 0.0;
    g.weather.fog = None;
    g.weather.roofs = weather_own_roofs(g);
}

/// The level's own storm is in force, cut by the level's own roofs (WX_OWN.roofs). (weather.ts Weather.ownStorm)
pub fn weather_own_storm(g: &mut crate::game::Game) -> bool { g.weather.own }

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
    let own = g.weather.kind.is_none() && g.k.wx_own.roofs && g.m.weather_kinds.weather_boot.packed && !own_floor(g) && !weather_office_fit(g);
    if own == g.weather.own && !tell_wind { return; }
    if own != g.weather.own {
        g.weather.own = own;
        weather_set_wet(g, false);   // (what is wet hangs on the roofs: put the ground back, the next update wets it by the new rule)
        if g.weather.kind.is_none() {
            g.weather.roofs = if own { weather_own_roofs(g) } else { vec![interior(), annex()] };
        }
    }
    if !built { return; }
    let roofs: Option<Vec<Vec<f64>>> = if g.weather.kind.is_some() || own { Some(g.weather.roofs.iter().map(|r| r.to_vec()).collect()) } else { None };
    crate::wind::wind_reroof(g, roofs.as_deref(), Some(own));
}

/// `roofs` are the level's own and in force. (weather.ts Weather.levelRoofs)
pub fn weather_level_roofs(g: &mut crate::game::Game) -> bool { g.weather.kind.is_some() || g.weather.own }

/// The level's own ceiling slabs as the no-rain footprints (roofs 0 / 1 exact, 2 / 3 as the Frame packs them). (weather.ts Weather.ownRoofs)
pub fn weather_own_roofs(g: &mut crate::game::Game) -> Vec<[f64; 4]> {
    let boxes = level_boxes(g);
    roofs_of(g, &boxes).into_iter().enumerate().map(|(i, r)| if i < 2 { r } else { crate::weather_kinds::unpack_rect(crate::weather_kinds::pack_rect(Some(r))) }).collect()
}

/// Put the level in a weather (None = back as it has always been). False = refused. (weather.ts Weather.setKind)
pub fn weather_set_kind(g: &mut crate::game::Game, kind: Option<crate::weather_kinds::WeatherKind>) -> bool { set_kind_at(g, kind, true) }

/// setKind, with `built` false while the constructor runs: `g.wind` and `g.airDust` are built after this object.
fn set_kind_at(g: &mut Game, kind: Option<crate::weather_kinds::WeatherKind>, built: bool) -> bool {
    let own = weather_level_kind(g);
    if own.is_some() && kind != own { return false; }   // a level built for snow stays in it
    let boot = g.m.weather_kinds.weather_boot;
    if boot.locked && kind != boot.kind { return false; }
    weather_set_wet(g, false);   // (what is wet depends on the kind)
    g.weather.kind = kind;
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
        g.weather.fog_word = crate::weather_kinds::pack_fog(fog);
        g.weather.fog = crate::weather_kinds::unpack_fog(g.weather.fog_word);
        g.weather.roofs = weather_own_roofs(g);
    } else if own_floor(g) {
        weather_own_night(g);
    } else {
        let l = legacy();
        g.weather.storm = l.storm;
        g.weather.rain_rate = l.rain;
        g.weather.wind = [l.wind[0], l.wind[1]];
        g.weather.fog_word = 0.0;
        g.weather.fog = None;
        g.weather.roofs = vec![interior(), annex()];
    }
    g.weather.wet = if g.weather.on && g.weather.rain_rate > 0.0 { 1.0 } else { 0.0 };
    sync_roofs(g, true, built);
    if built { crate::air_dust::air_dust_reseed(g); }
    crate::hours::hour_weather_changed(g);
    true
}

/// Can a checkpoint's weather be put in here? None, or the reason. (weather.ts Weather.refuses)
pub fn weather_refuses(g: &mut crate::game::Game, s: Option<&crate::weather::WeatherSnap>) -> Option<String> {
    let asked = s.and_then(|s| s.kind);
    let k = if crate::weather_kinds::is_any_kind(g, asked) { asked } else { None };
    let own = weather_level_kind(g);
    if let Some(o) = own {
        if k != own { return Some(format!("made in other weather ({}; this level is in {})", kind_words(k), o.as_str())); }
    }
    if g.m.weather_kinds.weather_boot.locked && k != g.weather.kind {
        return Some(format!("made in other weather ({}; this page is in {}) — open the mission in that weather to load it", kind_words(k), kind_words(g.weather.kind)));
    }
    None
}

/// `k ?? 'the level\'s own'`.
fn kind_words(k: Option<crate::weather_kinds::WeatherKind>) -> &'static str {
    match k { Some(k) => k.as_str(), None => "the level's own" }
}

/// The footprints in force now, as underRoof reads them. (weather.ts Weather.roofsNow)
pub fn weather_roofs_now(g: &mut crate::game::Game) -> Vec<[f64; 4]> {
    weather_sync_roofs(g, None);
    g.weather.roofs.clone()
}

/// Is (x, z) under one of the footprints the rain is cut by. (weather.ts Weather.underRoof)
pub fn weather_under_roof(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
    weather_sync_roofs(g, None);
    g.weather.roofs.iter().any(|r| crate::weather_kinds::in_rect(r, x, z))
}

/// The rain bed's level at a listener (0..1). (weather.ts Weather.rainBed)
pub fn weather_rain_bed(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    if !(g.weather.rain_rate > 0.001) { return 0.0; }
    let bed = 0.35 + 0.65 * g.weather.rain_rate;
    bed * (if weather_under_roof(g, x, z) { g.k.wx_sim.roof_bed } else { 1.0 })
}

/// A kind is picked: the simulation's half of the weather runs. (weather.ts Weather.simLive)
pub fn weather_sim_live(g: &mut crate::game::Game) -> bool { g.weather.kind.is_some() }

/// HEARING: the factor (>= 1) a sound's path is stretched by for a listener at `ear`. (weather.ts Weather.hearMask)
pub fn weather_hear_mask(g: &mut crate::game::Game, ear: V3) -> f64 {
    if !weather_sim_live(g) { return 1.0; }
    let rain_k = g.k.wx_sim.rain_k;
    let mut k = 1.0 + rain_k * weather_rain_bed(g, ear[0], ear[2]);
    if g.time < g.weather.thunder_until {
        let thunder_k = g.k.wx_sim.thunder_k;
        k += thunder_k * (if weather_under_roof(g, ear[0], ear[2]) { 0.6 } else { 1.0 });
    }
    k
}

/// SIGHT: the fog's transmittance along a to b; exactly 1 with no kind picked or no fog in it. (weather.ts Weather.airTrans)
pub fn weather_air_trans(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let f = match g.weather.fog { Some(f) => f, None => return 1.0 };
    if !weather_sim_live(g) { return 1.0; }
    let mut leak = 0.0;
    if crate::air_dust::air_dust_live(g) {
        let n = m::min(24.0, m::max(4.0, m::ceil(m::hyp2(b[0] - a[0], b[2] - a[2]) / 0.75)));
        let mut i = 0.0;
        while i < n {
            let t = (i + 0.5) / n;
            leak += crate::air_dust::air_dust_share_at(g, a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t);
            i += 1.0;
        }
        return m::exp(-crate::weather_kinds::fog_tau(f, a, b) * leak / n);
    }
    for i in 0..4 {
        let t = (i as f64 + 0.5) / 4.0;
        leak += if weather_under_roof(g, a[0] + (b[0] - a[0]) * t, a[2] + (b[2] - a[2]) * t) { f.leak } else { 1.0 };
    }
    m::exp(-crate::weather_kinds::fog_tau(f, a, b) * leak / 4.0)
}

/// The weather's dice, reseeded. (weather.ts Weather.seed)
pub fn weather_seed(g: &mut crate::game::Game, n: f64) {
    g.weather.dice = crate::dice::Die::new(js::to_u32(rng32(n).a));
    g.weather.next_flash_at = -1.0;
}

/// Everything the storm's next minutes hang on; a stroke in progress is not carried. hours.ts holds a level's own storm in one, so it is a rule's call and not only a checkpoint's. (weather.ts Weather.snapshot)
pub fn weather_snapshot(g: &mut crate::game::Game) -> crate::weather::WeatherSnap {
    let w = &g.weather;
    WeatherSnap {
        kind: w.kind,
        thunder_until: if w.kind.is_some() { Some(w.thunder_until) } else { None },
        on: w.on,
        storm: w.storm,
        rain_rate: w.rain_rate,
        wind: [w.wind[0], w.wind[1]],
        wet_gloss: w.wet_gloss,
        wet: w.wet,
        next_flash_at: w.next_flash_at,
        thunders: w.thunders.clone(),
        last_bark_t: w.last_bark_t,
        rng: w.dice.s as f64,
        stats: WeatherSnapStats { flashes: w.stats.flashes, thunders: w.stats.thunders, barks: w.stats.barks },
    }
}

/// `Number.isFinite(x) ? x : d`. (weather.ts Weather.restore f)
fn finite_or(x: f64, d: f64) -> f64 { if x.is_finite() { x } else { d } }

/// The storm as a snapshot had it; on / off stays the session's. (weather.ts Weather.restore)
pub fn weather_restore(g: &mut crate::game::Game, s: &crate::weather::WeatherSnap) {
    weather_end_flash(g);
    let k = if crate::weather_kinds::is_any_kind(g, s.kind) { s.kind } else { None };
    if k != g.weather.kind { weather_set_kind(g, k); }   // the kind is the file's: the run it holds was played under it
    let own = weather_level_kind(g);
    if own.is_some() && k != own { return; }   // made in other weather on a level that keeps its own: none of its rain, wind or wet comes in
    let w = &mut g.weather;
    w.thunder_until = finite_or(s.thunder_until.unwrap_or(-1e9), -1e9);
    w.storm = finite_or(s.storm, w.storm);
    w.rain_rate = finite_or(s.rain_rate, w.rain_rate);
    w.wind = [finite_or(s.wind[0], w.wind[0]), finite_or(s.wind[1], w.wind[1])];
    w.wet_gloss = s.wet_gloss;
    w.wet = finite_or(s.wet, w.wet);
    w.next_flash_at = finite_or(s.next_flash_at, -1.0);
    w.thunders = s.thunders.iter().map(|t| Thunder { at: finite_or(t.at, 0.0), dist_km: finite_or(t.dist_km, 1.0), bearing: finite_or(t.bearing, 0.0), loud: finite_or(t.loud, 0.5) }).collect();
    w.last_bark_t = finite_or(s.last_bark_t, -1e9);
    if s.rng.is_finite() { w.dice.s = js::to_u32(s.rng); }
    w.stats.flashes = js::to_i32(s.stats.flashes) as f64;
    w.stats.thunders = js::to_i32(s.stats.thunders) as f64;
    w.stats.barks = js::to_i32(s.stats.barks) as f64;
    weather_sync_roofs(g, None);   // a replay's knobs are put in after this object was made: the footprints follow the rule now in force
}

/// Switch the storm on or off; a picked weather stays on. (weather.ts Weather.setEnabled)
pub fn weather_set_enabled_2(g: &mut crate::game::Game, on: bool) {
    if on == g.weather.on { return; }
    if !on && g.weather.kind.is_some() { return; }
    g.weather.on = on;
    if !on { weather_teardown(g); } else { g.weather.wet = if g.weather.rain_rate > 0.0 { 1.0 } else { 0.0 }; }
}

/// Fire a stroke now. The Flash returned is a copy; the live one is `g.weather.flash`. (weather.ts Weather.strike)
pub fn weather_strike(g: &mut crate::game::Game, opts: Option<crate::weather::WeatherStrikeOpts>) -> Option<crate::weather::Flash> {
    let opts = opts.unwrap_or_default();
    if !g.weather.on || g.weather.suspended { return None; }
    if g.weather.flash.is_some() { weather_end_flash(g); }
    let bearing = match opts.bearing { Some(x) => x, None => weather_rnd(g) * m::PI * 2.0 };
    let elev = (match opts.elev_deg { Some(x) => x, None => 24.0 + weather_rnd(g) * 30.0 }) * m::PI / 180.0;
    let dist_km = match opts.dist_km {
        Some(x) => x,
        None => 0.3 + m::pow(weather_rnd(g), 1.3) * 3.7 * (1.15 - 0.5 * g.weather.storm),   // a heavier storm sits closer
    };
    let peak = match opts.peak {
        Some(x) => x,
        None => FLASH_PEAK * (0.55 + 0.9 * weather_rnd(g)) * v3::clamp(1.25 - dist_km * 0.18, 0.45, 1.2),
    };
    // two return strokes, sometimes a third flicker
    let amp2 = 0.35 + 0.45 * weather_rnd(g);
    let mut stroke = Stroke { pulses: vec![StrokePulsesItem { t0: 0.0, amp: 1.0, tau: 0.022 }, StrokePulsesItem { t0: 0.085, amp: amp2, tau: 0.05 }] };
    if weather_rnd(g) < 0.3 {
        let t0 = 0.2 + 0.05 * weather_rnd(g);
        let amp = 0.25 + 0.25 * weather_rnd(g);
        stroke.pulses.push(StrokePulsesItem { t0, amp, tau: 0.03 });
    }
    let dir = dir_of(bearing, elev);
    let light = seam_light_new(g, "lightning", "weather", 2.0, [0.0, 0.0, 0.0], dir, v3::copy(flash_color()), 0.0, 1e6, 0.0, 0.0, 0.006, 1.3, 0.0, true, -1.0, 0.0, -1.0, 0.0, peak, true);
    crate::render_lights::light_set_add(&mut g.engine.lights, light);
    g.weather.flash = Some(Flash { light, stroke, t: 0.0, peak, dir, bearing, dist_km });
    let at = g.time + dist_km * 1000.0 / SPEED_OF_SOUND;
    g.weather.thunders.push(Thunder { at, dist_km, bearing, loud: v3::clamp(1.15 - dist_km * 0.22, 0.25, 1.0) });
    g.weather.stats.flashes += 1.0;
    weather_apply_flash(g, 0.0, 1.0 / 60.0);   // lit from this very frame
    g.weather.flash.clone()
}

/// Once a frame from Game.update: the clock, the strokes, the flash lamp, the thunder due. The frame's numbers are the picture's (notes). (weather.ts Weather.update)
pub fn weather_update(g: &mut crate::game::Game, dt: f64, cam_yaw: Option<f64>) {
    g.weather.cam_yaw = cam_yaw.unwrap_or(0.0);
    weather_sync_roofs(g, None);
    if !g.weather.on { return; }
    // day: the storm holds still and shows nothing; the night gets it back as it was
    let day = crate::lighting::is_daylight(g) && g.weather.kind.is_none();
    if day != g.weather.suspended {
        g.weather.suspended = day;
        if day {
            if g.weather.flash.is_some() { weather_end_flash(g); }
            g.weather.thunders.clear();
            weather_set_wet(g, false);
            crate::audio_iface::game_audio_like_rain(g, 0.0);
        }
        g.weather.next_flash_at = -1.0;
    }
    if g.weather.suspended { return; }
    let raining = g.weather.rain_rate > 0.001;
    g.weather.wet = v3::clamp(g.weather.wet + (if raining { dt } else { -dt }) / WET_SECS, 0.0, 1.0);
    let wet = g.weather.wet > 0.02;
    weather_set_wet(g, wet);
    // ---- strokes
    if g.weather.next_flash_at < 0.0 { g.weather.next_flash_at = g.time + weather_gap(g, true); }
    if g.weather.storm > 0.01 && g.time >= g.weather.next_flash_at {
        weather_strike(g, None);
        g.weather.next_flash_at = g.time + weather_gap(g, false);
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
    let mut i = g.weather.thunders.len();
    while i > 0 {
        i -= 1;
        let th = g.weather.thunders[i];
        if g.time >= th.at {
            g.weather.thunders.remove(i);
            weather_play_thunder(g, th);
        }
    }
    // ---- the rain bed follows the rate and where the listener stands
    let seat = crate::game::game_host_seat(g);
    let man = crate::cmd::seat_man(g, seat);
    let pp = g.ch[g.pl[man].ch].pos;
    let bed = weather_rain_bed(g, pp[0], pp[2]);
    crate::audio_iface::game_audio_like_rain(g, bed);
}

/// Seconds to the next stroke; the first one of a night comes sooner. (weather.ts Weather.gap)
pub fn weather_gap(g: &mut crate::game::Game, first: bool) -> f64 {
    if first { return 6.0 + 6.0 * weather_rnd(g); }
    let mean = v3::clamp(FLASH_MEAN_GAP / m::max(g.weather.storm, 0.05), 8.0, 25.0);
    mean * (0.5 + weather_rnd(g))
}

/// The stroke's light over [t, t + dt]. (weather.ts Weather.applyFlash)
pub fn weather_apply_flash(g: &mut crate::game::Game, t: f64, dt: f64) {
    let f = g.weather.flash.as_ref().unwrap();
    let intensity = f.peak * stroke_mean(&f.stroke, t, t + m::max(dt, 1e-4));
    let enabled = intensity > 1e-3 * f.peak;
    let light = f.light;
    seam_light_set(g, light, intensity, enabled);
}

/// The stroke's light leaves the set. (weather.ts Weather.endFlash)
pub fn weather_end_flash(g: &mut crate::game::Game) {
    let light = match g.weather.flash.as_ref() { Some(f) => f.light, None => return };
    crate::render_lights::light_set_remove(&mut g.engine.lights, light);
    g.weather.flash = None;
}

/// Thunder reaches the floor: the sound, the noise it drowns, and now and then a calm man's remark. (weather.ts Weather.playThunder)
pub fn weather_play_thunder(g: &mut crate::game::Game, th: crate::weather::Thunder) {
    // near: the crack and the clap on top of the roll; far: the roll alone, darker and slower
    let near = th.dist_km < 1.3;
    let name = if near { "thunderNear" } else { "thunderFar" };
    let gain = (if near { 1.0 } else { 0.85 }) * th.loud * (0.6 + 0.4 * g.weather.storm);
    let lowpass = v3::clamp(9000.0 * m::exp(-th.dist_km * 0.75), 500.0, 9000.0);
    let rate = v3::clamp(1.02 - th.dist_km * 0.06, 0.78, 1.02) * (0.96 + 0.08 * weather_rnd(g));
    let cy = m::cos(g.weather.cam_yaw);
    let sy = m::sin(g.weather.cam_yaw);
    let bx = m::sin(th.bearing);
    let bz = -m::cos(th.bearing);
    let right = bx * cy - bz * sy;
    seam_audio_play(g, name, gain, lowpass, rate, v3::clamp(right * 0.6, -0.6, 0.6), if near { 0.35 } else { 0.55 });
    g.weather.stats.thunders += 1.0;
    g.weather.stats.last_thunder = Some(WeatherStatsLastThunder { at: th.at, dist_km: th.dist_km, bearing: th.bearing, loud: th.loud, played_at: g.time, name: String::from(name), gain });
    if g.weather.kind.is_some() && th.loud > g.k.wx_sim.thunder_loud { g.weather.thunder_until = g.time + g.k.wx_sim.thunder_secs; }
    // the three dice for a remark are thrown on EVERY thunder, whoever is on screen: same seed, same storm
    let r_bark = weather_rnd(g);
    let r_who = weather_rnd(g);
    let r_line = weather_rnd(g);
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
                if !crate::scene_collision::solid_blocked(g, [p[0], 1.9, p[2]], [p[0], 10.9, p[2]], None) { line = crate::words::wg(g, &THUNDER_OUT_GUIDE, &THUNDER_OUT); }   // he is out in it
            }
            crate::game::game_say(g, gd, line, None, None, None);
            g.weather.stats.barks += 1.0;
            g.weather.last_bark_t = g.time;
        }
    }
}

/// Is the ground wet now, as this weather last judged it. (weather.ts Weather.groundWet)
pub fn weather_ground_wet(g: &mut crate::game::Game) -> bool { g.weather.wet_on }

/// The wet lot: the asphalt slab darkened and glossy, or everything put back. Idempotent both ways. (weather.ts Weather.setWet)
pub fn weather_set_wet(g: &mut crate::game::Game, on: bool) {
    if on == g.weather.wet_on { return; }
    g.weather.wet_on = on;
    if on {
        // the lot's ground: the one big slab whose top is the y = 0 the cars stand on (40 x 28 m: the interior carpet base is 32 x 20 and must never match)
        let asphalt = g.engine.world.statics.iter().copied().find(|&b| {
            let (c, h) = (bx(g, b).c, bx(g, b).h);
            c[1] < 0.0 && c[1] + h[1] > -0.01 && h[0] > 18.0 && h[2] > 12.0
        });
        if let Some(b) = asphalt { soak(g, b); }
        g.weather.lot = asphalt;
    } else {
        let touched = core::mem::take(&mut g.weather.touched);
        for t in &touched { paint(g, t.box_, t.albedo, t.flags, t.rough, t.aniso); }
        g.weather.lot = None;
    }
    // the rest of the wet ground is the picture's and follows in this very call
    crate::wet_ground::wet_ground_follow(g);
}

/// Darkens a box and remembers what to put back. (weather.ts Weather.setWet soak)
fn soak(g: &mut Game, b: BoxId) {
    let (albedo, flags, rough, aniso) = { let x = bx(g, b); (x.albedo, x.flags, x.rough, x.aniso) };
    g.weather.touched.push(WeatherTouchedItem { box_: b, albedo: v3::copy(albedo), flags, rough, aniso });
    let gloss = crate::scene_boxes::box_flag().gloss;
    paint(g, b, [albedo[0] * WET_ALBEDO, albedo[1] * WET_ALBEDO, albedo[2] * WET_ALBEDO * 1.04], (js::to_i32(flags) | js::to_i32(gloss)) as f64, Some(WET_ROUGH), Some(true));
}

/// Everything off and put back. (weather.ts Weather.teardown)
pub fn weather_teardown(g: &mut crate::game::Game) {
    weather_end_flash(g);
    g.weather.thunders.clear();
    g.weather.next_flash_at = -1.0;
    weather_set_wet(g, false);
    g.weather.wet = 0.0;
    crate::audio_iface::game_audio_like_rain(g, 0.0);
}

/// One line for the panel and the probes. (weather.ts Weather.summary)
pub fn weather_summary(g: &mut crate::game::Game) -> String {
    let mut out = String::new();
    if g.weather.own { out.push_str(&format!("own roofs {} · ", g.weather.roofs.len())); }
    if let Some(k) = g.weather.kind {
        out.push_str(k.as_str());
        if let Some(f) = g.weather.fog { out.push_str(&format!(" fog σ{}/H{}", fixed(f.sigma, 3), f.h)); }
        out.push_str(&format!(" cloud {} · ", fixed(weather_cloud(g), 2)));
    }
    if weather_enabled(g) {
        if g.weather.suspended { out.push_str("held (day)"); } else { out.push_str(&format!("storm {}", fixed(g.weather.storm, 2))); }
    } else {
        out.push_str("off");
    }
    out.push_str(&format!(" · rain {} wet {} · ", fixed(g.weather.rain_rate, 2), fixed(g.weather.wet, 2)));
    if let Some((t, light)) = g.weather.flash.as_ref().map(|f| (f.t, f.light)) {
        let i = seam_light_intensity(g, light);
        out.push_str(&format!("STROKE {} s I={}", fixed(t, 2), fixed(i, 1)));
    } else {
        out.push_str(&format!("next stroke in {} s", fixed(m::max(0.0, g.weather.next_flash_at - g.time), 0)));
    }
    if let Some(th) = g.weather.thunders.first().copied() {
        out.push_str(&format!(" · thunder in {} s ({} km)", fixed(th.at - g.time, 1), fixed(th.dist_km, 1)));
    }
    let st = &g.weather.stats;
    out.push_str(&format!(" · {} strokes {} thunders {} remarks", st.flashes, st.thunders, st.barks));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Bits from the TypeScript's own rng32 / strokeMean / strokeAt, run under bun with EXACT.on (first pass). The die is `dice::Die` now: the same bits must come off it.
    #[test]
    fn the_dice_draw_the_typescripts_bits() {
        let mut d = crate::dice::Die::new(js::to_u32(rng32(20260809.0).a));
        let want: [u64; 6] = WANT_DICE;
        for w in want { assert_eq!(d.roll().to_bits(), w); }
        assert_eq!((d.s as f64).to_bits(), WANT_DICE_A);
    }

    fn stroke() -> Stroke {
        Stroke { pulses: vec![StrokePulsesItem { t0: 0.0, amp: 1.0, tau: 0.022 }, StrokePulsesItem { t0: 0.085, amp: 0.61, tau: 0.05 }, StrokePulsesItem { t0: 0.23, amp: 0.3, tau: 0.03 }] }
    }

    #[test]
    fn the_envelope_is_the_typescripts_bits() {
        let s = stroke();
        for (i, (a, b)) in SPANS.iter().enumerate() {
            assert_eq!(stroke_mean(&s, *a, *b).to_bits(), WANT_MEAN[i], "mean {}", i);
            assert_eq!(stroke_at(&s, *a).to_bits(), WANT_AT[i], "at {}", i);
        }
    }

    const SPANS: [(f64, f64); 5] = [(0.0, 0.016666666666666666), (0.08, 0.1), (0.2, 0.2), (0.25, 0.3), (0.5, 0.25)];
    const WANT_DICE: [u64; 6] = [0x3fc035bf77800000, 0x3fc7ff8513000000, 0x3fe1075491200000, 0x3fdd6ff37d800000, 0x3fe37995e5800000, 0x3fd29f69bb400000];
    const WANT_DICE_A: u64 = 0x41e2074070e00000;
    const WANT_MEAN: [u64; 5] = [0x3fe670152ad1625c, 0x3fda675a22e0a5c0, 0x3faf5edb84c60892, 0x3fb6d4ef3206a694, 0x3f28b8fb354e6075];
    const WANT_AT: [u64; 5] = [0x3ff0000000000000, 0x3f9afaf70fa79f10, 0x3faf5edb84c60892, 0x3fc698b6ec1ac1bc, 0x3f28b8fb354e6075];
}
