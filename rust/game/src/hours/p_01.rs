//! src/game/hours.ts lines 1 to 431
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the hours' words

/// (hours.ts hourLabel)
pub fn hour_label(id_2: crate::hours::HourId) -> String {
    match hours().get(String::from(id_2.as_str())) { Some(d) => d.label.clone(), None => night().label }
}

/// (hours.ts hourClock)
pub fn hour_clock(id_2: crate::hours::HourId) -> String {
    match hours().get(String::from(id_2.as_str())) { Some(d) => d.clock.clone(), None => night().clock }
}

/// (hours.ts hourWords)
pub fn hour_words(id_2: crate::hours::HourId) -> String {
    match hours().get(String::from(id_2.as_str())) { Some(d) => d.words.clone(), None => night().words }
}

/// The hour the page stands at: with a game to hand, the game's (main.ts sets `HOUR.live` to just that). (hours.ts hourLive)
pub fn hour_live(g: &mut crate::game::Game) -> crate::hours::HourId {
    hour_of(g)
}

/// The enum of one of HOUR_IDS' words.
fn hour_word(s: &str) -> Option<HourId> {
    match s {
        "dawn" => Some(HourId::Dawn),
        "midday" => Some(HourId::Midday),
        "overcast" => Some(HourId::Overcast),
        "dusk" => Some(HourId::Dusk),
        "night" => Some(HourId::Night),
        _ => None,
    }
}

/// A word to an hour; empty or unknown = night. (hours.ts hourIdOf)
pub fn hour_id_of(v: Option<&str>) -> crate::hours::HourId {
    let s = match v { Some(v) => v.to_lowercase(), None => String::new() };
    if hour_ids().contains(&s) { return hour_word(&s).unwrap_or(HourId::Night); }
    match alias().get(s) { Some(a) => hour_word(a).unwrap_or(HourId::Night), None => HourId::Night }
}

/// The visit's ?hour= and ?sun=, before the stand-up. (hours.ts initHour)
pub fn init_hour(g: &mut crate::game::Game, q: crate::Opaque) {
    g.m.hours.hour.id = hour_id_of(seam_query_get(&q, "hour").as_deref());
    if let Some(s) = seam_query_number(&q, "sun") {
        g.m.hours.hour.sun = v3::clamp(s, 0.0, 1.0);
        g.m.hours.hour.sun_given = true;
    }
    if seam_query_has(&q, "noweather") { g.m.hours.hour.dry = false; }
}

/// The hour off a query handed to a running game; no `hour` key = nothing touched. (hours.ts hourFromQuery)
pub fn hour_from_query(g: &mut crate::game::Game, q: crate::Opaque) {
    if !seam_query_has(&q, "hour") { return; }
    apply_hour(g, hour_id_of(seam_query_get(&q, "hour").as_deref()));
    if let Some(s) = seam_query_number(&q, "sun") { set_sun_strength(g, s, None); }
}

/// A link with the hour on it (night = none); whatever hour it carried is replaced. (hours.ts hourHref)
pub fn hour_href(href: &str, id_2: Option<crate::hours::HourId>) -> String {
    let id = match id_2 { Some(id) => id, None => seam_visit_hour() };
    let mut parts = href.split('?');
    let path = parts.next().unwrap_or("");
    let query = parts.next().unwrap_or("");
    let mut keys: Vec<String> = Vec::new();
    for k in query.split('&') {
        if !k.is_empty() && k != "hour" && !k.starts_with("hour=") { keys.push(String::from(k)); }
    }
    if id != HourId::Night { keys.push(alloc::format!("hour={}", id.as_str())); }
    if keys.is_empty() { String::from(path) } else { alloc::format!("{}?{}", path, keys.join("&")) }
}

// ---------------------------------------------------------------- the hour on a Game

/// The hour's row, for a body that has already seen it there and calls nothing that deletes it.
fn hour_row(g: &mut Game) -> &mut HourState {
    g.m.hours.states.as_mut().unwrap()
}

/// The hour this game stands at; Night = the level as authored. (hours.ts hourOf)
pub fn hour_of(g: &mut crate::game::Game) -> crate::hours::HourId {
    match &g.m.hours.states { Some(st) => st.def.id, None => HourId::Night }
}

/// The hour's sun in this game's light set; None at night. (hours.ts hourSun)
pub fn hour_sun(g: &mut crate::game::Game) -> Option<LightId> {
    g.m.hours.states.as_ref().map(|st| st.sun)
}

/// (hours.ts isHourSun)
pub fn is_hour_sun(g: &mut crate::game::Game, l: LightId) -> bool {
    g.m.hours.states.as_ref().map(|st| st.sun) == Some(l)
}

/// Is this directional light already in the estimate, so nobody counts it twice. (hours.ts inEstimate)
pub fn in_estimate(g: &mut crate::game::Game, l: LightId) -> bool {
    is_hour_sun(g, l) || (g.k.sense.moon && g.m.hours.states.is_none() && sky_lamp_of(g) == Some(l))
}

/// The level's brightest steady directional light that is up, when no hour is laid over it. (hours.ts skyLampOf)
pub fn sky_lamp_of(g: &mut crate::game::Game) -> Option<LightId> {
    if g.m.hours.states.is_some() { return None; }
    let mut moon: Option<LightId> = None;
    let mut best = 0.002;
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        let row = &g.tab.light[l];
        if row.kind != 2.0 || !row.enabled || row.ttl >= 0.0 || row.transient.unwrap_or(false) || !(row.dir[1] < 0.0) { continue; }
        let intensity = lum(row.color) * row.intensity;
        if intensity > best { best = intensity; moon = Some(l); }
    }
    moon
}

/// The sun's strength 0‥1 on this game; 1 at night. (hours.ts sunStrength)
pub fn sun_strength(g: &mut crate::game::Game) -> f64 {
    match &g.m.hours.states { Some(st) => st.sun_k, None => 1.0 }
}

/// Was the sun's strength set by hand rather than left to the cloud. (hours.ts sunGiven)
pub fn sun_given(g: &mut crate::game::Game) -> bool {
    match &g.m.hours.states { Some(st) => st.given, None => false }
}

/// THE ONE NUMBER, live: the sun at k of the hour's own; `given` holds it against a picked weather's cloud. (hours.ts setSunStrength)
pub fn set_sun_strength(g: &mut crate::game::Game, k: f64, given: Option<bool>) {
    let given = given.unwrap_or(true);
    let Some(st) = g.m.hours.states.as_mut() else { return };
    st.sun_k = v3::clamp(if k.is_finite() { k } else { 1.0 }, 0.0, 1.0);
    st.given = given;
    let (sun, power) = (st.sun, st.def.sun.intensity * st.sun_k);
    set_light_power(g, sun, power);
    if !given { under_weather(g); }
}

// ---------------------------------------------------------------- the weather over the hour

/// The light on open ground under cloud cover c, of a clear sky's. (hours.ts cloudSun)
pub fn cloud_sun(c_2: f64) -> f64 {
    1.0 - 0.75 * m::pow(v3::clamp(c_2, 0.0, 1.0), 3.4)
}

/// How far cloud cover c has spread the sun and greyed the sky. (hours.ts cloudSoft)
pub fn cloud_soft(c_2: f64) -> f64 {
    v3::smoothstep(0.5, 1.0, c_2)
}

/// Colour c drained toward a deck's grey by w and scaled by k, its luminance otherwise kept. (hours.ts greyed)
pub fn greyed(c_2: V3, w: f64, k: Option<f64>) -> V3 {
    let k = k.unwrap_or(1.0);
    let l = lum(c_2) / GREY_LUM;
    let grey = grey();
    [
        (c_2[0] + (l * grey[0] - c_2[0]) * w) * k,
        (c_2[1] + (l * grey[1] - c_2[1]) * w) * k,
        (c_2[2] + (l * grey[2] - c_2[2]) * w) * k,
    ]
}

/// The sky's share of the light on open ground at an hour as authored. (hours.ts skyShare)
pub fn sky_share(d: &crate::hours::HourDef) -> f64 {
    let (sky, _) = ray_tables();
    let zen = lum(d.sky.zenith);
    let hor = lum(d.sky.horizon);
    let mut k = 0.0;
    for q in sky.iter() { k += (hor + (zen - hor) * v3::clamp(q.d[1] * 2.5, 0.0, 1.0)) * q.c6[2]; }
    let s = lum(d.sun.color) * d.sun.intensity * m::max(0.0, m::sin(d.sun.elev_deg * DEG));
    if k + s > 0.0 { k / (k + s) } else { 1.0 }
}

/// [sun, sky] under cover c, each as a factor of the hour's own. (hours.ts cloudLight)
pub fn cloud_light(d: &crate::hours::HourDef, c_2: f64) -> [f64; 2] {
    let ch = d.cloud.unwrap_or(0.0);
    let c = m::max(v3::clamp(c_2, 0.0, 1.0), ch);
    if c == ch { return [1.0, 1.0]; }
    let own = sky_share(d);
    let u = (c * c - ch * ch) / (1.0 - ch * ch);
    let f = cloud_sun(c) / cloud_sun(ch);
    [(1.0 - u) * f, if own > 1e-6 { (own + (1.0 - own) * u) / own * f } else { 1.0 }]
}

/// The light on level open ground under cover c as a factor of the hour's own. (hours.ts cloudTotal)
pub fn cloud_total(d: &crate::hours::HourDef, c_2: f64) -> f64 {
    let ch = d.cloud.unwrap_or(0.0);
    cloud_sun(m::max(v3::clamp(c_2, 0.0, 1.0), ch)) / cloud_sun(ch)
}

/// The weather this game is in laid over the hour's row: the storm's switch, the sun and the sky. (hours.ts underWeather)
pub fn under_weather(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_ref() else { return };
    let (d, was_off, sun) = (st.def.clone(), st.was_off, st.sun);
    let kind = g.weather.kind.is_some();
    if !kind && crate::weather::weather_enabled(g) {
        if was_off {
            crate::weather::weather_set_enabled_2(g, false);
        } else if g.m.hours.hour.dry {
            if hour_row(g).storm.is_none() {
                let storm = crate::weather::weather_snapshot(g);
                hour_row(g).storm = Some(storm);
            }
            crate::weather::weather_set_enabled_2(g, false);
        }
    }
    let c = if kind { crate::weather::weather_cloud(g) } else { 0.0 };
    let [sun_k, sky_k] = cloud_light(&d, c);
    let soft = m::max(0.0, cloud_soft(c) - cloud_soft(d.cloud.unwrap_or(0.0)));
    let st = hour_row(g);
    if !st.given { st.sun_k = sun_k; }
    let power = d.sun.intensity * st.sun_k;
    g.weather.fog_glow = if sky_k > 1e-6 { cloud_total(&d, c) / sky_k } else { 1.0 };
    set_light_power(g, sun, power);
    let st = hour_row(g);
    if soft == st.soft && sky_k == st.sky_k { return; }
    st.soft = soft;
    st.sky_k = sky_k;
    g.tab.light[sun].color = greyed(d.sun.color, soft, None);
    g.tab.light[sun].volumetric = d.sun.volumetric * (1.0 - soft);
    let r = d.sun.radius + (m::max(d.sun.radius, SOFT_RADIUS) - d.sun.radius) * soft;
    if r != g.tab.light[sun].radius { g.tab.light[sun].radius = r; }   // (sunReach, the picture's mask, dropped)
    g.engine.settings.sky_zenith = greyed(d.sky.zenith, soft, Some(sky_k));
    g.engine.settings.sky_horizon = greyed(d.sky.horizon, soft, Some(sky_k));
}

/// The weather under a running game changed: the hour's storm switch, sun and sky follow; nothing at night. (hours.ts hourWeatherChanged)
pub fn hour_weather_changed(g: &mut crate::game::Game) {
    if g.m.hours.states.is_none() { return; }
    under_weather(g);
    crate::host_api::host_api_invalidate_world_cache(g);
}

/// A sun strength set by hand handed back to the weather's cloud. (hours.ts sunToCloud)
pub fn sun_to_cloud(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_mut() else { return };
    st.given = false;
    under_weather(g);
}

/// (hours.ts lum)
pub fn lum(c_2: V3) -> f64 {
    0.2126 * c_2[0] + 0.7152 * c_2[1] + 0.0722 * c_2[2]
}

/// Where the hour's sun stands, as a unit vector. (hours.ts toSunOf)
pub fn to_sun_of(d: &crate::hours::HourDef) -> V3 {
    let e = d.sun.elev_deg * DEG;
    let b = d.sun.bearing_deg * DEG;
    v3::normalize([m::sin(b) * m::cos(e), m::sin(e), -m::cos(b) * m::cos(e)])
}

/// Lay hour `id` over the game, or take it off (Night). Idempotent. True when something changed. (hours.ts applyHour)
pub fn apply_hour(g: &mut crate::game::Game, id_2: crate::hours::HourId) -> bool {
    if hour_of(g) == id_2 { return false; }
    if crate::lighting::is_daylight(g) { return false; }
    let cur = g.m.hours.states.clone();   // putIn reads the old row after takeOut has deleted it
    if cur.is_some() { take_out(g); }
    let table = hours();
    if let Some(def) = table.get(String::from(id_2.as_str())) { put_in(g, def, cur.as_ref()); }
    crate::host_api::host_api_invalidate_world_cache(g);
    true
}

/// The moon out, the hour's sun in at the moon's place, the sky, haze and exposure, the weather over it, the photocell. (hours.ts putIn)
pub fn put_in(g: &mut crate::game::Game, def: &crate::hours::HourDef, from: Option<&crate::hours::HourState>) {
    let to_sun = to_sun_of(def);
    let kind = g.weather.kind.is_some();
    let hour = g.m.hours.hour;
    let sun_k = match from { Some(f) => f.sun_k, None => v3::clamp(hour.sun, 0.0, 1.0) };
    let given = match from { Some(f) => f.given, None => hour.sun_given || hour.sun != 1.0 };
    let was_off = if kind { match from { Some(f) => f.was_off, None => false } } else { !crate::weather::weather_enabled(g) };
    let sky = HourStateSky { zenith: v3::copy(g.engine.settings.sky_zenith), horizon: v3::copy(g.engine.settings.sky_horizon) };
    let haze = g.engine.settings.haze_density;
    let exposure = g.engine.settings.exposure;
    let storm = if kind { from.and_then(|f| f.storm.clone()) } else { None };
    let casters = casters_of(g);
    let box_ = box_of(g);
    // the moon out (every steady directional light of the level), the sun in where the moon stood
    let mut moons: Vec<LightId> = Vec::new();
    let mut at = 0usize;
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        let row = &mut g.tab.light[l];
        if row.kind == 2.0 && row.ttl < 0.0 && !row.transient.unwrap_or(false) && row.enabled {
            row.enabled = false;
            moons.push(l);
            at = i + 1;
        }
        i += 1;
    }
    let power = def.sun.intensity * sun_k;
    let sun = g.tab.light.push(crate::render_lights::RtLight {
        name: String::from(SUN_NAME), group: String::from(SUN_GROUP), kind: 2.0, pos: [0.0, 0.0, 0.0], dir: v3::scale(to_sun, -1.0), color: v3::copy(def.sun.color),
        intensity: power, range: 1e6, inner_deg: 0.0, outer_deg: 0.0, radius: def.sun.radius, volumetric: def.sun.volumetric, owner: 0.0, enabled: true,
        fixture_box: -1.0, ttl: -1.0, age: 0.0, peak_intensity: power,
        ..crate::render_lights::RtLight::zeroed()   // every optional field absent, as in the literal
    });
    g.engine.lights.lights.insert(at, sun);
    g.engine.settings.sky_zenith = v3::copy(def.sky.zenith);
    g.engine.settings.sky_horizon = v3::copy(def.sky.horizon);
    g.engine.settings.haze_density = def.haze;
    g.engine.settings.exposure = def.exposure;
    g.m.hours.states = Some(HourState {
        sun, def: def.clone(), sun_k, given, soft: 0.0, sky_k: 1.0, was_off, moons, lamps: JsMap::new(), sky, haze, exposure, storm,
        to_sun, casters, box_, cells: JsMap::new(), fine: JsMap::new(), dome: None, at: None,
    });
    under_weather(g);   // (sunReach, the picture's mask, dropped)
    lamps_out(g);
}

/// Everything the hour laid over the level put back. (hours.ts takeOut)
pub fn take_out(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_ref() else { return };
    let lamps: Vec<(FixtureId, bool)> = st.lamps.iter().map(|(t, was)| (t, *was)).collect();
    let moons = st.moons.clone();
    let (sun, sky, haze, exposure, def_exposure) = (st.sun, st.sky, st.haze, st.exposure, st.def.exposure);
    let on = crate::sim_lightshome::light_in().on;
    for (t, was) in lamps {
        if !crate::sim_lightshome::fixture_on(g, t) { crate::sim_lightshome::light_home_in(g, on, Some(t), Some(if was { 1.0 } else { 0.0 }), None, None, None); }
    }
    for l in moons { g.tab.light[l].enabled = true; }
    let mut set = core::mem::take(&mut g.engine.lights);   // (the set is lent while g is: remove touches only the set's list)
    crate::render_lights::light_set_remove(g, &mut set, sun);
    g.engine.lights = set;
    g.engine.settings.sky_zenith = v3::copy(sky.zenith);
    g.engine.settings.sky_horizon = v3::copy(sky.horizon);
    g.engine.settings.haze_density = haze;
    g.engine.settings.exposure = exposure * (g.engine.settings.exposure / def_exposure);
    let storm = g.m.hours.states.take().and_then(|st| st.storm);
    g.weather.fog_glow = 1.0;
    if let Some(storm) = &storm {
        if !crate::weather::weather_enabled(g) {
            crate::weather::weather_set_enabled_2(g, true);
            crate::weather::weather_restore(g, storm);
        }
    }
}

/// PICTURE ONLY, not translated: a wide sun's reach mask, read by the renderer's candidates and game's reach passes. Its calls (hours.ts:231, 268) are dropped. (hours.ts sunReach)
pub fn sun_reach(g: &mut crate::game::Game, sun: LightId, radius: f64) {
    todo!("picture only: RtLight.reachDir, written and read by the renderer's side alone (hours.ts:284)")
}

/// The photocell: with the lamps out for the hour, every outside fixture's switch recorded as it stands and opened. (hours.ts lampsOut)
pub fn lamps_out(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_ref() else { return };
    if st.def.lamps { return; }
    let off = crate::sim_lightshome::light_in().on;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        let row = &g.tab.sim_lightshome_fixture[t];
        if row.group == "exterior" && row.kind != crate::sim_lightshome::FixtureKind::Breaker {
            let on = crate::sim_lightshome::fixture_on(g, t);
            hour_row(g).lamps.set(t, on);
            crate::sim_lightshome::light_home_in(g, off, Some(t), Some(0.0), None, None, None);
        }
    }
}

/// The level's switches are back as authored: the hour's are made again over them and the sky's reach asked afresh. (hours.ts hourAfterReset)
pub fn hour_after_reset(g: &mut crate::game::Game) {
    g.m.hours.nights = None;
    let Some(st) = g.m.hours.states.as_mut() else { return };
    st.lamps.clear();
    lamps_out(g);
    let casters = casters_of(g);
    let st = hour_row(g);
    st.cells.clear();
    st.fine.clear();
    st.casters = casters;
}

/// The statics changed under a running game: what the sky reaches is asked afresh. (hours.ts hourWorldChanged)
pub fn hour_world_changed(g: &mut crate::game::Game) {
    g.m.hours.nights = None;
    if g.m.hours.states.is_none() { return; }
    let box_ = box_of(g);
    let casters = casters_of(g);
    let st = hour_row(g);
    st.cells.clear();
    st.fine.clear();
    st.box_ = box_;
    st.casters = casters;
}

/// Which statics cast shadows right now, folded to one number. (hours.ts castersOf)
pub fn casters_of(g: &mut crate::game::Game) -> f64 {
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    let n = g.engine.world.statics.len();
    let mut h = n as f64 * 7919.0;
    let mut i = 0;
    while i < n {
        let s = g.engine.world.statics[i];
        if js::to_i32(g.col.boxes[s].flags) & no_shadow != 0 { h = js::to_i32(h + (i as f64 + 1.0) * 31.0) as f64; }
        i += 1;
    }
    h
}

// ---------------------------------------------------------------- the meter's stand-in

/// The ambient's sample directions: SKY (the dome in three bands) and GND (eight just under the horizon). The TypeScript fills them once on first asking; nothing in Rust holds them, so they are worked out where they are read. (hours.ts rays, SKY, GND)
fn ray_tables() -> (Vec<Ray>, Vec<Ray>) {
    fn add(to: &mut Vec<Ray>, n6: &[V3], elev: f64, n: usize, w: f64, turn: f64) {
        for i in 0..n {
            let a = (i as f64 + turn) / n as f64 * 2.0 * m::PI;
            let d: V3 = [m::cos(a) * m::cos(elev), m::sin(elev), m::sin(a) * m::cos(elev)];
            let mut c6 = Vec::with_capacity(6);
            for nn in n6.iter() { c6.push(m::max(0.0, v3::dot(*nn, d)) * w); }
            to.push(Ray { d, c6 });
        }
    }
    fn s(deg: f64) -> f64 { m::sin(deg * DEG) }
    let n6 = n6();
    let mut sky = Vec::with_capacity(15);
    let mut gnd = Vec::with_capacity(8);
    add(&mut sky, &n6, 90.0 * DEG, 1, 2.0 * m::PI * (1.0 - s(70.0)), 0.0);
    add(&mut sky, &n6, 50.0 * DEG, 6, 2.0 * m::PI * (s(70.0) - s(32.0)) / 6.0, 0.0);
    add(&mut sky, &n6, 16.0 * DEG, 8, 2.0 * m::PI * s(32.0) / 8.0, 0.5);
    add(&mut gnd, &n6, GND_ELEV, 8, 2.0 * m::PI * s(25.0) / 8.0, 0.25);
    (sky, gnd)
}

/// The TypeScript's lazy fill of SKY and GND: nothing to do, `ray_tables` answers. (hours.ts rays)
pub fn rays() {}

/// The sky's luminance at height y, off the live sky or the dome given. (hours.ts skyLum)
pub fn sky_lum(g: &mut crate::game::Game, y: f64, sky: Option<crate::hours::SkyStateDome>) -> f64 {
    let (zen, hor) = match sky {
        Some(d) => (lum(d.zenith), lum(d.horizon)),
        None => (lum(g.engine.settings.sky_zenith), lum(g.engine.settings.sky_horizon)),
    };
    let t = v3::clamp(y * 2.5, 0.0, 1.0);
    let below = v3::clamp(-y * 8.0, 0.0, 1.0);
    (hor + (zen - hor) * t) * (1.0 - below) + hor * 0.15 * below
}

/// The box the shadow casters stand in. (hours.ts boxOf)
pub fn box_of(g: &mut crate::game::Game) -> crate::hours::SkyStateBox {
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    let mut b = SkyStateBox { x0: m::INF, x1: -m::INF, z0: m::INF, z1: -m::INF, top: 0.0 };
    let mut i = 0;
    while i < g.engine.world.statics.len() {
        let s = g.engine.world.statics[i];
        i += 1;
        if js::to_i32(g.col.boxes[s].flags) & no_shadow != 0 { continue; }
        let (c, h) = (g.col.boxes[s].c, g.col.boxes[s].h);
        let r = m::hyp2(h[0], h[2]);
        b.x0 = m::min(b.x0, c[0] - r);
        b.x1 = m::max(b.x1, c[0] + r);
        b.z0 = m::min(b.z0, c[2] - r);
        b.z1 = m::max(b.z1, c[2] + r);
        b.top = m::max(b.top, c[1] + h[1]);
    }
    if !(b.x1 > b.x0) { return SkyStateBox { x0: 0.0, x1: 40.0, z0: 0.0, z1: 28.0, top: 8.0 }; }
    b.top += 0.1;
    b
}

/// How far a ray from P along d runs before it is out of the casters' box (≤ 60 m). (hours.ts reachOf)
pub fn reach_of(b: crate::hours::SkyStateBox, p: V3, d: V3) -> f64 {
    let mut t = 60.0;
    if d[1] > 1e-4 { t = m::min(t, (b.top - p[1]) / d[1]); }
    if d[0] > 1e-4 { t = m::min(t, (b.x1 - p[0]) / d[0]); } else if d[0] < -1e-4 { t = m::min(t, (b.x0 - p[0]) / d[0]); }
    if d[2] > 1e-4 { t = m::min(t, (b.z1 - p[2]) / d[2]); } else if d[2] < -1e-4 { t = m::min(t, (b.z0 - p[2]) / d[2]); }
    m::max(t, 0.0)
}

/// The statics alone between a and b: the door leaves are lent away for the question. (hours.ts staticBlocked)
pub fn static_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    let dynamic = core::mem::take(&mut g.col.dynamic_boxes);
    let blocked = crate::scene_collision::segment_blocked(g, a, b, None, None, None);
    g.col.dynamic_boxes = dynamic;
    blocked
}

/// What `skyLight` and `openRays` read of the row they are asked by. The TypeScript hands that row in; the signatures dropped it, and which it is follows from the game: hourLight runs only with an hour's row and moonLight only without one, so an hour's row if there is one, else the night's.
#[derive(Clone, Copy)]
struct SkyAsk { sun: Option<LightId>, to_sun: V3, box_: SkyStateBox, dome: Option<SkyStateDome> }

fn sky_ask(g: &Game) -> Option<SkyAsk> {
    if let Some(st) = &g.m.hours.states {
        return Some(SkyAsk { sun: Some(st.sun), to_sun: st.to_sun, box_: st.box_, dome: st.dome });
    }
    g.m.hours.nights.as_ref().map(|n| SkyAsk { sun: n.sun, to_sun: n.to_sun, box_: n.box_, dome: n.dome })
}

/// That same row's kept rays: its half-metre cells, or (`fine`) its quarter-metre ones.
fn sky_cells(g: &mut Game, fine: bool) -> Option<&mut JsMap<crate::Num, Vec<Open>>> {
    if let Some(st) = g.m.hours.states.as_mut() {
        return Some(if fine { &mut st.fine } else { &mut st.cells });
    }
    g.m.hours.nights.as_mut().map(|n| if fine { &mut n.fine } else { &mut n.cells })
}

/// The rays kept for a cell, asked once from `at`. A copy: the list is read across calls that take `g`, and nothing writes a kept list once it is set.
fn kept_rays(g: &mut Game, fine: bool, key: crate::Num, at: V3) -> Vec<Open> {
    if let Some(o) = sky_cells(g, fine).and_then(|cells| cells.get(key)) { return o.clone(); }
    let o = open_rays(g, at);
    if let Some(cells) = sky_cells(g, fine) { cells.set(key, o.clone()); }
    o
}

/// The ambient's rays that leave point C unblocked by the statics. (hours.ts openRays)
pub fn open_rays(g: &mut crate::game::Game, c_2: V3) -> Vec<crate::hours::Open> {
    let Some(st) = sky_ask(g) else { return Vec::new() };
    let (sky, gnd) = ray_tables();
    let mut out: Vec<Open> = Vec::new();
    let b = st.box_;
    for (i, r) in sky.iter().enumerate() {
        let len = reach_of(b, c_2, r.d);
        if len > 0.05 && !static_blocked(g, c_2, v3::mad(c_2, r.d, len)) { out.push(Open { ray: i as f64, len, sun_k: 0.0, sky_k: 0.0 }); }
    }
    for (i, r) in gnd.iter().enumerate() {
        let len = m::min((c_2[1] - 0.03) / -r.d[1], reach_of(b, c_2, r.d));
        if len < 0.3 { continue; }
        let g_pt = v3::mad(c_2, r.d, len);
        if g_pt[1] > 0.2 || static_blocked(g, c_2, g_pt) { continue; }
        let sun_k = if st.to_sun[1] > 0.0 && !static_blocked(g, g_pt, v3::mad(g_pt, st.to_sun, reach_of(b, g_pt, st.to_sun))) { st.to_sun[1] } else { 0.0 };
        let sky_k = if static_blocked(g, g_pt, [g_pt[0], b.top, g_pt[2]]) { 0.0 } else { 1.0 };
        if sun_k > 0.0 || sky_k > 0.0 { out.push(Open { ray: (sky.len() + i) as f64, len, sun_k, sky_k }); }
    }
    out
}

/// The hour's light at world point P, added to the meter stand-in's six faces. Nothing at night. (hours.ts hourLight)
pub fn hour_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) {
    let Some(casters) = g.m.hours.states.as_ref().map(|st| st.casters) else { return };
    if casters_of(g) != casters { hour_world_changed(g); }
    sky_light(g, p, acc);
}

/// The night's own light from the sky at P, for the level with no hour laid over it. (hours.ts moonLight)
pub fn moon_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) {
    if g.m.hours.states.is_some() { return; }
    let moon = sky_lamp_of(g);
    let day = if crate::lighting::is_daylight(g) { g.level.daylight.as_ref().map(|d| d.sky) } else { None };
    let sky = match day {
        Some(s) => SkyStateDome { zenith: s.zenith, horizon: s.horizon },
        None => SkyStateDome { zenith: g.level.sky.zenith, horizon: g.level.sky.horizon },
    };
    let moon_dir = moon.map(|l| g.tab.light[l].dir);
    let same = match &g.m.hours.nights {
        None => false,
        Some(n) => n.sun == moon
            && dome_same(n.dome, sky)
            && match moon_dir {
                None => true,
                Some(d) => n.to_sun[0] == -d[0] && n.to_sun[1] == -d[1] && n.to_sun[2] == -d[2],
            },
    };
    let time = g.time;
    let stale = match &g.m.hours.nights { None => true, Some(n) => !same || n.at != Some(time) };
    if stale {
        let c = casters_of(g);
        let keep = same && match &g.m.hours.nights { Some(n) => n.casters == c, None => false };
        if keep {
            if let Some(n) = g.m.hours.nights.as_mut() { n.at = Some(time); }
        } else {
            let to_sun = match moon_dir { Some(d) => [-d[0], -d[1], -d[2]], None => [0.0, 1.0, 0.0] };
            let box_ = box_of(g);
            g.m.hours.nights = Some(SkyState { sun: moon, to_sun, casters: c, box_, cells: JsMap::new(), fine: JsMap::new(), dome: Some(sky), at: Some(time) });
        }
    }
    sky_light(g, p, acc);
}

/// `N.dome === sky` of the TypeScript: the Rust dome is a copy, so its values stand for the object. (hours.ts moonLight)
fn dome_same(a: Option<SkyStateDome>, b: SkyStateDome) -> bool {
    match a { Some(a) => a.zenith == b.zenith && a.horizon == b.horizon, None => false }
}

/// What `moonLight` keeps of this game is dropped. (hours.ts moonForget)
pub fn moon_forget(g: &mut crate::game::Game) {
    g.m.hours.nights = None;
}

/// The light from the sky at P by the row in force (`SkyAsk`): its one directional light where P has a line to it, and the ambient of P's cell. (hours.ts skyLight)
pub fn sky_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) {
    let Some(st) = sky_ask(g) else { return };
    let i_sun = match st.sun {
        Some(sun) if g.tab.light[sun].enabled && g.tab.light[sun].intensity > 0.0 => lum(g.tab.light[sun].color) * g.tab.light[sun].intensity,
        _ => 0.0,
    };
    if i_sun > 0.002 {
        let t = reach_of(st.box_, p, st.to_sun);
        if t <= 0.05 || !crate::scene_collision::segment_blocked(g, p, v3::mad(p, st.to_sun, t), None, None, None) {
            let n6 = n6();
            for k in 0..6 {
                let cs = v3::dot(n6[k], st.to_sun);
                if cs > 0.0 { acc[k] += i_sun * cs; }
            }
        }
    }
    // the cell's rays stand for P while nothing is between P and the point they were asked from: else the quarter-cell's, else P's own, unkept
    let ix = m::floor(p[0] / CELL);
    let iz = m::floor(p[2] / CELL);
    let c: V3 = [(ix + 0.5) * CELL, CELL_Y, (iz + 0.5) * CELL];
    let mut from = c;
    let open: Vec<Open> = if !crate::scene_collision::segment_blocked(g, p, c, None, None, None) {
        kept_rays(g, false, crate::Num((iz + 2048.0) * 4096.0 + (ix + 2048.0)), c)
    } else {
        let jx = m::floor(p[0] / (CELL / 2.0));
        let jz = m::floor(p[2] / (CELL / 2.0));
        let c2: V3 = [(jx + 0.5) * CELL / 2.0, CELL_Y, (jz + 0.5) * CELL / 2.0];
        if !crate::scene_collision::segment_blocked(g, p, c2, None, None, None) {
            from = c2;
            kept_rays(g, true, crate::Num((jz + 4096.0) * 8192.0 + (jx + 4096.0)), c2)
        } else {
            from = p;
            open_rays(g, p)
        }
    };
    if open.is_empty() { return; }
    let mut leaves: Option<Vec<BoxId>> = None;
    let mut i = 0;
    while i < g.col.dynamic_boxes.len() {
        let db = g.col.dynamic_boxes[i];
        i += 1;
        let dc = g.col.boxes[db].c;
        if m::hyp2(dc[0] - p[0], dc[2] - p[2]) < DOOR_M { leaves.get_or_insert_with(Vec::new).push(db); }
    }
    let (sky_rays, gnd_rays) = ray_tables();
    let dome = st.dome;
    let mut e_up = -1.0;   // the sky's irradiance on open ground: asked only when a ground ray is open
    for o in open.iter() {
        let ray = o.ray as usize;
        let sky = ray < sky_rays.len();
        let r = if sky { &sky_rays[ray] } else { &gnd_rays[ray - sky_rays.len()] };
        if let Some(leaves) = &leaves {
            let mut hit = false;
            for &db in leaves.iter() {
                if let Some(h) = crate::scene_boxes::ray_box(g, from, r.d, db, o.len) {
                    if h.t >= 0.0 { hit = true; break; }
                }
            }
            if hit { continue; }
        }
        if !sky && e_up < 0.0 {
            e_up = 0.0;
            for q in sky_rays.iter() { e_up += sky_lum(g, q.d[1], dome) * q.c6[2]; }
        }
        let lr = if sky { sky_lum(g, r.d[1], dome) } else { GROUND_ALBEDO / m::PI * (i_sun * o.sun_k + e_up * o.sky_k) };
        if lr <= 0.0 { continue; }
        for k in 0..6 { acc[k] += lr * r.c6[k]; }
    }
}

/// The hour's light alone at P, as the meter would report it. (hours.ts hourLightAt)
pub fn hour_light_at(g: &mut crate::game::Game, p: V3) -> f64 {
    let mut acc = vec![0.0; 6];
    hour_light(g, p, &mut acc);
    m::max_n(&acc)
}

/// `x.toFixed(2)` for what the summary prints, none of it negative: a tie goes up, where Rust's `{:.2}` sends it to the even side. Only a multiple of 1/8 can be a tie.
fn to_fixed_2(x: f64) -> String {
    let e = x * 8.0;
    if m::floor(e) == e && js::rem(e, 2.0) == 1.0 { return alloc::format!("{:.2}", x + 0.001); }
    alloc::format!("{:.2}", x)
}

/// One line for the panel. (hours.ts hourSummary)
pub fn hour_summary(g: &mut crate::game::Game) -> String {
    let Some(st) = g.m.hours.states.as_ref() else { return String::from("night (the level as authored)") };
    let d = st.def.clone();
    let (sun, sun_k, given, sky_k, out, storm, cells) = (st.sun, st.sun_k, st.given, st.sky_k, st.lamps.len(), st.storm.is_some(), st.cells.len());
    let intensity = g.tab.light[sun].intensity;
    let why = if given {
        String::from(" set")
    } else {
        match g.weather.kind { Some(k) => alloc::format!(" under {}'s cloud", k.as_str()), None => String::new() }
    };
    let sky = if sky_k != 1.0 { alloc::format!(", sky ×{}", to_fixed_2(sky_k)) } else { String::new() };
    let lamps = if d.lamps { String::from("as authored") } else { alloc::format!("out ({})", out) };
    alloc::format!(
        "{} {} · sun {}° up at {} (×{}{}{}) · exposure {} · outside lamps {}{} · {} cells",
        d.label, d.clock, d.sun.elev_deg, to_fixed_2(intensity), to_fixed_2(sun_k), why, sky,
        to_fixed_2(g.engine.settings.exposure), lamps, if storm { " · storm held" } else { "" }, cells,
    )
}

/// `t.intensity = t.peakIntensity = x` of a light (hours.ts:171, 227)
fn set_light_power(g: &mut Game, l: LightId, x: f64) {
    let row = &mut g.tab.light[l];
    row.intensity = x;
    row.peak_intensity = x;
}

// ---------------------------------------------------------------- seams: what the skeleton still lacks

fn seam_query_get(q: &crate::Opaque, key: &str) -> Option<String> { todo!("needs: URLSearchParams.get: q is Opaque (hours.ts:94, 102)") }
fn seam_query_has(q: &crate::Opaque, key: &str) -> bool { todo!("needs: URLSearchParams.has: q is Opaque (hours.ts:96, 101)") }
/// `parseFloat(q.get(key))` where the key is there and the number finite (hours.ts:95, 103).
fn seam_query_number(q: &crate::Opaque, key: &str) -> Option<f64> { todo!("needs: URLSearchParams.get: q is Opaque (hours.ts:95, 103)") }
fn seam_visit_hour() -> HourId { todo!("needs: `g` in hourHref, whose default is HOUR.id (hours.ts:106)") }

#[cfg(test)]
mod tests {
    use super::*;
    // the TypeScript's own bits: bun, with EXACT.on, off src/game/hours.ts and its ray code
    const SUN_SOFT: &[(u64, u64, u64)] = &[
    (0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000),
    (0x3fa999999999999a, 0x3fefffc4ae7f7f16, 0x0000000000000000),
    (0x3fd3333333333333, 0x3fef9983a60d49ff, 0x0000000000000000),
    (0x3fe0000000000000, 0x3fedb9f6ffa73fa7, 0x0000000000000000),
    (0x3fe3d70a3d70a3d7, 0x3feb46905769df05, 0x3fc294573a797892),
    (0x3fe8000000000000, 0x3fe6f9bf4972a9c6, 0x3fe0000000000000),
    (0x3feccccccccccccd, 0x3fde73baa680cf86, 0x3fecac083126e97a),
    (0x3fee666666666666, 0x3fd7ae8840612028, 0x3fef1a9fbe76c8b4),
    (0x3ff0000000000000, 0x3fd0000000000000, 0x3ff0000000000000),
    ];
    const LIGHT: &[(HourId, u64, u64, u64, u64)] = &[
    (HourId::Dawn, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Dawn, 0x3fa999999999999a, 0x3fefeb49f32e89a6, 0x3ff008a0ecf8682a, 0x3fefffc4ae7f7f16),
    (HourId::Dawn, 0x3fd3333333333333, 0x3fecc1753d81da61, 0x3ff103a0ec557d64, 0x3fef9983a60d49ff),
    (HourId::Dawn, 0x3fe0000000000000, 0x3fe64b793fbd6fbd, 0x3ff20952e5a048c6, 0x3fedb9f6ffa73fa7),
    (HourId::Dawn, 0x3fe3d70a3d70a3d7, 0x3fe0ca775f2e6f62, 0x3ff21d5c2b33e026, 0x3feb46905769df05),
    (HourId::Dawn, 0x3fe8000000000000, 0x3fd41a876044548d, 0x3ff1018e9100f157, 0x3fe6f9bf4972a9c6),
    (HourId::Dawn, 0x3feccccccccccccd, 0x3fb724c10de7041c, 0x3fe9c21964f4dee7, 0x3fde73baa680cf86),
    (HourId::Dawn, 0x3fee666666666666, 0x3fa278c66fa7eb02, 0x3fe4f7813609ef4c, 0x3fd7ae8840612028),
    (HourId::Dawn, 0x3ff0000000000000, 0x0000000000000000, 0x3fdda9e3448f6454, 0x3fd0000000000000),
    (HourId::Midday, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Midday, 0x3fa999999999999a, 0x3fefeb49f32e89a6, 0x3ff0200e91d0fa5a, 0x3fefffc4ae7f7f16),
    (HourId::Midday, 0x3fd3333333333333, 0x3fecc1753d81da61, 0x3ff44484ef919d49, 0x3fef9983a60d49ff),
    (HourId::Midday, 0x3fe0000000000000, 0x3fe64b793fbd6fbd, 0x3ffa89c242c336ac, 0x3fedb9f6ffa73fa7),
    (HourId::Midday, 0x3fe3d70a3d70a3d7, 0x3fe0ca775f2e6f62, 0x3ffe1bf25170d850, 0x3feb46905769df05),
    (HourId::Midday, 0x3fe8000000000000, 0x3fd41a876044548d, 0x3fffca6f6de8ecda, 0x3fe6f9bf4972a9c6),
    (HourId::Midday, 0x3feccccccccccccd, 0x3fb724c10de7041c, 0x3ffafcf128f59ef5, 0x3fde73baa680cf86),
    (HourId::Midday, 0x3fee666666666666, 0x3fa278c66fa7eb02, 0x3ff6b56454601151, 0x3fd7ae8840612028),
    (HourId::Midday, 0x3ff0000000000000, 0x0000000000000000, 0x3ff0915e2cc30a04, 0x3fd0000000000000),
    (HourId::Overcast, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3fa999999999999a, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3fd3333333333333, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3fe0000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3fe3d70a3d70a3d7, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3fe8000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3feccccccccccccd, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Overcast, 0x3fee666666666666, 0x3fd98a5a06be4ab1, 0x3fed84146de4f652, 0x3fe8e2b39d8b76b4),
    (HourId::Overcast, 0x3ff0000000000000, 0x0000000000000000, 0x3fe73d3d83c21c78, 0x3fe0d034ffd62914),
    (HourId::Dusk, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3ff0000000000000),
    (HourId::Dusk, 0x3fa999999999999a, 0x3fefeb49f32e89a6, 0x3ff00761752bf96a, 0x3fefffc4ae7f7f16),
    (HourId::Dusk, 0x3fd3333333333333, 0x3fecc1753d81da61, 0x3ff0d743a2dfdece, 0x3fef9983a60d49ff),
    (HourId::Dusk, 0x3fe0000000000000, 0x3fe64b793fbd6fbd, 0x3ff195650e5544da, 0x3fedb9f6ffa73fa7),
    (HourId::Dusk, 0x3fe3d70a3d70a3d7, 0x3fe0ca775f2e6f62, 0x3ff179cdb05a30f6, 0x3feb46905769df05),
    (HourId::Dusk, 0x3fe8000000000000, 0x3fd41a876044548d, 0x3ff037f402fe4e1a, 0x3fe6f9bf4972a9c6),
    (HourId::Dusk, 0x3feccccccccccccd, 0x3fb724c10de7041c, 0x3fe84151f400713e, 0x3fde73baa680cf86),
    (HourId::Dusk, 0x3fee666666666666, 0x3fa278c66fa7eb02, 0x3fe3aa19c2370136, 0x3fd7ae8840612028),
    (HourId::Dusk, 0x3ff0000000000000, 0x0000000000000000, 0x3fdbb6b477ccad35, 0x3fd0000000000000),
    ];
    const GREYED: &[(HourId, [u64; 3], [u64; 3])] = &[
    (HourId::Dawn, [0x3ff21d5b51a05096, 0x3fead1ebc178e322, 0x3fe5706ef9a46e20], [0x3fc98046f5dae7c6, 0x3fcc254855a9dbb6, 0x3fd0ffa513459fd5]),
    (HourId::Midday, [0x3ff439a04d6dea36, 0x3ff438fbbd981e22, 0x3ff3f3899ab2baf0], [0x3fd98b30ebfc01f2, 0x3fdc919957040619, 0x3fe167d104546098]),
    (HourId::Overcast, [0x3ff3000a34c38792, 0x3ff3c631bb447a96, 0x3ff50f338e44e9ba], [0x3fe198cedfbe787a, 0x3fe28fa503659dfe, 0x3fe428790914f59b]),
    (HourId::Dusk, [0x3ff16c050225d92c, 0x3fe636aea1829509, 0x3fe0b50506ab5937], [0x3fc52c635eaab45b, 0x3fc7609bf6a2e47c, 0x3fcd6392f0a5a9da]),
    ];
    const TO_SUN: &[(HourId, [u64; 3])] = &[
    (HourId::Dawn, [0x3feca8ae1a225c4f, 0x3fd7f98deee59681, 0x3fceb76e1597a4ba]),
    (HourId::Midday, [0xbfc28681558cc9ee, 0x3fea8779cda8eea3, 0x3fe148d14e50c192]),
    (HourId::Overcast, [0xbfe1d03e70ea84ff, 0x3fe8836fa2cf503a, 0x3fd491b7523c1623]),
    (HourId::Dusk, [0xbfeca8ae1a225c4f, 0x3fd7f98deee59681, 0x3fceb76e1597a4b2]),
    ];
    const RAYS: &[([u64; 3], [u64; 6])] = &[
    ([0x3c91a62633145c00, 0x3ff0000000000000, 0x0000000000000000], [0x3c7ac01ccbc0a57e, 0x0000000000000000, 0x3fd84043e6cc5edd, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]),
    ([0x3fe491b7523c161d, 0x3fe8836fa2cf5039, 0x0000000000000000], [0x3fd1a72e6e22ca70, 0x0000000000000000, 0x3fd509c029598906, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000]),
    ([0x3fd491b7523c161e, 0x3fe8836fa2cf5039, 0x3fe1d03e70ea84ff], [0x3fc1a72e6e22ca71, 0x0000000000000000, 0x3fd509c029598906, 0x0000000000000000, 0x3fce9373a51b1677, 0x0000000000000000]),
    ([0xbfd491b7523c161a, 0x3fe8836fa2cf5039, 0x3fe1d03e70ea8500], [0x0000000000000000, 0x3fc1a72e6e22ca6e, 0x3fd509c029598906, 0x0000000000000000, 0x3fce9373a51b1679, 0x0000000000000000]),
    ([0xbfe491b7523c161d, 0x3fe8836fa2cf5039, 0x3c96b06bb1130ef4], [0x0000000000000000, 0x3fd1a72e6e22ca70, 0x3fd509c029598906, 0x0000000000000000, 0x3c8378f1fc254018, 0x0000000000000000]),
    ([0xbfd491b7523c1622, 0x3fe8836fa2cf5039, 0xbfe1d03e70ea84fe], [0x0000000000000000, 0x3fc1a72e6e22ca75, 0x3fd509c029598906, 0x0000000000000000, 0x0000000000000000, 0x3fce9373a51b1676]),
    ([0x3fd491b7523c161d, 0x3fe8836fa2cf5039, 0xbfe1d03e70ea84ff], [0x3fc1a72e6e22ca70, 0x0000000000000000, 0x3fd509c029598906, 0x0000000000000000, 0x0000000000000000, 0x3fce9373a51b1677]),
    ([0x3fec6b3bbb99ef55, 0x3fd1a40add328e29, 0x3fd78b00295267ac], [0x3fd7a7de9acae955, 0x0000000000000000, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x3fc398d8bab10a58, 0x0000000000000000]),
    ([0x3fd78b00295267ad, 0x3fd1a40add328e29, 0x3fec6b3bbb99ef55], [0x3fc398d8bab10a59, 0x0000000000000000, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x3fd7a7de9acae955, 0x0000000000000000]),
    ([0xbfd78b00295267ac, 0x3fd1a40add328e29, 0x3fec6b3bbb99ef55], [0x0000000000000000, 0x3fc398d8bab10a58, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x3fd7a7de9acae955, 0x0000000000000000]),
    ([0xbfec6b3bbb99ef55, 0x3fd1a40add328e29, 0x3fd78b00295267ad], [0x0000000000000000, 0x3fd7a7de9acae955, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x3fc398d8bab10a59, 0x0000000000000000]),
    ([0xbfec6b3bbb99ef56, 0x3fd1a40add328e29, 0xbfd78b00295267ab], [0x0000000000000000, 0x3fd7a7de9acae956, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x0000000000000000, 0x3fc398d8bab10a57]),
    ([0xbfd78b00295267b6, 0x3fd1a40add328e29, 0xbfec6b3bbb99ef53], [0x0000000000000000, 0x3fc398d8bab10a60, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x0000000000000000, 0x3fd7a7de9acae954]),
    ([0x3fd78b00295267b0, 0x3fd1a40add328e29, 0xbfec6b3bbb99ef54], [0x3fc398d8bab10a5c, 0x0000000000000000, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x0000000000000000, 0x3fd7a7de9acae954]),
    ([0x3fec6b3bbb99ef53, 0x3fd1a40add328e29, 0xbfd78b00295267b7], [0x3fd7a7de9acae954, 0x0000000000000000, 0x3fbd5e43afcb7af9, 0x0000000000000000, 0x0000000000000000, 0x3fc398d8bab10a61]),
    ([0x3feeb30493f2c897, 0xbfca9cd9ac4258f6, 0x3fc86d05fe248404], [0x3fd4612fe7126b2c, 0x0000000000000000, 0x0000000000000000, 0x3fb1aab111a22052, 0x3fb0370d5aa0911d, 0x0000000000000000]),
    ([0x3fe163c6a9876e5c, 0xbfca9cd9ac4258f6, 0x3fea068dbc247328], [0x3fc71699249c01a5, 0x0000000000000000, 0x0000000000000000, 0x3fb1aab111a22052, 0x3fd146eaecb8b588, 0x0000000000000000]),
    ([0xbfc86d05fe248402, 0xbfca9cd9ac4258f6, 0x3feeb30493f2c897], [0x0000000000000000, 0x3fb0370d5aa0911b, 0x0000000000000000, 0x3fb1aab111a22052, 0x3fd4612fe7126b2c, 0x0000000000000000]),
    ([0xbfea068dbc247329, 0xbfca9cd9ac4258f6, 0x3fe163c6a9876e5b], [0x0000000000000000, 0x3fd146eaecb8b589, 0x0000000000000000, 0x3fb1aab111a22052, 0x3fc71699249c01a4, 0x0000000000000000]),
    ([0xbfeeb30493f2c897, 0xbfca9cd9ac4258f6, 0xbfc86d05fe248408], [0x0000000000000000, 0x3fd4612fe7126b2c, 0x0000000000000000, 0x3fb1aab111a22052, 0x0000000000000000, 0x3fb0370d5aa0911f]),
    ([0xbfe163c6a9876e5b, 0xbfca9cd9ac4258f6, 0xbfea068dbc247329], [0x0000000000000000, 0x3fc71699249c01a4, 0x0000000000000000, 0x3fb1aab111a22052, 0x0000000000000000, 0x3fd146eaecb8b589]),
    ([0x3fc86d05fe248405, 0xbfca9cd9ac4258f6, 0xbfeeb30493f2c897], [0x3fb0370d5aa0911d, 0x0000000000000000, 0x0000000000000000, 0x3fb1aab111a22052, 0x0000000000000000, 0x3fd4612fe7126b2c]),
    ([0x3fea068dbc247329, 0xbfca9cd9ac4258f6, 0xbfe163c6a9876e5b], [0x3fd146eaecb8b589, 0x0000000000000000, 0x0000000000000000, 0x3fb1aab111a22052, 0x0000000000000000, 0x3fc71699249c01a4]),
    ];
    fn b3(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }
    fn def(id: HourId) -> HourDef { hours().get(String::from(id.as_str())).unwrap().clone() }

    #[test]
    fn cloud_sun_and_soft_are_the_typescripts_bits() {
        for &(c, s, f) in SUN_SOFT {
            let c = f64::from_bits(c);
            assert_eq!(cloud_sun(c).to_bits(), s);
            assert_eq!(cloud_soft(c).to_bits(), f);
        }
    }
    #[test]
    fn cloud_light_and_total_are_the_typescripts_bits() {
        for &(id, c, a, b, t) in LIGHT {
            let (d, c) = (def(id), f64::from_bits(c));
            let [x, y] = cloud_light(&d, c);
            assert_eq!((x.to_bits(), y.to_bits()), (a, b));
            assert_eq!(cloud_total(&d, c).to_bits(), t);
        }
    }
    #[test]
    fn greyed_is_the_typescripts_bits() {
        for &(id, a, b) in GREYED {
            let d = def(id);
            assert_eq!(b3(greyed(d.sun.color, 0.37, Some(1.3))), a);
            assert_eq!(b3(greyed(d.sky.zenith, 0.81, None)), b);
        }
    }
    #[test]
    fn to_sun_is_the_typescripts_bits() {
        for &(id, a) in TO_SUN { assert_eq!(b3(to_sun_of(&def(id))), a); }
    }
    #[test]
    fn rays_are_the_typescripts_bits() {
        let (sky, gnd) = ray_tables();
        assert_eq!((sky.len(), gnd.len()), (15, 8));
        for (r, &(d, c)) in sky.iter().chain(gnd.iter()).zip(RAYS.iter()) {
            assert_eq!(b3(r.d), d);
            for k in 0..6 { assert_eq!(r.c6[k].to_bits(), c[k]); }
        }
    }
    #[test]
    fn night_has_no_record() {
        assert!(hours().get(String::from("night")).is_none());
        assert_eq!(hour_label(HourId::Night), "Night");
        assert_eq!(hour_clock(HourId::Dusk), "17:00");
    }
    // the words checked against the TypeScript's own functions under bun
    #[test]
    fn hour_words_are_read_as_the_typescript_reads_them() {
        assert_eq!(hour_id_of(Some("MidDay")), HourId::Midday);
        assert_eq!(hour_id_of(Some("sunset")), HourId::Dusk);
        assert_eq!(hour_id_of(Some("constructor")), HourId::Night);
        assert_eq!(hour_id_of(None), HourId::Night);
    }
    #[test]
    fn hour_href_replaces_the_hour_on_a_link() {
        assert_eq!(hour_href("/play?a=1&hour=dawn&b", Some(HourId::Dusk)), "/play?a=1&b&hour=dusk");
        assert_eq!(hour_href("/play?hour=dawn", Some(HourId::Night)), "/play");
        assert_eq!(hour_href("/play?hours=2&hour", Some(HourId::Night)), "/play?hours=2");
        assert_eq!(hour_href("/play", Some(HourId::Midday)), "/play?hour=midday");
        assert_eq!(hour_href("/p?a?b", Some(HourId::Night)), "/p?a");
    }
    fn lamp(g: &mut Game, kind: f64, dir: V3, intensity: f64) -> LightId {
        let l = g.tab.light.push(crate::render_lights::RtLight { kind, dir, color: [1.0, 1.0, 1.0], intensity, enabled: true, ttl: -1.0, ..crate::render_lights::RtLight::zeroed() });
        g.engine.lights.lights.push(l);
        l
    }
    fn stat(g: &mut Game, c: V3, h: V3, flags: f64) -> BoxId {
        let b = g.col.boxes.push(crate::scene_boxes::Box { c, h, flags, ..Default::default() });
        g.engine.world.statics.push(b);
        b
    }
    #[test]
    fn the_sky_lamp_is_the_brightest_steady_one_pointing_down() {
        let mut g = Game::blank();
        assert_eq!(sky_lamp_of(&mut g), None);
        let dim = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 0.001);       // under the floor of 0.002
        let moon = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 0.5);
        let _spot = lamp(&mut g, 1.0, [0.0, -1.0, 0.0], 9.0);       // not directional
        let _up = lamp(&mut g, 2.0, [0.0, 1.0, 0.0], 9.0);          // from below the horizon
        let flash = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 9.0);
        g.tab.light[flash].ttl = 1.0;
        let held = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 9.0);
        g.tab.light[held].transient = Some(true);
        let out = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 9.0);
        g.tab.light[out].enabled = false;
        assert_eq!(sky_lamp_of(&mut g), Some(moon));
        g.k.sense.moon = true;
        assert!(in_estimate(&mut g, moon));
        assert!(!in_estimate(&mut g, dim));
        g.k.sense.moon = false;
        assert!(!in_estimate(&mut g, moon));
        g.m.hours.states = Some(HourState { sun: dim, ..HourState::zeroed() });   // an hour in: its sun is the estimate's, the night has no lamp
        assert_eq!(sky_lamp_of(&mut g), None);
        assert!(in_estimate(&mut g, dim));
    }
    #[test]
    fn the_sun_strength_lands_in_the_light() {
        let mut g = Game::blank();
        let sun = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 0.0);
        let mut st = HourState { sun, ..HourState::zeroed() };
        st.def.sun.intensity = 6.0;
        g.m.hours.states = Some(st);
        set_sun_strength(&mut g, 0.5, None);
        assert_eq!((g.tab.light[sun].intensity, g.tab.light[sun].peak_intensity), (3.0, 3.0));
        assert_eq!((sun_strength(&mut g), sun_given(&mut g)), (0.5, true));
        set_sun_strength(&mut g, f64::NAN, Some(true));   // not a number: the hour's own
        assert_eq!(g.tab.light[sun].intensity, 6.0);
        set_sun_strength(&mut g, 7.0, Some(true));
        assert_eq!(sun_strength(&mut g), 1.0);
    }
    #[test]
    fn the_casters_fold_and_their_box() {
        let no_shadow = crate::scene_boxes::box_flag().no_shadow;
        let mut g = Game::blank();
        assert_eq!(casters_of(&mut g), 0.0);
        let b = box_of(&mut g);   // nothing casts: the level's stand-in box
        assert_eq!((b.x0, b.x1, b.z0, b.z1, b.top), (0.0, 40.0, 0.0, 28.0, 8.0));
        stat(&mut g, [1.0, 2.0, 3.0], [3.0, 1.0, 4.0], 0.0);
        let hidden = stat(&mut g, [100.0, 50.0, 100.0], [1.0, 1.0, 1.0], no_shadow);
        assert_eq!(casters_of(&mut g), (2 * 7919 + 2 * 31) as f64);   // the second, by its place
        let b = box_of(&mut g);   // a yawed box's circle, radius 5; the hidden one is not in it
        assert_eq!((b.x0, b.x1, b.z0, b.z1, b.top), (-4.0, 6.0, -2.0, 8.0, 3.1));
        g.col.boxes[hidden].flags = 0.0;
        assert_eq!(casters_of(&mut g), (2 * 7919) as f64);
    }
    #[test]
    fn a_ray_runs_until_it_leaves_the_box() {
        let b = SkyStateBox { x0: -10.0, x1: 10.0, z0: -5.0, z1: 5.0, top: 4.0 };
        assert_eq!(reach_of(b, [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]), 4.0);
        assert_eq!(reach_of(b, [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]), 10.0);
        assert_eq!(reach_of(b, [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]), 5.0);
        assert_eq!(reach_of(b, [0.0, 1.0, 0.0], [0.0, -1.0, 0.0]), 60.0);   // down: the box has no floor, the 60 m stands
        assert_eq!(reach_of(b, [20.0, 1.0, 0.0], [1.0, 0.0, 0.0]), 0.0);    // already out
    }
    #[test]
    fn to_fixed_2_is_the_typescripts() {
        for (x, s) in [(0.625, "0.63"), (0.125, "0.13"), (0.375, "0.38"), (1.875, "1.88"), (1.005, "1.00"), (2.6, "2.60"), (0.0, "0.00"), (1.0, "1.00"), (0.25, "0.25"), (6.0, "6.00")] {
            assert_eq!(to_fixed_2(x), s);
        }
    }
}
