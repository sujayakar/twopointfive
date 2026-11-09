use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// What he watches from the spot: the arc's middle, moved round until his doors sit in his eyes.
pub fn watch(g: &mut crate::game::Game, gd: GuardId) {
    let eye_slack = g.k.netquiet.eye_slack;
    let n = g.m.netquiet.state_of.get_mut(gd).unwrap();
    if n.arc > m::PI { n.look = None; return; }
    let s = n.spot.unwrap();
    n.look = Some(look_from(s, n.yaw, n.eye, n.eye_half, n.arc, eye_slack));
}
/// watch's arithmetic: the point 6 m out along the arc's middle, moved round by what his eyes need.
fn look_from(s: V3, yaw: f64, eye: f64, eye_half: f64, arc: f64, eye_slack: f64) -> V3 {
    let room = m::max(0.0, eye_half - eye_slack * v3::DEG - arc / 2.0);
    let y = yaw - m::sign(eye) * m::max(0.0, m::abs(eye) - room);
    [s[0] + m::sin(y) * 6.0, 0.0, s[2] + m::cos(y) * 6.0]
}

/// What a man holding a room watches: the last man's arc, low on his spot; a whip's noise.
pub fn net_quiet_look(g: &crate::game::Game, gd: GuardId, pt: &mut V3) {
    if netquiet_qa().broken { return; }
    let Some(n) = g.m.netquiet.state_of.get(gd) else { return };
    let (low, look, whip_at) = (n.low, n.look, n.whip_at);
    let at = if low && look.is_some() {
        look
    } else if whipping(g, gd) {
        whip_at
    } else {
        None
    };
    if let Some(at) = at { pt[0] = at[0]; pt[1] = 0.0; pt[2] = at[2]; }
}

/// How he stands and where his gun goes: low on his spot he is crouched; a whipping man comes round to the noise.
pub fn net_quiet_pose(g: &mut crate::game::Game, gd: GuardId, dt: f64, eyes_on_foe: bool) -> Option<crate::netquiet::NetQuietPose> {
    let n = g.m.netquiet.state_of.get(gd)?;
    let (low, whip_at) = (n.low, n.whip_at);
    if low { return Some(crate::netquiet::NetQuietPose { aim: None, crouch: g.k.netquiet.crouch }); }
    if !whipping(g, gd) || eyes_on_foe || g.time - g.gd[gd].last_seen_t < g.k.netquiet.calm_secs { return None; }
    let c = g.gd[gd].ch;
    let at = whip_at.unwrap();
    let pos = g.ch[c].pos;
    let yaw = m::atan2(at[0] - pos[0], at[2] - pos[2]);
    let rate = if crate::morale::rung_of(g, gd) >= 2.0 { g.k.netquiet.whip_turn_broken } else { g.k.netquiet.whip_turn };
    g.ch[c].aim_yaw = v3::approach_angle(g.ch[c].aim_yaw, yaw, m::max(0.0, rate - 6.0) * dt);   // (the pose's own easing adds its 6 rad/s after this)
    if g.gd[gd].speed < 0.3 { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, rate * dt); }
    Some(crate::netquiet::NetQuietPose { aim: Some([at[0], 1.1, at[2]]), crouch: 0.0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The TypeScript's bits under bun, EXACT.on, from watch's expressions copied into a script (watch is not exported) and the module's eyeSlack of 4.
    #[test]
    fn look_from_matches_the_typescripts_bits() {
        let a = look_from([1.25, 0.0, -3.5], 0.7, 0.785, 0.45, 0.6, 4.0);
        assert_eq!(a.map(f64::to_bits), [0x3ff389b644e69a79, 0, 0x4003ffdb8fe90650]);
        let b = look_from([-2.0, 0.0, 4.75], -2.3, -0.7, 1.01, 1.9, 4.0);
        assert_eq!(b.map(f64::to_bits), [0xc01ffd6155e83214, 0, 0x40124c991f5fc57c]);
    }

    fn on_a_game(f: fn(&mut Game)) {
        extern crate std;
        std::thread::Builder::new().stack_size(1 << 30).spawn(move || { let mut g = Game::blank(); f(&mut g); }).unwrap().join().unwrap();
    }

    /// watch's own lines round the arithmetic: the row is written through its key, and an arc wider than half a turn clears the look with no spot read.
    #[test]
    fn watch_writes_the_row() {
        on_a_game(|g| {
            let gd = GuardId(0);
            let row = Mind { spot: Some([1.25, 0.0, -3.5]), yaw: 0.7, eye: 0.785, eye_half: 0.45, arc: 0.6, ..Mind::default() };
            g.m.netquiet.state_of.set(gd, row);
            watch(g, gd);
            let want = look_from([1.25, 0.0, -3.5], 0.7, 0.785, 0.45, 0.6, g.k.netquiet.eye_slack);
            assert_eq!(g.m.netquiet.state_of.get(gd).unwrap().look, Some(want));
            let n = g.m.netquiet.state_of.get_mut(gd).unwrap();
            n.arc = 4.0; n.spot = None;
            watch(g, gd);
            assert_eq!(g.m.netquiet.state_of.get(gd).unwrap().look, None);
        });
    }

}
