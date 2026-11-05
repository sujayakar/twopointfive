#![allow(unused_variables)]
//! Every function and method of `furnblast`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/furnblast.ts:52 chargeSource)
pub fn charge_source(radius: f64) -> crate::furnblast::BlastSource { todo!() }

/// (src/game/furnblast.ts:112 breakClassOf)
pub fn break_class_of(g: &mut crate::game::Game, b: &crate::furnblast::BreakClassOfB) -> crate::scene_materials::BreakClass { todo!() }

/// (src/game/furnblast.ts:130 aabbOf)
pub fn aabb_of(g: &mut crate::game::Game, b: BoxId) -> crate::furnblast::AABB { todo!() }

/// (src/game/furnblast.ts:134 distXZ)
pub fn dist_xz(p: V3, a: crate::furnblast::AABB) -> f64 { todo!() }

/// (src/game/furnblast.ts:135 volOf)
pub fn vol_of(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/furnblast.ts:141 overPlan)
pub fn over_plan(a: crate::furnblast::OverPlanA, q: crate::furnblast::OverPlanQ) -> bool { todo!() }

/// (src/game/furnblast.ts:157 Furniture.constructor)
pub fn furniture_new(g: &mut crate::game::Game) -> crate::furnblast::Furniture { todo!() }

/// (src/game/furnblast.ts:159 Furniture.hash)
pub fn furniture_hash(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/furnblast.ts:162 Furniture.index)
pub fn furniture_index(g: &mut crate::game::Game) -> Vec<crate::furnblast::Group> { todo!() }

/// (src/game/furnblast.ts:225 Furniture.warm)
pub fn furniture_warm(g: &mut crate::game::Game) { todo!() }

/// (src/game/furnblast.ts:227 Furniture.inventory)
pub fn furniture_inventory(g: &mut crate::game::Game) -> crate::furnblast::FurnitureInventory { todo!() }

/// (src/game/furnblast.ts:229 Furniture.stands)
pub fn furniture_stands(g: &mut crate::game::Game, m: FurnblastMemberId) -> bool { todo!() }

/// (src/game/furnblast.ts:235 Furniture.firstToGo)
pub fn furniture_first_to_go(g: &mut crate::game::Game, at: V3, src: crate::furnblast::BlastSource) -> Option<crate::furnblast::BlastFirst> { todo!() }

/// (src/game/furnblast.ts:249 Furniture.wouldTake)
pub fn furniture_would_take(g: &mut crate::game::Game, first: crate::furnblast::BlastFirst, i: f64, origins: &[V3]) -> bool { todo!() }

/// (src/game/furnblast.ts:258 Furniture.rowTake)
pub fn furniture_row_take(g: &mut crate::game::Game, first: crate::furnblast::BlastFirst, g_2: &crate::furnblast::Group, origins: &[V3]) -> JsSet<FurnblastMemberId> { todo!() }

/// (src/game/furnblast.ts:266 Furniture.rowFall)
pub fn furniture_row_fall(g: &mut crate::game::Game, standing: &[FurnblastMemberId], at: V3, reach: f64, origins: &[V3]) -> JsSet<FurnblastMemberId> { todo!() }

/// (src/game/furnblast.ts:279 Furniture.broken)
pub fn furniture_broken(g: &mut crate::game::Game, i: f64) -> bool { todo!() }

/// (src/game/furnblast.ts:283 Furniture.clear)
pub fn furniture_clear(g: &mut crate::game::Game, o: V3, p: V3) -> bool { todo!() }

/// (src/game/furnblast.ts:296 Furniture.exposedBox)
pub fn furniture_exposed_box(g: &mut crate::game::Game, origins: &[V3], a: crate::furnblast::AABB) -> bool { todo!() }

/// (src/game/furnblast.ts:309 Furniture.blast)
pub fn furniture_blast(g: &mut crate::game::Game, at: V3, src: crate::furnblast::BlastSource, who: f64, chaos: f64) { todo!() }

/// (src/game/furnblast.ts:335 Furniture.fixed)
pub fn furniture_fixed(g: &mut crate::game::Game, at: V3, k: f64, who: f64, origins: &[V3], budget: &mut crate::furnblast::FurnitureFixedBudget) { todo!() }

/// (src/game/furnblast.ts:379 Furniture.breakStatic)
pub fn furniture_break_static(g: &mut crate::game::Game, m: FurnblastMemberId, at: V3, mode: crate::furnblast::FurnitureBreakStaticMode, kk: f64, who: f64, budget: &mut crate::furnblast::FurnitureBreakStaticBudget) -> bool { todo!() }

/// (src/game/furnblast.ts:408 Furniture.throwPiece)
pub fn furniture_throw_piece(g: &mut crate::game::Game, f: BoxId, at: V3, mode: crate::furnblast::FurnitureThrowPieceMode, kk: f64, row: &crate::furnblast::BreakRow, who: f64, budget: &mut crate::furnblast::FurnitureThrowPieceBudget, of: BoxId, steel: Option<crate::furnblast::SteelPart>) { todo!() }

/// (src/game/furnblast.ts:434 Furniture.hungGoes)
pub fn furniture_hung_goes(g: &mut crate::game::Game, b: BoxId, at: V3, n: V3, who: f64) { todo!() }

/// (src/game/furnblast.ts:455 Furniture.steel)
pub fn furniture_steel(g: &mut crate::game::Game, b: BoxId, mode: crate::furnblast::FurnitureSteelMode, dir: V3, seed: f64) -> Option<Vec<crate::furnblast::SteelPart>> { todo!() }

/// (src/game/furnblast.ts:469 Furniture.fragments)
pub fn furniture_fragments(g: &mut crate::game::Game, b: BoxId, row: &crate::furnblast::BreakRow, mode: crate::furnblast::FurnitureFragmentsMode) -> Vec<BoxId> { todo!() }

/// (src/game/furnblast.ts:484 Furniture.pushable)
pub fn furniture_pushable(g: &mut crate::game::Game, at: V3, k: f64, who: f64, origins: &[V3], budget: &mut crate::furnblast::FurniturePushableBudget) { todo!() }

/// (src/game/furnblast.ts:498 Furniture.breakProp)
pub fn furniture_break_prop(g: &mut crate::game::Game, p: PropId, at: V3, fall: f64, who: f64, budget: &mut crate::furnblast::FurnitureBreakPropBudget) { todo!() }

/// (src/game/furnblast.ts:508 Furniture.roofed)
pub fn furniture_roofed(g: &mut crate::game::Game, x: f64, z: f64) -> bool { todo!() }

/// (src/game/furnblast.ts:510 Furniture.mouthOk)
pub fn furniture_mouth_ok(g: &mut crate::game::Game, i: f64, saved: Option<bool>) -> bool { todo!() }

/// (src/game/furnblast.ts:518 Furniture.mouthStack)
pub fn furniture_mouth_stack(g: &mut crate::game::Game, seed: &[crate::furnblast::AABB], done: &JsSet<crate::Num>, n: f64) -> Option<Vec<f64>> { todo!() }

/// (src/game/furnblast.ts:534 Furniture.mouthMine)
pub fn furniture_mouth_mine(g: &mut crate::game::Game, i: f64, b: BoxId, a: crate::furnblast::AABB) -> bool { todo!() }

/// (src/game/furnblast.ts:545 Furniture.mouthWouldTake)
pub fn furniture_mouth_would_take(g: &mut crate::game::Game, i: f64, r: [f64; 4], depth: Option<f64>) -> bool { todo!() }

/// (src/game/furnblast.ts:571 Furniture.mouth)
pub fn furniture_mouth(g: &mut crate::game::Game, at: V3, who: f64, r: [f64; 4], budget: &mut crate::furnblast::FurnitureMouthBudget) { todo!() }

/// (src/game/furnblast.ts:626 Furniture.away)
pub fn furniture_away(g: &mut crate::game::Game, at: V3, p: V3) -> V3 { todo!() }

/// (src/game/furnblast.ts:627 Furniture.chips)
pub fn furniture_chips(g: &mut crate::game::Game, c_2: V3, v: V3, b: BoxId, who: f64, n: f64) { todo!() }

/// (src/game/furnblast.ts:628 Furniture.bump)
pub fn furniture_bump(g: &mut crate::game::Game) { todo!() }

/// (src/game/furnblast.ts:631 Furniture.reset)
pub fn furniture_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/furnblast.ts:643 Furniture.snapshot)
pub fn furniture_snapshot(g: &mut crate::game::Game) -> Option<crate::furnblast::FurnSave> { todo!() }

/// (src/game/furnblast.ts:649 Furniture.check)
pub fn furniture_check(g: &mut crate::game::Game, s: Option<&crate::furnblast::FurnSave>) -> Option<String> { todo!() }

/// (src/game/furnblast.ts:659 Furniture.restore)
pub fn furniture_restore(g: &mut crate::game::Game, s: Option<&crate::furnblast::FurnSave>) { todo!() }

/// (src/game/furnblast.ts:677 Furniture.noteReset)
pub fn furniture_note_reset(g: &mut crate::game::Game, cause: crate::furnblast::FurnitureNoteResetCause) { todo!() }

/// (src/game/furnblast.ts:681 Furniture.summary)
pub fn furniture_summary(g: &mut crate::game::Game) -> String { todo!() }

pub mod none_done {
    #[allow(unused_imports)]
    use crate::{ids::*, js::*, Quat, V3};
    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};
    /// (src/game/furnblast.ts:142 NONE_DONE.has)
    pub fn has(_j: f64) -> bool { todo!() }

}
