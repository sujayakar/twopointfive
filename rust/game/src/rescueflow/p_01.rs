//! The hostage rescue's dynamic tempo: what the pair do about the openings Sam runs past, and what is known of each. (rescueflow.ts)
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `inBounds(p, r)` of a scenario's rectangle.
fn in_rect(p: V3, r: &crate::scene_scenarios::Rect) -> bool {
    crate::squad::in_bounds(p, Some(crate::scene_level::BreachPlanBounds { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }), None)
}

/// `g.col.segmentBlocked(a, b)`, the sight line with the collision's own defaults.
fn blocked(g: &mut Game, a: V3, b: V3) -> bool {
    crate::scene_collision::segment_blocked(g, a, b, None, None, None)
}

/// The door of an opening, if it names one and the level has it.
fn door_of(g: &mut Game, o: &crate::scene_scenarios::RouteOpening) -> Option<DoorId> {
    match &o.door {
        Some(n) if !n.is_empty() => crate::doors::doors_by_name(g, n),
        _ => None,
    }
}

/// Mark a hostile as known in a region, noting a room newly hot. (rescueflow.ts:164, 170)
fn set_hot(g: &mut Game, r: RescueflowRegionId, h: GuardId, t: f64, at: V3) {
    if !g.tab.rescueflow_region[r].hot.has(h) {
        let note = alloc::format!("flow:hot:{}", g.tab.rescueflow_region[r].id);
        crate::rescue::rescue_note(g, &note);
    }
    g.tab.rescueflow_region[r].hot.set(h, RegionHotValue { t, at: v3::copy(at) });
}

/// (rescueflow.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// The level's route data; None = this level's rescue has no flow. (rescueflow.ts RescueFlow.F)
pub fn rescue_flow_f(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RescueFlowData> {
    crate::rescue::rescue_data(g).and_then(|d| d.flow)
}

/// The level's openings. (rescueflow.ts RescueFlow.route)
pub fn rescue_flow_route(g: &mut crate::game::Game) -> Vec<crate::scene_scenarios::RouteOpening> {
    match rescue_flow_f(g) {
        Some(f) => f.route,
        None => Vec::new(),
    }
}

/// (rescueflow.ts RescueFlow.constructor)
pub fn rescue_flow_new(g: &mut crate::game::Game) -> crate::rescueflow::RescueFlow {
    RescueFlow::blank()
}

/// A fresh staging: nothing known, nobody on a job. (rescueflow.ts RescueFlow.reset)
pub fn rescue_flow_reset(g: &mut crate::game::Game) {
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
    f.stats = RescueFlowStats::zeroed();
}

/// Is this assignment one the flow gave. (rescueflow.ts RescueFlow.gave)
pub fn rescue_flow_gave(g: &mut crate::game::Game, a: Option<OrdersAssignmentId>) -> bool {
    match a {
        Some(a) => g.rescue.flow.mine.has(a),
        None => false,
    }
}

/// F has taken these men: their jobs end where they stand. (rescueflow.ts RescueFlow.release)
pub fn rescue_flow_release(g: &mut crate::game::Game, men: &[GuardId]) {
    for &gd in men {
        g.rescue.flow.jobs.delete(gd);
    }
}

/// (rescueflow.ts RescueFlow.stateOf)
pub fn rescue_flow_state_of(g: &mut crate::game::Game, id_2: &str) -> Option<crate::rescueflow::OpenState> {
    g.rescue.flow.known.iter().find(|k| k.o.id == id_2).map(|k| k.state)
}

/// (rescueflow.ts RescueFlow.jobOf)
pub fn rescue_flow_job_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::rescueflow::RescueFlowJobOf> {
    let j = g.rescue.flow.jobs.get(gd)?;
    Some(RescueFlowJobOf {
        kind: match j.kind {
            JobKind::Cover => RescueFlowJobOfKind::Cover,
            JobKind::Corridor => RescueFlowJobOfKind::Corridor,
        },
        opening: j.o.id.clone(),
        queued: j.chain.iter().map(|o| o.id.clone()).collect(),
    })
}

/// Sample each region's floor and make a row per opening. (rescueflow.ts RescueFlow.build)
pub fn rescue_flow_build(g: &mut crate::game::Game) {
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

/// What of the room can be seen through this opening at all, from the cover mark and three places across its threshold. `k` is not in the table while this runs. (rescueflow.ts RescueFlow.lookable)
pub fn rescue_flow_lookable(g: &mut crate::game::Game, k: &mut crate::rescueflow::Known) {
    let (mark, at) = (k.o.mark, k.o.at);
    let dz = if k.o.north { 0.7 } else { -0.7 };
    let eyes: [V3; 4] = [[mark[0], EYE, mark[2]], [at[0], EYE, at[2] + dz], [at[0] - 0.9, EYE, at[2] + dz], [at[0] + 0.9, EYE, at[2] + dz]];
    k.lookable = Vec::new();
    k.built_open = rescue_flow_open(g, &k.o);
    let eye_m = g.rescue.flow.knobs.eye_m;
    let pts = g.tab.rescueflow_region[k.region].pts.clone();
    for (i, p) in pts.iter().enumerate() {
        for e in eyes.iter() {
            if flat(*e, *p) < eye_m && !blocked(g, *e, *p) {
                k.lookable.push(i as f64);
                break;
            }
        }
    }
}

/// (rescueflow.ts RescueFlow.open)
pub fn rescue_flow_open(g: &mut crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> bool {
    match door_of(g, o) {
        None => true,
        Some(d) => m::abs(crate::doors::door_angle(g, d)) > 0.5,
    }
}

/// The team's eyes this moment: Sam's, and every living operative's who is not blind. (rescueflow.ts RescueFlow.eyes)
pub fn rescue_flow_eyes(g: &mut crate::game::Game) -> Vec<V3> {
    let mut out: Vec<V3> = Vec::new();
    let pl = g.player;
    if !g.pl[pl].down {
        let p = g.ch[g.pl[pl].ch].pos;
        out.push([p[0], EYE, p[2]]);
    }
    for &op in g.rescue.ops.iter() {
        if g.gd[op].state != crate::game::GuardState::Dead && g.time >= g.gd[op].dazzled_until {
            let p = g.ch[g.gd[op].ch].pos;
            out.push([p[0], EYE, p[2]]);
        }
    }
    out
}

/// Is this man of theirs seen by the team: the room fight's answer, else the fog's while it is up, else a plain line from any eye to his chest. (rescueflow.ts RescueFlow.sees)
pub fn rescue_flow_sees(g: &mut crate::game::Game, eyes: &[V3], h: GuardId) -> bool {
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
        if flat(e, c) < 30.0 && !blocked(g, e, c) {
            return true;
        }
    }
    false
}

/// (rescueflow.ts RescueFlow.regionAt)
pub fn rescue_flow_region_at(g: &mut crate::game::Game, p: V3) -> Option<RescueflowRegionId> {
    g.rescue.flow.regions.values().copied().find(|&r| in_rect(p, &g.tab.rescueflow_region[r].rect))
}

/// A few times a second: the floor seen, who of theirs is known where, what each room and opening reads. (rescueflow.ts RescueFlow.sense)
pub fn rescue_flow_sense(g: &mut crate::game::Game) {
    let t = g.time;
    let k = g.rescue.flow.knobs;
    let eyes = rescue_flow_eyes(g);
    let all: Vec<RescueflowRegionId> = g.rescue.flow.regions.values().copied().collect();
    {
        // the floor the eyes are on: ONE room a call, in turn
        let turn = g.rescue.flow.turn;
        g.rescue.flow.turn = turn + 1.0;
        if !all.is_empty() {
            let r = all[js::rem(turn, all.len() as f64) as usize];
            let rect = g.tab.rescueflow_region[r].rect;
            let n = g.tab.rescueflow_region[r].pts.len();
            for &e in eyes.iter() {
                if e[0] < rect.x0 - k.eye_m || e[0] > rect.x1 + k.eye_m || e[2] < rect.z0 - k.eye_m || e[2] > rect.z1 + k.eye_m {
                    continue;
                }
                for i in 0..n {
                    if t - g.tab.rescueflow_region[r].seen_t[i] < 0.5 {
                        continue;
                    }
                    let p = g.tab.rescueflow_region[r].pts[i];
                    if flat(e, p) < k.eye_m && !blocked(g, e, p) {
                        g.tab.rescueflow_region[r].seen_t[i] = t;
                    }
                }
            }
        }
    }
    for &r in all.iter() {
        let rect = g.tab.rescueflow_region[r].rect;
        for &e in eyes.iter() {
            if in_rect(e, &rect) {
                g.tab.rescueflow_region[r].was_in = t;
            }
        }
    }
    // who of theirs is known to be where: seen in a room marks it hot; seen anywhere else, or dead, lets it go
    let mut i = 0;
    while i < g.rescue.hostiles.len() {
        let h = g.rescue.hostiles[i];
        i += 1;
        if g.gd[h].state == crate::game::GuardState::Dead {
            for &r in all.iter() {
                g.tab.rescueflow_region[r].hot.delete(h);
            }
            continue;
        }
        if !rescue_flow_sees(g, &eyes, h) {
            continue;
        }
        let pos = g.ch[g.gd[h].ch].pos;
        let at = rescue_flow_region_at(g, pos);
        for &r in all.iter() {
            if at == Some(r) {
                set_hot(g, r, h, t, pos);
            } else {
                g.tab.rescueflow_region[r].hot.delete(h);
            }
        }
    }
    // a shot of theirs from inside a room says as much as a sight of him
    let mut i = 0;
    while i < g.events.len() {
        let at = i;
        i += 1;
        let key = g.events[at].serial;
        if g.rescue.flow.heard.has(key) {
            continue;
        }
        g.rescue.flow.heard.add(key);
        if g.events[at].kind != crate::director::EventKind::GuardShot {
            continue;
        }
        let (who, pos) = (g.events[at].who, g.events[at].pos);
        let h = g.rescue.hostiles.iter().copied().find(|&x| Some(g.ch[g.gd[x].ch].id) == who);
        let Some(h) = h else { continue };
        if g.gd[h].state == crate::game::GuardState::Dead {
            continue;
        }
        if let Some(r) = rescue_flow_region_at(g, pos) {
            set_hot(g, r, h, t, pos);
        }
    }
    // a hot room goes cold: nothing of him for a long while, or eyes on the spot he was last at, the room looked through since, and no sign of him
    for &r in all.iter() {
        let hot: Vec<(GuardId, RegionHotValue)> = g.tab.rescueflow_region[r].hot.iter().map(|(h, e)| (h, *e)).collect();
        for (h, e) in hot {
            if t - e.t < 1.0 {
                continue;
            }
            let mut cold = t - e.t > k.hot_secs;
            let row = &g.tab.rescueflow_region[r];
            if !cold && !row.pts.is_empty() {
                let mut n = 0.0;
                for i in 0..row.pts.len() {
                    if row.seen_t[i] > e.t + 0.5 {
                        n += 1.0;
                    }
                }
                let share = n / row.pts.len() as f64;
                let spot: V3 = [e.at[0], 1.1, e.at[2]];
                if share >= k.hot_empty {
                    for &eye in eyes.iter() {
                        if flat(eye, spot) < k.eye_m && !blocked(g, eye, spot) {
                            cold = true;
                            break;
                        }
                    }
                }
            }
            if cold {
                g.tab.rescueflow_region[r].hot.delete(h);
                let note = alloc::format!("flow:cold:{}", g.tab.rescueflow_region[r].id);
                crate::rescue::rescue_note(g, &note);
            }
        }
    }
    for &r in all.iter() {
        let row = &mut g.tab.rescueflow_region[r];
        let mut n = 0.0;
        for i in 0..row.pts.len() {
            if t - row.seen_t[i] < k.clear_secs {
                n += 1.0;
            }
        }
        let was = row.cleared;
        row.cleared = row.hot.is_empty() && (row.cleared || (t - row.was_in < 1.0 && !row.pts.is_empty() && n / row.pts.len() as f64 >= k.clear_share));
        if row.cleared && !was {
            let note = alloc::format!("flow:cleared:{}", row.id);
            crate::rescue::rescue_note(g, &note);
        }
    }
    let mut i = 0;
    while i < g.rescue.flow.known.len() {
        let at = i;
        i += 1;
        let o = g.rescue.flow.known[at].o.clone();
        if g.rescue.flow.known[at].built_open != rescue_flow_open(g, &o) {
            let mut row = core::mem::take(&mut g.rescue.flow.known[at]);
            rescue_flow_lookable(g, &mut row);
            g.rescue.flow.known[at] = row;
        }
        let row = &g.rescue.flow.known[at];
        let r = &g.tab.rescueflow_region[row.region];
        let (hot, cleared) = (!r.hot.is_empty(), r.cleared);
        let mut n = 0.0;
        for &ix in row.lookable.iter() {
            if t - r.seen_t[ix as usize] < k.look_secs {
                n += 1.0;
            }
        }
        let share = if !row.lookable.is_empty() { n / row.lookable.len() as f64 } else { 0.0 };
        let held = rescue_flow_holder(g, &o);
        let row = &mut g.rescue.flow.known[at];
        row.share = share;
        row.hot = hot;
        row.held = held;
        let was = row.state;
        row.state = if hot {
            OpenState::Hot
        } else if held.is_some() {
            OpenState::Held
        } else if cleared {
            OpenState::Cleared
        } else if share >= k.look_share {
            OpenState::Looked
        } else {
            OpenState::Unknown
        };
        let state = row.state;
        if state != was {
            crate::rescue::rescue_note(g, &alloc::format!("flow:state:{}:{}", o.id, state.as_str()));
            if state == OpenState::Hot {
                crate::game::game_msg(g, &alloc::format!("contact — {}", o.name));
            }
        }
    }
}

/// The man posted on this opening: on a mark of his own with his gun on it, the flow's cover or a menu order of Sam's. (rescueflow.ts RescueFlow.holder)
pub fn rescue_flow_holder(g: &mut crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> Option<GuardId> {
    let mut i = 0;
    while i < g.rescue.ops.len() {
        let op = g.rescue.ops[i];
        i += 1;
        if g.gd[op].state == crate::game::GuardState::Dead {
            continue;
        }
        let Some(a) = crate::orders::squad_orders_of(g, op) else { continue };
        let Some(post) = g.tab.orders_assignment[a].post else { continue };
        if flat(g.ch[g.gd[op].ch].pos, post) > 1.2 {
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
            Some(w) => flat(post, o.at) < 6.0 && (flat(w, o.at) < 1.5 || flat(w, o.watch) < 1.5),
            None => false,
        };
        if on_door || on_watch {
            return Some(op);
        }
    }
    None
}

/// Once a frame from Rescue.tick. (rescueflow.ts RescueFlow.tick)
pub fn rescue_flow_tick(g: &mut crate::game::Game) {
    let k = g.rescue.flow.knobs;
    let f = rescue_flow_f(g);
    let f = match f {
        Some(f) if g.rescue.active => f,
        _ => {
            if !g.rescue.flow.jobs.is_empty() {
                rescue_flow_drop_all(g, true);
            }
            return;
        }
    };
    let route = &f.route;
    if !g.rescue.flow.built {
        rescue_flow_build(g);
    }
    if g.time >= g.rescue.flow.sense_at {
        g.rescue.flow.sense_at = g.time + 1.0 / (k.sense_hz * m::max(1.0, g.rescue.flow.regions.len() as f64));
        rescue_flow_sense(g);
    }
    rescue_flow_boxes(g);
    let pl = g.player;
    let sam = g.ch[g.pl[pl].ch].pos;
    let t = g.time;
    let dynamic = crate::rescue::rescue_dynamic(g);
    let live = dynamic && g.rescue.result.is_none() && !g.pl[pl].down;
    let room = g.rescue.flow.room;
    let in_corr = in_rect(sam, &f.corridor);
    let on_route = in_corr || in_rect(sam, &room);
    let sam_in: Option<String> = if on_route { None } else { rescue_flow_region_at(g, sam).map(|r| g.tab.rescueflow_region[r].id.clone()) };
    // what Sam has come past: on the corridor short of it a moment ago, on the corridor abreast of it now
    if in_corr {
        let from = match g.rescue.flow.corr {
            Some(c) if t - c.t < 0.5 => Some(c.x),
            _ => None,
        };
        for o in route.iter() {
            let has = g.rescue.flow.passed.has(o.id.clone());
            let line = o.at[0] - k.abreast;
            if !has && from.is_some_and(|x| x <= line) && sam[0] > line {
                g.rescue.flow.passed.add(o.id.clone());
                crate::rescue::rescue_note(g, &alloc::format!("flow:pass:{}", o.id));
                if live {
                    g.rescue.flow.want.set(o.id.clone(), t);
                }
            } else if has && sam[0] < o.at[0] - k.unpass {
                g.rescue.flow.passed.delete(o.id.clone());
                g.rescue.flow.want.delete(o.id.clone());
            }
        }
        g.rescue.flow.corr = Some(RescueFlowCorr { x: sam[0], t });
        g.rescue.flow.came_in = None;
    } else if let Some(id) = &sam_in {
        // off the corridor into a side room: by the opening of that room nearest him as he went in
        if g.rescue.flow.came_in.as_ref().map(|c| &c.region) != Some(id) {
            let mut nearest: Option<&crate::scene_scenarios::RouteOpening> = None;
            for x in route.iter().filter(|x| x.region == *id) {
                nearest = match nearest {
                    Some(a) if flat(a.at, sam) <= flat(x.at, sam) => Some(a),
                    _ => Some(x),
                };
            }
            let recent = matches!(g.rescue.flow.corr, Some(c) if t - c.t < 0.5);
            g.rescue.flow.came_in = match nearest {
                Some(o) if recent => Some(RescueFlowCameIn { region: id.clone(), o: o.clone(), t }),
                _ => None,
            };
        }
    } else if on_route {
        g.rescue.flow.came_in = None;
    }
    if !dynamic {
        if !g.rescue.flow.jobs.is_empty() {
            rescue_flow_drop_all(g, true);
        }
        return;
    }
    if g.rescue.result.is_some() || g.pl[pl].down {
        // (it is over: whoever was covering stays covering)
        g.rescue.flow.jobs.clear();
        g.rescue.flow.want.clear();
        return;
    }
    // the men on a job
    for gd in g.rescue.flow.jobs.keys() {
        // (a local copy of the row: what a later read needs is written to both; a job only ever ends or changes on its own man's turn)
        let Some(mut j) = g.rescue.flow.jobs.get(gd).cloned() else { continue };
        let a = crate::orders::squad_orders_of(g, gd);
        if g.gd[gd].state == crate::game::GuardState::Dead || crate::rescue::rescue_owns(g, gd) {
            // (down on the job: what was queued behind him is wanted again)
            g.rescue.flow.jobs.delete(gd);
            if g.gd[gd].state == crate::game::GuardState::Dead {
                for o in j.chain.iter() {
                    g.rescue.flow.want.set(o.id.clone(), t);
                }
            }
            continue;
        }
        if a != Some(j.a) {
            // Sam's own order has him: the flow lets go, and does not send the next man to the same openings
            g.rescue.flow.jobs.delete(gd);
            g.rescue.flow.stats.overridden += 1.0;
            let note = alloc::format!("flow:override:{}", g.gd[gd].callsign);
            crate::rescue::rescue_note(g, &note);
            continue;
        }
        if g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert {
            continue; // (the fight AI has him: his fight first)
        }
        let o = j.o.clone();
        let callsign = g.gd[gd].callsign.clone();
        if j.kind == JobKind::Corridor {
            if on_route || sam_in.is_none() {
                rescue_flow_rejoin(g, gd, Some("on you."));
                crate::rescue::rescue_note(g, &alloc::format!("flow:done:corridor:{}", callsign));
            }
            continue;
        }
        let pos = g.ch[g.gd[gd].ch].pos;
        if j.on_t.is_nan() && (g.tab.orders_assignment[j.a].arrived || flat(pos, o.mark) < 0.6) {
            j.on_t = t;
            g.rescue.flow.jobs.get_mut(gd).unwrap().on_t = t;
        }
        let known = g.rescue.flow.known.iter().find(|x| x.o.id == o.id).unwrap();
        let (k_hot, k_region) = (known.hot, known.region);
        if k_hot {
            // one of theirs is known in it: it is held for as long as he has lately been seen or heard there
            let times: Vec<f64> = g.tab.rescueflow_region[k_region].hot.values().map(|e| e.t).collect();
            let last = m::max_n(&times);
            if t - last < k.hot_hold {
                if !j.said_hot {
                    j.said_hot = true;
                    g.rescue.flow.jobs.get_mut(gd).unwrap().said_hot = true;
                    g.rescue.flow.stats.hot_holds += 1.0;
                    crate::game::game_say(g, gd, &alloc::format!("contact {} — holding.", o.side.as_str()), None, None, None);
                    crate::rescue::rescue_note(g, &alloc::format!("flow:hold:{}:{}", o.id, callsign));
                }
                continue;
            }
        }
        if crate::roomfight::room_fight_engaged(g, gd) {
            continue; // (the man he was holding on has just gone from his sight: not released on that frame)
        }
        let sam_past = !on_route || sam[0] > o.at[0] + k.past;
        let done = if j.on_t.is_nan() { t - j.t0 > k.give_up } else { t - j.on_t >= o.look && (sam_past || t - j.on_t >= k.max_cover) };
        if !j.said_wait && !j.chain.is_empty() && on_route && sam[0] - pos[0] > k.wait_m {
            j.said_wait = true;
            g.rescue.flow.jobs.get_mut(gd).unwrap().said_wait = true;
            crate::game::game_say(g, gd, "wait one.", None, None, None);
            crate::rescue::rescue_note(g, &alloc::format!("flow:wait:{}", callsign));
        }
        if !done {
            continue;
        }
        crate::rescue::rescue_note(g, &alloc::format!("flow:done:{}:{}", o.id, callsign));
        // (the chain is shifted on the copy: from here this job is replaced by take or ended by rejoin, so the row's own list is never read again)
        let mut next = if j.chain.is_empty() { None } else { Some(j.chain.remove(0)) };
        while let Some(n) = &next {
            if !rescue_flow_settled(g, n) {
                break;
            }
            next = if j.chain.is_empty() { None } else { Some(j.chain.remove(0)) };
        }
        if let Some(n) = &next {
            if rescue_flow_stays(g) {
                g.stats.pairup.stays += 1;
                crate::rescue::rescue_note(g, &alloc::format!("flow:stay:{}", n.id));
                next = None;
            }
        }
        if let Some(n) = &next {
            rescue_flow_take(g, gd, n, &j.chain, j.said_wait, true);
            g.rescue.flow.stats.chained += 1.0;
        } else {
            let words = if j.said_hot {
                Some("lost him — moving.")
            } else if j.leap {
                Some("moving.")
            } else {
                None
            };
            rescue_flow_rejoin(g, gd, words);
        }
    }
    if crate::rescue::rescue_mode(g) == crate::rescue::RescueMode::Door {
        // F has the floor: whoever it left out stays where Sam put him
        g.rescue.flow.want.clear();
        return;
    }
    if !g.rescue.flow.want.is_empty() && rescue_flow_stays(g) {
        for id in g.rescue.flow.want.keys() {
            g.stats.pairup.stays += 1;
            crate::rescue::rescue_note(g, &alloc::format!("flow:stay:{}", id));
        }
        g.rescue.flow.want.clear();
    }
    // an opening just passed: the man already working the openings takes it next; else the last free man in the file
    for o in route.iter() {
        let Some(&at) = g.rescue.flow.want.get(o.id.clone()) else { continue };
        let taken = g.rescue.flow.jobs.values().any(|j| j.o.id == o.id || j.chain.iter().any(|c| c.id == o.id));
        if rescue_flow_settled(g, o) || taken {
            g.rescue.flow.want.delete(o.id.clone());
            continue;
        }
        if t - at > k.want_secs {
            g.rescue.flow.want.delete(o.id.clone());
            g.rescue.flow.stats.unmanned += 1.0;
            crate::rescue::rescue_note(g, &alloc::format!("flow:unmanned:{}", o.id));
            continue;
        }
        let busy = g.rescue.flow.jobs.iter().find(|(_, j)| j.kind == JobKind::Cover).map(|(gd, _)| gd);
        if let Some(busy) = busy {
            g.rescue.flow.jobs.get_mut(busy).unwrap().chain.push(o.clone());
            g.rescue.flow.want.delete(o.id.clone());
            let note = alloc::format!("flow:queue:{}:{}", o.id, g.gd[busy].callsign);
            crate::rescue::rescue_note(g, &note);
            continue;
        }
        let free = rescue_flow_free(g);
        if free.is_empty() {
            continue;
        }
        if free.len() == 1 {
            // the only free man: with another in the file who is in a fight or blind this moment it waits for him (wantSecs) and the man on Sam stays on Sam
            let mut file = 0;
            let mut i = 0;
            while i < g.rescue.ops.len() {
                let op = g.rescue.ops[i];
                i += 1;
                if g.gd[op].state != crate::game::GuardState::Dead && !crate::rescue::rescue_owns(g, op) {
                    if let Some(a) = crate::orders::squad_orders_of(g, op) {
                        if g.tab.orders_assignment[a].order == "fallIn" {
                            file += 1;
                        }
                    }
                }
            }
            if file >= 2 {
                continue;
            }
        }
        rescue_flow_take(g, free[0], o, &[], false, false);
        g.rescue.flow.want.delete(o.id.clone());
    }
    // Sam has turned off the corridor into a side room with both men on him: the last man holds the corridor at the opening he used, once
    let corridor_taken = g.rescue.flow.jobs.values().any(|j| j.kind == JobKind::Corridor);
    let c = match (&sam_in, &g.rescue.flow.came_in) {
        (Some(id), Some(c)) if c.region == *id && t - c.t < k.hold_secs && !corridor_taken => Some(c.o.clone()),
        _ => None,
    };
    if let Some(o) = c {
        let free = rescue_flow_free(g);
        if free.len() >= 2 {
            g.rescue.flow.came_in.as_mut().unwrap().t = -1e9;
            let gd = free[0];
            let watch: V3 = [m::min(o.mark[0] + f.hold.ahead, f.hold.x_max), 1.1, f.hold.z];
            // (the floor he holds by the level's own word for it)
            let word = if !crate::dialogue::is_office(g) { crate::dialogue::place_in(g, o.mark).map(|p| p.name) } else { None };
            let hall = match word {
                Some(w) => w,
                None => String::from("the corridor"),
            };
            crate::orders::squad_orders_new_group(g);
            crate::orders::squad_orders_assign(
                g,
                gd,
                &crate::orders::SquadOrdersAssignA {
                    order: String::from("cover"),
                    post: Some(v3::copy(o.mark)),
                    face: Some(v3::yaw_to(o.mark, watch)),
                    watch: Some(watch),
                    stance: Some(crate::character::Stance::None),
                    upper: Some(crate::character::UpperMode::Aim),
                    move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
                    going: alloc::format!("holding {}", hall),
                    there: alloc::format!("holding {}", hall),
                    ..Default::default()
                },
            );
            let a = crate::orders::squad_orders_of(g, gd).unwrap();
            g.rescue.flow.mine.add(a);
            let id = o.id.clone();
            g.rescue.flow.jobs.set(gd, Job { kind: JobKind::Corridor, o, a, t0: t, on_t: f64::NAN, chain: Vec::new(), said_hot: false, said_wait: false, leap: false });
            g.rescue.flow.stats.corridor += 1.0;
            crate::game::game_say(g, gd, &alloc::format!("{}.", hall.strip_prefix("the ").unwrap_or(&hall)), None, None, None);
            let note = alloc::format!("flow:corridor:{}:{}", id, g.gd[gd].callsign);
            crate::rescue::rescue_note(g, &note);
        }
    }
}

/// Nothing for a man to do at it: the room behind it is cleared, or a man Sam posted has it already. (rescueflow.ts RescueFlow.settled)
pub fn rescue_flow_settled(g: &mut crate::game::Game, o: &crate::scene_scenarios::RouteOpening) -> bool {
    let Some(k) = g.rescue.flow.known.iter().find(|x| x.o.id == o.id) else { return false };
    if g.tab.rescueflow_region[k.region].cleared {
        return true;
    }
    match k.held {
        Some(h) => !g.rescue.flow.jobs.has(h),
        None => false,
    }
}

/// pairup.ts 'stay': one partner, silent, not found out: the flow sends nobody. (rescueflow.ts RescueFlow.stays)
pub fn rescue_flow_stays(g: &mut crate::game::Game) -> bool {
    crate::pairup::pair_up(g, crate::pairup::PairUpRule::Stay) && crate::orders::squad_orders_squad(g).len() == 1 && crate::silent::squad_silent(g) && !crate::silent::silent_compromised(g)
}

/// The men the flow may send: alive, falling in, not F's, not in a fight, not blind; the LAST in the file first. (rescueflow.ts RescueFlow.free)
pub fn rescue_flow_free(g: &mut crate::game::Game) -> Vec<GuardId> {
    let mut out: Vec<(GuardId, f64)> = Vec::new();
    let mut i = 0;
    while i < g.rescue.ops.len() {
        let gd = g.rescue.ops[i];
        i += 1;
        if g.gd[gd].state == crate::game::GuardState::Dead || g.rescue.flow.jobs.has(gd) || crate::rescue::rescue_owns(g, gd) || g.time < g.gd[gd].dazzled_until {
            continue;
        }
        let Some(a) = crate::orders::squad_orders_of(g, gd) else { continue };
        if g.tab.orders_assignment[a].order != "fallIn" {
            continue;
        }
        if (g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert) || crate::roomfight::room_fight_engaged(g, gd) {
            continue;
        }
        out.push((gd, g.tab.orders_assignment[a].follow));
    }
    js::sort_by(&mut out, |a, b| b.1 - a.1);
    out.into_iter().map(|x| x.0).collect()
}

/// Send a man to cover an opening. (rescueflow.ts RescueFlow.take)
pub fn rescue_flow_take(g: &mut crate::game::Game, gd: GuardId, o: &crate::scene_scenarios::RouteOpening, chain: &[crate::scene_scenarios::RouteOpening], said_wait: bool, leap: bool) {
    let door = door_of(g, o);
    crate::orders::squad_orders_new_group(g); // (his own: a 'set.' Sam's last order is waiting to call does not wait on this man's mark)
    crate::orders::squad_orders_assign(
        g,
        gd,
        &crate::orders::SquadOrdersAssignA {
            order: String::from("cover"),
            post: Some(v3::copy(o.mark)),
            face: Some(v3::yaw_to(o.mark, o.watch)),
            watch: Some(v3::copy(o.watch)),
            door,
            stance: Some(crate::character::Stance::None),
            upper: Some(crate::character::UpperMode::Aim),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            going: alloc::format!("moving · {}", o.name),
            there: alloc::format!("covering {}", o.name),
            ..Default::default()
        },
    );
    let a = crate::orders::squad_orders_of(g, gd).unwrap();
    g.rescue.flow.mine.add(a);
    let t0 = g.time;
    g.rescue.flow.jobs.set(gd, Job { kind: JobKind::Cover, o: o.clone(), a, t0, on_t: f64::NAN, chain: chain.to_vec(), said_hot: false, said_wait, leap });
    g.rescue.flow.stats.takes += 1.0;
    crate::game::game_say(g, gd, &o.call, None, None, None);
    let note = alloc::format!("flow:take:{}:{}", o.id, g.gd[gd].callsign);
    crate::rescue::rescue_note(g, &note);
}

/// Back into the file behind Sam, at its tail. (rescueflow.ts RescueFlow.rejoin)
pub fn rescue_flow_rejoin(g: &mut crate::game::Game, gd: GuardId, words: Option<&str>) {
    g.rescue.flow.jobs.delete(gd);
    crate::orders::squad_orders_new_group(g);
    crate::orders::squad_orders_assign(
        g,
        gd,
        &crate::orders::SquadOrdersAssignA {
            order: String::from("fallIn"),
            post: None,
            follow: Some(9.0),
            face: None,
            stance: Some(crate::character::Stance::LowReady),
            upper: Some(crate::character::UpperMode::None),
            going: String::from("catching up"),
            there: String::from("with you"),
            ..Default::default()
        },
    );
    if let Some(a) = crate::orders::squad_orders_of(g, gd) {
        g.rescue.flow.mine.add(a);
    }
    if let Some(w) = words {
        crate::game::game_say(g, gd, w, None, None, None);
    }
}

/// (rescueflow.ts RescueFlow.dropAll)
pub fn rescue_flow_drop_all(g: &mut crate::game::Game, back: bool) {
    for gd in g.rescue.flow.jobs.keys() {
        let Some(j) = g.rescue.flow.jobs.delete(gd) else { continue };
        if back && g.gd[gd].state != crate::game::GuardState::Dead && !crate::rescue::rescue_owns(g, gd) && crate::orders::squad_orders_of(g, gd) == Some(j.a) {
            rescue_flow_rejoin(g, gd, None);
        }
    }
    g.rescue.flow.want.clear();
}

/// The strips: one flat glowing bar on the corridor floor across each opening, in its state's colour (a hot one beats). (rescueflow.ts RescueFlow.boxes)
pub fn rescue_flow_boxes(g: &mut crate::game::Game) {
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
        let box_ = crate::scene_boxes::make_box(
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

/// (rescueflow.ts RescueFlow.word)
pub fn rescue_flow_word(g: &mut crate::game::Game, k: &crate::rescueflow::Known) -> String {
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

/// The objective line's tail: the openings behind Sam and what is known of each, and any hot one still ahead of him. (rescueflow.ts RescueFlow.line)
pub fn rescue_flow_line(g: &mut crate::game::Game) -> String {
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

/// (rescueflow.ts RescueFlow.summary)
pub fn rescue_flow_summary(g: &mut crate::game::Game) -> String {
    if g.rescue.flow.known.is_empty() {
        return String::from("flow: not staged");
    }
    let mut jobs: Vec<String> = Vec::new();
    for (gd, j) in g.rescue.flow.jobs.iter() {
        let what = if j.kind == JobKind::Corridor { "corridor" } else { j.o.short.as_str() };
        let more = if !j.chain.is_empty() { alloc::format!(" +{}", j.chain.len()) } else { String::new() };
        jobs.push(alloc::format!("{} {}{}", g.gd[gd].callsign, what, more));
    }
    let jobs = jobs.join(", ");
    let mut rows: Vec<String> = Vec::new();
    for at in 0..g.rescue.flow.known.len() {
        let k = g.rescue.flow.known[at].clone();
        let word = rescue_flow_word(g, &k);
        let tail = if g.rescue.flow.passed.has(k.o.id.clone()) { "" } else { " (ahead)" };
        rows.push(alloc::format!("{} {}{}", k.o.short, word, tail));
    }
    let on_it = if !jobs.is_empty() { alloc::format!(" · on it: {}", jobs) } else { String::new() };
    let s = g.rescue.flow.stats;
    alloc::format!("openings {}{} · {} covers, {} corridor holds, {} overridden", rows.join(" · "), on_it, s.takes, s.corridor, s.overridden)
}

/// The marks against the floor: returns what is wrong. (rescueflow.ts RescueFlow.check)
pub fn rescue_flow_check(g: &mut crate::game::Game) -> Vec<String> {
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
        let d = flat(eye, w);
        let s = if is_door { m::max(0.0, d - 0.5) / d } else { 1.0 };
        if blocked(g, eye, [eye[0] + (w[0] - eye[0]) * s, w[1], eye[2] + (w[2] - eye[2]) * s]) {
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

    /// A write through a Known's region reaches the row the table holds: the two are one Region, as in the TypeScript.
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

    /// A man holding it counts only while he is not on one of the flow's own jobs. (rescueflow.ts:303)
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

    /// An event is heard once, by its serial, whatever else has the same fields. (rescueflow.ts:167)
    #[test]
    fn sense_hears_each_event_once_by_its_serial() {
        let mut g = Game::blank();
        g.player = g.pl.push(crate::game::Player { down: true, ..Default::default() });
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

    /// The objective line and the summary, against the TypeScript's templates. (rescueflow.ts:358, 363)
    #[test]
    fn the_line_and_the_summary_say_what_is_known() {
        let mut g = Game::blank();
        assert_eq!(rescue_flow_summary(&mut g), "flow: not staged");
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
        g.rescue.flow.stats.takes = 3.0;
        g.rescue.flow.stats.overridden = 1.0;
        assert_eq!(rescue_flow_summary(&mut g), "openings glass clear · west HOT, held (ahead) · server unknown (ahead) · 3 covers, 0 corridor holds, 1 overridden");
    }
}
