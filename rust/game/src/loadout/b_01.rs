//! Bodies for skeleton stubs of loadout that translated code calls and no part had (the stub round).
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The squad size a query string asks for: `squad=N` (0 to 4), else `squad4` = 4 unless `flag4` is false, else None. (loadout.ts squadOfQuery)
pub fn squad_of_query(search: &crate::loadout::SquadOfQuerySearch, flag4: Option<bool>) -> Option<f64> {
    let flag4 = flag4.unwrap_or(true);
    let q = match search {
        SquadOfQuerySearch::Text(s) => params_of_text(s),
        SquadOfQuerySearch::URLSearchParams(p) => seam_params_of_opaque(p),
    };
    let asked = q.iter().find(|(k, _)| k == "squad").and_then(|(_, v)| size_of_text(v));
    if asked.is_some() { return asked; }
    if flag4 && q.iter().any(|(k, _)| k == "squad4") { Some(4.0) } else { None }
}

/// The words for a size: 'solo', 'you + one', 'you + the pair', 'you + three', 'you + four'. (loadout.ts sizeWords)
pub fn size_words(n: f64) -> String {
    String::from(if n >= 4.0 { "you + four" } else if n == 3.0 { "you + three" } else if n == 2.0 { "you + the pair" } else if n == 1.0 { "you + one" } else { "solo" })
}

/// The URLSearchParams arm of the union: `crate::Opaque` holds no pairs. (loadout.ts:35)
fn seam_params_of_opaque(params: &crate::Opaque) -> Vec<(String, String)> { todo!("needs: a type for URLSearchParams that holds its pairs") }

/// `asSquadSize` of a string: one digit once trimmed as the language trims, and a size only if 0 to 4. (loadout.ts:31)
fn size_of_text(v: &str) -> Option<f64> {
    let mut chars = v.trim_matches(is_js_space).chars();
    let (d, rest) = (chars.next()?, chars.next());
    if rest.is_some() || !d.is_ascii_digit() { return None; }
    let n = d as u32 - '0' as u32;
    if n <= 4 { Some(n as f64) } else { None }
}

/// What `String.prototype.trim` strips: not Rust's `trim` set, which has U+0085 and lacks U+FEFF.
fn is_js_space(c: char) -> bool {
    matches!(c, '\u{9}'..='\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

/// `new URLSearchParams(text)`: one leading `?` off, `&` between pairs, empty pieces skipped, each side form-decoded.
fn params_of_text(s: &str) -> Vec<(String, String)> {
    let s = s.strip_prefix('?').unwrap_or(s);
    s.split('&')
        .filter(|piece| !piece.is_empty())
        .map(|piece| {
            let (k, v) = piece.split_once('=').unwrap_or((piece, ""));
            (form_decoded(k), form_decoded(v))
        })
        .collect()
}

/// `+` is a space, `%HH` a byte (a `%` with no two hex digits after it stays), the bytes read as UTF-8 with U+FFFD for what is not.
fn form_decoded(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        let hex = if b[i] == b'%' && i + 2 < b.len() { hex_digit(b[i + 1]).zip(hex_digit(b[i + 2])) } else { None };
        if let Some((hi, lo)) = hex {
            out.push(hi * 16 + lo);
            i += 3;
        } else {
            out.push(if b[i] == b'+' { b' ' } else { b[i] });
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_digit(b: u8) -> Option<u8> { (b as char).to_digit(16).map(|d| d as u8) }
