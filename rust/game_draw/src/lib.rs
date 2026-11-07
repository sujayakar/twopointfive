//! WHAT THE PICTURE IS TOLD, AND NOTHING ELSE OF THE GAME. A page that draws this game holds no game: after a tick it is told what to draw as a list of
//! SECTIONS, each a run of flat rows in this module's memory, read where they lie. A section is declared once (`section!`) and filled by a function that
//! takes the game shared, so drawing cannot change it; this crate is above the rules and no rule can name it. The page's offsets into a row are made from
//! the same declaration by `bun run tools/draw/gen.ts`.
//!
//! To add a section: a file beside this one with its `section!` and its `fill`, a line in `sections!` below, then `tools/draw/gen.ts`.
//!
//! - A field is a word or a run of words: `f32`, `u32`, `i32`, or an array of one. So a row has no padding and lies as it is declared.
//! - A row whose only reader is the GPU is written here as the GPU reads it: the page hands the rows over and packs nothing.
//! - A row holds values, never a reference into the game: what it says of another row is that row's place in its own section, or a key that means one
//!   thing for as long as the thing stands.
#![no_std]
extern crate alloc;
use alloc::{string::String, vec::Vec};
use game::game::Game;

/// a type that is nothing but words
pub trait Words: Copy { const ZERO: Self; }
impl Words for f32 { const ZERO: f32 = 0.0; }
impl Words for u32 { const ZERO: u32 = 0; }
impl Words for i32 { const ZERO: i32 = 0; }
impl<T: Words, const N: usize> Words for [T; N] { const ZERO: [T; N] = [T::ZERO; N]; }

/// a row of a section: its fields in their order, as the page's offsets are made from them
pub trait Row: Copy { const FIELDS: &'static [(&'static str, &'static str)]; const ZERO: Self; }

/// A SECTION'S ROW, declared: the struct as it lies in memory and the names its offsets are made from
macro_rules! section {
    ($(#[$m:meta])* pub struct $name:ident { $($(#[$fm:meta])* pub $f:ident: $t:ty,)* }) => {
        $(#[$m])* #[repr(C)] #[derive(Clone, Copy, Debug, PartialEq)] pub struct $name { $($(#[$fm])* pub $f: $t,)* }
        impl crate::Row for $name {
            const FIELDS: &'static [(&'static str, &'static str)] = &[$((stringify!($f), stringify!($t)),)*];
            const ZERO: Self = $name { $($f: <$t as crate::Words>::ZERO,)* };
        }
    };
}

/// THE SECTIONS, declared: each with its place (its bit in a host's `want`), its row and where its rows are kept
macro_rules! sections {
    ($($i:literal $name:ident: $t:ty = $($at:ident).+;)*) => {
        /// where each section lies, in their order: its address and how many rows
        fn table(d: &Drawn, out: &mut Vec<u32>) { out.clear(); $({ let v: &Vec<$t> = &d.$($at).+; out.push(v.as_ptr() as usize as u32); out.push(v.len() as u32); })* }
        /// the declarations as text, a line a section: what the page's offsets were made from is held to it as the module is loaded
        pub fn layout() -> String {
            let mut s = String::new();
            $(s.push_str(stringify!($name)); for (f, t) in <$t as Row>::FIELDS { s.push(' '); s.push_str(f); s.push(':'); for c in t.chars() { if !c.is_whitespace() { s.push(c); } } } s.push('\n');)*
            s
        }
        /// each section's bit
        #[allow(non_upper_case_globals)]
        pub mod want { $(pub const $name: u32 = 1 << $i;)* }
    };
}

pub mod fog;

sections! {
    0 fog: fog::Fog = fog;
}

/// what one host of one game draws it from
#[derive(Default)]
pub struct Drawn { pub fog: Vec<fog::Fog>, table: Vec<u32> }
impl Drawn {
    /// A TICK HAS RUN: the sections of `want`, made again from the game as it stands
    pub fn take(&mut self, g: &Game, want: u32) {
        if want & want::fog != 0 { self.fog.clear(); fog::fill(g, &mut self.fog); }
    }
    pub fn table(&mut self) -> &[u32] {
        let mut t = core::mem::take(&mut self.table);
        table(self, &mut t);
        self.table = t;
        &self.table
    }
}
