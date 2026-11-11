//! The numbers of spotted.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SpottedK {
    /// THE switch: off here, raised by the pair-up rule
    #[k(false)] pub on: bool,
    /// ---- suspicious
    /// … and still (following only), while Sam is within `leashM`
    #[k(7.0)] pub leash_m: f64,
    /// lit (the meter at `litAt` or over) and in his line: the nearest spot within `slipM` out of the line, or dark
    #[k(3.2)] pub slip_m: f64,
    #[k(0.3)] pub lit_at: f64,
    /// … but a moving shape draws the eye: he holds still first, and moves off the line only once a man has been turned his way this long (s; 0 = at once)
    #[k(1.5)] pub slip_wait: f64,
    /// how far off a suspicious man is his business (m, straight) · and inside which a boot is (a silent step carries about 3.6 m)
    #[k(14.0)] pub wary_m: f64,
    #[k(5.0)] pub ear_m: f64,
    /// he stays down this long after the last such man has settled or dropped (s)
    #[k(1.5)] pub linger_secs: f64,
    #[k(3.0)] pub brace_secs: f64,
    /// a roused man who has turned to him inside `takeM` is taken by hand: `takeSecs` with his hands on him inside `takeReach` (m)
    #[k(2.2)] pub take_m: f64,
    #[k(1.05)] pub take_reach: f64,
    #[k(0.6)] pub take_secs: f64,
    /// ---- caught
    /// an alert man's sighting of one of them counts for this long (s) · within this of the teammate (m) · and 'caught' lasts this long past the last one (s)
    #[k(2.5)] pub fresh_secs: f64,
    #[k(30.0)] pub caught_m: f64,
    #[k(6.0)] pub last_secs: f64,
    #[k(3.2)] pub step_m: f64,
    #[k(35.0)] pub face_deg: f64,
    #[k(6.0)] pub close_m: f64,
}
