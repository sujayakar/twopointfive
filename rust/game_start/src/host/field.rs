//! WHAT IS HANDED THE GAME, READ: the fields of a command, an ask or a call in their order, little end first. A yes or no is a byte; a text, a run of bytes and a list are how many (u32) and then each; what may be
//! none is a byte and, where it is there, itself; a row (`game::handed`) is as it lies.
use alloc::vec::Vec;

/// bytes being read
pub struct Bytes<'a> { b: &'a [u8], at: usize }
impl<'a> Bytes<'a> {
    pub fn of(b: &'a [u8]) -> Bytes<'a> { Bytes { b, at: 0 } }
    pub fn done(&self) -> bool { self.at >= self.b.len() }
    fn take(&mut self, n: usize) -> &'a [u8] { let r = &self.b[self.at..self.at + n]; self.at += n; r }
}
pub trait Field<'a>: Sized { fn read(b: &mut Bytes<'a>) -> Self; }
macro_rules! numbers { ($($t:ident $n:literal)*) => { $(impl<'a> Field<'a> for $t { #[inline] fn read(b: &mut Bytes<'a>) -> $t { $t::from_le_bytes(b.take($n).try_into().unwrap()) } })* } }
numbers!(u32 4 i32 4 f64 8);
impl<'a> Field<'a> for usize { fn read(b: &mut Bytes<'a>) -> usize { u32::read(b) as usize } }
impl<'a> Field<'a> for bool { fn read(b: &mut Bytes<'a>) -> bool { b.take(1)[0] != 0 } }
impl<'a> Field<'a> for [f64; 3] { fn read(b: &mut Bytes<'a>) -> [f64; 3] { [f64::read(b), f64::read(b), f64::read(b)] } }
impl<'a> Field<'a> for &'a [u8] { fn read(b: &mut Bytes<'a>) -> &'a [u8] { let n = usize::read(b); b.take(n) } }
impl<'a> Field<'a> for &'a str { fn read(b: &mut Bytes<'a>) -> &'a str { core::str::from_utf8(<&[u8]>::read(b)).unwrap_or("") } }
impl<'a, T: Field<'a>> Field<'a> for Option<T> { fn read(b: &mut Bytes<'a>) -> Option<T> { bool::read(b).then(|| T::read(b)) } }
impl<'a, T: Field<'a>> Field<'a> for Vec<T> { fn read(b: &mut Bytes<'a>) -> Vec<T> { let n = usize::read(b); (0..n).map(|_| T::read(b)).collect() } }
/// a row, as it lies
pub struct Laid<T>(pub T);
impl<'a, T: game::rows::Row> Field<'a> for Laid<T> {
    #[inline]
    fn read(b: &mut Bytes<'a>) -> Laid<T> { let at = b.take(core::mem::size_of::<T>()); Laid(unsafe { core::ptr::read_unaligned(at.as_ptr() as *const T) }) }
}
