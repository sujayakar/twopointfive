#![allow(unused_variables)]
//! Every function and method of `mission`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/mission.ts:22 missionLive)
pub fn mission_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/mission.ts:25 tally)
pub fn tally(g: &mut crate::game::Game, k: crate::mission::TallyK) { todo!() }

/// (src/game/mission.ts:30 debriefRows)
pub fn debrief_rows(s: &crate::game::MissionStats) -> Vec<[String; 2]> { todo!() }

/// (src/game/mission.ts:46 rackDist)
pub fn rack_dist(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/mission.ts:54 outNow)
pub fn out_now(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/mission.ts:64 updateMission)
pub fn update_mission(g: &mut crate::game::Game, dt: f64, input: InputId) { todo!() }

/// (src/game/mission.ts:104 cancelDrive)
pub fn cancel_drive(g: &mut crate::game::Game) { todo!() }

/// (src/game/mission.ts:109 drivePulled)
pub fn drive_pulled(g: &mut crate::game::Game) { todo!() }

/// (src/game/mission.ts:125 sendRackCheck)
pub fn send_rack_check(g: &mut crate::game::Game) { todo!() }

/// (src/game/mission.ts:139 objectivePos)
pub fn objective_pos(g: &mut crate::game::Game) -> Option<V3> { todo!() }

/// (src/game/mission.ts:154 objectiveText)
pub fn objective_text(g: &mut crate::game::Game) -> String { todo!() }
