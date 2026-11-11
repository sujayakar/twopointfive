//! A DOOR'S ROWS: its keypad, its lock, his key, its leaf; the boot, the big slot's tool, the wand and a charge.
use super::*;

const NO_BOOT: &str = "kick it — it opens toward you, so that won't work";

pub(super) fn door(g: &mut Game, o: &mut Offers, h: Him, d: DoorId) {
    if g.dr[d].leaf.off { return; }
    let at = crate::doors::door_pos(g, d);
    let near = m::min(v3::dist_xz(at, h.p), v3::dist_xz(g.dr[d].frame_centre, h.p)) < crate::doors::DOORS_USE_REACH;
    let reach = near && !h.sprints;
    let free = reach && !h.busy;
    let mut it = Interactable { door: Some(d), ..mark(InteractableKind::Door, at, "", if reach { "" } else { "step closer" }, reach) };
    let locked = crate::doors::doors_locked_out(g, d, h.p, Some(&g.pl[h.pl].keys));
    let mut rows = Vec::new();
    if let Some(pad) = crate::devices::pad_state(g, &crate::seats::here(g), d, h.p) {
        it.line1 = keypad_line(g, h, d, &pad);
        let boot = Boot { holds: crate::doors::doors_boot_holds(g, d, h.p), steel: false };
        if near && h.sprints { boot.run_word(&mut it); }
        if free { rows.push(code_row(&it, &pad)); rows.extend(boot.row(g, h, &it)); }
    } else if locked {
        it.line1 = lock_line(g, h, d);
        let pick = g.dr[d].leaf.pick;
        if pick > 0.0 { it.progress = Some(pick); }
        let steel = g.dr[d].def.steel == Some(true);
        let boot = Boot { holds: !steel && crate::doors::doors_boot_holds(g, d, h.p), steel };
        if near && h.sprints { boot.run_word(&mut it); }
        if free { rows.extend(lock_rows(h, &it, pick, steel)); rows.extend(boot.row(g, h, &it)); }
    } else if crate::doors::doors_key_opens(g, d, h.p, Some(&g.pl[h.pl].keys)) {
        it.line1 = String::from("door · locked — you have the key");
        if free { rows.push(row(&it, Verb::Unlock, "unlock — his key", "")); rows.push(ease_row(&it)); }
    } else {
        it.line1 = leaf_line(g, d);
        if free { rows.extend(leaf_rows(g, h, d, &it)); }
    }
    big_row(g, h, d, &mut it, reach, locked, &mut rows);
    if free && crate::wand::wand_offered(g, h.pl, d) {
        // (the last row of what the door offers: where they are as many as are listed it takes the place of the handle's)
        if rows.len() >= MAX_NEAR { rows.iter().position(|r| r.verb == Some(Verb::Rattle)).map(|k| rows.remove(k)); }
        rows.push(row(&it, Verb::Wand, "look under it — the wand, quiet", ""));
    }
    crate::doortools::door_tools_sam_rows(g, &mut it, d, &mut rows, free);
    o.add(it, rows);
}

/// His boot, at a door that is fast: whether its frame holds it on this side, and whether it is steel.
struct Boot { holds: bool, steel: bool }
impl Boot {
    /// running at it, he is told what running into it does
    fn run_word(&self, it: &mut Interactable) {
        if !self.holds { it.line2 = String::from(if self.steel { "run into it  (it won't give)" } else { "run into it  kick it in (LOUD)" }); }
    }
    fn row(&self, g: &Game, h: Him, it: &Interactable) -> Option<Interactable> {
        if h.drags || !h.stands(g) { return None; }
        if self.holds { return Some(Interactable { off: true, ..row(it, Verb::Kick, NO_BOOT, "") }); }
        Some(row(it, Verb::Kick, if self.steel { "kick it — LOUD, and steel won't give" } else { "kick it in — LOUD" }, if h.low { "" } else { "or sprint into it" }))
    }
}

fn keypad_line(g: &Game, h: Him, d: DoorId, pad: &crate::devices::PadState) -> String {
    if g.pl[h.pl].coding().is_some_and(|c| c.door == d) { String::from("door · keypad · keying it in") }
    else if pad.known { format!("door · keypad — you know the code ({})", crate::devices::spell(&pad.pad.def.code)) }
    else { String::from("door · keypad · locked") }
}
fn code_row(it: &Interactable, pad: &crate::devices::PadState) -> Interactable {
    if pad.known { row(it, Verb::Code, &format!("enter the code — {}", crate::devices::spell(&pad.pad.def.code)), "") }
    else { Interactable { off: true, ..row(it, Verb::Nocode, "keypad — no code", "") } }
}

fn lock_line(g: &Game, h: Him, d: DoorId) -> String {
    let leaf = &g.dr[d].leaf;
    if leaf.rattle_t > 0.0 { return String::from("door · locked — won't budge"); }
    if g.dr[d].def.steel == Some(true) { return String::from("steel fire door · bolted from the stairs"); }
    if g.pl[h.pl].picking() == Some(&d) { return String::from("door · locked · picking it"); }
    if leaf.pick > 0.0 { format!("door · locked · {}% picked", pct(leaf.pick)) } else { String::from("door · locked") }
}
fn lock_rows(h: Him, it: &Interactable, pick: f64, steel: bool) -> Vec<Interactable> {
    let mut rows = Vec::new();
    if !steel {
        let text = if pick > 0.0 { format!("pick the lock — quiet, {}% done", pct(pick)) } else { String::from("pick the lock — quiet, 4 s") };
        rows.push(row(it, Verb::Pick, &text, ""));
    }
    if h.low || h.drags || steel { rows.push(row(it, Verb::Rattle, if steel { "try the bar — no keyhole this side" } else { "try the handle" }, "")); }
    rows
}

fn leaf_line(g: &Game, d: DoorId) -> String {
    let (leaf, def) = (&g.dr[d].leaf, &g.dr[d].def);
    let tail = if leaf.lock_broken { " · lock kicked in" } else if def.locked == Some(true) && !leaf.locked { " · picked" } else { "" };
    format!("door · {}{tail}", if crate::doors::door_is_closed(g, d) { "shut" } else { "open" })
}
fn leaf_rows(g: &Game, h: Him, d: DoorId, it: &Interactable) -> Vec<Interactable> {
    let toward = crate::doors::doors_pull_face(g, d, h.p);
    let mut rows = vec![if crate::doors::door_is_closed(g, d) { row(it, Verb::Push, if toward { "pull it open" } else { "push it open" }, "") }
        else { row(it, Verb::Pull, if toward { "push it shut" } else { "pull it shut" }, "") }];
    if m::abs(g.dr[d].leaf.angle) < EASE_OFFER { rows.push(ease_row(it)); }
    rows
}
fn ease_row(it: &Interactable) -> Interactable { row(it, Verb::Ease, "ease it open — silent", "") }

/// The big slot's tool, where he has one for this door: in the boot's row where that stands, and offered from further off than his hands reach.
fn big_row(g: &Game, h: Him, d: DoorId, it: &mut Interactable, reach: bool, locked: bool, rows: &mut Vec<Interactable>) {
    if h.busy || h.drags || h.sprints || !h.stands(g) { return; }
    let far = if reach { None } else { crate::bigkit::sam_big_far(g, d, h.p) };
    let near = if reach { crate::bigkit::sam_big(g, &crate::seats::here(g), d) } else { None };
    if far.is_some() { it.in_reach = true; }
    let Some(tool) = far.or(near) else { return };
    let (verb, text) = match tool {
        crate::kit::BigTool::Shotgun => (Verb::Breach, if locked { "shoot the lock off — LOUD" } else { "shoot it open — LOUD" }),
        _ => (Verb::Hammer, "hammer it open — LOUD"),
    };
    let r = Interactable { past: Some(!locked), ..row(it, verb, text, "") };
    match rows.iter_mut().find(|r| r.verb == Some(Verb::Kick)) { Some(kick) => *kick = r, None => rows.push(r) }
}
