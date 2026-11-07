//! src/game/guards.ts lines 1502 to 1917
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `was.tgt === key`: the man by his ID, 'pt' by its word, null by None (guards.ts 1562)
fn same_key(a: &Option<LitSeenTgt>, b: &Option<LitSeenTgt>) -> bool {
    match (a, b) {
        (None, None) | (Some(LitSeenTgt::Pt), Some(LitSeenTgt::Pt)) => true,
        (Some(LitSeenTgt::Man { gd: x }), Some(LitSeenTgt::Man { gd: y })) => x == y,
        _ => false,
    }
}

/// is this floor point under what the meter calls lit, by the place's own lights only (guards.ts litPlace `dark`)
fn place_dark(g: &mut Game, x: f64, z: f64) -> bool {
    let mut carried = |_: &mut Game, _: f64| true;
    let e = light_estimate_at(g, [x, 1.2, z], Some(&mut carried), None, None, None, None);
    crate::game::game_visibility_of(g, e) < g.k.lights.lit_at
}

/// is where he stands, what his gun is on, the man he has and the mark he is nearly at lit by the place itself (guards.ts litPlace)
pub fn lit_place(g: &mut crate::game::Game, gd: GuardId, a: Option<OrdersAssignmentId>, tgt: Option<GuardId>, pt: Option<V3>) -> bool {
    let key = match (tgt, pt) {
        (Some(gd), _) => Some(LitSeenTgt::Man { gd }),
        (None, Some(_)) => Some(LitSeenTgt::Pt),
        (None, None) => None,
    };
    let now = g.time;
    if let Some(was) = g.m.guards.lit_seen.get(gd) {
        if was.a == a && same_key(&was.tgt, &key) && now - was.t >= 0.0 && now - was.t < 0.5 {
            return was.lit;
        }
    }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let aim_yaw = g.ch[c].aim_yaw;
    let eye: V3 = [pos[0], 1.5, pos[2]];
    let mut lit = !place_dark(g, pos[0], pos[2]);
    if lit {
        if let Some(t) = tgt {
            let p = g.ch[g.gd[t].ch].pos;
            lit = !place_dark(g, p[0], p[2]);
        }
    }
    if lit {
        if let Some(p) = pt {
            lit = !place_dark(g, p[0], p[2]);
        }
    }
    if lit {
        if let Some(id) = a {
            let (post, arrived) = (g.tab.orders_assignment[id].post, g.tab.orders_assignment[id].arrived);
            if let Some(post) = post {
                if !arrived && v3::dist_xz(pos, post) < g.k.lights.lit_mark_m {
                    lit = !place_dark(g, post[0], post[2]);
                }
            }
        }
    }
    for (off, step) in [(0.0, 1.0), (-0.2, 3.0), (0.2, 3.0)] {
        if !lit { break; }
        let fx = m::sin(aim_yaw + off);
        let fz = m::cos(aim_yaw + off);
        let mut r: f64 = step;
        while r <= g.k.lights.lit_m + 1e-6 {
            let p: V3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r];
            if crate::scene_collision::segment_blocked(g, eye, p, None, None, None) { break; }
            if place_dark(g, p[0], p[2]) { lit = false; break; }
            r += step;
        }
    }
    let t = g.time;
    g.m.guards.lit_seen.set(gd, GuardsModLitSeenValue { t, lit, a, tgt: key });
    lit
}

/// seven directions across a beam about `d`: its axis, four turned about the vertical, one tipped up and one down (guards.ts beamFan)
pub fn beam_fan(d: V3) -> Vec<V3> {
    let mut out: Vec<V3> = Vec::new();
    for a in [0.0, -0.17, 0.17, -0.33, 0.33] {
        let c = m::cos(a);
        let k = m::sin(a);
        out.push([d[0] * c + d[2] * k, d[1], d[2] * c - d[0] * k]);
    }
    for e in [-0.3, 0.3] {
        out.push(v3::normalize([d[0], d[1] + e, d[2]]));
    }
    out
}

/// what one ray of a beam ends on inside `m` of the lens: 3 something that stands, 2 its foot, 1 the ground, 0 nothing (guards.ts rayEnd)
pub fn ray_end(g: &mut crate::game::Game, o: V3, r: V3, m: f64) -> f64 {
    let dip = if r[1] < 0.0 { m::max(0.0, (o[1] - FLOOR_TOP) / -r[1]) } else { f64::INFINITY };
    let a = m::min(m, dip);
    if a > 0.0 && crate::scene_collision::segment_blocked(g, o, [o[0] + r[0] * a, o[1] + r[1] * a, o[2] + r[2] * a], None, None, None) {
        return 3.0;
    }
    if dip >= m {
        return 0.0;
    }
    let run = m::min(m, o[1] / -r[1]) - dip;
    let p: V3 = [o[0] + r[0] * dip, FLOOR_TOP, o[2] + r[2] * dip];
    if run > 0.0 && crate::scene_collision::segment_blocked(g, p, [p[0] + r[0] * run, FLOOR_TOP, p[2] + r[2] * run], None, None, None) {
        return 2.0;
    }
    1.0
}

/// is a beam into a wall: its middle ends on something that stands inside `m`, and no ray runs clear (guards.ts beamWalled)
pub fn beam_walled(g: &mut crate::game::Game, o: V3, fan: &[V3], m: f64, held: bool) -> bool {
    for i in 0..fan.len() {
        let e = ray_end(g, o, fan[i], m);
        if e == 0.0 || (i == 0 && !(e == 3.0 || (held && e == 2.0))) {
            return false;
        }
    }
    true
}

/// beamWalled of a beam laid by hand, from `o` along `d` (guards.ts beamWalledAt)
pub fn beam_walled_at(g: &mut crate::game::Game, o: V3, d: V3, m: f64, held: Option<bool>) -> bool {
    let held = held.unwrap_or(false);
    let fan = beam_fan(d);
    beam_walled(g, o, &fan, m, held)
}

/// what one ray of a beam ends on (guards.ts beamRayEnd)
pub fn beam_ray_end(g: &mut crate::game::Game, o: V3, r: V3, m: f64) -> f64 {
    ray_end(g, o, r, m)
}

/// where a carried light will burn: at its lens `p`, or this side of the wall the lens is carried through from `held` (guards.ts burnsFrom)
pub fn burns_from(g: &mut crate::game::Game, held: V3, p: V3) -> V3 {
    if crate::scene_collision::segment_blocked(g, held, p, None, None, None) {
        let radius = g.engine.settings.flashlight_radius;
        crate::lighting::lens_clear(g, held, p, radius, None)
    } else {
        p
    }
}

/// a living man's weapon light, for the checks: nothing before he has a pose (guards.ts beamLens)
pub fn beam_lens(g: &mut crate::game::Game, gd: GuardId) -> Option<V3> {
    let c = g.gd[gd].ch;
    let held = g.ch[c].bones.get(crate::character::Bone::UpperArmR).copied()?;
    let l = g.gd[gd].flashlight;
    let p = g.tab.light[l].pos;
    Some(burns_from(g, held, p))
}

/// somebody not of his side in the beam, this side of what it ends on (guards.ts wallDark `manIn`)
fn man_in(g: &Game, gd: GuardId, o: V3, d: V3, hl: f64, m: f64) -> bool {
    g.guards.iter().any(|&x| {
        if x == gd || g.gd[x].state == crate::game::GuardState::Dead || g.gd[x].team == crate::teams::Team::Operative {
            return false;
        }
        let at = g.ch[g.gd[x].ch].pos;
        let vx = at[0] - o[0];
        let vz = at[2] - o[2];
        let l = m::hyp2(vx, vz);
        l < m + 0.5 && (l < 0.3 || (vx * d[0] + vz * d[2]) / (l * hl) > 0.9)
    })
}

/// LIGHTS.wallOff: a teammate's weapon light is dark while its beam is into something that stands within reach of the lens (guards.ts wallDark)
pub fn wall_dark(g: &mut crate::game::Game, gd: GuardId, held: Option<V3>) -> bool {
    if g.gd[gd].team != crate::teams::Team::Operative || g.gd[gd].unit.is_none() || g.gd[gd].state == crate::game::GuardState::Dead {
        return false;
    }
    let Some(held) = held else { return false };
    let l = g.gd[gd].flashlight;
    let p = g.tab.light[l].pos;
    let d = g.tab.light[l].dir;
    let t = g.time;
    let o = burns_from(g, held, p);
    let fan = beam_fan(d);
    let hl = js::or(m::hyp2(d[0], d[2]), 1.0);
    let was = g.m.guards.wall_seen.get(gd).map(|w| (w.t, w.since, w.dark));
    let (s_since, s_dark) = match was {
        Some((was_t, since, dark)) if t >= was_t && t - was_t <= 0.25 => (since, dark),
        _ => (t - g.k.lights.wall_secs, false),
    };
    let reach = if s_dark { g.k.lights.wall_back_m } else { g.k.lights.wall_m };
    let near = beam_walled(g, o, &fan, reach, s_dark) && !man_in(g, gd, o, d, hl, reach);
    let since = if near { s_since } else { t };
    let dark = near && (s_dark || t - since >= g.k.lights.wall_secs - 1e-9);
    g.m.guards.wall_seen.set(gd, GuardsModWallSeenValue { t, since, dark, out: None });
    dark
}

/// TORCH.wallLow: a man with a hand torch whose carried beam would end whole on something that stands lets it hang (guards.ts torchLow)
pub fn torch_low(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let t = g.time;
    let Some(hand) = g.ch[c].bones.get(crate::character::Bone::HandR).copied() else { return false };
    let d = crate::rig_props::torch_carry_want(g, c, None);
    let low0 = match g.m.guards.torch_low_seen.get(gd) {
        Some(was) => t >= was.t && t - was.t <= 0.25 && was.low,
        None => false,
    };
    let reach = if low0 { g.k.torch.wall_back_m } else { g.k.torch.wall_m };
    let fan = beam_fan(d);
    let low = beam_walled(g, hand, &fan, reach, low0);
    g.m.guards.torch_low_seen.set(gd, GuardsModTorchLowSeenValue { t, low });
    low
}

/// is this man's weapon light this rule's to decide (guards.ts lightRuled)
pub fn light_ruled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
        && g.gd[gd].unit.is_some()
        && crate::roomfight::fight_live(g)
        && crate::seats::order_of(g, gd).is_some()
}

/// stands this floor point lit by the place's steady lights (guards.ts darkOnLitMan `lit`)
fn place_lit_steady(g: &mut Game, p: V3) -> bool {
    let mut carried = |_: &mut Game, _: f64| true;
    let e = light_estimate_at(g, [p[0], 1.2, p[2]], Some(&mut carried), Some(true), None, None, None);
    crate::game::game_visibility_of(g, e) >= g.k.lights.lit_at
}

/// LIGHTS.litMan: the man his eyes have stands lit by the place, he himself in the dark (guards.ts darkOnLitMan)
pub fn dark_on_lit_man(g: &mut crate::game::Game, gd: GuardId, tgt: GuardId) -> bool {
    if let Some(was) = g.m.guards.lit_man_seen.get(gd) {
        if was.tgt == tgt && g.time - was.t >= 0.0 && g.time - was.t < 0.25 {
            return was.v;
        }
    }
    let tgt_pos = g.ch[g.gd[tgt].ch].pos;
    let v = place_lit_steady(g, tgt_pos) && {
        let own_pos = g.ch[g.gd[gd].ch].pos;
        !place_lit_steady(g, own_pos)
    };
    let t = g.time;
    g.m.guards.lit_man_seen.set(gd, GuardsModLitManSeenValue { t, tgt, v });
    v
}

/// LIGHTS.litMan, asked of a teammate the light rule has: is he in the dark on a man who stands lit (guards.ts gunOnLitMan)
pub fn gun_on_lit_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !light_ruled(g, gd) {
        return false;
    }
    let Some(o) = crate::roomfight::room_fight_op_of(g, gd) else { return false };
    let t = g.time;
    let op = &g.tab.roomfight_op[o];
    let man = match op.tgt {
        Some(man) => Some(man),
        None => match op.kept.as_ref() {
            Some(kept)
                if op.turn.is_none()
                    && g.gd[gd].script.is_some()
                    && g.gd[kept.h].state != crate::game::GuardState::Dead
                    && t - kept.t >= 0.0
                    && t - kept.t < 1.5 =>
            {
                Some(kept.h)
            }
            _ => None,
        },
    };
    match man {
        Some(man) => dark_on_lit_man(g, gd, man),
        None => false,
    }
}

/// does his weapon light burn this frame, asked only of a man the rule has (guards.ts teamLit)
pub fn team_lit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if gun_on_lit_man(g, gd) {
        return false;
    }
    let t = g.time;
    let a = crate::orders::squad_orders_of(g, &crate::seats::here(g), gd);
    let o = crate::roomfight::room_fight_op_of(g, gd);
    let o_row = o.map(|id| &g.tab.roomfight_op[id]);
    let a_row = a.map(|id| &g.tab.orders_assignment[id]);
    let man = &g.gd[gd];
    let k = &g.k.lights;
    let pos = g.ch[man.ch].pos;
    let want = (man.script.is_none() && (man.state == crate::game::GuardState::Alert || man.state == crate::game::GuardState::Search))
        || matches!(&man.script, Some(s) if s.over.is_some())
        || matches!(o_row, Some(op) if op.tgt.is_some()
            || op.turn.is_some()
            || (t - op.hot_t < k.hot_secs && !matches!(a_row, Some(r) if r.door.is_some() && r.search != Some(true))))
        || matches!(a_row, Some(r) if r.search == Some(true)
            && !(r.arrived && t - m::max(r.arr_t, r.search_t.unwrap_or(f64::NEG_INFINITY)) > k.after_secs))
        || matches!(o_row, Some(op) if t - op.step_t < k.step_secs)
        || matches!(a_row, Some(r) if match r.post {
            Some(post) => v3::dist_xz(pos, post) > k.on_m,
            None => v3::dist_xz(pos, g.ch[g.pl[g.player].ch].pos) > k.alone_m,
        });
    let tgt = o_row.and_then(|op| op.tgt);
    let pt = o_row.and_then(|op| op.turn.as_ref()).and_then(|turn| turn.pt);
    if want && !lit_place(g, gd, a, tgt, pt) {
        let until = t + g.k.lights.tail;
        g.m.guards.lit_until.set(gd, until);
    }
    match g.m.guards.lit_until.get(gd).copied() {
        Some(u) => t < u && u - t <= g.k.lights.tail + 1e-6,
        None => false,
    }
}

/// is this man one of the team whose pace PACE governs right now (guards.ts teamPaced)
pub fn team_paced(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some() && crate::roomfight::fight_live(g)
}

/// the speed his legs may be asked for (guards.ts teamPace)
pub fn team_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 {
    let capped = if team_paced(g, gd) { m::min(speed, crate::teampace::team_cap(g, gd)) } else { speed };
    let silent = crate::silent::silent_pace(g, gd, capped);
    crate::condition::condition_pace(g, gd, silent)
}

/// the speed his legs get for a leg asked at `speed`: one call a leg (guards.ts teamLegs)
pub fn team_legs(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, speed: f64) -> f64 {
    match crate::teampace::team_run_legs(g, seat, gd, speed) {
        Some(legs) => legs,
        None => crate::kitload::load_pace(g, seat, gd, speed),
    }
}

/// where his muzzle is this frame, wanting yaw `to`: the team's comes round at PACE.turn, round his front (guards.ts teamTurn)
pub fn team_turn(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, to: f64, dt: f64) -> f64 {
    if !(g.k.pace.turn > 0.0) || !team_paced(g, gd) {
        return to;
    }
    let c = g.gd[gd].ch;
    let (aim_yaw, body_yaw) = (g.ch[c].aim_yaw, g.ch[c].body_yaw);
    let rel = v3::wrap_angle(aim_yaw - body_yaw);
    let want = v3::clamp(v3::wrap_angle(to - body_yaw), -1.5, 1.5);
    let turn = g.k.pace.turn;
    let step = turn * crate::kitload::load_pace_on(g, seat, crate::kitload::LoadPaceOnWho::Guard(gd)) * dt;
    body_yaw + (if m::abs(want - rel) <= step { want } else { rel + m::sign(want - rel) * step })
}

/// his speed this frame, having been `was` last frame: the team picks pace up at PACE.accel at most (guards.ts teamLean)
pub fn team_lean(g: &mut crate::game::Game, gd: GuardId, was: f64, dt: f64) {
    if g.gd[gd].speed > was && team_paced(g, gd) {
        let now = g.gd[gd].speed;
        let accel = crate::teampace::team_accel(g, gd);
        let load = crate::kitload::load_pace_on(g, &crate::seats::here(g), crate::kitload::LoadPaceOnWho::Guard(gd));
        g.gd[gd].speed = m::min(now, was + accel * load * dt);
    }
}

/// walks him along his path this frame; true when he has arrived (guards.ts followPath)
pub fn follow_path(g: &mut crate::game::Game, gd: GuardId, dt: f64, speed: f64) -> bool {
    let c = g.gd[gd].ch;
    let paced = team_pace(g, gd, speed);
    let mut speed = team_legs(g, &crate::seats::here(g), gd, paced);
    if unstick_first(g, gd, dt) {
        return false;
    }
    if g.tune.tactics && !g.gd[gd].fled && crate::tactics::door_gate(g, gd, dt, speed) {
        return false;
    }
    if !g.gd[gd].fled && crate::lanes::lane_gate(g, gd) {
        return false;
    }
    if crate::handleside::handle_gate(g, gd, dt, speed, None) {
        return false;
    }
    if !g.gd[gd].fled {
        match crate::doorway::doorway_gate(g, gd, dt, speed) {
            None => return false,
            Some(sp) => speed = sp,
        }
    }
    if !g.gd[gd].fled {
        match crate::darkdoor::dark_door_gate(g, gd, dt, speed) {
            None => return false,
            Some(sp) => speed = sp,
        }
    }
    let pos = g.ch[c].pos;
    let last = g.gd[gd].last_pos;
    let moved = m::hyp2(pos[0] - last[0], pos[2] - last[2]);
    g.gd[gd].last_pos = v3::copy(pos);
    let was_speed = g.gd[gd].speed;
    if was_speed > 0.3 && moved < was_speed * dt * 0.25 {
        g.gd[gd].stuck_t += dt;
    } else {
        g.gd[gd].stuck_t = m::max(0.0, g.gd[gd].stuck_t - dt * 2.0);
    }
    if g.gd[gd].stuck_t > 0.7 && g.gd[gd].path_i < g.gd[gd].path.len() {
        let path_i = g.gd[gd].path_i;
        let wp = g.gd[gd].path[path_i];
        let dx = wp[0] - pos[0];
        let dz = wp[2] - pos[2];
        let d = js::or(m::hyp2(dx, dz), 1.0);
        let stuck_n = g.gd[gd].stuck_n;
        g.gd[gd].stuck_n = stuck_n + 1.0;
        let side = if js::to_i32(stuck_n) & 1 != 0 { 1.0 } else { -1.0 };
        let px = -dz / d * side;
        let pz = dx / d * side;
        let cand: V3 = [pos[0] + px * 0.7 + dx / d * 0.2, 0.0, pos[2] + pz * 0.7 + dz / d * 0.2];
        if !crate::scene_collision::nav_grid_is_blocked(g, cand[0], cand[2]) {
            g.gd[gd].path.insert(path_i, cand);   // (in place: the same path to anyone holding it, so path_serial stays)
        } else if let Some(goal) = g.gd[gd].path_goal {
            g.gd[gd].path_goal = None;
            g.gd[gd].path_goal_orbit = false;   // (the back-off's own point is gone: goTo lays a copy)
            go_to(g, gd, goal, Some(true));
        }
        g.gd[gd].stuck_t = 0.0;
    }
    while g.gd[gd].path_i < g.gd[gd].path.len() {
        let path_i = g.gd[gd].path_i;
        let path_len = g.gd[gd].path.len();
        let wp = g.gd[gd].path[path_i];
        let pos = g.ch[c].pos;
        let d = m::hyp2(wp[0] - pos[0], wp[2] - pos[2]);
        if d < (if path_i == path_len - 1 { 0.25 } else { 0.45 }) {
            g.gd[gd].path_i += 1;
            continue;
        }
        let dir: V3 = [(wp[0] - pos[0]) / d, 0.0, (wp[2] - pos[2]) / d];
        let yaw_to = m::atan2(dir[0], dir[2]);
        let facing = m::cos(v3::wrap_angle(yaw_to - g.ch[c].body_yaw));
        let remaining = d + (path_len - 1 - path_i) as f64 * 1.0;
        let tgt_speed = speed * v3::clamp(facing * 1.5, 0.15, 1.0) * v3::clamp(remaining / 0.8, 0.35, 1.0);
        let was = g.gd[gd].speed;
        g.gd[gd].speed = v3::damp(was, tgt_speed, 4.0, dt);
        team_lean(g, gd, was, dt);
        let yaw = v3::approach_angle(g.ch[c].body_yaw, yaw_to, 5.5 * dt);
        g.ch[c].body_yaw = yaw;
        let walked = g.gd[gd].speed;
        let vel: V3 = [m::sin(yaw) * walked, 0.0, m::cos(yaw) * walked];
        g.ch[c].vel = vel;
        { let to = v3::mad(g.ch[c].pos, vel, dt); crate::character::pos_replace(g, c, to); }
        return false;
    }
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// the doorway cells of every door: nav cell index → the door whose closed frame line runs through it, worked out once (guards.ts doorCells)
pub fn door_cells(g: &mut crate::game::Game) -> JsMap<crate::Num, DoorId> {
    if let Some(cells) = &g.m.guards.door_cells_of {
        return cells.clone();
    }
    let mut cells: JsMap<crate::Num, DoorId> = JsMap::new();
    let list = g.doors.list.clone();
    for d in list {
        let hinge = g.dr[d].def.hinge;
        let (hx, hz) = (hinge[0], hinge[1]);
        let closed_dir = g.dr[d].def.closed_dir;
        let width = g.dr[d].def.width;
        let ux = m::cos(closed_dir);
        let uz = m::sin(closed_dir);
        let mut a = 0.1;
        while a < width - 0.05 {
            for off in [0.0, 0.18, -0.18] {
                let x = hx + ux * a - uz * off;
                let z = hz + uz * a + ux * off;
                if crate::scene_collision::nav_grid_is_blocked(g, x, z) {
                    continue;
                }
                let cell = g.col.nav.cell;
                let key = crate::scene_collision::nav_grid_idx(g, m::floor(x / cell), m::floor(z / cell));
                cells.set(crate::Num(key), d);
            }
            a += 0.2;
        }
    }
    g.m.guards.door_cells_of = Some(cells.clone());
    cells
}

/// the nav grid's cell index, as a list index (NavGrid.idx)
fn nav_cell(g: &Game, x: f64, z: f64) -> usize {
    crate::scene_collision::nav_grid_idx(g, x, z) as usize
}

/// Dijkstra over the nav grid outward from `from`, in seconds of Sam's sprint; doors cost or stop him unless `keyed` (guards.ts floodFrom)
pub fn flood_from(g: &mut crate::game::Game, from: V3, max_m: f64, keyed: Option<bool>) -> crate::guards::FloodFrom {
    let keyed = keyed.unwrap_or(false);
    let (w, h, cs) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let n = (w * h) as usize;
    let tune_sprint = g.tune.player_sprint;
    let sprint = m::max(1.0, tune_sprint * crate::kitload::load_sprint_on(g, &crate::seats::here(g)));
    let mut secs: Vec<f32> = vec![f32::INFINITY; n];
    let mut metres: Vec<f32> = vec![f32::INFINITY; n];
    let mut closed: Vec<bool> = vec![false; n];
    let dc = door_cells(g);
    let mut sx = m::floor(from[0] / cs);
    let mut sz = m::floor(from[2] / cs);
    // (a NaN passes the bounds test in TypeScript and then reaches nothing: all Infinity, as here)
    if sx.is_nan() || sz.is_nan() || sx < 0.0 || sz < 0.0 || sx >= w || sz >= h {
        return FloodFrom { secs, metres };
    }
    let first = nav_cell(g, sx, sz);
    if g.col.nav.blocked[first] != 0 {
        let Some(p) = crate::scene_collision::nav_grid_nearest_free_point(g, from[0], from[2]) else {
            return FloodFrom { secs, metres };
        };
        sx = m::floor(p[0] / cs);
        sz = m::floor(p[2] / cs);
    }
    let start = nav_cell(g, sx, sz);
    secs[start] = 0.0;
    metres[start] = 0.0;
    let mut open: Vec<usize> = vec![start];
    let (wi, hi) = (w as i32, h as i32);
    while !open.is_empty() {
        let mut mi = 0;
        for i in 1..open.len() {
            if secs[open[i]] < secs[open[mi]] {
                mi = i;
            }
        }
        let cur = open.swap_remove(mi);
        if closed[cur] {
            continue;
        }
        closed[cur] = true;
        if metres[cur] as f64 > max_m {
            continue;
        }
        let cx = cur as i32 % wi;
        let cz = cur as i32 / wi;
        let cur_door = dc.get(crate::Num(cur as f64)).copied();
        for dz in -1..=1i32 {
            for dx in -1..=1i32 {
                if dx == 0 && dz == 0 {
                    continue;
                }
                let (nx, nz) = (cx + dx, cz + dz);
                if nx < 0 || nz < 0 || nx >= wi || nz >= hi {
                    continue;
                }
                let ni = nav_cell(g, nx as f64, nz as f64);
                if g.col.nav.blocked[ni] != 0 || closed[ni] {
                    continue;
                }
                if dx != 0 && dz != 0 {
                    let across = nav_cell(g, (cx + dx) as f64, cz as f64);
                    let along = nav_cell(g, cx as f64, (cz + dz) as f64);
                    if g.col.nav.blocked[across] != 0 || g.col.nav.blocked[along] != 0 {
                        continue;
                    }
                }
                let step = (if dx != 0 && dz != 0 { core::f64::consts::SQRT_2 } else { 1.0 }) * cs;
                let mut extra = 0.0;
                let d = if keyed { None } else { dc.get(crate::Num(ni as f64)).copied() };
                if let Some(d) = d {
                    if Some(d) != cur_door && crate::doors::door_latched(g, d) {
                        if crate::doors::door_locked(g, d) && !crate::doors::door_lock_broken(g, d) {
                            let at: V3 = [(cx as f64 + 0.5) * cs, 0.0, (cz as f64 + 0.5) * cs];
                            let from_side = crate::doors::doors_side(g, d, at);
                            if from_side == g.dr[d].key_side {
                                continue;
                            }
                        }
                        extra = LATCH_SECS;
                    }
                }
                let ns = secs[cur] as f64 + step / sprint + extra;
                if ns < secs[ni] as f64 {
                    secs[ni] = ns as f32;
                    metres[ni] = (metres[cur] as f64 + step) as f32;
                    open.push(ni);
                }
            }
        }
    }
    FloodFrom { secs, metres }
}

/// A cell of the deal.   (guards.ts dealSpots, local type Cand)
#[derive(Clone, Copy, Debug)]
struct Cand { cell: usize, pos: V3, kind: crate::guards::SearchSpotKind, cheap: f64, m: f64, hidden: bool, cover: f64 }

/// Every one of `list` at least `r` from `p`.   (guards.ts dealSpots, farFrom)
fn far_from(p: V3, list: &[Cand], r: f64) -> bool { list.iter().all(|k| v3::dist_xz(k.pos, p) >= r) }

/// Deal the spots of the man's plan (his row) round its fix: flood, count what stands about every reached cell, keep the best covered a spacing apart, ask the
/// line from the fix, take the top spots by cover and a few well-spread cells of open floor.   (guards.ts dealSpots)
pub fn deal_spots(g: &mut crate::game::Game, gd: GuardId) {
    let (w, hn, cs) = (g.col.nav.w, g.col.nav.h, g.col.nav.cell);
    let n = (w * hn) as usize;
    let gi = g.guards.iter().position(|&o| o == gd).unwrap_or(0);
    let fix = g.gd[gd].search_plan.as_ref().unwrap().fix;
    let crate::guards::FloodFrom { secs, metres } = flood_from(g, fix, SEARCH_RADIUS_M, None);
    // props: every cell within reach of a chair / carton / plant / bin counts it
    let mut prop_n: Vec<u8> = vec![0; n];
    for i in 0..g.props.props.len() {
        let id = g.props.props[i];
        let r = g.tab.props_prop[id].radius + 0.7;
        let (px, pz) = (crate::props::prop_x(g, id), crate::props::prop_z(g, id));
        let (x0, x1, z0, z1) = (
            m::max(0.0, m::floor((px - r) / cs)),
            m::min(w - 1.0, m::floor((px + r) / cs)),
            m::max(0.0, m::floor((pz - r) / cs)),
            m::min(hn - 1.0, m::floor((pz + r) / cs)),
        );
        let mut z = z0;
        while z <= z1 {
            let mut x = x0;
            while x <= x1 {
                if m::hyp2((x + 0.5) * cs - px, (z + 0.5) * cs - pz) < r {
                    let at = (z * w + x) as usize;
                    prop_n[at] = if prop_n[at] == 255 { 255 } else { prop_n[at] + 1 };
                }
                x += 1.0;
            }
            z += 1.0;
        }
    }
    // open leaves: the cells close along a leaf standing well open, off the doorway itself: behind the door
    let mut leaf_n: Vec<u8> = vec![0; n];
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if m::abs(crate::doors::door_angle(g, d)) < 0.7 || crate::doors::door_off(g, d) { continue; }
        let [hx, hz] = g.dr[d].def.hinge;
        let u = crate::doors::door_dir(g, d);
        let fc = g.dr[d].frame_centre;
        let width = g.dr[d].def.width;
        let mut a = 0.2;
        while a <= width {
            for s in [-0.55, 0.55] {
                let (x, z) = (hx + u[0] * a - u[1] * s, hz + u[1] * a + u[0] * s);
                if m::hyp2(x - fc[0], z - fc[2]) < 0.7 || crate::scene_collision::nav_grid_is_blocked(g, x, z) { continue; }
                let at = crate::scene_collision::nav_grid_idx(g, m::floor(x / cs), m::floor(z / cs)) as usize;
                leaf_n[at] = 1;
            }
            a += 0.25;
        }
    }
    // every reached cell a stride or more off the fix: what stands about it
    let (mut cands, mut opens): (Vec<Cand>, Vec<Cand>) = (Vec::new(), Vec::new());
    for i in 0..n {
        let met = metres[i] as f64;
        if !secs[i].is_finite() || met > SEARCH_RADIUS_M || met < 1.5 { continue; }
        let (cx, cz) = ((i % w as usize) as f64, (i / w as usize) as f64);
        let mut n_wall = 0;
        for dz in [-1.0, 0.0, 1.0] {
            for dx in [-1.0, 0.0, 1.0] {
                if dx == 0.0 && dz == 0.0 { continue; }
                let (nx, nz): (f64, f64) = (cx + dx, cz + dz);
                if nx < 0.0 || nz < 0.0 || nx >= w || nz >= hn { n_wall += 1; continue; }
                let at = crate::scene_collision::nav_grid_idx(g, nx, nz) as usize;
                if g.col.nav.blocked[at] != 0 { n_wall += 1; }
            }
        }
        let (n_prop, n_leaf) = (prop_n[i], leaf_n[i]);
        let cheap = 0.12 * m::min(n_wall as f64, 5.0) + 0.2 * m::min(n_prop as f64, 2.0) + 0.45 * (n_leaf as f64);
        let kind = if n_leaf != 0 { SearchSpotKind::Leaf } else if n_wall >= 5 { SearchSpotKind::Corner } else if n_prop != 0 { SearchSpotKind::Prop } else if n_wall != 0 { SearchSpotKind::Wall } else { SearchSpotKind::Open };
        let c = Cand { cell: i, pos: [(cx + 0.5) * cs, 0.0, (cz + 0.5) * cs], kind, cheap, m: met, hidden: false, cover: 0.0 };
        if cheap > 0.0 { cands.push(c); } else { opens.push(c); }
    }
    js::sort_by(&mut cands, |a, b| js::or(b.cheap - a.cheap, a.m - b.m));
    let mut kept: Vec<Cand> = Vec::new();
    for c in cands.iter() {
        if kept.len() >= 40 { break; }
        if far_from(c.pos, &kept, SEARCH_SPACING_M) { kept.push(*c); }
    }
    let eye_fix: V3 = [fix[0], 1.0, fix[2]];
    for c in kept.iter_mut() {
        c.hidden = crate::scene_collision::segment_blocked(g, eye_fix, [c.pos[0], 0.6, c.pos[2]], None, None, None);
        c.cover = v3::clamp(c.cheap + (if c.hidden { 0.3 } else { 0.0 }), 0.05, 1.0);
    }
    js::sort_by(&mut kept, |a, b| js::or(b.cover - a.cover, a.m - b.m));
    let mut chosen: Vec<Cand> = kept.iter().take(SEARCH_SPOTS as usize).copied().collect();
    js::sort_by(&mut opens, |a, b| b.m - a.m);   // controls: open floor, as far out and as far apart as the place allows
    let mut controls = 0.0;
    for c in opens.iter_mut() {
        if controls >= SEARCH_CONTROLS { break; }
        if far_from(c.pos, &chosen, 2.5) {
            c.hidden = crate::scene_collision::segment_blocked(g, eye_fix, [c.pos[0], 0.6, c.pos[2]], None, None, None);
            c.cover = if c.hidden { 0.3 } else { 0.1 };
            chosen.push(*c);
            controls += 1.0;
        }
    }
    let mut spots: Vec<crate::guards::SearchSpot> = Vec::with_capacity(chosen.len());
    for (i, c) in chosen.iter().enumerate() {
        let key = alloc::format!("srch:{}:{}", gi, i);
        crate::game::game_light_forget(g, &key);   // (the last plan's answer under this key was for another spot)
        spots.push(crate::guards::SearchSpot { pos: c.pos, cell: c.cell as f64, key, kind: c.kind, hidden: c.hidden, cover: c.cover, dark: 0.5, e: None, need: secs[c.cell] as f64, plaus: 1.0, score: 0.0, visited: false, by: -1.0, unreachable: false, samples: Vec::new() });
    }
    let now = g.time;
    let p = g.gd[gd].search_plan.as_mut().unwrap();
    p.spots = spots;
    p.flood_at = now;
    p.current = -1.0;
    p.rr = 0.0;
}

/// The plan for this man's search, on his row: built on its first frame round his fix (his feet if he has none), rebuilt if the fix has moved a couple of
/// metres, plausibility re-flooded every SEARCH_REFLOOD_SECS.   (guards.ts searchPlanFor)
pub fn search_plan_for(g: &mut crate::game::Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    let at = g.ch[c].pos;
    let fix: V3 = match g.gd[gd].last_known { Some(k) => [k[0], 0.0, k[2]], None => [at[0], 0.0, at[2]] };
    let standing = match &g.gd[gd].search_plan { Some(q) => !(v3::dist_xz(q.fix, fix) > 2.0), None => false };   // a new place to search: deal again round it
    if !standing {
        let p = crate::guards::SearchPlan {
            fix,
            t0: if g.gd[gd].last_known.is_some() { g.gd[gd].fix_t } else { g.time },
            serial: (match &g.gd[gd].search_plan { Some(q) => q.serial, None => 0.0 }) + 1.0,
            built_at: g.time,
            flood_at: g.time,
            spots: Vec::new(),
            current: -1.0,
            phase: SearchPlanPhase::Sweep,
            phase_t: 0.0,
            visits: 0.0,
            rr: 0.0,
            said: false,
        };
        g.gd[gd].search_plan = Some(p);   // (before the deal, which works on the row: nothing it calls reads this man's plan)
        deal_spots(g, gd);
        return;
    }
    let (flood_at, plan_fix) = { let q = g.gd[gd].search_plan.as_ref().unwrap(); (q.flood_at, q.fix) };
    if g.time - flood_at > SEARCH_REFLOOD_SECS {   // time has passed, leaves have moved: how long Sam would need to each spot NOW
        let secs = flood_from(g, plan_fix, SEARCH_RADIUS_M, None).secs;
        let now = g.time;
        let q = g.gd[gd].search_plan.as_mut().unwrap();
        for s in q.spots.iter_mut() { s.need = secs[s.cell as usize] as f64; }
        q.flood_at = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own bits under EXACT, for one beam
    #[test]
    fn beam_fan_bits() {
        const WANT: [[u64; 3]; 7] = [
            [0x3fe3333333333333, 0xbfb999999999999a, 0x3fe96632cbc9f77f],
            [0x3fdd4093fd77530a, 0xbfb999999999999a, 0x3fec48086579091b],
            [0x3fe738674876bb5f, 0xbfb999999999999a, 0x3fe5c8e71306fe17],
            [0x3fd3ddfd92965f8b, 0xbfb999999999999a, 0x3fee4017612beb6c],
            [0x3fea64fab99e804c, 0xbfb999999999999a, 0x3fe1ce9e65c0ef04],
            [0x3fe1e772981b4b47, 0xbfd7df437579b9b5, 0x3fe7af54cf99707c],
            [0x3fe2eb1728e9bba4, 0x3fc93974368cfa30, 0x3fe906ce75529914],
        ];
        let fan = beam_fan([0.6, -0.1, 0.7937253933193772]);
        assert_eq!(fan.len(), 7);
        for (ray, want) in fan.iter().zip(WANT.iter()) {
            for i in 0..3 {
                assert_eq!(ray[i].to_bits(), want[i]);
            }
        }
    }
}
