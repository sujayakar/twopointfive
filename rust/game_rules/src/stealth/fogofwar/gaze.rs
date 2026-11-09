use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The field every eye has this frame, degrees in plan: the cone's rows under FOG.cone, else fovDeg.
pub fn field_deg(g: &crate::game::Game) -> f64 {
    if g.k.fog.cone { m::min(g.k.fog.fov_deg, g.k.fog.cone_deg) } else { g.k.fog.fov_deg }
}

/// Its all-round near field, metres.
pub fn field_near(g: &crate::game::Game) -> f64 {
    if g.k.fog.cone { g.k.fog.cone_near_m } else { g.k.fog.near_m }
}

/// Where a man looks, for the cone (unit, in plan): his head, or his aim with a long gun in the shoulder and level.
pub fn gaze_of(g: &crate::game::Game, c_2: CharId) -> V3 {
    let h = crate::character::character_head_dir(g, c_2);
    let body_yaw = g.ch[c_2].body_yaw;
    let aim = if g.ch[c_2].alive && g.ch[c_2].anim.long_shown == Some(crate::character::LongStance::Shoulder) {
        let gun_yaw = crate::character::character_gun_yaw(g, c_2);
        let (gun_dir, rl) = (g.ch[c_2].gun_dir, g.ch[c_2].anim.rl_shown);
        Some(GazeAim { gun_yaw, gun_x: gun_dir[0], gun_z: gun_dir[2], rl_carry: rl.carry, rl_head: rl.head })
    } else {
        None
    };
    gaze_from(h[0], h[2], body_yaw, aim)
}

/// What gazeOf reads of a shouldered gun.
#[derive(Clone, Copy)]
struct GazeAim { gun_yaw: f64, gun_x: f64, gun_z: f64, rl_carry: f64, rl_head: f64 }

/// gazeOf's arithmetic, on what it read of the man.
fn gaze_from(head_x: f64, head_z: f64, body_yaw: f64, aim: Option<GazeAim>) -> V3 {
    let (mut x, mut z) = (head_x, head_z);
    let hl = m::hyp2(x, z);
    if hl > 1e-6 { x /= hl; z /= hl; } else { x = m::sin(body_yaw); z = m::cos(body_yaw); }
    if let Some(a) = aim {
        let (ax, az, gl) = (m::sin(a.gun_yaw), m::cos(a.gun_yaw), m::hyp2(a.gun_x, a.gun_z));
        let lv = if gl > 1e-6 { (a.gun_x * ax + a.gun_z * az) / gl } else { 0.0 };
        let sights = gaze_sights();
        let t = m::min(1.0, m::max(0.0, (lv - sights[0]) / (sights[1] - sights[0])));
        let rl = m::min(1.0, m::max(a.rl_carry, a.rl_head));
        let w = t * t * (3.0 - 2.0 * t) * (1.0 - rl * rl * (3.0 - 2.0 * rl));
        if w > 0.0 {
            x += (ax - x) * w;
            z += (az - z) * w;
            let l = m::hyp2(x, z);
            if l > 1e-6 { x /= l; z /= l; } else { x = ax; z = az; }
        }
    }
    [x, 0.0, z]
}

/// FOG.eyes as the count the game uses: from five to the room the renderer has; a row that is no number is five.
pub(super) fn fog_eye_count(v: f64) -> f64 {
    if v.is_nan() { FOG_EYES_WAS } else { m::max(FOG_EYES_WAS, m::min(crate::render_fogofwar::FOG_EYES, m::floor(v))) }
}

/// The address's words for the fog, read by the Game's constructor. With no page it does nothing, as the TypeScript's catch does; the host owns the address.
pub fn fog_flags(g: &crate::game::Game) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn level(gun_yaw: f64) -> GazeAim { GazeAim { gun_yaw, gun_x: m::sin(gun_yaw), gun_z: m::cos(gun_yaw), rl_carry: 0.0, rl_head: 0.0 } }
    fn bearing(l: V3) -> f64 { m::atan2(l[0], l[2]) }

    /// With no gun in the shoulder he looks where his head is turned, whatever its length.
    #[test]
    fn a_man_looks_where_his_head_is_turned() {
        assert_eq!(gaze_from(3.0, 4.0, 1.0, None), [0.6, 0.0, 0.8]);
    }

    /// A long gun in the shoulder and level is where he looks, wherever his head is.
    #[test]
    fn a_level_gun_in_the_shoulder_is_his_look() {
        let l = gaze_from(0.6, 0.8, 0.0, Some(level(1.9)));
        assert!((bearing(l) - 1.9).abs() < 1e-12, "{:?}", l);
        assert_eq!(gaze_from(-1.0, 0.0, 0.0, Some(level(m::PI / 2.0))), [1.0, 0.0, m::cos(m::PI / 2.0)], "even with his head turned straight away from it");
    }

    /// A gun that is not level on its own yaw (low, or no direction at all) leaves his look to his head; between the two sights' numbers it shares it.
    #[test]
    fn a_gun_off_its_level_leaves_the_look_to_his_head() {
        let tipped = |cos: f64| GazeAim { gun_yaw: 0.0, gun_x: m::sqrt(1.0 - cos * cos), gun_z: cos, rl_carry: 0.0, rl_head: 0.0 };
        let (head_x, head_z) = (m::sin(1.0), m::cos(1.0));
        let head = gaze_from(head_x, head_z, 0.0, None);
        assert_eq!(gaze_from(head_x, head_z, 0.0, Some(tipped(0.5))), head);
        assert_eq!(gaze_from(head_x, head_z, 0.0, Some(GazeAim { gun_x: 0.0, gun_z: 0.0, ..level(0.0) })), head);
        let sights = gaze_sights();
        let half = bearing(gaze_from(head_x, head_z, 0.0, Some(tipped((sights[0] + sights[1]) / 2.0))));
        assert!(half > 0.0 && half < 1.0, "half way between head and gun: {half}");
        assert!((bearing(gaze_from(head_x, head_z, 0.0, Some(tipped(1.0))))).abs() < 1e-12);
    }

    /// A gun being carried up or lifted to rest (the ready layers) is not aimed: the look goes back to his head.
    #[test]
    fn a_gun_held_up_is_not_his_look() {
        let (head_x, head_z) = (m::sin(1.0), m::cos(1.0));
        let head = gaze_from(head_x, head_z, 0.0, None);
        assert_eq!(gaze_from(head_x, head_z, 0.0, Some(GazeAim { rl_carry: 1.0, ..level(0.0) })), head);
        assert_eq!(gaze_from(head_x, head_z, 0.0, Some(GazeAim { rl_head: 1.0, ..level(0.0) })), head);
        let part = bearing(gaze_from(head_x, head_z, 0.0, Some(GazeAim { rl_carry: 0.5, ..level(0.0) })));
        assert!(part > 0.0 && part < 1.0, "{part}");
    }

    /// A look is a bearing: a head bowed to the floor does not tip it, and one with no bearing at all is his facing.
    #[test]
    fn a_look_is_level_whatever_the_pose() {
        // (kneeling with the gun low, as measured: the head 45 degrees down and ahead)
        let l = gaze_from(-0.14, -0.72, 1.0, None);
        assert_eq!(l[1], 0.0);
        assert!((m::hyp2(l[0], l[2]) - 1.0).abs() < 1e-12 && l[2] < -0.95, "he looks the way his head is turned: {:?}", l);
        // (straight down: no bearing in it, so his facing)
        assert_eq!(gaze_from(0.0, 0.0, 0.5, None), [m::sin(0.5), 0.0, m::cos(0.5)]);
    }

    /// The eyes the fog keeps: five at the least, the renderer's room at the most, and a row that is no number is five.
    #[test]
    fn the_eyes_are_five_up_to_the_room_the_renderer_has() {
        for (v, want) in [(f64::NAN, 5.0), (f64::NEG_INFINITY, 5.0), (0.0, 5.0), (4.9, 5.0), (5.0, 5.0), (6.9, 6.0), (7.0, 7.0), (7.5, 7.0), (f64::INFINITY, 7.0)] {
            assert_eq!(fog_eye_count(v), want, "{v}");
        }
    }
}
