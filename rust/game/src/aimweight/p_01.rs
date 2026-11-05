//! src/game/aimweight.ts lines 1 to 157
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// May his aim be weighed now. `AIMWEIGHT.on` is settled on.   (aimweight.ts can)
pub fn can(g: &mut Game) -> bool {
    let pl = g.player;
    g.puppet.is_none() && !g.touring && !g.pl[pl].down
}

/// Does his aim have weight right now.   (aimweight.ts aimWeighs)
pub fn aim_weighs(g: &mut Game) -> bool {
    can(g)
}

/// The top rate his gun's heading turns at, rad/s, by what is in his hands.   (aimweight.ts aimTop)
pub fn aim_top(g: &mut Game, pl: PlayerId) -> f64 {
    let gun = crate::game::player_gun_in_hand(g, pl);
    let scale = match gun {
        Some(gun) => m::pow(crate::weapons::handling_of(g.m.weapons.weapons.get(g.fa[gun].def).unwrap()), m::max(0.0, g.k.aimweight.heft)),
        None => g.k.aimweight.bare,
    };
    m::max(0.5, g.k.aimweight.rate * scale)
}

/// The swing's arithmetic: what is still left to turn; `vel` carried on. `smooth` is `AIMWEIGHT.smooth`.   (aimweight.ts aimSwing, its working)
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

/// THE SWING: a heading `err` off the cursor's bearing, turning at `w.vel`, stepped `dt` on: what is still left to turn.   (aimweight.ts aimSwing)
pub fn aim_swing(g: &mut Game, w: &mut AimSwingW, err: f64, top: f64, dt: f64) -> f64 {
    swing(g.k.aimweight.smooth, &mut w.vel, err, top, dt)
}

/// First thing in the player's update: this frame has not weighed his aim yet.   (aimweight.ts aimWeighBegin)
pub fn aim_weigh_begin(g: &mut Game, pl: PlayerId) {
    let w = &mut g.pl[pl].aimw;
    w.was = w.live;
    w.live = false;
    w.on = true;
    w.landed = true;
    w.err = 0.0;
}

/// The plain aim branch's step: turn `aim_yaw` toward the cursor's bearing. False = not weighing (the caller keeps its own ease).   (aimweight.ts aimWeighTurn)
pub fn aim_weigh_turn(g: &mut Game, dt: f64) -> bool {
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

/// Where the gun points: the circle's world point; None when nothing weighs his aim.   (aimweight.ts aimGunPoint)
pub fn aim_gun_point(g: &mut Game) -> Option<V3> {
    let pl = g.player;
    let w = g.pl[pl].aimw;
    if !w.live {
        return None;
    }
    if w.landed {
        return Some([g.aim_point[0], g.aim_point[1], g.aim_point[2]]);
    }
    let c = g.pl[pl].ch;
    let (pos, yaw) = (g.ch[c].pos, g.ch[c].aim_yaw);
    Some([pos[0] + m::sin(yaw) * w.r, w.y, pos[2] + m::cos(yaw) * w.r])
}

/// Is the circle on the cursor (true when nothing weighs his aim).   (aimweight.ts aimGunOn)
pub fn aim_gun_on(g: &mut Game) -> bool {
    let w = g.pl[g.player].aimw;
    !w.live || w.on
}

/// What leaves the gun goes where the gun points: `tgt` as it is while on the cursor, else turned about him by what is left to swing.   (aimweight.ts aimWeighShot)
pub fn aim_weigh_shot(g: &mut Game, tgt: V3) -> V3 {
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

/// The launcher's release: true = send it now; a load let go of mid-swing waits up to `launch_wait`.   (aimweight.ts aimWeighLaunch)
pub fn aim_weigh_launch(g: &mut Game, dt: f64, up: bool) -> bool {
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

/// A load owed on a frame the gun is down: the wait's clock runs on. True = still owed.   (aimweight.ts aimWeighLaunchKeep)
pub fn aim_weigh_launch_keep(g: &mut Game, pl: PlayerId, dt: f64) -> bool {
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

/// `x.toFixed(0)` for the numbers a rate can be: nearest whole, a tie upward on the size, "-" for anything below zero (so -0.4 is "-0", -0 is "0").
fn to_fixed_0(x: f64) -> String {
    let size = m::round(m::abs(x));
    alloc::format!("{}{:.0}", if x < 0.0 { "-" } else { "" }, size)
}

/// QA / the page's summary line. `AIMWEIGHT.on` is settled on.   (aimweight.ts aimWeightSummary)
pub fn aim_weight_summary(g: &mut Game) -> String {
    let w = g.pl[g.player].aimw;
    if !w.live {
        return String::from("on · not weighing");
    }
    if w.on {
        return String::from("on the cursor");
    }
    alloc::format!("swinging {}°/s", to_fixed_0(w.vel * 180.0 / m::PI))
}

#[cfg(test)]
mod tests {
    use super::{swing, to_fixed_0};

    /// The TypeScript's own bits (bun, EXACT.on, AIMWEIGHT.smooth 0.05): vel, err, top, dt in; vel, left out.
    const CASES: [[u64; 6]; 7] = [
        [0x0000000000000000, 0x4004000000000000, 0x4026000000000000, 0x3f91111111111111, 0x401fb53fc5789426, 0x400359ea72f45940],
        [0x4008000000000000, 0xbff3333333333333, 0x402199999999999a, 0x3f81111111111111, 0xc005ef201dfeb7a1, 0xbff32c68826334ef],
        [0xc01c000000000000, 0x3fd999999999999a, 0x402b800000000000, 0x3f7c71c71c71c71c, 0xbfdda013d8220f83, 0x3fdb225356d4b942],
        [0x0000000000000000, 0x3f40624dd2f1a9fc, 0x4026000000000000, 0x3f91111111111111, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x4008000000000000, 0x4026000000000000, 0x3fe0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x4026000000000000, 0x3f91111111111111, 0x0000000000000000, 0x0000000000000000],
        [0x4034000000000000, 0xc008cccccccccccd, 0x3fe0000000000000, 0x3fa1111111111111, 0x3fe8d9869fa8879b, 0xc00a835ff8b0d050],
    ];

    const TO_FIXED: [(u64, &str); 12] = [
    (0x0000000000000000, "0"),
    (0x8000000000000000, "0"),
    (0x3fe0000000000000, "1"),
    (0x3ff8000000000000, "2"),
    (0x4004000000000000, "3"),
    (0xbfd999999999999a, "-0"),
    (0xbfe0000000000000, "-1"),
    (0xc004000000000000, "-3"),
    (0x3fdfffffffffffff, "0"),
    (0x4083b20000000000, "630"),
    (0xc0934a0000000000, "-1235"),
    (0x40667fffeb074a77, "180"),
    ];

    #[test]
    fn swing_matches_the_typescript_bits() {
        for c in CASES {
            let f = f64::from_bits;
            let mut vel = f(c[0]);
            let left = swing(0.05, &mut vel, f(c[1]), f(c[2]), f(c[3]));
            assert_eq!(vel.to_bits(), c[4], "vel of {:x?}", c);
            assert_eq!(left.to_bits(), c[5], "left of {:x?}", c);
        }
    }

    /// The TypeScript's own answers: `x.toFixed(0)` under bun.
    #[test]
    fn to_fixed_0_matches_the_typescript() {
        for (bits, want) in TO_FIXED {
            assert_eq!(to_fixed_0(f64::from_bits(bits)), want, "{:x}", bits);
        }
    }
}
