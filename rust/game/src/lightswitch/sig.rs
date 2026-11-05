#![allow(unused_variables)]
//! Every function and method of `lightswitch`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/lightswitch.ts:93 switchesLive)
pub fn switches_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lightswitch.ts:97 roomLight)
pub fn room_light(g: &mut crate::game::Game, t: FixtureId) -> bool { todo!() }

/// (src/game/lightswitch.ts:105 roomOfLight)
pub fn room_of_light(g: &mut crate::game::Game, t: FixtureId) -> f64 { todo!() }

/// (src/game/lightswitch.ts:116 wallRay)
pub fn wall_ray(g: &mut crate::game::Game, ro: V3, rd: V3, tmax: f64) -> Option<crate::scene_collision::Raycast> { todo!() }

/// (src/game/lightswitch.ts:124 plateOn)
pub fn plate_on(g: &mut crate::game::Game, stand: V3, n: V3, room: f64, d: DoorId) -> Option<crate::lightswitch::Plate> { todo!() }

/// (src/game/lightswitch.ts:141 plateAt)
pub fn plate_at(g: &mut crate::game::Game, d: DoorId, side: f64, room: f64) -> Option<crate::lightswitch::Plate> { todo!() }

/// (src/game/lightswitch.ts:150 plateNear)
pub fn plate_near(g: &mut crate::game::Game, d: DoorId, room: f64) -> Option<crate::lightswitch::Plate> { todo!() }

/// (src/game/lightswitch.ts:162 fewest)
pub fn fewest(g: &mut crate::game::Game, ps: &[crate::lightswitch::Plate]) -> Vec<crate::lightswitch::Plate> { todo!() }

/// (src/game/lightswitch.ts:171 build)
pub fn build(g: &mut crate::game::Game) -> Vec<crate::lightswitch::WallSwitch> { todo!() }

/// (src/game/lightswitch.ts:194 wallSwitches)
pub fn wall_switches(g: &mut crate::game::Game) -> Vec<crate::lightswitch::WallSwitch> { todo!() }

/// (src/game/lightswitch.ts:201 geomSerial)
pub fn geom_serial(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/lightswitch.ts:203 wallStands)
pub fn wall_stands(g: &mut crate::game::Game, sw: &crate::lightswitch::WallSwitch) -> bool { todo!() }

/// (src/game/lightswitch.ts:208 lightSwitchReset)
pub fn light_switch_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/lightswitch.ts:212 switchLit)
pub fn switch_lit(g: &mut crate::game::Game, sw: &crate::lightswitch::WallSwitch) -> bool { todo!() }

/// (src/game/lightswitch.ts:215 plants)
pub fn plants(g: &mut crate::game::Game, by: CharId) -> bool { todo!() }

/// (src/game/lightswitch.ts:218 turnSecs)
pub fn turn_secs(g: &mut crate::game::Game, by: CharId, sw: &crate::lightswitch::WallSwitch) -> f64 { todo!() }

/// (src/game/lightswitch.ts:225 plantBroken)
pub fn plant_broken(g: &mut crate::game::Game, h: &crate::lightswitch::Hand) -> bool { todo!() }

/// (src/game/lightswitch.ts:230 switchLetGo)
pub fn switch_let_go(g: &mut crate::game::Game) { todo!() }

/// (src/game/lightswitch.ts:231 switchHand)
pub fn switch_hand(g: &mut crate::game::Game) -> Option<crate::lightswitch::SwitchHand> { todo!() }

/// (src/game/lightswitch.ts:237 flipSwitch)
pub fn flip_switch(g: &mut crate::game::Game, sw: &crate::lightswitch::WallSwitch, on: bool, by: CharId) { todo!() }

/// (src/game/lightswitch.ts:248 handsFree)
pub fn hands_free(g: &mut crate::game::Game, pl: PlayerId) -> bool { todo!() }

/// (src/game/lightswitch.ts:250 switchNear)
pub fn switch_near(g: &mut crate::game::Game, pl: PlayerId, r: f64) -> Option<crate::lightswitch::WallSwitch> { todo!() }

/// (src/game/lightswitch.ts:258 switchRows)
pub fn switch_rows(g: &mut crate::game::Game, pl: PlayerId, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::lightswitch::SwitchRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) { todo!() }

/// (src/game/lightswitch.ts:265 switchAct)
pub fn switch_act(g: &mut crate::game::Game, pl: PlayerId, verb: crate::game::Verb) -> bool { todo!() }

/// (src/game/lightswitch.ts:275 putsBack)
pub fn puts_back(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lightswitch.ts:277 lightSwitchTick)
pub fn light_switch_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/lightswitch.ts:307 switchBoxes)
pub fn switch_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!() }
