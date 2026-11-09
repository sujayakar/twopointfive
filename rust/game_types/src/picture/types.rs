//! The types of `picture`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// exposedAt asked of many spots at one moment: the man's height asked at, and each live idea (a copy) with its weight, heaviest first, and the places its man may have walked to once first asked (picture.ts exposureAsker)
#[derive(Clone, Debug)]
pub enum ExposureAsker {
    Ideas { h: f64, rows: Vec<(crate::picture::Idea, f64, Option<Vec<V3>>)> },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
pub enum Who {
    Guard(GuardId),
    Player,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum IdeaHow {
    #[default] Heard,
    Seen,
    Told,
    Fix,
}

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Idea {
    pub who: crate::picture::Who,
    pub at: V3,
    pub t: f64,
    pub since: f64,
    pub how: crate::picture::IdeaHow,
    pub by: Option<GuardId>,
    pub lost: bool,
}
impl Default for Idea {
    fn default() -> Self {
        Idea {
            who: crate::picture::Who::Guard(GuardId(0)),
            at: [0.0; 3],
            t: 0.0,
            since: 0.0,
            how: crate::picture::IdeaHow::Heard,
            by: None,
            lost: false,
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Pic {
    pub owner: Option<GuardId>,
    pub ideas: JsMap<crate::picture::Who, crate::picture::Idea>,
    pub downs: JsMap<crate::picture::Who, f64>,
    pub heard: JsMap<crate::picture::Who, f64>,
    pub look_t: f64,
    pub step: Option<crate::picture::Step>,
    pub step_ask: Option<f64>,
    pub step_in: Option<f64>,
    pub step_next: Option<f64>,
    pub step_for: Option<V3>,
    pub step_bad: Option<V3>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Step {
    pub key: Option<u32>,
    pub mark: V3,
    pub at: V3,
    pub low: bool,
    pub look_t: f64,
    pub t0: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub men: JsMap<GuardId, crate::picture::Pic>,
    pub sam: crate::picture::Pic,
    pub tell_t: f64,
    pub done: JsSet<u32>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CoverSpot {
    pub at: V3,
    pub stand: f64,
    pub low: f64,
    pub score: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct KnownPlacesItem {
    pub who: crate::picture::Who,
    pub at: V3,
}
impl Default for KnownPlacesItem {
    fn default() -> Self {
        KnownPlacesItem {
            who: crate::picture::Who::Guard(GuardId(0)),
            at: [0.0; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PictureStep {
    pub at: V3,
    pub low: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct PictureMod {
    /// (src/game/picture.ts:118 STATES)
    pub states: Option<crate::picture::State>,
    /// (src/game/picture.ts:291 VIEWING)
    pub viewing: bool,
}
impl PictureMod {
    pub fn blank() -> Self {
        PictureMod {
            states: None,
            viewing: false,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct PictureStats {
    pub looks: i32,
    pub seen: i32,
    pub heard: i32,
    pub told: i32,
    pub fixes: i32,
    pub lost: i32,
    pub downs: i32,
    pub forgot: i32,
    pub asks: i32,
    pub lines: i32,
    pub moved: i32,
    pub shot_placed: i32,
    pub shot_off: i32,
    pub shot_kept: i32,
    pub ran: i32,
    pub ran_off: i32,
    pub stepped: i32,
    pub filed: i32,
    pub filed_hinge: i32,
}
