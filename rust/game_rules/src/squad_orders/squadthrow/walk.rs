use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec, vec::Vec};

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

/// a walk read as the Lent it extends
fn walk_lent(w: &Walk) -> Lent {
    Lent { mates: w.mates.clone(), gd: w.gd, t: w.t.clone(), a: w.a, home: w.home, home_yaw: w.home_yaw, back: w.back.clone().map(|b| back_as!(b, LentBack)) }
}

/// the row of the walk (only between calls that take `g`)
fn walk_of(g: &mut Game, key: GroupId, gd: GuardId) -> Option<&mut Walk> {
    g.m.squadthrow.states.get_mut(key).and_then(|s| s.walks.get_mut(gd))
}

fn arc_of(th: &SquadThrower) -> Arc {
    Arc { from: th.from, sol: th.sol.clone(), rest: th.rest }
}

/// The state's row, made if it is not there; its key, for a write that outlives a call taking `g`.
fn state_key(g: &mut Game) -> GroupId {
    state(g);
    g.group
}

/// This view's state, the row itself, made if it is not there.
pub(super) fn state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::squadthrow::State {
    let k = g.group;
    g.m.squadthrow.states.get_or_insert_with(k, || State::default())
}

/// The thing leaves his hand; the count is taken already.
pub(super) fn throw_it(g: &mut crate::game::Game, gd: GuardId, arc: &crate::squadthrow::Arc, kind: crate::squadthrow::SquadThrowKind) {
    let key = state_key(g);
    let c = g.gd[gd].ch;
    let who = g.ch[c].id;
    let heading = v3::yaw_of(arc.sol.v0);
    let now = g.time;
    {
        let s = g.m.squadthrow.states.get_mut(key).unwrap();
        s.last.set(gd, now);
        s.plans.clear();   // (one fewer in the bag, and his hands are taken: what the menu shows is worked out again)
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
    crate::game::game_say(g, gd, if kind == SquadThrowKind::Frag { "frag out!" } else { "bang out!" }, None, None);
    let text = format!("{} · {} out", g.gd[gd].callsign, if kind == SquadThrowKind::Frag { "frag" } else { "bang" });
    let t = g.time;
    g.m.squadthrow.states.get_mut(key).unwrap().shown = Some(StateShown { t, text });
    let who_said = g.gd[gd].callsign.clone();
    let log = &mut g.m.squadthrow.squadthrow_log;
    log.push(SquadthrowModSQUADTHROWLOGItem { who: who_said, rest: arc.rest });
    if log.len() > 100 {
        log.drain(0..50);
    }
}

/// The squad's order log, in its own words for a floor order.
pub(in crate::squad_orders) fn squad_log_row(g: &mut crate::game::Game, gd: GuardId, order: &str, at: V3) {
    let (t, who) = (g.time, g.gd[gd].callsign.clone());
    let target = format!("floor:{},{}", js::to_fixed(at[0], 1), js::to_fixed(at[2], 1));
    let serial = g.serial();
    g.groups[g.group].orders.log.push(crate::orders::SquadOrdersLogItem { t, order: String::from(order), who: vec![who], target, auto: None, serial });
}

/// The assignment that walks him to his throw, under the row's own id.
fn walk_to(g: &mut crate::game::Game, gd: GuardId, kind: crate::squadthrow::SquadThrowKind, post: V3, at: V3, arc: Option<&crate::squadthrow::Arc>) -> Option<TaskId> {
    let face = match arc {
        Some(a) => v3::yaw_of(a.sol.v0),
        _ => v3::yaw_to(post, at),
    };
    let w = word(kind).as_str();
    crate::orders::squad_orders_assign(g, gd, &crate::duty::TaskGiven {
        order: String::from(row_of(kind).as_str()),
        post: Some(v3::copy(post)),
        face: Some(face),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        going: format!("moving to throw · {}", w).into(),
        there: format!("throwing · {}", w).into(),
        set_line: None,
        ..Default::default()
    });
    crate::orders::squad_orders_of(&crate::seats::here(g), gd)
}

/// The order's walking case: he is sent to the floor he has a throw from; the can stays in the bag.
pub(super) fn start_walk(g: &mut crate::game::Game, th: &mut crate::squadthrow::SquadThrower, t: &crate::orders::OrderTarget, kind: crate::squadthrow::SquadThrowKind) {
    let key = state_key(g);
    let gd = th.gd;
    let post = th.post.unwrap();
    let at = t.at();
    let line = walk_line(g, gd, post);
    let lent = squad_lend(g, &crate::seats::here(g), gd, t);
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
        s.plans.clear();
    }
    crate::game::game_say(g, gd, "moving to throw.", None, None);
    squad_log_row(g, gd, row_of(kind).as_str(), at);
}

/// What a man is doing NOW, kept so that he can be handed back to it.
pub(in crate::squad_orders) fn squad_lend(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, t: &crate::orders::OrderTarget) -> crate::squadthrow::Lent {
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let was = crate::orders::squad_orders_of(seat, gd);
    let back = match was {
        Some(a) if crate::orders::rows::is(g, a, crate::orders::Is::Resumed) => {
            let w = g.tab.task[a].clone();
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
                ..LentBack::default()
            })
        }
        _ => None,
    };
    let mut mates = Vec::new();
    if let Some(a) = was {
        let (group, order) = (g.tab.task[a].group, g.tab.task[a].order.clone());
        for m in g.guards.clone() {
            if m == gd {
                continue;
            }
            if let Some(o) = crate::orders::squad_orders_of(seat, m) {
                let row = &g.tab.task[o];
                if row.group == group && row.order == order {
                    mates.push(m);
                }
            }
        }
    }
    Lent { mates, gd, t: t.clone(), a: None, home: [p[0], 0.0, p[2]], home_yaw: g.ch[c].body_yaw, back }
}

/// The throw is made, or will not be: back to where he stood and to what he was doing.
pub(in crate::squad_orders) fn squad_hand_back(g: &mut crate::game::Game, w: &mut crate::squadthrow::Lent, back: Option<bool>) {
    let back = back.unwrap_or(true);
    let gd = w.gd;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let b = if back { w.back.clone() } else { None };
    if crate::condition::down(g, g.gd[gd].ch) || crate::orders::squad_orders_of(&crate::seats::here(g), gd) != w.a {
        return;
    }
    crate::orders::squad_orders_new_group(g);
    if b.as_ref().map_or(false, |b| crate::orders::rows::id_is(&b.order, crate::orders::Is::Follows)) {
        let d = g.groups[g.group].orders.defs.iter().copied().find(|d| g.tab.orders_order_def[*d].id == crate::orders::rows::fallin::ORDER.id);
        if let Some(d) = d {
            crate::orders::order_issue(g, d, &[gd], &w.t, None);
            return;
        }
    }
    let mut stale = false;
    if let Some(b) = &b {
        if !crate::orders::rows::id_is(&b.order, crate::orders::Is::Holds) && !w.mates.is_empty() {
            let mut any = false;
            for m in w.mates.iter().copied() {
                if !crate::condition::down(g, g.gd[m].ch) {
                    if let Some(o) = crate::orders::squad_orders_of(&crate::seats::here(g), m) {
                        if g.tab.task[o].order == b.order {
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
        if !crate::orders::rows::id_is(&b.order, crate::orders::Is::Follows) && !stale {
            let post = Some(b.post.unwrap_or_else(|| v3::copy(w.home)));
            let face = Some(b.face.unwrap_or(w.home_yaw));
            let mut a = back_as!(b, crate::duty::TaskGiven);
            a.post = post;
            a.face = face;
            crate::orders::squad_orders_assign(g, gd, &a);
            return;
        }
    }
    let at: V3 = if back { w.home } else { [p[0], 0.0, p[2]] };
    let face = if back { w.home_yaw } else { g.ch[c].body_yaw };
    crate::orders::squad_orders_assign(g, gd, &crate::duty::TaskGiven {
        order: String::from(crate::orders::rows::hold::ORDER.id),
        post: Some(v3::copy(at)),
        face: Some(face),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
        going: String::from(if back { "moving back" } else { "holding" }).into(),
        there: "holding".into(),
        set_line: None,
        ..Default::default()
    });
}

/// the walk is over: his word if any, off the list, handed back (walkTick's `end`)
fn end_walk(g: &mut Game, key: GroupId, gd: GuardId, line: Option<&str>) {
    if let Some(l) = line {
        crate::game::game_say(g, gd, l, None, None);
    }
    let w = g.m.squadthrow.states.get_mut(key).and_then(|s| s.walks.delete(gd));
    if let Some(w) = w {
        squad_hand_back(g, &mut walk_lent(&w), None);
    }
}

/// One more look, from where he is (twice at most): true = he is walking again. (walkTick's lookAgain)
fn look_again(g: &mut Game, key: GroupId, gd: GuardId) -> bool {
    let (again, at, kind) = match walk_of(g, key, gd) {
        Some(w) => (w.again, w.at, w.kind),
        None => return false,
    };
    let th = if again < 2.0 {
        let mut lob = {
            let walls = crate::throwplan::throw_walls(g, at);
            let n = g.k.squadthrow.lob_sims;
            Some(Lob { walls, lim: LobLim { n }, clean: LobClean::LookAgain { kind, at, gd }, dirty: None })
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

/// One frame of a walking thrower.
#[derive(Clone, Copy)]
struct OnHisWalk {
    key: GroupId,
    gd: GuardId,
    a: TaskId,
    now: f64,
    p: V3,
    post: V3,
    at: V3,
    kind: SquadThrowKind,
}

/// A WALKING THROWER'S FRAME: the rows' `tick`, while he is under the row's id and the fight does not have him. Walking, nothing; on his place and settled, the throw, if it
/// still holds.
pub fn walk_tick(g: &mut crate::game::Game, gd: GuardId, a: TaskId) {
    let key = state_key(g);
    let now = g.time;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let had = g.m.squadthrow.states.get(key).and_then(|s| s.walks.get(gd)).map(|w| w.a);
    if had != Some(Some(a)) { holds_where_he_is(g, key, gd, had.is_some()); return; }
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
    if spent > g.k.squadthrow.walk_secs { return end_walk(g, key, gd, Some("can't get a throw.")); }
    if !g.tab.task[a].arrived && v3::dist_xz(p, post) > 0.4 { return; }
    if !crate::squadsmoke::squad_hand_free(g, gd) || hands_at_work(g, gd) { return; }
    let on_t = {
        let w = walk_of(g, key, gd).unwrap();
        if w.on_t < 0.0 {
            w.on_t = now;
        }
        w.on_t
    };
    if now - on_t < g.k.squadthrow.settle_secs || g.gd[gd].speed > 0.3 { return; }
    let f = OnHisWalk { key, gd, a, now, p, post, at, kind };
    let Some(arc) = throw_from_here(g, &f) else {
        if !look_again(g, key, gd) {
            end_walk(g, key, gd, Some("no throw from here."));
        }
        return;
    };
    if let Some(w) = walk_of(g, key, gd) {
        w.arc = arc.clone();
    }
    let toward = v3::yaw_of(arc.sol.v0);
    if m::abs(v3::wrap_angle(g.ch[c].body_yaw - toward)) > 0.9 && now - on_t < g.k.squadthrow.turn_secs { return; }
    if left(g, kind) <= 0.0 { return end_walk(g, key, gd, Some(none(kind).as_str())); }
    if frag_would_catch_ours(g, &f, &arc) { return; }
    let what = if kind == SquadThrowKind::Frag { crate::doortools::DoorToolsCan::Frag } else { crate::doortools::DoorToolsCan::Bang };
    if !crate::doortools::door_tools_take(g, what) { return end_walk(g, key, gd, Some(none(kind).as_str())); }
    throw_it(g, gd, &arc, kind);
    if let Some(w) = walk_of(g, key, gd) {
        w.thrown_t = now;
    }
}

/// His order is no longer this walk: it is forgotten, and he holds where he stands.
fn holds_where_he_is(g: &mut Game, key: GroupId, gd: GuardId, had_a_walk: bool) {
    if had_a_walk {
        g.m.squadthrow.states.get_mut(key).unwrap().walks.delete(gd);
    }
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let face = g.ch[c].body_yaw;
    crate::orders::squad_orders_assign(g, gd, &crate::duty::TaskGiven {
        order: String::from(crate::orders::rows::hold::ORDER.id),
        post: Some([p[0], 0.0, p[2]]),
        face: Some(face),
        stance: Some(crate::character::Stance::LowReady),
        upper: Some(crate::character::UpperMode::None),
        going: "holding".into(),
        there: "holding".into(),
        set_line: None,
        ..Default::default()
    });
}

/// A straight arc from where he stands, else the throw he came here for, kept while he stays put and for a second at most.
fn throw_from_here(g: &mut Game, f: &OnHisWalk) -> Option<Arc> {
    let OnHisWalk { key, gd, now, p, at, kind, .. } = *f;
    let roll = roll_of(g, kind);
    let arc: Option<Arc> = crate::squadsmoke::squad_arc_to(g, gd, at, roll, None).map(|t| Arc { from: t.from, sol: t.sol, rest: t.rest });
    if arc.is_some() { return arc; }
    let (held_at, held_t, was) = {
        let w = walk_of(g, key, gd).unwrap();
        (w.held_at, w.held_t, w.arc.clone())
    };
    let redo = match held_at {
        None => true,
        Some(h) => v3::dist_xz(p, h) > 0.05 || now - held_t.unwrap() > 1.0,
    };
    if redo { works_his_throw_out_again(g, f, &was); }
    walk_of(g, key, gd).and_then(|w| w.held.clone())
}

/// The arc he walked here for, made from where he actually stands, if it still comes down on the spot; else any he can work out.
fn works_his_throw_out_again(g: &mut Game, f: &OnHisWalk, was: &Arc) {
    let OnHisWalk { key, gd, a, now, p, post, at, kind } = *f;
    let from: V3 = [was.from[0] + p[0] - post[0], was.from[1], was.from[2] + p[2] - post[2]];
    let flight = if crate::scene_collision::solid_blocked(g, [p[0], from[1], p[2]], from, None) { None }
        else { Some(crate::throwplan::throw_flight(g, kind, from, was.sol.v0, None)) };
    let hits = match &flight {
        Some(f) => crate::throwplan::throw_hits(g, f.end, at, None),
        None => false,
    };
    let held: Option<Arc> = match (hits, flight) {
        (true, Some(f)) => Some(Arc { from, sol: crate::throwables::ThrowSolution { points: f.points, land: f.land, ..was.sol.clone() }, rest: f.end }),
        _ => {
            let n = g.k.squadthrow.lob_sims;
            crate::throwplan::throw_to(g, gd, at, kind, None, None, Some(&mut crate::throwplan::ThrowToLim { n })).map(|t| Arc { from: t.from, sol: t.sol, rest: t.rest })
        }
    };
    if let Some(h) = &held {
        g.tab.task[a].face = Some(v3::yaw_of(h.sol.v0));
    }
    if let Some(w) = walk_of(g, key, gd) {
        w.held = held;
        w.held_at = Some(v3::copy(p));
        w.held_t = Some(now);
    }
}

/// A frag thrown now would catch one of ours: the job ends with his word, unless it is only himself and he has another look. True: no throw this frame.
fn frag_would_catch_ours(g: &mut Game, f: &OnHisWalk, arc: &Arc) -> bool {
    let OnHisWalk { key, gd, p, at, kind, .. } = *f;
    if kind != SquadThrowKind::Frag { return false; }
    let Some(who) = squad_frag_catches(g, arc.rest, at, Some(SquadFragCatchesMoved { gd, at: p }), Some(false), None, None) else { return false };
    if who == format!("{} is in it.", g.gd[gd].callsign) && look_again(g, key, gd) { return true; }
    end_walk(g, key, gd, Some(who.as_str()));
    true
}

/// The row that opens the page and the page's two: made, not yet on the menu.
fn rows(g: &mut crate::game::Game) -> Vec<OrdersOrderDefId> {
    use crate::orders::rows;
    [&rows::throwopen::ORDER, &rows::fragthere::ORDER, &rows::stunthere::ORDER].into_iter().map(|r| crate::orders::squad_orders_row(g, rows::def(r))).collect()
}

/// The squad's HUD block: the throw just made.
pub fn notes(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    if let Some(w) = g.m.squadthrow.states.get(seat.group_id).and_then(|s| s.shown.as_ref()) {
        if g.time >= w.t && g.time - w.t < g.k.squadthrow.shown_secs {
            return vec![crate::view::Note { key: String::new(), text: w.text.clone() }];
        }
    }
    Vec::new()
}

/// The rows on the menu.
fn sync(g: &mut crate::game::Game) {
    let has = g.groups[g.group].orders.defs.iter().any(|d| g.tab.orders_order_def[*d].id == THROW_OPEN);
    if has {
        return;
    }
    state_key(g);
    let smoke = g.groups[g.group].orders.defs.iter().position(|d| g.tab.orders_order_def[*d].id == crate::orders::rows::smokethere::ORDER.id);
    let hold = g.groups[g.group].orders.defs.iter().position(|d| g.tab.orders_order_def[*d].id == crate::orders::rows::hold::ORDER.id);
    let after = if smoke.is_some() { smoke } else { hold };
    let made = rows(g);
    let before = after.and_then(|a| g.groups[g.group].orders.defs.get(a + 1).copied()).map(|d| g.tab.orders_order_def[d].id.clone());
    crate::orders::squad_orders_register(g, made[0], before.as_deref());
    for d in &made[1..] {
        crate::orders::squad_orders_register(g, *d, None);
    }
}

/// Once a frame after the ring's input: the rows in step with the switch, the walks whose man is gone, and what is worked out for an open menu.
pub fn squad_throw_frame(g: &mut crate::game::Game) {
    let key = g.group;
    if g.m.squadthrow.states.get(key).map_or(false, |s| !s.walks.is_empty()) {
        let mut at = 0usize;
        while let Some(gd) = g.m.squadthrow.states.get(key).and_then(|s| s.walks.next_key(&mut at)) {
            let w_a = match g.m.squadthrow.states.get(key).and_then(|s| s.walks.get(gd)) {
                Some(w) => w.a,
                None => continue,
            };
            if crate::condition::down(g, g.gd[gd].ch) || crate::orders::squad_orders_of(&crate::seats::here(g), gd) != w_a {
                g.m.squadthrow.states.get_mut(key).unwrap().walks.delete(gd);
            }
        }
    }
    sync(g);
    // (no menu up, nothing is shown: what was worked out for one, or for the order it gave, goes)
    if g.groups[g.group].orders.menu.is_none() { if let Some(s) = g.m.squadthrow.states.get_mut(key) { if s.plans.len() > 0 { s.plans.clear(); } } }
    else { follow(g); }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The order log's words for a floor order: its place to a tenth of a metre, a tie going up.
    #[test]
    fn a_floor_order_is_logged_with_its_place_to_a_tenth() {
        let mut g = Game::blank();
        let gd = g.gd.push(crate::game::Guard::blank());
        squad_log_row(&mut g, gd, "fragThere", [0.25, 0.0, -0.04]);
        squad_log_row(&mut g, gd, "fragThere", [1.45, 0.0, 12.0]);
        let log = &g.groups[g.group].orders.log;
        assert_eq!(log[0].target, "floor:0.3,-0.0");
        assert_eq!(log[1].target, "floor:1.4,12.0");
        assert_ne!(log[0].serial, log[1].serial);
    }
}
