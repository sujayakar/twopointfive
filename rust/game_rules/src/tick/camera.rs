pub use ::game_types::camera::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};

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

/// Hold the view (none: let it go).
pub fn follow_camera_set_pin(g: &mut Game, id: CameraId, p: Option<FollowCameraSetPinP>) -> Option<FollowCameraPin> { set_pin(&mut g.tab.camera[id], p) }

/// A drawn frame and a tick's follow, together.
pub fn follow_camera_update(g: &mut Game, id: CameraId, dt: f64, follow: V3, aim_point: Option<V3>) {
    update(&mut g.tab.camera[id], dt, follow, aim_point)
}
fn update(c: &mut FollowCamera, dt: f64, follow: V3, aim_point: Option<V3>) {
    frame(c);
    step(c, dt, follow, aim_point);
}

/// Once a game tick: the follow itself, or the pin verbatim.
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

/// Once a drawn frame: 'previous' becomes the matrix the last frame was drawn with.
fn frame(c: &mut FollowCamera) {
    c.prev_view_proj = v3::m4::copy(&c.view_proj);
}

/// The matrices from look-at, angles and distance; the tick calls it too.
pub fn follow_camera_rebuild(g: &mut Game, id: CameraId) {
    rebuild(&mut g.tab.camera[id])
}
pub(super) fn rebuild(c: &mut FollowCamera) {
    let (cp, sp, cy, sy) = (m::cos(c.pitch), m::sin(c.pitch), m::cos(c.yaw), m::sin(c.yaw));
    c.forward = [-sy * cp, -sp, -cy * cp];
    c.pos = v3::mad(c.target, c.forward, -c.distance);
    c.view = v3::m4::look_at(c.pos, c.target, [0.0, 1.0, 0.0]);
    c.proj = v3::m4::perspective(c.fov, c.aspect, c.near, c.far);
    c.view_proj = v3::m4::mul(&c.proj, &c.view);
    // (a singular matrix leaves the old inverse)
    if let Some(i) = v3::m4::invert(&c.view_proj) {
        c.inv_view_proj = i;
    }
}

pub fn follow_camera_zoom(g: &mut Game, id: CameraId, delta: f64) {
    zoom(&mut g.tab.camera[id], delta)
}
fn zoom(c: &mut FollowCamera, delta: f64) {
    c.desired_distance = v3::clamp(c.desired_distance * m::exp(delta * 0.0012), 6.0, 40.0);
}

pub fn follow_camera_rotate(g: &mut Game, id: CameraId, d_yaw: f64) {
    g.tab.camera[id].yaw += d_yaw;
}

/// World point to CSS pixels of a w×h view, and whether it is showable.
pub fn follow_camera_project(g: &Game, id: CameraId, p: V3, w: f64, h: f64, margin: Option<f64>, min_w: Option<f64>) -> (f64, f64, bool) {
    project(&g.tab.camera[id].view_proj, p, w, h, margin, min_w)
}
/// where a point of the world is in a picture `w` by `h`, and whether it is in front
pub fn project(vp: &crate::v3::m4::Mat4, p: V3, w: f64, h: f64, margin: Option<f64>, min_w: Option<f64>) -> (f64, f64, bool) {
    let margin = margin.unwrap_or(f64::INFINITY);
    let min_w = min_w.unwrap_or(0.0);
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

/// World-space ray through a pixel in [0,1]² UV, origin top-left.
pub fn follow_camera_ray(g: &Game, id: CameraId, u: f64, v: f64) -> FollowCameraRay {
    ray(&g.tab.camera[id].inv_view_proj, u, v)
}
/// the ray through a point of the picture (u, v from its top left)
pub fn ray(inv_view_proj: &crate::v3::m4::Mat4, u: f64, v: f64) -> FollowCameraRay {
    let (ndc_x, ndc_y) = (u * 2.0 - 1.0, 1.0 - v * 2.0);
    let p0 = v3::m4::transform_point(inv_view_proj, [ndc_x, ndc_y, 0.0]);
    let p1 = v3::m4::transform_point(inv_view_proj, [ndc_x, ndc_y, 1.0]);
    FollowCameraRay { ro: p0, rd: v3::normalize(v3::sub(p1, p0)) }
}

/// A UV ray against the plane y = h.
pub fn follow_camera_ground_point(g: &Game, id: CameraId, u: f64, v: f64, h: Option<f64>) -> Option<V3> {
    ground_point(&g.tab.camera[id].inv_view_proj, u, v, h)
}
/// that ray against the plane y = h
pub fn ground_point(inv_view_proj: &crate::v3::m4::Mat4, u: f64, v: f64, h: Option<f64>) -> Option<V3> {
    let h = h.unwrap_or(0.0);
    let FollowCameraRay { ro, rd } = ray(inv_view_proj, u, v);
    if m::abs(rd[1]) < 1e-5 {
        return None;
    }
    let t = (h - ro[1]) / rd[1];
    if t < 0.0 {
        return None;
    }
    Some(v3::mad(ro, rd, t))
}

/// Camera-relative planar basis for WASD.
pub fn follow_camera_planar_basis(g: &Game, id: CameraId) -> FollowCameraPlanarBasis {
    planar_basis(&g.tab.camera[id])
}
fn planar_basis(c: &FollowCamera) -> FollowCameraPlanarBasis {
    let f = v3::normalize([c.forward[0], 0.0, c.forward[2]]);
    FollowCameraPlanarBasis { fwd: f, right: [-f[2], 0.0, f[0]] }
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use super::*;

    fn at_near(a: V3, b: V3, what: &str) { assert!(v3::dist(a, b) < 1e-3, "{what}: {a:?} against {b:?}"); }
    fn near(a: f64, b: f64, what: &str) { assert!(m::abs(a - b) < 1e-3, "{what}: {a} against {b}"); }
    /// the game's usual camera over (3.25, 1.5, -7.75)
    fn cam() -> FollowCamera {
        let mut c = FollowCamera::blank();
        c.target = [3.25, 1.5, -7.75]; c.yaw = 0.7; c.pitch = 1.1; c.distance = 17.3; c.desired_distance = 17.3; c.aspect = 1.6;
        rebuild(&mut c);
        c
    }

    #[test]
    fn the_camera_stands_back_from_what_it_looks_at_along_its_angles() {
        let c = cam();
        near(v3::len(c.forward), 1.0, "its line");
        at_near(v3::mad(c.pos, c.forward, c.distance), c.target, "its line reaches what it looks at");
        assert!(c.pos[1] > c.target[1], "a pitch above nothing looks down");
        let mut top = FollowCamera::blank();
        top.pitch = m::PI / 2.0 - 1e-3; top.distance = 10.0; top.aspect = 1.6;
        rebuild(&mut top);
        near(top.pos[1], 10.0, "straight down: it hangs over the mark");
    }

    /// What a click means: the point of the floor under a pixel, and that pixel of the floor's point.
    #[test]
    fn a_point_of_the_picture_and_a_point_of_the_floor_come_back_to_each_other() {
        let c = cam();
        let (w, h) = (1280.0, 720.0);
        let (x, y, front) = project(&c.view_proj, c.target, w, h, None, None);
        assert!(front);
        assert!(m::abs(x - w / 2.0) < 0.5 && m::abs(y - h / 2.0) < 0.5, "what it looks at is the middle of the picture: {x} {y}");
        for (u, v) in [(0.3, 0.6), (0.9, 0.1), (0.5, 0.5)] {
            let p = ground_point(&c.inv_view_proj, u, v, Some(0.9)).expect("the floor is under the camera");
            near(p[1], 0.9, "on the height asked");
            let (x, y, _) = project(&c.view_proj, p, w, h, None, None);
            assert!(m::abs(x / w - u) < 1e-3 && m::abs(y / h - v) < 1e-3, "{u} {v} came back as {} {}", x / w, y / h);
            let r = ray(&c.inv_view_proj, u, v);
            near(v3::len(r.rd), 1.0, "a ray's way");
            near(v3::len(v3::sub(v3::mad(r.ro, r.rd, v3::dist(r.ro, p)), p)), 0.0, "the ray goes through the point");
        }
        assert!(ground_point(&c.inv_view_proj, 0.3, 0.6, Some(100.0)).is_none(), "a plane over the camera is not under the pixel");
        // the picture's top is further off than its bottom, and right is right
        let (top, bottom) = (ground_point(&c.inv_view_proj, 0.5, 0.2, None).unwrap(), ground_point(&c.inv_view_proj, 0.5, 0.8, None).unwrap());
        assert!(v3::dist(top, c.pos) > v3::dist(bottom, c.pos), "the top of a picture looks further off");
    }

    #[test]
    fn what_is_behind_the_camera_or_out_of_the_margin_is_not_showable() {
        let c = cam();
        let behind = v3::mad(c.pos, c.forward, -3.0);
        assert!(!project(&c.view_proj, behind, 1280.0, 720.0, None, None).2, "behind");
        assert!(project(&c.view_proj, c.target, 1280.0, 720.0, Some(0.01), Some(1e-3)).2, "in the very middle, inside the smallest margin");
        let off = ground_point(&c.inv_view_proj, 0.9, 0.5, None).unwrap();
        assert!(project(&c.view_proj, off, 1280.0, 720.0, None, None).2, "in front");
        assert!(!project(&c.view_proj, off, 1280.0, 720.0, Some(0.5), None).2, "and out of the margin");
    }

    /// W and D walk up and across the picture.
    #[test]
    fn w_goes_up_the_picture_and_d_across_it() {
        let c = cam();
        let b = planar_basis(&c);
        near(b.fwd[1], 0.0, "on the floor");
        near(v3::len(b.fwd), 1.0, "a metre");
        near(v3::dot(b.fwd, b.right), 0.0, "square");
        let (x0, y0, _) = project(&c.view_proj, c.target, 1280.0, 720.0, None, None);
        let (_, y, _) = project(&c.view_proj, v3::add(c.target, b.fwd), 1280.0, 720.0, None, None);
        assert!(y < y0, "forward is up the picture");
        let (x, _, _) = project(&c.view_proj, v3::add(c.target, b.right), 1280.0, 720.0, None, None);
        assert!(x > x0, "right is to the right of it");
    }

    #[test]
    fn the_wheel_zooms_by_a_share_of_the_distance_and_stops_at_both_ends() {
        let mut c = FollowCamera::blank();
        c.desired_distance = 20.0;
        zoom(&mut c, 100.0);
        let out = c.desired_distance;
        assert!(out > 20.0);
        zoom(&mut c, -100.0);
        near(c.desired_distance, 20.0, "as far in as it went out");
        zoom(&mut c, 1e5);
        assert_eq!(c.desired_distance, 40.0);
        zoom(&mut c, -1e5);
        assert_eq!(c.desired_distance, 6.0);
    }

    /// The pin holds the view as asked, kept off straight down and out of the near plane, and letting go gives the free camera back.
    #[test]
    fn a_pinned_view_stands_as_asked_and_gives_the_free_camera_back() {
        let mut c = cam();
        c.desired_distance = 22.5;
        let asked = |pitch: f64, dist: f64| Some(FollowCameraSetPinP { look: [1.0, 2.0, 3.0], yaw: 0.4, pitch, dist });
        let p = set_pin(&mut c, asked(2.0, 0.1)).unwrap();
        assert_eq!(p.pitch, PIN_PITCH_MAX);
        assert_eq!(p.dist, c.near + 0.05);
        // a second pin does not take the first for the free camera
        set_pin(&mut c, asked(-0.5, 9.0));
        let serial = c.desired_serial;
        step(&mut c, 0.016, [5.0, 0.0, 5.0], None);
        assert_eq!(c.desired_serial, serial + 1, "a cut each step");
        assert_eq!((c.target, c.desired, c.yaw, c.pitch, c.distance, c.desired_distance), ([1.0, 2.0, 3.0], [1.0, 2.0, 3.0], 0.4, -0.5, 9.0, 9.0), "the pin, whatever it was following");
        assert!(set_pin(&mut c, None).is_none());
        assert_eq!((c.yaw, c.pitch, c.distance, c.desired_distance), (0.7, 1.1, 17.3, 22.5));
        let shown = c.yaw;
        step(&mut c, 0.016, [5.0, 0.0, 5.0], None);
        assert_eq!(c.yaw, shown);
        assert_ne!(c.target, [1.0, 2.0, 3.0], "free again, it follows");
    }

    /// It leads toward the aim by its share and a peek, and closes on that at the same rate on any machine.
    /// A FAILURE MEANS: the camera lags more on a slower frame rate, or jumps to where it is going.
    #[test]
    fn the_camera_follows_toward_his_aim_at_the_same_rate_at_any_frame_rate() {
        let (follow, aim): (V3, V3) = ([4.0, 0.0, -6.0], [8.0, 0.0, -10.0]);
        let mut c = cam();
        c.aim_lead = 0.25;
        c.peek_offset = Some([0.5, 0.0, -0.25]);
        let start = c.target;
        step(&mut c, 0.016, follow, Some(aim));
        at_near(c.desired, [5.5, 0.0, -7.25], "a quarter of the way to the aim, and the peek");
        let (d0, d1) = (v3::dist(start, c.desired), v3::dist(c.target, c.desired));
        assert!(d1 < d0 && d1 > d0 * 0.8, "it closes, and not at once: {d0} to {d1}");
        let (mut one, mut four) = (cam(), cam());
        step(&mut one, 0.1, follow, None);
        for _ in 0..4 { step(&mut four, 0.025, follow, None); }
        at_near(one.target, four.target, "one long frame or four short ones");
        near(one.distance, four.distance, "and the zoom");
        for _ in 0..600 { step(&mut four, 0.05, follow, None); }
        at_near(four.target, follow, "and it gets there");
    }

    /// The frame's previous matrix is the last one drawn with: what the picture blends from.
    #[test]
    fn update_keeps_last_frames_matrix_as_the_previous() {
        let mut c = cam();
        let was = c.view_proj;
        update(&mut c, 0.02, [4.0, 0.0, -6.0], None);
        assert_eq!(c.prev_view_proj, was);
        assert_ne!(c.view_proj, was);
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
        assert_eq!(g.tab.camera[a].yaw, 0.95);
        follow_camera_zoom(&mut g, a, 120.0);
        follow_camera_update(&mut g, a, 0.016, [4.0, 0.0, -6.0], None);
        assert!(g.tab.camera[a].desired_serial == 1 && g.tab.camera[a].desired == [4.0, 0.0, -6.0]);

        let (bb, ub) = (&g.tab.camera[b], &untouched);
        assert_eq!((bb.yaw, bb.desired_distance, bb.desired, bb.desired_serial, bb.view_proj), (ub.yaw, ub.desired_distance, ub.desired, ub.desired_serial, ub.view_proj));
    }
}
