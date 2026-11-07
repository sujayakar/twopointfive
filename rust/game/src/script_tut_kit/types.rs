//! The frames of levels/tut-kit.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::io::Io as _;
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-kit.script.ts:71-78) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitHoldFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:81-101) the beat 'gun': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitGunFrame {
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

impl crate::io::Io for TutKitGunFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.u.w(o); self.k.w(o); self.dir.w(o); self.i.w(o); self.mode.w(o); self.at_2.w(o); self.at_3.w(o); self.pts.w(o); self.at_4.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.k = crate::io::Io::r(i); s.dir = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.mode = crate::io::Io::r(i); s.at_2 = crate::io::Io::r(i); s.at_3 = crate::io::Io::r(i); s.pts = crate::io::Io::r(i); s.at_4 = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:102-102) the beat 'bags': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBagsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitBagsFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:103-103) the beat 'gap': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitGapFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitGapFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:104-104) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitBlindFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:108-114) the beat 'shop': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitShopFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

impl crate::io::Io for TutKitShopFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.u.w(o); self.r.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:118-126) the beat 'room': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitRoomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// pair: a yield separates its declaration from a use
    pub pair: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
    /// r2: a yield separates its declaration from a use
    pub r2: Option<crate::director::CondResult>,
}

impl crate::io::Io for TutKitRoomFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.pair.w(o); self.r.w(o); self.r2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.pair = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s.r2 = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:131-131) the beat 'loose': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitLooseFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitLooseFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:132-150) the beat 'trucks': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitTrucksFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// two: a yield separates its declaration from a use
    pub two: Vec<crate::script_tut_kit::TutKitTrucksTwo>,
    /// all: a yield separates its declaration from a use
    pub all: f64,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_kit::TutKitTrucksM>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// shots: a yield separates its declaration from a use
    pub shots: Vec<crate::director::Cond>,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_kit::TutKitTrucksM>,
}

impl crate::io::Io for TutKitTrucksFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.two.w(o); self.all.w(o); self.m.w(o); self.i.w(o); self.shots.w(o); self.m_2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.two = crate::io::Io::r(i); s.all = crate::io::Io::r(i); s.m = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.shots = crate::io::Io::r(i); s.m_2 = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:153-153) the beat 'call': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitCallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitCallFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:154-154) the beat 'box': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBoxFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitBoxFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:156-156) the beat 'dark': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitDarkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitDarkFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:157-157) the beat 'bunk': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBunkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitBunkFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:158-158) the beat 'wall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitWallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitWallFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:160-160) the beat 'bunkroom': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitBunkroomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* room: yield* room: the helper's frame while it runs. Step it until it answers None
    pub sub_room: Option<alloc::boxed::Box<crate::script_tut_kit::TutKitRoomHelperFrame>>,
}

impl crate::io::Io for TutKitBunkroomFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.sub_room.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.sub_room = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:161-161) the beat 'fuelroom': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitFuelroomFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* room: yield* room: the helper's frame while it runs. Step it until it answers None
    pub sub_room: Option<alloc::boxed::Box<crate::script_tut_kit::TutKitRoomHelperFrame>>,
}

impl crate::io::Io for TutKitFuelroomFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.sub_room.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.sub_room = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:166-166) the beat 'free': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitFreeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitFreeFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:167-170) the beat 'seen': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitSeenFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutKitSeenFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:171-185) the beat 'yard': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitYardFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// two: a yield separates its declaration from a use
    pub two: Vec<crate::script_tut_kit::TutKitYardTwo>,
}

impl crate::io::Io for TutKitYardFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.two.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.two = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts:55-63) the helper generator room: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutKitRoomHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// names: a parameter
    pub names: Vec<String>,
    /// inside: a parameter
    pub inside: Option<crate::director::VolumeRef>,
    /// box: a parameter
    pub box_: Option<crate::script_tut_kit::TutKitRoomHelperBox>,
    /// held: a yield separates its declaration from a use
    pub held: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

impl crate::io::Io for TutKitRoomHelperFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.names.w(o); self.inside.w(o); self.box_.w(o); self.held.w(o); self.r.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.names = crate::io::Io::r(i); s.inside = crate::io::Io::r(i); s.box_ = crate::io::Io::r(i); s.held = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s } }

/// (levels/tut-kit.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: aim, men, radio
#[derive(Clone, Debug, Default)]
pub struct TutKitVars {
    pub back_door: Option<crate::director::DoorRef>,
    pub wash_door: Option<crate::director::DoorRef>,
    pub shop_line: Option<crate::director::VolumeRef>,
    pub shop_in: Option<crate::director::VolumeRef>,
    pub store_in: Option<crate::director::VolumeRef>,
    pub bunk_in: Option<crate::director::VolumeRef>,
    pub fuel_in: Option<crate::director::VolumeRef>,
    pub wash_in: Option<crate::director::VolumeRef>,
    pub gun: Option<crate::director::LoadoutRef>,
    pub rifleman: Option<crate::director::LoadoutRef>,
    pub torch: Option<crate::director::LoadoutRef>,
}

impl crate::io::Io for TutKitVars { fn w(&self, o: &mut crate::io::W) { self.back_door.w(o); self.wash_door.w(o); self.shop_line.w(o); self.shop_in.w(o); self.store_in.w(o); self.bunk_in.w(o); self.fuel_in.w(o); self.wash_in.w(o); self.gun.w(o); self.rifleman.w(o); self.torch.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.back_door = crate::io::Io::r(i); s.wash_door = crate::io::Io::r(i); s.shop_line = crate::io::Io::r(i); s.shop_in = crate::io::Io::r(i); s.store_in = crate::io::Io::r(i); s.bunk_in = crate::io::Io::r(i); s.fuel_in = crate::io::Io::r(i); s.wash_in = crate::io::Io::r(i); s.gun = crate::io::Io::r(i); s.rifleman = crate::io::Io::r(i); s.torch = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitRoomHelperBox {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

impl crate::io::Io for TutKitRoomHelperBox { fn w(&self, o: &mut crate::io::W) { self.x0.w(o); self.x1.w(o); self.z0.w(o); self.z1.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.x0 = crate::io::Io::r(i); s.x1 = crate::io::Io::r(i); s.z0 = crate::io::Io::r(i); s.z1 = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitTrucksTwo {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutKitTrucksTwo { fn w(&self, o: &mut crate::io::W) { self.k.w(o); self.u.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.k = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitTrucksM {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutKitTrucksM { fn w(&self, o: &mut crate::io::W) { self.k.w(o); self.u.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.k = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutKitYardTwo {
    pub k: f64,
    pub u: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutKitYardTwo { fn w(&self, o: &mut crate::io::W) { self.k.w(o); self.u.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.k = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s } }
