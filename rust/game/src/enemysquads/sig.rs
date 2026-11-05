#![allow(unused_variables)]
//! Every function and method of `enemysquads`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/enemysquads.ts:234 senses)
pub fn senses(g: &mut crate::game::Game, v: Option<&str>) { todo!() }

/// (src/game/enemysquads.ts:283 netAsks)
pub fn net_asks(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/enemysquads.ts:302 squadsWhere)
pub fn squads_where(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/enemysquads.ts:303 squadDrill)
pub fn squad_drill(g: &mut crate::game::Game) -> Option<crate::knobs::SquadsK> { todo!() }

/// (src/game/enemysquads.ts:309 squadDrillWords)
pub fn squad_drill_words(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/enemysquads.ts:317 squadSkill)
pub fn squad_skill(g: &mut crate::game::Game, base: crate::roomfight::FightSkill) -> crate::roomfight::FightSkill { todo!() }

/// (src/game/enemysquads.ts:323 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/enemysquads.ts:324 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/enemysquads.ts:325 alive)
pub fn alive(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:326 grown)
pub fn grown(b: crate::scene_scenarios::Rect, m: f64) -> crate::enemysquads::Grown { todo!() }

/// (src/game/enemysquads.ts:327 roomOf)
pub fn room_of(g: &mut crate::game::Game, q: EnemysquadsSquadId, p: V3, m: Option<f64>) -> Option<crate::scene_scenarios::StructureRoom> { todo!() }

/// (src/game/enemysquads.ts:328 note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/enemysquads.ts:330 along)
pub fn along(from: V3, path: &[V3], each: &mut dyn FnMut(V3, f64)) -> f64 { todo!() }

/// (src/game/enemysquads.ts:335 pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { todo!() }

/// (src/game/enemysquads.ts:340 squadsStage)
pub fn squads_stage(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::StructureData>, hostiles: &[GuardId], rooms: &[String]) { todo!() }

/// (src/game/enemysquads.ts:354 squadsStop)
pub fn squads_stop(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:363 squadsLive)
pub fn squads_live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/enemysquads.ts:365 squadOf)
pub fn squad_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::enemysquads::SquadOf> { todo!() }

/// (src/game/enemysquads.ts:369 squadOwns)
pub fn squad_owns(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:371 squadDefers)
pub fn squad_defers(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:376 squadMayWedge)
pub fn squad_may_wedge(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/enemysquads.ts:378 squadStaysIn)
pub fn squad_stays_in(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:382 squadRally)
pub fn squad_rally(g: &mut crate::game::Game, gd: GuardId, threat: V3) -> Option<V3> { todo!() }

/// (src/game/enemysquads.ts:400 defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/enemysquads.ts:402 fit)
pub fn fit(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:405 squadLookSince)
pub fn squad_look_since(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> { todo!() }

/// (src/game/enemysquads.ts:406 squadBusy)
pub fn squad_busy(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:408 squadHolds)
pub fn squad_holds(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool { todo!() }

/// (src/game/enemysquads.ts:410 home)
pub fn home(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/enemysquads.ts:422 tell)
pub fn tell(g: &mut crate::game::Game, d: RoomfightDefId, at: V3) { todo!() }

/// (src/game/enemysquads.ts:428 teamKnown)
pub fn team_known(g: &mut crate::game::Game) -> Vec<V3> { todo!() }

/// (src/game/enemysquads.ts:443 wayRound)
pub fn way_round(g: &mut crate::game::Game, d: RoomfightDefId, at: V3, known: &[V3], taken: &[DoorId], used: &[V3]) -> Option<crate::enemysquads::Way> { todo!() }

/// (src/game/enemysquads.ts:490 teamKnownHow)
pub fn team_known_how(g: &mut crate::game::Game) -> Vec<crate::enemysquads::Known> { todo!() }

/// (src/game/enemysquads.ts:508 roundCtx)
pub fn round_ctx(g: &mut crate::game::Game, t: RoomfightRoomId, known: &[crate::enemysquads::Known], theirs: &[DoorId]) -> crate::enemysquads::RoundCtx { todo!() }

/// (src/game/enemysquads.ts:547 wayRound2)
pub fn way_round2(g: &mut crate::game::Game, d: RoomfightDefId, at: V3, known: &[crate::enemysquads::Known], theirs: &[DoorId], taken: &[DoorId], used: &[V3], back: Option<f64>, says: Option<&mut crate::enemysquads::WayRound2Says>) -> Option<crate::enemysquads::Way> { todo!() }

/// (src/game/enemysquads.ts:593 event)
pub fn event(g: &mut crate::game::Game, q: EnemysquadsSquadId, what: crate::enemysquads::EventWhat, why: &str, by: Option<GuardId>) { todo!() }

/// (src/game/enemysquads.ts:600 goers)
pub fn goers(g: &mut crate::game::Game, h: &[RoomfightDefId], at: V3, k: f64) -> Vec<RoomfightDefId> { todo!() }

/// (src/game/enemysquads.ts:610 mate)
pub fn mate(g: &mut crate::game::Game, m: GuardId) -> String { todo!() }

/// (src/game/enemysquads.ts:612 stayKey)
pub fn stay_key(g: &mut crate::game::Game, gd: GuardId, room: &crate::enemysquads::StayKeyRoom) -> String { todo!() }

/// (src/game/enemysquads.ts:613 netK)
pub fn net_k(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/enemysquads.ts:615 ownK)
pub fn own_k(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/enemysquads.ts:618 mayGo)
pub fn may_go(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> bool { todo!() }

/// (src/game/enemysquads.ts:622 decide)
pub fn decide(g: &mut crate::game::Game, q: EnemysquadsSquadId) { todo!() }

/// (src/game/enemysquads.ts:705 doorAt)
pub fn door_at(g: &mut crate::game::Game, room: Option<&crate::scene_scenarios::StructureRoom>, at: V3) -> Option<DoorId> { todo!() }

/// (src/game/enemysquads.ts:712 contact)
pub fn contact(g: &mut crate::game::Game, q: EnemysquadsSquadId, at: V3, room: Option<&crate::scene_scenarios::StructureRoom>, by_drop: Option<bool>, by: Option<GuardId>, words: Option<&crate::enemysquads::ContactWords>) { todo!() }

/// (src/game/enemysquads.ts:737 follow)
pub fn follow(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) { todo!() }

/// (src/game/enemysquads.ts:745 movedOn)
pub fn moved_on(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId, reach: f64) -> Option<crate::enemysquads::MovedOn> { todo!() }

/// (src/game/enemysquads.ts:757 back)
pub fn back(g: &mut crate::game::Game, q: EnemysquadsSquadId, s: GuardId, d: RoomfightDefId) { todo!() }

/// (src/game/enemysquads.ts:782 lookOut)
pub fn look_out(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:825 lostRoomHas)
pub fn lost_room_has(g: &mut crate::game::Game, s: &crate::enemysquads::Sent, at: V3) -> bool { todo!() }

/// (src/game/enemysquads.ts:838 checkDice)
pub fn check_dice(g: &mut crate::game::Game, q: EnemysquadsSquadId, salt: Option<f64>) -> crate::math_dice::Die { todo!() }

/// (src/game/enemysquads.ts:842 checkBeat)
pub fn check_beat(g: &mut crate::game::Game, q: EnemysquadsSquadId, die: &mut crate::math_dice::Die) -> f64 { todo!() }

/// (src/game/enemysquads.ts:843 rcOf)
pub fn rc_of<'g>(g: &'g mut crate::game::Game, q: EnemysquadsSquadId) -> &'g mut crate::enemysquads::Check { todo!() }

/// (src/game/enemysquads.ts:845 wentSilent)
pub fn went_silent(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: Option<V3>, broke: Option<bool>) { todo!() }

/// (src/game/enemysquads.ts:862 onNet)
pub fn on_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: Option<f64>, chron: Option<bool>) { todo!() }

/// (src/game/enemysquads.ts:868 heardFire)
pub fn heard_fire(g: &mut crate::game::Game, q: EnemysquadsSquadId, up: &[GuardId]) -> bool { todo!() }

/// (src/game/enemysquads.ts:873 bangT)
pub fn bang_t(g: &mut crate::game::Game, x: GuardId) -> f64 { todo!() }

/// (src/game/enemysquads.ts:881 walk)
pub fn walk(g: &mut crate::game::Game, d: RoomfightDefId, to: V3) { todo!() }

/// (src/game/enemysquads.ts:885 radioCheck)
pub fn radio_check(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::RadioCheck> { todo!() }

/// (src/game/enemysquads.ts:1026 looker)
pub fn looker(g: &mut crate::game::Game, q: EnemysquadsSquadId, d: RoomfightDefId, who: GuardId, at: V3) -> crate::enemysquads::Looker { todo!() }

/// (src/game/enemysquads.ts:1038 turnBack)
pub fn turn_back(g: &mut crate::game::Game, q: EnemysquadsSquadId, who: GuardId) { todo!() }

/// (src/game/enemysquads.ts:1042 settle)
pub fn settle(g: &mut crate::game::Game, l: &crate::enemysquads::Looker, d: RoomfightDefId) { todo!() }

/// (src/game/enemysquads.ts:1051 looks)
pub fn looks(g: &mut crate::game::Game, q: EnemysquadsSquadId, l: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1122 noticed)
pub fn noticed(g: &mut crate::game::Game, m: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1125 glimpse)
pub fn glimpse(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1134 callQuiet)
pub fn call_quiet(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1138 keyUp)
pub fn key_up(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId, at: V3, room: Option<&crate::scene_scenarios::StructureRoom>, body: Option<GuardId>) -> bool { todo!() }

/// (src/game/enemysquads.ts:1150 callAbout)
pub fn call_about(g: &mut crate::game::Game, q: EnemysquadsSquadId, body: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1162 callsCut)
pub fn calls_cut(g: &mut crate::game::Game, q: EnemysquadsSquadId) { todo!() }

/// (src/game/enemysquads.ts:1191 cutAsk)
pub fn cut_ask(g: &mut crate::game::Game, q: EnemysquadsSquadId, m: GuardId) -> String { todo!() }

/// (src/game/enemysquads.ts:1215 owedOf)
pub fn owed_of(g: &mut crate::game::Game, c_2: &crate::enemysquads::Check, who: GuardId) -> bool { todo!() }

/// (src/game/enemysquads.ts:1219 callsHung)
pub fn calls_hung(g: &mut crate::game::Game, q: EnemysquadsSquadId) { todo!() }

/// (src/game/enemysquads.ts:1230 callsTick)
pub fn calls_tick(g: &mut crate::game::Game, q: EnemysquadsSquadId, busy: bool) -> Option<crate::enemysquads::Call> { todo!() }

/// (src/game/enemysquads.ts:1242 squadCalls)
pub fn squad_calls(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadCallsItem> { todo!() }

/// (src/game/enemysquads.ts:1249 netWary)
pub fn net_wary(g: &mut crate::game::Game, about: &str) { todo!() }

/// (src/game/enemysquads.ts:1255 seenTo)
pub fn seen_to(g: &mut crate::game::Game, x: EnemysquadsSquadId, body: GuardId) { todo!() }

/// (src/game/enemysquads.ts:1258 netSay)
pub fn net_say(g: &mut crate::game::Game, after: f64, gd: GuardId, text: &str, cls: Option<f64>) { todo!() }

/// (src/game/enemysquads.ts:1260 waitsOn)
pub fn waits_on(g: &mut crate::game::Game, q: EnemysquadsSquadId) -> Vec<GuardId> { todo!() }

/// (src/game/enemysquads.ts:1265 netSend)
pub fn net_send(g: &mut crate::game::Game, x: EnemysquadsSquadId, who: GuardId, stops: &[V3], all: bool) -> Option<crate::enemysquads::NetSend> { todo!() }

/// (src/game/enemysquads.ts:1286 netCheck)
pub fn net_check(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:1382 squadsTick)
pub fn squads_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/enemysquads.ts:1519 squadsReport)
pub fn squads_report(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadReport> { todo!() }

/// (src/game/enemysquads.ts:1523 squadsSummary)
pub fn squads_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/enemysquads.ts:1528 squadMen)
pub fn squad_men(g: &mut crate::game::Game, name: &str) -> Vec<GuardId> { todo!() }

/// (src/game/enemysquads.ts:1535 squadWaysRound)
pub fn squad_ways_round(g: &mut crate::game::Game) -> Vec<crate::enemysquads::SquadWaysRoundItem> { todo!() }
