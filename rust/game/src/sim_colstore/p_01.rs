//! src/sim/colstore.ts lines 1 to 64
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The part's length behind a head of `head` bytes, columns `cap` rows long, the doubles first, then the words. The columns' starts have no field
/// to go in (`ColsOf2` holds `bytes` alone): each family's own walk gives them (prop_cols, slab_cols, item_cols). (colstore.ts colsOf)
pub fn cols_of<F: Clone + core::fmt::Debug, U: Clone + core::fmt::Debug>(head: f64, f64: &[F], u32: &[U], cap: f64) -> crate::sim_colstore::ColsOf2 {
    let mut o = ColsOf2::zeroed();
    let mut at = head;
    for _ in f64 { at += cap * 8.0; }
    for _ in u32 { at += cap * 4.0; }
    o.bytes = at;
    o
}

/// Where two takes of a store's bytes differ, column by column and row by row over the first `rows` rows: at most `most` lines in all. (colstore.ts colDiff)
pub fn col_diff<F: Clone + core::fmt::Debug, U: Clone + core::fmt::Debug>(o: crate::sim_colstore::ColDiffO, f64: &[F], u32: &[U], a: &[u8], b: &[u8], rows: f64, who: &mut dyn FnMut(Vec<u32>, f64) -> String, most: f64, out: Option<&mut Vec<String>>) -> Vec<String> {
    let mut own = Vec::new();
    let out: &mut Vec<String> = match out { Some(given) => given, None => &mut own };
    let first = seam_col_at(&o, &u32[0]);
    let doubles_long = (js::to_u32(first) >> 1) as usize;
    let words_long = (js::to_u32(o.bytes) >> 2) as usize;
    let (da, db) = (doubles_of(a, doubles_long), doubles_of(b, doubles_long));
    let (ua, ub) = (words_of(a, words_long), words_of(b, words_long));
    for k in f64 {
        let col = seam_col_at(&o, k) as usize;
        let mut r = 0usize;
        while (r as f64) < rows && (out.len() as f64) < most {
            // past the view's end both reads are `undefined` in the TypeScript, and the same: no line
            if let (Some(&x), Some(&y)) = (da.get(col + r), db.get(col + r)) {
                if !object_is(x, y) {
                    let whom = who(ua.clone(), r as f64);
                    out.push(format!("row {} ({}).{} {} | {}", r, whom, seam_col_name(k), js::num_str(x), js::num_str(y)));
                }
            }
            r += 1;
        }
    }
    for k in u32 {
        let col = seam_col_at(&o, k) as usize;
        let mut r = 0usize;
        while (r as f64) < rows && (out.len() as f64) < most {
            if let (Some(&x), Some(&y)) = (ua.get(col + r), ub.get(col + r)) {
                if x != y { out.push(format!("row {}.{} {} | {}", r, seam_col_name(k), x, y)); }
            }
            r += 1;
        }
    }
    out.clone()
}

/// `O[k]`, a column's start by the column's name
fn seam_col_at<K>(o: &ColDiffO, k: &K) -> f64 {
    todo!("needs: ColDiffO to hold the start of each column, and a way from a column's name K to its field")
}

/// `${k}`, a column's name as the TypeScript's string
fn seam_col_name<K>(k: &K) -> String {
    todo!("needs: a column's name K as its string (Debug would quote a &str and print a variant's name)")
}

/// `new Float64Array(bytes.buffer, bytes.byteOffset, n)`, as values: too short panics where the TypeScript throws
fn doubles_of(bytes: &[u8], n: usize) -> Vec<f64> {
    bytes[..n * 8].chunks_exact(8).map(|c| f64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]])).collect()
}

/// `new Uint32Array(bytes.buffer, bytes.byteOffset, n)`, as values: too short panics where the TypeScript throws
fn words_of(bytes: &[u8], n: usize) -> Vec<u32> {
    bytes[..n * 4].chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// `Object.is` on numbers: a NaN is a NaN whatever its bits, −0 is not 0
fn object_is(x: f64, y: f64) -> bool {
    (x.is_nan() && y.is_nan()) || x.to_bits() == y.to_bits()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// colsOf(16, ['x', 'y'], ['a'], 10).bytes and colsOf(8, ['x'], [], 3.5).bytes, by the TypeScript's own sum.
    #[test]
    fn bytes_is_the_head_and_every_column() {
        assert_eq!(cols_of(16.0, &["x", "y"], &["a"], 10.0).bytes, 216.0);
        let none: &[&str] = &[];
        assert_eq!(cols_of(8.0, &["x"], none, 3.5).bytes, 36.0);
    }

    #[test]
    fn object_is_tells_minus_zero_and_not_nans() {
        assert!(object_is(f64::NAN, f64::from_bits(0x7ff8_0000_0000_0001)));
        assert!(!object_is(0.0, -0.0));
        assert!(object_is(1.5, 1.5));
        assert!(!object_is(1.5, 2.5));
    }

    #[test]
    fn a_view_reads_the_first_n_values_of_its_bytes() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1.5f64.to_le_bytes());
        bytes.extend_from_slice(&7u32.to_le_bytes());
        bytes.extend_from_slice(&9u32.to_le_bytes());
        assert_eq!(doubles_of(&bytes, 1), [1.5]);
        assert_eq!(words_of(&bytes, 4)[2..], [7, 9]);
    }
}
