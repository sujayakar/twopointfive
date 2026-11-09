//! The numbers of rescueflush.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RescueflushK {
    /// seconds from the room falling to the first man moving on it · between one man's word and the next man's · before a man who has come to rest is sent again
    #[k(4.0)] pub after_secs: f64,
    #[k(2.5)] pub gap_secs: f64,
    #[k(8.0)] pub again_secs: f64,
    /// a man with a clear line to a living hostage inside this many metres has her · one who came for her is this long from turning his gun on her
    #[k(10.0)] pub reach_m: f64,
    #[k(3.0)] pub kill_secs: f64,
    /// a man with one of the team in his sights this recently is fighting: not sent anywhere
    #[k(2.0)] pub busy_secs: f64,
    /// smoke between him and her thinner than this hides her from him (the bar guards.ts puts between two AI men)
    #[k(0.3)] pub smoke_k: f64,
    /// a man sent round has reached a mark of his way inside this many metres of it: on to the next, and from the side door in
    #[k(1.8)] pub at_door_m: f64,
}
