//! WHAT A TYPE IS, SAID BY ITSELF: the shape of what crosses to a page, for the program that writes the page's half (`game_start/src/bin/half`) and for the stamp both
//! halves are held to. A type that crosses as its bytes (`ToBytes`) derives `Crosses`, and nothing else lists its fields. What does not derive it cannot cross: a row's id does
//! not, for one, so a man is told by his character's number and a door by its name.
use alloc::{boxed::Box, string::String, vec::Vec};
pub use game_derive::Crosses;

/// a field's type, as far as the other side can read it
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    /// a number, by Rust's name of it
    Number(&'static str),
    YesNo,
    Text,
    /// a place or a way: three doubles
    V3,
    Option(Box<Ty>),
    List(Box<Ty>),
    /// so many, one after another
    Row(Vec<Ty>),
    /// another of the types told, by its name
    Named(&'static str),
}
/// what stands under a name
#[derive(Clone, Debug)]
pub enum Shape {
    Record(Vec<(&'static str, Ty)>),
    /// an enum of plain words, by its arm's place
    Words(Vec<&'static str>),
}
/// the types told, each after those it names
#[derive(Default)]
pub struct Types {
    pub list: Vec<(&'static str, Shape)>,
    begun: Vec<(&'static str, &'static str)>,
}
impl Types {
    /// a type by its name and the path it is declared at, said once
    pub fn tell(&mut self, name: &'static str, path: &'static str, shape: impl FnOnce(&mut Types) -> Shape) -> Ty {
        match self.begun.iter().find(|b| b.0 == name) {
            Some(b) => assert!(b.1 == path, "{} and {} would be told by one name", b.1, path),
            None => {
                self.begun.push((name, path));
                let s = shape(self);
                self.list.push((name, s));
            }
        }
        Ty::Named(name)
    }
}
pub trait Crosses { fn ty(t: &mut Types) -> Ty; }

macro_rules! numbers { ($($t:ident)*) => { $(impl Crosses for $t { fn ty(_: &mut Types) -> Ty { Ty::Number(stringify!($t)) } })* } }
numbers!(f64 f32 u32 usize i32 u16 i16 u8 i8);
impl Crosses for bool { fn ty(_: &mut Types) -> Ty { Ty::YesNo } }
impl Crosses for String { fn ty(_: &mut Types) -> Ty { Ty::Text } }
impl Crosses for str { fn ty(_: &mut Types) -> Ty { Ty::Text } }
impl<T: Crosses + ?Sized> Crosses for &T { fn ty(t: &mut Types) -> Ty { T::ty(t) } }
impl<T: Crosses> Crosses for Option<T> { fn ty(t: &mut Types) -> Ty { Ty::Option(Box::new(T::ty(t))) } }
impl<T: Crosses> Crosses for Vec<T> { fn ty(t: &mut Types) -> Ty { Ty::List(Box::new(T::ty(t))) } }
impl<T: Crosses, const N: usize> Crosses for [T; N] {
    fn ty(t: &mut Types) -> Ty { match T::ty(t) { Ty::Number("f64") if N == 3 => Ty::V3, x => Ty::Row(alloc::vec![x; N]) } }
}
impl<A: Crosses, B: Crosses> Crosses for (A, B) { fn ty(t: &mut Types) -> Ty { Ty::Row(alloc::vec![A::ty(t), B::ty(t)]) } }
impl<A: Crosses, B: Crosses, C: Crosses> Crosses for (A, B, C) { fn ty(t: &mut Types) -> Ty { Ty::Row(alloc::vec![A::ty(t), B::ty(t), C::ty(t)]) } }

/// something asked for by its number, with fields: a command, an ask
pub struct Asked {
    pub n: u32,
    pub name: &'static str,
    /// each by its name and Rust's name of its type
    pub fields: &'static [(&'static str, &'static str)],
}

impl core::fmt::Display for Ty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Ty::Number(n) | Ty::Named(n) => f.write_str(n),
            Ty::YesNo => f.write_str("bool"),
            Ty::Text => f.write_str("text"),
            Ty::V3 => f.write_str("v3"),
            Ty::Option(t) => write!(f, "{t}?"),
            Ty::List(t) => write!(f, "{t}*"),
            Ty::Row(r) => { f.write_str("(")?; for t in r { write!(f, "{t} ")?; } f.write_str(")") }
        }
    }
}
impl Types {
    /// the types as text, a line each: what their readers are held to
    pub fn text(&self) -> String {
        use core::fmt::Write;
        let mut s = String::new();
        for (name, shape) in &self.list {
            s.push_str(name);
            match shape {
                Shape::Record(r) => for (f, t) in r { let _ = write!(s, " {f}:{t}"); },
                Shape::Words(w) => for x in w { let _ = write!(s, " '{x}"); },
            }
            s.push('\n');
        }
        s
    }
}
/// those asked for by number as text, a line each
pub fn asked_text(list: &[Asked]) -> String {
    use core::fmt::Write;
    let mut s = String::new();
    for a in list {
        let _ = write!(s, "{} {}", a.n, a.name);
        for (f, t) in a.fields { let _ = write!(s, " {f}:{t}"); }
        s.push('\n');
    }
    s
}
/// the stamp of everything that crosses: the module answers it, the page's half was written with it, and a module of another is refused as it loads
pub fn stamp(text: &str) -> u32 { text.bytes().fold(0x811c_9dc5, |s, b| (s ^ b as u32).wrapping_mul(0x0100_0193)) }
