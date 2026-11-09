use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Once a frame before the guards: the seated men kept on their marks while calm, stood up when not.
pub fn close_seat_tick(g: &mut crate::game::Game, dt: f64) {
    match &g.m.close.seated {
        Some(seated) if !seated.is_empty() => {}
        _ => return,
    }
    let mut at = 0;
    // A live walk: closeUnseat deletes the row the walk stands on, as under the TypeScript's `for … of` over a Map.
    while let Some(gd) = g.m.close.seated.as_ref().and_then(|seated| seated.next_key(&mut at)) {
        let Some(st) = g.m.close.seated.as_ref().and_then(|seated| seated.get(gd).copied()) else { continue };
        if !g.guards.contains(&gd) || !calm(g, gd) {
            close_unseat(g, gd, None);
            continue;
        }
        let c = g.gd[gd].ch;
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, st.face, 3.0 * dt);
        let settled = m::abs(v3::wrap_angle(g.ch[c].body_yaw - st.face)) < 0.35;
        let tx = st.at[0] + m::sin(st.face) * crate::civilians::SIT_ADVANCE;
        let tz = st.at[2] + m::cos(st.face) * crate::civilians::SIT_ADVANCE;
        let k = 1.0 - m::exp(-5.0 * dt);
        if settled {
            g.ch[c].pos[0] += (tx - g.ch[c].pos[0]) * k;
            g.ch[c].pos[2] += (tz - g.ch[c].pos[2]) * k;
            let clip = sit_clip();
            g.ch[c].anim.act = Some(crate::character::AnimAct { clip: clip.clip, loop_: Some(clip.loop_), rate: Some(clip.rate), in_: Some(clip.in_), out: Some(clip.out) });
        }
        g.gd[gd].speed = 0.0;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_serial = g.gd[gd].path_serial.wrapping_add(1);
        g.gd[gd].path_goal = None;
        g.gd[gd].path_goal_orbit = false;
        let down = settled && crate::character::anim_act_shown(g, c) > 0.5;
        if let Some(row) = g.m.close.seated.as_mut().and_then(|seated| seated.get_mut(gd)) {
            row.down = down;
        }
    }
}

/// Everybody up, this seat's close-in memory gone.
pub fn close_reset(g: &mut crate::game::Game) {
    let seated = g.m.close.seated.as_ref().map(|seated| seated.keys()).unwrap_or_default();
    for gd in seated {
        close_unseat(g, gd, None);
    }
    let player = g.player;
    let Some(s) = g.m.close.state.get_mut(player) else { return };
    s.best = None;
    s.t = -1.0;
    s.mode = None;
    s.mode_d = f64::INFINITY;
    s.rmb_t = -1.0;
}

