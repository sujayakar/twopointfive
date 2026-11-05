//! src/game/debris.ts lines 450 to 468
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Turn box `b` by fraction `k` of the way toward lying with its nearest face on a surface of normal `n` (k = 1: exactly flat); keeps yaw as the heading.   (debris.ts layFlat)
pub fn lay_flat(g: &mut crate::game::Game, b: BoxId, n: V3, k: f64) {
    let r: Quat = match g.col.boxes[b].rot { Some(q) => q, None => v3::quat::yaw(g.col.boxes[b].yaw) };
    let axes: [V3; 3] = [v3::quat::rotate(r, [1.0, 0.0, 0.0]), v3::quat::rotate(r, [0.0, 1.0, 0.0]), v3::quat::rotate(r, [0.0, 0.0, 1.0])];
    // the axis most aligned with n PER UNIT OF EXTENT: a slab ends on its face unless it arrives all but exactly on edge
    let h = g.col.boxes[b].h;
    let (mut bi, mut bd) = (0usize, -1.0);
    for i in 0..3 {
        let d = m::abs(v3::dot(axes[i], n)) / m::max(0.01, h[i]);
        if d > bd { bd = d; bi = i; }
    }
    let mut a = axes[bi];
    if v3::dot(a, n) < 0.0 { a = v3::scale(a, -1.0); }
    // shortest arc a -> n, scaled by k
    let cr = v3::cross(a, n);
    let sl = v3::len(cr);
    let cs = m::max(-1.0, m::min(1.0, v3::dot(a, n)));
    if sl < 1e-6 {
        if k >= 1.0 { g.col.boxes[b].rot = Some(v3::quat::normalize(r)); }
        return;
    }
    let ang = m::atan2(sl, cs) * m::max(0.0, m::min(1.0, k));
    let axis = v3::scale(cr, 1.0 / sl);
    let s2 = m::sin(ang / 2.0);
    let q: Quat = [axis[0] * s2, axis[1] * s2, axis[2] * s2, m::cos(ang / 2.0)];
    let rot = v3::quat::normalize(v3::quat::mul(q, r));
    g.col.boxes[b].rot = Some(rot);
    let f = v3::quat::rotate(rot, [0.0, 0.0, 1.0]);
    if m::hyp2(f[0], f[2]) > 1e-3 { g.col.boxes[b].yaw = m::atan2(f[0], f[2]); }   // heading of local +Z, the level's yaw convention
}

/// Any change of a prop's pose changes this (exact float compare: 'did it move', not a hash).   (debris.ts poseKey)
pub fn pose_key(g: &mut crate::game::Game, p: PropId) -> f64 {
    crate::props::prop_x(g, p) * 7919.0 + crate::props::prop_z(g, p) * 104729.0 + crate::props::prop_yaw(g, p)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected bits are the TypeScript's own layFlat, run under bun with EXACT on.
    fn check(h: V3, yaw: f64, rot: Option<Quat>, n: V3, k: f64, want_rot: Option<[u64; 4]>, want_yaw: u64) {
        let mut g = alloc::boxed::Box::new(crate::game::Game::blank());
        let mut row = crate::scene_boxes::Box::zeroed();
        row.h = h;
        row.yaw = yaw;
        row.rot = rot;
        let b = g.col.boxes.push(row);
        lay_flat(&mut g, b, n, k);
        assert_eq!(g.col.boxes[b].rot.map(|q| q.map(f64::to_bits)), want_rot);
        assert_eq!(g.col.boxes[b].yaw.to_bits(), want_yaw);
    }

    #[test]
    fn a_tilted_slab_turns_part_of_the_way() {
        check([0.3, 0.015, 0.2], 0.7, Some([0.1, 0.2, -0.3, 0.9]), [0.0, 1.0, 0.0], 0.35,
            Some([0x3fb1aeb767af3821, 0x3fcb18bc56947e11, 0xbfca86131b86d430, 0x3fee7bd3e1670dd2]), 0x3fd93a327f7e8bf2);
    }

    #[test]
    fn a_tilted_slab_ends_flat() {
        check([0.3, 0.015, 0.2], 0.7, Some([0.1, 0.2, -0.3, 0.9]), [0.0, 1.0, 0.0], 1.0,
            Some([0x3f74146061812c38, 0x3fcbc385e673493b, 0xbf8e1e909241c24b, 0x3fef3bf6a341b262]), 0x3fdbf9de90c3fd18);
    }

    #[test]
    fn a_box_with_no_rot_starts_from_its_yaw() {
        check([0.05, 0.4, 0.1], -1.3, None, [0.6, 0.8, 0.0], 0.5,
            Some([0x3fc79c84eb034a9e, 0xbfe4006abf776b79, 0x3fc1f311f1bc973c, 0x3fe7da4f312bde50]), 0xbff66bf18bb72c94);
    }

    #[test]
    fn already_flat_writes_rot_only_at_full_k() {
        check([0.1, 0.1, 0.1], 2.1, None, [0.0, 1.0, 0.0], 1.0,
            Some([0x0000000000000000, 0x3febc1ee5a36f2c0, 0x0000000000000000, 0x3fdfd8343c886924]), 0x4000cccccccccccd);
        check([0.1, 0.1, 0.1], 2.1, None, [0.0, 1.0, 0.0], 0.5, None, 0x4000cccccccccccd);
    }

    #[test]
    fn a_flat_face_on_its_flipped_axis_keeps_its_yaw() {
        check([0.2, 0.1, 0.3], 0.0, Some([0.7071067811865476, 0.0, 0.0, 0.7071067811865476]), [0.0, 1.0, 0.0], 2.0,
            Some([0x3fe6a09e667f3bcc, 0x0000000000000000, 0x0000000000000000, 0x3fe6a09e667f3bcc]), 0x0000000000000000);
    }
}
