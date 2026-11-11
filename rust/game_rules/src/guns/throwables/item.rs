use super::*;                      // your own module: its types, its other parts' functions
use crate::ids::*;
use crate::js;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

/// The sponge round's three boxes about `c_2`, its length along `rot`'s z, the nose toward +z.
pub(in crate::guns) fn sponge_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>, c_2: V3, rot: Quat, yaw: f64) {
    let s = sponge_item();
    let flag = crate::scene_boxes::box_flag();
    let flags = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    for (z, h, albedo) in [(s.body.z, s.body.h, s.body.albedo), (s.nose.z, s.nose.h, s.nose.albedo), (s.band.z, s.band.h, s.band.albedo)] {
        out.push(item_box(g, v3::add(c_2, v3::quat::rotate(rot, [0.0, 0.0, z])), h, yaw, rot, albedo, flags).into());
    }
}

/// The thing of furniture it lies on.
pub fn item_on(g: &crate::game::Game, id: ItemId) -> Option<PropId> {
    let k = g.tab.item[id].on;
    if k != 0 { g.props.all.get(k as usize - 1).copied() } else { None }
}

pub fn item_set_on(g: &mut crate::game::Game, id: ItemId, p: Option<PropId>) {
    g.tab.item[id].on = match p {
        Some(p) => js::to_u32(g.tab.props_prop[p].id + 1.0),
        None => 0,
    };
}

/// A box for an item's part.
pub(super) fn item_box(g: &mut crate::game::Game, c_2: V3, h: V3, yaw: f64, rot: Quat, albedo: V3, flags: f64) -> FrameBoxId {
    crate::scene_boxes::box_of_frame(g, c_2, h, yaw, albedo, [0.0, 0.0, 0.0], flags, 0.0, Some(rot))
}

/// The share of its slide a thing keeps on lying snow.
pub fn bog_slide(g: &crate::game::Game, bog: f64) -> f64 {
    1.0 - bog * g.k.snowbog.grip
}

/// The share of its bounce.
pub fn bog_bounce(g: &crate::game::Game, bog: f64) -> f64 {
    1.0 - bog * g.k.snowbog.dead
}

/// How much faster than on a floor it may still be going and stop dead.
pub fn bog_stick(g: &crate::game::Game, bog: f64) -> f64 {
    bog * g.k.snowbog.stick
}
