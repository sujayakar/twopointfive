//! A MAN'S ROWS: the one in his arm, one he could take hold of, one on the floor.
use super::*;
use crate::character::{Bone, HoldPhase, HoldVariant};
use crate::civilians::CivState;

/// A man in his arm: HIS rows are the only rows, and a door near is marked by what walking him into it does.
pub(super) fn held(g: &mut Game, o: &mut Offers, h: Him) -> Vec<Interactable> {
    let hold = g.pl[h.pl].holding().cloned().unwrap();
    let gd = hold.g;
    let gun = hold.variant == HoldVariant::Gun;
    let c = g.gd[gd].ch;
    let head = g.ch[c].bones.get(Bone::Head).copied().unwrap_or(v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]));
    let name = &g.gd[gd].callsign;
    let line1 = match g.gd[gd].civ {
        Some(cv) if gun => format!("{name} · the {} · your pistol at {} head", g.cv[cv].label, head_of(g, cv)),
        Some(cv) => format!("{name} · the {} · in your arm", g.cv[cv].label),
        None => format!("{name} · {}", if gun { "your pistol at his head" } else { "in your arm" }),
    };
    let it = Interactable { guard: Some(gd), ..mark(InteractableKind::Held, [head[0], head[1] + 0.3, head[2]], line1, "", true) };
    let rows = held_rows(g, &hold, &it);
    o.list.push(Interactable { line2: joined(&rows), progress: rows.first().and_then(|r| r.progress), ..it });
    for &d in &g.doors.list { held_at_door(g, o, h, d); }
    rows
}
/// a civilian's word for whose head the pistol is at
fn head_of(g: &Game, cv: CivId) -> &str { if g.cv[cv].she == "he" { "his" } else { &g.cv[cv].her } }

fn held_rows(g: &mut Game, hold: &crate::player::Hold, it: &Interactable) -> Vec<Interactable> {
    let k = g.k.hold.secs;
    let gun = hold.variant == HoldVariant::Gun;
    let civ = g.gd[hold.g].civ;
    match hold.phase {
        HoldPhase::Grab => vec![said(it, &format!("getting hold of {}…", civ.map_or("him", |cv| &g.cv[cv].her)))],
        HoldPhase::Held if hold.swap_t < k.swap => vec![said(it, if gun { "the pistol coming up to his head…" } else { "holstering it to get both hands on him again…" })],
        HoldPhase::Choke => vec![choking(it, hold.t / k.choke)],
        HoldPhase::Whip => vec![whipping(it, hold.t / k.whip)],
        HoldPhase::Held if hold.wind_t >= 0.0 => vec![if gun { whipping(it, 0.0) } else { choking(it, 0.0) }],
        HoldPhase::Held => match civ {
            Some(cv) if gun => civilian_at_gunpoint(g, cv, it),
            Some(cv) => civilian_held(g, hold, cv, it),
            None if gun => at_gunpoint(g, it),
            None => man_held(g, hold, it),
        },
    }
}
fn choking(it: &Interactable, part: f64) -> Interactable { running(it, Verb::Choke, "choking him out… — Space or a step and he breathes again", Some(part)) }
fn whipping(it: &Interactable, part: f64) -> Interactable { running(it, Verb::Whip, "pistol-whip… — Space or a step checks it", Some(part)) }

fn civilian_at_gunpoint(g: &Game, cv: CivId, it: &Interactable) -> Vec<Interactable> {
    let her = &g.cv[cv].her;
    vec![row(it, Verb::Comply, &format!("“stay down, stay quiet” — lets {her} go, cowering"), ""),
        row(it, Verb::Whip, &format!("pistol-whip {her} — knocks {her} out, but a CIVILIAN costs −10 on the card"), &format!("E  holster — to question {her}"))]
}
fn civilian_held(g: &Game, hold: &crate::player::Hold, cv: CivId, it: &Interactable) -> Vec<Interactable> {
    let tk = held_talk(g, &crate::seats::here(g), hold);
    let c = &g.cv[cv];
    let (she, her) = (&c.she, &c.her);
    let stage = g.tab.game_talk[g.gd[hold.g].talk].stage;
    let told = crate::dialogue::civ_lines().intel.get(String::from(c.def.kind.as_str())).map_or(0, |l| l.len());
    let ask = if tk.asking { format!("you're asking — {she} answers in a beat") }
        else if tk.sam_up { String::from("once you're done talking") }
        else if !tk.ready { format!("{she}'s answering — tap to cut it short") }
        else if stage >= 1.0 && c.intel_n >= told { format!("{she}'s told you everything — ask anyway") }
        else if stage == 0.0 { format!("ask what {she} knows — quietly") }
        else { format!("press {her} — what else") };
    vec![row(it, Verb::Question, &ask, ""),
        row(it, Verb::Comply, &format!("“stay down, stay quiet” — lets {her} go, cowering; {she} tells nobody"), ""),
        row(it, Verb::Choke, &format!("choke {her} out — quiet, but a CIVILIAN costs −10 on the card"), &format!("1  draw — gun to {} head", if she == "he" { "his" } else { "her" }))]
}
fn at_gunpoint(g: &Game, it: &Interactable) -> Vec<Interactable> {
    let ready = crate::weapons::firearm_rounds_ready(g, g.pl[g.player].pistol) > 0.0;
    vec![row(it, Verb::Whip, "pistol-whip him — quick, a thud", &format!("LMB  fire past him{}  ·  E  holster — to question him", if ready { "" } else { " (empty)" }))]
}
fn man_held(g: &mut Game, hold: &crate::player::Hold, it: &Interactable) -> Vec<Interactable> {
    let q = crate::dialogue::prompt_for(g, hold.g);
    let tk = held_talk(g, &crate::seats::here(g), hold);
    let verb = if q.spent { String::from("he's said his piece — ask anyway") } else { format!("“{}”", q.text) };
    let ask = if tk.asking { String::from("you're asking — he answers in a beat") }
        else if tk.sam_up { format!("{verb} — once you're done talking") }
        else if !tk.ready { String::from("he's answering — tap to cut it short") }
        else if tk.settled { verb }
        else { format!("{verb} — cuts in") };
    vec![row(it, Verb::Question, &ask, ""), row(it, Verb::Choke, "choke him out — quiet", "RMB  choke him out  ·  1  draw — gun to his head")]
}

/// With a man in his arm: what walking him into a door near does.
fn held_at_door(g: &Game, o: &mut Offers, h: Him, d: DoorId) {
    let at = crate::doors::door_pos(g, d);
    if g.dr[d].leaf.off || m::min(v3::dist_xz(at, h.p), v3::dist_xz(g.dr[d].frame_centre, h.p)) > NEARBY_M { return; }
    let (is, does) = if crate::doors::doors_locked_out(g, d, h.p, None) { ("locked", "walk him into it — his key turns the lock") }
        else if crate::doors::door_is_closed(g, d) { ("shut", "walk him into it — it pushes open") }
        else { ("open", "open") };
    o.list.push(Interactable { door: Some(d), ..mark(InteractableKind::Door, at, format!("door · {is}"), does, false) });
}

/// One of the staff on his feet, seen.
pub(super) fn staff(g: &mut Game, o: &mut Offers, h: Him, gd: GuardId) {
    if crate::condition::down(g, g.gd[gd].ch) || g.team(gd).ours() || crate::fogofwar::fog_of_war_hidden(g, gd) { return; }
    crate::close::close_rows(g, gd, &h.p, h.busy, o);
}

/// A civilian near, seen, who is no captive of the mission's: whether he can be taken hold of, and why not.
pub(super) fn civilian(g: &mut Game, o: &mut Offers, h: Him, gd: GuardId) {
    if crate::fogofwar::fog_of_war_hidden(g, gd) || g.staged.captives.contains(&gd) { return; }
    let at = g.ch[g.gd[gd].ch].pos;
    if v3::dist_xz(h.p, at) >= 2.4 { return; }
    let cv = g.gd[gd].civ.unwrap();
    let gate = crate::close::close_civ_gate(g, gd, &h.p);
    let state = g.cv[cv].state;
    let catchable = !matches!(state, CivState::Flee | CivState::Evacuate | CivState::Report);
    let could = gate.near && gate.behind && catchable && !h.sprints && nothing_between(g, h.p, at);
    let can = could && h.stands(g);
    let c = &g.cv[cv];
    let (she, her) = (&c.she, &c.her);
    let is = match state { CivState::Work => String::new(), CivState::Cower => String::from(" · cowering"), s => format!(" · {}", s.as_str()) };
    let why_not = if !catchable { format!("not while {she}'s {}", if state == CivState::Report { "with a guard" } else { "running" }) }
        else if !gate.behind { format!("get behind {her}") }
        else if h.sprints { String::from("stop sprinting") }
        else if !could { String::from("get closer") }
        else { format!("no room to stand up behind {her} here") };
    let it = Interactable { guard: Some(gd), close: Some(gate.kind), ..mark(InteractableKind::Guard, v3::add(at, [0.0, 1.35, 0.0]), format!("{} · the {}{is}", c.name, c.label), why_not, can) };
    if !can || h.busy { o.add(it, Vec::new()); return; }
    let grab = if g.pl[h.pl].holstered { row(&it, Verb::Grab, &format!("grab {her} — a hand over the mouth"), "LMB  takedown — quiet, and −10") }
        else if grab_variant(g, &crate::seats::here(g)) == HoldVariant::Gun { row(&it, Verb::Grab, &format!("grab {her} — pistol to {her} head"), "E  holster for the LMB takedown") }
        else { row(&it, Verb::Grab, &format!("grab {her}"), "E  holster for the LMB takedown") };
    crate::close::close_civ_offer(g, gd, &h.p);
    o.add(it, vec![grab]);
}
fn nothing_between(g: &Game, a: V3, b: V3) -> bool {
    let (a, b) = ([a[0], 1.0, a[2]], [b[0], 1.0, b[2]]);
    !crate::scene_collision::solid_blocked(g, a, b, None) && !crate::scene_collision::segment_blocked_dynamic(g, a, b, None)
}

/// Every body on the floor, seen: a civilian's like any.
pub(super) fn bodies(g: &Game, o: &mut Offers, h: Him) {
    let mut all = g.guards.clone();
    all.extend(crate::civilians::civilians_shown(g));
    for gd in all {
        if crate::condition::down(g, g.gd[gd].ch) && !crate::fogofwar::fog_of_war_hidden(g, gd) { body(g, o, h, gd); }
    }
}
fn body(g: &Game, o: &mut Offers, h: Him, gd: GuardId) {
    let c = g.gd[gd].ch;
    let hips = g.ch[c].bones.get(Bone::Hips).copied().unwrap_or(g.ch[c].pos);
    let his = g.pl[h.pl].dragging() == Some(gd);
    let reach = his || body_dist(g, gd, Some(1.3)) < 1.3;
    let callsign = &g.gd[gd].callsign;
    let name = match g.gd[gd].civ { Some(cv) => format!("{callsign} (the {})", g.cv[cv].label), None => callsign.clone() };
    let line1 = format!("body · {name}, {}", if crate::game::is_breathing(g, gd) { "out cold — breathing" } else { "dead" });
    let it = Interactable { guard: Some(gd), ..mark(InteractableKind::Body, [hips[0], hips[1] + 0.12, hips[2]], line1, if reach { "" } else { "step closer" }, reach) };
    let verb = if !reach || h.busy { None }
        else if his { Some(row(&it, Verb::Drop, if g.pl[h.pl].haul().is_some_and(|w| w.carry.is_some()) { "put him down" } else { "let go of him" }, "sprinting drops him")) }
        else if !h.drags { Some(row(&it, Verb::Drag, "pick him up", "")) }
        else { None };
    o.add(it, verb.into_iter().collect());
}
