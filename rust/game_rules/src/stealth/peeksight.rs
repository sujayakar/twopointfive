use crate::m;
use crate::v3::V3;

/// The player's eye while he peeks round the end of a wall, in place of his own (`own`), or None: no hold, no peek in.
pub(super) fn peek_eye(g: &crate::game::Game, own: crate::fogofwar::FogEye) -> Option<crate::fogofwar::FogEye> {
    let pl = g.player;
    let w = g.pl[pl].wall?;
    if !(w.peek > 0.0) || crate::condition::down(g, g.pl[pl].ch) { return None; }
    let (out, off, turn_deg) = (g.k.peeksight.out, g.k.peeksight.off, g.k.peeksight.turn_deg);
    let e = ease(w.peek);
    let (s, t, n) = (w.peek_dir, w.along, w.n);
    let (along, inward) = lean(own.at, s, t, n, w.d, w.lo, w.hi, out, off);
    let mut reach = e;
    if crate::scene_collision::solid_blocked(g, own.at, lean_at(own.at, t, n, along, inward, reach), None) {
        let (mut lo, mut hi) = (0.0, reach);
        for _ in 0..5 {
            let mid = (lo + hi) / 2.0;
            if crate::scene_collision::solid_blocked(g, own.at, lean_at(own.at, t, n, along, inward, mid), None) { hi = mid; } else { lo = mid; }
        }
        reach = m::max(0.0, lo - 0.08);
    }
    let p = lean_at(own.at, t, n, along, inward, reach);
    if matches!(w.kind, Some(crate::player::WallHoldKind::Stack)) { return Some(eye_at(own, p, own.look)); }
    Some(eye_at(own, p, turned_look(own.look, s, t, n, e, turn_deg)))
}

fn ease(peek: f64) -> f64 {
    peek * peek * (3.0 - 2.0 * peek)
}

/// How far along the wall and how far toward its plane the head goes at full peek.
fn lean(own_at: V3, s: f64, t: V3, n: V3, d: f64, lo: f64, hi: f64, out: f64, off: f64) -> (f64, f64) {
    let u = own_at[0] * t[0] + own_at[2] * t[2];
    let dist = own_at[0] * n[0] + own_at[2] * n[2] - d;
    let along = m::max(0.0, if s > 0.0 { hi + out - u } else { u - (lo - out) }) * s;
    let inward = m::max(0.0, dist - off);
    (along, inward)
}

/// The head's place a share `k` of the way out.
fn lean_at(own_at: V3, t: V3, n: V3, along: f64, inward: f64, k: f64) -> V3 {
    [own_at[0] + (t[0] * along - n[0] * inward) * k, own_at[1], own_at[2] + (t[2] * along - n[2] * inward) * k]
}

/// The look turned with the peek, by angle, from his own to `turn_deg`: by way of 'along the wall', never the room behind him.
fn turned_look(own_look: V3, s: f64, t: V3, n: V3, e: f64, turn_deg: f64) -> V3 {
    let ox = (own_look[0] * t[0] + own_look[2] * t[2]) * s;
    let oy = -(own_look[0] * n[0] + own_look[2] * n[2]);
    let from = m::atan2(oy, ox);
    let a = from + (turn_deg * m::PI / 180.0 - from) * e;
    let cs = m::cos(a) * s;
    let sn = m::sin(a);
    [t[0] * cs - n[0] * sn, 0.0, t[2] * cs - n[2] * sn]
}

/// The eye the peek gives: `pos` and `at` both the head's place; fov, near and wand absent.
fn eye_at(own: crate::fogofwar::FogEye, p: V3, look: V3) -> crate::fogofwar::FogEye {
    crate::fogofwar::FogEye { id: own.id, pos: p, look, at: p, fov: None, near: None, wand: None }
}

#[cfg(test)]
mod tests {
    use crate::v3;
    use super::*;

    // A wall along x with him on its +z side, half a metre off it at x 2: its face runs from x -3 to x 4.
    const T: V3 = [1.0, 0.0, 0.0];
    const N: V3 = [0.0, 0.0, 1.0];
    const AT: V3 = [2.0, 1.6, 0.5];
    const OUT: f64 = 0.3;
    const OFF: f64 = 0.1;
    fn near(a: V3, b: V3) -> bool { v3::dist(a, b) < 1e-12 }
    fn eye(s: f64, k: f64, at: V3) -> V3 {
        let (along, inward) = lean(at, s, T, N, 0.0, -3.0, 4.0, OUT, OFF);
        lean_at(at, T, N, along, inward, k)
    }

    /// At full peek his head is `out` past the wall's end, whichever end, and `off` from its plane, at his own height; not peeking, it is where it was.
    #[test]
    fn a_full_peek_puts_his_head_past_the_end_of_the_wall() {
        assert!(near(eye(1.0, 0.0, AT), AT));
        assert!(near(eye(1.0, 1.0, AT), [4.3, 1.6, 0.1]));
        assert!(near(eye(-1.0, 1.0, AT), [-3.3, 1.6, 0.1]));
        assert!(near(eye(1.0, 0.5, AT), [3.15, 1.6, 0.3]), "half way out is half way along and half way in");
    }

    /// Already past that point or closer to the plane, a peek does not pull him back.
    #[test]
    fn a_peek_never_pulls_his_head_back() {
        assert!(near(eye(1.0, 1.0, [5.0, 1.6, 0.05]), [5.0, 1.6, 0.05]));
    }

    /// The look starts as his own and comes round to the row's angle: square through the opening, away from his side of the wall.
    #[test]
    fn a_peek_turns_his_look_into_the_opening() {
        let own = [0.31, 0.0, -0.95];
        let own = v3::normalize(own);
        assert!(near(turned_look(own, 1.0, T, N, 0.0, 90.0), own));
        assert!(v3::dist(turned_look(own, 1.0, T, N, 1.0, 90.0), [0.0, 0.0, -1.0]) < 1e-9);
        assert!(v3::dist(turned_look(own, -1.0, T, N, 1.0, 90.0), [0.0, 0.0, -1.0]) < 1e-9);
        assert!(v3::dist(turned_look(own, 1.0, T, N, 1.0, 0.0), [1.0, 0.0, 0.0]) < 1e-9, "at 0 degrees, along the wall toward that end");
        assert!(v3::dist(turned_look(own, -1.0, T, N, 1.0, 0.0), [-1.0, 0.0, 0.0]) < 1e-9);
    }

    /// Looking into his room as he starts, the look comes round by way of 'along the wall', never across the room behind him.
    #[test]
    fn a_peek_from_a_look_into_the_room_comes_round_along_the_wall() {
        let into_room = N;
        for s in [1.0, -1.0] {
            assert!(v3::dist(turned_look(into_room, s, T, N, 0.5, 90.0), [s, 0.0, 0.0]) < 1e-9, "going {s}");
        }
    }

    /// The peek starts and ends slowly.
    #[test]
    fn a_peek_eases_in_and_out() {
        assert_eq!((ease(0.0), ease(0.5), ease(1.0)), (0.0, 0.5, 1.0));
        assert!(ease(0.1) < 0.1 && ease(0.9) > 0.9);
    }
}
