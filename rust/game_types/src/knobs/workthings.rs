//! The numbers of workthings.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WorkK {
    /// the cursor's point within this of the thing (planar, metres), on its own side of the wall it is on, is the thing · the mission's thing, whose face is
    ///  as wide as its rack (and whose top the cursor may be on: `behindM` behind its face still counts; the breaker: its own box's depth)
    #[k(0.45)] pub pick_m: f64,
    #[k(0.9)] pub pick_wide_m: f64,
    #[k(0.5)] pub behind_m: f64,
    #[k(0.09)] pub behind_panel_m: f64,
    /// he stands this far off its face · and his partner this far behind him
    #[k(0.5)] pub stand_m: f64,
    #[k(1.4)] pub buddy_m: f64,
    /// the hand goes out for this long before the thing is worked · the breaker's handle then takes this long (the mission's thing: DRIVE_SECS)
    #[k(0.45)] pub hand_secs: f64,
    #[k(0.7)] pub breaker_secs: f64,
    /// the hand comes off over this long · and goes out only from within this of the face (a stand further off works it at Sam's own reach, no hand shown)
    #[k(0.4)] pub fade_secs: f64,
    #[k(0.85)] pub hand_m: f64,
    /// a following teammate does not wait, back off or stand still for a suspicious man who is walking to the breaker while he is stood within `walkM` (m)
    ///  of what is left of that man's walk and Sam is going on: he goes on after Sam (false: he waits where he is, as for any man who is looking, and is
    ///  walked into)
    #[k(2.5)] pub walk_m: f64,
}
