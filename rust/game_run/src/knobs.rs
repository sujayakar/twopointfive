//! THE KNOBS BY NAME: every number of the game's tables that a person may turn while it plays (`knobs_list.rs`, written by tools/draw/gen.ts from the wire's table).
use crate::knobs_list::KNOBS;
use game::game::Game;

/// what a knob is turned to; None = there is no such knob
pub fn get(g: &Game, name: &str) -> Option<f64> { KNOBS.iter().find(|k| k.0 == name).map(|k| read(g, k.1, k.2)) }
/// turn it; false = there is no such knob, or what it was handed is no number
pub fn set(g: &mut Game, name: &str, v: f64) -> bool {
    let Some(&(_, at, yes)) = KNOBS.iter().find(|k| k.0 == name) else { return false };
    if !v.is_finite() { return false; }
    // (the list's places are the compiler's own: `offset_of!` of a field of the type it says)
    unsafe {
        let p = (&mut g.k as *mut game::knobs::Knobs as *mut u8).add(at);
        if yes { *(p as *mut bool) = v != 0.0 } else { *(p as *mut f64) = v }
    }
    true
}
fn read(g: &Game, at: usize, yes: bool) -> f64 {
    unsafe {
        let p = (&g.k as *const game::knobs::Knobs as *const u8).add(at);
        if yes { *(p as *const bool) as u8 as f64 } else { *(p as *const f64) }
    }
}
/// every knob as a panel lists them: its name, whether it is a yes or no, and what it is turned to
pub fn all(g: &Game) -> impl Iterator<Item = (&'static str, bool, f64)> + '_ { KNOBS.iter().map(|&(name, at, yes)| (name, yes, read(g, at, yes))) }
