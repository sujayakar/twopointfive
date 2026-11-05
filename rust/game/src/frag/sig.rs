#![allow(unused_variables)]
//! Every function and method of `frag`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/frag.ts:214 fragStatsReset)
pub fn frag_stats_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/frag.ts:216 hash01)
pub fn hash01(a: f64, b: f64) -> f64 { todo!() }

/// (src/game/frag.ts:226 throughGlass)
pub fn through_glass<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, from: V3, view: crate::frag::GlassView, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/frag.ts:232 burstBlocked)
pub fn burst_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/frag.ts:237 burstLine)
pub fn burst_line(g: &mut crate::game::Game, from: V3, to: V3, view: Option<crate::frag::GlassView>) -> bool { todo!() }

/// (src/game/frag.ts:239 exposure)
pub fn exposure(g: &mut crate::game::Game, from: V3, feet: V3, head: V3, view: Option<crate::frag::GlassView>) -> f64 { todo!() }

/// (src/game/frag.ts:245 withoutLeaves)
pub fn without_leaves<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, leaves: &[BoxId], fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/frag.ts:251 leavesFor)
pub fn leaves_for(g: &mut crate::game::Game, from: V3) -> Vec<BoxId> { todo!() }

/// (src/game/frag.ts:259 exposedTo)
pub fn exposed_to(g: &mut crate::game::Game, from: V3, feet: V3, head: V3, view: Option<crate::frag::GlassView>) -> f64 { todo!() }

/// (src/game/frag.ts:270 wallsFor)
pub fn walls_for(g: &mut crate::game::Game, from: V3) -> Vec<crate::wallbreak::WallOpening> { todo!() }

/// (src/game/frag.ts:282 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/frag.ts:283 sameSide)
pub fn same_side(a: crate::teams::Team, b: crate::teams::Team) -> bool { todo!() }

/// (src/game/frag.ts:289 downAsked)
pub fn down_asked(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:291 lying)
pub fn lying(g: &mut crate::game::Game, gd: GuardId) -> Vec<V3> { todo!() }

/// (src/game/frag.ts:296 lyingShows)
pub fn lying_shows(g: &mut crate::game::Game, from: V3, pts: &[V3], view: crate::frag::GlassView) -> f64 { todo!() }

/// (src/game/frag.ts:297 blast)
pub fn blast(g: &mut crate::game::Game, at: V3, spec: &crate::frag::BlastSpec) -> crate::frag::BlastReport { todo!() }

/// (src/game/frag.ts:298 blastRaw)
pub fn blast_raw(g: &mut crate::game::Game, at: V3, spec: &crate::frag::BlastSpec) -> crate::frag::BlastReport { todo!() }

/// (src/game/frag.ts:438 detonateFrag)
pub fn detonate_frag(g: &mut crate::game::Game, it: ItemId) -> crate::frag::BlastReport { todo!() }

/// (src/game/frag.ts:443 fragFuse)
pub fn frag_fuse(g: &mut crate::game::Game, it: ItemId, dt: f64) { todo!() }

/// (src/game/frag.ts:452 booksOf)
pub fn books_of(g: &mut crate::game::Game) -> &mut Vec<crate::frag::Live> { todo!() }

/// (src/game/frag.ts:454 fragReset)
pub fn frag_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/frag.ts:460 comeDown)
pub fn come_down(g: &mut crate::game::Game, it: ItemId) -> V3 { todo!() }

/// (src/game/frag.ts:472 rollOn)
pub fn roll_on(g: &mut crate::game::Game, it: ItemId, touch_fuse: Option<f64>, props: Option<bool>, high: Option<bool>) -> crate::frag::RollOn { todo!() }

/// (src/game/frag.ts:503 restOfThrow)
pub fn rest_of_throw(g: &mut crate::game::Game, kind: crate::game::ThrowKind, from: V3, v0: V3) -> crate::frag::RestOfThrow { todo!() }

/// (src/game/frag.ts:509 restY)
pub fn rest_y(g: &mut crate::game::Game, y: f64) -> f64 { todo!() }

/// (src/game/frag.ts:511 fragRestGuess)
pub fn frag_rest_guess(g: &mut crate::game::Game, it: ItemId) -> V3 { todo!() }

/// (src/game/frag.ts:512 restOf)
pub fn rest_of(g: &mut crate::game::Game, l: usize) -> crate::frag::Rest { todo!() }

/// (src/game/frag.ts:518 shoulders)
pub fn shoulders(g: &mut crate::game::Game, a: V3, b: V3, q: V3) -> bool { todo!() }

/// (src/game/frag.ts:528 openLeaves)
pub fn open_leaves(g: &mut crate::game::Game, p: V3, r: f64) -> Vec<Vec<f64>> { todo!() }

/// (src/game/frag.ts:529 leafDist)
pub fn leaf_dist(l: &[Vec<f64>], x: f64, z: f64) -> f64 { todo!() }

/// (src/game/frag.ts:531 legByLeaf)
pub fn leg_by_leaf(g: &mut crate::game::Game, l: &[Vec<f64>], a: V3, b: V3, ref_: f64) -> bool { todo!() }

/// (src/game/frag.ts:544 fleeSpot)
pub fn flee_spot(g: &mut crate::game::Game, c_2: crate::frag::FleeSpotC, at: V3, speed: f64, left: f64, accel: Option<f64>, wise: Option<&crate::frag::FleeSpotWise>, who: Option<GuardId>) -> Option<V3> { todo!() }

/// (src/game/frag.ts:611 openWay)
pub fn open_way(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/frag.ts:613 inMouth)
pub fn in_mouth(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/frag.ts:621 fleeByDoor)
pub fn flee_by_door(g: &mut crate::game::Game, c_2: crate::frag::FleeByDoorC, at: V3, speed: f64, left: f64, accel: f64, wise: &crate::frag::FleeByDoorWise, who: Option<GuardId>) -> Option<crate::frag::FleeByDoor> { todo!() }

/// (src/game/frag.ts:685 takenBy)
pub fn taken_by(g: &mut crate::game::Game, l: Option<usize>, gd: GuardId, dz: V3) -> crate::frag::TakenBy { todo!() }

/// (src/game/frag.ts:699 fleeMates)
pub fn flee_mates(g: &mut crate::game::Game, gd: GuardId, dz: V3) -> Option<crate::frag::TakenBy> { todo!() }

/// (src/game/frag.ts:702 fragTick)
pub fn frag_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/frag.ts:704 fragSay)
pub fn frag_say(g: &mut crate::game::Game) { todo!() }

/// (src/game/frag.ts:705 stepFrag)
pub fn step_frag(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/frag.ts:775 fragFlee)
pub fn frag_flee(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> { todo!() }

/// (src/game/frag.ts:781 fragRunning)
pub fn frag_running(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:783 ownBang)
pub fn own_bang(g: &mut crate::game::Game, gd: GuardId, e: crate::frag::OwnBangE) -> bool { todo!() }

/// (src/game/frag.ts:785 ownNoise)
pub fn own_noise(g: &mut crate::game::Game, gd: GuardId, who: Option<f64>) -> bool { todo!() }

/// (src/game/frag.ts:790 samCue)
pub fn sam_cue(g: &mut crate::game::Game, l: usize, eye_at: V3) { todo!() }

/// (src/game/frag.ts:800 fragMarks)
pub fn frag_marks(g: &mut crate::game::Game) -> Vec<crate::frag::FragMarksItem> { todo!() }

/// (src/game/frag.ts:806 fragStepOff)
pub fn frag_step_off(g: &mut crate::game::Game, speed: Option<f64>) -> Option<V3> { todo!() }

/// (src/game/frag.ts:821 defendersOf)
pub fn defenders_of(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/frag.ts:823 dealCarriers)
pub fn deal_carriers(g: &mut crate::game::Game, was: Option<&crate::frag::DefBook>) { todo!() }

/// (src/game/frag.ts:834 fragCarriers)
pub fn frag_carriers(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/frag.ts:839 fragThrowTick)
pub fn frag_throw_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/frag.ts:906 aimSpots)
pub fn aim_spots(g: &mut crate::game::Game, gd: GuardId, from: Option<V3>, fix: V3, n: f64) -> Vec<V3> { todo!() }

/// (src/game/frag.ts:920 windUp)
pub fn wind_up(g: &mut crate::game::Game, c_2: usize, head: V3, from: V3, aim: V3, toward: Option<V3>) -> Option<crate::frag::WindUp> { todo!() }

/// (src/game/frag.ts:940 fragInFist)
pub fn frag_in_fist(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:942 armRound)
pub fn arm_round(g: &mut crate::game::Game, p: V3, to: V3) -> V3 { todo!() }

/// (src/game/frag.ts:948 free)
pub fn free(g: &mut crate::game::Game, gd: GuardId) -> Option<usize> { todo!() }

/// (src/game/frag.ts:956 fragReady)
pub fn frag_ready(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool { todo!() }

/// (src/game/frag.ts:969 fragThrowAt)
pub fn frag_throw_at(g: &mut crate::game::Game, gd: GuardId, at: V3, stand: V3, toward: V3) -> crate::frag::FragThrowAt { todo!() }

/// (src/game/frag.ts:977 fragWinding)
pub fn frag_winding(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:979 fragAway)
pub fn frag_away(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/frag.ts:983 lands)
pub fn lands(g: &mut crate::game::Game, sol: V3, p: V3, tol: f64) -> bool { todo!() }

/// (src/game/frag.ts:984 hostageNear)
pub fn hostage_near(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/frag.ts:986 ownClear)
pub fn own_clear(g: &mut crate::game::Game, gd: GuardId, land: V3, release: Option<bool>) -> bool { todo!() }


/// INVENTED (conventions 4, an object lent): what run_guard_script left in the script frag_flee lent goes back on the first book holding a row for him, the row frag_flee read (guards.ts:2143; squad.ts:130 writes arrived; frag.ts:687, 740, 745, 772 read it). None: the run left nothing, the row stays.
pub fn frag_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) { todo!() }
