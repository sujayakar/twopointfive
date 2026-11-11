//! Back to a wall.
use super::*;

/// a foot's place and its sole's rotation (model space)
type Sole = ([f64; 3], [f64; 4]);
/// …and of the hold's own step: where the knee points as well
type Tread = ([f64; 3], [f64; 4], [f64; 3]);
/// What the steps of a wall hold hand on to the ones after them.
#[derive(Clone, Copy, Default)]
struct OnWall {
    w: f64,
    p0: [f64; 3],
    // as_on_wall: how he stands on the wall, its normal into the room and his right along it, and the sidle's share
    at: usize,
    at_b: usize,
    at_rest: usize,
    at_c: usize,
    idle_t: f64,
    cr: f64,
    s: f64,
    side_v: f64,
    park: f64,
    aim_w: f64,
    ny: f64,
    phase: f64,
    wall_hit: f64,
    aim_yaw: f64,
    lk: f64,
    chest_rest_inv: [f64; 4],
    gaze_local: [f64; 3],
    wall_take: f64,
    on_wall: bool,
    yin: f64,
    take_feet: Option<game_types::character::AnimWallTakeFeet>,
    i_rl: [f64; 4],
    i_ly: f64,
    i_rr: [f64; 4],
    i_ry: f64,
    e: f64,
    n: [f64; 3],
    tr: [f64; 3],
    root_off: f64,
    g_share: f64,
    gw: f64,
    // wall_now: the joints, and the clip's feet underneath
    hips: usize,
    s1: usize,
    s2: usize,
    s3: usize,
    nk: usize,
    hd: usize,
    foot_l: usize,
    foot_r: usize,
    f_l: [f64; 3],
    f_r: [f64; 3],
    r_l: [f64; 4],
    r_r: [f64; 4],
    // wall_lean: the spine measured and put, twice over
    twist: f64,
    gaze_deg: f64,
    // step_clock: which foot leads, how much he moves, and the step's clock
    u: f64,
    mv: f64,
    wall_gait: f64,
    // treads: each foot's own step; and whether the take's steps hold (a short take ends before the pose's weight is whole: the soles stay where they stepped to)
    g_l: Option<Tread>,
    g_r: Option<Tread>,
    taking: bool,
    wtk_hold: bool,
    // soles: both soles' targets; in the take each stays where it stood in the world until its turn, then steps to the hold's spot in a low arc
    t_l: Sole,
    t_r: Sole,
}

/// The wall hold's body: the spine blended toward the standing idle's and then measured and put (the chest square to the wall but for a counter-turn
/// toward the lead side, the spine's line leant onto the plaster, a crouch's pelvis tilt taken back at the pelvis, the peek's own numbers taking over as
/// it comes in), the pelvis carried back onto the wall and along it onto the lead foot, the head turned along the wall, and the feet planted by IK:
/// parked on the hold's own spots, on the move on the hold's own step or the turned cycle narrowed toward the wall, each sole kept where it stood in the
/// world through a take until its turn to step. `at_c`: the standing idle sampled over the rest pose (its spine, and its pelvis) · `at_b`: the crouched
/// idle sampled the same way, read only under the hold's own step (gw > 1e-4): the host samples both until the clip sampler is the module's.
pub fn wall_body(g: usize, an: &mut Anim, w: f64, p0: [f64; 3], idle: f64, crouch_idle: f64, tune: &Knobs) -> bool {
    let mut o = OnWall { w, p0, ..OnWall::default() };
    const ROLES: [usize; 16] = [HIPS, SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R, THIGH_L, SHIN_L, FOOT_L, THIGH_R, SHIN_R, FOOT_R];
    as_on_wall(an, &mut o);
    // the idles underneath, sampled here: the standing one's spine and pelvis, the crouched one's pelvis
    let Some(nodes) = with_pose(g, o.at, |_d, _rig, nodes, _fk| nodes) else {
        return false;
    };
    if !sample_on_rest(g, o.at_rest, o.at_c, nodes, idle, o.idle_t, true) {
        return false;
    }
    let Some(Some(under)) = with_pose(g, o.at_c, |d, _rig, nodes, _fk| {
        if !cast(nodes, &ROLES) {
            return None;
        }
        Some([SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, HIPS].map(|r| own_rot(d, role(r))))
    }) else {
        return false;
    };
    let hips_low = if o.gw > 1e-4 {
        if !sample_on_rest(g, o.at_rest, o.at_b, nodes, crouch_idle, o.idle_t, true) {
            return false;
        }
        let Some(q) = with_pose(g, o.at_b, |d, _rig, _nodes, _fk| own_rot(d, role(HIPS))) else {
            return false;
        };
        q
    } else {
        [0.0; 4]
    };
    let (gait, held, wtk) = (&mut an.wall_gait, &mut an.wtk_hold, &mut an.wtk);
    with_pose(g, o.at, |d, rig, _nodes, fk| {
        wall_now(d, rig, fk, &mut o);
        // 1. the spine toward the standing idle's, joint by joint
        wall_spine(d, &under, &o);
        // 2. the pelvis
        wall_pelvis(tune, d, rig, fk, gait, &under, hips_low, &o);
        // 3. the spine measured and put, twice over
        wall_lean(d, rig, fk, &mut o);
        // 4. the head
        fk_run(rig, d, fk.rot, fk.pos);
        {
            wall_head(tune, d, rig, fk, &o);
        }
        // 5. the feet
        step_clock(gait, &mut o);
        // a parked spot in the wall's frame for one role of one foot, and its toes' yaw
        // the hold's own step: where this sole is on its track at the step clock's phase, its rotation, and where its knee points
        treads(held, &mut o);
        soles(tune, wtk, &mut o);
        fk_run(rig, d, fk.rot, fk.pos);
        let pole_of = |t: &Sole, gf: &Option<Tread>| match gf {
            Some(gf) if o.gw > 1e-4 => v3::normalize(v3::lerp3(knee_pole(&t.1), gf.2, o.gw)),
            _ => knee_pole(&t.1),
        };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), o.foot_l, &o.t_l.0, Some(&pole_of(&o.t_l, &o.g_l)), 0.6, Some(&o.t_l.1), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), o.foot_r, &o.t_r.0, Some(&pole_of(&o.t_r, &o.g_r)), 0.6, Some(&o.t_r.1), true);
        true
    })
    .unwrap_or(false)
}

/// How he stands on the wall, its normal into the room and his right along it, and the sidle's share.
fn as_on_wall(an: &mut Anim, o: &mut OnWall) {
    let OnWall { w, .. } = *o;
    let (at, at_b, at_rest) = poses(an);
    let (at_c, idle_t, ws) = (an.p_c.at as usize, an.idle_t, &an.wst);
    let (cr, s, side_v, pk, park) = (clamp(an.crouch, 0.0, 1.0), clamp(ws.side.x, -1.0, 1.0), ws.side.v, clamp(ws.peek.x, 0.0, 1.0), clamp(ws.park.x, 0.0, 1.0));
    let (aim_w, ny, phase, wall_hit, aim_yaw, lk) = (an.upper_w.aim, an.wall_yaw, an.phase, an.wall_hit, an.aim_yaw_offset, an.look_yaw_extra);
    let (chest_rest_inv, gaze_local, wall_take, on_wall, yin) = (an.chest_rest_inv, an.gaze_local, an.wall_take, an.stance == Stance::Wall, an.body_yaw_in);
    let (take_feet, f) = (an.wall_take_feet, &an.idle_feet);
    let (i_rl, i_ly, i_rr, i_ry) = (f.r_l, f.l[1], f.r_r, f.r[1]);
    let e = pk * pk * (3.0 - 2.0 * pk);
    // the wall's normal (into the room) and his right along the wall, model space
    let n = [sin(ny), 0.0, cos(ny)];
    let tr = [-n[2], 0.0, n[0]];
    let root_off = WALL_ROOT_OFF;
    // the sidle's own step: its share of the feet, and of the pelvis on the move
    let g_share = lerp(WALL_POSE.gait.share[0], WALL_POSE.gait.share[1], cr);
    let gw = g_share * (1.0 - park) * w;
    o.at = at;
    o.at_b = at_b;
    o.at_rest = at_rest;
    o.at_c = at_c;
    o.idle_t = idle_t;
    o.cr = cr;
    o.s = s;
    o.side_v = side_v;
    o.park = park;
    o.aim_w = aim_w;
    o.ny = ny;
    o.phase = phase;
    o.wall_hit = wall_hit;
    o.aim_yaw = aim_yaw;
    o.lk = lk;
    o.chest_rest_inv = chest_rest_inv;
    o.gaze_local = gaze_local;
    o.wall_take = wall_take;
    o.on_wall = on_wall;
    o.yin = yin;
    o.take_feet = take_feet;
    o.i_rl = i_rl;
    o.i_ly = i_ly;
    o.i_rr = i_rr;
    o.i_ry = i_ry;
    o.e = e;
    o.n = n;
    o.tr = tr;
    o.root_off = root_off;
    o.g_share = g_share;
    o.gw = gw;
}

/// The joints, and the clip's feet underneath.
fn wall_now(d: &mut [S], rig: &Rig, fk: &mut Fk, o: &mut OnWall) {
    let (hips, s1, s2, s3, nk, hd) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
    let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
    // what is underneath: the clip's feet
    fk_run(rig, d, fk.rot, fk.pos);
    let (f_l, f_r, r_l, r_r) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
    (o.hips, o.s1, o.s2, o.s3, o.nk, o.hd, o.foot_l, o.foot_r, o.f_l, o.f_r, o.r_l, o.r_r) = (hips, s1, s2, s3, nk, hd, foot_l, foot_r, f_l, f_r, r_l, r_r);
}

/// The spine toward the standing idle's, joint by joint.
fn wall_spine(d: &mut [S], under: &[[f64; 4]; 8], o: &OnWall) {
    let OnWall { w, s1, s2, s3, nk, hd, .. } = *o;
    for (j, (i, share)) in [(s1, 0.9), (s2, 0.9), (s3, 0.9), (nk, 0.85), (hd, 0.85), (role(SHOULDER_L), 0.7), (role(SHOULDER_R), 0.7)].into_iter().enumerate() {
        let q = quat::nlerp(own_rot(d, i), under[j], share * w);
        set_own_rot(d, i, &q);
    }
}

/// The pelvis: turned, put off the wall, dropped, and shifted with the sidle.
fn wall_pelvis(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, gait: &mut f64, under: &[[f64; 4]; 8], hips_low: [f64; 4], o: &OnWall) {
    let OnWall { w, cr, s, park, ny, phase, wall_hit, e, n, tr, root_off, g_share, gw, hips, .. } = *o;
    if park > 0.995 || w < 0.01 {
        *gait = WALL_GAIT_REST;
    }
    if gw > 1e-4 {
        let q = quat::nlerp(own_rot(d, hips), quat::nlerp(under[7], hips_low, cr), gw);
        set_own_rot(d, hips, &q);
    }
    fk_run(rig, d, fk.rot, fk.pos);
    let hp = fk.at(hips);
    let back_off = lerp(lerp(WALL_POSE.back[0], WALL_POSE.back[1], cr), WALL_POSE.back[2], 1.0 - park);
    let out = v3::dot(hp, n) - (-root_off + back_off);
    let drop = lerp(WALL_POSE.drop[0], WALL_POSE.drop[1], cr) + 0.006 * (1.0 - park) * (1.0 - g_share) * (1.0 - cos(phase * 4.0 * PI)) + tune.wallsnap.dip * wall_hit;
    shift(d, rig, hips, &v3::add(v3::add(v3::scale(n, -out * w), [0.0, -drop * w, 0.0]), v3::scale(tr, s * WALL_POSE.peek.shift * e * w)));
    if gw > 1e-4 {
        let lvl = lerp(WALL_POSE.gait.hips[0], WALL_POSE.gait.hips[1], cr) - lerp(WALL_POSE.gait.bob[0], WALL_POSE.gait.bob[1], cr) * 0.5 * (1.0 + cos((*gait - 0.05) * 4.0 * PI));
        shift(d, rig, hips, &v3::add([0.0, (lvl - hp[1]) * gw, 0.0], v3::scale(tr, -v3::dot(hp, tr) * gw)));
    }
    turn(d, rig, hips, quat::mul(quat::axis_angle(n, s * 3.0 * DEGR * e * w), q_yaw(ny * 0.33 * w)));
}

/// Which way the chest is turned.
fn chest_yaw_of(o: &OnWall, fk: &Fk) -> f64 {
    let OnWall { chest_rest_inv, s3, .. } = *o;
    let l = quat::rotate(quat::mul(fk.rot(s3), chest_rest_inv), AX);
    atan2(-l[2], l[0])
}

/// Which way the shoulders are turned.
fn shoulder_yaw_of(fk: &Fk) -> f64 {
    let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
    atan2(b[2] - a[2], a[0] - b[0])
}

/// The spine measured and put, twice over.
fn wall_lean(d: &mut [S], rig: &Rig, fk: &mut Fk, o: &mut OnWall) {
    let OnWall { w, cr, s, aim_w, ny, aim_yaw, e, n, tr, hips, s1, s2, s3, nk, .. } = *o;
    let ax_n = [n[2], 0.0, -n[0]];
    let mut hip_share = 0.0;
    let twist = aim_yaw * (1.0 - w * (1.0 - aim_w));
    let (turn_deg, lean_deg, gaze_deg) = (lerp(WALL_POSE.chest_turn, WALL_POSE.peek.turn, e), lerp(lerp(WALL_POSE.lean[0], WALL_POSE.lean[1], cr), WALL_POSE.peek.tilt, e), lerp(WALL_POSE.gaze, WALL_POSE.peek.gaze, e));
    for _pass in 0..2 {
        fk_run(rig, d, fk.rot, fk.pos);
        let dy = wrap_angle(ny - s * turn_deg * DEGR - (twist - aim_yaw * aim_w) - lerp(shoulder_yaw_of(fk), chest_yaw_of(o, fk), e)) * w;
        turn(d, rig, s1, q_yaw(dy * 0.3));
        turn(d, rig, s2, q_yaw(dy * 0.35));
        turn(d, rig, s3, q_yaw(dy * 0.35));
        fk_run(rig, d, fk.rot, fk.pos);
        let dv = v3::sub(fk.at(nk), fk.at(s1));
        let (tilt_n, tilt_t) = (atan2(v3::dot(dv, n), dv[1]), atan2(v3::dot(dv, tr), dv[1]));
        let (d_n, d_t) = ((lean_deg * DEGR - tilt_n) * w, (s * WALL_POSE.peek.lean * DEGR * e - tilt_t) * w);
        let hs = clamp(d_n, -0.45 * cr - hip_share, 0.45 * cr - hip_share);
        hip_share += hs;
        if abs(hs) > 1e-5 {
            turn(d, rig, hips, quat::axis_angle(ax_n, hs));
        }
        let rest = d_n - hs;
        turn(d, rig, s1, quat::mul(quat::axis_angle(n, d_t * 0.5), quat::axis_angle(ax_n, rest * 0.6)));
        turn(d, rig, s2, quat::mul(quat::axis_angle(n, d_t * 0.3), quat::axis_angle(ax_n, rest * 0.4)));
        turn(d, rig, s3, quat::axis_angle(n, d_t * 0.2));
    }
    (o.twist, o.gaze_deg) = (twist, gaze_deg);
}

/// The head.
fn wall_head(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, o: &OnWall) {
    let OnWall { w, s, aim_w, ny, wall_hit, lk, gaze_local, e, n, nk, hd, twist, gaze_deg, .. } = *o;
    let hw = w * (1.0 - aim_w);
    let gz = quat::rotate(fk.rot(hd), gaze_local);
    let gy = atan2(gz[0], gz[2]);
    let chest_yaw = chest_yaw_of(o, fk);
    let mut want = ny - s * gaze_deg * DEGR - twist;
    want = chest_yaw - lk + clamp(wrap_angle(want + lk - chest_yaw), -80.0 * DEGR, 80.0 * DEGR);
    let dyaw = wrap_angle(want - gy) * hw;
    let el = asin(clamp(gz[1], -1.0, 1.0));
    let dpitch = (el + (WALL_POSE.head_pitch - tune.wallsnap.nod * wall_hit) * DEGR) * hw;
    let gh = v3::normalize([gz[0], 0.0, gz[2] + 1e-6]);
    let ax_p = [gh[2], 0.0, -gh[0]];
    let unroll = -s * WALL_POSE.peek.lean * DEGR * e * 0.5 * hw;
    let q = |share: f64| quat::mul(quat::axis_angle(n, unroll * share), quat::mul(q_yaw(dyaw * share), quat::axis_angle(ax_p, dpitch * share)));
    turn(d, rig, nk, q(0.4));
    turn(d, rig, hd, q(0.6));
}

/// Which foot leads, how much he moves, and the step's clock.
fn step_clock(gait: &mut f64, o: &mut OnWall) {
    let OnWall { w, s, park, g_share, .. } = *o;
    let u = (1.0 + s) / 2.0;
    let mv = (1.0 - park) * w * (1.0 - g_share);
    let wall_gait = *gait;
    (o.u, o.mv, o.wall_gait) = (u, mv, wall_gait);
}

/// A parked spot in the wall's frame for one role of one foot, and its toes' yaw.
fn spot(o: &OnWall, lead: bool, right: bool) -> ([f64; 3], f64) {
    let OnWall { cr, n, tr, root_off, .. } = *o;
    let (stand, low) = if lead {
        ([WALL_POSE.feet.lead[0], WALL_POSE.feet.lead[1], WALL_POSE.feet.lead[2]], [WALL_POSE.feet.lead_crouch[0], WALL_POSE.feet.lead_crouch[1], WALL_POSE.feet.lead_crouch[2]])
    } else {
        ([WALL_POSE.feet.trail[0], WALL_POSE.feet.trail[1], WALL_POSE.feet.trail[2]], [WALL_POSE.feet.trail_crouch[0], WALL_POSE.feet.trail_crouch[1], WALL_POSE.feet.trail_crouch[2]])
    };
    let av = v3::lerp3(stand, low, cr);
    let sg = if right == lead { 1.0 } else { -1.0 };
    (v3::add(v3::scale(tr, sg * av[0]), v3::scale(n, av[1] - root_off)), -sg * av[2] * DEGR)
}

/// The hold's own step: where this sole is on its track at the step clock's phase, its rotation, and where its knee points.
fn gait_foot(o: &OnWall, right: bool, r_idle: &[f64; 4], y_idle: f64) -> Tread {
    let OnWall { cr, s, ny, n, tr, root_off, u, wall_gait, .. } = *o;
    let (sl, duty) = (lerp(WALL_POSE.gait.step[0], WALL_POSE.gait.step[1], cr), WALL_POSE.gait.duty);
    let half = duty * sl;
    let f = (wall_gait + if right { 0.5 } else { 0.0 }) % 1.0;
    let (along, up);
    if f < duty {
        along = half * (1.0 - 2.0 * f / duty);
        up = 0.0;
    } else {
        let q = (f - duty) / (1.0 - duty);
        let q2 = q * q;
        let q3 = q2 * q;
        let m = -2.0 * sl * (1.0 - duty);
        along = (2.0 * q3 - 3.0 * q2 + 1.0) * -half + (q3 - 2.0 * q2 + q) * m + (-2.0 * q3 + 3.0 * q2) * half + (q3 - q2) * m;
        up = sin(PI * q);
    }
    let heel = lerp(WALL_POSE.gait.heel[0], WALL_POSE.gait.heel[1], cr) * DEGR * smoothstep(0.1, 1.0, -along * clamp(s * 2.0, -1.0, 1.0) / half);
    let lw = if right { u } else { 1.0 - u };
    let off = lerp(lerp(WALL_POSE.gait.far[0], WALL_POSE.gait.far[1], cr), lerp(WALL_POSE.gait.near[0], WALL_POSE.gait.near[1], cr), lw);
    let yaw_abs = ny - s * lerp(WALL_POSE.gait.toe[0], WALL_POSE.gait.toe[1], cr) * DEGR * (0.5 + 0.5 * lw);
    let h = [sin(yaw_abs), 0.0, cos(yaw_abs)];
    const BALL: f64 = 0.12;
    let al = along + s * lerp(WALL_POSE.gait.ahead[0], WALL_POSE.gait.ahead[1], cr) + (if right { 1.0 } else { -1.0 }) * lerp(WALL_POSE.gait.apart[0], WALL_POSE.gait.apart[1], cr);
    let p = [
        tr[0] * al + n[0] * (off - root_off) + h[0] * BALL * (1.0 - cos(heel)),
        y_idle + lerp(WALL_POSE.gait.lift[0], WALL_POSE.gait.lift[1], cr) * up + BALL * sin(heel),
        tr[2] * al + n[2] * (off - root_off) + h[2] * BALL * (1.0 - cos(heel)),
    ];
    let fwd = quat::rotate(*r_idle, AY);
    let yaw0 = atan2(fwd[0], fwd[2]);
    let r = quat::normalize(quat::mul(quat::axis_angle([h[2], 0.0, -h[0]], heel), quat::mul(q_yaw(wrap_angle(yaw_abs - yaw0)), *r_idle)));
    let kr = lerp(WALL_POSE.gait.knee_room[0], WALL_POSE.gait.knee_room[1], cr);
    let dn = v3::normalize([n[0] * kr, 0.0, (1.0 - kr) + n[2] * kr]);
    (p, r, v3::normalize([h[0] * 0.25 + dn[0] * 0.75, 0.5, h[2] * 0.25 + dn[2] * 0.75]))
}

/// Each foot's own step; and whether the take's steps hold (a short take ends before the pose's weight is whole: the soles stay where they stepped to).
fn treads(held: &mut bool, o: &mut OnWall) {
    let OnWall { w, wall_take, on_wall, i_rl, i_ly, i_rr, i_ry, g_share, .. } = *o;
    let (g_l, g_r) = if g_share > 0.0 { (Some(gait_foot(o, false, &i_rl, i_ly)), Some(gait_foot(o, true, &i_rr, i_ry))) } else { (None, None) };
    // (a short take ends before the pose's weight is whole: the soles stay where they stepped to)
    let taking = wall_take >= 0.0 && wall_take < 1.0;
    if taking {
        *held = true;
    } else if *held && (w >= 0.98 || !on_wall) {
        *held = false;
    }
    let wtk_hold = *held;
    (o.g_l, o.g_r, o.taking, o.wtk_hold) = (g_l, g_r, taking, wtk_hold);
}

/// Where a sole is bound, parked or stepping.
fn foot_target(o: &OnWall, right: bool, f_clip0: &[f64; 3], r_clip0: &[f64; 4], r_idle: &[f64; 4], y_idle: f64, full: bool) -> Sole {
    let OnWall { w, cr, s, side_v, park, ny, n, root_off, g_share, u, mv, g_l, g_r, wtk_hold, .. } = *o;
    let gf = if right { &g_r } else { &g_l };
    let (f_clip, r_clip) = match gf {
        Some(gf) => {
            let gk = g_share * w;
            (v3::lerp3(*f_clip0, gf.0, gk), quat::nlerp(*r_clip0, gf.1, gk))
        }
        None => (*f_clip0, *r_clip0),
    };
    let lw = if right { u } else { 1.0 - u };
    let ((lp, ly), (tp, ty)) = (spot(o, true, right), spot(o, false, right));
    let mut p = v3::lerp3(tp, lp, lw);
    let yaw_rel = lerp(ty, ly, lw);
    let on_toes = cr * (1.0 - lw) * WALL_POSE.feet.trail_heel;
    p[1] = lerp(y_idle, f_clip[1], on_toes) + 0.04 * (1.0 - s * s) * (if (side_v > 0.0) == right { 1.0 } else { 0.25 }) + 0.08 * park * (1.0 - park) * (0.4 + 0.6 * lw);
    let base = quat::nlerp(*r_idle, r_clip, on_toes);
    let fwd = quat::rotate(base, AY);
    let yaw0 = atan2(fwd[0], fwd[2]);
    let r = quat::normalize(quat::mul(q_yaw(wrap_angle(ny + yaw_rel - yaw0)), base));
    let share = if full { 1.0 } else { park * (if wtk_hold { 1.0 } else { w }) };
    let mut q = v3::lerp3(f_clip, p, share);
    if mv > 1e-4 {
        let off_c = v3::dot(q, n) + root_off;
        q = v3::mad(q, n, (WALL_POSE.slide.line + (off_c - root_off) * WALL_POSE.slide.narrow - off_c) * mv);
    }
    let into = -root_off + 0.11 - v3::dot(q, n);
    (if into > 0.0 { v3::mad(q, n, into * w) } else { q }, quat::nlerp(r_clip, r, share))
}

/// Both soles' targets; in the take each stays where it stood in the world until its turn, then steps to the hold's spot in a low arc.
fn soles(tune: &Knobs, wtk: &mut Option<AnimWtk>, o: &mut OnWall) {
    let OnWall { p0, wall_take, yin, take_feet, i_rl, i_ly, i_rr, i_ry, f_l, f_r, r_l, r_r, taking, .. } = *o;
    let (mut t_l, mut t_r) = (foot_target(o, false, &f_l, &r_l, &i_rl, i_ly, false), foot_target(o, true, &f_r, &r_r, &i_rr, i_ry, false));
    if taking {
        // the take: each sole stays where it stood in the WORLD until its turn, then steps to the hold's spot in a low arc
        let (tk, stands) = (wall_take, Stands::at(p0, yin));
        let (s_l, s_r) = (foot_target(o, false, &f_l, &r_l, &i_rl, i_ly, true), foot_target(o, true, &f_r, &r_r, &i_rr, i_ry, true));
        if wtk.map_or(true, |k| tk < k.last) {
            let (l, r) = match take_feet {
                Some(t) => (t.l, t.r),
                None => (AnimFootSpot { p: stands.world(&f_l), yaw: yin + foot_yaw(&r_l) }, AnimFootSpot { p: stands.world(&f_r), yaw: yin + foot_yaw(&r_r) }),
            };
            let first = if v3::dist_xz(f_l, s_l.0) >= v3::dist_xz(f_r, s_r.0) { 0.0 } else { 1.0 };
            *wtk = Some(AnimWtk { l, r, first, last: tk });
        }
        let mut k = wtk.unwrap_or_default();
        k.last = tk;
        *wtk = Some(k);
        let step = |from: [f64; 3], from_yaw: f64, sp: &Sole, r_clip: &[f64; 4], t0: f64, t1: f64| -> Sole {
            let q = clamp((tk - t0) / (t1 - t0), 0.0, 1.0);
            let ease = q * q * (3.0 - 2.0 * q);
            let mut m = stands.model(&from);
            let (dx, dz) = (m[0] - sp.0[0], m[2] - sp.0[2]);
            let (dl, mx) = (hypot2(dx, dz), tune.wallsnap.step_trail);
            if t0 > 0.0 && dl > mx {
                m[0] = sp.0[0] + dx * mx / dl;
                m[2] = sp.0[2] + dz * mx / dl;
            }
            let mut p = v3::lerp3(m, sp.0, ease);
            p[1] += tune.wallsnap.step_lift * sin(PI * q);
            (p, quat::nlerp(quat::normalize(quat::mul(q_yaw(wrap_angle(from_yaw - yin - foot_yaw(r_clip))), *r_clip)), sp.1, ease))
        };
        // (the further foot goes at once, the other waits, then closes as the back lands)
        let (early, late) = ((0.0, 0.58), (0.42, 1.0));
        let (tl, trr) = if k.first == 0.0 { (early, late) } else { (late, early) };
        t_l = step(k.l.p, k.l.yaw, &s_l, &r_l, tl.0, tl.1);
        t_r = step(k.r.p, k.r.yaw, &s_r, &r_r, trr.0, trr.1);
    } else {
        *wtk = None;
    }
    (o.t_l, o.t_r) = (t_l, t_r);
}
/// The wall hold's arms: both palms laid on the face beside the hips, fingers down and out along it, the elbows where the pose says; mended (WALLSNAP),
/// the wrists hang beside the hips, a resting hand on the plaster with the thumb beside the index.
pub fn wall_arms(g: usize, an: &mut Anim, w: f64) -> bool {
    let (at, at_b, _) = poses(an);
    let (cr, side_x, peek, yaw, gait, park, fixes) = (an.crouch, an.wst.side.x, an.wst.peek.x, an.wall_yaw, an.wall_gait, an.wst.park.x, [an.hand_fix[0], an.hand_fix[1]]);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &ARM_ROLES) && cast(n, &[HIPS]) { Some(n) } else { None }) else {
        return false;
    };
    let Some(mut h) = hands_of(g, an) else {
        return false;
    };
    if !ask_blend(g, at, at, 1.0, false, at_b, n) {
        return false;
    }
    let hh = &mut h;
    let code = with_pose(g, at_b, |d, rig, _n, fk| {
        let (w_cr, s, pk) = (clamp(cr, 0.0, 1.0), clamp(side_x, -1.0, 1.0), clamp(peek, 0.0, 1.0));
        let e = pk * pk * (3.0 - 2.0 * pk);
        // the wall's normal (into the room) and his right along the wall, model space
        let nn = [sin(yaw), 0.0, cos(yaw)];
        let tr = [-nn[2], 0.0, nn[0]];
        fk_run(rig, d, fk.rot, fk.pos);
        let hips = fk.at(role(HIPS));
        let (hx, hy, hz) = (hips[0], hips[1] + lerp(WALL_POSE.mend.up[0], WALL_POSE.mend.up[1], w_cr), hips[2]);
        let along0 = lerp(WALL_POSE.mend.along[0], WALL_POSE.mend.along[1], w_cr) - WALL_POSE.mend.sway * cos(gait * 2.0 * PI) * (1.0 - clamp(park, 0.0, 1.0)) * lerp(WALL_POSE.gait.share[0], WALL_POSE.gait.share[1], w_cr);
        let (e_out, e_room, e_down) = (WALL_POSE.mend.elbow.out, WALL_POSE.mend.elbow.room, WALL_POSE.mend.elbow.down);
        let splay = WALL_POSE.mend.splay;
        for side in 0..2usize {
            // his right is +tr: the right hand goes that way, the left the other
            let sg = if side == 1 { 1.0 } else { -1.0 };
            let along = along0 - WALL_POSE.hands.peek_in * e * (if side == 1 { 1.0 + s } else { 1.0 - s }) / 2.0;
            let mut p = [hx + tr[0] * sg * along, hy, hz + tr[2] * sg * along];
            // carried square onto the plane, the wrist proud of it
            let onto = (-WALL_ROOT_OFF + WALL_POSE.hands.off) - v3::dot(p, nn);
            p[0] += nn[0] * onto;
            p[2] += nn[2] * onto;
            let fix = fixes[side];
            let r = hand_rot(&fix, [tr[0] * sg * splay, -1.0, tr[2] * sg * splay], nn);
            let pole = [tr[0] * sg * e_out + nn[0] * e_room, -e_down, tr[2] * sg * e_out + nn[2] * e_room];
            let (up, fore, hnd) = if side == 1 { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R)) } else { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L)) };
            two_bone_ik(rig, d, fk.rot, fk.pos, up, fore, hnd, &p, Some(&pole), 0.9, Some(&r), false);
            hand(d, hh, side, &[WALL_POSE.mend.curl[0], WALL_POSE.mend.curl[1], WALL_POSE.mend.curl[2], WALL_POSE.mend.curl[3]], WALL_POSE.mend.thumb, WALL_POSE.mend.fan);
            // the thumb beside the index, out of the plaster
            turn(d, rig, digits(side)[0] as usize, quat::axis_angle(quat::rotate(fk.rot(hnd), AY), sg * WALL_POSE.mend.thumb_in * DEGR));
        }
        true
    })
    .unwrap_or(false);
    if !code {
        return false;
    }
    arms_mask();
    if !ask_blend(g, at, at_b, w, true, at, n) {
        return false;
    }
    hands_to(&h, an);
    code
}
