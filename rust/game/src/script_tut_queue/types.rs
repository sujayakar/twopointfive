//! The frames of levels/tut-queue.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::io::Io as _;
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-queue.script.ts:44-82) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQueueGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::director::UnitRef>,
    /// n: a yield separates its declaration from a use
    pub n: Option<crate::director::UnitRef>,
    /// gone: a yield separates its declaration from a use
    pub gone: Option<crate::director::Cond>,
    /// hit: a yield separates its declaration from a use
    pub hit: Option<crate::director::Cond>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
    /// alone: a yield separates its declaration from a use
    pub alone: bool,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutQueueGunFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.m.w(o); self.n.w(o); self.gone.w(o); self.hit.w(o); self.r.w(o); self.alone.w(o); self.k.w(o); self.u.w(o); self.u_2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.m = crate::io::Io::r(i); s.n = crate::io::Io::r(i); s.gone = crate::io::Io::r(i); s.hit = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s.alone = crate::io::Io::r(i); s.k = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.u_2 = crate::io::Io::r(i); s } }

/// (levels/tut-queue.script.ts:88-124) the beat 'gates': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQueueGatesFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// road: a yield separates its declaration from a use
    pub road: Vec<Option<crate::director::UnitRef>>,
    /// lane: a yield separates its declaration from a use
    pub lane: Vec<Option<crate::director::UnitRef>>,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// off: a yield separates its declaration from a use
    pub off: bool,
}

impl crate::io::Io for TutQueueGatesFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.road.w(o); self.lane.w(o); self.men.w(o); self.off.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.road = crate::io::Io::r(i); s.lane = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.off = crate::io::Io::r(i); s } }

/// (levels/tut-queue.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: alive, isOpen
#[derive(Clone, Debug, Default)]
pub struct TutQueueVars {
    pub front: Option<crate::director::DoorRef>,
    pub back: Option<crate::director::DoorRef>,
    pub road_gate: Option<crate::director::DoorRef>,
    pub back_gate: Option<crate::director::DoorRef>,
    pub track_gate: Option<crate::director::DoorRef>,
    pub gun_mark: Option<crate::director::MarkRef>,
    pub mate_mark: Option<crate::director::MarkRef>,
    pub jamb: Option<crate::director::MarkRef>,
    pub doorway_at: [f64; 3],
    pub doorway: Option<crate::director::MarkRef>,
    pub road_camp: Vec<crate::director::MarkRef>,
    pub lane_camp: Vec<crate::director::MarkRef>,
    pub road_in: Vec<crate::director::MarkRef>,
    pub lane_in: Vec<crate::director::MarkRef>,
    pub post_a: Option<crate::director::MarkRef>,
    pub post_b: Option<crate::director::MarkRef>,
    pub road_on: Vec<crate::director::MarkRef>,
    pub lane_on: Vec<crate::director::MarkRef>,
}

impl crate::io::Io for TutQueueVars { fn w(&self, o: &mut crate::io::W) { self.front.w(o); self.back.w(o); self.road_gate.w(o); self.back_gate.w(o); self.track_gate.w(o); self.gun_mark.w(o); self.mate_mark.w(o); self.jamb.w(o); self.doorway_at.w(o); self.doorway.w(o); self.road_camp.w(o); self.lane_camp.w(o); self.road_in.w(o); self.lane_in.w(o); self.post_a.w(o); self.post_b.w(o); self.road_on.w(o); self.lane_on.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.front = crate::io::Io::r(i); s.back = crate::io::Io::r(i); s.road_gate = crate::io::Io::r(i); s.back_gate = crate::io::Io::r(i); s.track_gate = crate::io::Io::r(i); s.gun_mark = crate::io::Io::r(i); s.mate_mark = crate::io::Io::r(i); s.jamb = crate::io::Io::r(i); s.doorway_at = crate::io::Io::r(i); s.doorway = crate::io::Io::r(i); s.road_camp = crate::io::Io::r(i); s.lane_camp = crate::io::Io::r(i); s.road_in = crate::io::Io::r(i); s.lane_in = crate::io::Io::r(i); s.post_a = crate::io::Io::r(i); s.post_b = crate::io::Io::r(i); s.road_on = crate::io::Io::r(i); s.lane_on = crate::io::Io::r(i); s } }
