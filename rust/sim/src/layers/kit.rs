//! Things fetched, put away and swapped.
use super::*;

/// The gun hand's errand between the flashlight's ring at the front of the hip and the sidearm on the thigh: the wrist from wherever the pose has it
/// down onto the one, across to the other, and back. The pose as it stood is kept at `at_sw` (what was in the hand is drawn off it).
pub fn swap_pose(g: usize, an: &mut Anim, at_sw: usize, e: f64, across: f64, to_weapon: bool, tw_y: f64, tw_p: f64) -> u32 {
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
        Some((n, lerp3(&first, &second, across)))
    }) else {
        return 0;
    };
    let Some(mut h) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut h;
    with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (live0, rot0) = (fk.at(role(HAND_R)), fk.rot(role(HAND_R)));
        // (the shoulder drops a little with the reach down)
        turn(d, rig, role(SHOULDER_R), q_roll(4.0 * DEGR * e));
        fk_run(rig, d, fk.rot, fk.pos);
        // fingers down onto it, the back of the hand out to his right; the elbow back
        let r = nlerp(&rot0, &hand_rot(&fix_r, [0.05, -1.0, 0.15], [-0.75, 0.0, 0.6]), e);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &lerp3(&live0, &to, e), Some(&[-0.5, 0.15, -1.0]), e, Some(&r), false);
        hand(d, hh, 1, &[0.72, 0.78, 0.82, 0.85], 0.5, 0.0);
    });
    if !ask_blend(g, at, at, 1.0, false, at_sw, n) {
        return 0;
    }
    ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
    if !ask_blend(g, at, at_b, smoothstep(0.0, 0.25, e), true, at, n) {
        return 0;
    }
    hands_to(&h, an);
    RAN | RAN_ALL
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
    let z = normalize(along);
    let mut y = sub3(up_hint, scale3(z, dot3(&up_hint, &z)));
    if len3(&y) < 1e-3 {
        y = sub3([1.0, 0.0, 0.0], scale3(z, z[0]));
    }
    let y = normalize(y);
    Frame { o, x: cross3(&y, &z), y, z }
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
            let q = axis_angle(&axis, ang);
            x = q_rotate(&q, &x);
            y = q_rotate(&q, &y);
            z = q_rotate(&q, &z);
        }
    }
    Frame { o, x, y, z }
}
/// rigProps.ts applyAdj backwards — the parent frame whose child by `adj` is g
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
    let down = normalize(sub3(kn, th));
    let mut outward = sub3(th, hp);
    outward = sub3(outward, scale3(down, dot3(&outward, &down)));
    outward[1] = 0.0;
    let outward = if len3(&outward) > 1e-3 { normalize(outward) } else { cross3(&fwd, &[0.0, 1.0, 0.0]) };
    apply_adj(&frame_from(mad3(mad3(mad3(th, down, h[0]), outward, h[1]), fwd, h[2]), down, fwd), adj)
}
/// where a wrist is, how the hand lies and (for a place a thing lives at) where the elbow points
type Spot = ([f64; 3], [f64; 4], Option<[f64; 3]>);
/// The kit change's frame: the right hand's errand between what was in it and what will be, both arms by IK on a copy of the pose, whole from the upper
/// arm down. The path begins on the hand as the pose under it had it when the change began and ends on the hand of the pose under it now. The clock is
/// the caller's.
pub fn kit_layer(g: usize, an: &mut Anim, tw_y: f64, tw_p: f64, rc_inv: [f64; 4], sling: &SlingFrontHand, tune: &Knobs) -> u32 {
    let (at, at_b, _) = poses(an);
    let Some(k) = an.kit else {
        return 0;
    };
    let (t, t_a, t_r, dur, s0, roll0, re, front, from, to, cap, crouch, fix_r) = (k.t, k.t_a, k.t_r, k.d, k.s0, k.roll0, k.re, k.front, k.from, k.to, k.cap, an.crouch, an.hand_fix[1]);
    let (h, p) = (&RIG.holster, &RIG.pistol.adj);
    let adj = |a: &Option<RigAdj>| -> Adj { a.map_or((false, [0.0; 3], [0.0; 3]), |v| (true, v.off, v.rot)) };
    let (holster, h_adj, p_adj) = ([h.down, h.out, h.fwd], adj(&h.adj), adj(p));
    const PISTOL: Option<KitItem> = Some(KitItem::Pistol);
    const RIFLE: Option<KitItem> = Some(KitItem::Rifle);
    const CAN: Option<KitItem> = Some(KitItem::Can);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE2]) { Some(n) } else { None }) else {
        return 0;
    };
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    let Some((hold, stow, roll, lie, left, join)) = with_pose(g, at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        let (left, up, fwd) = trunk_frame(fk);
        let q3 = fk.rot(role(SPINE3));
        let dir_t = |q: [f64; 3]| -> [f64; 3] { [left[0] * q[0] + up[0] * q[1] + fwd[0] * q[2], left[1] * q[0] + up[1] * q[1] + fwd[1] * q[2], left[2] * q[0] + up[2] * q[1] + fwd[2] * q[2]] };
        let (s_r, s_l, elbow_u) = (fk.at(role(UPPER_ARM_R)), fk.at(role(UPPER_ARM_L)), fk.at(role(FOREARM_R)));
        let u_r: Spot = (fk.at(role(HAND_R)), fk.rot(role(HAND_R)), None);
        let u_l: Spot = (fk.at(role(HAND_L)), fk.rot(role(HAND_L)), None);
        let (la, lb) = (len3(&sub3(elbow_u, s_r)), len3(&sub3(u_r.0, elbow_u)));
        let rifle = from == RIFLE || to == RIFLE;
        let un = |p: [f64; 3]| untwist(fk, tw_y, tw_p, p);
        let und = |dv: [f64; 3]| untwist_dir(tw_y, tw_p, dv);
        let st: Spot = (match cap.mr { Some(m) => un(m), None => add3(s_r, dir_t(cap.r)) }, match (cap.mf, cap.mb) { (Some(f), Some(b)) => hand_rot(&fix_r, und(f), und(b)), _ => q_mul(&q3, &cap.rq) }, None);
        // where the right wrist is, how the hand lies and where the elbow points with the thing in the fist at the place it lives
        let spot = |it: Option<KitItem>| -> Spot {
            if it == RIFLE && front {
                return (add3(fk.at(role(SPINE2)), dir_t(sling.at)), hand_rot(&fix_r, dir_t(sling.fing), dir_t(sling.back)), Some(dir_t(tune.kit.front.pole)));
            }
            if it == RIFLE {
                let (p, f, b) = (tune.kit.apex.at, tune.kit.apex.fing, tune.kit.apex.back);
                return (add3(fk.at(role(UPPER_ARM_R)), dir_t(p)), hand_rot(&fix_r, dir_t(f), dir_t(b)), Some(dir_t(tune.kit.apex.pole)));
            }
            if it == CAN {
                let (p, f, b) = (tune.kit.belt.at, tune.kit.belt.fing, tune.kit.belt.back);
                return (un(add3(fk.at(role(HIPS)), dir_t(p))), hand_rot(&fix_r, und(dir_t(f)), und(dir_t(b))), Some(dir_t(tune.kit.belt.pole)));
            }
            if it == PISTOL {
                // the hand that holds the gun as it stands in the holster
                let p = un_adj(&holster_frame_of(fk.at(role(THIGH_R)), fk.at(role(SHIN_R)), fk.at(role(HIPS)), [0.0, 0.0, 1.0], holster, &h_adj), &p_adj);
                return (un(p.o), hand_rot(&fix_r, und(p.z), und(scale3(p.x, -1.0))), Some(dir_t(tune.kit.holster_pole)));
            }
            let (p, r, pl) = sheath_fist(fk, &rc_inv, &fix_r, crouch, 0.0, 0.0);
            (p, r, Some(pl))
        };
        let arc_of = |it: Option<KitItem>| -> [f64; 3] {
            if it == PISTOL {
                *tune.kit.arc.get(game_types::character::KitItem::Pistol).unwrap()
            } else if it == RIFLE {
                *tune.kit.arc.get(game_types::character::KitItem::Rifle).unwrap()
            } else if it == CAN {
                *tune.kit.arc.get(game_types::character::KitItem::Can).unwrap()
            } else {
                *tune.kit.arc.get(game_types::character::KitItem::Knife).unwrap()
            }
        };
        // (the rifle's home: over the shoulder onto his back, or let down on its sling in front)
        let back = rifle && !front;
        let rarc = if front { tune.kit.front.arc } else { arc_of(RIFLE) };
        let (over, cross) = (tune.kit.over, tune.kit.cross);
        let e3 = |u: f64| -> f64 {
            let u = clamp(u, 0.0, 1.0);
            u * u * (3.0 - 2.0 * u)
        };
        // (the elbow of a rifle errand: half of it at a steady rate, so it does not all move with the hand at the top of its speed)
        let eh = |u: f64| -> f64 {
            let u = clamp(u, 0.0, 1.0);
            0.5 * (u + u * u * (3.0 - 2.0 * u))
        };
        let mix = |sa: &Spot, sb: &Spot, e: f64, arc: Option<[f64; 3]>, via: Option<[f64; 4]>| -> Spot {
            let p = lerp3(&sa.0, &sb.0, e);
            (
                match arc {
                    Some(ar) => mad3(p, dir_t(ar), sin(PI * e)),
                    None => p,
                },
                match via {
                    Some(v) => slerp(&slerp(&sa.1, &v, e), &slerp(&v, &sb.1, e), e),
                    None => slerp(&sa.1, &sb.1, e),
                },
                None,
            )
        };
        // (the empty hand going over the shoulder leads with its fingers)
        let top = if back { Some(hand_rot(&fix_r, dir_t(tune.kit.top.fing), dir_t(tune.kit.top.back))) } else { None };
        let ap = clamp(if front { tune.kit.front.hand_at } else { tune.kit.apex_at }, 0.05, 0.95);
        let sl = clamp(if front { tune.kit.front.sling_at } else { tune.kit.sling_at }, 0.05, ap);
        let (has_from, has_to) = (from.is_some(), to.is_some());
        let mut hold: Option<KitItem> = None;
        let (mut stow, mut roll, mut join, mut free) = (0.0, 0.0, 0.0, 0.0);
        let (mut pole_w, grip): (f64, f64);
        let mut pole: Option<[f64; 3]>;
        let mut tg: Spot;
        if has_from && t < t_a {
            // the item in the fist to where it lives
            let (it, u) = (from, t / t_a);
            let hu = if it == RIFLE { clamp(u / ap, 0.0, 1.0) } else { u };
            let (e, s) = (e3(hu), spot(it));
            tg = mix(&st, &s, e, Some(if it == RIFLE { rarc } else { arc_of(it) }), None);
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
        } else if has_to && t < t_r {
            // the empty hand to where the next one lives
            let u = (t - t_a) / max(1e-3, t_r - t_a);
            let e = e3(u);
            let sa = if has_from { spot(from) } else { st };
            let s = spot(to);
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
                arc_of(to)
            };
            tg = mix(&sa, &s, e, Some(arc), top);
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
                    pole = Some(lerp3(&pa, &pb, eh(u)));
                }
            }
            grip = smoothstep(0.65, 1.0, u);
        } else if has_to {
            // …and up with it into the pose underneath
            let (it, u) = (to, (t - t_r) / max(1e-3, dur - t_r));
            let s = if re { st } else { spot(it) };
            let hu = if it == RIFLE && !re { clamp((u - (1.0 - ap)) / ap, 0.0, 1.0) } else { u };
            let e = e3(hu);
            tg = mix(&s, &u_r, e, if re { None } else { Some(if it == RIFLE { rarc } else { arc_of(it) }) }, None);
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
        } else {
            // nothing next: the emptied hand back to the pose underneath
            let u = (t - t_a) / max(1e-3, dur - t_a);
            let e = e3(u);
            let sa = if has_from { spot(from) } else { st };
            tg = mix(&sa, &u_r, e, if back && from == RIFLE { Some(over) } else { None }, if from == RIFLE { top } else { None });
            pole = sa.2;
            pole_w = 0.85 * (1.0 - e);
            grip = 1.0 - smoothstep(0.0, 0.45, u);
            free = e;
        }
        if let (true, Some(pl)) = (rifle, pole) {
            // a rifle's errand: the elbow exactly where its pole says, the pole itself led over from where the pose underneath has the elbow
            let (ax, eb) = (sub3(u_r.0, s_r), sub3(elbow_u, s_r));
            let al = len3(&ax);
            let pu = if al > 1e-4 { mad3(eb, ax, -dot3(&eb, &ax) / (al * al)) } else { eb };
            let pu = if len3(&pu) > 1e-3 { normalize(pu) } else { normalize(pl) };
            let (wv, pn) = (clamp(pole_w / 0.85, 0.0, 1.0), normalize(pl));
            let m = lerp3(&pu, &pn, wv);
            pole = Some(if len3(&m) > 1e-3 { normalize(m) } else { pn });
            pole_w = 1.0;
        }
        // what an arm can do, by how much of the hand is the errand's and not the pose's under it — none on the first frames nor as it lands
        let own = if rifle { clamp(t / 0.12, 0.0, 1.0) * (1.0 - free * free) } else { 0.0 };
        let mut lie: Option<[f64; 4]> = None;
        if own > 0.0 {
            let r_min = sqrt(max(0.0, la * la + lb * lb - 2.0 * la * lb * cos(tune.kit.elbow_min * PI / 180.0))) * own;
            let dv = sub3(tg.0, s_r);
            let dl = len3(&dv);
            if dl < r_min {
                tg.0 = mad3(s_r, if dl > 1e-4 { scale3(dv, 1.0 / dl) } else { dir_t([-0.4, 0.2, 0.9]) }, r_min);
            }
        }
        // the support hand, with a gun coming: off whatever it had to wait in front of him, onto the gun late
        let gun_next = to == PISTOL || to == RIFLE;
        let s_l0: Spot = (match cap.ml { Some(m) => un(m), None => add3(s_l, dir_t(cap.l)) }, q_mul(&q3, &cap.lq), None);
        let mut tl: Option<Spot> = None;
        if gun_next {
            let rest: Spot = (add3(s_l, dir_t(tune.kit.rest)), s_l0.1, None);
            tl = Some(mix(&mix(&s_l0, &rest, e3(t / 0.22), None, None), &u_l, join, None, None));
        } else if t < tune.kit.let_go {
            // …and no gun coming: it lets go and falls away, not snapped off
            join = e3(t / tune.kit.let_go);
            tl = Some(mix(&s_l0, &u_l, join, None, None));
        }
        let pw = if pole.is_some() { pole_w } else { 0.0 };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tg.0, pole.as_ref(), pw, Some(&tg.1), false);
        if own > 0.0 {
            // the wrist: the hand's line brought back toward its forearm's where the path asked more of it than a wrist bends
            let fa = sub3(fk.at(role(HAND_R)), fk.at(role(FOREARM_R)));
            let (fl, fg) = (len3(&fa), q_rotate(&q_mul(&tg.1, &conj(fix_r)), &[0.0, 1.0, 0.0]));
            if fl > 1e-4 {
                let fd = scale3(fa, 1.0 / fl);
                let (ang, kn) = (acos(clamp(dot3(&fd, &fg), -1.0, 1.0)), tune.kit.wrist_knee * PI / 180.0);
                let mx = max(kn + 0.01, tune.kit.wrist_max * PI / 180.0);
                if ang > kn {
                    let want = kn + (mx - kn) * (1.0 - exp(-(ang - kn) / (mx - kn)));
                    let ax = cross3(&fg, &fd);
                    let al = len3(&ax);
                    if al > 1e-4 {
                        let lim = q_mul(&axis_angle(&scale3(ax, 1.0 / al), (ang - want) * own), &tg.1);
                        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R), &tg.0, pole.as_ref(), pw, Some(&lim), false);
                        lie = Some(q_mul(&conj(lim), &tg.1));
                    }
                }
            }
        }
        let curl = [lerp(tune.kit.open[0], tune.kit.fist[0], grip), lerp(tune.kit.open[1], tune.kit.fist[1], grip), lerp(tune.kit.open[2], tune.kit.fist[2], grip), lerp(tune.kit.open[3], tune.kit.fist[3], grip)];
        hand(d, hh, 1, &curl, lerp(0.25, 0.6, grip), 0.1 * (1.0 - grip));
        if let Some(sp) = tl {
            two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L), &sp.0, None, 0.0, Some(&sp.1), false);
        }
        (hold, stow, roll, lie, tl.is_some(), join)
    }) else {
        return 0;
    };
    // (the support hand's fingers go back to the pose's as it joins)
    let side = |r: usize| if left { role(r) } else { usize::MAX };
    ask_mask(&[(role(UPPER_ARM_R), 1.0, true), (side(UPPER_ARM_L), 1.0, true), (side(HAND_L), 1.0 - join, true), (side(HAND_L), 1.0, false)]);
    if !ask_blend(g, at, at_b, 1.0, true, at, n) {
        return 0;
    }
    hands_to(&hs, an);
    (an.kit_hold, an.kit_stow, an.kit_roll, an.kit_lie) = (Some(hold), stow, roll, lie);
    RAN | RAN_ALL
}
