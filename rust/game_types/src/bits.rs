//! A WORD'S BITS, BY NAME. A word whose bits cross (to a page, a shader, a level file) is declared once (`bits!`): each bit by its name, and what is said of it. What the page and
//! the shaders know them by is written from that declaration (`game_start/src/bin/half`).
//!
//! - A bit is a `u32` of the module the declaration makes: `box_flag::GLASS`. Two words that share a bit say so: `GLASS = box_flag::GLASS`.

/// a word's bits as a table (what is said of each is carried by no module: only the program that writes the other halves reads it)
pub struct BitsTable { pub name: &'static str, pub doc: &'static [&'static str], pub bits: &'static [(&'static str, u32, &'static [&'static str])] }
impl BitsTable {
    /// the bit of this name, as a level file or a page writes it
    pub fn named(&self, name: &str) -> Option<u32> { self.bits.iter().find(|b| pascal(b.0).eq(name.chars())).map(|b| b.1) }
}
/// a bit's name as a page has it: `NO_SHADOW` is `NoShadow`
pub fn pascal(name: &str) -> impl Iterator<Item = char> + '_ { name.split('_').flat_map(|w| w.chars().enumerate().map(|(i, c)| if i == 0 { c } else { c.to_ascii_lowercase() })) }

/// A WORD'S BITS, DECLARED: a module of constants, and the table of itself
#[macro_export]
macro_rules! bits {
    ($(#[doc = $d:expr])* pub mod $m:ident as $name:ident { $($(#[doc = $bd:expr])* $b:ident = $v:expr,)* }) => {
        $(#[doc = $d])* pub mod $m {
            #[allow(unused_imports)] use super::*;
            $($(#[doc = $bd])* pub const $b: u32 = $v;)*
            pub const TABLE: $crate::bits::BitsTable = $crate::bits::BitsTable { name: stringify!($name), doc: &[$($d),*], bits: &[$((stringify!($b), $b, &[$($bd),*]),)*] };
        }
    };
}
/// the declarations as text, a line a word: what the other halves were written from is held to it as the module is loaded
pub fn text(tables: &[&BitsTable]) -> alloc::string::String {
    let mut s = alloc::string::String::new();
    for t in tables {
        s.push_str(t.name);
        for (n, v, _) in t.bits { s.push_str(&alloc::format!(" {n}={v}")); }
        s.push('\n');
    }
    s
}
