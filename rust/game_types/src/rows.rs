//! ROWS THAT CROSS AS THEY LIE: a run of words in the module's memory that a page reads where it stands, or hands to the GPU with no look at it. A row is declared once
//! (`row!`): its fields by name, in their order, and what is said of each. The page's offsets, and the struct a shader reads it by, are written from that declaration
//! (`game_start/src/bin/half`).
//!
//! - A field is a word or a run of words: `f32`, `u32`, `i32`, two halves in a word (`Half2`), or an array of one. So a row has no padding and lies as it is declared.
//! - A word may be packed (`packed!`): some whole numbers side by side in it, each by its name.
//! - A row a host hands over is all doubles (`f64`, `handed`), and its offsets are in doubles.
//! - A row a shader reads keeps the shader's own rule: three or four numbers begin at a multiple of four words, two at a multiple of two.

/// a type that is nothing but words
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

/// a packed word as a table: its name, whether it is a float (its parts are then of its whole number, and what is under the first of them is a share of its own), and each part's
/// name, where it begins and how many bits it has
pub struct PackedTable { pub name: &'static str, pub doc: &'static [&'static str], pub float: bool, pub share: &'static str, pub parts: &'static [(&'static str, u32, u32)] }
/// A WORD OF PARTS, declared: what puts them in, and the table of itself. `f32 over share`: a float, whose whole number above the first part's place holds them, and
/// which is `share` (a number to that place) below it
#[macro_export]
macro_rules! packed {
    ($(#[doc = $d:expr])* pub struct $name:ident: u32 { $($part:ident: $at:literal, $bits:literal;)* }) => {
        $(#[doc = $d])* #[repr(transparent)] #[derive(Clone, Copy, Debug, PartialEq)] pub struct $name(pub u32);
        impl $crate::rows::Plain for $name { const ZERO: Self = $name(0); }
        impl $name {
            pub const TABLE: $crate::rows::PackedTable = $crate::rows::PackedTable { name: stringify!($name), doc: &[$($d),*], float: false, share: "", parts: &[$((stringify!($part), $at, $bits)),*] };
            #[allow(clippy::too_many_arguments)]
            pub fn of($($part: u32),*) -> $name { $name(0 $(| ($part & ((1u64 << $bits) - 1) as u32) << $at)*) }
            $(pub fn $part(self) -> u32 { self.0 >> $at & ((1u64 << $bits) - 1) as u32 })*
        }
    };
    ($(#[doc = $d:expr])* pub struct $name:ident: f32 over $share:ident { $($part:ident: $at:literal, $bits:literal;)* }) => {
        $(#[doc = $d])* #[repr(transparent)] #[derive(Clone, Copy, Debug, PartialEq)] pub struct $name(pub f32);
        impl $crate::rows::Plain for $name { const ZERO: Self = $name(0.0); }
        impl $name {
            pub const TABLE: $crate::rows::PackedTable = $crate::rows::PackedTable { name: stringify!($name), doc: &[$($d),*], float: true, share: stringify!($share), parts: &[$((stringify!($part), $at, $bits)),*] };
            #[allow(clippy::too_many_arguments)]
            pub fn of($share: f64, $($part: u32),*) -> $name { $name(($share + (0 $(| ($part & ((1u64 << $bits) - 1) as u32) << $at)*) as f64) as f32) }
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

#[cfg(test)]
mod tests {
    packed! {
        /// whole numbers side by side
        pub struct Word: u32 {
            low: 0, 8;
            mid: 8, 5;
            top: 24, 8;
        }
    }
    packed! {
        /// a share, and whole numbers over it
        pub struct Share: f32 over rough {
            metal: 1, 1;
            stuff: 3, 5;
            own: 9, 6;
        }
    }

    /// A FAILURE MEANS: a part of a packed word is read as another's, or one too large runs into its neighbour.
    #[test]
    fn a_part_of_a_word_is_read_back_and_no_other_s() {
        let w = Word::of(200, 17, 255);
        assert_eq!((w.low(), w.mid(), w.top()), (200, 17, 255));
        let over = Word::of(256 + 3, 32 + 1, 0);
        assert_eq!((over.low(), over.mid(), over.top()), (3, 1, 0));
    }
    /// The share is read as a shader reads it (its place's remainder), under the largest whole number there can be, to a 500th.
    #[test]
    fn a_share_stands_under_every_whole_number() {
        for k in 0..=500 {
            let (rough, w) = (k as f64 / 500.0, Share::of(k as f64 / 500.0, 1, 31, 44).0);
            let read = w - 2.0 * crate::m::floor(w as f64 * 0.5) as f32;
            assert!((read as f64 - rough).abs() < 0.001, "{rough} is read as {read}");
            let whole = w as u32;
            assert_eq!((whole >> 1 & 1, whole >> 3 & 31, whole >> 9 & 63), (1, 31, 44), "under {rough}");
        }
    }
}
