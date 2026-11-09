//! WHAT A CARD CHANGED WRITES LIVE: command 30 compares the cards with the last told, for the men who come, and each thing is written on the man where the game can take it: a count on the pools, a teammate's gun, light, suit and trim, all of Sam's card. What it cannot (the size, the goggles, a man not there) is a restage under the dissolve.
use super::acts::{actor, ask_veil, cut, open_card, pick, pools, thing};
use super::light::KIT_VIEW_LOOP as K;
use super::types::*;
use super::walk::rotate;
use crate::game::{Game, SlotKind};
use crate::ids::*;
use crate::kit::{MemberKit, SuitId, TrimId};
use crate::weapons::WeaponId;
use alloc::vec::Vec;

/// the pistols' rows (the rest are long guns)
pub(super) fn short_gun(w: WeaponId) -> bool { let s = w.as_str(); ["fiveseven", "guard9mm", "hk45", "g18", "g19bare"].iter().any(|n| s.contains(n)) }
/// a teammate's gun written live: his hands' errand now (the right hand over the shoulder and back), the model at its far point
fn swap_gun(g: &mut Game, st: &mut KitStage, x: GuardId, id: WeaponId) {
    if let Some(w) = st.a.swaps.iter_mut().find(|w| w.man == x) { w.id = id; return; }
    let item = |w: WeaponId| if short_gun(w) { crate::character::KitItem::Pistol } else { crate::character::KitItem::Rifle };
    let (from, into) = (item(g.fa[g.gd[x].weapon].def), item(id));
    let c = g.gd[x].ch;
    crate::character::anim_kit_swap(g, c, Some(from), Some(into), None, Some(from == into));
    st.a.swaps.push(GunSwap { man: x, id, at: g.time + K.swap_at });
}
/// Sam's card put on him again as he stands, what is in his hand kept where the card still has it
pub(super) fn sam_card(g: &mut Game, st: &mut KitStage) {
    let pl = g.player;
    let (slot, hol) = (g.pl[pl].slot, g.pl[pl].holstered);
    crate::kit::kit_show(g);
    crate::kit::kit_player(g);
    if slot != SlotKind::Rifle || g.pl[pl].has_rifle { g.pl[pl].slot = slot; }
    g.pl[pl].holstered = hol;
    st.a.sam_card_due = false;
    let f = st.a.beat.as_ref().map(|b| b.field);
    pools(g, st, f);
    let (r, p) = (g.pl[pl].rifle, g.pl[pl].pistol);
    g.fa[r].light_on = st.sam_on;
    g.fa[p].light_on = st.sam_on;
}
/// every field that differs between two cards, as the page names them
fn moved(a: &MemberKit, b: &MemberKit) -> Vec<&'static str> {
    let mut o = Vec::new();
    if a.weapon != b.weapon { o.push("weapon"); }
    if a.suppressed != b.suppressed { o.push("suppressed"); }
    if a.light != b.light { o.push("light"); }
    if a.nvg != b.nvg { o.push("nvg"); }
    if a.wand != b.wand { o.push("wand"); }
    if a.bare != b.bare { o.push("bare"); }
    if a.big != b.big { o.push("big"); }
    if a.primary != b.primary { o.push("primary"); }
    if a.secondary != b.secondary { o.push("secondary"); }
    if a.side_can != b.side_can { o.push("sideCan"); }
    if a.suit != b.suit { o.push("suit"); }
    if a.trim != b.trim { o.push("trim"); }
    for (k, x, y) in [("bangs", a.bangs, b.bangs), ("frags", a.frags, b.frags), ("smoke", a.smoke, b.smoke), ("charges", a.charges, b.charges), ("walls", a.walls, b.walls), ("cams", a.cams, b.cams), ("shells", a.shells, b.shells), ("rings", a.rings, b.rings)] {
        if x != y && !(x.is_nan() && y.is_nan()) { o.push(k); }
    }
    o
}
const COUNTS: [&str; 8] = ["bangs", "frags", "smoke", "charges", "walls", "cams", "shells", "rings"];

/// the cards are written (the caller's) and how many come (`size`: asked, where the mission's men are not its own). WHAT MOVED is written live; what cannot be is staged again under the dissolve.
pub(super) fn told(g: &mut Game, st: &mut KitStage, size: Option<u32>) {
    let was = core::mem::replace(&mut st.a.cards, g.m.kit.kit.members.clone());
    let team = crate::scenario_run::team_of(g);
    st.a.hi = thing(g, &st.field, st.who);
    let alive = team.iter().filter(|&&x| g.gd[x].state != crate::game::GuardState::Dead).count() as u32;
    let n = size.unwrap_or(alive);
    let mut hard = st.none || n != alive;
    let mut play: Option<&'static str> = None;
    if !hard {
        'men: for i in 0..=n as usize {
            let (Some(a), Some(b)) = (was.get(i), st.a.cards.get(i).cloned()) else { continue };
            let man = if i > 0 { team.get(i - 1).copied() } else { None };
            for f in moved(a, &b) {
                if i > 0 && man.is_none() { hard = true; break 'men; }
                let open = i as u32 == st.who;
                if COUNTS.contains(&f) { if open && play != Some("swap") { if let Some(t) = thing(g, f, st.who) { play = Some(t); } } continue; }
                let Some(x) = man else {
                    st.a.sam_card_due = true;
                    if matches!(f, "weapon" | "primary" | "secondary") { if st.who == 0 { play = Some("swap"); } }
                    else if st.who == 0 && play != Some("swap") { if let Some(t) = thing(g, f, 0) { play = Some(t); } }
                    continue;
                };
                match f {
                    "weapon" | "primary" | "secondary" | "suppressed" => { let id = crate::kit::kit_gun(g, &b); swap_gun(g, st, x, id); if open { play = Some("swap"); } }
                    "light" => { g.gd[x].light_dead_until = if b.light { -1.0 } else { f64::INFINITY }; if open && play != Some("swap") { play = Some("light"); } }
                    "suit" | "trim" => {
                        let c = g.gd[x].ch;
                        let v = if f == "suit" {
                            if b.suit == SuitId::Issued { st.a.issued_suit.iter().find(|e| e.0 == i as u32).map(|e| e.1) } else { Some(crate::kit::suits().get(alloc::string::String::from(b.suit.as_str())).unwrap().tint) }
                        } else {
                            if b.trim == TrimId::Issued { st.a.issued_trim.iter().find(|e| e.0 == i as u32).map(|e| e.1) } else { Some(crate::kit::trims().get(alloc::string::String::from(b.trim.as_str())).unwrap().tint) }
                        };
                        let Some(v) = v else { hard = true; break 'men };
                        if f == "suit" { g.ch[c].tint = v; } else { g.ch[c].tint2 = v; }
                        if open && play.is_none() { play = Some("turn"); }
                    }
                    _ => { hard = true; break 'men; }   // (the goggles: no live path in the game)
                }
            }
        }
    }
    if hard {
        if st.a.beat.is_some() && !st.none { cut(g, st); }
        if let Some(p) = play { st.a.asked = Some(if p == "swap" { "weapon" } else { p }); }
        if st.none { st.due = true; } else { ask_veil(g, st); }
        return;
    }
    let f = st.a.beat.as_ref().map(|b| b.field);
    pools(g, st, f);
    if play.is_some() { pick(st, play); }
}
/// whose card is open and the row under its highlight: another card's man walks to the head of the file, the same men; another tile plays at once
pub(super) fn opened(g: &mut Game, st: &mut KitStage, who: u32, field: &str) {
    st.want = who;
    st.field = alloc::string::String::from(field);
    let changed = open_card(g, st);
    let f = thing(g, field, st.who);
    if changed {
        st.a.hi = f;
        pick(st, f);
        if !st.none {
            if st.a.beat.is_some() { st.a.cut_now = false; cut(g, st); }
            let m0 = actor(g, st);
            rotate(g, st, m0);
        }
    } else if f != st.a.hi {
        st.a.hi = f;
        pick(st, f);
    }
}
