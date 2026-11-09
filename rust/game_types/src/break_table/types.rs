//! The types of `break_table`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Breaks {
    #[is(1.0)] pub counter: f64,
    pub queue: Vec<DebrisChunkId>,
    #[is(1.0)] pub pcounter: f64,
    pub look: Option<crate::break_table::BreaksLook>,
    #[is(-1000000000.0)] pub kink_t: f64,
}

/// WHAT A THING IS INSIDE, where it is broken: asked of the thing (`inside_of`), never of the row that gives its pieces their shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inside {
    /// the colour of its broken faces; None = its own paint, lightened by `pale`
    pub core: Option<V3>,
    pub pale: Option<f64>,
    /// what its sheet is faced with; None = it has no facing
    pub facing: Option<V3>,
    /// built on a frame that a hole in it leaves showing (studs, a track, reinforcing); false = it stands by itself
    pub framed: bool,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreaksLook {
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum Mat {
    #[default] Carpet,
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Shape {
    Panel,
    Slab,
    Snapped,
    Strip,
    Wedge,
    Lump,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum Split {
    #[default] Along,
    Across,
    Diag,
}

#[derive(Clone, Debug)]
pub struct BreakRow {
    pub core: Option<V3>,
    pub pale: Option<f64>,
    /// what a sheet of this row is faced with, where a torn edge shows it (the paper on plasterboard); None = nothing
    pub facing: Option<V3>,
    pub cell: &'static [crate::break_table::Shape],
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
impl Default for BreakRow {
    fn default() -> Self {
        BreakRow {
            core: None,
            pale: None,
            facing: None,
            cell: &[],
            thick: 0.0,
            span: [0.0; 2],
            slim: [0.0; 2],
            tears: [0.0; 2],
            ragged: None,
            again: None,
            small: crate::break_table::BreakRowSmall::default(),
            dust: 0.0,
            sparks: 0.0,
            puff: 0.0,
            rest: 0.0,
            level: 0.0,
            mark: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreakRowRagged {
    pub deg: [f64; 2],
    pub bite: f64,
    pub grain: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreakRowAgain {
    pub land: f64,
    pub kick: f64,
    pub how: crate::break_table::Split,
    pub chips: f64,
    pub chance: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreakRowSmall {
    pub n: f64,
    pub size: [f64; 2],
    pub aspect: [f64; 2],
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BreakMeta {
    pub mat: crate::break_table::Mat,
    pub gen_: f64,
    pub paint: V3,
    pub fixed: Option<bool>,
}

#[derive(Clone, Debug, Default)]
pub struct Shaped {
    pub off: V3,
    pub h: V3,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
}

#[derive(Clone, Debug, Default)]
pub struct OpenFace {
    pub axis: f64,
    pub sign: f64,
    pub lat: Vec<f64>,
    pub seed: f64,
    pub lo: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ShedBudget {
    pub chunks: f64,
    pub heap: Option<f64>,
    pub from: Option<V3>,
    pub per: Option<f64>,
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, Default)]
pub struct MatOfBox {
    pub does: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreaksShare {
    pub thrown: f64,
    pub heap: f64,
    pub furniture: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreaksLands {
    pub level: f64,
    pub dust: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreaksShedCell {
    pub thrown: f64,
    pub refused: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreaksStudsItem {
    pub c: V3,
    pub h: V3,
    pub albedo: V3,
    pub rough: f64,
    pub metal: bool,
    pub does: crate::scene_materials::DoesWord,
}

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
}

pub const NEVER: f64 = f64::INFINITY;

pub fn break_() -> &'static [(crate::break_table::Mat, crate::break_table::BreakRow)] { &[(Mat::Plasterboard, BreakRow { core: Some([0.92, 0.91, 0.88]), pale: None, facing: Some([0.52, 0.47, 0.39]), cell: &[Shape::Slab, Shape::Wedge], thick: 0.015, span: [0.15, 0.7], slim: [0.35, 0.8], tears: [2.0, 3.0], ragged: Some(BreakRowRagged { deg: [8.0, 25.0], bite: 0.12, grain: None }), again: Some(BreakRowAgain { land: 6.5, kick: 0.7, how: Split::Across, chips: 4.0, chance: None }), small: BreakRowSmall { n: 18.0, size: [0.01, 0.035], aspect: [1.0, 1.6] }, dust: 6.0, sparks: 0.0, puff: 1.4, rest: 0.1, level: 0.0, mark: 0.03 }), (Mat::Masonry, BreakRow { core: Some([0.6, 0.59, 0.57]), pale: None, facing: None, cell: &[], thick: 0.03, span: [0.1, 0.2], slim: [0.6, 1.0], tears: [2.0, 3.0], ragged: None, again: None, small: BreakRowSmall { n: 16.0, size: [0.008, 0.02], aspect: [1.0, 1.0] }, dust: 5.0, sparks: 5.0, puff: 1.0, rest: 0.1, level: 0.0, mark: 0.02 }), (Mat::Blockwork, BreakRow { core: Some([0.6, 0.59, 0.57]), pale: None, facing: None, cell: &[Shape::Lump, Shape::Slab], thick: 0.05, span: [0.16, 0.4], slim: [0.55, 0.95], tears: [2.0, 3.0], ragged: None, again: None, small: BreakRowSmall { n: 20.0, size: [0.01, 0.035], aspect: [1.0, 1.4] }, dust: 8.0, sparks: 3.0, puff: 1.8, rest: 0.08, level: 0.0, mark: 0.03 }), (Mat::Wood, BreakRow { core: Some([0.78, 0.63, 0.42]), pale: None, facing: None, cell: &[Shape::Snapped, Shape::Strip], thick: 0.011, span: [0.3, 0.6], slim: [0.16, 0.33], tears: [1.0, 2.0], ragged: Some(BreakRowRagged { deg: [8.0, 25.0], bite: 0.12, grain: Some(true) }), again: Some(BreakRowAgain { land: NEVER, kick: 0.85, how: Split::Along, chips: 4.0, chance: None }), small: BreakRowSmall { n: 18.0, size: [0.006, 0.014], aspect: [3.0, 6.0] }, dust: 3.0, sparks: 0.0, puff: 0.0, rest: 0.25, level: 0.0, mark: 0.014 }), (Mat::Metal, BreakRow { core: None, pale: None, facing: None, cell: &[Shape::Panel], thick: 0.004, span: [0.3, 0.48], slim: [0.6, 1.0], tears: [0.0, 0.0], ragged: None, again: None, small: BreakRowSmall { n: 2.0, size: [0.006, 0.012], aspect: [1.0, 2.0] }, dust: 0.0, sparks: 16.0, puff: 0.0, rest: 0.35, level: 0.5, mark: 0.0 }), (Mat::Glass, BreakRow { core: None, pale: None, facing: None, cell: &[], thick: 0.003, span: [0.1, 0.3], slim: [0.4, 1.0], tears: [0.0, 0.0], ragged: None, again: None, small: BreakRowSmall { n: 0.0, size: [0.01, 0.04], aspect: [1.0, 2.0] }, dust: 0.0, sparks: 0.0, puff: 0.0, rest: 0.22, level: 0.0, mark: 0.0 }), (Mat::GlassTough, BreakRow { core: None, pale: None, facing: None, cell: &[], thick: 0.003, span: [0.1, 0.3], slim: [0.4, 1.0], tears: [0.0, 0.0], ragged: None, again: None, small: BreakRowSmall { n: 0.0, size: [0.005, 0.01], aspect: [1.0, 1.0] }, dust: 0.0, sparks: 0.0, puff: 0.0, rest: 0.22, level: 0.0, mark: 0.0 }), (Mat::GlassLam, BreakRow { core: None, pale: None, facing: None, cell: &[], thick: 0.003, span: [0.1, 0.3], slim: [0.4, 1.0], tears: [0.0, 0.0], ragged: None, again: None, small: BreakRowSmall { n: 0.0, size: [0.005, 0.01], aspect: [1.0, 1.0] }, dust: 0.0, sparks: 0.0, puff: 0.0, rest: 0.22, level: 0.0, mark: 0.0 }), (Mat::Screen, BreakRow { core: Some([0.09, 0.09, 0.1]), pale: None, facing: None, cell: &[Shape::Wedge, Shape::Wedge], thick: 0.02, span: [0.15, 0.3], slim: [0.5, 0.9], tears: [1.0, 2.0], ragged: None, again: Some(BreakRowAgain { land: 5.0, kick: NEVER, how: Split::Diag, chips: 5.0, chance: None }), small: BreakRowSmall { n: 14.0, size: [0.006, 0.014], aspect: [1.0, 2.0] }, dust: 0.0, sparks: 6.0, puff: 0.0, rest: 0.22, level: 0.0, mark: 0.0 }), (Mat::Fabric, BreakRow { core: None, pale: Some(0.35), facing: None, cell: &[Shape::Wedge, Shape::Wedge], thick: 0.012, span: [0.3, 0.5], slim: [0.6, 1.0], tears: [1.0, 1.0], ragged: Some(BreakRowRagged { deg: [8.0, 15.0], bite: 0.08, grain: None }), again: None, small: BreakRowSmall { n: 5.0, size: [0.01, 0.02], aspect: [1.0, 1.0] }, dust: 3.0, sparks: 0.0, puff: 0.0, rest: 0.1, level: 0.0, mark: 0.0 }), (Mat::Paper, BreakRow { core: None, pale: None, facing: None, cell: &[], thick: 0.001, span: [0.2, 0.3], slim: [0.7, 0.7], tears: [0.0, 0.0], ragged: None, again: None, small: BreakRowSmall { n: 0.0, size: [0.01, 0.02], aspect: [1.0, 1.0] }, dust: 0.0, sparks: 0.0, puff: 0.0, rest: 0.1, level: 0.0, mark: 0.0 }), (Mat::Cardboard, BreakRow { core: Some([0.66, 0.54, 0.38]), pale: None, facing: None, cell: &[Shape::Slab, Shape::Slab], thick: 0.015, span: [0.2, 0.4], slim: [0.55, 1.0], tears: [1.0, 1.0], ragged: Some(BreakRowRagged { deg: [8.0, 15.0], bite: 0.12, grain: None }), again: None, small: BreakRowSmall { n: 8.0, size: [0.008, 0.02], aspect: [1.0, 2.0] }, dust: 3.0, sparks: 0.0, puff: 0.0, rest: 0.1, level: 0.0, mark: 0.0 }), (Mat::Tile, BreakRow { core: Some([0.9, 0.89, 0.86]), pale: None, facing: None, cell: &[], thick: 0.0075, span: [0.3, 0.58], slim: [0.6, 1.0], tears: [1.0, 2.0], ragged: None, again: Some(BreakRowAgain { land: 5.0, kick: NEVER, how: Split::Across, chips: 6.0, chance: None }), small: BreakRowSmall { n: 6.0, size: [0.009, 0.022], aspect: [1.0, 1.6] }, dust: 4.0, sparks: 0.0, puff: 1.0, rest: 0.1, level: 0.0, mark: 0.03 }), (Mat::Plastic, BreakRow { core: None, pale: Some(0.25), facing: None, cell: &[Shape::Wedge, Shape::Wedge], thick: 0.01, span: [0.08, 0.2], slim: [0.5, 1.0], tears: [1.0, 2.0], ragged: None, again: Some(BreakRowAgain { land: 5.0, kick: NEVER, how: Split::Diag, chips: 8.0, chance: None }), small: BreakRowSmall { n: 8.0, size: [0.006, 0.014], aspect: [1.0, 2.0] }, dust: 0.0, sparks: 0.0, puff: 0.0, rest: 0.22, level: 0.0, mark: 0.0 }), (Mat::Carpet, BreakRow { core: None, pale: None, facing: None, cell: &[], thick: 0.005, span: [0.1, 0.2], slim: [0.6, 1.0], tears: [0.0, 0.0], ragged: None, again: None, small: BreakRowSmall { n: 4.0, size: [0.008, 0.016], aspect: [1.0, 3.0] }, dust: 4.0, sparks: 0.0, puff: 0.0, rest: 0.1, level: 0.0, mark: 0.0 }), (Mat::Plant, BreakRow { core: Some([0.62, 0.36, 0.24]), pale: None, facing: None, cell: &[Shape::Wedge, Shape::Wedge], thick: 0.02, span: [0.1, 0.25], slim: [0.6, 1.0], tears: [1.0, 2.0], ragged: None, again: Some(BreakRowAgain { land: NEVER, kick: 0.85, how: Split::Diag, chips: 6.0, chance: None }), small: BreakRowSmall { n: 16.0, size: [0.008, 0.018], aspect: [1.0, 2.5] }, dust: 2.0, sparks: 0.0, puff: 0.0, rest: 0.1, level: 0.0, mark: 0.02 })] }

pub const RAD: f64 = 0.017453292519943295;
