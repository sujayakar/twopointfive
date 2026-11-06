//! src/game/samlook.ts lines 1 to 105
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// This seat's row, made if it is missing.   (samlook.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::samlook::St {
    let pl = g.player;
    g.m.samlook.state.get_or_insert_with(pl, || St { on: None })
}

/// Smoothstep from `a` to `b`.   (samlook.ts smooth)
pub fn smooth(a: f64, b: f64, x: f64) -> f64 {
    let t = m::max(0.0, m::min(1.0, (x - a) / (b - a)));
    t * t * (3.0 - 2.0 * t)
}

/// An angle brought into -pi..pi.   (samlook.ts wrap)
pub fn wrap(a: f64) -> f64 {
    m::atan2(m::sin(a), m::cos(a))
}

/// His head is his own to turn.   (samlook.ts free)
pub fn free(g: &mut crate::game::Game) -> bool {
    let pl = &g.pl[g.player];
    !pl.down && !pl.sprinting && pl.holding.is_none() && pl.takedown.is_none() && pl.kick.is_none() && pl.picking.is_none() && pl.wanding.is_none() && pl.coding.is_none() && pl.wall.is_none() && pl.dragging.is_none() && pl.door_move.is_none() && pl.breaker_move.is_none() && !g.touring && g.puppet.is_none() && g.ch[pl.ch].anim.upper != crate::character::UpperMode::Aim
}

/// A man who is onto him, or has him in view this moment with his notice risen.   (samlook.ts samLookNoticed)
pub fn sam_look_noticed(g: &mut crate::game::Game, gd: GuardId) -> bool {
    use crate::game::GuardState;
    let man = &g.gd[gd];
    man.state == GuardState::Alert || man.state == GuardState::Search || man.state == GuardState::Suspicious || (man.awareness > 0.08 && g.time - man.sight_t < 0.4)
}

/// Metres to a man his head could go to (infinity: not one); `deg` how far round from his aim, 180 anywhere.   (samlook.ts reach)
pub fn reach(g: &mut crate::game::Game, gd: GuardId, max_m: f64, deg: f64) -> f64 {
    if !crate::close::close_subject(g, gd) { return f64::INFINITY; }
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let q = g.ch[g.gd[gd].ch].pos;
    let dx = q[0] - p[0];
    let dz = q[2] - p[2];
    let d = m::hyp2(dx, dz);
    if d > max_m || d < 0.15 { return f64::INFINITY; }
    if deg < 180.0 && m::abs(wrap(m::atan2(dx, dz) - g.ch[c].aim_yaw)) > deg * m::PI / 180.0 { return f64::INFINITY; }
    if crate::scene_collision::solid_blocked(g, [p[0], p[1] + 1.2, p[2]], [q[0], q[1] + 1.4, q[2]], None) { return f64::INFINITY; }
    d
}

/// Picks the man his head is on this frame and remembers him.   (samlook.ts pick)
pub fn pick(g: &mut crate::game::Game) -> Option<GuardId> {
    let was = state_of(g).on;
    if !free(g) {
        state_of(g).on = None;
        return None;
    }
    let st = crate::stalk::stalk_on(g);
    if let Some(st) = st {
        if crate::close::close_subject(g, st) {
            state_of(g).on = Some(st);
            return Some(st);
        }
    }
    let mut best = None;
    let mut bs = f64::INFINITY;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let mine = Some(gd) == was;
        let (near_m, keep_deg, back_deg) = (g.k.samlook.near_m, g.k.samlook.keep_deg, g.k.samlook.back_deg);
        let d = reach(g, gd, if mine { near_m + 0.5 } else { near_m }, if mine { keep_deg } else { back_deg });
        if d == f64::INFINITY { continue; }
        let s = d - (if sam_look_noticed(g, gd) { g.k.samlook.notice_m } else { 0.0 }) - (if mine { g.k.samlook.swap_m } else { 0.0 });
        if s < bs { bs = s; best = Some(gd); }
    }
    state_of(g).on = best;
    best
}

/// The man his head is on, as of this frame's step; never makes a row.   (samlook.ts samLookMan)
pub fn sam_look_man(g: &mut crate::game::Game) -> Option<GuardId> {
    let pl = g.player;
    g.m.samlook.state.get(pl).and_then(|s| s.on)
}

/// How much room one man asks of him: 0 past `slipFrom`, for a man onto him, and behind a man's back.   (samlook.ts slipOf)
pub fn slip_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if g.gd[gd].state == crate::game::GuardState::Alert { return 0.0; }
    let slip_from = g.k.samlook.slip_from;
    let d = reach(g, gd, slip_from, 180.0);
    if !(d < slip_from) { return 0.0; }
    let p = g.ch[g.pl[g.player].ch].pos;
    let c = g.gd[gd].ch;
    let q = g.ch[c].pos;
    let nose = m::cos(wrap(m::atan2(p[0] - q[0], p[2] - q[2]) - g.ch[c].body_yaw));
    let k = g.k.samlook;
    (1.0 - smooth(k.slip_by, k.slip_from, d)) * smooth(k.back_cos[0], k.back_cos[1], nose)
}

/// The slip's three conditions.   (samlook.ts slips)
pub fn slips(g: &mut crate::game::Game) -> bool {
    free(g) && g.pl[g.player].crouch && g.pl[g.player].holstered
}

/// The man who asks the most room of him, and how much (0..1).   (samlook.ts samSlipMan)
pub fn sam_slip_man(g: &mut crate::game::Game) -> Option<crate::samlook::SamSlipMan> {
    if !slips(g) { return None; }
    let mut best = None;
    let mut bk = 0.0;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let k = slip_of(g, gd);
        if k > bk { bk = k; best = Some(gd); }
    }
    best.map(|gd| SamSlipMan { gd, k: bk })
}

/// The slip as the animator is handed it: the most any man asks, the lean left of it, and where they pull him.   (samlook.ts slipNow)
pub fn slip_now(g: &mut crate::game::Game) -> Option<crate::samlook::SlipNow> {
    if !slips(g) { return None; }
    let p = g.ch[g.pl[g.player].ch].pos;
    let (mut vx, mut vz, mut top) = (0.0, 0.0, 0.0);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let k = slip_of(g, gd);
        if k <= 0.0 { continue; }
        let q = g.ch[g.gd[gd].ch].pos;
        let dx = q[0] - p[0];
        let dz = q[2] - p[2];
        let l = js::or(m::hyp2(dx, dz), 1.0);
        vx += k * dx / l;
        vz += k * dz / l;
        if k > top { top = k; }
    }
    if top <= 0.0 { return None; }
    let l = m::hyp2(vx, vz);
    let u = if l > 1e-3 { 1.0 / l } else { 0.0 };
    Some(SlipNow { k: top, lean: m::min(1.0, l / top), at: [p[0] + vx * u, p[1] + 1.1, p[2] + vz * u] })
}

/// A point `up` metres over a man's feet.   (samlook.ts headOf)
pub fn head_of(g: &mut crate::game::Game, gd: GuardId, up: f64) -> V3 {
    let q = g.ch[g.gd[gd].ch].pos;
    [q[0], q[1] + up, q[2]]
}

/// Once a step, after the player's own update: what his animator is handed for the next frame's pose.   (samlook.ts samLookStep)
pub fn sam_look_step(g: &mut crate::game::Game) {
    let c = g.pl[g.player].ch;
    let an = &mut g.ch[c].anim;
    an.watch_at = None;
    an.watch_k = 0.0;
    an.slip_at = None;
    an.slip_k = 0.0;
    an.slip_lean = 1.0;
    if let Some(gd) = pick(g) {
        let p = g.ch[c].pos;
        let gc = g.gd[gd].ch;
        let q = g.ch[gc].pos;
        let d = m::hyp2(q[0] - p[0], q[2] - p[2]);
        let at = match g.ch[gc].bones.get(crate::character::Bone::Head) {
            Some(h) => [h[0], h[1], h[2]],
            None => head_of(g, gd, 1.6),
        };
        g.ch[c].anim.watch_at = Some(at);
        let k = if crate::stalk::stalk_on(g) == Some(gd) {
            1.0
        } else {
            let (full_m, near_m) = (g.k.samlook.full_m, g.k.samlook.near_m);
            1.0 - smooth(m::min(full_m, near_m * 0.7), near_m, d)
        };
        g.ch[c].anim.watch_k = k;
    }
    if let Some(sl) = slip_now(g) {
        let an = &mut g.ch[c].anim;
        an.slip_at = Some(sl.at);
        an.slip_k = sl.k;
        an.slip_lean = sl.lean;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The TypeScript's own bits, under EXACT.on.
    #[test]
    fn smooth_matches_the_typescript() {
        for (a, b, x, bits) in [
            (1.2, 2.6, 1.7, 0x3fd2a8ad278e8dcdu64),
            (1.2, 2.6, 0.3, 0x0000000000000000),
            (1.2, 2.6, 9.0, 0x3ff0000000000000),
            (-0.45, -0.1, -0.3137, 0x3fd58ee4de63e278),
            (5.5, 8.0, 6.123456789, 0x3fc3e9435f4062f6),
        ] {
            assert_eq!(smooth(a, b, x).to_bits(), bits);
        }
        assert!(smooth(5.6, 5.6, 5.6).is_nan());
    }

    #[test]
    fn wrap_matches_the_typescript() {
        for (a, bits) in [
            (0.1, 0x3fb999999999999au64),
            (3.5, 0xc00643f6a8885a30),
            (-7.25, 0xbfeef0255dde973d),
            (100.125, 0xbfd9fb54442d1848),
            (m::PI, 0x400921fb54442d18),
            (-m::PI, 0xc00921fb54442d18),
        ] {
            assert_eq!(wrap(a).to_bits(), bits);
        }
    }
}
