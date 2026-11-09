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
    pub fn number(&self, name: &str) -> Option<f64> { self.get(name).map(crate::js::number_of) }
    /// `parseFloat(q.get(name))` where the name is there (the number the text starts with: `float_of`)
    pub fn float(&self, name: &str) -> Option<f64> { self.get(name).map(crate::js::float_of) }
    /// the words as text again, a bare flag left bare (`?mute`, not `?mute=`): scene/scenarioList.ts `bare`. No words = the empty text.
    pub fn bare(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.pairs { out.push(if out.is_empty() { '?' } else { '&' }); encode(k, &mut out); if !v.is_empty() { out.push('='); encode(v, &mut out); } }
        out
    }
}

/// `encodeURIComponent(text)`
pub fn encoded(s: &str) -> String { let mut out = String::new(); encode(s, &mut out); out }

/// `decodeURIComponent(text)`: every `%` starts a whole sequence of good UTF-8, a `+` is a plus; None where the language throws
pub fn decoded(s: &str) -> Option<String> {
    let b = s.as_bytes();
    let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'%' { out.push(b[i]); i += 1; continue; }
        out.push(hex(*b.get(i + 1)?)? * 16 + hex(*b.get(i + 2)?)?);
        i += 3;
    }
    String::from_utf8(out).ok()
}

/// the part of an address after its `?` (`text.replace(/^[^?]*\?/, '')`): all of it where it has none
pub fn after_mark(s: &str) -> &str { s.split_once('?').map_or(s, |(_, q)| q) }

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
    /// what the language's `decodeURIComponent` and `encodeURIComponent` answer
    #[test]
    fn decodes_and_encodes_a_side_as_the_language_does() {
        assert_eq!(decoded("a+b%20c%C3%A9").as_deref(), Some("a+b c\u{e9}"));
        assert_eq!((decoded("100%"), decoded("%zz"), decoded("%C3"), decoded("%4")), (None, None, None, None));
        assert_eq!(encoded("a b+c/\u{e9}-_.!~*'()"), "a%20b%2Bc%2F%C3%A9-_.!~*'()");
        assert_eq!((after_mark("/x?a?b"), after_mark("a=1")), ("a?b", "a=1"));
    }
    /// a word read as a number is the language's reading of it (js.rs), and a name that is not there is no number
    #[test]
    fn reads_a_word_as_a_number() {
        let q = Query::parse("?cams=3&sun=0.5x&none");
        assert_eq!((q.number("cams"), q.float("sun"), q.number("none"), q.number("gone")), (Some(3.0), Some(0.5), Some(0.0), None));
        assert!(q.number("sun").unwrap().is_nan());
    }
}
