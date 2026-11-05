//! src/game/snatchout.ts lines 1 to 262
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Distance on the floor. (snatchout.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// The player a row is built for and a press is read from. (snatchout.ts who)
pub fn who(g: &mut crate::game::Game) -> PlayerId { g.player }

/// Nothing led, nobody planted: the field initialisers; the constructor only keeps `g` and `S`. (snatchout.ts SnatchOut.constructor)
pub fn snatch_out_new(g: &mut crate::game::Game) -> crate::snatchout::SnatchOut { SnatchOut::blank() }

/// A fresh staging: nobody led, nobody planted. (snatchout.ts SnatchOut.reset)
pub fn snatch_out_reset(g: &mut crate::game::Game) {
    let o = &mut g.snatch.out;
    o.lead = None;
    o.led = false;
    o.sent = false;
    o.in_at = -1.0;
    o.jobs.clear();
    o.last.clear();
    o.told.clear();
    o.coming = None;
    o.moving = false;
    o.bounds = 0.0;
    o.hot = None;
    o.turned.clear();
}

/// The spot at the van's open doors (None: this level's block names no van). (snatchout.ts SnatchOut.van)
pub fn snatch_out_van(g: &mut crate::game::Game) -> Option<V3> {
    crate::snatch::snatch_block(g).and_then(|n| n.van).map(|v| v.pos)
}

/// This level's snatch ends in the van. (snatchout.ts SnatchOut.on)
pub fn snatch_out_on(g: &mut crate::game::Game) -> bool { snatch_out_van(g).is_some() }

/// ...and it is running. (snatchout.ts SnatchOut.ruled)
pub fn snatch_out_ruled(g: &mut crate::game::Game) -> bool {
    snatch_out_van(g).is_some() && crate::snatch::snatch_active(g)
}

/// He can be given the word: tied, on his feet or down, in nobody's arm. (snatchout.ts SnatchOut.free)
pub fn snatch_out_free(g: &mut crate::game::Game) -> bool {
    if !snatch_out_ruled(g) { return false; }
    let Some(gd) = g.snatch.man else { return false; };
    g.snatch.end.is_none() && g.snatch.tied && g.gd[gd].state != crate::game::GuardState::Dead && g.gd[gd].held.is_none()
}

/// The civilians' own rows on him stand down once the word can be given. (snatchout.ts SnatchOut.owns)
pub fn snatch_out_owns(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.snatch.man == Some(gd) && snatch_out_free(g)
}

/// The men the bounds have planted right now. (snatchout.ts SnatchOut.planted)
pub fn snatch_out_planted(g: &mut crate::game::Game) -> Vec<GuardId> { g.snatch.out.jobs.keys() }

/// The row on him in reach. (snatchout.ts SnatchOut.rows)
pub fn snatch_out_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::snatchout::SnatchOutRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    if !snatch_out_free(g) { return; }
    let pl = who(g);
    let gd = g.snatch.man.unwrap();
    let civ = g.gd[gd].civ.unwrap();
    let hp = g.ch[g.gd[gd].ch].pos;
    let d = flat(p, hp);
    if d > g.k.snatchout.marker_m || crate::fogofwar::fog_of_war_hidden(g, gd) { return; }
    let near = d < g.k.snatchout.reach_m && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [hp[0], 1.0, hp[2]], None);
    let can = near && !g.pl[pl].sprinting;
    let it = crate::game::Interactable {
        kind: crate::game::InteractableKind::Guard,
        pos: v3::add(hp, [0.0, 1.35, 0.0]),
        line1: format!("{} · the {}", g.cv[civ].name, g.cv[civ].label),
        line2: String::from(if can { "" } else if near { "stop sprinting" } else { "get closer" }),
        in_reach: can,
        off: false,
        progress: None,
        target: None,
        door: None,
        guard: Some(gd),
        verb: None,
        running: None,
        close: None,
        past: None,
        framed: None,
    };
    let verbs = if can && !busy {
        let made = if g.snatch.out.lead.is_some() {
            row(g, it.clone(), crate::game::Verb::Stay, String::from("stay here"), None, None)
        } else {
            row(g, it.clone(), crate::game::Verb::Lead, String::from("on your feet — with me"), None, None)
        };
        vec![made]
    } else {
        Vec::new()
    };
    offer(g, it, verbs);
}

/// The word: he follows `by`, or (None) stays where he stands. False = refused. (snatchout.ts SnatchOut.word)
pub fn snatch_out_word(g: &mut crate::game::Game, by: Option<CharId>) -> bool {
    if !snatch_out_free(g) { return false; }
    let gd = g.snatch.man.unwrap();
    let gap = g.k.snatchout.gap_m;
    crate::civilians::hostage_lead(g, gd, by, gap);
    g.snatch.out.lead = by;
    g.snatch.out.sent = false;
    g.snatch.out.moving = false;   // each word is a fresh start for the bounds
    if by.is_some() { g.snatch.out.led = true; }
    crate::snatch::snatch_note(g, if by.is_some() { "lead" } else { "stay" });
    true
}

/// Once a frame from snatch.ts tick: this frame's Space on his row, the last steps, the van. True = he is in it. (snatchout.ts SnatchOut.tick)
pub fn snatch_out_tick(g: &mut crate::game::Game, dt: f64) -> bool {
    let gd = g.snatch.man.unwrap();
    let van = snatch_out_van(g).unwrap();
    let pl = who(g);
    let act = if g.pl[pl].down { None } else { g.act.clone() };
    if let Some(act) = act {
        let lead = act.verb == Some(crate::game::Verb::Lead);
        if act.kind == crate::game::InteractableKind::Guard && act.guard == Some(gd) && (lead || act.verb == Some(crate::game::Verb::Stay)) {
            let by = if lead { Some(g.pl[pl].ch) } else { None };
            snatch_out_word(g, by);
        }
    }
    if let Some(c) = g.snatch.out.lead {
        if snatch_out_downed(g, c) { snatch_out_word(g, None); }   // nothing is won behind a man on the floor
    }
    // in Sam's arm, or out cold in his hands: he follows nobody, and Sam at the doors puts him in
    if g.gd[gd].held.is_some() || g.gd[gd].state == crate::game::GuardState::Dead {
        g.snatch.out.lead = None;
        g.snatch.out.sent = false;
        if (g.gd[gd].held.is_some() || g.pl[pl].dragging == Some(gd)) && !g.pl[pl].down && flat(g.ch[g.pl[pl].ch].pos, van) < g.k.snatchout.carry_m {
            snatch_out_into(g, false);
            return true;
        }
    } else if let (Some(lead), true) = (g.snatch.out.lead, g.snatch.tied) {
        let d_h = flat(g.ch[g.gd[gd].ch].pos, van);
        if !g.snatch.out.sent && flat(g.ch[lead].pos, van) < g.k.snatchout.send_m && d_h < g.k.snatchout.send_him_m {
            g.snatch.out.sent = true;
            crate::civilians::civilian_led_to(g, gd, Some(van));
            crate::snatch::snatch_note(g, "sent to the doors");
        }
        if g.snatch.out.sent && d_h < g.k.snatchout.in_m {
            snatch_out_into(g, true);
            return true;
        }
    }
    snatch_out_bound(g, dt);
    false
}

/// Is the one he follows down. (snatchout.ts SnatchOut.downed)
pub fn snatch_out_downed(g: &mut crate::game::Game, c_2: CharId) -> bool {
    let pl = who(g);
    if c_2 == g.pl[pl].ch { return g.pl[pl].down; }
    g.guards.iter().any(|x| g.gd[*x].ch == c_2 && g.gd[*x].state == crate::game::GuardState::Dead)
}

/// The debugger panel's switch thrown mid-run: SNATCHOUT.on is settled on, so its first line always returned. (snatchout.ts SnatchOut.switched)
pub fn snatch_out_switched(g: &mut crate::game::Game) {}

/// He is in the van: the mission is won. (snatchout.ts SnatchOut.into)
pub fn snatch_out_into(g: &mut crate::game::Game, walked: bool) {
    let gd = g.snatch.man.unwrap();
    let name = g.cv[g.gd[gd].civ.unwrap()].name.clone();
    let van = snatch_out_van(g).unwrap();
    g.snatch.out.in_at = g.time;
    g.snatch.out.lead = None;
    g.snatch.out.sent = false;
    if walked { crate::civilians::civilian_off(g, gd); }   // on his own feet: up and in, off the floor
    crate::audio_iface::game_audio_like_play(g, "doorBang", Some([van[0], 1.0, van[2]]), Some(0.5), None);
    snatch_out_let_go(g);
    let van_name = crate::snatch::snatch_block(g).unwrap().van.unwrap().name;
    crate::snatch::snatch_win(g, &format!("{} is in {}", name, van_name));
}

/// (snatchout.ts SnatchOut.fighting)
pub fn snatch_out_fighting(g: &mut crate::game::Game, gd: GuardId) -> bool {
    (g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert) || crate::roomfight::room_fight_engaged(g, gd)
}

/// Whoever is moving him: the one he follows, or the player with him in an arm or in his hands. (snatchout.ts SnatchOut.mover)
pub fn snatch_out_mover(g: &mut crate::game::Game) -> Option<CharId> {
    if let Some(lead) = g.snatch.out.lead { return Some(lead); }
    let pl = who(g);
    let gd = g.snatch.man?;
    if (g.gd[gd].held.is_some() || g.pl[pl].dragging == Some(gd)) && !g.pl[pl].down { Some(g.pl[pl].ch) } else { None }
}

/// An order of this module's, and the row it is from then on (what `last` and `jobs` know him by). (snatchout.ts SnatchOut.mine)
pub fn snatch_out_mine(g: &mut crate::game::Game, gd: GuardId, spec: &crate::snatchout::SnatchOutMineSpec) -> OrdersAssignmentId {
    crate::orders::squad_orders_assign(g, gd, &assign_spec(spec));
    let a = crate::orders::squad_orders_of(g, gd).unwrap();
    g.snatch.out.last.set(gd, Some(a));
    a
}

/// Forget a planted man, his order back on its own opening if it was turned. (snatchout.ts SnatchOut.drop)
pub fn snatch_out_drop(g: &mut crate::game::Game, gd: GuardId) {
    let a = g.snatch.out.jobs.get(gd).copied();
    let was = g.snatch.out.turned.get(gd).copied();
    if let (Some(a), Some(was)) = (a, was) {
        g.tab.orders_assignment[a].watch = was.watch;
        g.tab.orders_assignment[a].face = was.face;
    }
    g.snatch.out.jobs.delete(gd);
    g.snatch.out.turned.delete(gd);
}

/// A planted man whose order is still ours goes back to the file. (snatchout.ts SnatchOut.rejoin)
pub fn snatch_out_rejoin(g: &mut crate::game::Game, gd: GuardId) {
    let a = g.snatch.out.jobs.get(gd).copied();
    snatch_out_drop(g, gd);
    let Some(a) = a else { return; };
    if crate::orders::squad_orders_of(g, gd) == Some(a) && g.gd[gd].state != crate::game::GuardState::Dead {
        snatch_out_mine(g, gd, &SnatchOutMineSpec {
            order: String::from("fallIn"),
            post: None,
            follow: Some(9.0),
            face: None,
            stance: Some(crate::character::Stance::LowReady),
            upper: Some(crate::character::UpperMode::None),
            going: String::from("moving up"),
            there: String::from("with you"),
            ..Default::default()
        });
    }
}

/// It is over, or nobody is moving him: every man still ours goes back to the file. (snatchout.ts SnatchOut.letGo)
pub fn snatch_out_let_go(g: &mut crate::game::Game) {
    for gd in g.snatch.out.jobs.keys() { snatch_out_rejoin(g, gd); }
    g.snatch.out.coming = None;
    g.snatch.out.hot = None;
    g.snatch.out.moving = false;
}

/// Where a man plants: where he stands if that is good, else the nearest free floor he can walk straight to (None: none within `spotM`). (snatchout.ts SnatchOut.spot)
pub fn snatch_out_spot(g: &mut crate::game::Game, gd: GuardId, route: &[V3], taken: &[V3], own: bool) -> Option<V3> {
    let p = g.ch[g.gd[gd].ch].pos;
    let open = crate::cover::cover_ready(g);
    if spot_ok(g, p, route, taken, &open) { return Some([p[0], 0.0, p[2]]); }
    if own { return None; }
    let mut r = 0.6;
    while r <= g.k.snatchout.spot_m + 1e-6 {
        for i in 0..16 {
            let q = ring_point(p, i, r);
            if spot_ok(g, q, route, taken, &open)
                && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [q[0], 0.45, q[2]], None)
                && !crate::scene_collision::solid_blocked(g, [p[0], 1.3, p[2]], [q[0], 1.3, q[2]], None)
            {
                return Some(q);
            }
        }
        r += 0.4;
    }
    None
}

/// A pair plants: each man on a spot, facing the squad's own rule for men left holding together; the hold names nothing to watch. (snatchout.ts SnatchOut.plant)
pub fn snatch_out_plant(g: &mut crate::game::Game, men: &[GuardId]) {
    let mover = snatch_out_mover(g).unwrap();
    let mover_pos = g.ch[mover].pos;
    let van = snatch_out_van(g).unwrap();
    let mut route = vec![mover_pos];
    if let Some(path) = crate::scene_collision::nav_grid_find_path(g, mover_pos, van, None, None) { route.extend(path); }
    let mut spots: JsMap<GuardId, V3> = JsMap::new();
    for own in [true, false] {
        for &gd in men {
            if spots.has(gd) { continue; }
            let taken: Vec<V3> = spots.values().copied().collect();
            if let Some(q) = snatch_out_spot(g, gd, &route, &taken, own) { spots.set(gd, q); }
        }
    }
    if men.iter().any(|gd| !spots.has(*gd)) { return; }   // no floor for one of them here: the next half second asks again
    let at: Vec<V3> = men.iter().map(|gd| *spots.get(*gd).unwrap()).collect();
    let arcs = if men.len() > 1 { Some(crate::orders::squad_orders_hold_arcs(g, &at)) } else { None };
    crate::orders::squad_orders_new_group(g);
    for (i, &gd) in men.iter().enumerate() {
        let arc = arcs.as_ref().and_then(|a| a.get(i).copied().flatten());
        let yaw = match arc { Some(arc) => arc.yaw, None => g.ch[g.gd[gd].ch].aim_yaw };
        let job = snatch_out_mine(g, gd, &SnatchOutMineSpec {
            order: String::from("hold"),
            post: Some(at[i]),
            face: Some(yaw),
            watch: None,   // COVER.on: cover.ts deals him his opening
            stance: Some(crate::character::Stance::None),
            upper: Some(crate::character::UpperMode::Aim),
            going: String::from("holding"),
            there: String::from("holding — covering you"),
            ..Default::default()
        });
        g.snatch.out.jobs.set(gd, job);
    }
    g.snatch.out.bounds += 1.0;
    if g.snatch.out.bounds == 1.0 {
        crate::game::game_say(g, men[0], "holding here — get him out.", None, None, None);
    }
}

/// While one planted man fights, or one has just gone down, the others' guns go to where that is, and back `turnSecs` after the last of it. (snatchout.ts SnatchOut.turn)
pub fn snatch_out_turn(g: &mut crate::game::Game) {
    let turn_secs = g.k.snatchout.turn_secs;
    for gd in g.snatch.out.jobs.keys() {
        let t = crate::roomfight::room_fight_op_of(g, gd).and_then(|o| g.tab.roomfight_op[o].tgt);
        if let Some(t) = t {
            if g.gd[t].state != crate::game::GuardState::Dead {
                g.snatch.out.hot = Some(SnatchOutHot { at: v3::copy(g.ch[g.gd[t].ch].pos), until: g.time + turn_secs });
            }
        }
    }
    if let Some(h) = g.snatch.out.hot {
        if g.time >= h.until { g.snatch.out.hot = None; }
    }
    let at = g.snatch.out.hot.map(|h| h.at);
    let jobs: Vec<(GuardId, OrdersAssignmentId)> = g.snatch.out.jobs.iter().map(|(k, a)| (k, *a)).collect();
    for (gd, a) in jobs {
        if snatch_out_fighting(g, gd) { continue; }
        let was = g.snatch.out.turned.get(gd).copied();
        if let Some(at) = at {
            if was.is_none() {
                let (watch, face) = (g.tab.orders_assignment[a].watch, g.tab.orders_assignment[a].face);
                g.snatch.out.turned.set(gd, SnatchOutTurnedValue { watch, face });
            }
            g.tab.orders_assignment[a].watch = Some([at[0], 1.1, at[2]]);
            let from = match g.tab.orders_assignment[a].post { Some(post) => post, None => g.ch[g.gd[gd].ch].pos };
            g.tab.orders_assignment[a].face = Some(v3::yaw_to(from, at));
        } else if let Some(was) = was {
            g.tab.orders_assignment[a].watch = was.watch;
            g.tab.orders_assignment[a].face = was.face;
            g.snatch.out.turned.delete(gd);
        }
    }
}

/// The squad covers the walk out by bounds, twice a second. (snatchout.ts SnatchOut.bound)
pub fn snatch_out_bound(g: &mut crate::game::Game, dt: f64) {
    let mover = snatch_out_mover(g);
    let turn_secs = g.k.snatchout.turn_secs;
    for gd in g.snatch.out.jobs.keys() {
        if g.gd[gd].state != crate::game::GuardState::Dead { continue; }
        let killer = g.gd[gd].killer_id;
        let by = if killer >= 0.0 { g.guards.iter().copied().find(|x| g.ch[g.gd[*x].ch].id == killer) } else { None };
        if let Some(by) = by {
            g.snatch.out.hot = Some(SnatchOutHot { at: v3::copy(g.ch[g.gd[by].ch].pos), until: g.time + turn_secs });   // he went down at his post: the men still on theirs heard where from
        }
        snatch_out_drop(g, gd);
    }
    let Some(mover) = mover else {
        if !g.snatch.out.jobs.is_empty() || g.snatch.out.moving { snatch_out_let_go(g); }
        return;
    };
    if !g.snatch.out.moving {
        g.snatch.out.moving = true;
        g.snatch.out.told.clear();
        g.snatch.out.last.clear();
    }
    // an order that is not this module's since it last looked: the man is theirs, and is not asked again until he is next moved off
    for gd in crate::orders::squad_orders_men(g, Some(crate::orders::SquadOrdersMenSel::All), None) {
        let a = crate::orders::squad_orders_of(g, gd);
        if let Some(seen) = g.snatch.out.last.get(gd).copied() {
            if seen != a {
                g.snatch.out.told.add(gd);
                snatch_out_drop(g, gd);
            }
        }
        g.snatch.out.last.set(gd, a);
    }
    if m::floor(g.time * 2.0) == m::floor((g.time - dt) * 2.0) { return; }   // twice a second
    let l = g.ch[mover].pos;
    let van = snatch_out_van(g).unwrap();
    snatch_out_turn(g);
    // the groups this can move: each pair's men who are in the file (fall in) or planted by this
    let mut groups: JsMap<String, Vec<GuardId>> = JsMap::new();
    for gd in crate::orders::squad_orders_men(g, Some(crate::orders::SquadOrdersMenSel::All), None) {
        if g.snatch.out.told.has(gd) { continue; }
        if !g.snatch.out.jobs.has(gd) {
            let fall_in = match crate::orders::squad_orders_of(g, gd) {
                Some(a) => g.tab.orders_assignment[a].order == "fallIn",
                None => false,
            };
            if !fall_in { continue; }
        }
        let k = match crate::orders::squad_orders_pair_of(g, gd) {
            Some(pair) => String::from(pair.as_str()),
            None => solo_key(g.ch[g.gd[gd].ch].id),
        };
        groups.get_or_insert_with(k, Vec::new).push(gd);
    }
    let close_m = g.k.snatchout.close_m;
    let trail_m = g.k.snatchout.trail_m;
    if let Some(c) = g.snatch.out.coming.clone() {
        let done = match groups.get(c) {
            None => true,
            Some(men) => held(g, men) || near(g, men, l) < close_m,
        };
        if done { g.snatch.out.coming = None; }
    }
    let walk: Vec<(String, Vec<GuardId>)> = groups.iter().map(|(k, men)| (k, men.clone())).collect();
    // a pair left behind comes up: one at a time, and nobody is called off a fight
    if g.snatch.out.coming.is_none() {
        for (k, men) in &walk {
            if !held(g, men) || near(g, men, l) <= trail_m || any_fighting(g, men) { continue; }
            for &gd in men { snatch_out_rejoin(g, gd); }
            g.snatch.out.coming = Some(k.clone());
            crate::game::game_say(g, men[0], "moving.", None, None, None);
            break;
        }
    }
    // nobody planted: the pair in the file plants (at the word, the one farther from the van: the other walks him out)
    if !walk.iter().any(|(_, men)| held(g, men)) {
        let coming = g.snatch.out.coming.clone();
        let mut free: Vec<Vec<GuardId>> = walk.iter()
            .filter(|(k, men)| coming.as_deref() != Some(k.as_str()) && near(g, men, l) < trail_m)
            .map(|x| x.1.clone())
            .collect();
        {
            let seen: &Game = g;
            js::sort_by(&mut free, |a, b| if coming.is_some() { near(seen, a, l) - near(seen, b, l) } else { far(seen, b, van) - far(seen, a, van) });
        }
        if let Some(first) = free.first() { snatch_out_plant(g, first); }
    }
}

/// The objective's row once he is in hand: where to take him (None: not this rule's to say). (snatchout.ts SnatchOut.objective)
pub fn snatch_out_objective(g: &mut crate::game::Game) -> Option<crate::snatchout::SnatchOutObjective> {
    let ruled = snatch_out_ruled(g);
    let man = g.snatch.man;
    if !ruled || man.is_none() || g.snatch.end.is_some() || !crate::snatch::snatch_in_hand(g) { return None; }
    let gd = man.unwrap();
    let pl = who(g);
    let moving = g.snatch.out.lead.is_some() || g.gd[gd].held.is_some() || g.pl[pl].dragging == Some(gd);
    let van_name = crate::snatch::snatch_block(g).unwrap().van.unwrap().name;
    let text = format!("objective ▸ get {} to {}", g.cv[g.gd[gd].civ.unwrap()].name, van_name);
    let pos: V3 = if moving {
        v3::copy(snatch_out_van(g).unwrap())
    } else {
        let at = g.ch[g.gd[gd].ch].pos;
        [at[0], 0.0, at[2]]
    };
    Some(SnatchOutObjective { text, pos: Some(pos) })
}

/// His row, for player.ts buildInteractables. `rows` never hands an `over`, so none crosses between the two skeleton types. (snatchout.ts snatchOutRows)
pub fn snatch_out_rows_2(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::snatchout::SnatchOutRows2RowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    snatch_out_rows(g, p, busy, &mut |g, it, verb, text, hints, _over| row(g, it, verb, text, hints, None), offer);
}

/// `ok` in spot: free floor, in no doorway or gap, off his way out, on no mate's spot.
fn spot_ok(g: &mut Game, q: V3, route: &[V3], taken: &[V3], open: &[CoverOpeningId]) -> bool {
    if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) { return false; }
    let off_m = g.k.snatchout.off_m;
    let in_opening = open.iter().any(|o| {
        let o = &g.tab.cover_opening[*o];
        o.kind != crate::cover::OpeningKind::Rear && flat(q, o.at) < o.half + off_m
    });
    if in_opening { return false; }
    if !off_route(q, route, g.k.snatchout.lane_m) { return false; }
    !taken.iter().any(|t| flat(*t, q) < 0.9)
}

/// `offRoute` in spot: this far clear of every leg of the way out.
fn off_route(q: V3, route: &[V3], lane_m: f64) -> bool {
    for i in 1..route.len() {
        if leg_distance(q, route[i - 1], route[i]) < lane_m { return false; }
    }
    true
}

/// How far `q` is from the leg `a` to `b`, on the floor.
fn leg_distance(q: V3, a: V3, b: V3) -> f64 {
    let (dx, dz) = (b[0] - a[0], b[2] - a[2]);
    let l2 = js::or(dx * dx + dz * dz, 1.0);
    let t = m::max(0.0, m::min(1.0, ((q[0] - a[0]) * dx + (q[2] - a[2]) * dz) / l2));
    m::hyp2(q[0] - a[0] - dx * t, q[2] - a[2] - dz * t)
}

/// The `i`th of sixteen points on the ring of radius `r` round `p`.
fn ring_point(p: V3, i: i32, r: f64) -> V3 {
    let a = i as f64 * m::PI / 8.0;
    [p[0] + m::sin(a) * r, 0.0, p[2] + m::cos(a) * r]
}

/// `near` in bound: how close the group's nearest man is to the mover.
fn near(g: &Game, men: &[GuardId], l: V3) -> f64 {
    let d: Vec<f64> = men.iter().map(|x| flat(g.ch[g.gd[*x].ch].pos, l)).collect();
    m::min_n(&d)
}

/// `far` in bound: how far the group's farthest man is from the van.
fn far(g: &Game, men: &[GuardId], van: V3) -> f64 {
    let d: Vec<f64> = men.iter().map(|x| flat(g.ch[g.gd[*x].ch].pos, van)).collect();
    m::max_n(&d)
}

/// `held` in bound: one of the group is planted by this.
fn held(g: &Game, men: &[GuardId]) -> bool { men.iter().any(|x| g.snatch.out.jobs.has(*x)) }

/// `m.some(x => this.fighting(x))` in bound.
fn any_fighting(g: &mut Game, men: &[GuardId]) -> bool {
    for &gd in men {
        if snatch_out_fighting(g, gd) { return true; }
    }
    false
}

/// A man with no pair is a group of his own: the TypeScript's `m${gd.char.id}`.
fn solo_key(id: f64) -> String { format!("m{}", id) }

/// `mine`'s spec is `assign`'s own parameter type in the TypeScript; the skeleton made it two structs of one shape.
fn assign_spec(spec: &SnatchOutMineSpec) -> crate::orders::SquadOrdersAssignA {
    let s = spec.clone();
    crate::orders::SquadOrdersAssignA {
        post: s.post,
        going: s.going,
        order: s.order,
        face: s.face,
        there: s.there,
        t0: s.t0,
        follow: s.follow,
        watch: s.watch,
        door: s.door,
        strafe: s.strafe,
        stance: s.stance,
        upper: s.upper,
        stack_side: s.stack_side,
        move_: s.move_,
        set_line: s.set_line,
        group: s.group,
        set_said: s.set_said,
        goal: s.goal,
        arrived: s.arrived,
        held_set: s.held_set,
        held_fire: s.held_fire,
        arr_t: s.arr_t,
        moved_t: s.moved_t,
        light: s.light,
        low: s.low,
        yield_t: s.yield_t,
        face_now: s.face_now,
        search: s.search,
        search_t: s.search_t,
        via: s.via,
        anchor: s.anchor,
        mark: s.mark,
        mark_t: s.mark_t,
        had: s.had,
        back_n: s.back_n,
        shy: s.shy,
        thru: s.thru,
        via_for: s.via_for,
        via_t: s.via_t,
        pace: s.pace,
        spare: s.spare,
        from: s.from,
        foe: s.foe,
        crouch: s.crouch,
        lean: s.lean,
        fit: s.fit,
        tries: s.tries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // the hex below is the TypeScript's own, printed under bun with EXACT.on
    #[test]
    fn flat_matches_the_typescript_bits() {
        assert_eq!(flat([1.3, 9.0, -7.7], [4.1, 2.0, 0.35]).to_bits(), 0x40210bce1f4ecf69);
        assert_eq!(flat([0.1, 0.0, 0.2], [0.1, 5.0, 0.2]).to_bits(), 0);
    }

    #[test]
    fn leg_distance_matches_the_typescript_bits() {
        let (a, b) = ([0.3, 0.0, -0.4], [5.1, 0.0, 2.2]);
        assert_eq!(leg_distance([1.7, 0.0, 0.9], a, b).to_bits(), 0x3fde7b6c50a79491);   // inside the leg
        assert_eq!(leg_distance([9.0, 0.0, 9.0], a, b).to_bits(), 0x401f5b24230d20fe);   // past its end: t clamps
        assert_eq!(leg_distance([1.25, 0.0, 0.0], [2.0, 0.0, 2.0], [2.0, 0.0, 2.0]).to_bits(), 0x40011687a8ae14a3);   // a leg of no length: l2 is 1
    }

    #[test]
    fn ring_point_matches_the_typescript_bits() {
        let q = ring_point([1.5, 7.0, -2.5], 5, 1.4);
        assert_eq!(q[0].to_bits(), 0x400658f28884e4a5);
        assert_eq!(q[1].to_bits(), 0);
        assert_eq!(q[2].to_bits(), 0xc008493add2b5da4);
    }

    #[test]
    fn off_route_is_clear_only_of_every_leg() {
        let route = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 0.0, 10.0]];
        assert!(off_route([5.0, 0.0, 2.0], &route, 1.0));
        assert!(!off_route([5.0, 0.0, 0.5], &route, 1.0));
        assert!(!off_route([9.5, 0.0, 5.0], &route, 1.0));   // the second leg
        assert!(off_route([5.0, 0.0, 0.0], &route[..1], 1.0));   // only the mover's own spot: no leg
    }

    #[test]
    fn a_man_with_no_pair_keys_as_the_template_string_does() {
        assert_eq!(solo_key(3.0), "m3");
    }
}
