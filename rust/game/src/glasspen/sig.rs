#![allow(unused_variables)]
//! Every function and method of `glasspen`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/glasspen.ts:115 loadGlassPen)
pub fn load_glass_pen(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:116 saveGlassPen)
pub fn save_glass_pen(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:118 glassPenHarness)
pub fn glass_pen_harness(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:121 glassPenLive)
pub fn glass_pen_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/glasspen.ts:124 roundEnergy)
pub fn round_energy(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> f64 { todo!() }

/// (src/game/glasspen.ts:126 roundEnergyOf)
pub fn round_energy_of(g: &mut crate::game::Game, c_2: CharId) -> f64 { todo!() }

/// (src/game/glasspen.ts:135 sideShotEnergy)
pub fn side_shot_energy(g: &mut crate::game::Game, team: crate::teams::Team) -> Option<f64> { todo!() }

/// (src/game/glasspen.ts:140 shotEnergy)
pub fn shot_energy(g: &mut crate::game::Game, c_2: CharId, def: Option<&crate::weapons::WeaponDef>) -> Option<f64> { todo!() }

/// (src/game/glasspen.ts:147 paneType)
pub fn pane_type(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glassbreak::GlassType { todo!() }

/// (src/game/glasspen.ts:149 paneN)
pub fn pane_n(g: &mut crate::game::Game, b: BoxId) -> V3 { todo!() }

/// (src/game/glasspen.ts:156 bornIn)
pub fn born_in(g: &mut crate::game::Game, p: V3, d: V3, self_: f64) -> f64 { todo!() }

/// (src/game/glasspen.ts:169 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::glasspen::PenState { todo!() }

/// (src/game/glasspen.ts:171 resetGlassPen)
pub fn reset_glass_pen(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:174 glassTollSig)
pub fn glass_toll_sig(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glasspen.ts:175 nearHole)
pub fn near_hole(g: &mut crate::game::Game, bi: f64, p: V3) -> bool { todo!() }

/// (src/game/glasspen.ts:180 addHole)
pub fn add_hole(g: &mut crate::game::Game, bi: f64, p: V3) { todo!() }

/// (src/game/glasspen.ts:195 holeCount)
pub fn hole_count(g: &mut crate::game::Game, bi: f64) -> f64 { todo!() }

/// (src/game/glasspen.ts:201 paneToll)
pub fn pane_toll(g: &mut crate::game::Game, bi: f64, at: V3, d: V3, e: f64, nth: Option<f64>) -> Option<crate::glasspen::PaneToll> { todo!() }

/// (src/game/glasspen.ts:224 deflect)
pub fn deflect(g: &mut crate::game::Game, b: BoxId, at: V3, d: V3, t: crate::glasspen::PaneToll, e_in: f64) -> V3 { todo!() }

/// (src/game/glasspen.ts:237 paneCrossing)
pub fn pane_crossing(g: &mut crate::game::Game, bi: f64, o: V3, d: V3, t: f64, e: f64, nth: f64) -> Option<crate::glasspen::PaneCross> { todo!() }

/// (src/game/glasspen.ts:260 roundPath)
pub fn round_path(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<crate::glasspen::StaticHit2>, nth0: Option<f64>) -> Option<crate::glasspen::RoundPath> { todo!() }

/// (src/game/glasspen.ts:276 segAt)
pub fn seg_at(p: &crate::glasspen::RoundPath, t: f64) -> crate::glasspen::RoundSeg { todo!() }

/// (src/game/glasspen.ts:278 pathAt)
pub fn path_at(p: &crate::glasspen::RoundPath, t: f64) -> V3 { todo!() }

/// (src/game/glasspen.ts:284 paneImpact)
pub fn pane_impact(g: &mut crate::game::Game, bi: f64, at: V3, dir: V3, energy: f64, who: f64, through: bool, quiet: Option<bool>) { todo!() }

/// (src/game/glasspen.ts:291 shellMark)
pub fn shell_mark(g: &mut crate::game::Game, shell: &mut crate::glasspen::ShellGlass, bi: f64, at: V3, through: bool) -> crate::glasspen::ShellMark { todo!() }

/// (src/game/glasspen.ts:303 commitCrossings)
pub fn commit_crossings(g: &mut crate::game::Game, cross: &[crate::glasspen::PaneCross], t_hit: f64, who: f64, shell: Option<&mut crate::glasspen::ShellGlass>) { todo!() }

/// (src/game/glasspen.ts:318 spentInPane)
pub fn spent_in_pane(g: &mut crate::game::Game, bi: f64, at: V3, dir: V3, energy: f64, who: f64, shell: Option<&mut crate::glasspen::ShellGlass>) { todo!() }

/// (src/game/glasspen.ts:326 armInHole)
pub fn arm_in_hole(g: &mut crate::game::Game, from: V3, rd: V3, l: f64) -> bool { todo!() }

/// (src/game/glasspen.ts:343 armThroughGlass)
pub fn arm_through_glass(g: &mut crate::game::Game, c_2: CharId, dir: V3, energy: f64, shell: Option<&mut crate::glasspen::ShellGlass>) -> Option<crate::glasspen::ArmThroughGlass> { todo!() }

/// (src/game/glasspen.ts:361 shotPasses)
pub fn shot_passes(g: &mut crate::game::Game, a: V3, rd: V3, panes: &[crate::glasspen::ShotPassesPanesItem], energy: Option<f64>) -> bool { todo!() }

/// (src/game/glasspen.ts:371 shotThroughGlass)
pub fn shot_through_glass(g: &mut crate::game::Game, eyes: &[V3], p: V3, energy: Option<f64>) -> f64 { todo!() }

/// (src/game/glasspen.ts:390 glassOnLine)
pub fn glass_on_line(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/glasspen.ts:397 glassPenSummary)
pub fn glass_pen_summary(g: &mut crate::game::Game) -> String { todo!() }
