//! HOW A PUFF OF DUST IS DRAWN: the kernels read this for the dust kind (render/particles.ts) and take a change at the next frame. The picture alone.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DustLookK {
    /// opacity at a puff's center at its peak; it falls with (1 − age / life)² from birth
    #[k(0.3)] pub opacity: f64,
    /// how much a puff's radius has grown by the end of its life, as a share of its radius at birth (0 = it keeps its size)
    #[k(0.6)] pub growth: f64,
    /// seconds it takes to come up from nothing
    #[k(0.05)] pub fade_in: f64,
    /// × the softening where it meets a surface, which is a quarter of its width and at least 5 cm
    #[k(1.3)] pub soft: f64,
    /// the share of a crossing beam it scatters toward the eye, whichever side the beam is on
    #[k(0.5)] pub wrap: f64,
    /// × the light cache's ambient on it (× 0.9 on top)
    #[k(1.0)] pub ambient: f64,
}
