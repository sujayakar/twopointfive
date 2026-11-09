//! The numbers of close.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CloseK {
    #[k(0.2)] pub rmb_tap_secs: f64,
    #[k(0.15)] pub rmb_loud: f64,
    #[k(1.2)] pub rmb_wait_secs: f64,
    /// m: …and only under a neck this high off his own floor (a man on his feet: 1.44; one cowering or sat on the floor is under it and keeps the level reach)
    #[k(1.15)] pub hands_low_neck_m: f64,
    /// m: Sam is in the mode this near a man (centre to centre, nothing solid between) · and out of it again past this
    #[k(3.0)] pub mode_m: f64,
    #[k(3.6)] pub mode_out_m: f64,
    /// the light on Sam (his meter, 0‥1) under which the near-sight floor is nothing · from which it is the build's whole · m: inside this of a man who has
    ///  him in his cone the floor is whole whatever the light (he walks into you)
    #[k(0.1)] pub dark_at: f64,
    #[k(0.3)] pub lit_at: f64,
    #[k(0.55)] pub touch_m: f64,
    /// m: reach of each window, centre to centre (behind = the build's own 1.05)
    pub reach: CloseKReach,
    /// degrees off his facing where his FRONT ends and a side begins (the side runs from there to the rear arc, which is the build's own: cos < −0.25 = past
    ///  104.5°)
    #[k(50.0)] pub side_from_deg: f64,
    /// m: where Sam stands for a strike from a side / the front, off the man's centre (the takedown from behind keeps its own 0.55) · over a chair: he closes to this
    ///  on his own bearing, as near as the chair lets him (his collision), so the blow has an arm's length to land over
    #[k(0.6)] pub strike_dist: f64,
    #[k(0.72)] pub seat_dist: f64,
    /// hearing level of the fall, by the way in ('prop' events at the pair: the takedown from behind is the build's 0.3) · added when he was alert (he shouts)
    pub noise: CloseKNoise,
    #[k(0.15)] pub alert_noise: f64,
}

/// m: reach of each window, centre to centre (behind = the build's own 1.05)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CloseKReach {
    #[k(1.05)] pub behind: f64,
    #[k(0.95)] pub side: f64,
    #[k(0.85)] pub front: f64,
    #[k(1.2)] pub seat: f64,
    #[k(0.95)] pub seat_front: f64,
}

/// hearing level of the fall, by the way in ('prop' events at the pair: the takedown from behind is the build's 0.3) · added when he was alert (he shouts)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CloseKNoise {
    #[k(0.3)] pub behind: f64,
    #[k(0.38)] pub side: f64,
    #[k(0.5)] pub front: f64,
    #[k(0.3)] pub seat: f64,
}
