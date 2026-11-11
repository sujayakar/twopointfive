//! THE LAMPS A LEVEL'S FIXTURES FEED, AS A LIGHT QUESTION READS THEM: a row a lamp (a fixture's own light, an area light of a flat fixture or a marking, a beacon's
//! spot, a camera's cone), in the order of the engine's list, stood once as the level is built. The order is what a sum of floats over them is made of, so nothing
//! sorts them. Lamps nobody's fixture feeds stay the question's own (a man's torch, a flash), each with how many rows stand before it in the list.
//!
//! What a row is fed by (a fixture, the mains, a camera) and how that stands is the rules' (game_rules air/fixtures.rs), which write a row as it changes.
use core::mem::size_of;

/// lamps fixtures feed (a level over it is refused at load, by name).
pub const ROWS: usize = 256;

// a row's truths (`row_bits`): burning (the reader's gate is this, the first self's intensity > 0 and kind != 2: light.rs) · who asks what the PLACE lights leaves it
// out · a screen's own lamp · what feeds it (two bits) · it has a steady self (`sx` … `s_intensity`; else they are zeros and the first self is both)
pub const R_ON: u32 = 1;
pub const R_PASSING: u32 = 2;
pub const R_SCREEN: u32 = 4;
pub const R_ROLE: u32 = 24;
pub const R_OWN: u32 = 0;
pub const R_AREA: u32 = 8;
pub const R_BEACON: u32 = 16;
pub const R_CONE: u32 = 24;
pub const R_STEADY: u32 = 32;
/// what the lamp is by day (light.rs by_day), three bits from here
pub const R_DAY: u32 = 6;

/// The lamps' part of a game's block.
#[repr(C)]
pub struct Lights {
    pub rows: u32,
    pub spare: [u32; 3],
    pub x: [f64; ROWS],
    pub y: [f64; ROWS],
    pub z: [f64; ROWS],
    pub range: [f64; ROWS],
    pub radius: [f64; ROWS],
    pub kind: [f64; ROWS],
    pub dx: [f64; ROWS],
    pub dy: [f64; ROWS],
    pub dz: [f64; ROWS],
    pub outer: [f64; ROWS],
    pub inner: [f64; ROWS],
    pub r: [f64; ROWS],
    pub g: [f64; ROWS],
    pub b: [f64; ROWS],
    pub intensity: [f64; ROWS],
    // ---- its steady self, where R_STEADY is up: what who asks what the PLACE lights takes (a fire's, a bad tube's)
    pub sx: [f64; ROWS],
    pub sy: [f64; ROWS],
    pub sz: [f64; ROWS],
    pub s_radius: [f64; ROWS],
    pub s_intensity: [f64; ROWS],
    // ---- what feeds it: an area light's own strength (a beacon's: its peak), and the fixture it follows
    pub row_base: [f64; ROWS],
    pub row_bits: [u32; ROWS],
    pub row_target: [u32; ROWS],
}
const _: () = assert!(size_of::<Lights>() == 16 + 21 * ROWS * 8 + 2 * ROWS * 4);

impl Lights {
    /// a row's burning: whether it is on, and how strong
    #[inline]
    pub fn burn(&mut self, r: usize, on: bool, intensity: f64) {
        self.intensity[r] = intensity;
        self.row_bits[r] = (self.row_bits[r] & !R_ON) | if on { R_ON } else { 0 };
    }
}
