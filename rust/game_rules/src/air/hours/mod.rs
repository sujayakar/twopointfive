pub use ::game_types::hours::types;
pub use types::*;
pub mod turns;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec::Vec};

// ---------------------------------------------------------------- the hours' words

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

/// The order of a day, the night last: the order the sheet's row walks. (hours.ts HOUR_IDS)
pub const HOUR_ROW: [HourId; 5] = [HourId::Dawn, HourId::Midday, HourId::Overcast, HourId::Dusk, HourId::Night];

/// The hour's label on the sheet. (hours.ts hourLabel)
pub fn hour_label(id: HourId) -> String { if id == HourId::Night { night().label } else { hours().get(String::from(id.as_str())).map_or(String::new(), |h| h.label.clone()) } }
/// The local time it stands for. (hours.ts hourClock)
fn hour_clock(id: HourId) -> String { if id == HourId::Night { night().clock } else { hours().get(String::from(id.as_str())).map_or(String::new(), |h| h.clock.clone()) } }
/// The sheet's sentence. (hours.ts hourWords)
fn hour_words(id: HourId) -> String { if id == HourId::Night { night().words } else { hours().get(String::from(id.as_str())).map_or(String::new(), |h| h.words.clone()) } }
/// The cloud already in an hour's numbers. (hours.ts HOURS[h].cloud ?? 0)
pub(super) fn hour_cloud(id: HourId) -> f64 { hours().get(String::from(id.as_str())).and_then(|h| h.cloud).unwrap_or(0.0) }

/// A link with the hour on it (night = none): whatever hour it carried is replaced. (hours.ts hourHref)
pub fn hour_href(href: &str, id: HourId) -> String {
    let mut split = href.split('?');
    let path = split.next().unwrap_or("");
    let mut keys: Vec<String> = split.next().unwrap_or("").split('&').filter(|k| !k.is_empty() && *k != "hour" && !k.starts_with("hour=")).map(String::from).collect();
    if id != HourId::Night { keys.push(alloc::format!("hour={}", id.as_str())); }
    if keys.is_empty() { String::from(path) } else { alloc::format!("{}?{}", path, keys.join("&")) }
}

/// The Hour row's line, the weather over it. `own`: the weather is the mission's own, and no Weather row is drawn below to pick it on. (missions.ts hourNote)
pub fn hour_note(h: HourId, wx: bool, own: bool) -> String {
    let what = if h == HourId::Night && wx {
        alloc::format!("the level as it was built, with the moon and the lights outside, in {}", if own { "the mission's own weather" } else { "the weather picked below" })
    } else { hour_words(h) };
    alloc::format!("{} \u{b7} {}", hour_clock(h), what)
}

/// The hour off a query handed to a running game; no `hour` key = nothing touched. (hours.ts hourFromQuery)
pub fn hour_from_query(g: &mut crate::game::Game, q: &crate::query::Query) {
    if !q.has("hour") { return; }
    apply_hour(g, hour_id_of(q.get("hour")));
    if let Some(s) = q.float("sun").filter(|s| s.is_finite()) { set_sun_strength(g, s, None); }
}

// ---------------------------------------------------------------- the hour on a Game

/// The hour's row, for a body that has already seen it there and calls nothing that deletes it.
fn hour_row(g: &mut Game) -> &mut HourState {
    g.m.hours.states.as_mut().unwrap()
}

/// The hour this game stands at; Night = the level as authored. (hours.ts hourOf)
pub fn hour_of(g: &crate::game::Game) -> crate::hours::HourId {
    match &g.m.hours.states { Some(st) => st.def.id, None => HourId::Night }
}

/// The exposure the game's scene is taken at now: the show's day's while it is in, else the hour's own, else the level's night's. Never 0: a sum divides by it. (hours.ts exposureOf)
pub fn exposure_of(g: &crate::game::Game) -> f64 {
    let own = if crate::lighting::is_daylight(g) {
        g.level.daylight.as_ref().map(|d| d.exposure)
    } else {
        g.m.hours.states.as_ref().map(|st| st.def.exposure)
    };
    let e = match own { Some(e) => e, None => g.k.env.exposure };
    if e > 0.0 { e } else { 1.0 }
}

/// The level's brightest steady directional light that is up, when no hour is laid over it. (hours.ts skyLampOf)
fn sky_lamp_of(g: &crate::game::Game) -> Option<LightId> {
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
pub fn sun_strength(g: &crate::game::Game) -> f64 {
    match &g.m.hours.states { Some(st) => st.sun_k, None => 1.0 }
}

/// Was the sun's strength set by hand rather than left to the cloud. (hours.ts sunGiven)
pub(super) fn sun_given(g: &crate::game::Game) -> bool {
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
fn cloud_sun(c_2: f64) -> f64 {
    1.0 - 0.75 * m::pow(v3::clamp(c_2, 0.0, 1.0), 3.4)
}

/// How far cloud cover c has spread the sun and greyed the sky. (hours.ts cloudSoft)
fn cloud_soft(c_2: f64) -> f64 {
    v3::smoothstep(0.5, 1.0, c_2)
}

/// Colour c drained toward a deck's grey by w and scaled by k, its luminance otherwise kept. (hours.ts greyed)
fn greyed(c_2: V3, w: f64, k: Option<f64>) -> V3 {
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
fn sky_share(d: &crate::hours::HourDef) -> f64 {
    let rays = sky();
    let zen = lum(d.sky.zenith);
    let hor = lum(d.sky.horizon);
    let mut k = 0.0;
    for q in rays.iter() { k += (hor + (zen - hor) * v3::clamp(q.d[1] * 2.5, 0.0, 1.0)) * q.c6[2]; }
    let s = lum(d.sun.color) * d.sun.intensity * m::max(0.0, m::sin(d.sun.elev_deg * DEG));
    if k + s > 0.0 { k / (k + s) } else { 1.0 }
}

/// [sun, sky] under cover c, each as a factor of the hour's own. (hours.ts cloudLight)
fn cloud_light(d: &crate::hours::HourDef, c_2: f64) -> [f64; 2] {
    let ch = d.cloud.unwrap_or(0.0);
    let c = m::max(v3::clamp(c_2, 0.0, 1.0), ch);
    if c == ch { return [1.0, 1.0]; }
    let own = sky_share(d);
    let u = (c * c - ch * ch) / (1.0 - ch * ch);
    let f = cloud_sun(c) / cloud_sun(ch);
    [(1.0 - u) * f, if own > 1e-6 { (own + (1.0 - own) * u) / own * f } else { 1.0 }]
}

/// The light on level open ground under cover c as a factor of the hour's own. (hours.ts cloudTotal)
fn cloud_total(d: &crate::hours::HourDef, c_2: f64) -> f64 {
    let ch = d.cloud.unwrap_or(0.0);
    cloud_sun(m::max(v3::clamp(c_2, 0.0, 1.0), ch)) / cloud_sun(ch)
}

/// The weather this game is in laid over the hour's row: the storm's switch, the sun and the sky. (hours.ts underWeather)
fn under_weather(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_ref() else { return };
    let (d, was_off, sun) = (st.def.clone(), st.was_off, st.sun);
    let kind = crate::weather::weather_kind(g).is_some();
    if !kind && crate::weather::weather_enabled(g) {
        if was_off {
            crate::weather::weather_set_enabled_2(g, false);
        } else if g.stood.dry {
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
    if r != g.tab.light[sun].radius {
        g.tab.light[sun].radius = r;
    }
    g.engine.settings.sky_zenith = greyed(d.sky.zenith, soft, Some(sky_k));
    g.engine.settings.sky_horizon = greyed(d.sky.horizon, soft, Some(sky_k));
}

/// The weather under a running game changed: the hour's storm switch, sun and sky follow; nothing at night. (hours.ts hourWeatherChanged)
pub(super) fn hour_weather_changed(g: &mut crate::game::Game) {
    if g.m.hours.states.is_none() { return; }
    under_weather(g);
    crate::host_api::host_api_invalidate_world_cache(g);
}

/// A sun strength set by hand handed back to the weather's cloud. (hours.ts sunToCloud)
fn sun_to_cloud(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_mut() else { return };
    st.given = false;
    under_weather(g);
}

/// (hours.ts lum)
pub fn lum(c_2: V3) -> f64 {
    0.2126 * c_2[0] + 0.7152 * c_2[1] + 0.0722 * c_2[2]
}

/// Where the hour's sun stands, as a unit vector. (hours.ts toSunOf)
fn to_sun_of(d: &crate::hours::HourDef) -> V3 {
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
fn put_in(g: &mut crate::game::Game, def: &crate::hours::HourDef, from: Option<&crate::hours::HourState>) {
    let to_sun = to_sun_of(def);
    let kind = crate::weather::weather_kind(g).is_some();
    let stood = g.stood;
    let sun_k = match from { Some(f) => f.sun_k, None => v3::clamp(stood.sun, 0.0, 1.0) };
    let given = match from { Some(f) => f.given, None => stood.sun_given || stood.sun != 1.0 };
    let was_off = if kind { match from { Some(f) => f.was_off, None => false } } else { !crate::weather::weather_enabled(g) };
    let storm = if kind { from.and_then(|f| f.storm.clone()) } else { None };
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
        fixture_box: -1.0, flicker: 0.0, ttl: -1.0, age: 0.0, peak_intensity: power, broad: false,
        ..crate::render_lights::RtLight::default()   // every optional field absent, as in the literal
    });
    g.engine.lights.lights.insert(at, sun);
    g.engine.settings.sky_zenith = v3::copy(def.sky.zenith);
    g.engine.settings.sky_horizon = v3::copy(def.sky.horizon);
    g.engine.settings.haze_density = def.haze;
    g.engine.settings.exposure = def.exposure;
    g.m.hours.states = Some(HourState {
        sun, def: def.clone(), sun_k, given, soft: 0.0, sky_k: 1.0, was_off, moons, lamps: JsMap::new(), storm,
        to_sun,
    });
    under_weather(g);
    lamps_out(g);
}

/// THE NIGHT'S HAZE, what the picture's settings hold on a level's own night (`hazeDensity` in src/settings.ts: the test below reads the file)
pub const NIGHT_HAZE: f64 = 0.035;

/// The picture's settings as the level's own night has them, once the hour or the show's day taken off was at this exposure. (hours.ts nightPicture)
pub(super) fn night_picture(g: &mut crate::game::Game, was_exposure: f64) {
    g.engine.settings.sky_zenith = v3::copy(g.level.sky.zenith);
    g.engine.settings.sky_horizon = v3::copy(g.level.sky.horizon);
    g.engine.settings.haze_density = NIGHT_HAZE;
    g.engine.settings.exposure = g.k.env.exposure * (g.engine.settings.exposure / was_exposure);
}

/// Everything the hour laid over the level put back. (hours.ts takeOut)
fn take_out(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_ref() else { return };
    let lamps: Vec<(FixtureId, bool)> = st.lamps.iter().map(|(t, was)| (t, *was)).collect();
    let moons = st.moons.clone();
    let (sun, def_exposure) = (st.sun, st.def.exposure);
    let on = crate::sim_lightshome::light_in().on;
    for (t, was) in lamps {
        if !crate::sim_lightshome::fixture_on(g, t) { crate::sim_lightshome::light_home_in(g, on, Some(t), Some(if was { 1.0 } else { 0.0 }), None, None, None); }
    }
    for l in moons { g.tab.light[l].enabled = true; }
    crate::render_lights::light_set_remove(g, sun);
    // the picture told what the night's is: the level's sky, the night's haze and exposure, with a turn of the player's Brightness under the hour carried back. What the settings held before the hour is the picture's, so the game keeps none of it
    night_picture(g, def_exposure);
    let storm = g.m.hours.states.take().and_then(|st| st.storm);
    g.weather.fog_glow = 1.0;
    if let Some(storm) = &storm {
        if !crate::weather::weather_enabled(g) {
            crate::weather::weather_set_enabled_2(g, true);
            crate::weather::weather_restore(g, storm);
        }
    }
}

/// The photocell: with the lamps out for the hour, every outside fixture's switch recorded as it stands and opened. (hours.ts lampsOut)
fn lamps_out(g: &mut crate::game::Game) {
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

/// The level's switches are back as authored: the hour's are made again over them. (hours.ts hourAfterReset)
pub fn hour_after_reset(g: &mut crate::game::Game) {
    let Some(st) = g.m.hours.states.as_mut() else { return };
    st.lamps.clear();
    lamps_out(g);
}

// ---------------------------------------------------------------- the meter's stand-in

/// THE SKY THE GAME'S LIGHT IS WORKED FROM NOW: the hour's as its weather leaves it, else the level's own night. Worked out from state the game keeps, as `exposure_of` is, and never read off the picture's settings, which a panel turns. What `under_weather` tells the picture is this.
fn sky_dome(g: &crate::game::Game) -> crate::hours::SkyStateDome {
    match g.m.hours.states.as_ref() {
        Some(st) => crate::hours::SkyStateDome { zenith: greyed(st.def.sky.zenith, st.soft, Some(st.sky_k)), horizon: greyed(st.def.sky.horizon, st.soft, Some(st.sky_k)) },
        None => crate::hours::SkyStateDome { zenith: g.level.sky.zenith, horizon: g.level.sky.horizon },
    }
}

/// The sky's luminance at height y, off the game's sky now or the dome given.
fn sky_lum(g: &crate::game::Game, y: f64, sky: Option<crate::hours::SkyStateDome>) -> f64 {
    let d = match sky { Some(d) => d, None => sky_dome(g) };
    let (zen, hor) = (lum(d.zenith), lum(d.horizon));
    let t = v3::clamp(y * 2.5, 0.0, 1.0);
    let below = v3::clamp(-y * 8.0, 0.0, 1.0);
    (hor + (zen - hor) * t) * (1.0 - below) + hor * 0.15 * below
}

/// The box the shadow casters stand in. (hours.ts boxOf)
fn box_of(g: &mut crate::game::Game) -> crate::hours::SkyStateBox {
    let no_shadow = js::to_i32(crate::scene_boxes::box_flag().no_shadow);
    let mut b = SkyStateBox { x0: m::INF, x1: -m::INF, z0: m::INF, z1: -m::INF, top: 0.0 };
    let mut i = 0;
    while i < g.engine.world.statics.len() {
        let s = g.engine.world.statics[i];
        i += 1;
        if js::to_i32(seam_box(g, s).flags) & no_shadow != 0 { continue; }
        let (c, h) = { let row = seam_box(g, s); (row.c, row.h) };
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
fn reach_of(b: crate::hours::SkyStateBox, p: V3, d: V3) -> f64 {
    let mut t = 60.0;
    if d[1] > 1e-4 { t = m::min(t, (b.top - p[1]) / d[1]); }
    if d[0] > 1e-4 { t = m::min(t, (b.x1 - p[0]) / d[0]); } else if d[0] < -1e-4 { t = m::min(t, (b.x0 - p[0]) / d[0]); }
    if d[2] > 1e-4 { t = m::min(t, (b.z1 - p[2]) / d[2]); } else if d[2] < -1e-4 { t = m::min(t, (b.z0 - p[2]) / d[2]); }
    m::max(t, 0.0)
}

/// The statics alone between a and b: no door leaf is in the way. (hours.ts staticBlocked)
fn static_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    crate::scene_collision::segment_blocked_over(g, &crate::ask::Over::statics(), a, b, None, None)
}

/// The one directional light of the sky in force and its dome (None: the picture's own), by the hour laid over the level or by the level's night.
#[derive(Clone, Copy)]
struct Sky { sun: Option<LightId>, to_sun: V3, dome: Option<SkyStateDome> }

fn sky_in_force(g: &Game) -> Sky {
    if let Some(st) = &g.m.hours.states {
        return Sky { sun: Some(st.sun), to_sun: st.to_sun, dome: None };
    }
    let moon = sky_lamp_of(g);
    let day = if crate::lighting::is_daylight(g) { g.level.daylight.as_ref().map(|d| d.sky) } else { None };
    let dome = match day {
        Some(s) => SkyStateDome { zenith: s.zenith, horizon: s.horizon },
        None => SkyStateDome { zenith: g.level.sky.zenith, horizon: g.level.sky.horizon },
    };
    let to_sun = match moon { Some(l) => { let d = g.tab.light[l].dir; [-d[0], -d[1], -d[2]] } None => [0.0, 1.0, 0.0] };
    Sky { sun: moon, to_sun, dome: Some(dome) }
}

/// What is kept of the sky's reach is of the statics as the copy has them now, and of this sun.
fn reach_fresh(g: &mut Game, to_sun: V3) {
    let wrote = ::sim::world::world(g.sim).wrote as f64;
    if g.m.hours.reach.wrote == wrote && g.m.hours.reach.to_sun == to_sun { return; }
    let box_ = box_of(g);
    g.m.hours.reach = crate::hours::SkyReach { wrote, to_sun, box_, cells: JsMap::new(), fine: JsMap::new() };
}

/// The rays kept for a cell, asked once from `at`. A copy: the list is read across calls that take `g`, and nothing writes a kept list once it is set.
fn kept_rays(g: &mut Game, fine: bool, key: crate::Num, at: V3) -> Vec<Open> {
    let kept = if fine { g.m.hours.reach.fine.get(key) } else { g.m.hours.reach.cells.get(key) };
    if let Some(o) = kept { return o.clone(); }
    let o = open_rays(g, at);
    let keep = if fine { &mut g.m.hours.reach.fine } else { &mut g.m.hours.reach.cells };
    keep.set(key, o.clone());
    o
}

/// The ambient's rays that leave point C unblocked by the statics. (hours.ts openRays)
fn open_rays(g: &mut crate::game::Game, c_2: V3) -> Vec<crate::hours::Open> {
    let to_sun = g.m.hours.reach.to_sun;
    let (sky_rays, gnd_rays) = (sky(), gnd());
    let mut out: Vec<Open> = Vec::new();
    let b = g.m.hours.reach.box_;
    for (i, r) in sky_rays.iter().enumerate() {
        let len = reach_of(b, c_2, r.d);
        if len > 0.05 && !static_blocked(g, c_2, v3::mad(c_2, r.d, len)) { out.push(Open { ray: i, len, sun_k: 0.0, sky_k: 0.0 }); }
    }
    for (i, r) in gnd_rays.iter().enumerate() {
        let len = m::min((c_2[1] - 0.03) / -r.d[1], reach_of(b, c_2, r.d));
        if len < 0.3 { continue; }
        let g_pt = v3::mad(c_2, r.d, len);
        if g_pt[1] > 0.2 || static_blocked(g, c_2, g_pt) { continue; }
        let sun_k = if to_sun[1] > 0.0 && !static_blocked(g, g_pt, v3::mad(g_pt, to_sun, reach_of(b, g_pt, to_sun))) { to_sun[1] } else { 0.0 };
        let sky_k = if static_blocked(g, g_pt, [g_pt[0], b.top, g_pt[2]]) { 0.0 } else { 1.0 };
        if sun_k > 0.0 || sky_k > 0.0 { out.push(Open { ray: sky_rays.len() + i, len, sun_k, sky_k }); }
    }
    out
}

/// What the sky's one directional light puts on a face turned to it.
fn sun_of(g: &Game, st: &Sky) -> f64 {
    match st.sun {
        Some(sun) if g.tab.light[sun].enabled && g.tab.light[sun].intensity > 0.0 => lum(g.tab.light[sun].color) * g.tab.light[sun].intensity,
        _ => 0.0,
    }
}

/// The sky as the light under a roof is worked out from it (sim/glow.rs).
pub fn sky_now(g: &crate::game::Game) -> ::sim::glow::Sky {
    let st = sky_in_force(g);
    ::sim::glow::Sky { to_sun: st.to_sun, sun: sun_of(g, &st), zenith: sky_lum(g, 1.0, st.dome), horizon: sky_lum(g, 0.0, st.dome) }
}

/// THE HOUR'S EXPOSURE against a night's (1): what a lamp's real strength is multiplied by to stand beside the hour's sun and sky, which are the real ones
/// times it already. By the light (lux) the hour's sun stands for, of which its authored strength is the picture. (render/photometry.ts hourExposure)
pub fn exposure(g: &crate::game::Game) -> f64 {
    let Some(st) = &g.m.hours.states else { return 1.0 };
    let lux = match st.def.id { HourId::Dawn => 600.0, HourId::Midday => 100000.0, HourId::Overcast => 3800.0, HourId::Dusk => 500.0, HourId::Night => return 1.0 };
    if st.def.sun.intensity > 0.0 { m::min(1.0, st.def.sun.intensity * 3.3 / lux) } else { 1.0 }
}

/// The sky's one directional light on P's six faces, where P has a line to it.
pub fn sun_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) {
    let st = sky_in_force(g);
    let i_sun = sun_of(g, &st);
    if i_sun <= 0.002 { return; }
    reach_fresh(g, st.to_sun);
    let t = reach_of(g.m.hours.reach.box_, p, st.to_sun);
    if t <= 0.05 || !crate::scene_collision::segment_blocked(g, p, v3::mad(p, st.to_sun, t), None, None, None) {
        let n6 = n6();
        for k in 0..6 {
            let cs = v3::dot(n6[k], st.to_sun);
            if cs > 0.0 { acc[k] += i_sun * cs; }
        }
    }
}

/// The ambient of P's cell on P's six faces: the sky's dome and the ground, as far as P's cell sees them.
pub fn dome_light(g: &mut crate::game::Game, p: V3, acc: &mut Vec<f64>) {
    let st = sky_in_force(g);
    // (a line first: the copy is brought up to date by the question, and what is kept is of the copy)
    let ix = m::floor(p[0] / CELL);
    let iz = m::floor(p[2] / CELL);
    let c: V3 = [(ix + 0.5) * CELL, CELL_Y, (iz + 0.5) * CELL];
    let cut_off = crate::scene_collision::segment_blocked(g, p, c, None, None, None);
    reach_fresh(g, st.to_sun);
    let i_sun = sun_of(g, &st);
    // the cell's rays stand for P while nothing is between P and the point they were asked from: else the quarter-cell's, else P's own, unkept
    let mut from = c;
    let open: Vec<Open> = if !cut_off {
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
        let dc = seam_box(g, db).c;
        if m::hyp2(dc[0] - p[0], dc[2] - p[2]) < DOOR_M { leaves.get_or_insert_with(Vec::new).push(db); }
    }
    let (sky_rays, gnd_rays) = (sky(), gnd());
    let dome = st.dome;
    let mut e_up = -1.0;   // the sky's irradiance on open ground: asked only when a ground ray is open
    for o in open.iter() {
        let ray = o.ray;
        let sky = ray < sky_rays.len();
        let r = if sky { &sky_rays[ray] } else { &gnd_rays[ray - sky_rays.len()] };
        if let Some(leaves) = &leaves {
            let mut hit = false;
            for &db in leaves.iter() {
                if let Some(h) = crate::scene_boxes::ray_box(g, from, r.d, db.into(), o.len) {
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

/// `t.intensity = t.peakIntensity = x` of a light (hours.ts:171, 227)
fn set_light_power(g: &mut Game, l: LightId, x: f64) {
    let row = &mut g.tab.light[l];
    row.intensity = x;
    row.peak_intensity = x;
}

// ---------------------------------------------------------------- seams: what the skeleton still lacks

/// A box's row (Box.* in the TypeScript). On a skeleton with the table `g.tab.boxes` (5.1), the body is `&mut g.tab.boxes[b]`.
fn seam_box(g: &mut Game, b: BoxId) -> &mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

#[cfg(test)]
mod tests {
    /// the night's haze is the one the TypeScript ships
    #[test]
    fn the_nights_haze_is_the_shipped_one() {
        let text = include_str!("../../../../../src/settings.ts");
        // (the first line that gives it a number: the type's own line comes before)
        let num = text.split("hazeDensity:").skip(1).map(|r| r.trim_start().chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect::<String>()).find(|n| !n.is_empty()).expect("src/settings.ts gives its haze a number");
        assert_eq!(num.parse::<f64>().unwrap(), super::NIGHT_HAZE);
    }

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
        let (sky_rays, gnd_rays) = (sky(), gnd());
        assert_eq!((sky_rays.len(), gnd_rays.len()), (15, 8));
        for (r, &(d, c)) in sky_rays.iter().chain(gnd_rays.iter()).zip(RAYS.iter()) {
            assert_eq!(b3(r.d), d);
            for k in 0..6 { assert_eq!(r.c6[k].to_bits(), c[k]); }
        }
    }
    // the words checked against the TypeScript's own functions under bun
    #[test]
    fn hour_words_are_read_as_the_typescript_reads_them() {
        assert_eq!(hour_id_of(Some("MidDay")), HourId::Midday);
        assert_eq!(hour_id_of(Some("sunset")), HourId::Dusk);
        assert_eq!(hour_id_of(Some("constructor")), HourId::Night);
        assert_eq!(hour_id_of(None), HourId::Night);
    }
    fn lamp(g: &mut Game, kind: f64, dir: V3, intensity: f64) -> LightId {
        let l = g.tab.light.push(crate::render_lights::RtLight { kind, dir, color: [1.0, 1.0, 1.0], intensity, enabled: true, ttl: -1.0, ..crate::render_lights::RtLight::default() });
        g.engine.lights.lights.push(l);
        l
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
        g.m.hours.states = Some(HourState { sun: dim, ..HourState::default() });   // an hour in: its sun is the estimate's, the night has no lamp
        assert_eq!(sky_lamp_of(&mut g), None);
    }
    #[test]
    fn the_sun_strength_lands_in_the_light() {
        let mut g = Game::blank();
        let sun = lamp(&mut g, 2.0, [0.0, -1.0, 0.0], 0.0);
        let mut st = HourState { sun, ..HourState::default() };
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
    // (the fold over real boxes waits on the table of Box rows: `seam_box`)
    #[test]
    fn with_no_statics_the_box_is_the_stand_in() {
        let mut g = Game::blank();
        let b = box_of(&mut g);
        assert_eq!((b.x0, b.x1, b.z0, b.z1, b.top), (0.0, 40.0, 0.0, 28.0, 8.0));
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
}
