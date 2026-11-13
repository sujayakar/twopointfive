//! THE SIMULATION'S LOOPS: the questions of the box world (lines, rays, circles, a way across the floor), the senses, the smoke, the glass and the animator's layers, over
//! records and slices of the game. THE HOST, in these files, is whoever calls this crate: the rules (rust/game_rules), which write what a question reads and say by
//! name what a question could not answer. docs/agents/rust.md says what Rust code owes the game.
//!
//! What every line in this crate owes the game:
//!   · one arithmetic. Doubles, and only + − × ÷ √ and what the standard makes exact of them (floor, trunc, abs, comparisons, the bits). Nothing that a
//!     compiler or a machine may round as it likes: no `f64::sin` and friends (rust/maths has the game's own), no `mul_add`, no fast-math. An `f32` only where the
//!     standard makes it exact too (a stored pose: single.rs; a rough rectangle of the plan: glow.rs; a hand's note: layers/).
//!   · one order. Nothing whose order is not written down: no `HashMap` / `HashSet` iteration, no pointer values as keys, no threads.
//!   · no world outside. No clock, no random source of its own, no files.
//!   · no state. What a game keeps is the game's; a `static mut` here is scratch of one call or a table of numbers.
//!
//! WHO HOLDS IT. The compiler: the crate is `no_std`, so no line of it can name a hash map, a thread, a clock or a file (`std::…` does not resolve here).
//! tools/qa/rust-rules-check.ts: the float methods the compiler cannot keep out once any crate links the standard
//! library (`.sin()`, `.powf()`, `.mul_add()` …), and the SIMD that rounds as the machine likes. The reader: the order of the arithmetic.
//! √, floor and trunc, which the core library lacks, come from rust/prim.
#![no_std]
#![cfg_attr(not(debug_assertions), deny(unused, nonstandard_style))]   // (a warning in a build that is played or gated is an error; an edit's own check only says it)
extern crate alloc;

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
pub mod layers;
pub mod light;
pub mod lines;
pub use ::maths;
pub mod nav;
pub mod panes;
pub mod pattern;
pub mod pen;
pub mod sight;
pub mod single;
pub mod skeleton;
pub mod snow;
pub mod weather;
pub mod wind;
pub mod world;

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
