//! The numbers of sprintdoor.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct SprintdoorK {
    /// the share of his speed he keeps through a shoulder bash taken square on (under about 0.65 it reads as glue)
    #[k(0.75)] pub keep: f64,
    /// rad/s the leaf leaves his shoulder at (it was 7.5: at a 4.7 m/s run the leaf rode his chest; at 9 it leads him)
    #[k(9.0)] pub bash_vel: f64,
    /// rad his body turns the latch-side shoulder into the leaf, and the seconds it takes to come back square (0 = no turn)
    #[k(0.45)] pub lean: f64,
    #[k(0.25)] pub lean_secs: f64,
    /// the face it opens toward: the share of the sprint he is checked to, for how long, the rad/s the leaf comes round at and how loud that is (a bash is 1)
    #[k(0.3)] pub rip_pace: f64,
    #[k(0.3)] pub rip_secs: f64,
    #[k(6.0)] pub rip_vel: f64,
    #[k(0.55)] pub rip_level: f64,
    /// how hard the boot throws the slab, 0‥1 of DOORBLAST.speed (a charge set on the leaf is 0.55) · the m/s INTO the leaf under which a run is no kick ·
    ///  seconds into the one-second kick at which his legs are his own again (1 = the whole kick, as a kick at a locked door)
    #[k(0.25)] pub kick_k: f64,
    #[k(2.5)] pub kick_into: f64,
    #[k(0.8)] pub kick_free: f64,
}
