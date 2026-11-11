//! ROWS THAT CROSS AS THEY LIE: a run of words in the module's memory that a page reads where it stands, or hands to the GPU with no look at it. A row is declared once
//! (`row!`): its fields by name, in their order, and what is said of each. The page's offsets, and the struct a shader reads it by, are written from that declaration
//! (`game_start/src/bin/half`).
//!
//! - A field is a word or a run of words: `f32`, `u32`, `i32`, two halves in a word (`Half2`), or an array of one. So a row has no padding and lies as it is declared.
//! - A row a host hands over is all doubles (`f64`, `handed`), and its offsets are in doubles.
//! - A row a shader reads keeps the shader's own rule: three or four numbers begin at a multiple of four words, two at a multiple of two.

/// a type that is nothing but words, and that every bit pattern is one of: a row handed over is read by its bits (`Laid`)
pub trait Plain: Copy { const ZERO: Self; }
impl Plain for f32 { const ZERO: f32 = 0.0; }
impl Plain for u32 { const ZERO: u32 = 0; }
impl Plain for i32 { const ZERO: i32 = 0; }
impl Plain for f64 { const ZERO: f64 = 0.0; }
impl<T: Plain, const N: usize> Plain for [T; N] { const ZERO: [T; N] = [T::ZERO; N]; }

/// two numbers in a word, each as a half, the first in the low one (a shader's unpack2x16float)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Half2(pub u32);
impl Plain for Half2 { const ZERO: Half2 = Half2(0); }
impl Half2 { pub fn of(a: f64, b: f64) -> Half2 { Half2(half(a) | half(b) << 16) } }

/// half a float's bits, rounded to the nearest
pub fn half(v: f64) -> u32 {
    let x = (v as f32).to_bits();
    let sign = (x >> 16) & 0x8000;
    let exp = ((x >> 23) & 0xff) as i32 - 127 + 15;
    let mut mant = x & 0x7f_ffff;
    if exp <= 0 {
        if exp < -10 { return sign; }
        mant |= 0x80_0000;
        let shift = (14 - exp) as u32;
        let mut r = mant >> shift;
        if (mant >> (shift - 1)) & 1 != 0 { r += 1; }
        return sign | r;
    }
    if exp >= 31 { return sign | 0x7c00; }
    let mut r = sign | ((exp as u32) << 10) | (mant >> 13);
    if mant & 0x1000 != 0 { r += 1; }
    r
}
/// what a half's bits stand for
pub fn half_value(bits: u32) -> f64 {
    let (s, e, mant) = (if bits & 0x8000 != 0 { -1.0 } else { 1.0 }, ((bits >> 10) & 31) as i32, (bits & 1023) as f64);
    if e == 0 { s * mant * 5.9604644775390625e-8 } else { s * (1.0 + mant / 1024.0) * crate::m::pow(2.0, (e - 15) as f64) }
}

/// a row: its fields in their order, and what is said of each (which no module carries: only the program that writes the page's half reads it)
pub trait Row: Copy {
    const NAME: &'static str;
    /// each by its name and Rust's name of its type
    const FIELDS: &'static [(&'static str, &'static str)];
    const DOC: &'static [&'static str];
    const DOCS: &'static [&'static [&'static str]];
    const ZERO: Self;
}
/// a row as a table
pub struct RowTable { pub name: &'static str, pub doc: &'static [&'static str], pub fields: &'static [(&'static str, &'static str)], pub docs: &'static [&'static [&'static str]] }
pub const fn table<T: Row>() -> RowTable { RowTable { name: T::NAME, doc: T::DOC, fields: T::FIELDS, docs: T::DOCS } }
/// a row's words
pub fn words<T: Row>(r: &T) -> &[u32] { unsafe { core::slice::from_raw_parts(r as *const T as *const u32, core::mem::size_of::<T>() / 4) } }

/// A ROW, DECLARED: the struct as it lies in memory, and the table of itself
#[macro_export]
macro_rules! row {
    ($(#[doc = $d:expr])* pub struct $name:ident { $($(#[doc = $fd:expr])* pub $f:ident: $t:ty,)* }) => {
        $(#[doc = $d])* #[repr(C)] #[derive(Clone, Copy, Debug, PartialEq)] pub struct $name { $($(#[doc = $fd])* pub $f: $t,)* }
        impl $crate::rows::Row for $name {
            const NAME: &'static str = stringify!($name);
            const FIELDS: &'static [(&'static str, &'static str)] = &[$((stringify!($f), stringify!($t)),)*];
            const DOC: &'static [&'static str] = &[$($d),*];
            const DOCS: &'static [&'static [&'static str]] = &[$(&[$($fd),*],)*];
            const ZERO: Self = $name { $($f: <$t as $crate::rows::Plain>::ZERO,)* };
        }
    };
}

row! {
    /// a light, as a shader reads it
    pub struct LightRow {
        pub pos: [f32; 3],
        /// 0 a point, 1 a spot, 2 a directional one, 3 an area (a one-sided disk: its normal `dir`, its radius `radius`, its colour radiance by area); flags above the low byte
        pub kind: u32,
        pub dir: [f32; 3],
        pub range: f32,
        pub color: [f32; 3],
        pub cos_outer: f32,
        pub cos_inner: f32,
        pub volumetric: f32,
        /// the low byte a man's, the panes it shines through above it
        pub owner: u32,
        pub radius: f32,
    }
}
