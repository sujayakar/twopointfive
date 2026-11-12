//! THE BOX WORLD, ASKED BY THE PAGE: the camera's, the sound's, the cursor's questions. Nothing is kept.
use super::Held;
use game::view::{Met, SoundPath};
use std::cell::RefCell;

thread_local! {
    /// the ways round a sound's path found this tick, and for which game
    static WAYS: RefCell<(u32, game::ask::Ways)> = RefCell::new((0, Default::default()));
}
fn stopped(x: &Held, a: [f64; 3], b: [f64; 3], solid: Option<f64>) -> bool { game::scene_collision::segment_blocked(&x.g, a, b, None, solid, None) }
pub fn sight(x: &Held, a: [f64; 3], b: [f64; 3]) -> bool { stopped(x, a, b, None) }
pub fn solid(x: &Held, a: [f64; 3], b: [f64; 3]) -> bool { stopped(x, a, b, Some(0.0)) }
pub fn ray(x: &Held, o: [f64; 3], d: [f64; 3], max: f64) -> Option<Met> {
    game::scene_collision::raycast_host(&x.g, &game::ask::Over::OWN, o, d, max).map(|r| Met { t: r.t, n: r.n })
}
pub fn light(x: &mut Held, p: [f64; 3], owners: Vec<f64>, steady: bool, leave: f64) -> f64 { game::sense::light_asked(&mut x.g, p, &owners, steady, leave) }
pub fn hear(x: &Held, from: [f64; 3], to: [f64; 3]) -> SoundPath {
    // (taken out for the question, so a panic inside leaves no borrow behind)
    let mut k = WAYS.with(|k| std::mem::take(&mut *k.borrow_mut()));
    if k.0 != x.no { k = (x.no, Default::default()); }
    let p = game::ask::hear_kept(&x.g, &mut k.1, from, to);
    WAYS.with(|w| *w.borrow_mut() = k);
    SoundPath { direct: p.direct, routed: p.routed, path_len: p.path_len, via: p.via, apparent: p.apparent, muffle: p.muffle, carry: p.carry }
}
pub fn way(x: &Held, a: [f64; 3], b: [f64; 3], most: u32) -> Option<Vec<[f64; 3]>> {
    let on_floor = |p: [f64; 3]| [p[0], 0.0, p[2]];
    game::scene_collision::way_host(&x.g, on_floor(a), on_floor(b), most).map(|w| w.into_iter().map(on_floor).collect())
}
