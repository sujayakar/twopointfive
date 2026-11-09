//! The types of `launcher`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Ring {
    pub pos: V3,
    pub vel: V3,
    pub age: f64,
    pub shooter: CharId,
    pub spin: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Ballistics {
    pub speed: f64,
    pub gravity: f64,
    pub drag: f64,
    pub radius: f64,
    pub life: f64,
    pub substeps: f64,
    pub solve_iters: f64,
}

/// (src/game/launcher.ts:49 STAGGER_LINES: a table nothing writes)
pub fn stagger_lines_table() -> Vec<String> { alloc::vec![String::from("agh — what the—"), String::from("AH! son of a—"), String::from("ow! what— who's there?!"), String::from("nngh— something hit me!"), String::from("jesus— my ribs—")] }
