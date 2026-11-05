#![allow(unused_variables)]
//! Every function and method of `gunhold`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/gunhold.ts:89 gunHoldStats)
pub fn gun_hold_stats<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::gunhold::GunHoldStats { todo!() }

/// (src/game/gunhold.ts:92 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/gunhold.ts:93 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/gunhold.ts:95 menOn)
pub fn men_on(g: &mut crate::game::Game, held: GuardId) -> Vec<GuardId> { todo!() }

/// (src/game/gunhold.ts:99 gunHoldSee)
pub fn gun_hold_see(g: &mut crate::game::Game, gd: GuardId, p: crate::guards::Pair) { todo!() }

/// (src/game/gunhold.ts:112 learnShot)
pub fn learn_shot(g: &mut crate::game::Game, k: &mut crate::gunhold::Known, shot_t: f64, sensed: bool) { todo!() }

/// (src/game/gunhold.ts:117 partOf)
pub fn part_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::gunhold::PartOf> { todo!() }

/// (src/game/gunhold.ts:124 gunHoldKeeps)
pub fn gun_hold_keeps(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/gunhold.ts:130 gunHoldRuns)
pub fn gun_hold_runs(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/gunhold.ts:136 gunHoldFree)
pub fn gun_hold_free(g: &mut crate::game::Game, p: crate::guards::Pair) -> bool { todo!() }

/// (src/game/gunhold.ts:145 expo)
pub fn expo(g: &mut crate::game::Game, k: &mut crate::gunhold::Known, i: f64) -> f64 { todo!() }

/// (src/game/gunhold.ts:155 cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/gunhold.ts:156 centre)
pub fn centre(g: &mut crate::game::Game, i: f64) -> V3 { todo!() }

/// (src/game/gunhold.ts:158 facesHim)
pub fn faces_him(g: &mut crate::game::Game, k: &crate::gunhold::Known, p: V3) -> bool { todo!() }

/// (src/game/gunhold.ts:160 front)
pub fn front(g: &mut crate::game::Game, k: &crate::gunhold::Known, p: V3) -> bool { todo!() }

/// (src/game/gunhold.ts:162 shown)
pub fn shown(g: &mut crate::game::Game, k: &crate::gunhold::Known, p: V3, low: bool) -> bool { todo!() }

/// (src/game/gunhold.ts:168 price)
pub fn price(g: &mut crate::game::Game, k: &mut crate::gunhold::Known) { todo!() }

/// (src/game/gunhold.ts:177 wayTo)
pub fn way_to(g: &mut crate::game::Game, gd: GuardId, k: &mut crate::gunhold::Known, goal: V3) -> Option<crate::gunhold::WayTo> { todo!() }

/// (src/game/gunhold.ts:183 take)
pub fn take(g: &mut crate::game::Game, gd: GuardId, path: &[V3], goal: V3) { todo!() }

/// (src/game/gunhold.ts:190 pickCover)
pub fn pick_cover(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, k: &mut crate::gunhold::Known, round: bool) -> Option<crate::gunhold::PickCover> { todo!() }

/// (src/game/gunhold.ts:229 planRound)
pub fn plan_round(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, k: &mut crate::gunhold::Known) -> bool { todo!() }

/// (src/game/gunhold.ts:255 goesRound)
pub fn goes_round(g: &mut crate::game::Game, gd: GuardId, s: GuardsStandoffId, k: &mut crate::gunhold::Known) -> bool { todo!() }

/// (src/game/gunhold.ts:287 gunHoldPool)
pub fn gun_hold_pool(g: &mut crate::game::Game, gd: GuardId, kind: crate::gunhold::GunHoldPoolKind) -> Vec<String> { todo!() }

/// (src/game/gunhold.ts:288 bark)
pub fn bark(g: &mut crate::game::Game, gd: GuardId, k: &mut crate::gunhold::Known, kind: crate::gunhold::BarkKind, radio: Option<bool>) -> bool { todo!() }

/// (src/game/gunhold.ts:297 gunHoldFeet)
pub fn gun_hold_feet(g: &mut crate::game::Game, gd: GuardId, dt: f64, p: crate::guards::Pair, s: GuardsStandoffId, fs: crate::gunhold::GunHoldFeetFs) -> Option<crate::gunhold::GunHoldFeet> { todo!() }

/// (src/game/gunhold.ts:426 gunHoldSummary)
pub fn gun_hold_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/gunhold.ts:438 gunHoldPhase)
pub fn gun_hold_phase(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::gunhold::Phase> { todo!() }
