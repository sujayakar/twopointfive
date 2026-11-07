//! src/game/ownblow.ts lines 1 to 66
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The key a set charge's row names: the squad's while that key blows it, else the interaction's. (ownblow.ts blowKeyWord)
pub fn blow_key_word(key_is: bool, key: &str) -> String {
    if key_is { String::from(key) } else { String::from("Space") }
}

/// What a line drawn as written says after 'charge set': the squad's key by name while it blows it, nothing for his own. (ownblow.ts blowTail)
pub fn blow_tail(key_is: bool, key: &str, sep: Option<&str>) -> String {
    let sep = sep.unwrap_or(" · ");
    if key_is { alloc::format!("{}press {} to blow it", sep, key) } else { String::new() }
}

/// The nearest free charge of his own, a door's or a wall's. (ownblow.ts ownSet)
pub fn own_set(g: &mut crate::game::Game) -> Option<crate::ownblow::OwnSet> {
    if g.door_tools.set.is_empty() && g.wall_charge.set.is_empty() { return None; }
    let p = g.ch[g.pl[g.player].ch].pos;
    let d = crate::doortools::door_tools_sam_set(g);
    let w = crate::wallcharge::wall_charge_sam_set(g);
    if d.is_none() && w.is_none() { return None; }
    let w_at = match w { Some(w) => Some(crate::wallcharge::wall_charge_sam_at(g, w)), None => None };
    if let Some(d) = d {
        let door = g.tab.doortools_set_charge[d].door;
        let nearer = match w_at { None => true, Some(w_at) => v3::dist_xz(g.dr[door].frame_centre, p) <= v3::dist_xz(w_at, p) };
        if nearer {
            let at = crate::doors::door_pos(g, door);
            let words = crate::doortools::door_tools_close_words(g, d);
            let where_ = crate::doortools::door_tools_sam_where(g, d);
            return Some(OwnSet { at, words, where_, fatal: false, go: OwnSetGo::Door { d } });
        }
    }
    // The TypeScript's `w!`, `wAt!`: a door's charge loses the distance only to a wall's, and with no door's charge the wall's is what stands.
    let w = w?;
    let words = crate::wallcharge::wall_charge_sam_words(g, w);
    let fatal = crate::wallcharge::wall_charge_sam_fatal(g, w);
    let where_ = String::from("the wall");
    Some(OwnSet { at: w_at?, words, where_, fatal, go: OwnSetGo::Wall { w } })
}

/// player.ts buildInteractables: the row, or None. (ownblow.ts ownBlowRow)
pub fn own_blow_row(g: &mut crate::game::Game) -> Option<crate::game::Interactable> {
    let c = own_set(g)?;
    if !(crate::rails::rails_offers(g, "blow") || crate::rails::rails_offers(g, "charge"))
        || (crate::rails::rails_locks(g, "blow") && crate::rails::rails_locks(g, "word")) { return None; }
    Some(crate::game::Interactable {
        kind: crate::game::InteractableKind::Charge,
        pos: c.at,
        line1: alloc::format!("charge set{} · {}", c.words, c.where_),
        line2: String::from("Space  blow the charge"),
        in_reach: true,
        off: c.fatal,
        verb: Some(crate::game::Verb::Blow),
        running: Some(false),
        ..Default::default()
    })
}

/// interact.ts, at a bare tap's release: is the highlighted row his own charge where it would put him down. (ownblow.ts ownBlowHeld)
pub fn own_blow_held(g: &mut crate::game::Game, r: Option<&crate::game::Interactable>) -> bool {
    let Some(r) = r else { return false; };
    if r.kind != crate::game::InteractableKind::Charge || r.verb != Some(crate::game::Verb::Blow) { return false; }
    if !own_set(g).map_or(false, |c| c.fatal) { return false; }
    crate::game::game_msg(g, "too close to your own charge");
    true
}

/// game.ts, where this frame's act is read: the row given. (ownblow.ts ownBlowAct)
pub fn own_blow_act(g: &mut crate::game::Game) {
    let Some(a) = g.act.as_ref() else { return; };
    if a.kind != crate::game::InteractableKind::Charge || a.verb != Some(crate::game::Verb::Blow) || g.pl[g.player].down { return; }
    let Some(c) = own_set(g) else { return; };
    match c.go {
        OwnSetGo::Door { d } => crate::doortools::door_tools_trigger(g, Some(d)),
        OwnSetGo::Wall { w } => { crate::wallcharge::wall_charge_blow(g, Some(w), None); }
    }
}
