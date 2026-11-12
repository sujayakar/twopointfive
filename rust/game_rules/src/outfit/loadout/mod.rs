pub mod screen;
pub use screen::*;
use crate::js;
use alloc::string::String;

/// The squad size a query asks for: `squad=N` (0 to 4), else `squad4` = 4 unless `flag4` is false, else None.
pub fn squad_of(q: &crate::query::Query, flag4: bool) -> Option<f64> {
    let asked = q.get("squad").and_then(size_of_text);
    if asked.is_some() { return asked; }
    if flag4 && q.has("squad4") { Some(4.0) } else { None }
}

/// The words for a size: 'solo', 'you + one', 'you + the pair', 'you + three', 'you + four'.
pub fn size_words(n: f64) -> String {
    String::from(if n >= 4.0 { "you + four" } else if n == 3.0 { "you + three" } else if n == 2.0 { "you + the pair" } else if n == 1.0 { "you + one" } else { "solo" })
}

/// The short tag beside a title: 'solo', 'one', 'pair', 'three', 'four'.
pub fn size_tag(n: f64) -> &'static str {
    if n >= 4.0 { "four" } else if n == 3.0 { "three" } else if n == 2.0 { "pair" } else if n == 1.0 { "one" } else { "solo" }
}

/// The link a mission is deployed by at a size: at its own size the link its row loads.
pub fn squad_search(link: &str, n: f64, own: f64) -> String {
    let mut q = crate::query::Query::parse(crate::query::after_mark(link));
    q.delete("loadout");
    // (a row whose own size IS its `squad=N`, the four's rescue, keeps it: dropped, the page would stage the pair)
    let kept = n == own && own != 2.0 && q.get("squad") == Some(js::num_str(own).as_str()) && !q.has("squad4");
    if !kept { q.delete("squad"); }
    if n != own {
        q.delete("squad4");
        q.set("squad", &js::num_str(n));
    }
    q.bare()
}

/// `asSquadSize` of a string: one digit once trimmed as the language trims, and a size only if 0 to 4.
fn size_of_text(v: &str) -> Option<f64> {
    let mut chars = v.trim_matches(js::is_space).chars();
    let (d, rest) = (chars.next()?, chars.next());
    if rest.is_some() || !d.is_ascii_digit() { return None; }
    let n = d as u32 - '0' as u32;
    if n <= 4 { Some(n as f64) } else { None }
}
