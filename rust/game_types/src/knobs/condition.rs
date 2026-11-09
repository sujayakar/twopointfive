//! The numbers of condition.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ConditionK {
    /// the fastest a hurt AI man's legs are asked for, m/s (a teammate's own top pace is 2.2, a defender's chase 2.7, a walk on patrol 1.15)
    #[k(1.5)] pub hurt_pace: f64,
    /// the fire a hurt man's rounds are opened by, as if it were on him (suppression's own scalar: its flinch line is 0.3)
    #[k(0.35)] pub hurt_sup: f64,
    /// how long a man stays hurt, seconds (0 = for the rest of the mission)
    #[k(0.0)] pub hurt_secs: f64,
}
