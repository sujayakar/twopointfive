//! The numbers of chronicle.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// ------------------------------------------------------------ the tables the knobs carry
/// The stealth score's price list (points off a hundred; every term only ever takes away, so nothing a run does can raise its score — the probes hold it to that).
///  In the Chaos Theory spirit: being SEEN is the expensive thing (more so once the floor already knows), an alarm and each step the floor goes up cost on their own,
///  a body found costs, a kill costs ten times a knockout, rounds put near men and fixtures shot out cost a little, getting hit / going down / leaning on a
///  checkpoint cost, and dawdling past par costs a point a minute.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ScoreK {
    /// per sighting episode; + seenUp for one that happened with the floor already heightened / locked down
    #[k(12.0)] pub seen: f64,
    #[k(4.0)] pub seen_up: f64,
    /// per alarm episode (somebody went to alert while nobody was)
    #[k(6.0)] pub alarm: f64,
    /// the floor going up a level
    #[k(5.0)] pub heightened: f64,
    #[k(8.0)] pub lockdown: f64,
    /// per body found, dead or out cold
    #[k(6.0)] pub body: f64,
    /// per man: shot dead / put down breathing
    #[k(10.0)] pub kill: f64,
    #[k(1.0)] pub ko: f64,
    /// per round into or within SHOT_NEAR_M of a living man, capped
    #[k(2.0)] pub shot_near: f64,
    #[k(12.0)] pub shot_near_cap: f64,
    /// per fixture shot out, capped (the OCP is free: it is the quiet tool)
    #[k(1.0)] pub light: f64,
    #[k(4.0)] pub light_cap: f64,
    /// a round in the vest; going down; each checkpoint load
    #[k(4.0)] pub hit: f64,
    #[k(25.0)] pub down: f64,
    #[k(3.0)] pub load: f64,
    /// the civilians (civilians.ts): a report that reached the men (told to one, rung in, control's call after she got out); one put out cold — ten knockouts' worth of a guard's; one KILLED — four kills' worth (and with tune.civilianKillFails the run is over anyway); being merely SEEN by one costs nothing by itself: it is what she then does that costs
    #[k(15.0)] pub civ_reported: f64,
    #[k(10.0)] pub civ_ko: f64,
    #[k(40.0)] pub civ_killed: f64,
    /// per time a camera / a tripwire called him in (they know SOMETHING moved — being heard's worth), capped; the floor going up on a camera's word costs as any rise does
    #[k(2.0)] pub device: f64,
    #[k(8.0)] pub device_cap: f64,
    /// a point per whole minute over par, capped
    #[k(1200.0)] pub par_secs: f64,
    #[k(1.0)] pub per_min: f64,
    #[k(10.0)] pub time_cap: f64,
}

/// the badges' two numbers: fixtures shot out for 'lights out', and the clock 'speedrun' beats (a straight run — lobby, corridor, the picked server door, the rack,
///  out through the break room — is about two minutes flat out)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BadgeK {
    #[k(3.0)] pub lights_out_n: f64,
    #[k(240.0)] pub speedrun_secs: f64,
}
