//! The numbers of fixture_break: how a broken ceiling panel or lamp looks, its tray swinging down by one end and tearing out, its diffuser and glass on the floor, the torn
//! lead's sparks and glow. No rule reads them, but the pieces are real boxes on the frame's list and the fog's records.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct FixtureBreakK {
    /// switch. Off: a broken fixture shows none of this, and any pieces out are let go
    #[k(true)] pub on: bool,
    /// most panels with their boxes out at once, hanging or lying: past it a broken panel only sparks
    #[k(12.0)] pub max_panels: f64,
    /// degrees below the ceiling the tray settles at while it hangs: least · most, by fixture
    #[k([55.0, 65.0])] pub rest_deg: [f64; 2],
    /// its swing: the spring's pull to that angle (per s²) · the damping (per s)
    #[k(30.0)] pub swing_k: f64,
    #[k(2.2)] pub swing_c: f64,
    /// rad/s: how fast the swing starts when it breaks, × 0.85 to 1.15
    #[k(3.4)] pub kick: f64,
    /// s it hangs before the other end tears out and it comes down, × 0.85 to 1.15
    #[k(1.9)] pub hang_secs: f64,
    /// m: the least it lies to one side of where it hung (and at least 0.6 of its width) · rad: the most its long axis turns as it comes to lie
    #[k(0.35)] pub slide: f64,
    #[k(0.5)] pub skew: f64,
    /// the torn lead's pops: s after the break of each, in order, × 0.85 to 1.15 · the sparks each throws · the glow each adds
    #[k([0.62, 1.0, 1.5, 2.2, 3.1])] pub pops: [f64; 5],
    #[k([30.0, 22.0, 13.0, 9.0, 6.0])] pub pop_sparks: [f64; 5],
    #[k([1.0, 0.8, 0.55, 0.4, 0.3])] pub pop_gain: [f64; 5],
    /// s: the time constant a pop's glow fades with · how bright the lead is at a glow of 1
    #[k(0.07)] pub pop_tau: f64,
    #[k(12.0)] pub glow_tip: f64,
    pub spark: FixtureBreakKSpark,
    /// the way the sparks leave, before it is made a unit vector: out past the free end · down while it hangs · up once it lies.
    ///  A lamp with no panel: half the first, and 1 down
    #[k(0.9)] pub spark_aside: f64,
    #[k(0.55)] pub spark_down: f64,
    #[k(0.5)] pub spark_up: f64,
    /// m: the torn lead's half size: along the fixture · up · across
    #[k([0.018, 0.008, 0.012])] pub lead: V3,
    /// after the pops the lead spits, from `spit_from` s to `spit_secs` s after the break; past that it is dark. Each 0.11 s has odds `spit_rate` × e^(−s / `spit_tau`)
    ///  of a spit, which adds a glow of `spit_gain` × 0.5 to 1.5, and two in five of them throw 4 sparks
    #[k(3.6)] pub spit_from: f64,
    #[k(45.0)] pub spit_secs: f64,
    #[k(16.0)] pub spit_tau: f64,
    #[k(0.22)] pub spit_rate: f64,
    #[k(0.2)] pub spit_gain: f64,
    /// flakes of the tube's glass, half in each of two lots along the fixture · m: their width · s after the break they fall (a panel only)
    #[k(26.0)] pub glass: f64,
    #[k(0.013)] pub glass_size: f64,
    #[k(0.15)] pub glass_at: f64,
    /// loose pieces of glass that fall and lie, beside the two diffuser sheets
    #[k(5.0)] pub shards: f64,
    /// colors: the tray · the cover on its back · its face · the tube left in it · the torn lead's soot · the diffuser sheets that fall · the glass pieces
    #[k([0.4, 0.41, 0.42])] pub tray: V3,
    #[k([0.55, 0.55, 0.56])] pub cover: V3,
    #[k([0.78, 0.78, 0.75])] pub face: V3,
    #[k([0.8, 0.8, 0.78])] pub tube: V3,
    #[k([0.06, 0.055, 0.05])] pub soot: V3,
    #[k([0.8, 0.83, 0.86])] pub diffuser: V3,
    #[k([0.85, 0.9, 0.9])] pub shard: V3,
}

/// The sparks a pop or a spit throws.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct FixtureBreakKSpark {
    /// m/s · how much of that is jitter (0 to 1)
    #[k(6.0)] pub speed: f64,
    #[k(0.6)] pub speed_jit: f64,
    /// rad: the cone's half angle
    #[k(1.15)] pub spread: f64,
    /// s · how much of that is jitter (0 to 1)
    #[k(0.42)] pub life: f64,
    #[k(0.5)] pub life_jit: f64,
    /// m: a spark's streak thickness
    #[k(0.002)] pub size: f64,
}
