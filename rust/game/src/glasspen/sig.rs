#![allow(unused_variables)]
//! Every function and method of `glasspen`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/glasspen.ts:116 loadGlassPen)
pub fn load_glass_pen(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:117 saveGlassPen)
pub fn save_glass_pen(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:119 glassPenHarness)
pub fn glass_pen_harness(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:122 glassPenLive)
pub fn glass_pen_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/glasspen.ts:125 roundEnergy)
pub fn round_energy(g: &mut crate::game::Game, def: crate::weapons::WeaponId) -> f64 { todo!() }

/// (src/game/glasspen.ts:127 roundEnergyOf)
pub fn round_energy_of(g: &mut crate::game::Game, c_2: CharId) -> f64 { todo!() }

/// (src/game/glasspen.ts:136 sideShotEnergy)
pub fn side_shot_energy(g: &mut crate::game::Game, team: crate::teams::Team) -> Option<f64> { todo!() }

/// (src/game/glasspen.ts:141 shotEnergy)
pub fn shot_energy(g: &mut crate::game::Game, c_2: CharId, def: Option<crate::weapons::WeaponId>) -> Option<f64> { todo!() }

/// (src/game/glasspen.ts:148 paneType)
pub fn pane_type(g: &mut crate::game::Game, b: BoxId, bi: Option<f64>) -> crate::glasspen::PaneType { todo!() }

/// (src/game/glasspen.ts:153 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::glasspen::PenState { todo!() }

/// (src/game/glasspen.ts:155 resetGlassPen)
pub fn reset_glass_pen(g: &mut crate::game::Game) { todo!() }

/// (src/game/glasspen.ts:158 glassTollSig)
pub fn glass_toll_sig(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/glasspen.ts:159 addHole)
pub fn add_hole(g: &mut crate::game::Game, bi: f64, p: V3) { todo!() }

/// (src/game/glasspen.ts:176 holeCount)
pub fn hole_count(g: &mut crate::game::Game, bi: f64) -> f64 { todo!() }

/// (src/game/glasspen.ts:182 paneToll)
pub fn pane_toll(g: &mut crate::game::Game, bi: f64, at: V3, d: V3, e: f64, nth: Option<f64>) -> Option<crate::glasspen::PaneToll> { todo!() }

/// (src/game/glasspen.ts:188 crossAt)
pub fn cross_at(m: &[f64], o: f64, into: f64) -> crate::glasspen::PaneCross { todo!() }

/// (src/game/glasspen.ts:206 paneCrossing)
pub fn pane_crossing(g: &mut crate::game::Game, bi: f64, o: V3, d: V3, t: f64, e: f64, nth: f64) -> Option<crate::glasspen::PaneCross> { todo!() }

/// (src/game/glasspen.ts:226 roundPath)
pub fn round_path(g: &mut crate::game::Game, from: V3, dir: V3, range: f64, energy: f64, first: Option<Option<crate::glasspen::RoundPathSt>>, nth0: Option<f64>) -> Option<crate::glasspen::RoundPath> { todo!() }

/// (src/game/glasspen.ts:239 segAt)
pub fn seg_at(p: &crate::glasspen::RoundPath, t: f64) -> crate::glasspen::RoundSeg { todo!() }

/// (src/game/glasspen.ts:241 pathAt)
pub fn path_at(p: &crate::glasspen::RoundPath, t: f64) -> V3 { todo!() }

/// (src/game/glasspen.ts:247 paneImpact)
pub fn pane_impact(g: &mut crate::game::Game, bi: f64, at: V3, dir: V3, energy: f64, who: f64, through: bool, quiet: Option<bool>) { todo!() }

/// (src/game/glasspen.ts:254 shellMark)
pub fn shell_mark(g: &mut crate::game::Game, shell: &mut crate::glasspen::ShellGlass, bi: f64, at: V3, through: bool) -> crate::glasspen::ShellMark { todo!() }

/// (src/game/glasspen.ts:266 commitCrossings)
pub fn commit_crossings(g: &mut crate::game::Game, cross: &[crate::glasspen::PaneCross], t_hit: f64, who: f64, shell: Option<&mut crate::glasspen::ShellGlass>) { todo!() }

/// (src/game/glasspen.ts:281 spentInPane)
pub fn spent_in_pane(g: &mut crate::game::Game, bi: f64, at: V3, dir: V3, energy: f64, who: f64, shell: Option<&mut crate::glasspen::ShellGlass>) { todo!() }

/// (src/game/glasspen.ts:296 armThroughGlass)
pub fn arm_through_glass(g: &mut crate::game::Game, c_2: CharId, dir: V3, energy: f64, shell: Option<&mut crate::glasspen::ShellGlass>) -> Option<crate::glasspen::ArmThroughGlass> { todo!() }

/// (src/game/glasspen.ts:308 shotPasses)
pub fn shot_passes(g: &mut crate::game::Game, a: V3, rd: V3, panes: &[crate::scene_collision::SegmentBlockedPanesItem], energy: Option<f64>) -> bool { todo!() }

/// (src/game/glasspen.ts:318 shotThroughGlass)
pub fn shot_through_glass(g: &mut crate::game::Game, eyes: &[V3], p: V3, energy: Option<f64>) -> f64 { todo!() }

/// (src/game/glasspen.ts:337 glassOnLine)
pub fn glass_on_line(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/glasspen.ts:344 glassPenSummary)
pub fn glass_pen_summary(g: &mut crate::game::Game) -> String { todo!() }
