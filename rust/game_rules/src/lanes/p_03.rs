//! src/game/lanes.ts lines 807 to 1016
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// No state yet, or no lane in it (`!st || !st.lanes.length`).
fn no_lanes(g: &Game) -> bool { g.m.lanes.states.as_ref().map_or(true, |st| st.lanes.is_empty()) }

/// The list of lanes, for a walk whose body takes `g`. An empty list where there is no state.
fn lane_ids(g: &Game) -> Vec<LanesLaneId> { g.m.lanes.states.as_ref().map_or_else(Vec::new, |st| st.lanes.clone()) }

/// `st.lanes.find(x => x.id === id)`
fn find_lane(g: &Game, id: f64) -> Option<LanesLaneId> {
    let st = g.m.lanes.states.as_ref()?;
    st.lanes.iter().copied().find(|&l| g.tab.lanes_lane[l].id == id)
}

/// A key for a `Set<number>` of cells: JavaScript's Set counts -0 as +0, `Num` goes by the bits.
fn cell_key(i: f64) -> crate::Num { crate::Num(i + 0.0) }

fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn mode_of(g: &Game, d: RoomfightDefId) -> crate::roomfight::DefMode { g.tab.roomfight_def[d].mode }
fn is_dead(g: &Game, gd: GuardId) -> bool { g.gd[gd].state == crate::game::GuardState::Dead }
/// `gd.path = []; gd.pathGoal = null`
fn clear_path(g: &mut Game, gd: GuardId) { crate::game::guard_set_path(g, gd, Vec::new()); crate::guards::guard_set_path_goal(g, gd, None); }
/// `g.say(gd, text, true)`
fn say_net(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, Some(true), None, None); }
/// `g.col.segmentBlocked(a, b)`
fn seg_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::segment_blocked(g, a, b, None, None, None) }
/// `g.col.solidBlocked(a, b)`
fn solid_blocked(g: &mut Game, a: V3, b: V3) -> bool { crate::scene_collision::solid_blocked(g, a, b, None) }
/// `L.guns.every(m => spotSeen(g, eye, m))`
fn all_spots_seen(g: &mut Game, l: LanesLaneId, eye: V3) -> bool {
    let guns = g.tab.lanes_lane[l].guns.clone();
    for q in guns { if !spot_seen(g, eye, q) { return false; } }
    true
}
/// (lanes.ts:818 back) on his way back to where he stood. The row is there at every call: nothing between peek_step's read and a back() deletes it, and :861 puts it in again first
fn peek_back(g: &mut Game, gd: GuardId, d: RoomfightDefId) {
    let now = g.time;
    let home = { let r = s(g).peek.get_mut(gd).unwrap(); r.phase = crate::lanes::PeekPhase::Back; r.t = now; r.home };
    clear_path(g, gd);
    let fast = g.k.lanes.push_fast;
    crate::pressure::run(g, d, home, None, Some(fast));
}

/// every frame, a man out for a look: one of them in sight by the guns = the lane stands; a line down it and nobody there for `peekLook` = clear, forgotten, and the men held on it go on together (lanes.ts peekStep)
pub fn peek_step(g: &mut crate::game::Game, gd: GuardId) {
    use crate::lanes::{PeekPhase, StateHeldValue};
    // `p` is the row in the TypeScript. This copy is kept equal to it: every write below goes to both, and pin_step and frag_step (which write the row by path) end this call
    let mut p = match s(g).peek.get(gd) { Some(&p) => p, None => return };
    let ll = find_lane(g, p.lane);
    let dd = def_of(g, gd);
    let now = g.time;
    if is_dead(g, gd) {
        peek_off(g, gd);
        if p.frag.is_some() {
            s(g).stats.frag_lost += 1.0;
            if let Some(l) = ll { let gap = g.k.lanes.frag_gap; let r = &mut g.tab.lanes_lane[l]; r.frag_at = m::max(r.frag_at, now + gap); let id = r.id; note(g, Some(gd), &format!("lane {}: the grenade cost him", js::num_str(id))); }
        } else if p.fire.is_some() {
            s(g).stats.pin_lost += 1.0;
            if let Some(l) = ll { let rest = g.k.lanes.pin_rest; let r = &mut g.tab.lanes_lane[l]; r.pin_n = 0.0; r.pin_at = now + rest; let id = r.id; note(g, Some(gd), &format!("lane {}: firing down it cost him", js::num_str(id))); }
        } else {
            s(g).stats.peek_lost += 1.0;
            if let Some(l) = ll { let lost = g.k.lanes.peek_lost; let r = &mut g.tab.lanes_lane[l]; r.peek_n += 1.0; r.peek_at = now + lost; let id = r.id; note(g, Some(gd), &format!("lane {}: the look cost him", js::num_str(id))); }
        }
        return;
    }
    let off = match dd { Some(_) => !fit(g, gd) || pushing(g, gd), None => true };
    if off {
        peek_off(g, gd);
        if let Some(f) = p.frag {   // (blind, folded, or in behind a charge with a grenade errand on him: asked again soon; a man who never left his corner is still held on the lane)
            if let Some(l) = ll {
                g.tab.lanes_lane[l].frag_at = now + if f.at.is_some() { 6.0 } else { 2.0 };
                let here = pos_of(g, gd);
                if dd.is_some() && !f.going && flat(here, p.home) < 0.5 { let lane = g.tab.lanes_lane[l].id; held_on(g, gd, StateHeldValue { lane, t: now, look: p.look, aimed: true, at: here }); }
            }
        } else if let Some(l) = ll {   // (no longer this errand's man: the gate has his feet again, and the next look waits its gap)
            let gap = g.k.lanes.peek_gap;
            let r = &mut g.tab.lanes_lane[l];
            r.peek_at = m::max(r.peek_at, now + gap);
        }
        return;
    }
    let d = match dd { Some(d) => d, None => return };
    if ll.is_none() && p.phase != PeekPhase::Back { peek_back(g, gd, d); return; }   // (the lane gone meanwhile: back to where he stood)
    if p.phase == PeekPhase::Back {
        if mode_of(g, d) == crate::roomfight::DefMode::Move && flat(pos_of(g, gd), p.home) > 0.5 && g.time - p.t < 6.0 { return; }
        peek_off(g, gd);
        let here = pos_of(g, gd);
        if lane_on(g, here, gd).is_some() { return; }   // (stopped in the line: the fight's, and the tick's own way out of a line)
        let l = match ll { Some(l) => l, None => return };   // (his own look cleared it: nothing left to hold him on)
        if g.time - g.gd[gd].last_seen_t > 1.0 && crate::roomfight::room_fight_lane_stop(g, gd, p.look) {
            let (at, now) = (pos_of(g, gd), g.time);
            let lane = g.tab.lanes_lane[l].id;
            held_on(g, gd, StateHeldValue { lane, t: now, look: p.look, aimed: true, at });
        }
        return;
    }
    let l = match ll { Some(l) => l, None => return };
    if p.frag.is_some() { frag_step(g, gd, l, d); return; }
    // his head round the corner ahead of his feet: the look is taken from his eyes leant `peekLean` toward the mouth (nothing solid between the two)
    let mut eye = eye_of(g, gd);
    {
        let here = pos_of(g, gd);
        let (dx, dz) = (p.to[0] - here[0], p.to[2] - here[2]);
        let dl = m::hyp2(dx, dz);
        if dl > 0.05 && p.line_t < 0.0 {
            let lean = g.k.lanes.peek_lean;
            let out: V3 = [eye[0] + dx / dl * lean, eye[1], eye[2] + dz / dl * lean];
            if !solid_blocked(g, eye, out) { eye = out; }
            p.eye = Some(eye);
            if let Some(r) = s(g).peek.get_mut(gd) { r.eye = Some(eye); }
        } else if let (Some(e), true) = (p.eye, p.line_t >= 0.0) { eye = e; }
    }
    let seen = guns_manned(g, l, eye);
    // out to FIRE down it: his burst goes unless he can see all of where they stood and nobody is there (then it is a look: clear)
    if p.fire.map_or(false, |f| f.n > 0.0) && (seen.is_some() || !all_spots_seen(g, l, eye)) { pin_step(g, gd, l, d, seen); return; }
    let id = g.tab.lanes_lane[l].id;
    let gap = g.k.lanes.peek_gap;
    if let Some(seen) = seen {
        let (keep, most) = (g.k.lanes.peek_keep, g.k.lanes.peek_max);
        let r = &mut g.tab.lanes_lane[l];
        r.until = m::max(r.until, now + keep);
        r.peek_part = 0.0;
        r.peek_n += 1.0;
        r.peek_at = now + m::min(most, gap * m::pow(2.0, r.peek_n));
        if r.guns.len() < 4 && r.guns.iter().all(|&q| flat(q, seen) > 1.5) { r.guns.push([seen[0], 0.0, seen[2]]); }   // (where they stand now: a place to stop is out of that one's sight too)
        let next = r.peek_at - now;
        s(g).stats.peek_there += 1.0;
        if now - s(g).said_t > 2.0 { s(g).said_t = now; say_net(g, gd, "they're still on it — stay back!"); }
        note(g, Some(gd), &format!("lane {}: a look — still held, from ({}, {}); next no sooner than {} s", js::num_str(id), js::to_fixed(seen[0], 1), js::to_fixed(seen[2], 1), js::to_fixed(next, 0)));
        peek_back(g, gd, d);
        return;
    }
    if mode_of(g, d) != crate::roomfight::DefMode::Move && now - g.gd[gd].last_seen_t < 1.0 { peek_off(g, gd); g.tab.lanes_lane[l].peek_at = now + gap; return; }   // (a man in his sights somewhere else: the fight has him)
    // 'clear' wants the whole of it: the place round EVERY spot in his view, and nobody on their net with eyes on one of them. The spots but not all the floor round them is HALF a look
    let full = all_spots_seen(g, l, eye);
    let done = mode_of(g, d) != crate::roomfight::DefMode::Move || now - p.t > 8.0;
    let mut part = !full && done;
    if part {
        let guns = g.tab.lanes_lane[l].guns.clone();
        for q in guns {
            if seg_blocked(g, eye, [q[0], 1.4, q[2]]) { part = false; break; }
        }
    }
    if (full || part) && net_sees(g, l, gd) {
        let r = &mut g.tab.lanes_lane[l];
        r.peek_at = now + gap;
        r.peek_part = 0.0;
        note(g, Some(gd), &format!("lane {}: a look — nobody in his view, but a mate has eyes on them there: back", js::num_str(id)));
        peek_back(g, gd, d);
        return;
    }
    if full || part {
        if p.line_t < 0.0 {   // (far enough: he looks from here)
            p.line_t = now;
            p.to = pos_of(g, gd);
            let (line_t, to) = (p.line_t, p.to);
            if let Some(r) = s(g).peek.get_mut(gd) { r.line_t = line_t; r.to = to; }
            clear_path(g, gd);
            crate::pressure::run(g, d, to, None, Some(0.0));
        }
        if now - p.line_t < g.k.lanes.peek_look { return; }
        if !full {
            let sure = g.k.lanes.peek_sure;
            let r = &mut g.tab.lanes_lane[l];
            r.peek_part += 1.0;
            if r.peek_part < 2.0 || now - r.last_t < sure {
                r.peek_at = now + gap;
                let n = r.peek_part;
                note(g, Some(gd), &format!("lane {}: a look — nobody in his view, but not all of where they stood ({}): back", js::num_str(id), js::num_str(n)));
                peek_back(g, gd, d);
                return;
            }
        }
        peek_off(g, gd);
        {
            let st = s(g);
            st.stats.peek_clear += 1.0;
            st.stats.lapsed += 1.0;
            st.lanes.retain(|&x| x != l);
        }
        lanes_gone(g);
        s(g).due.retain(|x| x.lane != id);
        remask(g);   // (forgotten here and now: nobody is stopped short of it on his way out)
        let mut mates: Vec<GuardId> = Vec::new();
        let mut at = 0usize;
        while let Some(o) = s(g).held.next_key(&mut at) {
            if s(g).held.get(o).map_or(false, |h| h.lane == id) {
                held_off(g, o);
                if o != gd && fit(g, o) { mates.push(o); }
            }
        }
        let (downs, last_t, from) = { let r = &g.tab.lanes_lane[l]; (r.downs, r.last_t, r.from) };   // (the row outlives its place on the list)
        let tail = if !mates.is_empty() { format!(", {} move up together", mates.len() + 1) } else { String::from(", back to his place") };
        note(g, Some(gd), &format!("lane {}: a look — nobody on it: clear{}", js::num_str(id), tail));
        note(g, None, &format!("lane {}: forgotten ({} down, last round {} s ago)", js::num_str(id), js::num_str(downs), js::to_fixed(now - last_t, 0)));
        // with a mate held on it too they go on TOGETHER to where the guns were; a man by himself goes nowhere on it: back to his place
        if !mates.is_empty() {
            say_net(g, gd, "they're off it — move up, together!");
            for &o in mates.iter() { if let Some(dm) = def_of(g, o) { crate::pressure::loose(g, dm, from); } }
            crate::pressure::loose(g, d, from);
        } else {
            say_net(g, gd, "they've moved off it — eyes open!");
            peek_on(g, gd, p);
            peek_back(g, gd, d);
        }
        s(g).said_t = now;
        return;
    }
    if done {
        g.tab.lanes_lane[l].peek_at = now + gap;
        note(g, Some(gd), &format!("lane {}: no line down it from its mouth — back", js::num_str(id)));
        peek_back(g, gd, d);
    }
}

/// where a lane opens before a man standing at `p`: the nearest of its floor he has a line to, of the ten nearest; else the nearest (lanes.ts mouthOf)
pub fn mouth_of(g: &mut crate::game::Game, l: LanesLaneId, p: V3) -> V3 {
    let (cell, w) = (g.col.nav.cell, g.col.nav.w);
    let mut near: Vec<(f64, f64, f64)> = Vec::new();   // (d, x, z)
    if let Some(cells) = g.tab.lanes_lane[l].cells.as_ref() {
        for i in cells.iter() {
            let i = i.0;
            let x = (js::rem(i, w) + 0.5) * cell;
            let z = (js::to_i32(i / w) as f64 + 0.5) * cell;
            let d = m::hyp2(x - p[0], z - p[2]);
            if near.len() >= 10 && d >= near[9].0 { continue; }
            let mut k = near.len();
            while k > 0 && near[k - 1].0 > d { k -= 1; }
            near.insert(k, (d, x, z));
            if near.len() > 10 { near.pop(); }
        }
    }
    if near.is_empty() { return g.tab.lanes_lane[l].at; }
    let eye: V3 = [p[0], 1.5, p[2]];
    for &(_, x, z) in near.iter() { if !solid_blocked(g, eye, [x, 1.0, z]) { return [x, 0.0, z]; } }
    [near[0].1, 0.0, near[0].2]
}

/// the price of a walk with this mask's floor made dear, on top of `base` (lanes.ts priceOf)
pub fn price_of(g: &crate::game::Game, m: Option<&[u8]>, base: Option<&crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> {
    use crate::lanes::LanePenalty;
    let Some(mask) = m else { return base.map(|b| LanePenalty::Base { base: b.clone() }) };
    let (w, dear) = (g.col.nav.w, g.k.lanes.dear);
    Some(match base {
        Some(b) => LanePenalty::DearOver { base: b.clone(), m: mask.to_vec(), w, dear },
        None => LanePenalty::Dear { m: mask.to_vec(), w, dear },
    })
}

/// (lanes.ts:880) `m[iz * W + ix] ? d : 1`: a cell off the mask costs 1, as a Uint8Array's undefined does
fn dear_at(mask: &[u8], w: f64, dear: f64, ix: f64, iz: f64) -> f64 {
    let i = iz * w + ix;
    if i >= 0.0 && mask.get(i as usize).map_or(false, |&b| b != 0) { dear } else { 1.0 }
}

/// what a LanePenalty answers for a cell: the closure priceOf returns; the tactics' price is asked first (lanes.ts:880)
pub fn lane_penalty_at(pen: &crate::lanes::LanePenalty, ix: f64, iz: f64) -> f64 {
    use crate::lanes::LanePenalty;
    match pen {
        LanePenalty::Dear { m, w, dear } => dear_at(m, *w, *dear, ix, iz),
        LanePenalty::DearOver { base, m, w, dear } => { let b = crate::tactics::tactics_penalty_price(base, ix, iz); m::max(b, dear_at(m, *w, *dear, ix, iz)) }
        LanePenalty::Base { base } => crate::tactics::tactics_penalty_price(base, ix, iz),
    }
}

/// guards.ts goTo: the price of a walk for this man: `base` with the floor of every lane his side knows made dear (lanes.ts lanePenalty)
pub fn lane_penalty(g: &mut crate::game::Game, gd: GuardId, base: Option<&crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> {
    if g.gd[gd].script.as_ref().and_then(|x| x.direct) == Some(true) { return price_of(g, None, base); }   // (a walk a level's script sent the short way)
    if no_lanes(g) || !live(g) || !mine(g, gd) || pushing(g, gd) { return price_of(g, None, base); }
    let mask = mask_of(g, gd);
    price_of(g, mask.as_deref(), base)
}

/// True = this rule has his feet this frame: his way lies down a lane his side knows is covered, and he has been stopped short of it. (lanes.ts laneGate)
pub fn lane_gate(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].script.is_some() && crate::director::director_lesson_runs(g) { return false; }
    if no_lanes(g) || !live(g) || !mine(g, gd) || g.gd[gd].path.is_empty() || pushing(g, gd) { return false; }
    let Some(m) = mask_of(g, gd) else { return false };
    if flanking(g, gd) || s(g).peek.has(gd) { return false; }
    let sg = g.gd[gd].script.as_ref().and_then(|sc| sc.goal);
    if let (Some(h), Some(sg)) = (s(g).held.get(gd), sg) {
        if !h.aimed && flat(sg, h.at) < 0.8 { return false; }
    }
    let last_look = s(g).look.get(gd).copied().unwrap_or(-100.0);
    if g.time - last_look < 0.2 { return false; }
    let now = g.time;
    s(g).look.set(gd, now);
    if close_fight(g, gd) || crate::control::man::blind(g, gd) || crate::frag::frag_flee(g, gd).is_some() { return false; }
    let here = in_mask(g, Some(&m), pos_of(g, gd));
    let Some(mut first) = ahead(g, &m, gd, here) else { return false };
    if !here {
        if let Some(goal) = g.gd[gd].path_goal {
            crate::guards::go_to(g, gd, goal, Some(true));
            let Some(again) = ahead(g, &m, gd, false) else {
                s(g).stats.rerouted += 1.0;
                note(g, Some(gd), "a covered lane on his way: goes round it");
                return false;
            };
            first = again;
        }
    }
    let Some(l) = lane_at(g, first, gd) else { return false };
    let (lane_id, lane_from) = (g.tab.lanes_lane[l].id, g.tab.lanes_lane[l].from);
    if here {
        if g.time - g.tab.lanes_lane[l].last_t > g.k.lanes.hot_secs {
            let on = along(g, gd, g.k.lanes.ahead_m);
            if flat(on, lane_from) > flat(pos_of(g, gd), lane_from) - 0.5 { return false; }
        }
        if back_out(g, gd, l) { return true; }
    } else {
        let p = pos_of(g, gd);
        let shift = g.k.lanes.shift_m;
        if (gun_on(g, l, p, Some(shift)) || crate::doorway::doorway_in_way(g, p)) && back_out(g, gd, l) { return true; }
    }
    if def_of(g, gd).is_none() { return false; }
    if !s(g).due.iter().any(|d| d.gd == gd) {
        let mut look = first;
        let (p, look_m) = (pos_of(g, gd), g.k.lanes.look_m);
        if flat(p, first) < look_m {
            let far = along(g, gd, 3.0);
            look = if flat(p, far) >= look_m { far } else if flat(p, lane_from) >= look_m { lane_from } else { first };
        }
        s(g).due.push(StateDueItem { gd, lane: lane_id, look });
    }
    g.gd[gd].speed = 0.0;
    let c = g.gd[gd].ch;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// The point `m` metres on along his way. (lanes.ts along)
pub fn along(g: &crate::game::Game, gd: GuardId, m: f64) -> V3 {
    let mut a = pos_of(g, gd);
    let mut left = m;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() {
        let b = g.gd[gd].path[i];
        let d = flat(a, b);
        if d >= left { return [a[0] + (b[0] - a[0]) * left / js::or(d, 1.0), 0.0, a[2] + (b[2] - a[2]) * left / js::or(d, 1.0)]; }
        left -= d;
        a = b;
        i += 1;
    }
    a
}

/// He stops where he stands, short of a lane, his gun on `look`. (lanes.ts plantShort)
pub fn plant_short(g: &mut crate::game::Game, gd: GuardId, lane: f64, look: V3) {
    if !mine(g, gd) { return; }
    let mut named = false;
    for l in lane_ids(g) {
        if g.tab.lanes_lane[l].id == lane && knows(g, l, gd) { named = true; break; }
    }
    if !named { return; }
    if !crate::roomfight::room_fight_lane_stop(g, gd, look) { return; }
    crate::game::guard_set_path(g, gd, Vec::new());
    crate::guards::guard_set_path_goal(g, gd, None);
    let (p, now) = (pos_of(g, gd), g.time);
    held_on(g, gd, StateHeldValue { lane, t: now, look, aimed: true, at: p });
    s(g).stats.held += 1.0;
    if now - s(g).said_t > 5.0 {
        s(g).said_t = now;
        crate::game::game_say(g, gd, "holding here — I'm not walking into that!", Some(true), None, None);
    }
    let p = pos_of(g, gd);
    note(g, Some(gd), &format!("lane {}: stops short of it at ({}, {})", js::num_str(lane), js::to_fixed(p[0], 1), js::to_fixed(p[2], 1)));
}

/// The first point of a known lane on the next `aheadM` of his way; `inside`: he stands in one, and None means his way leaves it for good within reach. (lanes.ts ahead)
pub fn ahead(g: &crate::game::Game, m: &[u8], gd: GuardId, inside: bool) -> Option<V3> {
    let mut a = pos_of(g, gd);
    let mut left = g.k.lanes.ahead_m;
    let mut was = inside;
    let mut hit: Option<V3> = None;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() && left > 0.0 {
        let b = g.gd[gd].path[i];
        let d = flat(a, b);
        let k = m::max(1.0, m::ceil(m::min(d, left) / 0.35));
        let mut j = 1.0;
        while j <= k {
            let t = m::min(1.0, j * 0.35 / js::or(d, 1.0));
            let p: V3 = [a[0] + (b[0] - a[0]) * t, 0.0, a[2] + (b[2] - a[2]) * t];
            let on = in_mask(g, Some(m), p);
            if on && hit.is_none() { hit = Some(p); }
            if inside && was && !on { return None; }
            was = on;
            j += 1.0;
        }
        left -= d;
        a = b;
        i += 1;
    }
    if hit.is_some() { return hit; }
    if inside { Some(pos_of(g, gd)) } else { None }
}

/// He is held short of a lane his side knows is covered, and what wants him (`to`; none = anything) lies at its guns or down it. (lanes.ts laneBars)
pub fn lane_bars(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool {
    if crate::gunroles::gun_keeps_post(g, gd) { return true; }
    if g.m.lanes.states.is_none() || !live(g) || !crate::owner::has(g, gd, crate::owner::by::LANE) { return false; }
    let Some(l) = lane_of(g, gd) else { return false };
    let Some(to) = to else { return true };
    let field = g.k.lanes.field_m;
    if g.tab.lanes_lane[l].guns.iter().any(|m| flat(*m, to) < field + 4.0) { return true; }
    let i = cell_at(g, to);
    g.tab.lanes_lane[l].cells.as_ref().map_or(false, |c| c.has(cell_key(i)))
}

/// He has shouted for a lane and the wind-up still has his arm: the dig-in does not adopt him. (lanes.ts laneThrows)
pub fn lane_throws(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    if st.peek.is_empty() { return false; }
    if !live(g) { return false; }
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    let Some(f) = st.peek.get(gd).and_then(|p| p.frag) else { return false };
    winding(g, gd, f)
}

/// The noise he would walk over to by himself is at the guns of a lane he knows is covered, or down it. (lanes.ts laneKnownAt)
pub fn lane_known_at(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool {
    let Some(to) = to else { return false };
    if no_lanes(g) || !live(g) || !mine(g, gd) { return false; }
    let i = cell_at(g, to);
    let field = g.k.lanes.field_m;
    for l in lane_ids(g) {
        if !knows(g, l, gd) { continue; }
        let r = &g.tab.lanes_lane[l];
        let Some(cells) = r.cells.as_ref() else { continue };
        if cells.has(cell_key(i)) || r.guns.iter().any(|m| flat(*m, to) < field + 4.0) { return true; }
    }
    false
}

/// The far side of this doorway is in the line of guns his side knows of: no drill through it, unless his charge will burst on those guns. (lanes.ts laneDoor)
pub fn lane_door(g: &mut crate::game::Game, gd: GuardId, far: V3, centre: V3, can: bool) -> bool {
    if no_lanes(g) || !live(g) || !mine(g, gd) || pushing(g, gd) || flanking(g, gd) { return false; }
    let Some(l) = lane_on(g, far, gd) else { return false };
    let bang = g.k.lanes.bang_m;
    !(can && g.tab.lanes_lane[l].guns.iter().any(|m| flat(*m, centre) < bang + 2.5))
}

/// The two questions tryEngage asked, asked again while the drill has him this side of the doorway. True = the drill lets him go. (lanes.ts laneDrill)
pub fn lane_drill(g: &mut crate::game::Game, gd: GuardId, at: V3, far: V3, centre: V3, can: bool) -> bool {
    if !lane_covers(g, gd, at) && !lane_door(g, gd, far, centre, can) { return false; }
    note(g, Some(gd), "the doorway drill has him at a doorway in a lane his side knows of: it lets him go");
    true
}

/// This spot is in the line of guns his side knows of: no place to stack on a doorway. (lanes.ts laneCovers)
pub fn lane_covers(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool {
    if no_lanes(g) || !live(g) || !mine(g, gd) { return false; }
    !pushing(g, gd) && !flanking(g, gd) && lane_on(g, p, gd).is_some()
}

/// He stands in a lane his side knows is covered: no place to plant himself. (lanes.ts laneExposed)
pub fn lane_exposed(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.m.lanes.states.is_none() || !live(g) || !mine(g, gd) { return false; }
    if pushing(g, gd) || flanking(g, gd) { return false; }
    if no_lanes(g) || close_fight(g, gd) { return false; }
    let p = pos_of(g, gd);
    lane_on(g, p, gd).is_some()
}

/// A new state, the counters kept; every man this file had is let go of first. (lanes.ts lanesReset)
pub fn lanes_reset(g: &mut crate::game::Game) {
    let Some(stats) = g.m.lanes.states.as_ref().map(|st| st.stats) else { return };
    let_go(g);
    let mut st = fresh(g);
    st.stats = stats;
    g.m.lanes.states = Some(st);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_cell_of_a_lane_is_dear() {
        let pen = crate::lanes::LanePenalty::Dear { m: vec![0u8, 1, 0, 0], w: 2.0, dear: 14.0 };
        assert_eq!(lane_penalty_at(&pen, 1.0, 0.0), 14.0);
        assert_eq!(lane_penalty_at(&pen, 0.0, 1.0), 1.0);
    }

    #[test]
    fn a_cell_off_the_mask_costs_one() {
        let pen = crate::lanes::LanePenalty::Dear { m: vec![1u8, 1, 1, 1], w: 2.0, dear: 14.0 };
        assert_eq!(lane_penalty_at(&pen, 0.0, 2.0), 1.0);
        assert_eq!(lane_penalty_at(&pen, -1.0, 0.0), 1.0);
    }

    #[test]
    fn a_negative_zero_cell_is_cell_zero() {
        assert!(cell_key(-0.0) == cell_key(0.0));
    }
}
