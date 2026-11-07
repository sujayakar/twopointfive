//! The types of `scene_collision`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the share of the wind a square metre of the plan has (wind.ts:237 stormDrift): crate::wind::wind_share_under(g, x, z, roofs)
#[derive(Clone, Debug)]
pub enum GridWindShare {
    Under { roofs: Option<Vec<Vec<f64>>> },
}
/// is this point of a pane open (glassbreak.ts:213): glass_break_open_at(g, bi, p), or the burst's view `also` where one is on
#[derive(Clone, Copy, Debug)]
pub enum PaneRule {
    GlassBreak,
}
/// what the module's copy judges panes by (glassbreak.ts:213, () => this): g.glassBreak
#[derive(Clone, Copy, Debug)]
pub enum PaneRows {
    GlassBreak,
}
/// is this pane open along a walker's whole height (glassbreak.ts:215): glass_break_walkable(g, bi)
#[derive(Clone, Copy, Debug)]
pub enum GlassWalkable {
    GlassBreak,
}
/// the flags a line of fire passes (winbars.ts:79): BoxFlag.Bar if crate::winbars::bars_live(g), else 0
#[derive(Clone, Copy, Debug)]
pub enum FirePass {
    Bars,
}

/// (TypeScript: BlobRow, src/scene/collision.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct BlobRow {
    pub x: f64,
    pub y0: f64,
    pub z: f64,
    pub r2: f64,
    pub age: f64,
    pub mass: f64,
}
impl BlobRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BlobRow {
            x: 0.0,
            y0: 0.0,
            z: 0.0,
            r2: 0.0,
            age: 0.0,
            mass: 0.0,
        }
    }
}
impl Default for BlobRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SmokeNumbers, src/scene/collision.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct SmokeNumbers {
    pub sigma: f64,
    pub spread: f64,
    pub widest: f64,
    pub rise: f64,
    pub rise_rate: f64,
}
impl SmokeNumbers {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SmokeNumbers {
            sigma: 0.0,
            spread: 0.0,
            widest: 0.0,
            rise: 0.0,
            rise_rate: 0.0,
        }
    }
}
impl Default for SmokeNumbers { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: LampRows, src/scene/collision.ts:122)
#[derive(Clone, Copy, Debug)]
pub struct LampRows {
    pub st: crate::Opaque,
    pub leave: f64,
    pub steady: bool,
}
impl LampRows {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LampRows {
            st: crate::Opaque,
            leave: 0.0,
            steady: false,
        }
    }
}
impl Default for LampRows { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: GridWind, src/scene/collision.ts:136)
#[derive(Clone, Debug)]
pub struct GridWind {
    pub x: f64,
    pub z: f64,
    pub low: f64,
    pub gain: f64,
    pub full_at: f64,
    pub share: crate::scene_collision::GridWindShare,
}
impl GridWind {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
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
impl Default for GridWind { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StaticCollision, src/scene/collision.ts:141)
#[derive(Clone, Debug)]
pub struct StaticCollision {
    pub cells: Vec<Vec<f64>>,
    pub w: f64,
    pub h: f64,
    pub cs: f64,
    pub nav: crate::scene_collision::NavGrid,
    pub sound: crate::scene_collision::SoundPaths,
    pub boxes: Vec<BoxId>,
    pub mirror: crate::scene_box_mirror::BoxMirror,
    pub windowed: bool,
    pub tilted_at: Vec<f64>,
    pub pane_rule: Option<crate::scene_collision::PaneRule>,
    pub pane_rows: Option<crate::scene_collision::PaneRows>,
    pub glass_walkable: Option<crate::scene_collision::GlassWalkable>,
    pub dynamic_boxes: Vec<BoxId>,
    pub lamp_q: crate::scene_collision::StaticCollisionLampQ,
    pub fire_pass: Option<crate::scene_collision::FirePass>,
}
impl StaticCollision {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        StaticCollision {
            cells: Vec::new(),
            w: 40.0,
            h: 28.0,
            cs: 1.0,
            nav: crate::scene_collision::NavGrid::blank(),
            sound: crate::scene_collision::SoundPaths::blank(),
            boxes: Vec::new(),
            mirror: crate::scene_box_mirror::BoxMirror::blank(),
            windowed: false,
            tilted_at: Vec::new(),
            pane_rule: None,
            pane_rows: None,
            glass_walkable: None,
            dynamic_boxes: Vec::new(),
            lamp_q: crate::scene_collision::StaticCollisionLampQ { on: false, p: None, sim: None, stands: 0.0, k: None, nb: 0.0, parts: 0.0, rows: None },
            fire_pass: None,
        }
    }
}
impl Default for StaticCollision { fn default() -> Self { Self::blank() } }

/// (TypeScript: NavGrid, src/scene/collision.ts:972)
#[derive(Clone, Debug)]
pub struct NavGrid {
    pub cell: f64,
    pub w: f64,
    pub h: f64,
    pub blocked: Vec<u8>,
}
impl NavGrid {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        NavGrid {
            cell: 0.5,
            w: 0.0,
            h: 0.0,
            blocked: Vec::new(),
        }
    }
}
impl Default for NavGrid { fn default() -> Self { Self::blank() } }

/// (TypeScript: SoundPaths, src/scene/collision.ts:925)
#[derive(Clone, Debug)]
pub struct SoundPaths {
    pub cache: JsMap<String, crate::scene_collision::SoundPathsCacheValue>,
    pub now: f64,
    pub asking: bool,
}
impl SoundPaths {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        SoundPaths {
            cache: JsMap::new(),
            now: 0.0,
            asking: false,
        }
    }
}
impl Default for SoundPaths { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/scene/collision.ts:926 SoundPaths.cache, src/scene/collision.ts:926)
#[derive(Clone, Copy, Debug)]
pub struct SoundPathsCacheValue {
    pub t: f64,
    pub r: crate::scene_collision::Propagation,
}
impl SoundPathsCacheValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SoundPathsCacheValue {
            t: 0.0,
            r: crate::scene_collision::Propagation::zeroed(),
        }
    }
}
impl Default for SoundPathsCacheValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Propagation, src/scene/collision.ts:917)
#[derive(Clone, Copy, Debug)]
pub struct Propagation {
    pub direct: bool,
    pub routed: bool,
    pub path_len: f64,
    pub via: V3,
    pub apparent: V3,
    pub muffle: f64,
    pub carry: f64,
}
impl Propagation {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Propagation {
            direct: false,
            routed: false,
            path_len: 0.0,
            via: [0.0; 3],
            apparent: [0.0; 3],
            muffle: 0.0,
            carry: 0.0,
        }
    }
}
impl Default for Propagation { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:633 StaticCollision.lampQ, src/scene/collision.ts:633)
#[derive(Clone, Copy, Debug)]
pub struct StaticCollisionLampQ {
    pub on: bool,
    pub p: Option<V3>,
    pub sim: Option<crate::Opaque>,
    pub stands: f64,
    pub k: Option<crate::scene_collision::SmokeNumbers>,
    pub nb: f64,
    pub parts: f64,
    pub rows: Option<crate::scene_collision::LampRows>,
}
impl StaticCollisionLampQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StaticCollisionLampQ {
            on: false,
            p: None,
            sim: None,
            stands: 0.0,
            k: None,
            nb: 0.0,
            parts: 0.0,
            rows: None,
        }
    }
}
impl Default for StaticCollisionLampQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:90 shotPanes, src/scene/collision.ts:90)
#[derive(Clone, Copy, Debug)]
pub struct ShotPanesItem {
    pub bi: f64,
    pub t: f64,
}
impl ShotPanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShotPanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for ShotPanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:101 BOTH, src/scene/collision.ts:101)
#[derive(Clone, Debug)]
pub struct SceneCollisionModBOTH {
    pub on: bool,
    pub asked: f64,
    pub differ: f64,
    pub stale: f64,
    pub first: String,
    pub things: f64,
}
impl SceneCollisionModBOTH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCollisionModBOTH {
            on: false,
            asked: 0.0,
            differ: 0.0,
            stale: 0.0,
            first: String::new(),
            things: 0.0,
        }
    }
}
impl Default for SceneCollisionModBOTH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:105 ANYTIME, src/scene/collision.ts:105)
#[derive(Clone, Debug)]
pub struct SceneCollisionModANYTIME {
    pub on: bool,
    pub proofs: f64,
    pub windows: f64,
    pub who: Option<JsMap<String, f64>>,
}
impl SceneCollisionModANYTIME {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCollisionModANYTIME {
            on: false,
            proofs: 0.0,
            windows: 0.0,
            who: None,
        }
    }
}
impl Default for SceneCollisionModANYTIME { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:107 SENSED, src/scene/collision.ts:107)
#[derive(Clone, Copy, Debug)]
pub struct SceneCollisionModSENSED {
    pub sight: f64,
    pub light: f64,
    pub smoke: f64,
    pub sound: f64,
    pub paths: f64,
}
impl SceneCollisionModSENSED {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCollisionModSENSED {
            sight: 0.0,
            light: 0.0,
            smoke: 0.0,
            sound: 0.0,
            paths: 0.0,
        }
    }
}
impl Default for SceneCollisionModSENSED { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:113 SENSE_WATCH, src/scene/collision.ts:113)
#[derive(Clone, Copy, Debug)]
pub struct SENSEWATCH {
    pub see: Option<crate::FnField>,
}
impl SENSEWATCH {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SENSEWATCH {
            see: None,
        }
    }
}
impl Default for SENSEWATCH { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:118 LAMPS, src/scene/collision.ts:118)
#[derive(Clone, Copy, Debug)]
pub struct SceneCollisionModLAMPS {
    pub parts: f64,
}
impl SceneCollisionModLAMPS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCollisionModLAMPS {
            parts: 0.0,
        }
    }
}
impl Default for SceneCollisionModLAMPS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:126 RAYS, src/scene/collision.ts:126)
#[derive(Clone, Copy, Debug)]
pub struct SceneCollisionModRAYS {
    pub n: f64,
    pub host: f64,
}
impl SceneCollisionModRAYS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCollisionModRAYS {
            n: 0.0,
            host: 0.0,
        }
    }
}
impl Default for SceneCollisionModRAYS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:130 PRICED, src/scene/collision.ts:130)
#[derive(Clone, Copy, Debug)]
pub struct SceneCollisionModPRICED {
    pub n: f64,
    pub needs: f64,
    pub cells: f64,
    pub waits: bool,
}
impl SceneCollisionModPRICED {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SceneCollisionModPRICED {
            n: 0.0,
            needs: 0.0,
            cells: 0.0,
            waits: false,
        }
    }
}
impl Default for SceneCollisionModPRICED { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:133 bothPanes, src/scene/collision.ts:133)
#[derive(Clone, Copy, Debug)]
pub struct BothPanesItem {
    pub bi: f64,
    pub t: f64,
}
impl BothPanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BothPanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for BothPanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:15 lineBox ->, src/scene/collision.ts:15)
#[derive(Clone, Copy, Debug)]
pub struct LineBox {
    pub t0: f64,
    pub t1: f64,
    pub depth: f64,
}
impl LineBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LineBox {
            t0: 0.0,
            t1: 0.0,
            depth: 0.0,
        }
    }
}
impl Default for LineBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:267 aside(own), src/scene/collision.ts:267)
#[derive(Clone, Debug)]
pub struct AsideOwn {
    pub q: Option<crate::scene_collision::AsideOwnQ>,
}
impl AsideOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AsideOwn {
            q: None,
        }
    }
}
impl Default for AsideOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:267 AsideOwn.q, src/scene/collision.ts:267 AsideOwn.q)
#[derive(Clone, Debug)]
pub struct AsideOwnQ {
    /// INVENTED by a review: see the proposals
    pub stamp: Vec<i32>,
    /// INVENTED by a review: see the proposals
    pub id: f64,
    /// INVENTED by a review: see the proposals
    pub cell: Vec<i32>,
    /// INVENTED by a review: see the proposals
    pub seg: Vec<i32>,
    /// INVENTED by a review: see the proposals
    pub yaw: Vec<f64>,
    /// INVENTED by a review: see the proposals
    pub cos: Vec<f64>,
    /// INVENTED by a review: see the proposals
    pub sin: Vec<f64>,
}
impl AsideOwnQ {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AsideOwnQ {
            stamp: Vec::new(),
            id: 0.0,
            cell: Vec::new(),
            seg: Vec::new(),
            yaw: Vec::new(),
            cos: Vec::new(),
            sin: Vec::new(),
        }
    }
}
impl Default for AsideOwnQ { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet thingsRay ->, src/scene/collision.ts:470)
#[derive(Clone, Copy, Debug)]
pub struct ThingsRay {
    pub row: f64,
    pub t: f64,
    pub n: V3,
    pub box_: BoxId,
}
impl ThingsRay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThingsRay {
            row: 0.0,
            t: 0.0,
            n: [0.0; 3],
            box_: BoxId(0),
        }
    }
}
impl Default for ThingsRay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet thingsAsk ->, src/scene/collision.ts:474)
#[derive(Clone, Copy, Debug)]
pub struct ThingsAsk {
    pub row: f64,
    pub t: f64,
    pub n: V3,
    pub box_: BoxId,
}
impl ThingsAsk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThingsAsk {
            row: 0.0,
            t: 0.0,
            n: [0.0; 3],
            box_: BoxId(0),
        }
    }
}
impl Default for ThingsAsk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:484 thingsBoth(t), src/scene/collision.ts:484)
#[derive(Clone, Copy, Debug)]
pub struct ThingsBothT {
    pub t: f64,
    pub n: V3,
    pub prop: crate::Opaque,
    pub box_: BoxId,
}
impl ThingsBothT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThingsBothT {
            t: 0.0,
            n: [0.0; 3],
            prop: crate::Opaque,
            box_: BoxId(0),
        }
    }
}
impl Default for ThingsBothT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:526 segmentBlocked(panes), src/scene/collision.ts:526)
#[derive(Clone, Copy, Debug)]
pub struct SegmentBlockedPanesItem {
    pub bi: f64,
    pub t: f64,
}
impl SegmentBlockedPanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SegmentBlockedPanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for SegmentBlockedPanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:530 segmentBlockedAsk(panes), src/scene/collision.ts:530)
#[derive(Clone, Copy, Debug)]
pub struct SegmentBlockedAskPanesItem {
    pub bi: f64,
    pub t: f64,
}
impl SegmentBlockedAskPanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SegmentBlockedAskPanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for SegmentBlockedAskPanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:610 eyeContact(E), src/scene/collision.ts:610)
#[derive(Clone, Copy, Debug)]
pub struct EyeContactE {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}
impl EyeContactE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EyeContactE {
            pos: [0.0; 3],
            look: [0.0; 3],
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
            v_half_deg: None,
        }
    }
}
impl Default for EyeContactE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:610 eyeContact ->, src/scene/collision.ts:610)
#[derive(Clone, Copy, Debug)]
pub struct EyeContact {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl EyeContact {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EyeContact {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for EyeContact { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:615 eyeContactAsk(E), src/scene/collision.ts:615)
#[derive(Clone, Copy, Debug)]
pub struct EyeContactAskE {
    pub pos: V3,
    pub look: V3,
    pub range: f64,
    pub half_deg: f64,
    pub near_m: f64,
    pub near_deg: f64,
    pub v_half_deg: Option<f64>,
}
impl EyeContactAskE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EyeContactAskE {
            pos: [0.0; 3],
            look: [0.0; 3],
            range: 0.0,
            half_deg: 0.0,
            near_m: 0.0,
            near_deg: 0.0,
            v_half_deg: None,
        }
    }
}
impl Default for EyeContactAskE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:615 eyeContactAsk ->, src/scene/collision.ts:615)
#[derive(Clone, Copy, Debug)]
pub struct EyeContactAsk {
    pub visible: bool,
    pub dist: f64,
    pub in_cone: bool,
}
impl EyeContactAsk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EyeContactAsk {
            visible: false,
            dist: 0.0,
            in_cone: false,
        }
    }
}
impl Default for EyeContactAsk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:634 lampsBegin(shade), src/scene/collision.ts:634)
#[derive(Clone, Debug)]
pub struct LampsBeginShade {
    pub grid: crate::scene_collision::LampsBeginShadeGrid,
    pub blobs: Vec<crate::scene_collision::BlobRow>,
}
impl LampsBeginShade {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LampsBeginShade {
            grid: crate::scene_collision::LampsBeginShadeGrid::zeroed(),
            blobs: Vec::new(),
        }
    }
}
impl Default for LampsBeginShade { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:634 LampsBeginShade.grid, src/scene/collision.ts:634)
#[derive(Clone, Copy, Debug)]
pub struct LampsBeginShadeGrid {
    pub sim: crate::Opaque,
    pub stands: bool,
}
impl LampsBeginShadeGrid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LampsBeginShadeGrid {
            sim: crate::Opaque,
            stands: false,
        }
    }
}
impl Default for LampsBeginShadeGrid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SenseForRow {
    Sound,
    Paths,
}
impl SenseForRow {
    pub const fn as_str(self) -> &'static str {
        match self {
            SenseForRow::Sound => "sound",
            SenseForRow::Paths => "paths",
        }
    }
}

/// (TypeScript: inline at src/scene/collision.ts:682 senseFor ->, src/scene/collision.ts:682)
#[derive(Clone, Copy, Debug)]
pub struct SenseFor {
    pub set: f64,
    pub ask: f64,
}
impl SenseFor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SenseFor {
            set: 0.0,
            ask: 0.0,
        }
    }
}
impl Default for SenseFor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:694 gridStep(K), src/scene/collision.ts:694)
#[derive(Clone, Copy, Debug)]
pub struct GridStepK {
    pub decay: f64,
    pub spread: f64,
    pub rise: f64,
    pub rise_rate: f64,
    pub gone: f64,
    pub in_wind: f64,
}
impl GridStepK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GridStepK {
            decay: 0.0,
            spread: 0.0,
            rise: 0.0,
            rise_rate: 0.0,
            gone: 0.0,
            in_wind: 0.0,
        }
    }
}
impl Default for GridStepK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:695 gridStep(glow), src/scene/collision.ts:695)
#[derive(Clone, Copy, Debug)]
pub struct GridStepGlow {
    pub k: crate::scene_collision::GridStepGlowK,
}
impl GridStepGlow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GridStepGlow {
            k: crate::scene_collision::GridStepGlowK::zeroed(),
        }
    }
}
impl Default for GridStepGlow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:695 GridStepGlow.K, src/scene/collision.ts:695)
#[derive(Clone, Copy, Debug)]
pub struct GridStepGlowK {
    pub albedo: f64,
    pub share: f64,
    pub passes: f64,
    pub sweep: f64,
}
impl GridStepGlowK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GridStepGlowK {
            albedo: 0.0,
            share: 0.0,
            passes: 0.0,
            sweep: 0.0,
        }
    }
}
impl Default for GridStepGlowK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:721 segmentBlockedHere(panes), src/scene/collision.ts:721)
#[derive(Clone, Copy, Debug)]
pub struct SegmentBlockedHerePanesItem {
    pub bi: f64,
    pub t: f64,
}
impl SegmentBlockedHerePanesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SegmentBlockedHerePanesItem {
            bi: 0.0,
            t: 0.0,
        }
    }
}
impl Default for SegmentBlockedHerePanesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:861 raycast ->, src/scene/collision.ts:861)
#[derive(Clone, Copy, Debug)]
pub struct Raycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl Raycast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Raycast {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for Raycast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:866 raycastAsk ->, src/scene/collision.ts:866)
#[derive(Clone, Copy, Debug)]
pub struct RaycastAsk {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl RaycastAsk {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RaycastAsk {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RaycastAsk { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/scene/collision.ts:884 raycastHere ->, src/scene/collision.ts:884)
#[derive(Clone, Copy, Debug)]
pub struct RaycastHere {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}
impl RaycastHere {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RaycastHere {
            t: 0.0,
            index: 0.0,
            n: [0.0; 3],
        }
    }
}
impl Default for RaycastHere { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SceneCollisionMod {
    /// (src/scene/collision.ts:101 BOTH)
    pub both: crate::scene_collision::SceneCollisionModBOTH,
    /// (src/scene/collision.ts:105 ANYTIME)
    pub anytime: crate::scene_collision::SceneCollisionModANYTIME,
    /// (src/scene/collision.ts:107 SENSED)
    pub sensed: crate::scene_collision::SceneCollisionModSENSED,
    /// (src/scene/collision.ts:118 LAMPS)
    pub lamps: crate::scene_collision::SceneCollisionModLAMPS,
    /// (src/scene/collision.ts:126 RAYS)
    pub rays: crate::scene_collision::SceneCollisionModRAYS,
    /// (src/scene/collision.ts:130 PRICED)
    pub priced: crate::scene_collision::SceneCollisionModPRICED,
}
impl SceneCollisionMod {
    pub fn blank() -> Self {
        SceneCollisionMod {
            both: crate::scene_collision::SceneCollisionModBOTH { on: false, asked: 0.0, differ: 0.0, stale: 0.0, first: String::from(""), things: 0.0 },
            anytime: crate::scene_collision::SceneCollisionModANYTIME { on: true, proofs: 0.0, windows: 0.0, who: None },
            sensed: crate::scene_collision::SceneCollisionModSENSED { sight: 0.0, light: 0.0, smoke: 0.0, sound: 0.0, paths: 0.0 },
            lamps: crate::scene_collision::SceneCollisionModLAMPS { parts: 0.0 },
            rays: crate::scene_collision::SceneCollisionModRAYS { n: 0.0, host: 0.0 },
            priced: crate::scene_collision::SceneCollisionModPRICED { n: 0.0, needs: 0.0, cells: 0.0, waits: false },
        }
    }
}

/// (src/scene/collision.ts:48 SEG_PAD)
pub const SEG_PAD: f64 = 0.05;

/// (src/scene/collision.ts:87 FEET_Y)
pub const FEET_Y: f64 = 0.2;

/// (src/scene/collision.ts:87 BAKE_Y)
pub const BAKE_Y: f64 = 0.25;

/// (src/scene/collision.ts:90 shotPanes: a table nothing writes)
pub fn shot_panes() -> Vec<crate::scene_collision::ShotPanesItem> { Vec::new() }

/// (src/scene/collision.ts:113 SENSE_WATCH: a table nothing writes)
pub fn sense_watch() -> crate::scene_collision::SENSEWATCH { crate::scene_collision::SENSEWATCH { see: None } }

/// (src/scene/collision.ts:132 SIGHT_EYES)
pub const SIGHT_EYES: f64 = 7.0;

/// (src/scene/collision.ts:133 bothPanes: a table nothing writes)
pub fn both_panes() -> Vec<crate::scene_collision::BothPanesItem> { Vec::new() }
