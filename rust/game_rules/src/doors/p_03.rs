//! src/game/doors.ts lines 689 to 1040
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A box's row (`Box.*`, scene/boxes.ts:66). Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows, while the index says `g.col.boxes[id].c`. Once the table is back, the body is that row.   (doors.ts:725)
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// Hold-to-crack: ease a near-closed leaf open silently, up to `max_angle`.   (doors.ts Doors.crack)
pub fn doors_crack(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, dt: f64, keys: Option<&JsSet<String>>, rate: Option<f64>, max_angle: Option<f64>) {
    let rate = rate.unwrap_or(0.55);
    let max_angle = max_angle.unwrap_or(0.6);
    if g.dr[d].leaf.off || doors_locked_out(g, d, from, keys) {
        return;
    }
    let [hx, hz] = g.dr[d].def.hinge;
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - hx, from[2] - hz);
    let perp = u[0] * pz - u[1] * px;
    let sgn = door_swing(g, d);
    if (if perp >= 0.0 { 1.0 } else { -1.0 }) == door_swing(g, d) {
        let pull = g.dr[d].leaf.pull_t;
        g.dr[d].leaf.pull_t = m::max(pull, 0.3);
    }
    g.dr[d].leaf.latched = false;
    g.dr[d].leaf.closing = false;
    g.dr[d].leaf.vel = 0.0;
    g.dr[d].leaf.quiet_t = -1.5;
    g.dr[d].leaf.last_who = who;
    let a = g.dr[d].leaf.angle;
    if m::abs(a) < max_angle {
        let to = a + sgn * rate * dt;
        let (lo, hi) = (door_min_a(g, d), door_max_a(g, d));
        g.dr[d].leaf.angle = v3::clamp(to, lo, hi);
    }
}

/// A tap on F: closed, it swings open away from the user; open, it is pulled to; locked against him, the rattle or his key's clack.   (doors.ts Doors.use)
pub fn doors_use(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, keys: Option<&JsSet<String>>) {
    if g.dr[d].leaf.off {
        return;
    }
    if doors_locked_out(g, d, from, keys) {
        doors_rattle(g, d, who, None);
        return;
    }
    if doors_key_opens(g, d, from, keys) {
        let pos = door_pos(g, d);
        doors_sound(g, d, DoorSound::Unlock, pos, who, DOORS_KEY_LEVEL);
    }
    let [hx, hz] = g.dr[d].def.hinge;
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - hx, from[2] - hz);
    let perp = u[0] * pz - u[1] * px;
    if door_is_closed(g, d) {
        let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
        g.dr[d].leaf.latched = false;
        g.dr[d].leaf.closing = false;
        let v = sgn * (if sgn != door_swing(g, d) { PULL_VEL } else { 3.2 });
        g.dr[d].leaf.vel = v;
        g.dr[d].leaf.quiet_t = -1.0;
        let pos = door_pos(g, d);
        doors_sound(g, d, DoorSound::Creak, pos, who, 0.2);
    } else {
        doors_pull_to(g, d, who);
    }
    g.dr[d].leaf.last_who = who;
}
