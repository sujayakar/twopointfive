#![allow(unused_variables)]
//! Every function and method of `glassbreak`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/glassbreak.ts:124 glassBreakPersisted)
pub fn glass_break_persisted() -> bool { todo!() }

/// (src/game/glassbreak.ts:126 loadGlassBreak)
pub fn load_glass_break(g: &mut crate::game::Game, tune: &mut crate::glassbreak::GbTune) { todo!() }

/// (src/game/glassbreak.ts:136 saveGlassBreak)
pub fn save_glass_break(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:140 glassBreakLive)
pub fn glass_break_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/glassbreak.ts:180 blastReach)
pub fn blast_reach(k: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:209 GlassBreak.constructor)
pub fn glass_break_new(g: &mut crate::game::Game) -> crate::glassbreak::GlassBreak { todo!() }

/// (src/game/glassbreak.ts:223 GlassBreak.note)
pub fn glass_break_note(g: &mut crate::game::Game, s: &str) { todo!() }

/// (src/game/glassbreak.ts:224 GlassBreak.plog)
pub fn glass_break_plog(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, s: &str) { todo!() }

/// (src/game/glassbreak.ts:231 GlassBreak.skySide)
pub fn glass_break_sky_side(g: &mut crate::game::Game, bi: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:243 GlassBreak.exteriorOf)
pub fn glass_break_exterior_of(g: &mut crate::game::Game, bi: f64) -> bool { todo!() }

/// (src/game/glassbreak.ts:247 GlassBreak.typeOf)
pub fn glass_break_type_of(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glassbreak::GlassType { todo!() }

/// (src/game/glassbreak.ts:257 GlassBreak.doorGap)
pub fn glass_break_door_gap(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/glassbreak.ts:265 GlassBreak.typeOfPane)
pub fn glass_break_type_of_pane(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassType> { todo!() }

/// (src/game/glassbreak.ts:267 GlassBreak.setType)
pub fn glass_break_set_type(g: &mut crate::game::Game, bi: f64, type_: Option<crate::glassbreak::GlassType>) { todo!() }

/// (src/game/glassbreak.ts:268 GlassBreak.stateOf)
pub fn glass_break_state_of<'g>(g: &'g mut crate::game::Game, bi: f64) -> Option<&'g mut crate::glassbreak::PaneState> { todo!() }

/// (src/game/glassbreak.ts:279 GlassBreak.toUV)
pub fn glass_break_to_uv(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, x: V3) -> [f64; 2] { todo!() }

/// (src/game/glassbreak.ts:285 GlassBreak.toUVIn)
pub fn glass_break_to_uv_in(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, x: V3) -> [f64; 2] { todo!() }

/// (src/game/glassbreak.ts:287 GlassBreak.toWorld)
pub fn glass_break_to_world(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, u: f64, v: f64, w: Option<f64>) -> V3 { todo!() }

/// (src/game/glassbreak.ts:293 GlassBreak.normalOf)
pub fn glass_break_normal_of(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState) -> V3 { todo!() }

/// (src/game/glassbreak.ts:294 GlassBreak.fragsOf)
pub fn glass_break_frags_of(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) -> Vec<crate::glass_pattern::GpFrag> { todo!() }

/// (src/game/glassbreak.ts:298 GlassBreak.addSite)
pub fn glass_break_add_site(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, u: f64, v: f64, e: f64, seed32: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:313 GlassBreak.nearestSite)
pub fn glass_break_nearest_site(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, u: f64, v: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:322 GlassBreak.impact)
pub fn glass_break_impact(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact { todo!() }

/// (src/game/glassbreak.ts:325 GlassBreak.impactIn)
pub fn glass_break_impact_in(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact { todo!() }

/// (src/game/glassbreak.ts:374 GlassBreak.hit)
pub fn glass_break_hit(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, who: f64, energy: Option<f64>) { todo!() }

/// (src/game/glassbreak.ts:378 GlassBreak.widen)
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 { todo!() }

/// (src/game/glassbreak.ts:387 GlassBreak.knockOut)
pub fn glass_break_knock_out(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, fid: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:394 GlassBreak.setRun)
pub fn glass_break_set_run(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, why: &str) { todo!() }

/// (src/game/glassbreak.ts:403 GlassBreak.blast)
pub fn glass_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>) -> f64 { todo!() }

/// (src/game/glassbreak.ts:407 GlassBreak.blastIn)
pub fn glass_break_blast_in(g: &mut crate::game::Game, at: V3, opts: crate::glassbreak::GlassBlastOpts) -> f64 { todo!() }

/// (src/game/glassbreak.ts:459 GlassBreak.each)
pub fn glass_break_each(g: &mut crate::game::Game, to: &mut dyn FnMut(&mut crate::game::Game, crate::scene_box_mirror::PaneRow)) { todo!() }

/// (src/game/glassbreak.ts:464 GlassBreak.seeThrough)
pub fn glass_break_see_through<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, view: crate::glassbreak::GlassBreakSeeThroughView, from: V3, size: crate::glassbreak::GlassBlastOpts, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/glassbreak.ts:472 GlassBreak.view)
pub fn glass_break_view(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneView> { todo!() }

/// (src/game/glassbreak.ts:480 GlassBreak.gives)
pub fn glass_break_gives(g: &mut crate::game::Game, at: V3, opts: crate::glassbreak::GlassBlastOpts, sure: bool) -> Option<crate::scene_box_mirror::PaneView> { todo!() }

/// (src/game/glassbreak.ts:495 GlassBreak.give)
pub fn glass_break_give(g: &mut crate::game::Game, bi: f64, b: BoxId, at: V3, radius: f64, power: f64, sure: bool, q: &mut V3) -> f64 { todo!() }

/// (src/game/glassbreak.ts:508 GlassBreak.goingAt)
pub fn glass_break_going_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool { todo!() }

/// (src/game/glassbreak.ts:520 GlassBreak.wouldGive)
pub fn glass_break_would_give(g: &mut crate::game::Game, bi: f64, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>, x: Option<V3>) -> bool { todo!() }

/// (src/game/glassbreak.ts:530 GlassBreak.bang)
pub fn glass_break_bang(g: &mut crate::game::Game, at: V3, who: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:533 GlassBreak.startDice)
pub fn glass_break_start_dice(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, u: f64, v: f64, seed: f64, why: &str) { todo!() }

/// (src/game/glassbreak.ts:545 GlassBreak.diceDone)
pub fn glass_break_dice_done(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) { todo!() }

/// (src/game/glassbreak.ts:546 GlassBreak.stepDice)
pub fn glass_break_step_dice(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) { todo!() }

/// (src/game/glassbreak.ts:553 GlassBreak.releaseClump)
pub fn glass_break_release_clump(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, d: &mut crate::glassbreak::Dice, c_2: crate::glassbreak::Clump) { todo!() }

/// (src/game/glassbreak.ts:568 GlassBreak.dropAll)
pub fn glass_break_drop_all(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, d: &mut crate::glassbreak::Dice) { todo!() }

/// (src/game/glassbreak.ts:587 GlassBreak.releaseSheet)
pub fn glass_break_release_sheet(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, away: V3, k: f64) { todo!() }

/// (src/game/glassbreak.ts:598 GlassBreak.dirUV)
pub fn glass_break_dir_uv(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, du: f64, dv: f64) -> V3 { todo!() }

/// (src/game/glassbreak.ts:600 GlassBreak.outline)
pub fn glass_break_outline(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, pts: &[[f64; 2]], out_sign: f64) -> Vec<V3> { todo!() }

/// (src/game/glassbreak.ts:606 GlassBreak.releaseCrater)
pub fn glass_break_release_crater(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, i: f64, cause: crate::glassbreak::GlassBreakReleaseCraterCause) -> f64 { todo!() }

/// (src/game/glassbreak.ts:621 GlassBreak.releaseFrag)
pub fn glass_break_release_frag(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, f: &crate::glass_pattern::GpFrag, cause: crate::glassbreak::GlassBreakReleaseFragCause, from: Option<V3>, k: f64) { todo!() }

/// (src/game/glassbreak.ts:635 GlassBreak.afterRelease)
pub fn glass_break_after_release(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) { todo!() }

/// (src/game/glassbreak.ts:637 GlassBreak.emit)
pub fn glass_break_emit(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, f: &crate::glassbreak::GlassFragment, key: f64) { todo!() }

/// (src/game/glassbreak.ts:649 GlassBreak.standIn)
pub fn glass_break_stand_in(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, f: &crate::glassbreak::GlassFragment, key: f64) { todo!() }

/// (src/game/glassbreak.ts:671 GlassBreak.heldSet)
pub fn glass_break_held_set(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) -> JsSet<crate::Num> { todo!() }

/// (src/game/glassbreak.ts:693 GlassBreak.supportPass)
pub fn glass_break_support_pass(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) { todo!() }

/// (src/game/glassbreak.ts:705 GlassBreak.update)
pub fn glass_break_update(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/glassbreak.ts:734 GlassBreak.rake)
pub fn glass_break_rake(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) { todo!() }

/// (src/game/glassbreak.ts:735 GlassBreak.rakeBy)
pub fn glass_break_rake_by(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState, pl: PlayerId) { todo!() }

/// (src/game/glassbreak.ts:756 GlassBreak.walkableNow)
pub fn glass_break_walkable_now(g: &mut crate::game::Game, st: &mut crate::glassbreak::PaneState) -> bool { todo!() }

/// (src/game/glassbreak.ts:768 GlassBreak.updateGusts)
pub fn glass_break_update_gusts(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:791 GlassBreak.openUV)
pub fn glass_break_open_uv(g: &mut crate::game::Game, st: &crate::glassbreak::PaneState, u: f64, v: f64) -> bool { todo!() }

/// (src/game/glassbreak.ts:800 GlassBreak.openAt)
pub fn glass_break_open_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool { todo!() }

/// (src/game/glassbreak.ts:802 GlassBreak.walkable)
pub fn glass_break_walkable(g: &mut crate::game::Game, bi: f64) -> bool { todo!() }

/// (src/game/glassbreak.ts:804 GlassBreak.openSig)
pub fn glass_break_open_sig(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glassbreak.ts:806 GlassBreak.openFraction)
pub fn glass_break_open_fraction(g: &mut crate::game::Game, bi: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:814 GlassBreak.paneInfo)
pub fn glass_break_pane_info(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassBreakPaneInfo> { todo!() }

/// (src/game/glassbreak.ts:823 GlassBreak.fill)
pub fn glass_break_fill(g: &mut crate::game::Game, bi: f64, dst: &mut Vec<f32>, off: f64) -> bool { todo!() }

/// (src/game/glassbreak.ts:840 GlassBreak.breakTest)
pub fn glass_break_break_test(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glassbreak.ts:854 GlassBreak.summary)
pub fn glass_break_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/glassbreak.ts:860 GlassBreak.reset)
pub fn glass_break_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:862 nearestOn)
pub fn nearest_on(g: &mut crate::game::Game, b: BoxId, p: V3) -> V3 { todo!() }
