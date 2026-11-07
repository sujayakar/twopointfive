//! levels/tut-solo.script.ts translated: its define pass and a step function for each beat.
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec, format};

use crate::director::{
    director_beat_all, director_beat_any, director_beat_burst, director_beat_door_lock, director_beat_door_open, director_beat_frames,
    director_beat_on, director_beat_order, director_beat_rand, director_beat_release, director_beat_signal, director_beat_spawn,
    director_beat_unit, director_beat_until, director_beat_wait, director_inside_volume, director_query_alive, director_query_arrived,
    director_query_door, director_query_pos_unit,
};
use crate::director::{Cond, CondArm3Kind, CondResult, CondTest, DirectorInsideVolumeWho, EventFilter, Json, LoadoutSpec, OrderPatch2, Place, RunBurstsItemAt, UnitRef, VolumeRef};
use crate::squad::EndGuardScriptState;

const VAN_FLANK: [V3; 4] = [[5.5, 1.35, 13.65], [5.5, 0.85, 14.0], [5.5, 1.5, 14.35], [5.5, 1.1, 13.8]];
const TRUCK: [V3; 4] = [[12.4, 1.45, 18.6], [11.6, 1.5, 19.2], [13.2, 1.4, 18.3], [12.0, 1.55, 18.9]];
const BAGS_TOP: [V3; 8] = [
    [16.6, 1.45, 13.6], [17.2, 1.45, 13.6], [17.8, 1.45, 13.6], [18.4, 1.45, 13.6],
    [19.0, 1.45, 13.6], [19.6, 1.45, 13.6], [20.2, 1.45, 13.6], [20.8, 1.45, 13.6],
];
const GAP_FLOOR: [V3; 8] = [[21.55, 0.5, 13.75], [21.95, 0.75, 13.8], [21.75, 0.45, 13.75], [22.1, 0.7, 13.85], [21.65, 0.8, 13.7], [22.05, 0.5, 13.8], [21.85, 0.65, 13.75], [21.6, 0.6, 13.75]];
const WAS: [V3; 3] = [[17.2, 1.3, 13.2], [18.4, 1.3, 13.4], [16.8, 1.42, 13.6]];
const LAST_IN: [V3; 2] = [[37.4, 0.0, 14.0], [38.4, 0.0, 14.5]];
const LAST_MEN: [V3; 2] = [[25.7, 0.0, 13.98], [26.5, 0.0, 14.5]];
const ARCH_HEAD: [[V3; 3]; 2] = [[[16.0, 2.2, 12.5], [16.0, 2.1, 12.0], [16.0, 2.25, 12.3]], [[16.0, 2.2, 16.0], [16.0, 2.1, 16.5], [16.0, 2.25, 16.2]]];
const SHOP_DOORWAY: V3 = [25.4, 1.2, 14.25];
const PART_DOORWAY: V3 = [30.5, 1.2, 14.25];

/// `list[i % list.length]`, i a count (tut-solo.script.ts:68, :91, :94, :144)
fn pick(list: &[V3], i: f64) -> V3 { list[js::rem(i, list.len() as f64) as usize] }

/// `c.flag[name]` as a condition, or `!!q.flag(name)` (tut-solo.script.ts:66, :143, :146)
fn flag_truthy(g: &Game, r: DirectorRunId, name: &str) -> bool {
    match g.tab.director_run[r].flags.get(String::from(name)) {
        Some(Some(Json::Bool(x))) => *x,
        Some(Some(Json::Num(x))) => js::truthy(*x),
        Some(Some(Json::Text(s))) => !s.is_empty(),
        Some(Some(_)) => true,
        _ => false,
    }
}

/// `c.flag[name] = on` (tut-solo.script.ts:59, :131)
fn flag_set(g: &mut Game, r: DirectorRunId, name: &str, value: Json) { g.tab.director_run[r].flags.set(String::from(name), Some(value)); }

/// `c.flag.gun as string | undefined` (tut-solo.script.ts:84)
fn gun_mode(g: &Game, r: DirectorRunId) -> Option<String> {
    match g.tab.director_run[r].flags.get(String::from("gun")) {
        Some(Some(Json::Text(s))) => Some(s.clone()),
        _ => None,
    }
}

/// `c.query.inside(v, 'sam')` (tut-solo.script.ts:64, :110, :121)
fn sam_inside(g: &Game, r: DirectorRunId, which: fn(&TutSoloVars) -> &Option<VolumeRef>) -> bool {
    let id = which(tut_solo_vars(g, r)).as_ref().unwrap().id.clone();
    director_inside_volume(g, r, &id, &DirectorInsideVolumeWho::Sam)
}

/// the script's closure `aim`: a man told to aim somewhere turns his whole body to it (tut-solo.script.ts:44)
fn aim(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef, at: V3, hot: bool) -> Result<(), String> {
    let face = crate::director::util::yaw_to(director_query_pos_unit(g, r, u)?, at);
    let o = OrderPatch2 {
        aim_at: Some(Some(Place::List(at))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        face: Some(Some(face)),
        hot: if hot { Some(Some(true)) } else { None },
        ..Default::default()
    };
    director_beat_order(g, r, b, u, &o)
}

/// `c.stimulus.burst(u, at, rounds, { cadence, spread })` of a point (tut-solo.script.ts:69, :89, :92, :96, :144)
fn burst_at(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef, at: V3, rounds: f64, cadence: f64, spread: f64) -> Result<Cond, String> {
    director_beat_burst(g, r, b, u, RunBurstsItemAt::List(at), rounds, Some(cadence), Some(spread))
}

/// `if (c.query.alive(u)) c.release(u, 'alert')` (tut-solo.script.ts:112, :125, :149)
fn release_alert(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &UnitRef) -> Result<(), String> {
    if director_query_alive(g, r, u) { director_beat_release(g, r, b, u, Some(EndGuardScriptState::Alert))?; }
    Ok(())
}

/// the define pass: everything the script names, in its order; its locals go in the answer (tut-solo.script.ts:35-152)
pub fn tut_solo_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_solo::TutSoloVars, String> {
    let back_door = crate::director::manifest_door(m, "store");
    let inner_door = crate::director::manifest_door(m, "inner");
    let shop_line = crate::director::manifest_volume(m, "shop_line");
    let shop_in = crate::director::manifest_volume(m, "shop_in");
    let store_in = crate::director::manifest_volume(m, "store_in");
    let road_east = crate::director::manifest_volume(m, "road_east");
    let road_nose = crate::director::manifest_volume(m, "road_nose");
    let rifleman = crate::director::manifest_loadout(m, "rifleman", LoadoutSpec { team: crate::teams::Team::Guard, weapon: Some(crate::weapons::WeaponId::Akms), ..Default::default() });
    let staff: Vec<String> = ["Kowalski", "Novak", "Reyes", "Okafor", "Lindqvist"].iter().map(|n| String::from(*n)).collect();
    let mut aims: JsMap<String, V3> = JsMap::new();
    aims.set(String::from("Kowalski"), VAN_FLANK[1]);
    aims.set(String::from("Novak"), TRUCK[0]);
    aims.set(String::from("Reyes"), SHOP_DOORWAY);
    aims.set(String::from("Okafor"), PART_DOORWAY);
    aims.set(String::from("Lindqvist"), PART_DOORWAY);

    crate::director::manifest_program(m, "tutorial", &mut |p| {
        crate::director::program_on_start(p, "hold");
        for cue in ["road", "cease", "way", "gun", "bags", "gap", "blind", "shop", "room", "trucks", "loose"] {
            crate::director::program_on_signal(p, &format!("lesson:{}", cue), cue);
        }
        for beat in ["hold", "cease", "road", "way", "gun", "bags", "gap", "blind", "shop", "room", "loose", "trucks"] {
            crate::director::program_beat(p, beat, &crate::scene_level::LevelScriptFn::TutSolo, None)?;
        }
        Ok(())
    }, None)?;

    Ok(TutSoloVars {
        back_door: Some(back_door),
        inner_door: Some(inner_door),
        shop_line: Some(shop_line),
        shop_in: Some(shop_in),
        store_in: Some(store_in),
        road_east: Some(road_east),
        road_nose: Some(road_nose),
        rifleman: Some(rifleman),
        staff,
        aims: Some(aims),
    })
}

/// every man on his mark, his gun on the way the player will come (tut-solo.script.ts:52-55). Resume points: 1
pub fn tut_solo_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    let (staff, aims) = { let v = tut_solo_vars(g, r); (v.staff.clone(), v.aims.clone().unwrap()) };
    for n in &staff {
        let Some(u) = director_beat_unit(g, r, n) else { continue };
        if !director_query_alive(g, r, &u) { continue; }
        let at = *aims.get(n.clone()).unwrap();
        let face = crate::director::util::yaw_to(director_query_pos_unit(g, r, &u)?, at);
        let o = OrderPatch2 {
            goal: Some(None),
            aim_at: Some(Some(Place::List(at))),
            upper: Some(Some(crate::character::UpperMode::Aim)),
            face: Some(Some(face)),
            ..Default::default()
        };
        director_beat_order(g, r, b, &u, &o)?;
    }
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

/// the player is at the yard wall: the sentry stops (tut-solo.script.ts:59). Resume points: 1
pub fn tut_solo_cease_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloCeaseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    flag_set(g, r, "cease", Json::Bool(true));
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

/// the beat 'road' closure `lane`: the player is on the side the rounds cross (tut-solo.script.ts:64)
fn road_lane(g: &Game, r: DirectorRunId) -> bool {
    sam_inside(g, r, |v| &v.road_east) || sam_inside(g, r, |v| &v.road_nose)
}

/// the sentry puts rounds into the van until 'cease', then waits to be shot (tut-solo.script.ts:60-73). Resume points: 5, 6 is the loop's head
pub fn tut_solo_road_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloRoadFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    const HEAD: u16 = 6;
    loop {
        match f.at {
            0 => {
                let Some(s) = director_beat_unit(g, r, "Kowalski") else { return Ok(None) };
                f.s = Some(s);
                f.at = 1;
                return Ok(Some(director_beat_wait(g, r, 0.8, None)?));
            }
            1 => { f.i = 0.0; f.at = HEAD; }
            // after the wait while he is in the lane, `continue`; after the burst's pause, the end of the body: both go on to i++
            2 | 4 => { f.i += 1.0; f.at = HEAD; }
            3 => {
                let w = 1.3 + director_beat_rand(g, r) * 1.1;
                f.at = 4;
                return Ok(Some(director_beat_wait(g, r, w, None)?));
            }
            HEAD => {
                let s = f.s.clone().unwrap();
                if !(director_query_alive(g, r, &s) && !flag_truthy(g, r, "cease")) {
                    f.at = 5;
                    return Ok(Some(director_beat_until(CondTest::Script { id: 0 }, Some("the sentry is down"))));
                }
                if road_lane(g, r) {
                    f.at = 2;
                    return Ok(Some(director_beat_wait(g, r, 0.2, None)?));
                }
                let at = pick(&VAN_FLANK, f.i);
                aim(g, r, b, &s, at, true)?;
                f.at = 3;
                return Ok(Some(burst_at(g, r, b, &s, at, 3.0, 0.12, 0.02)?));
            }
            _ => return Ok(None),
        }
    }
}

/// the beat 'road' c.until: 0 = the sentry is down (tut-solo.script.ts:72)
pub fn tut_solo_road_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_solo::TutSoloRoadFrame, id: u32) -> Result<bool, String> {
    match id {
        0 => Ok(!director_query_alive(g, r, f.s.as_ref().unwrap())),
        _ => Err(format!("tut-solo road: no test {}", id)),
    }
}

/// the inner door thrown open if his rounds did not take it off (tut-solo.script.ts:77). Resume points: 1
pub fn tut_solo_way_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloWayFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    let d = tut_solo_vars(g, r).inner_door.clone().unwrap();
    if !director_query_door(g, &d)?.open {
        director_beat_door_lock(g, &d, false)?;
        director_beat_door_open(g, &d, Some(1.5), Some(true))?;
    }
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

/// the belt-fed gun: what he shoots at is the lesson of the step that is up (tut-solo.script.ts:80-100). Resume points: 7, 8 is the loop's head
pub fn tut_solo_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    const HEAD: u16 = 8;
    loop {
        match f.at {
            0 => {
                let Some(u) = director_beat_unit(g, r, "Novak") else { return Ok(None) };
                f.u = Some(u);
                f.at = 1;
                return Ok(Some(director_beat_wait(g, r, 0.5, None)?));
            }
            1 => { f.k = (BAGS_TOP.len() - 1) as f64; f.dir = -1.0; f.i = 0.0; f.at = HEAD; }
            2 => { f.at = 3; return Ok(Some(director_beat_wait(g, r, 0.1, None)?)); }
            4 => { f.at = 5; return Ok(Some(director_beat_wait(g, r, 0.08, None)?)); }
            6 => {
                let w = if f.mode.as_deref() == Some("blind") { 1.6 + director_beat_rand(g, r) } else { 0.7 + director_beat_rand(g, r) * 0.7 };
                f.at = 7;
                return Ok(Some(director_beat_wait(g, r, w, None)?));
            }
            // the end of the loop's body, in each of its three branches: i++
            3 | 5 | 7 => { f.i += 1.0; f.at = HEAD; }
            HEAD => {
                let u = f.u.clone().unwrap();
                if !director_query_alive(g, r, &u) { return Ok(None); }
                f.mode = gun_mode(g, r);
                match f.mode.as_deref() {
                    Some("bags") => {
                        let at = BAGS_TOP[f.k as usize];
                        f.k += f.dir;
                        if f.k <= 0.0 || f.k >= (BAGS_TOP.len() - 1) as f64 { f.dir = -f.dir; }
                        aim(g, r, b, &u, at, true)?;
                        f.at = 2;
                        return Ok(Some(burst_at(g, r, b, &u, at, 4.0, 0.08, 0.01)?));
                    }
                    Some("gap") => {
                        let at = pick(&GAP_FLOOR, f.i);
                        aim(g, r, b, &u, at, true)?;
                        f.at = 4;
                        return Ok(Some(burst_at(g, r, b, &u, at, 2.0, 0.08, 0.006)?));
                    }
                    mode => {
                        let blind = mode == Some("blind");
                        let pts: &[V3] = if blind { &WAS } else { &TRUCK };
                        let at = pick(pts, m::floor(director_beat_rand(g, r) * pts.len() as f64));
                        aim(g, r, b, &u, at, true)?;
                        f.at = 6;
                        return Ok(Some(burst_at(g, r, b, &u, at, if blind { 5.0 } else { 8.0 }, 0.1, 0.022)?));
                    }
                }
            }
            _ => return Ok(None),
        }
    }
}

/// the gun's lesson: the sandbags (tut-solo.script.ts:101). Resume points: 1
pub fn tut_solo_bags_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloBagsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    flag_set(g, r, "gun", Json::Text(String::from("bags")));
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

/// the gun's lesson: the gap (tut-solo.script.ts:102). Resume points: 1
pub fn tut_solo_gap_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloGapFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    flag_set(g, r, "gun", Json::Text(String::from("gap")));
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

/// the gun's lesson: blind, where he was before the smoke (tut-solo.script.ts:103). Resume points: 1
pub fn tut_solo_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    flag_set(g, r, "gun", Json::Text(String::from("blind")));
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

const SHOP_IN_LINE: u32 = 0;
const SHOP_DOWN: u32 = 1;

/// the beat 'shop' closure `down` (tut-solo.script.ts:109)
fn shop_down() -> Cond { director_beat_until(CondTest::Script { id: SHOP_DOWN }, Some("he is down")) }

/// the man behind the door: held until the player is in his line and a second more (tut-solo.script.ts:107-113). Resume points: 2
pub fn tut_solo_shop_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloShopFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            let Some(u) = director_beat_unit(g, r, "Reyes") else { return Ok(None) };
            f.u = Some(u);
            f.at = 1;
            let line = director_beat_until(CondTest::Script { id: SHOP_IN_LINE }, Some("the player is in his line"));
            Ok(Some(director_beat_any(vec![line, shop_down()])))
        }
        1 if got.index == 0.0 => {
            f.at = 2;
            let w = director_beat_wait(g, r, 1.0, None)?;
            Ok(Some(director_beat_any(vec![w, shop_down()])))
        }
        1 | 2 => {
            f.at = 3;
            release_alert(g, r, b, f.u.as_ref().unwrap())?;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// the beat 'shop' c.untils: 0 = the player is in his line, 1 = he is down (tut-solo.script.ts:109-110)
pub fn tut_solo_shop_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_solo::TutSoloShopFrame, id: u32) -> Result<bool, String> {
    match id {
        SHOP_IN_LINE => Ok(sam_inside(g, r, |v| &v.shop_line) || sam_inside(g, r, |v| &v.shop_in)),
        SHOP_DOWN => Ok(!director_query_alive(g, r, f.u.as_ref().unwrap())),
        _ => Err(format!("tut-solo shop: no test {}", id)),
    }
}

/// the script's closure `pair`: the two in the store who are on the floor (tut-solo.script.ts:117)
fn pair(g: &mut Game, r: DirectorRunId) -> Vec<UnitRef> {
    let mut men = Vec::new();
    for n in ["Okafor", "Lindqvist"] {
        if let Some(u) = director_beat_unit(g, r, n) { men.push(u); }
    }
    men
}

const ROOM_HIT: u32 = 0;
const ROOM_INSIDE: u32 = 1;

/// the beat 'room' closure `hit` (tut-solo.script.ts:120)
fn room_hit() -> Cond { director_beat_until(CondTest::Script { id: ROOM_HIT }, Some("one of the pair is down")) }
/// the beat 'room' closure `inside` (tut-solo.script.ts:121)
fn room_inside() -> Cond { director_beat_until(CondTest::Script { id: ROOM_INSIDE }, Some("the player is in the store")) }

/// the last lines of the beat 'room': both let go (tut-solo.script.ts:125)
fn room_release(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, men: &[UnitRef]) -> Result<(), String> {
    for u in men { release_alert(g, r, b, u)?; }
    Ok(())
}

/// the two who hold the store: folded by a stun can, or shooting from both sides (tut-solo.script.ts:118-126). Resume points: 4
pub fn tut_solo_room_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloRoomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    match f.at {
        0 => {
            f.men = pair(g, r);
            if f.men.is_empty() { return Ok(None); }
            f.at = 1;
            let bang = director_beat_on(g, r, CondArm3Kind::Bang, None, Some(EventFilter::XAbove { x: PART_DOORWAY[0] }));
            Ok(Some(director_beat_any(vec![bang, room_hit(), room_inside()])))
        }
        1 if got.index == 0.0 => {
            f.at = 2;
            Ok(Some(director_beat_any(vec![room_inside(), room_hit()])))
        }
        1 if got.index == 1.0 => {
            f.at = 4;
            Ok(Some(director_beat_wait(g, r, 0.6, None)?))
        }
        2 => {
            f.at = 3;
            let w = director_beat_wait(g, r, if got.index == 0.0 { 2.0 } else { 0.6 }, None)?;
            Ok(Some(director_beat_any(vec![w, room_hit()])))
        }
        1 | 3 | 4 => {
            f.at = 5;
            room_release(g, r, b, &f.men)?;
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// the beat 'room' c.untils: 0 = one of the pair is down, 1 = the player is in the store (tut-solo.script.ts:120-121)
pub fn tut_solo_room_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_solo::TutSoloRoomFrame, id: u32) -> Result<bool, String> {
    match id {
        ROOM_HIT => {
            for u in &f.men { if !director_query_alive(g, r, u) { return Ok(true); } }
            Ok(false)
        }
        ROOM_INSIDE => Ok(sam_inside(g, r, |v| &v.store_in)),
        _ => Err(format!("tut-solo room: no test {}", id)),
    }
}

/// the line has gone by: the last two's show ends (tut-solo.script.ts:131). Resume points: 1
pub fn tut_solo_loose_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloLooseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at != 0 { return Ok(None); }
    flag_set(g, r, "loose", Json::Bool(true));
    f.at = 1;
    Ok(Some(director_beat_frames(g, r, Some(1.0))))
}

const TRUCKS_ARRIVED: u32 = 0;
const TRUCKS_ONE_DOWN: u32 = 1;
const TRUCKS_SHOW_OVER: u32 = 2;

/// the beat 'trucks' closure `up`: how many of the last two stand (tut-solo.script.ts:138)
fn trucks_up(g: &Game, r: DirectorRunId, men: &[TutSoloTrucksMen]) -> f64 {
    let mut n = 0.0;
    for m in men { if director_query_alive(g, r, m.u.as_ref().unwrap()) { n += 1.0; } }
    n
}

/// the last two come through the building, stand in the shop's doorway and shoot over him (tut-solo.script.ts:132-150). Resume points: 5, 6 is the loop's head
pub fn tut_solo_trucks_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_solo::TutSoloTrucksFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    const HEAD: u16 = 6;
    loop {
        match f.at {
            0 => {
                let d = tut_solo_vars(g, r).back_door.clone().unwrap();
                director_beat_door_lock(g, &d, false)?;
                director_beat_door_open(g, &d, Some(1.5), Some(true))?;
                f.at = 1;
                return Ok(Some(director_beat_wait(g, r, 0.4, None)?));
            }
            1 => {
                let rifleman = tut_solo_vars(g, r).rifleman.clone().unwrap();
                let names = ["Brandt", "Marsh"];
                let mut all_men = Vec::new();
                for (k, p) in LAST_IN.iter().enumerate() {
                    let u = match director_beat_unit(g, r, names[k]) {
                        Some(u) => u,
                        None => director_beat_spawn(g, r, b, &rifleman, &Place::List(*p), Some(names[k]), Some(-m::PI / 2.0), None)?,
                    };
                    all_men.push(TutSoloTrucksMen { k: k as f64, u: Some(u) });
                }
                f.men = Vec::new();
                for m in all_men {
                    if director_query_alive(g, r, m.u.as_ref().unwrap()) { f.men.push(m); }
                }
                if f.men.is_empty() {
                    director_beat_signal(g, r, b, "done:trucks", None);
                    return Ok(None);
                }
                f.all = f.men.len() as f64;
                for m in &f.men {
                    let k = m.k as usize;
                    let o = OrderPatch2 {
                        goal: Some(Some(Place::List(LAST_MEN[k]))),
                        speed: Some(Some(3.4)),
                        aim_at: Some(Some(Place::List(ARCH_HEAD[k][0]))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        hot: Some(Some(true)),
                        ..Default::default()
                    };
                    director_beat_order(g, r, b, m.u.as_ref().unwrap(), &o)?;
                }
                f.at = 2;
                let arrived = director_beat_until(CondTest::Script { id: TRUCKS_ARRIVED }, Some("in the doorway"));
                let w = director_beat_wait(g, r, 6.0, None)?;
                return Ok(Some(director_beat_any(vec![arrived, w])));
            }
            2 => {
                director_beat_signal(g, r, b, "done:trucks", None);
                f.i = 0.0;
                f.at = HEAD;
            }
            3 => {
                let secs = 0.9 + director_beat_rand(g, r) * 0.6;
                let w = director_beat_wait(g, r, secs, None)?;
                f.at = 4;
                let over = director_beat_until(CondTest::Script { id: TRUCKS_SHOW_OVER }, Some("the show is over"));
                return Ok(Some(director_beat_any(vec![w, over])));
            }
            4 => { f.i += 1.0; f.at = HEAD; }
            5 => {
                f.at = HEAD + 1;
                for m in &f.men { release_alert(g, r, b, m.u.as_ref().unwrap())?; }
                return Ok(None);
            }
            HEAD => {
                if !(trucks_up(g, r, &f.men) == f.all && !flag_truthy(g, r, "loose")) {
                    f.at = 5;
                    return Ok(Some(director_beat_wait(g, r, 0.6, None)?));
                }
                let mut shots = Vec::new();
                for m in &f.men {
                    let u = m.u.as_ref().unwrap();
                    let at = pick(&ARCH_HEAD[m.k as usize], f.i);
                    let face = crate::director::util::yaw_to(director_query_pos_unit(g, r, u)?, at);
                    let o = OrderPatch2 { aim_at: Some(Some(Place::List(at))), face: Some(Some(face)), ..Default::default() };
                    director_beat_order(g, r, b, u, &o)?;
                    shots.push(burst_at(g, r, b, u, at, 4.0, 0.14, 0.008)?);
                }
                f.at = 3;
                let down = director_beat_until(CondTest::Script { id: TRUCKS_ONE_DOWN }, Some("one of them is down"));
                return Ok(Some(director_beat_any(vec![director_beat_all(shots), down])));
            }
            _ => return Ok(None),
        }
    }
}

/// the beat 'trucks' c.untils: 0 = in the doorway, 1 = one of them is down, 2 = the show is over (tut-solo.script.ts:140, :145, :146)
pub fn tut_solo_trucks_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_solo::TutSoloTrucksFrame, id: u32) -> Result<bool, String> {
    match id {
        TRUCKS_ARRIVED => {
            for m in &f.men {
                let u = m.u.as_ref().unwrap();
                if !(!director_query_alive(g, r, u) || director_query_arrived(g, r, u)?) { return Ok(false); }
            }
            Ok(true)
        }
        TRUCKS_ONE_DOWN => Ok(trucks_up(g, r, &f.men) < f.all),
        TRUCKS_SHOW_OVER => Ok(trucks_up(g, r, &f.men) < f.all || flag_truthy(g, r, "loose")),
        _ => Err(format!("tut-solo trucks: no test {}", id)),
    }
}
