//! src/game/doorway.ts lines 401 to 481
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// FIGHT: a man planted in an open doorway `funnel` seconds steps off the opening. True = this file moved his feet this frame. (doorway.ts offThreshold)
pub fn off_threshold(g: &mut crate::game::Game, gd: GuardId, aim_at: Option<V3>, dt: f64, need_shot: bool) -> bool {
    if !ours(g, gd) || g.gd[gd].script.is_some() || g.touring { return false; }
    let aim_at = match aim_at { Some(a) => a, None => return false };
    let _ = state_of(g, gd);   // makes the row if it is new; every read and write below goes through the table
    let c = g.gd[gd].ch;
    let t = g.time;
    let pace = m::min(g.tune.guard_investigate, g.k.doormanners.step);
    let (step, step_by, step_rest) = { let st = g.m.doorway.states.get(gd).unwrap(); (st.step, st.step_by, st.step_rest) };
    if let Some(step) = step {
        if t > step_by || step_to(g, gd, step, pace, dt, false) {
            let funnel_rest = g.k.doormanners.funnel_rest;
            let st = g.m.doorway.states.get_mut(gd).unwrap();
            st.step = None;
            st.step_rest = t + funnel_rest;
            return false;
        }
        return true;
    }
    if t < step_rest { return false; }
    let pos = g.ch[c].pos;   // nothing below moves him before the last step_to
    let mut door: Option<DoorId> = None;
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if crate::doors::door_latched(g, d) || m::abs(crate::doors::door_angle(g, d)) < 0.6 || flat(pos, g.dr[d].frame_centre) > g.dr[d].def.width + 1.0 { continue; }
        if in_opening(g, d, pos, None) { door = Some(d); break; }
    }
    let door = match door {
        Some(d) => d,
        None => { g.m.doorway.states.get_mut(gd).unwrap().funnel_t = 0.0; return false; }
    };
    let funnel = g.k.doormanners.funnel;
    {
        let st = g.m.doorway.states.get_mut(gd).unwrap();
        st.funnel_t += dt;
        if st.funnel_t < funnel { return false; }
        st.funnel_t = 0.0;
    }
    let f = crate::guards::frame_of(g, door);
    let off = (aim_at[0] - f.hx) * f.nx + (aim_at[2] - f.hz) * f.nz;
    let sgn: f64 = if off >= 0.0 { 1.0 } else { -1.0 };
    // the squad steps THROUGH, to the side its man is on (failing that, back off it); a defender steps back on his own side, never out toward them
    let foe = g.gd[gd].team == crate::teams::Team::Hostile;
    let cov = foe;   // (stepCover: settled on)
    // them ALONG the door's wall: the half-plane says nothing there, so covered floor on either face will do
    let along = (aim_at[0] - f.hx) * f.ux + (aim_at[2] - f.hz) * f.uz - f.w / 2.0;
    let flat_on = cov && m::abs(off) < m::max(0.5, m::abs(along) * m::tan(g.k.doormanners.plane_deg * m::PI / 180.0));
    let sides: Vec<f64> = if foe { if flat_on { vec![-sgn, sgn] } else { vec![-sgn] } } else { vec![sgn, -sgn] };
    let here = if cov { exposed(g, aim_at, pos) } else { 0 };
    // the passes (line, cover): a spot with the shot; (a defender) a covered spot; then, with nobody in his sights, any free spot off the opening
    let passes: &[(bool, bool)] = if need_shot { &[(true, false)] } else if cov { &[(true, false), (false, true), (false, false)] } else { &[(true, false), (false, false)] };
    let (past, thru) = (g.k.doormanners.past, g.k.doormanners.through);
    for &(line, cover) in passes {
        for &side in &sides {
            for lat in [f.w + past, -past] {
                if foe && side == sgn && !cover { continue; }   // (the face the half-plane calls theirs: only ever under cover)
                let p: V3 = [f.hx + f.ux * lat + f.nx * side * thru, 0.0, f.hz + f.uz * lat + f.nz * side * thru];
                if !spot_free(g, gd, p, 0.8)
                    || crate::scene_collision::solid_blocked(g, [pos[0], 0.45, pos[2]], [p[0], 0.45, p[2]], None)
                    || (line && crate::scene_collision::shot_blocked(g, [p[0], 1.4, p[2]], aim_at, None, None)) { continue; }
                let mut bad = false;
                for i in 0..g.doors.list.len() {
                    let o = g.doors.list[i];
                    if flat(p, g.dr[o].frame_centre) > g.dr[o].def.width + 1.0 { continue; }
                    if in_opening(g, o, p, None) || (o != door && in_sweep(g, o, p)) { bad = true; break; }
                }
                if bad { continue; }
                // never to a spot more exposed to them than the frame he stands in, and the cover pass takes covered floor only
                if cov {
                    let e = exposed(g, aim_at, p);
                    if if cover { e > 0 } else { e > here } {
                        if !cover && e == 1 { g.stats.doorway.step_glass += 1; }
                        continue;
                    }
                }
                {
                    let st = g.m.doorway.states.get_mut(gd).unwrap();
                    st.step = Some(p);
                    st.step_by = t + 1.5;
                }
                g.stats.doorway.stepped += 1;
                if cover { g.stats.doorway.step_cover += 1; }
                step_to(g, gd, p, pace, dt, false);
                return true;
            }
        }
    }
    g.m.doorway.states.get_mut(gd).unwrap().step_rest = t + 1.0;
    false
}

/// How a man stood at p (chest and head) is exposed to a rifle at `aim_at`: 0 covered, 1 reached only through glass, 2 reached in clear air. (the closure `exposed` of offThreshold)
fn exposed(g: &mut Game, aim_at: V3, p: V3) -> i32 {
    let mut e = 0;
    for y in [1.1, 1.5] {
        let at: V3 = [p[0], y, p[2]];
        let energy = g.k.glasspen.stack_energy;
        if crate::scene_collision::shot_blocked(g, aim_at, at, Some(0.02), Some(energy)) { continue; }
        if !crate::scene_collision::solid_blocked(g, aim_at, at, None) { return 2; }
        e = 1;
    }
    e
}

/// `p` is in a door's way for a man left standing there: in some frame's own strip, or where a leaf swings. (doorway.ts doorwayInWay)
pub fn doorway_in_way(g: &mut crate::game::Game, p: V3) -> bool {
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if flat(p, g.dr[d].frame_centre) > g.dr[d].def.width + 1.0 { continue; }
        if in_opening(g, d, p, None) || in_sweep(g, d, p) { return true; }
    }
    false
}

/// Who the PASS gate has `gd` held for this frame: 'Player', a man's callsign, or None when he is not held short of a man. (doorway.ts doorwayHeldBy)
pub fn doorway_held_by(g: &mut crate::game::Game, gd: GuardId) -> Option<String> {
    let time = g.time;
    match g.m.doorway.states.get(gd) {
        Some(st) if time - st.t <= 0.25 && time >= st.t && st.held_t > 0.0 => st.by.clone(),
        _ => None,
    }
}

/// The PASS gate has `gd` held this frame: short of a frame for a man in it, or stepped off to a jamb. (doorway.ts doorwayHeld)
pub fn doorway_held(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let time = g.time;
    match g.m.doorway.states.get(gd) {
        Some(st) => time - st.t <= 0.25 && time >= st.t && (st.held_t > 0.0 || st.spot.is_some()),
        None => false,
    }
}

/// The nearest floor to `p`, within `clearM`, that is not `bad`, walkable straight from `p`, never through a frame to its other face, `apart` clear of every
/// `taken`; of the nearest ring that has any, the spot farthest from the doors by it. (doorway.ts doorwayClearNear)
pub fn doorway_clear_near(g: &mut crate::game::Game, p: V3, bad: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool, taken: Option<&[V3]>, apart: Option<f64>, face: Option<V3>) -> Option<V3> {
    let taken: &[V3] = taken.unwrap_or(&[]);
    let apart = apart.unwrap_or(g.k.doormanners.apart);
    let (half, clear_m) = (g.k.doormanners.half, g.k.doormanners.clear_m);
    // the doors by p, with p's place against each (`face`: with p in a frame's own strip, the face of that frame the answer is on)
    let mut by: Vec<(DoorId, Rel)> = Vec::new();
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if !(flat(p, g.dr[d].frame_centre) <= g.dr[d].def.width + 1.0 + clear_m) { continue; }
        let at = rel(g, d, p);
        let at = match face { Some(fc) if m::abs(at.perp) <= half => rel(g, d, fc), _ => at };
        by.push((d, at));
    }
    let mut r = 0.2;
    while r <= clear_m + 1e-6 {
        let mut best: Option<V3> = None;
        let mut bs = -1.0;
        for k in 0..12 {
            let a = k as f64 * m::PI / 6.0;
            let q: V3 = [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                || bad(g, q)
                || taken.iter().any(|t| flat(*t, q) < apart)
                || crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None) { continue; }
            let mut ok = true;
            let mut far = m::INF;
            for &(d, at) in &by {
                let rq = rel(g, d, q);
                far = m::min(far, flat(q, g.dr[d].frame_centre));
                if m::abs(at.perp) > half && rq.perp * at.perp < 0.0 && rq.along > -0.3 && rq.along < rq.f.w + 0.3 { ok = false; break; }
            }
            if ok && far > bs { best = Some(q); bs = far; }
        }
        if best.is_some() { return best; }
        r += 0.2;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The held window: a row counts only if written this quarter second, and not from the future (a load).
    #[test]
    fn held_window() {
        let mut g = Game::blank();
        let gd = GuardId(0);
        assert!(!doorway_held(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd), None);
        let mut st = St::zeroed();
        st.t = 10.0;
        st.held_t = 1.0;
        st.by = Some(String::from("Player"));
        g.m.doorway.states.set(gd, st);
        g.time = 10.25;
        assert!(doorway_held(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd).as_deref(), Some("Player"));
        g.time = 10.26;
        assert!(!doorway_held(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd), None);
        g.time = 9.9;
        assert!(!doorway_held(&mut g, gd));
        // stepped off to a jamb: held, but not short of a man
        g.time = 10.0;
        let st = g.m.doorway.states.get_mut(gd).unwrap();
        st.held_t = 0.0;
        st.spot = Some([1.0, 0.0, 1.0]);
        assert!(doorway_held(&mut g, gd));
        assert_eq!(doorway_held_by(&mut g, gd), None);
    }
}
