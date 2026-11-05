//! The types of `break_table`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Breaks, src/game/breakTable.ts:335)
#[derive(Clone, Debug)]
pub struct Breaks {
    pub counter: f64,
    pub queue: Vec<DebrisChunkId>,
    pub pcounter: f64,
    pub look: Option<crate::break_table::BreaksLook>,
    pub kink_t: f64,
}
impl Breaks {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Breaks {
            counter: 1.0,
            queue: Vec::new(),
            pcounter: 1.0,
            look: None,
            kink_t: -1000000000.0,
        }
    }
}
impl Default for Breaks { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/breakTable.ts:529 Breaks.look, src/game/breakTable.ts:529)
#[derive(Clone, Copy, Debug)]
pub struct BreaksLook {
    pub on: bool,
}
impl BreaksLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksLook {
            on: false,
        }
    }
}
impl Default for BreaksLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Mat {
    Carpet,
    Tile,
    Metal,
    Glass,
    Wood,
    Plastic,
    Cardboard,
    Fabric,
    Screen,
    Plasterboard,
    Paper,
    Plant,
    Masonry,
    Blockwork,
    GlassTough,
    GlassLam,
}
impl Mat {
    pub const fn as_str(self) -> &'static str {
        match self {
            Mat::Carpet => "carpet",
            Mat::Tile => "tile",
            Mat::Metal => "metal",
            Mat::Glass => "glass",
            Mat::Wood => "wood",
            Mat::Plastic => "plastic",
            Mat::Cardboard => "cardboard",
            Mat::Fabric => "fabric",
            Mat::Screen => "screen",
            Mat::Plasterboard => "plasterboard",
            Mat::Paper => "paper",
            Mat::Plant => "plant",
            Mat::Masonry => "masonry",
            Mat::Blockwork => "blockwork",
            Mat::GlassTough => "glassTough",
            Mat::GlassLam => "glassLam",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Shape {
    Panel,
    Slab,
    Snapped,
    Strip,
    Wedge,
    Lump,
}
impl Shape {
    pub const fn as_str(self) -> &'static str {
        match self {
            Shape::Panel => "panel",
            Shape::Slab => "slab",
            Shape::Snapped => "snapped",
            Shape::Strip => "strip",
            Shape::Wedge => "wedge",
            Shape::Lump => "lump",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Split {
    Along,
    Across,
    Diag,
}
impl Split {
    pub const fn as_str(self) -> &'static str {
        match self {
            Split::Along => "along",
            Split::Across => "across",
            Split::Diag => "diag",
        }
    }
}

/// (TypeScript: BreakRow, src/game/breakTable.ts:37)
#[derive(Clone, Debug)]
pub struct BreakRow {
    pub core: Option<V3>,
    pub pale: Option<f64>,
    pub cell: Vec<crate::break_table::Shape>,
    pub thick: f64,
    pub span: [f64; 2],
    pub slim: [f64; 2],
    pub tears: [f64; 2],
    pub ragged: Option<crate::break_table::BreakRowRagged>,
    pub again: Option<crate::break_table::BreakRowAgain>,
    pub small: crate::break_table::BreakRowSmall,
    pub dust: f64,
    pub sparks: f64,
    pub puff: f64,
    pub rest: f64,
    pub level: f64,
    pub mark: f64,
}
impl BreakRow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRow {
            core: None,
            pale: None,
            cell: Vec::new(),
            thick: 0.0,
            span: [0.0; 2],
            slim: [0.0; 2],
            tears: [0.0; 2],
            ragged: None,
            again: None,
            small: crate::break_table::BreakRowSmall::zeroed(),
            dust: 0.0,
            sparks: 0.0,
            puff: 0.0,
            rest: 0.0,
            level: 0.0,
            mark: 0.0,
        }
    }
}
impl Default for BreakRow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:44 BreakRow.ragged, src/game/breakTable.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct BreakRowRagged {
    pub deg: [f64; 2],
    pub bite: f64,
    pub grain: Option<bool>,
}
impl BreakRowRagged {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowRagged {
            deg: [0.0; 2],
            bite: 0.0,
            grain: None,
        }
    }
}
impl Default for BreakRowRagged { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:47 BreakRow.again, src/game/breakTable.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct BreakRowAgain {
    pub land: f64,
    pub kick: f64,
    pub how: crate::break_table::Split,
    pub chips: f64,
    pub chance: Option<f64>,
}
impl BreakRowAgain {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowAgain {
            land: 0.0,
            kick: 0.0,
            how: crate::break_table::Split::Along,
            chips: 0.0,
            chance: None,
        }
    }
}
impl Default for BreakRowAgain { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:49 BreakRow.small, src/game/breakTable.ts:49)
#[derive(Clone, Copy, Debug)]
pub struct BreakRowSmall {
    pub n: f64,
    pub size: [f64; 2],
    pub aspect: [f64; 2],
}
impl BreakRowSmall {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakRowSmall {
            n: 0.0,
            size: [0.0; 2],
            aspect: [0.0; 2],
        }
    }
}
impl Default for BreakRowSmall { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BreakMeta, src/game/breakTable.ts:150)
#[derive(Clone, Copy, Debug)]
pub struct BreakMeta {
    pub mat: crate::break_table::Mat,
    pub gen_: f64,
    pub paint: V3,
    pub fixed: Option<bool>,
}
impl BreakMeta {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreakMeta {
            mat: crate::break_table::Mat::Carpet,
            gen_: 0.0,
            paint: [0.0; 3],
            fixed: None,
        }
    }
}
impl Default for BreakMeta { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Shaped, src/game/breakTable.ts:228)
#[derive(Clone, Debug)]
pub struct Shaped {
    pub off: V3,
    pub h: V3,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
}
impl Shaped {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Shaped {
            off: [0.0; 3],
            h: [0.0; 3],
            cut: None,
        }
    }
}
impl Default for Shaped { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: OpenFace, src/game/breakTable.ts:261)
#[derive(Clone, Debug)]
pub struct OpenFace {
    pub axis: f64,
    pub sign: f64,
    pub lat: Vec<f64>,
    pub seed: f64,
    pub lo: Option<f64>,
    pub head: Option<f64>,
}
impl OpenFace {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OpenFace {
            axis: 0.0,
            sign: 0.0,
            lat: Vec::new(),
            seed: 0.0,
            lo: None,
            head: None,
        }
    }
}
impl Default for OpenFace { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ShedBudget, src/game/breakTable.ts:332)
#[derive(Clone, Copy, Debug)]
pub struct ShedBudget {
    pub chunks: f64,
    pub heap: Option<f64>,
    pub from: Option<V3>,
    pub per: Option<f64>,
}
impl ShedBudget {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ShedBudget {
            chunks: 0.0,
            heap: None,
            from: None,
            per: None,
        }
    }
}
impl Default for ShedBudget { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CellGrid, src/game/breakTable.ts:333)
#[derive(Clone, Debug)]
pub struct CellGrid {
    pub id: f64,
    pub surf: crate::scene_materials::Surf,
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub yb: Vec<f64>,
    pub open: Vec<u8>,
    pub n_open: Option<f64>,
    pub box_: BoxId,
}
impl CellGrid {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CellGrid {
            id: 0.0,
            surf: crate::scene_materials::Surf::Carpet,
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            yb: Vec::new(),
            open: Vec::new(),
            n_open: None,
            box_: BoxId(0),
        }
    }
}
impl Default for CellGrid { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:139 matOf(box), src/game/breakTable.ts:139)
#[derive(Clone, Debug)]
pub struct MatOfBox {
    pub does: Option<String>,
}
impl MatOfBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MatOfBox {
            does: None,
        }
    }
}
impl Default for MatOfBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:351 share ->, src/game/breakTable.ts:351)
#[derive(Clone, Copy, Debug)]
pub struct BreaksShare {
    pub thrown: f64,
    pub heap: f64,
    pub furniture: f64,
}
impl BreaksShare {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksShare {
            thrown: 0.0,
            heap: 0.0,
            furniture: 0.0,
        }
    }
}
impl Default for BreaksShare { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:361 lands ->, src/game/breakTable.ts:361)
#[derive(Clone, Copy, Debug)]
pub struct BreaksLands {
    pub level: f64,
    pub dust: f64,
}
impl BreaksLands {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksLands {
            level: 0.0,
            dust: 0.0,
        }
    }
}
impl Default for BreaksLands { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:612 shedCell ->, src/game/breakTable.ts:612)
#[derive(Clone, Copy, Debug)]
pub struct BreaksShedCell {
    pub thrown: f64,
    pub refused: f64,
}
impl BreaksShedCell {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksShedCell {
            thrown: 0.0,
            refused: 0.0,
        }
    }
}
impl Default for BreaksShedCell { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/breakTable.ts:722 studs ->, src/game/breakTable.ts:722)
#[derive(Clone, Copy, Debug)]
pub struct BreaksStudsItem {
    pub c: V3,
    pub h: V3,
    pub albedo: V3,
    pub rough: f64,
    pub metal: bool,
    pub does: crate::scene_materials::DoesWord,
}
impl BreaksStudsItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BreaksStudsItem {
            c: [0.0; 3],
            h: [0.0; 3],
            albedo: [0.0; 3],
            rough: 0.0,
            metal: false,
            does: crate::scene_materials::DoesWord::Snow,
        }
    }
}
impl Default for BreaksStudsItem { fn default() -> Self { Self::zeroed() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct BreakTableStats {
    pub shaped: i32,
    pub plain: i32,
    pub refused: i32,
    pub again: i32,
    pub mayhem: i32,
    pub china: i32,
    pub crumbled: i32,
    pub heap: i32,
    pub ragged: i32,
    pub planes: i32,
    pub dropped: i32,
    pub chips: i32,
    pub dressed: i32,
    pub kinked: i32,
    pub loose: i32,
    pub sheets: i32,
    pub carcasses: i32,
    pub notched: i32,
}

/// (src/game/breakTable.ts:58 NEVER)
pub const NEVER: f64 = f64::INFINITY;

/// (src/game/breakTable.ts:61 BREAK)
pub const BREAK: bool = false;

/// (src/game/breakTable.ts:136 SURF_MAT: a table nothing writes)
pub fn surf_mat() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("metal"), String::from("metal")); t.set(String::from("plastic"), String::from("plastic")); t.set(String::from("glass"), String::from("screen")); t.set(String::from("wood"), String::from("wood")); t.set(String::from("cardboard"), String::from("cardboard")); t.set(String::from("drywall"), String::from("plasterboard")); t.set(String::from("concrete"), String::from("masonry")); t.set(String::from("carpet"), String::from("carpet")); t.set(String::from("fabric"), String::from("fabric")); t.set(String::from("foliage"), String::from("plant")); t } }

/// (src/game/breakTable.ts:157 RAD)
pub const RAD: f64 = 0.017453292519943295;
