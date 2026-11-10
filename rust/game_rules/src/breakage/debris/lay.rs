use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Turn box `b` by fraction `k` of the way toward lying with its nearest face on a surface of normal `n` (k = 1: exactly flat); keeps yaw as the heading.
pub(super) fn lay_flat(g: &mut crate::game::Game, b: AnyBoxId, n: V3, k: f64) {
    lay_flat_row(&mut g.tab.edit(b), n, k);
}

/// The whole of layFlat on the box's row, so the only step the skeleton blocks is finding the row.
fn lay_flat_row(row: &mut crate::scene_boxes::Box, n: V3, k: f64) {
    let r: Quat = match row.rot { Some(q) => q, None => v3::quat::yaw(row.yaw) };
    let axes: [V3; 3] = [v3::quat::rotate(r, [1.0, 0.0, 0.0]), v3::quat::rotate(r, [0.0, 1.0, 0.0]), v3::quat::rotate(r, [0.0, 0.0, 1.0])];
    // the axis most aligned with n PER UNIT OF EXTENT: a slab ends on its face unless it arrives all but exactly on edge
    let h = row.h;
    let (mut bi, mut bd) = (0usize, -1.0);
    for i in 0..3 {
        let d = m::abs(v3::dot(axes[i], n)) / m::max(0.01, h[i]);
        if d > bd { bd = d; bi = i; }
    }
    lay_axis_row(row, bi, n, k);
}

/// A SHEET DOES NOT STAND ON ITS EDGE: one that came to rest up on end goes down on its face (a thing that is many times its thickness tall has nothing to stand on).
pub(super) fn lay_sheet(g: &mut crate::game::Game, b: AnyBoxId, n: V3) {
    const SHEET: f64 = 3.0;
    let row = &mut *g.tab.edit(b);
    let r: Quat = match row.rot { Some(q) => q, None => v3::quat::yaw(row.yaw) };
    let h = row.h;
    let (mut up, mut most) = (0usize, -1.0);
    for i in 0..3 {
        let d = m::abs(v3::dot(v3::quat::rotate(r, if i == 0 { [1.0, 0.0, 0.0] } else if i == 1 { [0.0, 1.0, 0.0] } else { [0.0, 0.0, 1.0] }), n));
        if d > most { most = d; up = i; }
    }
    let thin = if h[0] <= h[1] && h[0] <= h[2] { 0 } else if h[1] <= h[2] { 1 } else { 2 };
    if h[up] > SHEET * h[thin] { lay_axis_row(row, thin, n, 1.0); }
}

/// The turn that brings axis `bi` of the box onto n, a share k of the way.
fn lay_axis_row(row: &mut crate::scene_boxes::Box, bi: usize, n: V3, k: f64) {
    let r: Quat = match row.rot { Some(q) => q, None => v3::quat::yaw(row.yaw) };
    let axes: [V3; 3] = [v3::quat::rotate(r, [1.0, 0.0, 0.0]), v3::quat::rotate(r, [0.0, 1.0, 0.0]), v3::quat::rotate(r, [0.0, 0.0, 1.0])];
    let mut a = axes[bi];
    if v3::dot(a, n) < 0.0 { a = v3::scale(a, -1.0); }
    // shortest arc a -> n, scaled by k
    let cr = v3::cross(a, n);
    let sl = v3::len(cr);
    let cs = m::max(-1.0, m::min(1.0, v3::dot(a, n)));
    if sl < 1e-6 {
        if k >= 1.0 { row.rot = Some(v3::quat::normalize(r)); }
        return;
    }
    let ang = m::atan2(sl, cs) * m::max(0.0, m::min(1.0, k));
    let axis = v3::scale(cr, 1.0 / sl);
    let s2 = m::sin(ang / 2.0);
    let q: Quat = [axis[0] * s2, axis[1] * s2, axis[2] * s2, m::cos(ang / 2.0)];
    let rot = v3::quat::normalize(v3::quat::mul(q, r));
    row.rot = Some(rot);
    let f = v3::quat::rotate(rot, [0.0, 0.0, 1.0]);
    if m::hyp2(f[0], f[2]) > 1e-3 { row.yaw = m::atan2(f[0], f[2]); }   // heading of local +Z, the level's yaw convention
}


/// Any change of a prop's pose changes this (exact float compare: 'did it move', not a hash).
pub(super) fn pose_key(g: &crate::game::Game, p: PropId) -> f64 {
    g.tab.props_prop[p].x * 7919.0 + g.tab.props_prop[p].z * 104729.0 + g.tab.props_prop[p].yaw
}

#[cfg(test)]
mod tests {
    use super::*;

    const UP: V3 = [0.0, 1.0, 0.0];
    const AXIS: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    fn about_x(a: f64) -> Quat { [m::sin(a / 2.0), 0.0, 0.0, m::cos(a / 2.0)] }
    fn laid(h: V3, yaw: f64, rot: Option<Quat>, n: V3, k: f64) -> crate::scene_boxes::Box {
        let mut row = crate::scene_boxes::Box::default();
        row.h = h;
        row.yaw = yaw;
        row.rot = rot;
        lay_flat_row(&mut row, n, k);
        row
    }
    /// the angle from axis `i` of a turn to n, either way up
    fn off(r: Quat, i: usize, n: V3) -> f64 { m::acos(m::min(1.0, m::abs(v3::dot(v3::quat::rotate(r, AXIS[i]), n)))) }
    fn heading_is_its_yaw(row: &crate::scene_boxes::Box) {
        let f = v3::quat::rotate(row.rot.unwrap(), AXIS[2]);
        assert!((m::atan2(f[0], f[2]) - row.yaw).abs() < 1e-12, "{} {f:?}", row.yaw);
    }
    const SLAB: V3 = [0.3, 0.015, 0.2];

    #[test]
    fn a_tilted_slab_turns_part_of_the_way() {
        let q = v3::quat::normalize([0.1, 0.2, -0.3, 0.9]);
        let row = laid(SLAB, 0.7, Some(q), UP, 0.35);
        assert!((off(row.rot.unwrap(), 1, UP) - off(q, 1, UP) * 0.65).abs() < 1e-9);
        heading_is_its_yaw(&row);
    }

    #[test]
    fn a_tilted_slab_ends_flat() {
        let row = laid(SLAB, 0.7, Some(v3::quat::normalize([0.1, 0.2, -0.3, 0.9])), UP, 1.0);
        assert!(off(row.rot.unwrap(), 1, UP) < 1e-7);
        heading_is_its_yaw(&row);
    }

    /// A failure means sheets and slabs come to rest up on end.
    #[test]
    fn a_slab_nearly_on_edge_still_goes_down_on_its_face() {
        let up_on_end = about_x(80.0 * m::PI / 180.0);    // its thin axis nearly level: its long face is the nearer to the floor
        assert!(off(up_on_end, 2, UP) < off(up_on_end, 1, UP));
        let row = laid(SLAB, 0.0, Some(up_on_end), UP, 1.0);
        assert!(off(row.rot.unwrap(), 1, UP) < 1e-7);
    }

    #[test]
    fn a_box_with_no_rot_starts_from_its_yaw() {
        let n = [0.6, 0.8, 0.0];
        let from = v3::quat::yaw(-1.3);
        let row = laid([0.05, 0.4, 0.1], -1.3, None, n, 0.5);
        assert!((off(row.rot.unwrap(), 2, n) - off(from, 2, n) * 0.5).abs() < 1e-9);    // its z is the axis nearest n for its size
        heading_is_its_yaw(&row);
    }

    #[test]
    fn already_flat_writes_rot_only_at_full_k() {
        let row = laid([0.1, 0.1, 0.1], 2.1, None, UP, 1.0);
        assert!(off(row.rot.unwrap(), 1, UP) < 1e-7);
        assert_eq!(row.yaw, 2.1);
        let row = laid([0.1, 0.1, 0.1], 2.1, None, UP, 0.5);
        assert!(row.rot.is_none());
        assert_eq!(row.yaw, 2.1);
    }

    /// Lying on the face its axis points out of the other way is lying flat all the same.
    #[test]
    fn a_flat_face_on_its_flipped_axis_keeps_its_yaw() {
        let q = about_x(m::PI / 2.0);
        let row = laid([0.2, 0.1, 0.3], 0.0, Some(q), UP, 2.0);
        assert!(off(row.rot.unwrap(), 2, UP) < 1e-7);
        assert_eq!(row.yaw, 0.0);
        // tilted a little on that face, it turns the little way, not over
        let q = about_x(m::PI / 2.0 + 0.2);
        let row = laid([0.2, 0.1, 0.3], 0.0, Some(q), UP, 0.5);
        assert!((off(row.rot.unwrap(), 2, UP) - 0.1).abs() < 1e-9);
    }
}
