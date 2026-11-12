use super::*;                      // your own module: its types, its other parts' functions
use crate::ids::*;

/// The item is still in the hand.
pub fn anim_throwing(g: &crate::game::Game, c: CharId) -> bool {
    let t = g.ch[c].anim.throw_t;
    t >= 0.0 && t < THROW_RELEASE + 0.05
}
/// The throw one-shot is playing at all.
pub fn anim_in_throw(g: &crate::game::Game, c: CharId) -> bool { g.ch[c].anim.throw_t >= 0.0 }
/// The throw under way is the free (left) hand's.
pub fn anim_throw_off(g: &crate::game::Game, c: CharId) -> bool { g.ch[c].anim.throw_t >= 0.0 && g.ch[c].anim.thr_off }
/// How much of the pistol's aim clip the pose shows, 0..1.
pub(super) fn anim_aim_shown(g: &crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.upper_w.aim }
/// How far the torch is up in the hand that holds it, 0..1.
pub(super) fn anim_torch_up(g: &crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.tu_k }
/// Nothing but the aim or the carry has the gun arm this frame.
pub(super) fn anim_gun_arm_own(g: &crate::game::Game, c: CharId) -> bool {
    let a = &g.ch[c].anim;
    let taken = a.hold_pose.is_some() || a.held_pose.is_some() || a.haul.is_some() || a.lockpick || a.act.is_some() || a.kit.is_some() || a.sw.shown || a.hide_held_item || a.throw_t >= 0.0 || a.strike_t >= 0.0
        || a.hit_t >= 0.0 || a.reload_t >= 0.0 || a.sig_t >= 0.0 || a.tell_t >= 0.0;
    !(taken || anim_mv_busy(g, c) || g.ch[c].anim.mvs.ts_t >= 0.0)
}
/// Seconds into the throw, -1 when not throwing.
pub fn anim_throw_time(g: &crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.throw_t }
/// A shot's first 0.35 s.
pub(super) fn anim_shooting(g: &crate::game::Game, c: CharId) -> bool {
    let t = g.ch[c].anim.shoot_t;
    t >= 0.0 && t < 0.35
}
/// Starts the front kick at a door, unless one is playing.
pub fn anim_kick_door(g: &mut crate::game::Game, c: CharId, opts: Option<crate::character::AnimKickDoorOpts>) {
    let a = &mut g.ch[c].anim;
    if a.kick_t < 0.0 {
        a.kick_t = 0.0;
        a.kick_hit = false;
        a.kick_from = None;
        a.kick_leg = opts.and_then(|o| o.leg).unwrap_or(AnimKickLeg::Right);
    }
}
/// The kick is playing.
pub fn anim_kicking(g: &crate::game::Game, c: CharId) -> bool { g.ch[c].anim.kick_t >= 0.0 }
/// Seconds into the kick, -1 when not kicking.
pub fn anim_kick_time(g: &crate::game::Game, c: CharId) -> f64 { g.ch[c].anim.kick_t }
/// True for exactly the update in which the sole reached the door.
pub fn anim_kick_impact(g: &crate::game::Game, c: CharId) -> bool { g.ch[c].anim.kick_hit }
/// Starts a left-hand signal, unless one is playing.
pub fn anim_signal(g: &mut crate::game::Game, c: CharId, kind: crate::character::SignalKind, side: Option<f64>) {
    let side = side.unwrap_or(1.0);
    let a = &mut g.ch[c].anim;
    if a.sig_t < 0.0 { a.sig_t = 0.0; a.sig_kind = kind; a.sig_side = side; }
}
/// A signal is playing.
pub fn anim_signalling(g: &crate::game::Game, c: CharId) -> bool { g.ch[c].anim.sig_t >= 0.0 }
/// His left hand is his own this frame.
pub fn anim_signal_free(g: &crate::game::Game, c: CharId) -> bool {
    if !anim_gun_arm_own(g, c) { return false; }
    let a = &g.ch[c].anim;
    a.kick_t < 0.0 && a.dazzle.is_none() && a.reload_at < 0.0 && a.rl_t < 0.0
        && a.rx.w.x < 0.02 && a.wk.w_l.x < 0.02 && a.dp.w_l.x < 0.02 && !a.nv.as_ref().map_or(false, |n| n.hand)
}
/// Starts an idle tell, unless one or a signal is playing.
pub fn anim_tell(g: &mut crate::game::Game, c: CharId, kind: crate::character::TellKind) {
    let a = &mut g.ch[c].anim;
    if a.tell_t < 0.0 && a.sig_t < 0.0 { a.tell_t = 0.0; a.tell_k = kind; }
}
/// Drops a tell where it is.
pub fn anim_cancel_tell(g: &mut crate::game::Game, c: CharId) {
    let a = &mut g.ch[c].anim;
    a.tell_t = -1.0;
}
/// The update.
pub(super) fn anim_update(g: &mut crate::game::Game, c: CharId, dt: f64) { anim_update_standing(g, c, dt); }
