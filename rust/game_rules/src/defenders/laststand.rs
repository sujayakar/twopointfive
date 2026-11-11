pub use ::game_types::laststand::types;
pub use types::*;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec::Vec};

/// THE LINES, each to their own side on their net: (kind, cls, say, now, guide). {n} = how many answered, {the} = 'the office', {at} = 'in the office'.
/// `now` / `guide` are the same lines, place for place, as the dialogue pass and the writing guide have them.
static LINES: &[(LineKind, f64, &[&str], Option<&[&str]>, Option<&[&str]>)] = &[
    (LineKind::Check1, 4.0, &["radio check — who's still up? sound off!", "all posts, radio check — who's left? sound off!"], None, None),
    (LineKind::CheckN, 4.0, &["radio check — sound off, who's still up?", "count off — who's still with us?"], None, None),
    (LineKind::CheckStand, 4.0, &["anyone else out there? we're holed up in {the} — get to us!", "any post still up — we're in {the}. sound off!"], None, None),
    (LineKind::CheckLoneAt, 4.0, &["anybody? anybody still up? I'm still {at}.", "if anyone's left — I'm still {at}. sound off!"], None, None),
    (LineKind::AnsAt, 3.0, &["still up — {at}.", "here — holding {the}.", "still here — {at}."], None, None),
    (LineKind::AnsFreeAt, 3.0, &["still up — after them, {at}.", "here — on them, {at}."], None, Some(&["still up — going after them, {at}.", "here — I'm after them, {at}."])),
    (LineKind::FewAt, 3.0, &["that's only {n} of us — I'm {at}. stay on your doors!", "{n} of us left — I'm {at}. nobody opens a door!"], Some(&["that's only {n} of us — I'm {at}.", "{n} of us left — I'm {at}."]), None),
    (LineKind::LastAt, 3.0, &["nobody's answering — if anyone can hear me, I'm {at}!", "no answer… it's just me. if anyone's left, I'm {at}!"], None, None),
    (LineKind::MovedAt, 3.0, &["moved — I'm {at} now.", "I've moved — {at} now."], None, None),
    (LineKind::MovedFree, 3.0, &["I'm out of {the} — going after them.", "left {the} — I'm on them."], None, None),
    (LineKind::LastGo, 3.0, &["nobody's answering — if anyone can hear me, I'm going back to {the}!", "no answer… it's just me. I'll be in {the} if anyone's left!"], None, None),
    (LineKind::Order, 1.0, &["that's {n} of us left — everybody to {the}, we hold it there!", "only {n} of us — fall back on {the}. we make our stand there!"], None, None),
    (LineKind::OrderFrom, 1.0, &["that's {n} of us left — get together in {the} and hold it. I can't reach you: I'm keeping {at}.", "only {n} of us — make the stand in {the}. no way through for me, I'm staying {at}."], None, None),
    (LineKind::OrderStay, 3.0, &["that's {n} of us and no way to each other — I'm {at}. block your doors and hold where you are!"], None, None),
    (LineKind::OrderShut, 3.0, &["that's {n} of us left — I'm {at}. stay shut in where you are, nobody opens a door!"], None, None),
    (LineKind::OrderOut, 3.0, &["that's {n} of us and no way to each other — I'm {at}. hold where you are and watch every way in!"], None, None),
    (LineKind::Go, 1.0, &["moving to {the} — don't shoot me coming in!", "on my way to {the}!"], None, None),
    (LineKind::Cant, 3.0, &["can't get to you — digging in {at}.", "no way through to you. I'll hold {the}."], None, None),
    (LineKind::In, 3.0, &["I'm in — {n} of us in {the}.", "made it. {n} of us in {the}."], None, None),
    (LineKind::Short, 3.0, &["can't make it — holding {at}.", "not getting through. holding {at}."], None, None),
    (LineKind::Set, 3.0, &["{the} is shut — let them come.", "door's blocked. we're set in {the}."], None, None),
    (LineKind::SetOpen, 3.0, &["we're set in {the}. they'll have to come through the door.", "in position in {the} — guns on the door."], None, None),
    (LineKind::SetLone, 3.0, &["I'm shut in {the}. if anyone's left — I'm not coming out.", "door's blocked, {the}. I'm staying put."], None, None),
    (LineKind::SetLoneOpen, 3.0, &["I'm set in {the}. if anyone's left — I'm not coming out.", "gun's on the door, {the}. I'm staying put."], None, None),
];

// ---------------------------------------------------------------- what a line keeps: Later.then, Later.redo

/// the closures of lines 264, 265, 360, 374, 395, 409 and part 2's: a line naming where its man is has been said
fn run_then(g: &mut Game, then: LaterThen) {
    match then {
        LaterThen::MovedFree { d } => {  name(g, d, None); }
        LaterThen::MovedAt { d, w } => {  name_at(g, d, Some(&w)); }
        LaterThen::NameAt { d, w } => name_at(g, d, w.as_ref()),
        LaterThen::NameAtUnlessFree { d, w } => {
            let free = crate::roomfight::is_loose(g, d);
            name_at(g, d, if free { None } else { Some(&w) });
        }
    }
}
/// the `f` of lines 265, 360, 374, 395: a line's words and its `then`, the place asked now
fn redo_words(g: &Game, redo: &LaterRedo) -> NetRedoRet {
    match *redo {
        LaterRedo::MovedAt { d } => {
            let w = man_spoken(g, d);
            NetRedoRet { vars: vars_of(&[("at", w.at.as_str()), ("the", w.the.as_str())]), then: Some(LaterThen::MovedAt { d, w }) }
        }
        LaterRedo::CheckLone { d } => {
            let w = man_spoken(g, d);
            NetRedoRet { vars: vars_of(&[("at", w.at.as_str())]), then: Some(LaterThen::NameAtUnlessFree { d, w }) }
        }
        LaterRedo::Answer { d, free } => {
            let w = man_spoken(g, d);
            NetRedoRet { vars: vars_of(&[("at", w.at.as_str()), ("the", w.the.as_str())]), then: Some(LaterThen::NameAt { d, w: if free { None } else { Some(w) } }) }
        }
        LaterRedo::Tell { d, n, free } => {
            let w = man_spoken(g, d);
            let count = alloc::format!("{}", n);
            NetRedoRet { vars: vars_of(&[("n", count.as_str()), ("at", w.at.as_str())]), then: Some(LaterThen::NameAt { d, w: if free { None } else { Some(w) } }) }
        }
    }
}
/// what `net` works out before it says or keeps a line: the wording, its words, its class, when it may go out. Draws the die for a kind's first line
fn line_slot(g: &mut Game, kind: LineKind, vars: Option<&JsMap<String, String>>) -> (&'static str, String, f64, f64) {
    let row = LINES.iter().find(|l| l.0 == kind).unwrap();
    let (cls, say) = (row.1, row.2);
    let len = say.len() as f64;
    let st = st_of(g);
    let i = match st.rot.get(kind) { Some(i) => *i, None => js::rem(m::floor(st.die.roll() * len), len) };
    st.rot.set(kind, js::rem(i + 1.0, len));
    let tpl: &'static str = row.4.or(row.3).unwrap_or(say)[i as usize];
    let empty = JsMap::new();
    let text = fill(tpl, vars.unwrap_or(&empty));
    let wait = crate::cues::cues_net_wait(g, cls);
    let time = g.time;
    let gap = g.k.cues.gaps.get(cls as usize).copied().unwrap_or(0.0);
    let st = st_of(g);
    let at = m::max(time + wait, st.cls_at.get(cls as usize).copied().unwrap_or(0.0));
    if st.cls_at.len() <= cls as usize { st.cls_at.resize(cls as usize + 1, 0.0); }
    st.cls_at[cls as usize] = at + gap + 0.1;
    (tpl, text, cls, at)
}
// ---------------------------------------------------------------- small helpers

/// the row (STATES.get(g)): it stays in g.m.laststand.states while the tick runs; let go of it before the next call that takes g
fn st_of(g: &mut Game) -> &mut State { g.m.laststand.states.as_mut().unwrap() }
/// the check that is open (st.check), by its path: tick_check's C
fn check_of(g: &mut Game) -> &mut Check { st_of(g).check.as_mut().unwrap() }
/// JavaScript's truth of an optional string
fn id_truthy(id: &Option<String>) -> bool { id.as_deref().map_or(false, |s| !s.is_empty()) }
/// where this man's body stands
fn man_pos(g: &Game, d: RoomfightDefId) -> V3 { let gd = g.tab.roomfight_def[d].gd; g.pos_of(gd) }
/// (:230) the place as this man says it, asked now
fn man_spoken(g: &Game, d: RoomfightDefId) -> Spoken { let p = man_pos(g, d); spoken(g, p) }
/// the defenders who are `alive`, in the list's order
fn up_defs(g: &Game) -> Vec<RoomfightDefId> { let defs = g.fight.defs.clone(); defs.into_iter().filter(|&d| alive(g, d)).collect() }
/// a line's `vars`
fn vars_of(pairs: &[(&str, &str)]) -> JsMap<String, String> { let mut v = JsMap::new(); for (k, x) in pairs { v.set(String::from(*k), String::from(*x)); } v }
/// `/^the |\d$/.test(n) ? n : 'the ' + n` (:191, :327)
fn the_word(n: &str) -> String {
    if n.starts_with("the ") || n.ends_with(|c: char| c.is_ascii_digit()) { String::from(n) } else { alloc::format!("the {}", n) }
}
// ---------------------------------------------------------------- the module's functions

/// a new tally of the endgame
fn new_stats() -> crate::laststand::StateStats {
    StateStats {
        t0: f64::NAN, checks: 0.0, last_man: 0.0,
        stand_t: f64::NAN, set_t: f64::NAN,
    }
}

/// up, and not broken and run: off the net from his shout on
fn alive(g: &crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    !crate::condition::down(g, g.gd[gd].ch) && !crate::owner::has(g, gd, crate::owner::by::RUNNER)
}

/// a beat between two numbers, off the state's own die
fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 {
    r[0] + st_of(g).die.roll() * (r[1] - r[0])
}

/// fit to speak or be spoken to: up, not held, his wits about him
fn fit_gd(g: &crate::game::Game, gd: GuardId) -> bool {
    !crate::condition::down(g, g.gd[gd].ch) && g.gd[gd].held.is_none() && !crate::owner::has(g, gd, crate::owner::by::RUNNER)
        && !crate::control::man::blind(g, gd) && !crate::control::man::staggered(g, gd)
}

fn fit(g: &crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    fit_gd(g, gd)
}

/// fit, and nobody of the team in his sight these `quiet` seconds
fn calm(g: &crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    fit(g, d) && g.time - g.gd[gd].last_seen_t > g.k.laststand.quiet
}

/// where a point is, by the structure's own rooms
fn where_(g: &crate::game::Game, p: V3) -> crate::laststand::Where {
    let rooms = g.level.rooms().to_vec();
    let mut best: Option<usize> = None;
    let mut bd = f64::INFINITY;
    for (i, r) in rooms.iter().enumerate() {
        let b = &r.bounds;
        let d = m::hyp2(m::max_n(&[b.x0 - p[0], 0.0, p[0] - b.x1]), m::max_n(&[b.z0 - p[2], 0.0, p[2] - b.z1]));
        if d == 0.0 {
            let the = the_word(&r.name);
            let at = alloc::format!("in {}", the);
            return Where { id: Some(r.id.clone()), the, at };
        }
        if d < bd { bd = d; best = Some(i); }
    }
    let out = match best { Some(i) => alloc::format!("outside {}", the_word(&rooms[i].name)), None => String::from("out here") };
    Where { id: None, the: out.clone(), at: out }
}

/// where a man says he is: the level's own name for the floor he stands on, only on ground it has no word for 'outside' the nearest room
fn spoken(g: &crate::game::Game, p: V3) -> crate::laststand::Spoken {
    let w = where_(g, p);
    if id_truthy(&w.id) { return Spoken { id: w.id, the: w.the, at: w.at, spot: None }; }
    let r = match crate::dialogue::place_in(g, p) {
        Some(r) => r,
        None => return Spoken { id: w.id, the: w.the, at: w.at, spot: None },
    };
    let id = if id_truthy(&r.id) {
        let held = g.level.rooms().iter().any(|x| Some(x.id.as_str()) == r.id.as_deref());
        if held { r.id.clone() } else { None }
    } else { None };
    let spot = if id.is_some() { None } else {
        Some(Spot { name: String::from(r.name.strip_prefix("the ").unwrap_or(&r.name)), pos: [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0] })
    };
    let at = alloc::format!("in {}", r.name);
    Spoken { id, the: r.name, at, spot }
}

/// a line on their net: the strip's row, the list the checks read
fn voice(g: &mut crate::game::Game, gd: GuardId, text: &str, cls: f64) {
    let callsign = g.gd[gd].callsign.clone();
    crate::cues::cues_net(g, gd, text, cls);
    let st = st_of(g);
    st.said.push(LastStandSaid { who: callsign, text: String::from(text) });
    if st.said.len() > 200 { st.said.drain(0..50); }
}

/// a line's words with its {names} filled, an unknown name by nothing
fn fill(tpl: &str, vars: &JsMap<String, String>) -> String {
    let mut out = String::new();
    let mut rest = tpl;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let word = after.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(after.len());
        if word > 0 && after[word..].starts_with('}') {
            if let Some(v) = vars.of(&after[..word]) { out.push_str(v); }
            rest = &after[word + 1..];
        } else {
            out.push('{');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// a line on the net, now or when the strip has room for its class; `then` happens when it is said
fn net(g: &mut crate::game::Game, gd: GuardId, kind: crate::laststand::LineKind, vars: Option<&JsMap<String, String>>, then: Option<&crate::laststand::LaterThen>, redo: Option<&crate::laststand::LaterRedo>) {
    let (tpl, text, cls, at) = line_slot(g, kind, vars);
    let t0 = g.time;
    if at > t0 {
        st_of(g).later.push(Later { at, t0, gd, kind, text, cls, then: then.cloned(), tpl: String::from(tpl), redo: redo.cloned() });
    } else {
        voice(g, gd, &text, cls);
        if let Some(t) = then { run_then(g, t.clone()); }
    }
}

/// a line that says where its man stands: the place is asked again as the line is said
fn net_at(g: &mut crate::game::Game, d: RoomfightDefId, kind: crate::laststand::LineKind, f: &crate::laststand::LaterRedo) {
    let gd = g.tab.roomfight_def[d].gd;
    let a = redo_words(g, f);
    net(g, gd, kind, Some(&a.vars), a.then.as_ref(), Some(f));
}

/// the lines waiting their turn on the strip: said by the same man if he is still fit to say it
fn tick_later(g: &mut crate::game::Game) {
    if st_of(g).later.is_empty() { return; }
    let list = core::mem::take(&mut st_of(g).later);
    let mut kept = Vec::new();
    for mut l in list {
        if g.time < l.at { kept.push(l); continue; }
        if !fit_gd(g, l.gd) { continue; }
        let w = crate::cues::cues_net_wait(g, l.cls);
        if w > 0.0 && g.time - l.t0 < 8.0 { l.at = g.time + w; kept.push(l); continue; }
        if let Some(r) = l.redo.take() { let a = redo_words(g, &r); l.text = fill(&l.tpl, &a.vars); l.then = a.then; }
        voice(g, l.gd, &l.text, l.cls);
        if let Some(t) = l.then { run_then(g, t); }
    }
    st_of(g).later = kept;
}

/// this man's word on the net: a room, or none; what he said before is forgotten
fn name(g: &mut crate::game::Game, d: RoomfightDefId, id_2: Option<&str>) {
    let st = st_of(g);
    match id_2 {
        Some(id) if !id.is_empty() => { st.named.set(d, String::from(id)); }
        _ => { st.named.delete(d); }
    }
    st.spots.delete(d);
    st.at.delete(d);
}

/// named by what he said, the room or the place; kept, to be said again when it stops being true
fn name_at(g: &mut crate::game::Game, d: RoomfightDefId, w: Option<&crate::laststand::Spoken>) {
    name(g, d, w.and_then(|w| w.id.as_deref()));
    let w = match w { Some(w) if id_truthy(&w.id) || w.spot.is_some() => w, _ => return };
    let (p, time) = (man_pos(g, d), g.time);
    let st = st_of(g);
    if let Some(s) = &w.spot { st.spots.set(d, s.clone()); }
    let key = match &w.id { Some(id) => id.clone(), None => w.spot.as_ref().unwrap().name.clone() };
    st.at.set(d, Said { key, the: w.the.clone(), off_t: f64::NAN, still_at: v3::copy(p), still_t: time, said_t: time });
}

/// a man who is no longer where he said says where he is now, or that he has gone after the team
fn tick_moved(g: &mut crate::game::Game) {
    let (moved_secs, still_secs, moved_gap) = (g.k.laststand.moved_secs, g.k.laststand.still_secs, g.k.laststand.moved_gap);
    for d in st_of(g).at.keys() {
        if !alive(g, d) { st_of(g).at.delete(d); continue; }
        let gd = g.tab.roomfight_def[d].gd;
        let p = man_pos(g, d);
        let time = g.time;
        let st = st_of(g);
        let said = st.at.get_mut(d).unwrap();
        if dist_xz(p, said.still_at) > 0.4 { said.still_at = v3::copy(p); said.still_t = time; }
        let w = spoken(g, p);
        let free = crate::roomfight::is_loose(g, d);
        let now: Option<&str> = match &w.id { Some(id) => Some(id.as_str()), None => w.spot.as_ref().map(|s| s.name.as_str()) };
        let said = st_of(g).at.get_mut(d).unwrap();
        if now == Some(said.key.as_str()) { said.off_t = f64::NAN; continue; }
        if said.off_t.is_nan() { said.off_t = time; }
        let (off_t, said_t, still_t, the) = (said.off_t, said.said_t, said.still_t, said.the.clone());
        if time - off_t < moved_secs || time - said_t < moved_gap || !calm(g, d) { continue; }
        let st = st_of(g);
        if let Some(c) = st.check.as_ref() { if c.caller == d || c.due.iter().any(|x| x.d == d && x.at >= 0.0) { continue; } }
        if !free && time - still_t < still_secs { continue; }
        if st.later.iter().any(|l| l.gd == gd && (l.kind == LineKind::MovedAt || l.kind == LineKind::MovedFree)) { continue; }
        st.at.get_mut(d).unwrap().said_t = time;
        if free {
            net(g, gd, LineKind::MovedFree, Some(&vars_of(&[("the", the.as_str())])), Some(&LaterThen::MovedFree { d }), None);
        } else {
            net_at(g, d, LineKind::MovedAt, &LaterRedo::MovedAt { d });
        }
    }
}

/// one of the fit asks the net who is still up; every other man up gets his beat to answer
fn start_check(g: &mut crate::game::Game) {
    let men = up_defs(g);
    let can: Vec<RoomfightDefId> = men.iter().copied().filter(|&d| calm(g, d)).collect();
    let caller = can.iter().copied().find(|&d| crate::roomfight::at_post(g, d))
        .or_else(|| can.first().copied());
    let time = g.time;
    let st = st_of(g);
    let caller = match caller { Some(c) => c, None => { st.next_at = time + 1.0; return; } };
    let first = st.stats.checks == 0.0;
    let lone_now = st.lone_said && st.known.len() <= 1;
    st.stats.checks += 1.0;
    let caller_gd = g.tab.roomfight_def[caller].gd;
    if lone_now { net_at(g, caller, LineKind::CheckLoneAt, &LaterRedo::CheckLone { d: caller }); }
    else { net(g, caller_gd, if first { LineKind::Check1 } else { LineKind::CheckN }, None, None, None); }
    let answer = g.k.laststand.answer;
    let between = g.k.laststand.between;
    let listen = g.k.laststand.listen;
    let mut at = g.time + pick(g, answer);
    let mut due: Vec<CheckDueItem> = Vec::new();
    for &d in &men {
        if d == caller { continue; }
        due.push(CheckDueItem { d, at });
        at += pick(g, between);
    }
    let close_at = (match due.last() { Some(x) => x.at, None => g.time + 1.2 }) + pick(g, listen);
    st_of(g).check = Some(Check { caller, close_at, due, got: Vec::new() });
}

/// the answers on their beats; at its close what the survivors know of each other. C is st.check, read by its path
fn tick_check(g: &mut crate::game::Game) {
    let caller = check_of(g).caller;
    let caller_gd = g.tab.roomfight_def[caller].gd;
    if !alive(g, caller) {
        let time = g.time;
        let st = st_of(g);
        st.check = None;
        st.next_at = time + 4.0;
        return;
    }
    for i in 0..check_of(g).due.len() {
        let due = check_of(g).due[i];
        if g.time < due.at || due.at < 0.0 { continue; }
        check_of(g).due[i].at = -1.0;
        let d = due.d;
        if !calm(g, d) { continue; }
        let free = crate::roomfight::is_loose(g, d);
        net_at(g, d, if free { LineKind::AnsFreeAt } else { LineKind::AnsAt }, &LaterRedo::Answer { d, free });
        check_of(g).got.push(d);
    }
    if g.time < check_of(g).close_at { return; }
    let (time, every) = (g.time, g.k.laststand.every);
    let st = st_of(g);
    let got = st.check.take().unwrap().got;
    st.next_at = time + every;
    let mut heard: Vec<RoomfightDefId> = Vec::new();
    if alive(g, caller) { heard.push(caller); }
    for &d in &got { if alive(g, d) { heard.push(d); } }
    let mut known = JsSet::new();
    for &d in &heard { known.add(d); }
    let silent: Vec<GuardId> = g.fight.defs.iter().filter(|&&d| !known.has(d)).map(|&d| g.tab.roomfight_def[d].gd).collect();
    let heard_gds: Vec<GuardId> = heard.iter().map(|&d| g.tab.roomfight_def[d].gd).collect();
    let st = st_of(g);
    st.known = known.clone();
    crate::netquiet::net_quiet_closed(g, caller_gd, &silent, &heard_gds);
    let st = st_of(g);
    let mut names = st.named.keys();
    names.extend(st.spots.keys());
    for d in names {
        if !known.has(d) { name(g, d, None); }
    }
    if !calm(g, caller) {
        let time = g.time;
        let st = st_of(g);
        st.next_at = m::min(st.next_at, time + 5.0);
        return;
    }
    tell_only(g, caller);
}

/// nobody answers him: true the first time
fn lone(st: &mut crate::laststand::State) -> bool {
    if st.lone_said { return false; }
    st.lone_said = true;
    st.stats.last_man += 1.0;
    true
}

/// call out only: the one who asked says what the count is, and where he is
fn tell_only(g: &mut crate::game::Game, c_2: RoomfightDefId) {
    let free = crate::roomfight::is_loose(g, c_2);
    let st = st_of(g);
    let n = st.known.len() as i32;
    let redo = LaterRedo::Tell { d: c_2, n, free };
    if n <= 1 {
        if lone(st) { net_at(g, c_2, LineKind::LastAt, &redo); }
        return;
    }
    st.lone_said = false;
    if n as f64 != st.few_n {
        st.few_n = n as f64;
        net_at(g, c_2, LineKind::FewAt, &redo);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_as_the_regex_does() {
        let v = vars_of(&[("at", "in the office"), ("n", "2")]);
        assert_eq!(fill("still up — {at}.", &v), "still up — in the office.");
        assert_eq!(fill("{n} of us, {n} — {gone}!", &v), "2 of us, 2 — !");
        assert_eq!(fill("{} {a b} {", &v), "{} {a b} {");
        assert_eq!(fill("{{at}}", &v), "{in the office}");
    }

    #[test]
    fn the_word_keeps_a_name_with_its_the_or_its_number() {
        assert_eq!(the_word("office"), "the office");
        assert_eq!(the_word("the yard"), "the yard");
        assert_eq!(the_word("block 2"), "block 2");
        assert_eq!(the_word("theatre"), "the theatre");
    }

    #[test]
    fn every_kind_has_its_lines_place_for_place() {
        assert_eq!(LINES.len(), 24);
        for l in LINES {
            assert_eq!(LINES.iter().filter(|o| o.0 == l.0).count(), 1);
            if let Some(n) = l.3 { assert_eq!(n.len(), l.2.len()); }
            if let Some(n) = l.4 { assert_eq!(n.len(), l.2.len()); }
        }
    }

    #[test]
    fn new_stats_clocks_start_unset() {
        let s = new_stats();
        assert!(s.t0.is_nan() && s.stand_t.is_nan() && s.set_t.is_nan());
        assert_eq!(s.checks, 0.0);
    }

    #[test]
    fn lone_is_said_once() {
        let mut st = State::default();
        assert!(lone(&mut st));
        assert!(!lone(&mut st));
        assert_eq!(st.stats.last_man, 1.0);
    }

    #[test]
    fn flat_is_on_the_floor() {
        assert_eq!(dist_xz([3.0, 9.0, 0.0], [0.0, -2.0, 4.0]), 5.0);
    }

}

// ---------------------------------------------------------------- small private helpers

/// the module's state: every caller here runs with it made
fn st(g: &mut Game) -> &mut State { g.m.laststand.states.as_mut().unwrap() }
/// the die's seed off the room clear's seed and attempt
fn fresh_seed(seed: f64, attempt: f64) -> f64 {
    ((m::imul(js::to_i32(seed), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(attempt), 0x85EBCA6Bu32 as i32) ^ 0x1A575A4D) as i32) as f64
}

// ---------------------------------------------------------------- the stand

// ---------------------------------------------------------------- the seams with the rest of the game

/// A new state for the endgame: its own die off the room clear's seed and attempt, four draws spent; the posts as staged, kept if it is the same staging.
fn fresh(g: &crate::game::Game) -> crate::laststand::State {
    let (seed, attempt) = (g.room_clear.seed, g.room_clear.attempt);
    let mut die = crate::dice::Die(js::to_u32(fresh_seed(seed, attempt)));
    for _ in 0..4 { die.roll(); }
    let key = alloc::format!("{}:{}", seed, attempt);
    let kept = match &g.m.laststand.states { Some(was) if was.key == key => Some(was.post.clone()), _ => None };
    let post = match kept {
        Some(p) => p,
        None => {
            let mut p: JsMap<GuardId, String> = JsMap::new();
            for r in &g.staged.rooms { for &gd in &r.defenders { p.set(gd, r.def.id.clone()); } }
            p
        }
    };
    let mut rot: JsMap<LineKind, f64> = JsMap::new();
    rot.set(LineKind::MovedAt, 0.0);
    rot.set(LineKind::MovedFree, 0.0);
    State {
        post, key, die, next_at: 0.0, check: None, known: JsSet::new(), named: JsMap::new(), spots: JsMap::new(), at: JsMap::new(), 
        lone_said: false, few_n: 0.0, said: Vec::new(), stats: new_stats(), stale: false, rot, cls_at: Vec::new(), later: Vec::new(),
    }
}

/// Once a frame from RoomFight.tick, after the defenders' own ticks.
pub(super) fn last_stand_tick(g: &mut crate::game::Game) {
    step_stand(g);
}

/// The frame of the endgame: 'most are down' starts it; then the lines waiting, the check, who has moved, the stand.
fn step_stand(g: &mut crate::game::Game) {
    if g.staged.rooms.is_empty() || g.staged.over { return; }
    if g.m.laststand.states.as_ref().map_or(true, |st| st.stale) {
        let all = g.staged.foes.clone();
        let few = m::min(g.k.laststand.few, m::floor(all.len() as f64 / 3.0));
        let mut up = 0;
        for &h in &all { if !crate::condition::down(g, g.gd[h].ch) && !g.gd[h].fled { up += 1; } }
        if up as f64 > few || up == 0 { return; }
        let row = fresh(g);
        g.m.laststand.states = Some(row);
        let t = g.time;
        st(g).stats.t0 = t;
        let first = g.k.laststand.first;
        let at = t + pick(g, first);
        st(g).next_at = at;
    }
    let defs = g.fight.defs.clone();
    if !defs.iter().any(|&d| alive(g, d)) { return; }
    let mut names = st(g).named.keys();
    names.extend(st(g).spots.keys());
    for d in names { if !alive(g, d) { name(g, d, None); } }
    tick_later(g);
    if st(g).check.is_some() {
        tick_check(g);
    } else if g.time >= st(g).next_at {
        start_check(g);
    }
    if !st(g).at.is_empty() { tick_moved(g); }
}

/// The fight is over (RoomFight.stop): everything forgotten but the counts and the lines.
pub(super) fn last_stand_stop(g: &mut crate::game::Game) {
    let Some(st) = g.m.laststand.states.as_mut() else { return };
    st.check = None;
    st.known.clear();
    st.named.clear();
    st.spots.clear();
    st.at.clear();
    st.later.clear();
    st.stale = true;
}

/// the objective's hint: the rooms their net has named and the named places that are no room of it, by the men still up who said so
pub fn last_stand_hint(g: &crate::game::Game) -> Option<crate::laststand::Hint> {
    let st = g.m.laststand.states.as_ref()?;
    if st.stale { return None; }
    let mut rooms: JsSet<String> = JsSet::new();
    for (d, id) in st.named.iter() { if alive(g, d) { rooms.add(id.clone()); } }
    let mut spots: Vec<Spot> = Vec::new();
    for (d, x) in st.spots.iter() { if alive(g, d) && !spots.iter().any(|y| y.name == x.name) { spots.push(x.clone()); } }
    if rooms.is_empty() && spots.is_empty() { None } else { Some(Hint { rooms, spots }) }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn the_die_is_seeded_as_the_language_multiplies() {
        for &(seed, attempt, want) in SEEDS { assert_eq!(fresh_seed(seed, attempt).to_bits(), want.to_bits(), "{} {}", seed, attempt); }
    }

    const SEEDS: &[(f64, f64, f64)] = &[(0.0, 0.0, 441932365.0), (1.0, 1.0, 25946519.0), (12345.0, 2.0, -1330936078.0), (4294967295.0, 3.0, -363035837.0), (-7.0, 9.0, 57076647.0), (2147483648.0, 1.0, 532451366.0), (3.9, 2.9, -886660216.0), (987654321.0, 1000.0, -1273962284.0)];
}
