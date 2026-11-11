use crate::v3::dist_xz;
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

/// `L.guns.every(m => spotSeen(g, eye, m))`
fn all_spots_seen(g: &Game, l: LanesLaneId, eye: V3) -> bool {
    let guns = g.tab.lanes_lane[l].guns.clone();
    for q in guns { if !spot_seen(g, eye, q) { return false; } }
    true
}

/// A man out for his look, and what he is looking at: fixed for the frame.
#[derive(Clone, Copy)]
struct Out {
    gd: GuardId,
    l: LanesLaneId,
    d: RoomfightDefId,
    id: f64,
    now: f64,
    gap: f64,
}

/// every frame, a man out for a look: one of them in sight by the guns = the lane stands; a line down it and nobody there for `peekLook` = clear, forgotten, and the men held on it go on together
pub(super) fn peek_step(g: &mut crate::game::Game, gd: GuardId) {
    use crate::lanes::PeekPhase;
    // `p` is kept equal to the row: every write below goes to both, and pin_step and frag_step (which write the row by path) end this call
    let Some(mut p) = s(g).peek.get(gd).copied() else { return };
    let ll = find_lane(g, p.lane);
    let dd = def_of(g, gd);
    let now = g.time;
    if crate::condition::down(g, g.gd[gd].ch) {
        return fell_on_the_errand(g, gd, &p, ll);
    }
    let away = dd.map_or(false, |d| crate::roomfight::on_leg(g, d) && !crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE));   // (sent elsewhere meanwhile)
    let off = away || match dd { Some(_) => !fit(g, gd) || pushing(g, gd), None => true };
    if off {
        return off_the_errand(g, gd, &p, ll, dd.is_some() && !away);
    }
    let Some(d) = dd else { return };
    if ll.is_none() && p.phase != PeekPhase::Back { return peek_back(g, gd, d); }   // (the lane gone meanwhile: back to where he stood)
    if p.phase == PeekPhase::Back {
        return comes_home(g, gd, d, &p, ll);
    }
    let Some(l) = ll else { return };
    if p.frag.is_some() { return frag_step(g, gd, l, d); }
    let eye = eye_round_the_corner(g, gd, &mut p);
    let seen = guns_manned(g, l, eye);
    // out to FIRE down it: his burst goes unless he can see all of where they stood and nobody is there (then it is a look: clear)
    if p.fire.map_or(false, |f| f.n > 0.0) && (seen.is_some() || !all_spots_seen(g, l, eye)) { return pin_step(g, gd, l, d, seen); }
    let id = g.tab.lanes_lane[l].id;
    let gap = g.k.lanes.peek_gap;
    let o = Out { gd, l, d, id, now, gap };
    if let Some(seen) = seen {
        return they_are_still_on_it(g, o, seen);
    }
    if !crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE) && now - g.gd[gd].last_seen_t < 1.0 { peek_off(g, gd); g.tab.lanes_lane[l].peek_at = now + gap; return; }   // (a man in his sights somewhere else: the fight has him)
    // 'clear' wants the whole of it: the place round EVERY spot in his view, and nobody on their net with eyes on one of them. The spots but not all the floor round them is HALF a look
    let full = all_spots_seen(g, l, eye);
    let done = !crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE) || now - p.t > 8.0;
    let part = !full && done && every_gun_spot_in_line(g, l, eye);
    if !full && !part {
        if done {
            g.tab.lanes_lane[l].peek_at = now + gap;
            peek_back(g, gd, d);
        }
        return;
    }
    if net_sees(g, l, gd) {
        let r = &mut g.tab.lanes_lane[l];
        r.peek_at = now + gap;
        r.peek_part = 0.0;
        return peek_back(g, gd, d);
    }
    stops_to_look(g, o, &mut p);
    if now - p.line_t < g.k.lanes.peek_look { return; }
    if !full && half_a_look_will_not_do(g, o) { return; }
    lane_is_clear(g, o, p);
}

/// He went down on it: what he was out for waits, and a look that cost a man waits longer each time.
fn fell_on_the_errand(g: &mut Game, gd: GuardId, p: &crate::lanes::Peek, ll: Option<LanesLaneId>) {
    let now = g.time;
    peek_off(g, gd);
    let Some(l) = ll else { return };
    let k = g.k.lanes;
    let r = &mut g.tab.lanes_lane[l];
    if p.frag.is_some() {
        r.frag_at = m::max(r.frag_at, now + k.frag_gap);
    } else if p.fire.is_some() {
        r.pin_n = 0.0;
        r.pin_at = now + k.pin_rest;
    } else {
        r.peek_n += 1.0;
        r.peek_at = now + k.peek_lost;
    }
}

/// No longer this errand's man: the gate has his feet and the next look waits its gap. With a grenade errand: asked again soon, and a man who never left his corner is still held.
fn off_the_errand(g: &mut Game, gd: GuardId, p: &crate::lanes::Peek, ll: Option<LanesLaneId>, still_his_own: bool) {
    let now = g.time;
    peek_off(g, gd);
    let Some(l) = ll else { return };
    let Some(f) = p.frag else {
        let gap = g.k.lanes.peek_gap;
        let r = &mut g.tab.lanes_lane[l];
        r.peek_at = m::max(r.peek_at, now + gap);
        return;
    };
    g.tab.lanes_lane[l].frag_at = now + if f.at.is_some() { 6.0 } else { 2.0 };
    let here = g.pos_of(gd);
    if still_his_own && !f.going && dist_xz(here, p.home) < 0.5 {
        let lane = g.tab.lanes_lane[l].id;
        held_on(g, gd, crate::lanes::StateHeldValue { lane, t: now, look: p.look, aimed: true, at: here });
    }
}

/// On his way back: once there (or the way is done) the look is over, and he is held on the lane again if it stands and he is out of its line.
fn comes_home(g: &mut Game, gd: GuardId, d: RoomfightDefId, p: &crate::lanes::Peek, ll: Option<LanesLaneId>) {
    if crate::roomfight::on_leg_of(g, d, crate::owner::by::LANE) && dist_xz(g.pos_of(gd), p.home) > 0.5 && g.time - p.t < 6.0 { return; }
    peek_off(g, gd);
    let here = g.pos_of(gd);
    if lane_on(g, here, gd).is_some() { return; }   // (stopped in the line: the fight's, and the tick's own way out of a line)
    let Some(l) = ll else { return };   // (his own look cleared it: nothing left to hold him on)
    if g.time - g.gd[gd].last_seen_t > 1.0 && crate::roomfight::room_fight_lane_stop(g, gd, p.look) {
        let (at, now) = (g.pos_of(gd), g.time);
        let lane = g.tab.lanes_lane[l].id;
        held_on(g, gd, crate::lanes::StateHeldValue { lane, t: now, look: p.look, aimed: true, at });
    }
}

/// His head round the corner ahead of his feet: the look is from his eyes leant `peekLean` toward the mouth (nothing solid between), and from there still once he has his line.
fn eye_round_the_corner(g: &mut Game, gd: GuardId, p: &mut crate::lanes::Peek) -> V3 {
    let eye = eye_of(g, gd);
    let here = g.pos_of(gd);
    let (dx, dz) = (p.to[0] - here[0], p.to[2] - here[2]);
    let dl = m::hyp2(dx, dz);
    if dl > 0.05 && p.line_t < 0.0 {
        let lean = g.k.lanes.peek_lean;
        let out: V3 = [eye[0] + dx / dl * lean, eye[1], eye[2] + dz / dl * lean];
        let eye = if !crate::scene_collision::solid_blocked(g, eye, out, None) { out } else { eye };
        p.eye = Some(eye);
        if let Some(r) = s(g).peek.get_mut(gd) { r.eye = Some(eye); }
        return eye;
    }
    match (p.eye, p.line_t >= 0.0) {
        (Some(e), true) => e,
        _ => eye,
    }
}

/// One of them is in sight by the guns: the lane stands, the next look waits longer, and he gets back.
fn they_are_still_on_it(g: &mut Game, o: Out, seen: V3) {
    let Out { gd, l, d, now, gap, .. } = o;
    let (keep, most) = (g.k.lanes.peek_keep, g.k.lanes.peek_max);
    let r = &mut g.tab.lanes_lane[l];
    r.until = m::max(r.until, now + keep);
    r.peek_part = 0.0;
    r.peek_n += 1.0;
    r.peek_at = now + m::min(most, gap * m::pow(2.0, r.peek_n));
    if r.guns.len() < 4 && r.guns.iter().all(|&q| dist_xz(q, seen) > 1.5) { r.guns.push([seen[0], 0.0, seen[2]]); }   // (where they stand now: a place to stop is out of that one's sight too)
    if now - s(g).said_t > 2.0 { s(g).said_t = now; crate::game::game_say(g, gd, "they're still on it — stay back!", Some(true), None); }
    peek_back(g, gd, d);
}

/// Has he a line to every one of the lane's gun spots.
fn every_gun_spot_in_line(g: &Game, l: LanesLaneId, eye: V3) -> bool {
    let guns = g.tab.lanes_lane[l].guns.clone();
    guns.iter().all(|&q| !crate::scene_collision::segment_blocked(g, eye, [q[0], 1.4, q[2]], None, None, None))
}

/// Far enough: he looks from here.
fn stops_to_look(g: &mut Game, o: Out, p: &mut crate::lanes::Peek) {
    if p.line_t >= 0.0 { return; }
    p.line_t = o.now;
    p.to = g.pos_of(o.gd);
    let (line_t, to) = (p.line_t, p.to);
    if let Some(r) = s(g).peek.get_mut(o.gd) { r.line_t = line_t; r.to = to; }
    g.gd[o.gd].drop_path();
    crate::roomfight::run(g, o.d, to, None, 0.0, crate::owner::by::LANE);
}

/// One half look proves nothing, nor does any on a lane fired down lately: he gets back. True = he goes back.
fn half_a_look_will_not_do(g: &mut Game, o: Out) -> bool {
    let sure = g.k.lanes.peek_sure;
    let r = &mut g.tab.lanes_lane[o.l];
    r.peek_part += 1.0;
    if !(r.peek_part < 2.0 || o.now - r.last_t < sure) {
        return false;
    }
    r.peek_at = o.now + o.gap;
    peek_back(g, o.gd, o.d);
    true
}

/// Clear: the lane is forgotten now, so nobody is stopped short of it. With a mate held on it they go on TOGETHER to where the guns were; a man alone goes back to his place.
fn lane_is_clear(g: &mut Game, o: Out, p: crate::lanes::Peek) {
    let Out { gd, l, d, id, now, .. } = o;
    peek_off(g, gd);
    s(g).lanes.retain(|&x| x != l);
    lanes_gone(g);
    s(g).due.retain(|x| x.lane != id);
    remask(g);
    let mates = let_go_the_men_held_on_it(g, gd, id);
    let from = g.tab.lanes_lane[l].from;   // (the row outlives its place on the list)
    if !mates.is_empty() {
        crate::game::game_say(g, gd, "they're off it — move up, together!", Some(true), None);
        for &o in mates.iter() { if let Some(dm) = def_of(g, o) { crate::roomfight::loose_at(g, dm, from); } }
        crate::roomfight::loose_at(g, d, from);
    } else {
        crate::game::game_say(g, gd, "they've moved off it — eyes open!", Some(true), None);
        peek_on(g, gd, p);
        peek_back(g, gd, d);
    }
    s(g).said_t = now;
}

/// Every man held on that lane is let go of; the fit ones other than `gd` are returned.
fn let_go_the_men_held_on_it(g: &mut Game, gd: GuardId, id: f64) -> Vec<GuardId> {
    let mut mates: Vec<GuardId> = Vec::new();
    let mut at = 0usize;
    while let Some(o) = s(g).held.next_key(&mut at) {
        if s(g).held.get(o).map_or(false, |h| h.lane == id) {
            held_off(g, o);
            if o != gd && fit(g, o) { mates.push(o); }
        }
    }
    mates
}

/// where a lane opens before a man standing at `p`: the nearest of its floor he has a line to, of the ten nearest; else the nearest
pub(super) fn mouth_of(g: &crate::game::Game, l: LanesLaneId, p: V3) -> V3 {
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
    for &(_, x, z) in near.iter() { if !crate::scene_collision::solid_blocked(g, eye, [x, 1.0, z], None) { return [x, 0.0, z]; } }
    [near[0].1, 0.0, near[0].2]
}

/// the price of a walk with this mask's floor made dear, on top of `base`
pub(super) fn price_of(g: &crate::game::Game, m: Option<&[u8]>, base: Option<&crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> {
    use crate::lanes::LanePenalty;
    let Some(mask) = m else { return base.map(|b| LanePenalty::Base { base: b.clone() }) };
    let (w, dear) = (g.col.nav.w, g.k.lanes.dear);
    Some(match base {
        Some(b) => LanePenalty::DearOver { base: b.clone(), m: mask.to_vec(), w, dear },
        None => LanePenalty::Dear { m: mask.to_vec(), w, dear },
    })
}

///  `m[iz * W + ix] ? d : 1`: a cell off the mask costs 1, as a Uint8Array's undefined does
fn dear_at(mask: &[u8], w: f64, dear: f64, ix: f64, iz: f64) -> f64 {
    let i = iz * w + ix;
    if i >= 0.0 && mask.get(i as usize).map_or(false, |&b| b != 0) { dear } else { 1.0 }
}

/// what a LanePenalty answers for a cell: the closure priceOf returns; the tactics' price is asked first
pub fn lane_penalty_at(pen: &crate::lanes::LanePenalty, ix: f64, iz: f64) -> f64 {
    use crate::lanes::LanePenalty;
    match pen {
        LanePenalty::Dear { m, w, dear } => dear_at(m, *w, *dear, ix, iz),
        LanePenalty::DearOver { base, m, w, dear } => { let b = crate::tactics::tactics_penalty_price(base, ix, iz); m::max(b, dear_at(m, *w, *dear, ix, iz)) }
        LanePenalty::Base { base } => crate::tactics::tactics_penalty_price(base, ix, iz),
    }
}

/// The price of a walk for this man: `base` with the floor of every lane his side knows made dear
pub fn lane_penalty(g: &mut crate::game::Game, gd: GuardId, base: Option<&crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> {
    if g.gd[gd].script.as_ref().and_then(|x| x.direct) == Some(true) { return price_of(g, None, base); }   // (a walk a level's script sent the short way)
    if no_lanes(g) || !live(g) || !mine(g, gd) || pushing(g, gd) { return price_of(g, None, base); }
    let mask = mask_of(g, gd);
    price_of(g, mask.as_deref(), base)
}

/// True = this rule has his feet this frame: his way lies down a lane his side knows is covered, and he has been stopped short of it.
pub fn lane_gate(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].script.is_some() && crate::director::director_lesson_runs(g) { return false; }
    if no_lanes(g) || !live(g) || !mine(g, gd) || g.gd[gd].path.is_empty() || pushing(g, gd) { return false; }
    let Some(m) = mask_of(g, gd) else { return false };
    if flanking(g, gd) || s(g).peek.has(gd) { return false; }
    let sg = g.gd[gd].script.as_ref().and_then(|sc| sc.goal);
    if let (Some(h), Some(sg)) = (s(g).held.get(gd), sg) {
        if !h.aimed && dist_xz(sg, h.at) < 0.8 { return false; }
    }
    let last_look = s(g).look.get(gd).copied().unwrap_or(-100.0);
    if g.time - last_look < 0.2 { return false; }
    let now = g.time;
    s(g).look.set(gd, now);
    if close_fight(g, gd) || crate::control::man::blind(g, gd) || crate::frag::frag_flee(g, gd).is_some() { return false; }
    let here = in_mask(g, Some(&m), g.pos_of(gd));
    let Some(mut first) = ahead(g, &m, gd, here) else { return false };
    if !here {
        if let Some(goal) = g.gd[gd].path_goal {
            crate::guards::go_to(g, gd, goal, Some(true));
            let Some(again) = ahead(g, &m, gd, false) else {
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
            if dist_xz(on, lane_from) > dist_xz(g.pos_of(gd), lane_from) - 0.5 { return false; }
        }
        if back_out(g, gd, l) { return true; }
    } else {
        let p = g.pos_of(gd);
        let shift = g.k.lanes.shift_m;
        if (gun_on(g, l, p, Some(shift)) || crate::doorway::doorway_stand(g, p)) && back_out(g, gd, l) { return true; }
    }
    if def_of(g, gd).is_none() { return false; }
    if !s(g).due.iter().any(|d| d.gd == gd) {
        let mut look = first;
        let (p, look_m) = (g.pos_of(gd), g.k.lanes.look_m);
        if dist_xz(p, first) < look_m {
            let far = along(g, gd, 3.0);
            look = if dist_xz(p, far) >= look_m { far } else if dist_xz(p, lane_from) >= look_m { lane_from } else { first };
        }
        s(g).due.push(StateDueItem { gd, lane: lane_id, look });
    }
    g.gd[gd].speed = 0.0;
    let c = g.gd[gd].ch;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// The point `m` metres on along his way.
pub(super) fn along(g: &crate::game::Game, gd: GuardId, m: f64) -> V3 {
    let mut a = g.pos_of(gd);
    let mut left = m;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() {
        let b = g.gd[gd].path[i];
        let d = dist_xz(a, b);
        if d >= left { return [a[0] + (b[0] - a[0]) * left / js::or(d, 1.0), 0.0, a[2] + (b[2] - a[2]) * left / js::or(d, 1.0)]; }
        left -= d;
        a = b;
        i += 1;
    }
    a
}

/// He stops where he stands, short of a lane, his gun on `look`.
pub(super) fn plant_short(g: &mut crate::game::Game, gd: GuardId, lane: f64, look: V3) {
    if !mine(g, gd) { return; }
    let mut named = false;
    for l in lane_ids(g) {
        if g.tab.lanes_lane[l].id == lane && knows(g, l, gd) { named = true; break; }
    }
    if !named { return; }
    if !crate::roomfight::room_fight_lane_stop(g, gd, look) { return; }
    g.gd[gd].drop_path();
    let (p, now) = (g.pos_of(gd), g.time);
    held_on(g, gd, StateHeldValue { lane, t: now, look, aimed: true, at: p });
    if now - s(g).said_t > 5.0 {
        s(g).said_t = now;
        crate::game::game_say(g, gd, "holding here — I'm not walking into that!", Some(true), None);
    }
}

/// The first point of a known lane on the next `aheadM` of his way; `inside`: he stands in one, and None means his way leaves it for good within reach.
fn ahead(g: &crate::game::Game, m: &[u8], gd: GuardId, inside: bool) -> Option<V3> {
    let mut a = g.pos_of(gd);
    let mut left = g.k.lanes.ahead_m;
    let mut was = inside;
    let mut hit: Option<V3> = None;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() && left > 0.0 {
        let b = g.gd[gd].path[i];
        let d = dist_xz(a, b);
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
    if inside { Some(g.pos_of(gd)) } else { None }
}

/// He is held short of a lane his side knows is covered, and what wants him (`to`; none = anything) lies at its guns or down it.
pub fn lane_bars(g: &crate::game::Game, gd: GuardId, to: Option<V3>) -> bool {
    if crate::gunroles::gun_keeps_post(g, gd) { return true; }
    if g.m.lanes.states.is_none() || !live(g) || !crate::owner::has(g, gd, crate::owner::by::LANE) { return false; }
    let Some(l) = lane_of(g, gd) else { return false };
    let Some(to) = to else { return true };
    let field = g.k.lanes.field_m;
    if g.tab.lanes_lane[l].guns.iter().any(|m| dist_xz(*m, to) < field + 4.0) { return true; }
    let i = cell_at(g, to);
    g.tab.lanes_lane[l].cells.as_ref().map_or(false, |c| c.has(crate::Num::of(i)))
}

/// He has shouted for a lane and the wind-up still has his arm: the dig-in does not adopt him.
pub(in crate::defenders) fn lane_throws(g: &crate::game::Game, gd: GuardId) -> bool {
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    if st.peek.is_empty() { return false; }
    if !live(g) { return false; }
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    let Some(f) = st.peek.get(gd).and_then(|p| p.frag) else { return false };
    winding(g, gd, f)
}

/// The noise he would walk over to by himself is at the guns of a lane he knows is covered, or down it.
pub fn lane_known_at(g: &crate::game::Game, gd: GuardId, to: Option<V3>) -> bool {
    let Some(to) = to else { return false };
    if no_lanes(g) || !live(g) || !mine(g, gd) { return false; }
    let i = cell_at(g, to);
    let field = g.k.lanes.field_m;
    for l in lane_ids(g) {
        if !knows(g, l, gd) { continue; }
        let r = &g.tab.lanes_lane[l];
        let Some(cells) = r.cells.as_ref() else { continue };
        if cells.has(crate::Num::of(i)) || r.guns.iter().any(|m| dist_xz(*m, to) < field + 4.0) { return true; }
    }
    false
}

/// The far side of this doorway is in the line of guns his side knows of: no drill through it, unless his charge will burst on those guns.
pub fn lane_door(g: &mut crate::game::Game, gd: GuardId, far: V3, centre: V3, can: bool) -> bool {
    if no_lanes(g) || !live(g) || !mine(g, gd) || pushing(g, gd) || flanking(g, gd) { return false; }
    let Some(l) = lane_on(g, far, gd) else { return false };
    let bang = g.k.lanes.bang_m;
    !(can && g.tab.lanes_lane[l].guns.iter().any(|m| dist_xz(*m, centre) < bang + 2.5))
}

/// The two questions tryEngage asked, asked again while the drill has him this side of the doorway. True = the drill lets him go.
pub fn lane_drill(g: &mut crate::game::Game, gd: GuardId, at: V3, far: V3, centre: V3, can: bool) -> bool {
    if !lane_covers(g, gd, at) && !lane_door(g, gd, far, centre, can) { return false; }
    true
}

/// This spot is in the line of guns his side knows of: no place to stack on a doorway.
pub fn lane_covers(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool {
    if no_lanes(g) || !live(g) || !mine(g, gd) { return false; }
    !pushing(g, gd) && !flanking(g, gd) && lane_on(g, p, gd).is_some()
}

/// A new state, the counters kept; every man this file had is let go of first.
pub fn lanes_reset(g: &mut crate::game::Game) {
    let Some(stats) = g.m.lanes.states.as_ref().map(|st| st.stats) else { return };
    let_go(g);
    let mut st = fresh();
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
        assert!(crate::Num::of(-0.0) == crate::Num::of(0.0));
    }
}
