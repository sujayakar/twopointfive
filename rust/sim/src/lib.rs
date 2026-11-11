//! THE SIMULATION'S LOOPS: the questions of the box world (lines, rays, circles, a way across the floor), the senses, the smoke, the glass and the animator's layers, over
//! flat state in one block a game (state.rs). THE HOST, in these files, is whoever calls this crate: the rules (rust/game_rules), which write what a question reads and say by
//! name what a question could not answer. docs/agents/rust.md says what Rust code owes the game.
//!
//! What every line in this crate owes the game:
//!   · one arithmetic. Doubles, and only + − × ÷ √ and what the standard makes exact of them (floor, trunc, abs, comparisons, the bits). Nothing that a
//!     compiler or a machine may round as it likes: no `f64::sin` and friends (rust/maths has the game's own), no `mul_add`, no fast-math. An `f32` only where the
//!     standard makes it exact too (a stored pose: single.rs; a rough rectangle of the plan: glow.rs; a hand's note: layers/).
//!   · one order. Nothing whose order is not written down: no `HashMap` / `HashSet` iteration, no pointer values as keys, no threads.
//!   · no world outside. No clock, no random source of its own, no files.
//!   · state in one place. What a game keeps here from one call to the next is in its block (state.rs: indices, never addresses); every other `static mut`
//!     is scratch of one call or a copy the host proves.
//!
//! WHO HOLDS IT. The compiler: the crate is `no_std`, so no line of it can name a hash map, a thread, a clock or a file (`std::…` does not resolve here).
//! tools/qa/rust-rules-check.ts: the float methods the compiler cannot keep out once any crate links the standard
//! library (`.sin()`, `.powf()`, `.mul_add()` …), and the SIMD that rounds as the machine likes. The reader: the order of the arithmetic.
//! √, floor and trunc, which the core library lacks, come from rust/prim.
#![no_std]
extern crate alloc;

use game_types::boxworld::World;
use game_types::knobs::Knobs;
pub mod air;
pub mod bricks;
pub mod clips;
pub mod collide;
pub mod dome;
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
// ---- the game's own smoke on the coarse grid (bricks.rs) and the light that comes round (glow.rs)

// ---- WHAT A PANE OF GLASS IS TO PLAY (panes.rs, pen.rs): the rule, a round's toll. A rule's call names its world and the time once (`panes::Ctx`), and what it let go
// is left in that for the caller.
/// the seed of a box of the world (`panes::box_seed`)
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
/// what a game holds of what a level is counted for as it loads (game_rules levelfit.rs): 0 window's bars gone · 1 clumps of a sheet
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
