//! The game's state as bytes: the bridge's export and import, the filtered export, each type's own way onto the wire. May name the data only, no rule.
#![no_std]
#![allow(dead_code, unused_variables, unused_imports, non_snake_case, non_camel_case_types, non_upper_case_globals, unused_mut)]
extern crate alloc;
pub use game_types::*;
pub mod io;
pub mod puppet;
pub mod id_tables;
pub mod type_impls;
