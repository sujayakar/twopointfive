#![allow(unused_variables)]
//! Every function and method of `frag`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/frag.ts:213 fragStatsReset)
pub fn frag_stats_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/frag.ts:215 hash01)
pub fn hash01(a: f64, b: f64) -> f64 { todo!() }

/// (src/game/frag.ts:225 throughGlass)
pub fn through_glass<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, from: V3, view: crate::frag::GlassView, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/frag.ts:231 burstBlocked)
pub fn burst_blocked(g: &mut crate::game::Game, a: V3, b: V3) -> bool { todo!() }

/// (src/game/frag.ts:236 burstLine)
pub fn burst_line(g: &mut crate::game::Game, from: V3, to: V3, view: Option<crate::frag::GlassView>) -> bool { todo!() }

/// (src/game/frag.ts:238 exposure)
pub fn exposure(g: &mut crate::game::Game, from: V3, feet: V3, head: V3, view: Option<crate::frag::GlassView>) -> f64 { todo!() }

/// (src/game/frag.ts:244 withoutLeaves)
pub fn without_leaves<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, leaves: &[BoxId], fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/frag.ts:250 leavesFor)
pub fn leaves_for(g: &mut crate::game::Game, from: V3) -> Vec<BoxId> { todo!() }

/// (src/game/frag.ts:258 exposedTo)
pub fn exposed_to(g: &mut crate::game::Game, from: V3, feet: V3, head: V3, view: Option<crate::frag::GlassView>) -> f64 { todo!() }

/// (src/game/frag.ts:269 wallsFor)
pub fn walls_for(g: &mut crate::game::Game, from: V3) -> Vec<crate::wallbreak::WallOpening> { todo!() }

/// (src/game/frag.ts:281 flat)
pub fn flat(a: V3, b: V3) -> f64 { todo!() }

/// (src/game/frag.ts:282 sameSide)
pub fn same_side(a: crate::teams::Team, b: crate::teams::Team) -> bool { todo!() }

/// (src/game/frag.ts:288 downAsked)
pub fn down_asked(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:290 lying)
pub fn lying(g: &mut crate::game::Game, gd: GuardId) -> Vec<V3> { todo!() }

/// (src/game/frag.ts:295 lyingShows)
pub fn lying_shows(g: &mut crate::game::Game, from: V3, pts: &[V3], view: crate::frag::GlassView) -> f64 { todo!() }

/// (src/game/frag.ts:296 blast)
pub fn blast(g: &mut crate::game::Game, at: V3, spec: &crate::frag::BlastSpec) -> crate::frag::BlastReport { todo!() }

/// (src/game/frag.ts:297 blastRaw)
pub fn blast_raw(g: &mut crate::game::Game, at: V3, spec: &crate::frag::BlastSpec) -> crate::frag::BlastReport { todo!() }

/// (src/game/frag.ts:437 detonateFrag)
pub fn detonate_frag(g: &mut crate::game::Game, it: ItemId) -> crate::frag::BlastReport { todo!() }

/// (src/game/frag.ts:442 fragFuse)
pub fn frag_fuse(g: &mut crate::game::Game, it: ItemId, dt: f64) { todo!() }

/// (src/game/frag.ts:450 booksOf)
pub fn books_of(g: &mut crate::game::Game) -> Vec<crate::frag::Live> { todo!() }

/// (src/game/frag.ts:452 fragReset)
pub fn frag_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/frag.ts:458 comeDown)
pub fn come_down(g: &mut crate::game::Game, it: ItemId) -> V3 { todo!() }

/// (src/game/frag.ts:470 rollOn)
pub fn roll_on(g: &mut crate::game::Game, it: ItemId, touch_fuse: Option<f64>, props: Option<bool>, high: Option<bool>) -> crate::frag::RollOn { todo!() }

/// (src/game/frag.ts:501 restOfThrow)
pub fn rest_of_throw(g: &mut crate::game::Game, kind: crate::game::ThrowKind, from: V3, v0: V3) -> crate::frag::RestOfThrow { todo!() }

/// (src/game/frag.ts:507 restY)
pub fn rest_y(g: &mut crate::game::Game, y: f64) -> f64 { todo!() }

/// (src/game/frag.ts:509 fragRestGuess)
pub fn frag_rest_guess(g: &mut crate::game::Game, it: ItemId) -> V3 { todo!() }

/// (src/game/frag.ts:510 restOf)
pub fn rest_of(g: &mut crate::game::Game, l: &mut crate::frag::Live) -> crate::frag::Rest { todo!() }

/// (src/game/frag.ts:516 shoulders)
pub fn shoulders(g: &mut crate::game::Game, a: V3, b: V3, q: V3) -> bool { todo!() }

/// (src/game/frag.ts:526 openLeaves)
pub fn open_leaves(g: &mut crate::game::Game, p: V3, r: f64) -> Vec<Vec<f64>> { todo!() }

/// (src/game/frag.ts:527 leafDist)
pub fn leaf_dist(l: &[Vec<f64>], x: f64, z: f64) -> f64 { todo!() }

/// (src/game/frag.ts:529 legByLeaf)
pub fn leg_by_leaf(g: &mut crate::game::Game, l: &[Vec<f64>], a: V3, b: V3, ref_: f64) -> bool { todo!() }

/// (src/game/frag.ts:542 fleeSpot)
pub fn flee_spot(g: &mut crate::game::Game, c_2: crate::frag::FleeSpotC, at: V3, speed: f64, left: f64, accel: Option<f64>, wise: Option<&crate::frag::FleeSpotWise>, who: Option<GuardId>) -> Option<V3> { todo!() }

/// (src/game/frag.ts:609 openWay)
pub fn open_way(g: &mut crate::game::Game, d: DoorId) -> bool { todo!() }

/// (src/game/frag.ts:611 inMouth)
pub fn in_mouth(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/frag.ts:619 fleeByDoor)
pub fn flee_by_door(g: &mut crate::game::Game, c_2: crate::frag::FleeByDoorC, at: V3, speed: f64, left: f64, accel: f64, wise: &crate::frag::FleeByDoorWise, who: Option<GuardId>) -> Option<crate::frag::FleeByDoor> { todo!() }

/// (src/game/frag.ts:683 takenBy)
pub fn taken_by(g: &mut crate::game::Game, l: Option<&crate::frag::Live>, gd: GuardId, dz: V3) -> crate::frag::TakenBy { todo!() }

/// (src/game/frag.ts:697 fleeMates)
pub fn flee_mates(g: &mut crate::game::Game, gd: GuardId, dz: V3) -> Option<crate::squadthrow::SquadThrowKeptDuckM> { todo!() }

/// (src/game/frag.ts:700 fragTick)
pub fn frag_tick(g: &mut crate::game::Game, _dt: f64) { todo!() }

/// (src/game/frag.ts:770 fragFlee)
pub fn frag_flee(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::GuardScript> { todo!() }

/// (src/game/frag.ts:776 fragRunning)
pub fn frag_running(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:778 ownBang)
pub fn own_bang(g: &mut crate::game::Game, gd: GuardId, e: crate::frag::OwnBangE) -> bool { todo!() }

/// (src/game/frag.ts:780 ownNoise)
pub fn own_noise(g: &mut crate::game::Game, gd: GuardId, who: Option<f64>) -> bool { todo!() }

/// (src/game/frag.ts:785 samCue)
pub fn sam_cue(g: &mut crate::game::Game, l: &mut crate::frag::Live, eye_at: V3) { todo!() }

/// (src/game/frag.ts:795 fragMarks)
pub fn frag_marks(g: &mut crate::game::Game) -> Vec<crate::frag::FragMarksItem> { todo!() }

/// (src/game/frag.ts:801 fragStepOff)
pub fn frag_step_off(g: &mut crate::game::Game, speed: Option<f64>) -> Option<V3> { todo!() }

/// (src/game/frag.ts:816 defendersOf)
pub fn defenders_of(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/frag.ts:818 dealCarriers)
pub fn deal_carriers(g: &mut crate::game::Game, was: Option<&crate::frag::DefBook>) -> crate::frag::DefBook { todo!() }

/// (src/game/frag.ts:829 fragCarriers)
pub fn frag_carriers(g: &mut crate::game::Game) -> Vec<GuardId> { todo!() }

/// (src/game/frag.ts:834 fragThrowTick)
pub fn frag_throw_tick(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/frag.ts:901 aimSpots)
pub fn aim_spots(g: &mut crate::game::Game, gd: GuardId, from: Option<V3>, fix: V3, n: f64) -> Vec<V3> { todo!() }

/// (src/game/frag.ts:915 windUp)
pub fn wind_up(g: &mut crate::game::Game, c_2: &mut crate::frag::Carrier, head: V3, from: V3, aim: V3, toward: Option<V3>) -> Option<crate::frag::WindUp> { todo!() }

/// (src/game/frag.ts:935 fragInFist)
pub fn frag_in_fist(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:937 armRound)
pub fn arm_round(g: &mut crate::game::Game, p: V3, to: V3) -> V3 { todo!() }

/// (src/game/frag.ts:943 free)
pub fn free(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::frag::Free> { todo!() }

/// (src/game/frag.ts:951 fragReady)
pub fn frag_ready(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool { todo!() }

/// (src/game/frag.ts:964 fragThrowAt)
pub fn frag_throw_at(g: &mut crate::game::Game, gd: GuardId, at: V3, stand: V3, toward: V3) -> crate::frag::FragThrowAt { todo!() }

/// (src/game/frag.ts:972 fragWinding)
pub fn frag_winding(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/frag.ts:974 fragAway)
pub fn frag_away(g: &mut crate::game::Game, gd: GuardId) -> f64 { todo!() }

/// (src/game/frag.ts:978 lands)
pub fn lands(g: &mut crate::game::Game, sol: Option<crate::frag::LandsSol>, p: V3, tol: f64) -> bool { todo!() }

/// (src/game/frag.ts:979 hostageNear)
pub fn hostage_near(g: &mut crate::game::Game, p: V3) -> bool { todo!() }

/// (src/game/frag.ts:981 ownClear)
pub fn own_clear(g: &mut crate::game::Game, gd: GuardId, land: V3, release: Option<bool>) -> bool { todo!() }
