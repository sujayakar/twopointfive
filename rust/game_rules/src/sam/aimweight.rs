pub use ::game_types::aimweight::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};

/// May his aim be weighed now. `AIMWEIGHT.on` is settled on.
fn can(g: &mut Game) -> bool {
    let pl = g.player;
    g.puppet.is_none() && !crate::condition::down(g, g.pl[pl].ch)
}

/// Does his aim have weight right now.
pub fn aim_weighs(g: &mut Game) -> bool {
    can(g)
}

/// The top rate his gun's heading turns at, rad/s, by what is in his hands.
pub fn aim_top(g: &Game, pl: PlayerId) -> f64 {
    let gun = crate::game::player_gun_in_hand(g, pl);
    let scale = match gun {
        Some(gun) => m::pow(crate::weapons::handling_of(g.m.weapons.weapons.get(g.fa[gun].def).unwrap()), m::max(0.0, g.k.aimweight.heft)),
        None => g.k.aimweight.bare,
    };
    m::max(0.5, g.k.aimweight.rate * scale)
}

/// The swing's arithmetic: what is still left to turn; `vel` carried on. `smooth` is `AIMWEIGHT.smooth`.
fn swing(smooth: f64, vel: &mut f64, err: f64, top: f64, dt: f64) -> f64 {
    let mut err = err;
    let t = m::max(1e-3, smooth);
    let om = 2.0 / t;
    let lim = top * t;
    let n = v3::clamp(m::ceil(dt * 240.0 - 1e-6), 1.0, 48.0);
    let h = dt / n;
    let ex = m::exp(-om * h);
    let mut i = 0.0;
    while i < n && err != 0.0 {
        let ch = v3::clamp(-err, -lim, lim);
        let tmp = (*vel + om * ch) * h;
        *vel = (*vel - om * tmp) * ex;
        let mut step = -ch + (ch + tmp) * ex;
        if m::abs(step) >= m::abs(err) && step * err >= 0.0 {
            step = err;
            *vel = 0.0;
        }
        err -= step;
        if m::abs(err) < 1e-3 && m::abs(*vel) < 0.05 {
            err = 0.0;
            *vel = 0.0;
        }
        i += 1.0;
    }
    err
}

/// THE SWING: a heading `err` off the cursor's bearing, turning at `w.vel`, stepped `dt` on: what is still left to turn.
pub fn aim_swing(g: &Game, w: &mut AimSwingW, err: f64, top: f64, dt: f64) -> f64 {
    swing(g.k.aimweight.smooth, &mut w.vel, err, top, dt)
}

/// First thing in the player's update: this frame has not weighed his aim yet.
pub(super) fn aim_weigh_begin(g: &mut Game, pl: PlayerId) {
    let w = &mut g.pl[pl].aimw;
    w.was = w.live;
    w.live = false;
    w.on = true;
    w.landed = true;
    w.err = 0.0;
}

/// The plain aim branch's step: turn `aim_yaw` toward the cursor's bearing. False = not weighing (the caller keeps its own ease).
pub(super) fn aim_weigh_turn(g: &mut Game, dt: f64) -> bool {
    if !can(g) {
        return false;
    }
    let pl = g.player;
    let c = g.pl[pl].ch;
    let ap = g.aim_point;
    let pos = g.ch[c].pos;
    let dx = ap[0] - pos[0];
    let dz = ap[2] - pos[2];
    let d = m::hyp2(dx, dz);
    if !g.pl[pl].aimw.was {
        let w = &mut g.pl[pl].aimw;
        w.vel = 0.0;
        w.r = d;
        w.y = ap[1];
    }
    g.pl[pl].aimw.live = true;
    g.stats.aimweight.steps += 1;
    let mut err = 0.0;
    if d > 0.25 && dt > 0.0 {
        let y0 = g.ch[c].aim_yaw;
        let err0 = v3::wrap_angle(m::atan2(dx, dz) - y0);
        let top = aim_top(g, pl);
        // the same working as aim_swing, on the row's own vel: `w` here is the whole AimWeight, not an AimSwingW
        let smooth = g.k.aimweight.smooth;
        err = swing(smooth, &mut g.pl[pl].aimw.vel, err0, top, dt);
        g.ch[c].aim_yaw = y0 + (err0 - err);
    } else {
        g.pl[pl].aimw.vel = 0.0;
    }
    let k = 1.0 - m::exp(-m::max(0.0, g.k.aimweight.reach) * dt);
    let on_m = g.k.aimweight.on_m;
    let w = &mut g.pl[pl].aimw;
    w.r += (d - w.r) * k;
    w.y += (ap[1] - w.y) * k;
    w.landed = err == 0.0 && m::abs(d - w.r) < 0.02;
    if w.landed {
        w.r = d;
        w.y = ap[1];
    }
    w.err = err;
    w.on = m::abs(err) * m::max(d, 1.0) <= on_m;
    true
}

/// What leaves the gun goes where the gun points: `tgt` as it is while on the cursor, else turned about him by what is left to swing.
pub(super) fn aim_weigh_shot(g: &mut Game, tgt: V3) -> V3 {
    let pl = g.player;
    let w = g.pl[pl].aimw;
    if !w.live || w.on {
        return tgt;
    }
    let c = g.pl[pl].ch;
    let pos = g.ch[c].pos;
    let dx = tgt[0] - pos[0];
    let dz = tgt[2] - pos[2];
    let cs = m::cos(w.err);
    let sn = m::sin(w.err);
    g.stats.aimweight.off_shots += 1;
    [pos[0] + dx * cs - dz * sn, tgt[1], pos[2] + dz * cs + dx * sn]
}

/// The launcher's release: true = send it now; a load let go of mid-swing waits up to `launch_wait`.
pub(super) fn aim_weigh_launch(g: &mut Game, dt: f64, up: bool) -> bool {
    let pl = g.player;
    let wait = g.k.aimweight.launch_wait;
    let w = &mut g.pl[pl].aimw;
    if !up {
        w.launch_t = 0.0;
        return false;
    }
    if !w.live || w.on || !(wait > 0.0) {
        w.launch_t = 0.0;
        return true;
    }
    w.launch_t += dt;
    if w.launch_t >= wait {
        w.launch_t = 0.0;
        return true;
    }
    false
}

/// A load owed on a frame the gun is down: the wait's clock runs on. True = still owed.
pub(super) fn aim_weigh_launch_keep(g: &mut Game, pl: PlayerId, dt: f64) -> bool {
    let wait = g.k.aimweight.launch_wait;
    let w = &mut g.pl[pl].aimw;
    if !(w.launch_t > 0.0) {
        return false;
    }
    w.launch_t += dt;
    if w.launch_t >= wait {
        w.launch_t = 0.0;
        return false;
    }
    true
}

/// Where the gun points: the circle's world point. None when nothing is weighing his aim (the circle is the cursor's).
pub fn aim_gun_point(g: &Game, seat: &crate::seats::SeatRef) -> Option<V3> {
    let pl = &g.pl[seat.player];
    let w = &pl.aimw;
    if !w.live { return None; }
    if w.landed { return Some(seat.aim_point); }
    let c = &g.ch[pl.ch];
    Some([c.pos[0] + m::sin(c.aim_yaw) * w.r, w.y, c.pos[2] + m::cos(c.aim_yaw) * w.r])
}
/// Is the circle on the cursor (true when nothing is weighing his aim).
pub fn aim_gun_on(g: &Game, pl: PlayerId) -> bool {
    let w = &g.pl[pl].aimw;
    !w.live || w.on
}

#[cfg(test)]
mod tests {
    use super::swing;


    const FRAME: f64 = 1.0 / 60.0;

    /// A failure means the aim feels different: a gun that overshoots, never lands, swings one way faster than the other, or turns at another rate on another machine's frame rate.
    #[test]
    fn a_swing_closes_the_gap_and_lands_dead_on_it() {
        let (mut vel, top) = (0.0, 11.0);
        let left = swing(0.05, &mut vel, 2.5, top, FRAME);
        assert!(left > 0.0 && left < 2.5 && vel > 0.0, "the first frame starts it: {left}, {vel}");
        let mut vel = 0.0;
        assert_eq!((swing(0.05, &mut vel, 3.0, top, 0.5), vel), (0.0, 0.0), "a long frame gets there and stops");
        let mut vel = 0.0;
        assert_eq!((swing(0.05, &mut vel, 5e-4, top, FRAME), vel), (0.0, 0.0), "a sliver left, moving slowly, is there");
        let mut vel = 0.0;
        assert_eq!((swing(0.05, &mut vel, 0.0, top, FRAME), vel), (0.0, 0.0), "no gap: nothing moves");
    }

    #[test]
    fn a_swing_is_the_same_either_way_and_at_any_frame_rate_and_held_to_its_top_speed() {
        let (mut a, mut b) = (0.0, 0.0);
        let (l, r) = (swing(0.05, &mut a, 2.5, 11.0, FRAME), swing(0.05, &mut b, -2.5, 11.0, FRAME));
        assert!((l + r).abs() < 1e-12 && (a + b).abs() < 1e-12, "{l} {r} {a} {b}");
        let mut once = 0.0;
        let one = swing(0.05, &mut once, 2.5, 11.0, FRAME);
        let mut twice = 0.0;
        let half = swing(0.05, &mut twice, 2.5, 11.0, FRAME / 2.0);
        let two = swing(0.05, &mut twice, half, 11.0, FRAME / 2.0);
        assert!((one - two).abs() < 1e-9 && (once - twice).abs() < 1e-9, "{one} {two}");
        let after = |top: f64| { let (mut vel, mut left) = (0.0, 2.5); for _ in 0..15 { left = swing(0.05, &mut vel, left, top, FRAME); } left };
        assert!(after(1.0) > after(11.0) + 0.5, "{} {}", after(1.0), after(11.0));
        assert!(2.5 - after(1.0) < 0.3, "a quarter of a second at 1 rad/s covers about a quarter radian: {}", after(1.0));
    }
}
