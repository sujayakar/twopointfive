//! src/game/stickycam.ts lines 1 to 302
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `g.audio.play(name, at, vol, { rate })` (stickycam.ts:110, :126, :235)
fn seam_audio_play(g: &mut Game, name: &str, at: V3, vol: f64, rate: f64) {
    todo!("needs: game_audio_like_play's opts to carry a rate (JsMap<String, Opaque> cannot)")
}

/// The module's knobs as the launcher's flight reads them. (stickycam.ts B)
pub fn b(g: &mut crate::game::Game) -> crate::launcher::Ballistics {
    let r = &g.k.sticky;
    crate::launcher::Ballistics { speed: r.speed, gravity: r.gravity, drag: r.drag, radius: r.radius, life: r.life, substeps: r.substeps, solve_iters: r.solve_iters }
}

/// What this visit's mission hands him at least. (stickycam.ts stickyVisit)
pub fn sticky_visit(g: &mut crate::game::Game) -> f64 {
    g.m.stickycam.visit.unwrap_or(0.0)
}

/// Written at a mission's start, let go of at its quit. (stickycam.ts stickyVisitSet)
pub fn sticky_visit_set(g: &mut crate::game::Game, n: f64) {
    let k = m::max(0.0, m::min(g.k.sticky.carry, js::or(m::floor(n), 0.0)));
    if js::truthy(k) { g.m.stickycam.visit = Some(k); } else { g.m.stickycam.visit = None; }
}

/// Hand a player `n` cameras: the count on the rifle, and the load listed while the gun has a tube and he was issued any. (stickycam.ts stickyIssue)
pub fn sticky_issue(g: &mut crate::game::Game, pl: PlayerId, n: f64, listed: Option<bool>) {
    let listed = match listed { Some(l) => l, None => n > 0.0 };
    let rf = g.pl[pl].rifle;
    let carry = g.k.sticky.carry;
    let has = g.fa[rf].attachments.iter().position(|a| *a == crate::weapons::RifleAttachment::Cam);
    let want = listed && g.fa[rf].attachments.contains(&crate::weapons::RifleAttachment::Ring);
    g.fa[rf].cams = if want { m::max(0.0, m::min(m::floor(n), carry)) } else { 0.0 };
    if want && has.is_none() {
        g.fa[rf].attachments.push(crate::weapons::RifleAttachment::Cam);
    } else if !want {
        if let Some(k) = has {
            g.fa[rf].attachments.remove(k);
            let len = g.fa[rf].attachments.len() as i32;
            if g.fa[rf].att_idx >= len { g.fa[rf].att_idx = if len != 0 { 0 } else { -1 }; }
        }
    }
}

/// Where to send one for what the cursor names: over that spot at the flight's height. (stickycam.ts camAimPoint)
pub fn cam_aim_point(g: &mut crate::game::Game, from: V3) -> V3 {
    let ap = g.aim_point;
    let (mut x, mut z) = (ap[0], ap[2]);
    let (dx, dz) = (x - from[0], z - from[2]);
    let d = m::hyp2(dx, dz);
    if d < 1.5 && d > 1e-3 { x = from[0] + dx / d * 1.5; z = from[2] + dz / d * 1.5; }
    [x, g.k.sticky.aim_y, z]
}

/// The previewed flight, by the live flight's own stepping. (stickycam.ts camSolve)
pub fn cam_solve(g: &mut crate::game::Game, from: V3, target: V3) -> crate::throwables::ThrowSolution {
    let r = b(g);
    crate::launcher::arc_solve(g, from, target, r)
}

/// The launch velocity: dead level through the aim point's height. (stickycam.ts camVel)
pub fn cam_vel(g: &mut crate::game::Game, from: V3, target: V3) -> V3 {
    let r = b(g);
    crate::launcher::launch_vel(from, target, Some(r))
}

/// One camera leaves the shooter's tube; the count is the caller's to take. (stickycam.ts fireCam)
pub fn fire_cam(g: &mut crate::game::Game, pl: PlayerId, from: V3, target: V3) -> StickyCamId {
    g.pl[pl].cam_seq += 1.0;
    let id = g.pl[pl].cam_seq;
    let vel = cam_vel(g, from, target);
    let cam = g.tab.stickycam_sticky_cam.push(StickyCam { pos: v3::copy(from), n: [0.0, 1.0, 0.0], vel: Some(vel), age: 0.0, si: -1.0, id });
    g.pl[pl].cams.push(cam);
    g.stats.stickycam.fired += 1;
    seam_audio_play(g, "launcher", from, 0.5, 1.25);
    let level = g.k.sticky.launch_noise;
    prop_event(g, pl, level);
    g.pl[pl].noise = m::max(g.pl[pl].noise, level);
    cam
}

/// A 'prop' event at the player's feet, his own. (stickycam.ts:111, :236)
fn prop_event(g: &mut Game, pl: PlayerId, level: f64) {
    let c = g.pl[pl].ch;
    let (pos, who, time, serial) = (v3::copy(g.ch[c].pos), g.ch[c].id, g.time, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos, time, loud: false, level: Some(level), who: Some(who), serial, ..crate::game::GameEvent::zeroed() });
}

/// It has come to rest: stuck to static `si` with normal `n`, or (si −1) lying on what is under `at`. (stickycam.ts settle)
pub fn settle(g: &mut crate::game::Game, pl: PlayerId, cam: StickyCamId, at: V3, n: V3, si: f64) {
    g.tab.stickycam_sticky_cam[cam].vel = None;
    g.tab.stickycam_sticky_cam[cam].si = si;
    if si < 0.0 {
        let o: V3 = [at[0], m::max(at[1], 0.05), at[2]];
        let dn = crate::scene_collision::raycast(g, o, [0.0, -1.0, 0.0], 4.0);
        let dp = crate::props::prop_system_raycast(g, o, [0.0, -1.0, 0.0], 4.0);
        let t = m::min(
            match &dn { Some(d) if d.index < 1e6 => d.t, _ => f64::INFINITY },
            match &dp { Some(d) => d.t, None => f64::INFINITY },
        );
        let row = &mut g.tab.stickycam_sticky_cam[cam];
        row.pos = [at[0], if t.is_finite() { m::max(0.0, o[1] - t) } else { 0.0 }, at[2]];
        row.n = [0.0, 1.0, 0.0];
        g.stats.stickycam.dropped += 1;
    } else {
        let row = &mut g.tab.stickycam_sticky_cam[cam];
        row.pos = v3::copy(at);
        row.n = v3::normalize(n);
        g.stats.stickycam.stuck += 1;
    }
    let pos = g.tab.stickycam_sticky_cam[cam].pos;
    seam_audio_play(g, "canisterBounce", pos, if si < 0.0 { 0.3 } else { 0.22 }, 2.1);
    let (time, level, serial) = (g.time, g.k.sticky.stick_noise, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos: [pos[0], 0.0, pos[2]], time, loud: false, level: Some(level), who: Some(0.0), serial, ..crate::game::GameEvent::zeroed() });
    // the most that stay out: the oldest stuck one retires
    let mut out = 0.0;
    for i in 0..g.pl[pl].cams.len() {
        let c = g.pl[pl].cams[i];
        if g.tab.stickycam_sticky_cam[c].vel.is_none() { out += 1.0; }
    }
    while out > g.k.sticky.max {
        let mut k = None;
        for i in 0..g.pl[pl].cams.len() {
            let c = g.pl[pl].cams[i];
            if g.tab.stickycam_sticky_cam[c].vel.is_none() { k = Some(i); break; }
        }
        let Some(k) = k else { break };
        g.pl[pl].cams.remove(k);
        out -= 1.0;
        g.stats.stickycam.retired += 1;
    }
}

/// One body tried against the substep's ray; `best` and `hit` are the nearest so far. (stickycam.ts:153 tryChar)
fn try_char(g: &mut Game, c: CharId, gd: GuardId, pos: V3, dir: V3, reach: f64, best: &mut f64, hit: &mut Option<(CharId, GuardId)>) {
    if !g.ch[c].alive { return; }
    // (`() => false`: a muzzle inside him answers -1 either way; Entering answers the entering hit's own negative, and the test below is `t >= 0`)
    let t = crate::combat::body_hit(g, c, pos, dir, reach, Some(crate::combat::BodyHitContact::Entering));
    if t >= 0.0 && t < *best { *best = t; *hit = Some((c, gd)); }
}

/// One frame of every camera a player has in the air, and the once-a-second check of a stuck one's surface. (stickycam.ts updateStickies)
pub fn update_stickies(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    if g.pl[pl].cams.is_empty() { return; }
    // (a copy: a camera coming to rest may retire the oldest out of the list; a retired one is still walked, as there)
    let walk: Vec<StickyCamId> = g.pl[pl].cams.clone();
    for cam in walk {
        if g.tab.stickycam_sticky_cam[cam].vel.is_none() {
            let was = m::floor(g.tab.stickycam_sticky_cam[cam].age);
            g.tab.stickycam_sticky_cam[cam].age += dt;
            let row = g.tab.stickycam_sticky_cam[cam];
            if row.si >= 0.0 && m::floor(row.age) != was && crate::scene_collision::raycast(g, v3::mad(row.pos, row.n, 0.05), v3::scale(row.n, -1.0), 0.1).is_none() {
                settle(g, pl, cam, row.pos, [0.0, 1.0, 0.0], -1.0);
            }
            continue;
        }
        let (substeps, drag, gravity, radius, life) = (g.k.sticky.substeps, g.k.sticky.drag, g.k.sticky.gravity, g.k.sticky.radius, g.k.sticky.life);
        let h = dt / substeps;
        let mut done = false;
        let mut s = 0.0;
        while s < substeps && !done {
            s += 1.0;
            g.tab.stickycam_sticky_cam[cam].age += h;
            let k = m::exp(-drag * h);
            let v = g.tab.stickycam_sticky_cam[cam].vel.unwrap();
            let vel: V3 = [v[0] * k, v[1] * k - gravity * h, v[2] * k];
            g.tab.stickycam_sticky_cam[cam].vel = Some(vel);
            let seg = v3::scale(vel, h);
            let len = v3::len(seg);
            if len < 1e-6 { continue; }
            let dir = v3::scale(seg, 1.0 / len);
            let reach = len + radius;
            let pos = g.tab.stickycam_sticky_cam[cam].pos;
            let mut best = f64::INFINITY;
            let mut hit: Option<(CharId, GuardId)> = None;
            for i in 0..g.guards.len() {
                let gd = g.guards[i];
                if g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].team != crate::teams::Team::Operative {
                    let c = g.gd[gd].ch;
                    try_char(g, c, gd, pos, dir, reach, &mut best, &mut hit);
                }
            }
            for cv in crate::civilians::civilian_bodies(g) {
                let c = g.gd[cv].ch;
                try_char(g, c, cv, pos, dir, reach, &mut best, &mut hit);
            }
            let st = crate::scene_collision::raycast(g, pos, dir, reach);
            let ph = crate::props::prop_system_raycast(g, pos, dir, reach);
            let t_s = match &st { Some(x) => x.t, None => f64::INFINITY };
            let t_p = match &ph { Some(x) => x.t, None => f64::INFINITY };
            if let Some((c, gd)) = hit {
                if best <= t_s && best <= t_p {
                    let at = v3::mad(pos, dir, m::max(0.0, best - 0.1));
                    if g.gd[gd].civ.is_some() {
                        crate::character::anim_hit(g, c);
                        if g.gd[gd].held.is_none() {
                            let p = g.ch[c].pos;
                            crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Noise, [p[0] - dir[0] * 4.0, 0.0, p[2] - dir[2] * 4.0], false);
                        }
                    } else {
                        let shooter = g.pl[pl].ch;
                        crate::launcher::stagger_guard(g, gd, dir, shooter);
                    }
                    settle(g, pl, cam, at, [0.0, 1.0, 0.0], -1.0);
                    done = true;
                    break;
                }
            }
            if ph.is_some() && t_p <= t_s {
                settle(g, pl, cam, v3::mad(pos, dir, m::max(0.0, t_p - 0.1)), [0.0, 1.0, 0.0], -1.0);
                done = true;
                break;
            }
            if let Some(hit) = &st {
                let at = v3::mad(pos, dir, hit.t);
                if hit.index >= 1e6 {
                    settle(g, pl, cam, v3::mad(pos, dir, m::max(0.0, hit.t - 0.1)), [0.0, 1.0, 0.0], -1.0);
                } else {
                    settle(g, pl, cam, at, hit.n, hit.index);
                }
                done = true;
                break;
            }
            let moved = v3::add(pos, seg);
            g.tab.stickycam_sticky_cam[cam].pos = moved;
            if moved[1] < radius {
                settle(g, pl, cam, [moved[0], 0.0, moved[2]], [0.0, 1.0, 0.0], -1.0);
                done = true;
                break;
            }
        }
        let row = g.tab.stickycam_sticky_cam[cam];
        if !done && row.age > life {
            settle(g, pl, cam, row.pos, [0.0, 1.0, 0.0], -1.0);
        }
    }
}

/// Is its surface flat. (stickycam.ts flat)
pub fn flat(g: &mut crate::game::Game, c_2: StickyCamId) -> bool {
    m::abs(g.tab.stickycam_sticky_cam[c_2].n[1]) > g.k.sticky.flat_y
}

/// Where a stuck camera's lens stands. (stickycam.ts camLens)
pub fn cam_lens(g: &mut crate::game::Game, c_2: StickyCamId) -> V3 {
    let c = g.tab.stickycam_sticky_cam[c_2];
    v3::mad(c.pos, c.n, g.k.sticky.off)
}

/// The stuck cameras that send a picture: the newest `live` of them. (stickycam.ts stickyLive)
pub fn sticky_live(g: &mut crate::game::Game, pl: &crate::stickycam::StickyLivePl) -> Vec<StickyCamId> {
    let mut out: Vec<StickyCamId> = Vec::new();
    let n = m::max(0.0, m::floor(g.k.sticky.live));
    let mut i = pl.cams.len();
    while i > 0 && (out.len() as f64) < n {
        i -= 1;
        if g.tab.stickycam_sticky_cam[pl.cams[i]].vel.is_none() { out.push(pl.cams[i]); }
    }
    out
}

/// The fog's eyes a player's live stuck cameras are, newest first. (stickycam.ts stickyEyes)
pub fn sticky_eyes(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogEye> {
    let pl = g.player;
    if g.pl[pl].cams.is_empty() { return Vec::new(); }
    let mut out: Vec<crate::fogofwar::FogEye> = Vec::new();
    let wide = g.k.sticky.fov_deg;
    let live = sticky_live(g, &crate::stickycam::StickyLivePl { cams: g.pl[pl].cams.clone() });
    for c in live {
        let p = cam_lens(g, c);
        let all = flat(g, c);
        let row = g.tab.stickycam_sticky_cam[c];
        out.push(crate::fogofwar::FogEye {
            id: EYE_ID + js::rem(row.id, 50.0),
            pos: p,
            look: if all { [1.0, 0.0, 0.0] } else { v3::normalize([row.n[0], 0.0, row.n[2]]) },
            at: p,
            fov: Some(if all { 360.0 } else { wide }),
            near: Some(0.0),
            wand: None,
        });
    }
    out
}

/// Has he a tube on the gun in his hands to put one on. (stickycam.ts canTake)
pub fn can_take(g: &mut crate::game::Game, pl: PlayerId) -> bool {
    let rf = g.pl[pl].rifle;
    g.fa[rf].attachments.contains(&crate::weapons::RifleAttachment::Ring)
}

/// His own stuck camera nearest his feet within `r` that he has a clear line to from his eyes. (stickycam.ts camNear)
pub fn cam_near(g: &mut crate::game::Game, pl: PlayerId, r: f64) -> Option<StickyCamId> {
    let ch = g.pl[pl].ch;
    let p = g.ch[ch].pos;
    let eye: V3 = [p[0], if g.pl[pl].crouch { 0.9 } else { 1.3 }, p[2]];
    let mut best = None;
    let mut bd = r;
    for i in 0..g.pl[pl].cams.len() {
        let c = g.pl[pl].cams[i];
        if g.tab.stickycam_sticky_cam[c].vel.is_some() { continue; }
        let d = v3::dist_xz(g.tab.stickycam_sticky_cam[c].pos, p);
        if d < bd {
            let lens = cam_lens(g, c);
            if !crate::scene_collision::solid_blocked(g, eye, lens, None) { best = Some(c); bd = d; }
        }
    }
    best
}

/// Nothing in his hands that takes them. (stickycam.ts handsFree)
pub fn hands_free(g: &mut crate::game::Game, pl: PlayerId) -> bool {
    let p = &g.pl[pl];
    p.dragging.is_none() && !p.sprinting && p.takedown.is_none() && p.holding.is_none()
}

/// The camera he could take back this frame. (stickycam.ts camToTake)
pub fn cam_to_take(g: &mut crate::game::Game, pl: PlayerId) -> Option<StickyCamId> {
    if can_take(g, pl) && hands_free(g, pl) {
        let reach = g.k.sticky.take_reach;
        cam_near(g, pl, reach)
    } else {
        None
    }
}

/// The interaction row of the nearest stuck camera. (stickycam.ts stickyRows)
pub fn sticky_rows(g: &mut crate::game::Game, pl: PlayerId, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::stickycam::StickyRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    if !can_take(g, pl) { return; }
    let Some(c) = cam_near(g, pl, crate::player::NEARBY_M) else { return };
    let ch = g.pl[pl].ch;
    let near = v3::dist_xz(g.tab.stickycam_sticky_cam[c].pos, g.ch[ch].pos) < g.k.sticky.take_reach;
    let reach = near && hands_free(g, pl);
    let pos = cam_lens(g, c);
    let it = crate::game::Interactable {
        kind: crate::game::InteractableKind::Cam,
        pos,
        line1: String::from("sticky camera"),
        line2: String::from(if reach { "" } else if near { "hands full" } else { "step closer" }),
        in_reach: reach,
        off: false,
        ..crate::game::Interactable::zeroed()
    };
    // (row spreads `it` into a new object and offer keeps `it` itself: the clone is the spread)
    let verbs = if reach && !busy {
        vec![row(g, it.clone(), crate::game::Verb::TakeCam, String::from("take it back"), None, None)]
    } else {
        Vec::new()
    };
    offer(g, it, verbs);
}

/// Space on that row: the camera comes off its wall and is one more on his launcher. False = nothing to take. (stickycam.ts takeCam)
pub fn take_cam(g: &mut crate::game::Game, pl: PlayerId) -> bool {
    let Some(c) = cam_to_take(g, pl) else { return false };
    let k = g.pl[pl].cams.iter().position(|x| *x == c).unwrap();
    g.pl[pl].cams.remove(k);
    let rf = g.pl[pl].rifle;
    let n = m::min(g.k.sticky.carry, g.fa[rf].cams + 1.0);
    sticky_issue(g, pl, n, Some(true));
    g.stats.stickycam.taken += 1;
    let pos = g.tab.stickycam_sticky_cam[c].pos;
    seam_audio_play(g, "canisterBounce", pos, 0.16, 2.6);
    let level = g.k.sticky.take_noise;
    prop_event(g, pl, level);
    g.pl[pl].noise = m::max(g.pl[pl].noise, level);
    true
}

/// A camera of his is still in the air. (stickycam.ts stickyFlying)
pub fn sticky_flying(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    g.pl[pl].cams.iter().any(|c| g.tab.stickycam_sticky_cam[*c].vel.is_some())
}

/// What a checkpoint writes as left to send: the rifle's count and any still in the air. (stickycam.ts stickyLeft)
pub fn sticky_left(g: &mut crate::game::Game, pl: PlayerId) -> f64 {
    let rf = g.pl[pl].rifle;
    let mut n = g.fa[rf].cams;
    for c in &g.pl[pl].cams {
        if g.tab.stickycam_sticky_cam[*c].vel.is_some() { n += 1.0; }
    }
    m::min(n, g.k.sticky.carry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::RifleAttachment as A;

    fn game() -> (Box<Game>, PlayerId) {
        let mut g = Box::new(Game::blank());
        let rifle = g.fa.push(crate::weapons::Firearm::blank());
        let pl = g.pl.push(crate::game::Player { rifle, ..crate::game::Player::blank() });
        g.player = pl;
        (g, pl)
    }
    fn cam(g: &mut Game, pl: PlayerId, id: f64, flying: bool) -> StickyCamId {
        let c = g.tab.stickycam_sticky_cam.push(StickyCam { pos: [0.0; 3], n: [0.0, 1.0, 0.0], vel: if flying { Some([1.0, 0.0, 0.0]) } else { None }, age: 0.0, si: -1.0, id });
        g.pl[pl].cams.push(c);
        c
    }

    /// the TypeScript's own bits, under EXACT
    #[test]
    fn aim_point_pushes_a_near_cursor_out_to_a_throw() {
        let (mut g, _) = game();
        g.aim_point = [1.1, 0.0, 0.7];
        let p = cam_aim_point(&mut g, [0.3, 1.2, -0.4]);
        assert_eq!((p[0].to_bits(), p[2].to_bits()), (0x3ff2ea86e39b87ec, 0x3fea04f2f1ebb5c9));
        assert_eq!(p[1], g.k.sticky.aim_y);
        g.aim_point = [10.25, 0.0, -3.5];
        let p = cam_aim_point(&mut g, [0.3, 1.2, -0.4]);
        assert_eq!((p[0].to_bits(), p[2].to_bits()), (0x4024800000000000, 0xc00c000000000000));
    }

    #[test]
    fn issue_lists_the_load_only_on_a_gun_with_a_tube() {
        let (mut g, pl) = game();
        let rf = g.pl[pl].rifle;
        g.fa[rf].attachments = vec![A::Ring, A::He40];
        sticky_issue(&mut g, pl, 9.7, None);
        assert_eq!(g.fa[rf].cams, g.k.sticky.carry);
        assert_eq!(g.fa[rf].attachments, vec![A::Ring, A::He40, A::Cam]);
        // nothing issued: the load comes off, and the selection stays on the list
        g.fa[rf].att_idx = 2;
        sticky_issue(&mut g, pl, 0.0, None);
        assert_eq!((g.fa[rf].cams, g.fa[rf].att_idx), (0.0, 0));
        assert_eq!(g.fa[rf].attachments, vec![A::Ring, A::He40]);
        // all sent, still listed: the load stays, dry
        sticky_issue(&mut g, pl, 0.0, Some(true));
        assert_eq!(g.fa[rf].attachments, vec![A::Ring, A::He40, A::Cam]);
        // no tube: never listed
        g.fa[rf].attachments = vec![A::He40];
        sticky_issue(&mut g, pl, 2.0, None);
        assert_eq!((g.fa[rf].cams, g.fa[rf].attachments.len()), (0.0, 1));
    }

    #[test]
    fn the_newest_stuck_camera_sends_and_one_in_the_air_does_not() {
        let (mut g, pl) = game();
        let (a, b, _flying) = (cam(&mut g, pl, 1.0, false), cam(&mut g, pl, 2.0, false), cam(&mut g, pl, 3.0, true));
        g.k.sticky.live = 1.0;
        let cams = StickyLivePl { cams: g.pl[pl].cams.clone() };
        assert_eq!(sticky_live(&mut g, &cams), vec![b]);
        g.k.sticky.live = 3.0;
        assert_eq!(sticky_live(&mut g, &cams), vec![b, a]);
        g.k.sticky.live = 1.0;
        let eyes = sticky_eyes(&mut g);
        assert_eq!(eyes.len(), 1);
        assert_eq!((eyes[0].id, eyes[0].fov, eyes[0].near), (202.0, Some(360.0), Some(0.0)));
    }

    #[test]
    fn left_to_send_counts_the_air_up_to_the_carry() {
        let (mut g, pl) = game();
        let rf = g.pl[pl].rifle;
        g.fa[rf].cams = 1.0;
        cam(&mut g, pl, 1.0, false);
        assert!(!sticky_flying(&mut g));
        cam(&mut g, pl, 2.0, true);
        assert!(sticky_flying(&mut g));
        assert_eq!(sticky_left(&mut g, pl), 2.0);
        g.fa[rf].cams = g.k.sticky.carry;
        assert_eq!(sticky_left(&mut g, pl), g.k.sticky.carry);
    }

    #[test]
    fn a_visit_is_held_floored_and_capped_and_zero_lets_go() {
        let (mut g, _) = game();
        sticky_visit_set(&mut g, 2.9);
        assert_eq!(sticky_visit(&mut g), 2.0);
        sticky_visit_set(&mut g, 99.0);
        assert_eq!(sticky_visit(&mut g), g.k.sticky.carry);
        sticky_visit_set(&mut g, f64::NAN);
        assert_eq!(g.m.stickycam.visit, None);
    }
}
