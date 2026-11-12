//! The types of `duty`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHAT A MAN IS HELD TO, whoever gave it him: one record for every man of any side. A squad's order, a defender's post, a mission's mark and a night man's lockdown all write it
/// (duty.rs), and a rule that asks whether he may leave, where his gun is meant to be or where he may go reads it there.
#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Duty {
    /// the squad he is of, where it is no player's (`group::of`)
    pub group: Option<GroupId>,
    /// what he has been given to do; None: nothing
    pub task: Option<TaskId>,
    /// he does not leave where he is of his own accord · who last changed that, and when
    pub holds: bool,
    pub holds_by: Giver,
    /// … and he fights from it
    pub pinned: bool,
    pub pinned_by: Giver,
    /// he stays in his squad's rooms
    pub rooms: bool,
    /// what it has his gun on, this tick
    pub gun: Option<V3>,
}

impl Duty {
    /// nothing holds him or pins him: he goes where his fight takes him
    pub fn free(&self) -> bool { !self.holds && !self.pinned }
}

/// The module that last changed a word of it, and when: read by nothing in the tick, only by what says why he does what he does
#[derive(Clone, Copy, PartialEq, Debug, Default, ToBytes)]
pub struct Giver {
    pub by: &'static str,
    pub since: f64,
}

/// WHAT HE HAS BEEN GIVEN TO DO, by whoever gives his group its tasks: where he is to be, what he is to have his gun on, and how he goes there.
#[derive(Clone, Debug, ToBytes, Default)]
pub struct Task {
    pub order: String,
    pub t0: f64,
    pub post: Option<V3>,
    pub follow: f64,
    pub face: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: crate::character::Stance,
    pub upper: crate::character::UpperMode,
    pub stack_side: f64,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub going: crate::orders::Doing,
    pub there: crate::orders::Doing,
    pub set_line: Option<String>,
    pub group: f64,
    pub set_said: bool,
    pub goal: V3,
    pub arrived: bool,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: f64,
    pub moved_t: f64,
    pub light: Option<bool>,
    pub low: bool,
    pub yield_t: f64,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Vec<V3>,
    pub anchor: V3,
    pub mark: V3,
    pub mark_t: f64,
    pub had: Option<V3>,
    pub back_n: Option<f64>,
    pub shy: Option<DoorId>,
    pub thru: Option<DoorId>,
    pub via_for: Option<V3>,
    pub via_t: Option<f64>,
    pub pace: Option<f64>,
    pub spare: Option<bool>,
    pub from: Option<V3>,
    pub foe: Option<GuardId>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub fit: Option<crate::roomfit::Fit>,
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
    /// true at assign and at the goal's unguarded writes, false once his script's goal is the order's own. `tell_him` passes held: !(new_leg && the order's own goal is the arm chosen) and clears it then
    pub new_leg: bool,
}

/// … as it is given: what is left out is as a fresh task has it
#[derive(Clone, Debug, ToBytes, Default)]
pub struct TaskGiven {
    pub post: Option<V3>,
    pub going: crate::orders::Doing,
    pub order: String,
    pub face: Option<f64>,
    pub there: crate::orders::Doing,
    pub t0: Option<f64>,
    pub follow: Option<f64>,
    pub watch: Option<V3>,
    pub door: Option<DoorId>,
    pub strafe: Option<bool>,
    pub stance: Option<crate::character::Stance>,
    pub upper: Option<crate::character::UpperMode>,
    pub stack_side: Option<f64>,
    pub move_: Option<crate::doortools::DoorToolsDoneItemLaneBackMove>,
    pub set_line: Option<String>,
    pub group: Option<f64>,
    pub set_said: Option<bool>,
    pub goal: Option<V3>,
    pub arrived: Option<bool>,
    pub held_set: Option<bool>,
    pub held_fire: Option<bool>,
    pub arr_t: Option<f64>,
    pub moved_t: Option<f64>,
    pub light: Option<bool>,
    pub low: Option<bool>,
    pub yield_t: Option<f64>,
    pub face_now: Option<f64>,
    pub search: Option<bool>,
    pub search_t: Option<f64>,
    pub via: Option<Vec<V3>>,
    pub anchor: Option<V3>,
    pub mark: Option<V3>,
    pub mark_t: Option<f64>,
    pub had: Option<V3>,
    pub back_n: Option<f64>,
    pub shy: Option<DoorId>,
    pub thru: Option<DoorId>,
    pub via_for: Option<V3>,
    pub via_t: Option<f64>,
    pub pace: Option<f64>,
    pub spare: Option<bool>,
    pub from: Option<V3>,
    pub foe: Option<GuardId>,
    pub crouch: Option<bool>,
    pub lean: Option<f64>,
    pub fit: Option<crate::roomfit::Fit>,
    pub tries: Option<crate::doortools::DoorToolsDoneItemLaneBackTries>,
}
