use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, vec, vec::Vec};

/// The blows a big tool's layer reads at the record's clock: the last that has landed and the next coming, at most two.   (character.ts blowsNear)
pub fn blows_near(b: &crate::character::BlowsNearB) -> Vec<f64> {
    let (mut prev, mut next) = (f64::NEG_INFINITY, f64::INFINITY);
    for &blow in &b.blows {
        if blow <= b.t { prev = m::max(prev, blow); } else if !(b.end >= 0.0 && blow > b.end) { next = m::min(next, blow); }
    }
    let mut near = Vec::new();
    if prev > f64::NEG_INFINITY { near.push(prev); }
    if next < f64::INFINITY { near.push(next); }
    near
}

/// The game's own key tables against what a layer of the module is handed, said by name as an animator is made.   (character.ts layerTablesFit)
pub fn layer_tables_fit() {
    let cross_jab = knife_blow();
    let blows: [(&str, usize); 4] = [("cross", cross_jab.cross.keys.len()), ("jab", cross_jab.jab.keys.len()), ("slash", knife_slash().keys.len()), ("back", knife_back().keys.len())];
    for (k, n) in blows {
        crate::anim_skeleton::layer_fits(&alloc::format!("the knife's {} blow", k), n as f64, KNIFE_KEYS_MAX, None);
    }
    let (nums, vecs) = (throw_nums(), throw_vecs());
    for low in [false, true] {
        let ThrowTables { k1, k3 } = throw_tables(low);
        let side = if low { "underhand" } else { "overhand" };
        let lens = k1.iter().map(|k| k.len()).chain(k3.iter().map(|k| k.len()));
        for (name, n) in nums.iter().chain(vecs.iter()).zip(lens) {
            crate::anim_skeleton::layer_fits(&alloc::format!("the {} throw's table \"{}\"", side, name), n as f64, THROW_KEYS_MAX, Some(2.0));
        }
    }
}

/// A swing's tables as the module's layer is handed them, by name and in its order: eleven of numbers, then six of vectors. A table the underhand lacks is the overhand's (`T[n] ?? THR[n]`).   (character.ts throwTables)
pub fn throw_tables(low: bool) -> crate::character::ThrowTables {
    let t = thr();
    if !low {
        return ThrowTables {
            k1: vec![t.chest_yaw, t.pelvis_yaw, t.pitch, t.roll, t.pel_x, t.pel_y, t.pel_z, t.clav_up, t.clav_fwd, t.swiv_r, t.swiv_l],
            k3: vec![t.hand_r, t.fing_r, t.back_r, t.hand_l, t.fing_l, t.back_l],
        };
    }
    let u = thr_low();
    ThrowTables {
        k1: vec![u.chest_yaw, u.pelvis_yaw, u.pitch, u.roll, u.pel_x, u.pel_y, u.pel_z, u.clav_up, u.clav_fwd, u.swiv_r, t.swiv_l],
        k3: vec![u.hand_r, u.fing_r, u.back_r, t.hand_l, t.fing_l, t.back_l],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FNV-1a over a double's bits, the walk the TypeScript's digest made
    fn eat(h: &mut u64, x: f64) { *h = (*h ^ x.to_bits()).wrapping_mul(0x100000001b3); }
    const FNV: u64 = 0xcbf29ce484222325;

    #[test]
    fn throw_tables_hold_the_typescripts_bits() {
        let mut h = FNV;
        for low in [false, true] {
            let t = throw_tables(low);
            assert_eq!((t.k1.len(), t.k3.len()), (11, 6));
            for k in &t.k1 { for key in k { eat(&mut h, key[0]); eat(&mut h, key[1]); } }
            for k in &t.k3 { for (at, v) in k { eat(&mut h, *at); for &c in v { eat(&mut h, c); } } }
        }
        assert_eq!(h, 0x69d144554334ebd5);
    }

    #[test]
    fn the_toss_borrows_the_overhands_left_hand() {
        let (over, low) = (throw_tables(false), throw_tables(true));
        assert_eq!(low.k1[10], over.k1[10]); // swivL
        for i in 3..6 { assert_eq!(low.k3[i], over.k3[i]); } // handL, fingL, backL
        assert_ne!(low.k1[9], over.k1[9]); // swivR is its own
        assert_ne!(low.k3[0], over.k3[0]); // handR is its own
    }

    #[test]
    fn knife_keys_hold_the_typescripts_bits() {
        let mut h = FNV;
        let (kb, slash, back) = (knife_blow(), knife_slash(), knife_back());
        let all: [&[BlowKey]; 4] = [&kb.cross.keys, &kb.jab.keys, &slash.keys, &back.keys];
        for keys in all {
            for k in keys {
                eat(&mut h, k.t);
                for a in [k.at, k.blade, k.fing, k.pole] { for c in a { eat(&mut h, c); } }
            }
        }
        assert_eq!(h, 0x56072479a3f0ecf);
    }

    #[test]
    fn blows_near_answers_as_the_typescript_does() {
        // (t, end, blows, the answer's bits, printed by the TypeScript's own blowsNear under bun)
        let nan = f64::NAN;
        let cases: [(f64, f64, &[f64], &[u64]); 7] = [
            (0.3, -1.0, &[0.1, 0.2, 0.5, 0.9], &[0x3fc999999999999a, 0x3fe0000000000000]),
            (0.3, 0.6, &[0.5, 0.9, 0.1], &[0x3fb999999999999a, 0x3fe0000000000000]),
            (0.3, 0.4, &[0.5], &[]),
            (0.0, 0.0, &[], &[]),
            (0.5, 0.5, &[0.5, 0.5, 0.6], &[0x3fe0000000000000]),
            (0.3, -1.0, &[nan, 0.4], &[]),
            (0.3, -1.0, &[0.2, nan], &[0x3fc999999999999a]),
        ];
        for (t, end, blows, want) in cases {
            let got = blows_near(&BlowsNearB { t, end, blows: blows.to_vec() });
            let bits: Vec<u64> = got.iter().map(|x| x.to_bits()).collect();
            assert_eq!(bits, want);
        }
    }
}

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
mod tests_2 {
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
