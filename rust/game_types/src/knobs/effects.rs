//! WHAT A SHOT AND A STUN CAN PUT IN THE AIR, read by `tick/effects.rs` and `guns/combat/shots.rs`. Not the picture alone: the smoke sources are the game's own smoke
//! too, which men see through, wherever a source lets out at least `sense.smoke.least` in its life (a stun's stronger end jet and its slow cloud do; a muzzle's gas and a
//! spark's trail hide nobody and stay the picture's), and a flash is a light the men's light meter reads.
//! Every smoke source has the same numbers. Radius: m, a soft ball. Density: smoke let out a second at its middle. Temp: heat let out a second at its middle, which lifts
//! it (0 = none). Speed: m/s of the gas. Push: how fast the air there is dragged to that speed (a rate, 1/s). Expand: how fast it swells, shoving its neighbors apart
//! (1/s, 0 = none). Life: seconds. Attack: seconds to come up (it fades over the last quarter of its life). Jitter: how much its way and speed wobble each frame.
use crate::bytes::ToBytes;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsK {
    pub shot: EffectsKShot,
    pub muzzle: EffectsKMuzzle,
    pub card: EffectsKCard,
    pub bang: EffectsKBang,
    pub sparks: EffectsKSparks,
    pub trail: EffectsKTrail,
}

/// A SHOT, from `spawn_shot`: the gas (a jet down the bore, side ports, a wisp that rides the muzzle, a wisp that rides the ejection port), the flash and the powder
///  sparks. A shooter the fog of war hides from the air lets out no gas.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsKShot {
    /// m: the flash stands 0.4 × this out of the muzzle
    #[k(0.22)] pub standoff: f64,
    /// m, each [across, up, down the bore] off the muzzle: where the jet · a port · the wisp start
    #[k([0.0, 0.0, 0.22])] pub jet_off: V3,
    #[k([0.0, 0.0, 0.154])] pub port_off: V3,
    #[k([0.0, 0.0, 0.03])] pub wisp_off: V3,
    /// THE JET, a smoke source: radius · density · temp · speed. `jet_rise` (m/s) is added upward to its speed
    #[k(0.08)] pub jet_radius: f64,
    #[k(24.0)] pub jet_density: f64,
    #[k(120.0)] pub jet_temp: f64,
    #[k(10.5)] pub jet_speed: f64,
    #[k(0.2)] pub jet_rise: f64,
    /// push · expand · life · attack
    #[k(90.0)] pub jet_push: f64,
    #[k(24.0)] pub jet_expand: f64,
    #[k(0.08)] pub jet_life: f64,
    #[k(0.01)] pub jet_attack: f64,
    /// THE SIDE PORTS: how many (they alternate sides)
    #[k(2.0)] pub ports: f64,
    /// of the jet's density and temp, a port's
    #[k(0.4)] pub port_fraction: f64,
    /// radians out from the bore on either side, level. A port's gas also climbs at half its speed
    #[k(1.1)] pub port_angle: f64,
    /// × the jet's radius · speed · push · expand, a port's
    #[k(0.7)] pub port_radius_k: f64,
    #[k(0.55)] pub port_speed_k: f64,
    #[k(0.6)] pub port_push_k: f64,
    #[k(0.5)] pub port_expand_k: f64,
    /// THE WISP, a slow smoke source that rides the muzzle: radius · density · temp · how fast it rises (m/s, with 0.25 m/s down the bore) · push · life · attack
    #[k(0.042)] pub wisp_radius: f64,
    #[k(4.1)] pub wisp_density: f64,
    #[k(43.0)] pub wisp_temp: f64,
    #[k(0.6)] pub wisp_rise: f64,
    #[k(4.0)] pub wisp_push: f64,
    #[k(2.5)] pub wisp_life: f64,
    #[k(0.15)] pub wisp_attack: f64,
    /// THE EJECTION WISP, a smoke source that rides the port: radius · density · temp · life
    #[k(0.018)] pub eject_radius: f64,
    #[k(1.6)] pub eject_density: f64,
    #[k(2.5)] pub eject_temp: f64,
    #[k(0.7)] pub eject_life: f64,
    /// the jet's and the ports' jitter
    #[k(0.1)] pub jitter: f64,
    /// THE FLASH: color · peak strength (× the gun's `light_gain`) · seconds it lasts, falling off as (1 − t)^2.2 · range (m; a gun's own `light_range` goes first)
    #[k([1.0, 0.62, 0.26])] pub light_color: V3,
    #[k(90.0)] pub light_power: f64,
    #[k(0.09)] pub light_duration: f64,
    #[k(8.0)] pub light_range: f64,
    /// × the strength of a suppressed gun's flash
    #[k(0.45)] pub suppressed_gain: f64,
    /// THE POWDER: sparks out of the muzzle down the bore (0 = none) · speed (m/s, × 1 − 0.55 × a draw) · half-angle of their cone (radians) · life (s, ± 0.35 of it).
    ///  They also take the shooter's speed
    #[k(20.0)] pub spark_count: f64,
    #[k(31.0)] pub spark_speed: f64,
    #[k(0.25)] pub spark_cone: f64,
    #[k(0.06)] pub spark_life: f64,
}


/// A GUN'S MUZZLE: one row of `MuzzleFxK` for each `flame` a gun fires with (its `muzzle_fx` where it has no `flame` of its own).
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsKMuzzle {
    #[k(MuzzleFxK { card_ahead: 0.05, card_half: [0.018, 0.018, 0.045], card_emissive: [40.0, 22.0, 8.0], light_gain: 1.0, light_range: None })] pub pistol_suppressed: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.09, card_half: [0.03, 0.03, 0.08], card_emissive: [90.0, 50.0, 16.0], light_gain: 1.0, light_range: None })] pub pistol_bare: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.14, card_half: [0.04, 0.04, 0.12], card_emissive: [110.0, 60.0, 18.0], light_gain: 1.7, light_range: Some(10.0) })] pub carbine: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.09, card_half: [0.03, 0.03, 0.09], card_emissive: [60.0, 34.0, 12.0], light_gain: 1.2, light_range: None })] pub sc20k: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.07, card_half: [0.024, 0.024, 0.07], card_emissive: [44.0, 24.0, 8.0], light_gain: 1.0, light_range: None })] pub mk12: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.05, card_half: [0.016, 0.016, 0.05], card_emissive: [34.0, 19.0, 7.0], light_gain: 0.85, light_range: None })] pub mp7: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.11, card_half: [0.05, 0.022, 0.085], card_emissive: [95.0, 52.0, 16.0], light_gain: 1.45, light_range: Some(10.0) })] pub mk46: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.15, card_half: [0.06, 0.06, 0.11], card_emissive: [120.0, 58.0, 14.0], light_gain: 2.0, light_range: Some(11.0) })] pub m1014: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.045, card_half: [0.016, 0.016, 0.035], card_emissive: [26.0, 14.0, 5.0], light_gain: 0.7, light_range: None })] pub hk45: MuzzleFxK,
    #[k(MuzzleFxK { card_ahead: 0.08, card_half: [0.026, 0.042, 0.07], card_emissive: [95.0, 52.0, 16.0], light_gain: 1.1, light_range: None })] pub g18: MuzzleFxK,
}

/// one gun's flame
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct MuzzleFxK {
    /// m: the flame card's middle, down the bore from the muzzle
    #[k(0.05)] pub card_ahead: f64,
    /// m: the card's half size [across, up, along the bore]
    #[k([0.018, 0.018, 0.045])] pub card_half: V3,
    /// the card's glow (linear red, green, blue)
    #[k([40.0, 22.0, 8.0])] pub card_emissive: V3,
    /// × `shot.light_power`, this gun's flash
    #[k(1.0)] pub light_gain: f64,
    /// m: this gun's flash reaches this far (None = `shot.light_range`)
    #[k(None)] pub light_range: Option<f64>,
}


/// THE FLAME CARD, the glowing box at the muzzle that casts no shadow, for every gun.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsKCard {
    /// its color
    #[k([1.0, 0.8, 0.5])] pub albedo: V3,
    /// seconds it stays
    #[k(0.035)] pub ttl: f64,
}


/// A STUN'S SPARKS, a burst over the upper half of a sphere. The game's own (`bang.trail_count` of them, boxes of the `sparks` table) carry the smoke trails; the rest are the
///  `pfx` and `blast_look` particles, which read this table for their speed and life too.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsKSparks {
    /// m/s: the fastest (a pool spark × 0.45‥1, a particle × 0.45‥1 the same way)
    #[k(20.0)] pub speed: f64,
    /// lays the burst flatter: a throw's height is a draw to the power 1 + 3 × this
    #[k(0.55)] pub spread: f64,
    /// the share of what's left of a throw's height that turns straight up
    #[k(0.5)] pub lift: f64,
    /// seconds (a pool spark × 0.5‥1.2, a particle ± 0.35 of it)
    #[k(0.55)] pub life: f64,
}


/// THE SMOKE TRAIL A SPARK DRAWS, a source that rides its spark and goes when the spark dies, the first to be dropped when the solver has too many.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsKTrail {
    /// radius (m) · at least this many cells of the lattice wide · density (× `bang.density_gain`) · temp (× `bang.temp_gain`) · life · attack
    #[k(0.036)] pub radius: f64,
    #[k(1.0)] pub min_voxels: f64,
    #[k(8.6)] pub density: f64,
    #[k(45.0)] pub temp: f64,
    #[k(0.9)] pub life: f64,
    #[k(0.01)] pub attack: f64,
}



/// A STUN CAN COMING APART, from `spawn_burst`: two end jets along the can's axis, the body vents around it, a slow cloud, sparks with their trails and the flash. A
///  frag's burst (`pursuit/frag`) takes all of it too, with its own warmer flash at 0.7 of `light_power`. What the rules read of a stun charge, its fuse and its dazzle,
///  is in `crate::effects::bang()`.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct EffectsKBang {
    /// THE END JETS, smoke sources: radius · density · temp · expand · life · attack (density and temp × `density_gain`, `temp_gain` and the end's share of the lean)
    #[k(0.23)] pub radius: f64,
    #[k(6.3)] pub density: f64,
    #[k(48.0)] pub temp: f64,
    #[k(90.0)] pub expand: f64,
    #[k(0.12)] pub life: f64,
    #[k(0.02)] pub attack: f64,
    /// their push
    #[k(40.0)] pub end_push: f64,
    /// m/s: their speed (the vents' too, × `body_speed_k`)
    #[k(7.0)] pub vent_speed: f64,
    /// m: half the can's length. The end jets start 0.6‥1.4 × this off the middle, the body vents anywhere along it
    #[k(0.1)] pub half_length: f64,
    /// how much more one end lets out than the other: the lean is a draw of ± this (0 = even). The can's two lobes of sparks lean the same way
    #[k(0.45)] pub asymmetry: f64,
    /// radians: the can's axis is a draw of ± this out of level
    #[k(0.25)] pub tilt: f64,
    /// how much the burst's draws scatter: of a jet's strength, way, speed, width and life (0 = every jet as written)
    #[k(0.6)] pub jitter: f64,
    /// THE BODY VENTS, around the can's middle: how many · each one's share of the density and temp (× 0.5‥1.5) · push · × `radius` · × `vent_speed` · × `expand`
    #[k(5.0)] pub body_vents: f64,
    #[k(0.25)] pub body_fraction: f64,
    #[k(30.0)] pub body_push: f64,
    #[k(0.7)] pub body_radius_k: f64,
    #[k(0.6)] pub body_speed_k: f64,
    #[k(0.5)] pub body_expand_k: f64,
    /// THE SLOW CLOUD left over the can, a smoke source: radius · density · temp (× `density_gain`, `temp_gain`) · how fast it rises (m/s) · push · life · attack
    #[k(0.24)] pub wisp_radius: f64,
    #[k(1.2)] pub wisp_density: f64,
    #[k(10.5)] pub wisp_temp: f64,
    #[k(0.6)] pub wisp_rise: f64,
    #[k(5.0)] pub wisp_push: f64,
    #[k(3.0)] pub wisp_life: f64,
    #[k(0.3)] pub wisp_attack: f64,
    /// the jets' and the vents' jitter
    #[k(0.08)] pub frame_jitter: f64,
    /// × the density and the temp of all a burst's smoke: the jets, the vents, the cloud and the trails
    #[k(1.0)] pub density_gain: f64,
    #[k(1.0)] pub temp_gain: f64,
    /// THE FLASH: color · peak strength · seconds it lasts, falling off as (1 − t)^2.2 · range (m) · the lamp's radius (m) · how far over the can it stands (m).
    ///  `blast_look` finds a wall charge's flash by this very `light_duration` and `light_lift`
    #[k([1.0, 0.98, 0.95])] pub light_color: V3,
    #[k(1600.0)] pub light_power: f64,
    #[k(0.14)] pub light_duration: f64,
    #[k(14.0)] pub light_range: f64,
    #[k(0.15)] pub light_radius: f64,
    #[k(0.3)] pub light_lift: f64,
    /// how many sparks of the game's own it throws, each with a smoke trail if there is room · how many of the game's emitters a trail must leave free
    #[k(12.0)] pub trail_count: f64,
    #[k(2.0)] pub trail_reserve: f64,
}
