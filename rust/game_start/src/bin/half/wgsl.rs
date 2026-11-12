//! The shaders' half of the rows they read.
use crate::typescript::{camel, said, WRITTEN};
use game_draw::gpu::{shaded, As, Shaded, BOX_MAT, PACKED};

pub fn files() -> Vec<(&'static str, String)> { vec![("src/render/shaders/rows.wgsl", structs()), ("src/render/shaders/boxmat.wgsl", tables())] }

/// a field as a shader has it: its type, how many words, where it may begin, and what reads it off the word it lies as (a packed one)
struct Field { ty: String, words: usize, align: usize, packed: Option<(&'static str, &'static str)> }
fn field(ty: &str) -> Field {
    let t: String = ty.chars().filter(|c| !c.is_whitespace()).collect();
    if let Some((inner, n)) = t.strip_prefix('[').and_then(|t| t.strip_suffix(']')).and_then(|t| t.rsplit_once(';')) {
        let n: usize = n.parse().unwrap_or_else(|_| panic!("a shader's field of {ty}"));
        if inner.starts_with('[') {
            let f = field(inner);
            assert!(f.words == 4, "a shader's list is of rows of four, not {inner}");
            return Field { ty: format!("array<{}, {n}>", f.ty), words: 4 * n, align: 4, packed: None };
        }
        assert!((2..=4).contains(&n), "a shader's numbers come in twos, threes and fours, not {ty}");
        return Field { ty: format!("vec{n}{}", &inner[..1]), words: n, align: if n == 2 { 2 } else { 4 }, packed: None };
    }
    match t.as_str() {
        "f32" | "u32" | "i32" => Field { ty: t, words: 1, align: 1, packed: None },
        "Half2" => Field { ty: "u32".into(), words: 1, align: 1, packed: Some(("vec2f", "unpack2x16float")) },
        _ => match PACKED.iter().find(|p| p.name == t) { Some(p) => Field { ty: if p.float { "f32" } else { "u32" }.into(), words: 1, align: 1, packed: None }, None => panic!("a shader has no {ty}") },
    }
}
fn note(doc: &[&str]) -> String { if doc.is_empty() { String::new() } else { format!("   // {}", said(doc)) } }
fn up(s: &str) -> String { s[..1].to_uppercase() + &s[1..] }

/// what reads a packed word's parts
fn parts() -> Vec<String> {
    let mut l = Vec::new();
    for p in PACKED {
        l.extend([String::new(), format!("// {}", said(p.doc))]);
        if p.float {
            let under = 1u32 << p.parts[0].1;
            l.push(format!("fn {}{}(w: f32) -> f32 {{ return w - {under}.0 * floor(w * {:?}); }}", small(p.name), up(p.share), 1.0 / under as f64));
        }
        let (ty, whole) = if p.float { ("f32", "u32(w)") } else { ("u32", "w") };
        for (n, at, bits) in p.parts { l.push(format!("fn {}{}(w: {ty}) -> u32 {{ return ({whole} >> {at}u) & {}u; }}", small(p.name), up(&camel(n)), (1u64 << bits) - 1)); }
    }
    l
}
fn structs() -> String {
    let mut l = vec![format!("{WRITTEN} from rust/game_draw/src/gpu.rs: THE ROWS A SHADER READS THAT RUST LAYS OUT. To change one, change it there.")];
    for Shaded { row, lies } in shaded() {
        let As::Struct(name) = lies else { continue };
        l.extend([String::new(), format!("// {}", said(row.doc)), format!("struct {name} {{")]);
        let (mut at, mut readers) = (0, Vec::new());
        for ((f, ty), doc) in row.fields.iter().zip(row.docs) {
            let w = field(ty);
            assert!(at % w.align == 0, "{name}.{f}: {ty} cannot begin at word {at} in a shader (it begins at a multiple of {})", w.align);
            at += w.words;
            l.push(format!("  {}: {},{}", camel(f), w.ty, note(doc)));
            if let Some((out, by)) = w.packed { readers.push(format!("fn {}{}(r: {name}) -> {out} {{ return {by}(r.{}); }}", small(name), up(&camel(f)), camel(f))); }
        }
        assert!(at % 4 == 0, "{name}: {at} words, and a shader's row is so many fours");
        l.push("};".into());
        l.extend(readers);
    }
    l.extend(parts());
    l.join("\n") + "\n"
}
fn small(s: &str) -> String { s[..1].to_lowercase() + &s[1..] }

fn tables() -> String {
    let mut l = vec![format!("{WRITTEN} from rust/game_draw/src/gpu.rs: THE BOXES' MATERIAL BUFFER (`boxMat`, rows of four floats), where each of its tables begins, and a reader a field of those"),
        "// Rust lays out. To change one, change it there.".into()];
    let mut at = 0;
    for (name, rows) in BOX_MAT { l.push(format!("const {}_BASE: u32 = {at}u;", name.to_uppercase())); at += rows; }
    for Shaded { row, lies } in shaded() {
        let (region, reader, record) = match lies { As::Table { region, reader } => (region, reader, false), As::Record { region, reader } => (region, reader, true), As::Struct(_) => continue };
        let of: usize = row.fields.iter().map(|(_, ty)| field(ty).words).sum();
        let (base, rows) = (format!("{}_BASE", region.to_uppercase()), of / 4);
        assert!(of % 4 == 0, "{}: {of} words, and a table's row is so many fours", row.name);
        l.extend(if record { [String::new(), format!("// {}: {rows} rows from `row` of the table", said(row.doc)), format!("const {}_HEAD: u32 = {rows}u;", region.to_uppercase())] }
            else { [String::new(), format!("// {}: {rows} rows a box", said(row.doc)), format!("const {}_STRIDE: u32 = {rows}u;", region.to_uppercase())] });
        let mut at = 0;
        for ((f, ty), doc) in row.fields.iter().zip(row.docs) {
            let w = field(ty);
            let (fn_, here) = (format!("{reader}{}", up(&camel(f))), if record { format!("{base} + row + {}u", at / 4) } else { format!("{base} + i * {rows}u + {}u", at / 4) });
            let list = w.ty.starts_with("array<");
            let which = if record { "row" } else { "i" };
            let (args, out, read) = if list { (format!("{which}: u32, k: u32"), "vec4f".to_string(), format!("boxMat[{here} + k]")) }
                else { (format!("{which}: u32"), w.ty.replace("u32", "f32"), format!("boxMat[{here}]{}", if w.words == 4 { String::new() } else { format!(".{}", &"xyzw"[at % 4..at % 4 + w.words]) })) };
            assert!(list || (at % 4) + w.words <= 4, "{}.{f} lies over two rows", row.name);
            assert!(w.packed.is_none() && !w.ty.contains("u32"), "{}.{f}: a table of floats holds floats", row.name);
            if list { l.push(format!("const {}_{}S: u32 = {}u;", reader.to_uppercase(), f.to_uppercase(), w.words / 4)); }
            l.push(format!("fn {fn_}({args}) -> {out} {{ return {read}; }}{}", note(doc)));
            at += w.words;
        }
    }
    l.join("\n") + "\n"
}
