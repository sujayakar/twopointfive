//! src/game/cues.ts lines 1 to 362
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the tables

/// One alternative of a ranking row's regular expression (all case-blind, all plain ASCII but the dash, which has no case).
enum Pat {
    /// anywhere in the line
    In(&'static str),
    /// `^…`
    Head(&'static str),
    /// `^…\b`
    HeadWord(&'static str),
}

/// The ranking of the lines the defenders already had: a radio line's class by its words, first match; no row = 4.   (cues.ts CLASSES)
static CLASSES: &[(&[Pat], f64)] = &[
    (&[Pat::In("fall back"), Pat::In("falling back"), Pat::In("fell back"), Pat::In("is lost"), Pat::In("out the back"), Pat::In("get back"), Pat::In("behind me"), Pat::In("i'm coming to you")], 5.0),
    (&[Pat::In("still out there")], 6.0),
    (&[Pat::In("hold where you are"), Pat::In("lost them"), Pat::In("quiet now"), Pat::In("standing down"), Pat::In("gone quiet"), Pat::In("covering him"), Pat::In("not answering — going back")], 2.0),
    (&[Pat::In("going round"), Pat::In("going around"), Pat::In("moving round"), Pat::In("moving around"), Pat::In("locked down. box him in"), Pat::In("moving to"), Pat::In("moved on — after them"), Pat::In("coming to you"), Pat::In("coming at"), Pat::In("coming up behind"), Pat::In("with you"), Pat::In("get round"), Pat::In("get around"), Pat::In("hit the others"), Pat::In("on that door"), Pat::In("get in there"), Pat::In("in position"), Pat::HeadWord("set"), Pat::In("go go go"), Pat::In("go — go"), Pat::In("go, go"), Pat::In("pushing"), Pat::In("on my way"), Pat::In("two on this room"), Pat::In("everybody, go"), Pat::In("stand and fight"), Pat::In("charge first"), Pat::In("moving up behind"), Pat::In("coming in — i"), Pat::In("bang out"), Pat::In("move up"), Pat::In("putting fire"), Pat::In("covering — move"), Pat::Head("moving!")], 1.0),
    (&[Pat::In("lock the doors"), Pat::In("on the doors"), Pat::In("back on our doors"), Pat::In("stay on your doors"), Pat::In("digging in"), Pat::In("getting ready"), Pat::In("ready for them"), Pat::In("room ready"), Pat::In("staying on my door"), Pat::In("holding my room"), Pat::In("staying on my post"), Pat::In("holding my post"), Pat::In("let them come"), Pat::In("not coming out"), Pat::In("holding my corner"), Pat::In("holding here"), Pat::In("i've got this door"), Pat::In("on my own")], 3.0),
];

/// A kind's row of LINES (WORDS.on and WORDS.guide are settled on: `guide`, else `now`, else `say`).
struct Line {
    kind: CueKind,
    cls: f64,
    say: &'static [&'static str],
    now: Option<&'static [&'static str]>,
    guide: Option<&'static [&'static str]>,
}
/// The new lines, in the written order.   (cues.ts LINES)
static LINES: &[Line] = &[
    Line { kind: CueKind::Help, cls: 1.0, say: &["moving to the fight — {the}", "they're hit {at} — moving to them"], now: None, guide: None },
    Line { kind: CueKind::HuntOver, cls: 2.0, say: &["lost them — holding {the}", "lost them — staying {at}"], now: None, guide: None },
    Line { kind: CueKind::Calm, cls: 2.0, say: &["quiet now — staying put", "standing down — holding here"], now: None, guide: None },
    Line { kind: CueKind::Cover, cls: 2.0, say: &["one of them down {at} — covering him"], now: None, guide: None },
    Line { kind: CueKind::SquadQuiet, cls: 2.0, say: &["{sq}: gone quiet here — still holding"], now: None, guide: None },
    Line { kind: CueKind::StayOurs, cls: 3.0, say: &["they're hit {at} — staying on my door", "shots {at} — staying on my door"], now: Some(&["they're hit {at} — staying on my post", "that was {at} — staying on my post"]), guide: None },
    Line { kind: CueKind::Stay, cls: 3.0, say: &["shots {at} — holding my room", "heard that, {the} — holding my room"], now: Some(&["that was {at} — holding my post", "heard that, {the} — holding my post"]), guide: None },
    Line { kind: CueKind::SquadStays, cls: 3.0, say: &["{sq}: still on the doors — nobody goes"], now: None, guide: None },
    Line { kind: CueKind::SquadHears, cls: 4.0, say: &["{sq}: heard that — stand by"], now: None, guide: None },
    Line { kind: CueKind::SquadCopies, cls: 4.0, say: &["{sq}: copy, {ally} — stand by"], now: None, guide: None },
    Line { kind: CueKind::SquadWaits, cls: 3.0, say: &["{sq}: hold — let them come in. Wait for my word!"], now: Some(&["{sq}: hold — let them come in. wait for my word!"]), guide: Some(&["{sq}: hold — let them come in. wait for my call!"]) },
    Line { kind: CueKind::SquadCopiesWaits, cls: 3.0, say: &["{sq}: copy, {ally} — hold, let them come in. Wait for my word!"], now: Some(&["{sq}: copy, {ally} — hold, let them come in. wait for my word!"]), guide: Some(&["{sq}: copy, {ally} — hold, let them come in. wait for my call!"]) },
    Line { kind: CueKind::FellBack, cls: 5.0, say: &["{the} is lost — falling back", "falling back — {the} is lost"], now: None, guide: None },
    Line { kind: CueKind::Planted, cls: 5.0, say: &["fell back — set {at}"], now: None, guide: None },
    Line { kind: CueKind::PinnedIn, cls: 5.0, say: &["can't get out — fell back to cover {at}"], now: None, guide: None },
    Line { kind: CueKind::Still, cls: 6.0, say: &["they're still out there — holding {the}", "still out there — staying on this door"], now: Some(&["they're still out there — holding {the}", "still out there — staying on my post"]), guide: None },
    Line { kind: CueKind::Surprised, cls: 0.0, say: &["behind—!", "there—!"], now: None, guide: None },
    Line { kind: CueKind::Flinch, cls: 0.0, say: &["the door—!"], now: None, guide: None },
];
fn line_of(k: CueKind) -> &'static Line {
    LINES.iter().find(|l| l.kind == k).unwrap()
}

/// s before the same kind of cue is said again by anybody on the floor; absent = no wait.   (cues.ts KIND_GAP)
fn kind_gap(k: CueKind) -> f64 {
    match k {
        CueKind::Cover => 15.0,
        CueKind::HuntOver => 4.0,
        CueKind::Calm => 4.0,
        CueKind::Still => 6.0,
        CueKind::Stay => 3.0,
        CueKind::StayOurs => 3.0,
        _ => 0.0,
    }
}

// ---------------------------------------------------------------- the file's regular expressions, by hand

fn word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
/// the prefix test of a case-blind pattern (bytes: JavaScript's /i folds no non-ASCII letter onto an ASCII one, and the only non-ASCII in a pattern is the dash)
fn has_head_blind(hay: &[u8], needle: &[u8]) -> bool {
    hay.len() >= needle.len() && hay[..needle.len()].eq_ignore_ascii_case(needle)
}
fn has_blind(hay: &[u8], needle: &[u8]) -> bool {
    hay.len() >= needle.len() && (0..=hay.len() - needle.len()).any(|i| has_head_blind(&hay[i..], needle))
}
fn pat_hits(p: &Pat, text: &[u8]) -> bool {
    match p {
        Pat::In(s) => has_blind(text, s.as_bytes()),
        Pat::Head(s) => has_head_blind(text, s.as_bytes()),
        Pat::HeadWord(s) => has_head_blind(text, s.as_bytes()) && !text.get(s.len()).copied().is_some_and(word_byte),
    }
}
/// `/^[A-Z][a-z]{1,11}: /`: a row under a squad's name, not a man's callsign.   (cues.ts SQUAD_WORD)
fn squad_word(text: &str) -> bool {
    let b = text.as_bytes();
    if b.is_empty() || !b[0].is_ascii_uppercase() { return false; }
    let mut i = 1;
    while i < b.len() && i <= 11 && b[i].is_ascii_lowercase() { i += 1; }
    i >= 2 && i + 1 < b.len() && b[i] == b':' && b[i + 1] == b' '
}
/// `.replace(/\{(\w+)\}/g, (_m, k) => vars[k] ?? '')`
fn fill(line: &str, vars: Option<&JsMap<String, String>>) -> String {
    let b = line.as_bytes();
    let mut out = String::new();
    let (mut i, mut from) = (0, 0);
    while i < b.len() {
        if b[i] == b'{' {
            let mut j = i + 1;
            while j < b.len() && word_byte(b[j]) { j += 1; }
            if j > i + 1 && j < b.len() && b[j] == b'}' {
                out.push_str(&line[from..i]);
                if let Some(v) = vars.and_then(|m| m.get(String::from(&line[i + 1..j]))) { out.push_str(v); }
                i = j + 1;
                from = i;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&line[from..]);
    out
}
/// `\s` as the language has it
fn js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}
#[derive(Clone, Copy, PartialEq)]
enum SquadSaid { Hears, Stays, Quiet }
/// The squad's name, what it did, and whose call it heard, of
/// `/^\[squads\] (\S+) (hears of them \((\S+?)'s call\)|hears of them|stays on its doors|quiet:)/`.
fn squads_note(what: &str) -> Option<(&str, SquadSaid, Option<&str>)> {
    let rest = what.strip_prefix("[squads] ")?;
    let end = rest.find(js_space)?;
    if end == 0 { return None; }
    let (name, rest) = rest.split_at(end);
    let rest = rest.strip_prefix(' ')?;   // (\S+ cannot give a character back and still meet a space)
    if let Some(call) = rest.strip_prefix("hears of them (") {
        // \S+?: the shortest run of at least one non-space that the words follow
        let mut chars = call.char_indices();
        while let Some((_, c)) = chars.next() {
            if js_space(c) { break; }
            let at = chars.as_str();
            if at.starts_with("'s call)") { return Some((name, SquadSaid::Hears, Some(&call[..call.len() - at.len()]))); }
        }
    }
    if rest.starts_with("hears of them") { return Some((name, SquadSaid::Hears, None)); }
    if rest.starts_with("stays on its doors") { return Some((name, SquadSaid::Stays, None)); }
    if rest.starts_with("quiet:") { return Some((name, SquadSaid::Quiet, None)); }
    None
}
/// `+t.toFixed(2)`: the hundredth nearest the exact value, a tie going up (only a time in odd eighths can tie).
fn fixed2(t: f64) -> f64 {
    let e = t * 8.0;
    if e == m::floor(e) && js::rem(e, 2.0) != 0.0 { return m::ceil(t * 100.0) / 100.0; }
    alloc::format!("{:.2}", t).parse().unwrap_or(t)
}
/// A hole of the sparse `classT` / `squadT` is a NaN (no clock is ever NaN).
fn sparse_get(v: &[f64], i: usize) -> Option<f64> {
    v.get(i).copied().filter(|x| !x.is_nan())
}
fn sparse_set(v: &mut Vec<f64>, i: usize, x: f64) {
    if v.len() <= i { v.resize(i + 1, f64::NAN); }
    v[i] = x;
}
/// `kindT[kind]`, the place of a kind's clock.
fn kind_t_of(v: &mut CuesKindT, k: CueKind) -> &mut Option<f64> {
    match k {
        CueKind::Stay => &mut v.stay,
        CueKind::Calm => &mut v.calm,
        CueKind::Cover => &mut v.cover,
        CueKind::Flinch => &mut v.flinch,
        CueKind::Still => &mut v.still,
        CueKind::Help => &mut v.help,
        CueKind::StayOurs => &mut v.stay_ours,
        CueKind::HuntOver => &mut v.hunt_over,
        CueKind::FellBack => &mut v.fell_back,
        CueKind::Planted => &mut v.planted,
        CueKind::PinnedIn => &mut v.pinned_in,
        CueKind::SquadHears => &mut v.squad_hears,
        CueKind::SquadCopies => &mut v.squad_copies,
        CueKind::SquadWaits => &mut v.squad_waits,
        CueKind::SquadCopiesWaits => &mut v.squad_copies_waits,
        CueKind::SquadStays => &mut v.squad_stays,
        CueKind::SquadQuiet => &mut v.squad_quiet,
        CueKind::Surprised => &mut v.surprised,
    }
}
/// `kinds[kind]`, the place of a kind's tally.
fn kinds_of(v: &mut CuesKinds, k: CueKind) -> &mut Option<f64> {
    match k {
        CueKind::Stay => &mut v.stay,
        CueKind::Calm => &mut v.calm,
        CueKind::Cover => &mut v.cover,
        CueKind::Flinch => &mut v.flinch,
        CueKind::Still => &mut v.still,
        CueKind::Help => &mut v.help,
        CueKind::StayOurs => &mut v.stay_ours,
        CueKind::HuntOver => &mut v.hunt_over,
        CueKind::FellBack => &mut v.fell_back,
        CueKind::Planted => &mut v.planted,
        CueKind::PinnedIn => &mut v.pinned_in,
        CueKind::SquadHears => &mut v.squad_hears,
        CueKind::SquadCopies => &mut v.squad_copies,
        CueKind::SquadWaits => &mut v.squad_waits,
        CueKind::SquadCopiesWaits => &mut v.squad_copies_waits,
        CueKind::SquadStays => &mut v.squad_stays,
        CueKind::SquadQuiet => &mut v.squad_quiet,
        CueKind::Surprised => &mut v.surprised,
    }
}
/// Where `slice(-n)` of a list starts (`slice(-0)` is the whole list).
fn tail_from(len: usize, n: f64) -> usize {
    let n = n as usize;
    if n == 0 { 0 } else { len.saturating_sub(n) }
}
fn file_kind(k: CueKind) -> CuesFileKind {
    match k {
        CueKind::Stay => CuesFileKind::Stay,
        CueKind::Calm => CuesFileKind::Calm,
        CueKind::Cover => CuesFileKind::Cover,
        CueKind::Flinch => CuesFileKind::Flinch,
        CueKind::Still => CuesFileKind::Still,
        CueKind::Help => CuesFileKind::Help,
        CueKind::StayOurs => CuesFileKind::StayOurs,
        CueKind::HuntOver => CuesFileKind::HuntOver,
        CueKind::FellBack => CuesFileKind::FellBack,
        CueKind::Planted => CuesFileKind::Planted,
        CueKind::PinnedIn => CuesFileKind::PinnedIn,
        CueKind::SquadHears => CuesFileKind::SquadHears,
        CueKind::SquadCopies => CuesFileKind::SquadCopies,
        CueKind::SquadWaits => CuesFileKind::SquadWaits,
        CueKind::SquadCopiesWaits => CuesFileKind::SquadCopiesWaits,
        CueKind::SquadStays => CuesFileKind::SquadStays,
        CueKind::SquadQuiet => CuesFileKind::SquadQuiet,
        CueKind::Surprised => CuesFileKind::Surprised,
    }
}
fn log_kind(k: CuesFileKind) -> CuesLogItemKind {
    match k {
        CuesFileKind::Heard => CuesLogItemKind::Heard,
        CuesFileKind::Stay => CuesLogItemKind::Stay,
        CuesFileKind::Calm => CuesLogItemKind::Calm,
        CuesFileKind::Cover => CuesLogItemKind::Cover,
        CuesFileKind::Call => CuesLogItemKind::Call,
        CuesFileKind::Flinch => CuesLogItemKind::Flinch,
        CuesFileKind::Still => CuesLogItemKind::Still,
        CuesFileKind::Help => CuesLogItemKind::Help,
        CuesFileKind::StayOurs => CuesLogItemKind::StayOurs,
        CuesFileKind::HuntOver => CuesLogItemKind::HuntOver,
        CuesFileKind::FellBack => CuesLogItemKind::FellBack,
        CuesFileKind::Planted => CuesLogItemKind::Planted,
        CuesFileKind::PinnedIn => CuesLogItemKind::PinnedIn,
        CuesFileKind::SquadHears => CuesLogItemKind::SquadHears,
        CuesFileKind::SquadCopies => CuesLogItemKind::SquadCopies,
        CuesFileKind::SquadWaits => CuesLogItemKind::SquadWaits,
        CuesFileKind::SquadCopiesWaits => CuesLogItemKind::SquadCopiesWaits,
        CuesFileKind::SquadStays => CuesLogItemKind::SquadStays,
        CuesFileKind::SquadQuiet => CuesLogItemKind::SquadQuiet,
        CuesFileKind::Surprised => CuesLogItemKind::Surprised,
        CuesFileKind::Raw => CuesLogItemKind::Raw,
    }
}

// ---------------------------------------------------------------- the functions of the module

/// A radio line's class by its words, first match; none = the net knowing something.   (cues.ts cueClass)
pub fn cue_class(text: &str) -> f64 {
    let b = text.as_bytes();
    for (pats, c) in CLASSES {
        if pats.iter().any(|p| pat_hits(p, b)) { return *c; }
    }
    4.0
}

/// A kind's lines as they are said now.   (cues.ts said)
pub fn said(g: &mut crate::game::Game, k: crate::cues::CueKind) -> Vec<String> {
    let l = line_of(k);
    l.guide.or(l.now).unwrap_or(l.say).iter().map(|s| String::from(*s)).collect()
}

/// A line is shouted when its writer put an exclamation mark in it.   (cues.ts isShout)
pub fn is_shout(text: &str) -> bool {
    text.contains('!')
}

/// Every new line's words, for the QA probe.   (cues.ts cueLines)
pub fn cue_lines(g: &mut crate::game::Game) -> Vec<crate::cues::CueLinesItem> {
    let mut out = Vec::new();
    for l in LINES {
        for text in said(g, l.kind) { out.push(CueLinesItem { kind: l.kind, cls: l.cls, text }); }
    }
    out
}

/// The tallies as a new encounter has them.   (cues.ts newStats)
pub fn new_stats() -> crate::cues::CuesStats {
    CuesStats {
        heard: 0.0,
        heard_drawn: 0.0,
        shouts: 0.0,
        shouts_drawn: 0.0,
        cues: 0.0,
        cue_net: 0.0,
        cue_drawn: 0.0,
        dup: 0.0,
        gap: 0.0,
        full: 0.0,
        bumped: 0.0,
        bumped_young: 0.0,
        filed: vec![0.0; 7],
        drawn: vec![0.0; 7],
    }
}

/// The distance on the floor.   (cues.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A new Cues: the field initialisers, nothing more.   (cues.ts Cues.constructor)
pub fn cues_new(g: &mut crate::game::Game) -> crate::cues::Cues {
    Cues::blank()
}

/// The encounter is another one (a restart, a checkpoint loaded: the clock may have gone back).   (cues.ts Cues.reset)
pub fn cues_reset(g: &mut crate::game::Game) {
    g.cues.heard_row = None;
    g.cues.heard_log.clear();
    g.cues.heard_said.clear();
    g.cues.shout_t = None;
    g.cues.rows.clear();
    g.cues.bubbles = JsMap::new();
    g.cues.men = JsMap::new();
    g.cues.last_note = None;
    g.cues.said.clear();
    g.cues.class_t.clear();
    g.cues.squad_t.clear();
    g.cues.kind_t = CuesKindT::zeroed();
    g.cues.squelch_due = false;
    g.cues.stats = new_stats();
    g.cues.kinds = CuesKinds::zeroed();
    g.cues.log.clear();
}

/// Drops the rows no longer up, in place: what `alive()` does to `this.rows`, the array `file` then goes on writing.
fn prune_rows(g: &mut Game) {
    let (t, secs) = (g.time, g.k.cues.secs);
    g.cues.rows.retain(|r| r.t <= t && t - r.t < secs);
}

/// The rows still up, oldest first.   (cues.ts Cues.alive)
pub fn cues_alive(g: &mut crate::game::Game) -> Vec<crate::cues::CueRow> {
    prune_rows(g);
    g.cues.rows.clone()
}

/// `done` of `file`: tally, log, answer.
fn file_done(g: &mut Game, t: f64, text: &str, cls: f64, who: &str, kind: CuesFileKind, drawn: bool, why: &str) -> bool {
    if drawn { g.cues.stats.drawn[cls as usize] += 1.0; }
    g.cues.log.push(CuesLogItem { t: fixed2(t), who: String::from(who), text: String::from(text), cls, kind: log_kind(kind), drawn, why: String::from(why) });
    if g.cues.log.len() > 400 { g.cues.log.drain(0..100); }
    drawn
}

/// One row for the strip, or not; `raw` is always drawn. `classT`, the gate's clock, is read by laststand.ts through netWait.   (cues.ts Cues.file)
pub fn cues_file(g: &mut crate::game::Game, text: &str, cls: f64, who: &str, kind: crate::cues::CuesFileKind) -> bool {
    let t = g.time;
    prune_rows(g);   // (`live` of the TypeScript IS this.rows: every write below goes to g.cues.rows)
    let c = cls as usize;
    g.cues.stats.filed[c] += 1.0;
    // (the same words from another man are the same news)
    let words: &str = if kind == CuesFileKind::Raw || who.is_empty() { text } else {
        match text.strip_prefix(who).and_then(|r| r.strip_prefix(": ")) { Some(r) => r, None => text }
    };
    let squad = squad_word(text);
    let rank = cls * 2.0 + (if squad { 0.0 } else { 1.0 });
    let within = |at: Option<f64>, secs: f64| at.is_some_and(|at| t >= at && t - at < secs);
    if cls > 0.0 {
        if within(g.cues.said.get(String::from(words)).copied(), g.k.cues.dup_secs) {
            g.cues.stats.dup += 1.0;
            return file_done(g, t, text, cls, who, kind, false, "dup");
        }
        let gap = g.k.cues.gaps.get(c).copied().unwrap_or(0.0);   // (a squad's word waits only on another squad's; with the strip nearly empty nobody waits)
        let clock = if squad { sparse_get(&g.cues.squad_t, c) } else { sparse_get(&g.cues.class_t, c) };
        if g.cues.rows.len() as f64 >= g.k.cues.rows - 1.0 && within(clock, gap) {
            g.cues.stats.gap += 1.0;
            return file_done(g, t, text, cls, who, kind, false, "gap");
        }
    }
    if g.cues.rows.len() as f64 >= g.k.cues.rows {   // the strip is full: whose place does it take?
        let (per_class, min_show) = (g.k.cues.per_class, g.k.cues.min_show);
        let live = &g.cues.rows;
        let held = |c: f64| live.iter().filter(|r| r.cls == c).count() as f64;
        let over = live.iter().position(|r| r.cls > 0.0 && r.cls != cls && held(r.cls) > per_class);
        let v;
        let mut no_room = false;
        if let Some(o) = over {
            v = o;   // another class holds more than its share: its oldest row
        } else if cls > 0.0 && held(cls) >= per_class {
            v = live.iter().position(|r| r.cls == cls).unwrap();   // its own class has its share: the older of those, once that has been read
            if t - live[v].t < min_show { no_room = true; }
        } else {
            let mut w = 0;   // else the worst-ranked row, the oldest of them: never a better one
            for i in 1..live.len() {
                if live[i].rank > live[w].rank { w = i; }
            }
            v = w;
            if rank > live[v].rank || (rank == live[v].rank && cls > 0.0 && t - live[v].t < min_show) { no_room = true; }
        }
        if no_room {
            g.cues.stats.full += 1.0;
            return file_done(g, t, text, cls, who, kind, false, "full");
        }
        if t - g.cues.rows[v].t < 1.5 { g.cues.stats.bumped_young += 1.0; }
        g.cues.rows.remove(v);
        g.cues.stats.bumped += 1.0;
    }
    if cls > 0.0 { g.cues.squelch_due = true; }   // (a row that arrives is heard arriving: tick, once a frame)
    g.cues.rows.push(CueRow { t, text: String::from(text), cls, rank, who: String::from(who) });
    g.cues.said.set(String::from(words), t);
    sparse_set(&mut g.cues.class_t, c, t);
    if squad { sparse_set(&mut g.cues.squad_t, c, t); }
    if g.cues.heard_row.is_some() && g.cues.rows.len() as f64 >= g.k.cues.rows { g.cues.heard_row = None; }   // (a shout heard gives its place up, for good)
    if g.cues.said.len() > 96 {
        let dup_secs = g.k.cues.dup_secs;
        for k in g.cues.said.keys() {
            let v = *g.cues.said.get(k.clone()).unwrap();
            if t - v > dup_secs || v > t { g.cues.said.delete(k); }
        }
        g.cues.said.pack();
    }
    file_done(g, t, text, cls, who, kind, true, "")
}

/// A row written 'radio: …' (control, the staff's net): on the strip as it always was.   (cues.ts Cues.raw)
pub fn cues_raw(g: &mut crate::game::Game, text: &str) {
    if let Some(r) = text.strip_prefix("radio: ") { cues_file(g, r, 0.0, "", CuesFileKind::Raw); }
}

/// The row a man's net line files: under his callsign, a squad's word under the squad's name already.
fn net_row(callsign: &str, text: &str) -> String {
    if squad_word(text) { String::from(text) } else { alloc::format!("{}: {}", callsign, text) }
}

/// A unit's line went out on his side's net: the strip gets it.   (cues.ts Cues.heard)
pub fn cues_heard(g: &mut crate::game::Game, gd: GuardId, text: &str) {
    if g.gd[gd].team != crate::teams::Team::Hostile || !crate::roomfight::fight_live(g) { return; }
    g.cues.stats.heard += 1.0;
    let callsign = g.gd[gd].callsign.clone();
    if cues_file(g, &net_row(&callsign, text), cue_class(text), &callsign, CuesFileKind::Heard) { g.cues.stats.heard_drawn += 1.0; }
}

/// A defender's line that is telling only: the strip's row, and this file's own bubble (never Guard.bubble).   (cues.ts Cues.net)
pub fn cues_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: f64) {
    let t = g.time;
    g.cues.bubbles.set(gd, Bubble { text: String::from(text), t, dur: crate::guards::bubble_secs(text), radio: true });
    if !crate::roomfight::fight_live(g) { return; }
    g.cues.stats.heard += 1.0;
    let callsign = g.gd[gd].callsign.clone();
    if cues_file(g, &net_row(&callsign, text), cls, &callsign, CuesFileKind::Heard) { g.cues.stats.heard_drawn += 1.0; }
}

/// A teammate calls what the fog hides: a bubble, and a row in the first class. Whether the strip drew it.   (cues.ts Cues.call)
pub fn cues_call(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool {
    let t = g.time;
    g.cues.bubbles.set(gd, Bubble { text: String::from(text), t, dur: crate::guards::bubble_secs(text), radio: false });
    let callsign = g.gd[gd].callsign.clone();
    cues_file(g, &alloc::format!("{}: {}", callsign, text), 1.0, &callsign, CuesFileKind::Call)
}

/// Every ear of the team's a shout could reach.   (cues.ts Cues.ears)
pub fn cues_ears(g: &mut crate::game::Game) -> Vec<V3> {
    let at = |p: V3| -> V3 { [p[0], p[1] + 1.6, p[2]] };
    let mut out = Vec::new();
    let who = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &who) {
        if !g.pl[pl].down { out.push(at(g.ch[g.pl[pl].ch].pos)); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let c = g.gd[o].ch;
        if g.gd[o].team == crate::teams::Team::Operative && g.gd[o].state != crate::game::GuardState::Dead && g.ch[c].alive { out.push(at(g.ch[c].pos)); }
    }
    out
}

/// `done` of `overheard`.
fn heard_done(g: &mut Game, t: f64, text: &str, why: &str) -> bool {
    g.cues.heard_log.push(CuesHeardLogItem { t: fixed2(t), text: String::from(text), drawn: why.is_empty(), why: String::from(why) });
    if g.cues.heard_log.len() > 200 { g.cues.heard_log.drain(0..50); }
    why.is_empty()
}

/// A hidden defender's shout that an ear of the team's really gets: a row of its own, his words alone. Whether the strip drew it.   (cues.ts Cues.overheard)
pub fn cues_overheard(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool {
    let t = g.time;
    if !is_shout(text) || g.gd[gd].state == crate::game::GuardState::Dead { return false; }
    let team = g.gd[gd].team;
    if !crate::teams::is_hostile(g, team, crate::teams::Team::Player) || !crate::fogofwar::fog_of_war_hidden(g, gd) { return false; }
    if let Some(s) = g.cues.shout_t {
        if t < s {   // (the clock went back: a checkpoint loaded)
            g.cues.shout_t = None;
            g.cues.heard_row = None;
            g.cues.heard_said.clear();
        }
    }
    g.cues.stats.shouts += 1.0;
    let within = |at: Option<f64>, secs: f64| at.is_some_and(|at| t - at < secs);
    if within(g.cues.heard_said.get(String::from(text)).copied(), g.k.cues.dup_secs) { return heard_done(g, t, text, "dup"); }
    if within(g.cues.shout_t, g.k.cues.overheard_gap) { return heard_done(g, t, text, "gap"); }
    let secs = g.k.cues.secs;
    if g.cues.rows.iter().filter(|r| r.t <= t && t - r.t < secs).count() as f64 >= g.k.cues.rows { return heard_done(g, t, text, "full"); }
    let p = g.ch[g.gd[gd].ch].pos;
    let mouth: V3 = [p[0], p[1] + 1.2, p[2]];
    let reach = g.k.digin.shout_m;
    let mut reached = false;
    for ear in cues_ears(g) {
        if flat(mouth, ear) > reach { continue; }
        let h = crate::weather::hear_path(g, mouth, ear, Some(false));
        if h.path_len < reach * h.carry { reached = true; break; }
    }
    if !reached { return heard_done(g, t, text, "far"); }
    if g.cues.heard_said.len() > 64 { g.cues.heard_said.clear(); }
    g.cues.heard_row = Some(CuesHeardRow { t, text: String::from(text) });
    g.cues.heard_said.set(String::from(text), t);
    g.cues.shout_t = Some(t);
    g.cues.stats.shouts_drawn += 1.0;
    heard_done(g, t, text, "")
}

/// A word over a man for the player's eyes alone: the bubble, no strip row, no class clock.   (cues.ts Cues.aside)
pub fn cues_aside(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool {
    let t = g.time;
    g.cues.bubbles.set(gd, Bubble { text: String::from(text), t, dur: crate::guards::bubble_secs(text), radio: false });
    true
}

/// Seconds until the strip would take a line of this class without dropping it for the class's gap (0 = now).   (cues.ts Cues.netWait)
pub fn cues_net_wait(g: &mut crate::game::Game, cls: f64) -> f64 {
    let c = cls as usize;
    let t = g.time;
    match sparse_get(&g.cues.class_t, c) {
        Some(at) if t >= at => m::max(0.0, at + g.k.cues.gaps.get(c).copied().unwrap_or(0.0) + 0.05 - t),
        _ => 0.0,
    }
}

/// The message ring has been put back as it was saved: its 'radio: ' rows are the strip's again.   (cues.ts Cues.adopt)
pub fn cues_adopt(g: &mut crate::game::Game, msgs: &[crate::cues::CuesAdoptMsgsItem]) {
    g.cues.heard_row = None;
    let radio: Vec<&crate::cues::CuesAdoptMsgsItem> = msgs.iter().filter(|m| m.text.starts_with("radio: ")).collect();
    let from = tail_from(radio.len(), g.k.cues.rows);
    g.cues.rows = radio[from..].iter().map(|m| CueRow { t: m.t, text: String::from(&m.text[7..]), cls: 0.0, rank: 1.0, who: String::new() }).collect();
}

/// What a man is saying: his line (Guard.bubble) or a cue of this file's, whichever is the later and still running. A copy: `=== by.bubble` of the TypeScript compares `.t` and `.text` here.   (cues.ts Cues.bubbleOf)
pub fn cues_bubble_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::cues::Bubble> {
    let b = g.gd[gd].bubble.as_ref().map(|b| Bubble { text: b.text.clone(), t: b.t, dur: b.dur, radio: b.radio });
    let Some(c) = g.cues.bubbles.get(gd).cloned() else { return b; };
    let t = g.time;
    if t < c.t || t - c.t > c.dur {
        g.cues.bubbles.delete(gd);
        return b;
    }
    match b {
        Some(b) if b.t >= c.t => Some(b),
        _ => Some(c),
    }
}

// ---------------------------------------------------------------- the words

/// A place as a line says it.   (cues.ts Cues.at)
pub fn cues_at(g: &mut crate::game::Game, p: V3) -> crate::cues::CuesAt {
    let n = crate::dialogue::place_name(g, p);
    let at = if n.starts_with("outside ") { n.clone() } else { alloc::format!("in {}", n) };
    CuesAt { the: n, at }
}
/// `at`'s two words as a cue's vars.
fn at_vars(a: &CuesAt) -> JsMap<String, String> {
    let mut v = JsMap::new();
    v.set(String::from("the"), a.the.clone());
    v.set(String::from("at"), a.at.clone());
    v
}

/// His row of `men`, made the first time he is asked after: the row itself.   (cues.ts Cues.man)
pub fn cues_man<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::cues::Man {
    if !g.cues.men.has(gd) {
        let row = Man { state: String::from(g.gd[gd].state.as_str()), alert_t: g.time, said_t: -100.0, again_t: -100.0, stood_t: -100.0, moved: None, from: String::new(), noise_t: -100.0, ep: -1.0, rally_at: f64::NAN, covered: false };
        g.cues.men.set(gd, row);
    }
    g.cues.men.get_mut(gd).unwrap()
}

/// One cue from this man, if he can speak and has not just spoken: a bubble, and (a net line) a row for the strip.   (cues.ts Cues.cue)
pub fn cues_cue(g: &mut crate::game::Game, gd: GuardId, kind: crate::cues::CueKind, vars: Option<&JsMap<String, String>>) -> bool {
    let t = g.time;
    let said_t = cues_man(g, gd).said_t;
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || t < g.gd[gd].dazzled_until || t - said_t < g.k.cues.man_secs || crate::runner::runner_mute(g, gd) { return false; }
    let own = g.gd[gd].bubble.as_ref().is_some_and(|b| t >= b.t && t - b.t < 1.5);   // he has just said something of his own: that was his word on it,
    if own && !matches!(kind, CueKind::Help | CueKind::FellBack | CueKind::SquadWaits | CueKind::SquadCopiesWaits) { return false; }   // but a shout to the men beside him, or a squad's word on its net, still goes out
    if cues_kind_busy(g, kind) { return false; }   // (the floor has just heard this from somebody)
    *kind_t_of(&mut g.cues.kind_t, kind) = Some(t);
    let cls = line_of(kind).cls;
    let say = said(g, kind);
    let n = g.cues.n;
    g.cues.n += 1.0;
    let callsign = g.gd[gd].callsign.clone();
    let pick = js::rem(crate::math_dice::hash_str(&callsign) + n, say.len() as f64) as usize;
    let text = fill(&say[pick], vars);
    cues_man(g, gd).said_t = t;
    g.cues.stats.cues += 1.0;
    let tally = kinds_of(&mut g.cues.kinds, kind);
    *tally = Some(tally.unwrap_or(0.0) + 1.0);
    if !own { g.cues.bubbles.set(gd, Bubble { text: text.clone(), t, dur: crate::guards::bubble_secs(&text), radio: cls > 0.0 }); }
    if cls == 0.0 { return true; }
    g.cues.stats.cue_net += 1.0;
    if cues_file(g, &net_row(&callsign, &text), cls, &callsign, file_kind(kind)) { g.cues.stats.cue_drawn += 1.0; }
    true
}

/// Has the floor just heard this kind from somebody?   (cues.ts Cues.kindBusy)
pub fn cues_kind_busy(g: &mut crate::game::Game, kind: crate::cues::CueKind) -> bool {
    let t = g.time;
    match *kind_t_of(&mut g.cues.kind_t, kind) {
        Some(kt) => t >= kt && t - kt < kind_gap(kind),
        None => false,
    }
}

/// Can he speak for his squad?   (cues.ts Cues.fit)
pub fn cues_fit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let t = g.time;
    g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].held.is_none() && t >= g.gd[gd].dazzled_until && t >= g.gd[gd].stagger_until && !crate::runner::runner_mute(g, gd)
}

/// Who says a squad's word: the man the note names, else one holding, else the first fit.   (cues.ts Cues.squadVoice)
pub fn cues_squad_voice(g: &mut crate::game::Game, name: &str, by: &str) -> Option<GuardId> {
    let mut men = Vec::new();
    for m in crate::enemysquads::squad_men(g, name) {
        if cues_fit(g, m) { men.push(m); }
    }
    if men.is_empty() { return None; }
    if let Some(&m) = men.iter().find(|&&m| g.gd[m].callsign == by) { return Some(m); }
    let holding = men.iter().copied().find(|&m| g.fight.defs.iter().any(|&d| g.tab.roomfight_def[d].gd == m && g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold));
    Some(holding.unwrap_or(men[0]))
}

// ---------------------------------------------------------------- once a frame, after everybody has moved

/// Read the fight's notes and each defender as he stands, and say what they decided. It reaches `file`, so it moves the clock laststand.ts waits on.   (cues.ts Cues.tick)
pub fn cues_tick(g: &mut crate::game::Game) {
    let t = g.time;
    if !g.fight.live { return; }
    if t < g.cues.last_t { cues_reset(g); }
    g.cues.last_t = t;
    // 1 · what the fight's modules noted this frame
    // (lastNote is a kept copy, found again by its three fields, last match first; not found = from the start, as after the log is emptied)
    let mut i = match &g.cues.last_note {
        Some(n) => g.fight.log.iter().rposition(|x| x.t == n.t && x.who == n.who && x.what == n.what).map_or(0, |at| at + 1),
        None => 0,
    };
    while i < g.fight.log.len() {
        g.cues.last_note = Some(g.fight.log[i].clone());
        if t - g.fight.log[i].t <= 0.5 {
            let (who, what) = (g.fight.log[i].who.clone(), g.fight.log[i].what.clone());
            cues_note(g, &who, &what);
        }
        i += 1;
    }
    // 2 · each defender as he now stands
    let mut di = 0;
    while di < g.fight.defs.len() {
        let d = g.fight.defs[di];
        di += 1;
        let gd = g.tab.roomfight_def[d].gd;
        cues_man(g, gd);
        let state = g.gd[gd].state;
        if state == crate::game::GuardState::Dead { continue; }
        if state == crate::game::GuardState::Alert && cues_man(g, gd).state != "alert" { cues_man(g, gd).alert_t = t; }
        let c = g.gd[gd].ch;
        let sight = t - g.gd[gd].last_seen_t;
        let still = m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]) < 0.3;
        let hold = g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold;
        let on_player = matches!(g.gd[gd].foe, Some(crate::game::Foe::Player));
        let faced_down = |g: &mut Game| -> Option<PlayerId> {
            let f = crate::seats::faced_man(g, gd);
            if g.pl[f].down { Some(f) } else { None }
        };
        if state == crate::game::GuardState::Patrol && cues_man(g, gd).state != "patrol" && hold && t - cues_man(g, gd).stood_t > 15.0 {
            if cues_cue(g, gd, CueKind::Calm, None) { cues_man(g, gd).stood_t = t; }
        } else if on_player && faced_down(g).is_some() && state == crate::game::GuardState::Alert && !cues_man(g, gd).covered && still && {
            let f = crate::seats::faced_man(g, gd);
            flat(g.ch[c].pos, g.ch[g.pl[f].ch].pos) < 3.5
        } {
            let f = crate::seats::faced_man(g, gd);
            let p = g.ch[g.pl[f].ch].pos;
            let at = cues_at(g, p);
            // (said, or a man beside him has just said it for both)
            if cues_cue(g, gd, CueKind::Cover, Some(&at_vars(&at))) || cues_kind_busy(g, CueKind::Cover) { cues_man(g, gd).covered = true; }
        } else if state == crate::game::GuardState::Alert && hold && still && sight > g.k.cues.still_secs && t - cues_man(g, gd).alert_t > g.k.cues.still_secs
            && t - g.gd[gd].witness.heard.last_shot_t > g.k.cues.still_quiet && t - cues_man(g, gd).again_t > g.k.cues.again_secs
            && !(on_player && faced_down(g).is_some()) && !crate::rescue::rescue_crew_hides(g, gd)
        {
            let p = g.ch[c].pos;
            let at = cues_at(g, p);
            if cues_cue(g, gd, CueKind::Still, Some(&at_vars(&at))) { cues_man(g, gd).again_t = t; }
        }
        // a loud thing outside his room, and he has not gone: the dice said stay (rallyAt drawn again), or nobody of his side is posted where it came from
        let (noise_at, noise_out, noise_t, d_rally) = { let row = &g.tab.roomfight_def[d]; (row.noise_at, row.noise_out, row.noise_t, row.rally_at) };
        if noise_at.is_some() && noise_out && noise_t > cues_man(g, gd).noise_t && cues_man(g, gd).ep < 0.0 && hold { cues_man(g, gd).ep = noise_t; }
        if cues_man(g, gd).ep >= 0.0 {
            let m_rally = cues_man(g, gd).rally_at;
            let drawn_again = !m_rally.is_nan() && !d_rally.is_nan() && d_rally > m_rally + 1.0;
            if !hold || t - noise_t > 6.0 {
                cues_man(g, gd).ep = -1.0;
            } else if !crate::enemysquads::squad_owns(g, gd) && !crate::pressure::press_owns(g, d) && sight > 3.0 && t - cues_man(g, gd).again_t > g.k.cues.again_secs && noise_at.is_some()
                && (drawn_again || (d_rally.is_nan() && t - cues_man(g, gd).ep >= g.k.cues.stay_secs && t - noise_t < 1.5))
            {
                let at = cues_at(g, noise_at.unwrap());
                if cues_cue(g, gd, if drawn_again { CueKind::StayOurs } else { CueKind::Stay }, Some(&at_vars(&at))) { cues_man(g, gd).again_t = t; }
                cues_man(g, gd).ep = -1.0;
            }
        }
        let now = String::from(g.gd[gd].state.as_str());
        let M = cues_man(g, gd);
        M.noise_t = noise_t;
        M.rally_at = d_rally;
        M.state = now;
    }
    if g.cues.squelch_due {
        g.cues.squelch_due = false;
        let squelch = g.k.cues.squelch;
        if squelch > 0.0 { crate::audio_iface::game_audio_like_play(g, "radio", None, Some(squelch), None); }
    }
}

/// `{sq}` and, where a squad heard another's call, `{ally}`.
fn squad_vars(sq: &str, ally: Option<&str>) -> JsMap<String, String> {
    let mut v = JsMap::new();
    v.set(String::from("sq"), String::from(sq));
    if let Some(a) = ally { v.set(String::from("ally"), String::from(a)); }
    v
}

/// One note of the fight's log, in the words of the man it is about.   (cues.ts Cues.note)
pub fn cues_note(g: &mut crate::game::Game, who: &str, what: &str) {
    let d = if who == "-" { None } else {
        g.fight.defs.iter().copied().find(|&d| { let gd = g.tab.roomfight_def[d].gd; g.gd[gd].callsign == who && g.gd[gd].state != crate::game::GuardState::Dead })
    };
    if let Some((name, said_word, ally)) = squads_note(what) {
        let Some(v) = cues_squad_voice(g, name, who) else { return; };
        if said_word == SquadSaid::Hears {
            let vars = squad_vars(name, Some(ally.unwrap_or("")));
            if !what.contains("), holds:") {
                cues_cue(g, v, if ally.is_some() { CueKind::SquadCopies } else { CueKind::SquadHears }, Some(&vars));
            } else {
                // (a drilled garrison holding on purpose is the tell that its word comes late: if its voice has just spoken, the next man of the squad says it)
                let mut voices = vec![v];
                for x in crate::enemysquads::squad_men(g, name) {
                    if x != v && cues_fit(g, x) { voices.push(x); }
                }
                for m in voices {
                    if cues_cue(g, m, if ally.is_some() { CueKind::SquadCopiesWaits } else { CueKind::SquadWaits }, Some(&vars)) { break; }
                }
            }
        } else {
            cues_cue(g, v, if said_word == SquadSaid::Stays { CueKind::SquadStays } else { CueKind::SquadQuiet }, Some(&squad_vars(name, None)));
        }
        return;
    }
    let Some(d) = d else { return; };
    let gd = g.tab.roomfight_def[d].gd;
    cues_man(g, gd);
    let c = g.gd[gd].ch;
    if what.starts_with("goes to the fight next door") {
        let p = g.tab.roomfight_def[d].noise_at.or(g.gd[gd].last_known).unwrap_or(g.ch[c].pos);
        let at = cues_at(g, p);
        cues_cue(g, gd, CueKind::Help, Some(&at_vars(&at)));
    } else if what.starts_with("holds where he ended up") {
        if g.time - cues_man(g, gd).stood_t > 15.0 {
            let p = g.ch[c].pos;
            let at = cues_at(g, p);
            if cues_cue(g, gd, CueKind::HuntOver, Some(&at_vars(&at))) { cues_man(g, gd).stood_t = g.time; }
        }
    } else if what.starts_with("falls back through") || what.starts_with("falls back to his squad") {
        cues_man(g, gd).moved = Some(ManMoved::Fallback);
        let station = g.tab.roomfight_def[d].station;
        let from = crate::dialogue::place_name(g, station);
        cues_man(g, gd).from = from.clone();
        cues_man(g, gd).said_t = -100.0;
        let mut vars = JsMap::new();
        vars.set(String::from("the"), from);
        cues_cue(g, gd, CueKind::FellBack, Some(&vars));
    } else if what.starts_with("takes cover") {
        cues_man(g, gd).moved = Some(ManMoved::Cover);
    } else if what.starts_with("plants (") {
        if cues_man(g, gd).moved == Some(ManMoved::Fallback) && !what.starts_with("plants (contact)") {
            let p = g.ch[c].pos;
            let h = cues_at(g, p);
            let floor = g.time - g.k.cues.man_secs;
            let row = cues_man(g, gd);
            row.said_t = m::min(row.said_t, floor);
            let kind = if h.the == row.from { CueKind::PinnedIn } else { CueKind::Planted };
            cues_cue(g, gd, kind, Some(&at_vars(&h)));
        }
        cues_man(g, gd).moved = None;
    } else if what.starts_with("surprised ") {
        cues_cue(g, gd, CueKind::Surprised, None);
    } else if what.starts_with("flinches:") {
        cues_cue(g, gd, CueKind::Flinch, None);
    } else if what.contains("stays to keep his room") {
        let noise_at = g.tab.roomfight_def[d].noise_at;
        if g.time - cues_man(g, gd).again_t > g.k.cues.again_secs {
            if let Some(p) = noise_at {
                let at = cues_at(g, p);
                if cues_cue(g, gd, CueKind::StayOurs, Some(&at_vars(&at))) { cues_man(g, gd).again_t = g.time; }
            }
        }
    }
}

/// The panel's line.   (cues.ts Cues.summary)
pub fn cues_summary(g: &mut crate::game::Game) -> String {
    let S = &g.cues.stats;
    let (drawn, of) = (S.heard_drawn + S.cue_drawn, S.heard + S.cue_net);
    let held = if js::truthy(S.dup + S.gap + S.full) { alloc::format!(" · held back: {} said already, {} too soon, {} outranked", S.dup, S.gap, S.full) } else { String::new() };
    alloc::format!("{} of {} net lines drawn · {} new lines{}", drawn, of, S.cues, held)
}

/// The strip's rows this frame as 'radio: …' rows, and at most one 'heard: …' row in a place no other row wants, oldest first.   (cues.ts stripRows)
pub fn strip_rows(g: &mut crate::game::Game) -> Vec<crate::cues::StripRowsItem> {
    let (t, secs, most) = (g.time, g.k.cues.secs, g.k.cues.rows);
    let live: Vec<&CueRow> = g.cues.rows.iter().filter(|r| r.t <= t && t - r.t < secs).collect();
    let from = tail_from(live.len(), most);
    let mut out: Vec<StripRowsItem> = live[from..].iter().map(|r| StripRowsItem { text: alloc::format!("radio: {}", r.text), t: r.t }).collect();
    if let Some(h) = &g.cues.heard_row {
        if (out.len() as f64) < most && h.t <= t && t - h.t < secs {
            out.push(StripRowsItem { text: alloc::format!("heard: {}", h.text), t: h.t });
            js::sort_by(&mut out, |a, b| a.t - b.t);
        }
    }
    out
}

/// Host work: the debugger's panel keeps three rows in the browser's storage. No game state moves.   (cues.ts saveCues)
pub fn save_cues(g: &mut crate::game::Game) { todo!("host work: the panel's localStorage, kept in the TypeScript host") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squad_word_is_the_regexp() {
        for (s, want) in [("Bravo: copy", true), ("Ab: x", true), ("A: x", false), ("Tango 4: x", false), ("bravo: x", false), ("Abcdefghijkl: x", true), ("Abcdefghijklm: x", false), ("Bravo:x", false), ("Bravo: ", true), ("Bravo:", false), ("", false)] {
            assert_eq!(squad_word(s), want, "{}", s);
        }
    }

    #[test]
    fn fill_replaces_words_only() {
        let mut v = JsMap::new();
        v.set(String::from("at"), String::from("in the mess"));
        assert_eq!(fill("shots {at} — {nope}{ x} {} {at", Some(&v)), "shots in the mess — { x} {} {at");
        assert_eq!(fill("the door—!", None), "the door—!");
    }

    #[test]
    fn squad_notes_are_the_regexp() {
        assert!(squads_note("[squads] Bravo hears of them (Alpha's call), holds: 4 s") == Some(("Bravo", SquadSaid::Hears, Some("Alpha"))));
        assert!(squads_note("[squads] Bravo hears of them") == Some(("Bravo", SquadSaid::Hears, None)));
        assert!(squads_note("[squads] Bravo hears of them (Al pha's call)") == Some(("Bravo", SquadSaid::Hears, None)));
        assert!(squads_note("[squads] Bravo hears of them ('s call)") == Some(("Bravo", SquadSaid::Hears, None)));
        assert!(squads_note("[squads] Bravo hears of them (''s call)") == Some(("Bravo", SquadSaid::Hears, Some("'"))));
        assert!(squads_note("[squads] Bravo stays on its doors") == Some(("Bravo", SquadSaid::Stays, None)));
        assert!(squads_note("[squads] Bravo quiet: 3") == Some(("Bravo", SquadSaid::Quiet, None)));
        assert!(squads_note("[squads] Bravo  quiet:").is_none());
        assert!(squads_note("[squads]  Bravo quiet:").is_none());
        assert!(squads_note("[squads] Bravo sends a pair").is_none());
        assert!(squads_note("holds where he ended up").is_none());
    }

    #[test]
    fn fixed2_is_the_languages() {
        // (bun: +t.toFixed(2))
        for (t, want) in [(0.0, 0.0), (0.125, 0.13), (0.375, 0.38), (0.625, 0.63), (0.875, 0.88), (1.125, 1.13), (2.5, 2.5), (1.005, 1.0), (1.015, 1.01), (1.025, 1.02), (8.345, 8.35), (100.125, 100.13), (0.016666666666666666, 0.02), (3.3333333333333335, 3.33), (12.995, 12.99), (12.995, 12.99), (1234.565, 1234.57), (0.005, 0.01), (0.015, 0.01), (7.875, 7.88)] {
            assert_eq!(fixed2(t).to_bits(), f64::to_bits(want), "{}", t);
        }
    }

    #[test]
    fn cue_class_is_the_regexps() {
        for (s, want) in [
            ("moving to the fight — {the}", 1.0),
            ("they're hit {at} — moving to them", 1.0),
            ("lost them — holding {the}", 2.0),
            ("lost them — staying {at}", 2.0),
            ("quiet now — staying put", 2.0),
            ("standing down — holding here", 2.0),
            ("one of them down {at} — covering him", 2.0),
            ("{sq}: gone quiet here — still holding", 2.0),
            ("they're hit {at} — staying on my door", 3.0),
            ("shots {at} — staying on my door", 3.0),
            ("shots {at} — holding my room", 3.0),
            ("heard that, {the} — holding my room", 3.0),
            ("{sq}: still on the doors — nobody goes", 3.0),
            ("{sq}: heard that — stand by", 4.0),
            ("{sq}: copy, {ally} — stand by", 4.0),
            ("{sq}: hold — let them come in. Wait for my word!", 3.0),
            ("{sq}: copy, {ally} — hold, let them come in. Wait for my word!", 3.0),
            ("{the} is lost — falling back", 5.0),
            ("falling back — {the} is lost", 5.0),
            ("fell back — set {at}", 5.0),
            ("can't get out — fell back to cover {at}", 5.0),
            ("they're still out there — holding {the}", 6.0),
            ("still out there — staying on this door", 6.0),
            ("behind—!", 4.0),
            ("there—!", 4.0),
            ("the door—!", 4.0),
            ("set", 1.0),
            ("SET up the door", 1.0),
            ("settle down", 4.0),
            ("set-up", 1.0),
            ("set_up", 4.0),
            ("set up", 1.0),
            ("reset the door", 4.0),
            ("Moving!", 1.0),
            ("moving", 4.0),
            ("moving! now", 1.0),
            ("not moving!", 4.0),
            ("I'M COMING TO YOU", 5.0),
            ("i'm coming to you, hold", 5.0),
            ("Fall back!", 5.0),
            ("falling back", 5.0),
            ("fallback", 4.0),
            ("fall  back", 4.0),
            ("GO GO GO", 1.0),
            ("go — go", 1.0),
            ("go - go", 4.0),
            ("GO, GO", 1.0),
            ("locked down. box him in", 1.0),
            ("locked downx box him in", 4.0),
            ("not answering — going back", 2.0),
            ("not answering - going back", 4.0),
            ("moved on — after them", 1.0),
            ("coming in — i see", 1.0),
            ("covering — move", 1.0),
            ("still out there", 6.0),
            ("They Are Still Out There", 6.0),
            ("hold where you are", 2.0),
            ("on my own", 3.0),
            ("I've got this door", 3.0),
            ("nothing special", 4.0),
            ("", 4.0),
            ("contact left!", 4.0),
            ("Bravo: copy, Alpha — stand by", 4.0),
            ("is lost", 5.0),
            ("with you", 1.0),
            ("hit the others", 1.0),
            ("é set", 4.0),
            ("ſet", 4.0),
            ("ſet up", 4.0),
            ("ıs lost", 4.0),
            ("K", 4.0),
            ("pushing", 1.0),
            ("two on this room", 1.0),
            ("the mess is lost — getting ready", 5.0),
        ] {
            assert_eq!(cue_class(s), want, "{}", s);
        }
    }

    #[test]
    fn every_line_as_said_has_its_place_for_place_count() {
        let mut seen = 0;
        for l in LINES {
            seen += 1;
            if let Some(n) = l.now { assert_eq!(n.len(), l.say.len()); }
            if let Some(n) = l.guide { assert_eq!(n.len(), l.say.len()); }
        }
        assert_eq!(seen, 18);
    }

    #[test]
    fn the_tables_are_the_skeletons() {
        let t = crate::cues::lines();
        assert_eq!(t.len(), LINES.len());
        let strs = |v: &[&str]| v.iter().map(|s| String::from(*s)).collect::<Vec<String>>();
        for l in LINES {
            let row = t.get(String::from(l.kind.as_str())).unwrap();
            assert_eq!(row.cls, l.cls);
            assert_eq!(row.say, strs(l.say));
            assert_eq!(row.now, l.now.map(strs));
            assert_eq!(row.guide, l.guide.map(strs));
        }
        let want = crate::cues::kind_gap();
        for (k, w) in [(CueKind::Stay, want.stay), (CueKind::Calm, want.calm), (CueKind::Cover, want.cover), (CueKind::Still, want.still), (CueKind::StayOurs, want.stay_ours), (CueKind::HuntOver, want.hunt_over), (CueKind::Help, want.help), (CueKind::Flinch, want.flinch)] {
            assert_eq!(kind_gap(k), w.unwrap_or(0.0), "{}", k.as_str());
        }
        let c = crate::cues::classes();
        assert_eq!(c.len(), CLASSES.len());
        for (row, (pats, cls)) in c.iter().zip(CLASSES) {
            assert_eq!(row.3, *cls);
            for p in row.0.iter().chain(row.1).chain(row.2) {
                assert!(pats.iter().any(|q| match q { Pat::In(s) | Pat::Head(s) | Pat::HeadWord(s) => s == p }), "{}", p);
            }
            assert_eq!(pats.len(), row.0.len() + row.1.len() + row.2.len());
        }
    }

    #[test]
    fn the_readers_are_the_typescripts() {
        // (bun: the TypeScript's own Cues, adopt, stripRows, summary and cueLines with WORDS.on, WORDS.guide and CUES.overheard up)
        let mut game = Game::blank();
        let g = &mut game;
        g.time = 20.0;
        let msg = |text: &str, t: f64| CuesAdoptMsgsItem { text: String::from(text), t };
        cues_adopt(g, &[msg("radio: a", 1.0), msg("Sam: x", 2.0), msg("radio: b", 3.0), msg("radio: c", 15.0), msg("radio: d", 16.0)]);
        let rows: Vec<(f64, &str, f64, f64)> = g.cues.rows.iter().map(|r| (r.t, r.text.as_str(), r.cls, r.rank)).collect();
        assert_eq!(rows, [(3.0, "b", 0.0, 1.0), (15.0, "c", 0.0, 1.0), (16.0, "d", 0.0, 1.0)]);
        fn strip(g: &mut Game) -> Vec<(String, f64)> { strip_rows(g).into_iter().map(|r| (r.text, r.t)).collect() }
        fn want(v: &[(&str, f64)]) -> Vec<(String, f64)> { v.iter().map(|r| (String::from(r.0), r.1)).collect() }
        assert_eq!(strip(g), want(&[("radio: c", 15.0), ("radio: d", 16.0)]));
        let row = |t: f64, text: &str| CueRow { t, text: String::from(text), cls: 1.0, rank: 3.0, who: String::new() };
        g.cues.rows = vec![row(14.0, "p"), row(18.0, "q"), row(21.0, "future")];
        g.cues.heard_row = Some(CuesHeardRow { t: 16.0, text: String::from("find them!") });
        assert_eq!(strip(g), want(&[("radio: p", 14.0), ("heard: find them!", 16.0), ("radio: q", 18.0)]));
        g.cues.heard_row = Some(CuesHeardRow { t: 11.0, text: String::from("old!") });
        assert_eq!(strip(g), want(&[("radio: p", 14.0), ("radio: q", 18.0)]));
        assert_eq!(cues_summary(g), "0 of 0 net lines drawn · 0 new lines");
        let S = &mut g.cues.stats;
        (S.heard, S.heard_drawn, S.cue_net, S.cue_drawn, S.cues, S.dup, S.gap, S.full) = (41.0, 10.0, 3.0, 2.0, 9.0, 1.0, 2.0, 3.0);
        assert_eq!(cues_summary(g), "12 of 44 net lines drawn · 9 new lines · held back: 1 said already, 2 too soon, 3 outranked");
        let lines = cue_lines(g);
        assert_eq!(lines.len(), 26);
        assert_eq!((lines[0].kind, lines[0].cls, lines[0].text.as_str()), (CueKind::Help, 1.0, "moving to the fight — {the}"));
        let last = &lines[25];
        assert_eq!((last.kind, last.cls, last.text.as_str()), (CueKind::Flinch, 0.0, "the door—!"));
        let w = lines.iter().find(|l| l.kind == CueKind::SquadWaits).unwrap();
        assert_eq!((w.cls, w.text.as_str()), (3.0, "{sq}: hold — let them come in. wait for my call!"));
    }

    #[test]
    fn a_kinds_clock_and_tally_are_its_own() {
        let mut v = CuesKindT::zeroed();
        *kind_t_of(&mut v, CueKind::StayOurs) = Some(1.0);
        assert_eq!(v.stay, None);
        assert_eq!(v.stay_ours, Some(1.0));
        let mut k = CuesKinds::zeroed();
        *kinds_of(&mut k, CueKind::Surprised) = Some(2.0);
        assert_eq!(k.surprised, Some(2.0));
        assert_eq!(k.squad_quiet, None);
        assert_eq!((tail_from(5, 3.0), tail_from(2, 3.0), tail_from(5, 0.0)), (2, 0, 0));
    }

    #[test]
    fn the_strip_files_as_the_typescript_does() {
        // (rows generated by the TypeScript's own Cues class under bun)
        let mut game = Game::blank();
        let g = &mut game;
        for (t, text, cls, who, kind, want, why, rows, wait1) in [
            (0.05, "Tango 1: moving to the fight", 1.0, "Tango 1", "heard", true, "", vec!["Tango 1: moving to the fight"] as Vec<&str>, 1.25),
            (4.15, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", true, "", vec!["Tango 1: moving to the fight", "Bravo: gone quiet here"] as Vec<&str>, 0.0),
            (4.2, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", false, "dup", vec!["Tango 1: moving to the fight", "Bravo: gone quiet here"] as Vec<&str>, 0.0),
            (6.8, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", false, "dup", vec!["Tango 1: moving to the fight", "Bravo: gone quiet here"] as Vec<&str>, 0.0),
            (9.4, "Tango 4: the mess is lost", 5.0, "Tango 4", "heard", true, "", vec!["Bravo: gone quiet here", "Tango 4: the mess is lost"] as Vec<&str>, 0.0),
            (9.65, "control to all", 0.0, "", "raw", true, "", vec!["Bravo: gone quiet here", "Tango 4: the mess is lost", "control to all"] as Vec<&str>, 0.0),
            (10.35, "Tango 5: still out there", 6.0, "Tango 5", "heard", false, "full", vec!["Bravo: gone quiet here", "Tango 4: the mess is lost", "control to all"] as Vec<&str>, 0.0),
            (12.95, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", true, "", vec!["Tango 4: the mess is lost", "control to all", "Tango 2: moving to the fight"] as Vec<&str>, 1.25),
            (15.55, "Charlie: pushing", 1.0, "Tango 9", "heard", true, "", vec!["control to all", "Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 1.25),
            (15.6, "Sam: contact left!", 1.0, "Sam", "call", false, "gap", vec!["control to all", "Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 1.200000000000001),
            (15.6, "Tango 6: lost them", 2.0, "Tango 6", "heard", false, "full", vec!["control to all", "Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 1.200000000000001),
            (16.9, "Charlie: pushing", 1.0, "Tango 9", "heard", false, "dup", vec!["control to all", "Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 0.0),
            (16.95, "Tango 5: still out there", 6.0, "Tango 5", "heard", false, "full", vec!["control to all", "Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 0.0),
            (17.65, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["control to all", "Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 0.0),
            (17.7, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Tango 2: moving to the fight", "Charlie: pushing"] as Vec<&str>, 0.0),
            (17.75, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", true, "", vec!["Tango 2: moving to the fight", "Charlie: pushing", "Bravo: gone quiet here"] as Vec<&str>, 0.0),
            (18.0, "Tango 8: on my way", 1.0, "Tango 8", "heard", true, "", vec!["Charlie: pushing", "Bravo: gone quiet here", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (18.0, "Tango 2: quiet now", 2.0, "Tango 2", "heard", false, "gap", vec!["Charlie: pushing", "Bravo: gone quiet here", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (18.05, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Charlie: pushing", "Bravo: gone quiet here", "Tango 8: on my way"] as Vec<&str>, 1.1999999999999993),
            (18.3, "Sam: contact left!", 1.0, "Sam", "call", true, "", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 1.25),
            (18.55, "Tango 7: moving around", 1.0, "Tango 7", "heard", false, "gap", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 1.0),
            (18.8, "Tango 7: moving around", 1.0, "Tango 7", "heard", false, "gap", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 0.75),
            (19.5, "Tango 3: holding my post", 3.0, "Tango 3", "heard", false, "full", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 0.05000000000000071),
            (20.8, "Alpha: on the doors", 3.0, "Tango 1", "heard", false, "full", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 0.0),
            (23.4, "Bravo: copy, Alpha — stand by", 4.0, "Tango 3", "heard", false, "full", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 0.0),
            (24.1, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", false, "dup", vec!["Bravo: gone quiet here", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 0.0),
            (28.2, "Bravo: copy, Alpha — stand by", 4.0, "Tango 3", "heard", true, "", vec!["Bravo: copy, Alpha — stand by"] as Vec<&str>, 0.0),
            (30.8, "Tango 6: lost them", 2.0, "Tango 6", "heard", true, "", vec!["Bravo: copy, Alpha — stand by", "Tango 6: lost them"] as Vec<&str>, 0.0),
            (31.05, "Tango 3: holding my post", 3.0, "Tango 3", "heard", true, "", vec!["Bravo: copy, Alpha — stand by", "Tango 6: lost them", "Tango 3: holding my post"] as Vec<&str>, 0.0),
            (35.15, "Tango 8: on my way", 1.0, "Tango 8", "heard", true, "", vec!["Tango 6: lost them", "Tango 3: holding my post", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (35.4, "Tango 7: moving around", 1.0, "Tango 7", "heard", false, "gap", vec!["Tango 6: lost them", "Tango 3: holding my post", "Tango 8: on my way"] as Vec<&str>, 1.0),
            (35.4, "Sam: contact left!", 1.0, "Sam", "call", true, "", vec!["Tango 6: lost them", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 1.25),
            (35.4, "Tango 1: moving to the fight", 1.0, "Tango 1", "heard", false, "gap", vec!["Tango 6: lost them", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 1.25),
            (36.1, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "gap", vec!["Tango 6: lost them", "Tango 8: on my way", "Sam: contact left!"] as Vec<&str>, 0.5499999999999972),
            (40.2, "Tango 1: moving to the fight", 1.0, "Tango 1", "heard", true, "", vec!["Tango 8: on my way", "Sam: contact left!", "Tango 1: moving to the fight"] as Vec<&str>, 1.25),
            (40.9, "Charlie: pushing", 1.0, "Tango 9", "heard", true, "", vec!["Sam: contact left!", "Tango 1: moving to the fight", "Charlie: pushing"] as Vec<&str>, 1.25),
            (45.0, "Sam: contact left!", 1.0, "Sam", "call", false, "dup", vec!["Tango 1: moving to the fight", "Charlie: pushing"] as Vec<&str>, 0.0),
            (45.7, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Tango 1: moving to the fight", "Charlie: pushing"] as Vec<&str>, 0.0),
            (45.95, "Tango 6: lost them", 2.0, "Tango 6", "heard", true, "", vec!["Tango 1: moving to the fight", "Charlie: pushing", "Tango 6: lost them"] as Vec<&str>, 0.0),
            (48.55, "Tango 1: moving to the fight", 1.0, "Tango 1", "heard", false, "dup", vec!["Charlie: pushing", "Tango 6: lost them"] as Vec<&str>, 0.0),
            (49.25, "Tango 6: lost them", 2.0, "Tango 6", "heard", false, "dup", vec!["Tango 6: lost them"] as Vec<&str>, 0.0),
            (49.3, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", true, "", vec!["Tango 6: lost them", "Bravo: gone quiet here"] as Vec<&str>, 0.0),
            (50.6, "Bravo: copy, Alpha — stand by", 4.0, "Tango 3", "heard", true, "", vec!["Tango 6: lost them", "Bravo: gone quiet here", "Bravo: copy, Alpha — stand by"] as Vec<&str>, 0.0),
            (50.65, "Tango 1: moving to the fight", 1.0, "Tango 1", "heard", true, "", vec!["Tango 6: lost them", "Bravo: gone quiet here", "Tango 1: moving to the fight"] as Vec<&str>, 1.25),
            (50.9, "Tango 4: the mess is lost", 5.0, "Tango 4", "heard", false, "full", vec!["Tango 6: lost them", "Bravo: gone quiet here", "Tango 1: moving to the fight"] as Vec<&str>, 1.0),
            (52.2, "Tango 1: moving to the fight", 1.0, "Tango 1", "heard", false, "dup", vec!["Tango 6: lost them", "Bravo: gone quiet here", "Tango 1: moving to the fight"] as Vec<&str>, 0.0),
            (52.25, "Sam: contact left!", 1.0, "Sam", "call", true, "", vec!["Bravo: gone quiet here", "Tango 1: moving to the fight", "Sam: contact left!"] as Vec<&str>, 1.25),
            (52.25, "Charlie: pushing", 1.0, "Tango 9", "heard", false, "gap", vec!["Bravo: gone quiet here", "Tango 1: moving to the fight", "Sam: contact left!"] as Vec<&str>, 1.25),
            (52.5, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Bravo: gone quiet here", "Tango 1: moving to the fight", "Sam: contact left!"] as Vec<&str>, 1.0),
            (53.8, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Bravo: gone quiet here", "Tango 1: moving to the fight", "Sam: contact left!"] as Vec<&str>, 0.0),
            (57.9, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", false, "dup", vec!["Tango 1: moving to the fight", "Sam: contact left!"] as Vec<&str>, 0.0),
            (57.9, "Tango 2: quiet now", 2.0, "Tango 2", "heard", true, "", vec!["Tango 1: moving to the fight", "Sam: contact left!", "Tango 2: quiet now"] as Vec<&str>, 0.0),
            (57.9, "Tango 2: quiet now", 2.0, "Tango 2", "heard", false, "dup", vec!["Tango 1: moving to the fight", "Sam: contact left!", "Tango 2: quiet now"] as Vec<&str>, 0.0),
            (58.15, "Tango 8: on my way", 1.0, "Tango 8", "heard", true, "", vec!["Sam: contact left!", "Tango 2: quiet now", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (58.15, "Tango 2: quiet now", 2.0, "Tango 2", "heard", false, "dup", vec!["Sam: contact left!", "Tango 2: quiet now", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (58.2, "Charlie: pushing", 1.0, "Tango 9", "heard", true, "", vec!["Tango 2: quiet now", "Tango 8: on my way", "Charlie: pushing"] as Vec<&str>, 1.25),
            (58.25, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", false, "dup", vec!["Tango 2: quiet now", "Tango 8: on my way", "Charlie: pushing"] as Vec<&str>, 1.2000000000000028),
            (58.95, "Charlie: pushing", 1.0, "Tango 9", "heard", false, "dup", vec!["Tango 2: quiet now", "Tango 8: on my way", "Charlie: pushing"] as Vec<&str>, 0.5),
            (59.65, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Tango 2: quiet now", "Tango 8: on my way", "Charlie: pushing"] as Vec<&str>, 0.0),
            (63.75, "Sam: contact left!", 1.0, "Sam", "call", true, "", vec!["Tango 2: quiet now", "Charlie: pushing", "Sam: contact left!"] as Vec<&str>, 1.25),
            (67.85, "Charlie: pushing", 1.0, "Tango 9", "heard", false, "dup", vec!["Sam: contact left!"] as Vec<&str>, 0.0),
            (70.45, "Charlie: pushing", 1.0, "Tango 9", "heard", true, "", vec!["Sam: contact left!", "Charlie: pushing"] as Vec<&str>, 1.25),
            (74.55, "Tango 6: lost them", 2.0, "Tango 6", "heard", true, "", vec!["Charlie: pushing", "Tango 6: lost them"] as Vec<&str>, 0.0),
            (75.85, "control to all", 0.0, "", "raw", true, "", vec!["Charlie: pushing", "Tango 6: lost them", "control to all"] as Vec<&str>, 0.0),
            (79.95, "Alpha: on the doors", 3.0, "Tango 1", "heard", true, "", vec!["Tango 6: lost them", "control to all", "Alpha: on the doors"] as Vec<&str>, 0.0),
            (80.0, "Alpha: on the doors", 3.0, "Tango 1", "heard", false, "dup", vec!["Tango 6: lost them", "control to all", "Alpha: on the doors"] as Vec<&str>, 0.0),
            (80.7, "Tango 6: lost them", 2.0, "Tango 6", "heard", false, "dup", vec!["Tango 6: lost them", "control to all", "Alpha: on the doors"] as Vec<&str>, 0.0),
            (83.3, "Alpha: on the doors", 3.0, "Tango 1", "heard", false, "dup", vec!["control to all", "Alpha: on the doors"] as Vec<&str>, 0.0),
            (83.3, "Charlie: pushing", 1.0, "Tango 9", "heard", true, "", vec!["control to all", "Alpha: on the doors", "Charlie: pushing"] as Vec<&str>, 1.25),
            (83.55, "Tango 3: holding my post", 3.0, "Tango 3", "heard", false, "full", vec!["control to all", "Alpha: on the doors", "Charlie: pushing"] as Vec<&str>, 1.0),
            (83.55, "Sam: contact left!", 1.0, "Sam", "call", false, "gap", vec!["control to all", "Alpha: on the doors", "Charlie: pushing"] as Vec<&str>, 1.0),
            (83.55, "Tango 8: on my way", 1.0, "Tango 8", "heard", false, "gap", vec!["control to all", "Alpha: on the doors", "Charlie: pushing"] as Vec<&str>, 1.0),
            (84.25, "Tango 6: lost them", 2.0, "Tango 6", "heard", false, "dup", vec!["Alpha: on the doors", "Charlie: pushing"] as Vec<&str>, 0.29999999999999716),
            (86.85, "Tango 8: on my way", 1.0, "Tango 8", "heard", true, "", vec!["Alpha: on the doors", "Charlie: pushing", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (89.45, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", true, "", vec!["Charlie: pushing", "Tango 8: on my way", "Tango 2: moving to the fight"] as Vec<&str>, 1.25),
            (89.5, "Tango 2: moving to the fight", 1.0, "Tango 2", "heard", false, "dup", vec!["Charlie: pushing", "Tango 8: on my way", "Tango 2: moving to the fight"] as Vec<&str>, 1.2000000000000028),
            (93.6, "Tango 8: on my way", 1.0, "Tango 8", "heard", false, "dup", vec!["Tango 8: on my way", "Tango 2: moving to the fight"] as Vec<&str>, 0.0),
            (93.85, "Tango 5: still out there", 6.0, "Tango 5", "heard", true, "", vec!["Tango 8: on my way", "Tango 2: moving to the fight", "Tango 5: still out there"] as Vec<&str>, 0.0),
            (93.85, "Tango 6: lost them", 2.0, "Tango 6", "heard", true, "", vec!["Tango 8: on my way", "Tango 2: moving to the fight", "Tango 6: lost them"] as Vec<&str>, 0.0),
            (95.15, "control to all", 0.0, "", "raw", true, "", vec!["Tango 2: moving to the fight", "Tango 6: lost them", "control to all"] as Vec<&str>, 0.0),
            (95.4, "Tango 4: the mess is lost", 5.0, "Tango 4", "heard", false, "full", vec!["Tango 2: moving to the fight", "Tango 6: lost them", "control to all"] as Vec<&str>, 0.0),
            (95.45, "Tango 2: quiet now", 2.0, "Tango 2", "heard", false, "gap", vec!["Tango 2: moving to the fight", "Tango 6: lost them", "control to all"] as Vec<&str>, 0.0),
            (99.55, "Bravo: gone quiet here", 2.0, "Tango 3", "heard", true, "", vec!["Tango 6: lost them", "control to all", "Bravo: gone quiet here"] as Vec<&str>, 0.0),
            (103.65, "Bravo: copy, Alpha — stand by", 4.0, "Tango 3", "heard", true, "", vec!["Bravo: gone quiet here", "Bravo: copy, Alpha — stand by"] as Vec<&str>, 0.0),
            (103.9, "Tango 7: moving around", 1.0, "Tango 7", "heard", true, "", vec!["Bravo: gone quiet here", "Bravo: copy, Alpha — stand by", "Tango 7: moving around"] as Vec<&str>, 1.25),
            (105.2, "Charlie: pushing", 1.0, "Tango 9", "heard", true, "", vec!["Bravo: gone quiet here", "Tango 7: moving around", "Charlie: pushing"] as Vec<&str>, 1.25),
            (109.3, "Tango 7: moving around", 1.0, "Tango 7", "heard", false, "dup", vec!["Tango 7: moving around", "Charlie: pushing"] as Vec<&str>, 0.0),
            (109.3, "Tango 8: on my way", 1.0, "Tango 8", "heard", true, "", vec!["Tango 7: moving around", "Charlie: pushing", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (109.3, "Bravo: copy, Alpha — stand by", 4.0, "Tango 3", "heard", false, "dup", vec!["Tango 7: moving around", "Charlie: pushing", "Tango 8: on my way"] as Vec<&str>, 1.25),
            (110.0, "control to all", 0.0, "", "raw", true, "", vec!["Charlie: pushing", "Tango 8: on my way", "control to all"] as Vec<&str>, 0.5499999999999972),
        ] {
            g.time = t;
            let kind = match kind { "raw" => CuesFileKind::Raw, "call" => CuesFileKind::Call, _ => CuesFileKind::Heard };
            assert_eq!(cues_file(g, text, cls, who, kind), want, "{} at {}", text, t);
            assert_eq!(g.cues.log.last().unwrap().why, why, "{} at {}", text, t);
            let got: Vec<&str> = g.cues.rows.iter().map(|r| r.text.as_str()).collect();
            assert_eq!(got, rows, "{} at {}", text, t);
            assert_eq!(cues_net_wait(g, 1.0).to_bits(), f64::to_bits(wait1), "{} at {}", text, t);
        }
    }
}
