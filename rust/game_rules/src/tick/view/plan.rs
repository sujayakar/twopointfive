//! THE SQUAD'S PLAN ON THE FLOOR, as a seat is told it: per man with a mark his way, his mark and what he watches; everything waiting on the player's key, with whether the key pressed
//! now would send it; the throws. Only men of the seat's own squad are listed, and nothing of an enemy the team cannot see. A leg of a man's way that is not his own path is a question
//! of the floor the HUD asks (`scene_collision::way_host`) and keeps by its own clock; nothing here searches.
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
fn cap_of(now: bool, ready: bool) -> KeyCap { if now && ready { KeyCap::Lit } else { KeyCap::Dark } }
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
    let mut w = Waiting::of(g, seat, paired);
    w.ops();
    w.stacks();
    w.set_charges();
    w.missions();
    let pair_first = w.pair_marks(out);
    if !w.by.is_empty() || !w.walls.is_empty() {
        let press = w.press(pair_first);
        w.doors(&press, out);
        w.wall_charges(&press, out);
    }
    // on rails a key is named only once its lesson has opened it: the bar across the doorway stays (the men ARE waiting), the cap goes
    if crate::rails::rails_hushes(g, "word") {
        for d in out.iter_mut() { if d.why != WaitWhy::Pair { d.key = None; d.now = false; d.cap = KeyCap::None; } }
    }
}

/// what waits, as it is gathered: by the door it is at, with the charges of his own that the key is for and the men of a drill
struct Waiting<'a, 'g> {
    g: &'a Game,
    seat: &'a SeatRef<'g>,
    paired: bool,
    /// the whole squad's word is out, his own body is up, the men the word is for, his squad
    big: bool,
    up: bool,
    addr: Vec<GuardId>,
    squad: Vec<GuardId>,
    by: Vec<Wait>,
    charges: Vec<ChargeId>,
    walls: Vec<WallChargeId>,
    drill_men: Vec<GuardId>,
}
/// what the key pressed now would get to
struct Press { up: bool, pair_first: bool, reaches: bool, any_word: bool, row_key: bool, last: bool, drill_now: bool, free: Vec<ChargeId>, near: Option<ChargeId>, wall: Option<WallChargeId>, wall_first: bool }

impl<'a, 'g> Waiting<'a, 'g> {
    fn of(g: &'a Game, seat: &'a SeatRef<'g>, paired: bool) -> Self {
        // (a charge of his own that the squad is not at waits on HIS key, not this one)
        let charges = seat.door_tools.set.iter().copied().filter(|&c| crate::doortools::door_tools_key_is(g, seat, c)).collect();
        let walls = seat.wall_charge.set.iter().copied().filter(|&c| crate::wallcharge::wall_charge_key_is(g, seat, c)).collect();
        Waiting {
            g,
            seat,
            paired,
            big: crate::orders::squad_orders_big(g, seat),
            up: !crate::condition::down(g, g.pl[seat.player].ch),
            addr: crate::doortools::door_tools_word_men(g, seat),
            squad: crate::orders::squad_orders_squad(g, seat),
            by: Vec::new(),
            charges,
            walls,
            drill_men: Vec::new(),
        }
    }
    fn down(&self, gd: GuardId) -> bool { crate::condition::down(self.g, self.g.gd[gd].ch) }
    /// what is being done at a door and waits: a can held on its throw, or, with the whole squad's word out, a tool at the ready
    fn ops(&mut self) {
        for &o in self.seat.door_tools.ops.iter() {
            let op = &self.g.tab.doortools_door_op[o];
            if op.word.is_some() { continue; }
            let held = op.thr.is_some() && op.phase == Phase::Thr;
            if held || (self.big && matches!(op.phase, Phase::Armed | Phase::Standoff | Phase::Place)) {
                let men: Vec<GuardId> = op.men.iter().copied().filter(|&m| !self.down(m)).collect();
                let why = if held { WaitWhy::Thr } else if op.kind == crate::doortools::DoorToolKind::Charge { WaitWhy::Charge } else { WaitWhy::Tool };
                let word = men.iter().any(|m| self.addr.contains(m));
                let e = put(&mut self.by, op.door, &men, why, word);
                if held { if let Some(th) = &op.thr { if th.hold && !th.told { e.untold = true; } } }
            }
        }
    }
    /// the men stacked on a door
    fn stacks(&mut self) {
        let (g, seat) = (self.g, self.seat);
        if self.big {
            for &gd in self.squad.iter() {
                let Some(a) = crate::orders::squad_orders_of(seat, gd) else { continue };
                let a = &g.tab.task[a];
                if !self.down(gd) && crate::orders::rows::id_is(&a.order, crate::orders::Is::Stacks) && a.post.is_some() && a.there.at == crate::orders::At::Stacked {
                    if let Some(d) = a.door { put(&mut self.by, d, &[gd], WaitWhy::Stack, self.addr.contains(&gd)); }
                }
            }
        } else {
            // (one or two teammates on a plain stack wait on the key too; the key's own list, asked as the key asks it, is the addressed men's)
            for x in crate::doortools::door_tools_stack_waits(g, seat, &self.squad) { for &gd in &x.men { put(&mut self.by, x.stack, &[gd], WaitWhy::Stack, false); } }
            for x in crate::doortools::door_tools_stack_waits(g, seat, &self.addr) { for &gd in &x.men { put(&mut self.by, x.stack, &[gd], WaitWhy::Stack, true); } }
        }
    }
    /// the men of the squad who set a charge
    fn set_by(&self, name: &str) -> Vec<GuardId> { self.squad.iter().copied().filter(|&m| !self.down(m) && self.g.gd[m].callsign == name).collect() }
    /// a door's set charge
    fn set_charges(&mut self) {
        for c in self.charges.clone() {
            let sc = &self.g.tab.doortools_set_charge[c];
            let men = self.set_by(&sc.by);
            put(&mut self.by, sc.door, &men, WaitWhy::Charge, false).set = Some(c);
        }
    }
    /// what of the mission's own waits for the second word: a drill's pair, an entry held at its doors
    fn missions(&mut self) {
        for (door, men, drill) in crate::mission::waits(self.g) {
            if drill { self.drill_men = men.clone(); }
            let men: Vec<GuardId> = men.into_iter().filter(|&m| !self.down(m)).collect();
            let row = put(&mut self.by, door, &men, if drill { WaitWhy::Drill } else { WaitWhy::Stack }, false);
            if drill { row.drill = true; } else { row.gate = true; }
        }
    }
    /// The quiet pair's mark: the first reader of the key. The key a step ahead of each man who waits, on his gun's line: nothing of the man he waits on.
    /// A lesson that still holds the word shut lists no mark. → whether the key would be the pair's
    fn pair_marks(&self, out: &mut Vec<PlanDoor>) -> bool {
        let g = self.g;
        let marked = crate::pairorders::pair_orders_waiting(g, self.seat);
        let pair_first = self.up && !marked.is_empty();
        if !crate::rails::rails_locks(g, "word") {
            for (gd, j) in marked.iter() {
                let c = &g.ch[g.gd[*gd].ch];
                let (fx, fz) = (crate::m::sin(c.aim_yaw), crate::m::cos(c.aim_yaw));
                let (cx, cz) = (c.pos[0] + fx * g.k.plan.pair_bar_m, c.pos[2] + fz * g.k.plan.pair_bar_m);
                let key = Some(String::from("F"));
                let cap = cap_of(pair_first, j.set || j.watcher.is_some());
                out.push(PlanDoor { door: None, a: [cx - fz * 0.3, 0.0, cz + fx * 0.3], b: [cx + fz * 0.3, 0.0, cz - fx * 0.3], tone: tone_of(g, self.paired, *gd), key, now: pair_first, cap, why: WaitWhy::Pair });
            }
        }
        pair_first
    }
    /// how far the press would get, past whoever reads the key before
    fn press(&self, pair_first: bool) -> Press {
        let (g, ops, up, big) = (self.g, &self.seat.door_tools.ops, self.up, self.big);
        let sam = g.seat_pos(self.seat);
        // (a set charge not on a word already given)
        let free: Vec<ChargeId> = self.charges.iter().copied().filter(|&c| {
            let door = g.tab.doortools_set_charge[c].door;
            !ops.iter().any(|&o| { let op = &g.tab.doortools_door_op[o]; op.kind == crate::doortools::DoorToolKind::Charge && op.door == door && op.word.is_some_and(|w| g.tab.doortools_word[w].go1.is_nan()) })
        }).collect();
        let door_d = |c: ChargeId| v3::dist_xz(g.dr[g.tab.doortools_set_charge[c].door].frame_centre, sam);
        let near = free.iter().copied().reduce(|a, b| if door_d(b) < door_d(a) { b } else { a });
        let wall_foot = |c: WallChargeId| crate::wallcharge::wall_pt(g, &g.tab.wallcharge_set_wall_charge[c].wall, 0.0, 0.0, None);
        let wall = self.walls.iter().copied().reduce(|a, b| if v3::dist_xz(wall_foot(b), sam) < v3::dist_xz(wall_foot(a), sam) { b } else { a });
        let wall_d = wall.map_or(f64::INFINITY, |c| v3::dist_xz(wall_foot(c), sam));
        let wall_first = wall.is_some() && !self.charges.iter().any(|&c| door_d(c) < wall_d);
        let reaches = up && !pair_first && (!wall_first || self.charges.is_empty());   // the press gets as far as the door tools
        let any_word = self.by.iter().any(|e| e.word);
        let row_key = self.seat.orders.menu.as_ref().is_some_and(|m| m.rows.iter().any(|&d| g.tab.orders_order_def[d].word));   // a menu up with a row on the key: the key IS that row
        let early = big && ops.iter().any(|&o| {
            let op = &g.tab.doortools_door_op[o];
            op.thr.as_ref().is_some_and(|th| matches!(th.first, Some(crate::doortools::ThresholdFirst::Can | crate::doortools::ThresholdFirst::Burst))) && op.men.iter().any(|m| self.addr.contains(m))
        });
        let none_set = self.charges.is_empty() && self.walls.is_empty();
        let last = reaches && !any_word && !early && !row_key && none_set;   // … and past them all, to the rescue
        let drill_now = if any_word { self.drill_men.iter().any(|m| self.addr.contains(m)) } else { none_set };   // (the drill reads the key after them)
        Press { up, pair_first, reaches, any_word, row_key, last, drill_now, free, near, wall, wall_first }
    }
    /// every man of it on his place, his feet there: a gun still coming round does not put the cap out, and none taken off it by a fight
    fn set_up(&self, men: &[GuardId]) -> bool {
        men.iter().all(|&m| {
            if self.down(m) { return true; }
            let Some(a) = crate::orders::squad_orders_of(self.seat, m) else { return true };
            let a = &self.g.tab.task[a];
            a.yield_t < 0.0 && (a.arrived || a.post.is_none_or(|p| v3::dist_xz(self.g.pos_of(m), p) < 0.5))
        })
    }
    /// the bar across each doorway
    fn doors(&self, p: &Press, out: &mut Vec<PlanDoor>) {
        let g = self.g;
        let f = key_cap(&g.k.doortools.blow_key);
        for e in self.by.iter() {
            let (ja, jb) = jambs(g, e.door);
            let mut now = false;
            if p.reaches {
                if e.word || (e.gate && p.last) || (e.drill && p.drill_now) { now = true; }
                else if let Some(c) = e.set {
                    if p.free.contains(&c) { now = if self.big { !p.any_word && Some(c) == p.near } else { p.any_word || Some(c) == p.near }; }
                }
            }
            let cap = cap_of(now, self.set_up(&e.men) && !e.untold);
            out.push(PlanDoor { door: Some(g.dr[e.door].def.name.clone()), a: ja, b: jb, tone: tone_for(g, self.seat, self.paired, &e.men), key: Some(f.clone()), now, cap, why: e.why });
        }
    }
    /// the bar along each wall with a charge of his own
    fn wall_charges(&self, p: &Press, out: &mut Vec<PlanDoor>) {
        let g = self.g;
        let f = key_cap(&g.k.doortools.blow_key);
        for &c in &self.walls {
            let sc = &g.tab.wallcharge_set_wall_charge[c];
            let now = p.up && !p.pair_first && !p.row_key && p.wall_first && Some(c) == p.wall;
            let cap = cap_of(now, true);
            out.push(PlanDoor { door: None, a: crate::wallcharge::wall_pt(g, &sc.wall, -0.4, 0.03, None), b: crate::wallcharge::wall_pt(g, &sc.wall, 0.4, 0.03, None), tone: tone_for(g, self.seat, self.paired, &self.set_by(&sc.by)), key: Some(f.clone()), now, cap, why: WaitWhy::Wall });
        }
    }
}

/// the way he has still to walk, as far as the order says it
fn way_of(g: &Game, gd: GuardId, a: &crate::duty::Task, mark: V3) -> PlanWay {
    let m = &g.gd[gd];
    let p = g.ch[m.ch].pos;
    let mut stops: Vec<V3> = a.via.clone();
    stops.push(mark);
    // his own path, where it is this order's and runs all the way (a one-point side step to get unstuck is not his way)
    let own = match (m.path_goal, m.path.last()) {
        (Some(goal), Some(&end)) if m.path_i < m.path.len() && (v3::dist_xz(goal, stops[0]) < 0.9 || v3::dist_xz(goal, a.goal) < 0.9) && v3::dist_xz(end, goal) < 1.2 => Some(m.path[m.path_i..].to_vec()),
        _ => None,
    };
    PlanWay { from: crate::v3::on_floor(p), own, stops }
}

/// a seat's plan: its men's marks, what waits on its key, its throws, and, for its own player alone, who a menu is for
fn seat_plan(g: &Game, seat: &SeatRef, who: For) -> SeatPlan {
    let s = seat.orders;
    let mut out = SeatPlan { men: Vec::new(), who: Vec::new(), you: None, doors: Vec::new(), throws: Vec::new(), mode: true };
    if !crate::orders::squad_orders_active(g, seat) { return out; }
    // outside the order mode: only what the order book was just told
    out.mode = s.menu.is_some() || who.reading;
    let told = if out.mode { None } else { Some(just_told(g, seat)) };
    let paired = crate::verdict::team_paired(g, seat);
    let squad = crate::orders::squad_orders_squad(g, seat);
    for &gd in squad.iter() { out.men.extend(man_plan(g, seat, paired, gd, told.as_deref())); }
    waits_on_key(g, seat, paired, &mut out.doors);
    if !who.own { for d in out.doors.iter_mut() { d.key = None; d.now = false; d.cap = KeyCap::None; } }   // (the press is its own player's)
    throws_plan(g, seat, who, paired, &squad, &mut out.throws);
    if who.own {
        if let Some(menu) = &s.menu {
            for &gd in menu.men.iter() { if !crate::condition::down(g, g.gd[gd].ch) { out.who.push(PlanWho { man: g.char_id(gd), at: crate::v3::on_floor(g.pos_of(gd)), tone: tone_of(g, paired, gd) }); } }
            if !crate::condition::down(g, g.pl[seat.player].ch) { out.you = Some(crate::v3::on_floor(g.seat_pos(seat))); }
        }
    }
    out
}

/// a man an order has just named: his callsign, when, and how fast his mark ages (an order the squad took on itself is shown longer, so its ages slower)
struct Told { who: String, t: f64, slow: f64 }

/// the men the order book was just told of. The NEWEST thing that names a man is the one he is drawn by, and it alone
fn just_told(g: &Game, seat: &SeatRef) -> Vec<Told> {
    let (s, k) = (seat.orders, &g.k.plan);
    let far = crate::m::max(k.fresh_secs, g.k.autopilot.show_secs);
    let shown = crate::m::max(0.2, g.k.autopilot.show_secs);
    let mut last: Vec<(String, f64, f64)> = Vec::new();
    let mut note = |c: &String, t: f64, secs: f64| match last.iter_mut().find(|e| &e.0 == c) {
        Some(e) => if t > e.1 { e.1 = t; e.2 = secs; },
        None => last.push((c.clone(), t, secs)),
    };
    for row in s.log.iter().rev() {
        if !(g.time - row.t < far && row.t <= g.time) { break; }
        let secs = if row.auto == Some(true) { shown } else { k.fresh_secs };
        for c in &row.who { note(c, row.t, secs); }
    }
    for row in s.shows.iter().rev() {
        if !(g.time - row.t < far && row.t <= g.time) { break; }
        for c in &row.who { note(c, row.t, shown); }
    }
    last.into_iter().filter(|e| g.time - e.1 < e.2).map(|(who, t, secs)| Told { who, t, slow: if secs != k.fresh_secs { k.fresh_secs / secs } else { 1.0 } }).collect()
}

/// one man of the squad's: his mark, the way he has still to walk to it, where he looks. None = he has no mark to show, or (with a list of them) was not
/// just told of one
fn man_plan(g: &Game, seat: &SeatRef, paired: bool, gd: GuardId, told: Option<&[Told]>) -> Option<PlanMan> {
    let m = &g.gd[gd];
    if g.cond[m.ch].down { return None; }
    let a = &g.tab.task[crate::orders::squad_orders_of(seat, gd)?];
    let post = a.post?;
    let mut age = crate::m::max(0.0, g.time - a.t0);
    if let Some(told) = told {
        let said = told.iter().find(|e| e.who == m.callsign)?;
        if a.t0 < said.t - 0.02 { return None; }   // (his since that order: a door tool hands him his mark a beat after it)
        age = crate::m::max(age, g.time - said.t) * said.slow;
    }
    if a.foe.is_some_and(|f| !crate::fogofwar::fog_of_war_seen(g, f)) { return None; }   // (a mark taken from a man the team cannot see would walk the picture to him)
    // a step off the mark that the order's own goal names: that is where he will stand
    let mark = crate::v3::on_floor(if a.via.is_empty() && v3::dist_xz(a.goal, post) > 0.05 { a.goal } else { post });
    let off = a.yield_t >= 0.0;
    let c = &g.ch[m.ch];
    let there = !off && v3::dist_xz(c.pos, mark) < 0.45;
    let walking = !off && !there;
    let (aim, watch, on_door) = man_looks(g, a, c, mark, there, walking);
    Some(PlanMan {
        man: c.id, tag: plan_tag(&m.callsign), tone: tone_of(g, paired, gd), age,
        way: if walking { Some(way_of(g, gd, a, mark)) } else { None },
        mark, there,
        held: off || m.script.as_ref().is_some_and(|x| x.over.is_some()),
        aim,
        watch,
        on_door: watch.is_some() && on_door,
    })
}

/// the way he faces on his mark, what he was told to watch from it if he does, and whether that is his door
fn man_looks(g: &Game, a: &crate::duty::Task, c: &crate::character::Character, mark: V3, there: bool, walking: bool) -> (Option<f64>, Option<V3>, bool) {
    let fit_watch = a.fit.as_ref().and_then(|f| f.watch);
    let raw = a.watch.or(fit_watch);
    let on_door = raw.is_some_and(|w| a.door.is_some_and(|d| v3::dist_xz(w, g.dr[d].frame_centre) < 1.5));
    let told = raw.filter(|&w| a.foe.is_none() && (on_door || Some(w) == fit_watch || a.order == crate::orders::rows::cover::ORDER.id || a.order == crate::orders::rows::movethere::ORDER.id || a.order == crate::orders::rows::hold::ORDER.id));
    let (mut aim, mut watch): (Option<f64>, Option<V3>) = (None, None);
    if there {
        aim = Some(c.aim_yaw);
        if let Some(w) = told { if ang_diff(v3::yaw_to(c.pos, w), c.aim_yaw) < 0.4 { watch = Some(w); } }
    } else if walking {
        match told {
            Some(w) if v3::dist_xz(mark, w) > 0.3 => { aim = Some(v3::yaw_to(mark, w)); watch = Some(w); }
            _ => if a.foe.is_none() { aim = a.fit.as_ref().map(|f| f.face).or(a.face); },
        }
    }
    if watch.is_some_and(|w| v3::dist_xz(mark, w) > g.k.plan.sight_m) { watch = None; }
    (aim, watch, on_door)
}

/// the squad's throws: the walk to it, the arc and where it lands
fn throws_plan(g: &Game, seat: &SeatRef, who: For, paired: bool, squad: &[GuardId], out: &mut Vec<PlanThrow>) {
    for t in throws_view(g, seat) {
        if t.kind == ThrowKind::Shell || t.points.len() < 2 || (t.preview && !who.own) { continue; }
        let Some(&gd) = squad.iter().find(|&&m| g.char_id(m) == t.man) else { continue };
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let walk = match &t.walk { Some(w) if w.len() > 1 => w.iter().map(|&q| crate::v3::on_floor(q)).collect(), _ => Vec::new() };
        let end = t.points[t.points.len() - 1];
        let mut arc = t.points.clone();
        if v3::dist_xz(end, t.land) > 0.1 { arc.push(crate::v3::on_floor(t.land)); }
        out.push(PlanThrow { man: t.man, tag: plan_tag(&g.gd[gd].callsign), tone: tone_of(g, paired, gd), kind: t.kind, preview: t.preview, from: crate::v3::on_floor(t.from), walk, arc, land: crate::v3::on_floor(t.land), reach: if t.kind == ThrowKind::Frag { t.kill } else { 0.0 } });
    }
}

/// the same wait in two plans (a door's, or a wall's two ends)
fn same_wait(a: &PlanDoor, b: &PlanDoor) -> bool {
    (a.door.is_some() && a.door == b.door) || (crate::m::abs(a.a[0] - b.a[0]) + crate::m::abs(a.a[2] - b.a[2]) + crate::m::abs(a.b[0] - b.b[0]) + crate::m::abs(a.b[2] - b.b[2]) < 0.02)
}

/// the plan on the floor, about the seat it is handed: its own whole, then each other player's (a wait both plans list is on the page once, as the seat's own)
pub(super) fn plan_view(g: &Game, seat: &SeatRef) -> PlanView {
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
    PlanView { own, others }
}
