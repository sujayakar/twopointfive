//! The types of `particle_fx`.
use crate::{Blank, bytes::ToBytes, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum ImpactRecipeChipTint {
    List(V3),
    Albedo,
    Pale,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub enum ImpactRecipeDustTint {
    List(V3),
    Albedo,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, ToBytes)]
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

/// the kind of mark a solid glass's crack is
pub const SOLID_GLASS_LOOK: f64 = 9.0;
pub const SOLID_GLASS_CHIPS: f64 = 3.0;

