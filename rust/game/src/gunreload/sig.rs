#![allow(unused_variables)]
//! Every function and method of `gunreload`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/gunreload.ts:48 g2w)
pub fn g2w(p: V3, d: Option<V3>) -> V3 { todo!() }

/// (src/game/gunreload.ts:71 feedOf)
pub fn feed_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<crate::gunreload::FEEDValue> { todo!() }

/// (src/game/gunreload.ts:73 feedBy)
pub fn feed_by(m: crate::weapons::GunModelId) -> crate::gunreload::FEEDValue { todo!() }

/// (src/game/gunreload.ts:77 beltTimes)
pub fn belt_times(b: crate::gunreload::FeedBeat) -> crate::gunreload::BeltTimes { todo!() }

/// (src/game/gunreload.ts:83 shellTimes)
pub fn shell_times(b: crate::gunreload::FeedBeat, n: f64) -> Vec<f64> { todo!() }

/// (src/game/gunreload.ts:85 shellRate)
pub fn shell_rate(k: f64) -> f64 { todo!() }

/// (src/game/gunreload.ts:89 feedCues)
pub fn feed_cues(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef, b: crate::gunreload::FeedBeat, n: f64, port: bool) -> Option<Vec<crate::gunreload::FeedCue>> { todo!() }

/// (src/game/gunreload.ts:109 feedPath)
pub fn feed_path(g: &mut crate::game::Game, model: crate::weapons::GunModelId, b: crate::gunreload::FeedBeat, n: f64, port: bool) -> Option<crate::gunreload::FeedPath> { todo!() }

/// (src/game/gunreload.ts:138 feedDeal)
pub fn feed_deal(g: &mut crate::game::Game, ch: CharId, gun: Option<FirearmId>, dry: Option<bool>) { todo!() }

/// (src/game/gunreload.ts:145 feedLive)
pub fn feed_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunreload.ts:150 feedPlan)
pub fn feed_plan(g: &mut crate::game::Game, gun: FirearmId, row: crate::weapons::WeaponDefReload, shells: Option<crate::gunreload::FeedPlanShells>) -> Option<crate::gunreload::FeedPlan> { todo!() }

/// (src/game/gunreload.ts:158 beatOf)
pub fn beat_of(r: crate::weapons::WeaponDefReload, pull: bool) -> crate::gunreload::FeedBeat { todo!() }

/// (src/game/gunreload.ts:169 shellPlan)
pub fn shell_plan(g: &mut crate::game::Game, gun: FirearmId, free: Option<bool>, max: Option<f64>) -> Option<crate::gunreload::ShellPlan> { todo!() }

/// (src/game/gunreload.ts:182 feedBegin)
pub fn feed_begin(g: &mut crate::game::Game, gun: FirearmId) -> crate::weapons::WeaponDefReload { todo!() }

/// (src/game/gunreload.ts:188 feedPlay)
pub fn feed_play(g: &mut crate::game::Game, at: V3, c_2: &crate::gunreload::FeedCue, vol: Option<f64>) { todo!() }

/// (src/game/gunreload.ts:193 feedTick)
pub fn feed_tick(g: &mut crate::game::Game, c_2: CharId, def: &crate::weapons::WeaponDef, before: f64, now: f64, vol: Option<f64>, gun: Option<FirearmId>) { todo!() }

/// (src/game/gunreload.ts:201 feedDrop)
pub fn feed_drop(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<f64> { todo!() }

/// (src/game/gunreload.ts:203 feedStarts)
pub fn feed_starts(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> bool { todo!() }

/// (src/game/gunreload.ts:207 ownOf)
pub fn own_of(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId) -> Option<crate::gunreload::FEEDValue> { todo!() }

/// (src/game/gunreload.ts:210 reloadShow)
pub fn reload_show(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId) -> Option<crate::gunreload::ReloadShow> { todo!() }

/// (src/game/gunreload.ts:218 sstep)
pub fn sstep(a: f64, b: f64, x: f64) -> f64 { todo!() }

/// (src/game/gunreload.ts:222 reloadHandBoxes)
pub fn reload_hand_boxes(g: &mut crate::game::Game, c_2: CharId, id_2: crate::weapons::WeaponId, out: &mut Vec<BoxId>) { todo!() }
