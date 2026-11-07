#![allow(unused_variables)]
//! Every function and method of `pressure`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/pressure.ts:225 hooked)
pub fn hooked(g: &mut crate::game::Game, d: RoomfightDefId, why: crate::pressure::HookedWhy, threat: V3) -> Option<crate::pressure::Hooked> { todo!() }

/// (src/game/pressure.ts:252 newStats)
pub fn new_stats() -> crate::pressure::StateStats { todo!() }

/// (src/game/pressure.ts:299 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::pressure::State { todo!() }

/// (src/game/pressure.ts:302 wayOut)
pub fn way_out(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pressure.ts:304 forms)
pub fn forms(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pressure.ts:310 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/pressure.ts:311 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/pressure.ts:312 alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/pressure.ts:313 note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: &str) { todo!() }

/// (src/game/pressure.ts:314 cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/pressure.ts:315 eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/pressure.ts:319 target)
pub fn target(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::pressure::Target> { todo!() }

/// (src/game/pressure.ts:339 picketHears)
pub fn picket_hears(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<RoomfightRoomId> { todo!() }

/// (src/game/pressure.ts:346 picketMarks)
pub fn picket_marks(g: &mut crate::game::Game, marks: &[crate::nightbreach::NightBreachMark], team_at: V3) -> Vec<crate::nightbreach::NightBreachMark> { todo!() }

/// (src/game/pressure.ts:385 postPicket)
pub fn post_picket(g: &mut crate::game::Game, gd: GuardId, room: RoomfightRoomId) { todo!() }

/// (src/game/pressure.ts:388 pressSay)
pub fn press_say(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:396 pressKeepsHome)
pub fn press_keeps_home(g: &mut crate::game::Game, d: RoomfightDefId, stop_clock: Option<bool>) -> bool { todo!() }

/// (src/game/pressure.ts:405 pressWants)
pub fn press_wants(g: &mut crate::game::Game, def: RoomfightDefId) -> bool { todo!() }

/// (src/game/pressure.ts:414 pressKeeps)
pub fn press_keeps(g: &mut crate::game::Game, def: RoomfightDefId) -> bool { todo!() }

/// (src/game/pressure.ts:420 staysHome)
pub fn stays_home(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/pressure.ts:425 gatherSpot)
pub fn gather_spot(g: &mut crate::game::Game, d: RoomfightDefId, t: RoomfightRoomId, e: crate::roomfight::Entry) -> Option<crate::pressure::GatherSpot> { todo!() }

/// (src/game/pressure.ts:448 standOffSpot)
pub fn stand_off_spot(g: &mut crate::game::Game, d: RoomfightDefId, t: RoomfightRoomId, e: crate::roomfight::Entry, dear: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64) -> Option<crate::pressure::StandOffSpot> { todo!() }

/// (src/game/pressure.ts:490 run)
pub fn run(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, door: Option<DoorId>, fast: Option<f64>) { todo!() }

/// (src/game/pressure.ts:499 charge)
pub fn charge(g: &mut crate::game::Game, d: RoomfightDefId, door: DoorId, t: Option<RoomfightRoomId>) -> bool { todo!() }

/// (src/game/pressure.ts:514 roomsBy)
pub fn rooms_by(g: &mut crate::game::Game, d: RoomfightDefId, door: DoorId) -> Vec<RoomfightRoomId> { todo!() }

/// (src/game/pressure.ts:520 inRoom)
pub fn in_room(g: &mut crate::game::Game, t: RoomfightRoomId, p: V3) -> bool { todo!() }

/// (src/game/pressure.ts:522 teamStands)
pub fn team_stands(g: &mut crate::game::Game, r: RoomfightRoomId) -> bool { todo!() }

/// (src/game/pressure.ts:524 lastAlone)
pub fn last_alone(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/pressure.ts:529 teamPlaced)
pub fn team_placed(g: &mut crate::game::Game, t: RoomfightRoomId) -> crate::pressure::TeamPlaced { todo!() }

/// (src/game/pressure.ts:538 heardAt)
pub fn heard_at(st: &mut crate::pressure::State, t: f64, p: V3) { todo!() }

/// (src/game/pressure.ts:546 learn)
pub fn learn(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:577 loose)
pub fn loose(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) { todo!() }

/// (src/game/pressure.ts:584 boots)
pub fn boots(g: &mut crate::game::Game, gd: GuardId, last: f64) -> f64 { todo!() }

/// (src/game/pressure.ts:590 teamBoots)
pub fn team_boots(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> { todo!() }

/// (src/game/pressure.ts:600 hearsTeam)
pub fn hears_team(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/pressure.ts:602 watchDoor)
pub fn watch_door(g: &mut crate::game::Game, d: RoomfightDefId, door: DoorId) { todo!() }

/// (src/game/pressure.ts:607 toldOf)
pub fn told_of(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::pressure::StateToldValue> { todo!() }

/// (src/game/pressure.ts:609 nextDoor)
pub fn next_door(g: &mut crate::game::Game, r: Option<RoomfightRoomId>, t: RoomfightRoomId) -> bool { todo!() }

/// (src/game/pressure.ts:612 consider)
pub fn consider(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/pressure.ts:708 drop)
pub fn drop(g: &mut crate::game::Game, m: PressureManId) { todo!() }

/// (src/game/pressure.ts:710 tickPress)
pub fn tick_press(g: &mut crate::game::Game, p: &mut crate::pressure::Press) -> bool { todo!() }

/// (src/game/pressure.ts:845 lastMan)
pub fn last_man(g: &mut crate::game::Game, d: RoomfightDefId) { todo!() }

/// (src/game/pressure.ts:899 tickOut)
pub fn tick_out(g: &mut crate::game::Game, o: &mut crate::pressure::Out) -> bool { todo!() }

/// (src/game/pressure.ts:919 charges)
pub fn charges(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/pressure.ts:922 fightOn)
pub fn fight_on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pressure.ts:931 awaiting)
pub fn awaiting(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pressure.ts:933 stoodTo)
pub fn stood_to(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/pressure.ts:935 readyMan)
pub fn ready_man(g: &mut crate::game::Game, d: RoomfightDefId, now: f64) -> bool { todo!() }

/// (src/game/pressure.ts:941 reportStands)
pub fn report_stands(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/pressure.ts:942 aim)
pub fn aim(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::pressure::Aim> { todo!() }

/// (src/game/pressure.ts:944 roomFor)
pub fn room_for(g: &mut crate::game::Game, at: V3) -> Option<RoomfightRoomId> { todo!() }

/// (src/game/pressure.ts:964 standTo)
pub fn stand_to(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:1002 standAndFight)
pub fn stand_and_fight(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:1021 goLook)
pub fn go_look(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:1063 pressTick)
pub fn press_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/pressure.ts:1064 stepPress)
pub fn step_press(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/pressure.ts:1082 pressStop)
pub fn press_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:1090 lookSince)
pub fn look_since(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/pressure.ts:1094 pressureStats)
pub fn pressure_stats(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/pressure.ts:1098 pressureSummary)
pub fn pressure_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/pressure.ts:1109 loadPressure)
pub fn load_pressure(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:1113 resetPressure)
pub fn reset_pressure(g: &mut crate::game::Game) { todo!() }

/// (src/game/pressure.ts:1117 savePressure)
pub fn save_pressure(g: &mut crate::game::Game) { todo!() }
