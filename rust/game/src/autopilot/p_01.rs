//! src/game/autopilot.ts lines 1 to 153
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The team has that man in sight: the fog of war's answer where it runs, the room fight's otherwise.   (autopilot.ts inSight)
pub fn in_sight(g: &mut crate::game::Game, o: GuardId) -> bool {
    if crate::fogofwar::fog_of_war_active(g) { crate::fogofwar::fog_of_war_seen(g, o) } else { crate::roomfight::room_fight_team_sees(g, o) }
}

/// The entries the squad would take on a door it is stacked on, first choice first. AUTOPILOT.plain is settled on: the key never picks a tool.   (autopilot.ts autoEntries)
pub fn auto_entries(g: &mut crate::game::Game, t: crate::orders::OrderTargetDoor, shut: Option<bool>) -> crate::autopilot::AutoEntries {
    let why = if crate::silent::squad_silent(g) { crate::autopilot::AutoPickWhy::Silent } else { crate::autopilot::AutoPickWhy::Known };
    crate::autopilot::AutoEntries { ids: vec![String::from("openClear")], why, kept: None }
}

/// `S.defs.find(x => x.id === id)`
fn def_of(g: &mut Game, id: &str) -> Option<OrdersOrderDefId> {
    let mut i = 0;
    while i < g.orders.defs.len() {
        let d = g.orders.defs[i];
        i += 1;
        if g.tab.orders_order_def[d].id == id { return Some(d); }
    }
    None
}

/// The target's first row: floor, nothing, a door nobody waits on.   (autopilot.ts autoPick `first`)
fn first(g: &mut Game, t: &crate::orders::OrderTarget) -> crate::autopilot::AutoPick {
    let id = crate::orders::squad_orders_default_for(g, t).map(|d| g.tab.orders_order_def[d].id.clone());
    crate::autopilot::AutoPick { id, men: None, word: false, why: crate::autopilot::AutoPickWhy::First, t: t.clone(), kept: None }
}

/// What the quick key does on `t` for the men it would go to. `check`: the entry is the first its row's `valid` takes; without, the first choice unasked.   (autopilot.ts autoPick)
pub fn auto_pick(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, check: Option<bool>, men: Option<&[GuardId]>) -> crate::autopilot::AutoPick {
    use crate::autopilot::{AutoPick, AutoPickWhy};
    let check = check.unwrap_or(true);
    let crate::orders::OrderTarget::Door(td) = t else { return first(g, t) };
    let door = td.door;
    let r: Vec<GuardId> = match men {
        Some(m) => m.to_vec(),
        None => crate::orders::squad_orders_recipients(g, Some(t), None),
    };
    let Some(w) = crate::doortools::door_tools_waits_on(g, &r, door) else {
        let Some(c) = crate::doortools::door_tools_charge_on(g, door) else { return first(g, t) };
        return if crate::doortools::door_tools_word_pending(g, c) {
            AutoPick { id: None, men: None, word: false, why: AutoPickWhy::Pending, t: t.clone(), kept: None }
        } else {
            AutoPick { id: Some(String::from("charge")), men: None, word: false, why: AutoPickWhy::Blow, t: t.clone(), kept: None }
        };
    };
    if w.kind == crate::doortools::DoorToolsWaitsOnKind::Word {
        if crate::orders::squad_orders_big(g) || !crate::doortools::door_tools_threshold_held(g, &w.men, Some(door)).is_empty() {
            return AutoPick { id: None, men: Some(w.men), word: true, why: AutoPickWhy::Word, t: t.clone(), kept: None };
        }
        // (the pair has no word but a held threshold's: its own set charge is blown by the row)
        return if crate::doortools::door_tools_charge_on(g, door).is_some() {
            AutoPick { id: Some(String::from("charge")), men: Some(w.men), word: false, why: AutoPickWhy::Blow, t: t.clone(), kept: None }
        } else {
            first(g, t)
        };
    }
    if !w.set { return first(g, t); } // (still walking to their marks: 'stack up' again)
    // AUTOPILOT.restack is settled on: on their marks it is 'stack up' again, answered and not walked, but for the two cases below it
    if w.men.len() < r.len() { return first(g, t); }
    if crate::orders::squad_orders_big(g) || !crate::doortools::door_tools_stack_waits(g, &w.men).is_empty() {
        return AutoPick { id: Some(String::from("stack")), men: Some(w.men), word: false, why: AutoPickWhy::Set, t: t.clone(), kept: None };
    }
    // the men's own face of the door, off the marks they stand on: no path search to find it
    let a = match w.men.first().copied() { Some(m) => crate::orders::squad_orders_of(g, m), None => None };
    let post = match a { Some(a) => g.tab.orders_assignment[a].post, None => None };
    let side = match post { Some(p) => crate::orders::side_of(g, door, p), None => td.side };
    let td_men = crate::orders::OrderTargetDoor { side, ..*td };
    let tm = crate::orders::OrderTarget::Door(td_men);
    let e = auto_entries(g, td_men, Some(check));
    let why = e.why;
    if check {
        for id in &e.ids {
            let Some(d) = def_of(g, id) else { continue };
            if crate::orders::squad_orders_why_not(g, d, &tm, Some(&w.men)).is_none() {
                return AutoPick { id: Some(id.clone()), men: Some(w.men), word: false, why, t: tm, kept: e.kept };
            }
        }
    }
    // (nothing fits: given all the same — a man says why, in the row's words)
    AutoPick { id: Some(e.ids[0].clone()), men: Some(w.men), word: false, why, t: tm, kept: e.kept }
}

/// The quick key, pressed on `t`: true when the press was the autopilot's (given and stamped, refused aloud, or a charge left on its word).   (autopilot.ts autoQuick)
pub fn auto_quick(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> bool {
    use crate::autopilot::AutoPickWhy;
    let p = auto_pick(g, t, None, None);
    let tail = g.orders.log.last().map(|row| row.serial); // the last row, asked by identity in the TypeScript: its serial
    g.stats.autopilot.presses += 1;
    if p.why == AutoPickWhy::Pending { return true; }
    if p.why == AutoPickWhy::Set {
        let lines = crate::autopilot::restack_lines();
        let men = p.men.as_deref().unwrap_or(&[]);
        crate::orders::squad_orders_again(g, "stack", t, men, &lines);
        g.stats.autopilot.again += 1;
        return true;
    }
    let no: Option<String>;
    if p.word {
        let at = g.ch[g.pl[g.player].ch].pos;
        no = if crate::orders::squad_orders_big(g) {
            let none = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [at[0], 0.0, at[2]], man: None });
            let opts = crate::orders::GiveOpts { men: p.men.clone(), ack: None, on_word: None };
            crate::orders::squad_orders_give(g, "go", Some(&none), None, Some(&opts))
        } else {
            let who = g.orders.who;
            crate::doortools::door_tools_go(g, Some(who))
        };
    } else if let Some(id) = &p.id {
        let ack = crate::squadnet::why_words(g, &p).map(|w| w.ack);
        let opts = crate::orders::GiveOpts { men: p.men.clone(), ack, on_word: None };
        no = crate::orders::squad_orders_give(g, id, Some(t), None, Some(&opts));
    } else {
        return false;
    }
    if no.is_some() {
        g.stats.autopilot.refused += 1;
        return true;
    }
    if !p.word && crate::squadnet::why_words(g, &p).is_some() { g.squad_net.stats.whys += 1.0; }
    if let Some(row) = g.orders.log.last_mut() {
        if Some(row.serial) != tail { row.auto = Some(true); }
    }
    let id = p.id.as_deref();
    if p.word {
        g.stats.autopilot.word += 1;
    } else if p.why == AutoPickWhy::First {
        g.stats.autopilot.first += 1;
    } else if p.why == AutoPickWhy::Blow {
        g.stats.autopilot.blow += 1;
    } else if id == Some("bangClear") {
        g.stats.autopilot.bang += 1;
    } else if id == Some("charge") {
        g.stats.autopilot.charge += 1;
    } else {
        g.stats.autopilot.open += 1;
    }
    true
}

/// The quick key's words: the row's own for the squad's first choice on `t` (None: nothing). Read by the ring's seats as well as the HUD.   (autopilot.ts autoHint)
pub fn auto_hint(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, p: Option<&crate::autopilot::AutoPick>) -> Option<String> {
    let own;
    let p = match p {
        Some(p) => p,
        None => {
            own = auto_pick(g, t, Some(false), None);
            &own
        }
    };
    if p.word && !crate::orders::squad_orders_big(g) { return Some(String::from("go")); }
    let id = if p.word { Some("go") } else { p.id.as_deref() };
    let d = match id { Some(id) => def_of(g, id), None => None };
    d.map(|d| crate::orders::squad_orders_label_of(g, d, &p.t))
}

