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
    match search {
        SquadOfQuerySearch::Text(s) => squad_of(&crate::query::Query::parse(s), flag4),
        SquadOfQuerySearch::URLSearchParams(q) => squad_of(q, flag4),
    }
}

/// The squad size a query asks for: `squad=N` (0 to 4), else `squad4` = 4 unless `flag4` is false, else None. (loadout.ts squadOfQuery, of
/// words already read)
pub fn squad_of(q: &crate::query::Query, flag4: bool) -> Option<f64> {
    let asked = q.get("squad").and_then(size_of_text);
    if asked.is_some() { return asked; }
    if flag4 && q.has("squad4") { Some(4.0) } else { None }
}

/// The words for a size: 'solo', 'you + one', 'you + the pair', 'you + three', 'you + four'. (loadout.ts sizeWords)
pub fn size_words(n: f64) -> String {
    String::from(if n >= 4.0 { "you + four" } else if n == 3.0 { "you + three" } else if n == 2.0 { "you + the pair" } else if n == 1.0 { "you + one" } else { "solo" })
}


/// `asSquadSize` of a string: one digit once trimmed as the language trims, and a size only if 0 to 4. (loadout.ts:31)
fn size_of_text(v: &str) -> Option<f64> {
    let mut chars = v.trim_matches(js::is_space).chars();
    let (d, rest) = (chars.next()?, chars.next());
    if rest.is_some() || !d.is_ascii_digit() { return None; }
    let n = d as u32 - '0' as u32;
    if n <= 4 { Some(n as f64) } else { None }
}

