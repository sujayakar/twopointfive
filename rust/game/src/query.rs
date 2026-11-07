//! AN ADDRESS'S WORDS: the part of a page's address after the `?`, as the game reads it (`?levelFile=depot.json&roomclear&squad4`).
//!
//! The page starts a mission from these words (`scenario_run::start_from_query`: the mission, the team's size, the hour, the weather, the fog, a
//! quiet mission's rung and cameras), and every reader of them in the TypeScript takes the browser's `URLSearchParams`. This is that type in plain
//! Rust: the browser's rules for reading the text (`application/x-www-form-urlencoded`), the four calls the game makes on it (`has`, `get`, `set`,
//! `delete`), and the two ways the game reads a word as a number (`Number(text)`, `parseFloat(text)`). A host hands the address's text across
//! once and the game reads its words here; nothing of the browser is needed.
use alloc::string::String;
use alloc::vec::Vec;

/// the words of an address, in their order: a name and its value (a bare flag, `?squad4`, has the empty value)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query { pairs: Vec<(String, String)> }

impl Query {
    /// `new URLSearchParams(text)`: a leading `?` is dropped, the words are split at `&`, each at its first `=`, a `+` is a space and `%XX` a byte
    pub fn parse(text: &str) -> Query {
        let text = text.strip_prefix('?').unwrap_or(text);
        let pairs = text.split('&').filter(|w| !w.is_empty()).map(|w| match w.split_once('=') { Some((k, v)) => (decode(k), decode(v)), None => (decode(w), String::new()) }).collect();
        Query { pairs }
    }
    /// `q.has(name)`
    pub fn has(&self, name: &str) -> bool { self.pairs.iter().any(|(k, _)| k == name) }
    /// `q.get(name)`: the first value under the name
    pub fn get(&self, name: &str) -> Option<&str> { self.pairs.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str()) }
    /// `q.set(name, value)`: the first word of that name takes the value and the others of the name go; a new name goes last
    pub fn set(&mut self, name: &str, value: &str) {
        match self.pairs.iter().position(|(k, _)| k == name) {
            Some(i) => { self.pairs[i].1 = String::from(value); let mut at = 0; self.pairs.retain(|(k, _)| { at += 1; at - 1 <= i || k != name }); }
            None => self.pairs.push((String::from(name), String::from(value))),
        }
    }
    /// `q.delete(name)`: every word of that name
    pub fn delete(&mut self, name: &str) { self.pairs.retain(|(k, _)| k != name); }
    /// the words in their order (`q.forEach`)
    pub fn words(&self) -> &[(String, String)] { &self.pairs }
    /// `Number(q.get(name))` where the name is there (JavaScript's reading of the whole text: `number_of`)
    pub fn number(&self, name: &str) -> Option<f64> { self.get(name).map(number_of) }
    /// `parseFloat(q.get(name))` where the name is there (the number the text starts with: `float_of`)
    pub fn float(&self, name: &str) -> Option<f64> { self.get(name).map(float_of) }
    /// the words as text again, a bare flag left bare (`?mute`, not `?mute=`): scene/scenarioList.ts `bare`. No words = the empty text.
    pub fn bare(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.pairs { out.push(if out.is_empty() { '?' } else { '&' }); encode(k, &mut out); if !v.is_empty() { out.push('='); encode(v, &mut out); } }
        out
    }
}

/// one side of a word: `+` is a space, `%XX` a byte, the bytes read as UTF-8 (a broken sequence becomes U+FFFD, a `%` with no two hex digits stays)
fn decode(s: &str) -> String {
    let b = s.as_bytes();
    let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'+' => out.push(b' '),
            b'%' if i + 2 < b.len() && hex(b[i + 1]).is_some() && hex(b[i + 2]).is_some() => { out.push(hex(b[i + 1]).unwrap() * 16 + hex(b[i + 2]).unwrap()); i += 2; }
            c => out.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
/// `encodeURIComponent`: letters, digits and `-_.!~*'()` as they are, every other byte of the UTF-8 as `%XX`
fn encode(s: &str, out: &mut String) {
    for &c in s.as_bytes() {
        if c.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&c) { out.push(c as char); } else { out.push('%'); out.push(char::from_digit((c >> 4) as u32, 16).unwrap().to_ascii_uppercase()); out.push(char::from_digit((c & 15) as u32, 16).unwrap().to_ascii_uppercase()); }
    }
}


/// how much of `s` is a decimal number as JavaScript writes one (`+.5e1`, `-3`, `5.`, `Infinity`): its length in bytes, 0 = none
fn decimal_prefix(s: &str) -> usize {
    let b = s.as_bytes();
    let mut i = if matches!(b.first(), Some(b'+') | Some(b'-')) { 1 } else { 0 };
    if s[i..].starts_with("Infinity") { return i + 8; }
    let d0 = i;
    while i < b.len() && b[i].is_ascii_digit() { i += 1; }
    let whole = i - d0;
    let mut frac = 0;
    if i < b.len() && b[i] == b'.' { let f0 = i + 1; let mut j = f0; while j < b.len() && b[j].is_ascii_digit() { j += 1; } frac = j - f0; if whole + frac > 0 { i = j; } }
    if whole + frac == 0 { return 0; }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') { j += 1; }
        let e0 = j;
        while j < b.len() && b[j].is_ascii_digit() { j += 1; }
        if j > e0 { i = j; }
    }
    i
}
fn decimal(s: &str) -> f64 {
    match s.trim_start_matches('+') { "Infinity" => f64::INFINITY, "-Infinity" => f64::NEG_INFINITY, t => t.parse::<f64>().unwrap_or(f64::NAN) }
}
/// `Number(string)`: the language's own grammar, not Rust's (which takes 'inf' and 'nan', and refuses '' and hex).
pub fn number_of(s: &str) -> f64 {
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
pub fn is_js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{B}' | '\u{C}' | '\r' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}')
}
/// `parseFloat(text)`: leading blanks passed, then the decimal the text STARTS with (`2px` is 2); NaN when it starts with none
pub fn float_of(text: &str) -> f64 {
    let s = text.trim_start_matches(is_js_space);
    match decimal_prefix(s) { 0 => f64::NAN, n => decimal(&s[..n]) }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// each line was read off the browser's own `URLSearchParams` (bun's is the same one): the text, then `name=value` of every word in order
    #[test]
    fn reads_an_address_as_the_browser_does() {
        let words = |t: &str| Query::parse(t).words().iter().map(|(k, v)| alloc::format!("{}={}", k, v)).collect::<Vec<_>>().join("|");
        assert_eq!(words("?levelFile=depot.json&roomclear&squad4"), "levelFile=depot.json|roomclear=|squad4=");
        assert_eq!(words("levelFile=tut-kit.json&roomclear=1&squad4"), "levelFile=tut-kit.json|roomclear=1|squad4=");
        assert_eq!(words("?a=1&&b=x=y&=z&c="), "a=1|b=x=y|=z|c=");
        assert_eq!(words("?name=the+pass%20road&caf%C3%A9=%e2%9c%93&bad=100%&worse=%zz&cut=%4"), "name=the pass road|caf\u{e9}=\u{2713}|bad=100%|worse=%zz|cut=%4");
        assert_eq!(words(""), "");
        assert_eq!(words("?"), "");
    }
    #[test]
    fn answers_the_four_calls_the_game_makes() {
        let mut q = Query::parse("?quiet&squad=1&hour=dusk&squad=4");
        assert!(q.has("quiet") && !q.has("roomclear"));
        assert_eq!((q.get("quiet"), q.get("squad"), q.get("nofow")), (Some(""), Some("1"), None));
        q.set("squad", "2"); q.set("weather", "rain"); q.delete("hour");
        assert_eq!(q.bare(), "?quiet&squad=2&weather=rain");
        q.set("levelFile", "the pass.json");
        assert_eq!(q.bare(), "?quiet&squad=2&weather=rain&levelFile=the%20pass.json");
        assert_eq!(Query::parse("").bare(), "");
    }
    /// each row: the text, JavaScript's `Number(text)` and `parseFloat(text)` of it as the language prints them (read off bun)
    #[test]
    fn reads_a_word_as_a_number_as_the_language_does() {
        let rows: [(&str, &str, &str); 32] = [("2", "2", "2"), (" 2 ", "2", "2"), ("", "0", "NaN"), ("  ", "0", "NaN"), ("2.5", "2.5", "2.5"), (".5", "0.5", "0.5"), ("5.", "5", "5"), ("-3", "-3", "-3"),
            ("+4", "4", "4"), ("1e3", "1000", "1000"), ("1E-2", "0.01", "0.01"), ("0x10", "16", "0"), ("0b11", "3", "0"), ("0o17", "15", "0"), ("Infinity", "Infinity", "Infinity"),
            ("-Infinity", "-Infinity", "-Infinity"), ("inf", "NaN", "NaN"), ("nan", "NaN", "NaN"), ("NaN", "NaN", "NaN"), ("2px", "NaN", "2"), ("px2", "NaN", "NaN"), ("1.2.3", "NaN", "1.2"),
            ("1e", "NaN", "1"), ("--1", "NaN", "NaN"), ("0.5x", "NaN", "0.5"), ("1_0", "NaN", "1"), ("+.5e1", "5", "5"), (".", "NaN", "NaN"), ("-", "NaN", "NaN"), ("e5", "NaN", "NaN"),
            ("12abc", "NaN", "12"), (" 3.25cams", "NaN", "3.25")];
        let said = |x: f64| if x.is_nan() { String::from("NaN") } else if x.is_infinite() { String::from(if x > 0.0 { "Infinity" } else { "-Infinity" }) } else { alloc::format!("{}", x) };
        for (text, number, float) in rows {
            assert_eq!(said(number_of(text)), number, "Number({:?})", text);
            assert_eq!(said(float_of(text)), float, "parseFloat({:?})", text);
        }
        let q = Query::parse("?cams=3&sun=0.5x&none");
        assert_eq!((q.number("cams"), q.float("sun"), q.number("none"), q.number("gone")), (Some(3.0), Some(0.5), Some(0.0), None));
        assert!(q.number("sun").unwrap().is_nan());
    }
}
