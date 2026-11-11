//! WHERE HE STANDS: how wide he is, what pushes him out of what, whether he can stand up there, and being put somewhere else.
use super::*;
use crate::game::Game;
use crate::m;
use crate::v3::{self, V3};

/// he is as wide as his shoulders in a door frame: BODY_R narrows to DOOR_R on a doorway's middle line
pub fn body_r(g: &crate::game::Game, p: V3) -> f64 {
    let fit = door_fit();
    let mut k: f64 = 0.0;
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        let fc = g.dr[d].frame_centre;
        let dx = p[0] - fc[0];
        let dz = p[2] - fc[2];
        if dx * dx + dz * dz > 1.3 { continue; }
        let c = g.dr[d].def.closed_dir;
        let ux = m::cos(c);
        let uz = m::sin(c);
        let lat = m::abs(dx * ux + dz * uz);
        let out = m::abs(dz * ux - dx * uz);
        let kl = 1.0 - v3::clamp((lat - fit.lat[0]) / (fit.lat[1] - fit.lat[0]), 0.0, 1.0);
        let ko = 1.0 - v3::clamp((out - fit.out[0]) / (fit.out[1] - fit.out[0]), 0.0, 1.0);
        k = m::max(k, kl * kl * (3.0 - 2.0 * kl) * ko * ko * (3.0 - 2.0 * ko));
    }
    BODY_R - (BODY_R - DOOR_R) * k
}

/// the player's own collision: a sweep from the last settled spot, then the push-out
pub(super) fn collide_player(g: &mut crate::game::Game, y1: f64) {
    let id = g.player;
    let c = g.pl[id].ch;
    let mut pl = crate::player::CollideManPl { char: crate::player::CollideManPlChar { pos: g.ch[c].pos }, safe: g.pl[id].safe };
    collide_man(g, &mut pl, y1);
    g.ch[c].pos = pl.char.pos;
    g.pl[id].safe = pl.safe;
}

/// the same settle for any body with a place and the spot it last stood clear at
pub(super) fn collide_man(g: &crate::game::Game, pl: &mut crate::player::CollideManPl, y1: f64) {
    let mut p = pl.char.pos;
    if let Some(s) = pl.safe {
        let dx = p[0] - s[0];
        let dz = p[2] - s[2];
        let n = m::min(32.0, m::ceil(m::hyp2(dx, dz) / SWEEP_STEP));
        if n > 1.0 {
            p[0] = s[0];
            p[2] = s[2];
            let mut i = 1.0;
            while i < n {
                p[0] += dx / n;
                p[2] += dz / n;
                let r = body_r(g, p);
                crate::scene_collision::collide_circle(g, &mut p, r, 0.2, y1, Some(4.0));
                crate::scene_collision::clamp_to_world(&mut p, BODY_R);
                i += 1.0;
            }
            p[0] += dx / n;
            p[2] += dz / n;
        }
    }
    for _k in 0..4 {
        let r = body_r(g, p);
        crate::scene_collision::collide_circle(g, &mut p, r, 0.2, y1, Some(6.0));
        if !crate::scene_collision::clamp_to_world(&mut p, BODY_R) { break; }
    }
    pl.char.pos = p;
    pl.safe = Some([p[0], 0.0, p[2]]);
}

/// may he stand up here
pub fn can_stand(g: &crate::game::Game) -> bool {
    let p = g.player_pos(g.player);
    can_stand_at(g, p)
}

/// may he stand up at a spot: settles a copy against the band between crouched and standing head height
pub(super) fn can_stand_at(g: &crate::game::Game, p: V3) -> bool {
    let mut q: V3 = [p[0], p[1], p[2]];
    let r = body_r(g, p);
    crate::scene_collision::collide_circle(g, &mut q, r, 1.0, 1.7, Some(2.0));
    v3::dist_xz(q, p) < 0.01
}

/// settle the player against the statics now, swept from where he last stood clear
pub fn settle_player(g: &mut crate::game::Game) {
    let pl = g.player;
    let p = g.player_pos(pl);
    let still = match g.pl[pl].safe { Some(s) => s[0] == p[0] && s[2] == p[2], None => false };
    if crate::condition::down(g, g.pl[pl].ch) || still { return; }
    let y1 = if g.pl[pl].crouch { 1.0 } else { 1.7 };
    collide_player(g, y1);
}

/// Break off whatever the hands and feet are in the middle of that belongs to the SPOT, and stand ready
pub fn cancel_actions(g: &mut crate::game::Game) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if g.pl[pl].deed.is_some() || g.sets[g.seat].hand.sel.down_t >= 0.0 {
        g.sets[g.seat].hand.f_held = true;
    }
    crate::deed::hands_free(g);
    crate::bigkit::sam_big_drop(g);
    g.sets[g.seat].hand.sel.down_t = -1.0;
    g.sets[g.seat].hand.sel.open = false;
    g.sets[g.seat].hand.sel.ring = None;
    g.ch[c].anim.door_push = None;
    g.pl[pl].speed_sm = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
}

/// pushed out of whatever is there: did that converge (false: a gap the body cannot fit)
fn settle(g: &Game, q: &mut V3) -> bool {
    crate::scene_collision::clamp_to_world(q, BODY_R);
    let r = body_r(g, *q);
    crate::scene_collision::collide_circle(g, q, r, 0.05, 2.2, Some(6.0));
    let mut t = v3::copy(*q);
    let r = body_r(g, t);
    !crate::scene_collision::collide_circle(g, &mut t, r, 0.05, 2.2, Some(1.0)) || v3::dist_xz(t, *q) < 0.005
}

/// Sandbox / harness: stand the player at p, free of anything he was in the middle of
pub fn teleport_player(g: &mut crate::game::Game, p: V3) {
    let pl = g.player;
    if crate::condition::down(g, g.pl[pl].ch) { return; }
    if !p[0].is_finite() || !p[2].is_finite() { return; }
    cancel_actions(g);
    let mut q: V3 = [p[0], 0.0, p[2]];
    if !settle(g, &mut q) || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
        if let Some(c) = crate::scene_collision::nav_grid_nearest_free_point(g, q[0], q[2]) {
            q = c;
            settle(g, &mut q);
        }
    }
    crate::scene_collision::clamp_to_world(&mut q, BODY_R);
    let c = g.pl[pl].ch;
    { let to = q; crate::character::pos_replace(g, c, to); }
    g.pl[pl].safe = Some([q[0], 0.0, q[2]]);
    crate::character::character_update(g, c, 0.0);
}
