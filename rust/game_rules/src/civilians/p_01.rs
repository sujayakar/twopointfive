//! src/game/civilians.ts lines 1 to 414
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The office's phones and ways off the floor: every level with no places of its own reads these.   (civilians.ts CIV_SPOTS)
fn office_spots() -> crate::civilians::CivSpots {
    let t = crate::civilians::civ_spots_table();
    crate::civilians::CivSpots { phones: t.phones, exits: t.exits }
}

/// The phones and the ways off this level's floor: the snatch's or the quiet mission's own, else the office's.   (civilians.ts civSpots)
pub fn civ_spots(g: &mut crate::game::Game) -> crate::civilians::CivSpots {
    let Some(sc) = g.level.scenarios.as_ref() else { return office_spots(); };
    if sc.snatch.is_none() && sc.quiet.is_none() {
        return office_spots();
    }
    if let Some(o) = g.m.civilians.own_spots.as_ref() {
        return o.clone();
    }
    let o = own_spots_of(sc);
    g.m.civilians.own_spots = Some(o.clone());
    o
}

/// The level's own places, as civSpots keeps them in ownSpots: the snatch's if it has one, else the quiet mission's.   (civilians.ts:73,75)
fn own_spots_of(sc: &crate::scene_scenarios::LevelScenarios) -> crate::civilians::CivSpots {
    if let Some(n) = sc.snatch.as_ref() {
        let exits = n.exits.iter().map(|e| crate::civilians::CivSpot { name: e.name.clone(), pos: e.pos, face: 0.0 }).collect();
        return crate::civilians::CivSpots { phones: Vec::new(), exits };
    }
    let Some(q) = sc.quiet.as_ref() else { return crate::civilians::CivSpots { phones: Vec::new(), exits: Vec::new() }; };
    let stands = |list: &Option<Vec<crate::scene_scenarios::QuietStand>>| -> Vec<crate::civilians::CivSpot> {
        match list {
            Some(l) => l.iter().map(|s| crate::civilians::CivSpot { name: s.name.clone(), pos: s.pos, face: s.face }).collect(),
            None => Vec::new(),
        }
    };
    crate::civilians::CivSpots { phones: stands(&q.phones), exits: stands(&q.wayouts) }
}


/// Runs `fn_` with a private seeded die in the game die's place, so a civilian's construction moves the game's stream by no draw.   (civilians.ts withDice)
fn with_dice_on<T>(g: &mut Game, seed: f64, fn_: &mut dyn FnMut(&mut Game) -> T) -> T {
    let private = crate::math_dice::mulberry32(seed);
    let stood = g.dice;
    let block = &mut ::sim::state::state(g.sim).dice;
    let (word, count) = (block.s, block.n);
    block.s = private.s;
    block.n = private.n;
    let mut stand_in = crate::math_dice::sim_dice_new(seed);
    stand_in.home = stood.and_then(|d| d.home);
    g.dice = Some(stand_in);
    let out = fn_(g);
    g.dice = stood;
    let block = &mut ::sim::state::state(g.sim).dice;
    block.s = word;
    block.n = count;
    out
}

/// One line of what happened to her, kept on the record, the last few only.   (civilians.ts log)
pub fn log(g: &mut crate::game::Game, gd: GuardId, what: &str) {
    let line = format!("{} {}", js::to_fixed(g.time, 1), what);
    let c = g.gd[gd].civ.unwrap();
    g.cv[c].log.push(line);
    if g.cv[c].log.len() > 40 {
        g.cv[c].log.drain(0..10);
    }
    let row = crate::civilians::CiviliansModCIVLOGItem { t: g.time, who: g.cv[c].name.clone(), what: String::from(what) };
    g.m.civilians.civ_log.push(row);
    if g.m.civilians.civ_log.len() > 200 {
        g.m.civilians.civ_log.drain(0..50);
    }
}

/// A civilian comes into a game here and nowhere else; one more than a step of the doors can be handed is a level too big.   (civilians.ts join)
pub fn join(g: &mut crate::game::Game, gd: GuardId) -> GuardId {
    let most = crate::levelfit::civ_most(g);
    if g.civilians.len() as f64 >= most {
        let why = format!("more than {} civilians at once (its own, a rescue's hostages, the man a snatch is for), and a game holds no more men than one step of its doors can be handed (game/levelfit.ts civMost)", most);
        let said = crate::levelfit::level_too_big(g, &why);
        panic!("{}", said);
    }
    g.civilians.push(gd);
    gd
}

/// Fresh civilians on their first marks; nothing with the switch off or none authored.   (civilians.ts spawnCivilians)
pub fn spawn_civilians(g: &mut crate::game::Game) {
    g.civilians = Vec::new();
    if !g.tune.civilians || g.night_breach.active || g.rescue.active {
        return;
    }
    let defs = g.level.civilians.clone().unwrap_or_default();
    for (i, def) in defs.iter().enumerate() {
        let gd = make_civilian(g, def, i as f64);
        join(g, gd);
        g.stats.civilians.spawned += 1;
    }
}

/// Whoever a seat has hold of or is hauling let go of, then fresh civilians.   (civilians.ts resetCivilians)
pub fn reset_civilians(g: &mut crate::game::Game) {
    for s in g.seats.clone() {
        let pl = crate::cmd::seat_man(g, s);
        if let Some(h) = g.pl[pl].holding.as_ref().map(|h| h.g) {
            if g.gd[h].civ.is_some() {
                g.gd[h].held = None;
                let hc = g.gd[h].ch;
                g.ch[hc].anim.held_pose = None;
                g.pl[pl].holding = None;
                let pc = g.pl[pl].ch;
                g.ch[pc].anim.hold_pose = None;
            }
        }
        if let Some(d) = g.pl[pl].dragging {
            if g.gd[d].civ.is_some() {
                crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::game::game_toggle_drag(g, None));
            }
        }
        if let Some(t) = g.pl[pl].takedown.as_ref().map(|t| t.g) {
            if g.gd[t].civ.is_some() {
                g.pl[pl].takedown = None;
            }
        }
    }
    for cv in g.civilians.clone() {
        let c = g.gd[cv].ch;
        crate::sim_men_store::man_leaves(g, c);
    }
    let floor = floor_of(g);
    floor.evac_told = false;
    floor.greet_at = -1e9;
    floor.pending.clear();
    spawn_civilians(g);
}

/// A civilian of the usual make at her first mark, doing what she does there.   (civilians.ts makeCivilian)
pub fn make_civilian(g: &mut crate::game::Game, def: &crate::scene_level::CivilianDef, i: f64) -> GuardId {
    let id = CIV_ID0 + i;
    let route = crate::scene_level::PatrolRoute {
        name: def.name.clone(),
        points: def.points.iter().map(|p| v3::copy(*p)).collect(),
        wait: def.wait.clone(),
        weapon: None,
        keys: Some(Vec::new()),
    };
    let fl = crate::rig_props::torch_light_def(g, &format!("civlight{}", id), id, 0.0, 10.0, 20.0);
    g.tab.light[fl].enabled = false;
    g.tab.light[fl].intensity = 0.0;
    let yaw = match def.face.as_ref().and_then(|f| f.first().copied()) {
        Some(y) => y,
        None if route.points.len() > 1 => v3::yaw_to(route.points[0], route.points[1 % route.points.len()]),
        None => 0.0,
    };
    let wait0 = route.wait.first().copied().unwrap_or(0.0);
    let route = g.tab.route.push(route);
    let look = crate::game::GuardNewLook {
        tint2: Some(match def.tint2 { Some(t) => v3::copy(t), None => [0.06, 0.06, 0.08] }),
        yaw: Some(yaw),
        name: Some(def.name.clone()),
        team: Some(crate::teams::Team::Civilian),
    };
    let tint = v3::copy(def.tint);
    let seed = crate::math_dice::hash_str(&format!("civ:{}:{}", def.name, i));
    let (men, rig) = (g.men, g.rig);
    let gd = with_dice_on(g, seed, &mut |g: &mut Game| crate::game::guard_new(g, men, rig, id, route, -1, fl, tint, Some(&look)));
    g.gd[gd].foe = None;
    g.gd[gd].drawn = false;
    g.gd[gd].heard_up_to = g.event_seq;
    let worker = def.kind == crate::scene_level::CivilianDefKind::Worker;
    let civ = crate::civilians::Civ {
        def: def.clone(),
        name: def.name.clone(),
        label: String::from(if worker { "late worker" } else { "cleaner" }),
        she: String::from(if worker { "he" } else { "she" }),
        her: String::from(if worker { "him" } else { "her" }),
        state: CivState::Work,
        state_at: g.time,
        task_t: 0.0,
        rise_t: 0.0,
        seated: false,
        settle_t: -1.0,
        look_at: None,
        look_until: -1.0,
        shrug_at: -1e9,
        cause: None,
        cause_pos: None,
        cause_at: -1.0,
        armed: false,
        alarm_n: 0.0,
        seen_at: -1e9,
        seen_pos: None,
        saw_gun: false,
        target: None,
        retarget_at: -1.0,
        bad_targets: JsSet::new(),
        freeze_until: -1.0,
        pin_target: false,
        report_to: None,
        talk_t: 0.0,
        reported: false,
        phoned: false,
        acked: false,
        cower_at: None,
        complied: false,
        comply_until: -1.0,
        refled: false,
        found_t: 0.0,
        found_by: None,
        found_line_at: -1.0,
        found_told: false,
        lead: None,
        lead_gap: 0.0,
        lead_to: None,
        scream_at: -1e9,
        whimper_at: -1e9,
        greet_at: -1e9,
        last_line_at: -1e9,
        told: JsMap::new(),
        known_bodies: JsSet::new(),
        intel_n: 0,
        outside_call_at: -1.0,
        vis: 0.0,
        dice: crate::dice::Die::new(js::to_i32(crate::math_dice::hash_str(&format!("civdice:{}", def.name))) as u32),
        log: Vec::new(),
    };
    let c = g.cv.push(civ);
    g.gd[gd].civ = Some(c);
    g.gd[gd].wait = wait0;
    let body = g.gd[gd].ch;
    crate::character::character_update(g, body, 0.0);
    gd
}

/// A hostage: a civilian of the usual make at `pos`, down and told to stay down with no end to it.   (civilians.ts spawnHostage)
pub fn spawn_hostage(g: &mut crate::game::Game, def: &crate::civilians::SpawnHostageDef) -> GuardId {
    let kind = match def.kind {
        SpawnHostageDefKind::Cleaner => crate::scene_level::CivilianDefKind::Cleaner,
        SpawnHostageDefKind::Worker => crate::scene_level::CivilianDefKind::Worker,
    };
    let made = crate::scene_level::CivilianDef {
        name: def.name.clone(),
        kind,
        tint: def.tint,
        tint2: def.tint2,
        points: vec![v3::copy(def.pos)],
        wait: vec![0.0],
        tasks: None,
        face: Some(vec![def.face]),
    };
    let i = g.civilians.len() as f64;
    let gd = make_civilian(g, &made, i);
    join(g, gd);
    to_cower(g, gd, true);
    let c = g.gd[gd].civ.unwrap();
    g.cv[c].comply_until = m::INF;
    g.cv[c].cause = Some(CivCause::Gun);
    g.cv[c].armed = true;
    g.cv[c].label = String::from("hostage");
    gd
}

/// The man a snatch is for: a civilian of the usual make at his own marks, working them until something frightens him.   (civilians.ts spawnTarget)
pub fn spawn_target(g: &mut crate::game::Game, def: &crate::scene_scenarios::SnatchDataMan) -> GuardId {
    let pts: Vec<V3> = match def.points.as_ref() {
        Some(p) if !p.is_empty() => p.clone(),
        _ => vec![def.pos],
    };
    let made = crate::scene_level::CivilianDef {
        name: def.name.clone(),
        kind: crate::scene_level::CivilianDefKind::Worker,
        tint: def.tint,
        tint2: def.tint2,
        points: pts.iter().map(|p| v3::copy(*p)).collect(),
        wait: (0..pts.len()).map(|i| def.wait.as_ref().and_then(|w| w.get(i).copied()).unwrap_or(30.0)).collect(),
        tasks: None,
        face: Some(pts.iter().map(|_| def.face).collect()),
    };
    let i = g.civilians.len() as f64;
    let gd = make_civilian(g, &made, i);
    join(g, gd);
    let c = g.gd[gd].civ.unwrap();
    g.cv[c].label = String::from("man you came for");
    gd
}

/// He gives up where he stands, to the man at `to`: the complied cower with no end to it.   (civilians.ts civilianYield)
pub fn civilian_yield(g: &mut crate::game::Game, gd: GuardId, to: V3) {
    let c = g.gd[gd].civ.unwrap();
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || g.cv[c].state == CivState::Gone {
        return;
    }
    to_cower(g, gd, true);
    g.cv[c].comply_until = m::INF;
    g.cv[c].cause = Some(CivCause::Gun);
    g.cv[c].armed = true;
    g.cv[c].cause_pos = Some(v3::copy(to));
    g.cv[c].cause_at = g.time;
    civ_say(g, gd, "don't shoot! don't shoot!", None, None);
}

/// His hands were up and nobody stayed on him: the promise is off and he is frightened afresh, of `at`.   (civilians.ts civilianBolt)
pub fn civilian_bolt(g: &mut crate::game::Game, gd: GuardId, at: V3) {
    let c = g.gd[gd].civ.unwrap();
    if g.gd[gd].state == crate::game::GuardState::Dead || g.gd[gd].held.is_some() || g.cv[c].state != CivState::Cower {
        return;
    }
    g.cv[c].complied = false;
    g.cv[c].comply_until = -1.0;
    g.cv[c].refled = false;
    to_alarmed(g, gd, CivCause::Gun, v3::copy(at), true);
}

/// Does he make out this man of the team (`mate` None = Sam)? The civilians' own eye through the guards' eyeContact and sight gain.   (civilians.ts civilianSees)
pub fn civilian_sees(g: &mut crate::game::Game, gd: GuardId, pc: CharId, mate: Option<GuardId>, bar: f64) -> bool {
    let c = g.gd[gd].ch;
    let eye = match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]),
    };
    let look = crate::guards::look_of(g, c);
    let e = eye_of(g, gd);
    let smoke = if mate.is_some() { 1.0 } else { g.gd[gd].smoke_trans };
    let vis = crate::guards::eye_contact(
        g,
        crate::guards::Eye { pos: eye, look, range: e.range, half_deg: e.half_deg, near_m: e.near_m, near_deg: e.near_deg, v_half_deg: None },
        pc,
        mate.is_none(),
        false,
        smoke,
        None,
    );
    if !vis.visible {
        return false;
    }
    let air = if mate.is_some() {
        let chest = match g.ch[pc].bones.get(crate::character::Bone::Chest).copied() {
            Some(b) => b,
            None => v3::add(g.ch[pc].pos, [0.0, 1.1, 0.0]),
        };
        v3::clamp((crate::weather::air_trans(g, eye, chest) - 0.3) / 0.5, 0.0, 1.0)
    } else {
        1.0
    };
    let gain = match mate {
        Some(o) => crate::guards::sight_gain_on(g, gd, o, vis.dist),
        None => crate::guards::sight_gain(g, gd, vis.dist),
    };
    gain * g.k.civ.keen_k * air > bar
}

/// Tied where he is: the promise with no end, whatever state the tie found him in.   (civilians.ts civilianTie)
pub fn civilian_tie(g: &mut crate::game::Game, gd: GuardId) {
    let c = g.gd[gd].civ.unwrap();
    if g.gd[gd].state == crate::game::GuardState::Dead || g.cv[c].state == CivState::Gone {
        return;
    }
    if g.cv[c].state != CivState::Cower {
        to_cower(g, gd, true);
    }
    g.cv[c].complied = true;
    g.cv[c].comply_until = m::INF;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_office_table_is_as_written() {
        let t = office_spots();
        assert_eq!(t.phones[1].face.to_bits(), 0x3FF921FB54442D18);
        assert_eq!((t.phones.len(), t.exits.len()), (2, 2));
        assert_eq!(t.exits[1].name, "the outside door");
    }
}
