//! WALL WEAR: what a breakable wall or piece of cover shows while it's being shot, before it gives. The picture alone: how many rounds a cell takes is the wall's own
//! rule, and every counted round's mark, and the dust and chips it throws, are sized here by how far gone its cell is.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallwearK {
    /// a cell's strength = (rounds counted ÷ rounds it takes) to this power: under 1, the first rounds already show a lot, 1 = in step with the count
    #[k(0.4)] pub curve: f64,
    /// each mark's strength is up to this share below its cell's, by the mark's own seed, so that one cell's marks aren't alike (0 = all the same)
    #[k(0.25)] pub vary: f64,
    /// a mark through a thin wall throws one chip off the far face while its cell's strength is under this, and 3 + 8 × the strength from here on
    #[k(0.3)] pub exit_chips_from: f64,
    /// the most particles one round may throw off the wall, puffs, far-face chips and near-face flakes together
    #[k(16.0)] pub max_births: f64,
    /// how many cells remember their marks; past that the oldest forgets, and its marks stop growing
    #[k(256.0)] pub max_cells: f64,
    pub mark: WallwearKMark,
}

/// The size of a mark, and which walls it shows through.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallwearKMark {
    /// a mark's reach (m) = `reach0` + `reach1` × its strength. That's its radius; a through mark's radius is the long side of a right triangle with the reach and the wall's half thickness
    #[k(0.09)] pub reach0: f64,
    #[k(0.21)] pub reach1: f64,
    /// half thicknesses (m) between these two make a thin wall: a hit on its broad face marks both faces and throws out the far side. Thinner or thicker, it marks only the near face
    #[k(0.035)] pub min_half: f64,
    #[k(0.09)] pub max_half: f64,
}

