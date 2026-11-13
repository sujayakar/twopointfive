//! THE MAP OF A RUN being watched: the floor, asked once a stand, and who and what stands on it now. Both are reads, and everybody is on them, seen or not.
use super::Held;
use game::view::{FloorView, MapDoor, MapMan, MapView};

/// what a box's flags say of it here (game_draw's own words for them)
const NO_SHADOW: u32 = game_draw::boxes::NO_SHADOW;
const GLASS: u32 = game_draw::boxes::GLASS;

fn r2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }

/// every solid thing between knee and head height
pub fn floor(x: &Held) -> FloorView {
    let g = &x.g;
    let mut b = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
    let mut boxes = Vec::new();
    for &id in &g.engine.world.statics {
        let q = &g.tab.boxes[id];
        let f = q.flags as i64 as u32;
        if f & NO_SHADOW != 0 && f & GLASS == 0 { continue; }
        let (top, bot) = (q.c[1] + q.h[1], q.c[1] - q.h[1]);
        if top < 0.35 || bot > 1.6 || q.h[0] < 0.01 || q.h[2] < 0.01 { continue; }
        let kind = if f & GLASS != 0 { 2 } else if top >= 1.9 && bot <= 0.5 { 0 } else { 1 };
        boxes.extend([r2(q.c[0]), r2(q.c[2]), r2(q.h[0]), r2(q.h[2]), (q.yaw.to_degrees() * 10.0).round() / 10.0, kind as f64]);
        if kind == 0 { let e = (q.h[0] * q.h[0] + q.h[2] * q.h[2]).sqrt(); b = [b[0].min(q.c[0] - e), b[1].min(q.c[2] - e), b[2].max(q.c[0] + e), b[3].max(q.c[2] + e)]; }
    }
    if !b[0].is_finite() { b = [0.0, 0.0, 40.0, 28.0]; }
    FloorView { bounds: b.map(r2), boxes }
}

pub fn map(x: &Held) -> MapView {
    let g = &x.g;
    let men = game_start::figures::men(g).into_iter().map(|m| { let c = &g.ch[m.ch]; MapMan { id: c.id, x: r2(c.pos[0]), z: r2(c.pos[2]), aim: r2(c.aim_yaw), team: m.team, kind: m.kind, down: m.down, name: m.name, state: m.state } }).collect();
    let doors = g.doors.list.iter().map(|&d| { let (r, u) = (&g.dr[d], game::doors::door_dir(g, d)); MapDoor { hx: r2(r.def.hinge[0]), hz: r2(r.def.hinge[1]), ux: r2(u[0]), uz: r2(u[1]), width: r2(r.def.width), locked: r.leaf.locked, broken: r.leaf.lock_broken, off: r.leaf.off, name: r.def.name.clone() } }).collect();
    MapView { men, doors }
}
