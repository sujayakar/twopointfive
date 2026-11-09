//! WHAT A TYPE SAYS OF ITSELF, made from its declaration: a field is declared once and nothing else lists it.
//!
//!   #[derive(ToBytes)]           its bytes (game_types `bytes`): a struct's fields in their order, an enum's arm by its place and then what it holds
//!   #[bytes(skip)]               on a field: it is in no bytes; its doc says why
//!   #[bytes(parts)]              on a struct: its fields one at a time too, by name (`bytes::Parts`)
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
