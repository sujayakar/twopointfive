//! THE MAP OF A RUN being watched: the floor, asked once a stand, and who and what stands on it now. Both are reads, and everybody is on them, seen or not.
use std::fmt::Write;

/// what a box's flags say of it here (game_draw's own words for them)
const NO_SHADOW: u32 = game_draw::boxes::NO_SHADOW;
const GLASS: u32 = game_draw::boxes::GLASS;

fn r2(v: f64) -> f64 { (v * 100.0).round() / 100.0 }
fn text(s: &str) -> String { format!("\"{}\"", s.replace('\\', "").replace('"', "")) }

/// {bounds: [x0, z0, x1, z1], boxes: [x, z, half width, half depth, yaw in degrees, kind …]}: every solid thing between knee and head height (0 a wall, 1 low, 2 glass)
#[no_mangle]
pub extern "C" fn face_floor(h: u32) {
    let s = super::with_ref(h, |x| {
        let g = &x.g;
        let mut b = [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
        let mut rows = String::new();
        for &id in &g.engine.world.statics {
            let q = &g.tab.boxes[id];
            let f = q.flags as i64 as u32;
            if f & NO_SHADOW != 0 && f & GLASS == 0 { continue; }
            let (top, bot) = (q.c[1] + q.h[1], q.c[1] - q.h[1]);
            if top < 0.35 || bot > 1.6 || q.h[0] < 0.01 || q.h[2] < 0.01 { continue; }
            let kind = if f & GLASS != 0 { 2 } else if top >= 1.9 && bot <= 0.5 { 0 } else { 1 };
            let _ = write!(rows, "{}{},{},{},{},{},{}", if rows.is_empty() { "" } else { "," }, r2(q.c[0]), r2(q.c[2]), r2(q.h[0]), r2(q.h[2]), (q.yaw.to_degrees() * 10.0).round() / 10.0, kind);
            if kind == 0 { let e = (q.h[0] * q.h[0] + q.h[2] * q.h[2]).sqrt(); b = [b[0].min(q.c[0] - e), b[1].min(q.c[2] - e), b[2].max(q.c[0] + e), b[3].max(q.c[2] + e)]; }
        }
        if !b[0].is_finite() { b = [0.0, 0.0, 40.0, 28.0]; }
        format!("{{\"bounds\":[{},{},{},{}],\"boxes\":[{}]}}", r2(b[0]), r2(b[1]), r2(b[2]), r2(b[3]), rows)
    });
    super::out(s.into_bytes());
}

/// {men: [{id, x, z, aim, team, kind, down, name, state}], doors: [{hx, hz, ux, uz, width, locked, broken, off}]}: the players first, in their seats' order
#[no_mangle]
pub extern "C" fn face_map(h: u32) {
    let s = super::with_ref(h, |x| {
        let g = &x.g;
        let mut men: Vec<String> = Vec::new();
        let mut man = |c: game::ids::CharId, team: &str, kind: &str, down: bool, name: &str, state: &str| { let c = &g.ch[c]; men.push(format!("{{\"id\":{},\"x\":{},\"z\":{},\"aim\":{},\"team\":{},\"kind\":{},\"down\":{},\"name\":{},\"state\":{}}}", c.id, r2(c.pos[0]), r2(c.pos[2]), r2(c.aim_yaw), text(team), text(kind), down, text(name), text(state))); };
        for &s in &g.seats { let p = game::cmd::seat_man(g, s); let q = &g.pl[p]; man(q.ch, "player", "player", q.down, &game::seats::player_name(g, p), ""); }
        for &gd in &g.guards { let m = &g.gd[gd]; man(m.ch, m.team.as_str(), "man", m.state == game::game::GuardState::Dead, &m.callsign, m.state.as_str()); }
        for &gd in &g.civilians { let m = &g.gd[gd]; man(m.ch, "civilian", "civ", m.state == game::game::GuardState::Dead, &m.callsign, ""); }
        let doors: Vec<String> = g.doors.list.iter().map(|&d| { let (f, u) = (&g.dr[d].def, game::doors::door_dir(g, d)); format!("{{\"hx\":{},\"hz\":{},\"ux\":{},\"uz\":{},\"width\":{},\"locked\":{},\"broken\":{},\"off\":{}}}", r2(f.hinge[0]), r2(f.hinge[1]), r2(u[0]), r2(u[1]), r2(f.width), game::doors::door_locked(g, d), game::doors::door_lock_broken(g, d), game::doors::door_off(g, d)) }).collect();
        format!("{{\"men\":[{}],\"doors\":[{}]}}", men.join(","), doors.join(","))
    });
    super::out(s.into_bytes());
}
