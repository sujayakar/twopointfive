//! The frames of levels/tut-quiet.script.ts, written by the skeleton's program from the checker. Never edited by hand: a local a body wants across a yield that is not here is a question for the program.
#![allow(unused_imports, unused_variables, unused_mut)]
use crate::io::Io as _;
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (levels/tut-quiet.script.ts:34-43) the beat 'lane': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQuietLaneFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutQuietLaneFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-quiet.script.ts:46-93) the beat 'night': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQuietNightFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
    /// u: a yield separates its declaration from a use
    pub u: Option<crate::director::UnitRef>,
    /// at: a yield separates its declaration from a use
    pub at_2: Option<[f64; 3]>,
    /// w: a yield separates its declaration from a use
    pub w: Option<crate::director::CondResult>,
    /// r: a yield separates its declaration from a use
    pub r: Option<crate::director::CondResult>,
}

impl crate::io::Io for TutQuietNightFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); self.u.w(o); self.at_2.w(o); self.w.w(o); self.r.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s.u = crate::io::Io::r(i); s.at_2 = crate::io::Io::r(i); s.w = crate::io::Io::r(i); s.r = crate::io::Io::r(i); s } }

/// (levels/tut-quiet.script.ts:96-102) the beat 'store': where it stands, and what lives across a yield
#[derive(Clone, Debug, Default)]
pub struct TutQuietStoreFrame {
    /// at: the resume point: 0 before the first step. A state per yield
    pub at: u16,
}

impl crate::io::Io for TutQuietStoreFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }

/// (levels/tut-quiet.script.ts) what the define pass names and a beat reads: the script's and its programs' locals. Its closures are your private fns: calm
#[derive(Clone, Debug, Default)]
pub struct TutQuietVars {
    pub walk: Option<crate::director::VolumeRef>,
    pub beam: Option<crate::director::VolumeRef>,
    pub past: Option<crate::director::VolumeRef>,
}

impl crate::io::Io for TutQuietVars { fn w(&self, o: &mut crate::io::W) { self.walk.w(o); self.beam.w(o); self.past.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.walk = crate::io::Io::r(i); s.beam = crate::io::Io::r(i); s.past = crate::io::Io::r(i); s } }
