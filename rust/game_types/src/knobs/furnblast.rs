//! The numbers of furnblast.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::vec::Vec;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct FurnK {
    /// every reach below × this, by what went off
    #[k({ let mut t = JsMap::new(); t.set(crate::furnblast::BlastSource::Wall, 1.0); t.set(crate::furnblast::BlastSource::Door, 0.7); t.set(crate::furnblast::BlastSource::Frag, 0.6); t })] pub k: JsMap<crate::furnblast::BlastSource, f64>,
    /// how hard the pieces of a thing right by it are flung (× their speed; 1 = a hard shove, about 5 m/s at the charge): a wall charge reads as a blast
    #[k({ let mut t = JsMap::new(); t.set(crate::furnblast::BlastSource::Wall, 1.6); t.set(crate::furnblast::BlastSource::Door, 1.2); t.set(crate::furnblast::BlastSource::Frag, 1.2); t })] pub fling: JsMap<crate::furnblast::BlastSource, f64>,
    /// fixed furniture: split and thrown inside shatterM, comes apart / goes over inside apartM (metres from the blast to the nearest point of it)
    #[k(1.6)] pub shatter_m: f64,
    #[k(4.0)] pub apart_m: f64,
    /// pushable furniture: thrown inside throwM (m/s at the charge, falling off linearly), comes apart inside propBreakM
    #[k(5.5)] pub throw_m: f64,
    #[k(6.0)] pub throw_v: f64,
    #[k(1.5)] pub prop_break_m: f64,
    /// rigid chunks one blast may make (the pool's own cap still rules under it)
    #[k(36.0)] pub chunks_per_blast: f64,
    /// a hole this blast opened lets it through: the second origin stands this far past the wall
    #[k(0.45)] pub mouth_m: f64,
    /// two calls in one frame within this of each other are one blast (the wall charge's borrowed bang)
    #[k(1.5)] pub merge_m: f64,
    /// A HOLE A BLAST OPENS CAN BE WALKED THROUGH (mouth(); `on`: raised by the page — src/host/profile.ts pageSwitches, ?holeclear=0 — off here and on the frozen
    ///  stand, TPS_HOLECLEAR=1): what stands in a body's way in either mouth of the hole — the stretch of the wall open from the floor up, `depth` m out from each
    ///  face — goes with the blast, WHOEVER OWNS IT: a cover piece the wall shielded from the burst (the stretch of it in the way), a piece of furniture too heavy
    ///  for the bands above, a thing under a roof that neither engine lists (a counter in white, shelving on legs, a board on the wall), and what rests on them.
    ///  Never: anything taller than `top` or heavier than `vol` m³, a structure paint other than white, a post (a door frame's jamb), a light's or a fixture's box
    ///  or what carries one, the mission's own thing, the breaker. `chunks` = rigid pieces a blast's mouths may make on top of the blast's own (out of pieces a
    ///  thing still goes, as chips). A thing the engines do not list goes WHOLE (a long counter for a short hole). off = whatever stood by the wall stands in it.
    ///  `snow` (raised by the page — pageSwitches, ?holesnow=0 — off here and on the frozen stand, TPS_HOLESNOW=1; acts only with `on`): LYING SNOW IN A MOUTH GOES
    ///  TOO — a bank against the wall's foot on a snow level (a box in a snow paint, which reads as the ground to a blast and so as structure here; the pass keeps
    ///  every bank over 0.2 m clear of its doors because it stops feet, and a charge makes a door anywhere). The maintainer, 10-15, of a wall charge on the pass:
    ///  "couldn't path through a hole from a wall breach" — two banks at the foot stopped him, the walk grid and the charge's own 'is it a hole' ray ("it held").
    ///  off = a bank stands in the hole it was blown under. PICTURE: the snow field (render/snowField.ts) is baked at the level's start and still draws its hill.
    pub mouth: FurnKMouth,
    /// WHAT HUNG ON THE WALL UNDER A CHARGE: cut into slabs `len` m long (at most `max`) by `strips`; the
    ///  slabs within `underM` of the charge along the wall are driven through the hole at `through` m/s (lo, hi), the rest torn off into the room at `back` m/s
    ///  and along the wall at up to `side` m/s, `up` m/s off the floor, tumbling (`spin`, as throwPiece's); a long slab counts as under the charge only with its middle within `holeM` of it; `chunks` = rigid pieces one thing may make (past it,
    ///  splinters); `chips` = splinters thrown back into the room and through, at `chipV` m/s.
    pub hung: FurnKHung,
    /// what counts as a piece of furniture standing on the floor (top / long side in m, volume in m³; heavier than apartMaxVol only breaks in the shatter band) …
    pub root: FurnKRoot,
    /// … and as a thing standing on / fixed to one
    pub rider: FurnKRider,
    /// boxes nearer than this are joined
    #[k(0.04)] pub touch: f64,
    /// a run of joined furniture longer than this (a row of desks) breaks part by part, not as one
    #[k(3.6)] pub row_m: f64,
}

/// A HOLE A BLAST OPENS CAN BE WALKED THROUGH (mouth(); `on`: raised by the page — src/host/profile.ts pageSwitches, ?holeclear=0 — off here and on the frozen
///  stand, TPS_HOLECLEAR=1): what stands in a body's way in either mouth of the hole — the stretch of the wall open from the floor up, `depth` m out from each
///  face — goes with the blast, WHOEVER OWNS IT: a cover piece the wall shielded from the burst (the stretch of it in the way), a piece of furniture too heavy
///  for the bands above, a thing under a roof that neither engine lists (a counter in white, shelving on legs, a board on the wall), and what rests on them.
///  Never: anything taller than `top` or heavier than `vol` m³, a structure paint other than white, a post (a door frame's jamb), a light's or a fixture's box
///  or what carries one, the mission's own thing, the breaker. `chunks` = rigid pieces a blast's mouths may make on top of the blast's own (out of pieces a
///  thing still goes, as chips). A thing the engines do not list goes WHOLE (a long counter for a short hole). off = whatever stood by the wall stands in it.
///  `snow` (raised by the page — pageSwitches, ?holesnow=0 — off here and on the frozen stand, TPS_HOLESNOW=1; acts only with `on`): LYING SNOW IN A MOUTH GOES
///  TOO — a bank against the wall's foot on a snow level (a box in a snow paint, which reads as the ground to a blast and so as structure here; the pass keeps
///  every bank over 0.2 m clear of its doors because it stops feet, and a charge makes a door anywhere). The maintainer, 10-15, of a wall charge on the pass:
///  "couldn't path through a hole from a wall breach" — two banks at the foot stopped him, the walk grid and the charge's own 'is it a hole' ray ("it held").
///  off = a bank stands in the hole it was blown under. PICTURE: the snow field (render/snowField.ts) is baked at the level's start and still draws its hill.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FurnKMouth {
    #[k(0.9)] pub depth: f64,
    #[k(2.3)] pub top: f64,
    #[k(3.0)] pub vol: f64,
    #[k(12.0)] pub chunks: f64,
    #[k(0.08)] pub base_thick: f64,
}

/// WHAT HUNG ON THE WALL UNDER A CHARGE: cut into slabs `len` m long (at most `max`) by `strips`; the
///  slabs within `underM` of the charge along the wall are driven through the hole at `through` m/s (lo, hi), the rest torn off into the room at `back` m/s
///  and along the wall at up to `side` m/s, `up` m/s off the floor, tumbling (`spin`, as throwPiece's); a long slab counts as under the charge only with its middle within `holeM` of it; `chunks` = rigid pieces one thing may make (past it,
///  splinters); `chips` = splinters thrown back into the room and through, at `chipV` m/s.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct FurnKHung {
    #[k(0.32)] pub len: f64,
    #[k(4.0)] pub max: f64,
    #[k(2.0)] pub strips: f64,
    #[k(0.3)] pub under_m: f64,
    #[k(alloc::vec![7.0, 11.0])] pub through: Vec<f64>,
    #[k(alloc::vec![3.5, 7.0])] pub back: Vec<f64>,
    #[k(4.5)] pub side: f64,
    #[k(alloc::vec![1.6, 3.0])] pub up: Vec<f64>,
    #[k(24.0)] pub spin: f64,
    #[k(8.0)] pub chunks: f64,
    #[k(alloc::vec![40.0, 24.0])] pub chips: Vec<f64>,
    #[k(alloc::vec![5.0, 7.0])] pub chip_v: Vec<f64>,
    #[k(0.5)] pub hole_m: f64,
}

/// what counts as a piece of furniture standing on the floor (top / long side in m, volume in m³; heavier than apartMaxVol only breaks in the shatter band) …
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FurnKRoot {
    #[k(0.3)] pub min_top: f64,
    #[k(2.0)] pub max_top: f64,
    #[k(2.6)] pub max_long: f64,
    #[k(0.7)] pub max_vol: f64,
    #[k(0.6)] pub apart_max_vol: f64,
}

/// … and as a thing standing on / fixed to one
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FurnKRider {
    #[k(2.3)] pub max_top: f64,
    #[k(0.3)] pub max_vol: f64,
}
