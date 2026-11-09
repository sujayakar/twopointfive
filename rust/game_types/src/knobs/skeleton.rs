//! The numbers of skeleton.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// A POSE'S BULK ARITHMETIC IN SINGLES (the maintainer, 2026-11-03, of the kernels a pose is blended and sampled by: "Singles, plain root for lengths"): with
/// `singles` on, blendPose and the step sampleClip takes between two keys are worked in singles, every operation rounded once (`Math.fround` here; the Rust
/// module's are singles to begin with — rust/sim/src/single.rs — and four nodes a step there), and a rotation is made a unit again by the plain root of its
/// squares where the doubles used the game's compensated length. A pose's numbers are stored as singles either way; what moves is their LAST PLACE, so it is a
/// rule of play: off = the build as it was, to the bit. Why: four singles fit a lane where two doubles do, and the compensated length is eight divides and a
/// root a node — in the module a blend is a third to a quarter of the time (docs/agents/rust.md has both languages' numbers).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PosemathK {
    #[k(true)] pub singles: bool,
}
