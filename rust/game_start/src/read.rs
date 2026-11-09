//! A RECORD OF A LEVEL FILE AS A TYPE: the game's own where the game keeps the record (`records.rs`), the file's where it does not (`file.rs`). Each type says which member of the file each of its
//! fields is, a line a type, and this file reads the kinds of value a field can be.
//!
//! WHAT IS WRONG WITH A VALUE IS SAID AND THE READING GOES ON, so that a file is told every fault it has at once, each by its place: a blank value stands in for what was wrong, and whoever reads
//! takes the whole only where nothing was said. A member no type has a field for is not passed over in silence either: the file's tree remembers what was asked for (`json.rs`), and whoever reads the file names the rest.
use crate::json::{to_line, Json};
use alloc::{format, string::String, vec::Vec};
use core::cell::RefCell;
use core::fmt::Display;
use game::js::JsMap;

/// every fault found so far, in the order they were met
#[derive(Default)]
pub struct Faults(RefCell<Vec<String>>);
impl Faults {
    pub fn say(&self, place: impl Display, words: impl Display) { self.0.borrow_mut().push(format!("{place}: {words}")); }
    pub fn count(&self) -> usize { self.0.borrow().len() }
    pub fn all(self) -> Vec<String> { self.0.into_inner() }
}

/// A VALUE'S PLACE IN THE FILE, by the members and the items on the way to it, and where its faults go (nowhere: a blank is being made).
#[derive(Clone, Copy)]
pub struct At<'a> { up: Option<&'a At<'a>>, step: Step<'a>, faults: Option<&'a Faults> }
#[derive(Clone, Copy)]
enum Step<'a> { Top, Key(&'a str), Item(usize, Option<&'a str>) }
impl<'a> At<'a> {
    pub fn top(faults: &'a Faults) -> At<'a> { At { up: None, step: Step::Top, faults: Some(faults) } }
    pub fn key<'b>(&'b self, k: &'b str) -> At<'b> where 'a: 'b { At { up: Some(self), step: Step::Key(k), faults: self.faults } }
    /// the `i`th of a list; a record that has an id is called by it
    pub fn item<'b>(&'b self, i: usize, id: Option<&'b str>) -> At<'b> where 'a: 'b { At { up: Some(self), step: Step::Item(i, id), faults: self.faults } }
    pub fn place(&self) -> String {
        let up = self.up.map_or(String::new(), At::place);
        match self.step {
            Step::Top => up,
            Step::Key(k) => if up.is_empty() { String::from(k) } else { format!("{up}.{k}") },
            Step::Item(i, Some(id)) => format!("{up}[{i}] '{id}'"),
            Step::Item(i, None) => format!("{up}[{i}]"),
        }
    }
    pub fn fault(&self, words: &str) { if let Some(f) = self.faults { f.say(self.place(), words); } }
    /// the value here should be `want`
    #[cold]
    pub fn wants(&self, want: &str, j: &Json) {
        // (a member left out; and what stands under a value that is none of its place's is nobody's to read)
        if j.is_null() && !matches!(self.step, Step::Item(..)) { return self.fault(&format!("should be {want}, and the file says nothing")); }
        j.skip_all();
        let has = to_line(j);
        let short: String = has.chars().take(60).collect();
        self.fault(&format!("should be {want}, and the file has {short}{}", if short.len() < has.len() { "…" } else { "" }));
    }
    /// the place of a record: where `j` is none, that alone is said
    pub fn of_record(&self, j: &Json) -> At<'a> {
        if matches!(j, Json::Obj(..)) { return *self; }
        self.wants("a record", j);
        At { faults: None, ..*self }
    }
    /// …one of these words
    #[cold]
    pub fn wants_word(&self, words: &[&str], j: &Json) { self.wants(&format!("one of {}", words.join(", ")), j); }
}

pub trait FromJson: Sized {
    /// the value at `at` as this type; where it is not one its faults are said, and what is wrong is blank
    fn from_json(j: &Json, at: &At) -> Self;
}
static NOTHING: Json = Json::Null;
/// a value with nothing in it, for a place whose value was wrong
pub fn blank<T: FromJson>() -> T { T::from_json(&NOTHING, &At { up: None, step: Step::Top, faults: None }) }

impl FromJson for f64 { fn from_json(j: &Json, at: &At) -> Self { j.num().unwrap_or_else(|| { at.wants("a number", j); 0.0 }) } }
impl FromJson for f32 { fn from_json(j: &Json, at: &At) -> Self { f64::from_json(j, at) as f32 } }
impl FromJson for bool { fn from_json(j: &Json, at: &At) -> Self { j.bool().unwrap_or_else(|| { at.wants("true or false", j); false }) } }
impl FromJson for String { fn from_json(j: &Json, at: &At) -> Self { String::from(j.str().unwrap_or_else(|| { at.wants("text", j); "" })) } }
impl<T: FromJson> FromJson for Option<T> { fn from_json(j: &Json, at: &At) -> Self { if j.is_null() { None } else { Some(T::from_json(j, at)) } } }
impl<T: FromJson> FromJson for Vec<T> {
    fn from_json(j: &Json, at: &At) -> Self {
        if !matches!(j, Json::Arr(_)) { at.wants("a list", j); }
        j.items().iter().enumerate().map(|(i, x)| T::from_json(x, &at.item(i, x.peek("id").and_then(Json::str)))).collect()
    }
}
/// a list of so many; what is no such list is wrong as a whole, and its items are not read
fn so_many<'j>(j: &'j Json, n: usize, at: &At) -> &'j [Json] {
    if j.items().len() == n { return j.items(); }
    at.wants(&format!("a list of {n}"), j);
    &[]
}
impl<T: FromJson, const N: usize> FromJson for [T; N] {
    fn from_json(j: &Json, at: &At) -> Self {
        let a = so_many(j, N, at);
        core::array::from_fn(|i| a.get(i).map_or_else(blank, |x| T::from_json(x, &at.item(i, x.peek("id").and_then(Json::str)))))
    }
}
impl<A: FromJson, B: FromJson> FromJson for (A, B) {
    fn from_json(j: &Json, at: &At) -> Self {
        match so_many(j, 2, at) { [a, b] => (A::from_json(a, &at.item(0, None)), B::from_json(b, &at.item(1, None))), _ => (blank(), blank()) }
    }
}
/// an object used as a table by name (`{ [place: string]: Card }`): its members in the file's order
impl<T: FromJson> FromJson for JsMap<String, T> {
    fn from_json(j: &Json, at: &At) -> Self {
        if !matches!(j, Json::Obj(..)) { at.wants("a table by name", j); }
        let mut out = JsMap::new();
        for (k, v) in j.members() { out.set(k.clone(), T::from_json(v, &at.key(k))); }
        out
    }
}

/// the member `key` of the record `j` as a field's value. A member a record does not say is left out: null is not a way to say nothing
#[inline(never)]
pub fn field<T: FromJson>(j: &Json, key: &str, at: &At) -> T {
    let (v, at) = (j.get(key), at.key(key));
    if v.is_null() && j.peek(key).is_some() { at.fault("null says nothing: the member is left out"); return blank(); }
    T::from_json(v, &at)
}
/// …of a member whose null is a word (none), which is always said
pub fn or_null<T: FromJson>(j: &Json, key: &str, at: &At) -> Option<T> {
    if j.peek(key).is_none() { at.key(key).fault("is said, or null"); }
    FromJson::from_json(j.get(key), &at.key(key))
}
/// A TYPE THAT IS A RECORD OF THE FILE: each field and the member it is. `drops {..}`: members of the file the game leaves behind by rule (an author's notes, a floor), each read as what it is and let go.
/// `nulls {..}`: fields whose null is a word, which the file always says.
#[macro_export]
macro_rules! json_struct {
    ($t:ty { $($f:ident: $k:literal),* $(,)? } $(drops { $($d:literal: $dt:ty),* })? $(nulls { $($nf:ident: $nk:literal),* })?) => {
        impl $crate::read::FromJson for $t {
            fn from_json(j: &$crate::json::Json, at: &$crate::read::At) -> Self {
                let at = &at.of_record(j);
                $($($crate::read::field::<$dt>(j, $d, at);)*)?
                Self { $($f: $crate::read::field(j, $k, at),)* $($($nf: $crate::read::or_null(j, $nk, at),)*)? }
            }
        }
    };
}
/// A TYPE THAT IS ONE OF A FEW WORDS. (The first is its blank.)
#[macro_export]
macro_rules! json_words {
    ($t:ty { $v0:ident: $w0:literal $(, $v:ident: $w:literal)* $(,)? }) => {
        impl $crate::read::FromJson for $t {
            fn from_json(j: &$crate::json::Json, at: &$crate::read::At) -> Self {
                match j.str() { Some($w0) => <$t>::$v0, $(Some($w) => <$t>::$v,)* _ => { at.wants_word(&[$w0 $(, $w)*], j); <$t>::$v0 } }
            }
        }
    };
}
