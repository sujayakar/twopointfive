#![allow(unused_variables)]
//! Every function and method of `particle_fx`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/particleFx.ts:25 gpuFx)
pub fn gpu_fx(g: &mut crate::game::Game) -> Option<crate::render_particles::Particles> { todo!() }

/// (src/game/particleFx.ts:42 loadDust)
pub fn load_dust(g: &mut crate::game::Game) { todo!() }

/// (src/game/particleFx.ts:48 saveDust)
pub fn save_dust(g: &mut crate::game::Game) { todo!() }

/// (src/game/particleFx.ts:49 resetDust)
pub fn reset_dust(g: &mut crate::game::Game) { todo!() }

/// (src/game/particleFx.ts:51 applyDustLook)
pub fn apply_dust_look(g: &mut crate::game::Game, p: Option<&mut crate::render_particles::Particles>) { todo!() }

/// (src/game/particleFx.ts:56 emitDust)
pub fn emit_dust(g: &mut crate::game::Game, p: &mut crate::render_particles::Particles, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) -> f64 { todo!() }

/// (src/game/particleFx.ts:68 surfAt)
pub fn surf_at(g: &mut crate::game::Game, b: BoxId) -> crate::scene_materials::Surf { todo!() }

/// (src/game/particleFx.ts:105 saidSurf)
pub fn said_surf(g: &mut crate::game::Game, b: Option<BoxId>) -> Option<crate::scene_materials::Surf> { todo!() }

/// (src/game/particleFx.ts:107 solidGlass)
pub fn solid_glass(g: &mut crate::game::Game, b: Option<BoxId>) -> bool { todo!() }

/// (src/game/particleFx.ts:117 hitLook)
pub fn hit_look(g: &mut crate::game::Game, hit: crate::particle_fx::HitLookHit) -> crate::particle_fx::HitLook { todo!() }

/// (src/game/particleFx.ts:162 impactFx)
pub fn impact_fx(g: &mut crate::game::Game, hit_p: V3, dir: V3, hit: crate::particle_fx::ImpactFxHit, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/particleFx.ts:181 muzzleSparksFx)
pub fn muzzle_sparks_fx(g: &mut crate::game::Game, muzzle: V3, dir: V3, shooter: CharId) { todo!() }

/// (src/game/particleFx.ts:191 bangFx)
pub fn bang_fx(g: &mut crate::game::Game, at: V3) { todo!() }

/// (src/game/particleFx.ts:200 kickDustFx)
pub fn kick_dust_fx(g: &mut crate::game::Game, d: DoorId, from: V3) { todo!() }

/// (src/game/particleFx.ts:214 fixtureShatterFx)
pub fn fixture_shatter_fx(g: &mut crate::game::Game, t: &crate::particle_fx::FixtureShatterFxT, from: V3) { todo!() }

/// (src/game/particleFx.ts:226 lensShatterFx)
pub fn lens_shatter_fx(g: &mut crate::game::Game, t: crate::particle_fx::LensShatterFxT, from: V3) { todo!() }

/// (src/game/particleFx.ts:233 breakerSparkFx)
pub fn breaker_spark_fx(g: &mut crate::game::Game, at: V3, from: V3) { todo!() }

/// (src/game/particleFx.ts:242 lockSplinterFx)
pub fn lock_splinter_fx(g: &mut crate::game::Game, d: DoorId, from: V3) { todo!() }

/// (src/game/particleFx.ts:253 clackFx)
pub fn clack_fx(g: &mut crate::game::Game, at: V3) { todo!() }

/// (src/game/particleFx.ts:259 emberFx)
pub fn ember_fx(g: &mut crate::game::Game, p: &mut crate::render_particles::Particles, at: V3, dir: V3, o: crate::particle_fx::EmberFxO) { todo!() }

/// (src/game/particleFx.ts:288 casingOf)
pub fn casing_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<crate::particle_fx::CasingSpec> { todo!() }

/// (src/game/particleFx.ts:291 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::particle_fx::FxState { todo!() }

/// (src/game/particleFx.ts:294 casingFx)
pub fn casing_fx(g: &mut crate::game::Game, shooter: CharId, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/particleFx.ts:308 updateParticleFx)
pub fn update_particle_fx(g: &mut crate::game::Game) { todo!() }

/// (src/game/particleFx.ts:314 clearParticleFx)
pub fn clear_particle_fx(g: &mut crate::game::Game) { todo!() }

/// (src/game/particleFx.ts:319 pendingTinks)
pub fn pending_tinks(g: &mut crate::game::Game) -> Vec<crate::particle_fx::PendingTinksItem> { todo!() }

/// (src/game/particleFx.ts:323 sparkSink)
pub fn spark_sink(g: &mut crate::game::Game) -> crate::tracers::SparkSink { todo!() }

/// (src/game/particleFx.ts:328 testBurst)
pub fn test_burst(g: &mut crate::game::Game, kind: crate::particle_fx::TestBurstKind, at: V3) -> f64 { todo!() }
