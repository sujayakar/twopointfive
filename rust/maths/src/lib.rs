//! THE GAME'S OWN MATHS AND ITS DIE: what the types, the loops and the rules all count with, and nothing that keeps anything.
#![no_std]
#![cfg_attr(not(debug_assertions), deny(unused, nonstandard_style))]   // (a warning in a build that is played or gated is an error; an edit's own check only says it)
mod fns;
pub use fns::*;
