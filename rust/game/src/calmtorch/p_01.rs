//! A calm guard burns his torch only where he needs it. (src/game/calmtorch.ts)
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Skip every light with an owner: the place's own lights only. (calmtorch.ts carried)
pub fn carried() -> bool {
    true
}

/// The place's steady light at a point of floor, on the meter's scale. (calmtorch.ts placeLight)
pub fn place_light(g: &mut crate::game::Game, x: f64, z: f64) -> f64 {
    let mut skip = |_: &mut Game, _: f64| carried();
    let e = crate::guards::light_estimate_at(g, [x, 1.2, z], Some(&mut skip), Some(true), None, None);
    crate::game::game_visibility_of(g, e)
}

/// Is this point lit enough to work in: `bar` there, on floor in plain view inside `spillM`, or by day on daylit floor out to `dayM`. (calmtorch.ts workLit)
pub fn work_lit(g: &mut crate::game::Game, x: f64, z: f64, bar: Option<f64>) -> bool {
    let bar = bar.unwrap_or(g.k.torch.lit_at);
    if place_light(g, x, z) >= bar {
        return true;
    }
    let eye: V3 = [x, 1.5, z];
    for ring in 1..=2i32 {
        let r = g.k.torch.spill_m * (ring as f64) / 2.0;
        for k in 0..8i32 {
            let a = ((k as f64) + ((ring & 1) as f64) * 0.5) * m::PI / 4.0;
            let qx = x + m::sin(a) * r;
            let qz = z + m::cos(a) * r;
            if crate::scene_collision::segment_blocked(g, eye, [qx, 1.5, qz], None, None, None) {
                continue;
            }
            if place_light(g, qx, qz) >= bar {
                return true;
            }
        }
    }
    if crate::hours::hour_of(g) == crate::hours::HourId::Night {
        return false;
    }
    for k in 0..8i32 {
        let a = (k as f64) * m::PI / 4.0;
        let sx = m::sin(a);
        let sz = m::cos(a);
        let mut r = g.k.torch.spill_m + 3.0;
        while r <= g.k.torch.day_m + 1e-6 {
            let qx = x + sx * r;
            let qz = z + sz * r;
            if crate::scene_collision::segment_blocked(g, eye, [qx, 1.5, qz], None, None, None) {
                break;
            }
            let e = crate::hours::hour_light_at(g, [qx, 1.2, qz]);
            if crate::game::game_visibility_of(g, e) >= bar {
                return true;
            }
            r += 3.0;
        }
    }
    false
}

/// His place read for the switch: 1 lit all down his look, -1 dark here or just ahead, 0 between. (calmtorch.ts placeRead)
pub fn place_read(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let c = g.gd[gd].ch;
    let x = g.ch[c].pos[0];
    let z = g.ch[c].pos[2];
    g.stats.calmtorch.asks += 1;
    if !work_lit(g, x, z, None) {
        return -1.0;
    }
    let yaw = g.ch[c].body_yaw;
    let fx = m::sin(yaw);
    let fz = m::cos(yaw);
    let eye: V3 = [x, 1.5, z];
    let hi = g.k.torch.lit_at * g.k.torch.off_k;
    let mut lit = work_lit(g, x, z, Some(hi));
    for k in 1..=2i32 {
        let r = g.k.torch.ahead_m * (k as f64) / 2.0;
        let qx = x + fx * r;
        let qz = z + fz * r;
        if crate::scene_collision::segment_blocked(g, eye, [qx, 1.5, qz], None, None, None) {
            break;
        }
        if k == 1 && !work_lit(g, qx, qz, None) {
            return -1.0;
        }
        if lit && !work_lit(g, qx, qz, Some(hi)) {
            lit = false;
        }
    }
    if lit { 1.0 } else { 0.0 }
}

/// Is his torch this rule's to decide this frame: a staff man's hand torch on a plain patrol with nothing on his mind. (calmtorch.ts calmTorchRuled)
pub fn calm_torch_ruled(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !g.touring
        && !g.quiet_utility
        && g.gd[gd].unit.is_none()
        && g.gd[gd].team == crate::teams::Team::Guard
        && g.gd[gd].state == crate::game::GuardState::Patrol
        && crate::guards::hand_carry(g, gd) == crate::guards::HandCarry::Torch
        && g.gd[gd].awareness < g.k.torch.calm_aw
        && g.gd[gd].task.is_none()
        && g.gd[gd].body_duty.is_none()
}

/// True = his torch is dark this frame; counts `dt` on his dwell. (calmtorch.ts calmTorchDark)
pub fn calm_torch_dark(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> bool {
    let dt = if g.m.calmtorch.posing { 0.0 } else { dt };
    if !calm_torch_ruled(g, gd) {
        let mut relit = false;
        if let Some(s) = g.m.calmtorch.state_of.get_mut(gd) {
            if s.dark {
                s.dark = false;
                s.against = 0.0;
                s.held = 0.0;
                relit = true;
            }
        }
        if relit {
            click(g, gd, true);
        }
        return false;
    }
    if !g.m.calmtorch.state_of.has(gd) {
        let read = place_read(g, gd);
        let c = g.gd[gd].ch;
        let s = TorchState {
            dark: read > 0.0,
            read,
            against: 0.0,
            ask: js::rem(g.ch[c].id, 4.0) / 4.0 * g.k.torch.ask_secs,
            held: g.k.torch.hold_secs,
        };
        g.m.calmtorch.state_of.set(gd, s);
        return s.dark;
    }
    let ask_secs = g.k.torch.ask_secs;
    let s = g.m.calmtorch.state_of.get_mut(gd).unwrap();
    s.held = m::min(s.held + dt, 60.0);
    s.ask += dt;
    if dt > 0.0 && s.ask >= ask_secs {
        s.ask = 0.0;
        let read = place_read(g, gd);
        g.m.calmtorch.state_of.get_mut(gd).unwrap().read = read;
    }
    let (on_secs, off_secs, hold_secs) = (g.k.torch.on_secs, g.k.torch.off_secs, g.k.torch.hold_secs);
    let s = g.m.calmtorch.state_of.get_mut(gd).unwrap();
    if s.read != (if s.dark { -1.0 } else { 1.0 }) {
        s.against = 0.0;
    } else {
        s.against += dt;
        if dt > 0.0 && s.against >= (if s.dark { on_secs } else { off_secs }) && s.held >= hold_secs {
            s.dark = !s.dark;
            s.against = 0.0;
            s.held = 0.0;
            let dark = s.dark;
            click(g, gd, !dark);
            return dark;
        }
    }
    s.dark
}

/// The switch under his thumb, for the player's ears only: no event is filed, nobody hears it. (calmtorch.ts click)
pub fn click(g: &mut crate::game::Game, gd: GuardId, on: bool) {
    if on {
        g.stats.calmtorch.clicks_on += 1;
    } else {
        g.stats.calmtorch.clicks_off += 1;
    }
    let pos = g.ch[g.gd[gd].ch].pos;
    crate::audio_iface::game_audio_like_play(g, "click", Some(pos), Some(0.25), None);
}

/// What the rule has of a man; None = never asked of him. (calmtorch.ts calmTorchOf)
pub fn calm_torch_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::calmtorch::CalmTorchOf> {
    g.m.calmtorch.state_of.get(gd).map(|s| crate::calmtorch::CalmTorchOf {
        dark: s.dark,
        read: s.read,
        against: s.against,
        ask: s.ask,
        held: s.held,
    })
}

/// The panel's line: each staff man's torch and why. (calmtorch.ts calmTorchSummary)
pub fn calm_torch_summary(g: &mut crate::game::Game) -> String {
    let guards = g.guards.clone();
    let mut parts: Vec<String> = Vec::new();
    for gd in guards {
        if g.gd[gd].unit.is_some() || g.gd[gd].team != crate::teams::Team::Guard || g.gd[gd].state == crate::game::GuardState::Dead {
            continue;
        }
        let word = if !calm_torch_ruled(g, gd) {
            "burning (not calm)"
        } else {
            match g.m.calmtorch.state_of.get(gd) {
                None => "\u{2014}",
                Some(s) if s.dark => "off (lit place)",
                Some(_) => "ON (dark)",
            }
        };
        parts.push(alloc::format!("{} {}", g.gd[gd].callsign, word));
    }
    let men = if parts.is_empty() { String::from("nobody on the floor") } else { parts.join(" \u{b7} ") };
    alloc::format!("calm torches \u{25b8} {}", men)
}

/// The load has posed the lights: time counts again. (calmtorch.ts calmTorchPosed)
pub fn calm_torch_posed(g: &mut crate::game::Game) {
    g.m.calmtorch.posing = false;
}
