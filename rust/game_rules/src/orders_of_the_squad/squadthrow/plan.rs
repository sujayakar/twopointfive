//! A SQUAD THROW IS ONE PLAN. "Who of these men would throw it there, from where, and whom would it catch" is worked out once, as a `ThrowPlan`, and the
//! menu's row (its words, the one man it is for, its refusal, its warning), the picture of the highlighted row and the order given are all read off it.
//!
//! WHAT IS CHEAP IS ALWAYS NOW, WHAT IS DEAR IS KEPT. The nearest man's straight throw from where he stands, and whom a throw would catch, are asked each time;
//! a throw somebody has to work out, or walk to, is a search of some milliseconds and is kept on the open menu. Its reading (`squad_orders_refresh`, once a
//! second and as its list changes) drops what is kept (`forget`); between readings the plan of the LIT row follows the squad (`follow`, once a frame), and
//! the search is made again as one of ours comes into another half metre of floor. A press, or any other order, takes the kept throw made again as
//! things are (`throw_again`: the same man, the same spot); one that no longer goes off, or that was refused, is searched for afresh. With no menu up
//! nothing is kept past the frame.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::V3;
use alloc::{format, string::String, vec, vec::Vec};

/// the menu is being read again: what was worked out for it is worked out again as its rows ask
pub fn forget(g: &mut Game) {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    if let Some(s) = g.m.squadthrow.states.get_mut(key) { s.plans.clear(); }
}

/// the throw row lit on the open menu: the men it is asked of, the spot, the can
pub fn lit(g: &Game, seat: &crate::seats::SeatRef) -> Option<(Vec<GuardId>, V3, SquadThrowKind)> {
    let mn = seat.orders.menu.as_ref()?;
    if !(mn.page.as_deref() == Some(THROW_PAGE) || mn.flat == Some(true)) || !(mn.sel >= 0.0) { return None; }
    let crate::orders::OrderTarget::Floor(t) = &mn.target else { return None };
    match g.tab.orders_order_def[*mn.rows.get(mn.sel as usize)?].id.as_str() {
        FRAG_THERE => Some((mn.men.clone(), t.at, SquadThrowKind::Frag)),
        STUN_THERE => Some((mn.men.clone(), t.at, SquadThrowKind::Stun)),
        _ => None,
    }
}

/// what the plan was read off, as far as it moves while a menu is up: where each of ours stands, to half a metre, who is up, and what is left
fn read_of(g: &mut Game, kind: SquadThrowKind) -> String {
    let mut s = js::num_str(left(g, kind));
    let mut at = |p: V3, up: bool| { s.push_str(&format!(";{},{}{}", js::num_str(m::floor(p[0] * 2.0)), js::num_str(m::floor(p[2] * 2.0)), if up { "" } else { "x" })); };
    at(g.ch[g.pl[g.player].ch].pos, !g.pl[g.player].down);
    for &gd in g.guards.iter() {
        if g.gd[gd].team == crate::teams::Team::Operative { at(g.ch[g.gd[gd].ch].pos, g.gd[gd].state != crate::game::GuardState::Dead); }
    }
    s
}

/// once a frame with a menu up: the lit row's plan as it is now
pub fn follow(g: &mut Game) {
    let Some((men, at, kind)) = lit(g, &crate::seats::here(g)) else { return };
    if quick(g, &men, at, kind).is_some() { return; }
    let read = read_of(g, kind);
    let kept = state(g).plans.get(kind).filter(|p| p.at == at && p.men == men && p.read == read).cloned();
    match kept {
        Some(p) => { keep(g, &men, at, kind, p.thrower); }
        None => { fresh(g, &men, at, kind); }
    }
}

/// the nearest man with a straight throw from where he stands that catches none of ours: asked, never kept
fn quick(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind) -> Option<ThrowPlan> {
    if left(g, kind) <= 0.0 { return None; }
    let th = squad_throw_now(g, men, at, kind)?;
    Some(keep(g, men, at, kind, Some(th)))
}

/// The plan for these men at this spot, as it would be given now.
fn plan_for(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind) -> ThrowPlan {
    let now = g.time;
    // (asked of the whole of them, or of the one man it names: the order is the thrower's alone)
    let kept = state(g).plans.get(kind).filter(|p| p.at == at && (p.men == men || matches!((&p.thrower, men), (Some(th), [one]) if th.gd == *one && p.men.contains(one)))).cloned();
    let Some(p) = kept else { return quick(g, men, at, kind).unwrap_or_else(|| fresh(g, men, at, kind)) };
    if p.t == now { return p; }
    if let Some(p) = quick(g, &p.men, at, kind) { return p; }
    if p.refuse.is_none() && left(g, kind) > 0.0 {
        if let Some(th) = p.thrower.as_ref().and_then(|was| throw_again(g, was, at, kind)) { return keep(g, &p.men, at, kind, Some(th)); }
    }
    fresh(g, &p.men, at, kind)
}

fn fresh(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind) -> ThrowPlan {
    let th = if left(g, kind) > 0.0 { squad_throw_plan(g, men, at, kind, None, None) } else { None };
    keep(g, men, at, kind, th)
}

/// what is said and shown of that throw; kept for whoever asks next
fn keep(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind, thrower: Option<SquadThrower>) -> ThrowPlan {
    let (mut refuse, mut warn, mut walk) = (None, None, None);
    match &thrower {
        None => refuse = Some(String::from(if left(g, kind) > 0.0 { "no throw from here." } else { none(g, kind).as_str() })),
        Some(th) => {
            if kind == SquadThrowKind::Frag {
                let stands = th.post.unwrap_or(g.ch[g.gd[th.gd].ch].pos);
                refuse = squad_frag_catches(g, th.rest, at, Some(SquadFragCatchesMoved { gd: th.gd, at: stands }), Some(false), None, None);
                if squad_frag_on_sam(g, th.rest, at) { warn = Some(String::from(" · too close: you")); }
            } else {
                let who = squad_stun_catches(g, th.rest, at);
                if !who.is_empty() { warn = Some(format!(" · too close: {}", who.join(", "))); }
            }
            if let Some(post) = th.post {
                // (the way there is the dear part of it: kept while the man and the spot are the same)
                let had = state(g).plans.get(kind).and_then(|p| p.thrower.as_ref().filter(|t| t.gd == th.gd && t.post == Some(post)).and(p.walk.clone()));
                walk = Some(match had { Some(w) => w, None => walk_line(g, th.gd, post) });
            }
        }
    }
    let p = ThrowPlan { at, men: men.to_vec(), t: g.time, read: read_of(g, kind), thrower, refuse, warn, walk };
    state(g).plans.set(kind, p.clone());
    p
}

/// The row's words, with the warning of the plan for this spot.
pub fn row_label(g: &mut crate::game::Game, kind: crate::squadthrow::SquadThrowKind, at: V3) -> String {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let warn = g.m.squadthrow.states.get(key).and_then(|s| s.plans.get(kind)).filter(|p| p.at == at).and_then(|p| p.warn.clone()).unwrap_or_default();
    let n = left(g, kind);
    format!("{} there · {} left{}", word(kind).as_str(), n, warn)
}

/// The order is one man's, the thrower's.
pub fn row_only(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind) -> Vec<GuardId> {
    match plan_for(g, men, at, kind).thrower { Some(th) => vec![th.gd], None => Vec::new() }
}

/// Err: why not.
pub fn row_valid(g: &mut crate::game::Game, men: &[GuardId], at: V3, kind: crate::squadthrow::SquadThrowKind) -> Result<(), String> {
    match plan_for(g, men, at, kind).refuse { Some(why) => Err(why), None => Ok(()) }
}

/// Some: the refusal.
pub fn row_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget, kind: crate::squadthrow::SquadThrowKind) -> Option<String> {
    let at = crate::orders::rows::at(t);
    let p = plan_for(g, men, at, kind);
    if let Some(why) = p.refuse {
        g.stats.squadthrow.refused += 1;
        return Some(why);
    }
    let mut th = p.thrower.expect("a plan that is not refused has a thrower");
    if th.post.is_some() {
        start_walk(g, &mut th, t, kind);
        return None;
    }
    let what = if kind == SquadThrowKind::Frag { crate::doortools::DoorToolsTakeWhat::Frag } else { crate::doortools::DoorToolsTakeWhat::Bang };
    if !crate::doortools::door_tools_take(g, what) { return Some(String::from(none(g, kind).as_str())); }
    let arc = Arc { from: th.from, sol: th.sol, rest: th.rest };
    throw_it(g, th.gd, &arc, kind, at);
    squad_log_row(g, th.gd, row_of(kind).as_str(), at);
    None
}
