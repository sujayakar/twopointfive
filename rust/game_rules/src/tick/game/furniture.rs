use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

/// one row of the walk over who has hands
#[derive(Clone, Copy)]
struct WhoRow { c: CharId, can: bool, right_free: bool, gd: Option<GuardId>, pl: Option<PlayerId> }

/// `before[k]`, with the language's answer past the end (undefined: a NaN by the time it is subtracted)
fn was_at(before: &[f64], k: Option<usize>) -> f64 {
    match k { Some(i) if i < before.len() => before[i], _ => m::NAN }
}

/// what spot() closes over: the leaf a frame on, his side of it, his shoulders
#[derive(Clone, Copy)]
struct Leaf { u: [f64; 2], fx: f64, fz: f64, lo: f64, hi: f64, crack: bool, chest: V3, rgt: [f64; 2], arm_l: Option<V3>, arm_r: Option<V3> }

/// where one hand goes on the leaf
#[derive(Clone, Copy, Debug)]
struct Spot { p: V3, reach: bool, dist: f64 }

/// the spot for one hand (`right`: the right), off that shoulder joint
fn spot(l: &Leaf, right: bool) -> Spot {
    let (u, rgt, chest, fx, fz, lo, hi) = (l.u, l.rgt, l.chest, l.fx, l.fz, l.lo, l.hi);
    let side = if right { 0.2 } else { -0.2 };
    let sh = match if right { l.arm_r } else { l.arm_l } {
        Some(s) => s,
        None => [chest[0] + rgt[0] * side, chest[1] + 0.15, chest[2] + rgt[1] * side],
    };
    let ex = fx - sh[0];
    let ez = fz - sh[2];
    let b = u[0] * ex + u[1] * ez;
    let dh = m::hyp2(ex - u[0] * b, ez - u[1] * b);
    let y = v3::clamp(sh[1] - (if l.crack { v3::lerp(0.22, 0.1, v3::smoothstep(0.32, 0.5, dh)) } else { 0.1 }), 0.85, 1.36);
    let inb = if right { -0.05 } else { 0.05 };
    let mut s = v3::clamp((sh[0] + rgt[0] * inb - fx) * u[0] + (sh[2] + rgt[1] * inb - fz) * u[1] + (if l.crack { 0.08 } else { 0.0 }), lo, hi);
    let mut reach = false;
    let rp = 0.5;
    let dy = y - sh[1];
    let r2 = rp * rp - dy * dy - dh * dh;
    if r2 > 0.0 {
        let half = m::sqrt(r2) - 0.03;
        let s0 = -b - half;
        let s1 = -b + half;
        if half > 0.0 && s1 >= lo && s0 <= hi {
            s = v3::clamp(s, m::max(s0, lo), m::min(s1, hi));
            reach = true;
        }
    }
    if !reach { s = v3::clamp(-b, lo, hi); }
    let m2 = 0.33 * 0.33 - dy * dy - dh * dh;
    if m2 > 0.0 {
        let q = m::sqrt(m2);
        if m::abs(s + b) < q {
            s = if -b + q <= hi { -b + q } else if -b - q >= lo { -b - q } else { s };
        }
    }
    let p: V3 = [fx + u[0] * s, y, fz + u[1] * s];
    Spot { p, reach, dist: m::hyp3(p[0] - sh[0], p[1] - sh[1], p[2] - sh[2]) }
}

/// a hand that has stopped pushing stays on the leaf this long, and is let go of it over this long after
const FOLLOW: f64 = 0.12;
const FADE: f64 = 0.4;

/// who is pushing which leaf open right now, and where on it his free hand goes
pub(super) fn game_hands_on_doors(g: &mut crate::game::Game, before: &[f64]) {
    let now = g.time;
    let list: Vec<DoorId> = g.doors.list.clone();
    let who = who_have_hands(g);
    for c in g.door_hand_recs.keys() {
        if !who.iter().any(|w| w.c == c) { g.door_hand_recs.delete(c); }
    }
    for w in who.iter() {
        let rec = leaf_in_hand(g, w, &list, before);
        match rec {
            Some(r) if now - r.t <= FOLLOW + FADE => hand_on_leaf(g, w, r, &list, before),
            _ => {
                if rec.is_some() { g.door_hand_recs.delete(w.c); }
                hand_at_work(g, w);
            }
        }
    }
}

/// every man with hands: the players, the guards and the civilians in sight, whether he can use them now and whether the right one is free
fn who_have_hands(g: &Game) -> Vec<WhoRow> {
    let mut who: Vec<WhoRow> = Vec::new();
    for s in g.seats.clone() {
        let pl = crate::cmd::seat_man(g, s);
        let p = &g.pl[pl];
        who.push(WhoRow { c: p.ch, pl: Some(pl), gd: None, can: !g.cond[p.ch].down && p.tow.is_none() && !p.planted(), right_free: p.holstered });
    }
    for &gd in g.guards.iter() {
        let m_ = &g.gd[gd];
        who.push(WhoRow {
            c: m_.ch,
            gd: Some(gd),
            pl: None,
            can: !g.cond[m_.ch].down && m_.held.is_none(),
            right_free: m_.state == crate::game::GuardState::Patrol && !m_.drawn && m_.dropped.is_some(),
        });
    }
    for cv in crate::civilians::civilians_shown(g) {
        let m_ = &g.gd[cv];
        who.push(WhoRow { c: m_.ch, gd: None, pl: None, can: !g.cond[m_.ch].down && m_.held.is_none() && g.ch[m_.ch].anim.act.is_none(), right_free: true });
    }
    who
}

/// a leaf a man meets: through a crack of it, and moving it by its handle
struct Met { d: DoorId, crack: bool, move_: bool }

/// The leaf his hand is on or was: the one he meets now, else the one he met last, while he stands by it. None = none.
fn leaf_in_hand(g: &mut Game, w: &WhoRow, list: &[DoorId], before: &[f64]) -> Option<crate::game::GameDoorHandRecsValue> {
    // a copy of the row, written back wherever it changes
    let mut rec = g.door_hand_recs.get(w.c).copied();
    if !w.can {
        if rec.is_some() { g.door_hand_recs.delete(w.c); }
        return None;
    }
    if let Some(met) = leaf_met(g, w, list, before) {
        let now = g.time;
        let r = match rec {
            Some(mut r) => {
                if r.door != met.d { r.hand = -1.0; r.t0 = now; }
                r.door = met.d;
                r.t = now;
                r.crack = met.crack;
                r.move_ = Some(met.move_);
                r
            }
            None => crate::game::GameDoorHandRecsValue { door: met.d, t: now, t0: now, crack: met.crack, hand: -1.0, move_: Some(met.move_) },
        };
        g.door_hand_recs.set(w.c, r);
        rec = Some(r);
    }
    let r = rec?;
    let (h, pos) = (g.dr[r.door].def.hinge, g.ch[w.c].pos);
    if m::hyp2(pos[0] - h[0], pos[2] - h[1]) > g.dr[r.door].def.width + 1.2 {
        g.door_hand_recs.delete(w.c);
        return None;
    }
    rec
}

/// The leaf he meets now: the one he moves by its handle, else the first he pushes, pulls, eases or is about to walk into, else the one a teammate has
/// his hand out to.
fn leaf_met(g: &mut Game, w: &WhoRow, list: &[DoorId], before: &[f64]) -> Option<Met> {
    if let Some(p) = w.pl {
        if let Some(d) = g.pl[p].door_move().map(|dm| dm.d) {
            if !g.dr[d].leaf.off { return Some(Met { d, crack: true, move_: true }); }
        }
    }
    for (k, &d) in list.iter().enumerate() {
        if let Some(met) = meets_leaf(g, w, d, was_at(before, Some(k))) { return Some(met); }
    }
    let d = crate::hands::team_door_hand(g, w.gd?)?;
    Some(Met { d, crack: true, move_: true })
}

/// whether he meets this leaf, which stood at `was` as the frame opened
fn meets_leaf(g: &mut Game, w: &WhoRow, d: DoorId, was: f64) -> Option<Met> {
    if g.dr[d].leaf.off { return None; }
    let (cpos, cvel, cid) = (g.ch[w.c].pos, g.ch[w.c].vel, g.ch[w.c].id);
    let (hinge, wd, thick) = (g.dr[d].def.hinge, g.dr[d].def.width, g.dr[d].thick);
    let vx = cpos[0] - hinge[0];
    let vz = cpos[2] - hinge[1];
    if vx * vx + vz * vz > m::pow(wd + 1.2, 2.0) { return None; }
    if w.pl.map_or(false, |p| g.pl[p].easing().map(|e| e.door) == Some(d)) { return Some(Met { d, crack: true, move_: false }); }
    let met = Some(Met { d, crack: false, move_: false });
    let u = crate::doors::door_dir(g, d);
    let along = vx * u[0] + vz * u[1];
    let perp = u[0] * vz - u[1] * vx;
    let t = v3::clamp(along, 0.0, wd);
    let dist = m::hyp2(vx - u[0] * t, vz - u[1] * t);
    let touching = along > 0.14 && along < wd + 0.3 && dist < 0.34 + thick / 2.0 + 0.05;
    let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
    let angle = g.dr[d].leaf.angle;
    // he pushes it
    if touching && (angle - was) * sgn > 1e-4 { return met; }
    // he has pulled it, and stands by it
    let swing = crate::doors::door_swing(g, d);
    let mine = sgn * swing < 0.0 && angle * swing < 1.35;
    if mine && g.dr[d].leaf.pull_t > 0.0 && g.dr[d].leaf.last_who == cid && dist < 0.9 { return met; }
    // he is about to walk into it
    let speed = m::hyp2(cvel[0], cvel[2]);
    if touching || !(speed > 0.3) { return None; }
    let wx = cvel[0] / speed;
    let wz = cvel[2] / speed;
    let closing = -(wx * -u[1] + wz * u[0]) * m::sign(perp);
    if !(closing > 0.4) { return None; }
    let t_hit = (m::abs(perp) - 0.34 - thick / 2.0) / (closing * speed);
    let ah = along + (wx * u[0] + wz * u[1]) * speed * m::max(0.0, t_hit);
    let at_stop = if sgn < 0.0 { angle <= crate::doors::door_min_a(g, d) + 0.02 } else { angle >= crate::doors::door_max_a(g, d) - 0.02 };
    if !(t_hit >= 0.0 && t_hit < 0.4 && ah > 0.2 && ah < wd + 0.1 && (!at_stop || mine)) { return None; }
    let locked = match w.pl {
        Some(p) => { let keys = g.pl[p].keys.clone(); crate::doors::doors_locked_out(g, d, cpos, Some(&keys)) }
        None => false,
    };
    if locked { None } else { met }
}

/// on no leaf: his hand is on the thing he is working, if he is
fn hand_at_work(g: &mut Game, w: &WhoRow) {
    let wh = if w.can { crate::workthings::work_hand(g, w.c) } else { None };
    let wj = if wh.is_some() { crate::workthings::work_job(g, w.c) } else { None };
    let worked = match wh.as_ref() {
        Some(h) => {
            let hand = crate::handwork::TeamWorkHandH { p: h.p, n: h.n, w: h.w };
            let verb = wj.as_ref().map(|j| j.verb);
            let kind = if verb == Some(crate::workthings::WorkVerb::Drive) { crate::character::WorkKind::Take } else { crate::character::WorkKind::Lever };
            let secs = wj.as_ref().map_or(0.0, |j| j.secs);
            let need = wj.as_ref().map_or(1.0, |j| j.need);
            let dir = if verb == Some(crate::workthings::WorkVerb::BreakerOn) { -1.0 } else { 1.0 };
            crate::handwork::team_work_hand(g, w.c, Some(hand), Some(kind), Some(secs), Some(need), Some(dir))
        }
        None => false,
    };
    g.ch[w.c].anim.door_push = if worked { None } else { wh };
}

/// his hand on the leaf: the one he has settled on, or the one that reaches it better until he has
fn hand_on_leaf(g: &mut Game, w: &WhoRow, mut r: crate::game::GameDoorHandRecsValue, list: &[DoorId], before: &[f64]) {
    let (c, now) = (w.c, g.time);
    let (leaf, nx, nz) = leaf_a_frame_on(g, c, &r, list, before);
    let (yaw, cpos) = (g.ch[c].body_yaw, g.ch[c].pos);
    let (u, rgt) = (leaf.u, leaf.rgt);
    let mut right: bool;
    let mut sp: Spot;
    if r.hand == -1.0 || (r.hand == 1.0 && !w.right_free) {
        right = w.right_free && u[0] * rgt[0] + u[1] * rgt[1] > 0.0;
        sp = spot(&leaf, right);
        if !sp.reach && (right || w.right_free) {
            let so = spot(&leaf, !right);
            if so.reach || so.dist < sp.dist - 0.12 { right = !right; sp = so; }
        }
        let sx = sp.p[0] - cpos[0];
        let sz = sp.p[2] - cpos[2];
        let sl = js::or(m::hyp2(sx, sz), 1.0);
        let settled = if r.move_.unwrap_or(false) {
            (sx * m::sin(yaw) + sz * m::cos(yaw)) / sl > 0.5
        } else {
            -nx * m::sin(yaw) - nz * m::cos(yaw) > 0.5 && now - r.t0 > 0.25
        };
        if settled {
            r.hand = if right { 1.0 } else { 0.0 };
            g.door_hand_recs.set(c, r);
        }
    } else {
        right = r.hand == 1.0;
        sp = spot(&leaf, right);
    }
    g.ch[c].anim.door_push = Some(crate::character::DoorPush {
        p: sp.p,
        n: [nx, 0.0, nz],
        w: if now - r.t <= FOLLOW { 1.0 } else { 0.0 },
        hand: if r.hand == -1.0 { -1.0 } else if right { 1.0 } else { 0.0 },
        reach: r.move_,
    });
}

/// the leaf where it will stand a frame on, his side of it and his shoulders, and the way that side faces
fn leaf_a_frame_on(g: &Game, c: CharId, r: &crate::game::GameDoorHandRecsValue, list: &[DoorId], before: &[f64]) -> (Leaf, f64, f64) {
    let (d, cpos) = (r.door, g.ch[c].pos);
    let hinge = g.dr[d].def.hinge;
    let (hx, hz) = (hinge[0], hinge[1]);
    let angle = g.dr[d].leaf.angle;
    let da = v3::clamp(angle - was_at(before, list.iter().position(|x| *x == d)), -0.3, 0.3);
    let (min_a, max_a) = (crate::doors::door_min_a(g, d), crate::doors::door_max_a(g, d));
    let a = g.dr[d].def.closed_dir + v3::clamp(angle + da, min_a, max_a);
    let u: [f64; 2] = [m::cos(a), m::sin(a)];
    let px = cpos[0] - hx;
    let pz = cpos[2] - hz;
    let face = if u[0] * pz - u[1] * px >= 0.0 { 1.0 } else { -1.0 };
    let nx = -u[1] * face;
    let nz = u[0] * face;
    let yaw = g.ch[c].body_yaw;
    let off = g.dr[d].thick / 2.0;
    let bones = &g.ch[c].bones;
    let leaf = Leaf {
        u,
        fx: hx + nx * off,
        fz: hz + nz * off,
        lo: 0.3,
        hi: g.dr[d].def.width - (if r.crack { 0.1 } else { 0.2 }),
        crack: r.crack,
        chest: match bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => [cpos[0], cpos[1] + 1.25, cpos[2]] },
        rgt: [-m::cos(yaw), m::sin(yaw)],
        arm_l: bones.get(crate::character::Bone::UpperArmL).copied(),
        arm_r: bones.get(crate::character::Bone::UpperArmR).copied(),
    };
    (leaf, nx, nz)
}

/// loose furniture: the same contact pattern as the doors, after everyone has moved
pub fn game_update_props(g: &mut crate::game::Game, dt: f64) {
    let seats: Vec<SeatId> = g.seats.clone();
    let mut chars: Vec<CharId> = Vec::new();
    for s in seats.iter().copied() {
        let pl = crate::cmd::seat_man(g, s);
        if !crate::condition::down(g, g.pl[pl].ch) { chars.push(g.pl[pl].ch); }
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !crate::condition::down(g, g.gd[gd].ch) && !crate::doze::doze_in_chair(g, gd) { chars.push(g.gd[gd].ch); }
    }
    for cv in crate::civilians::civilians_shown(g) {
        if !crate::condition::down(g, g.gd[cv].ch) && !crate::civilians::civilian_seated(g, cv) && crate::civilians::civilian_pushes_props(g, cv) { chars.push(g.gd[cv].ch); }
    }
    let mut contacts: Vec<crate::props::PropContact> = Vec::with_capacity(chars.len());
    for c in chars.iter().copied() {
        let man = crate::seats::man_of(g, Some(c));
        let quiet = crate::game::quiet_man(g, man, c);
        contacts.push(crate::props::PropContact { pos: g.ch[c].pos, vel: g.ch[c].vel, radius: 0.3, who: g.ch[c].id, mass: 75.0, quiet });
    }
    let before: Vec<[f64; 2]> = chars.iter().map(|c| [g.ch[*c].pos[0], g.ch[*c].pos[2]]).collect();
    let list: Vec<DoorId> = g.doors.list.clone();
    let mut any_off = false;
    for d in list.iter().copied() {
        if g.dr[d].leaf.off { any_off = true; break; }
    }
    let leaves: Vec<BoxId> = if any_off {
        let mut l = Vec::new();
        for d in list.iter().copied() {
            if !g.dr[d].leaf.off { l.push(g.dr[d].box_); }
        }
        l
    } else {
        g.col.dynamic_boxes.clone()
    };
    crate::props::prop_system_update(g, dt, &mut contacts, &leaves);
    // a contact's pos is a copy of the man's: what the step moved him by is his
    for (i, c) in chars.iter().enumerate() {
        g.ch[*c].pos[0] = contacts[i].pos[0];
        g.ch[*c].pos[2] = contacts[i].pos[2];
    }
    crate::game::game_roll_tick(g, dt);
    for s in seats.iter().copied() {
        let pl = crate::cmd::seat_man(g, s);
        if !crate::condition::down(g, g.pl[pl].ch) {
            crate::seats::with_seat(g, s, &mut |g: &mut Game| {
                crate::player::settle_player(g);
                crate::player::settle_held(g);
            });
        }
    }
    for (i, c) in chars.iter().enumerate() {
        let p = g.ch[*c].pos;
        if m::abs(p[0] - before[i][0]) + m::abs(p[2] - before[i][1]) > 0.005 { crate::character::character_update(g, *c, 0.0); }
    }
}

pub fn game_drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) { crate::combat::drop_throwable(g, kind, p) }

pub fn game_throw_count(g: &crate::game::Game, seat: &crate::seats::SeatRef, kind: crate::game::ThrowKind) -> f64 { crate::player::throw_count(g, seat, kind) }

#[cfg(test)]
mod tests {
    use super::*;

    /// A leaf from (0, 0) along +z, its handle reach from 0.3 to 0.8 of it, a man at his ease beside it.
    fn leaf(chest: V3, crack: bool) -> Leaf { Leaf { u: [0.0, 1.0], fx: 0.0, fz: 0.0, lo: 0.3, hi: 0.8, crack, chest, rgt: [0.0, -1.0], arm_l: None, arm_r: None } }

    /// A FAILURE MEANS: his hand floats off the door he is pushing, or goes through his own chest to it.
    #[test]
    fn a_hand_goes_on_the_leaf_at_a_height_he_could_hold_it() {
        for chest in [[0.45, 1.25, 0.55], [0.3, 1.1, 0.2], [0.5, 1.4, 0.9], [2.0, 1.25, 0.5]] {
            for (right, crack) in [(false, false), (true, false), (false, true), (true, true)] {
                let s = spot(&leaf(chest, crack), right);
                assert!(s.p[0].abs() < 1e-12, "{chest:?}: on the leaf's line: {:?}", s.p);
                assert!(s.p[2] >= 0.3 - 1e-12 && s.p[2] <= 0.8 + 1e-12, "{chest:?}: on the part of it that takes a hand: {:?}", s.p);
                assert!(s.p[1] >= 0.85 && s.p[1] <= 1.36, "{chest:?}: at a hand's height: {:?}", s.p);
            }
        }
    }

    #[test]
    fn he_reaches_a_leaf_his_arm_is_long_enough_for_and_says_so() {
        let by = spot(&leaf([0.3, 1.25, 0.55], false), false);
        assert!(by.reach, "beside it");
        assert!(by.dist <= 0.5, "within an arm: {}", by.dist);
        let far = spot(&leaf([2.0, 1.25, 0.55], false), false);
        assert!(!far.reach, "two metres off");
        assert!(far.dist > 1.0, "{}", far.dist);
    }

    /// His elbow has room: the hand does not come in nearer his shoulder than a bent arm can put it, when the leaf has somewhere else for it.
    #[test]
    fn a_hand_keeps_an_arms_bend_off_his_shoulder() {
        let s = spot(&leaf([0.1, 1.2, 0.5], false), false);
        assert!(s.dist >= 0.33 - 1e-9, "{}", s.dist);
    }

    #[test]
    fn each_hand_works_off_its_own_shoulder_and_the_joint_is_used_where_it_is_given() {
        let plain = leaf([0.3, 1.25, 0.55], false);
        let (l, r) = (spot(&plain, false), spot(&plain, true));
        assert_ne!(l.p, r.p, "the two hands have different places");
        let mut joint = plain;
        joint.arm_l = Some([0.2, 1.4, 0.4]);
        let j = spot(&joint, false);
        assert!(m::abs(m::hyp3(j.p[0] - 0.2, j.p[1] - 1.4, j.p[2] - 0.4) - j.dist) < 1e-12, "the distance is off the shoulder he has");
        assert_eq!(spot(&joint, true).p, r.p, "the left shoulder is nothing to the right hand");
    }

    /// Through a crack, up close, the hand comes in low and further along than on a door he is shoving flat.
    #[test]
    fn a_hand_at_a_crack_goes_lower_when_he_is_up_against_it() {
        let chest = [0.3, 1.25, 0.55];
        assert!(spot(&leaf(chest, true), false).p[1] < spot(&leaf(chest, false), false).p[1]);
    }
}
