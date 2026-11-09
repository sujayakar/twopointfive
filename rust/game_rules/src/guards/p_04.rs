//! src/game/guards.ts lines 1131 to 1502
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- private helpers ----

/// `gd.path = path`: a whole replacement, so the identity serial moves
fn lay_path(g: &mut Game, gd: GuardId, path: Vec<V3>) {
    g.gd[gd].path = path;
    g.gd[gd].path_serial = g.gd[gd].path_serial.wrapping_add(1);
}

/// `gd.pathGoal = <anything>`: THE ONE SETTER of a man's path goal. The standoff asks whether his goal IS its own back-off point, the very object
/// (guards.ts:1163 `gd.pathGoal !== S.orbitGoal`); a Vec3 here is a value, so `path_goal_orbit` says it, and any other write of the goal is by
/// that fact no longer that object. Every whole write of `path_goal` outside the standoff goes through here, so the flag cannot be left standing.
pub fn guard_set_path_goal(g: &mut crate::game::Game, gd: GuardId, goal: Option<V3>) {
    let o = &mut g.gd[gd];
    if o.path_goal_orbit { crate::game::b_tap::tap(crate::game::b_tap::PATH_GOAL_SET_WITH_ORBIT_UP); }
    o.path_goal = goal;
    o.path_goal_orbit = false;
}

/// standoffAlert's setGoal
fn set_goal(g: &mut Game, gd: GuardId, s: GuardsStandoffId, p: Option<V3>) {
    g.gd[gd].path_goal_orbit = false;   // whatever comes next, S.orbitGoal is no longer the object pathGoal may hold
    let p = match p {
        None => {
            if g.tab.guards_standoff[s].orbit_goal.is_some() {
                lay_path(g, gd, Vec::new());
                g.gd[gd].path_goal = None;
            }
            g.tab.guards_standoff[s].orbit_goal = None;
            return;
        }
        Some(p) => p,
    };
    let far_from_old = match g.tab.guards_standoff[s].orbit_goal {
        None => true,
        Some(o) => v3::dist_xz(o, p) > 0.3,
    };
    if far_from_old { g.gd[gd].stall_ref = None; }
    g.tab.guards_standoff[s].orbit_goal = Some(p);
}

/// goTo's plan. `round` None is the TypeScript's `round === pen`: no second search.
fn go_to_plan(g: &mut Game, gd: GuardId, goal: V3, pen: Option<&crate::lanes::LanePenalty>, round: Option<&crate::orders::PropPenalty>) -> Option<Vec<V3>> {
    let from = g.ch[g.gd[gd].ch].pos;
    let mut p = match pen {
        Some(q) => crate::scene_collision::nav_grid_find_path(g, from, goal, Some(6000.0), Some(&mut |_g: &mut Game, ix: f64, iz: f64| crate::lanes::lane_penalty_at(q, ix, iz))),
        None => crate::scene_collision::nav_grid_find_path(g, from, goal, Some(6000.0), None),
    };
    if let (Some(plain), Some(q)) = (p.as_ref(), round) {
        let r = crate::scene_collision::nav_grid_find_path(g, from, goal, Some(6000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| crate::orders::prop_penalty_at(g, q, ix, iz)));
        p = Some(crate::orders::prop_way(g, from, plain, r.as_deref()));
    }
    if p.is_some() { return p; }
    if crate::scene_collision::nav_grid_walkable(g, from, goal) && !crate::scene_collision::solid_blocked(g, [from[0], 0.5, from[2]], [goal[0], 0.5, goal[2]], None) {
        Some(vec![v3::copy(goal)])
    } else {
        None
    }
}

// ---- the translation ----

/// The alert branch for a man in the standoff: feet, gun, mouth; his move speed and the point he looks / aims at.   (guards.ts standoffAlert)
pub fn standoff_alert(g: &mut crate::game::Game, gd: GuardId, dt: f64, p: crate::guards::Pair) -> crate::guards::StandoffAlert {
    let s = g.gd[gd].standoff.unwrap();
    let c = g.gd[gd].ch;
    let aim = p.aim;
    let dist = v3::dist_xz(g.ch[c].pos, p.centre);
    let fs = flank_shot(g, gd, p);
    let except = if fs.ok || crate::gunhold::gun_hold_phase(g, gd) == Some(crate::gunhold::Phase::Line) { Some(p.held) } else { None };
    muzzle_check(g, gd, Some(aim), dt, except);
    if fs.ok && !g.tab.guards_standoff[s].line_open {
        let force = g.time - g.tab.guards_standoff[s].last_bark_t > 2.5;
        standoff_bark(g, gd, crate::guards::StandoffMoment::Line, Some(crate::guards::StandoffBarkForce::Bool(force)), None);
    }
    g.tab.guards_standoff[s].line_open = fs.ok;
    // --- feet ---
    let mut move_ = 0.0;
    let far = dist > STANDOFF_MAX_M;
    let before0 = g.ch[c].pos[0];
    let before2 = g.ch[c].pos[2];
    g.tab.guards_standoff[s].replan_t -= dt;
    g.tab.guards_standoff[s].rest_t = m::max(0.0, g.tab.guards_standoff[s].rest_t - dt);
    let gh = crate::gunhold::gun_hold_feet(g, gd, dt, p, s, crate::gunhold::GunHoldFeetFs { ok: fs.ok, rear: fs.rear, clear: fs.clear });
    let backing = gh.is_none() && dist < STANDOFF_MIN_M - 0.3;
    let pace_base = if far || backing { g.tune.guard_investigate } else { g.tune.guard_walk };
    let pace_k = if crate::suppression::flinching(g, gd) { g.k.suppression.standoff_pace_k } else { 1.0 };
    let pace = pace_base * pace_k;
    if gh.is_some() {
        // his feet are gunhold.ts's
    } else if backing {
        if g.tab.guards_standoff[s].replan_t <= 0.0 || g.tab.guards_standoff[s].orbit_goal.is_none() || !g.tab.guards_standoff[s].backing {
            g.tab.guards_standoff[s].replan_t = 0.5;
            let goal = retreat_goal(g, gd, p);
            set_goal(g, gd, s, goal);
        }
    } else if fs.ok && (g.tab.guards_standoff[s].orbit_goal.is_none() || fs.rear < (STANDOFF_REAR_DEG - 15.0) * v3::DEG) {
        set_goal(g, gd, s, None);
    } else if g.tab.guards_standoff[s].rest_t > 0.0 {
        set_goal(g, gd, s, None);
    } else if g.tab.guards_standoff[s].replan_t <= 0.0 || g.tab.guards_standoff[s].orbit_goal.is_none() {
        g.tab.guards_standoff[s].replan_t = STANDOFF_REPLAN_SECS;
        let goal = if unstick_begin(g, gd) == crate::guards::UnstickBegin::Leg {
            Some(v3::copy(g.m.guards.unstick_legs.get(gd).unwrap().to))
        } else {
            orbit_goal_for(g, gd, s, p)
        };
        set_goal(g, gd, s, goal);
    }
    g.tab.guards_standoff[s].backing = backing;
    let mut jogging = false;
    let mut look_at = aim;
    if let Some(h) = gh {
        move_ = h.move_;
        jogging = h.run;
        if let Some(l) = h.look { look_at = l; }
    } else if let Some(goal) = g.tab.guards_standoff[s].orbit_goal {
        if backing {
            // (pathGoal !== S.orbitGoal, of two objects: path_goal_orbit says they are the one object)
            // (…and the flag is believed only while the goal still holds that point's numbers: a write that slipped past the setter cannot leave it up)
            {   // (the loop's counters, b_tap.rs)
                use crate::game::b_tap as tp;
                tp::tap(tp::STANDOFF_ORBIT_ASKED);
                if !(g.gd[gd].path_goal_orbit && g.gd[gd].path_goal == Some(goal)) { tp::tap(tp::STANDOFF_ORBIT_RELAID); if g.gd[gd].path_goal_orbit { tp::tap(tp::STANDOFF_ORBIT_FLAG_UP_GOAL_MOVED); } }
            }
            if !(g.gd[gd].path_goal_orbit && g.gd[gd].path_goal == Some(goal)) {
                lay_path(g, gd, vec![goal]);
                g.gd[gd].path_i = 0;
                g.gd[gd].path_goal = Some(goal);
                g.gd[gd].path_goal_orbit = true;
            }
        } else {
            go_to(g, gd, goal, None);
        }
        let done = if far { follow_path(g, gd, dt, pace) } else { step_along_path(g, gd, dt, pace) };
        move_ = g.gd[gd].speed;
        if done {
            set_goal(g, gd, s, None);
            g.tab.guards_standoff[s].replan_t = m::min(g.tab.guards_standoff[s].replan_t, 0.25);
        } else if not_getting_anywhere(g, gd, dt, STANDOFF_STALL_SECS) {
            set_goal(g, gd, s, None);
            g.gd[gd].stall_ref = None;
            g.tab.guards_standoff[s].rest_t = STANDOFF_REST_SECS;
            g.tab.guards_standoff[s].stalls += 1.0;
            if js::rem(g.tab.guards_standoff[s].stalls, 2.0) == 0.0 {
                let so = &mut g.tab.guards_standoff[s];
                so.side = if so.side > 0.0 { -1.0 } else { 1.0 };
                so.way = if so.way > 0.0 { -1.0 } else if so.way < 0.0 { 1.0 } else { 0.0 };
            }
        } else if far && g.gd[gd].path_i < g.gd[gd].path.len() {
            // the jog over: the legs have the body, and a leg that turns its back on them he runs looking where he is going
            jogging = true;
            let wp = g.gd[gd].path[g.gd[gd].path_i];
            let pos = g.ch[c].pos;
            let yaw_path = m::atan2(wp[0] - pos[0], wp[2] - pos[2]);
            if m::abs(v3::wrap_angle(m::atan2(aim[0] - pos[0], aim[2] - pos[2]) - yaw_path)) > 75.0 * v3::DEG {
                look_at = [pos[0] + m::sin(yaw_path) * 4.0, aim[1], pos[2] + m::cos(yaw_path) * 4.0];
            }
        }
    } else {
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 6.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].stall_ref = None;
    }
    let pos = g.ch[c].pos;
    g.tab.guards_standoff[s].orbit_m += m::hyp2(pos[0] - before0, pos[2] - before2);
    if !jogging {
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, m::atan2(look_at[0] - pos[0], look_at[2] - pos[2]), 4.0 * dt);
    }
    // --- the round ---
    g.gd[gd].fire_cd -= dt;
    let gun_yaw = crate::character::character_gun_yaw(g, c);
    let facing_err = m::abs(v3::wrap_angle(m::atan2(aim[0] - pos[0], aim[2] - pos[2]) - gun_yaw));
    let w = g.gd[gd].weapon;
    g.fa[w].burst = None;
    let held = g.tune.muzzle_discipline
        && !(gh.is_some() && crate::gunhold::gun_hold_phase(g, gd) == Some(crate::gunhold::Phase::Line) && crate::gunhold::gun_hold_free(g, p));
    if g.gd[gd].reload_t >= 0.0 {
        tick_reload_beat(g, gd, dt);
    } else if !held && fs.ok && !backing && g.gd[gd].fire_cd <= 0.0 && facing_err < 0.12 && !g.gd[gd].muzzle_down
        && !crate::combat::arm_through(g, c)
        && !round_past_colleague(g, gd, aim, Some(p.held), None)
        && crate::aim::aim_ready(g, gd)
        && crate::aim::feet_settled(g, gd)
        && crate::weapons::firearm_fire(g, w)
    {
        let mods = crate::suppression::guard_fire_mods(g, gd);
        g.gd[gd].fire_cd = (STANDOFF_SHOT_SECS + crate::dice::draw_of(g, "guards::standoff_alert#1", gd) * 0.6) * mods.gap;
        let o = crate::aim::guard_round(g, gd, mods.spread, None);
        crate::combat::fire_weapon(g, c, aim, false, Some(&o));
        g.tab.guards_standoff[s].shots += 1.0;
        let w = g.gd[gd].weapon;
        if crate::weapons::firearm_rounds_ready(g, w) == 0.0 { start_reload_beat(g, gd); }
    }
    if held { g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, SHIELD_RELEASE_BEAT_MIN + dt); }
    // --- the talking ---
    if gh.map_or(false, |h| h.quiet) {
        // on his way round, or running: nothing to say to Sam
    } else if !fs.ok {
        let moment = if g.tab.guards_standoff[s].cornered && g.tab.guards_standoff[s].orbit_goal.is_none() { crate::guards::StandoffMoment::Cornered } else { crate::guards::StandoffMoment::NoShot };
        standoff_bark(g, gd, moment, None, None);
    } else if held {
        standoff_bark(g, gd, crate::guards::StandoffMoment::Line, None, None);
    }
    crate::guards::StandoffAlert { move_, aim: look_at, crouch: gh.map_or(0.0, |h| h.crouch) }
}

/// Every frame for a man carrying a standoff record: is it still one? The pair when it stands.   (guards.ts standoffUpkeep)
pub fn standoff_upkeep(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::Pair> {
    let s = g.gd[gd].standoff?;
    let p = pair_of(g, &crate::seats::here(g));
    let pair = g.tab.guards_standoff[s].pair;
    let why = if g.quiet_utility || g.gd[gd].hold || g.gd[gd].pinned || !g.ai_enabled {
        Some("off")
    } else if g.gd[gd].state != crate::game::GuardState::Alert {
        Some("state")
    } else if p.map_or(true, |q| q.held != pair) {
        Some(if g.gd[pair].state == crate::game::GuardState::Dead { "dead" } else if g.pl[g.player].down { "samDown" } else { "free" })
    } else if g.time - g.tab.guards_standoff[s].seen_t > STANDOFF_LOSE_SECS && !crate::gunhold::standoff_fix_kept(g, gd) {
        Some("lost")
    } else {
        None
    };
    let why = match why { None => return p, Some(w) => w };
    g.gd[gd].standoff = None;
    lay_path(g, gd, Vec::new());
    g.gd[gd].path_goal = None;
    g.gd[gd].path_goal_orbit = false;
    g.gd[gd].stall_ref = None;
    reset_chase(g, gd);
    let seen_t = g.tab.guards_standoff[s].seen_t;
    let watched = g.time - seen_t < 0.6;
    if why == "dead" && watched {
        let b = pair;
        if g.gd[b].state == crate::game::GuardState::Dead {
            if g.guards.contains(&b) {
                if !g.gd[b].found {
                    g.gd[b].found = true;
                    crate::mission::tally(g, crate::mission::TallyK::Bodies);
                    crate::chronicle::chron_body_found(g, b, gd);
                }
                let victim = g.gd[b].callsign.clone();
                let t = g.time;
                let breathing = crate::game::is_breathing(g, b);
                g.gd[gd].witness.saw_body = Some(crate::game::WitnessSawBody { victim, t, breathing });
            }
            if g.gd[b].down_by != Some(crate::game::DownBy::Guard) {
                standoff_bark(g, gd, crate::guards::StandoffMoment::Dropped, Some(crate::guards::StandoffBarkForce::Bool(true)), Some(b));
            }
        }
        g.gd[gd].fire_cd = if g.tune.muzzle_discipline {
            SHIELD_RELEASE_BEAT_MIN + crate::dice::draw_of(g, "guards::standoff_upkeep#1", gd) * SHIELD_RELEASE_BEAT_JITTER
        } else {
            let cd = g.gd[gd].fire_cd;
            m::min(cd, 0.15 + crate::dice::draw_of(g, "guards::standoff_upkeep#2", gd) * 0.3)
        };
    } else if why == "free" && watched {
        standoff_bark(g, gd, crate::guards::StandoffMoment::Freed, Some(crate::guards::StandoffBarkForce::Bool(true)), Some(pair));
        g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, 0.4);
    } else if why == "lost" {
        standoff_bark(g, gd, crate::guards::StandoffMoment::Lost, Some(crate::guards::StandoffBarkForce::Bool(true)), Some(pair));
        g.gd[gd].last_seen_t = m::min(g.gd[gd].last_seen_t, seen_t);
    }
    None
}

/// The friendly-fire line: the shooter says so, whatever the clocks.   (guards.ts hostageHitBark)
pub fn hostage_hit_bark(g: &mut crate::game::Game, shooter: GuardId, victim: GuardId) {
    standoff_bark(g, shooter, crate::guards::StandoffMoment::HitHim, Some(crate::guards::StandoffBarkForce::Always), Some(victim));
}

/// Panel / QA: one line per man in a standoff. Text only; `{:.n}` stands for toFixed (they differ on an exact tie).   (guards.ts standoffSummary)
pub fn standoff_summary(g: &mut crate::game::Game) -> String {
    let p = pair_of(g, &crate::seats::here(g));
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let s = match g.gd[gd].standoff { Some(s) => s, None => continue };
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let fs = match p {
            Some(q) if q.held == g.tab.guards_standoff[s].pair => Some(flank_shot(g, gd, q)),
            _ => None,
        };
        let line = match fs {
            Some(f) => format!("{} m, {}° off their rear, clear {}{}", js::to_fixed(f.dist, 1), m::round(f.rear / v3::DEG), js::to_fixed(f.clear, 2), if f.ok { " LINE" } else { "" }),
            None => String::from("pair gone"),
        };
        let so = g.tab.guards_standoff[s];
        let feet = match so.orbit_goal {
            Some(o) => format!("to ({}, {})", js::to_fixed(o[0], 1), js::to_fixed(o[2], 1)),
            None => String::from(if so.cornered { "cornered" } else if so.rest_t > 0.0 { "resting" } else { "planted" }),
        };
        parts.push(format!(
            "{}: {} · side {} · {} · {} shot{}, {} m walked, seen {} s ago",
            g.gd[gd].callsign, line, if so.side > 0.0 { "+" } else { "−" }, feet, so.shots, if so.shots == 1.0 { "" } else { "s" }, js::to_fixed(so.orbit_m, 1), js::to_fixed(g.time - so.seen_t, 1)));
    }
    parts.join(" | ")
}

/// The player he has in mind, in the view of the seat that has this man.   (guards.ts canSee)
pub fn can_see(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::CanSee {
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    let seat = crate::seats::view_of_body(g, crate::seats::ViewOfBodyC { id });
    // (a game of one seat, or a view handed in: the seat is already the one on Game, and there is nothing to swap)
    let r = if seat == g.seat {
        let pc = g.pl[g.player].ch;
        can_see_char(g, gd, pc, true)
    } else {
        crate::seats::with_seat(g, seat, &mut |g: &mut Game| {
            let pc = g.pl[g.player].ch;
            can_see_char(g, gd, pc, true)
        })
    };
    crate::guards::CanSee { visible: r.visible, dist: r.dist, in_cone: r.in_cone }
}

/// he sees a man who is up: any player, each asked of on his own seat
pub fn sees_a_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.seats.len() == 1 { let p = g.player; return !g.pl[p].down && can_see(g, gd).visible; }
    for s in g.seats.clone() {
        if crate::seats::with_seat(g, s, &mut |g: &mut Game| { let p = g.player; !g.pl[p].down && { let pc = g.pl[p].ch; can_see_char(g, gd, pc, true).visible } }) { return true; }
    }
    false
}

/// WHERE A MAN LOOKS, for every eye in the game: his head, or his sights when a long gun is in his shoulder and level; level, always. It is the
/// fog's own answer (`fogofwar::gaze_of`), so what the player is shown of where a man looks and what that man can see are one thing.
///
/// His pose turns his eyes; it does not tip them. A chest bowed over a knee or a low gun points at the floor, and with the tip kept his cone left the
/// room: a kneeling man saw nobody past the near cone. Nor does his chest turn them: a man with a rifle up stands bladed, his chest 60 to 90 degrees
/// to the right of his gun, and with the chest in the mean his cone was centred 40 degrees right of his sights, so a defender with his gun on a door
/// never saw a lit man 20 degrees to the left of it. Height is the line test's business (a wall or a floor stops the line). A pose that is MEANT to
/// take a man's sight says so by name where the eye is asked (asleep, gripped, dazzled), not by its bones.
/// Every cone in the game that asks "does this man see that" is asked along this: a man or Sam (`can_see_char`), a point, a body, a kicked lock, the
/// standoff's pair, the floor, a civilian's eye, the sneak's reading of a defender.
pub fn look_of(g: &crate::game::Game, c: CharId) -> V3 { crate::fogofwar::gaze_of(g, c) }

/// canSee against any man `pc`.   (guards.ts canSeeChar)
pub fn can_see_char(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool) -> crate::guards::CanSeeChar {
    let c = g.gd[gd].ch;
    let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(h) => *h,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    };
    let look = look_of(g, c);
    let sam = crate::seats::here(g).player;
    let range = reach_for(g, sam, gd, pc, is_player, eye);
    let e = crate::guards::Eye { pos: eye, look, range, half_deg: g.k.guard_eye.half_deg, near_m: g.k.guard_eye.near_m, near_deg: g.k.guard_eye.near_deg, v_half_deg: None };
    let through = eye_through(g, gd);
    let blind = crate::control::man::blind(g, gd) || crate::pairorders::pair_gripped(g, gd) || crate::doze::doze_asleep(g, gd);
    let smoke_trans = g.gd[gd].smoke_trans;
    let mut r = eye_contact(g, e, pc, is_player, blind, smoke_trans, Some(through));
    if !is_player && r.visible && crate::roomfight::pair_smoke(g, c, pc) < 0.3 { r.visible = false; }
    crate::guards::CanSeeChar { visible: r.visible, dist: r.dist, in_cone: r.in_cone }
}

/// How far a man's eye reaches for a man with `lit` on him.   (guards.ts eyeReach)
pub fn eye_reach(g: &crate::game::Game, lit: f64) -> f64 {
    let k = g.k.guard_eye;
    if g.touring { return k.range; }
    reach_of(&k, lit)
}

/// eyeReach's sum, for a game not under a show
fn reach_of(k: &crate::knobs::GuardEyeK, lit: f64) -> f64 {
    k.range + m::max(0.0, k.far_range - k.range) * v3::smoothstep(k.far_lo, k.far_hi, lit)
}

/// …for the man `pc` in this man's view.   (guards.ts reachFor)
pub fn reach_for(g: &mut crate::game::Game, sam: PlayerId, gd: GuardId, pc: CharId, is_player: bool, eye: V3) -> f64 {
    if g.touring { return g.k.guard_eye.range; }
    if is_player {
        let vis = g.pl[sam].visibility;
        return eye_reach(g, vis);
    }
    let t = match g.ch[pc].bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => g.ch[pc].pos };
    if m::hyp2(t[0] - eye[0], t[2] - eye[2]) < g.k.guard_eye.range - 1.0 { return g.k.guard_eye.range; }
    let mut o = None;
    for &x in g.guards.iter() {
        if g.gd[x].ch == pc { o = Some(x); break; }
    }
    match o {
        Some(o) => {
            let vis = g.gd[o].visibility;
            let c = g.gd[gd].ch;
            let glare = crate::roomfight::lamp_glare(g, c, o);
            eye_reach(g, m::max(vis, glare))
        }
        None => g.k.guard_eye.range,
    }
}

/// True = this glimpse from past `range` feeds his awareness and writes no fix.   (guards.ts farFaint)
pub fn far_faint(g: &crate::game::Game, gd: GuardId, dist: f64, was: f64) -> bool {
    let k = g.k.guard_eye;
    faint_of(&k, g.gd[gd].state, g.gd[gd].awareness, dist, was)
}

/// farFaint's rule, over what it reads of the man
fn faint_of(k: &crate::knobs::GuardEyeK, state: crate::game::GuardState, awareness: f64, dist: f64, was: f64) -> bool {
    if dist <= k.range { return false; }
    if state == crate::game::GuardState::Patrol { return !(was < 0.3 && awareness >= 0.3); }
    if state == crate::game::GuardState::Suspicious { return awareness < k.far_fix_at; }
    false
}

/// The distance term of a sighting's rate.   (guards.ts distTerm)
pub fn dist_term(g: &crate::game::Game, dist: f64) -> f64 {
    let k = g.k.guard_eye;
    dist_term_of(&k, dist)
}

/// distTerm's sum
fn dist_term_of(k: &crate::knobs::GuardEyeK, dist: f64) -> f64 {
    if dist > k.range { return v3::lerp(0.2, k.far_k, v3::clamp((dist - k.range) / m::max(1e-3, k.far_range - k.range), 0.0, 1.0)); }
    v3::clamp(1.0 - (dist - 3.0) / 13.0, 0.2, 1.0)
}

/// What the team's own eyes pass (FOG.mates on: what the fog's lines pass).   (guards.ts teamThrough)
pub fn team_through(g: &crate::game::Game) -> f64 {
    crate::fogofwar::fog_through(g)
}

/// What a man's eye passes: the team's for an operative, glass alone for every other side.   (guards.ts eyeThrough)
pub fn eye_through(g: &crate::game::Game, gd: GuardId) -> f64 {
    if g.gd[gd].team == crate::teams::Team::Operative { team_through(g) } else { crate::scene_boxes::box_flag().glass }
}

/// That man's own line to a point, asked bare.   (guards.ts eyeBlocked)
pub fn eye_blocked(g: &mut crate::game::Game, gd: GuardId, a: V3, b: V3) -> bool {
    let through = eye_through(g, gd);
    crate::scene_collision::segment_blocked(g, a, b, Some(0.02), Some(through), None)
}

/// THE line-of-sight test, whoever's eye it is: one question to the module; what hides Sam is read last, of a man the eye would otherwise see.   (guards.ts eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::guards::Eye, pc: CharId, is_player: bool, blind: bool, smoke_trans: f64, through: Option<f64>) -> crate::guards::EyeContact {
    let through = match through { Some(t) => t, None => crate::scene_boxes::box_flag().glass };
    let tgt = match g.ch[pc].bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => v3::add(g.ch[pc].pos, [0.0, 1.1, 0.0]) };
    let tgt2 = match g.ch[pc].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(g.ch[pc].pos, [0.0, 1.6, 0.0]) };
    let smoked = is_player && smoke_trans < 0.3;
    let unseen = blind || (!is_player && !g.ch[pc].alive);
    let lens = crate::scene_collision::EyeContactE { pos: e.pos, look: e.look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: e.v_half_deg };
    let r = crate::scene_collision::eye_contact(g, lens, tgt, tgt2, unseen, smoked, through);
    let mut visible = r.visible;
    if visible && is_player && (g.player_invisible || crate::seats::man_of(g, Some(pc)).map_or(g.pl[g.player].down, |p| g.pl[p].down)) { visible = false; }
    crate::guards::EyeContact { visible, dist: r.dist, in_cone: r.in_cone }
}

/// Is the nav cell under (x, z) one no plan can leave?   (guards.ts strandedAt)
pub fn stranded_at(g: &crate::game::Game, x: f64, z: f64) -> bool {
    let cs = g.col.nav.cell;
    crate::scene_collision::nav_grid_is_blocked(g, x, z)
        && crate::scene_collision::nav_grid_is_blocked(g, x + cs, z)
        && crate::scene_collision::nav_grid_is_blocked(g, x - cs, z)
        && crate::scene_collision::nav_grid_is_blocked(g, x, z + cs)
        && crate::scene_collision::nav_grid_is_blocked(g, x, z - cs)
}

/// Can his body be walked straight from `from` to `to`?   (guards.ts legClear)
pub fn leg_clear(g: &mut crate::game::Game, from: V3, to: V3) -> bool {
    let dx = to[0] - from[0];
    let dz = to[2] - from[2];
    let l = m::hyp2(dx, dz);
    if l < 1e-3 { return true; }
    if crate::scene_collision::segment_blocked_dynamic(g, [from[0], 0.45, from[2]], [to[0], 0.45, to[2]], None) || crate::scene_collision::segment_blocked_dynamic(g, [from[0], 1.2, from[2]], [to[0], 1.2, to[2]], None) { return false; }
    let n = m::ceil(l / 0.15);
    let mut i = 1.0;
    while i <= n {
        let t = i / n;
        let mut q = [from[0] + dx * t, 0.0, from[2] + dz * t];
        if crate::scene_collision::collide_circle(g, &mut q, UNSTICK_BODY_R, 0.2, 1.5, Some(1.0)) { return false; }
        i += 1.0;
    }
    true
}

/// The free cell centre nearest to `from` within `reach` that passes `test`.   (guards.ts freeCellNear)
pub fn free_cell_near(g: &mut crate::game::Game, from: V3, reach: f64, test: crate::guards::FreeCellNearTest) -> Option<V3> {
    let cs = g.col.nav.cell;
    let r = m::ceil(reach / cs);
    let cx = m::floor(from[0] / cs);
    let cz = m::floor(from[2] / cs);
    let mut cands: Vec<(V3, f64)> = Vec::new();
    let mut dz = -r;
    while dz <= r {
        let mut dx = -r;
        while dx <= r {
            let x = cx + dx;
            let z = cz + dz;
            dx += 1.0;
            if x < 0.0 || z < 0.0 || x >= g.col.nav.w || z >= g.col.nav.h { continue; }
            let ix = crate::scene_collision::nav_grid_idx(g, x, z) as usize;
            if g.col.nav.blocked[ix] != 0 { continue; }
            let q = [(x + 0.5) * cs, 0.0, (z + 0.5) * cs];
            let d = v3::dist_xz(from, q);
            if d <= reach { cands.push((q, d)); }
        }
        dz += 1.0;
    }
    js::sort_by(&mut cands, |a, b| a.1 - b.1);
    if test == crate::guards::FreeCellNearTest::Walk {
        for c in cands.iter() {
            if leg_clear(g, from, c.0) { return Some(c.0); }
        }
        return None;
    }
    for c in cands.iter() {
        if !crate::scene_collision::solid_blocked(g, [from[0], 2.0, from[2]], [c.0[0], 2.0, c.0[2]], None) { return Some(c.0); }
    }
    cands.first().map(|c| c.0)
}

/// Put him on the free cell centre `to` (or the nearest free one) here and now: the last resort, counted, never spoken.   (guards.ts unstickSnap)
pub fn unstick_snap(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>, why: &str) {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let p = match to {
        Some(p) => Some(p),
        None => match free_cell_near(g, pos, UNSTICK_SNAP_REACH_M, crate::guards::FreeCellNearTest::Room) {
            Some(p) => Some(p),
            None => crate::scene_collision::nav_grid_nearest_free_point(g, pos[0], pos[2]),
        },
    };
    let p = match p { Some(p) => p, None => return };
    g.ch[c].pos[0] = p[0];
    g.ch[c].pos[2] = p[2];
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.gd[gd].speed = 0.0;
    g.gd[gd].last_pos = v3::copy(g.ch[c].pos);
    g.gd[gd].stall_ref = None;
    g.gd[gd].stuck_t = 0.0;
    g.m.guards.unstick.snaps += 1.0;
    g.m.guards.unstick.last_snap_t = g.time;
    g.m.guards.unstick.last_snap_who = Some(gd);
}

/// The side-step he has going, if it still means anything.   (guards.ts liveLeg)
pub fn live_leg(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::guards::UnstickLeg> {
    let u = *g.m.guards.unstick_legs.get(gd)?;
    let pos = g.ch[g.gd[gd].ch].pos;
    if v3::dist_xz(pos, u.to) > 0.36 && !crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) {
        g.m.guards.unstick_legs.delete(gd);
        return None;
    }
    Some(u)
}

/// If he stands stranded and has no side-step going, begin one, or snap.   (guards.ts unstickBegin)
pub fn unstick_begin(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::UnstickBegin {
    let c = g.gd[gd].ch;
    if live_leg(g, gd).is_some() { return crate::guards::UnstickBegin::Leg; }
    let pos = g.ch[c].pos;
    if g.gd[gd].state == crate::game::GuardState::Dead
        || g.gd[gd].held.is_some()
        || !stranded_at(g, pos[0], pos[2])
        || g.time < g.m.guards.unstick_retry_at.get(gd).copied().unwrap_or(-1.0)
    {
        return crate::guards::UnstickBegin::No;
    }
    let to = match free_cell_near(g, pos, UNSTICK_REACH_M, crate::guards::FreeCellNearTest::Walk) {
        Some(to) => to,
        None => {
            unstick_snap(g, gd, None, "stranded, no free cell within reach he can walk to");
            let pos = g.ch[c].pos;
            if stranded_at(g, pos[0], pos[2]) {
                let t = g.time + 2.0;
                g.m.guards.unstick_retry_at.set(gd, t);
                return crate::guards::UnstickBegin::No;
            }
            return crate::guards::UnstickBegin::Snapped;
        }
    };
    g.m.guards.unstick_legs.set(gd, crate::guards::UnstickLeg { to, best: v3::dist_xz(pos, to), stall_t: 0.0 });
    g.m.guards.unstick.legs += 1.0;
    crate::guards::UnstickBegin::Leg
}

/// One frame of the side-step: `u` is the frame's copy of his row of unstick_legs; what it remembers is written through the table.   (guards.ts unstickStep)
pub fn unstick_step(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> crate::guards::UnstickStep {
    let u = *g.m.guards.unstick_legs.get(gd).unwrap();
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let dx = u.to[0] - pos[0];
    let dz = u.to[2] - pos[2];
    let d = m::hyp2(dx, dz);
    g.gd[gd].last_pos = v3::copy(pos);
    if d < UNSTICK_DONE_M || (d > 0.36 && !crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2])) { return crate::guards::UnstickStep::Done; }
    if d > UNSTICK_REACH_M + 0.6 { return crate::guards::UnstickStep::Stale; }
    if d < u.best - 0.02 {
        if let Some(row) = g.m.guards.unstick_legs.get_mut(gd) { row.best = d; row.stall_t = 0.0; }
    } else {
        let stall_t = u.stall_t + dt;
        if let Some(row) = g.m.guards.unstick_legs.get_mut(gd) { row.stall_t = stall_t; }
        if stall_t > UNSTICK_STALL_SECS {
            if crate::scene_collision::nav_grid_is_blocked(g, pos[0], pos[2]) {
                let why = format!("no nearer than {} m in {} s of stepping", js::to_fixed(u.best, 2), UNSTICK_STALL_SECS);
                unstick_snap(g, gd, Some(u.to), &why);
            }
            return crate::guards::UnstickStep::Done;
        }
    }
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, g.tune.guard_walk * v3::clamp(d / 0.3, 0.4, 1.0), 6.0, dt);
    let speed = g.gd[gd].speed;
    let step = m::min(d, speed * dt);
    g.ch[c].pos[0] += dx / d * step;
    g.ch[c].pos[2] += dz / d * step;
    g.ch[c].vel = [dx / d * speed, 0.0, dz / d * speed];
    crate::guards::UnstickStep::Walking
}

/// The leg ahead of any path-following this frame; when it ends, plan the remembered goal afresh. True = the leg had this frame.   (guards.ts unstickFirst)
pub fn unstick_first(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> bool {
    if !g.m.guards.unstick_legs.has(gd) { return false; }
    let r = unstick_step(g, gd, dt);
    if r == crate::guards::UnstickStep::Walking { return true; }
    g.m.guards.unstick_legs.delete(gd);
    let goal = g.gd[gd].path_goal;
    lay_path(g, gd, Vec::new());
    g.gd[gd].path_goal = None;
    g.gd[gd].path_goal_orbit = false;
    if let Some(goal) = goal { go_to(g, gd, goal, Some(true)); }
    g.m.guards.unstick_legs.has(gd)
}

/// Plan him a way to `goal`: a side-step under way first, a bee-line only where a man his width can walk it, else an empty path.   (guards.ts goTo)
pub fn go_to(g: &mut crate::game::Game, gd: GuardId, goal: V3, force: Option<bool>) {
    let force = force.unwrap_or(false);
    if !force {
        if let Some(pg) = g.gd[gd].path_goal {
            if v3::dist(pg, goal) < 0.75 && !g.gd[gd].path.is_empty() { return; }
        }
    }
    if let Some(u0) = live_leg(g, gd) {
        lay_path(g, gd, vec![v3::copy(u0.to)]);
        g.gd[gd].path_i = 0;
        g.gd[gd].path_goal = Some(v3::copy(goal));
        g.gd[gd].path_goal_orbit = false;
        g.gd[gd].repath_t = 0.0;
        g.gd[gd].line_side = 0.0;
        return;
    }
    let tactics = crate::tactics::tactics_penalty(g, gd, goal);
    let pen = crate::lanes::lane_penalty(g, gd, tactics.as_ref());
    let round = if crate::teampace::team_at_walk(g, gd) { crate::orders::prop_penalty(g, &crate::seats::here(g), gd, pen.as_ref()) } else { None };
    let mut p = go_to_plan(g, gd, goal, pen.as_ref(), round.as_ref());
    if p.is_none() {
        let u = unstick_begin(g, gd);
        if u == crate::guards::UnstickBegin::Leg {
            let to = g.m.guards.unstick_legs.get(gd).unwrap().to;
            lay_path(g, gd, vec![v3::copy(to)]);
            g.gd[gd].path_i = 0;
            g.gd[gd].path_goal = Some(v3::copy(goal));
            g.gd[gd].path_goal_orbit = false;
            g.gd[gd].repath_t = 0.0;
            g.gd[gd].line_side = 0.0;
            return;
        }
        if u == crate::guards::UnstickBegin::Snapped { p = go_to_plan(g, gd, goal, pen.as_ref(), round.as_ref()); }
    }
    let way = p.unwrap_or_default();
    let start = g.ch[g.gd[gd].ch].pos;
    let mut is_free = |g: &mut Game, x: f64, z: f64| -> bool { !crate::scene_collision::nav_grid_is_blocked(g, x, z) };
    let threaded = crate::doors::doors_thread_path(g, &way, start, Some(&mut is_free));
    lay_path(g, gd, threaded);
    g.gd[gd].path_i = 0;
    g.gd[gd].path_goal = Some(v3::copy(goal));
    g.gd[gd].path_goal_orbit = false;
    g.gd[gd].repath_t = 0.0;
    g.gd[gd].line_side = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GuardState;

    // the TypeScript's own bits, under bun with EXACT on and the page's GUARD_EYE
    #[test]
    fn dist_term_is_the_typescripts_bits() {
        let k = crate::knobs::GuardEyeK::page();
        for (d, bits) in [
            (0f64, 0x3ff0000000000000u64),
            (3f64, 0x3ff0000000000000u64),
            (7.3f64, 0x3fe56a56a56a56a6u64),
            (13.4f64, 0x3fc999999999999au64),
            (15.999f64, 0x3fc999999999999au64),
            (16f64, 0x3fc999999999999au64),
            (16.1f64, 0x3fc978d4fdf3b646u64),
            (19.7f64, 0x3fc4dd2f1a9fbe77u64),
            (25.9f64, 0x3fb9db22d0e56043u64),
            (26f64, 0x3fb999999999999au64),
            (40f64, 0x3fb999999999999au64),
        ] {
            assert_eq!(dist_term_of(&k, d).to_bits(), bits, "{}", d);
        }
    }

    #[test]
    fn eye_reach_is_the_typescripts_bits() {
        let k = crate::knobs::GuardEyeK::page();
        for (l, bits) in [
            (0f64, 0x4030000000000000u64),
            (0.39f64, 0x4030000000000000u64),
            (0.4f64, 0x4030000000000000u64),
            (0.41f64, 0x403004b851eb851fu64),
            (0.55f64, 0x40332a0000000000u64),
            (0.63f64, 0x40361dd70a3d70a4u64),
            (0.79f64, 0x4039fb47ae147ae1u64),
            (0.8f64, 0x403a000000000000u64),
            (1f64, 0x403a000000000000u64),
        ] {
            assert_eq!(reach_of(&k, l).to_bits(), bits, "{}", l);
        }
    }

    #[test]
    fn a_far_glimpse_writes_a_fix_by_his_state() {
        let k = crate::knobs::GuardEyeK::page();
        // inside `range`: never faint
        assert!(!faint_of(&k, GuardState::Patrol, 0.0, 16.0, 0.0));
        // calm, past `range`: only the frame the sight itself tips him over 0.3
        assert!(!faint_of(&k, GuardState::Patrol, 0.3, 20.0, 0.29));
        assert!(faint_of(&k, GuardState::Patrol, 0.3, 20.0, 0.3));
        assert!(faint_of(&k, GuardState::Patrol, 0.29, 20.0, 0.1));
        // wary: from farFixAt
        assert!(faint_of(&k, GuardState::Suspicious, 0.49, 20.0, 0.0));
        assert!(!faint_of(&k, GuardState::Suspicious, 0.5, 20.0, 0.0));
        // hunting: every glimpse
        assert!(!faint_of(&k, GuardState::Alert, 0.0, 25.0, 0.0));
    }
}
