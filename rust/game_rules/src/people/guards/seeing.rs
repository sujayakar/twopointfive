use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

fn hunting(g: &Game, gd: GuardId) -> bool {
    let st = g.gd[gd].state;
    st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search
}

pub(crate) fn shot_e(e: &crate::game::GameEvent) -> crate::guards::ShotRelationE {
    crate::guards::ShotRelationE { kind: e.kind, who: e.who }
}

/// How a 'glass' event reaches an ear: from its own point, and from each face of the pane it carries, whichever arrives best.
pub(in crate::people) fn glass_heard(g: &mut crate::game::Game, e: crate::game::GameEvent, ear: V3) -> crate::scene_collision::Propagation {
    let mut best = crate::weather::hear_path(g, [e.pos[0], e.pos[1] + 1.0, e.pos[2]], ear, None);
    let ears = match e.ears {
        Some(q) if !best.direct => q,
        _ => return best,
    };
    let hear_m = g.k.glass.hear_m;
    let score = |p: &crate::scene_collision::Propagation| -> f64 {
        if p.path_len < hear_m { m::min(1.0, m::max(0.15, 1.0 - p.path_len / hear_m)) * p.carry } else { 0.0 }
    };
    let mut s = score(&best);
    for q in ears {
        let p = crate::weather::hear_path(g, [q[0], q[1] + 1.0, q[2]], ear, None);
        let k = score(&p);
        if k > s {
            s = k;
            best = p;
        }
    }
    best
}

/// His eyes have a man at `at` this moment, by another rule's look than updateGuard's own: the record of a sighting, in one place.
pub fn eyes_on(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    crate::mind::fix(g, gd, Some(at), "guards");
    let now = g.time;
    let man = &mut g.gd[gd];
    man.sight_pos = at;
    man.sight_t = now;
    man.last_seen_t = now;
}

/// Gunfire filed and not through this man's ears yet that hearingCheck will count when it next runs for him; `bangs` counts a stun charge or a door blown too.
pub fn gunfire_due(g: &crate::game::Game, gd: GuardId, bangs: Option<bool>) -> bool {
    use crate::director::EventKind as K;
    let bangs = bangs.unwrap_or(false);
    let c = g.gd[gd].ch;
    let mut i = 0;
    while i < g.events.len() {
        let e = g.events[i];
        i += 1;
        if let Some(id) = e.id {
            if id <= g.gd[gd].heard_up_to {
                continue;
            }
        }
        if bangs && e.kind == K::Bang {
            if v3::dist(e.pos, g.ch[c].pos) <= 80.0 {
                return true;
            }
            continue;
        }
        if !((e.kind == K::Shot || e.kind == K::GuardShot) && e.level.unwrap_or(0.0) >= 1.0) {
            continue;
        }
        let d = v3::dist(e.pos, g.ch[c].pos);
        if d > 80.0 || (e.kind == K::GuardShot && d < 0.1) {
            continue;
        }
        if crate::guards::shot_relation(g, gd, shot_e(&e)) != crate::guards::ShotRelation::None {
            return true;
        }
    }
    false
}

/// Whose gunfire this is to `gd`: a man of his side's, an enemy's, or neither.
pub(super) fn shot_relation(g: &crate::game::Game, gd: GuardId, e: crate::guards::ShotRelationE) -> crate::guards::ShotRelation {
    use crate::guards::ShotRelation;
    let team = g.team(gd);
    if e.kind == crate::director::EventKind::Shot { return if team.theirs() { ShotRelation::Threat } else { ShotRelation::None }; }   // (a player's round is no news to his squad)
    let shooter = crate::teams::team_of_id(g, e.who.unwrap_or(-1.0));
    if shooter == team { ShotRelation::Own } else if team.foe_of(shooter) { ShotRelation::Threat } else { ShotRelation::None }
}

/// Where a colleague's shots say the enemy is: the shooter's own idea of his man at its own age; none when the hearer keeps what he has.
pub(super) fn colleague_fire_pos(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::ColleagueFirePosE) -> Option<V3> {
    if e.blind == Some(true) {
        let p = match e.to {
            Some(t) => t,
            None => e.pos,
        };
        return Some([p[0], 0.0, p[2]]);
    }
    let mut sh: Option<GuardId> = None;
    if let Some(who) = e.who {
        for i in 0..g.guards.len() {
            let x = g.guards[i];
            if g.char_id(x) == who {
                sh = Some(x);
                break;
            }
        }
    }
    let player_pos = g.player_pos(g.player);
    if let Some(s) = sh {
        let who = g.gd[s].foe.or_else(|| if g.team(s) == crate::teams::Team::Guard { Some(g.pl[g.player].ch) } else { None });
        if let Some(who) = who {
            if let Some(i) = crate::picture::idea_of(g, s, who) {
                let mine = crate::picture::idea_of(g, gd, who);
                let own = match &mine {
                    Some(mi) => mi.t >= i.t,
                    None => false,
                };
                if g.gd[gd].last_known.is_some() && (i.lost || own || g.gd[gd].fix_t >= i.t) {
                    return None;
                }
                let w = if own { mine.unwrap() } else { i };
                if own {
                } else if !i.lost {
                    let age = g.time - i.t;
                    crate::picture::picture_tell(g, gd, who, i.at, Some(age));
                }
                g.gd[gd].fix_ref = Some([w.at[0], 0.0, w.at[2]]);
                g.gd[gd].fix_t = w.t;
                return Some([w.at[0], 0.0, w.at[2]]);
            }
        }
    }
    let s = match sh {
        Some(s) if g.team(s) != crate::teams::Team::Guard => s,
        _ => return Some(player_pos),
    };
    if let Some(fc) = g.gd[s].foe {
        return Some(g.ch[fc].pos);
    }
    Some(match e.to {
        Some(t) => t,
        None => e.pos,
    })
}

/// Is this footfall an enemy's: Sam's steps carry no `who`; a unit's carry his id.
pub(super) fn step_is_threat(g: &crate::game::Game, gd: GuardId, e: crate::guards::StepIsThreatE) -> bool {
    let who = e.who.unwrap_or(crate::consts::PLAYER_ID);
    let team = g.team(gd);
    if crate::seats::is_man_id(g, who) {
        team.theirs()
    } else {
        let of = crate::teams::team_of_id(g, who);
        team.foe_of(of)
    }
}

/// Sam's velocity over the ground this frame, as anybody watching sees it: his root's displacement since the frame before, capped.
pub fn player_ground_vel(g: &mut crate::game::Game) -> V3 {
    let view = crate::seats::view_key(g, &crate::seats::here(g));
    let p = g.player_pos(g.player);
    let now = g.time;
    let Some(row) = g.m.guards.ground_track.get_mut(view) else {
        let vel: V3 = [0.0, 0.0, 0.0];
        g.m.guards.ground_track.set(view, crate::guards::GuardsModGroundTrackValue { pos: p, t: now, vel });
        return vel;
    };
    ground_step(row, p, now);
    row.vel
}

/// A new frame's reading of the track: the displacement over the time since, capped at 6 m/s; a teleport or a long gap reads as still.
fn ground_step(row: &mut crate::guards::GuardsModGroundTrackValue, p: V3, now: f64) {
    if now != row.t {
        let since = now - row.t;
        if since > 1e-4 && since < 0.25 {
            let vx = (p[0] - row.pos[0]) / since;
            let vz = (p[2] - row.pos[2]) / since;
            let sp = m::hyp2(vx, vz);
            let k = if sp > 6.0 { 6.0 / sp } else { 1.0 };
            row.vel = [vx * k, 0.0, vz * k];
        } else {
            row.vel = [0.0, 0.0, 0.0];
        }
        row.pos = p;
        row.t = now;
    }
}

/// motionCore's sum, off the eye's place and whether he is hunting.
fn motion_sum(gp: V3, hunts: bool, pp: V3, v: V3, lit: f64, dist: f64) -> f64 {
    let mut rx = gp[0] - pp[0];
    let mut rz = gp[2] - pp[2];
    let rl = js::or(m::hyp2(rx, rz), 1.0);
    rx /= rl;
    rz /= rl;
    let vr = m::abs(v[0] * rx + v[2] * rz);
    let vt = m::abs(v[0] * rz - v[2] * rx);
    let sp = m::hyp2(v[0], v[2]);
    let still = v3::lerp(MOTION_STILL, 1.0, v3::smoothstep(MOTION_LIT_LO, MOTION_LIT_HI, lit));
    let mut k = v3::lerp(still, 1.0, v3::smoothstep(MOTION_LO, MOTION_HI, vt + MOTION_RADIAL * vr));
    if sp > MOTION_SPRINT_MS {
        k *= MOTION_SPRINT;
    }
    if dist < MOTION_NEAR_M {
        k = m::max(k, MOTION_NEAR_FLOOR);
    }
    if hunts {
        k = m::max(k, MOTION_HUNT_FLOOR);
    }
    k
}

/// a man an eye has: what the light shows of him, how he moves, whether he is low, the smoke between, and what being next to him is worth
struct Watched { c: CharId, light: f64, vel: V3, low: bool, smoke: f64, near: f64 }

/// Awareness per second a visible player builds in this man at `dist`.
pub fn sight_gain(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let near = if dist < 1.8 { crate::close::close_near_k(g, gd, dist) } else { 1.0 };
    let w = Watched { c, light: g.shown[c].visibility, vel: crate::guards::player_ground_vel(g), low: g.pl[pl].crouch, smoke: g.gd[gd].smoke_trans, near };
    gain(g, gd, &w, dist)
}

/// …and an AI man `o`: by the same formula, a lamp he carries and a tube that finds him counted.
pub fn sight_gain_on(g: &mut crate::game::Game, gd: GuardId, o: GuardId, dist: f64) -> f64 {
    let (c, oc) = (g.gd[gd].ch, g.gd[o].ch);
    let (glare, tube) = (crate::roomfight::lamp_glare(g, c, o), crate::nvg::nvg_sight(g, gd, o));
    let light = m::max_n(&[g.shown[oc].visibility, glare, tube]);
    let smoke = crate::roomfight::pair_smoke(g, c, oc);
    let w = Watched { c: oc, light, vel: g.ch[oc].vel, low: g.ch[oc].anim.crouch > 0.5, smoke: m::min(smoke, 1.0), near: 1.0 };
    gain(g, gd, &w, dist)
}

/// What a watched man builds a second in this one's eye.
fn gain(g: &Game, gd: GuardId, w: &Watched, dist: f64) -> f64 {
    let at = g.ch[w.c].pos;
    let hunts = hunting(g, gd);
    let moving = motion_sum(g.pos_of(gd), hunts, at, w.vel, w.light, dist);
    let keen = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    let mut rate = g.tune.detect_rate * m::pow(w.light, 1.5) * crate::guards::dist_term(g, dist) * (if w.low { 0.75 } else { 1.0 }) * moving * v3::clamp((w.smoke - 0.3) / 0.5, 0.0, 1.0) * keen;
    rate *= crate::attention::attention_side_k(g, gd, at) * crate::kitload::load_seen(g, w.c);
    if dist < 1.8 { rate = m::max(rate, 3.5 * w.near); }
    if hunts { rate *= 2.2; }
    rate
}

/// A dead colleague lying in this man's view, not yet called in.
pub fn body_in_view(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || !crate::condition::down(g, g.gd[o].ch) || g.gd[o].found || g.team(o) != g.team(gd) {
            continue;
        }
        if crate::guards::body_seen(g, gd, o, None) {
            return Some(o);
        }
    }
    None
}

/// One body's test: in range, in the cone (none with `around`), a clear line, light enough. `around`: None keeps the cone, a number is how far round his feet a body is plain.
pub fn body_seen(g: &mut crate::game::Game, gd: GuardId, o: GuardId, around: Option<f64>) -> bool {
    if crate::pairorders::pair_grip_body(g, gd) || crate::doze::doze_asleep(g, gd) {
        return false;
    }
    let c = g.gd[gd].ch;
    let oc = g.gd[o].ch;
    let pos = g.ch[c].pos;
    let eye = g.ch[c].bone(crate::character::Bone::Head).unwrap_or(v3::add(pos, [0.0, 1.65, 0.0]));
    let look = crate::guards::look_of(g, c);
    let body = g.ch[oc].bone(crate::character::Bone::Hips).unwrap_or(g.ch[oc].pos);
    let to = v3::sub(body, eye);
    let dist = v3::len(to);
    if dist > 12.0 {
        return false;
    }
    // `!around`: false, and a number that is 0 or NaN
    let cone = match around {
        Some(n) => !js::truthy(n),
        None => true,
    };
    if cone && v3::dot([look[0], 0.0, look[2]], v3::normalize([to[0], 0.0, to[2]])) < m::cos(60.0 * v3::DEG) {
        return false;
    }
    if crate::scene_collision::segment_blocked(g, eye, v3::add(body, [0.0, 0.3, 0.0]), None, None, None) {
        return false;
    }
    let at_feet = match around {
        Some(n) => v3::dist_xz(body, pos) <= n,
        None => false,
    };
    if g.tune.body_light {
        let seen_at = if at_feet { m::min(dist, g.k.body_light.near_m) } else { dist };
        if !crate::guards::body_lit(g, o, eye, seen_at) {
            return false;
        }
    }
    true
}

/// How lit this body reads now (0‥1): the light at its trunk, eased at the meter's rate.
pub fn body_visibility(g: &mut crate::game::Game, o: GuardId) -> f64 {
    let was = g.m.guards.body_meters.get(o).copied();
    let now = g.time;
    if let Some(row) = was {
        if row.t == now {
            return row.vis;
        }
    }
    let c = g.gd[o].ch;
    let hp = g.ch[c].bone(crate::character::Bone::Hips).unwrap_or(g.ch[c].pos);
    let at: V3 = [hp[0], m::max(hp[1], 0.12) + 0.22, hp[2]];
    let e = crate::guards::light_estimate_at(g, at, None, None, None);
    let dt = match was {
        Some(row) => m::min(0.1, m::max(0.0, now - row.t)),
        None => 0.0,
    };
    let target = crate::game::game_visibility_of(g, e);
    let vis = match was {
        Some(row) if dt > 0.0 => v3::damp(row.vis, target, 6.0, dt),
        _ => target,
    };
    g.m.guards.body_meters.set(o, crate::guards::GuardsModBodyMetersValue { vis, t: now });
    vis
}

/// Is body `o`, `dist` off `gd`'s eye at `eye`, lit enough for him to make out, and not behind smoke?
pub(super) fn body_lit(g: &mut crate::game::Game, o: GuardId, eye: V3, dist: f64) -> bool {
    let oc = g.gd[o].ch;
    let hp = g.ch[oc].bone(crate::character::Bone::Hips).unwrap_or(g.ch[oc].pos);
    let at: V3 = [hp[0], hp[1] + 0.25, hp[2]];
    let trans = crate::sense::clear_between(g, eye, at);
    let b = g.k.body_light;
    if trans < b.smoke {
        return false;
    }
    let vis = crate::guards::body_visibility(g, o) * v3::clamp((trans - b.smoke) / 0.5 + 0.4, 0.4, 1.0);
    let bar = if dist <= b.near_m {
        -1.0
    } else {
        v3::lerp(b.lit_near, b.lit, v3::clamp((dist - b.near_m) / m::max(0.1, b.far_m - b.near_m), 0.0, 1.0))
    };
    vis >= bar
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pinned bits of the motion factor and the player's ground velocity, on the same frames.
    #[test]
    fn ground_track_and_motion_match_the_typescript() {
        let eye: V3 = [3.25, 0.0, -1.5];
        let lit = 0.4;
        let mut row = crate::guards::GuardsModGroundTrackValue { pos: [0.0, 0.0, 0.0], t: 10.0, vel: [0.0, 0.0, 0.0] };
        assert_eq!(motion_sum(eye, false, row.pos, row.vel, lit, 3.57).to_bits(), 0x3fdba661e7b30641);

        let p = [0.02, 0.0, 0.015];
        ground_step(&mut row, p, 10.0 + 1.0 / 60.0);
        assert_eq!(row.vel[0].to_bits(), 0x3ff3333333333227);
        assert_eq!(row.vel[2].to_bits(), 0x3feccccccccccb39);
        assert_eq!(motion_sum(eye, false, p, row.vel, lit, 3.57).to_bits(), 0x3ff0000000000000);

        let p = [0.2, 0.0, 0.015];
        ground_step(&mut row, p, 10.0 + 2.0 / 60.0);
        assert_eq!(row.vel[0].to_bits(), 0x4018000000000000);
        assert_eq!(row.vel[2].to_bits(), 0x0000000000000000);
        assert_eq!(motion_sum(eye, false, p, row.vel, lit, 2.0).to_bits(), 0x3ff999999999999a);

        ground_step(&mut row, p, 11.0);
        assert_eq!(row.vel, [0.0, 0.0, 0.0]);
        assert_eq!(motion_sum(eye, true, p, row.vel, lit, 3.0).to_bits(), 0x3fe6666666666666);
    }
}
