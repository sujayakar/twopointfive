//! levels/tut-kit.script.ts translated: its define pass and a step function for each beat.
#![allow(unused_imports, unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec, format};

/// the man on the gun, behind his two low walls in the gun court's far corner
const GUNNER: &str = "Novak";
const GUN_AT: V3 = [22.7, 0.0, 6.9];
/// through the arch, what the gun can reach of the truck court
const TRUCK: [V3; 4] = [[12.4, 1.45, 18.6], [11.6, 1.5, 19.2], [13.2, 1.4, 18.3], [12.0, 1.55, 18.9]];
/// along the sandbags, a point every 0.6 m, a hand over their top
const BAGS_TOP: [V3; 8] = [[16.6, 1.45, 13.6], [17.2, 1.45, 13.6], [17.8, 1.45, 13.6], [18.4, 1.45, 13.6], [19.0, 1.45, 13.6], [19.6, 1.45, 13.6], [20.2, 1.45, 13.6], [20.8, 1.45, 13.6]];
/// the gap between the sandbags and the barrier: eight lanes a hand apart
const GAP_FLOOR: [V3; 8] = [[21.55, 0.5, 13.75], [21.95, 0.75, 13.8], [21.75, 0.45, 13.75], [22.1, 0.7, 13.85], [21.65, 0.8, 13.7], [22.05, 0.5, 13.8], [21.85, 0.65, 13.75], [21.6, 0.6, 13.75]];
/// where he was: the sandbags' near end
const WAS: [V3; 3] = [[17.2, 1.3, 13.2], [18.4, 1.3, 13.4], [16.8, 1.42, 13.6]];
/// the two for the launcher: where they come onto the floor, where they stand, what each shoots at
const LAST_IN: [V3; 2] = [[37.4, 0.0, 14.0], [38.4, 0.0, 14.5]];
const LAST_MEN: [V3; 2] = [[25.7, 0.0, 13.98], [26.5, 0.0, 14.5]];
const LAST: [&str; 2] = ["Brandt", "Marsh"];
const ARCH_HEAD: [[V3; 3]; 2] = [[[16.0, 2.2, 12.5], [16.0, 2.1, 12.0], [16.0, 2.25, 12.3]], [[16.0, 2.2, 16.0], [16.0, 2.1, 16.5], [16.0, 2.25, 16.2]]];
/// the two of the wash yard: where they come onto the floor, where they get down
const YARD_IN: [V3; 2] = [[37.6, 0.0, 19.5], [38.6, 0.0, 20.1]];
const YARD_MEN: [V3; 2] = [[29.5, 0.0, 19.2], [29.5, 0.0, 20.4]];
const YARD: [&str; 2] = ["Voss", "Kerr"];
const YARD_GATEWAY: V3 = [25.0, 1.2, 19.75];
/// the doorways, chest high
const SHOP_DOORWAY: V3 = [25.4, 1.2, 14.25];
const PART_DOORWAY: V3 = [30.5, 1.2, 14.25];
const BUNK_DOORWAY: V3 = [27.75, 1.1, 10.2];
const FUEL_DOORWAY: V3 = [31.3, 1.1, 9.25];
/// who is aimed where while he is held, in the written order (Object.keys)
const AIMS: [(&str, V3); 6] = [("Tango 1", SHOP_DOORWAY), ("Tango 2", PART_DOORWAY), ("Tango 3", PART_DOORWAY), ("Tango 4", BUNK_DOORWAY), ("Tango 5", BUNK_DOORWAY), ("Tango 6", FUEL_DOORWAY)];
const SQUAD: [&str; 4] = ["Red One", "Red Two", "Blue One", "Blue Two"];
/// the cues the lesson sends, in the written order
const CUES: [&str; 16] = ["gun", "bags", "gap", "blind", "shop", "room", "trucks", "loose", "call", "box", "dark", "bunk", "wall", "yard", "seen", "free"];

/// a man told to aim somewhere turns his whole body to it   (tut-kit.script.ts:50 aim)
fn aim(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef, at: V3, hot: bool) -> Result<(), String> {
    let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, u)?, at);
    let o = crate::director::OrderPatch2 {
        aim_at: Some(Some(crate::director::Place::List(at))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        face: Some(Some(face)),
        hot: if hot { Some(Some(true)) } else { None },
        ..Default::default()
    };
    crate::director::director_beat_order(g, r, b, u, &o)
}

/// the named men on the floor and alive, in the order named   (tut-kit.script.ts:51 men)
fn men<S: AsRef<str>>(g: &mut Game, r: DirectorRunId, names: &[S]) -> Vec<crate::director::UnitRef> {
    let mut out = Vec::new();
    for n in names {
        if let Some(u) = crate::director::director_beat_unit(g, r, n.as_ref()) {
            if crate::director::director_query_alive(g, r, &u) { out.push(u); }
        }
    }
    out
}

/// a line on the squad's net from the first of `names` on his feet   (tut-kit.script.ts:53 radio)
fn radio(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, names: &[&str], line: &str) -> Result<(), String> {
    let u = match men(g, r, names).into_iter().next() {
        Some(u) => Some(u),
        None => men(g, r, &SQUAD).into_iter().next(),
    };
    if let Some(u) = u { crate::director::director_beat_say(g, r, b, &u, line, Some(true))?; }
    Ok(())
}

/// `!!q.flag(n)`, `!c.flag.n`: the run's flag as a condition   (director.ts:854)
fn flag_truthy(g: &Game, r: DirectorRunId, n: &str) -> bool {
    match g.tab.director_run[r].flags.get(String::from(n)) {
        Some(Some(crate::director::Json::Text(s))) => !s.is_empty(),
        Some(Some(crate::director::Json::Num(x))) => js::truthy(*x),
        Some(Some(crate::director::Json::Bool(v))) => *v,
        Some(Some(_)) => true,
        _ => false,
    }
}

/// `c.flag.gun as string | undefined`   (tut-kit.script.ts:85)
fn flag_text(g: &Game, r: DirectorRunId, n: &str) -> Option<String> {
    match g.tab.director_run[r].flags.get(String::from(n)) {
        Some(Some(crate::director::Json::Text(s))) => Some(s.clone()),
        _ => None,
    }
}

/// `c.flag.<n> = <text>`
fn set_flag_text(g: &mut Game, r: DirectorRunId, n: &str, v: &str) {
    g.tab.director_run[r].flags.set(String::from(n), Some(crate::director::Json::Text(String::from(v))));
}

/// `c.flag.<n> = true`
fn set_flag_true(g: &mut Game, r: DirectorRunId, n: &str) {
    g.tab.director_run[r].flags.set(String::from(n), Some(crate::director::Json::Bool(true)));
}

/// `q.inside(v, who)` of a volume the frame or the vars hold
fn inside(g: &mut Game, r: DirectorRunId, v: &Option<crate::director::VolumeRef>, who: crate::director::DirectorInsideVolumeWho) -> bool {
    let id = &v.as_ref().unwrap().id;
    crate::director::director_inside_volume(g, &crate::seats::here(g), r, id, &who)
}

/// `yield c.wait(secs)`'s condition
fn wait(g: &mut Game, r: DirectorRunId, secs: f64) -> Result<crate::director::Cond, String> {
    crate::director::director_beat_wait(g, r, secs, None)
}

/// `c.until(<test id>, label)`
fn until(id: u32, label: &str) -> crate::director::Cond {
    crate::director::director_beat_until(crate::director::CondTest::Script { id }, Some(label))
}

/// `c.on('bang', filter)`
fn on_bang(g: &Game, r: DirectorRunId, filter: crate::director::EventFilter) -> crate::director::Cond {
    crate::director::director_beat_on(g, r, crate::director::CondArm3Kind::Bang, None, Some(filter))
}

/// `if (c.query.alive(u)) c.release(u, 'alert')`
fn release_if_alive(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, u: &crate::director::UnitRef) -> Result<(), String> {
    if crate::director::director_query_alive(g, r, u) {
        crate::director::director_beat_release(g, r, b, u, Some(crate::squad::EndGuardScriptState::Alert))?;
    }
    Ok(())
}

/// a beat that only waits and says a line: tut-kit.script.ts:153-158
fn wait_then_radio(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, at: &mut u16, secs: f64, names: &[&str], line: &str) -> Result<Option<crate::director::Cond>, String> {
    if *at == 0 {
        *at = 1;
        return Ok(Some(wait(g, r, secs)?));
    }
    radio(g, r, b, names, line)?;
    Ok(None)
}

/// a beat that sets a flag and gives a frame: tut-kit.script.ts:102-104
fn flag_then_frame(g: &mut Game, r: DirectorRunId, at: &mut u16, set: &mut dyn FnMut(&mut Game)) -> Result<Option<crate::director::Cond>, String> {
    if *at == 0 {
        set(g);
        *at = 1;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// (levels/tut-kit.script.ts:42 `script(D)`) the define pass
pub fn tut_kit_define(m: &mut crate::director::Manifest) -> Result<crate::script_tut_kit::TutKitVars, String> {
    let back_door = crate::director::manifest_door(m, "store");
    let wash_door = crate::director::manifest_door(m, "wash");
    let shop_line = crate::director::manifest_volume(m, "shop_line");
    let shop_in = crate::director::manifest_volume(m, "shop_in");
    let store_in = crate::director::manifest_volume(m, "store_in");
    let bunk_in = crate::director::manifest_volume(m, "bunk_in");
    let fuel_in = crate::director::manifest_volume(m, "fuel_in");
    let wash_in = crate::director::manifest_volume(m, "wash_in");
    let gun = crate::director::manifest_loadout(m, "gunner", crate::director::LoadoutSpec { team: crate::teams::Team::Hostile, weapon: Some(crate::weapons::WeaponId::Pkm), light: Some(false), ..Default::default() });
    let rifleman = crate::director::manifest_loadout(m, "rifleman", crate::director::LoadoutSpec { team: crate::teams::Team::Hostile, weapon: Some(crate::weapons::WeaponId::Akms), light: Some(false), ..Default::default() });
    let torch = crate::director::manifest_loadout(m, "torch", crate::director::LoadoutSpec { team: crate::teams::Team::Hostile, weapon: Some(crate::weapons::WeaponId::Akms), light: Some(true), ..Default::default() });
    m.systems.set("qrf", false);          // nobody is called: every man on this floor is one the mission put there
    m.systems.set("theirFrags", false);   // the only grenades thrown here are his and his squad's

    crate::director::manifest_program(m, "tutorial", &mut |p: &mut crate::director::ProgramDef| -> Result<(), String> {
        let script = crate::scene_level::LevelScriptFn::TutKit;
        crate::director::program_on_start(p, "hold");
        for cue in CUES { crate::director::program_on_signal(p, &format!("lesson:{}", cue), cue); }
        for beat in ["hold", "gun", "bags", "gap", "blind", "shop", "room", "loose", "trucks", "call", "box", "dark", "bunk", "wall", "bunkroom", "fuelroom", "free", "seen", "yard"] {
            crate::director::program_beat(p, beat, &script, None)?;
        }
        Ok(())
    }, None)?;

    Ok(crate::script_tut_kit::TutKitVars {
        back_door: Some(back_door),
        wash_door: Some(wash_door),
        shop_line: Some(shop_line),
        shop_in: Some(shop_in),
        store_in: Some(store_in),
        bunk_in: Some(bunk_in),
        fuel_in: Some(fuel_in),
        wash_in: Some(wash_in),
        gun: Some(gun),
        rifleman: Some(rifleman),
        torch: Some(torch),
    })
}

/// every man on his mark, his gun on the way the team will come; the gun's man is put on his gun once   (tut-kit.script.ts:71 'hold')
pub fn tut_kit_hold_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitHoldFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 && crate::director::director_beat_unit(g, r, GUNNER).is_none() {
        f.at = 1;
        return Ok(Some(wait(g, r, 0.5)?));
    }
    if f.at == 1 && crate::director::director_beat_unit(g, r, GUNNER).is_none() {
        let gun = tut_kit_vars(g, r).gun.clone().unwrap();
        crate::director::director_beat_spawn(g, r, b, &gun, &crate::director::Place::List(GUN_AT), Some(GUNNER), Some(m::PI / 2.0), Some(false))?;
    }
    if f.at < 2 {
        for (n, at) in AIMS { hold_on_mark(g, r, b, n, at)?; }
        hold_on_mark(g, r, b, GUNNER, TRUCK[0])?;
        crate::director::director_beat_start(g, r, b, "bunkroom");
        crate::director::director_beat_start(g, r, b, "fuelroom");
        f.at = 2;
        return Ok(Some(crate::director::director_beat_frames(g, r, Some(1.0))));
    }
    Ok(None)
}

/// `const u = c.unit(n); if (u && alive) c.order(u, { goal: null, aimAt: at, upper: 'aim', face: yawTo(pos(u), at) })`   (tut-kit.script.ts:74, :75)
fn hold_on_mark(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, n: &str, at: V3) -> Result<(), String> {
    let Some(u) = crate::director::director_beat_unit(g, r, n) else { return Ok(()) };
    if !crate::director::director_query_alive(g, r, &u) { return Ok(()); }
    let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, &u)?, at);
    let o = crate::director::OrderPatch2 {
        goal: Some(None),
        aim_at: Some(Some(crate::director::Place::List(at))),
        upper: Some(Some(crate::character::UpperMode::Aim)),
        face: Some(Some(face)),
        ..Default::default()
    };
    crate::director::director_beat_order(g, r, b, &u, &o)
}

/// THE GUN: what he shoots at is the lesson of the step that is up   (tut-kit.script.ts:81 'gun')
pub fn tut_kit_gun_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitGunFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                f.u = crate::director::director_beat_unit(g, r, GUNNER);
                if f.u.is_none() { return Ok(None); }
                f.at = 1;
                return Ok(Some(wait(g, r, 0.5)?));
            }
            1 => {
                f.k = (BAGS_TOP.len() - 1) as f64;
                f.dir = -1.0;
                f.i = 0.0;
                f.at = 2;
            }
            // the loop's head: its condition, then the mode this pass goes by
            2 => {
                let u = f.u.as_ref().unwrap();
                if !crate::director::director_query_alive(g, r, u) { return Ok(None); }
                f.mode = flag_text(g, r, "gun");
                if f.mode.as_deref() == Some("bags") {
                    let at = BAGS_TOP[f.k as usize];
                    f.k += f.dir;
                    if f.k <= 0.0 || f.k >= (BAGS_TOP.len() - 1) as f64 { f.dir = -f.dir; }
                    aim(g, r, b, u, at, true)?;
                    f.at = 3;
                    return Ok(Some(crate::director::director_beat_burst(g, r, b, u, crate::director::RunBurstsItemAt::List(at), 4.0, Some(0.08), Some(0.01))?));
                } else if f.mode.as_deref() == Some("gap") {
                    let at = GAP_FLOOR[js::rem(f.i, GAP_FLOOR.len() as f64) as usize];
                    aim(g, r, b, u, at, true)?;
                    f.at = 5;
                    return Ok(Some(crate::director::director_beat_burst(g, r, b, u, crate::director::RunBurstsItemAt::List(at), 2.0, Some(0.08), Some(0.006))?));
                } else {
                    let blind = f.mode.as_deref() == Some("blind");
                    let pts: &[V3] = if blind { &WAS } else { &TRUCK };
                    let len = pts.len() as f64;
                    let at = pts[js::rem(m::floor(crate::director::director_beat_rand(g, r) * len), len) as usize];
                    aim(g, r, b, u, at, true)?;
                    f.at = 7;
                    return Ok(Some(crate::director::director_beat_burst(g, r, b, u, crate::director::RunBurstsItemAt::List(at), if blind { 5.0 } else { 8.0 }, Some(0.1), Some(0.022))?));
                }
            }
            3 => {
                f.at = 4;
                return Ok(Some(wait(g, r, 0.1)?));
            }
            5 => {
                f.at = 6;
                return Ok(Some(wait(g, r, 0.08)?));
            }
            7 => {
                f.at = 8;
                let secs = if f.mode.as_deref() == Some("blind") {
                    1.6 + crate::director::director_beat_rand(g, r)
                } else {
                    0.7 + crate::director::director_beat_rand(g, r) * 0.7
                };
                return Ok(Some(wait(g, r, secs)?));
            }
            // the end of a pass, whichever branch it took: i++, back to the head
            4 | 6 | 8 => {
                f.i += 1.0;
                f.at = 2;
            }
            n => return Err(format!("tut-kit gun: no resume point {}", n)),
        }
    }
}

/// (tut-kit.script.ts:102 'bags')
pub fn tut_kit_bags_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBagsFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    flag_then_frame(g, r, &mut f.at, &mut |g| set_flag_text(g, r, "gun", "bags"))
}

/// (tut-kit.script.ts:103 'gap')
pub fn tut_kit_gap_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitGapFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    flag_then_frame(g, r, &mut f.at, &mut |g| set_flag_text(g, r, "gun", "gap"))
}

/// (tut-kit.script.ts:104 'blind')
pub fn tut_kit_blind_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBlindFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    flag_then_frame(g, r, &mut f.at, &mut |g| set_flag_text(g, r, "gun", "blind"))
}

/// test ids of 'shop': the player in his line; the man down
const SHOP_IN_LINE: u32 = 1;
const SHOP_DOWN: u32 = 2;

/// THE SHOP: the man behind the door, held until the player steps into his line and a second more   (tut-kit.script.ts:108 'shop')
pub fn tut_kit_shop_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitShopFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        f.u = crate::director::director_beat_unit(g, r, "Tango 1");
        if f.u.is_none() { return Ok(None); }
        f.at = 1;
        return Ok(Some(crate::director::director_beat_any(vec![until(SHOP_IN_LINE, "the player is in his line"), until(SHOP_DOWN, "he is down")])));
    }
    if f.at == 1 {
        f.at = 2;
        if got.index == 0.0 {
            return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, 1.0)?, until(SHOP_DOWN, "he is down")])));
        }
    }
    release_if_alive(g, r, b, f.u.as_ref().unwrap())?;
    Ok(None)
}

/// (tut-kit.script.ts:108-114) the c.until tests of 'shop'
pub fn tut_kit_shop_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_kit::TutKitShopFrame, id: u32) -> Result<bool, String> {
    match id {
        SHOP_IN_LINE => {
            let (line, room) = { let v = tut_kit_vars(g, r); (v.shop_line.clone(), v.shop_in.clone()) };
            Ok(inside(g, r, &line, crate::director::DirectorInsideVolumeWho::Sam) || inside(g, r, &room, crate::director::DirectorInsideVolumeWho::Sam))
        }
        SHOP_DOWN => Ok(!crate::director::director_query_alive(g, r, f.u.as_ref().unwrap())),
        n => Err(format!("tut-kit shop: no test {}", n)),
    }
}

/// test ids of 'room': one of the pair down; the player in the store
const ROOM_HIT: u32 = 1;
const ROOM_INSIDE: u32 = 2;

/// THE STORE: two men hold the room on its doorway; a stun grenade folds them   (tut-kit.script.ts:118 'room')
pub fn tut_kit_room_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitRoomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let hit = || until(ROOM_HIT, "one of the pair is down");
    let in_store = || until(ROOM_INSIDE, "the player is in the store");
    match f.at {
        0 => {
            f.pair = Vec::new();
            for n in ["Tango 2", "Tango 3"] {
                if let Some(u) = crate::director::director_beat_unit(g, r, n) { f.pair.push(u); }
            }
            if f.pair.is_empty() { return Ok(None); }
            f.at = 1;
            let bang = on_bang(g, r, crate::director::EventFilter::XAbove { x: PART_DOORWAY[0] });
            return Ok(Some(crate::director::director_beat_any(vec![bang, hit(), in_store()])));
        }
        1 => {
            if got.index == 0.0 {
                f.at = 2;
                return Ok(Some(crate::director::director_beat_any(vec![in_store(), hit()])));
            }
            f.at = 3;
            if got.index == 1.0 { return Ok(Some(wait(g, r, 0.6)?)); }
        }
        2 => {
            f.at = 3;
            let secs = if got.index == 0.0 { 2.0 } else { 0.6 };
            return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, secs)?, hit()])));
        }
        _ => {}
    }
    for u in &f.pair { release_if_alive(g, r, b, u)?; }
    Ok(None)
}

/// (tut-kit.script.ts:118-126) the c.until tests of 'room'
pub fn tut_kit_room_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_kit::TutKitRoomFrame, id: u32) -> Result<bool, String> {
    match id {
        ROOM_HIT => Ok(f.pair.iter().any(|u| !crate::director::director_query_alive(g, r, u))),
        ROOM_INSIDE => {
            let store = tut_kit_vars(g, r).store_in.clone();
            Ok(inside(g, r, &store, crate::director::DirectorInsideVolumeWho::Sam))
        }
        n => Err(format!("tut-kit room: no test {}", n)),
    }
}

/// (tut-kit.script.ts:131 'loose')
pub fn tut_kit_loose_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitLooseFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    flag_then_frame(g, r, &mut f.at, &mut |g| set_flag_true(g, r, "loose"))
}

/// how many of the two are still up   (tut-kit.script.ts:138 up)
fn trucks_up(g: &Game, r: DirectorRunId, two: &[crate::script_tut_kit::TutKitTrucksTwo]) -> f64 {
    let mut n = 0.0;
    for m in two { if crate::director::director_query_alive(g, r, m.u.as_ref().unwrap()) { n += 1.0; } }
    n
}

/// test ids of 'trucks': both in the doorway; one of them down; the show over
const TRUCKS_IN_DOORWAY: u32 = 1;
const TRUCKS_DOWN: u32 = 2;
const TRUCKS_OVER: u32 = 3;

/// THE LAUNCHER: two more through the building, stood in the shop's doorway, shooting the head of the arch's wall   (tut-kit.script.ts:132 'trucks')
pub fn tut_kit_trucks_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitTrucksFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    loop {
        match f.at {
            0 => {
                let back = tut_kit_vars(g, r).back_door.clone().unwrap();
                crate::director::director_beat_door_lock(g, &back, false)?;
                crate::director::director_beat_door_open(g, &back, Some(1.5), Some(true))?;
                f.at = 1;
                return Ok(Some(wait(g, r, 0.4)?));
            }
            1 => {
                let rifleman = tut_kit_vars(g, r).rifleman.clone().unwrap();
                let mut two = Vec::new();
                for k in 0..LAST_IN.len() {
                    let u = match crate::director::director_beat_unit(g, r, LAST[k]) {
                        Some(u) => u,
                        None => crate::director::director_beat_spawn(g, r, b, &rifleman, &crate::director::Place::List(LAST_IN[k]), Some(LAST[k]), Some(-m::PI / 2.0), Some(false))?,
                    };
                    two.push(crate::script_tut_kit::TutKitTrucksTwo { k: k as f64, u: Some(u) });
                }
                two.retain(|m| crate::director::director_query_alive(g, r, m.u.as_ref().unwrap()));
                f.two = two;
                if f.two.is_empty() {
                    crate::director::director_beat_signal(g, r, b, "done:trucks", None);
                    return Ok(None);
                }
                f.all = f.two.len() as f64;
                for m in &f.two {
                    let k = m.k as usize;
                    let o = crate::director::OrderPatch2 {
                        goal: Some(Some(crate::director::Place::List(LAST_MEN[k]))),
                        speed: Some(Some(3.4)),
                        aim_at: Some(Some(crate::director::Place::List(ARCH_HEAD[k][0]))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        hot: Some(Some(true)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, m.u.as_ref().unwrap(), &o)?;
                }
                f.at = 2;
                return Ok(Some(crate::director::director_beat_any(vec![until(TRUCKS_IN_DOORWAY, "in the doorway"), wait(g, r, 6.0)?])));
            }
            2 => {
                crate::director::director_beat_signal(g, r, b, "done:trucks", None);
                f.i = 0.0;
                f.at = 3;
            }
            // the loop's head: the show, until one of them is hit or the line has gone by
            3 => {
                if trucks_up(g, r, &f.two) == f.all && !flag_truthy(g, r, "loose") {
                    let mut shots = Vec::new();
                    for m in &f.two {
                        let row = &ARCH_HEAD[m.k as usize];
                        let at = row[js::rem(f.i, row.len() as f64) as usize];
                        let u = m.u.as_ref().unwrap();
                        let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, u)?, at);
                        let o = crate::director::OrderPatch2 { aim_at: Some(Some(crate::director::Place::List(at))), face: Some(Some(face)), ..Default::default() };
                        crate::director::director_beat_order(g, r, b, u, &o)?;
                        shots.push(crate::director::director_beat_burst(g, r, b, u, crate::director::RunBurstsItemAt::List(at), 4.0, Some(0.14), Some(0.008))?);
                    }
                    f.at = 4;
                    return Ok(Some(crate::director::director_beat_any(vec![crate::director::director_beat_all(shots), until(TRUCKS_DOWN, "one of them is down")])));
                }
                f.at = 6;
                return Ok(Some(wait(g, r, 0.6)?));
            }
            4 => {
                f.at = 5;
                let secs = 0.9 + crate::director::director_beat_rand(g, r) * 0.6;
                return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, secs)?, until(TRUCKS_OVER, "the show is over")])));
            }
            5 => {
                f.i += 1.0;
                f.at = 3;
            }
            _ => {
                for m in &f.two { release_if_alive(g, r, b, m.u.as_ref().unwrap())?; }
                return Ok(None);
            }
        }
    }
}

/// (tut-kit.script.ts:132-150) the c.until tests of 'trucks'
pub fn tut_kit_trucks_test(g: &crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_kit::TutKitTrucksFrame, id: u32) -> Result<bool, String> {
    match id {
        TRUCKS_IN_DOORWAY => {
            for m in &f.two {
                let u = m.u.as_ref().unwrap();
                if crate::director::director_query_alive(g, r, u) && !crate::director::director_query_arrived(g, r, u)? { return Ok(false); }
            }
            Ok(true)
        }
        TRUCKS_DOWN => Ok(trucks_up(g, r, &f.two) < f.all),
        TRUCKS_OVER => Ok(trucks_up(g, r, &f.two) < f.all || flag_truthy(g, r, "loose")),
        n => Err(format!("tut-kit trucks: no test {}", n)),
    }
}

/// (tut-kit.script.ts:153 'call')
pub fn tut_kit_call_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitCallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    wait_then_radio(g, r, b, &mut f.at, 0.8, &["Red One"], "We heard all that from the road. Say the word.")
}

/// (tut-kit.script.ts:154 'box')
pub fn tut_kit_box_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBoxFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    wait_then_radio(g, r, b, &mut f.at, 0.6, &["Red One"], "There's a breaker box on the court wall. We can kill the lights.")
}

/// (tut-kit.script.ts:156 'dark')
pub fn tut_kit_dark_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitDarkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    wait_then_radio(g, r, b, &mut f.at, 0.5, &["Red Two"], "Lights are out. Goggles on.")
}

/// (tut-kit.script.ts:157 'bunk')
pub fn tut_kit_bunk_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBunkFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    wait_then_radio(g, r, b, &mut f.at, 0.6, &["Blue One"], "There's a door off the shop we haven't opened.")
}

/// (tut-kit.script.ts:158 'wall')
pub fn tut_kit_wall_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitWallFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    wait_then_radio(g, r, b, &mut f.at, 0.6, &["Blue One"], "That door's locked. We can put a charge on the wall.")
}

/// a fresh frame of the helper `room`, its parameters in it
fn room_frame(names: &[&str], inside: Option<crate::director::VolumeRef>, x0: f64, x1: f64, z0: f64, z1: f64) -> Box<crate::script_tut_kit::TutKitRoomHelperFrame> {
    let mut f = crate::script_tut_kit::TutKitRoomHelperFrame::default();
    f.names = names.iter().map(|n| String::from(*n)).collect();
    f.inside = inside;
    f.box_ = Some(crate::script_tut_kit::TutKitRoomHelperBox { x0, x1, z0, z1 });
    Box::new(f)
}

/// `yield* room(…)` over a beat's `sub_room`: its conditions pass up unchanged; its end clears the field and ends the beat
fn step_room(g: &mut Game, r: DirectorRunId, b: DirectorLiveBeatId, sub: &mut Option<Box<crate::script_tut_kit::TutKitRoomHelperFrame>>, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let Some(h) = sub.as_mut() else { return Ok(None) };
    let out = tut_kit_room_helper_step(g, r, b, h, got)?;
    if out.is_none() { *sub = None; }
    Ok(out)
}

/// (tut-kit.script.ts:160 'bunkroom')
pub fn tut_kit_bunkroom_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitBunkroomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        f.at = 1;
        let bunk_in = tut_kit_vars(g, r).bunk_in.clone();
        f.sub_room = Some(room_frame(&["Tango 4", "Tango 5"], bunk_in, 25.0, 31.0, 5.0, 10.5));
    }
    step_room(g, r, b, &mut f.sub_room, got)
}

/// (tut-kit.script.ts:161 'fuelroom')
pub fn tut_kit_fuelroom_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitFuelroomFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 {
        f.at = 1;
        let fuel_in = tut_kit_vars(g, r).fuel_in.clone();
        f.sub_room = Some(room_frame(&["Tango 6"], fuel_in, 30.4, 36.0, 5.0, 10.5));
    }
    step_room(g, r, b, &mut f.sub_room, got)
}

/// (tut-kit.script.ts:166 'free')
pub fn tut_kit_free_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitFreeFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    flag_then_frame(g, r, &mut f.at, &mut |g| set_flag_true(g, r, "free"))
}

/// a teammate calls the two in the yard, and the line goes by a breath later   (tut-kit.script.ts:167 'seen')
pub fn tut_kit_seen_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitSeenFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    if f.at == 0 && !men(g, r, &YARD).is_empty() {
        radio(g, r, b, &["Red Two", "Blue Two"], "Two more in the wash yard, down behind the sandbags. No shot from here.")?;
        f.at = 1;
        return Ok(Some(wait(g, r, 2.2)?));
    }
    crate::director::director_beat_signal(g, r, b, "done:seen", None);
    Ok(None)
}

/// test ids of 'yard': both behind the sandbags; one of them down; the team in the yard, or the last line up
const YARD_BEHIND_BAGS: u32 = 1;
const YARD_DOWN: u32 = 2;
const YARD_TEAM_IN: u32 = 3;

/// THE WASH YARD: two more by its back door, low, down behind the sandbags with their flashlights on   (tut-kit.script.ts:171 'yard')
pub fn tut_kit_yard_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitYardFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let hit = || until(YARD_DOWN, "one of them is down");
    match f.at {
        0 => {
            let wash = tut_kit_vars(g, r).wash_door.clone().unwrap();
            crate::director::director_beat_door_lock(g, &wash, false)?;
            crate::director::director_beat_door_open(g, &wash, Some(1.5), Some(true))?;
            f.at = 1;
            Ok(Some(wait(g, r, 0.4)?))
        }
        1 => {
            let torch = tut_kit_vars(g, r).torch.clone().unwrap();
            let mut two = Vec::new();
            for k in 0..YARD_IN.len() {
                let u = match crate::director::director_beat_unit(g, r, YARD[k]) {
                    Some(u) => u,
                    None => crate::director::director_beat_spawn(g, r, b, &torch, &crate::director::Place::List(YARD_IN[k]), Some(YARD[k]), Some(-m::PI / 2.0), Some(true))?,
                };
                two.push(crate::script_tut_kit::TutKitYardTwo { k: k as f64, u: Some(u) });
            }
            two.retain(|m| crate::director::director_query_alive(g, r, m.u.as_ref().unwrap()));
            f.two = two;
            if f.two.is_empty() {
                crate::director::director_beat_signal(g, r, b, "done:yard", None);
                return Ok(None);
            }
            for m in &f.two {
                let o = crate::director::OrderPatch2 {
                    goal: Some(Some(crate::director::Place::List(YARD_MEN[m.k as usize]))),
                    speed: Some(Some(2.6)),
                    crouch: Some(Some(true)),
                    aim_at: Some(Some(crate::director::Place::List(YARD_GATEWAY))),
                    upper: Some(Some(crate::character::UpperMode::Aim)),
                    light: Some(Some(false)),
                    hot: Some(Some(false)),
                    ..Default::default()
                };
                crate::director::director_beat_order(g, r, b, m.u.as_ref().unwrap(), &o)?;
            }
            f.at = 2;
            Ok(Some(crate::director::director_beat_any(vec![until(YARD_BEHIND_BAGS, "behind the sandbags"), wait(g, r, 9.0)?, hit()])))
        }
        2 => {
            for m in &f.two {
                let u = m.u.as_ref().unwrap();
                if crate::director::director_query_alive(g, r, u) {
                    let face = crate::director::util::yaw_to(crate::director::director_query_pos_unit(g, r, u)?, YARD_GATEWAY);
                    let o = crate::director::OrderPatch2 {
                        goal: Some(None),
                        crouch: Some(Some(true)),
                        aim_at: Some(Some(crate::director::Place::List(YARD_GATEWAY))),
                        upper: Some(Some(crate::character::UpperMode::Aim)),
                        face: Some(Some(face)),
                        light: Some(Some(true)),
                        ..Default::default()
                    };
                    crate::director::director_beat_order(g, r, b, u, &o)?;
                }
            }
            crate::director::director_beat_signal(g, r, b, "done:yard", None);
            f.at = 3;
            let bang = on_bang(g, r, crate::director::EventFilter::XZAbove { x: 25.0, z: 17.5 });
            Ok(Some(crate::director::director_beat_any(vec![bang, hit(), until(YARD_TEAM_IN, "the team is in the yard")])))
        }
        3 => {
            f.at = 4;
            Ok(Some(wait(g, r, 0.3)?))
        }
        _ => {
            for m in &f.two { release_if_alive(g, r, b, m.u.as_ref().unwrap())?; }
            Ok(None)
        }
    }
}

/// (tut-kit.script.ts:171-185) the c.until tests of 'yard'
pub fn tut_kit_yard_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_kit::TutKitYardFrame, id: u32) -> Result<bool, String> {
    match id {
        YARD_BEHIND_BAGS => {
            for m in &f.two {
                let u = m.u.as_ref().unwrap();
                if crate::director::director_query_alive(g, r, u) && !crate::director::director_query_arrived(g, r, u)? { return Ok(false); }
            }
            Ok(true)
        }
        YARD_DOWN => Ok(f.two.iter().any(|m| !crate::director::director_query_alive(g, r, m.u.as_ref().unwrap()))),
        YARD_TEAM_IN => {
            let wash = tut_kit_vars(g, r).wash_in.clone();
            Ok(inside(g, r, &wash, crate::director::DirectorInsideVolumeWho::Team) || flag_truthy(g, r, "free"))
        }
        n => Err(format!("tut-kit yard: no test {}", n)),
    }
}

/// test ids of the helper `room`: one of them down; the team in the room
const HELPER_HIT: u32 = 1;
const HELPER_IN_ROOM: u32 = 2;

/// a held room is the AI's own once a stun grenade or a blast has gone off in it, one of its men is hit, or the player or a teammate is inside it   (tut-kit.script.ts:55 room)
pub fn tut_kit_room_helper_step(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &mut crate::script_tut_kit::TutKitRoomHelperFrame, got: &crate::director::CondResult) -> Result<Option<crate::director::Cond>, String> {
    let hit = || until(HELPER_HIT, "one of them is down");
    let in_room = || until(HELPER_IN_ROOM, "the team is in the room");
    match f.at {
        0 => {
            f.held = men(g, r, &f.names);
            if f.held.is_empty() { return Ok(None); }
            let bx = f.box_.clone().unwrap();
            f.at = 1;
            let bang = on_bang(g, r, crate::director::EventFilter::InBox { x0: bx.x0, x1: bx.x1, z0: bx.z0, z1: bx.z1 });
            Ok(Some(crate::director::director_beat_any(vec![bang, hit(), in_room()])))
        }
        1 => {
            if got.index == 0.0 {
                f.at = 2;
                return Ok(Some(crate::director::director_beat_any(vec![wait(g, r, 2.0)?, hit(), in_room()])));
            }
            f.at = 3;
            Ok(Some(wait(g, r, 0.3)?))
        }
        2 => {
            f.at = 3;
            Ok(Some(wait(g, r, 0.3)?))
        }
        _ => {
            for u in &f.held { release_if_alive(g, r, b, u)?; }
            Ok(None)
        }
    }
}

/// (tut-kit.script.ts:55-63) the c.until tests of the helper `room`, reached through `director::beat_test`'s arms for bunkroom and fuelroom
pub fn tut_kit_room_helper_test(g: &mut crate::game::Game, r: DirectorRunId, b: DirectorLiveBeatId, f: &crate::script_tut_kit::TutKitRoomHelperFrame, id: u32) -> Result<bool, String> {
    match id {
        HELPER_HIT => Ok(f.held.iter().any(|u| !crate::director::director_query_alive(g, r, u))),
        HELPER_IN_ROOM => Ok(inside(g, r, &f.inside, crate::director::DirectorInsideVolumeWho::Team)),
        n => Err(format!("tut-kit room helper: no test {}", n)),
    }
}
