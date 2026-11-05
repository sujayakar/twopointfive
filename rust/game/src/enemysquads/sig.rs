#![allow(unused_variables)]
//! Every function and method of `enemysquads`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/enemysquads.ts:234 senses)
pub fn senses(g: &mut crate::game::Game, v: Option<&str>) { todo!() }

/// (src/game/enemysquads.ts:286 netAsks)
pub fn net_asks(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/enemysquads.ts:305 squadsWhere)
pub fn squads_where(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/enemysquads.ts:306 squadDrill)
pub fn squad_drill(g: &mut crate::game::Game) -> Option<crate::knobs::SquadsK> { todo!() }

/// (src/game/enemysquads.ts:312 squadDrillWords)
pub fn squad_drill_words(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/enemysquads.ts:320 squadSkill)
pub fn squad_skill(g: &mut crate::game::Game, base: crate::roomfight::FightSkill) -> crate::roomfight::FightSkill { todo!() }

/// (src/game/enemysquads.ts:326 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/enemysquads.ts:327 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/enemysquads.ts:328 alive)
pub fn alive(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:329 grown)
pub fn grown(b: crate::scene_scenarios::Rect, m: f64) -> crate::enemysquads::Grown { todo!() }

/// (src/game/enemysquads.ts:330 roomOf)
pub fn room_of(g: &mut crate::game::Game, q: EnemysquadsSquadId, p: V3, m: Option<f64>) -> Option<crate::scene_scenarios::StructureRoom> { todo!() }

/// (src/game/enemysquads.ts:331 note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/enemysquads.ts:333 along)
pub fn along(from: V3, path: &[V3], each: &mut dyn FnMut(V3, f64)) -> f64 { todo!() }

/// (src/game/enemysquads.ts:338 pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { todo!() }

/// (src/game/enemysquads.ts:341 squadLegOf)
pub fn squad_leg_of(g: &mut crate::game::Game, q: EnemysquadsSquadId, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:342 squadWordOf)
pub fn squad_word_of(g: &mut crate::game::Game, q: EnemysquadsSquadId, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:343 squadSay)
pub fn squad_say(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:350 squadsLetGo)
pub fn squads_let_go(g: &mut crate::game::Game, st: Option<&crate::enemysquads::State>) { todo!() }

/// (src/game/enemysquads.ts:354 squadsStage)
pub fn squads_stage(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::StructureData>, hostiles: &[GuardId], rooms: &[String]) { todo!() }

/// (src/game/enemysquads.ts:368 squadsStop)
pub fn squads_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:377 squadsLive)
pub fn squads_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/enemysquads.ts:379 squadOf)
pub fn squad_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::enemysquads::SquadOf> { todo!() }

/// (src/game/enemysquads.ts:385 squadMayWedge)
pub fn squad_may_wedge(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/enemysquads.ts:390 squadRally)
pub fn squad_rally(g: &mut crate::game::Game, gd: GuardId, threat: V3) -> Option<V3> { todo!() }

/// (src/game/enemysquads.ts:408 defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/enemysquads.ts:410 fit)
pub fn fit(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:413 squadLookSince)
pub fn squad_look_since(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/enemysquads.ts:415 inSquadRooms)
pub fn in_squad_rooms(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool { todo!() }

/// (src/game/enemysquads.ts:417 home)
pub fn home(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/enemysquads.ts:429 tell)
pub fn tell(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) { todo!() }

/// (src/game/enemysquads.ts:435 teamKnown)
pub fn team_known(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/enemysquads.ts:450 wayRound)
pub fn way_round(g: &mut crate::game::Game, d: RoomfightDefId, at: V3, known: &[V3], taken: &[DoorId], used: &[V3]) -> Option<crate::enemysquads::Way> { todo!() }

/// (src/game/enemysquads.ts:497 teamKnownHow)
pub fn team_known_how(g: &mut crate::game::Game) -> Vec<crate::enemysquads::Known> { todo!() }

/// (src/game/enemysquads.ts:515 roundCtx)
pub fn round_ctx(g: &mut crate::game::Game, t: RoomfightRoomId, known: &[crate::enemysquads::Known], theirs: &[DoorId]) -> crate::enemysquads::RoundCtx { todo!() }

/// (src/game/enemysquads.ts:554 wayRound2)
pub fn way_round2(g: &mut crate::game::Game, d: RoomfightDefId, at: V3, known: &[crate::enemysquads::Known], theirs: &[DoorId], taken: &[DoorId], used: &[V3], back: Option<f64>, says: Option<&mut crate::enemysquads::WayRound2Says>) -> Option<crate::enemysquads::Way> { todo!() }

/// (src/game/enemysquads.ts:600 event)
pub fn event(g: &mut crate::game::Game, q: EnemysquadsSquadId, what: crate::enemysquads::SquadEventWhat, why: &str, by: Option<GuardId>) { todo!() }

/// (src/game/enemysquads.ts:607 goers)
pub fn goers(g: &mut crate::game::Game, h: &[RoomfightDefId], at: V3, k: f64) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/enemysquads.ts:617 mate)
pub fn mate(g: &mut crate::game::Game, m: GuardId) -> String { todo!() }

/// (src/game/enemysquads.ts:619 stayKey)
pub fn stay_key(g: &mut crate::game::Game, gd: GuardId, room: &crate::enemysquads::StayKeyRoom) -> String { todo!() }

/// (src/game/enemysquads.ts:620 netK)
pub fn net_k(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/enemysquads.ts:622 ownK)
pub fn own_k(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/enemysquads.ts:625 mayGo)
pub fn may_go(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> bool { todo!() }

/// (src/game/enemysquads.ts:629 decide)
pub fn decide(g: &mut crate::game::Game, q: EnemysquadsSquadId) { todo!() }

/// (src/game/enemysquads.ts:712 doorAt)
pub fn door_at(g: &mut crate::game::Game, room: Option<&crate::scene_scenarios::StructureRoom>, at: V3) -> Option<DoorId> { todo!() }

/// (src/game/enemysquads.ts:719 contact)
pub fn contact(g: &mut crate::game::Game, q: EnemysquadsSquadId, at: V3, room: Option<&crate::scene_scenarios::StructureRoom>, by_drop: Option<bool>, by: Option<GuardId>, words: Option<&crate::enemysquads::ContactWords>) { todo!() }

/// (src/game/enemysquads.ts:744 follow)
pub fn follow(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) { todo!() }

/// (src/game/enemysquads.ts:752 movedOn)
pub fn moved_on(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId, reach: f64) -> Option<crate::enemysquads::MovedOn> { todo!() }

/// (src/game/enemysquads.ts:764 back)
pub fn back(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) { todo!() }

/// (src/game/enemysquads.ts:789 lookOut)
pub fn look_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:832 lostRoomHas)
pub fn lost_room_has(g: &mut crate::game::Game, s: GuardId, at: V3, q: EnemysquadsSquadId) -> bool { todo!() }

/// (src/game/enemysquads.ts:845 checkDice)
pub fn check_dice(g: &mut crate::game::Game, q: EnemysquadsSquadId, salt: Option<f64>) -> crate::math_dice::Die { todo!() }

/// (src/game/enemysquads.ts:849 checkBeat)
pub fn check_beat(g: &mut crate::game::Game, q: EnemysquadsSquadId, die: &mut crate::math_dice::Die) -> f64 { todo!() }

/// (src/game/enemysquads.ts:850 rcOf)
pub fn rc_of<'g>(g: &'g mut crate::game::Game, q: EnemysquadsSquadId) -> &'g mut crate::enemysquads::Check { todo!() }

/// (src/game/enemysquads.ts:852 wentSilent)
pub fn went_silent(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: Option<V3>, broke: Option<bool>) { todo!() }

/// (src/game/enemysquads.ts:869 onNet)
pub fn on_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: Option<f64>, chron: Option<bool>) { todo!() }

/// (src/game/enemysquads.ts:875 heardFire)
pub fn heard_fire(g: &mut crate::game::Game, q: EnemysquadsSquadId, up: &[GuardId]) -> bool { todo!() }

/// (src/game/enemysquads.ts:880 bangT)
pub fn bang_t(g: &mut crate::game::Game, x: GuardId) -> f64 { todo!() }

/// (src/game/enemysquads.ts:888 walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) { todo!() }

/// (src/game/enemysquads.ts:892 radioCheck)
pub fn radio_check(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::RadioCheck> { todo!() }

/// (src/game/enemysquads.ts:1033 looker)
pub fn looker(g: &mut crate::game::Game, q: EnemysquadsSquadId, d: RoomfightDefId, who: GuardId, at: V3) -> crate::enemysquads::Looker { todo!() }

/// (src/game/enemysquads.ts:1045 turnBack)
pub fn turn_back(g: &mut crate::game::Game, q: EnemysquadsSquadId, who: GuardId) { todo!() }

/// (src/game/enemysquads.ts:1049 settle)
pub fn settle(g: &mut crate::game::Game, l: &crate::enemysquads::Looker, d: RoomfightDefId) { todo!() }

/// (src/game/enemysquads.ts:1058 looks)
pub fn looks(g: &mut crate::game::Game, q: EnemysquadsSquadId, l: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1129 noticed)
pub fn noticed(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1132 glimpse)
pub fn glimpse(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1141 callQuiet)
pub fn call_quiet(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1145 keyUp)
pub fn key_up(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: V3, room: Option<&crate::scene_scenarios::StructureRoom>, body: Option<GuardId>) -> bool { todo!() }

/// (src/game/enemysquads.ts:1157 callAbout)
pub fn call_about(g: &mut crate::game::Game, q: EnemysquadsSquadId, body: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1169 callsCut)
pub fn calls_cut(g: &mut crate::game::Game, q: EnemysquadsSquadId) { todo!() }

/// (src/game/enemysquads.ts:1198 cutAsk)
pub fn cut_ask(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> String { todo!() }

/// (src/game/enemysquads.ts:1222 owedOf)
pub fn owed_of(g: &mut crate::game::Game, who: GuardId, q: EnemysquadsSquadId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1226 callsHung)
pub fn calls_hung(g: &mut crate::game::Game, q: EnemysquadsSquadId) { todo!() }

/// (src/game/enemysquads.ts:1237 callsTick)
pub fn calls_tick(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::Call> { todo!() }

/// (src/game/enemysquads.ts:1249 squadCalls)
pub fn squad_calls(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadCallsItem> { todo!() }

/// (src/game/enemysquads.ts:1256 netWary)
pub fn net_wary(g: &mut crate::game::Game, about: &str) { todo!() }

/// (src/game/enemysquads.ts:1262 seenTo)
pub fn seen_to(g: &mut crate::game::Game, x: EnemysquadsSquadId, body: GuardId) { todo!() }

/// (src/game/enemysquads.ts:1265 netSay)
pub fn net_say(g: &mut crate::game::Game, after: f64, gd: GuardId, text: &str, cls: Option<f64>) { todo!() }

/// (src/game/enemysquads.ts:1267 waitsOn)
pub fn waits_on(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> Vec<GuardId> { todo!() }

/// (src/game/enemysquads.ts:1272 netSend)
pub fn net_send(g: &mut crate::game::Game, x: EnemysquadsSquadId, who: GuardId, stops: &[V3], all: bool) -> Option<crate::enemysquads::NetSend> { todo!() }

/// (src/game/enemysquads.ts:1293 netCheck)
pub fn net_check(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:1389 squadsTick)
pub fn squads_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:1527 squadsReport)
pub fn squads_report(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadReport> { todo!() }

/// (src/game/enemysquads.ts:1531 squadsSummary)
pub fn squads_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/enemysquads.ts:1536 squadMen)
pub fn squad_men(g: &mut crate::game::Game, name: &str) -> Vec<GuardId> { todo!() }

/// (src/game/enemysquads.ts:1543 squadWaysRound)
pub fn squad_ways_round(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadWaysRoundItem> { todo!() }
