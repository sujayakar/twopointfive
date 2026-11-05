//! src/game/guards.ts lines 1128 to 1500
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

/// lanePenalty(g, gd, base) where base is the price tacticsPenalty made: asked with no base, lanes.ts answers None (it leaves him to `base`) or the mask's own price, and the base goes under it
fn lane_over(asked: Option<crate::lanes::LanePenalty>, base: Option<crate::tactics::TacticsPenalty>) -> Option<crate::lanes::LanePenalty> {
    match (asked, base) {
        (None, None) => None,
        (None, Some(base)) => Some(crate::lanes::LanePenalty::Base { base }),
        (Some(crate::lanes::LanePenalty::Dear { m, w, dear }), Some(base)) => Some(crate::lanes::LanePenalty::DearOver { base, m, w, dear }),
        (Some(p), _) => Some(p),
    }
}

/// tacticsPenalty's closure
fn price_tactics(p: &crate::tactics::TacticsPenalty, ix: f64, iz: f64) -> f64 {
    let crate::tactics::TacticsPenalty::Cells { w, door_idx, door_mult, avoid } = p;
    let i = crate::Num(iz * *w + ix);
    let mut cost = 1.0;
    if let Some(d) = door_idx { if d.has(i) { cost = *door_mult; } }
    if let Some(a) = avoid { if a.has(i) { cost = m::max(cost, crate::tactics::CORRIDOR_COST); } }
    cost
}

/// `m[iz * W + ix] ? dear : 1` (past the end is undefined, so 1)
fn price_mask(mask: &[u8], w: f64, dear: f64, ix: f64, iz: f64) -> f64 {
    let lit = mask.get((iz * w + ix) as usize).map_or(false, |v| *v != 0);
    if lit { dear } else { 1.0 }
}

/// priceOf's closures
fn price_lane(p: &crate::lanes::LanePenalty, ix: f64, iz: f64) -> f64 {
    match p {
        crate::lanes::LanePenalty::Dear { m: mask, w, dear } => price_mask(mask, *w, *dear, ix, iz),
        crate::lanes::LanePenalty::DearOver { base, m: mask, w, dear } => m::max(price_tactics(base, ix, iz), price_mask(mask, *w, *dear, ix, iz)),
        crate::lanes::LanePenalty::Base { base } => price_tactics(base, ix, iz),
    }
}

/// propPenalty's closure
fn price_prop(g: &mut Game, p: &crate::orders::PropPenalty, ix: f64, iz: f64) -> f64 {
    let crate::orders::PropPenalty::Dear { dear, cost, pen } = p;
    let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
    let here = if dear.has(crate::Num(i)) { *cost } else { 1.0 };
    here * match pen { Some(q) => price_lane(q, ix, iz), None => 1.0 }
}

/// goTo's plan. `round` None is the TypeScript's `round === pen`: no second search.
fn go_to_plan(g: &mut Game, gd: GuardId, goal: V3, pen: Option<&crate::lanes::LanePenalty>, round: Option<&crate::orders::PropPenalty>) -> Option<Vec<V3>> {
    let from = g.ch[g.gd[gd].ch].pos;
    let mut p = match pen {
        Some(q) => crate::scene_collision::nav_grid_find_path(g, from, goal, Some(6000.0), Some(&mut |_g: &mut Game, ix: f64, iz: f64| price_lane(q, ix, iz))),
        None => crate::scene_collision::nav_grid_find_path(g, from, goal, Some(6000.0), None),
    };
    if let (Some(plain), Some(q)) = (p.as_ref(), round) {
        let r = crate::scene_collision::nav_grid_find_path(g, from, goal, Some(6000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| price_prop(g, q, ix, iz)));
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
            if !g.gd[gd].path_goal_orbit {
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
        g.gd[gd].fire_cd = (STANDOFF_SHOT_SECS + crate::math_dice::sim_random(g) * 0.6) * mods.gap;
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
    let p = pair_of(g);
    let pair = g.tab.guards_standoff[s].pair;
    let why = if g.quiet_utility || g.gd[gd].hold || g.gd[gd].pinned || !g.ai_enabled {
        Some("off")
    } else if g.gd[gd].state != crate::game::GuardState::Alert {
        Some("state")
    } else if p.map_or(true, |q| q.held != pair) {
        Some(if g.gd[pair].state == crate::game::GuardState::Dead { "dead" } else if g.pl[g.player].down { "samDown" } else { "free" })
    } else if g.time - g.tab.guards_standoff[s].seen_t > STANDOFF_LOSE_SECS && !crate::gunhold::gun_hold_keeps(g, gd) {
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
            SHIELD_RELEASE_BEAT_MIN + crate::math_dice::sim_random(g) * SHIELD_RELEASE_BEAT_JITTER
        } else {
            let cd = g.gd[gd].fire_cd;
            m::min(cd, 0.15 + crate::math_dice::sim_random(g) * 0.3)
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
    let p = pair_of(g);
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
            Some(f) => format!("{:.1} m, {}° off their rear, clear {:.2}{}", f.dist, m::round(f.rear / v3::DEG), f.clear, if f.ok { " LINE" } else { "" }),
            None => String::from("pair gone"),
        };
        let so = g.tab.guards_standoff[s];
        let feet = match so.orbit_goal {
            Some(o) => format!("to ({:.1}, {:.1})", o[0], o[2]),
            None => String::from(if so.cornered { "cornered" } else if so.rest_t > 0.0 { "resting" } else { "planted" }),
        };
        parts.push(format!(
            "{}: {} · side {} · {} · {} shot{}, {:.1} m walked, seen {:.1} s ago",
            g.gd[gd].callsign, line, if so.side > 0.0 { "+" } else { "−" }, feet, so.shots, if so.shots == 1.0 { "" } else { "s" }, so.orbit_m, g.time - so.seen_t
        ));
    }
    parts.join(" | ")
}

/// The player he has in mind, in the view of the seat that has this man.   (guards.ts canSee)
pub fn can_see(g: &mut crate::game::Game, gd: GuardId) -> crate::guards::CanSee {
    let c = g.gd[gd].ch;
    let id = g.ch[c].id;
    let seat = crate::seats::view_of_body(g, crate::seats::ViewOfBodyC { id });
    // (a game of one seat, or a view handed in: the seat is already the one on Game, and there is nothing to swap)
    let r = if seat == g.player {
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

/// canSee against any man `pc`; a teammate with his long gun in the shoulder also sees an AI man along his sights.   (guards.ts canSeeChar)
pub fn can_see_char(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool) -> crate::guards::CanSeeChar {
    let c = g.gd[gd].ch;
    let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(h) => *h,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    };
    let chest_dir = g.ch[c].chest_dir;
    let head_dir = crate::character::character_head_dir(g, c);
    let look = v3::normalize(v3::lerp3(chest_dir, head_dir, 0.5));
    let range = reach_for(g, gd, pc, is_player, eye);
    let mut e = crate::guards::Eye { pos: eye, look, range, half_deg: g.k.guard_eye.half_deg, near_m: g.k.guard_eye.near_m, near_deg: g.k.guard_eye.near_deg, v_half_deg: None };
    let through = eye_through(g, gd);
    let blind = g.time < g.gd[gd].dazzled_until || crate::pairorders::pair_gripped(g, gd) || crate::doze::doze_asleep(g, gd);
    let smoke_trans = g.gd[gd].smoke_trans;
    let mut r = eye_contact(g, e, pc, is_player, blind, smoke_trans, Some(through));
    // ROOMFIGHT.sights on, sightsFoe off: the eye along the sights is an operative's alone
    if !r.in_cone && r.dist < e.range && !is_player && g.gd[gd].team == crate::teams::Team::Operative && crate::roomfight::fight_live(g) {
        let gun_yaw = crate::character::character_gun_yaw(g, c);
        let (ax, az) = (m::sin(gun_yaw), m::cos(gun_yaw));
        let gun_dir = g.ch[c].gun_dir;
        let mut lv = gun_dir[0] * ax + gun_dir[2] * az;
        let lift = g.ch[c].anim.sights_lift;
        if lift > 0.0 {
            let gh = m::hyp2(gun_dir[0], gun_dir[2]);
            if gh > 1e-6 { lv *= m::cos(lift) - gun_dir[1] / gh * m::sin(lift); }
        }
        if g.ch[c].anim.long_shown == Some(crate::character::LongStance::Shoulder) && !crate::character::anim_at_well(g, c) && lv > SIGHTS_COS {
            e.look = [ax, 0.0, az];
            e.half_deg = m::min(e.half_deg, g.k.roomfight.sights_deg);
            let blind = g.time < g.gd[gd].dazzled_until || crate::pairorders::pair_gripped(g, gd) || crate::doze::doze_asleep(g, gd);
            let smoke_trans = g.gd[gd].smoke_trans;
            r = eye_contact(g, e, pc, false, blind, smoke_trans, Some(through));
        }
    }
    if !is_player && r.visible && crate::roomfight::pair_smoke(g, c, pc) < 0.3 { r.visible = false; }
    crate::guards::CanSeeChar { visible: r.visible, dist: r.dist, in_cone: r.in_cone }
}

/// How far a man's eye reaches for a man with `lit` on him.   (guards.ts eyeReach)
pub fn eye_reach(g: &mut crate::game::Game, lit: f64) -> f64 {
    let k = g.k.guard_eye;
    if g.touring { return k.range; }
    reach_of(&k, lit)
}

/// eyeReach's sum, for a game not under a show
fn reach_of(k: &crate::knobs::GuardEyeK, lit: f64) -> f64 {
    k.range + m::max(0.0, k.far_range - k.range) * v3::smoothstep(k.far_lo, k.far_hi, lit)
}

/// …for the man `pc` in this man's view.   (guards.ts reachFor)
pub fn reach_for(g: &mut crate::game::Game, gd: GuardId, pc: CharId, is_player: bool, eye: V3) -> f64 {
    if g.touring { return g.k.guard_eye.range; }
    if is_player {
        let vis = g.pl[g.player].visibility;
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
pub fn far_faint(g: &mut crate::game::Game, gd: GuardId, dist: f64, was: f64) -> bool {
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
pub fn dist_term(g: &mut crate::game::Game, dist: f64) -> f64 {
    let k = g.k.guard_eye;
    dist_term_of(&k, dist)
}

/// distTerm's sum
fn dist_term_of(k: &crate::knobs::GuardEyeK, dist: f64) -> f64 {
    if dist > k.range { return v3::lerp(0.2, k.far_k, v3::clamp((dist - k.range) / m::max(1e-3, k.far_range - k.range), 0.0, 1.0)); }
    v3::clamp(1.0 - (dist - 3.0) / 13.0, 0.2, 1.0)
}

/// What the team's own eyes pass (FOG.mates on: what the fog's lines pass).   (guards.ts teamThrough)
pub fn team_through(g: &mut crate::game::Game) -> f64 {
    crate::fogofwar::fog_through(g)
}

/// What a man's eye passes: the team's for an operative, glass alone for every other side.   (guards.ts eyeThrough)
pub fn eye_through(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if g.gd[gd].team == crate::teams::Team::Operative { team_through(g) } else { crate::scene_boxes::box_flag().glass }
}

/// That man's own line to a point, asked bare.   (guards.ts eyeBlocked)
pub fn eye_blocked(g: &mut crate::game::Game, gd: GuardId, a: V3, b: V3) -> bool {
    let through = eye_through(g, gd);
    crate::scene_collision::segment_blocked(g, a, b, Some(0.02), Some(through), None)
}

/// THE line-of-sight test, whoever's eye it is. RUST.sight answers through the collision's wrapper; None = this file answers.   (guards.ts eyeContact)
pub fn eye_contact(g: &mut crate::game::Game, e: crate::guards::Eye, pc: CharId, is_player: bool, blind: bool, smoke_trans: f64, through: Option<f64>) -> crate::guards::EyeContact {
    let through = match through { Some(t) => t, None => crate::scene_boxes::box_flag().glass };
    let tgt = match g.ch[pc].bones.get(crate::character::Bone::Chest) { Some(b) => *b, None => v3::add(g.ch[pc].pos, [0.0, 1.1, 0.0]) };
    let tgt2 = match g.ch[pc].bones.get(crate::character::Bone::Head) { Some(b) => *b, None => v3::add(g.ch[pc].pos, [0.0, 1.6, 0.0]) };
    let smoked = is_player && smoke_trans < 0.3;
    let unseen = blind || (!is_player && !g.ch[pc].alive);
    let lens = crate::scene_collision::EyeContactE { pos: e.pos, look: e.look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: e.v_half_deg };
    let r = match crate::scene_collision::eye_contact(g, lens, tgt, tgt2, unseen, smoked, through) {
        Some(r) => r,
        None => {
            let t = eye_contact_here(g, e, pc, is_player, tgt, tgt2, blind, smoked, through);
            return crate::guards::EyeContact { visible: t.visible, dist: t.dist, in_cone: t.in_cone };
        }
    };
    let mut visible = r.visible;
    if visible && is_player && (g.player_invisible || g.pl[g.player].down) { visible = false; }
    crate::guards::EyeContact { visible, dist: r.dist, in_cone: r.in_cone }
}

/// eyeContact as this file answers it, wherever the module does not.   (guards.ts eyeContactHere)
pub fn eye_contact_here(g: &mut crate::game::Game, e: crate::guards::Eye, pc: CharId, is_player: bool, tgt: V3, tgt2: V3, blind: bool, smoked: bool, through: f64) -> crate::guards::EyeContactHere {
    let eye = e.pos;
    let to = v3::sub(tgt, eye);
    let dist = v3::len(to);
    let dir_to = v3::scale(to, 1.0 / m::max(dist, 1e-4));
    let look = e.look;
    let cos_a = v3::dot([look[0], 0.0, look[2]], v3::normalize([dir_to[0], 0.0, dir_to[2]]));
    let fov = if dist < e.near_m { m::cos(e.near_deg * v3::DEG) } else { m::cos(e.half_deg * v3::DEG) };
    let mut in_cone = cos_a > fov && dist < e.range;
    if in_cone {
        if let Some(v) = e.v_half_deg {
            in_cone = m::abs(m::asin(v3::clamp(dir_to[1], -1.0, 1.0)) - m::asin(v3::clamp(look[1], -1.0, 1.0))) < v * v3::DEG;
        }
    }
    if !in_cone || (if is_player { g.player_invisible || g.pl[g.player].down } else { !g.ch[pc].alive }) || blind { return crate::guards::EyeContactHere { visible: false, dist, in_cone }; }
    let blocked = (crate::scene_collision::segment_blocked(g, eye, tgt, Some(0.02), Some(through), None) && crate::scene_collision::segment_blocked(g, eye, tgt2, Some(0.02), Some(through), None)) || smoked;
    crate::guards::EyeContactHere { visible: !blocked, dist, in_cone }
}

/// Is the nav cell under (x, z) one no plan can leave?   (guards.ts strandedAt)
pub fn stranded_at(g: &mut crate::game::Game, x: f64, z: f64) -> bool {
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
                let why = format!("no nearer than {:.2} m in {} s of stepping", u.best, UNSTICK_STALL_SECS);
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
    let asked = crate::lanes::lane_penalty(g, gd, None);
    let pen = lane_over(asked, tactics);
    let round = if crate::teampace::team_at_walk(g, gd) { crate::orders::prop_penalty(g, gd, pen.as_ref()) } else { None };
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
        let k = crate::knobs::page_guard_eye();
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
        let k = crate::knobs::page_guard_eye();
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

    fn doors() -> crate::tactics::TacticsPenalty {
        let mut door_idx = JsSet::new();
        door_idx.add(crate::Num(5.0));   // W 4: cell (1, 1)
        door_idx.add(crate::Num(6.0));   // (2, 1), which is also in the flank's way
        let mut avoid = JsSet::new();
        avoid.add(crate::Num(6.0));
        avoid.add(crate::Num(7.0));      // (3, 1)
        crate::tactics::TacticsPenalty::Cells { w: 4.0, door_idx: Some(door_idx), door_mult: 3.0, avoid: Some(avoid) }
    }

    #[test]
    fn the_tactics_price_is_the_dearer_of_door_and_flank() {
        let p = doors();
        assert_eq!(price_tactics(&p, 0.0, 0.0), 1.0);
        assert_eq!(price_tactics(&p, 1.0, 1.0), 3.0);
        assert_eq!(price_tactics(&p, 2.0, 1.0), crate::tactics::CORRIDOR_COST);
        assert_eq!(price_tactics(&p, 3.0, 1.0), crate::tactics::CORRIDOR_COST);
    }

    #[test]
    fn a_lane_goes_over_the_tactics_price() {
        use crate::lanes::LanePenalty;
        let mask = vec![0u8, 1, 0, 0, 0, 1, 0, 0];   // (1, 0) and (1, 1)
        let dear = || LanePenalty::Dear { m: mask.clone(), w: 4.0, dear: 4.0 };
        // the rule leaves him alone: nothing, or the base as it was
        assert!(lane_over(None, None).is_none());
        match lane_over(None, Some(doors())) { Some(LanePenalty::Base { .. }) => {}, x => panic!("{:?}", x) }
        // the mask alone
        let only = lane_over(Some(dear()), None).unwrap();
        assert_eq!(price_lane(&only, 1.0, 0.0), 4.0);
        assert_eq!(price_lane(&only, 0.0, 0.0), 1.0);
        // Math.max(base, mask): the dearer, whichever it is
        let both = lane_over(Some(dear()), Some(doors())).unwrap();
        assert_eq!(price_lane(&both, 1.0, 0.0), 4.0);                              // mask, base 1
        assert_eq!(price_lane(&both, 1.0, 1.0), 4.0);                              // mask 4 over door 3
        assert_eq!(price_lane(&both, 2.0, 1.0), crate::tactics::CORRIDOR_COST);    // flank 6 over a clear cell
        assert_eq!(price_lane(&both, 0.0, 0.0), 1.0);
    }

    #[test]
    fn a_far_glimpse_writes_a_fix_by_his_state() {
        let k = crate::knobs::page_guard_eye();
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
