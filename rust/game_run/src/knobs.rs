//! THE KNOBS BY NAME: every number of the game's tables that a person may turn while it plays (`knobs_list.rs`, written by tools/draw/gen.ts from the wire's table).
use crate::knobs_list::KNOBS;
use game::game::Game;

/// what a knob is turned to; None = there is no such knob
pub fn get(g: &Game, name: &str) -> Option<f64> { live(g, name).or_else(|| KNOBS.iter().find(|k| k.0 == name).map(|k| read(g, k.1, k.2))) }
/// turn it; false = there is no such knob, or what it was handed is no number
pub fn set(g: &mut Game, name: &str, v: f64) -> bool {
    if !v.is_finite() { return false; }
    set_live(g, name, v) || set_table(g, name, v)
}
/// turn a number of the tables alone: what a game not made yet has
pub fn set_table(g: &mut Game, name: &str, v: f64) -> bool {
    if !v.is_finite() { return false; }
    let Some(&(_, at, yes)) = KNOBS.iter().find(|k| k.0 == name) else { return false };
    // (the list's places are the compiler's own: `offset_of!` of a field of the type it says)
    unsafe {
        let p = (&mut g.k as *mut game::knobs::Knobs as *mut u8).add(at);
        if yes { *(p as *mut bool) = v != 0.0 } else { *(p as *mut f64) = v }
    }
    true
}
/// a number of the tables as the game ships it
pub fn shipped(name: &str) -> Option<f64> {
    let &(_, at, yes) = KNOBS.iter().find(|k| k.0 == name)?;
    Some(read_of(&game::knobs::Knobs::page(), at, yes))
}
fn read(g: &Game, at: usize, yes: bool) -> f64 { read_of(&g.k, at, yes) }
fn read_of(k: &game::knobs::Knobs, at: usize, yes: bool) -> f64 {
    unsafe {
        let p = (k as *const game::knobs::Knobs as *const u8).add(at);
        if yes { *(p as *const bool) as u8 as f64 } else { *(p as *const f64) }
    }
}
/// every knob as a panel lists them: its name, whether it is a yes or no, and what it is turned to
pub fn all(g: &Game) -> impl Iterator<Item = (&'static str, bool, f64)> + '_ { KNOBS.iter().map(|&(name, at, yes)| (name, yes, read(g, at, yes))) }

/// THE NUMBERS A PERSON TURNS THAT ARE THE GAME'S OWN STATE AND NOT A TABLE'S: the tune, the impacts' table, the weather as it stands
fn live(g: &Game, name: &str) -> Option<f64> {
    let (t, p) = (&g.tune, &g.m.particle_fx.pfx);
    Some(match name {
        "tune.flash_intensity" => t.flash_intensity, "tune.player_light" => t.player_light, "tune.flash_outer" => t.flash_outer, "tune.flash_inner" => t.flash_inner, "tune.ocp_duration" => t.ocp_duration, "wind.def" => t.wind, "weather.on" => game::weather::weather_enabled(g) as u8 as f64,
        "pfx.impact_scale_rifle" => p.impact_scale_rifle, "pfx.chip_size" => p.chip_size, "pfx.chip_life" => p.chip_life, "pfx.dust_size" => p.dust_size, "pfx.bang_sparks" => p.bang_sparks, "pfx.bang_ring" => p.bang_ring, "pfx.casing_life" => p.casing_life, "pfx.tink_gain" => p.tink_gain,
        "weather.storm" => game::weather::weather_storm(g), "weather.rain_rate" => game::weather::weather_rain_rate(g), "weather.wind.0" => game::weather::weather_wind(g)[0], "weather.wind.1" => game::weather::weather_wind(g)[1],
        _ => return None,
    })
}
fn set_live(g: &mut Game, name: &str, v: f64) -> bool {
    match name {
        "tune.flash_intensity" => g.tune.flash_intensity = v, "tune.player_light" => g.tune.player_light = v,
        // (the inner cone stays two degrees inside the outer, whichever is turned)
        "tune.flash_outer" => { g.tune.flash_outer = v; g.tune.flash_inner = g.tune.flash_inner.min(v - 2.0); }
        "tune.flash_inner" => g.tune.flash_inner = v.min(g.tune.flash_outer - 2.0),
        "tune.ocp_duration" => g.tune.ocp_duration = v,
        // (a game starts its wind from the table)
        "wind.def" => { g.tune.wind = v; g.k.wind.def = v; }
        "pfx.impact_scale_rifle" => g.m.particle_fx.pfx.impact_scale_rifle = v, "pfx.chip_size" => g.m.particle_fx.pfx.chip_size = v, "pfx.chip_life" => g.m.particle_fx.pfx.chip_life = v, "pfx.dust_size" => g.m.particle_fx.pfx.dust_size = v,
        "pfx.bang_sparks" => g.m.particle_fx.pfx.bang_sparks = v, "pfx.bang_ring" => g.m.particle_fx.pfx.bang_ring = v, "pfx.casing_life" => g.m.particle_fx.pfx.casing_life = v, "pfx.tink_gain" => g.m.particle_fx.pfx.tink_gain = v,
        // (a storm turned by hand strikes on its own new clock)
        "weather.on" => game::weather::weather_set_enabled_2(g, v >= 0.5),
        "weather.storm" => if game::weather::weather_storm(g) != v { game::weather::weather_set_storm(g, v); game::weather::weather_set_next_flash_at(g, -1.0); },
        "weather.rain_rate" => game::weather::weather_set_rain_rate(g, v),
        "weather.wind.0" => { let w = game::weather::weather_wind(g); game::weather::weather_set_wind(g, [v, w[1]]); }
        "weather.wind.1" => { let w = game::weather::weather_wind(g); game::weather::weather_set_wind(g, [w[0], v]); }
        _ => return false,
    }
    true
}
