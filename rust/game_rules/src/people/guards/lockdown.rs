use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// One more round for a side.
fn rounds_add(r: &mut crate::guards::EngagementRounds, t: crate::teams::Team) {
    let slot = match t {
        crate::teams::Team::Guard => &mut r.guard,
        crate::teams::Team::Ours => &mut r.operative,
        crate::teams::Team::Hostile => &mut r.hostile,
        crate::teams::Team::Civilian => &mut r.civilian,
    };
    *slot = Some(slot.unwrap_or(0.0) + 1.0);
}

/// The same chokepoint, by value: a man's post is a copy of the level's row.
fn same_post(a: &crate::scene_level::Chokepoint, b: &crate::scene_level::Chokepoint) -> bool {
    a.name == b.name && a.pos == b.pos && a.yaw == b.yaw
}

/// Deal the lockdown duties: the nearest two pair up, a third holds the junction nearest the fix.
pub(super) fn assign_lockdown(g: &mut crate::game::Game, redeal: bool, except: Option<GuardId>) {
    let (mut old_lead, mut old_follow, mut old_post_man): (Option<GuardId>, Option<GuardId>, Option<GuardId>) = (None, None, None);
    let mut old_post: Option<crate::scene_level::Chokepoint> = None;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        if let Some(l) = g.gd[gd].leader { old_follow = Some(gd); old_lead = Some(l); }
        if g.gd[gd].post.is_some() { old_post = g.gd[gd].post.clone(); old_post_man = Some(gd); }
    }
    clear_lockdown(g);
    g.alarm.apart_t = 0.0;
    let mut alive: Vec<GuardId> = Vec::new();
    for &x in g.guards.iter() {
        let m_ = &g.gd[x];
        if !g.cond[m_.ch].down && m_.held.is_none() && g.ch[m_.ch].team == crate::teams::Team::Guard && m_.unit.is_none() { alive.push(x); }
    }
    if alive.len() < 2 { return; }
    let (mut a, mut b, mut bd) = (alive[0], alive[1], m::INF);
    let clearing: Option<(GuardId, Option<GuardId>)> = g.clearing.as_ref().map(|c| (c.a, c.b));
    let kept = match clearing {
        Some((ca, Some(cb))) if alive.contains(&ca) && alive.contains(&cb) => Some((ca, cb)),
        _ => None,
    };
    if let Some((ca, cb)) = kept {
        a = ca;
        b = cb;
    } else {
        for i in 0..alive.len() {
            for j in (i + 1)..alive.len() {
                let (ci, cj) = (g.gd[alive[i]].ch, g.gd[alive[j]].ch);
                let d = v3::dist(g.ch[ci].pos, g.ch[cj].pos);
                if d < bd { bd = d; a = alive[i]; b = alive[j]; }
            }
        }
    }
    let lead = if clearing.map_or(false, |(ca, _)| a == ca) { a } else if route_length(g, a) >= route_length(g, b) { a } else { b };
    let follow = if lead == a { b } else { a };
    let lead_c = g.gd[lead].ch;
    let lead_pos = g.ch[lead_c].pos;
    let now = g.time;
    {
        let f = &mut g.gd[follow];
        f.leader = Some(lead);
        f.follow_t = 0.0;
        f.leader_still_t = 0.0;
        f.drop_path();
        f.leader_seen_t = now;
        f.leader_seen_pos = v3::copy(lead_pos);
    }
    if redeal
        && (Some(lead) != old_lead || Some(follow) != old_follow)
        && Some(lead) != except
        && g.gd[lead].state != crate::game::GuardState::Alert
        && !g.gd[lead].bubble.as_ref().map_or(false, |bu| now - bu.t < 0.5)
    {
        crate::game::game_say(g, lead, "you're with me — close up", None, None);
    }
    let third = alive.iter().copied().find(|&x| x != a && x != b);
    let cps: Vec<crate::scene_level::Chokepoint> = g.level.chokepoints.clone().unwrap_or_default();
    if let Some(third) = third {
        if !cps.is_empty() {
            let third_c = g.gd[third].ch;
            let at = if g.alarm.placed { g.alarm.pos } else { g.ch[third_c].pos };
            let (mut bi, mut cd) = (0, m::INF);
            for (k, cp) in cps.iter().enumerate() {
                let d = v3::dist(cp.pos, at);
                if d < cd { cd = d; bi = k; }
            }
            let best = cps[bi].clone();
            {
                let t = &mut g.gd[third];
                t.post = Some(best.clone());
                t.follow_t = 0.0;
                t.drop_path();
            }
            if (old_post.as_ref().map_or(true, |o| !same_post(o, &best)) || Some(third) != old_post_man)
                && Some(third) != except
                && g.gd[third].state != crate::game::GuardState::Alert
            {
                crate::game::game_say(g, third, &format!("holding the {}", best.name), Some(true), None);
            }
        }
    }
}

/// Take every man off his lockdown duty.
pub(super) fn clear_lockdown(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].leader.is_some() || g.gd[gd].post.is_some() { g.gd[gd].drop_path(); }
        g.gd[gd].leader = None;
        g.gd[gd].post = None;
    }
}

/// How long a man's route is, round the loop.
fn route_length(g: &crate::game::Game, gd: GuardId) -> f64 {
    let Ok(ri) = usize::try_from(g.gd[gd].route_i) else { return 0.0; };   // (a unit's is -1)
    let Some(&r) = g.level.routes.get(ri) else { return 0.0; };
    let pts = &g.tab.route[r].points;
    let mut s = 0.0;
    for i in 0..pts.len() { s += v3::dist(pts[i], pts[(i + 1) % pts.len()]); }
    s
}

/// Lockdown follower on patrol: keep station behind-left of the leader as he knows him. True while walking.
pub(super) fn follow_leader(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    let l = g.gd[gd].leader.unwrap();
    let (c, lc) = (g.gd[gd].ch, g.gd[l].ch);
    let fresh = g.gd[gd].leader_seen_t < 0.0 || g.time - g.gd[gd].leader_seen_t < PARTNER_TRUST_SECS;
    let lp = if fresh { g.ch[lc].pos } else { g.gd[gd].leader_seen_pos };
    g.gd[gd].follow_t -= dt;
    let lvel = g.ch[lc].vel;
    g.gd[gd].leader_still_t = if !fresh || m::hyp2(lvel[0], lvel[2]) < 0.15 { g.gd[gd].leader_still_t + dt } else { 0.0 };
    if g.gd[gd].follow_t <= 0.0 {
        g.gd[gd].follow_t = 0.5;
        let f = crate::character::character_forward(g, lc);
        let lv: V3 = if fresh { g.ch[lc].vel } else { [0.0, 0.0, 0.0] };
        let mut goal = g.gd[gd].follow_goal;
        goal[0] = lp[0] - f[0] * 1.3 + f[2] * 0.45;
        goal[1] = 0.0;
        goal[2] = lp[2] - f[2] * 1.3 - f[0] * 0.45;
        if crate::scene_collision::nav_grid_is_blocked(g, goal[0], goal[2]) || !crate::scene_collision::nav_grid_walkable(g, lp, goal) {
            goal[0] = lp[0] - f[0] * 1.2;
            goal[2] = lp[2] - f[2] * 1.2;
        }
        if crate::scene_collision::nav_grid_is_blocked(g, goal[0], goal[2]) || !crate::scene_collision::nav_grid_walkable(g, lp, goal) {
            goal[0] = lp[0];
            goal[2] = lp[2];
        } else if !crate::scene_collision::nav_grid_is_blocked(g, goal[0] + lv[0] * 0.6, goal[2] + lv[2] * 0.6) {
            goal[0] += lv[0] * 0.6;
            goal[2] += lv[2] * 0.6;
        }
        g.gd[gd].follow_goal = goal;
        let p = g.ch[c].pos;
        let off = v3::dist_xz(goal, p);
        let stale = match g.gd[gd].path_goal {
            None => true,
            Some(pg) => v3::dist_xz(pg, goal) > 0.5 || (g.gd[gd].path_i >= g.gd[gd].path.len() && off > 0.7),
        };
        if stale && off > 0.35 { go_to(g, gd, goal, Some(true)); }
    }
    let p = g.ch[c].pos;
    let gap = v3::dist_xz(lp, p);
    let speed = if gap > 4.5 { m::max(walk, g.tune.guard_investigate * 1.1) } else if gap > 2.0 { walk * 1.3 } else { walk };
    let arrived = follow_path(g, gd, dt, speed);
    if arrived {
        let look_at = g.gd[gd].leader_still_t > 6.0 && gap < 2.2;
        let lp = if fresh { g.ch[lc].pos } else { lp };
        let p = g.ch[c].pos;
        let want = if look_at { v3::yaw_to(p, lp) } else { g.ch[lc].body_yaw + 0.45 };
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, want, 3.0 * dt);
        let phase = g.gd[gd].look_phase;
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.6) * (if look_at { 0.25 } else { 0.6 }), 2.0, dt);
        g.ch[c].anim.look_yaw_extra = m::sin(phase * 1.1) * (if look_at { 0.15 } else { 0.4 });
    } else {
        let phase = g.gd[gd].look_phase;
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.7) * 0.3, 2.0, dt);
    }
    !arrived
}

/// Lockdown post on patrol: walk to the junction, then stand on it sweeping. True while walking.
pub(super) fn hold_post(g: &mut crate::game::Game, gd: GuardId, dt: f64, walk: f64) -> bool {
    let (ppos, pyaw) = { let p_ = g.gd[gd].post.as_ref().unwrap(); (p_.pos, p_.yaw) };
    let c = g.gd[gd].ch;
    g.gd[gd].follow_t -= dt;
    let p = g.ch[c].pos;
    let replan = g.gd[gd].follow_t <= 0.0
        && v3::dist_xz(ppos, p) > 0.45
        && match g.gd[gd].path_goal {
            None => true,
            Some(pg) => g.gd[gd].path_i >= g.gd[gd].path.len() || v3::dist_xz(pg, ppos) > 0.5,
        };
    if replan {
        g.gd[gd].follow_t = 0.5;
        go_to(g, gd, ppos, Some(true));
        let push = match g.gd[gd].path.last() {
            None => true,
            Some(last) => v3::dist_xz(*last, ppos) > 0.02,
        };
        if push { g.gd[gd].path.push(v3::copy(ppos)); }
    }
    let arrived = follow_path(g, gd, dt, walk);
    let phase = g.gd[gd].look_phase;
    if arrived {
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, pyaw, 2.5 * dt);
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.45) * 0.75, 2.0, dt);
        g.ch[c].anim.look_yaw_extra = m::sin(phase * 0.9) * 0.45;
    } else {
        g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(phase * 0.7) * 0.3, 2.0, dt);
    }
    !arrived
}

/// Housekeeping on a calm pass, one door a frame: pull a leaf to, throw a bolt again.
pub(super) fn lockdown_doors(g: &mut crate::game::Game, gd: GuardId, calm: Option<bool>) {
    let calm = calm.unwrap_or(false);
    let c = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    let mut di = 0;
    while di < g.doors.list.len() {
        let d = g.doors.list[di];
        di += 1;
        let (def_locked, def_exterior, def_closer) = { let def = &g.dr[d].def; (def.locked.unwrap_or(false), def.exterior.unwrap_or(false), def.closer) };
        if g.dr[d].leaf.off { continue; }
        let barred = if calm {
            !(def_locked || def_exterior) || def_closer == Some(false)
        } else {
            def_exterior && !(def_closer != Some(false))
        };
        if barred || g.time - g.dr[d].leaf.guard_t < 12.0 || g.dr[d].leaf.quiet_t < 0.6 { continue; }
        let angle = g.dr[d].leaf.angle;
        let open = !g.dr[d].leaf.latched && (calm || m::abs(angle) > 0.03);
        let relock = !calm && def_locked && !g.dr[d].leaf.locked && !g.dr[d].leaf.lock_broken;
        if !open && !relock { continue; }
        let fc = g.dr[d].frame_centre;
        let p = g.ch[c].pos;
        let (dx, dz) = (fc[0] - p[0], fc[2] - p[2]);
        let dd = m::hyp2(dx, dz);
        if dd > 2.0 || dd < 0.9 || (dx * f[0] + dz * f[2]) / dd > 0.55 { continue; }
        if open {
            let (hinge, cd, width) = { let def = &g.dr[d].def; (def.hinge, def.closed_dir, def.width) };
            let (px, pz) = (p[0] - hinge[0], p[2] - hinge[1]);
            let th = m::atan2(-m::sin(cd) * px + m::cos(cd) * pz, m::cos(cd) * px + m::sin(cd) * pz);
            if th * angle > 0.0 && m::abs(th) < m::abs(angle) + 0.4 && m::hyp2(px, pz) < width + 0.45 { continue; }
        }
        let mut busy = false;
        for &o in g.guards.iter() {
            if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
            let oc = g.gd[o].ch;
            let op = g.ch[oc].pos;
            if v3::dist_xz(op, fc) < 2.0 { busy = true; break; }
        }
        if busy { continue; }
        let now = g.time;
        g.dr[d].leaf.guard_t = now;
        if open { let who = g.ch[c].id; crate::doors::doors_pull_to(g, d, who); }
        if relock {
            let line = if g.dr[d].leaf.noticed { "locking this back up" } else if open { "shutting this — and it stays locked" } else { "someone left this unlocked — locking it" };
            crate::game::game_say(g, gd, line, None, None);
            g.dr[d].leaf.locked = true;
            g.dr[d].leaf.noticed = true;
            let now = g.time;
            g.dr[d].leaf.noticed_t = now;
            let at = crate::doors::door_pos(g, d);
            crate::audio_iface::game_audio_like_play(g, "lockOpen", Some(at), Some(0.6), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.85 }));
        }
        break;
    }
}

/// Stand an AI man of a side, held on a one-point route of his own. A refusal is the Err.
pub fn spawn_unit(g: &mut crate::game::Game, o: &crate::guards::UnitOpts) -> Result<GuardId, String> {
    if !g.tune.teams { return Err(String::from("spawnUnit: sides are switched off (Game.tune.teams) — no unit may be spawned")); }
    if o.team == crate::teams::Team::Civilian { return Err(String::from("spawnUnit: civilians are not units — civilians.ts stands them up from the level's civilians block (Game.civilians)")); }
    let chars = crate::seats::seat_count(g) + g.guards.len() as f64;   // (a man a seat, and everybody else who stands)
    let eight = crate::seats::budget_with_seats(g, MAX_CHARACTERS);   // (the eight is written for one player: a seat more, a man more)
    let cap = m::min(crate::guards::MAX_SKINNED, m::max(eight, o.budget.unwrap_or(0.0)));
    if chars + 1.0 > cap {
        if cap > eight {
            return Err(format!("spawnUnit: {} characters already stand (the player + {} guards/units) and this caller's budget is {} — despawn a unit or reset the guards first", chars, g.guards.len(), cap));
        }
        return Err(format!("spawnUnit: {} characters already stand (the player + {} guards/units) and the shadow-capsule budget is {} (scene/boxes.ts BoxWorld.addCharacterCapsules) — despawn a unit or reset the guards first", chars, g.guards.len(), eight));
    }
    let steady = g.engine.lights.lights.iter().filter(|&&l| g.tab.light[l].ttl < 0.0).count() as f64;
    if steady + 1.0 > crate::render_lights::MAX_LIGHTS {
        return Err(format!("spawnUnit: {} steady lights already exist and the light buffer holds {} (render/lights.ts) — his weapon light would never upload; despawn a unit first", steady, crate::render_lights::MAX_LIGHTS));
    }
    let mut used: Vec<f64> = Vec::new();
    for pl in crate::seats::men(g) { let c = g.pl[pl].ch; used.push(g.ch[c].id); }
    for &x in g.guards.iter() { let c = g.gd[x].ch; used.push(g.ch[c].id); }
    let mut id = UNIT_ID0;
    while used.contains(&id) && id <= UNIT_ID1 { id += 1.0; }
    if id > UNIT_ID1 { return Err(format!("spawnUnit: no character id left in {}‥{} (the owner byte) — despawn units first", UNIT_ID0, UNIT_ID1)); }
    g.m.guards.unit_serial += 1.0;
    let n = g.m.guards.unit_serial;
    let name: String = match &o.name { Some(s) => s.clone(), None => format!("{}-{}", o.team.as_str(), n) };
    let weapon = if o.carbine_rig.unwrap_or(false) {
        crate::weapons::WeaponId::Aks74u
    } else {
        match o.weapon {
            Some(w) => w,
            None => match o.team {
                crate::teams::Team::Ours => crate::weapons::WeaponId::Fn2000,
                crate::teams::Team::Hostile => crate::weapons::WeaponId::Aks74u,
                _ => crate::weapons::WeaponId::Guard9mm,
            },
        }
    };
    let route = g.tab.route.push(crate::scene_level::PatrolRoute {
        name: name.clone(),
        points: vec![[o.pos[0], 0.0, o.pos[2]]],
        wait: vec![0.0],
        weapon: Some(weapon),
        keys: Some(Vec::new()),
    });
    let (fi, f0, f1) = (g.tune.flash_intensity, g.tune.flash_inner, g.tune.flash_outer);
    let def = crate::rig_props::torch_light_def(g, &format!("unitlight{}", id), id, fi, f0, f1);
    let fl = crate::render_lights::light_set_add(g, def);
    let look = crate::game::GuardNewLook {
        tint2: Some(match o.tint2 { Some(t) => t, None => *crate::guards::team_tint2().of(o.team.as_str()).unwrap() }),
        yaw: Some(o.yaw.unwrap_or(0.0)),
        name: Some(name.clone()),
        team: Some(o.team),
    };
    let (men, rig) = (g.men, g.rig);
    let tint = match o.tint { Some(t) => t, None => *crate::guards::team_tint().of(o.team.as_str()).unwrap() };
    let gd = crate::game::guard_new(g, men, rig, id, route, -1, fl, tint, Some(&look));
    g.gd[gd].unit = Some(crate::game::UnitInfo { light: o.light != Some(false), pair: o.pair, seat: None });
    crate::duty::hold(g, gd, true, "guards");
    g.gd[gd].heard_up_to = g.event_seq;
    let c = g.gd[gd].ch;
    crate::character::character_update(g, c, 0.0);
    g.guards.push(gd);
    Ok(gd)
}

/// What of a unit lives outside his row: his weapon light.
pub(super) fn release_unit(g: &mut crate::game::Game, gd: GuardId) {
    let fl = g.gd[gd].flashlight;
    crate::render_lights::light_set_remove(g, fl);
}

/// Take a spawned man back out, and every hand on him. True if he was one.
pub fn despawn_unit(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].unit.is_none() { return false; }
    let Some(i) = g.guards.iter().position(|&x| x == gd) else { return false; };
    crate::seats::each_seat(g, &mut |g| {
        if g.pl[g.player].holding().map(|h| h.g) == Some(gd) { crate::game::game_free_held(g, crate::player::FreeHeldWhy::Cancel); }
        if g.pl[g.player].dragging() == Some(gd) { crate::game::game_toggle_drag(g, None); }
        if g.pl[g.player].takedown().map(|t| t.g) == Some(gd) { g.pl[g.player].takedown_over(); }
    });
    release_unit(g, gd);
    g.guards.remove(i);
    let at = i as f64;
    if g.spectate == at { g.spectate = -1.0; } else if g.spectate > at { g.spectate -= 1.0; }
    crate::seats::each_seat(g, &mut |g| if g.sets[g.seat].aim_guard == Some(gd) { g.sets[g.seat].aim_guard = None; });
    let mut k = 0;
    while k < g.guards.len() {
        let o = g.guards[k];
        k += 1;
        if g.gd[o].foe == Some(g.gd[gd].ch) {
            crate::mind::foe(g, o, None, "guards");
            crate::aim::aim_state_clear(&mut g.arms[g.gd[o].ch].aim);
        }
        if let Some(s) = g.gd[o].standoff { if g.tab.guards_standoff[s].pair == gd { g.gd[o].standoff = None; } }
        if g.gd[o].body_duty.as_ref().map_or(false, |b| b.body == gd) { g.gd[o].body_duty = None; }
        if g.gd[o].leader == Some(gd) { g.gd[o].leader = None; }
        if matches!(&g.gd[o].task, Some(crate::game::GuardTask::CheckOn(t)) if t.who == gd) { g.gd[o].task = None; }
    }
    let c = g.gd[gd].ch;
    crate::sim_men_store::man_leaves(g, c);
    true
}

/// The engagement on this game, done or not. A copy: re-ask for it to see it move.
pub(in crate::people) fn engagement_of(g: &crate::game::Game) -> Option<crate::guards::Engagement> {
    g.m.guards.engagements.clone()
}

/// Close the running engagement with `outcome`.
pub(super) fn end_engagement(g: &mut crate::game::Game) {
    if let Some(e) = g.m.guards.engagements.as_mut() { e.done = true; }
}

/// Is anybody of this list still in the fight?
fn living_any(g: &Game, xs: &[GuardId]) -> bool {
    for &x in xs {
        if !crate::condition::down(g, g.gd[x].ch) && g.guards.contains(&x) && !g.gd[x].fled { return true; }
    }
    false
}

/// Once a frame: book this frame's rounds and deaths into the engagement, end it when decided.
pub(super) fn tick_engagement(g: &mut crate::game::Game) {
    let (t0, timeout, ops, hos, alive) = match g.m.guards.engagements.as_ref() {
        None => return,
        Some(e) if e.done => return,
        Some(e) => (e.t0, e.timeout, e.operatives.clone(), e.hostiles.clone(), e.alive.keys()),
    };
    let mut i = 0;
    while i < g.events.len() {
        let (time, kind, level, who) = { let e = &g.events[i]; (e.time, e.kind, e.level, e.who) };
        i += 1;
        if time == g.time && (kind == crate::director::EventKind::Shot || kind == crate::director::EventKind::GuardShot) && level.unwrap_or(0.0) >= 1.0 {
            let t = if kind == crate::director::EventKind::Shot { crate::teams::Team::Ours } else { crate::teams::team_of_id(g, who.unwrap_or(-1.0)) };
            if let Some(e) = g.m.guards.engagements.as_mut() { rounds_add(&mut e.rounds, t); }
        }
    }
    let mut gone = false;
    for x in alive {
        if !g.guards.contains(&x) {
            gone = true;
            if let Some(e) = g.m.guards.engagements.as_mut() { e.alive.delete(x); }
            continue;
        }
        if crate::condition::down(g, g.gd[x].ch) {
            if let Some(e) = g.m.guards.engagements.as_mut() { e.alive.delete(x); }
        }
    }
    if gone { end_engagement(g); }
    else if !living_any(g, &hos) { end_engagement(g); }
    else if !living_any(g, &ops) { end_engagement(g); }
    else if g.time - t0 > timeout { end_engagement(g); }
}
