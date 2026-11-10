pub use ::game_types::cues::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::roomfight::{Did, News, Word};
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

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

/// The ranking of the lines the defenders already had: a radio line's class by its words, first match; no row = 4.
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
/// The new lines, in the written order.
static LINES: &[Line] = &[
    Line { kind: CueKind::HuntOver, cls: 2.0, say: &["lost them — holding {the}", "lost them — staying {at}"], now: None, guide: None },
    Line { kind: CueKind::Calm, cls: 2.0, say: &["quiet now — staying put", "standing down — holding here"], now: None, guide: None },
    Line { kind: CueKind::Cover, cls: 2.0, say: &["one of them down {at} — covering him"], now: None, guide: None },
    Line { kind: CueKind::SquadQuiet, cls: 2.0, say: &["{sq}: gone quiet here — still holding"], now: None, guide: None },
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

/// s before the same kind of cue is said again by anybody on the floor; absent = no wait.
fn gap_of(k: CueKind) -> f64 {
    match k {
        CueKind::Cover => 15.0,
        CueKind::HuntOver => 4.0,
        CueKind::Calm => 4.0,
        CueKind::Still => 6.0,
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
/// `/^[A-Z][a-z]{1,11}: /`: a row under a squad's name, not a man's callsign.
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
/// A read of the sparse `classT` / `squadT`: past the end or a hole is None.
fn sparse_get(v: &[Option<f64>], i: usize) -> Option<f64> {
    v.get(i).copied().flatten()
}
fn sparse_set(v: &mut Vec<Option<f64>>, i: usize, x: f64) {
    if v.len() <= i { v.resize(i + 1, None); }
    v[i] = Some(x);
}
/// `kindT[kind]`, the place of a kind's clock.
fn kind_t_of(v: &mut CuesKindT, k: CueKind) -> &mut Option<f64> {
    match k {
        CueKind::Calm => &mut v.calm,
        CueKind::Cover => &mut v.cover,
        CueKind::Flinch => &mut v.flinch,
        CueKind::Still => &mut v.still,
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
        CueKind::Calm => &mut v.calm,
        CueKind::Cover => &mut v.cover,
        CueKind::Flinch => &mut v.flinch,
        CueKind::Still => &mut v.still,
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
        CueKind::Calm => CuesFileKind::Calm,
        CueKind::Cover => CuesFileKind::Cover,
        CueKind::Flinch => CuesFileKind::Flinch,
        CueKind::Still => CuesFileKind::Still,
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

// ---------------------------------------------------------------- the functions of the module

/// A radio line's class by its words, first match; none = the net knowing something.
pub fn cue_class(text: &str) -> f64 {
    let b = text.as_bytes();
    for (pats, c) in CLASSES {
        if pats.iter().any(|p| pat_hits(p, b)) { return *c; }
    }
    4.0
}

/// A kind's lines as they are said now.
fn said(g: &mut crate::game::Game, k: crate::cues::CueKind) -> Vec<String> {
    let l = line_of(k);
    l.guide.or(l.now).unwrap_or(l.say).iter().map(|s| String::from(*s)).collect()
}

/// A line is shouted when its writer put an exclamation mark in it.
fn is_shout(text: &str) -> bool {
    text.contains('!')
}

/// The tallies as a new encounter has them.
fn new_stats() -> crate::cues::CuesStats {
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

/// The distance on the floor.
fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A new Cues: the field initialisers, nothing more.
pub fn cues_new(g: &crate::game::Game) -> crate::cues::Cues {
    Cues::blank()
}

/// The encounter is another one (a restart, a checkpoint loaded: the clock may have gone back).
pub fn cues_reset(g: &mut crate::game::Game) {
    g.cues.heard_row = None;
    g.cues.heard_said.clear();
    g.cues.shout_t = None;
    g.cues.rows.clear();
    g.cues.bubbles = JsMap::new();
    g.cues.men = JsMap::new();
    g.cues.said.clear();
    g.cues.class_t.clear();
    g.cues.squad_t.clear();
    g.cues.kind_t = CuesKindT::default();
    g.cues.squelch_due = false;
    g.cues.stats = new_stats();
    g.cues.kinds = CuesKinds::default();
    g.cues.log.clear();
}

/// Drops the rows no longer up, in place: what `alive()` does to `this.rows`, the array `file` then goes on writing.
fn prune_rows(g: &mut Game) {
    let (t, secs) = (g.time, g.k.cues.secs);
    g.cues.rows.retain(|r| r.t <= t && t - r.t < secs);
}

/// `done` of `file`: tally, log, answer.
fn file_done(g: &mut Game, t: f64, text: &str, cls: f64, who: &str, kind: CuesFileKind, drawn: bool, why: &str) -> bool {
    if drawn { g.cues.stats.drawn[cls as usize] += 1.0; }
    g.cues.log.push(CuesLogItem { why: String::from(why) });
    if g.cues.log.len() > 400 { g.cues.log.drain(0..100); }
    drawn
}

/// One row for the strip, or not; `raw` is always drawn. `classT`, the gate's clock, is read by laststand.ts through netWait.
fn cues_file(g: &mut crate::game::Game, text: &str, cls: f64, who: &str, kind: crate::cues::CuesFileKind) -> bool {
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
    g.cues.rows.push(CueRow { t, text: String::from(text), cls, rank });
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

/// A row in the first class with no man's name on it: control, the handler, the staff's net.
pub fn cues_radio(g: &mut crate::game::Game, text: &str) {
    cues_file(g, text, 0.0, "", CuesFileKind::Raw);
}

/// The row a man's net line files: under his callsign, a squad's word under the squad's name already.
fn net_row(callsign: &str, text: &str) -> String {
    if squad_word(text) { String::from(text) } else { alloc::format!("{}: {}", callsign, text) }
}

/// A unit's line went out on his side's net: the strip gets it.
pub fn cues_heard(g: &mut crate::game::Game, gd: GuardId, text: &str) {
    if g.gd[gd].team != crate::teams::Team::Hostile || !crate::roomfight::fight_live(g) { return; }
    g.cues.stats.heard += 1.0;
    let callsign = g.gd[gd].callsign.clone();
    if cues_file(g, &net_row(&callsign, text), cue_class(text), &callsign, CuesFileKind::Heard) { g.cues.stats.heard_drawn += 1.0; }
}

/// A defender's line that is telling only: the strip's row, and this file's own bubble (never Guard.bubble).
pub fn cues_net(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: f64) {
    let t = g.time;
    g.cues.bubbles.set(gd, Bubble { text: String::from(text), t, dur: crate::guards::bubble_secs(text), radio: true });
    if !crate::roomfight::fight_live(g) { return; }
    g.cues.stats.heard += 1.0;
    let callsign = g.gd[gd].callsign.clone();
    if cues_file(g, &net_row(&callsign, text), cls, &callsign, CuesFileKind::Heard) { g.cues.stats.heard_drawn += 1.0; }
}

/// A teammate calls what the fog hides: a bubble, and a row in the first class. Whether the strip drew it.
pub fn cues_call(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool {
    let t = g.time;
    g.cues.bubbles.set(gd, Bubble { text: String::from(text), t, dur: crate::guards::bubble_secs(text), radio: false });
    let callsign = g.gd[gd].callsign.clone();
    cues_file(g, &alloc::format!("{}: {}", callsign, text), 1.0, &callsign, CuesFileKind::Call)
}

/// Every ear of the team's a shout could reach.
fn cues_ears(g: &mut crate::game::Game) -> Vec<V3> {
    let at = |p: V3| -> V3 { [p[0], p[1] + 1.6, p[2]] };
    let mut out = Vec::new();
    for pl in crate::seats::men(g) {
        if !crate::condition::down(g, g.pl[pl].ch) { out.push(at(g.ch[g.pl[pl].ch].pos)); }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let c = g.gd[o].ch;
        if g.gd[o].team == crate::teams::Team::Operative && !crate::condition::down(g, g.gd[o].ch) && g.ch[c].alive { out.push(at(g.ch[c].pos)); }
    }
    out
}

/// `done` of `overheard`.
fn heard_done(g: &Game, t: f64, text: &str, why: &str) -> bool {
    why.is_empty()
}

/// A hidden defender's shout that an ear of the team's really gets: a row of its own, his words alone. Whether the strip drew it.
pub fn cues_overheard(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool {
    let t = g.time;
    if !is_shout(text) || crate::condition::down(g, g.gd[gd].ch) { return false; }
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

/// Seconds until the strip would take a line of this class without dropping it for the class's gap (0 = now).
pub fn cues_net_wait(g: &crate::game::Game, cls: f64) -> f64 {
    let c = cls as usize;
    let t = g.time;
    match sparse_get(&g.cues.class_t, c) {
        Some(at) if t >= at => m::max(0.0, at + g.k.cues.gaps.get(c).copied().unwrap_or(0.0) + 0.05 - t),
        _ => 0.0,
    }
}

/// A word over a man and nothing else: the bubble, no strip row, no class clock: nothing the net's own lines wait on (the recap's side call)
pub(super) fn cues_aside(g: &mut crate::game::Game, gd: GuardId, text: &str) {
    let t = g.time;
    g.cues.bubbles.set(gd, Bubble { text: String::from(text), t, dur: crate::guards::bubble_secs(text), radio: false });
}

/// What a man is saying: his line (Guard.bubble) or a cue of this file's, whichever is the later and still running. A cue that has had its time is passed over and
/// left where it is: the next one takes its place.
pub(super) fn cues_bubble_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::cues::Bubble> {
    let b = g.gd[gd].bubble.as_ref().map(|b| Bubble { text: b.text.clone(), t: b.t, dur: b.dur, radio: b.radio });
    let Some(c) = g.cues.bubbles.get(gd).cloned() else { return b; };
    let t = g.time;
    if t < c.t || t - c.t > c.dur { return b; }
    match b {
        Some(b) if b.t >= c.t => Some(b),
        _ => Some(c),
    }
}

/// A man's bubble as a HUD words it: a civilian by his own state; anybody else calm, wary while he is searching or suspicious, the red of a fight while alert. A
/// man of the player's own never wears the wary tone: left searching after a fight went cold, he speaks calmly.
fn bubble_tone(g: &crate::game::Game, gd: GuardId) -> crate::view::BubbleTone {
    use crate::civilians::CivState as C;
    use crate::game::GuardState as S;
    use crate::view::BubbleTone as T;
    let m = &g.gd[gd];
    if let Some(c) = m.civ {
        return match g.cv[c].state { C::Work | C::Held => T::Calm, C::Startled | C::Cower => T::Sus, _ => T::Alert };
    }
    match m.state {
        S::Alert => T::Alert,
        S::Suspicious | S::Search if m.team != crate::teams::Team::Operative => T::Sus,
        _ => T::Calm,
    }
}

/// WHAT A SEAT'S HUD IS TOLD IS BEING SAID ON THE FLOOR, each line with where it goes. A man nobody on the team sees has no line over him (his line is still heard); a
/// man of the seat's own is drawn at the edge of the view while he is off the screen, and always while the fog hides him from the shown picture; a man of the
/// team who went down is called from where he fell; the seat's own man has his line to the man in his arm. A man is named by his character's id.
pub fn bubbles_view(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::BubbleView> {
    use crate::view::{BubblePlace, BubbleTone, BubbleView};
    let t = g.time;
    let up = |said: f64, dur: f64| said <= t && t - said <= dur;
    let mut out = Vec::new();
    for &gd in g.guards.iter().chain(g.civilians.iter()) {
        let Some(b) = cues_bubble_of(g, gd) else { continue; };
        let m = &g.gd[gd];
        if !up(b.t, b.dur) || g.cond[m.ch].down { continue; }
        if m.civ.map_or(false, |c| g.cv[c].state == crate::civilians::CivState::Gone) { continue; }
        let hidden = crate::fogofwar::fog_of_war_hidden(g, gd);
        let place = if !crate::orders::squad_orders_mine(g, seat, gd) {
            if hidden { continue; }
            BubblePlace::Over
        } else if hidden { BubblePlace::Edge } else { BubblePlace::OverOrEdge };
        let c = &g.ch[m.ch];
        out.push(BubbleView { man: Some(c.id), edge: alloc::format!("{}: {}", m.callsign, b.text), text: b.text, age: t - b.t, dur: b.dur, radio: b.radio, tone: bubble_tone(g, gd), place, at: c.pos });
    }
    for r in g.squad_net.rows.iter().filter(|r| up(r.t, r.dur)) {
        out.push(BubbleView { man: None, text: r.text.clone(), edge: r.text.clone(), age: t - r.t, dur: r.dur, radio: false, tone: BubbleTone::Alert, place: BubblePlace::OverOrEdge, at: r.at });
    }
    let me = &g.pl[seat.player];
    if let Some(b) = me.bubble.as_ref().filter(|b| !g.cond[me.ch].down && up(b.t, b.dur)) {
        let c = &g.ch[me.ch];
        out.push(BubbleView { man: Some(c.id), text: b.text.clone(), edge: b.text.clone(), age: t - b.t, dur: b.dur, radio: b.radio, tone: BubbleTone::Own, place: BubblePlace::Over, at: c.pos });
    }
    out
}

// ---------------------------------------------------------------- the words

/// A place as a line says it.
fn cues_at(g: &crate::game::Game, p: V3) -> crate::cues::CuesAt {
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

/// His row of `men`, made the first time he is asked after: the row itself.
fn cues_man<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::cues::Man {
    if !g.cues.men.has(gd) {
        let row = Man { state: g.gd[gd].state, alert_t: g.time, said_t: -100.0, again_t: -100.0, stood_t: -100.0, moved: None, from: String::new(), covered: false };
        g.cues.men.set(gd, row);
    }
    g.cues.men.get_mut(gd).unwrap()
}

/// One cue from this man, if he can speak and has not just spoken: a bubble, and (a net line) a row for the strip.
fn cues_cue(g: &mut crate::game::Game, gd: GuardId, kind: crate::cues::CueKind, vars: Option<&JsMap<String, String>>) -> bool {
    let t = g.time;
    let said_t = cues_man(g, gd).said_t;
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].held.is_some() || crate::condition::is_blind(g, g.gd[gd].ch) || t - said_t < g.k.cues.man_secs || crate::runner::runner_mute(g, gd) { return false; }
    let own = g.gd[gd].bubble.as_ref().is_some_and(|b| t >= b.t && t - b.t < 1.5);   // he has just said something of his own: that was his word on it,
    if own && !matches!(kind, CueKind::FellBack | CueKind::SquadWaits | CueKind::SquadCopiesWaits) { return false; }   // but a shout to the men beside him, or a squad's word on its net, still goes out
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

/// Has the floor just heard this kind from somebody?
fn cues_kind_busy(g: &mut crate::game::Game, kind: crate::cues::CueKind) -> bool {
    let t = g.time;
    match *kind_t_of(&mut g.cues.kind_t, kind) {
        Some(kt) => t >= kt && t - kt < gap_of(kind),
        None => false,
    }
}

/// Can he speak for his squad?
fn cues_fit(g: &crate::game::Game, gd: GuardId) -> bool {
    let t = g.time;
    !crate::condition::down(g, g.gd[gd].ch) && g.gd[gd].held.is_none() && !crate::condition::is_blind(g, g.gd[gd].ch) && !crate::condition::is_folded(g, g.gd[gd].ch) && !crate::runner::runner_mute(g, gd)
}

/// The men of a squad who can speak for it.
fn cues_voices(g: &crate::game::Game, squad: EnemysquadsSquadId) -> Vec<GuardId> {
    g.tab.enemysquads_squad[squad].men.iter().copied().filter(|&m| cues_fit(g, m)).collect()
}
/// Who says a squad's word: the man the news names, else one holding, else the first fit.
fn cues_squad_voice(g: &crate::game::Game, squad: EnemysquadsSquadId, by: Option<GuardId>) -> Option<GuardId> {
    let men = cues_voices(g, squad);
    if let Some(m) = by.filter(|m| men.contains(m)) { return Some(m); }
    let holding = men.iter().copied().find(|&m| g.fight.defs.iter().any(|&d| g.tab.roomfight_def[d].gd == m && crate::roomfight::at_post(g, d)));
    holding.or(men.first().copied())
}

// ---------------------------------------------------------------- once a frame, after everybody has moved

/// Hear the fight's news and each defender as he stands, and say what they decided. It reaches `file`, so it moves the clock laststand.ts waits on.
pub fn cues_tick(g: &mut crate::game::Game, news: &[News]) {
    let t = g.time;
    if !g.fight.live { return; }
    if t < g.cues.last_t { cues_reset(g); }
    g.cues.last_t = t;
    // 1 · what the fight's men and squads did this tick
    for &n in news {
        match n {
            News::Man(gd, did) => cues_man_did(g, gd, did),
            News::Squad(squad, by, word) => cues_squad_word(g, squad, by, word),
        }
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
        if state == crate::game::GuardState::Alert && cues_man(g, gd).state != crate::game::GuardState::Alert { cues_man(g, gd).alert_t = t; }
        let c = g.gd[gd].ch;
        let sight = t - g.gd[gd].last_seen_t;
        let still = m::hyp2(g.ch[c].vel[0], g.ch[c].vel[2]) < 0.3;
        let hold = crate::roomfight::at_post(g, d);
        let on_player = crate::teams::foe_is_player(g, gd);
        let faced_down = |g: &Game| -> Option<PlayerId> {
            let f = crate::seats::faced_man(g, gd);
            if crate::condition::down(g, g.pl[f].ch) { Some(f) } else { None }
        };
        if state == crate::game::GuardState::Patrol && cues_man(g, gd).state != crate::game::GuardState::Patrol && hold && t - cues_man(g, gd).stood_t > 15.0 {
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
            && !(on_player && faced_down(g).is_some()) && !crate::mission::lies_in_wait(g, gd)
        {
            let p = g.ch[c].pos;
            let at = cues_at(g, p);
            if cues_cue(g, gd, CueKind::Still, Some(&at_vars(&at))) { cues_man(g, gd).again_t = t; }
        }
        let now = g.gd[gd].state;
        let M = cues_man(g, gd);
        M.state = now;
    }
    if g.cues.squelch_due {
        g.cues.squelch_due = false;
        let squelch = g.k.cues.squelch;
        if squelch > 0.0 { crate::audio_iface::game_audio_like_play(g, "radio", None, Some(squelch), None); }
    }
}

/// A squad's word, in the words of a man of it. `{sq}` is its name and `{ally}` whose call it heard.
fn cues_squad_word(g: &mut crate::game::Game, squad: EnemysquadsSquadId, by: Option<GuardId>, word: Word) {
    let Some(v) = cues_squad_voice(g, squad, by) else { return };
    let name = |q: EnemysquadsSquadId| g.tab.enemysquads_squad[q].name.clone();
    let mut vars = JsMap::new();
    vars.set(String::from("sq"), name(squad));
    let mut voices = vec![v];
    let kind = match word {
        Word::Hears { ally, waits } => {
            if let Some(a) = ally { vars.set(String::from("ally"), name(a)); }
            // (a drilled garrison holding on purpose is the tell that its word comes late: if its voice has just spoken, the next man of the squad says it)
            if waits { voices.extend(cues_voices(g, squad).into_iter().filter(|&m| m != v)); }
            match (ally.is_some(), waits) {
                (false, false) => CueKind::SquadHears,
                (true, false) => CueKind::SquadCopies,
                (false, true) => CueKind::SquadWaits,
                (true, true) => CueKind::SquadCopiesWaits,
            }
        }
        Word::Stays => CueKind::SquadStays,
        Word::Quiet => CueKind::SquadQuiet,
    };
    for m in voices {
        if cues_cue(g, m, kind, Some(&vars)) { break; }
    }
}

/// What a defender has done, in his own words.
fn cues_man_did(g: &mut crate::game::Game, gd: GuardId, did: Did) {
    let Some(d) = crate::roomfight::def_of(g, gd) else { return };
    if crate::condition::down(g, g.gd[gd].ch) { return; }
    cues_man(g, gd);
    let p = g.ch[g.gd[gd].ch].pos;
    match did {
        Did::HoldsWhereHeIs => {
            if g.time - cues_man(g, gd).stood_t > 15.0 {
                let at = cues_at(g, p);
                if cues_cue(g, gd, CueKind::HuntOver, Some(&at_vars(&at))) { cues_man(g, gd).stood_t = g.time; }
            }
        }
        Did::FallsBack => {
            cues_man(g, gd).moved = Some(ManMoved::Fallback);
            let station = g.tab.roomfight_def[d].station;
            let from = crate::dialogue::place_name(g, station);
            cues_man(g, gd).from = from.clone();
            cues_man(g, gd).said_t = -100.0;
            let mut vars = JsMap::new();
            vars.set(String::from("the"), from);
            cues_cue(g, gd, CueKind::FellBack, Some(&vars));
        }
        Did::TakesCover => cues_man(g, gd).moved = Some(ManMoved::Cover),
        Did::Plants { fighting } => {
            if cues_man(g, gd).moved == Some(ManMoved::Fallback) && !fighting {
                let h = cues_at(g, p);
                let floor = g.time - g.k.cues.man_secs;
                let row = cues_man(g, gd);
                row.said_t = m::min(row.said_t, floor);
                let kind = if h.the == row.from { CueKind::PinnedIn } else { CueKind::Planted };
                cues_cue(g, gd, kind, Some(&at_vars(&h)));
            }
            cues_man(g, gd).moved = None;
        }
        Did::Surprised => { cues_cue(g, gd, CueKind::Surprised, None); }
        Did::Flinches => { cues_cue(g, gd, CueKind::Flinch, None); }
    }
}

/// THE RADIO STRIP, oldest first: the rows alive on the net, and at most one shout that reached an ear, only in a place no other row wants.
pub fn radio_rows(g: &crate::game::Game) -> Vec<crate::view::RadioRow> {
    use crate::view::{RadioKind, RadioRow};
    let (t, secs, most) = (g.time, g.k.cues.secs, g.k.cues.rows);
    let live: Vec<&CueRow> = g.cues.rows.iter().filter(|r| r.t <= t && t - r.t < secs).collect();
    let from = tail_from(live.len(), most);
    let mut out: Vec<RadioRow> = live[from..].iter().map(|r| RadioRow { kind: RadioKind::Net, text: r.text.clone(), age: t - r.t }).collect();
    if let Some(h) = &g.cues.heard_row {
        if (out.len() as f64) < most && h.t <= t && t - h.t < secs {
            out.push(RadioRow { kind: RadioKind::Heard, text: h.text.clone(), age: t - h.t });
            js::sort_by(&mut out, |a, b| b.age - a.age);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A line filed at time t by the man it is under.
    fn heard(g: &mut Game, t: f64, who: &str, words: &str, cls: f64) -> (bool, String) {
        g.time = t;
        let drawn = cues_file(g, &alloc::format!("{}: {}", who, words), cls, who, CuesFileKind::Heard);
        (drawn, g.cues.log.last().unwrap().why.clone())
    }
    fn rows(g: &Game) -> Vec<&str> { g.cues.rows.iter().map(|r| r.text.as_str()).collect() }

    /// A FAILURE MEANS: a squad's word is filed as a man's, or a man's callsign reads as a squad: the row is written twice over or waits on the wrong clock.
    #[test]
    fn a_row_under_one_capitalised_word_is_a_squads_and_a_callsign_is_not() {
        for s in ["Bravo: copy", "Ab: x", "Abcdefghijkl: x"] { assert!(squad_word(s), "{}", s); }
        for s in ["Tango 4: x", "bravo: x", "Bravo:x", "A: x", "Abcdefghijklm: x", "moving to the fight"] { assert!(!squad_word(s), "{}", s); }
        assert_eq!(net_row("Tango 4", "Bravo: copy"), "Bravo: copy");
        assert_eq!(net_row("Tango 4", "on my way"), "Tango 4: on my way");
    }

    /// A FAILURE MEANS: a player reads "{at}" on the strip.
    #[test]
    fn a_cue_fills_its_places_and_a_place_nobody_named_says_nothing() {
        let mut v = JsMap::new();
        v.set(String::from("at"), String::from("in the mess"));
        assert_eq!(fill("shots {at} — holding{nope}", Some(&v)), "shots in the mess — holding");
        assert_eq!(fill("the door—!", None), "the door—!");
    }

    /// A FAILURE MEANS: a written line ranks otherwise on the net than its own row says: a retreat is outranked by chatter, or chatter takes a retreat's place.
    #[test]
    fn a_lines_words_rank_it_as_its_row_does() {
        for l in LINES.iter().filter(|l| l.cls > 0.0) {
            for text in l.say.iter().chain(l.now.unwrap_or(&[])).chain(l.guide.unwrap_or(&[])) { assert_eq!(cue_class(text), l.cls, "{}", text); }
        }
    }

    /// A FAILURE MEANS: the net's lines change rank: the first words that match decide, the case does not, a word at the head must be that word, and what matches nothing is the net knowing something.
    #[test]
    fn a_radio_line_ranks_by_its_first_matching_words() {
        for (text, cls) in [
            ("the mess is lost — getting ready", 5.0),   // a retreat, whatever else it says
            ("They Are Still Out There", 6.0),
            ("hold where you are", 2.0),
            ("I've got this door", 3.0),
            ("GO GO GO", 1.0),
            ("set up the door", 1.0),
            ("settle down", 4.0),                        // "set" only as a word
            ("reset the door", 4.0),                     // and only at the head
            ("Moving!", 1.0),
            ("not moving!", 4.0),
            ("contact left!", 4.0),
        ] {
            assert_eq!(cue_class(text), cls, "{}", text);
        }
    }

    /// A FAILURE MEANS: one kind of cue hushes another (a man standing down silences the man who has lost them), or the floor says one kind over itself.
    #[test]
    fn a_kind_waits_only_on_itself_and_only_for_its_gap() {
        let mut game = Game::blank();
        let g = &mut game;
        *kind_t_of(&mut g.cues.kind_t, CueKind::Calm) = Some(10.0);
        g.time = 13.9;
        assert!(cues_kind_busy(g, CueKind::Calm));
        assert!(!cues_kind_busy(g, CueKind::HuntOver));
        g.time = 14.0;
        assert!(!cues_kind_busy(g, CueKind::Calm));
        *kind_t_of(&mut g.cues.kind_t, CueKind::Flinch) = Some(14.0);
        assert!(!cues_kind_busy(g, CueKind::Flinch), "a shout at the door has no gap");
        g.time = 9.0;   // (the clock went back: a checkpoint loaded)
        assert!(!cues_kind_busy(g, CueKind::Calm));
    }

    /// A FAILURE MEANS: the strip tells one piece of news twice in ten seconds because another man said it, or holds a line back from a net that has not said it for longer.
    #[test]
    fn the_same_words_from_another_man_wait_out_the_dup_time() {
        let mut game = Game::blank();
        let g = &mut game;
        let dup = g.k.cues.dup_secs;
        assert_eq!(heard(g, 1.0, "Tango 1", "on my way", 1.0), (true, String::new()));
        assert_eq!(heard(g, 1.5, "Tango 2", "on my way", 1.0), (false, String::from("dup")));
        assert_eq!(heard(g, 1.0 + dup, "Tango 2", "on my way", 1.0), (true, String::new()));
    }

    /// A FAILURE MEANS: laststand's men wait on a clock the strip does not keep, or a class talks over the net once the strip is nearly full.
    #[test]
    fn a_class_waits_its_gap_only_with_the_strip_nearly_full_and_the_wait_is_told() {
        let mut game = Game::blank();
        let g = &mut game;
        let gap = g.k.cues.gaps[1];
        assert_eq!(cues_net_wait(g, 1.0), 0.0, "nothing said yet");
        assert!(heard(g, 1.0, "Tango 1", "on my way", 1.0).0);
        assert!((cues_net_wait(g, 1.0) - (gap + 0.05)).abs() < 1e-9);
        assert_eq!(cues_net_wait(g, 2.0), 0.0, "another class has its own clock");
        assert!(heard(g, 1.1, "Tango 2", "pushing", 1.0).0, "one row up: nobody waits");
        assert_eq!(heard(g, 1.2, "Tango 3", "moving around", 1.0), (false, String::from("gap")));
        g.time = 1.1 + gap + 0.05;
        assert_eq!(cues_net_wait(g, 1.0), 0.0);
        assert!(heard(g, 1.1 + gap + 0.05, "Tango 3", "moving around", 1.0).0, "the wait told is enough");
        g.time = 0.5;   // (the clock went back)
        assert_eq!(cues_net_wait(g, 1.0), 0.0);
    }

    /// A FAILURE MEANS: a squad's word is held back by a man's chatter of its class, or two squads talk over each other.
    #[test]
    fn a_squads_word_waits_only_on_another_squads() {
        let mut game = Game::blank();
        let g = &mut game;
        heard(g, 1.0, "Tango 1", "on my way", 1.0);
        heard(g, 1.1, "Tango 2", "pushing", 1.0);
        g.time = 1.2;
        assert!(cues_file(g, "Bravo: moving up", 1.0, "Tango 3", CuesFileKind::Heard), "two men just spoke: not a squad");
        g.cues.rows.remove(0);
        assert!(!cues_file(g, "Charlie: coming to you", 1.0, "Tango 4", CuesFileKind::Heard));
        assert_eq!(g.cues.log.last().unwrap().why, "gap");
    }

    /// A FAILURE MEANS: one class holds the whole strip, a line is bumped before it could be read, or a worse line takes a better row's place.
    #[test]
    fn a_full_strip_takes_a_place_from_a_class_over_its_share_then_its_own_then_the_worst() {
        // three rows of one class, over its share of two: its oldest goes for any other class
        let mut game = Game::blank();
        let g = &mut game;
        heard(g, 1.0, "T1", "on my way", 1.0);
        heard(g, 1.1, "T2", "pushing", 1.0);
        heard(g, 2.4, "T3", "moving around", 1.0);
        assert!(heard(g, 2.5, "T4", "lost them", 2.0).0);
        assert_eq!(rows(g), ["T2: pushing", "T3: moving around", "T4: lost them"]);
        // its own class has its share: the older of those, but not before it has been read
        let mut game = Game::blank();
        let g = &mut game;
        heard(g, 0.0, "T1", "on my way", 1.0);
        heard(g, 0.5, "T2", "pushing", 1.0);
        heard(g, 1.0, "T3", "lost them", 2.0);
        assert_eq!(heard(g, 2.0, "T4", "moving around", 1.0), (false, String::from("full")));
        assert!(heard(g, g.k.cues.min_show, "T4", "moving around", 1.0).0);
        assert_eq!(rows(g), ["T2: pushing", "T3: lost them", "T4: moving around"]);
        // else the worst row, and only for a better line
        let mut game = Game::blank();
        let g = &mut game;
        heard(g, 0.0, "T1", "on my way", 1.0);
        heard(g, 0.1, "T2", "lost them", 2.0);
        heard(g, 0.2, "T3", "holding my post", 3.0);
        assert_eq!(heard(g, 0.3, "T4", "still out there", 6.0), (false, String::from("full")));
        assert!(heard(g, 3.2, "T5", "quiet now", 2.0).0);
        assert_eq!(rows(g), ["T1: on my way", "T2: lost them", "T5: quiet now"]);
    }

    /// A FAILURE MEANS: control's word to all is lost to the fight's chatter, or the chatter bumps it off.
    #[test]
    fn controls_lines_are_always_drawn_and_a_mans_line_never_takes_their_place() {
        let mut game = Game::blank();
        let g = &mut game;
        g.time = 1.0;
        heard(g, 1.0, "T1", "on my way", 1.0);
        heard(g, 1.1, "T2", "lost them", 2.0);
        heard(g, 1.2, "T3", "holding my post", 3.0);
        g.time = 1.3;
        for _ in 0..3 { assert!(cues_file(g, "control to all", 0.0, "", CuesFileKind::Raw)); }
        assert_eq!(rows(g), ["control to all", "control to all", "control to all"]);
        assert_eq!(heard(g, 3.0, "T4", "pushing", 1.0), (false, String::from("full")));
    }

    /// A FAILURE MEANS: the strip reads out of order, shows a row of a future the clock went back from, or a shout that has had its time.
    #[test]
    fn the_strip_reads_oldest_first_with_a_shout_in_its_place() {
        let mut game = Game::blank();
        let g = &mut game;
        g.time = 20.0;
        let row = |t: f64, text: &str| CueRow { t, text: String::from(text), cls: 1.0, rank: 3.0 };
        let strip = |g: &Game| radio_rows(g).into_iter().map(|r| r.text).collect::<Vec<_>>();
        g.cues.rows = vec![row(14.0, "p"), row(18.0, "q"), row(21.0, "future")];
        g.cues.heard_row = Some(CuesHeardRow { t: 16.0, text: String::from("find them!") });
        assert_eq!(strip(g), ["p", "find them!", "q"]);
        g.cues.heard_row = Some(CuesHeardRow { t: 11.0, text: String::from("old!") });
        assert_eq!(strip(g), ["p", "q"]);
    }
}
