//! The types of `particle_fx`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug)]
pub struct ImpactRecipe {
    pub sparks: f64,
    pub chips: f64,
    pub dust: f64,
    pub chip_tint: crate::particle_fx::ImpactRecipeChipTint,
    pub dust_tint: crate::particle_fx::ImpactRecipeDustTint,
    pub spark_speed: Option<f64>,
    pub chip_size: Option<f64>,
}
impl Default for ImpactRecipe {
    fn default() -> Self {
        ImpactRecipe {
            sparks: 0.0,
            chips: 0.0,
            dust: 0.0,
            chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.0; 3]),
            dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.0; 3]),
            spark_speed: None,
            chip_size: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImpactRecipeChipTint {
    List(V3),
    Albedo,
    Pale,
}

#[derive(Clone, Copy, Debug)]
pub enum ImpactRecipeDustTint {
    List(V3),
    Albedo,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CasingSpec {
    pub color: V3,
    pub diameter: f64,
    pub from: crate::particle_fx::CasingSpecFrom,
    pub ahead: f64,
    pub up: f64,
    pub right: f64,
    pub dir: V3,
    pub speed: f64,
    pub tink: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum CasingSpecFrom {
    #[default] Hand,
    Muzzle,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct FxState {
    pub tinks: Vec<crate::particle_fx::FxStateTinksItem>,
    pub n: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FxStateTinksItem {
    pub t: f64,
    pub pos: V3,
    pub rate: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SOLIDGLASSCRACK {
    pub look: f64,
    pub pistol: f64,
    pub rifle: f64,
    pub pellet: f64,
    pub strength: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CASINGS {
    pub fiveseven: Option<crate::particle_fx::CasingSpec>,
    pub guard9mm: Option<crate::particle_fx::CasingSpec>,
    pub aks74u: Option<crate::particle_fx::CasingSpec>,
    pub ak74m: Option<crate::particle_fx::CasingSpec>,
    pub akms: Option<crate::particle_fx::CasingSpec>,
    pub fn2000: Option<crate::particle_fx::CasingSpec>,
    pub m870: Option<crate::particle_fx::CasingSpec>,
    pub mk12: Option<crate::particle_fx::CasingSpec>,
    pub mp7: Option<crate::particle_fx::CasingSpec>,
    pub mk46: Option<crate::particle_fx::CasingSpec>,
    pub m1014: Option<crate::particle_fx::CasingSpec>,
    pub hk45: Option<crate::particle_fx::CasingSpec>,
    pub g18: Option<crate::particle_fx::CasingSpec>,
    pub pkm: Option<crate::particle_fx::CasingSpec>,
    pub mk18bare: Option<crate::particle_fx::CasingSpec>,
    pub g19bare: Option<crate::particle_fx::CasingSpec>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HitLook {
    pub surf: crate::scene_materials::Surf,
    pub albedo: V3,
    pub solid: bool,
    pub said: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FixtureShatterFxT {
    pub pos: V3,
    pub kind: String,
    pub boxes: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LensShatterFxT {
    pub pos: V3,
    pub lens: Option<V3>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EmberFxO {
    pub count: f64,
    pub speed: f64,
    pub cone: f64,
    pub life: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ParticleFxMod {
    pub state: Option<crate::particle_fx::FxState>,
}

pub const SOLID_GLASS_CHIPS: f64 = 3.0;

pub fn solid_glass_crack() -> crate::particle_fx::SOLIDGLASSCRACK { crate::particle_fx::SOLIDGLASSCRACK { look: 9.0, pistol: 0.085, rifle: 0.11, pellet: 0.035, strength: 0.9 } }

pub fn impact() -> JsMap<String, crate::particle_fx::ImpactRecipe> { { let mut t = JsMap::new(); t.set(String::from("metal"), crate::particle_fx::ImpactRecipe { sparks: 16.0, chips: 2.0, dust: 0.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.08, 0.08, 0.09]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: Some(6.0), chip_size: Some(0.005) }); t.set(String::from("plastic"), crate::particle_fx::ImpactRecipe { sparks: 6.0, chips: 5.0, dust: 1.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: None, chip_size: None }); t.set(String::from("glass"), crate::particle_fx::ImpactRecipe { sparks: 4.0, chips: 9.0, dust: 0.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.1, 0.11, 0.12]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: None, chip_size: Some(0.006) }); t.set(String::from("wood"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 12.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.66, 0.52, 0.34]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.62, 0.52, 0.4]), spark_speed: None, chip_size: None }); t.set(String::from("cardboard"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 8.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.6, 0.5, 0.36]), spark_speed: None, chip_size: None }); t.set(String::from("drywall"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 10.0, dust: 6.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Pale, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.8, 0.78, 0.74]), spark_speed: None, chip_size: None }); t.set(String::from("concrete"), crate::particle_fx::ImpactRecipe { sparks: 5.0, chips: 10.0, dust: 5.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.45, 0.44, 0.42]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.55, 0.54, 0.52]), spark_speed: Some(5.0), chip_size: None }); t.set(String::from("carpet"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 4.0, dust: 4.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::Albedo, spark_speed: None, chip_size: None }); t.set(String::from("fabric"), crate::particle_fx::ImpactRecipe { sparks: 2.0, chips: 5.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::Albedo, spark_speed: None, chip_size: None }); t.set(String::from("foliage"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 10.0, dust: 2.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.14, 0.32, 0.1]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.3, 0.22, 0.14]), spark_speed: None, chip_size: None }); t } }

pub fn casings() -> crate::particle_fx::CASINGS { crate::particle_fx::CASINGS { fiveseven: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.008, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.9, -0.25], speed: 2.6, tink: 0.5 }), guard9mm: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 3.0, tink: 0.5 }), aks74u: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.01, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.8, tink: 0.55 }), ak74m: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.01, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.8, tink: 0.55 }), akms: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.0113, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.5, tink: 0.55 }), fn2000: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Muzzle, ahead: -0.16, up: -0.01, right: 0.045, dir: [0.35, -0.15, 1.0], speed: 1.1, tink: 0.4 }), m870: None, mk12: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.5, 0.15], speed: 3.4, tink: 0.5 }), mp7: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0075, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.1, up: 0.06, right: 0.02, dir: [1.0, 0.7, -0.1], speed: 3.0, tink: 0.5 }), mk46: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.02, right: 0.03, dir: [1.0, -0.1, 0.1], speed: 3.0, tink: 0.45 }), m1014: Some(crate::particle_fx::CasingSpec { color: [0.55, 0.06, 0.04], diameter: 0.02, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.14, up: 0.07, right: 0.03, dir: [1.0, 0.35, 0.1], speed: 2.6, tink: 0.5 }), hk45: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.012, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 2.6, tink: 0.5 }), g18: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 3.0, tink: 0.5 }), pkm: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.0124, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.05, right: -0.03, dir: [-1.0, 0.2, 0.1], speed: 3.6, tink: 0.5 }), mk18bare: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Muzzle, ahead: -0.16, up: -0.01, right: 0.045, dir: [0.35, -0.15, 1.0], speed: 1.1, tink: 0.4 }), g19bare: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.008, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.9, -0.25], speed: 2.6, tink: 0.5 }) } }
