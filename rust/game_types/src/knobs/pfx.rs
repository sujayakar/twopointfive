//! WHAT THINGS THROW WHEN THEY ARE HIT OR BREAK: the particles of a bullet hit, a stun charge, a kicked door, a shot lamp and a spent case. Counts are particles for
//! one event. The picture alone, but for one sound.
//! Sparks and chips give their width in meters and dust its radius at birth. All dust goes through the `dust` table on its way out.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PfxK {
    /// × the sparks, chips and dust of a hit, for every gun that isn't a pistol (a pistol = 1)
    #[k(1.4)] pub impact_scale_rifle: f64,
    /// a hit's sparks, and those of a thing breaking: width (m) · life (s) · half-angle of their cone (radians). The width is also the ring airfoil's sparks'
    #[k(0.003)] pub spark_size: f64,
    #[k(0.38)] pub spark_life: f64,
    #[k(0.9)] pub spark_spread: f64,
    /// a hit's chips: speed (m/s) · life (s) · width (m), where the surface's row names none · half-angle of their cone (radians)
    #[k(1.7)] pub chip_speed: f64,
    #[k(18.0)] pub chip_life: f64,
    #[k(0.007)] pub chip_size: f64,
    #[k(1.0)] pub chip_spread: f64,
    /// a hit's puff, and what a thing gives off as it breaks: speed (m/s; × 0.6 at a piece coming apart) · life (s) · radius (m)
    #[k(0.3)] pub dust_speed: f64,
    #[k(1.0)] pub dust_life: f64,
    #[k(0.1)] pub dust_size: f64,
    /// width (m) of a muzzle's unburnt powder sparks
    #[k(0.002)] pub muzzle_spark_size: f64,
    /// a stun charge's sparks: how many · width (m). Where a blast's own picture takes over (a door or wall charge, a thrown can), every fan is × these over what the
    ///  tables were set at, so 0 = no sparks either way
    #[k(160.0)] pub bang_sparks: f64,
    #[k(0.0025)] pub bang_spark_size: f64,
    /// its ring of dust along the floor: how many · radius at birth (m) · speed (m/s) · life (s) · radius of a puff (m)
    #[k(40.0)] pub bang_ring: f64,
    #[k(0.25)] pub bang_ring_radius: f64,
    #[k(3.2)] pub bang_ring_speed: f64,
    #[k(1.7)] pub bang_ring_life: f64,
    #[k(0.16)] pub bang_ring_size: f64,
    /// … and how many puffs over the can
    #[k(10.0)] pub bang_puff: f64,
    /// a boot on a door: puffs off the jamb · wood splinters off it
    #[k(9.0)] pub kick_dust: f64,
    #[k(14.0)] pub kick_splinters: f64,
    /// a lock kicked through: long splinters on ahead · splinters back past the boot · the strike plate's screws
    #[k(22.0)] pub lock_splinters: f64,
    #[k(6.0)] pub lock_splinters_back: f64,
    #[k(4.0)] pub lock_screws: f64,
    /// what a shot light rains down: a glowing tube or panel · any other lamp, and a vehicle's lens
    #[k(34.0)] pub tube_shards: f64,
    #[k(18.0)] pub bulb_shards: f64,
    /// sparks off the ballast of any shot light
    #[k(16.0)] pub tube_sparks: f64,
    /// the hard burst out of a shot breaker
    #[k(70.0)] pub breaker_sparks: f64,
    /// sparks where a ring airfoil meets a wall or a desk
    #[k(5.0)] pub clack_sparks: f64,
    /// seconds a spent case lasts
    #[k(45.0)] pub casing_life: f64,
    /// gain of the sound of a case landing (for the ear alone: no guard hears it)
    #[k(0.16)] pub tink_gain: f64,
    /// width (m) of the sparks where a tracer stops (the rest of them are the `tracers` table)
    #[k(0.0025)] pub ember_size: f64,
    pub solid_glass: PfxKSolidGlass,
}

/// THE CRACK A ROUND LEAVES IN SOLID GLASS (a car's window, which cracks and stays): the star of pale lines drawn as a mark. The picture alone.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct PfxKSolidGlass {
    /// its radius (m) by what fired: a pistol · any other gun. A hit on a breakable thing draws these at ± 15%
    #[k(0.085)] pub pistol: f64,
    #[k(0.11)] pub rifle: f64,
    /// its radius (m) for a gun that fires pellets, where a round stopped in the glass lays its own mark; a hit on a breakable thing takes `rifle`
    #[k(0.035)] pub pellet: f64,
    /// how strongly the crack's pale heart and bruise show, under 1: a mark's record holds its look as the whole number and this as the fraction, so 1 or more would make it another mark
    #[k(0.9)] pub strength: f64,
}

