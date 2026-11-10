//! The numbers of doorpath.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct DoorpathK {
    /// where the lane cuts the frame line: this share of the opening from the hinge (the leaf's root is at 0). 0.52 since the doors are real size
    ///  : between a leaf standing square on its stop and the latch stop a man's body has 0.325 to 0.556 m of a 0.86 m leaf's frame line to
    ///  put his middle on, and the lane runs down the middle of that (0.6, as it was for the 1.16 m leaves, ran him a hand off the latch stop)
    #[k(0.52)] pub at: f64,
    /// the lane's ends: metres short of the frame line, metres past it
    #[k(0.7)] pub out_m: f64,
    #[k(0.7)] pub in_m: f64,
    /// how far ahead of him on his line the point he walks at is kept
    #[k(0.5)] pub look: f64,
    /// no leg nearer than this to a leaf's centre line as it stands (the door's own contact starts at 0.365)
    #[k(0.45)] pub leaf_clear: f64,
    /// the arc round a leaf's free tip is laid this far out from it (a man walking at a point ahead of him cuts a corner by a hand or two)
    #[k(0.75)] pub tip_m: f64,
    /// let go when the man before him will be this far ahead at the frame line · eased off inside this of a man sent before him (stopped 0.35 nearer).
    ///  0 and 0.6 = the next man goes the moment the man before him will reach the line first, and gives way only to a body's width: the four cross inside
    ///  about 1.1 s, half a second apart. (0.5 and 1.05, as first built, spaced them 0.6 s apart and brought the second man through the depot's first door
    ///  0.2 s after the old entry did — his partner fought the room alone for that long, and the depot lost 24 teammates in 576 over 144 seeds)
    #[k(0.0)] pub gap_m: f64,
    #[k(0.6)] pub short_m: f64,
    /// the most a man is held back, seconds: at the stack, and again on the way
    #[k(2.5)] pub cap: f64,
    /// a point behind a leaf is moved at most this far · and stays this far from another man's point
    /// (moveM 0 since the doors rules share this tree: where a body rests behind a leaf is theirs; 1.8 = the walk moving a point by itself)
    #[k(0.0)] pub move_m: f64,
    #[k(0.7)] pub apart_m: f64,
    /// a man down within this of a point still being walked to: the point is moved off him (0: never — a man walks onto the fallen as before) · the new
    ///  point stands this far from where any man down lies · and at most this far from the point as dealt
    #[k(0.0)] pub body_m: f64,
    #[k(0.6)] pub body_off: f64,
    #[k(1.2)] pub body_move: f64,
}
