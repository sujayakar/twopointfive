//! src/sim/colstore.ts lines 1 to 64
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Where each column starts: a double column as an index into doubles, a word column into words, from the part's start; `bytes` the part's length. (colstore.ts colsOf)
pub fn cols_of<F: Clone + core::fmt::Debug, U: Clone + core::fmt::Debug>(head: f64, f64: &[F], u32: &[U], cap: f64) -> crate::sim_colstore::ColsOf2 {
    let mut o = ColsOf2::zeroed();
    let mut at = head;
    for k in f64 { seam_set_col(&mut o, k, at / 8.0); at += cap * 8.0; }
    for k in u32 { seam_set_col(&mut o, k, at / 4.0); at += cap * 4.0; }
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
                    out.push(format!("row {} ({}).{} {} | {}", r, whom, seam_col_name(k), num_str(x), num_str(y)));
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

/// `o[k] = at`, a column's start by the column's name
fn seam_set_col<K>(o: &mut ColsOf2, k: &K, at: f64) {
    todo!("needs: ColsOf2 to hold the start of each column, and a way from a column's name K to its field")
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

/// A number in a template string, as the language writes it (Number::toString, radix 10): `{}` differs at −0, the infinities and large or small exponents
fn num_str(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    if x == 0.0 { return String::from("0"); }
    if x < 0.0 { return format!("-{}", num_str(-x)); }
    if x == f64::INFINITY { return String::from("Infinity"); }
    // `{:e}` writes the same shortest digits that round-trip, as `d.ddde<exp>`
    let sci = format!("{:e}", x);
    let (mantissa, exp) = sci.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| *c != '.').collect();
    let k = digits.len() as i32;
    let n = exp.parse::<i32>().unwrap() + 1;
    if k <= n && n <= 21 {
        let mut s = digits;
        for _ in 0..(n - k) { s.push('0'); }
        return s;
    }
    if 0 < n && n <= 21 {
        return format!("{}.{}", &digits[..n as usize], &digits[n as usize..]);
    }
    if -6 < n && n <= 0 {
        let mut s = String::from("0.");
        for _ in 0..-n { s.push('0'); }
        s.push_str(&digits);
        return s;
    }
    let e = n - 1;
    let sign = if e < 0 { '-' } else { '+' };
    let size = if e < 0 { -e } else { e };
    if k == 1 { return format!("{}e{}{}", digits, sign, size); }
    format!("{}.{}e{}{}", &digits[..1], &digits[1..], sign, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The right-hand sides are node's own output for `${x}`.
    #[test]
    fn numbers_read_as_the_language_writes_them() {
        let cases: &[(f64, &str)] = &[
            (0.0, "0"), (-0.0, "0"), (1.0, "1"), (-1.5, "-1.5"), (1e21, "1e+21"), (1e-7, "1e-7"),
            (123456789012345680000.0, "123456789012345680000"), (1.7976931348623157e308, "1.7976931348623157e+308"),
            (5e-324, "5e-324"), (f64::NAN, "NaN"), (f64::INFINITY, "Infinity"), (f64::NEG_INFINITY, "-Infinity"),
            (0.000001, "0.000001"), (1.5e300, "1.5e+300"), (100.0, "100"), (0.1, "0.1"), (2.5e-7, "2.5e-7"), (1e20, "100000000000000000000"),
        ];
        for (x, want) in cases { assert_eq!(num_str(*x), *want); }
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
