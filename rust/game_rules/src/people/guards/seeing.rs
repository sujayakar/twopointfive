//! src/game/guards.ts lines 1 to 418
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A bone of a body, or a stand-in where it has none yet (`c.bones.head ?? …`).
pub(crate) fn bone_or(g: &Game, c: CharId, bone: crate::character::Bone, or: V3) -> V3 {
    match g.ch[c].bones.get(bone) {
        Some(b) => *b,
        None => or,
    }
}

fn hunting(g: &Game, gd: GuardId) -> bool {
    let st = g.gd[gd].state;
    st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search
}

pub(crate) fn shot_e(e: &crate::game::GameEvent) -> crate::guards::ShotRelationE {
    crate::guards::ShotRelationE { kind: e.kind, who: e.who }
}

/// How a 'glass' event reaches an ear: from its own point, and from each face of the pane it carries, whichever arrives best. (guards.ts glassHeard)
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

/// His eyes have a man at `at` this moment, by another rule's look than updateGuard's own: the record of a sighting, in one place. (guards.ts eyesOn)
pub fn eyes_on(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    crate::mind::fix(g, gd, Some(at), "guards");
    let now = g.time;
    let man = &mut g.gd[gd];
    man.sight_pos = at;
    man.sight_t = now;
    man.last_seen_t = now;
}

/// Gunfire filed and not through this man's ears yet that hearingCheck will count when it next runs for him; `bangs` counts a stun charge or a door blown too. (guards.ts gunfireDue)
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

/// Whose gunfire this is to `gd`: a man of his side's, an enemy's, or neither. (guards.ts shotRelation)
pub(super) fn shot_relation(g: &crate::game::Game, gd: GuardId, e: crate::guards::ShotRelationE) -> crate::guards::ShotRelation {
    let shooter = if e.kind == crate::director::EventKind::Shot { crate::teams::Team::Player } else { crate::teams::team_of_id(g, e.who.unwrap_or(-1.0)) };
    let team = g.gd[gd].team;
    if shooter == team {
        return crate::guards::ShotRelation::Own;
    }
    if crate::teams::is_hostile(g, team, shooter) { crate::guards::ShotRelation::Threat } else { crate::guards::ShotRelation::None }
}

/// Where a colleague's shots say the enemy is: the shooter's own idea of his man at its own age; none when the hearer keeps what he has. (guards.ts colleagueFirePos)
pub(super) fn colleague_fire_pos(g: &mut crate::game::Game, gd: GuardId, e: crate::guards::ColleagueFirePosE) -> Option<V3> {
    use crate::picture::Who;
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
            if g.ch[g.gd[x].ch].id == who {
                sh = Some(x);
                break;
            }
        }
    }
    let player_pos = g.ch[g.pl[g.player].ch].pos;
    if let Some(s) = sh {
        let who: Option<Who> = match g.gd[s].foe {
            Some(crate::game::Foe::Guard(f)) => Some(Who::Guard(f)),
            Some(crate::game::Foe::Player) => Some(Who::Player),
            None => {
                if g.gd[s].team == crate::teams::Team::Guard { Some(Who::Player) } else { None }
            }
        };
        if let Some(who) = who {
            if let Some(i) = crate::picture::idea_of(g, s, who) {
                let tr = match who {
                    Who::Player => player_pos,
                    Who::Guard(f) => g.ch[g.gd[f].ch].pos,
                };
                let mine = crate::picture::idea_of(g, gd, who);
                g.stats.picture.shot_placed += 1;
                if m::hyp2(i.at[0] - tr[0], i.at[2] - tr[2]) >= 1.0 {
                    g.stats.picture.shot_off += 1;
                }
                let own = match &mine {
                    Some(mi) => mi.t >= i.t,
                    None => false,
                };
                if g.gd[gd].last_known.is_some() && (i.lost || own || g.gd[gd].fix_t >= i.t) {
                    g.stats.picture.shot_kept += 1;
                    return None;
                }
                let w = if own { mine.unwrap() } else { i };
                if own {
                    g.stats.picture.shot_kept += 1;
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
        Some(s) if g.gd[s].team != crate::teams::Team::Guard => s,
        _ => return Some(player_pos),
    };
    if let Some(fc) = crate::teams::foe_char(g, s) {
        return Some(g.ch[fc].pos);
    }
    Some(match e.to {
        Some(t) => t,
        None => e.pos,
    })
}

/// Is this footfall an enemy's: Sam's steps carry no `who`; a unit's carry his id. (guards.ts stepIsThreat)
pub(super) fn step_is_threat(g: &crate::game::Game, gd: GuardId, e: crate::guards::StepIsThreatE) -> bool {
    let who = e.who.unwrap_or(crate::consts::PLAYER_ID);
    let team = g.gd[gd].team;
    if crate::seats::is_man_id(g, who) {
        crate::teams::is_hostile(g, team, crate::teams::Team::Player)
    } else {
        let of = crate::teams::team_of_id(g, who);
        crate::teams::is_hostile(g, team, of)
    }
}

/// A restart: the fresh man's start is not a step from where the last one fell. (guards.ts groundFresh)
pub fn ground_fresh(g: &mut crate::game::Game) {
    let view = crate::seats::view_key(g, &crate::seats::here(g));
    g.m.guards.ground_track.delete(view);
}

/// Sam's velocity over the ground this frame, as anybody watching sees it: his root's displacement since the frame before, capped. (guards.ts playerGroundVel)
pub fn player_ground_vel(g: &mut crate::game::Game) -> V3 {
    let view = crate::seats::view_key(g, &crate::seats::here(g));
    let p = g.ch[g.pl[g.player].ch].pos;
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

/// × the sight gain for how Sam is moving as this man sees it. (guards.ts motionFactor)
pub(super) fn motion_factor(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 {
    let pp = g.ch[g.pl[g.player].ch].pos;
    let v = crate::guards::player_ground_vel(g);
    let lit = g.pl[g.player].visibility;
    crate::guards::motion_core(g, gd, pp, v, lit, dist)
}

/// motionFactor's arithmetic for any watched man at `pp` moving at `v`, lit `lit`, `dist` off. (guards.ts motionCore)
pub(super) fn motion_core(g: &crate::game::Game, gd: GuardId, pp: V3, v: V3, lit: f64, dist: f64) -> f64 {
    let gp = g.ch[g.gd[gd].ch].pos;
    let hunts = hunting(g, gd);
    motion_sum(gp, hunts, pp, v, lit, dist)
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

/// Awareness per second a visible player builds in this man at `dist`. (guards.ts sightGain)
pub fn sight_gain(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> f64 {
    let detect = g.tune.detect_rate;
    let light = g.pl[g.player].visibility;
    let dist_f = crate::guards::dist_term(g, dist);
    let moving = crate::guards::motion_factor(g, gd, dist);
    let crouch = g.pl[g.player].crouch;
    let smoke = g.gd[gd].smoke_trans;
    let keen = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    let mut rate = detect * m::pow(light, 1.5) * dist_f * (if crouch { 0.75 } else { 1.0 }) * moving * v3::clamp((smoke - 0.3) / 0.5, 0.0, 1.0) * keen;
    let pp = g.ch[g.pl[g.player].ch].pos;
    let side = crate::attention::attention_side_k(g, gd, pp);
    rate *= side * crate::kitload::load_seen(g, &crate::seats::here(g), crate::kitload::LoadSeenWho::Player);
    if dist < 1.8 {
        rate = m::max(rate, 3.5 * crate::close::close_near_k(g, gd, dist));
    }
    if hunting(g, gd) {
        rate *= 2.2;
    }
    rate
}

/// sightGain for an AI target `o`: the same formula off his light meter, his crouch, his motion. (guards.ts sightGainOn)
pub fn sight_gain_on(g: &mut crate::game::Game, gd: GuardId, o: GuardId, dist: f64) -> f64 {
    let detect = g.tune.detect_rate;
    let c = g.gd[gd].ch;
    let oc = g.gd[o].ch;
    let vis = g.gd[o].visibility;
    let glare = crate::roomfight::lamp_glare(g, c, o);
    let tube = crate::nvg::nvg_sight(g, gd, o);
    let light = m::max_n(&[vis, glare, tube]);
    let dist_f = crate::guards::dist_term(g, dist);
    let v = g.ch[oc].vel;
    let op = g.ch[oc].pos;
    let moving = crate::guards::motion_core(g, gd, op, v, light, dist);
    let smoke = crate::roomfight::pair_smoke(g, c, oc);
    let crouched = g.ch[oc].anim.crouch > 0.5;
    let keen = if crate::guards::escalation_of(g) >= 1.0 { crate::guards::KEEN } else { 1.0 };
    let mut rate = detect
        * m::pow(light, 1.5)
        * dist_f
        * (if crouched { 0.75 } else { 1.0 })
        * moving
        * (if smoke < 1.0 { v3::clamp((smoke - 0.3) / 0.5, 0.0, 1.0) } else { 1.0 })
        * keen;
    let op = g.ch[oc].pos;
    let side = crate::attention::attention_side_k(g, gd, op);
    rate *= side * crate::kitload::load_seen(g, &crate::seats::here(g), crate::kitload::LoadSeenWho::Guard(o));
    if dist < 1.8 {
        rate = m::max(rate, 3.5);
    }
    if hunting(g, gd) {
        rate *= 2.2;
    }
    rate
}

/// A dead colleague lying in this man's view, not yet called in. (guards.ts bodyInView)
pub fn body_in_view(g: &mut crate::game::Game, gd: GuardId) -> Option<GuardId> {
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].state != crate::game::GuardState::Dead || g.gd[o].found || g.gd[o].team != g.gd[gd].team {
            continue;
        }
        if crate::guards::body_seen(g, gd, o, None) {
            return Some(o);
        }
    }
    None
}

/// bodyInView's test of one body: in range, in the cone (none with `around`), a clear line, light enough. `around`: None is the TypeScript's false, a number how far round his feet a body is plain. (guards.ts bodySeen)
pub fn body_seen(g: &mut crate::game::Game, gd: GuardId, o: GuardId, around: Option<f64>) -> bool {
    if crate::pairorders::pair_grip_body(g, gd) || crate::doze::doze_asleep(g, gd) {
        return false;
    }
    let c = g.gd[gd].ch;
    let oc = g.gd[o].ch;
    let pos = g.ch[c].pos;
    let eye = bone_or(g, c, crate::character::Bone::Head, v3::add(pos, [0.0, 1.65, 0.0]));
    let look = crate::guards::look_of(g, c);
    let body = bone_or(g, oc, crate::character::Bone::Hips, g.ch[oc].pos);
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
        Some(n) => m::hyp2(body[0] - pos[0], body[2] - pos[2]) <= n,
        None => false,
    };
    if g.tune.body_light {
        let seen_at = if at_feet { m::min(dist, g.k.body_light.near_m) } else { dist };
        if !crate::guards::body_lit(g, gd, o, eye, seen_at) {
            return false;
        }
    }
    true
}

/// How lit this body reads now (0‥1): the light at its trunk, eased at the meter's rate. (guards.ts bodyVisibility)
pub fn body_visibility(g: &mut crate::game::Game, o: GuardId) -> f64 {
    let was = g.m.guards.body_meters.get(o).copied();
    let now = g.time;
    if let Some(row) = was {
        if row.t == now {
            return row.vis;
        }
    }
    let c = g.gd[o].ch;
    let hp = bone_or(g, c, crate::character::Bone::Hips, g.ch[c].pos);
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

/// Is body `o`, `dist` off `gd`'s eye at `eye`, lit enough for him to make out, and not behind smoke? (guards.ts bodyLit)
pub(super) fn body_lit(g: &mut crate::game::Game, gd: GuardId, o: GuardId, eye: V3, dist: f64) -> bool {
    let oc = g.gd[o].ch;
    let hp = bone_or(g, oc, crate::character::Bone::Hips, g.ch[oc].pos);
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

    /// The bits are the TypeScript's own: motionFactor and playerGroundVel under EXACT.on, bun, this part's guards.ts, the same frames.
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
