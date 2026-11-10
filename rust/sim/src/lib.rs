//! THE SIMULATION IN RUST — the game's update, moved here a system at a time and built to WebAssembly for the page, the game server and the headless tools
//! (docs/agents/rust.md says how a system moves, and what holds it to the TypeScript one it replaces).
//!
//! What every line in this crate owes the game:
//!   · one arithmetic. Doubles, and only + − × ÷ √ and what the standard makes exact of them (floor, trunc, abs, comparisons, the bits). Nothing that a
//!     compiler or a machine may round as it likes: no `f64::sin` and friends (rust/maths has the game's own), no `mul_add`, no fast-math, no `f32` where the
//!     TypeScript holds a double.
//!   · one order. Nothing whose order is not written down: no `HashMap` / `HashSet` iteration, no pointer values as keys, no threads.
//!   · no world outside. No clock, no random source of its own, no files.
//!   · state in one place. What a game keeps here from one call to the next is in its block (state.rs: indices, never addresses); every other `static mut`
//!     is scratch of one call or a copy the host proves.
//!
//! WHO HOLDS IT. The compiler: the crate is `no_std`, so no line of it can name a hash map, a thread, a clock or a file (`std::…` does not resolve here).
//! tools/qa/rust-rules-check.ts: the float methods the compiler cannot keep out once any crate links the standard
//! library (`.sin()`, `.powf()`, `.mul_add()` …), an `f32`, and the SIMD that rounds as the machine likes. The reader: the order of the arithmetic.
//! √, floor and trunc, which the core library lacks, come from rust/prim.
#![no_std]
extern crate alloc;

use game_types::boxworld::World;
use game_types::knobs::Knobs;
pub mod air;
pub mod bricks;
pub mod clips;
pub mod collide;
pub mod falls;
pub mod fog;
pub mod glass;
pub mod glow;
pub mod hear;
pub mod roles;
pub mod layers;
pub mod light;
pub mod lights;
pub mod lines;
pub use ::maths;
pub mod men;
pub mod nav;
pub mod panes;
pub mod pattern;
pub mod pen;
pub mod poses;
pub mod sight;
pub mod single;
pub mod skeleton;
pub mod snow;
pub mod state;
pub mod weather;
pub mod wind;
pub mod world;

// ---- the game's state in the module (state.rs: a block a game, bound to a slot by its host)

// ---- the senses (sight.rs, light.rs, bricks.rs, hear.rs, nav.rs): what a man sees, how lit he is, what the smoke hides, how a sound reaches him
// ---- the game's own smoke on the coarse grid (bricks.rs): its state is a part of the game's block (state.rs), so a call names the block's slot `g`;
// one that reads the tables made from the state names the block's `stamp` too (`state::stamp`), and a step the box world `w` it is
// taken over and that copy's number `copy`

// ---- WHAT A PANE OF GLASS IS TO PLAY (panes.rs, pen.rs): the state in the game's block, the rule, a round's toll. A rule's call names its game, its world, the leaves
// and the time once (`panes_ctx`), and what it let go is read from `panes_out_*` and cleared before the next pane's.
/// glass.ts boxSeed of a box of the world; pointSeed
pub fn panes_box_seed(cx0: f64, cy: f64, cz: f64, hx: f64, hy: f64, hz: f64) -> u32 {
    let mut b = [0.0; world::REC];
    b[world::R_C] = cx0;
    b[world::R_C + 1] = cy;
    b[world::R_C + 2] = cz;
    b[world::R_H] = hx;
    b[world::R_H + 1] = hy;
    b[world::R_H + 2] = hz;
    panes::box_seed(&b)
}
/// what a game holds of what levelfit.ts counts: 0 window's bars gone · 1 clumps of a sheet
pub fn panes_cap(i: u32) -> u32 {
    (if i == 0 { panes::MAX_BARS } else { pattern::MAX_CLUMPS }) as u32
}

// ---- the dust (air.rs)
/// the dust's table of the block in slot `g`, whose stamp is `stamp`, and the slot's tables made from it and the roofs (air.rs plan)
fn air_of(g: usize, stamp: f64, roofs: &[[f64; 4]]) -> (&'static mut air::Air, &'static mut air::Plan) {
    let wx = weather::of(g);
    let p = air::plan(g, stamp, &wx.air, roofs);
    (&mut wx.air, p)
}
/// the dust's field stood up afresh: every cell the open air's, no opening asked, no list
pub fn air_clear(g: usize, stamp: f64, roofs: &[[f64; 4]]) {
    let (a, p) = air_of(g, stamp, roofs);
    a.clear(p)
}
/// the field begun: its switch, nothing owed, no opening known
pub fn air_begin(g: usize, stamp: f64, on: bool, roofs: &[[f64; 4]]) {
    let (a, p) = air_of(g, stamp, roofs);
    a.begin(p, on)
}
/// the field seeded under the roofs in force at the leak given
pub fn air_seed(g: usize, stamp: f64, leak: f64, roofs: &[[f64; 4]]) {
    let (a, p) = air_of(g, stamp, roofs);
    a.seed(p, leak, |x, z| weather::under_any(roofs, x, z))
}
/// …every opening asked and run on until it stands still
pub fn air_settle(g: usize, stamp: f64, w: &World, roofs: &[[f64; 4]], tune: &Knobs) -> air::Said {
    let mut said = air::Said { moved: 0.0, steps: 0.0, probes: 0.0, open: 0.0 };
    let (a, p) = air_of(g, stamp, roofs);
    a.settle(p, w, &mut said, tune);
    said
}
/// one tick of it
pub fn air_update(g: usize, stamp: f64, w: &World, dt: f64, roofs: &[[f64; 4]], tune: &Knobs) -> air::Said {
    let mut said = air::Said { moved: 0.0, steps: 0.0, probes: 0.0, open: 0.0 };
    let (a, p) = air_of(g, stamp, roofs);
    a.update(p, w, dt, &mut said, tune);
    said
}
