//! src/game/workthings.ts lines 1 to 407
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ seams: what skeleton 4 cannot say yet

/// `g.audio.play(name, at, volume, { rate })`: `opts` of game_audio_like_play holds `crate::Opaque`, which cannot carry the rate. (workthings.ts:304)
fn seam_audio_play_rate(g: &mut Game, name: &str, at: Option<V3>, volume: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

// ------------------------------------------------------------ small local helpers

/// a man's feet
fn pos(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
/// the mains' two flags, `bo.active` and `bo.permanent`
fn mains(g: &mut Game) -> (bool, bool) {
    let active = crate::sim_lightshome::mains_active(g);
    let permanent = crate::sim_lightshome::mains_permanent(g);
    (active, permanent)
}
/// `t.at`, which every arm of the union has
fn target_at(t: &crate::orders::OrderTarget) -> V3 {
    match t {
        crate::orders::OrderTarget::Floor(x) => x.at,
        crate::orders::OrderTarget::Door(x) => x.at,
        crate::orders::OrderTarget::None(x) => x.at,
        crate::orders::OrderTarget::Foe(x) => x.at,
    }
}
/// `v.join(',')`, only ever compared with another such key
fn join_v3(p: V3) -> String { format!("{},{},{}", p[0], p[1], p[2]) }
/// J is one object in TypeScript: what a body wrote on its copy goes to the row, if there still is one
fn put_job(g: &mut Game, gd: GuardId, j: &Job) {
    if let Some(r) = g.work.jobs.get_mut(gd) { r.t0 = j.t0; r.last_t = j.last_t; }
}
/// a floor target standing on the thing; a new object each asking, so serial 0: orders.ts:937 stamps the one a menu keeps
fn thing_target(th: WorkThing) -> crate::orders::OrderTargetFloor {
    crate::orders::OrderTargetFloor { at: th.stand.unwrap_or([th.face[0], 0.0, th.face[2]]), wall: None, thing: Some(th), serial: 0 }
}

// ------------------------------------------------------------ the functions

/// Planar distance. (workthings.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// Where he stands to work a thing on a face: the free nav cell nearest `standM` off it with a clear line. (workthings.ts standFor)
pub fn stand_for(g: &mut crate::game::Game, face: V3, n: Option<V3>, area: Option<crate::workthings::Area>) -> Option<V3> {
    let e = 0.3;
    let mut best: Option<V3> = None;
    let mut bs = m::INF;
    for i in -6i32..=6 {
        for j in -6i32..=6 {
            let dx = i as f64 * 0.2;
            let dz = j as f64 * 0.2;
            let d = m::hyp2(dx, dz);
            if d < 0.3 || d > 1.0 { continue; }
            let front = match n { Some(n) => (dx * n[0] + dz * n[2]) / d, None => 1.0 };
            if front < 0.5 { continue; }
            let x = face[0] + dx;
            let z = face[2] + dz;
            if crate::scene_collision::nav_grid_is_blocked(g, x, z) { continue; }
            if crate::scene_collision::solid_blocked(g, [x, 1.0, z], [face[0] + dx / d * 0.2, 1.0, face[2] + dz / d * 0.2], None) { continue; }
            let inside = match area { None => true, Some(a) => x >= a.x0 - e && x <= a.x1 + e && z >= a.z0 - e && z <= a.z1 + e };
            let s = m::abs(d - g.k.work.stand_m) + (1.0 - front) * 0.8 + (if inside { 0.0 } else { 10.0 });
            if s < bs { bs = s; best = Some([x, 0.0, z]); }
        }
    }
    best
}

/// The breaker panel: the point on its face and the normal out of it, out of the box's thin side. (workthings.ts panelFace)
pub fn panel_face(g: &mut crate::game::Game) -> crate::workthings::PanelFace {
    let bp = g.level.breaker.pos;
    // a missing index is `statics[undefined]`, a missing box `undefined`: no box
    let bx = g.level.breaker.boxes.first().and_then(|&i| g.engine.world.statics.get(i as usize).copied());
    let mut n: V3 = [0.0, 0.0, -1.0];
    let mut face: V3 = [bp[0], bp[1], bp[2]];
    if let Some(bx) = bx {
        let (c, h) = (g.col.boxes[bx].c, g.col.boxes[bx].h);
        let dx = bp[0] - c[0];
        let dz = bp[2] - c[2];
        if m::abs(dx) > m::abs(dz) {
            n = [js::or(m::sign(dx), 1.0), 0.0, 0.0];
            face = [c[0] + n[0] * m::min(h[0], h[2]), bp[1], bp[2]];
        } else {
            n = [0.0, 0.0, js::or(m::sign(dz), -1.0)];
            face = [bp[0], bp[1], c[2] + n[2] * m::min(h[0], h[2])];
        }
    }
    PanelFace { face, n }
}

/// The point just off the panel's real face that Sam's eye needs a line to; None when he is not in front of it. (workthings.ts samPanelPoint)
pub fn sam_panel_point(g: &mut crate::game::Game, from: V3) -> Option<V3> {
    let PanelFace { face, n } = panel_face(g);
    if (from[0] - face[0]) * n[0] + (from[2] - face[2]) * n[2] <= 0.0 { return None; }
    Some([face[0] + n[0] * 0.05, face[1], face[2] + n[2] * 0.05])
}

/// lighting.ts's 'check the breaker' errand, read off the panel, asked again every second. None: the fixed offsets, as under a show. (workthings.ts breakerErrand)
pub fn breaker_errand(g: &mut crate::game::Game) -> Option<crate::workthings::BreakerErrand> {
    if g.touring || g.quiet_utility { return None; }
    let bp = g.level.breaker.pos;
    let key = join_v3(bp);
    if let Some(c) = &g.m.workthings.errands {
        if c.key == key && g.time >= c.t && g.time - c.t < 1.0 { return c.e; }
    }
    let PanelFace { face, n } = panel_face(g);
    let at: V3 = [face[0] + n[0] * 0.05, 1.0, face[2] + n[2] * 0.05];
    let fixed: V3 = [bp[0] - 0.4, 0.0, bp[2] - 0.9];
    // walls only: a door leaf swung across the spot for a moment does not move the errand
    let dyn_boxes = core::mem::take(&mut g.col.dynamic_boxes);
    let kept = !crate::scene_collision::nav_grid_is_blocked(g, fixed[0], fixed[2]) && !crate::scene_collision::solid_blocked(g, [fixed[0], 1.0, fixed[2]], at, None);
    g.col.dynamic_boxes = dyn_boxes;
    let stand = if kept { Some(fixed) } else { stand_for(g, bp, Some(n), None) };
    let e = if kept {
        Some(BreakerErrand { goal: fixed, mid: [bp[0], 0.0, bp[2] - 0.6], at })
    } else {
        stand.map(|goal| BreakerErrand { goal, mid: [face[0] + n[0] * 0.6, 0.0, face[2] + n[2] * 0.6], at })
    };
    g.m.workthings.errands = Some(WorkthingsModErrandsValue { t: g.time, key, e });
    e
}

/// A question of the picture's: the list it reads is kept in the asker's store and the game's own put back after. (workthings.ts WorkThings.askAside)
pub fn work_things_ask_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, own: &mut crate::workthings::WorkThingsAskAsideOwn, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let was = g.work.asking;
    let mine = g.work.cache.take();
    g.work.asking = true;
    g.work.cache = own.c.take();
    let r = fn_(g);
    own.c = g.work.cache.take();
    g.work.cache = mine;
    g.work.asking = was;
    r
}

/// Makes the row and registers it on the orders' menu. (workthings.ts WorkThings.constructor)
pub fn work_things_new(g: &mut crate::game::Game) -> crate::workthings::WorkThings {
    let def = crate::orders::OrderDef {
        id: String::from(WORK_ORDER),
        key: Some(1.0),
        label: crate::orders::OrderLabel::Work,
        targets: vec![crate::orders::TargetKind::Floor],
        listed: Some(Vec::new()),
        ack: Some(vec![String::from("on it."), String::from("moving.")]),
        can: Some(crate::orders::OrderCan::Work),
        only: Some(crate::orders::OrderOnly::Work),
        valid: Some(crate::orders::OrderValid::Work),
        issue: crate::orders::OrderIssue::Work,
        tick: Some(crate::orders::OrderTick::Work),
        ..crate::orders::OrderDef::zeroed()
    };
    let row = g.tab.orders_order_def.push(def);
    crate::orders::squad_orders_register(g, row, None);
    WorkThings { row, ..WorkThings::blank() }
}

/// The things that can be worked now, read afresh when what places them changes and once a second besides. (workthings.ts WorkThings.things)
pub fn work_things_things(g: &mut crate::game::Game) -> Vec<crate::workthings::WorkThing> {
    let (active, permanent) = mains(g);
    let drive = g.mission.stage == crate::game::MissionStage::Drive && !g.quiet_utility && g.puppet.is_none();
    let w = if drive { crate::quiet::quiet_words(g) } else { None };
    let live = !(active && permanent);
    let bp = g.level.breaker.pos;
    let rf = g.level.mission.rack.front;
    let dkey = crate::defuse::defuse_key(g);
    let key = format!(
        "{}|{}|{}|{}|{}",
        join_v3(bp),
        live,
        if drive { join_v3(rf) } else { String::new() },
        match &w { Some(w) => w.holder.as_str(), None => "" },
        dkey,
    );
    if let Some(c) = &g.work.cache {
        if c.key == key && g.time >= c.t && g.time - c.t < 1.0 { return c.list.clone(); }
    }
    if !g.work.asking { work_things_prune(g); }
    let mut list: Vec<WorkThing> = Vec::new();
    if live {
        let PanelFace { face, n } = panel_face(g);
        let stand = stand_for(g, bp, Some(n), None);
        list.push(WorkThing { kind: WorkThingKind::Breaker, face, n, stand, name: String::from("the breaker"), dev: None });
    }
    if drive {
        let y = m::min(1.35, m::max(0.9, js::or(rf[1], 1.1)));
        let sr = &g.level.mission.server_room;
        let (x0, x1, z0, z1) = (sr.x0, sr.x1, sr.z0, sr.z1);
        let mut stand = if w.is_some() { stand_for(g, rf, None, Some(Area { x0, x1, z0, z1 })) } else { stand_for(g, rf, Some([0.0, 0.0, 1.0]), None) };
        if stand.is_none() && w.is_some() {
            let h = crate::quiet::holder_stand(g, rf, Some(crate::quiet::HolderStandArea { x0, x1, z0, z1 }));
            if let (Some(p), true) = (h.pos, h.clear) {
                if crate::mission::rack_dist(g, p) < crate::consts::DRIVE_REACH { stand = Some(p); }
            }
        }
        let mut n: V3 = [0.0, 0.0, 1.0];
        if let (Some(st), true) = (stand, w.is_some()) {
            let d = js::or(flat(st, rf), 1.0);
            n = [(st[0] - rf[0]) / d, 0.0, (st[2] - rf[2]) / d];
        }
        let name = match &w { Some(w) => w.holder.clone(), None => String::from("the rack") };
        list.push(WorkThing { kind: WorkThingKind::Objective, face: [rf[0], y, rf[2]], n, stand, name, dev: None });
    }
    for d in crate::defuse::defuse_armed(g) {
        let stand = stand_for(g, d.face, Some(d.n), None);
        list.push(WorkThing { kind: WorkThingKind::Device, face: d.face, n: d.n, stand, name: d.name, dev: Some(d.i) });
    }
    g.work.cache = Some(WorkThingsCache { key, t: g.time, list: list.clone() });
    list
}

/// Drops the records of men no longer at it, and hands with no living owner on the roster. (workthings.ts WorkThings.prune)
pub fn work_things_prune(g: &mut crate::game::Game) {
    let squad = crate::orders::squad_orders_squad(g);
    for gd in g.work.jobs.keys() {
        let a = g.work.jobs.get(gd).unwrap().a;
        if g.gd[gd].state == crate::game::GuardState::Dead || !squad.contains(&gd) || crate::orders::squad_orders_of(g, gd) != Some(a) {
            g.work.jobs.delete(gd);
        }
    }
    for c in g.work.hands.keys() {
        if !squad.iter().any(|&gd| g.gd[gd].ch == c && g.gd[gd].state != crate::game::GuardState::Dead) {
            g.work.hands.delete(c);
        }
    }
}

/// What working it would do now. (workthings.ts WorkThings.verbOf)
pub fn work_things_verb_of(g: &mut crate::game::Game, th: &crate::workthings::WorkThing) -> crate::workthings::WorkVerb {
    match th.kind {
        WorkThingKind::Device => WorkVerb::Defuse,
        WorkThingKind::Objective => WorkVerb::Drive,
        WorkThingKind::Breaker => if mains(g).0 { WorkVerb::BreakerOn } else { WorkVerb::BreakerOff },
    }
}

/// The row's words. (workthings.ts WorkThings.words)
pub fn work_things_words(g: &mut crate::game::Game, th: &crate::workthings::WorkThing) -> String {
    if th.kind == WorkThingKind::Device { return format!("make {} safe", th.name); }
    if th.kind == WorkThingKind::Breaker {
        return String::from(if mains(g).0 { "turn the power back on" } else { "throw the breaker" });
    }
    match crate::quiet::quiet_words(g) {
        Some(w) => format!("take {}", w.thing),
        None => String::from("pull the drive"),
    }
}

/// Why that is not there to be done any more (None: it is): the word the man says. (workthings.ts WorkThings.moot)
pub fn work_things_moot(g: &mut crate::game::Game, th: &crate::workthings::WorkThing, verb: crate::workthings::WorkVerb) -> Option<String> {
    if th.kind == WorkThingKind::Device { return crate::defuse::defuse_moot(g, th.dev.unwrap_or(-1.0)); }
    if th.kind == WorkThingKind::Objective {
        if g.mission.stage == crate::game::MissionStage::Drive { return None; }
        return Some(String::from(if crate::quiet::quiet_words(g).is_some() { "it's already gone." } else { "it's already out." }));
    }
    let (active, permanent) = mains(g);
    if active && permanent { return Some(String::from("it's scrap.")); }
    if verb == WorkVerb::BreakerOff {
        if active { Some(String::from("it's already dark.")) } else { None }
    } else if active {
        None
    } else {
        Some(String::from("it's already on."))
    }
}

/// orders.ts targetAt: the thing a point is on, on its own side of whatever it is fixed to. (workthings.ts WorkThings.near)
pub fn work_things_near(g: &mut crate::game::Game, at: V3) -> Option<crate::workthings::WorkThingsNear> {
    let mut best: Option<WorkThing> = None;
    let mut bd = m::INF;
    for th in work_things_things(g) {
        let d = flat(th.face, at);
        let wide = th.kind != WorkThingKind::Breaker;
        if d >= (if wide { g.k.work.pick_wide_m } else { g.k.work.pick_m }) || d >= bd { continue; }
        if (at[0] - th.face[0]) * th.n[0] + (at[2] - th.face[2]) * th.n[2] < -(if wide { g.k.work.behind_m } else { g.k.work.behind_panel_m }) { continue; }
        bd = d;
        best = Some(th);
    }
    best.map(|th| WorkThingsNear { d: bd, target: crate::orders::OrderTarget::Floor(thing_target(th)) })
}

/// orders.ts targetUnder: the thing whose own box the cursor's ray landed on. (workthings.ts WorkThings.on)
pub fn work_things_on(g: &mut crate::game::Game, box_: Option<BoxId>) -> Option<crate::workthings::WorkThingsOn> {
    let b = box_?;
    if !g.level.breaker.boxes.iter().any(|&i| g.engine.world.statics.get(i as usize) == Some(&b)) { return None; }
    let th = work_things_things(g).into_iter().find(|x| x.kind == WorkThingKind::Breaker)?;
    Some(WorkThingsOn::Floor(thing_target(th)))
}

/// orders.ts rowsFor: a thing's menu is the floor's with its own order where 'move there' was. (workthings.ts WorkThings.rows)
pub fn work_things_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> {
    let on_thing = matches!(t, crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor { thing: Some(_), .. }));
    if page.is_some() || !on_thing { return rows.to_vec(); }
    let mut out = vec![g.work.row];
    for &id in rows {
        let d = &g.tab.orders_order_def[id];
        if d.id != "move" && d.stands_in.as_ref().map_or(true, |s| s.for_ != "move") && d.key != Some(1.0) { out.push(id); }
    }
    out
}

/// Where the man who covers goes: a free spot he can reach behind the man at work (None: none). (workthings.ts WorkThings.coverPost)
pub fn work_things_cover_post(g: &mut crate::game::Game, mate: GuardId, th: &crate::workthings::WorkThing) -> Option<V3> {
    let stand = th.stand?;
    let p = pos(g, mate);
    let side = js::or(m::sign((p[0] - stand[0]) * th.n[2] - (p[2] - stand[2]) * th.n[0]), 1.0);
    let buddy = g.k.work.buddy_m;
    for k in [side, -side, 0.0] {
        let post = crate::orders::squad_orders_free_spot(g, [stand[0] + th.n[0] * buddy + th.n[2] * 0.5 * k, 0.0, stand[2] + th.n[2] * buddy - th.n[0] * 0.5 * k]);
        if let Some(post) = post {
            if flat(post, stand) > 0.7 && flat(post, stand) < buddy + 1.2 && crate::orders::squad_orders_reach(g, mate, post).is_none() { return Some(post); }
        }
    }
    None
}

/// Who it is for: the nearest who can get there, and with more than one addressed his partner if he has a mark to cover from. (workthings.ts WorkThings.pick)
pub fn work_things_pick(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    let at = target_at(t);
    let mut able: Vec<GuardId> = Vec::new();
    for &gd in men {
        if crate::orders::squad_orders_reach(g, gd, at).is_none() { able.push(gd); }
    }
    let th = match t {
        crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor { thing: Some(th), .. }) if !able.is_empty() => th,
        _ => return Vec::new(),
    };
    let mut w = able[0];
    for &b in &able[1..] {
        if flat(pos(g, b), at) < flat(pos(g, w), at) { w = b; }
    }
    let id = crate::orders::squad_orders_pair_of(g, w);
    let mut mate: Option<GuardId> = None;
    if men.len() > 1 {
        for &gd in &able {
            if gd != w
                && (!crate::orders::squad_orders_paired(g) || crate::orders::squad_orders_pair_of(g, gd) == id)
                && work_things_cover_post(g, gd, th).is_some()
            {
                mate = Some(gd);
                break;
            }
        }
    }
    match mate { Some(mate) => vec![w, mate], None => vec![w] }
}

/// Gives the order: the nearest works it, any other covers from a mark behind him or holds. (workthings.ts WorkThings.issue)
pub fn work_things_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::workthings::WorkThingsIssue {
    let th = match t {
        crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor { thing: Some(th), .. }) if th.stand.is_some() => th.clone(),
        _ => return WorkThingsIssue::Text(String::from("nothing to work there.")),
    };
    let stand = th.stand.unwrap();
    let verb = work_things_verb_of(g, &th);
    work_things_prune(g);
    let mut w = men[0];
    for &b in &men[1..] {
        if flat(pos(g, b), stand) < flat(pos(g, w), stand) { w = b; }
    }
    let to_it = m::atan2(-th.n[0], -th.n[2]);
    let out = m::atan2(th.n[0], th.n[2]);
    let walk = crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim };
    crate::orders::squad_orders_assign(g, w, &crate::orders::SquadOrdersAssignA {
        order: String::from(WORK_ORDER),
        post: Some([stand[0], 0.0, stand[2]]),
        face: Some(to_it),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(walk),
        going: format!("to {}", th.name),
        there: format!("at {}", th.name),
        set_line: None,
        ..Default::default()
    });
    let a = crate::orders::squad_orders_of(g, w).unwrap();
    let now = g.time;
    g.work.jobs.set(w, Job { thing: th.clone(), verb, a, t0: -1.0, last_t: now });
    let mut covers: Vec<String> = Vec::new();
    for &mate in men {
        if mate == w { continue; }
        let p = pos(g, mate);
        let post = work_things_cover_post(g, mate, &th);
        if let Some(post) = post {
            let there = format!("covering {}", g.gd[w].callsign);
            crate::orders::squad_orders_assign(g, mate, &crate::orders::SquadOrdersAssignA {
                order: String::from("move"),
                post: Some(post),
                face: Some(out),
                stance: Some(crate::character::Stance::LowReady),
                upper: Some(crate::character::UpperMode::None),
                move_: Some(walk),
                spare: Some(true),
                from: Some([p[0], 0.0, p[2]]),
                going: String::from("moving"),
                there,
                set_line: Some(String::from("set.")),
                ..Default::default()
            });
        } else {
            let yaw = g.ch[g.gd[mate].ch].body_yaw;
            crate::orders::squad_orders_assign(g, mate, &crate::orders::SquadOrdersAssignA {
                order: String::from("hold"),
                post: Some([p[0], 0.0, p[2]]),
                face: Some(yaw),
                stance: Some(crate::character::Stance::LowReady),
                upper: Some(crate::character::UpperMode::None),
                going: String::from("holding"),
                there: String::from("holding"),
                ..Default::default()
            });
        }
        covers.push(g.gd[mate].callsign.clone());
    }
    g.stats.workthings.given += 1;
    let tail = if covers.is_empty() { String::new() } else { format!(" · {} covers", covers.join(", ")) };
    let what = format!("sent to {} at {},{}{}", verb.as_str(), js::to_fixed(stand[0], 1), js::to_fixed(stand[2], 1), tail);
    work_things_note(g, w, &what);
    WorkThingsIssue::Arm1(())
}

/// Once a frame for the man at work, before the orders pose him. (workthings.ts WorkThings.tick)
pub fn work_things_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let now = g.time;
    let mut j = match g.work.jobs.get(gd) { Some(j) if j.a == a => j.clone(), _ => return };
    let th = j.thing.clone();
    let verb = j.verb;
    let stand = th.stand.unwrap();
    let c = g.gd[gd].ch;
    let h0 = g.work.hands.get(c).copied();
    if j.t0 >= 0.0 && (now - j.last_t > 0.25 || h0.map_or(true, |h| h.until != m::INF)) { work_things_hand_off(g, gd, &mut j); }
    j.last_t = now;
    put_job(g, gd, &j);
    // the run has ended under his hand: the mission's thing is nobody's, a charge stays armed; the breaker is the level's and goes on
    let pl = g.player;
    if g.pl[pl].down && (verb == WorkVerb::Drive || verb == WorkVerb::Defuse) {
        g.stats.workthings.moot += 1;
        work_things_note(g, gd, "moot: the run is over");
        work_things_settle(g, gd, &mut j, None, false);
        return;
    }
    if let Some(why) = work_things_moot(g, &th, verb) {
        g.stats.workthings.moot += 1;
        work_things_note(g, gd, &format!("moot: {}", why));
        work_things_settle(g, gd, &mut j, Some(&why), true);
        return;
    }
    // the orders bring a mark to a man who cannot reach it, and call that arrived: he is not at the thing
    let far = match g.tab.orders_assignment[a].post { None => true, Some(post) => flat(post, stand) > 0.3 };
    if far {
        g.stats.workthings.moot += 1;
        work_things_note(g, gd, "stopped short of it");
        let line = if g.tab.orders_assignment[a].there == "stopped short" { None } else { Some("can't get to it.") };
        work_things_settle(g, gd, &mut j, line, false);
        return;
    }
    let p = pos(g, gd);
    let d_f = js::or(flat(p, th.face), 1.0);
    let on = g.tab.orders_assignment[a].arrived
        && flat(p, stand) < 0.45
        && m::abs(v3::wrap_angle(g.ch[c].body_yaw - g.tab.orders_assignment[a].face.unwrap_or(0.0))) < 0.6
        && !crate::scene_collision::solid_blocked(g, [p[0], 1.0, p[2]], [th.face[0] + (p[0] - th.face[0]) / d_f * 0.2, 1.0, th.face[2] + (p[2] - th.face[2]) / d_f * 0.2], None);
    let low = !on && crate::sneak::sneak_low(g, gd);
    let low_pace = g.k.sneak.low_pace;
    g.tab.orders_assignment[a].crouch = Some(low);
    g.tab.orders_assignment[a].pace = if low { Some(low_pace) } else { None };
    if !on {
        if j.t0 >= 0.0 { work_things_hand_off(g, gd, &mut j); }
        g.tab.orders_assignment[a].there = format!("at {}", th.name);
        return;
    }
    if j.t0 < 0.0 {
        j.t0 = now;
        put_job(g, gd, &j);
        if d_f < g.k.work.hand_m {
            let need = work_things_need_of(g, verb);
            let push = crate::character::DoorPush { p: th.face, n: th.n, w: 1.0, hand: 0.0, hold_only: None, reach: None };
            g.work.hands.set(c, WorkThingsHandsValue { push, until: m::INF, seen: now, verb, t0: now, need });
        }
        if verb == WorkVerb::Drive {
            let w = crate::quiet::quiet_words(g);
            crate::audio_iface::game_audio_like_play(g, "click", Some(th.face), Some(0.8), None);
            seam_audio_play_rate(g, "magOut", Some(th.face), 0.4, 0.55);
            if !g.mission.hands_on {
                g.mission.hands_on = true;
                let text = match &w { Some(w) => format!("hands on {}", w.thing), None => String::from("hands on the drive") };
                crate::chronicle::chron_objective(g, &text, Some(th.face));
            }
        }
    }
    if let Some(h) = g.work.hands.get_mut(c) { h.seen = now; }
    let need = work_things_need_of(g, verb);
    let k = (now - j.t0) / need;
    g.tab.orders_assignment[a].there = if verb == WorkVerb::Drive || verb == WorkVerb::Defuse {
        format!("{}… {}%", work_things_words(g, &th), m::min(99.0, m::round(k * 100.0)))
    } else {
        format!("on {}", th.name)
    };
    if k < 1.0 { return; }
    let line: String;
    match verb {
        WorkVerb::BreakerOff => {
            crate::game::game_set_blackout(g, 86400.0, crate::game::GameSetBlackoutCause::Hand);
            line = String::from("lights out.");
        }
        WorkVerb::BreakerOn => {
            crate::game::game_end_blackout(g);
            line = String::from("power's back.");
        }
        WorkVerb::Defuse => {
            let i = th.dev.unwrap_or(-1.0);
            crate::defuse::defuse_make_safe(g, i, crate::defuse::DefuseMakeSafeBy::Guard(gd));
            // (the word follows the deed: makeSafe has its own reasons to do nothing; index -1 is no row)
            let safe = i >= 0.0 && g.defuse.states.get(i as usize) == Some(&crate::defuse::ChargeState::Safe);
            if !safe {
                g.stats.workthings.moot += 1;
                work_things_note(g, gd, "moot: not made safe");
                work_things_settle(g, gd, &mut j, None, false);
                return;
            }
            line = String::from("charge is safe.");
        }
        WorkVerb::Drive => {
            // the clunk is the rack's, heard as ever: it is not Sam's own noise
            let n0 = g.pl[pl].noise;
            let w = crate::quiet::quiet_words(g);
            crate::mission::drive_pulled(g);
            g.pl[pl].noise = n0;
            line = match &w { Some(w) => format!("got {}.", w.bare), None => String::from("got the drive.") };
        }
    }
    g.stats.workthings.done += 1;
    work_things_note(g, gd, &format!("done: {}", verb.as_str()));
    work_things_settle(g, gd, &mut j, Some(&line), true);
}

/// The hand comes off, eased by the layer, and the job's clock stops. (workthings.ts WorkThings.handOff)
pub fn work_things_hand_off(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::workthings::Job) {
    j.t0 = -1.0;
    put_job(g, gd, j);
    let c = g.gd[gd].ch;
    let until = g.time + g.k.work.fade_secs;
    if let Some(h) = g.work.hands.get_mut(c) {
        if h.until == m::INF { h.until = until; h.push.w = 0.0; }
    }
}

/// Done with it: a word, the hand off, and a plain hold where he stands, his back to the thing if he was at it. (workthings.ts WorkThings.settle)
pub fn work_things_settle(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::workthings::Job, line: Option<&str>, turn: bool) {
    let p = pos(g, gd);
    work_things_hand_off(g, gd, j);
    g.work.jobs.delete(gd);
    if let Some(line) = line { crate::game::game_say(g, gd, line, None, None, None); }
    let th = &j.thing;
    let face = if turn { m::atan2(th.n[0], th.n[2]) } else { g.ch[g.gd[gd].ch].body_yaw };
    crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
        order: String::from("hold"),
        post: Some([p[0], 0.0, p[2]]),
        face: Some(face),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        going: String::from("holding"),
        there: if turn { format!("holding · {}", th.name) } else { String::from("holding") },
        set_line: None,
        ..Default::default()
    });
}

/// game.ts handsOnDoors: the hand a man at work has out (None: none). (workthings.ts WorkThings.hand)
pub fn work_things_hand(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::character::DoorPush> {
    let now = g.time;
    let fade = g.k.work.fade_secs;
    let h = g.work.hands.get_mut(c_2)?;
    if h.until == m::INF && now - h.seen > 0.25 { h.until = now + fade; h.push.w = 0.0; }   // (the fight has him)
    if now > h.until || now < h.seen - 1.0 {   // (… or the clock has gone back under it: a restart)
        g.work.hands.delete(c_2);
        return None;
    }
    Some(h.push)
}

/// How long the work on a thing takes from the hand going out, s. (workthings.ts WorkThings.needOf)
pub fn work_things_need_of(g: &mut crate::game::Game, verb: crate::workthings::WorkVerb) -> f64 {
    let hand = g.k.work.hand_secs;
    hand + (match verb {
        WorkVerb::Drive => crate::consts::DRIVE_SECS,
        WorkVerb::Defuse => crate::defuse::defuse_work_secs(g),
        _ => g.k.work.breaker_secs,
    })
}

/// What the hand out is doing, for whoever poses it. (workthings.ts WorkThings.job)
pub fn work_things_job(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::workthings::WorkJob> {
    let now = g.time;
    let h = g.work.hands.get(c_2)?;
    Some(WorkJob { verb: h.verb, secs: m::max(0.0, now - h.t0), need: h.need })
}

/// He is at work and stood on the spot his thing is worked from. (workthings.ts WorkThings.atThing)
pub fn work_things_at_thing(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let (a, stand) = match g.work.jobs.get(gd) { Some(j) => (j.a, j.thing.stand), None => return false };
    if !work_things_working(g, gd) || !g.tab.orders_assignment[a].arrived { return false; }
    match stand { Some(s) => flat(pos(g, gd), s) < 0.45, None => false }
}

/// Is this man working a thing. (workthings.ts WorkThings.working)
pub fn work_things_working(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let a = match g.work.jobs.get(gd) { Some(j) => j.a, None => return false };
    g.gd[gd].state != crate::game::GuardState::Dead && crate::orders::squad_orders_of(g, gd) == Some(a)
}

/// The man an order has on a defuse mission's charge i; a man with his hand on it is named before one still walking. (workthings.ts WorkThings.onDevice)
pub fn work_things_on_device(g: &mut crate::game::Game, i: f64) -> Option<crate::workthings::WorkThingsOnDevice> {
    let mut best: Option<WorkThingsOnDevice> = None;
    for gd in g.work.jobs.keys() {
        let (t0, last_t, here) = {
            let j = g.work.jobs.get(gd).unwrap();
            (j.t0, j.last_t, j.thing.kind == WorkThingKind::Device && j.thing.dev == Some(i))
        };
        if here && work_things_working(g, gd) {
            let on = t0 >= 0.0 && g.time - last_t <= 0.25;
            if on { return Some(WorkThingsOnDevice { gd, on }); }
            if best.is_none() { best = Some(WorkThingsOnDevice { gd, on }); }
        }
    }
    best
}

/// A line of the module's log, kept to 200. (workthings.ts WorkThings.note)
pub fn work_things_note(g: &mut crate::game::Game, gd: GuardId, what: &str) {
    let row = WorkThingsLogItem { t: g.time, who: g.gd[gd].callsign.clone(), what: String::from(what) };
    g.work.log.push(row);
    if g.work.log.len() > 200 { g.work.log.drain(0..50); }
}

/// One line for a probe. (workthings.ts WorkThings.summary)
pub fn work_things_summary(g: &mut crate::game::Game) -> String {
    let th = work_things_things(g);
    let mut n = 0;
    for gd in g.work.jobs.keys() {
        if work_things_working(g, gd) { n += 1; }
    }
    let parts: Vec<String> = th.iter().map(|x| format!("{}{}", x.name, if x.stand.is_some() { "" } else { " (no place to stand)" })).collect();
    let joined = parts.join(" · ");
    let things = if joined.is_empty() { String::from("nothing to work") } else { joined };
    let s = &g.stats.workthings;
    format!("{} · {} at work · given {} done {} moot {}", things, n, s.given, s.done, s.moot)
}

/// orders.ts targetAt: a thing a teammate can work near a point. (workthings.ts workNear)
pub fn work_near(g: &mut crate::game::Game, at: V3) -> Option<crate::workthings::WorkNear> {
    work_things_near(g, at).map(|x| WorkNear { d: x.d, target: x.target })
}

/// orders.ts targetUnder: the thing whose own box the cursor is on. (workthings.ts workOn)
pub fn work_on(g: &mut crate::game::Game, box_: Option<BoxId>) -> Option<crate::workthings::WorkOn> {
    work_things_on(g, box_).map(|t| match t {
        WorkThingsOn::Floor(x) => WorkOn::Floor(x),
        WorkThingsOn::Door(x) => WorkOn::Door(x),
        WorkThingsOn::None(x) => WorkOn::None(x),
        WorkThingsOn::Foe(x) => WorkOn::Foe(x),
    })
}

/// orders.ts rowsFor. (workthings.ts workRows)
pub fn work_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> {
    work_things_rows(g, t, page, rows)
}

/// game.ts handsOnDoors: the first seat with a hand out for this body. (workthings.ts workHand)
pub fn work_hand(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::character::DoorPush> {
    if g.seats.len() == 1 { return work_things_hand(g, c_2); }
    for i in 0..g.seats.len() {
        let pl = crate::cmd::seat_man(g, g.seats[i]);
        let r = crate::seats::with_seat(g, pl, &mut |v: &mut Game| work_things_hand(v, c_2));
        if r.is_some() { return r; }
    }
    None
}

/// A man stood at the thing he was sent to work does not take as a target a man his round is held for. (workthings.ts workPasses)
pub fn work_passes(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    work_things_at_thing(g, gd) && crate::silent::silent_holds(g, gd, h)
}

/// The man is walking to the breaker and this teammate is stood on that walk: he is not held for him. (workthings.ts workComing)
pub fn work_coming(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    let end = match &g.gd[h].task {
        Some(crate::game::GuardTask::Arm0(t)) if t.kind == crate::game::GuardTaskArm0Kind::Breaker => t.pos,
        _ => return false,
    };
    if g.gd[h].state != crate::game::GuardState::Suspicious { return false; }
    let p = pos(g, gd);
    let n = g.gd[h].path.len();
    let goal = match g.gd[h].path_goal { Some(goal) => goal, None => return false };
    if flat(goal, end) > 1.0 { return false; }
    let walk_m = g.k.work.walk_m;
    let mut a = pos(g, h);
    let mut i = core::cmp::min(g.gd[h].path_i, n);
    while i <= n {
        let b = if i < n { g.gd[h].path[i] } else { end };
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let l2 = dx * dx + dz * dz;
        let k = if l2 > 1e-6 { m::max(0.0, m::min(1.0, ((p[0] - a[0]) * dx + (p[2] - a[2]) * dz) / l2)) } else { 0.0 };
        let qx = a[0] + dx * k;
        let qz = a[2] + dz * k;
        if m::hyp2(p[0] - qx, p[2] - qz) < walk_m && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [qx, 0.45, qz], None) { return true; }
        a = b;
        i += 1;
    }
    false
}

/// A pose layer's read beside workHand: what that hand is doing (None whenever workHand is). (workthings.ts workJob)
pub fn work_job(g: &mut crate::game::Game, c_2: CharId) -> Option<crate::workthings::WorkJob> {
    if work_things_hand(g, c_2).is_some() { work_things_job(g, c_2) } else { None }
}

/// lighting.ts: is the breaker none of this man's business unless he is told. (workthings.ts workNotHis)
pub fn work_not_his(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some()
}

/// lighting.ts setBlackout: a man who cannot leave where he stands is not handed the errand. (workthings.ts workStaysPut)
pub fn work_stays_put(g: &mut crate::game::Game, gd: GuardId) -> bool {
    !g.touring && !g.quiet_utility && (g.gd[gd].hold || g.gd[gd].pinned || g.gd[gd].script.is_some())
}
