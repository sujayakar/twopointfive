//! THE GAME'S OWN START, IN RUST (README.md beside this crate). A level file's text and the character's files in, a `Game` the tick can run out.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
extern crate alloc;
pub mod json;
pub mod gltf;
pub mod level;
pub mod passes;
pub mod settings;
pub mod stand;
#[cfg(feature = "std")]
pub mod files;
pub mod scene;
#[cfg(feature = "std")]
pub mod play;
pub mod block;
