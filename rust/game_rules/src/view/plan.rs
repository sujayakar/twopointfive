//! THE SQUAD'S PLAN ON THE FLOOR, as a seat is told it: per man with a mark his way, his mark and what he watches; everything waiting on the player's key, with whether the key pressed
//! now would send it; the throws. Only men of the seat's own squad are listed, and nothing of an enemy the team cannot see. A leg of a man's way that is not his own path is a question
//! of the floor the HUD asks (`ask::WorldAsk::way`) and keeps by its own clock; nothing here searches.
//!
//! `waits_on_key` repeats the rules of the press as a reader (the pair's mark first, then the nearest set charge of a wall or a door's, then the word, the drill and the rescue's entry).
use super::*;
use crate::doortools::Phase;
use crate::game::{Game, GuardState};
use crate::ids::*;
use crate::seats::SeatRef;
use crate::v3;
use crate::V3;
use alloc::{string::String, vec::Vec};

/// for whom a plan is asked: `own` false, for a page that cannot act on it (nothing of its own player's alone); `reading`, the one who looks has his own menu up
#[derive(Clone, Copy)]
struct For { own: bool, reading: bool }

fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }
fn alive(g: &Game, gd: GuardId) -> bool { g.gd[gd].state != GuardState::Dead }
fn floor(p: V3) -> V3 { [p[0], 0.0, p[2]] }
fn id_of(g: &Game, gd: GuardId) -> f64 { g.ch[g.gd[gd].ch].id }

/// 'Red Two' is R2, 'Blue One' B1, 'Kestrel' K
pub fn plan_tag(callsign: &str) -> String {
    let mut w = callsign.split_whitespace();
    let Some(first) = w.next() else { return String::new() };
    let mut out: String = first.chars().take(1).flat_map(|c| c.to_uppercase()).collect();
    for x in w {
        match x.to_lowercase().as_str() {
            "one" => out.push('1'), "two" => out.push('2'), "three" => out.push('3'), "four" => out.push('4'), "five" => out.push('5'), "six" => out.push('6'),
            _ => out.extend(x.chars().take(1).flat_map(|c| c.to_uppercase())),
        }
    }
    out.chars().take(3).collect()
}
/// a key's code as its cap says it
pub fn key_cap(code: &str) -> String { String::from(code.strip_prefix("Key").or(code.strip_prefix("Digit")).unwrap_or(code)) }
fn ang_diff(a: f64, b: f64) -> f64 {
    let tau = core::f64::consts::PI * 2.0;
    let mut d = (a - b) % tau;
    if d > core::f64::consts::PI { d -= tau; }
    if d < -core::f64::consts::PI { d += tau; }
    crate::m::abs(d)
}
fn cap_of(g: &Game, key: &Option<String>, now: bool, ready: bool) -> KeyCap {
    if key.is_none() { return KeyCap::None; }
    if !g.k.plan.key_lit { return if now { KeyCap::Plate } else { KeyCap::None }; }
    if now && ready { KeyCap::Lit } else { KeyCap::Dark }
}
/// a man's colour: his pair's wherever the team is in pairs
fn tone_of(g: &Game, paired: bool, gd: GuardId) -> Option<Tone> {
    if !paired { return None; }
    crate::orders::squad_orders_pair_of(g, gd).map(|p| match p { crate::game::PairId::Red => Tone::Red, crate::game::PairId::Blue => Tone::Blue })
}
/// one colour for some men: their pair's where they share one, else the whole squad's
fn tone_for(g: &Game, seat: &SeatRef, paired: bool, men: &[GuardId]) -> Option<Tone> {
    let mut t = men.iter().map(|&m| tone_of(g, paired, m));
    match t.next() {
        Some(a) if t.all(|b| b == a) => a,
        _ => if crate::orders::squad_orders_paired(g, seat) { Some(Tone::Gold) } else { None },
    }
}

/// a doorway's two jambs on the floor
fn jambs(g: &Game, d: DoorId) -> (V3, V3) {
    let def = &g.dr[d].def;
    let (cx, cz) = (crate::m::cos(def.closed_dir), crate::m::sin(def.closed_dir));
    ([def.hinge[0], 0.0, def.hinge[1]], [def.hinge[0] + cx * def.width, 0.0, def.hinge[1] + cz * def.width])
}

#[derive(Clone)]
struct Wait { door: DoorId, men: Vec<GuardId>, why: WaitWhy, untold: bool, word: bool, set: Option<ChargeId>, drill: bool, gate: bool }
fn rank(w: WaitWhy) -> i32 { match w { WaitWhy::Stack | WaitWhy::Pair => 0, WaitWhy::Tool => 1, WaitWhy::Thr | WaitWhy::Drill => 2, WaitWhy::Charge | WaitWhy::Wall => 3 } }
fn put<'a>(by: &'a mut Vec<Wait>, door: DoorId, men: &[GuardId], why: WaitWhy, word: bool) -> &'a mut Wait {
    let i = match by.iter().position(|e| e.door == door) {
        Some(i) => i,
        None => { by.push(Wait { door, men: Vec::new(), why, untold: false, word: false, set: None, drill: false, gate: false }); by.len() - 1 }
    };
    let e = &mut by[i];
    for &m in men { if !e.men.contains(&m) { e.men.push(m); } }
    if rank(why) > rank(e.why) { e.why = why; }
    if word { e.word = true; }
    e
}

/// what waits on the player's key, and for each whether the key pressed now would send it
fn waits_on_key(g: &Game, seat: &SeatRef, paired: bool, out: &mut Vec<PlanDoor>) {
    let (s, t) = (seat.orders, seat.door_tools);
    let big = crate::orders::squad_orders_big(g, seat);
    let sam = g.ch[g.pl[seat.player].ch].pos;
    let up = !g.pl[seat.player].down;
    let addr = crate::doortools::door_tools_word_men(g, seat);
    let f = key_cap(&g.k.doortools.blow_key);
    let word_key = !big || g.k.doortools.word_on_key;
    let squad = crate::orders::squad_orders_squad(g, seat);
    let mut by: Vec<Wait> = Vec::new();
    for &o in t.ops.iter() {
        let op = &g.tab.doortools_door_op[o];
        if op.word.is_some() { continue; }
        let held = op.thr.is_some() && op.phase == Phase::Thr;
        if held || (big && (op.on_word || matches!(op.phase, Phase::Held | Phase::Armed | Phase::Standoff | Phase::Place))) {
            let men: Vec<GuardId> = op.men.iter().copied().filter(|&m| alive(g, m)).collect();
            let why = if held { WaitWhy::Thr } else if op.kind == crate::doortools::DoorToolKind::Charge { WaitWhy::Charge } else { WaitWhy::Tool };
            let word = men.iter().any(|m| addr.contains(m));
            let e = put(&mut by, op.door, &men, why, word);
            if held { if let Some(th) = &op.thr { if th.hold && !th.told { e.untold = true; } } }
        }
    }
    if big {
        for &gd in squad.iter() {
            let Some(a) = crate::orders::squad_orders_of(g, seat, gd) else { continue };
            let a = &g.tab.orders_assignment[a];
            if alive(g, gd) && a.order == "stack" && a.post.is_some() && a.there.starts_with("stacked") {
                if let Some(d) = a.door { put(&mut by, d, &[gd], WaitWhy::Stack, addr.contains(&gd)); }
            }
        }
    } else {
        // (one or two teammates on a plain stack wait on the key too; the key's own list, asked as the key asks it, is the addressed men's)
        for x in crate::doortools::door_tools_stack_waits(g, seat, &squad) { for &gd in &x.men { put(&mut by, x.stack, &[gd], WaitWhy::Stack, false); } }
        for x in crate::doortools::door_tools_stack_waits(g, seat, &addr) { for &gd in &x.men { put(&mut by, x.stack, &[gd], WaitWhy::Stack, true); } }
    }
    // (a charge of his own that the squad is not at waits on HIS key, not this one)
    let t_set: Vec<ChargeId> = t.set.iter().copied().filter(|&c| crate::doortools::door_tools_key_is(g, seat, c)).collect();
    let w_set: Vec<WallChargeId> = seat.wall_charge.set.iter().copied().filter(|&c| crate::wallcharge::wall_charge_key_is(g, seat, c)).collect();
    let charge_men = |by_name: &str| -> Vec<GuardId> { squad.iter().copied().filter(|&m| alive(g, m) && g.gd[m].callsign == by_name).collect() };
    for &c in &t_set {
        let sc = &g.tab.doortools_set_charge[c];
        put(&mut by, sc.door, &charge_men(&sc.by), WaitWhy::Charge, false).set = Some(c);
    }
    let nb = crate::nightbreach::night_breach_state(g) == crate::nightbreach::NightBreachState::Set;
    let drill_ops: Vec<GuardId> = crate::nightbreach::night_breach_drill_ops(g);
    if nb {
        if let Some(d) = g.night_breach.drill.as_ref().and_then(|x| crate::doors::doors_by_name(g, &x.plan.door)) {
            let men: Vec<GuardId> = drill_ops.iter().copied().filter(|&m| alive(g, m)).collect();
            put(&mut by, d, &men, WaitWhy::Drill, false).drill = true;
        }
    }
    // the rescue's entry on the hostage room, held for the second word
    if g.rescue.active && g.rescue.result.is_none() {
        if let Some(entry) = &g.rescue.entry {
            if !g.tab.doortools_door_gate[entry.gate].go {
                for e in entry.ops.iter() {
                    if e.ended.is_some() { continue; }
                    let op = &g.tab.doortools_door_op[e.op];
                    let men: Vec<GuardId> = op.men.iter().copied().filter(|&m| alive(g, m)).collect();
                    put(&mut by, op.door, &men, WaitWhy::Stack, false).gate = true;
                }
            }
        }
    }
    // the quiet pair's mark: the first reader of the key. The key a step ahead of each man who waits, on his gun's line: nothing of the man he waits on. A lesson that still holds the word shut lists no mark
    let marked = crate::pairorders::pair_orders_waiting(g, seat);
    let pair_first = up && !marked.is_empty();
    if !crate::rails::rails_locks(g, "word") {
        for (gd, j) in marked.iter() {
            let c = &g.ch[g.gd[*gd].ch];
            let (fx, fz) = (crate::m::sin(c.aim_yaw), crate::m::cos(c.aim_yaw));
            let (cx, cz) = (c.pos[0] + fx * g.k.plan.pair_bar_m, c.pos[2] + fz * g.k.plan.pair_bar_m);
            let key = Some(String::from("F"));
            let cap = cap_of(g, &key, pair_first, j.set || j.watcher.is_some());
            out.push(PlanDoor { door: None, a: [cx - fz * 0.3, 0.0, cz + fx * 0.3], b: [cx + fz * 0.3, 0.0, cz - fx * 0.3], tone: tone_of(g, paired, *gd), key, now: pair_first, cap, why: WaitWhy::Pair });
        }
    }
    if !by.is_empty() || !w_set.is_empty() {
        // (a set charge not on a word already given)
        let free: Vec<ChargeId> = t_set.iter().copied().filter(|&c| {
            let door = g.tab.doortools_set_charge[c].door;
            !t.ops.iter().any(|&o| { let op = &g.tab.doortools_door_op[o]; op.kind == crate::doortools::DoorToolKind::Charge && op.door == door && op.word.is_some_and(|w| g.tab.doortools_word[w].go1.is_nan()) })
        }).collect();
        let door_d = |c: ChargeId| flat(g.dr[g.tab.doortools_set_charge[c].door].frame_centre, sam);
        let near_set = free.iter().copied().reduce(|a, b| if door_d(b) < door_d(a) { b } else { a });
        let wall_foot = |c: WallChargeId| crate::wallcharge::wall_pt(g, &g.tab.wallcharge_set_wall_charge[c].wall, 0.0, 0.0, None);
        let wall = w_set.iter().copied().reduce(|a, b| if flat(wall_foot(b), sam) < flat(wall_foot(a), sam) { b } else { a });
        let wall_d = wall.map_or(f64::INFINITY, |c| flat(wall_foot(c), sam));
        let wall_first = wall.is_some() && !t_set.iter().any(|&c| door_d(c) < wall_d);
        let reaches = up && !pair_first && (!wall_first || t_set.is_empty());   // the press gets as far as the door tools
        let any_word = word_key && by.iter().any(|e| e.word);
        let row_key = s.menu.as_ref().is_some_and(|m| m.rows.iter().any(|&d| g.tab.orders_order_def[d].hot_key.as_deref() == Some(g.k.doortools.blow_key.as_str())));   // a menu up with a row on the key: the key IS that row
        let early = big && t.ops.iter().any(|&o| {
            let op = &g.tab.doortools_door_op[o];
            op.thr.as_ref().is_some_and(|th| matches!(th.first, Some(crate::doortools::ThresholdFirst::Can | crate::doortools::ThresholdFirst::Burst))) && op.men.iter().any(|m| addr.contains(m))
        });
        let last = reaches && !any_word && !early && !row_key && !s.menu.as_ref().is_some_and(|m| m.word_row) && t_set.is_empty() && w_set.is_empty();   // … and past them all, to the rescue
        let drill_now = if any_word { drill_ops.iter().any(|m| addr.contains(m)) } else { t_set.is_empty() && w_set.is_empty() };   // (the drill reads the key after them)
        // (every man of it on his place, his feet there: a gun still coming round does not put the cap out, and none taken off it by a fight)
        let set = |men: &[GuardId]| men.iter().all(|&m| {
            if !alive(g, m) { return true; }
            let Some(a) = crate::orders::squad_orders_of(g, seat, m) else { return true };
            let a = &g.tab.orders_assignment[a];
            a.yield_t < 0.0 && (a.arrived || a.post.is_none_or(|p| flat(g.ch[g.gd[m].ch].pos, p) < 0.5))
        });
        for e in by.iter() {
            let (ja, jb) = jambs(g, e.door);
            let mut now = false;
            if reaches {
                if (e.word && word_key) || (e.gate && last) || (e.drill && drill_now) { now = true; }
                else if let Some(c) = e.set {
                    if free.contains(&c) { now = if big { !any_word && Some(c) == near_set } else { any_word || Some(c) == near_set }; }
                }
            }
            let key = if word_key || e.set.is_some() || e.drill || e.gate { Some(f.clone()) } else { None };
            let ready = set(&e.men) && !(g.k.plan.thr_told && e.untold);
            let cap = cap_of(g, &key, now, ready);
            out.push(PlanDoor { door: Some(g.dr[e.door].def.name.clone()), a: ja, b: jb, tone: tone_for(g, seat, paired, &e.men), key, now, cap, why: e.why });
        }
        for &c in &w_set {
            let sc = &g.tab.wallcharge_set_wall_charge[c];
            let key = Some(f.clone());
            let now = up && !pair_first && !row_key && wall_first && Some(c) == wall;
            let cap = cap_of(g, &key, now, true);
            out.push(PlanDoor { door: None, a: crate::wallcharge::wall_pt(g, &sc.wall, -0.4, 0.03, None), b: crate::wallcharge::wall_pt(g, &sc.wall, 0.4, 0.03, None), tone: tone_for(g, seat, paired, &charge_men(&sc.by)), key, now, cap, why: WaitWhy::Wall });
        }
    }
    // on rails a key is named only once its lesson has opened it: the bar across the doorway stays (the men ARE waiting), the cap goes
    if crate::rails::rails_hushes(g, "word") {
        for d in out.iter_mut() { if d.why != WaitWhy::Pair { d.key = None; d.now = false; d.cap = KeyCap::None; } }
    }
}

/// the way he has still to walk, as far as the order says it
fn way_of(g: &Game, gd: GuardId, a: &crate::orders::Assignment, mark: V3) -> PlanWay {
    let m = &g.gd[gd];
    let p = g.ch[m.ch].pos;
    let mut stops: Vec<V3> = a.via.clone();
    stops.push(mark);
    // his own path, where it is this order's and runs all the way (a one-point side step to get unstuck is not his way)
    let own = match (m.path_goal, m.path.last()) {
        (Some(goal), Some(&end)) if m.path_i < m.path.len() && (flat(goal, stops[0]) < 0.9 || flat(goal, a.goal) < 0.9) && flat(end, goal) < 1.2 => Some(m.path[m.path_i..].to_vec()),
        _ => None,
    };
    PlanWay { from: floor(p), own, stops }
}

fn seat_plan(g: &Game, seat: &SeatRef, who: For) -> SeatPlan {
    let s = seat.orders;
    let k = &g.k.plan;
    let mut out = SeatPlan { men: Vec::new(), who: Vec::new(), you: None, doors: Vec::new(), throws: Vec::new(), mode: true };
    if !k.on || !crate::orders::squad_orders_active(g, seat) { return out; }
    // outside the order mode: only what the order book was just told. The NEWEST thing that names a man is the one he is drawn by, and it alone
    let mode = !k.menu_only || s.menu.is_some() || who.reading;
    out.mode = mode;
    let (mut given, mut slow): (Vec<(String, f64)>, Vec<(String, f64)>) = (Vec::new(), Vec::new());
    if !mode {
        let auto = g.k.autopilot.on;
        let far = if auto { crate::m::max(k.fresh_secs, g.k.autopilot.show_secs) } else { k.fresh_secs };
        let shown = if auto { crate::m::max(0.2, g.k.autopilot.show_secs) } else { k.fresh_secs };
        let mut last: Vec<(String, f64, f64)> = Vec::new();
        let mut note = |c: &String, t: f64, secs: f64| match last.iter_mut().find(|e| &e.0 == c) {
            Some(e) => if t > e.1 { e.1 = t; e.2 = secs; },
            None => last.push((c.clone(), t, secs)),
        };
        for row in s.log.iter().rev() {
            if !(g.time - row.t < far && row.t <= g.time) { break; }
            let secs = if row.auto == Some(true) { crate::m::max(0.2, g.k.autopilot.show_secs) } else { k.fresh_secs };
            for c in &row.who { note(c, row.t, secs); }
        }
        if auto {
            for row in s.shows.iter().rev() {
                if !(g.time - row.t < far && row.t <= g.time) { break; }
                for c in &row.who { note(c, row.t, shown); }
            }
        }
        for (c, t, secs) in last {
            if g.time - t < secs {
                if secs != k.fresh_secs { slow.push((c.clone(), k.fresh_secs / secs)); }
                given.push((c, t));
            }
        }
    }
    let paired = crate::verdict::team_paired(g, seat);
    let squad = crate::orders::squad_orders_squad(g, seat);
    for &gd in squad.iter() {
        let m = &g.gd[gd];
        if m.state == GuardState::Dead { continue; }
        let Some(aid) = crate::orders::squad_orders_of(g, seat, gd) else { continue };
        let a = &g.tab.orders_assignment[aid];
        let Some(post) = a.post else { continue };
        let mut age = crate::m::max(0.0, g.time - a.t0);
        if !mode {
            let Some(&(_, t)) = given.iter().find(|e| e.0 == m.callsign) else { continue };
            if a.t0 < t - 0.02 { continue; }   // (his since that order: a door tool hands him his mark a beat after it)
            let k_slow = slow.iter().find(|e| e.0 == m.callsign).map_or(1.0, |e| e.1);
            age = crate::m::max(age, g.time - t) * k_slow;
        }
        if a.foe.is_some_and(|f| !crate::fogofwar::fog_of_war_seen(g, f)) { continue; }   // (a mark taken from a man the team cannot see would walk the picture to him)
        // a step off the mark that the order's own goal names: that is where he will stand
        let at = if a.via.is_empty() && flat(a.goal, post) > 0.05 { a.goal } else { post };
        let mark = floor(at);
        let off = a.yield_t >= 0.0;
        let held = off || m.script.as_ref().is_some_and(|x| x.over.is_some());
        let c = &g.ch[m.ch];
        let there = !off && flat(c.pos, mark) < 0.45;
        let walking = !off && !there;
        let fit_watch = a.fit.as_ref().and_then(|f| f.watch);
        let raw = a.watch.or(fit_watch);
        let on_door = raw.is_some_and(|w| a.door.is_some_and(|d| flat(w, g.dr[d].frame_centre) < 1.5));
        let told = raw.filter(|&w| a.foe.is_none() && (on_door || Some(w) == fit_watch || a.order == "cover" || a.order == "move" || a.order == "hold"));
        let (mut aim, mut watch): (Option<f64>, Option<V3>) = (None, None);
        if there {
            aim = Some(c.aim_yaw);
            if let Some(w) = told { if ang_diff(v3::yaw_to(c.pos, w), c.aim_yaw) < 0.4 { watch = Some(w); } }
        } else if walking {
            match told {
                Some(w) if flat(mark, w) > 0.3 => { aim = Some(v3::yaw_to(mark, w)); watch = Some(w); }
                _ => if a.foe.is_none() { aim = a.fit.as_ref().map(|f| f.face).or(a.face); },
            }
        }
        if watch.is_some_and(|w| flat(mark, w) > k.sight_m) { watch = None; }
        out.men.push(PlanMan {
            man: c.id, tag: plan_tag(&m.callsign), tone: tone_of(g, paired, gd), age,
            way: if walking && k.paths { Some(way_of(g, gd, a, mark)) } else { None },
            mark, there, held,
            aim: if k.sectors { aim } else { None },
            watch: if k.sectors { watch } else { None },
            on_door: k.sectors && watch.is_some() && on_door,
        });
    }
    if k.word { waits_on_key(g, seat, paired, &mut out.doors); }
    if !who.own { for d in out.doors.iter_mut() { d.key = None; d.now = false; d.cap = KeyCap::None; } }   // (the press is its own player's)
    if k.throws {
        for t in throws_view(g, seat) {
            if t.kind == ThrowKind::Shell || t.points.len() < 2 || (t.preview && !who.own) { continue; }
            let Some(&gd) = squad.iter().find(|&&m| id_of(g, m) == t.man) else { continue };
            if !alive(g, gd) { continue; }
            let walk = match &t.walk { Some(w) if k.paths && w.len() > 1 => w.iter().map(|&q| floor(q)).collect(), _ => Vec::new() };
            let end = t.points[t.points.len() - 1];
            let mut arc = t.points.clone();
            if flat(end, t.land) > 0.1 { arc.push(floor(t.land)); }
            out.throws.push(PlanThrow { man: t.man, tag: plan_tag(&g.gd[gd].callsign), tone: tone_of(g, paired, gd), kind: t.kind, preview: t.preview, from: floor(t.from), walk, arc, land: floor(t.land), reach: if t.kind == ThrowKind::Frag { t.kill } else { 0.0 } });
        }
    }
    if who.own && k.who {
        if let Some(menu) = &s.menu {
            for &gd in menu.men.iter() { if alive(g, gd) { out.who.push(PlanWho { man: id_of(g, gd), at: floor(g.ch[g.gd[gd].ch].pos), tone: tone_of(g, paired, gd) }); } }
            if !g.pl[seat.player].down { out.you = Some(floor(g.ch[g.pl[seat.player].ch].pos)); }
        }
    }
    out
}

/// the same wait in two plans (a door's, or a wall's two ends)
fn same_wait(a: &PlanDoor, b: &PlanDoor) -> bool {
    (a.door.is_some() && a.door == b.door) || (crate::m::abs(a.a[0] - b.a[0]) + crate::m::abs(a.a[2] - b.a[2]) + crate::m::abs(a.b[0] - b.b[0]) + crate::m::abs(a.b[2] - b.b[2]) < 0.02)
}

/// the plan on the floor, about the seat it is handed: its own whole, then each other player's (a wait both plans list is on the page once, as the seat's own)
pub fn plan_view(g: &Game, seat: &SeatRef) -> Option<PlanView> {
    if !g.k.plan.on { return None; }
    let own = seat_plan(g, seat, For { own: true, reading: false });
    let mut others = Vec::new();
    if g.seats.len() > 1 && g.k.plan.others > 0.0 {
        let reading = seat.orders.menu.is_some();
        for &s in g.seats.iter() {
            if s == seat.id { continue; }
            let mut p = seat_plan(g, &crate::seats::seat(g, s), For { own: false, reading });
            p.doors.retain(|d| !own.doors.iter().any(|o| same_wait(o, d)));
            others.push(p);
        }
    }
    Some(PlanView { own, others })
}
