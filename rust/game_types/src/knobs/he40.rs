//! The numbers of he40.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// Every number of the round's flight and of what a dud does (the blast's numbers are frag.ts FRAG's: one blast).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct He40K {
    /// muzzle speed (m/s: the 40×46 mm low-velocity round), gravity, air drag (/s), contact radius (m), seconds of flight before it is simply gone (a round sent
    ///  up and out over the lot), substeps a frame, and the passes of the elevation solve
    #[k(76.0)] pub speed: f64,
    #[k(9.81)] pub gravity: f64,
    #[k(0.12)] pub drag: f64,
    #[k(0.03)] pub radius: f64,
    #[k(2.5)] pub life: f64,
    #[k(4.0)] pub substeps: f64,
    #[k(2.0)] pub solve_iters: f64,
    /// metres of its own path before the fuze is armed: anything it meets sooner gets a dud
    #[k(8.0)] pub arm_m: f64,
    /// how far off the surface it struck the burst is judged from (m): the blast's point must sit in the room, not in the plaster
    #[k(0.15)] pub standoff: f64,
    /// hearing ('prop' events, as the ring's): the tube's thump AT THE SHOOTER (not gunfire: a hollow pop, but a bigger one than the ring's), a dud slapping
    ///  into a wall (who 0: a noise nobody can put a man behind) (into a man the one noise is his own: flight::struck)
    #[k(0.7)] pub launch_noise: f64,
    #[k(0.45)] pub dud_noise: f64,
    /// a dud: N·s into the chair it strikes, the share of its normal / sliding speed it keeps coming off a surface, and the most it carries on as an item
    #[k(9.0)] pub prop_j: f64,
    #[k(0.15)] pub restitution: f64,
    #[k(0.3)] pub keep_tangent: f64,
    #[k(6.0)] pub spent_speed_max: f64,
}
