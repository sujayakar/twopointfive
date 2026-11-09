//! The numbers of unseen.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct UnseenK {
    /// the wheel's pace at or under this share of his walk is sneaking (the notches: 1 · 0.87 · 0.74 · 0.61 · 0.48 · 0.35) · sneaking lasts this long after he stops (s)
    #[k(0.5)] pub pace_at: f64,
    #[k(1.5)] pub linger: f64,
    /// following at Sam's pace plus what closes the gap (off: the file's own pace, capped) · the least his legs are asked for · the most while low · the gap's gain (1/s)
    #[k(0.5)] pub min_pace: f64,
    #[k(1.45)] pub low_pace: f64,
    #[k(1.5)] pub gain: f64,
    /// nobody's legs for more than this on any walk while unseen (m/s; 0 = no cap — the silent order's own is 1.6)
    #[k(1.6)] pub cap: f64,
    /// a follower's place is along the wall Sam is on · how far off the face a body stands (m) · and how much further he may be stood where that floor is shut ·
    ///  how far past Sam along the face a man may be and still be dealt a place on Sam's other side (further: he would walk across Sam's front) · Sam nearer than this
    ///  to one end of the face: the file stacks away from that end
    #[k(0.42)] pub wall_off: f64,
    #[k(0.4)] pub wall_out: f64,
    #[k(0.5)] pub wall_cross: f64,
    #[k(1.6)] pub wall_end: f64,
    /// quiet footfalls · a crouched one's level · an upright walking one's (the walk's is 0.33; the silent order's 0.2; Sam's crouch 0.125)
    #[k(0.14)] pub low_step: f64,
    #[k(0.2)] pub walk_step: f64,
    #[k(30.0)] pub cue_gap: f64,
    #[k(0.8)] pub wall_secs: f64,
}
