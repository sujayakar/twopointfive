//! src/game/nvg.ts lines 1 to 210
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---- what the skeleton does not reach yet: the next skeleton changes these, not the bodies

/// `l.kind` of a light (nvg.ts:126).
fn seam_light_kind(g: &mut Game, l: LightId) -> f64 { g.tab.light[l].kind } // SKELETON-GAP was: RtLight.kind by LightId; skeleton 4 has the row (g.tab.light)
/// `l.enabled` of a light (nvg.ts:126, 183, 208).
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool { g.tab.light[l].enabled } // SKELETON-GAP was: RtLight.enabled by LightId; skeleton 4 has the row (g.tab.light)
/// `l.intensity` of a light (nvg.ts:126, 127).
fn seam_light_intensity(g: &mut Game, l: LightId) -> f64 { g.tab.light[l].intensity } // SKELETON-GAP was: RtLight.intensity by LightId; skeleton 4 has the row (g.tab.light)
/// `l.transient` of a light; absent reads false (nvg.ts:126).
fn seam_light_transient(g: &mut Game, l: LightId) -> bool { g.tab.light[l].transient.unwrap_or(false) } // SKELETON-GAP was: RtLight.transient by LightId; skeleton 4 has the row (g.tab.light)
/// `l.ttl` of a light (nvg.ts:126).
fn seam_light_ttl(g: &mut Game, l: LightId) -> f64 { g.tab.light[l].ttl } // SKELETON-GAP was: RtLight.ttl by LightId; skeleton 4 has the row (g.tab.light)
/// `l.dir` of a light (nvg.ts:127).
fn seam_light_dir(g: &mut Game, l: LightId) -> V3 { g.tab.light[l].dir } // SKELETON-GAP was: RtLight.dir by LightId; skeleton 4 has the row (g.tab.light)
/// `l.color` of a light (nvg.ts:127).
fn seam_light_color(g: &mut Game, l: LightId) -> V3 { g.tab.light[l].color } // SKELETON-GAP was: RtLight.color by LightId; skeleton 4 has the row (g.tab.light)

// ---- the team's state

/// A team nothing has happened to.   (nvg.ts fresh)
pub fn fresh() -> crate::nvg::Team {
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

/// One row of the QA log, the last 200.   (nvg.ts note)
pub fn note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) {
    let who = match gd {
        Some(id) => g.gd[id].callsign.clone(),
        None => String::from("-"),
    };
    let t = g.time;
    g.m.nvg.nvg_log.push(NvgNote { t, who, what: String::from(what) });
    if g.m.nvg.nvg_log.len() > 200 {
        g.m.nvg.nvg_log.drain(..50);
    }
}

/// Is the team's state this frame's (or the last few: the rig is drawn after the tick, a paused frame draws again).
fn is_live(g: &Game) -> bool {
    match &g.m.nvg.teams {
        Some(t) => t.mode != Mode::Off && g.time - t.t >= 0.0 && g.time - t.t < 0.25,
        None => false,
    }
}

/// The team's state while it is live.   (nvg.ts live)
pub fn live(g: &mut crate::game::Game) -> Option<crate::nvg::Team> {
    if is_live(g) { g.m.nvg.teams.clone() } else { None }
}

/// `live(g)?.men.get(gd)` with no copy of the team: the readers below are asked once for each pair of men a frame.
fn live_man(g: &Game, gd: GuardId) -> Option<Man> {
    if !is_live(g) {
        return None;
    }
    g.m.nvg.teams.as_ref().and_then(|t| t.men.get(gd).copied())
}

// ---- the tube

/// post.wgsl's intensifier curve: the signal's `k` and the signal of irradiance `e`.   (nvg.ts:102, 103, 108, 109)
fn tube_curve_of(exposure: f64, nv_gain: f64, albedo: f64, e: f64) -> (f64, f64) {
    let k = albedo / m::PI * (if exposure > 0.0 { exposure } else { 1.0 });
    let gain = m::max(if nv_gain > 0.0 { nv_gain } else { 1.0 }, 1.0);
    let sig = v3::clamp(m::log(1.0 + m::max(e, 0.0) * k * gain) / m::log(1.0 + gain), 0.0, 1.0);
    (k, sig)
}

/// The curve at the scene's exposure and gain, the picture's own settings.
fn tube_curve(g: &Game, e: f64) -> (f64, f64) {
    tube_curve_of(g.engine.settings.exposure, g.engine.settings.nv_gain, g.k.nvg.albedo, e)
}

/// The meter's 0..1 for a man under irradiance `e` seen through the goggles.   (nvg.ts tubeVis)
pub fn tube_vis(g: &mut crate::game::Game, e: f64) -> f64 {
    let (k, sig) = tube_curve(g, e);
    if sig > 0.0 {
        let seen = crate::game::game_visibility_of(g, sig / k);
        seen * sig / (sig + g.k.nvg.noise)
    } else {
        0.0
    }
}

/// The tube's picture of a matte floor under irradiance `e`, 0..1 of its range.   (nvg.ts tubeSig)
pub fn tube_sig(g: &mut crate::game::Game, e: f64) -> f64 {
    tube_curve(g, e).1
}

/// A living operative with a unit whose card left him his goggles.   (nvg.ts isMate)
pub fn is_mate(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
        && g.gd[gd].unit.is_some()
        && g.gd[gd].state != crate::game::GuardState::Dead
        && crate::kit::kit_goggles(g, gd)
}

/// When the other side last gave the team reason to think it knows.   (nvg.ts loudT)
pub fn loud_t(g: &mut crate::game::Game) -> f64 {
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

/// The light of the place at a point: the fixtures, nothing carried, plus the moon.   (nvg.ts nvgPlaceLight)
pub fn nvg_place_light(g: &mut crate::game::Game, p: V3) -> f64 {
    let mut every_owner = |_: &mut Game, _: f64| true;
    let mut e = crate::guards::light_estimate_at(g, p, Some(&mut every_owner), None, None, None);
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        if seam_light_kind(g, l) != 2.0
            || !seam_light_enabled(g, l)
            || seam_light_intensity(g, l) <= 0.0
            || seam_light_transient(g, l)
            || seam_light_ttl(g, l) >= 0.0
            || crate::hours::in_estimate(g, l)
        {
            continue;
        }
        let d = seam_light_dir(g, l);
        if !crate::scene_collision::segment_blocked(g, p, [p[0] - d[0] * 60.0, p[1] - d[1] * 60.0, p[2] - d[2] * 60.0], None, None, None) {
            let c = seam_light_color(g, l);
            e += (0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]) * seam_light_intensity(g, l);
        }
    }
    e
}

/// One point of his place: is it dark to the bare eye, and (if `tube`) too dark for the tube.   (nvg.ts readPlace's `at`)
fn read_at(g: &mut Game, x: f64, z: f64, tube: bool, lamp: bool, dim: &mut bool, black: &mut bool) {
    let e = nvg_place_light(g, [x, 1.2, z]);
    if crate::game::game_visibility_of(g, e) < g.k.nvg.dark_at {
        *dim = true;
    }
    if tube && !lamp && tube_vis(g, e) < g.k.nvg.black_at {
        *black = true;
    }
}

/// His place, read twice a second: dark to the bare eye, too dark for the tube.   (nvg.ts readPlace)
pub fn read_place(g: &mut crate::game::Game, gd: GuardId, m: &mut crate::nvg::Man) {
    if g.time - m.place_t >= 0.0 && g.time - m.place_t < 0.5 {
        return;
    }
    let c = g.gd[gd].ch;
    let (pos, aim_yaw) = (g.ch[c].pos, g.ch[c].aim_yaw);
    let eye: V3 = [pos[0], 1.5, pos[2]];
    let (fx, fz) = (m::sin(aim_yaw), m::cos(aim_yaw));
    let (mut dim, mut black) = (false, false);
    let lamp = crate::irlight::ir_carried(g);   // (a man with an illuminator has no place too dark for his tube)
    read_at(g, pos[0], pos[2], false, lamp, &mut dim, &mut black);
    for r in look_m() {
        let p: V3 = [eye[0] + fx * r, 1.2, eye[2] + fz * r];
        if crate::scene_collision::segment_blocked(g, eye, p, None, None, None) {
            break;
        }
        read_at(g, p[0], p[2], r <= TUBE_M, lamp, &mut dim, &mut black);
    }
    if let Some(a) = crate::seats::order_of(g, gd) {
        let (post, arrived) = (g.tab.orders_assignment[a].post, g.tab.orders_assignment[a].arrived);
        if let Some(post) = post {
            if !arrived && v3::dist_xz(pos, post) < g.k.lights.lit_mark_m {
                read_at(g, post[0], post[2], true, lamp, &mut dim, &mut black);
            }
        }
    }
    if let Some(o) = crate::roomfight::room_fight_op_of(g, gd) {
        if let Some(tgt) = g.tab.roomfight_op[o].tgt {
            if g.gd[tgt].state != crate::game::GuardState::Dead {
                let at = g.ch[g.gd[tgt].ch].pos;
                read_at(g, at[0], at[2], false, lamp, &mut dim, &mut black);
            }
        }
        if let Some((by_t, by_at)) = g.tab.roomfight_op[o].step_by.as_ref().map(|by| (by.t, by.at)) {
            if g.time - by_t < g.k.lights.step_secs {
                read_at(g, by_at[0], by_at[2], true, lamp, &mut dim, &mut black);
            }
        }
        if let Some(pt) = g.tab.roomfight_op[o].turn.as_ref().and_then(|turn| turn.pt) {
            read_at(g, pt[0], pt[2], true, lamp, &mut dim, &mut black);
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

/// Is the light switched on on the gun in this player's hands.   (nvg.ts:166)
fn lead_lit(g: &mut Game, pl: PlayerId) -> bool {
    let (pistol, rifle) = (g.pl[pl].pistol, g.pl[pl].rifle);
    (crate::game::player_pistol_up(g, pl) && g.fa[pistol].light_on) || (crate::game::player_rifle_up(g, pl) && g.fa[rifle].light_on)
}

/// Once a frame from Game.update: is the team dark, and each man's goggles.   (nvg.ts nvgTick)
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
    let seats = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let mut up = crate::seats::men(g, &seats);
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
            !seam_light_enabled(g, l)
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
            crate::game::game_say(g, who, "going dark.", None, None, None);   // (said by the first man who really is dark, the frame there is one)
        }
    }
}

/// His weapon light is held off this frame.   (nvg.ts nvgDark)
pub fn nvg_dark(g: &mut crate::game::Game, gd: GuardId) -> bool {
    live_man(g, gd).map_or(false, |row| row.dark)
}

/// He wears the goggles at all.   (nvg.ts nvgWorn)
pub fn nvg_worn(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative
        && g.gd[gd].unit.is_some()
        && crate::kit::kit_goggles(g, gd)
        && crate::roomfight::fight_live(g)
}

/// His goggles are over his eyes.   (nvg.ts nvgDown)
pub fn nvg_down(g: &mut crate::game::Game, gd: GuardId) -> bool {
    live_man(g, gd).map_or(false, |row| row.down)
}

/// What he is looking at shows nothing even through the tube.   (nvg.ts nvgBlack)
pub fn nvg_black(g: &mut crate::game::Game, gd: GuardId) -> bool {
    live_man(g, gd).map_or(false, |row| row.black)
}

/// The light to read for `o` through `viewer`'s goggles; 0 with them up.   (nvg.ts nvgSight)
pub fn nvg_sight(g: &mut crate::game::Game, viewer: GuardId, o: GuardId) -> f64 {
    if !nvg_down(g, viewer) {
        return 0.0;
    }
    let on_him = g.gd[o].irradiance;
    let e = on_him + crate::irlight::ir_on_man(g, o);
    tube_vis(g, e)
}

/// QA / the panel: the team's mode and why, and each man's kit.   (nvg.ts nvgState)
pub fn nvg_state(g: &mut crate::game::Game) -> crate::nvg::NvgState {
    let Some(team) = live(g) else {
        return NvgState { mode: Mode::Off, why: Why::NoRule, quiet_for: m::INF, men: Vec::new() };
    };
    let quiet_for = g.time - loud_t(g);
    let men = team.men.iter().map(|(gd, row)| NvgStateMenItem { gd, down: row.down, dark: row.dark, dim: row.dim, black: row.black }).collect();
    NvgState { mode: team.mode, why: team.why, quiet_for, men }
}

/// 'team dark (not compromised) · Blue One goggles · Blue Two light'.   (nvg.ts nvgSummary)
pub fn nvg_summary(g: &mut crate::game::Game) -> String {
    let s = nvg_state(g);
    if s.mode == Mode::Off {
        return String::from("night vision: no team on the floor");
    }
    let mut out = format!("team {} ({}) · ", if s.mode == Mode::Dark { "dark" } else { "lights free" }, s.why.as_str());
    for (i, row) in s.men.iter().enumerate() {
        if i > 0 {
            out.push_str(" · ");
        }
        let kit = if row.down {
            "goggles"
        } else {
            let l = g.gd[row.gd].flashlight;
            if seam_light_enabled(g, l) { "light" } else { "bare eyes" }
        };
        out.push_str(&format!("{} {}{}", g.gd[row.gd].callsign, kit, if row.black { ", black ahead" } else { "" }));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own bits under EXACT (bun, nvg.ts:108 and 109 with albedo 0.3)
    #[test]
    fn the_tube_curve_gives_the_typescripts_bits() {
        let rows: [(f64, f64, f64, u64); 6] = [
            (1.0, 1000.0, 0.03, 0x3fc90c0d2d4ae68b),
            (0.7, 35.0, 0.5, 0x3fcbab47e33c45a4),
            (0.0, 0.0, 2.25, 0x3fd1f872c94058d2),
            (1.3, 0.5, 0.0001, 0x3ef2c7944cbe6151),
            (2.5, 1000.0, 50.0, 0x3ff0000000000000),
            (1.0, 1000.0, -1.0, 0),
        ];
        for (exposure, gain, e, bits) in rows {
            assert_eq!(tube_curve_of(exposure, gain, 0.3, e).1.to_bits(), bits, "{} {} {}", exposure, gain, e);
        }
    }

    #[test]
    fn a_fresh_team_remembers_nothing() {
        let t = fresh();
        assert_eq!((t.t, t.mode, t.why, t.call_due), (-100.0, Mode::Off, Why::NoRule, false));
        assert_eq!([t.lead_lit_t, t.hunt_t, t.alarm_t, t.called_t], [-m::INF; 4]);
    }
}
