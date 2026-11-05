//! src/game/wallcharge.ts lines 785 to 1139
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

macro_rules! o { ($g:expr, $op:expr) => { $g.tab.wallcharge_wall_op[$op] } }
macro_rules! asg { ($g:expr, $a:expr) => { $g.tab.orders_assignment[$a] } }

// ------------------------------------------------------------ seams: what skeleton 3 still lacks; its next run changes these, not the bodies
/// `M.target === t` (ts 785): the open menu's serial when `t` is that menu's very target, else None. The serial is also `M` itself (ts 826)
fn seam_own_menu(g: &mut Game, t: &crate::orders::OrderTarget) -> Option<u32> { todo!("needs: OrderTargetFloor.serial, stamped where a menu is opened") }
/// plan with the open menu's Kept (ts 804)
fn seam_plan_kept(g: &mut Game, w: &WallSpot, men: &[GuardId]) -> WallChargePlan { todo!("needs: wall_charge_plan's `kept` as WallChargeKept (Menu)") }
/// safePosts with the order's own Kept, op.repickKept (ts 872, 873)
fn seam_safe_posts_kept(g: &mut Game, op: WallOpId, w: &WallSpot, n: f64, spot: V3) -> Option<Vec<V3>> { todo!("needs: wall_charge_safe_posts's `kept` as WallChargeKept (Repick)") }

// ------------------------------------------------------------ small helpers
/// a truthy string that may be absent
fn word_on(s: &Option<String>) -> bool { s.as_deref().map_or(false, |x| !x.is_empty()) }
/// `S.of(gd) === op.mine.get(gd)`: `of` answers null and `get` undefined, so a man in neither is NOT the same
fn still_mine(g: &mut Game, op: WallOpId, gd: GuardId) -> bool {
    let now = crate::orders::squad_orders_of(g, gd);
    match (now, o!(g, op).mine.get(gd).copied()) { (Some(a), Some(b)) => a == b, _ => false }
}
fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
/// a callsign, or the words for a man with none
fn callsign_or(g: &Game, gd: GuardId, none: &str) -> String { let c = &g.gd[gd].callsign; if c.is_empty() { String::from(none) } else { c.clone() } }
/// an assignment's two words and nothing else: the rest as `a` is filled in
fn job(going: &str, there: &str) -> WallChargeAssignA { WallChargeAssignA { going: String::from(going), there: String::from(there), ..Default::default() } }
fn none_upper(a: WallChargeAssignA) -> WallChargeAssignA { WallChargeAssignA { stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::None), ..a } }
fn none_aim(a: WallChargeAssignA) -> WallChargeAssignA { WallChargeAssignA { stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), ..a } }

// ------------------------------------------------------------ underFire's two answers: the match where they are called (conventions 3; the captures are WallChargeFireAsk)
/// a round's line, the wall about to go being no cover where it will open   (underFire's `line`)
fn fire_line(g: &mut Game, wall: &Option<WallSpot>, a: V3, b: V3) -> bool {
    if !crate::scene_collision::segment_blocked(g, a, b, None, None, None) { return true; }
    let w = match wall { Some(w) => w, None => return false };
    let tk = thick(g, w);
    let half = g.k.walls.blast.radius * g.k.walls.blast.hole_k;
    let (oa, ob) = (wall_off(g, w, a), wall_off(g, w, b));
    if !(oa < -tk && ob > 0.0) { return false; }   // (he is past its far face, the place on the charge's side)
    let at = |off: f64| v3::lerp3(a, b, (oa - off) / (oa - ob));
    if m::abs(wall_along(w, at(-tk / 2.0))) > half { return false; }
    !crate::scene_collision::segment_blocked(g, a, at(-tk - 0.05), None, None, None) && !crate::scene_collision::segment_blocked(g, at(0.05), b, None, None, None)
}
/// how many of a man's six places on `q` a foe at `f` has a line to; `stop`: the first ends the asking   (underFire's `lines`)
fn fire_lines(g: &mut Game, wall: &Option<WallSpot>, f: V3, q: V3, stop: bool) -> f64 {
    let e = g.k.wallcharge.fire_step;
    let from = [f[0], 1.4, f[2]];
    let mut n = 0.0;
    for [dx, dz, y] in [[0.0, 0.0, 1.1], [0.0, 0.0, 1.55], [e, 0.0, 1.1], [-e, 0.0, 1.1], [0.0, e, 1.1], [0.0, -e, 1.1]] {
        let (x, z) = (q[0] + dx, q[2] + dz);
        if (js::truthy(dx) || js::truthy(dz)) && crate::scene_collision::nav_grid_is_blocked(g, x, z) { continue; }
        if fire_line(g, wall, from, [x, y, z]) { n += 1.0; if stop { break; } }
    }
    n
}
fn fire_key(q: V3) -> [crate::Num; 3] { [crate::Num(q[0]), crate::Num(q[1]), crate::Num(q[2])] }
/// `fire.score(q)`: how much under known fire a place is, 0 = not at all; each place asked once   (underFire's `score`)
fn fire_score(g: &mut Game, fire: &mut WallChargeFireAsk, q: V3) -> f64 {
    let WallChargeFireAsk::Lines { wall, all, memo, hit } = fire;
    let key = fire_key(q);
    if let Some(&v) = memo.get(key) { return v; }
    let fire_m = g.k.wallcharge.fire_m;
    let mut sum = 0.0;
    for &f in all.iter().filter(|&&f| v3::dist_xz(f, q) <= fire_m) {
        sum += fire_lines(g, wall, f, q, false) / 6.0 / m::max(1.0, v3::dist_xz(f, q));
    }
    let v = if sum > 0.0 { 1.0 + m::floor(sum / g.k.wallcharge.fire_grain) } else { 0.0 };
    memo.set(key, v);
    hit.set(key, v > 0.0);
    v
}
/// `fire.any(q)`: is it under fire at all, the first clear line ending the asking   (underFire's `any`)
fn fire_any(g: &mut Game, fire: &mut WallChargeFireAsk, q: V3) -> bool {
    let WallChargeFireAsk::Lines { wall, all, hit, .. } = fire;
    let key = fire_key(q);
    if let Some(&v) = hit.get(key) { return v; }
    let fire_m = g.k.wallcharge.fire_m;
    let mut v = false;
    for &f in all.iter().filter(|&&f| v3::dist_xz(f, q) <= fire_m) {
        if fire_lines(g, wall, f, q, true) > 0.0 { v = true; break; }
    }
    hit.set(key, v);
    v
}

// ------------------------------------------------------------ the row's tests
/// the walks of the open menu when `t` is that menu's own target   (wallcharge.ts WallCharge.keptFor)
pub fn wall_charge_kept_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::FnField> {
    if !g.wall_charge.keep.on || g.orders.menu.is_none() { return None; }
    let m = seam_own_menu(g, t)?;
    Some(wall_charge_kept_walks(g, m))
}

/// `kept(key, () => S.reach(gd, spot))` on the open menu's store: asked once while nothing a walk is judged by changes   (wallcharge.ts keptWalks's closure)
fn kept_reach(g: &mut Game, key: String, gd: GuardId, spot: V3) -> Option<String> {
    g.wall_charge.keep.asks += 1.0;
    let had = g.wall_charge.walks.as_ref().and_then(|k| k.map.get(key.clone()).cloned());
    if let Some(ans) = had {
        g.wall_charge.keep.hits += 1.0;
        return match ans { WallChargeWalkAnswer::Reach { why } => why, WallChargeWalkAnswer::Walk { .. } => unreachable!("an r| key holds a Reach") };
    }
    let why = crate::orders::squad_orders_reach(g, gd, spot);
    if let Some(k) = g.wall_charge.walks.as_mut() { k.map.set(key, WallChargeWalkAnswer::Reach { why: why.clone() }); }
    why
}

/// one man's part: a man with no walk to the wall says so   (wallcharge.ts WallCharge.canOf)
pub fn wall_charge_can_of(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> crate::wallcharge::WallChargeCanOf {
    if !wall_charge_sync(g) { return WallChargeCanOf::Bool(true); }
    let w = match wall_charge_spot(g, t) { Some(w) => w, None => return WallChargeCanOf::Bool(true) };
    if !w.ok || wall_charge_charge_on(g, &w).is_some() || wall_charge_open_at(g, &w) { return WallChargeCanOf::Bool(true); }
    let spot = match wall_charge_set_spot(g, &w) { Some(s) => s, None => return WallChargeCanOf::Bool(true) };
    let cs = g.col.nav.cell;
    let said = if wall_charge_kept_for(g, t).is_some() {
        let p = pos_of(g, gd);
        let key = format!("r|{}|{},{}|{},{}", g.ch[g.gd[gd].ch].id, m::floor(p[0] / cs), m::floor(p[2] / cs), spot[0], spot[2]);
        kept_reach(g, key, gd, spot)
    } else { crate::orders::squad_orders_reach(g, gd, spot) };
    match said { Some(s) => WallChargeCanOf::Text(s), None => WallChargeCanOf::Bool(true) }
}

/// the row's test: true, or the refusal in words   (wallcharge.ts WallCharge.valid)
pub fn wall_charge_valid(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::wallcharge::WallChargeValid {
    if !wall_charge_sync(g) { return WallChargeValid::Bool(false); }
    let w = match wall_charge_spot(g, t) { Some(w) => w, None => return WallChargeValid::Bool(false) };
    if wall_charge_charge_on(g, &w).is_some() { return WallChargeValid::Bool(true); }
    if !w.ok { return WallChargeValid::Text(w.why.clone()); }
    if wall_charge_open_at(g, &w) { return WallChargeValid::Text(String::from("it's open.")); }
    if !wall_charge_affordable(g) { return WallChargeValid::Text(wall_charge_none_words(g)); }
    let on_it = g.wall_charge.ops.iter().any(|&x| { let r = &g.tab.wallcharge_wall_op[x]; r.phase == Phase::Place && same_wall(&r.wall, &w) });
    if on_it { return WallChargeValid::Text(String::from("already on it.")); }
    let plan = if wall_charge_kept_for(g, t).is_some() { seam_plan_kept(g, &w, men) } else { wall_charge_plan(g, &w, men, None) };
    match plan { WallChargePlan::Text(s) => WallChargeValid::Text(s), _ => WallChargeValid::Bool(true) }
}

/// the walks the open menu asks, kept while nothing a walk is judged by has changed   (wallcharge.ts WallCharge.keptWalks)
pub fn wall_charge_kept_walks(g: &mut crate::game::Game, m: u32) -> crate::FnField {
    let p = g.player;
    let mut locks = String::new();
    for d in g.doors.list.clone() {
        let shut = crate::doors::door_locked(g, d) && crate::doors::door_latched(g, d);
        locks.push(if shut { '1' } else { '0' });
    }
    let mut h: i32 = 0;
    for &b in g.col.nav.blocked.iter() { h = h.wrapping_mul(31).wrapping_add(b as i32); }
    let stamp = format!("{}|{}|{}|{}|{}|{}|{}:{}|{},{}|1",
        g.pl[p].keys.keys().join(","), g.pl[p].codes.keys().join(","), locks, g.cleared.known, g.destr.walls.serial,
        g.engine.world.geom_serial, g.col.nav.blocked.len(), h, g.orders.knobs.cleared_dear, g.orders.knobs.round_cap);
    let same = match &g.wall_charge.walks { Some(k) => k.menu == m && k.stamp == stamp && k.map.len() <= 512, None => false };
    if !same { g.wall_charge.walks = Some(WallChargeWalks { menu: m, stamp, map: JsMap::new() }); }
    crate::FnField
}

// ------------------------------------------------------------ the order
/// one man's job on this order, kept as the order's   (wallcharge.ts WallCharge.assign)
pub fn wall_charge_assign(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, a: &crate::wallcharge::WallChargeAssignA) {
    let b = crate::orders::SquadOrdersAssignA {
        post: a.post, going: a.going.clone(), order: a.order.clone().unwrap_or_else(|| String::from(WALL_ORDER)),
        face: a.face, there: a.there.clone(), t0: a.t0, follow: a.follow, watch: a.watch, door: a.door, strafe: a.strafe, stance: a.stance,
        upper: a.upper, stack_side: a.stack_side, move_: a.move_.clone(), set_line: a.set_line.clone(), group: a.group, set_said: a.set_said,
        goal: a.goal, arrived: a.arrived, held_set: a.held_set, held_fire: a.held_fire, arr_t: a.arr_t, moved_t: a.moved_t, light: a.light,
        low: a.low, yield_t: a.yield_t, face_now: a.face_now, search: a.search, search_t: a.search_t, via: a.via.clone(), anchor: a.anchor,
        mark: a.mark, mark_t: a.mark_t, had: a.had, back_n: a.back_n, shy: a.shy, thru: a.thru, via_for: a.via_for, via_t: a.via_t, pace: a.pace,
        spare: a.spare, from: a.from, foe: a.foe, crouch: a.crouch, lean: a.lean, fit: a.fit.clone(), tries: a.tries.clone(),
    };
    crate::orders::squad_orders_assign(g, gd, &b);
    let mine = crate::orders::squad_orders_of(g, gd).unwrap();
    o!(g, op).mine.set(gd, mine);
}

/// a man to his stand-off mark, his back to the charge until it has gone   (wallcharge.ts WallCharge.standOff)
pub fn wall_charge_stand_off(g: &mut crate::game::Game, op: WallOpId, gd: GuardId) {
    let w = o!(g, op).wall.clone();
    let p = *o!(g, op).post.get(gd).unwrap();
    let height = g.k.wallcharge.height;
    let watch = wall_pt(g, &w, 0.0, 0.0, Some(height));
    let away = o!(g, op).phase != Phase::Wait;
    let a = WallChargeAssignA {
        post: Some(p),
        face: Some(if away { v3::yaw_to(watch, p) } else { v3::yaw_to(p, watch) }),
        watch: if away { None } else { Some(watch) },
        stance: Some(crate::character::Stance::HighReady),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        ..job("standing off", "stood off · the wall")
    };
    wall_charge_assign(g, op, gd, &a);
}

/// the setter's job: to the wall's foot, facing it
fn set_job(g: &mut Game, w: &WallSpot, spot: V3) -> WallChargeAssignA {
    let a = job("charge · the wall", "setting the charge");
    let foot = wall_pt(g, w, 0.0, 0.0, None);
    WallChargeAssignA { post: Some(spot), face: Some(v3::yaw_to(spot, foot)), watch: None, ..none_upper(a) }
}

/// the row's release: a set charge there is blown; else one man to the wall, the rest to the stand-off   (wallcharge.ts WallCharge.issue)
pub fn wall_charge_issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::wallcharge::WallChargeIssue {
    if !wall_charge_sync(g) { return WallChargeIssue::Text(String::from("not here")); }
    let w = match wall_charge_spot(g, t) { Some(w) => w, None => return WallChargeIssue::Text(String::from("no wall there")) };
    if let Some(c) = wall_charge_charge_on(g, &w) { wall_charge_blow(g, Some(c), None); return WallChargeIssue::Arm1(()); }
    let p = match wall_charge_plan(g, &w, men, None) { WallChargePlan::Text(s) => return WallChargeIssue::Text(s), WallChargePlan::Arm1(p) => p };
    for x in g.wall_charge.ops.clone() {
        o!(g, x).men.retain(|m| !men.contains(m));
        if o!(g, x).men.is_empty() { wall_charge_end(g, x, "re-ordered"); }
    }
    let mut post = JsMap::new();
    for &(gd, q) in p.deal.iter() { post.set(gd, q); }
    let mut from = JsMap::new();
    for &m in men { let q = pos_of(g, m); from.set(m, [q[0], 0.0, q[2]]); }
    let row = WallOp {
        wall: w.clone(), men: men.to_vec(), lead: p.lead, phase: Phase::Place, t0: g.time, spot: p.spot, set_t: f64::NAN, stow_off_t: None, owes: None,
        mine: JsMap::new(), post, from, hole: None, queue: Vec::new(), stage: JsMap::new(), pod: JsMap::new(), crossed: JsMap::new(), last: JsMap::new(),
        in_t: f64::NAN, beats: Vec::new(), close: if p.close { Some(true) } else { None }, repick_t: None, repick_bad: None, redealt: None, repick_n: None, repick_kept: None,
    };
    let op = g.tab.wallcharge_wall_op.push(row);
    g.wall_charge.ops.push(op);
    g.wall_charge.stats.orders += 1.0;
    let a = set_job(g, &w, p.spot);
    wall_charge_assign(g, op, p.lead, &a);
    for &gd in men { if gd != p.lead { wall_charge_stand_off(g, op, gd); } }
    let names: Vec<String> = men.iter().map(|&m| g.gd[m].callsign.clone()).collect();
    let line = format!("ordered · {} · {} sets it", names.join(", "), g.gd[p.lead].callsign);
    wall_charge_beat(g, op, &line);
    WallChargeIssue::Arm1(())
}

/// the order by code: null when given, else why not   (wallcharge.ts WallCharge.order)
pub fn wall_charge_order(g: &mut crate::game::Game, at: V3, who: Option<crate::wallcharge::WallChargeOrderWho>) -> Option<String> {
    use crate::orders::SquadOrdersGiveWho as W;
    let t = crate::orders::squad_orders_target_at(g, at);
    let who = who.map(|x| match x { WallChargeOrderWho::Num(n) => W::Num(n), WallChargeOrderWho::All => W::All, WallChargeOrderWho::Red => W::Red, WallChargeOrderWho::Blue => W::Blue });
    crate::orders::squad_orders_give(g, WALL_ORDER, Some(&t), who, None)
}

/// the order is over: off the list, nobody left with his back to the wall, onto the done list   (wallcharge.ts WallCharge.end)
pub fn wall_charge_end(g: &mut crate::game::Game, op: WallOpId, why: &str) {
    let i = match g.wall_charge.ops.iter().position(|&x| x == op) { Some(i) => i, None => return };
    g.wall_charge.ops.remove(i);
    let line = format!("end: {}", why);
    wall_charge_beat(g, op, &line);
    let ph = o!(g, op).phase;
    if ph == Phase::Place || ph == Phase::Standoff || ph == Phase::Armed {
        let wall = o!(g, op).wall.clone();
        let height = g.k.wallcharge.height;
        let watch = wall_pt(g, &wall, 0.0, 0.0, Some(height));
        for gd in o!(g, op).men.clone() {
            let a = match o!(g, op).mine.get(gd).copied() { Some(a) => a, None => continue };
            if crate::orders::squad_orders_of(g, gd) != Some(a) { continue; }
            let r = &mut asg!(g, a);
            if r.going == "standing off" && !r.spare.unwrap_or(false) && r.watch.is_none() {
                if let Some(post) = r.post { r.face = Some(v3::yaw_to(post, watch)); r.watch = Some(watch); }
            }
        }
    }
    let item = done_item(&o!(g, op), why);
    g.wall_charge.done.push(item);
    if g.wall_charge.done.len() > 12 { g.wall_charge.done.remove(0); }
}

/// `Object.assign(op, { ended })`: the done list's copy of the order
fn done_item(r: &WallOp, why: &str) -> WallChargeDoneItem {
    let mut stage = JsMap::new();
    for (k, v) in r.stage.iter() { stage.set(k, match *v { WallOpStageValue::Lane => WallChargeDoneItemStageValue::Lane, WallOpStageValue::Pod => WallChargeDoneItemStageValue::Pod }); }
    WallChargeDoneItem {
        wall: r.wall.clone(), men: r.men.clone(), lead: r.lead, phase: r.phase, t0: r.t0, spot: r.spot, set_t: r.set_t, stow_off_t: r.stow_off_t, owes: r.owes,
        mine: r.mine.clone(), post: r.post.clone(), from: r.from.clone(), hole: r.hole, queue: r.queue.clone(), stage, pod: r.pod.clone(), crossed: r.crossed.clone(),
        last: r.last.clone(), in_t: r.in_t, beats: r.beats.clone(), close: r.close, repick_t: r.repick_t, repick_bad: r.repick_bad.clone(), redealt: r.redealt.clone(),
        repick_n: r.repick_n, repick_kept: r.repick_kept.clone(), ended: String::from(why),
    }
}

/// (the walk: never deeper into the blast than where he stands now)   (wallcharge.ts repick's safeWalk)
fn safe_walk(g: &mut Game, w: &WallSpot, gd: GuardId, q: V3) -> bool {
    let p0 = pos_of(g, gd);
    let path = match crate::scene_collision::nav_grid_find_path(g, p0, q, Some(4000.0), None) { Some(p) => p, None => return false };
    let lim = wall_charge_blast_share(g, w, p0, None, None) + 0.05;
    let mut a = p0;
    for b in path {
        let n = m::max(1.0, m::ceil(v3::dist_xz(a, b) / 0.5));
        let mut i = 1.0;
        while i <= n {
            if wall_charge_blast_share(g, w, v3::lerp3(a, b, i / n), None, None) > lim { return false; }
            i += 1.0;
        }
        a = b;
    }
    true
}

/// a stand-off mark that has come under known fire is dealt again, once a man   (wallcharge.ts WallCharge.repick)
pub fn wall_charge_repick(g: &mut crate::game::Game, op: WallOpId) {
    let t = g.time;
    if o!(g, op).close.unwrap_or(false) { return; }
    let rt = o!(g, op).repick_t.unwrap_or(0.0);
    if t < rt && t >= rt - 3.0 { return; }
    o!(g, op).repick_t = Some(t + 1.0);
    if o!(g, op).redealt.is_none() { o!(g, op).redealt = Some(JsSet::new()); }
    let was = o!(g, op).repick_bad.clone().unwrap_or_else(JsSet::new);
    let men = o!(g, op).men.clone();
    let open: Vec<GuardId> = men.iter().copied().filter(|&gd| {
        let r = &o!(g, op);
        !r.redealt.as_ref().unwrap().has(gd) && r.post.get(gd).is_some()
    }).collect();
    if open.is_empty() || o!(g, op).repick_n.unwrap_or(0.0) >= 4.0 { o!(g, op).repick_t = Some(t + 3.0); return; }
    let w = o!(g, op).wall.clone();
    let mut fire = wall_charge_under_fire(g, Some(&w), None);
    let mut hot = JsSet::new();
    for &gd in open.iter() {
        let q = *o!(g, op).post.get(gd).unwrap();
        if fire_any(g, &mut fire.score, q) { hot.add(gd); }
    }
    let mut bad: Vec<GuardId> = hot.keys().into_iter().filter(|&gd| was.has(gd)).collect();
    o!(g, op).repick_bad = Some(hot);
    if bad.is_empty() { return; }
    if o!(g, op).repick_kept.is_none() { o!(g, op).repick_kept = Some(JsMap::new()); }
    let n = men.len() as f64;
    let spot = o!(g, op).spot;
    let posts = match seam_safe_posts_kept(g, op, &w, n + 2.0, spot) { Some(p) => Some(p), None => seam_safe_posts_kept(g, op, &w, n, spot) };
    let mut moved = 0;
    o!(g, op).repick_n = Some(o!(g, op).repick_n.unwrap_or(0.0) + 1.0);
    if let Some(posts) = posts {
        js::sort_by(&mut bad, |&a, &b| {
            let (qa, qb) = (*o!(g, op).post.get(a).unwrap(), *o!(g, op).post.get(b).unwrap());
            let sb = fire_score(g, &mut fire.score, qb);
            sb - fire_score(g, &mut fire.score, qa)
        });
        for gd in bad {   // (the worst off first; a place another man has, or has just been dealt, is his)
            let cur = *o!(g, op).post.get(gd).unwrap();
            let now = fire_score(g, &mut fire.score, cur);
            let held: Vec<V3> = o!(g, op).men.iter().filter(|&&m| m != gd).filter_map(|&m| o!(g, op).post.get(m).copied()).collect();
            let mut can: Vec<V3> = Vec::new();
            for &q in posts.iter() {
                let f = fire_score(g, &mut fire.score, q);
                if (f == 0.0 || f <= now - 2.0) && !held.iter().any(|&k| v3::dist_xz(k, q) < 0.8) { can.push(q); }
            }
            let me = pos_of(g, gd);
            js::sort_by(&mut can, |&a, &b| {
                let by = fire_score(g, &mut fire.score, a) - fire_score(g, &mut fire.score, b);
                js::or(by, v3::dist_xz(a, me) - v3::dist_xz(b, me))
            });
            let mut pick = None;
            for q in can {
                if crate::orders::squad_orders_reach(g, gd, q).is_none() && safe_walk(g, &w, gd, q) { pick = Some(q); break; }
            }
            let q = match pick { Some(q) => q, None => continue };
            o!(g, op).post.set(gd, q);
            o!(g, op).redealt.as_mut().unwrap().add(gd);
            let standing = match o!(g, op).mine.get(gd).copied() { Some(a) => asg!(g, a).going == "standing off", None => false };
            if standing { wall_charge_stand_off(g, op, gd); }
            moved += 1;
            let line = format!("{} dealt again: his mark came under known fire", g.gd[gd].callsign);
            wall_charge_beat(g, op, &line);
        }
    }
    if moved != 0 { g.stats.picture.stepped += moved; } else { o!(g, op).repick_t = Some(t + 3.0); }
}

/// rear security while they stand off: the man furthest from the charge has his eyes spare   (wallcharge.ts WallCharge.spareRear)
pub fn wall_charge_spare_rear(g: &mut crate::game::Game, op: WallOpId) {
    let w = o!(g, op).wall.clone();
    let at = wall_pt(g, &w, 0.0, 0.0, None);
    let live: Vec<(GuardId, OrdersAssignmentId)> = o!(g, op).men.iter().filter_map(|&m| {
        let a = o!(g, op).mine.get(m).copied()?;
        if asg!(g, a).post.is_some() { Some((m, a)) } else { None }
    }).collect();
    for &(_, a) in live.iter() { asg!(g, a).spare = Some(false); }
    if live.len() < 2 { return; }
    let mut rows: Vec<(usize, f64, GuardId, OrdersAssignmentId)> = live.iter().enumerate().map(|(i, &(m, a))| (i, v3::dist_xz(asg!(g, a).post.unwrap(), at), m, a)).collect();
    js::sort_by(&mut rows, |x, y| js::or(y.1 - x.1, y.0 as f64 - x.0 as f64));
    let take = if live.len() >= 4 { 2 } else { 1 };
    for &(_, _, m, a) in rows.iter().take(take) {
        asg!(g, a).spare = Some(true);
        if asg!(g, a).from.is_none() {
            let anchor = asg!(g, a).anchor;
            let from = o!(g, op).from.get(m).copied().unwrap_or([anchor[0], 0.0, anchor[2]]);
            asg!(g, a).from = Some(from);
        }
    }
}

/// is a walk to the marks still worth waiting on   (wallcharge.ts WallCharge.waiting)
pub fn wall_charge_waiting(g: &mut crate::game::Game, op: WallOpId, el: f64) -> bool {
    let (cap, still) = (g.k.doortools.stack_cap, g.k.doortools.still_secs);
    if el >= cap { return false; }
    el < still || o!(g, op).men.iter().any(|&gd| {
        let a = *o!(g, op).mine.get(gd).unwrap();
        !asg!(g, a).arrived && g.time - asg!(g, a).moved_t < still
    })
}

// ------------------------------------------------------------ Sam's key, the frame
/// the set charge nearest Sam (null: none)   (wallcharge.ts WallCharge.nearest)
pub fn wall_charge_nearest(g: &mut crate::game::Game, of: Option<&mut dyn FnMut(&mut crate::game::Game, WallChargeId) -> bool>) -> Option<WallChargeId> {
    let sam = g.ch[g.pl[g.player].ch].pos;
    let mut list: Vec<WallChargeId> = Vec::new();
    match of {
        Some(f) => for c in g.wall_charge.set.clone() { if f(g, c) { list.push(c); } },
        None => list = g.wall_charge.set.clone(),
    }
    let mut rows: Vec<(WallChargeId, f64)> = Vec::new();
    for &c in list.iter() {
        let wall = g.tab.wallcharge_set_wall_charge[c].wall.clone();
        let foot = wall_pt(g, &wall, 0.0, 0.0, None);
        rows.push((c, v3::dist_xz(foot, sam)));
    }
    js::sort_by(&mut rows, |a, b| a.1 - b.1);
    rows.first().map(|r| r.0)
}

/// Sam's trigger: the nearest set charge goes; with door orders under way the press is also their word   (wallcharge.ts WallCharge.input)
pub fn wall_charge_input(g: &mut crate::game::Game, input: InputId) {
    g.wall_charge.pressed = false;
    if !wall_charge_sync(g) || g.wall_charge.set.is_empty() { return; }
    let key = g.k.doortools.blow_key.clone();
    if !crate::input::input_hit(g, input, &key) { return; }
    if g.pl[g.player].down {
        let mut any = false;
        for c in g.wall_charge.set.clone() { if wall_charge_key_is(g, c) { any = true; break; } }
        if any { g.tab.input[input].pressed.delete(key.clone()); }
        return;
    }
    if let Some(rows) = g.orders.menu.as_ref().map(|mn| mn.rows.clone()) {   // (a menu up with a row on this key: the key IS that row)
        if rows.iter().any(|&d| g.tab.orders_order_def[d].hot_key.as_deref() == Some(key.as_str())) { return; }
    }
    let sam = g.ch[g.pl[g.player].ch].pos;
    let c = match wall_charge_nearest(g, Some(&mut |g: &mut Game, x: WallChargeId| wall_charge_key_is(g, x))) { Some(c) => c, None => return };
    let wall = g.tab.wallcharge_set_wall_charge[c].wall.clone();
    let foot = wall_pt(g, &wall, 0.0, 0.0, None);
    let mine = v3::dist_xz(foot, sam);
    for x in g.door_tools.set.clone() {
        if crate::doortools::door_tools_key_is(g, x) {
            let d = g.tab.doortools_set_charge[x].door;
            if v3::dist_xz(g.dr[d].frame_centre, sam) < mine { return; }
        }
    }
    wall_charge_blow(g, Some(c), None);
    let mut word = false;
    for x in g.door_tools.set.clone() { if crate::doortools::door_tools_key_is(g, x) { word = true; break; } }
    if word { g.tab.input[input].pressed.delete(key.clone()); } else { g.wall_charge.pressed = true; }   // (men waiting at a door get the word on this same press)
}

/// after the door tools' input: a press that blew a wall and was nobody's word besides is used up   (wallcharge.ts WallCharge.inputAfter)
pub fn wall_charge_input_after(g: &mut crate::game::Game, input: InputId) {
    if g.wall_charge.pressed {
        g.wall_charge.pressed = false;
        let key = g.k.doortools.blow_key.clone();
        g.tab.input[input].pressed.delete(key.clone());
    }
}

/// the module's frame: Sam's own charge, the set charges, the orders   (wallcharge.ts WallCharge.tick)
pub fn wall_charge_tick(g: &mut crate::game::Game) {
    if !wall_charge_sync(g) { return; }
    wall_charge_sam_tick(g);
    for c in g.wall_charge.set.clone() {
        let w = g.tab.wallcharge_set_wall_charge[c].wall.clone();
        if wall_charge_open_at(g, &w) {
            match g.wall_charge.set.iter().position(|&x| x == c) { Some(i) => { g.wall_charge.set.remove(i); } None => { g.wall_charge.set.pop(); } }   // (splice(-1, 1) takes the last)
            wall_charge_give_back(g, c, "the wall came open under it");
        } else { wall_charge_draw(g, c); }
    }
    for op in g.wall_charge.ops.clone() { wall_charge_step(g, op); }
    if let Some(last) = g.wall_charge.done.last() { if g.time < last.t0 { g.wall_charge.done.clear(); } }   // (the clock went back: a load)
}

/// one order, one frame: the switch on its phase   (wallcharge.ts WallCharge.step)
pub fn wall_charge_step(g: &mut crate::game::Game, op: WallOpId) {
    let t = g.time;
    let el = t - o!(g, op).t0;
    let w = o!(g, op).wall.clone();
    let mut men = Vec::new();
    for gd in o!(g, op).men.clone() {   // (down, or given another order: not this order's any more)
        if g.gd[gd].state != crate::game::GuardState::Dead && still_mine(g, op, gd) { men.push(gd); }
    }
    o!(g, op).men = men.clone();
    if men.is_empty() { wall_charge_end(g, op, "nobody left on it"); return; }
    if !men.contains(&o!(g, op).lead) {   // (the setter is down, or given another order: the man nearest the charge's place has it)
        let spot = o!(g, op).spot;
        let mut rows: Vec<(GuardId, f64)> = men.iter().map(|&m| (m, v3::dist_xz(pos_of(g, m), spot))).collect();
        js::sort_by(&mut rows, |a, b| a.1 - b.1);
        let lead = rows[0].0;
        o!(g, op).lead = lead;
        if o!(g, op).phase == Phase::Place {
            o!(g, op).set_t = f64::NAN;
            o!(g, op).t0 = t;
            crate::game::game_say(g, lead, "i've got the charge.", None, None, None);
            let a = set_job(g, &w, spot);
            wall_charge_assign(g, op, lead, &a);
            let line = format!("setter now {}", g.gd[lead].callsign);
            wall_charge_beat(g, op, &line);
        }
    }
    let lead = o!(g, op).lead;
    let ph = o!(g, op).phase;
    if ph == Phase::Standoff || ph == Phase::Armed { wall_charge_repick(g, op); wall_charge_spare_rear(g, op); }   // (set again every frame)
    match ph {
        Phase::Place => step_place(g, op, &w, lead, t, el),
        Phase::Standoff => step_standoff(g, op, &w, lead, t, el),
        Phase::Armed => { if wall_charge_charge_on(g, &w).is_none() { wall_charge_end(g, op, "the charge is gone"); } }   // (blow() moves the order on itself)
        Phase::Wait => { if el >= wall_charge_blow_wait(g, op) { wall_charge_enter(g, op); } }
        Phase::Enter => step_enter(g, op, &w, t, el),
    }
}

/// he is at the wall for setSecs, then it is on the wall and he walks off to the stand-off   (wallcharge.ts step, 'place')
fn step_place(g: &mut Game, op: WallOpId, w: &WallSpot, lead: GuardId, t: f64, el: f64) {
    let height = g.k.wallcharge.height;
    let a = *o!(g, op).mine.get(lead).unwrap();
    let near = v3::dist_xz(pos_of(g, lead), o!(g, op).spot) < g.k.wallcharge.set_near;
    let mut stow = crate::workstow::StowFightOp { stow_off_t: o!(g, op).stow_off_t };   // (lent: stowFight writes op.stowOffT)
    let yield_t = asg!(g, a).yield_t;
    let fought = crate::workstow::stow_fight(g, lead, Some(crate::workstow::StowFightA { yield_t }), &mut stow);
    o!(g, op).stow_off_t = stow.stow_off_t;
    if fought { o!(g, op).set_t = f64::NAN; o!(g, op).t0 = t; return; }   // (the fight has him: not set, nothing spent)
    if o!(g, op).set_t.is_nan() {
        if asg!(g, a).arrived && near {
            o!(g, op).set_t = t + crate::workstow::mate_stow_lead(g);
        } else if !wall_charge_waiting(g, op, el) {
            crate::game::game_say(g, lead, "can't get on that wall.", None, None, None);
            wall_charge_end(g, op, "never made the wall");
        }
        return;
    }
    if !near { o!(g, op).set_t = f64::NAN; return; }   // (walked off it: not set)
    crate::workstow::mate_stow(g, lead);
    let (set_t, secs) = (o!(g, op).set_t, g.k.doortools.set_secs);
    if t >= set_t {
        let c = g.gd[lead].ch;
        let at = wall_pt(g, w, 0.0, 0.0, Some(height));
        crate::handwork::charge_hands(g, c, at, wall_n(w), t - set_t, secs, None);
    }
    if t - set_t < secs { return; }
    if wall_charge_open_at(g, w) {
        crate::game::game_say(g, lead, "it's open.", None, None, None);
        wall_charge_end(g, op, "the wall opened");
        return;
    }
    if wall_charge_charge_on(g, w).is_none() {
        let paid = wall_charge_take(g);
        if paid < 0.0 {
            let none = wall_charge_none_words(g);
            crate::game::game_say(g, lead, &none, None, None, None);
            wall_charge_end(g, op, "no charges");
            return;
        }
        let row = SetWallCharge { wall: w.clone(), t, by: g.gd[lead].callsign.clone(), paid, free: if g.wall_charge.free_set { Some(true) } else { None } };
        let c = g.tab.wallcharge_set_wall_charge.push(row);
        g.wall_charge.set.push(c);
        let sound_at = wall_pt(g, w, 0.0, 0.0, Some(height));
        crate::audio_iface::game_audio_like_play(g, "lockPick", Some(sound_at), Some(0.6), None);
        crate::furnblast::furniture_warm(g);
    }
    o!(g, op).owes = Some(true);   // (his word is what sends a thumb to the trigger: he says it once he is clear of it)
    wall_charge_beat(g, op, "set");
    wall_charge_stand_off(g, op, lead);
    o!(g, op).phase = Phase::Standoff;
    o!(g, op).t0 = t;
}

/// the men walk to their marks; then the call   (wallcharge.ts step, 'standoff')
fn step_standoff(g: &mut Game, op: WallOpId, w: &WallSpot, lead: GuardId, t: f64, el: f64) {
    if wall_charge_charge_on(g, w).is_none() { wall_charge_end(g, op, "the charge is gone"); return; }
    if o!(g, op).owes.unwrap_or(false) && v3::dist_xz(pos_of(g, lead), wall_pt(g, w, 0.0, 0.0, None)) >= g.k.wallkill.near_m {
        o!(g, op).owes = Some(false);
        crate::game::game_say(g, lead, "charge set.", None, None, None);
    }
    let men = o!(g, op).men.clone();
    let all_there = men.iter().all(|&gd| { let a = *o!(g, op).mine.get(gd).unwrap(); asg!(g, a).arrived });
    if !all_there && wall_charge_waiting(g, op, el) { return; }
    let mut inside = false;
    for &m in men.iter() {
        let p = pos_of(g, m);
        if wall_charge_blast_share(g, w, p, None, Some(false)) > 0.0 { inside = true; break; }
    }
    crate::game::game_say(g, lead, if inside { "can't get clear of it. your call." } else { "stood off. your call." }, None, None, None);
    let line = format!("charge set on the wall. press {} to blow it", g.k.doortools.blow_key.replacen("Key", "", 1));
    crate::game::game_msg(g, &line);
    o!(g, op).phase = Phase::Armed;
    o!(g, op).t0 = t;
    wall_charge_beat(g, op, "armed");
}

/// the job of a man going in: gun up, light, searching
fn entry_job(g: &Game, post: V3, face: f64, watch: Option<V3>, light: bool, there: &str) -> WallChargeAssignA {
    let a = none_aim(job("entering · the wall", there));
    WallChargeAssignA { post: Some(post), face: Some(face), watch, pace: Some(g.orders.knobs.entry), light: Some(light), search: Some(true), ..a }
}

/// through the hole, each to his own point; then the room is theirs   (wallcharge.ts step, 'enter')
fn step_enter(g: &mut Game, op: WallOpId, w: &WallSpot, t: f64, el: f64) {
    let tk = thick(g, w);
    let light = g.door_tools.lights.unwrap_or(true);
    let into = v3::yaw_to(wall_pt(g, w, 0.0, 0.0, None), wall_pt(g, w, 0.0, -1.0, None));
    let mut i = o!(g, op).queue.len();
    while i > 0 {
        i -= 1;
        let q = o!(g, op).queue[i].clone();
        if t < q.at { continue; }
        o!(g, op).queue.remove(i);
        if o!(g, op).men.contains(&q.gd) {
            o!(g, op).pod.set(q.gd, q.pod.clone());
            o!(g, op).stage.set(q.gd, WallOpStageValue::Lane);
            let lw = g.k.wallcharge.lane_watch;
            let watch = if lw > 0.0 { Some(wall_pt(g, w, o!(g, op).hole.unwrap().du, -(tk + lw), Some(g.k.wallcharge.height))) } else { None };
            let a = entry_job(g, q.lane, into, watch, light, "entering · the wall");
            wall_charge_assign(g, op, q.gd, &a);
        }
    }
    let mut settled = o!(g, op).queue.is_empty();
    let men = o!(g, op).men.clone();
    for &gd in men.iter() {
        let st = o!(g, op).stage.get(gd).copied();
        let st = match st {
            Some(s) => s,
            None => {
                if !o!(g, op).pod.has(gd) && !o!(g, op).queue.iter().any(|q| q.gd == gd) { continue; }
                settled = false;
                continue;
            }
        };
        let a = *o!(g, op).mine.get(gd).unwrap();
        let p = pos_of(g, gd);
        asg!(g, a).search_t = Some(t);
        wall_charge_cross(g, op, gd, p);
        let (arrived, yield_t, arr_t, post) = { let r = &asg!(g, a); (r.arrived, r.yield_t, r.arr_t, r.post) };
        if st == WallOpStageValue::Lane {
            settled = false;
            if wall_off(g, w, p) < -(tk + 0.15) || arrived || post.map_or(false, |q| v3::dist_xz(p, q) < 0.35) {   // through the hole: on to his own point
                let pod = o!(g, op).pod.get(gd).unwrap().clone();
                if !o!(g, op).crossed.has(gd) { let along = wall_along(w, p); o!(g, op).crossed.set(gd, along); }
                o!(g, op).stage.set(gd, WallOpStageValue::Pod);
                let there = if pod.watch.is_some() && !word_on(&pod.door) { "holding the hole" } else { "holding the room" };
                let job = entry_job(g, pod.pos, pod.face, pod.watch, light, there);
                wall_charge_assign(g, op, gd, &job);
            }
        } else if !arrived && yield_t < 0.0 {   // (a man the fight has is not waited for, nor one who has taken up his door)
            if !o!(g, op).pod.get(gd).map_or(false, |x| x.took.unwrap_or(false)) { settled = false; }
        } else if arrived && yield_t < 0.0 && t - arr_t >= g.k.wallcharge.hold_secs {   // (on his point, the room looked over)
            wall_charge_take_hold(g, op, gd, Some(light));
        }
    }
    if settled && o!(g, op).in_t.is_nan() { o!(g, op).in_t = t; }
    if (settled && t - o!(g, op).in_t >= g.k.wallcharge.sweep_secs) || el >= g.k.wallcharge.enter_cap {
        for gd in o!(g, op).men.clone() {
            if o!(g, op).stage.get(gd).copied() == Some(WallOpStageValue::Pod) { wall_charge_take_hold(g, op, gd, Some(light)); }
        }
        wall_charge_beat(g, op, if settled { "in" } else { "in (cap)" });
        g.wall_charge.stats.entries += 1.0;
        wall_charge_end(g, op, "in");
    }
}

/// a man on his point inside turns to what he was dealt to hold from it, once   (wallcharge.ts WallCharge.takeHold)
pub fn wall_charge_take_hold(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, light: Option<bool>) {
    let pod = match o!(g, op).pod.get(gd) { Some(x) => x.clone(), None => return };
    let h = match pod.hold.clone() { Some(h) => h, None => return };
    o!(g, op).pod.get_mut(gd).unwrap().hold = None;
    // (one man always keeps the room: with nobody left alive in it who has no door or hole of his own, this man's eyes stay on the room)
    let keeper = o!(g, op).men.iter().any(|&x| {
        let r = &o!(g, op);
        let q = r.pod.get(x).or_else(|| r.queue.iter().find(|y| y.gd == x).map(|y| &y.pod));
        x != gd && g.gd[x].state != crate::game::GuardState::Dead && q.map_or(false, |q| q.hold.is_none() && !q.took.unwrap_or(false) && q.watch.is_none())
    });
    if !keeper { let line = format!("keeps the room · {}", g.gd[gd].callsign); wall_charge_beat(g, op, &line); return; }
    // (from where he STANDS: the line to his door is tested again from there)
    let at = o!(g, op).mine.get(gd).copied().and_then(|a| asg!(g, a).post).unwrap_or(pod.pos);
    let moved = v3::dist_xz(at, pod.pos) > 0.3;
    let door = word_on(&h.door);
    if moved && door && crate::scene_collision::shot_blocked(g, [at[0], 1.2, at[2]], h.watch, None, None) {
        let line = format!("no line to {} from where he stands · {}", h.door.as_deref().unwrap_or(""), g.gd[gd].callsign);
        wall_charge_beat(g, op, &line);
        return;
    }
    o!(g, op).pod.get_mut(gd).unwrap().took = Some(true);
    let a = none_aim(job("entering · the wall", if door { "holding the room" } else { "holding the hole" }));
    let a = WallChargeAssignA {
        post: Some(if moved { at } else { pod.pos }),
        face: Some(if moved && door { v3::yaw_to(at, h.watch) } else { h.face }),
        watch: Some(h.watch), pace: Some(g.orders.knobs.entry), light, search: Some(true), ..a
    };
    wall_charge_assign(g, op, gd, &a);
    let line = format!("holds {} · {}", h.door.as_deref().unwrap_or("the hole"), g.gd[gd].callsign);
    wall_charge_beat(g, op, &line);
}

/// s the men stand after the blast before the order sends them in   (wallcharge.ts WallCharge.blowWait)
pub fn wall_charge_blow_wait(g: &mut crate::game::Game, op: WallOpId) -> f64 {
    let (after, now) = (g.k.wallcharge.after_blow, g.time);
    let h = match o!(g, op).hole { Some(h) => h, None => return after };
    if o!(g, op).men.iter().any(|&m| g.gd[m].dazzled_until > now) { return after; }   // (a man the blast caught: the beat as it was)
    let wall = o!(g, op).wall.clone();
    let at = wall_pt(g, &wall, h.du, 0.0, None);
    let mut near = f64::INFINITY;
    for &gd in o!(g, op).men.iter() { near = m::min(near, v3::dist_xz(pos_of(g, gd), at)); }
    m::max(0.0, after - near / g.k.pace.max)
}

fn nav_free(g: &mut Game, q: V3) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) }
fn pod_has(p: &Pod) -> bool { p.watch.is_some() || p.hold.is_some() }

/// behind the blast: the men nearest the hole first, each through its far mouth to a point of his own inside   (wallcharge.ts WallCharge.enter)
pub fn wall_charge_enter(g: &mut crate::game::Game, op: WallOpId) {
    let w = o!(g, op).wall.clone();
    let h = o!(g, op).hole.unwrap();
    let tk = thick(g, &w);
    let t = g.time;
    let mouth_in = g.k.wallcharge.mouth_in;
    let mouth = wall_pt(g, &w, h.du, -(tk + mouth_in), None);
    let mut by_near: Vec<(GuardId, f64)> = o!(g, op).men.iter().map(|&m| (m, v3::dist_xz(pos_of(g, m), mouth))).collect();
    js::sort_by(&mut by_near, |a, b| a.1 - b.1);
    let sorted: Vec<GuardId> = by_near.iter().map(|r| r.0).collect();
    let men = crate::gunroles::gun_last(g, &sorted);
    let mut pods: Vec<Pod> = Vec::new();
    let into = v3::yaw_to(wall_pt(g, &w, 0.0, 0.0, None), wall_pt(g, &w, 0.0, -1.0, None));
    let off_frame = g.k.wallcharge.off_frame_pod;
    let table = g.k.wallcharge.pods.clone();
    for i in 0..men.len() {
        let [lat, deep] = table[i % table.len()];
        let mut got: Option<V3> = None;
        'search: for strict in [true, false] {   // (a point in one of the room's doorways only when the room has no other floor for him)
            for k in [1.0, 0.6, 0.3, -1.0, -0.6, 0.0] {
                for dd in [0.0, 0.6, -0.5, 1.2] {
                    let q = wall_pt(g, &w, h.du + lat * k, -(tk + m::max(0.8, deep + dd)), None);
                    if nav_free(g, q) && !crate::scene_collision::solid_blocked(g, [mouth[0], 0.45, mouth[2]], [q[0], 0.45, q[2]], None)
                        && !(strict && wall_charge_in_frame(g, q, Some(off_frame)))
                        && !pods.iter().any(|x| v3::dist_xz(x.pos, q) < 0.7) { got = Some(q); break 'search; }
                }
            }
        }
        if let Some(q) = got {
            let face = wall_charge_look_from(g, q, into, v3::yaw_to(mouth, q));
            pods.push(Pod { pos: q, face, watch: None, door: None, hold: None, took: None });
        }
    }
    if pods.is_empty() {
        let lead = o!(g, op).lead;
        crate::game::game_say(g, lead, "no way in.", None, None, None);
        wall_charge_end(g, op, "no floor inside");
        return;
    }
    let n_in = pods.len().min(men.len());
    let in_men: Vec<GuardId> = men[..n_in].to_vec();
    let stay: Vec<GuardId> = men[n_in..].to_vec();
    if !stay.is_empty() {   // (no floor for him inside: he keeps the stand-off, and is not this order's any more)
        o!(g, op).men.retain(|m| !stay.contains(m));
        let names: Vec<String> = stay.iter().map(|&m| g.gd[m].callsign.clone()).collect();
        let line = format!("outside · {}", names.join(", "));
        wall_charge_beat(g, op, &line);
    }
    let mut deal: JsMap<GuardId, Pod> = JsMap::new();
    let mut k = 0;
    while k < in_men.len() {
        let end = (k + 2).min(in_men.len());
        let pend = (k + 2).min(pods.len());
        let sp = &pods[k..pend];
        let spots: Vec<V3> = sp.iter().map(|x| x.pos).collect();
        for (gd, pos) in crate::orders::squad_orders_pair(g, &in_men[k..end], &spots, None, None) {
            deal.set(gd, sp.iter().find(|x| x.pos == pos).unwrap().clone());
        }
        k += 2;
    }
    let half = m::max(0.0, m::min(g.k.wallcharge.lane, h.half - 0.4));
    let second = g.k.doortools.second;
    let mut queue: Vec<WallChargeDoneItemQueueItem> = Vec::new();
    for (i, &gd) in in_men.iter().enumerate() {
        let pod = deal.get(gd).unwrap_or(&pods[0]).clone();
        let sg = js::or(m::sign(wall_along(&w, pod.pos) - h.du), if i % 2 != 0 { 1.0 } else { -1.0 });
        let mut lane = wall_pt(g, &w, h.du + sg * half, -(tk + mouth_in), None);
        if !nav_free(g, lane) { lane = if nav_free(g, mouth) { mouth } else { pod.pos }; }
        queue.push(WallChargeDoneItemQueueItem { gd, at: t + i as f64 * second, lane, pod });
    }
    // (the room's own doors: each, nearest the hole first, is one man's to hold from his point. A man with no door keeps the room ahead of him, and one man always does)
    let free2: Vec<usize> = (0..queue.len()).filter(|&i| !pod_has(&queue[i].pod)).collect();
    struct Way { name: String, p: V3, back: bool }
    let mut ins: Vec<Way> = Vec::new();
    if let Some(room) = crate::roomfight::room_fight_room_at(g, mouth) {
        for e in g.tab.roomfight_room[room].entries.clone() {
            let p = [e.at[0] - e.out[0] * 0.5, 1.2, e.at[2] - e.out[1] * 0.5];
            let sq = m::abs(if w.along_x { e.out[1] } else { e.out[0] }) > 0.7;
            if wall_off(g, &w, p) < -tk { ins.push(Way { name: g.dr[e.door].def.name.clone(), p, back: sq && wall_off(g, &w, p) > -(tk + 1.2) }); }
        }
    }
    js::sort_by(&mut ins, |a, b| js::or((a.back as i32 - b.back as i32) as f64, v3::dist_xz(a.p, mouth) - v3::dist_xz(b.p, mouth)));
    let mate_line = g.k.wallcharge.mate_line;
    for e in ins.iter() {
        if free2.iter().filter(|&&i| !pod_has(&queue[i].pod)).count() <= 1 { break; }   // (the last man free keeps the room)
        let mut pick: Option<usize> = None;
        let mut bd = f64::INFINITY;
        for &i in free2.iter() {
            let p = queue[i].pod.pos;
            let d = v3::dist_xz(p, e.p);
            if pod_has(&queue[i].pod) || d < 0.8 || d >= bd || crate::scene_collision::shot_blocked(g, [p[0], 1.2, p[2]], e.p, None, None)
                || queue.iter().enumerate().any(|(j, x)| j != i && near_seg(x.pod.pos, p, e.p) < mate_line) { continue; }   // (a line of FIRE: a mate's point on it is a mate down his muzzle)
            pick = Some(i);
            bd = d;
        }
        if let Some(i) = pick {
            let face = v3::yaw_to(queue[i].pod.pos, e.p);
            queue[i].pod.hold = Some(PodHold { face, watch: e.p, door: Some(e.name.clone()) });
            let line = format!("{} · {}", e.name, g.gd[queue[i].gd].callsign);
            wall_charge_beat(g, op, &line);
        }
    }
    o!(g, op).queue = queue;
    o!(g, op).phase = Phase::Enter;
    o!(g, op).t0 = t;
    wall_charge_beat(g, op, "go");
    let lead = o!(g, op).lead;
    crate::game::game_say(g, lead, "going in.", None, None, None);
}

/// where he crossed the wall's line; read by the checks and their words only   (wallcharge.ts WallCharge.cross)
pub fn wall_charge_cross(g: &mut crate::game::Game, op: WallOpId, gd: GuardId, p: V3) {
    let was = o!(g, op).last.get(gd).copied();
    if was == Some(None) { return; }
    let w = o!(g, op).wall.clone();
    let mid = -thick(g, &w) / 2.0;
    let o1 = wall_off(g, &w, p);
    let was = match was {
        Some(Some(v)) => v,
        _ => { o!(g, op).last.set(gd, if o1 < mid { None } else { Some(p) }); return; }   // (first seen already past it: the entry's own sample is his)
    };
    let o0 = wall_off(g, &w, was);
    if o1 < mid {
        let at = v3::lerp(wall_along(&w, was), wall_along(&w, p), (o0 - mid) / (o0 - o1));
        o!(g, op).crossed.set(gd, at);
        o!(g, op).last.set(gd, None);
        return;
    }
    o!(g, op).last.set(gd, Some(p));
}

/// how far a line runs open from `p` at `yaw`, up to 6 m   (lookFrom's `open`)
fn open_way(g: &mut Game, p: V3, yaw: f64) -> f64 {
    let (s, c) = (m::sin(yaw), m::cos(yaw));
    let mut d = 0.0;
    while d < 6.0 && !crate::scene_collision::solid_blocked(g, [p[0], 1.2, p[2]], [p[0] + s * (d + 1.0), 1.2, p[2] + c * (d + 1.0)], None) { d += 1.0; }
    d
}

/// the way a man on his point inside faces: the way he came unless that is a wall at his chest   (wallcharge.ts WallCharge.lookFrom)
pub fn wall_charge_look_from(g: &mut crate::game::Game, p: V3, into: f64, want: f64) -> f64 {
    if open_way(g, p, want) >= 2.0 { return want; }
    let mut best = want;
    let mut score = f64::NEG_INFINITY;
    let mut a = -90.0;
    while a <= 90.0 {
        let yaw = into + a * core::f64::consts::PI / 180.0;
        let sc = open_way(g, p, yaw) - 0.4 * m::abs(m::atan2(m::sin(yaw - want), m::cos(yaw - want)));
        if sc > score { score = sc; best = yaw; }
        a += 15.0;
    }
    best
}

// ------------------------------------------------------------ the blast
/// Sam's trigger on one charge: one window round the blow, the whole of it the team's doing   (wallcharge.ts WallCharge.blow)
pub fn wall_charge_blow(g: &mut crate::game::Game, c_2: Option<WallChargeId>, pressed: Option<bool>) -> Option<crate::wallcharge::WallChargeBlow> {
    let c = match c_2 { Some(c) => Some(c), None => wall_charge_nearest(g, None) };
    let pressed = pressed.unwrap_or(true);
    let who = g.ch[g.pl[g.player].ch].id;
    let r = crate::scene_collision::window(g, &mut |g: &mut Game| crate::fogofwar::fog_of_war_doing(g, Some(who), &mut |g: &mut Game| wall_charge_blow_raw(g, c, pressed)));
    r.map(|x| WallChargeBlow { hole: x.hole, knocked: x.knocked, stunned: x.stunned, backblast: x.backblast, lost: x.lost })
}

/// Sam inside the reach when it went: knocked along the blast, no round for a beat, his aim open. Never a hit   (wallcharge.ts WallCharge.samCaught)
pub fn wall_charge_sam_caught(g: &mut crate::game::Game, pl: PlayerId, k: f64, from: V3) {
    let c = g.pl[pl].ch;
    let along = v3::sub(g.ch[c].pos, from);
    crate::character::anim_hit_from(g, c, Some(along));
    g.pl[pl].hit_flash = m::max(g.pl[pl].hit_flash, k);
    let (ss, sup) = (g.k.wallcharge.sam_secs, g.k.wallcharge.sam_sup);
    g.pl[pl].fire_cd = m::max(g.pl[pl].fire_cd, v3::lerp(ss[1], ss[0], k));
    g.pl[pl].sup.v = m::max(g.pl[pl].sup.v, v3::lerp(sup, 1.0, k));
    g.pl[pl].sup.kick = m::max(g.pl[pl].sup.kick, k);
}

/// the charge's own side: blind the seconds the safety radius gives   (blowRaw's nearSecs)
fn near_secs(g: &Game, d: f64, reach: f64) -> f64 {
    let s = g.k.wallcharge.safe_secs;
    v3::lerp(s[0], s[1], v3::clamp(d / reach, 0.0, 1.0))
}
/// (blowRaw's away: the way a man at the charge is thrown)
fn away_from(p: V3, xz: V3) -> V3 { v3::normalize([p[0] - xz[0], 0.15, p[2] - xz[2]]) }

/// what the blast does to one seat's player   (blowRaw's eachSeat body)
fn blow_seat(g: &mut Game, w: &WallSpot, hole: bool, r: crate::wallcharge::WallChargeAwayMarksValueR, wk: bool, xz: V3, out: V3, inn: V3, sam: &mut f64, sam_down: &mut bool) {
    let tk = thick(g, w);
    let (knock_m, stun_m, ours_secs, near_m) = (g.k.doortools.knock_m, g.k.doortools.stun_m, g.k.wallshot.ours_secs, g.k.wallkill.near_m);
    let pl = g.player;
    if wk && !g.pl[pl].down && !g.god_mode && g.puppet.is_none() && !g.touring {
        let p = g.ch[g.pl[pl].ch].pos;
        let far = wall_off(g, w, p) < -tk / 2.0;
        let dist = v3::dist_xz(p, xz);
        if !(far && !hole) && dist < (if far { knock_m } else { near_m }) && !crate::scene_collision::solid_blocked(g, if far { inn } else { out }, [p[0], 1.1, p[2]], None) {
            crate::combat::hit_player(g, away_from(p, xz));
            g.pl[pl].hit_flash = 1.0;
            crate::chronicle::chron_hit(g, Some("the wall charge"));
            *sam_down = *sam_down || g.pl[pl].down;
        }
    }
    let k = if !g.pl[pl].down { let p = g.ch[g.pl[pl].ch].pos; wall_charge_blast_share(g, w, p, Some(r), None) } else { 0.0 };
    if k > 0.0 { wall_charge_sam_caught(g, pl, k, xz); }
    *sam = m::max(*sam, k);
    // (samstun.ts: the blast does to the player what it does to a teammate standing there)
    if !g.pl[pl].down && g.puppet.is_none() && !g.touring {
        let p = g.ch[g.pl[pl].ch].pos;
        let far = wall_off(g, w, p) < -tk / 2.0;
        let dist = v3::dist_xz(p, xz);
        if !(far && !hole) && dist < (if far { stun_m } else { r.reach }) && !crate::scene_collision::solid_blocked(g, if far { inn } else { out }, [p[0], 1.1, p[2]], None) {
            let secs = if far { ours_secs } else { near_secs(g, dist, r.reach) };
            crate::samstun::dazzle_player(g, secs, inn);
        }
    }
}

/// the blast itself: the wall, the report, who it catches, and the squad goes in behind it   (wallcharge.ts WallCharge.blowRaw)
pub fn wall_charge_blow_raw(g: &mut crate::game::Game, c_2: Option<WallChargeId>, pressed: bool) -> Option<crate::wallcharge::WallChargeBlowRaw> {
    use crate::game::{DownBy, DownKind, GuardState};
    let c = c_2?;
    let i = g.wall_charge.set.iter().position(|&x| x == c)?;
    let w = g.tab.wallcharge_set_wall_charge[c].wall.clone();
    g.wall_charge.set.remove(i);
    crate::holdfire::hold_fire_order(g, "wallCharge", Some(crate::holdfire::HoldFireOrderO { blown: Some(true), ..Default::default() }));   // (a charge starts the fight when it GOES)
    let tk = thick(g, &w);
    let at = charge_at(g, &w);
    let outer = w.outer.unwrap_or(false);
    let breach = if outer && crate::wallbreak::wall_break_adopt(g, w.index) { Some(w.index) } else { None };   // (an outer wall: this blast alone opens it)
    let hung = Some(crate::wallbreak::BlastOptsHung { wall: w.index, r: g.k.wallcharge.hung_m });
    crate::cleared::cleared_rooms_room_at(g, at);   // (the room beyond is named on the floor as it is BEFORE this hole joins two rooms)
    let who = g.ch[g.pl[g.player].ch].id;
    crate::destruction::destruction_blast(g, at, Some(crate::wallbreak::BlastOpts { who: Some(who), chaos: Some(false), breach, hung, ..Default::default() }));
    let hole = wall_charge_hole_at(g, &w);
    let du = hole.map_or(0.0, |h| h.du);
    let has_hole = hole.is_some();
    let pall = g.k.wallshot.pall;
    let base = wall_pt(g, &w, du, 0.0, Some(0.0));
    crate::destruction::destruction_pall_wall(g, base, wall_n(&w), if w.along_x { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] }, tk, has_hole,
        Some(if outer { crate::scene_materials::Surf::Concrete } else { crate::scene_materials::Surf::Drywall }), Some(pall));
    crate::floormess::mess_wall(g, base, wall_n(&w), tk, has_hole);
    let flash_at = wall_pt(g, &w, 0.0, 0.15, Some(0.5));
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, flash_at, [0.0, 0.0, 0.0],
        Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(99.0), who: Some(who), ..Default::default() }));
    crate::throwables::item_set_fired(g, it, true);
    let last = g.messages.last().map(|m| (m.text.clone(), m.t));
    let last_n = g.messages.len();
    let all = core::mem::take(&mut g.guards);
    crate::combat::detonate_stun(g, it, Some(true), None);
    g.guards = all;
    let fresh = match g.messages.last() {   // ('flashbang went off': this was not one)
        Some(m) => (g.messages.len() != last_n || last.as_ref().map_or(true, |l| l.0 != m.text || l.1 != m.t)) && m.text.contains("flashbang"),
        None => false,
    };
    if fresh { g.messages.pop(); }
    if let Some(ii) = g.items.items.iter().position(|&x| x == it) { g.items.items.remove(ii); }

    let (mut knocked, mut stunned, mut backblast, mut lost) = (0.0, 0.0, 0.0, 0.0);
    let r = safe_radius(g, &w);
    let xz = wall_pt(g, &w, 0.0, 0.0, None);
    let out = wall_pt(g, &w, 0.0, 0.5, Some(1.1));
    let inn = wall_pt(g, &w, du, -(tk + 0.45), Some(1.1));
    let room = match hole { Some(h) => room_beyond(g, &w, h.du, tk), None => 0.0 };
    let wk = pressed;
    let mut sam_down = false;
    let (knock_m, stun_m, near_m) = (g.k.doortools.knock_m, g.k.doortools.stun_m, g.k.wallkill.near_m);
    let (room_m, shot_stagger, ours_secs, ss) = (g.k.wallshot.room_m, g.k.wallshot.stagger_secs, g.k.wallshot.ours_secs, g.k.wallshot.stun_secs);
    for gd in g.guards.clone() {
        if g.gd[gd].state == GuardState::Dead || (wk && g.gd[gd].god) { continue; }
        let p = pos_of(g, gd);
        let far = wall_off(g, &w, p) < -tk / 2.0;
        let dist = v3::dist_xz(p, xz);
        let chest = [p[0], 1.1, p[2]];
        let ours = g.gd[gd].team == crate::teams::Team::Operative;
        let lined = dist <= stun_m && !crate::scene_collision::solid_blocked(g, if far { inn } else { out }, chest, None);
        let in_room = js::truthy(room) && far && !ours && dist <= room_m && crate::cleared::cleared_rooms_room_at(g, p) == room;
        if (far && !has_hole) || (!lined && !in_room) { continue; }   // (a wall that held is still a wall to the men behind it)
        if !far {   // too near the charge on its own side (at it, he is killed)
            if wk && dist < near_m {
                let line = format!("{} down — at the charge when it went", callsign_or(g, gd, "a man"));
                crate::combat::kill_guard(g, gd, away_from(p, xz), Some(false), Some(DownKind::Shot), Some(DownBy::Player), Some(&line), None);
                if !ours { knocked += 1.0; } else if !crate::rails::rails_is_spared(g, gd) { lost += 1.0; }
            } else if dist < r.reach {
                let secs = near_secs(g, dist, r.reach);
                wall_charge_dazzle(g, gd, secs, inn);
                if ours { backblast += 1.0; } else { stunned += 1.0; }
            }
            continue;
        }
        if !lined { wall_charge_shock(g, gd, shot_stagger, inn); stunned += 1.0; continue; }   // (in the room with no line to the hole: the shock, not the flash)
        if ours && (if wk { dist >= knock_m } else { crate::orders::squad_orders_big(g) }) {   // (one of ours past the wall: dazzled, not felled)
            wall_charge_dazzle(g, gd, ours_secs, inn);
            backblast += 1.0;
            continue;
        }
        if dist < knock_m {
            let line = format!("{} down — behind the wall when it went", callsign_or(g, gd, "a man"));
            crate::combat::kill_guard(g, gd, away_from(p, xz), Some(false), Some(DownKind::Struck), Some(DownBy::Player), Some(&line), None);
            if !(ours && wk) { knocked += 1.0; } else if !crate::rails::rails_is_spared(g, gd) { lost += 1.0; }
        } else {
            wall_charge_dazzle(g, gd, v3::lerp(ss[0], ss[1], v3::clamp(dist / stun_m, 0.0, 1.0)), inn);
            g.gd[gd].stagger_until = m::max(g.gd[gd].stagger_until, g.time + shot_stagger);
            let f = g.gd[gd].weapon;
            g.fa[f].burst = None;
            stunned += 1.0;
        }
    }
    // (a civilian in the building is a man on the floor too: at the charge, or behind the wall inside knockM, with the same line)
    if wk {
        for cv in crate::civilians::civilians_shown(g) {
            if g.gd[cv].state == GuardState::Dead || g.gd[cv].god { continue; }
            let p = pos_of(g, cv);
            let far = wall_off(g, &w, p) < -tk / 2.0;
            let dist = v3::dist_xz(p, xz);
            if (far && !has_hole) || dist >= (if far { knock_m } else { near_m }) || crate::scene_collision::solid_blocked(g, if far { inn } else { out }, [p[0], 1.1, p[2]], None) { continue; }
            let line = format!("{} down — {} when it went", callsign_or(g, cv, "a civilian"), if far { "behind the wall" } else { "at the charge" });
            crate::combat::kill_guard(g, cv, away_from(p, xz), Some(false), Some(if far { DownKind::Struck } else { DownKind::Shot }), Some(DownBy::Player), Some(&line), None);
            knocked += 1.0;
        }
    }
    let mut sam = 0.0;
    crate::seats::each_seat(g, &mut |g: &mut Game, _view: &mut Game| blow_seat(g, &w, has_hole, r, wk, xz, out, inn, &mut sam, &mut sam_down));   // (every player's man)
    g.wall_charge.last_blow = Some(WallChargeLastBlow { reach: r.reach, indoors: r.indoors, sam, lost, sam_down });
    if has_hole { crate::morale::morale_blast(g, inn, Some(who)); }   // (the men past the wall it opened are shaken by it)
    let st = &mut g.wall_charge.stats;
    st.blown += 1.0;
    if has_hole { st.holes += 1.0; } else { st.duds += 1.0; }
    st.knocked += knocked + lost;
    st.stunned += stunned;
    st.backblast += backblast;
    let mut said = format!("charge blown — the wall{}", if has_hole { "" } else { " · it held" });
    if knocked != 0.0 { said += &format!(" · {} down", knocked); }
    if stunned != 0.0 { said += &format!(" · {} stunned", stunned); }
    if lost != 0.0 { said += &format!(" · {} of yours down", lost); }
    if backblast != 0.0 { said += &format!(" · {} of yours too close", backblast); }
    if sam > 0.0 { said += " · you were too close"; }
    crate::game::game_msg(g, &said);
    let hole_words = match hole { Some(h) => format!("hole {:.2} m", h.half * 2.0), None => String::from("no hole") };
    let tail = if wk { format!(" lost {}{}", lost, if sam_down { " · the player down" } else { "" }) } else { String::new() };
    let note = format!("blow wall {}: {} knocked {} stunned {} backblast {}{}", w.index, hole_words, knocked, stunned, backblast, tail);
    wall_charge_note(g, &note);
    for op in g.wall_charge.ops.clone() {   // the squad that set it goes in behind it
        let ph = o!(g, op).phase;
        if !(same_wall(&o!(g, op).wall, &w) && (ph == Phase::Place || ph == Phase::Standoff || ph == Phase::Armed)) { continue; }
        let h = match hole {
            Some(h) => h,
            None => {
                let lead = o!(g, op).lead;
                let up = if g.gd[lead].state != GuardState::Dead { Some(lead) } else { o!(g, op).men.iter().copied().find(|&m| g.gd[m].state != GuardState::Dead) };
                if let Some(up) = up { crate::game::game_say(g, up, "no way through.", None, None, None); }
                wall_charge_end(g, op, "no hole");
                continue;
            }
        };
        o!(g, op).hole = Some(WallChargeDoneItemHole { du: h.du, half: h.half });
        o!(g, op).phase = Phase::Wait;
        o!(g, op).t0 = g.time;
        for gd in o!(g, op).men.clone() {
            if g.gd[gd].state == GuardState::Dead || !still_mine(g, op, gd) { continue; }
            if g.time >= g.gd[gd].dazzled_until { let c = g.gd[gd].ch; crate::character::anim_flinch(g, c); }   // (the start at the blast; then round to the wall)
            let spare = o!(g, op).mine.get(gd).copied().map_or(false, |a| asg!(g, a).spare.unwrap_or(false));
            if !spare { wall_charge_stand_off(g, op, gd); }
        }
        let line = format!("blow · hole {:.2} m · {} down {} stunned", h.half * 2.0, knocked, stunned);
        wall_charge_beat(g, op, &line);
    }
    Some(WallChargeBlowRaw { hole: has_hole, knocked, stunned, backblast, lost })
}
