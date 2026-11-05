//! src/scene/scenarioList.ts lines 1 to 278
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// How many teammates, in the picker's words: 'you + the pair', 'solo' … (scenarioList.ts teamWords)
pub fn team_words(e: crate::scene_scenario_list::TeamWordsE) -> String { crate::loadout::size_words(e.squad) }

/// What a query string asks for, read as the page starts it: which scenario, how many teammates, which level file and seed; None when it names none.
/// (scenarioList.ts specOfQuery)
pub fn spec_of_query(search: &crate::scene_scenario_list::SpecOfQuerySearch) -> Option<crate::scene_scenario_list::ScenarioSpec> {
    let (q, asked) = match search {
        SpecOfQuerySearch::Text(s) => (params_of_text(s), crate::loadout::SquadOfQuerySearch::Text(s.clone())),
        SpecOfQuerySearch::URLSearchParams(p) => (seam_params_of_opaque(p), crate::loadout::SquadOfQuerySearch::URLSearchParams(*p)),
    };
    let kind = kind_of(&q)?;
    // a quiet mission is alone unless `squad=1`; any bigger size it is asked for is the one partner
    let squad = if kind == ScenarioSpecKind::Quiet {
        if crate::loadout::squad_of_query(&asked, Some(false)).unwrap_or(0.0) > 0.0 { 1.0 } else { 0.0 }
    } else {
        crate::loadout::squad_of_query(&asked, Some(kind != ScenarioSpecKind::Rescue)).unwrap_or(2.0)
    };
    let (level, seed) = place_of(kind, &q);
    Some(ScenarioSpec { kind, squad, level, seed })
}

/// The URLSearchParams arm of the union: `crate::Opaque` holds no pairs. (scenarioList.ts:241)
fn seam_params_of_opaque(params: &crate::Opaque) -> Vec<(String, String)> { todo!("needs: a type for URLSearchParams that holds its pairs") }

/// `?squad4` alone is the room clear with the four; beside another scenario's flag it is only that scenario's size.
fn kind_of(q: &[(String, String)]) -> Option<ScenarioSpecKind> {
    let four = has(q, "squad4");
    if has(q, "roomclear") || (four && !has(q, "nightbreach") && !has(q, "rescue") && !has(q, "quiet")) {
        Some(ScenarioSpecKind::Roomclear)
    } else if has(q, "nightbreach") {
        Some(ScenarioSpecKind::Nightbreach)
    } else if has(q, "rescue") {
        Some(ScenarioSpecKind::Rescue)
    } else if has(q, "quiet") {
        Some(ScenarioSpecKind::Quiet)
    } else {
        None
    }
}

/// The level file and the placement seed the query pins for this kind.
fn place_of(kind: ScenarioSpecKind, q: &[(String, String)]) -> (Option<String>, Option<f64>) {
    let level = get(q, "levelFile").filter(|lf| is_level_file(lf)).map(String::from);
    let seed = match kind {
        ScenarioSpecKind::Roomclear => seed_of_word(get(q, "roomclear").filter(|s| !s.is_empty()).or(get(q, "squad4"))),
        ScenarioSpecKind::Quiet => seed_of_word(get(q, "quiet")),
        _ => None,
    };
    (level, seed)
}

/// `s && Number.isFinite(Number(s))` then `Math.floor(Number(s))`.
fn seed_of_word(s: Option<&str>) -> Option<f64> {
    let n = number_of(s.filter(|s| !s.is_empty())?);
    if n.is_finite() { Some(m::floor(n)) } else { None }
}

/// /^[\w.-]+\.json$/
fn is_level_file(lf: &str) -> bool {
    match lf.strip_suffix(".json") {
        Some(stem) => !stem.is_empty() && stem.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.' || b == b'-'),
        None => false,
    }
}

/// URLSearchParams.has
fn has(q: &[(String, String)], key: &str) -> bool { q.iter().any(|(k, _)| k == key) }
/// URLSearchParams.get: the first value of the key
fn get<'a>(q: &'a [(String, String)], key: &str) -> Option<&'a str> { q.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str()) }

/// `new URLSearchParams(string)`: one leading `?` off, the pairs in order, an empty piece skipped, a bare key's value ''.
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

/// `Number(string)`: the language's own grammar, not Rust's (which takes 'inf' and 'nan', and refuses '' and hex).
fn number_of(s: &str) -> f64 {
    let t = s.trim_matches(is_js_space);
    if t.is_empty() { return 0.0; }
    let b = t.as_bytes();
    if b.len() >= 2 && b[0] == b'0' {
        let bits = match b[1] { b'x' | b'X' => 4, b'o' | b'O' => 3, b'b' | b'B' => 1, _ => 0 };
        if bits != 0 { return radix_value(&b[2..], bits); }
    }
    let unsigned = t.strip_prefix(['+', '-']).unwrap_or(t);
    if unsigned == "Infinity" { return if b[0] == b'-' { f64::NEG_INFINITY } else { f64::INFINITY }; }
    if !is_decimal_literal(unsigned) { return f64::NAN; }
    t.parse().unwrap_or(f64::NAN)
}

/// digits [. digits] [e [+-] digits], with a digit somewhere before the exponent.
fn is_decimal_literal(s: &str) -> bool {
    let digits = |s: &str| s.bytes().take_while(u8::is_ascii_digit).count();
    let whole = digits(s);
    let mut rest = &s[whole..];
    let mut fraction = 0;
    if let Some(after) = rest.strip_prefix('.') {
        fraction = digits(after);
        rest = &after[fraction..];
    }
    if whole + fraction == 0 { return false; }
    if rest.is_empty() { return true; }
    let Some(exp) = rest.strip_prefix(['e', 'E']) else { return false };
    let exp = exp.strip_prefix(['+', '-']).unwrap_or(exp);
    !exp.is_empty() && digits(exp) == exp.len()
}

/// A `0x` / `0o` / `0b` literal's digits, `bits` a digit, rounded once to the nearest double as the language does; NaN if none or a bad one.
fn radix_value(digits: &[u8], bits: u32) -> f64 {
    if digits.is_empty() { return f64::NAN; }
    let (mut kept, mut scale) = (0u64, 0u32);
    for &b in digits {
        let Some(d) = (b as char).to_digit(1 << bits) else { return f64::NAN };
        if kept >> (64 - bits) == 0 {
            kept = kept << bits | d as u64;
        } else {
            scale += bits;
            if d != 0 { kept |= 1; } // past 54 bits only "was anything left" matters to the rounding
        }
    }
    let mut v = kept as f64;
    for _ in 0..scale {
        v *= 2.0;
        if v.is_infinite() { break; }
    }
    v
}

/// The language's StrWhiteSpaceChar (`char::is_whitespace` differs at U+0085 and U+FEFF).
fn is_js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{B}' | '\u{C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}')
}

#[cfg(test)]
mod tests {
    use super::*;
    use ScenarioSpecKind as K;

    // The TypeScript's own answers: src/scene/scenarioList.ts specOfQuery run on each query under bun and node (they agree): kind, level, seed's bits.
    // squad is not here: it is loadout's answer, a stub in this crate.
    const CASES: &[(&str, Option<K>, Option<&str>, Option<u64>)] = &[
    ("?quiet=12.7", Some(K::Quiet), None, Some(0x4028000000000000)),
    ("?quiet=-3.5", Some(K::Quiet), None, Some(0xc010000000000000)),
    ("?quiet=1e3", Some(K::Quiet), None, Some(0x408f400000000000)),
    ("?quiet=0x1F", Some(K::Quiet), None, Some(0x403f000000000000)),
    ("?quiet=0X1f", Some(K::Quiet), None, Some(0x403f000000000000)),
    ("?quiet=0b101", Some(K::Quiet), None, Some(0x4014000000000000)),
    ("?quiet=0B11", Some(K::Quiet), None, Some(0x4008000000000000)),
    ("?quiet=0o17", Some(K::Quiet), None, Some(0x402e000000000000)),
    ("?quiet=0O7", Some(K::Quiet), None, Some(0x401c000000000000)),
    ("?quiet=Infinity", Some(K::Quiet), None, None),
    ("?quiet=-Infinity", Some(K::Quiet), None, None),
    ("?quiet=+Infinity", Some(K::Quiet), None, None),
    ("?quiet=infinity", Some(K::Quiet), None, None),
    ("?quiet=abc", Some(K::Quiet), None, None),
    ("?quiet=%201%20", Some(K::Quiet), None, Some(0x3ff0000000000000)),
    ("?quiet=+5", Some(K::Quiet), None, Some(0x4014000000000000)),
    ("?quiet=%2B5", Some(K::Quiet), None, Some(0x4014000000000000)),
    ("?quiet=.5", Some(K::Quiet), None, Some(0x0000000000000000)),
    ("?quiet=5.", Some(K::Quiet), None, Some(0x4014000000000000)),
    ("?quiet=.", Some(K::Quiet), None, None),
    ("?quiet=1e", Some(K::Quiet), None, None),
    ("?quiet=1e+", Some(K::Quiet), None, None),
    ("?quiet=1E2", Some(K::Quiet), None, Some(0x4059000000000000)),
    ("?quiet=-0", Some(K::Quiet), None, Some(0x8000000000000000)),
    ("?quiet=-0x10", Some(K::Quiet), None, None),
    ("?quiet=%2B0x10", Some(K::Quiet), None, None),
    ("?quiet=1e999", Some(K::Quiet), None, None),
    ("?quiet=-1e999", Some(K::Quiet), None, None),
    ("?quiet=%C2%A07", Some(K::Quiet), None, Some(0x401c000000000000)),
    ("?quiet=%EF%BB%BF7", Some(K::Quiet), None, Some(0x401c000000000000)),
    ("?quiet=%C2%857", Some(K::Quiet), None, None),
    ("?quiet=%E2%80%A87", Some(K::Quiet), None, Some(0x401c000000000000)),
    ("?quiet=%E2%80%897", Some(K::Quiet), None, Some(0x401c000000000000)),
    ("?quiet=%E1%A0%8E7", Some(K::Quiet), None, None),
    ("?quiet=%0B7%0C", Some(K::Quiet), None, Some(0x401c000000000000)),
    ("?quiet=+", Some(K::Quiet), None, Some(0x0000000000000000)),
    ("?quiet=1_000", Some(K::Quiet), None, None),
    ("?quiet=nan", Some(K::Quiet), None, None),
    ("?quiet=NaN", Some(K::Quiet), None, None),
    ("?quiet=inf", Some(K::Quiet), None, None),
    ("?quiet=0x", Some(K::Quiet), None, None),
    ("?quiet=0xg", Some(K::Quiet), None, None),
    ("?quiet=0b2", Some(K::Quiet), None, None),
    ("?quiet=0o8", Some(K::Quiet), None, None),
    ("?quiet=1.5e-3", Some(K::Quiet), None, Some(0x0000000000000000)),
    ("?quiet=123456789012345678901234567890", Some(K::Quiet), None, Some(0x45f8ee90ff6c373e)),
    ("?quiet=0x20000000000001", Some(K::Quiet), None, Some(0x4340000000000000)),
    ("?quiet=0x200000000000011", Some(K::Quiet), None, Some(0x4380000000000001)),
    ("?quiet=0xFFFFFFFFFFFFFFFFF", Some(K::Quiet), None, Some(0x4430000000000000)),
    ("?quiet=0x1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", Some(K::Quiet), None, None),
    ("?quiet=00012", Some(K::Quiet), None, Some(0x4028000000000000)),
    ("?quiet=1.7976931348623157e308", Some(K::Quiet), None, Some(0x7fefffffffffffff)),
    ("?quiet=4.9e-324", Some(K::Quiet), None, Some(0x0000000000000000)),
    ("?quiet=-.5", Some(K::Quiet), None, Some(0xbff0000000000000)),
    ("?quiet=-.", Some(K::Quiet), None, None),
    ("?quiet=1 2", Some(K::Quiet), None, None),
    ("?quiet=\u{663}", Some(K::Quiet), None, None),
    ("", None, None, None),
    ("?", None, None, None),
    ("?mute", None, None, None),
    ("rescue", Some(K::Rescue), None, None),
    ("??rescue", None, None, None),
    ("?a=1&&rescue", Some(K::Rescue), None, None),
    ("?ROOMCLEAR", None, None, None),
    ("?room%63lear=4", Some(K::Roomclear), None, Some(0x4010000000000000)),
    ("?roomclear=7", Some(K::Roomclear), None, Some(0x401c000000000000)),
    ("?roomclear", Some(K::Roomclear), None, None),
    ("?roomclear&squad4=9", Some(K::Roomclear), None, Some(0x4022000000000000)),
    ("?roomclear=x&squad4=9", Some(K::Roomclear), None, None),
    ("?roomclear=&squad4=&x", Some(K::Roomclear), None, None),
    ("?roomclear=1&roomclear=2", Some(K::Roomclear), None, Some(0x3ff0000000000000)),
    ("?squad4", Some(K::Roomclear), None, None),
    ("?squad4=3", Some(K::Roomclear), None, Some(0x4008000000000000)),
    ("?squad4=%ZZ", Some(K::Roomclear), None, None),
    ("?squad4=3.9&nightbreach", Some(K::Nightbreach), None, None),
    ("?squad4&rescue", Some(K::Rescue), None, None),
    ("?squad4&quiet", Some(K::Quiet), None, None),
    ("?squad4&quiet=8", Some(K::Quiet), None, Some(0x4020000000000000)),
    ("?nightbreach&rescue", Some(K::Nightbreach), None, None),
    ("?rescue&quiet", Some(K::Rescue), None, None),
    ("?roomclear&quiet=5", Some(K::Roomclear), None, None),
    ("?nightbreach=4", Some(K::Nightbreach), None, None),
    ("?rescue=4", Some(K::Rescue), None, None),
    ("?quiet=-2.5&squad=1", Some(K::Quiet), None, Some(0xc008000000000000)),
    ("?quiet=1&quiet=2", Some(K::Quiet), None, Some(0x3ff0000000000000)),
    ("?quiet", Some(K::Quiet), None, None),
    ("?quiet=", Some(K::Quiet), None, None),
    ("?rescue&levelFile=a.json", Some(K::Rescue), Some("a.json"), None),
    ("?rescue&levelFile=a.json%0A", Some(K::Rescue), None, None),
    ("?rescue&levelFile=.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=..json", Some(K::Rescue), Some("..json"), None),
    ("?rescue&levelFile=a-b_c.d.json", Some(K::Rescue), Some("a-b_c.d.json"), None),
    ("?rescue&levelFile=a%20b.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=a+b.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=%C3%A9.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=a.JSON", Some(K::Rescue), None, None),
    ("?rescue&levelFile=../x.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=", Some(K::Rescue), None, None),
    ("?rescue&levelFile=a.jsonx", Some(K::Rescue), None, None),
    ("?rescue&levelFile=x%2Fy.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=%FF.json", Some(K::Rescue), None, None),
    ("?rescue&levelFile=levels.json&levelFile=b.json", Some(K::Rescue), Some("levels.json"), None),
    ("?rescue&levelFile=a.json%00", Some(K::Rescue), None, None),
    ("?roomclear=3.5&levelFile=a.json", Some(K::Roomclear), Some("a.json"), Some(0x4008000000000000)),
    ("?roomclear=3.5&levelFile=bad", Some(K::Roomclear), None, Some(0x4008000000000000)),
    ];

    #[test]
    fn reads_every_query_as_the_typescript_does() {
        for &(text, kind, level, seed) in CASES {
            let q = params_of_text(text);
            let got = kind_of(&q);
            assert_eq!(got, kind, "kind of {text:?}");
            let Some(k) = got else { continue };
            let (l, s) = place_of(k, &q);
            assert_eq!(l.as_deref(), level, "level of {text:?}");
            assert_eq!(s.map(f64::to_bits), seed, "seed of {text:?}");
        }
    }

    #[test]
    fn a_query_naming_no_scenario_is_none() {
        for text in ["", "?", "?mute", "?squad=4", "??rescue", "?ROOMCLEAR"] {
            assert!(spec_of_query(&SpecOfQuerySearch::Text(String::from(text))).is_none(), "{text:?}");
        }
    }
}
