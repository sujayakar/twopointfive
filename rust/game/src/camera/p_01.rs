//! src/game/camera.ts lines 1 to 154
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Pin the framing, or with None release it and hand back the free camera's angles and zoom. (camera.ts FollowCamera.setPin)
pub fn follow_camera_set_pin(g: &mut Game, id: CameraId, p: Option<FollowCameraSetPinP>) -> Option<FollowCameraPin> {
    set_pin(&mut g.tab.camera[id], p)
}
fn set_pin(c: &mut FollowCamera, p: Option<FollowCameraSetPinP>) -> Option<FollowCameraPin> {
    let p = match p {
        Some(p) => p,
        None => {
            if let (Some(_), Some(b)) = (c.pin, c.before_pin) {
                c.yaw = b.yaw;
                c.pitch = b.pitch;
                c.distance = b.distance;
                c.desired_distance = b.desired_distance;
            }
            c.pin = None;
            c.before_pin = None;
            return None;
        }
    };
    if ![p.look[0], p.look[1], p.look[2], p.yaw, p.pitch, p.dist].iter().all(|x| x.is_finite()) {
        panic!("camera pin: look[3], yaw, pitch and dist must all be finite numbers");
    }
    if c.pin.is_none() {
        c.before_pin = Some(FollowCameraBeforePin { yaw: c.yaw, pitch: c.pitch, distance: c.distance, desired_distance: c.desired_distance });
    }
    c.pin = Some(FollowCameraPin {
        look: v3::copy(p.look),
        yaw: p.yaw,
        pitch: v3::clamp(p.pitch, -PIN_PITCH_MAX, PIN_PITCH_MAX),
        dist: m::max(c.near + 0.05, p.dist),
    });
    c.pin
}

/// A drawn frame and a tick's follow, together. (camera.ts FollowCamera.update)
pub fn follow_camera_update(g: &mut Game, id: CameraId, dt: f64, follow: V3, aim_point: Option<V3>) {
    update(&mut g.tab.camera[id], dt, follow, aim_point)
}
fn update(c: &mut FollowCamera, dt: f64, follow: V3, aim_point: Option<V3>) {
    frame(c);
    step(c, dt, follow, aim_point);
}

/// Once a game tick: the follow itself, or the pin verbatim. (camera.ts FollowCamera.step)
pub fn follow_camera_step(g: &mut Game, id: CameraId, dt: f64, follow: V3, aim_point: Option<V3>) {
    step(&mut g.tab.camera[id], dt, follow, aim_point)
}
fn step(c: &mut FollowCamera, dt: f64, follow: V3, aim_point: Option<V3>) {
    if let Some(pin) = c.pin {
        c.desired = v3::copy(pin.look);
        c.desired_serial = c.desired_serial.wrapping_add(1);
        c.target = v3::copy(pin.look);
        c.yaw = pin.yaw;
        c.pitch = pin.pitch;
        c.desired_distance = pin.dist;
        c.distance = pin.dist;
        rebuild(c);
        return;
    }
    let mut d = v3::copy(follow);
    if let Some(a) = aim_point {
        d = v3::lerp3(follow, a, c.aim_lead);
    }
    if let Some(o) = c.peek_offset {
        d = v3::add(d, o);
    }
    c.desired = d;
    c.desired_serial = c.desired_serial.wrapping_add(1);
    c.target = [v3::damp(c.target[0], d[0], 4.0, dt), v3::damp(c.target[1], d[1], 4.0, dt), v3::damp(c.target[2], d[2], 4.0, dt)];
    c.distance = v3::damp(c.distance, c.desired_distance, 6.0, dt);
    rebuild(c);
}

/// Once a drawn frame: 'previous' becomes the matrix the last frame was drawn with. (camera.ts FollowCamera.frame)
pub fn follow_camera_frame(g: &mut Game, id: CameraId) {
    frame(&mut g.tab.camera[id])
}
fn frame(c: &mut FollowCamera) {
    match c.kick_drawn.take() {
        Some(d) => c.prev_view_proj.copy_from_slice(&d),
        None => c.prev_view_proj = v3::m4::copy(&c.view_proj),
    }
}

/// The look-at without the kick or the pose. (camera.ts FollowCamera.focus)
pub fn follow_camera_focus(g: &mut Game, id: CameraId) -> V3 {
    focus(&g.tab.camera[id])
}
fn focus(c: &FollowCamera) -> V3 {
    match (&c.draw_held, c.kick_held) {
        (Some(h), _) => h.target,
        (None, Some(t)) => t,
        (None, None) => c.target,
    }
}

/// The blast's kick, in for the render. (camera.ts FollowCamera.kickIn)
pub fn follow_camera_kick_in(g: &mut Game, id: CameraId) {
    kick_in(&mut g.tab.camera[id])
}
fn kick_in(c: &mut FollowCamera) {
    let k = match c.kick {
        Some(k) if c.kick_held.is_none() => k,
        _ => return,
    };
    c.kick_held = Some(c.target);
    c.target = v3::add(c.target, k);
    rebuild(c);
}

/// The blast's kick, out again, keeping the matrix the frame was drawn with. (camera.ts FollowCamera.kickOut)
pub fn follow_camera_kick_out(g: &mut Game, id: CameraId) {
    kick_out(&mut g.tab.camera[id])
}
fn kick_out(c: &mut FollowCamera) {
    let t = match c.kick_held {
        Some(t) => t,
        None => return,
    };
    c.kick_vp = v3::m4::copy(&c.view_proj);
    c.kick_drawn = Some(c.kick_vp.to_vec());
    c.kick_held = None;
    c.target = t;
    rebuild(c);
}

/// The picture's pose in for the frame's render; the tick's is kept. (camera.ts FollowCamera.drawIn)
pub fn follow_camera_draw_in(g: &mut Game, id: CameraId, p: FollowCameraDrawInP) {
    draw_in(&mut g.tab.camera[id], p)
}
fn draw_in(c: &mut FollowCamera, p: FollowCameraDrawInP) {
    if c.draw_held.is_some() {
        return;
    }
    c.draw_held = Some(FollowCameraDrawHeld { target: c.target, yaw: c.yaw, pitch: c.pitch, distance: c.distance });
    c.target = [p.target[0], p.target[1], p.target[2]];
    c.yaw = p.yaw;
    c.pitch = p.pitch;
    c.distance = p.distance;
    rebuild(c);
}

/// The tick's pose put back, keeping the matrix the frame was drawn with. (camera.ts FollowCamera.drawOut)
pub fn follow_camera_draw_out(g: &mut Game, id: CameraId) {
    draw_out(&mut g.tab.camera[id])
}
fn draw_out(c: &mut FollowCamera) {
    let h = match c.draw_held {
        Some(h) => h,
        None => return,
    };
    if c.kick_drawn.is_none() {
        c.draw_vp = v3::m4::copy(&c.view_proj);
        c.kick_drawn = Some(c.draw_vp.to_vec());
    }
    c.draw_held = None;
    c.target = h.target;
    c.yaw = h.yaw;
    c.pitch = h.pitch;
    c.distance = h.distance;
    rebuild(c);
}

/// The matrices from look-at, angles and distance; the tick calls it too (game.ts:1665, journal.ts:879). (camera.ts FollowCamera.rebuild)
pub fn follow_camera_rebuild(g: &mut Game, id: CameraId) {
    rebuild(&mut g.tab.camera[id])
}
fn rebuild(c: &mut FollowCamera) {
    let (cp, sp, cy, sy) = (m::cos(c.pitch), m::sin(c.pitch), m::cos(c.yaw), m::sin(c.yaw));
    c.forward = [-sy * cp, -sp, -cy * cp];
    c.pos = v3::mad(c.target, c.forward, -c.distance);
    c.view = v3::m4::look_at(c.pos, c.target, [0.0, 1.0, 0.0]);
    c.proj = v3::m4::perspective(c.fov, c.aspect, c.near, c.far);
    c.view_proj = v3::m4::mul(&c.proj, &c.view);
    // (a singular matrix leaves the old inverse, as the TypeScript's `out` does)
    if let Some(i) = v3::m4::invert(&c.view_proj) {
        c.inv_view_proj = i;
    }
}

/// Spectating: every half second, pick the yaw the followed man can be seen from. (camera.ts FollowCamera.autoYaw)
pub fn follow_camera_auto_yaw(g: &mut Game, id: CameraId, dt: f64, target: V3, blocked: &mut dyn FnMut(&mut Game, V3, V3) -> bool) {
    // (`blocked` needs g, so the camera is let go across the search; the level's segment test writes no yaw)
    let yaw = match auto_yaw_due(&mut g.tab.camera[id], dt) {
        Some(yaw) => yaw,
        None => return,
    };
    let best = auto_yaw_pick(yaw, target, &mut |a, b| blocked(g, a, b));
    g.tab.camera[id].yaw_goal = Some(best);
}
/// The half-second timer: Some(the yaw to search from) when a search is due.
fn auto_yaw_due(c: &mut FollowCamera, dt: f64) -> Option<f64> {
    c.auto_yaw_t -= dt;
    if c.auto_yaw_t > 0.0 {
        return None;
    }
    c.auto_yaw_t = 0.5;
    Some(c.yaw)
}
/// Eight yaws, the current first, then ±45°, ±90°, ±135°, 180°; the clearest low approach wins, a hair of preference for turning less.
fn auto_yaw_pick(from: f64, target: V3, blocked: &mut dyn FnMut(V3, V3) -> bool) -> f64 {
    let chest: V3 = [target[0], 1.3, target[2]];
    let (mut best, mut best_score) = (from, -1.0);
    for i in 0..8 {
        let turns = m::ceil(i as f64 / 2.0);
        let yaw = from + if i == 0 { 0.0 } else { (if i % 2 != 0 { 1.0 } else { -1.0 }) * turns * (m::PI / 4.0) };
        let (sy, cy) = (m::sin(yaw), m::cos(yaw));
        let mut clear = 0.0;
        for s in 1..=4 {
            let s = s as f64;
            let p: V3 = [chest[0] + sy * s, 1.3, chest[2] + cy * s];
            if blocked(chest, p) {
                break;
            }
            clear = s;
        }
        let score = clear - m::abs(if i == 0 { 0.0 } else { turns }) * 0.01;
        if score > best_score {
            best_score = score;
            best = yaw;
        }
    }
    best
}

/// Ease toward the autoYaw goal; told to stop, forget it. (camera.ts FollowCamera.easeYaw)
pub fn follow_camera_ease_yaw(g: &mut Game, id: CameraId, dt: f64, on: bool) {
    ease_yaw(&mut g.tab.camera[id], dt, on)
}
fn ease_yaw(c: &mut FollowCamera, dt: f64, on: bool) {
    if !on {
        c.yaw_goal = None;
        return;
    }
    let goal = match c.yaw_goal {
        Some(y) => y,
        None => return,
    };
    let d = goal - c.yaw;
    let d = m::atan2(m::sin(d), m::cos(d));
    c.yaw += d * m::min(1.0, dt * 3.0);
}

/// (camera.ts FollowCamera.zoom)
pub fn follow_camera_zoom(g: &mut Game, id: CameraId, delta: f64) {
    zoom(&mut g.tab.camera[id], delta)
}
fn zoom(c: &mut FollowCamera, delta: f64) {
    c.desired_distance = v3::clamp(c.desired_distance * m::exp(delta * 0.0012), 6.0, 40.0);
}

/// (camera.ts FollowCamera.rotate)
pub fn follow_camera_rotate(g: &mut Game, id: CameraId, d_yaw: f64) {
    g.tab.camera[id].yaw += d_yaw;
}

/// World point to CSS pixels of a w×h view, and whether it is showable. (camera.ts FollowCamera.project)
pub fn follow_camera_project(g: &mut Game, id: CameraId, p: V3, w: f64, h: f64, margin: Option<f64>, min_w: Option<f64>) -> (f64, f64, bool) {
    project(&g.tab.camera[id], p, w, h, margin, min_w)
}
fn project(c: &FollowCamera, p: V3, w: f64, h: f64, margin: Option<f64>, min_w: Option<f64>) -> (f64, f64, bool) {
    let margin = margin.unwrap_or(f64::INFINITY);
    let min_w = min_w.unwrap_or(0.0);
    let vp = &c.view_proj;
    let cw = vp[3] as f64 * p[0] + vp[7] as f64 * p[1] + vp[11] as f64 * p[2] + vp[15] as f64;
    let d = js::or(cw, 1.0);
    let nx = (vp[0] as f64 * p[0] + vp[4] as f64 * p[1] + vp[8] as f64 * p[2] + vp[12] as f64) / d;
    let ny = (vp[1] as f64 * p[0] + vp[5] as f64 * p[1] + vp[9] as f64 * p[2] + vp[13] as f64) / d;
    let front = cw > min_w;
    (
        (nx * 0.5 + 0.5) * w,
        (1.0 - (ny * 0.5 + 0.5)) * h,
        if margin == f64::INFINITY { front } else { front && nx > -margin && nx < margin && ny > -margin && ny < margin },
    )
}

/// World-space ray through a pixel in [0,1]² UV, origin top-left. (camera.ts FollowCamera.ray)
pub fn follow_camera_ray(g: &mut Game, id: CameraId, u: f64, v: f64) -> FollowCameraRay {
    ray(&g.tab.camera[id], u, v)
}
fn ray(c: &FollowCamera, u: f64, v: f64) -> FollowCameraRay {
    let (ndc_x, ndc_y) = (u * 2.0 - 1.0, 1.0 - v * 2.0);
    let p0 = v3::m4::transform_point(&c.inv_view_proj, [ndc_x, ndc_y, 0.0]);
    let p1 = v3::m4::transform_point(&c.inv_view_proj, [ndc_x, ndc_y, 1.0]);
    FollowCameraRay { ro: p0, rd: v3::normalize(v3::sub(p1, p0)) }
}

/// A UV ray against the plane y = h. (camera.ts FollowCamera.groundPoint)
pub fn follow_camera_ground_point(g: &mut Game, id: CameraId, u: f64, v: f64, h: Option<f64>) -> Option<V3> {
    ground_point(&g.tab.camera[id], u, v, h)
}
fn ground_point(c: &FollowCamera, u: f64, v: f64, h: Option<f64>) -> Option<V3> {
    let h = h.unwrap_or(0.0);
    let FollowCameraRay { ro, rd } = ray(c, u, v);
    if m::abs(rd[1]) < 1e-5 {
        return None;
    }
    let t = (h - ro[1]) / rd[1];
    if t < 0.0 {
        return None;
    }
    Some(v3::mad(ro, rd, t))
}

/// Camera-relative planar basis for WASD. (camera.ts FollowCamera.planarBasis)
pub fn follow_camera_planar_basis(g: &mut Game, id: CameraId) -> FollowCameraPlanarBasis {
    planar_basis(&g.tab.camera[id])
}
fn planar_basis(c: &FollowCamera) -> FollowCameraPlanarBasis {
    let f = v3::normalize([c.forward[0], 0.0, c.forward[2]]);
    FollowCameraPlanarBasis { fwd: f, right: [-f[2], 0.0, f[0]] }
}

/// Every number below is the TypeScript's own, under EXACT.on, on the same camera.
#[cfg(test)]
mod tests {
    use super::*;
    const FORWARD: [u64; 3] = [0xbfd2b3a50d27623a, 0xbfec84c54c2d6338, 0xbfd6341794125f58];
    const POS: [u64; 3] = [0x40209c5207e94ab4, 0x4030eafaa9fbbcd8, 0xbffbf84d39308a08];
    const VIEW: [u32; 16] = [0x3f43ccb3, 0xbf12fa49, 0x3e959d28, 0x00000000, 0x00000000, 0x3ee83dc0, 0x3f64262a, 0x00000000, 0xbf24eb73, 0xbf2e7f80, 0x3eb1a0bd, 0x00000000, 0xc0ef4f40, 0xc0831ba0, 0xc1872eb5, 0x3f800000];
    const PROJ: [u32; 16] = [0x40206e66, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x40805851, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf80891b, 0xbf800000, 0x00000000, 0x00000000, 0xbf00891b, 0x00000000];
    const VP: [u32; 16] = [0x3ff568c0, 0xc0135fb2, 0xbe963d6a, 0xbe959d28, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfceb48e, 0xc02ef7e6, 0xbeb25f01, 0xbeb1a0bd, 0xc195f8bb, 0xc1837616, 0x4183bb39, 0x41872eb5];
    const IVP: [u32; 16] = [0x3e9c37f9, 0x33816418, 0xbe8394c5, 0x315f4604, 0xbe129522, 0x3de79e00, 0xbe2e076e, 0x31e23113, 0xc18454db, 0xc206c782, 0x405ed3c7, 0xbffeef02, 0x41828c25, 0x4203c748, 0xc075f68c, 0x4000000a];
    const P1: [u64; 2] = [0x40825bf1425c0160, 0x40802f31a32c167f];
    const P1_F: bool = true;
    const P2: [u64; 2] = [0x40825bf1425c0160, 0x40802f31a32c167f];
    const P2_F: bool = false;
    const P3: [u64; 2] = [0x4083ffff2369f375, 0x40767fff7c8bf64f];
    const P3_F: bool = false;
    const RO: [u64; 3] = [0x40203999388e6301, 0x403076028a23ba68, 0xbffda6aafba13b1a];
    const RD: [u64; 3] = [0xbfd857b660e2f209, 0xbfecd796ac82b0ec, 0xbfca87ab8bcd57b1];
    const GP: [u64; 3] = [0x3ff8bb9c5df74160, 0x3fecccccccccccd0, 0xc015b9ec7a7f825a];
    const GP0: [u64; 3] = [0x3ff2a7f58640d454, 0x0000000000000000, 0xc0168dda7328fa24];
    const GP_ABOVE_NONE: bool = true;
    const FWD: [u64; 3] = [0xbfe49d6e694619b9, 0x0000000000000000, 0xbfe87996529f9d92];
    const RIGHT: [u64; 3] = [0x3fe87996529f9d92, 0x0000000000000000, 0xbfe49d6e694619b9];
    const ZOOM: [u64; 1] = [0x403840a82f047a1c];
    const ZOOM_HI: [u64; 1] = [0x4044000000000000];
    const EASE: [u64; 1] = [0x3fe9eecbfb15b574];
    const KICKED: [u64; 3] = [0x400acccccccccccd, 0x3ff8000000000000, 0xc01fcccccccccccd];
    const KICK_VP: [u32; 16] = [0x3fdd2427, 0xc025c100, 0xbea8fa3a, 0xbea845fc, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfe87c2d, 0xc01daa87, 0xbea0bb7d, 0xbea0100a, 0xc1a1d145, 0xc15a3483, 0x4186949d, 0x418a050f];
    const FOCUS_IN: [u64; 3] = [0x400a000000000000, 0x3ff8000000000000, 0xc01f000000000000];
    const AFTER: [u64; 3] = [0x400a000000000000, 0x3ff8000000000000, 0xc01f000000000000];
    const AFTER_VP: [u32; 16] = [0x3fdd2427, 0xc025c100, 0xbea8fa3a, 0xbea845fc, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfe87c2d, 0xc01daa87, 0xbea0bb7d, 0xbea0100a, 0xc19d877e, 0xc1567736, 0x4186d19c, 0x418a41cd];
    const PREV: [u32; 16] = [0x3fdd2427, 0xc025c100, 0xbea8fa3a, 0xbea845fc, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfe87c2d, 0xc01daa87, 0xbea0bb7d, 0xbea0100a, 0xc1a1d145, 0xc15a3483, 0x4186949d, 0x418a050f];

    const PIN_APPLIED: [u64; 6] = [0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x3fd999999999999a, 0x3ff8da7e39bae2a3, 0x3fe199999999999a];
    const PIN_STEP: [u64; 10] = [0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x3fd999999999999a, 0xbfe0000000000000, 0x4022000000000000, 0x4022000000000000];
    const PIN_VP: [u32; 16] = [0x4004fd81, 0x3f3fb16e, 0xbeafb4da, 0xbeaef96e, 0x00000000, 0x40614443, 0x3ef67e31, 0x3ef57744, 0xbf60e8d5, 0x3fe2b2cb, 0xbf4fcadc, 0xbf4eed36, 0x3f0ec47c, 0xc151a044, 0x41259d79, 0x412cecd1];
    const PIN_BACK: [u64; 4] = [0x3fe6666666666666, 0x3ff199999999999a, 0x40314ccccccccccd, 0x4036800000000000];
    const STEP: [u64; 7] = [0x4014e147ae147ae1, 0x0000000000000000, 0xc01b28f5c28f5c29, 0x400afa1f5f609be1, 0x3ff6831a4bab4167, 0xc01ec30e7281cd06, 0x4031a380bd012696];
    const STEP_VP: [u32; 16] = [0x3ff568c0, 0xc0135fb2, 0xbe963d6a, 0xbe959d28, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfceb48e, 0xc02ef7e6, 0xbeb25f01, 0xbeb1a0bd, 0xc1971373, 0xc17d1cd8, 0x41863d2a, 0x4189adfa];
    const UPDATE: [u64; 7] = [0x4010000000000000, 0x0000000000000000, 0xc018000000000000, 0x400b5cfc4a189f89, 0x3ff4c803aabe1cea, 0xc01e3df76bfb6678, 0x403204ceb9c41ddd];
    const UPDATE_VP: [u32; 16] = [0x3ff568c0, 0xc0135fb2, 0xbe963d6a, 0xbe959d28, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfceb48e, 0xc02ef7e6, 0xbeb25f01, 0xbeb1a0bd, 0xc1962325, 0xc1728008, 0x4188fe56, 0x418c6c35];
    const UPDATE_PREV: [u32; 16] = [0x3ff568c0, 0xc0135fb2, 0xbe963d6a, 0xbe959d28, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xbfceb48e, 0xc02ef7e6, 0xbeb25f01, 0xbeb1a0bd, 0xc1971373, 0xc17d1cd8, 0x41863d2a, 0x4189adfa];
    const ROTATED: [u64; 1] = [0x3fee666666666666];
    const DRAW_FOCUS: [u64; 3] = [0x400b5cfc4a189f89, 0x3ff4c803aabe1cea, 0xc01e3df76bfb6678];
    const DRAW_TARGET: [u64; 3] = [0x3ff0000000000000, 0x0000000000000000, 0x4000000000000000];
    const DRAW_VP: [u32; 16] = [0x401d3bba, 0xbf1fc999, 0xbdfdf9c5, 0xbdfceadc, 0x00000000, 0x401f8fa1, 0xbf495ee9, 0xbf48881d, 0xbefefb6e, 0xc0451069, 0xbf1c9cd7, 0xbf1bf5c9, 0xbfbaf9bc, 0x40d9099c, 0x414e54a5, 0x4155788f];
    const DRAW_AGAIN: [u64; 3] = [0x3ff0000000000000, 0x0000000000000000, 0x4000000000000000];
    const DRAW_OUT: [u64; 6] = [0x400b5cfc4a189f89, 0x3ff4c803aabe1cea, 0xc01e3df76bfb6678, 0x3fee666666666666, 0x3ff199999999999a, 0x403204ceb9c41ddd];
    const DRAW_OUT_VP: [u32; 16] = [0x3fbaa3ea, 0xc03a1487, 0xbebdb2f3, 0xbebce89a, 0x00000000, 0x3fe8ddfd, 0xbf651a8b, 0xbf64262a, 0xc0027f4d, 0xc0051172, 0xbe87a7f1, 0xbe87173e, 0xc1a33a34, 0xc102349c, 0x419025fd, 0x41938c3b];
    const DRAW_PREV: [u32; 16] = [0x401d3bba, 0xbf1fc999, 0xbdfdf9c5, 0xbdfceadc, 0x00000000, 0x401f8fa1, 0xbf495ee9, 0xbf48881d, 0xbefefb6e, 0xc0451069, 0xbf1c9cd7, 0xbf1bf5c9, 0xbfbaf9bc, 0x40d9099c, 0x414e54a5, 0x4155788f];
    const AUTO_GOAL: [u64; 1] = [0x3ff7c430dd5549bf];
    const AUTO_T: [u64; 1] = [0x3fe0000000000000];
    const AUTO_CALLS: usize = 58;
    const AUTO_LAST: [u64; 2] = [0xbfe275b9a51866e4, 0xc0183ccb294fcec8];
    const AUTO_GOAL2: [u64; 1] = [0x3ff7c430dd5549bf];
    const AUTO_T2: [u64; 1] = [0x3fd999999999999a];
    const AUTO_GOAL3: [u64; 1] = [0x3fe6666666666666];
    const AUTO_T3: [u64; 1] = [0x3fe0000000000000];

    fn same(got: &[f64], want: &[u64]) { assert_eq!(got.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), want); }
    fn same32(got: &[f32], want: &[u32]) { assert_eq!(got.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), want); }

    #[test]
    fn the_same_bits_as_the_typescript() {
        let mut c = FollowCamera::blank();
        c.target = [3.25, 1.5, -7.75]; c.yaw = 0.7; c.pitch = 1.1; c.distance = 17.3; c.aspect = 1.6;
        rebuild(&mut c);
        same(&c.forward, &FORWARD); same(&c.pos, &POS);
        same32(&c.view, &VIEW); same32(&c.proj, &PROJ); same32(&c.view_proj, &VP); same32(&c.inv_view_proj, &IVP);

        let r = project(&c, [4.0, 1.0, -6.0], 1280.0, 720.0, None, None);
        same(&[r.0, r.1], &P1); assert_eq!(r.2, P1_F);
        let r = project(&c, [4.0, 1.0, -6.0], 1280.0, 720.0, Some(0.01), Some(1e-3));
        same(&[r.0, r.1], &P2); assert_eq!(r.2, P2_F);
        let behind = [c.pos[0] - c.forward[0] * 3.0, c.pos[1] - c.forward[1] * 3.0, c.pos[2] - c.forward[2] * 3.0];
        let r = project(&c, behind, 1280.0, 720.0, None, None);
        same(&[r.0, r.1], &P3); assert_eq!(r.2, P3_F);

        let r = ray(&c, 0.3, 0.6);
        same(&r.ro, &RO); same(&r.rd, &RD);
        same(&ground_point(&c, 0.3, 0.6, Some(0.9)).unwrap(), &GP);
        same(&ground_point(&c, 0.3, 0.6, None).unwrap(), &GP0);
        assert_eq!(ground_point(&c, 0.3, 0.6, Some(100.0)).is_none(), GP_ABOVE_NONE);
        let b = planar_basis(&c);
        same(&b.fwd, &FWD); same(&b.right, &RIGHT);

        zoom(&mut c, 120.0); same(&[c.desired_distance], &ZOOM);
        zoom(&mut c, 1e5); same(&[c.desired_distance], &ZOOM_HI);
        c.yaw_goal = Some(3.0); ease_yaw(&mut c, 0.016, true); same(&[c.yaw], &EASE);
        ease_yaw(&mut c, 0.016, false); assert!(c.yaw_goal.is_none());

        c.kick = Some([0.1, 0.0, -0.2]);
        kick_in(&mut c);
        same(&c.target, &KICKED); same32(&c.view_proj, &KICK_VP); same(&focus(&c), &FOCUS_IN);
        kick_in(&mut c); same(&c.target, &KICKED);   // (already in: nothing)
        kick_out(&mut c);
        same(&c.target, &AFTER); same32(&c.view_proj, &AFTER_VP);
        frame(&mut c);
        same32(&c.prev_view_proj, &PREV); assert!(c.kick_drawn.is_none());
    }

    #[test]
    fn the_picture_half_the_same_bits() {
        // pin: clamped as asked, applied verbatim by step, the free camera handed back
        let mut c = FollowCamera::blank();
        c.yaw = 0.7; c.pitch = 1.1; c.distance = 17.3; c.desired_distance = 22.5;
        let p = set_pin(&mut c, Some(FollowCameraSetPinP { look: [1.0, 2.0, 3.0], yaw: 0.4, pitch: 2.0, dist: 0.1 })).unwrap();
        same(&[p.look[0], p.look[1], p.look[2], p.yaw, p.pitch, p.dist], &PIN_APPLIED);
        set_pin(&mut c, Some(FollowCameraSetPinP { look: [1.0, 2.0, 3.0], yaw: 0.4, pitch: -0.5, dist: 9.0 }));
        let serial = c.desired_serial;
        step(&mut c, 0.016, [5.0, 0.0, 5.0], None);
        assert_eq!(c.desired_serial, serial + 1);
        same(&[c.target[0], c.target[1], c.target[2], c.desired[0], c.desired[1], c.desired[2], c.yaw, c.pitch, c.distance, c.desired_distance], &PIN_STEP);
        same32(&c.view_proj, &PIN_VP);
        assert!(set_pin(&mut c, None).is_none());
        same(&[c.yaw, c.pitch, c.distance, c.desired_distance], &PIN_BACK);

        // the follow, then update's frame + step
        let mut c = FollowCamera::blank();
        c.target = [3.25, 1.5, -7.75]; c.yaw = 0.7; c.pitch = 1.1; c.distance = 17.3; c.aspect = 1.6; c.peek_offset = Some([0.5, 0.0, -0.25]);
        step(&mut c, 0.016, [4.0, 0.0, -6.0], Some([8.0, 0.0, -9.0]));
        same(&[c.desired[0], c.desired[1], c.desired[2], c.target[0], c.target[1], c.target[2], c.distance], &STEP);
        same32(&c.view_proj, &STEP_VP);
        c.peek_offset = None;
        update(&mut c, 0.02, [4.0, 0.0, -6.0], None);
        same(&[c.desired[0], c.desired[1], c.desired[2], c.target[0], c.target[1], c.target[2], c.distance], &UPDATE);
        same32(&c.view_proj, &UPDATE_VP); same32(&c.prev_view_proj, &UPDATE_PREV);

        // the pose in and out, focus while it is in, and the frame's 'previous'
        c.yaw += 0.25; same(&[c.yaw], &ROTATED);
        rebuild(&mut c);
        draw_in(&mut c, FollowCameraDrawInP { target: [1.0, 0.0, 2.0], yaw: 0.2, pitch: 0.9, distance: 12.0 });
        same(&focus(&c), &DRAW_FOCUS); same(&c.target, &DRAW_TARGET); same32(&c.view_proj, &DRAW_VP);
        draw_in(&mut c, FollowCameraDrawInP { target: [9.0, 9.0, 9.0], yaw: 1.0, pitch: 1.0, distance: 1.0 });
        same(&c.target, &DRAW_AGAIN);
        draw_out(&mut c);
        same(&[c.target[0], c.target[1], c.target[2], c.yaw, c.pitch, c.distance], &DRAW_OUT);
        same32(&c.view_proj, &DRAW_OUT_VP);
        frame(&mut c);
        same32(&c.prev_view_proj, &DRAW_PREV);
    }

    /// `this` is the ID: a method writes the camera it is named, and no other.
    #[test]
    fn a_method_reaches_only_its_own_camera() {
        let mut g = Box::new(Game::blank());
        let a = g.tab.camera.push(FollowCamera::blank());
        let b = g.tab.camera.push(FollowCamera::blank());
        g.tab.camera[a].yaw = 0.7;
        g.tab.camera[b].yaw = -1.5;
        let untouched = g.tab.camera[b].clone();

        follow_camera_rotate(&mut g, a, 0.25);
        same(&[g.tab.camera[a].yaw], &[0.95f64.to_bits()]);
        follow_camera_zoom(&mut g, a, 120.0);
        follow_camera_step(&mut g, a, 0.016, [4.0, 0.0, -6.0], None);
        follow_camera_frame(&mut g, a);
        assert!(g.tab.camera[a].desired_serial == 1 && g.tab.camera[a].desired == [4.0, 0.0, -6.0]);

        // autoYaw lets the camera go across `blocked`, which is handed g, and files the goal on the same camera
        let mut seen = 0;
        follow_camera_auto_yaw(&mut g, a, 0.1, [2.0, 0.0, -3.0], &mut |g, _, _| { seen += g.tab.camera.len(); false });
        assert_eq!(seen, 64);    // nothing blocked: eight yaws, four samples each, and each call sees both cameras in g
        assert_eq!(g.tab.camera[a].yaw_goal, Some(0.95));
        assert!(g.tab.camera[b].yaw_goal.is_none());

        let (bb, ub) = (&g.tab.camera[b], &untouched);
        assert_eq!((bb.yaw, bb.desired_distance, bb.desired, bb.desired_serial, bb.view_proj), (ub.yaw, ub.desired_distance, ub.desired, ub.desired_serial, ub.view_proj));
    }

    #[test]
    fn auto_yaw_the_same_bits() {
        fn run(c: &mut FollowCamera, dt: f64, blocked: &mut dyn FnMut(V3, V3) -> bool) {
            if let Some(y) = auto_yaw_due(c, dt) { c.yaw_goal = Some(auto_yaw_pick(y, [2.0, 0.0, -3.0], blocked)); }
        }
        let mut c = FollowCamera::blank();
        c.yaw = 0.7;
        let mut calls: Vec<f64> = Vec::new();
        run(&mut c, 0.1, &mut |_, p| { calls.push(p[0]); calls.push(p[2]); m::sin(p[0] * 1.7) + m::cos(p[2]) > 1.0 });
        same(&[c.yaw_goal.unwrap()], &AUTO_GOAL); same(&[c.auto_yaw_t], &AUTO_T);
        assert_eq!(calls.len(), AUTO_CALLS); same(&calls[calls.len() - 2..], &AUTO_LAST);
        run(&mut c, 0.1, &mut |_, _| true);
        same(&[c.yaw_goal.unwrap()], &AUTO_GOAL2); same(&[c.auto_yaw_t], &AUTO_T2);
        run(&mut c, 0.45, &mut |_, _| true);
        same(&[c.yaw_goal.unwrap()], &AUTO_GOAL3); same(&[c.auto_yaw_t], &AUTO_T3);
    }
}
