//! DRESSING IN RUST — what only makes the picture and is worth doing on the CPU: the pieces of a broken thing tumbling, the fine detail of smoke, anything
//! whose every bit could differ between two machines and no fight would end differently (docs/agents/rust.md, gameplay and dressing; the list of what is
//! dressing is src/game/dress.ts, and tools/qa/dress-check.ts holds it).
//!
//! THE OTHER SIDE OF THE LINE FROM rust/sim, AND BUILT APART FROM IT: this crate is its own WebAssembly module with its own memory. It never links the
//! simulation and the two modules never share a memory, so nothing here CAN write what the simulation holds: what dressing needs to know of the game
//! (where a wall broke, where a man stands) the page copies into this module's own memory, one way.
//!
//! So this crate is free where the simulation is bound:
//!   · it may round as the machine likes: singles, the standard library's functions, fused multiply-add, relaxed SIMD.
//!   · it may keep its own time and its own random numbers. Two machines need not agree, and the game server does not run it at all.
//! What it still may not: import anything (a module that imports nothing cannot reach a clock, a network or another module's memory; the build is red on one),
//! and be asked a question by a rule of play. An answer a rule needs is the simulation's to give.

/// How many singles the page's hand-over holds.
pub const IN_LEN: usize = 4096;
/// What the page copies in of the game before a step: this module's own memory, never the simulation's.
static mut IN: [f32; IN_LEN] = [0.0; IN_LEN];

/// The version of the exports: src/dress/dress.ts names the one it was written against.
pub const ABI: u32 = 1;

#[no_mangle]
pub extern "C" fn dress_abi() -> u32 {
    ABI
}
#[no_mangle]
pub extern "C" fn dress_in() -> *mut f32 {
    core::ptr::addr_of_mut!(IN) as *mut f32
}
#[no_mangle]
pub extern "C" fn dress_in_len() -> usize {
    IN_LEN
}
