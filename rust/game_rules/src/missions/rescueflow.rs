//! The hostage rescue's dynamic tempo: what the pair do about the openings Sam runs past, and what is known of each.

pub use ::game_types::rescueflow::types;
pub use types::*;
use crate::v3::dist_xz;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// The door of an opening, if it names one and the level has it.
fn door_of(g: &Game, o: &crate::scene_scenarios::RouteOpening) -> Option<DoorId> {
    match &o.door {
        Some(n) if !n.is_empty() => crate::doors::doors_by_name(g, n),
        _ => None,
    }
}

/// Mark a hostile as known in a region, noting a room newly hot.
fn set_hot(g: &mut Game, r: RescueflowRegionId, h: GuardId, t: f64, at: V3) {
    if !g.tab.rescueflow_region[r].hot.has(h) {
        let note = alloc::format!("flow:hot:{}", g.tab.rescueflow_region[r].id);
        crate::rescue::rescue_note(g, &note);
    }
    g.tab.rescueflow_region[r].hot.set(h, RegionHotValue { t, at: v3::copy(at) });
}

/// The level's route data; None = this level's rescue has no flow.
fn rescue_flow_f(g: &crate::game::Game) -> Option<crate::scene_scenarios::RescueFlowData> {
    crate::rescue::rescue_data(g).and_then(|d| d.flow)
}

pub(super) fn rescue_flow_new() -> crate::rescueflow::RescueFlow {
    RescueFlow::blank()
}

/// A fresh staging: nothing known, nobody on a job.
pub(super) fn rescue_flow_reset(g: &mut crate::game::Game) {
    let f = &mut g.rescue.flow;
    f.jobs.clear();
    f.passed.clear();
    f.want.clear();
    f.sense_at = 0.0;
    f.turn = 0.0;
    f.built = false;
    f.known = Vec::new();
    f.regions.clear();
    f.corr = None;
    f.came_in = None;
    f.stats = RescueFlowStats::default();
}

/// Is this assignment one the flow gave.
pub(super) fn rescue_flow_gave(g: &crate::game::Game, a: Option<OrdersAssignmentId>) -> bool {
    match a {
        Some(a) => g.rescue.flow.mine.has(a),
        None => false,
    }
}

/// F has taken these men: their jobs end where they stand.
pub(super) fn rescue_flow_release(g: &mut crate::game::Game, men: &[GuardId]) {
    for &gd in men {
        g.rescue.flow.jobs.delete(gd);
    }
}

/// Sample each region's floor and make a row per opening.
fn rescue_flow_build(g: &mut crate::game::Game) {
    let f = rescue_flow_f(g);
    g.rescue.flow.built = true;
    g.rescue.flow.known = Vec::new();
    g.rescue.flow.regions.clear();
    let Some(f) = f else { return };
    if let Some(b) = crate::rescue::rescue_bounds(g) {
        g.rescue.flow.room = b;
    }
    let sample_y = g.rescue.flow.knobs.sample_y;
    for reg in f.regions.iter() {
        let id = reg.id.clone();
        // a region with a clear plan on its door has the plan's bounds, if the first plan of that door has any
        let plan = match &reg.door {
            Some(d) if !d.is_empty() => match &g.level.clear_plans {
                Some(plans) => plans.iter().find(|p| p.door == *d).and_then(|p| p.bounds),
                None => None,
            },
            _ => None,
        };
        let rect = match plan {
            Some(b) => crate::scene_scenarios::Rect { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 },
            None => reg.rect,
        };
        let mut pts: Vec<V3> = Vec::new();
        let mut x = rect.x0 + 0.4;
        while x < rect.x1 {
            let mut z = rect.z0 + 0.4;
            while z < rect.z1 {
                if !crate::scene_collision::nav_grid_is_blocked(g, x, z) {
                    pts.push([x, sample_y, z]);
                }
                z += 1.0;
            }
            x += 1.0;
        }
        let n = pts.len();
        let row = g.tab.rescueflow_region.push(Region { id: id.clone(), rect, pts, seen_t: vec![-1e9; n], cleared: false, hot: JsMap::new(), was_in: -1e9 });
        g.rescue.flow.regions.set(id, row);
    }
    for o in f.route.iter() {
        let region = *g.rescue.flow.regions.get(o.region.clone()).unwrap();
        let mut k = Known {
            o: o.clone(),
            region,
            lookable: Vec::new(),
            built_open: false,
            share: 0.0,
            state: OpenState::Unknown,
            held: None,
            hot: false,
        };
        rescue_flow_lookable(g, &mut k);
        g.rescue.flow.known.push(k);
    }
}

/// What of the room can be seen through this opening at all, from the cover mark and three places across its threshold. `k` is not in the table while this runs.
fn rescue_flow_lookable(g: &crate::game::Game, k: &mut crate::rescueflow::Known) {
    let (mark, at) = (k.o.mark, k.o.at);
    let dz = if k.o.north { 0.7 } else { -0.7 };
    let eyes: [V3; 4] = [[mark[0], EYE, mark[2]], [at[0], EYE, at[2] + dz], [at[0] - 0.9, EYE, at[2] + dz], [at[0] + 0.9, EYE, at[2] + dz]];
    k.lookable = Vec::new();
    k.built_open = rescue_flow_open(g, &k.o);
    let eye_m = g.rescue.flow.knobs.eye_m;
    let pts = g.tab.rescueflow_region[k.region].pts.clone();
    for (i, p) in pts.iter().enumerate() {
        for e in eyes.iter() {
            if dist_xz(*e, *p) < eye_m && !crate::scene_collision::segment_blocked(g, *e, *p, None, None, None) {
                k.lookable.push(i as f64);
                break;
            }
        }
    }
}

fn rescue_flow_open(g: &crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> bool {
    match door_of(g, o) {
        None => true,
        Some(d) => m::abs(g.dr[d].leaf.angle) > 0.5,
    }
}

/// The team's eyes this moment: Sam's, and every living operative's who is not blind.
fn rescue_flow_eyes(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<V3> {
    let mut out: Vec<V3> = Vec::new();
    let pl = seat.player;
    if !crate::condition::down(g, g.pl[pl].ch) {
        let p = g.player_pos(pl);
        out.push([p[0], EYE, p[2]]);
    }
    for &op in g.staged.team.iter() {
        if !crate::condition::down(g, g.gd[op].ch) && !crate::control::man::blind(g, op) {
            let p = g.pos_of(op);
            out.push([p[0], EYE, p[2]]);
        }
    }
    out
}

/// Is this man of theirs seen by the team: the room fight's answer, else the fog's while it is up, else a plain line from any eye to his chest.
fn rescue_flow_sees(g: &crate::game::Game, eyes: &[V3], h: GuardId) -> bool {
    if crate::roomfight::room_fight_team_sees(g, h) {
        return true;
    }
    if crate::fogofwar::fog_of_war_active(g) {
        return crate::fogofwar::fog_of_war_seen(g, h);
    }
    let ch = g.gd[h].ch;
    let c = match g.ch[ch].bones.get(crate::character::Bone::Chest) {
        Some(&c) => c,
        None => v3::add(g.ch[ch].pos, [0.0, 1.1, 0.0]),
    };
    for &e in eyes {
        if dist_xz(e, c) < 30.0 && !crate::scene_collision::segment_blocked(g, e, c, None, None, None) {
            return true;
        }
    }
    false
}

fn rescue_flow_region_at(g: &crate::game::Game, p: V3) -> Option<RescueflowRegionId> {
    g.rescue.flow.regions.values().copied().find(|&r| super::roomclear::in_rect(p, g.tab.rescueflow_region[r].rect))
}

/// What the stages of a sensing share: the moment, the knobs, the team's eyes and every room.
struct Sensing {
    t: f64,
    k: RescueFlowKnobs,
    eyes: Vec<V3>,
    rooms: Vec<RescueflowRegionId>,
}

/// A few times a second: the floor seen, who of theirs is known where, what each room and opening reads.
fn rescue_flow_sense(g: &mut crate::game::Game) {
    let eyes = rescue_flow_eyes(g, &crate::seats::here(g));
    let s = Sensing { t: g.time, k: g.rescue.flow.knobs, eyes, rooms: g.rescue.flow.regions.values().copied().collect() };
    look_over_a_room(g, &s);
    for &r in s.rooms.iter() {
        let rect = g.tab.rescueflow_region[r].rect;
        if s.eyes.iter().any(|&e| super::roomclear::in_rect(e, rect)) {
            g.tab.rescueflow_region[r].was_in = s.t;
        }
    }
    for i in 0..g.staged.foes.len() {
        let h = g.staged.foes[i];
        place_him(g, h, &s);
    }
    hear_their_shots(g, &s);
    for &r in s.rooms.iter() {
        let_cold_men_go(g, r, &s);
    }
    for &r in s.rooms.iter() {
        read_the_room(g, r, &s);
    }
    for at in 0..g.rescue.flow.known.len() {
        read_the_opening(g, at, &s);
    }
}

/// The floor the eyes are on: ONE room a call, in turn.
fn look_over_a_room(g: &mut Game, s: &Sensing) {
    let turn = g.rescue.flow.turn;
    g.rescue.flow.turn = turn + 1.0;
    if s.rooms.is_empty() {
        return;
    }
    let (t, eye_m) = (s.t, s.k.eye_m);
    let r = s.rooms[js::rem(turn, s.rooms.len() as f64) as usize];
    let rect = g.tab.rescueflow_region[r].rect;
    let n = g.tab.rescueflow_region[r].pts.len();
    for &e in s.eyes.iter() {
        if e[0] < rect.x0 - eye_m || e[0] > rect.x1 + eye_m || e[2] < rect.z0 - eye_m || e[2] > rect.z1 + eye_m {
            continue;
        }
        for i in 0..n {
            if t - g.tab.rescueflow_region[r].seen_t[i] < 0.5 {
                continue;
            }
            let p = g.tab.rescueflow_region[r].pts[i];
            if dist_xz(e, p) < eye_m && !crate::scene_collision::segment_blocked(g, e, p, None, None, None) {
                g.tab.rescueflow_region[r].seen_t[i] = t;
            }
        }
    }
}

/// Who of theirs is known to be where: seen in a room marks it hot; seen anywhere else, or dead, lets it go.
fn place_him(g: &mut Game, h: GuardId, s: &Sensing) {
    if crate::condition::down(g, g.gd[h].ch) {
        for &r in s.rooms.iter() {
            g.tab.rescueflow_region[r].hot.delete(h);
        }
        return;
    }
    if !rescue_flow_sees(g, &s.eyes, h) {
        return;
    }
    let pos = g.pos_of(h);
    let at = rescue_flow_region_at(g, pos);
    for &r in s.rooms.iter() {
        if at == Some(r) {
            set_hot(g, r, h, s.t, pos);
        } else {
            g.tab.rescueflow_region[r].hot.delete(h);
        }
    }
}

/// A shot of theirs from inside a room says as much as a sight of him; each event is heard once.
fn hear_their_shots(g: &mut Game, s: &Sensing) {
    for at in 0..g.events.len() {
        let key = g.events[at].serial;
        if g.rescue.flow.heard.has(key) {
            continue;
        }
        g.rescue.flow.heard.add(key);
        if g.events[at].kind != crate::director::EventKind::GuardShot {
            continue;
        }
        let (who, pos) = (g.events[at].who, g.events[at].pos);
        let Some(h) = g.staged.foes.iter().copied().find(|&x| Some(g.char_id(x)) == who) else { continue };
        if crate::condition::down(g, g.gd[h].ch) {
            continue;
        }
        if let Some(r) = rescue_flow_region_at(g, pos) {
            set_hot(g, r, h, s.t, pos);
        }
    }
}

/// Those known in this room whom it has gone cold on come off it, with a note each.
fn let_cold_men_go(g: &mut Game, r: RescueflowRegionId, s: &Sensing) {
    let hot: Vec<(GuardId, RegionHotValue)> = g.tab.rescueflow_region[r].hot.iter().map(|(h, e)| (h, *e)).collect();
    for (h, e) in hot {
        if gone_cold(g, r, e, s) {
            g.tab.rescueflow_region[r].hot.delete(h);
            let note = alloc::format!("flow:cold:{}", g.tab.rescueflow_region[r].id);
            crate::rescue::rescue_note(g, &note);
        }
    }
}

/// A hot room goes cold: nothing of him for a long while, or eyes on the spot he was last at, the room looked through since, and no sign of him.
fn gone_cold(g: &Game, r: RescueflowRegionId, e: RegionHotValue, s: &Sensing) -> bool {
    let (t, k) = (s.t, s.k);
    if t - e.t < 1.0 {
        return false;
    }
    if t - e.t > k.hot_secs {
        return true;
    }
    let row = &g.tab.rescueflow_region[r];
    if row.pts.is_empty() {
        return false;
    }
    let looked = (0..row.pts.len()).filter(|&i| row.seen_t[i] > e.t + 0.5).count() as f64;
    let share = looked / row.pts.len() as f64;
    let spot: V3 = [e.at[0], 1.1, e.at[2]];
    share >= k.hot_empty && s.eyes.iter().any(|&eye| dist_xz(eye, spot) < k.eye_m && !crate::scene_collision::segment_blocked(g, eye, spot, None, None, None))
}

/// A room is cleared with nobody of theirs known in it and, once, a man in it with enough of its floor lately seen.
fn read_the_room(g: &mut Game, r: RescueflowRegionId, s: &Sensing) {
    let (t, k) = (s.t, s.k);
    let row = &mut g.tab.rescueflow_region[r];
    let seen = (0..row.pts.len()).filter(|&i| t - row.seen_t[i] < k.clear_secs).count() as f64;
    let was = row.cleared;
    row.cleared = row.hot.is_empty() && (row.cleared || (t - row.was_in < 1.0 && !row.pts.is_empty() && seen / row.pts.len() as f64 >= k.clear_share));
    if row.cleared && !was {
        let note = alloc::format!("flow:cleared:{}", row.id);
        crate::rescue::rescue_note(g, &note);
    }
}

/// What this opening reads now, with a note when that changes.
fn read_the_opening(g: &mut Game, at: usize, s: &Sensing) {
    let (t, k) = (s.t, s.k);
    // (the row out of the table for the calls, and back before the note and the message: open, lookable and holder do not read `known`)
    let mut row = core::mem::take(&mut g.rescue.flow.known[at]);
    if row.built_open != rescue_flow_open(g, &row.o) {
        rescue_flow_lookable(g, &mut row);
    }
    let r = &g.tab.rescueflow_region[row.region];
    let (hot, cleared) = (!r.hot.is_empty(), r.cleared);
    let looked = row.lookable.iter().filter(|&&ix| t - r.seen_t[ix as usize] < k.look_secs).count() as f64;
    row.share = if !row.lookable.is_empty() { looked / row.lookable.len() as f64 } else { 0.0 };
    row.hot = hot;
    row.held = rescue_flow_holder(g, &crate::seats::here(g), &row.o);
    let was = row.state;
    row.state = if hot {
        OpenState::Hot
    } else if row.held.is_some() {
        OpenState::Held
    } else if cleared {
        OpenState::Cleared
    } else if row.share >= k.look_share {
        OpenState::Looked
    } else {
        OpenState::Unknown
    };
    let state = row.state;
    g.rescue.flow.known[at] = row;
    if state != was {
        let note = alloc::format!("flow:state:{}:{}", g.rescue.flow.known[at].o.id, state.as_str());
        crate::rescue::rescue_note(g, &note);
    }
}

/// The man posted on this opening: on a mark of his own with his gun on it, the flow's cover or a menu order of Sam's.
fn rescue_flow_holder(g: &crate::game::Game, seat: &crate::seats::SeatRef, o: &crate::scene_scenarios::RouteOpening) -> Option<GuardId> {
    let mut i = 0;
    while i < g.staged.team.len() {
        let op = g.staged.team[i];
        i += 1;
        if crate::condition::down(g, g.gd[op].ch) {
            continue;
        }
        let Some(a) = crate::orders::squad_orders_of(seat, op) else { continue };
        let Some(post) = g.tab.orders_assignment[a].post else { continue };
        if dist_xz(g.pos_of(op), post) > 1.2 {
            continue;
        }
        if let Some(j) = g.rescue.flow.jobs.get(op) {
            if j.a == a {
                // the flow's own man holds HIS opening and no other (the corridor's man holds the corridor)
                if j.kind == JobKind::Cover && j.o.id == o.id {
                    return Some(op);
                }
                continue;
            }
        }
        let row = &g.tab.orders_assignment[a];
        let on_door = match row.door {
            Some(d) => o.door.as_deref() == Some(g.dr[d].def.name.as_str()),
            None => false,
        };
        let on_watch = match row.watch {
            Some(w) => dist_xz(post, o.at) < 6.0 && (dist_xz(w, o.at) < 1.5 || dist_xz(w, o.watch) < 1.5),
            None => false,
        };
        if on_door || on_watch {
            return Some(op);
        }
    }
    None
}

/// What a frame's stages share: the knobs, the moment, and where Sam is on the route.
struct FlowNow {
    k: RescueFlowKnobs,
    t: f64,
    sam: V3,
    in_corr: bool,
    on_route: bool,
    /// the side room he is in, off the route
    sam_in: Option<String>,
    /// the clock runs, nothing has ended it and Sam is on his feet
    live: bool,
    dynamic: bool,
}

/// What a man's turn reads of his job, as locals: a job only ever ends or changes on its own man's turn; the row itself is cloned only once the job is done.
#[derive(Clone, Copy)]
struct OnJob {
    kind: JobKind,
    a: OrdersAssignmentId,
    t0: f64,
    on_t: f64,
    said_hot: bool,
    said_wait: bool,
    leap: bool,
    has_chain: bool,
    mark: V3,
    at: V3,
    look: f64,
}

/// Once a frame from Rescue.tick.
pub(super) fn rescue_flow_tick(g: &mut crate::game::Game) {
    let k = g.rescue.flow.knobs;
    let Some(f) = rescue_flow_f(g).filter(|_| crate::rescue::rescue_up(g)) else {
        send_everyone_back(g);
        return;
    };
    if !g.rescue.flow.built {
        rescue_flow_build(g);
    }
    if g.time >= g.rescue.flow.sense_at {
        g.rescue.flow.sense_at = g.time + 1.0 / (k.sense_hz * m::max(1.0, g.rescue.flow.regions.len() as f64));
        rescue_flow_sense(g);
    }
    rescue_flow_boxes(g);
    let now = flow_now(g, &f, k);
    follow_sam(g, &f.route, &now);
    if !now.dynamic {
        send_everyone_back(g);
        return;
    }
    if g.rescue.result.is_some() || crate::condition::down(g, g.pl[g.player].ch) {
        // (it is over: whoever was covering stays covering)
        g.rescue.flow.jobs.clear();
        g.rescue.flow.want.clear();
        return;
    }
    for gd in g.rescue.flow.jobs.keys() {
        job_turn(g, gd, &now);
    }
    if crate::rescue::rescue_mode(g) == crate::rescue::RescueMode::Door {
        // F has the floor: whoever it left out stays where Sam put him
        g.rescue.flow.want.clear();
        return;
    }
    if !g.rescue.flow.want.is_empty() && rescue_flow_stays(g, &crate::seats::here(g)) {
        for id in g.rescue.flow.want.keys() {
            crate::rescue::rescue_note(g, &alloc::format!("flow:stay:{}", id));
        }
        g.rescue.flow.want.clear();
    }
    for o in f.route.iter() {
        man_the_opening(g, o, &now);
    }
    hold_the_corridor(g, &f, &now);
}

/// With anybody on a job the flow lets them all go, back into the file; with nobody on one nothing changes.
fn send_everyone_back(g: &mut Game) {
    if !g.rescue.flow.jobs.is_empty() {
        rescue_flow_drop_all(g, true);
    }
}

/// Where Sam stands this frame, read once.
fn flow_now(g: &Game, f: &crate::scene_scenarios::RescueFlowData, k: RescueFlowKnobs) -> FlowNow {
    let pl = g.player;
    let sam = g.player_pos(pl);
    let dynamic = crate::rescue::rescue_dynamic(g);
    let live = dynamic && g.rescue.result.is_none() && !crate::condition::down(g, g.pl[pl].ch);
    let in_corr = super::roomclear::in_rect(sam, f.corridor);
    let on_route = in_corr || super::roomclear::in_rect(sam, g.rescue.flow.room);
    let sam_in = if on_route { None } else { rescue_flow_region_at(g, sam).map(|r| g.tab.rescueflow_region[r].id.clone()) };
    FlowNow { k, t: g.time, sam, in_corr, on_route, sam_in, live, dynamic }
}

/// What the corridor knows of Sam: the openings he has come past on it, or the one he left it by; back in the room it forgets that.
fn follow_sam(g: &mut Game, route: &[crate::scene_scenarios::RouteOpening], now: &FlowNow) {
    if now.in_corr {
        openings_come_past(g, route, now);
    } else if let Some(id) = &now.sam_in {
        way_into_side_room(g, route, id, now);
    } else if now.on_route {
        g.rescue.flow.came_in = None;
    }
}

/// What Sam has come past: on the corridor short of it a moment ago, on the corridor abreast of it now.
fn openings_come_past(g: &mut Game, route: &[crate::scene_scenarios::RouteOpening], now: &FlowNow) {
    let (t, sam) = (now.t, now.sam);
    let from = match g.rescue.flow.corr {
        Some(c) if t - c.t < 0.5 => Some(c.x),
        _ => None,
    };
    for o in route.iter() {
        let has = g.rescue.flow.passed.has(o.id.clone());
        let line = o.at[0] - now.k.abreast;
        if !has && from.is_some_and(|x| x <= line) && sam[0] > line {
            g.rescue.flow.passed.add(o.id.clone());
            crate::rescue::rescue_note(g, &alloc::format!("flow:pass:{}", o.id));
            if now.live {
                g.rescue.flow.want.set(o.id.clone(), t);
            }
        } else if has && sam[0] < o.at[0] - now.k.unpass {
            g.rescue.flow.passed.delete(o.id.clone());
            g.rescue.flow.want.delete(o.id.clone());
        }
    }
    g.rescue.flow.corr = Some(RescueFlowCorr { x: sam[0], t });
    g.rescue.flow.came_in = None;
}

/// Off the corridor into a side room: by the opening of that room nearest him as he went in.
fn way_into_side_room(g: &mut Game, route: &[crate::scene_scenarios::RouteOpening], id: &String, now: &FlowNow) {
    if g.rescue.flow.came_in.as_ref().map(|c| &c.region) == Some(id) {
        return;
    }
    let (t, sam) = (now.t, now.sam);
    let nearest = route.iter().filter(|x| x.region == *id).reduce(|a, x| if dist_xz(a.at, sam) <= dist_xz(x.at, sam) { a } else { x });
    let recent = matches!(g.rescue.flow.corr, Some(c) if t - c.t < 0.5);
    g.rescue.flow.came_in = match nearest {
        Some(o) if recent => Some(RescueFlowCameIn { region: id.clone(), o: o.clone(), t }),
        _ => None,
    };
}

/// One man on a job, his turn: he comes off it, or fights, or holds on a contact, or looks on until it is done and he moves on.
fn job_turn(g: &mut Game, gd: GuardId, now: &FlowNow) {
    let Some(j) = g.rescue.flow.jobs.get(gd) else { return };
    let job = OnJob {
        kind: j.kind,
        a: j.a,
        t0: j.t0,
        on_t: j.on_t,
        said_hot: j.said_hot,
        said_wait: j.said_wait,
        leap: j.leap,
        has_chain: !j.chain.is_empty(),
        mark: j.o.mark,
        at: j.o.at,
        look: j.o.look,
    };
    let (k, t, sam) = (now.k, now.t, now.sam);
    if off_the_job(g, gd, job.a, t) || fight_ai_has(g, gd) {
        return;
    }
    if job.kind == JobKind::Corridor {
        corridor_man_turn(g, gd, now);
        return;
    }
    let pos = g.pos_of(gd);
    let on_t = on_his_mark_since(g, gd, &job, pos, t);
    if holds_on_a_contact(g, gd, job.said_hot, now) {
        return;
    }
    if crate::roomfight::room_fight_engaged(g, gd) {
        return; // (the man he was holding on has just gone from his sight: not released on that frame)
    }
    let sam_past = !now.on_route || sam[0] > job.at[0] + k.past;
    let done = if on_t.is_nan() { t - job.t0 > k.give_up } else { t - on_t >= job.look && (sam_past || t - on_t >= k.max_cover) };
    if !job.said_wait && job.has_chain && now.on_route && sam[0] - pos[0] > k.wait_m {
        ask_sam_to_wait(g, gd);
    }
    if done {
        move_on(g, gd, &job);
    }
}

/// His job ends here: he is down or F has him (what was queued behind a man down is wanted again), or Sam's own order has him.
fn off_the_job(g: &mut Game, gd: GuardId, job_a: OrdersAssignmentId, t: f64) -> bool {
    let down = crate::condition::down(g, g.gd[gd].ch);
    if down || crate::rescue::rescue_owns(g, gd) {
        let gone = g.rescue.flow.jobs.delete(gd);
        if down {
            for o in gone.iter().flat_map(|j| j.chain.iter()) {
                g.rescue.flow.want.set(o.id.clone(), t);
            }
        }
        return true;
    }
    if crate::orders::squad_orders_of(&crate::seats::here(g), gd) != Some(job_a) {
        // Sam's own order has him: the flow lets go, and does not send the next man to the same openings
        g.rescue.flow.jobs.delete(gd);
        let note = alloc::format!("flow:override:{}", g.gd[gd].callsign);
        crate::rescue::rescue_note(g, &note);
        return true;
    }
    false
}

/// The fight AI has him: his fight first.
fn fight_ai_has(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert
}

/// The corridor's man stays until Sam is back on the route.
fn corridor_man_turn(g: &mut Game, gd: GuardId, now: &FlowNow) {
    if now.on_route || now.sam_in.is_none() {
        rescue_flow_rejoin(g, gd, Some("on you."));
        let note = alloc::format!("flow:done:corridor:{}", g.gd[gd].callsign);
        crate::rescue::rescue_note(g, &note);
    }
}

/// The moment he got to his mark, written the frame he does; NaN until then.
fn on_his_mark_since(g: &mut Game, gd: GuardId, job: &OnJob, pos: V3, t: f64) -> f64 {
    if job.on_t.is_nan() && (g.tab.orders_assignment[job.a].arrived || dist_xz(pos, job.mark) < 0.6) {
        g.rescue.flow.jobs.get_mut(gd).unwrap().on_t = t;
        return t;
    }
    job.on_t
}

/// One of theirs is known in his opening's room: it is held for as long as he has lately been seen or heard there, and the man says so once.
fn holds_on_a_contact(g: &mut Game, gd: GuardId, said_hot: bool, now: &FlowNow) -> bool {
    let j = g.rescue.flow.jobs.get(gd).unwrap();
    let side = j.o.side;
    let known = g.rescue.flow.known.iter().find(|x| x.o.id == j.o.id).unwrap();
    if !known.hot {
        return false;
    }
    let times: Vec<f64> = g.tab.rescueflow_region[known.region].hot.values().map(|e| e.t).collect();
    let lately = now.t - m::max_n(&times) < now.k.hot_hold;
    if !lately {
        return false;
    }
    if !said_hot {
        g.rescue.flow.jobs.get_mut(gd).unwrap().said_hot = true;
        crate::game::game_say(g, gd, &alloc::format!("contact {} — holding.", side.as_str()), None, None);
        let note = alloc::format!("flow:hold:{}:{}", g.rescue.flow.jobs.get(gd).unwrap().o.id, g.gd[gd].callsign);
        crate::rescue::rescue_note(g, &note);
    }
    true
}

/// A man with more queued and Sam far ahead of him says so, once.
fn ask_sam_to_wait(g: &mut Game, gd: GuardId) {
    g.rescue.flow.jobs.get_mut(gd).unwrap().said_wait = true;
    crate::game::game_say(g, gd, "wait one.", None, None);
    let note = alloc::format!("flow:wait:{}", g.gd[gd].callsign);
    crate::rescue::rescue_note(g, &note);
}

/// His opening is done: on to the next queued one that still wants a man, unless the pair stays; else back to the file.
fn move_on(g: &mut Game, gd: GuardId, job: &OnJob) {
    // (the chain is shifted on a copy: from here this job is replaced by take or ended by rejoin, so the row's own list is never read again)
    let mut j = g.rescue.flow.jobs.get(gd).cloned().unwrap();
    let note = alloc::format!("flow:done:{}:{}", j.o.id, g.gd[gd].callsign);
    crate::rescue::rescue_note(g, &note);
    let mut next = next_unsettled(g, &mut j.chain);
    if let Some(n) = &next {
        if rescue_flow_stays(g, &crate::seats::here(g)) {
            crate::rescue::rescue_note(g, &alloc::format!("flow:stay:{}", n.id));
            next = None;
        }
    }
    if let Some(n) = &next {
        rescue_flow_take(g, gd, n, &j.chain, j.said_wait, true);
        g.rescue.flow.stats.chained += 1.0;
        return;
    }
    let words = if job.said_hot {
        Some("lost him — moving.")
    } else if job.leap {
        Some("moving.")
    } else {
        None
    };
    rescue_flow_rejoin(g, gd, words);
}

/// The first opening on the chain that still wants a man; it and those settled before it come off the chain.
fn next_unsettled(g: &Game, chain: &mut Vec<crate::scene_scenarios::RouteOpening>) -> Option<crate::scene_scenarios::RouteOpening> {
    while !chain.is_empty() {
        let n = chain.remove(0);
        if !rescue_flow_settled(g, &n) {
            return Some(n);
        }
    }
    None
}

/// An opening just passed: the man already working the openings takes it next; else the last free man in the file.
fn man_the_opening(g: &mut Game, o: &crate::scene_scenarios::RouteOpening, now: &FlowNow) {
    let Some(&at) = g.rescue.flow.want.get(o.id.clone()) else { return };
    let taken = g.rescue.flow.jobs.values().any(|j| j.o.id == o.id || j.chain.iter().any(|c| c.id == o.id));
    if rescue_flow_settled(g, o) || taken {
        g.rescue.flow.want.delete(o.id.clone());
        return;
    }
    if now.t - at > now.k.want_secs {
        g.rescue.flow.want.delete(o.id.clone());
        crate::rescue::rescue_note(g, &alloc::format!("flow:unmanned:{}", o.id));
        return;
    }
    let busy = g.rescue.flow.jobs.iter().find(|(_, j)| j.kind == JobKind::Cover).map(|(gd, _)| gd);
    if let Some(busy) = busy {
        g.rescue.flow.jobs.get_mut(busy).unwrap().chain.push(o.clone());
        g.rescue.flow.want.delete(o.id.clone());
        let note = alloc::format!("flow:queue:{}:{}", o.id, g.gd[busy].callsign);
        crate::rescue::rescue_note(g, &note);
        return;
    }
    let free = rescue_flow_free(g, &crate::seats::here(g));
    // the only free man: with another in the file who is in a fight or blind this moment it waits for him (want_secs) and the man on Sam stays on Sam
    if free.is_empty() || (free.len() == 1 && men_in_the_file(g) >= 2) {
        return;
    }
    rescue_flow_take(g, free[0], o, &[], false, false);
    g.rescue.flow.want.delete(o.id.clone());
}

/// Everybody alive falling in behind Sam, whatever he is doing this moment; not F's.
fn men_in_the_file(g: &Game) -> usize {
    let seat = crate::seats::here(g);
    let follows = |op: GuardId| crate::orders::squad_orders_of(&seat, op).is_some_and(|a| crate::orders::rows::is(g, a, crate::orders::Is::Follows));
    g.staged.team.iter().filter(|&&op| !crate::condition::down(g, g.gd[op].ch) && !crate::rescue::rescue_owns(g, op) && follows(op)).count()
}

/// Sam has turned off the corridor into a side room with both men on him: the last man holds the corridor at the opening he used, once.
fn hold_the_corridor(g: &mut Game, f: &crate::scene_scenarios::RescueFlowData, now: &FlowNow) {
    let t = now.t;
    let corridor_taken = g.rescue.flow.jobs.values().any(|j| j.kind == JobKind::Corridor);
    let c = match (&now.sam_in, &g.rescue.flow.came_in) {
        (Some(id), Some(c)) if c.region == *id && t - c.t < now.k.hold_secs && !corridor_taken => Some(c.o.clone()),
        _ => None,
    };
    let Some(o) = c else { return };
    let free = rescue_flow_free(g, &crate::seats::here(g));
    if free.len() < 2 {
        return;
    }
    g.rescue.flow.came_in.as_mut().unwrap().t = -1e9;
    let gd = free[0];
    let watch: V3 = [m::min(o.mark[0] + f.hold.ahead, f.hold.x_max), 1.1, f.hold.z];
    // (the floor he holds by the level's own word for it)
    let word = if !crate::dialogue::is_office(g) { crate::dialogue::place_in(g, o.mark).map(|p| p.name) } else { None };
    let hall = word.unwrap_or_else(|| String::from("the corridor"));
    crate::orders::squad_orders_new_group(g);
    crate::orders::squad_orders_assign(
        g,
        gd,
        &crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::cover::ORDER.id),
            post: Some(v3::copy(o.mark)),
            face: Some(v3::yaw_to(o.mark, watch)),
            watch: Some(watch),
            stance: Some(crate::character::Stance::None),
            upper: Some(crate::character::UpperMode::Aim),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            going: alloc::format!("holding {}", hall).into(),
            there: alloc::format!("holding {}", hall).into(),
            ..Default::default()
        },
    );
    let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd).unwrap();
    g.rescue.flow.mine.add(a);
    let id = o.id.clone();
    g.rescue.flow.jobs.set(gd, Job { kind: JobKind::Corridor, o, a, t0: t, on_t: f64::NAN, chain: Vec::new(), said_hot: false, said_wait: false, leap: false });
    crate::game::game_say(g, gd, &alloc::format!("{}.", hall.strip_prefix("the ").unwrap_or(&hall)), None, None);
    let note = alloc::format!("flow:corridor:{}:{}", id, g.gd[gd].callsign);
    crate::rescue::rescue_note(g, &note);
}

/// Nothing for a man to do at it: the room behind it is cleared, or a man Sam posted has it already.
fn rescue_flow_settled(g: &crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> bool {
    let Some(k) = g.rescue.flow.known.iter().find(|x| x.o.id == o.id) else { return false };
    if g.tab.rescueflow_region[k.region].cleared {
        return true;
    }
    match k.held {
        Some(h) => !g.rescue.flow.jobs.has(h),
        None => false,
    }
}

/// pairup.ts 'stay': one partner, silent, not found out: the flow sends nobody.
fn rescue_flow_stays(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    crate::pairup::pair_up(g) && crate::orders::squad_orders_squad(g, seat).len() == 1 && crate::silent::squad_silent(g, seat) && !crate::silent::silent_compromised(g, seat)
}

/// The men the flow may send: alive, falling in, not F's, not in a fight, not blind; the LAST in the file first.
fn rescue_flow_free(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    let mut out: Vec<(GuardId, f64)> = Vec::new();
    let mut i = 0;
    while i < g.staged.team.len() {
        let gd = g.staged.team[i];
        i += 1;
        if crate::condition::down(g, g.gd[gd].ch) || g.rescue.flow.jobs.has(gd) || crate::rescue::rescue_owns(g, gd) || crate::control::man::blind(g, gd) {
            continue;
        }
        let Some(a) = crate::orders::squad_orders_of(seat, gd) else { continue };
        if !crate::orders::rows::is(g, a, crate::orders::Is::Follows) {
            continue;
        }
        if fight_ai_has(g, gd) || crate::roomfight::room_fight_engaged(g, gd) {
            continue;
        }
        out.push((gd, g.tab.orders_assignment[a].follow));
    }
    js::sort_by(&mut out, |a, b| b.1 - a.1);
    out.into_iter().map(|x| x.0).collect()
}

/// Send a man to cover an opening.
fn rescue_flow_take(g: &mut crate::game::Game, gd: GuardId, o: &crate::scene_scenarios::RouteOpening, chain: &[crate::scene_scenarios::RouteOpening], said_wait: bool, leap: bool) {
    let door = door_of(g, o);
    crate::orders::squad_orders_new_group(g); // (his own: a 'set.' Sam's last order is waiting to call does not wait on this man's mark)
    crate::orders::squad_orders_assign(
        g,
        gd,
        &crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::cover::ORDER.id),
            post: Some(v3::copy(o.mark)),
            face: Some(v3::yaw_to(o.mark, o.watch)),
            watch: Some(v3::copy(o.watch)),
            door,
            stance: Some(crate::character::Stance::None),
            upper: Some(crate::character::UpperMode::Aim),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            going: alloc::format!("moving · {}", o.name).into(),
            there: alloc::format!("covering {}", o.name).into(),
            ..Default::default()
        },
    );
    let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd).unwrap();
    g.rescue.flow.mine.add(a);
    let t0 = g.time;
    g.rescue.flow.jobs.set(gd, Job { kind: JobKind::Cover, o: o.clone(), a, t0, on_t: f64::NAN, chain: chain.to_vec(), said_hot: false, said_wait, leap });
    g.rescue.flow.stats.takes += 1.0;
    crate::game::game_say(g, gd, &o.call, None, None);
    let note = alloc::format!("flow:take:{}:{}", o.id, g.gd[gd].callsign);
    crate::rescue::rescue_note(g, &note);
}

/// Back into the file behind Sam, at its tail.
fn rescue_flow_rejoin(g: &mut crate::game::Game, gd: GuardId, words: Option<&str>) {
    g.rescue.flow.jobs.delete(gd);
    crate::orders::squad_orders_new_group(g);
    crate::orders::squad_orders_assign(
        g,
        gd,
        &crate::orders::SquadOrdersAssignA {
            order: String::from(crate::orders::rows::fallin::ORDER.id),
            post: None,
            follow: Some(9.0),
            face: None,
            stance: Some(crate::character::Stance::LowReady),
            upper: Some(crate::character::UpperMode::None),
            going: "catching up".into(),
            there: "with you".into(),
            ..Default::default()
        },
    );
    if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), gd) {
        g.rescue.flow.mine.add(a);
    }
    if let Some(w) = words {
        crate::game::game_say(g, gd, w, None, None);
    }
}

fn rescue_flow_drop_all(g: &mut crate::game::Game, back: bool) {
    for gd in g.rescue.flow.jobs.keys() {
        let Some(j) = g.rescue.flow.jobs.delete(gd) else { continue };
        if back && !crate::condition::down(g, g.gd[gd].ch) && !crate::rescue::rescue_owns(g, gd) && crate::orders::squad_orders_of(&crate::seats::here(g), gd) == Some(j.a) {
            rescue_flow_rejoin(g, gd, None);
        }
    }
    g.rescue.flow.want.clear();
}

/// The strips: one flat glowing bar on the corridor floor across each opening, in its state's colour (a hot one beats).
fn rescue_flow_boxes(g: &mut crate::game::Game) {
    let flag = crate::scene_boxes::box_flag();
    let flags = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    let colours = colour();
    let glow = g.rescue.flow.knobs.glow;
    let time = g.time;
    for at in 0..g.rescue.flow.known.len() {
        let (state, oat, north, half) = {
            let k = &g.rescue.flow.known[at];
            (k.state, k.o.at, k.o.north, k.o.half)
        };
        let c = *colours.get(String::from(state.as_str())).unwrap();
        let beat = if state == OpenState::Hot { 0.65 + 0.35 * m::sin(time * 9.0) } else { 1.0 };
        let e = glow * beat;
        let box_ = crate::scene_boxes::make_box_kept(
            g,
            &crate::scene_boxes::MakeBoxP {
                c: [oat[0], 0.012, oat[2] + (if north { 0.36 } else { -0.3 })],
                h: [half, 0.008, 0.05],
                albedo: Some([0.03, 0.03, 0.03]),
                emissive: Some([c[0] * e, c[1] * e, c[2] * e]),
                flags: Some(flags),
                ..Default::default()
            },
        );
        g.fx.push(crate::game::GameFxItem { box_, ttl: 1e-3 });
    }
}

fn rescue_flow_word(g: &crate::game::Game, k: &crate::rescueflow::Known) -> String {
    let shut = k.o.kind == crate::scene_scenarios::RouteOpeningKind::Door && !rescue_flow_open(g, &k.o);
    let w = match k.state {
        OpenState::Hot => {
            if k.held.is_some() {
                "HOT, held"
            } else {
                "HOT"
            }
        }
        OpenState::Held => "held",
        OpenState::Cleared => "clear",
        OpenState::Looked => "looked",
        OpenState::Unknown => {
            if shut {
                "shut"
            } else {
                "unknown"
            }
        }
    };
    String::from(w)
}

/// The objective line's tail: the openings behind Sam and what is known of each, and any hot one still ahead of him.
pub(super) fn rescue_flow_line(g: &crate::game::Game) -> String {
    let mut behind: Vec<String> = Vec::new();
    let mut ahead: Vec<String> = Vec::new();
    for at in 0..g.rescue.flow.known.len() {
        let k = g.rescue.flow.known[at].clone();
        let passed = g.rescue.flow.passed.has(k.o.id.clone());
        if passed || k.state == OpenState::Hot {
            let row = alloc::format!("{} {}", k.o.short, rescue_flow_word(g, &k));
            if passed {
                behind.push(row);
            } else {
                ahead.push(row);
            }
        }
    }
    let mut parts: Vec<String> = Vec::new();
    if !behind.is_empty() {
        parts.push(alloc::format!("behind you ▸ {}", behind.join(" · ")));
    }
    if !ahead.is_empty() {
        parts.push(alloc::format!("ahead ▸ {}", ahead.join(" · ")));
    }
    parts.join(" · ")
}


/// The marks against the floor: returns what is wrong.
pub(super) fn rescue_flow_check(g: &mut crate::game::Game) -> Vec<String> {
    let mut bad: Vec<String> = Vec::new();
    if rescue_flow_f(g).is_none() {
        return bad;
    }
    if !g.rescue.flow.built {
        rescue_flow_build(g);
    }
    for at in 0..g.rescue.flow.known.len() {
        let k = g.rescue.flow.known[at].clone();
        let o = &k.o;
        if crate::scene_collision::nav_grid_is_blocked(g, o.mark[0], o.mark[2]) {
            bad.push(alloc::format!("{}: the cover mark is not on free floor", o.id));
        }
        let is_door = o.kind == crate::scene_scenarios::RouteOpeningKind::Door;
        let eye: V3 = [o.mark[0], EYE, o.mark[2]];
        let w = o.watch;
        let d = dist_xz(eye, w);
        let s = if is_door { m::max(0.0, d - 0.5) / d } else { 1.0 };
        if crate::scene_collision::segment_blocked(g, eye, [eye[0] + (w[0] - eye[0]) * s, w[1], eye[2] + (w[2] - eye[2]) * s], None, None, None) {
            bad.push(alloc::format!("{}: no line from the cover mark to its watch point", o.id));
        }
        if let Some(n) = o.door.as_deref().filter(|n| !n.is_empty()) {
            if crate::doors::doors_by_name(g, n).is_none() {
                bad.push(alloc::format!("{}: no door '{}' in the level", o.id, n));
            }
        }
        if !is_door && k.lookable.len() < 6 {
            bad.push(alloc::format!("{}: only {} floor samples can be seen through it", o.id, k.lookable.len()));
        }
        if g.tab.rescueflow_region[k.region].pts.is_empty() {
            bad.push(alloc::format!("{}: its room has no floor", o.id));
        }
    }
    bad
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opening(id: &str, short: &str, region: RescueflowRegionId) -> Known {
        let o = crate::scene_scenarios::RouteOpening { id: String::from(id), short: String::from(short), kind: crate::scene_scenarios::RouteOpeningKind::Doorway, ..Default::default() };
        Known { o, region, state: OpenState::Unknown, ..Default::default() }
    }

    /// A write through a Known's region reaches the row the table holds: the two are one Region.
    #[test]
    fn a_known_row_and_the_table_share_one_region() {
        let mut g = Game::blank();
        let r = g.tab.rescueflow_region.push(Region { id: String::from("a"), ..Default::default() });
        g.rescue.flow.regions.set(String::from("a"), r);
        g.rescue.flow.known.push(opening("o", "o", r));
        let o = g.rescue.flow.known[0].o.clone();
        assert!(!rescue_flow_settled(&mut g, &o));
        g.tab.rescueflow_region[r].cleared = true;
        assert!(rescue_flow_settled(&mut g, &o));
    }

    /// A man holding it counts only while he is not on one of the flow's own jobs.
    #[test]
    fn a_held_opening_is_settled_only_by_a_man_with_no_job() {
        let mut g = Game::blank();
        let r = g.tab.rescueflow_region.push(Region::default());
        let mut k = opening("o", "o", r);
        k.held = Some(GuardId(3));
        let o = k.o.clone();
        g.rescue.flow.known.push(k);
        assert!(rescue_flow_settled(&mut g, &o));
        g.rescue.flow.jobs.set(GuardId(3), Job::default());
        assert!(!rescue_flow_settled(&mut g, &o));
    }

    /// An event is heard once, by its serial, whatever else has the same fields.
    #[test]
    fn sense_hears_each_event_once_by_its_serial() {
        let mut g = Game::blank();
        let c = crate::character::man_new(&mut g, crate::character::Character::blank());
        g.cond[c].down = true;
        g.player = g.pl.push(crate::game::Player { ch: c, ..Default::default() });
        let mut e = crate::game::GameEvent::default();
        e.serial = 7;
        g.events.push(e);
        e.serial = 8;
        g.events.push(e);
        g.events.push(e);
        rescue_flow_sense(&mut g);
        assert_eq!(g.rescue.flow.heard.keys(), vec![7u32, 8u32]);
        assert_eq!(g.rescue.flow.turn, 1.0);
    }

    /// A row taken out of `known` for sense's calls comes back whole, with what sense worked out written on it.
    #[test]
    fn sense_puts_each_known_row_back_with_its_share() {
        let mut g = Game::blank();
        let c = crate::character::man_new(&mut g, crate::character::Character::blank());
        g.cond[c].down = true;
        g.player = g.pl.push(crate::game::Player { ch: c, ..Default::default() });
        let r = g.tab.rescueflow_region.push(Region { seen_t: vec![0.0, -1e9], ..Default::default() });
        let mut k = opening("o", "glass", r);
        k.built_open = true;
        k.lookable = vec![0.0, 1.0];
        g.rescue.flow.known.push(k);
        rescue_flow_sense(&mut g);
        let row = &g.rescue.flow.known[0];
        assert_eq!(row.o.id, "o");
        assert_eq!(row.region, r);
        assert_eq!(row.lookable.len(), 2);
        assert_eq!(row.share, 0.5);
        assert!(row.state == OpenState::Unknown);
    }

    /// The objective line tells what is known of the openings behind him, and of any hot one ahead of him; nothing yet known says nothing.
    #[test]
    fn the_line_says_what_is_known_behind_him_and_what_is_hot_ahead() {
        let mut g = Game::blank();
        assert_eq!(rescue_flow_line(&mut g), "");
        let r = g.tab.rescueflow_region.push(Region::default());
        let mut a = opening("a", "glass", r);
        a.state = OpenState::Cleared;
        let mut b = opening("b", "west", r);
        b.state = OpenState::Hot;
        b.held = Some(GuardId(0));
        let c = opening("c", "server", r);
        g.rescue.flow.known = vec![a, b, c];
        g.rescue.flow.passed.add(String::from("a"));
        assert_eq!(rescue_flow_line(&mut g), "behind you ▸ glass clear · ahead ▸ west HOT, held");
    }
}
