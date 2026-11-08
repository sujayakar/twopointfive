//! THE PINS (`rust/game_start/pins.txt`): what the rows are held to, as text that a commit's diff can be read in. Made by `gate pin`, read by the rows. Three sections.
//! ROWS: every row's name, a line each, in order: a mission or a run that leaves the list leaves its pins with no row, and a whole `gate` says so.
//! STARTS: the columns that are refused with the game's words, a line of the columns' names, then a line a part: its name, then an 8-digit word a column, or one word where every column has
//! the same. A part that joins or leaves the state is then one line of the file. SERIES: a line a run: its name and a colon, then a word a second.
use crate::rows::Row;
use std::collections::BTreeMap;

const HEAD: &str = "# THE PINS OF THE GATE. Made by `gate pin`, read by its rows; never by hand. A commit that moves a pin says why (rust/game_start/README.md, \"The gate\").";
/// no word: this column was recorded before the part was a part
const NONE: &str = "--------";

#[derive(Clone, PartialEq)]
pub enum Start { Refused(String), Parts(BTreeMap<String, u32>) }

#[derive(Default)]
pub struct Pins { pub rows: Vec<String>, pub starts: BTreeMap<String, Start>, pub series: BTreeMap<String, Vec<u32>> }

pub fn path() -> std::path::PathBuf { std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pins.txt") }
fn hex(w: u32) -> String { format!("{:08x}", w) }
fn word(w: &str) -> u32 { u32::from_str_radix(w, 16).unwrap_or_else(|_| panic!("pins.txt: '{}' is no word", w)) }

impl Pins {
    pub fn read_file() -> Pins { std::fs::read_to_string(path()).map_or_else(|_| Pins::default(), |t| Pins::read(&t)) }
    pub fn read(text: &str) -> Pins {
        let mut p = Pins::default();
        let (mut columns, mut section): (Vec<String>, u8) = (Vec::new(), 0);
        for l in text.lines() {
            if l.starts_with('#') || l.is_empty() { continue; }
            if l == "rows" { section = 3; continue; }
            if l == "starts" { section = 1; continue; }
            if l == "series" { section = 2; continue; }
            if section == 3 { p.rows.push(l.to_string()); continue; }
            if section == 2 {
                let (name, words) = l.split_once(": ").unwrap_or_else(|| panic!("pins.txt: '{}' is no line of a run", l));
                p.series.insert(name.to_string(), words.split(' ').map(word).collect());
                continue;
            }
            let (first, rest) = l.split_once(' ').unwrap_or((l, ""));
            match first {
                "refused" => { let (col, why) = rest.split_once(' ').unwrap_or((rest, "")); p.starts.insert(col.to_string(), Start::Refused(why.to_string())); }
                "columns" => {
                    columns = rest.split(' ').map(String::from).collect();
                    for c in &columns { p.starts.insert(c.clone(), Start::Parts(BTreeMap::new())); }
                }
                part => {
                    let w: Vec<&str> = rest.split(' ').collect();
                    for (i, c) in columns.iter().enumerate() {
                        let at = if w.len() == 1 { w[0] } else { w[i] };
                        if at == NONE { continue; }
                        if let Some(Start::Parts(m)) = p.starts.get_mut(c) { m.insert(part.to_string(), word(at)); }
                    }
                }
            }
        }
        p
    }
    /// the file's text: columns and runs in the rows' order, parts in the state's order (a part no longer in it, kept by a partial pin, after them)
    pub fn text(&self, rows: &[Row]) -> String {
        let mut order: Vec<String> = rows.iter().filter_map(|r| r.column()).filter(|c| self.starts.contains_key(c)).collect();
        order.extend(self.starts.keys().filter(|c| !rows.iter().any(|r| r.column().as_ref() == Some(*c))).cloned());
        let mut out = format!("{}\nrows\n", HEAD);
        for r in &self.rows { out += &format!("{}\n", r); }
        out += "starts\n";
        for c in &order { if let Some(Start::Refused(why)) = self.starts.get(c) { out += &format!("refused {} {}\n", c, why); } }
        let columns: Vec<(&String, &BTreeMap<String, u32>)> = order.iter().filter_map(|c| match self.starts.get(c) { Some(Start::Parts(m)) => Some((c, m)), _ => None }).collect();
        out += &format!("columns {}\n", columns.iter().map(|c| c.0.as_str()).collect::<Vec<_>>().join(" "));
        let mut parts = game_start::parts::names();
        for (_, m) in &columns { for k in m.keys() { if !parts.contains(k) { parts.push(k.clone()); } } }
        for part in &parts {
            let w: Vec<Option<u32>> = columns.iter().map(|(_, m)| m.get(part).copied()).collect();
            if w.iter().all(|x| x.is_none()) { continue; }
            if w.iter().all(|x| *x == w[0]) { out += &format!("{} {}\n", part, hex(w[0].unwrap())); continue; }
            out += &format!("{} {}\n", part, w.iter().map(|x| x.map_or(String::from(NONE), hex)).collect::<Vec<_>>().join(" "));
        }
        out += "series\n";
        let runs: Vec<&String> = rows.iter().filter_map(|r| r.series()).filter_map(|n| self.series.get_key_value(&n).map(|x| x.0)).collect();
        for n in runs.iter().copied().chain(self.series.keys().filter(|n| !runs.contains(n))) {
            out += &format!("{}: {}\n", n, self.series[n].iter().map(|w| hex(*w)).collect::<Vec<_>>().join(" "));
        }
        out
    }
    /// what the rows' list has lost and gained against the pinned one, in the words a whole `gate` says it
    pub fn rows_differ(&self, now: &[String]) -> Vec<String> {
        let gone: Vec<&str> = self.rows.iter().filter(|r| !now.contains(r)).map(|r| r.as_str()).collect();
        let new: Vec<&str> = now.iter().filter(|r| !self.rows.contains(r)).map(|r| r.as_str()).collect();
        let mut said = Vec::new();
        if !gone.is_empty() { said.push(format!("rows gone: {}", gone.join(", "))); }
        if !new.is_empty() { said.push(format!("new rows: {} (`gate pin`)", new.join(", "))); }
        if said.is_empty() && self.rows != now { said.push(String::from("the rows are in another order (`gate pin`)")); }
        said
    }
    /// what the children recorded takes the place of what stood (`whole`: of all of it, so a column or a run that is gone goes, and the rows' list is written anew): each thing that moved, in a line
    pub fn take(&mut self, lines: &[String], whole: bool) -> Vec<String> {
        let (mut starts, mut series, mut said): (BTreeMap<String, Start>, BTreeMap<String, Vec<u32>>, Vec<String>) = (BTreeMap::new(), BTreeMap::new(), Vec::new());
        for l in lines {
            let w: Vec<&str> = l.splitn(4, '\t').collect();
            match w[..] {
                ["start", col, "refused", why] => { starts.insert(col.to_string(), Start::Refused(why.to_string())); }
                ["start", col, "parts", list] => { starts.insert(col.to_string(), Start::Parts(list.split(' ').map(|x| { let (n, v) = x.split_once('=').unwrap(); (n.to_string(), word(v)) }).collect())); }
                ["series", name, list] => { series.insert(name.to_string(), list.split(' ').map(word).collect()); }
                _ => panic!("a child's pin: '{}'", l),
            }
        }
        for (col, now) in &starts {
            match (self.starts.get(col), now) {
                (None, _) => said.push(format!("moved  start {}: a new column", col)),
                (Some(a), b) if a == b => {}
                (Some(Start::Parts(a)), Start::Parts(b)) => said.push(format!("moved  start {}: {}", col, crate::rows::moved_parts(a, b))),
                (Some(Start::Refused(a)), Start::Refused(b)) => said.push(format!("moved  start {}: refused in other words: '{}', then '{}'", col, a, b)),
                (Some(Start::Refused(a)), _) => said.push(format!("moved  start {}: it stands now, and was refused: '{}'", col, a)),
                (Some(_), Start::Refused(b)) => said.push(format!("moved  start {}: it is refused now: '{}'", col, b)),
            }
        }
        for (name, now) in &series {
            match self.series.get(name) {
                None => said.push(format!("moved  {}: a new run", name)),
                Some(a) if a == now => {}
                Some(a) => said.push(format!("moved  {}: from second {} on (`hold keep` on the build before and `hold check` on this one name the part)", name, a.iter().zip(now).position(|(x, y)| x != y).unwrap_or(a.len().min(now.len())) + 1)),
            }
        }
        if whole {
            for col in self.starts.keys().filter(|c| !starts.contains_key(*c)) { said.push(format!("moved  start {}: the column is gone", col)); }
            for name in self.series.keys().filter(|n| !series.contains_key(*n)) { said.push(format!("moved  {}: the run is gone", name)); }
            let now: Vec<String> = crate::rows::all().into_iter().map(|r| r.name).collect();
            said.extend(self.rows_differ(&now).into_iter().map(|s| format!("moved  {}", s)));
            self.rows = now;
            self.starts.clear();
            self.series.clear();
        }
        self.starts.extend(starts);
        self.series.extend(series);
        said
    }
}
