//! The TypeScript.
use game::crosses::{self, Asked, Shape, Ty};
use game::rows::RowTable;
use game_start::host::{command, crosses as host, desk};

/// the stamp of everything that crosses (game_face `face_draw_layout` answers the module's)
pub fn stamp() -> u32 { crosses::stamp(&(game_draw::layout() + &host::text())) }

pub fn files(stamp: u32) -> Vec<(&'static str, String)> {
    vec![("src/draw/sections.ts", sections(stamp)), ("src/draw/gpu.ts", gpu()), ("src/draw/commands.ts", commands()), ("src/draw/asks.ts", asks()), ("src/view/read.ts", views())]
}

pub const WRITTEN: &str = "// Written by the program `half` (rust/game_start/src/bin/half)";
/// a name as TypeScript has it
pub fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut up = false;
    for c in s.trim_end_matches('_').chars() {
        if c == '_' { up = true; } else if up { out.extend(c.to_uppercase()); up = false; } else { out.push(c); }
    }
    out
}
fn small(s: &str) -> String { s[..1].to_lowercase() + &s[1..] }
/// what is said of a thing, on one line
pub fn said(doc: &[&str]) -> String { doc.iter().map(|l| l.trim()).collect::<Vec<_>>().join(" ") }

/// a row's field: how many words, and the kind of each
fn words(ty: &str) -> (usize, &str) {
    match ty.trim().strip_prefix('[').and_then(|t| t.strip_suffix(']')).and_then(|t| t.rsplit_once(';')) {
        Some((t, n)) => { let (k, kind) = words(t); (k * n.trim().parse::<usize>().unwrap_or_else(|_| panic!("a row's field of {ty}")), kind) }
        None => match ty.trim() { "Half2" => (1, "two halves"), t => (1, t) },
    }
}
/// a row's offsets, in words
fn offsets(r: &RowTable) -> Vec<String> {
    let mut l = Vec::new();
    if !r.doc.is_empty() { l.push(format!("/** {} */", said(r.doc))); }
    l.push(format!("export const enum {} {{", r.name));
    let mut at = 0;
    for ((f, ty), doc) in r.fields.iter().zip(r.docs) {
        let (n, kind) = words(ty);
        l.push(format!("  /** {kind}{}{} */", if n > 1 { format!(" × {n}") } else { String::new() }, if doc.is_empty() { String::new() } else { format!(": {}", said(doc)) }));
        l.push(format!("  {} = {at},", camel(f)));
        at += n;
    }
    l.extend(["  /** words a row */".into(), format!("  row = {at},"), "}".into()]);
    l
}
/// the rows a shader reads and the page writes, and what shares the boxes' material buffer
fn gpu() -> String {
    let told: Vec<&str> = game_draw::sections().iter().map(|s| s.1.name).collect();
    let mut l = vec![format!("{WRITTEN} from rust/game_draw/src/gpu.rs. To change a row a shader reads, change it there: the shader's own struct is written with this (src/render/shaders/rows.wgsl).")];
    for s in game_draw::gpu::shaded() { if !told.contains(&s.row.name) { l.push(String::new()); l.extend(offsets(&s.row)); } }
    let mut at = 0;
    let bases: Vec<String> = game_draw::gpu::BOX_MAT.iter().map(|(n, rows)| { let b = format!("{n} = {at}"); at += rows; b }).collect();
    l.extend([String::new(), "/** THE BOXES' MATERIAL BUFFER: where each of its tables begins, in rows of four floats, and how long it is */".into(), format!("export const enum BoxMat {{ {}, rows = {at} }}", bases.join(", ")), String::new()]);
    l.join("\n")
}
fn sections(stamp: u32) -> String {
    let (groups, list) = (game_draw::GROUPS, game_draw::sections());
    assert!(groups.len() <= 32, "a want has 32 bits, and there are more groups than that");
    let names = |v: Vec<String>| v.join(", ");
    let mut l = vec![
        format!("{WRITTEN} from rust/game_draw. To change a section, change it there."), String::new(),
        "/** the stamp of everything that crosses, as this was written: a module of another stamp is refused (drawn.ts) */".into(), format!("export const LAYOUT = {stamp};"),
        "/** what a host may ask for: what is made together is asked for together */".into(),
        format!("export const enum Want {{ {}, all = {} }}", names(groups.iter().enumerate().map(|(i, g)| format!("{} = {}", camel(g), 1u32 << i)).collect()), ((1u64 << groups.len()) - 1) as u32),
        "/** each section's place in the table the module answers with */".into(),
        format!("export const enum Section {{ {}, count = {} }}", names(list.iter().enumerate().map(|(i, s)| format!("{} = {i}", camel(s.0))).collect()), list.len()),
    ];
    let mut had = Vec::new();
    for (_, r) in &list {
        if had.contains(&r.name) { continue; }
        had.push(r.name);
        l.push(String::new());
        l.extend(offsets(r));
    }
    // (a word that crosses by its number: an enum's, in their order)
    let lists: [(&str, &[&str], &str); 5] = [
        ("STUFFS", game::scene_stuff::BoxStuff::WORDS, "what a box may say it is made of"), ("PKINDS", game::render_particles::PKind::WORDS, "what a particle is"),
        ("EMIT_SHAPES", game::render_particles::EmitShape::WORDS, "how particles are thrown"), ("SMOKE_KINDS", game::smoke_types::SmokeEmitterKind::WORDS, "what smoke comes from"),
        ("FLOORS", game::floor::FloorMat::WORDS, "what a floor is"),
    ];
    for (name, words, doc) in lists {
        l.extend([String::new(), format!("/** {doc}, by its number in Rust */"), format!("export const {name} = [{}] as const;", names(words.iter().map(|w| format!("'{w}'")).collect()))]);
    }
    l.join("\n") + "\n"
}

/// what is asked for by number: its arguments, and what writes them
fn asked(a: &Asked) -> (String, String) {
    let kind = |ty: &str| match ty { "u32" => "u32", "i32" => "i32", "f64" => "f64", "&str" => "text", _ => panic!("{}: a field is u32, i32, f64 or text, not {ty}", a.name) };
    let args: Vec<String> = a.fields.iter().map(|(f, ty)| format!("{}: {}", camel(f), if kind(ty) == "text" { "string" } else { "number" })).collect();
    let writes: String = a.fields.iter().map(|(f, ty)| format!(" this.{}({});", kind(ty), camel(f))).collect();
    (args.join(", "), format!("this.u32({});{writes}", a.n))
}
fn commands() -> String {
    let mut l = vec![format!("{WRITTEN} from rust/game_start/src/host/command.rs. To change a command, change it there."), "import { Writer } from './writer';".into(), String::new(),
        "/** what a player or a panel does that is not input, in its order, until a step takes it */".into(), "export class Commands extends Writer {".into()];
    let mut n = 0;
    for (a, doc) in command::COMMANDS.iter().zip(command::DOCS) {
        assert!(a.n >= n, "commands!: {} says {}, and the numbers only go up", a.name, a.n);
        n = a.n + 1;
        let (args, writes) = asked(a);
        l.push(format!("  /** {} */", said(doc)));
        l.push(format!("  {}({args}): void {{ {writes} }}", small(a.name)));
    }
    l.extend(["}".into(), String::new()]);
    l.join("\n")
}
fn asks() -> String {
    let mut body = Vec::new();
    let mut read = Vec::new();
    let (mut word, mut number) = (false, false);
    for (i, ((a, doc), ty)) in desk::ASKS.iter().zip(desk::DOCS).zip(desk::answers(&mut Default::default())).enumerate() {
        assert!(a.n as usize == i, "asks!: {} says {}", a.name, a.n);
        let (args, writes) = asked(a);
        let (out, reader) = match ty {
            Ty::Text => { word = true; ("string", String::from("readWord")) }
            Ty::Number("u32") => { number = true; ("number", String::from("readNumber")) }
            Ty::Named(n) => { if !read.contains(&n) { read.push(n); } (n, format!("read{n}")) }
            t => panic!("asks!: {} answers {t}: a text, a u32 or a type of its own", a.name),
        };
        let answer = format!("{reader}(this.answer(this.take()))");
        body.push(format!("  /** {} */", said(doc)));
        body.push(format!("  {}({args}): {out} {{ {writes} return {}; }}", small(a.name), if out == "Pressed" { format!("this.told({answer})") } else { answer }));
    }
    read.sort();
    let mut l = vec![format!("{WRITTEN} from rust/game_start/src/host/desk.rs. To change an ask, change it there."), "import { Writer } from './writer';".into(),
        format!("import {{ {}, {} }} from '../view/read';", read.iter().map(|n| format!("read{n}")).collect::<Vec<_>>().join(", "), read.iter().map(|n| format!("type {n}")).collect::<Vec<_>>().join(", ")), String::new()];
    if word { l.extend(["/** an answer that is one word: its length, then its bytes */".into(), "const readWord = (b: Uint8Array): string => new TextDecoder().decode(b.subarray(4));".into(), String::new()]); }
    if number { l.extend(["/** an answer that is one number */".into(), "const readNumber = (b: Uint8Array): number => new DataView(b.buffer, b.byteOffset, b.byteLength).getUint32(0, true);".into(), String::new()]); }
    l.extend(["/** what the cards before a game show and press; `answer` is the module's, to the bytes of an ask */".into(), "export class Asks extends Writer {".into(),
        "  constructor(private readonly answer: (asked: Uint8Array) => Uint8Array) { super(); }".into(), "  /** a press, as it went */".into(), "  protected told(p: Pressed): Pressed { return p; }".into()]);
    l.extend(body);
    l.extend(["}".into(), String::new()]);
    l.join("\n")
}

/// a number by Rust's name of it: the reader's, and how many bytes
const NUMBERS: [(&str, &str, usize); 8] = [("f64", "Float64", 8), ("f32", "Float32", 4), ("u32", "Uint32", 4), ("i32", "Int32", 4), ("u16", "Uint16", 2), ("i16", "Int16", 2), ("u8", "Uint8", 1), ("i8", "Int8", 1)];
/// a type in TypeScript, and what reads one off `I`
fn read(t: &Ty) -> (String, String) {
    match t {
        Ty::Number(n) => {
            let n = if *n == "usize" { "u32" } else { n };
            let (_, get, _) = NUMBERS.iter().find(|x| x.0 == n).unwrap_or_else(|| panic!("no number is a {n}"));
            ("number".into(), format!("I.{}()", get.to_lowercase()))
        }
        Ty::YesNo => ("boolean".into(), "I.bool()".into()),
        Ty::Text => ("string".into(), "I.str()".into()),
        Ty::V3 => ("Vec3".into(), "[I.float64(), I.float64(), I.float64()]".into()),
        Ty::Option(t) => { let (ts, rd) = read(t); (format!("{ts} | null"), format!("I.bool() ? {rd} : null")) }
        Ty::List(t) => { let (ts, rd) = read(t); (if ts.contains([' ', '|']) { format!("({ts})[]") } else { format!("{ts}[]") }, format!("I.list(() => {rd})")) }
        Ty::Row(r) => { let (ts, rd): (Vec<_>, Vec<_>) = r.iter().map(read).unzip(); (format!("[{}]", ts.join(", ")), format!("[{}]", rd.join(", "))) }
        Ty::Named(n) => (n.to_string(), format!("read{n}In(I)")),
    }
}
fn views() -> String {
    let mut l = vec![format!("{WRITTEN}: THE VIEW, READ, the types an answer is of (rust/game_types/src/view and what they name) and their bytes, as plain values. To change one, change it there."),
        "import type { Vec3 } from '../math/vec';".into(), String::new(),
        "class In { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }".into()];
    l.extend(NUMBERS.iter().map(|(_, get, w)| format!("  {}(): number {{ const v = this.dv.get{get}(this.p{}); this.p += {w}; return v; }}", get.to_lowercase(), if *w > 1 { ", true" } else { "" })));
    l.extend(["  bool(): boolean { return this.b[this.p++] !== 0; }", "  str(): string { const n = this.uint32(), s = DEC.decode(this.b.subarray(this.p, this.p + n)); this.p += n; return s; }",
        "  list<T>(f: () => T): T[] { const n = this.uint32(), o: T[] = []; for (let i = 0; i < n; i++) o.push(f()); return o; }",
        "  word<T>(W: readonly T[], of: string): T { const k = this.uint16(); if (k >= W.length) throw new Error(`the view: no such word of ${of} (${k})`); return W[k]; } }", "const DEC = new TextDecoder();", ""].map(String::from));
    let (whole, types) = host::answers();
    for (n, shape) in types.list {
        match shape {
            Shape::Words(w) => {
                let quoted = |by: &str| w.iter().map(|x| format!("'{x}'")).collect::<Vec<_>>().join(by);
                l.push(format!("export type {n} = {};", quoted(" | ")));
                l.push(format!("const {n}Words: readonly {n}[] = [{}];", quoted(", ")));
                l.push(format!("function read{n}In(I: In): {n} {{ return I.word({n}Words, '{n}'); }}"));
            }
            Shape::Record(r) => {
                let f: Vec<(String, (String, String))> = r.iter().map(|(f, t)| (camel(f), read(t))).collect();
                l.push(format!("export interface {n} {{ {} }}", f.iter().map(|(k, (ts, _))| format!("{k}: {ts}")).collect::<Vec<_>>().join("; ")));
                l.push(format!("function read{n}In(I: In): {n} {{ return {{ {} }}; }}", f.iter().map(|(k, (_, rd))| format!("{k}: {rd}")).collect::<Vec<_>>().join(", ")));
                if !whole.contains(&Ty::Named(n)) { continue; }
                l.push(format!("export function read{n}(b: Uint8Array): {n} {{ const I = new In(b), r = read{n}In(I); if (I.p !== b.length) throw new Error(`the view: {n} is ${{b.length}} bytes and ${{I.p}} were read`); return r; }}"));
            }
        }
    }
    l.join("\n") + "\n"
}
