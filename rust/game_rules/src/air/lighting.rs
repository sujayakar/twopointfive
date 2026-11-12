pub use ::game_types::lighting::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};

// ---- the bodies

/// Where a carried light is, given where its lens is drawn: pulled back along from→lens short of a wall or leaf.
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
    let h = match crate::scene_collision::raycast(g, from, rd, len + keep / LENS_SLANT) {
        Some(h) => h,
        None => return lens,
    };
    let t = h.t - keep / m::max(LENS_SLANT, m::abs(v3::dot(rd, h.n)));
    if t >= len - 1e-6 {
        return lens;
    }
    v3::mad(from, rd, m::max(0.0, t))
}

/// one burning carried light, from where it was placed
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

/// Again after the doors have moved: a leaf that swung onto the line this frame stops the light this frame.
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

/// The mains are the business of the side whose building it is.
fn their_mains(g: &crate::game::Game, gd: GuardId) -> bool {
    g.team(gd).theirs()
}

/// The re-arm's gate for one man: a trip he can undo, a few seconds old, and he alive, not held, within reach of the panel's front.
pub fn breaker_reset_near(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let bp = g.level.breaker.pos;
    if !(g.light_home.mains.active
        && !g.light_home.mains.permanent
        && g.time - g.light_home.mains.since > 3.0
        && !crate::condition::down(g, g.gd[gd].ch)
        && g.gd[gd].held.is_none())
    {
        return false;
    }
    let e = crate::workthings::breaker_errand(g);
    let p = g.pos_of(gd);
    if let Some(e) = e {
        return (v3::dist_xz(p, e.mid) < 1.3 || v3::dist_xz(p, e.goal) < 0.7)
            && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], e.at, None);
    }
    m::hyp2(p[0] - bp[0], p[2] - (bp[2] - 0.6)) < 1.3
}

/// The mains' clocks and everything that goes with them, each tick, then the rows onto the lamps. The boxes' paint is the picture's.
pub fn update_targets(g: &mut crate::game::Game, dt: f64) {
    let mut near: Option<GuardId> = None;
    // no gate outside the walk: breaker_reset_near holds it
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        // no break: the LAST guard that qualifies wins
        if breaker_reset_near(g, gd) && their_mains(g, gd) && !crate::workthings::work_not_his(g, gd) {
            near = Some(gd);
        }
        i += 1;
    }
    let fumble = g.level.breaker.fumble.unwrap_or(2.2);
    let hand_t = g.light_home.mains.fix_t + dt;
    let ev = crate::fixtures::lights_step(g, dt, near.is_some(), fumble);
    if ev.ran_out { mains_back(g); }
    if ev.hand {
        let c = g.gd[near.unwrap()].ch;
        crate::handwork::lever_hands(g, c, hand_t, fumble);
    }
    if ev.fixed {
        crate::game::game_say(g, near.unwrap(), "breaker's back on", Some(true), None);
        mains_back(g);
    }
    // ---- THE PICTURE, FROM THE STATE. Written though the picture is its reader: the fog's memory of a lamp's face reads the boxes' `emissive`.
    crate::fixtures::light_home_paint(g);
    let mut k = 0;
    while k < g.targets.len() {
        let t = g.targets[k];
        k += 1;
        let f = g.tab.fixture[t].factor;
        if g.tab.fixture[t].lens.is_some() { crate::carlamps::car_lamp_paint(g, t); }
        if g.tab.fixture[t].kind == crate::fixtures::FixtureKind::Breaker {
            let e: V3 = if g.tab.fixture[t].broken { [0.0, 0.0, 0.0] } else if g.light_home.mains.active { [5.0, 0.3, 0.1] } else { [0.3, 4.0, 0.6] };
            for bi in g.tab.fixture[t].boxes.clone() { let b = g.engine.world.statics[bi as usize]; g.tab.boxes.edit(b).emissive = e; }
            continue;
        }
        let base = g.tab.fixture[t].base_emissive;
        for bi in g.tab.fixture[t].boxes.clone() {
            let b = g.engine.world.statics[bi as usize];
            let mut lens = g.tab.boxes.edit(b);
            lens.emissive = [base[0] * f, base[1] * f, base[2] * f];
            lens.mark_gain = Some(f);
        }
    }
}

/// Trip the mains: everything fed from the breaker goes dark, the men get edgy, the nearest goes to check it.
pub fn set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::lighting::SetBlackoutCause) {
    let Some(fresh) = crate::fixtures::mains_trip(g, seconds) else { return };
    let bp = g.level.breaker.pos;
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::PowerDown, Some(bp), Some(1.0), None);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::PowerDown, None, Some(0.35), None);
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
        if crate::condition::down(g, g.gd[gd].ch) {
            continue;
        }
        if g.gd[gd].state == crate::game::GuardState::Patrol {
            crate::mind::raise(g, gd, 0.34, "lighting");
            g.gd[gd].react_t = 1.0 + crate::dice::draw_of(g, "lighting::set_blackout#1", gd);
        }
        let d = v3::dist(g.pos_of(gd), g.level.breaker.pos);
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
        if g.gd[best].errand.is_none() {
            let e = crate::workthings::breaker_errand(g);
            let bp = g.level.breaker.pos;
            let goal: V3 = match e {
                Some(e) => e.goal,
                None => [bp[0] - 0.4, 0.0, bp[2] - 0.9],
            };
            g.gd[best].errand = Some(crate::game::Errand::At(crate::game::ErrandAt {
                kind: crate::game::ErrandThing::Breaker,
                pos: goal,
            }));
            crate::mind::fix(g, best, Some(goal), "lighting");
            crate::mind::raise(g, best, 0.42, "lighting");
            if g.gd[best].state == crate::game::GuardState::Patrol {
                crate::mind::set_state(g, best, crate::game::GuardState::Suspicious, "lighting");
                g.gd[best].react_t = 1.2;
                g.gd[best].search_t = 0.0;
            }
            crate::game::game_say(g, best, "power's out — checking the breaker", Some(true), None);
            let p = g.pos_of(best);
            crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Radio, Some(p), Some(0.6), None);
        }
    }
    if fresh {
        crate::powerout::power_out(g);
    }
}

/// The mains back, if they were out.
pub fn end_blackout(g: &mut crate::game::Game) {
    if crate::fixtures::mains_restore(g) { mains_back(g); }
}

/// The mains have come back (the module has said so): what goes with it.
fn mains_back(g: &mut crate::game::Game) {
    let bp = g.level.breaker.pos;
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::PowerUp, Some(bp), Some(1.0), None);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::PowerUp, None, Some(0.3), None);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        if let Some(crate::game::Errand::At(a)) = &g.gd[gd].errand {
            if a.kind == crate::game::ErrandThing::Breaker {
                g.gd[gd].errand = None;
            }
        }
        i += 1;
    }
}

/// A light went out: an event for the men's eyes.
pub fn light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::lighting::LightEventCause>, sw_at: Option<V3>) {
    if crate::carlamps::dead_lamp(g, t) {
        return;
    }
    let cause = cause.unwrap_or(crate::lighting::LightEventCause::Shot);
    let pos = g.tab.fixture[t].pos;
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

/// Is there something under the reticle the OCP would act on right now?
pub fn ocpable(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    if let Some(t) = seat.aim_target {
        return !crate::carlamps::dead_lamp(g, t);
    }
    let gd = match seat.aim_guard {
        Some(gd) => gd,
        None => return false,
    };
    if crate::condition::down(g, g.gd[gd].ch) {
        return false;
    }
    let l = g.gd[gd].flashlight;
    g.tab.light[l].enabled && g.time >= g.gd[gd].light_dead_until
}

/// The OCP's press: on a man's light, on the breaker, or on a fixture.
pub fn ocp(g: &mut crate::game::Game, t: Option<FixtureId>) {
    let p = g.player;
    if !crate::weapons::ocp_ready(&g.pl[p].ocp) {
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Deny, None, Some(0.5), None);
        return;
    }
    let t = match t {
        Some(t) => t,
        None => {
            if let Some(gd) = g.sets[g.seat].aim_guard {
                if !crate::condition::down(g, g.gd[gd].ch) {
                    if !ocpable(g, &crate::seats::here(g)) {
                        return;
                    }
                    crate::weapons::ocp_use(&mut g.pl[p].ocp);
                    let muzzle = g.ch[g.pl[p].ch].muzzle;
                    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Ocp, Some(muzzle), Some(1.0), None);
                    g.gd[gd].light_dead_until = g.time + g.k.tune.ocp_duration * 0.7;
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
                    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LightOff, Some(pos), Some(0.6), None);
                    return;
                }
            }
            crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Deny, None, Some(0.5), None);
            return;
        }
    };
    let kind = g.tab.fixture[t].kind;
    let tpos = g.tab.fixture[t].pos;
    if kind == crate::fixtures::FixtureKind::Breaker {
        let c = g.pl[p].ch;
        let pos = g.ch[c].pos;
        let eye = g.ch[c].bone(crate::character::Bone::Head).unwrap_or(v3::add(pos, [0.0, 1.6, 0.0]));
        let own = crate::workthings::sam_panel_point(g, pos);
        if crate::scene_collision::segment_blocked(g, eye, v3::add(tpos, [0.0, 0.0, -0.18]), None, None, None)
            && !(match own {
                Some(own) => !crate::scene_collision::segment_blocked(g, eye, own, None, None, None),
                None => false,
            })
        {
            crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Deny, None, Some(0.5), None);
            return;
        }
        if g.light_home.mains.active {
            crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Deny, None, Some(0.5), None);
            return;
        }
        crate::weapons::ocp_use(&mut g.pl[p].ocp);
        let muzzle = g.ch[c].muzzle;
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Ocp, Some(muzzle), Some(1.0), None);
        let secs = g.k.tune.ocp_duration * 2.5;
        crate::game::game_set_blackout(g, secs, crate::lighting::SetBlackoutCause::Ocp);
        return;
    }
    let name = g.tab.fixture[t].name.clone();
    if g.tab.fixture[t].factor <= 0.0 {
        return;
    }
    crate::weapons::ocp_use(&mut g.pl[p].ocp);
    let muzzle = g.ch[g.pl[p].ch].muzzle;
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Ocp, Some(muzzle), Some(1.0), None);
    let until = g.time + g.k.tune.ocp_duration;
    g.tab.fixture[t].blind_until = until;
    if kind != crate::fixtures::FixtureKind::Device {
        crate::game::game_light_event(g, t, Some(crate::lighting::LightEventCause::Ocp), None);
    }
    let tg = crate::chronicle::FixtureNameTg { name, group: g.tab.fixture[t].group.clone() };
    let what = crate::chronicle::fixture_name(&tg);
    crate::chronicle::chron_ocp(g, &what, tpos);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LightOff, Some(tpos), Some(0.7), None);
}

/// Every fixture of a group on or off.
pub fn set_group(g: &mut crate::game::Game, group: &str, on: bool) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        if g.tab.fixture[t].group == group {
            g.tab.fixture[t].on = on;
        }
        i += 1;
    }
}

/// All fixtures repaired, OCP timers cleared, mains back.
pub fn repair_lights(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        g.tab.fixture[t].broken = false;
        g.tab.fixture[t].blind_until = -1.0;
        i += 1;
    }
    if g.light_home.mains.active {
        crate::game::game_end_blackout(g);
    }
}

