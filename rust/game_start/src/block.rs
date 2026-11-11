//! A GAME'S BLOCK OF rust/sim, AS A HOST COMPARES IT: the bytes of a slot, and a hash of each part (over a part's words up to the
//! last that is not zero).
use alloc::vec::Vec;

/// the parts' names, in the parts' order (rust/sim/src/state.rs PARTS)
pub const PART_NAMES: [&str; 1] = ["head"];
const _: () = assert!(PART_NAMES.len() == sim::state::PARTS.len());

/// how long a block is, in bytes
pub fn len() -> usize { core::mem::size_of::<sim::state::State>() }
/// the block in slot `slot`, as bytes
pub fn bytes(slot: usize) -> &'static [u8] { unsafe { core::slice::from_raw_parts(sim::state::addr(slot) as *const u8, len()) } }
fn words(slot: usize) -> &'static [u32] { unsafe { core::slice::from_raw_parts(sim::state::addr(slot) as *const u32, len() / 4) } }
/// a hash of each part of the block in slot `slot`, in the parts' order
pub fn part_hashes(slot: usize) -> Vec<u32> {
    let w = words(slot);
    sim::state::PARTS.iter().map(|p| {
        let part = &w[p.off / 4..(p.off + p.len) / 4];
        let mut n = part.len();
        while n > 0 && part[n - 1] == 0 { n -= 1; }
        let mut h: u32 = 0x811c9dc5;
        for &x in &part[..n] { h = (h ^ x).wrapping_mul(0x01000193); }
        h
    }).collect()
}
