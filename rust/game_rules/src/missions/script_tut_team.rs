#![allow(unused_variables, unused_imports)]
//! levels/tut-team.script.ts: the squad's tutorial mission on the ground. Its define pass, a step function for each beat, its three helpers.

pub use ::game_types::script_tut_team::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the script's locals, as this run's define pass left them
fn tut_team_vars(g: &crate::game::Game, r: DirectorRunId) -> &crate::script_tut_team::TutTeamVars { match &g.tab.director_run[r].vars { crate::director::ScriptVars::TutTeam(v) => v, _ => panic!("the run is not of levels/tut-team.script.ts") } }

/// the roadblock's three posts, and the names its men answer to (POSTS)
const POSTS: [(&str, V3); 3] = [("Tango 6", [34.0, 0.0, 21.6]), ("Tango 7", [34.9, 0.0, 23.6]), ("Tango 8", [32.4, 0.0, 23.2])];
/// where the roadblock's rounds go while the squad is behind its trucks: lo, hi, w, by (ZONES)
const ZONES: [(V3, V3, f64, Option<&[&str]>); 7] = [
    ([10.9, 0.65, 21.7], [11.9, 1.15, 22.9], 3.0, None),
    ([9.3, 0.65, 24.0], [10.5, 1.15, 25.2], 3.0, Some(&["Tango 6", "Tango 8"])),
    ([16.0, 0.05, 21.6], [19.0, 0.05, 23.0], 3.0, Some(&["Tango 7", "Tango 8"])),
    ([15.0, 0.05, 23.6], [18.5, 0.05, 24.4], 2.0, Some(&["Tango 6"])),
    ([3.0, 0.7, 20.5], [9.0, 1.6, 20.5], 3.0, None),
    ([7.5, 0.7, 25.7], [11.0, 1.1, 26.1], 3.0, Some(&["Tango 6"])),
    ([1.5, 2.9, 22.0], [1.5, 3.2, 25.0], 2.0, Some(&["Tango 6", "Tango 8"])),
];
/// what the men behind the trucks call while they are pinned, in turn (PINNED)
const PINNED: [(&str, &str); 3] = [("Blue Two", "Taking fire! Stay down!"), ("Red Two", "Rounds coming past the trucks!"), ("Blue One", "We can't stay here!")];
/// where the squad's two smokes land (SMOKES)
const SMOKES: [V3; 2] = [[15.6, 0.0, 22.4], [17.8, 0.0, 24.2]];
const SMOKE_AGAIN: f64 = 27.0;
/// what each thrower calls, the first cans and the cans after them (SMOKE_CALLS)
const SMOKE_CALLS: [[&str; 2]; 2] = [["Throwing smoke!", "Smoke out!"], ["More smoke!", "Smoke out!"]];
const SMOKE_CALLED: f64 = 0.7;
const SMOKE_APART: f64 = 0.45;
const SMOKE_UP: f64 = 1.6;
/// once the smoke is out: into it, and the south lane behind it (AT_SMOKE)
const AT_SMOKE: [V3; 4] = [[15.4, 0.9, 24.5], [14.2, 0.9, 24.8], [16.6, 0.95, 24.6], [13.6, 0.85, 24.9]];
const LINGER: f64 = 9.0;
const SQUAD: [&str; 4] = ["Red One", "Red Two", "Blue One", "Blue Two"];
const YARD_X0: f64 = 4.4;
const YARD_X1: f64 = 20.5;
const YARD_Z0: f64 = 12.2;
const YARD_Z1: f64 = 19.7;
const WEST: V3 = [4.5, 1.1, 23.6];
const BUNK_MEN: [&str; 2] = ["Tango 4", "Tango 5"];
const BUNK_DOOR: V3 = [28.6, 1.1, 16.25];
const COOK: &str = "Tango 3";
const PASS_IN: V3 = [14.2, 1.1, 6.25];

/// `stop` of rake and harass: which closure a caller handed in (ambush's, blind's, linger's)
const STOP_SMOKE_OR_BROKE: u16 = 1;
const STOP_THROUGH: u16 = 2;
const STOP_PUSHED: u16 = 3;

/// ZONES as the frame holds it
fn zones() -> Vec<TutTeamZone> {
    ZONES.iter().map(|&(lo, hi, w, by)| TutTeamZone { lo, hi, w, by: by.map(|names| names.iter().map(|n| String::from(*n)).collect()) }).collect()
}

/// c.flag[name] === true (a flag of this script is only ever written true, so its truthy tests read the same)
fn flag(g: &Game, r: DirectorRunId, name: &str) -> bool {
    matches!(g.tab.director_run[r].flags.get(String::from(name)), Some(Some(crate::director::Json::Bool(true))))
}
/// c.flag[name] = true
fn set_flag(g: &mut Game, r: DirectorRunId, name: &str) {
    g.tab.director_run[r].flags.set(String::from(name), Some(crate::director::Json::Bool(true)));
}
/// the four flags a beat that starts clean from a checkpoint sets, in the script's order
fn set_all_flags(g: &mut Game, r: DirectorRunId) {
    set_flag(g, r, "broke");
    set_flag(g, r, "smoke");
    set_flag(g, r, "through");
    set_flag(g, r, "pushed");
}
/// the `stop` a rake or harass was handed
fn stop_holds(g: &Game, r: DirectorRunId, stop: u16) -> bool {
    match stop {
        STOP_SMOKE_OR_BROKE => flag(g, r, "smoke") || flag(g, r, "broke"),
        STOP_THROUGH => flag(g, r, "through"),
        STOP_PUSHED => flag(g, r, "pushed"),
        _ => true,
    }
}

/// the men of the roadblock on their feet, by name, spawned once
fn block(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, spawn: bool) -> Result<Vec<crate::director::UnitRef>, String> {
    let mut out = Vec::new();
    for (name, at) in POSTS {
        let mut u = crate::director::director_beat_unit(g, r, name);
        if u.is_none() && spawn {
            let (rifle, yaw) = {
                let v = tut_team_vars(g, r);
                (v.rifle.clone().unwrap(), v.yaw)
            };
            u = Some(crate::director::director_beat_spawn(g, r, b, &rifle, &crate::director::Place::List(at), Some(name), Some(yaw), Some(false))?);
        }
        if let Some(u) = u {
            if crate::director::director_query_alive(g, r, &u) {
                out.push(u);
            }
        }
    }
    Ok(out)
}
/// the squad's men on their feet
fn squad(g: &mut Game, r: DirectorRunId) -> Vec<crate::director::UnitRef> {
    let mut out = Vec::new();
    for name in SQUAD {
        if let Some(u) = crate::director::director_beat_unit(g, r, name) {
            if crate::director::director_query_alive(g, r, &u) {
                out.push(u);
            }
        }
    }
    out
}
/// inside the bunkhouse's own walls
fn in_bunk(g: &Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<bool, String> {
    let p = crate::director::director_query_pos_unit(g, r, u)?;
    Ok(p[0] > 29.1 && p[0] < 35.9 && p[2] > 13.6 && p[2] < 19.9)
}
fn in_kitchen(g: &Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<bool, String> {
    let p = crate::director::director_query_pos_unit(g, r, u)?;
    Ok(p[0] > 14.0 && p[0] < 22.0 && p[2] > 4.0 && p[2] < 11.5)
}
fn in_yard(g: &Game, r: DirectorRunId, u: &crate::director::UnitRef) -> Result<bool, String> {
    let p = crate::director::director_query_pos_unit(g, r, u)?;
    Ok(p[0] > YARD_X0 && p[0] < YARD_X1 && p[2] > YARD_Z0 && p[2] < YARD_Z1)
}
/// which index pick(c, a) takes of a list this long: one die
fn pick(g: &mut Game, r: DirectorRunId, len: usize) -> usize {
    let n = len as f64;
    let roll = crate::director::director_beat_rand(g, r);
    m::min(n - 1.0, m::floor(roll * n)) as usize
}
/// a + c.rand() * (b - a) (rake's closure mix, tut-team.script.ts:74)
fn mix(g: &mut Game, r: DirectorRunId, a: f64, b: f64) -> f64 {
    a + crate::director::director_beat_rand(g, r) * (b - a)
}
/// { goal: null, aimAt: at, upper: 'aim', hot }: the order the script gives most
fn aim_order(at: V3, hot: bool) -> crate::director::OrderPatch2 {
    crate::director::OrderPatch2 { goal: Some(None), aim_at: Some(Some(crate::director::Place::List(at))), upper: Some(Some(crate::character::UpperMode::Aim)), hot: Some(Some(hot)), ..Default::default() }
}
/// c.until(<the id>)
fn until(id: u32) -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id }, None)
}
/// c.any(c.wait(secs), c.until(<the id>))
fn wait_or(g: &Game, r: DirectorRunId, secs: f64, id: u32) -> Result<crate::director::Cond, String> {
    let wait = crate::director::director_beat_wait(g, r, secs, None)?;
    Ok(crate::director::director_beat_any(vec![wait, until(id)]))
}
/// `c.unit('Red One') ?? squad(c)[0]`, the man who speaks for the squad
fn speaker(g: &mut Game, r: DirectorRunId) -> Option<crate::director::UnitRef> {
    match crate::director::director_beat_unit(g, r, "Red One") {
        Some(u) => Some(u),
        None => squad(g, r).into_iter().next(),
    }
}
/// hold's closure met
fn hold_met(g: &mut Game, r: DirectorRunId) -> Result<bool, String> {
    let door = tut_team_vars(g, r).bunk_door.clone().unwrap();
    if crate::director::director_query_door(g, &door)?.open {
        return Ok(true);
    }
    for m in squad(g, r) {
        if in_bunk(g, r, &m)? {
            return Ok(true);
        }
    }
    Ok(false)
}
/// cook's closure met
fn cook_met(g: &mut Game, r: DirectorRunId) -> Result<bool, String> {
    let (pass, kitchen) = {
        let v = tut_team_vars(g, r);
        (v.pass_door.clone().unwrap(), v.kitchen_door.clone().unwrap())
    };
    if crate::director::director_query_door(g, &pass)?.open || crate::director::director_query_door(g, &kitchen)?.open {
        return Ok(true);
    }
    for m in squad(g, r) {
        if in_kitchen(g, r, &m)? {
            return Ok(true);
        }
    }
    Ok(false)
}

///  the define pass: the loadout, the three doors, the response team off, the one program.
pub(super) fn tut_team_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_team::TutTeamVars, String> {
    let rifle = crate::director::manifest_loadout(m, "roadblock", crate::director::LoadoutSpec { team: crate::teams::Team::Hostile, weapon: Some(crate::weapons::WeaponId::Akms), tint: None, tint2: None, light: Some(false), trained: None });
    let yaw = -crate::m::PI / 2.0;
    let pass_door = crate::director::manifest_door(m, "pass");
    let kitchen_door = crate::director::manifest_door(m, "kitchen");
    let bunk_door = crate::director::manifest_door(m, "bunk");
    m.systems.set("qrf", false);
    crate::director::manifest_program(
        m,
        "tutorial",
        &mut |p: &mut crate::director::ProgramDef| -> Result<(), String> {
            crate::director::program_on_start(p, "hold");
            crate::director::program_on_start(p, "cook");
            crate::director::program_on_signal(p, "lesson:ambush", "ambush");
            crate::director::program_on_signal(p, "lesson:smoke", "smoke");
            crate::director::program_on_signal(p, "lesson:push", "push");
            crate::director::program_on_signal(p, "lesson:indoors", "indoors");
            crate::director::program_on_signal(p, "lesson:house", "house");
            crate::director::program_on_signal(p, "lesson:out", "out");
            for name in ["hold", "cook", "ambush", "pinned", "smoke", "break", "ordered", "blind", "linger", "push", "indoors", "house", "out"] {
                crate::director::program_beat(p, name, &crate::scene_level::LevelScriptFn::TutTeam, None)?;
            }
            Ok(())
        })?;
    Ok(crate::script_tut_team::TutTeamVars { rifle: Some(rifle), yaw, pass_door: Some(pass_door), kitchen_door: Some(kitchen_door), bunk_door: Some(bunk_door) })
}

/// the bunkhouse keeps to itself until its door opens or one of the squad is in its room (P.beat 'hold')
pub(super) fn tut_team_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            if hold_met(g, r)? {
                return Ok(None);
            }
            let mut held = Vec::new();
            for name in BUNK_MEN {
                if let Some(u) = crate::director::director_beat_unit(g, r, name) {
                    if crate::director::director_query_alive(g, r, &u) && in_bunk(g, r, &u)? {
                        held.push(u);
                    }
                }
            }
            for u in &held {
                crate::director::director_beat_order(g, r, b, u, &aim_order(BUNK_DOOR, false))?;
            }
            f.held = held;
            f.at = 1;
            Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))))
        }
        1 => {
            f.at = 2;
            Ok(Some(until(1)))
        }
        2 => {
            for i in 0..f.held.len() {
                let u = f.held[i].clone();
                if crate::director::director_query_alive(g, r, &u) && in_bunk(g, r, &u)? {
                    crate::director::director_beat_release(g, r, b, &u, Some(crate::squad::EndGuardScriptState::Alert))?;
                }
            }
            f.at = 3;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// (P.beat 'hold') 1: met
pub(super) fn tut_team_hold_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamHoldFrame, id: u32) -> Result<bool, String> {
    match id {
        1 => hold_met(g, r),
        _ => Err(String::from("tut-team hold: no such test")),
    }
}

/// the kitchen's man stays in his kitchen until a door of his room opens or one of the squad is in it (P.beat 'cook')
pub(super) fn tut_team_cook_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamCookFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            let u = match crate::director::director_beat_unit(g, r, COOK) {
                Some(u) => u,
                None => return Ok(None),
            };
            if !crate::director::director_query_alive(g, r, &u) || !in_kitchen(g, r, &u)? {
                return Ok(None);
            }
            if cook_met(g, r)? {
                return Ok(None);
            }
            crate::director::director_beat_order(g, r, b, &u, &aim_order(PASS_IN, false))?;
            f.u = Some(u);
            f.at = 1;
            Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))))
        }
        1 => {
            f.at = 2;
            Ok(Some(until(1)))
        }
        2 => {
            let u = f.u.clone().unwrap();
            if crate::director::director_query_alive(g, r, &u) {
                crate::director::director_beat_release(g, r, b, &u, Some(crate::squad::EndGuardScriptState::Alert))?;
            }
            f.at = 3;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// (P.beat 'cook') 1: he is down, or met
pub(super) fn tut_team_cook_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamCookFrame, id: u32) -> Result<bool, String> {
    match id {
        1 => {
            let u = f.u.clone().unwrap();
            if !crate::director::director_query_alive(g, r, &u) {
                return Ok(true);
            }
            cook_met(g, r)
        }
        _ => Err(String::from("tut-team cook: no such test")),
    }
}

/// the roadblock opens up on the two trucks and keeps it up; the squad is held on its knees (P.beat 'ambush')
pub(super) fn tut_team_ambush_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamAmbushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            for u in block(g, r, b, true)? {
                let o = crate::director::OrderPatch2 { goal: Some(None), aim_at: Some(Some(crate::director::Place::List(WEST))), upper: Some(Some(crate::character::UpperMode::Aim)), crouch: Some(Some(false)), hot: Some(Some(true)), flinch: Some(Some(false)), ..Default::default() };
                crate::director::director_beat_order(g, r, b, &u, &o)?;
            }
            for u in squad(g, r) {
                let o = crate::director::OrderPatch2 { goal: Some(None), crouch: Some(Some(true)), hot: Some(Some(false)), flinch: Some(Some(true)), ..Default::default() };
                crate::director::director_beat_order(g, r, b, &u, &o)?;
            }
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, 0.6, None)?))
        }
        1 => {
            if let Some(red) = crate::director::director_beat_unit(g, r, "Red One") {
                crate::director::director_beat_say(g, r, b, &red, "Contact front! Roadblock!", Some(true))?;
            }
            crate::director::director_beat_start(g, r, b, "pinned");
            f.sub_rake = Some(Box::new(crate::script_tut_team::TutTeamRakeHelperFrame {
                zones: zones(),
                stop: STOP_SMOKE_OR_BROKE,
                o: Some(crate::script_tut_team::TutTeamRakeHelperO { rounds: [5.0, 9.0], gap: [0.2, 0.7], spread: 0.014 }),
                ..Default::default()
            }));
            f.at = 2;
            tut_team_ambush_step(g, r, b, f, got)
        }
        2 => {
            let mut sub = match f.sub_rake.take() {
                Some(sub) => sub,
                None => return Ok(None),
            };
            match tut_team_rake_helper_step(g, r, b, &mut sub, got)? {
                Some(c) => {
                    f.sub_rake = Some(sub);
                    Ok(Some(c))
                }
                None => {
                    f.at = 3;
                    Ok(None)
                }
            }
        }
        _ => Ok(None),
    }
}

/// what the men behind the trucks call while they are pinned (P.beat 'pinned')
pub(super) fn tut_team_pinned_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPinnedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                f.k = 0.0;
                f.at = 1;
            }
            1 => {
                if flag(g, r, "smoke") || flag(g, r, "broke") {
                    f.at = 3;
                    return Ok(None);
                }
                let secs = 5.5 + crate::director::director_beat_rand(g, r) * 2.0;
                f.at = 2;
                return Ok(Some(wait_or(g, r, secs, 1)?));
            }
            2 => {
                if flag(g, r, "smoke") || flag(g, r, "broke") {
                    f.at = 3;
                    return Ok(None);
                }
                let (who, line) = PINNED[js::rem(f.k, PINNED.len() as f64) as usize];
                if let Some(u) = crate::director::director_beat_unit(g, r, who) {
                    if crate::director::director_query_alive(g, r, &u) {
                        crate::director::director_beat_say(g, r, b, &u, line, Some(true))?;
                    }
                }
                f.k += 1.0;
                f.at = 1;
            }
            _ => return Ok(None),
        }
    }
}

/// (P.beat 'pinned') 1: smoke or broke
pub(super) fn tut_team_pinned_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamPinnedFrame, id: u32) -> Result<bool, String> {
    match id {
        1 => Ok(flag(g, r, "smoke") || flag(g, r, "broke")),
        _ => Err(String::from("tut-team pinned: no such test")),
    }
}

/// the squad's own smoke goes out; more goes out while no order comes (P.beat 'smoke')
pub(super) fn tut_team_smoke_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamSmokeFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                if flag(g, r, "smoke") {
                    f.at = 7;
                    return Ok(None);
                }
                set_flag(g, r, "smoke");
                crate::director::director_beat_start(g, r, b, "ordered");
                crate::director::director_beat_start(g, r, b, "break");
                crate::director::director_beat_start(g, r, b, "linger");
                f.at = 1;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.4, None)?));
            }
            1 => {
                crate::director::director_beat_start(g, r, b, "blind");
                f.sub_smoke = Some(Box::new(crate::script_tut_team::TutTeamSmokeHelperFrame { again: false, ..Default::default() }));
                f.at = 2;
            }
            2 => {
                let mut sub = match f.sub_smoke.take() {
                    Some(sub) => sub,
                    None => return Ok(None),
                };
                if let Some(c) = tut_team_smoke_helper_step(g, r, b, f, &mut sub, got)? {
                    f.sub_smoke = Some(sub);
                    return Ok(Some(c));
                }
                f.at = 3;
                return Ok(Some(crate::director::director_beat_wait(g, r, SMOKE_UP, None)?));
            }
            3 => {
                crate::director::director_beat_signal(g, r, b, "done:smoke", None);
                f.at = 4;
            }
            4 => {
                if flag(g, r, "broke") {
                    f.at = 7;
                    return Ok(None);
                }
                f.at = 5;
                return Ok(Some(wait_or(g, r, SMOKE_AGAIN - SMOKE_CALLED - SMOKE_APART - SMOKE_UP, 1)?));
            }
            5 => {
                if flag(g, r, "broke") {
                    f.at = 4;
                } else {
                    f.sub_smoke = Some(Box::new(crate::script_tut_team::TutTeamSmokeHelperFrame { again: true, ..Default::default() }));
                    f.at = 6;
                }
            }
            6 => {
                let mut sub = match f.sub_smoke.take() {
                    Some(sub) => sub,
                    None => return Ok(None),
                };
                if let Some(c) = tut_team_smoke_helper_step(g, r, b, f, &mut sub, got)? {
                    f.sub_smoke = Some(sub);
                    return Ok(Some(c));
                }
                f.at = 4;
            }
            _ => return Ok(None),
        }
    }
}

/// (P.beat 'smoke') 1: broke
pub(super) fn tut_team_smoke_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamSmokeFrame, id: u32) -> Result<bool, String> {
    match id {
        1 => Ok(flag(g, r, "broke")),
        _ => Err(String::from("tut-team smoke: no such test")),
    }
}

/// the first order lets the squad go, for the yard (P.beat 'break')
pub(super) fn tut_team_break_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamBreakFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(until(1)))
        }
        1 => {
            f.at = 2;
            Ok(Some(crate::director::director_beat_wait(g, r, 0.25, None)?))
        }
        2 => {
            for u in squad(g, r) {
                crate::director::director_beat_release(g, r, b, &u, Some(crate::squad::EndGuardScriptState::Patrol))?;
            }
            f.at = 3;
            Ok(Some(wait_or(g, r, 2.2, 2)?))
        }
        3 => {
            set_flag(g, r, "through");
            f.at = 4;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// (P.beat 'break') 1: broke; 2: every man of the squad in the yard
pub(super) fn tut_team_break_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamBreakFrame, id: u32) -> Result<bool, String> {
    match id {
        1 => Ok(flag(g, r, "broke")),
        2 => {
            for u in squad(g, r) {
                if !in_yard(g, r, &u)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Err(String::from("tut-team break: no such test")),
    }
}

/// the player's first order to the squad breaks them out (P.beat 'ordered')
pub(super) fn tut_team_ordered_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamOrderedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(crate::director::director_beat_on(g, r, crate::director::CondArm3Kind::Signal, None, Some(crate::director::EventFilter::OrderSignal))))
        }
        1 => {
            set_flag(g, r, "broke");
            f.at = 2;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// the blind fire into the smoke, until the squad is through (P.beat 'blind')
pub(super) fn tut_team_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        f.sub_harass = Some(Box::new(crate::script_tut_team::TutTeamHarassHelperFrame {
            marks: AT_SMOKE.to_vec(),
            stop: STOP_THROUGH,
            o: Some(crate::script_tut_team::TutTeamHarassHelperO { rounds: [3.0, 5.0], gap: [0.5, 1.2], spread: 0.02 }),
            ..Default::default()
        }));
        f.at = 1;
    }
    let mut sub = match f.sub_harass.take() {
        Some(sub) => sub,
        None => return Ok(None),
    };
    match tut_team_harass_helper_step(g, r, b, &mut sub, got)? {
        Some(c) => {
            f.sub_harass = Some(sub);
            Ok(Some(c))
        }
        None => {
            f.at = 2;
            Ok(None)
        }
    }
}

/// if he stays on the road the roadblock starts again, until the gate's step (P.beat 'linger')
pub(super) fn tut_team_linger_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamLingerFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            f.at = 1;
            Ok(Some(until(1)))
        }
        1 => {
            f.at = 2;
            Ok(Some(wait_or(g, r, LINGER, 2)?))
        }
        2 => {
            f.sub_harass = Some(Box::new(crate::script_tut_team::TutTeamHarassHelperFrame {
                marks: AT_SMOKE.to_vec(),
                stop: STOP_PUSHED,
                o: Some(crate::script_tut_team::TutTeamHarassHelperO { rounds: [3.0, 5.0], gap: [0.7, 1.6], spread: 0.02 }),
                ..Default::default()
            }));
            f.at = 3;
            tut_team_linger_step(g, r, b, f, got)
        }
        3 => {
            let mut sub = match f.sub_harass.take() {
                Some(sub) => sub,
                None => return Ok(None),
            };
            match tut_team_harass_helper_step(g, r, b, &mut sub, got)? {
                Some(c) => {
                    f.sub_harass = Some(sub);
                    Ok(Some(c))
                }
                None => {
                    f.at = 4;
                    Ok(None)
                }
            }
        }
        _ => Ok(None),
    }
}

/// (P.beat 'linger') 1: through or pushed; 2: pushed
pub(super) fn tut_team_linger_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamLingerFrame, id: u32) -> Result<bool, String> {
    match id {
        1 => Ok(flag(g, r, "through") || flag(g, r, "pushed")),
        2 => Ok(flag(g, r, "pushed")),
        _ => Err(String::from("tut-team linger: no such test")),
    }
}

/// the roadblock's men come on for the gate one after another, and again while any is on his feet (P.beat 'push')
pub(super) fn tut_team_push_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                set_all_flags(g, r);
                f.at = 1;
                return Ok(Some(crate::director::director_beat_wait(g, r, 0.8, None)?));
            }
            1 => {
                if let Some(u) = speaker(g, r) {
                    if !block(g, r, b, false)?.is_empty() {
                        crate::director::director_beat_say(g, r, b, &u, "They're coming for the gate. We've got it.", Some(true))?;
                    }
                }
                f.k = 0.0;
                f.at = 2;
            }
            // the outer for's test, and its body's head
            2 => {
                if block(g, r, b, false)?.is_empty() {
                    crate::director::director_beat_signal(g, r, b, "done:roadblock", None);
                    f.at = 7;
                    return Ok(None);
                }
                f.left = block(g, r, b, false)?;
                f.i = 0.0;
                f.at = 3;
            }
            // the inner for over left.entries()
            3 => {
                let i = f.i;
                if (i as usize) < f.left.len() {
                    let u = f.left[i as usize].clone();
                    let bend = m::min(1.0, f.k);
                    let o = crate::director::OrderPatch2 {
                        goal: Some(Some(crate::director::Place::List([14.6 + i * 0.7 - bend * 0.9, 0.0, 19.2 - i * 0.5 - bend * 0.6]))),
                        speed: Some(Some(3.4)),
                        aim_at: Some(Some(crate::director::Place::List([11.4, 1.1, 16.4]))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        hot: Some(Some(true)),
                        direct: Some(Some(true)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, &u, &o)?;
                    if i < f.left.len() as f64 - 1.0 {
                        f.at = 4;
                        return Ok(Some(wait_or(g, r, 0.7, 1)?));
                    }
                    f.i += 1.0;
                } else {
                    f.at = 5;
                    let secs = if js::truthy(f.k) { 5.0 } else { 7.0 };
                    return Ok(Some(wait_or(g, r, secs, 2)?));
                }
            }
            4 => {
                f.i += 1.0;
                f.at = 3;
            }
            5 => {
                for u in block(g, r, b, false)? {
                    crate::director::director_beat_release(g, r, b, &u, Some(crate::squad::EndGuardScriptState::Alert))?;
                }
                f.at = 6;
                return Ok(Some(wait_or(g, r, 4.0, 3)?));
            }
            6 => {
                f.k += 1.0;
                f.at = 2;
            }
            _ => return Ok(None),
        }
    }
}

/// (P.beat 'push') 1 and 3: the roadblock is down; 2: every man of `left` is down or there
pub(super) fn tut_team_push_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamPushFrame, id: u32) -> Result<bool, String> {
    match id {
        1 | 3 => Ok(block(g, r, b, false)?.is_empty()),
        2 => {
            for i in 0..f.left.len() {
                let u = f.left[i].clone();
                if crate::director::director_query_alive(g, r, &u) && !crate::director::director_query_arrived(g, r, &u)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Err(String::from("tut-team push: no such test")),
    }
}

/// the road is quiet; whoever of the roadblock is left is held where he stands (P.beat 'indoors')
pub(super) fn tut_team_indoors_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamIndoorsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            set_all_flags(g, r);
            for u in block(g, r, b, false)? {
                crate::director::director_beat_order(g, r, b, &u, &aim_order([16.5, 1.1, 21.4], false))?;
            }
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, 0.1, None)?))
        }
        1 => {
            crate::director::director_beat_signal(g, r, b, "done:roadblock", None);
            f.at = 2;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// one man says the range is theirs (P.beat 'house')
pub(super) fn tut_team_house_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHouseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            set_all_flags(g, r);
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, 0.6, None)?))
        }
        1 => {
            if let Some(u) = speaker(g, r) {
                crate::director::director_beat_say(g, r, b, &u, "Kitchen's clear. There's one more building across the yard.", Some(true))?;
            }
            f.at = 2;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// one man says the station is theirs (P.beat 'out')
pub(super) fn tut_team_out_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamOutFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            set_all_flags(g, r);
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, 0.4, None)?))
        }
        1 => {
            if let Some(u) = speaker(g, r) {
                crate::director::director_beat_say(g, r, b, &u, "Station's clear.", Some(true))?;
            }
            f.at = 2;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// the roadblock fires in turns, each burst walked over one of `zones`, until `stop` (function* rake)
fn tut_team_rake_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamRakeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let o = f.o.clone().unwrap_or_default();
    loop {
        match f.at {
            // the head of the function: total, once
            0 => {
                f.total = f.zones.iter().fold(0.0, |a, z| a + z.w);
                f.at = 1;
            }
            // the outer while, its test and the body as far as the first yield
            1 => {
                if stop_holds(g, r, f.stop) {
                    f.at = 4;
                    return Ok(None);
                }
                let men = block(g, r, b, false)?;
                if men.is_empty() {
                    f.at = 4;
                    return Ok(None);
                }
                let mut roll = crate::director::director_beat_rand(g, r) * f.total;
                let mut zone = f.zones[0].clone();
                for z in &f.zones {
                    roll -= z.w;
                    if roll <= 0.0 {
                        zone = z.clone();
                        break;
                    }
                }
                let can: Vec<crate::director::UnitRef> = men
                    .into_iter()
                    .filter(|m| match &zone.by {
                        None => true,
                        Some(by) => by.iter().any(|name| *name == m.callsign),
                    })
                    .collect();
                if can.is_empty() {
                    f.at = 1;
                    return Ok(Some(crate::director::director_beat_wait(g, r, 0.05, None)?));
                }
                let u = can[pick(g, r, can.len())].clone();
                let heavy = u.callsign == "Tango 7";
                let rounds = m::round(mix(g, r, o.rounds[0], o.rounds[1]));
                f.left = rounds * (if heavy { 2.0 } else { 1.0 });
                f.u = Some(u);
                f.heavy = heavy;
                f.z = Some(zone);
                f.at = 2;
            }
            // the inner while, its test and the body as far as the burst
            2 => {
                let u = f.u.clone().unwrap();
                if f.left > 0.0 && !stop_holds(g, r, f.stop) && crate::director::director_query_alive(g, r, &u) {
                    let (lo, hi) = {
                        let z = f.z.as_ref().unwrap();
                        (z.lo, z.hi)
                    };
                    let x = mix(g, r, lo[0], hi[0]);
                    let y = mix(g, r, lo[1], hi[1]);
                    let z = mix(g, r, lo[2], hi[2]);
                    let at: V3 = [x, y, z];
                    let n = m::min(f.left, if crate::director::director_beat_rand(g, r) < 0.5 { 2.0 } else { 3.0 });
                    f.left -= n;
                    crate::director::director_beat_order(g, r, b, &u, &aim_order(at, true))?;
                    f.at_2 = at;
                    f.n = n;
                    f.at = 3;
                    let cadence = if f.heavy { 0.085 } else { 0.11 };
                    return Ok(Some(crate::director::director_beat_burst(g, r, b, &u, crate::director::RunBurstsItemAt::List(at), n, Some(cadence), Some(o.spread))?));
                }
                let secs = mix(g, r, o.gap[0], o.gap[1]);
                f.at = 1;
                return Ok(Some(crate::director::director_beat_wait(g, r, secs, None)?));
            }
            3 => {
                let secs = mix(g, r, 0.05, 0.16);
                f.at = 2;
                return Ok(Some(crate::director::director_beat_wait(g, r, secs, None)?));
            }
            _ => return Ok(None),
        }
    }
}

/// the roadblock fires in turns at `marks` until `stop` (function* harass)
fn tut_team_harass_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_team::TutTeamHarassHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let o = f.o.clone().unwrap_or_default();
    match f.at {
        // the while's test and its body as far as the burst
        0 | 1 => {
            if stop_holds(g, r, f.stop) {
                f.at = 3;
                return Ok(None);
            }
            let men = block(g, r, b, false)?;
            if men.is_empty() {
                f.at = 3;
                return Ok(None);
            }
            let at = f.marks[pick(g, r, f.marks.len())];
            let north = men.iter().find(|m| m.callsign == "Tango 6").cloned();
            let u = match north {
                Some(north) if at[2] > 24.0 => north,
                _ => men[pick(g, r, men.len())].clone(),
            };
            let heavy = u.callsign == "Tango 7";
            crate::director::director_beat_order(g, r, b, &u, &aim_order(at, true))?;
            let rounds = m::round(o.rounds[0] + crate::director::director_beat_rand(g, r) * (o.rounds[1] - o.rounds[0]));
            let n = rounds * (if heavy { 2.0 } else { 1.0 });
            let cadence = if heavy { 0.085 } else { 0.11 };
            f.at_2 = at;
            f.u = Some(u.clone());
            f.heavy = heavy;
            f.n = n;
            f.at = 2;
            Ok(Some(crate::director::director_beat_burst(g, r, b, &u, crate::director::RunBurstsItemAt::List(at), n, Some(cadence), Some(o.spread))?))
        }
        2 => {
            let secs = o.gap[0] + crate::director::director_beat_rand(g, r) * (o.gap[1] - o.gap[0]);
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, secs, None)?))
        }
        _ => Ok(None),
    }
}

/// the helper's closure call: a man calls his can
fn smoke_call(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamSmokeHelperFrame, i: usize) {
    if let Some(u) = &f.by[i] {
        if crate::director::director_query_alive(g, r, u) {
            let line = SMOKE_CALLS[if f.again { 1 } else { 0 }][i];
            crate::director::director_beat_radio(g, b, &u.callsign, line);
        }
    }
}
/// the helper's closure toss: his can leaves his hand
fn smoke_toss(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_team::TutTeamSmokeHelperFrame, i: usize) -> Result<(), String> {
    if let Some(u) = &f.by[i] {
        if crate::director::director_query_alive(g, r, u) {
            crate::director::director_beat_charge(g, r, b, &crate::director::Place::List(SMOKES[i]), u, Some(crate::throwables::ItemKind::Smoke), Some(0.8), Some(true))?;
        }
    }
    Ok(())
}

/// every can called by the man who throws it, a beat before it leaves his hand (the smoke beat's function* smoke)
fn tut_team_smoke_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, parent: &mut crate::script_tut_team::TutTeamSmokeFrame, f: &mut crate::script_tut_team::TutTeamSmokeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            let men = squad(g, r);
            let first = crate::director::director_beat_unit(g, r, "Red Two").or_else(|| men.first().cloned());
            let second = crate::director::director_beat_unit(g, r, "Blue One").or_else(|| men.get(1).cloned()).or_else(|| men.first().cloned());
            f.by = vec![first, second];
            smoke_call(g, r, b, f, 0);
            f.at = 1;
            Ok(Some(crate::director::director_beat_wait(g, r, SMOKE_APART, None)?))
        }
        1 => {
            smoke_call(g, r, b, f, 1);
            f.at = 2;
            Ok(Some(crate::director::director_beat_wait(g, r, SMOKE_CALLED - SMOKE_APART, None)?))
        }
        2 => {
            smoke_toss(g, r, b, f, 0)?;
            f.at = 3;
            Ok(Some(crate::director::director_beat_wait(g, r, SMOKE_APART, None)?))
        }
        3 => {
            smoke_toss(g, r, b, f, 1)?;
            f.at = 4;
            Ok(None)
        }
        _ => Ok(None),
    }
}
