//! WHAT THE GAME TELLS A PICTURE of its sky, its air and how the scene is taken. Worked out from state whenever it is asked, and nothing of it is kept: no rule reads
//! a picture's settings, and what a player turns on his own (Brightness, a panel's row) is his picture's and stays there (src/draw/sky.ts, the page's end).
use crate::game::Game;
use crate::v3::V3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Told { pub zenith: V3, pub horizon: V3, pub haze: f64, pub exposure: f64, pub gloss: bool }

/// the sky as the world has it, the kit stage's own left out: the show's day's, else the hour's under its weather, else the level's own night
pub fn sky_now(g: &Game) -> (V3, V3) {
    if crate::lighting::is_daylight(g) { if let Some(d) = g.level.daylight.as_ref() { return (d.sky.zenith, d.sky.horizon); } }
    let d = crate::hours::sky_dome(g);
    (d.zenith, d.horizon)
}

pub fn told(g: &Game) -> Told {
    let (zenith, horizon) = match g.kit_stage.as_ref().and_then(|st| st.sky_at) { Some(s) => s, None => sky_now(g) };
    let haze = match (crate::lighting::is_daylight(g), g.level.daylight.as_ref(), g.m.hours.states.as_ref()) {
        (true, Some(d), _) => d.haze,
        (_, _, Some(st)) => st.def.haze,
        _ => crate::hours::NIGHT_HAZE,
    };
    // (the wet lot's own gloss: while the ground is wet, and only under a weather that says so)
    Told { zenith, horizon, haze, exposure: crate::hours::exposure_of(g), gloss: g.weather.wet_on && g.weather.wet_gloss }
}
