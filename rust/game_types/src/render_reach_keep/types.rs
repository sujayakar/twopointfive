//! The types of `render_reach_keep`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

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
    /// what a new one holds
    pub fn blank() -> Self {
        ReachKeep {
            world: crate::render_reach_keep::ReachKeepWorld::default(),
            lights: crate::render_reach_keep::ReachKeepLights::default(),
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

#[derive(Clone, Debug, Default)]
pub struct ReachKeepWorld {
    pub statics: Vec<BoxId>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachKeepLights {
    pub lights: Vec<LightId>,
}

#[derive(Clone, Debug, Default)]
pub struct Made {
    pub r: f64,
    pub d: V3,
    pub on: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachKeepBootMaskValue {
    pub m: Vec<u32>,
    pub made: Option<crate::render_reach_keep::Made>,
}

#[derive(Clone, Debug, Default)]
pub struct Asked {
    pub l: LightId,
    pub rects: Vec<[f64; 4]>,
    pub boxes: Vec<[f64; 4]>,
    pub job: Option<crate::render_reach::ReachJob>,
    pub dir_job: Option<crate::render_reach_keep::AskedDirJob>,
    pub restarts: f64,
}

#[derive(Clone, Debug, Default)]
pub struct AskedDirJob {
    pub it: crate::Opaque,
    pub prev: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachKeepReading {
    pub rects: Vec<[f64; 4]>,
    pub solid: Vec<u8>,
    pub it: crate::Opaque,
}

#[derive(Clone, Debug, Default)]
pub struct ReachKeepNewWorld {
    pub statics: Vec<BoxId>,
}

#[derive(Clone, Debug, Default)]
pub struct ReachKeepNewLights {
    pub lights: Vec<LightId>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ReachKeepScene {
    pub blocked: crate::FnField,
    pub free: crate::FnField,
}
