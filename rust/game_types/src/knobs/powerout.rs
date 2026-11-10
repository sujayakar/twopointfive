//! The numbers of powerout.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// UNTUNED by play: nobody has played against it.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PoweroutK {
    /// m: a burning mains fixture in a man's plain view this near is one he saw die
    #[k(14.0)] pub see_m: f64,
    /// s between the trip and their word on the net (by id) · s between the word and the first step
    #[k([3.0, 6.0])] pub think: [f64; 2],
    #[k(2.0)] pub beat: f64,
    /// … or the floor he stands on, watches (out to `lookM`, every `stepM`) or has about him (`aboutM` every way) read `litAt` or over by the place's light and falls under `dropK` of it, read `darkAfter` s after the trip
    #[k(12.0)] pub look_m: f64,
    #[k(4.0)] pub about_m: f64,
    #[k(2.0)] pub step_m: f64,
    #[k(0.3)] pub fan_rad: f64,
    #[k(0.3)] pub lit_at: f64,
    #[k(0.5)] pub drop_k: f64,
    #[k(0.3)] pub dark_after: f64,
    /// men who go · the longest way they will walk for it, m · a garrison with nobody to spare sends its nearest man alone
    #[k(2.0)] pub men: f64,
    #[k(60.0)] pub max_m: f64,
    /// times a garrison walks to the breaker in one fight (the next time the lights go it is no fault)
    #[k(1.0)] pub parties: f64,
    /// s a man looks at a breaker shot to scrap before he says so · legs a walk is tried in before it is given up
    #[k(1.5)] pub look_secs: f64,
    /// s the first man stands at a lever that will not take his hand before he gives it up · s past the walk's own length before a party that lost a man and
    ///  sent no word is missed ("nothing from the breaker") · the nearest men, in a straight line, whose way to the lever is worked out
    #[k(8.0)] pub lever_secs: f64,
    #[k(15.0)] pub overdue: f64,
    #[k(6.0)] pub ask: f64,
    /// the second man: how far short of the lever he stops, m (nearest free floor in that band with a line to it)
    #[k([1.6, 3.2])] pub cover_m: [f64; 2],
}
