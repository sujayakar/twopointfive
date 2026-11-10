//! WORK HE STANDS OR KNEELS AT: a lock, the wand, a code, a job of his hands, a door kicked.
use super::*;
use crate::game::{Deed, Game};
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The 'pick the lock' row: drop what the hands were doing and start working the lock.
pub fn start_picking(g: &mut crate::game::Game, d: DoorId) {
    let pl = g.player;
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].planted() {
        return;
    }
    if g.dr[d].def.steel.unwrap_or(false) {
        return;
    }
    crate::unseen::kneel(g, pl);
    crate::deed::hands_free(g);
    crate::deed::begin(g, Deed::Pick(d));
}

/// Hands off the tools: the progress stays on the door, he stays on his knee.
pub(in crate::sam) fn stop_picking(g: &mut crate::game::Game) {
    let pl = g.player;
    if g.pl[pl].picking().is_none() {
        return;
    }
    crate::deed::put_down(g, pl);
    g.pl[pl].speed_sm = 0.0;
}

/// Working the lock, on its own clock once begun.
pub(super) fn update_picking(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let d = g.pl[pl].picking().copied().unwrap();
    let pos = g.ch[c].pos;
    if !crate::doors::doors_locked_out(g, d, pos, None) {
        stop_picking(g);
    } else {
        let spot = crate::doors::doors_work_spot(g, d, pos, None, None);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        { let to = v3::lerp3(g.ch[c].pos, spot.pos, 1.0 - m::exp(-10.0 * dt)); crate::character::pos_replace(g, c, to); }
        collide_player(g, 1.0);
        keep_off_guards(g, dt, 1.0, None);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, spot.yaw, 12.0 * dt);
        g.ch[c].aim_yaw = g.ch[c].body_yaw;
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
        crate::unseen::kneel(g, pl);
        g.pl[pl].sprinting = false;
        g.pl[pl].backpedal = false;
        g.ch[c].anim.reverse = false;
        g.ch[c].anim.lockpick = true;
        g.ch[c].anim.speed = 0.0;
        g.ch[c].anim.crouch_target = 1.0;
        g.ch[c].anim.upper = crate::character::UpperMode::None;
        let planted = v3::dist_xz(g.ch[c].pos, spot.pos) < 0.12 && g.ch[c].anim.hide_held_item;
        let who = g.ch[c].id;
        if planted && crate::doors::doors_pick(g, d, who, dt) {
            crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::LockPicked, d);
            stop_picking(g);
        }
    }
    let in_hands = crate::game::player_rifle_up(g, pl) && !g.ch[c].anim.hide_held_item;
    arm_rifle(g, in_hands, Some(crate::character::LongStance::LowLong));
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
}

/// The 'look under it' row of a shut door: the hands' work dropped, onto his knee at the middle of the leaf.
pub fn start_wand(g: &mut crate::game::Game, d: DoorId) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].planted() || crate::handwork::sam_work(g).is_some() {
        return;
    }
    if !crate::wand::wand_offered(g, pl, d) {
        return;
    }
    crate::unseen::kneel(g, pl);
    crate::deed::hands_free(g);
    let pos = g.ch[c].pos;
    let side = crate::doors::doors_side(g, d, pos);
    crate::deed::begin(g, Deed::Wand(crate::wand::Wanding { door: d, side, pan: 0.0, live: false, off: 0.0, age: 0.0 }));
}

/// The wand pulled out: he stays on his knee.
pub(in crate::sam) fn stop_wand(g: &mut crate::game::Game) {
    let pl = g.player;
    if g.pl[pl].wanding().is_none() {
        return;
    }
    crate::wand::wand_drop(g, pl);
    g.pl[pl].speed_sm = 0.0;
}

/// One frame of the look: planted on his knee at the leaf; once there the lens is live and turns after the aim point.
pub(super) fn update_wanding(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    // The TypeScript's W outlives a stop (its object is only dropped from pl.wanding), so a local copy written back while he still has the look does the same.
    let mut w = g.pl[pl].wanding().copied().unwrap();
    let d = w.door;
    let pos = g.ch[c].pos;
    if !crate::fogofwar::fog_of_war_active(g) {
        stop_wand(g);
    } else if !crate::wand::wand_fits(g, d) || crate::doors::doors_side(g, d, pos) != w.side {
        stop_wand(g);
    } else if crate::samstun::sam_staggered(g, &crate::seats::here(g), None) || crate::condition::is_blind(g, g.pl[pl].ch) {
        stop_wand(g);
    } else {
        let along = g.dr[d].def.width * 0.5;
        let spot = crate::doors::doors_work_spot(g, d, pos, Some(0.5), Some(along));
        g.ch[c].vel = [0.0, 0.0, 0.0];
        { let to = v3::lerp3(g.ch[c].pos, spot.pos, 1.0 - m::exp(-10.0 * dt)); crate::character::pos_replace(g, c, to); }
        collide_player(g, 1.0);
        keep_off_guards(g, dt, 1.0, None);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, spot.yaw, 12.0 * dt);
        g.ch[c].aim_yaw = g.ch[c].body_yaw;
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
        crate::unseen::kneel(g, pl);
        g.pl[pl].sprinting = false;
        g.pl[pl].backpedal = false;
        g.ch[c].anim.reverse = false;
        g.ch[c].anim.lockpick = true;
        g.ch[c].anim.speed = 0.0;
        g.ch[c].anim.crouch_target = 1.0;
        g.ch[c].anim.upper = crate::character::UpperMode::None;
        let off_spot = v3::dist_xz(g.ch[c].pos, spot.pos);
        let planted = off_spot < 0.12 && g.ch[c].anim.hide_held_item;
        w.age += dt;
        if planted {
            w.off = 0.0;
            let crouch = g.ch[c].anim.crouch;
            if !w.live && crate::wand::wand_through(g, w.age, crouch, Some(true)) {
                w.live = true;
                wand_row_back(g, pl, w);
            }
        } else if w.live {
            wand_row_back(g, pl, w);
            stop_wand(g);
        } else if off_spot >= 0.12 && {
            w.off += dt;
            w.off >= g.k.wand.give_up_secs
        } {
            wand_row_back(g, pl, w);
            stop_wand(g);
        }
        wand_row_back(g, pl, w);
        if g.pl[pl].wanding().is_some() && w.live {
            let aim = g.sets[g.seat].aim_point;
            crate::wand::wand_turn(g, pl, aim, dt);
        }
    }
    let in_hands = crate::game::player_rifle_up(g, pl) && !g.ch[c].anim.hide_held_item;
    arm_rifle(g, in_hands, Some(crate::character::LongStance::LowLong));
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
}

/// W's fields back on the row, if he still has the look.
fn wand_row_back(g: &mut crate::game::Game, pl: PlayerId, w: crate::wand::Wanding) {
    if let Some(row) = g.pl[pl].wanding_mut() {
        *row = w;
    }
}

/// Keying a code in: stood at the pad with the feet still, turned to the plate, the rifle low. HANDWORK.on and .work are settled on.
pub(super) fn update_coding_plant(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let d = g.pl[pl].coding().unwrap().door;
    let k = crate::devices::devices_pad_of(g, d);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    if let Some(k) = &k {
        if g.pl[pl].wall.is_some() {
            release_wall(g);
        }
        let hand = crate::handwork::sam_work_hand(g, &crate::seats::here(g));
        crate::handwork::work_stand(g, c, k.def.pos, k.n, dt, Some(hand), None);
    }
    collide_player(g, 1.0);
    keep_off_guards(g, dt, 1.0, None);
    let step_v = if k.is_some() && dt > 0.0 { m::hyp2(g.ch[c].pos[0] - x0, g.ch[c].pos[2] - z0) / dt } else { 0.0 };
    if let Some(k) = &k {
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, m::atan2(-k.n[0], -k.n[2]), 10.0 * dt);
        g.ch[c].aim_yaw = g.ch[c].body_yaw;
    }
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
    g.pl[pl].sprinting = false;
    g.pl[pl].backpedal = false;
    g.ch[c].anim.reverse = false;
    g.ch[c].anim.speed = if step_v > 0.2 { m::min(step_v, 1.7) } else { 0.0 };
    g.ch[c].anim.upper = crate::character::UpperMode::None;
    let in_hands = crate::game::player_rifle_up(g, pl);
    arm_rifle(g, in_hands, Some(crate::character::LongStance::LowLong));
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
}

/// Hands at work on a thing that is not a lock or a pad: the keypad's plant.
pub(super) fn update_work_plant(g: &mut crate::game::Game, dt: f64, w: &mut crate::handwork::SamWork) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if g.pl[pl].wall.is_some() {
        release_wall(g);
    }
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let hand = crate::handwork::sam_work_hand(g, &crate::seats::here(g));
    let yaw = crate::handwork::work_stand(g, c, w.p, w.n, dt, Some(hand), None);
    collide_player(g, 1.0);
    keep_off_guards(g, dt, 1.0, None);
    if let Some(at) = w.at.as_mut() {
        let (dx, dz) = (g.ch[c].pos[0] - x0, g.ch[c].pos[2] - z0);
        at[0] += dx;
        at[2] += dz;
        work_at_walked(g, pl, w.kind, dx, dz);
    }
    let step_v = if dt > 0.0 { m::hyp2(g.ch[c].pos[0] - x0, g.ch[c].pos[2] - z0) / dt } else { 0.0 };
    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, crate::handwork::WORK_TURN * dt);
    g.ch[c].aim_yaw = g.ch[c].body_yaw;
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
    g.pl[pl].sprinting = false;
    g.pl[pl].backpedal = false;
    g.ch[c].anim.reverse = false;
    g.ch[c].anim.speed = if step_v > 0.2 { m::min(step_v, 1.7) } else { 0.0 };
    g.ch[c].anim.upper = crate::character::UpperMode::None;
    g.ch[c].anim.lockpick = false;
    let in_hands = crate::game::player_rifle_up(g, pl);
    arm_rifle(g, in_hands, Some(crate::character::LongStance::LowLong));
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
}

/// W.at[0] += dx; W.at[2] += dz. W.at IS the verb's owner's own vector in the TypeScript: the write goes to that owner, by the kind of work.
fn work_at_walked(g: &mut crate::game::Game, pl: PlayerId, kind: crate::handwork::SamWorkKind, dx: f64, dz: f64) {
    let at = match kind {
        crate::handwork::SamWorkKind::Wall => g.pl[pl].wall_work_mut().map(|s| &mut s.at),
        crate::handwork::SamWorkKind::Door => g.pl[pl].charging_mut().map(|s| &mut s.at),
        _ => None,
    };
    if let Some(at) = at {
        at[0] += dx;
        at[2] += dz;
    }
}

/// Kick a door in: one committed second, updateKick runs it; with a tool the big slot's beat in the boot's place.
pub fn start_kick(g: &mut crate::game::Game, d: DoorId, run: Option<bool>, tool: Option<crate::kit::BigTool>) {
    let run = run.unwrap_or(false);
    let pl = g.player;
    let c = g.pl[pl].ch;
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].planted() || crate::character::anim_kicking(g, c) {
        return;
    }
    if g.pl[pl].crouch && !can_stand(g) {
        return;
    }
    if let Some(t) = tool {
        let (pos, door_at, frame) = (g.ch[c].pos, crate::doors::door_pos(g, d), g.dr[d].frame_centre);
        if m::min(v3::dist_xz(door_at, pos), v3::dist_xz(frame, pos)) >= crate::doors::DOORS_USE_REACH && crate::bigkit::sam_big_spot(g, d, pos, t).is_none() {
            return;
        }
    }
    crate::deed::hands_free(g);
    g.pl[pl].crouch = false;
    g.sets[g.seat].hand.f_held = true;
    g.pl[pl].speed_sm = 0.0;
    g.pl[pl].door_kick_tail = false;
    let pos = g.ch[c].pos;
    let beside = match tool {
        Some(t) => crate::bigkit::sam_big_spot(g, d, pos, t),
        None => None,
    };
    // taken ONCE, off the leaf as it stands now
    let mut spot = beside.as_ref().map(|s| (s.pos, s.yaw));
    if spot.is_none() && !run {
        spot = side_kick_spot(g, d, pos).map(|s| (s.pos, s.yaw));
    }
    let (spot_pos, spot_yaw) = match spot {
        Some(s) => s,
        None => {
            let s = crate::doors::doors_kick_spot(g, d, pos);
            (s.pos, s.yaw)
        }
    };
    if let Some(t) = tool {
        let lead = crate::bigkit::sam_big_lead_of(g);
        let side = crate::bigkit::big_side(g, d, &pos);
        let need = crate::bigkit::hammer_blows(g, d, &pos);
        let at = match beside.as_ref().and_then(|s| s.at) {
            Some(at) => at,
            None => {
                let s = crate::bigkit::big_side(g, d, &pos);
                crate::shotgun::lock_point(g, d, s)
            }
        };
        let kind = t;
        crate::deed::begin(g, Deed::Kick(crate::game::PlayerKick {
            door: d,
            spot: spot_pos,
            yaw: spot_yaw,
            run: None,
            tool: Some(crate::game::PlayerKickTool {
                kind,
                side,
                t: -lead,
                lead: if lead > 0.0 { Some(lead) } else { None },
                n: 0.0,
                need,
                next: f64::INFINITY,
                done: -1.0,
                gave: None,
                moves: Some(crate::game::PlayerKickToolMoves { arrived: false, at, blows: Vec::new(), end: -1.0, via: beside.as_ref().and_then(|s| s.via) }),
            }),
        }));
        return;
    }
    crate::deed::begin(g, Deed::Kick(crate::game::PlayerKick { door: d, spot: spot_pos, yaw: spot_yaw, run: if run { Some(true) } else { None }, tool: None }));
    crate::character::anim_kick_door(g, c, None);
}

/// The spot a kick is thrown from BESIDE the door, for a kicker already beside its latch jamb; None = the square kick.
fn side_kick_spot(g: &crate::game::Game, d: DoorId, from: V3) -> Option<crate::player::SideKickSpot> {
    if g.dr[d].leaf.off {
        return None;
    }
    let f = crate::doors::doors_frame_coords(g, d, from);
    let width = g.dr[d].def.width;
    if f.along < width - 0.05 || f.along > width + 1.2 || f.off > 1.3 {
        return None;
    }
    let s = crate::doors::doors_kick_spot_side(g, d, from);
    if crate::scene_collision::nav_grid_is_blocked(g, s.pos[0], s.pos[2]) || crate::scene_collision::solid_blocked(g, [from[0], 0.5, from[2]], [s.pos[0], 0.5, s.pos[2]], None) {
        return None;
    }
    Some(crate::player::SideKickSpot { pos: s.pos, yaw: s.yaw })
}

/// The kick beat: eased to the kicking distance over the wind-up, rooted there; on the impact frame the lock goes.
pub(super) fn update_kick(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    // `spot` stays the same object in the TypeScript even if pl.kick is cleared under the walk: what it is read for below is taken here.
    let k = g.pl[pl].kick().unwrap();
    let (d, spot_at, spot_yaw) = (k.door, k.spot, k.yaw);
    let run = k.run.unwrap_or(false);
    let has_tool = k.tool.is_some();
    let walked = k.tool.as_ref().map_or(false, |t| t.moves.is_some());
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let mut yaw = spot_yaw;
    if walked {
        yaw = crate::bigkit::sam_big_walk(g, dt);
        collide_player(g, 1.7);
    } else if crate::character::anim_kick_time(g, c) < 0.45 {
        { let to = v3::lerp3(g.ch[c].pos, spot_at, 1.0 - m::exp(-12.0 * dt)); crate::character::pos_replace(g, c, to); }
        collide_player(g, 1.7);
    }
    keep_off_guards(g, dt, 1.7, None);
    let step_v = if walked && dt > 0.0 { m::hyp2(g.ch[c].pos[0] - x0, g.ch[c].pos[2] - z0) / dt } else { 0.0 };
    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, (if walked { 9.0 } else { 16.0 }) * dt);
    g.ch[c].aim_yaw = g.ch[c].body_yaw;
    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
    g.pl[pl].sprinting = false;
    g.pl[pl].crouch = false;
    g.pl[pl].backpedal = false;
    g.ch[c].anim.reverse = false;
    g.ch[c].anim.lockpick = false;
    g.ch[c].anim.speed = if step_v > 0.2 { m::min(step_v, 1.7) } else { 0.0 };
    g.ch[c].anim.crouch_target = 0.0;
    g.ch[c].anim.upper = if crate::game::player_pistol_up(g, pl) { crate::character::UpperMode::Relaxed } else { crate::character::UpperMode::None };
    g.ch[c].anim.big_tool = crate::bigkit::sam_big_pose(g, &crate::seats::here(g));
    let in_hands = crate::game::player_rifle_up(g, pl);
    arm_rifle(g, in_hands, Some(crate::character::LongStance::PortLong));
    crate::character::character_update(g, c, dt);
    if has_tool {
        if crate::bigkit::sam_big_tick(g, dt) {
            g.pl[pl].kick_over();
            crate::bigkit::sam_big_ended(g);
        }
        pose_gun_light(g);
        return;
    }
    let pos = g.ch[c].pos;
    let home = !run || crate::doors::doors_boot_holds(g, d, pos);
    let who = g.ch[c].id;
    let kick_k = g.k.sprintdoor.kick_k;
    if crate::character::anim_kick_impact(g, c) && !g.dr[d].leaf.off && run && home && crate::doorblast::tear_off(g, d, pos, who, Some(kick_k)) {
        g.stats.sprintdoor.kicks += 1;
        crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
    } else if crate::character::anim_kick_impact(g, c) && !g.dr[d].leaf.off && home {
        let pos = g.ch[c].pos;
        let gave = crate::doors::doors_kick_in(g, d, pos, who, None);
        let pos = g.ch[c].pos;
        crate::marks::kick_scuff(g, d, pos);
        if gave {
            crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
        }
    }
    if !crate::character::anim_kicking(g, c) {
        g.pl[pl].kick_over();
    } else if run && crate::character::anim_kick_time(g, c) >= m::max(g.k.sprintdoor.kick_free, 0.5) {
        g.pl[pl].kick_over();
        g.pl[pl].door_kick_tail = true;
    }
    pose_gun_light(g);
}
