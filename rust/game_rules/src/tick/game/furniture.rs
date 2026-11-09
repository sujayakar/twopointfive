use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

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

/// who is pushing which leaf open right now, and where on it his free hand goes
pub(super) fn game_hands_on_doors(g: &mut crate::game::Game, before: &[f64]) {
    const FOLLOW: f64 = 0.12;
    const FADE: f64 = 0.4;
    let now = g.time;
    let list: Vec<DoorId> = g.doors.list.clone();
    let mut who: Vec<WhoRow> = Vec::new();
    for s in g.seats.clone() {
        let pl = crate::cmd::seat_man(g, s);
        let p = &g.pl[pl];
        who.push(WhoRow {
            c: p.ch,
            pl: Some(pl),
            gd: None,
            can: !p.down && p.dragging.is_none() && p.holding.is_none() && p.kick.is_none() && p.picking.is_none() && p.wanding.is_none() && p.takedown.is_none(),
            right_free: p.holstered,
        });
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        let m_ = &g.gd[gd];
        who.push(WhoRow {
            c: m_.ch,
            gd: Some(gd),
            pl: None,
            can: m_.state != crate::game::GuardState::Dead && m_.held.is_none(),
            right_free: m_.state == crate::game::GuardState::Patrol && !m_.drawn && m_.dropped.is_some(),
        });
    }
    for cv in crate::civilians::civilians_shown(g) {
        let m_ = &g.gd[cv];
        let c = m_.ch;
        who.push(WhoRow { c, gd: None, pl: None, can: m_.state != crate::game::GuardState::Dead && m_.held.is_none() && g.ch[c].anim.act.is_none(), right_free: true });
    }
    for c in g.door_hand_recs.keys() {
        if !who.iter().any(|w| w.c == c) { g.door_hand_recs.delete(c); }
    }
    for w in who.iter().copied() {
        let WhoRow { c, can, right_free, gd, pl } = w;
        let cpos = g.ch[c].pos;
        let cvel = g.ch[c].vel;
        let cid = g.ch[c].id;
        // a copy of the row, written back wherever it changes
        let mut rec: Option<crate::game::GameDoorHandRecsValue> = g.door_hand_recs.get(c).copied();
        if can {
            let mut hit: Option<DoorId> = None;
            let (mut crack, mut move_) = (false, false);
            let speed = m::hyp2(cvel[0], cvel[2]);
            if let Some(p) = pl {
                if let Some(dd) = g.pl[p].door_move.as_ref().map(|dm| dm.d) {
                    if !crate::doors::door_off(g, dd) { hit = Some(dd); crack = true; move_ = true; }
                }
            }
            for k in 0..list.len() {
                if hit.is_some() { break; }
                let d = list[k];
                let hx = g.dr[d].def.hinge[0];
                let hz = g.dr[d].def.hinge[1];
                let wd = g.dr[d].def.width;
                if crate::doors::door_off(g, d) { continue; }
                let vx = cpos[0] - hx;
                let vz = cpos[2] - hz;
                if vx * vx + vz * vz > m::pow(wd + 1.2, 2.0) { continue; }
                if let Some(p) = pl {
                    if g.pl[p].easing == Some(d) { hit = Some(d); crack = true; break; }
                }
                let u = crate::doors::door_dir(g, d);
                let along = vx * u[0] + vz * u[1];
                let perp = u[0] * vz - u[1] * vx;
                let t = v3::clamp(along, 0.0, wd);
                let dist = m::hyp2(vx - u[0] * t, vz - u[1] * t);
                let thick = g.dr[d].thick;
                let touching = along > 0.14 && along < wd + 0.3 && dist < 0.34 + thick / 2.0 + 0.05;
                let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
                let angle = crate::doors::door_angle(g, d);
                let da = angle - was_at(before, Some(k));
                let pushing = touching && da * sgn > 1e-4;
                if pushing { hit = Some(d); break; }
                let swing = crate::doors::door_swing(g, d);
                let mine = sgn * swing < 0.0 && angle * swing < 1.35;
                if mine && crate::doors::door_pull_t(g, d) > 0.0 && crate::doors::door_last_who(g, d) == cid && dist < 0.9 { hit = Some(d); break; }
                if !touching && speed > 0.3 {
                    let wx = cvel[0] / speed;
                    let wz = cvel[2] / speed;
                    let closing = -(wx * -u[1] + wz * u[0]) * m::sign(perp);
                    if closing > 0.4 {
                        let t_hit = (m::abs(perp) - 0.34 - thick / 2.0) / (closing * speed);
                        let ah = along + (wx * u[0] + wz * u[1]) * speed * m::max(0.0, t_hit);
                        let at_stop = if sgn < 0.0 { angle <= crate::doors::door_min_a(g, d) + 0.02 } else { angle >= crate::doors::door_max_a(g, d) - 0.02 };
                        if t_hit >= 0.0 && t_hit < 0.4 && ah > 0.2 && ah < wd + 0.1 && (!at_stop || mine) {
                            let locked = match pl {
                                Some(p) => { let keys = g.pl[p].keys.clone(); crate::doors::doors_locked_out(g, d, cpos, Some(&keys)) }
                                None => false,
                            };
                            if !locked { hit = Some(d); }
                        }
                    }
                }
            }
            if let (None, Some(gd)) = (hit, gd) {
                if let Some(th) = crate::hands::team_door_hand(g, gd) { hit = Some(th); crack = true; move_ = true; }
            }
            if let Some(h) = hit {
                let r = match rec {
                    Some(mut r) => {
                        if r.door != h { r.hand = -1.0; r.t0 = now; }
                        r.door = h;
                        r.t = now;
                        r.crack = crack;
                        r.move_ = Some(move_);
                        r
                    }
                    None => crate::game::GameDoorHandRecsValue { door: h, t: now, t0: now, crack, hand: -1.0, move_: Some(move_) },
                };
                g.door_hand_recs.set(c, r);
                rec = Some(r);
            }
        } else if rec.is_some() {
            g.door_hand_recs.delete(c);
            rec = None;
        }
        if let Some(r) = rec {
            let h = g.dr[r.door].def.hinge;
            if m::hyp2(cpos[0] - h[0], cpos[2] - h[1]) > g.dr[r.door].def.width + 1.2 {
                g.door_hand_recs.delete(c);
                rec = None;
            }
        }
        let gone = match rec { None => true, Some(r) => now - r.t > FOLLOW + FADE };
        if gone {
            if rec.is_some() { g.door_hand_recs.delete(c); }
            let wh = if can { crate::workthings::work_hand(g, c) } else { None };
            let wj = if wh.is_some() { crate::workthings::work_job(g, c) } else { None };
            let worked = match wh.as_ref() {
                Some(h) => {
                    let hand = crate::handwork::TeamWorkHandH { p: h.p, n: h.n, w: h.w };
                    let verb = wj.as_ref().map(|j| j.verb);
                    let kind = if verb == Some(crate::workthings::WorkVerb::Drive) { crate::handwork::TeamWorkHandKind::Take } else { crate::handwork::TeamWorkHandKind::Lever };
                    let secs = wj.as_ref().map_or(0.0, |j| j.secs);
                    let need = wj.as_ref().map_or(1.0, |j| j.need);
                    let dir = if verb == Some(crate::workthings::WorkVerb::BreakerOn) { -1.0 } else { 1.0 };
                    crate::handwork::team_work_hand(g, c, Some(hand), Some(kind), Some(secs), Some(need), Some(dir))
                }
                None => false,
            };
            g.ch[c].anim.door_push = if worked { None } else { wh };
            continue;
        }
        let mut r = rec.unwrap();
        let d = r.door;
        let hinge = g.dr[d].def.hinge;
        let (hx, hz) = (hinge[0], hinge[1]);
        let wd = g.dr[d].def.width;
        let k = list.iter().position(|x| *x == d);
        let angle = crate::doors::door_angle(g, d);
        let da = v3::clamp(angle - was_at(before, k), -0.3, 0.3);
        let (min_a, max_a) = (crate::doors::door_min_a(g, d), crate::doors::door_max_a(g, d));
        let a = g.dr[d].def.closed_dir + v3::clamp(angle + da, min_a, max_a);
        let u: [f64; 2] = [m::cos(a), m::sin(a)];
        let px = cpos[0] - hx;
        let pz = cpos[2] - hz;
        let face = if u[0] * pz - u[1] * px >= 0.0 { 1.0 } else { -1.0 };
        let nx = -u[1] * face;
        let nz = u[0] * face;
        let yaw = g.ch[c].body_yaw;
        let rgt: [f64; 2] = [-m::cos(yaw), m::sin(yaw)];
        let chest: V3 = match g.ch[c].bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => [cpos[0], cpos[1] + 1.25, cpos[2]] };
        let off = g.dr[d].thick / 2.0;
        let leaf = Leaf {
            u,
            fx: hx + nx * off,
            fz: hz + nz * off,
            lo: 0.3,
            hi: wd - (if r.crack { 0.1 } else { 0.2 }),
            crack: r.crack,
            chest,
            rgt,
            arm_l: g.ch[c].bones.get(crate::character::Bone::UpperArmL).copied(),
            arm_r: g.ch[c].bones.get(crate::character::Bone::UpperArmR).copied(),
        };
        let mut right: bool;
        let mut sp: Spot;
        if r.hand == -1.0 || (r.hand == 1.0 && !right_free) {
            right = right_free && u[0] * rgt[0] + u[1] * rgt[1] > 0.0;
            sp = spot(&leaf, right);
            if !sp.reach && (right || right_free) {
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
}

/// loose furniture: the same contact pattern as the doors, after everyone has moved
pub fn game_update_props(g: &mut crate::game::Game, dt: f64) {
    let seats: Vec<SeatId> = g.seats.clone();
    let mut chars: Vec<CharId> = Vec::new();
    for s in seats.iter().copied() {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].down { chars.push(g.pl[pl].ch); }
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].state != crate::game::GuardState::Dead && !crate::doze::doze_in_chair(g, gd) { chars.push(g.gd[gd].ch); }
    }
    for cv in crate::civilians::civilians_shown(g) {
        if g.gd[cv].state != crate::game::GuardState::Dead && !crate::civilians::civilian_seated(g, cv) && crate::civilians::civilian_pushes_props(g, cv) { chars.push(g.gd[cv].ch); }
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
        if crate::doors::door_off(g, d) { any_off = true; break; }
    }
    let leaves: Vec<BoxId> = if any_off {
        let mut l = Vec::new();
        for d in list.iter().copied() {
            if !crate::doors::door_off(g, d) { l.push(g.dr[d].box_); }
        }
        l
    } else {
        g.col.dynamic_boxes.clone()
    };
    crate::props::prop_system_update(g, dt, &mut contacts, &leaves);
    // a contact's pos is the man's own in the TypeScript: what the step moved him by is his
    for (i, c) in chars.iter().enumerate() {
        g.ch[*c].pos[0] = contacts[i].pos[0];
        g.ch[*c].pos[2] = contacts[i].pos[2];
    }
    crate::game::game_roll_tick(g, dt);
    for s in seats.iter().copied() {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].down {
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
