#![allow(unused_variables)]
//! Every function and method of `defuse`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/defuse.ts:54 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/defuse.ts:56 who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/defuse.ts:57 mmss)
pub fn mmss(s: f64) -> String { todo!() }

/// (src/game/defuse.ts:58 n3)
pub fn n3(d: &crate::scene_scenarios::DefuseDevice) -> V3 { todo!() }

/// (src/game/defuse.ts:81 Defuse.constructor)
pub fn defuse_new(g: &mut crate::game::Game) -> crate::defuse::Defuse { todo!() }

/// (src/game/defuse.ts:84 Defuse.data)
pub fn defuse_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::DefuseData> { todo!() }

/// (src/game/defuse.ts:91 Defuse.mine)
pub fn defuse_mine(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/defuse.ts:92 Defuse.active)
pub fn defuse_active(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/defuse.ts:93 Defuse.started)
pub fn defuse_started(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/defuse.ts:95 Defuse.workSecs)
pub fn defuse_work_secs(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/defuse.ts:97 Defuse.left)
pub fn defuse_left(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/defuse.ts:99 Defuse.won)
pub fn defuse_won(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/defuse.ts:103 Defuse.key)
pub fn defuse_key(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/defuse.ts:105 Defuse.armed)
pub fn defuse_armed(g: &mut crate::game::Game) -> Vec<crate::defuse::DefuseArmedItem> { todo!() }

/// (src/game/defuse.ts:110 Defuse.moot)
pub fn defuse_moot(g: &mut crate::game::Game, i: f64) -> Option<String> { todo!() }

/// (src/game/defuse.ts:113 Defuse.tick)
pub fn defuse_tick(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/defuse.ts:139 Defuse.makeSafe)
pub fn defuse_make_safe(g: &mut crate::game::Game, i: f64, by: crate::defuse::DefuseMakeSafeBy) { todo!() }

/// (src/game/defuse.ts:151 Defuse.blow)
pub fn defuse_blow(g: &mut crate::game::Game, d: &crate::scene_scenarios::DefuseData) { todo!() }

/// (src/game/defuse.ts:167 Defuse.draw)
pub fn defuse_draw(g: &mut crate::game::Game, d: &crate::scene_scenarios::DefuseData) { todo!() }

/// (src/game/defuse.ts:185 Defuse.samRows)
pub fn defuse_sam_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::defuse::DefuseSamRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/defuse.ts:203 Defuse.samTick)
pub fn defuse_sam_tick(g: &mut crate::game::Game, d: &crate::scene_scenarios::DefuseData) { todo!() }

/// (src/game/defuse.ts:216 Defuse.working)
pub fn defuse_working(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/defuse.ts:218 Defuse.samJob)
pub fn defuse_sam_job(g: &mut crate::game::Game) -> Option<crate::defuse::DefuseSamJob> { todo!() }

/// (src/game/defuse.ts:220 Defuse.cancel)
pub fn defuse_cancel(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/defuse.ts:222 Defuse.runningRow)
pub fn defuse_running_row(g: &mut crate::game::Game, running: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, f64) -> crate::game::Interactable) -> crate::game::Interactable { todo!() }

/// (src/game/defuse.ts:230 Defuse.objective)
pub fn defuse_objective(g: &mut crate::game::Game) -> Option<crate::defuse::DefuseObjective> { todo!() }

/// (src/game/defuse.ts:250 Defuse.onIt)
pub fn defuse_on_it(g: &mut crate::game::Game, i: f64) -> Option<String> { todo!() }

/// (src/game/defuse.ts:256 Defuse.snapshot)
pub fn defuse_snapshot(g: &mut crate::game::Game) -> Option<crate::defuse::DefuseSnap> { todo!() }

/// (src/game/defuse.ts:262 Defuse.restore)
pub fn defuse_restore(g: &mut crate::game::Game, s: Option<&crate::defuse::DefuseSnap>) { todo!() }

/// (src/game/defuse.ts:276 Defuse.note)
pub fn defuse_note(g: &mut crate::game::Game, what: &str) { todo!() }

/// (src/game/defuse.ts:277 Defuse.summary)
pub fn defuse_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/defuse.ts:285 defuseRows)
pub fn defuse_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::defuse::DefuseRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }
