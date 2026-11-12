//! THE KNOBS BY NAME: every number of the game's tables that a person may turn while it plays, and the live numbers a panel row turns.
use alloc::{string::String, vec::Vec};
use game::game::Game;
use game::knobs::{KnobTable, Knobs};

/// what a knob is turned to; None = there is no such knob
pub fn get(g: &Game, name: &str) -> Option<f64> { live(g, name).or_else(|| g.k.get(name)) }
/// turn it; false = there is no such knob, or what it was handed is no number
pub fn set(g: &mut Game, name: &str, v: f64) -> bool {
    if !v.is_finite() { return false; }
    set_live(g, name, v) || set_table(g, name, v)
}
/// turn a number of the tables alone: what a game not made yet has
pub fn set_table(g: &mut Game, name: &str, v: f64) -> bool { v.is_finite() && g.k.set(name, v) }
/// a number of the tables as the game ships it
pub fn shipped(name: &str) -> Option<f64> { Knobs::page().get(name) }
/// every knob as a panel lists them: its name, whether it is a yes or no, and what it is turned to
pub fn all(g: &Game) -> Vec<(String, bool, f64)> {
    let mut out = Vec::new();
    g.k.walk(&mut String::new(), &mut |name, yes, v| out.push((name.into(), yes, v)));
    out
}

/// THE NUMBERS A PERSON TURNS THAT ARE THE GAME'S OWN STATE AND NOT A TABLE'S: the weather as it stands
fn live(g: &Game, name: &str) -> Option<f64> {
    Some(match name {
        "weather.on" => g.weather.on as u8 as f64,
        "weather.storm" => g.weather.storm, "weather.rain_rate" => g.weather.rain_rate, "weather.wind.0" => g.weather.wind[0], "weather.wind.1" => g.weather.wind[1],
        _ => return None,
    })
}
fn set_live(g: &mut Game, name: &str, v: f64) -> bool {
    match name {
        // (the inner cone stays two degrees inside the outer, whichever is turned)
        "tune.flash_outer" => { g.k.tune.flash_outer = v; g.k.tune.flash_inner = g.k.tune.flash_inner.min(v - 2.0); }
        "tune.flash_inner" => g.k.tune.flash_inner = v.min(g.k.tune.flash_outer - 2.0),
        // (a storm turned by hand strikes on its own new clock)
        "weather.on" => game::weather::weather_set_enabled_2(g, v >= 0.5),
        "weather.storm" => if g.weather.storm != v { g.weather.storm = v; g.weather.next_flash_at = -1.0; },
        "weather.rain_rate" => g.weather.rain_rate = v,
        "weather.wind.0" => { let w = g.weather.wind; g.weather.wind = [v, w[1]]; }
        "weather.wind.1" => { let w = g.weather.wind; g.weather.wind = [w[0], v]; }
        _ => return false,
    }
    true
}
