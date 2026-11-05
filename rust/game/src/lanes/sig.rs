#![allow(unused_variables)]
//! Every function and method of `lanes`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/lanes.ts:215 fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::lanes::State { todo!() }

/// (src/game/lanes.ts:217 S)
pub fn s<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::lanes::State { todo!() }

/// (src/game/lanes.ts:219 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/lanes.ts:222 mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:223 fit)
pub fn fit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:224 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lanes.ts:226 laneOf)
pub fn lane_of(g: &mut crate::game::Game, gd: GuardId) -> Option<LanesLaneId> { todo!() }

/// (src/game/lanes.ts:227 laneSay)
pub fn lane_say(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/lanes.ts:229 heldOn)
pub fn held_on(g: &mut crate::game::Game, gd: GuardId, h: crate::lanes::StateHeldValue) { todo!() }

/// (src/game/lanes.ts:230 heldOff)
pub fn held_off(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/lanes.ts:231 peekOn)
pub fn peek_on(g: &mut crate::game::Game, gd: GuardId, p: crate::lanes::Peek) { todo!() }

/// (src/game/lanes.ts:232 peekOff)
pub fn peek_off(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/lanes.ts:234 letGo)
pub fn let_go(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:236 lanesGone)
pub fn lanes_gone(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:237 eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/lanes.ts:238 cellAt)
pub fn cell_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/lanes.ts:239 inMask)
pub fn in_mask(g: &mut crate::game::Game, m: Option<&[u8]>, p: V3) -> bool { todo!() }

/// (src/game/lanes.ts:240 knows)
pub fn knows(g: &mut crate::game::Game, l: LanesLaneId, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:241 closeFight)
pub fn close_fight(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:242 pushing)
pub fn pushing(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:245 laneE)
pub fn lane_e(g: &mut crate::game::Game, l: LanesLaneId) -> Option<f64> { todo!() }

/// (src/game/lanes.ts:247 roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId, t: f64) -> bool { todo!() }

/// (src/game/lanes.ts:248 flanking)
pub fn flanking(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:249 upId)
pub fn up_id(g: &mut crate::game::Game, id_2: f64) -> bool { todo!() }

/// (src/game/lanes.ts:250 defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/lanes.ts:251 note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/lanes.ts:254 wedge)
pub fn wedge(g: &mut crate::game::Game, into: &mut JsSet<crate::Num>, from: V3, at: V3, e: Option<f64>) { todo!() }

/// (src/game/lanes.ts:268 remask)
pub fn remask(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:274 maskOf)
pub fn mask_of(g: &mut crate::game::Game, gd: GuardId) -> Option<Vec<u8>> { todo!() }

/// (src/game/lanes.ts:283 gunOn)
pub fn gun_on(g: &mut crate::game::Game, l: LanesLaneId, p: V3, wide: Option<f64>) -> bool { todo!() }

/// (src/game/lanes.ts:293 laneAt)
pub fn lane_at(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<LanesLaneId> { todo!() }

/// (src/game/lanes.ts:295 laneOn)
pub fn lane_on(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<LanesLaneId> { todo!() }

/// (src/game/lanes.ts:298 cost)
pub fn cost(g: &mut crate::game::Game, from: V3, at: V3, man: GuardId, down: bool, who: f64) { todo!() }

/// (src/game/lanes.ts:334 sees)
pub fn sees(g: &mut crate::game::Game, o: GuardId, at: V3, reach: f64) -> bool { todo!() }

/// (src/game/lanes.ts:339 becomeKnown)
pub fn become_known(g: &mut crate::game::Game, l: LanesLaneId, who: GuardId) { todo!() }

/// (src/game/lanes.ts:349 readRounds)
pub fn read_rounds(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:370 lanesTick)
pub fn lanes_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/lanes.ts:456 heldAt)
pub fn held_at(g: &mut crate::game::Game, l: LanesLaneId) -> Vec<GuardId> { todo!() }

/// (src/game/lanes.ts:458 backOut)
pub fn back_out(g: &mut crate::game::Game, gd: GuardId, l: LanesLaneId) -> bool { todo!() }

/// (src/game/lanes.ts:488 round)
pub fn round(g: &mut crate::game::Game, l: LanesLaneId, at: &[GuardId]) { todo!() }

/// (src/game/lanes.ts:532 stun)
pub fn stun(g: &mut crate::game::Game, l: LanesLaneId, at: &[GuardId]) { todo!() }

/// (src/game/lanes.ts:555 bangs)
pub fn bangs(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:577 gunsManned)
pub fn guns_manned(g: &mut crate::game::Game, l: LanesLaneId, eye: V3) -> Option<V3> { todo!() }

/// (src/game/lanes.ts:590 spotSeen)
pub fn spot_seen(g: &mut crate::game::Game, eye: V3, m: V3) -> bool { todo!() }

/// (src/game/lanes.ts:600 netSees)
pub fn net_sees(g: &mut crate::game::Game, l: LanesLaneId, but: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:606 stepper)
pub fn stepper(g: &mut crate::game::Game, l: LanesLaneId, skip: Option<&JsSet<GuardId>>, armed: bool, min_gun: Option<f64>) -> Option<crate::lanes::Stepper> { todo!() }

/// (src/game/lanes.ts:636 flankTick)
pub fn flank_tick(g: &mut crate::game::Game, l: LanesLaneId) { todo!() }

/// (src/game/lanes.ts:669 flankGo)
pub fn flank_go(g: &mut crate::game::Game, l: LanesLaneId) { todo!() }

/// (src/game/lanes.ts:681 pinStart)
pub fn pin_start(g: &mut crate::game::Game, l: LanesLaneId) { todo!() }

/// (src/game/lanes.ts:696 pinStep)
pub fn pin_step(g: &mut crate::game::Game, gd: GuardId, l: LanesLaneId, d: RoomfightDefId, seen: Option<V3>) { todo!() }

/// (src/game/lanes.ts:718 winding)
pub fn winding(g: &mut crate::game::Game, gd: GuardId, f: crate::lanes::PeekFrag) -> bool { todo!() }

/// (src/game/lanes.ts:722 fragOut)
pub fn frag_out(g: &mut crate::game::Game, l: LanesLaneId, at: &[GuardId]) { todo!() }

/// (src/game/lanes.ts:739 gunsMid)
pub fn guns_mid(g: &mut crate::game::Game, l: LanesLaneId) -> V3 { todo!() }

/// (src/game/lanes.ts:747 fragPlace)
pub fn frag_place(g: &mut crate::game::Game, gd: GuardId, to: V3, s: f64, going: bool) -> Option<crate::lanes::FragPlace> { todo!() }

/// (src/game/lanes.ts:766 fragStep)
pub fn frag_step(g: &mut crate::game::Game, gd: GuardId, l: LanesLaneId, d: RoomfightDefId) { todo!() }

/// (src/game/lanes.ts:798 peekStart)
pub fn peek_start(g: &mut crate::game::Game, l: LanesLaneId) { todo!() }

/// (src/game/lanes.ts:810 peekStep)
pub fn peek_step(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/lanes.ts:868 mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, l: LanesLaneId, p: V3) -> V3 { todo!() }

/// (src/game/lanes.ts:878 priceOf)
pub fn price_of(g: &mut crate::game::Game, m: Option<&[u8]>, base: Option<&crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> { todo!() }

/// (src/game/lanes.ts:883 lanePenalty)
pub fn lane_penalty(g: &mut crate::game::Game, gd: GuardId, base: Option<&crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> { todo!() }

/// (src/game/lanes.ts:889 laneGate)
pub fn lane_gate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:917 along)
pub fn along(g: &mut crate::game::Game, gd: GuardId, m: f64) -> V3 { todo!() }

/// (src/game/lanes.ts:918 plantShort)
pub fn plant_short(g: &mut crate::game::Game, gd: GuardId, lane: f64, look: V3) { todo!() }

/// (src/game/lanes.ts:926 ahead)
pub fn ahead(g: &mut crate::game::Game, m: &[u8], gd: GuardId, inside: bool) -> Option<V3> { todo!() }

/// (src/game/lanes.ts:938 laneBars)
pub fn lane_bars(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool { todo!() }

/// (src/game/lanes.ts:946 laneThrows)
pub fn lane_throws(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:952 laneKnownAt)
pub fn lane_known_at(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool { todo!() }

/// (src/game/lanes.ts:960 laneDoor)
pub fn lane_door(g: &mut crate::game::Game, gd: GuardId, far: V3, centre: V3, can: bool) -> bool { todo!() }

/// (src/game/lanes.ts:967 laneDrill)
pub fn lane_drill(g: &mut crate::game::Game, gd: GuardId, at: V3, far: V3, centre: V3, can: bool) -> bool { todo!() }

/// (src/game/lanes.ts:973 laneCovers)
pub fn lane_covers(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool { todo!() }

/// (src/game/lanes.ts:978 laneExposed)
pub fn lane_exposed(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:986 laneReload)
pub fn lane_reload(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/lanes.ts:987 lanesReset)
pub fn lanes_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:988 lanesStats)
pub fn lanes_stats(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/lanes.ts:989 lanesSummary)
pub fn lanes_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/lanes.ts:995 lanesList)
pub fn lanes_list(g: &mut crate::game::Game) -> Vec<crate::lanes::LanesListItem> { todo!() }

/// (src/game/lanes.ts:996 lanesKnows)
pub fn lanes_knows(g: &mut crate::game::Game, p: V3) -> bool { todo!() }


/// INVENTED: what a LanePenalty answers for a cell, the closure priceOf returned (lanes.ts:865), so a caller can hand one to nav_grid_find_path: |_g, ix, iz| lane_penalty_at(&pen, ix, iz). DearOver asks base first. A cell off the mask costs 1, as a Uint8Array's undefined does: read m with .get()
pub fn lane_penalty_at(pen: &crate::lanes::LanePenalty, ix: f64, iz: f64) -> f64 { todo!() }
