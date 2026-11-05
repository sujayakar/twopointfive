//! src/game/player.ts lines 1281 to 1610
use super::*;                      // your own module: its types, its other parts' functions
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

// ---- what the skeleton still lacks: each body stays whole and only this step waits (notes.md, ## Skeleton)
/// `g.audio.play(name, at, gain, { rate })` (player.ts:1372, 1518): `game_audio_like_play`'s opts hold `crate::Opaque`, which cannot carry a rate.
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }
/// `gd.flashlight.enabled` (player.ts:1587): a LightId has no table and RtLight no accessors.
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.enabled of a LightId (no entry in the index)") }

/// `pl.rifle.attachment` (weapons.ts:533): the stub `rifle_attachment()` takes no id, so the getter is read off the gun's own row here.
fn rifle_attachment_of(g: &mut Game, rifle: FirearmId) -> Option<crate::weapons::RifleAttachment> {
    let fa = &g.fa[rifle];
    if fa.att_idx >= 0 { fa.attachments.get(fa.att_idx as usize).copied() } else { None }
}

/// `gd.path = []; gd.pathGoal = null`: a whole new path, so its serial moves, and the goal is no longer the standoff's own point.
fn lose_path(g: &mut Game, gd: GuardId) {
    let m_ = &mut g.gd[gd];
    m_.path = Vec::new();
    m_.path_serial = m_.path_serial.wrapping_add(1);
    m_.path_goal = None;
    m_.path_goal_orbit = false;
}

/// Esc with the launcher's button down: the gesture is let go of, nothing sent.   (player.ts launchCancel)
pub fn launch_cancel(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    if g.pl[p].launch_held.is_none() { return false; }
    g.pl[p].launch_held = None;
    g.pl[p].throw_preview = None;
    true
}

/// Stacked: is the first WALLSNAP.bore of the rifle's line in a wall?   (player.ts stackInWay)
pub fn stack_in_way(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    let (ap, pos) = (g.aim_point, g.ch[c].pos);
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

/// His gun cannot get a round out this frame (the aim ring's bar). Read only.   (player.ts gunShut)
pub fn gun_shut(g: &mut crate::game::Game) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    if g.pl[p].holstered { return false; }
    if crate::gunwall::gun_wall_is(g, c) { return true; }
    if let Some(w) = g.pl[p].wall {
        if w.kind == Some(WallHoldKind::Stack) {
            if yes(w.lean_want) || un(w.lean) > 0.0 || un(w.lean_dwell) > 0.0 {
                if un(w.lean) < 1.0 { return false; }
            } else if w.settle >= 1.0 && stack_in_way(g) {
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

/// Stacked on the wall: is the rifle UP this frame? Clicks are held and paid out through input.clicked.   (player.ts stackUp)
pub fn stack_up(g: &mut crate::game::Game, dt: f64, input: InputId) -> bool {
    let p = g.player;
    let ws = g.k.wallsnap;
    if g.pl[p].wall.unwrap().settle < 1.0 {
        w!(g, p).up = Some(0.0);
        if ws.slow {
            let clicked = g.tab.input[input].clicked;
            let w = w!(g, p);
            w.owed = Some(owed_with(w.owed, clicked));
        }
        stack_hold(g, p);
        return false;
    }
    if crate::samstun::sam_staggered(g, None) {
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
        || (rifle_attachment_of(g, rifle).is_some() && (crate::input::input_rmb(g, input) || crate::input::input_rmb_hit(g, input)))
        || {
            let w = g.pl[p].wall.unwrap();
            ws.slow && truthy_opt(w.owed) && !(un(w.up) > 0.0)
        };
    if stack_in_way(g) {
        let w = w!(g, p);
        w.up = Some(0.0);
        w.rise = Some(0.0);
        w.owed = Some(0.0);
        stack_hold(g, p);
        if crate::input::input_lmb_hit(g, input) && g.time - g.pl[p].wall.unwrap().say_t.unwrap_or(-10.0) > 1.5 {
            let now = g.time;
            w!(g, p).say_t = Some(now);
            crate::game::game_msg(g, "the wall is in the way, so step out past its end or come off it");
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
        if g.orders.menu.is_none() {
            let clicked = g.tab.input[input].clicked;
            g.tab.input[input].clicked = (js::to_i32(clicked) | js::to_i32(un(w0.owed))) as f64;
        }
        w!(g, p).owed = Some(0.0);
    }
    true
}

/// Stacked at an open end: does he step out round it? The signed metres along the wall to stand past his station. `w` is `pl.wall`, read and written there.   (player.ts stackLean)
pub fn stack_lean(g: &mut crate::game::Game, dt: f64, pl: PlayerId, dir: f64) -> f64 {
    let p = pl;
    let w0 = g.pl[p].wall.unwrap();
    if w0.lean.is_none() { return 0.0; }
    let ws = g.k.wallsnap;
    let (t, n) = (w0.along, w0.n);
    if crate::samstun::sam_rooted(g, None) {
        let l = un(w0.lean);
        return un(w0.lean_dir) * (ws.lean_along + w0.lean_more.unwrap_or(0.0)) * l * l * (3.0 - 2.0 * l);
    }
    let mut want = false;
    let edge = w0.at_edge;
    let rifle = g.pl[p].rifle;
    if w0.kind == Some(WallHoldKind::Stack)
        && w0.settle >= 1.0
        && edge != 0.0
        && dir != -edge
        && !crate::weapons::firearm_reloading(g, rifle)
        && (yes(w0.lean_want) || m::abs(w0.v) < 0.05)
    {
        let kept_out = yes(w0.lean_want);
        let mut ap = g.aim_point;
        let u_end = if edge > 0.0 { w0.hi } else { w0.lo };
        if ws.lean_over && ap[1] > ws.lean_over_y && g.aim_guard.is_none() {
            let cam_pos = g.tab.cmd_seat[g.seat].cam.pos;
            let dy = ap[1] - cam_pos[1];
            if dy < -0.5 {
                let k = (LEAN_CHEST - ap[1]) / dy;
                ap = [ap[0] + (ap[0] - cam_pos[0]) * k, LEAN_CHEST, ap[2] + (ap[2] - cam_pos[2]) * k];
            }
        }
        let a_ = crate::wallsnap::ws_corner_angle((ap[0] * t[0] + ap[2] * t[2] - u_end) * edge, ap[0] * n[0] + ap[2] * n[2] - w0.d);
        want = a_.dist >= ws.lean_min_dist - (if kept_out { ws.lean_keep_dist } else { 0.0 })
            && a_.deg < ws.lean_max_deg + (if kept_out { ws.lean_keep_deg } else { 0.0 })
            && a_.deg > crate::wallsnap::ws_seen_deg(BODY_R, BODY_R + WALL_GAP) + (if kept_out { ws.lean_off_deg } else { ws.lean_on_deg });
        // …and only where stepping out buys a line
        if want && !kept_out && ws.lean_clear_m > 0.0 {
            let c = g.pl[p].ch;
            let h = if g.pl[p].crouch { 0.85 } else { 1.3 };
            let k = edge * ws.lean_along - w0.lean_off.unwrap_or(0.0);
            let pos = g.ch[c].pos;
            let a: V3 = [pos[0] + t[0] * k, h, pos[2] + t[2] * k];
            let d = m::hyp2(ap[0] - a[0], ap[2] - a[2]);
            let r = m::min(1.0, ws.lean_clear_m / m::max(d, 1e-3));
            if d < 0.2 || crate::scene_collision::shot_blocked(g, a, [a[0] + (ap[0] - a[0]) * r, h + (ap[1] - h) * r, a[2] + (ap[2] - a[2]) * r], None, None) {
                want = false;
            }
        }
        // the gun's share: the least further step from which the rifle and the start of its round's way are clear
        if want && ws.lean_gun > 0.0 {
            let c = g.pl[p].ch;
            let k0 = edge * ws.lean_along - w0.lean_off.unwrap_or(0.0);
            let mut need = 0.0;
            if w0.lean_more.unwrap_or(0.0) < ws.lean_gun {
                let mut s = 0.0;
                while s <= ws.lean_gun + 1e-6 {
                    let pos = g.ch[c].pos;
                    if crate::gunwall::gun_clear_from(g, c, pos[0] + t[0] * (k0 + edge * s), pos[2] + t[2] * (k0 + edge * s), ap, ws.lean_clear_m) {
                        need = s;
                        break;
                    }
                    s += m::max(0.02, ws.lean_gun_step);
                }
            }
            if need > w0.lean_more.unwrap_or(0.0) {
                w!(g, p).lean_more = Some(m::min(need, w0.lean_more.unwrap_or(0.0) + dt * ws.lean_along / ws.lean_out));
            }
        }
    }
    let w1 = g.pl[p].wall.unwrap();
    if Some(want) != w1.lean_want {
        let dwell = un(w1.lean_dwell) + dt;
        w!(g, p).lean_dwell = Some(dwell);
        let kept = !want
            && (un(w1.lean_out_t) < ws.min_out || g.time - g.pl[p].last_fire_t < ws.after_shot)
            && w1.kind == Some(WallHoldKind::Stack)
            && !crate::weapons::firearm_reloading(g, rifle)
            && dir != -edge;
        if dwell >= (if want { ws.dwell_out } else { ws.dwell_back }) && !kept {
            let w = w!(g, p);
            w.lean_want = Some(want);
            w.lean_dwell = Some(0.0);
            if want {
                w.lean_dir = Some(edge);
                w.side = edge;
            }
        }
    } else {
        w!(g, p).lean_dwell = Some(0.0);
    }
    let w = w!(g, p);
    let out = yes(w.lean_want);
    w.lean_out_t = Some(if out { un(w.lean_out_t) + dt } else { 0.0 });
    let l = v3::clamp(un(w.lean) + (if out { dt / ws.lean_out } else { -dt / ws.lean_back }), 0.0, 1.0);
    w.lean = Some(l);
    if w.lean_more.is_some() && l <= 0.0 && !out {
        w.lean_more = None;
    }
    un(w.lean_dir) * (ws.lean_along + w.lean_more.unwrap_or(0.0)) * l * l * (3.0 - 2.0 * l)
}

/// One frame of the hold's pose: the take and the landing to the animator; on the stack the stance, the press and the rifle's carriage.   (player.ts wallSnapPose)
pub fn wall_snap_pose(g: &mut crate::game::Game, dt: f64, aiming: bool) {
    let p = g.player;
    let c = g.pl[p].ch;
    let ws = g.k.wallsnap;
    let w0 = g.pl[p].wall.unwrap();
    if w0.take_t.is_none() { return; }
    let stacked = w0.kind == Some(WallHoldKind::Stack);
    g.ch[c].anim.wall_take = if w0.settle < 1.0 { w0.settle } else { -1.0 };
    g.ch[c].anim.stance_in = if ws.slow { if stacked { ws.slow_in * 13.0 / 22.0 } else { ws.slow_in } } else { 0.0 };
    let landed = w0.settle >= 1.0 && w0.seated;
    if landed {
        if w0.hit_t == Some(0.0) {
            if ws.voice {
                wall_voice(g, w0, true);
            } else if ws.scuff > 0.0 {
                let pos = g.ch[c].pos;
                seam_audio_play_rate(g, "footSoft", pos, ws.scuff, 0.7);
            }
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

/// The camera on the wall: the look-at eased into the room and toward the side he looks; through a stack's step-out it holds still. `w` is `pl.wall`, read and written there.   (player.ts wallSnapCam)
pub fn wall_snap_cam(g: &mut crate::game::Game, pl: PlayerId) {
    let p = pl;
    let w0 = g.pl[p].wall.unwrap();
    if w0.take_t.is_none() || w0.settle < 1.0 { return; }
    let ws = g.k.wallsnap;
    let (n, t) = (w0.n, w0.along);
    let po = g.tab.cmd_seat[g.seat].cam.peek_offset.unwrap_or([0.0, 0.0, 0.0]);
    let out = truthy_opt(w0.lean) || yes(w0.lean_want) || truthy_opt(w0.lean_off);
    if !out || w0.cam_along.is_none() {
        w!(g, p).cam_along = Some(w0.side * ws.cam_side * (1.0 - w0.peek));
    }
    let cam_along = un(g.pl[p].wall.unwrap().cam_along);
    let along = cam_along - (if out { w0.lean_off.unwrap_or(0.0) * (1.0 - g.tab.cmd_seat[g.seat].cam.aim_lead) } else { 0.0 });
    let room = ws.cam_room * (if w0.kind == Some(WallHoldKind::Stack) { 0.5 } else { 1.0 });
    g.tab.cmd_seat[g.seat].cam.peek_offset = Some([po[0] + n[0] * room + t[0] * along, 0.0, po[2] + n[2] * room + t[2] * along]);
    w!(g, p).cam = true;
}

/// `take` of moveOnWall: read where he actually is back into the hold; shoved off it or past its end, let go (false).
fn wall_take_back(g: &mut Game, p: PlayerId, c: CharId, firm: bool, dir: f64, base: f64, u_lo: f64, u_hi: f64) -> bool {
    let w0 = g.pl[p].wall.unwrap();
    let (n, t) = (w0.n, w0.along);
    let pos = g.ch[c].pos;
    let u_real = pos[0] * t[0] + pos[2] * t[2] - w0.lean_off.unwrap_or(0.0);
    let off_real = pos[0] * n[0] + pos[2] * n[2] - base;
    if u_real < u_lo - 0.25 || u_real > u_hi + 0.25 || off_real > WALL_SHOVE_OFF {
        crate::game::game_msg(g, if w0.seated { "pushed off the wall" } else { "can't get to the wall here" });
        if w0.seated { wall_voice(g, w0, false); }
        release_wall(g);
        return false;
    }
    let w = w!(g, p);
    w.seated = true;
    if m::abs(u_real - w.u) > 0.003 {
        if firm {
            if js::truthy(dir) && (u_real - w.u) * dir < 0.0 {
                w.v = 0.0;
                w.block_dir = dir;
                w.block_t = 0.0;
            } else if (u_real - w.u) * w.v < 0.0 {
                w.v = 0.0;
            }
        }
        w.u = v3::clamp(u_real, u_lo, u_hi);
    }
    w.off = m::max(0.0, off_real);
    true
}

/// `ease` of moveOnWall: the take eases out on its own clock; a hold taken with the switch off is a smoothstep.
fn wall_ease(s: f64, take_t: Option<f64>, ease_pow: f64) -> f64 {
    if s < 1.0 {
        if take_t.is_some() { crate::wallsnap::ws_ease(s, ease_pow) } else { s * s * (3.0 - 2.0 * s) }
    } else {
        1.0
    }
}

/// One frame on the wall, in place of free movement; false = he has just come off it and the caller moves him as a free man this same frame.   (player.ts moveOnWall)
pub fn move_on_wall(g: &mut crate::game::Game, dt: f64, mx: f64, mz: f64, ml: f64) -> bool {
    let p = g.player;
    let c = g.pl[p].ch;
    let ws = g.k.wallsnap;
    let w0 = g.pl[p].wall.unwrap();
    let (n, t) = (w0.n, w0.along);
    let y1 = if g.pl[p].crouch { 1.0 } else { 1.7 };
    let u_lo = w0.lo + BODY_R;
    let u_hi = w0.hi - BODY_R;
    let base = w0.d + BODY_R + WALL_GAP;
    if w0.kind == Some(WallHoldKind::Stack) && (!crate::game::player_rifle_up(g, p) || !ws.on) {
        let drawn = !g.pl[p].holstered && g.pl[p].dragging.is_none();
        let w = w!(g, p);
        w.kind = None;
        w.up = Some(0.0);
        w.drawn = drawn;
    }
    let w1 = g.pl[p].wall.unwrap();
    if w1.kind != Some(WallHoldKind::Stack) && wall_stacks(g, w1) && crate::game::player_rifle_up(g, p) {
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
    if !g.pl[p].holstered && g.pl[p].wall.unwrap().kind != Some(WallHoldKind::Stack) {
        w!(g, p).drawn = true;
        set_holstered(g, true, Some(true));
    }
    // the stick: along the face (± his right) or square to it (+ away)
    let mut dir = 0.0;
    let mut away = 0.0;
    if ml > 0.0 {
        let a = mx * t[0] + mz * t[2];
        away = mx * n[0] + mz * n[2];
        let wide = ws.on && ws.along_deg > 0.0;
        let along = if wide { m::abs(a) >= m::cos(ws.along_deg * v3::DEG) } else { m::abs(a) > 0.2 && m::abs(a) >= m::abs(away) - 0.05 };
        if along { dir = if a > 0.0 { 1.0 } else { -1.0 }; }
        let w = g.pl[p].wall.unwrap();
        if wide && js::truthy(dir) && dir != w.block_dir && (if dir > 0.0 { w.u < u_hi - 0.02 } else { w.u > u_lo + 0.02 }) {
            away = 0.0;
        }
    }
    {
        let w = w!(g, p);
        w.away_t = if away > 0.5 { w.away_t + dt } else { 0.0 };
    }
    let away_limit = if ws.on { if away > 0.82 { ws.away_secs } else { ws.away_diag } } else { WALL_AWAY_SECS };
    if g.pl[p].wall.unwrap().away_t > away_limit {
        wall_step_off(g, mx, mz, ml, WallStepOffHow::Away);
        return false;
    }
    if js::truthy(dir) { w!(g, p).side = dir; }
    if g.pl[p].wall.unwrap().settle >= 1.0 && !wall_take_back(g, p, c, false, dir, base, u_lo, u_hi) { return false; }
    {
        let w = w!(g, p);
        if js::truthy(w.block_dir) && (dir != w.block_dir || { w.block_t += dt; w.block_t > 0.5 }) {
            w.block_dir = 0.0;
            w.block_t = 0.0;
        }
    }
    g.pl[p].sprinting = false;
    let crouch = g.pl[p].crouch;
    w!(g, p).crouched = crouch;
    let block_dir = g.pl[p].wall.unwrap().block_dir;
    let want = if dir == block_dir {
        0.0
    } else {
        dir * (if crouch { g.tune.player_crouch } else { WALL_PACE }) * g.puppet.as_ref().and_then(|s| s.walk).unwrap_or(1.0)
    };
    {
        let w = w!(g, p);
        w.v = v3::damp(w.v, want, if m::abs(want) > m::abs(w.v) && want * w.v >= 0.0 { 6.0 } else { 12.0 }, dt);
        if !js::truthy(want) && m::abs(w.v) < 0.02 { w.v = 0.0; }
    }
    // his station along the face, the whole body kept on it
    let mut u;
    {
        let w = w!(g, p);
        u = w.u + w.v * dt;
        if u >= u_hi {
            u = u_hi;
            if w.v > 0.0 { w.v = 0.0; }
        } else if u <= u_lo {
            u = u_lo;
            if w.v < 0.0 { w.v = 0.0; }
        }
    }
    let w2 = g.pl[p].wall.unwrap();
    let e_prev = wall_ease(w2.settle, w2.take_t, ws.ease_pow);
    let settle_secs = match w2.take_t { Some(s) => s, None => WALL_SETTLE };
    let settle = m::min(1.0, w2.settle + dt / settle_secs);
    w!(g, p).settle = settle;
    if w2.off > 0.0 && settle >= 1.0 {
        let off_want = m::max(0.0, w2.off - 0.8 * dt);
        let mut q: V3 = [t[0] * w2.u + n[0] * (base + off_want), 0.0, t[2] * w2.u + n[2] * (base + off_want)];
        crate::scene_collision::collide_circle(g, &mut q, BODY_R, 0.2, y1, Some(4.0));
        if m::abs(q[0] * t[0] + q[2] * t[2] - w2.u) < 1e-3 && q[0] * n[0] + q[2] * n[2] - (base + off_want) < 0.5 * (w2.off - off_want) {
            w!(g, p).off = off_want;
        }
    }
    let mut stand = base + g.pl[p].wall.unwrap().off;
    if settle >= 1.0 && m::abs(u - w2.u) > 1e-6 {
        let mut q: V3 = [t[0] * u + n[0] * stand, 0.0, t[2] * u + n[2] * stand];
        crate::scene_collision::collide_circle(g, &mut q, BODY_R, 0.2, y1, Some(4.0));
        let adv = u - w2.u;
        let kept = q[0] * t[0] + q[2] * t[2] - w2.u;
        let out = q[0] * n[0] + q[2] * n[2] - stand;
        let w = w!(g, p);
        if kept / adv < 0.3 || w.off + out > 0.12 {
            u = w.u;
            w.v = 0.0;
            if js::truthy(dir) {
                w.block_dir = dir;
                w.block_t = 0.0;
            }
        } else {
            u = v3::clamp(w.u + kept, u_lo, u_hi);
            if out > 0.0 {
                w.off += out;
                stand = base + w.off;
            }
        }
    }
    let u_was = g.pl[p].wall.unwrap().u;
    w!(g, p).u = u;
    // onto the plane at that station, then his own collision and the guards have their say
    let lo = if ws.on { stack_lean(g, dt, p, dir) } else { 0.0 };
    let tx = t[0] * (u + lo) + n[0] * stand;
    let tz = t[2] * (u + lo) + n[2] * stand;
    if e_prev < 1.0 {
        if ws.on && ws.slow && u != u_was {
            let pos = g.ch[c].pos;
            g.ch[c].pos = [pos[0] + t[0] * (u - u_was), pos[1], pos[2] + t[2] * (u - u_was)];
        }
        let w3 = g.pl[p].wall.unwrap();
        let e0 = wall_ease(w3.settle, w3.take_t, ws.ease_pow);
        let k = if e0 < 1.0 { (e0 - e_prev) / (1.0 - e_prev) } else { 1.0 };
        let pos = g.ch[c].pos;
        let mut sx = (tx - pos[0]) * k;
        let mut sz = (tz - pos[2]) * k;
        let sl = m::hyp2(sx, sz);
        let cap = m::hyp2(tx - w3.from[0], tz - w3.from[2]) * (e0 - e_prev) + 0.02;
        if sl > cap {
            sx *= cap / sl;
            sz *= cap / sl;
        }
        g.ch[c].pos = [pos[0] + sx, pos[1], pos[2] + sz];
    } else {
        let py = g.ch[c].pos[1];
        g.ch[c].pos = [tx, py, tz];
    }
    collide_player(g, y1);
    keep_off_guards(g, dt, y1, None);
    if lo != 0.0 || truthy_opt(g.pl[p].wall.unwrap().lean_off) {
        let pos = g.ch[c].pos;
        let w = w!(g, p);
        let got = pos[0] * t[0] + pos[2] * t[2] - w.u;
        w.lean_off = Some(if lo > 0.0 { v3::clamp(got, 0.0, lo) } else if lo < 0.0 { v3::clamp(got, lo, 0.0) } else { 0.0 });
    }
    if g.pl[p].wall.unwrap().settle >= 1.0 && !wall_take_back(g, p, c, true, dir, base, u_lo, u_hi) { return false; }
    let v = g.pl[p].wall.unwrap().v;
    g.ch[c].vel = [t[0] * v, 0.0, t[2] * v];
    g.pl[p].speed_sm = m::abs(v);
    // the ends, and peeking round an open one
    let (peek_dir, e) = {
        let w = w!(g, p);
        w.at_edge = if w.u >= u_hi - 0.005 && w.open_hi { 1.0 } else if w.u <= u_lo + 0.005 && w.open_lo { -1.0 } else { 0.0 };
        let pushing_on = w.at_edge != 0.0 && dir == w.at_edge;
        if pushing_on { w.peek_dir = w.at_edge; }
        w.peek = v3::clamp(w.peek + (if pushing_on { 1.0 } else { -1.0 }) * dt / WALL_PEEK_SECS, 0.0, 1.0);
        (w.peek_dir, w.peek * w.peek * (3.0 - 2.0 * w.peek))
    };
    let offset = if e > 0.0 {
        Some([(t[0] * peek_dir * (BODY_R + 0.6) + n[0] * 0.3) * e, 0.0, (t[2] * peek_dir * (BODY_R + 0.6) + n[2] * 0.3) * e])
    } else {
        None
    };
    g.tab.cmd_seat[g.seat].cam.peek_offset = offset;
    g.ch[c].anim.look_yaw_extra = -peek_dir * WALL_PEEK_LOOK * e;
    if ws.on { wall_snap_cam(g, p); }
    true
}

/// Silent takedown: lock in on the guard, throw the blow, he drops with a thud only the room hears.   (player.ts startTakedown)
pub fn start_takedown(g: &mut crate::game::Game, gd: GuardId, way: Option<crate::close::CloseKind>, lethal: Option<bool>, loud: Option<bool>) {
    let lethal = lethal.unwrap_or(false);
    let loud = loud.unwrap_or(false);
    let p = g.player;
    if g.gd[gd].state == crate::game::GuardState::Dead || g.pl[p].down { return; }
    let c = g.pl[p].ch;
    let gc = g.gd[gd].ch;
    let kind: Option<crate::close::CloseKind> = if crate::close::close_live(g) {
        Some(match way {
            Some(way) => way,
            None => {
                let pos = g.ch[c].pos;
                let o = crate::close::close_at(g, gd, &pos[..]);
                crate::close::close_kind_of(Some(&o))
            }
        })
    } else {
        None
    };
    let side_on = kind.is_some() && kind != Some(crate::close::CloseKind::Behind);
    let from = if side_on {
        let (a, b) = (g.ch[c].pos, g.ch[gc].pos);
        v3::wrap_angle(m::atan2(a[0] - b[0], a[2] - b[2]) - g.ch[gc].body_yaw)
    } else {
        0.0
    };
    if let Some(w) = g.pl[p].wall.as_mut() { w.drawn = false; }
    release_wall(g);
    if g.pl[p].dragging.is_some() { crate::game::game_toggle_drag(g, None); }
    let mut td = crate::game::PlayerTakedown { g: gd, t: 0.0, kind: None, from: None, alert: None, yaw: None, at: None, lethal: None, loud: None };
    if let (true, Some(k)) = (side_on, kind) {
        td.kind = Some(k);
        td.from = Some(from);
        td.alert = Some(g.gd[gd].state == crate::game::GuardState::Alert);
        td.at = Some(crate::close::close_station(g, gd, k, from).pos);
    }
    g.pl[p].takedown = Some(td);
    g.pl[p].sprinting = false;
    if loud { g.pl[p].takedown.as_mut().unwrap().loud = Some(true); }
    let blow = if kind == Some(crate::close::CloseKind::Front) { crate::character::StrikeKind::Jab } else { crate::character::StrikeKind::Cross };
    if lethal {
        g.pl[p].takedown.as_mut().unwrap().lethal = Some(true);
        crate::character::anim_strike(g, c, Some(blow), Some(true));
    } else {
        crate::character::anim_strike(g, c, Some(blow), None);
    }
    crate::takedown::take_begin(g, c, gd, kind, lethal);
    cancel_reloads(g, p);
    if let Some(k) = kind {
        let (a, b) = (g.ch[c].pos, g.ch[gc].pos);
        crate::close::close_took(g, gd, crate::close::CloseTookWhat::Take, k, from, m::hyp2(a[0] - b[0], a[2] - b[2]));
    }
    throw_off(g, p);
    g.pl[p].throw_held = false;
    g.pl[p].throw_preview = None;
    g.pl[p].pending_throw = None;
    g.gd[gd].speed = 0.0;
    g.ch[gc].vel = [0.0, 0.0, 0.0];
    lose_path(g, gd);
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, 1.0);
    let at = v3::add(g.ch[gc].pos, [0.0, 1.3, 0.0]);
    seam_audio_play_rate(g, "bodyHit", at, if loud { 0.8 } else { 0.55 }, 0.8);
}

/// One frame of the takedown: hold station on the man, land the blow at its beat, give control back as it recovers.   (player.ts updateTakedown)
pub fn update_takedown(g: &mut crate::game::Game, dt: f64) {
    let p = g.player;
    let Some(td) = g.pl[p].takedown else { return; };
    let gd = td.g;
    let c = g.pl[p].ch;
    if !crate::game::game_on_floor(g, gd) {
        g.pl[p].takedown = None;
        return;
    }
    let gc = g.gd[gd].ch;
    let mut yaw = g.ch[gc].body_yaw;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let y1 = if g.pl[p].crouch { 1.0 } else { 1.7 };
    if let Some(kind) = td.kind {
        let at = match td.at {
            Some(at) => at,
            None => crate::close::close_station(g, gd, kind, td.from.unwrap_or(0.0)).pos,
        };
        let mut td_yaw = td.yaw;
        if g.gd[gd].state != crate::game::GuardState::Dead {
            g.ch[c].pos = v3::lerp3(g.ch[c].pos, at, 1.0 - m::exp(-14.0 * dt));
            collide_player(g, y1);
            let (a, b) = (g.ch[gc].pos, g.ch[c].pos);
            let y = m::atan2(a[0] - b[0], a[2] - b[2]);
            td_yaw = Some(y);
            if let Some(t) = g.pl[p].takedown.as_mut() { t.yaw = Some(y); }
        }
        yaw = td_yaw.unwrap_or(g.ch[c].body_yaw);
    } else if g.gd[gd].state != crate::game::GuardState::Dead {
        let b = g.ch[gc].pos;
        let want: V3 = [b[0] - m::sin(yaw) * 0.55, 0.0, b[2] - m::cos(yaw) * 0.55];
        g.ch[c].pos = v3::lerp3(g.ch[c].pos, want, 1.0 - m::exp(-14.0 * dt));
        collide_player(g, y1);
    }
    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, 14.0 * dt);
    g.ch[c].aim_yaw = g.ch[c].body_yaw;
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.1, 8.0, dt);
    let t = td.t + dt;
    if let Some(row) = g.pl[p].takedown.as_mut() { row.t = t; }
    if t >= 0.28 && g.gd[gd].state != crate::game::GuardState::Dead {
        let was_alert = g.gd[gd].state == crate::game::GuardState::Alert;
        let noise = if let Some(kind) = td.kind {
            crate::close::close_noise(g, kind, yes(td.alert))
        } else if crate::close::close_live(g) {
            crate::close::close_noise(g, crate::close::CloseKind::Behind, was_alert)
        } else {
            0.3
        };
        let level = noise + (if yes(td.loud) { g.k.close.rmb_loud } else { 0.0 });
        if td.kind.is_some() && crate::close::close_seated(g, gd) { crate::close::close_unseat(g, gd, None); }
        crate::takedown::take_fall(g, gd, td.kind, yes(td.lethal), None);
        let way: V3 = [m::sin(yaw), 0.0, m::cos(yaw)];
        if yes(td.lethal) {
            crate::combat::kill_guard(g, gd, way, Some(true), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Player), Some("knife — dead"), None);
            let now = g.time;
            let victim = g.gd[gd].callsign.clone();
            for i in 0..g.guards.len() {
                let o = g.guards[i];
                if let Some(w) = g.gd[o].witness.saw_act.as_mut() {
                    if w.kind == crate::game::WitnessSawActKind::Shot && w.t == now && w.victim == victim {
                        w.kind = crate::game::WitnessSawActKind::Struck;
                    }
                }
            }
        } else {
            crate::combat::kill_guard(g, gd, way, Some(true), None, None, None, None);
        }
        crate::close::close_blow(g, yes(td.lethal));
        let ev = crate::game::GameEvent {
            kind: crate::director::EventKind::Prop,
            pos: v3::copy(g.ch[gc].pos),
            time: g.time,
            loud: false,
            level: Some(level),
            who: Some(g.ch[c].id),
            serial: g.serial(),
            ..crate::game::GameEvent::zeroed()
        };
        g.events.push(ev);
        g.pl[p].noise = m::max(g.pl[p].noise, level);
    }
    if t > 0.62 { g.pl[p].takedown = None; }
}

/// Which hold a grab made now would be to begin with.   (player.ts grabVariant)
pub fn grab_variant(g: &mut crate::game::Game) -> crate::character::HoldVariant {
    let p = g.player;
    if crate::game::player_pistol_up(g, p) { crate::character::HoldVariant::Gun } else { crate::character::HoldVariant::Arm }
}

/// The 'grab him' row: drop whatever the hands had; he is frozen for the beat updateHold's 'grab' phase closes in. From here he is `held`.   (player.ts startGrab)
pub fn start_grab(g: &mut crate::game::Game, gd: GuardId, way: Option<crate::close::CloseKind>) {
    let p = g.player;
    let c = g.pl[p].ch;
    let gc = g.gd[gd].ch;
    let kind: Option<crate::close::CloseKind> = if crate::close::close_live(g) {
        Some(match way {
            Some(way) => way,
            None => {
                let pos = g.ch[c].pos;
                let o = crate::close::close_at(g, gd, &pos[..]);
                crate::close::close_kind_of(Some(&o))
            }
        })
    } else {
        None
    };
    let (a, b) = (g.ch[c].pos, g.ch[gc].pos);
    let b0 = if kind.is_some() { v3::wrap_angle(m::atan2(a[0] - b[0], a[2] - b[2]) - g.ch[gc].body_yaw) } else { 0.0 };
    let r0 = m::hyp2(a[0] - b[0], a[2] - b[2]);
    {
        let (st, pl) = (g.gd[gd].state, &g.pl[p]);
        if st == crate::game::GuardState::Dead || st == crate::game::GuardState::Alert || g.gd[gd].held.is_some() || pl.down || pl.holding.is_some() || pl.takedown.is_some() || pl.kick.is_some() || pl.picking.is_some() || pl.wanding.is_some() {
            return;
        }
    }
    if g.pl[p].crouch && !can_stand(g) {
        crate::game::game_msg(g, "no room to stand up behind him here");
        return;
    }
    if crate::game::player_rifle_up(g, p) { set_holstered(g, true, Some(true)); }
    let variant = grab_variant(g);
    if let Some(w) = g.pl[p].wall.as_mut() { w.drawn = false; }
    release_wall(g);
    if g.pl[p].dragging.is_some() { crate::game::game_toggle_drag(g, None); }
    g.pl[p].sprinting = false;
    g.pl[p].crouch = false;
    g.pl[p].speed_sm = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.pl[p].backpedal = false;
    g.ch[c].anim.reverse = false;
    cancel_reloads(g, p);
    throw_off(g, p);
    g.pl[p].throw_held = false;
    g.pl[p].throw_preview = None;
    g.pl[p].easing = None;
    // him: whatever he was saying is cut off
    if let Some(bub) = g.gd[gd].bubble.clone() {
        let now = g.time;
        if now - bub.t < bub.dur {
            if now - bub.t < 0.8 {
                let line = if g.gd[gd].unit.is_some() { alloc::format!("{}: {}", g.gd[gd].callsign, bub.text) } else { alloc::format!("guard: {}", bub.text) };
                crate::game::game_msg(g, &line);
            }
            g.gd[gd].bubble = None;
        }
    }
    if let Some((stage, ca, cb)) = g.clearing.as_ref().map(|cl| (cl.stage, cl.a, cl.b)) {
        if stage != crate::squad::ClearStage::Done && (ca == gd || cb == Some(gd)) {
            let other = if ca == gd { cb } else { Some(ca) };
            crate::squad::clearing_cancel(g);
            g.clearing = None;
            if let Some(o) = other {
                if g.gd[o].state != crate::game::GuardState::Dead && g.gd[o].held.is_none() {
                    let seen_at = v3::copy(g.ch[gc].pos);
                    lose_path(g, o);
                    let m_ = &mut g.gd[o];
                    m_.state = crate::game::GuardState::Suspicious;
                    m_.awareness = m::max(m_.awareness, 0.6);
                    m_.last_known = Some(seen_at);
                    m_.fix_stamp = m_.fix_stamp.wrapping_add(1);
                    m_.react_t = 0.7;
                    m_.search_t = 0.0;
                    m_.search_plan = None;
                    m_.pieing = None;
                }
            }
        }
    }
    if g.gd[gd].script.is_some() {
        let st = match g.gd[gd].state {
            crate::game::GuardState::Patrol => crate::squad::EndGuardScriptState::Patrol,
            crate::game::GuardState::Search => crate::squad::EndGuardScriptState::Search,
            _ => crate::squad::EndGuardScriptState::Suspicious,
        };
        crate::squad::end_guard_script(g, gd, Some(st));
    }
    lose_path(g, gd);
    {
        let m_ = &mut g.gd[gd];
        m_.speed = 0.0;
        m_.body_duty = None;
        m_.pieing = None;
        m_.search_plan = None;
        m_.muzzle_down = false;
        m_.stall_ref = None;
        m_.sweep = 0.0;
    }
    g.ch[gc].vel = [0.0, 0.0, 0.0];
    g.ch[gc].anim.look_yaw_extra = 0.0;
    let flash = g.gd[gd].flashlight;
    g.gd[gd].light_lit_at_loss = seam_light_enabled(g, flash);
    if crate::guards::in_hand(g, gd) == Some(crate::guards::InHand::Torch) {
        let beam = g.gd[gd].beam_dir;
        let item = crate::combat::drop_from_hand(g, gc, crate::combat::DropFromHandKind::Torch, beam, [0.0, 0.0, 0.0], Some(false), Some(true), None);
        g.gd[gd].dropped = Some(item);
        g.gd[gd].torch_lost_at = g.time;
    }
    if kind.is_some() && g.gd[gd].civ.is_none() && crate::close::close_seated(g, gd) { crate::close::close_unseat(g, gd, None); }
    g.gd[gd].held = Some(crate::game::GuardHeld {
        by: crate::game::GuardHeldBy::Player,
        since: g.time,
        variant,
        phase: crate::character::HoldPhase::Grab,
        reverse: false,
    });
    if g.gd[gd].civ.is_some() { crate::civilians::civilian_grabbed(g, gd); }
    if g.ai_enabled { g.gd[gd].witness.was_held += 1.0; }
    let talk = g.gd[gd].talk;
    g.tab.game_talk[talk].held_since = g.time;
    let serial = g.serial();
    g.pl[p].holding = Some(Hold {
        g: gd,
        variant,
        phase: crate::character::HoldPhase::Grab,
        t: 0.0,
        since: g.time,
        wind_t: -1.0,
        swap_t: f64::INFINITY,
        remarked: false,
        gun_out: 0.0,
        fire_queued: false,
        shot_t: -1.0,
        ask: None,
        after: None,
        serial,
        ..Hold::zeroed()
    });
    crate::disarm::disarm_begin(g);
    if let Some(k) = kind { crate::close::close_took(g, gd, crate::close::CloseTookWhat::Grab, k, b0, r0); }
    g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, g.k.hold.secs.grab);
    let chest = v3::add(g.ch[gc].pos, [0.0, 1.4, 0.0]);
    crate::disarm::disarm_catch(g, chest);
    let ev = crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: v3::copy(g.ch[gc].pos),
        time: g.time,
        loud: false,
        level: Some(GRAB_NOISE),
        who: Some(g.ch[c].id),
        serial: g.serial(),
        ..crate::game::GameEvent::zeroed()
    };
    g.events.push(ev);
    g.pl[p].noise = m::max(g.pl[p].noise, GRAB_NOISE);
    let _ = crate::chronicle::chron_grab(g, gd);
}

/// N: the goggles' switch, and the tube's picture easing after them.   (player.ts nvKey)
pub fn nv_key(g: &mut crate::game::Game, input: Option<InputId>, dt: f64) {
    let p = g.player;
    if g.pl[p].nv && !crate::kit::kit_you_nvg(g, p) { g.pl[p].nv = false; }
    let (rifle, pistol) = (g.pl[p].rifle, g.pl[p].pistol);
    if !crate::kit::kit_you_light(g, p) && (g.fa[rifle].light_on || g.fa[pistol].light_on) {
        g.fa[rifle].light_on = false;
        g.fa[pistol].light_on = false;
    }
    if input.map_or(false, |i| crate::input::input_hit(g, i, "KeyN")) && crate::kit::kit_you_nvg(g, p) {
        let on = !g.pl[p].nv;
        g.pl[p].nv = on;
        crate::audio_iface::game_audio_like_play(g, if on { "nvOn" } else { "nvOff" }, None, Some(0.7), None);
        crate::game::game_msg(g, if on { "night vision on" } else { "night vision off" });
        crate::kitanim::kit_nv(g);
    }
    let eye = crate::kitanim::kit_nv_eye(g);
    g.pl[p].nv_amount = v3::damp(g.pl[p].nv_amount, if eye { 1.0 } else { 0.0 }, if eye { 5.0 } else { 9.0 }, dt);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// stackUp's `(w.owed ?? 0) | (input.clicked & 5)`: only the fire buttons' bits (1 and 4) are kept, what was owed stays, absent is 0.
    #[test]
    fn owed_keeps_the_fire_bits() {
        assert_eq!(owed_with(None, 0.0), 0.0);
        assert_eq!(owed_with(None, 7.0), 5.0);   // 2 (the middle button) is not owed a round
        assert_eq!(owed_with(Some(4.0), 1.0), 5.0);
        assert_eq!(owed_with(Some(2.0), 8.0), 2.0);
    }

    /// a `w.x!` absent reads NaN as JavaScript's undefined does in a sum or a comparison, never a panic
    #[test]
    fn an_absent_number_is_nan_and_falsy() {
        assert!(!(un(None) > 0.0) && !(un(None) < 1.0));
        assert!(!truthy_opt(None) && !truthy_opt(Some(0.0)) && truthy_opt(Some(0.5)));
    }
}
