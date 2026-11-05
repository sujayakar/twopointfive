//! src/game/teampace.ts lines 1 to 273
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `TEAMPACE_STATS.secs += dt`, teampace.ts:198: the stats field is i32 and this is a sum of seconds.
fn seam_stats_secs(g: &mut Game, dt: f64) { todo!("needs: TeampaceStats.secs as f64") }

/// this teammate's pace is the rule's this frame   (teampace.ts teamSlow)
pub fn team_slow(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.k.teampace.on && crate::guards::team_paced(g, gd)
}

/// the point his gun leads to while he has no line to the order's watch, or None while he sees it   (teampace.ts walkLead)
pub fn walk_lead(g: &mut crate::game::Game, gd: GuardId, aim: V3) -> Option<V3> {
    if !g.k.teampace.walk_aim || !crate::guards::team_paced(g, gd) { return None; }
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let y = 1.45 - 0.45 * g.ch[c].anim.crouch;
    let (dx, dy, dz) = (aim[0] - pos[0], aim[1] - y, aim[2] - pos[2]);
    let l = m::hyp3(dx, dy, dz);
    let t = g.time;
    let k = if l > AIM_SHORT + 0.1 { (l - AIM_SHORT) / l } else { 0.0 };
    let shut = k > 0.0 && crate::scene_collision::segment_blocked(g, [pos[0], y, pos[2]], [pos[0] + dx * k, y + dy * k, pos[2] + dz * k], None, None, None);
    let (s_blind, s_since) = match g.m.teampace.aim_seen.get(gd).copied() {
        Some(was) if t >= was.t && t - was.t <= 0.25 && m::hyp2(was.x - aim[0], was.z - aim[2]) < 0.5 => (was.blind, was.since),
        _ => (shut, t),
    };
    let since = if shut == s_blind { t } else { s_since };
    let wait = if shut { g.k.teampace.walk_aim_off } else { g.k.teampace.walk_aim_on };
    let blind = if shut != s_blind && t - since >= wait - 1e-9 { shut } else { s_blind };
    let kept_since = if blind == s_blind { since } else { t };
    g.m.teampace.aim_seen.set(gd, TeampaceModAimSeenValue { t, x: aim[0], z: aim[2], blind, since: kept_since });
    if !blind { return None; }
    let yaw = g.ch[c].body_yaw;
    Some([pos[0] + m::sin(yaw) * WALK_AHEAD, y, pos[2] + m::cos(yaw) * WALK_AHEAD])
}

/// a teammate's walk, m/s   (teampace.ts teamWalk)
pub fn team_walk(g: &mut crate::game::Game) -> f64 {
    g.tune.player_walk * g.k.teampace.k
}

/// is the player, at `s` and doing (vx, vz) of speed `v`, moving and at that spot   (teampace.ts teamWalkOf's `toward`)
fn toward(s: V3, vx: f64, vz: f64, v: f64, pace_cos: f64, x: f64, z: f64) -> bool {
    let l = m::hyp2(x - s[0], z - s[2]);
    l > 1e-3 && v > 0.3 && (vx * (x - s[0]) + vz * (z - s[2])) / (l * v) >= pace_cos
}

/// THIS teammate's walk this frame, m/s: paced on the player   (teampace.ts teamWalkOf)
pub fn team_walk_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let slow = team_walk(g);
    if !g.k.teampace.pacing { return slow; }
    let own = g.tune.player_walk;
    let post = match crate::orders::squad_orders_of(g, gd) { Some(a) => g.tab.orders_assignment[a].post, None => None };
    let mark = match post { Some(mark) => mark, None => return own };
    let (near, gap, cos, k, top) = (g.k.teampace.pace_near, g.k.teampace.pace_gap, g.k.teampace.pace_cos, g.k.teampace.k, g.k.pace.max);
    let p = g.ch[g.gd[gd].ch].pos;
    let sam = g.pl[g.player].ch;
    let s = g.ch[sam].pos;
    if m::hyp2(p[0] - s[0], p[2] - s[2]) > near { return own; }
    let dm = m::hyp2(p[0] - mark[0], p[2] - mark[2]);
    let ds = m::hyp2(s[0] - mark[0], s[2] - mark[2]);
    let (vx, vz) = (g.ch[sam].vel[0], g.ch[sam].vel[2]);
    let v = m::hyp2(vx, vz);
    if ds > dm + gap {
        return if toward(s, vx, vz, v, cos, p[0], p[2]) { m::max(slow * 0.5, m::min(slow, k * v)) } else { own };
    }
    if ds < dm - gap {
        return if toward(s, vx, vz, v, cos, mark[0], mark[2]) { m::max(own, m::min(v, top)) } else { own };
    }
    own
}

/// he is one of the quiet: no run of his own, none behind the player   (teampace.ts quiet)
pub fn quiet(g: &mut crate::game::Game, gd: GuardId) -> bool {
    crate::sneak::sneaking(g, gd) || crate::unseen::unseen_man(g, gd) || crate::silent::silent_man(g, gd)
}

/// is this reason a run   (teampace.ts isRun)
pub fn is_run(why: crate::teampace::IsRunWhy) -> bool {
    why != IsRunWhy::Close && why != IsRunWhy::Fight
}

/// the same word in the type a `Leave` holds
fn leave_why(why: TeamLeaveWhy) -> LeaveWhy {
    match why {
        TeamLeaveWhy::Blast => LeaveWhy::Blast,
        TeamLeaveWhy::Cover => LeaveWhy::Cover,
        TeamLeaveWhy::Follow => LeaveWhy::Follow,
        TeamLeaveWhy::Told => LeaveWhy::Told,
        TeamLeaveWhy::Close => LeaveWhy::Close,
        TeamLeaveWhy::Fight => LeaveWhy::Fight,
    }
}
/// the same word in the type `is_run` takes
fn is_run_why(why: TeamLeaveWhy) -> IsRunWhy {
    match why {
        TeamLeaveWhy::Blast => IsRunWhy::Blast,
        TeamLeaveWhy::Cover => IsRunWhy::Cover,
        TeamLeaveWhy::Follow => IsRunWhy::Follow,
        TeamLeaveWhy::Told => IsRunWhy::Told,
        TeamLeaveWhy::Close => IsRunWhy::Close,
        TeamLeaveWhy::Fight => IsRunWhy::Fight,
    }
}
/// a leave's word as a run's reason, or None for one that is no run   (teampace.ts runWhy's `isRun(L.why)`)
fn run_word(why: LeaveWhy) -> Option<RunWhy> {
    match why {
        LeaveWhy::Blast => Some(RunWhy::Blast),
        LeaveWhy::Cover => Some(RunWhy::Cover),
        LeaveWhy::Follow => Some(RunWhy::Follow),
        LeaveWhy::Told => Some(RunWhy::Told),
        LeaveWhy::Close | LeaveWhy::Fight => None,
    }
}

/// leave to go faster than the walk for `secs` more than this frame. False = none   (teampace.ts teamLeave)
pub fn team_leave(g: &mut crate::game::Game, gd: GuardId, why: crate::teampace::TeamLeaveWhy, top: f64, secs: Option<f64>) -> bool {
    let secs = secs.unwrap_or(0.0);
    if !team_slow(g, gd) { return false; }
    if is_run(is_run_why(why)) {
        let barred = if why == TeamLeaveWhy::Follow { !g.k.teampace.follow } else { !g.k.teampace.own };
        if barred { return false; }
        if quiet(g, gd) || crate::condition::is_hurt(g, crate::condition::Man::Guard(gd), None) { return false; }
    }
    let until = g.time + secs + FRAME;
    if let Some(was) = g.m.teampace.leave.get(gd) {
        if g.time <= was.until && was.until > until && was.top >= top { return true; }
    }
    g.m.teampace.leave.set(gd, Leave { why: leave_why(why), top, until });
    true
}

/// his leave, if it still runs   (teampace.ts live)
pub fn live(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::teampace::Leave> {
    match g.m.teampace.leave.get(gd).copied() {
        Some(l) if g.time <= l.until => Some(l),
        _ => None,
    }
}

/// the fight has his legs: a man in his sights, or one lost `fightSecs` ago   (teampace.ts fighting)
pub fn fighting(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match crate::roomfight::room_fight_op_of(g, gd) {
        Some(o) => g.tab.roomfight_op[o].tgt.is_some() || g.time - g.tab.roomfight_op[o].lost_at < g.k.teampace.fight_secs,
        None => false,
    }
}

/// he may run from the grenade he is leaving   (teampace.ts blastRun)
pub fn blast_run(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.k.teampace.own && !crate::condition::is_hurt(g, crate::condition::Man::Guard(gd), None)
}

/// the most a teammate's legs may be asked for this frame   (teampace.ts teamCap)
pub fn team_cap(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if !g.k.teampace.on { return g.k.pace.max; }
    if crate::frag::frag_running(g, gd) {
        return if blast_run(g, gd) { m::max(g.k.pace.max, g.tune.player_sprint) } else { g.k.pace.max };
    }
    let l = live(g, gd);
    let walk = team_walk_of(g, gd);
    let cap = m::max(walk, match l { Some(l) => l.top, None => 0.0 });
    if cap < g.k.pace.max && fighting(g, gd) { g.k.pace.max } else { cap }
}

/// he is at the team's walk: no leave for the fight's pace or a run, nobody in his sights, no grenade he runs from   (teampace.ts teamAtWalk)
pub fn team_at_walk(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !team_slow(g, gd) || crate::frag::frag_running(g, gd) || fighting(g, gd) { return false; }
    match live(g, gd) {
        None => true,
        Some(l) => l.why == LeaveWhy::Close,
    }
}

/// the reason he may be running for this frame, if any   (teampace.ts runWhy)
pub fn run_why(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::teampace::RunWhy> {
    if crate::frag::frag_running(g, gd) {
        return if blast_run(g, gd) { Some(RunWhy::Blast) } else { None };
    }
    live(g, gd).and_then(|l| run_word(l.why))
}

/// ONCE A FRAME A MAN, after his legs have moved: a run under a reason is written down, once a run, and its seconds   (teampace.ts teamRan)
pub fn team_ran(g: &mut crate::game::Game, gd: GuardId) {
    if !team_slow(g, gd) { return; }
    if g.k.teampace.back_secs > 0.0 && crate::frag::frag_running(g, gd) {
        let (top, secs) = (g.k.pace.max, g.k.teampace.back_secs);
        team_leave(g, gd, TeamLeaveWhy::Fight, top, Some(secs));
    }
    let why = match run_why(g, gd) { Some(why) => why, None => return };
    let now = g.time;
    g.m.teampace.reason_t.set(gd, now);
    if g.gd[gd].speed < g.k.teampace.run_at { return; }
    let was = g.m.teampace.ran.get(gd).copied();
    let t = g.time;
    match was {
        Some(was) if !(t - was.t > g.k.teampace.gap) => {
            if t > was.t { seam_stats_secs(g, t - was.t); }
        }
        _ => match why {
            RunWhy::Follow => g.stats.teampace.follow += 1,
            RunWhy::Blast => g.stats.teampace.blast += 1,
            RunWhy::Cover => g.stats.teampace.cover += 1,
            RunWhy::Told => g.stats.teampace.told += 1,
        },
    }
    g.m.teampace.ran.set(gd, TeampaceModRanValue { t, why });
}

/// seconds since he last had a reason to run (infinity: never, or the rule is off)   (teampace.ts teamRunAgo)
pub fn team_run_ago(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.teampace.reason_t.get(gd).copied() {
        Some(t) if !(t > g.time) => g.time - t,
        _ => m::INF,
    }
}

/// × his card for a leg asked at `speed`: at a run a card weighs as it does on the player's sprint. None = as it was   (teampace.ts teamRunLegs)
pub fn team_run_legs(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> Option<f64> {
    if speed < g.k.teampace.run_at || !team_slow(g, gd) { return None; }
    let k = crate::kitload::load_pace_on(g, crate::kitload::LoadPaceOnWho::Guard(gd));
    Some(speed * (1.0 + (k - 1.0) * g.k.kitload.sprint))
}

/// how fast his legs pick pace up: a man with a reason to run gets to it as a runner does   (teampace.ts teamAccel)
pub fn team_accel(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if !g.k.teampace.on { return g.k.pace.accel; }
    if run_why(g, gd).is_some() { m::max(g.k.pace.accel, g.k.teampace.accel) } else { g.k.pace.accel }
}

/// he is at a run this frame: gun low, nothing fired, a sprinter's boots   (teampace.ts teamSprinting)
pub fn team_sprinting(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].speed >= g.k.teampace.run_at && team_slow(g, gd)
}

/// A TEAMMATE'S FOOTFALL, the level one step is heard at   (teampace.ts teamTread)
pub fn team_tread(g: &mut crate::game::Game, gd: GuardId, speed: f64, base: f64) -> f64 {
    let level = if speed >= g.k.teampace.run_at && team_slow(g, gd) {
        g.k.teampace.step
    } else if crate::kitload::load_gait(g, crate::kitload::LoadGaitWho::Guard(gd), speed) > 2.0 {
        0.8
    } else {
        crate::silent::silent_step(g, gd).unwrap_or(base)
    };
    level * crate::kitload::load_heard(g, crate::kitload::LoadHeardWho::Guard(gd))
}

/// fall in's pace for a man `d` off his place, the player doing `sam_v`. None = as it was   (teampace.ts teamFollow)
pub fn team_follow(g: &mut crate::game::Game, gd: GuardId, sam_v: f64, d: f64, gain: f64) -> Option<f64> {
    if !team_slow(g, gd) { return None; }
    let walk = team_walk(g);
    let want = sam_v + gain * (d - 0.3);
    let sprint = g.tune.player_sprint;
    if g.k.teampace.follow && g.pl[g.player].sprinting && !crate::roomfight::room_fight_busy(g, gd, 0.5) {
        let running = match live(g, gd) { Some(l) => l.why == LeaveWhy::Follow, None => false };
        if (running || d > g.k.teampace.behind_m) && team_leave(g, gd, TeamLeaveWhy::Follow, sprint, None) {
            return Some(m::max(walk, m::min(sprint, want)));
        }
    }
    let top = m::max(walk, g.tune.player_walk * g.k.teampace.follow_k);
    if top > walk { team_leave(g, gd, TeamLeaveWhy::Close, top, None); }
    Some(m::max(walk, m::min(top, want)))
}

/// a move the fight asks of him keeps the pace it had, for `secs` more than this frame   (teampace.ts teamFight)
pub fn team_fight(g: &mut crate::game::Game, gd: GuardId, secs: Option<f64>) {
    if g.k.teampace.on {
        let top = g.k.pace.max;
        team_leave(g, gd, TeamLeaveWhy::Fight, top, secs);
    }
}

/// A RUN TO COVER, the question itself. `running` = asked of a man already on the leg   (teampace.ts teamDashNow)
pub fn team_dash_now(g: &mut crate::game::Game, gd: GuardId, to: V3, d: f64, least: Option<f64>, running: Option<bool>) -> bool {
    let least = least.unwrap_or(g.k.teampace.dash_m);
    let running = running.unwrap_or(false);
    if !g.k.teampace.own || !team_slow(g, gd) { return false; }
    let p = g.ch[g.gd[gd].ch].pos;
    d >= least
        && (running || g.time - crate::roomfight::room_fight_hot_of(g, gd) < g.k.teampace.dash_hot)
        && !crate::roomfight::room_fight_engaged(g, gd)
        && !quiet(g, gd)
        && !crate::condition::is_hurt(g, crate::condition::Man::Guard(gd), None)
        && crate::roomfight::room_fight_room_at(g, p) == crate::roomfight::room_fight_room_at(g, to)
        && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [to[0], 0.45, to[2]], None)
        && (running || crate::picture::exposed_at(g, gd, p, None) >= g.k.teampace.dash_seen)
        && crate::picture::exposed_at(g, gd, to, None) <= 0.0
}

/// … and the pace to run the leg at, or None. `stack` = his mark is a place in a stack at a door   (teampace.ts teamDash)
pub fn team_dash(g: &mut crate::game::Game, gd: GuardId, to: V3, d: f64, stack: Option<bool>) -> Option<f64> {
    let stack = stack.unwrap_or(false);
    if !g.k.teampace.own || !team_slow(g, gd) { return None; }
    let brisk = stack && g.k.teampace.door_brisk;
    let t = g.time;
    let run = if brisk { g.k.pace.max } else { g.tune.player_sprint };
    let every = g.k.teampace.dash_every;
    let was = match g.m.teampace.dash.get(gd).copied() {
        Some(old) if t >= old.t && t - old.t < every * 2.0 => Some(old),
        _ => None,
    };
    let same = match was { Some(was) => m::hyp2(was.x - to[0], was.z - to[2]) < 0.5, None => false };
    let yes = match was {
        Some(was) if same && t - was.t < every => was.yes && d > 0.6,
        _ => {
            let on = same && match was { Some(was) => was.yes, None => false };
            let least = if on { 0.6 } else { g.k.teampace.dash_m };
            let yes = team_dash_now(g, gd, to, d, Some(least), Some(on));
            g.m.teampace.dash.set(gd, TeampaceModDashValue { t, x: to[0], z: to[2], yes });
            yes
        }
    };
    let why = if brisk { TeamLeaveWhy::Fight } else { TeamLeaveWhy::Cover };
    if yes && team_leave(g, gd, why, run, None) { Some(run) } else { None }
}

/// A RUN THE MISSION TELLS HIM TO MAKE: the pace to run the leg at, or None   (teampace.ts teamTold)
pub fn team_told(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> {
    if !crate::rails::rails_runs(g, gd) || !team_slow(g, gd) || crate::roomfight::room_fight_engaged(g, gd) { return None; }
    let run = g.tune.player_sprint;
    if team_leave(g, gd, TeamLeaveWhy::Told, run, None) { Some(run) } else { None }
}

/// the pace a teammate leaves a grenade at   (teampace.ts teamBlastPace)
pub fn team_blast_pace(g: &mut crate::game::Game, gd: GuardId, run: f64) -> f64 {
    if g.gd[gd].team != crate::teams::Team::Operative { return run; }
    if g.k.teampace.on && blast_run(g, gd) { run } else { m::min(run, g.k.pace.max) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the words a leave can hold: two walk's and the runs
    #[test]
    fn run_words() {
        assert!(!is_run(IsRunWhy::Close) && !is_run(IsRunWhy::Fight));
        assert!(is_run(IsRunWhy::Blast) && is_run(IsRunWhy::Cover) && is_run(IsRunWhy::Follow) && is_run(IsRunWhy::Told));
        assert!(run_word(LeaveWhy::Close).is_none() && run_word(LeaveWhy::Fight).is_none());
        assert!(run_word(LeaveWhy::Told) == Some(RunWhy::Told));
        for w in [TeamLeaveWhy::Blast, TeamLeaveWhy::Cover, TeamLeaveWhy::Follow, TeamLeaveWhy::Told, TeamLeaveWhy::Close, TeamLeaveWhy::Fight] {
            assert_eq!(leave_why(w).as_str(), w.as_str());
            assert_eq!(is_run_why(w).as_str(), w.as_str());
        }
    }

    /// the player is moving, and at that spot: within the cosine, not when still, not at his own feet
    #[test]
    fn toward_the_spot() {
        let s = [0.0, 0.0, 0.0];
        assert!(toward(s, 1.0, 0.0, 1.0, 0.5, 5.0, 0.0));
        assert!(!toward(s, 1.0, 0.0, 1.0, 0.5, -5.0, 0.0));
        assert!(!toward(s, 0.1, 0.0, 0.1, 0.5, 5.0, 0.0));
        assert!(!toward(s, 1.0, 0.0, 1.0, 0.5, 0.0, 0.0));
    }

    /// the two clocks: a leave ends after its `until`, a man who never had a reason is infinitely long ago
    #[test]
    fn the_clocks() {
        let mut g = Game::blank();
        let gd = GuardId::from_ix(0);
        assert!(live(&mut g, gd).is_none());
        assert_eq!(team_run_ago(&mut g, gd), m::INF);
        g.time = 5.0;
        g.m.teampace.leave.set(gd, Leave { why: LeaveWhy::Close, top: 2.0, until: 5.0 });
        assert!(live(&mut g, gd).is_some());
        g.time = 5.5;
        assert!(live(&mut g, gd).is_none());
        g.m.teampace.reason_t.set(gd, 5.0);
        assert_eq!(team_run_ago(&mut g, gd), 0.5);
        g.time = 4.0;
        assert_eq!(team_run_ago(&mut g, gd), m::INF);
    }
}
