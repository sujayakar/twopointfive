//! A man held: the one who holds and the one who is held.
use super::*;

/// HOLD.lower's number, or HOLD.beats.lower's while the hold is played in beats
#[inline(always)]
fn lower_k(beats: bool, in_beats: f64, plain: f64) -> f64 {
    if beats {
        in_beats
    } else {
        plain
    }
}
const WHIP_WIND: f64 = 0.36;
const WHIP_STRIKE: f64 = 0.42;
const WHIP_THRU: f64 = 0.14;
/// The disarm's clock on the way in, gentle off the mark and at speed as the hand lands at `take`
fn strip_in(t: f64, take: f64) -> f64 {
    if t >= take {
        return t;
    }
    let x = t / take;
    take * x * x * (2.0 - x)
}
/// Down with its clock, back up on a spring of its own
fn lower_ease(c: &mut Spring, target: f64, up: f64, dt: f64) -> f64 {
    if target >= c.x {
        c.x = target;
        c.v = 0.0;
        return target;
    }
    spring_step(c, target, up, dt);
    max(target, c.x)
}
/// The choke's own curve while it runs, let down on a spring when it is let go of
fn haul_ease(c: &mut Spring, on: bool, target: f64, down: f64, dt: f64) -> f64 {
    if on {
        c.x = target;
        c.v = 0.0;
        return target;
    }
    if c.x <= 1e-4 {
        c.x = 0.0;
        c.v = 0.0;
        return 0.0;
    }
    spring_step(c, 0.0, down, dt);
    max(0.0, c.x)
}
/// What the steps of a hold hand on to the ones after them: its clocks, and how much of each thing it does shows.
#[derive(Clone, Copy, Default)]
struct Holding {
    w: f64,
    dt: f64,
    beats: bool,
    man: Option<[f64; 3]>,
    s_at: Option<[f64; 3]>,
    // as_he_is: what he and the hold are as the step begins
    at: usize,
    at_b: usize,
    at_c: usize,
    at_aim: usize,
    live: bool,
    gun_v: bool,
    no_knife: bool,
    t: f64,
    low: f64,
    haul: bool,
    yank: Option<f64>,
    step: Option<f64>,
    pull: Option<f64>,
    grab: bool,
    held: bool,
    choke: bool,
    whip: bool,
    idle_t: f64,
    strip: Option<f64>,
    tw_y: f64,
    tw_p: f64,
    gun_out: f64,
    gun_pitch: f64,
    shoot_t: f64,
    rd_hand: Option<[f64; 3]>,
    fix_l: [f64; 4],
    fix_r: [f64; 4],
    grip_up: [f64; 3],
    // reach_and_choke: how far the reach, the elbow's closing, the whip, the disarm, the lowering and the haul have come
    gv: f64,
    ug: f64,
    cs_on: bool,
    cr: bool,
    ck: f64,
    kn: f64,
    fight: f64,
    wnd: f64,
    stk: f64,
    thru: f64,
    s_t: f64,
    s_arm: bool,
    e_body: f64,
    lck: f64,
    lo: f64,
    lw: f64,
    hu: f64,
    hauled: bool,
    // yank_and_seat: how much the yank, the blow he takes, the lunge's dip, the sway, the sitting back and the lag of his feet show
    yk: f64,
    hb_s: f64,
    dip_w: f64,
    sw_s: f64,
    seat_f: f64,
    seat: f64,
    ps: f64,
    e_sl: f64,
    e_sr: f64,
    lag_on: bool,
    leg_w: f64,
    legs: bool,
    // lean_and_turn: how the trunk leans and turns, and where his two hands have the man
    lean: f64,
    brace: f64,
    side: f64,
    yaw_w: f64,
    hy: f64,
    hp: f64,
    hold_l: [f64; 3],
    hold_r: [f64; 3],
    e_l: f64,
    e_r: f64,
    xg: f64,
    back_k: f64,
    back_g: f64,
    // hands_go: where each hand is bound: round the outside of him and on, the left first
    hl: usize,
    hr: usize,
    live_r: [f64; 3],
    r_l0: [f64; 4],
    r_r0: [f64; 4],
    tgt_l: [f64; 3],
    tgt_r: [f64; 3],
    // knife_fist: the knife fist: the blade kept ahead round the outside of him, turned across only as it comes in over the shoulder
    uar: usize,
    far: usize,
    rot_r: [f64; 4],
    pole_k: [f64; 3],
    base_r: [f64; 3],
}

/// Sam holding a man from behind: the legs sat, sunk and stepped under him, the trunk over the man's back, then both arms by IK on a copy (the left
/// across his throat, the right the knife's fist or, on a scratch copy cross-faded in, the pistol at his head).
pub fn hold_layer(g: usize, an: &mut Anim, w: f64, dt: f64, h: &HoldPose, beats: bool, man: Option<[f64; 3]>, s_at: Option<[f64; 3]>, tune: &Knobs) -> u32 {
    let mut f = Holding { w, dt, beats, man, s_at, ..Holding::default() };
    as_he_is(an, h, &mut f);
    // (read before anything is posed: an answer of 0 leaves the pose as it was handed)
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    // ---- the clocks and shares: nothing here reads the pose
    reach_and_choke(an, tune, &mut f);
    yank_and_seat(an, tune, &mut f);
    lean_and_turn(tune, &mut f);
    // ---- the legs under him, then body English straight into the pose
    let Some(Some(n)) = with_pose(g, f.at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD, MIDDLE1_R])) {
            return None;
        }
        hold_legs(tune, d, rig, fk, &f);
        hold_trunk(tune, d, rig, &f);
        Some(n)
    }) else {
        return 0;
    };
    // ---- arms on a copy, against the body as it now is
    if !ask_blend(g, f.at, f.at, 1.0, false, f.at_b, n) {
        return 0;
    }
    let (hh, from, caught, fix) = (&mut hs, &mut an.hd_from, &mut an.hd_caught, &mut an.hd_fix);
    let Some(()) = with_pose(g, f.at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        hands_go(tune, fk, from, &mut f);
        // the left's fingers run on across the throat and hook back round it, palm to the neck
        left_arm(tune, d, rig, fk, hh, caught, fix, &f);
        if f.xg < 1.0 {
            // ---- the knife fist: the blade kept pointing ahead round the outside of him, turned across only as it comes in over the shoulder
            knife_fist(tune, &mut f);
            if f.s_arm {
                // THE DISARM: one path for the wrist, keyed in seconds: onto the gun in his hand, torn out and down to Sam's hip, let go, then up to its station
                disarm_arm(tune, d, rig, fk, hh, &f);
            } else {
                // elbow out to the right and up, over the man's shoulder, for the throat; down and back once the fist is on the shoulder
                two_bone_ik(rig, d, fk.rot, fk.pos, f.uar, f.far, f.hr, &f.base_r, Some(&f.pole_k), 0.9 * max(f.e_r, 0.3), Some(&f.rot_r), false);
                hand(d, hh, 1, &[0.62, 0.68, 0.72, 0.76], 0.6, 0.0);
            }
        }
    }) else {
        return 0;
    };
    if f.xg > 0.0 {
        // ---- the gun hand: the right arm re-seeded from the aim clip, one rigid turn of that grip onto the wanted bore, the wrist to its station;
        //      on a scratch copy while the knife fist still has a share of the arm
        let p = if f.xg < 1.0 { f.at_c } else { f.at_b };
        if p != f.at_b && !ask_blend(g, f.at_b, f.at_b, 1.0, false, p, n) {
            return 0;
        }
        // (the clavicle stays the body's)
        ask_mask(&[(role(SHOULDER_R), 1.0, true), (role(SHOULDER_R), 0.0, false)]);
        if !ask_blend(g, p, f.at_aim, 1.0, true, p, n) {
            return 0;
        }
        with_pose(g, p, |d, rig, _n, fk| {
            fk_run(rig, d, fk.rot, fk.pos);
            gun_arm(tune, d, rig, fk, &f);
        });
        if p != f.at_b {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, f.at_b, p, f.xg, true, f.at_b, n) {
                return 0;
            }
        }
    }
    arms_mask();
    if !ask_blend(g, f.at, f.at_b, w, true, f.at, n) {
        return 0;
    }
    if !left_arm_first(g, n, &f) {
        return 0;
    }
    if !disarm_first(g, n, tune, &f) {
        return 0;
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}

/// What he and the hold are as the step begins.
fn as_he_is(an: &mut Anim, h: &HoldPose, f: &mut Holding) {
    let (at, at_b, _) = poses(an);
    let (at_c, at_aim, live, gun_v, no_knife) = (an.p_c.at as usize, an.p_aim.at as usize, an.hold_pose.is_some(), h.variant == Some(HoldVariant::Gun), an.no_knife);
    let (ph, t, low, haul, yank, step, pull) = (h.phase, h.t, h.low.unwrap_or(0.0), h.haul.unwrap_or(false), h.yank, h.step, h.pull);
    let (grab, held, choke, whip) = (ph == HoldPhase::Grab, ph == HoldPhase::Held, ph == HoldPhase::Choke, ph == HoldPhase::Whip);
    let (idle_t, strip, tw_y, tw_p) = (an.idle_t, an.hold_pose.and_then(|h| h.strip).map(|s| s.t), an.aim_yaw_offset, -an.aim_pitch * an.upper_w.aim * 0.35);
    let (gun_out, gun_pitch, shoot_t, rd_hand) = (h.gun_out.unwrap_or(0.0), h.gun_pitch.unwrap_or(0.0), an.shoot_t, an.rd_hand_l);
    let (fix_l, fix_r, grip_up) = (an.hand_fix[0], an.hand_fix[1], an.grip_up);
    f.at = at;
    f.at_b = at_b;
    f.at_c = at_c;
    f.at_aim = at_aim;
    f.live = live;
    f.gun_v = gun_v;
    f.no_knife = no_knife;
    f.t = t;
    f.low = low;
    f.haul = haul;
    f.yank = yank;
    f.step = step;
    f.pull = pull;
    f.grab = grab;
    f.held = held;
    f.choke = choke;
    f.whip = whip;
    f.idle_t = idle_t;
    f.strip = strip;
    f.tw_y = tw_y;
    f.tw_p = tw_p;
    f.gun_out = gun_out;
    f.gun_pitch = gun_pitch;
    f.shoot_t = shoot_t;
    f.rd_hand = rd_hand;
    f.fix_l = fix_l;
    f.fix_r = fix_r;
    f.grip_up = grip_up;
}

/// How far the reach, the elbow's closing, the whip, the disarm, the lowering and the haul have come.
fn reach_and_choke(an: &mut Anim, tune: &Knobs, f: &mut Holding) {
    let Holding { w, dt, beats, live, gun_v, no_knife, t, low, haul, grab, choke, whip, strip, .. } = *f;
    let gv = clamp(an.hd_gv.x, 0.0, 1.0);
    // the reach, 0 → 1
    let ug = if grab { clamp(t / tune.hold.secs.grab, 0.0, 1.0) } else { 1.0 };
    let cs_on = no_knife;
    let cr = cs_on && !gun_v && grab;
    // the elbow closing
    let ck = if choke && !gun_v { smoothstep(0.0, tune.hold.secs.choke * 0.75, t) } else { 0.0 };
    spring_step(&mut an.hd_kn, if choke { 1.0 } else { 0.0 }, if choke { 6.0 / tune.hold.knife.off_secs } else { 3.0 / tune.hold.knife.off_secs }, dt);
    let kn_e = clamp(an.hd_kn.x, 0.0, 1.0);
    let kn = if no_knife { 1.0 } else { kn_e };
    let fight = kn_e * if choke { 1.0 - smoothstep(tune.hold.struggle.fade_from, tune.hold.struggle.fade_to, t / tune.hold.secs.choke) } else { 1.0 };
    let wnd = if whip { smoothstep(0.0, WHIP_WIND, t) } else { 0.0 };
    let stk = if !whip || t < WHIP_STRIKE { 0.0 } else { hermite(t, WHIP_STRIKE, 0.0, 0.0, tune.hold.secs.whip, 1.0, 7.0) };
    let thru = if whip { smoothstep(tune.hold.secs.whip, tune.hold.secs.whip + WHIP_THRU, t) } else { 0.0 };
    let s_t = strip.unwrap_or(0.0);
    let s_arm = strip.is_some() && !gun_v && s_t < tune.hold.strip.home;
    let e_body = smoothstep(0.15, 0.9, ug);
    spring_step(&mut an.hd_ck, if choke && !gun_v { clamp(t / tune.hold.secs.choke, 0.0, 1.0) } else { 0.0 }, if choke { 30.0 } else { 9.0 }, dt);
    let lck = clamp(an.hd_ck.x, 0.0, 1.0);
    let lo = lower_ease(&mut an.hd_lo, if low > 0.0 { clamp(low, 0.0, 1.0) * smoothstep(lower_k(beats, tune.hold.beats.lower.from, tune.hold.lower.from), lower_k(beats, tune.hold.beats.lower.by, tune.hold.lower.by), lck) } else { 0.0 }, lower_k(beats, tune.hold.beats.lower.up, tune.hold.lower.up), dt) * (1.0 - gv);
    let lw = if live { 1.0 } else { w };
    let hu = haul_ease(&mut an.hd_hu, haul && choke && !gun_v, smoothstep(tune.hold.haul.in_[0], tune.hold.haul.in_[1], lck) * (1.0 - smoothstep(tune.hold.haul.out[0], tune.hold.haul.out[1], lck)), tune.hold.haul.down, dt) * (1.0 - gv) * lw;
    let hauled = hu > 1e-4;
    f.gv = gv;
    f.ug = ug;
    f.cs_on = cs_on;
    f.cr = cr;
    f.ck = ck;
    f.kn = kn;
    f.fight = fight;
    f.wnd = wnd;
    f.stk = stk;
    f.thru = thru;
    f.s_t = s_t;
    f.s_arm = s_arm;
    f.e_body = e_body;
    f.lck = lck;
    f.lo = lo;
    f.lw = lw;
    f.hu = hu;
    f.hauled = hauled;
}

/// How much the yank, the blow he takes, the lunge's dip, the sway, the sitting back and the lag of his feet show.
fn yank_and_seat(an: &mut Anim, tune: &Knobs, f: &mut Holding) {
    let Holding { w, dt, live, no_knife, t, yank, step, pull, grab, held, gv, ug, cs_on, lo, hu, .. } = *f;
    let yk = match yank {
        Some(y) if live => bump(y, 0.0, tune.hold.yank.in_, tune.hold.yank.in_ + tune.hold.yank.hold, tune.hold.yank.in_ + tune.hold.yank.hold + tune.hold.yank.out),
        _ => 0.0,
    };
    let hb_s = match yank {
        Some(y) if cs_on => bump(y, tune.hold.hit.t[0], tune.hold.hit.t[1], tune.hold.hit.t[2], tune.hold.hit.t[3]) * (1.0 - gv) * w,
        _ => 0.0,
    };
    let dip_w = match step {
        Some(s) if live && grab => bump(s, 0.0, tune.hold.lunge.t[0], tune.hold.lunge.t[0] + tune.hold.lunge.t[1], tune.hold.lunge.t[0] + tune.hold.lunge.t[1] + tune.hold.lunge.t[2]),
        _ => 0.0,
    };
    let sw_live = no_knife && live && held;
    an.hd_sw = if sw_live { sin(t / tune.hold.sway.secs * 2.0 * PI) * smoothstep(0.0, tune.hold.sway.in_, t) } else { damp(an.hd_sw, 0.0, tune.hold.sway.out, dt) };
    let sw_s = an.hd_sw * (1.0 - gv) * w;
    let sw_on = tune.hold.sway.m != 0.0 && (1.0 - gv) * w > 1e-4 && if sw_live { smoothstep(0.0, tune.hold.sway.in_, t) > 1e-4 } else { abs(an.hd_sw) > 1e-4 };
    let seat_f = if cs_on { (if grab { smoothstep(0.55, 1.0, ug) } else { 1.0 }) * (1.0 - gv) * w } else { 0.0 };
    let seat = seat_f * (1.0 - clamp(lo, 0.0, 1.0)) * (1.0 - clamp(hu, 0.0, 1.0));
    let ps = match (pull, yank) {
        (Some(p), Some(_)) => p * w,
        _ => 0.0,
    };
    let lag_s = |y0: f64, y1: f64| if ps > 1e-4 { smoothstep(y0, y1, yank.unwrap_or(0.0)) } else { 1.0 };
    let (e_sl, e_sr) = (lag_s(tune.hold.catch.sam_feet[0][0], tune.hold.catch.sam_feet[0][1]), lag_s(tune.hold.catch.sam_feet[1][0], tune.hold.catch.sam_feet[1][1]));
    let lag_on = ps * (1.0 - min(e_sl, e_sr)) > 1e-4;
    let leg_w = if cs_on {
        lerp(0.6 * clamp(8.0 * max(max(max(max(max(max(seat_f, lo), yk), dip_w), hb_s), if sw_on { 1.0 } else { 0.0 }), 20.0 * ps * (1.0 - min(e_sl, e_sr))), 0.0, 1.0), 0.6, gv)
    } else {
        0.6
    };
    let legs = lo > 1e-4 || yk > 1e-4 || ((tune.hold.lunge.dip != 0.0 || tune.hold.lunge.back != 0.0) && dip_w > 1e-4) || sw_on || seat_f > 1e-4 || lag_on || hb_s > 1e-4;
    (f.yk, f.hb_s, f.dip_w, f.sw_s, f.seat_f, f.seat, f.ps, f.e_sl, f.e_sr, f.lag_on, f.leg_w, f.legs) = (yk, hb_s, dip_w, sw_s, seat_f, seat, ps, e_sl, e_sr, lag_on, leg_w, legs);
}

/// How the trunk leans and turns, and where his two hands have the man.
fn lean_and_turn(tune: &Knobs, f: &mut Holding) {
    let Holding { w, beats, gun_v, no_knife, t, low, grab, choke, idle_t, gv, ug, ck, kn, fight, wnd, stk, thru, s_t, s_arm, e_body, lck, lo, lw, hu, hauled, hb_s, seat, .. } = *f;
    let ck_e = if low > 0.0 { max(ck, smoothstep(0.0, 0.75, lck)) } else { ck };
    let lean0 = lerp(lerp(tune.hold.lean, tune.hold.gun.lean, gv), tune.hold.choke_lean, ck_e) * DEGR * w * e_body;
    let lean1 = if lo > 1e-4 { lean0 + lower_k(beats, tune.hold.beats.lower.sam_lean, tune.hold.lower.sam_lean) * lo * DEGR * lw } else { lean0 };
    let lean = (if hauled { lean1 + tune.hold.haul.sam_lean * hu * DEGR } else { lean1 }) + tune.hold.catch.seat_lean * seat * DEGR - tune.hold.hit.heave * hb_s * DEGR;
    let brace = tune.hold.brace * sin(idle_t * 2.0 * PI * tune.hold.struggle.hz[2] + 0.7) * fight * DEGR * w;
    let side = (lerp(tune.hold.knife.side, tune.hold.gun.side, gv) - if no_knife { tune.hold.apart.side * (1.0 - gv) } else { 0.0 }) * DEGR * w * e_body + brace;
    let yaw_w = (tune.hold.gun.wind_yaw * wnd * (1.0 - stk) + tune.hold.gun.hit_yaw * (stk - thru)) * DEGR * w
        + if s_arm { tune.hold.strip.yaw * bump(s_t, tune.hold.strip.take - 0.05, tune.hold.strip.out - 0.02, tune.hold.strip.out + 0.06, tune.hold.strip.drop + 0.14) * DEGR * w } else { 0.0 };
    let hy = lerp(tune.hold.knife.head_yaw, tune.hold.gun.head_yaw, gv) * DEGR * w * e_body - yaw_w;
    let hp = (lerp(tune.hold.head_pitch, tune.hold.gun.head_pitch, gv) * DEGR - lean) * w * e_body + if lo > 1e-4 { lower_k(beats, tune.hold.beats.lower.sam_look, tune.hold.lower.sam_look) * lower_k(beats, tune.hold.beats.lower.sam_lean, tune.hold.lower.sam_lean) * lo * DEGR * lw } else { 0.0 } + 0.0;
    let limp0 = if choke && !gun_v { smoothstep(0.62 * tune.hold.secs.choke, 0.95 * tune.hold.secs.choke, t) } else { 0.0 };
    let limp = if low > 0.0 { max(limp0, smoothstep(0.62, 0.95, lck)) } else { limp0 };
    let mut give = if lo > 1e-4 {
        [0.0, -(tune.hold.sag[1] - tune.hold.sag[0]) * limp - lower_k(beats, tune.hold.beats.lower.sam_arm[0], tune.hold.lower.sam_arm[0]) * lo, 0.025 * limp + lower_k(beats, tune.hold.beats.lower.sam_arm[1], tune.hold.lower.sam_arm[1]) * lo]
    } else {
        [0.0, -(tune.hold.sag[1] - tune.hold.sag[0]) * limp, 0.025 * limp]
    };
    if hauled {
        give[1] += tune.hold.haul.sam_arm[0] * hu;
        give[2] -= tune.hold.haul.sam_arm[1] * hu;
    }
    if seat > 1e-4 {
        give[1] -= tune.hold.catch.sink * seat / max(w, 1e-3);
    }
    if hb_s > 1e-4 {
        give[1] -= tune.hold.hit.dip * hb_s / max(w, 1e-3);
    }
    let (hold_l, hold_r) = (v3::add(v3::lerp3(tune.hold.throat, tune.hold.tight, ck_e), give), v3::add(v3::lerp3(tune.hold.knife.hand, tune.hold.knife.off_hand, kn), give));
    let (e_l, e_r) = (if grab { smoothstep(0.1, 0.7, ug) } else { 1.0 }, if grab { smoothstep(0.3, 0.95, ug) } else { 1.0 });
    let xg = smoothstep(0.3, 0.7, gv);
    let (back_k, back_g) = (if grab { 0.0 } else { smoothstep(0.0, 0.55, gv) }, if grab { 0.0 } else { 1.0 - smoothstep(0.45, 1.0, gv) });
    (f.lean, f.brace, f.side, f.yaw_w, f.hy, f.hp, f.hold_l, f.hold_r, f.e_l, f.e_r, f.xg, f.back_k, f.back_g) = (lean, brace, side, yaw_w, hy, hp, hold_l, hold_r, e_l, e_r, xg, back_k, back_g);
}

/// The legs under him: the hips moved, and both feet put back where they stood or stepped.
fn hold_legs(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, f: &Holding) {
    let Holding { w, beats, grab, ug, cs_on, lo, lw, yk, hb_s, dip_w, sw_s, seat_f, seat, ps, e_sl, e_sr, lag_on, leg_w, legs, .. } = *f;
    if legs {
        fk_run(rig, d, fk.rot, fk.pos);
        let (f_l, f_r, r_l, r_r) = (fk.at(role(FOOT_L)), fk.at(role(FOOT_R)), fk.rot(role(FOOT_L)), fk.rot(role(FOOT_R)));
        let off = [
            tune.hold.sway.m * sw_s + tune.hold.catch.seat[2] * seat,
            -lower_k(beats, tune.hold.beats.lower.sam_sink, tune.hold.lower.sam_sink) * lo * lw - tune.hold.yank.sink * yk * w * (if cs_on { tune.hold.catch.yank_sit } else { 1.0 }) - tune.hold.lunge.dip * dip_w - tune.hold.catch.seat[1] * seat - tune.hold.hit.sam_dip * hb_s,
            lower_k(beats, tune.hold.beats.lower.sam_fwd, tune.hold.lower.sam_fwd) * lo * lw - tune.hold.lunge.back * dip_w - tune.hold.catch.seat[0] * seat + tune.hold.hit.sam_in * hb_s,
        ];
        shift(d, rig, role(HIPS), &off);
        fk_run(rig, d, fk.rot, fk.pos);
        let pole = |r: &[f64; 4], sx: f64| {
            let kp = knee_pole(r);
            v3::normalize([kp[0] + sx * lower_k(beats, tune.hold.beats.lower.sam_knees, tune.hold.lower.sam_knees) * lo, kp[1], kp[2]])
        };
        let step_s = |f: [f64; 3], e: f64| if lag_on { [f[0], f[1] + tune.hold.catch.sam_lift * sin(PI * e) * min(1.0, ps / 0.1), f[2] + ps * (1.0 - e)] } else { f };
        let (s_l, s_r) = (step_s(f_l, e_sl), step_s(f_r, e_sr));
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_L), role(SHIN_L), role(FOOT_L), &s_l, Some(&pole(&r_l, 1.0)), leg_w, Some(&r_l), false);
        // (the rear foot back and out under the seat)
        let t_r = if seat_f > 1e-4 {
            [s_r[0] - tune.hold.catch.brace[1] * seat_f, s_r[1] + if grab { tune.hold.catch.brace_lift * sin(PI * smoothstep(0.55, 1.0, ug)) * seat_f } else { 0.0 }, s_r[2] - tune.hold.catch.brace[0] * seat_f]
        } else {
            s_r
        };
        two_bone_ik(rig, d, fk.rot, fk.pos, role(THIGH_R), role(SHIN_R), role(FOOT_R), &t_r, Some(&pole(&r_r, -1.0)), leg_w, Some(&r_r), true);
    }
}

/// The trunk over the man's back, and the head with it.
fn hold_trunk(tune: &Knobs, d: &mut [S], rig: &Rig, f: &Holding) {
    let Holding { w, gv, kn, wnd, stk, e_body, lean, brace, side, yaw_w, hy, hp, .. } = *f;
    turn(d, rig, role(SPINE1), quat::mul(q_yaw(yaw_w * 0.3), q_pitch(lean * 0.45)));
    turn(d, rig, role(SPINE2), quat::mul(q_yaw(yaw_w * 0.35), quat::mul(q_roll(side * 0.4), q_pitch(lean * 0.35))));
    turn(d, rig, role(SPINE3), quat::mul(q_yaw(yaw_w * 0.35), quat::mul(q_roll(side * 0.6), q_pitch(lean * 0.2))));
    turn(d, rig, role(NECK), quat::mul(q_roll(side * 0.8 - brace), quat::mul(q_yaw(hy * 0.4), q_pitch(hp * 0.4))));
    turn(d, rig, role(HEAD), quat::mul(q_roll(-side * 0.9), quat::mul(q_yaw(hy * 0.6), q_pitch(hp * 0.6))));
    turn(d, rig, role(SHOULDER_L), q_roll(5.0 * DEGR * w * e_body));
    turn(d, rig, role(SHOULDER_R), q_roll(-lerp(tune.hold.knife.clav * (1.0 - 0.6 * kn), tune.hold.gun.clav + 4.0 * wnd * (1.0 - stk), gv) * DEGR * w * e_body));
}

/// Where each hand is bound: round the outside of him and on, the left first.
fn hands_go(tune: &Knobs, fk: &mut Fk, from: &mut Option<[f64; 3]>, f: &mut Holding) {
    let Holding { man, grab, rd_hand, ug, cr, hold_l, hold_r, .. } = *f;
    let (hl, hr) = (role(HAND_L), role(HAND_R));
    let (live_l, live_r, r_l0, r_r0) = (fk.at(hl), fk.at(hr), fk.rot(hl), fk.rot(hr));
    let (tgt_l, tgt_r);
    if grab {
        // round the outside of him and on: the left leads, the right follows
        let (u_l, u_r) = (smoothstep(0.0, 0.75, ug), smoothstep(0.2, 1.0, ug));
        if cr {
            // over his left shoulder beside the neck and round the front of the throat: keys that ride the man, from where the reach had the hand
            let off = match man {
                Some(m) => [clamp(m[0], -0.3, 0.3), clamp(m[2] - tune.hold.dist, -0.12, 0.45)],
                None => [0.0, 0.0],
            };
            let on_him = |p: [f64; 3]| [p[0] + off[0], p[1], p[2] + off[1]];
            let from = *from.get_or_insert(rd_hand.unwrap_or(live_l));
            tgt_l = curve3(u_l, &[(0.0, from), (tune.hold.catch.at[0], on_him(tune.hold.catch.via)), (tune.hold.catch.at[1], on_him(tune.hold.catch.over)), (1.0, on_him(hold_l))]);
        } else {
            tgt_l = curve3(u_l, &[(0.0, live_l), (0.4, tune.hold.via_l), (0.72, tune.hold.over_l), (1.0, hold_l)]);
        }
        tgt_r = curve3(u_r, &[(0.0, live_r), (0.55, tune.hold.knife.via), (1.0, hold_r)]);
    } else {
        tgt_l = hold_l;
        tgt_r = hold_r;
    }
    (f.hl, f.hr, f.live_r, f.r_l0, f.r_r0, f.tgt_l, f.tgt_r) = (hl, hr, live_r, r_l0, r_r0, tgt_l, tgt_r);
}

/// The left arm across his throat, its fingers run on round it.
fn left_arm(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, caught: &mut bool, fix: &mut Option<AnimHdFix>, f: &Holding) {
    let Holding { w, live, gun_v, t, grab, held, fix_l, ug, cs_on, cr, e_l, hl, r_l0, tgt_l, .. } = *f;
    let rot_hold_l = hand_rot(&fix_l, [-1.0, 0.12, -0.4], [0.1, 0.35, 1.0]);
    let u_lc = if cr { smoothstep(0.0, 0.75, ug) } else { 1.0 };
    let rot_reach = if cr { hand_rot(&fix_l, [-0.3, 0.1, 1.0], [0.75, 0.6, -0.1]) } else { rot_hold_l };
    let lv = if live { 1.0 } else { w };
    let rot_l = if cr {
        quat::nlerp(quat::nlerp(r_l0, rot_reach, smoothstep(0.0, tune.hold.catch.arm_in, ug)), rot_hold_l, smoothstep(tune.hold.catch.at[0], 0.95, u_lc))
    } else {
        quat::nlerp(r_l0, rot_hold_l, e_l)
    };
    let (ual, fal) = (role(UPPER_ARM_L), role(FOREARM_L));
    if cr {
        // (the elbow under the wrist and close to him until the hand is round the front of the throat, then up onto his collarbone)
        let pole = v3::lerp3([0.3, -0.92, 0.15], [0.55, 0.25, 0.85], smoothstep(0.62, 1.0, u_lc));
        let a_in = smoothstep(0.0, tune.hold.catch.arm_in, ug);
        two_bone_ik(rig, d, fk.rot, fk.pos, ual, fal, hl, &tgt_l, Some(&pole), 0.95 * a_in, Some(&rot_l), false);
        *caught = true;
        *fix = Some(AnimHdFix { tgt: tgt_l, pole, pole_w: 0.95 * a_in, rot: quat::nlerp(rot_reach, rot_hold_l, smoothstep(tune.hold.catch.at[0], 0.95, u_lc)), w: smoothstep(0.0, 0.1, ug) * lv, rw: a_in * lv });
    } else {
        if *caught && cs_on && !gun_v && held && live && t < tune.hold.catch.settle {
            let f = 1.0 - smoothstep(0.0, tune.hold.catch.settle, t);
            *fix = Some(AnimHdFix { tgt: tgt_l, pole: [0.55, 0.25, 0.85], pole_w: 0.95, rot: rot_hold_l, w: f, rw: f });
        } else if !grab {
            *caught = false;
        }
        // elbow forward and a little left, up on his left collarbone
        let pole = [0.55, 0.25, 0.85];
        two_bone_ik(rig, d, fk.rot, fk.pos, ual, fal, hl, &tgt_l, Some(&pole), 0.95 * e_l, Some(&rot_l), false);
    }
    // fingers close as the hands arrive
    let open = 1.0 - if grab { smoothstep(0.5, 1.0, ug) } else { 1.0 };
    let cl = |x: f64| lerp(x, 0.08, open);
    hand(d, hh, 0, &[cl(0.5), cl(0.58), cl(0.64), cl(0.68)], lerp(0.55, 0.1, open), open * 0.4);
}

/// The knife fist: the blade kept ahead round the outside of him, turned across only as it comes in over the shoulder.
fn knife_fist(tune: &Knobs, f: &mut Holding) {
    let Holding { grab, fix_r, ug, kn, back_k, r_r0, tgt_r, .. } = *f;
    let (uar, far) = (role(UPPER_ARM_R), role(FOREARM_R));
    let e_rk = if grab { smoothstep(0.5, 1.0, ug) } else { 1.0 };
    let rot_k = quat::nlerp(knife_rot(&fix_r, tune.hold.knife.blade, tune.hold.knife.back), knife_rot(&fix_r, tune.hold.knife.off_blade, tune.hold.knife.off_back), kn);
    let rot_r = quat::nlerp(r_r0, rot_k, e_rk);
    let pole_k = v3::lerp3([-0.9, 0.15, -0.4], [-0.8, -0.55, -0.3], kn);
    let base_r = v3::lerp3(tgt_r, tune.hold.knife.via, back_k);
    (f.uar, f.far, f.rot_r, f.pole_k, f.base_r) = (uar, far, rot_r, pole_k, base_r);
}

/// The disarm: one path for the wrist, keyed in seconds: onto the gun in his hand, torn out and down to the hip, let go, then up to its station.
fn disarm_arm(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, f: &Holding) {
    let Holding { s_at, tw_y, tw_p, fix_r, s_t, e_r, hr, live_r, r_r0, uar, far, rot_r, pole_k, base_r, .. } = *f;
    let (take, out, drop, home, follow) = (tune.hold.strip.take, tune.hold.strip.out, tune.hold.strip.drop, tune.hold.strip.home, tune.hold.strip.follow);
    let m = v3::add(
        match s_at {
            Some(sa) => untwist(fk, tw_y, tw_p, sa),
            None => [tune.hold.strip.meet[0], tune.hold.strip.meet[1], tune.hold.strip.meet[2] + tune.hold.dist],
        },
        tune.hold.strip.over,
    );
    let back = lerp(drop, home, 0.45);
    let p = curve3(
        strip_in(s_t, take),
        &[
            (0.0, live_r),
            (take * 0.55, [lerp(live_r[0], m[0], 0.5) - 0.07, lerp(live_r[1], m[1], 0.6), lerp(live_r[2], m[2], 0.45)]),
            (take, m),
            (out, v3::add(m, tune.hold.strip.wrench)),
            (drop, tune.hold.strip.low),
            (drop + follow, tune.hold.strip.low),
            (max(back, drop + follow + 0.06), tune.hold.knife.via),
            (home, base_r),
        ],
    );
    let (u1, u2, u3, u4) = (smoothstep(take * 0.3, take, s_t), smoothstep(take, out, s_t), smoothstep(out, drop, s_t), smoothstep(drop + 0.08, home - 0.06, s_t));
    // over the top of the gun, fingers round it · rolled out, the back of the hand to the right · hanging at the thigh
    let (r_grip, r_tear, r_low) = (hand_rot(&fix_r, [0.3, -0.5, 0.8], [-0.25, 0.9, 0.3]), hand_rot(&fix_r, [-0.2, -0.8, 0.55], [-0.9, 0.25, 0.3]), hand_rot(&fix_r, [0.05, -1.0, 0.15], [-1.0, 0.0, 0.1]));
    let grip = smoothstep(take - 0.05, take + 0.02, s_t) * (1.0 - smoothstep(drop, drop + 0.07, s_t));
    let fist = smoothstep(tune.hold.strip.knife - 0.12, tune.hold.strip.knife, s_t);
    let cu = |x: f64| lerp(lerp(0.14, 0.7, grip), x, fist);
    let rot = quat::nlerp(quat::nlerp(quat::nlerp(quat::nlerp(r_r0, r_grip, u1), r_tear, u2), r_low, u3), rot_r, u4);
    two_bone_ik(rig, d, fk.rot, fk.pos, uar, far, hr, &p, Some(&v3::lerp3([-0.95, -0.3, -0.2], pole_k, u4)), lerp(0.95 * smoothstep(0.0, 0.1, s_t), 0.9 * max(e_r, 0.3), u4), Some(&rot), false);
    hand(d, hh, 1, &[cu(0.62), cu(0.68), cu(0.72), cu(0.76)], lerp(lerp(0.15, 0.5, grip), 0.6, fist), 0.25 * (1.0 - grip) * (1.0 - fist));
}

/// The gun hand: the aim clip's grip turned whole onto the wanted bore, the wrist to its station.
fn gun_arm(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, f: &Holding) {
    let Holding { s_at, gun_v, t, grab, whip, tw_y, tw_p, gun_out, gun_pitch, shoot_t, grip_up, ug, wnd, stk, thru, back_g, live_r, r_r0, .. } = *f;
    let hr = role(HAND_R);
    let (p_r0, r_aim) = (fk.at(hr), fk.rot(hr));
    // the clip's bore as posed on this body, and where the carry underneath actually has it pointing
    let g0 = v3::normalize(v3::sub(fk.at(role(MIDDLE1_R)), p_r0));
    let g_live = v3::normalize(quat::rotate(quat::mul(r_r0, quat::conj(r_aim)), g0));
    let (out, pitch) = (clamp(gun_out, 0.0, 1.0) * (1.0 - wnd), gun_pitch);
    // recoil: up in ~35 ms, back down over ~0.2 s
    let kick = if shoot_t >= 0.0 { bump(shoot_t, 0.0, 0.035, 0.06, 0.24) } else { 0.0 };
    let rest = v3::lerp3(v3::lerp3(tune.hold.gun.hand, tune.hold.gun.out_hand, out), tune.hold.gun.via_b, back_g);
    let bore_rest = v3::normalize(v3::lerp3(v3::lerp3(tune.hold.gun.bore, [0.0, sin(pitch), cos(pitch)], out), tune.hold.gun.via_bore, back_g));
    let (mut hand_at, mut bore);
    if grab {
        let u_r = smoothstep(0.1, 1.0, ug);
        hand_at = match s_at {
            // (the disarm, the pistol hold's: the frame comes down on the man's gun wrist on its way round and up to his head)
            Some(sa) if gun_v => {
                let (rap, gs, take) = (v3::add(untwist(fk, tw_y, tw_p, sa), tune.hold.strip.rap), tune.hold.secs.grab, tune.hold.strip.take);
                curve3(strip_in(t, take), &[(0.0, live_r), (take, rap), (lerp(take, gs, 0.35), tune.hold.gun.via_a), (lerp(take, gs, 0.7), tune.hold.gun.via_b), (gs, rest)])
            }
            _ => curve3(u_r, &[(0.0, live_r), (0.38, tune.hold.gun.via_a), (0.7, tune.hold.gun.via_b), (1.0, rest)]),
        };
        bore = v3::normalize(curve3(u_r, &[(0.0, g_live), (0.5, tune.hold.gun.via_bore), (1.0, bore_rest)]));
    } else if whip {
        hand_at = v3::lerp3(v3::lerp3(v3::lerp3(rest, tune.hold.gun.wind_hand, wnd), tune.hold.gun.hit_hand, stk), tune.hold.gun.thru_hand, thru);
        bore = v3::normalize(v3::lerp3(v3::lerp3(v3::lerp3(bore_rest, tune.hold.gun.wind_bore, wnd), tune.hold.gun.hit_bore, stk), tune.hold.gun.bore, thru));
    } else {
        hand_at = rest;
        bore = bore_rest;
    }
    if kick > 0.0 {
        // the muzzle flips up about the wrist and the hand comes back along the bore
        let up_k = sin(tune.hold.gun.kick_pitch * DEGR * kick);
        bore = v3::normalize([bore[0], bore[1] + up_k, bore[2]]);
        hand_at = v3::mad(v3::add(hand_at, [0.0, 0.01 * kick, 0.0]), bore, -tune.hold.gun.kick_back * kick);
    }
    let turned = grip_turn(&g0, &r_aim, &bore, &AY, &grip_up);
    // (the first frames of the reach: still mostly the carry's own hand)
    let e_g = if grab { smoothstep(0.05, 0.6, ug) } else { 1.0 };
    let rot_r = quat::nlerp(r_r0, quat::mul(turned, r_aim), e_g);
    // elbow out to the right and a little down; up and back as it cocks
    let pole = v3::lerp3([-1.0, -0.35, -0.45], [-0.7, 0.2, -0.7], wnd * (1.0 - stk));
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hr, &hand_at, Some(&pole), 0.9 * max(e_g, 0.3), Some(&rot_r), false);
}

/// The left arm is the hold's own before the layer's weight has come in: its path starts where the reach had the hand.
fn left_arm_first(g: usize, n: usize, f: &Holding) -> bool {
    let Holding { w, at, at_b, live, ug, cr, .. } = *f;
    if cr && live && w < 1.0 {
        // (the left arm is the hold's own before the layer's weight has come in: its path starts where the reach had the hand)
        let w_l = max(w, smoothstep(0.0, 0.1, ug));
        if w_l > w {
            ask_mask(&[(role(SHOULDER_L), 1.0, true)]);
            if !ask_blend(g, at, at_b, (w_l - w) / (1.0 - w), true, at, n) {
                return false;
            }
        }
    }
    true
}

/// The disarm: the hand is on the gun at `take`, before the layer's own weight has come in.
fn disarm_first(g: usize, n: usize, tune: &Knobs, f: &Holding) -> bool {
    let Holding { w, at, at_b, strip, s_t, .. } = *f;
    if strip.is_some() && s_t < tune.hold.strip.out && w < 1.0 {
        // (the disarm: the hand is on the gun at `take`, before the layer's own weight has come in)
        let w_s = max(w, smoothstep(0.0, 0.8 * tune.hold.strip.take, s_t));
        if w_s > w {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at, at_b, (w_s - w) / (1.0 - w), true, at, n) {
                return false;
            }
        }
    }
    true
}
/// CharacterAnimator.catchFix's arm: the catch's left hand put back where the hold's own arm had it, after the layers that still move his trunk.
pub fn catch_arm(g: usize, an: &Anim, x: &AnimHdFix) -> u32 {
    with_pose(g, an.p_a.at as usize, |d, rig, n, fk| {
        if !cast(n, &ARM_ROLES) {
            return 0;
        }
        fk_run(rig, d, fk.rot, fk.pos);
        let hl = role(HAND_L);
        let rot = quat::nlerp(fk.rot(hl), x.rot, x.rw);
        two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hl, &v3::lerp3(fk.at(hl), x.tgt, x.w), Some(&x.pole), x.pole_w, Some(&rot), false);
        RAN | RAN_ALL
    })
    .unwrap_or(0)
}
/// what the held man's two arms do: each wrist's spot, turn and elbow pole, and each hand's fingers
#[derive(Clone, Copy, Default)]
struct ArmSet {
    tgt: [[f64; 3]; 2],
    rot: [[f64; 4]; 2],
    pole: [[f64; 3]; 2],
    curl: [[f64; 4]; 2],
    thumb: [f64; 2],
    spread: [f64; 2],
}
/// What the steps of being held hand on to the ones after them: his clocks, and how much of each thing he does shows.
#[derive(Clone, Copy, Default)]
struct BeingHeld {
    w: f64,
    dt: f64,
    beats: bool,
    // as_held: what he and the hold are as the step begins
    at: usize,
    at_b: usize,
    gun: bool,
    bare: bool,
    live: bool,
    t: f64,
    grab: bool,
    held: bool,
    choke: bool,
    whip: bool,
    ag: f64,
    low: f64,
    haul: bool,
    yank: Option<f64>,
    pull: f64,
    shot_in: Option<f64>,
    armed: bool,
    tt: f64,
    strip: Option<f64>,
    fix_l: [f64; 4],
    fix_r: [f64; 4],
    // struggle: how far the reach and the choke have come, and how he fights, is jolted, shot at, knocked and breathes
    tau: f64,
    gv: f64,
    ug: f64,
    ck: f64,
    limp: f64,
    e_in: f64,
    cs_on: bool,
    cr: bool,
    jl: f64,
    hb: f64,
    shot: f64,
    knock: f64,
    amp: f64,
    jerk: f64,
    fgt: f64,
    s1: f64,
    s2: f64,
    s3: f64,
    roll: f64,
    yaw_s: f64,
    br: f64,
    // let_down: how far he is hauled, sunk, swayed, lowered and tipped
    lv: f64,
    hu: f64,
    hauled: bool,
    drop: f64,
    sw: f64,
    lo0: f64,
    lo: f64,
    b_off: f64,
    b_in: f64,
    tip_u: f64,
    b_tip: f64,
    // sat_back: how far he is sat back into the arm, and how far each foot has been left behind
    limp_a: f64,
    sit: f64,
    off: f64,
    off_l: [f64; 3],
    off_r: [f64; 3],
    off_on: bool,
    // bend: how his trunk, shoulders and head bend, and where his hands would hang
    arch: f64,
    heave: f64,
    clav: f64,
    chin1: f64,
    head_roll: f64,
    hyw: f64,
    h0: [f64; 3],
    h1: [f64; 3],
    hang_l: [f64; 3],
    hang_r: [f64; 3],
    // arm_sets: the two sets of the arms, or the two blended for the beat of a change; and where the disarm has got to
    z: ArmSet,
    s_t: f64,
    s_on: bool,
    e_a: f64,
    kept: bool,
    // held_hips: the pelvis sunk, hauled, swayed, sat and lowered; his feet noted first
    sp1: usize,
    sp2: usize,
    sp3: usize,
    f_l: [f64; 3],
    f_r: [f64; 3],
    r_l: [f64; 4],
    r_r: [f64; 4],
    hit_rec: f64,
    // held_hands_go: where each hand is bound: flung out and onto the arm, up, or after the gun torn out of it
    hl: usize,
    hr: usize,
    r_l0: [f64; 4],
    r_r0: [f64; 4],
    tgt_l: [f64; 3],
    tgt_r: [f64; 3],
    // arm_share: how much of the arms the layer has (the first frames of the grab: mostly still his own; the catch's fling has them before the layer's weight is in)
    w_a: f64,
}

/// The held man: the pelvis sunk, hauled, sat and lowered over feet put back where they stood (or stepped after him), the spine arched onto the arm with
/// his struggle, his breathing and the choke's fight on it, the head pulled back and lolling; then both arms by IK on a copy (the arm hold's set, the
/// gun hold's, or the two blended for the beat of a change).
pub fn held_layer(g: usize, an: &mut Anim, w: f64, dt: f64, h: &HeldPose, beats: bool, tune: &Knobs) -> u32 {
    let mut f = BeingHeld { w, dt, beats, ..BeingHeld::default() };
    as_held(an, h, &mut f);
    // (read before anything is posed: an answer of 0 leaves the pose as it was handed)
    let Some(mut hs) = hands_of(g, an) else {
        return 0;
    };
    struggle(an, tune, &mut f);
    let_down(an, tune, &mut f);
    // his arms hang for as long as he is down
    sat_back(tune, &mut f);
    bend(tune, &mut f);
    // ---- the arms' sets: the arm hold's (gv 0) and / or the gun hold's (gv 1)
    arm_sets(tune, &mut f);
    let (he_span, he_hr) = (&mut an.he_span, &mut an.he_hr);
    // ---- body: feet noted first (put back after the pelvis sinks), then arch / hunch / head
    let Some(Some(n)) = with_pose(g, f.at, |d, rig, n, fk| {
        if !(cast(n, &ARM_ROLES) && cast(n, &LEG_ROLES) && cast(n, &[SPINE1, SPINE2, NECK, HEAD])) {
            return None;
        }
        held_hips(tune, d, rig, fk, he_span, &mut f);
        held_trunk(tune, d, rig, &f);
        held_feet(tune, d, rig, fk, &f);
        Some(n)
    }) else {
        return 0;
    };
    if !ask_blend(g, f.at, f.at, 1.0, false, f.at_b, n) {
        return 0;
    }
    let hh = &mut hs;
    if with_pose(g, f.at_b, |d, rig, _n, fk| {
        fk_run(rig, d, fk.rot, fk.pos);
        held_hands_go(tune, fk, &mut f);
        // limp, both hang palm-in
        held_arms(tune, d, rig, fk, hh, he_hr, &f);
    })
    .is_none()
    {
        return 0;
    }
    // (the first frames of the grab: mostly still his own arms; the catch's fling has them before the layer's weight is in)
    arm_share(tune, &mut f);
    arms_mask();
    if !ask_blend(g, f.at, f.at_b, f.w_a, true, f.at, n) {
        return 0;
    }
    if !disarm_held(g, n, tune, &f) {
        return 0;
    }
    hands_to(&hs, an);
    RAN | RAN_ALL
}

/// What he and the hold are as the step begins.
fn as_held(an: &mut Anim, h: &HeldPose, f: &mut BeingHeld) {
    let (at, at_b, _) = poses(an);
    let (gun, bare, live, ph, t) = (h.variant == Some(HoldVariant::Gun), h.bare.unwrap_or(false), an.held_pose.is_some(), h.phase, h.t);
    let (grab, held, choke, whip) = (ph == HoldPhase::Grab, ph == HoldPhase::Held, ph == HoldPhase::Choke, ph == HoldPhase::Whip);
    let (ag, low, haul, yank, pull, shot_in, armed) = (clamp(h.agitation, 0.0, 1.0), h.low.unwrap_or(0.0), h.haul.unwrap_or(false), h.yank, h.pull.unwrap_or(0.0), h.shot.filter(|s| *s >= 0.0), h.armed.unwrap_or(false));
    let (tt, strip, fix_l, fix_r) = (an.idle_t, an.held_pose.and_then(|h| h.strip).map(|s| s.t), an.hand_fix[0], an.hand_fix[1]);
    f.at = at;
    f.at_b = at_b;
    f.gun = gun;
    f.bare = bare;
    f.live = live;
    f.t = t;
    f.grab = grab;
    f.held = held;
    f.choke = choke;
    f.whip = whip;
    f.ag = ag;
    f.low = low;
    f.haul = haul;
    f.yank = yank;
    f.pull = pull;
    f.shot_in = shot_in;
    f.armed = armed;
    f.tt = tt;
    f.strip = strip;
    f.fix_l = fix_l;
    f.fix_r = fix_r;
}

/// How far the reach and the choke have come, and how he fights, is jolted, shot at, knocked and breathes.
fn struggle(an: &mut Anim, tune: &Knobs, f: &mut BeingHeld) {
    let BeingHeld { w, dt, gun, bare, t, grab, choke, whip, ag, yank, shot_in, tt, .. } = *f;
    let tau = 2.0 * PI;
    // 0 = the arm at his throat ↔ 1 = the gun at his head
    let gv = clamp(an.he_gv.x, 0.0, 1.0);
    let ug = if grab { clamp(t / tune.hold.secs.grab, 0.0, 1.0) } else { 1.0 };
    spring_step(&mut an.he_ck, if choke { clamp(t / tune.hold.secs.choke, 0.0, 1.0) } else { 0.0 }, if choke { 30.0 } else { 9.0 }, dt);
    let ck = clamp(an.he_ck.x, 0.0, 1.0);
    // going: the arch and the struggle give way, the head lolls, the hands fall
    let limp = smoothstep(0.62, 0.95, ck);
    let e_in = smoothstep(0.05, 0.6, ug);
    let cs_on = bare;
    let cr = cs_on && !gun && grab;
    let jl = if cr { bump(t, tune.hold.catch.jolt.t[0], tune.hold.catch.jolt.t[1], tune.hold.catch.jolt.t[2], tune.hold.catch.jolt.t[3]) * w } else { 0.0 };
    let hb = match yank {
        Some(y) if cs_on => bump(y, tune.hold.hit.t[0], tune.hold.hit.t[1], tune.hold.hit.t[2], tune.hold.hit.t[3]) * (1.0 - gv) * w,
        _ => 0.0,
    };
    let shot = match shot_in {
        Some(s) => bump(s, 0.0, 0.05, 0.12, tune.hold.gun.duck.secs) * w,
        None => 0.0,
    };
    let knock = if whip { smoothstep(tune.hold.secs.whip - 0.1, tune.hold.secs.whip, t) * w } else { 0.0 };
    // ---- restlessness, and now and then a tug at the arm
    let amp = (0.35 + 0.65 * ag) * (1.0 - limp) * (if grab { 1.4 } else { 1.0 }) * lerp(1.0, 0.55, gv) * w;
    let period = 3.0 + 3.0 * (0.5 + 0.5 * sin(floor(tt / 4.5) * 12.9898));
    let jerk = bump(tt % period, 0.0, 0.06, 0.14, 0.24) * (1.0 - limp) * w * (0.5 + 0.5 * ag);
    // ---- the choke's fight: an envelope on the eased choke amount, three sines off his own clock
    let fgt = smoothstep(0.0, tune.hold.struggle.rise, ck) * (1.0 - smoothstep(tune.hold.struggle.fade_from, tune.hold.struggle.fade_to, ck)) * (1.0 - gv) * w;
    let (s1, s2, s3) = (sin(tt * tau * tune.hold.struggle.hz[0] + 0.4), sin(tt * tau * tune.hold.struggle.hz[1] + 1.3), sin(tt * tau * tune.hold.struggle.hz[2] + 2.1));
    let roll = (4.0 * sin(tt * tau * 0.7) * amp + 5.0 * jerk + tune.hold.struggle.roll * s3 * fgt) * DEGR;
    let yaw_s = (3.0 * sin(tt * tau * 0.45 + 1.0) * amp - 7.0 * jerk + tune.hold.struggle.yaw * (0.65 * s1 + 0.35 * s2) * fgt) * DEGR;
    // ---- hard breathing while he is held
    let br = (0.5 - 0.5 * cos(tt * tau * tune.hold.breath.hz)) * (1.0 - limp) * (1.0 - 0.7 * fgt) * lerp(1.0, 0.6, gv) * w * e_in;
    f.tau = tau;
    f.gv = gv;
    f.ug = ug;
    f.ck = ck;
    f.limp = limp;
    f.e_in = e_in;
    f.cs_on = cs_on;
    f.cr = cr;
    f.jl = jl;
    f.hb = hb;
    f.shot = shot;
    f.knock = knock;
    f.amp = amp;
    f.jerk = jerk;
    f.fgt = fgt;
    f.s1 = s1;
    f.s2 = s2;
    f.s3 = s3;
    f.roll = roll;
    f.yaw_s = yaw_s;
    f.br = br;
}

/// How far he is hauled, sunk, swayed, lowered and tipped.
fn let_down(an: &mut Anim, tune: &Knobs, f: &mut BeingHeld) {
    let BeingHeld { w, dt, beats, bare, live, t, held, choke, low, haul, tt, tau, gv, ck, limp, e_in, jl, shot, fgt, .. } = *f;
    let lv = if live { 1.0 } else { w };
    let hu = haul_ease(&mut an.he_hu, haul && choke, smoothstep(tune.hold.haul.in_[0], tune.hold.haul.in_[1], ck) * (1.0 - smoothstep(tune.hold.haul.out[0], tune.hold.haul.out[1], ck)), tune.hold.haul.down, dt) * (1.0 - gv) * lv;
    let hauled = hu > 1e-4;
    let drop = (lerp(tune.hold.sag[0], tune.hold.sag[1], ck) + 0.02 * shot + tune.hold.struggle.dip * (0.5 + 0.5 * sin(tt * tau * 1.3)) * fgt) * w * e_in + tune.hold.catch.jolt.dip * jl;
    an.he_sw = if bare && live && held { sin(t / tune.hold.sway.secs * 2.0 * PI) * smoothstep(0.0, tune.hold.sway.in_, t) } else { damp(an.he_sw, 0.0, tune.hold.sway.out, dt) };
    let sw = an.he_sw * (1.0 - gv) * (1.0 - limp) * w;
    spring_step(&mut an.he_lk, if choke { clamp(t / tune.hold.secs.choke, 0.0, 1.0) } else { 0.0 }, if choke { 30.0 } else { 9.0 }, dt);
    let lk = clamp(an.he_lk.x, 0.0, 1.0);
    let lo0 = lower_ease(&mut an.he_lo, if low > 0.0 { clamp(low, 0.0, 1.0) * smoothstep(lower_k(beats, tune.hold.beats.lower.from, tune.hold.lower.from), lower_k(beats, tune.hold.beats.lower.by, tune.hold.lower.by), lk) } else { 0.0 }, lower_k(beats, tune.hold.beats.lower.up, tune.hold.lower.up), dt);
    let lo = lo0 * (1.0 - gv) * lv;
    let b_off = lower_ease(&mut an.he_bo, if beats { smoothstep(tune.hold.beats.off[0], tune.hold.beats.off[1], lk) } else { 0.0 }, tune.hold.beats.back, dt);
    let b_in = lower_ease(&mut an.he_bc, if beats { smoothstep(tune.hold.beats.clamp[0], tune.hold.beats.clamp[1], lk) } else { 0.0 }, tune.hold.beats.back, dt);
    let tip_u = if beats { smoothstep(tune.hold.beats.tip_by[0], tune.hold.beats.tip_by[1], lk) } else { 0.0 };
    let b_tip = tip_u * (1.0 - limp) * (1.0 - gv);
    (f.lv, f.hu, f.hauled, f.drop, f.sw, f.lo0, f.lo, f.b_off, f.b_in, f.tip_u, f.b_tip) = (lv, hu, hauled, drop, sw, lo0, lo, b_off, b_in, tip_u, b_tip);
}

/// How far he is sat back into the arm, and how far each foot has been left behind.
fn sat_back(tune: &Knobs, f: &mut BeingHeld) {
    let BeingHeld { live, grab, yank, pull, gv, ug, limp, cs_on, lv, hu, lo0, b_off, .. } = *f;
    let limp_a = max(max(limp, lo0), b_off);
    let sit_in = if grab { smoothstep(0.55, 1.0, ug) } else { 1.0 };
    let sit = if cs_on { sit_in * (1.0 - gv) * (1.0 - clamp(lo0, 0.0, 1.0)) * (1.0 - clamp(hu, 0.0, 1.0)) * lv } else { 0.0 };
    let off = sit * (1.0 - limp);
    let sit_f = if cs_on { sit_in * (1.0 - gv) * lv } else { 0.0 };
    let foot = |feet0: f64, feet1: f64, heels: f64| -> [f64; 3] {
        if !cs_on {
            return [0.0, 0.0, 0.0];
        }
        let p = pull * (1.0 - limp) * (1.0 - gv);
        let e = match yank {
            Some(y) if live => smoothstep(feet0, feet1, y),
            _ => 1.0,
        };
        let (lag, rest) = (p * (1.0 - e), heels * sit_f);
        [0.0, if p > 1e-4 { tune.hold.catch.lift * sin(PI * e) * min(1.0, p / 0.1) } else { 0.0 }, max(lag, rest)]
    };
    let (off_l, off_r) = (foot(tune.hold.catch.feet[0][0], tune.hold.catch.feet[0][1], tune.hold.catch.heels[0]), foot(tune.hold.catch.feet[1][0], tune.hold.catch.feet[1][1], tune.hold.catch.heels[1]));
    let off_on = off_l[2] > 1e-4 || off_r[2] > 1e-4;
    (f.limp_a, f.sit, f.off, f.off_l, f.off_r, f.off_on) = (limp_a, sit, off, off_l, off_r, off_on);
}

/// How his trunk, shoulders and head bend, and where his hands would hang.
fn bend(tune: &Knobs, f: &mut BeingHeld) {
    let BeingHeld { w, beats, bare, live, yank, tt, gv, limp, e_in, jl, shot, knock, amp, jerk, fgt, s1, s2, s3, br, hu, hauled, drop, lo, tip_u, b_tip, .. } = *f;
    let arch0 = lerp(tune.hold.arch, 2.0, limp) * DEGR * w * e_in + tune.hold.gun.duck.pitch * DEGR * 0.5 * shot;
    let yk_h = match yank {
        Some(y) if live => bump(y, 0.0, tune.hold.yank.in_, tune.hold.yank.in_ + tune.hold.yank.hold, tune.hold.yank.in_ + tune.hold.yank.hold + tune.hold.yank.out),
        _ => 0.0,
    };
    let arch_b = if bare { (tune.hold.bare_arch * (1.0 - limp) * (1.0 - gv) * e_in + tune.hold.yank.arch * yk_h) * DEGR * w } else { 0.0 };
    let arch = ((if hauled { arch0 + tune.hold.haul.arch * hu * DEGR } else { arch0 }) + arch_b) * (1.0 - jl) + tune.hold.catch.jolt.fwd * jl * DEGR;
    let heave = -tune.hold.breath.pitch * br * DEGR;
    let clav = (tune.hold.clav_up * e_in * (1.0 - limp) + tune.hold.gun.duck.hunch * shot + tune.hold.breath.clav * br) * DEGR * w + tune.hold.catch.jolt.clav * jl * DEGR;
    let (aside, chin_up) = (lerp(tune.hold.aside, tune.hold.gun.aside, gv), lerp(tune.hold.chin_up, tune.hold.gun.chin_up, gv));
    let chin0 = lerp(chin_up, tune.hold.loll, limp);
    let chin1 = chin0 * DEGR * w * e_in - drop * 40.0 * DEGR + (tune.hold.gun.duck.pitch * shot + tune.hold.gun.knock.pitch * knock + tune.hold.struggle.head * 0.35 * s3 * fgt) * DEGR - heave;
    let head_roll = (tune.hold.loll_roll * limp - aside * (1.0 - limp) * e_in * (1.0 - tip_u)
        + if beats { tune.hold.beats.tip * b_tip } else { 0.0 }
        + 3.0 * sin(tt * 3.1) * amp
        + tune.hold.struggle.roll * 0.7 * s1 * fgt
        - tune.hold.gun.duck.roll * shot
        - tune.hold.gun.knock.roll * knock)
        * DEGR
        * w;
    let hyw = (6.0 * sin(tt * 1.7 + 2.0) * amp + 9.0 * jerk + tune.hold.struggle.head * (0.6 * s2 + 0.4 * s3) * fgt + tune.hold.gun.away_yaw * e_in * gv + tune.hold.gun.duck.yaw * shot + tune.hold.gun.knock.yaw * knock)
        * DEGR;
    let hg_y = lower_k(beats, tune.hold.beats.lower.hang[0], tune.hold.lower.hang[0]) * lo;
    let hg_z = lower_k(beats, tune.hold.beats.lower.hang[1], tune.hold.lower.hang[1]) * lo;
    let dn = lo > 1e-4;
    // (his hands hang from where his shoulders now are)
    let (h0, h1) = (tune.hold.hang[0], tune.hold.hang[1]);
    let (hang_l, hang_r) = if dn { ([h0[0], h0[1] - hg_y, h0[2] + hg_z], [h1[0], h1[1] - hg_y, h1[2] + hg_z]) } else { (h0, h1) };
    (f.arch, f.heave, f.clav, f.chin1, f.head_roll, f.hyw, f.h0, f.h1, f.hang_l, f.hang_r) = (arch, heave, clav, chin1, head_roll, hyw, h0, h1, hang_l, hang_r);
}

/// Both arms in the arm hold: on the forearm, clawing at it, or hanging.
fn arm_hold(tune: &Knobs, f: &BeingHeld) -> ArmSet {
    let BeingHeld { beats, bare, armed, tt, fix_l, fix_r, tau, ck, amp, jerk, fgt, hu, hauled, b_in, limp_a, sit, hang_l, hang_r, .. } = *f;
    let c_in = if beats { b_in } else { smoothstep(0.04, 0.3, ck) };
    let c_r = if bare { max(c_in, tune.hold.bare_grip) } else { c_in };
    let cw_l = tune.hold.claw_l;
    let cw_r = tune.hold.claw_r;
    // (the haul: his hands go with the forearm they are on)
    let (cl_l, cl_r) = if hauled || sit > 1e-4 {
        let hv = [0.0, tune.hold.haul.sam_arm[0] * hu - tune.hold.catch.sink * sit, -tune.hold.haul.sam_arm[1] * hu];
        (v3::add(cw_l, hv), v3::add(cw_r, hv))
    } else {
        (cw_l, cw_r)
    };
    let claw = |o: f64| [tune.hold.struggle.claw * 0.6 * sin(tt * tau * 1.7 + o) * fgt, tune.hold.struggle.claw * sin(tt * tau * 2.6 + o) * fgt, tune.hold.struggle.claw * 0.4 * sin(tt * tau * 2.6 + o + 1.0) * fgt];
    let snk = tune.hold.catch.sink * sit;
    let rest = [tune.hold.rest_l[0], tune.hold.rest_l[1] - 0.03 * jerk - snk, tune.hold.rest_l[2] - 0.02 * jerk];
    let up = [tune.hold.raised_r[0] + 0.01 * sin(tt * tau * 1.1) * amp, tune.hold.raised_r[1] + 0.012 * sin(tt * tau * 0.9 + 1.0) * amp + 0.03 * jerk - snk, tune.hold.raised_r[2] + 0.01 * cos(tt * tau * 0.6) * amp];
    // clawing: the fingers working
    let cl = lerp(0.62, 0.82, 0.5 + 0.5 * sin(tt * tau * 2.6)) * fgt + 0.65 * (1.0 - fgt);
    let pole_l = v3::lerp3([1.0, -0.9, 0.3], [1.0, -0.5, 0.55], c_in);
    let pole_r = v3::lerp3([-0.8, -1.0, 0.15], [-1.0, -0.6, 0.5], c_r);
    let cu_l = |x: f64| lerp(lerp(x, cl, c_in), 0.2, limp_a);
    let cu_r = |x: f64| lerp(lerp(x, cl, c_r), 0.25, limp_a);
    let fi = |x: f64| lerp(x, 0.35, limp_a);
    ArmSet {
        tgt: [v3::lerp3(v3::lerp3(rest, v3::add(cl_l, claw(0.0)), c_in), hang_l, limp_a), v3::lerp3(v3::lerp3(up, v3::add(cl_r, claw(PI)), c_r), hang_r, limp_a)],
        rot: [
            quat::nlerp(hand_rot(&fix_l, [-0.25, -0.8, 0.5], [0.0, 0.55, 0.8]), hand_rot(&fix_l, [-0.5, 0.6, -0.6], [0.2, 0.5, 0.85]), c_in),
            quat::nlerp(hand_rot(&fix_r, [-0.2, 1.0, 0.1], [-0.15, 0.1, -1.0]), hand_rot(&fix_r, [0.5, 0.55, -0.6], [-0.2, 0.5, 0.85]), c_r),
        ],
        pole: [if bare { v3::lerp3(pole_l, [0.45, -1.0, 0.35], tune.hold.bare_elbow) } else { pole_l }, if bare { v3::lerp3(pole_r, [-0.45, -1.0, 0.35], tune.hold.bare_elbow) } else { pole_r }],
        curl: [[cu_l(0.25), cu_l(0.3), cu_l(0.32), cu_l(0.35)], if armed { [fi(0.75), fi(0.8), fi(0.8), fi(0.8)] } else { [cu_r(0.08), cu_r(0.1), cu_r(0.12), cu_r(0.18)] }],
        thumb: [lerp(0.15, 0.3, c_in), if armed { 0.6 } else { lerp(0.05, 0.3, c_r) }],
        spread: [0.0, if armed { 0.0 } else { 0.7 * (1.0 - c_r) }],
    }
}

/// Both arms in the gun hold: half up in front of the shoulders, palms forward, trembling, jerked in when a round goes past.
fn gun_hold(tune: &Knobs, f: &BeingHeld) -> ArmSet {
    let BeingHeld { tt, fix_l, fix_r, tau, shot, amp, jerk, limp_a, h0, h1, .. } = *f;
    // both hands half up in front of the shoulders, palms forward, trembling, jerked in when a round goes past
    let trem = 0.012 * sin(tt * tau * 1.1) * amp;
    let (u0, u1) = (tune.hold.gun.hands_up[0], tune.hold.gun.hands_up[1]);
    let (open_l, fist) = (max(limp_a, 0.85), 1.0 - limp_a * 0.7);
    ArmSet {
        tgt: [
            v3::lerp3([u0[0] - 0.03 * shot, u0[1] + trem + 0.05 * shot + 0.03 * jerk, u0[2] - 0.04 * shot], h0, limp_a),
            v3::lerp3([u1[0] + 0.03 * shot + 0.03 * sin(tt * 0.8) * amp, u1[1] - trem + 0.05 * shot + 0.04 * jerk, u1[2] - 0.04 * shot + 0.02 * cos(tt * 0.6) * amp], h1, limp_a),
        ],
        rot: [hand_rot(&fix_l, [0.15, 1.0, 0.2], [0.1, 0.15, -1.0]), hand_rot(&fix_r, [-0.15, 1.0, 0.2], [-0.1, 0.15, -1.0])],
        pole: [[0.8, -1.0, 0.2], [-0.8, -1.0, 0.2]],
        curl: [[lerp(0.6, 0.15, open_l), lerp(0.7, 0.2, open_l), lerp(0.75, 0.25, open_l), lerp(0.8, 0.3, open_l)], [0.2 + 0.55 * fist, 0.25 + 0.55 * fist, 0.3 + 0.55 * fist, 0.3 + 0.55 * fist]],
        thumb: [lerp(0.3, 0.15, open_l), 0.2 + 0.4 * fist],
        spread: [0.35, 0.0],
    }
}

/// The two sets of the arms, or the two blended for the beat of a change; and where the disarm has got to.
fn arm_sets(tune: &Knobs, f: &mut BeingHeld) {
    let BeingHeld { grab, strip, gv, ug, lo0, b_off, .. } = *f;
    let z = if gv < 1.0 && gv > 0.0 {
        let (x, y) = (arm_hold(tune, f), gun_hold(tune, f));
        let mut z = x;
        for s in 0..2 {
            z.tgt[s] = v3::lerp3(x.tgt[s], y.tgt[s], gv);
            z.rot[s] = quat::nlerp(x.rot[s], y.rot[s], gv);
            z.pole[s] = v3::lerp3(x.pole[s], y.pole[s], gv);
            for i in 0..4 {
                z.curl[s][i] = lerp(x.curl[s][i], y.curl[s][i], gv);
            }
            z.thumb[s] = lerp(x.thumb[s], y.thumb[s], gv);
            z.spread[s] = lerp(x.spread[s], y.spread[s], gv);
        }
        z
    } else if gv < 1.0 {
        arm_hold(tune, f)
    } else {
        gun_hold(tune, f)
    };
    let s_t = strip.unwrap_or(0.0);
    let s_on = strip.is_some() && s_t < tune.hold.strip.home;
    let e_a = if grab { smoothstep(0.1, 0.75, ug) } else { 1.0 };
    let kept = lo0 > 1e-4 || b_off > 1e-4;
    (f.z, f.s_t, f.s_on, f.e_a, f.kept) = (z, s_t, s_on, e_a, kept);
}

/// The pelvis sunk, hauled, swayed, sat and lowered; his feet noted first.
fn held_hips(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, he_span: &mut f64, f: &mut BeingHeld) {
    let BeingHeld { w, beats, bare, t, grab, ag, gv, limp, hb, hu, hauled, drop, sw, lo, sit, off, .. } = *f;
    let (hips, sp1, sp2, sp3) = (role(HIPS), role(SPINE1), role(SPINE2), role(SPINE3));
    fk_run(rig, d, fk.rot, fk.pos);
    let (f_l, f_r, r_l, r_r) = (fk.at(role(FOOT_L)), fk.at(role(FOOT_R)), fk.rot(role(FOOT_L)), fk.rot(role(FOOT_R)));
    if drop > 1e-4 {
        shift(d, rig, hips, &[0.0, -drop, 0.0]);
        turn(d, rig, hips, q_pitch(drop * 40.0 * DEGR));
    }
    if hauled {
        shift(d, rig, hips, &[0.0, tune.hold.haul.lift * hu, -tune.hold.haul.back * hu]);
        turn(d, rig, hips, q_pitch(tune.hold.haul.recline * hu * DEGR));
    }
    if bare {
        // (his head tips away from Sam's)
        turn(d, rig, role(NECK), q_roll(tune.hold.apart.tip * DEGR * w * (1.0 - gv) * (1.0 - limp) * (1.0 - clamp(hu, 0.0, 1.0)) * (if grab { smoothstep(0.3, 1.0, t / tune.hold.secs.grab) } else { 1.0 })));
    }
    if abs(sw) > 1e-4 {
        shift(d, rig, hips, &[tune.hold.sway.m * tune.hold.sway.his * sw, 0.0, 0.0]);
        turn(d, rig, sp2, q_yaw(tune.hold.sway.twist * (0.6 + 0.8 * ag) * sw * DEGR));
    }
    // hauled off his feet's line: sat down into the arm, the hips ahead of where they stood with the trunk reclined about his own neck
    let mut hit_rec = 0.0;
    if sit > 1e-4 || hb > 1e-4 {
        if falsy(*he_span) {
            fk_run(rig, d, fk.rot, fk.pos);
            *he_span = max(0.2, fk.at(role(NECK))[1] - fk.at(hips)[1]);
        }
        let (span, ahead) = (*he_span, tune.hold.catch.hips * off + tune.hold.hit.hips * hb);
        let (rec, rec0) = (asin(clamp(ahead / span, 0.0, 0.6)), asin(clamp(tune.hold.catch.hips * off / span, 0.0, 0.6)));
        hit_rec = rec - rec0;
        shift(d, rig, hips, &[0.0, -tune.hold.catch.sink * sit - tune.hold.hit.dip * hb + span * (cos(rec0) - cos(rec)), ahead]);
        turn(d, rig, hips, q_pitch(-rec));
    }
    if lo > 1e-4 {
        shift(d, rig, hips, &[0.0, -lower_k(beats, tune.hold.beats.lower.sink, tune.hold.lower.sink) * lo, lower_k(beats, tune.hold.beats.lower.fwd, tune.hold.lower.fwd) * lo]);
        turn(d, rig, hips, q_pitch(lower_k(beats, tune.hold.beats.lower.hip, tune.hold.lower.hip) * lo * DEGR));
    }
    (f.sp1, f.sp2, f.sp3, f.f_l, f.f_r, f.r_l, f.r_r, f.hit_rec) = (sp1, sp2, sp3, f_l, f_r, r_l, r_r, hit_rec);
}

/// The spine arched onto the arm with his struggle and his breath, the shoulders, the head.
fn held_trunk(tune: &Knobs, d: &mut [S], rig: &Rig, f: &BeingHeld) {
    let BeingHeld { beats, jl, fgt, s1, s2, roll, yaw_s, hu, hauled, lo, arch, heave, clav, chin1, head_roll, hyw, sp1, sp2, sp3, hit_rec, .. } = *f;
    turn(d, rig, sp1, quat::mul(q_yaw(yaw_s * 0.3), q_pitch(arch * 0.3)));
    turn(d, rig, sp2, quat::mul(q_yaw(yaw_s * 0.4), quat::mul(q_roll(roll * 0.4), q_pitch(arch * 0.35 + heave * 0.4))));
    turn(d, rig, sp3, quat::mul(q_yaw(yaw_s * 0.3), quat::mul(q_roll(roll * 0.6), q_pitch(arch * 0.35 + heave * 0.6))));
    // (the shoulders working against the arm, out of step)
    turn(d, rig, role(SHOULDER_L), q_roll(clav + 3.0 * s1 * fgt * DEGR));
    turn(d, rig, role(SHOULDER_R), q_roll(-clav + 3.0 * s2 * fgt * DEGR));
    let chin = (if hauled { chin1 + tune.hold.haul.chin * hu * DEGR } else { chin1 }) * (1.0 - jl) + tune.hold.catch.jolt.head * jl * DEGR + tune.hold.hit.head * hit_rec;
    turn(d, rig, role(NECK), quat::mul(q_yaw(hyw * 0.4), quat::mul(q_roll(head_roll * 0.4), q_pitch(chin * 0.45))));
    turn(d, rig, role(HEAD), quat::mul(q_yaw(hyw * 0.6), quat::mul(q_roll(head_roll * 0.6), q_pitch(chin * 0.55))));
    if lo > 1e-4 {
        let f = lower_k(beats, tune.hold.beats.lower.fold, tune.hold.lower.fold) * lo * DEGR;
        turn(d, rig, sp1, q_pitch(f * 0.3));
        turn(d, rig, sp2, q_pitch(f * 0.35));
        turn(d, rig, sp3, q_pitch(f * 0.35));
    }
}

/// The feet put back where they stood, or stepped after him, or kicking.
fn held_feet(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, f: &BeingHeld) {
    let BeingHeld { hb, fgt, s1, s2, hu, hauled, drop, lo, off_l, off_r, off_on, f_l, f_r, r_l, r_r, .. } = *f;
    let (thl, shl, ftl, thr, shr, ftr) = (role(THIGH_L), role(SHIN_L), role(FOOT_L), role(THIGH_R), role(SHIN_R), role(FOOT_R));
    if hauled {
        // off his feet: they come up and forward off where they stood, toes down, kicking with the fight
        let (toe, kf, ku) = (q_pitch(tune.hold.haul.toes * hu * DEGR), tune.hold.haul.kick[0] * fgt * hu, tune.hold.haul.kick[1] * fgt * hu);
        let t_l = [f_l[0], f_l[1] + tune.hold.haul.feet[0] * hu + ku * (0.5 + 0.5 * s2) + off_l[1], f_l[2] + tune.hold.haul.feet[1] * hu + kf * s1 + off_l[2]];
        let t_r = [f_r[0], f_r[1] + tune.hold.haul.feet[0] * hu + ku * (0.5 - 0.5 * s2) + off_r[1], f_r[2] + tune.hold.haul.feet[1] * hu - kf * s1 + off_r[2]];
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, thl, shl, ftl, &t_l, Some(&knee_pole(&r_l)), 0.6, Some(&quat::mul(toe, r_l)), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, thr, shr, ftr, &t_r, Some(&knee_pole(&r_r)), 0.6, Some(&quat::mul(toe, r_r)), true);
    } else if off_on {
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, thl, shl, ftl, &v3::add(f_l, off_l), Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, thr, shr, ftr, &v3::add(f_r, off_r), Some(&knee_pole(&r_r)), 0.6, Some(&r_r), true);
    } else if drop > 1e-4 || lo > 1e-4 || hb > 1e-4 {
        fk_run(rig, d, fk.rot, fk.pos);
        two_bone_ik(rig, d, fk.rot, fk.pos, thl, shl, ftl, &f_l, Some(&knee_pole(&r_l)), 0.6, Some(&r_l), false);
        two_bone_ik(rig, d, fk.rot, fk.pos, thr, shr, ftr, &f_r, Some(&knee_pole(&r_r)), 0.6, Some(&r_r), true);
    }
}

/// Where each hand is bound: flung out and onto the arm, up, or after the gun torn out of it.
fn held_hands_go(tune: &Knobs, fk: &mut Fk, f: &mut BeingHeld) {
    let BeingHeld { grab, ug, cr, z, s_t, s_on, .. } = *f;
    let (hl, hr) = (role(HAND_L), role(HAND_R));
    let (live_l, live_r, r_l0, r_r0) = (fk.at(hl), fk.at(hr), fk.rot(hl), fk.rot(hr));
    let (mut tgt_l, mut tgt_r) = (z.tgt[0], z.tgt[1]);
    if cr {
        // (both hands fly out from his sides first, hang there a tenth of a second, and come onto the arm from there one after the other)
        let way = |live: [f64; 3], land: f64, to: [f64; 3], end: [f64; 3]| curve3(clamp(ug / land, 0.0, 1.0), &[(0.0, live), (tune.hold.catch.fling.t[0] / land, to), (tune.hold.catch.fling.t[1] / land, v3::add(to, tune.hold.catch.fling.drift)), (1.0, end)]);
        tgt_l = way(live_l, tune.hold.catch.fling.land[0], tune.hold.catch.fling.to[0], tgt_l);
        tgt_r = way(live_r, tune.hold.catch.fling.land[1], tune.hold.catch.fling.to[1], tgt_r);
    } else if grab {
        tgt_l = v3::lerp3(live_l, tgt_l, smoothstep(0.15, 0.7, ug));
        tgt_r = curve3(smoothstep(0.05, 0.85, ug), &[(0.0, live_r), (0.5, tune.hold.via_up), (1.0, tgt_r)]);
    }
    // the disarm: his gun hand is thrown out beside his ribs as the arm lands, dragged after the wrench, let go of, and only then comes up
    if s_on {
        tgt_r = curve3(strip_in(s_t, tune.hold.strip.take), &[(0.0, live_r), (tune.hold.strip.take, tune.hold.strip.meet), (tune.hold.strip.take + 0.06, tune.hold.strip.torn), (tune.hold.strip.free, tune.hold.strip.torn), (tune.hold.strip.up, z.tgt[1])]);
    }
    (f.hl, f.hr, f.r_l0, f.r_r0, f.tgt_l, f.tgt_r) = (hl, hr, r_l0, r_r0, tgt_l, tgt_r);
}

/// Both arms onto their spots, and the fingers (a blend between the hold and the hang goes round the way it went the frame before).
fn held_arms(tune: &Knobs, d: &mut [S], rig: &Rig, fk: &mut Fk, hh: &mut Hands, he_hr: &mut [Option<[f64; 4]>; 2], f: &BeingHeld) {
    let BeingHeld { fix_l, fix_r, limp_a, z, s_t, s_on, e_a, kept, hl, hr, r_l0, r_r0, tgt_l, tgt_r, .. } = *f;
    let (rot_hang_l, rot_hang_r) = (hand_rot(&fix_l, [0.1, -1.0, 0.15], [1.0, 0.0, 0.1]), hand_rot(&fix_r, [-0.1, -1.0, 0.15], [-1.0, 0.0, 0.1]));
    // (which way round the blend between the hold and the hang goes is the way it went the frame before)
    let to_hang = |q: [f64; 4], h: [f64; 4], prev: Option<[f64; 4]>| -> [f64; 4] {
        let dot = |x: &[f64; 4], y: &[f64; 4]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2] + x[3] * y[3];
        let near = quat::nlerp(q, h, limp_a);
        let Some(prev) = prev else {
            return near;
        };
        let s = if dot(&q, &h) < 0.0 { 1.0 } else { -1.0 };
        let far = quat::normalize([lerp(q[0], h[0] * s, limp_a), lerp(q[1], h[1] * s, limp_a), lerp(q[2], h[2] * s, limp_a), lerp(q[3], h[3] * s, limp_a)]);
        if abs(dot(&far, &prev)) > abs(dot(&near, &prev)) {
            far
        } else {
            near
        }
    };
    let was = if kept { *he_hr } else { [None, None] };
    let rot_l = to_hang(quat::nlerp(r_l0, z.rot[0], e_a), rot_hang_l, was[0]);
    let rot_r = to_hang(quat::nlerp(r_r0, z.rot[1], if s_on { smoothstep(tune.hold.strip.take, tune.hold.strip.up, s_t) } else { e_a }), rot_hang_r, was[1]);
    *he_hr = if kept { [Some(rot_l), Some(rot_r)] } else { [None, None] };
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_L), role(FOREARM_L), hl, &tgt_l, Some(&z.pole[0]), 0.85 * e_a, Some(&rot_l), false);
    let w_r = if s_on { max(0.85 * e_a, 0.9 * smoothstep(0.0, 0.1, s_t) * (1.0 - smoothstep(tune.hold.strip.up, tune.hold.strip.home, s_t))) } else { 0.85 * e_a };
    two_bone_ik(rig, d, fk.rot, fk.pos, role(UPPER_ARM_R), role(FOREARM_R), hr, &tgt_r, Some(&z.pole[1]), w_r, Some(&rot_r), false);
    hand(d, hh, 0, &z.curl[0], z.thumb[0], z.spread[0]);
    if s_on {
        // (the fist round the gun opens as the gun leaves it, not in a frame)
        let op = smoothstep(tune.hold.strip.take, tune.hold.strip.take + 0.1, s_t);
        hand(d, hh, 1, &[lerp(0.75, z.curl[1][0], op), lerp(0.8, z.curl[1][1], op), lerp(0.8, z.curl[1][2], op), lerp(0.8, z.curl[1][3], op)], lerp(0.6, z.thumb[1], op), z.spread[1] * op);
    } else {
        hand(d, hh, 1, &z.curl[1], z.thumb[1], z.spread[1]);
    }
}

/// How much of the arms the layer has (the first frames of the grab: mostly still his own; the catch's fling has them before the layer's weight is in).
fn arm_share(tune: &Knobs, f: &mut BeingHeld) {
    let BeingHeld { w, ug, cr, lv, e_a, .. } = *f;
    let w_a0 = w * e_a + w * (1.0 - e_a) * 0.35;
    let w_a = if cr { max(w_a0, smoothstep(tune.hold.catch.fling.arm_in[0], tune.hold.catch.fling.arm_in[1], ug) * lv) } else { w_a0 };
    f.w_a = w_a;
}

/// His gun arm is where the disarm wants it by `take`.
fn disarm_held(g: usize, n: usize, tune: &Knobs, f: &BeingHeld) -> bool {
    let BeingHeld { at, at_b, s_t, s_on, w_a, .. } = *f;
    if s_on && w_a < 1.0 {
        // (…but his gun arm is where the disarm wants it by `take`)
        let w_s = max(w_a, smoothstep(0.0, 0.8 * tune.hold.strip.take, s_t) * (1.0 - smoothstep(tune.hold.strip.up, tune.hold.strip.home, s_t)));
        if w_s > w_a {
            ask_mask(&[(role(SHOULDER_R), 1.0, true)]);
            if !ask_blend(g, at, at_b, (w_s - w_a) / (1.0 - w_a), true, at, n) {
                return false;
            }
        }
    }
    true
}
