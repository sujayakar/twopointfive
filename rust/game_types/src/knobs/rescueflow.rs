//! The numbers of rescueflow.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RescueflowK {
    /// metres: Sam passes an opening this far short of it · back this far short of one he has passed, it is unpassed · this far past a job's opening he is past it
    #[k(0.6)] pub abreast: f64,
    #[k(2.0)] pub unpass: f64,
    #[k(1.5)] pub past: f64,
    /// seconds: the most a man covers an opening once he is on it · a man not on his mark by then is let off it · how long a wish for a man stands
    #[k(2.5)] pub max_cover: f64,
    #[k(7.0)] pub give_up: f64,
    #[k(6.0)] pub want_secs: f64,
    /// seconds: a man holds at a hot opening while the last word of somebody behind it is this fresh · a place somebody was known at is dropped after this long, or sooner
    /// with this share of the room looked over since and the place itself in view
    #[k(6.0)] pub hot_hold: f64,
    #[k(20.0)] pub hot_secs: f64,
    #[k(0.6)] pub hot_empty: f64,
    /// an opening is looked into with this share of what can be seen through it seen inside these seconds
    #[k(0.6)] pub look_share: f64,
    #[k(10.0)] pub look_secs: f64,
    /// a room somebody has just been in is cleared with this share of its floor seen inside these seconds
    #[k(0.7)] pub clear_share: f64,
    #[k(25.0)] pub clear_secs: f64,
    /// how far an eye counts (m) · times a second the floor is sensed · the height its points stand at (m)
    #[k(16.0)] pub eye_m: f64,
    #[k(6.0)] pub sense_hz: f64,
    #[k(0.9)] pub sample_y: f64,
    /// how long after Sam has turned into a side room the last man is still told to hold the corridor behind him (s) · how far ahead of a man with more to do Sam is before he
    /// is told to wait (m)
    #[k(3.0)] pub hold_secs: f64,
    #[k(9.0)] pub wait_m: f64,
    /// how bright the strips on the floor are (the picture's)
    #[k(1.0)] pub glow: f64,
}
