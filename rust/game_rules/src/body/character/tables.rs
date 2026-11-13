use super::*;
use crate::ids::*;
use crate::m;
use crate::v3::V3;
use alloc::{vec, vec::Vec};

/// The blows a big tool's layer reads at the record's clock: the last that has landed and the next coming, at most two.
pub(super) fn blows_near(b: &crate::character::BlowsNearB) -> Vec<f64> {
    let (mut prev, mut next) = (f64::NEG_INFINITY, f64::INFINITY);
    for &blow in &b.blows {
        if blow <= b.t { prev = m::max(prev, blow); } else if !(b.end >= 0.0 && blow > b.end) { next = m::min(next, blow); }
    }
    let mut near = Vec::new();
    if prev > f64::NEG_INFINITY { near.push(prev); }
    if next < f64::INFINITY { near.push(next); }
    near
}

/// A swing's tables as the module's layer is handed them, by name and in its order: eleven of numbers, then six of vectors. A table the underhand lacks is the overhand's (`T[n] ?? THR[n]`).
pub(super) fn throw_tables(low: bool) -> crate::character::ThrowTables {
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

    #[test]
    fn the_toss_borrows_the_overhands_left_hand() {
        let (over, low) = (throw_tables(false), throw_tables(true));
        assert_eq!(low.k1[10], over.k1[10]); // swivL
        for i in 3..6 { assert_eq!(low.k3[i], over.k3[i]); } // handL, fingL, backL
        assert_ne!(low.k1[9], over.k1[9]); // swivR is its own
        assert_ne!(low.k3[0], over.k3[0]); // handR is its own
    }

    /// A big tool's layer eases between the blow that landed and the next.
    #[test]
    fn a_tool_reads_the_blow_that_landed_and_the_next_and_none_past_its_end() {
        let near = |t: f64, end: f64, blows: &[f64]| blows_near(&BlowsNearB { t, end, blows: blows.to_vec() });
        assert_eq!(near(0.3, -1.0, &[0.1, 0.2, 0.5, 0.9]), vec![0.2, 0.5], "the latest landed, the soonest to come");
        assert_eq!(near(0.3, 0.6, &[0.5, 0.9, 0.1]), vec![0.1, 0.5], "in any order, and nothing after the end");
        assert_eq!(near(0.3, 0.4, &[0.5]), Vec::<f64>::new(), "the only blow is past the end");
        assert_eq!(near(0.0, 0.0, &[]), Vec::<f64>::new());
        assert_eq!(near(0.5, -1.0, &[0.5, 0.5, 0.6]), vec![0.5, 0.6], "a blow on the clock has landed");
    }
}

/// How long a landed blow or shell is followed through before the tool may start back, s.
pub fn big_rebound(kind: crate::kit::BigTool) -> f64 {
    let t = &crate::character::BIGMOVE;
    if kind == crate::kit::BigTool::Hammer { t.hammer.rebound } else { t.shotgun.kick_in + t.shotgun.kick_out }
}

/// His hands are off his own gun for the big slot's tool this frame: from `hide` of the equip until `show` of the stow.
pub fn big_off(b: crate::character::BigOffB) -> bool {
    if !(b.t >= 0.0) { return false; }
    let s = &crate::character::BIGMOVE.snug;
    if b.end >= 0.0 && b.t >= b.end { return (b.t - b.end) / m::max(1e-3, b.stow) < s.show; }
    b.t >= s.hide * b.equip
}

/// Where a ray enters a vertical column; -1 for a miss, an origin inside it, or beyond tmax.
pub(super) fn column_entry(ro: V3, rd: V3, tmax: f64, cx: f64, cz: f64, r: f64, bot: f64, top: f64) -> f64 {
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

/// The action layer's weight this frame.
pub fn anim_act_shown(g: &crate::game::Game, c: CharId) -> f64 {
    g.ch[c].anim.act_w
}

/// A MOVES reaction has the trunk and head this frame.
pub(super) fn anim_mv_busy(g: &crate::game::Game, c: CharId) -> bool {
    let m = &g.ch[c].anim.mvs;
    m.fl_t >= 0.0 || m.st_t >= 0.0 || m.ht_t >= 0.0 || m.dz.x > 0.0 || m.cf.x > 0.0
}

/// A MOVES reaction that also has his arms: all of them but a soft duck.
pub(super) fn anim_mv_hands(g: &crate::game::Game, c: CharId) -> bool {
    let m = &g.ch[c].anim.mvs;
    (m.fl_t >= 0.0 && !m.fl_soft) || m.st_t >= 0.0 || m.ht_t >= 0.0 || m.dz.x > 0.0 || m.cf.x > 0.0
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    use crate::character::BIGMOVE;

    /// A man's column, 0.28 m round and 1.8 m tall, at the origin.
    fn entry(ro: V3, rd: V3, tmax: f64) -> f64 { column_entry(ro, rd, tmax, 0.0, 0.0, 0.28, 0.0, 1.8) }
    fn near(a: f64, b: f64) { assert!(m::abs(a - b) < 1e-9, "{a} against {b}"); }

    /// A round is tested against a man's column before his limbs: it answers where the round goes in, or that it does not.
    #[test]
    fn a_line_enters_a_mans_column_at_its_skin() {
        near(entry([0.0, 1.0, -3.0], [0.0, 0.0, 1.0], 10.0), 3.0 - 0.28);
        near(entry([0.1, 5.0, 0.05], [0.0, -1.0, 0.0], 10.0), 3.2);
    }

    #[test]
    fn a_line_that_misses_the_column_or_stops_short_or_starts_inside_is_no_entry() {
        assert_eq!(entry([3.0, 1.0, -3.0], [0.0, 0.0, 1.0], 10.0), -1.0, "past its side");
        assert_eq!(entry([0.0, 3.0, -3.0], [0.0, 0.0, 1.0], 10.0), -1.0, "over his head");
        assert_eq!(entry([1.0, 5.0, 0.05], [0.0, -1.0, 0.0], 10.0), -1.0, "straight down beside him");
        assert_eq!(entry([0.0, 1.0, -3.0], [0.0, 0.0, 1.0], 2.0), -1.0, "it stops short");
        assert_eq!(entry([0.0, 0.5, 0.0], [0.0, 0.0, 1.0], 5.0), -1.0, "from inside it");
        assert_eq!(entry([0.0, 1.0, 3.0], [0.0, 0.0, 1.0], 10.0), -1.0, "going away from it");
    }

    /// From `hide` of the equip until `show` of the stow, and not a moment outside.
    #[test]
    fn his_hands_are_off_his_gun_from_the_tools_hide_until_its_show() {
        let s = &BIGMOVE.snug;
        let (equip, stow, end) = (0.6, 0.45, 1.0);
        let off = |t: f64, end: f64| big_off(BigOffB { t, equip, stow, end });
        assert!(!off(-1.0, -1.0), "no tool out");
        assert!(!off(s.hide * equip - 0.001, -1.0), "still drawing it");
        assert!(off(s.hide * equip + 0.001, -1.0), "the tool is in his hands");
        assert!(off(0.5, end), "and through its work");
        assert!(off(end + s.show * stow - 0.001, end), "as he stows it");
        assert!(!off(end + s.show * stow + 0.001, end), "back on his gun");
        // a stow of no length is over at once
        assert!(!big_off(BigOffB { t: end + 0.01, equip, stow: 0.0, end }));
    }

    #[test]
    fn a_knifes_blow_has_no_more_keys_than_its_layer_holds() {
        for n in [KNIFE_BLOW.cross.keys.len(), KNIFE_BLOW.jab.keys.len(), KNIFE_SLASH.keys.len(), KNIFE_BACK.keys.len()] { assert!(n <= ::sim::layers::KNIFE_KEYS, "a blow of {n} keys"); }
    }

    /// A swing is keyed: from somewhere to somewhere, and no more keys than its layer holds.
    #[test]
    fn every_table_of_a_swing_has_the_keys_a_layer_reads() {
        for low in [false, true] {
            let t = throw_tables(low);
            for n in t.k1.iter().map(|k| k.len()).chain(t.k3.iter().map(|k| k.len())) { assert!((2..=::sim::layers::THROW_KEYS).contains(&n), "a table of {n} keys (the underhand: {low})"); }
        }
    }
}
