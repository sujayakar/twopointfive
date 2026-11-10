//! The numbers of wallreact.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallreactK {
    /// ---- noticing a breach
    /// a man sees a breach he has a line to inside this, in a cone `fovDeg` wide about where he faces (the guards' own eye cone is 116°)
    #[k(18.0)] pub see_m: f64,
    #[k(116.0)] pub fov_deg: f64,
    /// path length (the sound model's) a wall going is heard at: by a blast · by fire · a collapse
    #[k(26.0)] pub hear_blast_m: f64,
    #[k(10.0)] pub hear_fire_m: f64,
    #[k(14.0)] pub hear_fall_m: f64,
    /// a defender who learned of one by his own senses tells his side's men within this much sound path, this many seconds later
    #[k(12.0)] pub tell_m: f64,
    #[k(1.2)] pub tell_secs: f64,
    /// ---- cover gone
    /// a man this near what changed asks whether he is still covered
    #[k(2.6)] pub near_m: f64,
    /// … and is asked again for this long while he cannot move (blind, held, a man at arm's length)
    #[k(5.0)] pub retry_secs: f64,
    #[k(6.0)] pub gone_max: f64,
    /// a mark is not moved further than this
    #[k(3.5)] pub step_m: f64,
    /// a holding man with nothing nearer runs this far for a corner
    #[k(9.0)] pub run_m: f64,
    /// the far side of a hole beside a man counts as where trouble is from this far beyond it
    #[k(2.5)] pub beyond_m: f64,
    /// ---- watching a breach (defenders)
    /// evidence: hearing or seeing it open · per second with an enemy in sight through it · per second with rounds coming through it · forgotten per second
    #[k(1.6)] pub ev_open: f64,
    #[k(1.5)] pub ev_seen: f64,
    #[k(1.0)] pub ev_fire: f64,
    #[k(0.12)] pub forget: f64,
    /// what a breach is worth with no evidence at all: one a man fits through · a port · less per mate already on it · kept once on it
    #[k(0.45)] pub hole_base: f64,
    #[k(0.2)] pub port_base: f64,
    #[k(0.35)] pub crowd: f64,
    #[k(0.15)] pub sticky: f64,
    /// … and for this long after a man learns of one it outweighs any door: he looks at what just opened
    #[k(3.0)] pub fresh_secs: f64,
    #[k(3.0)] pub fresh: f64,
    /// … inside this, with a line to it
    #[k(16.0)] pub watch_m: f64,
}
