//! The types of `scene_collision`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the share of the wind a square metre of the plan has (wind.ts:237 stormDrift): crate::wind::wind_share_under(g, x, z, roofs)
#[derive(Clone, Debug)]
pub enum GridWindShare {
    Under { roofs: Option<Vec<Vec<f64>>> },
}
/// is this point of a pane open (glassbreak.ts:213): glass_break_open_at(g, bi, p), or the burst's view `also` where one is on
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum PaneRule {
    GlassBreak,
}
/// what the module's copy judges panes by (glassbreak.ts:213, () => this): g.glassBreak
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum PaneRows {
    GlassBreak,
}
/// is this pane open along a walker's whole height (glassbreak.ts:215): glass_break_walkable(g, bi)
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum GlassWalkable {
    GlassBreak,
}
/// the flags a line of fire passes (winbars.ts:79): BoxFlag.Bar if crate::winbars::bars_live(g), else 0
#[derive(Clone, Copy, Debug, ToBytes)]
pub enum FirePass {
    Bars,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LampRows {
    pub leave: f64,
    pub steady: bool,
}

#[derive(Clone, Debug)]
pub struct GridWind {
    pub x: f64,
    pub z: f64,
    pub low: f64,
    pub gain: f64,
    pub full_at: f64,
    pub share: crate::scene_collision::GridWindShare,
}
impl Default for GridWind {
    fn default() -> Self {
        GridWind {
            x: 0.0,
            z: 0.0,
            low: 0.0,
            gain: 0.0,
            full_at: 0.0,
            share: crate::scene_collision::GridWindShare::Under { roofs: None },
        }
    }
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct StaticCollision {
    pub cells: Vec<Vec<f64>>,
    #[is(40.0)] pub w: f64,
    #[is(28.0)] pub h: f64,
    #[is(1.0)] pub cs: f64,
    pub nav: crate::scene_collision::NavGrid,
    pub sound: crate::scene_collision::SoundPaths,
    pub boxes: Vec<BoxId>,
    /// (a copy made again from the boxes and lamps: in no bytes)
    #[bytes(skip)]
    pub mirror: crate::scene_box_mirror::BoxMirror,
    pub windowed: bool,
    pub tilted_at: Vec<f64>,
    pub pane_rule: Option<crate::scene_collision::PaneRule>,
    pub pane_rows: Option<crate::scene_collision::PaneRows>,
    pub glass_walkable: Option<crate::scene_collision::GlassWalkable>,
    pub dynamic_boxes: Vec<BoxId>,
    /// (a copy made again from the boxes and lamps: in no bytes)
    #[bytes(skip)]
    #[is(crate::scene_collision::StaticCollisionLampQ { on: false, p: None, sim: None, stands: 0.0, parts: 0.0, rows: None })] pub lamp_q: crate::scene_collision::StaticCollisionLampQ,
    pub fire_pass: Option<crate::scene_collision::FirePass>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NavGrid {
    #[is(0.5)] pub cell: f64,
    pub w: f64,
    pub h: f64,
    pub blocked: Vec<u8>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SoundPaths {
    pub cache: JsMap<String, crate::scene_collision::SoundPathsCacheValue>,
    pub now: f64,
    pub asking: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct SoundPathsCacheValue {
    pub t: f64,
    pub r: crate::scene_collision::Propagation,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Propagation {
    pub direct: bool,
    pub routed: bool,
    pub path_len: f64,
    pub via: V3,
    pub apparent: V3,
    pub muffle: f64,
    pub carry: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StaticCollisionLampQ {
    pub on: bool,
    pub p: Option<V3>,
    pub sim: Option<crate::Opaque>,
    pub stands: f64,
    pub parts: f64,
    pub rows: Option<crate::scene_collision::LampRows>,
}

#[derive(Clone, Debug, Default)]
pub struct SceneCollisionModANYTIME {
    pub on: bool,
    pub proofs: f64,
    pub windows: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneCollisionModSENSED {
    pub sight: f64,
    pub light: f64,
    pub sound: f64,
    pub paths: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneCollisionModLAMPS {
    pub parts: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneCollisionModRAYS {
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SceneCollisionModPRICED {
    pub n: f64,
    pub needs: f64,
    pub cells: f64,
    pub waits: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LineBox {
    pub t0: f64,
    pub t1: f64,
    pub depth: f64,
}

#[derive(Clone, Debug, Default)]
pub struct AsideOwn {
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThingsRay {
    pub row: f64,
    pub t: f64,
    pub n: V3,
    pub box_: BoxId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThingsAsk {
    pub row: f64,
    pub t: f64,
    pub n: V3,
    pub box_: BoxId,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SegmentBlockedPanesItem {
    pub bi: f64,
    pub t: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContactE {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContact {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContactAskE {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EyeContactAsk {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum SenseForRow {
    Sound,
    Paths,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SenseFor {
    pub set: f64,
    pub ask: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GridStepK {
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub gone: f64,
    pub in_wind: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Raycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct SceneCollisionMod {
    /// (src/scene/collision.ts:105 ANYTIME)
    #[is(crate::scene_collision::SceneCollisionModANYTIME { on: true, proofs: 0.0, windows: 0.0 })] pub anytime: crate::scene_collision::SceneCollisionModANYTIME,
    /// (src/scene/collision.ts:107 SENSED)
    #[is(crate::scene_collision::SceneCollisionModSENSED { sight: 0.0, light: 0.0, sound: 0.0, paths: 0.0 })] pub sensed: crate::scene_collision::SceneCollisionModSENSED,
    /// (src/scene/collision.ts:118 LAMPS)
    #[is(crate::scene_collision::SceneCollisionModLAMPS { parts: 0.0 })] pub lamps: crate::scene_collision::SceneCollisionModLAMPS,
    /// (src/scene/collision.ts:126 RAYS)
    #[is(crate::scene_collision::SceneCollisionModRAYS { n: 0.0 })] pub rays: crate::scene_collision::SceneCollisionModRAYS,
    /// (src/scene/collision.ts:130 PRICED)
    #[is(crate::scene_collision::SceneCollisionModPRICED { n: 0.0, needs: 0.0, cells: 0.0, waits: false })] pub priced: crate::scene_collision::SceneCollisionModPRICED,
}

/// (src/scene/collision.ts:87 FEET_Y)
pub const FEET_Y: f64 = 0.2;

/// (src/scene/collision.ts:87 BAKE_Y)
pub const BAKE_Y: f64 = 0.25;

/// (src/scene/collision.ts:132 SIGHT_EYES)
pub const SIGHT_EYES: f64 = 7.0;

