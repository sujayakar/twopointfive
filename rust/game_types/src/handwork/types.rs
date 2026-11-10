//! The types of `handwork`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct SamWork {
    pub kind: crate::handwork::SamWorkKind,
    pub p: V3,
    pub n: V3,
    pub at: Option<V3>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum SamWorkKind {
    #[default] Breaker,
    Door,
    Drive,
    Wall,
    Switch,
}

#[derive(Clone, Debug, Default)]
pub struct WorkAtMore {
    pub keys: Option<Vec<[f64; 2]>>,
    pub hand: Option<f64>,
    pub dir: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct PadHandsPad {
    pub def: crate::handwork::PadHandsPadDef,
    pub n: V3,
}

#[derive(Clone, Debug, Default)]
pub struct PadHandsPadDef {
    pub pos: V3,
    pub code: String,
    pub yaw: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BreakerFace {
    pub p: V3,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RackFace {
    pub p: V3,
    pub n: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TeamWorkHandH {
    pub p: V3,
    pub n: V3,
    pub w: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TeamWorkHandKind {
    Take,
    Lever,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LeverStand {
    pub yaw: f64,
    pub speed: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwapFeedHeld {
    Torch,
    Weapon,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwapHeldHeld {
    Torch,
    Weapon,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SwapHeld {
    Torch,
    Weapon,
}

pub const WORK_TURN: f64 = 10.0;

pub fn key_at() -> JsMap<String, [f64; 2]> { { let mut m = JsMap::new(); for (r, row) in ["123", "456", "789", "*0#"].iter().enumerate() { for (c, ch) in row.chars().enumerate() { m.set(String::from(ch), [(c as f64 - 1.0) * 0.02, -0.015 + (1.5 - r as f64) * 0.0175]); } } m } }
