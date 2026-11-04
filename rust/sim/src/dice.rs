//! THE GAME'S OWN DIE (src/math/dice.ts SimDice, simRandom) — the one stream every draw of the simulation comes off: mulberry32, its whole state one word, and
//! the count of draws taken beside it. It lives in the game's block (state.rs) so that a rule here rolls it with no call out of the module, and so that a
//! rollback puts it back with everything else.
use crate::state::state;
use core::mem::size_of;

#[repr(C)]
pub struct Dice {
    /// draws taken off it (a count the host reads as a number)
    pub n: f64,
    /// the mulberry32 word
    pub s: u32,
    pub spare: u32,
}
const _: () = assert!(size_of::<Dice>() == 16);

/// dice.ts simRandom's own lines: one draw, uniform in [0, 1)
#[inline]
pub fn roll(d: &mut Dice) -> f64 {
    d.n += 1.0;
    let a = d.s.wrapping_add(0x6D2B79F5);
    d.s = a;
    let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
    t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
    ((t ^ (t >> 14)) as f64) / 4294967296.0
}
/// a draw off the die of the game in slot `g`
pub fn roll_of(g: usize) -> f64 {
    roll(&mut state(g).dice)
}
