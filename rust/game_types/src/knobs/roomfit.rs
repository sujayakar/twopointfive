//! The numbers of roomfit.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// The knobs: no panel rows; the table is turned by name (`?knob.roomfit.<field>=<number>`).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct RoomfitK {
    /// the floor the men may take: within `spread0 + spreadPer × men` metres of the point (two men 2.2, four 3.2)
    #[k(1.2)] pub spread0: f64,
    #[k(0.5)] pub spread_per: f64,
    /// no two marks nearer than `apart` · under `loose` costs `looseK` a metre
    #[k(1.1)] pub apart: f64,
    #[k(1.7)] pub loose: f64,
    #[k(0.8)] pub loose_k: f64,
    /// a metre further from the point than `nearFree` costs this
    #[k(0.35)] pub near_k: f64,
    #[k(0.4)] pub near_free: f64,
    /// a wall or furniture at his back · and at a shoulder as well
    #[k(0.45)] pub wall_k: f64,
    #[k(0.15)] pub corner_k: f64,
    /// THE FUNNEL of a way in: `funnelM` deep, the way's own width plus `funnelPad` a side, widening `funnelFlare` a metre · what standing in one costs (his own
    ///  way's: `ownK` of it; a way onto known floor: `knownFunnelK` of it; behind a shut leaf: `shutFunnelK` of it)
    #[k(4.5)] pub funnel_m: f64,
    #[k(0.3)] pub funnel_pad: f64,
    #[k(0.12)] pub funnel_flare: f64,
    #[k(1.6)] pub funnel_k: f64,
    #[k(0.7)] pub own_k: f64,
    #[k(0.4)] pub known_funnel_k: f64,
    #[k(0.6)] pub shut_funnel_k: f64,
    /// the angle he takes on his own way, off its centre line: full marks between these (degrees), none square on or past `angMax`
    #[k(0.5)] pub ang_k: f64,
    #[k(18.0)] pub ang_lo: f64,
    #[k(60.0)] pub ang_hi: f64,
    #[k(85.0)] pub ang_max: f64,
    /// nearer his own way than this costs `closeK` a metre
    #[k(1.6)] pub close_m: f64,
    #[k(0.8)] pub close_k: f64,
    /// a mate within this of the line from a man to what he watches
    #[k(0.7)] pub lane_m: f64,
    #[k(4.0)] pub lane_k: f64,
    /// the weights: a way further than `reach` is nobody's · the nearer the heavier (`nearM`: where it has halved) · known floor beyond · a shut leaf · a locked
    ///  one · how much the way Sam pointed adds in front and takes off behind · a second gun on an uncleared way
    #[k(10.0)] pub reach: f64,
    #[k(4.0)] pub near_m: f64,
    #[k(0.35)] pub known_k: f64,
    #[k(0.7)] pub shut_k: f64,
    #[k(0.3)] pub locked_k: f64,
    #[k(0.25)] pub front_k: f64,
    #[k(0.3)] pub second_k: f64,
    /// open floor running on at least `sectorM` with no way's line across it is a thing to watch: little at that length (a room's own far wall), `sectorK` of a
    ///  way by a hall's (7.5 m more) · no nearer a way's bearing than `sectorDeg` — a way that FACES back up the run by at least `sectorAcross` (the cosine; 0 =
    ///  any way near the bearing, as it first shipped: a hall with doors down its sides then had nobody on its length) · the gun is put `sectorLook` metres down it at most ·
    ///  a run past `sectorFarM` in a room the team has stood in counts `sectorFarK`, not `knownK`: standing in a hall's near end says little of its far one (`knownK`'s 0.35 = as it first shipped)
    #[k(5.0)] pub sector_m: f64,
    #[k(0.55)] pub sector_k: f64,
    #[k(35.0)] pub sector_deg: f64,
    #[k(0.5)] pub sector_across: f64,
    #[k(6.0)] pub sector_look: f64,
    #[k(10.0)] pub sector_far_m: f64,
    #[k(0.7)] pub sector_far_k: f64,
    /// he takes up his watch this near his mark
    #[k(1.5)] pub watch_in_m: f64,
    /// a standing man looks again this often (s) · and changes only for something this much heavier
    #[k(1.0)] pub again_secs: f64,
    #[k(1.3)] pub sticky: f64,
}
