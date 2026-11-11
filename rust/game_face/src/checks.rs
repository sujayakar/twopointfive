//! FOR A CHECK, and no host that plays.
use super::Held;

pub fn debug_panic(x: &mut Held) {
    // (half a write, as a rule that panics leaves one: the clock moved and nothing else)
    x.g.time += 1.0 / 60.0;
    let none: Option<u32> = None;
    let at = x.g.time;
    if none.is_none() { panic!("a planted panic, inside the game at {at:.3} s (face_debug_panic)"); }
}
