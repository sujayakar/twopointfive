//! levels/tut-sky.script.ts translated: its define pass and a step function for each beat.
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec, format};

type Cond = crate::director::Cond;
type CondResult = crate::director::CondResult;
type UnitRef = crate::director::UnitRef;
type Place = crate::director::Place;
type Patch = crate::director::OrderPatch2;
type Upper = crate::character::UpperMode;
type EndState = crate::squad::EndGuardScriptState;
type Json = crate::director::Json;
type Step = Result<Option<Cond>, String>;

/// the roadblock's three posts and the names its men answer to
const POSTS: [(&str, V3); 3] = [("Tango 8", [34.0, 0.0, 21.6]), ("Tango 9", [34.9, 0.0, 23.6]), ("Tango 10", [32.4, 0.0, 23.2])];
/// where the roadblock's rounds go while the squad is behind its trucks: lo, hi, w, by (None: every post)
const ZONES: [(V3, V3, f64, Option<&[&str]>); 7] = [
    ([10.9, 0.4, 21.6], [11.9, 1.7, 22.9], 3.0, None),
    ([9.3, 0.4, 23.9], [10.5, 1.7, 25.2], 3.0, Some(&["Tango 8", "Tango 10"])),
    ([12.3, 0.05, 21.6], [16.0, 0.05, 23.0], 3.0, Some(&["Tango 9", "Tango 10"])),
    ([11.0, 0.05, 23.6], [15.5, 0.05, 24.4], 2.0, Some(&["Tango 8"])),
    ([3.0, 0.7, 20.5], [9.0, 1.6, 20.5], 3.0, None),
    ([7.5, 0.7, 25.7], [11.0, 1.1, 26.1], 3.0, Some(&["Tango 8"])),
    ([1.5, 2.05, 22.0], [1.5, 2.35, 25.0], 2.0, Some(&["Tango 8", "Tango 10"])),
];
const PINNED: [(&str, &str); 3] = [("Blue Two", "Taking fire! Stay down!"), ("Red Two", "Rounds coming past the trucks!"), ("Blue One", "We can not stay here!")];
const SMOKES: [V3; 2] = [[15.6, 0.0, 22.4], [17.8, 0.0, 24.2]];
const SMOKE_AGAIN: f64 = 27.0;
const SMOKE_CALLS: [[&str; 2]; 2] = [["Throwing smoke!", "Smoke out!"], ["More smoke!", "Smoke out!"]];
const SMOKE_CALLED: f64 = 0.7;
const SMOKE_APART: f64 = 0.45;
const AT_SMOKE: [V3; 5] = [[15.4, 0.9, 23.6], [14.0, 0.6, 24.6], [12.4, 1.0, 24.4], [16.8, 1.1, 24.9], [9.6, 0.9, 24.6]];
const SQUAD: [&str; 4] = ["Red One", "Red Two", "Blue One", "Blue Two"];
const YARD_X0: f64 = 4.4;
const YARD_X1: f64 = 20.5;
const YARD_Z0: f64 = 12.2;
const YARD_Z1: f64 = 19.7;
const PUSH_LATEST: f64 = 14.0;
const WEST: V3 = [4.5, 1.1, 23.6];
const BUNK_MEN: [&str; 3] = ["Tango 5", "Tango 6", "Tango 7"];
const BUNK_DOOR: V3 = [28.6, 1.1, 16.25];
const TORCH_BACK: f64 = 12.0;
const TORCH_MEN: [&str; 2] = ["Tango 5", "Tango 6"];
const TORCH_TO: [V3; 2] = [[23.2, 0.0, 13.4], [25.6, 0.0, 13.5]];
const TORCH_LOOK: V3 = [27.9, 1.2, 13.2];

/// which of the script's `stop` closures a rake or a harass was handed
const STOP_SMOKE_OR_BROKE: u16 = 0;
const STOP_THROUGH: u16 = 1;

/// ZONES as the frame holds them
fn zones() -> Vec<crate::script_tut_sky::TutSkyZone> {
    ZONES.iter().map(|&(lo, hi, w, by)| crate::script_tut_sky::TutSkyZone { lo, hi, w, by: by.map(|names| names.iter().map(|n| String::from(*n)).collect()) }).collect()
}

// ---- the verbs, short
fn wait(g: &mut Game, r: DirectorRunId, secs: f64) -> Result<Cond, String> { crate::director::director_beat_wait(g, r, secs, None) }
fn until(id: u32) -> Cond { crate::director::director_beat_until(crate::director::CondTest::Script { id }, None) }
/// c.any(a, c.until(id))
fn any_until(a: Cond, id: u32) -> Cond { crate::director::director_beat_any(vec![a, until(id)]) }
fn rand(g: &mut Game, r: DirectorRunId) -> f64 { crate::director::director_beat_rand(g, r) }
fn unit(g: &mut Game, r: DirectorRunId, callsign: &str) -> Option<UnitRef> { crate::director::director_beat_unit(g, r, callsign) }
fn alive(g: &mut Game, r: DirectorRunId, u: &UnitRef) -> bool { crate::director::director_query_alive(g, r, u) }
fn order(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef, o: Patch) -> Result<(), String> { crate::director::director_beat_order(g, r, b, u, &o) }
fn release(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef, state: EndState) -> Result<(), String> { crate::director::director_beat_release(g, r, b, u, Some(state)) }
fn say_radio(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef, line: &str) -> Result<(), String> { crate::director::director_beat_say(g, r, b, u, line, Some(true)) }
fn start(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, name: &str) { crate::director::director_beat_start(g, r, b, name) }
fn signal(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, name: &str) { crate::director::director_beat_signal(g, r, b, name, None) }
/// c.stimulus.burst of a point, with its cadence and spread
fn burst(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef, at: V3, rounds: f64, cadence: f64, spread: f64) -> Result<Cond, String> {
    crate::director::director_beat_burst(g, r, b, u, crate::director::RunBurstsItemAt::List(at), rounds, Some(cadence), Some(spread))
}
/// the run's bunkhouse door (define pass)
fn bunk_door(g: &mut Game, r: DirectorRunId) -> crate::director::DoorRef { tut_sky_vars(g, r).bunk_door.clone().unwrap() }

// ---- c.flag
fn flag_is_true(g: &mut Game, r: DirectorRunId, name: &str) -> bool {
    matches!(g.tab.director_run[r].flags.get(String::from(name)), Some(Some(Json::Bool(true))))
}
/// `c.flag.x` as a condition
fn flag_truthy(g: &mut Game, r: DirectorRunId, name: &str) -> bool {
    match g.tab.director_run[r].flags.get(String::from(name)) {
        None | Some(None) => false,
        Some(Some(Json::Bool(v))) => *v,
        Some(Some(Json::Num(n))) => js::truthy(*n),
        Some(Some(Json::Text(s))) => !s.is_empty(),
        Some(Some(_)) => true,
    }
}
fn flag_set(g: &mut Game, r: DirectorRunId, name: &str) { g.tab.director_run[r].flags.set(String::from(name), Some(Json::Bool(true))); }

// ---- the script's closures
/// names.map(c.unit).filter(alive) (torchMen, squad)
fn alive_of(g: &mut Game, r: DirectorRunId, names: &[&str]) -> Vec<UnitRef> {
    let mut out = Vec::new();
    for n in names { if let Some(u) = unit(g, r, n) { if alive(g, r, &u) { out.push(u); } } }
    out
}
/// (tut-sky.script.ts:55 block) the men of the roadblock on their feet, by name (spawned once)
fn block(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, spawn: bool) -> Result<Vec<UnitRef>, String> {
    let mut out = Vec::new();
    for (name, at) in POSTS {
        let mut u = unit(g, r, name);
        if u.is_none() && spawn {
            let (rifle, yaw) = { let v = tut_sky_vars(g, r); (v.rifle.clone().unwrap(), v.yaw) };
            u = Some(crate::director::director_beat_spawn(g, r, b, &rifle, &Place::List(at), Some(name), Some(yaw), Some(false))?);
        }
        if let Some(u) = u { if alive(g, r, &u) { out.push(u); } }
    }
    Ok(out)
}
fn torch_men(g: &mut Game, r: DirectorRunId) -> Vec<UnitRef> { alive_of(g, r, &TORCH_MEN) }
fn squad(g: &mut Game, r: DirectorRunId) -> Vec<UnitRef> { alive_of(g, r, &SQUAD) }
/// (tut-sky.script.ts:64 inBunk)
fn in_bunk(g: &mut Game, r: DirectorRunId, u: &UnitRef) -> Result<bool, String> {
    let p = crate::director::director_query_pos_unit(g, r, u)?;
    Ok(p[0] > 29.1 && p[0] < 35.9 && p[2] > 13.6 && p[2] < 19.9)
}
/// (tut-sky.script.ts:65 atBunk)
fn at_bunk(g: &mut Game, r: DirectorRunId, u: &UnitRef) -> Result<bool, String> {
    let p = crate::director::director_query_pos_unit(g, r, u)?;
    Ok(m::hyp2(p[0] - BUNK_DOOR[0], p[2] - BUNK_DOOR[2]) < 3.6)
}
/// (tut-sky.script.ts:66 inYard)
fn in_yard(g: &mut Game, r: DirectorRunId, u: &UnitRef) -> Result<bool, String> {
    let p = crate::director::director_query_pos_unit(g, r, u)?;
    Ok(p[0] > YARD_X0 && p[0] < YARD_X1 && p[2] > YARD_Z0 && p[2] < YARD_Z1)
}
/// (tut-sky.script.ts:67 pick)
fn pick<T: Clone>(g: &mut Game, r: DirectorRunId, a: &[T]) -> T {
    let len = a.len() as f64;
    let i = m::min(len - 1.0, m::floor(rand(g, r) * len));
    a[i as usize].clone()
}
/// squad(c).some(u => atBunk(u) || inBunk(u)): lines 108, 227, 246
fn squad_at_house(g: &mut Game, r: DirectorRunId) -> Result<bool, String> {
    for u in squad(g, r) { if at_bunk(g, r, &u)? || in_bunk(g, r, &u)? { return Ok(true); } }
    Ok(false)
}
/// `c.unit(name) ?? squad(c)[0]`
fn unit_or_first(g: &mut Game, r: DirectorRunId, name: &str) -> Option<UnitRef> {
    match unit(g, r, name) { Some(u) => Some(u), None => squad(g, r).into_iter().next() }
}
/// `const u = c.unit(name) ?? squad(c)[0]; if (u) c.say(u, line, { radio: true })`
fn say_first(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, name: &str, line: &str) -> Result<(), String> {
    if let Some(u) = unit_or_first(g, r, name) { say_radio(g, r, b, &u, line)?; }
    Ok(())
}
/// a rake's or a harass's `stop` (lines 125, 167)
fn stop(g: &mut Game, r: DirectorRunId, which: u16) -> bool {
    match which {
        STOP_SMOKE_OR_BROKE => flag_is_true(g, r, "smoke") || flag_is_true(g, r, "broke"),
        _ => flag_is_true(g, r, "through"),
    }
}
/// (tut-sky.script.ts:70 mix)
fn mix(g: &mut Game, r: DirectorRunId, a: f64, b: f64) -> f64 { a + rand(g, r) * (b - a) }

/// (levels/tut-sky.script.ts: `script(D)`) the define pass.
pub fn tut_sky_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_sky::TutSkyVars, String> {
    let rifle = crate::director::manifest_loadout(m, "roadblock", crate::director::LoadoutSpec { team: crate::teams::Team::Hostile, weapon: Some(crate::weapons::WeaponId::Akms), light: Some(false), ..Default::default() });
    let yaw = -core::f64::consts::PI / 2.0;
    m.systems.set("qrf", false);
    let bunk_door = crate::director::manifest_door(m, "bunk");
    crate::director::manifest_program(m, "tutorial", &mut |p: &mut crate::director::ProgramDef| -> Result<(), String> {
        crate::director::program_on_start(p, "hold");
        for (sig, beat) in [("lesson:ambush", "ambush"), ("lesson:smoke", "smoke"), ("lesson:break", "break"), ("lesson:indoors", "indoors"), ("lesson:wall", "wall"), ("lesson:dark", "dark"), ("lesson:nv", "nv"), ("lesson:torches", "torches"), ("lesson:out", "out")] {
            crate::director::program_on_signal(p, sig, beat);
        }
        for beat in ["hold", "ambush", "pinned", "smoke", "break", "ordered", "blind", "push", "indoors", "wall", "dark", "nv", "torches", "inside", "out"] {
            crate::director::program_beat(p, beat, &crate::scene_level::LevelScriptFn::TutSky, None)?;
        }
        Ok(())
    }, None)?;
    Ok(crate::script_tut_sky::TutSkyVars { rifle: Some(rifle), yaw, bunk_door: Some(bunk_door) })
}

/// (tut-sky.script.ts:107 'hold') the bunkhouse keeps to itself until the squad comes to its door
pub fn tut_sky_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            if squad_at_house(g, r)? { return Ok(None); }
            let mut held = Vec::new();
            for n in BUNK_MEN { if let Some(u) = unit(g, r, n) { if alive(g, r, &u) && in_bunk(g, r, &u)? { held.push(u); } } }
            for u in &held {
                order(g, r, b, u, Patch { goal: Some(None), aim_at: Some(Some(Place::List(BUNK_DOOR))), upper: Some(Some(Upper::Aim)), hot: Some(Some(false)), ..Default::default() })?;
            }
            f.held = held;
            f.at = 1;
            Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))))
        }
        1 => { f.at = 2; Ok(Some(until(0))) }
        _ => {
            let held = core::mem::take(&mut f.held);
            for u in &held { if alive(g, r, u) && in_bunk(g, r, u)? { release(g, r, b, u, EndState::Alert)?; } }
            Ok(None)
        }
    }
}
/// (tut-sky.script.ts:112) 0: a man of the squad at the door
pub fn tut_sky_hold_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyHoldFrame, id: u32) -> Result<bool, String> {
    for u in squad(g, r) { if at_bunk(g, r, &u)? { return Ok(true); } }
    Ok(false)
}

/// (tut-sky.script.ts:119 'ambush') the roadblock opens up on the two trucks and keeps it up
pub fn tut_sky_ambush_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyAmbushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        for u in block(g, r, b, true)? {
            order(g, r, b, &u, Patch { goal: Some(None), aim_at: Some(Some(Place::List(WEST))), upper: Some(Some(Upper::Aim)), crouch: Some(Some(false)), hot: Some(Some(true)), flinch: Some(Some(false)), ..Default::default() })?;
        }
        for u in squad(g, r) {
            order(g, r, b, &u, Patch { goal: Some(None), crouch: Some(Some(true)), hot: Some(Some(false)), flinch: Some(Some(true)), ..Default::default() })?;
        }
        f.at = 1;
        return Ok(Some(wait(g, r, 0.6)?));
    }
    if f.at == 1 {
        if let Some(red) = unit(g, r, "Red One") { say_radio(g, r, b, &red, "Contact front! Roadblock!")?; }
        start(g, r, b, "pinned");
        f.sub_rake = Some(Box::new(crate::script_tut_sky::TutSkyRakeHelperFrame {
            zones: zones(),
            stop: STOP_SMOKE_OR_BROKE,
            o: Some(crate::script_tut_sky::TutSkyRakeHelperO { rounds: [5.0, 9.0], gap: [0.2, 0.7], spread: 0.014 }),
            ..Default::default()
        }));
        f.at = 2;
    }
    let mut sub = f.sub_rake.take().unwrap();
    match tut_sky_rake_helper_step(g, r, b, &mut sub, got)? {
        Some(c) => { f.sub_rake = Some(sub); Ok(Some(c)) }
        None => Ok(None),
    }
}

/// (tut-sky.script.ts:127 'pinned') what the men behind the trucks call while they are pinned
pub fn tut_sky_pinned_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPinnedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => { f.k = 0.0; f.at = 1; }
            1 => {
                if flag_truthy(g, r, "smoke") || flag_truthy(g, r, "broke") { return Ok(None); }
                f.at = 2;
                let secs = 5.5 + rand(g, r) * 2.0;
                let w = wait(g, r, secs)?;
                return Ok(Some(any_until(w, 0)));
            }
            _ => {
                if flag_truthy(g, r, "smoke") || flag_truthy(g, r, "broke") { return Ok(None); }
                let (who, line) = PINNED[js::rem(f.k, PINNED.len() as f64) as usize];
                if let Some(u) = unit(g, r, who) { if alive(g, r, &u) { say_radio(g, r, b, &u, line)?; } }
                f.k += 1.0;
                f.at = 1;
            }
        }
    }
}
/// (tut-sky.script.ts:129) 0: the smoke is out or the squad has broken
pub fn tut_sky_pinned_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPinnedFrame, id: u32) -> Result<bool, String> {
    Ok(flag_is_true(g, r, "smoke") || flag_is_true(g, r, "broke"))
}

/// the frame of a `yield* smoke(again)`, not begun
fn smoke_sub(again: bool) -> Option<Box<crate::script_tut_sky::TutSkySmokeHelperFrame>> {
    Some(Box::new(crate::script_tut_sky::TutSkySmokeHelperFrame { again, ..Default::default() }))
}
/// one resume of the running `yield* smoke`: its yield to pass up, or None when it has returned (the field cleared)
fn smoke_sub_step(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkySmokeFrame, got: &CondResult) -> Step {
    let mut sub = f.sub_smoke.take().unwrap();
    let out = tut_sky_smoke_helper_step(g, r, b, f, &mut sub, got)?;
    if out.is_some() { f.sub_smoke = Some(sub); }
    Ok(out)
}
/// (tut-sky.script.ts:142 'smoke') the squad's own smoke, called by the man who throws it, and more while no order comes
pub fn tut_sky_smoke_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkySmokeFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                if flag_truthy(g, r, "smoke") { return Ok(None); }
                flag_set(g, r, "smoke");
                start(g, r, b, "ordered");
                f.at = 1;
                return Ok(Some(wait(g, r, 0.4)?));
            }
            1 => {
                start(g, r, b, "blind");
                f.sub_smoke = smoke_sub(false);
                f.at = 2;
            }
            2 => {
                if let Some(c) = smoke_sub_step(g, r, b, f, got)? { return Ok(Some(c)); }
                f.at = 3;
            }
            3 => {
                if flag_truthy(g, r, "broke") { return Ok(None); }
                f.at = 4;
                let w = wait(g, r, SMOKE_AGAIN - SMOKE_CALLED - SMOKE_APART)?;
                return Ok(Some(any_until(w, 0)));
            }
            4 => {
                if flag_truthy(g, r, "broke") { f.at = 3; }
                else { f.sub_smoke = smoke_sub(true); f.at = 5; }
            }
            _ => {
                if let Some(c) = smoke_sub_step(g, r, b, f, got)? { return Ok(Some(c)); }
                f.at = 3;
            }
        }
    }
}
/// (tut-sky.script.ts:153) 0: the squad has broken
pub fn tut_sky_smoke_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkySmokeFrame, id: u32) -> Result<bool, String> {
    Ok(flag_is_true(g, r, "broke"))
}

/// (tut-sky.script.ts:155 'break') the order has come: the squad runs for the yard
pub fn tut_sky_break_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyBreakFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            if !flag_truthy(g, r, "smoke") { start(g, r, b, "smoke"); }
            f.at = 1;
            Ok(Some(until(0)))
        }
        1 => { f.at = 2; Ok(Some(wait(g, r, 0.25)?)) }
        2 => {
            for u in squad(g, r) { release(g, r, b, &u, EndState::Patrol)?; }
            f.at = 3;
            let w = wait(g, r, 2.2)?;
            Ok(Some(any_until(w, 1)))
        }
        _ => {
            flag_set(g, r, "through");
            start(g, r, b, "push");
            Ok(None)
        }
    }
}
/// (tut-sky.script.ts:157, :160) 0: the squad has broken · 1: all of it is in the yard
pub fn tut_sky_break_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyBreakFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => Ok(flag_is_true(g, r, "broke")),
        _ => {
            for u in squad(g, r) { if !in_yard(g, r, &u)? { return Ok(false); } }
            Ok(true)
        }
    }
}

/// (tut-sky.script.ts:164 'ordered') the first order given breaks them out
pub fn tut_sky_ordered_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyOrderedFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        f.at = 1;
        return Ok(Some(crate::director::director_beat_on(g, r, crate::director::CondArm3Kind::Signal, None, Some(crate::director::EventFilter::OrderSignal))));
    }
    flag_set(g, r, "broke");
    Ok(None)
}

/// (tut-sky.script.ts:167 'blind') the roadblock fires blind into the smoke until the squad is through
pub fn tut_sky_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        f.sub_harass = Some(Box::new(crate::script_tut_sky::TutSkyHarassHelperFrame {
            marks: AT_SMOKE.to_vec(),
            stop: STOP_THROUGH,
            o: Some(crate::script_tut_sky::TutSkyHarassHelperO { rounds: [3.0, 5.0], gap: [0.5, 1.2], spread: 0.05 }),
            ..Default::default()
        }));
        f.at = 1;
    }
    let mut sub = f.sub_harass.take().unwrap();
    match tut_sky_harass_helper_step(g, r, b, &mut sub, got)? {
        Some(c) => { f.sub_harass = Some(sub); Ok(Some(c)) }
        None => Ok(None),
    }
}

/// (tut-sky.script.ts:173 'push') the roadblock's men come on for the gate, one after another; whoever is left is walked in again, further each time
pub fn tut_sky_push_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPushFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                f.at = 1;
                let w = wait(g, r, PUSH_LATEST)?;
                return Ok(Some(any_until(w, 0)));
            }
            1 => {
                say_first(g, r, b, "Red One", "They're coming for the gate. We've got it.")?;
                f.k = 0.0;
                f.at = 2;
            }
            // the outer `for`'s test
            2 => {
                if block(g, r, b, false)?.is_empty() {
                    signal(g, r, b, "done:roadblock");
                    return Ok(None);
                }
                f.left = block(g, r, b, false)?;
                f.i = 0.0;
                f.at = 3;
            }
            // the inner `for`'s head: the man f.i, or on past the last
            3 => {
                let i = f.i as usize;
                if i >= f.left.len() {
                    f.at = 5;
                    let secs = if js::truthy(f.k) { 5.0 } else { 7.0 };
                    let w = wait(g, r, secs)?;
                    return Ok(Some(any_until(w, 2)));
                }
                let u = f.left[i].clone();
                let spread = m::min(1.0, f.k);
                let goal: V3 = [14.6 + f.i * 0.7 - spread * 0.9, 0.0, 19.2 - f.i * 0.5 - spread * 0.6];
                order(g, r, b, &u, Patch { goal: Some(Some(Place::List(goal))), speed: Some(Some(3.4)), aim_at: Some(Some(Place::List([11.4, 1.1, 16.4]))), upper: Some(Some(Upper::Aim)), hot: Some(Some(true)), direct: Some(Some(true)), ..Default::default() })?;
                if i + 1 < f.left.len() {
                    f.at = 4;
                    let w = wait(g, r, 0.7)?;
                    return Ok(Some(any_until(w, 1)));
                }
                f.i += 1.0;
            }
            4 => { f.i += 1.0; f.at = 3; }
            5 => {
                for u in block(g, r, b, false)? { release(g, r, b, &u, EndState::Alert)?; }
                f.at = 6;
                let w = wait(g, r, 4.0)?;
                return Ok(Some(any_until(w, 3)));
            }
            _ => { f.k += 1.0; f.at = 2; }
        }
    }
}
/// (tut-sky.script.ts:174, :180, :182, :184) 0: three of the squad (or all that stand) in the yard · 1, 3: the roadblock is down · 2: each of `left` down or there
pub fn tut_sky_push_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyPushFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => {
            let men = squad(g, r);
            let mut n = 0usize;
            for u in &men { if in_yard(g, r, u)? { n += 1; } }
            Ok(n >= core::cmp::min(3, men.len()))
        }
        2 => {
            for u in &f.left {
                if !(!alive(g, r, u) || crate::director::director_query_arrived(g, r, u)?) { return Ok(false); }
            }
            Ok(true)
        }
        _ => Ok(block(g, r, b, false)?.is_empty()),
    }
}

/// (tut-sky.script.ts:190 'indoors') the road is quiet: whoever of the roadblock is left is held where he stands
pub fn tut_sky_indoors_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyIndoorsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        flag_set(g, r, "broke");
        flag_set(g, r, "smoke");
        flag_set(g, r, "through");
        for u in block(g, r, b, false)? {
            order(g, r, b, &u, Patch { goal: Some(None), aim_at: Some(Some(Place::List([16.5, 1.1, 21.4]))), upper: Some(Some(Upper::Aim)), hot: Some(Some(false)), ..Default::default() })?;
        }
        f.at = 1;
        return Ok(Some(wait(g, r, 0.1)?));
    }
    signal(g, r, b, "done:roadblock");
    Ok(None)
}

/// (tut-sky.script.ts:198 'wall')
pub fn tut_sky_wall_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyWallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        flag_set(g, r, "broke");
        f.at = 1;
        return Ok(Some(wait(g, r, 0.8)?));
    }
    say_first(g, r, b, "Blue One", "Store's locked. We need a charge on the wall.")?;
    Ok(None)
}

/// (tut-sky.script.ts:203 'dark')
pub fn tut_sky_dark_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyDarkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        flag_set(g, r, "broke");
        f.at = 1;
        return Ok(Some(wait(g, r, 0.6)?));
    }
    say_first(g, r, b, "Red One", "There's a breaker box on the yard wall. We can kill the lights.")?;
    Ok(None)
}

/// (tut-sky.script.ts:208 'nv')
pub fn tut_sky_nv_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyNvFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        flag_set(g, r, "broke");
        f.at = 1;
        return Ok(Some(wait(g, r, 0.4)?));
    }
    say_first(g, r, b, "Red Two", "Lights are out. Goggles on.")?;
    Ok(None)
}

/// (tut-sky.script.ts:228 down)
fn down(g: &mut Game, r: DirectorRunId, men: &[UnitRef]) -> bool {
    for u in men { if !alive(g, r, u) { return true; } }
    false
}
/// (tut-sky.script.ts:228 gone)
fn gone(g: &mut Game, r: DirectorRunId, men: &[UnitRef]) -> bool {
    for u in men { if alive(g, r, u) { return false; } }
    true
}
/// (tut-sky.script.ts:224 'torches') the bunkhouse sends two out behind their weapon lights for the mains box
pub fn tut_sky_torches_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyTorchesFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                flag_set(g, r, "broke");
                let mut men = Vec::new();
                for u in torch_men(g, r) { if in_bunk(g, r, &u)? { men.push(u); } }
                if men.is_empty() || squad_at_house(g, r)? {
                    signal(g, r, b, "done:torches");
                    return Ok(None);
                }
                let mut home = Vec::new();
                for u in &men {
                    let p = crate::director::director_query_pos_unit(g, r, u)?;
                    home.push(Some(Json::Array(vec![Some(Json::Text(u.callsign.clone())), Some(Json::Num(p[0])), Some(Json::Num(p[1])), Some(Json::Num(p[2]))])));
                }
                g.tab.director_run[r].flags.set(String::from("torchHome"), Some(Json::Array(home)));
                let line = if men.len() > 1 { "Power's out! Two of us are going to the breaker box!" } else { "Power's out! I'm going to the breaker box!" };
                say_radio(g, r, b, &men[0], line)?;
                for u in &men {
                    order(g, r, b, u, Patch { goal: Some(None), aim_at: Some(Some(Place::List(BUNK_DOOR))), upper: Some(Some(Upper::Aim)), light: Some(Some(true)), hot: Some(Some(false)), ..Default::default() })?;
                }
                f.men = men;
                f.at = 1;
                return Ok(Some(wait(g, r, 1.3)?));
            }
            1 => {
                let door = bunk_door(g, r);
                crate::director::director_beat_door_open(g, &door, None, Some(true))?;
                f.i = 0.0;
                f.at = 2;
            }
            // the `for`'s head: the man f.i, or on past the last
            2 => {
                let i = f.i as usize;
                if i >= f.men.len() {
                    start(g, r, b, "inside");
                    f.at = 4;
                    let w = wait(g, r, 12.0)?;
                    return Ok(Some(any_until(w, 0)));
                }
                let u = f.men[i].clone();
                order(g, r, b, &u, Patch { goal: Some(Some(Place::List(TORCH_TO[i]))), speed: Some(Some(1.6)), aim_at: Some(Some(Place::List(TORCH_LOOK))), upper: Some(Some(Upper::Aim)), light: Some(Some(true)), crouch: Some(Some(false)), hot: Some(Some(false)), flinch: Some(Some(true)), ..Default::default() })?;
                f.at = 3;
                return Ok(Some(wait(g, r, 0.9)?));
            }
            3 => { f.i += 1.0; f.at = 2; }
            4 => {
                if !down(g, r, &f.men) { for u in &f.men { release(g, r, b, u, EndState::Alert)?; } }
                f.at = 5;
                let w = wait(g, r, 7.0)?;
                return Ok(Some(any_until(w, 1)));
            }
            5 => {
                f.at = 6;
                return Ok(Some(wait(g, r, 0.8)?));
            }
            _ => {
                if let Some(by) = unit_or_first(g, r, "Blue One") {
                    if gone(g, r, &f.men) {
                        let line = if f.men.len() > 1 { "Two down. They never saw us." } else { "He never saw us." };
                        say_radio(g, r, b, &by, line)?;
                    }
                }
                signal(g, r, b, "done:torches");
                return Ok(None);
            }
        }
    }
}
/// (tut-sky.script.ts:236, :238) 0: one of them down, or both where they were going · 1: both down
pub fn tut_sky_torches_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyTorchesFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => {
            if down(g, r, &f.men) { return Ok(true); }
            for u in &f.men { if !crate::director::director_query_arrived(g, r, u)? { return Ok(false); } }
            Ok(true)
        }
        _ => Ok(gone(g, r, &f.men)),
    }
}

/// `c.flag.torchHome ?? []` as its rows (line 245). A row of another shape is what the TypeScript's destructuring would throw on
fn torch_home(g: &mut Game, r: DirectorRunId) -> Result<Vec<(String, V3)>, String> {
    let rows = match g.tab.director_run[r].flags.get(String::from("torchHome")) {
        None | Some(None) => return Ok(Vec::new()),
        Some(Some(Json::Array(rows))) => rows,
        Some(Some(_)) => return Err(String::from("flag torchHome is not a list")),
    };
    let mut out = Vec::new();
    for row in rows {
        match row {
            Some(Json::Array(c)) => match c.as_slice() {
                [Some(Json::Text(n)), Some(Json::Num(x)), Some(Json::Num(y)), Some(Json::Num(z)), ..] => out.push((n.clone(), [*x, *y, *z])),
                _ => return Err(String::from("a row of flag torchHome is not [name, x, y, z]")),
            },
            _ => return Err(String::from("a row of flag torchHome is not [name, x, y, z]")),
        }
    }
    Ok(out)
}
/// (tut-sky.script.ts:246 taken) the squad has come to the bunkhouse
fn inside_taken(g: &mut Game, r: DirectorRunId) -> Result<bool, String> { squad_at_house(g, r) }
/// (tut-sky.script.ts:247 shut) the last room is a shut door again
fn inside_shut(g: &mut Game, r: DirectorRunId, f: &crate::script_tut_sky::TutSkyInsideFrame) -> Result<(), String> {
    for m in &f.men {
        let u = m.u.as_ref().unwrap();
        if !(!alive(g, r, u) || in_bunk(g, r, u)?) { return Ok(()); }
    }
    if !inside_taken(g, r)? {
        let door = bunk_door(g, r);
        crate::director::director_beat_door_shut(g, &door)?;
    }
    Ok(())
}
/// (tut-sky.script.ts:253 free) whoever is not run back is let go here
fn inside_free(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_sky::TutSkyInsideFrame) -> Result<(), String> {
    for m in &f.left { release(g, r, b, m.u.as_ref().unwrap(), EndState::Alert)?; }
    Ok(())
}
/// (tut-sky.script.ts:244 'inside') the one left when his mate goes down runs back in and is let go inside his own room
pub fn tut_sky_inside_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyInsideFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                let mut men = Vec::new();
                for (n, to) in torch_home(g, r)? {
                    if let Some(u) = unit(g, r, &n) { men.push(crate::script_tut_sky::TutSkyInsideMen { u: Some(u), to }); }
                }
                f.men = men;
                f.at = 1;
                return Ok(Some(until(0)));
            }
            1 => {
                let mut left = Vec::new();
                for m in &f.men {
                    if alive(g, r, m.u.as_ref().unwrap()) { left.push(crate::script_tut_sky::TutSkyInsideLeft { u: m.u.clone(), to: m.to }); }
                }
                f.left = left;
                let mut all_in = true;
                for m in &f.left { if !in_bunk(g, r, m.u.as_ref().unwrap())? { all_in = false; break; } }
                if all_in {   // (of nobody too: `!left.length ||`)
                    inside_free(g, r, b, f)?;
                    inside_shut(g, r, f)?;
                    return Ok(None);
                }
                if inside_taken(g, r)? {
                    inside_free(g, r, b, f)?;
                    crate::director::director_beat_note(g, b, "the bunkhouse is a fight already: the man left in the yard is the game's own where he stands", None);
                    return Ok(None);
                }
                let line = if f.left.len() > 1 { "Man down! Get back inside!" } else { "Man down! I'm going back inside!" };
                say_radio(g, r, b, f.left[0].u.as_ref().unwrap(), line)?;
                for m in &f.left {
                    order(g, r, b, m.u.as_ref().unwrap(), Patch { goal: Some(Some(Place::List(m.to))), speed: Some(Some(3.4)), aim_at: Some(None), upper: Some(Some(Upper::Aim)), light: Some(Some(true)), crouch: Some(Some(false)), hot: Some(Some(false)), flinch: Some(Some(true)), ..Default::default() })?;
                }
                let mut pending = JsSet::new();
                for i in 0..f.left.len() { pending.add(crate::ObjKey(i as u32)); }   // (a man of `left`, by his place in it: the Set of line 258 holds the objects)
                f.pending = Some(pending);
                f.t = 0.0;
                f.at = 2;
            }
            // the `for`'s test
            2 => {
                if f.t < TORCH_BACK && !f.pending.as_ref().unwrap().is_empty() {
                    f.at = 3;
                    return Ok(Some(wait(g, r, 0.2)?));
                }
                for k in f.pending.as_ref().unwrap().keys() {
                    let u = f.left[k.0 as usize].u.clone().unwrap();
                    release(g, r, b, &u, EndState::Alert)?;
                    crate::director::director_beat_note(g, b, &format!("{} was not back inside in {} s: let go where he is", u.callsign, js::num_str(TORCH_BACK)), None);
                }
                inside_shut(g, r, f)?;
                return Ok(None);
            }
            _ => {
                for k in f.pending.as_ref().unwrap().keys() {   // (a copy: the body removes only the entry it stands on)
                    let u = f.left[k.0 as usize].u.clone().unwrap();
                    if !alive(g, r, &u) { f.pending.as_mut().unwrap().delete(k); }
                    else if crate::director::director_query_arrived(g, r, &u)? {
                        f.pending.as_mut().unwrap().delete(k);
                        release(g, r, b, &u, EndState::Alert)?;
                    }
                }
                f.t += 0.2;
                f.at = 2;
            }
        }
    }
}
/// (tut-sky.script.ts:248) 0: one of the men down
pub fn tut_sky_inside_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyInsideFrame, id: u32) -> Result<bool, String> {
    for m in &f.men { if !alive(g, r, m.u.as_ref().unwrap()) { return Ok(true); } }
    Ok(false)
}

/// (tut-sky.script.ts:266 'out')
pub fn tut_sky_out_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyOutFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        flag_set(g, r, "broke");
        f.at = 1;
        return Ok(Some(wait(g, r, 0.4)?));
    }
    say_first(g, r, b, "Red One", "House is clear.")?;
    Ok(None)
}

/// (tut-sky.script.ts:69 rake) the roadblock fires in turns, each burst walked over one of `zones`, until `stop`
pub fn tut_sky_rake_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyRakeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            // the top: line 70's `total`, once
            0 => {
                f.total = f.zones.iter().fold(0.0, |a, z| a + z.w);
                f.at = 1;
            }
            // the outer `while`
            1 => {
                if stop(g, r, f.stop) { return Ok(None); }
                f.men = block(g, r, b, false)?;
                if f.men.is_empty() { return Ok(None); }
                f.r = rand(g, r) * f.total;
                let mut chosen = f.zones[0].clone();
                for z in &f.zones { f.r -= z.w; if f.r <= 0.0 { chosen = z.clone(); break; } }
                f.can = f.men.iter().filter(|m| match &chosen.by { None => true, Some(by) => by.iter().any(|n| *n == m.callsign) }).cloned().collect();
                f.z = Some(chosen);
                if f.can.is_empty() {
                    f.at = 2;
                    return Ok(Some(wait(g, r, 0.05)?));
                }
                let u = pick(g, r, &f.can);
                f.heavy = u.callsign == "Tango 9";
                let rounds = f.o.as_ref().unwrap().rounds;
                f.left = m::round(mix(g, r, rounds[0], rounds[1])) * (if f.heavy { 2.0 } else { 1.0 });
                f.u = Some(u);
                f.at = 3;
            }
            2 => { f.at = 1; }   // (`continue`)
            // the inner `while`
            3 => {
                let u = f.u.clone().unwrap();
                let o = f.o.clone().unwrap();
                if f.left > 0.0 && !stop(g, r, f.stop) && alive(g, r, &u) {
                    let (lo, hi) = { let z = f.z.as_ref().unwrap(); (z.lo, z.hi) };
                    let a0 = mix(g, r, lo[0], hi[0]);
                    let a1 = mix(g, r, lo[1], hi[1]);
                    let a2 = mix(g, r, lo[2], hi[2]);
                    f.at_2 = [a0, a1, a2];
                    f.n = m::min(f.left, if rand(g, r) < 0.5 { 2.0 } else { 3.0 });
                    f.left -= f.n;
                    order(g, r, b, &u, Patch { goal: Some(None), aim_at: Some(Some(Place::List(f.at_2))), upper: Some(Some(Upper::Aim)), hot: Some(Some(true)), ..Default::default() })?;
                    f.at = 4;
                    return Ok(Some(burst(g, r, b, &u, f.at_2, f.n, if f.heavy { 0.085 } else { 0.11 }, o.spread)?));
                }
                f.at = 6;
                let secs = mix(g, r, o.gap[0], o.gap[1]);
                return Ok(Some(wait(g, r, secs)?));
            }
            4 => {
                f.at = 5;
                let secs = mix(g, r, 0.05, 0.16);
                return Ok(Some(wait(g, r, secs)?));
            }
            5 => { f.at = 3; }
            _ => { f.at = 1; }
        }
    }
}

/// (tut-sky.script.ts:87 harass) the roadblock fires in turns at `marks` until `stop`
pub fn tut_sky_harass_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_sky::TutSkyHarassHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let o = f.o.clone().unwrap();
    match f.at {
        // the `while`
        0 => {
            if stop(g, r, f.stop) { return Ok(None); }
            f.men = block(g, r, b, false)?;
            if f.men.is_empty() { return Ok(None); }
            f.at_2 = pick(g, r, &f.marks);
            f.north = f.men.iter().find(|m| m.callsign == "Tango 8").cloned();
            let u = match (&f.north, f.at_2[2] > 24.0) {
                (Some(north), true) => north.clone(),
                _ => pick(g, r, &f.men),
            };
            f.heavy = u.callsign == "Tango 9";
            order(g, r, b, &u, Patch { goal: Some(None), aim_at: Some(Some(Place::List(f.at_2))), upper: Some(Some(Upper::Aim)), hot: Some(Some(true)), ..Default::default() })?;
            f.n = m::round(o.rounds[0] + rand(g, r) * (o.rounds[1] - o.rounds[0])) * (if f.heavy { 2.0 } else { 1.0 });
            f.at = 1;
            let c = burst(g, r, b, &u, f.at_2, f.n, if f.heavy { 0.085 } else { 0.11 }, o.spread)?;
            f.u = Some(u);
            Ok(Some(c))
        }
        1 => {
            f.at = 2;
            let secs = o.gap[0] + rand(g, r) * (o.gap[1] - o.gap[0]);
            Ok(Some(wait(g, r, secs)?))
        }
        // back to the `while`, in this same resume
        _ => {
            f.at = 0;
            tut_sky_harass_helper_step(g, r, b, f, got)
        }
    }
}

/// (tut-sky.script.ts:146 call) a man calls his can: `by[i]`, if there is one
fn smoke_call(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_sky::TutSkySmokeHelperFrame, i: usize) {
    if let Some(u) = &f.by[i] {
        if alive(g, r, u) {
            crate::director::director_beat_radio(g, b, &u.callsign, SMOKE_CALLS[if f.again { 1 } else { 0 }][i]);
        }
    }
}
/// (tut-sky.script.ts:147 toss) he is seen to throw it
fn smoke_toss(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_sky::TutSkySmokeHelperFrame, i: usize) -> Result<(), String> {
    if let Some(u) = &f.by[i] {
        if alive(g, r, u) {
            crate::director::director_beat_charge(g, r, b, &Place::List(SMOKES[i]), u, Some(crate::throwables::ItemKind::Smoke), Some(0.8), Some(true))?;
        }
    }
    Ok(())
}
/// (tut-sky.script.ts:144 smoke) two men call and throw a can each
pub fn tut_sky_smoke_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, parent: &mut crate::script_tut_sky::TutSkySmokeFrame, f: &mut crate::script_tut_sky::TutSkySmokeHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            let men = squad(g, r);
            let by0 = match unit(g, r, "Red Two") { Some(u) => Some(u), None => men.get(0).cloned() };
            let by1 = match unit(g, r, "Blue One") { Some(u) => Some(u), None => men.get(1).or(men.get(0)).cloned() };
            f.by = vec![by0, by1];
            smoke_call(g, r, b, f, 0);
            f.at = 1;
            Ok(Some(wait(g, r, SMOKE_APART)?))
        }
        1 => {
            smoke_call(g, r, b, f, 1);
            f.at = 2;
            Ok(Some(wait(g, r, SMOKE_CALLED - SMOKE_APART)?))
        }
        2 => {
            smoke_toss(g, r, b, f, 0)?;
            f.at = 3;
            Ok(Some(wait(g, r, SMOKE_APART)?))
        }
        _ => {
            smoke_toss(g, r, b, f, 1)?;
            Ok(None)
        }
    }
}
