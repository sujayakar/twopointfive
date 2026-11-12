//! WHAT A ROUND KNOCKS OUT OF WHAT IT HITS, by the surface it hit. The picture alone: `impact_fx` throws it as particles, and no rule reads a number here.
//! Speeds, lives and the spread of the three kinds are in the `pfx` table.
use crate::bytes::ToBytes;
use crate::js::JsMap;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct ImpactK {
    /// one row for every surface a thing can be made of; a surface with no row brings the game down at the first shot at it.
    /// Metal never throws chips, whatever its row says, so its `chips`, `chip_tint` and `chip_size` are read by nothing. Solid glass (a car's window) throws 3 chips in place of the row's.
    #[k({ let mut t = JsMap::new(); t.set(String::from("metal"), ImpactKRow { sparks: 16.0, chips: 2.0, dust: 0.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.08, 0.08, 0.09]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: Some(6.0), chip_size: Some(0.005) }); t.set(String::from("plastic"), ImpactKRow { sparks: 6.0, chips: 5.0, dust: 1.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: None, chip_size: None }); t.set(String::from("glass"), ImpactKRow { sparks: 4.0, chips: 9.0, dust: 0.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.1, 0.11, 0.12]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: None, chip_size: Some(0.006) }); t.set(String::from("wood"), ImpactKRow { sparks: 0.0, chips: 12.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.66, 0.52, 0.34]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.62, 0.52, 0.4]), spark_speed: None, chip_size: None }); t.set(String::from("cardboard"), ImpactKRow { sparks: 0.0, chips: 8.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.6, 0.5, 0.36]), spark_speed: None, chip_size: None }); t.set(String::from("drywall"), ImpactKRow { sparks: 0.0, chips: 10.0, dust: 6.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Pale, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.8, 0.78, 0.74]), spark_speed: None, chip_size: None }); t.set(String::from("concrete"), ImpactKRow { sparks: 5.0, chips: 10.0, dust: 5.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.45, 0.44, 0.42]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.55, 0.54, 0.52]), spark_speed: Some(5.0), chip_size: None }); t.set(String::from("carpet"), ImpactKRow { sparks: 0.0, chips: 4.0, dust: 4.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::Albedo, spark_speed: None, chip_size: None }); t.set(String::from("fabric"), ImpactKRow { sparks: 2.0, chips: 5.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::Albedo, spark_speed: None, chip_size: None }); t.set(String::from("foliage"), ImpactKRow { sparks: 0.0, chips: 10.0, dust: 2.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.14, 0.32, 0.1]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.3, 0.22, 0.14]), spark_speed: None, chip_size: None }); t })] pub on: JsMap<String, ImpactKRow>,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ImpactKRow {
    /// how many of each a pistol round throws (× `pfx.impact_scale_rifle` for any other gun, rounded): sparks · chips · puffs of dust. 0 = none of that kind
    #[k(0.0)] pub sparks: f64,
    #[k(0.0)] pub chips: f64,
    #[k(0.0)] pub dust: f64,
    /// the chips' color: `Albedo` = the color of what was hit · `Pale` = that, 55% of the way to plaster white · `List` = this color
    #[k(crate::particle_fx::ImpactRecipeChipTint::Albedo)] pub chip_tint: crate::particle_fx::ImpactRecipeChipTint,
    /// the dust's color: `Albedo` = the color of what was hit, halfway to gray · `List` = this color
    #[k(crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]))] pub dust_tint: crate::particle_fx::ImpactRecipeDustTint,
    /// m/s the sparks leave at (`None` = 5)
    #[k(None)] pub spark_speed: Option<f64>,
    /// width of a chip (m; `None` = `pfx.chip_size`)
    #[k(None)] pub chip_size: Option<f64>,
}
