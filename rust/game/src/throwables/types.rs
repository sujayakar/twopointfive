//! The types of `throwables`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what an item does when it bounces (Item.onBounce); called from throwables_say with speed, point, item and face normal
#[derive(Clone, Debug)]
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

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ItemKind {
    Pistol,
    Rifle,
    Ring,
    Smoke,
    Flash,
    Frag,
    Mag,
    Torch,
    Shell,
}
impl ItemKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            ItemKind::Pistol => "pistol",
            ItemKind::Rifle => "rifle",
            ItemKind::Ring => "ring",
            ItemKind::Smoke => "smoke",
            ItemKind::Flash => "flash",
            ItemKind::Frag => "frag",
            ItemKind::Mag => "mag",
            ItemKind::Torch => "torch",
            ItemKind::Shell => "shell",
        }
    }
}

/// (TypeScript: ItemFields, src/game/throwables.ts:102)
#[derive(Clone, Copy, Debug)]
pub struct ItemFields {
    pub kind: crate::throwables::ItemKind,
    pub pos: V3,
    pub vel: V3,
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub yaw: f64,
    pub spin: f64,
    pub settled: bool,
    pub age: f64,
    pub fuse: f64,
    pub fired: bool,
    pub life: f64,
    pub tumble: f64,
    pub tumble_rate: f64,
    pub emit_t: f64,
    pub fill: f64,
    pub bounces: f64,
    pub heading: f64,
    pub who: f64,
    pub on: Option<PropId>,
    pub on_pose: f64,
    pub serial: f64,
    pub side: Option<crate::teams::Team>,
    pub clatters: Option<f64>,
    pub carried: Option<bool>,
}
impl ItemFields {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ItemFields {
            kind: crate::throwables::ItemKind::Pistol,
            pos: [0.0; 3],
            vel: [0.0; 3],
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
            yaw: 0.0,
            spin: 0.0,
            settled: false,
            age: 0.0,
            fuse: 0.0,
            fired: false,
            life: 0.0,
            tumble: 0.0,
            tumble_rate: 0.0,
            emit_t: 0.0,
            fill: 0.0,
            bounces: 0.0,
            heading: 0.0,
            who: 0.0,
            on: None,
            on_pose: 0.0,
            serial: 0.0,
            side: None,
            clatters: None,
            carried: None,
        }
    }
}
impl Default for ItemFields { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Item, src/game/throwables.ts:131)
#[derive(Clone, Debug)]
pub struct Item {
    pub r: f64,
    pub kept: Option<Vec<f64>>,
    pub side: Option<crate::teams::Team>,
    pub on_bounce: Option<crate::throwables::ItemBounce>,
    pub clatters: Option<f64>,
    pub hand: Option<bool>,
    pub things: Vec<PropId>,
}
impl Item {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Item {
            r: 0.0,
            kept: None,
            side: None,
            on_bounce: None,
            clatters: None,
            hand: None,
            things: Vec::new(),
        }
    }
}
impl Default for Item { fn default() -> Self { Self::blank() } }

/// (TypeScript: ThrowSolution, src/game/throwables.ts:180)
#[derive(Clone, Debug)]
pub struct ThrowSolution {
    pub v0: V3,
    pub t: f64,
    pub points: Vec<V3>,
    pub land: V3,
}
impl ThrowSolution {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowSolution {
            v0: [0.0; 3],
            t: 0.0,
            points: Vec::new(),
            land: [0.0; 3],
        }
    }
}
impl Default for ThrowSolution { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: BogGround, src/game/throwables.ts:201)
#[derive(Clone, Copy, Debug)]
pub struct BogGround {
}
impl BogGround {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BogGround {
        }
    }
}
impl Default for BogGround { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Throwables, src/game/throwables.ts:203)
#[derive(Clone, Debug)]
pub struct Throwables {
    pub items: Vec<ItemId>,
    pub rows: Vec<ItemId>,
    pub store: crate::sim_itemshome::ItemStore,
    pub serial: f64,
}
impl Throwables {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Throwables {
            items: Vec::new(),
            rows: Vec::new(),
            store: crate::sim_itemshome::ItemStore::blank(),
            serial: 1.0,
        }
    }
}
impl Default for Throwables { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/throwables.ts:40 SC20K_ITEM, src/game/throwables.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct SC20KITEM {
    pub hull: crate::throwables::SC20KITEMHull,
    pub fore: crate::throwables::SC20KITEMFore,
    pub mag: crate::throwables::SC20KITEMMag,
    pub optic: crate::throwables::SC20KITEMOptic,
    pub thick: f64,
    pub polymer: V3,
    pub metal: V3,
}
impl SC20KITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SC20KITEM {
            hull: crate::throwables::SC20KITEMHull::zeroed(),
            fore: crate::throwables::SC20KITEMFore::zeroed(),
            mag: crate::throwables::SC20KITEMMag::zeroed(),
            optic: crate::throwables::SC20KITEMOptic::zeroed(),
            thick: 0.0,
            polymer: [0.0; 3],
            metal: [0.0; 3],
        }
    }
}
impl Default for SC20KITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:41 SC20KITEM.hull, src/game/throwables.ts:41)
#[derive(Clone, Copy, Debug)]
pub struct SC20KITEMHull {
    pub z: f64,
    pub h: [f64; 3],
}
impl SC20KITEMHull {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SC20KITEMHull {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for SC20KITEMHull { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:42 SC20KITEM.fore, src/game/throwables.ts:42)
#[derive(Clone, Copy, Debug)]
pub struct SC20KITEMFore {
    pub z: f64,
    pub h: [f64; 3],
}
impl SC20KITEMFore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SC20KITEMFore {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for SC20KITEMFore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:43 SC20KITEM.mag, src/game/throwables.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct SC20KITEMMag {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
}
impl SC20KITEMMag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SC20KITEMMag {
            z: 0.0,
            y: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for SC20KITEMMag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:44 SC20KITEM.optic, src/game/throwables.ts:44)
#[derive(Clone, Copy, Debug)]
pub struct SC20KITEMOptic {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
}
impl SC20KITEMOptic {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SC20KITEMOptic {
            z: 0.0,
            y: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for SC20KITEMOptic { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:51 M4_ITEM, src/game/throwables.ts:51)
#[derive(Clone, Copy, Debug)]
pub struct M4ITEM {
    pub hull: crate::throwables::M4ITEMHull,
    pub fore: crate::throwables::M4ITEMFore,
    pub mag: crate::throwables::M4ITEMMag,
    pub optic: crate::throwables::M4ITEMOptic,
    pub polymer: V3,
    pub metal: V3,
    pub fore_col: V3,
}
impl M4ITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        M4ITEM {
            hull: crate::throwables::M4ITEMHull::zeroed(),
            fore: crate::throwables::M4ITEMFore::zeroed(),
            mag: crate::throwables::M4ITEMMag::zeroed(),
            optic: crate::throwables::M4ITEMOptic::zeroed(),
            polymer: [0.0; 3],
            metal: [0.0; 3],
            fore_col: [0.0; 3],
        }
    }
}
impl Default for M4ITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:52 M4ITEM.hull, src/game/throwables.ts:52)
#[derive(Clone, Copy, Debug)]
pub struct M4ITEMHull {
    pub z: f64,
    pub h: [f64; 3],
}
impl M4ITEMHull {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        M4ITEMHull {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for M4ITEMHull { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:53 M4ITEM.fore, src/game/throwables.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct M4ITEMFore {
    pub z: f64,
    pub h: [f64; 3],
}
impl M4ITEMFore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        M4ITEMFore {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for M4ITEMFore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:54 M4ITEM.mag, src/game/throwables.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct M4ITEMMag {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
}
impl M4ITEMMag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        M4ITEMMag {
            z: 0.0,
            y: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for M4ITEMMag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:55 M4ITEM.optic, src/game/throwables.ts:55)
#[derive(Clone, Copy, Debug)]
pub struct M4ITEMOptic {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
}
impl M4ITEMOptic {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        M4ITEMOptic {
            z: 0.0,
            y: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for M4ITEMOptic { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:59 CAN_ITEM, src/game/throwables.ts:59)
#[derive(Clone, Copy, Debug)]
pub struct CANITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
}
impl CANITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CANITEM {
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
        }
    }
}
impl Default for CANITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:63 FRAG_ITEM, src/game/throwables.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct FRAGITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub half: [f64; 3],
    pub body: V3,
    pub band: V3,
    pub fuze: V3,
}
impl FRAGITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FRAGITEM {
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
            half: [0.0; 3],
            body: [0.0; 3],
            band: [0.0; 3],
            fuze: [0.0; 3],
        }
    }
}
impl Default for FRAGITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:65 RING_ITEM, src/game/throwables.ts:65)
#[derive(Clone, Copy, Debug)]
pub struct RINGITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub life: f64,
    pub half: [f64; 3],
    pub albedo: V3,
}
impl RINGITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RINGITEM {
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
            life: 0.0,
            half: [0.0; 3],
            albedo: [0.0; 3],
        }
    }
}
impl Default for RINGITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:71 SPONGE_ITEM, src/game/throwables.ts:71)
#[derive(Clone, Copy, Debug)]
pub struct SPONGEITEM {
    pub wide: f64,
    pub nose: crate::throwables::SPONGEITEMNose,
    pub body: crate::throwables::SPONGEITEMBody,
    pub band: crate::throwables::SPONGEITEMBand,
}
impl SPONGEITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPONGEITEM {
            wide: 0.0,
            nose: crate::throwables::SPONGEITEMNose::zeroed(),
            body: crate::throwables::SPONGEITEMBody::zeroed(),
            band: crate::throwables::SPONGEITEMBand::zeroed(),
        }
    }
}
impl Default for SPONGEITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:73 SPONGEITEM.nose, src/game/throwables.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct SPONGEITEMNose {
    pub z: f64,
    pub h: [f64; 3],
    pub albedo: V3,
}
impl SPONGEITEMNose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPONGEITEMNose {
            z: 0.0,
            h: [0.0; 3],
            albedo: [0.0; 3],
        }
    }
}
impl Default for SPONGEITEMNose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:74 SPONGEITEM.body, src/game/throwables.ts:74)
#[derive(Clone, Copy, Debug)]
pub struct SPONGEITEMBody {
    pub z: f64,
    pub h: [f64; 3],
    pub albedo: V3,
}
impl SPONGEITEMBody {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPONGEITEMBody {
            z: 0.0,
            h: [0.0; 3],
            albedo: [0.0; 3],
        }
    }
}
impl Default for SPONGEITEMBody { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:75 SPONGEITEM.band, src/game/throwables.ts:75)
#[derive(Clone, Copy, Debug)]
pub struct SPONGEITEMBand {
    pub z: f64,
    pub h: [f64; 3],
    pub albedo: V3,
}
impl SPONGEITEMBand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SPONGEITEMBand {
            z: 0.0,
            h: [0.0; 3],
            albedo: [0.0; 3],
        }
    }
}
impl Default for SPONGEITEMBand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:85 SHELL_ITEM, src/game/throwables.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct SHELLITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub life: f64,
    pub half: [f64; 3],
    pub body: V3,
    pub nose: V3,
}
impl SHELLITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SHELLITEM {
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
            life: 0.0,
            half: [0.0; 3],
            body: [0.0; 3],
            nose: [0.0; 3],
        }
    }
}
impl Default for SHELLITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:91 RIFLE_ITEM, src/game/throwables.ts:91)
#[derive(Clone, Copy, Debug)]
pub struct RIFLEITEM {
    pub radius: f64,
    pub restitution: f64,
    pub friction: f64,
    pub life: f64,
    pub prop_share: f64,
    pub thick: f64,
    pub receiver: crate::throwables::RIFLEITEMReceiver,
    pub fore: crate::throwables::RIFLEITEMFore,
    pub mag: crate::throwables::RIFLEITEMMag,
    pub stock: crate::throwables::RIFLEITEMStock,
    pub steel: V3,
    pub wood: V3,
}
impl RIFLEITEM {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RIFLEITEM {
            radius: 0.0,
            restitution: 0.0,
            friction: 0.0,
            life: 0.0,
            prop_share: 0.0,
            thick: 0.0,
            receiver: crate::throwables::RIFLEITEMReceiver::zeroed(),
            fore: crate::throwables::RIFLEITEMFore::zeroed(),
            mag: crate::throwables::RIFLEITEMMag::zeroed(),
            stock: crate::throwables::RIFLEITEMStock::zeroed(),
            steel: [0.0; 3],
            wood: [0.0; 3],
        }
    }
}
impl Default for RIFLEITEM { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:94 RIFLEITEM.receiver, src/game/throwables.ts:94)
#[derive(Clone, Copy, Debug)]
pub struct RIFLEITEMReceiver {
    pub z: f64,
    pub h: [f64; 3],
}
impl RIFLEITEMReceiver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RIFLEITEMReceiver {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for RIFLEITEMReceiver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:95 RIFLEITEM.fore, src/game/throwables.ts:95)
#[derive(Clone, Copy, Debug)]
pub struct RIFLEITEMFore {
    pub z: f64,
    pub h: [f64; 3],
}
impl RIFLEITEMFore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RIFLEITEMFore {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for RIFLEITEMFore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:96 RIFLEITEM.mag, src/game/throwables.ts:96)
#[derive(Clone, Copy, Debug)]
pub struct RIFLEITEMMag {
    pub z: f64,
    pub y: f64,
    pub h: [f64; 3],
    pub cant: f64,
}
impl RIFLEITEMMag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RIFLEITEMMag {
            z: 0.0,
            y: 0.0,
            h: [0.0; 3],
            cant: 0.0,
        }
    }
}
impl Default for RIFLEITEMMag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:97 RIFLEITEM.stock, src/game/throwables.ts:97)
#[derive(Clone, Copy, Debug)]
pub struct RIFLEITEMStock {
    pub z: f64,
    pub h: [f64; 3],
}
impl RIFLEITEMStock {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RIFLEITEMStock {
            z: 0.0,
            h: [0.0; 3],
        }
    }
}
impl Default for RIFLEITEMStock { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:218 ray ->, src/game/throwables.ts:218)
#[derive(Clone, Copy, Debug)]
pub struct ThrowablesRay {
    pub t: f64,
    pub n: V3,
    pub prop: Option<PropId>,
}
impl ThrowablesRay {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowablesRay {
            t: 0.0,
            n: [0.0; 3],
            prop: None,
        }
    }
}
impl Default for ThrowablesRay { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/throwables.ts:238 spawn(opts), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Copy, Debug)]
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
impl ThrowablesSpawnOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ThrowablesSpawnOpts {
            kind: None,
            pos: None,
            vel: None,
            radius: None,
            restitution: None,
            friction: None,
            yaw: None,
            spin: None,
            settled: None,
            age: None,
            fuse: None,
            fired: None,
            life: None,
            tumble: None,
            tumble_rate: None,
            emit_t: None,
            fill: None,
            bounces: None,
            heading: None,
            who: None,
            on: None,
            on_pose: None,
            serial: None,
            side: None,
            clatters: None,
            carried: None,
        }
    }
}
impl Default for ThrowablesSpawnOpts { fn default() -> Self { Self::zeroed() } }

/// (src/game/throwables.ts:40 SC20K_ITEM: a table nothing writes)
pub fn sc20_k_item() -> crate::throwables::SC20KITEM { crate::throwables::SC20KITEM { hull: crate::throwables::SC20KITEMHull { z: -0.06, h: [0.024, 0.04, 0.215] }, fore: crate::throwables::SC20KITEMFore { z: 0.3, h: [0.02, 0.024, 0.155] }, mag: crate::throwables::SC20KITEMMag { z: -0.115, y: -0.1, h: [0.012, 0.06, 0.016] }, optic: crate::throwables::SC20KITEMOptic { z: 0.02, y: 0.06, h: [0.017, 0.02, 0.06] }, thick: 0.024, polymer: [0.045, 0.05, 0.046], metal: [0.026, 0.026, 0.03] } }

/// (src/game/throwables.ts:51 M4_ITEM: a table nothing writes)
pub fn m4_item() -> crate::throwables::M4ITEM { crate::throwables::M4ITEM { hull: crate::throwables::M4ITEMHull { z: -0.09, h: [0.017, 0.034, 0.2] }, fore: crate::throwables::M4ITEMFore { z: 0.285, h: [0.023, 0.021, 0.17] }, mag: crate::throwables::M4ITEMMag { z: 0.085, y: -0.095, h: [0.011, 0.065, 0.028] }, optic: crate::throwables::M4ITEMOptic { z: 0.035, y: 0.052, h: [0.016, 0.02, 0.035] }, polymer: [0.03, 0.03, 0.033], metal: [0.03, 0.03, 0.033], fore_col: [0.2, 0.155, 0.1] } }

/// (src/game/throwables.ts:59 CAN_ITEM: a table nothing writes)
pub fn can_item() -> crate::throwables::CANITEM { crate::throwables::CANITEM { radius: 0.055, restitution: 0.38, friction: 0.35 } }

/// (src/game/throwables.ts:63 FRAG_ITEM: a table nothing writes)
pub fn frag_item() -> crate::throwables::FRAGITEM { crate::throwables::FRAGITEM { radius: 0.04, restitution: 0.3, friction: 0.45, half: [0.032, 0.032, 0.038], body: [0.13, 0.17, 0.08], band: [0.75, 0.62, 0.1], fuze: [0.3, 0.3, 0.28] } }

/// (src/game/throwables.ts:65 RING_ITEM: a table nothing writes)
pub fn ring_item() -> crate::throwables::RINGITEM { crate::throwables::RINGITEM { radius: 0.03, restitution: 0.3, friction: 0.6, life: 60.0, half: [0.03, 0.006, 0.007], albedo: [0.42, 0.4, 0.36] } }

/// (src/game/throwables.ts:71 SPONGE_ITEM: a table nothing writes)
pub fn sponge_item() -> crate::throwables::SPONGEITEM { crate::throwables::SPONGEITEM { wide: 0.023, nose: crate::throwables::SPONGEITEMNose { z: 0.019, h: [0.022, 0.022, 0.019], albedo: [0.07, 0.24, 0.62] }, body: crate::throwables::SPONGEITEMBody { z: -0.014, h: [0.021, 0.021, 0.014], albedo: [0.05, 0.05, 0.055] }, band: crate::throwables::SPONGEITEMBand { z: -0.033, h: [0.023, 0.023, 0.005], albedo: [0.3, 0.3, 0.31] } } }

/// (src/game/throwables.ts:85 SHELL_ITEM: a table nothing writes)
pub fn shell_item() -> crate::throwables::SHELLITEM { crate::throwables::SHELLITEM { radius: 0.035, restitution: 0.22, friction: 0.5, life: 120.0, half: [0.02, 0.02, 0.05], body: [0.42, 0.43, 0.4], nose: [0.72, 0.55, 0.12] } }

/// (src/game/throwables.ts:91 RIFLE_ITEM: a table nothing writes)
pub fn rifle_item() -> crate::throwables::RIFLEITEM { crate::throwables::RIFLEITEM { radius: 0.06, restitution: 0.18, friction: 0.55, life: 240.0, prop_share: 0.3, thick: 0.02, receiver: crate::throwables::RIFLEITEMReceiver { z: -0.02, h: [0.018, 0.03, 0.13] }, fore: crate::throwables::RIFLEITEMFore { z: 0.24, h: [0.02, 0.022, 0.13] }, mag: crate::throwables::RIFLEITEMMag { z: 0.06, y: -0.075, h: [0.013, 0.05, 0.03], cant: 0.35 }, stock: crate::throwables::RIFLEITEMStock { z: -0.27, h: [0.008, 0.026, 0.12] }, steel: [0.05, 0.05, 0.055], wood: [0.2, 0.085, 0.05] } }

/// (src/game/throwables.ts:182 GRAVITY)
pub const GRAVITY: f64 = 9.81;
