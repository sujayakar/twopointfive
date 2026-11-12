//! A table that Rust has, as the level tools' own source: its types, and its rows written out.
use crate::typescript::{camel, read, WRITTEN};
use game::bytes::{ToBytes, W};
use game::crosses::{Crosses, Shape, Ty, Types};
use game::scene_materials::{Does, DoesWord};

pub fn files() -> Vec<(&'static str, String)> { vec![("tools/levels/lib/materials.ts", materials())] }

fn materials() -> String {
    let mut types = Types::default();
    let row = Does::ty(&mut types);
    let mut l = vec![format!("{WRITTEN} from rust/game_types/src/scene_materials/types.rs: THE TABLE OF MATERIALS, what a thing does, a row a word. To change a row, change it there."),
        "import type { DoesWord } from '../../../src/draw/names';".into(), String::new()];
    l.extend(types.list.iter().map(|(n, shape)| declared(n, shape)));
    l.extend([String::new(), "export const MATERIALS: Readonly<Record<DoesWord, Readonly<Does>>> = {".into()]);
    l.extend(DoesWord::ALL.iter().map(|w| format!("  '{}': {},", w.as_str(), literal(w.row(), &row, &types))));
    l.extend(["};".into(), String::new()]);
    l.join("\n")
}

/// a type, declared
fn declared(name: &str, shape: &Shape) -> String {
    match shape {
        Shape::Words(w) => format!("export type {name} = {};", w.iter().map(|x| format!("'{x}'")).collect::<Vec<_>>().join(" | ")),
        Shape::Record(r) => format!("export interface {name} {{ {} }}", r.iter().map(|(f, t)| format!("{}: {}", camel(f), read(t).0)).collect::<Vec<_>>().join("; ")),
    }
}
/// a value, written out: by its own bytes and what its type says of them
fn literal<T: ToBytes>(v: &T, ty: &Ty, types: &Types) -> String {
    let mut w = W::new();
    v.w(&mut w);
    let mut b = &w.b[..];
    let s = value(&mut b, ty, types);
    assert!(b.is_empty(), "{} bytes of a value are left", b.len());
    s
}
fn value(b: &mut &[u8], ty: &Ty, types: &Types) -> String {
    fn take<const N: usize>(b: &mut &[u8]) -> [u8; N] { let (a, rest) = b.split_at(N); *b = rest; a.try_into().unwrap() }
    match ty {
        Ty::YesNo => (take::<1>(b)[0] != 0).to_string(),
        Ty::Number("f64") => f64::from_le_bytes(take(b)).to_string(),
        Ty::Number("u32") => u32::from_le_bytes(take(b)).to_string(),
        Ty::Option(t) => if take::<1>(b)[0] == 0 { "null".into() } else { value(b, t, types) },
        Ty::Named(n) => match &types.list.iter().find(|x| x.0 == *n).expect("a type told").1 {
            Shape::Words(w) => format!("'{}'", w[u16::from_le_bytes(take(b)) as usize]),
            Shape::Record(r) => format!("{{ {} }}", r.iter().map(|(f, t)| format!("{}: {}", camel(f), value(b, t, types))).collect::<Vec<_>>().join(", ")),
        },
        t => panic!("a table's {t} is not written out yet"),
    }
}
