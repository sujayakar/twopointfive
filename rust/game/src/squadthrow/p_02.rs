//! src/game/squadthrow.ts lines 421 to 607
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `n.toFixed(1)`: the nearer tenth, the larger magnitude on an exact tie; a negative zero prints as zero, a small negative as "-0.0"
fn to_fixed_1(x: f64) -> String {
    if x.is_nan() {
        return String::from("NaN");
    }
    if x == 0.0 {
        return String::from("0.0");
    }
    let a = m::abs(x);
    // a tenth's tie in binary is an odd number of quarters and nothing else; the formatter rounds those to even, the language up
    let q = a * 4.0;
    if m::floor(q) == q && js::rem(q, 2.0) == 1.0 {
        let n = m::floor(a * 10.0) + 1.0;
        let sign = if x < 0.0 { "-" } else { "" };
        return format!("{}{:.1}", sign, n / 10.0);
    }
    format!("{:.1}", x)
}

/// every kind of target carries `at` (the rows here are floor rows; squad40 lends on a floor target)
fn target_at(t: &crate::orders::OrderTarget) -> V3 {
    match t {
        crate::orders::OrderTarget::Floor(x) => x.at,
        crate::orders::OrderTarget::Door(x) => x.at,
        crate::orders::OrderTarget::None(x) => x.at,
        crate::orders::OrderTarget::Foe(x) => x.at,
    }
}

/// a man's `{ ...fields of a lent order }` in another struct of the same shape (Lent.back, Walk.back, the assignment handed to `assign`)
macro_rules! back_as {
    ($b:expr, $T:path) => {{
        let b = $b;
        $T {
            post: b.post, going: b.going, order: b.order, face: b.face, there: b.there, t0: b.t0, follow: b.follow, watch: b.watch, door: b.door,
            strafe: b.strafe, stance: b.stance, upper: b.upper, stack_side: b.stack_side, move_: b.move_, set_line: b.set_line, group: b.group,
            set_said: b.set_said, goal: b.goal, arrived: b.arrived, held_set: b.held_set, held_fire: b.held_fire, arr_t: b.arr_t, moved_t: b.moved_t,
            light: b.light, low: b.low, yield_t: b.yield_t, face_now: b.face_now, search: b.search, search_t: b.search_t, via: b.via, anchor: b.anchor,
            mark: b.mark, mark_t: b.mark_t, had: b.had, back_n: b.back_n, shy: b.shy, thru: b.thru, via_for: b.via_for, via_t: b.via_t, pace: b.pace,
            spare: b.spare, from: b.from, foe: b.foe, crouch: b.crouch, lean: b.lean, fit: b.fit, tries: b.tries,
        }
    }};
}

/// a walk read as the Lent it extends (the TypeScript passes the same object)
fn walk_lent(w: &Walk) -> Lent {
    Lent { mates: w.mates.clone(), gd: w.gd, t: w.t.clone(), a: w.a, home: w.home, home_yaw: w.home_yaw, back: w.back.clone().map(|b| back_as!(b, LentBack)) }
}

/// the row of the walk (only between calls that take `g`)
fn walk_of(g: &mut Game, key: PlayerId, gd: GuardId) -> Option<&mut Walk> {
    g.m.squadthrow.states.get_mut(key).and_then(|s| s.walks.get_mut(gd))
}

fn arc_of(th: &SquadThrower) -> Arc {
    Arc { from: th.from, sol: th.sol.clone(), rest: th.rest }
}

/// The state's row, made if it is not there; its key, for a write that outlives a call taking `g`. (squadthrow.ts state)
fn state_key(g: &mut Game) -> PlayerId {
    state(g);
    crate::seats::view_key(g)
}

/// …and never his call inside what it kills at. (squadthrow.ts samNear)
pub fn sam_near(g: &mut crate::game::Game, pts: &[V3]) -> bool {
    let p = g.ch[g.pl[g.player].ch].pos;
    let min = g.k.squadthrow.duck_min_m;
    pts.iter().any(|l| v3::dist_xz(p, *l) < min)
}

/// This view's state, the row itself, made if it is not there. (squadthrow.ts state)
pub fn state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::squadthrow::State {
    let k = crate::seats::view_key(g);
    g.m.squadthrow.states.get_or_insert_with(k, || State { last: JsMap::new(), noted: false, shown: None, warn: None, warn_frag: None, caught: None, asked: JsMap::new(), walks: JsMap::new(), kept: JsMap::new(), scratch: None })
}

/// The thing leaves his hand; the count is taken already. (squadthrow.ts throwIt)
pub fn throw_it(g: &mut crate::game::Game, gd: GuardId, arc: &crate::squadthrow::Arc, kind: crate::squadthrow::SquadThrowKind, at: V3) {
    let key = state_key(g);
    let c = g.gd[gd].ch;
    let who = g.ch[c].id;
    let heading = v3::yaw_of(arc.sol.v0);
    let now = g.time;
    {
        let s = g.m.squadthrow.states.get_mut(key).unwrap();
        s.last.set(gd, now);
        s.warn = None;
        s.warn_frag = None;
    }
    let it = if kind == SquadThrowKind::Frag {
        let fuse = g.k.frag.order_fuse;
        crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Frag, arc.from, arc.sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(fuse), who: Some(who), heading: Some(heading), ..Default::default() }))
    } else {
        let fuse = crate::combat::flash_fuse(g);
        crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Flash, arc.from, arc.sol.v0, Some(crate::throwables::ThrowablesSpawnOpts { fuse: Some(fuse), who: Some(who), heading: Some(heading), ..Default::default() }))
    };
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::character::anim_toss_item(g, c, Some(crate::character::TOSS_QUICK));
    crate::game::game_say(g, gd, if kind == SquadThrowKind::Frag { "frag out!" } else { "bang out!" }, None, None, None);
    let text = format!("{} · {} out", g.gd[gd].callsign, if kind == SquadThrowKind::Frag { "frag" } else { "bang" });
    let t = g.time;
    g.m.squadthrow.states.get_mut(key).unwrap().shown = Some(StateShown { t, text });
    g.stats.squadthrow.thrown += 1;
    if kind == SquadThrowKind::Frag {
        g.stats.squadthrow.frags += 1;
    } else {
        g.stats.squadthrow.stuns += 1;
    }
    let who_said = g.gd[gd].callsign.clone();
    let log = &mut g.m.squadthrow.squadthrow_log;
    log.push(SquadthrowModSQUADTHROWLOGItem { t, kind, who: who_said, from: arc.from, at: v3::copy(at), land: v3::copy(arc.sol.land), rest: arc.rest });
    if log.len() > 100 {
        log.drain(0..50);
    }
}

/// The squad's order log, in its own words for a floor order. (squadthrow.ts squadLogRow)
pub fn squad_log_row(g: &mut crate::game::Game, gd: GuardId, order: &str, at: V3) {
    let (t, who) = (g.time, g.gd[gd].callsign.clone());
    let target = format!("floor:{},{}", to_fixed_1(at[0]), to_fixed_1(at[2]));
    let serial = g.serial();
    g.orders.log.push(crate::orders::SquadOrdersLogItem { t, order: String::from(order), who: vec![who], target, auto: None, laid: None, serial });
}

/// The assignment that walks him to his throw, under the row's own id. (squadthrow.ts walkTo)
pub fn walk_to(g: &mut crate::game::Game, gd: GuardId, kind: crate::squadthrow::SquadThrowKind, post: V3, at: V3, arc: Option<&crate::squadthrow::Arc>) -> Option<OrdersAssignmentId> {
    let face = match arc {
        Some(a) if g.k.squadthrow.lob => v3::yaw_of(a.sol.v0),
        _ => v3::yaw_to(post, at),
    };
    let w = word(kind).as_str();
    crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
        order: String::from(row_of(kind).as_str()),
        post: Some(v3::copy(post)),
        face: Some(face),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        going: format!("moving to throw · {}", w),
        there: format!("throwing · {}", w),
        set_line: None,
        ..Default::default()
    });
    crate::orders::squad_orders_of(g, gd)
}

/// The order's walking case: he is sent to the floor he has a throw from; the can stays in the bag. (squadthrow.ts startWalk)
pub fn start_walk(g: &mut crate::game::Game, th: &mut crate::squadthrow::SquadThrower, t: &crate::orders::OrderTarget, kind: crate::squadthrow::SquadThrowKind) {
    let key = state_key(g);
    let gd = th.gd;
    let post = th.post.unwrap();
    let at = target_at(t);
    let line = walk_line(g, gd, post);
    let lent = squad_lend(g, gd, t);
    let last_t = g.time;
    let arc = arc_of(th);
    let mut w = Walk {
        kind,
        at: v3::copy(at),
        post: v3::copy(post),
        arc: arc.clone(),
        spent: 0.0,
        last_t,
        on_t: -1.0,
        thrown_t: -1.0,
        again: 0.0,
        held: None,
        held_at: None,
        held_t: None,
        line,
        mates: lent.mates,
        gd: lent.gd,
        t: lent.t,
        a: lent.a,
        home: lent.home,
        home_yaw: lent.home_yaw,
        back: lent.back.map(|b| back_as!(b, WalkBack)),
    };
    w.a = walk_to(g, gd, kind, post, at, Some(&arc));
    {
        let s = g.m.squadthrow.states.get_mut(key).unwrap();
        s.walks.set(gd, w);
        s.warn = None;
    }
    crate::game::game_say(g, gd, "moving to throw.", None, None, None);
    squad_log_row(g, gd, row_of(kind).as_str(), at);
    g.stats.squadthrow.walks += 1;
}

/// What a man is doing NOW, kept so that he can be handed back to it. (squadthrow.ts squadLend)
pub fn squad_lend(g: &mut crate::game::Game, gd: GuardId, t: &crate::orders::OrderTarget) -> crate::squadthrow::Lent {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let was = crate::orders::squad_orders_of(g, gd);
    let back = match was {
        Some(a) if resumed().iter().any(|r| *r == g.tab.orders_assignment[a].order) => {
            let w = g.tab.orders_assignment[a].clone();
            Some(LentBack {
                order: w.order,
                post: w.post.map(v3::copy),
                follow: Some(w.follow),
                face: w.face,
                watch: w.watch,
                door: w.door,
                strafe: w.strafe,
                stance: Some(w.stance),
                upper: Some(w.upper),
                stack_side: Some(w.stack_side),
                move_: w.move_,
                going: w.going,
                there: w.there,
                shy: w.shy,
                spare: w.spare,
                from: w.from,
                crouch: w.crouch,
                fit: w.fit,
                tries: w.tries,
                set_line: None,
                ..LentBack::zeroed()
            })
        }
        _ => None,
    };
    let mut mates = Vec::new();
    if let Some(a) = was {
        let (group, order) = (g.tab.orders_assignment[a].group, g.tab.orders_assignment[a].order.clone());
        for m in g.guards.clone() {
            if m == gd {
                continue;
            }
            if let Some(o) = crate::orders::squad_orders_of(g, m) {
                let row = &g.tab.orders_assignment[o];
                if row.group == group && row.order == order {
                    mates.push(m);
                }
            }
        }
    }
    Lent { mates, gd, t: t.clone(), a: None, home: [p[0], 0.0, p[2]], home_yaw: g.ch[c].body_yaw, back }
}

/// The throw is made, or will not be: back to where he stood and to what he was doing. (squadthrow.ts squadHandBack)
pub fn squad_hand_back(g: &mut crate::game::Game, w: &mut crate::squadthrow::Lent, back: Option<bool>) {
    let back = back.unwrap_or(g.k.squadthrow.back);
    let gd = w.gd;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let b = if back { w.back.clone() } else { None };
    if g.gd[gd].state == crate::game::GuardState::Dead || crate::orders::squad_orders_of(g, gd) != w.a {
        return;
    }
    crate::orders::squad_orders_new_group(g);
    if b.as_ref().map_or(false, |b| b.order == "fallIn") {
        let d = g.orders.defs.iter().copied().find(|d| g.tab.orders_order_def[*d].id == "fallIn");
        if let Some(d) = d {
            seam_def_issue(g, d, &[gd], &w.t);
            return;
        }
    }
    let mut stale = false;
    if let Some(b) = &b {
        if b.order != "hold" && !w.mates.is_empty() {
            let mut any = false;
            for m in w.mates.iter().copied() {
                if g.gd[m].state != crate::game::GuardState::Dead {
                    if let Some(o) = crate::orders::squad_orders_of(g, m) {
                        if g.tab.orders_assignment[o].order == b.order {
                            any = true;
                            break;
                        }
                    }
                }
            }
            stale = !any;
        }
    }
    if let Some(b) = b {
        if b.order != "fallIn" && !stale {
            let post = Some(b.post.unwrap_or_else(|| v3::copy(w.home)));
            let face = Some(b.face.unwrap_or(w.home_yaw));
            let mut a = back_as!(b, crate::orders::SquadOrdersAssignA);
            a.post = post;
            a.face = face;
            crate::orders::squad_orders_assign(g, gd, &a);
            return;
        }
    }
    let at: V3 = if back { w.home } else { [p[0], 0.0, p[2]] };
    let face = if back { w.home_yaw } else { g.ch[c].body_yaw };
    crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
        order: String::from("hold"),
        post: Some(v3::copy(at)),
        face: Some(face),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        going: String::from(if back { "moving back" } else { "holding" }),
        there: String::from("holding"),
        set_line: None,
        ..Default::default()
    });
}

/// the walk is over: his word if any, off the list, handed back (walkTick's `end`)
fn end_walk(g: &mut Game, key: PlayerId, gd: GuardId, line: Option<&str>) {
    if let Some(l) = line {
        crate::game::game_say(g, gd, l, None, None, None);
        g.stats.squadthrow.gave_up += 1;
    }
    let w = g.m.squadthrow.states.get_mut(key).and_then(|s| s.walks.delete(gd));
    if let Some(w) = w {
        squad_hand_back(g, &mut walk_lent(&w), None);
    }
}

/// One more look, from where he is (twice at most): true = he is walking again. (walkTick's lookAgain)
fn look_again(g: &mut Game, key: PlayerId, gd: GuardId) -> bool {
    let (again, at, kind) = match walk_of(g, key, gd) {
        Some(w) => (w.again, w.at, w.kind),
        None => return false,
    };
    let th = if again < 2.0 {
        let mut lob = if g.k.squadthrow.lob {
            let walls = crate::throwplan::throw_walls(g, at);
            let n = g.k.squadthrow.lob_sims;
            Some(Lob { walls, lim: LobLim { n }, clean: LobClean::LookAgain { kind, at, gd }, dirty: None })
        } else {
            None
        };
        walk_spot(g, &[gd], at, kind, Some(true), lob.as_mut())
    } else {
        None
    };
    let Some(th) = th else { return false };
    let post = v3::copy(th.post.unwrap());
    let arc = arc_of(&th);
    if let Some(w) = walk_of(g, key, gd) {
        w.again += 1.0;
        w.post = post;
        w.arc = arc.clone();
        w.on_t = -1.0;
        w.held_at = None;
    }
    let line = walk_line(g, gd, post);
    if let Some(w) = walk_of(g, key, gd) {
        w.line = line;
    }
    let a = walk_to(g, gd, kind, post, at, Some(&arc));
    if let Some(w) = walk_of(g, key, gd) {
        w.a = a;
    }
    true
}

/// A WALKING THROWER'S FRAME: the rows' `tick`, while he is under the row's id and the fight does not have him. (squadthrow.ts walkTick)
pub fn walk_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let key = state_key(g);
    let now = g.time;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let had = g.m.squadthrow.states.get(key).and_then(|s| s.walks.get(gd)).map(|w| w.a);
    if had != Some(Some(a)) {
        if had.is_some() {
            g.m.squadthrow.states.get_mut(key).unwrap().walks.delete(gd);
        }
        let face = g.ch[c].body_yaw;
        crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
            order: String::from("hold"),
            post: Some([p[0], 0.0, p[2]]),
            face: Some(face),
            stance: Some(crate::character::Stance::LowReady),
            upper: Some(crate::character::UpperMode::None),
            going: String::from("holding"),
            there: String::from("holding"),
            set_line: None,
            ..Default::default()
        });
        return;
    }
    let (thrown_t, spent, post, at, kind) = {
        let w = walk_of(g, key, gd).unwrap();
        let dt = now - w.last_t;
        w.last_t = now;
        if dt > 0.0 && dt < 0.25 {
            w.spent += dt;
        }
        (w.thrown_t, w.spent, w.post, w.at, w.kind)
    };
    if thrown_t >= 0.0 {
        if now < thrown_t || now - thrown_t >= g.k.squadthrow.toss_secs {
            end_walk(g, key, gd, None);
        }
        return;
    }
    if !g.k.squadthrow.on || !g.k.squadthrow.walk {
        return end_walk(g, key, gd, None);
    }
    if spent > g.k.squadthrow.walk_secs {
        return end_walk(g, key, gd, Some("can't get a throw."));
    }
    if !g.tab.orders_assignment[a].arrived && v3::dist_xz(p, post) > 0.4 {
        return;
    }
    if !crate::squadsmoke::squad_hand_free(g, gd) || hands_busy(g, gd) {
        return;
    }
    let on_t = {
        let w = walk_of(g, key, gd).unwrap();
        if w.on_t < 0.0 {
            w.on_t = now;
        }
        w.on_t
    };
    let lob = g.k.squadthrow.lob;
    if lob && (now - on_t < g.k.squadthrow.settle_secs || g.gd[gd].speed > 0.3) {
        return;
    }
    let roll = roll_of(g, kind);
    let mut arc: Option<Arc> = crate::squadsmoke::squad_arc_to(g, gd, at, roll, None).map(|t| Arc { from: t.from, sol: t.sol, rest: t.rest });
    if arc.is_none() && lob {
        let (held_at, held_t, was) = {
            let w = walk_of(g, key, gd).unwrap();
            (w.held_at, w.held_t, w.arc.clone())
        };
        let redo = match held_at {
            None => true,
            Some(h) => v3::dist_xz(p, h) > 0.05 || now - held_t.unwrap() > 1.0,
        };
        if redo {
            let from: V3 = [was.from[0] + p[0] - post[0], was.from[1], was.from[2] + p[2] - post[2]];
            let f = if crate::scene_collision::solid_blocked(g, [p[0], from[1], p[2]], from, None) { None } else { Some(crate::throwplan::throw_flight(g, kind, from, was.sol.v0, None)) };
            let hits = match &f {
                Some(f) => crate::throwplan::throw_hits(g, f.end, at, None),
                None => false,
            };
            let held: Option<Arc> = match (hits, f) {
                (true, Some(f)) => Some(Arc { from, sol: crate::throwables::ThrowSolution { points: f.points, land: f.land, ..was.sol.clone() }, rest: f.end }),
                _ => {
                    let n = g.k.squadthrow.lob_sims;
                    crate::throwplan::throw_to(g, gd, at, kind, None, None, Some(&mut crate::throwplan::ThrowToLim { n })).map(|t| Arc { from: t.from, sol: t.sol, rest: t.rest })
                }
            };
            if let Some(h) = &held {
                g.tab.orders_assignment[a].face = Some(v3::yaw_of(h.sol.v0));
            }
            if let Some(w) = walk_of(g, key, gd) {
                w.held = held;
                w.held_at = Some(v3::copy(p));
                w.held_t = Some(now);
            }
        }
        arc = walk_of(g, key, gd).and_then(|w| w.held.clone());
    }
    let Some(arc) = arc else {
        if !look_again(g, key, gd) {
            end_walk(g, key, gd, Some("no throw from here."));
        }
        return;
    };
    if let Some(w) = walk_of(g, key, gd) {
        w.arc = arc.clone();
    }
    let toward = if lob { v3::yaw_of(arc.sol.v0) } else { v3::yaw_to(p, at) };
    if m::abs(v3::wrap_angle(g.ch[c].body_yaw - toward)) > 0.9 && now - on_t < g.k.squadthrow.turn_secs {
        return;
    }
    if left(g, kind) <= 0.0 {
        let w = none(g, kind).as_str();
        return end_walk(g, key, gd, Some(w));
    }
    if kind == SquadThrowKind::Frag {
        let sam = !sam_calls(g);
        let who = squad_frag_catches(g, arc.rest, at, Some(SquadFragCatchesMoved { gd, at: p }), Some(sam), None, None);
        if let Some(who) = who {
            if lob && who == format!("{} is in it.", g.gd[gd].callsign) && look_again(g, key, gd) {
                return;
            }
            g.stats.squadthrow.refused += 1;
            return end_walk(g, key, gd, Some(who.as_str()));
        }
    }
    let what = if kind == SquadThrowKind::Frag { crate::doortools::DoorToolsTakeWhat::Frag } else { crate::doortools::DoorToolsTakeWhat::Bang };
    if !crate::doortools::door_tools_take(g, what) {
        let w = none(g, kind).as_str();
        return end_walk(g, key, gd, Some(w));
    }
    throw_it(g, gd, &arc, kind, at);
    if let Some(w) = walk_of(g, key, gd) {
        w.thrown_t = now;
    }
    g.stats.squadthrow.walked += 1;
}

/// What the picture draws of the highlighted row, asked aside. (the closure of squadThrowPlans)
fn preview_plan(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind) -> Option<SquadThrowPlan> {
    let th = if left(g, kind) > 0.0 { squad_throw_plan(g, men, at, kind, Some(false), Some(true)) } else { None };
    let th = th?;
    let pos = g.ch[g.gd[th.gd].ch].pos;
    if kind == SquadThrowKind::Frag {
        let sam = !sam_calls(g);
        if squad_frag_catches(g, th.rest, at, Some(SquadFragCatchesMoved { gd: th.gd, at: th.post.unwrap_or(pos) }), Some(sam), Some(true), None).is_some() {
            return None;
        }
    }
    let mut walk = None;
    if let Some(post) = th.post {
        // (`c.th === th` of the TypeScript always holds here: an answer with a `post` came out of this very slot: see notes)
        let slot = format!("v{}{}", kind.as_str(), if g.k.squadthrow.lob && men.len() == 1 { "1" } else { "" });
        let key = crate::seats::view_key(g);
        let kept = g.m.squadthrow.states.get(key).and_then(|s| s.kept.get(slot.clone())).map(|c| c.line.clone());
        walk = Some(match kept {
            Some(Some(line)) => line,
            Some(None) => {
                let line = walk_line(g, th.gd, post);
                if let Some(c) = g.m.squadthrow.states.get_mut(key).and_then(|s| s.kept.get_mut(slot)) {
                    c.line = Some(line.clone());
                }
                line
            }
            None => walk_line(g, th.gd, post),
        });
    }
    let from = th.post.unwrap_or([pos[0], 0.0, pos[2]]);
    Some(SquadThrowPlan { gd: th.gd, from, points: th.sol.points, land: th.rest, kind, preview: Some(true), walk })
}

/// WHAT THE PICTURE DRAWS, and what a module drawing the squad's plan may read: one entry per throw on its way, and the highlighted row's preview. (squadthrow.ts squadThrowPlans)
pub fn squad_throw_plans(g: &mut crate::game::Game, v: Option<&mut crate::squadthrow::SquadThrowKept>) -> Vec<crate::squadthrow::SquadThrowPlan> {
    let mut out = Vec::new();
    let key = crate::seats::view_key(g);
    if !g.m.squadthrow.states.has(key) || !g.k.squadthrow.on {
        return out;
    }
    for gd in g.m.squadthrow.states.get(key).unwrap().walks.keys() {
        let w = g.m.squadthrow.states.get(key).unwrap().walks.get(gd).unwrap();
        let (thrown_t, w_gd, w_a) = (w.thrown_t, w.gd, w.a);
        if thrown_t >= 0.0 || g.gd[w_gd].state == crate::game::GuardState::Dead || crate::orders::squad_orders_of(g, w_gd) != w_a || !g.guards.contains(&w_gd) {
            continue;
        }
        let p = g.ch[g.gd[w_gd].ch].pos;
        let w = g.m.squadthrow.states.get(key).unwrap().walks.get(gd).unwrap();
        let l = &w.line;
        let (mut bi, mut bd) = (0usize, f64::INFINITY);
        for i in 0..l.len() {
            let d = v3::dist_xz(l[i], p);
            if d < bd {
                bd = d;
                bi = i;
            }
        }
        let walk = if v3::dist_xz(p, w.post) > 0.4 {
            let mut way = vec![v3::copy(p)];
            if bi + 1 < l.len() {
                way.extend_from_slice(&l[bi + 1..]);
            }
            way.push(w.post);
            Some(way)
        } else {
            None
        };
        out.push(SquadThrowPlan { gd: w_gd, from: w.post, points: w.arc.sol.points.clone(), land: w.arc.rest, kind: w.kind, preview: None, walk });
    }
    let asked = match &g.orders.menu {
        Some(mn) => {
            // (`rows[sel]` of a selection off the list, or of none, is no row)
            let row = if mn.sel >= 0.0 { mn.rows.get(mn.sel as usize).copied() } else { None };
            let id = row.map(|d| g.tab.orders_order_def[d].id.clone());
            let id_is = |want: &str| id.as_deref() == Some(want);
            let on_page = mn.page.as_deref() == Some(THROW_PAGE) || mn.flat == Some(true);
            match &mn.target {
                crate::orders::OrderTarget::Floor(t) if on_page && (id_is(FRAG_THERE) || id_is(STUN_THERE)) => {
                    Some((mn.men.clone(), t.at, if id_is(STUN_THERE) { SquadThrowKind::Stun } else { SquadThrowKind::Frag }))
                }
                _ => None,
            }
        }
        None => None,
    };
    if let Some((men, at, kind)) = asked {
        // (with no store handed in, nothing is kept past the one ask)
        let mut own = None;
        let store: &mut SquadThrowKept = match v {
            Some(v) => v,
            None => own.insert(squad_throw_kept()),
        };
        let got = aside(g, key, store, &mut |g: &mut Game| preview_plan(g, &men, at, kind));
        if let Some(p) = got {
            out.push(p);
        }
    }
    out
}

/// who it would catch, in the row's words (the row's `caught`)
fn row_caught(g: &mut Game, th: &SquadThrower, at: V3, kind: SquadThrowKind, fresh: bool) -> Option<String> {
    if kind != SquadThrowKind::Frag {
        return None;
    }
    let pos = g.ch[g.gd[th.gd].ch].pos;
    let sam = !sam_calls(g);
    squad_frag_catches(g, th.rest, at, Some(SquadFragCatchesMoved { gd: th.gd, at: th.post.unwrap_or(pos) }), Some(sam), Some(!fresh), None)
}

/// Why not (Err), or the throw (Ok; with `post`: after a walk). (the rows' `ask`)
fn ask(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind, fresh: bool) -> Result<SquadThrower, String> {
    if left(g, kind) <= 0.0 {
        return Err(String::from(none(g, kind).as_str()));
    }
    let asked = if g.k.squadthrow.lob && men.len() == 1 {
        let key = state_key(g);
        g.m.squadthrow.states.get(key).unwrap().asked.get(kind).cloned()
    } else {
        None
    };
    if let Some(a) = asked {
        if a.key == key_of(at) && a.men.len() > 1 && a.men.contains(&men[0]) {
            if let Some(th) = squad_throw_plan(g, &a.men, at, kind, Some(fresh), None) {
                if th.gd == men[0] && row_caught(g, &th, at, kind, fresh).is_none() {
                    return Ok(th);
                }
            }
        }
    }
    let Some(th) = squad_throw_plan(g, men, at, kind, Some(fresh), None) else {
        return Err(String::from("no throw from here."));
    };
    if let Some(who) = row_caught(g, &th, at, kind, fresh) {
        return Err(who);
    }
    Ok(th)
}

/// (the row's `label`)
fn row_label(g: &mut Game, kind: SquadThrowKind, at: V3) -> String {
    let key = crate::seats::view_key(g);
    let want = key_of(at);
    let x = g.m.squadthrow.states.get(key).and_then(|s| {
        if kind == SquadThrowKind::Stun {
            s.warn.as_ref().map(|x| (x.key.clone(), x.text.clone()))
        } else {
            s.warn_frag.as_ref().map(|x| (x.key.clone(), x.text.clone()))
        }
    });
    let w = match x {
        Some((k, text)) if k == want => text,
        _ => String::new(),
    };
    let n = left(g, kind);
    format!("{} there · {} left{}", word(kind).as_str(), n, w)
}

/// The order is one man's, the thrower's. (the row's `only`)
fn row_only(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind) -> Vec<GuardId> {
    if g.k.squadthrow.lob {
        let key = state_key(g);
        let k = key_of(at);
        g.m.squadthrow.states.get_mut(key).unwrap().asked.set(kind, StateAskedValue { key: k, men: men.to_vec() });
    }
    match squad_throw_plan(g, men, at, kind, None, None) {
        Some(th) => vec![th.gd],
        None => Vec::new(),
    }
}

/// Err: why not. Also sets the row's warning. (the row's `valid`)
fn row_valid(g: &mut Game, men: &[GuardId], at: V3, kind: SquadThrowKind) -> Result<(), String> {
    let a = ask(g, men, at, kind, false);
    if kind == SquadThrowKind::Stun {
        let who = match &a {
            Ok(th) => squad_stun_catches(g, th.rest, at),
            Err(_) => Vec::new(),
        };
        let warn = if who.is_empty() { None } else { Some(StateWarn { key: key_of(at), text: format!(" · too close: {}", who.join(", ")) }) };
        let key = state_key(g);
        g.m.squadthrow.states.get_mut(key).unwrap().warn = warn;
    } else {
        let on = match &a {
            Ok(th) => sam_calls(g) && squad_frag_on_sam(g, th.rest, at),
            Err(_) => false,
        };
        let warn = if on { Some(StateWarnFrag { key: key_of(at), text: String::from(" · too close: you") }) } else { None };
        let key = state_key(g);
        g.m.squadthrow.states.get_mut(key).unwrap().warn_frag = warn;
    }
    a.map(|_| ())
}

/// Some: the refusal. (the row's `issue`)
fn row_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, kind: SquadThrowKind) -> Option<String> {
    let at = target_at(t);
    let mut a = match ask(g, men, at, kind, true) {
        Ok(a) => a,
        Err(why) => {
            g.stats.squadthrow.refused += 1;
            return Some(why);
        }
    };
    if a.post.is_some() {
        start_walk(g, &mut a, t, kind);
        return None;
    }
    let what = if kind == SquadThrowKind::Frag { crate::doortools::DoorToolsTakeWhat::Frag } else { crate::doortools::DoorToolsTakeWhat::Bang };
    if !crate::doortools::door_tools_take(g, what) {
        return Some(String::from(none(g, kind).as_str()));
    }
    throw_it(g, a.gd, &arc_of(&a), kind, at);
    squad_log_row(g, a.gd, row_of(kind).as_str(), at);
    None
}

/// The row that opens the page and the page's two: made, not yet on the menu. (squadthrow.ts rows)
pub fn rows(g: &mut crate::game::Game) -> Vec<OrdersOrderDefId> {
    let open = crate::orders::OrderDef {
        id: String::from(THROW_OPEN),
        opens: Some(String::from(THROW_PAGE)),
        hot_key: Some(g.k.frag.order_key.clone()),
        targets: vec![crate::orders::TargetKind::Floor],
        free: Some(true),
        label: crate::orders::OrderLabel::ThrowOpen,
        issue: crate::orders::OrderIssue::Nothing,
        ..crate::orders::OrderDef::zeroed()
    };
    let open = g.tab.orders_order_def.push(open);
    let mut out = vec![open];
    for kind in [SquadThrowKind::Frag, SquadThrowKind::Stun] {
        let one = crate::orders::OrderDef {
            id: String::from(row_of(kind).as_str()),
            page: Some(String::from(THROW_PAGE)),
            targets: vec![crate::orders::TargetKind::Floor],
            free: Some(true),
            label: crate::orders::OrderLabel::ThrowRow { kind },
            only: Some(crate::orders::OrderOnly::ThrowRow { kind }),
            valid: Some(crate::orders::OrderValid::ThrowRow { kind }),
            issue: crate::orders::OrderIssue::ThrowRow { kind },
            tick: Some(crate::orders::OrderTick::ThrowRow),
            ..crate::orders::OrderDef::zeroed()
        };
        out.push(g.tab.orders_order_def.push(one));
    }
    out
}

/// The squad's HUD block: the throw just made. (squadthrow.ts notes)
pub fn notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> {
    let key = crate::seats::view_key(g);
    if let Some(w) = g.m.squadthrow.states.get(key).and_then(|s| s.shown.as_ref()) {
        if g.time >= w.t && g.time - w.t < g.k.squadthrow.shown_secs {
            return vec![crate::orders::OrderNote::Text(w.text.clone())];
        }
    }
    Vec::new()
}

/// The rows on the menu while the switch is on, off it otherwise. (squadthrow.ts sync)
pub fn sync(g: &mut crate::game::Game) {
    let has = g.orders.defs.iter().any(|d| g.tab.orders_order_def[*d].id == THROW_OPEN);
    let on = g.k.squadthrow.on;
    if on == has {
        return;
    }
    if !on {
        for id in [THROW_OPEN, FRAG_THERE, STUN_THERE] {
            crate::orders::squad_orders_unregister(g, id);
        }
        if g.orders.menu.is_some() {
            crate::orders::squad_orders_close(g);
        }
        return;
    }
    state_key(g);
    let smoke = g.orders.defs.iter().position(|d| g.tab.orders_order_def[*d].id == "smokeThere");
    let hold = g.orders.defs.iter().position(|d| g.tab.orders_order_def[*d].id == "hold");
    let after = if smoke.is_some() { smoke } else { hold };
    let made = rows(g);
    let before = after.and_then(|a| g.orders.defs.get(a + 1).copied()).map(|d| g.tab.orders_order_def[d].id.clone());
    crate::orders::squad_orders_register(g, made[0], before.as_deref());
    for d in &made[1..] {
        crate::orders::squad_orders_register(g, *d, None);
    }
    let s = state(g);
    if !s.noted {
        s.noted = true;
        g.orders.notes.push(crate::orders::OrderNoteFn::SquadThrow);
    }
}

/// Once a frame after the menu's own input: the rows in step with the switch, and G with a floor menu's first page up turns it to the throws. True: the press was the row's. (squadthrow.ts squadThrowKey)
pub fn squad_throw_key(g: &mut crate::game::Game, input: InputId) -> bool {
    let key = crate::seats::view_key(g);
    if !g.k.squadthrow.on && !g.m.squadthrow.states.has(key) {
        return false;
    }
    if g.m.squadthrow.states.get(key).map_or(false, |s| !s.walks.is_empty()) {
        let mut at = 0usize;
        while let Some(gd) = g.m.squadthrow.states.get(key).and_then(|s| s.walks.next_key(&mut at)) {
            let w_a = match g.m.squadthrow.states.get(key).and_then(|s| s.walks.get(gd)) {
                Some(w) => w.a,
                None => continue,
            };
            if g.gd[gd].state == crate::game::GuardState::Dead || crate::orders::squad_orders_of(g, gd) != w_a {
                g.m.squadthrow.states.get_mut(key).unwrap().walks.delete(gd);
            } else if !g.k.squadthrow.on {
                let w = g.m.squadthrow.states.get_mut(key).unwrap().walks.delete(gd);
                if let Some(w) = w {
                    squad_hand_back(g, &mut walk_lent(&w), None);
                }
            }
        }
    }
    sync(g);
    let hit_key = g.k.frag.order_key.clone();
    if g.orders.menu.is_none() || !crate::input::input_hit(g, input, &hit_key) {
        return false;
    }
    let page = g.orders.menu.as_ref().unwrap().page.clone();
    if page.as_deref() == Some(THROW_PAGE) {
        g.tab.input[input].pressed.delete(hit_key);
        return true;
    }
    if page.is_some() {
        return false;
    }
    let i = g.orders.menu.as_ref().unwrap().rows.iter().position(|d| g.tab.orders_order_def[*d].id == THROW_OPEN);
    let Some(i) = i else { return false };
    g.tab.input[input].pressed.delete(hit_key);
    crate::orders::squad_orders_pick(g, Some(i as f64));
    true
}

/// (squadthrow.ts squadThrowSummary)
pub fn squad_throw_summary(g: &mut crate::game::Game) -> String {
    if !g.k.squadthrow.on {
        return String::from("off");
    }
    let (f, s) = (left(g, SquadThrowKind::Frag), left(g, SquadThrowKind::Stun));
    format!("frag {} · stun {} · X on a spot of floor, {}, then 1 or 2 · thrown {}", f, s, g.k.frag.order_key.replacen("Key", "", 1), g.stats.squadthrow.thrown)
}

// ---- seam: only what the skeleton blocks (notes.md, ## Skeleton) ----

/// squadthrow.ts:461 `d.issue(S, [gd], W.t)`: the arm of the `fallIn` row
fn seam_def_issue(g: &mut Game, d: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget) {
    todo!("needs: a public name for OrderIssue's arms, here OrderIssue::FallIn (orders.ts:611); no function of crate::orders runs a row's issue")
}

#[cfg(test)]
mod tests {
    use super::*;

    // the expected strings are the TypeScript's own: bun -e "console.log((x).toFixed(1))"
    #[test]
    fn to_fixed_is_the_languages() {
        for (x, s) in [(0.25, "0.3"), (-0.25, "-0.3"), (0.75, "0.8"), (1.25, "1.3"), (2.5, "2.5"), (1.45, "1.4"), (8.35, "8.3"), (-0.04, "-0.0"), (-0.0, "0.0"), (0.0, "0.0"), (12.0, "12.0"), (-3.14159, "-3.1"), (0.05, "0.1"), (0.15, "0.1"), (1234.75, "1234.8")] {
            assert_eq!(to_fixed_1(x), s, "{}", x);
        }
    }
}
