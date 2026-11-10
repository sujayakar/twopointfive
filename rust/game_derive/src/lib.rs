//! WHAT A TYPE SAYS OF ITSELF, made from its declaration: a field is declared once and nothing else lists it.
//!
//!   #[derive(ToBytes)]           its bytes (game_types `bytes`): a struct's fields in their order, an enum's arm by its place and then what it holds
//!   #[bytes(skip)]               on a field: it is in no bytes; its doc says why
//!   #[bytes(parts)]              on a struct: its fields one at a time too, by name (`bytes::Parts`)
//!
//!   #[derive(Knobs)]             a table of numbers (game_types `knobs`): what it ships at, and each number by its dotted name
//!   #[k(value)]                  on a field: what it ships at. A field with none is a table of its own
//!
//!   #[derive(Words)]             an enum's words (`as_str`)
//!   #[word("…")]                 on an arm: its word, where that is not its name with a small first letter
//!
//!   #[derive(Blank)]             what a new record holds (`blank`, `Default`)
//!   #[is(value)]                 on a field: what it starts at, where that is not its type's own start
//!
//! It reads a declaration as tokens and needs no parser of the language: attributes, names, and where one field ends.
extern crate proc_macro;
use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

/// a declaration as far as it is read
struct Declared { name: String, parts: bool, body: Body }
enum Body { Struct(Fields), Enum(Vec<(String, Fields)>) }
/// each field by its name (or its place), and whether it is skipped
enum Fields { Named(Vec<(String, bool)>), Placed(Vec<bool>), None }

#[proc_macro_derive(ToBytes, attributes(bytes))]
pub fn to_bytes(item: TokenStream) -> TokenStream {
    let d = read(item);
    let mut s = format!("impl ::game_types::bytes::ToBytes for {} {{ #[allow(unused_variables)] fn w(&self, o: &mut ::game_types::bytes::W) {{ ", d.name);
    match &d.body {
        Body::Struct(f) => for name in kept(f) { s += &format!("::game_types::bytes::ToBytes::w(&self.{name}, o); "); },
        Body::Enum(arms) => {
            s += "match self { ";
            for (i, (arm, f)) in arms.iter().enumerate() {
                let (form, held) = pattern(f);
                s += &format!("Self::{arm}{form} => {{ o.b.extend_from_slice(&{i}u16.to_le_bytes()); ");
                for name in held { s += &format!("::game_types::bytes::ToBytes::w({name}, o); "); }
                s += "} ";
            }
            s += "} ";
        }
    }
    s += "} } ";
    if d.parts {
        let Body::Struct(f) = &d.body else { panic!("#[bytes(parts)] is a struct's: {}", d.name) };
        let names: Vec<String> = kept(f).iter().map(|n| format!("{:?}", n.trim_start_matches("r#"))).collect();
        s += &format!("impl ::game_types::bytes::Parts for {} {{ const NAMES: &'static [&'static str] = &[{}]; fn parts(&self, each: &mut dyn FnMut(&'static str, &dyn ::game_types::bytes::ToBytes)) {{ ", d.name, names.join(", "));
        for name in kept(f) { s += &format!("each({:?}, &self.{name}); ", name.trim_start_matches("r#")); }
        s += "} } ";
    }
    s.parse().unwrap()
}
/// what the walk knows of a field by its type: a number, a yes or no, a row of numbers, another table, or none of these (a list, a word): a value and nothing a person turns
enum Holds { Number, YesNo, Row(usize), Table, Other }
#[proc_macro_derive(Knobs, attributes(k))]
pub fn knobs(item: TokenStream) -> TokenStream {
    let mut t = item.into_iter().peekable();
    attributes(&mut t, &["parts"]);
    visibility(&mut t);
    let (kind, name) = (word(t.next()), word(t.next()));
    let Some(TokenTree::Group(g)) = t.next() else { panic!("Knobs is for a struct with named fields: {name}") };
    if kind != "struct" || g.delimiter() != Delimiter::Brace { panic!("Knobs is for a struct with named fields: {name}") }
    let rows: Vec<(String, Holds, Option<String>)> = split(g.stream()).into_iter().map(|f| {
        let mut t = f.into_iter().peekable();
        let mut value = None;
        while matches!(t.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '#') {
            t.next();
            let Some(TokenTree::Group(g)) = t.next() else { panic!("an attribute with nothing in it") };
            let mut a = g.stream().into_iter();
            if word(a.next()) != "k" { continue; }
            let Some(TokenTree::Group(v)) = a.next() else { panic!("#[k] says nothing") };
            value = Some(v.stream().to_string());
        }
        visibility(&mut t);
        let field = word(t.next());
        t.next();   // (its colon)
        let ty: String = t.map(|x| x.to_string()).collect::<String>().replace(' ', "");
        let holds = match (ty.as_str(), &value) {
            ("f64", _) => Holds::Number,
            ("bool", _) => Holds::YesNo,
            ("V3" | "[f64;3]", _) => Holds::Row(3),
            ("[f64;2]", _) => Holds::Row(2),
            ("[f64;4]", _) => Holds::Row(4),
            (_, None) => Holds::Table,
            _ => Holds::Other,
        };
        if value.is_none() && !matches!(holds, Holds::Table) { panic!("{name}.{field} says no value: #[k(…)]") }
        (field, holds, Some(value.unwrap_or(format!("<{ty}>::page()"))))
    }).collect();
    let (mut page, mut walk, mut get, mut set) = (String::new(), String::new(), String::new(), String::new());
    for (f, holds, value) in &rows {
        page += &format!("{f}: {}, ", value.as_deref().unwrap_or(""));
        match holds {
            Holds::Number => { walk += &format!("::game_types::knobs::each_of(at, {f:?}, false, self.{f}, each); "); get += &format!("({f:?}, \"\") => Some(self.{f}), "); set += &format!("({f:?}, \"\") => {{ self.{f} = v; true }} "); }
            Holds::YesNo => { walk += &format!("::game_types::knobs::each_of(at, {f:?}, true, self.{f} as u8 as f64, each); "); get += &format!("({f:?}, \"\") => Some(self.{f} as u8 as f64), "); set += &format!("({f:?}, \"\") => {{ self.{f} = v != 0.0; true }} "); }
            Holds::Row(n) => for i in 0..*n { let part = format!("{f}.{i}"); walk += &format!("::game_types::knobs::each_of(at, {part:?}, false, self.{f}[{i}], each); "); get += &format!("({f:?}, \"{i}\") => Some(self.{f}[{i}]), "); set += &format!("({f:?}, \"{i}\") => {{ self.{f}[{i}] = v; true }} "); },
            Holds::Table => { walk += &format!("::game_types::knobs::under(at, {f:?}, |at| ::game_types::knobs::KnobTable::walk(&self.{f}, at, &mut *each)); "); get += &format!("({f:?}, rest) => ::game_types::knobs::KnobTable::get(&self.{f}, rest), "); set += &format!("({f:?}, rest) => ::game_types::knobs::KnobTable::set(&mut self.{f}, rest, v), "); }
            Holds::Other => {}
        }
    }
    format!("impl {name} {{ pub fn page() -> Self {{ Self {{ {page} }} }} }} \
             impl ::core::default::Default for {name} {{ fn default() -> Self {{ Self::page() }} }} \
             impl ::game_types::knobs::KnobTable for {name} {{ \
                 #[allow(unused_variables)] fn walk(&self, at: &mut ::alloc::string::String, each: &mut dyn FnMut(&str, bool, f64)) {{ {walk} }} \
                 #[allow(unused_variables)] fn get(&self, name: &str) -> ::core::option::Option<f64> {{ match ::game_types::knobs::first_part(name) {{ {get} _ => None }} }} \
                 #[allow(unused_variables)] fn set(&mut self, name: &str, v: f64) -> bool {{ match ::game_types::knobs::first_part(name) {{ {set} _ => false }} }} \
             }}").parse().unwrap()
}
/// an enum's word for each arm: its name with a small first letter, or what `#[word("…")]` says
#[proc_macro_derive(Words, attributes(word))]
pub fn words(item: TokenStream) -> TokenStream {
    let (name, body) = named(item, "enum");
    let arms: String = split(body).into_iter().map(|arm| {
        let mut t = arm.into_iter().peekable();
        let said = said(&mut t, "word");
        let arm = word(t.next());
        let w = said.unwrap_or_else(|| format!("{:?}", arm[..1].to_lowercase() + &arm[1..]));
        format!("Self::{arm} => {w}, ")
    }).collect();
    format!("impl {name} {{ pub const fn as_str(self) -> &'static str {{ match self {{ {arms} }} }} }}").parse().unwrap()
}
/// what a new record holds (`blank`, and its `Default`): each field at its type's own start, or at what `#[is(…)]` says
#[proc_macro_derive(Blank, attributes(is))]
pub fn blank(item: TokenStream) -> TokenStream {
    let (name, body) = named(item, "struct");
    let fields: String = split(body).into_iter().map(|f| {
        let mut t = f.into_iter().peekable();
        let value = said(&mut t, "is").unwrap_or_else(|| String::from("::core::default::Default::default()"));
        visibility(&mut t);
        format!("{}: {value}, ", word(t.next()))
    }).collect();
    format!("impl {name} {{ pub fn blank() -> Self {{ Self {{ {fields} }} }} }} impl ::core::default::Default for {name} {{ fn default() -> Self {{ Self::blank() }} }}").parse().unwrap()
}
/// a declaration of this kind with a body in braces: its name and what is in them
fn named(item: TokenStream, kind: &str) -> (String, TokenStream) {
    let mut t = item.into_iter().peekable();
    attributes(&mut t, &["parts"]);
    visibility(&mut t);
    let (is, name) = (word(t.next()), word(t.next()));
    match t.next() {
        Some(TokenTree::Group(g)) if is == kind && g.delimiter() == Delimiter::Brace => (name, g.stream()),
        _ => panic!("this derive is for a {kind} with no parameters of its own: {name}"),
    }
}
/// the attributes a field or an arm opens with are passed; what the one of this name holds is the answer
fn said(t: &mut Tokens, name: &str) -> Option<String> {
    let mut said = None;
    while matches!(t.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '#') {
        t.next();
        let Some(TokenTree::Group(g)) = t.next() else { panic!("an attribute with nothing in it") };
        let mut a = g.stream().into_iter();
        if word(a.next()) == name { if let Some(TokenTree::Group(v)) = a.next() { said = Some(v.stream().to_string()); } }
    }
    said
}
/// the fields of a struct that are written, as they are named after `self.`
fn kept(f: &Fields) -> Vec<String> {
    match f {
        Fields::Named(v) => v.iter().filter(|x| !x.1).map(|x| x.0.clone()).collect(),
        Fields::Placed(v) => v.iter().enumerate().filter(|x| !x.1).map(|(i, _)| i.to_string()).collect(),
        Fields::None => Vec::new(),
    }
}
/// an arm's pattern, and the names it binds that are written
fn pattern(f: &Fields) -> (String, Vec<String>) {
    match f {
        Fields::None => (String::new(), Vec::new()),
        Fields::Placed(v) => {
            let bound: Vec<String> = v.iter().enumerate().map(|(i, skip)| if *skip { String::from("_") } else { format!("x{i}") }).collect();
            (format!("({})", bound.join(", ")), bound.into_iter().filter(|b| b != "_").collect())
        }
        Fields::Named(v) => {
            let held: Vec<String> = v.iter().filter(|x| !x.1).map(|x| x.0.clone()).collect();
            (format!(" {{ {} .. }}", held.iter().map(|n| format!("{n}, ")).collect::<String>()), held)
        }
    }
}

fn read(item: TokenStream) -> Declared {
    let mut t = item.into_iter().peekable();
    let words = attributes(&mut t, &["parts"]);
    visibility(&mut t);
    let kind = word(t.next());
    let name = word(t.next());
    let body = match (kind.as_str(), t.next()) {
        ("struct", Some(TokenTree::Group(g))) => Body::Struct(fields(&g)),
        ("struct", _) => Body::Struct(Fields::None),
        ("enum", Some(TokenTree::Group(g))) => Body::Enum(split(g.stream()).into_iter().map(|arm| {
            let mut t = arm.into_iter().peekable();
            attributes(&mut t, &[]);
            let name = word(t.next());
            (name, match t.next() { Some(TokenTree::Group(g)) => fields(&g), _ => Fields::None })
        }).collect()),
        _ => panic!("ToBytes is for a struct or an enum with no parameters of its own: {name}"),
    };
    Declared { name, parts: !words.is_empty(), body }
}
fn fields(g: &Group) -> Fields {
    let each = split(g.stream());
    let skip = |t: &mut Tokens| !attributes(t, &["skip"]).is_empty();
    match g.delimiter() {
        Delimiter::Brace => Fields::Named(each.into_iter().map(|f| { let mut t = f.into_iter().peekable(); let skipped = skip(&mut t); visibility(&mut t); (word(t.next()), skipped) }).collect()),
        _ => Fields::Placed(each.into_iter().map(|f| skip(&mut f.into_iter().peekable())).collect()),
    }
}
type Tokens = std::iter::Peekable<std::vec::IntoIter<TokenTree>>;
/// the attributes a thing opens with are passed; what `#[bytes(…)]` says among them is the answer, and it may say no word that does nothing there
fn attributes<I: Iterator<Item = TokenTree>>(t: &mut std::iter::Peekable<I>, may: &[&str]) -> Vec<String> {
    let mut said = Vec::new();
    while matches!(t.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '#') {
        t.next();
        let Some(TokenTree::Group(g)) = t.next() else { panic!("an attribute with nothing in it") };
        let mut a = g.stream().into_iter();
        if word(a.next()) != "bytes" { continue; }
        let Some(TokenTree::Group(g)) = a.next() else { panic!("#[bytes] says nothing") };
        for x in g.stream() {
            let x = x.to_string();
            if !may.contains(&x.as_str()) { panic!("#[bytes({x})]: `skip` is a field's and `parts` a type's"); }
            said.push(x);
        }
    }
    said
}
fn visibility<I: Iterator<Item = TokenTree>>(t: &mut std::iter::Peekable<I>) {
    if !matches!(t.peek(), Some(TokenTree::Ident(i)) if i.to_string() == "pub") { return; }
    t.next();
    if matches!(t.peek(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis) { t.next(); }
}
fn word(t: Option<TokenTree>) -> String { t.map_or(String::new(), |t| t.to_string()) }
/// what stands between the commas, those inside angle brackets left alone (an arrow's `>` closes none)
fn split(s: TokenStream) -> Vec<Vec<TokenTree>> {
    let (mut out, mut now, mut depth, mut dash) = (Vec::new(), Vec::new(), 0, false);
    for t in s {
        if let TokenTree::Punct(p) = &t {
            match p.as_char() {
                '<' => depth += 1,
                '>' if !dash => depth -= 1,
                ',' if depth == 0 => { out.push(std::mem::take(&mut now)); dash = false; continue; }
                _ => {}
            }
            dash = p.as_char() == '-';
        } else { dash = false; }
        now.push(t);
    }
    if !now.is_empty() { out.push(now); }
    out
}
