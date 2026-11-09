pub use ::game_types::he40::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// The length of a previewed arc in metres. (he40.ts arcLength)
pub fn arc_length(sol: &crate::throwables::ThrowSolution) -> f64 {
    let mut l = 0.0;
    let mut i = 1;
    while i < sol.points.len() {
        l += v3::dist(sol.points[i - 1], sol.points[i]);
        i += 1;
    }
    l
}

/// Would a round that flew this arc be armed where it lands. (he40.ts arcArmed)
pub fn arc_armed(g: &crate::game::Game, sol: &crate::throwables::ThrowSolution) -> bool {
    arc_length(sol) >= g.k.he40.arm_m
}

/// The rifle is up with rounds in its 40 mm under it. (he40.ts, the arc's own gate)
pub fn he40_ready(g: &crate::game::Game, pl: PlayerId) -> bool {
    let r = g.pl[pl].rifle;
    crate::game::player_rifle_up(g, pl) && crate::weapons::rifle_attachment(g, r) == Some(crate::weapons::RifleAttachment::He40) && g.fa[r].shells > 0.0
}

/// The arc he has up is the launcher's, and too short to arm. (he40.ts previewTooClose)
pub fn preview_too_close(g: &crate::game::Game, pl: PlayerId) -> bool {
    let r = g.pl[pl].rifle;
    match &g.pl[pl].throw_preview {
        Some(sol) => crate::game::player_rifle_up(g, pl) && crate::weapons::rifle_attachment(g, r) == Some(crate::weapons::RifleAttachment::He40) && !arc_armed(g, sol),
        None => false,
    }
}

/// The living man under the reticle a round can meet, never one of Sam's own side. (he40.ts shellMan)
fn shell_man(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<GuardId> {
    let puppet_target = match &g.puppet {
        Some(p) => p.target,
        None => None,
    };
    let gd = match puppet_target {
        Some(t) if !crate::condition::down(g, g.gd[t].ch) => Some(t),
        _ => seat.aim_guard,
    };
    match gd {
        Some(id) if !crate::condition::down(g, g.gd[id].ch) && g.gd[id].team != crate::teams::Team::Operative => Some(id),
        _ => None,
    }
}

/// The living bodies a round stops on, as far as Sam can see them. (he40.ts shellBodies)
pub fn shell_bodies(g: &crate::game::Game) -> Vec<CharId> {
    let mut out: Vec<CharId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let ch = g.gd[gd].ch;
        if !crate::condition::down(g, g.gd[gd].ch)
            && g.gd[gd].team != crate::teams::Team::Operative
            && g.ch[ch].alive
            && !crate::fogofwar::fog_of_war_hidden(g, gd)
        {
            out.push(ch);
        }
    }
    for cv in crate::civilians::civilian_bodies(g) {
        let ch = g.gd[cv].ch;
        if g.ch[ch].alive && !crate::fogofwar::fog_of_war_hidden(g, cv) {
            out.push(ch);
        }
    }
    out
}

/// This round's flight table in the launcher's shape.
fn ballistics(g: &Game) -> crate::launcher::Ballistics {
    let k = g.k.he40;
    crate::launcher::Ballistics {
        speed: k.speed,
        gravity: k.gravity,
        drag: k.drag,
        radius: k.radius,
        life: k.life,
        substeps: k.substeps,
        solve_iters: k.solve_iters,
    }
}

/// Where a round fired now would go, cut where it first enters one of `bodies`. (he40.ts shellSolve)
pub fn shell_solve(g: &mut crate::game::Game, from: V3, target: V3, bodies: Option<&[CharId]>) -> crate::throwables::ThrowSolution {
    let r = ballistics(g);
    let sol = crate::launcher::arc_solve(g, from, target, r);
    let bodies = bodies.unwrap_or(&[]);
    if bodies.is_empty() {
        return sol;
    }
    let mut i = 1;
    while i < sol.points.len() {
        let (p0, p1) = (sol.points[i - 1], sol.points[i]);
        let at = i;
        i += 1;
        let seg = v3::sub(p1, p0);
        let len = v3::len(seg);
        if len < 1e-6 {
            continue;
        }
        let dir = v3::scale(seg, 1.0 / len);
        let mut best = f64::INFINITY;
        for &c in bodies {
            let radius = g.k.he40.radius;
            let t = crate::combat::body_hit(g, c, p0, dir, len, Some(crate::combat::BodyHitContact::Flight { radius }));
            if t >= 0.0 && t < best {
                best = t;
            }
        }
        if best < f64::INFINITY {
            let land = v3::mad(p0, dir, best);
            let mut points: Vec<V3> = sol.points[..at].to_vec();
            points.push(land);
            return crate::throwables::ThrowSolution { v0: sol.v0, t: sol.t, points, land };
        }
    }
    sol
}

/// The point to send a round at: the chest of the man under the reticle, led; else the fixture; else the cursor's point. (he40.ts shellAimPoint)
pub fn shell_aim_point(g: &crate::game::Game, seat: &crate::seats::SeatRef, from: V3) -> V3 {
    if let Some(gd) = shell_man(g, seat) {
        let c = g.gd[gd].ch;
        let bone = g.ch[c].bones.get(crate::character::Bone::Chest).copied();
        let chest = v3::add(bone.unwrap_or_else(|| v3::add(g.ch[c].pos, [0.0, 1.2, 0.0])), [0.0, 0.0, 0.0]);
        let tof = v3::dist(from, chest) / (g.k.he40.speed * 0.97);
        let vel = g.ch[c].vel;
        return v3::mad(chest, [vel[0], 0.0, vel[2]], tof);
    }
    if let Some(t) = seat.aim_target {
        return v3::copy(g.tab.sim_lightshome_fixture[t].pos);
    }
    let ap = seat.aim_point;
    [ap[0], ap[1] + 0.03, ap[2]]
}

/// Lets one round go off `shooter`'s launcher: in the air from this frame, the thump, and the room hears it. (he40.ts fireShell)
pub fn fire_shell(g: &mut crate::game::Game, shooter: CharId, from: V3, target: V3) -> crate::he40::Shell {
    let b = ballistics(g);
    let sh = crate::he40::Shell {
        pos: v3::copy(from),
        vel: crate::launcher::launch_vel(from, target, b),
        age: 0.0,
        flown: 0.0,
        shooter,
        spin: 0.0,
    };
    g.shells.push(sh);
    g.stats.he40.fired += 1;
    crate::audio_iface::game_audio_like_play(g, "launcher", Some(from), Some(1.0), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.6 }));
    let launch_noise = g.k.he40.launch_noise;
    let serial = g.serial();
    let ev = crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: v3::copy(g.ch[shooter].pos),
        time: g.time,
        loud: false,
        level: Some(launch_noise),
        who: Some(g.ch[shooter].id),
        serial,
        ..Default::default()
    };
    g.events.push(ev);
    let pl = g.player;
    if shooter == g.pl[pl].ch {
        g.pl[pl].noise = m::max(g.pl[pl].noise, launch_noise);
    }
    sh
}

/// Keeps a round's end in the capped QA log. (he40.ts logEnd)
fn log_end(g: &mut crate::game::Game, e: &crate::he40::ShellEnd) {
    g.m.he40.he40_log.push(e.clone());
    if g.m.he40.he40_log.len() > 32 {
        g.m.he40.he40_log.remove(0);
    }
}

/// Armed and stopped: frag.ts's blast, judged from `pt`. (he40.ts burst)
fn burst(g: &mut crate::game::Game, sh: crate::he40::Shell, pt: V3, met: crate::he40::ShellEndMet) {
    let spec = crate::frag::BlastSpec {
        who: g.ch[sh.shooter].id,
        name: Some(String::from("40 mm grenade")),
        ..Default::default()
    };
    let lift = g.k.frag.lift;
    let report = crate::frag::blast(g, [pt[0], pt[1] - lift, pt[2]], &spec);
    g.stats.he40.bursts += 1;
    log_end(g, &crate::he40::ShellEnd { flown: sh.flown, armed: true, met });
}

/// A dud's velocity coming back off a surface with normal `n`. (he40.ts bounceOff)
fn bounce_off(g: &crate::game::Game, v: V3, n: V3) -> V3 {
    let vn = v3::dot(v, n);
    let v_n = v3::scale(n, vn);
    let v_t = v3::sub(v, v_n);
    v3::add(
        v3::scale(v_t, g.k.he40.keep_tangent),
        v3::scale(v_n, if vn < 0.0 { -g.k.he40.restitution } else { 1.0 }),
    )
}

/// Not armed and stopped: a 'shell' item with what motion is left, and a noise the room hears. (he40.ts dud)
fn dud(g: &mut crate::game::Game, sh: crate::he40::Shell, at: V3, vel: V3, met: crate::he40::ShellEndMet, noise: Option<f64>, who: Option<f64>) {
    let noise = noise.unwrap_or(g.k.he40.dud_noise);
    let who = who.unwrap_or(0.0);
    let mut vel = vel;
    let sp = v3::len(vel);
    if sp > g.k.he40.spent_speed_max {
        vel = v3::scale(vel, g.k.he40.spent_speed_max / sp);
    }
    let shooter_id = g.ch[sh.shooter].id;
    let opts = crate::throwables::ThrowablesSpawnOpts {
        who: Some(shooter_id),
        heading: Some(v3::yaw_of(vel)),
        ..Default::default()
    };
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Shell, v3::copy(at), vel, Some(opts));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::ShellSkitter);
    crate::audio_iface::game_audio_like_play(g, "canisterBounce", Some(at), Some(0.8), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
    crate::particle_fx::clack_fx(g, at);
    if noise > 0.0 {
        let serial = g.serial();
        let ev = crate::game::GameEvent {
            kind: crate::director::EventKind::Prop,
            pos: [at[0], 0.0, at[2]],
            time: g.time,
            loud: false,
            level: Some(noise),
            who: Some(who),
            serial,
            ..Default::default()
        };
        g.events.push(ev);
    }
    g.stats.he40.duds += 1;
    log_end(g, &crate::he40::ShellEnd { flown: sh.flown, armed: false, met });
}

/// One candidate body along a substep's segment, nearest kept. (he40.ts updateShells tryChar)
fn try_char(
    g: &mut Game,
    shooter: CharId,
    c: CharId,
    gd: Option<GuardId>,
    pos: V3,
    dir: V3,
    reach: f64,
    best: &mut f64,
    hit_c: &mut Option<CharId>,
    hit_g: &mut Option<GuardId>,
) {
    if c == shooter || !g.ch[c].alive {
        return;
    }
    let radius = g.k.he40.radius;
    let t = crate::combat::body_hit(g, c, pos, dir, reach, Some(crate::combat::BodyHitContact::Flight { radius }));
    if t >= 0.0 && t < *best {
        *best = t;
        *hit_c = Some(c);
        *hit_g = gd;
    }
}

/// Is the fuze armed `t` metres on from what has been flown. (he40.ts updateShells armedAt)
fn armed_at(flown: f64, t: f64, arm_m: f64) -> bool {
    flown + m::max(0.0, t) >= arm_m
}

/// One frame of every round in the air: drag and gravity in substeps, the first thing met along each, then a burst or a dud. (he40.ts updateShells)
pub fn update_shells(g: &mut crate::game::Game, dt: f64) {
    let r_k = g.k.he40;
    let mut i = g.shells.len();
    while i > 0 {
        i -= 1;
        let mut done = false;
        let h = dt / r_k.substeps;
        let mut s = 0.0;
        while s < r_k.substeps && !done {
            s += 1.0;
            g.shells[i].age += h;
            g.shells[i].spin += 40.0 * h;
            let k = m::exp(-r_k.drag * h);
            let v = g.shells[i].vel;
            g.shells[i].vel = [v[0] * k, v[1] * k - r_k.gravity * h, v[2] * k];
            let seg = v3::scale(g.shells[i].vel, h);
            let len = v3::len(seg);
            if len < 1e-6 {
                continue;
            }
            let dir = v3::scale(seg, 1.0 / len);
            let reach = len + r_k.radius;
            let mut best = f64::INFINITY;
            let mut hit_c: Option<CharId> = None;
            let mut hit_g: Option<GuardId> = None;
            let pos = g.shells[i].pos;
            let shooter = g.shells[i].shooter;
            let mut w = 0;
            while w < g.guards.len() {
                let gd = g.guards[w];
                w += 1;
                if !crate::condition::down(g, g.gd[gd].ch) && g.gd[gd].team != crate::teams::Team::Operative {
                    let c = g.gd[gd].ch;
                    try_char(g, shooter, c, Some(gd), pos, dir, reach, &mut best, &mut hit_c, &mut hit_g);
                }
            }
            for cv in crate::civilians::civilian_bodies(g) {
                let c = g.gd[cv].ch;
                try_char(g, shooter, c, Some(cv), pos, dir, reach, &mut best, &mut hit_c, &mut hit_g);
            }
            for pl in crate::seats::men(g) {
                if !crate::condition::down(g, g.pl[pl].ch) {
                    let c = g.pl[pl].ch;
                    try_char(g, shooter, c, None, pos, dir, reach, &mut best, &mut hit_c, &mut hit_g);
                }
            }
            let st = crate::scene_collision::raycast(g, pos, dir, reach);
            let ph = crate::props::prop_system_raycast(g, pos, dir, reach);
            let t_s = match &st {
                Some(x) => x.t,
                None => f64::INFINITY,
            };
            let t_p = match &ph {
                Some(x) => x.t,
                None => f64::INFINITY,
            };
            if let Some(c) = hit_c {
                if best <= t_s && best <= t_p {
                    // a man
                    let at = v3::mad(pos, dir, best);
                    if armed_at(g.shells[i].flown, best, r_k.arm_m) {
                        g.shells[i].flown += best;
                        let sh = g.shells[i];
                        burst(g, sh, v3::mad(at, dir, -r_k.standoff), crate::he40::ShellEndMet::Man);
                    } else {
                        g.shells[i].flown += best;
                        let sh = g.shells[i];
                        dud_hits_character(g, sh, c, hit_g, dir, at);
                    }
                    done = true;
                    break;
                }
            }
            if let Some(ph) = ph.as_ref().filter(|_| t_p <= t_s) {
                // furniture: its path is counted to where its centre stops, as the preview's is
                g.shells[i].flown += m::max(0.0, t_p - r_k.radius);
                let hit_p = v3::mad(pos, dir, t_p);
                if armed_at(g.shells[i].flown, 0.0, r_k.arm_m) {
                    let sh = g.shells[i];
                    burst(g, sh, v3::mad(hit_p, ph.n, r_k.standoff), crate::he40::ShellEndMet::Prop);
                } else {
                    let who = g.ch[shooter].id;
                    crate::props::prop_system_apply_impulse(g, ph.prop, hit_p, [dir[0] * r_k.prop_j, 0.0, dir[2] * r_k.prop_j], who, Some(true));
                    crate::marks::chip_mark(g, hit_p, ph.n, Some(crate::marks::Surface2 { box_: ph.box_, dynamic: true }), None);
                    let sh = g.shells[i];
                    let back = bounce_off(g, sh.vel, ph.n);
                    dud(g, sh, v3::mad(pos, dir, m::max(0.0, t_p - r_k.radius)), back, crate::he40::ShellEndMet::Prop, None, None);
                }
                done = true;
                break;
            }
            if let Some(st) = st.as_ref() {
                // a wall, a slab, a leaf, a pane
                g.shells[i].flown += m::max(0.0, t_s - r_k.radius);
                let hit_p = v3::mad(pos, dir, t_s);
                if armed_at(g.shells[i].flown, 0.0, r_k.arm_m) {
                    let sh = g.shells[i];
                    burst(g, sh, v3::mad(hit_p, st.n, r_k.standoff), crate::he40::ShellEndMet::Static);
                } else {
                    let surface = crate::marks::static_surface(g, st.index);
                    crate::marks::chip_mark(g, hit_p, st.n, surface, None);
                    let sh = g.shells[i];
                    let back = bounce_off(g, sh.vel, st.n);
                    dud(g, sh, v3::mad(pos, dir, m::max(0.0, t_s - r_k.radius)), back, crate::he40::ShellEndMet::Static, None, None);
                }
                done = true;
                break;
            }
            g.shells[i].pos = v3::add(pos, seg);
            g.shells[i].flown += len;
            if g.shells[i].pos[1] < r_k.radius {
                // the ground where no slab is under it
                let p = g.shells[i].pos;
                let at: V3 = [p[0], r_k.radius, p[2]];
                let sh = g.shells[i];
                if armed_at(sh.flown, 0.0, r_k.arm_m) {
                    burst(g, sh, [at[0], r_k.standoff, at[2]], crate::he40::ShellEndMet::Floor);
                } else {
                    let back = bounce_off(g, sh.vel, [0.0, 1.0, 0.0]);
                    dud(g, sh, at, back, crate::he40::ShellEndMet::Floor, None, None);
                }
                done = true;
                break;
            }
        }
        if !done && g.shells[i].age > r_k.life {
            // up and out over the lot: gone
            g.stats.he40.gone += 1;
            let sh = g.shells[i];
            log_end(g, &crate::he40::ShellEnd {
                flown: sh.flown,
                armed: sh.flown >= r_k.arm_m,
                met: crate::he40::ShellEndMet::Air,
            });
            done = true;
        }
        if done && i < g.shells.len() {
            g.shells.remove(i);
        }
    }
}

/// A round that had not armed has found a man: he folds or flinches, it never wounds, and it drops at his feet. (he40.ts dudHitsCharacter)
fn dud_hits_character(g: &mut crate::game::Game, r: crate::he40::Shell, c_2: CharId, gd: Option<GuardId>, dir: V3, at: V3) {
    let c = c_2;
    match gd {
        None => {
            crate::character::anim_hit(g, c);
            crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(at), Some(0.6), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.9 }));
        }
        Some(id) if g.gd[id].civ.is_some() => {
            crate::character::anim_hit(g, c);
            let pos = g.ch[c].pos;
            crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(v3::add(pos, [0.0, 1.1, 0.0])), Some(0.6), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.9 }));
            if g.gd[id].held.is_none() {
                let pos = g.ch[c].pos;
                crate::civilians::to_alarmed(g, id, crate::civilians::CivCause::Noise, [pos[0] - dir[0] * 4.0, 0.0, pos[2] - dir[2] * 4.0], false);
            }
        }
        Some(id) => crate::launcher::stagger_guard(g, id, dir, r.shooter),
    }
    let noise = match gd {
        Some(id) if g.gd[id].civ.is_none() => 0.0,
        _ => g.k.he40.dud_hit_noise,
    };
    let who = g.ch[r.shooter].id;
    dud(
        g,
        r,
        v3::mad(at, dir, -0.06),
        [-dir[0] * 1.2, 0.8, -dir[2] * 1.2],
        crate::he40::ShellEndMet::Man,
        Some(noise),
        Some(who),
    );
}

/// Rounds in flight as boxes: the stubby body and its gold nose along the line of flight. Picture only; nothing in the tick calls it. (he40.ts shellBoxes)
pub fn shell_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let flag = crate::scene_boxes::box_flag();
    let flags = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    let item = crate::throwables::shell_item();
    let mut i = 0;
    while i < g.shells.len() {
        let r = g.shells[i];
        i += 1;
        let sp = v3::len(r.vel);
        if sp < 1e-3 {
            continue;
        }
        let z = v3::scale(r.vel, 1.0 / sp);
        let up: V3 = if m::abs(z[1]) < 0.95 { [0.0, 1.0, 0.0] } else { [1.0, 0.0, 0.0] };
        let x0 = v3::normalize(v3::cross(up, z));
        let y0 = v3::cross(z, x0);
        let (ca, sa) = (m::cos(r.spin), m::sin(r.spin));
        let x = v3::mad(v3::scale(x0, ca), y0, sa);
        let y = v3::mad(v3::scale(x0, -sa), y0, ca);
        let rot = v3::quat::from_basis(x, y, z);
        let yaw = v3::yaw_of(z);
        let body = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
            c: v3::copy(r.pos),
            h: v3::copy(item.half),
            yaw: Some(yaw),
            rot: Some(rot),
            albedo: Some(v3::copy(item.body)),
            flags: Some(flags),
            ..Default::default()
        });
        out.push(body.into());
        let nose = crate::scene_boxes::make_box_frame(g, &crate::scene_boxes::MakeBoxP {
            c: v3::mad(r.pos, z, 0.031),
            h: [0.021, 0.021, 0.02],
            yaw: Some(yaw),
            rot: Some(rot),
            albedo: Some(v3::copy(item.nose)),
            flags: Some(flags),
            ..Default::default()
        });
        out.push(nose.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    fn arc(points: Vec<V3>) -> crate::throwables::ThrowSolution {
        crate::throwables::ThrowSolution { v0: [0.0; 3], t: 0.0, points, land: [0.0; 3] }
    }

    /// The round arms by the length of the arc it flew, its bends counted: a shot closer than that is a dud.
    #[test]
    fn a_round_is_armed_by_the_length_of_its_arc() {
        assert_eq!(arc_length(&arc(vec![[0.0, 0.0, 0.0], [3.0, 4.0, 0.0], [3.0, 4.0, 12.0]])), 17.0);
        assert_eq!(arc_length(&arc(vec![[1.0, 2.0, 3.0]])), 0.0);
        let g = Box::new(Game::blank());
        let arm = g.k.he40.arm_m;
        assert!(arc_armed(&g, &arc(vec![[0.0; 3], [arm, 0.0, 0.0]])));
        assert!(!arc_armed(&g, &arc(vec![[0.0; 3], [arm - 0.01, 0.0, 0.0]])));
        assert!(arc_armed(&g, &arc(vec![[0.0; 3], [arm * 0.6, 0.0, 0.0], [arm * 0.6, 0.0, arm * 0.6]])), "bent, it is flown further");
    }
}
