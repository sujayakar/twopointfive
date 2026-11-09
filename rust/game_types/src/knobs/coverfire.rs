//! The numbers of coverfire.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct CoverfireK {
    /// s a sighting of his own stays a place worth rounds · s a mate's eyes-on stays a call · how far a call carries between two men of a side
    #[k(5.0)] pub stale_secs: f64,
    #[k(1.2)] pub call_secs: f64,
    #[k(14.0)] pub call_m: f64,
    /// m from the piece's face to the man's known place, beyond which he is not 'behind it'
    #[k({ let mut t = JsMap::new(); t.set(String::from("cover"), 3.0); t.set(String::from("wall"), 4.0); t })] pub behind_m: JsMap<String, f64>,
    /// pieces of breakable the line may cross before the man · this gun's rounds all of them still want, at most · the reach of it
    #[k(3.0)] pub max_layers: f64,
    #[k(20.0)] pub max_rounds: f64,
    #[k(30.0)] pub max_m: f64,
    #[k(1.5)] pub min_m: f64,
    /// m from his shoulder to the piece, under which it is not shot at: his muzzle is against it or through it (a round fired there is nobody's)
    #[k(1.0)] pub near_m: f64,
    /// rounds he will put into a piece that stands: slackK × what it wanted when he started + slack; then it is left alone this long
    #[k(1.5)] pub slack_k: f64,
    #[k(3.0)] pub slack: f64,
    #[k(20.0)] pub give_secs: f64,
    /// m off the lane (wider by coneK per metre out) a friend / hostage / civilian holds the round · m past the place a round that got through is followed
    #[k(0.6)] pub lane: f64,
    #[k(0.03)] pub cone_k: f64,
    #[k(8.0)] pub past_m: f64,
    /// the aim point is kept through a newer sighting within this of it · s a chew stays his with his line off it (a step of his own)
    #[k(1.5)] pub same_m: f64,
    #[k(1.0)] pub linger_secs: f64,
    /// 'he went behind it': the place a man seen moving is taken to be — this many seconds of his pace on from where he was last seen, this far at most
    #[k(0.8)] pub lead_secs: f64,
    #[k(1.5)] pub lead_m: f64,
    /// the ducked man's point above the floor
    #[k(0.55)] pub low_y: f64,
    /// a free man's sum: seconds a round (this many a round) against the walk round at this pace
    #[k(0.25)] pub round_secs: f64,
    #[k(3.0)] pub walk_pace: f64,
    #[k(1.5)] pub walk_ask_secs: f64,
    /// the team's line when one of them starts (once per man per saySecs, floor-wide every 4 s); '' = said by nobody
    #[k(String::from("he's behind that. going through it."))] pub line: String,
    #[k(12.0)] pub say_secs: f64,
}
