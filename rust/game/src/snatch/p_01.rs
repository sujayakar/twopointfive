//! src/game/snatch.ts lines 1 to 263
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The key an exit goes by in `Civ.badTargets`: identity of a CivSpot in a Set. The place in the level's list, phones first, as civilians.ts:809, 857, 860 count it; `u32::MAX` = not on the list. (snatch.ts:179)
fn exit_spot_key(g: &mut Game, spot: &crate::civilians::CivSpot) -> crate::civilians::CivBadTarget {
    let spots = crate::civilians::civ_spots(g);
    let place = spots.exits.iter().position(|s| s.name == spot.name && s.pos == spot.pos);
    crate::civilians::CivBadTarget::Spot(place.map_or(u32::MAX, |i| (spots.phones.len() + i) as u32))
}

/// `x.toFixed(1)`. Rust's `{:.1}` takes an exact tie to the even digit, JavaScript to the larger: a tie is an odd number of quarters.
fn to_fixed_1(x: f64) -> String {
    let a = m::abs(x);
    let quarters = a * 4.0;
    if a < 1e15 && m::floor(quarters) == quarters && js::rem(quarters, 2.0) == 1.0 {
        let n = m::floor(a * 10.0) + 1.0;
        let sign = if x < 0.0 { "-" } else { "" };
        return format!("{}{}.{}", sign, m::floor(n / 10.0), js::rem(n, 10.0));
    }
    if x == 0.0 { return String::from("0.0"); }
    format!("{:.1}", x)
}
/// A number in a template string: `-0` reads `0`. (Exponents, which only |x| under 1e-6 or from 1e21 would show, do not arise in these lines.)
fn num_str(x: f64) -> String {
    if x == 0.0 { return String::from("0"); }
    format!("{}", x)
}
/// `text.length`: JavaScript counts UTF-16 units.
fn js_len(text: &str) -> f64 { text.encode_utf16().count() as f64 }

/// The first man on `g.guards` with this body. (snatch.ts:190, 197)
fn mate_of(g: &Game, c: CharId) -> Option<GuardId> {
    g.guards.iter().copied().find(|&x| g.gd[x].ch == c)
}
/// A bone of a body, or the point this far above its feet. (snatch.ts:145, 150)
fn bone_or(g: &Game, c: CharId, bone: crate::character::Bone, up: f64) -> V3 {
    match g.ch[c].bones.get(bone) {
        Some(b) => *b,
        None => v3::add(g.ch[c].pos, [0.0, up, 0.0]),
    }
}
/// `!!this.data`, with no copy of the block.
fn has_data(g: &Game) -> bool {
    g.level.scenarios.as_ref().map_or(false, |s| s.snatch.is_some())
}

/// A new overlay with its van rule. (snatch.ts Snatch.constructor)
pub fn snatch_new(g: &mut crate::game::Game) -> crate::snatch::Snatch {
    let mut s = Snatch::blank();
    s.out = crate::snatchout::snatch_out_new(g);
    s
}

/// This level's block. (snatch.ts Snatch.block)
pub fn snatch_block(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::SnatchData> {
    snatch_data(g)
}

/// The mission still hangs on him: loose, or where he is to be driven off, not yet in the van. (snatch.ts Snatch.owes)
pub fn snatch_owes(g: &mut crate::game::Game) -> bool {
    g.snatch.end.is_none() && (crate::snatchout::snatch_out_ruled(g) || !snatch_in_hand(g))
}

/// This level's block. (snatch.ts Snatch.data)
pub fn snatch_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::SnatchData> {
    g.level.scenarios.as_ref().and_then(|s| s.snatch.clone())
}

/// He is staged on a level with the block, in a live room clear. (snatch.ts Snatch.active)
pub fn snatch_active(g: &mut crate::game::Game) -> bool {
    g.snatch.man.is_some() && has_data(g) && crate::roomclear::room_clear_scenario_active(g)
}

/// Is this the man? (snatch.ts Snatch.isMan)
pub fn snatch_is_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.snatch.man == Some(gd)
}

/// The wrists-behind pose is his once he is tied. (snatch.ts Snatch.isTied)
pub fn snatch_is_tied(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.snatch.man == Some(gd) && g.snatch.tied
}

/// Alive and in the team's hands: tied, in Sam's arm, or out cold by Sam's hands. (snatch.ts Snatch.inHand)
pub fn snatch_in_hand(g: &mut crate::game::Game) -> bool {
    let Some(m) = g.snatch.man else { return false };
    if g.gd[m].state == crate::game::GuardState::Dead {
        g.gd[m].down_kind != Some(crate::game::DownKind::Shot)
    } else {
        g.snatch.tied || g.gd[m].held.is_some()
    }
}

/// What he says in place of a civilian's line; a line already his passes as it is. No die: the pick is off the clock. (snatch.ts Snatch.voice)
pub fn snatch_voice(g: &mut crate::game::Game, gd: GuardId, text: &str) -> Option<String> {
    if crate::dialogue::snatch_lines().iter().any(|l| l == text) { return Some(String::from(text)); }
    let l = crate::dialogue::snatch_lines_table();
    let s = g.cv[g.gd[gd].civ.unwrap()].state;
    let pool = if g.snatch.tied {
        Some(&l.tied)
    } else if g.snatch.hands_up {
        Some(&l.hands_up)
    } else if s == crate::civilians::CivState::Cower {
        Some(&l.cornered)
    } else if s == crate::civilians::CivState::Alarmed || s == crate::civilians::CivState::Flee {
        Some(&l.running)
    } else {
        None
    };
    let pool = pool?;
    let at = js::rem(m::floor(g.time * 3.0 + js_len(text)), pool.len() as f64) as usize;
    Some(pool[at].clone())
}

/// A round has found him, before he goes down: whose, for the tally. (snatch.ts Snatch.roundOn)
pub fn snatch_round_on(g: &mut crate::game::Game, gd: GuardId, shooter: Option<GuardId>) {
    if g.snatch.man != Some(gd) { return; }
    g.snatch.killer = Some(match shooter {
        Some(s) => SnatchKiller { side: String::from(g.gd[s].team.as_str()), who: g.gd[s].callsign.clone() },
        None => SnatchKiller { side: String::from("player"), who: String::from("Player") },
    });
}

/// The team's living characters: Sam and every operative unit that is up. (snatch.ts Snatch.teamOf)
pub fn snatch_team_of(g: &mut crate::game::Game) -> Vec<crate::snatch::SnatchTeamOfItem> {
    let mut out = Vec::new();
    let p = g.player;
    if !g.pl[p].down { out.push(SnatchTeamOfItem { c: g.pl[p].ch, gd: None }); }
    for &gd in &g.guards {
        let man = &g.gd[gd];
        if man.unit.is_some() && man.team == crate::teams::Team::Operative && man.state != crate::game::GuardState::Dead && g.ch[man.ch].alive {
            out.push(SnatchTeamOfItem { c: man.ch, gd: Some(gd) });
        }
    }
    out
}

/// A note on the run's log, cut to its last rows. (snatch.ts Snatch.note)
pub fn snatch_note(g: &mut crate::game::Game, what: &str) {
    g.snatch.notes += 1.0;
    let line = format!("{} {}", to_fixed_1(g.time), what);
    g.snatch.log.push(line);
    if g.snatch.log.len() > 60 { g.snatch.log.drain(0..20); }
}

/// Puts him on the floor for this run, everything of the last one cleared. (snatch.ts Snatch.stage)
pub fn snatch_stage(g: &mut crate::game::Game, n: &crate::scene_scenarios::SnatchData) {
    if let Some(man) = g.snatch.man { g.civilians.retain(|&c| c != man); }
    g.snatch.run = Some(g.mission.serial);
    g.snatch.end = None;
    g.snatch.why = String::new();
    g.snatch.lost = None;
    g.snatch.killer = None;
    g.snatch.hands_up = false;
    g.snatch.tied = false;
    g.snatch.times = SnatchTimes { ran: -1.0, up: -1.0, tied: -1.0, end: -1.0 };
    g.snatch.stats = SnatchStats { turned: 0.0, bolted: 0.0 };
    g.snatch.near_t = 0.0;
    g.snatch.alone_t = 0.0;
    g.snatch.seen = None;
    g.snatch.fear = None;
    g.snatch.said_out = false;
    g.snatch.last_exit = None;
    g.snatch.log.clear();
    g.snatch.notes = 0.0;
    crate::snatchout::snatch_out_reset(g);
    let def = crate::civilians::SpawnTargetDef {
        name: n.man.name.clone(),
        pos: n.man.pos,
        face: n.man.face,
        tint: n.man.tint,
        tint2: n.man.tint2,
        points: n.man.points.clone(),
        wait: n.man.wait.clone(),
    };
    g.snatch.man = Some(crate::civilians::spawn_target(g, &def));
    snatch_note(g, &format!("{} staged in his room", n.man.name));
}

/// Takes him off the floor. (snatch.ts Snatch.drop)
pub fn snatch_drop(g: &mut crate::game::Game) {
    if let Some(man) = g.snatch.man { g.civilians.retain(|&c| c != man); }
    g.snatch.man = None;
    g.snatch.run = None;
    g.snatch.end = None;
    crate::snatchout::snatch_out_reset(g);
}

/// The overlay's frame: stages him, ends the mission on him, and runs the hold the team has of him. (snatch.ts Snatch.tick)
pub fn snatch_tick(g: &mut crate::game::Game, dt: f64) {

    let Some(n) = snatch_data(g) else {
        if g.snatch.man.is_some() { snatch_drop(g); }
        return;
    };
    if !crate::roomclear::room_clear_scenario_active(g) || g.touring {
        if g.snatch.man.is_some() { snatch_drop(g); }
        return;
    }
    let stale = g.snatch.run != Some(g.mission.serial) || match g.snatch.man {
        None => true,
        Some(man) => !g.civilians.contains(&man),
    };
    if stale { snatch_stage(g, &n); }
    if g.snatch.end.is_some() { return; }
    let gd = g.snatch.man.unwrap();
    let civ = g.gd[gd].civ.unwrap();
    let c = g.gd[gd].ch;
    let name = g.cv[civ].name.clone();
    if g.cv[civ].state != crate::civilians::CivState::Work && g.cv[civ].state != crate::civilians::CivState::Startled && g.snatch.times.ran < 0.0 { g.snatch.times.ran = g.time; }
    if g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Wiped) { return; }

    // the two ways it is lost
    if g.cv[civ].state == crate::civilians::CivState::Gone {
        let by = g.snatch.last_exit.clone().unwrap_or_else(|| String::from("a way nobody had a man on"));
        snatch_lose(g, SnatchLoseHow::Gone, &format!("{} got out by {}", name, by));
        return;
    }
    if g.gd[gd].state == crate::game::GuardState::Dead && g.gd[gd].down_kind == Some(crate::game::DownKind::Shot) {
        // no round of anybody's was on him: a blast killed him, and its thrower is named on killer_id after the fact
        let blast = g.snatch.killer.is_none();
        if blast {
            let killer_id = g.gd[gd].killer_id;
            let by = if killer_id >= 0.0 { g.guards.iter().copied().find(|&x| g.ch[g.gd[x].ch].id == killer_id) } else { None };
            g.snatch.killer = match by {
                Some(by) => Some(SnatchKiller { side: String::from(g.gd[by].team.as_str()), who: g.gd[by].callsign.clone() }),
                None if g.gd[gd].down_by == Some(crate::game::DownBy::Player) => Some(SnatchKiller { side: String::from("player"), who: String::from("Player") }),
                None => None,
            };
        }
        let what = if blast { "grenade" } else { "round" };
        let whose = match &g.snatch.killer {
            Some(k) if k.side == "player" => format!("your {}", what),
            Some(k) if k.side == "operative" => format!("{}'s {}", k.who, what),
            Some(_) => format!("one of their own men's {}s", what),
            None => format!("a {}", if blast { "blast" } else { "round" }),
        };
        snatch_lose(g, SnatchLoseHow::Dead, &format!("{} is dead — {}", name, whose));
        return;
    }

    // won: he is in the van, or the house is the team's and he is in hand
    let van = crate::snatchout::snatch_out_ruled(g);
    if van {
        if crate::snatchout::snatch_out_tick(g, dt) { return; }
    } else if g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Taken) && snatch_in_hand(g) {
        g.snatch.end = Some(SnatchEnd::Won);
        g.snatch.why = format!("{} taken alive", name);
        g.snatch.times.end = g.time;
        snatch_note(g, "WON");
        crate::game::game_msg(g, &format!("MISSION COMPLETE — {} taken alive and the house is yours. press Shift+Enter to restart", name));
        return;
    }
    if g.gd[gd].state == crate::game::GuardState::Dead {
        if !g.snatch.said_out {
            g.snatch.said_out = true;
            snatch_note(g, "out cold");
            let line = if van {
                format!("{} is out cold but breathing, and he's yours. carry him to {}", name, n.van.as_ref().unwrap().name)
            } else if g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Taken) {
                format!("{} is out cold but breathing, and he's yours", name)
            } else {
                format!("{} is out cold but breathing, and he's yours. now clear the rest of the house", name)
            };
            crate::game::game_msg(g, &line);
        }
        return;
    }
    if g.gd[gd].held.is_some() {
        g.snatch.hands_up = false;
        g.snatch.seen = Some(v3::copy(g.ch[c].pos));
        return;
    }
    // tied stays tied: Sam's arm letting go of him, or his own 'stay down' row with its clock, sets nobody running
    if g.snatch.tied {
        if g.cv[civ].state != crate::civilians::CivState::Cower || !g.cv[civ].complied || g.cv[civ].comply_until != f64::INFINITY {
            crate::civilians::civilian_tie(g, gd);
        }
        return;
    }

    // who of the team has a line to him (all round, whatever the light) and whom he makes out (his own eye, by the light on the man)
    let eye = bone_or(g, c, crate::character::Bone::Head, 1.5);
    let mut near: Option<CharId> = None;
    let mut near_d = f64::INFINITY;
    let mut in_sight: Vec<(CharId, f64)> = Vec::new();
    for item in snatch_team_of(g) {
        let (m, mate) = (item.c, item.gd);
        let d = v3::dist_xz(g.ch[m].pos, g.ch[c].pos);
        if d <= g.k.snatch.see_m && d < near_d {
            let chest = bone_or(g, m, crate::character::Bone::Chest, 1.1);
            let head = bone_or(g, m, crate::character::Bone::Head, 1.6);
            if !(crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) && crate::scene_collision::segment_blocked(g, eye, head, None, None, None)) {
                near_d = d;
                near = Some(m);
            }
        }
        let see_rate = g.k.snatch.see_rate;
        if crate::civilians::civilian_sees(g, gd, m, mate, see_rate) { in_sight.push((m, d)); }
    }
    js::sort_by(&mut in_sight, |a, b| a.1 - b.1);
    if let Some(m) = near {
        g.snatch.seen = Some(v3::copy(g.ch[c].pos));
        g.snatch.fear = Some(v3::copy(g.ch[m].pos));
    }
    // a promise Sam took from him by hand is his hands up as well; one that something else took off him is not
    if g.snatch.hands_up && (g.cv[civ].state != crate::civilians::CivState::Cower || !g.cv[civ].complied) { g.snatch.hands_up = false; }
    if !g.snatch.hands_up && g.cv[civ].state == crate::civilians::CivState::Cower && g.cv[civ].complied {
        g.snatch.hands_up = true;
        g.snatch.near_t = 0.0;
        g.snatch.alone_t = 0.0;
        if g.snatch.times.up < 0.0 { g.snatch.times.up = g.time; }
    }
    if g.snatch.hands_up {
        g.cv[civ].comply_until = f64::INFINITY;
        match near {
            Some(m) if near_d <= g.k.snatch.cuff_m => {
                g.snatch.near_t += dt;
                g.snatch.alone_t = 0.0;
                if g.snatch.near_t >= g.k.snatch.cuff_secs { snatch_tie(g, m); }
            }
            Some(_) => {
                g.snatch.alone_t = 0.0;
                g.snatch.near_t = 0.0;
            }
            None => {
                g.snatch.near_t = 0.0;
                g.snatch.alone_t += dt;
                if g.snatch.alone_t >= g.k.snatch.bolt_secs {
                    g.snatch.hands_up = false;
                    g.snatch.alone_t = 0.0;
                    g.snatch.stats.bolted += 1.0;
                    snatch_note(g, "nobody on him: he bolts");
                    let at = g.snatch.fear.unwrap_or(g.ch[c].pos);
                    crate::civilians::civilian_bolt(g, gd, at);
                }
            }
        }
        return;
    }
    // he gives up to one of the team close by with a line to him: from any side
    if let Some(m) = near {
        if near_d <= g.k.snatch.give_up_m {
            snatch_give_up(g, m, "close");
            return;
        }
    }
    // quiet at his marks and he makes out one of the team: the fright the civilians give Sam with a gun out
    if (g.cv[civ].state == crate::civilians::CivState::Work || g.cv[civ].state == crate::civilians::CivState::Startled) && !in_sight.is_empty() {
        let at = v3::copy(g.ch[in_sight[0].0].pos);
        crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Gun, at, true);
        return;
    }
    // running at a way out and he makes out one of the team between him and it
    let target = if g.cv[civ].state == crate::civilians::CivState::Flee { g.cv[civ].target.clone() } else { None };
    if let Some(t) = target {
        if t.kind == crate::civilians::CivTargetKind::Exit {
            g.snatch.last_exit = Some(t.name.clone());
            let mine = v3::dist_xz(g.ch[c].pos, t.pos);
            let ahead = in_sight.iter().copied().find(|x| v3::dist_xz(g.ch[x.0].pos, t.pos) < mine - 0.5);
            if let Some((m, d)) = ahead {
                if d <= g.k.snatch.halt_m {
                    snatch_give_up(g, m, "cut off");
                    return;
                }
                let key = exit_spot_key(g, t.spot.as_ref().unwrap());
                g.cv[civ].bad_targets.add(key);
                g.cv[civ].target = None;
                g.snatch.stats.turned += 1.0;
                snatch_note(g, &format!("turned from {}: a man on it", t.name));
            }
        }
    }
}

/// His hands go up to this man. (snatch.ts Snatch.giveUp)
pub fn snatch_give_up(g: &mut crate::game::Game, to: CharId, how: &str) {
    let gd = g.snatch.man.unwrap();
    let at = g.ch[to].pos;
    crate::civilians::civilian_yield(g, gd, at);
    g.snatch.hands_up = true;
    g.snatch.near_t = 0.0;
    g.snatch.alone_t = 0.0;
    if g.snatch.times.up < 0.0 { g.snatch.times.up = g.time; }
    snatch_note(g, &format!("hands up ({})", how));
    if let Some(mate) = mate_of(g, to) {
        crate::game::game_say(g, mate, "hands! on your head! down!", None, None, None);
    }
}

/// This man ties him, for good. (snatch.ts Snatch.tie)
pub fn snatch_tie(g: &mut crate::game::Game, by: CharId) {
    let gd = g.snatch.man.unwrap();
    let name = g.cv[g.gd[gd].civ.unwrap()].name.clone();
    g.snatch.tied = true;
    g.snatch.times.tied = g.time;
    crate::civilians::civilian_tie(g, gd);
    snatch_note(g, "tied");
    if let Some(mate) = mate_of(g, by) {
        crate::game::game_say(g, mate, &format!("{} secured.", name), Some(true), None, None);
    }
    let line = if crate::snatchout::snatch_out_ruled(g) {
        let van = snatch_data(g).unwrap().van.unwrap().name;
        format!("{} is secured. now get him to {}", name, van)
    } else if g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Taken) {
        format!("{} is secured", name)
    } else {
        format!("{} is secured. now clear the rest of the house", name)
    };
    crate::game::game_msg(g, &line);
}

/// He is in the van. (snatch.ts Snatch.win)
pub fn snatch_win(g: &mut crate::game::Game, why: &str) {
    g.snatch.end = Some(SnatchEnd::Won);
    g.snatch.why = String::from(why);
    g.snatch.times.end = g.time;
    snatch_note(g, "WON");
    crate::game::game_msg(g, &format!("MISSION COMPLETE — {}. press Shift+Enter to restart", why));
}

/// He got out or he is dead: the mission fails the way a civilian's death can. (snatch.ts Snatch.lose)
pub fn snatch_lose(g: &mut crate::game::Game, how: crate::snatch::SnatchLoseHow, why: &str) {
    g.snatch.end = Some(SnatchEnd::Lost);
    g.snatch.lost = Some(match how {
        SnatchLoseHow::Gone => SnatchLost::Gone,
        SnatchLoseHow::Dead => SnatchLost::Dead,
    });
    g.snatch.why = String::from(why);
    g.snatch.times.end = g.time;
    snatch_note(g, &format!("LOST — {}", why));
    crate::snatchout::snatch_out_let_go(g);
    let stage = g.mission.stage;
    let p = g.player;
    if stage != crate::game::MissionStage::Failed && stage != crate::game::MissionStage::Done && !g.pl[p].down {
        g.mission.stage = crate::game::MissionStage::Failed;
        g.mission.drive_t = -1.0;
        g.mission.down_card_at = if g.mission_in_tour { -1.0 } else { g.time + crate::mission::DOWN_CARD_SECS };
    }
    crate::game::game_msg(g, &format!("MISSION FAILED — {}. press Enter to restart", why));
}

/// His line and marker while he is loose or it is over; None = the room clear's own. (snatch.ts Snatch.objective)
pub fn snatch_objective(g: &mut crate::game::Game) -> Option<crate::snatch::SnatchObjective> {
    let man = g.snatch.man;
    if !snatch_active(g) { return None; }
    let gd = man?;
    let n = snatch_data(g).unwrap();
    let civ = g.gd[gd].civ.unwrap();
    let name = g.cv[civ].name.clone();
    if g.snatch.end == Some(SnatchEnd::Won) {
        let text = if g.snatch.out.in_at >= 0.0 { g.snatch.why.clone() } else { format!("{} taken alive — the house is yours", name) };
        return Some(SnatchObjective { text, pos: None });
    }
    if g.snatch.end == Some(SnatchEnd::Lost) {
        return Some(SnatchObjective { text: format!("mission failed — {}", g.snatch.why), pos: None });
    }
    if g.room_clear.end == Some(crate::roomclear::RoomClearEnd::Wiped) { return None; }
    if snatch_in_hand(g) {
        return crate::snatchout::snatch_out_objective(g).map(|o| SnatchObjective { text: o.text, pos: o.pos });
    }
    if g.snatch.hands_up {
        return Some(SnatchObjective {
            text: format!("objective ▸ {} has his hands up, so get someone to him before he runs again", name),
            pos: Some(v3::copy(g.ch[g.gd[gd].ch].pos)),
        });
    }
    let loud = g.cv[civ].state != crate::civilians::CivState::Work && g.cv[civ].state != crate::civilians::CivState::Startled;
    let text = if loud {
        format!("objective ▸ {} is running — take him alive", name)
    } else {
        format!("objective ▸ take {} alive. he'll run for {} the moment it goes loud", name, n.exits[0].name)
    };
    Some(SnatchObjective { text, pos: Some(v3::copy(g.snatch.seen.unwrap_or(n.man.pos))) })
}

/// One line for the tally. (snatch.ts Snatch.summary)
pub fn snatch_summary(g: &mut crate::game::Game) -> String {
    let Some(gd) = g.snatch.man else { return String::from("snatch: none on this level") };
    if !has_data(g) { return String::from("snatch: none on this level"); }
    let civ = g.gd[gd].civ.unwrap();
    let st = if let Some(end) = g.snatch.end {
        if g.snatch.why.is_empty() { String::from(end.as_str()) } else { format!("{} ({})", end.as_str(), g.snatch.why) }
    } else if g.snatch.tied {
        String::from("tied")
    } else if g.snatch.hands_up {
        String::from("hands up")
    } else {
        String::from(g.cv[civ].state.as_str())
    };
    format!("snatch: {} · {} · turned {} · bolted {}", g.cv[civ].name, st, num_str(g.snatch.stats.turned), num_str(g.snatch.stats.bolted))
}

/// The block against the built floor: he and his ways out stand on floor a man can walk, and every way out can be walked to from where he keeps. (snatch.ts checkSnatch)
pub fn check_snatch(g: &mut crate::game::Game) -> Vec<String> {
    let Some(n) = g.level.scenarios.as_ref().and_then(|s| s.snatch.clone()) else { return Vec::new() };
    let mut out: Vec<String> = Vec::new();
    let mut pts: Vec<(String, V3)> = vec![(String::from("man.pos"), n.man.pos)];
    if let Some(points) = &n.man.points {
        for (i, p) in points.iter().enumerate() { pts.push((format!("man.points[{}]", i), *p)); }
    }
    for (i, e) in n.exits.iter().enumerate() { pts.push((format!("exits[{}] {}", i, e.name), e.pos)); }
    for (w, p) in &pts {
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
            out.push(format!("scenarios.snatch.{}: ({}, {}) is not floor a man can stand on", w, num_str(p[0]), num_str(p[2])));
        }
    }
    let mut walk: Vec<f64> = Vec::new();
    for e in &n.exits {
        match crate::scene_collision::nav_grid_find_path(g, n.man.pos, e.pos, None, None) {
            Some(w) if !w.is_empty() => {
                let mut l = 0.0;
                let mut q = n.man.pos;
                for p in w {
                    l += v3::dist_xz(q, p);
                    q = p;
                }
                walk.push(l);
            }
            _ => {
                out.push(format!("scenarios.snatch: no way on foot from where he keeps to {}", e.name));
                walk.push(f64::INFINITY);
            }
        }
    }
    if let Some(van) = &n.van {
        if crate::scene_collision::nav_grid_is_blocked(g, van.pos[0], van.pos[2]) {
            out.push(format!("scenarios.snatch.van.pos: ({}, {}) is not floor a man can stand on", num_str(van.pos[0]), num_str(van.pos[2])));
        } else if !crate::scene_collision::nav_grid_find_path(g, n.man.pos, van.pos, None, None).map_or(false, |w| !w.is_empty()) {
            out.push(format!("scenarios.snatch: no way on foot from where he keeps to {}", van.name));
        }
    }
    if walk.iter().any(|&l| l < walk[0] - 0.5) {
        out.push(format!("scenarios.snatch.exits[0] ({}) is not his nearest way out on foot: the objective's line and the page name the first as the one he runs for", n.exits[0].name));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The strings JavaScript gives (node: `x.toFixed(1)`), ties to the larger digit where `{:.1}` goes to the even one.
    #[test]
    fn to_fixed_1_is_javascripts() {
        for (x, s) in [
            (0.0, "0.0"), (-0.0, "0.0"), (0.25, "0.3"), (0.75, "0.8"), (1.25, "1.3"), (2.5, "2.5"), (12.75, "12.8"),
            (1.05, "1.1"), (1.15, "1.1"), (0.04, "0.0"), (0.05, "0.1"), (59.99, "60.0"), (9.95, "9.9"), (9.75, "9.8"),
            (123.456, "123.5"), (-0.25, "-0.3"), (-1.25, "-1.3"), (99.75, "99.8"),
        ] {
            assert_eq!(to_fixed_1(x), s, "{}", x);
        }
    }

    #[test]
    fn a_number_in_a_template_reads_as_javascripts() {
        assert_eq!(num_str(3.0), "3");
        assert_eq!(num_str(-0.0), "0");
        assert_eq!(num_str(-12.5), "-12.5");
        assert_eq!(num_str(0.1), "0.1");
    }

    #[test]
    fn a_length_is_in_utf16_units() {
        assert_eq!(js_len("hands"), 5.0);
        assert_eq!(js_len("—"), 1.0);
        assert_eq!(js_len("😀"), 2.0);
    }
}
