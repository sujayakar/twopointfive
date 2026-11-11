//! The TypeScript.
use game::crosses::{self, Asked, Shape, Ty};
use game::rows::RowTable;
use game_start::host::{calls::{self, Back, CALLS}, command, crosses as host, desk};

/// the stamp of everything that crosses (game_face `face_draw_layout` answers the module's)
pub fn stamp() -> u32 { crosses::stamp(&(game_draw::layout() + &host::text())) }

pub fn files(stamp: u32) -> Vec<(&'static str, String)> {
    vec![("src/draw/sections.ts", sections(stamp)), ("src/draw/gpu.ts", gpu()), ("src/draw/commands.ts", commands()), ("src/draw/asks.ts", asks()), ("src/draw/face.ts", face()), ("src/view/read.ts", views())]
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
        None => match ty.trim() { "Half2" => (1, "two halves"), t => (1, game_draw::gpu::PACKED.iter().find(|p| p.name == t).map_or(t, |p| if p.float { "f32" } else { "u32" })) },
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
    for p in game_draw::gpu::PACKED {
        if p.float { continue; }   // (the module's alone to write)
        let (name, mask) = (small(p.name), |bits: u32| (1u64 << bits) - 1);
        l.extend([String::new(), format!("/** {} */", said(p.doc)),
            format!("export const {name} = ({}): number => ({}) >>> 0;", p.parts.iter().map(|x| format!("{}: number", camel(x.0))).collect::<Vec<_>>().join(", "),
                p.parts.iter().map(|(n, at, bits)| format!("({} & {}) << {at}", camel(n), mask(*bits))).collect::<Vec<_>>().join(" | "))]);
        for (n, at, bits) in p.parts { let n = camel(n); l.push(format!("export const {name}{} = (w: number): number => w >>> {at} & {};", n[..1].to_uppercase() + &n[1..], mask(*bits))); }
    }
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

/// what is handed, by Rust's name of its type: its type in TypeScript, and what writes `x` of it
fn handed(ty: &str, x: &str) -> (String, String) {
    let same = |ts: &str, w: &str| (String::from(ts), format!("this.{w}({x});"));
    let inner = |t: &str| t.trim().trim_end_matches('>').to_string();
    match ty.replace(' ', "").as_str() {
        "u32" | "usize" => same("number", "u32"),
        "i32" => same("number", "i32"),
        "f64" => same("number", "f64"),
        "bool" => same("boolean", "yes"),
        "&str" => same("string", "text"),
        "&[u8]" => same("Uint8Array", "bytes"),
        "[f64;3]" => same("ArrayLike<number>", "three"),
        "Vec<f64>" => same("ArrayLike<number>", "doubles"),
        t @ ("GameNo" | "MomentNo") => same(t, "u32"),
        t => match (t.strip_prefix("Option<").map(inner), t.strip_prefix("Vec<").map(inner)) {
            (Some(t), _) => { let (ts, w) = handed(&t, x); (format!("{ts} | null"), format!("this.yes({x} !== null); if ({x} !== null) {w}")) }
            (_, Some(t)) => { let (ts, w) = handed(&t, "v"); (format!("readonly {ts}[]"), format!("this.u32({x}.length); for (const v of {x}) {w}")) }
            _ => panic!("{ty} is nothing that is handed (host/field.rs)"),
        },
    }
}
/// its arguments, and what writes them
fn arguments(fields: &[(&str, &str)]) -> (String, String) {
    let (args, writes): (Vec<_>, Vec<_>) = fields.iter().map(|(f, ty)| { let f = camel(f); let (ts, w) = handed(ty, &f); (format!("{f}: {ts}"), w) }).unzip();
    (args.join(", "), writes.join(" "))
}
/// what is asked for by number
fn asked(a: &Asked) -> (String, String) {
    let (args, writes) = arguments(a.fields);
    (args, format!("this.u32({}); {writes}", a.n).trim_end().to_string())
}
/// the module's calls
fn face() -> String {
    let (mut types, mut readers, mut body) = (Vec::new(), Vec::new(), Vec::new());
    for (c, doc) in CALLS.iter().zip(calls::DOCS) {
        let (mut args, mut writes) = arguments(c.args);
        if c.held { args = if args.is_empty() { "h: GameNo".into() } else { format!("h: GameNo, {args}") }; }
        if !writes.is_empty() { writes.push(' '); }
        let call = format!("this.X.face_{}({}, {})", c.name, if c.held { "h" } else { "0" }, if c.args.is_empty() { "0, 0" } else { "this.at, this.sent()" });
        let (ts, back) = match c.back {
            Back::Nothing => ("void".into(), format!("{call};")),
            Back::Number("bool") => ("boolean".into(), format!("return {call} !== 0;")),
            Back::Number(n @ ("GameNo" | "MomentNo")) => (String::from(n), format!("return {call} as {n};")),
            Back::Number(_) => ("number".into(), format!("return {call};")),
            Back::Told(ty, _) => {
                let ty = ty(&mut Default::default());
                named(&ty, &mut types, &mut readers);
                match &ty {
                    Ty::Row(r) if r.is_empty() => ("void".into(), format!("this.told({call}, nothing);")),
                    // (a run of numbers, whole: an array of its own)
                    Ty::List(n) if matches!(**n, Ty::Number(_)) => { let Ty::Number(n) = **n else { unreachable!() }; let get = number(n).1; (format!("{get}Array<ArrayBuffer>"), format!("return this.told({call}, I => I.run({get}Array));")) }
                    Ty::Named(n) => (n.to_string(), format!("return this.told({call}, read{n}In);")),
                    t => { let (ts, rd) = read(t); (ts, format!("return this.told({call}, I => {rd});")) }
                }
            }
        };
        if !doc.is_empty() { body.push(format!("  /** {} */", said(doc))); }
        body.push(format!("  {}({args}): {ts} {{ {writes}{back} }}", camel(c.name)));
    }
    types.sort();
    readers.sort();
    let mut l = vec![format!("{WRITTEN} from rust/game_start/src/host/calls.rs. To change a call, change it there."), "import type { Vec3 } from '../math/vec';".into(), "import { Wire, nothing, type Carrier } from './wire';".into(),
        format!("import {{ {}, {} }} from '../view/read';", readers.iter().map(|n| format!("read{n}In")).collect::<Vec<_>>().join(", "), types.iter().map(|n| format!("type {n}")).collect::<Vec<_>>().join(", ")), String::new(),
        "/** a game's number in the module, and a moment's: each is what the module answered with (0: none) */".into(), "export type GameNo = number & { readonly of: 'game' };".into(), "export type MomentNo = number & { readonly of: 'moment' };".into(), String::new(),
        "/** the module's exports: the game, where it is of one, and where its arguments lie */".into()];
    let names = |held: bool, by: &str| CALLS.iter().filter(|c| c.held || !held).map(|c| format!("'face_{}'", c.name)).collect::<Vec<_>>().join(by);
    l.push(format!("export type FaceExports = Carrier & Record<{}, (h: number, p: number, n: number) => number>;", names(false, " | ")));
    l.extend(["/** those that hold a game: none is asked of one lost to a panic */".into(), format!("export const HOLDS: ReadonlySet<string> = new Set([{}]);", names(true, ", ")), String::new(),
        "/** THE MODULE, CALLED */".into(), "export class Face extends Wire<FaceExports> {".into()]);
    l.extend(body);
    l.extend(["}".into(), String::new()]);
    l.join("\n")
}
/// the types an answer names itself, and those whose readers it is read by
fn named(t: &Ty, types: &mut Vec<&'static str>, readers: &mut Vec<&'static str>) {
    match t {
        Ty::Named(n) => for l in [types, readers] { if !l.contains(n) { l.push(n); } },
        Ty::Option(t) | Ty::List(t) => named(t, types, readers),
        Ty::Row(r) => for t in r { named(t, types, readers); },
        _ => {}
    }
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
fn number(n: &str) -> &'static (&'static str, &'static str, usize) { let n = if n == "usize" { "u32" } else { n }; NUMBERS.iter().find(|x| x.0 == n).unwrap_or_else(|| panic!("no number is a {n}")) }
/// a type in TypeScript, and what reads one off `I`
fn read(t: &Ty) -> (String, String) {
    match t {
        Ty::Number(n) => {
("number".into(), format!("I.{}()", number(n).1.to_lowercase()))
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
        "export class In { p = 0; dv: DataView; constructor(readonly b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }".into()];
    l.extend(NUMBERS.iter().map(|(_, get, w)| format!("  {}(): number {{ const v = this.dv.get{get}(this.p{}); this.p += {w}; return v; }}", get.to_lowercase(), if *w > 1 { ", true" } else { "" })));
    l.extend(["  bool(): boolean { return this.b[this.p++] !== 0; }", "  str(): string { const n = this.uint32(), s = DEC.decode(this.b.subarray(this.p, this.p + n)); this.p += n; return s; }",
        "  /** a list of numbers as an array of its own */", "  run<T>(A: { new (b: ArrayBuffer): T; readonly BYTES_PER_ELEMENT: number }): T { const n = this.uint32() * A.BYTES_PER_ELEMENT, at = this.b.byteOffset + this.p; this.p += n; return new A((this.b.buffer as ArrayBuffer).slice(at, at + n)); }",
        "  list<T>(f: () => T): T[] { const n = this.uint32(), o: T[] = []; for (let i = 0; i < n; i++) o.push(f()); return o; }",
        "  word<T>(W: readonly T[], of: string): T { const k = this.uint16(); if (k >= W.length) throw new Error(`the view: no such word of ${of} (${k})`); return W[k]; } }", "const DEC = new TextDecoder();", ""].map(String::from));
    let (asked, told, types) = host::answers();
    let (mut own, mut whole) = (Vec::new(), Vec::new());
    for t in &told { named(t, &mut own, &mut whole); }
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
                l.push(format!("{}function read{n}In(I: In): {n} {{ return {{ {} }}; }}", if whole.contains(&n) { "export " } else { "" }, f.iter().map(|(k, (_, rd))| format!("{k}: {rd}")).collect::<Vec<_>>().join(", ")));
                if !asked.contains(&Ty::Named(n)) { continue; }
                l.push(format!("export function read{n}(b: Uint8Array): {n} {{ const I = new In(b), r = read{n}In(I); if (I.p !== b.length) throw new Error(`the view: {n} is ${{b.length}} bytes and ${{I.p}} were read`); return r; }}"));
            }
        }
    }
    l.join("\n") + "\n"
}
