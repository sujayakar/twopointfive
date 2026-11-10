//! A BODY IN TOW: taken hold of, dragged or carried, let go.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

/// F: let go of the body you are dragging, or take hold of the nearest one within reach
pub fn toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) {
    let pl = g.player;
    if g.pl[pl].tow.is_some() {
        release_body(g, None);
        return;
    }
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].sprinting { return; }
    let mut best: Option<GuardId> = match which {
        Some(w) if crate::condition::down(g, g.gd[w].ch) => Some(w),
        _ => None,
    };
    if best.is_none() {
        let mut bd = 1.3;
        let mut all: Vec<GuardId> = g.guards.clone();
        all.extend(crate::civilians::civilians_shown(g));
        for gd in all {
            if !crate::condition::down(g, g.gd[gd].ch) { continue; }
            let d = body_dist(g, gd, Some(1.3));
            if d < bd {
                bd = d;
                best = Some(gd);
            }
        }
    }
    let best = match best {
        Some(b) if g.ch[g.gd[b].ch].ragdoll.is_some() => b,
        _ => {
            return;
        }
    };
    crate::unseen::kneel(g, pl);
    release_wall(g);
    crate::deed::put_down(g, pl);
    g.pl[pl].tow = Some(crate::game::Tow { who: best, haul: None, yaw: 0.0, noise_t: 0.3, stuck_t: 0.0 });
    let bc = g.gd[best].ch;
    if let Some(r) = g.ch[bc].ragdoll { crate::ragdoll::ragdoll_wake(g, r); }
    crate::hands::haul_begin(g, best);
}

/// Let go of the body in tow: the ankle pins come off and its feet fall where they are
pub(in crate::sam) fn release_body(g: &mut crate::game::Game, laid: Option<bool>) {
    let laid = laid.unwrap_or(false);
    let pl = g.player;
    let Some(tow) = g.pl[pl].tow.take() else { return };
    let b = g.gd[tow.who].ch;
    if let Some(r) = g.ch[b].ragdoll { crate::ragdoll::ragdoll_clear_pins(g, r); }
    let c = g.pl[pl].ch;
    g.ch[c].anim.haul = None;
    let at = match g.ch[b].bones.get(crate::character::Bone::FootL) { Some(f) => *f, None => g.ch[b].pos };
    crate::audio_iface::game_audio_like_play(g, "bodyFall", Some(at), Some(if laid { 0.12 } else { 0.3 }), None);
}

/// A body in tow: pinned at his hands by the ankles, and he faces its hips; torn out of his hands if it wedges.
pub(super) fn tow_tick(g: &mut Game, t: Tick, speed: f64) {
    let Tick { pl, c, dt, .. } = t;
    let Some(mut tow) = g.pl[pl].tow else { return };
    let body = g.gd[tow.who].ch;
    let rag = g.ch[body].ragdoll;
    let hips = g.ch[body].bones.get(crate::character::Bone::Hips).copied().unwrap_or(g.ch[body].pos);
    let (dx, dz) = (hips[0] - g.ch[c].pos[0], hips[2] - g.ch[c].pos[2]);
    let d = m::hyp2(dx, dz);
    if d > 1e-4 { tow.yaw = m::atan2(dx, dz); }
    let strained = match rag { Some(r) => g.tab.ragdoll[r].strain > 0.2 || crate::hands::haul_slack(g), None => false };
    tow.stuck_t = if strained { tow.stuck_t + dt } else { 0.0 };
    g.pl[pl].tow = Some(tow);
    match rag {
        Some(rag) if d <= 2.2 && tow.stuck_t <= 0.25 => {
            if crate::hands::haul_set_done(g) { release_body(g, Some(true)); return; }
            if !crate::hands::haul_pins(g, dt) { pin_ankles(g, c, rag); }
            face_body(g, t, tow.yaw, speed);
            scrape(g, t, rag, hips, speed);
        }
        _ => crate::game::game_toggle_drag(g, None),
    }
}
fn pin_ankles(g: &mut Game, c: CharId, rag: RagdollId) {
    let f = crate::character::character_forward(g, c);
    let (hx, hz) = (g.ch[c].pos[0] + f[0] * 0.35, g.ch[c].pos[2] + f[2] * 0.35);
    let bones = crate::ragdoll::rag();
    crate::ragdoll::ragdoll_set_pin(g, rag, bones.ank_l, [hx + f[2] * 0.1, 0.42, hz - f[0] * 0.1]);
    crate::ragdoll::ragdoll_set_pin(g, rag, bones.ank_r, [hx - f[2] * 0.1, 0.42, hz + f[0] * 0.1]);
}
/// With it on his shoulder he faces the way he walks; hauling or dragging it he faces it, and backs off.
fn face_body(g: &mut Game, t: Tick, yaw: f64, speed: f64) {
    let Tick { pl, c, dt, .. } = t;
    if crate::hands::haul_upright(g, &crate::seats::here(g)) {
        let turn = g.k.hands.carry_turn;
        let ch = &mut g.ch[c];
        if speed > 0.2 { ch.body_yaw = v3::approach_angle(ch.body_yaw, v3::yaw_of(ch.vel), turn * dt); }
        ch.aim_yaw = ch.body_yaw;
        ch.aim_pitch = v3::damp(ch.aim_pitch, 0.0, 6.0, dt);
        g.pl[pl].backpedal = false;
        g.ch[c].anim.reverse = false;
        return;
    }
    let face_yaw = crate::hands::haul_yaw(g, &crate::seats::here(g)).unwrap_or(yaw);
    let hauling = g.pl[pl].haul().is_some();
    let ch = &mut g.ch[c];
    ch.aim_pitch = v3::damp(ch.aim_pitch, if hauling { -0.45 } else { -0.9 }, 6.0, dt);
    let rel = v3::approach_angle(v3::wrap_angle(ch.aim_yaw - ch.body_yaw), 0.0, 8.0 * dt);
    ch.body_yaw = v3::approach_angle(ch.body_yaw, face_yaw, 10.0 * dt);
    ch.aim_yaw = if hauling { ch.body_yaw + rel } else { face_yaw };
    let back = speed > 0.2 && m::cos(v3::wrap_angle(face_yaw - v3::yaw_of(ch.vel))) < 0.0;
    g.pl[pl].backpedal = back;
    g.ch[c].anim.reverse = back;
}
/// A soft scrape now and then, while the body really slides.
fn scrape(g: &mut Game, t: Tick, rag: RagdollId, hips: V3, speed: f64) {
    let Tick { pl, c, dt, .. } = t;
    let carried = g.pl[pl].haul().map_or(false, |h| h.carry.is_some());
    if speed <= 0.3 || g.tab.ragdoll[rag].sleeping || carried { return; }
    let Some(tow) = g.pl[pl].tow.as_mut() else { return };
    tow.noise_t -= dt;
    if tow.noise_t > 0.0 { return; }
    tow.noise_t = 0.55;
    crate::audio_iface::game_audio_like_play(g, "propScrape", Some(hips), Some(0.25), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
    prop_noise(g, [hips[0], 0.0, hips[2]], 0.22);
}

/// Planar distance from the player to a corpse: the nearer of its ankles, its hips and its grip.
pub(super) fn body_dist(g: &mut crate::game::Game, gd: GuardId, within: Option<f64>) -> f64 {
    let within = within.unwrap_or(f64::INFINITY);
    let me = g.pl[g.player].ch;
    let p = g.ch[me].pos;
    let c = g.gd[gd].ch;
    let a: V3 = [p[0], 0.5, p[2]];
    let foot_l = g.ch[c].bones.get(crate::character::Bone::FootL).copied();
    let foot_r = g.ch[c].bones.get(crate::character::Bone::FootR).copied();
    let hips = g.ch[c].bones.get(crate::character::Bone::Hips).copied();
    let l = body_reach(g, a, p, foot_l, 0.0, within);
    let r = body_reach(g, a, p, foot_r, 0.0, within);
    let h = body_reach(g, a, p, hips, 0.3, within) + 0.3;
    let grip = crate::hands::body_grip_dist(g, gd, within);
    m::min_n(&[l, r, h, grip])
}

/// bodyDist's reach: not through a partition or a shut door; a point already past `within` keeps its plain distance, its line never walked.
fn body_reach(g: &mut crate::game::Game, a: V3, p: V3, q: Option<V3>, add: f64, within: f64) -> f64 {
    let Some(q) = q else { return 99.0 };
    let d = v3::dist_xz(q, p);
    if d + add >= within {
        return d;
    }
    let t: V3 = [q[0], 0.5, q[2]];
    if crate::scene_collision::solid_blocked(g, a, t, None) { 99.0 } else { d }
}
