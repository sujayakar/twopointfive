//! The types of `render_reach_keep`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: ReachKeep, src/render/reachKeep.ts:61)
#[derive(Clone, Debug)]
pub struct ReachKeep {
    pub world: crate::render_reach_keep::ReachKeepWorld,
    pub lights: crate::render_reach_keep::ReachKeepLights,
    pub point: JsMap<LightId, Vec<u32>>,
    pub dirs: JsMap<LightId, Vec<u32>>,
    pub made: JsMap<LightId, crate::render_reach_keep::Made>,
    pub solid: Option<Vec<u8>>,
    pub solid_boot: Option<Vec<u8>>,
    pub boot_mask: JsMap<LightId, crate::render_reach_keep::ReachKeepBootMaskValue>,
    pub doors: Vec<crate::render_reach::ReachDoor>,
    pub interior: Option<Vec<u8>>,
    pub masked: Vec<LightId>,
    pub queue: Vec<crate::render_reach_keep::Asked>,
    pub held: Vec<[f64; 4]>,
    pub hold: f64,
    pub now: Vec<[f64; 4]>,
    pub stale: bool,
    pub put_backs: f64,
    pub reading: Option<crate::render_reach_keep::ReachKeepReading>,
}
impl ReachKeep {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        ReachKeep {
            world: crate::render_reach_keep::ReachKeepWorld::zeroed(),
            lights: crate::render_reach_keep::ReachKeepLights::zeroed(),
            point: JsMap::new(),
            dirs: JsMap::new(),
            made: JsMap::new(),
            solid: None,
            solid_boot: None,
            boot_mask: JsMap::new(),
            doors: Vec::new(),
            interior: None,
            masked: Vec::new(),
            queue: Vec::new(),
            held: Vec::new(),
            hold: 0.0,
            now: Vec::new(),
            stale: false,
            put_backs: 0.0,
            reading: None,
        }
    }
}
impl Default for ReachKeep { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/render/reachKeep.ts:62 ReachKeep.world, src/render/reachKeep.ts:62)
#[derive(Clone, Debug)]
pub struct ReachKeepWorld {
    pub statics: Vec<BoxId>,
}
impl ReachKeepWorld {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepWorld {
            statics: Vec::new(),
        }
    }
}
impl Default for ReachKeepWorld { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:62 ReachKeep.lights, src/render/reachKeep.ts:62)
#[derive(Clone, Debug)]
pub struct ReachKeepLights {
    pub lights: Vec<LightId>,
}
impl ReachKeepLights {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepLights {
            lights: Vec::new(),
        }
    }
}
impl Default for ReachKeepLights { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Made, src/render/reachKeep.ts:30)
#[derive(Clone, Debug)]
pub struct Made {
    pub r: f64,
    pub d: V3,
    pub on: Option<Vec<u8>>,
}
impl Made {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Made {
            r: 0.0,
            d: [0.0; 3],
            on: None,
        }
    }
}
impl Default for Made { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:68 ReachKeep.bootMask, src/render/reachKeep.ts:68)
#[derive(Clone, Debug)]
pub struct ReachKeepBootMaskValue {
    pub m: Vec<u32>,
    pub made: Option<crate::render_reach_keep::Made>,
}
impl ReachKeepBootMaskValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepBootMaskValue {
            m: Vec::new(),
            made: None,
        }
    }
}
impl Default for ReachKeepBootMaskValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Asked, src/render/reachKeep.ts:28)
#[derive(Clone, Debug)]
pub struct Asked {
    pub l: LightId,
    pub rects: Vec<[f64; 4]>,
    pub boxes: Vec<[f64; 4]>,
    pub job: Option<crate::render_reach::ReachJob>,
    pub dir_job: Option<crate::render_reach_keep::AskedDirJob>,
    pub restarts: f64,
}
impl Asked {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Asked {
            l: LightId(0),
            rects: Vec::new(),
            boxes: Vec::new(),
            job: None,
            dir_job: None,
            restarts: 0.0,
        }
    }
}
impl Default for Asked { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:28 Asked.dirJob, src/render/reachKeep.ts:28)
#[derive(Clone, Debug)]
pub struct AskedDirJob {
    pub it: crate::Opaque,
    pub prev: Option<Vec<u32>>,
}
impl AskedDirJob {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AskedDirJob {
            it: crate::Opaque,
            prev: None,
        }
    }
}
impl Default for AskedDirJob { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:76 ReachKeep.reading, src/render/reachKeep.ts:76)
#[derive(Clone, Debug)]
pub struct ReachKeepReading {
    pub rects: Vec<[f64; 4]>,
    pub solid: Vec<u8>,
    pub it: crate::Opaque,
}
impl ReachKeepReading {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepReading {
            rects: Vec::new(),
            solid: Vec::new(),
            it: crate::Opaque,
        }
    }
}
impl Default for ReachKeepReading { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:80 constructor(world), src/render/reachKeep.ts:80)
#[derive(Clone, Debug)]
pub struct ReachKeepNewWorld {
    pub statics: Vec<BoxId>,
}
impl ReachKeepNewWorld {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepNewWorld {
            statics: Vec::new(),
        }
    }
}
impl Default for ReachKeepNewWorld { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:80 constructor(lights), src/render/reachKeep.ts:80)
#[derive(Clone, Debug)]
pub struct ReachKeepNewLights {
    pub lights: Vec<LightId>,
}
impl ReachKeepNewLights {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepNewLights {
            lights: Vec::new(),
        }
    }
}
impl Default for ReachKeepNewLights { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/reachKeep.ts:307 scene ->, src/render/reachKeep.ts:307)
#[derive(Clone, Copy, Debug)]
pub struct ReachKeepScene {
    pub blocked: crate::FnField,
    pub free: crate::FnField,
}
impl ReachKeepScene {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ReachKeepScene {
            blocked: crate::FnField,
            free: crate::FnField,
        }
    }
}
impl Default for ReachKeepScene { fn default() -> Self { Self::zeroed() } }
