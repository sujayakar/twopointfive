//! The types of `particle_fx`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ImpactRecipe, src/game/particleFx.ts:130)
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
impl ImpactRecipe {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
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
impl Default for ImpactRecipe { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum ImpactRecipeChipTint {
    List(V3),
    Albedo,
    Pale,
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum ImpactRecipeDustTint {
    List(V3),
    Albedo,
}

/// (TypeScript: CasingSpec, src/game/particleFx.ts:266)
#[derive(Clone, Copy, Debug)]
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
impl CasingSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CasingSpec {
            color: [0.0; 3],
            diameter: 0.0,
            from: crate::particle_fx::CasingSpecFrom::Hand,
            ahead: 0.0,
            up: 0.0,
            right: 0.0,
            dir: [0.0; 3],
            speed: 0.0,
            tink: 0.0,
        }
    }
}
impl Default for CasingSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CasingSpecFrom {
    Hand,
    Muzzle,
}
impl CasingSpecFrom {
    pub const fn as_str(self) -> &'static str {
        match self {
            CasingSpecFrom::Hand => "hand",
            CasingSpecFrom::Muzzle => "muzzle",
        }
    }
}

/// (TypeScript: FxState, src/game/particleFx.ts:289)
#[derive(Clone, Debug)]
pub struct FxState {
    pub tinks: Vec<crate::particle_fx::FxStateTinksItem>,
    pub n: f64,
}
impl FxState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FxState {
            tinks: Vec::new(),
            n: 0.0,
        }
    }
}
impl Default for FxState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:289 FxState.tinks, src/game/particleFx.ts:289)
#[derive(Clone, Copy, Debug)]
pub struct FxStateTinksItem {
    pub t: f64,
    pub pos: V3,
    pub rate: f64,
}
impl FxStateTinksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FxStateTinksItem {
            t: 0.0,
            pos: [0.0; 3],
            rate: 0.0,
        }
    }
}
impl Default for FxStateTinksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:35 DUST, src/game/particleFx.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct ParticleFxModDUST {
    pub count: f64,
    pub size: f64,
    pub life: f64,
    pub speed: f64,
    pub rise: f64,
    pub spread: f64,
}
impl ParticleFxModDUST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParticleFxModDUST {
            count: 0.0,
            size: 0.0,
            life: 0.0,
            speed: 0.0,
            rise: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for ParticleFxModDUST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:39 DUST_LOOK, src/game/particleFx.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct ParticleFxModDUSTLOOK {
    pub opacity: f64,
    pub growth: f64,
    pub fade_in: f64,
    pub soft: f64,
    pub wrap: f64,
    pub ambient: f64,
}
impl ParticleFxModDUSTLOOK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParticleFxModDUSTLOOK {
            opacity: 0.0,
            growth: 0.0,
            fade_in: 0.0,
            soft: 0.0,
            wrap: 0.0,
            ambient: 0.0,
        }
    }
}
impl Default for ParticleFxModDUSTLOOK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:40 DUST_DEFAULTS, src/game/particleFx.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct DUSTDEFAULTS {
    pub emit: crate::particle_fx::DUSTDEFAULTSEmit,
    pub look: crate::particle_fx::DUSTDEFAULTSLook,
}
impl DUSTDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DUSTDEFAULTS {
            emit: crate::particle_fx::DUSTDEFAULTSEmit::zeroed(),
            look: crate::particle_fx::DUSTDEFAULTSLook::zeroed(),
        }
    }
}
impl Default for DUSTDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:40 DUSTDEFAULTS.emit, src/game/particleFx.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct DUSTDEFAULTSEmit {
    pub count: f64,
    pub size: f64,
    pub life: f64,
    pub speed: f64,
    pub rise: f64,
    pub spread: f64,
}
impl DUSTDEFAULTSEmit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DUSTDEFAULTSEmit {
            count: 0.0,
            size: 0.0,
            life: 0.0,
            speed: 0.0,
            rise: 0.0,
            spread: 0.0,
        }
    }
}
impl Default for DUSTDEFAULTSEmit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:40 DUSTDEFAULTS.look, src/game/particleFx.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct DUSTDEFAULTSLook {
    pub opacity: f64,
    pub growth: f64,
    pub fade_in: f64,
    pub soft: f64,
    pub wrap: f64,
    pub ambient: f64,
}
impl DUSTDEFAULTSLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DUSTDEFAULTSLook {
            opacity: 0.0,
            growth: 0.0,
            fade_in: 0.0,
            soft: 0.0,
            wrap: 0.0,
            ambient: 0.0,
        }
    }
}
impl Default for DUSTDEFAULTSLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:96 STUFF_SURF, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct STUFFSURF {
    pub carpet: Option<String>,
    pub grass: Option<String>,
    pub rubber: Option<String>,
    pub metal: Option<String>,
    pub glass: Option<String>,
    pub wood: Option<String>,
    pub plastic: Option<String>,
    pub fabric: Option<String>,
    pub steel: Option<String>,
    pub enamel: Option<String>,
    pub tarmac: Option<String>,
    pub mud_wall: Option<String>,
    pub plaster: Option<String>,
    pub block: Option<String>,
    pub earth: Option<String>,
    pub gravel: Option<String>,
    pub slab: Option<String>,
    pub sheet: Option<String>,
    pub cloth: Option<String>,
    pub paint: Option<String>,
    pub veneer: Option<String>,
    pub carpaint: Option<String>,
    pub stucco: Option<String>,
    pub skim: Option<String>,
    pub gloss_block: Option<String>,
    pub board_concrete: Option<String>,
    pub cast_panel: Option<String>,
    pub paver: Option<String>,
    pub brushed: Option<String>,
    pub louvre: Option<String>,
    pub plain: Option<String>,
    pub waits: Option<String>,
}
impl STUFFSURF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        STUFFSURF {
            carpet: None,
            grass: None,
            rubber: None,
            metal: None,
            glass: None,
            wood: None,
            plastic: None,
            fabric: None,
            steel: None,
            enamel: None,
            tarmac: None,
            mud_wall: None,
            plaster: None,
            block: None,
            earth: None,
            gravel: None,
            slab: None,
            sheet: None,
            cloth: None,
            paint: None,
            veneer: None,
            carpaint: None,
            stucco: None,
            skim: None,
            gloss_block: None,
            board_concrete: None,
            cast_panel: None,
            paver: None,
            brushed: None,
            louvre: None,
            plain: None,
            waits: None,
        }
    }
}
impl Default for STUFFSURF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:113 SOLID_GLASS_CRACK, src/game/particleFx.ts:113)
#[derive(Clone, Copy, Debug)]
pub struct SOLIDGLASSCRACK {
    pub look: f64,
    pub pistol: f64,
    pub rifle: f64,
    pub pellet: f64,
    pub strength: f64,
}
impl SOLIDGLASSCRACK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SOLIDGLASSCRACK {
            look: 0.0,
            pistol: 0.0,
            rifle: 0.0,
            pellet: 0.0,
            strength: 0.0,
        }
    }
}
impl Default for SOLIDGLASSCRACK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:144 PFX, src/game/particleFx.ts:144)
#[derive(Clone, Copy, Debug)]
pub struct ParticleFxModPFX {
    pub impact_scale_rifle: f64,
    pub spark_size: f64,
    pub spark_life: f64,
    pub spark_spread: f64,
    pub chip_speed: f64,
    pub chip_life: f64,
    pub chip_size: f64,
    pub chip_spread: f64,
    pub dust_speed: f64,
    pub dust_life: f64,
    pub dust_size: f64,
    pub muzzle_spark_size: f64,
    pub bang_sparks: f64,
    pub bang_spark_size: f64,
    pub bang_ring: f64,
    pub bang_ring_radius: f64,
    pub bang_ring_speed: f64,
    pub bang_ring_life: f64,
    pub bang_ring_size: f64,
    pub bang_puff: f64,
    pub kick_dust: f64,
    pub kick_splinters: f64,
    pub lock_splinters: f64,
    pub lock_splinters_back: f64,
    pub lock_screws: f64,
    pub tube_shards: f64,
    pub bulb_shards: f64,
    pub tube_sparks: f64,
    pub breaker_sparks: f64,
    pub clack_sparks: f64,
    pub casing_life: f64,
    pub tink_gain: f64,
    pub ember_size: f64,
}
impl ParticleFxModPFX {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParticleFxModPFX {
            impact_scale_rifle: 0.0,
            spark_size: 0.0,
            spark_life: 0.0,
            spark_spread: 0.0,
            chip_speed: 0.0,
            chip_life: 0.0,
            chip_size: 0.0,
            chip_spread: 0.0,
            dust_speed: 0.0,
            dust_life: 0.0,
            dust_size: 0.0,
            muzzle_spark_size: 0.0,
            bang_sparks: 0.0,
            bang_spark_size: 0.0,
            bang_ring: 0.0,
            bang_ring_radius: 0.0,
            bang_ring_speed: 0.0,
            bang_ring_life: 0.0,
            bang_ring_size: 0.0,
            bang_puff: 0.0,
            kick_dust: 0.0,
            kick_splinters: 0.0,
            lock_splinters: 0.0,
            lock_splinters_back: 0.0,
            lock_screws: 0.0,
            tube_shards: 0.0,
            bulb_shards: 0.0,
            tube_sparks: 0.0,
            breaker_sparks: 0.0,
            clack_sparks: 0.0,
            casing_life: 0.0,
            tink_gain: 0.0,
            ember_size: 0.0,
        }
    }
}
impl Default for ParticleFxModPFX { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:269 CASINGS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
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
impl CASINGS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CASINGS {
            fiveseven: None,
            guard9mm: None,
            aks74u: None,
            ak74m: None,
            akms: None,
            fn2000: None,
            m870: None,
            mk12: None,
            mp7: None,
            mk46: None,
            m1014: None,
            hk45: None,
            g18: None,
            pkm: None,
            mk18bare: None,
            g19bare: None,
        }
    }
}
impl Default for CASINGS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:117 hitLook(hit), src/game/particleFx.ts:117)
#[derive(Clone, Copy, Debug)]
pub struct HitLookHit {
    pub index: Option<f64>,
    pub prop: Option<PropId>,
    pub box_: Option<BoxId>,
}
impl HitLookHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HitLookHit {
            index: None,
            prop: None,
            box_: None,
        }
    }
}
impl Default for HitLookHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:117 hitLook ->, src/game/particleFx.ts:117)
#[derive(Clone, Copy, Debug)]
pub struct HitLook {
    pub surf: crate::scene_materials::Surf,
    pub albedo: V3,
    pub solid: bool,
    pub said: bool,
}
impl HitLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HitLook {
            surf: crate::scene_materials::Surf::Carpet,
            albedo: [0.0; 3],
            solid: false,
            said: false,
        }
    }
}
impl Default for HitLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:162 impactFx(hit), src/game/particleFx.ts:162)
#[derive(Clone, Copy, Debug)]
pub struct ImpactFxHit {
    pub n: V3,
    pub index: Option<f64>,
    pub prop: Option<PropId>,
    pub box_: Option<BoxId>,
}
impl ImpactFxHit {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ImpactFxHit {
            n: [0.0; 3],
            index: None,
            prop: None,
            box_: None,
        }
    }
}
impl Default for ImpactFxHit { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:214 fixtureShatterFx(t), src/game/particleFx.ts:214)
#[derive(Clone, Debug)]
pub struct FixtureShatterFxT {
    pub pos: V3,
    pub kind: String,
    pub boxes: Vec<f64>,
}
impl FixtureShatterFxT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FixtureShatterFxT {
            pos: [0.0; 3],
            kind: String::new(),
            boxes: Vec::new(),
        }
    }
}
impl Default for FixtureShatterFxT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:226 lensShatterFx(t), src/game/particleFx.ts:226)
#[derive(Clone, Copy, Debug)]
pub struct LensShatterFxT {
    pub pos: V3,
    pub lens: Option<V3>,
}
impl LensShatterFxT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LensShatterFxT {
            pos: [0.0; 3],
            lens: None,
        }
    }
}
impl Default for LensShatterFxT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:259 emberFx(o), src/game/particleFx.ts:259)
#[derive(Clone, Copy, Debug)]
pub struct EmberFxO {
    pub count: f64,
    pub speed: f64,
    pub cone: f64,
    pub life: f64,
}
impl EmberFxO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EmberFxO {
            count: 0.0,
            speed: 0.0,
            cone: 0.0,
            life: 0.0,
        }
    }
}
impl Default for EmberFxO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/particleFx.ts:319 pendingTinks ->, src/game/particleFx.ts:319)
#[derive(Clone, Copy, Debug)]
pub struct PendingTinksItem {
    pub t: f64,
    pub pos: V3,
}
impl PendingTinksItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PendingTinksItem {
            t: 0.0,
            pos: [0.0; 3],
        }
    }
}
impl Default for PendingTinksItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TestBurstKind {
    Dust,
    Bang,
    Debris,
    Sparks,
    Casings,
    Sheets,
}
impl TestBurstKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            TestBurstKind::Dust => "dust",
            TestBurstKind::Bang => "bang",
            TestBurstKind::Debris => "debris",
            TestBurstKind::Sparks => "sparks",
            TestBurstKind::Casings => "casings",
            TestBurstKind::Sheets => "sheets",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct ParticleFxMod {
    /// (src/game/particleFx.ts:35 DUST)
    pub dust: crate::particle_fx::ParticleFxModDUST,
    /// (src/game/particleFx.ts:39 DUST_LOOK)
    pub dust_look: crate::particle_fx::ParticleFxModDUSTLOOK,
    /// (src/game/particleFx.ts:144 PFX)
    pub pfx: crate::particle_fx::ParticleFxModPFX,
    /// (src/game/particleFx.ts:290 STATE)
    pub state: Option<crate::particle_fx::FxState>,
}
impl ParticleFxMod {
    pub fn blank() -> Self {
        ParticleFxMod {
            dust: crate::particle_fx::ParticleFxModDUST { count: 1.0, size: 1.0, life: 1.0, speed: 1.0, rise: 1.0, spread: 1.0 },
            dust_look: crate::particle_fx::ParticleFxModDUSTLOOK { opacity: 0.3, growth: 0.6, fade_in: 0.05, soft: 1.3, wrap: 0.5, ambient: 1.0 },
            pfx: crate::particle_fx::ParticleFxModPFX { impact_scale_rifle: 1.4, spark_size: 0.003, spark_life: 0.38, spark_spread: 0.9, chip_speed: 1.7, chip_life: 18.0, chip_size: 0.007, chip_spread: 1.0, dust_speed: 0.3, dust_life: 1.0, dust_size: 0.1, muzzle_spark_size: 0.002, bang_sparks: 160.0, bang_spark_size: 0.0025, bang_ring: 40.0, bang_ring_radius: 0.25, bang_ring_speed: 3.2, bang_ring_life: 1.7, bang_ring_size: 0.16, bang_puff: 10.0, kick_dust: 9.0, kick_splinters: 14.0, lock_splinters: 22.0, lock_splinters_back: 6.0, lock_screws: 4.0, tube_shards: 34.0, bulb_shards: 18.0, tube_sparks: 16.0, breaker_sparks: 70.0, clack_sparks: 5.0, casing_life: 45.0, tink_gain: 0.16, ember_size: 0.0025 },
            state: None,
        }
    }
}

/// (src/game/particleFx.ts:40 DUST_DEFAULTS: a table nothing writes)
pub fn dust_defaults() -> crate::particle_fx::DUSTDEFAULTS { crate::particle_fx::DUSTDEFAULTS { emit: crate::particle_fx::DUSTDEFAULTSEmit { count: 1.0, size: 1.0, life: 1.0, speed: 1.0, rise: 1.0, spread: 1.0 }, look: crate::particle_fx::DUSTDEFAULTSLook { opacity: 0.3, growth: 0.6, fade_in: 0.05, soft: 1.3, wrap: 0.5, ambient: 1.0 } } }

/// (src/game/particleFx.ts:41 DUST_LS)
pub const DUST_LS: &str = "twopointsix.dust.v2";

/// (src/game/particleFx.ts:69 PROP_SURF: a table nothing writes)
pub fn prop_surf() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("chair"), String::from("fabric")); t.set(String::from("cardboard"), String::from("cardboard")); t.set(String::from("bin"), String::from("metal")); t.set(String::from("plant"), String::from("foliage")); t } }

/// (src/game/particleFx.ts:96 STUFF_SURF: a table nothing writes)
pub fn stuff_surf() -> crate::particle_fx::STUFFSURF { crate::particle_fx::STUFFSURF { carpet: Some(String::from("carpet")), grass: None, rubber: Some(String::from("plastic")), metal: Some(String::from("metal")), glass: Some(String::from("glass")), wood: Some(String::from("wood")), plastic: Some(String::from("plastic")), fabric: Some(String::from("fabric")), steel: Some(String::from("metal")), enamel: None, tarmac: Some(String::from("concrete")), mud_wall: Some(String::from("concrete")), plaster: Some(String::from("drywall")), block: Some(String::from("concrete")), earth: None, gravel: None, slab: Some(String::from("concrete")), sheet: Some(String::from("metal")), cloth: Some(String::from("carpet")), paint: None, veneer: Some(String::from("wood")), carpaint: Some(String::from("metal")), stucco: Some(String::from("concrete")), skim: Some(String::from("drywall")), gloss_block: Some(String::from("concrete")), board_concrete: Some(String::from("concrete")), cast_panel: Some(String::from("concrete")), paver: Some(String::from("concrete")), brushed: Some(String::from("metal")), louvre: Some(String::from("metal")), plain: None, waits: None } }

/// (src/game/particleFx.ts:102 HIT_STUFFS: a table nothing writes)
pub fn hit_stuffs() -> Vec<String> { alloc::vec![String::from("carpaint"), String::from("steel"), String::from("metal"), String::from("sheet"), String::from("brushed"), String::from("louvre"), String::from("glass"), String::from("rubber"), String::from("plastic"), String::from("wood"), String::from("veneer"), String::from("fabric"), String::from("cloth"), String::from("carpet"), String::from("plaster"), String::from("skim"), String::from("stucco"), String::from("mud wall"), String::from("block"), String::from("gloss block"), String::from("board concrete"), String::from("cast panel"), String::from("slab"), String::from("paver"), String::from("tarmac")] }

/// (src/game/particleFx.ts:109 SOLID_GLASS_CHIPS)
pub const SOLID_GLASS_CHIPS: f64 = 3.0;

/// (src/game/particleFx.ts:113 SOLID_GLASS_CRACK: a table nothing writes)
pub fn solid_glass_crack() -> crate::particle_fx::SOLIDGLASSCRACK { crate::particle_fx::SOLIDGLASSCRACK { look: 9.0, pistol: 0.085, rifle: 0.11, pellet: 0.035, strength: 0.9 } }

/// (src/game/particleFx.ts:131 IMPACT: a table nothing writes)
pub fn impact() -> JsMap<String, crate::particle_fx::ImpactRecipe> { { let mut t = JsMap::new(); t.set(String::from("metal"), crate::particle_fx::ImpactRecipe { sparks: 16.0, chips: 2.0, dust: 0.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.08, 0.08, 0.09]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: Some(6.0), chip_size: Some(0.005) }); t.set(String::from("plastic"), crate::particle_fx::ImpactRecipe { sparks: 6.0, chips: 5.0, dust: 1.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: None, chip_size: None }); t.set(String::from("glass"), crate::particle_fx::ImpactRecipe { sparks: 4.0, chips: 9.0, dust: 0.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.1, 0.11, 0.12]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.5, 0.5, 0.5]), spark_speed: None, chip_size: Some(0.006) }); t.set(String::from("wood"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 12.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.66, 0.52, 0.34]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.62, 0.52, 0.4]), spark_speed: None, chip_size: None }); t.set(String::from("cardboard"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 8.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.6, 0.5, 0.36]), spark_speed: None, chip_size: None }); t.set(String::from("drywall"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 10.0, dust: 6.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Pale, dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.8, 0.78, 0.74]), spark_speed: None, chip_size: None }); t.set(String::from("concrete"), crate::particle_fx::ImpactRecipe { sparks: 5.0, chips: 10.0, dust: 5.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.45, 0.44, 0.42]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.55, 0.54, 0.52]), spark_speed: Some(5.0), chip_size: None }); t.set(String::from("carpet"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 4.0, dust: 4.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::Albedo, spark_speed: None, chip_size: None }); t.set(String::from("fabric"), crate::particle_fx::ImpactRecipe { sparks: 2.0, chips: 5.0, dust: 3.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::Albedo, dust_tint: crate::particle_fx::ImpactRecipeDustTint::Albedo, spark_speed: None, chip_size: None }); t.set(String::from("foliage"), crate::particle_fx::ImpactRecipe { sparks: 0.0, chips: 10.0, dust: 2.0, chip_tint: crate::particle_fx::ImpactRecipeChipTint::List([0.14, 0.32, 0.1]), dust_tint: crate::particle_fx::ImpactRecipeDustTint::List([0.3, 0.22, 0.14]), spark_speed: None, chip_size: None }); t } }

/// (src/game/particleFx.ts:267 HULL: a table nothing writes)
pub fn hull() -> V3 { [0.55, 0.06, 0.04] }

/// (src/game/particleFx.ts:268 BRASS: a table nothing writes)
pub fn brass() -> V3 { [0.72, 0.5, 0.16] }

/// (src/game/particleFx.ts:268 LACQUERED_STEEL: a table nothing writes)
pub fn lacquered_steel() -> V3 { [0.2, 0.19, 0.12] }

/// (src/game/particleFx.ts:269 CASINGS: a table nothing writes)
pub fn casings() -> crate::particle_fx::CASINGS { crate::particle_fx::CASINGS { fiveseven: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.008, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.9, -0.25], speed: 2.6, tink: 0.5 }), guard9mm: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 3.0, tink: 0.5 }), aks74u: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.01, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.8, tink: 0.55 }), ak74m: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.01, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.8, tink: 0.55 }), akms: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.0113, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.5, tink: 0.55 }), fn2000: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Muzzle, ahead: -0.16, up: -0.01, right: 0.045, dir: [0.35, -0.15, 1.0], speed: 1.1, tink: 0.4 }), m870: None, mk12: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.5, 0.15], speed: 3.4, tink: 0.5 }), mp7: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0075, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.1, up: 0.06, right: 0.02, dir: [1.0, 0.7, -0.1], speed: 3.0, tink: 0.5 }), mk46: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.02, right: 0.03, dir: [1.0, -0.1, 0.1], speed: 3.0, tink: 0.45 }), m1014: Some(crate::particle_fx::CasingSpec { color: [0.55, 0.06, 0.04], diameter: 0.02, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.14, up: 0.07, right: 0.03, dir: [1.0, 0.35, 0.1], speed: 2.6, tink: 0.5 }), hk45: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.012, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 2.6, tink: 0.5 }), g18: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 3.0, tink: 0.5 }), pkm: Some(crate::particle_fx::CasingSpec { color: [0.2, 0.19, 0.12], diameter: 0.0124, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.05, right: -0.03, dir: [-1.0, 0.2, 0.1], speed: 3.6, tink: 0.5 }), mk18bare: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Muzzle, ahead: -0.16, up: -0.01, right: 0.045, dir: [0.35, -0.15, 1.0], speed: 1.1, tink: 0.4 }), g19bare: Some(crate::particle_fx::CasingSpec { color: [0.72, 0.5, 0.16], diameter: 0.008, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.9, -0.25], speed: 2.6, tink: 0.5 }) } }
