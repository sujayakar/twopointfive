//! WHAT HE CAN DO WITH HIS HANDS: a marker on each thing near him; under the things in reach their rows, a row a verb, which is what Space
//! does; and which row is lit. A thing of his own hands is a file here; anything else adds itself (`Offers::add`) from where its rule stands.
use super::*;
use crate::game::{Deed, Game, Interactable, InteractableKind, Verb};
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec, vec::Vec};

mod doors;
mod men;
mod things;

/// This tick's markers, and the rows of those in reach, each with its marker's place in the list.
#[derive(Default)]
pub struct Offers { list: Vec<Interactable>, things: Vec<(usize, Vec<Interactable>)> }
impl Offers {
    /// A thing, and what can be done with it: one with rows is worded by them.
    pub fn add(&mut self, mut it: Interactable, rows: Vec<Interactable>) {
        if !rows.is_empty() {
            it.line2 = joined(&rows);
            self.things.push((self.list.len(), rows));
        }
        self.list.push(it);
    }
    /// The door's marker, or a plain one if it has none.
    fn door(&self, g: &Game, d: DoorId) -> Interactable {
        if let Some(it) = self.list.iter().find(|it| it.door == Some(d)) { return it.clone(); }
        Interactable { door: Some(d), ..mark(InteractableKind::Door, crate::doors::door_pos(g, d), "door", "", true) }
    }
}

/// A marker on a thing: what it is, and what stands under that.
pub fn mark(kind: InteractableKind, pos: V3, line1: impl Into<String>, line2: impl Into<String>, in_reach: bool) -> Interactable {
    Interactable { kind, pos, line1: line1.into(), line2: line2.into(), in_reach, ..Default::default() }
}
/// A row of a thing: what Space does with it, and a word on what its other keys do.
pub fn row(it: &Interactable, verb: Verb, text: &str, hints: &str) -> Interactable {
    let line2 = if hints.is_empty() { format!("Space  {text}") } else { format!("Space  {text}  ·  {hints}") };
    Interactable { verb: Some(verb), running: Some(false), progress: None, line2, ..it.clone() }
}
/// The row of what he is in the middle of: how far along it is, and how it stops.
pub fn running(it: &Interactable, verb: Verb, text: &str, progress: Option<f64>) -> Interactable {
    Interactable { verb: Some(verb), running: Some(true), progress: progress.map(|x| v3::clamp(x, 0.0, 1.0)), in_reach: true, line2: String::from(text), ..it.clone() }
}
/// A line of a thing's that Space does nothing with.
pub fn said(it: &Interactable, text: &str) -> Interactable {
    Interactable { verb: None, running: Some(false), progress: None, in_reach: true, line2: String::from(text), ..it.clone() }
}
fn joined(rows: &[Interactable]) -> String { rows.iter().map(|r| r.line2.as_str()).collect::<Vec<_>>().join("  ·  ") }
/// how far along, as a whole percentage that never says it is done
fn pct(x: f64) -> String { crate::js::num_str(m::min(99.0, m::round(x * 100.0))) }

/// Him, as the rows ask: where he is and what his hands and feet are at.
#[derive(Clone, Copy)]
struct Him { pl: PlayerId, p: V3, busy: bool, sprints: bool, low: bool, drags: bool }
impl Him {
    fn of(g: &Game) -> Him {
        let pl = g.player;
        let p = &g.pl[pl];
        Him { pl, p: g.ch[p.ch].pos, busy: p.deed.is_some(), sprints: p.sprinting, low: p.crouch, drags: p.tow.is_some() }
    }
    /// He is on his feet, or has the room to be.
    fn stands(&self, g: &Game) -> bool { !self.low || can_stand(g) }
}

/// The markers and the rows of this tick, and the one that is lit.
pub(super) fn build_interactables(g: &mut Game) {
    let h = Him::of(g);
    let mut o = Offers::default();
    things::lamps(g, &mut o);
    let rows = if crate::condition::down(g, g.pl[h.pl].ch) { Vec::new() }
        else if g.pl[h.pl].holding().is_some() { men::held(g, &mut o, h) }
        else { around(g, &mut o, h) };
    g.sets[g.seat].interactables = o.list;
    keep_lit(g, &rows);
    g.sets[g.seat].nearby = rows;
    pick_hover(g);
    crate::close::close_tick(g);
}

/// With nobody in his arm: everything round him, and the rows of what he is in the middle of or could begin.
fn around(g: &mut Game, o: &mut Offers, h: Him) -> Vec<Interactable> {
    for i in 0..g.doors.list.len() { let d = g.doors.list[i]; doors::door(g, o, h, d); }
    let strikes = g.pl[h.pl].takedown().is_some();
    if !strikes {
        for i in 0..g.guards.len() { let gd = g.guards[i]; men::staff(g, o, h, gd); }
        for gd in crate::civilians::civilian_bodies(g) { men::civilian(g, o, h, gd); }
    }
    men::bodies(g, o, h);
    things::errand(g, o, h, strikes);
    things::breaker(g, o, h);
    crate::wallcharge::wall_charge_rows(g, h.p, h.busy, o);
    crate::rescueend::rescue_end_rows(g, h.p, h.busy, o);
    crate::stickycam::sticky_rows(g, h.pl, h.busy, o);
    crate::lightswitch::switch_rows(g, h.pl, h.busy, o);
    under_way(g, o, h).unwrap_or_else(|| listed(g, o, h))
}

/// The one row of what he is in the middle of. None: nothing that has the rows to itself.
fn under_way(g: &Game, o: &Offers, h: Him) -> Option<Vec<Interactable>> {
    let it = match g.pl[h.pl].deed.as_ref()? {
        &Deed::Pick(d) => {
            let pick = g.dr[d].leaf.pick;
            running(&o.door(g, d), Verb::Pick, &format!("picking… {}%  ·  Space or a step breaks off (the lock keeps it a moment)", pct(pick)), Some(pick))
        }
        Deed::Wand(w) => running(&o.door(g, w.door), Verb::Wand, if w.live { "looking under it — the cursor turns the lens  ·  Space or a step pulls it out" } else { "feeding the wand under…  ·  Space or a step pulls it out" }, None),
        Deed::Code(c) => {
            let name = match crate::devices::devices_pad_of(g, c.door) { Some(k) => crate::devices::pad_door_name(g, &k), None => String::from("door") };
            running(&o.door(g, c.door), Verb::Code, &format!("keying the {name} code… — Space or a step breaks off"), Some(c.t / crate::devices::CODE_SECS))
        }
        Deed::Kick(k) => said(&o.door(g, k.door), match k.tool.as_ref().map(|t| t.kind) { Some(crate::kit::BigTool::Hammer) => "hammering it…", Some(_) => "on the lock…", None => "kicking it in…" }),
        Deed::Ease(e) => running(&o.door(g, e.door), Verb::Ease, "easing it open… — Space or a step stops it where it is", Some(m::abs(g.dr[e.door].leaf.angle) / EASE_MAX)),
        Deed::Take(t) => {
            let part = t / crate::consts::DRIVE_SECS;
            let at = o.list.iter().find(|it| it.kind == InteractableKind::Objective);
            return Some(at.map(|it| running(it, Verb::Drive, &format!("pulling… {}% — stay on it (Space or a step lets go)", pct(part)), Some(part))).into_iter().collect());
        }
        Deed::Charge(c) => running(&o.door(g, c.door), Verb::Charge, "setting the charge… — Space or a step breaks it off", Some(crate::doortools::door_tools_sam_progress(g, &crate::seats::here(g)))),
        Deed::Wall(_) => crate::wallcharge::wall_charge_running_row(g),
        Deed::Takedown(_) => return Some(Vec::new()),
        Deed::Breaker(_) | Deed::Door(_) | Deed::Hold(_) => return None,
    };
    Some(vec![it])
}

/// The rows listed: the things in reach by what matters most, as many as the list holds, and past those a row each for a charge to set, a
/// big tool on a door not locked, a stuck camera and a charge of his to blow.
fn listed(g: &mut Game, o: &mut Offers, h: Him) -> Vec<Interactable> {
    let mut rows = in_order(g, o, h.p);
    rows.sort_by_key(|r| r.verb == Some(Verb::Wand));   // (the look under a door gives way to every other thing's rows)
    let hint = rows.iter().find(|r| r.verb.is_none()).cloned();   // (a man he has only the mouse for: no verb, so it stands under what Space does)
    let door = |r: &Interactable| r.kind == InteractableKind::Door;
    let charge = |r: &Interactable| door(r) && r.verb == Some(Verb::Charge);
    let past = |r: &Interactable| door(r) && r.past == Some(true);
    let cam = |r: &Interactable| r.kind == InteractableKind::Cam;
    let own = [rows.iter().find(|r| charge(r)), rows.iter().find(|r| past(r)), rows.iter().find(|r| cam(r))];
    let blow = if h.busy || h.sprints || h.drags { None } else { crate::ownblow::own_blow_row(g) };
    let mut out: Vec<Interactable> = rows.iter().filter(|r| r.verb.is_some() && !charge(r) && !past(r) && !cam(r)).take(MAX_NEAR - hint.is_some() as usize).cloned().collect();
    // (his own charge to blow: under the things in reach, ahead of the rows past the cap; with nothing in reach, after those)
    let under = !out.is_empty();
    if under { out.extend(blow.clone()); }
    out.extend(own.into_iter().flatten().cloned());
    if !under { out.extend(blow); }
    out.extend(hint);
    out
}

/// The rows of the things in reach, the thing that matters most first. A tutorial offers only the rows its steps teach: a thing left with none is marked.
fn in_order(g: &Game, o: &mut Offers, p: V3) -> Vec<Interactable> {
    let Offers { list, things } = o;
    let rank = interact_priority();
    let key = |at: usize| { let it = &list[at]; ([!it.in_reach as i32 as f64, *rank.get(String::from(it.kind.as_str())).unwrap()], thing_dist(g, it, p)) };
    js::sort_by(things, |a, b| { let (a, b) = (key(a.0), key(b.0)); js::or(js::or(a.0[0] - b.0[0], a.0[1] - b.0[1]), a.1 - b.1) });
    let mut rows = Vec::new();
    for (at, its) in things.iter() {
        let before = rows.len();
        rows.extend(its.iter().filter(|r| r.verb.is_none_or(|v| crate::rails::rails_offers(g, v.as_str()))).cloned());
        if rows.len() == before { list[*at].framed = Some(true); }
    }
    rows
}

/// The light stays on its row while that row is listed, else goes to the top: but never by itself from what he was in the middle of, nor
/// onto his own charge to blow, unless the ring is open.
fn keep_lit(g: &mut Game, rows: &[Interactable]) {
    let key = rows.iter().map(|r| row_key(g, r)).collect::<Vec<_>>().join("|");
    if key == g.sets[g.seat].hand.near_key { return; }
    let lit = g.sets[g.seat].hover.as_ref().filter(|r| r.running != Some(true)).map(|r| row_key(g, r));
    let at = lit.and_then(|lit| rows.iter().position(|r| row_key(g, r) == lit));
    let hand = &mut g.sets[g.seat].hand;
    hand.near_key = key;
    hand.near_sel = match at {
        Some(k) if !(rows[k].kind == InteractableKind::Charge && k > 0 && !hand.sel.open) => k as i32,
        _ => 0,
    };
}

/// the man or the door a row is about, by the name it goes by ('' = neither)
pub(in crate::sam) fn who_of<'g>(g: &'g Game, r: &Interactable) -> &'g str {
    match (r.door, r.guard) {
        (Some(d), _) => g.dr[d].def.name.as_str(),
        (None, Some(gd)) => g.gd[gd].callsign.as_str(),
        (None, None) => "",
    }
}

/// which row it is, from tick to tick: the thing and the verb
fn row_key(g: &Game, r: &Interactable) -> String {
    let who = who_of(g, r);
    match r.verb {
        Some(v) => format!("{}:{}:{}", r.kind.as_str(), who, v.as_str()),
        None => format!("{}:{}:~{}", r.kind.as_str(), who, r.line2),
    }
}

/// Game.hover ← the highlighted row, Game.useDoor ← its door if it is a door row in reach
pub fn pick_hover(g: &mut Game) {
    let n = g.sets[g.seat].nearby.len() as i32;
    let sel = g.sets[g.seat].hand.near_sel;
    g.sets[g.seat].hand.near_sel = if n > 0 { ((sel % n) + n) % n } else { 0 };
    let best = g.sets[g.seat].nearby.get(g.sets[g.seat].hand.near_sel as usize).cloned();
    g.sets[g.seat].hover = best;
}

/// how far a thing is from him, as its reach is measured: a door by its leaf or its doorway's centre, whichever is nearer
pub(in crate::sam) fn thing_dist(g: &Game, it: &Interactable, p: V3) -> f64 {
    let d = v3::dist_xz(it.pos, p);
    match it.door {
        Some(door) => m::min(d, v3::dist_xz(g.dr[door].frame_centre, p)),
        None => d,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn door_marker() -> Interactable {
        Interactable { progress: Some(0.5), ..mark(InteractableKind::Door, [1.0, 2.0, 3.0], "door · shut", "x", false) }
    }

    #[test]
    fn a_row_words_its_verb_and_hint_and_clears_the_progress() {
        let r = row(&door_marker(), Verb::Kick, "kick it in — LOUD", "or sprint into it");
        assert_eq!(r.line2, "Space  kick it in — LOUD  ·  or sprint into it");
        assert_eq!(r.running, Some(false));
        assert_eq!(r.progress, None);
        assert_eq!(row(&door_marker(), Verb::Push, "push it open", "").line2, "Space  push it open");
    }

    #[test]
    fn a_running_row_clamps_its_progress_and_is_in_reach() {
        let r = running(&door_marker(), Verb::Ease, "easing", Some(1.7));
        assert_eq!(r.progress, Some(1.0));
        assert!(r.in_reach);
        assert_eq!(r.running, Some(true));
        assert_eq!(running(&door_marker(), Verb::Ease, "easing", None).progress, None);
    }

    #[test]
    fn a_thing_with_verbs_is_listed_and_named_by_its_place() {
        let mut o = Offers::default();
        let it = door_marker();
        let verbs = vec![row(&it, Verb::Push, "push it open", ""), row(&it, Verb::Ease, "ease it open — silent", "")];
        o.add(it.clone(), verbs);
        o.add(it, Vec::new());
        assert_eq!(o.list.len(), 2);
        assert_eq!(o.things.len(), 1);
        assert_eq!(o.things[0].0, 0);
        assert_eq!(o.list[0].line2, "Space  push it open  ·  Space  ease it open — silent");
        assert_eq!(o.list[1].line2, "x");
    }

    #[test]
    fn a_percentage_is_whole_and_stops_short_of_a_hundred() {
        assert_eq!(pct(0.999), "99");
        assert_eq!(pct(0.256), "26");
    }
}
