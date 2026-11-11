//! The types of `air_dust`.
use crate::{Blank, bytes::ToBytes};
use alloc::{sync::Arc, vec::Vec};

/// THE WEATHER'S DUST IN THE ROOMS: the share of the open air's density at each half metre of the plan (rust/sim air.rs), and the ways between them.
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct AirDust {
    /// the roofs or the weather changed, and the field is to be built
    #[is(true)] pub dirty: bool,
    /// the men's eyes read the field this run
    pub on: bool,
    pub leak: f64,
    pub acc: f64,
    /// the re-check's place in the plan's edges
    pub cur: usize,
    /// a cell's share (none at all: no weather has had dust in it, and it is the open air's everywhere)
    pub share: Vec<f64>,
    /// by edge: no static stands across it
    pub open: Vec<u8>,
    /// (made from the roofs and the leaves: in no bytes)
    #[bytes(skip)]
    pub plan: AirPlan,
}
/// WHAT THE ROOFS AND THE DOOR LEAVES MAKE of the plan.
#[derive(Clone, Debug, Default)]
pub struct AirPlan {
    pub under: Arc<Roofed>,
    /// by edge: how many leaves stand across it, and the leaves as they stood then (None: they are to be read)
    pub leaf: Vec<u8>,
    pub read: Option<i32>,
}
/// the roofed cells, and the edges with a roofed cell at either end (cell × 2 + 0 to +x, 1 to +z): in the order a step walks them and the re-check goes round them
#[derive(Clone, Debug, Default)]
pub struct Roofed { pub cells: Vec<u32>, pub edges: Vec<u32> }
