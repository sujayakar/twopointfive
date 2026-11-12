//! FOR A CHECK, and no host that plays.
use super::Held;
use game::game::Game;

pub fn debug_panic(x: &mut Held) {
    // (half a write, as a rule that panics leaves one: the clock moved and nothing else)
    x.g.time += 1.0 / 60.0;
    let g: &Game = &x.g;
    let words = format!("a planted panic, inside the game at {:.3} s (face_debug_panic)", g.time);
    // (from a price of a search for a way, which has the tables every game searches with)
    let from = g.level.player_spawn;
    for &d in &g.doors.list { let _ = game::scene_collision::way_priced(g, from, g.dr[d].frame_centre, 6000.0, &mut |_: &mut &Game, _, _| -> f64 { panic!("{words}, in a search") }); }
    panic!("{words}");
}
