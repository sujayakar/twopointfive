#![allow(unused_variables)]
//! Every function and method of `digin`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/digin.ts:198 saveDigIn)
pub fn save_dig_in(g: &mut crate::game::Game) { todo!() }

/// (src/game/digin.ts:199 loadDigIn)
pub fn load_dig_in(g: &mut crate::game::Game) { todo!() }

/// (src/game/digin.ts:203 resetDigIn)
pub fn reset_dig_in(g: &mut crate::game::Game) { todo!() }

/// (src/game/digin.ts:221 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::digin::St { todo!() }

/// (src/game/digin.ts:223 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/digin.ts:224 pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/digin.ts:225 paced)
pub fn paced(g: &mut crate::game::Game, r: [f64; 2]) -> f64 { todo!() }

/// (src/game/digin.ts:226 oneOf)
pub fn one_of<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, a: &[T]) -> T { todo!() }

/// (src/game/digin.ts:227 at)
pub fn at(p: V3, y: f64) -> V3 { todo!() }

/// (src/game/digin.ts:228 log)
pub fn log(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/digin.ts:231 listeners)
pub fn listeners(g: &mut crate::game::Game) -> Vec<crate::digin::ListenersItem> { todo!() }

/// (src/game/digin.ts:238 heardBy)
pub fn heard_by(g: &mut crate::game::Game, src: V3, reach_m: f64) -> Option<crate::digin::HeardBy> { todo!() }

/// (src/game/digin.ts:244 heardNoise)
pub fn heard_noise(g: &mut crate::game::Game, src: V3, level: f64) -> Option<crate::digin::HeardNoise> { todo!() }

/// (src/game/digin.ts:249 cue)
pub fn cue(g: &mut crate::game::Game, r: Option<&mut crate::digin::RoomSt>, kind: crate::digin::Cue) { todo!() }

/// (src/game/digin.ts:251 shout)
pub fn shout(g: &mut crate::game::Game, gd: GuardId, text: &str, r: Option<&mut crate::digin::RoomSt>) -> bool { todo!() }

/// (src/game/digin.ts:257 pathLen)
pub fn path_len(from: V3, path: &[V3]) -> f64 { todo!() }

/// (src/game/digin.ts:259 wayTo)
pub fn way_to(g: &mut crate::game::Game, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/digin.ts:261 expectEntries)
pub fn expect_entries(g: &mut crate::game::Game, room: RoomfightRoomId, from: V3) -> [f64; 2] { todo!() }

/// (src/game/digin.ts:277 coverSpot)
pub fn cover_spot(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt, deep: bool) -> crate::digin::CoverSpot { todo!() }

/// (src/game/digin.ts:364 knowsBangs)
pub fn knows_bangs(g: &mut crate::game::Game, d: RoomfightDefId) -> bool { todo!() }

/// (src/game/digin.ts:368 wantsDeep)
pub fn wants_deep(g: &mut crate::game::Game, d: RoomfightDefId, r: &crate::digin::RoomSt) -> bool { todo!() }

/// (src/game/digin.ts:374 move)
pub fn move_(g: &mut crate::game::Game, d: RoomfightDefId, goal: V3, speed: f64) { todo!() }

/// (src/game/digin.ts:379 fear)
pub fn fear(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man) { todo!() }

/// (src/game/digin.ts:381 nearestProp)
pub fn nearest_prop(g: &mut crate::game::Game, d: RoomfightDefId, e: crate::roomfight::Entry, skip: Option<&JsSet<PropId>>) -> Option<PropId> { todo!() }

/// (src/game/digin.ts:391 scrape)
pub fn scrape(g: &mut crate::game::Game, gd: GuardId, m: crate::digin::Man, r: Option<&mut crate::digin::RoomSt>, thud: Option<bool>) { todo!() }

/// (src/game/digin.ts:400 wedge)
pub fn wedge(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &mut crate::digin::RoomSt, e: &mut crate::roomfight::Entry) { todo!() }

/// (src/game/digin.ts:408 wedgeable)
pub fn wedgeable(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/digin.ts:410 heardAt)
pub fn heard_at(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man) -> V3 { todo!() }

/// (src/game/digin.ts:414 shutsOpen)
pub fn shuts_open(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/digin.ts:419 doorOf)
pub fn door_of(g: &mut crate::game::Game, m: crate::digin::Man) -> f64 { todo!() }

/// (src/game/digin.ts:421 latchSecs)
pub fn latch_secs(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/digin.ts:423 handOn)
pub fn hand_on(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, d_2: DoorId) { todo!() }

/// (src/game/digin.ts:429 handWaits)
pub fn hand_waits(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, d_2: DoorId) -> bool { todo!() }

/// (src/game/digin.ts:443 atADoor)
pub fn at_a_door(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt) -> bool { todo!() }

/// (src/game/digin.ts:446 shutMore)
pub fn shut_more(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &crate::digin::RoomSt, not: f64) -> bool { todo!() }

/// (src/game/digin.ts:465 wayInRoom)
pub fn way_in_room(g: &mut crate::game::Game, room: RoomfightRoomId, from: V3, to: V3) -> bool { todo!() }

/// (src/game/digin.ts:472 pushable)
pub fn pushable(g: &mut crate::game::Game, d: RoomfightDefId, room: RoomfightRoomId, pp: V3, q: V3, stand: V3) -> bool { todo!() }

/// (src/game/digin.ts:478 pushSpot)
pub fn push_spot(g: &mut crate::game::Game, d: RoomfightDefId, room: RoomfightRoomId, e: crate::roomfight::Entry, pp: V3, stand: V3) -> Option<V3> { todo!() }

/// (src/game/digin.ts:493 startBrace)
pub fn start_brace(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &mut crate::digin::RoomSt, idx: f64) -> bool { todo!() }

/// (src/game/digin.ts:520 gatherTo)
pub fn gather_to(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man) -> Option<crate::digin::GatherTo> { todo!() }

/// (src/game/digin.ts:538 begin)
pub fn begin(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man) { todo!() }

/// (src/game/digin.ts:575 holds)
pub fn holds(g: &mut crate::game::Game, e: crate::roomfight::Entry) -> bool { todo!() }

/// (src/game/digin.ts:578 wouldBrace)
pub fn would_brace(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt, e: crate::roomfight::Entry) -> bool { todo!() }

/// (src/game/digin.ts:584 away)
pub fn away(g: &mut crate::game::Game, d: RoomfightDefId, m: crate::digin::Man, r: &crate::digin::RoomSt, e: crate::roomfight::Entry) -> bool { todo!() }

/// (src/game/digin.ts:590 nextDoors)
pub fn next_doors(g: &mut crate::game::Game, r: &crate::digin::RoomSt) -> Vec<f64> { todo!() }

/// (src/game/digin.ts:592 shutOrCover)
pub fn shut_or_cover(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &crate::digin::RoomSt) { todo!() }

/// (src/game/digin.ts:602 toCover)
pub fn to_cover(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &crate::digin::RoomSt, or_stay: Option<bool>) -> bool { todo!() }

/// (src/game/digin.ts:615 settle)
pub fn settle(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man) { todo!() }

/// (src/game/digin.ts:620 newMan)
pub fn new_man() -> crate::digin::Man { todo!() }

/// (src/game/digin.ts:621 drop)
pub fn drop(g: &mut crate::game::Game, m: &mut crate::digin::Man, gd: Option<GuardId>) { todo!() }

/// (src/game/digin.ts:627 caught)
pub fn caught(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, what: &str) { todo!() }

/// (src/game/digin.ts:629 afterWedge)
pub fn after_wedge(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &mut crate::digin::RoomSt) { todo!() }

/// (src/game/digin.ts:643 bringsHome)
pub fn brings_home(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: &mut crate::digin::RoomSt, e: Option<crate::roomfight::Entry>, p: PropId) -> bool { todo!() }

/// (src/game/digin.ts:652 haul)
pub fn haul(g: &mut crate::game::Game, d: RoomfightDefId, m: &mut crate::digin::Man, r: Option<&mut crate::digin::RoomSt>) { todo!() }

/// (src/game/digin.ts:668 digInTick)
pub fn dig_in_tick(g: &mut crate::game::Game, def: RoomfightDefId, _dt: f64) { todo!() }

/// (src/game/digin.ts:797 watch)
pub fn watch(g: &mut crate::game::Game) { todo!() }

/// (src/game/digin.ts:831 digInBraced)
pub fn dig_in_braced(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/digin.ts:836 digInSetOn)
pub fn dig_in_set_on(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> bool { todo!() }

/// (src/game/digin.ts:843 digInOrder)
pub fn dig_in_order(g: &mut crate::game::Game, def: RoomfightDefId, know: V3, beat: f64) -> bool { todo!() }

/// (src/game/digin.ts:853 digInRelease)
pub fn dig_in_release(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/digin.ts:855 digInPhase)
pub fn dig_in_phase(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/digin.ts:857 digInEntered)
pub fn dig_in_entered(g: &mut crate::game::Game, room: RoomfightRoomId) -> bool { todo!() }

/// (src/game/digin.ts:859 digInWedged)
pub fn dig_in_wedged(g: &mut crate::game::Game, room: RoomfightRoomId) -> Vec<crate::digin::DigInWedgedItem> { todo!() }

/// (src/game/digin.ts:861 digInLook)
pub fn dig_in_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) { todo!() }

/// (src/game/digin.ts:867 digInStays)
pub fn dig_in_stays(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/digin.ts:874 digInSet)
pub fn dig_in_set(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/digin.ts:881 digInRefuge)
pub fn dig_in_refuge(g: &mut crate::game::Game, def: RoomfightDefId, threat: V3) -> Option<crate::digin::DigInRefuge> { todo!() }

/// (src/game/digin.ts:902 legDist)
pub fn leg_dist(p: V3, a: V3, b: V3) -> f64 { todo!() }

/// (src/game/digin.ts:908 digInReset)
pub fn dig_in_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/digin.ts:916 digInSummary)
pub fn dig_in_summary(g: &mut crate::game::Game) -> String { todo!() }
