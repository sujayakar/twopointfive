//! The doors' part of a block: the leaves stand on the game's own doors now.
/// doors a level can have
pub const CAP: usize = 64;
#[repr(C)]
pub struct Doors {
    pub spare: [f64; 2],
}
