//! The fields of a type on the wire that its TypeScript no longer declares.
//!
//! A struct of the wire's table (tools/blitz/schema.json) mirrors a TypeScript interface, field for field, each with the name
//! the TypeScript has for it. When the interface loses a member the mirror keeps the field, inert: carried over the wire and
//! read by nothing. This lists them, by name, for `tools/wire/wire.ts drop --file` and `fold fields-drop --file`.

use std::path::Path;

/// The text with its comments blanked (a string's content is kept: a `//` inside one is no comment).
fn no_comments(ts: &str) -> String {
    let b: Vec<char> = ts.chars().collect();
    let mut out = String::with_capacity(ts.len());
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c == '/' && b.get(i + 1) == Some(&'/') {
            while i < b.len() && b[i] != '\n' {
                i += 1;
            }
        } else if c == '/' && b.get(i + 1) == Some(&'*') {
            i += 2;
            while i < b.len() && !(b[i] == '*' && b.get(i + 1) == Some(&'/')) {
                if b[i] == '\n' {
                    out.push('\n');
                }
                i += 1;
            }
            i += 2;
        } else if c == '\'' || c == '"' || c == '`' {
            out.push(c);
            i += 1;
            while i < b.len() && b[i] != c {
                if b[i] == '\\' {
                    out.push(b[i]);
                    i += 1;
                }
                if i < b.len() {
                    out.push(b[i]);
                    i += 1;
                }
            }
            if i < b.len() {
                out.push(c);
                i += 1;
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// The members `export interface NAME { … }` declares, in order: the names at the interface's own depth.
pub fn interface_members(ts: &str, name: &str) -> Option<Vec<String>> {
    let src = no_comments(ts);
    let head = format!("interface {name}");
    let mut from = 0;
    let at = loop {
        let i = from + src[from..].find(&head)?;
        let after = src[i + head.len()..].chars().next();
        if after.map(|c| !(c.is_alphanumeric() || c == '_' || c == '$')).unwrap_or(false) {
            break i;
        }
        from = i + head.len();
    };
    let open = at + src[at..].find('{')?;
    let mut depth = 0i32;
    let mut names = Vec::new();
    let mut chunk = String::new();
    let take = |chunk: &mut String, names: &mut Vec<String>| {
        let c = chunk.trim_start();
        let c = c.strip_prefix("readonly ").unwrap_or(c).trim_start();
        let id: String = c.chars().take_while(|x| x.is_alphanumeric() || *x == '_' || *x == '$').collect();
        let rest = c[id.len()..].trim_start();
        let rest = rest.strip_prefix('?').unwrap_or(rest).trim_start();
        if !id.is_empty() && rest.starts_with(':') {
            names.push(id);
        }
        chunk.clear();
    };
    for c in src[open..].chars() {
        match c {
            '{' | '(' | '[' => {
                depth += 1;
                if depth > 1 {
                    chunk.push(c);
                }
            }
            '}' | ')' | ']' => {
                depth -= 1;
                if depth == 0 {
                    take(&mut chunk, &mut names);
                    return Some(names);
                }
                chunk.push(c);
            }
            ';' | ',' | '\n' if depth == 1 => take(&mut chunk, &mut names),
            c => chunk.push(c),
        }
    }
    None
}

pub struct Gone {
    /// `mod::Type.field`, a field the interface no longer declares
    pub names: Vec<String>,
    pub declared: usize,
    pub on_wire: usize,
    /// members the interface declares and the wire's type has no field for
    pub not_on_wire: Vec<String>,
}

/// `ty`: `mod::Type` of the wire's table. `interface`: the TypeScript interface's name, the type's own where none is given.
pub fn gone(root: &Path, ty: &str, ts_file: &str, interface: Option<&str>) -> Result<Gone, String> {
    let schema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(root.join("tools/blitz/schema.json")).map_err(|e| format!("cannot read tools/blitz/schema.json: {e}"))?).map_err(|e| format!("schema.json: {e}"))?;
    let t = &schema["types"][format!("crate::{ty}")];
    let fields = t["fields"].as_array().ok_or(format!("{ty} is no struct of the wire's table"))?;
    let name = interface.or(t["name"].as_str()).unwrap_or("");
    let ts = std::fs::read_to_string(root.join(ts_file)).map_err(|e| format!("cannot read {ts_file}: {e}"))?;
    let members = interface_members(&ts, name).ok_or(format!("{ts_file} declares no interface {name}"))?;
    let mut out = Gone { names: Vec::new(), declared: members.len(), on_wire: fields.len(), not_on_wire: Vec::new() };
    for f in fields {
        let (Some(rust), Some(tsn)) = (f["rust"].as_str(), f["ts"].as_str()) else { continue };
        if !members.iter().any(|m| m == tsn) {
            out.names.push(format!("{ty}.{rust}"));
        }
    }
    out.not_on_wire = members.iter().filter(|m| !fields.iter().any(|f| f["ts"].as_str() == Some(m.as_str()))).cloned().collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_members_of_an_interface_are_its_own_depth_only() {
        let ts = "// interface Settings { not: this }\nexport interface SettingsMore { x: number }\nexport interface Settings {\n  a: number; b?: boolean;   // c: gone\n  /* d: number; */ readonly e: 'x' | 'y';\n  f: { inner: number; other: string };\n  g(n: number): void;\n  h: string, i: number\n}\nconst j: number = 1;";
        assert_eq!(interface_members(ts, "Settings").unwrap(), ["a", "b", "e", "f", "h", "i"]);
        assert_eq!(interface_members(ts, "SettingsMore").unwrap(), ["x"]);
        assert!(interface_members(ts, "Other").is_none());
    }
}
