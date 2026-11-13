pub use ::game_types::he40::types;
pub use types::*;
use crate::flight::Met;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::{string::String, vec::Vec};

/// The length of a previewed arc in metres.
pub fn arc_length(sol: &crate::throwables::ThrowSolution) -> f64 {
    let mut l = 0.0;
    let mut i = 1;
    while i < sol.points.len() {
        l += v3::dist(sol.points[i - 1], sol.points[i]);
        i += 1;
    }
    l
}

/// Would a round that flew this arc be armed where it lands.
pub fn arc_armed(g: &crate::game::Game, sol: &crate::throwables::ThrowSolution) -> bool {
    arc_length(sol) >= g.k.he40.arm_m
}

/// The rifle is up with rounds in its 40 mm under it.
pub fn he40_ready(g: &crate::game::Game, pl: PlayerId) -> bool {
    let r = g.pl[pl].rifle;
    crate::game::player_rifle_up(g, pl) && crate::weapons::rifle_attachment(g, r) == Some(crate::weapons::RifleAttachment::He40) && g.fa[r].shells > 0.0
}

/// The arc he has up is the launcher's, and too short to arm.
pub fn preview_too_close(g: &crate::game::Game, pl: PlayerId) -> bool {
    let r = g.pl[pl].rifle;
    match &g.sets[crate::seats::seat_of_man(g, pl)].hand.throw_preview {
        Some(sol) => crate::game::player_rifle_up(g, pl) && crate::weapons::rifle_attachment(g, r) == Some(crate::weapons::RifleAttachment::He40) && !arc_armed(g, sol),
        None => false,
    }
}

/// The living man under the reticle a round can meet, never one of Sam's own side.
fn shell_man(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<GuardId> {
    match seat.aim_guard {
        Some(id) if !crate::condition::down(g, g.gd[id].ch) && !g.team(id).ours() => Some(id),
        _ => None,
    }
}

/// The living bodies a round stops on, as far as Sam can see them.
pub fn shell_bodies(g: &crate::game::Game) -> Vec<CharId> {
    let mut out: Vec<CharId> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let ch = g.gd[gd].ch;
        if !crate::condition::down(g, g.gd[gd].ch)
            && !g.team(gd).ours()
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

/// Where a round fired now would go, cut where it first enters one of `bodies`.
pub fn shell_solve(g: &crate::game::Game, from: V3, target: V3, bodies: Option<&[CharId]>) -> crate::throwables::ThrowSolution {
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

/// The point to send a round at: the chest of the man under the reticle, led; else the fixture; else the cursor's point.
pub fn shell_aim_point(g: &crate::game::Game, seat: &crate::seats::SeatRef, from: V3) -> V3 {
    if let Some(gd) = shell_man(g, seat) {
        let c = g.gd[gd].ch;
        let bone = g.ch[c].bone(crate::character::Bone::Chest);
        let chest = v3::add(bone.unwrap_or_else(|| v3::add(g.ch[c].pos, [0.0, 1.2, 0.0])), [0.0, 0.0, 0.0]);
        let tof = v3::dist(from, chest) / (g.k.he40.speed * 0.97);
        let vel = g.ch[c].vel;
        return v3::mad(chest, [vel[0], 0.0, vel[2]], tof);
    }
    if let Some(t) = seat.aim_target {
        return v3::copy(g.tab.fixture[t].pos);
    }
    let ap = seat.aim_point;
    [ap[0], ap[1] + 0.03, ap[2]]
}

/// Lets one round go off `shooter`'s launcher: in the air from this frame, the thump, and the room hears it.
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
    let level = g.k.he40.launch_noise;
    crate::flight::launched(g, shooter, from, 1.0, Some(0.6), level);
    sh
}

/// Keeps a round's end in the capped QA log.
fn log_end(g: &mut crate::game::Game, e: &crate::he40::ShellEnd) {
    g.m.he40.he40_log.push(e.clone());
    if g.m.he40.he40_log.len() > 32 {
        g.m.he40.he40_log.remove(0);
    }
}

/// Armed and stopped: a grenade's blast, judged from `pt`.
fn burst(g: &mut crate::game::Game, sh: crate::he40::Shell, pt: V3, met: crate::he40::ShellEndMet) {
    let spec = crate::frag::BlastSpec {
        who: g.ch[sh.shooter].id,
        name: Some(String::from("40 mm grenade")),
        ..Default::default()
    };
    let lift = g.k.frag.lift;
    crate::frag::blast(g, [pt[0], pt[1] - lift, pt[2]], &spec);
    log_end(g, &crate::he40::ShellEnd { flown: sh.flown, armed: true, met });
}

/// Not armed and stopped: a 'shell' item with what motion is left, and a noise the room hears.
fn dud(g: &mut crate::game::Game, sh: crate::he40::Shell, at: V3, vel: V3, met: crate::he40::ShellEndMet) {
    let most = g.k.he40.spent_speed_max;
    crate::flight::to_item(g, crate::throwables::ItemKind::Shell, sh.shooter, at, vel, most, crate::throwables::ItemBounce::ShellSkitter);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::CanisterBounce, Some(at), Some(0.8), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.7 }));
    crate::particle_fx::clack_fx(g, at);
    let level = g.k.he40.dud_noise;
    if met != ShellEndMet::Man { crate::flight::noise(g, [at[0], 0.0, at[2]], level, 0.0, None); }   // (a man is its noise)
    g.stats.he40.duds += 1;
    log_end(g, &crate::he40::ShellEnd { flown: sh.flown, armed: false, met });
}

/// Is the fuze armed `t` metres on from what has been flown.
fn armed_at(flown: f64, t: f64, arm_m: f64) -> bool {
    flown + m::max(0.0, t) >= arm_m
}

/// One frame of every round in the air: armed, it bursts on what it meets; before that it is a dud.
pub fn update_shells(g: &mut crate::game::Game, dt: f64) {
    let r_k = g.k.he40;
    let b = ballistics(g);
    let h = dt / b.substeps;
    let off = |v: V3, n: V3| crate::flight::bounce(v, n, r_k.restitution, r_k.keep_tangent);
    let mut i = g.shells.len();
    while i > 0 {
        i -= 1;
        let mut done = false;
        let mut s = 0.0;
        while s < b.substeps && !done {
            s += 1.0;
            g.shells[i].age += h;
            g.shells[i].spin += 40.0 * h;
            let Shell { pos, mut vel, shooter, .. } = g.shells[i];
            let bodies = crate::flight::bodies(g, shooter);
            let leg = crate::flight::leg(g, &b, h, pos, &mut vel, &bodies);
            g.shells[i].vel = vel;
            let Some(leg) = leg else { continue };
            let Some(met) = leg.met else {
                g.shells[i].pos = leg.to;
                g.shells[i].flown += leg.len;
                continue;
            };
            done = true;
            // (its path is counted to where its centre stops, as the preview's is)
            let was = g.shells[i].flown;
            g.shells[i].flown += match &met {
                Met::Man(_, t) => *t,
                Met::Prop(ph) => m::max(0.0, ph.t - b.radius),
                Met::Static(st) => m::max(0.0, st.t - b.radius),
                Met::Floor => leg.len,
            };
            if let Met::Floor = met { g.shells[i].pos = leg.to; }
            let sh = g.shells[i];
            let armed = match &met { Met::Man(_, t) => armed_at(was, *t, r_k.arm_m), _ => armed_at(sh.flown, 0.0, r_k.arm_m) };
            match met {
                Met::Man(c, t) => {
                    let at = v3::mad(pos, leg.dir, t);
                    if armed { burst(g, sh, v3::mad(at, leg.dir, -r_k.standoff), ShellEndMet::Man); } else {
                        crate::flight::struck(g, c, leg.dir, shooter);
                        dud(g, sh, v3::mad(at, leg.dir, -0.06), [-leg.dir[0] * 1.2, 0.8, -leg.dir[2] * 1.2], ShellEndMet::Man);
                    }
                }
                Met::Prop(ph) => {
                    let hit_p = v3::mad(pos, leg.dir, ph.t);
                    if armed {
                        burst(g, sh, v3::mad(hit_p, ph.n, r_k.standoff), ShellEndMet::Prop);
                    } else {
                        let who = g.ch[shooter].id;
                        crate::props::prop_system_apply_impulse(g, ph.prop, hit_p, [leg.dir[0] * r_k.prop_j, 0.0, leg.dir[2] * r_k.prop_j], who, Some(true));
                        crate::marks::chip_mark(g, hit_p, ph.n, Some(crate::marks::Surface2 { box_: ph.box_, dynamic: true }), None);
                        dud(g, sh, v3::mad(pos, leg.dir, m::max(0.0, ph.t - b.radius)), off(sh.vel, ph.n), ShellEndMet::Prop);
                    }
                }
                Met::Static(st) => {
                    // a wall, a slab, a leaf, a pane
                    let hit_p = v3::mad(pos, leg.dir, st.t);
                    if armed {
                        burst(g, sh, v3::mad(hit_p, st.n, r_k.standoff), ShellEndMet::Static);
                    } else {
                        let surface = crate::marks::static_surface(g, st.index);
                        crate::marks::chip_mark(g, hit_p, st.n, surface, None);
                        dud(g, sh, v3::mad(pos, leg.dir, m::max(0.0, st.t - b.radius)), off(sh.vel, st.n), ShellEndMet::Static);
                    }
                }
                Met::Floor => {
                    let at: V3 = [leg.to[0], b.radius, leg.to[2]];
                    if armed { burst(g, sh, [at[0], r_k.standoff, at[2]], ShellEndMet::Floor); } else { dud(g, sh, at, off(sh.vel, [0.0, 1.0, 0.0]), ShellEndMet::Floor); }
                }
            }
        }
        if !done && g.shells[i].age > b.life {
            // up and out over the lot: gone
            let sh = g.shells[i];
            log_end(g, &ShellEnd { flown: sh.flown, armed: sh.flown >= r_k.arm_m, met: ShellEndMet::Air });
            done = true;
        }
        if done && i < g.shells.len() {
            g.shells.remove(i);
        }
    }
}

/// Rounds in flight as boxes: the stubby body and its gold nose along the line of flight.
pub fn shell_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let flags = (crate::scene_boxes::box_flag::DYNAMIC as i32 | crate::scene_boxes::box_flag::NO_SHADOW as i32) as f64;
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
    use alloc::vec;
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
