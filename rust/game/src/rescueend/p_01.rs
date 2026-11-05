//! src/game/rescueend.ts lines 1 to 125
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The distance across the floor.   (rescueend.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// The player a row is built for and a press is read from.   (rescueend.ts who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { g.player }

/// Nobody led, nobody counted.   (rescueend.ts RescueEnd.constructor)
pub fn rescue_end_new(g: &mut crate::game::Game) -> crate::rescueend::RescueEnd { crate::rescueend::RescueEnd::blank() }

/// A fresh staging: nobody led, nobody counted.   (rescueend.ts RescueEnd.reset)
pub fn rescue_end_reset(g: &mut crate::game::Game) {
    let e = &mut g.rescue.ending;
    e.lead = None;
    e.led = false;
    e.clear_at = f64::NAN;
}

/// The way out: the mark the team was staged on.   (rescueend.ts RescueEnd.out)
pub fn rescue_end_out(g: &mut crate::game::Game) -> Option<V3> {
    let d = crate::rescue::rescue_data(g)?;
    Some([d.team.sam[0], 0.0, d.team.sam[2]])
}

/// The men of theirs still to be dealt with: not down, and not run off with the gun dropped.   (rescueend.ts RescueEnd.left)
pub fn rescue_end_left(g: &mut crate::game::Game) -> Vec<GuardId> {
    let all = g.guards.clone();
    let mut left = Vec::new();
    for h in all {
        if g.gd[h].civ.is_none()
            && crate::teams::hostile_to(g, h, crate::teams::HostileToO::Player)
            && g.gd[h].state != crate::game::GuardState::Dead
            && !crate::runner::runner_fled(g, h)
        {
            left.push(h);
        }
    }
    left
}

/// The hostages still alive.   (rescueend.ts RescueEnd.alive)
pub fn rescue_end_alive(g: &mut crate::game::Game) -> Vec<GuardId> {
    let mut alive = Vec::new();
    for &h in g.rescue.hostages.iter() {
        if g.gd[h].state != crate::game::GuardState::Dead { alive.push(h); }
    }
    alive
}

/// A word can be given to the hostages: the run is on and nobody of theirs has them.   (rescueend.ts RescueEnd.free)
pub fn rescue_end_free(g: &mut crate::game::Game) -> bool {
    g.rescue.active && g.rescue.result.is_none() && crate::rescue::rescue_captor(g).is_none()
}

/// The row on each living hostage in reach.   (rescueend.ts RescueEnd.rows)
pub fn rescue_end_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::rescueend::RescueEndRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    if !g.rescue.active || g.rescue.result.is_some() { return; }
    let pl = who(g);
    let had = crate::rescue::rescue_captor(g).is_some();
    for h in rescue_end_alive(g) {
        let hp = g.ch[g.gd[h].ch].pos;
        let d = flat(p, hp);
        if d > g.k.rescueend.marker_m || crate::fogofwar::fog_of_war_hidden(g, h) { continue; }
        let near = d < g.k.rescueend.reach_m && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [hp[0], 1.0, hp[2]], None);
        let can = near && !g.pl[pl].sprinting;
        let c = g.gd[h].civ.unwrap();
        let line1 = alloc::format!("{} · the {}", g.cv[c].name, g.cv[c].label);
        let line2 = if had {
            alloc::format!("one of them has a gun on {}", g.cv[c].her)
        } else if can {
            String::new()
        } else if near {
            String::from("stop sprinting")
        } else {
            String::from("get closer")
        };
        let it = crate::game::Interactable {
            kind: crate::game::InteractableKind::Guard,
            pos: v3::add(hp, [0.0, 1.35, 0.0]),
            line1,
            line2,
            in_reach: can && !had,
            off: false,
            guard: Some(h),
            ..crate::game::Interactable::zeroed()
        };
        if had { offer(g, it, Vec::new()); continue; }
        let verbs = if can && !busy {
            let v = if g.rescue.ending.lead.is_some() {
                row(g, it.clone(), crate::game::Verb::Stay, String::from("stay here"), None, None)
            } else {
                row(g, it.clone(), crate::game::Verb::Lead, String::from("on your feet — with me"), None, None)
            };
            vec![v]
        } else {
            Vec::new()
        };
        offer(g, it, verbs);
    }
}

/// The word: the living hostages follow `by`, or (None) stay where they stand. False = refused.   (rescueend.ts RescueEnd.word)
pub fn rescue_end_word(g: &mut crate::game::Game, by: Option<CharId>) -> bool {
    if !rescue_end_free(g) { return false; }
    let (gap, step) = (g.k.rescueend.gap_m, g.k.rescueend.gap_step_m);
    for (i, h) in rescue_end_alive(g).into_iter().enumerate() {
        crate::civilians::hostage_lead(g, h, by, gap + i as f64 * step);
    }
    g.rescue.ending.lead = by;
    if by.is_some() { g.rescue.ending.led = true; }
    crate::rescue::rescue_note(g, if by.is_some() { "lead" } else { "stay" });
    true
}

/// Once a frame: this frame's Space on a hostage's row; at the run's end they stop where they are.   (rescueend.ts RescueEnd.tick)
pub fn rescue_end_tick(g: &mut crate::game::Game) {
    if !g.rescue.active { return; }
    let act = g.act.as_ref().map(|a| (a.kind, a.guard, a.verb));
    if let Some((crate::game::InteractableKind::Guard, Some(guard), Some(verb))) = act {
        if (verb == crate::game::Verb::Lead || verb == crate::game::Verb::Stay) && crate::rescue::rescue_is_hostage(g, guard) {
            let by = if verb == crate::game::Verb::Lead { let pl = who(g); Some(g.pl[pl].ch) } else { None };
            rescue_end_word(g, by);
        }
    }
    if g.rescue.ending.lead.is_some() && g.rescue.result.is_some() {
        for h in rescue_end_alive(g) { crate::civilians::hostage_lead(g, h, None, 0.0); }
        g.rescue.ending.lead = None;
    }
}

/// Has the run been won this frame, and how; None = not yet.   (rescueend.ts RescueEnd.won)
pub fn rescue_end_won(g: &mut crate::game::Game) -> Option<crate::rescueend::RescueEndWon> {
    let alive = rescue_end_alive(g);
    if alive.is_empty() { return None; }
    if !rescue_end_left(g).is_empty() {
        g.rescue.ending.clear_at = f64::NAN;
    } else {
        if g.rescue.ending.clear_at.is_nan() {
            g.rescue.ending.clear_at = g.time;
            crate::rescue::rescue_note(g, "cleared");
        }
        if g.time - g.rescue.ending.clear_at >= g.k.rescueend.clear_secs { return Some(crate::rescueend::RescueEndWon::Cleared); }
    }
    let o = rescue_end_out(g);
    if let (Some(o), true) = (o, g.rescue.ending.lead.is_some()) {
        let out_m = g.k.rescueend.out_m;
        if alive.iter().all(|&h| flat(g.ch[g.gd[h].ch].pos, o) < out_m) { return Some(crate::rescueend::RescueEndWon::Out); }
    }
    None
}

/// The objective row once the room is the team's or the hostages are on their feet.   (rescueend.ts RescueEnd.objective)
pub fn rescue_end_objective(g: &mut crate::game::Game) -> crate::rescueend::RescueEndObjective {
    if g.rescue.ending.lead.is_some() {
        return crate::rescueend::RescueEndObjective { text: String::from("objective ▸ bring them out the way you came in"), pos: rescue_end_out(g) };
    }
    let pl = who(g);
    let p = g.ch[g.pl[pl].ch].pos;
    let mut near: Option<V3> = None;
    for h in rescue_end_alive(g) {
        let b = g.ch[g.gd[h].ch].pos;
        near = match near {
            Some(a) if !(flat(b, p) < flat(a, p)) => Some(a),
            _ => Some(b),
        };
    }
    crate::rescueend::RescueEndObjective {
        text: String::from("objective ▸ bring them out, or clear the building"),
        pos: near.map(|n| [n[0], 0.0, n[2]]),
    }
}

/// Not translated: a save is a clone of Game (conventions 6).   (rescueend.ts RescueEnd.snapshot)
pub fn rescue_end_snapshot(g: &mut crate::game::Game) -> crate::rescueend::RescueEndSnap { todo!("not translated: checkpoint, a clone of Game replaces it") }

/// Not translated: a save is a clone of Game (conventions 6).   (rescueend.ts RescueEnd.restore)
pub fn rescue_end_restore(g: &mut crate::game::Game, s: crate::rescueend::RescueEndSnap) { todo!("not translated: checkpoint, a clone of Game replaces it") }

/// The hostages' rows, from player.ts buildInteractables.   (rescueend.ts rescueEndRows)
pub fn rescue_end_rows_2(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::rescueend::RescueEndRows2RowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    if !g.rescue.active { return; }
    // RescueEndRowsRowOver and RescueEndRows2RowOver are one type in the TypeScript; rows above never passes one, so `over` is always None here.
    rescue_end_rows(g, p, busy, &mut |g, base, verb, text, hints, _over| row(g, base, verb, text, hints, None), offer);
}
