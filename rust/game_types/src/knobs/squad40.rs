//! The numbers of squad40.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct Squad40K {
    /// a shot is one whose round goes off within this of the spot (m) …
    #[k(1.2)] pub near_m: f64,
    /// … having flown this much more than it arms at (m: HE40.armM is counted along its path, and he is not a range table)
    #[k(0.5)] pub arm_slack: f64,
    /// nobody of ours inside the burst's own outer radius and this (m: FRAG.woundM — it goes off where it lands, so there is no run to allow for as a thrown frag's)
    #[k(0.5)] pub reach_slack: f64,
    /// the tube: this far ahead of him toward the spot (m), this high (m) — or `headDrop` under his head, if that is lower (a man down on a knee)
    #[k(0.55)] pub muzzle_ahead: f64,
    #[k(1.35)] pub muzzle_up: f64,
    #[k(0.3)] pub head_drop: f64,
    /// his gun is ON the spot within this (rad): such a man fires at once; another is turned to it first …
    #[k(0.2)] pub aim_rad: f64,
    /// … for this long at most (s), his gun up at least this long before the round goes (s), and stays on it this long after it has gone (s)
    #[k(2.0)] pub aim_secs: f64,
    #[k(0.35)] pub raise_secs: f64,
    #[k(0.4)] pub after_secs: f64,
    /// the squad block names the shot this long (s)
    #[k(2.5)] pub shown_secs: f64,
}
