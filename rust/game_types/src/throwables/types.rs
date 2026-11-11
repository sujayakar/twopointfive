//! The types of `throwables`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what an item does when it bounces (Item.onBounce); called from throwables_say with speed, point, item and face normal
#[derive(Clone, Debug, ToBytes)]
pub enum ItemBounce {
    Can,
    CanHeard,
    RingSkitter,
    ShellSkitter,
    Mag,
    MagRate { belt: bool },
    Dropped { kind: crate::throwables::ItemKind, quiet: bool },
    GunDown,
    Loaded { kind: crate::throwables::ItemKind },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum ItemKind {
    Smoke,
    Mag,
    Flash,
    Frag,
    #[default] Pistol,
    Torch,
    Rifle,
    Ring,
    Shell,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct ThrowSolution {
    pub v0: V3,
    pub t: f64,
    pub points: Vec<V3>,
    pub land: V3,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Throwables {
    /// the things in the pool, the oldest first: a full pool gives up the first spent thing in it, the boxes the picture draws are listed in it, the step walks it from the newest down
    pub items: Vec<ItemId>,
    #[is(1.0)] pub serial: f64,
}

/// a thing thrown or dropped, in the air, in a hand or on the floor
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Item {
    pub kind: crate::throwables::ItemKind,
    pub pos: V3,
    pub vel: V3,
    pub yaw: f64,
    pub spin: f64,
    pub tumble: f64,
    pub tumble_rate: f64,
    pub age: f64,
    pub life: f64,
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    /// the way it travels
    pub heading: f64,
    /// the thing of furniture it lies on: its id plus one (0: none), and that thing's pose as one number
    pub on: u32,
    pub on_pose: f64,
    /// lying at rest
    pub settled: bool,
    pub bounces: f64,
    pub who: f64,
    pub serial: f64,
    // other rules' own: the fuse, the seconds of venting left, what a magazine holds
    pub fuse: f64,
    pub fired: bool,
    pub emit_t: f64,
    pub fill: f64,
    /// in a hand, placed by whoever carries it; unset is not false to a save file
    pub carried: Option<bool>,
    pub side: Option<crate::teams::Team>,
    pub on_bounce: Option<crate::throwables::ItemBounce>,
    pub clatters: Option<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SC20KITEM {
    pub thick: f64,
    pub metal: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M4ITEM {
    pub hull: crate::throwables::M4ITEMHull,
    pub fore: crate::throwables::M4ITEMFore,
    pub mag: crate::throwables::M4ITEMMag,
    pub optic: crate::throwables::M4ITEMOptic,
    pub polymer: V3,
    pub metal: V3,
    pub fore_col: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M4ITEMHull {
    pub z: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M4ITEMFore {
    pub z: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M4ITEMMag {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M4ITEMOptic {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CANITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct FRAGITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub half: [f64; 3],
    pub body: V3,
    pub band: V3,
    pub fuze: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RINGITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub life: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SPONGEITEM {
    pub wide: f64,
    pub nose: crate::throwables::SPONGEITEMNose,
    pub body: crate::throwables::SPONGEITEMBody,
    pub band: crate::throwables::SPONGEITEMBand,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SPONGEITEMNose {
    pub z: f64,
    pub h: [f64; 3],
    pub albedo: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SPONGEITEMBody {
    pub z: f64,
    pub h: [f64; 3],
    pub albedo: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SPONGEITEMBand {
    pub z: f64,
    pub h: [f64; 3],
    pub albedo: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SHELLITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub life: f64,
    pub half: [f64; 3],
    pub body: V3,
    pub nose: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIFLEITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub prop_share: f64,
    pub thick: f64,
    pub receiver: crate::throwables::RIFLEITEMReceiver,
    pub fore: crate::throwables::RIFLEITEMFore,
    pub mag: crate::throwables::RIFLEITEMMag,
    pub stock: crate::throwables::RIFLEITEMStock,
    pub steel: V3,
    pub wood: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIFLEITEMReceiver {
    pub z: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIFLEITEMFore {
    pub z: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIFLEITEMMag {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
    pub cant: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RIFLEITEMStock {
    pub z: f64,
    pub h: [f64; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ThrowablesSpawnOpts {
    pub kind: Option<crate::throwables::ItemKind>,
    pub pos: Option<V3>,
    pub vel: Option<V3>,
    pub radius: Option<f64>,
    pub restitution: Option<f64>,
    pub friction: Option<f64>,
    pub yaw: Option<f64>,
    pub spin: Option<f64>,
    pub settled: Option<bool>,
    pub age: Option<f64>,
    pub fuse: Option<f64>,
    pub fired: Option<bool>,
    pub life: Option<f64>,
    pub tumble: Option<f64>,
    pub tumble_rate: Option<f64>,
    pub emit_t: Option<f64>,
    pub fill: Option<f64>,
    pub bounces: Option<f64>,
    pub heading: Option<f64>,
    pub who: Option<f64>,
    pub on: Option<PropId>,
    pub on_pose: Option<f64>,
    pub serial: Option<f64>,
    pub side: Option<crate::teams::Team>,
    pub clatters: Option<f64>,
    pub carried: Option<bool>,
}

pub fn sc20_k_item() -> crate::throwables::SC20KITEM { crate::throwables::SC20KITEM { thick: 0.024, metal: [0.026, 0.026, 0.03] } }

pub fn m4_item() -> crate::throwables::M4ITEM { crate::throwables::M4ITEM { hull: crate::throwables::M4ITEMHull { z: -0.09, h: [0.017, 0.034, 0.2] }, fore: crate::throwables::M4ITEMFore { z: 0.285, h: [0.023, 0.021, 0.17] }, mag: crate::throwables::M4ITEMMag { z: 0.085, y: -0.095, h: [0.011, 0.065, 0.028] }, optic: crate::throwables::M4ITEMOptic { z: 0.035, y: 0.052, h: [0.016, 0.02, 0.035] }, polymer: [0.03, 0.03, 0.033], metal: [0.03, 0.03, 0.033], fore_col: [0.2, 0.155, 0.1] } }

pub fn can_item() -> crate::throwables::CANITEM { crate::throwables::CANITEM { radius: 0.055, restitution: 0.38, friction: 0.35 } }

pub fn frag_item() -> crate::throwables::FRAGITEM { crate::throwables::FRAGITEM { radius: 0.04, restitution: 0.3, friction: 0.45, half: [0.032, 0.032, 0.038], body: [0.13, 0.17, 0.08], band: [0.75, 0.62, 0.1], fuze: [0.3, 0.3, 0.28] } }

pub fn ring_item() -> crate::throwables::RINGITEM { crate::throwables::RINGITEM { radius: 0.03, restitution: 0.3, friction: 0.6, life: 60.0 } }

pub fn sponge_item() -> crate::throwables::SPONGEITEM { crate::throwables::SPONGEITEM { wide: 0.023, nose: crate::throwables::SPONGEITEMNose { z: 0.019, h: [0.022, 0.022, 0.019], albedo: [0.07, 0.24, 0.62] }, body: crate::throwables::SPONGEITEMBody { z: -0.014, h: [0.021, 0.021, 0.014], albedo: [0.05, 0.05, 0.055] }, band: crate::throwables::SPONGEITEMBand { z: -0.033, h: [0.023, 0.023, 0.005], albedo: [0.3, 0.3, 0.31] } } }

pub fn shell_item() -> crate::throwables::SHELLITEM { crate::throwables::SHELLITEM { radius: 0.035, restitution: 0.22, friction: 0.5, life: 120.0, half: [0.02, 0.02, 0.05], body: [0.42, 0.43, 0.4], nose: [0.72, 0.55, 0.12] } }

pub fn rifle_item() -> crate::throwables::RIFLEITEM { crate::throwables::RIFLEITEM { radius: 0.06, restitution: 0.18, friction: 0.55, prop_share: 0.3, thick: 0.02, receiver: crate::throwables::RIFLEITEMReceiver { z: -0.02, h: [0.018, 0.03, 0.13] }, fore: crate::throwables::RIFLEITEMFore { z: 0.24, h: [0.02, 0.022, 0.13] }, mag: crate::throwables::RIFLEITEMMag { z: 0.06, y: -0.075, h: [0.013, 0.05, 0.03], cant: 0.35 }, stock: crate::throwables::RIFLEITEMStock { z: -0.27, h: [0.008, 0.026, 0.12] }, steel: [0.05, 0.05, 0.055], wood: [0.2, 0.085, 0.05] } }

pub const GRAVITY: f64 = 9.81;

/// the pool's own size: a thirty-third gives one up, but never a can still on its fuse or venting
pub const ITEM_CAP: f64 = 32.0;
/// the most it may grow to with those: sixty-four live cans at once is nobody's game, and there the oldest gives way
pub const ITEM_ROWS: f64 = 64.0;
