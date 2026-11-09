//! The numbers of lanes.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct LanesK {
    /// how long a lane is remembered after it last cost them — × the men lost in it, up to three
    #[k(150.0)] pub secs: f64,
    /// how far down its line a gun's lane runs, m · the half angle of the wedge about that line · floor this near the line is in it whatever the angle, m
    #[k(32.0)] pub reach_m: f64,
    #[k(22.0)] pub half_deg: f64,
    #[k(1.1)] pub width_m: f64,
    /// …and everything that gun has a line of fire to within this of it, whichever way: men who know where a gun is know what it looks at (0 = the line alone), m
    #[k(12.0)] pub field_m: f64,
    /// floor this near the gun itself is not part of its lane (he is on top of it: that is a fight), m
    #[k(1.0)] pub own_m: f64,
    /// fire that landed nearer its gun than this opens no lane, m
    #[k(2.0)] pub min_m: f64,
    /// a round passing within `nearM` of a living man is worth this much of a man dropped (1 = a lane)
    #[k(0.5)] pub round_k: f64,
    #[k(1.5)] pub near_m: f64,
    /// the beat before a man who saw it acts on it himself · the beat he needs to get it said · how far a mate sees it happen, m · how near a man coming on the bodies knows what they mean, m
    #[k(0.35)] pub react_secs: f64,
    #[k(0.9)] pub call_secs: f64,
    #[k(30.0)] pub see_m: f64,
    #[k(12.0)] pub body_m: f64,
    /// the price of a lane's floor in a walk (× plain floor)
    #[k(14.0)] pub dear: f64,
    /// how far along his way a walking man looks for a lane, m
    #[k(3.5)] pub ahead_m: f64,
    /// an enemy he has seen this close inside the last second is a fight already: no gate, no backing out, m (never less than the distance the room fight itself
    ///  breaks a walk off at for a man in sight)
    #[k(4.5)] pub close_m: f64,
    /// rounds down it inside this many seconds: a man standing in it off his post gets out of the line
    #[k(8.0)] pub hot_secs: f64,
    /// GO ROUND: on · how many of the men held at its mouths at most · the breath before anybody is sent · the longest way round, m · no sooner again, s
    #[k(2.0)] pub round_men: f64,
    #[k(2.5)] pub think_secs: f64,
    #[k(80.0)] pub round_max_m: f64,
    #[k(25.0)] pub round_gap: f64,
    /// …and he may first show himself to the guns only this far round from the lane's own line, deg, and only for the last this much of his way, m
    #[k(55.0)] pub round_side_deg: f64,
    #[k(8.0)] pub round_show_m: f64,
    /// a place to stop is out of sight of the guns as they stood AND of a gun a stride to either side of each (they shift about), m
    #[k(0.9)] pub shift_m: f64,
    /// THE CHARGE: on · a throw's reach to the guns, m · a bang this near the guns has them blind, m · how long the lane is theirs to push down behind the bang, s · who goes (this near the guns), m · how much faster than a chase they go, m/s · no sooner again, s
    #[k(12.0)] pub stun_m: f64,
    #[k(5.0)] pub bang_m: f64,
    #[k(6.0)] pub push_secs: f64,
    #[k(12.0)] pub push_m: f64,
    #[k(0.8)] pub push_fast: f64,
    #[k(20.0)] pub stun_gap: f64,
    /// THE GUNS HAVE MOVED: once every gun that made a lane has cost them from further than `movedM` from it (or is down), it is forgotten `movedSecs` later, m · s
    #[k(15.0)] pub moved_m: f64,
    #[k(20.0)] pub moved_secs: f64,
    #[k(10.0)] pub peek_quiet: f64,
    #[k(15.0)] pub peek_gap: f64,
    #[k(60.0)] pub peek_max: f64,
    #[k(45.0)] pub peek_lost: f64,
    #[k(5.0)] pub peek_reach_m: f64,
    #[k(10.0)] pub peek_near_m: f64,
    #[k(0.8)] pub peek_look: f64,
    #[k(45.0)] pub peek_sure: f64,
    #[k(60.0)] pub peek_keep: f64,
    /// …he comes up to the corner at this pace, m/s, and looks round it with his head this far out past his feet, m (so the guns never get his chest for it)
    #[k(1.3)] pub peek_pace: f64,
    #[k(0.35)] pub peek_lean: f64,
    /// HELD AT TWO ANGLES, WITH FIRE (off = a way round must hide its man to its last `roundShowM`, and the men at a lane's mouth only keep their guns on it):
    ///  on · a way that comes at the guns from another side but would show its man for longer than that (up to `pinShowM`) is taken as far as where the guns
    ///  would first see him: he STOPS there and says so — one more man held on the lane, from its other side. While a man is set there, the men held on the lane, either side, take turns stepping
    ///  to their corners — gun already on where the fire came from — to put `pinRounds` down it at that place, one every `pinEvery` s, from where the muzzle first clears the corner, and get back within `pinSnap` s of that first line (no longer in the guns' sight than a look is — a man who stayed out a second and a half to fire was lost on 5 of 14 bursts): a burst no
    ///  sooner than `pinGap` s after the last, `pinMax` of them, then `pinRest` s of quiet · a round goes wide by this (the round's own spread number: 0.24 = up
    ///  to 1.2 m off the place at 10 m on each axis, on top of where his bore sits inside `pinAimDeg` — fire AT A PLACE to keep heads down, far wider than any aimed round of his; it can still hit a man who stands on it) ·
    ///  his gun must be within this of the place before a round leaves, deg · his pace out, m/s
    #[k(3.0)] pub pin_rounds: f64,
    #[k(0.12)] pub pin_every: f64,
    #[k(0.6)] pub pin_snap: f64,
    #[k(4.0)] pub pin_gap: f64,
    #[k(4.0)] pub pin_max: f64,
    #[k(20.0)] pub pin_rest: f64,
    #[k(0.24)] pub pin_spread: f64,
    #[k(9.0)] pub pin_aim_deg: f64,
    #[k(2.0)] pub pin_pace: f64,
    /// the longest end of a way round the guns may see, with this on, m · nobody fires from a corner nearer the guns than this, m
    #[k(12.0)] pub pin_show_m: f64,
    #[k(7.0)] pub pin_min_m: f64,
    /// WHERE HE STOPS (off = planted where the gate caught him, as it was): never in a door's opening or sweep, never where a gun of the lane a stride to either
    ///  side of where it fired from has him — he backs to cover first, and a held man since moved onto such floor backs out again; the point he watches is never
    ///  nearer than `lookM`; a man whose own look cleared the lane is not planted on it afterwards
    #[k(1.5)] pub look_m: f64,
    #[k(4.0)] pub frag_think: f64,
    #[k(30.0)] pub frag_gap: f64,
    #[k(1.5)] pub frag_look: f64,
    #[k(0.9)] pub frag_step_m: f64,
    /// lanes a side keeps at once
    #[k(6.0)] pub max: f64,
}
