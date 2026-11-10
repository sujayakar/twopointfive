//! WHERE HE AIMS: the cursor's ray and the point it gives, and the reticle's snap to a man or a lamp.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};

/// The gun's heading eased to the cursor's bearing.
pub fn aim_ease(yaw: f64, to_aim: V3, dt: f64) -> f64 {
    yaw + v3::wrap_angle(v3::yaw_of(to_aim) - yaw) * (1.0 - m::exp(-16.0 * dt))
}

/// The aim of a cursor whose ray met nothing: level, a metre ahead of the way his body faces.
pub fn aim_level(c_2: crate::player::AimLevelC) -> V3 {
    v3::add(v3::add(c_2.pos, [m::sin(c_2.body_yaw), 0.0, m::cos(c_2.body_yaw)]), [0.0, 1.15, 0.0])
}

/// What the cursor's ray meets: the first thing of the world on it, never the man's own body.
pub fn cursor_hit(g: &mut crate::game::Game, ray: crate::player::CursorHitRay, own: f64) -> Option<crate::scene_boxes::BoxWorldRaycast> {
    crate::scene_boxes::box_world_raycast(g, ray.ro, ray.rd, Some(200.0), None, Some(own), None)
}

/// The point the cursor's ray gives: where it hit, else the floor under it, else None.
pub fn cursor_point(g: &crate::game::Game, cam: CameraId, u: f64, v: f64, ray: crate::player::CursorPointRay, hit: Option<crate::player::CursorPointHit>) -> Option<V3> {
    match hit {
        Some(h) => Some(v3::mad(ray.ro, ray.rd, h.t)),
        None => crate::camera::follow_camera_ground_point(g, cam, u, v, Some(0.0)),
    }
}

/// His cursor this tick: where it is in the view, the ray it casts and what that meets.
struct Cursor { cam: CameraId, view: crate::host_hud::View, at: [f64; 2], ray: crate::camera::FollowCameraRay, hit_t: Option<f64>, hit_box: Option<f64> }
impl Cursor {
    fn of(g: &mut Game, input: InputId, cam: CameraId, view: crate::host_hud::View) -> Cursor {
        let at = [g.tab.input[input].mouse_x, g.tab.input[input].mouse_y];
        let ray = crate::camera::follow_camera_ray(g, cam, at[0] / m::max(1.0, view.client_width), at[1] / m::max(1.0, view.client_height));
        let own = g.ch[g.pl[g.player].ch].id;
        let hit = cursor_hit(g, crate::player::CursorHitRay { ro: ray.ro, rd: ray.rd }, own);
        Cursor { cam, view, at, ray, hit_t: hit.as_ref().map(|h| h.t), hit_box: hit.as_ref().map(|h| h.index) }
    }
    /// Where it points.
    fn point(&self, g: &Game) -> V3 {
        let (u, v) = (self.at[0] / m::max(1.0, self.view.client_width), self.at[1] / m::max(1.0, self.view.client_height));
        let hit = self.hit_t.map(|t| crate::player::CursorPointHit { t });
        let him = &g.ch[g.pl[g.player].ch];
        cursor_point(g, self.cam, u, v, crate::player::CursorPointRay { ro: self.ray.ro, rd: self.ray.rd }, hit)
            .unwrap_or_else(|| aim_level(crate::player::AimLevelC { pos: him.pos, body_yaw: him.body_yaw }))
    }
    /// How far from it a place is in the view, in pixels; None: the place is behind the eye.
    fn px(&self, g: &Game, p: V3) -> Option<f64> {
        let (x, y, front) = crate::camera::follow_camera_project(g, self.cam, p, self.view.client_width, self.view.client_height, None, None);
        if front { Some(m::hyp2(x - self.at[0], y - self.at[1])) } else { None }
    }
}

/// What the reticle is on: a lamp or a man, where on it, how near the cursor that is; and one of his own standing in the way.
#[derive(Clone, Copy, Default)]
struct Reticle { lamp: Option<FixtureId>, man: Option<GuardId>, at: Option<V3>, friend: Option<GuardId>, px: f64, on_lamp: bool }

/// His aim this tick: the cursor's point, his rows, then the reticle's snap to a man or a lamp.
pub fn update_aim(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    let hand_at = g.hand_at;
    if let Some(aim) = hand_at {
        let (x, y, front) = crate::camera::follow_camera_project(g, cam, aim, canvas.client_width, canvas.client_height, Some(f64::INFINITY), Some(1e-3));
        if front { g.tab.input[input].mouse_x = x; g.tab.input[input].mouse_y = y; }
    }
    let q = Cursor::of(g, input, cam, canvas);
    g.sets[g.seat].aim_point = match hand_at { Some(aim) => aim, None => q.point(g) };
    build_interactables(g);
    let taken_at = g.m.player.light_at.get(g.player).copied();
    let mut r = Reticle { px: crate::aim::SNAP_PX, ..Default::default() };
    if hand_at.is_none() { lamp_under(g, &q, taken_at, &mut r); }
    man_near(g, &q, &mut r);
    if r.man.is_none() { man_on_ray(g, &q, &mut r); }
    if crate::samstun::sam_blind(g, &crate::seats::here(g), None) > 0.5 { r = Reticle { friend: r.friend, ..Default::default() }; }
    let s = &mut g.sets[g.seat];
    s.aim_target = r.lamp;
    s.aim_guard = r.man;
    s.aim_snap = r.at;
    s.aim_friend = r.friend;
    if let (Some(at), None) = (r.at, hand_at) { s.aim_point = at; }
    if r.lamp.is_none() { g.m.player.light_at.delete(g.player); }
    else if r.on_lamp || taken_at.is_none() { g.m.player.light_at.set(g.player, q.at); }
}

/// The lamp under the cursor. One taken is KEPT while the cursor stays where it took it.
fn lamp_under(g: &Game, q: &Cursor, taken_at: Option<[f64; 2]>, r: &mut Reticle) {
    let s = &g.sets[g.seat];
    let reach = m::min(crate::aim::SNAP_PX, g.k.aim.player.light_snap_px);
    let kept = s.aim_target.filter(|_| taken_at.is_some_and(|a| m::hyp2(q.at[0] - a[0], q.at[1] - a[1]) <= reach));
    for it in s.interactables.iter().filter(|it| it.kind == crate::game::InteractableKind::Light) {
        let Some(lamp) = it.target else { continue };
        let Some(d) = q.px(g, it.pos).filter(|&d| d < r.px) else { continue };
        let on = d < reach || q.hit_box.is_some_and(|x| g.tab.fixture[lamp].boxes.contains(&x));
        if on || Some(lamp) == kept { *r = Reticle { lamp: Some(lamp), at: Some(it.pos), px: d, on_lamp: on, ..*r }; }
    }
}

/// The man nearest the cursor, if he is nearer than the lamp and nothing stands between.
fn man_near(g: &mut Game, q: &Cursor, r: &mut Reticle) {
    let eye = v3::add(g.ch[g.pl[g.player].ch].pos, [0.0, 1.3, 0.0]);
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if crate::condition::down(g, g.gd[gd].ch) || in_his_arm(g, gd) || crate::fogofwar::fog_of_war_hidden(g, gd) || crate::friends::friend(g, gd) { continue; }
        let chest = chest_or(g, gd, 1.2);
        let Some(d) = q.px(g, chest).map(|d| d - 6.0) else { continue };
        if d < r.px && !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) { *r = Reticle { man: Some(gd), lamp: None, at: Some(chest), px: d, ..*r }; }
    }
}
fn in_his_arm(g: &Game, gd: GuardId) -> bool { g.pl[g.player].holding().is_some_and(|h| h.g == gd) }

/// The ray itself still counts where it lands ON a man the cursor is not near the middle of: the nearest body on it decides, and one of his
/// own nearer than that is who is in the way.
fn man_on_ray(g: &mut Game, q: &Cursor, r: &mut Reticle) {
    let lamp = *r;
    let (mut best, mut friend_t) = (q.hit_t.unwrap_or(1e9), f64::INFINITY);
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if in_his_arm(g, gd) || crate::fogofwar::fog_of_war_hidden(g, gd) { continue; }
        let t = crate::character::character_ray_hit(g, g.gd[gd].ch, q.ray.ro, q.ray.rd, best + 0.5);
        if !(t >= 0.0 && t < best + 0.5) { continue; }
        if crate::friends::friend(g, gd) {
            if t < friend_t { friend_t = t; r.friend = Some(gd); }
            continue;
        }
        *r = Reticle { man: Some(gd), lamp: None, at: Some(chest_or(g, gd, 1.2)), ..*r };
        best = t;
    }
    if r.friend.is_none() || r.man.is_none() { return; }
    if friend_t >= best { r.friend = None; } else { *r = Reticle { friend: r.friend, ..lamp }; }
}

/// A man's centre of mass: his chest bone, or his feet raised by `up` before he has bones.
fn chest_or(g: &crate::game::Game, gd: GuardId, up: f64) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(b) => *b,
        None => v3::add(g.ch[c].pos, [0.0, up, 0.0]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// A failure means the gun swings the long way round to a cursor just past behind him, or at another rate at another frame rate.
    #[test]
    fn the_gun_eases_the_short_way_round_at_any_frame_rate() {
        let behind_him: V3 = [-0.04, 0.0, -1.0];                     // a bearing just over the seam at half a turn
        let eased = aim_ease(3.0, behind_him, 1.0 / 60.0);
        assert!(eased > 3.0 && eased < 3.0 + 0.4, "{eased}");
        assert_eq!(aim_ease(0.3, [1.0, 0.0, 2.0], 0.0), 0.3);
        let bearing = v3::yaw_of([1.0, 0.0, 2.0]);
        assert!((aim_ease(0.3, [1.0, 0.0, 2.0], 5.0) - bearing).abs() < 1e-9, "he gets there and no further");
        let once = aim_ease(0.3, [1.0, 0.0, 2.0], 1.0 / 30.0);
        let twice = aim_ease(aim_ease(0.3, [1.0, 0.0, 2.0], 1.0 / 60.0), [1.0, 0.0, 2.0], 1.0 / 60.0);
        assert!((once - twice).abs() < 1e-12, "{once} {twice}");
    }
    /// A cursor whose ray met nothing still aims somewhere: a metre ahead of the way his body faces, at chest height off his feet.
    #[test]
    fn a_cursor_on_nothing_aims_level_a_metre_ahead_of_his_body() {
        let near = |a: V3, b: V3| for k in 0..3 { assert!((a[k] - b[k]).abs() < 1e-12, "{a:?} {b:?}"); };
        near(aim_level(crate::player::AimLevelC { pos: [1.5, 0.0, -2.25], body_yaw: 0.0 }), [1.5, 1.15, -1.25]);
        near(aim_level(crate::player::AimLevelC { pos: [1.5, 1.0, -2.25], body_yaw: m::PI / 2.0 }), [2.5, 2.15, -2.25]);
    }
}
