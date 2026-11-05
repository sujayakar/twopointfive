//! src/game/wounds.ts lines 1 to 140
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Zero what the rule has answered since the last reset. (wounds.ts resetWoundStats)
pub fn reset_wound_stats() { todo!("needs: g, the counters are g.stats.wounds and the stub has no g; only tools/qa call it") }

/// Closest distance between segments p1→q1 and p2→q2. (wounds.ts segDist)
pub fn seg_dist(p1: V3, q1: V3, p2: V3, q2: V3) -> f64 {
    let (d1, d2, r) = (v3::sub(q1, p1), v3::sub(q2, p2), v3::sub(p1, p2));
    let (a, e, f) = (v3::dot(d1, d1), v3::dot(d2, d2), v3::dot(d2, r));
    let (mut s, mut t) = (0.0, 0.0);
    if a <= 1e-9 && e <= 1e-9 { return v3::len(r); }
    if a <= 1e-9 {
        t = m::min(1.0, m::max(0.0, f / e));
    } else {
        let c = v3::dot(d1, r);
        if e <= 1e-9 {
            s = m::min(1.0, m::max(0.0, -c / a));
        } else {
            let b = v3::dot(d1, d2);
            let den = a * e - b * b;
            s = if den > 1e-9 { m::min(1.0, m::max(0.0, (b * f - c * e) / den)) } else { 0.0 };
            t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = m::min(1.0, m::max(0.0, -c / a));
            } else if t > 1.0 {
                t = 1.0;
                s = m::min(1.0, m::max(0.0, (b - c) / a));
            }
        }
    }
    v3::len(v3::sub(v3::mad(p1, d1, s), v3::mad(p2, d2, t)))
}

/// The closest the stretch a→b comes to a chain of bones; a bone he lacks skips its links. (wounds.ts hitZone `line`)
fn line(bones: &JsMap<crate::character::Bone, V3>, a: V3, b: V3, k: &[crate::character::Bone]) -> f64 {
    let mut d = 9.0;
    let mut i = 0;
    while i + 1 < k.len() {
        if let (Some(&p), Some(&q)) = (bones.get(k[i]), bones.get(k[i + 1])) {
            d = m::min(d, seg_dist(a, b, p, q));
        }
        i += 1;
    }
    d
}

/// Where the round landed, measured against his bones as they stand this frame. (wounds.ts hitZone)
pub fn hit_zone(g: &mut crate::game::Game, c_2: CharId, ro: V3, rd: V3, t_hit: f64) -> crate::wounds::HitZone {
    let k = g.k.wound;
    let a = v3::mad(ro, rd, m::max(0.0, t_hit - k.lead));
    let b = v3::mad(ro, rd, t_hit + k.depth);
    let bones = &g.ch[c_2].bones;
    let head = match (bones.get(crate::character::Bone::Neck), bones.get(crate::character::Bone::Head)) {
        (Some(&neck), Some(&crown)) => seg_dist(a, b, neck, v3::mad(crown, v3::normalize(v3::sub(crown, neck)), 0.16)),
        _ => 9.0,
    };
    let trunk = line(bones, a, b, &[crate::character::Bone::Hips, crate::character::Bone::Spine2, crate::character::Bone::Chest, crate::character::Bone::Neck]);
    let leg = m::min(
        line(bones, a, b, &[crate::character::Bone::ThighL, crate::character::Bone::ShinL, crate::character::Bone::FootL]),
        line(bones, a, b, &[crate::character::Bone::ThighR, crate::character::Bone::ShinR, crate::character::Bone::FootR]),
    );
    let zone = if head <= k.head {
        WoundZone::Head
    } else if trunk <= k.centre {
        WoundZone::Centre
    } else if leg < trunk {
        WoundZone::Leg
    } else {
        WoundZone::Side
    };
    crate::wounds::HitZone { zone, head, trunk, leg }
}

/// What the round does to the man it stopped in. (wounds.ts roundDoes)
pub fn round_does(g: &mut crate::game::Game, gd: GuardId, ro: V3, rd: V3, t_hit: f64, hostage: bool, by_player: bool) -> crate::wounds::RoundDoes {
    if !g.k.wound.on && g.gd[gd].wound.is_none() { return RoundDoes::Shot; }
    let c = g.gd[gd].ch;
    if g.gd[gd].wound.is_some() {
        let z = hit_zone(g, c, ro, rd, t_hit).zone;
        g.stats.wounds.again += 1;
        if z == WoundZone::Head || z == WoundZone::Centre {
            g.stats.wounds.finished += 1;
            finish_wounded(g, gd, rd, by_player);
        }
        return RoundDoes::None;
    }
    if g.gd[gd].civ.is_some() || hostage || g.gd[gd].held.is_some() || g.gd[gd].state == crate::game::GuardState::Dead || g.touring || crate::director::director_showing(g) {
        return RoundDoes::Shot;
    }
    let z = hit_zone(g, c, ro, rd, t_hit).zone;
    g.stats.wounds.by_zone[z as usize] += 1;
    match z {
        WoundZone::Head | WoundZone::Centre => {
            g.stats.wounds.dead += 1;
            RoundDoes::Shot
        }
        WoundZone::Side => {
            g.stats.wounds.wounded += 1;
            RoundDoes::Side
        }
        WoundZone::Leg => {
            g.stats.wounds.wounded += 1;
            RoundDoes::Leg
        }
    }
}

/// Is there clear floor behind him at this height? (wounds.ts woundDown `room`)
fn room(g: &mut Game, c: CharId, f: V3, y: f64) -> bool {
    let (p, back) = (g.ch[c].pos, g.k.wound.back_room);
    let a: V3 = [p[0], y, p[2]];
    let b: V3 = [p[0] - f[0] * back, y, p[2] - f[2] * back];
    !crate::scene_collision::solid_blocked(g, a, b, None) && !crate::scene_collision::segment_blocked_dynamic(g, a, b, None)
}

/// In place of the ragdoll: he stays a posed figure, on his back or on one knee, the fight cleared off him. (wounds.ts woundDown)
pub fn wound_down(g: &mut crate::game::Game, gd: GuardId, dir: V3, zone: Option<crate::wounds::WoundDownZone>) {
    let zone = zone.unwrap_or(WoundDownZone::Side);
    let c = g.gd[gd].ch;
    crate::character::character_turn_drop(g, c);
    let f = crate::character::character_forward(g, c);
    let from_front = f[0] * dir[0] + f[2] * dir[2] < -0.25;
    let pose = if zone == WoundDownZone::Leg && from_front && room(g, c, f, 0.25) && room(g, c, f, 0.9) { WoundPose::Back } else { WoundPose::Knee };
    let kept = match zone {
        WoundDownZone::Side => WoundZone2::Side,
        WoundDownZone::Leg => WoundZone2::Leg,
    };
    let t = g.time;
    g.gd[gd].wound = Some(Wound { zone: kept, pose, t });
    lay(g, gd, false, None);
    g.gd[gd].stagger_until = -1.0;
    g.gd[gd].dazzled_until = -1.0;
    crate::character::anim_hit(g, c);
    let said = if zone == WoundDownZone::Leg { "my leg — I'm down!" } else { "I'm hit — I'm hit!" };
    crate::game::game_say(g, gd, said, None, None, None);
}

/// The pose and nothing else: his animator given the wound, the fight's layers taken off him. (wounds.ts lay)
pub fn lay(g: &mut crate::game::Game, gd: GuardId, settled: bool, clock: Option<f64>) {
    let c = g.gd[gd].ch;
    let w = g.gd[gd].wound.unwrap();
    let ch = &mut g.ch[c];
    ch.anim.wound = Some(crate::character::AnimWound {
        pose: match w.pose { WoundPose::Knee => crate::character::AnimWoundPose::Knee, WoundPose::Back => crate::character::AnimWoundPose::Back },
        at: if w.zone == WoundZone2::Leg { crate::character::AnimWoundAt::Thigh } else { crate::character::AnimWoundAt::Ribs },
        settled: Some(settled),
        clock,
    });
    ch.anim.act = None;
    ch.anim.upper = crate::character::UpperMode::None;
    ch.anim.speed = 0.0;
    ch.vel = [0.0, 0.0, 0.0];
    ch.aim_yaw = ch.body_yaw;
    ch.aim_pitch = 0.0;
    ch.anim.look_yaw_extra = 0.0;
}

/// A round through the head or the middle of a man already down wounded: he is dead and the books are put right. (wounds.ts finishWounded)
pub fn finish_wounded(g: &mut crate::game::Game, gd: GuardId, dir: V3, by_player: bool) {
    let c = g.gd[gd].ch;
    g.gd[gd].wound = None;
    g.ch[c].anim.wound = None;
    g.gd[gd].down_kind = Some(crate::game::DownKind::Shot);
    if by_player {
        if crate::mission::mission_live(g) {
            if g.gd[gd].down_by == Some(crate::game::DownBy::Player) && g.mission.stats.knockouts > 0.0 { g.mission.stats.knockouts -= 1.0; }
            g.mission.stats.kills += 1.0;
        }
        crate::chronicle::chron_down(g, gd, crate::game::DownKind::Shot, None);
        g.gd[gd].down_by = Some(crate::game::DownBy::Player);
        crate::guards::witness_kill(g, gd, crate::game::DownKind::Shot, crate::game::DownBy::Player);
    }
    let world = crate::ragdoll::RagdollWorld { boxes: g.col.boxes.ids().collect(), dynamic_boxes: g.col.dynamic_boxes.clone() };
    crate::character::character_ragdollize(g, c, Some(&world), dir, true);
    let text = alloc::format!("{} — finished on the floor", g.gd[gd].callsign);
    crate::game::game_msg(g, &text);
}

/// Every man down wounded right now. (wounds.ts woundedMen)
pub fn wounded_men(g: &mut crate::game::Game) -> Vec<GuardId> {
    g.guards.iter().copied().filter(|&x| g.gd[x].wound.is_some()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The inputs and answers are the TypeScript's own bits, under bun with EXACT.on, one case for each way through segDist.
    #[test]
    fn seg_dist_matches_the_typescript_bits() {
        let cases: &[([u64; 12], u64)] = &[
            // both points
            ([0x3ff199999999999a, 0x3fd3333333333333, 0xc00599999999999a, 0x3ff199999999999a, 0x3fd3333333333333, 0xc00599999999999a, 0x3fc999999999999a, 0x3ffe666666666666, 0x3fd999999999999a, 0x3fc999999999999a, 0x3ffe666666666666, 0x3fd999999999999a], 0x400cd27c981873b2),
            // first a point
            ([0x3fd3333333333333, 0x3ff199999999999a, 0x3fe6666666666666, 0x3fd3333333333333, 0x3ff199999999999a, 0x3fe6666666666666, 0xbff4cccccccccccd, 0x3fc999999999999a, 0x3fb999999999999a, 0x3ffb333333333333, 0x3ffe666666666666, 0x3fe3333333333333], 0x3fd5255eaa80a0e5),
            // second a point
            ([0xbff4cccccccccccd, 0x3fc999999999999a, 0x3fb999999999999a, 0x3ffb333333333333, 0x3ffe666666666666, 0x3fe3333333333333, 0x3fd3333333333333, 0x3ff199999999999a, 0x3fe6666666666666, 0x3fd3333333333333, 0x3ff199999999999a, 0x3fe6666666666666], 0x3fd5255eaa80a0e5),
            // crossing
            ([0x3fbc28f5c28f5c29, 0x3ff5eb851eb851ec, 0xc002666666666666, 0x3fc851eb851eb852, 0x3ff4f5c28f5c28f6, 0x3ffe666666666666, 0xbfe6666666666666, 0x3ff3333333333333, 0x3fd3333333333333, 0x3feccccccccccccd, 0x3ff8000000000000, 0x3fb999999999999a], 0x3f9a910f000d647e),
            // t under 0
            ([0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3fb999999999999a, 0x0000000000000000, 0x3fd3333333333333, 0x3ff0000000000000, 0x3fc999999999999a, 0x3fd999999999999a, 0x4008000000000000, 0x3fe0000000000000], 0x3fef8ac51908a48d),
            // t over 1
            ([0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3fb999999999999a, 0x0000000000000000, 0x3fd3333333333333, 0x4008000000000000, 0x3fc999999999999a, 0x3fd999999999999a, 0x3ff0000000000000, 0x3fe0000000000000], 0x3ff1403b323a2cf6),
            // parallel
            ([0x0000000000000000, 0x3fb999999999999a, 0x0000000000000000, 0x3ff0000000000000, 0x3fb999999999999a, 0x0000000000000000, 0x3fc999999999999a, 0x3feccccccccccccd, 0x3fd3333333333333, 0x3ffb333333333333, 0x3feccccccccccccd, 0x3fd3333333333333], 0x3feb573f7449ba9f),
            // s clamps
            ([0x4000cccccccccccd, 0x3fe6666666666666, 0xbfd3333333333333, 0x400a666666666666, 0x3ff199999999999a, 0x3feccccccccccccd, 0x3fb999999999999a, 0x3fd999999999999a, 0x3fc999999999999a, 0xbfe999999999999a, 0x3ff999999999999a, 0x3fd3333333333333], 0x4000aa87b6acca63),
            // irrational
            ([0x3fdcb91f3bbba140, 0x3fd5555555555555, 0xbfe165a208dd12ba, 0x3fc2492492492492, 0x3ff6a09e667f3bcd, 0x3fe5555555555555, 0x3fd3333333333334, 0x3ff7333333333333, 0x3fcddddddddddddd, 0xbfa5f15f15f15f16, 0x3fed1eb851eb851f, 0x3fc27d27d27d27d3], 0x3fc70ca229eed2bb),
        ];
        for (v, want) in cases {
            let f = |i: usize| -> V3 { [f64::from_bits(v[i]), f64::from_bits(v[i + 1]), f64::from_bits(v[i + 2])] };
            assert_eq!(seg_dist(f(0), f(3), f(6), f(9)).to_bits(), *want);
        }
    }
}
