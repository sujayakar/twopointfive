//! src/game/player.ts lines 2026 to 2300
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// (player.ts WOUNDED_SNAP_PX) px: the cursor this close to a wounded man's chest puts the reticle on him
const WOUNDED_SNAP_PX: f64 = 26.0;

/// Let go at once: the man is handed straight back, Sam's side cleared; past killCut with the knife in, he dies by it.   (player.ts freeHeld)
pub fn free_held(g: &mut crate::game::Game, why: crate::player::FreeHeldWhy) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let Some(h) = g.pl[pl].holding.as_ref() else { return };
    let (killing, h_t, h_g) = (hold_killing(h), h.t, h.g);
    let shot = why == crate::player::FreeHeldWhy::Shot;
    if shot && killing && h_t >= kill_cut(g) {
        kill_out(g);
        g.ch[c].anim.hold_pose = None;
        return;
    }
    hand_back(g, h_g, if shot { 0.6 } else { 0.3 });
    g.pl[pl].holding = None;
    g.ch[c].anim.hold_pose = None;
    if shot {
        let hc = g.gd[h_g].ch;
        let at = v3::add(g.ch[hc].pos, [0.0, 1.3, 0.0]);
        crate::audio_iface::game_audio_like_play(g, "shove", Some(at), Some(0.3), None);
        let text = alloc::format!("{} is loose", g.gd[h_g].callsign);
        crate::game::game_msg(g, &text);
    }
}

/// The hold simply ceases: Sam's side cleared, and his if he is still about.   (player.ts dropHold)
pub fn drop_hold(g: &mut crate::game::Game) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let Some(gd) = g.pl[pl].holding.as_ref().map(|h| h.g) else { return };
    if g.gd[gd].held.as_ref().map_or(false, |h| h.by == crate::game::GuardHeldBy::Player) {
        g.gd[gd].held = None;
    }
    let gc = g.gd[gd].ch;
    g.ch[gc].anim.held_pose = None;
    if g.gd[gd].state != crate::game::GuardState::Dead {
        let talk = g.gd[gd].talk;
        g.tab.game_talk[talk].held_since = -1.0;
    }
    g.pl[pl].holding = None;
    g.ch[c].anim.hold_pose = None;
}

/// The 'pick the lock' row: drop what the hands were doing and start working the lock.   (player.ts startPicking)
pub fn start_picking(g: &mut crate::game::Game, d: DoorId) {
    let pl = g.player;
    if g.pl[pl].down || g.pl[pl].takedown.is_some() || g.pl[pl].kick.is_some() {
        return;
    }
    if g.dr[d].def.steel.unwrap_or(false) {
        crate::game::game_msg(g, "the stair door has no keyhole on this side, so you can't pick it");
        return;
    }
    release_wall(g);
    if g.pl[pl].dragging.is_some() {
        crate::game::game_toggle_drag(g, None);
    }
    cancel_reloads(g, pl);
    g.pl[pl].throw_held = false;
    g.pl[pl].throw_preview = None;
    g.pl[pl].sprinting = false;
    g.pl[pl].easing = None;
    crate::wand::wand_drop(g, pl);
    g.pl[pl].picking = Some(d);
    g.pl[pl].crouch = true;
    if crate::doors::door_pick(g, d) <= 0.0 {
        crate::game::game_msg(g, "picking the lock. take a step or press Space to stop");
    }
}

/// Hands off the tools: the progress stays on the door, he stays on his knee.   (player.ts stopPicking)
pub fn stop_picking(g: &mut crate::game::Game, text: Option<&str>) {
    let pl = g.player;
    if g.pl[pl].picking.is_none() {
        return;
    }
    let c = g.pl[pl].ch;
    g.pl[pl].picking = None;
    g.ch[c].anim.lockpick = false;
    g.pl[pl].speed_sm = 0.0;
    if let Some(t) = text.filter(|t| !t.is_empty()) {
        crate::game::game_msg(g, t);
    }
}

/// Working the lock, on its own clock once begun.   (player.ts updatePicking)
pub fn update_picking(g: &mut crate::game::Game, dt: f64, input: InputId) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let d = g.pl[pl].picking.unwrap();
    let pos = g.ch[c].pos;
    if g.puppet.is_some() {
        stop_picking(g, None);
    } else if !crate::doors::doors_locked_out(g, d, pos, None) {
        let text = if crate::doors::door_locked(g, d) { Some("someone is coming through") } else { None };
        stop_picking(g, text);
    } else {
        let spot = crate::doors::doors_work_spot(g, d, pos, None, None);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].pos = v3::lerp3(g.ch[c].pos, spot.pos, 1.0 - m::exp(-10.0 * dt));
        collide_player(g, 1.0);
        keep_off_guards(g, dt, 1.0, None);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, spot.yaw, 12.0 * dt);
        g.ch[c].aim_yaw = g.ch[c].body_yaw;
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
        g.pl[pl].crouch = true;
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
            stop_picking(g, Some("the lock gives — it opens like any other door now"));
        }
    }
    let in_hands = crate::game::player_rifle_up(g, pl) && !g.ch[c].anim.hide_held_item;
    arm_rifle(g, in_hands, Some(crate::character::LongStance::LowLong));
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
}

/// The 'look under it' row of a shut door: the hands' work dropped, onto his knee at the middle of the leaf.   (player.ts startWand)
pub fn start_wand(g: &mut crate::game::Game, d: DoorId) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if g.pl[pl].down || g.pl[pl].takedown.is_some() || g.pl[pl].kick.is_some() || g.pl[pl].holding.is_some() || g.pl[pl].wanding.is_some() {
        return;
    }
    if g.pl[pl].coding.is_some() || g.pl[pl].door_move.is_some() || g.pl[pl].breaker_move.is_some() || g.mission.drive_t >= 0.0 || crate::handwork::sam_work(g).is_some() {
        return;
    }
    if !crate::wand::wand_offered(g, pl, d) {
        if g.pl[pl].wand {
            let text = if crate::wand::wand_fits(g, d) {
                "nothing here the wand would show you"
            } else if g.dr[d].def.steel.unwrap_or(false) {
                "no gap under a steel fire door"
            } else {
                "the wand goes under a shut door"
            };
            crate::game::game_msg(g, text);
        }
        return;
    }
    release_wall(g);
    if g.pl[pl].dragging.is_some() {
        crate::game::game_toggle_drag(g, None);
    }
    cancel_reloads(g, pl);
    g.pl[pl].throw_held = false;
    g.pl[pl].throw_preview = None;
    g.pl[pl].sprinting = false;
    g.pl[pl].easing = None;
    g.pl[pl].picking = None;
    let pos = g.ch[c].pos;
    let side = crate::doors::doors_side(g, d, pos);
    g.pl[pl].wanding = Some(crate::wand::Wanding { door: d, side, t: 0.0, pan: 0.0, live: false, off: 0.0, age: 0.0 });
    g.pl[pl].crouch = true;
    g.stats.wand.looks += 1;
}

/// The wand pulled out: he stays on his knee.   (player.ts stopWand)
pub fn stop_wand(g: &mut crate::game::Game, text: Option<&str>) {
    let pl = g.player;
    if g.pl[pl].wanding.is_none() {
        return;
    }
    crate::wand::wand_drop(g, pl);
    g.pl[pl].speed_sm = 0.0;
    if let Some(t) = text.filter(|t| !t.is_empty()) {
        crate::game::game_msg(g, t);
    }
}

/// One frame of the look: planted on his knee at the leaf; once there the lens is live and turns after the aim point.   (player.ts updateWanding)
pub fn update_wanding(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    // The TypeScript's W outlives a stop (its object is only dropped from pl.wanding), so a local copy written back while he still has the look does the same.
    let mut w = g.pl[pl].wanding.unwrap();
    let d = w.door;
    let pos = g.ch[c].pos;
    if g.puppet.is_some() || !crate::fogofwar::fog_of_war_active(g) {
        stop_wand(g, None);
    } else if !crate::wand::wand_fits(g, d) || crate::doors::doors_side(g, d, pos) != w.side {
        stop_wand(g, Some("the door moved — the wand is out"));
    } else if crate::samstun::sam_staggered(g, None) || g.time < g.pl[pl].dazzled_until {
        stop_wand(g, None);
    } else {
        let along = g.dr[d].def.width * 0.5;
        let spot = crate::doors::doors_work_spot(g, d, pos, Some(0.5), Some(along));
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].pos = v3::lerp3(g.ch[c].pos, spot.pos, 1.0 - m::exp(-10.0 * dt));
        collide_player(g, 1.0);
        keep_off_guards(g, dt, 1.0, None);
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, spot.yaw, 12.0 * dt);
        g.ch[c].aim_yaw = g.ch[c].body_yaw;
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
        g.pl[pl].crouch = true;
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
            w.t += dt;
            let crouch = g.ch[c].anim.crouch;
            if !w.live && crate::wand::wand_through(g, w.t, w.age, crouch, Some(true)) {
                w.live = true;
                g.stats.wand.live += 1;
                wand_row_back(g, pl, w);
                crate::game::game_msg(g, "the wand is through");
            }
        } else if w.live {
            wand_row_back(g, pl, w);
            stop_wand(g, Some("shoved off it — the wand is out"));
        } else if off_spot >= 0.12 && {
            w.off += dt;
            w.off >= g.k.wand.give_up_secs
        } {
            wand_row_back(g, pl, w);
            stop_wand(g, Some("no room to kneel at that door"));
        }
        wand_row_back(g, pl, w);
        if g.pl[pl].wanding.is_some() && w.live {
            let aim = g.aim_point;
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
    if let Some(row) = g.pl[pl].wanding.as_mut() {
        *row = w;
    }
}

/// Keying a code in: stood at the pad with the feet still, turned to the plate, the rifle low. HANDWORK.on and .work are settled on.   (player.ts updateCodingPlant)
pub fn update_coding_plant(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let coding = g.pl[pl].coding.unwrap();
    let k = crate::devices::devices_pad_of(g, coding);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    if let Some(k) = &k {
        if g.pl[pl].wall.is_some() {
            release_wall(g);
        }
        let hand = crate::handwork::sam_work_hand(g);
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

/// Hands at work on a thing that is not a lock or a pad: the keypad's plant.   (player.ts updateWorkPlant)
pub fn update_work_plant(g: &mut crate::game::Game, dt: f64, w: &mut crate::handwork::SamWork) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    if g.pl[pl].wall.is_some() {
        release_wall(g);
    }
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let (x0, z0) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let hand = crate::handwork::sam_work_hand(g);
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

/// W.at[0] += dx; W.at[2] += dz. W.at IS the verb's owner's own vector in the TypeScript (handwork.ts samWork): the write goes to that owner, by the kind of work.
fn work_at_walked(g: &mut crate::game::Game, pl: PlayerId, kind: crate::handwork::SamWorkKind, dx: f64, dz: f64) {
    let at = match kind {
        crate::handwork::SamWorkKind::Wall => g.wall_charge.sam.as_mut().map(|s| &mut s.at),
        crate::handwork::SamWorkKind::Door => g.pl[pl].charging.as_mut().map(|s| &mut s.at),
        crate::handwork::SamWorkKind::Defuse => g.defuse.sam.as_mut().map(|s| &mut s.at),
        _ => None,
    };
    if let Some(at) = at {
        at[0] += dx;
        at[2] += dz;
    }
}

/// Kick a door in: one committed second, updateKick runs it; with a tool the big slot's beat in the boot's place.   (player.ts startKick)
pub fn start_kick(g: &mut crate::game::Game, d: DoorId, run: Option<bool>, tool: Option<crate::player::StartKickTool>) {
    let run = run.unwrap_or(false);
    let pl = g.player;
    let c = g.pl[pl].ch;
    if g.pl[pl].kick.is_some() || g.pl[pl].down || g.pl[pl].takedown.is_some() || crate::character::anim_kicking(g, c) {
        return;
    }
    if g.pl[pl].crouch && !can_stand(g) {
        crate::game::game_msg(g, "no room to stand for a kick here");
        return;
    }
    if let Some(t) = tool {
        let (pos, door_at, frame) = (g.ch[c].pos, crate::doors::door_pos(g, d), g.dr[d].frame_centre);
        if m::min(v3::dist_xz(door_at, pos), v3::dist_xz(frame, pos)) >= seam_use_reach(g) && crate::bigkit::sam_big_spot(g, d, pos, big_spot_tool(t)).is_none() {
            return;
        }
    }
    release_wall(g);
    if g.pl[pl].dragging.is_some() {
        crate::game::game_toggle_drag(g, None);
    }
    g.pl[pl].picking = None;
    crate::wand::wand_drop(g, pl);
    g.ch[c].anim.lockpick = false;
    g.pl[pl].easing = None;
    g.pl[pl].sprinting = false;
    g.pl[pl].crouch = false;
    g.pl[pl].throw_held = false;
    g.pl[pl].throw_preview = None;
    g.pl[pl].f_held = true;
    g.pl[pl].speed_sm = 0.0;
    cancel_reloads(g, pl);
    g.pl[pl].door_kick_tail = false;
    let pos = g.ch[c].pos;
    let beside = match tool {
        Some(t) => crate::bigkit::sam_big_spot(g, d, pos, big_spot_tool(t)),
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
        let kind = match t {
            crate::player::StartKickTool::Shotgun => crate::game::PlayerKickToolKind::Shotgun,
            crate::player::StartKickTool::Hammer => crate::game::PlayerKickToolKind::Hammer,
        };
        g.pl[pl].kick = Some(crate::game::PlayerKick {
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
        });
        return;
    }
    g.pl[pl].kick = Some(crate::game::PlayerKick { door: d, spot: spot_pos, yaw: spot_yaw, run: if run { Some(true) } else { None }, tool: None });
    crate::character::anim_kick_door(g, c, None);
}

fn big_spot_tool(t: crate::player::StartKickTool) -> crate::bigkit::SamBigSpotTool {
    match t {
        crate::player::StartKickTool::Shotgun => crate::bigkit::SamBigSpotTool::Shotgun,
        crate::player::StartKickTool::Hammer => crate::bigkit::SamBigSpotTool::Hammer,
    }
}

/// The spot a kick is thrown from BESIDE the door, for a kicker already beside its latch jamb; None = the square kick.   (player.ts sideKickSpot)
pub fn side_kick_spot(g: &mut crate::game::Game, d: DoorId, from: V3) -> Option<crate::player::SideKickSpot> {
    if crate::doors::door_off(g, d) {
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

/// The kick beat: eased to the kicking distance over the wind-up, rooted there; on the impact frame the lock goes.   (player.ts updateKick)
pub fn update_kick(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    // `spot` stays the same object in the TypeScript even if pl.kick is cleared under the walk: what it is read for below is taken here.
    let k = g.pl[pl].kick.as_ref().unwrap();
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
        g.ch[c].pos = v3::lerp3(g.ch[c].pos, spot_at, 1.0 - m::exp(-12.0 * dt));
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
    g.ch[c].anim.big_tool = crate::bigkit::sam_big_pose(g).map(|p| crate::character::AnimBigTool {
        kind: match p.kind {
            crate::bigkit::SamBigPoseKind::Shotgun => crate::character::AnimBigToolKind::Shotgun,
            crate::bigkit::SamBigPoseKind::Hammer => crate::character::AnimBigToolKind::Hammer,
        },
        t: p.t,
        equip: p.equip,
        windup: p.windup,
        stow: p.stow,
        blows: p.blows,
        end: p.end,
        at: p.at,
        snug: Some(p.snug),
    });
    let in_hands = crate::game::player_rifle_up(g, pl);
    arm_rifle(g, in_hands, Some(crate::character::LongStance::PortLong));
    crate::character::character_update(g, c, dt);
    if has_tool {
        if crate::bigkit::sam_big_tick(g, dt) {
            g.pl[pl].kick = None;
            crate::bigkit::sam_big_ended(g);
        }
        pose_gun_light(g);
        return;
    }
    let pos = g.ch[c].pos;
    let home = !run || crate::doors::doors_boot_holds(g, d, pos);
    let who = g.ch[c].id;
    let kick_k = g.k.sprintdoor.kick_k;
    if crate::character::anim_kick_impact(g, c) && !crate::doors::door_off(g, d) && run && home && crate::doorblast::tear_off(g, d, pos, who, Some(kick_k)) {
        g.stats.sprintdoor.kicks += 1;
        crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
    } else if crate::character::anim_kick_impact(g, c) && !crate::doors::door_off(g, d) && home {
        let pos = g.ch[c].pos;
        let gave = crate::doors::doors_kick_in(g, d, pos, who, None);
        let pos = g.ch[c].pos;
        crate::marks::kick_scuff(g, d, pos);
        if gave {
            crate::game::game_msg(g, "you kicked it in, and the whole floor heard that");
            crate::chronicle::chron_door(g, crate::chronicle::ChronDoorKind::DoorKicked, d);
        }
    }
    if !crate::character::anim_kicking(g, c) {
        g.pl[pl].kick = None;
    } else if run && crate::character::anim_kick_time(g, c) >= m::max(g.k.sprintdoor.kick_free, 0.5) {
        g.pl[pl].kick = None;
        g.pl[pl].door_kick_tail = true;
    }
    pose_gun_light(g);
}

/// Planar distance from the player to a corpse: the nearer of its ankles, its hips and its grip.   (player.ts bodyDist)
pub fn body_dist(g: &mut crate::game::Game, gd: GuardId, within: Option<f64>) -> f64 {
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

/// Throw origin, right hand-ish.   (player.ts throwOrigin)
pub fn throw_origin(g: &mut crate::game::Game) -> V3 {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let f = crate::character::character_forward(g, c);
    let pos = g.ch[c].pos;
    [pos[0] + f[0] * 0.32, pos[1] + (if g.pl[pl].crouch { 1.05 } else { 1.5 }), pos[2] + f[2] * 0.32]
}

/// The wind-up of a throw: the can counted now, it leaves the hand at THROW_RELEASE along the arc previewed.   (player.ts doThrow)
pub fn do_throw(g: &mut crate::game::Game, sol: &crate::throwables::ThrowSolution, kind: crate::game::ThrowKind) {
    let pl = g.player;
    if crate::game::game_throw_count(g, kind) <= 0.0 {
        let text = match kind {
            crate::game::ThrowKind::Frag => "no frag grenades left",
            crate::game::ThrowKind::Flash => "no flashbangs left",
            _ => "no smoke canisters left",
        };
        crate::game::game_msg(g, text);
        return;
    }
    if g.pl[pl].pending_throw.is_some() {
        return;
    }
    match kind {
        crate::game::ThrowKind::Frag => g.pl[pl].frags -= 1.0,
        crate::game::ThrowKind::Flash => g.pl[pl].flashbangs -= 1.0,
        _ => g.pl[pl].canisters -= 1.0,
    }
    let c = g.pl[pl].ch;
    let dist = v3::dist_xz(g.ch[c].pos, sol.land);
    crate::character::anim_throw_item(g, c, Some(dist), Some(m::hyp2(sol.v0[0], sol.v0[2])));
    g.pl[pl].pending_throw = Some(crate::game::PlayerPendingThrow { t: crate::character::THROW_RELEASE, sol: sol.clone(), kind });
    let at = throw_origin(g);
    crate::audio_iface::game_audio_like_play(g, "throw", Some(at), Some(0.7), None);
}

/// The gun's heading eased to the cursor's bearing.   (player.ts aimEase)
pub fn aim_ease(yaw: f64, to_aim: V3, dt: f64) -> f64 {
    yaw + v3::wrap_angle(v3::yaw_of(to_aim) - yaw) * (1.0 - m::exp(-16.0 * dt))
}

/// The aim of a cursor whose ray met nothing: level, a metre ahead of the way his body faces.   (player.ts aimLevel)
pub fn aim_level(c_2: crate::player::AimLevelC) -> V3 {
    v3::add(v3::add(c_2.pos, [m::sin(c_2.body_yaw), 0.0, m::cos(c_2.body_yaw)]), [0.0, 1.15, 0.0])
}

/// What the cursor's ray meets: the first thing of the world on it, never the man's own body.   (player.ts cursorHit)
pub fn cursor_hit(g: &mut crate::game::Game, ray: crate::player::CursorHitRay, own: f64) -> Option<crate::scene_boxes::BoxWorldRaycast> {
    crate::scene_boxes::box_world_raycast(g, ray.ro, ray.rd, Some(200.0), None, Some(own), None)
}

/// The point the cursor's ray gives: where it hit, else the floor under it, else None.   (player.ts cursorPoint)
pub fn cursor_point(g: &mut crate::game::Game, u: f64, v: f64, ray: crate::player::CursorPointRay, hit: Option<crate::player::CursorPointHit>) -> Option<V3> {
    match hit {
        Some(h) => Some(v3::mad(ray.ro, ray.rd, h.t)),
        None => crate::camera::follow_camera_ground_point(g, u, v, Some(0.0)),
    }
}

/// The frame's aim: the cursor's point, the interaction rows, then the reticle's snap to a man or a light.   (player.ts updateAim)
pub fn update_aim(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) {
    let puppet_aim = g.puppet.as_ref().and_then(|p| p.aim);
    if let Some(aim) = puppet_aim {
        let (sx, sy, front) = crate::camera::follow_camera_project(g, aim, canvas.client_width, canvas.client_height, Some(f64::INFINITY), Some(1e-3));
        if front {
            g.tab.input[input].mouse_x = sx;
            g.tab.input[input].mouse_y = sy;
        }
    }
    let (mx, my) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    let u = mx / m::max(1.0, canvas.client_width);
    let v = my / m::max(1.0, canvas.client_height);
    let ray = crate::camera::follow_camera_ray(g, u, v);
    let me = g.pl[g.player].ch;
    let own = g.ch[me].id;
    let hit = cursor_hit(g, crate::player::CursorHitRay { ro: ray.ro, rd: ray.rd }, own);
    let hit_t = hit.as_ref().map(|h| h.t);
    let hit_index = hit.as_ref().map(|h| h.index);
    let p = cursor_point(g, u, v, crate::player::CursorPointRay { ro: ray.ro, rd: ray.rd }, hit_t.map(|t| crate::player::CursorPointHit { t }));
    let p = match p {
        Some(p) => p,
        None => aim_level(crate::player::AimLevelC { pos: g.ch[me].pos, body_yaw: g.ch[me].body_yaw }),
    };
    let p = match puppet_aim {
        Some(aim) => v3::copy(aim),
        None => p,
    };
    g.aim_point = p;
    let (w, h) = (canvas.client_width, canvas.client_height);
    build_interactables(g, mx, my, w, h);
    // A light TAKEN is KEPT while the cursor stays where it took it.
    let reach = m::min(crate::aim::SNAP_PX, g.k.aim.player.light_snap_px);
    let at = g.m.player.light_at.get(g.player).copied();
    let kept = match (g.aim_target, at) {
        (Some(t), Some(a)) if m::hyp2(mx - a[0], my - a[1]) <= reach => Some(t),
        _ => None,
    };
    let mut on_it = false;
    let mut best_px = crate::aim::SNAP_PX;
    g.aim_target = None;
    g.aim_guard = None;
    g.aim_snap = None;
    g.aim_friend = None;
    if !g.puppet.as_ref().map_or(false, |p| !p.fixtures.unwrap_or(false)) {
        let mut i = 0;
        while i < g.interactables.len() {
            let it = &g.interactables[i];
            i += 1;
            if it.kind != crate::game::InteractableKind::Light {
                continue;
            }
            let Some(target) = it.target else { continue };
            let pos = it.pos;
            let (sx, sy, front) = crate::camera::follow_camera_project(g, pos, w, h, None, None);
            if !front {
                continue;
            }
            let d = m::hyp2(sx - mx, sy - my);
            if d >= best_px {
                continue;
            }
            let on = d < reach || hit_index.map_or(false, |x| g.tab.sim_lightshome_fixture[target].boxes.contains(&x));
            if !on && Some(target) != kept {
                continue;
            }
            best_px = d;
            g.aim_target = Some(target);
            g.aim_snap = Some(v3::copy(pos));
            on_it = on;
        }
    }
    let held = g.pl[g.player].holding.as_ref().map(|h| h.g);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || Some(gd) == held || crate::fogofwar::fog_of_war_hidden(g, gd) || crate::friends::friend_no_snap(g, gd) {
            continue;
        }
        let com = chest_or(g, gd, 1.2);
        let (sx, sy, front) = crate::camera::follow_camera_project(g, com, w, h, None, None);
        if !front {
            continue;
        }
        let d = m::hyp2(sx - mx, sy - my) - 6.0;
        let eye = v3::add(g.ch[me].pos, [0.0, 1.3, 0.0]);
        if d < best_px && !crate::scene_collision::segment_blocked(g, eye, com, None, None, None) {
            best_px = d;
            g.aim_guard = Some(gd);
            g.aim_target = None;
            g.aim_snap = Some(com);
        }
    }
    // a man down WOUNDED takes the reticle only with the cursor ON him
    if g.aim_guard.is_none() {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if g.gd[gd].wound.is_none() || Some(gd) == held || crate::fogofwar::fog_of_war_hidden(g, gd) || crate::friends::friend_no_snap(g, gd) {
                continue;
            }
            let com = chest_or(g, gd, 0.6);
            let (sx, sy, front) = crate::camera::follow_camera_project(g, com, w, h, None, None);
            if !front {
                continue;
            }
            let d = m::hyp2(sx - mx, sy - my);
            let eye = v3::add(g.ch[me].pos, [0.0, 1.3, 0.0]);
            if d < m::min(best_px, WOUNDED_SNAP_PX) && !crate::scene_collision::segment_blocked(g, eye, com, None, None, None) {
                best_px = d;
                g.aim_guard = Some(gd);
                g.aim_target = None;
                g.aim_snap = Some(com);
            }
        }
    }
    // the exact ray still counts when it lands ON a man the loose test missed; the NEAREST body on the ray decides
    if g.aim_guard.is_none() {
        let mut best_t = hit_t.unwrap_or(1e9);
        let mut friend_t = f64::INFINITY;
        let (lit, lit_at) = (g.aim_target, g.aim_snap);
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            i += 1;
            if Some(gd) == held || crate::fogofwar::fog_of_war_hidden(g, gd) {
                continue;
            }
            let c = g.gd[gd].ch;
            let t = crate::character::character_ray_hit(g, c, ray.ro, ray.rd, best_t + 0.5);
            if !(t >= 0.0 && t < best_t + 0.5) {
                continue;
            }
            if crate::friends::friend_on_ray(g, gd) {
                if t < friend_t {
                    friend_t = t;
                    g.aim_friend = Some(gd);
                }
                if crate::friends::friend_no_snap(g, gd) {
                    continue;
                }
            }
            g.aim_guard = Some(gd);
            g.aim_target = None;
            g.aim_snap = Some(chest_or(g, gd, 1.2));
            best_t = t;
        }
        if let (Some(friend), Some(guard)) = (g.aim_friend, g.aim_guard) {
            if guard != friend {
                if friend_t >= best_t {
                    g.aim_friend = None;
                } else if crate::friends::friend_no_snap(g, friend) {
                    g.aim_guard = None;
                    g.aim_target = lit;
                    g.aim_snap = lit_at;
                }
            }
        }
    }
    if crate::samstun::sam_blind(g, None) > 0.5 {
        g.aim_guard = None;
        g.aim_target = None;
        g.aim_snap = None;
    }
    if let Some(snap) = g.aim_snap {
        if puppet_aim.is_none() {
            g.aim_point = v3::copy(snap);
        }
    }
    if g.aim_target.is_none() {
        g.m.player.light_at.delete(g.player);
    } else if on_it || at.is_none() {
        g.m.player.light_at.set(g.player, [mx, my]);
    }
}

/// A man's centre of mass: his chest bone, or his feet raised by `up` before he has bones.
fn chest_or(g: &crate::game::Game, gd: GuardId, up: f64) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Chest) {
        Some(b) => *b,
        None => v3::add(g.ch[c].pos, [0.0, up, 0.0]),
    }
}

// ---- seams: what skeleton 3 cannot give yet

/// Doors.USE_REACH: a class's static number the skeleton does not carry
fn seam_use_reach(g: &mut crate::game::Game) -> f64 {
    todo!("needs: Doors.USE_REACH")
}

#[cfg(test)]
mod tests {
    use super::*;
    /// bits printed by the TypeScript under EXACT
    #[test]
    fn aim_ease_is_the_typescripts_bits() {
        assert_eq!(aim_ease(0.3, [1.0, 0.0, 2.0], 1.0 / 60.0).to_bits(), 0x3fd5a6cb23466713);
        assert_eq!(aim_ease(-3.1, [-0.7, 0.2, -0.4], 1.0 / 30.0).to_bits(), 0xc00575bcbc220316);
        assert_eq!(aim_ease(3.0, [-0.1, 0.0, -1.0], 0.25).to_bits(), 0x4009e50da7aef3a5);
    }
    #[test]
    fn aim_level_is_the_typescripts_bits() {
        let a = aim_level(crate::player::AimLevelC { pos: [1.5, 0.0, -2.25], body_yaw: 0.7 });
        assert_eq!(a.map(f64::to_bits), [0x4001275b9a51866e, 0x3ff2666666666666, 0xbff7c334d6b03137]);
        let b = aim_level(crate::player::AimLevelC { pos: [-12.125, 1.05, 33.3], body_yaw: -2.4 });
        assert_eq!(b.map(f64::to_bits), [0xc02999d64f5c3d1a, 0x400199999999999a, 0x404048037b942320]);
    }
}
