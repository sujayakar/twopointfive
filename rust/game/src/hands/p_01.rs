//! src/game/hands.ts lines 1 to 512
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- private

/// the ragdoll of a man's body, when it has one (`gd.char.ragdoll`)
fn rag_char(g: &Game, gd: GuardId) -> Option<RagdollId> { g.ch[g.gd[gd].ch].ragdoll }
/// `rag.point(i)`
fn pt(g: &mut Game, rc: RagdollId, i: f64) -> V3 { crate::ragdoll::ragdoll_point(g, rc, i, None) }
/// `rag.radius(i)`
fn rad(g: &mut Game, rc: RagdollId, i: f64) -> f64 { crate::ragdoll::ragdoll_radius(g, rc, i) }
/// `rag.setPin(i, p)`
fn pin(g: &mut Game, rc: RagdollId, i: f64, p: V3) { crate::ragdoll::ragdoll_set_pin(g, rc, i, p) }
/// `rag.wake()`
fn wake(g: &mut Game, rc: RagdollId) { crate::ragdoll::ragdoll_wake(g, rc) }
/// the door's hinge on the floor
fn hinge(g: &Game, d: DoorId) -> [f64; 2] { g.dr[d].def.hinge }

// ------------------------------------------------------------------------------------------------ a body

/// HANDS.on && HANDS.body, both settled on on the page. (hands.ts bodyOn)
pub fn body_on(g: &mut crate::game::Game) -> bool { true }

/// HANDS.on && HANDS.door, both settled on on the page. (hands.ts doorOn)
pub fn door_on(g: &mut crate::game::Game) -> bool { true }

/// His hands or feet are spoken for by something else: no door is worked by hand. (hands.ts handsBusy)
pub fn hands_busy(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    let p = &g.pl[pl];
    if p.down
        || p.holding.is_some()
        || p.dragging.is_some()
        || p.takedown.is_some()
        || p.kick.is_some()
        || p.picking.is_some()
        || p.wanding.is_some()
        || p.coding.is_some()
        || p.wall.is_some()
        || p.charging.is_some()
    {
        return true;
    }
    crate::wallcharge::wall_charge_setting(g)
}

/// The middle of two points. (hands.ts mid)
pub fn mid(a: V3, b: V3) -> V3 { [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0, (a[2] + b[2]) / 2.0] }

/// Floor distance from the player to an END of a body, Infinity when a static stands between. (hands.ts endDist)
pub fn end_dist(g: &mut crate::game::Game, gd: GuardId, end: crate::hands::EndDistEnd) -> f64 {
    let Some(rc) = rag_char(g, gd) else { return m::INF };
    let rag = crate::ragdoll::rag();
    let p = g.ch[g.pl[g.player].ch].pos;
    let mp = if end == crate::hands::EndDistEnd::Feet {
        let (a, b) = (pt(g, rc, rag.ank_l), pt(g, rc, rag.ank_r));
        mid(a, b)
    } else {
        let (a, b) = (pt(g, rc, rag.sho_l), pt(g, rc, rag.sho_r));
        mid(a, b)
    };
    if crate::scene_collision::solid_blocked(g, [p[0], 0.5, p[2]], [mp[0], 0.5, mp[2]], None) { m::INF } else { v3::dist_xz(mp, p) }
}

/// player.ts bodyDist's extra term: the reach to a body's HEAD end. (hands.ts bodyGripDist)
pub fn body_grip_dist(g: &mut crate::game::Game, gd: GuardId, within: f64) -> f64 {
    let Some(rc) = rag_char(g, gd) else { return m::INF };
    let rag = crate::ragdoll::rag();
    let (a, b) = (pt(g, rc, rag.sho_l), pt(g, rc, rag.sho_r));
    let p = g.ch[g.pl[g.player].ch].pos;
    if v3::dist_xz(mid(a, b), p) >= within { return m::INF; }
    end_dist(g, gd, crate::hands::EndDistEnd::Head)
}

/// player.ts toggleDrag, the body just taken: which end, which hand has which limb, the move's first beat. (hands.ts haulBegin)
pub fn haul_begin(g: &mut crate::game::Game, gd: GuardId) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    g.pl[pl].haul = None;
    let Some(rc) = rag_char(g, gd) else { return };
    if g.puppet.is_some() { return; }
    let pos = g.ch[c].pos;
    if stand_room(g, pos) {
        let rag = crate::ragdoll::rag();
        let (hl, hr) = (pt(g, rc, rag.hip_l), pt(g, rc, rag.hip_r));
        let pm = mid(hl, hr);
        if !crate::scene_collision::solid_blocked(g, [pos[0], 0.5, pos[2]], [pm[0], 0.5, pm[2]], None) {
            let (sp, kl, kr) = (pt(g, rc, rag.spine), pt(g, rc, rag.knee_l), pt(g, rc, rag.knee_r));
            let dim: [f64; 4] = [v3::dist(hl, hr), v3::dist(pm, sp), v3::dist(hl, kl), v3::dist(hr, kr)];
            // he lifts it from BESIDE its hips: a grip's length off them square to its trunk, on his side
            let ax = v3::sub(sp, pm);
            let n = unit([-ax[2], 0.0, ax[0]], [1.0, 0.0, 0.0]);
            let sgn = if n[0] * (pos[0] - pm[0]) + n[2] * (pos[2] - pm[2]) < 0.0 { -1.0 } else { 1.0 };
            let grip_m = g.k.hands.grip_m;
            let mut spot: Option<V3> = Some([pm[0] + n[0] * sgn * grip_m, pos[1], pm[2] + n[2] * sgn * grip_m]);
            let s = spot.unwrap();
            if crate::scene_collision::solid_blocked(g, [pm[0], 0.5, pm[2]], [s[0], 0.5, s[2]], None) || !stand_room(g, s) { spot = None; }
            let lie = lie_of_g(g, rc);
            let yaw = g.ch[c].body_yaw;
            g.pl[pl].haul = Some(crate::hands::Haul {
                end: crate::hands::HaulEnd::Feet,
                phase: crate::hands::HaulPhase::Reach,
                t: 0.0,
                i_l: rag.hip_l,
                i_r: rag.hip_r,
                half: 0.1,
                from: [v3::copy(hl), v3::copy(hr)],
                trunk: 0.45,
                carry: Some(crate::hands::HaulCarry { dim, lie, to: None, yaw, at: None, spot }),
            });
            return;
        }
    }
    let h = drag_haul(g, gd);
    g.pl[pl].haul = Some(h);
}

/// The haul along the floor: which end, which hand has which limb. (hands.ts dragHaul)
pub fn drag_haul(g: &mut crate::game::Game, gd: GuardId) -> crate::hands::Haul {
    let c = g.pl[g.player].ch;
    let rc = rag_char(g, gd).unwrap();
    let rag = crate::ragdoll::rag();
    let dh = end_dist(g, gd, crate::hands::EndDistEnd::Head);
    let df = end_dist(g, gd, crate::hands::EndDistEnd::Feet);
    let feet = !(dh + g.k.hands.head_bias < df);
    let end = if feet { crate::hands::HaulEnd::Feet } else { crate::hands::HaulEnd::Head };
    let mut i_l = if feet { rag.ank_l } else { rag.sho_l };
    let mut i_r = if feet { rag.ank_r } else { rag.sho_r };
    let (a, b) = (pt(g, rc, i_l), pt(g, rc, i_r));
    let mp = mid(a, b);
    // the limb on his right goes to his right hand (seen from where he stands, facing the grip)
    let pos = g.ch[c].pos;
    let (fx, fz) = (mp[0] - pos[0], mp[2] - pos[2]);
    if (a[0] - mp[0]) * -fz + (a[2] - mp[2]) * fx > 0.0 { let k = i_l; i_l = i_r; i_r = k; }
    let pelvis = pt(g, rc, rag.pelvis);
    let (sl, sr) = (pt(g, rc, rag.sho_l), pt(g, rc, rag.sho_r));
    let trunk = m::max(0.45, v3::dist(pelvis, mid(sl, sr)));
    let (fl, fr) = (pt(g, rc, i_l), pt(g, rc, i_r));
    crate::hands::Haul {
        end,
        phase: crate::hands::HaulPhase::Reach,
        t: 0.0,
        i_l,
        i_r,
        half: if feet { 0.1 } else { v3::clamp(v3::dist(a, b) / 2.0, 0.1, 0.24) },
        from: [v3::copy(fl), v3::copy(fr)],
        trunk,
        carry: None,
    }
}

// ---- carried over the shoulder

/// `v` made unit, or `or` when it has no length. (hands.ts unit)
pub fn unit(v: V3, or: V3) -> V3 {
    let l = v3::len(v);
    if l > 1e-6 { v3::scale(v, 1.0 / l) } else { or }
}

/// Room for him to stand upright at p (player.ts canStand's own test, at a spot). (hands.ts standRoom)
pub fn stand_room(g: &mut crate::game::Game, p: V3) -> bool {
    let mut q: V3 = [p[0], p[1], p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.42, 1.0, 1.7, Some(2.0));
    m::hyp2(q[0] - p[0], q[2] - p[2]) < 0.01
}

/// The unit direction a turned toward b by s of the angle between them. (hands.ts turnDir)
pub fn turn_dir(a: V3, b: V3, s: f64) -> V3 {
    let ang = m::acos(v3::clamp(v3::dot(a, b), -1.0, 1.0));
    if ang < 1e-4 { return b; }
    let mut ax = v3::cross(a, b);
    if v3::len(ax) < 1e-4 { ax = v3::cross(a, crate::hands::y()); }
    if v3::len(ax) < 1e-4 { ax = v3::cross(a, [1.0, 0.0, 0.0]); }
    ax = v3::normalize(ax);
    let th = ang * s;
    let (co, si) = (m::cos(th), m::sin(th));
    let x = v3::cross(ax, a);
    let k = v3::dot(ax, a) * (1.0 - co);
    unit([a[0] * co + x[0] * si + ax[0] * k, a[1] * co + x[1] * si + ax[1] * k, a[2] * co + x[2] * si + ax[2] * k], b)
}

/// Waits on the skeleton: no `g`, and a ragdoll's points are reached through it. The body is `lie_of_g`. (hands.ts lieOf)
pub fn lie_of(rag: RagdollId) -> crate::hands::Lie { todo!("needs: g: &mut Game (hands.ts:177), ragdoll_point takes it; body is lie_of_g") }

/// How the body lies now. (hands.ts lieOf)
fn lie_of_g(g: &mut Game, r: RagdollId) -> crate::hands::Lie {
    let rag = crate::ragdoll::rag();
    let (hl, hr) = (pt(g, r, rag.hip_l), pt(g, r, rag.hip_r));
    let p = mid(hl, hr);
    let u = unit(v3::sub(hr, hl), [1.0, 0.0, 0.0]);
    let t0 = v3::sub(pt(g, r, rag.spine), p);
    let t = unit(v3::mad(t0, u, -v3::dot(t0, u)), [0.0, 0.0, 1.0]);
    let kl = unit(v3::sub(pt(g, r, rag.knee_l), hl), [0.0, -1.0, 0.0]);
    let kr = unit(v3::sub(pt(g, r, rag.knee_r), hr), [0.0, -1.0, 0.0]);
    crate::hands::Lie { p, u, t, kl, kr }
}

/// Between two lies: the hips along the straight line, every direction turned. (hands.ts lieMix)
pub fn lie_mix(a: crate::hands::Lie, b: crate::hands::Lie, s: f64) -> crate::hands::Lie {
    let u = turn_dir(a.u, b.u, s);
    let t0 = turn_dir(a.t, b.t, s);
    let t = unit(v3::mad(t0, u, -v3::dot(t0, u)), b.t);
    crate::hands::Lie { p: v3::lerp3(a.p, b.p, s), u, t, kl: turn_dir(a.kl, b.kl, s), kr: turn_dir(a.kr, b.kr, s) }
}

/// On his shoulder the body rides him: its free limbs go through `carryRide` of his own step and turn since the last frame. (hands.ts carryRides)
pub fn carry_rides(g: &mut crate::game::Game, c_2: crate::hands::CarryRidesC, rag: RagdollId, k: &mut crate::hands::HaulCarry) {
    let was = k.at;
    let ride = g.k.hands.carry_ride;
    k.at = Some([c_2.pos[0], c_2.pos[2], c_2.body_yaw]);
    let Some(was) = was else { return };
    if !(ride > 0.0) { return; }
    let (dx, dz) = (c_2.pos[0] - was[0], c_2.pos[2] - was[1]);
    let dyaw = v3::wrap_angle(c_2.body_yaw - was[2]);
    if m::hyp2(dx, dz) > 0.5 || m::abs(dyaw) > 1.0 { return; }   // put somewhere by something other than his legs
    crate::ragdoll::ragdoll_ride(g, rag, [was[0], 0.0, was[1]], dx * ride, dz * ride, dyaw * ride);
}

/// Waits on the skeleton: no `g`, and a ragdoll's pins are reached through it. The body is `lie_pins_g`. (hands.ts liePins)
pub fn lie_pins(rag: RagdollId, dim: [f64; 4], l: crate::hands::Lie) { todo!("needs: g: &mut Game (hands.ts:197), ragdoll_set_pin takes it; body is lie_pins_g") }

/// The five pins of a lie: both hips, the spine, both knees, each at the body's own length from the last. (hands.ts liePins)
fn lie_pins_g(g: &mut Game, r: RagdollId, dim: [f64; 4], l: crate::hands::Lie) {
    let rag = crate::ragdoll::rag();
    let hl = v3::mad(l.p, l.u, -dim[0] / 2.0);
    let hr = v3::mad(l.p, l.u, dim[0] / 2.0);
    pin(g, r, rag.hip_l, hl);
    pin(g, r, rag.hip_r, hr);
    pin(g, r, rag.spine, v3::mad(l.p, l.t, dim[1]));
    pin(g, r, rag.knee_l, v3::mad(hl, l.kl, dim[2]));
    pin(g, r, rag.knee_r, v3::mad(hr, l.kr, dim[3]));
}

/// Over his left shoulder, face down. (hands.ts shoulderLie)
pub fn shoulder_lie(g: &mut crate::game::Game) -> crate::hands::Lie {
    let c = g.pl[g.player].ch;
    let f = crate::character::character_forward(g, c);
    let l: V3 = [f[2], 0.0, -f[0]];
    let pos = g.ch[c].pos;
    let (side, fwd, up, legs, back) = (g.k.hands.carry_side, g.k.hands.carry_fwd, g.k.hands.carry_up, g.k.hands.carry_legs, g.k.hands.carry_back);
    let p: V3 = [pos[0] + l[0] * side + f[0] * fwd, pos[1] + up, pos[2] + l[2] * side + f[2] * fwd];
    let k = v3::normalize([f[0] * legs, -1.0, f[2] * legs]);
    crate::hands::Lie { p, u: l, t: v3::normalize([-f[0] * back, -1.0, -f[2] * back]), kl: k, kr: k }
}

/// Where it can be laid: on its back, straight, across the front of him, else round to whichever side has the floor. None = no clear floor. (hands.ts layLie)
pub fn lay_lie(g: &mut crate::game::Game) -> Option<crate::hands::LayLie> {
    let c = g.pl[g.player].ch;
    let pos = g.ch[c].pos;
    let body_yaw = g.ch[c].body_yaw;
    let lay_ahead = g.k.hands.lay_ahead;
    let at: V3 = [pos[0], pos[1] + 0.3, pos[2]];
    for turn in [0.0, 0.785, -0.785, 1.571, -1.571, 2.356, -2.356, 3.1416] {
        let yaw = body_yaw + turn;
        let d: V3 = [m::sin(yaw), 0.0, m::cos(yaw)];
        let p0 = v3::mad(pos, d, lay_ahead);
        let mp: V3 = [p0[0], pos[1] + 0.3, p0[2]];
        if crate::scene_collision::solid_blocked(g, at, mp, None) { continue; }
        for side in [1.0, -1.0] {
            let t: V3 = [-d[2] * side, 0.0, d[0] * side];
            if crate::scene_collision::solid_blocked(g, mp, v3::mad(mp, t, 0.95), None) || crate::scene_collision::solid_blocked(g, mp, v3::mad(mp, t, -0.95), None) { continue; }
            let k = v3::normalize([-t[0], -0.06, -t[2]]);
            return Some(crate::hands::LayLie {
                lie: crate::hands::Lie { p: [p0[0], pos[1] + 0.11, p0[2]], u: v3::cross(crate::hands::y(), t), t: v3::normalize([t[0], 0.17, t[2]]), kl: k, kr: k },
                yaw,
            });
        }
    }
    None
}

/// Is he walking with it on his shoulder? (hands.ts haulUpright)
pub fn haul_upright(g: &mut crate::game::Game) -> bool {
    match g.pl[g.player].haul {
        Some(h) => h.carry.is_some() && h.phase == crate::hands::HaulPhase::Carry,
        None => false,
    }
}

/// Is he down on his heels for this beat? (hands.ts haulCrouch)
pub fn haul_crouch(g: &mut crate::game::Game) -> bool {
    use crate::hands::HaulPhase as P;
    let Some(h) = g.pl[g.player].haul else { return true };
    h.carry.is_none()
        || h.phase == P::Reach
        || (h.phase == P::Set && h.t > g.k.hands.lower_secs * 0.2)
        || (h.phase == P::Rise && h.t < g.k.hands.hoist_secs * 0.45)
}

/// The facing he keeps through the lift and the laying-down of a carried body. (hands.ts haulYaw)
pub fn haul_yaw(g: &mut crate::game::Game) -> Option<f64> {
    use crate::hands::HaulPhase as P;
    let h = g.pl[g.player].haul?;
    let k = h.carry?;
    if h.phase == P::Rise || h.phase == P::Set { Some(k.yaw) } else { None }
}

/// The two spots in front of him the held limbs are carried at (left hand's, right hand's). (hands.ts carrySpots)
pub fn carry_spots(g: &mut crate::game::Game, h: crate::hands::Haul) -> [V3; 2] {
    let c = g.pl[g.player].ch;
    let f = crate::character::character_forward(g, c);
    let feet = h.end == crate::hands::HaulEnd::Feet;
    let y = if feet { g.k.hands.carry_feet } else { g.k.hands.carry_head };
    let ahead = if feet { g.k.hands.ahead } else { g.k.hands.ahead_head };
    let pos = g.ch[c].pos;
    let (hx, hz) = (pos[0] + f[0] * ahead, pos[2] + f[2] * ahead);
    [[hx + f[2] * h.half, y, hz - f[0] * h.half], [hx - f[2] * h.half, y, hz + f[0] * h.half]]
}

/// Each frame a body is in tow: runs the move's beat, places the pins, feeds the animator's hands. False = no move of this file's is on. (hands.ts haulPins)
pub fn haul_pins(g: &mut crate::game::Game, dt: f64) -> bool {
    use crate::hands::HaulPhase as P;
    let pl = g.player;
    let c = g.pl[pl].ch;
    let (mut h, body, rc) = match (g.pl[pl].haul, g.pl[pl].dragging) {
        (Some(h), Some(body)) => match rag_char(g, body) {
            Some(rc) => (h, body, rc),
            None => { g.ch[c].anim.haul = None; return false; }
        },
        _ => { g.ch[c].anim.haul = None; return false; }
    };
    // `h` is the player's own in the TypeScript: this copy goes back to g.pl[pl].haul before anything reads it
    h.t += dt;
    let (p_l, p_r) = (pt(g, rc, h.i_l), pt(g, rc, h.i_r));
    if h.carry.is_some() && h.phase == P::Reach {
        let pos = g.ch[c].pos;
        let (grip_m, reach_secs, step_secs, near_m) = (g.k.hands.grip_m, g.k.hands.reach_secs, g.k.hands.carry_step_secs, g.k.hands.carry_near_m);
        let d_hip = v3::dist_xz(mid(p_l, p_r), pos);
        let there = match h.carry.unwrap().spot {
            Some(sp) => v3::dist_xz(sp, pos) < 0.1,
            None => d_hip <= grip_m + 0.08,
        };
        if (there && h.t >= reach_secs) || h.t >= step_secs {
            if d_hip <= grip_m + near_m && stand_room(g, pos) {
                h.phase = P::Rise;
                h.t = 0.0;
                let lie = lie_of_g(g, rc);
                let k = h.carry.as_mut().unwrap();
                k.lie = lie;
                k.yaw = v3::yaw_to(pos, mid(p_l, p_r));
                wake(g, rc);
            } else {
                // no room to stand where the step in left him, or he walked off from it: by an end along the floor
                let nh = drag_haul(g, body);
                g.pl[pl].haul = Some(nh);
                return haul_pins(g, 0.0);
            }
        }
    }
    if let Some(mut k) = h.carry {
        let (hoist, lower) = (g.k.hands.hoist_secs, g.k.hands.lower_secs);
        if h.phase == P::Rise {
            let sl = shoulder_lie(g);
            lie_pins_g(g, rc, k.dim, lie_mix(k.lie, sl, v3::smoothstep(0.0, 1.0, h.t / hoist)));
            if h.t >= hoist { h.phase = P::Carry; h.t = 0.0; }
        } else if h.phase == P::Carry {
            let (pos, body_yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
            carry_rides(g, crate::hands::CarryRidesC { pos, body_yaw }, rc, &mut k);
            let sl = shoulder_lie(g);
            lie_pins_g(g, rc, k.dim, sl);
        } else if h.phase == P::Set {
            if let Some(to) = k.to {
                lie_pins_g(g, rc, k.dim, lie_mix(k.lie, to, v3::smoothstep(0.0, 1.0, h.t / lower)));
            }
        }
        h.carry = Some(k);
        g.pl[pl].haul = Some(h);
        feed(g, h);
        return true;
    }
    let carry = carry_spots(g, h);
    if h.phase == P::Reach {
        let pos = g.ch[c].pos;
        let near = v3::dist_xz(mid(p_l, p_r), pos) <= g.k.hands.grip_m + 0.08;
        if (near && h.t >= g.k.hands.reach_secs) || h.t >= g.k.hands.step_secs {
            h.phase = P::Rise;
            h.t = 0.0;
            h.from = [v3::copy(p_l), v3::copy(p_r)];
            wake(g, rc);
        }
    }
    if h.phase == P::Rise {
        let lift = g.k.hands.lift_secs;
        let s = v3::smoothstep(0.0, 1.0, h.t / lift);
        pin(g, rc, h.i_l, v3::lerp3(h.from[0], carry[0], s));
        pin(g, rc, h.i_r, v3::lerp3(h.from[1], carry[1], s));
        if h.t >= lift { h.phase = P::Carry; h.t = 0.0; }
    } else if h.phase == P::Carry {
        pin(g, rc, h.i_l, carry[0]);
        pin(g, rc, h.i_r, carry[1]);
    } else if h.phase == P::Set {
        let s = v3::smoothstep(0.0, 1.0, h.t / g.k.hands.set_secs);
        let y = rad(g, rc, h.i_l) + 0.02;
        pin(g, rc, h.i_l, [h.from[0][0], v3::lerp(h.from[0][1], y, s), h.from[0][2]]);
        pin(g, rc, h.i_r, [h.from[1][0], v3::lerp(h.from[1][1], y, s), h.from[1][2]]);
    }
    g.pl[pl].haul = Some(h);
    feed(g, h);
    true
}

/// A held limb's wrist spot: a palm's depth above its middle. (hands.ts feed's `up`)
fn up(g: &mut Game, rc: RagdollId, i: f64) -> V3 {
    let p = pt(g, rc, i);
    [p[0], p[1] + rad(g, rc, i) * 0.6 + 0.02, p[2]]
}

/// The animator's two wrist spots: on the limbs themselves; open hands on the way down, closed from the lift on. (hands.ts feed)
pub fn feed(g: &mut crate::game::Game, h: crate::hands::Haul) {
    use crate::hands::HaulPhase as P;
    let pl = g.player;
    let c = g.pl[pl].ch;
    let rc = rag_char(g, g.pl[pl].dragging.unwrap()).unwrap();
    if h.carry.is_some() {
        // his left hand on its thighs, his right on its back until it is nearly up
        let rag = crate::ragdoll::rag();
        let f = crate::character::character_forward(g, c);
        let lf: V3 = [f[2], 0.0, -f[0]];
        let (kl, kr, hl, hr) = (pt(g, rc, rag.knee_l), pt(g, rc, rag.knee_r), pt(g, rc, rag.hip_l), pt(g, rc, rag.hip_r));
        let (km, hm) = (mid(kl, kr), mid(hl, hr));
        let on = if h.phase == P::Carry {
            1.0
        } else if h.phase == P::Rise {
            v3::smoothstep(0.35, 0.9, h.t / g.k.hands.hoist_secs)
        } else if h.phase == P::Set {
            1.0 - v3::smoothstep(0.1, 0.6, h.t / g.k.hands.lower_secs)
        } else {
            0.0
        };
        let th = v3::lerp3(hm, km, 0.75);
        let l: V3 = [th[0] + lf[0] * 0.06 * on, th[1] + 0.085, th[2] + lf[2] * 0.06 * on];
        let r = up(g, rc, rag.spine);
        let y = l[1] - g.ch[c].pos[1];
        g.ch[c].anim.haul = Some(crate::character::HaulPose {
            l,
            r,
            rw: Some(1.0 - on),
            look: Some(0.7 * (1.0 - on)),
            tilt: Some(0.1 * on),
            bow: v3::lerp(0.12 + 0.5 * (1.0 - v3::smoothstep(0.2, 0.7, y)), 0.07, on),
            grip: if h.phase == P::Reach { 0.25 } else { 0.8 },
        });
        return;
    }
    let (l, r) = (up(g, rc, h.i_l), up(g, rc, h.i_r));
    let y = (l[1] + r[1]) / 2.0 - g.ch[c].pos[1];
    let hauling = if h.phase == P::Carry { v3::smoothstep(0.0, 0.4, h.t) } else { 0.0 };
    let grip = if h.phase == P::Reach {
        0.25
    } else if h.phase == P::Set {
        v3::lerp(0.85, 0.3, v3::clamp(h.t / g.k.hands.set_secs, 0.0, 1.0))
    } else {
        0.85
    };
    g.ch[c].anim.haul = Some(crate::character::HaulPose {
        l,
        r,
        rw: None,
        look: Some(v3::lerp(0.7, 0.15, hauling)),
        tilt: None,
        bow: (0.12 + 0.5 * (1.0 - v3::smoothstep(0.2, 0.6, y))) * v3::lerp(1.0, 0.6, hauling),
        grip,
    });
}

/// The body in his hands by the head end, when it is wedged in a way the leg sticks do not show. (hands.ts haulSlack)
pub fn haul_slack(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    let Some(h) = g.pl[pl].haul else { return false };
    let Some(rc) = g.pl[pl].dragging.and_then(|b| rag_char(g, b)) else { return false };
    if h.end != crate::hands::HaulEnd::Head || h.phase != crate::hands::HaulPhase::Carry { return false; }
    let carry = carry_spots(g, h);
    let (s_l, s_r) = (pt(g, rc, h.i_l), pt(g, rc, h.i_r));
    if m::max(v3::dist(s_l, carry[0]), v3::dist(s_r, carry[1])) > g.k.hands.slack_m { return true; }
    let pelvis = pt(g, rc, crate::ragdoll::rag().pelvis);
    v3::dist(pelvis, mid(s_l, s_r)) > h.trunk * 1.3 + 0.1
}

/// The 'let go of him' row: lay it down instead of dropping it. False = nothing to lay down this way. (hands.ts haulSetDown)
pub fn haul_set_down(g: &mut crate::game::Game) -> bool {
    use crate::hands::HaulPhase as P;
    let pl = g.player;
    let Some(mut h) = g.pl[pl].haul else { return false };
    let Some(rc) = g.pl[pl].dragging.and_then(|b| rag_char(g, b)) else { return false };
    if h.phase == P::Set { return true; }
    if h.phase == P::Reach { return false; }   // not yet in his hands: he simply leaves it
    if h.carry.is_some() {
        let Some(to) = lay_lie(g) else { return false };
        let lie = lie_of_g(g, rc);
        let k = h.carry.as_mut().unwrap();
        k.lie = lie;
        k.to = Some(to.lie);
        k.yaw = to.yaw;
        h.phase = P::Set;
        h.t = 0.0;
        g.pl[pl].haul = Some(h);
        return true;
    }
    h.phase = P::Set;
    h.t = 0.0;
    let (a, b) = (pt(g, rc, h.i_l), pt(g, rc, h.i_r));
    h.from = [v3::copy(a), v3::copy(b)];
    g.pl[pl].haul = Some(h);
    true
}

/// Has the laying-down run its course? (hands.ts haulSetDone)
pub fn haul_set_done(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    let Some(h) = g.pl[pl].haul else { return false };
    if h.phase != crate::hands::HaulPhase::Set { return false; }
    let Some(k) = h.carry else { return h.t >= g.k.hands.set_secs };
    let Some(rc) = g.pl[pl].dragging.and_then(|b| rag_char(g, b)) else { return true };
    let Some(to) = k.to else { return true };
    let rag = crate::ragdoll::rag();
    let (a, b) = (pt(g, rc, rag.hip_l), pt(g, rc, rag.hip_r));
    let at = mid(a, b);
    let lower = g.k.hands.lower_secs;
    h.t >= lower && (h.t >= lower + 0.5 || (m::hyp2(at[0] - to.p[0], at[2] - to.p[2]) < 0.06 && at[1] < to.p[1] + 0.12))
}

/// How fast his feet may go with a body, as a share of the crouch. (hands.ts haulPace)
pub fn haul_pace(g: &mut crate::game::Game) -> f64 {
    use crate::hands::HaulPhase as P;
    match g.pl[g.player].haul {
        None => 0.6,
        Some(h) if h.phase == P::Carry => 0.6,
        Some(h) if h.phase == P::Reach => 1.0,
        Some(_) => 0.0,
    }
}

/// The same in m/s; over his shoulder he walks, at carryPace × his walk. (hands.ts haulSpeed)
pub fn haul_speed(g: &mut crate::game::Game) -> f64 {
    if haul_upright(g) { g.tune.player_walk * g.k.hands.carry_pace } else { let k = g.tune.player_crouch; k * haul_pace(g) }
}

// ------------------------------------------------------------------------------------------------ a door

/// The unit line of the leaf were it at `angle`. (hands.ts leafU)
pub fn leaf_u(g: &mut crate::game::Game, d: DoorId, angle: f64) -> [f64; 2] {
    let a = g.dr[d].def.closed_dir + angle;
    [m::cos(a), m::sin(a)]
}

/// Floor distance from q to the leaf's centre line were it at `angle`. (hands.ts leafDist)
pub fn leaf_dist(g: &mut crate::game::Game, d: DoorId, angle: f64, q: V3) -> f64 {
    let u = leaf_u(g, d, angle);
    let h = hinge(g, d);
    let (vx, vz) = (q[0] - h[0], q[2] - h[1]);
    let t = v3::clamp(vx * u[0] + vz * u[1], 0.0, g.dr[d].def.width);
    m::hyp2(vx - u[0] * t, vz - u[1] * t)
}

/// Which side of the leaf's line at `angle` q is on, and how far off it. (hands.ts leafPerp)
pub fn leaf_perp(g: &mut crate::game::Game, d: DoorId, angle: f64, q: V3) -> f64 {
    let u = leaf_u(g, d, angle);
    let h = hinge(g, d);
    u[0] * (q[2] - h[1]) - u[1] * (q[0] - h[0])
}

/// How far his centre stays off the leaf's line. (hands.ts bodyR)
pub fn body_r(g: &mut crate::game::Game, d: DoorId) -> f64 { 0.34 + g.dr[d].thick / 2.0 + g.k.hands.clear_m }

/// The nearest the leaf's line comes to q anywhere on its way from a0 to a1. (hands.ts sweepDist)
pub fn sweep_dist(g: &mut crate::game::Game, d: DoorId, a0: f64, a1: f64, q: V3) -> f64 {
    let n = m::max(1.0, m::ceil(m::abs(a1 - a0) / 0.1));
    let mut best = m::INF;
    let mut k = 0.0;
    while k <= n {
        best = m::min(best, leaf_dist(g, d, a0 + (a1 - a0) * k / n, q));
        k += 1.0;
    }
    best
}

/// Does the floor segment p → q cross the leaf's line as it stands at `angle`? (hands.ts crossesLeaf)
pub fn crosses_leaf(g: &mut crate::game::Game, d: DoorId, angle: f64, p: V3, q: V3) -> bool {
    let (s0, s1) = (leaf_perp(g, d, angle, p), leaf_perp(g, d, angle, q));
    if s0 * s1 > 0.0 { return false; }
    let t = s0 / js::or(s0 - s1, 1e-9);
    let (x, z) = (p[0] + (q[0] - p[0]) * t, p[2] + (q[2] - p[2]) * t);
    let u = leaf_u(g, d, angle);
    let h = hinge(g, d);
    let along = (x - h[0]) * u[0] + (z - h[1]) * u[1];
    along > -0.1 && along < g.dr[d].def.width + 0.1
}

/// The side of the line the leaf will STAND on at a1 that q is on: > 0 = the opening's, < 0 = the pocket behind the leaf. (hands.ts openSide)
pub fn open_side(g: &mut crate::game::Game, d: DoorId, a1: f64, q: V3) -> f64 {
    let u = leaf_u(g, d, 0.0);
    let h = hinge(g, d);
    let w = g.dr[d].def.width;
    let l: V3 = [h[0] + u[0] * w, 0.0, h[1] + u[1] * w];
    let of_q = leaf_perp(g, d, a1, q);
    of_q * m::sign(js::or(leaf_perp(g, d, a1, l), 1.0))
}

/// Waits on the skeleton: the TypeScript's three answers (a spot, null, undefined) do not fit `Option<V3>`. The body is `clear_spot_3`. (hands.ts clearSpot)
pub fn clear_spot(g: &mut crate::game::Game, d: DoorId, a0: f64, a1: f64) -> Option<V3> { todo!("needs: -> Option<Option<V3>> (hands.ts:367; both readers tell undefined from null); body is clear_spot_3") }

/// Where to stand for the leaf to go from a0 to a1 without meeting him: Some(None) = where he is (`null`); Some(Some(spot)) = the nearest standing room the sweep
/// misses, on his side of the wall; None = there is none within a step (`undefined`). (hands.ts clearSpot)
fn clear_spot_3(g: &mut Game, d: DoorId, a0: f64, a1: f64) -> Option<Option<V3>> {
    let p = g.ch[g.pl[g.player].ch].pos;
    let need = body_r(g, d);
    let clear = sweep_dist(g, d, a0, a1, p) >= need;
    let wall = leaf_perp(g, d, 0.0, p);   // his side of the wall the door is in; in the threshold itself either side will do
    let toward = m::abs(a1) > m::abs(a0) + 0.3 && m::abs(wall) > 0.05 && wall * a1 > 0.0;
    if clear && !(toward && open_side(g, d, a1, p) < 0.05) { return Some(None); }
    if toward {
        let reach = g.k.hands.round_m;
        if let Some(q) = find_spot(g, d, a0, a1, p, need, wall, reach, true) { return Some(Some(q)); }
        if clear { return Some(None); }
    }
    let reach = g.k.hands.step_m;
    find_spot(g, d, a0, a1, p, need, wall, reach, false).map(Some)
}

/// The rings round him, nearest first; of a ring's free spots, the one nearest the handle. (hands.ts clearSpot's `find`)
fn find_spot(g: &mut Game, d: DoorId, a0: f64, a1: f64, p: V3, need: f64, wall: f64, reach: f64, front: bool) -> Option<V3> {
    let mut best: Option<V3> = None;
    let mut bs = m::INF;
    let mut r = 0.15;
    while r <= reach + 1e-6 && best.is_none() {
        for k in 0..16 {
            let a = k as f64 * m::PI / 8.0;
            let q: V3 = [p[0] + m::sin(a) * r, p[1], p[2] + m::cos(a) * r];
            if sweep_dist(g, d, a0, a1, q) < need + 0.02 { continue; }
            if front && open_side(g, d, a1, q) < need { continue; }   // beyond the tip but ON the line he would clip it walking in
            let side = leaf_perp(g, d, 0.0, q);
            if m::abs(wall) > 0.2 && side * wall <= 0.0 { continue; }
            if crosses_leaf(g, d, a0, p, q) { continue; }
            if crate::scene_collision::solid_blocked(g, [p[0], 0.5, p[2]], [q[0], 0.5, q[2]], None) { continue; }
            let mut t: V3 = [q[0], q[1], q[2]];
            if crate::scene_collision::collide_circle(g, &mut t, 0.32, 0.2, 1.5, None) && v3::dist_xz(t, q) > 0.02 { continue; }
            let tip = leaf_u(g, d, a0);
            let h = hinge(g, d);
            let w = g.dr[d].def.width;
            let (hx, hz) = (h[0] + tip[0] * w * 0.85, h[1] + tip[1] * w * 0.85);
            let score = m::hyp2(q[0] - hx, q[2] - hz);   // the hand stays on it longest
            if score < bs { bs = score; best = Some(q); }
        }
        r += 0.1;
    }
    best
}

/// A limp body lying where the leaf would sweep on its way from a0 to a1. (hands.ts limpInSweep)
pub fn limp_in_sweep(g: &mut crate::game::Game, d: DoorId, a0: f64, a1: f64) -> bool {
    let h = hinge(g, d);
    let reach = g.dr[d].def.width + 0.3;
    let (thick, height) = (g.dr[d].thick, g.dr[d].height);
    let mut q: V3 = [0.0, 0.0, 0.0];
    for list in 0..2 {
        let mut n = 0;
        loop {
            let of = if list == 0 { &g.guards } else { &g.civilians };
            let Some(&gd) = of.get(n) else { break };
            n += 1;
            let c = g.gd[gd].ch;
            if let Some(wound) = g.gd[gd].wound {
                let at = if wound.pose == crate::wounds::WoundPose::Back {
                    *g.ch[c].bones.get(crate::character::Bone::Hips).unwrap()
                } else {
                    g.ch[c].pos
                };
                if sweep_dist(g, d, a0, a1, at) < 0.3 + thick / 2.0 + 0.02 { return true; }
            }
            let Some(rag) = g.ch[c].ragdoll else { continue };
            let mut i = 0.0;
            while i < crate::ragdoll::RAG_COUNT {
                let this = i;
                i += 1.0;
                crate::ragdoll::ragdoll_point(g, rag, this, Some(&mut q));
                let r = rad(g, rag, this);
                if q[1] - r > height || m::hyp2(q[0] - h[0], q[2] - h[1]) > reach { continue; }
                if sweep_dist(g, d, a0, a1, q) < r + thick / 2.0 + 0.02 { return true; }
            }
        }
    }
    false
}

/// A 'push' / 'pull' / 'unlock' row performed on d: start working it by hand. False = not this way, the caller does what it always did. (hands.ts doorMoveBegin)
pub fn door_move_begin(g: &mut crate::game::Game, d: DoorId) -> bool {
    use crate::doors::{door_angle, door_is_closed, door_max_a, door_min_a, door_off, door_swing};
    let pl = g.player;
    let p = g.ch[g.pl[pl].ch].pos;
    if g.puppet.is_some() || door_off(g, d) || hands_busy(g) { return false; }
    let keys = g.pl[pl].keys.clone();
    if crate::doors::doors_locked_out(g, d, p, Some(&keys)) { return false; }
    let open = door_is_closed(g, d);
    if let Some(m0) = g.pl[pl].door_move {
        if m0.d == d && m0.open == open { return true; }   // the same row tapped again while it is being done: one move, one creak
    }
    let mut target = 0.0;
    if open {
        let sgn = door_swing(g, d);
        let stop = if sgn < 0.0 { -door_min_a(g, d) } else { door_max_a(g, d) };
        target = sgn * m::min(g.k.hands.open_to, stop - 0.02);
        if m::abs(target) < 0.3 { return false; }
    }
    let angle = door_angle(g, d);
    if clear_spot_3(g, d, angle, target).is_none() { return false; }
    // a step in to it as the hand goes out, when the leaf is to be pushed open AWAY from him from further off than a hand reaches and the step is a short one
    let mut goal: Option<V3> = None;
    if open && leaf_perp(g, d, angle, p) * target < 0.0 {
        let u = leaf_u(g, d, angle);
        let w = g.dr[d].def.width;
        let h = hinge(g, d);
        let (vx, vz) = (p[0] - h[0], p[2] - h[1]);
        let s = v3::clamp(vx * u[0] + vz * u[1], w * 0.45, w * 0.85);
        let (fx, fz) = (h[0] + u[0] * s, h[1] + u[1] * s);
        let off = m::hyp2(p[0] - fx, p[2] - fz);
        let (hand_m, in_m) = (g.k.hands.hand_m, g.k.hands.in_m);
        if off > hand_m + 0.12 && off - hand_m <= in_m {
            let k = hand_m / off;
            let q: V3 = [fx + (p[0] - fx) * k, p[1], fz + (p[2] - fz) * k];
            if !crate::scene_collision::solid_blocked(g, [p[0], 0.5, p[2]], [q[0], 0.5, q[2]], None) { goal = Some(q); }
        }
    }
    g.pl[pl].easing = None;
    g.pl[pl].door_move = Some(crate::hands::DoorMove {
        d,
        open,
        target,
        t: 0.0,
        stage: crate::hands::DoorMoveStage::Hand,
        goal,
        stall_t: 0.0,
        best: m::abs(target - angle),
        step_best: m::INF,
        step_stall: 0.0,
        round: None,
    });
    true
}

/// Each frame before the leaves are stepped: the leaf of the move on round under his hand. (hands.ts doorMoveTick)
pub fn door_move_tick(g: &mut crate::game::Game, dt: f64) {
    use crate::doors::*;
    let pl = g.player;
    let Some(mut mv) = g.pl[pl].door_move else { return };
    let c = g.pl[pl].ch;
    let d = mv.d;
    let p = g.ch[c].pos;
    let who = g.ch[c].id;
    mv.t += dt;
    g.pl[pl].door_move = Some(mv);
    let pos = door_pos(g, d);
    let far = m::min(v3::dist_xz(pos, p), v3::dist_xz(g.dr[d].frame_centre, p)) > g.dr[d].def.width + 1.4;
    if door_off(g, d) || far || hands_busy(g) { g.pl[pl].door_move = None; return; }
    if mv.stage == crate::hands::DoorMoveStage::Hand {
        // the hand first: on the leaf, or as long as a hand takes when the leaf is out of its reach; then it is unlatched, and his feet set off for standing room
        let hw = crate::character::anim_door_hands(g, c);
        if mv.t < g.k.hands.door_reach || (m::max(hw.l, hw.r) < 0.6 && mv.t < g.k.hands.hand_secs) { return; }
        let angle = door_angle(g, d);
        let Some(goal) = clear_spot_3(g, d, angle, mv.target) else {
            // no room after all: as it always was, unless the leaf has meanwhile got where the row wanted it
            g.pl[pl].door_move = None;
            if mv.open == door_is_closed(g, d) {
                let keys = g.pl[pl].keys.clone();
                doors_use(g, d, p, who, Some(&keys));
            }
            return;
        };
        if mv.open {
            let keys = g.pl[pl].keys.clone();
            doors_use(g, d, p, who, Some(&keys));
            if door_latched(g, d) { g.pl[pl].door_move = None; return; }
        }
        mv.stage = crate::hands::DoorMoveStage::Swing;
        mv.t = 0.0;
        if let Some(goal) = goal {
            mv.goal = Some(goal);
            mv.step_best = m::INF;
            mv.step_stall = 0.0;
            mv.round = Some(mv.open && open_side(g, d, mv.target, p) < 0.05 && open_side(g, d, mv.target, goal) > 0.0);
        }
    }
    let angle = door_angle(g, d);
    let rem = mv.target - angle;
    let left = m::abs(rem);
    let there = if mv.open { left < 0.02 } else { m::abs(angle) < 0.14 };
    if there {   // open: left standing; shut: the sprung last bit and the latch are the door's own
        door_set_vel(g, d, 0.0);
        if !mv.open { doors_pull_to(g, d, who); }
        g.pl[pl].door_move = None;
        return;
    }
    if left < mv.best - 1e-4 || mv.goal.is_some() {   // while his feet are still on their way clear, the leaf is only waiting for them: no stall is counted
        mv.best = m::min(mv.best, left);
        mv.stall_t = 0.0;
    } else {
        mv.stall_t += dt;
        if mv.stall_t > g.k.hands.stall_secs { door_set_vel(g, d, 0.0); g.pl[pl].door_move = None; return; }
    }
    let rate = g.k.hands.door_rate * v3::clamp(left / 0.45, 0.3, 1.0) * v3::smoothstep(-0.05, 0.12, mv.t);
    let next = angle + m::sign(rem) * m::min(left, rate * m::min(dt, 0.05));
    let now = leaf_dist(g, d, angle, p);
    let then = leaf_dist(g, d, next, p);
    door_set_latched(g, d, false);
    door_set_closing(g, d, false);
    door_set_vel(g, d, 0.0);
    door_set_last_who(g, d, who);
    if mv.open { let q = door_quiet_t(g, d); door_set_quiet_t(g, d, m::min(q, -1.0)); }
    let need = body_r(g, d) - 0.02;
    let free = then >= need;
    // coming out round its free end from behind where it will stand: the leaf keeps clear of the whole of the way he still has to go
    let mut held = false;
    if let (Some(goal), Some(true)) = (mv.goal, mv.round) {
        let mut i = 1.0;
        while i <= 6.0 && !held {
            let k = i / 6.0;
            held = leaf_dist(g, d, next, [p[0] + (goal[0] - p[0]) * k, p[1], p[2] + (goal[2] - p[2]) * k]) < need;
            i += 1.0;
        }
    }
    if !held && (free || then >= now) {   // never into him: it waits for his step (and one already against him only ever goes away)
        let (lo, hi) = (door_min_a(g, d), door_max_a(g, d));
        door_set_angle(g, d, v3::clamp(next, lo, hi));
    }
    // a leaf he is bringing round TOWARD himself is a pull: it comes past whoever stands with him, only while clear of HIM
    if mv.open && free && doors_pull_face(g, d, p) {
        let t = door_pull_t(g, d);
        door_set_pull_t(g, d, m::max(t, 0.05));
    }
    g.pl[pl].door_move = Some(mv);
}

/// What the hands' moves do to his feet this frame: None = nothing, [0, 0] = planted, else the unit direction of the step being taken for him. (hands.ts handsStep)
pub fn hands_step(g: &mut crate::game::Game, own: bool, dt: f64) -> Option<[f64; 2]> {
    use crate::hands::HaulPhase as P;
    let pl = g.player;
    let c = g.pl[pl].ch;
    if let (Some(h), Some(body)) = (g.pl[pl].haul, g.pl[pl].dragging) {
        if h.phase == P::Rise || h.phase == P::Set { return Some([0.0, 0.0]); }
        if h.phase == P::Reach {
            let Some(rc) = rag_char(g, body) else { return Some([0.0, 0.0]) };
            if crate::game::player_list_open(g, pl) { return Some([0.0, 0.0]); }
            if own && h.carry.is_none() {   // a step of his own: no walking him in
                let t = m::max(h.t, g.k.hands.step_secs);
                g.pl[pl].haul.as_mut().unwrap().t = t;
                return None;
            }
            let pos = g.ch[c].pos;
            if let Some(sp) = h.carry.and_then(|k| k.spot) {
                let (dx, dz) = (sp[0] - pos[0], sp[2] - pos[2]);
                let dl = m::hyp2(dx, dz);
                return if dl > 0.06 { Some([dx / dl, dz / dl]) } else { Some([0.0, 0.0]) };
            }
            let (a, b) = (pt(g, rc, h.i_l), pt(g, rc, h.i_r));
            let mp = mid(a, b);
            let (dx, dz) = (mp[0] - pos[0], mp[2] - pos[2]);
            let dl = m::hyp2(dx, dz);
            return if dl > g.k.hands.grip_m { Some([dx / dl, dz / dl]) } else { Some([0.0, 0.0]) };
        }
        return None;
    }
    let Some(mv) = g.pl[pl].door_move else { return None };
    let Some(goal) = mv.goal else { return None };
    if own {   // a step of his own: his feet are his again (the leaf still will not go into him)
        g.pl[pl].door_move.as_mut().unwrap().goal = None;
        return None;
    }
    if crate::game::player_list_open(g, pl) { return Some([0.0, 0.0]); }   // choosing a row: feet still, the leaf waits
    let pos = g.ch[c].pos;
    let (dx, dz) = (goal[0] - pos[0], goal[2] - pos[2]);
    let dl = m::hyp2(dx, dz);
    let w = g.pl[pl].door_move.as_mut().unwrap();
    if dl < w.step_best - 0.002 { w.step_best = dl; w.step_stall = 0.0; } else { w.step_stall += dt; }
    if dl < 0.06 || w.step_stall > 0.25 { w.goal = None; return None; }
    Some([dx / dl, dz / dl])
}

/// Does the door he is working bring his gun down off the cursor? (hands.ts doorMoveLowers)
pub fn door_move_lowers(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let Some(mv) = g.pl[pl].door_move else { return false };
    if g.time - g.pl[pl].last_fire_t <= 0.6 { return false; }
    let u = crate::doors::door_dir(g, mv.d);
    let w = g.dr[mv.d].def.width * 0.6;
    let h = hinge(g, mv.d);
    let pos = g.ch[c].pos;
    let (x, z) = (h[0] + u[0] * w - pos[0], h[1] + u[1] * w - pos[2]);
    let mut off = m::atan2(x, z) - g.ch[c].aim_yaw;
    off = m::atan2(m::sin(off), m::cos(off));
    m::abs(off) > g.k.hands.aim_arc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hands::Lie;

    /// The TypeScript's own bits: hands.ts turnDir and lieMix run under bun with EXACT.on.
    const BITS: [[u64; 3]; 17] = [
[0x3ff4b851eb851eb8, 0x3fd051eb851eb852, 0xc00bae147ae147ae],
[0x3fc8d139bf8a9338, 0x3fb98466e614d492, 0x3fef3af5dfa34f53],
[0xbfef631ace3483e6, 0x3fa3bf03ee22eca2, 0x3fc8700e24e70efe],
[0x3fc09aa031b61ecf, 0xbfeacbe04f580fcc, 0x3fe0fde927e89d50],
[0xbfbc640a5b864c79, 0xbfedd4883f283dfa, 0x3fd60d1fee6ebf74],
[0x3ff6a9fbe76c8b44, 0x3fe4b020c49ba5e3, 0xc00ad0e560418937],
[0xbfc219222a53bf0a, 0x3fb40b152f857b33, 0x3fef944c32c36912],
[0xbfea6473d43046fb, 0xbfe1f0e906a02cfb, 0xbfb2dd2cd61926ed],
[0x3fd063788ed3fcce, 0xbfec01c7bb4472c4, 0x3fda43dfd9a511f9],
[0x3fb5ae03b68cbbff, 0xbfee9bca3ef2afb0, 0x3fd1dcf31c9a26f9],
[0x3ff9d9999999999a, 0x3ff49c28f5c28f5c, 0xc009666666666666],
[0xbfe476e27d6c7d6a, 0x3f9a479ef161c768, 0x3fe89657beedbe2c],
[0x3fd9c2740c6877f0, 0xbfeae41a2841c65a, 0x3fd73c8b9440b092],
[0x3fdc65bb262021e2, 0xbfec0cf6f7b32a7d, 0x3fc7da35b23bfc99],
[0x3fd92f330c1ba0bf, 0xbfed09b7b9297643, 0x3fc2dc6ef46dfaca],
[0x3fe6a09e667f3bcc, 0x3fe6a09e667f3bcd, 0x0000000000000000],
[0x3c91a62633145c00, 0x3ff0000000000000, 0x0000000000000000]
    ];

    fn eq(i: usize, v: V3) {
        for k in 0..3 {
            assert_eq!(v[k].to_bits(), BITS[i][k], "row {i} axis {k}: {}", v[k]);
        }
    }

    #[test]
    fn lie_mix_and_turn_dir_have_the_typescripts_bits() {
        let n = v3::normalize;
        let a = Lie { p: [1.25, 0.11, -3.5], u: n([0.3, 0.1, 0.9]), t: n([-0.9, 0.2, 0.3]), kl: n([0.1, -1.0, 0.7]), kr: n([-0.2, -1.0, 0.4]) };
        let b = Lie { p: [1.7, 1.56, -3.1], u: n([-0.8, 0.0, 0.6]), t: n([0.5, -1.0, 0.7]), kl: n([0.6, -1.0, 0.1]), kr: n([0.6, -1.0, 0.1]) };
        let mut i = 0;
        for s in [0.1, 0.37, 0.8125] {
            let l = lie_mix(a, b, s);
            for v in [l.p, l.u, l.t, l.kl, l.kr] { eq(i, v); i += 1; }
        }
        eq(15, turn_dir([0.0, 1.0, 0.0], [0.0, -1.0, 0.0], 0.25));   // dead opposite: round by way of the fallback axis
        eq(16, turn_dir([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.5));
    }
}
