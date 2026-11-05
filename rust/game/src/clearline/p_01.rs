//! src/game/clearline.ts lines 1 to 407
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `bodies` as a list: nothing between its yields writes, so laying it out at once reads the same floor. (clearline.ts bodies)
fn bodies_of(g: &Game, self_: GuardId) -> Vec<V3> {
    let mut out = Vec::new();
    if !g.pl[g.player].down {
        out.push(g.ch[g.pl[g.player].ch].pos);
    }
    for &o in g.guards.iter() {
        if o != self_ && g.gd[o].state != crate::game::GuardState::Dead {
            out.push(g.ch[g.gd[o].ch].pos);
        }
    }
    for &o in g.civilians.iter() {
        if g.gd[o].state != crate::game::GuardState::Dead {
            out.push(g.ch[g.gd[o].ch].pos);
        }
    }
    out
}
/// The live `D` of the TypeScript: his row's Doing, for a write. Only where `doing` has just answered Some and nothing since has ended it.
fn live<'g>(g: &'g mut Game, gd: GuardId) -> &'g mut Doing {
    mem_of(g, gd).doing.as_mut().unwrap()
}
/// `M.leadOf === by`: one friend, by value. (clearline.ts:156, 159)
fn lead_of_is(lead_of: Option<LeadsBy>, by: LeadsBy) -> bool {
    match (lead_of, by) {
        (Some(LeadsBy::Player), LeadsBy::Player) => true,
        (Some(LeadsBy::Guard(a)), LeadsBy::Guard(b)) => a == b,
        _ => false,
    }
}
fn lead_by(by: ClearLineBlockedBy) -> LeadsBy {
    match by {
        ClearLineBlockedBy::Player => LeadsBy::Player,
        ClearLineBlockedBy::Guard(b) => LeadsBy::Guard(b),
    }
}
/// The line he says with a friend in his line and nothing to do about it. (clearline.ts:362, 393)
fn my_line(g: &Game, by: ClearLineBlockedBy) -> String {
    match by {
        ClearLineBlockedBy::Player => String::from("no shot — you're in my line!"),
        ClearLineBlockedBy::Guard(b) => alloc::format!("no shot — {}, my line!", g.gd[b].callsign),
    }
}

/// The panel's keeping of its rows in the browser's storage: host work, nothing of the game's to do. (clearline.ts saveClearLine)
pub fn save_clear_line(g: &mut crate::game::Game) {}

/// His row of the rule's memory, made on first asking. (clearline.ts memOf)
pub fn mem_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::clearline::Mem {
    g.m.clearline.mem.get_or_insert_with(gd, || Mem {
        since: -100.0,
        seen_t: -100.0,
        said_t: -100.0,
        said: String::new(),
        doing: None,
        rest_until: -100.0,
        try_t: -100.0,
        run_t: -100.0,
        run_of: None,
        lead_t: -100.0,
        lead_p: None,
        lead_of: None,
        lead_for: None,
    })
}

/// What a living man on a script is doing about it. (clearline.ts liveDoing)
pub fn live_doing(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::clearline::Doing> {
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].script.is_none() {
        return None;
    }
    g.m.clearline.mem.get(gd).and_then(|row| row.doing)
}

/// (clearline.ts chestOf)
pub fn chest_of(g: &mut crate::game::Game, h: GuardId) -> V3 {
    let p = g.ch[g.gd[h].ch].pos;
    [p[0], 1.2, p[2]]
}

/// The point on the man this was begun on: his chest, or his head. (clearline.ts aimOf)
pub fn aim_of(g: &mut crate::game::Game, d: crate::clearline::Doing) -> V3 {
    if !d.head {
        return chest_of(g, d.of);
    }
    let c = g.gd[d.of].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(&h) => h,
        None => {
            let p = g.ch[c].pos;
            [p[0], p[1] + 1.6, p[2]]
        }
    }
}

/// (clearline.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// How far `p` is from the segment a → b in plan, and how far along it (0‥1, clamped). (clearline.ts offSeg)
pub fn off_seg(p: V3, a: V3, b: V3) -> [f64; 2] {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let l2 = dx * dx + dz * dz;
    let t = if l2 < 1e-6 { 0.0 } else { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) };
    [m::hyp2(a[0] + dx * t - p[0], a[2] + dz * t - p[2]), t]
}

/// Sam is in this man's line only by the wider lane a player on the move is owed. (clearline.ts samAtEdge)
pub fn sam_at_edge(g: &mut crate::game::Game, gd: GuardId, to: V3) -> bool {
    let c = g.gd[gd].ch;
    let muzzle = g.ch[c].muzzle;
    if crate::roomfight::room_fight_friend_in_line(g, muzzle, to, gd, Some(true)).is_some() {
        return false;
    }
    let shoulder = match g.ch[c].bones.get(crate::character::Bone::UpperArmR) {
        Some(&b) => b,
        None => v3::add(g.ch[c].pos, [0.0, 1.4, 0.0]),
    };
    crate::roomfight::room_fight_friend_in_line(g, shoulder, to, gd, Some(true)).is_none()
}

/// The friend in his line leads on that man: nearer him and walking at him, or was inside `secs`. (clearline.ts leads)
pub fn leads(g: &mut crate::game::Game, gd: GuardId, by: crate::clearline::LeadsBy, tgt: GuardId, to: V3, secs: f64) -> bool {
    let row = mem_of(g, gd);
    let same = lead_of_is(row.lead_of, by);
    if !same || row.lead_for != Some(tgt) {
        row.lead_t = -100.0;
    }
    row.lead_for = Some(tgt);
    let p = if same { row.lead_p } else { None };
    let t = g.time;
    let f = match by {
        LeadsBy::Player => g.pl[g.player].ch,
        LeadsBy::Guard(b) => g.gd[b].ch,
    };
    let (f_pos, f_vel) = (g.ch[f].pos, g.ch[f].vel);
    let dx = to[0] - f_pos[0];
    let dz = to[2] - f_pos[2];
    let l = m::hyp2(dx, dz);
    let dt = match p {
        Some(q) => t - q[2],
        None => 0.0,
    };
    let (vx, vz) = match p {
        Some(q) if dt > 0.1 && dt < 1.5 => ((f_pos[0] - q[0]) / dt, (f_pos[2] - q[1]) / dt),
        _ => (f_vel[0], f_vel[2]),
    };
    if p.is_none() || dt > 0.1 || dt < 0.0 {
        let row = mem_of(g, gd);
        row.lead_p = Some([f_pos[0], f_pos[2], t]);
        row.lead_of = Some(by);
    }
    let own = g.ch[g.gd[gd].ch].pos;
    if l > 0.5 && l < flat(own, to) {
        let at = (vx * dx + vz * dz) / l;
        let across = m::abs(vx * dz - vz * dx) / l;
        if at >= LEAD_V && at >= across {
            mem_of(g, gd).lead_t = t;
        }
    }
    let lead_t = mem_of(g, gd).lead_t;
    t - lead_t < secs && t >= lead_t
}

/// The player is on the move, now or inside the last `secs` seconds; one record a game. (clearline.ts samMoves)
pub fn sam_moves(g: &mut crate::game::Game, secs: f64) -> bool {
    let t = g.time;
    let f = g.pl[g.player].ch;
    let (f_pos, f_vel) = (g.ch[f].pos, g.ch[f].vel);
    let asked = m::hyp2(f_vel[0], f_vel[2]) >= LEAD_V;
    let s = match g.m.clearline.sam_at.as_mut() {
        Some(s) if !(t < s.p[2]) => s,
        _ => {
            g.m.clearline.sam_at = Some(ClearlineModSamAtValue { p: [f_pos[0], f_pos[2], t], move_t: if asked { t } else { -100.0 } });
            return asked;
        }
    };
    let dt = t - s.p[2];
    if dt > 0.1 {
        let moved = if dt < 1.5 { m::hyp2(f_pos[0] - s.p[0], f_pos[2] - s.p[1]) / dt >= LEAD_V } else { asked };
        if moved {
            s.move_t = t;
        }
        s.p = [f_pos[0], f_pos[2], t];
    }
    t - s.move_t < secs && t >= s.move_t
}

/// Is this man between the shove and his point on a door tool's order? Reads the door tools' record only. (clearline.ts entering)
pub fn entering(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(a) = crate::orders::squad_orders_of(g, gd) else { return false };
    let found = g.door_tools.ops.iter().copied().find(|&o| {
        let r = &g.tab.doortools_door_op[o];
        r.men.contains(&gd) && r.mine.get(gd) == Some(&a)
    });
    let Some(op) = found else { return false };
    let r = &g.tab.doortools_door_op[op];
    if r.phase != crate::doortools::Phase::Enter {
        return r.phase == crate::doortools::Phase::Toss
            || r.phase == crate::doortools::Phase::Fuse
            || r.phase == crate::doortools::Phase::Wait
            || r.phase == crate::doortools::Phase::Pocket
            || (r.phase == crate::doortools::Phase::Kick && r.kick.as_ref().map_or(false, |k| !k.pick.unwrap_or(false) && !k.t.is_nan()));
    }
    if r.queue.iter().any(|q| q.gd == gd) {
        return true;
    }
    if !r.pod.has(gd) {
        return false;
    }
    let on_pod = r.on_pod.has(gd);
    let same = matches!(g.m.clearline.entered.get(gd), Some(e) if e.op == op);
    if !same {
        g.m.clearline.entered.set(gd, ClearlineModEnteredValue { op, a, past: false });
    }
    let e = g.m.clearline.entered.get_mut(gd).unwrap();
    if on_pod || e.a != a {
        e.past = true;
    }
    !e.past
}

/// A round from `p` at `to` is clear by the fire gate's own tests, from everywhere the gate may measure it. (clearline.ts shotFrom)
pub fn shot_from(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool {
    let dx = to[0] - p[0];
    let dz = to[2] - p[2];
    let l = m::hyp2(dx, dz);
    if l < 0.5 {
        return false;
    }
    let ux = dx / l;
    let uz = dz / l;
    let sp = g.k.clearline.spare_m;
    if crate::scene_collision::shot_blocked(g, [p[0], 1.4, p[2]], to, None, None) {
        return false;
    }
    for ahead in [0.0, 0.55] {
        for side in [-sp, SHOULDER + sp] {
            let from: V3 = [p[0] + ux * ahead - uz * side, 1.4, p[2] + uz * ahead + ux * side];
            if crate::roomfight::room_fight_friend_in_line(g, from, to, gd, None).is_some() {
                return false;
            }
        }
    }
    true
}

/// From `p` the man at `to` is in plain view and a friend is in the gun's line. (clearline.ts masked)
pub fn masked(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool {
    let from: V3 = [p[0], 1.4, p[2]];
    !crate::scene_collision::shot_blocked(g, from, to, None, None) && crate::roomfight::room_fight_friend_in_line(g, from, to, gd, None).is_some()
}

/// May he stand at `p` and shoot his man from it. (clearline.ts spotOk)
pub fn spot_ok(g: &mut crate::game::Game, gd: GuardId, p: V3, to: V3) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || crate::roomfight::room_fight_funnel_of(g, p).is_some() {
        return false;
    }
    for [ox, oz] in room() {
        if crate::scene_collision::nav_grid_is_blocked(g, p[0] + ox, p[2] + oz) {
            return false;
        }
    }
    let body_m = g.k.clearline.body_m;
    for b in bodies_of(g, gd) {
        if flat(b, p) < body_m {
            return false;
        }
    }
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd {
            continue;
        }
        if let Some(d) = live_doing(g, o) {
            if d.what != What::Kneel && flat(d.spot, p) < 0.8 {
                return false;
            }
        }
    }
    for i in 0..g.civilians.len() {
        let o = g.civilians[i];
        if g.gd[o].state == crate::game::GuardState::Dead {
            continue;
        }
        let [d, t] = off_seg(g.ch[g.gd[o].ch].pos, p, to);
        if d < 0.5 && t > 0.02 && t < 1.0 {
            return false;
        }
    }
    for i in 0..g.fight.ops.len() {
        let x = g.fight.ops[i];
        let (x_gd, x_tgt) = (g.tab.roomfight_op[x].gd, g.tab.roomfight_op[x].tgt);
        if x_gd == gd || g.gd[x_gd].state == crate::game::GuardState::Dead {
            continue;
        }
        let Some(x_tgt) = x_tgt else { continue };
        if g.gd[x_tgt].state == crate::game::GuardState::Dead {
            continue;
        }
        let [d, t] = off_seg(p, g.ch[g.gd[x_gd].ch].pos, g.ch[g.gd[x_tgt].ch].pos);
        if d < 0.6 && t > 0.02 && t < 1.0 {
            return false;
        }
    }
    let me = g.player;
    if !g.pl[me].down && crate::game::player_gun_in_hand(g, me).is_some() {
        let s = g.ch[g.pl[me].ch].pos;
        let a = g.aim_point;
        let len = flat(s, a);
        if len > 1.0 {
            let k = m::min(1.0, 15.0 / len);
            let end: V3 = [s[0] + (a[0] - s[0]) * k, 0.0, s[2] + (a[2] - s[2]) * k];
            let [d, t] = off_seg(p, s, end);
            if d < 0.5 && t > 0.02 {
                return false;
            }
        }
    }
    shot_from(g, gd, p, to)
}

/// The step: the cheapest spot within reach that opens the line, on a lattice laid along the line to the man. (clearline.ts findSpot)
pub fn find_spot(g: &mut crate::game::Game, gd: GuardId, to: V3) -> Option<crate::clearline::FindSpot> {
    let (reach_m, body_m) = (g.k.clearline.reach_m, g.k.clearline.body_m);
    let c = g.ch[g.gd[gd].ch].pos;
    let dx = to[0] - c[0];
    let dz = to[2] - c[2];
    let d0 = m::hyp2(dx, dz);
    if d0 < 0.5 {
        return None;
    }
    let ux = dx / d0;
    let uz = dz / d0; // (side > 0 = his RIGHT as he faces the man: yaw 0 looks down +z, his right hand is at −x)
    // (p, along, side, cost)
    let mut cands: Vec<(V3, f64, f64, f64)> = Vec::new();
    let mut along = -1.5;
    while along <= reach_m + 1e-6 {
        for w in [0.5, 0.7, 0.9, 1.2, 1.6, 2.0, 2.5, 3.0] {
            for side in [w, -w] {
                let walk = m::hyp2(along, side);
                if walk > reach_m + 1e-6 {
                    continue;
                }
                let p: V3 = [c[0] + ux * along - uz * side, 0.0, c[2] + uz * along + ux * side];
                let d1 = flat(p, to);
                if d1 < m::min(d0, 2.5) {
                    continue;
                }
                cands.push((p, along, side, walk + 0.5 * m::max(0.0, d0 - d1)));
            }
        }
        along += 0.5;
    }
    js::sort_by(&mut cands, |a, b| a.3 - b.3);
    for &(p, along, side, _) in cands.iter() {
        if !spot_ok(g, gd, p, to) {
            continue;
        }
        if crate::scene_collision::solid_blocked(g, [c[0], 0.45, c[2]], [p[0], 0.45, p[2]], None) {
            continue;
        }
        let mut through = false;
        for b in bodies_of(g, gd) {
            let [d, t] = off_seg(b, c, p);
            if d < body_m - 0.1 && t > 0.15 {
                through = true;
                break;
            }
        }
        if through {
            continue;
        }
        return Some(FindSpot { p, along, side });
    }
    None
}

/// The panel's and the probes' log of what the rule did. (clearline.ts note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: crate::clearline::NoteWhat, by: &str, at: Option<V3>, walk: Option<f64>) {
    let walk = walk.unwrap_or(0.0);
    let what = match what {
        NoteWhat::Cover => ClearNoteWhat::Cover,
        NoteWhat::Step => ClearNoteWhat::Step,
        NoteWhat::End => ClearNoteWhat::End,
        NoteWhat::Kneel => ClearNoteWhat::Kneel,
    };
    let row = ClearNote { t: g.time, who: g.gd[gd].callsign.clone(), what, by: String::from(by), at: at.map(|a| [a[0], a[2]]), walk };
    let log = &mut g.m.clearline.clearline_log;
    log.push(row);
    if log.len() > 300 {
        log.drain(0..100);
    }
}

/// He says what he is doing, not too often. (clearline.ts cue)
pub fn cue(g: &mut crate::game::Game, gd: GuardId, text: &str) {
    let now = g.time;
    let row = mem_of(g, gd);
    if (text == row.said && now - row.said_t < 6.0) || now - row.said_t < 1.2 {
        return;
    }
    row.said_t = now;
    row.said = String::from(text);
    g.stats.clearline.cues += 1;
    crate::game::game_say(g, gd, text, None, None, None);
}

/// His step / cover / knee is over; `rest` s of being left alone by the rule. (clearline.ts endIt)
pub fn end_it(g: &mut crate::game::Game, gd: GuardId, rest: Option<f64>) {
    let rest = rest.unwrap_or(0.0);
    mem_of(g, gd).doing = None;
    g.stats.clearline.ended += 1;
    note(g, gd, NoteWhat::End, "", None, None);
    if rest > 0.0 {
        let until = g.time + rest;
        mem_of(g, gd).rest_until = until;
    }
}

/// What he is doing about a masked shot right now, if anything, and the end of it. The row itself. (clearline.ts doing)
pub fn doing<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> Option<&'g mut crate::clearline::Doing> {
    let d = g.m.clearline.mem.get(gd)?.doing?; // (what it held as asked: to read; every write goes to the row)
    let k = g.k.clearline;
    let op = crate::roomfight::room_fight_op_of(g, gd);
    let t = g.time;
    let gone = !k.on
        || g.gd[gd].state == crate::game::GuardState::Dead
        || g.gd[gd].script.is_none()
        || op.is_none()
        || !g.fight.live
        || g.gd[d.of].state == crate::game::GuardState::Dead
        || t < d.t0
        || match crate::orders::squad_orders_of(g, gd) {
            Some(a) => g.tab.orders_assignment[a].t0 != d.order_t,
            None => true,
        }
        || (d.what != What::Step && crate::roomfight::room_fight_short(g, gd));
    if gone {
        end_it(g, gd, None);
        return None;
    }
    if d.what == What::Step {
        if op.and_then(|o| g.tab.roomfight_op[o].tgt) == Some(d.of) {
            live(g, gd).on_t = t;
        } else if t - d.on_t > k.lose {
            end_it(g, gd, None);
            return None;
        }
        if t > d.eta && flat(g.ch[g.gd[gd].ch].pos, d.spot) >= 0.5 {
            end_it(g, gd, Some(3.0));
            return None;
        }
        return Some(live(g, gd));
    }
    if t - d.t0 > k.hold_max {
        end_it(g, gd, Some(k.rest));
        return None;
    }
    if t >= d.until {
        let to = aim_of(g, d);
        if masked(g, gd, d.from, to) {
            live(g, gd).until = t + 0.5;
        } else {
            end_it(g, gd, None);
            return None;
        }
    }
    Some(live(g, gd))
}

/// orders.ts: this man's stuck clocks wait. (clearline.ts clearLineLive)
pub fn clear_line_live(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.k.clearline.on && doing(g, gd).is_some()
}

/// orders.ts, as it poses him: where his feet go while this lasts; None: as ordered. (clearline.ts clearLineFeet)
pub fn clear_line_feet(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::clearline::ClearLineFeet> {
    if !g.k.clearline.on {
        return None;
    }
    let d = *doing(g, gd)?; // (what it held as asked: to read; every write goes to the row)
    let k = g.k.clearline;
    let t = g.time;
    let p = g.ch[g.gd[gd].ch].pos;
    let face = v3::yaw_to(p, g.ch[g.gd[d.of].ch].pos);
    let there = flat(p, d.spot) < 0.5;
    if d.what != What::Kneel && t >= d.look_t {
        live(g, gd).look_t = t + 0.2;
        let mine = match crate::roomfight::room_fight_op_of(g, gd) {
            Some(o) => g.tab.roomfight_op[o].tgt == Some(d.of),
            None => false,
        };
        let to = aim_of(g, d);
        if !there {
            if mine && spot_ok(g, gd, [p[0], 0.0, p[2]], to) {
                let row = live(g, gd);
                row.what = What::Step;
                row.spot = [p[0], 0.0, p[2]];
                row.until = m::INF;
                row.eta = m::INF;
                row.good_t = t;
                g.stats.clearline.planted += 1;
            }
        } else if d.what == What::Step {
            if !mine || shot_from(g, gd, p, to) || crate::roomfight::room_fight_friend_in_line(g, [p[0], 1.4, p[2]], to, gd, None).is_some() {
                live(g, gd).good_t = t;
            } else if t - d.good_t > k.stale {
                end_it(g, gd, Some(3.0));
                return None;
            }
        }
    }
    let d = *live(g, gd); // (again: a plant above changed what he is doing)
    if d.what == What::Kneel {
        return Some(ClearLineFeet { goal: d.spot, speed: 0.0, face: None, strafe: false, crouch: true });
    }
    Some(ClearLineFeet { goal: d.spot, speed: m::min(k.pace, g.k.pace.max), face: Some(face), strafe: true, crouch: d.what == What::Cover && there })
}

/// A cover spot is a place a man may stand. (clearline.ts coverOk)
pub fn cover_ok(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool {
    if crate::roomfight::room_fight_funnel_of(g, p).is_some() {
        return false;
    }
    let body_m = g.k.clearline.body_m;
    for b in bodies_of(g, gd) {
        if flat(b, p) < body_m {
            return false;
        }
    }
    for i in 0..g.guards.len() {
        let o = g.guards[i];
        if o == gd {
            continue;
        }
        if let Some(d) = live_doing(g, o) {
            if d.what != What::Kneel && flat(d.spot, p) < 0.8 {
                return false;
            }
        }
    }
    true
}

/// roomfight.ts (tickOp): this frame his shot at `tgt` is masked by a friend. True: this rule has it. (clearline.ts clearLineBlocked)
pub fn clear_line_blocked(g: &mut crate::game::Game, gd: GuardId, tgt: GuardId, to: V3, by: crate::clearline::ClearLineBlockedBy) -> bool {
    let k = g.k.clearline;
    if !k.on || g.gd[gd].script.is_none() {
        return false;
    }
    let Some(a) = crate::orders::squad_orders_of(g, gd) else { return false };
    let t = g.time;
    if t < mem_of(g, gd).rest_until {
        return false;
    }
    let mut ladder = !crate::roomfight::room_fight_short(g, gd);
    if let ClearLineBlockedBy::Guard(b) = by {
        let v = g.ch[g.gd[b].ch].vel;
        if m::hyp2(v[0], v[2]) > 0.5 {
            return false;
        }
        let on_him = match crate::roomfight::room_fight_op_of(g, b) {
            Some(o) => g.tab.roomfight_op[o].tgt == Some(tgt),
            None => false,
        };
        if on_him && t >= g.gd[b].dazzled_until {
            return false;
        }
        ladder = ladder && g.tab.orders_assignment[a].arrived;
    }
    let is_player = matches!(by, ClearLineBlockedBy::Player);
    g.stats.clearline.blocked += 1;
    let row = mem_of(g, gd);
    if t - row.seen_t > 0.5 || t < row.seen_t {
        row.since = t;
    }
    if t - row.seen_t > 0.15 || t < row.seen_t || row.run_of != Some(tgt) {
        row.run_t = t;
        row.run_of = Some(tgt);
    }
    row.seen_t = t;
    let c = g.ch[g.gd[gd].ch].pos;
    let moves = k.still > 0.0 && is_player && sam_moves(g, k.still) && !(k.still_far > 0.0 && flat(c, g.ch[g.gd[tgt].ch].pos) <= k.still_far);
    let mut d = doing(g, gd).map(|x| *x); // (what it held as asked: to read; every write goes to the row)
    if let Some(x) = d {
        if x.of != tgt {
            end_it(g, gd, None);
            d = None;
        }
    }
    if let Some(x) = d {
        if t < x.ask_t {
            return true;
        }
        live(g, gd).ask_t = t + k.again;
        if x.what == What::Cover {
            return true;
        }
        if x.what == What::Step && t <= x.eta && flat(c, x.spot) > 0.25 && spot_ok(g, gd, x.spot, to) {
            return true;
        }
    } else {
        let row = mem_of(g, gd);
        if t - row.since < k.after || t < row.try_t {
            return true;
        }
        let run_t = row.run_t;
        if k.after_edge > k.after && is_player && t - run_t < k.after_edge && sam_at_edge(g, gd, to) {
            return true;
        }
    }
    if crate::silent::silent_holds(g, gd, tgt) {
        return false;
    }
    let kneeling = matches!(d, Some(x) if x.what == What::Kneel);
    if moves {
        if kneeling {
            return true;
        }
        g.stats.clearline.moving += 1;
        cue(g, gd, "no shot — you're in my line!");
        look_later(g, gd, d.is_some(), t + k.again);
        return false;
    }
    if k.hold_behind > 0.0 && leads(g, gd, lead_by(by), tgt, to, k.hold_behind) {
        if kneeling {
            return true;
        }
        g.stats.clearline.behind += 1;
        let line = my_line(g, by);
        cue(g, gd, &line);
        look_later(g, gd, d.is_some(), t + k.again);
        return false;
    }
    let who = match by {
        ClearLineBlockedBy::Player => String::from("Player"),
        ClearLineBlockedBy::Guard(b) => g.gd[b].callsign.clone(),
    };
    if let Some(s) = find_spot(g, gd, to) {
        let walk = m::hyp2(s.along, s.side);
        let step = fresh(g, gd, tgt, to, a, What::Step, s.p, walk, m::INF);
        mem_of(g, gd).doing = Some(step);
        g.stats.clearline.steps += 1;
        if s.along > m::abs(s.side) {
            g.stats.clearline.up += 1;
        }
        note(g, gd, NoteWhat::Step, &who, Some(s.p), Some(walk));
        let line = if s.along > m::abs(s.side) {
            "blocked — moving up."
        } else if -s.along > m::abs(s.side) {
            "blocked — stepping back."
        } else if s.side > 0.0 {
            "blocked — stepping right."
        } else {
            "blocked — stepping left."
        };
        cue(g, gd, line);
        return true;
    }
    if kneeling {
        return true;
    }
    if !ladder {
        look_later(g, gd, d.is_some(), t + k.again);
        return false;
    }
    if crate::scene_collision::shot_blocked(g, [c[0], 1.4, c[2]], to, None, None) {
        look_later(g, gd, d.is_some(), t + k.again);
        return false;
    }
    let tgt_pos = g.ch[g.gd[tgt].ch].pos;
    if let Some(cov) = crate::suppression::cover_from(g, gd, tgt_pos) {
        if cover_ok(g, gd, cov) {
            let walk = flat(c, cov);
            let cover = fresh(g, gd, tgt, to, a, What::Cover, cov, walk, t + walk / m::max(0.5, k.pace) + k.cover_secs);
            mem_of(g, gd).doing = Some(cover);
            g.stats.clearline.covers += 1;
            note(g, gd, NoteWhat::Cover, &who, Some(cov), Some(walk));
            cue(g, gd, "no shot — getting off the line.");
            return true;
        }
    }
    if crate::roomfight::room_fight_funnel_of(g, c).is_none() {
        let knee = fresh(g, gd, tgt, to, a, What::Kneel, [c[0], 0.0, c[2]], 0.0, t + 0.5);
        mem_of(g, gd).doing = Some(knee);
        g.stats.clearline.kneels += 1;
        note(g, gd, NoteWhat::Kneel, &who, None, None);
        let line = my_line(g, by);
        cue(g, gd, &line);
        return true;
    }
    g.stats.clearline.nothing += 1;
    look_later(g, gd, d.is_some(), t + k.again);
    false
}

/// `M.tryT = …; if (D) endIt(g, gd, M)`, the rule's way of giving a look up. (clearline.ts:358, 363, 378, 379, 396)
fn look_later(g: &mut Game, gd: GuardId, was_doing: bool, try_t: f64) {
    mem_of(g, gd).try_t = try_t;
    if was_doing {
        end_it(g, gd, None);
    }
}

/// A new step / cover / knee on `tgt`. (clearline.ts:366 fresh)
fn fresh(g: &mut Game, gd: GuardId, tgt: GuardId, to: V3, a: OrdersAssignmentId, what: What, spot: V3, walk: f64, until: f64) -> Doing {
    let k = g.k.clearline;
    let t = g.time;
    let c = g.ch[g.gd[gd].ch].pos;
    let tc = g.gd[tgt].ch;
    let chest_y = match g.ch[tc].bones.get(crate::character::Bone::Chest) {
        Some(b) => b[1],
        None => g.ch[tc].pos[1] + 1.1,
    };
    Doing {
        what,
        spot,
        of: tgt,
        t0: t,
        on_t: t,
        until,
        ask_t: t + k.again,
        look_t: t + 0.2,
        from: [c[0], 0.0, c[2]],
        order_t: g.tab.orders_assignment[a].t0,
        eta: t + 2.0 * walk / m::max(0.5, k.pace) + 2.0,
        good_t: t,
        head: to[1] - chest_y > 0.2,
    }
}

/// The panel's line. (clearline.ts clearLineSummary)
pub fn clear_line_summary(g: &mut crate::game::Game) -> String {
    if !g.k.clearline.on {
        return String::from("off");
    }
    let mut now: Vec<String> = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if let Some(d) = live_doing(g, gd) {
            now.push(alloc::format!("{} {}", g.gd[gd].callsign, d.what.as_str()));
        }
    }
    let s = &g.stats.clearline;
    let tail = if now.is_empty() { String::new() } else { alloc::format!(" · now: {}", now.join(", ")) };
    alloc::format!("on · steps {} ({} up level) · cover {} · knee {} · said {}{}", s.steps, s.up, s.covers, s.kneels, s.cues, tail)
}

/// What a man is doing about it this moment. (clearline.ts clearLineOf)
pub fn clear_line_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::clearline::ClearLineOf> {
    let d = if g.k.clearline.on { doing(g, gd) } else { None }?;
    Some(ClearLineOf { what: d.what, spot: d.spot, of: d.of })
}

/// The generator's shape has no type in the skeleton (it came out `crate::Opaque`); this part uses `bodies_of`. (clearline.ts bodies)
pub fn bodies(g: &mut crate::game::Game, self_: GuardId) -> crate::Opaque {
    todo!("needs: bodies to return Vec<V3>; bodies_of in this file is the body")
}

#[cfg(test)]
mod tests {
    use super::{flat, off_seg};
    /// The TypeScript's own bits (bun, EXACT.on): offSeg's distance and t, then flat(p, b).
    #[test]
    fn plan_distances_match_the_typescript() {
        let a = [0.1, 0.0, 0.2];
        let b = [7.3, 0.0, -4.9];
        let cases: [([f64; 3], [f64; 3], [f64; 3], [u64; 3]); 4] = [
            ([1.25, 0.0, -3.7], a, b, [0x400424677fd74da1, 0x3fd7288bb0af3462, 0x4018abe38e57b736]),
            ([9.0, 0.0, 9.0], a, b, [0x4028a696ddb5a5f8, 0x3fcf9182b6813ba9, 0x402c01d40df9c5ef]),
            ([-4.0, 0.0, 2.0], a, b, [0x4011e9301f5f39d3, 0x0000000000000000, 0x402a7aed28db5603]),
            ([1.0, 0.0, 1.0], [2.0, 0.0, 2.0], [2.0, 0.0, 2.0], [0x3ff6a09e667f3bcd, 0x0000000000000000, 0x3ff6a09e667f3bcd]),
        ];
        for (p, s, e, bits) in cases {
            let [d, t] = off_seg(p, s, e);
            assert_eq!(d.to_bits(), bits[0]);
            assert_eq!(t.to_bits(), bits[1]);
            assert_eq!(flat(p, e).to_bits(), bits[2]);
        }
    }
}
