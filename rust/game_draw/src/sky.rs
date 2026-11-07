//! THE SKY, THE HOUR AND THE WEATHER as the picture draws them, and what the level says of its own look.
use crate::Row;
use alloc::vec::Vec;
use game::game::Game;
use game::hours::HourId;

section! {
    /// the sky: one row
    pub struct Sky {
        /// the sky's colours up and level, the haze and the exposure the picture is taken at
        pub zenith: [f32; 3],
        pub horizon: [f32; 3],
        pub haze: f32,
        pub exposure: f32,
        /// polished floors mirror
        pub gloss: u32,
        /// the hour that is up (0 night, 1 dawn, 2 midday, 3 overcast, 4 dusk) and how strong its sun is in the open
        pub hour: u32,
        pub hour_sun: f32,
        /// snow lies on this level
        pub snow_level: u32,
        /// the level's own sky, how kept the place is (under 0: it does not say) and its marks' texel (0: the usual)
        pub own_zenith: [f32; 3],
        pub own_horizon: [f32; 3],
        pub kept: f32,
        pub mark_texel: f32,
        /// the weather in the open air (render/frame.ts WeatherFrame, in its order)
        pub weather: u32,
        pub rain: f32,
        pub snow: f32,
        pub shear: [f32; 2],
        pub wet: f32,
        pub flash: [f32; 3],
        pub rain_top: f32,
        pub interior: [f32; 4],
        pub annex: [f32; 4],
        pub roof_c: u32,
        pub roof_d: u32,
        pub fog: u32,
        pub near_fade: [f32; 4],
        /// the streaks' phase function; its last number 0 = none said
        pub phase: [f32; 4],
    }
}

pub fn fill(g: &Game, out: &mut Vec<Sky>) {
    let f3 = |v: [f64; 3]| v.map(|x| x as f32);
    let f4 = |v: [f64; 4]| v.map(|x| x as f32);
    let s = &g.engine.settings;
    let mut r = Sky::ZERO;
    r.zenith = f3(s.sky_zenith);
    r.horizon = f3(s.sky_horizon);
    r.haze = s.haze_density as f32;
    r.exposure = s.exposure as f32;
    r.gloss = s.gloss as u32;
    r.hour = match game::hours::hour_of(g) { HourId::Night => 0, HourId::Dawn => 1, HourId::Midday => 2, HourId::Overcast => 3, HourId::Dusk => 4 };
    r.hour_sun = g.m.hours.states.as_ref().map_or(0.0, |st| st.def.sun.intensity as f32);
    r.snow_level = g.snow_tracks.is_some() as u32;
    r.own_zenith = f3(g.level.sky.zenith);
    r.own_horizon = f3(g.level.sky.horizon);
    r.kept = g.level.kept.unwrap_or(-1.0) as f32;
    r.mark_texel = g.level.mark_texel.unwrap_or(0.0) as f32;
    if let Some(w) = g.engine.weather.as_ref() {
        r.weather = 1;
        r.rain = w.rain as f32;
        r.snow = w.snow.unwrap_or(0.0) as f32;
        r.shear = [w.shear_x as f32, w.shear_z as f32];
        r.wet = w.wet as f32;
        r.flash = f3(w.flash);
        r.rain_top = w.rain_top as f32;
        r.interior = f4(w.interior);
        r.annex = f4(w.annex);
        // (packed words that ride a float's slot: 32 bits each)
        r.roof_c = w.roof_c.unwrap_or(0.0) as u32;
        r.roof_d = w.roof_d.unwrap_or(0.0) as u32;
        r.fog = w.fog.unwrap_or(0.0) as u32;
        r.near_fade = f4(w.near_fade);
        r.phase = w.phase.map_or([0.0; 4], f4);
    }
    out.push(r);
}
