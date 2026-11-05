//! src/game/friends.ts lines 1 to 89
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The reticle passes this man over: he is not the player's enemy.   (friends.ts friendNoSnap)
pub fn friend_no_snap(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player)
}

/// A man of his own side whose body the cursor's ray should know it landed on.   (friends.ts friendOnRay)
pub fn friend_on_ray(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player)
}

/// Is anybody in a fight, as far as Sam's own gun is concerned.   (friends.ts friendsInFight)
pub fn friends_in_fight(g: &mut crate::game::Game) -> bool {
    let quiet_s = g.k.friends.quiet_s;
    if g.time - g.pl[g.player].last_fire_t < quiet_s { return true; }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || !g.ch[g.gd[gd].ch].alive { continue; }
        if crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player) {
            let state = g.gd[gd].state;
            if state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search { return true; }
        } else if g.gd[gd].unit.is_some() && g.time - crate::roomfight::room_fight_hot_of(g, gd) < quiet_s {
            return true;
        }
    }
    false
}

/// The friend nearest along the line Sam's gun is held on, as far as what it is aimed at and `reachM` at most.   (friends.ts friendInSamsLine)
pub fn friend_in_sams_line(g: &mut crate::game::Game, c_2: CharId) -> Option<GuardId> {
    let from = bone_or(g, c_2, crate::character::Bone::UpperArmR, 1.4);
    let aim_yaw = g.ch[c_2].aim_yaw;
    let (dx, dz) = (m::sin(aim_yaw), m::cos(aim_yaw));
    let reach = m::min(g.k.friends.reach_m, m::hyp2(g.aim_point[0] - from[0], g.aim_point[2] - from[2]) + g.k.friends.past_m);
    let mut line = SamsLine { from, dx, dz, reach, best: None, bt: m::INF };
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if !crate::teams::hostile_to(g, o, crate::teams::HostileToO::Player) { ask(g, o, &mut line); }
    }
    if g.rescue.active {
        let mut i = 0;
        while i < g.civilians.len() {
            let o = g.civilians[i];
            i += 1;
            if crate::rescue::rescue_is_hostage(g, o) { ask(g, o, &mut line); }
        }
    }
    line.best
}

/// …or the friend the CURSOR is on, only while the aim is free.   (friends.ts friendUnderCursor)
pub fn friend_under_cursor(g: &mut crate::game::Game, c_2: CharId) -> Option<GuardId> {
    let o = g.aim_friend?;
    if g.aim_snap.is_some() || g.gd[o].state == crate::game::GuardState::Dead || !g.ch[g.gd[o].ch].alive { return None; }
    let from = bone_or(g, c_2, crate::character::Bone::UpperArmR, 1.4);
    let chest = bone_or(g, g.gd[o].ch, crate::character::Bone::Chest, 1.2);
    if m::hyp2(chest[0] - from[0], chest[2] - from[2]) <= g.k.friends.reach_m && !crate::scene_collision::segment_blocked(g, from, chest, None, None, None) { Some(o) } else { None }
}

/// This frame's answer: is his raised gun down for a friend. `Guard` = the man it is down for, `None` = down inside the beat after he left the line,
/// `Bool(false)` = not lowered.   (friends.ts samOffFriend)
pub fn sam_off_friend(g: &mut crate::game::Game, c_2: CharId, up: bool, dt: f64) -> Option<crate::friends::SamOffFriend> {
    let pl = g.player;
    let live = up
        && !g.touring
        && !g.puppet.is_some()
        && !g.pl[pl].wall.is_some()
        && !g.pl[pl].holding.is_some()
        && !g.pl[pl].dragging.is_some()
        && !friends_in_fight(g);
    let o = if live { friend_in_sams_line(g, c_2).or_else(|| friend_under_cursor(g, c_2)) } else { None };
    if o.is_some() {
        if !g.pl[pl].friend_low { g.stats.friends.lowered += 1; }
        g.pl[pl].friend_low = true;
        g.pl[pl].friend_t = g.time;
    } else if g.pl[pl].friend_low && (!live || !(g.time - g.pl[pl].friend_t >= 0.0 && g.time - g.pl[pl].friend_t <= g.k.friends.up_secs)) {
        g.pl[pl].friend_low = false;
    }
    if g.pl[pl].friend_low { seam_stats_lowered_s_add(g, dt); }
    if g.pl[pl].friend_low {
        match o {
            Some(o) => Some(crate::friends::SamOffFriend::Guard(o)),
            None => None,
        }
    } else {
        Some(crate::friends::SamOffFriend::Bool(false))
    }
}

/// What `ask` of friendInSamsLine closed over.
struct SamsLine {
    from: V3,
    dx: f64,
    dz: f64,
    reach: f64,
    best: Option<GuardId>,
    bt: f64,
}

/// The closure `ask` of friendInSamsLine: one man against the line, the nearest so far kept in `line`.   (friends.ts:52)
fn ask(g: &mut Game, o: GuardId, line: &mut SamsLine) {
    let c = g.gd[o].ch;
    if g.gd[o].state == crate::game::GuardState::Dead || !g.ch[c].alive { return; }
    let h = match g.ch[c].bones.get(crate::character::Bone::Hips) { Some(h) => *h, None => g.ch[c].pos };
    let (rx, rz) = (h[0] - line.from[0], h[2] - line.from[2]);
    let t = rx * line.dx + rz * line.dz;
    if t < 0.15 || t > line.reach || t >= line.bt { return; }
    let vel = g.ch[c].vel;
    let clear = g.k.friends.clear_m + m::min(g.k.friends.lead_max_m, g.k.friends.lead_s * m::hyp2(vel[0], vel[2]));
    if m::abs(rx * line.dz - rz * line.dx) > clear { return; }
    let chest = bone_or(g, c, crate::character::Bone::Chest, 1.2);
    if crate::scene_collision::segment_blocked(g, line.from, chest, None, None, None) { return; }
    line.bt = t;
    line.best = Some(o);
}

/// `c.bones.<bone> ?? v3.add(c.pos, [0, up, 0])`, by value: nothing here moves it.
fn bone_or(g: &Game, c: CharId, bone: crate::character::Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(bone) {
        Some(b) => *b,
        None => v3::add(g.ch[c].pos, [0.0, up, 0.0]),
    }
}

/// `FRIENDS_STATS.loweredS += dt`: the skeleton has the field as i32, the TypeScript sums seconds.   (friends.ts:86)
fn seam_stats_lowered_s_add(g: &mut Game, dt: f64) {
    todo!("needs: FriendsStats.lowered_s as f64")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One player, one body, no men: nothing below reaches another part's stub.
    fn lone_player() -> (Box<Game>, CharId) {
        let mut g = Box::new(Game::blank());
        let c = g.ch.push(crate::character::Character::blank());
        g.pl.push(crate::game::Player::blank());
        g.time = 100.0;
        g.pl[g.player].last_fire_t = -10.0;
        (g, c)
    }

    fn not_lowered(a: Option<SamOffFriend>) -> bool { matches!(a, Some(SamOffFriend::Bool(false))) }

    #[test]
    fn his_own_round_is_a_fight_for_quiet_s() {
        let (mut g, _) = lone_player();
        assert!(!friends_in_fight(&mut g));
        g.pl[g.player].last_fire_t = g.time - g.k.friends.quiet_s + 0.01;
        assert!(friends_in_fight(&mut g));
        g.pl[g.player].last_fire_t = g.time - g.k.friends.quiet_s;
        assert!(!friends_in_fight(&mut g));
    }

    #[test]
    fn a_gun_not_up_is_up_again_at_once() {
        let (mut g, c) = lone_player();
        let p = g.player;
        assert!(not_lowered(sam_off_friend(&mut g, c, false, 0.016)));
        assert!(!g.pl[p].friend_low);
        g.pl[p].friend_low = true;
        g.pl[p].friend_t = g.time;
        assert!(not_lowered(sam_off_friend(&mut g, c, false, 0.016)));
        assert!(!g.pl[p].friend_low);
    }

    #[test]
    fn the_floor_going_loud_ends_the_beat_at_once() {
        let (mut g, c) = lone_player();
        let p = g.player;
        g.pl[p].friend_low = true;
        g.pl[p].friend_t = g.time;
        g.pl[p].last_fire_t = g.time;
        assert!(not_lowered(sam_off_friend(&mut g, c, true, 0.016)));
        assert!(!g.pl[p].friend_low);
    }

    #[test]
    fn the_beat_runs_out_past_up_secs_and_on_a_clock_gone_back() {
        let (mut g, c) = lone_player();
        let p = g.player;
        g.pl[p].friend_low = true;
        g.pl[p].friend_t = g.time - g.k.friends.up_secs - 0.01;
        assert!(not_lowered(sam_off_friend(&mut g, c, true, 0.016)));
        assert!(!g.pl[p].friend_low);
        g.pl[p].friend_low = true;
        g.pl[p].friend_t = g.time + 1.0;
        assert!(not_lowered(sam_off_friend(&mut g, c, true, 0.016)));
        assert!(!g.pl[p].friend_low);
    }

    #[test]
    fn nobody_under_a_free_cursor_is_nobody() {
        let (mut g, c) = lone_player();
        assert!(friend_under_cursor(&mut g, c).is_none());
        assert!(friend_in_sams_line(&mut g, c).is_none());
    }
}
