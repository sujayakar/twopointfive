//! src/game/character.ts lines 1358 to 1733
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// How long a landed blow or shell is followed through before the tool may start back, s. (character.ts bigRebound)
pub fn big_rebound(kind: crate::character::BigReboundKind) -> f64 {
    let t = crate::character::bigmove();
    if kind == crate::character::BigReboundKind::Hammer { t.hammer.rebound } else { t.shotgun.kick_in + t.shotgun.kick_out }
}

/// His hands are off his own gun for the big slot's tool this frame: from `hide` of the equip until `show` of the stow. (character.ts bigOff)
pub fn big_off(b: crate::character::BigOffB) -> bool {
    if !(b.t >= 0.0) { return false; }
    let s = crate::character::bigmove().snug;
    if b.end >= 0.0 && b.t >= b.end { return (b.t - b.end) / m::max(1e-3, b.stow) < s.show; }
    b.t >= s.hide * b.equip
}

/// Where a ray enters a vertical column; -1 for a miss, an origin inside it, or beyond tmax. (character.ts columnEntry)
pub fn column_entry(ro: V3, rd: V3, tmax: f64, cx: f64, cz: f64, r: f64, bot: f64, top: f64) -> f64 {
    let ox = ro[0] - cx;
    let oz = ro[2] - cz;
    let a = rd[0] * rd[0] + rd[2] * rd[2];
    let bq = ox * rd[0] + oz * rd[2];
    let c = ox * ox + oz * oz - r * r;
    let mut c0 = -m::INF;
    let mut c1 = m::INF;
    if a < 1e-8 {
        if c > 0.0 { return -1.0; }
    } else {
        let disc = bq * bq - a * c;
        if disc < 0.0 { return -1.0; }
        let sq = m::sqrt(disc);
        c0 = (-bq - sq) / a;
        c1 = (-bq + sq) / a;
    }
    let mut s0 = -m::INF;
    let mut s1 = m::INF;
    if m::abs(rd[1]) < 1e-8 {
        if ro[1] < bot || ro[1] > top { return -1.0; }
    } else {
        s0 = (bot - ro[1]) / rd[1];
        s1 = (top - ro[1]) / rd[1];
        if s0 > s1 { let w = s0; s0 = s1; s1 = w; }
    }
    let t_in = m::max(c0, s0);
    let t_out = m::min(c1, s1);
    if t_in > t_out || t_in < 0.0 || t_in > tmax { return -1.0; }
    t_in
}

/// The action layer's weight this frame. (character.ts CharacterAnimator.actShown)
pub fn anim_act_shown(g: &crate::game::Game, c: CharId) -> f64 {
    g.ch[c].anim.act_w
}

/// A MOVES reaction has the trunk and head this frame. (character.ts CharacterAnimator.mvBusy)
pub fn anim_mv_busy(g: &crate::game::Game, c: CharId) -> bool {
    let m = &g.ch[c].anim.mvs;
    m.fl_t >= 0.0 || m.st_t >= 0.0 || m.ht_t >= 0.0 || m.dz.x > 0.0 || m.cf.x > 0.0
}

/// A MOVES reaction that also has his arms: all of them but a soft duck. (character.ts CharacterAnimator.mvHands)
pub fn anim_mv_hands(g: &crate::game::Game, c: CharId) -> bool {
    let m = &g.ch[c].anim.mvs;
    (m.fl_t >= 0.0 && !m.fl_soft) || m.st_t >= 0.0 || m.ht_t >= 0.0 || m.dz.x > 0.0 || m.cf.x > 0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected bits printed by bun from character.ts lines 1381 to 1422, run as written.
    #[test]
    fn column_entry_matches_the_typescript_bits() {
        let cases: [(V3, V3, f64, f64, f64, f64, f64, f64, u64); 10] = [
            ([0.0, 1.0, -3.0], [0.0, 0.0, 1.0], 10.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0x4005c28f5c28f5c2),
            ([0.1, 1.3, -3.7], [0.13, 0.07, 0.98], 10.0, 0.2, 0.1, 0.28, 0.4, 1.8, 0xbff0000000000000),
            ([0.1, 5.0, 0.05], [0.0, -1.0, 0.0], 10.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0x400999999999999a),
            ([1.0, 5.0, 0.05], [0.0, -1.0, 0.0], 10.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0xbff0000000000000),
            ([0.0, 2.5, -1.0], [0.01, -0.37, 0.93], 9.0, 0.03, 0.02, 0.25, 0.9, 1.6, 0xbff0000000000000),
            ([0.0, 0.5, 0.0], [0.0, 0.0, 1.0], 5.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0xbff0000000000000),
            ([0.0, 1.0, -3.0], [0.0, 0.0, 1.0], 2.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0xbff0000000000000),
            ([3.0, 1.0, -3.0], [0.0, 0.0, 1.0], 10.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0xbff0000000000000),
            ([0.0, 3.0, -3.0], [0.0, 0.0, 1.0], 10.0, 0.0, 0.0, 0.28, 0.0, 1.8, 0xbff0000000000000),
            ([0.0, -1.0, -2.0], [0.05, 0.6, 0.8], 10.0, 0.1, 0.0, 0.3, 0.2, 1.1, 0x4001002aba1a27a1),
        ];
        for (ro, rd, tmax, cx, cz, r, bot, top, want) in cases {
            assert_eq!(column_entry(ro, rd, tmax, cx, cz, r, bot, top).to_bits(), want, "{:?} {:?}", ro, rd);
        }
    }

    #[test]
    fn big_rebound_matches_the_typescript_bits() {
        assert_eq!(big_rebound(BigReboundKind::Hammer).to_bits(), 0x3fd0000000000000);
        assert_eq!(big_rebound(BigReboundKind::Shotgun).to_bits(), 0x3fc5c28f5c28f5c3);
    }

    #[test]
    fn big_off_matches_the_typescript_with_snug() {
        let cases: [(f64, f64, f64, f64, bool); 10] = [
            (-1.0, 0.6, 0.45, -1.0, false),
            (0.03, 0.6, 0.45, -1.0, false),
            (0.04, 0.6, 0.45, -1.0, true),
            (0.5, 0.6, 0.45, 1.0, true),
            (1.4, 0.6, 0.45, 1.0, true),
            (1.42, 0.6, 0.45, 1.0, true),
            (1.43, 0.6, 0.45, 1.0, false),
            (1.001, 0.6, 0.0, 1.0, false),
            (1.0009, 0.6, 0.0, 1.0, true),
            (f64::NAN, 0.6, 0.45, 1.0, false),
        ];
        for (t, equip, stow, end, want) in cases {
            assert_eq!(big_off(BigOffB { t, equip, stow, end }), want, "t {}", t);
        }
    }
}
