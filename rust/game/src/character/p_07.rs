//! src/game/character.ts lines 2111 to 2620
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// How the throw under way reads: none, a toss, short or full.   (character.ts throwRead)
pub fn anim_throw_read(g: &mut crate::game::Game, c: CharId) -> Option<crate::character::AnimThrowRead> {
    let a = &g.ch[c].anim;
    if a.throw_t < 0.0 { None } else if a.thr_low >= 0.0 { Some(AnimThrowRead::Toss) } else if a.thr_far < 1.0 { Some(AnimThrowRead::Short) } else { Some(AnimThrowRead::Full) }
}
/// The item is still in the hand.   (character.ts throwing)
pub fn anim_throwing(g: &mut crate::game::Game, c: CharId) -> bool {
    let t = g.ch[c].anim.throw_t;
    t >= 0.0 && t < THROW_RELEASE + 0.05
}
/// The throw one-shot is playing at all.   (character.ts inThrow)
pub fn anim_in_throw(g: &mut crate::game::Game, c: CharId) -> bool { g.ch[c].anim.throw_t >= 0.0 }
/// The throw under way is the free (left) hand's.   (character.ts throwOff)
pub fn anim_throw_off(g: &mut crate::game::Game, c: CharId) -> bool { g.ch[c].anim.throw_t >= 0.0 && g.ch[c].anim.thr_off }
/// How much of the pistol's aim clip the pose shows, 0..1.   (character.ts aimShown)
pub fn anim_aim_shown(g: &mut crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.upper_w.aim }
/// How far the torch is up in the hand that holds it, 0..1.   (character.ts torchUp)
pub fn anim_torch_up(g: &mut crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.tu_k }
/// His long gun is off its line and his eyes are at its magazine well.   (character.ts atWell)
pub fn anim_at_well(g: &mut crate::game::Game, c: CharId) -> bool { m::max(g.ch[c].anim.rlc.k, g.ch[c].anim.rlc.h) > 0.5 }
/// Nothing but the aim or the carry has the gun arm this frame.   (character.ts gunArmOwn)
pub fn anim_gun_arm_own(g: &mut crate::game::Game, c: CharId) -> bool {
    let a = &g.ch[c].anim;
    let taken = a.hold_pose.is_some() || a.held_pose.is_some() || a.haul.is_some() || a.lockpick || a.act.is_some() || a.wound.is_some() || a.kit.is_some() || a.sw.shown || a.hide_held_item || a.throw_t >= 0.0 || a.strike_t >= 0.0
        || a.hit_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0 || a.tell_t >= 0.0;
    !(taken || anim_mv_busy(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0)
}
/// Seconds into the throw, -1 when not throwing.   (character.ts throwTime)
pub fn anim_throw_time(g: &mut crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.throw_t }
/// A shot's first 0.35 s.   (character.ts shooting)
pub fn anim_shooting(g: &mut crate::game::Game, c: CharId) -> bool {
    let t = g.ch[c].anim.shoot_t;
    t >= 0.0 && t < 0.35
}
/// Starts the front kick at a door, unless one is playing.   (character.ts kickDoor)
pub fn anim_kick_door(g: &mut crate::game::Game, c: CharId, opts: Option<crate::character::AnimKickDoorOpts>) {
    let a = &mut g.ch[c].anim;
    if a.kick_t < 0.0 {
        a.kick_t = 0.0;
        a.kick_hit = false;
        a.kick_from = None;
        a.kick_leg = match opts.and_then(|o| o.leg) {
            Some(AnimKickDoorOptsLeg::Left) => AnimKickLeg::Left,
            Some(AnimKickDoorOptsLeg::Right) | None => AnimKickLeg::Right,
        };
    }
}
/// The kick is playing.   (character.ts kicking)
pub fn anim_kicking(g: &mut crate::game::Game, c: CharId) -> bool { g.ch[c].anim.kick_t >= 0.0 }
/// Which leg the kick in progress, or the last one, swings.   (character.ts kickingLeg)
pub fn anim_kicking_leg(g: &mut crate::game::Game, c: CharId) -> crate::character::AnimKickingLeg {
    match g.ch[c].anim.kick_leg { AnimKickLeg::Left => AnimKickingLeg::Left, AnimKickLeg::Right => AnimKickingLeg::Right }
}
/// Seconds into the kick, -1 when not kicking.   (character.ts kickTime)
pub fn anim_kick_time(g: &mut crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.kick_t }
/// True for exactly the update in which the sole reached the door.   (character.ts kickImpact)
pub fn anim_kick_impact(g: &mut crate::game::Game, c: CharId) -> bool { g.ch[c].anim.kick_hit }
/// Starts a left-hand signal, unless one is playing.   (character.ts signal)
pub fn anim_signal(g: &mut crate::game::Game, c: CharId, kind: crate::character::SignalKind, side: Option<f64>) {
    let side = side.unwrap_or(1.0);
    let mut kind = kind;
    if kind == SignalKind::Squeeze && !mv_on(g, MvOnK::Squeeze) { kind = SignalKind::Hold; }
    let a = &mut g.ch[c].anim;
    if a.sig_t < 0.0 { a.sig_t = 0.0; a.sig_kind = kind; a.sig_side = side; }
}
/// A signal is playing.   (character.ts signalling)
pub fn anim_signalling(g: &mut crate::game::Game, c: CharId) -> bool { g.ch[c].anim.sig_t >= 0.0 }
/// His left hand is his own this frame.   (character.ts signalFree)
pub fn anim_signal_free(g: &mut crate::game::Game, c: CharId) -> bool {
    if !anim_gun_arm_own(g, c) { return false; }
    let a = &g.ch[c].anim;
    a.kick_t < 0.0 && a.dazzle.is_none() && a.reload_at < 0.0 && a.rl_t < 0.0
        && a.rx.w.x < 0.02 && a.wk.w_l.x < 0.02 && a.dp.w_l.x < 0.02 && !a.nv.as_ref().map_or(false, |n| n.hand)
}
/// Starts an idle tell, unless one or a signal is playing.   (character.ts tell)
pub fn anim_tell(g: &mut crate::game::Game, c: CharId, kind: crate::character::TellKind) {
    let a = &mut g.ch[c].anim;
    if a.tell_t < 0.0 && a.sig_t < 0.0 { a.tell_t = 0.0; a.tell_k = kind; }
}
/// Drops a tell where it is.   (character.ts cancelTell)
pub fn anim_cancel_tell(g: &mut crate::game::Game, c: CharId) {
    let a = &mut g.ch[c].anim;
    a.tell_t = -1.0;
    a.tell_shown = 0.0;
}
/// A tell is playing.   (character.ts telling)
pub fn anim_telling(g: &mut crate::game::Game, c: CharId) -> bool { g.ch[c].anim.tell_t >= 0.0 }
/// Which tell is playing, or played last.   (character.ts tellKind)
pub fn anim_tell_kind(g: &mut crate::game::Game, c: CharId) -> crate::character::TellKind { g.ch[c].anim.tell_k }

/// The update, over poses that stand in a block of the module for its length.   (character.ts update)
pub fn anim_update(g: &mut crate::game::Game, c: CharId, dt: f64) {
    let lent = crate::anim_skeleton::stage_open(g, crate::Opaque);
    anim_update_standing(g, c, dt);
    if lent { crate::anim_skeleton::stage_shut(g, crate::Opaque); }
}

// ---- helpers and seams of updateStanding

/// A clip of his rig by its name (`this.clips.Walk_Loop`).
fn clip_of(g: &Game, c: CharId, name: &'static str) -> ClipId { *g.ch[c].anim.clips.get(name).unwrap() }
/// A clip's length (`c.Walk_Loop.duration`).
fn clip_dur(g: &Game, c: CharId, name: &'static str) -> f64 { g.tab.clip[clip_of(g, c, name)].duration }
/// `bigOff(this.bigTool)`: the skeleton's parameter is the fields it reads.
fn big_off_tool(b: &AnimBigTool) -> bool { big_off(BigOffB { t: b.t, equip: b.equip, stow: b.stow, end: b.end, snug: b.snug }) }
/// The nod of the goggles onto pA (`this.nvTip(this.pA, nod)`): pA is lent out and put back; nvTip and nvHead read only the pose handed in.
fn nv_tip_p_a(g: &mut Game, c: CharId, nod: f64) {
    let mut pa = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
    anim_nv_tip(g, c, &mut pa, nod, None);
    g.ch[c].anim.p_a = pa;
}
/// What the goggles' flip and a smoke both ask of the hands (lines 2585 and 2588 share every term after `!this.nv`).
fn left_hand_clear(g: &mut Game, c: CharId, w_hd: f64, w_he: f64, w_lp: f64, w_hl: f64, w_lg: f64, w_wall_arms: f64, w_act: f64, w_strike: f64, kt: f64) -> bool {
    let a = &g.ch[c].anim;
    let first = w_hd < 0.05 && w_he < 0.05 && w_lp < 0.05 && w_hl < 0.05 && w_lg < 0.05 && w_wall_arms < 0.05 && w_act <= 0.0 && w_strike <= 0.0 && kt < 0.0 && a.throw_t < 0.0 && a.reload_t < 0.0 && a.hit_t < 0.0 && a.tell_t < 0.0
        && a.upper_w.aim < 0.3 && a.kit.is_none() && a.ready.is_none() && a.wound.is_none() && a.dazzle.is_none();
    if !first || anim_mv_busy(g, c) { return false; }
    let a = &g.ch[c].anim;
    a.mvs.arm_l <= 0.0 && a.mvs.ts_t < 0.0 && a.dp.w_l.x < 0.05 && !a.wk.fed && a.wk.shown < 0.05
}

/// `LAYERS.knobs?.()`: the one thing it holds is rustknobs.ts knobsNow; inside a tick the list is the tick's already.
fn knobs_now(g: &mut Game) { if !g.m.rustknobs.rust_knobs.tick { crate::rustknobs::knobs_to_rust(g); } }
/// `sampleAt0(clip, this.pUpper)`: the pose is lent out of g for the call; sampleAt0 reads the clip and never the pose in g.
fn at0_upper(g: &mut Game, c: CharId, clip: ClipId) {
    let mut out = core::mem::replace(&mut g.ch[c].anim.p_upper, crate::anim_skeleton::Pose::blank());
    sample_at0(g, clip, &mut out);
    g.ch[c].anim.p_upper = out;
}
/// `sampleAt0(clip, this.pB)`, the same way.
fn at0_b(g: &mut Game, c: CharId, clip: ClipId) {
    let mut out = core::mem::replace(&mut g.ch[c].anim.p_b, crate::anim_skeleton::Pose::blank());
    sample_at0(g, clip, &mut out);
    g.ch[c].anim.p_b = out;
}
/// `STANCES[s]`, for a stance that has a shape (not 'none').
fn stance_shape(s: Stance) -> StanceShape { *stances().get(String::from(s.as_str())).unwrap() }
/// `SIGNAL_DUR[k]`
fn signal_secs(k: SignalKind) -> f64 { *signal_dur().get(String::from(k.as_str())).unwrap() }
/// `TELL_DUR[k]`
fn tell_secs(k: TellKind) -> f64 { *tell_dur().get(String::from(k.as_str())).unwrap() }
/// `READY[k].knife`
fn ready_knife(k: ReadyKind) -> bool { ready().get(String::from(k.as_str())).unwrap().knife }
/// `BLOW.clip[s]`
fn blow_clip(s: BlowStyle) -> f64 { *blow().clip.get(String::from(s.as_str())).unwrap() }

/// `this.runArms(this.pA, w, ws)`
fn seam_run_arms(g: &mut Game, c: CharId, w: f64, ws: f64) {
    // BRING-UP: pA is lent out of g for the call; the layer's write comes back into it through the stage (anim_skeleton/b_01.rs stage_back writes the pose's own store)
    let pose = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
    anim_run_arms(g, c, &pose, w, ws);
    g.ch[c].anim.p_a = pose;
}
/// `this.softKnees(this.pA, w)`
fn seam_soft_knees(g: &mut Game, c: CharId, w: f64) {
    // BRING-UP: as seam_run_arms
    let pose = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
    anim_soft_knees(g, c, &pose, w);
    g.ch[c].anim.p_a = pose;
}
/// `this.kitHold !== undefined`
fn seam_kit_hold_defined(g: &mut Game, c: CharId) -> bool { g.ch[c].anim.kit_hold.is_some() }   // (skeleton 4 has it as Option<Option<KitItem>>: None = undefined)
/// The swap's clock, read off `carry`. Once Anim.carry and AnimSw.cur are Option<Option<_>> (None = undefined, Some(None) = null):
/// want = if bake { S.cur } else { carry }; if !bake { carry = None }; then, unless bake:
/// want None: cur = None, t = wait = -1. cur None: cur = want, t = wait = -1. want != cur: if both are Some(Some(_)) and t < 0
/// { if wait < 0 { wait = swap_lag }; if wait > 0 && shoot_t < 0 { wait = max(0, wait - dt) } else { from = cur; to = want; t = 0;
/// dur = want == weapon ? swap().draw : swap().stow; cur = want; wait = -1 } } else { t = wait = -1; cur = want }. Else wait = -1.
fn seam_swap_clock(g: &mut Game, c: CharId, bake: bool, dt: f64) {
    // SKELETON-GAP: Anim.carry and AnimSw.cur as Option<Option<_>> (undefined | null | a kind). With one Option the two merge, and by reading that changes nothing here:
    // `t` is -1 whenever `cur` is null or undefined (every path into either sets it), so "want null, cur null: wait = -1" and "want undefined: t = wait = -1" leave the
    // same numbers; and a `cur` of null or undefined takes a `want` the same way (cur = want, t = wait = -1). What IS lost is the state's own word (the bridge's diff).
    use crate::character::{AnimCarry, AnimSwCur, AnimSwFrom, AnimSwTo};
    if bake { return; }
    let (draw, stow) = (swap().draw, swap().stow);
    let a = &mut g.ch[c].anim;
    let want = a.carry.take().map(|k| match k { AnimCarry::Torch => AnimSwCur::Torch, AnimCarry::Weapon => AnimSwCur::Weapon });
    match (want, a.sw.cur) {
        (None, _) => { a.sw.cur = None; a.sw.t = -1.0; a.sw.wait = -1.0; }
        (Some(w), None) => { a.sw.cur = Some(w); a.sw.t = -1.0; a.sw.wait = -1.0; }
        (Some(w), Some(cur)) if w != cur => {
            if a.sw.t < 0.0 {
                if a.sw.wait < 0.0 { a.sw.wait = a.swap_lag; }
                if a.sw.wait > 0.0 && a.shoot_t < 0.0 { a.sw.wait = m::max(0.0, a.sw.wait - dt); }
                else {
                    a.sw.from = Some(match cur { AnimSwCur::Torch => AnimSwFrom::Torch, AnimSwCur::Weapon => AnimSwFrom::Weapon });
                    a.sw.to = Some(match w { AnimSwCur::Torch => AnimSwTo::Torch, AnimSwCur::Weapon => AnimSwTo::Weapon });
                    a.sw.t = 0.0;
                    a.sw.dur = if w == AnimSwCur::Weapon { draw } else { stow };
                    a.sw.cur = Some(w);
                    a.sw.wait = -1.0;
                }
            } else { a.sw.t = -1.0; a.sw.wait = -1.0; a.sw.cur = Some(w); }
        }
        _ => { a.sw.wait = -1.0; }
    }
}

/// The animator's whole frame: every layer, in order, over pA; then the pose and the twists.   (character.ts updateStanding)
pub fn anim_update_standing(g: &mut crate::game::Game, c: CharId, dt: f64) {
    knobs_now(g);
    let sn = sneak();
    let cr = creep();
    let singles = g.k.posemath.singles;   // (an open row: POSEMATH.singles stays a test)
    let dash_k_stride = g.k.dash.stride;
    let dash_k_rate = g.k.dash.rate;
    let pr = g.ch[c].anim.params.clone();
    // ---- one-shot clocks that shape the base graph
    let kick_dur = KICK_DUR;
    let kt = g.ch[c].anim.kick_t;
    let w_k = if kt >= 0.0 { bump(kt, 0.0, 0.12, kick_dur - 0.2, kick_dur) } else { 0.0 };
    let w_kbody = if kt >= 0.0 { 1.0 - v3::smoothstep(kick_dur - 0.2, kick_dur, kt) } else { 0.0 };
    let w_kstand = if kt >= 0.0 { bump(kt, 0.0, 0.2, kick_dur - 0.3, kick_dur) } else { 0.0 };
    let w_kslow = if kt >= 0.0 { bump(kt, 0.0, 0.3, kick_dur - 0.3, kick_dur) } else { 0.0 };
    let whip = g.k.hold.secs.whip;
    let whip_thru = WHIP_THRU;
    let a = &mut g.ch[c].anim;
    let w_lp = v3::clamp(spring_step(&mut a.lp_w, if a.lockpick { 1.0 } else { 0.0 }, 17.0, dt), 0.0, 1.0);
    if a.haul.is_some() { a.hl_last = a.haul.clone(); } else if a.hl_w.x < 0.003 { a.hl_last = None; }
    let w_hl = if a.hl_last.is_some() { v3::clamp(spring_step(&mut a.hl_w, if a.haul.is_some() { 1.0 } else { 0.0 }, 16.0, dt), 0.0, 1.0) } else { 0.0 };
    if let Some(h) = &a.hold_pose { a.hd_last = h.clone(); } else if a.hd_last.phase == HoldPhase::Whip && a.hd_last.t < whip + 1.0 { a.hd_last.t = a.hd_last.t + dt; }
    if let Some(h) = &a.held_pose { a.he_last = h.clone(); }
    let has_hold = a.hold_pose.is_some();
    let thru_whip = !has_hold && a.hd_last.phase == HoldPhase::Whip && a.hd_last.t >= whip && a.hd_last.t < whip + whip_thru;
    let hd_rate = if has_hold { 16.0 } else {
        let (last, speed) = (a.hd_last.clone(), a.speed);
        let busy = anim_hold_busy(g, c);
        hold_out_rate(g, last, speed, busy)
    };
    let a = &mut g.ch[c].anim;
    let w_hd = v3::clamp(spring_step(&mut a.hd_w, if has_hold || thru_whip { 1.0 } else { 0.0 }, hd_rate, dt), 0.0, 1.0);
    let w_he = v3::clamp(spring_step(&mut a.he_w, if a.held_pose.is_some() { 1.0 } else { 0.0 }, 16.0, dt), 0.0, 1.0);
    // each figure's eased share of the variant (0 = arm, 1 = gun)
    let swap_rate = SWAP_RATE;
    let gv_v = a.hold_pose.as_ref().map_or(a.hd_last.variant, |h| h.variant);
    let gv_t = if gv_v == Some(HoldVariant::Gun) { 1.0 } else { 0.0 };
    if has_hold && !a.hd_on {
        a.hd_from = None; a.hd_caught = false;
        a.hd_gv.x = gv_t; a.hd_gv.v = 0.0; a.hd_ck.x = 0.0; a.hd_ck.v = 0.0; a.hd_kc.x = 0.0; a.hd_kc.v = 0.0;
        a.hd_lo.x *= w_hd; a.hd_lo.v = 0.0; a.hd_hu.x *= w_hd; a.hd_hu.v = 0.0;
    } else {
        spring_step(&mut a.hd_gv, gv_t, swap_rate, dt);
        if !has_hold && a.hd_on && a.hd_lo.x > 0.0 { a.hd_lo.x = m::min(1.0, a.hd_lo.x / m::max(w_hd, 0.05)); }
    }
    a.hd_on = has_hold;
    let has_held = a.held_pose.is_some();
    let he_v = a.held_pose.as_ref().map_or(a.he_last.variant, |h| h.variant);
    let he_t = if he_v == Some(HoldVariant::Gun) { 1.0 } else { 0.0 };
    if has_held && !a.he_on {
        a.he_gv.x = he_t; a.he_gv.v = 0.0; a.he_lk.x = 0.0; a.he_lk.v = 0.0; a.he_kc.x = 0.0; a.he_kc.v = 0.0; a.he_bo.x = 0.0; a.he_bo.v = 0.0; a.he_bc.x = 0.0; a.he_bc.v = 0.0;
        a.he_lo.x *= w_he; a.he_lo.v = 0.0; a.he_hu.x *= w_he; a.he_hu.v = 0.0;
        a.he_hr[0] = None; a.he_hr[1] = None;
    } else {
        spring_step(&mut a.he_gv, he_t, swap_rate, dt);
        if !has_held && a.he_on && a.he_lo.x > 0.0 { a.he_lo.x = m::min(1.0, a.he_lo.x / m::max(w_he, 0.05)); }
    }
    a.he_on = has_held;
    a.hold_gun_side = a.hd_gv.x >= 0.5;
    let gun_hold = w_hd > 0.003 && a.hold_gun_side;
    a.hide_held_item = w_lp > 0.5 || w_hl > 0.5 || (w_hd > 0.5 && !gun_hold) || a.big_tool.as_ref().map_or(false, big_off_tool);
    { // CREEP.wide: slow in, and only while he moves; fast out
        let want = v3::clamp(a.wide, 0.0, 1.0);
        let inn = want >= a.wide_s;
        if !inn || m::abs(a.speed) > cr.wide.still {
            a.wide_s = if js::truthy(want) || a.wide_s > 1e-3 { v3::damp(a.wide_s, want, if inn { cr.wide.in_ } else { cr.wide.out }, dt) } else { 0.0 };
        }
    }
    a.low_s = if a.low > 0.0 || a.low_s > 1e-3 { v3::damp(a.low_s, v3::clamp(a.low, 0.0, 1.0), if a.low <= 0.0 && a.spring > 0.0 { cr.low.up } else { cr.low.rate }, dt) } else { 0.0 };
    a.crouch = v3::damp(a.crouch, m::max_n(&[a.crouch_target, v3::clamp(a.coil, 0.0, 1.0), if a.lockpick { 1.0 } else { 0.0 }]) * (1.0 - (if a.lockpick { 0.0 } else { v3::clamp(a.rise, 0.0, 1.0) })), if a.spring > 0.0 { a.spring } else { 10.0 }, dt);
    let s = m::abs(a.speed) * (1.0 - w_k);
    let crouch_pose = if a.bw.is_some() || a.tk.is_some() {
        let cp = a.crouch * (1.0 - w_kslow);
        anim_take_pre(g, c, dt, cp)
    } else { a.crouch * (1.0 - w_kslow) };
    // ---- deliberate movement, and the crouch's centre of gravity by pace
    let a = &mut g.ch[c].anim;
    let d_ = v3::clamp(spring_step(&mut a.dlb, v3::clamp(a.deliberate, 0.0, 1.0), 15.0, dt), 0.0, 1.0);
    let off_wall = 1.0 - v3::clamp(a.st.w.x, 0.0, 1.0) * v3::clamp(a.wst.wl.x, 0.0, 1.0);
    // the held gait (creep)
    let g_h = if a.creep || a.cr_t > 0.0 { anim_creep_gait(g, c, dt) } else { 0.0 };
    let a = &mut g.ch[c].anim;
    let mut mv = v3::smoothstep(0.15, 0.5, s);
    let mut pace_s = v3::smoothstep(0.38, 0.95, s / pr.crouch_speed);
    if g_h > 0.0 {
        if a.cr_v > 0.05 && s > 0.05 { a.cr_pace = v3::damp(a.cr_pace, pace_s, 10.0, dt); }
        mv = v3::lerp(mv, 1.0, g_h);
        pace_s = v3::lerp(pace_s, a.cr_pace, g_h);
    } else { a.cr_pace = pace_s; }
    let dl = d_ * mv * off_wall * crouch_pose;
    let pace = pace_s * mv * off_wall * crouch_pose;
    let soft = d_ * mv * off_wall * (1.0 - crouch_pose);
    let arms_in = d_ * off_wall * v3::lerp(0.4, 1.0, mv);
    let dsh = if a.dash > 0.0 || a.dash_w > 0.002 {
        a.dash_w = v3::clamp(v3::damp(a.dash_w, v3::clamp(a.dash, 0.0, 1.0), dash_k_rate, dt), 0.0, 1.0);
        a.dash_w * mv * off_wall * crouch_pose
    } else { a.dash_w = 0.0; 0.0 };
    let stride_d = (1.0 + sn.stride * d_ * mv * off_wall) * (1.0 + dash_k_stride * dsh);
    // ---- locomotion: weights among idle / walk / jog by speed
    let bl = blend();
    let (mut w_walk, mut w_jog, mut w_sprint) = (0.0, 0.0, 0.0);
    if s <= bl.walk_top { w_walk = v3::smoothstep(bl.walk_in0, bl.walk_in1, s); }
    else if s <= bl.jog_at { w_jog = v3::smoothstep(0.0, 1.0, (s - bl.walk_top) / (bl.jog_at - bl.walk_top)); w_walk = 1.0 - w_jog; }
    else { w_sprint = v3::clamp((s - bl.jog_at) / (pr.sprint_speed - bl.jog_at), 0.0, 1.0); w_jog = 1.0 - w_sprint; }
    let mut w_idle = 1.0 - w_walk - w_jog - w_sprint;
    if g_h > 0.0 { let hold = w_idle * g_h; w_walk += hold; w_idle -= hold; }
    // phase advance: rate so that feet match ground speed
    let rw = run_warp();
    let kj = if w_jog > 0.001 { v3::clamp(v3::lerp(1.0, rw.stride, w_jog + w_sprint), rw.stride, 1.0) } else { 1.0 };
    a.run_k = kj;
    let d_walk = clip_dur(g, c, "Walk_Loop");
    let d_jog = clip_dur(g, c, "Jog_Fwd_Loop");
    let d_crouch = clip_dur(g, c, "Crouch_Fwd_Loop");
    let d_sprint = clip_dur(g, c, "Sprint_Loop");
    let rate_walk = (s / pr.walk_speed) / d_walk;
    let rate_jog = (s / pr.jog_speed) / d_jog / kj;
    let rate_crouch = (s / pr.crouch_speed) / d_crouch / stride_d;
    let rate_sprint = (s / pr.sprint_speed) / d_sprint;
    let wsum = m::max(1e-3, w_walk + w_jog + w_sprint);
    let a = &mut g.ch[c].anim;
    let mut rate = (w_walk * rate_walk + w_jog * rate_jog + w_sprint * rate_sprint) / wsum;
    rate = v3::lerp(rate, rate_crouch, a.crouch);
    if s < 0.05 { rate = 0.0; }
    if g_h > 0.0 { rate = v3::lerp(rate, v3::lerp((w_walk / (pr.walk_speed * d_walk) + w_jog / (pr.jog_speed * d_jog * kj) + w_sprint / (pr.sprint_speed * d_sprint)) / wsum, 1.0 / (pr.crouch_speed * d_crouch * stride_d), a.crouch) * a.cr_v, g_h); }
    let rev = if a.reverse { -1.0 } else { 1.0 };
    a.phase = js::rem(a.phase + dt * rate * rev + 100.0, 1.0);
    if a.tap_u >= 0.0 && g_h > 0.0 {   // (tapstep.ts: the stride belongs to the step)
        if a.tap_was < 0.0 { a.tap_seen = a.tap_n; a.tap_from = a.phase; a.tap_to = a.phase; }
        else if a.tap_n != a.tap_seen {
            a.tap_seen = a.tap_n;
            let p = anim_creep_plant(g, c)[1];
            let a = &mut g.ch[c].anim;
            let w = a.tap_way;
            let off = js::rem(js::rem(a.phase - p, 0.5) + 0.5, 0.5);
            let go = if w > 0.0 { 0.5 - off } else { off };
            a.tap_from = a.phase;
            a.tap_to = a.phase + w * (if go < 0.2 { go + 0.5 } else { go });
        }
        let a = &mut g.ch[c].anim;
        a.phase = js::rem(a.tap_from + (a.tap_to - a.tap_from) * v3::clamp(a.tap_u, 0.0, 1.0) + 100.0, 1.0);
    }
    let a = &mut g.ch[c].anim;
    a.tap_was = if g_h > 0.0 { a.tap_u } else { -1.0 };
    if g.k.wallsnap.on && g.k.wallsnap.pose {   // the wall hold's own step clock, signed along the wall
        if a.stance == Stance::Wall {
            let st_ = wall_pose().gait.step;
            a.wall_gait = js::rem(a.wall_gait + dt * s * a.stack_side * rev / (2.0 * v3::lerp(st_[0], st_[1], v3::clamp(a.crouch, 0.0, 1.0))) + 100.0, 1.0);
        } else { a.wall_gait = WALL_GAIT_REST; }
    }
    a.idle_t += dt;
    // the linger (deliberate, crouched) and the run's stance linger: the cycle sampled at a phase eased about the true one
    let rl = run_linger();
    let four_pi = 4.0 * m::PI;
    let a_l = sn.linger * dl;
    let th = four_pi * (a.phase - a.gait.linger);
    let w_run = v3::smoothstep(0.08, 0.45, w_jog) * (1.0 - crouch_pose);
    let a_r = rl.depth * w_run;
    let th_r = four_pi * (a.phase - v3::lerp(a.run_gait.jog.c, a.run_gait.sprint.c, w_sprint));
    a.lg.run = v3::smoothstep(0.08, 0.45, w_jog + w_sprint) * (1.0 - crouch_pose);
    a.lg.sprint = w_sprint * (1.0 - crouch_pose);
    let ph = if a_l > 1e-5 || a_r > 1e-5 {
        js::rem(a.phase - a_l / four_pi * m::sin(th) - a_r * (m::sin(th_r) - m::sin(3.0 * th_r) / 27.0) / (8.0 / 9.0) / four_pi + 1.0, 1.0)
    } else { a.phase };
    let surge = if a_l > 1e-5 { -a_l * sn.surge * stride_d * (1.0 + sn.pace_stride * pace) * pr.crouch_speed * d_crouch / four_pi * m::sin(th) * rev } else { 0.0 };
    // stand pose
    anim_stand_pose(g, c, ph, w_idle, w_walk, w_jog, w_sprint);
    let a = &g.ch[c].anim;
    if g.k.runarms.on && a.lg.run > 1e-3 && a.lg.w.x < 0.999 { let (r, sp) = (a.lg.run, a.lg.sprint); seam_run_arms(g, c, r, sp); }
    if w_run > 1e-3 && soft_knee().on > 0.5 { seam_soft_knees(g, c, w_run); }
    // the run's stance feet pinned to the floor and the ground's speed through each stance window
    let w_pin = if rl.depth > 0.0 { w_run } else { 0.0 };
    let a = &mut g.ch[c].anim;
    a.plant_w = if w_pin >= a.plant_w { w_pin } else { a.plant_w * m::exp(-dt / 0.04) };
    if a.plant_w < 1e-3 { a.plant_w = 0.0; }
    if a.plant_w > 0.0 || a.plant_l.dev != 0.0 || a.plant_r.dev != 0.0 { let pw = a.plant_w; anim_run_plant(g, c, ph, s, dt, pw, w_sprint); } else { a.plant_l.pz = f64::NAN; a.plant_r.pz = f64::NAN; }
    // ---- stance springs (run always: they hold the shape the arms morph through)
    let a = &mut g.ch[c].anim;
    const SW: f64 = 13.0;
    let run_low = a.run_ready && a.stance == Stance::None;
    let stance = if run_low { Stance::LowReady } else { a.stance };
    let held = if kt >= 0.0 && kt < 0.55 { Stance::HighReady } else { stance };
    let shape = if held != Stance::None { Some(stance_shape(held)) } else { None };
    let side = a.stack_side;
    let w_was = m::max(v3::clamp(a.st.w.x, 0.0, 1.0), w_kslow);
    let st_rate = if stance == Stance::None && a.stance_out > 0.0 { a.stance_out } else if a.stance_in > 0.0 && stance != Stance::None { a.stance_in } else if stance == Stance::Wall || a.stance_seen == Stance::Wall { 22.0 } else { SW };
    let w_st = v3::clamp(spring_step(&mut a.st.w, if stance != Stance::None { 1.0 } else { 0.0 }, st_rate, dt), 0.0, 1.0);
    if a.stance_out > 0.0 && (stance != Stance::None || w_st < 0.02) { a.stance_out = 0.0; }
    if a.stack_press != a.stack_press_sm { a.stack_press_sm += v3::clamp(a.stack_press - a.stack_press_sm, -0.9 * dt, 0.9 * dt); }
    let armed = a.upper == UpperMode::Relaxed || a.upper == UpperMode::Aim || a.shoot_t >= 0.0;
    let park_now = 1.0 - v3::smoothstep(0.12, 0.7, s);
    if let Some(sh) = shape {
        let snap = w_was < 0.02 && a.stance_seen != held;
        if snap {   // taking a shape from (next to) nothing: start AT it
            a.st.fwd.v = 0.0; a.st.fwd.x = sh.fwd;
            a.st.up.v = 0.0; a.st.up.x = sh.up;
            a.st.side.v = 0.0; a.st.side.x = sh.side - 0.02 * side * (sh.blade / 22.0);
            a.st.pitch.v = 0.0; a.st.pitch.x = sh.pitch;
            a.st.lean.v = 0.0; a.st.lean.x = sh.lean;
            a.st.head_pitch.v = 0.0; a.st.head_pitch.x = sh.head_pitch;
            a.st.tuck.v = 0.0; a.st.tuck.x = sh.tuck;
            a.st.blade.v = 0.0; a.st.blade.x = -side * sh.blade;
            a.st.head_yaw.v = 0.0; a.st.head_yaw.x = -side * sh.head_yaw;
            a.st.squat.v = 0.0; a.st.squat.x = sh.squat;
            a.wst.wl.v = 0.0; a.wst.side.v = 0.0; a.wst.peek.v = 0.0; a.wst.park.v = 0.0;
            a.wst.wl.x = if held == Stance::Wall { 1.0 } else { 0.0 };
            a.wst.side.x = side;
            a.wst.peek.x = v3::clamp(a.wall_peek, 0.0, 1.0);
            a.wst.park.x = park_now;
        }
        a.stance_seen = held;
        let wt_ = wall_tuck();
        let tp = wt_.pistol;
        let gw = if held == Stance::LowReady && a.gun_wall > 1e-4 { v3::clamp(a.gun_wall, 0.0, 1.0) } else { 0.0 };
        let gwo = if js::truthy(gw) { v3::clamp(a.gun_wall - 1.0, 0.0, 1.0) * wt_.over } else { 0.0 };
        let (rr_fwd, rr_up, rr_pitch, rr_tuck) = if run_low && held == Stance::LowReady { (g.k.runready.fwd, g.k.runready.up, g.k.runready.pitch, g.k.runready.tuck) } else { (sh.fwd, sh.up, sh.pitch, sh.tuck) };
        let on = js::truthy(gw);
        spring_step(&mut a.st.fwd, if on { v3::lerp(rr_fwd, tp.fwd, gw) - gwo } else { rr_fwd }, SW, dt);
        spring_step(&mut a.st.up, if on { v3::lerp(rr_up, tp.up, gw) } else { rr_up }, SW, dt);
        spring_step(&mut a.st.side, sh.side - 0.02 * side * (sh.blade / 22.0), SW, dt);
        spring_step(&mut a.st.pitch, if on { v3::lerp(rr_pitch, tp.pitch, gw) } else { rr_pitch }, SW, dt);
        spring_step(&mut a.st.lean, sh.lean, SW, dt);
        spring_step(&mut a.st.head_pitch, sh.head_pitch, SW, dt);
        spring_step(&mut a.st.tuck, if on { v3::lerp(rr_tuck, tp.tuck, gw) } else { rr_tuck }, SW, dt);
        spring_step(&mut a.st.blade, -side * sh.blade, SW * 0.8, dt);
        spring_step(&mut a.st.head_yaw, -side * sh.head_yaw, SW, dt);
        spring_step(&mut a.st.squat, sh.squat, SW * 0.8, dt);
        spring_step(&mut a.wst.wl, if held == Stance::Wall { 1.0 } else { 0.0 }, if a.stance_in > 0.0 && held == Stance::Wall { a.stance_in } else { 22.0 }, dt);
        spring_step(&mut a.wst.side, side, SW, dt);
        spring_step(&mut a.wst.peek, v3::clamp(a.wall_peek, 0.0, 1.0), 20.0, dt);
        let park_rate = if park_now < a.wst.park.x { 30.0 } else { 12.0 };
        spring_step(&mut a.wst.park, park_now, park_rate, dt);
    }
    let w_carry = m::max(w_st, w_kslow);
    let wl = v3::clamp(a.wst.wl.x, 0.0, 1.0);
    // ---- the long gun's carriage
    let mut w_lg = 0.0;
    let mut lg_want: Option<LongStance> = None;
    if a.long_gun || a.lg.w.x > 0.002 {
        let want = if a.thr_off && a.throw_t >= 0.0 && a.throw_t < THROW_DUR - 0.25 && a.shoot_t < 0.0 { LongStance::LowLong }
            else { match a.long_stance { Some(l) => l, None => if a.upper == UpperMode::Aim || a.shoot_t >= 0.0 { LongStance::Shoulder } else if held == Stance::LowReady { LongStance::LowLong } else if held == Stance::HighReady || held == Stance::Stack { LongStance::PortLong } else { LongStance::PatrolLong } } };
        let w_lg_was = v3::clamp(a.lg.w.x, 0.0, 1.0);
        let drew = a.long_gun && (a.lg.seen.is_none() || w_lg_was < 1.0 && !a.lg_armed_was);
        if drew {
            if !anim_kit_rifle(g, c) { let a = &mut g.ch[c].anim; a.lg.w.x = 1.0; a.lg.w.v = 0.0; }
            let a = &mut g.ch[c].anim;
            a.lg.lat[0] = f64::NAN; a.lg.lat[1] = f64::NAN;
        }
        let a = &mut g.ch[c].anim;
        w_lg = v3::clamp(spring_step(&mut a.lg.w, if a.long_gun { 1.0 } else { 0.0 }, SW, dt), 0.0, 1.0);
        a.lg_armed_was = a.long_gun;
        let mut tgt = long_targets(want, crouch_pose);
        if a.gun_wall > 1e-4 && (want == LongStance::LowLong || want == LongStance::PatrolLong) { long_tuck(&mut tgt, a.gun_wall); }
        if drew || a.lg.seen.is_none() || (w_lg_was < 0.02 && a.lg.seen != Some(want)) {
            let from = if a.lg.seen.is_none() && want == LongStance::Shoulder && w_st * wl > 0.1 { long_targets(LongStance::LowLong, crouch_pose) } else { tgt };
            for (i, sp) in a.lg.ch.iter_mut().enumerate() { sp.x = from[i]; sp.v = 0.0; }
        } else {
            for (i, sp) in a.lg.ch.iter_mut().enumerate() { spring_step(sp, tgt[i], SW, dt); }
        }
        a.lg.seen = Some(want);
        a.long_shown = if w_lg > 0.5 { Some(want) } else { None };
        if a.long_gun { lg_want = Some(want); }
        anim_reload_carry(g, c, dt, want);
    } else {
        a.lg.seen = None; a.long_shown = None;
        a.rlc.k = 0.0; a.rlc.h = 0.0; a.rl_shown.carry = 0.0; a.rl_shown.head = 0.0;
    }
    let a = &mut g.ch[c].anim;
    let w_wall_arms = w_st * wl * (if a.stance == Stance::Wall { 1.0 } else { 1.0 - w_lg });
    // crouch pose (the kick stands him up: crouch_pose above)
    if crouch_pose > 0.001 {
        let w_move = if g_h > 0.0 { v3::lerp(v3::clamp(s / (pr.crouch_speed * 0.6), 0.0, 1.0), 1.0, g_h) } else { v3::clamp(s / (pr.crouch_speed * 0.6), 0.0, 1.0) };
        anim_crouch_laid(g, c, ph, w_move, crouch_pose);
        g.ch[c].anim.cr_walk = crouch_pose * w_move;
    } else { a.cr_walk = 0.0; }
    // ---- MOVES: the crouch walk carried level (steadyLayer)
    {
        let sw = if mv_on(g, MvOnK::Steady) { g.ch[c].anim.cr_walk * off_wall } else { 0.0 };
        let steady = g.k.moves.steady;
        let (rise, rise_rate) = (g.k.mv.steady.rise, g.k.mv.steady.rise_rate);
        let a = &mut g.ch[c].anim;
        if steady > 1.5 || a.steady_up > 0.0 {
            a.steady_up = v3::damp(a.steady_up, if steady > 1.5 { rise * sw } else { 0.0 }, rise_rate, dt);
            if a.steady_up < 1e-5 && sw <= 0.003 { a.steady_up = 0.0; }
        }
        if sw > 0.003 || a.steady_up > 0.0 {
            let sq = w_st * (1.0 - wl) * (1.0 - w_kstand);
            let sunk = v3::smoothstep(g.k.mv.steady.deep[0], g.k.mv.steady.deep[1], sn.drop * dl + sn.pace_drop * pace + v3::lerp(cr.drop[0], cr.drop[1], crouch_pose) * v3::clamp(a.cr_w, 0.0, 1.0));
            let base = (1.0 - v3::smoothstep(0.0, 0.01, (m::max(0.0, a.st.squat.x) + m::abs(a.st.blade.x) * 0.003) * sq)) * (1.0 - sunk);
            anim_steady_layer(g, c, if sw > 0.003 { sw } else { 0.0 }, Some(base), Some(sunk));
        }
    }
    // the hand pass: where the locomotion has left every digit, before any layer shapes a hand
    {
        let a = &mut g.ch[c].anim;
        let n = a.p_a.rows;
        for sd in 0..2 {
            for k in 0..15 {
                let o = 3 * n + a.hn.nodes[sd][k] as usize;
                a.hn.base[sd][k * 4] = a.p_a.cols[o];
                a.hn.base[sd][k * 4 + 1] = a.p_a.cols[o + n];
                a.hn.base[sd][k * 4 + 2] = a.p_a.cols[o + 2 * n];
                a.hn.base[sd][k * 4 + 3] = a.p_a.cols[o + 3 * n];
            }
            a.hn.note[sd].fill(f32::INFINITY);
        }
    }
    // ---- life: breathing, the body's mass, the weight shifting; and the standing foot lock
    {
        let body = (1.0 - w_st) * (1.0 - w_hd) * (1.0 - w_he) * (1.0 - w_lp);
        let hard = {
            let a = &g.ch[c].anim;
            if kt >= 0.0 || a.throw_t >= 0.0 || a.hit_t >= 0.0 || a.strike_t >= 0.0 || anim_mv_busy(g, c) { 0.0 } else { 1.0 }
        };
        let a = &mut g.ch[c].anim;
        a.life.gate = v3::damp(a.life.gate, hard, 12.0, dt);
        a.feet.gate = v3::damp(a.feet.gate, if kt >= 0.0 { 0.0 } else { 1.0 }, 12.0, dt);
        let (lg_, fg_) = (body * a.life.gate, body * a.feet.gate);
        anim_life_layer(g, c, dt, s, crouch_pose, lg_, fg_);
    }
    // ---- upper body layers
    let a = &mut g.ch[c].anim;
    let mut tgt_relaxed = if a.upper == UpperMode::Relaxed { 1.0 } else { 0.0 };
    let mut tgt_aim = if a.upper == UpperMode::Aim { 1.0 } else { 0.0 };
    let mut tgt_torch = if a.upper == UpperMode::Torch { 1.0 } else { 0.0 };
    if (a.shoot_t >= 0.0 || lg_want == Some(LongStance::Shoulder)) && w_hd <= 0.003 { tgt_aim = 1.0; tgt_relaxed = 0.0; tgt_torch = 0.0; }
    if a.throw_t >= 0.0 && a.throw_t < THROW_DUR - 0.25 { tgt_aim = 0.0; }
    let (snap_r, snap_a, snap_t) = (a.upper_w.relaxed, a.upper_w.aim, a.upper_w.torch);
    a.upper_w.relaxed = v3::damp(a.upper_w.relaxed, tgt_relaxed, 9.0, dt);
    let kit_rifle = anim_kit_rifle(g, c);
    let kit_trunk = g.k.kit.trunk;
    let a = &mut g.ch[c].anim;
    a.upper_w.aim = v3::damp(a.upper_w.aim, tgt_aim, if kit_rifle { kit_trunk } else { 14.0 }, dt);
    a.upper_w.torch = v3::damp(a.upper_w.torch, tgt_torch, 9.0, dt);
    if a.shoot_t == 0.0 { let on = w_lg <= 0.0 && mv_on(g, MvOnK::Snap); g.ch[c].anim.snap_shot = on; }
    if w_lg <= 0.0 && mv_on(g, MvOnK::Snap) {   // the pistol's carry and its aim TRADE (MOVES.snap)
        let (s_in, s_ease, s_cover, s_under) = (g.k.mv.snap.in_, g.k.mv.snap.ease, g.k.mv.snap.cover, g.k.mv.snap.under);
        let over = |x: f64| v3::smoothstep(s_cover[0], m::max(s_cover[1], s_cover[0] + 1e-6), x);
        let u = &mut g.ch[c].anim.upper_w;
        if tgt_aim > 0.0 {
            if u.aim > snap_a { u.aim = v3::lerp(snap_a, u.aim, v3::lerp(v3::clamp(s_in, 0.0, 1.0), 1.0, v3::smoothstep(0.0, js::or(s_ease, 1e-6), snap_a))); }
            let k = over(u.aim);
            if u.relaxed < snap_r { u.relaxed = v3::lerp(snap_r, u.relaxed, k); }
            if u.torch < snap_t { u.torch = v3::lerp(snap_t, u.torch, k); }
        } else if snap_a > 0.003 && (tgt_relaxed > 0.0 || tgt_torch > 0.0) {
            let hid = m::max(0.0, s_under) * over(snap_a);
            let back = if tgt_relaxed > 0.0 { u.relaxed = v3::damp(u.relaxed, 1.0, hid, dt); u.relaxed } else { u.torch = v3::damp(u.torch, 1.0, hid, dt); u.torch };
            if u.aim < snap_a { u.aim = v3::lerp(snap_a, u.aim, over(back)); }
        }
    }
    let upper_w_relaxed = g.ch[c].anim.upper_w.relaxed;
    if upper_w_relaxed > 0.003 {
        let (clip, t) = (clip_of(g, c, "Pistol_Idle_Loop"), g.ch[c].anim.idle_t);
        crate::anim_skeleton::sample_clip(&g.tab.clip[clip], t, true, &mut g.ch[c].anim.p_upper);
        let a = &mut g.ch[c].anim;
        crate::anim_skeleton::blend_pose(None, &a.p_upper, upper_w_relaxed, Some(&a.upper_mask), &mut a.p_a, singles);
    }
    let a = &mut g.ch[c].anim;
    a.tu_k = v3::clamp(a.tu_k + (if tgt_torch > 0.0 { dt } else { -dt }) / 0.4, 0.0, 1.0);
    let upper_w_torch = a.upper_w.torch;
    if upper_w_torch > 0.003 {
        let (clip, t) = (clip_of(g, c, "Idle_Torch_Loop"), g.ch[c].anim.idle_t);
        crate::anim_skeleton::sample_clip(&g.tab.clip[clip], t, true, &mut g.ch[c].anim.p_upper);
        let a = &mut g.ch[c].anim;
        crate::anim_skeleton::blend_pose(None, &a.p_upper, upper_w_torch, Some(&a.torso_mask), &mut a.p_a, singles);
    }
    let tu_k = g.ch[c].anim.tu_k;
    if tu_k > 0.0 { anim_torch_up_layer(g, c, v3::smoothstep(0.0, 1.0, tu_k)); }
    // ---- deliberate movement / the crouch's centre of gravity
    if dl > 0.002 || pace > 0.002 || soft > 0.002 || arms_in > 0.003 || dsh > 0.002 { anim_sneak_layer(g, c, dl, pace, soft, arms_in, surge, Some(dsh)); }
    // ---- MOVES: the recoil's sums bled off, and how each side carries itself
    let mv_g = anim_mv_pre(g, c, dt, mv * off_wall * (1.0 - crouch_pose), (1.0 - crouch_pose) * off_wall * (1.0 - w_kslow));
    // ---- procedural: stance body, then the kick, then the pistol carry arms
    if w_st * (1.0 - wl) > 0.003 { anim_stance_body(g, c, w_st * (1.0 - wl), s, w_kstand); }
    if w_st * wl > 0.003 { anim_wall_body(g, c, w_st * wl); }
    if kt >= 0.0 { anim_kick_body(g, c, kt, w_kbody); }
    if w_carry * (1.0 - w_st * wl) * (1.0 - w_lg) > 0.003 { anim_carry_arms(g, c, w_carry * (1.0 - w_st * wl), kt); }
    if w_lg * (1.0 - w_wall_arms) > 0.003 {   // the long gun: trunk set to the carriage, then both arms onto the weapon
        let aim = g.ch[c].anim.upper_w.aim;
        if mv_g.live { anim_mv_trunk(g, c, w_lg * (1.0 - w_st * wl) * (1.0 - aim), mv_g, false); }
        let (run, sprint) = (g.ch[c].anim.lg.run, g.ch[c].anim.lg.sprint);
        let aim = g.ch[c].anim.upper_w.aim;
        anim_long_body(g, c, w_lg * (1.0 - w_st * wl) * (1.0 - aim), Some(run), Some(sprint));
        let aim = g.ch[c].anim.upper_w.aim;
        if aim < 0.995 { anim_long_arms(g, c, w_lg * (1.0 - w_wall_arms), kt, dt, false, aim <= 0.003); }
    }
    if w_wall_arms > 0.003 { anim_wall_arms(g, c, w_wall_arms); }
    if w_lp > 0.003 { anim_lockpick_layer(g, c, w_lp); }
    if w_hd > 0.003 { anim_hold_layer(g, c, w_hd, dt); } else {
        let a = &mut g.ch[c].anim;
        a.hd_kn.x = 0.0; a.hd_kn.v = 0.0; a.hd_ck.x = 0.0; a.hd_ck.v = 0.0; a.hd_lo.x = 0.0; a.hd_lo.v = 0.0; a.hd_kc.x = 0.0; a.hd_kc.v = 0.0; a.hd_hu.x = 0.0; a.hd_hu.v = 0.0;
    }
    if w_he > 0.003 { anim_held_layer(g, c, w_he, dt); } else {
        let a = &mut g.ch[c].anim;
        a.he_lo.x = 0.0; a.he_lo.v = 0.0; a.he_lk.x = 0.0; a.he_lk.v = 0.0; a.he_kc.x = 0.0; a.he_kc.v = 0.0; a.he_bo.x = 0.0; a.he_bo.v = 0.0; a.he_bc.x = 0.0; a.he_bc.v = 0.0; a.he_hu.x = 0.0; a.he_hu.v = 0.0;
        a.he_hr[0] = None; a.he_hr[1] = None;
    }
    // the chest twist the skeleton applies after this pose: worked out here because the door hand has to be posed through its inverse
    let a = &g.ch[c].anim;
    let y0 = a.aim_yaw_offset * (1.0 - w_st * wl * (1.0 - a.upper_w.aim));
    let p = a.aim_pitch * (a.upper_w.aim);
    // ---- a hand on the door he is pushing open
    let mut y;
    let mut head_keep;
    {
        let a = &mut g.ch[c].anim;
        let d = a.door_push.clone();
        if d.is_some() { a.dp.last = d.clone(); }
        let gate = (1.0 - w_st * wl) * (1.0 - w_hd) * (1.0 - w_he) * (1.0 - w_lp) * (1.0 - a.upper_w.aim) * (if kt >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.reload_t >= 0.0 { 0.0 } else { 1.0 });
        let intent = match &d { Some(dd) => v3::clamp(dd.w, 0.0, 1.0) * gate, None => 0.0 };
        let square = if intent > 0.003 && gate > 0.5 && anim_to_model_p(g, c, d.as_ref().unwrap().p)[2] > 0.1 { 1.0 } else { 0.0 };
        let a = &mut g.ch[c].anim;
        let tw = v3::clamp(spring_step(&mut a.dp.tw, square, if square > 0.0 { 24.0 } else { 14.0 }, dt), 0.0, 1.0);
        y = y0 * (1.0 - tw * (1.0 - a.upper_w.aim));
        head_keep = v3::clamp(y0 - y, -0.7, 0.7);
        anim_door_layer(g, c, dt, intent, armed, y, -p * 0.35);
    }
    // ---- hands at work on a thing; the swap's clock is read off `carry` the same way (a dt-0 call is a re-bake: it takes nothing)
    {
        let bake = dt <= 0.0;
        let a = &mut g.ch[c].anim;
        if !bake {
            if a.work.is_some() { a.wk.last = a.work.clone(); a.wk.fed = true; a.wk.after = 0.0; }
            else if a.wk.after > 0.0 && a.wk.last.is_some() {
                a.wk.after -= dt;
                let l = a.wk.last.as_mut().unwrap();
                l.t = m::max(l.t, l.dur) + dt;
                a.wk.fed = true;
            } else { a.wk.fed = false; }
            a.work = None;
        }
        if let Some(lp) = a.wk.last.as_ref().map(|l| l.p) {
            let sq = if a.wk.fed && w_st * wl < 0.5 && anim_to_model_p(g, c, lp)[2] > 0.1 { 1.0 } else { 0.0 };
            let a = &mut g.ch[c].anim;
            let tw = v3::clamp(spring_step(&mut a.wk.tw, sq, if sq > 0.0 { 24.0 } else { 14.0 }, dt), 0.0, 1.0);
            if tw > 0.003 {
                let yk = y0 * (1.0 - tw * (1.0 - a.upper_w.aim));
                if m::abs(yk) < m::abs(y) { y = yk; head_keep *= 1.0 - tw; }
            }
        } else { a.wk.tw.x = 0.0; a.wk.tw.v = 0.0; }
        let a = &mut g.ch[c].anim;
        a.sw.shown = false;
        a.sw.belt = 0.0;
        seam_swap_clock(g, c, bake, dt);
    }
    let a = &g.ch[c].anim;
    if w_hl > 0.003 && (a.haul.is_some() || a.hold_pose.is_none()) { anim_haul_layer(g, c, w_hl, y, -p * 0.35); }
    let upper_w_aim = g.ch[c].anim.upper_w.aim;
    if upper_w_aim > 0.003 {
        // pitch: blend aim down / neutral / up, the remainder handled by the twist
        let pn = v3::clamp(g.ch[c].anim.aim_pitch / (45.0 * m::PI / 180.0), -1.0, 1.0);
        let clip = clip_of(g, c, "Pistol_Aim_Neutral");
        at0_upper(g, c, clip);
        if pn > 0.01 { let clip = clip_of(g, c, "Pistol_Aim_Up"); at0_b(g, c, clip); let a = &mut g.ch[c].anim; crate::anim_skeleton::blend_pose(None, &a.p_b, pn, None, &mut a.p_upper, singles); }
        else if pn < -0.01 { let clip = clip_of(g, c, "Pistol_Aim_Down"); at0_b(g, c, clip); let a = &mut g.ch[c].anim; crate::anim_skeleton::blend_pose(None, &a.p_b, -pn, None, &mut a.p_upper, singles); }
        let shoot_t = g.ch[c].anim.shoot_t;
        if shoot_t >= 0.0 {
            let w = if shoot_t < 0.04 { shoot_t / 0.04 } else { v3::clamp(1.0 - (shoot_t - 0.3) / 0.3, 0.0, 1.0) };
            let clip = clip_of(g, c, "Pistol_Shoot");
            if g.ch[c].anim.snap_shot && m::abs(pn) > 0.01 {   // MOVES.snap, a shot along a pitched aim: its recoil alone, laid on the pitched pose
                let a = &mut g.ch[c].anim;
                a.p_b.cols.clone_from(&a.p_aim.cols);
                crate::anim_skeleton::sample_clip(&g.tab.clip[clip], shoot_t, false, &mut g.ch[c].anim.p_b);
                let a = &mut g.ch[c].anim;
                add_pose_delta(&mut a.p_upper, &a.p_aim, &a.p_b, w, &a.upper_mask);
            } else {
                crate::anim_skeleton::sample_clip(&g.tab.clip[clip], shoot_t, false, &mut g.ch[c].anim.p_b);
                let a = &mut g.ch[c].anim;
                crate::anim_skeleton::blend_pose(None, &a.p_b, w, None, &mut a.p_upper, singles);
            }
        }
        let aim = g.ch[c].anim.upper_w.aim;
        if w_lg <= 0.0 { let a = &mut g.ch[c].anim; crate::anim_skeleton::blend_pose(None, &a.p_upper, aim, Some(&a.upper_mask), &mut a.p_a, singles); }
        else {   // the rifleman: the aim clip's trunk and head; its pistol arms only by what the long gun has not got of them
            let a = &mut g.ch[c].anim;
            crate::anim_skeleton::blend_pose(None, &a.p_upper, aim, Some(&a.torso_mask), &mut a.p_a, singles);
            crate::anim_skeleton::blend_pose(None, &a.p_upper, aim * (1.0 - w_lg), Some(&a.arms_mask), &mut a.p_a, singles);
            let gate = w_lg * (1.0 - w_hd) * (1.0 - w_he) * (1.0 - w_lp);
            if gate * (1.0 - w_wall_arms) > 0.003 {
                if mv_g.live { let aim = g.ch[c].anim.upper_w.aim; anim_mv_trunk(g, c, gate * (1.0 - w_st * wl) * aim, mv_g, true); }
                let aim = g.ch[c].anim.upper_w.aim;
                anim_long_body(g, c, gate * (1.0 - w_st * wl) * aim, None, None);
                anim_long_arms(g, c, gate * (1.0 - w_wall_arms), kt, dt, true, true);
            }
        }
    }
    if g.ch[c].anim.shoot_t >= 0.0 {
        let d = clip_dur(g, c, "Pistol_Shoot");
        let a = &mut g.ch[c].anim;
        a.shoot_t += dt;
        if a.shoot_t > d { a.shoot_t = -1.0; }
    }
    let a = &mut g.ch[c].anim;
    if a.sig_t >= 0.0 {
        let t = a.sig_t;
        anim_signal_layer(g, c, t);
        let a = &mut g.ch[c].anim;
        a.sig_t += dt;
        if a.sig_t > signal_secs(a.sig_kind) { a.sig_t = -1.0; }
    }
    if g.ch[c].anim.tell_t >= 0.0 {   // the idle tells: skipped (the clock still runs out) under anything that has the arms or the head
        let a = &g.ch[c].anim;
        let first = w_hd > 0.003 || w_he > 0.003 || w_lp > 0.003 || kt >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.hit_t >= 0.0 || a.reload_t >= 0.0 || a.upper_w.aim > 0.003 || a.sig_t >= 0.0;
        if first || anim_mv_busy(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0 { g.ch[c].anim.tell_shown = 0.0; } else { let t = g.ch[c].anim.tell_t; anim_tell_layer(g, c, t); }
        let a = &mut g.ch[c].anim;
        a.tell_t += dt;
        if a.tell_t > tell_secs(a.tell_k) { a.tell_t = -1.0; a.tell_shown = 0.0; }
    }
    let a = &g.ch[c].anim;
    if a.wk.last.is_some() || a.sw.t >= 0.0 || a.rx.left > 0.0 || a.rx.w.x > 0.003 {   // handwork: the swap, the work hands, the radio hand, each stood down by whatever has the arms
        let first = w_hd > 0.003 || w_he > 0.003 || w_lp > 0.003 || w_hl > 0.003 || kt >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.hit_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0;
        let taken = first || anim_mv_hands(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0 || w_st * wl > 0.003;
        let a = &g.ch[c].anim;
        let rifle_up = a.long_gun && a.upper_w.aim > 0.3 && !a.stowed;
        if a.sw.t >= 0.0 { anim_swap_layer(g, c, dt, taken, y, -p * 0.35); }
        let stowed = g.ch[c].anim.stowed;
        anim_work_layer(g, c, dt, !taken && !rifle_up, armed && !stowed, y, -p * 0.35);
        let a = &g.ch[c].anim;
        let radio_free = !taken && !rifle_up && a.tell_t < 0.0 && a.shoot_t < 0.0 && !a.wk.fed && a.wk.shown < 0.003;
        anim_radio_layer(g, c, dt, radio_free);
    }
    let a = &g.ch[c].anim;
    if a.creep || a.cr_w > 0.0 {   // the close-in mode's carriage: under the ready pose, let go of by whatever else has the body or the arms
        let first = w_hd > 0.003 || w_he > 0.003 || w_lp > 0.003 || kt >= 0.0 || a.throw_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0 || a.tell_t >= 0.0;
        let busy = first || anim_mv_busy(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0 || w_st * wl > 0.003 || g.ch[c].anim.long_gun || w_lg > 0.003;
        let kit_close_rate = g.k.kit.close.rate;
        let a = &mut g.ch[c].anim;
        let want = if a.creep && !busy { 1.0 } else { 0.0 };
        a.cr_w = if m::abs(want - a.cr_w) < 0.01 { want } else { a.cr_w + (want - a.cr_w) * m::min(1.0, dt * (if js::truthy(want) { cr.in_ } else { cr.out })) };
        let gun = v3::clamp(a.upper_w.relaxed + a.upper_w.aim + a.upper_w.torch + w_carry, 0.0, 1.0);
        if a.close_k >= 0.0 {
            a.cl_k = if a.cr_w > 0.003 { a.cl_k + (v3::clamp(a.close_k, 0.0, 1.0) - a.cl_k) * (1.0 - m::exp(-kit_close_rate * dt)) } else { v3::clamp(a.close_k, 0.0, 1.0) };
            a.cl_t += dt;
        }
        if a.cr_w > 0.003 {
            let w = if a.bw.is_some() { a.cr_w * (1.0 - a.bw_rise) } else { a.cr_w };
            let w_l = 1.0 - m::max(v3::clamp(a.upper_w.aim, 0.0, 1.0), w_carry);
            anim_creep_layer(g, c, w, w_l, 1.0 - gun, y, -p * 0.35, dt);
        } else { a.creep_shown = 0.0; }
    }
    let a = &g.ch[c].anim;
    if a.watch_at.is_some() || a.slip_at.is_some() || a.wt.w > 0.0 || a.wt.sw > 0.0 {   // WATCH: over the locomotion and the creep's carriage
        let first = w_hd > 0.003 || w_he > 0.003 || w_lp > 0.003 || w_hl > 0.003 || kt >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.hit_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0 || a.tell_t >= 0.0;
        let busy = first || anim_mv_busy(g, c) || g.ch[c].anim.bw.is_some() || g.ch[c].anim.tk.is_some();
        let a = &g.ch[c].anim;
        let gun_w = v3::clamp(a.upper_w.relaxed + a.upper_w.aim + a.upper_w.torch + w_carry, 0.0, 1.0);
        let aim_c = v3::clamp(a.upper_w.aim, 0.0, 1.0);
        anim_watch_layer(g, c, dt, if busy { 0.0 } else { 1.0 - aim_c }, 1.0 - m::max(aim_c, w_carry), 1.0 - gun_w, y, -p * 0.35);
    } else {
        let a = &mut g.ch[c].anim;
        a.watch_shown.w = 0.0; a.watch_shown.yaw = 0.0; a.watch_shown.slip = 0.0; a.watch_shown.lean = 1.0;
    }
    let a = &g.ch[c].anim;
    if a.stealth.is_some() || a.sp.is_some() {   // STEALTH POSES: over the crouch and the close-in carriage, under the ready tell; out, eased, for anything else that has his arms
        let first = w_hd > 0.003 || w_he > 0.003 || w_lp > 0.003 || w_hl > 0.003 || kt >= 0.0 || a.throw_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0 || a.tell_t >= 0.0 || a.hit_t >= 0.0 || a.strike_t >= 0.0 || a.bw.is_some() || a.tk.is_some() || a.clutch > 0.0 || a.bound != 0.0;
        let busy = first || anim_mv_busy(g, c) || {
            let a = &g.ch[c].anim;
            a.mvs.ts_t >= 0.0 || a.mvs.arm_l > 0.0 || w_st * wl > 0.003 || w_wall_arms > 0.003 || a.long_gun || w_lg > 0.003 || a.kit.is_some() || a.nv.as_ref().map_or(false, |n| n.hand) || a.act.is_some() || a.wound.is_some() || a.dazzle.is_some() || a.wk.fed || a.wk.shown > 0.003 || a.dp.w_l.x > 0.003 || a.dp.w_r.x > 0.003 || a.rx.w.x > 0.003 || a.upper_w.relaxed + a.upper_w.aim + a.upper_w.torch + w_carry > 0.003
        };
        let w_move = if crouch_pose > 0.001 { g.ch[c].anim.cr_walk / crouch_pose } else { 0.0 };
        anim_stealth_layer(g, c, dt, busy, crouch_pose, w_move, y, -p * 0.35);
    }
    if let Some(b) = g.ch[c].anim.bw.clone() { anim_blow_legs(g, c, &b); }
    let a = &g.ch[c].anim;
    if a.ready.is_some() || a.ready_w > 0.0 || a.rc_t > 0.0 {   // the ready pose: over the carries and the aim clip, under the one-shots below; stood down, eased, by whatever has the arms
        let first = w_hd > 0.003 || w_he > 0.003 || w_lp > 0.003 || kt >= 0.0 || a.throw_t >= 0.0 || a.hit_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0 || a.tell_t >= 0.0;
        let held = first || anim_mv_hands(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0;
        let rb = ready_bare();
        let (catch_ready_out, way_rate, way_by, way_drop, ready_up_slow) = (g.k.hold.catch.ready_out, rb.way.rate, rb.way.by, rb.way.drop, ready_up().slow);
        let grab_secs = g.k.hold.secs.grab;
        let a = &mut g.ch[c].anim;
        let amt = v3::clamp(a.ready_amt, 0.0, 1.0);
        let shaped = a.no_knife && a.ready == Some(ReadyKind::Grab) && !a.grip[1] && amt < 1.0;
        a.rd_a = if a.ready_w > 0.003 { v3::damp(a.rd_a, if a.ready.is_some() { amt } else { a.rd_a }, way_rate, dt) } else { amt };
        let busy = held || a.strike_t >= 0.0 || a.bw.is_some();
        let want = if a.ready.is_some() && !busy { if shaped { v3::smoothstep(0.0, way_by, amt) } else { amt } } else { 0.0 };
        if let Some(r) = a.ready { a.ready_k = r; a.rd_at = a.ready_at; }
        let g_in = match &a.hold_pose {
            Some(h) if a.no_knife && a.ready_w > 0.0 && h.phase == HoldPhase::Grab && a.ready_k == ReadyKind::Grab => v3::clamp(h.t / grab_secs, 0.0, 1.0),
            _ => -1.0,
        };
        a.rd_out_r = if g_in >= 0.0 && a.hold_pose.as_ref().map_or(false, |h| h.strip.is_some()) { 1.0 - v3::smoothstep(0.0, rb.out_r, g_in) } else { 1.0 };
        if g_in >= 0.0 {
            let o = if a.hold_pose.as_ref().map_or(None, |h| h.variant) != Some(HoldVariant::Gun) { catch_ready_out } else { rb.out };
            a.ready_w = m::min(a.ready_w, 1.0 - v3::smoothstep(o[0], o[1], g_in));
        } else {
            a.ready_w = if m::abs(want - a.ready_w) < 0.01 { want } else { a.ready_w + (want - a.ready_w) * m::min(1.0, dt * (if js::truthy(want) { 10.0 } else if busy { 8.0 } else if a.no_knife && a.ready_k == ReadyKind::Grab { way_drop } else { 8.0 / (1.0 + ready_up_slow * a.rd[rd().up as usize]) })) };
        }
        // the reach's clock: on with a live pose whose right hand is free for the knife, and kept on through the knife's own blow
        let on = (want > 0.0 && ready_knife(a.ready_k) && !a.no_knife && !a.grip[1]) || (a.strike_t >= 0.0 && a.strike_leth && !held && a.ready.is_some());
        a.rc_t = v3::clamp(a.rc_t + (if on { dt / reach().in_ } else { -dt / reach().out }), 0.0, 1.0);
        if a.ready_w > 0.003 || a.rc_t > 0.0 {
            let (w, k) = (a.ready_w, a.ready_k);
            anim_ready_layer(g, c, w, k, want > 0.0, dt, y, -p * 0.35);
        } else {
            a.ready_shown = 0.0; a.reach_shown = 0.0; a.rd_live = false;
            if a.ready.is_none() { a.rd_at = None; }
        }
    }
    anim_catch_fix(g, c);
    if g.ch[c].anim.kit.is_some() {   // KIT: the right hand's errand; dropped by whatever else takes the arms, NOT by the wall hold
        let a = &g.ch[c].anim;
        let first = a.hold_pose.is_some() || a.held_pose.is_some() || a.haul.is_some() || a.lockpick || kt >= 0.0 || a.throw_t >= 0.0 || a.strike_t >= 0.0 || a.hit_t >= 0.0;
        if first || anim_mv_hands(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0 || g.ch[c].anim.act.is_some() || g.ch[c].anim.wound.is_some() { anim_kit_cancel(g, c); }
        else { anim_kit_layer(g, c, dt, y, -p * 0.35); }
    } else if seam_kit_hold_defined(g, c) { anim_kit_cancel(g, c); }
    g.ch[c].anim.kit_gun_free = 0.0;
    let d_reload = if g.ch[c].anim.reload_t >= 0.0 { clip_dur(g, c, "Pistol_Reload") } else { 0.0 };
    let a = &mut g.ch[c].anim;
    if a.reload_t >= 0.0 && a.long_gun {   // GUN HAND: a long gun's reload is its carriage's own move; the clip's clock runs on
        a.reload_t += dt * (d_reload / a.reload_dur);
        if a.reload_t > d_reload { a.reload_t = -1.0; }
    } else if a.reload_t >= 0.0 {
        let t = a.reload_t;
        let clip = clip_of(g, c, "Pistol_Reload");
        crate::anim_skeleton::sample_clip(&g.tab.clip[clip], t, false, &mut g.ch[c].anim.p_b);
        let w = v3::clamp(m::min(t / 0.12, (d_reload - t) / 0.15), 0.0, 1.0);
        let a = &mut g.ch[c].anim;
        if a.rl_b.hand && !a.long_gun { a.kit_gun_free = w; }
        crate::anim_skeleton::blend_pose(None, &a.p_b, w, Some(&a.upper_mask), &mut a.p_a, singles);
        a.reload_t += dt * (d_reload / a.reload_dur);
        if a.reload_t > d_reload { a.reload_t = -1.0; }
    }
    let t = g.ch[c].anim.throw_t;
    if t >= 0.0 {
        anim_throw_layer(g, c, t);
        let a = &mut g.ch[c].anim;
        a.throw_t += dt;
        if a.throw_t > THROW_DUR { a.throw_t = -1.0; a.thr_carry = None; a.thr_last = None; }
    }
    let mut w_strike = 0.0;
    let mut bw_trunk = false;   // (the hand pass: a punch keeps its fists)
    if g.ch[c].anim.strike_t >= 0.0 {
        let cross = clip_of(g, c, "Punch_Cross");
        let a = &g.ch[c].anim;
        let sc = if a.strike_k == StrikeKind::Jab { a.clips.get("Punch_Jab").copied().unwrap_or(cross) } else { cross };
        let t = a.strike_t;
        crate::anim_skeleton::sample_clip(&g.tab.clip[sc], t, false, &mut g.ch[c].anim.p_b);
        let d = g.tab.clip[sc].duration;
        let w = v3::clamp(m::min(t / 0.05, (d - t) / 0.15), 0.0, 1.0);
        w_strike = w;
        let bw_style = g.ch[c].anim.bw.as_ref().map(|b| b.style);
        let a = &mut g.ch[c].anim;
        crate::anim_skeleton::blend_pose(None, &a.p_b, match bw_style { Some(st) => w * blow_clip(st), None => w }, Some(&a.upper_mask), &mut a.p_a, singles);
        if let Some(b) = g.ch[c].anim.bw.clone() { anim_blow_trunk(g, c, &b); bw_trunk = true; }
        if g.ch[c].anim.strike_leth { let k = g.ch[c].anim.strike_k; anim_knife_blow(g, c, k, dt, d / 1.15, y, -p * 0.35); }
        let a = &mut g.ch[c].anim;
        a.strike_t += dt * 1.15;
        if a.strike_t > d { a.strike_t = -1.0; a.strike_k = StrikeKind::Cross; }
    }
    let a = &mut g.ch[c].anim;
    if a.strike_leth && a.strike_t < 0.0 { a.knife_shown = false; a.strike_leth = false; }
    if let Some(b) = a.bw.clone() {
        if !bw_trunk { anim_blow_trunk(g, c, &b); }
        let b = g.ch[c].anim.bw.clone().unwrap_or(b);
        let shown = anim_blow_arms(g, c, &b, y, -p * 0.35);
        w_strike = m::max(w_strike, shown);
    }
    if let Some(t) = g.ch[c].anim.tk.clone() { anim_taken_layer(g, c, AnimTakenLayerT { style: t.style, t: t.t, delay: t.delay, off: t.off }); }
    let hit_t = g.ch[c].anim.hit_t;
    if hit_t >= 0.0 {
        let clip = clip_of(g, c, "Hit_Chest");
        crate::anim_skeleton::sample_clip(&g.tab.clip[clip], hit_t, false, &mut g.ch[c].anim.p_b);
        let d = g.tab.clip[clip].duration;
        let w = v3::clamp(1.0 - m::abs(hit_t / d - 0.3) / 0.7, 0.0, 1.0) * 0.8;
        let a = &mut g.ch[c].anim;
        crate::anim_skeleton::blend_pose(None, &a.p_b, w, Some(&a.upper_mask), &mut a.p_a, singles);
        a.hit_t += dt;
        if a.hit_t > d { a.hit_t = -1.0; }
    }
    // ---- the big slot's tool: the hammer or the breaching shotgun in both hands, over every arm pose and one-shot above
    g.ch[c].anim.big_at_back = 0.0;
    if let Some(b) = g.ch[c].anim.big_tool.clone() { anim_big_layer(g, c, &b, y, -p * 0.35); } else { let a = &mut g.ch[c].anim; a.big_k = 0.0; a.big_under = false; }
    // ---- a lean round an edge
    if g.ch[c].anim.lean != 0.0 || g.ch[c].anim.lean_sm != 0.0 { anim_lean_body(g, c, dt); }
    // ---- MOVES: the reactions that take the whole upper body with the weapon, and the support hand's toss
    {
        let gate = (1.0 - w_hd) * (1.0 - w_he) * (1.0 - w_lp) * (1.0 - w_hl) * (1.0 - w_st * wl) * (1.0 - w_kslow);
        g.ch[c].anim.mvs.arm_l = 0.0;
        let run_post = anim_mv_busy(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0 || g.ch[c].anim.dazzle.is_some() || (g.ch[c].anim.clutch > 0.0 && { let too = g.ch[c].anim.fold_too; fold_on(g, Some(too)) }) || {
            let a = &g.ch[c].anim;
            a.hurt > 0.0 || a.shaken > 0.0 || a.mvs.hu.x != 0.0 || a.mvs.sk.x != 0.0
        };
        if run_post { anim_mv_post(g, c, dt, gate); }
        let a = &g.ch[c].anim;
        if a.rl_t >= 0.0 || a.reload_at >= 0.0 || a.bound != 0.0 || a.clutch > 0.0 || a.hurt > 0.0 || a.mv2.rw.x != 0.0 || a.mv2.bd.x != 0.0 || a.mv2.cl.x != 0.0 {
            if a.arms_buf.is_none() {
                let n = a.p_a.num_nodes as f64;
                g.ch[c].anim.arms_buf = Some(crate::anim_skeleton::pose_new(n));
            }
            let a = &mut g.ch[c].anim;
            let src = &a.p_a.cols;
            a.arms_buf.as_mut().unwrap().cols.clone_from(src);
            anim_mv_arms(g, c, dt, gate);
            let a = &mut g.ch[c].anim;
            a.arms_pre = if a.mv2_shown.reload > 0.0 || a.mv2_shown.bound > 0.0 || a.mv2_shown.clutch > 0.0 { a.arms_buf.clone() } else { None };
        } else { g.ch[c].anim.arms_pre = None; }
    }
    // kick clock (the impact flag is for this one frame: a zero-dt re-bake advances nothing and must not eat it)
    let a = &mut g.ch[c].anim;
    if dt > 0.0 {
        a.kick_hit = false;
        if kt >= 0.0 {
            let nt = kt + dt;
            a.kick_hit = kt < KICK_IMPACT && nt >= KICK_IMPACT;
            a.kick_t = if nt > kick_dur { -1.0 } else { nt };
        }
    }
    a.lp_t += dt;
    // ---- the whole-body action clip, over everything above with its eased weight
    let mut w_act = 0.0;
    if a.act.is_some() || a.act_w > 0.0 { w_act = anim_act_layer(g, c, dt); }
    if g.ch[c].anim.arms_pre.is_some() && w_act > 0.0 { let a = &mut g.ch[c].anim; crate::anim_skeleton::blend_pose(None, &a.p_b, w_act, None, a.arms_pre.as_mut().unwrap(), singles); }
    // ---- MOVES: a crouched man's toes stay out of the floor
    if crouch_pose > 0.003 { anim_toes_up(g, c, crouch_pose); }
    if g.ch[c].anim.wound.is_some() || g.ch[c].anim.wd_w > 0.0 {
        let wd = anim_wound_layer(g, c, dt);
        w_act = m::max(w_act, wd);
        g.ch[c].anim.arms_pre = None;
    }
    // ---- MOVES: the hand pass, last of all, finger and thumb joints only
    {
        let a = &g.ch[c].anim;
        let (rel, aim, tor) = (a.upper_w.relaxed, a.upper_w.aim, a.upper_w.torch);
        let gun = m::max_n(&[rel, aim, w_lg, w_carry * (1.0 - w_st * wl) * (if armed { 1.0 } else { 0.0 })]);
        let gun_r = m::max_n(&[rel, aim, w_lg]) * (1.0 - w_hd) * (1.0 - w_he) * (1.0 - w_lp) * (1.0 - w_wall_arms) * (1.0 - w_act) * (if a.throw_t >= 0.0 || a.reload_t >= 0.0 || w_strike > 0.0 { 0.0 } else { 1.0 }) * (if a.big_tool.as_ref().map_or(false, |b| b.kind == AnimBigToolKind::Hammer && big_off_tool(b)) { 0.0 } else { 1.0 });
        anim_hands_pass(g, c, dt, m::max(w_strike, gun), m::max_n(&[w_strike, gun, tor]), gun_r);
        let a = &g.ch[c].anim;
        if a.grip_can && a.grip[1] && a.throw_t < 0.0 {   // a can in the fist: pA is lent out and put back (hand() reads only the pose handed in and the rest pose)
            let can = hand().can;
            let mut pa = core::mem::replace(&mut g.ch[c].anim.p_a, crate::anim_skeleton::Pose::blank());
            anim_hand(g, c, &mut pa, 1.0, &can.curl, can.thumb, None);
            g.ch[c].anim.p_a = pa;
        }
    }
    // ---- KIT: the goggles flipped by hand or by a nod
    if g.ch[c].anim.nv.is_some() {
        let free = left_hand_clear(g, c, w_hd, w_he, w_lp, w_hl, w_lg, w_wall_arms, w_act, w_strike, kt);
        anim_nv_layer(g, c, dt, free);
    }
    // ---- a smoke: the left hand up to the mouth and down again
    if g.ch[c].anim.smoke.is_some() || g.ch[c].anim.smoke_arm > 0.0 {
        let free = g.ch[c].anim.nv.is_none() && left_hand_clear(g, c, w_hd, w_he, w_lp, w_hl, w_lg, w_wall_arms, w_act, w_strike, kt);
        anim_smoke_layer(g, c, dt, free);
    }
    let a = &mut g.ch[c].anim;
    let nod = a.nv_nod;
    let nod_in = a.sp.is_some();
    a.nv_a = nod;
    if nod != 0.0 && !nod_in { nv_tip_p_a(g, c, nod); }
    if g.ch[c].anim.sp.is_some() {
        anim_stealth_picture(g, c, dt);
        let a = &mut g.ch[c].anim;
        let mut copied = false;
        if let Some(sp) = &a.sp {
            if sp.pic_on { if let Some(pic) = &sp.pic { a.p_a.cols.clone_from(&pic.cols); copied = true; } }
        }
        if !copied && nod != 0.0 && nod_in {
            let tip = if a.sp.as_ref().map_or(false, |sp| sp.nod_up == Some(true)) { nod_lift(nod) } else { nod };
            a.nv_a = tip;
            nv_tip_p_a(g, c, tip);
        }
    }
    let a = &mut g.ch[c].anim;
    a.pose.cols.clone_from(&a.p_a.cols);
    if a.sw.shown && a.sw_buf.is_some() {
        if a.hand_buf.is_none() {
            let n = a.p_a.num_nodes as f64;
            a.hand_buf = Some(crate::anim_skeleton::pose_new(n));
        }
        crate::anim_skeleton::blend_pose(Some(&a.p_a), a.sw_buf.as_ref().unwrap(), 1.0, Some(&a.arm_r_only), a.hand_buf.as_mut().unwrap(), singles);
        a.hand_pre = a.hand_buf.clone();
    } else { a.hand_pre = None; }
    let a = &mut g.ch[c].anim;
    if a.big_under && a.big_buf.is_some() { a.hand_pre = a.big_buf.clone(); }
    // ---- procedural twists
    a.twists.clear();
    let nd = a.nodes.clone();
    // the gaze stabiliser: head level and steady while the body works under it
    {
        let first = (1.0 - w_st) * (1.0 - w_hd) * (1.0 - w_he) * (1.0 - w_lp) * (1.0 - a.upper_w.aim) * (1.0 - w_act);
        let authored = kt >= 0.0 || a.throw_t >= 0.0 || a.hit_t >= 0.0 || a.strike_t >= 0.0 || a.bw.is_some() || a.tk.is_some() || a.reload_t >= 0.0 || a.rlc.h > 0.0 || a.tell_t >= 0.0;
        let authored = authored || anim_mv_busy(g, c) || { let a = &g.ch[c].anim; a.wk.shown > 0.02 || a.rx.w.x > 0.02 || a.big_tool.is_some() };
        let want_w = first * (if authored { 0.0 } else { 1.0 });
        let yaw_fix = anim_gaze_keep(g, c, dt, want_w, crouch_pose, w_sprint);
        let a = &mut g.ch[c].anim;
        if a.gz.w > 0.01 {
            let w = a.gz.w;
            let gp = a.gz.pitch;
            a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.neck as f64, yaw: yaw_fix * 0.4 * w, pitch: gp * 0.4 * w });
            a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.head as f64, yaw: yaw_fix * 0.6 * w, pitch: gp * 0.6 * w });
        }
    }
    let a = &mut g.ch[c].anim;
    let look = if a.turn_lead != 0.0 { a.look_yaw_extra + head_keep + a.turn_lead } else { a.look_yaw_extra + head_keep };
    a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.spine1 as f64, yaw: y * 0.25, pitch: 0.0 });
    a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.spine2 as f64, yaw: y * 0.35, pitch: 0.0 });
    a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.spine3 as f64, yaw: y * 0.40, pitch: -p * 0.35 });
    if m::abs(look) > 1e-3 {
        a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.neck as f64, yaw: look * 0.4, pitch: 0.0 });
        a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.head as f64, yaw: look * 0.6, pitch: 0.0 });
    }
    if a.look_pitch_extra != 0.0 {
        let e = a.look_pitch_extra;
        a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.neck as f64, yaw: 0.0, pitch: e * 0.4 });
        a.twists.push(crate::anim_skeleton::TwistSpec { node: nd.head as f64, yaw: 0.0, pitch: e * 0.6 });
    }
    if a.nv.is_some() || a.smoke.is_some() {   // what turns the head after the layers, kept for the hand's reach next frame
        a.nv_tw = [0.0; 4];
        for i in 0..a.twists.len() {
            let t = a.twists[i];
            if t.node == nd.neck as f64 { a.nv_tw[0] += t.yaw; a.nv_tw[1] += t.pitch; } else if t.node == nd.head as f64 { a.nv_tw[2] += t.yaw; a.nv_tw[3] += t.pitch; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Each word of the unions these tables are read by finds its row: a word spelt otherwise than the table's key would panic only when that move plays.
    #[test]
    fn every_word_finds_its_row() {
        for k in [Stance::Wall, Stance::Stack, Stance::HighReady, Stance::LowReady] { let _ = stance_shape(k); }
        assert_eq!(stance_shape(Stance::LowReady).pitch.to_bits(), (-42.0f64).to_bits());
        for (k, d) in [(SignalKind::Hold, 0.9), (SignalKind::Go, 0.95), (SignalKind::Rally, 1.3), (SignalKind::Aside, 1.15), (SignalKind::Squeeze, 0.9)] { assert_eq!(signal_secs(k), d); }
        for (k, d) in [(TellKind::Watch, 1.05), (TellKind::Neck, 1.35)] { assert_eq!(tell_secs(k), d); }
        for k in [ReadyKind::Grab, ReadyKind::Seat, ReadyKind::Strike, ReadyKind::GrabGun] { let _ = ready_knife(k); }
        for (k, d) in [(BlowStyle::Chop, 0.25), (BlowStyle::Palm, 0.35), (BlowStyle::Mouth, 1.0), (BlowStyle::Sweep, 1.0), (BlowStyle::Slash, 1.0)] { assert_eq!(blow_clip(k), d); }
    }
}
