//! src/game/orders.ts lines 366 to 774
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ small words

/// `t.at`, whichever kind of target it is
fn target_pos(t: &OrderTarget) -> V3 {
    match t {
        OrderTarget::Floor(x) => x.at,
        OrderTarget::Door(x) => x.at,
        OrderTarget::None(x) => x.at,
        OrderTarget::Foe(x) => x.at,
    }
}
/// `a === b` of two `Who`s (ring.indexOf)
fn same_who(a: Who, b: Who) -> bool {
    match (a, b) {
        (Who::Num(x), Who::Num(y)) => x == y,
        (Who::All, Who::All) | (Who::Red, Who::Red) | (Who::Blue, Who::Blue) => true,
        _ => false,
    }
}
fn who_of_pair(id: crate::game::PairId) -> Who {
    match id {
        crate::game::PairId::Red => Who::Red,
        crate::game::PairId::Blue => Who::Blue,
    }
}
fn sel_of_pair(id: crate::game::PairId) -> SquadOrdersMenSel {
    match id {
        crate::game::PairId::Red => SquadOrdersMenSel::Red,
        crate::game::PairId::Blue => SquadOrdersMenSel::Blue,
    }
}
/// SquadOrders.people.near: the man of the other side nearest a point, with how near; only targetAt asks, while `people` is set (pairorders.ts:159)
fn people_near(g: &mut Game, at: V3) -> Option<(GuardId, f64)> {
    if !crate::pairorders::pair_orders_up(g) { return None; }
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.pair.pick_m;
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if !crate::pairorders::pair_orders_is_foe(g, h) { continue; }
        let d = v3::dist_xz(g.ch[g.gd[h].ch].pos, at);
        if d < bd { bd = d; best = Some(h); }
    }
    best.map(|gd| (gd, bd))
}
fn is_dead(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].state == crate::game::GuardState::Dead
}
/// a string as a condition
fn truthy_str(s: &Option<String>) -> bool {
    s.as_deref().map_or(false, |x| !x.is_empty())
}

// ------------------------------------------------------------ the built-in rows' functions (orders.ts:581-627)
// The arms of OrderLabel, OrderCan, OrderValid, OrderIssue and OrderTick: orders' match calls these, `row_<arm>_<field>`, with what the TypeScript calls the field with.
// `can`: None is true, Some the word why not. `valid`: Ok is true, Err(None) false, Err(Some) the word. `issue`: the reason it cannot be done.

/// 'move'.can (orders.ts:581)
pub(crate) fn row_move_can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    let at = target_pos(t);
    if squad_orders_walk_try(g, gd, at).is_some() { return None; }
    squad_orders_reach(g, gd, at)
}
/// 'move'.valid: a door nobody has tried is walked up to, where it has room for them (orders.ts:582)
pub(crate) fn row_move_valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match squad_orders_walk_stacks(g, men, target_pos(t)) {
        Some(SquadOrdersWalkStacks::Text(w)) => Err(Some(w)),
        _ => Ok(()),
    }
}
/// 'move'.issue: a place and a watch each that fit the room, or the line abreast (orders.ts:583)
pub(crate) fn row_move_issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let t_at = target_pos(t);
    let pc = g.pl[g.player].ch;
    let sam = g.ch[pc].pos;
    let face = if v3::dist_xz(sam, t_at) > 0.3 { v3::yaw_to(sam, t_at) } else { g.ch[pc].body_yaw };
    // (the room kept: Sam's, and every other man's mark: where he is walking to, not where he is)
    let rest: Vec<GuardId> = squad_orders_men(g, None, None).into_iter().filter(|m| !men.contains(m)).collect();
    let mut bodies: Vec<V3> = vec![sam];
    let mut held: Vec<String> = Vec::new();
    for &m in &rest {
        let of = squad_orders_of(g, m);
        let post = of.and_then(|a| g.tab.orders_assignment[a].post);
        bodies.push(match post { Some(p) => p, None => g.ch[g.gd[m].ch].pos });
    }
    for &m in &rest {
        let of = squad_orders_of(g, m);
        let key = of.and_then(|a| g.tab.orders_assignment[a].fit.as_ref().and_then(|f| f.key.clone()));
        if let Some(k) = key { if !k.is_empty() { held.push(k); } }
    }
    let fits = crate::roomfit::fit_spots(g, t_at, men.len() as f64, face, sam, Some(&bodies), Some(&held));
    let deal: Vec<(GuardId, V3)> = match &fits {
        Some(f) => {
            let spots: Vec<V3> = f.iter().map(|x| x.pos).collect();
            let groups = crate::roomfit::fit_groups(f);
            squad_orders_pair(g, men, &spots, Some(&groups), None)
        }
        None => {
            let spots = squad_orders_spots_at(g, t_at, face, men.len() as f64);
            let groups: Vec<f64> = (0..spots.len()).map(|i| (i >> 1) as f64).collect();
            squad_orders_pair(g, men, &spots, Some(&groups), None)
        }
    };
    let at = match squad_orders_walk_stacks(g, men, t_at) {
        Some(SquadOrdersWalkStacks::Text(w)) => return Some(w),
        Some(SquadOrdersWalkStacks::Map(m)) => Some(m),
        None => None,
    };
    for (gd, p) in deal {
        let fit = fits.as_ref().and_then(|f| f.iter().find(|x| v3::dist_xz(x.pos, p) < 1e-6).cloned());
        let walk = at.as_ref().and_then(|m| m.get(gd).copied());
        if let Some(w) = walk {
            let on = SquadOrdersWalkSendOn {
                order: String::from("move"), post: Some(p), face: Some(face),
                stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
                going: String::from("moving"), there: String::from("in position"), set_line: Some(String::from("set.")), fit,
                ..Default::default()
            };
            squad_orders_walk_send(g, gd, SquadOrdersWalkSendT { door: w.door, side: w.side, post: w.post }, t_at, &on);
        } else {
            let on = SquadOrdersAssignA {
                order: String::from("move"), post: Some(p), face: Some(face),
                stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
                going: String::from("moving"), there: String::from("in position"), set_line: Some(String::from("set.")), fit,
                ..Default::default()
            };
            squad_orders_assign(g, gd, &on);
        }
    }
    None
}
/// 'move'.tick (orders.ts:591)
pub(crate) fn row_move_tick(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    crate::roomfit::fit_tick(g, gd, a);
}
/// 'stack'.valid (orders.ts:592)
pub(crate) fn row_stack_valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let (door, side) = match t { OrderTarget::Door(x) => (x.door, x.side), _ => return Err(None) };
    let posts = match squad_orders_stack_posts_wait(g, door, side, men) {
        Some(p) => p,
        None => return Err(Some(squad_orders_no_room_words(g, "no room on that door.", &mut |g, _| squad_orders_stack_posts_wait(g, door, side, men).is_some()))),   // (ORDERS.inWay: … or that it is Sam who is in the way, where it is)
    };
    let spots: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|x| x.group.unwrap_or(0.0)).collect();
    let front = g.dr[door].frame_centre;
    for (gd, p) in squad_orders_pair(g, men, &spots, Some(&groups), Some(front)) {
        if let Some(w) = squad_orders_reach(g, gd, p) { if !w.is_empty() { return Err(Some(w)); } }
    }
    Ok(())
}
/// 'stack'.issue (orders.ts:597)
pub(crate) fn row_stack_issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let (door, side) = match t { OrderTarget::Door(x) => (x.door, x.side), _ => return Some(String::from("no door")) };
    let posts = match squad_orders_stack_posts_wait(g, door, side, men) {
        Some(p) => p,
        None => return Some(squad_orders_no_room_words(g, "no room to stack on that door", &mut |g, _| squad_orders_stack_posts_wait(g, door, side, men).is_some())),
    };
    let name = door_words(g, door);
    let spots: Vec<V3> = posts.iter().map(|p| p.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|p| p.group.unwrap_or(0.0)).collect();
    let fc = g.dr[door].frame_centre;
    for (gd, p) in squad_orders_pair(g, men, &spots, Some(&groups), Some(fc)) {
        // (`x.pos === p` in the TypeScript: the very mark dealt. Marks of one stack are never at one spot, so the first equal is that one)
        let post = *posts.iter().find(|x| x.pos == p).unwrap();
        let a = SquadOrdersAssignA {
            order: String::from("stack"), post: Some(post.pos), face: Some(post.face),
            watch: Some([fc[0], 1.05, fc[2]]), door: Some(door), shy: Some(door),
            stance: Some(if post.lead { crate::character::Stance::Stack } else { crate::character::Stance::HighReady }),
            upper: Some(crate::character::UpperMode::None),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            stack_side: Some(post.stack_side),
            going: format!("stacking · {}", name), there: format!("stacked · {}", name), set_line: Some(String::from("set.")),
            ..Default::default()
        };
        squad_orders_assign(g, gd, &a);
    }
    squad_orders_spare_rear(g, men, door, None, None);
    None
}
/// 'cover'.label (orders.ts:605)
pub(crate) fn row_cover_label(g: &mut Game, t: &OrderTarget) -> String {
    String::from(if matches!(t, OrderTarget::Door(_)) { "cover this door" } else { "cover that way" })
}
/// 'cover'.can (orders.ts:606)
pub(crate) fn row_cover_can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    if squad_orders_has_line(g, gd, t) { None } else { Some(String::from("no line from here.")) }
}
/// 'cover'.issue (orders.ts:606)
pub(crate) fn row_cover_issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let t_at = target_pos(t);
    let watch: V3 = [t_at[0], 1.1, t_at[2]];
    for &gd in men {
        let p = g.ch[g.gd[gd].ch].pos;
        let yaw = v3::yaw_to(p, watch);
        let (place, door) = match t {
            OrderTarget::Door(x) => (door_words(g, x.door), Some(x.door)),
            _ => (crate::dialogue::watched_place(g, p, watch).unwrap_or_else(|| String::from("that way")), None),
        };
        let words = format!("covering {}", place);
        let a = SquadOrdersAssignA {
            order: String::from("cover"), post: Some([p[0], 0.0, p[2]]), face: Some(yaw), watch: Some(watch), door,
            stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
            going: words.clone(), there: words,
            ..Default::default()
        };
        squad_orders_assign(g, gd, &a);
    }
    None
}
/// 'fallIn'.issue's sort of one man against another: pair_near, near, pair (orders.ts:615)
fn fall_in_cmp(many: bool, a: (f64, f64, Option<crate::game::PairId>), b: (f64, f64, Option<crate::game::PairId>)) -> f64 {
    let pair = if many && a.2 != b.2 {
        let (x, y) = (a.2.map_or("", |p| p.as_str()), b.2.map_or("", |p| p.as_str()));   // (localeCompare of '', 'blue', 'red': the same order as bytes)
        match x.cmp(y) {
            core::cmp::Ordering::Less => -1.0,
            core::cmp::Ordering::Equal => 0.0,
            core::cmp::Ordering::Greater => 1.0,
        }
    } else {
        0.0
    };
    js::or(js::or(a.0 - b.0, pair), a.1 - b.1)
}
/// 'fallIn'.issue: a man already in the file keeps his place; three or more file by pairs, the pair with the nearer man first (orders.ts:611)
pub(crate) fn row_fall_in_issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let mut taken: Vec<f64> = Vec::new();
    for (gd, a) in g.orders.asg.iter() {
        let row = &g.tab.orders_assignment[*a];
        if row.post.is_none() && !men.contains(&gd) { taken.push(row.follow); }
    }
    let sam = g.ch[g.pl[g.player].ch].pos;
    let nears: Vec<f64> = men.iter().map(|&m| v3::dist_xz(g.ch[g.gd[m].ch].pos, sam)).collect();
    let pair_ids: Vec<Option<crate::game::PairId>> = men.iter().map(|&m| squad_orders_pair_of(g, m)).collect();
    let many = men.len() > 2;
    let mut sorted: Vec<(GuardId, (f64, f64, Option<crate::game::PairId>))> = Vec::with_capacity(men.len());
    for (i, &gd) in men.iter().enumerate() {
        let id = pair_ids[i];
        let pair_near = if many && id.is_some() {
            let of_pair: Vec<f64> = (0..men.len()).filter(|&j| pair_ids[j] == id).map(|j| nears[j]).collect();
            m::min_n(&of_pair)
        } else {
            nears[i]
        };
        sorted.push((gd, (pair_near, nears[i], id)));
    }
    js::sort_by(&mut sorted, |a, b| fall_in_cmp(many, a.1, b.1));
    let mut k = 0.0;
    for (gd, _) in sorted {
        while taken.contains(&k) { k += 1.0; }
        taken.push(k);
        let a = SquadOrdersAssignA {
            order: String::from("fallIn"), post: None, follow: Some(k), face: None,
            stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
            going: String::from("falling in"), there: String::from("with you"),
            ..Default::default()
        };
        squad_orders_assign(g, gd, &a);
    }
    None
}
/// 'hold'.issue: a running man plants a stride on; the squad of four take an approach each (orders.ts:618)
pub(crate) fn row_hold_issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let mut posts: Vec<V3> = Vec::with_capacity(men.len());
    for &gd in men {
        let c = g.gd[gd].ch;
        let (p, v, k) = (g.ch[c].pos, g.ch[c].vel, g.k.orders.hold_stride);
        let on: V3 = [p[0] + v[0] * k, 0.0, p[2] + v[2] * k];
        let ok = !crate::scene_collision::nav_grid_is_blocked(g, on[0], on[2]) && !crate::scene_collision::solid_blocked(g, [p[0], 0.45, p[2]], [on[0], 0.45, on[2]], None);
        posts.push(if ok { on } else { [p[0], 0.0, p[2]] });
    }
    let arcs = if squad_orders_big(g) && men.len() > 1 { Some(squad_orders_hold_arcs(g, &posts)) } else { None };
    for (i, &gd) in men.iter().enumerate() {
        let arc = arcs.as_ref().and_then(|a| a.get(i).copied().flatten());
        let a = match arc {
            Some(SquadOrdersHoldArcsItem { yaw, door: Some(door), .. }) => {   // (a door close by in his arc: the gun, and the light on it, is ON it)
                let fc = g.dr[door].frame_centre;
                let there = format!("holding · {}", door_words(g, door));
                SquadOrdersAssignA {
                    order: String::from("hold"), post: Some(posts[i]), face: Some(yaw), watch: Some([fc[0], 1.1, fc[2]]), door: Some(door),
                    stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true),
                    going: String::from("holding"), there,
                    ..Default::default()
                }
            }
            Some(arc) => {   // (… else out along his arc)
                let watch: V3 = [posts[i][0] + m::sin(arc.yaw) * arc.reach, 1.1, posts[i][2] + m::cos(arc.yaw) * arc.reach];
                let pl = crate::dialogue::watched_place(g, posts[i], [posts[i][0] + m::sin(arc.yaw) * arc.reach, 0.0, posts[i][2] + m::cos(arc.yaw) * arc.reach]);
                let there = if truthy_str(&pl) { format!("holding · {}", pl.unwrap()) } else { String::from("holding") };
                SquadOrdersAssignA {
                    order: String::from("hold"), post: Some(posts[i]), face: Some(arc.yaw), watch: Some(watch),
                    stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true),
                    going: String::from("holding"), there,
                    ..Default::default()
                }
            }
            None => SquadOrdersAssignA {
                order: String::from("hold"), post: Some(posts[i]), face: Some(g.ch[g.gd[gd].ch].body_yaw),
                stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
                going: String::from("holding"), there: String::from("holding"),
                ..Default::default()
            },
        };
        squad_orders_assign(g, gd, &a);
    }
    None
}

// ------------------------------------------------------------ the night breach's owner (orders.ts:630-637)

/// owner 'nightbreach'.owns: of four, the drill's are the first pair alone
pub(crate) fn nightbreach_owns(g: &mut Game, gd: GuardId) -> bool {
    g.night_breach.active && crate::nightbreach::night_breach_drill_ops(g).contains(&gd) && crate::nightbreach::night_breach_owns_pair(g)
}
/// owner 'nightbreach'.release: the drill is the pair's, letting go of one man ends it for both
pub(crate) fn nightbreach_release(g: &mut Game, gd: GuardId) -> Option<String> {
    if g.night_breach.active && crate::nightbreach::night_breach_drill_ops(g).contains(&gd) { crate::nightbreach::night_breach_call_off(g) } else { None }
}
/// owner 'nightbreach'.status
pub(crate) fn nightbreach_status(g: &mut Game, gd: GuardId) -> Option<String> {
    use crate::nightbreach::NightBreachState as St;
    if !g.night_breach.active || !g.night_breach.ops.contains(&gd) { return None; }
    let state = crate::nightbreach::night_breach_state(g);
    if !crate::nightbreach::night_breach_drill_ops(g).contains(&gd) {
        return if state == St::Off { None } else { Some(String::from("standing by")) };
    }
    Some(String::from(match state {
        St::Staged => "waiting",
        St::Stacking => "stacking · the breach",
        St::Set => "set · press F to send them in",
        St::Going | St::Inside => "breaching",
        St::Clear => "holding the room",
        _ => "holding",
    }))
}

// ------------------------------------------------------------ the class

/// A module that changed these men's plan with nobody asking says so: drawn for a moment.   (orders.ts SquadOrders.show)
pub fn squad_orders_show(g: &mut crate::game::Game, men: &[GuardId]) {
    let t = g.time;
    let who: Vec<String> = men.iter().filter(|&&m| !is_dead(g, m)).map(|&m| g.gd[m].callsign.clone()).collect();
    g.orders.shows.push(SquadOrdersShowsItem { t, who });
    if g.orders.shows.len() > 16 { g.orders.shows.drain(0..8); }
    g.stats.autopilot.shows += 1;
}

/// The built-in rows of the menu and the night breach's owner.   (orders.ts SquadOrders.constructor)
pub fn squad_orders_new(g: &mut crate::game::Game, cast: Option<&crate::orders::OtherCast>) -> crate::orders::SquadOrders {
    let mut s = SquadOrders::blank();
    s.knobs = g.k.orders.clone();
    s.cast = cast.cloned();
    let words = |w: &[&str]| -> Option<Vec<String>> { Some(w.iter().map(|x| String::from(*x)).collect()) };
    let rows = [
        OrderDef {
            id: String::from("move"), key: Some(1.0), label: OrderLabel::Text { text: String::from("move there") }, targets: vec![TargetKind::Floor],
            ack: words(&["moving.", "on it."]),
            can: Some(OrderCan::Move), valid: Some(OrderValid::Move), issue: OrderIssue::Move, tick: Some(OrderTick::Move),
            ..Default::default()
        },
        OrderDef {
            id: String::from("stack"), key: Some(1.0), label: OrderLabel::Text { text: String::from("stack up") }, targets: vec![TargetKind::Door],
            ack: words(&["stacking up.", "on the door."]),
            valid: Some(OrderValid::Stack), issue: OrderIssue::Stack,
            ..Default::default()
        },
        OrderDef {
            id: String::from("cover"), key: Some(2.0), label: OrderLabel::Cover, targets: vec![TargetKind::Floor, TargetKind::Door],
            ack: words(&["covering.", "i've got it."]),
            can: Some(OrderCan::Cover), issue: OrderIssue::Cover,
            ..Default::default()
        },
        OrderDef {
            id: String::from("fallIn"), key: Some(3.0), label: OrderLabel::Text { text: String::from("fall in") }, targets: vec![TargetKind::Floor, TargetKind::Door, TargetKind::None],
            ack: words(&["on you.", "with you."]),
            issue: OrderIssue::FallIn,
            ..Default::default()
        },
        OrderDef {
            id: String::from("hold"), key: Some(4.0), label: OrderLabel::Text { text: String::from("hold position") }, targets: vec![TargetKind::Floor, TargetKind::Door, TargetKind::None],
            ack: words(&["holding."]),
            issue: OrderIssue::Hold,
            ..Default::default()
        },
    ];
    for row in rows {
        let id = g.tab.orders_order_def.push(row);
        s.defs.push(id);
    }
    // the night breach's drill owns the pair from Sam's 'stack up' (F) until the room is called or lost (its closures, reached by this row's name: nightbreach_owns, _release, _status)
    s.owners.push(SquadOwner { name: String::from("nightbreach") });
    s
}

/// Is there a squad to order at all (a living operative unit, outside any show).   (orders.ts SquadOrders.active)
pub fn squad_orders_active(g: &mut crate::game::Game) -> bool {
    if g.touring || g.quiet_utility || g.puppet.is_some() || crate::director::director_showing(g) { return false; }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if squad_orders_mine(g, gd) && !is_dead(g, gd) { return true; }
    }
    false
}

/// The roster: every operative unit on the guard list, in its order, down or up.   (orders.ts SquadOrders.squad)
pub fn squad_orders_squad(g: &mut crate::game::Game) -> Vec<GuardId> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if squad_orders_mine(g, gd) { out.push(gd); }
    }
    out
}

/// Is he one of the men these orders move: the one roster test.   (orders.ts SquadOrders.mine)
pub fn squad_orders_mine(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.orders.cast.is_some() { return g.qrf.men.contains(&gd); }   // (OtherCast.mine: its one closure, qrf.ts:1021)
    if g.gd[gd].unit.is_none() || g.gd[gd].team != crate::teams::Team::Operative { return false; }
    let seat = g.seat;
    crate::seats::seat_has(g, seat, gd)
}

/// The player is one of these men's own.   (orders.ts SquadOrders.sams)
pub fn squad_orders_sams(g: &mut crate::game::Game) -> bool {
    g.orders.cast.is_none()
}

/// Whose side a can of theirs is.   (orders.ts SquadOrders.team)
pub fn squad_orders_team(g: &mut crate::game::Game) -> crate::teams::Team {
    match &g.orders.cast {
        Some(c) => c.team,
        None => crate::teams::Team::Operative,
    }
}

/// The keys a locked door is tried with for them: Sam's ring and the codes he has learned.   (orders.ts SquadOrders.ring)
pub fn squad_orders_ring(g: &mut crate::game::Game) -> JsSet<String> {
    if let Some(c) = &g.orders.cast { return c.ring.clone(); }
    let p = g.player;
    let mut out = g.pl[p].keys.clone();
    for code in g.pl[p].codes.keys() { out.add(code); }
    out
}

/// The living men an order would go to: the teammate under the reticle, else `who`.   (orders.ts SquadOrders.recipients)
pub fn squad_orders_recipients(g: &mut crate::game::Game, t: Option<&crate::orders::SquadOrdersRecipientsT>, who: Option<crate::orders::SquadOrdersRecipientsWho>) -> Vec<GuardId> {
    if let Some(SquadOrdersRecipientsT::None(x)) = t {
        if let Some(man) = x.man {
            return if !is_dead(g, man) && squad_orders_squad(g).contains(&man) { vec![man] } else { Vec::new() };
        }
    }
    let sel = match who {
        Some(SquadOrdersRecipientsWho::Num(k)) => SquadOrdersMenSel::Num(k),
        Some(SquadOrdersRecipientsWho::All) => SquadOrdersMenSel::All,
        Some(SquadOrdersRecipientsWho::Red) => SquadOrdersMenSel::Red,
        Some(SquadOrdersRecipientsWho::Blue) => SquadOrdersMenSel::Blue,
        None => match g.orders.who {
            Who::Num(k) => SquadOrdersMenSel::Num(k),
            Who::All => SquadOrdersMenSel::All,
            Who::Red => SquadOrdersMenSel::Red,
            Who::Blue => SquadOrdersMenSel::Blue,
        },
    };
    squad_orders_men(g, Some(sel), None)
}

/// The one place to ask for men: the squad, a pair, or a man; the living unless `down`.   (orders.ts SquadOrders.men)
pub fn squad_orders_men(g: &mut crate::game::Game, sel: Option<crate::orders::SquadOrdersMenSel>, opts: Option<crate::orders::SquadOrdersMenOpts>) -> Vec<GuardId> {
    let s = squad_orders_squad(g);
    let pair_is = |g: &Game, gd: GuardId, id: crate::game::PairId| g.gd[gd].unit.as_ref().and_then(|u| u.pair) == Some(id);
    let list: Vec<GuardId> = match sel.unwrap_or(SquadOrdersMenSel::All) {
        SquadOrdersMenSel::All => s,
        SquadOrdersMenSel::Red => s.into_iter().filter(|&gd| pair_is(g, gd, crate::game::PairId::Red)).collect(),
        SquadOrdersMenSel::Blue => s.into_iter().filter(|&gd| pair_is(g, gd, crate::game::PairId::Blue)).collect(),
        // (a place past the roster's end is `undefined`, and the filter below takes it out)
        SquadOrdersMenSel::Num(k) => {
            let i = k as usize;
            if k >= 0.0 && i as f64 == k && i < s.len() { vec![s[i]] } else { Vec::new() }
        }
        SquadOrdersMenSel::Guard(gd) => if s.contains(&gd) { vec![gd] } else { Vec::new() },
    };
    let down = opts.and_then(|o| o.down).unwrap_or(false);
    list.into_iter().filter(|&gd| down || !is_dead(g, gd)).collect()
}

/// The squad of four (any roster of more than two).   (orders.ts SquadOrders.big)
pub fn squad_orders_big(g: &mut crate::game::Game) -> bool {
    squad_orders_men(g, Some(SquadOrdersMenSel::All), Some(SquadOrdersMenOpts { down: Some(true) })).len() > 2
}

/// The pair a man was dealt into.   (orders.ts SquadOrders.pairOf)
pub fn squad_orders_pair_of(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::game::PairId> {
    g.gd[gd].unit.as_ref().and_then(|u| u.pair)
}

/// The pairs on the roster, in its order.   (orders.ts SquadOrders.pairs)
pub fn squad_orders_pairs(g: &mut crate::game::Game) -> Vec<crate::game::PairId> {
    let mut out = Vec::new();
    for gd in squad_orders_squad(g) {
        if let Some(id) = g.gd[gd].unit.as_ref().and_then(|u| u.pair) {
            if !out.contains(&id) { out.push(id); }
        }
    }
    out
}

/// Is the squad dealt into more than one pair.   (orders.ts SquadOrders.paired)
pub fn squad_orders_paired(g: &mut crate::game::Game) -> bool {
    squad_orders_pairs(g).len() > 1
}

/// The next element: gold, red, blue for a squad in pairs; all, then man by man for the one pair.   (orders.ts SquadOrders.cycleWho)
pub fn squad_orders_cycle_who(g: &mut crate::game::Game, dir: Option<f64>) -> crate::orders::Who {
    let dir = dir.unwrap_or(1.0);
    let s = squad_orders_squad(g);
    let mut ring: Vec<Who> = vec![Who::All];
    if squad_orders_paired(g) {
        for id in squad_orders_pairs(g) {
            if !squad_orders_men(g, Some(sel_of_pair(id)), None).is_empty() { ring.push(who_of_pair(id)); }
        }
    } else {
        for (k, &gd) in s.iter().enumerate() {
            if !is_dead(g, gd) { ring.push(Who::Num(k as f64)); }
        }
    }
    let was = g.orders.who;
    let now = match ring.iter().position(|&w| same_who(w, was)) {   // (a man of a squad in pairs is not on the ring: gold, either way)
        None => Who::All,
        Some(i) => ring[js::rem(i as f64 + dir + ring.len() as f64, ring.len() as f64) as usize],
    };
    g.orders.who = now;
    now
}

/// CMDRING.wholeSquad, the player's own squad: every order of his is everyone's.   (orders.ts SquadOrders.whole)
pub fn squad_orders_whole(g: &mut crate::game::Game) -> bool {
    g.orders.cast.is_none()
}

/// The orders go to that one man until T is pressed again.   (orders.ts SquadOrders.address)
pub fn squad_orders_address(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let k = match squad_orders_squad(g).iter().position(|&x| x == gd) { Some(k) => k, None => return false };
    if is_dead(g, gd) { return false; }
    g.orders.who = Who::Num(k as f64);
    true
}

/// Who these men are, in a word or two.   (orders.ts SquadOrders.whoWords)
pub fn squad_orders_who_words(g: &mut crate::game::Game, men: Option<&[GuardId]>) -> String {
    let own;
    let men: &[GuardId] = match men {
        Some(m) => m,
        None => { own = squad_orders_recipients(g, None, None); &own }
    };
    let alive = squad_orders_squad(g).into_iter().filter(|&gd| !is_dead(g, gd)).count();
    if men.len() == alive && men.len() > 1 {
        return String::from(if squad_orders_paired(g) { "gold" } else { "all" });
    }
    let id = if men.len() > 1 { squad_orders_pair_of(g, men[0]) } else { None };
    if let Some(id) = id {
        let all_of = men.iter().all(|&gd| g.gd[gd].unit.as_ref().and_then(|u| u.pair) == Some(id));
        if all_of && men.len() == squad_orders_men(g, Some(sel_of_pair(id)), None).len() { return String::from(id.as_str()); }   // (a whole pair of a bigger squad)
    }
    let names: Vec<&str> = men.iter().map(|&gd| g.gd[gd].callsign.as_str()).collect();
    let joined = names.join(", ");
    if joined.is_empty() { String::from("nobody") } else { joined }
}

/// The element's colour for the panel.   (orders.ts SquadOrders.toneOf)
pub fn squad_orders_tone_of(g: &mut crate::game::Game, men: &[GuardId]) -> Option<crate::orders::Tone> {
    if !squad_orders_paired(g) || men.is_empty() { return None; }
    let id = squad_orders_pair_of(g, men[0]);
    if let Some(id) = id {
        if men.iter().all(|&gd| g.gd[gd].unit.as_ref().and_then(|u| u.pair) == Some(id)) {
            return Some(match id {
                crate::game::PairId::Red => Tone::Red,
                crate::game::PairId::Blue => Tone::Blue,
            });
        }
    }
    Some(Tone::Gold)
}

/// (orders.ts SquadOrders.has)
pub fn squad_orders_has(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.orders.asg.has(gd)
}

/// (orders.ts SquadOrders.of)
pub fn squad_orders_of(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> {
    g.orders.asg.get(gd).copied()
}

/// The tick's one write of a follower's latched word: `d` is how far he is off his place.   (orders.ts SquadOrders.followWord)
pub fn squad_orders_follow_word(g: &mut crate::game::Game, a: OrdersAssignmentId, d: f64) {
    let t = g.time;
    let arrived = g.tab.orders_assignment[a].arrived;
    let (far, hold) = (g.k.orders.word_far_m, g.k.orders.word_hold_s);
    let w = g.m.orders.follow_word.get_or_insert_with(a, || OrdersModFOLLOWWORDValue { on: false, out_t: -1.0 });
    if arrived {
        w.on = true;
        w.out_t = -1.0;
    } else if w.on {
        if w.out_t < 0.0 || t < w.out_t { w.out_t = t; }   // (t < outT: a checkpoint took the clock back)
        if d > far || t - w.out_t >= hold { w.on = false; }
    }
}

/// Is he shown as there: a follower's is the latch, every other order's is `arrived`.   (orders.ts SquadOrders.shownThere)
pub fn squad_orders_shown_there(g: &mut crate::game::Game, a: OrdersAssignmentId) -> bool {
    let row = &g.tab.orders_assignment[a];
    if row.post.is_some() { return row.arrived; }
    match g.m.orders.follow_word.get(a) {
        Some(w) => w.on,
        None => row.arrived,
    }
}

/// `d.listed ?? d.targets` of a row (orders.ts:714)
fn listed_on(d: &OrderDef) -> &[TargetKind] {
    match &d.listed {
        Some(l) => l,
        None => &d.targets,
    }
}
/// Is this digit taken on `def`'s page by a row offered on one of its targets (orders.ts:714)
fn digit_clash(g: &mut Game, def: OrdersOrderDefId, k: f64) -> bool {
    let me = &g.tab.orders_order_def[def];
    let page = me.page.as_deref().unwrap_or("");
    g.orders.defs.iter().any(|&o| {
        let d = &g.tab.orders_order_def[o];
        d.key == Some(k) && d.stands_in.is_none() && d.page.as_deref().unwrap_or("") == page && listed_on(d).iter().any(|t| listed_on(me).contains(t))
    })
}

/// Add an order to the menu: at the foot or before `before`, on the next free digit of its page.   (orders.ts SquadOrders.register)
pub fn squad_orders_register(g: &mut crate::game::Game, def: OrdersOrderDefId, before: Option<&str>) {
    let id = g.tab.orders_order_def[def].id.clone();
    squad_orders_unregister(g, &id);
    if let Some(si) = &g.tab.orders_order_def[def].stands_in {   // a stand-in: the other row's digit, page and place, the two never listed together
        let wants = si.for_.clone();
        let stood = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == wants);
        if let Some(stood) = stood {
            squad_orders_seat(g, def, stood);
            return;
        }
        g.tab.orders_order_def[def].stands_in = None;   // (not registered: listed as a row of its own)
    }
    if truthy_str(&g.tab.orders_order_def[def].hot_key) {
        g.tab.orders_order_def[def].key = None;
    } else {
        let key = g.tab.orders_order_def[def].key;
        let redeal = match key {
            None => true,
            Some(k) => k < 1.0 || k > ORDER_DIGITS || digit_clash(g, def, k),
        };
        if redeal {
            let mut k = if truthy_str(&g.tab.orders_order_def[def].page) { 1.0 } else { 5.0 };
            while k <= ORDER_DIGITS && digit_clash(g, def, k) { k += 1.0; }
            g.tab.orders_order_def[def].key = if k <= ORDER_DIGITS { Some(k) } else { None };   // (no digit left: the wheel still reaches the row)
        }
    }
    let i = match before {
        Some(b) if !b.is_empty() => g.orders.defs.iter().position(|&d| g.tab.orders_order_def[d].id == b),
        _ => None,
    };
    let at = i.unwrap_or(g.orders.defs.len());
    g.orders.defs.insert(at, def);
    let stand_ins: Vec<OrdersOrderDefId> = g.orders.defs.iter().copied()
        .filter(|&d| g.tab.orders_order_def[d].stands_in.as_ref().map_or(false, |x| x.for_ == id))
        .collect();
    for s in stand_ins {   // (registered again: its stand-ins move with it)
        if let Some(j) = g.orders.defs.iter().position(|&d| d == s) { g.orders.defs.remove(j); }
        squad_orders_seat(g, s, def);
    }
}

/// Open a fresh group (the men whose arrival one 'set.' waits on).   (orders.ts SquadOrders.newGroup)
pub fn squad_orders_new_group(g: &mut crate::game::Game) -> f64 {
    g.orders.group += 1.0;
    g.orders.group
}

/// (orders.ts SquadOrders.unregister)
pub fn squad_orders_unregister(g: &mut crate::game::Game, id_2: &str) {
    if let Some(i) = g.orders.defs.iter().position(|&d| g.tab.orders_order_def[d].id == id_2) {
        g.orders.defs.remove(i);
    }
}

/// Take the men off their orders (all of them by default): they stop where they are, still on the runner's script.   (orders.ts SquadOrders.clear)
pub fn squad_orders_clear(g: &mut crate::game::Game, men: Option<&[GuardId]>) {
    let own;
    let men: &[GuardId] = match men {
        Some(m) => m,
        None => { own = g.orders.asg.keys(); &own }
    };
    for &gd in men {
        if g.orders.asg.has(gd) && g.gd[gd].script.is_some() && !is_dead(g, gd) { crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(None), ..Default::default() }); }
        squad_orders_drop(g, gd);
    }
    if !g.orders.asg.values().any(|&a| g.tab.orders_assignment[a].post.is_none()) { g.orders.trail.clear(); }
}

/// What a point in the world is, as a target: the nearest of a door, a teammate and Sam himself; else the free floor at or nearest it; else nothing.   (orders.ts SquadOrders.targetAt)
pub fn squad_orders_target_at(g: &mut crate::game::Game, p: V3) -> crate::orders::OrderTarget {
    let (door_pick_m, man_m, self_m) = (g.k.orders.door_pick_m, g.k.orders.man_m, g.k.orders.self_m);
    let sam = g.ch[g.pl[g.player].ch].pos;
    let at: V3 = [p[0], 0.0, p[2]];
    let mut door: Option<DoorId> = None;
    let mut dd = door_pick_m;
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        let centre = g.dr[d].frame_centre;
        let leaf = crate::doors::door_pos(g, d);
        let x = m::min(v3::dist_xz(centre, at), v3::dist_xz(leaf, at));
        if x < dd { dd = x; door = Some(d); }
    }
    let mut man: Option<GuardId> = None;
    let mut dm = man_m;
    for gd in squad_orders_squad(g) {
        let x = v3::dist_xz(g.ch[g.gd[gd].ch].pos, at);
        if !is_dead(g, gd) && x < dm { dm = x; man = Some(gd); }
    }
    let ds = v3::dist_xz(sam, at);
    let fo = if g.orders.people.is_some() { people_near(g, at) } else { None };   // (pairorders.ts: a man of the other side nearer the point than a door, a teammate or Sam himself)
    if let Some((foe, fd)) = fo {
        if !(door.is_some() && dd < fd) && !(man.is_some() && dm < fd) && !(ds < self_m && ds < fd) {
            let fp = g.ch[g.gd[foe].ch].pos;
            return OrderTarget::Foe(OrderTargetFoe { at: [fp[0], 0.0, fp[2]], foe });
        }
    }
    let wk = crate::workthings::work_near(g, at);   // (workthings.ts: a thing a teammate can work, nearer the point than a door, a teammate or Sam himself)
    if let Some(wk) = wk {
        if !(door.is_some() && dd < wk.d) && !(man.is_some() && dm < wk.d) && !(ds < self_m && ds < wk.d) { return wk.target; }
    }
    if man.is_some() && !(door.is_some() && dd < dm) && !(ds < self_m && ds < dm) {
        return OrderTarget::None(OrderTargetNone { at, man });
    }
    if let Some(d) = door {
        if !(ds < self_m && ds < dd) {
            let c = g.dr[d].frame_centre;
            let side = side_of(g, d, sam);
            return OrderTarget::Door(OrderTargetDoor { at: [c[0], 0.0, c[2]], door: d, side });
        }
    }
    if ds < self_m { return OrderTarget::None(OrderTargetNone { at, man: None }); }
    let base = match squad_orders_free_spot(g, at) {
        Some(spot) => OrderTarget::Floor(OrderTargetFloor { at: spot, wall: None, thing: None }),
        None => OrderTarget::None(OrderTargetNone { at, man: None }),
    };
    crate::wallcharge::wall_target(g, at, &base)   // (wallcharge.ts: a floor target under a wall carries the wall)
}

/// How far along the ray it crosses one door's doorway rectangle, nearer than `bt` and with nothing of the scene short of the frame line (orders.ts:757-763)
fn ray_on_door(ro: V3, rd: V3, c: V3, th: f64, width: f64, height: f64, pad: f64, t_scene: f64, bt: f64) -> Option<f64> {
    let (ax, az) = (m::cos(th), m::sin(th));   // (the frame line's own direction; its normal is (−az, ax))
    let dn = rd[2] * ax - rd[0] * az;
    if m::abs(dn) < 1e-4 { return None; }   // (along the wall: no face of the doorway is shown)
    let t = ((c[2] - ro[2]) * ax - (c[0] - ro[0]) * az) / dn;
    if !(t > 0.0) || t >= bt { return None; }
    if (t - t_scene) * m::abs(dn) > 0.1 { return None; }   // the scene is drawn in front of the frame line there
    let y = ro[1] + rd[1] * t;
    if y < 0.0 || y > height { return None; }
    let u = (ro[0] + rd[0] * t - c[0]) * ax + (ro[2] + rd[2] * t - c[2]) * az;
    if m::abs(u) > width * 0.5 + pad { return None; }
    Some(t)
}

/// The door a ray is on as drawn: the nearest doorway it crosses with nothing of the scene nearer.   (orders.ts SquadOrders.doorOnRay)
pub fn squad_orders_door_on_ray(g: &mut crate::game::Game, ro: V3, rd: V3, t_scene: Option<f64>) -> Option<DoorId> {
    let t_scene = t_scene.unwrap_or(f64::INFINITY);
    let pad = g.k.orders.door_ray_pad;
    let mut best: Option<DoorId> = None;
    let mut bt = f64::INFINITY;
    for &d in &g.doors.list {
        let door = &g.dr[d];
        if let Some(t) = ray_on_door(ro, rd, door.frame_centre, door.def.closed_dir, door.def.width, door.height, pad, t_scene, bt) {
            bt = t;
            best = Some(d);
        }
    }
    best
}

/// What the cursor is on, for the menu; with CMDRING.wholeSquad a teammate under it is the squad itself.   (orders.ts SquadOrders.targetUnder)
pub fn squad_orders_target_under(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) -> crate::orders::OrderTarget {
    let t = squad_orders_target_under_of(g, input, canvas);
    if let OrderTarget::None(x) = &t {
        if x.man.is_some() && squad_orders_whole(g) { return OrderTarget::None(OrderTargetNone { at: x.at, man: None }); }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::PairId;

    /// the TypeScript's own bits for doorOnRay's arithmetic (bun, EXACT.on): t of a ray crossing a doorway
    #[test]
    fn a_ray_crosses_a_doorway_on_the_typescripts_bits() {
        let ro: V3 = [1.25, 1.6, -3.5];
        let rd: V3 = v3::normalize([0.3, -0.1, 0.9]);
        let t = ray_on_door(ro, rd, [2.0, 0.0, 1.0], 0.4, 2.0, 2.1, 0.08, f64::INFINITY, f64::INFINITY).unwrap();
        assert_eq!(t.to_bits(), TS_T);
        // nothing of the scene may lie more than a hand short of the frame line, and a nearer door already found wins
        assert!(ray_on_door(ro, rd, [2.0, 0.0, 1.0], 0.4, 2.0, 2.1, 0.08, 1.0, f64::INFINITY).is_none());
        assert!(ray_on_door(ro, rd, [2.0, 0.0, 1.0], 0.4, 2.0, 2.1, 0.08, f64::INFINITY, 1.0).is_none());
        // past the frame's width (u is 0.948 on this ray), over its top (y is 1.059)
        assert!(ray_on_door(ro, rd, [2.0, 0.0, 1.0], 0.4, 1.1, 2.1, 0.08, f64::INFINITY, f64::INFINITY).is_none());
        assert!(ray_on_door(ro, rd, [9.0, 0.0, 1.0], 0.4, 2.0, 2.1, 0.08, f64::INFINITY, f64::INFINITY).is_none());
        assert!(ray_on_door(ro, rd, [2.0, 0.0, 1.0], 0.4, 2.0, 0.5, 0.08, f64::INFINITY, f64::INFINITY).is_none());
    }
    const TS_T: u64 = 0x4014a4cb8302a980;

    /// fall in: of three or more a pair files together, the pair with the nearer man first; a tie of pair distance goes to the pair's word, then the man
    #[test]
    fn a_pair_files_together() {
        let mut men = [
            ('a', (1.0, 5.0, Some(PairId::Red))),
            ('b', (2.0, 2.0, Some(PairId::Blue))),
            ('c', (1.0, 1.0, Some(PairId::Red))),
            ('d', (2.0, 9.0, Some(PairId::Blue))),
        ];
        js::sort_by(&mut men, |a, b| fall_in_cmp(true, a.1, b.1));
        assert_eq!(men.map(|x| x.0), ['c', 'a', 'b', 'd']);
        let mut tie = [('r', (1.0, 1.0, Some(PairId::Red))), ('b', (1.0, 3.0, Some(PairId::Blue)))];
        js::sort_by(&mut tie, |a, b| fall_in_cmp(true, a.1, b.1));
        assert_eq!(tie.map(|x| x.0), ['b', 'r']);
        // two men: the pair's word is not asked
        let mut two = [('r', (1.0, 1.0, Some(PairId::Red))), ('b', (1.0, 3.0, Some(PairId::Blue)))];
        js::sort_by(&mut two, |a, b| fall_in_cmp(false, a.1, b.1));
        assert_eq!(two.map(|x| x.0), ['r', 'b']);
    }
}
