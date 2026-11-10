//! A TICK ON A WALL: his station along it, the way round its end, the rifle up off a stack.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `pl.wall!`, for a write: the hold itself, never held across a call that takes `g`.
macro_rules! w {
    ($g:ident, $p:ident) => {
        $g.pl[$p].wall.as_mut().unwrap()
    };
}

/// A `w.x!` of an optional number: JavaScript reads undefined as NaN in a sum or a comparison, and so does this.
#[inline]
fn un(x: Option<f64>) -> f64 {
    x.unwrap_or(f64::NAN)
}
/// `!!x` of an optional number.
#[inline]
fn truthy_opt(x: Option<f64>) -> bool {
    x.map_or(false, js::truthy)
}
/// `!!x` of an optional boolean.
#[inline]
fn yes(x: Option<bool>) -> bool {
    x == Some(true)
}
/// `(w.owed ?? 0) | (input.clicked & 5)`: the buttons kept are a word of bits in a number's clothes.
#[inline]
fn owed_with(owed: Option<f64>, clicked: f64) -> f64 {
    (js::to_i32(owed.unwrap_or(0.0)) | (js::to_i32(clicked) & 5)) as f64
}

/// Stacked: is the first WALLSNAP.bore of the rifle's line in a wall?
fn stack_in_way(g: &Game, p: PlayerId, ap: V3) -> bool {
    let c = g.pl[p].ch;
    let pos = g.ch[c].pos;
    let (dx, dz) = (ap[0] - pos[0], ap[2] - pos[2]);
    let d = m::hyp2(dx, dz);
    let hit = if d > 0.25 {
        let h = if g.pl[p].crouch { 0.85 } else { 1.3 };
        let bore = g.k.wallsnap.bore;
        crate::scene_collision::raycast(g, [pos[0], h, pos[2]], [dx / d, 0.0, dz / d], bore)
    } else {
        None
    };
    match hit {
        Some(hit) => hit.index < 1e6,
        None => false,
    }
}

/// His gun cannot get a round out this frame (the aim ring's bar). Read only.
pub fn gun_shut(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    let p = seat.player;
    let c = g.pl[p].ch;
    if g.pl[p].holstered { return false; }
    if crate::gunwall::gun_wall_is(g, c) { return true; }
    if let Some(w) = g.pl[p].wall {
        if w.kind == Some(WallHoldKind::Stack) {
            if yes(w.lean_want) || un(w.lean) > 0.0 || un(w.lean_dwell) > 0.0 {
                if un(w.lean) < 1.0 { return false; }
            } else if w.settle >= 1.0 && stack_in_way(g, p, seat.aim_point) {
                return true;
            }
        }
    }
    let raised = g.ch[c].anim.upper == crate::character::UpperMode::Aim
        || (crate::game::player_rifle_up(g, p) && g.ch[c].anim.long_stance == Some(crate::character::LongStance::Shoulder));
    if !raised { return false; }
    let a = match g.ch[c].bones.get(crate::character::Bone::UpperArmR) {
        Some(a) => *a,
        None => v3::add(g.ch[c].pos, [0.0, 1.4, 0.0]),
    };
    let muzzle = g.ch[c].muzzle;
    crate::gunwall::wood_between(g, a, muzzle)
}

/// `hold()` of stackUp: the fire sites' 'still coming into the hands' beat.
fn stack_hold(g: &mut Game, p: PlayerId) {
    g.pl[p].equip_t = m::max(g.pl[p].equip_t, 1e-3);
}

/// Stacked on the wall: is the rifle UP this frame? Clicks are held and paid out through input.clicked.
pub(super) fn stack_up(g: &mut crate::game::Game, dt: f64, input: InputId) -> bool {
    let p = g.player;
    let ws = g.k.wallsnap;
    if g.pl[p].wall.unwrap().settle < 1.0 {
        w!(g, p).up = Some(0.0);
        let clicked = g.tab.input[input].clicked;
        let w = w!(g, p);
        w.owed = Some(owed_with(w.owed, clicked));
        stack_hold(g, p);
        return false;
    }
    if crate::samstun::sam_staggered(g, &crate::seats::here(g), None) {
        let w = w!(g, p);
        w.up = Some(0.0);
        w.rise = Some(0.0);
        w.owed = Some(0.0);
        stack_hold(g, p);
        return false;
    }
    let rifle = g.pl[p].rifle;
    let pull = crate::input::input_lmb(g, input)
        || crate::input::input_lmb_hit(g, input)
        || (crate::weapons::rifle_attachment(g, rifle).is_some() && (crate::input::input_rmb(g, input) || crate::input::input_rmb_hit(g, input)))
        || {
            let w = g.pl[p].wall.unwrap();
            truthy_opt(w.owed) && !(un(w.up) > 0.0)
        };
    if { let ap = g.sets[g.seat].aim_point; stack_in_way(g, p, ap) } {
        let w = w!(g, p);
        w.up = Some(0.0);
        w.rise = Some(0.0);
        w.owed = Some(0.0);
        stack_hold(g, p);
        if crate::input::input_lmb_hit(g, input) && g.time - g.pl[p].wall.unwrap().say_t.unwrap_or(-10.0) > 1.5 {
            let now = g.time;
            w!(g, p).say_t = Some(now);
        }
        return false;
    }
    let w0 = g.pl[p].wall.unwrap();
    let was = un(w0.up) > 0.0;
    let up = if pull || (yes(w0.lean_want) && un(w0.lean) > 0.3) { ws.up_secs } else { m::max(0.0, un(w0.up) - dt) };
    w!(g, p).up = Some(up);
    if up <= 0.0 {
        let w = w!(g, p);
        w.rise = Some(0.0);
        w.owed = Some(0.0);
        return false;
    }
    let rise = if was { w0.rise.unwrap_or(0.0) + dt } else { 0.0 };
    w!(g, p).rise = Some(rise);
    if rise < ws.up_rise {
        let clicked = g.tab.input[input].clicked;
        let w = w!(g, p);
        w.owed = Some(owed_with(w.owed, clicked));
        stack_hold(g, p);
    } else if truthy_opt(w0.owed) {
        if g.sets[g.seat].orders.menu.is_none() {
            let clicked = g.tab.input[input].clicked;
            g.tab.input[input].clicked = (js::to_i32(clicked) | js::to_i32(un(w0.owed))) as f64;
        }
        w!(g, p).owed = Some(0.0);
    }
    true
}

/// Stacked at an open end: does he step out round it? The signed metres along the wall to stand past his station.
fn stack_lean(g: &mut crate::game::Game, dt: f64, cam: CameraId, p: PlayerId, dir: f64) -> f64 {
    let w = g.pl[p].wall.unwrap();
    if w.lean.is_none() { return 0.0; }
    let ws = g.k.wallsnap;
    let past = |w: &crate::player::WallHold| { let l = un(w.lean); un(w.lean_dir) * (ws.lean_along + w.lean_more.unwrap_or(0.0)) * l * l * (3.0 - 2.0 * l) };
    if crate::samstun::sam_rooted(g, &crate::seats::here(g), None) { return past(&w); }
    let able = w.kind == Some(WallHoldKind::Stack) && dir != -w.at_edge && !crate::weapons::firearm_reloading(g, g.pl[p].rifle);
    let still = yes(w.lean_want) || m::abs(w.v) < 0.05;
    let want = able && still && w.settle >= 1.0 && w.at_edge != 0.0 && steps_out(g, p, &w, lean_aim(g, cam), dt);
    minds_lean(g, p, want, able, dt);
    let w = w!(g, p);
    let out = yes(w.lean_want);
    w.lean_out_t = Some(if out { un(w.lean_out_t) + dt } else { 0.0 });
    let l = v3::clamp(un(w.lean) + (if out { dt / ws.lean_out } else { -dt / ws.lean_back }), 0.0, 1.0);
    w.lean = Some(l);
    if w.lean_more.is_some() && l <= 0.0 && !out { w.lean_more = None; }
    past(w)
}

/// What he would look at round the end: the cursor, brought down to a man's chest where it lies high and on nobody.
fn lean_aim(g: &Game, cam: CameraId) -> V3 {
    let at = g.sets[g.seat].aim_point;
    if at[1] <= g.k.wallsnap.lean_over_y || g.sets[g.seat].aim_guard.is_some() { return at; }
    let eye = g.tab.camera[cam].pos;
    let dy = at[1] - eye[1];
    if dy >= -0.5 { return at; }
    let k = (LEAN_CHEST - at[1]) / dy;
    [at[0] + (at[0] - eye[0]) * k, LEAN_CHEST, at[2] + (at[2] - eye[2]) * k]
}

/// Is that seen from past the end and not from here, at an angle and a distance worth it, and does stepping out buy a line? Out already, he
/// keeps out a little longer. Going out, he goes as much further as his gun needs to be clear of the corner.
fn steps_out(g: &mut Game, p: PlayerId, w: &crate::player::WallHold, at: V3, dt: f64) -> bool {
    let ws = g.k.wallsnap;
    let (t, n, edge, out) = (w.along, w.n, w.at_edge, yes(w.lean_want));
    let end = if edge > 0.0 { w.hi } else { w.lo };
    let corner = crate::wallsnap::ws_corner_angle((at[0] * t[0] + at[2] * t[2] - end) * edge, at[0] * n[0] + at[2] * n[2] - w.d);
    let worth = corner.dist >= ws.lean_min_dist - (if out { ws.lean_keep_dist } else { 0.0 })
        && corner.deg < ws.lean_max_deg + (if out { ws.lean_keep_deg } else { 0.0 })
        && corner.deg > crate::wallsnap::ws_seen_deg(BODY_R, BODY_R + WALL_GAP) + (if out { ws.lean_off_deg } else { ws.lean_on_deg });
    let there = edge * ws.lean_along - w.lean_off.unwrap_or(0.0);   // (how far along the wall from where he is)
    if !worth || (!out && ws.lean_clear_m > 0.0 && !buys_line(g, p, w, at, there)) { return false; }
    let more = w.lean_more.unwrap_or(0.0);
    if more < ws.lean_gun {
        let need = gun_needs(g, p, w, at, there);
        if need > more { w!(g, p).lean_more = Some(m::min(need, more + dt * ws.lean_along / ws.lean_out)); }
    }
    true
}
/// From past the end, the start of the way to it is open.
fn buys_line(g: &mut Game, p: PlayerId, w: &crate::player::WallHold, at: V3, there: f64) -> bool {
    let h = if g.pl[p].crouch { 0.85 } else { 1.3 };
    let pos = g.ch[g.pl[p].ch].pos;
    let a: V3 = [pos[0] + w.along[0] * there, h, pos[2] + w.along[2] * there];
    let d = m::hyp2(at[0] - a[0], at[2] - a[2]);
    let r = m::min(1.0, g.k.wallsnap.lean_clear_m / m::max(d, 1e-3));
    d >= 0.2 && !crate::scene_collision::shot_blocked(g, a, [a[0] + (at[0] - a[0]) * r, h + (at[1] - h) * r, a[2] + (at[2] - a[2]) * r], None, None)
}
/// The least further step from which his rifle, and the start of its round's way, are clear of the corner; 0: none is.
fn gun_needs(g: &mut Game, p: PlayerId, w: &crate::player::WallHold, at: V3, there: f64) -> f64 {
    let ws = g.k.wallsnap;
    let c = g.pl[p].ch;
    let mut s = 0.0;
    while s <= ws.lean_gun + 1e-6 {
        let (pos, k) = (g.ch[c].pos, there + w.at_edge * s);
        if crate::gunwall::gun_clear_from(g, c, pos[0] + w.along[0] * k, pos[2] + w.along[2] * k, at, ws.lean_clear_m) { return s; }
        s += m::max(0.02, ws.lean_gun_step);
    }
    0.0
}

/// He makes up his mind only once he has wanted the same for a while, and is not back in before he has been out a while or just after a shot.
fn minds_lean(g: &mut Game, p: PlayerId, want: bool, able: bool, dt: f64) {
    let ws = g.k.wallsnap;
    let shot = g.time - g.pl[p].last_fire_t < ws.after_shot;
    let w = w!(g, p);
    if Some(want) == w.lean_want { w.lean_dwell = Some(0.0); return; }
    let dwell = un(w.lean_dwell) + dt;
    w.lean_dwell = Some(dwell);
    let stays = !want && able && (un(w.lean_out_t) < ws.min_out || shot);
    if stays || dwell < (if want { ws.dwell_out } else { ws.dwell_back }) { return; }
    w.lean_want = Some(want);
    w.lean_dwell = Some(0.0);
    if want { w.lean_dir = Some(w.at_edge); w.side = w.at_edge; }
}

/// One frame of the hold's pose: the take and the landing to the animator; on the stack the stance, the press and the rifle's carriage.
pub(super) fn wall_snap_pose(g: &mut crate::game::Game, dt: f64, aiming: bool) {
    let p = g.player;
    let c = g.pl[p].ch;
    let ws = g.k.wallsnap;
    let w0 = g.pl[p].wall.unwrap();
    let stacked = w0.kind == Some(WallHoldKind::Stack);
    g.ch[c].anim.wall_take = if w0.settle < 1.0 { w0.settle } else { -1.0 };
    g.ch[c].anim.stance_in = if stacked { ws.slow_in * 13.0 / 22.0 } else { ws.slow_in };
    let landed = w0.settle >= 1.0 && w0.seated;
    if landed {
        if w0.hit_t == Some(0.0) {
            wall_voice(g, w0, true);
        }
        let w = w!(g, p);
        w.hit_t = Some(un(w.hit_t) + dt);
    }
    let hit_t = un(g.pl[p].wall.unwrap().hit_t);
    g.ch[c].anim.wall_hit = if landed { crate::wallsnap::ws_hit(g, hit_t) } else { 0.0 };
    if stacked {
        g.ch[c].anim.stance = crate::character::Stance::Stack;
        g.ch[c].anim.stack_side = w0.side;
        g.ch[c].anim.stack_press = w0.side * STACK_PRESS * (1.0 - w0.lean.unwrap_or(0.0));
        arm_rifle(g, true, Some(if aiming { crate::character::LongStance::Shoulder } else { crate::character::LongStance::LowLong }));
    } else {
        g.ch[c].anim.stack_press = 0.0;
    }
}

/// The camera on the wall: the look-at eased into the room and toward the side he looks; through a stack's step-out it holds still. `w` is `pl.wall`, read and written there.
fn wall_snap_cam(g: &mut crate::game::Game, cam: CameraId, pl: PlayerId) {
    let p = pl;
    let w0 = g.pl[p].wall.unwrap();
    if w0.settle < 1.0 { return; }
    let ws = g.k.wallsnap;
    let (n, t) = (w0.n, w0.along);
    let po = g.tab.camera[cam].peek_offset.unwrap_or([0.0, 0.0, 0.0]);
    let out = truthy_opt(w0.lean) || yes(w0.lean_want) || truthy_opt(w0.lean_off);
    if !out || w0.cam_along.is_none() {
        w!(g, p).cam_along = Some(w0.side * ws.cam_side * (1.0 - w0.peek));
    }
    let cam_along = un(g.pl[p].wall.unwrap().cam_along);
    let along = cam_along - (if out { w0.lean_off.unwrap_or(0.0) * (1.0 - g.tab.camera[cam].aim_lead) } else { 0.0 });
    let room = ws.cam_room * (if w0.kind == Some(WallHoldKind::Stack) { 0.5 } else { 1.0 });
    g.tab.camera[cam].peek_offset = Some([po[0] + n[0] * room + t[0] * along, 0.0, po[2] + n[2] * room + t[2] * along]);
    w!(g, p).cam = true;
}

/// The face he is on, as a tick along it reads it: its normal, its way along, where he stands off it, the ends of his run and how tall he is.
#[derive(Clone, Copy)]
struct Face { n: V3, t: V3, base: f64, lo: f64, hi: f64, y1: f64 }
impl Face {
    fn of(g: &Game, p: PlayerId, w: &crate::player::WallHold) -> Face {
        Face { n: w.n, t: w.along, base: w.d + BODY_R + WALL_GAP, lo: w.lo + BODY_R, hi: w.hi - BODY_R, y1: if g.pl[p].crouch { 1.0 } else { 1.7 } }
    }
    /// how far along it a point is, and how far off its plane
    fn along(&self, q: V3) -> f64 { q[0] * self.t[0] + q[2] * self.t[2] }
    fn out(&self, q: V3) -> f64 { q[0] * self.n[0] + q[2] * self.n[2] }
    fn at(&self, u: f64, out: f64) -> V3 { [self.t[0] * u + self.n[0] * out, 0.0, self.t[2] * u + self.n[2] * out] }
}

/// Where he actually is, read back into the hold; shoved off the face or past its end he lets go (false). `firm`: what stopped him stops his pace too.
fn wall_take_back(g: &mut Game, p: PlayerId, f: Face, firm: bool, dir: f64) -> bool {
    let w0 = g.pl[p].wall.unwrap();
    let pos = g.ch[g.pl[p].ch].pos;
    let u = f.along(pos) - w0.lean_off.unwrap_or(0.0);
    let off = f.out(pos) - f.base;
    if u < f.lo - 0.25 || u > f.hi + 0.25 || off > WALL_SHOVE_OFF {
        if w0.seated { wall_voice(g, w0, false); }
        else { crate::interact::refuse(g, p, Some(crate::view::ThingKind::Wall), "can't get to that wall"); }
        release_wall(g);
        return false;
    }
    let w = w!(g, p);
    w.seated = true;
    if m::abs(u - w.u) > 0.003 {
        if firm && dir != 0.0 && (u - w.u) * dir < 0.0 { w.v = 0.0; w.block_dir = dir; w.block_t = 0.0; }
        else if firm && (u - w.u) * w.v < 0.0 { w.v = 0.0; }
        w.u = v3::clamp(u, f.lo, f.hi);
    }
    w.off = m::max(0.0, off);
    true
}

/// The take onto the face eases out on its own clock.
fn wall_ease(s: f64, ease_pow: f64) -> f64 { if s < 1.0 { crate::wallsnap::ws_ease(s, ease_pow) } else { 1.0 } }

/// One tick on the wall, in place of free movement; false = he has just come off it, and goes on as a free man in the same tick.
pub(super) fn move_on_wall(g: &mut crate::game::Game, dt: f64, cam: CameraId, mx: f64, mz: f64, ml: f64) -> bool {
    let p = g.player;
    let f = Face::of(g, p, &g.pl[p].wall.unwrap());
    hold_fits_gun(g, p);
    let (dir, away) = stick_on_face(g, p, f, mx, mz, ml);
    if pulls_away(g, p, away, dt) { wall_step_off(g, mx, mz, ml, WallStepOffHow::Away); return false; }
    if dir != 0.0 { w!(g, p).side = dir; }
    if g.pl[p].wall.unwrap().settle >= 1.0 && !wall_take_back(g, p, f, false, dir) { return false; }
    g.pl[p].sprinting = false;
    let (u, u_was, e_prev, stand) = slide(g, p, f, dir, dt);
    let lean = stack_lean(g, dt, cam, p, dir);
    onto_face(g, p, f, u, u_was, f.at(u + lean, stand), e_prev);
    collide_player(g, f.y1);
    keep_off_guards(g, dt, f.y1, None);
    if lean != 0.0 || truthy_opt(g.pl[p].wall.unwrap().lean_off) {
        let got = f.along(g.ch[g.pl[p].ch].pos) - w!(g, p).u;
        w!(g, p).lean_off = Some(if lean > 0.0 { v3::clamp(got, 0.0, lean) } else if lean < 0.0 { v3::clamp(got, lean, 0.0) } else { 0.0 });
    }
    if g.pl[p].wall.unwrap().settle >= 1.0 && !wall_take_back(g, p, f, true, dir) { return false; }
    let v = g.pl[p].wall.unwrap().v;
    g.ch[g.pl[p].ch].vel = [f.t[0] * v, 0.0, f.t[2] * v];
    g.pl[p].speed_sm = m::abs(v);
    peek_round(g, p, f, cam, dir, dt);
    wall_snap_cam(g, cam, p);
    true
}

/// The hold is the one his hands can do: stacked with the rifle up, his back flat to it with anything else, which is put away.
fn hold_fits_gun(g: &mut Game, p: PlayerId) {
    let (stacked, rifle) = (on_stack(g, p), crate::game::player_rifle_up(g, p));
    if stacked && !rifle {
        let drawn = !g.pl[p].holstered && g.pl[p].dragging().is_none();
        let w = w!(g, p);
        w.kind = None;
        w.up = Some(0.0);
        w.drawn = drawn;
    }
    if !stacked && rifle {
        let w = w!(g, p);
        w.kind = Some(WallHoldKind::Stack);
        w.up = Some(0.0);
        w.rise = Some(0.0);
        w.owed = Some(0.0);
        w.drawn = true;
        if w.lean.is_none() {
            w.lean = Some(0.0);
            w.lean_want = Some(false);
            w.lean_dir = Some(w.side);
            w.lean_dwell = Some(0.0);
            w.lean_out_t = Some(0.0);
            w.lean_off = Some(0.0);
        }
    }
    if !g.pl[p].holstered && wall_not_stack(g, p) {
        w!(g, p).drawn = true;
        set_holstered(g, true, Some(true));
    }
}

/// What the stick says on a face: which way along it (+ his right, 0: neither) and how much straight off it.
fn stick_on_face(g: &Game, p: PlayerId, f: Face, mx: f64, mz: f64, ml: f64) -> (f64, f64) {
    if ml <= 0.0 { return (0.0, 0.0); }
    let deg = g.k.wallsnap.along_deg;
    let (a, away) = (f.along([mx, 0.0, mz]), f.out([mx, 0.0, mz]));
    let wide = deg > 0.0;
    let goes = if wide { m::abs(a) >= m::cos(deg * v3::DEG) } else { m::abs(a) > 0.2 && m::abs(a) >= m::abs(away) - 0.05 };
    let dir = if !goes { 0.0 } else if a > 0.0 { 1.0 } else { -1.0 };
    let w = g.pl[p].wall.unwrap();
    let room = if dir > 0.0 { w.u < f.hi - 0.02 } else { w.u > f.lo + 0.02 };
    (dir, if wide && dir != 0.0 && dir != w.block_dir && room { 0.0 } else { away })
}
/// The stick kept off the face for long enough: he steps off it.
fn pulls_away(g: &mut Game, p: PlayerId, away: f64, dt: f64) -> bool {
    let limit = if away > 0.82 { g.k.wallsnap.away_secs } else { g.k.wallsnap.away_diag };
    let w = w!(g, p);
    w.away_t = if away > 0.5 { w.away_t + dt } else { 0.0 };
    w.away_t > limit
}

/// His pace along the face and the station it takes him to, the whole of him kept on it and out of what stands by it. Returns that station,
/// the one he was at, how far in the take was, and how far off the face's plane he stands.
fn slide(g: &mut Game, p: PlayerId, f: Face, dir: f64, dt: f64) -> (f64, f64, f64, f64) {
    let (ease_pow, pace) = (g.k.wallsnap.ease_pow, if g.pl[p].crouch { g.tune.player_crouch } else { WALL_PACE });
    let w = w!(g, p);
    if w.block_dir != 0.0 && (dir != w.block_dir || { w.block_t += dt; w.block_t > 0.5 }) { w.block_dir = 0.0; w.block_t = 0.0; }
    let want = if dir == w.block_dir { 0.0 } else { dir * pace };
    w.v = v3::damp(w.v, want, if m::abs(want) > m::abs(w.v) && want * w.v >= 0.0 { 6.0 } else { 12.0 }, dt);
    if want == 0.0 && m::abs(w.v) < 0.02 { w.v = 0.0; }
    let mut u = w.u + w.v * dt;
    if u >= f.hi { u = f.hi; if w.v > 0.0 { w.v = 0.0; } } else if u <= f.lo { u = f.lo; if w.v < 0.0 { w.v = 0.0; } }
    let (u_was, off, e_prev) = (w.u, w.off, wall_ease(w.settle, ease_pow));
    w.settle = m::min(1.0, w.settle + dt / w.take_t);
    let settled = w.settle >= 1.0;
    if settled && off > 0.0 { sinks_in(g, p, f, off, dt); }
    let mut stand = f.base + w!(g, p).off;
    if settled && m::abs(u - u_was) > 1e-6 {
        let mut q = f.at(u, stand);
        crate::scene_collision::collide_circle(g, &mut q, BODY_R, 0.2, f.y1, Some(4.0));
        let (kept, out) = (f.along(q) - u_was, f.out(q) - stand);
        let w = w!(g, p);
        if kept / (u - u_was) < 0.3 || w.off + out > 0.12 {
            u = u_was;
            w.v = 0.0;
            if dir != 0.0 { w.block_dir = dir; w.block_t = 0.0; }
        } else {
            u = v3::clamp(u_was + kept, f.lo, f.hi);
            if out > 0.0 { w.off += out; stand = f.base + w.off; }
        }
    }
    w!(g, p).u = u;
    (u, u_was, e_prev, stand)
}
/// Held off the face by something he has passed, he comes back in to it where there is room.
fn sinks_in(g: &mut Game, p: PlayerId, f: Face, off: f64, dt: f64) {
    let want = m::max(0.0, off - 0.8 * dt);
    let u = w!(g, p).u;
    let mut q = f.at(u, f.base + want);
    crate::scene_collision::collide_circle(g, &mut q, BODY_R, 0.2, f.y1, Some(4.0));
    if m::abs(f.along(q) - u) < 1e-3 && f.out(q) - (f.base + want) < 0.5 * (off - want) { w!(g, p).off = want; }
}

/// Onto the face at his station: at once, or while he is still taking it a share of the way that is left.
fn onto_face(g: &mut Game, p: PlayerId, f: Face, u: f64, u_was: f64, to: V3, e_prev: f64) {
    let c = g.pl[p].ch;
    let pos = g.ch[c].pos;
    if e_prev >= 1.0 { crate::character::pos_replace(g, c, [to[0], pos[1], to[2]]); return; }
    if u != u_was { crate::character::pos_replace(g, c, [pos[0] + f.t[0] * (u - u_was), pos[1], pos[2] + f.t[2] * (u - u_was)]); }
    let w = g.pl[p].wall.unwrap();
    let e = wall_ease(w.settle, g.k.wallsnap.ease_pow);
    let k = if e < 1.0 { (e - e_prev) / (1.0 - e_prev) } else { 1.0 };
    let pos = g.ch[c].pos;
    let (mut x, mut z) = ((to[0] - pos[0]) * k, (to[2] - pos[2]) * k);
    let l = m::hyp2(x, z);
    let most = m::hyp2(to[0] - w.from[0], to[2] - w.from[2]) * (e - e_prev) + 0.02;
    if l > most { x *= most / l; z *= most / l; }
    crate::character::pos_replace(g, c, [pos[0] + x, pos[1], pos[2] + z]);
}

/// At an open end, going on past it looks round it: his head and the camera.
fn peek_round(g: &mut Game, p: PlayerId, f: Face, cam: CameraId, dir: f64, dt: f64) {
    let w = w!(g, p);
    w.at_edge = if w.u >= f.hi - 0.005 && w.open_hi { 1.0 } else if w.u <= f.lo + 0.005 && w.open_lo { -1.0 } else { 0.0 };
    let on = w.at_edge != 0.0 && dir == w.at_edge;
    if on { w.peek_dir = w.at_edge; }
    w.peek = v3::clamp(w.peek + (if on { 1.0 } else { -1.0 }) * dt / WALL_PEEK_SECS, 0.0, 1.0);
    let (side, e) = (w.peek_dir, w.peek * w.peek * (3.0 - 2.0 * w.peek));
    let past = side * (BODY_R + 0.6);
    g.tab.camera[cam].peek_offset = if e > 0.0 { Some([(f.t[0] * past + f.n[0] * 0.3) * e, 0.0, (f.t[2] * past + f.n[2] * 0.3) * e]) } else { None };
    g.ch[g.pl[p].ch].anim.look_yaw_extra = -side * WALL_PEEK_LOOK * e;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Only the fire buttons' bits (1 and 4) are owed a round, what was owed stays, none owed is 0.
    #[test]
    fn owed_keeps_the_fire_bits() {
        assert_eq!(owed_with(None, 0.0), 0.0);
        assert_eq!(owed_with(None, 7.0), 5.0);   // 2 (the middle button) is not owed a round
        assert_eq!(owed_with(Some(4.0), 1.0), 5.0);
        assert_eq!(owed_with(Some(2.0), 8.0), 2.0);
    }

}
