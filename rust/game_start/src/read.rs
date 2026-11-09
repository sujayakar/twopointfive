//! A RECORD OF A LEVEL FILE AS A TYPE: the game's own where the game keeps the record (`records.rs`), the file's where it does not (`file.rs`). Each type says which member of the file each of its
//! fields is, a line a type, and this file reads the kinds of value a field can be.
//!
//! WHAT IS WRONG WITH A VALUE IS SAID AND THE READING GOES ON, so that a file is told every fault it has at once, each by its place. A member no type has a field for is not passed over in silence
//! either: the file's tree remembers what was asked for (`json.rs`), and whoever reads the file names the rest.
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

/// A VALUE'S PLACE IN THE FILE, by the members and the items on the way to it, and where its faults go.
#[derive(Clone, Copy)]
pub struct At<'a> { up: Option<&'a At<'a>>, step: Step<'a>, faults: &'a Faults }
#[derive(Clone, Copy)]
enum Step<'a> { Top, Key(&'a str), Item(usize, Option<&'a str>) }
impl<'a> At<'a> {
    pub fn top(faults: &'a Faults) -> At<'a> { At { up: None, step: Step::Top, faults } }
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
    pub fn fault(&self, words: impl Display) { self.faults.say(self.place(), words); }
    /// the value here should be `want`
    pub fn wants<T>(&self, want: impl Display, j: &Json) -> Option<T> {
        // (a member left out; and what stands under a value that is none of its place's is nobody's to read)
        if j.is_null() && !matches!(self.step, Step::Item(..)) { self.fault(format!("should be {want}, and the file says nothing")); return None; }
        j.skip_all();
        let has = to_line(j);
        let short: String = has.chars().take(60).collect();
        self.fault(format!("should be {want}, and the file has {short}{}", if short.len() < has.len() { "…" } else { "" }));
        None
    }
}

pub trait FromJson: Sized {
    /// the value at `at` as this type; none, with its faults said, where it is not one
    fn from_json(j: &Json, at: &At) -> Option<Self>;
}

impl FromJson for f64 { fn from_json(j: &Json, at: &At) -> Option<Self> { j.num().or_else(|| at.wants("a number", j)) } }
impl FromJson for f32 { fn from_json(j: &Json, at: &At) -> Option<Self> { f64::from_json(j, at).map(|x| x as f32) } }
impl FromJson for bool { fn from_json(j: &Json, at: &At) -> Option<Self> { j.bool().or_else(|| at.wants("true or false", j)) } }
impl FromJson for String { fn from_json(j: &Json, at: &At) -> Option<Self> { j.str().map(String::from).or_else(|| at.wants("text", j)) } }
impl<T: FromJson> FromJson for Option<T> { fn from_json(j: &Json, at: &At) -> Option<Self> { if j.is_null() { Some(None) } else { T::from_json(j, at).map(Some) } } }
impl<T: FromJson> FromJson for Vec<T> {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        let Json::Arr(a) = j else { return at.wants("a list", j) };
        // (every item is read, so that each says its own faults)
        let read: Vec<Option<T>> = a.iter().enumerate().map(|(i, x)| T::from_json(x, &at.item(i, x.peek("id").and_then(Json::str)))).collect();
        read.into_iter().collect()
    }
}
impl<T: FromJson, const N: usize> FromJson for [T; N] {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        if j.items().len() != N { return at.wants(format!("a list of {N}"), j); }
        let read: Option<Vec<T>> = FromJson::from_json(j, at);
        read?.try_into().ok()
    }
}
impl<A: FromJson, B: FromJson> FromJson for (A, B) {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        if j.items().len() != 2 { return at.wants("a pair", j); }
        let (a, b) = (A::from_json(j.at(0), &at.item(0, None)), B::from_json(j.at(1), &at.item(1, None)));
        Some((a?, b?))
    }
}
/// an object used as a table by name (`{ [place: string]: Card }`): its members in the file's order
impl<T: FromJson> FromJson for JsMap<String, T> {
    fn from_json(j: &Json, at: &At) -> Option<Self> {
        if !matches!(j, Json::Obj(..)) { return at.wants("a table by name", j); }
        let (mut out, mut whole) = (JsMap::new(), true);
        for (k, v) in j.members() {
            match T::from_json(v, &at.key(k)) { Some(t) => { out.set(k.clone(), t); } None => whole = false }
        }
        whole.then_some(out)
    }
}

/// the member `key` of the record `j` as a field's value. A member a record does not say is left out: null is not a way to say nothing
pub fn field<T: FromJson>(j: &Json, key: &str, at: &At) -> Option<T> {
    let v = j.get(key);
    if v.is_null() && j.peek(key).is_some() { at.key(key).fault("null says nothing: the member is left out"); return None; }
    T::from_json(v, &at.key(key))
}
/// …of a member whose null is a word (none), which is always said
pub fn or_null<T: FromJson>(j: &Json, key: &str, at: &At) -> Option<Option<T>> {
    if j.peek(key).is_none() { at.key(key).fault("is said, or null"); return None; }
    FromJson::from_json(j.get(key), &at.key(key))
}
/// `j` is a record
pub fn record(j: &Json, at: &At) -> Option<()> { if matches!(j, Json::Obj(..)) { Some(()) } else { at.wants("a record", j) } }

/// A TYPE THAT IS A RECORD OF THE FILE: each field and the member it is. `drops {..}`: members of the file the game leaves behind by rule (an author's notes, a floor), each read as what it is and let go.
/// `nulls {..}`: fields whose null is a word, which the file always says.
#[macro_export]
macro_rules! json_struct {
    ($t:ty { $($f:ident: $k:literal),* $(,)? } $(drops { $($d:literal: $dt:ty),* })? $(nulls { $($nf:ident: $nk:literal),* })?) => {
        impl $crate::read::FromJson for $t {
            fn from_json(j: &$crate::json::Json, at: &$crate::read::At) -> Option<Self> {
                $crate::read::record(j, at)?;
                $(let $f = $crate::read::field(j, $k, at);)*
                $($(let $nf = $crate::read::or_null(j, $nk, at);)*)?
                let whole = true $($(& $crate::read::field::<$dt>(j, $d, at).is_some())*)?;
                if !whole { return None; }
                Some(Self { $($f: $f?,)* $($($nf: $nf?),*)? })
            }
        }
    };
}
/// A TYPE THAT IS ONE OF A FEW WORDS.
#[macro_export]
macro_rules! json_words {
    ($t:ty { $($v:ident: $w:literal),* $(,)? }) => {
        impl $crate::read::FromJson for $t {
            fn from_json(j: &$crate::json::Json, at: &$crate::read::At) -> Option<Self> {
                match j.str() { $(Some($w) => Some(<$t>::$v),)* _ => at.wants(alloc::format!("one of {}", [$($w),*].join(", ")), j) }
            }
        }
    };
}
