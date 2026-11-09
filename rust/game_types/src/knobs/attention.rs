//! The numbers of attention.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the switch and every number of it (UNTUNED by play: nobody has sneaked past a held man on a real page yet)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct AttentionK {
    #[k(true)] pub on: bool,
    /// the half-angle (deg, in plan) of what he is LOOKING AT, about the line from his eye to the thing; outside it a man's sight rate is × `sideK` and a footfall at his ear × `earK`
    #[k(25.0)] pub focus_deg: f64,
    #[k(0.3)] pub side_k: f64,
    #[k(0.5)] pub ear_k: f64,
    /// the awareness a man glimpsed OUTSIDE the focus has to build, in sum, for the hold to break and his head to come round
    #[k(0.2)] pub break_gain: f64,
    /// only a man under this awareness can have his attention taken (over it something has him properly worried: a slam, glass, a glimpse)
    #[k(0.6)] pub calm_aw: f64,
    /// a knock made to be looked at (GameEvent.lure: a ring's clack) that reaches a calm man lifts his suspicion to this: over the 'huh?' line, so he goes to look at it
    #[k(0.34)] pub draw_aw: f64,
    /// his suspicion is held at least this high while his attention is on a thing he is walking to or looking at (WHISTLE.walkFloor's number and reason)
    #[k(0.32)] pub floor: f64,
    /// seconds AT the thing, by what it was: a whistle he answered, a knock / clack / thud, a door, a light gone out in his view
    #[k(4.0)] pub whistle_secs: f64,
    #[k(5.0)] pub noise_secs: f64,
    #[k(4.0)] pub door_secs: f64,
    #[k(5.0)] pub light_secs: f64,
    /// how long before the end he says he is done (the tell: the eye is still narrow under it), how far his chest swings while he looks at the thing (rad; the
    ///  old look-about swung 1.1), and the longest walk a hold lasts through (s)
    #[k(0.8)] pub tell_secs: f64,
    #[k(0.3)] pub dwell_sweep: f64,
    #[k(15.0)] pub walk_cap: f64,
    /// every taking inside `memorySecs` of the last is × `wearK` as long; the `spent`-th inside it is a search, not a look
    #[k(0.6)] pub wear_k: f64,
    #[k(60.0)] pub memory_secs: f64,
    #[k(3.0)] pub spent: f64,
    /// a small thing inside this of the last one that reached him is the same happening (a canister's bounces, a carton's thud and its scrape, a thing dragged):
    ///  it moves the place his attention is on and is no new taking; and how long a man a glimpse or a step took off a lent hold (a screen) stays off it
    #[k(2.5)] pub same_secs: f64,
    #[k(6.0)] pub lent_back_secs: f64,
    /// the floor marks (the thread and the ring), and the ring's radius (m)
    #[k(true)] pub marks: bool,
    #[k(0.42)] pub ring_m: f64,
    /// the thread runs the WAY HE WILL WALK to the place (attentionWay: round the walls, through the doors), not straight through them; `wayReask` = the
    ///  seconds an answer of the nav grid's is kept. Picture only (on on every stand; `?straightthread` in the page = the straight thread it was).
    #[k(true)] pub way: bool,
    #[k(0.5)] pub way_reask: f64,
}
