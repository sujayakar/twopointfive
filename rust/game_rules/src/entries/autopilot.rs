pub use ::game_types::autopilot::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use alloc::{string::String, vec, vec::Vec};

/// The team has that man in sight: the fog of war's answer where it runs, the room fight's otherwise.
pub(super) fn in_sight(g: &crate::game::Game, o: GuardId) -> bool {
    if crate::fogofwar::fog_of_war_active(g) { crate::fogofwar::fog_of_war_seen(g, o) } else { crate::roomfight::room_fight_team_sees(g, o) }
}

/// The entries the squad would take on a door it is stacked on, first choice first. AUTOPILOT.plain is settled on: the key never picks a tool.
fn auto_entries(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> crate::autopilot::AutoEntries {
    let why = if crate::silent::squad_silent(g, seat) { crate::autopilot::AutoPickWhy::Silent } else { crate::autopilot::AutoPickWhy::Known };
    crate::autopilot::AutoEntries { ids: vec![String::from(crate::orders::rows::openclear::ORDER.id)], why, kept: None }
}

/// `S.defs.find(x => x.id === id)`
fn def_of(g: &Game, seat: &crate::seats::SeatRef, id: &str) -> Option<OrdersOrderDefId> {
    let mut i = 0;
    while i < seat.orders.defs.len() {
        let d = seat.orders.defs[i];
        i += 1;
        if g.tab.orders_order_def[d].id == id { return Some(d); }
    }
    None
}

/// The target's first row: floor, nothing, a door nobody waits on.
fn first(g: &Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget) -> crate::autopilot::AutoPick {
    let id = crate::orders::squad_orders_default_for(g, seat, t).map(|d| g.tab.orders_order_def[d].id.clone());
    crate::autopilot::AutoPick { id, men: None, word: false, why: crate::autopilot::AutoPickWhy::First, t: t.clone(), kept: None }
}

/// What the quick key does on `t` for the men it would go to: one answer, whoever asks (the call surface still hands the page's `_check`).
pub fn auto_pick(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget, _check: Option<bool>, men: Option<&[GuardId]>) -> crate::autopilot::AutoPick {
    use crate::autopilot::{AutoPick, AutoPickWhy};
    let crate::orders::OrderTarget::Door(td) = t else { return first(g, seat, t) };
    let door = td.door;
    let r: Vec<GuardId> = match men {
        Some(m) => m.to_vec(),
        None => crate::orders::squad_orders_recipients(g, &crate::seats::here(g), Some(t), None),
    };
    let Some(w) = crate::doortools::door_tools_waits_on(g, &r, door) else {
        let Some(c) = crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), door) else { return first(g, seat, t) };
        return if crate::doortools::door_tools_word_pending(g, &crate::seats::here(g), c) {
            AutoPick { id: None, men: None, word: false, why: AutoPickWhy::Pending, t: t.clone(), kept: None }
        } else {
            AutoPick { id: Some(String::from(crate::orders::rows::charge::ORDER.id)), men: None, word: false, why: AutoPickWhy::Blow, t: t.clone(), kept: None }
        };
    };
    if w.kind == crate::doortools::DoorToolsWaitsOnKind::Word {
        if crate::orders::squad_orders_big(g, &crate::seats::here(g)) || !crate::doortools::door_tools_threshold_held(g, &crate::seats::here(g), &w.men, Some(door)).is_empty() {
            return AutoPick { id: None, men: Some(w.men), word: true, why: AutoPickWhy::Word, t: t.clone(), kept: None };
        }
        // (the pair has no word but a held threshold's: its own set charge is blown by the row)
        return if crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), door).is_some() {
            AutoPick { id: Some(String::from(crate::orders::rows::charge::ORDER.id)), men: Some(w.men), word: false, why: AutoPickWhy::Blow, t: t.clone(), kept: None }
        } else {
            first(g, seat, t)
        };
    }
    if !w.set { return first(g, seat, t); } // (still walking to their marks: 'stack up' again)
    // AUTOPILOT.restack is settled on: on their marks it is 'stack up' again, answered and not walked, but for the two cases below it
    if w.men.len() < r.len() { return first(g, seat, t); }
    if crate::orders::squad_orders_big(g, &crate::seats::here(g)) || !crate::doortools::door_tools_stack_waits(g, &crate::seats::here(g), &w.men).is_empty() {
        return AutoPick { id: Some(String::from(crate::orders::rows::stack::ORDER.id)), men: Some(w.men), word: false, why: AutoPickWhy::Set, t: t.clone(), kept: None };
    }
    // the men's own face of the door, off the marks they stand on: no path search to find it
    let a = match w.men.first().copied() { Some(m) => crate::orders::squad_orders_of(&crate::seats::here(g), m), None => None };
    let post = match a { Some(a) => g.tab.task[a].post, None => None };
    let side = match post { Some(p) => crate::orders::side_of(g, door, p), None => td.side };
    let td_men = crate::orders::OrderTargetDoor { side, ..*td };
    let tm = crate::orders::OrderTarget::Door(td_men);
    let e = auto_entries(g, &crate::seats::here(g));
    let why = e.why;
    // (nothing fits: given all the same — a man says why, in the row's words)
    AutoPick { id: Some(e.ids[0].clone()), men: Some(w.men), word: false, why, t: tm, kept: e.kept }
}

/// The quick key tapped on `t`: everyone it goes to.
pub fn auto_quick(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> bool { quick(g, t, None) }

/// WHAT THE QUICK KEY MEANS ON `t`, GIVEN: to everyone it goes to, or with Shift to `men` alone. One answer (`auto_pick`), so a part of the squad is given
/// what all of it would be. True: a new group of men was sent.
pub fn quick(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> bool {
    use crate::autopilot::AutoPickWhy;
    let p = auto_pick(g, &crate::seats::here(g), t, None, men);
    let (tail, group) = (g.sets[g.seat].orders.log.last().map(|row| row.serial), g.sets[g.seat].orders.group); // the last row's serial: a row the call below logs has another
    let to = p.men.clone().or_else(|| men.map(|m| m.to_vec()));
    if p.why == AutoPickWhy::Pending { return false; }
    if p.why == AutoPickWhy::Set {
        let lines = crate::autopilot::restack_lines();
        crate::orders::squad_orders_again(g, crate::orders::rows::stack::ORDER.id, to.as_deref().unwrap_or(&[]), &lines);
        return false;
    }
    let no = if p.word {
        let at = g.player_pos(g.player);
        if men.is_some() || crate::orders::squad_orders_big(g, &crate::seats::here(g)) {
            let none = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [at[0], 0.0, at[2]], man: None });
            let opts = crate::orders::GiveOpts { men: to, ack: None };
            crate::orders::squad_orders_give(g, crate::orders::rows::go::ORDER.id, Some(&none), None, Some(&opts))
        } else {
            let who = g.sets[g.seat].orders.who;
            crate::doortools::door_tools_go(g, Some(who))
        }
    } else if let Some(id) = &p.id {
        let ack = crate::squadnet::why_words(&p).map(|w| w.ack);
        let opts = crate::orders::GiveOpts { men: to, ack };
        crate::orders::squad_orders_give(g, id, Some(t), None, Some(&opts))
    } else {
        return false;
    };
    if no.is_some() {
        return false;
    }
    if let Some(row) = g.sets[g.seat].orders.log.last_mut() {
        if Some(row.serial) != tail { row.auto = Some(true); }
    }
    let id = p.id.as_deref();
    if p.word {
    } else if p.why == AutoPickWhy::First {
    } else if p.why == AutoPickWhy::Blow {
    } else if id == Some(crate::orders::rows::bangclear::ORDER.id) {
    } else if id == Some(crate::orders::rows::charge::ORDER.id) {
    } else {
    }
    g.sets[g.seat].orders.group != group
}

/// The quick key's words: the row's own for the squad's first choice on `t` (None: nothing). Read by the ring's seats as well as the HUD.
pub fn auto_hint(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, p: Option<&crate::autopilot::AutoPick>) -> Option<String> {
    let own;
    let p = match p {
        Some(p) => p,
        None => {
            own = auto_pick(g, &crate::seats::here(g), t, Some(false), None);
            &own
        }
    };
    if p.word && !crate::orders::squad_orders_big(g, &crate::seats::here(g)) { return Some(String::from(crate::orders::rows::go::ORDER.id)); }
    let id = if p.word { Some(crate::orders::rows::go::ORDER.id) } else { p.id.as_deref() };
    let d = match id { Some(id) => def_of(g, &crate::seats::here(g), id), None => None };
    d.map(|d| crate::orders::squad_orders_label_of(g, d, &p.t))
}
