//! The numbers of handwork.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct HandworkK {
    /// m off the thing's face he is stood to work at it · how he is walked there (an ease, 1/s, at no more than this pace, m/s) · m to one side, so the working shoulder is before the spot
    #[k(0.52)] pub stand_m: f64,
    #[k(7.0)] pub step_rate: f64,
    #[k(1.7)] pub step_pace: f64,
    #[k(0.1)] pub stand_side: f64,
    /// s a verb's clock waits, at most, for his hand to reach the thing (a hand that cannot get there keys nothing — the verb still goes)
    #[k(0.7)] pub wait_secs: f64,
    /// Sam's breaker: s the move takes once his hand is on the lever · s into it the lever goes over (the lights with it)
    #[k(0.5)] pub lever_secs: f64,
    #[k(0.3)] pub lever_at: f64,
    /// the objective: s the hand takes putting it away once it is out
    #[k(0.45)] pub stow_secs: f64,
    /// the radio hand: × the line's own time on screen, and its bounds (s)
    #[k(0.6)] pub radio_k: f64,
    #[k(0.9)] pub radio_min: f64,
    #[k(2.4)] pub radio_max: f64,
}
