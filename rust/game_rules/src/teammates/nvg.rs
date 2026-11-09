pub use ::game_types::nvg::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, vec::Vec};

// ---- the team's state

/// A team nothing has happened to.
fn fresh() -> crate::nvg::Team {
    Team {
        t: -100.0,
        mode: Mode::Off,
        why: Why::NoRule,
        men: JsMap::new(),
        lead_lit_t: -m::INF,
        hunt_t: -m::INF,
        alarm_t: -m::INF,
        called_t: -m::INF,
        call_due: false,
    }
}

/// One row of the QA log, the last 200.
fn note(g: &crate::game::Game, gd: Option<GuardId>, what: &str) {
}

/// Is the team's state this frame's (or the last few: the rig is drawn after the tick, a paused frame draws again).
fn is_live(g: &Game) -> bool {
    match &g.m.nvg.teams {
        Some(t) => t.mode != Mode::Off && g.time - t.t >= 0.0 && g.time - t.t < 0.25,
        None => false,
    }
}

/// `live(g)?.men.get(gd)` with no copy of the team: the readers below are asked once for each pair of men a frame.
fn live_man(g: &Game, gd: GuardId) -> Option<Man> {
    if !is_live(g) {
        return None;
    }
    g.m.nvg.teams.as_ref().and_then(|t| t.men.get(gd).copied())
}

// ---- the tube

/// post.wgsl's intensifier curve at an exposure and a gain: the signal's `k` and the signal of irradiance `e`.
fn tube_curve_of(exposure: f64, gain: f64, albedo: f64, e: f64) -> (f64, f64) {
    let k = albedo / m::PI * exposure;
    let sig = v3::clamp(m::log(1.0 + m::max(e, 0.0) * k * gain) / m::log(1.0 + gain), 0.0, 1.0);
    (k, sig)
}

/// The curve at the game's own exposure and tube gain.
fn tube_curve(g: &Game, e: f64) -> (f64, f64) {
    let exposure = crate::hours::exposure_of(g);
    let gain = crate::env::tube_gain(g);
    tube_curve_of(exposure, gain, g.k.nvg.albedo, e)
}

/// The meter's 0..1 for a man under irradiance `e` seen through the goggles.
pub fn tube_vis(g: &crate::game::Game, e: f64) -> f64 {
    let (k, sig) = tube_curve(g, e);
    if sig > 0.0 {
        let seen = crate::game::game_visibility_of(g, sig / k);
        seen * sig / (sig + g.k.nvg.noise)
    } else {
        0.0
    }
}

/// The tube's picture of a matte floor under irradiance `e`, 0..1 of its range.
pub(super) fn tube_sig(g: &crate::game::Game, e: f64) -> f64 {
    tube_curve(g, e).1
}

/// A living operative with a unit whose card left him his goggles.
fn is_mate(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
        && g.gd[gd].unit.is_some()
        && g.gd[gd].state != crate::game::GuardState::Dead
        && crate::kit::kit_goggles(g, gd)
}

/// When the other side last gave the team reason to think it knows.
fn loud_t(g: &crate::game::Game) -> f64 {
    let t = g.m.nvg.teams.as_ref().unwrap();   // (only the tick and nvgState ask, each with the team in hand)
    let mut last = m::max_n(&[t.hunt_t, t.alarm_t, g.fight.hot_t]);
    for &o in &g.fight.ops {
        let hot_t = g.tab.roomfight_op[o].hot_t;
        if hot_t > last {
            last = hot_t;
        }
    }
    if last > -100.0 { last } else { -m::INF }   // (roomfight's 'never' is -100)
}

/// The light of the place at a point, nothing carried, as it stands: the tube is the eye, and no eye's habit is in it.
pub fn nvg_place_light(g: &mut crate::game::Game, p: V3) -> f64 {
    crate::guards::light_at(g, p, Some(&mut |_: &mut Game, _: f64| true), None, None).0
}

/// One point of his place: is it dark to the bare eye, and (if `tube`) too dark for the tube.
fn read_at(g: &mut Game, x: f64, z: f64, tube: bool, lamp: bool, dim: &mut bool, black: &mut bool) {
    let e = nvg_place_light(g, [x, 1.2, z]);
    let bare = e * crate::guards::eyes_in(g, [x, 1.2, z]);
    if crate::game::game_visibility_of(g, bare) < g.k.nvg.dark_at {
        *dim = true;
    }
    if tube && !lamp && tube_vis(g, e) < g.k.nvg.black_at {
        *black = true;
    }
}

/// His place, read twice a second: dark to the bare eye, too dark for the tube.
fn read_place(g: &mut crate::game::Game, gd: GuardId, m: &mut crate::nvg::Man) {
    if g.time - m.place_t >= 0.0 && g.time - m.place_t < 0.5 {
        return;
    }
    let c = g.gd[gd].ch;
    let (pos, aim_yaw) = (g.ch[c].pos, g.ch[c].aim_yaw);
    let eye: V3 = [pos[0], 1.5, pos[2]];
    let (fx, fz) = (m::sin(aim_yaw), m::cos(aim_yaw));
    let (mut dim, mut black) = (false, false);
    read_at(g, pos[0], pos[2], false, true, &mut dim, &mut black);
    for r in look_m() {
        let p: V3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r];
        if crate::scene_collision::segment_blocked(g, eye, p, None, None, None) {
            break;
        }
        read_at(g, p[0], p[2], r <= TUBE_M, true, &mut dim, &mut black);
    }
    if let Some(a) = crate::seats::order_of(g, gd) {
        let (post, arrived) = (g.tab.orders_assignment[a].post, g.tab.orders_assignment[a].arrived);
        if let Some(post) = post {
            if !arrived && v3::dist_xz(pos, post) < g.k.lights.lit_mark_m {
                read_at(g, post[0], post[2], true, true, &mut dim, &mut black);
            }
        }
    }
    if let Some(o) = crate::roomfight::room_fight_op_of(g, gd) {
        if let Some(tgt) = g.tab.roomfight_op[o].tgt {
            if g.gd[tgt].state != crate::game::GuardState::Dead {
                let at = g.ch[g.gd[tgt].ch].pos;
                read_at(g, at[0], at[2], false, true, &mut dim, &mut black);
            }
        }
        if let Some((by_t, by_at)) = g.tab.roomfight_op[o].step_by.as_ref().map(|by| (by.t, by.at)) {
            if g.time - by_t < g.k.lights.step_secs {
                read_at(g, by_at[0], by_at[2], true, true, &mut dim, &mut black);
            }
        }
        if let Some(pt) = g.tab.roomfight_op[o].turn.as_ref().and_then(|turn| turn.pt) {
            read_at(g, pt[0], pt[2], true, true, &mut dim, &mut black);
        }
    }
    m.place_t = g.time;
    m.dim = dim;
    m.run = if black { m::max(m.run, 0.0) + 1.0 } else { m::min(m.run, 0.0) - 1.0 };
    if m.run >= g.k.nvg.black_reads {
        m.black = true;
    } else if -m.run >= g.k.nvg.clear_reads {
        m.black = false;
    }
}

/// Is the light switched on on the gun in this player's hands.
fn lead_lit(g: &Game, pl: PlayerId) -> bool {
    let (pistol, rifle) = (g.pl[pl].pistol, g.pl[pl].rifle);
    (crate::game::player_pistol_up(g, pl) && g.fa[pistol].light_on) || (crate::game::player_rifle_up(g, pl) && g.fa[rifle].light_on)
}

/// Once a frame from Game.update: is the team dark, and each man's goggles.
pub fn nvg_tick(g: &mut crate::game::Game) {
    if !crate::roomfight::fight_live(g) {
        if let Some(t) = g.m.nvg.teams.as_mut() {
            if t.mode != Mode::Off {
                t.mode = Mode::Off;
                t.men.clear();
            }
        }
        return;
    }
    let keep = match &g.m.nvg.teams {
        None => false,
        Some(t) => {
            let known = t.men.is_empty() || t.men.iter().any(|(gd, _)| g.guards.contains(&gd));
            t.mode != Mode::Off && !(t.t > g.time) && known
        }
    };
    if !keep {
        g.m.nvg.teams = Some(fresh());   // (a fresh staging, or the clock put back by a load: nothing remembered)
    }
    let mut squad: Vec<GuardId> = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if is_mate(g, gd) {
            squad.push(gd);
        }
    }
    let team = g.m.nvg.teams.as_mut().unwrap();
    for gd in team.men.keys() {
        if !squad.contains(&gd) {
            team.men.delete(gd);
        }
    }
    if squad.is_empty() {
        team.mode = Mode::Off;
        return;
    }
    // ---- what the team knows of its own secret
    let mut up = crate::seats::men(g);
    up.retain(|&pl| !g.pl[pl].down);
    let mut lit = false;
    for &pl in &up {
        if lead_lit(g, pl) {
            lit = true;
            break;
        }
    }
    if lit {
        g.m.nvg.teams.as_mut().unwrap().lead_lit_t = g.time;
    }
    if crate::guards::escalation_of(g) >= 1.0 {
        g.m.nvg.teams.as_mut().unwrap().alarm_t = g.time;
    }
    let (lead_lit_t, was) = {
        let team = g.m.nvg.teams.as_ref().unwrap();
        (team.lead_lit_t, team.mode)
    };
    let lead: Option<Mode> = if up.is_empty() {
        None
    } else if g.time - lead_lit_t < g.k.nvg.lead_tail {
        Some(Mode::Free)
    } else if up.iter().any(|&pl| g.pl[pl].nv) {
        Some(Mode::Dark)
    } else {
        None
    };
    let quiet = g.time - loud_t(g) >= g.k.nvg.quiet_secs;
    let mode = lead.unwrap_or(if quiet { Mode::Dark } else { Mode::Free });
    let why = match lead {
        Some(Mode::Free) => Why::YourLight,
        Some(_) => Why::YourGoggles,
        None => if quiet { Why::NotCompromised } else { Why::Compromised },
    };
    let now = g.time;
    let team = g.m.nvg.teams.as_mut().unwrap();
    team.t = now;
    team.mode = mode;
    team.why = why;
    if mode != was {
        let word = if mode == Mode::Dark { "dark" } else { "lights free" };
        note(g, None, &format!("team {} ({})", word, why.as_str()));
        g.m.nvg.teams.as_mut().unwrap().call_due = mode == Mode::Dark && was == Mode::Free;
    }
    // ---- each man
    let mut caller: Option<GuardId> = None;
    for &gd in &squad {
        let now = g.time;
        let team = g.m.nvg.teams.as_mut().unwrap();
        if !team.men.has(gd) {
            team.men.set(gd, Man { down: false, dark: false, agree_t: now - 10.0, place_t: -m::INF, dim: false, black: false, run: 0.0 });
        }
        let mut row = *team.men.get(gd).unwrap();
        read_place(g, gd, &mut row);   // (the row lives in g: lent by copy, written back at the foot of this walk. It writes only at its own foot and nothing it calls writes a row)
        let ruled = crate::guards::light_ruled(g, gd);
        let dark = mode == Mode::Dark && ruled && !(lead.is_none() && row.black);
        if dark != row.dark {
            let what = if dark {
                "dark"
            } else if mode == Mode::Dark && ruled {
                "lights: too dark for the tube"
            } else {
                "light his own again"
            };
            note(g, Some(gd), what);
        }
        row.dark = dark;
        if dark && caller.is_none() {
            caller = Some(gd);
        }
        let want = row.dim && (dark || {
            let l = g.gd[gd].flashlight;   // (his light as last frame posed it; a dark man's is going off this frame)
            !g.tab.light[l].enabled
        });
        if want == row.down {
            row.agree_t = g.time;
        } else if g.time - row.agree_t >= (if want { g.k.nvg.down_secs } else { g.k.nvg.up_secs }) {
            row.down = want;
            row.agree_t = g.time;
            note(g, Some(gd), if want { "goggles down" } else { "goggles up" });
        }
        *g.m.nvg.teams.as_mut().unwrap().men.get_mut(gd).unwrap() = row;
    }
    let now = g.time;
    let gap = g.k.nvg.call_gap;
    let team = g.m.nvg.teams.as_mut().unwrap();
    if let Some(who) = caller {
        if team.call_due && now - team.called_t >= gap {
            team.call_due = false;
            team.called_t = now;
            crate::game::game_say(g, who, "going dark.", None, None);   // (said by the first man who really is dark, the frame there is one)
        }
    }
}

/// His weapon light is held off this frame.
pub fn nvg_dark(g: &crate::game::Game, gd: GuardId) -> bool {
    live_man(g, gd).map_or(false, |row| row.dark)
}

/// He wears the goggles at all.
pub fn nvg_worn(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
        && g.gd[gd].unit.is_some()
        && crate::kit::kit_goggles(g, gd)
        && crate::roomfight::fight_live(g)
}

/// His goggles are over his eyes.
pub fn nvg_down(g: &crate::game::Game, gd: GuardId) -> bool {
    live_man(g, gd).map_or(false, |row| row.down)
}

/// What he is looking at shows nothing even through the tube.
pub fn nvg_black(g: &crate::game::Game, gd: GuardId) -> bool {
    live_man(g, gd).map_or(false, |row| row.black)
}

/// The light to read for `o` through `viewer`'s goggles; 0 with them up.
pub fn nvg_sight(g: &mut crate::game::Game, viewer: GuardId, o: GuardId) -> f64 {
    if !nvg_down(g, viewer) {
        return 0.0;
    }
    let on_him = g.gd[o].irradiance;
    let e = on_him + crate::irlight::ir_on_man(g, o);
    tube_vis(g, e)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own bits under EXACT (bun, nvg.ts:109 and 110 with albedo 0.3, re-run on the snapshot): exposure, gain, E, k, sig
    #[test]
    fn the_tube_curve_gives_the_typescripts_bits() {
        let rows: [(f64, f64, f64, u64, u64); 6] = [
            (1.0, 1000.0, 0.03, 0x3fb8723a1d588a36, 0x3fc90c0d2d4ae68b),
            (0.7, 35.0, 0.5, 0x3fb11cc247bdfa59, 0x3fcbab47e33c45a4),
            (1.0, 1.0, 2.25, 0x3fb8723a1d588a36, 0x3fd1f872c94058d2),
            (1.3, 1.0, 0.0001, 0x3fbfc7b1f2f31a13, 0x3ef2c7944cbe6151),
            (2.5, 1000.0, 50.0, 0x3fce8ec8a4aeacc4, 0x3ff0000000000000),
            (1.0, 1000.0, -1.0, 0x3fb8723a1d588a36, 0),
        ];
        for (exposure, gain, e, k_bits, sig_bits) in rows {
            let (k, sig) = tube_curve_of(exposure, gain, 0.3, e);
            assert_eq!((k.to_bits(), sig.to_bits()), (k_bits, sig_bits), "{} {} {}", exposure, gain, e);
        }
    }

    #[test]
    fn a_fresh_team_remembers_nothing() {
        let t = fresh();
        assert_eq!((t.t, t.mode, t.why, t.call_due), (-100.0, Mode::Off, Why::NoRule, false));
        assert_eq!([t.lead_lit_t, t.hunt_t, t.alarm_t, t.called_t], [-m::INF; 4]);
    }
}
