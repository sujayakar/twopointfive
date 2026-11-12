//! The numbers of blast_feel: how a wall charge going off feels, in the camera's kick, the report's sounds, the fireball, the floor dust, the pushes and the
//! ceiling's dust. The picture and the sound alone: no rule reads them.
use crate::bytes::ToBytes;
use alloc::vec::Vec;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct BlastFeelK {
    /// switch. Off, or with destruction's chaos at 0: a wall charge plays the can's report, with no kick and none of the rest
    #[k(true)] pub on: bool,
    /// the camera's snap away from the blast: × all of it (0 = none) · m it moves the look-at, right at the blast
    #[k(1.0)] pub kick: f64,
    #[k(0.45)] pub kick_m: f64,
    /// s a kick lasts, fading out over its last 0.08 · s: the time constant it dies down with · Hz it rings at
    #[k(0.45)] pub kick_secs: f64,
    #[k(0.14)] pub kick_tau: f64,
    #[k(7.0)] pub kick_hz: f64,
    /// m on plan from the blast where the kick reaches nothing, falling as (1 − d / this)^1.5
    #[k(28.0)] pub kick_reach: f64,
    pub sound: BlastFeelKSound,
    pub ring: BlastFeelKRing,
    /// what lies loose is thrown, on each face of the wall, following the ring: rows of s after the blast · m of reach · m/s at its center
    ///  (× each kind of particle's own gain), falling to 0 at its reach
    #[k(alloc::vec![[0.0, 3.0, 7.0], [0.075, 5.5, 4.5], [0.15, 8.0, 3.0]])] pub push: Vec<[f64; 3]>,
    /// how much of a push goes up, against 1 straight out
    #[k(0.5)] pub push_up: f64,
    pub ceil: BlastFeelKCeil,
    pub fire: BlastFeelKFire,
}

/// The report's sounds, played at the blast the moment it goes off.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastFeelKSound {
    /// gain of the crack, × 0.25 to 1 as distance dulls it · of the thump · of the patter of debris, × 0.4 to 1 as distance dulls it
    #[k(1.0)] pub crack: f64,
    #[k(1.0)] pub thump: f64,
    #[k(0.6)] pub debris: f64,
    /// how much louder all three are at range, at most 1 + this · m from which that starts · m more until it is all there
    #[k(0.8)] pub carry: f64,
    #[k(3.0)] pub carry_from: f64,
    #[k(7.0)] pub carry_over: f64,
    /// m/s: the speed of sound · s: the most it may hold the report back. (read by nothing yet: worked into a local nobody uses, and the report plays at once)
    #[k(343.0)] pub c: f64,
    #[k(0.25)] pub late_max: f64,
    /// m at which distance has dulled it halfway: 1 / (1 + (d / this)²)
    #[k(14.0)] pub dull_m: f64,
    /// gain of the second ring in the ears, right at the blast · m from the blast where it falls to 0. None for a player who is down
    #[k(0.3)] pub ring: f64,
    #[k(7.0)] pub ring_m: f64,
}

/// The low ring of floor dust racing out from the wall on each face: stages of puffs, each stage one radius, in a fan of sectors. A sector a wall stops stays stopped.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastFeelKRing {
    /// stages · sectors in the fan (at most 32: the stopped ones are a bit mask) · rad: the fan's half width, either side of straight out
    #[k(9.0)] pub stages: f64,
    #[k(13.0)] pub sectors: f64,
    #[k(1.4)] pub arc: f64,
    /// m: the first stage's radius · m more per stage · s between stages, from the frame after the blast
    #[k(0.5)] pub r0: f64,
    #[k(0.55)] pub step: f64,
    #[k(0.025)] pub every: f64,
    /// m: the puffs' height
    #[k(0.14)] pub y: f64,
    /// m/s, from all of this at the first stage to about half at the last · s · m, from all of this to about 1.5 × at the last · the dust's color
    #[k(5.0)] pub speed: f64,
    #[k(0.5)] pub life: f64,
    #[k(0.3)] pub size: f64,
    #[k([0.74, 0.72, 0.68])] pub color: V3,
}

/// The dust and crumbs shaken off the ceiling a beat after the wave gets under it.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct BlastFeelKCeil {
    /// spots tried over the room on each face (none where a wall stands in the way, or under open sky)
    #[k(13.0)] pub spots: f64,
    /// m: the farthest spot from the face (the nearest is 0.7)
    #[k(6.5)] pub reach: f64,
    /// a spot shakes this many s after the blast + its distance / the wave's speed in m/s
    #[k(0.1)] pub after: f64,
    #[k(14.0)] pub speed: f64,
    /// dust puffs per spot · crumbs per spot · the color of both
    #[k(3.0)] pub puffs: f64,
    #[k(3.0)] pub crumbs: f64,
    #[k([0.82, 0.81, 0.78])] pub color: V3,
}

/// The fireball in the hole. A pair of counts or speeds is by way out: through the hole into the room, or off the face when the wall held ·
///  back off the face when the wall gave.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct BlastFeelKFire {
    pub core: BlastFeelKFireCore,
    pub roll: BlastFeelKFireRoll,
    #[k(alloc::vec![BlastFeelKFireSootItem { after: 2.0, count: alloc::vec![12.0, 6.0], speed: 2.8, size: 0.3, life: 1.4 }, BlastFeelKFireSootItem { after: 5.0, count: alloc::vec![9.0, 5.0], speed: 2.2, size: 0.38, life: 1.7 }, BlastFeelKFireSootItem { after: 8.0, count: alloc::vec![8.0, 4.0], speed: 1.7, size: 0.44, life: 2.0 }, BlastFeelKFireSootItem { after: 13.0, count: alloc::vec![6.0, 3.0], speed: 1.1, size: 0.54, life: 2.6 }])] pub soot: Vec<BlastFeelKFireSootItem>,
    /// the color of the soot's rows above
    #[k([0.05, 0.044, 0.038])] pub soot_color: V3,
    pub chunks: BlastFeelKFireChunks,
}

/// The first fat sparks, at the blast's own frame: count by way · m/s by way · s · m.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct BlastFeelKFireCore {
    #[k([30.0, 16.0])] pub count: [f64; 2],
    #[k([4.2, 3.0])] pub speed: [f64; 2],
    #[k(0.3)] pub life: f64,
    #[k(0.11)] pub size: f64,
}

/// The second roll of slower, larger sparks: count by way · m/s by way · s · m.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct BlastFeelKFireRoll {
    #[k([14.0, 8.0])] pub count: [f64; 2],
    #[k([2.2, 1.6])] pub speed: [f64; 2],
    #[k(0.5)] pub life: f64,
    #[k(0.15)] pub size: f64,
    /// frames (at 60 a second) after the blast
    #[k(2.0)] pub after: f64,
}

/// The dark chunks it throws: count by way · m/s by way · s · m · their color.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct BlastFeelKFireChunks {
    #[k([28.0, 14.0])] pub count: [f64; 2],
    #[k([8.0, 5.5])] pub speed: [f64; 2],
    #[k(7.0)] pub life: f64,
    #[k(0.05)] pub size: f64,
    #[k([0.05, 0.046, 0.042])] pub color: V3,
}

/// One round of the dark smoke that follows the fire.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct BlastFeelKFireSootItem {
    /// frames (at 60 a second) after the blast
    #[k(0.0)] pub after: f64,
    /// puffs by way
    #[k(Vec::new())] pub count: Vec<f64>,
    /// m/s, and 0.6 of it again straight out of the wall
    #[k(0.0)] pub speed: f64,
    /// m: a puff's birth radius
    #[k(0.0)] pub size: f64,
    /// s
    #[k(0.0)] pub life: f64,
}
