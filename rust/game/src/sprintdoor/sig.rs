#![allow(unused_variables)]
//! Every function and method of `sprintdoor`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/sprintdoor.ts:52 live)
pub fn live(g: &mut crate::game::Game, pl: crate::sprintdoor::LivePl) -> bool { todo!() }

/// (src/game/sprintdoor.ts:55 sprintDoorRun)
pub fn sprint_door_run(g: &mut crate::game::Game, pl: PlayerId) -> Option<crate::doors::DoorContactRun> { todo!() }

/// (src/game/sprintdoor.ts:70 sprintDoorKick)
pub fn sprint_door_kick(g: &mut crate::game::Game, pl: PlayerId) -> Option<DoorId> { todo!() }

/// (src/game/sprintdoor.ts:94 sprintDoorAfter)
pub fn sprint_door_after(g: &mut crate::game::Game, pl: PlayerId, own: &crate::doors::DoorContact) { todo!() }

/// (src/game/sprintdoor.ts:116 sprintDoorPace)
pub fn sprint_door_pace(g: &mut crate::game::Game, pl: crate::sprintdoor::DoorStamps, now: Option<f64>) -> f64 { todo!() }

/// (src/game/sprintdoor.ts:119 sprintDoorTurning)
pub fn sprint_door_turning(g: &mut crate::game::Game, pl: crate::sprintdoor::DoorStamps, now: Option<f64>) -> bool { todo!() }

/// (src/game/sprintdoor.ts:124 sprintDoorLean)
pub fn sprint_door_lean(g: &mut crate::game::Game, pl: crate::sprintdoor::DoorStamps, now: Option<f64>) -> Option<f64> { todo!() }

/// (src/game/sprintdoor.ts:131 sprintDoorHead)
pub fn sprint_door_head(off: f64) -> f64 { todo!() }
