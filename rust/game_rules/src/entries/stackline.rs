pub use ::game_types::stackline::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};

/// A leaf that stands open (or is off its hinges): what is beyond it sees through it now.
fn stack_line_open(g: &crate::game::Game, d: DoorId) -> bool {
    if crate::doors::door_off(g, d) { return true; }
    !crate::doors::door_latched(g, d) && m::abs(crate::doors::door_angle(g, d)) >= g.k.stackline.ajar
}

/// Has the floor point `p`, on the face `face` of `d`, a clear chest-height line to the mouth of its opening.
fn stack_line_sees(g: &mut crate::game::Game, d: DoorId, p: V3, face: f64) -> bool {
    let kn = g.k.stackline;
    let f = crate::guards::frame_of(g, d);
    let mut k = 0.0;
    while k < kn.bins {
        let a = f.w * (k + 0.5) / kn.bins;
        let dx = f.hx + f.ux * a + f.nx * face * kn.inset - p[0];
        let dz = f.hz + f.uz * a + f.nz * face * kn.inset - p[2];
        let l = m::hyp2(dx, dz);
        if l < 0.05 || crate::scene_collision::raycast(g, [p[0], kn.eye, p[2]], [dx / l, 0.0, dz / l], l).is_none() { return true; }
        k += 1.0;
    }
    false
}

/// The pace for a man `left` metres short of his mark: `brisk`, or none unless he is walking up to a stack on a door that stands open with a line to its mouth.
pub fn stack_line_pace(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId, left: f64, brisk: f64) -> Option<f64> {
    let on_mark = g.k.stackline.on_mark;
    let (shy, post) = {
        let row = &g.tab.orders_assignment[a];
        if !crate::orders::rows::id_is(&row.order, crate::orders::Is::Stacks) { return None; }
        let Some(shy) = row.shy else { return None; };
        if row.door != Some(shy) { return None; }
        let Some(post) = row.post else { return None; };
        (shy, post)
    };
    if left < on_mark { return None; }
    let d = shy;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    if !stack_line_open(g, d) || crate::silent::squad_silent(g, &crate::seats::here(g)) { return None; }
    let f = crate::guards::frame_of(g, d);
    let face = if (post[0] - f.hx) * f.nx + (post[2] - f.hz) * f.nz >= 0.0 { 1.0 } else { -1.0 };
    if ((p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz) * face < 0.05 || !stack_line_sees(g, d, p, face) { return None; }
    g.stats.stackline.paced += 1;
    Some(brisk)
}
