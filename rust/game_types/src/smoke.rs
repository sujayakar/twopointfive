//! THE GAME'S OWN SMOKE, and the light that comes round under a roof. The rules are sim's bricks.rs and glow.rs.
use crate::bytes::ToBytes;
use crate::V3;
use alloc::vec::Vec;

/// WHAT LETS SMOKE OUT: where it stands, what it lets out a second (unit·m³), how long it runs, how long it has, how wide it is (m), and what is missing from the color of
/// what it lets out (1 − its tint, r g b: 0 = smoke's own grey)
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct Emitter {
    pub at: V3,
    pub rate: f64,
    pub ttl: f64,
    pub age: f64,
    pub radius: f64,
    pub dye: V3,
}
/// A BRICK: its place in plan (None = the slot is free), and what is missing from the color of its smoke, r g b, on average over all the smoke in it (whoever draws a game put back has
/// no other word of it; no eye and no rule reads it)
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct Brick {
    pub at: Option<(u16, u16)>,
    pub dye: V3,
}
/// THE LIGHT UNDER A ROOF, a number a square metre of the plan in each list: the best of the six faces at its middle, their mean, the mean with the lamps', which is what leaves its
/// surfaces, the face that looks up, and the level of the place it is in (a logarithm). Empty where nothing has swept: no light.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Glow {
    pub best: Vec<f64>,
    pub mean: Vec<f64>,
    pub all: Vec<f64>,
    pub up: Vec<f64>,
    pub place: Vec<f64>,
    /// the word of the lamps it is swept under (0 = never, or not since the light model was last down), which metre the sweep comes to next, how many it has still to go through at the
    /// quick pace
    pub lamps: f64,
    pub at: f64,
    pub left: f64,
}
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Smoke {
    /// the plan in bricks, along x and along z (0 = no step has been taken on any level yet)
    pub wide: u32,
    pub deep: u32,
    /// steps taken
    pub steps: u32,
    /// THE BOOK, two sums in unit·m³ that thin at the cells' own rate, so that at every step what the cells hold + what was given back = what was let out: all that was let out; what thin
    /// bricks held when they were given back, and what the wind carried over the plan's edge
    pub out: f64,
    pub given: f64,
    /// seconds owed toward the next step
    pub clock: f64,
    pub emitters: Vec<Emitter>,
    /// by slot, no free one at the end: a new brick takes the first that is free
    pub bricks: Vec<Brick>,
    /// the density of every cell: a brick's 128 stand together, four along x, then along z, then eight up
    pub rho: Vec<f64>,
    /// the brick at a place of the plan, plus one (0 = none)
    /// (as the bricks say: in no bytes)
    #[bytes(skip)]
    pub at: Vec<u16>,
    pub glow: Glow,
}
