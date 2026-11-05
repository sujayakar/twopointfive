#![allow(unused_variables)]
//! Every function and method of `glassbreak`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/glassbreak.ts:127 glassBreakPersisted)
pub fn glass_break_persisted() -> bool { todo!() }

/// (src/game/glassbreak.ts:129 loadGlassBreak)
pub fn load_glass_break(g: &mut crate::game::Game, tune: &mut crate::glassbreak::GbTune) { todo!() }

/// (src/game/glassbreak.ts:139 saveGlassBreak)
pub fn save_glass_break(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:143 glassBreakLive)
pub fn glass_break_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/glassbreak.ts:196 tally)
pub fn tally(i: Option<f64>) -> f64 { todo!() }

/// (src/game/glassbreak.ts:203 panesUntouched)
pub fn panes_untouched(b: crate::Opaque) -> bool { todo!() }

/// (src/game/glassbreak.ts:214 fields)
pub fn fields(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/glassbreak.ts:226 GlassBreak.constructor)
pub fn glass_break_new(g: &mut crate::game::Game) -> crate::glassbreak::GlassBreak { todo!() }

/// (src/game/glassbreak.ts:241 GlassBreak.asking)
pub fn glass_break_asking<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> T) -> T { todo!() }

/// (src/game/glassbreak.ts:242 GlassBreak.ask)
pub fn glass_break_ask<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> T) -> T { todo!() }

/// (src/game/glassbreak.ts:256 GlassBreak.sheets)
pub fn glass_break_sheets(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/glassbreak.ts:262 GlassBreak.stood)
pub fn glass_break_stood(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:268 GlassBreak.doorGap)
pub fn glass_break_door_gap(g: &mut crate::game::Game, b: BoxId) -> f64 { todo!() }

/// (src/game/glassbreak.ts:276 GlassBreak.drain)
pub fn glass_break_drain(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:293 GlassBreak.letGo)
pub fn glass_break_let_go(g: &mut crate::game::Game, f: &crate::glassbreak::GlassBreakLetGoF) { todo!() }

/// (src/game/glassbreak.ts:306 GlassBreak.geom)
pub fn glass_break_geom(g: &mut crate::game::Game, bi: f64) -> crate::glassbreak::PaneFrame { todo!() }

/// (src/game/glassbreak.ts:311 GlassBreak.world)
pub fn glass_break_world(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, u: f64, v_2: f64, w: Option<f64>) -> V3 { todo!() }

/// (src/game/glassbreak.ts:315 GlassBreak.uv)
pub fn glass_break_uv(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, x: V3) -> [f64; 2] { todo!() }

/// (src/game/glassbreak.ts:320 GlassBreak.standIn)
pub fn glass_break_stand_in(g: &mut crate::game::Game, v: crate::glassbreak::PaneFrame, f: &crate::glassbreak::GlassFragment, key: f64, bi: f64) { todo!() }

/// (src/game/glassbreak.ts:342 GlassBreak.typeOf)
pub fn glass_break_type_of(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glasspen::PaneType { todo!() }

/// (src/game/glassbreak.ts:346 GlassBreak.typeOfPane)
pub fn glass_break_type_of_pane(g: &mut crate::game::Game, bi: f64) -> Option<crate::glasspen::PaneType> { todo!() }

/// (src/game/glassbreak.ts:350 GlassBreak.setType)
pub fn glass_break_set_type(g: &mut crate::game::Game, bi: f64, type_: Option<crate::glasspen::PaneType>) { todo!() }

/// (src/game/glassbreak.ts:358 GlassBreak.impact)
pub fn glass_break_impact(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, energy: f64, who: f64, through: Option<bool>) -> crate::glassbreak::GlassImpact { todo!() }

/// (src/game/glassbreak.ts:368 GlassBreak.hit)
pub fn glass_break_hit(g: &mut crate::game::Game, bi: f64, x: V3, dir: Option<V3>, who: f64, energy: Option<f64>) { todo!() }

/// (src/game/glassbreak.ts:370 GlassBreak.widen)
pub fn glass_break_widen(g: &mut crate::game::Game, bi: f64, x: V3) -> f64 { todo!() }

/// (src/game/glassbreak.ts:376 GlassBreak.blast)
pub fn glass_break_blast(g: &mut crate::game::Game, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>) -> f64 { todo!() }

/// (src/game/glassbreak.ts:387 GlassBreak.bang)
pub fn glass_break_bang(g: &mut crate::game::Game, at: V3, who: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:392 GlassBreak.home)
pub fn glass_break_home(g: &mut crate::game::Game) -> Option<crate::Opaque> { todo!() }

/// (src/game/glassbreak.ts:394 GlassBreak.seeThrough)
pub fn glass_break_see_through<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, view: crate::glassbreak::GlassBreakSeeThroughView, from: V3, size: crate::glassbreak::GlassBlastOpts, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/glassbreak.ts:399 GlassBreak.view)
pub fn glass_break_view(g: &mut crate::game::Game) -> Option<crate::scene_box_mirror::PaneView> { todo!() }

/// (src/game/glassbreak.ts:404 GlassBreak.ruleOpen)
pub fn glass_break_rule_open(g: &mut crate::game::Game, bi: f64, p: V3) -> bool { todo!() }

/// (src/game/glassbreak.ts:410 GlassBreak.update)
pub fn glass_break_update(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/glassbreak.ts:429 GlassBreak.updateGusts)
pub fn glass_break_update_gusts(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:455 GlassBreak.untouched)
pub fn glass_break_untouched(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/glassbreak.ts:457 GlassBreak.openAt)
pub fn glass_break_open_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool { todo!() }

/// (src/game/glassbreak.ts:459 GlassBreak.walkable)
pub fn glass_break_walkable(g: &mut crate::game::Game, bi: f64) -> bool { todo!() }

/// (src/game/glassbreak.ts:461 GlassBreak.openSig)
pub fn glass_break_open_sig(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glassbreak.ts:463 GlassBreak.openFraction)
pub fn glass_break_open_fraction(g: &mut crate::game::Game, bi: f64) -> f64 { todo!() }

/// (src/game/glassbreak.ts:465 GlassBreak.paneInfo)
pub fn glass_break_pane_info(g: &mut crate::game::Game, bi: f64) -> Option<crate::glassbreak::GlassBreakPaneInfo> { todo!() }

/// (src/game/glassbreak.ts:470 GlassBreak.panes)
pub fn glass_break_panes(g: &mut crate::game::Game) -> JsMap<crate::Num, crate::glassbreak::PaneState> { todo!() }

/// (src/game/glassbreak.ts:477 GlassBreak.handle)
pub fn glass_break_handle(g: &mut crate::game::Game, bi: f64) -> crate::glassbreak::PaneState { todo!() }

/// (src/game/glassbreak.ts:482 GlassBreak.rowState)
pub fn glass_break_row_state(g: &mut crate::game::Game, s: f64, r: f64) -> crate::glassbreak::PaneState { todo!() }

/// (src/game/glassbreak.ts:503 GlassBreak.heldSet)
pub fn glass_break_held_set(g: &mut crate::game::Game, without: Option<f64>, bi: f64) -> JsSet<crate::Num> { todo!() }

/// (src/game/glassbreak.ts:509 GlassBreak.toUV)
pub fn glass_break_to_uv(g: &mut crate::game::Game, x: V3, bi: f64) -> [f64; 2] { todo!() }

/// (src/game/glassbreak.ts:514 GlassBreak.toWorld)
pub fn glass_break_to_world(g: &mut crate::game::Game, u: f64, v: f64, w: Option<f64>, bi: f64) -> V3 { todo!() }

/// (src/game/glassbreak.ts:520 GlassBreak.normalOf)
pub fn glass_break_normal_of(g: &mut crate::game::Game, bi: f64) -> V3 { todo!() }

/// (src/game/glassbreak.ts:522 GlassBreak.goingAt)
pub fn glass_break_going_at(g: &mut crate::game::Game, bi: f64, x: V3) -> bool { todo!() }

/// (src/game/glassbreak.ts:523 GlassBreak.wouldGive)
pub fn glass_break_would_give(g: &mut crate::game::Game, bi: f64, at: V3, opts: Option<crate::glassbreak::GlassBlastOpts>, x: Option<V3>) -> bool { todo!() }

/// (src/game/glassbreak.ts:532 GlassBreak.fill)
pub fn glass_break_fill(g: &mut crate::game::Game, bi: f64, dst: &mut Vec<f32>, off: f64) -> bool { todo!() }

/// (src/game/glassbreak.ts:554 GlassBreak.breakTest)
pub fn glass_break_break_test(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glassbreak.ts:568 GlassBreak.summary)
pub fn glass_break_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/glassbreak.ts:574 GlassBreak.reset)
pub fn glass_break_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/glassbreak.ts:581 s0Bi)
pub fn s0_bi(g: &mut crate::game::Game, s: f64, r: f64) -> f64 { todo!() }
