//! src/game/he40.ts lines 1 to 206
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Empties the QA log and the counters. (he40.ts he40StatsReset)
pub fn he40_stats_reset(g: &mut crate::game::Game) {
    g.m.he40.he40_log.clear();
    let s = &mut g.stats.he40;
    s.fired = 0;
    s.bursts = 0;
    s.duds = 0;
    s.gone = 0;
}

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
pub fn arc_armed(g: &mut crate::game::Game, sol: &crate::throwables::ThrowSolution) -> bool {
    arc_length(sol) >= g.k.he40.arm_m
}

/// Nothing to do: the crown test this made is `BodyHitContact::Flight` now, which combat's `body_hit` runs itself. (he40.ts insideAt)
pub fn inside_at(g: &mut crate::game::Game, pos: V3) {}

/// The living man under the reticle a round can meet, never one of Sam's own side. (he40.ts shellMan)
pub fn shell_man(g: &mut crate::game::Game) -> Option<GuardId> {
    let puppet_target = match &g.puppet {
        Some(p) => p.target,
        None => None,
    };
    let gd = match puppet_target {
        Some(t) if g.gd[t].state != crate::game::GuardState::Dead => Some(t),
        _ => g.aim_guard,
    };
    match gd {
        Some(id) if g.gd[id].state != crate::game::GuardState::Dead && g.gd[id].team != crate::teams::Team::Operative => Some(id),
        _ => None,
    }
}

/// The living bodies a round stops on, as far as Sam can see them. (he40.ts shellBodies)
pub fn shell_bodies(g: &mut crate::game::Game) -> Vec<CharId> {
    let mut out: Vec<CharId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let ch = g.gd[gd].ch;
        if g.gd[gd].state != crate::game::GuardState::Dead
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

/// This round's flight table in the launcher's shape (TypeScript hands HE40 itself to launcher.ts).
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

/// The 40 mm is up, its arc is drawn, and the arc is too short to arm. (he40.ts previewTooClose)
pub fn preview_too_close(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    if g.pl[pl].throw_preview.is_none() || !crate::game::player_rifle_up(g, pl) {
        return false;
    }
    let rifle = g.pl[pl].rifle;
    if seam_rifle_attachment(g, rifle) != Some(crate::weapons::RifleAttachment::He40) {
        return false;
    }
    let preview = g.pl[pl].throw_preview.clone().unwrap();
    !arc_armed(g, &preview)
}

/// The point to send a round at: the chest of the man under the reticle, led; else the fixture; else the cursor's point. (he40.ts shellAimPoint)
pub fn shell_aim_point(g: &mut crate::game::Game, from: V3) -> V3 {
    if let Some(gd) = shell_man(g) {
        let c = g.gd[gd].ch;
        let bone = g.ch[c].bones.get(crate::character::Bone::Chest).copied();
        let chest = v3::add(bone.unwrap_or_else(|| v3::add(g.ch[c].pos, [0.0, 1.2, 0.0])), [0.0, 0.0, 0.0]);
        let tof = v3::dist(from, chest) / (g.k.he40.speed * 0.97);
        let vel = g.ch[c].vel;
        return v3::mad(chest, [vel[0], 0.0, vel[2]], tof);
    }
    if let Some(t) = g.aim_target {
        return v3::copy(g.tab.sim_lightshome_fixture[t].pos);
    }
    let ap = g.aim_point;
    [ap[0], ap[1] + 0.03, ap[2]]
}

/// Lets one round go off `shooter`'s launcher: in the air from this frame, the thump, and the room hears it. (he40.ts fireShell)
pub fn fire_shell(g: &mut crate::game::Game, shooter: CharId, from: V3, target: V3) -> crate::he40::Shell {
    let b = ballistics(g);
    let sh = crate::he40::Shell {
        pos: v3::copy(from),
        vel: crate::launcher::launch_vel(from, target, Some(b)),
        age: 0.0,
        flown: 0.0,
        shooter,
        spin: 0.0,
    };
    g.shells.push(sh);
    g.stats.he40.fired += 1;
    seam_audio_play(g, "launcher", from, 1.0, 0.6);
    let launch_noise = g.k.he40.launch_noise;
    let ev = crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: v3::copy(g.ch[shooter].pos),
        time: g.time,
        loud: false,
        level: Some(launch_noise),
        who: Some(g.ch[shooter].id),
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
pub fn log_end(g: &mut crate::game::Game, e: &crate::he40::ShellEnd) {
    g.m.he40.he40_log.push(e.clone());
    if g.m.he40.he40_log.len() > 32 {
        g.m.he40.he40_log.remove(0);
    }
}

/// Armed and stopped: frag.ts's blast, judged from `pt`. (he40.ts burst)
pub fn burst(g: &mut crate::game::Game, sh: crate::he40::Shell, pt: V3, met: crate::he40::BurstMet) {
    let spec = crate::frag::BlastSpec {
        who: g.ch[sh.shooter].id,
        name: Some(String::from("40 mm grenade")),
        ..Default::default()
    };
    let lift = g.k.frag.lift;
    let report = crate::frag::blast(g, [pt[0], pt[1] - lift, pt[2]], &spec);
    g.stats.he40.bursts += 1;
    let met = match met {
        crate::he40::BurstMet::Prop => crate::he40::ShellEndMet::Prop,
        crate::he40::BurstMet::Air => crate::he40::ShellEndMet::Air,
        crate::he40::BurstMet::Man => crate::he40::ShellEndMet::Man,
        crate::he40::BurstMet::Floor => crate::he40::ShellEndMet::Floor,
        crate::he40::BurstMet::Static => crate::he40::ShellEndMet::Static,
    };
    log_end(g, &crate::he40::ShellEnd { at: v3::copy(pt), flown: sh.flown, armed: true, met, report: Some(report) });
}

/// A dud's velocity coming back off a surface with normal `n`. (he40.ts bounceOff)
pub fn bounce_off(g: &mut crate::game::Game, v: V3, n: V3) -> V3 {
    let vn = v3::dot(v, n);
    let v_n = v3::scale(n, vn);
    let v_t = v3::sub(v, v_n);
    v3::add(
        v3::scale(v_t, g.k.he40.keep_tangent),
        v3::scale(v_n, if vn < 0.0 { -g.k.he40.restitution } else { 1.0 }),
    )
}

/// Not armed and stopped: a 'shell' item with what motion is left, and a noise the room hears. (he40.ts dud)
pub fn dud(g: &mut crate::game::Game, sh: crate::he40::Shell, at: V3, vel: V3, met: crate::he40::DudMet, noise: Option<f64>, who: Option<f64>) {
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
    seam_audio_play(g, "canisterBounce", at, 0.8, 0.7);
    crate::particle_fx::clack_fx(g, at);
    if noise > 0.0 {
        let ev = crate::game::GameEvent {
            kind: crate::director::EventKind::Prop,
            pos: [at[0], 0.0, at[2]],
            time: g.time,
            loud: false,
            level: Some(noise),
            who: Some(who),
            ..Default::default()
        };
        g.events.push(ev);
    }
    if crate::seats::is_man_id(g, shooter_id) {
        let text = format!(
            "the 40 mm was a dud because it hit something {} m out, short of the {} m it needs to arm",
            to_fixed_1(sh.flown),
            g.k.he40.arm_m
        );
        crate::game::game_msg(g, &text);
    }
    g.stats.he40.duds += 1;
    let met = match met {
        crate::he40::DudMet::Prop => crate::he40::ShellEndMet::Prop,
        crate::he40::DudMet::Air => crate::he40::ShellEndMet::Air,
        crate::he40::DudMet::Man => crate::he40::ShellEndMet::Man,
        crate::he40::DudMet::Floor => crate::he40::ShellEndMet::Floor,
        crate::he40::DudMet::Static => crate::he40::ShellEndMet::Static,
    };
    log_end(g, &crate::he40::ShellEnd { at: v3::copy(at), flown: sh.flown, armed: false, met, report: None });
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
                if g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].team != crate::teams::Team::Operative {
                    let c = g.gd[gd].ch;
                    try_char(g, shooter, c, Some(gd), pos, dir, reach, &mut best, &mut hit_c, &mut hit_g);
                }
            }
            for cv in crate::civilians::civilian_bodies(g) {
                let c = g.gd[cv].ch;
                try_char(g, shooter, c, Some(cv), pos, dir, reach, &mut best, &mut hit_c, &mut hit_g);
            }
            let pl = g.player;
            if !g.pl[pl].down {
                let c = g.pl[pl].ch;
                try_char(g, shooter, c, None, pos, dir, reach, &mut best, &mut hit_c, &mut hit_g);
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
                        burst(g, sh, v3::mad(at, dir, -r_k.standoff), crate::he40::BurstMet::Man);
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
                    burst(g, sh, v3::mad(hit_p, ph.n, r_k.standoff), crate::he40::BurstMet::Prop);
                } else {
                    let who = g.ch[shooter].id;
                    crate::props::prop_system_apply_impulse(g, ph.prop, hit_p, [dir[0] * r_k.prop_j, 0.0, dir[2] * r_k.prop_j], who, Some(true));
                    crate::marks::chip_mark(g, hit_p, ph.n, Some(crate::marks::Surface2 { box_: ph.box_, dynamic: true }), None);
                    let sh = g.shells[i];
                    let back = bounce_off(g, sh.vel, ph.n);
                    dud(g, sh, v3::mad(pos, dir, m::max(0.0, t_p - r_k.radius)), back, crate::he40::DudMet::Prop, None, None);
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
                    burst(g, sh, v3::mad(hit_p, st.n, r_k.standoff), crate::he40::BurstMet::Static);
                } else {
                    let surface = crate::marks::static_surface(g, st.index);
                    crate::marks::chip_mark(g, hit_p, st.n, surface, None);
                    let sh = g.shells[i];
                    let back = bounce_off(g, sh.vel, st.n);
                    dud(g, sh, v3::mad(pos, dir, m::max(0.0, t_s - r_k.radius)), back, crate::he40::DudMet::Static, None, None);
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
                    burst(g, sh, [at[0], r_k.standoff, at[2]], crate::he40::BurstMet::Floor);
                } else {
                    let back = bounce_off(g, sh.vel, [0.0, 1.0, 0.0]);
                    dud(g, sh, at, back, crate::he40::DudMet::Floor, None, None);
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
                at: v3::copy(sh.pos),
                flown: sh.flown,
                armed: sh.flown >= r_k.arm_m,
                met: crate::he40::ShellEndMet::Air,
                report: None,
            });
            done = true;
        }
        if done && i < g.shells.len() {
            g.shells.remove(i);
        }
    }
}

/// A round that had not armed has found a man: he folds or flinches, it never wounds, and it drops at his feet. (he40.ts dudHitsCharacter)
pub fn dud_hits_character(g: &mut crate::game::Game, r: crate::he40::Shell, c_2: CharId, gd: Option<GuardId>, dir: V3, at: V3) {
    let c = c_2;
    match gd {
        None => {
            crate::character::anim_hit(g, c);
            seam_audio_play(g, "bodyHit", at, 0.6, 0.9);
            crate::game::game_msg(g, "a 40 mm round in the ribs — unarmed, luckily");
        }
        Some(id) if g.gd[id].civ.is_some() => {
            crate::character::anim_hit(g, c);
            let pos = g.ch[c].pos;
            seam_audio_play(g, "bodyHit", v3::add(pos, [0.0, 1.1, 0.0]), 0.6, 0.9);
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
        crate::he40::DudMet::Man,
        Some(noise),
        Some(who),
    );
}

/// `x.toFixed(1)` for the non-negative distance in the dud's line: `{:.1}` rounds an exact tie to even, toFixed upward.
fn to_fixed_1(x: f64) -> String {
    let q = x * 4.0;
    if q == m::floor(q) && js::rem(q, 2.0) != 0.0 {
        return format!("{:.1}", x + 0.05);
    }
    format!("{:.1}", x)
}

/// Rounds in flight as boxes: the stubby body and its gold nose along the line of flight. Picture only; nothing in the tick calls it. (he40.ts shellBoxes)
pub fn shell_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
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
        let body = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: v3::copy(r.pos),
            h: v3::copy(item.half),
            yaw: Some(yaw),
            rot: Some(rot),
            albedo: Some(v3::copy(item.body)),
            flags: Some(flags),
            ..Default::default()
        });
        out.push(body);
        let nose = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: v3::mad(r.pos, z, 0.031),
            h: [0.021, 0.021, 0.02],
            yaw: Some(yaw),
            rot: Some(rot),
            albedo: Some(v3::copy(item.nose)),
            flags: Some(flags),
            ..Default::default()
        });
        out.push(nose);
    }
}

/// he40.ts:105,124,187,189 g.audio.play(name, at, volume, { rate }): the play is real, but `opts` holds `Opaque` values and cannot carry the rate, so it plays at its own pitch.
fn seam_audio_play(g: &mut Game, name: &str, at: V3, volume: f64, rate: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(at), Some(volume), None);
}

/// he40.ts:86 pl.rifle.attachment: `rifle_attachment()` has no way to name the rifle, so this is its getter (weapons.ts:533) over the rifle's own fields.
fn seam_rifle_attachment(g: &mut Game, rifle: FirearmId) -> Option<crate::weapons::RifleAttachment> {
    let at = g.fa[rifle].att_idx;
    if at >= 0 { g.fa[rifle].attachments.get(at as usize).copied() } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_length_matches_the_typescript_bits() {
        let sol = crate::throwables::ThrowSolution {
            v0: [0.0; 3],
            t: 0.0,
            points: vec![[0.1, 1.5, -2.25], [3.3, 1.2, -7.1], [9.7, 0.4, -11.9]],
            land: [0.0; 3],
        };
        assert_eq!(arc_length(&sol).to_bits(), 0x402bb764c2ed4dde);
    }

    #[test]
    fn to_fixed_1_matches_the_typescript() {
        let cases: [(f64, &str); 9] = [
            (2.25, "2.3"),
            (0.75, "0.8"),
            (2.35, "2.4"),
            (1.45, "1.4"),
            (0.05, "0.1"),
            (7.95, "8.0"),
            (0.0, "0.0"),
            (8.0, "8.0"),
            (3.15, "3.1"),
        ];
        for (x, want) in cases {
            assert_eq!(to_fixed_1(x), want);
        }
    }
}
