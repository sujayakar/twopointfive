#![allow(unused_variables)]
//! Every function and method of `destruction`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/destruction.ts:159 chaosOf)
pub fn chaos_of(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/destruction.ts:163 loudNow)
pub fn loud_now(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/destruction.ts:164 impulsesOn)
pub fn impulses_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/destruction.ts:165 loadChaos)
pub fn load_chaos(g: &mut crate::game::Game, tune: &mut crate::destruction::ChaosTune) { todo!() }

/// (src/game/destruction.ts:170 saveChaos)
pub fn save_chaos(g: &mut crate::game::Game) { todo!() }

/// (src/game/destruction.ts:171 resetChaosKnobs)
pub fn reset_chaos_knobs(g: &mut crate::game::Game) { todo!() }

/// (src/game/destruction.ts:174 impK)
pub fn imp_k(g: &mut crate::game::Game, chaos: f64) -> f64 { todo!() }

/// (src/game/destruction.ts:180 indoors)
pub fn indoors(p: V3) -> bool { todo!() }

/// (src/game/destruction.ts:182 topOf)
pub fn top_of(g: &mut crate::game::Game, b: BoxId) -> V3 { todo!() }

/// (src/game/destruction.ts:190 dlog)
pub fn dlog(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/destruction.ts:197 RateCap.constructor)
pub fn rate_cap_new(n: &crate::destruction::RateCapN) -> crate::destruction::RateCap { todo!() }

/// (src/game/destruction.ts:198 RateCap.take)
pub fn rate_cap_take(this: &mut crate::destruction::RateCap, t: f64, k: &crate::knobs::ChaosK) -> bool { todo!() }

/// (src/game/destruction.ts:204 RateCap.reset)
pub fn rate_cap_reset(this: &mut crate::destruction::RateCap) { todo!() }

/// (src/game/destruction.ts:242 Destruction.constructor)
pub fn destruction_new(g: &mut crate::game::Game) -> crate::destruction::Destruction { todo!() }

/// (src/game/destruction.ts:255 Destruction.index)
pub fn destruction_index(g: &mut crate::game::Game) { todo!() }

/// (src/game/destruction.ts:314 Destruction.roofY)
pub fn destruction_roof_y(g: &mut crate::game::Game, x: f64, z: f64) -> Option<f64> { todo!() }

/// (src/game/destruction.ts:323 Destruction.near)
pub fn destruction_near(g: &mut crate::game::Game, p: V3, out: &mut Vec<crate::destruction::Destructible>) -> Vec<crate::destruction::Destructible> { todo!() }

/// (src/game/destruction.ts:330 Destruction.inventory)
pub fn destruction_inventory(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/destruction.ts:331 Destruction.itemOf)
pub fn destruction_item_of(g: &mut crate::game::Game, i: f64) -> Option<crate::destruction::Destructible> { todo!() }

/// (src/game/destruction.ts:334 Destruction.hidx)
pub fn destruction_hidx(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/destruction.ts:336 Destruction.stoke)
pub fn destruction_stoke(g: &mut crate::game::Game, p: V3, amount: f64, chaos: f64) -> f64 { todo!() }

/// (src/game/destruction.ts:337 Destruction.heatAt)
pub fn destruction_heat_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/destruction.ts:338 Destruction.escalationAt)
pub fn destruction_escalation_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/destruction.ts:339 Destruction.hash)
pub fn destruction_hash(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/destruction.ts:342 Destruction.take)
pub fn destruction_take(g: &mut crate::game::Game, k: crate::destruction::DestructionTakeK) -> bool { todo!() }

/// (src/game/destruction.ts:345 Destruction.update)
pub fn destruction_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/destruction.ts:363 Destruction.onFoot)
pub fn destruction_on_foot(g: &mut crate::game::Game) -> Vec<CharId> { todo!() }

/// (src/game/destruction.ts:371 Destruction.boxes)
pub fn destruction_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }

/// (src/game/destruction.ts:374 Destruction.reset)
pub fn destruction_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/destruction.ts:386 Destruction.clearMoving)
pub fn destruction_clear_moving(g: &mut crate::game::Game) { todo!() }

/// (src/game/destruction.ts:391 Destruction.round)
pub fn destruction_round(g: &mut crate::game::Game, shooter: CharId, hit_p: V3, dir: V3, hit: &mut crate::destruction::DestructionRoundHit, def: &crate::weapons::WeaponDef) { todo!() }

/// (src/game/destruction.ts:441 Destruction.blast)
pub fn destruction_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::wallbreak::BlastOpts>) -> crate::wallbreak::BlastResult { todo!() }

/// (src/game/destruction.ts:442 Destruction.blastRaw)
pub fn destruction_blast_raw(g: &mut crate::game::Game, at: V3, opts: crate::wallbreak::BlastOpts) -> crate::wallbreak::BlastResult { todo!() }

/// (src/game/destruction.ts:458 Destruction.doorsFor)
pub fn destruction_doors_for(g: &mut crate::game::Game, at: V3, opts: Option<crate::destruction::DestructionDoorsForOpts>) -> Vec<DoorId> { todo!() }

/// (src/game/destruction.ts:468 Destruction.wallsFor)
pub fn destruction_walls_for(g: &mut crate::game::Game, at: V3, opts: Option<crate::destruction::DestructionWallsForOpts>) -> Vec<crate::wallbreak::WallOpening> { todo!() }

/// (src/game/destruction.ts:482 Destruction.withWallsOpen)
pub fn destruction_with_walls_open<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, walls: &[crate::wallbreak::WallOpening], fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/destruction.ts:491 Destruction.furnBlast)
pub fn destruction_furn_blast(g: &mut crate::game::Game, at: V3, src: crate::furnblast::BlastSource, who: Option<f64>) { todo!() }

/// (src/game/destruction.ts:493 Destruction.flags0)
pub fn destruction_flags0(g: &mut crate::game::Game, i: f64) -> Option<f64> { todo!() }

/// (src/game/destruction.ts:495 Destruction.onBlast)
pub fn destruction_on_blast(g: &mut crate::game::Game, fn_: &crate::destruction::BlastListener) { todo!() }

/// (src/game/destruction.ts:499 Destruction.dropItem)
pub fn destruction_drop_item(g: &mut crate::game::Game, i: f64, dirn: V3, who: Option<f64>, quiet: Option<bool>) -> bool { todo!() }

/// (src/game/destruction.ts:510 Destruction.breakFixtureBox)
pub fn destruction_break_fixture_box(g: &mut crate::game::Game, i: f64) { todo!() }

/// (src/game/destruction.ts:515 Destruction.bang)
pub fn destruction_bang(g: &mut crate::game::Game, at: V3, who: Option<f64>, power: Option<f64>, stun: Option<bool>) { todo!() }

/// (src/game/destruction.ts:548 Destruction.bangImpulse)
pub fn destruction_bang_impulse(g: &mut crate::game::Game, at: V3, eye: V3, chaos: f64, who: f64) { todo!() }

/// (src/game/destruction.ts:557 Destruction.impactImpulse)
pub fn destruction_impact_impulse(g: &mut crate::game::Game, hit_p: V3, dir: V3, n: V3, chaos: f64, who: f64) { todo!() }

/// (src/game/destruction.ts:568 Destruction.testImpulse)
pub fn destruction_test_impulse(g: &mut crate::game::Game, at: V3, kind: Option<crate::destruction::DestructionTestImpulseKind>, dir: Option<V3>, who: Option<f64>) { todo!() }

/// (src/game/destruction.ts:576 Destruction.thud)
pub fn destruction_thud(g: &mut crate::game::Game, pos: V3, scale: Option<f64>, who: Option<f64>) { todo!() }

/// (src/game/destruction.ts:588 Destruction.damageDecal)
pub fn destruction_damage_decal(g: &mut crate::game::Game, p: V3, n: V3, surf: crate::scene_materials::Surf, box_: BoxId, dynamic: bool, rifle: bool, item: Option<&crate::destruction::Destructible>) { todo!() }

/// (src/game/destruction.ts:609 Destruction.breakTarget)
pub fn destruction_break_target(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/destruction.ts:611 Destruction.killScreen)
pub fn destruction_kill_screen(g: &mut crate::game::Game, item: &crate::destruction::Destructible, hit_p: V3, n: V3, by_player: bool) { todo!() }

/// (src/game/destruction.ts:621 Destruction.papersNear)
pub fn destruction_papers_near(g: &mut crate::game::Game, hit_p: V3, dir: V3, e: f64, chaos: f64, k: f64, seg_len: f64) { todo!() }

/// (src/game/destruction.ts:640 Destruction.convertPaper)
pub fn destruction_convert_paper(g: &mut crate::game::Game, d: &crate::destruction::Destructible, dirn: V3, speed: f64, chaos: f64) -> bool { todo!() }

/// (src/game/destruction.ts:655 Destruction.hide)
pub fn destruction_hide(g: &mut crate::game::Game, d: &crate::destruction::Destructible) { todo!() }

/// (src/game/destruction.ts:662 Destruction.testBurst)
pub fn destruction_test_burst(g: &mut crate::game::Game, at: V3, surf: Option<crate::scene_materials::Surf>, albedo: Option<V3>) { todo!() }

/// (src/game/destruction.ts:663 Destruction.burst)
pub fn destruction_burst(g: &mut crate::game::Game, hit_p: V3, dir: V3, n: V3, surf: crate::scene_materials::Surf, albedo: V3, e: f64, k: f64, owner: f64, hit: Option<crate::destruction::DestructionBurstHit>) { todo!() }

/// (src/game/destruction.ts:695 Destruction.dustPuff)
pub fn destruction_dust_puff(g: &mut crate::game::Game, p: V3, n: V3, scale: f64, surf: crate::scene_materials::Surf) -> bool { todo!() }

/// (src/game/destruction.ts:709 Destruction.pallTake)
pub fn destruction_pall_take(g: &mut crate::game::Game, always: Option<bool>) -> bool { todo!() }

/// (src/game/destruction.ts:716 Destruction.pallHash)
pub fn destruction_pall_hash(g: &mut crate::game::Game, at: V3, k: f64) -> f64 { todo!() }

/// (src/game/destruction.ts:724 Destruction.pallEmit)
pub fn destruction_pall_emit(g: &mut crate::game::Game, r: crate::destruction::DestructionPallEmitR, from: V3, pos: V3, dir: V3, j: f64, tint: V3, gain: Option<f64>) -> bool { todo!() }

/// (src/game/destruction.ts:739 Destruction.pallWall)
pub fn destruction_pall_wall(g: &mut crate::game::Game, p: V3, n: V3, t: V3, thick: f64, through: bool, surf: Option<crate::scene_materials::Surf>, gain: Option<f64>) { todo!() }

/// (src/game/destruction.ts:753 Destruction.pallBurst)
pub fn destruction_pall_burst(g: &mut crate::game::Game, at: V3, size: Option<f64>) { todo!() }

/// (src/game/destruction.ts:768 Destruction.cellDust)
pub fn destruction_cell_dust(g: &mut crate::game::Game, p: V3, n: V3, scale: f64, surf: crate::scene_materials::Surf) { todo!() }

/// (src/game/destruction.ts:778 Destruction.dustBurst)
pub fn destruction_dust_burst(g: &mut crate::game::Game, at: V3, n: V3, count: f64, scale: f64, surf: crate::scene_materials::Surf, spread: Option<f64>) { todo!() }

/// (src/game/destruction.ts:791 Destruction.clutterNear)
pub fn destruction_clutter_near(g: &mut crate::game::Game, hit_p: V3, dir: V3, chaos: f64, who: f64) { todo!() }

/// (src/game/destruction.ts:801 Destruction.knockOff)
pub fn destruction_knock_off(g: &mut crate::game::Game, d: &crate::destruction::Destructible, dirn: V3, chaos: f64, kind: crate::debris::ChunkKind, who: Option<f64>) -> bool { todo!() }

/// (src/game/destruction.ts:822 Destruction.tileDrop)
pub fn destruction_tile_drop(g: &mut crate::game::Game, p: V3, chaos: f64, force: Option<bool>, who: Option<f64>, panels: Option<bool>) -> bool { todo!() }

/// (src/game/destruction.ts:860 Destruction.fixtureLive)
pub fn destruction_fixture_live(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/destruction.ts:863 Destruction.killFixture)
pub fn destruction_kill_fixture(g: &mut crate::game::Game, t: FixtureId, from: V3, why: &str, force: Option<bool>) -> bool { todo!() }

/// (src/game/destruction.ts:873 Destruction.fixtureRound)
pub fn destruction_fixture_round(g: &mut crate::game::Game, muzzle: V3, dir: V3, t_hit: f64, index: Option<f64>) { todo!() }

/// (src/game/destruction.ts:885 Destruction.summary)
pub fn destruction_summary(g: &mut crate::game::Game) -> String { todo!() }
