//! The frames of levels/tut-squad.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::io::Io as _;
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-squad.script.ts:75-134) the beat 'picket': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadPicketFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// g: a yield separates its declaration from a use
    pub g: Option<crate::director::DoorRef>,
    /// front: a yield separates its declaration from a use
    pub front: Option<crate::director::UnitRef>,
    /// g: a yield separates its declaration from a use
    pub g_2: Option<crate::director::DoorRef>,
    /// lane: a yield separates its declaration from a use
    pub lane: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_4: Option<crate::director::UnitRef>,
    /// off: kept by the closures of lastMan (up): written by one call and read by the next across yields
    pub off: bool,
}

impl crate::io::Io for TutSquadPicketFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.g.w(o); self.front.w(o); self.g_2.w(o); self.lane.w(o); self.u.w(o); self.u_2.w(o); self.u_3.w(o); self.u_4.w(o); self.off.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.g = crate::io::Io::r(i); s.front = crate::io::Io::r(i); s.g_2 = crate::io::Io::r(i); s.lane = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.u_2 = crate::io::Io::r(i); s.u_3 = crate::io::Io::r(i); s.u_4 = crate::io::Io::r(i); s.off = crate::io::Io::r(i); s } }

/// (levels/tut-squad.script.ts:143-149) the beat 'store': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadStoreFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
}

impl crate::io::Io for TutSquadStoreFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.men.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s } }

/// (levels/tut-squad.script.ts:161-195) the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::director::UnitRef>,
    /// since: a yield separates its declaration from a use
    pub since: f64,
    /// end: a yield separates its declaration from a use
    pub end: f64,
    /// said: a yield separates its declaration from a use
    pub said: bool,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// b: a yield separates its declaration from a use
    pub b: f64,
}

impl crate::io::Io for TutSquadShopFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.m.w(o); self.since.w(o); self.end.w(o); self.said.w(o); self.k.w(o); self.b.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.m = crate::io::Io::r(i); s.since = crate::io::Io::r(i); s.end = crate::io::Io::r(i); s.said = crate::io::Io::r(i); s.k = crate::io::Io::r(i); s.b = crate::io::Io::r(i); s } }

/// (levels/tut-squad.script.ts:205-235) the beat 'hall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadHallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSquadHallFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.men.w(o); self.u.w(o); self.u_2.w(o); self.u_3.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.u_2 = crate::io::Io::r(i); s.u_3 = crate::io::Io::r(i); s } }

/// (levels/tut-squad.script.ts:253-300) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSquadGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::director::UnitRef>,
    /// n: a yield separates its declaration from a use
    pub n: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSquadGunFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.m.w(o); self.n.w(o); self.u.w(o); self.u_2.w(o); self.u_3.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.m = crate::io::Io::r(i); s.n = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.u_2 = crate::io::Io::r(i); s.u_3 = crate::io::Io::r(i); s } }

/// (levels/tut-squad.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: alive, open
#[derive(Clone, Debug, Default)]
pub struct TutSquadVars {
    pub store_door: Option<crate::director::DoorRef>,
    pub shop_door: Option<crate::director::DoorRef>,
    pub side_door: Option<crate::director::DoorRef>,
    pub hall_door: Option<crate::director::DoorRef>,
    pub road_gate: Option<crate::director::DoorRef>,
    pub back_gate: Option<crate::director::DoorRef>,
    pub track_gate: Option<crate::director::DoorRef>,
    pub shop_fire: Option<crate::director::MarkRef>,
    pub shop_aim: [f64; 3],
    pub in_store: [f64; 3],
    pub in_hall: [f64; 3],
    pub hall_window: [f64; 3],
    pub shop_lane: Option<crate::director::VolumeRef>,
    pub shop_line: Option<crate::director::VolumeRef>,
    pub shop_in: Option<crate::director::VolumeRef>,
    pub hall_post: Vec<crate::director::MarkRef>,
    pub hall_back: Vec<crate::director::MarkRef>,
    pub lane: Option<crate::director::MarkRef>,
    pub camp: Vec<crate::director::MarkRef>,
    pub gate_post: Option<crate::director::MarkRef>,
    pub bags: Option<crate::director::MarkRef>,
    pub round: Vec<crate::director::MarkRef>,
    pub open: Option<crate::director::MarkRef>,
    pub out: Vec<crate::director::MarkRef>,
}

impl crate::io::Io for TutSquadVars { fn w(&self, o: &mut crate::io::W) { self.store_door.w(o); self.shop_door.w(o); self.side_door.w(o); self.hall_door.w(o); self.road_gate.w(o); self.back_gate.w(o); self.track_gate.w(o); self.shop_fire.w(o); self.shop_aim.w(o); self.in_store.w(o); self.in_hall.w(o); self.hall_window.w(o); self.shop_lane.w(o); self.shop_line.w(o); self.shop_in.w(o); self.hall_post.w(o); self.hall_back.w(o); self.lane.w(o); self.camp.w(o); self.gate_post.w(o); self.bags.w(o); self.round.w(o); self.open.w(o); self.out.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.store_door = crate::io::Io::r(i); s.shop_door = crate::io::Io::r(i); s.side_door = crate::io::Io::r(i); s.hall_door = crate::io::Io::r(i); s.road_gate = crate::io::Io::r(i); s.back_gate = crate::io::Io::r(i); s.track_gate = crate::io::Io::r(i); s.shop_fire = crate::io::Io::r(i); s.shop_aim = crate::io::Io::r(i); s.in_store = crate::io::Io::r(i); s.in_hall = crate::io::Io::r(i); s.hall_window = crate::io::Io::r(i); s.shop_lane = crate::io::Io::r(i); s.shop_line = crate::io::Io::r(i); s.shop_in = crate::io::Io::r(i); s.hall_post = crate::io::Io::r(i); s.hall_back = crate::io::Io::r(i); s.lane = crate::io::Io::r(i); s.camp = crate::io::Io::r(i); s.gate_post = crate::io::Io::r(i); s.bags = crate::io::Io::r(i); s.round = crate::io::Io::r(i); s.open = crate::io::Io::r(i); s.out = crate::io::Io::r(i); s } }
