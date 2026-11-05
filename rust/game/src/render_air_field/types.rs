//! The types of `render_air_field`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: AirField, src/render/airField.ts:63)
#[derive(Clone, Debug)]
pub struct AirField {
    pub w: f64,
    pub h: f64,
    pub view: crate::Opaque,
    pub cells: crate::Opaque,
    pub ready: bool,
    pub when_ready: crate::Opaque,
    pub device: crate::Opaque,
    pub out: crate::Opaque,
    pub cell_tex: crate::Opaque,
    pub bufs: Vec<crate::Opaque>,
    pub ubo: crate::Opaque,
    pub pipes: Option<JsMap<crate::render_air_field::AirFieldPipesKey, crate::Opaque>>,
    pub bind: crate::Opaque,
    pub gx: f64,
    pub gy: f64,
    pub dead: bool,
    pub last_wind: [f64; 2],
    pub scale: f64,
    pub clock: f64,
    pub t0: f64,
}
impl AirField {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        AirField {
            w: 0.0,
            h: 0.0,
            view: crate::Opaque,
            cells: crate::Opaque,
            ready: false,
            when_ready: crate::Opaque,
            device: crate::Opaque,
            out: crate::Opaque,
            cell_tex: crate::Opaque,
            bufs: Vec::new(),
            ubo: crate::Opaque,
            pipes: None,
            bind: crate::Opaque,
            gx: 0.0,
            gy: 0.0,
            dead: false,
            last_wind: [0.0, 0.0],
            scale: -1.0,
            clock: 0.0,
            t0: f64::NAN,
        }
    }
}
impl Default for AirField { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AirFieldPipesKey {
    Curl,
    Advect,
    Divergence,
    PressureRed,
    PressureBlack,
    Project,
}
impl AirFieldPipesKey {
    pub const fn as_str(self) -> &'static str {
        match self {
            AirFieldPipesKey::Curl => "curl",
            AirFieldPipesKey::Advect => "advect",
            AirFieldPipesKey::Divergence => "divergence",
            AirFieldPipesKey::PressureRed => "pressureRed",
            AirFieldPipesKey::PressureBlack => "pressureBlack",
            AirFieldPipesKey::Project => "project",
        }
    }
}

/// (src/render/airField.ts:25 AIR_RES)
pub const AIR_RES: f64 = 5.0;

/// (src/render/airField.ts:27 AIR_SRC_MAX)
pub const AIR_SRC_MAX: f64 = 16.0;

/// (src/render/airField.ts:52 U_HEAD)
pub const U_HEAD: f64 = 28.0;

/// (src/render/airField.ts:52 U_WORDS)
pub const U_WORDS: f64 = 156.0;

/// (src/render/airField.ts:53 KERNELS: a table nothing writes)
pub fn kernels() -> [String; 6] { [String::from("curl"), String::from("advect"), String::from("divergence"), String::from("pressureRed"), String::from("pressureBlack"), String::from("project")] }
