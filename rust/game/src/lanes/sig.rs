#![allow(unused_variables)]
//! Every function and method of `lanes`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/lanes.ts:212 fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::lanes::State { todo!() }

/// (src/game/lanes.ts:214 S)
pub fn s<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::lanes::State { todo!() }

/// (src/game/lanes.ts:216 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/lanes.ts:219 mine)
pub fn mine(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:220 fit)
pub fn fit(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:221 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/lanes.ts:222 eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 { todo!() }

/// (src/game/lanes.ts:223 cellAt)
pub fn cell_at(g: &mut crate::game::Game, p: V3) -> f64 { todo!() }

/// (src/game/lanes.ts:224 inMask)
pub fn in_mask(g: &mut crate::game::Game, m: Option<&[u8]>, p: V3) -> bool { todo!() }

/// (src/game/lanes.ts:225 knows)
pub fn knows(g: &mut crate::game::Game, l: &crate::lanes::Lane, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:226 closeFight)
pub fn close_fight(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:227 pushing)
pub fn pushing(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:230 laneE)
pub fn lane_e(g: &mut crate::game::Game, l: &crate::lanes::Lane) -> Option<f64> { todo!() }

/// (src/game/lanes.ts:232 roused)
pub fn roused(g: &mut crate::game::Game, gd: GuardId, t: f64) -> bool { todo!() }

/// (src/game/lanes.ts:233 flanking)
pub fn flanking(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:234 upId)
pub fn up_id(g: &mut crate::game::Game, id_2: f64) -> bool { todo!() }

/// (src/game/lanes.ts:235 defOf)
pub fn def_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightDefId> { todo!() }

/// (src/game/lanes.ts:236 note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) { todo!() }

/// (src/game/lanes.ts:239 wedge)
pub fn wedge(g: &mut crate::game::Game, into: &mut JsSet<crate::Num>, from: V3, at: V3, e: Option<f64>) { todo!() }

/// (src/game/lanes.ts:253 remask)
pub fn remask(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:259 maskOf)
pub fn mask_of(g: &mut crate::game::Game, gd: GuardId) -> Option<Vec<u8>> { todo!() }

/// (src/game/lanes.ts:268 gunOn)
pub fn gun_on(g: &mut crate::game::Game, l: &crate::lanes::Lane, p: V3, wide: Option<f64>) -> bool { todo!() }

/// (src/game/lanes.ts:278 laneAt)
pub fn lane_at(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<crate::lanes::Lane> { todo!() }

/// (src/game/lanes.ts:280 laneOn)
pub fn lane_on(g: &mut crate::game::Game, p: V3, gd: GuardId) -> Option<crate::lanes::Lane> { todo!() }

/// (src/game/lanes.ts:283 cost)
pub fn cost(g: &mut crate::game::Game, from: V3, at: V3, man: GuardId, down: bool, who: f64) { todo!() }

/// (src/game/lanes.ts:319 sees)
pub fn sees(g: &mut crate::game::Game, o: GuardId, at: V3, reach: f64) -> bool { todo!() }

/// (src/game/lanes.ts:324 becomeKnown)
pub fn become_known(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, who: GuardId) { todo!() }

/// (src/game/lanes.ts:334 readRounds)
pub fn read_rounds(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:355 lanesTick)
pub fn lanes_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/lanes.ts:441 heldAt)
pub fn held_at(g: &mut crate::game::Game, l: &crate::lanes::Lane) -> Vec<GuardId> { todo!() }

/// (src/game/lanes.ts:443 backOut)
pub fn back_out(g: &mut crate::game::Game, gd: GuardId, l: &crate::lanes::Lane) -> bool { todo!() }

/// (src/game/lanes.ts:473 round)
pub fn round(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, at: &[GuardId]) { todo!() }

/// (src/game/lanes.ts:517 stun)
pub fn stun(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, at: &[GuardId]) { todo!() }

/// (src/game/lanes.ts:540 bangs)
pub fn bangs(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:562 gunsManned)
pub fn guns_manned(g: &mut crate::game::Game, l: &crate::lanes::Lane, eye: V3) -> Option<V3> { todo!() }

/// (src/game/lanes.ts:575 spotSeen)
pub fn spot_seen(g: &mut crate::game::Game, eye: V3, m: V3) -> bool { todo!() }

/// (src/game/lanes.ts:585 netSees)
pub fn net_sees(g: &mut crate::game::Game, l: &crate::lanes::Lane, but: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:591 stepper)
pub fn stepper(g: &mut crate::game::Game, l: &crate::lanes::Lane, skip: Option<&JsSet<GuardId>>, armed: bool, min_gun: Option<f64>) -> Option<crate::lanes::Stepper> { todo!() }

/// (src/game/lanes.ts:621 flankTick)
pub fn flank_tick(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) { todo!() }

/// (src/game/lanes.ts:654 flankGo)
pub fn flank_go(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) { todo!() }

/// (src/game/lanes.ts:666 pinStart)
pub fn pin_start(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) { todo!() }

/// (src/game/lanes.ts:681 pinStep)
pub fn pin_step(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::lanes::Peek, l: &mut crate::lanes::Lane, d: RoomfightDefId, seen: Option<V3>, back: &mut dyn FnMut(&mut crate::game::Game)) { todo!() }

/// (src/game/lanes.ts:703 winding)
pub fn winding(g: &mut crate::game::Game, gd: GuardId, f: crate::lanes::PeekFrag) -> bool { todo!() }

/// (src/game/lanes.ts:707 fragOut)
pub fn frag_out(g: &mut crate::game::Game, l: &mut crate::lanes::Lane, at: &[GuardId]) { todo!() }

/// (src/game/lanes.ts:724 gunsMid)
pub fn guns_mid(l: &crate::lanes::Lane) -> V3 { todo!() }

/// (src/game/lanes.ts:732 fragPlace)
pub fn frag_place(g: &mut crate::game::Game, gd: GuardId, to: V3, s: f64, going: bool) -> Option<crate::lanes::FragPlace> { todo!() }

/// (src/game/lanes.ts:751 fragStep)
pub fn frag_step(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::lanes::Peek, l: &mut crate::lanes::Lane, d: RoomfightDefId, back: &mut dyn FnMut(&mut crate::game::Game)) { todo!() }

/// (src/game/lanes.ts:783 peekStart)
pub fn peek_start(g: &mut crate::game::Game, l: &mut crate::lanes::Lane) { todo!() }

/// (src/game/lanes.ts:795 peekStep)
pub fn peek_step(g: &mut crate::game::Game, gd: GuardId, p: &mut crate::lanes::Peek) { todo!() }

/// (src/game/lanes.ts:853 mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, l: &crate::lanes::Lane, p: V3) -> V3 { todo!() }

/// (src/game/lanes.ts:863 priceOf)
pub fn price_of(g: &mut crate::game::Game, m: Option<&[u8]>, base: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<crate::lanes::LanePenalty> { todo!() }

/// (src/game/lanes.ts:868 lanePenalty)
pub fn lane_penalty(g: &mut crate::game::Game, gd: GuardId, base: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> f64>) -> Option<crate::lanes::LanePenalty> { todo!() }

/// (src/game/lanes.ts:874 laneGate)
pub fn lane_gate(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:902 along)
pub fn along(g: &mut crate::game::Game, gd: GuardId, m: f64) -> V3 { todo!() }

/// (src/game/lanes.ts:903 plantShort)
pub fn plant_short(g: &mut crate::game::Game, gd: GuardId, lane: f64, look: V3) { todo!() }

/// (src/game/lanes.ts:911 ahead)
pub fn ahead(g: &mut crate::game::Game, m: &[u8], gd: GuardId, inside: bool) -> Option<V3> { todo!() }

/// (src/game/lanes.ts:923 laneHolds)
pub fn lane_holds(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool { todo!() }

/// (src/game/lanes.ts:931 laneThrows)
pub fn lane_throws(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:937 laneWarns)
pub fn lane_warns(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool { todo!() }

/// (src/game/lanes.ts:945 laneDoor)
pub fn lane_door(g: &mut crate::game::Game, gd: GuardId, far: V3, centre: V3, can: bool) -> bool { todo!() }

/// (src/game/lanes.ts:952 laneDrill)
pub fn lane_drill(g: &mut crate::game::Game, gd: GuardId, at: V3, far: V3, centre: V3, can: bool) -> bool { todo!() }

/// (src/game/lanes.ts:958 laneCovers)
pub fn lane_covers(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool { todo!() }

/// (src/game/lanes.ts:963 laneExposed)
pub fn lane_exposed(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/lanes.ts:971 laneReload)
pub fn lane_reload(g: &mut crate::game::Game, gd: GuardId) { todo!() }

/// (src/game/lanes.ts:972 lanesReset)
pub fn lanes_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/lanes.ts:973 lanesStats)
pub fn lanes_stats(g: &mut crate::game::Game) -> JsMap<String, f64> { todo!() }

/// (src/game/lanes.ts:974 lanesSummary)
pub fn lanes_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/lanes.ts:980 lanesList)
pub fn lanes_list(g: &mut crate::game::Game) -> Vec<crate::lanes::LanesListItem> { todo!() }

/// (src/game/lanes.ts:981 lanesKnows)
pub fn lanes_knows(g: &mut crate::game::Game, p: V3) -> bool { todo!() }
