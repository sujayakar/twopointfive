//! The frames of levels/tut-solo.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::io::Io as _;
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-solo.script.ts:52-55) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloHoldFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:59-59) the beat 'cease': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloCeaseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloCeaseFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:60-73) the beat 'road': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloRoadFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// s: a yield separates its declaration from a use
    pub s: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
}

impl crate::io::Io for TutSoloRoadFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.s.w(o); self.i.w(o); self.at_2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.s = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.at_2 = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:77-77) the beat 'way': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloWayFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloWayFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:80-100) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloGunFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// dir: a yield separates its declaration from a use
    pub dir: f64,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// mode: a yield separates its declaration from a use
    pub mode: Option<String>,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
    /// at: a yield separates its declaration from a use
    pub at_3: [f64; 3],
    /// pts: a yield separates its declaration from a use
    pub pts: Vec<[f64; 3]>,
    /// at: a yield separates its declaration from a use
    pub at_4: [f64; 3],
}

impl crate::io::Io for TutSoloGunFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.u.w(o); self.k.w(o); self.dir.w(o); self.i.w(o); self.mode.w(o); self.at_2.w(o); self.at_3.w(o); self.pts.w(o); self.at_4.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.k = crate::io::Io::r(i); s.dir = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.mode = crate::io::Io::r(i); s.at_2 = crate::io::Io::r(i); s.at_3 = crate::io::Io::r(i); s.pts = crate::io::Io::r(i); s.at_4 = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:101-101) the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloBagsFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:102-102) the beat 'gap': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloGapFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloGapFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:103-103) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloBlindFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:107-113) the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

impl crate::io::Io for TutSoloShopFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.u.w(o); self.r.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:118-126) the beat 'room': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloRoomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
    /// r2: a yield separates its declaration from a use
    pub r2: Option<crate::director::CondResult>,
}

impl crate::io::Io for TutSoloRoomFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.men.w(o); self.r.w(o); self.r2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s.r2 = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:131-131) the beat 'loose': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloLooseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSoloLooseFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts:132-150) the beat 'trucks': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSoloTrucksFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::script_tut_solo::TutSoloTrucksMen>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_solo::TutSoloTrucksM>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// shots: a yield separates its declaration from a use
    pub shots: Vec<crate::director::Cond>,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_solo::TutSoloTrucksM>,
}

impl crate::io::Io for TutSoloTrucksFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.men.w(o); self.all.w(o); self.m.w(o); self.i.w(o); self.shots.w(o); self.m_2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.all = crate::io::Io::r(i); s.m = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.shots = crate::io::Io::r(i); s.m_2 = crate::io::Io::r(i); s } }

/// (levels/tut-solo.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim, pair
#[derive(Clone, Debug, Default)]
pub struct TutSoloVars {
    pub back_door: Option<crate::director::DoorRef>,
    pub inner_door: Option<crate::director::DoorRef>,
    pub shop_line: Option<crate::director::VolumeRef>,
    pub shop_in: Option<crate::director::VolumeRef>,
    pub store_in: Option<crate::director::VolumeRef>,
    pub road_east: Option<crate::director::VolumeRef>,
    pub road_nose: Option<crate::director::VolumeRef>,
    pub rifleman: Option<crate::director::LoadoutRef>,
    pub staff: Vec<String>,
    pub aims: Option<JsMap<String, V3>>,
}

impl crate::io::Io for TutSoloVars { fn w(&self, o: &mut crate::io::W) { self.back_door.w(o); self.inner_door.w(o); self.shop_line.w(o); self.shop_in.w(o); self.store_in.w(o); self.road_east.w(o); self.road_nose.w(o); self.rifleman.w(o); self.staff.w(o); self.aims.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.back_door = crate::io::Io::r(i); s.inner_door = crate::io::Io::r(i); s.shop_line = crate::io::Io::r(i); s.shop_in = crate::io::Io::r(i); s.store_in = crate::io::Io::r(i); s.road_east = crate::io::Io::r(i); s.road_nose = crate::io::Io::r(i); s.rifleman = crate::io::Io::r(i); s.staff = crate::io::Io::r(i); s.aims = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSoloTrucksMen {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSoloTrucksMen { fn w(&self, o: &mut crate::io::W) { self.k.w(o); self.u.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.k = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSoloTrucksM {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSoloTrucksM { fn w(&self, o: &mut crate::io::W) { self.k.w(o); self.u.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.k = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s } }
