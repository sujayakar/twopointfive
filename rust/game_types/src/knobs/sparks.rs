//! THE GAME'S OWN SPARKS: what a stun throws to hang its smoke trails on and a device shot dead gives off, each a thin glowing box along its travel. The smoke on them is the game's
//! (`effects.trail`); the boxes are the picture's, no ray meets them.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SparksK {
    /// how one flies: m/s² down · its speed falls off at this rate (1/s) · it meets walls, and bounces at a quarter (off: it flies through)
    #[k(16.0)] pub gravity: f64,
    #[k(1.6)] pub drag: f64,
    #[k(true)] pub collide: bool,
    /// the share of its life it burns at its brightest, and then fades by the square · its brightness · and of the box at that brightness of 1 (the hotter, the whiter)
    #[k(0.15)] pub hold: f64,
    #[k(2.6)] pub brightness: f64,
    #[k(25.0)] pub emissive: f64,
    /// m: a box across · the shortest it is drawn · s: the travel one box shows
    #[k(0.002)] pub thickness: f64,
    #[k(0.02)] pub min_streak: f64,
    #[k(0.017)] pub shutter: f64,
    /// as many as are alive at once, up to the pool's 96: one more takes the slot of the one nearest its end
    #[k(96.0)] pub max_live: f64,
}
