#![allow(unused_variables)]
//! Every function and method of `winbars`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/winbars.ts:63 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::winbars::BarState { todo!() }

/// (src/game/winbars.ts:66 barsLive)
pub fn bars_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/winbars.ts:68 barAt)
pub fn bar_at(g: &mut crate::game::Game, index: f64) -> bool { todo!() }

/// (src/game/winbars.ts:70 barsStanding)
pub fn bars_standing(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/winbars.ts:72 barsGone)
pub fn bars_gone(g: &mut crate::game::Game) -> Vec<f64> { todo!() }

/// (src/game/winbars.ts:75 barFlags0)
pub fn bar_flags0(g: &mut crate::game::Game, index: f64) -> Option<f64> { todo!() }

/// (src/game/winbars.ts:78 bootBars)
pub fn boot_bars(g: &mut crate::game::Game) { todo!() }

/// (src/game/winbars.ts:88 barMeet)
pub fn bar_meet(g: &mut crate::game::Game, index: f64, t: f64, at: V3, dir: V3) -> crate::winbars::BarMet { todo!() }

/// (src/game/winbars.ts:93 barsUnmeet)
pub fn bars_unmeet(g: &mut crate::game::Game) { todo!() }

/// (src/game/winbars.ts:96 barsSettle)
pub fn bars_settle(g: &mut crate::game::Game, met: &[crate::winbars::BarMet], t_hit: f64, who: f64, lead: bool) -> f64 { todo!() }

/// (src/game/winbars.ts:110 breakBar)
pub fn break_bar(g: &mut crate::game::Game, index: f64, at: V3, dir: V3, who: f64, cause: crate::winbars::BreakBarCause) -> bool { todo!() }

/// (src/game/winbars.ts:135 changed)
pub fn changed(g: &mut crate::game::Game, b: BoxId) { todo!() }

/// (src/game/winbars.ts:146 barsBlast)
pub fn bars_blast(g: &mut crate::game::Game, ev: crate::wallbreak::BlastEvent) { todo!() }

/// (src/game/winbars.ts:168 resetBars)
pub fn reset_bars(g: &mut crate::game::Game) { todo!() }

/// (src/game/winbars.ts:177 barsCheck)
pub fn bars_check(g: &mut crate::game::Game, s: Option<&crate::winbars::BarsSave>) -> Option<String> { todo!() }

/// (src/game/winbars.ts:190 winBarsSummary)
pub fn win_bars_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/winbars.ts:195 winBarsLog)
pub fn win_bars_log(g: &mut crate::game::Game) -> Vec<String> { todo!() }
