#![allow(unused_variables)]
//! Every function and method of `holdfire`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/holdfire.ts:75 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::holdfire::State { todo!() }

/// (src/game/holdfire.ts:77 leadOf)
pub fn lead_of(g: &mut crate::game::Game) -> PlayerId { todo!() }

/// (src/game/holdfire.ts:78 mate)
pub fn mate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/holdfire.ts:79 living)
pub fn living(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/holdfire.ts:85 holdFireFound)
pub fn hold_fire_found(g: &mut crate::game::Game, men: &[GuardId]) -> Option<String> { todo!() }

/// (src/game/holdfire.ts:93 working)
pub fn working(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/holdfire.ts:103 holdFireLifted)
pub fn hold_fire_lifted(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/holdfire.ts:112 lift)
pub fn lift(g: &mut crate::game::Game, why: crate::holdfire::LiftWhy, what: &str, by: PlayerId, loud: Option<bool>) { todo!() }

/// (src/game/holdfire.ts:119 holdFireShot)
pub fn hold_fire_shot(g: &mut crate::game::Game, shooter: CharId) { todo!() }

/// (src/game/holdfire.ts:126 holdFireOrder)
pub fn hold_fire_order(g: &mut crate::game::Game, id_2: &str, o: Option<crate::holdfire::HoldFireOrderO>) { todo!() }

/// (src/game/holdfire.ts:133 holdFireSilent)
pub fn hold_fire_silent(g: &mut crate::game::Game, id_2: &str) -> bool { todo!() }

/// (src/game/holdfire.ts:136 holdFireLoud)
pub fn hold_fire_loud(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/holdfire.ts:141 holdFireDrill)
pub fn hold_fire_drill(g: &mut crate::game::Game, by: Option<PlayerId>) { todo!() }

/// (src/game/holdfire.ts:144 held)
pub fn held(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/holdfire.ts:149 holds)
pub fn holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut crate::game::Game) -> bool) -> bool { todo!() }

/// (src/game/holdfire.ts:155 holdFireHolds)
pub fn hold_fire_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut crate::game::Game) -> bool) -> bool { todo!() }

/// (src/game/holdfire.ts:163 holdFireOnMan)
pub fn hold_fire_on_man(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/holdfire.ts:168 holdFireGlass)
pub fn hold_fire_glass(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/holdfire.ts:170 holdFireGlassHeld)
pub fn hold_fire_glass_held(g: &mut crate::game::Game) { todo!() }

/// (src/game/holdfire.ts:173 holdFireReset)
pub fn hold_fire_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/holdfire.ts:174 holdFireSummary)
pub fn hold_fire_summary(g: &mut crate::game::Game) -> String { todo!() }
