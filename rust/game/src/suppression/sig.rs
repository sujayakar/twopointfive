#![allow(unused_variables)]
//! Every function and method of `suppression`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/suppression.ts:95 Sup.clearReactions)
pub fn sup_clear_reactions(this: &mut crate::suppression::Sup) { todo!() }

/// (src/game/suppression.ts:107 keenOf)
pub fn keen_of(g: &mut crate::game::Game, c_2: CharId) -> f64 { todo!() }

/// (src/game/suppression.ts:116 noteRound)
pub fn note_round(g: &mut crate::game::Game, shooter: CharId, from: V3, to: V3, hit: Option<CharId>) { todo!() }

/// (src/game/suppression.ts:146 forgetSuppressionClocks)
pub fn forget_suppression_clocks(g: &mut crate::game::Game) { todo!() }

/// (src/game/suppression.ts:162 watchYourFire)
pub fn watch_your_fire(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/suppression.ts:163 supBark)
pub fn sup_bark(g: &mut crate::game::Game, gd: GuardId, moment: crate::suppression::SupMoment, force: Option<bool>) -> bool { todo!() }

/// (src/game/suppression.ts:177 suppressionLines)
pub fn suppression_lines(moment: Option<crate::suppression::SupMoment>) -> Vec<String> { todo!() }

/// (src/game/suppression.ts:180 reacts)
pub fn reacts(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/suppression.ts:185 updateSuppression)
pub fn update_suppression(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/suppression.ts:227 headDown)
pub fn head_down(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::suppression::HeadDown { todo!() }

/// (src/game/suppression.ts:247 endDisplace)
pub fn end_displace(g: &mut crate::game::Game, gd: GuardId, why: &str) { todo!() }

/// (src/game/suppression.ts:253 holdingCover)
pub fn holding_cover(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/suppression.ts:258 coverFrom)
pub fn cover_from(g: &mut crate::game::Game, gd: GuardId, threat: V3) -> Option<V3> { todo!() }

/// (src/game/suppression.ts:298 guardFireMods)
pub fn guard_fire_mods(g: &mut crate::game::Game, gd: GuardId) -> crate::suppression::GuardFireMods { todo!() }

/// (src/game/suppression.ts:303 flinching)
pub fn flinching(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/suppression.ts:305 samSpread)
pub fn sam_spread(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/suppression.ts:308 suppressionSummary)
pub fn suppression_summary(g: &mut crate::game::Game) -> String { todo!() }
