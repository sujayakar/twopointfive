#![allow(unused_variables)]
//! Every function and method of `silent`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/silent.ts:115 mate)
pub fn mate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:116 castOf)
pub fn cast_of(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/silent.ts:120 squadSilent)
pub fn squad_silent(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/silent.ts:122 setSilent)
pub fn set_silent(g: &mut crate::game::Game, on: bool, by: Option<crate::silent::By>) { todo!() }

/// (src/game/silent.ts:129 underSilence)
pub fn under_silence(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:133 silentPace)
pub fn silent_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 { todo!() }

/// (src/game/silent.ts:139 reached)
pub fn reached(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:141 urgent)
pub fn urgent(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/silent.ts:145 silentStep)
pub fn silent_step(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/silent.ts:152 silentDark)
pub fn silent_dark(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:154 silentHush)
pub fn silent_hush(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:156 silentEase)
pub fn silent_ease(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:158 silentNoBoot)
pub fn silent_no_boot(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/silent.ts:164 silentPick)
pub fn silent_pick(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/silent.ts:170 silentBoot)
pub fn silent_boot(g: &mut crate::game::Game, d: DoorId) -> Option<crate::silent::SilentBoot> { todo!() }

/// (src/game/silent.ts:176 silentNoCans)
pub fn silent_no_cans(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/silent.ts:179 quietGun)
pub fn quiet_gun(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:181 ownHold)
pub fn own_hold(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:187 roundHeld)
pub fn round_held(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:188 silentHoldsRound)
pub fn silent_holds_round(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:197 silentHoldsTarget)
pub fn silent_holds_target(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/silent.ts:205 silentHoldsGlass)
pub fn silent_holds_glass(g: &mut crate::game::Game, gd: GuardId, h: GuardId, c_2: CharId, to: V3) -> bool { todo!() }

/// (src/game/silent.ts:214 silentTool)
pub fn silent_tool(g: &mut crate::game::Game, loud: bool) { todo!() }

/// (src/game/silent.ts:219 silentCompromised)
pub fn silent_compromised(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/silent.ts:224 rows)
pub fn rows(g: &mut crate::game::Game) -> [OrdersOrderDefId; 2] { todo!() }

/// (src/game/silent.ts:236 loudWord)
pub fn loud_word(w: Option<&str>) -> String { todo!() }

/// (src/game/silent.ts:237 seat)
pub fn seat(g: &mut crate::game::Game) { todo!() }

/// (src/game/silent.ts:255 silentTick)
pub fn silent_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/silent.ts:268 silentSummary)
pub fn silent_summary(g: &mut crate::game::Game) -> String { todo!() }


/// INVENTED: `posture.word()` (silent.ts:247)
pub fn posture_word(g: &mut crate::game::Game) -> Option<String> { todo!() }
/// INVENTED: `posture.mark(def)` (silent.ts:248)
pub fn posture_mark(g: &mut crate::game::Game, def: OrdersOrderDefId) -> &'static str { todo!() }
/// INVENTED: `posture.given(def)` (silent.ts:249)
pub fn posture_given(g: &mut crate::game::Game, def: OrdersOrderDefId) { todo!() }

