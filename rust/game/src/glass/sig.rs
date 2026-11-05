#![allow(unused_variables)]
//! Every function and method of `glass`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/glass.ts:50 glassFxForced)
pub fn glass_fx_forced(g: &mut crate::game::Game) -> Option<bool> { todo!() }

/// (src/game/glass.ts:52 loadGlassFx)
pub fn load_glass_fx(g: &mut crate::game::Game) { todo!() }

/// (src/game/glass.ts:53 saveGlassFx)
pub fn save_glass_fx(g: &mut crate::game::Game) { todo!() }

/// (src/game/glass.ts:55 glassFxBoot)
pub fn glass_fx_boot(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/glass.ts:60 glassState)
pub fn glass_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::glass::GlassState { todo!() }

/// (src/game/glass.ts:68 resetGlass)
pub fn reset_glass(g: &mut crate::game::Game) { todo!() }

/// (src/game/glass.ts:71 glassPanes)
pub fn glass_panes(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/glass.ts:72 isPane)
pub fn is_pane(g: &mut crate::game::Game, b: Option<BoxId>) -> bool { todo!() }

/// (src/game/glass.ts:76 isSheet)
pub fn is_sheet(g: &mut crate::game::Game, b: Option<BoxId>) -> bool { todo!() }

/// (src/game/glass.ts:79 paneNormal)
pub fn pane_normal(g: &mut crate::game::Game, b: BoxId) -> V3 { todo!() }

/// (src/game/glass.ts:86 glassEvent)
pub fn glass_event(g: &mut crate::game::Game, b: BoxId, at: V3, level: f64, who: f64) { todo!() }

/// (src/game/glass.ts:95 nearestOnBox)
pub fn nearest_on_box(g: &mut crate::game::Game, b: BoxId, p: V3) -> crate::glass::NearestOnBox { todo!() }

/// (src/game/glass.ts:102 seedMix)
pub fn seed_mix(h: f64, x: f64) -> f64 { todo!() }

/// (src/game/glass.ts:115 boxSeed)
pub fn box_seed(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/glass.ts:117 pointSeed)
pub fn point_seed(word: f64, p: V3) -> f64 { todo!() }

/// (src/game/glass.ts:120 hitSeed)
pub fn hit_seed(g: &mut crate::game::Game, b: BoxId, p: V3) -> f64 { todo!() }

/// (src/game/glass.ts:124 blastSeed)
pub fn blast_seed(word: f64, at: V3) -> f64 { todo!() }

/// (src/game/glass.ts:131 glassImpact)
pub fn glass_impact(g: &mut crate::game::Game, pane: f64, point: V3, dir: Option<V3>, energy: f64, who: f64, cause: Option<crate::glass::GlassImpactCause>, quiet: Option<bool>, opts: Option<crate::glass::GlassImpactOpts>) -> crate::glassbreak::GlassImpact { todo!() }

/// (src/game/glass.ts:185 glassHit)
pub fn glass_hit(g: &mut crate::game::Game, bi: f64, hit_p: V3, dir: Option<V3>, who: f64, cause: Option<crate::glass::GlassHitCause>, quiet: Option<bool>) -> bool { todo!() }

/// (src/game/glass.ts:191 crackNear)
pub fn crack_near(g: &mut crate::game::Game, at: V3, radius: f64, who: f64) -> f64 { todo!() }

/// (src/game/glass.ts:213 updateGlass)
pub fn update_glass(g: &mut crate::game::Game) { todo!() }

/// (src/game/glass.ts:224 crackTest)
pub fn crack_test(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glass.ts:238 crackCount)
pub fn crack_count(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glass.ts:241 glassSummary)
pub fn glass_summary(g: &mut crate::game::Game) -> String { todo!() }
