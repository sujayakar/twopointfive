//! The numbers of laststand.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LaststandK {
    /// `places`: he has been out of the place he named this long (s), and has stood still this long (s), before he says where he is now; no man says it more
    ///  often than every `movedGap` s
    #[k(3.0)] pub moved_secs: f64,
    #[k(2.0)] pub still_secs: f64,
    #[k(8.0)] pub moved_gap: f64,
    /// 'most are down': this many defenders or fewer are up — and never more than a third of the staging (eleven: three; eight: two; five: the last man)
    #[k(3.0)] pub few: f64,
    /// nobody of the team in his sight these seconds: a man may ask, answer, be ordered
    #[k(3.0)] pub quiet: f64,
    /// seconds from 'most are down' to the first radio check [min, max]
    #[k([3.0, 6.0])] pub first: [f64; 2],
    /// seconds between checks, and between them once the stand is made
    #[k(22.0)] pub every: f64,
    /// an answer's beat after the check [min, max], and between two answers (their net's strip takes a line of a kind every three seconds)
    #[k([1.2, 2.2])] pub answer: [f64; 2],
    #[k([3.2, 3.9])] pub between: [f64; 2],
    /// the one who asked listens this long after the last answer is due [min, max] (what he says then is a line of the answers' kind: past the strip's three seconds)
    #[k([3.2, 3.8])] pub listen: [f64; 2],
}
