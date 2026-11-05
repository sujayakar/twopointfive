//! The types of `picture`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// exposedAt asked of many spots at one moment: the man's height asked at, and each live idea (a copy) with its weight, heaviest first, and the places its man may have walked to once first asked (picture.ts exposureAsker)
#[derive(Clone, Debug)]
pub enum ExposureAsker {
    Ideas { h: f64, rows: Vec<(crate::picture::Idea, f64, Option<Vec<V3>>)> },
}

/// (TypeScript: a union)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Who {
    Guard(GuardId),
    Player,
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum IdeaHow {
    Heard,
    Seen,
    Told,
    Fix,
}
impl IdeaHow {
    pub const fn as_str(self) -> &'static str {
        match self {
            IdeaHow::Heard => "heard",
            IdeaHow::Seen => "seen",
            IdeaHow::Told => "told",
            IdeaHow::Fix => "fix",
        }
    }
}

/// (TypeScript: Idea, src/game/picture.ts:104)
#[derive(Clone, Copy, Debug)]
pub struct Idea {
    pub who: crate::picture::Who,
    pub at: V3,
    pub t: f64,
    pub since: f64,
    pub how: crate::picture::IdeaHow,
    pub by: Option<GuardId>,
    pub lost: bool,
}
impl Idea {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
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
impl Default for Idea { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Pic, src/game/picture.ts:114)
#[derive(Clone, Debug)]
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
impl Pic {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Pic {
            owner: None,
            ideas: JsMap::new(),
            downs: JsMap::new(),
            heard: JsMap::new(),
            look_t: 0.0,
            step: None,
            step_ask: None,
            step_in: None,
            step_next: None,
            step_for: None,
            step_bad: None,
        }
    }
}
impl Default for Pic { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Step, src/game/picture.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct Step {
    pub key: Option<u32>,
    pub mark: V3,
    pub at: V3,
    pub low: bool,
    pub look_t: f64,
    pub t0: f64,
}
impl Step {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Step {
            key: None,
            mark: [0.0; 3],
            at: [0.0; 3],
            low: false,
            look_t: 0.0,
            t0: 0.0,
        }
    }
}
impl Default for Step { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet Step.key, quiet Step.key)
#[derive(Clone, Copy, Debug)]
pub struct StepKey {
}
impl StepKey {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StepKey {
        }
    }
}
impl Default for StepKey { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: State, src/game/picture.ts:117)
#[derive(Clone, Debug)]
pub struct State {
    pub men: JsMap<GuardId, crate::picture::Pic>,
    pub sam: crate::picture::Pic,
    pub tell_t: f64,
    pub done: JsSet<u32>,
    pub view: Option<crate::picture::PictureView>,
    pub view_t: f64,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            men: JsMap::new(),
            sam: crate::picture::Pic::zeroed(),
            tell_t: 0.0,
            done: JsSet::new(),
            view: None,
            view_t: 0.0,
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PictureView, src/game/picture.ts:416)
#[derive(Clone, Debug)]
pub struct PictureView {
    pub gd: GuardId,
    pub from: V3,
    pub ideas: Vec<crate::picture::PictureViewIdeasItem>,
    pub unsafe_: Vec<[f64; 3]>,
    pub cover: Vec<crate::picture::CoverSpot>,
}
impl PictureView {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PictureView {
            gd: GuardId(0),
            from: [0.0; 3],
            ideas: Vec::new(),
            unsafe_: Vec::new(),
            cover: Vec::new(),
        }
    }
}
impl Default for PictureView { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:418 PictureView.ideas, src/game/picture.ts:418)
#[derive(Clone, Debug)]
pub struct PictureViewIdeasItem {
    pub who: String,
    pub at: V3,
    pub age: f64,
    pub how: crate::picture::IdeaHow,
    pub w: f64,
    pub reach: f64,
    pub lost: bool,
    pub maybe: Vec<V3>,
}
impl PictureViewIdeasItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PictureViewIdeasItem {
            who: String::new(),
            at: [0.0; 3],
            age: 0.0,
            how: crate::picture::IdeaHow::Heard,
            w: 0.0,
            reach: 0.0,
            lost: false,
            maybe: Vec::new(),
        }
    }
}
impl Default for PictureViewIdeasItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CoverSpot, src/game/picture.ts:351)
#[derive(Clone, Copy, Debug)]
pub struct CoverSpot {
    pub at: V3,
    pub stand: f64,
    pub low: f64,
    pub edge: bool,
    pub score: f64,
}
impl CoverSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverSpot {
            at: [0.0; 3],
            stand: 0.0,
            low: 0.0,
            edge: false,
            score: 0.0,
        }
    }
}
impl Default for CoverSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: IdeaSnap, src/game/picture.ts:449)
#[derive(Clone, Copy, Debug)]
pub struct IdeaSnap {
    pub who: f64,
    pub at: V3,
    pub age: f64,
    pub known: f64,
    pub how: crate::picture::IdeaHow,
    pub by: f64,
    pub lost: bool,
}
impl IdeaSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        IdeaSnap {
            who: 0.0,
            at: [0.0; 3],
            age: 0.0,
            known: 0.0,
            how: crate::picture::IdeaHow::Heard,
            by: 0.0,
            lost: false,
        }
    }
}
impl Default for IdeaSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PicSnap, src/game/picture.ts:451)
#[derive(Clone, Debug)]
pub struct PicSnap {
    pub ideas: Vec<crate::picture::IdeaSnap>,
    pub downs: Vec<[f64; 2]>,
    pub heard: Vec<[f64; 2]>,
    pub look: f64,
    pub step: Option<crate::picture::PicSnapStep>,
    pub step_bad: Option<V3>,
    pub step_in: Option<f64>,
    pub step_next: Option<f64>,
}
impl PicSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PicSnap {
            ideas: Vec::new(),
            downs: Vec::new(),
            heard: Vec::new(),
            look: 0.0,
            step: None,
            step_bad: None,
            step_in: None,
            step_next: None,
        }
    }
}
impl Default for PicSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:451 PicSnap.step, src/game/picture.ts:451)
#[derive(Clone, Copy, Debug)]
pub struct PicSnapStep {
    pub mark: V3,
    pub at: V3,
    pub low: bool,
    pub look: f64,
    pub since: Option<f64>,
}
impl PicSnapStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PicSnapStep {
            mark: [0.0; 3],
            at: [0.0; 3],
            low: false,
            look: 0.0,
            since: None,
        }
    }
}
impl Default for PicSnapStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PictureSnap, src/game/picture.ts:452)
#[derive(Clone, Debug)]
pub struct PictureSnap {
    pub men: Vec<Option<crate::picture::PicSnap>>,
    pub sam: crate::picture::PicSnap,
    pub tell: f64,
}
impl PictureSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PictureSnap {
            men: Vec::new(),
            sam: crate::picture::PicSnap::zeroed(),
            tell: 0.0,
        }
    }
}
impl Default for PictureSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:331 knownPlaces ->, src/game/picture.ts:331)
#[derive(Clone, Copy, Debug)]
pub struct KnownPlacesItem {
    pub who: crate::picture::Who,
    pub at: V3,
    pub age: f64,
}
impl KnownPlacesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KnownPlacesItem {
            who: crate::picture::Who::Guard(GuardId(0)),
            at: [0.0; 3],
            age: 0.0,
        }
    }
}
impl Default for KnownPlacesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:346 safestSets(sets), src/game/picture.ts:346)
#[derive(Clone, Copy, Debug)]
pub struct SafestSetsSetsItemItem {
    pub pos: V3,
}
impl SafestSetsSetsItemItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SafestSetsSetsItemItem {
            pos: [0.0; 3],
        }
    }
}
impl Default for SafestSetsSetsItemItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:373 pictureStep ->, src/game/picture.ts:373)
#[derive(Clone, Copy, Debug)]
pub struct PictureStep {
    pub at: V3,
    pub low: bool,
}
impl PictureStep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PictureStep {
            at: [0.0; 3],
            low: false,
        }
    }
}
impl Default for PictureStep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/picture.ts:413 pictureStepOf ->, src/game/picture.ts:413)
#[derive(Clone, Copy, Debug)]
pub struct PictureStepOf {
    pub at: V3,
    pub mark: V3,
    pub low: bool,
}
impl PictureStepOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PictureStepOf {
            at: [0.0; 3],
            mark: [0.0; 3],
            low: false,
        }
    }
}
impl Default for PictureStepOf { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
