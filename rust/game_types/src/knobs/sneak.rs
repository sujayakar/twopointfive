//! The numbers of sneak.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SneakK {
    /// THE switch: off = the follower exactly as he was
    #[k(false)] pub on: bool,
    /// his legs: the least he is asked for while following (m/s; the loud file's is the walk, 1.5) · the most while low · across an exposed stretch
    #[k(0.5)] pub min_pace: f64,
    #[k(1.45)] pub low_pace: f64,
    #[k(1.6)] pub cross_pace: f64,
    /// a crouched footfall's level while he sneaks (silent's walk is 0.2, Sam's crouch 0.125; 0 = silent's own) — "only the player is truly amazing" at it
    #[k(0.14)] pub step: f64,
    /// he walks Sam's own trail (off: his place is the build's, by its own route) · metres of it kept · metres further back than the loud file he closes to
    #[k(60.0)] pub trail_m: f64,
    #[k(0.3)] pub back: f64,
    /// he looks before he steps (off: he never waits) · how far up the trail he reads · this lit (0‥1 on the meter's curve) is a lit step
    #[k(2.4)] pub ahead_m: f64,
    #[k(0.1)] pub lit_at: f64,
    /// LIT TO WHOM (his partner has to follow him across an open yard at night, and on a GPU the open night shadow reads over `lit_at` on the meter, where the
    /// software renderer reads near 0). A step
    /// at or over `lit_at` with a line from a defender is SEEN only where THAT defender's eye would gain on a man there: the guards' own arithmetic (`eye_rate`:
    /// detect rate × light^1.5 × the distance term × the crouch) over `seen_rate`, which is the guards' own floor — a rate at or under 0.05 /s moves no
    /// awareness. Dim ground far from an eye is ground he may stand on and cross; dim ground near one is not. Read
    /// so: the follower's step and standing place (read), the stalk's watched cells and its priced way (the walker's map), a door frame's funnel (an eye
    /// behind a frame that would not gain on a man at the spot is no eye on it). NOT read so: a remembered stretch with no eye on it now (the flat number
    /// stands), the dark foe, the seen walk, the spotted rule. His kit card's shape is in the rate, as the guards have it.
    #[k(0.05)] pub seen_rate: f64,
    /// a defender's cone is read this many degrees wider (a glance travels) · a step whose chest is inside this of an unwary defender's eye, in his near
    ///  field, is exposed, lit or not (the eye's own point-blank rule is 1.8 m, eye to chest)
    #[k(25.0)] pub cone_deg: f64,
    #[k(2.1)] pub near_m: f64,
    /// an exposed stretch longer than this is not crossed while anybody has a line to it (he waits for the light, not the glance)
    #[k(4.5)] pub cross_m: f64,
    /// … for this long (s), then he goes at the next moment nobody looks (0 = for as long as it takes). A stretch somebody IS looking at he never walks into.
    #[k(8.0)] pub patience: f64,
    /// a crossing that has not made its far side in this long is let go of (s)
    #[k(8.0)] pub cross_secs: f64,
    /// a looked-at stretch is crossed all the same when the most stirred looker's awareness after it — his now + `margin` × what the eye's own arithmetic
    ///  adds over the crossing — stays within this (suspicion starts at 0.3)
    #[k(0.12)] pub budget: f64,
    #[k(1.6)] pub margin: f64,
    /// a man he saw looking at a lit stretch, and can no longer see (a door swung to between them): taken to be looking still for this long (s)
    #[k(12.0)] pub memory: f64,
    /// his words over the net when he waits (and after `nagSecs` of waiting, and when he comes on) — at most one every `cueGap` seconds
    #[k(7.0)] pub nag_secs: f64,
    #[k(12.0)] pub cue_gap: f64,
    /// Sam stopped: he stands in the nearest unexposed spot along the trail (back within `standM` of his place, or on towards Sam but never inside
    ///  `standNear` of him) rather than on a lit, seen one
    #[k(4.0)] pub stand_m: f64,
    #[k(0.8)] pub stand_near: f64,
    /// … and with no such step on the trail (a corridor that runs straight at an open door: every step Sam took is in the doorway's line), a spot OFF it,
    ///  within `sideM` of where he is, that is not exposed: he steps aside out of the door's line — never into a door frame, never onto Sam or a mate.
    ///  A lit spot a defender would have a line to THROUGH AN OPEN DOORWAY but for its leaf is no such spot: a leaf swings (the door's FUNNEL: the lines from
    ///  his eye through the frame's whole opening)
    #[k(2.6)] pub side_m: f64,
    /// … and where he will STOP on his way to such a spot is out of sight too: he pulls up within a quarter metre of it, on the side he comes from, so a
    ///  spot centimetres past the edge of a door's line left him standing in the line (the points `sideStop` and half of it short of the spot, towards
    ///  him, are read as well). 0 = only the spot itself is read, as it was.
    #[k(0.25)] pub side_stop: f64,
    #[k(1.8)] pub door_m: f64,
    #[k(0.4)] pub door_pad: f64,
    #[k(1.0)] pub door_settle: f64,
    /// caught where he stands (seen by his own bones and meter, and looked at or stood still): to the nearest dark step — not again inside `hideGap` s
    #[k(2.5)] pub hide_gap: f64,
    #[k(2.5)] pub noise_walk_m: f64,
    /// how often his read of the way is redone (game seconds)
    #[k(0.12)] pub plan_secs: f64,
    /// the partner's sneaking skill, 1 = this one. Under 1 he reads less of the way ahead and a narrower glance (0 = he never waits). The page's call.
    #[k(1.0)] pub skill: f64,
}
