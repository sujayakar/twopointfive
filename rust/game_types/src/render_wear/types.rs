//! The types of `render_wear`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct WearMap {
    pub data: Vec<u8>,
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WAY {
    pub patrol: f64,
    pub team: f64,
    pub doors: f64,
    pub hint: f64,
}

/// (src/render/wear.ts:24 WEAR_RES)
pub const WEAR_RES: f64 = 8.0;

/// (src/render/wear.ts:25 LANE_SIGMA)
pub const LANE_SIGMA: f64 = 0.34;

/// (src/render/wear.ts:26 DOOR_SIGMA)
pub const DOOR_SIGMA: f64 = 0.55;

/// (src/render/wear.ts:27 DOOR_AMP)
pub const DOOR_AMP: f64 = 1.4;

/// (src/render/wear.ts:28 TRAFFIC_K)
pub const TRAFFIC_K: f64 = 1.1;

/// (src/render/wear.ts:29 GRIME_REACH)
pub const GRIME_REACH: f64 = 0.16;

/// (src/render/wear.ts:31 WAY: a table nothing writes)
pub fn way() -> crate::render_wear::WAY { crate::render_wear::WAY { patrol: 1.0, team: 0.45, doors: 0.3, hint: 1.0 } }

/// (src/render/wear.ts:32 RUN_LO)
pub const RUN_LO: f64 = 1.25;

/// (src/render/wear.ts:32 RUN_HI)
pub const RUN_HI: f64 = 2.5;

/// (src/render/wear.ts:33 RUN_MEET)
pub const RUN_MEET: f64 = 0.03;

/// (src/render/wear.ts:34 RUN_STEP)
pub const RUN_STEP: f64 = 0.12;

/// (src/render/wear.ts:35 RUN_TURN)
pub const RUN_TURN: f64 = 0.9986;
