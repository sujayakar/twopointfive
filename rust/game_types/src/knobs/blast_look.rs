//! WHAT A BLOWN CHARGE THROWS: the sparks, chips and dust of a wall charge, a door charge and a thrown stun can, all thrown once, in the frame it goes off. The picture
//! alone: no rule reads a particle. Only `on` reaches further, because a set charge's boxes are on the frame's list, which the fog of war and the frame's budget count,
//! and it warms a wall charge's flash, which the light meter reads, keeping the meter's sum the same to the bit.
//! Every fan has the same numbers. `count`: particles (rounded). `speed`: m/s, each × 1 − `jit` × a draw of 0‥1. `cone`: the half-angle of its cone (radians). `tilt`:
//! how far it leans up, the height of its direction per unit of way out (below 0 is down). `life`: seconds, ± `life_jit` of it. `size`: a spark's or a chip's width (m),
//! ± `size_jit` of it.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookK {
    /// false: no charge is dressed and no burst is thrown here. A set wall charge draws no frame, lamp or firing line, its flash stays the stun's white, and every burst
    ///  is the plain record of `pfx`. (It also takes `chaos_of` over 0.)
    #[k(true)] pub on: bool,
    /// false: a thrown stun can throws the plain record of `pfx`, not `stun_sparks`
    #[k(true)] pub stun: bool,
    /// false: a door charge throws the plain record of `pfx`, not `door_sparks`
    #[k(true)] pub door: bool,
    /// what the spark fans below were tuned against
    pub reference: BlastLookKReference,
    /// a wall charge's fans
    pub wall: BlastLookKWall,
    /// a thrown stun can's fans
    pub stun_sparks: BlastLookKStunSparks,
    /// a door charge's fans
    pub door_sparks: BlastLookKDoorSparks,
}

/// What every spark fan is scaled by: its count × `pfx.bang_sparks` / `sparks`, its speed × `effects.sparks.speed` / `speed`, its life × `effects.sparks.life` / `life`
///  and its size × `pfx.bang_spark_size` / `size`. So those four turn all the sparks up together; with the numbers as they stand all four come to 1. A wall's chips and
///  dust are not scaled.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKReference {
    #[k(160.0)] pub sparks: f64,
    #[k(20.0)] pub speed: f64,
    #[k(0.55)] pub life: f64,
    #[k(0.0025)] pub size: f64,
}


/// A WALL CHARGE. Each fan is thrown out of the wall 0.1 m off the face at the charge's height (`wallcharge.height`), born within 6 cm of there. In `count`, `speed` and
///  `cone` the first number is for the way through a wall that gave, out of its far face, and the second for back off the near face. A wall that held throws one way only,
///  back off the near face, by the first numbers. They go out in the order jet, core, dust, shower, chips, embers.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWall {
    pub jet: BlastLookKWallJet,
    pub core: BlastLookKWallCore,
    pub shower: BlastLookKWallShower,
    pub embers: BlastLookKWallEmbers,
    pub chips: BlastLookKWallChips,
    pub dust: BlastLookKWallDust,
}

/// the fast thin sparks, gone in a blink
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWallJet {
    #[k([30.0, 14.0])] pub count: [f64; 2],
    #[k([21.0, 15.0])] pub speed: [f64; 2],
    #[k(0.65)] pub jit: f64,
    #[k([0.36, 0.5])] pub cone: [f64; 2],
    #[k(-0.2)] pub tilt: f64,
    #[k(0.24)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.003)] pub size: f64,
    #[k(0.5)] pub size_jit: f64,
}

/// the slow fat sparks that hang at the hole
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWallCore {
    #[k([14.0, 7.0])] pub count: [f64; 2],
    #[k([1.6, 1.1])] pub speed: [f64; 2],
    #[k(0.6)] pub jit: f64,
    #[k([1.3, 1.3])] pub cone: [f64; 2],
    #[k(0.1)] pub tilt: f64,
    #[k(0.95)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.04)] pub size: f64,
    #[k(0.5)] pub size_jit: f64,
}

/// the sparks that fall away: thrown slower than the jet and leaning down
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWallShower {
    #[k([18.0, 10.0])] pub count: [f64; 2],
    #[k([10.0, 8.0])] pub speed: [f64; 2],
    #[k(0.7)] pub jit: f64,
    #[k([0.85, 0.95])] pub cone: [f64; 2],
    #[k(-0.3)] pub tilt: f64,
    #[k(0.55)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.004)] pub size: f64,
    #[k(0.5)] pub size_jit: f64,
}

/// the slow small sparks that last
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWallEmbers {
    #[k([18.0, 9.0])] pub count: [f64; 2],
    #[k([6.0, 4.5])] pub speed: [f64; 2],
    #[k(0.85)] pub jit: f64,
    #[k([1.1, 1.2])] pub cone: [f64; 2],
    #[k(-0.1)] pub tilt: f64,
    #[k(1.2)] pub life: f64,
    #[k(0.8)] pub life_jit: f64,
    #[k(0.0035)] pub size: f64,
    #[k(0.8)] pub size_jit: f64,
}

/// the wall's own chips, as debris, in the wall's color paled by nearly half
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWallChips {
    #[k([48.0, 22.0])] pub count: [f64; 2],
    #[k([7.5, 4.5])] pub speed: [f64; 2],
    #[k(0.75)] pub jit: f64,
    #[k([0.7, 0.85])] pub cone: [f64; 2],
    #[k(0.12)] pub tilt: f64,
    #[k(20.0)] pub life: f64,
    #[k(0.3)] pub life_jit: f64,
    #[k(0.04)] pub size: f64,
    #[k(0.7)] pub size_jit: f64,
    /// a chip's length ÷ its width
    #[k(1.7)] pub aspect: f64,
}

/// the wall's dust, through the `dust` table on its way out, in the chips' color × 0.85. Its `size` is a puff's radius at birth (m)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKWallDust {
    #[k([44.0, 24.0])] pub count: [f64; 2],
    #[k([5.5, 3.6])] pub speed: [f64; 2],
    #[k(0.6)] pub jit: f64,
    #[k([0.55, 0.7])] pub cone: [f64; 2],
    #[k(0.05)] pub tilt: f64,
    #[k(1.7)] pub life: f64,
    #[k(0.3)] pub life_jit: f64,
    #[k(0.3)] pub size: f64,
    #[k(0.35)] pub size_jit: f64,
}


/// A THROWN STUN CAN, all from the can: two lobes, then a fan, a fountain and embers.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKStunSparks {
    pub lobes: BlastLookKStunSparksLobes,
    pub fan: BlastLookKStunSparksFan,
    pub fountain: BlastLookKStunSparksFountain,
    pub embers: BlastLookKStunSparksEmbers,
}

/// the two unequal lobes along the can's own axis, the two ways its smoke jets go (`effects.bang`)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKStunSparksLobes {
    /// sparks a lobe, × 1 ± the burst's lean (`effects.bang.asymmetry`, never under a quarter), shared 0.3 · 0.4 · 0.3 over its three fans
    #[k(34.0)] pub count: f64,
    /// m/s, × 0.85 · 1 · 0.9 over the three fans; the rest as in the head of this file
    #[k(17.0)] pub speed: f64,
    #[k(0.6)] pub jit: f64,
    #[k(0.17)] pub cone: f64,
    /// radians between the three fans of a lobe, around the up axis
    #[k(0.3)] pub fan: f64,
    /// how far a lobe leans up: the height of its direction per unit of way out
    #[k(0.05)] pub rise: f64,
    #[k(0.3)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.0025)] pub size: f64,
    #[k(0.4)] pub size_jit: f64,
}

/// the low ring thrown out all around from a circle 3 cm wide
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKStunSparksFan {
    #[k(40.0)] pub count: f64,
    #[k(10.0)] pub speed: f64,
    #[k(0.7)] pub jit: f64,
    /// how far it leans up: the height of its direction per unit of way out
    #[k(0.12)] pub lift: f64,
    /// how much a spark's direction scatters: a draw of ± half of this on each axis, per unit of way out
    #[k(0.5)] pub spread: f64,
    #[k(0.5)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.003)] pub size: f64,
    #[k(0.4)] pub size_jit: f64,
}

/// the small fountain, a burst over the upper half of a sphere. `spread` lays it flatter (a throw's height is a draw to the power 1 + 3 × spread) and `lift` is the share
///  of what's left that turns straight up
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKStunSparksFountain {
    #[k(22.0)] pub count: f64,
    #[k(6.0)] pub speed: f64,
    #[k(0.5)] pub jit: f64,
    #[k(0.55)] pub spread: f64,
    #[k(0.5)] pub lift: f64,
    #[k(0.6)] pub life: f64,
    #[k(0.4)] pub life_jit: f64,
    #[k(0.003)] pub size: f64,
    #[k(0.4)] pub size_jit: f64,
}

/// the embers: slow small sparks that last, thrown the way the fountain is
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKStunSparksEmbers {
    #[k(18.0)] pub count: f64,
    #[k(3.0)] pub speed: f64,
    #[k(0.6)] pub jit: f64,
    #[k(0.8)] pub spread: f64,
    #[k(0.2)] pub lift: f64,
    #[k(1.5)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.0045)] pub size: f64,
    #[k(0.4)] pub size_jit: f64,
}


/// A DOOR CHARGE: a jet and a skitter into the room from ahead of the leaf it threw in, a share back at the squad's wall, then the stun's own fountain and embers from
///  the squad's side.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKDoorSparks {
    /// the flat fast jet into the room
    pub jet: BlastLookKDoorSparksJet,
    /// the slower sparks into the room, leaning down to skitter along the floor
    pub skitter: BlastLookKDoorSparksSkitter,
    /// the share thrown back at the squad's wall
    pub back: BlastLookKDoorSparksBack,
    /// sparks in the stun's fountain here, in place of its `count`
    #[k(16.0)] pub fountain: f64,
    /// m: the room's share starts this far ahead of the leaf
    #[k(0.2)] pub ahead: f64,
    /// and this many fixed steps (1/60 s) of the leaf's flight further on, so the sparks don't start inside a leaf that is moving
    #[k(1.5)] pub lead: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKDoorSparksJet {
    #[k(56.0)] pub count: f64,
    #[k(19.0)] pub speed: f64,
    #[k(0.65)] pub jit: f64,
    #[k(0.4)] pub cone: f64,
    #[k(-0.08)] pub tilt: f64,
    #[k(0.45)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.003)] pub size: f64,
    #[k(0.5)] pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKDoorSparksSkitter {
    #[k(44.0)] pub count: f64,
    #[k(10.0)] pub speed: f64,
    #[k(0.7)] pub jit: f64,
    #[k(0.9)] pub cone: f64,
    #[k(-0.3)] pub tilt: f64,
    #[k(0.55)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.004)] pub size: f64,
    #[k(0.5)] pub size_jit: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastLookKDoorSparksBack {
    #[k(22.0)] pub count: f64,
    #[k(12.0)] pub speed: f64,
    #[k(0.65)] pub jit: f64,
    #[k(0.5)] pub cone: f64,
    #[k(-0.2)] pub tilt: f64,
    #[k(0.3)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    #[k(0.003)] pub size: f64,
    #[k(0.5)] pub size_jit: f64,
}

