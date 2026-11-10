//! A walk, a run, a crouch and a stance: the legs and the trunk.
use super::*;

/// The trunk's share of MOVES, straight into the pose.
pub fn mv_trunk(g: usize, an: &Anim, w: f64, o: MvGait, aim_pass: bool, tune: &Knobs) -> u32 {
    let (og, ou, ost, phase, speed, rock, sho, head) = (o.g, o.u, o.st, an.phase, an.speed, an.mvs.rock, an.mvs.sho, an.mvs.head);
    with_pose(g, an.p_a.at as usize, |d, rig, n, _fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
        }
        let (s1, s2, s3, nk, hd) = (role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (mut lean, mut chin, mut sway, mut roll);
        if aim_pass {
            lean = tune.mv.gait.aim_lean * ost + tune.mv.gait.lean * 0.5 * og - tune.mv.gait.back * ou;
            chin = tune.mv.gait.chin * ou;
            (sway, roll) = (0.0, 0.0);
        } else {
            lean = tune.mv.gait.lean * og;
            let ph = 2.0 * PI * phase;
            let mw = smoothstep(0.2, 0.8, abs(speed));
            sway = tune.mv.gait.slouch_sway * ou * mw * sin(ph);
            roll = tune.mv.gait.slouch_roll * ou * mw * cos(ph);
            chin = 0.0;
        }
        lean *= DEGR * w;
        chin *= DEGR * w;
        sway *= DEGR * w;
        roll *= DEGR * w;
        if abs(lean) > 1e-5 {
            turn(d, rig, s1, q_pitch(lean * 0.45));
            turn(d, rig, s2, q_pitch(lean * 0.35));
            turn(d, rig, s3, q_pitch(lean * 0.2));
            turn(d, rig, nk, q_pitch(-lean * 0.4));
            turn(d, rig, hd, q_pitch(-lean * 0.6));
        }
        if abs(sway) + abs(roll) > 1e-5 {
            let q = |kk: f64| q_mul(&q_yaw(sway * kk), &q_roll(roll * kk));
            turn(d, rig, s2, q(0.45));
            turn(d, rig, s3, q(0.55));
            turn(d, rig, nk, q_yaw(-sway * 0.4));
            turn(d, rig, hd, q_yaw(-sway * 0.4));
        }
        if chin > 1e-5 {
            turn(d, rig, nk, q_pitch(-chin * 0.4));
            turn(d, rig, hd, q_pitch(-chin * 0.6));
        }
        if rock > 1e-4 || sho > 1e-4 {
            let q = |kk: f64| q_mul(&q_yaw(-sho * DEGR * w * kk), &q_pitch(-rock * DEGR * w * kk));
            turn(d, rig, s2, q(0.4));
            turn(d, rig, s3, q(0.6));
            // the head keeps most of its line, and is jarred back off the stock
            let hq = |kk: f64| q_mul(&q_yaw(sho * 0.6 * DEGR * w * kk), &q_pitch((rock * 0.6 - head) * DEGR * w * kk));
            turn(d, rig, nk, hq(0.4));
            turn(d, rig, hd, hq(0.6));
        }
        RAN
    })
    .unwrap_or(0)
}
/// The long gun, trunk half: the run's pumping of the shoulder line taken back out down the spine (about the model's upright, or by the sprint's share
/// about the trunk's own line), the upper spine bladed toward the gun, the lean, the head set to the stock.
pub fn long_body(g: usize, an: &Anim, w: f64, run: f64, sprint: f64, ch: &[f64; LC_COUNT]) -> u32 {
    let (long_steady, c_blade, c_lean, c_hy, c_hp, c_hr) = (an.long_steady, ch[LC_BLADE], ch[LC_LEAN], ch[LC_HEAD_YAW], ch[LC_HEAD_PITCH], ch[LC_HEAD_ROLL]);
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD, SHOULDER_L, SHOULDER_R, UPPER_ARM_L, UPPER_ARM_R]) {
            return 0;
        }
        if w < 0.003 {
            return RAN;
        }
        let (s1, s2, s3, nk, hd) = (role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let mut ran = RAN;
        let hold = run * clamp(long_steady, 0.0, 1.0) * LONG_STEADY.run_yaw * w;
        if hold > 0.003 {
            fk.part(rig, d, list(FK_ARMS));
            ran |= ran_list(FK_ARMS);
            let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
            // (by the SPRINT's share of the pose: a jog is held as it always was, to the bit)
            let lv = clamp(sprint, 0.0, 1.0);
            if lv > 0.0 {
                // … about the TRUNK'S OWN LINE: spine1 → between the clavicle roots
                let (ca, cb, sp1) = (fk.at(role(SHOULDER_L)), fk.at(role(SHOULDER_R)), fk.at(s1));
                let u = [ca[0] + (cb[0] - ca[0]) * 0.5 - sp1[0], ca[1] + (cb[1] - ca[1]) * 0.5 - sp1[1], ca[2] + (cb[2] - ca[2]) * 0.5 - sp1[2]];
                let ul = hypot3(u[0], u[1], u[2]);
                if ul > 1e-4 {
                    let inv = 1.0 / ul;
                    let un = [u[0] * inv, u[1] * inv, u[2] * inv];
                    let s = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
                    let m = -(s[0] * un[0] + s[1] * un[1] + s[2] * un[2]);
                    let sp = [s[0] + un[0] * m, s[1] + un[1] * m, s[2] + un[2] * m];
                    let rf = [1.0 - un[0] * un[0], -un[0] * un[1], -un[0] * un[2]];
                    let cr = [rf[1] * sp[2] - rf[2] * sp[1], rf[2] * sp[0] - rf[0] * sp[2], rf[0] * sp[1] - rf[1] * sp[0]];
                    // the twist about it off square, the share of it to take out
                    let ty = atan2(cr[0] * un[0] + cr[1] * un[1] + cr[2] * un[2], rf[0] * sp[0] + rf[1] * sp[1] + rf[2] * sp[2]) * hold * lv;
                    if abs(ty) > 1e-4 {
                        turn(d, rig, s1, axis_angle(&un, -ty * 0.3));
                        turn(d, rig, s2, axis_angle(&un, -ty * 0.35));
                        turn(d, rig, s3, axis_angle(&un, -ty * 0.35));
                        turn(d, rig, nk, axis_angle(&un, ty));
                    }
                }
            }
            if lv < 1.0 {
                // the shoulder line's yaw off square (+ = turned to the figure's left), the share of it to take out
                let sy = atan2(b[2] - a[2], a[0] - b[0]) * hold * (1.0 - lv);
                if abs(sy) > 1e-4 {
                    turn(d, rig, s1, q_yaw(-sy * 0.3));
                    turn(d, rig, s2, q_yaw(-sy * 0.35));
                    turn(d, rig, s3, q_yaw(-sy * 0.35));
                }
            }
        }
        let (blade, lean) = (c_blade * DEGR * w, c_lean * DEGR * w);
        let (hy, hp, hr) = (c_hy * DEGR * w, c_hp * DEGR * w, c_hr * DEGR * w);
        if abs(blade) > 1e-5 {
            turn(d, rig, s2, q_yaw(blade * 0.4));
            turn(d, rig, s3, q_yaw(blade * 0.6));
            turn(d, rig, nk, q_yaw(-blade * 0.5));
            turn(d, rig, hd, q_yaw(-blade * 0.5));
        }
        if abs(lean) > 1e-5 {
            turn(d, rig, s1, q_pitch(lean * 0.45));
            turn(d, rig, s2, q_pitch(lean * 0.35));
            turn(d, rig, s3, q_pitch(lean * 0.2));
            turn(d, rig, nk, q_pitch(-lean * 0.4));
            turn(d, rig, hd, q_pitch(-lean * 0.6));
        }
        if abs(hy) + abs(hp) + abs(hr) > 1e-5 {
            turn(d, rig, nk, q_mul(&q_yaw(hy * 0.4), &q_pitch(hp * 0.4)));
            turn(d, rig, hd, q_mul(&q_roll(hr), &q_mul(&q_yaw(hy * 0.6), &q_pitch(hp * 0.6))));
        }
        ran
    })
    .unwrap_or(0)
}
/// Before the stances and the arms: the recoil's sums bled off (each at its own rate, by training), and the
/// pelvis's share of how a side carries itself: a trained man with a long gun sinks onto bent knees when he moves and most of the walk's bob is taken
/// out, the feet held where the cycle has them.
pub fn mv_pre(g: usize, an: &mut Anim, dt: f64, move_w: f64, stand_w: f64, tune: &Knobs) -> u32 {
    let (at, trained, lg_w, aim) = (an.p_a.at as usize, an.trained, an.lg.w.x, an.upper_w.aim);
    let (m, o) = (&mut an.mvs, &mut an.mv_out);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        let t = clamp(trained, 0.0, 1.0);
        let kk = |p0: f64, p1: f64| lerp(p0, p1, t);
        let sum = m.back + m.climb + abs(m.yaw) + m.rock + m.sho + m.head;
        if sum > 0.0 {
            if sum < 1e-4 {
                (m.back, m.climb, m.yaw, m.rock, m.sho, m.head) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            } else {
                let (eb, ec, er) = (exp(-dt / kk(tune.mv.recoil.back_tau[0], tune.mv.recoil.back_tau[1])), exp(-dt / kk(tune.mv.recoil.climb_tau[0], tune.mv.recoil.climb_tau[1])), exp(-dt / kk(tune.mv.recoil.rock_tau[0], tune.mv.recoil.rock_tau[1])));
                m.back *= eb;
                m.climb *= ec;
                m.yaw *= ec;
                m.rock *= er;
                m.sho *= er;
                m.head *= er;
            }
        }
        let mut ran = RAN;
        let (lgw, tw) = (clamp(lg_w, 0.0, 1.0), smoothstep(0.5, 1.0, trained));
        spring_step(&mut m.glide, tw * move_w * lgw, 9.0, dt);
        o.g = clamp(m.glide.x, 0.0, 1.0);
        o.u = (1.0 - smoothstep(0.0, 0.5, trained)) * lgw * stand_w;
        o.st = tw * lgw * stand_w;
        let drop = tune.mv.gait.drop * o.g + tune.mv.gait.aim_drop * o.st * aim * (1.0 - o.g);
        if drop > 2e-4 || o.g > 0.003 {
            fk.part(rig, d, list(FK_LEGS));
            let hy = fk.at(role(HIPS))[1];
            m.hip_y = if finite(m.hip_y) { m.hip_y + (hy - m.hip_y) * (1.0 - exp(-dt / 0.35)) } else { hy };
            let flat = clamp((m.hip_y - hy) * tune.mv.gait.bob * o.g, -0.03, 0.03);
            mv_hips(d, rig, fk, &[0.0, flat - drop, 0.0]);
            ran |= RAN_ALL;
        } else {
            m.hip_y = f64::NAN;
        }
        o.live = o.g > 0.003 || o.u > 0.003 || o.st > 0.003 || m.rock > 1e-4 || m.sho > 1e-4;
        ran
    })
    .unwrap_or(0)
}
/// The stance, body half: the pelvis bladed toward the wall and settled onto soft knees over feet put back exactly where the locomotion has them, the
/// upper spine countering, a lean spread hips-first with the head levelled back, the stance's own head yaw and chin.
pub fn stance_body(g: usize, an: &Anim, w: f64, speed: f64, stand: f64) -> u32 {
    let (s_blade, s_squat, s_lean, s_hp, s_hy, press) = (an.st.blade.x, an.st.squat.x, an.st.lean.x, an.st.head_pitch.x, an.st.head_yaw.x, an.stack_press_sm);
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let (hips, s1, s2, s3, nk, hd) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3), role(NECK), role(HEAD));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let blade = s_blade * DEGR * w * (1.0 - stand) * lerp(1.0, 0.5, smoothstep(0.3, 1.3, speed));
        let drop = max(0.0, s_squat) * w * (1.0 - stand);
        let replant = abs(blade) > 1e-4 || drop > 1e-4;
        let mut ran = RAN;
        let (mut fl, mut fr, mut rl, mut rr) = ([0.0; 3], [0.0; 3], [0.0; 4], [0.0; 4]);
        if replant {
            fk.part(rig, d, list(FK_LEGS));
            ran |= ran_list(FK_LEGS);
            (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        }
        if abs(blade) > 1e-4 {
            turn(d, rig, hips, q_yaw(blade));
            turn(d, rig, s1, q_yaw(-blade * 0.25));
            turn(d, rig, s2, q_yaw(-blade * 0.25));
            turn(d, rig, s3, q_yaw(-blade * 0.15));
        }
        if drop > 1e-4 {
            shift(d, rig, hips, &[0.0, -drop, 0.0]);
            turn(d, rig, hips, q_pitch(drop * 40.0 * DEGR));
        }
        let lean = s_lean * DEGR * w * (1.0 - 0.5 * stand);
        turn(d, rig, s1, q_pitch(lean * 0.45));
        turn(d, rig, s2, q_pitch(lean * 0.35));
        turn(d, rig, s3, q_pitch(lean * 0.2));
        let (hp, hy, unl) = (s_hp * DEGR * w, s_hy * DEGR * w, lean + drop * 40.0 * DEGR);
        turn(d, rig, nk, q_mul(&q_yaw(hy * 0.4), &q_pitch(-unl * 0.4 + hp * 0.4)));
        turn(d, rig, hd, q_mul(&q_yaw(hy * 0.6), &q_pitch(-unl * 0.6 + hp * 0.6)));
        if replant {
            fk.part(rig, d, list(FK_LEGS));
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &fl, Some(&knee_pole(&rl)), 0.6, Some(&rl), false);
            two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &fr, Some(&knee_pole(&rr)), 0.6, Some(&rr), true);
            ran |= RAN_ALL;
        }
        if press != 0.0 {
            shift(d, rig, hips, &[-press * w * (1.0 - stand), 0.0, 0.0]);
        }
        ran
    })
    .unwrap_or(0)
}
/// CharacterAnimator.jogShape — the run clip as the game shows it: its bounce flattened toward the mean by phase, and at a jog its stride warped shorter at
/// the feet about the gait's middle. `a`: 0 ph · the clip's pelvis height at the two samples about the phase: 1, 2 · 3 its mean · 4 runK · 5 jogMid
fn jog_shape(g: usize, at: usize, an: &Anim, ph: f64) -> u32 {
    let i0 = (floor(ph * 64.0) as i32 & 63) as usize;
    let (y0, y1, mean, run_k, mid) = (an.jog_hips_y[i0] as f64, an.jog_hips_y[(i0 + 1) & 63] as f64, an.jog_hips_mean, an.run_k, an.jog_mid);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        let fi = ph * 64.0;
        let fr = fi - floor(fi);
        let hy = y0 * (1.0 - fr) + y1 * fr;
        shift(d, rig, role(HIPS), &[0.0, -(hy - mean) * (1.0 - BOB.keep), 0.0]);
        if !(run_k < 0.999) {
            return RAN;
        }
        fk.part(rig, d, list(FK_LEGS));
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let tgt = |f: &[f64; 3]| -> [f64; 3] { [f[0], f[1] * lerp(1.0, run_k, 0.5), mid + (f[2] - mid) * run_k] };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tgt(&fl), Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tgt(&fr), Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// CharacterAnimator.sprintShape — the sprint clip as the game shows it: its stride warped at the ankles about the gait's middle, heights eased half way with
/// it about the ankles' floor, never an ankle so low its toe goes through the floor. `a`: 0 sprintMid · 1 sprintFloorY
fn sprint_shape(g: usize, at: usize, an: &Anim) -> u32 {
    let (mid, y0) = (an.sprint_mid, an.sprint_floor_y);
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[TOE_L, TOE_R]) {
            return 0;
        }
        let kk = SPRINT_WARP.stride;
        if kk > 0.999 {
            return RAN;
        }
        fk.part(rig, d, list(FK_LEGS));
        let ky = lerp(1.0, kk, 0.5);
        let (foot_l, foot_r) = (role(FOOT_L), role(FOOT_R));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let (tl0, tr0) = (fk.at(role(TOE_L))[1] - fl[1], fk.at(role(TOE_R))[1] - fr[1]);
        let tgt = |f: &[f64; 3], toe_off: f64| -> [f64; 3] {
            let mut y = y0 + (f[1] - y0) * ky;
            if y + toe_off < TOE_FLOOR_Y {
                y = TOE_FLOOR_Y - toe_off;
            }
            [f[0], y, mid + (f[2] - mid) * kk]
        };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), foot_l, &tgt(&fl, tl0), Some(&knee_pole(&rl)), 0.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), foot_r, &tgt(&fr, tr0), Some(&knee_pole(&rr)), 0.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// a clip sampled into a pose as it stands (sampleClip with no copy first: the nodes the clip does not write keep what the pose held)
fn clip_at(g: usize, at: usize, nodes: usize, clip: f64, t: f64) -> bool {
    clip >= 0.0 && ask_sample(g, at, nodes, clip as usize, t, true)
}
/// CharacterAnimator.sampleJog — the jog clip at phase `ph` (its time `t`), its SEAM closed: the clip's last key is not its first, and each joint that
/// jumps there is carried half the jump back at the cycle's end and half forward at its start (the rig's table JOG_SEAM, six numbers a joint: its
/// node · 0 an origin, 1 a rotation · the jump: an offset, or an axis and, last, its angle)
fn sample_jog(g: usize, at: usize, nodes: usize, clip: f64, t: f64, ph: f64) -> bool {
    if !clip_at(g, at, nodes, clip, t) {
        return false;
    }
    let kk = ph - floor(ph) - 0.5;
    with_pose(g, at, |d, _rig, n, _fk| {
        for s in nums(JOG_SEAM).chunks_exact(6) {
            // (a seam's node; which of its channels moves is s[1], an origin or a rotation)
            let i = s[0] as usize;
            if i >= n {
                continue;
            }
            if s[1] == 0.0 {
                let (x, y, z) = (ch(d, i, 0) + s[2] * kk, ch(d, i, 1) + s[3] * kk, ch(d, i, 2) + s[4] * kk);
                set_ch(d, i, 0, x);
                set_ch(d, i, 1, y);
                set_ch(d, i, 2, z);
            } else {
                let q = q_mul(&own_rot(d, i), &axis_angle(&[s[2], s[3], s[4]], s[5] * kk));
                set_own_rot(d, i, &q);
            }
        }
    })
    .is_some()
}
/// a clip by its number, and where in it
#[derive(Clone, Copy)]
pub struct At {
    pub clip: f64,
    pub t: f64,
}
/// how much of each gait the stand pose shows, and the clips of them
pub struct Cycle {
    pub ph: f64,
    pub w_idle: f64,
    pub w_walk: f64,
    pub w_jog: f64,
    pub w_sprint: f64,
    pub idle: At,
    pub jog: At,
    pub sprint: At,
    pub walk: At,
}
/// The standing locomotion: the idle, and over it the cycle (the walk, the jog laid on it, or the jog and the sprint laid together), each run shaped as
/// the game shows it before the blend. The cycle's own poses are left in the scratch poses for the layers that read them.
pub fn stand_pose(g: usize, an: &Anim, c: &Cycle) -> u32 {
    let (at, at_b, at_c) = (an.p_a.at as usize, an.p_b.at as usize, an.p_c.at as usize);
    let (ph, w_idle, w_walk, w_jog, w_sprint) = (c.ph, c.w_idle, c.w_walk, c.w_jog, c.w_sprint);
    let Some(n) = with_pose(g, at, |_d, _rig, n, _fk| n) else {
        return 0;
    };
    let moving = w_walk + w_jog + w_sprint > 0.001;
    let mut code = RAN;
    // (the idle is not sampled when the cycle's pose is about to replace all of it)
    if !(moving && 1.0 - w_idle >= 1.0) && !clip_at(g, at, n, c.idle.clip, c.idle.t) {
        return 0;
    }
    if moving {
        let mut shaped = |s: u32| {
            code |= s;
            s != 0
        };
        if w_sprint > 0.001 {
            if !sample_jog(g, at_b, n, c.jog.clip, c.jog.t, ph) || !shaped(jog_shape(g, at_b, an, ph)) {
                return 0;
            }
            if !clip_at(g, at_c, n, c.sprint.clip, c.sprint.t) || !shaped(sprint_shape(g, at_c, an)) || !ask_blend(g, at_b, at_c, w_sprint, false, at_b, n) {
                return 0;
            }
        } else {
            if !clip_at(g, at_b, n, c.walk.clip, c.walk.t) {
                return 0;
            }
            if w_jog > 0.001 && (!sample_jog(g, at_c, n, c.jog.clip, c.jog.t, ph) || !shaped(jog_shape(g, at_c, an, ph)) || !ask_blend(g, at_b, at_c, w_jog, false, at_b, n)) {
                return 0;
            }
        }
        if !ask_blend(g, at, at_b, 1.0 - w_idle, false, at, n) {
            return 0;
        }
    }
    code
}
/// The crouch over the stand pose: its idle, the crouch walk's cycle laid on it by how fast he goes, and that laid over the pose by the crouch shown.
pub fn crouch_pose(g: usize, an: &Anim, w_move: f64, crouch: f64, idle: At, fwd: At) -> u32 {
    let (at, at_b, at_c) = (an.p_a.at as usize, an.p_b.at as usize, an.p_c.at as usize);
    let Some(n) = with_pose(g, at, |_d, _rig, n, _fk| n) else {
        return 0;
    };
    if !(w_move >= 1.0) && !clip_at(g, at_b, n, idle.clip, idle.t) {
        return 0;
    }
    if w_move > 0.001 && (!clip_at(g, at_c, n, fwd.clip, fwd.t) || !ask_blend(g, at_b, at_c, w_move, false, at_b, n)) {
        return 0;
    }
    if !ask_blend(g, at, at_b, crouch, false, at, n) {
        return 0;
    }
    RAN
}
/// SOFT_KNEE: each leg re-solved toward its own ankle with the hip→ankle reach eased below full extension.
pub fn soft_knees(g: usize, at: usize, w: f64) -> u32 {
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) {
            return 0;
        }
        fk.part(rig, d, list(FK_LEGS));
        let mut cur = false;
        for (thigh, shin, foot) in [(role(THIGH_L), role(SHIN_L), role(FOOT_L)), (role(THIGH_R), role(SHIN_R), role(FOOT_R))] {
            let (ph, pk, pa) = (fk.at(thigh), fk.at(shin), fk.at(foot));
            let (l, dd) = (dist3(&ph, &pk) + dist3(&pk, &pa), dist3(&ph, &pa));
            let r = dd / l;
            if r <= SOFT_KNEE.from {
                continue;
            }
            let span = SOFT_KNEE.to - SOFT_KNEE.from;
            let x = (r - SOFT_KNEE.from) / span;
            let r_eased = SOFT_KNEE.from + span * (1.0 - exp(-x));
            let r_new = r + (r_eased - r) * w;
            let ar = fk.rot(foot);
            let tgt = [ph[0] + (pa[0] - ph[0]) * r_new / r, ph[1] + (pa[1] - ph[1]) * r_new / r, ph[2] + (pa[2] - ph[2]) * r_new / r];
            two_bone_ik(rig, d, fk.rot, fk.pos, thigh, shin, foot, &tgt, Some(&knee_pole(&ar)), 0.0, Some(&ar), cur);
            cur = true;
        }
        RAN | if cur { RAN_ALL } else { ran_list(FK_LEGS) }
    })
    .unwrap_or(0)
}
/// A run carries its arms close: each hand read in the shoulder line's own frame (to his left along the two shoulder joints,
/// level; up; forward), its sideways travel drawn in by the run's share of the standing pose, its fore-aft travel lengthened and its front lifted by the
/// sprint's, and the arm solved to it with the elbow behind him. The hand follows its forearm.
pub fn run_arms(g: usize, at: usize, w: f64, ws: f64, tune: &Knobs) -> u32 {
    with_pose(g, at, |d, rig, n, fk| {
        if !cast(n, &ARM_ROLES) {
            return 0;
        }
        fk.part(rig, d, list(FK_ARMS));
        let (a, b) = (fk.at(role(UPPER_ARM_L)), fk.at(role(UPPER_ARM_R)));
        let (mut lx, mut lz) = (a[0] - b[0], a[2] - b[2]);
        let ll = hypot2(lx, lz);
        if ll < 1e-4 {
            return RAN | ran_list(FK_ARMS);
        }
        lx /= ll;
        lz /= ll;
        // (forward, level: his left × up)
        let (fx, fz, keep, pump) = (-lz, lx, lerp(1.0, tune.runarms.keep, w), lerp(1.0, tune.runarms.pump, ws));
        // (neither solve moves what the other reads: the second finds the skeleton current)
        for right in [false, true] {
            let sg = if right { -1.0 } else { 1.0 };
            let (ua, fa, ha) = if right { (role(UPPER_ARM_R), role(FOREARM_R), role(HAND_R)) } else { (role(UPPER_ARM_L), role(FOREARM_L), role(HAND_L)) };
            let (s, h) = (fk.at(ua), fk.at(ha));
            let (dx, dz) = (h[0] - s[0], h[2] - s[2]);
            let (out, fwd) = ((dx * lx + dz * lz) * sg, dx * fx + dz * fz);
            let (o, f) = (tune.runarms.at + (out - tune.runarms.at) * keep, tune.runarms.mid + (fwd - tune.runarms.mid) * pump);
            let y = h[1] - s[1] + tune.runarms.lift * ws * clamp((fwd - tune.runarms.mid) / tune.runarms.swing, 0.0, 1.0);
            let to = [s[0] + lx * sg * o + fx * f, s[1] + y, s[2] + lz * sg * o + fz * f];
            let pole = [lx * sg * tune.runarms.pole[0] + fx * tune.runarms.pole[2], tune.runarms.pole[1], lz * sg * tune.runarms.pole[0] + fz * tune.runarms.pole[2]];
            two_bone_ik(rig, d, fk.rot, fk.pos, ua, fa, ha, &to, Some(&pole), tune.runarms.pole_w * w, None, right);
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// The run's stance feet: through each foot's stance window of the sampled phase the toe is put on the floor and carried back at the ground's speed, the
/// offset it is left holding bled away up the swing.
pub fn run_plant(g: usize, an: &mut Anim, ph: f64, s: f64, dt: f64, w_run: f64, w_sprint: f64) -> u32 {
    let (jog, sprint, reverse) = (&an.run_gait.jog, &an.run_gait.sprint, an.reverse);
    let mids = [lerp(jog.c_l, sprint.c_l, w_sprint), lerp(jog.c_r, sprint.c_r, w_sprint)];
    let mut plant = [&mut an.plant_l, &mut an.plant_r];
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[TOE_L, TOE_R]) {
            return 0;
        }
        let dz = if reverse { 1.0 } else { -1.0 } * s * dt;
        fk.part(rig, d, list(FK_LEGS));
        let legs = [(role(THIGH_L), role(SHIN_L), role(FOOT_L), role(TOE_L)), (role(THIGH_R), role(SHIN_R), role(FOOT_R), role(TOE_R))];
        let mut cur = false;
        for i in 0..2 {
            let (thigh, shin, foot, toe) = legs[i];
            let mut x = ph - mids[i];
            x -= round(x);
            let w = bump(x, RUN_LINGER.on[0], RUN_LINGER.on[1], RUN_LINGER.off[0], RUN_LINGER.off[1]) * w_run;
            let tp = fk.at(toe);
            let miss = if finite(plant[i].pz) { dz - (tp[2] - plant[i].pz) } else { 0.0 };
            plant[i].pz = tp[2];
            plant[i].dev = clamp((plant[i].dev + if abs(miss) < 0.2 { miss } else { 0.0 } * w) * exp(-(1.0 - w) * dt / max(RUN_LINGER.bleed, 1e-3)), -0.25, 0.25);
            if w < 1e-3 && abs(plant[i].dev) < 5e-4 {
                plant[i].dev = 0.0;
                continue;
            }
            let (ap, ar) = (fk.at(foot), fk.rot(foot));
            two_bone_ik(rig, d, fk.rot, fk.pos, thigh, shin, foot, &[ap[0], ap[1] - (tp[1] - TOE_FLOOR_Y) * w, ap[2] + plant[i].dev], Some(&knee_pole(&ar)), 0.0, Some(&ar), cur);
            cur = true;
        }
        RAN | if cur { RAN_ALL } else { ran_list(FK_LEGS) }
    })
    .unwrap_or(0)
}
/// MOVES.steady: this frame's swing of the crouch cycle (its own sample, the pose at `at_c`, against its own mean) taken out by MV.steady's shares: the
/// pelvis's roll at the hips with the chest handed the same turn back, a little of the hips' sideways travel, both ankles put back nearer his middle and
/// the knees swung in over them.
pub fn steady_layer(g: usize, an: &Anim, w: f64, base: f64, sunk: f64, mean: [f64; 2], tune: &Knobs) -> u32 {
    let (mean_roll, mean_x) = (mean[0], mean[1]);
    let roll_now = |fk: &Fk| -> f64 {
        let (a, b) = (fk.at(role(THIGH_L)), fk.at(role(THIGH_R)));
        atan2(a[1] - b[1], a[0] - b[0])
    };
    // this frame's swing: the cycle's sample off the cycle's mean
    let Some(Some((d_roll, d_x))) = with_pose(g, an.p_c.at as usize, |d, rig, n, fk| {
        if !cast(n, &LEG_ROLES) || !cast(n, &[SPINE3]) {
            return None;
        }
        fk.part(rig, d, list(FK_LEGS));
        Some((roll_now(fk) - mean_roll, fk.at(role(HIPS))[0] - mean_x))
    }) else {
        return 0;
    };
    with_pose(g, an.p_a.at as usize, |d, rig, _n, fk| {
        let (hips, foot_l, foot_r) = (role(HIPS), role(FOOT_L), role(FOOT_R));
        let legs = [(role(THIGH_L), role(SHIN_L), foot_l), (role(THIGH_R), role(SHIN_R), foot_r)];
        fk.part(rig, d, list(FK_LEGS));
        let (fl, fr, rl, rr) = (fk.at(foot_l), fk.at(foot_r), fk.rot(foot_l), fk.rot(foot_r));
        let (tl, tr, sl, sr) = (fk.at(legs[0].0), fk.at(legs[1].0), fk.at(legs[0].1), fk.at(legs[1].1));
        // each knee's set off its hip, and its height, as the cycle has them
        let (ol, or) = ([sl[0] - tl[0], sl[1] - tl[1], sl[2] - tl[2]], [sr[0] - tr[0], sr[1] - tr[1], sr[2] - tr[2]]);
        let (yl, yr) = (sl[1], sr[1]);
        let (a, sx) = (d_roll * tune.mv.steady.roll * w, d_x * tune.mv.steady.sway * w);
        if abs(a) > 1e-6 {
            turn(d, rig, hips, q_roll(-a));
            turn(d, rig, role(SPINE3), q_roll(a));
        }
        if abs(sx) > 1e-6 {
            shift(d, rig, hips, &[-sx, 0.0, 0.0]);
        }
        let kx = 1.0 - (1.0 - tune.mv.steady.feet) * w * base;
        fk.part(rig, d, list(FK_LEGS));
        let (al, ar) = ([fl[0] * kx, fl[1], fl[2]], [fr[0] * kx, fr[1], fr[2]]);
        let (kn, ko) = (tune.mv.steady.knee * w * base, tune.mv.steady.knee_out * w * sunk);
        let (hl, hr) = (smoothstep(0.10, 0.22, yl), smoothstep(0.10, 0.22, yr));
        let (kl, kr) = (1.0 - kn * lerp(tune.mv.steady.knee_low, 1.0, hl) + ko * (1.0 - hl), 1.0 - kn * lerp(tune.mv.steady.knee_low, 1.0, hr) + ko * (1.0 - hr));
        let (pl, pr) = ([ol[0] * kl, ol[1], ol[2]], [or[0] * kr, or[1], or[2]]);
        two_bone_ik(rig, d, fk.rot, fk.pos, legs[0].0, legs[0].1, foot_l, &al, Some(&pl), 1.0, Some(&rl), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, legs[1].0, legs[1].1, foot_r, &ar, Some(&pr), 1.0, Some(&rr), true);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// A crouched man's toes kept out of the floor: the foot joint alone turned about the ankle until the toe is back on the floor, laid away from the knee
/// along the shin's own heading unless the foot already points somewhere along the ground. The ankle is not moved.
pub fn toes_up(g: usize, an: &Anim, w: f64) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &[SHIN_L, FOOT_L, TOE_L, SHIN_R, FOOT_R, TOE_R]) {
            return 0;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let floor_y = TOE_FLOOR_Y;
        for (shin, foot, toe) in [(role(SHIN_L), role(FOOT_L), role(TOE_L)), (role(SHIN_R), role(FOOT_R), role(TOE_R))] {
            let (a, t) = (fk.at(foot), fk.at(toe));
            if t[1] >= floor_y - 1e-4 {
                continue;
            }
            let dv = sub3(t, a);
            let (l, dy) = (hypot3(dv[0], dv[1], dv[2]), floor_y - a[1]);
            if l < 1e-4 || abs(dy) >= l {
                continue;
            }
            let kn = fk.at(shin);
            let (mut hx, mut hz) = (dv[0] / l, dv[2] / l);
            let (bx, bz) = (a[0] - kn[0], a[2] - kn[2]);
            let bl = hypot2(bx, bz);
            if bl > 1e-4 {
                let own = hypot2(hx, hz);
                let kb = 0.35 * (1.0 - own) * (1.0 - own);
                hx += bx / bl * kb;
                hz += bz / bl * kb;
            }
            let hl = hypot2(hx, hz);
            if hl < 1e-4 {
                continue;
            }
            let r = sqrt(l * l - dy * dy);
            let (to, il) = ([hx / hl * r, dy, hz / hl * r], 1.0 / l);
            let q = slerp(&[0.0, 0.0, 0.0, 1.0], &from_to(&[dv[0] * il, dv[1] * il, dv[2] * il], &[to[0] * il, to[1] * il, to[2] * il]), clamp(w, 0.0, 1.0));
            let par = rig.parent[foot];
            let up = if par >= 0 { fk.rot(par as usize) } else { [0.0, 0.0, 0.0, 1.0] };
            set_model_rot(d, foot, &q_mul(&q, &fk.rot(foot)), &up);
        }
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// A turn's speed toward what is left of it (`err`), never faster than it can stop in and never carried past the point. Answers how
/// far it goes this step and the speed it has after.
fn turn_step(w: f64, err: f64, w_max: f64, acc: f64, dt: f64) -> (f64, f64) {
    let a = abs(err);
    if a < 1e-5 {
        return (err, 0.0);
    }
    let s = if err < 0.0 { -1.0 } else { 1.0 };
    let wt = s * min(w_max, sqrt(2.0 * acc * a));
    let mut nw = w + clamp(wt - w, -acc * dt, acc * dt);
    if s * nw > s * wt {
        nw = wt;
    }
    let d = nw * dt;
    if s * d >= a {
        (err, s * min(w_max, a / dt))
    } else {
        (d, nw)
    }
}
/// TURN: bodyYaw and aimYaw as his AI left them are where he WANTS to face; he comes round to them as a mass (his feet the short way round, the gun about his
/// waist through his front), and the two are rewritten with where he has got to. A rule of the character's and not of his pose.
pub fn turn_weigh(ch: &mut Character, dt: f64, tune: &Knobs) {
    let (px, pz, body_yaw, aim_yaw) = (ch.pos[0], ch.pos[2], ch.body_yaw, ch.aim_yaw);
    let stands = ch.tn.filter(|t| !(hypot2(px - t.x, pz - t.z) > tune.turn.jump) && finite(body_yaw + aim_yaw));
    let Some(mut t) = stands else {
        ch.tn = Some(CharacterTn { body: body_yaw, aim: aim_yaw, wb: 0.0, wa: 0.0, ib: body_yaw, ia: aim_yaw, x: px, z: pz });
        return;
    };
    if body_yaw != t.body {
        t.ib = body_yaw;
    }
    if aim_yaw != t.aim {
        t.ia = aim_yaw;
    }
    let (h, kk) = (clamp(ch.turn.h, 0.2, 1.5), lerp(tune.turn.green, 1.0, clamp(ch.turn.k, 0.0, 1.0)));
    let hb = lerp(1.0, h, tune.turn.body_share);
    let sb = turn_step(t.wb, wrap_angle(t.ib - t.body), tune.turn.body * hb, tune.turn.body_acc * hb * kk, dt);
    t.body = wrap_angle(t.body + sb.0);
    t.wb = sb.1;
    let r0 = clamp(wrap_angle(t.aim - t.body), -1.9, 1.9);
    let r1 = clamp(wrap_angle(t.ia - t.ib) + wrap_angle(t.ib - t.body), -1.9, 1.9);
    let sa = turn_step(t.wa, r1 - r0, tune.turn.aim * h, tune.turn.aim_acc * h * kk, dt);
    let rr = clamp(r0 + sa.0, -1.9, 1.9);
    t.wa = if rr == r0 + sa.0 { sa.1 } else { 0.0 };
    t.aim = wrap_angle(t.body + rr);
    (t.x, t.z) = (px, pz);
    (ch.body_yaw, ch.aim_yaw) = (t.body, t.aim);
    let (ld, was) = (clamp(wrap_angle(t.ia - t.aim), -tune.turn.lead, tune.turn.lead), ch.anim.turn_lead);
    ch.anim.turn_lead = damp(was, ld, if abs(ld) < abs(was) { tune.turn.lead_rate * 2.5 } else { tune.turn.lead_rate }, dt);
    ch.tn = Some(t);
}
/// A lean round an edge: the trunk rolled about the figure's fore-aft line over the three spine joints, the arms and the gun with it, and the neck and
/// head brought part of the way back so the eyes stay near level.
pub fn lean_body(g: usize, an: &mut Anim, dt: f64) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, _fk| {
        if !cast(n, &[SPINE1, SPINE2, SPINE3, NECK, HEAD]) {
            return 0;
        }
        let want = clamp(an.lean, -1.0, 1.0);
        an.lean_sm += (want - an.lean_sm) * min(1.0, LEAN_POSE.rate * dt);
        if want == 0.0 && abs(an.lean_sm) < 0.01 {
            an.lean_sm = 0.0;
        }
        let r = an.lean_sm * LEAN_POSE.deg * DEGR;
        if r != 0.0 {
            turn(d, rig, role(SPINE1), q_roll(r * 0.3));
            turn(d, rig, role(SPINE2), q_roll(r * 0.35));
            turn(d, rig, role(SPINE3), q_roll(r * 0.35));
            turn(d, rig, role(NECK), q_roll(-r * LEAN_POSE.level * 0.5));
            turn(d, rig, role(HEAD), q_roll(-r * LEAN_POSE.level * 0.5));
        }
        RAN
    })
    .unwrap_or(0)
}
/// A one-shot action's clip laid over the finished pose (the animator keeps which clip, its clock and its weight), and a man asleep in his chair slumped
/// over it.
pub fn act_pose(g: usize, an: &mut Anim, clip: f64, w: f64, dt: f64) -> u32 {
    let (at, at_b, at_rest, t, looped, slump) = (an.p_a.at as usize, an.p_b.at as usize, an.rest.at as usize, an.act_t, an.act_loop, an.slump);
    let Some(Some(n)) = with_pose(g, at, |_d, _rig, n, _fk| if cast(n, &[SPINE3, NECK, HEAD]) { Some(n) } else { None }) else {
        return 0;
    };
    if !sample_on_rest(g, at_rest, at_b, n, clip, t, looped) || !ask_blend(g, at, at_b, w, false, at, n) {
        return 0;
    }
    let mut code = RAN;
    if slump > 0.0 || an.slump_w > 0.0 {
        let ds = slump - an.slump_w;
        an.slump_w += if ds > 0.0 { min(ds, dt / 1.6) } else { max(ds, -dt / 0.35) };
        let s = an.slump_w * w;
        if s > 1e-4 {
            let br = sin(t * 1.75);
            code = with_pose(g, at, |d, rig, _n, fk| {
                fk_run(rig, d, fk.rot, fk.pos);
                turn(d, rig, role(SPINE3), q_pitch((10.0 + 1.5 * br) * DEGR * s));
                turn(d, rig, role(NECK), q_pitch(17.0 * DEGR * s));
                turn(d, rig, role(HEAD), q_mul(&q_pitch((25.0 + br) * DEGR * s), &q_roll(8.0 * DEGR * s)));
                RAN | RAN_ALL
            })
            .unwrap_or(0);
        }
    }
    code
}
const LC_BLADE: usize = 6;
const LC_LEAN: usize = 7;
const LC_HEAD_YAW: usize = 8;
const LC_HEAD_PITCH: usize = 9;
const LC_HEAD_ROLL: usize = 10;
