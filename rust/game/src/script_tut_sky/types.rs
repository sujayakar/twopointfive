//! The frames of levels/tut-sky.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::io::Io as _;
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-sky.script.ts:107-114) the beat 'hold': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyHoldFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// held: a yield separates its declaration from a use
    pub held: Vec<crate::director::UnitRef>,
}

impl crate::io::Io for TutSkyHoldFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.held.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.held = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:119-126) the beat 'ambush': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyAmbushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* rake: yield* rake: the helper's frame while it runs. Step it until it answers None
    pub sub_rake: Option<alloc::boxed::Box<crate::script_tut_sky::TutSkyRakeHelperFrame>>,
}

impl crate::io::Io for TutSkyAmbushFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.sub_rake.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.sub_rake = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:127-132) the beat 'pinned': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyPinnedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// who: a yield separates its declaration from a use
    pub who: String,
    /// line: a yield separates its declaration from a use
    pub line: String,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSkyPinnedFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.k.w(o); self.who.w(o); self.line.w(o); self.u.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.k = crate::io::Io::r(i); s.who = crate::io::Io::r(i); s.line = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:142-154) the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkySmokeFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* smoke: yield* smoke: the helper's frame while it runs. Step it until it answers None
    pub sub_smoke: Option<alloc::boxed::Box<crate::script_tut_sky::TutSkySmokeHelperFrame>>,
}

impl crate::io::Io for TutSkySmokeFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.sub_smoke.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.sub_smoke = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:155-163) the beat 'break': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyBreakFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyBreakFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:164-164) the beat 'ordered': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyOrderedFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyOrderedFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:167-167) the beat 'blind': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyBlindFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// yield* harass: yield* harass: the helper's frame while it runs. Step it until it answers None
    pub sub_harass: Option<alloc::boxed::Box<crate::script_tut_sky::TutSkyHarassHelperFrame>>,
}

impl crate::io::Io for TutSkyBlindFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.sub_harass.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.sub_harass = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:173-187) the beat 'push': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyPushFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// k: a yield separates its declaration from a use
    pub k: f64,
    /// left: a yield separates its declaration from a use
    pub left: Vec<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSkyPushFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.k.w(o); self.left.w(o); self.i.w(o); self.u.w(o); self.u_2.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.k = crate::io::Io::r(i); s.left = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.u_2 = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:190-195) the beat 'indoors': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyIndoorsFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyIndoorsFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:198-202) the beat 'wall': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyWallFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyWallFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:203-207) the beat 'dark': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyDarkFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyDarkFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:208-212) the beat 'nv': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyNvFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyNvFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:224-243) the beat 'torches': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyTorchesFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// i: a yield separates its declaration from a use
    pub i: f64,
    /// u: a yield separates its declaration from a use
    pub u_2: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u_3: Option<crate::director::UnitRef>,
}

impl crate::io::Io for TutSkyTorchesFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.men.w(o); self.u.w(o); self.i.w(o); self.u_2.w(o); self.u_3.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.i = crate::io::Io::r(i); s.u_2 = crate::io::Io::r(i); s.u_3 = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:244-265) the beat 'inside': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::script_tut_sky::TutSkyInsideMen>,
    /// left: a yield separates its declaration from a use
    pub left: Vec<crate::script_tut_sky::TutSkyInsideLeft>,
    /// m: a yield separates its declaration from a use
    pub m: Option<crate::script_tut_sky::TutSkyInsideM>,
    /// pending: a yield separates its declaration from a use
    pub pending: Option<JsSet<crate::ObjKey>>,
    /// t: a yield separates its declaration from a use
    pub t: f64,
    /// m: a yield separates its declaration from a use
    pub m_2: Option<crate::script_tut_sky::TutSkyInsideM>,
    /// m: a yield separates its declaration from a use
    pub m_3: Option<crate::script_tut_sky::TutSkyInsideM>,
}

impl crate::io::Io for TutSkyInsideFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.men.w(o); self.left.w(o); self.m.w(o); self.pending.w(o); self.t.w(o); self.m_2.w(o); self.m_3.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.left = crate::io::Io::r(i); s.m = crate::io::Io::r(i); s.pending = crate::io::Io::r(i); s.t = crate::io::Io::r(i); s.m_2 = crate::io::Io::r(i); s.m_3 = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:266-270) the beat 'out': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyOutFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutSkyOutFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:69-85) the helper generator rake: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyRakeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// zones: a parameter
    pub zones: Vec<crate::script_tut_sky::TutSkyZone>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_sky::TutSkyRakeHelperO>,
    /// total: a yield separates its declaration from a use
    pub total: f64,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// r: a yield separates its declaration from a use
    pub r: f64,
    /// Z: a yield separates its declaration from a use
    pub z: Option<crate::script_tut_sky::TutSkyZone>,
    /// z: a yield separates its declaration from a use
    pub z_2: Option<crate::script_tut_sky::TutSkyZone>,
    /// can: a yield separates its declaration from a use
    pub can: Vec<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// heavy: a yield separates its declaration from a use
    pub heavy: bool,
    /// left: a yield separates its declaration from a use
    pub left: f64,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
    /// n: a yield separates its declaration from a use
    pub n: f64,
}

impl crate::io::Io for TutSkyRakeHelperFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.zones.w(o); self.stop.w(o); self.o.w(o); self.total.w(o); self.men.w(o); self.r.w(o); self.z.w(o); self.z_2.w(o); self.can.w(o); self.u.w(o); self.heavy.w(o); self.left.w(o); self.at_2.w(o); self.n.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.zones = crate::io::Io::r(i); s.stop = crate::io::Io::r(i); s.o = crate::io::Io::r(i); s.total = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s.z = crate::io::Io::r(i); s.z_2 = crate::io::Io::r(i); s.can = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.heavy = crate::io::Io::r(i); s.left = crate::io::Io::r(i); s.at_2 = crate::io::Io::r(i); s.n = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:87-97) the helper generator harass: where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkyHarassHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// marks: a parameter
    pub marks: Vec<[f64; 3]>,
    /// stop: a closure parameter: which of the script's own functions the caller means. Your private fn matches on it
    pub stop: u16,
    /// o: a parameter
    pub o: Option<crate::script_tut_sky::TutSkyHarassHelperO>,
    /// men: a yield separates its declaration from a use
    pub men: Vec<crate::director::UnitRef>,
    /// at: a yield separates its declaration from a use
    pub at_2: [f64; 3],
    /// north: a yield separates its declaration from a use
    pub north: Option<crate::director::UnitRef>,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// heavy: a yield separates its declaration from a use
    pub heavy: bool,
    /// n: a yield separates its declaration from a use
    pub n: f64,
}

impl crate::io::Io for TutSkyHarassHelperFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.marks.w(o); self.stop.w(o); self.o.w(o); self.men.w(o); self.at_2.w(o); self.north.w(o); self.u.w(o); self.heavy.w(o); self.n.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.marks = crate::io::Io::r(i); s.stop = crate::io::Io::r(i); s.o = crate::io::Io::r(i); s.men = crate::io::Io::r(i); s.at_2 = crate::io::Io::r(i); s.north = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.heavy = crate::io::Io::r(i); s.n = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts:144-149) the helper generator smoke, inside the beat 'smoke': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutSkySmokeHelperFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// again: a parameter
    pub again: bool,
    /// by: a yield separates its declaration from a use
    pub by: Vec<Option<crate::director::UnitRef>>,
}

impl crate::io::Io for TutSkySmokeHelperFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.again.w(o); self.by.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.again = crate::io::Io::r(i); s.by = crate::io::Io::r(i); s } }

/// (levels/tut-sky.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: block, torchMen, squad, inBunk, atBunk, inYard, pick
#[derive(Clone, Debug, Default)]
pub struct TutSkyVars {
    pub rifle: Option<crate::director::LoadoutRef>,
    pub yaw: f64,
    pub bunk_door: Option<crate::director::DoorRef>,
}

impl crate::io::Io for TutSkyVars { fn w(&self, o: &mut crate::io::W) { self.rifle.w(o); self.yaw.w(o); self.bunk_door.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.rifle = crate::io::Io::r(i); s.yaw = crate::io::Io::r(i); s.bunk_door = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyZone {
    pub lo: [f64; 3],
    pub hi: [f64; 3],
    pub w: f64,
    pub by: Option<Vec<String>>,
}

impl crate::io::Io for TutSkyZone { fn w(&self, o: &mut crate::io::W) { self.lo.w(o); self.hi.w(o); self.w.w(o); self.by.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.lo = crate::io::Io::r(i); s.hi = crate::io::Io::r(i); s.w = crate::io::Io::r(i); s.by = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyRakeHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

impl crate::io::Io for TutSkyRakeHelperO { fn w(&self, o: &mut crate::io::W) { self.rounds.w(o); self.gap.w(o); self.spread.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.rounds = crate::io::Io::r(i); s.gap = crate::io::Io::r(i); s.spread = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyHarassHelperO {
    pub rounds: [f64; 2],
    pub gap: [f64; 2],
    pub spread: f64,
}

impl crate::io::Io for TutSkyHarassHelperO { fn w(&self, o: &mut crate::io::W) { self.rounds.w(o); self.gap.w(o); self.spread.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.rounds = crate::io::Io::r(i); s.gap = crate::io::Io::r(i); s.spread = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideMen {
    pub u: Option<crate::director::UnitRef>,
    pub to: [f64; 3],
}

impl crate::io::Io for TutSkyInsideMen { fn w(&self, o: &mut crate::io::W) { self.u.w(o); self.to.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.u = crate::io::Io::r(i); s.to = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideLeft {
    pub u: Option<crate::director::UnitRef>,
    pub to: [f64; 3],
}

impl crate::io::Io for TutSkyInsideLeft { fn w(&self, o: &mut crate::io::W) { self.u.w(o); self.to.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.u = crate::io::Io::r(i); s.to = crate::io::Io::r(i); s } }

/// an inline shape of the script
#[derive(Clone, Debug, Default)]
pub struct TutSkyInsideM {
    pub u: Option<crate::director::UnitRef>,
    pub to: [f64; 3],
}

impl crate::io::Io for TutSkyInsideM { fn w(&self, o: &mut crate::io::W) { self.u.w(o); self.to.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.u = crate::io::Io::r(i); s.to = crate::io::Io::r(i); s } }
