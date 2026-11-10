//! The types of `ragdoll`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct RagdollWorld {
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Ragdoll {
    #[is(alloc::vec![0.0; 48])] pub x: Vec<f64>,
    #[is(alloc::vec![0.0; 48])] pub px: Vec<f64>,
    #[is(alloc::vec![0.0; 16])] pub w: Vec<f64>,
    #[is(alloc::vec![0.0; 48])] pub pin_t: Vec<f64>,
    #[is(alloc::vec![0; 16])] pub pinned: Vec<u8>,
    pub any_pin: bool,
    #[is(alloc::vec![0.0; 5])] pub ride_v: Vec<f64>,
    pub ride_owed: bool,
    #[is(alloc::vec![0.0; 16])] pub pen: Vec<f64>,
    #[is(alloc::vec![0.0; 48])] pub cn: Vec<f64>,
    #[is(alloc::vec![0.0; 9])] pub stick_len: Vec<f64>,
    #[is(alloc::vec![0.0; 21])] pub r0: Vec<f64>,
    pub trunk_mass: f64,
    #[is([0.0, 0.0, 0.0, 1.0])] pub rot: Quat,
    #[is(alloc::vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])] pub r: Vec<f64>,
    pub centre: V3,
    pub prev_centre: V3,
    #[is(alloc::vec![0.0; 9])] pub a: Vec<f64>,
    pub fwd0: V3,
    pub up0: V3,
    pub hip_axis0: V3,
    #[is(alloc::vec![0; 384])] pub near: Vec<i32>,
    #[is(alloc::vec![0.0; 768])] pub near_cs: Vec<f64>,
    pub n_near: f64,
    #[is(alloc::vec![0; 32])] pub dyn_: Vec<i32>,
    #[is(alloc::vec![0.0; 64])] pub dyn_cs: Vec<f64>,
    pub n_dyn: f64,
    #[is(alloc::vec![0.0; 2304])] pub near_b: Vec<f64>,
    #[is(alloc::vec![0.0; 192])] pub dyn_b: Vec<f64>,
    #[is(alloc::vec![0.0; 3])] pub exit_l: Vec<f64>,
    #[is(alloc::vec![0.0; 3])] pub exit_p: Vec<f64>,
    #[is(alloc::vec![0.0; 48])] pub snap: Vec<f64>,
    pub snap_t: f64,
    pub sleeping: bool,
    pub awake: f64,
    pub acc: f64,
    pub strain: f64,
    pub impact_v: f64,
    #[is(1.0)] pub since_impact: f64,
    pub world: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RAG {
    pub pelvis: f64,
    pub spine: f64,
    pub neck: f64,
    pub head: f64,
    pub sho_l: f64,
    pub sho_r: f64,
    pub hand_l: f64,
    pub hand_r: f64,
    pub hip_l: f64,
    pub hip_r: f64,
    pub knee_l: f64,
    pub knee_r: f64,
    pub ank_l: f64,
    pub ank_r: f64,
}

pub fn rag() -> crate::ragdoll::RAG { crate::ragdoll::RAG { pelvis: 0.0, spine: 1.0, neck: 2.0, head: 3.0, sho_l: 4.0, sho_r: 5.0, hand_l: 8.0, hand_r: 9.0, hip_l: 10.0, hip_r: 11.0, knee_l: 12.0, knee_r: 13.0, ank_l: 14.0, ank_r: 15.0 } }

pub const RAG_COUNT: f64 = 16.0;









pub const HIP_CONE_COS: f64 = 0.25881904510252074;

pub const H: f64 = 0.005555555555555556;

pub const MAX_SUBSTEPS: f64 = 9.0;

pub const ITERS: f64 = 8.0;

pub const GRAV: f64 = 9.8;

pub const DAMP: f64 = 0.9955736713286294;

pub const DAMP_PINNED: f64 = 0.9682421997846513;

pub const DAMP_SETTLE: f64 = 0.9785010088386569;

pub const SETTLE_V: f64 = 0.25;

pub const MAX_STEP: f64 = 0.045;

pub const PIN_STEP: f64 = 0.013333333333333334;

pub const MU: f64 = 0.7;

pub const SLEEP_WINDOW: f64 = 0.35;

pub const SLEEP_DISP: f64 = 0.012;

pub const MAX_AWAKE: f64 = 12.0;

pub const Y_MIN: f64 = 0.03;

