//! The numbers of startle.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and its numbers: on (?startle=0 = a man who sees you goes for his gun at once,
///  as he did). `beat` s of the start; `draw` s after it for the gun to come up before a round can leave it (a man who draws: his feet still, his gun rising); `fresh` s since he last stood calm inside which a sighting still catches him unaware; `late` s past the end of a beat
///  after which draw-or-run is no longer asked (something else had him meanwhile: he is in the fight by now); `run`: a green man alone, caught inside `runM`
///  m and not inside `nearM` (at arm's length nobody turns his back: he draws), runs for it instead of drawing; `clearSecs`: how long a man running off a post is let run before the man he ran from can stop him to fight; `runSecs`:
///  the longest a run for cover lasts, and `stuckSecs` gaining under `stuckM` m ends it where he is
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct StartleK {
    #[k(0.5)] pub beat: f64,
    #[k(0.3)] pub draw: f64,
    #[k(1.0)] pub fresh: f64,
    #[k(0.25)] pub late: f64,
    #[k(8.0)] pub run_m: f64,
    #[k(2.5)] pub near_m: f64,
    #[k(1.5)] pub clear_secs: f64,
    #[k(4.0)] pub run_secs: f64,
    #[k(0.75)] pub stuck_secs: f64,
    #[k(0.3)] pub stuck_m: f64,
}
