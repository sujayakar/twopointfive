//! src/game/rescue.ts lines 437 to 896
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small readers
/// his row of `defs`, a copy: read again after a call that may have written it
fn def_of(g: &Game, h: GuardId) -> Def { *g.rescue.defs.get(h).unwrap() }
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn pl_pos(g: &Game) -> V3 { g.ch[g.pl[g.player].ch].pos }
fn alive(g: &Game, gd: GuardId) -> bool { g.gd[gd].state != crate::game::GuardState::Dead }
/// inBounds, for a scenario's Rect
fn in_rect(p: V3, r: Option<crate::scene_scenarios::Rect>) -> bool {
    crate::squad::in_bounds(p, r.map(|b| crate::scene_level::BreachPlanBounds { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 }), None)
}
/// g.orders.log's last row, as `lastOrder` holds it: its serial
fn last_log_order(g: &Game) -> Option<u32> { g.orders.log.last().map(|l| l.serial) }
fn hot(g: &Game) -> bool { g.k.rescue.start == crate::knobs::RescueKStart::Hot }
fn say_radio(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, Some(true), None, None); }

/// (rescue.ts Rescue.note)
pub fn rescue_note(g: &mut crate::game::Game, what: &str) {
    let t = g.time;
    g.rescue.log.push(RescueLogItem { t, what: String::from(what) });
    if g.rescue.log.len() > 600 { g.rescue.log.drain(0..150); }
}

/// A line said once a staging. (rescue.ts Rescue.once)
pub fn rescue_once(g: &mut crate::game::Game, key: &str, text: &str) {
    if !g.rescue.said.has(String::from(key)) {
        g.rescue.said.add(String::from(key));
        crate::game::game_msg(g, text);
    }
}

/// (rescue.ts Rescue.over)
pub fn rescue_over(g: &mut crate::game::Game) -> bool { g.rescue.result.is_some() }

/// Dynamic tempo: the clock is running. (rescue.ts Rescue.dynamic)
pub fn rescue_dynamic(g: &mut crate::game::Game) -> bool { !g.rescue.t0.is_nan() }

/// What F does for a caller that is not the key. (rescue.ts Rescue.press)
pub fn rescue_press(g: &mut crate::game::Game) -> Option<crate::rescue::RescuePress> {
    if !g.rescue.active || g.pl[g.player].down { return None; }
    crate::doortools::door_tools_sync(g);
    if !g.door_tools.set.is_empty() {
        crate::doortools::door_tools_trigger(g, None);
        return Some(RescuePress::Blow);
    }
    match rescue_order_door(g) {
        Some(RescueOrderDoor::Stack) => Some(RescuePress::Stack),
        Some(RescueOrderDoor::Go) => Some(RescuePress::Go),
        Some(RescueOrderDoor::Bang) => Some(RescuePress::Bang),
        None => None,
    }
}

/// Sam's word on the hostage room. (rescue.ts Rescue.orderDoor)
pub fn rescue_order_door(g: &mut crate::game::Game) -> Option<crate::rescue::RescueOrderDoor> {
    if !g.rescue.active || rescue_over(g) || g.pl[g.player].down { return None; }
    crate::doortools::door_tools_sync(g);
    let hot_start = hot(g);
    if let Some(gate) = g.rescue.entry.as_ref().map(|e| e.gate) {
        if g.tab.doortools_door_gate[gate].go { return None; }
        g.tab.doortools_door_gate[gate].go = true;
        g.rescue.door_orders = 2.0;
        rescue_note(g, "order:go");
        crate::game::game_msg(g, "you: go");
        return Some(RescueOrderDoor::Go);
    }
    if g.rescue.called {
        if rescue_captor(g).is_none() { return None; }
        g.rescue.called = false;
    }
    let s = rescue_data(g)?;
    let fd = crate::doors::doors_by_name(g, &s.room.door);
    let sd = match &s.side { Some(side) => crate::doors::doors_by_name(g, &side.door), None => None };
    let fd = fd?;
    let on: Vec<DoorOpId> = g.door_tools.ops.iter().copied().filter(|&o| {
        let door = g.tab.doortools_door_op[o].door;
        door == fd || Some(door) == sd
    }).collect();
    let ch = on.iter().copied().find(|&o| g.tab.doortools_door_op[o].kind == crate::doortools::DoorToolKind::Charge);
    if let Some(ch) = ch {
        let door = g.tab.doortools_door_op[ch].door;
        let set = crate::doortools::door_tools_charge_on(g, door).is_some();
        if set || g.tab.doortools_door_op[ch].phase == crate::doortools::Phase::Place {
            let set = crate::doortools::door_tools_charge_on(g, door).is_some();
            crate::game::game_msg(g, if set { "the charge is set. press F to blow it" } else { "the charge is not set yet" });
            return None;
        }
    }
    if on.iter().any(|&o| g.tab.doortools_door_op[o].kind != crate::doortools::DoorToolKind::Outside) {
        crate::game::game_msg(g, "they are on that door already");
        return None;
    }
    let who = match rescue_pick(g) {
        Some(who) => who,
        None => {
            crate::game::game_msg(g, if g.rescue.ops.is_empty() { "you're alone, so you'll have to take the hostage room yourself" } else { "nobody's free for the hostage room. everyone still up is covering where you put them (X, then fall in)" });
            return None;
        }
    };
    let dyn_ = rescue_dynamic(g) || hot_start;
    let two = rescue_launch(g, fd, sd, &who.front, &who.side, dyn_)?;
    g.rescue.door_orders = if dyn_ { 2.0 } else { 1.0 };
    if hot_start { rescue_start_clock(g, "order"); }
    rescue_note(g, if dyn_ { "order:bang" } else { "order:stack" });
    let line = if dyn_ {
        let bang = match &g.rescue.entry {
            Some(e) => e.ops.iter().any(|e| g.tab.doortools_door_op[e.op].kind == crate::doortools::DoorToolKind::BangClear),
            None => false,
        };
        format!("you: {} and clear — hostage room{}", if bang { "bang" } else { "open" }, two)
    } else {
        format!("you: stack up — hostage room{}", two)
    };
    crate::game::game_msg(g, &line);
    Some(if dyn_ { RescueOrderDoor::Bang } else { RescueOrderDoor::Stack })
}

/// Sam put him there himself. (rescue.ts Rescue.pick, its `posted`)
fn posted(g: &mut Game, o: GuardId) -> bool {
    let a = match crate::orders::squad_orders_of(g, o) { Some(a) => a, None => return false };
    if g.tab.orders_assignment[a].post.is_none() { return false; }
    if g.rescue.auto.has(a) { return false; }
    !crate::rescueflow::rescue_flow_gave(g, Some(a))
}
/// posted, and on a 'cover' of his own: he stays on it. (rescue.ts Rescue.pick)
fn posted_cover(g: &mut Game, o: GuardId) -> bool {
    if !posted(g, o) { return false; }
    let a = crate::orders::squad_orders_of(g, o).unwrap();
    g.tab.orders_assignment[a].order == "cover"
}
/// The man of `l` in the room nearest the side door. (rescue.ts Rescue.pick, its `near`)
fn near(g: &Game, l: &[GuardId], sb: Option<crate::scene_scenarios::Rect>, sd: DoorId) -> Option<GuardId> {
    let fc = g.dr[sd].frame_centre;
    let mut inside: Vec<(GuardId, V3)> = l.iter().map(|&o| (o, pos_of(g, o))).filter(|x| in_rect(x.1, sb)).collect();
    js::sort_by(&mut inside, |a, b| flat(a.1, fc) - flat(b.1, fc));
    inside.first().map(|x| x.0)
}

/// Who F takes, and to which door. (rescue.ts Rescue.pick)
pub fn rescue_pick(g: &mut crate::game::Game) -> Option<crate::rescue::RescuePick> {
    let up: Vec<GuardId> = g.rescue.ops.iter().copied().filter(|&o| alive(g, o)).collect();
    if up.is_empty() { return None; }
    let data = rescue_data(g);
    let side_data = data.as_ref().and_then(|s| s.side.as_ref());
    let sd = match side_data { Some(sd) => crate::doors::doors_by_name(g, &sd.door), None => None };
    let sb = rescue_from_bounds(g);
    let mut side: Vec<GuardId> = Vec::new();
    if let (Some(sd), true, true) = (sd, sb.is_some(), rescue_bounds(g).is_some()) {
        let mut is_posted: Vec<GuardId> = Vec::new();
        for &o in &up { if posted(g, o) { is_posted.push(o); } }
        let mut first = near(g, &is_posted, sb, sd);
        if first.is_none() && in_rect(pl_pos(g), sb) {
            let mut free: Vec<GuardId> = Vec::new();
            for &o in &up { if !posted(g, o) { free.push(o); } }
            if free.len() >= 2 { first = near(g, &free, sb, sd); }
        }
        if let Some(first) = first {
            side.push(first);
            let pr = g.gd[first].unit.as_ref().and_then(|u| u.pair);
            let wants_pair = side_data.and_then(|s| s.pair).unwrap_or(false);
            let mate = if wants_pair && pr.is_some() {
                up.iter().copied().find(|&o| o != first && g.gd[o].unit.as_ref().and_then(|u| u.pair) == pr && in_rect(pos_of(g, o), sb))
            } else { None };
            if let Some(mate) = mate {
                let mut other = false;
                for &o in &up {
                    if o != first && o != mate && !posted_cover(g, o) { other = true; break; }
                }
                if other { side.push(mate); }
            }
        }
    }
    let mut front: Vec<GuardId> = Vec::new();
    for &o in &up {
        if !side.contains(&o) && !posted_cover(g, o) { front.push(o); }
    }
    if !front.is_empty() || !side.is_empty() { Some(RescuePick { front, side }) } else { None }
}

/// One door's order. (rescue.ts Rescue.launch, its `give`)
fn give(g: &mut Game, gate: DoorGateId, ops: &mut Vec<EntryOpsItem>, why: &mut Vec<String>, who: &[GuardId], d: DoorId, is_side: bool) {
    let kind = if g.door_tools.bangs > 0.0 { crate::doortools::DoorToolsOrderKind::BangClear } else { crate::doortools::DoorToolsOrderKind::OpenClear };
    let face = rescue_face_of(g, d);
    match crate::doortools::door_tools_order(g, kind, who, d, face, Some(gate), None) {
        crate::doortools::DoorToolsOrder::Text(text) => {
            let door = if is_side { "side door" } else if !crate::dialogue::is_office(g) { "main door" } else { "corridor door" };
            why.push(format!("{} — {}", door, text));
        }
        crate::doortools::DoorToolsOrder::DoorOp(op) => {
            let men = g.tab.doortools_door_op[op].men.clone();
            crate::rescueflow::rescue_flow_release(g, &men);
            ops.push(EntryOpsItem { op, side: is_side, seen: 0.0, kicked: false, ended: None });
            let mut i = 0;
            while i < g.tab.doortools_door_op[op].men.len() {
                let o = g.tab.doortools_door_op[op].men[i];
                rescue_round_by(g, o);
                i += 1;
            }
        }
    }
}

/// F's entry, given: the door tools' order on each door, through one gate. (rescue.ts Rescue.launch)
pub fn rescue_launch(g: &mut crate::game::Game, fd: DoorId, sd: Option<DoorId>, front: &[GuardId], side: &[GuardId], go: bool) -> Option<String> {
    let gate = crate::doortools::door_tools_together(g, Some(go));
    let mut ops: Vec<EntryOpsItem> = Vec::new();
    let mut why: Vec<String> = Vec::new();
    if !front.is_empty() { give(g, gate, &mut ops, &mut why, front, fd, false); }
    if let (false, Some(sd)) = (side.is_empty(), sd) { give(g, gate, &mut ops, &mut why, side, sd, true); }
    if !why.is_empty() || ops.is_empty() {
        let line = if why.is_empty() { String::from("nobody to take it") } else { why.join(" · ") };
        crate::game::game_msg(g, &format!("the hostage room: {}", line));
    }
    if ops.is_empty() { return None; }
    let is_bang = |g: &Game, e: &EntryOpsItem| g.tab.doortools_door_op[e.op].kind == crate::doortools::DoorToolKind::BangClear;
    let cans = ops.iter().filter(|e| is_bang(g, e)).count();
    if cans < ops.len() {
        let line = if cans > 0 {
            let side_can = ops.iter().find(|e| is_bang(g, e)).unwrap().side;
            let door = if side_can { "side" } else if crate::dialogue::is_office(g) { "corridor" } else { "main" };
            format!("one flashbang left, so it'll go in through the {} door and the teammates at the other door will go in after it", door)
        } else {
            String::from("no flashbangs left, so they'll go in without one")
        };
        crate::game::game_msg(g, &line);
    }
    let mut taken: Vec<GuardId> = Vec::new();
    for e in &ops { taken.extend(g.tab.doortools_door_op[e.op].men.iter().copied()); }
    let split = rescue_bounds(g).is_some() && (ops.iter().any(|e| e.side) || taken.len() == 1);
    let tail = if ops.len() > 1 { " · both doors" } else if ops[0].side { " · side door" } else { "" };
    let mut words: Vec<String> = Vec::new();
    for e in &ops {
        let names: Vec<String> = g.tab.doortools_door_op[e.op].men.iter().map(|&m| g.gd[m].callsign.clone()).collect();
        words.push(format!("{}={}", if e.side { "side" } else { "front" }, names.join("+")));
    }
    g.rescue.entry = Some(Entry { ops, gate, men: taken, split, set_t: f64::NAN, go_t: f64::NAN, in_t: f64::NAN });
    g.rescue.said.delete(String::from("set"));
    if split { rescue_note(g, &format!("split:{}", words.join(","))); }
    Some(String::from(tail))
}

/// Starts clock one; at a free start the men not on an order fall in. (rescue.ts Rescue.startClock)
pub fn rescue_start_clock(g: &mut crate::game::Game, why: &str) {
    if !g.rescue.t0.is_nan() { return; }
    g.rescue.t0 = g.time;
    rescue_note(g, &format!("clock:{}", why));
    let up: Vec<GuardId> = g.rescue.ops.iter().copied().filter(|&o| alive(g, o)).collect();
    if rescue_mode(g) == RescueMode::Free && !up.is_empty() {
        let p = pl_pos(g);
        let mut idle: Vec<GuardId> = Vec::new();
        for &o in &up { if !crate::orders::squad_orders_has(g, o) { idle.push(o); } }
        let target = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [p[0], 0.0, p[2]], man: None });
        if idle.len() == up.len() {
            crate::orders::squad_orders_give(g, "fallIn", Some(&target), Some(crate::orders::SquadOrdersGiveWho::All), None);
        } else {
            for &o in &idle {
                let at = match crate::orders::squad_orders_squad(g).iter().position(|&x| x == o) { Some(i) => i as f64, None => -1.0 };
                crate::orders::squad_orders_give(g, "fallIn", Some(&target), Some(crate::orders::SquadOrdersGiveWho::Num(at)), None);
            }
        }
        if !idle.is_empty() { g.rescue.last_order = last_log_order(g); }
    }
}

/// Once a frame from the game's update. (rescue.ts Rescue.tick)
pub fn rescue_tick(g: &mut crate::game::Game, dt: f64, input: InputId) {
    if !g.rescue.active { return; }
    let gone = g.rescue.ops.iter().chain(g.rescue.hostiles.iter()).any(|x| !g.guards.contains(x))
        || g.rescue.hostages.iter().any(|h| !g.civilians.contains(h));
    if gone {
        crate::game::game_restart_encounter(g);
        if !g.rescue.active { return; }
    }
    if g.rescue.cut { g.rescue.cut = false; crate::seats::cut_to_men(g); }
    rescue_tick_entry(g);
    if crate::input::input_hit(g, input, "KeyF") { rescue_order_door(g); }
    rescue_hear(g);
    rescue_defenders(g);
    crate::rescueflush::rescue_flush_tick(g);
    crate::rescueflow::rescue_flow_tick(g);
    rescue_search(g);
    g.door_tools.lights = Some(true);
    let lit = rescue_dynamic(g);
    let mut i = 0;
    while i < g.rescue.ops.len() {
        let o = g.rescue.ops[i];
        if let Some(u) = g.gd[o].unit.as_mut() { u.light = lit; }
        i += 1;
    }
    rescue_clock(g, dt);
    crate::rescueend::rescue_end_tick(g);
    rescue_outcome(g);
    rescue_voices(g);
}

/// This frame's loud things: the last one the team did is where a roamer goes. (rescue.ts Rescue.hear)
pub fn rescue_hear(g: &mut crate::game::Game) {
    let mut mine: Vec<f64> = vec![g.ch[g.pl[g.player].ch].id];
    for &o in &g.rescue.ops { mine.push(g.ch[g.gd[o].ch].id); }
    let mut i = 0;
    while i < g.events.len() {
        let e = g.events[i];
        i += 1;
        if g.rescue.heard.has(e.serial) { continue; }
        g.rescue.heard.add(e.serial);
        let by_team = mine.contains(&e.who.unwrap_or(-1.0));
        let bang = e.kind == crate::director::EventKind::Bang && !(e.frag.unwrap_or(false) && !by_team);
        if !bang && (e.level.unwrap_or(0.0) < 1.0 || !by_team) { continue; }
        if bang || e.kind == crate::director::EventKind::Shot || e.kind == crate::director::EventKind::GuardShot || e.kind == crate::director::EventKind::Kick {
            g.rescue.noise = Some([e.pos[0], 0.0, e.pos[2]]);
            if hot(g) { rescue_start_clock(g, "noise"); }
        }
    }
    let o = last_log_order(g);
    if o != g.rescue.last_order {
        g.rescue.last_order = o;
        if o.is_some() && hot(g) { rescue_start_clock(g, "order"); }
    }
    if hot(g) && g.rescue.t0.is_nan() && flat(pl_pos(g), g.k.rescue.sam_at) > 0.6 { rescue_start_clock(g, "move"); }
}

/// Clock two: who has learned, who is set, the late reaction, the roamers let go. (rescue.ts Rescue.defenders)
pub fn rescue_defenders(g: &mut crate::game::Game) {
    use crate::game::GuardState;
    use crate::roomfight::DefMode;
    let own = crate::roomfight::room_fight_on(g);
    g.fight.blind_fire = true;
    let hard = own && rescue_crew_on(g);
    if hard && g.rescue.alarm_at.is_nan() && !g.rescue.t0.is_nan() && g.time >= g.rescue.t0 + g.k.rescue.crew_call_secs { rescue_crew_call(g); }
    let mut i = 0;
    while i < g.rescue.hostiles.len() {
        let h = g.rescue.hostiles[i];
        i += 1;
        if g.gd[h].state == GuardState::Dead { continue; }
        let acting = g.rescue.act.as_ref().map(|a| a.by) == Some(h);
        let d = def_of(g, h);
        if hard && d.role != Role::Captor && d.learned.is_nan() && g.time >= g.rescue.alarm_at {
            g.rescue.defs.get_mut(h).unwrap().learned = g.time;
            let line = format!("learned:{}", g.gd[h].callsign);
            rescue_note(g, &line);
        }
        if def_of(g, h).learned.is_nan() && (g.gd[h].state != GuardState::Patrol || g.gd[h].awareness >= 0.3) && !crate::rescuecheck::rescue_check_owns(g, h) {
            g.rescue.defs.get_mut(h).unwrap().learned = g.time;
            let line = format!("learned:{}", g.gd[h].callsign);
            rescue_note(g, &line);
            if g.k.rescue.start == crate::knobs::RescueKStart::Quiet { rescue_start_clock(g, "heard"); }
        }
        let d = def_of(g, h);
        if !d.saw_foe && g.gd[h].saw_foe_this_frame {
            g.rescue.defs.get_mut(h).unwrap().saw_foe = true;
            if !d.set {
                g.gd[h].react_t = m::max(g.gd[h].react_t, g.k.rescue.unset_react);
                let line = format!("caught:{}", g.gd[h].callsign);
                rescue_note(g, &line);
            }
        }
        let d = def_of(g, h);
        if !d.set && !d.learned.is_nan() && g.time >= d.learned + g.k.rescue.set_secs && g.time >= g.gd[h].dazzled_until && g.gd[h].held.is_none() {
            g.rescue.defs.get_mut(h).unwrap().set = true;
            let line = format!("set:{}", g.gd[h].callsign);
            rescue_note(g, &line);
        }
        let f: Option<RoomfightDefId> = if own { g.fight.defs.iter().copied().find(|&x| g.tab.roomfight_def[x].gd == h) } else { None };
        let crewed = if hard { let mut d = def_of(g, h); rescue_crew(g, h, &mut d, f) } else { false };
        let d = def_of(g, h);
        if d.role == Role::Roamer && !crewed {
            if d.released { continue; }
            if let Some(f) = f {
                if g.tab.roomfight_def[f].mode == DefMode::Free {
                    g.rescue.defs.get_mut(h).unwrap().released = true;
                    let line = format!("released:{}", g.gd[h].callsign);
                    rescue_note(g, &line);
                    continue;
                }
            }
            if d.set {
                g.rescue.defs.get_mut(h).unwrap().released = true;
                g.gd[h].hold = false;
                g.gd[h].pinned = false;
                if let Some(f) = f {
                    let now = g.time;
                    let fd = &mut g.tab.roomfight_def[f];
                    fd.mode = DefMode::Free; fd.free_at = now; fd.rally_at = f64::NAN;
                }
                if g.gd[h].script.is_some() { crate::squad::end_guard_script(g, h, Some(crate::squad::EndGuardScriptState::Alert)); }
                if g.gd[h].state != GuardState::Alert || g.gd[h].foe.is_none() {
                    let to = match g.rescue.noise { Some(n) => n, None => rescue_room_point(g, RescueRoomPointWhich::Outside) };
                    g.gd[h].state = GuardState::Alert;
                    g.gd[h].awareness = 1.0;
                    g.gd[h].react_t = 0.0;
                    g.gd[h].last_known = Some(v3::copy(to));
                    crate::guards::reset_chase(g, h);
                }
                let line = format!("released:{}", g.gd[h].callsign);
                rescue_note(g, &line);
                continue;
            }
        }
        if acting { continue; }
        if own {
            if let Some(f) = f {
                if d.in_room {
                    let now = g.time;
                    let fd = &mut g.tab.roomfight_def[f];
                    fd.rally_at = f64::INFINITY; fd.lost_at = now + 1.0;
                }
                if !d.set && g.time < g.gd[h].dazzled_until {
                    g.tab.roomfight_def[f].burst.n = crate::nightbreach::blind().rounds;
                }
            }
            continue;
        }
        if g.gd[h].script.is_some() {
            let state = match g.gd[h].state {
                GuardState::Patrol => crate::squad::EndGuardScriptState::Patrol,
                GuardState::Suspicious => crate::squad::EndGuardScriptState::Suspicious,
                GuardState::Alert => crate::squad::EndGuardScriptState::Alert,
                GuardState::Search => crate::squad::EndGuardScriptState::Search,
                GuardState::Dead => unreachable!("the dead were passed over at the top of the loop"),
            };
            crate::squad::end_guard_script(g, h, Some(state));
        }
        g.gd[h].hold = true;
        g.gd[h].pinned = true;
        if g.gd[h].state == GuardState::Patrol && g.gd[h].awareness < 0.3 {
            let c = g.gd[h].ch;
            g.ch[c].body_yaw = v3::yaw_to(g.ch[c].pos, d.mark.aim_at);
        }
        if d.set && g.ai_enabled {
            let mut b = d.burst;
            let fired = crate::nightbreach::blind_turn(g, h, d.k, &mut b, d.mark.aim_at);
            g.rescue.defs.get_mut(h).unwrap().burst = b;
            if fired { g.rescue.blind_rounds += 1.0; }
        }
    }
    if own { g.rescue.blind_rounds = g.fight.blind_rounds; }
}

/// The captor's word on their net. (rescue.ts Rescue.crewCall)
pub fn rescue_crew_call(g: &mut crate::game::Game) {
    let hostiles = g.rescue.hostiles.clone();
    let cap = hostiles.iter().copied().find(|&h| def_of(g, h).role == Role::Captor && alive(g, h) && g.gd[h].held.is_none());
    let mates = hostiles.iter().any(|&h| alive(g, h) && def_of(g, h).role != Role::Captor);
    let cap = match (cap, mates) {
        (Some(cap), true) => cap,
        _ => { g.rescue.alarm_at = f64::INFINITY; return; }
    };
    let p = rescue_room_point(g, RescueRoomPointWhich::Outside);
    if rescue_crew_way(g, p).is_empty() { g.rescue.alarm_at = f64::INFINITY; return; }
    if g.time < g.gd[cap].dazzled_until { return; }
    g.rescue.alarm_at = g.time;
    rescue_note(g, "crew:call");
    let line = if crate::dialogue::is_office(g) { "they're here! get off the corridor — guns on my door!" } else { "they're here! get out of the open — guns on my door!" };
    say_radio(g, cap, line);
}

/// One of the captor's two men on the word: true = a roamer who is the crew's this frame. `d` is the caller's copy of his row: it is read as it stands on the way in and put right on the way out. (rescue.ts Rescue.crew)
pub fn rescue_crew(g: &mut crate::game::Game, h: GuardId, d: &mut crate::rescue::Def, f: Option<RoomfightDefId>) -> bool {
    let crewed = crew_on_row(g, h, *d, f);
    *d = *g.rescue.defs.get(h).unwrap();
    crewed
}

/// rescue_crew's body. Every write goes to the row in `defs`, since what it calls reads the row (cues, crewHides, crewLook): `d` is the row at entry, and nothing here reads a field of it after writing that field.
fn crew_on_row(g: &mut Game, h: GuardId, d: Def, f: Option<RoomfightDefId>) -> bool {
    use crate::game::GuardState;
    use crate::roomfight::DefMode;
    if d.role == Role::Captor || d.crew == DefCrew::Out || d.crew == DefCrew::In { return false; }
    let f = match f { Some(f) => f, None => return false };
    let pt = rescue_room_point(g, RescueRoomPointWhich::Outside);
    let name = g.gd[h].callsign.clone();
    if d.crew == DefCrew::Empty {
        if d.in_room || d.released || g.tab.roomfight_def[f].mode == DefMode::Free {
            g.rescue.defs.get_mut(h).unwrap().crew = DefCrew::Out;
            return false;
        }
        if !(g.time >= g.rescue.alarm_at + (if d.role == Role::Roamer { 0.8 } else { 1.6 })) { return false; }
        if g.tab.roomfight_def[f].mode != DefMode::Hold || g.gd[h].held.is_some() || g.time < g.gd[h].dazzled_until || g.time - g.gd[h].last_seen_t < 2.0 { return false; }
        let one = g.rescue.hostiles.iter().any(|&o| o != h && alive(g, o) && def_of(g, o).crew == DefCrew::Cross);
        let x = if !(one && d.role == Role::Door) { rescue_crew_cross_spot(g, h, pt, if d.role == Role::Door { 14.0 } else { 9.0 }) } else { None };
        if let Some(x) = x {
            if flat(x, pos_of(g, h)) > 0.4 {
                rescue_crew_move(g, h, f, x, None);
            } else {
                g.gd[h].state = GuardState::Alert;
                g.gd[h].awareness = m::max(g.gd[h].awareness, 0.92);
            }
            let noise = g.rescue.noise;
            let row = g.rescue.defs.get_mut(h).unwrap();
            row.crew = DefCrew::Cross; row.look = Some(pt); row.spot = Some(x); row.noise0 = noise;
            rescue_note(g, &format!("crew:cross:{}", name));
            let line = if d.role == Role::Door {
                if crate::dialogue::is_office(g) { "off the corridor — I've got your door from the side." } else { "out of the open — I've got your door from the side." }
            } else {
                "I can see your door from here — let them stack on it."
            };
            say_radio(g, h, line);
        } else if d.role == Role::Door {
            let s = rescue_data(g).unwrap();
            let dr = crate::doors::doors_by_name(g, &s.room.door);
            let spot = s.door_man.inside.pos;
            let path = if dr.is_some() { let at = pos_of(g, h); crate::scene_collision::nav_grid_find_path(g, at, spot, None, None) } else { None };
            let there = match (&dr, &path) {
                (Some(_), Some(path)) => !path.is_empty() && !(flat(path[path.len() - 1], spot) > 0.8),
                _ => false,
            };
            if !there {
                g.rescue.defs.get_mut(h).unwrap().crew = DefCrew::Out;
                return false;
            }
            rescue_crew_move(g, h, f, spot, dr);
            let row = g.rescue.defs.get_mut(h).unwrap();
            row.crew = DefCrew::Going; row.spot = Some(v3::copy(spot));
            rescue_note(g, &format!("crew:in:{}", name));
            say_radio(g, h, "coming in — I've got the door.");
        } else {
            let y = rescue_crew_hide(g, h, f, pt);
            if let Some(y) = y {
                rescue_crew_move(g, h, f, y, None);
            } else {
                g.gd[h].state = GuardState::Alert;
                g.gd[h].awareness = m::max(g.gd[h].awareness, 0.92);
            }
            let noise = g.rescue.noise;
            let row = g.rescue.defs.get_mut(h).unwrap();
            row.crew = DefCrew::Wait; row.spot = y; row.noise0 = noise;
            rescue_note(g, &format!("crew:wait:{}", name));
            say_radio(g, h, "staying dark — I'll come up behind them.");
        }
        return d.role == Role::Roamer;
    }
    let bounds = rescue_bounds(g);
    if d.crew == DefCrew::Going {
        let room_door = rescue_data(g).unwrap().room.door;
        let dr = crate::doors::doors_by_name(g, &room_door);
        if let Some(dr) = dr {
            if !crate::doors::door_latched(g, dr) && !crate::doors::door_closing(g, dr) && in_rect(pos_of(g, h), bounds) {
                let (at, fc) = (pos_of(g, h), g.dr[dr].frame_centre);
                if flat(at, fc) > 1.1 && flat(at, fc) < 3.0 {
                    let who = g.ch[g.gd[h].ch].id;
                    crate::doors::doors_pull_to(g, dr, who);
                }
            }
        }
        if g.tab.roomfight_def[f].mode == DefMode::Move { return false; }
        if in_rect(pos_of(g, h), bounds) {
            let row = g.rescue.defs.get_mut(h).unwrap();
            row.in_room = true; row.crew = DefCrew::In;
            rescue_note(g, &format!("crew:inside:{}", name));
        } else {
            g.rescue.defs.get_mut(h).unwrap().crew = DefCrew::Out;
            rescue_note(g, &format!("crew:short:{}", name));
        }
        return false;
    }
    if d.released || g.tab.roomfight_def[f].mode == DefMode::Free {
        let row = g.rescue.defs.get_mut(h).unwrap();
        row.released = true; row.crew = DefCrew::Out;
        return false;
    }
    if d.crew == DefCrew::Cross && g.tab.roomfight_def[f].mode == DefMode::Hold {
        if let Some(spot) = d.spot {
            if flat(pos_of(g, h), spot) > 1.2 {
                g.rescue.defs.get_mut(h).unwrap().crew = DefCrew::Out;
                g.tab.roomfight_def[f].rally_at = f64::NAN;
                rescue_note(g, &format!("crew:short:{}", name));
                return false;
            }
        }
    }
    g.tab.roomfight_def[f].rally_at = f64::INFINITY;
    // by value where the TypeScript asks identity: notes.md, ## Skeleton (rescue.ts:675)
    let at_door = match g.rescue.noise {
        Some(n) => g.rescue.noise != d.noise0 && (flat(n, pt) < 3.0 || in_rect(n, bounds)),
        None => false,
    };
    if at_door && g.time - g.gd[h].last_seen_t > 2.5 && g.gd[h].held.is_none() && g.time >= g.gd[h].dazzled_until {
        let row = g.rescue.defs.get_mut(h).unwrap();
        row.released = true; row.crew = DefCrew::Out;
        g.gd[h].hold = false;
        g.gd[h].pinned = false;
        let now = g.time;
        let fd = &mut g.tab.roomfight_def[f];
        fd.mode = DefMode::Free; fd.free_at = now; fd.rally_at = f64::NAN;
        if g.gd[h].script.is_some() { crate::squad::end_guard_script(g, h, Some(crate::squad::EndGuardScriptState::Alert)); }
        if g.gd[h].state != GuardState::Alert || g.gd[h].foe.is_none() {
            g.gd[h].state = GuardState::Alert;
            g.gd[h].awareness = 1.0;
            g.gd[h].react_t = 0.0;
            g.gd[h].last_known = Some(v3::copy(pt));
            crate::guards::reset_chase(g, h);
        }
        rescue_note(g, &format!("crew:behind:{}", name));
        rescue_note(g, &format!("released:{}", name));
        say_radio(g, h, "they're on the door — moving up behind!");
    }
    d.role == Role::Roamer
}

/// Off his post on the room fight's 'move'. (rescue.ts Rescue.crewMove)
pub fn rescue_crew_move(g: &mut crate::game::Game, h: GuardId, f: RoomfightDefId, goal: V3, by: Option<DoorId>) {
    let at = pos_of(g, h);
    let fd = &mut g.tab.roomfight_def[f];
    fd.mode = crate::roomfight::DefMode::Move;
    fd.move_kind = crate::roomfight::DefMoveKind::Fallback;
    fd.came_by = by;
    fd.move_t = 0.0;
    fd.stall_at = v3::copy(at);
    fd.stall_t = 0.0;
    g.gd[h].pinned = false;
    let speed = g.tune.guard_chase + 0.3;
    crate::squad::order(g, h, crate::squad::OrderPatch {
        goal: Some(Some(v3::copy(goal))), speed: Some(Some(speed)), face: Some(None), strafe: Some(Some(false)), aim_at: Some(None),
        upper: Some(Some(crate::character::UpperMode::Aim)), stance: Some(Some(crate::character::Stance::None)), crouch: Some(Some(false)),
        ..Default::default()
    });
}

/// The way a man coming for the hostages walks, a point at eye height every metre and a half. (rescue.ts Rescue.crewWay)
pub fn rescue_crew_way(g: &mut crate::game::Game, p: V3) -> Vec<V3> {
    if let Some(way) = &g.rescue.way { return way.clone(); }
    let sam_at = g.k.rescue.sam_at;
    let path = crate::scene_collision::nav_grid_find_path(g, sam_at, p, Some(12000.0), None).unwrap_or_default();
    let mut out: Vec<V3> = Vec::new();
    let mut q = sam_at;
    let mut t = 0.0;
    for w in path {
        let l = flat(q, w);
        while t < l {
            out.push([q[0] + (w[0] - q[0]) * t / l, 1.5, q[2] + (w[2] - q[2]) * t / l]);
            t += 1.5;
        }
        t -= l;
        q = w;
    }
    g.rescue.way = Some(out.clone());
    out
}

/// Does a walk from `c` along `path` put a foot where `bad` says, asked every half metre. (rescue.ts Rescue.crewLegs)
pub fn rescue_crew_legs(g: &mut crate::game::Game, c_2: V3, path: &[V3], bad: &mut dyn FnMut(&mut crate::game::Game, V3) -> bool) -> bool {
    let mut q = c_2;
    for &w in path {
        let l = flat(q, w);
        let n = m::max(1.0, m::ceil(l / 0.5));
        let mut i = 1.0;
        while i <= n {
            if bad(g, [q[0] + (w[0] - q[0]) * i / n, 0.0, q[2] + (w[2] - q[2]) * i / n]) { return true; }
            i += 1.0;
        }
        q = w;
    }
    false
}

/// From how many points of the way a man standing at `p` is seen. (rescue.ts Rescue.crewSeen)
pub fn rescue_crew_seen(g: &mut crate::game::Game, p: V3, p_2: V3, near_m: f64) -> f64 {
    let e: V3 = [p[0], 1.5, p[2]];
    let mut n = 0.0;
    for w in rescue_crew_way(g, p_2) {
        if flat(w, p_2) > near_m && flat(w, p) < 25.0 && !crate::scene_collision::segment_blocked(g, w, e, None, None, None) { n += 1.0; }
    }
    n
}

/// Floor in another room with a clear line of fire to two of the places a stack stands. (rescue.ts Rescue.crewCrossSpot, its `fires`)
fn fires(g: &mut Game, p: V3, pt: V3, out: Option<RoomfightRoomId>, stack: &[V3]) -> bool {
    let d = flat(p, pt);
    if d < 5.0 || d > 14.0 || crate::roomfight::room_fight_room_at(g, p) == out { return false; }
    let mut n = 0;
    for &t in stack {
        if !crate::scene_collision::shot_blocked(g, [p[0], 1.5, p[2]], t, None, None) { n += 1; }
    }
    n >= 2
}

/// A gun on the stack: where he stands if it passes, else a spot he can walk to. (rescue.ts Rescue.crewCrossSpot)
pub fn rescue_crew_cross_spot(g: &mut crate::game::Game, h: GuardId, p: V3, walk: f64) -> Option<V3> {
    let c = pos_of(g, h);
    let b = rescue_bounds(g);
    let room_door = rescue_data(g).unwrap().room.door;
    let dr = crate::doors::doors_by_name(g, &room_door);
    let th = match dr { Some(dr) => g.dr[dr].def.closed_dir, None => 0.0 };
    let (ux, uz) = (m::cos(th), m::sin(th));
    let out = crate::roomfight::room_fight_room_at(g, p);
    let mut stack: Vec<V3> = Vec::new();
    for k in [-2.4, -1.2, 0.0, 1.2, 2.4] {
        let t: V3 = [p[0] + ux * k, 1.2, p[2] + uz * k];
        if !crate::scene_collision::nav_grid_is_blocked(g, t[0], t[2]) { stack.push(t); }
    }
    let mut taken: Vec<V3> = Vec::new();
    for &o in &g.rescue.hostiles {
        if o != h && alive(g, o) {
            taken.push(pos_of(g, o));
            if let Some(s) = def_of(g, o).spot { taken.push(s); }
        }
    }
    if fires(g, c, p, out, &stack) { return Some(v3::copy(c)); }
    let cell = g.col.nav.cell;
    let (x0, x1, z0, z1) = (m::floor((c[0] - walk) / cell), m::ceil((c[0] + walk) / cell), m::floor((c[2] - walk) / cell), m::ceil((c[2] + walk) / cell));
    let mut cand: Vec<(V3, f64, f64)> = Vec::new();
    let mut iz = z0;
    while iz <= z1 {
        let mut ix = x0;
        while ix <= x1 {
            let q: V3 = [(ix + 0.5) * cell, 0.0, (iz + 0.5) * cell];
            let d = flat(q, c);
            ix += 1.0;
            if d < 0.5 || d > walk || crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || in_rect(q, b) || !fires(g, q, p, out, &stack) || taken.iter().any(|&t| flat(t, q) < 1.5) { continue; }
            let x = rescue_crew_seen(g, q, p, 4.0);
            cand.push((q, x, d));
        }
        iz += 1.0;
    }
    js::sort_by(&mut cand, |a, b| js::or(a.1 - b.1, a.2 - b.2));
    let mut tries = 0;
    for k in cand {
        if k.1 > CREW_SEEN { break; }
        let tried = tries;
        tries += 1;
        if tried >= 24 { break; }
        let path = match crate::scene_collision::nav_grid_find_path(g, c, k.0, None, None) { Some(path) => path, None => continue };
        if path.is_empty() || flat(path[path.len() - 1], k.0) > 0.8 { continue; }
        let mut l = 0.0;
        let mut q = c;
        for &w in &path { l += flat(q, w); q = w; }
        if l <= walk * 1.4 && !rescue_crew_legs(g, c, &path, &mut |_g: &mut Game, w: V3| in_rect(w, b)) { return Some(k.0); }
    }
    None
}

/// How many places looking in see a man at `p`. (rescue.ts Rescue.crewHide, its `seen`)
fn hide_seen(g: &mut Game, p: V3, pt: V3, looks: &[V3]) -> f64 {
    let mut n = rescue_crew_seen(g, p, pt, 0.0);
    for &w in looks {
        if !crate::scene_collision::segment_blocked(g, w, [p[0], 1.5, p[2]], None, None, None) { n += 1.0; }
    }
    n
}

/// A roamer out of sight: the floor of his own room seen from the fewest places. (rescue.ts Rescue.crewHide)
pub fn rescue_crew_hide(g: &mut crate::game::Game, h: GuardId, f: RoomfightDefId, p: V3) -> Option<V3> {
    let c = pos_of(g, h);
    let room = g.tab.roomfight_def[f].room?;
    if g.tab.roomfight_room[room].cells.len() > 900 { return None; }
    let (cell, nav_w) = (g.col.nav.cell, g.col.nav.w);
    let mut looks: Vec<V3> = Vec::new();
    for e in &g.tab.roomfight_room[room].entries {
        for k in [1.6, 3.2] { looks.push([e.at[0] + e.out[0] * k, 1.5, e.at[2] + e.out[1] * k]); }
    }
    let mut best: Option<V3> = None;
    let mut bs = hide_seen(g, c, p, &looks);
    let mut bl = 8.0;
    if bs == 0.0 { return None; }
    for ci in g.tab.roomfight_room[room].cells.keys() {
        let q: V3 = [(js::rem(ci.0, nav_w) + 0.5) * cell, 0.0, (js::to_i32(ci.0 / nav_w) as f64 + 0.5) * cell];
        let d = flat(q, c);
        if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) || d > 8.0 { continue; }
        let n = hide_seen(g, q, p, &looks);
        if n > bs || (n == bs && (best.is_none() || d >= bl)) { continue; }
        let path = match crate::scene_collision::nav_grid_find_path(g, c, q, None, None) { Some(path) => path, None => continue };
        if path.is_empty() || flat(path[path.len() - 1], q) > 0.8 { continue; }
        let left = rescue_crew_legs(g, c, &path, &mut |g: &mut Game, w: V3| {
            let at = crate::scene_collision::nav_grid_idx(g, m::floor(w[0] / cell), m::floor(w[2] / cell));
            !g.tab.roomfight_room[room].cells.has(crate::Num(at)) && flat(w, q) > 0.8
        });
        if left { continue; }
        bs = n;
        bl = d;
        best = Some(q);
    }
    best
}

/// A man with the stack in his line of fire rests his gun on the floor outside the hostage door. (rescue.ts Rescue.crewLook)
pub fn rescue_crew_look(g: &mut crate::game::Game, gd: GuardId, pt: &mut V3) {
    let look = match g.rescue.defs.get(gd) { Some(d) if d.crew == DefCrew::Cross => d.look, _ => None };
    if let Some(look) = look {
        if rescue_crew_on(g) { pt[0] = look[0]; pt[2] = look[2]; }
    }
}

/// A man of Hard's crew lying in wait, whose place the net must not give away. (rescue.ts Rescue.crewHides)
pub fn rescue_crew_hides(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.rescue.defs.get(gd).map(|d| d.crew) {
        Some(DefCrew::Cross) | Some(DefCrew::Wait) => rescue_crew_on(g),
        _ => false,
    }
}

/// Hard's rescue crew is on: Hard writes both rows, so they stay tests. (rescue.ts Rescue.crewOn)
pub fn rescue_crew_on(g: &mut crate::game::Game) -> bool { g.k.pressure.on && g.k.pressure.rescue_crew }

/// A man sent to the corridor door from inside the server room goes out the way he came in. (rescue.ts Rescue.roundBy)
pub fn rescue_round_by(g: &mut crate::game::Game, op: GuardId) {
    let a = crate::orders::squad_orders_of(g, op);
    let sb = rescue_from_bounds(g);
    let a = match a { Some(a) => a, None => return };
    let post = match g.tab.orders_assignment[a].post { Some(post) => post, None => return };
    if !g.tab.orders_assignment[a].via.is_empty() || sb.is_none() { return; }
    let out = match g.rescue.server_out { Some(out) => out, None => return };
    if !in_rect(pos_of(g, op), sb) || in_rect(post, sb) { return; }
    let leg = v3::copy(out);
    g.tab.orders_assignment[a].via = vec![leg];
    g.tab.orders_assignment[a].goal = leg;
}

/// A man the scenario lets go of where he stands: the order menu's 'cover'. (rescue.ts Rescue.post)
pub fn rescue_post(g: &mut crate::game::Game, op: GuardId, look: Option<V3>, door: Option<&str>) {
    let p = pos_of(g, op);
    let y = match look { Some(look) => v3::yaw_to(p, look), None => g.ch[g.gd[op].ch].body_yaw };
    let watch: V3 = match look {
        Some(look) => [look[0], 1.1, look[2]],
        None => [p[0] + m::sin(y) * 4.0, 1.1, p[2] + m::cos(y) * 4.0],
    };
    let words = match door { Some(door) if !door.is_empty() => format!("covering {}", door), _ => String::from("holding") };
    crate::orders::squad_orders_assign(g, op, &crate::orders::SquadOrdersAssignA {
        order: String::from("cover"), post: Some([p[0], 0.0, p[2]]), face: Some(y), watch: Some(watch),
        stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
        going: words.clone(), there: words,
        ..Default::default()
    });
    if let Some(a) = crate::orders::squad_orders_of(g, op) { g.rescue.auto.add(a); }
}

/// F's entry as it goes: the door tools' beats into the log, 'set', the room called. (rescue.ts Rescue.tickEntry)
pub fn rescue_tick_entry(g: &mut crate::game::Game) {
    use crate::doortools::DoorToolKind;
    if g.rescue.entry.is_none() { return; }
    let t = g.time;
    let b = rescue_bounds(g);
    let mut out: Vec<(f64, String, f64)> = Vec::new();
    let n_ops = g.rescue.entry.as_ref().unwrap().ops.len();
    for i in 0..n_ops {
        let en = g.rescue.entry.as_mut().unwrap();
        let (op, is_side) = (en.ops[i].op, en.ops[i].side);
        let tag = if is_side { ":side" } else { "" };
        let row = &g.tab.doortools_door_op[op];
        if !en.ops[i].kicked {
            if let Some(kick) = &row.kick {
                if !kick.t.is_nan() {
                    en.ops[i].kicked = true;
                    out.push((kick.t, format!("kick{}", tag), 3.0));
                }
            }
        }
        let open_clear = row.kind == DoorToolKind::OpenClear;
        while (en.ops[i].seen as usize) < row.beats.len() {
            let beat = &row.beats[en.ops[i].seen as usize];
            let (at, w) = (beat.t, beat.what.as_str());
            if w == "stacked" { out.push((at, format!("stacked{}", tag), 0.0)); }
            else if w == "kick" || w == "shove" {
                out.push((at, format!("door-flung{}", tag), 4.0));
                if open_clear { out.push((at, format!("nobang{}", tag), 5.0)); }
            }
            else if w == "open already" {
                out.push((at, format!("door-open{}", tag), 4.0));
                if open_clear { out.push((at, format!("nobang{}", tag), 5.0)); }
            }
            else if w.starts_with("toss:") { out.push((at, format!("toss{}", tag), 5.0)); }
            else if w.starts_with("bang") { out.push((at, format!("bang{}", tag), 5.0)); }
            else if w == "go" {
                out.push((at, String::from(if is_side { "enter:side" } else { "enter1" }), 5.0));
                if en.in_t.is_nan() { en.in_t = at; }
            }
            else if w.starts_with("end: ") { en.ops[i].ended = Some(String::from(&w[5..])); }
            en.ops[i].seen += 1.0;
        }
    }
    let en = g.rescue.entry.as_ref().unwrap();
    let (gate, split) = (en.gate, en.split);
    let live: Vec<DoorOpId> = en.ops.iter().filter(|e| e.ended.is_none()).map(|e| e.op).collect();
    if en.set_t.is_nan() && !live.is_empty() && live.iter().all(|&o| !g.tab.doortools_door_op[o].ready_t.is_nan()) {
        let readies: Vec<f64> = live.iter().map(|&o| g.tab.doortools_door_op[o].ready_t).collect();
        let set_t = m::max_n(&readies);
        g.rescue.entry.as_mut().unwrap().set_t = set_t;
        if split { out.push((set_t, String::from("split:set"), 1.0)); }
        let last = g.tab.doortools_door_op[live[live.len() - 1]].lead;
        if alive(g, last) { crate::game::game_say(g, last, if live.len() > 1 { "both doors — set." } else { "set." }, None, None, None); }
    }
    let open_t = g.tab.doortools_door_gate[gate].open_t;
    if let Some(en) = g.rescue.entry.as_mut() {
        if en.go_t.is_nan() && !open_t.is_nan() {
            en.go_t = open_t;
            if split { out.push((open_t, String::from("split:go"), 2.0)); }
        }
    }
    js::sort_by(&mut out, |a, b| js::or(a.0 - b.0, a.2 - b.2));
    for o in out { g.rescue.log.push(RescueLogItem { t: o.0, what: o.1 }); }
    let en = match g.rescue.entry.clone() { Some(en) => en, None => return };
    if !en.set_t.is_nan() && !g.tab.doortools_door_gate[gate].go { rescue_once(g, "set", "stacked and set. press F to send them in"); }
    if !live.is_empty() { return; }
    let up: Vec<GuardId> = en.men.iter().copied().filter(|&o| alive(g, o)).collect();
    if !up.is_empty() && en.ops.iter().all(|e| matches!(e.ended.as_deref(), Some("re-ordered") | Some("nobody left on it"))) {
        rescue_note(g, "drill:called-off");
        rescue_end_entry(g, false);
        return;
    }
    if en.in_t.is_nan() {
        let why = if up.is_empty() { String::from("lost") } else { en.ops[0].ended.clone().unwrap_or_else(|| String::from("null")) };
        rescue_note(g, &format!("entry:{}", why));
        rescue_end_entry(g, false);
        return;
    }
    let theirs = g.rescue.hostiles.iter().any(|&h| alive(g, h) && g.gd[h].held.is_none() && in_rect(pos_of(g, h), b));
    let in_room = up.iter().copied().find(|&o| in_rect(pos_of(g, o), b));
    if !theirs && in_room.is_some() {
        rescue_note(g, "clear");
        crate::game::game_say(g, in_room.unwrap(), "room clear — hostages.", None, None, None);
        g.rescue.called = true;
        rescue_end_entry(g, true);
    } else if up.is_empty() {
        rescue_note(g, "entry:lost");
        rescue_end_entry(g, false);
    } else {
        let mut ends: Vec<f64> = vec![en.in_t + 14.0];
        for e in &en.ops {
            let last = g.tab.doortools_door_op[e.op].beats.last().map(|x| x.t).unwrap_or(f64::NEG_INFINITY);
            ends.push(last + 5.0);
        }
        if t > m::max_n(&ends) {
            rescue_note(g, "entry:stalled");
            rescue_end_entry(g, false);
        }
    }
}

/// The team's men in the hostage room pan their lights across it until somebody's eyes find a man. (rescue.ts Rescue.search)
pub fn rescue_search(g: &mut crate::game::Game) {
    let b = rescue_bounds(g);
    let t = g.time;
    let r = match b { Some(r) => r, None => return };
    let theirs = g.rescue.hostiles.iter().any(|&h| alive(g, h) && g.gd[h].held.is_none() && in_rect(pos_of(g, h), b));
    if !theirs {
        for o in g.rescue.search_t.keys() { rescue_searched(g, o); }
        return;
    }
    let ops = g.rescue.ops.clone();
    for &o in &ops {
        let a = crate::orders::squad_orders_of(g, o);
        let p = pos_of(g, o);
        let stands = match a {
            Some(a) => {
                let row = &g.tab.orders_assignment[a];
                alive(g, o) && g.m.rescue.door_orders.has(row.order.clone()) && match row.post { Some(post) => !(flat(p, post) > 0.6), None => false }
                    && in_rect(p, b) && !g.door_tools.ops.iter().any(|&x| g.tab.doortools_door_op[x].men.contains(&o))
            }
            None => false,
        };
        if !stands { rescue_searched(g, o); continue; }
        let a = a.unwrap();
        if crate::roomfight::room_fight_contact(g, o) || crate::roomfight::room_fight_engaged(g, o) { continue; }
        let t1 = match g.rescue.search_t.get(o) {
            Some(&t1) => t1,
            None => {
                g.rescue.search_t.set(o, t);
                let row = &g.tab.orders_assignment[a];
                let kept = RescueSearchAValue { a, face: row.face, watch: row.watch, there: row.there.clone() };
                g.rescue.search_a.set(o, kept);
                g.tab.orders_assignment[a].there = String::from("searching the room");
                let line = format!("search:{}", g.gd[o].callsign);
                rescue_note(g, &line);
                t
            }
        };
        let mid = v3::yaw_to(p, [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0]);
        let (mut lo, mut hi) = (0.0, 0.0);
        for c in [[r.x0, r.z0], [r.x1, r.z0], [r.x0, r.z1], [r.x1, r.z1]] {
            let d = v3::clamp(wrap(v3::yaw_to(p, [c[0], 0.0, c[1]]) - mid), -SEARCH_HALF, SEARCH_HALF);
            lo = m::min(lo, d);
            hi = m::max(hi, d);
        }
        let span = hi - lo;
        if span < 0.1 { continue; }
        let at = g.rescue.ops.iter().position(|&x| x == o).unwrap_or(0);
        let u = js::rem((t - t1) * SEARCH_RATE / span + (if at % 2 != 0 { 0.5 } else { 0.0 }), 2.0);
        let yaw = mid + lo + span * (if u < 1.0 { u } else { 2.0 - u });
        let row = &mut g.tab.orders_assignment[a];
        row.watch = Some([p[0] + m::sin(yaw) * 4.0, 1.2, p[2] + m::cos(yaw) * 4.0]);
        row.face = Some(yaw);
        row.search = Some(true);
        row.search_t = Some(t);
    }
}

/// His search is over: the sector the door tools left him on, back. (rescue.ts Rescue.searched)
pub fn rescue_searched(g: &mut crate::game::Game, o: GuardId) {
    let was = g.rescue.search_a.get(o).cloned();
    g.rescue.search_t.delete(o);
    g.rescue.search_a.delete(o);
    if let Some(was) = was {
        if crate::orders::squad_orders_of(g, o) == Some(was.a) {
            let row = &mut g.tab.orders_assignment[was.a];
            row.face = was.face; row.watch = was.watch; row.there = was.there;
        }
    }
}

/// F's entry is over; the room called, its men hold it. (rescue.ts Rescue.endEntry)
pub fn rescue_end_entry(g: &mut crate::game::Game, called: bool) {
    let en = match g.rescue.entry.take() { Some(en) => en, None => return };
    g.rescue.door_orders = 0.0;
    g.rescue.said.delete(String::from("set"));
    if !called { return; }
    let s = match rescue_data(g) { Some(s) => s, None => return };
    let mut first = true;
    for e in &en.ops {
        let mut j = 0;
        while j < g.tab.doortools_door_op[e.op].men.len() {
            let o = g.tab.doortools_door_op[e.op].men[j];
            j += 1;
            if !alive(g, o) { continue; }
            let now = crate::orders::squad_orders_of(g, o);
            let mine = g.tab.doortools_door_op[e.op].mine.get(o).copied();
            if now.is_none() || now != mine { continue; }
            match &s.side {
                Some(side) if e.side || !first => {
                    let name = door_name(g, &side.door);
                    rescue_post(g, o, Some(v3::copy(side.hold)), Some(&name));
                }
                _ => {
                    first = false;
                    let hold = rescue_room_point(g, RescueRoomPointWhich::Hold);
                    let name = door_name(g, &s.room.door);
                    rescue_post(g, o, Some(hold), Some(&name));
                }
            }
        }
    }
}

/// The last loud thing the team did. (rescue.ts Rescue.noiseAt)
pub fn rescue_noise_at(g: &mut crate::game::Game) -> Option<V3> { g.rescue.noise }

/// The living and the dead of the cast as their radio check reads them. (rescue.ts Rescue.cast)
pub fn rescue_cast(g: &mut crate::game::Game) -> Vec<crate::rescuecheck::CastItem> {
    g.rescue.hostiles.iter().map(|&gd| {
        let d = def_of(g, gd);
        crate::rescuecheck::CastItem { gd, role: d.role, in_room: d.in_room, post: d.mark.pos, learned_at: d.learned, released: d.released, crewed: d.crew != DefCrew::Empty }
    }).collect()
}

/// A man the flush sends is nobody's to send again. (rescue.ts Rescue.letGo)
pub fn rescue_let_go(g: &mut crate::game::Game, gd: GuardId) {
    if let Some(d) = g.rescue.defs.get_mut(gd) { d.released = true; d.crew = DefCrew::Out; }
}

/// The floor just outside the hostage room's main door. (rescue.ts Rescue.outsidePoint)
pub fn rescue_outside_point(g: &mut crate::game::Game) -> V3 { rescue_room_point(g, RescueRoomPointWhich::Outside) }

/// The floor a metre outside the hostage room's side door. (rescue.ts Rescue.sideOutside)
pub fn rescue_side_outside(g: &mut crate::game::Game) -> Option<V3> {
    let n = rescue_data(g)?.side?.door;
    let d = crate::doors::doors_by_name(g, &n)?;
    let b = rescue_bounds(g);
    Some(rescue_out_of(g, d, b, 1.0))
}

/// The way round to the side door. (rescue.ts Rescue.sideWay)
pub fn rescue_side_way(g: &mut crate::game::Game) -> Vec<V3> {
    let o = match rescue_side_outside(g) { Some(o) => o, None => return Vec::new() };
    let mut way: Vec<V3> = Vec::new();
    if let Some(out) = g.rescue.server_out { way.push(v3::copy(out)); }
    way.push(o);
    way
}

/// Who holds the hostages right now. (rescue.ts Rescue.captor)
pub fn rescue_captor(g: &mut crate::game::Game) -> Option<GuardId> {
    let b = rescue_bounds(g);
    let all: Vec<GuardId> = g.rescue.hostages.iter().copied().filter(|&h| alive(g, h)).collect();
    if all.is_empty() { return None; }
    let held: Vec<V3> = all.iter().map(|&x| pos_of(g, x)).filter(|&p| in_rect(p, b)).collect();
    let was = g.rescue.cap_by;
    if let (false, Some(w)) = (held.is_empty(), was) {
        if alive(g, w) && g.gd[w].held.is_none() && in_rect(pos_of(g, w), b) { return Some(w); }
    }
    let mut best: Option<GuardId> = None;
    let mut bd = 1e9;
    if !held.is_empty() {
        for &h in &g.rescue.hostiles {
            if !alive(g, h) || g.gd[h].held.is_some() || !in_rect(pos_of(g, h), b) { continue; }
            let at = pos_of(g, h);
            let dists: Vec<f64> = held.iter().map(|&x| flat(x, at)).collect();
            let d = m::min_n(&dists);
            if d < bd { bd = d; best = Some(h); }
        }
    }
    match best {
        Some(best) => Some(best),
        None => crate::rescueflush::rescue_flush_holder(g, was),
    }
}

/// The captor's last warning, as the men at a door have it. (rescue.ts Rescue.lastWarning)
pub fn rescue_last_warning(g: &mut crate::game::Game, d: DoorId) -> bool {
    if !g.rescue.active || rescue_over(g) || g.rescue.t0.is_nan() { return false; }
    let s = match rescue_data(g) { Some(s) => s, None => return false };
    let name = g.dr[d].def.name.as_str();
    if name != s.room.door && Some(name) != s.side.as_ref().map(|x| x.door.as_str()) { return false; }
    (g.rescue.step == RescueStep::AboutTo || g.rescue.act.is_some() || g.rescue.executed.len() > 0) && rescue_captor(g).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crew_on_game() -> Game {
        let mut g = Game::blank();
        g.k.pressure.on = true;
        g.k.pressure.rescue_crew = true;
        g
    }
    fn man_on(g: &mut Game, crew: DefCrew, look: Option<V3>) -> GuardId {
        let h = GuardId(1);
        g.rescue.defs.set(h, Def { crew, look, ..Def::zeroed() });
        h
    }

    #[test]
    fn the_log_loses_its_oldest_150_past_600() {
        let mut g = Game::blank();
        for _ in 0..600 { rescue_note(&mut g, "x"); }
        assert_eq!(g.rescue.log.len(), 600);
        rescue_note(&mut g, "last");
        assert_eq!(g.rescue.log.len(), 451);
        assert_eq!(g.rescue.log[450].what, "last");
    }

    #[test]
    fn a_man_on_a_cross_writes_x_and_z_of_the_point_and_leaves_its_height() {
        let mut g = crew_on_game();
        let h = man_on(&mut g, DefCrew::Cross, Some([3.0, 9.0, 4.0]));
        let mut pt: V3 = [1.0, 2.0, 1.0];
        rescue_crew_look(&mut g, h, &mut pt);
        assert_eq!(pt, [3.0, 2.0, 4.0]);
    }

    #[test]
    fn nothing_else_moves_the_point() {
        let mut g = crew_on_game();
        let h = man_on(&mut g, DefCrew::Wait, Some([3.0, 9.0, 4.0]));
        let mut pt: V3 = [1.0, 2.0, 1.0];
        rescue_crew_look(&mut g, h, &mut pt);
        rescue_crew_look(&mut g, GuardId(2), &mut pt);
        g.rescue.defs.get_mut(h).unwrap().crew = DefCrew::Cross;
        g.k.pressure.on = false;
        rescue_crew_look(&mut g, h, &mut pt);
        assert_eq!(pt, [1.0, 2.0, 1.0]);
    }

    #[test]
    fn let_go_writes_the_row_itself() {
        let mut g = Game::blank();
        let h = man_on(&mut g, DefCrew::Wait, None);
        rescue_let_go(&mut g, h);
        let d = g.rescue.defs.get(h).unwrap();
        assert!(d.released);
        assert!(d.crew == DefCrew::Out);
    }
}
