//! WRITTEN BY tools/sim/men.ts FROM src/sim/men/schema.ts — never by hand (tools/qa/men-check.ts is red on a difference). A man's columns, as a rule here reads them.
//! A man is a row: `state(g).men.<column>[row]`. The host writes the columns through src/sim/men/cols.ts, in these same bytes, with no copy.
use core::mem::{offset_of, size_of};

/// rows of each column (src/sim/men/schema.ts MEN_ROWS)
pub const ROWS: usize = 256;
/// the hash of the layout: src/sim/men/cols.ts MEN_SCHEMA is the same number
pub const SCHEMA: u32 = 0xab4ea830;

#[repr(C)]
pub struct Men {
    pub spare: [u32; 4],
    pub since: [f64; ROWS],
    pub gen: [u32; ROWS],
    pub owner: [u32; ROWS],
    pub rank: [u32; ROWS],
    pub reason: [u32; ROWS],
}
const _: () = assert!(size_of::<Men>() == 6160);

/// where each column is in the part, in bytes, in the order of the schema's layout (the host checks these against its own)
pub const COLS: [usize; 5] = [offset_of!(Men, since), offset_of!(Men, gen), offset_of!(Men, owner), offset_of!(Men, rank), offset_of!(Men, reason)];
// every column is whole vectors from a 16-byte boundary: the part starts on one (state.rs), the head is 16 bytes, the rows a multiple of four
const _: () = assert!(ROWS % 4 == 0);
const _: () = { let mut i = 0; while i < COLS.len() { assert!(COLS[i] % 16 == 0); i += 1; } };

#[no_mangle]
pub extern "C" fn men_schema() -> u32 { SCHEMA }
/// column `i`'s place in the part in bytes, or the part's rows (`i` = the number of columns) · 0 past that
#[no_mangle]
pub extern "C" fn men_col(i: u32) -> usize { let i = i as usize; if i < COLS.len() { COLS[i] } else if i == COLS.len() { ROWS } else { 0 } }
/// EVERY NUMBER OF A MAN'S ROW AS THIS MODULE READS IT, folded (FNV-1a over the words, the columns in their order): what tools/qa/men-check.ts holds to the TypeScript's accessors
#[no_mangle]
pub extern "C" fn men_fold(g: u32, row: u32) -> u32 {
    let m = &crate::state::state(g as usize).men;
    let r = row as usize % ROWS;
    let mut h: u32 = 0x811c9dc5;
    let mut w = |v: u32| { h = (h ^ v).wrapping_mul(0x01000193); };
    let b = m.since[r].to_bits(); w(b as u32); w((b >> 32) as u32);
    w(m.gen[r]);
    w(m.owner[r]);
    w(m.rank[r]);
    w(m.reason[r]);
    h
}
