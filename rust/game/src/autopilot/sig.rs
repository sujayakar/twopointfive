#![allow(unused_variables)]
//! Every function and method of `autopilot`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/autopilot.ts:81 inSight)
pub fn in_sight(g: &mut crate::game::Game, o: GuardId) -> bool { todo!() }

/// (src/game/autopilot.ts:84 contactBehind)
pub fn contact_behind(g: &mut crate::game::Game, t: crate::autopilot::DoorTarget, far: f64) -> bool { todo!() }

/// (src/game/autopilot.ts:93 autoEntries)
pub fn auto_entries(g: &mut crate::game::Game, t: crate::autopilot::DoorTarget, shut: Option<bool>) -> crate::autopilot::AutoEntries { todo!() }

/// (src/game/autopilot.ts:106 autoPick)
pub fn auto_pick(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, check: Option<bool>, men: Option<&[GuardId]>) -> crate::autopilot::AutoPick { todo!() }

/// (src/game/autopilot.ts:128 autoQuick)
pub fn auto_quick(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> bool { todo!() }

/// (src/game/autopilot.ts:144 autoHint)
pub fn auto_hint(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, p: Option<&crate::autopilot::AutoPick>) -> Option<String> { todo!() }

/// (src/game/autopilot.ts:152 showSecsOf)
pub fn show_secs_of(g: &mut crate::game::Game, row: crate::autopilot::ShowSecsOfRow, fresh: f64) -> f64 { todo!() }
