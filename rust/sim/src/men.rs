//! THE MEN'S COLUMNS, as a rule here reads them. A man is a row: `state(g).men.<column>[row]`; the game writes the columns in these same bytes, with no copy.
use core::mem::{offset_of, size_of};

/// rows of each column
pub const ROWS: usize = 64;
/// the layout's own number, in the block's word (state.rs `PARTS`)
pub const SCHEMA: u32 = 0xa1f8eb71;

#[repr(C)]
pub struct Men {
    pub spare: [u32; 4],
    pub since: [f64; ROWS],
    pub gen: [u32; ROWS],
    pub owner: [u32; ROWS],
    pub rank: [u32; ROWS],
    pub reason: [u32; ROWS],
}
const _: () = assert!(size_of::<Men>() == 1552);
// every column is whole vectors from a 16-byte boundary: the part starts on one (state.rs), the head is 16 bytes, the rows a multiple of four
const _: () = assert!(ROWS % 4 == 0);
const _: () = assert!(offset_of!(Men, since) % 16 == 0 && offset_of!(Men, gen) % 16 == 0 && offset_of!(Men, owner) % 16 == 0 && offset_of!(Men, rank) % 16 == 0 && offset_of!(Men, reason) % 16 == 0);
