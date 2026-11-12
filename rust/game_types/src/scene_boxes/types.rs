//! The types of `scene_boxes`.
use crate::{Blank, bytes::ToBytes, ids::*, Quat, V3};
use alloc::{sync::Arc, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Box {
    pub c: V3,
    pub h: V3,
    pub yaw: f64,
    pub albedo: V3,
    pub emissive: V3,
    pub flags: f64,
    pub owner: f64,
    pub cut_height: f64,
    pub wet_k: Option<f64>,
    pub rot: Option<Quat>,
    pub dc: Option<V3>,
    pub dh: Option<V3>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub aniso: Option<bool>,
    pub marks: Option<Arc<[crate::scene_markings::MarkingDef]>>,
    pub mark_gain: Option<f64>,
    pub fog_emit: Option<[f64; 3]>,
    pub fog_mark: Option<f64>,
    pub mkey: Option<f64>,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub cut_core: Option<V3>,
    pub cut_glass: Option<crate::scene_cut::CutGlass>,
    pub cut_round: Option<f64>,
    pub cut_arc: Option<f64>,
    pub cut_old: Option<bool>,
    pub cut_core_stuff: Option<crate::scene_stuff::BoxStuff>,
    pub round: Option<bool>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    /// a wall's stud, left standing where its board went
    pub stud: bool,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct BoxWorld {
    pub statics: Vec<BoxId>,
    pub dynamics: Vec<AnyBoxId>,
    pub max_boxes: f64,
    pub geom_serial: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MKEY {
    pub door: f64,
    pub prop: f64,
    pub item: f64,
    pub rig: f64,
    pub device: f64,
    pub test: f64,
    pub debris: f64,
    pub look: f64,
    pub skin: f64,
}

#[derive(Clone, Debug, Default)]
pub struct MakeBoxP {
    pub c: V3,
    pub h: V3,
    pub yaw: Option<f64>,
    pub albedo: Option<V3>,
    pub emissive: Option<V3>,
    pub flags: Option<f64>,
    pub owner: Option<f64>,
    pub cut_height: Option<f64>,
    pub wet_k: Option<f64>,
    pub rot: Option<Quat>,
    pub dc: Option<V3>,
    pub dh: Option<V3>,
    pub rough: Option<f64>,
    pub metal: Option<bool>,
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
    pub aniso: Option<bool>,
    pub marks: Option<Arc<[crate::scene_markings::MarkingDef]>>,
    pub mark_gain: Option<f64>,
    pub fog_emit: Option<[f64; 3]>,
    pub fog_mark: Option<f64>,
    pub mkey: Option<f64>,
    pub cut: Option<Vec<crate::scene_cut::CutPlane>>,
    pub cut_core: Option<V3>,
    pub cut_glass: Option<crate::scene_cut::CutGlass>,
    pub cut_round: Option<f64>,
    pub cut_arc: Option<f64>,
    pub cut_old: Option<bool>,
    pub cut_core_stuff: Option<crate::scene_stuff::BoxStuff>,
    pub round: Option<bool>,
    pub shape: Option<crate::scene_shapes::BoxShape>,
    pub shaped: Option<bool>,
    pub piece: Option<bool>,
    pub of: Option<f64>,
    pub dress: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordOfB {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WordOf {
    pub stuff: Option<crate::scene_stuff::BoxStuff>,
    pub does: Option<crate::scene_materials::DoesWord>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoxWorldRaycast {
    pub t: f64,
    pub index: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBox {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxCut {
    pub t: f64,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RayBoxRound {
    pub t: f64,
    pub n: V3,
}

pub const BOX_WORLD_RAY_PAD: f64 = 0.001;

crate::bits! {
    /// what a box is, as bits of its `flags`: some a level file says, some a rule sets as it makes the box
    pub mod box_flag as BoxFlag {
        /// not rasterized into the G-buffer (ceilings, light-only geometry)
        NO_PRIMARY = 1,
        /// ignored by all rays (decals, glow cards)
        NO_SHADOW = 2,
        /// rasterized only below the cut height (a stage drops its back wall to a lip when the orbit camera goes behind it); rays see the full box
        CUTAWAY = 4,
        /// rebuilt every frame (character proxies, props on bones)
        DYNAMIC = 8,
        /// has non-zero emission (lets a shader early-out)
        EMISSIVE = 16,
        /// its emission is an analytic area light's: rays ignore the emissive term (no double count, no small-emitter aliasing in the cascades)
        AREA_LIT = 32,
        /// carries a full 3D rotation (set as it is packed): raster and rays take the quaternion in place of the packed yaw
        ROT = 64,
        /// polished floor finish (lobby stone, the lino): the gather traces one Fresnel-weighted reflection ray for on-screen pixels of its top face
        GLOSS = 128,
        /// the rules' alone: a door's leaf near closed, which seals sound
        SOUND_SEAL = 1 << 16,
        /// a pane: no traced ray and no raster triangle sees it, view and shadow segments are tested against it analytically (reflection, tint, dust glow). Movement and
        /// sound treat it as a wall, sight passes it, a round or a kick breaks it
        GLASS = 1 << 17,
        /// the rules' alone: a thin thing that stands free (a stovepipe, a window's bar, a lone post), which the fog of war's own sight passes
        THIN = 1 << 18,
        /// the rules' alone: a window's bar (the kit's upright between a window's bays): a round that meets one breaks it, and a line of fire passes one
        BAR = 1 << 19,
    }
}

pub const SHEET_MASK: u32 = box_flag::GLASS | box_flag::DYNAMIC | box_flag::NO_SHADOW | box_flag::NO_PRIMARY;

pub fn mkey() -> crate::scene_boxes::MKEY { crate::scene_boxes::MKEY { door: 16777216.0, prop: 33554432.0, item: 50331648.0, rig: 67108864.0, device: 83886080.0, test: 117440512.0, debris: 134217728.0, look: 150994944.0, skin: 167772160.0 } }

