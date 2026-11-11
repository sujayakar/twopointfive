//! Things fetched, put away and swapped.
use super::*;

/// The gun hand's errand between the flashlight's ring at the front of the hip and the sidearm on the thigh: the wrist from wherever the pose has it
/// down onto the one, across to the other, and back. The pose as it stood is kept at `at_sw` (what was in the hand is drawn off it).
pub fn swap_pose(g: usize, an: &mut Anim, at_sw: usize, e: f64, across: f64, to_weapon: bool, tw_y: f64, tw_p: f64) -> bool {
    let (at, at_b, _) = poses(an);
    let fix_r = an.hand_fix[1];
    let Some(Some((n, to))) = with_pose(g, at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &[HIPS, THIGH_R, SPINE1, SPINE2])) {
            return None;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let (hp, th) = (fk.at(role(HIPS)), fk.at(role(THIGH_R)));
        // the wrist over the flashlight's ring at the front of the right hip · over the sidearm's grip on the thigh
        let (belt, hol) = (untwist(fk, tw_y, tw_p, [hp[0] - 0.13, hp[1] + 0.1, hp[2] + 0.12]), untwist(fk, tw_y, tw_p, [th[0] - 0.1, th[1] + 0.06, th[2] + 0.01]));
        let (first, second) = if to_weapon { (belt, hol) } else { (hol, belt) };
        Some((n, v3::lerp3(first, second, across)))
    }) else {
        return false;
    };
    let Some(mut h) = hands_of(g, an) else {
        return false;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return false;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (live0, rot0) = (fk.at(role(HAND_R)), fk.rot(role(HAND_R)));
        // (the shoulder drops a little with the reach down)
        turn(d, rig, role(SHOULDER_R), q_roll(4.0 * DEGR * e));
        fk_run(rig, d, fk.rot, fk.pos);
        // fingers down onto it, the back of the hand out to his right; the elbow back
        let r = quat::nlerp(rot0, hand_rot(&fix_r, [0.05, -1.0, 0.15], [-0.75, 0.0, 0.6]), e);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &v3::lerp3(live0, to, e), Some(&[-0.5, 0.15, -1.0]), e, Some(&r), false);
        hand(d, hh, 1, &[0.72, 0.78, 0.82, 0.85], 0.5, 0.0);
    });
    if !ask_blend(g, at, at, 1.0, false, at_sw, n) {
        return false;
    }
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
        return false;
    }
    hands_to(&h, an);
    true
}
/// A place and three axes
#[derive(Clone, Copy)]
struct Frame {
    o: [f64; 3],
    x: [f64; 3],
    y: [f64; 3],
    z: [f64; 3],
}
/// an Adj as a layer is handed it: is there one, its offset along the parent's axes, its turn in degrees about the parent's x, then y, then z
type Adj = (bool, [f64; 3], [f64; 3]);
fn frame_from(o: [f64; 3], along: [f64; 3], up_hint: [f64; 3]) -> Frame {
    let z = v3::normalize(along);
    let mut y = v3::sub(up_hint, v3::scale(z, v3::dot(up_hint, z)));
    if v3::len(y) < 1e-3 {
        y = v3::sub([1.0, 0.0, 0.0], v3::scale(z, z[0]));
    }
    let y = v3::normalize(y);
    Frame { o, x: v3::cross(y, z), y, z }
}
/// The child frame = the parent's moved by the offset and turned about the parent's x, then y, then z
fn apply_adj(f: &Frame, adj: &Adj) -> Frame {
    if !adj.0 {
        return *f;
    }
    let (fo, fx, fy, fz, a) = (f.o, f.x, f.y, f.z, adj.1);
    let o = [((fo[0] + fx[0] * a[0]) + fy[0] * a[1]) + fz[0] * a[2], ((fo[1] + fx[1] * a[0]) + fy[1] * a[1]) + fz[1] * a[2], ((fo[2] + fx[2] * a[0]) + fy[2] * a[1]) + fz[2] * a[2]];
    let dg = PI / 180.0;
    let (mut x, mut y, mut z) = (f.x, f.y, f.z);
    for (axis, deg) in [(f.x, adj.2[0]), (f.y, adj.2[1]), (f.z, adj.2[2])] {
        let ang = deg * dg;
        if !(abs(ang) < 1e-6) {
            let q = quat::axis_angle(axis, ang);
            x = quat::rotate(q, x);
            y = quat::rotate(q, y);
            z = quat::rotate(q, z);
        }
    }
    Frame { o, x, y, z }
}
/// the parent frame whose child by `adj` is g
fn un_adj(g: &Frame, adj: &Adj) -> Frame {
    if !adj.0 {
        return *g;
    }
    // the child's axes in the parent's own coordinates
    let i = apply_adj(&Frame { o: [0.0; 3], x: [1.0, 0.0, 0.0], y: [0.0, 1.0, 0.0], z: [0.0, 0.0, 1.0] }, &(true, [0.0; 3], adj.2));
    let ax = |j: usize| -> [f64; 3] { [g.x[0] * i.x[j] + g.y[0] * i.y[j] + g.z[0] * i.z[j], g.x[1] * i.x[j] + g.y[1] * i.y[j] + g.z[1] * i.z[j], g.x[2] * i.x[j] + g.y[2] * i.y[j] + g.z[2] * i.z[j]] };
    let (x, y, z, a) = (ax(0), ax(1), ax(2), adj.1);
    Frame { o: [g.o[0] - x[0] * a[0] - y[0] * a[1] - z[0] * a[2], g.o[1] - x[1] * a[0] - y[1] * a[1] - z[1] * a[2], g.o[2] - x[2] * a[0] - y[2] * a[1] - z[2] * a[2]], x, y, z }
}
/// The thigh holster's group frame off three joints and the body's forward, in whatever one space they are given in;
/// `h` = how far down the thigh, out from the pelvis and forward
fn holster_frame_of(th: [f64; 3], kn: [f64; 3], hp: [f64; 3], fwd: [f64; 3], h: [f64; 3], adj: &Adj) -> Frame {
    let down = v3::normalize(v3::sub(kn, th));
    let mut outward = v3::sub(th, hp);
    outward = v3::sub(outward, v3::scale(down, v3::dot(outward, down)));
    outward[1] = 0.0;
    let outward = if v3::len(outward) > 1e-3 { v3::normalize(outward) } else { v3::cross(fwd, [0.0, 1.0, 0.0]) };
    apply_adj(&frame_from(v3::mad(v3::mad(v3::mad(th, down, h[0]), outward, h[1]), fwd, h[2]), down, fwd), adj)
}
const PISTOL: Option<KitItem> = Some(KitItem::Pistol);
const RIFLE: Option<KitItem> = Some(KitItem::Rifle);
const CAN: Option<KitItem> = Some(KitItem::Can);
/// where a wrist is, how the hand lies and (for a place a thing lives at) where the elbow points
type Spot = ([f64; 3], [f64; 4], Option<[f64; 3]>);
/// What the steps of an errand for a thing he carries hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct Errand {
    tw_y: f64,
    tw_p: f64,
    rc_inv: [f64; 4],
    // as_kit: the errand as it stands, and what the things it goes between are
    t: f64,
    t_a: f64,
    t_r: f64,
    dur: f64,
    s0: f64,
    roll0: f64,
    re: bool,
    front: bool,
    from: Option<KitItem>,
    to: Option<KitItem>,
    cap: game_types::character::AnimKitCap,
    crouch: f64,
    fix_r: [f64; 4],
    holster: [f64; 3],
    h_adj: Adj,
    p_adj: Adj,
    // kit_frame: the trunk's own frame
    left: [f64; 3],
    up: [f64; 3],
    fwd: [f64; 3],
    q3: [f64; 4],
    // kit_hands: the two arms as the pose underneath has them, and whether a rifle is in it
    s_r: [f64; 3],
    s_l: [f64; 3],
    elbow_u: [f64; 3],
    u_r: Spot,
    u_l: Spot,
    la: f64,
    lb: f64,
    rifle: bool,
    // kit_start: where the right hand set out from
    st: Spot,
    // kit_home: the rifle's home: over the shoulder onto his back, or let down on its sling in front
    back: bool,
    rarc: [f64; 3],
    over: [f64; 3],
    cross: [f64; 3],
    // kit_shares: the empty hand going over the shoulder leads with its fingers; and nothing moved yet
    top: Option<[f64; 4]>,
    ap: f64,
    sl: f64,
    has_from: bool,
    has_to: bool,
    hold: Option<KitItem>,
    stow: f64,
    roll: f64,
    join: f64,
    free: f64,
    pole_w: f64,
    grip: f64,
    pole: Option<[f64; 3]>,
    tg: Spot,
    // arm_room: what an arm can do, by how much of the hand is the errand's and not the pose's under it: none on the first frames nor as it lands
    own: f64,
    lie: Option<[f64; 4]>,
    // support: the support hand: off whatever it had, to wait in front of him and onto the gun late; or let go of, and falling away
    tl: Option<Spot>,
}

/// The kit change's frame: the right hand's errand between what was in it and what will be, both arms by IK on a copy of the pose, whole from the upper
/// arm down. The path begins on the hand as the pose under it had it when the change began and ends on the hand of the pose under it now. The clock is
/// the caller's.
pub fn kit_layer(g: usize, an: &mut Anim, tw_y: f64, tw_p: f64, rc_inv: [f64; 4], sling: &SlingFrontHand, tune: &Knobs) -> bool {
    let mut er = Errand { tw_y, tw_p, rc_inv, ..Errand::default() };
    let (at, at_b, _) = poses(an);
    let Some(k) = an.kit else {
        return false;
    };
    as_kit(an, &k, &mut er);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE2]) { Some(n) } else { None }) else {
        return false;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return false;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return false;
    }
    let hh = &mut hs;
    let Some(left_too) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        kit_frame(fk, &mut er);
        kit_hands(fk, &mut er);
        kit_start(fk, &mut er);
        // where the right wrist is, how the hand lies and where the elbow points with the thing in the fist at the place it lives
        // (the rifle's home: over the shoulder onto his back, or let down on its sling in front)
        kit_home(tune, &mut er);
        // (the elbow of a rifle errand: half of it at a steady rate, so it does not all move with the hand at the top of its speed)
        // (the empty hand going over the shoulder leads with its fingers)
        kit_shares(tune, &mut er);
        if er.has_from && er.t < er.t_a {
            // the item in the fist to where it lives
            put_away(tune, fk, sling, &mut er);
        } else if er.has_to && er.t < er.t_r {
            // the empty hand to where the next one lives
            go_across(tune, fk, sling, &mut er);
        } else if er.has_to {
            // …and up with it into the pose underneath
            bring_up(tune, fk, sling, &mut er);
        } else {
            // nothing next: the emptied hand back to the pose underneath
            back_empty(tune, fk, sling, &mut er);
        }
        rifle_elbow(&mut er);
        // what an arm can do, by how much of the hand is the errand's and not the pose's under it — none on the first frames nor as it lands
        arm_room(tune, &mut er);
        // the support hand, with a gun coming: off whatever it had to wait in front of him, onto the gun late
        support(tune, fk, &mut er);
        right_arm(tune, d, rig, fk, &mut er);
        let curl = [lerp(tune.kit.open[0], tune.kit.fist[0], er.grip), lerp(tune.kit.open[1], tune.kit.fist[1], er.grip), lerp(tune.kit.open[2], tune.kit.fist[2], er.grip), lerp(tune.kit.open[3], tune.kit.fist[3], er.grip)];
        hand(d, hh, 1, &curl, lerp(0.25, 0.6, er.grip), 0.1 * (1.0 - er.grip));
        if let Some(sp) = er.tl {
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &sp.0, None, 0.0, Some(&sp.1), false);
        }
        er.tl.is_some()
    }) else {
        return false;
    };
    // (the support hand's fingers go back to the pose's as it joins)
    let side = |r: usize| if left_too { role(r) } else { usize::MAX };
    ask_mask(&[(role(UPPER_ARM_R), 1.0, true), (side(UPPER_ARM_L), 1.0, true), (side(HAND_L), 1.0 - er.join, true), (side(HAND_L), 1.0, false)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return false;
    }
    hands_to(&hs, an);
    (an.kit_hold, an.kit_stow, an.kit_roll, an.kit_lie) = (Some(er.hold), er.stow, er.roll, er.lie);
    true
}

/// The errand as it stands, and what the things it goes between are.
fn as_kit(an: &mut Anim, k: &game_types::character::AnimKit, er: &mut Errand) {
    let (t, t_a, t_r, dur, s0, roll0, re, front, from, to, cap, crouch, fix_r) = (k.t, k.t_a, k.t_r, k.d, k.s0, k.roll0, k.re, k.front, k.from, k.to, k.cap, an.crouch, an.hand_fix[1]);
    let (h, p) = (&RIG.holster, &RIG.pistol.adj);
    let adj = |a: &Option<RigAdj>| -> Adj { a.map_or((false, [0.0; 3], [0.0; 3]), |v| (true, v.off, v.rot)) };
    let (holster, h_adj, p_adj) = ([h.down, h.out, h.fwd], adj(&h.adj), adj(p));
    er.t = t;
    er.t_a = t_a;
    er.t_r = t_r;
    er.dur = dur;
    er.s0 = s0;
    er.roll0 = roll0;
    er.re = re;
    er.front = front;
    er.from = from;
    er.to = to;
    er.cap = cap;
    er.crouch = crouch;
    er.fix_r = fix_r;
    er.holster = holster;
    er.h_adj = h_adj;
    er.p_adj = p_adj;
}

/// The trunk's own frame.
fn kit_frame(fk: &mut Fk, er: &mut Errand) {
    let (left, up, fwd) = trunk_frame(fk);
    let q3 = fk.rot(role(SPINE3));
    (er.left, er.up, er.fwd, er.q3) = (left, up, fwd, q3);
}

/// A direction of the trunk's frame.
fn dir_t(er: &Errand, q: [f64; 3]) -> [f64; 3] {
    let Errand { left, up, fwd, .. } = *er;
    [left[0] * q[0] + up[0] * q[1] + fwd[0] * q[2], left[1] * q[0] + up[1] * q[1] + fwd[1] * q[2], left[2] * q[0] + up[2] * q[1] + fwd[2] * q[2]]
}

/// The two arms as the pose underneath has them, and whether a rifle is in it.
fn kit_hands(fk: &mut Fk, er: &mut Errand) {
    let Errand { from, to, .. } = *er;
    let (s_r, s_l, elbow_u) = (fk.at(role(UPPER_ARM_R)), fk.at(role(UPPER_ARM_L)), fk.at(role(FOREARM_R)));
    let u_r: Spot = (fk.at(role(HAND_R)), fk.rot(role(HAND_R)), None);
    let u_l: Spot = (fk.at(role(HAND_L)), fk.rot(role(HAND_L)), None);
    let (la, lb) = (v3::len(v3::sub(elbow_u, s_r)), v3::len(v3::sub(u_r.0, elbow_u)));
    let rifle = from == RIFLE || to == RIFLE;
    (er.s_r, er.s_l, er.elbow_u, er.u_r, er.u_l, er.la, er.lb, er.rifle) = (s_r, s_l, elbow_u, u_r, u_l, la, lb, rifle);
}

/// A point as this pose sees it.
fn un(fk: &mut Fk, er: &Errand, p: [f64; 3]) -> [f64; 3] {
    let Errand { tw_y, tw_p, .. } = *er;
    untwist(fk, tw_y, tw_p, p)
}

/// A direction as this pose sees it.
fn und(er: &Errand, dv: [f64; 3]) -> [f64; 3] {
    let Errand { tw_y, tw_p, .. } = *er;
    untwist_dir(tw_y, tw_p, dv)
}

/// Where the right hand set out from.
fn kit_start(fk: &mut Fk, er: &mut Errand) {
    let Errand { cap, fix_r, q3, s_r, .. } = *er;
    let st: Spot = (match cap.mr { Some(m) => un(fk, er, m), None => v3::add(s_r, dir_t(er, cap.r)) }, match (cap.mf, cap.mb) { (Some(f), Some(b)) => hand_rot(&fix_r, und(er, f), und(er, b)), _ => quat::mul(q3, cap.rq) }, None);
    er.st = st;
}

/// Where the right wrist is, how the hand lies and where the elbow points with the thing in the fist at the place it lives.
fn spot(tune: &Knobs, fk: &mut Fk, sling: &SlingFrontHand, er: &Errand, it: Option<KitItem>) -> Spot {
    let Errand { rc_inv, front, crouch, fix_r, holster, h_adj, p_adj, .. } = *er;
    if it == RIFLE && front {
        return (v3::add(fk.at(role(SPINE2)), dir_t(er, sling.at)), hand_rot(&fix_r, dir_t(er, sling.fing), dir_t(er, sling.back)), Some(dir_t(er, tune.kit.front.pole)));
    }
    if it == RIFLE {
        let (p, f, b) = (tune.kit.apex.at, tune.kit.apex.fing, tune.kit.apex.back);
        return (v3::add(fk.at(role(UPPER_ARM_R)), dir_t(er, p)), hand_rot(&fix_r, dir_t(er, f), dir_t(er, b)), Some(dir_t(er, tune.kit.apex.pole)));
    }
    if it == CAN {
        let (p, f, b) = (tune.kit.belt.at, tune.kit.belt.fing, tune.kit.belt.back);
        return (un(fk, er, v3::add(fk.at(role(HIPS)), dir_t(er, p))), hand_rot(&fix_r, und(er, dir_t(er, f)), und(er, dir_t(er, b))), Some(dir_t(er, tune.kit.belt.pole)));
    }
    if it == PISTOL {
        // the hand that holds the gun as it stands in the holster
        let p = un_adj(&holster_frame_of(fk.at(role(THIGH_R)), fk.at(role(SHIN_R)), fk.at(role(HIPS)), [0.0, 0.0, 1.0], holster, &h_adj), &p_adj);
        return (un(fk, er, p.o), hand_rot(&fix_r, und(er, p.z), und(er, v3::scale(p.x, -1.0))), Some(dir_t(er, tune.kit.holster_pole)));
    }
    let (p, r, pl) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
    (p, r, Some(pl))
}

/// How the hand rounds the way to where a thing lives.
fn arc_of(tune: &Knobs, it: Option<KitItem>) -> [f64; 3] {
    if it == PISTOL {
        *tune.kit.arc.get(game_types::character::KitItem::Pistol).unwrap()
    } else if it == RIFLE {
        *tune.kit.arc.get(game_types::character::KitItem::Rifle).unwrap()
    } else if it == CAN {
        *tune.kit.arc.get(game_types::character::KitItem::Can).unwrap()
    } else {
        *tune.kit.arc.get(game_types::character::KitItem::Knife).unwrap()
    }
}

/// The rifle's home: over the shoulder onto his back, or let down on its sling in front.
fn kit_home(tune: &Knobs, er: &mut Errand) {
    let Errand { front, rifle, .. } = *er;
    let back = rifle && !front;
    let rarc = if front { tune.kit.front.arc } else { arc_of(tune, RIFLE) };
    let (over, cross) = (tune.kit.over, tune.kit.cross);
    (er.back, er.rarc, er.over, er.cross) = (back, rarc, over, cross);
}

/// A smooth share.
fn e3(u: f64) -> f64 {
    let u = clamp(u, 0.0, 1.0);
    u * u * (3.0 - 2.0 * u)
}

/// The elbow of a rifle errand: half of it at a steady rate, so it does not all move with the hand at the top of its speed.
fn eh(u: f64) -> f64 {
    let u = clamp(u, 0.0, 1.0);
    0.5 * (u + u * u * (3.0 - 2.0 * u))
}

/// Between two spots, rounded by an arc and turned through a third.
fn mix(er: &Errand, sa: &Spot, sb: &Spot, e: f64, arc: Option<[f64; 3]>, via: Option<[f64; 4]>) -> Spot {
    let p = v3::lerp3(sa.0, sb.0, e);
    (
        match arc {
            Some(ar) => v3::mad(p, dir_t(er, ar), sin(PI * e)),
            None => p,
        },
        match via {
            Some(v) => quat::slerp(quat::slerp(sa.1, v, e), quat::slerp(v, sb.1, e), e),
            None => quat::slerp(sa.1, sb.1, e),
        },
        None,
    )
}

/// The empty hand going over the shoulder leads with its fingers; and nothing moved yet.
fn kit_shares(tune: &Knobs, er: &mut Errand) {
    let Errand { front, from, to, fix_r, back, .. } = *er;
    let top = if back { Some(hand_rot(&fix_r, dir_t(er, tune.kit.top.fing), dir_t(er, tune.kit.top.back))) } else { None };
    let ap = clamp(if front { tune.kit.front.hand_at } else { tune.kit.apex_at }, 0.05, 0.95);
    let sl = clamp(if front { tune.kit.front.sling_at } else { tune.kit.sling_at }, 0.05, ap);
    let (has_from, has_to) = (from.is_some(), to.is_some());
    let hold: Option<KitItem> = None;
    let (stow, roll, join, free) = (0.0, 0.0, 0.0, 0.0);
    let (pole_w, grip) = (0.0, 0.0);
    let pole: Option<[f64; 3]> = None;
    let tg = Spot::default();
    er.top = top;
    er.ap = ap;
    er.sl = sl;
    er.has_from = has_from;
    er.has_to = has_to;
    er.hold = hold;
    er.stow = stow;
    er.roll = roll;
    er.join = join;
    er.free = free;
    er.pole_w = pole_w;
    er.grip = grip;
    er.pole = pole;
    er.tg = tg;
}

/// The item in the fist to where it lives.
fn put_away(tune: &Knobs, fk: &mut Fk, sling: &SlingFrontHand, er: &mut Errand) {
    let Errand { t, t_a, s0, roll0, from, rifle, st, rarc, ap, sl, .. } = *er;
    let (hold, stow, roll, pole_w, grip, pole, tg);
    let (it, u) = (from, t / t_a);
    let hu = if it == RIFLE { clamp(u / ap, 0.0, 1.0) } else { u };
    let (e, s) = (e3(hu), spot(tune, fk, sling, er, it));
    tg = mix(er, &st, &s, e, Some(if it == RIFLE { rarc } else { arc_of(tune, it) }), None);
    pole = s.2;
    pole_w = 0.85 * if rifle { eh(hu) } else { e3(hu * 1.6) };
    hold = it;
    grip = 1.0;
    roll = max(smoothstep(0.0, if it == RIFLE { tune.kit.roll_in } else { 0.35 }, hu), roll0);
    stow = max(
        s0,
        if it == RIFLE {
            e3((u - sl) / (1.0 - sl))
        } else if it == PISTOL {
            smoothstep(0.65, 1.0, u)
        } else {
            0.0
        },
    );
    (er.hold, er.stow, er.roll, er.pole_w, er.grip, er.pole, er.tg) = (hold, stow, roll, pole_w, grip, pole, tg);
}

/// The empty hand to where the next one lives.
fn go_across(tune: &Knobs, fk: &mut Fk, sling: &SlingFrontHand, er: &mut Errand) {
    let Errand { t, t_a, t_r, from, to, rifle, st, back, rarc, over, cross, top, has_from, .. } = *er;
    let (pole_w, grip, mut pole, tg);
    let u = (t - t_a) / max(1e-3, t_r - t_a);
    let e = e3(u);
    let sa = if has_from { spot(tune, fk, sling, er, from) } else { st };
    let s = spot(tune, fk, sling, er, to);
    let arc = if back {
        if has_from {
            over
        } else {
            tune.kit.up
        }
    } else if to == RIFLE {
        rarc
    } else if has_from {
        cross
    } else {
        arc_of(tune, to)
    };
    tg = mix(er, &sa, &s, e, Some(arc), top);
    pole = s.2;
    pole_w = 0.85
        * if has_from {
            1.0
        } else if rifle {
            eh(u)
        } else {
            e3(u * 1.6)
        };
    if rifle && has_from {
        // (shoulder ↔ hip: the elbow goes with the hand, not ahead of it)
        if let (Some(pa), Some(pb)) = (sa.2, pole) {
            pole = Some(v3::lerp3(pa, pb, eh(u)));
        }
    }
    grip = smoothstep(0.65, 1.0, u);
    (er.pole_w, er.grip, er.pole, er.tg) = (pole_w, grip, pole, tg);
}

/// Up with it into the pose underneath.
fn bring_up(tune: &Knobs, fk: &mut Fk, sling: &SlingFrontHand, er: &mut Errand) {
    let Errand { t, t_r, dur, s0, roll0, re, to, u_r, rifle, st, rarc, ap, sl, .. } = *er;
    let (hold, stow, roll, join, free, pole_w, grip, pole, tg);
    let (it, u) = (to, (t - t_r) / max(1e-3, dur - t_r));
    let s = if re { st } else { spot(tune, fk, sling, er, it) };
    let hu = if it == RIFLE && !re { clamp((u - (1.0 - ap)) / ap, 0.0, 1.0) } else { u };
    let e = e3(hu);
    tg = mix(er, &s, &u_r, e, if re { None } else { Some(if it == RIFLE { rarc } else { arc_of(tune, it) }) }, None);
    pole = s.2;
    pole_w = 0.85 * (1.0 - if rifle { eh(hu) } else { e3(hu * 1.25) });
    hold = it;
    grip = 1.0;
    join = smoothstep(tune.kit.join, 1.0, hu);
    roll = if re { roll0 * (1.0 - smoothstep(0.4, 1.0, hu)) } else { 1.0 - smoothstep(if it == RIFLE { 1.0 - tune.kit.roll_in } else { 0.65 }, 1.0, hu) };
    free = e;
    stow = if re {
        s0 * (1.0 - smoothstep(0.0, 0.6, u))
    } else if it == RIFLE {
        1.0 - e3(u / (1.0 - sl))
    } else if it == PISTOL {
        1.0 - smoothstep(0.0, 0.35, u)
    } else {
        0.0
    };
    (er.hold, er.stow, er.roll, er.join, er.free, er.pole_w, er.grip, er.pole, er.tg) = (hold, stow, roll, join, free, pole_w, grip, pole, tg);
}

/// Nothing next: the emptied hand back to the pose underneath.
fn back_empty(tune: &Knobs, fk: &mut Fk, sling: &SlingFrontHand, er: &mut Errand) {
    let Errand { t, t_a, dur, from, u_r, st, back, over, top, has_from, .. } = *er;
    let (free, pole_w, grip, pole, tg);
    let u = (t - t_a) / max(1e-3, dur - t_a);
    let e = e3(u);
    let sa = if has_from { spot(tune, fk, sling, er, from) } else { st };
    tg = mix(er, &sa, &u_r, e, if back && from == RIFLE { Some(over) } else { None }, if from == RIFLE { top } else { None });
    pole = sa.2;
    pole_w = 0.85 * (1.0 - e);
    grip = 1.0 - smoothstep(0.0, 0.45, u);
    free = e;
    (er.free, er.pole_w, er.grip, er.pole, er.tg) = (free, pole_w, grip, pole, tg);
}

/// A rifle's errand: the elbow exactly where its pole says, the pole itself led over from where the pose underneath has the elbow.
fn rifle_elbow(er: &mut Errand) {
    let Errand { s_r, elbow_u, u_r, rifle, mut pole_w, mut pole, .. } = *er;
    if let (true, Some(pl)) = (rifle, pole) {
        // a rifle's errand: the elbow exactly where its pole says, the pole itself led over from where the pose underneath has the elbow
        let (ax, eb) = (v3::sub(u_r.0, s_r), v3::sub(elbow_u, s_r));
        let al = v3::len(ax);
        let pu = if al > 1e-4 { v3::mad(eb, ax, -v3::dot(eb, ax) / (al * al)) } else { eb };
        let pu = if v3::len(pu) > 1e-3 { v3::normalize(pu) } else { v3::normalize(pl) };
        let (wv, pn) = (clamp(pole_w / 0.85, 0.0, 1.0), v3::normalize(pl));
        let m = v3::lerp3(pu, pn, wv);
        pole = Some(if v3::len(m) > 1e-3 { v3::normalize(m) } else { pn });
        pole_w = 1.0;
    }
    (er.pole_w, er.pole) = (pole_w, pole);
}

/// What an arm can do, by how much of the hand is the errand's and not the pose's under it: none on the first frames nor as it lands.
fn arm_room(tune: &Knobs, er: &mut Errand) {
    let Errand { t, s_r, la, lb, rifle, free, mut tg, .. } = *er;
    let own = if rifle { clamp(t / 0.12, 0.0, 1.0) * (1.0 - free * free) } else { 0.0 };
    let lie: Option<[f64; 4]> = None;
    if own > 0.0 {
        let r_min = sqrt(max(0.0, la * la + lb * lb - 2.0 * la * lb * cos(tune.kit.elbow_min * PI / 180.0))) * own;
        let dv = v3::sub(tg.0, s_r);
        let dl = v3::len(dv);
        if dl < r_min {
            tg.0 = v3::mad(s_r, if dl > 1e-4 { v3::scale(dv, 1.0 / dl) } else { dir_t(er, [-0.4, 0.2, 0.9]) }, r_min);
        }
    }
    (er.tg, er.own, er.lie) = (tg, own, lie);
}

/// The support hand: off whatever it had, to wait in front of him and onto the gun late; or let go of, and falling away.
fn support(tune: &Knobs, fk: &mut Fk, er: &mut Errand) {
    let Errand { t, to, cap, q3, s_l, u_l, mut join, .. } = *er;
    let gun_next = to == PISTOL || to == RIFLE;
    let s_l0: Spot = (match cap.ml { Some(m) => un(fk, er, m), None => v3::add(s_l, dir_t(er, cap.l)) }, quat::mul(q3, cap.lq), None);
    let mut tl: Option<Spot> = None;
    if gun_next {
        let rest: Spot = (v3::add(s_l, dir_t(er, tune.kit.rest)), s_l0.1, None);
        tl = Some(mix(er, &mix(er, &s_l0, &rest, e3(t / 0.22), None, None), &u_l, join, None, None));
    } else if t < tune.kit.let_go {
        // …and no gun coming: it lets go and falls away, not snapped off
        join = e3(t / tune.kit.let_go);
        tl = Some(mix(er, &s_l0, &u_l, join, None, None));
    }
    (er.join, er.tl) = (join, tl);
}

/// The right arm, its wrist's line brought back toward its forearm's where the path asked more than a wrist bends.
fn right_arm(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, er: &mut Errand) {
    let Errand { fix_r, pole_w, pole, tg, own, mut lie, .. } = *er;
    let pw = if pole.is_some() { pole_w } else { 0.0 };
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tg.0, pole.as_ref(), pw, Some(&tg.1), false);
    if own > 0.0 {
        // the wrist: the hand's line brought back toward its forearm's where the path asked more of it than a wrist bends
        let fa = v3::sub(fk.at(role(HAND_R)), fk.at(role(FOREARM_R)));
        let (fl, fg) = (v3::len(fa), quat::rotate(quat::mul(tg.1, quat::conj(fix_r)), [0.0, 1.0, 0.0]));
        if fl > 1e-4 {
            let fd = v3::scale(fa, 1.0 / fl);
            let (ang, kn) = (acos(clamp(v3::dot(fd, fg), -1.0, 1.0)), tune.kit.wrist_knee * PI / 180.0);
            let mx = max(kn + 0.01, tune.kit.wrist_max * PI / 180.0);
            if ang > kn {
                let want = kn + (mx - kn) * (1.0 - exp(-(ang - kn) / (mx - kn)));
                let ax = v3::cross(fg, fd);
                let al = v3::len(ax);
                if al > 1e-4 {
                    let lim = quat::mul(quat::axis_angle(v3::scale(ax, 1.0 / al), (ang - want) * own), tg.1);
                    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tg.0, pole.as_ref(), pw, Some(&lim), false);
                    lie = Some(quat::mul(quat::conj(lim), tg.1));
                }
            }
        }
    }
    er.lie = lie;
}
