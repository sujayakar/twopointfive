//! src/game/launcher.ts lines 1 to 253
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A table of the ring's as a Ballistics (launchVel and arcSolve read only these numbers).
fn ballistics_of(k: &crate::knobs::RingK) -> Ballistics {
    Ballistics { speed: k.speed, gravity: k.gravity, drag: k.drag, radius: k.radius, life: k.life, substeps: k.substeps, solve_iters: k.solve_iters }
}

/// g.audio.play(name, at, gain, { rate }): the only sound here the skeleton cannot say yet.
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate, crate::Opaque holds nothing (launcher.ts:168,172,185,187,192,208)") }

/// A 'prop' noise in the room's business, dealt its serial here at its birth.
fn prop_event(g: &mut Game, pos: V3, level: f64, who: f64, lure: Option<bool>) {
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Prop, pos, time, loud: false, level: Some(level), who: Some(who), lure, serial, ..crate::game::GameEvent::zeroed() });
}

/// The launch velocity to pass through `target`: dead at it, raised by however far a simulated flight falls short, solveIters times. (launcher.ts launchVel)
pub fn launch_vel(from: V3, target: V3, r: Option<crate::launcher::Ballistics>) -> V3 {
    let r = match r { Some(r) => r, None => ballistics_of(&crate::knobs::page_ring()) };
    let d = v3::sub(target, from);
    let l = v3::len(d);
    if l < 1e-3 { return [0.0, 0.0, r.speed]; }
    let mut aim = v3::copy(target);
    let mut v0: V3 = v3::scale(v3::normalize(d), r.speed);
    let mut it = 0.0;
    while it < r.solve_iters {
        v0 = v3::scale(v3::normalize(v3::sub(aim, from)), r.speed);
        let hd = m::hyp2(d[0], d[2]);
        let mut p = v3::copy(from);
        let mut v = v3::copy(v0);
        let h = 1.0 / 240.0;
        let mut y_at = m::NAN;
        let mut i = 0.0;
        while i < r.life / h {
            let k = m::exp(-r.drag * h);
            v = [v[0] * k, v[1] * k - r.gravity * h, v[2] * k];
            let np: V3 = v3::mad(p, v, h);
            let a = m::hyp2(p[0] - from[0], p[2] - from[2]);
            let b = m::hyp2(np[0] - from[0], np[2] - from[2]);
            if b >= hd {
                let u = if b > a { (hd - a) / (b - a) } else { 1.0 };
                y_at = p[1] + (np[1] - p[1]) * u;
                break;
            }
            p = np;
            i += 1.0;
        }
        if !y_at.is_finite() { break; }
        aim = [aim[0], aim[1] + (target[1] - y_at), aim[2]];
        it += 1.0;
    }
    v0
}

/// Where a ring fired now would go, for the preview. (launcher.ts ringSolve)
pub fn ring_solve(g: &mut crate::game::Game, from: V3, target: V3) -> crate::throwables::ThrowSolution {
    let r = ballistics_of(&g.k.ring);
    arc_solve(g, from, target, r)
}

/// ringSolve for anything the tube throws: stepped as a live ring at 60 Hz, until it meets a static, a leaf, a prop or the floor. (launcher.ts arcSolve)
pub fn arc_solve(g: &mut crate::game::Game, from: V3, target: V3, r: crate::launcher::Ballistics) -> crate::throwables::ThrowSolution {
    let v0 = launch_vel(from, target, Some(r));
    let mut points: Vec<V3> = Vec::new();
    let mut p = v3::copy(from);
    let mut v = v3::copy(v0);
    let mut land = v3::copy(target);
    let h = 1.0 / 60.0 / r.substeps;
    let mut t_end = 0.0;
    let mut i = 0.0;
    while i * h < r.life {
        if js::rem(i, r.substeps) == 0.0 { points.push(v3::copy(p)); }
        let k = m::exp(-r.drag * h);
        v = [v[0] * k, v[1] * k - r.gravity * h, v[2] * k];
        let np = v3::mad(p, v, h);
        let seg = v3::sub(np, p);
        let len = v3::len(seg);
        t_end = i * h;
        if len > 1e-6 {
            let dir = v3::scale(seg, 1.0 / len);
            let reach = len + r.radius;
            let st = crate::scene_collision::raycast(g, p, dir, reach);
            let ph = crate::props::prop_system_raycast(g, p, dir, match st { Some(s) => s.t, None => reach });
            let t = match (ph, st) { (Some(ph), _) => ph.t, (None, Some(st)) => st.t, (None, None) => -1.0 };
            if t >= 0.0 {
                land = v3::mad(p, dir, m::max(0.0, t - r.radius));
                points.push(land);
                break;
            }
        }
        if np[1] < r.radius {
            land = [np[0], r.radius, np[2]];
            points.push(land);
            break;
        }
        p = np;
        land = v3::copy(np);
        i += 1.0;
    }
    crate::throwables::ThrowSolution { v0, t: t_end, points, land }
}

/// The point to send a ring at: the head of the man under the reticle, led by his ground velocity; else the fixture or world point the cursor names. (launcher.ts ringAimPoint)
pub fn ring_aim_point(g: &mut crate::game::Game, from: V3) -> V3 {
    let puppet = g.puppet.as_ref().and_then(|p| p.target);
    let gd = match puppet {
        Some(t) if g.gd[t].state != crate::game::GuardState::Dead => Some(t),
        _ => g.aim_guard,
    };
    if let Some(gd) = gd {
        let c = g.gd[gd].ch;
        let head_bone = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]) };
        let head = v3::add(head_bone, [0.0, 0.04, 0.0]);
        let tof = v3::dist(from, head) / (g.k.ring.speed * 0.9);
        let vel = g.ch[c].vel;
        return v3::mad(head, [vel[0], 0.0, vel[2]], tof);
    }
    if let Some(t) = g.aim_target { return v3::copy(g.tab.sim_lightshome_fixture[t].pos); }
    let ap = g.aim_point;
    [ap[0], ap[1] + 0.03, ap[2]]
}

/// Let one ring go: in the air from this frame, the launcher's sound at the tube, a 'prop' noise at the shooter. (launcher.ts fireRing)
pub fn fire_ring(g: &mut crate::game::Game, shooter: CharId, from: V3, target: V3) -> crate::launcher::Ring {
    let vel = launch_vel(from, target, Some(ballistics_of(&g.k.ring)));
    let spin = crate::math_dice::sim_random(g) * m::PI;
    let ring = Ring { pos: v3::copy(from), vel, age: 0.0, shooter, spin };
    g.rings.push(ring);
    crate::audio_iface::game_audio_like_play(g, "launcher", Some(from), Some(0.8), None);
    let (pos, level, who) = (v3::copy(g.ch[shooter].pos), g.k.ring.launch_noise, g.ch[shooter].id);
    prop_event(g, pos, level, who, None);
    let pl = g.player;
    if shooter == g.pl[pl].ch { g.pl[pl].noise = m::max(g.pl[pl].noise, level); }
    ring
}

/// tryChar of updateRings: the nearest living man along this substep, other than the shooter. (launcher.ts:142-143)
fn try_char(g: &mut Game, r: &Ring, radius: f64, dir: V3, reach: f64, c: CharId, gd: Option<GuardId>, best: &mut f64, hit_c: &mut Option<CharId>, hit_g: &mut Option<GuardId>) {
    if c == r.shooter || !g.ch[c].alive { return; }
    let t = crate::combat::body_hit(g, c, r.pos, dir, reach, Some(crate::combat::BodyHitContact::Flight { radius }));
    if t >= 0.0 && t < *best {
        *best = t;
        *hit_c = Some(c);
        *hit_g = gd;
    }
}

/// One frame of every ring in the air: drag and gravity in substeps, and along each substep's segment the first man, static, leaf, prop or floor it meets. (launcher.ts updateRings)
pub fn update_rings(g: &mut crate::game::Game, dt: f64) {
    let mut i = g.rings.len();
    while i > 0 {
        i -= 1;
        let mut r = g.rings[i];
        let mut done = false;
        let (substeps, drag, gravity, radius, life, prop_j) = (g.k.ring.substeps, g.k.ring.drag, g.k.ring.gravity, g.k.ring.radius, g.k.ring.life, g.k.ring.prop_j);
        let h = dt / substeps;
        let mut s = 0.0;
        while s < substeps && !done {
            s += 1.0;
            r.age += h;
            r.spin += 25.0 * h;
            let k = m::exp(-drag * h);
            r.vel = [r.vel[0] * k, r.vel[1] * k - gravity * h, r.vel[2] * k];
            let seg = v3::scale(r.vel, h);
            let len = v3::len(seg);
            if len < 1e-6 { continue; }
            let dir = v3::scale(seg, 1.0 / len);
            let reach = len + radius;
            let mut best = m::INF;
            let mut hit_c: Option<CharId> = None;
            let mut hit_g: Option<GuardId> = None;
            let mut j = 0;
            while j < g.guards.len() {
                let gd = g.guards[j];
                j += 1;
                if g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].team != crate::teams::Team::Operative {
                    let c = g.gd[gd].ch;
                    try_char(g, &r, radius, dir, reach, c, Some(gd), &mut best, &mut hit_c, &mut hit_g);
                }
            }
            for cv in crate::civilians::civilian_bodies(g) {
                let c = g.gd[cv].ch;
                try_char(g, &r, radius, dir, reach, c, Some(cv), &mut best, &mut hit_c, &mut hit_g);
            }
            let pl = g.player;
            if !g.pl[pl].down {
                let c = g.pl[pl].ch;
                try_char(g, &r, radius, dir, reach, c, None, &mut best, &mut hit_c, &mut hit_g);
            }
            let st = crate::scene_collision::raycast(g, r.pos, dir, reach);
            let ph = crate::props::prop_system_raycast(g, r.pos, dir, reach);
            let t_s = match st { Some(s) => s.t, None => m::INF };
            let t_p = match ph { Some(p) => p.t, None => m::INF };
            if let Some(c) = hit_c {
                if best <= t_s && best <= t_p {
                    ring_hits_character(g, r, c, hit_g, dir, v3::mad(r.pos, dir, best));
                    done = true;
                    break;
                }
            }
            if let Some(ph) = ph {
                if t_p <= t_s {
                    let at = v3::mad(r.pos, dir, m::max(0.0, t_p - radius));
                    let who = g.ch[r.shooter].id;
                    crate::props::prop_system_apply_impulse(g, ph.prop, at, [dir[0] * prop_j, 0.0, dir[2] * prop_j], who, Some(true));
                    clack(g, at, None);
                    crate::marks::chip_mark(g, v3::mad(r.pos, dir, t_p), ph.n, Some(crate::marks::Surface2 { box_: ph.box_, dynamic: true }), None);
                    let back = bounce_off(g, r.vel, ph.n);
                    spend(g, r, at, back);
                    done = true;
                    break;
                }
            }
            if let Some(st) = st {
                let at = v3::mad(r.pos, dir, m::max(0.0, t_s - radius));
                clack(g, at, None);
                let surface = crate::marks::static_surface(g, st.index);
                crate::marks::chip_mark(g, v3::mad(r.pos, dir, t_s), st.n, surface, None);
                let back = bounce_off(g, r.vel, st.n);
                spend(g, r, at, back);
                done = true;
                break;
            }
            r.pos = v3::add(r.pos, seg);
            if r.pos[1] < radius {
                let at: V3 = [r.pos[0], radius, r.pos[2]];
                clack(g, at, Some(0.6));
                let back = bounce_off(g, r.vel, [0.0, 1.0, 0.0]);
                spend(g, r, at, back);
                done = true;
                break;
            }
        }
        if !done && r.age > life {
            spend(g, r, r.pos, v3::scale(r.vel, 0.3));
            done = true;
        }
        // JavaScript's splice past the end is nothing (a restart under this walk empties g.rings); the row is the walked copy until here
        if i < g.rings.len() {
            if done { g.rings.remove(i); } else { g.rings[i] = r; }
        }
    }
}

/// Velocity coming back off a surface: the normal share reversed × restitution, the slide × keepTangent. (launcher.ts bounceOff)
pub fn bounce_off(g: &mut crate::game::Game, v: V3, n: V3) -> V3 {
    let vn = v3::dot(v, n);
    let v_n = v3::scale(n, vn);
    let v_t = v3::sub(v, v_n);
    v3::add(v3::scale(v_t, g.k.ring.keep_tangent), v3::scale(v_n, if vn < 0.0 { -g.k.ring.restitution } else { 1.0 }))
}

/// The ring's flight is over at `at`: from here it is a 'ring' item with its motion capped. (launcher.ts spend)
pub fn spend(g: &mut crate::game::Game, r: crate::launcher::Ring, at: V3, vel: V3) {
    let mut vel = vel;
    let sp = v3::len(vel);
    if sp > g.k.ring.spent_speed_max { vel = v3::scale(vel, g.k.ring.spent_speed_max / sp); }
    let who = g.ch[r.shooter].id;
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Ring, v3::copy(at), vel, Some(crate::throwables::ThrowablesSpawnOpts { who: Some(who), heading: Some(v3::yaw_of(vel)), ..crate::throwables::ThrowablesSpawnOpts::zeroed() }));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::RingSkitter);
}

/// The ring meeting something hard: a clack, sparks, and a noise the room goes to look at. (launcher.ts clack)
pub fn clack(g: &mut crate::game::Game, at: V3, k: Option<f64>) {
    let k = k.unwrap_or(1.0);
    seam_audio_play_rate(g, "canisterBounce", at, 0.55 * k, 1.5);
    crate::particle_fx::clack_fx(g, at);
    let level = g.k.ring.clack_noise * k;
    prop_event(g, [at[0], 0.0, at[2]], level, 0.0, Some(true));
}

/// A ring has found a man at `at`: high is out cold, low the stagger, a civilian low a fright, the player a flinch; the ring drops at his feet. (launcher.ts ringHitsCharacter)
pub fn ring_hits_character(g: &mut crate::game::Game, r: crate::launcher::Ring, c_2: CharId, gd: Option<GuardId>, dir: V3, at: V3) {
    let c = c_2;
    let shooter_id = g.ch[r.shooter].id;
    let by_player = crate::seats::is_man_id(g, shooter_id);
    let neck = match g.ch[c].bones.get(crate::character::Bone::Neck) { Some(b) => *b, None => v3::add(g.ch[c].pos, [0.0, 1.42, 0.0]) };
    let high = at[1] >= neck[1] - g.k.ring.ko_below_neck;
    match gd {
        None => {
            crate::character::anim_hit(g, c);
            seam_audio_play_rate(g, "bodyHit", at, 0.5, 1.1);
            let text = crate::gear::gear_text(g, if high { "a ring airfoil off the side of your head — from a friend, luckily" } else { "a ring airfoil in the ribs" });
            crate::game::game_msg(g, &text);
        }
        Some(gd) if g.gd[gd].civ.is_some() && !high => {
            crate::character::anim_hit(g, c);
            let thump = v3::add(g.ch[c].pos, [0.0, 1.1, 0.0]);
            seam_audio_play_rate(g, "bodyHit", thump, 0.55, 1.05);
            if g.gd[gd].held.is_none() {
                let back = g.k.ring.stagger_back;
                let p = g.ch[c].pos;
                crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Noise, [p[0] - dir[0] * back, 0.0, p[2] - dir[2] * back], false);
            }
            let (pos, level) = (v3::copy(g.ch[c].pos), g.k.ring.hit_noise);
            prop_event(g, pos, level, shooter_id, None);
            if by_player { let pl = g.player; g.pl[pl].noise = m::max(g.pl[pl].noise, level); }
        }
        Some(gd) if high => {
            seam_audio_play_rate(g, "bodyHit", at, 0.6, 0.75);
            let line = match g.gd[gd].civ {
                Some(cv) => format!("ring to the head — {}, the {}, is out cold. breathing — but a civilian: that costs, and they may find {}", g.cv[cv].name, g.cv[cv].label, g.cv[cv].her),
                None => format!("ring to the head — {} is out cold", g.gd[gd].callsign),
            };
            crate::combat::kill_guard(g, gd, [dir[0], 0.0, dir[2]], Some(true), Some(crate::game::DownKind::Struck), Some(if by_player { crate::game::DownBy::Player } else { crate::game::DownBy::Guard }), Some(&line), None);
            let (pos, level) = (v3::copy(g.ch[c].pos), g.k.ring.ko_noise);
            prop_event(g, pos, level, shooter_id, None);
            if by_player { let pl = g.player; g.pl[pl].noise = m::max(g.pl[pl].noise, level); }
        }
        Some(gd) => stagger_guard(g, gd, dir, r.shooter),
    }
    let drop_at = v3::mad(at, dir, -0.05);
    let vx = -dir[0] * 1.5 + (crate::math_dice::sim_random(g) - 0.5);
    let vy = 1.0 + crate::math_dice::sim_random(g);
    let vz = -dir[2] * 1.5 + (crate::math_dice::sim_random(g) - 0.5);
    spend(g, r, drop_at, [vx, vy, vz]);
}

/// A ring in the body: he doubles over, rooted, gun down, alert, with a fix back up the ring's line; a yelp and a noise at him. A man held only flinches. (launcher.ts staggerGuard)
pub fn stagger_guard(g: &mut crate::game::Game, gd: GuardId, dir: V3, shooter: CharId) {
    use crate::game::GuardState;
    if g.gd[gd].state == GuardState::Dead { return; }
    let c = g.gd[gd].ch;
    crate::character::anim_hit(g, c);
    let thump = v3::add(g.ch[c].pos, [0.0, 1.1, 0.0]);
    seam_audio_play_rate(g, "bodyHit", thump, 0.55, 1.05);
    if g.gd[gd].held.is_some() { return; }
    if g.gd[gd].script.is_some() {
        use crate::squad::EndGuardScriptState as End;
        let to = match g.gd[gd].state { GuardState::Patrol => End::Suspicious, GuardState::Search => End::Search, GuardState::Alert => End::Alert, _ => End::Suspicious };
        crate::squad::end_guard_script(g, gd, Some(to));
    }
    let (stagger, fire_cd, aware, back_m, hit_noise) = (g.k.ring.stagger, g.k.ring.stagger_fire_cd, g.k.ring.stagger_aware, g.k.ring.stagger_back, g.k.ring.hit_noise);
    g.gd[gd].stagger_until = m::max(g.gd[gd].stagger_until, g.time + stagger);
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, stagger);
    g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, fire_cd);
    let weapon = g.gd[gd].weapon;
    g.fa[weapon].burst = None;
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    g.gd[gd].stall_ref = None;
    g.gd[gd].speed = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    if g.gd[gd].state != GuardState::Alert || g.gd[gd].last_known.is_none() {
        let p = g.ch[c].pos;
        let mut back: V3 = [p[0] - dir[0] * back_m, 0.0, p[2] - dir[2] * back_m];
        if crate::scene_collision::nav_grid_is_blocked(g, back[0], back[2]) {
            back = match crate::scene_collision::nav_grid_nearest_free_point(g, back[0], back[2]) { Some(q) => q, None => v3::copy(g.ch[c].pos) };
        }
        g.gd[gd].last_known = Some(back);
    }
    g.gd[gd].awareness = m::max(g.gd[gd].awareness, aware);
    if g.ai_enabled { g.gd[gd].witness.heard.small += 1.0; }
    let all = stagger_lines_table();
    let now: Vec<String> = if g.gd[gd].state == GuardState::Alert { all.iter().filter(|t| !t.contains("who's there")).cloned().collect() } else { all.clone() };
    let pool = crate::words::w(g, &now, &all);
    let roll = m::floor(crate::math_dice::sim_random(g) * pool.len() as f64);
    let line = &pool[js::rem(roll, pool.len() as f64) as usize];
    crate::game::game_say(g, gd, line, None, None, None);
    let shooter_id = g.ch[shooter].id;
    let pos = v3::copy(g.ch[c].pos);
    prop_event(g, pos, hit_noise, shooter_id, None);
    if let Some(pl) = crate::seats::man_by_id(g, shooter_id) { g.pl[pl].noise = m::max(g.pl[pl].noise, hit_noise); }
}

/// The stagger lines. (launcher.ts staggerLines)
pub fn stagger_lines() -> Vec<String> {
    stagger_lines_table()
}

/// Rings in flight as boxes. Picture-only (triage: dress): not translated, the skeleton's stub. (launcher.ts ringBoxes)
pub fn ring_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) { todo!("picture-only: left to the picture's side (conventions 6)") }

/// The panel's line of rings. Host only (triage: host): not translated, the skeleton's stub. (launcher.ts ringSummary)
pub fn ring_summary(g: &mut crate::game::Game) -> String { todo!("host only: panel / QA text (conventions 6)") }

#[cfg(test)]
mod tests {
    use super::*;

    const R: Ballistics = Ballistics { speed: 30.0, gravity: 9.81, drag: 0.4, radius: 0.04, life: 1.5, substeps: 4.0, solve_iters: 2.0 };

    fn same(from: V3, target: V3, r: Ballistics, bits: [u64; 3]) {
        let v = launch_vel(from, target, Some(r));
        assert_eq!([v[0].to_bits(), v[1].to_bits(), v[2].to_bits()], bits, "{:?} to {:?}", from, target);
    }

    /// launchVel against the TypeScript's own bits (bun, EXACT.on): a plain shot, a long downhill one, one past the ring's life, one under the millimetre, straight up, a table of its own.
    #[test]
    fn launch_vel_is_the_typescripts() {
        same([0.0, 1.5, 0.0], [10.0, 1.6, 7.0], R, [0x40387db5187982bb, 0x40040cc55c3628b9, 0x403124cb9121db84]);
        same([3.25, 1.2, -8.5], [-14.75, 0.3, 22.125], R, [0xc02e649b97715293, 0xbfe8507c79277542, 0x4039daeb78d4d8e9]);
        same([0.0, 2.0, 0.0], [80.0, 1.7, 3.0], R, [0x403dfa8d42f4c8e6, 0xbfbcc791d9e0c0de, 0x3ff1fcbb282c788a]);
        same([1.0, 1.0, 1.0], [1.0, 1.0, 1.0005], R, [0x0000000000000000, 0x0000000000000000, 0x403e000000000000]);
        same([0.0, 1.0, 0.0], [0.0, 6.0, 0.0], R, [0x0000000000000000, 0x403e000000000000, 0x0000000000000000]);
        same([0.5, 1.4, 0.25], [20.0, 3.0, -15.0], Ballistics { speed: 76.0, drag: 0.12, solve_iters: 3.0, ..R }, [0x404dd24d263a8c2f, 0x401a3fdaaad96f33, 0xc047526a48903faf]);
    }

    #[test]
    fn stagger_lines_are_the_five() {
        let l = stagger_lines();
        assert_eq!(l.len(), 5);
        assert_eq!(l.iter().filter(|t| t.contains("who's there")).count(), 1);
    }
}
