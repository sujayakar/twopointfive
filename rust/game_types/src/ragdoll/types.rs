//! The types of `ragdoll`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct RagdollWorld {
}

#[derive(Clone, Debug, ToBytes)]
pub struct Ragdoll {
    pub x: Vec<f64>,
    pub px: Vec<f64>,
    pub w: Vec<f64>,
    pub pin_t: Vec<f64>,
    pub pinned: Vec<u8>,
    pub any_pin: bool,
    pub ride_v: Vec<f64>,
    pub ride_owed: bool,
    pub pen: Vec<f64>,
    pub cn: Vec<f64>,
    pub stick_len: Vec<f64>,
    pub r0: Vec<f64>,
    pub trunk_mass: f64,
    pub rot: Quat,
    pub r: Vec<f64>,
    pub centre: V3,
    pub prev_centre: V3,
    pub a: Vec<f64>,
    pub fwd0: V3,
    pub up0: V3,
    pub hip_axis0: V3,
    pub near: Vec<i32>,
    pub near_cs: Vec<f64>,
    pub n_near: f64,
    pub dyn_: Vec<i32>,
    pub dyn_cs: Vec<f64>,
    pub n_dyn: f64,
    pub near_b: Vec<f64>,
    pub dyn_b: Vec<f64>,
    pub exit_l: Vec<f64>,
    pub exit_p: Vec<f64>,
    pub snap: Vec<f64>,
    pub snap_t: f64,
    pub sleeping: bool,
    pub awake: f64,
    pub acc: f64,
    pub strain: f64,
    pub impact_v: f64,
    pub since_impact: f64,
    pub world: bool,
}
impl Ragdoll {
    /// what a new one holds
    pub fn blank() -> Self {
        Ragdoll {
            x: alloc::vec![0.0; 48],
            px: alloc::vec![0.0; 48],
            w: alloc::vec![0.0; 16],
            pin_t: alloc::vec![0.0; 48],
            pinned: alloc::vec![0; 16],
            any_pin: false,
            ride_v: alloc::vec![0.0; 5],
            ride_owed: false,
            pen: alloc::vec![0.0; 16],
            cn: alloc::vec![0.0; 48],
            stick_len: alloc::vec![0.0; 9],
            r0: alloc::vec![0.0; 21],
            trunk_mass: 0.0,
            rot: [0.0, 0.0, 0.0, 1.0],
            r: alloc::vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            centre: [0.0, 0.0, 0.0],
            prev_centre: [0.0, 0.0, 0.0],
            a: alloc::vec![0.0; 9],
            fwd0: [0.0; 3],
            up0: [0.0; 3],
            hip_axis0: [0.0; 3],
            near: alloc::vec![0; 384],
            near_cs: alloc::vec![0.0; 768],
            n_near: 0.0,
            dyn_: alloc::vec![0; 32],
            dyn_cs: alloc::vec![0.0; 64],
            n_dyn: 0.0,
            near_b: alloc::vec![0.0; 2304],
            dyn_b: alloc::vec![0.0; 192],
            exit_l: alloc::vec![0.0; 3],
            exit_p: alloc::vec![0.0; 3],
            snap: alloc::vec![0.0; 48],
            snap_t: 0.0,
            sleeping: false,
            awake: 0.0,
            acc: 0.0,
            strain: 0.0,
            impact_v: 0.0,
            since_impact: 1.0,
            world: false,
        }
    }
}
impl Default for Ragdoll { fn default() -> Self { Self::blank() } }

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

pub fn radius() -> Vec<f64> { alloc::vec![0.12, 0.13, 0.07, 0.11, 0.07, 0.07, 0.05, 0.05, 0.05, 0.05, 0.09, 0.09, 0.065, 0.065, 0.06, 0.06] }

pub fn mass() -> Vec<f64> { alloc::vec![12.0, 15.0, 3.0, 5.0, 3.0, 3.0, 2.0, 2.0, 1.0, 1.0, 6.0, 6.0, 4.0, 4.0, 2.0, 2.0] }

pub fn trunk() -> Vec<f64> { alloc::vec![0.0, 1.0, 2.0, 4.0, 5.0, 10.0, 11.0] }

pub fn sticks() -> Vec<[f64; 2]> { alloc::vec![[2.0, 3.0], [4.0, 6.0], [6.0, 8.0], [5.0, 7.0], [7.0, 9.0], [10.0, 12.0], [12.0, 14.0], [11.0, 13.0], [13.0, 15.0]] }

pub fn leg_sticks() -> Vec<f64> { alloc::vec![5.0, 6.0, 7.0, 8.0] }

pub fn mindist() -> Vec<[f64; 3]> { alloc::vec![[4.0, 8.0, 0.16], [5.0, 9.0, 0.16], [10.0, 14.0, 0.3], [11.0, 15.0, 0.3]] }

pub fn apart() -> Vec<[f64; 3]> { alloc::vec![[12.0, 13.0, 0.9], [14.0, 15.0, 0.9], [12.0, 15.0, 0.9], [13.0, 14.0, 0.9], [6.0, 0.0, 0.6], [6.0, 1.0, 0.6], [7.0, 0.0, 0.6], [7.0, 1.0, 0.6], [8.0, 0.0, 0.6], [8.0, 1.0, 0.6], [8.0, 2.0, 0.6], [8.0, 3.0, 0.7], [9.0, 0.0, 0.6], [9.0, 1.0, 0.6], [9.0, 2.0, 0.6], [9.0, 3.0, 0.7], [8.0, 9.0, 0.8], [6.0, 7.0, 0.8], [4.0, 3.0, 0.9], [5.0, 3.0, 0.9]] }

pub fn knees() -> Vec<[f64; 3]> { alloc::vec![[10.0, 12.0, 14.0], [11.0, 13.0, 15.0]] }

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

