//! A RECORD OF A LEVEL FILE AS THE GAME'S OWN TYPE. Where the TypeScript loader keeps a block of the file verbatim (`scenarios`: "the runtime
//! structs verbatim"), the Rust `Level` holds it typed, so each type says which member of the file each of its fields is (`records.rs`, a line a
//! type) and this file reads the kinds of value a field can be. A member the type has no field for is NOT passed over in silence: the file's
//! tree remembers what was asked for (`json.rs`), and whoever loads the file names the rest.
use crate::json::Json;
use alloc::{format, string::String, vec::Vec};
use game::js::JsMap;

pub trait FromJson: Sized {
    /// the value at `at` (its place in the file, for a refusal's words) as this type
    fn from_json(j: &Json, at: &str) -> Self;
}
fn refuse(at: &str, want: &str, j: &Json) -> ! { panic!("level: {} should be {}, and the file has {:?}", at, want, j) }

impl FromJson for f64 { fn from_json(j: &Json, at: &str) -> Self { j.num().unwrap_or_else(|| refuse(at, "a number", j)) } }
impl FromJson for f32 { fn from_json(j: &Json, at: &str) -> Self { j.num().unwrap_or_else(|| refuse(at, "a number", j)) as f32 } }
// (a member left out is false: where the TypeScript says `quick?: boolean` the game's type holds a plain bool, as the bridge reads it)
impl FromJson for bool { fn from_json(j: &Json, at: &str) -> Self { if j.is_null() { return false; } j.bool().unwrap_or_else(|| refuse(at, "true or false", j)) } }
impl FromJson for String { fn from_json(j: &Json, at: &str) -> Self { String::from(j.str().unwrap_or_else(|| refuse(at, "text", j))) } }
impl<T: FromJson> FromJson for Option<T> { fn from_json(j: &Json, at: &str) -> Self { if j.is_null() { None } else { Some(T::from_json(j, at)) } } }
impl<T: FromJson> FromJson for Vec<T> {
    fn from_json(j: &Json, at: &str) -> Self {
        match j { Json::Arr(a) => a.iter().enumerate().map(|(i, x)| T::from_json(x, &format!("{}[{}]", at, i))).collect(), _ => refuse(at, "a list", j) }
    }
}
impl<T: FromJson, const N: usize> FromJson for [T; N] {
    fn from_json(j: &Json, at: &str) -> Self {
        match j { Json::Arr(a) if a.len() == N => core::array::from_fn(|i| T::from_json(&a[i], &format!("{}[{}]", at, i))), _ => refuse(at, &format!("a list of {}", N), j) }
    }
}
impl<A: FromJson, B: FromJson> FromJson for (A, B) {
    fn from_json(j: &Json, at: &str) -> Self {
        match j { Json::Arr(a) if a.len() == 2 => (A::from_json(&a[0], &format!("{}[0]", at)), B::from_json(&a[1], &format!("{}[1]", at))), _ => refuse(at, "a pair", j) }
    }
}
/// an object used as a table by name (`{ [place: string]: Card }`): its members in the file's order
impl<T: FromJson> FromJson for JsMap<String, T> {
    fn from_json(j: &Json, at: &str) -> Self {
        if !matches!(j, Json::Obj(..)) { refuse(at, "a table by name", j) }
        let mut out = JsMap::new();
        for (k, v) in j.members() { out.set(k.clone(), T::from_json(v, &format!("{}.{}", at, k))); }
        out
    }
}

/// the member `key` of the record `j` as a field's value
pub fn field<T: FromJson>(j: &Json, key: &str, at: &str) -> T { T::from_json(j.get(key), &format!("{}.{}", at, key)) }
/// `j` is a record
pub fn record(j: &Json, at: &str) { if !matches!(j, Json::Obj(..)) { refuse(at, "a record", j) } }

/// A TYPE THAT IS A RECORD OF THE FILE: each field and the member it is. `drops [..]`: members of the file the loader leaves behind by rule
/// (an id beside a name, an author's notes, a floor: levelLoad.ts).
#[macro_export]
macro_rules! json_struct {
    ($t:ty { $($f:ident: $k:literal),* $(,)? } $(drops [$($d:literal),*])?) => {
        impl $crate::read::FromJson for $t {
            fn from_json(j: &$crate::json::Json, at: &str) -> Self {
                $crate::read::record(j, at);
                $(j.skip(&[$($d),*]);)?
                Self { $($f: $crate::read::field(j, $k, at)),* }
            }
        }
    };
}
/// A TYPE THAT IS ONE OF A FEW WORDS.
#[macro_export]
macro_rules! json_words {
    ($t:ty { $($v:ident: $w:literal),* $(,)? }) => {
        impl $crate::read::FromJson for $t {
            fn from_json(j: &$crate::json::Json, at: &str) -> Self {
                match j.str() { $(Some($w) => <$t>::$v,)* _ => panic!("level: {} should be one of {:?}, and the file has {:?}", at, [$($w),*], j) }
            }
        }
    };
}
