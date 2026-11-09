//! The numbers of deathrecap.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RecapK {
    /// seconds of the floor kept before a death
    #[k(8.0)] pub secs: f64,
    /// seconds kept after it (he is seen to fall, the shooter to move on)
    #[k(1.2)] pub after: f64,
    /// samples a second
    #[k(20.0)] pub hz: f64,
    /// deaths kept with their own last seconds (a player's is never the one dropped)
    #[k(6.0)] pub keep: f64,
    /// saw him go down = a line to him, within this far, inside this long of the fall · heard = gunfire (a burst) whose sound path to an ear is this long at most
    #[k(25.0)] pub live_see_m: f64,
    #[k(1.0)] pub live_saw_secs: f64,
    #[k(25.0)] pub live_hear_m: f64,
    /// the beat before the squad has it, s · the man who saw says which side only with the player this near the fallen man (his shoulders mean nothing further
    ///  off) · how long the stroke on the ground stays, s
    #[k(0.6)] pub live_beat: f64,
    #[k(8.0)] pub live_side_m: f64,
}
