//! THE STREAK OF A TRACER ROUND: the thin glowing box a gun with a tracer row fires, its head light and the ember where it stops. The picture alone; the round itself
//! has already been traced.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct TracersK {
    /// m/s the head travels at
    #[k(280.0)] pub speed: f64,
    /// seconds a streak lasts at least: on a short shot the head slows down to take this long
    #[k(0.042)] pub min_life: f64,
    /// seconds of travel one streak shows: its length is the speed × this, at least 5 cm and never past the muzzle
    #[k(0.011)] pub shutter: f64,
    /// m across the box
    #[k(0.012)] pub thickness: f64,
    /// × the tracer's color, for its glow
    #[k(60.0)] pub emissive: f64,
    /// m: a round that goes less than this shows no streak
    #[k(0.5)] pub min_path: f64,
    /// the head light, riding 5 cm behind the head: intensity · range (m) · radius of the emitter (m)
    #[k(4.0)] pub light_intensity: f64,
    #[k(4.0)] pub light_range: f64,
    #[k(0.06)] pub light_radius: f64,
    /// where it stops in something solid (not a man): how many sparks, 0 = none · their speed (m/s) · half-angle of their cone (radians) · their life (s)
    #[k(3.0)] pub ember_sparks: f64,
    #[k(5.0)] pub ember_speed: f64,
    #[k(0.9)] pub ember_cone: f64,
    #[k(0.3)] pub ember_life: f64,
    /// … and a brief glow there: intensity, 0 = none · range (m) · seconds
    #[k(2.5)] pub ember_intensity: f64,
    #[k(2.5)] pub ember_range: f64,
    #[k(0.06)] pub ember_ttl: f64,
    /// streaks in the air at once: a whole number, at least 1, read when the pool is made. One more takes the oldest's slot
    #[k(24.0)] pub max_live: f64,
    /// how many of them have a head light at once; a streak fired past that flies dark
    #[k(6.0)] pub max_lights: f64,
}
