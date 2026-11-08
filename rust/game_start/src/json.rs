//! A JSON reader: text in, a tree of values out. A level file and the character's files are JSON, and the start reads them as the TypeScript start
//! reads a parsed object: by key, in the file's order. Numbers are read by the core library's decimal reader, which rounds as the language's own
//! `JSON.parse` does (to the nearest double), so a number here is the number there to the bit.
//!
//! AN OBJECT REMEMBERS WHICH OF ITS MEMBERS WERE ASKED FOR (`get`, `has`, `members`; `skip` for a member passed by on purpose). So whoever has
//! read a file can ask what it did NOT read (`unread`): a member the reader has no line for is found by the reading itself, with no second
//! list of a record's members to keep beside each reader.
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    /// an object's members in the file's order (what `Object.keys` walks for keys that are not whole numbers), and which were asked for
    Obj(Vec<(String, Json)>, Asked),
}

/// which members of an object a reader asked for, by their place (one flag a member, made with the object, and made again as `set_member` adds one). Not part of the value: two objects are equal by their members alone.
#[derive(Debug, Default)]
pub struct Asked(Vec<AtomicBool>);
impl PartialEq for Asked { fn eq(&self, _: &Asked) -> bool { true } }
impl Clone for Asked { fn clone(&self) -> Asked { Asked(self.0.iter().map(|b| AtomicBool::new(b.load(Relaxed))).collect()) } }
impl Asked {
    fn of(members: usize) -> Asked { Asked((0..members).map(|_| AtomicBool::new(false)).collect()) }
    fn mark(&self, i: usize) { if let Some(b) = self.0.get(i) { b.store(true, Relaxed); } }
    fn is(&self, i: usize) -> bool { self.0.get(i).map_or(false, |b| b.load(Relaxed)) }
}

static NULL: Json = Json::Null;

impl Json {
    /// the member `k` of an object; `Null` where this is no object or has no such member (so a chain of asks reads like `a?.b?.c`)
    pub fn get(&self, k: &str) -> &Json {
        match self { Json::Obj(m, asked) => match m.iter().position(|(n, _)| n == k) { Some(i) => { asked.mark(i); &m[i].1 } None => &NULL }, _ => &NULL }
    }
    /// the item `i` of an array; `Null` past its end or where this is no array
    pub fn at(&self, i: usize) -> &Json {
        match self { Json::Arr(a) => a.get(i).unwrap_or(&NULL), _ => &NULL }
    }
    /// is the member there (`k in o`, with `undefined` never written: a JSON file has no such value)
    pub fn has(&self, k: &str) -> bool { !self.get(k).is_null() || matches!(self, Json::Obj(m, _) if m.iter().any(|(n, _)| n == k)) }
    pub fn is_null(&self) -> bool { matches!(self, Json::Null) }
    pub fn num(&self) -> Option<f64> { match self { Json::Num(x) => Some(*x), _ => None } }
    pub fn bool(&self) -> Option<bool> { match self { Json::Bool(x) => Some(*x), _ => None } }
    pub fn str(&self) -> Option<&str> { match self { Json::Str(s) => Some(s.as_str()), _ => None } }
    /// an array's items; none where this is no array
    pub fn items(&self) -> &[Json] { match self { Json::Arr(a) => a.as_slice(), _ => &[] } }
    /// an object's members in the file's order; none where this is no object
    pub fn members(&self) -> &[(String, Json)] { match self { Json::Obj(m, asked) => { for i in 0..m.len() { asked.mark(i); } m.as_slice() } _ => &[] } }
    /// THESE MEMBERS ARE PASSED BY ON PURPOSE (an author's notes, an id beside a name), with all beneath them: asked for, as far as `unread` goes
    pub fn skip(&self, keys: &[&str]) { for k in keys { self.get(k).skip_all(); } }
    /// …and everything under this value is (a table of which only the rows in use are read, a record a switch leaves out)
    pub fn skip_all(&self) {
        match self { Json::Obj(..) => for (_, v) in self.members() { v.skip_all(); }, Json::Arr(a) => for v in a { v.skip_all(); }, _ => {} }
    }
    /// EVERY MEMBER UNDER THIS VALUE THAT NOBODY ASKED FOR, by its place (`at` = this value's own place). A member not asked for is named and
    /// not looked into; one asked for is looked into.
    pub fn unread(&self, at: &str, out: &mut Vec<String>) {
        match self {
            Json::Obj(m, asked) => for (i, (k, v)) in m.iter().enumerate() {
                let place = if at.is_empty() { k.clone() } else { alloc::format!("{}.{}", at, k) };
                if asked.is(i) { v.unread(&place, out); } else { out.push(place); }
            },
            Json::Arr(a) => for (i, v) in a.iter().enumerate() { v.unread(&alloc::format!("{}[{}]", at, i), out); },
            _ => {}
        }
    }
    /// an array of numbers
    pub fn nums(&self) -> Vec<f64> { self.items().iter().map(|x| x.num().expect("json: a number")).collect() }
    /// three numbers
    pub fn v3(&self) -> Option<[f64; 3]> { let a = self.items(); if a.len() == 3 { Some([a[0].num()?, a[1].num()?, a[2].num()?]) } else { None } }
    /// the member `k` takes this value, in its place where it has one, at the end where it has none (as the language's spread and overwrite keep a key's place)
    pub fn set_member(&mut self, k: &str, v: Json) {
        if let Json::Obj(m, asked) = self {
            match m.iter_mut().find(|(n, _)| n == k) {
                Some(e) => e.1 = v,
                None => {
                    m.push((String::from(k), v));
                    let more = Asked::of(m.len());
                    for i in 0..m.len() - 1 { if asked.is(i) { more.mark(i); } }
                    *asked = more;
                }
            }
        }
    }
    /// a copy with the member `k` gone
    pub fn without(&self, k: &str) -> Json {
        match self { Json::Obj(m, _) => { let kept: Vec<(String, Json)> = m.iter().filter(|(n, _)| n != k).cloned().collect(); let asked = Asked::of(kept.len()); Json::Obj(kept, asked) } v => v.clone() }
    }
}

/// The text as a value. A text that is not JSON is refused with where it stopped.
pub fn parse(text: &str) -> Result<Json, String> {
    let mut p = P { b: text.as_bytes(), i: 0 };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.b.len() { return Err(p.err("text after the value")); }
    Ok(v)
}

struct P<'a> { b: &'a [u8], i: usize }

impl<'a> P<'a> {
    fn err(&self, what: &str) -> String { alloc::format!("json: {} at byte {}", what, self.i) }
    fn ws(&mut self) { while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\n' | b'\r') { self.i += 1; } }
    fn eat(&mut self, w: &[u8]) -> bool { if self.b[self.i..].starts_with(w) { self.i += w.len(); true } else { false } }
    fn value(&mut self) -> Result<Json, String> {
        match self.b.get(self.i).copied() {
            None => Err(self.err("the text ends where a value should stand")),
            Some(b'{') => {
                self.i += 1;
                let mut m: Vec<(String, Json)> = Vec::new();
                self.ws();
                if self.eat(b"}") { let asked = Asked::of(m.len()); return Ok(Json::Obj(m, asked)); }
                loop {
                    self.ws();
                    if self.b.get(self.i) != Some(&b'"') { return Err(self.err("a key")); }
                    let k = self.string()?;
                    self.ws();
                    if !self.eat(b":") { return Err(self.err("a colon")); }
                    self.ws();
                    let v = self.value()?;
                    // (a key written twice: the last one stands, in the first one's place, as the language's own reader has it)
                    match m.iter_mut().find(|(n, _)| *n == k) { Some(e) => e.1 = v, None => m.push((k, v)) }
                    self.ws();
                    if self.eat(b",") { continue; }
                    if self.eat(b"}") { let asked = Asked::of(m.len()); return Ok(Json::Obj(m, asked)); }
                    return Err(self.err("a comma or the object's end"));
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut a: Vec<Json> = Vec::new();
                self.ws();
                if self.eat(b"]") { return Ok(Json::Arr(a)); }
                loop {
                    self.ws();
                    a.push(self.value()?);
                    self.ws();
                    if self.eat(b",") { continue; }
                    if self.eat(b"]") { return Ok(Json::Arr(a)); }
                    return Err(self.err("a comma or the array's end"));
                }
            }
            Some(b'"') => Ok(Json::Str(self.string()?)),
            Some(b't') => if self.eat(b"true") { Ok(Json::Bool(true)) } else { Err(self.err("a value")) },
            Some(b'f') => if self.eat(b"false") { Ok(Json::Bool(false)) } else { Err(self.err("a value")) },
            Some(b'n') => if self.eat(b"null") { Ok(Json::Null) } else { Err(self.err("a value")) },
            Some(_) => {
                let s = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') { self.i += 1; }
                let t = core::str::from_utf8(&self.b[s..self.i]).map_err(|_| self.err("a number"))?;
                t.parse::<f64>().map(Json::Num).map_err(|_| self.err("a number"))
            }
        }
    }
    fn hex4(&mut self) -> Result<u32, String> {
        let t = self.b.get(self.i..self.i + 4).and_then(|h| core::str::from_utf8(h).ok()).and_then(|h| u32::from_str_radix(h, 16).ok());
        match t { Some(v) => { self.i += 4; Ok(v) } None => Err(self.err("four hex digits")) }
    }
    fn string(&mut self) -> Result<String, String> {
        self.i += 1;
        let mut out: Vec<u8> = Vec::new();
        loop {
            // (a run with no escape is copied whole: the character's files are megabytes of it)
            let s = self.i;
            while self.i < self.b.len() && self.b[self.i] != b'"' && self.b[self.i] != b'\\' { self.i += 1; }
            out.extend_from_slice(&self.b[s..self.i]);
            match self.b.get(self.i).copied() {
                None => return Err(self.err("the text ends inside a string")),
                Some(b'"') => { self.i += 1; break; }
                _ => {
                    self.i += 1;
                    let c = self.b.get(self.i).copied().ok_or_else(|| self.err("the text ends inside a string"))?;
                    self.i += 1;
                    let ch: char = match c {
                        b'"' => '"', b'\\' => '\\', b'/' => '/', b'b' => '\u{8}', b'f' => '\u{c}', b'n' => '\n', b'r' => '\r', b't' => '\t',
                        b'u' => {
                            let mut u = self.hex4()?;
                            if (0xd800..0xdc00).contains(&u) && self.eat(b"\\u") { let lo = self.hex4()?; u = 0x10000 + ((u - 0xd800) << 10) + (lo.wrapping_sub(0xdc00) & 0x3ff); }
                            char::from_u32(u).unwrap_or('\u{fffd}')
                        }
                        _ => return Err(self.err("an escape")),
                    };
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
                }
            }
        }
        String::from_utf8(out).map_err(|_| self.err("a string that is not UTF-8"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_what_a_level_file_holds() {
        let v = parse(r#"{ "a": [1, -2.5e3, 12.259999999999998], "s": "q\"é\n", "t": true, "n": null, "o": {} }"#).unwrap();
        assert_eq!(v.get("a").nums(), [1.0, -2500.0, 12.259999999999998]);
        assert_eq!(v.get("s").str(), Some("q\"\u{e9}\n"));
        assert_eq!(v.get("t").bool(), Some(true));
        assert!(v.get("n").is_null() && v.has("n") && !v.has("zz") && v.get("o").members().is_empty());
        assert!(parse("[1,]").is_err() && parse("{} x").is_err());
    }
    #[test]
    fn says_what_nobody_asked_for() {
        let v = parse(r#"{ "name": "x", "boxes": [{ "c": 1, "tint": 2 }, { "c": 3 }], "later": { "deep": 1 }, "notes": "n" }"#).unwrap();
        v.get("name"); v.skip(&["notes"]);
        for b in v.get("boxes").items() { b.get("c"); }
        let mut left = Vec::new(); v.unread("", &mut left);
        assert_eq!(left, ["boxes[0].tint", "later"]);
        v.get("later").skip_all();
        let mut left = Vec::new(); v.unread("", &mut left);
        assert_eq!(left, ["boxes[0].tint"]);
    }
}

// ---- writing: a value as a level file's text

/// a number as the language writes it where that is a whole number, else its shortest text that reads back the same
fn num_text(x: f64, out: &mut String) {
    use core::fmt::Write;
    if !x.is_finite() { out.push_str("null"); return; }
    let _ = write!(out, "{}", x);
}
fn str_text(s: &str, out: &mut String) {
    use core::fmt::Write;
    out.push('"');
    for c in s.chars() {
        match c { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), '\n' => out.push_str("\\n"), '\r' => out.push_str("\\r"), '\t' => out.push_str("\\t"), c if (c as u32) < 0x20 => { let _ = write!(out, "\\u{:04x}", c as u32); } c => out.push(c) }
    }
    out.push('"');
}
fn value_text(v: &Json, depth: usize, out: &mut String) {
    match v {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Num(x) => num_text(*x, out),
        Json::Str(s) => str_text(s, out),
        Json::Arr(a) => {
            // (a list of records: one to a line, as a level file is kept; a list of numbers on one line)
            let lines = depth < 2 && a.iter().any(|x| matches!(x, Json::Obj(..)));
            out.push('[');
            for (i, x) in a.iter().enumerate() { if i > 0 { out.push_str(if lines { "," } else { ", " }); } if lines { out.push_str("\n    "); } value_text(x, depth + 1, out); }
            if lines && !a.is_empty() { out.push_str("\n  "); }
            out.push(']');
        }
        Json::Obj(m, _) => {
            let lines = depth == 0;
            out.push('{');
            for (i, (k, x)) in m.iter().enumerate() { if i > 0 { out.push_str(if lines { "," } else { ", " }); } if lines { out.push_str("\n  "); } str_text(k, out); out.push_str(": "); value_text(x, depth + 1, out); }
            if lines { out.push('\n'); }
            out.push('}');
        }
    }
}
/// The value as text: an object's members in their order, a top-level list a record to a line.
pub fn to_text(v: &Json) -> String { let mut out = String::new(); value_text(v, 0, &mut out); out.push('\n'); out }

/// The value as one line of text, with no spaces.
pub fn to_line(v: &Json) -> String { let mut out = String::new(); line_text(v, &mut out); out }
fn line_text(v: &Json, out: &mut String) {
    match v {
        Json::Arr(a) => { out.push('['); for (i, x) in a.iter().enumerate() { if i > 0 { out.push(','); } line_text(x, out); } out.push(']'); }
        Json::Obj(m, _) => { out.push('{'); for (i, (k, x)) in m.iter().enumerate() { if i > 0 { out.push(','); } str_text(k, out); out.push(':'); line_text(x, out); } out.push('}'); }
        v => value_text(v, 0, out),
    }
}

/// short hands for making a value
pub fn n(x: f64) -> Json { Json::Num(x) }
pub fn s(x: &str) -> Json { Json::Str(String::from(x)) }
pub fn b(x: bool) -> Json { Json::Bool(x) }
pub fn nums(x: &[f64]) -> Json { Json::Arr(x.iter().map(|v| Json::Num(*v)).collect()) }
pub fn arr(x: Vec<Json>) -> Json { Json::Arr(x) }
pub fn obj(m: Vec<(&str, Json)>) -> Json { let asked = Asked::of(m.len()); Json::Obj(m.into_iter().map(|(k, v)| (String::from(k), v)).collect(), asked) }
