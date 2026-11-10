//! THE STATE AND THE VIEWS AS BYTES, one way: a type says its own where it is declared (`#[derive(ToBytes)]`, rust/game_derive). Written to be told to a host or to be
//! hashed; nothing reads the state back in.
use crate::ids::{AnyBoxId, Id, Tab};
use crate::js::{JsMap, JsSet};
use alloc::{boxed::Box, collections::VecDeque, string::String, vec::Vec};
use core::hash::Hash;
pub use game_derive::ToBytes;

#[derive(Default)]
pub struct W { pub b: Vec<u8> }
impl W { pub const fn new() -> W { W { b: Vec::new() } } }

/// numbers little end first; a text, a list, a set and a map by how many and then each, in their own order; an option and a yes or no as a byte
pub trait ToBytes { fn w(&self, o: &mut W); }
/// a struct's fields one at a time, by name
pub trait Parts { const NAMES: &'static [&'static str]; fn parts(&self, each: &mut dyn FnMut(&'static str, &dyn ToBytes)); }

macro_rules! numbers { ($($t:ty)*) => { $(impl ToBytes for $t { #[inline] fn w(&self, o: &mut W) { o.b.extend_from_slice(&self.to_le_bytes()); } })* } }
numbers!(f64 f32 u8 i8 u16 i16 u32 i32 u64 i64);
impl ToBytes for usize { #[inline] fn w(&self, o: &mut W) { (*self as u32).w(o) } }
impl ToBytes for bool { #[inline] fn w(&self, o: &mut W) { o.b.push(*self as u8) } }
impl ToBytes for () { fn w(&self, _: &mut W) {} }
impl ToBytes for str { fn w(&self, o: &mut W) { self.len().w(o); o.b.extend_from_slice(self.as_bytes()); } }
impl ToBytes for String { fn w(&self, o: &mut W) { self.as_str().w(o) } }
impl<T: ToBytes + ?Sized> ToBytes for &T { fn w(&self, o: &mut W) { (**self).w(o) } }
impl<T: ToBytes + ?Sized> ToBytes for Box<T> { fn w(&self, o: &mut W) { (**self).w(o) } }
impl<T: ToBytes + ?Sized> ToBytes for alloc::sync::Arc<T> { fn w(&self, o: &mut W) { (**self).w(o) } }
impl<T: ToBytes> ToBytes for Option<T> { fn w(&self, o: &mut W) { match self { None => o.b.push(0), Some(x) => { o.b.push(1); x.w(o) } } } }
impl<T: ToBytes> ToBytes for Vec<T> { fn w(&self, o: &mut W) { self.len().w(o); for x in self { x.w(o) } } }
impl<T: ToBytes> ToBytes for VecDeque<T> { fn w(&self, o: &mut W) { self.len().w(o); for x in self { x.w(o) } } }
impl<T: ToBytes, const N: usize> ToBytes for [T; N] { fn w(&self, o: &mut W) { for x in self { x.w(o) } } }
impl<A: ToBytes, B: ToBytes> ToBytes for (A, B) { fn w(&self, o: &mut W) { self.0.w(o); self.1.w(o) } }
impl<A: ToBytes, B: ToBytes, C: ToBytes> ToBytes for (A, B, C) { fn w(&self, o: &mut W) { self.0.w(o); self.1.w(o); self.2.w(o) } }
impl<A: ToBytes, B: ToBytes, C: ToBytes, D: ToBytes> ToBytes for (A, B, C, D) { fn w(&self, o: &mut W) { self.0.w(o); self.1.w(o); self.2.w(o); self.3.w(o) } }
impl<K: ToBytes + Eq + Clone + Hash, V: ToBytes> ToBytes for JsMap<K, V> { fn w(&self, o: &mut W) { self.len().w(o); for (k, v) in self.iter() { k.w(o); v.w(o) } } }
impl<K: ToBytes + Eq + Clone + Hash> ToBytes for JsSet<K> { fn w(&self, o: &mut W) { self.len().w(o); for k in self.iter() { k.w(o) } } }
impl<I: Id, T: ToBytes> ToBytes for Tab<I, T> { fn w(&self, o: &mut W) { self.len().w(o); for (_, x) in self.iter() { x.w(o) } } }
/// a box of either kind is one number: a world box's row twice over, a frame box's row twice over and one more
impl ToBytes for AnyBoxId { fn w(&self, o: &mut W) { (match *self { AnyBoxId::World(b) => 2 * b.ix() as u32, AnyBoxId::Frame(b) => 2 * b.ix() as u32 + 1 }).w(o) } }

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    #[derive(ToBytes)]
    #[bytes(parts)]
    struct Man { name: String, /// (nobody's) #[bytes(skip)] is an attribute, and this is a doc
        at: Option<(u8, u8)>, #[bytes(skip)] f: fn(u8) -> u8, pub(crate) r#in: Vec<bool> }
    #[derive(ToBytes)]
    enum Word { A, B(u8, #[bytes(skip)] u32), C { x: u16, #[bytes(skip)] y: u32 } }
    #[derive(ToBytes)]
    struct Row(pub u8, #[bytes(skip)] u32, u8);
    fn of(x: &dyn ToBytes) -> Vec<u8> { let mut o = W::new(); x.w(&mut o); o.b }
    #[test]
    fn a_type_writes_what_it_declares() {
        let m = Man { name: String::from("Al"), at: Some((1, 2)), f: |x| x, r#in: vec![true, false] };
        assert_eq!((m.f)(1), 1);
        assert_eq!(of(&m), [2, 0, 0, 0, b'A', b'l', 1, 1, 2, 2, 0, 0, 0, 1, 0]);
        assert_eq!(Man::NAMES, ["name", "at", "in"]);
        let mut each = Vec::new();
        m.parts(&mut |n, x| each.push((n, of(x).len())));
        assert_eq!(each, [("name", 6), ("at", 3), ("in", 6)]);
        assert_eq!(of(&Word::A), [0, 0]);
        assert_eq!(of(&Word::B(7, 9)), [1, 0, 7]);
        assert_eq!(of(&Word::C { x: 5, y: 9 }), [2, 0, 5, 0]);
        assert_eq!(of(&Row(1, 2, 3)), [1, 3]);
    }
}
