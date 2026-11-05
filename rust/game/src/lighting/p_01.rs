//! src/game/lighting.ts lines 1 to 234
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

// ---- seams: what the skeleton lacks (notes.md, Skeleton)

fn seam_lens_max_back(g: &mut Game, back: f64) {
    todo!("needs: LightingStats.max_back as f64, it is metres: LENS_STATS.maxBack = Math.max(LENS_STATS.maxBack, back) (lighting.ts:49)")
}
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool {
    todo!("needs: RtLight.enabled by LightId (lighting.ts:56, 142, 202)")
}
fn seam_light_set_enabled(g: &mut Game, l: LightId, on: bool) {
    todo!("needs: RtLight.enabled written by LightId (lighting.ts:202, 209)")
}
fn seam_light_pos(g: &mut Game, l: LightId) -> V3 {
    todo!("needs: RtLight.pos by LightId (lighting.ts:56)")
}
fn seam_light_set_pos(g: &mut Game, l: LightId, p: V3) {
    todo!("needs: RtLight.pos written by LightId (lighting.ts:56)")
}
fn seam_light_radius(g: &mut Game, l: LightId) -> f64 {
    todo!("needs: RtLight.radius by LightId (lighting.ts:56)")
}
fn seam_light_name(g: &mut Game, l: LightId) -> String {
    todo!("needs: RtLight.name by LightId (lighting.ts:202)")
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
    seam_lens_max_back(g, len - m::max(0.0, t));
    v3::mad(from, rd, m::max(0.0, t))
}

/// one burning carried light, from where it was placed   (lighting.ts lensRecheck `one`)
fn recheck_one(g: &mut Game, l: LightId) {
    let from = match g.m.lighting.lens_from.get(l) {
        Some(from) => *from,
        None => return,
    };
    if seam_light_enabled(g, l) {
        let pos = seam_light_pos(g, l);
        let radius = seam_light_radius(g, l);
        let p = lens_clear(g, from, pos, radius, None);
        seam_light_set_pos(g, l, p);
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
    let who = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    for pl in crate::seats::men(g, &who) {
        let l = g.pl[pl].light;
        recheck_one(g, l);
    }
}

/// The mains are the business of the side whose building it is.   (lighting.ts theirMains)
pub fn their_mains(g: &mut crate::game::Game, gd: GuardId) -> bool {
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

/// The mains' clocks and everything that goes with them, each tick. The paint loop is the picture's.   (lighting.ts updateTargets)
pub fn update_targets(g: &mut crate::game::Game, dt: f64) {
    let time = g.time;
    let mut near: Option<GuardId> = None;
    if crate::sim_lightshome::mains_active(g)
        && !crate::sim_lightshome::mains_permanent(g)
        && time - crate::sim_lightshome::mains_since(g) > 3.0
    {
        let mut i = 0;
        while i < g.guards.len() {
            let gd = g.guards[i];
            // no break: the LAST guard that qualifies wins (blitz/lights.md trap 5)
            if breaker_reset_near(g, gd) && their_mains(g, gd) && !crate::workthings::work_not_his(g, gd) {
                near = Some(gd);
            }
            i += 1;
        }
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
            g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.34);
            g.gd[gd].react_t = 1.0 + crate::math_dice::sim_random(g);
        }
        let d = v3::dist(g.ch[g.gd[gd].ch].pos, g.level.breaker.pos);
        if d < bd
            && g.gd[gd].state != crate::game::GuardState::Alert
            && g.gd[gd].held.is_none()
            && their_mains(g, gd)
            && !crate::workthings::work_not_his(g, gd)
            && !crate::workthings::work_stays_put(g, gd)
            && !crate::powerout::power_out_man(g, gd)
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
            g.gd[best].last_known = Some(goal);
            g.gd[best].awareness = m::max(g.gd[best].awareness, 0.42);
            if g.gd[best].state == crate::game::GuardState::Patrol {
                g.gd[best].state = crate::game::GuardState::Suspicious;
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
pub fn ocpable(g: &mut crate::game::Game) -> bool {
    if let Some(t) = g.aim_target {
        return !crate::carlamps::dead_lamp(g, t);
    }
    let gd = match g.aim_guard {
        Some(gd) => gd,
        None => return false,
    };
    if g.gd[gd].state == crate::game::GuardState::Dead {
        return false;
    }
    let l = g.gd[gd].flashlight;
    seam_light_enabled(g, l) && g.time >= g.gd[gd].light_dead_until
}

/// The OCP's press: on a man's light, on the breaker, or on a fixture.   (lighting.ts ocp)
pub fn ocp(g: &mut crate::game::Game, t: Option<FixtureId>) {
    let p = g.player;
    if !crate::weapons::ocp_ready(&mut g.pl[p].ocp) {
        crate::game::game_msg(g, "OCP recharging");
        crate::audio_iface::game_audio_like_play(g, "deny", None, Some(0.5), None);
        return;
    }
    let t = match t {
        Some(t) => t,
        None => {
            if let Some(gd) = g.aim_guard {
                if g.gd[gd].state != crate::game::GuardState::Dead {
                    if !ocpable(g) {
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
                        g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.32);
                        let pos = g.ch[c].pos;
                        let fwd = crate::character::character_forward(g, c);
                        g.gd[gd].last_known = Some(v3::add(pos, v3::scale(fwd, 2.0)));
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
pub fn is_daylight(g: &mut crate::game::Game) -> bool {
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
                active: true,
                permanent: true,
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
        let sky = crate::lighting::DayStateSky { zenith: g.engine.settings.sky_zenith, horizon: g.engine.settings.sky_horizon };
        let haze = g.engine.settings.haze_density;
        let exposure = g.engine.settings.exposure;
        let sun = crate::render_lights::light_from_def(&d.sun);
        let sun = crate::render_lights::light_set_add(&mut g.engine.lights, sun);
        let mut st = crate::lighting::DayState {
            hour,
            hour_sun_k,
            hour_sun_given,
            targets: JsMap::new(),
            lights_off: Vec::new(),
            sky,
            haze,
            exposure,
            sun,
            blackout,
        };
        crate::game::game_build_dir_reach(g, st.sun, None);
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
            let name = seam_light_name(g, l);
            if d.lights_off.contains(&name) && seam_light_enabled(g, l) {
                seam_light_set_enabled(g, l, false);
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
            seam_light_set_enabled(g, l, true);
            crate::sim_lightshome::light_home_feed(g, l);
        }
        g.engine.settings.sky_zenith = cur.sky.zenith;
        g.engine.settings.sky_horizon = cur.sky.horizon;
        g.engine.settings.haze_density = cur.haze;
        g.engine.settings.exposure = cur.exposure;
        crate::render_lights::light_set_remove(&mut g.engine.lights, cur.sun);
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

/// A wall switch's or the panel's flip of one fixture.   (lighting.ts toggleTarget)
pub fn toggle_target(g: &mut crate::game::Game, t: FixtureId) {
    light_in(g, ::sim::lights::IN_TOGGLE, Some(t), 0.0, 0.0, 0.0, 0.0);
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

/// A restart's lights: as a level just opened has them, nothing said or sounded. Whether anything was out.   (lighting.ts freshLights)
pub fn fresh_lights(g: &mut crate::game::Game) -> bool {
    let mut was = crate::sim_lightshome::mains_active(g) || g.time - crate::sim_lightshome::mains_restored_at(g) < 4.0;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        if crate::sim_lightshome::fixture_broken(g, t) || crate::sim_lightshome::fixture_disabled_until(g, t) > 0.0 {
            was = true;
        }
        light_in(g, ::sim::lights::IN_MEND, Some(t), 0.0, 0.0, 0.0, 0.0);
        light_in(g, ::sim::lights::IN_BLIND, Some(t), -1.0, 0.0, 0.0, 0.0);
        i += 1;
    }
    light_in(g, ::sim::lights::IN_FRESH, None, 0.0, 0.0, 0.0, 0.0);
    was
}
