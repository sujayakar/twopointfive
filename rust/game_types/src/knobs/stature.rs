//! STATURE: how much taller or shorter than his skeleton a cast man is drawn, as a share of full height. The picture alone: his bones, hit capsules and every box a
//! ray meets stay as they are. A player's man is always 1, and so are two bodies that have to meet (one held, one hauled). Each cast variant's own stature is in `variant_stature`.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StatureK {
    /// ± this much on top of his variant's stature, dealt by his id
    #[k(0.015)] pub jitter: f64,
    /// the shortest and the tallest anybody is drawn, however his variant and `jitter` add up
    #[k(0.92)] pub min: f64,
    #[k(1.05)] pub max: f64,
    /// how fast a man's drawn scale moves toward what it should be, per second (it's how he comes back to 1 when he's grabbed, and goes back after)
    #[k(0.25)] pub ease_per_sec: f64,
}

