//! WHAT A SHADER READS OF ROWS THAT ARE WRITTEN HERE: which rows, by what name, and how they lie. A shader's struct or its readers, the bases of the tables that share a
//! buffer and the page's offsets are written from this (`game_start/src/bin/half`); no shader lays one out.
use crate::{boxes, men};
use alloc::string::String;
use game::rows::{table, LightRow, PackedTable, RowTable};

/// how a shader has a row
pub enum As {
    /// a struct of this name
    Struct(&'static str),
    /// rows of four floats in the boxes' material buffer, in this region of it: a reader a field, its name behind this one
    Table { region: &'static str, reader: &'static str },
    /// the same of a record that begins at a row of the region and goes on as long as it is
    Record { region: &'static str, reader: &'static str },
}
pub struct Shaded { pub row: RowTable, pub lies: As }

game::packed! {
    /// whose a proxy is, its colour, and the radius at its other end in millimetres (0: the first's again)
    pub struct CapsuleWord: u32 {
        owner: 0, 8;
        blue: 8, 5;
        green: 13, 6;
        red: 19, 5;
        rb: 24, 8;
    }
}
section! {
    /// one of a man's proxies, as a shader traces it: a round cone
    pub struct Capsule {
        pub a: [f32; 3],
        pub r: f32,
        pub b: [f32; 3],
        pub misc: CapsuleWord,
    }
}

/// the words that hold parts
pub const PACKED: [PackedTable; 3] = [boxes::BoxWord::TABLE, boxes::Finish::TABLE, CapsuleWord::TABLE];

pub fn shaded() -> [Shaded; 9] {
    use As::{Record, Struct, Table};
    [
        Shaded { row: table::<boxes::Geo>(), lies: Struct("BoxGeo") },
        Shaded { row: table::<boxes::Info>(), lies: Struct("SceneInfo") },
        Shaded { row: table::<Capsule>(), lies: Struct("Capsule") },
        Shaded { row: table::<LightRow>(), lies: Struct("Light") },
        Shaded { row: table::<men::Inst>(), lies: Struct("CharInst") },
        Shaded { row: table::<boxes::Mat>(), lies: Table { region: "mat", reader: "box" } },
        Shaded { row: table::<boxes::Cut>(), lies: Table { region: "cut", reader: "cut" } },
        Shaded { row: table::<boxes::glass::Pane>(), lies: Table { region: "pane", reader: "pane" } },
        Shaded { row: table::<boxes::marks::MarkHead>(), lies: Record { region: "mark", reader: "mark" } },
    ]
}

/// the rows a shader reads and the tables of the boxes' buffer, as text: what the page's half is held to as the module is loaded
pub fn text() -> String {
    use core::fmt::Write;
    let mut s = String::new();
    for Shaded { row, .. } in shaded() {
        s.push_str(row.name);
        for (f, t) in row.fields { let _ = write!(s, " {f}:{}", t.replace(' ', "")); }
        s.push('\n');
    }
    for p in PACKED {
        s.push_str(p.name);
        for (n, at, bits) in p.parts { let _ = write!(s, " {n}:{at}+{bits}"); }
        s.push('\n');
    }
    for (n, rows) in BOX_MAT { let _ = write!(s, "{n} {rows}\n"); }
    s
}

/// cells of the air over the plan, half a metre to a side, four to a row
const AIR: usize = ::sim::air::N;
/// rows of four floats a row of these takes
const fn fours<T>() -> usize { core::mem::size_of::<T>() / 16 }
/// THE BOXES' MATERIAL BUFFER: what shares it, in rows of four floats, one after another. The cut planes are last: a page that cuts nothing has none
pub const BOX_MAT: [(&str, usize); 5] =
    [("mat", boxes::MAX * fours::<boxes::Mat>()), ("mark", boxes::marks::TABLE), ("pane", boxes::grid::MAX_PANES * fours::<boxes::glass::Pane>()), ("air", AIR / 4), ("cut", boxes::MAX * fours::<boxes::Cut>())];
