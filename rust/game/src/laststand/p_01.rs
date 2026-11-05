//! src/game/laststand.ts lines 1 to 408
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// THE LINES, each to their own side on their net: (kind, cls, say, now, guide). {n} = how many answered, {the} = 'the office', {at} = 'in the office'.
/// `now` / `guide` are the same lines, place for place, as the dialogue pass and the writing guide have them. (laststand.ts LINES)
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

// ---------------------------------------------------------------- seams: what the skeleton cannot do yet (notes.md, ## Skeleton)

/// (:222) a `then` handed to the public `net` kept in Later for a line that must wait: a borrowed closure cannot be kept
fn seam_keep_then(then: Option<&mut dyn FnMut(&mut Game)>) -> Option<LaterThen> {
    match then { None => None, Some(_) => todo!("needs: net's then as Option<LaterThen>") }
}
/// (:222) a `redo` handed to the public `net` kept in Later: a borrowed closure cannot be kept
fn seam_keep_redo(redo: Option<&mut dyn FnMut(&mut Game) -> NetRedoRet>) -> Option<LaterRedo> {
    match redo { None => None, Some(_) => todo!("needs: net's redo as Option<LaterRedo>") }
}

// ---------------------------------------------------------------- what a line keeps: Later.then, Later.redo

/// the closures of lines 262, 263, 358, 372, 393, 407, 418, 430: a line naming where its man is has been said
fn run_then(g: &mut Game, then: LaterThen) {
    match then {
        LaterThen::MovedFree { d } => { st_of(g).stats.moved += 1.0; name(g, d, None); }
        LaterThen::MovedAt { d, w } => { st_of(g).stats.moved += 1.0; name_at(g, d, Some(&w)); }
        LaterThen::NameAt { d, w } => name_at(g, d, w.as_ref()),
        LaterThen::NameAtUnlessFree { d, w } => {
            let free = g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Free;
            name_at(g, d, if free { None } else { Some(&w) });
        }
    }
}
/// the `f` of lines 263, 358, 372, 393: a line's words and its `then`, the place asked now
fn redo_words(g: &mut Game, redo: &LaterRedo) -> NetRedoRet {
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
    let i = match st.rot.get(kind) { Some(i) => *i, None => js::rem(m::floor(crate::math_dice::roll(&mut st.die) * len), len) };
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
/// `net` with what the line keeps as Later holds it: every line of this part goes this way (:218)
fn net_kept(g: &mut Game, gd: GuardId, kind: LineKind, vars: Option<&JsMap<String, String>>, then: Option<LaterThen>, redo: Option<LaterRedo>) {
    let (tpl, text, cls, at) = line_slot(g, kind, vars);
    let t0 = g.time;
    if at > t0 {
        st_of(g).later.push(Later { at, t0, gd, kind, text, cls, then, tpl: String::from(tpl), redo });
    } else {
        voice(g, gd, kind, &text, cls);
        if let Some(t) = then { run_then(g, t); }
    }
}
/// `netAt` by the name of its `f`: every such line of this part goes this way (:226)
fn net_at_kept(g: &mut Game, d: RoomfightDefId, kind: LineKind, redo: LaterRedo) {
    let gd = g.tab.roomfight_def[d].gd;
    let a = redo_words(g, &redo);
    net_kept(g, gd, kind, Some(&a.vars), a.then, Some(redo));
}

// ---------------------------------------------------------------- small helpers

/// the row (STATES.get(g)): it stays in g.m.laststand.states while the tick runs; let go of it before the next call that takes g
fn st_of(g: &mut Game) -> &mut State { g.m.laststand.states.as_mut().unwrap() }
/// the check that is open (st.check), by its path: tick_check's C
fn check_of(g: &mut Game) -> &mut Check { st_of(g).check.as_mut().unwrap() }
/// (:403) member, which needs nothing of g
fn of_stand(s: &Stand, d: RoomfightDefId) -> bool { s.in_.has(d) || s.out.has(d) || s.movers.iter().any(|m| m.d == d) }
/// JavaScript's truth of an optional string
fn id_truthy(id: &Option<String>) -> bool { id.as_deref().map_or(false, |s| !s.is_empty()) }
/// where this man's body stands
fn man_pos(g: &Game, d: RoomfightDefId) -> V3 { let gd = g.tab.roomfight_def[d].gd; g.ch[g.gd[gd].ch].pos }
/// (:228) the place as this man says it, asked now
fn man_spoken(g: &mut Game, d: RoomfightDefId) -> Spoken { let p = man_pos(g, d); spoken(g, p) }
/// the defenders who are `alive`, in the list's order
fn up_defs(g: &mut Game) -> Vec<RoomfightDefId> { let defs = g.fight.defs.clone(); defs.into_iter().filter(|&d| alive(g, d)).collect() }
/// a line's `vars`
fn vars_of(pairs: &[(&str, &str)]) -> JsMap<String, String> { let mut v = JsMap::new(); for (k, x) in pairs { v.set(String::from(*k), String::from(*x)); } v }
/// `/^the |\d$/.test(n) ? n : 'the ' + n` (:189, :325)
fn the_word(n: &str) -> String {
    if n.starts_with("the ") || n.ends_with(|c: char| c.is_ascii_digit()) { String::from(n) } else { alloc::format!("the {}", n) }
}
/// `+x.toFixed(2)` (:211)
fn to_fixed2(x: f64) -> f64 { js::to_fixed(x, 2).parse().unwrap_or(f64::NAN) }
/// the cell's number as a Set's key: -0 is 0 to a JavaScript Set
fn room_has(g: &mut Game, r: RoomfightRoomId, p: V3) -> bool {
    let c = cell_of(g, p);
    g.tab.roomfight_room[r].cells.has(crate::Num(c + 0.0))
}

// ---------------------------------------------------------------- the skeleton's functions

/// host work, nothing of the simulation: the debugger's panel keeping its rows in the browser's storage (laststand.ts saveLastStand)
pub fn save_last_stand(g: &mut crate::game::Game) {}

/// a new tally of the endgame (laststand.ts newStats)
pub fn new_stats() -> crate::laststand::StateStats {
    StateStats {
        t0: f64::NAN, up0: 0.0, checks: 0.0, answers: 0.0, silent: 0.0, last_man: 0.0,
        stands: 0.0, gathered: 0.0, sent: 0.0, arrived: 0.0, short: 0.0, met: 0.0, died_on_way: 0.0, dropped: 0.0, quiet: 0.0, no_way: 0.0,
        moved: 0.0,
        dig_orders: 0.0, stand_t: f64::NAN, set_t: f64::NAN, set_men: 0.0, thumps: 0.0, lines: 0.0,
    }
}

/// distance on the floor (laststand.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// up, and not broken and run: off the net from his shout on (laststand.ts alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    g.gd[gd].state != crate::game::GuardState::Dead && !crate::runner::runner_owns(g, gd)
}

/// a beat between two numbers, off the state's own die (laststand.ts pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 {
    r[0] + crate::math_dice::roll(&mut st_of(g).die) * (r[1] - r[0])
}

/// the nav cell of a point (laststand.ts cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 {
    let cell = g.col.nav.cell;
    crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell))
}

/// is the point on this room's cells (laststand.ts inRoom)
pub fn in_room(g: &mut crate::game::Game, r: Option<RoomfightRoomId>, p: V3) -> bool {
    match r { Some(r) => room_has(g, r, p), None => false }
}

/// a row of the room fight's log (laststand.ts note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) {
    let who = match gd { Some(gd) => g.gd[gd].callsign.clone(), None => String::from("-") };
    let t = g.time;
    g.fight.log.push(crate::roomfight::FightNote { t, who, what: alloc::format!("[last] {}", what) });
    if g.fight.log.len() > 600 { g.fight.log.drain(0..200); }
}

/// fit to speak or be spoken to: up, not held, his wits about him (laststand.ts fitGd)
pub fn fit_gd(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].held.is_none() && !crate::runner::runner_owns(g, gd)
        && g.time >= g.gd[gd].dazzled_until && g.time >= g.gd[gd].stagger_until
}

/// (laststand.ts fit)
pub fn fit(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    fit_gd(g, gd)
}

/// fit, and nobody of the team in his sight these `quiet` seconds (laststand.ts calm)
pub fn calm(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    fit(g, d) && g.time - g.gd[gd].last_seen_t > g.k.laststand.quiet
}

/// calm, holding a post, nobody else's man, his hands not on a door (laststand.ts movable)
pub fn movable(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    calm(g, d) && g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold && g.tab.roomfight_def[d].rally_at != f64::INFINITY
        && !crate::enemysquads::squad_defers(g, gd) && !crate::enemysquads::squad_busy(g, gd) && !crate::pressure::press_owns(g, d)
        && (crate::digin::dig_in_phase(g, gd) == "idle" || crate::digin::dig_in_phase(g, gd) == "set")
}

/// where a point is, by the structure's own rooms (laststand.ts where)
pub fn where_(g: &mut crate::game::Game, p: V3) -> crate::laststand::Where {
    let rooms = match crate::roomclear::room_clear_scenario_structure(g) { Some(s) => s.rooms, None => Vec::new() };
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

/// where a man says he is: the level's own name for the floor he stands on, only on ground it has no word for 'outside' the nearest room (laststand.ts spoken)
pub fn spoken(g: &mut crate::game::Game, p: V3) -> crate::laststand::Spoken {
    let w = where_(g, p);
    if id_truthy(&w.id) { return Spoken { id: w.id, the: w.the, at: w.at, spot: None }; }
    let r = match crate::dialogue::place_in(g, p) {
        Some(r) => r,
        None => return Spoken { id: w.id, the: w.the, at: w.at, spot: None },
    };
    let id = if id_truthy(&r.id) {
        let structure = crate::roomclear::room_clear_scenario_structure(g);
        let held = structure.map_or(false, |s| s.rooms.iter().any(|x| Some(x.id.as_str()) == r.id.as_deref()));
        if held { r.id.clone() } else { None }
    } else { None };
    let spot = if id.is_some() { None } else {
        Some(Spot { name: String::from(r.name.strip_prefix("the ").unwrap_or(&r.name)), pos: [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0] })
    };
    let at = alloc::format!("in {}", r.name);
    Spoken { id, the: r.name, at, spot }
}

/// a line on their net: the log row, the strip's row, the list the checks read (laststand.ts voice)
pub fn voice(g: &mut crate::game::Game, gd: GuardId, kind: crate::laststand::LineKind, text: &str, cls: f64) {
    let callsign = g.gd[gd].callsign.clone();
    crate::game::game_msg(g, &alloc::format!("{} (radio): {}", callsign, text));
    crate::cues::cues_net(g, gd, text, cls);
    let p = g.ch[g.gd[gd].ch].pos;
    let t = to_fixed2(g.time);
    let st = st_of(g);
    st.said.push(LastStandSaid { t, who: callsign, kind: String::from(kind.as_str()), text: String::from(text), at: [to_fixed2(p[0]), to_fixed2(p[2])] });
    if st.said.len() > 200 { st.said.drain(0..50); }
    st.stats.lines += 1.0;
    note(g, Some(gd), &alloc::format!("net: {}", text));
}

/// a line's words with its {names} filled, an unknown name by nothing (laststand.ts fill)
pub fn fill(tpl: &str, vars: &JsMap<String, String>) -> String {
    let mut out = String::new();
    let mut rest = tpl;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let word = after.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(after.len());
        if word > 0 && after[word..].starts_with('}') {
            if let Some(v) = vars.get(String::from(&after[..word])) { out.push_str(v); }
            rest = &after[word + 1..];
        } else {
            out.push('{');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// a line on the net, now or when the strip has room for its class; `then` happens when it is said (laststand.ts net)
pub fn net(g: &mut crate::game::Game, gd: GuardId, kind: crate::laststand::LineKind, vars: Option<&JsMap<String, String>>, then: Option<&mut dyn FnMut(&mut crate::game::Game)>, redo: Option<&mut dyn FnMut(&mut crate::game::Game) -> crate::laststand::NetRedoRet>) {
    let (tpl, text, cls, at) = line_slot(g, kind, vars);
    let t0 = g.time;
    if at > t0 {
        let (then, redo) = (seam_keep_then(then), seam_keep_redo(redo));
        st_of(g).later.push(Later { at, t0, gd, kind, text, cls, then, tpl: String::from(tpl), redo });
    } else {
        voice(g, gd, kind, &text, cls);
        if let Some(t) = then { t(g); }
    }
}

/// a line that says where its man stands: the place is asked again as the line is said (laststand.ts netAt).
/// `f` cannot be kept, so what is kept is the Later.redo that the line's kind names; every call in the TypeScript is in this part and goes by `net_at_kept`
pub fn net_at(g: &mut crate::game::Game, d: RoomfightDefId, kind: crate::laststand::LineKind, f: &mut dyn FnMut(&mut crate::game::Game, crate::laststand::Spoken) -> crate::laststand::NetAtFRet) {
    let gd = g.tab.roomfight_def[d].gd;
    let w = man_spoken(g, d);
    let a = f(g, w);
    let free = matches!(a.then, Some(LaterThen::NameAt { w: None, .. }));
    let n = a.vars.get(String::from("n")).and_then(|n| n.parse::<i32>().ok()).unwrap_or(0);
    let redo = match kind {
        LineKind::MovedAt => Some(LaterRedo::MovedAt { d }),
        LineKind::CheckLoneAt => Some(LaterRedo::CheckLone { d }),
        LineKind::AnsAt | LineKind::AnsFreeAt => Some(LaterRedo::Answer { d, free: kind == LineKind::AnsFreeAt }),
        LineKind::LastAt | LineKind::FewAt => Some(LaterRedo::Tell { d, n, free }),
        _ => None,
    };
    net_kept(g, gd, kind, Some(&a.vars), a.then, redo);
}

/// the lines waiting their turn on the strip: said by the same man if he is still fit to say it (laststand.ts tickLater)
pub fn tick_later(g: &mut crate::game::Game) {
    if st_of(g).later.is_empty() { return; }
    let list = core::mem::take(&mut st_of(g).later);
    let mut kept = Vec::new();
    for mut l in list {
        if g.time < l.at { kept.push(l); continue; }
        if !fit_gd(g, l.gd) { continue; }
        let w = crate::cues::cues_net_wait(g, l.cls);
        if w > 0.0 && g.time - l.t0 < 8.0 { l.at = g.time + w; kept.push(l); continue; }
        if let Some(r) = l.redo.take() { let a = redo_words(g, &r); l.text = fill(&l.tpl, &a.vars); l.then = a.then; }
        voice(g, l.gd, l.kind, &l.text, l.cls);
        if let Some(t) = l.then { run_then(g, t); }
    }
    st_of(g).later = kept;
}

/// this man's word on the net: a room, or none; what he said before is forgotten (laststand.ts name)
pub fn name(g: &mut crate::game::Game, d: RoomfightDefId, id_2: Option<&str>) {
    let st = st_of(g);
    match id_2 {
        Some(id) if !id.is_empty() => { st.named.set(d, String::from(id)); }
        _ => { st.named.delete(d); }
    }
    st.spots.delete(d);
    st.at.delete(d);
    st.hint = None;
}

/// named by what he said, the room or the place; kept, to be said again when it stops being true (laststand.ts nameAt)
pub fn name_at(g: &mut crate::game::Game, d: RoomfightDefId, w: Option<&crate::laststand::Spoken>) {
    name(g, d, w.and_then(|w| w.id.as_deref()));
    let w = match w { Some(w) if id_truthy(&w.id) || w.spot.is_some() => w, _ => return };
    let (p, time) = (man_pos(g, d), g.time);
    let st = st_of(g);
    if let Some(s) = &w.spot { st.spots.set(d, s.clone()); }
    let key = match &w.id { Some(id) => id.clone(), None => w.spot.as_ref().unwrap().name.clone() };
    st.at.set(d, Said { key, the: w.the.clone(), off_t: f64::NAN, still_at: v3::copy(p), still_t: time, said_t: time });
}

/// a man who is no longer where he said says where he is now, or that he has gone after the team (laststand.ts tickMoved)
pub fn tick_moved(g: &mut crate::game::Game) {
    let (moved_secs, still_secs, moved_gap) = (g.k.laststand.moved_secs, g.k.laststand.still_secs, g.k.laststand.moved_gap);
    for d in st_of(g).at.keys() {
        if !alive(g, d) { st_of(g).at.delete(d); continue; }
        let gd = g.tab.roomfight_def[d].gd;
        let p = man_pos(g, d);
        let time = g.time;
        let st = st_of(g);
        let said = st.at.get_mut(d).unwrap();
        if flat(p, said.still_at) > 0.4 { said.still_at = v3::copy(p); said.still_t = time; }
        if st.stand.as_ref().map_or(false, |s| of_stand(s, d)) { continue; }
        let w = spoken(g, p);
        let free = g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Free;
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
            net_kept(g, gd, LineKind::MovedFree, Some(&vars_of(&[("the", the.as_str())])), Some(LaterThen::MovedFree { d }), None);
        } else {
            net_at_kept(g, d, LineKind::MovedAt, LaterRedo::MovedAt { d });
        }
    }
}

/// where the team is, as far as the survivors know; never the team's true positions (laststand.ts teamKnown)
pub fn team_known(g: &mut crate::game::Game, near: V3) -> crate::laststand::TeamKnown {
    let known_secs = g.k.laststand.known_secs;
    let defs = g.fight.defs.clone();
    let mut pts: Vec<V3> = Vec::new();
    let mut last: Option<V3> = None;
    let mut lt = f64::NEG_INFINITY;
    for &d in &defs {
        if !alive(g, d) { continue; }
        let gd = g.tab.roomfight_def[d].gd;
        if let Some(lk) = g.gd[gd].last_known {
            let is_pt = g.tab.roomfight_def[d].fix_is_pt && lk == g.tab.roomfight_def[d].pt;   // (gd.lastKnown === D.pt: roomfight's rule, the flag alone can be stale)
            if !is_pt && g.gd[gd].last_seen_t > -50.0 {
                let seen = g.gd[gd].last_seen_t;
                if g.time - seen < known_secs { pts.push(lk); }
                if seen > lt { lt = seen; last = Some(lk); }
            }
        }
        if let Some(at) = g.tab.roomfight_def[d].noise_at {
            let heard = g.tab.roomfight_def[d].noise_t;
            if g.time - heard < known_secs { pts.push(at); }
            if heard > lt { lt = heard; last = Some(at); }
        }
    }
    if last.is_none() {
        let mut bd = f64::INFINITY;
        for &d in &defs {
            let gd = g.tab.roomfight_def[d].gd;
            if g.gd[gd].state == crate::game::GuardState::Dead {
                let p = man_pos(g, d);
                let dist = flat(p, near);
                if dist < bd { bd = dist; last = Some(p); }
            }
        }
    }
    TeamKnown { pts, last: v3::copy(last.unwrap_or(near)) }
}

/// a free spot of a room a couple of strides from its anchor, off its doorways and off the spots given out (laststand.ts spotIn)
pub fn spot_in(g: &mut crate::game::Game, r: RoomfightRoomId, p0: V3, taken: &[V3]) -> V3 {
    let defs = g.fight.defs.clone();
    for rad in [1.8, 2.6, 1.2, 3.4] {
        for a in 0..8 {
            let turn = (a as f64) * core::f64::consts::PI / 4.0;
            let p: V3 = [p0[0] + m::cos(turn) * rad, 0.0, p0[2] + m::sin(turn) * rad];
            if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || !room_has(g, r, p) { continue; }
            if g.tab.roomfight_room[r].entries.iter().any(|e| flat(e.at, p) < 1.5) { continue; }
            if defs.iter().any(|&x| alive(g, x) && flat(man_pos(g, x), p) < 0.9) || taken.iter().any(|&q| flat(q, p) < 0.9) { continue; }
            if crate::scene_collision::segment_blocked(g, [p0[0], 1.2, p0[2]], [p[0], 1.2, p[2]], None, None, None) { continue; }
            return p;
        }
    }
    [p0[0], 0.0, p0[2]]
}

/// his way to `spot`: its length, or none: no way, too long, or past where the team is known to be (laststand.ts wayTo)
pub fn way_to(g: &mut crate::game::Game, d: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> {
    let from = man_pos(g, d);
    let path = crate::scene_collision::nav_grid_find_path(g, from, spot, None, None)?;
    match path.last() { Some(&end) if !(flat(end, spot) > 0.8) => {} _ => return None }
    let mut len = 0.0;
    let mut q = from;
    let mut bad = false;
    let clear = g.k.laststand.clear_m;
    for &w in &path {
        let dist = flat(q, w);
        let n = m::max(1.0, m::ceil(dist / 0.5));
        if clear > 0.0 {
            let mut i = 1.0;
            while i <= n && !bad {
                let p: V3 = [q[0] + (w[0] - q[0]) * i / n, 0.0, q[2] + (w[2] - q[2]) * i / n];
                if known.iter().any(|&f| flat(f, p) < clear) { bad = true; }
                i += 1.0;
            }
        }
        len += dist;
        q = w;
    }
    if bad || len > g.k.laststand.max_m { None } else { Some(len) }
}

/// may this man walk to that spot (laststand.ts canCome)
pub fn can_come(g: &mut crate::game::Game, x: RoomfightDefId, spot: V3, known: &[V3]) -> Option<f64> {
    if !movable(g, x) { return None; }
    let gd = g.tab.roomfight_def[x].gd;
    if crate::enemysquads::squad_stays_in(g, gd) && !crate::enemysquads::squad_holds(g, gd, spot) { return None; }
    way_to(g, x, spot, known)
}

/// a room the stand could be made in, as one of the men holding it stands (laststand.ts standRoom)
pub fn stand_room(g: &mut crate::game::Game, d: RoomfightDefId, known: &[V3]) -> Option<crate::laststand::StandRoom> {
    if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Hold { return None; }
    let room = g.tab.roomfight_def[d].room?;
    let p = man_pos(g, d);
    if g.tab.roomfight_room[room].entries.is_empty() || !in_room(g, Some(room), p) { return None; }
    let id = where_(g, p).id.filter(|s| !s.is_empty())?;
    if crate::digin::dig_in_entered(g, room) || known.iter().any(|&q| in_room(g, Some(room), q)) { return None; }
    Some(StandRoom { room, id })
}

/// or the post one of them was staged on, standing empty (laststand.ts homeRoom)
pub fn home_room(g: &mut crate::game::Game, d: RoomfightDefId, known: &[V3]) -> Option<crate::laststand::HomeRoom> {
    let gd = g.tab.roomfight_def[d].gd;
    let id = g.m.laststand.states.as_ref().and_then(|st| st.post.get(gd).cloned()).filter(|s| !s.is_empty())?;
    let def = g.room_clear.rooms.iter().find(|r| r.def.id == id).map(|r| r.def.clone())?;
    let c: V3 = [(def.bounds.x0 + def.bounds.x1) / 2.0, 0.0, (def.bounds.z0 + def.bounds.z1) / 2.0];
    let anchor = crate::roomfight::room_fight_free_near(g, c).unwrap_or(c);
    let room = crate::roomfight::room_fight_room_at(g, anchor)?;
    if g.tab.roomfight_room[room].entries.is_empty() || where_(g, anchor).id.as_deref() != Some(def.id.as_str()) { return None; }
    if crate::digin::dig_in_entered(g, room) || known.iter().any(|&q| in_room(g, Some(room), q)) { return None; }
    Some(HomeRoom { room, id: def.id, anchor })
}

/// a room's name as a line says it (laststand.ts theOf)
pub fn the_of(g: &mut crate::game::Game, id_2: &str) -> String {
    match g.room_clear.rooms.iter().find(|r| r.def.id == id_2) {
        Some(r) => the_word(&r.def.name),
        None => the_word(id_2),
    }
}

/// which room: most men in or able to reach it, then most doors wedged, most men there, farthest from the team, least walking (laststand.ts choose)
pub fn choose(g: &mut crate::game::Game, men: &[RoomfightDefId], known: &[V3], min: Option<f64>) -> Option<crate::laststand::Plan> {
    let min = min.unwrap_or(2.0);
    let mut cands: JsMap<RoomfightRoomId, (String, V3, Vec<RoomfightDefId>)> = JsMap::new();
    for &d in men {
        let r = match stand_room(g, d, known) { Some(r) => r, None => continue };
        if let Some(c) = cands.get_mut(r.room) { c.2.push(d); } else { let p = man_pos(g, d); cands.set(r.room, (r.id, p, vec![d])); }
    }
    for &d in men {
        if let Some(r) = home_room(g, d, known) {
            if !cands.has(r.room) { cands.set(r.room, (r.id, r.anchor, Vec::new())); }
        }
    }
    let mut best: Option<Plan> = None;
    let mut bs: [f64; 5] = [0.0; 5];
    for room in cands.keys() {
        let (id, anchor, inside) = cands.get(room).unwrap().clone();
        let mut come: Vec<PlanComeItem> = Vec::new();
        let mut taken: Vec<V3> = Vec::new();
        let mut walk = 0.0;
        for &x in men {
            if inside.contains(&x) { continue; }
            let spot = spot_in(g, room, anchor, &taken);
            let l = match can_come(g, x, spot, known) { Some(l) => l, None => continue };
            come.push(PlanComeItem { d: x, spot, l });
            taken.push(spot);
            walk += l;
        }
        let far = if known.is_empty() { 0.0 } else {
            let dists: Vec<f64> = known.iter().map(|&p| flat(p, anchor)).collect();
            m::min_n(&dists)
        };
        let s: [f64; 5] = [(inside.len() + come.len()) as f64, crate::digin::dig_in_wedged(g, room).len() as f64, inside.len() as f64, m::round(far), -walk];
        let mut better = best.is_none();
        if best.is_some() {
            for i in 0..s.len() {
                if s[i] != bs[i] { better = s[i] > bs[i]; break; }
            }
        }
        if better { best = Some(Plan { room, id, anchor, inside, come }); bs = s; }
    }
    if best.is_some() && bs[0] >= min { best } else { None }
}

/// one of the fit asks the net who is still up; every other man up gets his beat to answer (laststand.ts startCheck)
pub fn start_check(g: &mut crate::game::Game) {
    let stand = st_of(g).stand.clone();
    let men = up_defs(g);
    let can: Vec<RoomfightDefId> = men.iter().copied().filter(|&d| calm(g, d)).collect();
    let in_stand = stand.as_ref().and_then(|s| can.iter().copied().find(|&d| s.in_.has(d)));
    let caller = in_stand
        .or_else(|| can.iter().copied().find(|&d| g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Hold))
        .or_else(|| can.first().copied());
    let time = g.time;
    let st = st_of(g);
    let caller = match caller { Some(c) => c, None => { st.next_at = time + 1.0; return; } };
    let first = st.stats.checks == 0.0;
    let lone_now = st.lone_said && st.known.len() <= 1;
    st.stats.checks += 1.0;
    let checks = st.stats.checks;
    let caller_gd = g.tab.roomfight_def[caller].gd;
    // (the room's men in the room with him: only with a room, and him in it)
    let caller_in_room = stand.as_ref().map_or(false, |s| s.room.is_some() && s.in_.has(caller));
    match stand.as_ref() {
        Some(s) if caller_in_room && s.in_.len() + s.movers.len() > 1 => {
            net_kept(g, caller_gd, LineKind::CheckStand, Some(&vars_of(&[("the", s.the.as_str())])), None, None);
        }
        _ if lone_now => net_at_kept(g, caller, LineKind::CheckLoneAt, LaterRedo::CheckLone { d: caller }),
        _ => net_kept(g, caller_gd, if first { LineKind::Check1 } else { LineKind::CheckN }, None, None, None),
    }
    let answer = g.k.laststand.answer;
    let between = g.k.laststand.between;
    let listen = g.k.laststand.listen;
    let mut at = g.time + pick(g, answer);
    let mut due: Vec<CheckDueItem> = Vec::new();
    for &d in &men {
        if d == caller || (caller_in_room && stand.as_ref().map_or(false, |s| s.in_.has(d))) { continue; }
        due.push(CheckDueItem { d, at });
        at += pick(g, between);
    }
    let close_at = (match due.last() { Some(x) => x.at, None => g.time + 1.2 }) + pick(g, listen);
    let t = g.time;
    st_of(g).check = Some(Check { caller, t, close_at, due, got: Vec::new() });
    note(g, Some(caller_gd), &alloc::format!("asks the net who is up (check {}, {} up)", checks, men.len()));
}

/// the answers on their beats; at its close what the survivors know of each other (laststand.ts tickCheck). C is st.check, read by its path
pub fn tick_check(g: &mut crate::game::Game) {
    let caller = check_of(g).caller;
    let caller_gd = g.tab.roomfight_def[caller].gd;
    if !alive(g, caller) {
        let time = g.time;
        let st = st_of(g);
        st.check = None;
        st.next_at = time + 4.0;
        note(g, Some(caller_gd), "down mid-check");
        return;
    }
    for i in 0..check_of(g).due.len() {
        let due = check_of(g).due[i];
        if g.time < due.at || due.at < 0.0 { continue; }
        check_of(g).due[i].at = -1.0;
        let d = due.d;
        if !calm(g, d) { continue; }
        let free = g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Free;
        net_at_kept(g, d, if free { LineKind::AnsFreeAt } else { LineKind::AnsAt }, LaterRedo::Answer { d, free });
        check_of(g).got.push(d);
        st_of(g).stats.answers += 1.0;
    }
    if g.time < check_of(g).close_at { return; }
    let (time, again, every) = (g.time, g.k.laststand.again, g.k.laststand.every);
    let st = st_of(g);
    let got = st.check.take().unwrap().got;
    let stand = st.stand.clone();
    st.next_at = time + if stand.as_ref().map_or(false, |s| s.set_said) { again } else { every };
    let with_: Vec<RoomfightDefId> = match stand.as_ref() {
        Some(s) if s.room.is_some() && s.in_.has(caller) => { let all = s.in_.keys(); all.into_iter().filter(|&d| alive(g, d)).collect() }
        _ => Vec::new(),
    };
    let mut heard: Vec<RoomfightDefId> = Vec::new();
    if alive(g, caller) { heard.push(caller); }
    for &d in &got { if alive(g, d) { heard.push(d); } }
    let mut known = JsSet::new();
    for &d in &heard { known.add(d); }
    for &d in &with_ { known.add(d); }
    let up = up_defs(g).len() as f64;
    let silent: Vec<GuardId> = g.fight.defs.iter().filter(|&&d| !known.has(d)).map(|&d| g.tab.roomfight_def[d].gd).collect();
    let heard_gds: Vec<GuardId> = heard.iter().map(|&d| g.tab.roomfight_def[d].gd).collect();
    let st = st_of(g);
    st.known = known.clone();
    st.stats.silent += up - known.len() as f64;
    crate::netquiet::net_quiet_closed(g, caller_gd, &silent, &heard_gds);
    let st = st_of(g);
    let mut names = st.named.keys();
    names.extend(st.spots.keys());
    for d in names {
        let held = stand.as_ref().map_or(false, |s| of_stand(s, d));
        if !known.has(d) && !held { name(g, d, None); }
    }
    let signs: Vec<String> = known.keys().iter().map(|&d| g.gd[g.tab.roomfight_def[d].gd].callsign.clone()).collect();
    note(g, Some(caller_gd), &alloc::format!("check closes: {} known up ({})", known.len(), signs.join(", ")));
    if !calm(g, caller) {
        let time = g.time;
        let st = st_of(g);
        st.next_at = m::min(st.next_at, time + 5.0);
        return;
    }
    if g.k.laststand.regroup { decide(g, caller, &heard); } else { tell_only(g, caller); }
}

/// nobody answers him: true the first time (laststand.ts lone)
pub fn lone(st: &mut crate::laststand::State) -> bool {
    if st.lone_said { return false; }
    st.lone_said = true;
    st.stats.last_man += 1.0;
    true
}

/// call out only: the one who asked says what the count is, and where he is (laststand.ts tellOnly)
pub fn tell_only(g: &mut crate::game::Game, c_2: RoomfightDefId) {
    let free = g.tab.roomfight_def[c_2].mode == crate::roomfight::DefMode::Free;
    let st = st_of(g);
    let n = st.known.len() as i32;
    let redo = LaterRedo::Tell { d: c_2, n, free };
    if n <= 1 {
        if lone(st) { net_at_kept(g, c_2, LineKind::LastAt, redo); }
        return;
    }
    st.lone_said = false;
    if n as f64 != st.few_n {
        st.few_n = n as f64;
        net_at_kept(g, c_2, LineKind::FewAt, redo);
    }
}

/// a new stand (laststand.ts newStand)
pub fn new_stand(g: &mut crate::game::Game, room: Option<RoomfightRoomId>, id_2: Option<&str>, the: &str, anchor: V3) -> crate::laststand::Stand {
    Stand {
        room, id: id_2.map(String::from), the: String::from(the), anchor: v3::copy(anchor), t: g.time,
        in_: JsSet::new(), out: JsSet::new(), movers: Vec::new(),
        set_said: false, set_t: f64::NAN, thump_at: f64::NAN, look_t: g.time, orders: JsMap::new(), homed: JsSet::new(),
    }
}

/// where a man coming in is put beside: a man of theirs in the room, else the spot the room was picked by (laststand.ts anchorOf)
pub fn anchor_of(g: &mut crate::game::Game, s: &crate::laststand::Stand) -> V3 {
    for d in s.in_.keys() {
        if alive(g, d) { return man_pos(g, d); }
    }
    s.anchor
}

/// is he of this stand (laststand.ts member)
pub fn member(g: &mut crate::game::Game, s: &crate::laststand::Stand, d: RoomfightDefId) -> bool { of_stand(s, d) }

/// told to hold where he is: he says so, once, with where (laststand.ts holdsOwn)
pub fn holds_own(g: &mut crate::game::Game, s: &mut crate::laststand::Stand, d: RoomfightDefId, say: Option<crate::laststand::LineKind>) {
    s.out.add(d);
    let w = man_spoken(g, d);
    let say = match say { Some(k) => k, None => return };
    if calm(g, d) && !st_of(g).told.has(d) {
        st_of(g).told.add(d);
        let gd = g.tab.roomfight_def[d].gd;
        let vars = vars_of(&[("at", w.at.as_str()), ("the", w.the.as_str())]);
        net_kept(g, gd, say, Some(&vars), Some(LaterThen::NameAt { d, w: Some(w) }), None);
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
    fn to_fixed2_is_the_languages() {
        // (x, the bits of `+x.toFixed(2)` under bun)
        for (x, bits) in TO_FIXED {
            assert_eq!(to_fixed2(*x).to_bits(), *bits, "{}", x);
        }
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
        let mut st = State::zeroed();
        assert!(lone(&mut st));
        assert!(!lone(&mut st));
        assert_eq!(st.stats.last_man, 1.0);
    }

    #[test]
    fn flat_is_on_the_floor() {
        assert_eq!(flat([3.0, 9.0, 0.0], [0.0, -2.0, 4.0]), 5.0);
    }

    const TO_FIXED: &[(f64, u64)] = &[
        (f64::from_bits(0x3fc0000000000000), 0x3fc0a3d70a3d70a4),
        (f64::from_bits(0x3fd8000000000000), 0x3fd851eb851eb852),
        (f64::from_bits(0x3ffa000000000000), 0x3ffa147ae147ae14),
        (f64::from_bits(0xbfc0000000000000), 0xbfc0a3d70a3d70a4),
        (f64::from_bits(0xbffa000000000000), 0xbffa147ae147ae14),
        (f64::from_bits(0x3ff0147ae147ae14), 0x3ff0000000000000),
        (f64::from_bits(0x4005666666666666), 0x40055c28f5c28f5c),
        (f64::from_bits(0xbf50624dd2f1a9fc), 0x8000000000000000),
        (f64::from_bits(0x3f50624dd2f1a9fc), 0x0),
        (f64::from_bits(0x0), 0x0),
        (f64::from_bits(0x8000000000000000), 0x0),
        (f64::from_bits(0x40934a428f5c28f6), 0x40934a47ae147ae1),
        (f64::from_bits(0x4031555554c62af2), 0x4031547ae147ae14),
        (f64::from_bits(0xc00921f9f01b866e), 0xc0091eb851eb851f),
        (f64::from_bits(0x40593fae147ae148), 0x4059400000000000),
        (f64::from_bits(0x3fe0000000000000), 0x3fe0000000000000),
        (f64::from_bits(0x444b1ae4d6e2ef50), 0x444b1ae4d6e2ef50),
        (f64::from_bits(0x3e7ad7f29abcaf48), 0x0),
        (f64::from_bits(0x4009000000000000), 0x40090a3d70a3d70a),
        (f64::from_bits(0x404f700000000000), 0x404f70a3d70a3d71),
    ];
}
