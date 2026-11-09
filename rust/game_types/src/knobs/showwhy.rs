//! The numbers of showwhy.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ShowwhyK {
    /// LINE. s the player has masked his round without a break before he shows it · a break shorter than this is no break · s from his first wave to his
    ///  second · s between waves after that · the line stands at least this long once said · s after one man's line before another man says it · m either
    ///  side of his line a mate counts as standing in it
    #[k(0.6)] pub line_after: f64,
    #[k(0.3)] pub line_gap: f64,
    #[k(2.6)] pub wave_every: f64,
    #[k(6.0)] pub wave_later: f64,
    #[k(1.2)] pub line_min: f64,
    #[k(4.0)] pub line_say_gap: f64,
    #[k(0.45)] pub mate_half: f64,
    /// STACK. s between two squeezes up the file · m a hand reaches the man ahead · s after the last squeeze (or, with none, after the stack is made) before
    ///  the lead's fist · s to the second fist (doubled each time after) · fists in all for one wait
    #[k(0.35)] pub squeeze_gap: f64,
    #[k(1.15)] pub squeeze_reach: f64,
    #[k(0.45)] pub fist_after: f64,
    #[k(8.0)] pub hold_every: f64,
    #[k(3.0)] pub hold_max: f64,
    /// DOOR. s held at a doorway the player stands in before the hand · s before the same man shows it again · s between two "after you." from the squad
    #[k(0.5)] pub door_after: f64,
    #[k(4.0)] pub door_every: f64,
    #[k(10.0)] pub after_you_gap: f64,
    /// HURT. "I'm hit." is not said later than this many s after the blast (a load, a man folded for a long time) · m behind a running player before
    ///  "I can't keep up." · s between two of those from one man · how many he says in all
    #[k(6.0)] pub hurt_late: f64,
    #[k(6.0)] pub lag_m: f64,
    #[k(20.0)] pub lag_every: f64,
    #[k(2.0)] pub lag_max: f64,
    /// KEEP. s he has kept the door before he says so · s after a mate's keep line before his · s before the same man says it again
    #[k(0.4)] pub keep_after: f64,
    #[k(1.5)] pub keep_gap: f64,
    #[k(30.0)] pub keep_every: f64,
    /// a line owed is dropped when it has waited this long for his own words to be spent
    #[k(5.0)] pub owed_for: f64,
}
