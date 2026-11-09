//! src/game/lighting.ts lines 1 to 231
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- what lights.rs answers: called, as blitz/lights.md's table says

/// EVERY EVENT FROM OUTSIDE (lightshome.ts LightHome.in): a kind, a fixture where it has one, its numbers. The kind's own answer.
fn light_in(g: &mut Game, kind: u32, t: Option<FixtureId>, a: f64, b: f64, c: f64, d: f64) -> u32 {
    let i = match t {
        Some(t) => g.tab.sim_lightshome_fixture[t].i as usize,
        None => 0,
    };
    ::sim::lights::input(g.sim, kind, i, a, b, c, d)
}

// ---- the bodies

/// Where a carried light is, given where its lens is drawn: pulled back along from→lens short of a wall or leaf.   (lighting.ts lensClear)
pub fn lens_clear(g: &mut crate::game::Game, from: V3, lens: V3, radius: f64, light: Option<LightId>) -> V3 {
    if let Some(l) = light {
        g.m.lighting.lens_from.set(l, from);
    }
    let d = v3::sub(lens, from);
    let len = v3::len(d);
    if len < 1e-3 {
        return lens;
    }
    let rd = v3::scale(d, 1.0 / len);
    let keep = m::max(0.0, radius) + g.k.lightwall.gap;
    g.stats.lighting.asked += 1;
    let h = match crate::scene_collision::raycast(g, from, rd, len + keep / LENS_SLANT) {
        Some(h) => h,
        None => return lens,
    };
    let t = h.t - keep / m::max(LENS_SLANT, m::abs(v3::dot(rd, h.n)));
    if t >= len - 1e-6 {
        return lens;
    }
    g.stats.lighting.pulled += 1;
    g.stats.lighting.max_back = m::max(g.stats.lighting.max_back, len - m::max(0.0, t));
    v3::mad(from, rd, m::max(0.0, t))
}

/// one burning carried light, from where it was placed   (lighting.ts lensRecheck `one`)
fn recheck_one(g: &mut Game, l: LightId) {
    let from = match g.m.lighting.lens_from.get(l) {
        Some(from) => *from,
        None => return,
    };
    if g.tab.light[l].enabled {
        let (pos, radius) = (g.tab.light[l].pos, g.tab.light[l].radius);
        g.tab.light[l].pos = lens_clear(g, from, pos, radius, None);
    }
}

/// Again after the doors have moved: a leaf that swung onto the line this frame stops the light this frame.   (lighting.ts lensRecheck)
pub fn lens_recheck(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.guards.len() {
        let l = g.gd[g.guards[i]].flashlight;
        recheck_one(g, l);
        i += 1;
    }
    for pl in crate::seats::men(g) {
        let l = g.pl[pl].light;
        recheck_one(g, l);
    }
}

/// The mains are the business of the side whose building it is.   (lighting.ts theirMains)
pub fn their_mains(g: &crate::game::Game, gd: GuardId) -> bool {
    let team = g.gd[gd].team;
    crate::teams::is_hostile(g, team, crate::teams::Team::Player)
}

/// The re-arm's gate for one man: a trip he can undo, a few seconds old, and he alive, not held, within reach of the panel's front.   (lighting.ts breakerResetNear)
pub fn breaker_reset_near(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let bp = g.level.breaker.pos;
    if !(crate::sim_lightshome::mains_active(g)
        && !crate::sim_lightshome::mains_permanent(g)
        && g.time - crate::sim_lightshome::mains_since(g) > 3.0
        && g.gd[gd].state != crate::game::GuardState::Dead
        && g.gd[gd].held.is_none())
    {
        return false;
    }
    let e = crate::workthings::breaker_errand(g);
    let p = g.ch[g.gd[gd].ch].pos;
    if let Some(e) = e {
        return (m::hyp2(p[0] - e.mid[0], p[2] - e.mid[2]) < 1.3 || m::hyp2(p[0] - e.goal[0], p[2] - e.goal[2]) < 0.7)
            && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], e.at, None);
    }
    m::hyp2(p[0] - bp[0], p[2] - (bp[2] - 0.6)) < 1.3
}

/// The mains' clocks and everything that goes with them, each tick, then the rows onto the lamps. The boxes' paint is the picture's.   (lighting.ts updateTargets)
pub fn update_targets(g: &mut crate::game::Game, dt: f64) {
    let time = g.time;
    let mut near: Option<GuardId> = None;
    // the lights' mend (blitz/lights.md): no gate outside the walk, breaker_reset_near holds it
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        // no break: the LAST guard that qualifies wins (blitz/lights.md trap 5)
        if breaker_reset_near(g, gd) && their_mains(g, gd) && !crate::workthings::work_not_his(g, gd) {
            near = Some(gd);
        }
        i += 1;
    }
    let fumble = g.level.breaker.fumble.unwrap_or(2.2);
    let hand_t = crate::sim_lightshome::mains_fix_t(g) + dt;
    let es = g.engine.settings.emissive_scale;
    let ev = ::sim::lights::step(g.sim, time, dt, near.is_some(), fumble, es);
    if ev & ::sim::lights::EV_RAN_OUT != 0 {
        mains_back(g);
    }
    if ev & ::sim::lights::EV_HAND != 0 {
        let c = g.gd[near.unwrap()].ch;
        crate::handwork::lever_hands(g, c, hand_t, fumble);
    }
    if ev & ::sim::lights::EV_FIXED != 0 {
        crate::game::game_say(g, near.unwrap(), "breaker's back on", Some(true), None, None);
        mains_back(g);
    }
    // ---- THE PICTURE, FROM THE STATE (lighting.ts:85-92). Kept though the picture is its reader: set_daylight reads a row lamp's `enabled` (lighting.ts:199),
    // and the fog's memory of a lamp's face reads the boxes' `emissive` (fogofwar.ts:829), so both are written as the TypeScript writes them.
    crate::sim_lightshome::light_home_paint(g);
    let mut k = 0;
    while k < g.targets.len() {
        let t = g.targets[k];
        k += 1;
        let f = crate::sim_lightshome::fixture_factor(g, t);
        if g.tab.sim_lightshome_fixture[t].lens.is_some() { crate::carlamps::car_lamp_paint(g, t); }
        if g.tab.sim_lightshome_fixture[t].kind == crate::sim_lightshome::FixtureKind::Breaker {
            let e: V3 = if crate::sim_lightshome::fixture_broken(g, t) { [0.0, 0.0, 0.0] } else if crate::sim_lightshome::mains_active(g) { [5.0, 0.3, 0.1] } else { [0.3, 4.0, 0.6] };
            for bi in g.tab.sim_lightshome_fixture[t].boxes.clone() { let b = g.engine.world.statics[bi as usize]; g.tab.boxes[b].emissive = e; }
            continue;
        }
        let base = g.tab.sim_lightshome_fixture[t].base_emissive;
        for bi in g.tab.sim_lightshome_fixture[t].boxes.clone() {
            let b = g.engine.world.statics[bi as usize];
            g.tab.boxes[b].emissive = [base[0] * f, base[1] * f, base[2] * f];
            g.tab.boxes[b].mark_gain = Some(f);
        }
    }
}

/// Trip the mains: everything fed from the breaker goes dark, the men get edgy, the nearest goes to check it.   (lighting.ts setBlackout)
pub fn set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::lighting::SetBlackoutCause) {
    let took = light_in(g, ::sim::lights::IN_TRIP, None, g.time, seconds, 0.0, 0.0);
    if took == 0 {
        return;
    }
    let fresh = took == 3;
    let bp = g.level.breaker.pos;
    crate::audio_iface::game_audio_like_play(g, "powerDown", Some(bp), Some(1.0), None);
    crate::audio_iface::game_audio_like_play(g, "powerDown", None, Some(0.35), None);
    let said = match cause {
        crate::lighting::SetBlackoutCause::Shot => "breaker destroyed — power gone for good",
        crate::lighting::SetBlackoutCause::Hand => "breaker thrown — emergency lighting until someone resets it",
        _ => "power is out — emergency lighting",
    };
    crate::game::game_msg(g, said);
    if cause != crate::lighting::SetBlackoutCause::Debug && crate::mission::mission_live(g) {
        g.mission.stats.blackout = true;
        crate::chronicle::chron_blackout(g, cause.as_str());
    }
    let mut best: Option<GuardId> = None;
    let mut bd = 1e9;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead {
            continue;
        }
        if g.gd[gd].state == crate::game::GuardState::Patrol {
            crate::mind::raise(g, gd, 0.34, "lighting");
            g.gd[gd].react_t = 1.0 + crate::dice::draw_of(g, "lighting::set_blackout#1", gd);
        }
        let d = v3::dist(g.ch[g.gd[gd].ch].pos, g.level.breaker.pos);
        if d < bd
            && g.gd[gd].state != crate::game::GuardState::Alert
            && g.gd[gd].held.is_none()
            && their_mains(g, gd)
            && !crate::workthings::work_not_his(g, gd)
            && !crate::workthings::work_stays_put(g, gd)
            && !crate::powerout::power_out_rules(g, gd)
        {
            bd = d;
            best = Some(gd);
        }
    }
    if let Some(best) = best {
        if g.gd[best].task.is_none() {
            let e = crate::workthings::breaker_errand(g);
            let bp = g.level.breaker.pos;
            let goal: V3 = match e {
                Some(e) => e.goal,
                None => [bp[0] - 0.4, 0.0, bp[2] - 0.9],
            };
            g.gd[best].task = Some(crate::game::GuardTask::Arm0(crate::game::GuardTaskArm0 {
                kind: crate::game::GuardTaskArm0Kind::Breaker,
                pos: goal,
            }));
            crate::mind::fix(g, best, Some(goal), "lighting");
            crate::mind::raise(g, best, 0.42, "lighting");
            if g.gd[best].state == crate::game::GuardState::Patrol {
                crate::mind::set_state(g, best, crate::game::GuardState::Suspicious, "lighting");
                g.gd[best].react_t = 1.2;
                g.gd[best].search_t = 0.0;
            }
            crate::game::game_say(g, best, "power's out — checking the breaker", Some(true), None, None);
            let p = g.ch[g.gd[best].ch].pos;
            crate::audio_iface::game_audio_like_play(g, "radio", Some(p), Some(0.6), None);
        }
    }
    if fresh {
        crate::powerout::power_out(g);
    }
}

/// The mains back, if they were out.   (lighting.ts endBlackout)
pub fn end_blackout(g: &mut crate::game::Game) {
    if light_in(g, ::sim::lights::IN_BACK, None, g.time, 0.0, 0.0, 0.0) != 0 {
        mains_back(g);
    }
}

/// The mains have come back (the module has said so): what goes with it.   (lighting.ts mainsBack)
pub fn mains_back(g: &mut crate::game::Game) {
    let bp = g.level.breaker.pos;
    crate::audio_iface::game_audio_like_play(g, "powerUp", Some(bp), Some(1.0), None);
    crate::audio_iface::game_audio_like_play(g, "powerUp", None, Some(0.3), None);
    crate::game::game_msg(g, "power is back on");
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        if let Some(crate::game::GuardTask::Arm0(a)) = &g.gd[gd].task {
            if a.kind == crate::game::GuardTaskArm0Kind::Breaker {
                g.gd[gd].task = None;
            }
        }
        i += 1;
    }
}

/// A light went out: an event for the men's eyes.   (lighting.ts lightEvent)
pub fn light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::lighting::LightEventCause>, sw_at: Option<V3>) {
    if crate::carlamps::dead_lamp(g, t) {
        return;
    }
    let cause = match cause.unwrap_or(crate::lighting::LightEventCause::Shot) {
        crate::lighting::LightEventCause::Shot => crate::game::GameEventCause::Shot,
        crate::lighting::LightEventCause::Ocp => crate::game::GameEventCause::Ocp,
    };
    let pos = g.tab.sim_lightshome_fixture[t].pos;
    let time = g.time;
    let serial = g.serial();
    g.events.push(crate::game::GameEvent {
        announced: None,
        kind: crate::director::EventKind::LightOut,
        pos: [pos[0], 0.0, pos[2]],
        time,
        loud: false,
        level: None,
        id: None,
        who: None,
        weapon: None,
        from: Some(pos),
        to: None,
        cause: Some(cause),
        sw_at,
        blind: None,
        lure: None,
        frag: None,
        charge: None,
        ears: None,
        serial,
    });
}

/// Is there something under the reticle the OCP would act on right now?   (lighting.ts ocpable)
pub fn ocpable(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    if let Some(t) = seat.aim_target {
        return !crate::carlamps::dead_lamp(g, t);
    }
    let gd = match seat.aim_guard {
        Some(gd) => gd,
        None => return false,
    };
    if g.gd[gd].state == crate::game::GuardState::Dead {
        return false;
    }
    let l = g.gd[gd].flashlight;
    g.tab.light[l].enabled && g.time >= g.gd[gd].light_dead_until
}

/// The OCP's press: on a man's light, on the breaker, or on a fixture.   (lighting.ts ocp)
pub fn ocp(g: &mut crate::game::Game, t: Option<FixtureId>) {
    let p = g.player;
    if !crate::weapons::ocp_ready(&g.pl[p].ocp) {
        crate::game::game_msg(g, "OCP recharging");
        crate::audio_iface::game_audio_like_play(g, "deny", None, Some(0.5), None);
        return;
    }
    let t = match t {
        Some(t) => t,
        None => {
            if let Some(gd) = g.aim_guard {
                if g.gd[gd].state != crate::game::GuardState::Dead {
                    if !ocpable(g, &crate::seats::here(g)) {
                        return;
                    }
                    crate::weapons::ocp_use(&mut g.pl[p].ocp);
                    let muzzle = g.ch[g.pl[p].ch].muzzle;
                    crate::audio_iface::game_audio_like_play(g, "ocp", Some(muzzle), Some(1.0), None);
                    g.gd[gd].light_dead_until = g.time + g.tune.ocp_duration * 0.7;
                    crate::game::game_msg(g, "OCP → guard's flashlight fried");
                    let c = g.gd[gd].ch;
                    let what = alloc::format!("{}'s torch", g.gd[gd].callsign);
                    let pos = g.ch[c].pos;
                    crate::chronicle::chron_ocp(g, &what, pos);
                    if g.gd[gd].state == crate::game::GuardState::Patrol {
                        crate::mind::raise(g, gd, 0.32, "lighting");
                        let pos = g.ch[c].pos;
                        let fwd = crate::character::character_forward(g, c);
                        crate::mind::fix(g, gd, Some(v3::add(pos, v3::scale(fwd, 2.0))), "lighting");
                    }
                    let pos = g.ch[c].pos;
                    crate::audio_iface::game_audio_like_play(g, "lightOff", Some(pos), Some(0.6), None);
                    return;
                }
            }
            crate::audio_iface::game_audio_like_play(g, "deny", None, Some(0.5), None);
            return;
        }
    };
    let kind = g.tab.sim_lightshome_fixture[t].kind;
    let tpos = g.tab.sim_lightshome_fixture[t].pos;
    if kind == crate::sim_lightshome::FixtureKind::Breaker {
        let c = g.pl[p].ch;
        let pos = g.ch[c].pos;
        let eye = match g.ch[c].bones.get(crate::character::Bone::Head) {
            Some(head) => *head,
            None => v3::add(pos, [0.0, 1.6, 0.0]),
        };
        let own = crate::workthings::sam_panel_point(g, pos);
        if crate::scene_collision::segment_blocked(g, eye, v3::add(tpos, [0.0, 0.0, -0.18]), None, None, None)
            && !(match own {
                Some(own) => !crate::scene_collision::segment_blocked(g, eye, own, None, None, None),
                None => false,
            })
        {
            crate::game::game_msg(g, "the OCP can't see the breaker from here");
            crate::audio_iface::game_audio_like_play(g, "deny", None, Some(0.5), None);
            return;
        }
        if crate::sim_lightshome::mains_active(g) {
            crate::game::game_msg(g, "power is already off");
            crate::audio_iface::game_audio_like_play(g, "deny", None, Some(0.5), None);
            return;
        }
        crate::weapons::ocp_use(&mut g.pl[p].ocp);
        let muzzle = g.ch[c].muzzle;
        crate::audio_iface::game_audio_like_play(g, "ocp", Some(muzzle), Some(1.0), None);
        let secs = g.tune.ocp_duration * 2.5;
        crate::game::game_set_blackout(g, secs, crate::game::GameSetBlackoutCause::Ocp);
        return;
    }
    let name = g.tab.sim_lightshome_fixture[t].name.clone();
    if crate::sim_lightshome::fixture_factor(g, t) <= 0.0 {
        crate::game::game_msg(g, &alloc::format!("OCP: {} already off", name));
        return;
    }
    crate::weapons::ocp_use(&mut g.pl[p].ocp);
    let muzzle = g.ch[g.pl[p].ch].muzzle;
    crate::audio_iface::game_audio_like_play(g, "ocp", Some(muzzle), Some(1.0), None);
    let until = g.time + g.tune.ocp_duration;
    light_in(g, ::sim::lights::IN_BLIND, Some(t), until, 0.0, 0.0, 0.0);
    crate::game::game_msg(g, &alloc::format!("OCP → {} disabled", name));
    if kind != crate::sim_lightshome::FixtureKind::Device {
        crate::game::game_light_event(g, t, Some(crate::game::GameLightEventCause::Ocp), None);
    }
    let tg = crate::chronicle::FixtureNameTg { name, group: g.tab.sim_lightshome_fixture[t].group.clone() };
    let what = crate::chronicle::fixture_name(&tg);
    crate::chronicle::chron_ocp(g, &what, tpos);
    crate::audio_iface::game_audio_like_play(g, "lightOff", Some(tpos), Some(0.7), None);
}

/// Every fixture of a group on or off.   (lighting.ts setGroup)
pub fn set_group(g: &mut crate::game::Game, group: &str, on: bool) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        if g.tab.sim_lightshome_fixture[t].group == group {
            light_in(g, ::sim::lights::IN_ON, Some(t), if on { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0);
        }
        i += 1;
    }
}

/// Is the daytime variant in (setDaylight)?   (lighting.ts isDaylight)
pub fn is_daylight(g: &crate::game::Game) -> bool {
    g.m.lighting.day_states.is_some()
}

/// Swap the level's daytime variant in (on) or back out (off). Off restores exactly what On recorded.   (lighting.ts setDaylight)
pub fn set_daylight(g: &mut crate::game::Game, on: bool) {
    if on {
        if g.m.lighting.day_states.is_some() {
            return;
        }
        let d = match g.level.daylight.clone() {
            Some(d) => d,
            None => return,
        };
        let hour = crate::hours::hour_of(g);
        let hour_sun_k = crate::hours::sun_strength(g);
        let hour_sun_given = crate::hours::sun_given(g);
        if hour != crate::hours::HourId::Night {
            crate::hours::apply_hour(g, crate::hours::HourId::Night);
        }
        let blackout = if crate::sim_lightshome::mains_active(g) && crate::sim_lightshome::mains_permanent(g) {
            Some(crate::lighting::DayStateBlackout {
                until: crate::sim_lightshome::mains_until(g),
                since: crate::sim_lightshome::mains_since(g),
            })
        } else {
            None
        };
        if crate::sim_lightshome::mains_active(g) {
            crate::game::game_end_blackout(g);
            // (`bo` is a live view in the TypeScript: until and since are read AFTER the mains came back)
            let until = crate::sim_lightshome::mains_until(g);
            let since = crate::sim_lightshome::mains_since(g);
            light_in(g, ::sim::lights::IN_MAINS, None, 0.0, until, since, -100.0);
            let mut i = 0;
            while i < g.targets.len() {
                let t = g.targets[i];
                light_in(g, ::sim::lights::IN_BLIND, Some(t), -1.0, 0.0, 0.0, 0.0);
                i += 1;
            }
        }
        let sun = crate::render_lights::light_from_def(g, &d.sun);
        let sun = crate::render_lights::light_set_add(g, sun);
        let mut st = crate::lighting::DayState {
            hour,
            hour_sun_k,
            hour_sun_given,
            targets: JsMap::new(),
            lights_off: Vec::new(),
            sun,
            blackout,
        };
        let mut i = 0;
        while i < g.targets.len() {
            let t = g.targets[i];
            let was = crate::sim_lightshome::fixture_on(g, t);
            st.targets.set(t, was);
            i += 1;
        }
        for grp in d.groups_on.iter() {
            set_group(g, grp, true);
        }
        for grp in d.groups_off.iter() {
            set_group(g, grp, false);
        }
        let mut i = 0;
        while i < g.engine.lights.lights.len() {
            let l = g.engine.lights.lights[i];
            if d.lights_off.contains(&g.tab.light[l].name) && g.tab.light[l].enabled {
                g.tab.light[l].enabled = false;
                crate::sim_lightshome::light_home_feed(g, l);
                st.lights_off.push(l);
            }
            i += 1;
        }
        g.engine.settings.sky_zenith = d.sky.zenith;
        g.engine.settings.sky_horizon = d.sky.horizon;
        g.engine.settings.haze_density = d.haze;
        g.engine.settings.exposure = d.exposure;
        g.m.lighting.day_states = Some(st);
        crate::host_api::host_api_invalidate_world_cache(g);
    } else {
        let cur = match g.m.lighting.day_states.clone() {
            Some(cur) => cur,
            None => return,
        };
        for (t, was) in cur.targets.iter() {
            light_in(g, ::sim::lights::IN_ON, Some(t), if *was { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0);
        }
        for &l in cur.lights_off.iter() {
            g.tab.light[l].enabled = true;
            crate::sim_lightshome::light_home_feed(g, l);
        }
        let was = g.level.daylight.as_ref().map_or(g.k.env.exposure, |d| d.exposure);
        crate::hours::night_picture(g, was);
        crate::render_lights::light_set_remove(g, cur.sun);
        if let Some(b) = cur.blackout {
            if !crate::sim_lightshome::mains_active(g) {
                let restored = crate::sim_lightshome::mains_restored_at(g);
                let word = (::sim::lights::M_ACTIVE | ::sim::lights::M_PERMANENT) as f64;
                light_in(g, ::sim::lights::IN_MAINS, None, word, b.until, b.since, restored);
                light_in(g, ::sim::lights::IN_FIX, None, 0.0, 0.0, 0.0, 0.0);
            }
        }
        g.m.lighting.day_states = None;
        if cur.hour != crate::hours::HourId::Night {
            crate::hours::apply_hour(g, cur.hour);
            crate::hours::set_sun_strength(g, cur.hour_sun_k, Some(cur.hour_sun_given));
        }
        crate::host_api::host_api_invalidate_world_cache(g);
    }
}

/// All fixtures repaired, OCP timers cleared, mains back.   (lighting.ts repairLights)
pub fn repair_lights(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        light_in(g, ::sim::lights::IN_MEND, Some(t), 0.0, 0.0, 0.0, 0.0);
        light_in(g, ::sim::lights::IN_BLIND, Some(t), -1.0, 0.0, 0.0, 0.0);
        i += 1;
    }
    if crate::sim_lightshome::mains_active(g) {
        crate::game::game_end_blackout(g);
    }
}

