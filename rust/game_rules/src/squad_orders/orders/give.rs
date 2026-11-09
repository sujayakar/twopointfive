use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn kind_of(t: &OrderTarget) -> TargetKind {
    match t {
        OrderTarget::Floor(_) => TargetKind::Floor,
        OrderTarget::Door(_) => TargetKind::Door,
        OrderTarget::None(_) => TargetKind::None,
        OrderTarget::Foe(_) => TargetKind::Foe,
    }
}
fn at_of(t: &OrderTarget) -> V3 {
    match t {
        OrderTarget::Floor(x) => x.at,
        OrderTarget::Door(x) => x.at,
        OrderTarget::None(x) => x.at,
        OrderTarget::Foe(x) => x.at,
    }
}
/// `recipients(t)` with its `who` left to the standing choice
pub(crate) fn recipients_of(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget) -> Vec<GuardId> {
    squad_orders_recipients(g, seat, Some(t), None)
}
pub(crate) fn on(b: Option<bool>) -> bool {
    b.unwrap_or(false)
}
/// `if (word)` of an optional string
fn word(s: &Option<String>) -> bool {
    s.as_ref().map_or(false, |x| !x.is_empty())
}
fn player_ch(g: &Game, seat: &crate::seats::SeatRef) -> CharId {
    g.pl[seat.player].ch
}
fn press_delete(g: &mut Game, input: InputId, code: &str) -> bool {
    g.tab.input[input].pressed.delete(String::from(code))
}
fn press_has(g: &Game, input: InputId, code: &str) -> bool {
    g.tab.input[input].pressed.has(String::from(code))
}
/// `input.clicked &= ~bit`
fn unclick(g: &mut Game, input: InputId, bit: i32) {
    let c = g.tab.input[input].clicked;
    g.tab.input[input].clicked = (js::to_i32(c) & !bit) as f64;
}
/// a row of the list by a number, as `rows[i]`
fn row_at(rows: &[OrdersOrderDefId], i: f64) -> Option<OrdersOrderDefId> {
    if i >= 0.0 && i == m::floor(i) && (i as usize) < rows.len() { Some(rows[i as usize]) } else { None }
}
/// `v.input = input; v.cam = cam; v.canvas = canvas`
fn set_view(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    g.orders.view.input = Some(input);
    g.orders.view.cam = Some(cam);
    g.orders.view.canvas = Some(canvas);
}
/// the open menu's target as its men will work it (menuTarget, on the menu in hand)
fn menu_target_of(m: &Menu) -> OrderTarget {
    match (&m.target, m.men_side) {
        (OrderTarget::Door(x), Some(side)) => OrderTarget::Door(OrderTargetDoor { side, ..*x }),
        (t, _) => t.clone(),
    }
}
// ---- what skeleton 5 still cannot reach ----
/// `b.owner`. Once the table of Box rows is there, the body is that row's field
fn seam_box_owner(g: &Game, b: AnyBoxId) -> f64 { g.tab[b].owner }
/// `b.flags`. As seam_box_owner
fn seam_box_flags(g: &Game, b: AnyBoxId) -> f64 { g.tab[b].flags }
// ---- an order's functions, called through its row (orders/rows/) ----
/// `d.label(t)`
fn order_label(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget) -> String { (g.tab.orders_order_def[d].row.label)(g, t) }
/// `d.can(S, gd, t)`: None he can, or the row has no `can`; Some the word he says
fn order_can(g: &mut crate::game::Game, d: OrdersOrderDefId, gd: GuardId, t: &crate::orders::OrderTarget) -> Option<String> {
    match g.tab.orders_order_def[d].row.can { Some(f) => f(g, gd, t), None => None }
}
/// `d.only(S, men, t)`. A row with no `only`: men, as a copy
fn order_only(g: &mut crate::game::Game, d: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    match g.tab.orders_order_def[d].row.only { Some(f) => f(g, men, t), None => men.to_vec() }
}
/// `d.valid(S, t, men)`: Ok true, or the row has no `valid`; Err(None) false, whyNot's 'not here.'; Err(Some) the word
fn order_valid(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match g.tab.orders_order_def[d].row.valid { Some(f) => f(g, t, men), None => Ok(()) }
}
/// `d.issue(S, men, t, opts)`: Some is the refusal, the string the closure returned
pub fn order_issue(g: &mut crate::game::Game, d: OrdersOrderDefId, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> { (g.tab.orders_order_def[d].row.issue)(g, men, t, o) }
/// the flat ring does not offer the row on this target at all. False with no `gone`
fn order_gone(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool {
    g.tab.orders_order_def[d].row.gone.map_or(false, |f| f(g, seat, t, men))
}
/// the row stands in for the one its data names now. False with no `when`
fn order_when(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: &[GuardId]) -> bool {
    g.tab.orders_order_def[d].row.when.map_or(false, |f| f(g, seat, t, men))
}
/// `this.people?.ray(ro, rd, max) ?? null`: the man of the other side a cursor ray lands on nearer than `max`
fn squad_orders_people_ray(g: &crate::game::Game, seat: &crate::seats::SeatRef, ro: V3, rd: V3, max: f64) -> Option<GuardId> {
    if seat.orders.people.is_none() || !crate::pairorders::pair_orders_up(g, seat) { return None; }
    let (mut best, mut bt) = (None, max);
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if !crate::pairorders::pair_orders_is_foe(g, h) { continue; }
        let c = g.gd[h].ch;
        let t = crate::character::character_ray_hit(g, c, ro, rd, bt);
        if t >= 0.0 && t < bt { bt = t; best = Some(h); }
    }
    best
}

// ---- the bodies ----

/// What the cursor is on, before the whole-squad rule
pub(super) fn target_under_of(g: &Game, seat: &crate::seats::SeatRef, now: &crate::view::HostNow, asked: &mut Vec<(f64, f64)>) -> crate::orders::OrderTarget {
    if g.puppet.as_ref().map_or(false, |p| p.aim.is_some()) { return target_at(g, seat, seat.aim_point, asked); }
    if let Some(mk) = crate::rails::rails_mark_under(g, now) { return target_at(g, seat, mk, asked); }
    let (u, v) = (now.mouse[0] / m::max(1.0, now.client[0]), now.mouse[1] / m::max(1.0, now.client[1]));
    let ray = crate::camera::ray(&now.inv_view_proj, u, v);
    let (ro, rd) = (ray.ro, ray.rd);
    let pc = player_ch(g, seat);
    let own_id = g.ch[pc].id;
    let hit = crate::scene_boxes::box_world_raycast(g, ro, rd, Some(200.0), None, Some(own_id), None);
    let p: V3 = match &hit {
        Some(h) => v3::mad(ro, rd, h.t),
        None => match crate::camera::ground_point(&now.inv_view_proj, u, v, Some(0.0)) { Some(q) => q, None => seat.aim_point },
    };
    let mut man: Option<GuardId> = None;
    let mut best = match &hit { Some(h) => h.t + 0.3, None => 200.0 };
    for gd in squad_orders_squad(g, seat) {
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let c = g.gd[gd].ch;
        let t = crate::character::character_ray_hit(g, c, ro, rd, best);
        if t >= 0.0 && t < best { best = t; man = Some(gd); }
    }
    let fo = squad_orders_people_ray(g, seat, ro, rd, best);
    if let Some(foe) = fo {
        let q = g.ch[g.gd[foe].ch].pos;
        return OrderTarget::Foe(OrderTargetFoe { at: [q[0], 0.0, q[2]], foe });
    }
    if let Some(gd) = man {
        let q = g.ch[g.gd[gd].ch].pos;
        return OrderTarget::None(OrderTargetNone { at: [q[0], 0.0, q[2]], man: Some(gd) });
    }
    if let Some(h) = &hit {
        let b = crate::scene_boxes::box_world_at(g, h.index as usize);
        if let Some(wt) = crate::workthings::work_on(g, seat, b) { return wt; }
    }
    let self_to = match &hit { Some(h) => h.t, None => 200.0 };
    if crate::character::character_ray_hit(g, pc, ro, rd, self_to) >= 0.0 {
        let q = g.ch[pc].pos;
        return OrderTarget::None(OrderTargetNone { at: [q[0], 0.0, q[2]], man: None });
    }
    let base = target_at(g, seat, p, asked);
    if matches!(base, OrderTarget::Door(_)) || crate::character::character_ray_hit(g, pc, ro, rd, best) >= 0.0 { return base; }
    let first = hit.as_ref().map(|h| SquadOrdersSceneHitFirst { t: h.t, index: h.index });
    let sc = squad_orders_scene_hit(g, seat, ro, rd, first);
    let bx = if sc.index >= 0.0 { crate::scene_boxes::box_world_at(g, sc.index as usize) } else { None };
    let land = if sc.index >= 0.0 { v3::mad(ro, rd, sc.t) } else { p };
    let own = bx.and_then(|b| g.doors.list.iter().copied().find(|&d| AnyBoxId::World(g.dr[d].box_) == b));
    let leaf = match (own, bx) {
        (Some(d), _) => if crate::doors::door_off(g, d) { None } else { Some(d) },
        (None, Some(b)) => crate::fogofwar::fog_of_war_door_of_box(g, b),
        (None, None) => None,
    };
    if leaf.is_none() {
        if let OrderTarget::Floor(f) = &base { if f.thing.is_some() { return base; } }
    }
    let door = match leaf { Some(d) => Some(d), None => squad_orders_door_on_ray(g, ro, rd, Some(sc.t)) };
    if let Some(d) = door {
        let fc = g.dr[d].frame_centre;
        if Some(d) == leaf || v3::dist_xz(fc, land) <= g.k.orders.door_ray_reach_m {
            let q = g.ch[pc].pos;
            let side = crate::orders::side_of(g, d, q);
            return OrderTarget::Door(OrderTargetDoor { at: [fc[0], 0.0, fc[2]], door: d, side });
        }
    }
    base
}

/// What a cursor ray lands on of the scene, through glass and past the men
fn squad_orders_scene_hit(g: &crate::game::Game, seat: &crate::seats::SeatRef, ro: V3, rd: V3, first: Option<crate::orders::SquadOrdersSceneHitFirst>) -> crate::orders::SquadOrdersSceneHit {
    let f = crate::scene_boxes::box_flag();
    let skip = (js::to_i32(f.no_shadow) | js::to_i32(f.no_primary) | js::to_i32(f.glass)) as f64;
    if let Some(h) = first {
        if let Some(b) = crate::scene_boxes::box_world_at(g, h.index as usize) {
            if !js::truthy(seam_box_owner(g, b)) && (js::to_i32(seam_box_flags(g, b)) & js::to_i32(f.glass)) == 0 {
                return SquadOrdersSceneHit { t: h.t, index: h.index };
            }
        }
    }
    let mut t0 = 0.0;
    let pc = player_ch(g, seat);
    let mut owner = g.ch[pc].id;
    for k in 0..16 {
        let from = if k != 0 { v3::mad(ro, rd, t0) } else { ro };
        let Some(hit) = crate::scene_boxes::box_world_raycast(g, from, rd, Some(200.0), Some(skip), Some(owner), None) else { break };
        owner = match crate::scene_boxes::box_world_at(g, hit.index as usize) { Some(b) => seam_box_owner(g, b), None => 0.0 };
        t0 += hit.t;
        if !js::truthy(owner) { return SquadOrdersSceneHit { t: t0, index: hit.index }; }
    }
    SquadOrdersSceneHit { t: f64::INFINITY, index: -1.0 }
}

/// The rows that fit a target's kind on one page of the menu
pub fn squad_orders_rows_for(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget, page: Option<&str>, men: Option<&[GuardId]>) -> Vec<OrdersOrderDefId> {
    let kind = kind_of(t);
    let listed: Vec<OrdersOrderDefId> = seat.orders.defs.iter().copied().filter(|&d| {
        let r = &g.tab.orders_order_def[d];
        r.listed.as_ref().unwrap_or(&r.targets).contains(&kind) && r.page.as_deref() == page
    }).collect();
    let walled = crate::wallcharge::wall_rows(g, &crate::seats::here(g), t, page, &listed);
    let rows = crate::workthings::work_rows(g, &crate::seats::here(g), t, page, &walled);
    if !rows.iter().any(|&d| g.tab.orders_order_def[d].stands_in.is_some()) { return crate::rails::rails_order_rows(g, &rows); }
    let who = match men { Some(x) => x.to_vec(), None => recipients_of(g, &crate::seats::here(g), t) };
    let mut ins: Vec<OrdersOrderDefId> = Vec::new();
    for &d in &rows {
        if g.tab.orders_order_def[d].stands_in.is_some() && order_when(g, &crate::seats::here(g), d, t, &who) { ins.push(d); }
    }
    let mut out: JsSet<String> = JsSet::new();
    for &d in &ins {
        if let Some(s) = &g.tab.orders_order_def[d].stands_in { out.add(s.for_.clone()); }
    }
    let all: Vec<OrdersOrderDefId> = rows.iter().copied().filter(|&d| {
        let r = &g.tab.orders_order_def[d];
        if r.stands_in.is_some() { ins.contains(&d) } else { !out.has(r.id.clone()) }
    }).collect();
    crate::rails::rails_order_rows(g, &all)
}

/// A ring put up now is one of the flat tree
pub(super) fn squad_orders_flat_ring(g: &crate::game::Game) -> bool {
    !crate::rails::rails_live(g) || crate::rails::rails_flat_ring(g)
}

fn flat_walk(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget, men: Option<&[GuardId]>, who: &[GuardId], page: Option<&str>, out: &mut Vec<OrdersOrderDefId>, seen: &mut JsSet<String>) {
    let rows = squad_orders_rows_for(g, seat, t, page, men);
    for &d in &rows {
        let r = &g.tab.orders_order_def[d];
        if word(&r.opens) || out.iter().any(|&x| g.tab.orders_order_def[x].id == r.id) { continue; }
        if order_gone(g, &crate::seats::here(g), d, t, who) { continue; }
        out.push(d);
    }
    for &d in &rows {
        let opens = g.tab.orders_order_def[d].opens.clone();
        if let Some(p) = opens.filter(|p| !p.is_empty()) {
            if !seen.has(p.clone()) { seen.add(p.clone()); flat_walk(g, seat, t, men, who, Some(&p), out, seen); }
        }
    }
}

/// Every order of the target for a flat ring
fn squad_orders_flat_rows(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> Vec<OrdersOrderDefId> {
    let mut out: Vec<OrdersOrderDefId> = Vec::new();
    let mut seen: JsSet<String> = JsSet::new();
    let who = match men { Some(x) => x.to_vec(), None => recipients_of(g, &crate::seats::here(g), t) };
    flat_walk(g, seat, t, men, &who, None, &mut out, &mut seen);
    let kind = kind_of(t);
    if kind != TargetKind::None {
        let place = OrderTarget::None(OrderTargetNone { at: at_of(t), man: None });
        for d in squad_orders_rows_for(g, seat, &place, None, men) {
            let r = &g.tab.orders_order_def[d];
            if on(r.free) && !word(&r.opens) && !word(&r.hot_key) && r.targets.contains(&kind) && !out.iter().any(|&x| g.tab.orders_order_def[x].id == r.id) { out.push(d); }
        }
    }
    out
}

/// The rows of a menu that is up, read afresh
fn squad_orders_menu_rows(g: &crate::game::Game, seat: &crate::seats::SeatRef, m: &crate::orders::Menu, t: &crate::orders::OrderTarget) -> Vec<OrdersOrderDefId> {
    if on(m.flat) { squad_orders_flat_rows(g, seat, t, Some(&m.men)) } else { squad_orders_rows_for(g, seat, t, m.page.as_deref(), Some(&m.men)) }
}

/// A stand-in row put where the row it stands in for is
pub(super) fn squad_orders_seat(g: &mut crate::game::Game, def: OrdersOrderDefId, stood: OrdersOrderDefId) {
    let (key, hot, page) = { let s = &g.tab.orders_order_def[stood]; (s.key, s.hot_key.clone(), s.page.clone()) };
    let r = &mut g.tab.orders_order_def[def];
    r.key = key; r.hot_key = hot; r.page = page;
    let at = g.orders.defs.iter().position(|&d| d == stood).map_or(0, |i| i + 1);
    g.orders.defs.insert(at, def);
}

/// Is the 'wait for my word' row offered under these rows
fn squad_orders_word_row_of(g: &crate::game::Game, t: &crate::orders::OrderTarget, rows: &[OrdersOrderDefId]) -> bool {
    matches!(t, OrderTarget::Door(_)) && squad_orders_big(g, &crate::seats::here(g)) && rows.iter().any(|&d| on(g.tab.orders_order_def[d].on_word))
}

/// The open menu's page that has the 'wait for my word' row
fn squad_orders_word_page(g: &crate::game::Game, m: &crate::orders::Menu) -> Option<String> {
    if m.page.is_some() { return None; }
    for &d in &m.rows {
        let opens = g.tab.orders_order_def[d].opens.clone();
        if let Some(p) = opens.filter(|p| !p.is_empty()) {
            let rows = squad_orders_rows_for(g, &crate::seats::here(g), &m.target, Some(&p), Some(&m.men));
            if squad_orders_word_row_of(g, &m.target, &rows) { return Some(p); }
        }
    }
    None
}

/// The digit the open menu's 'wait for my word' row is on
fn squad_orders_wait_digit(g: &crate::game::Game, m: &crate::orders::Menu) -> f64 {
    let k = crate::orders::digit_of(&g.k.orders.wait_key);
    if m.word_row && k <= crate::orders::ORDER_DIGITS && !m.rows.iter().any(|&d| g.tab.orders_order_def[d].key == Some(k)) { k } else { 0.0 }
}

/// Why these men cannot carry that order out on that target
pub fn squad_orders_why_not(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget, men: Option<&[GuardId]>) -> Option<String> {
    let men: Vec<GuardId> = match men { Some(x) => x.to_vec(), None => recipients_of(g, &crate::seats::here(g), t) };
    if !g.tab.orders_order_def[d].targets.contains(&kind_of(t)) { return Some(String::from("not here.")); }
    if men.is_empty() { return Some(String::from("nobody to take it.")); }
    let mut first: Option<String> = None;
    let able: Vec<GuardId> = if g.tab.orders_order_def[d].row.can.is_some() {
        let mut a = Vec::with_capacity(men.len());
        for &gd in &men {
            match order_can(g, d, gd, t) {
                None => a.push(gd),
                Some(w) => { if first.is_none() { first = Some(w); } }
            }
        }
        a
    } else { men };
    if able.is_empty() { return Some(first.unwrap_or_else(|| String::from("not here."))); }
    if g.tab.orders_order_def[d].row.valid.is_none() { return None; }
    match order_valid(g, d, t, &able) {
        Ok(()) => None,
        Err(Some(w)) => Some(w),
        Err(None) => Some(String::from("not here.")),
    }
}

/// A row's words
pub fn squad_orders_label_of(g: &mut crate::game::Game, d: OrdersOrderDefId, t: &crate::orders::OrderTarget) -> String {
    order_label(g, d, t)
}

/// Issue an order by id; None when given, else why not
pub fn squad_orders_give(g: &mut crate::game::Game, id_2: &str, t: Option<&crate::orders::OrderTarget>, who: Option<crate::orders::Who>, opts: Option<&crate::orders::GiveOpts>) -> Option<String> {
    let t: OrderTarget = match t { Some(x) => x.clone(), None => { let a = g.aim_point; squad_orders_target_at(g, a) } };
    if !squad_orders_active(g, &crate::seats::here(g)) { return Some(String::from("no squad")); }
    let Some(def) = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == id_2) else { return Some(format!("no such order '{}'", id_2)); };
    if word(&g.tab.orders_order_def[def].opens) { return Some(format!("'{}' is a page of the menu, not an order", id_2)); }
    if !g.tab.orders_order_def[def].targets.contains(&kind_of(&t)) { return Some(format!("'{}' does not fit a {} target", id_2, kind_of(&t).as_str())); }
    let mut asked: Vec<GuardId> = match opts.and_then(|o| o.men.as_ref()) {
        Some(list) => {
            let mut a = Vec::with_capacity(list.len());
            for &gd in list { if !crate::condition::down(g, g.gd[gd].ch) && squad_orders_mine(g, &crate::seats::here(g), gd) { a.push(gd); } }
            a
        }
        None => squad_orders_recipients(g, &crate::seats::here(g), Some(&t), who),
    };
    if asked.is_empty() { return Some(String::from("nobody to take it")); }
    if g.tab.orders_order_def[def].row.only.is_some() {
        let o = order_only(g, def, &asked, &t);
        if !o.is_empty() { asked = o; }
    }
    g.orders.face_flip = false;
    let t = squad_orders_for_men(g, &t, &asked);
    let flipped = g.orders.face_flip;
    let mut cant: Vec<(GuardId, String)> = Vec::new();
    let all: Vec<GuardId> = if g.tab.orders_order_def[def].row.can.is_some() {
        let mut a = Vec::with_capacity(asked.len());
        for &gd in &asked {
            match order_can(g, def, gd, &t) {
                None => a.push(gd),
                Some(w) => cant.push((gd, w)),
            }
        }
        a
    } else { asked };
    let no = if !all.is_empty() { squad_orders_why_not(g, def, &t, Some(&all)) } else { Some(cant[0].1.clone()) };
    if let Some(no) = no.filter(|w| !w.is_empty()) {
        let by = if !all.is_empty() {
            let k = g.orders.acks;
            g.orders.acks += 1.0;
            all[js::rem(k, all.len() as f64) as usize]
        } else { cant[0].0 };
        crate::game::game_say(g, by, &no, None, None);
        return Some(no);
    }
    for (gd, w) in &cant { crate::game::game_say(g, *gd, w, None, None); }
    let mut why: Option<String> = None;
    let mut men: Vec<GuardId> = Vec::new();
    let free = on(g.tab.orders_order_def[def].free);
    for &gd in &all {
        let w = if !free && nightbreach_owns(g, gd) { nightbreach_release(g, gd) } else { None };
        match w { Some(w) if !w.is_empty() => why = Some(w), _ => men.push(gd) }
    }
    if men.is_empty() {
        let line = why.clone().unwrap_or_default();
        crate::interact::say_why(g, all.first().copied(), crate::view::ThingKind::Wall, &line, line.trim_end_matches('.'));
        return why;
    }
    g.orders.group += 1.0;
    let was_silent = crate::holdfire::hold_fire_silent(g, &crate::seats::here(g), id_2);
    if let Some(refused) = order_issue(g, def, &men, &t, opts) {
        crate::interact::say_why(g, men.first().copied(), crate::view::ThingKind::Wall, &refused, refused.trim_end_matches('.'));
        return Some(refused);
    }
    if men.len() > 1 { squad_orders_same_ways(g, &men); }
    if flipped { g.cleared.stats.face += 1.0; }
    if !free {
        let target = match &t {
            OrderTarget::Door(x) => format!("door:{}", g.dr[x.door].def.name),
            OrderTarget::Floor(x) => match &x.thing {
                Some(th) => format!("thing:{}", th.kind.as_str()),
                None => format!("floor:{},{}", js::to_fixed(x.at[0], 1), js::to_fixed(x.at[2], 1)),
            },
            OrderTarget::Foe(x) => format!("foe:{}", g.gd[x.foe].callsign),
            OrderTarget::None(_) => String::from("none"),
        };
        let serial = g.serial();
        let item = SquadOrdersLogItem { t: g.time, order: String::from(id_2), who: men.iter().map(|&gd| g.gd[gd].callsign.clone()).collect(), target, auto: None, serial };
        g.orders.log.push(item);
    }
    if g.orders.log.len() > 200 { g.orders.log.drain(0..50); }
    let ack = g.tab.orders_order_def[def].ack.clone().filter(|a| !a.is_empty());
    if let Some(ack) = ack {
        let k = g.orders.acks;
        g.orders.acks += 1.0;
        let lines: Vec<String> = match opts.and_then(|o| o.ack.clone()).filter(|a| !a.is_empty()) {
            Some(own) => vec![own],
            None => {
                let now = g.tab.orders_order_def[def].ack_now.clone().unwrap_or_else(|| ack.clone());
                crate::words::wg(g, &now, &ack)
            }
        };
        let n = men.len() as f64;
        let by = men[js::rem(k, n) as usize];
        let said = lines[js::rem(m::floor(k / n), lines.len() as f64) as usize].clone();
        crate::game::game_say(g, by, &said, None, None);
    }
    if g.orders.posture.is_some() {
        let loud = crate::orders::rows::id_is(id_2, crate::orders::Is::Loud);
        crate::silent::silent_tool(g, loud);
    }
    let waits = opts.map_or(false, |o| on(o.on_word)) && squad_orders_big(g, &crate::seats::here(g));
    crate::holdfire::hold_fire_order(g, id_2, Some(crate::holdfire::HoldFireOrderO { waits: Some(waits), silent: Some(was_silent), ..Default::default() }));
    None
}

/// An order asked again of men who already stand on it
pub fn squad_orders_again(g: &mut crate::game::Game, id_2: &str, men: &[GuardId], lines: &[String]) {
    if !g.orders.defs.iter().any(|&d| g.tab.orders_order_def[d].id == id_2) { return; }
    if men.is_empty() || lines.is_empty() { return; }
    let k = g.orders.acks;
    g.orders.acks += 1.0;
    let by = men[js::rem(k, men.len() as f64) as usize];
    let said = &lines[js::rem(k, lines.len() as f64) as usize];
    crate::game::game_say(g, by, said, None, None);
}

/// Hand a man his assignment
pub fn squad_orders_assign(g: &mut crate::game::Game, gd: GuardId, a: &crate::orders::SquadOrdersAssignA) {
    let p = g.ch[g.gd[gd].ch].pos;
    let t = g.time;
    let was = g.orders.asg.get(gd).copied();
    let fighting = g.gd[gd].script.is_none() && g.gd[gd].state == crate::game::GuardState::Alert;
    crate::control::set_pinned(g, gd, false, "orders");
    crate::control::set_hold(g, gd, false, "orders");
    g.orders.ours.delete(gd);
    let r = a.post.map(|post| squad_orders_route(g, gd, post, a.shy, a.thru, None));
    let via: Vec<V3> = r.as_ref().map_or(Vec::new(), |x| x.via.clone());
    match r.as_ref().and_then(|x| x.how) {
        Some(SquadOrdersRouteHow::Round) => g.cleared.stats.round += 1.0,
        Some(SquadOrdersRouteHow::Through) => g.cleared.stats.through += 1.0,
        Some(SquadOrdersRouteHow::Mouth) => g.cleared.stats.mouth += 1.0,
        None => {}
    }
    let light = match &g.gd[gd].script {
        Some(s) => s.light,
        None => was.and_then(|w| g.tab.orders_assignment[w].light),
    };
    let yield_t = match was { Some(w) if fighting => g.tab.orders_assignment[w].yield_t, _ => -1.0 };
    let goal = if !via.is_empty() { via[0] } else { match a.post { Some(q) => v3::copy(q), None => [p[0], 0.0, p[2]] } };
    let row = Assignment {
        order: a.order.clone(),
        t0: t,
        post: a.post,
        follow: a.follow.unwrap_or(0.0),
        face: a.face,
        watch: a.watch,
        door: a.door,
        strafe: a.strafe,
        stance: a.stance.unwrap_or(crate::character::Stance::LowReady),
        upper: a.upper.unwrap_or(crate::character::UpperMode::None),
        stack_side: a.stack_side.unwrap_or(1.0),
        move_: a.move_.clone(),
        going: a.going.clone(),
        there: a.there.clone(),
        set_line: a.set_line.clone(),
        group: g.orders.group,
        set_said: false,
        goal,
        arrived: false,
        held_set: a.held_set,
        held_fire: a.held_fire,
        arr_t: -1.0,
        moved_t: t,
        light: a.light.or(light),
        low: false,
        yield_t,
        face_now: None,
        search: a.search,
        search_t: a.search_t,
        via,
        anchor: [p[0], 0.0, p[2]],
        mark: [p[0], 0.0, p[2]],
        mark_t: t,
        had: a.had,
        back_n: a.back_n,
        shy: a.shy,
        thru: a.thru,
        via_for: a.via_for,
        via_t: a.via_t,
        pace: a.pace,
        spare: a.spare,
        from: a.from,
        foe: a.foe,
        crouch: a.crouch,
        lean: a.lean,
        fit: a.fit.clone(),
        tries: a.tries.clone(),
        new_leg: true,
    };
    let id = g.tab.orders_assignment.push(row);
    g.orders.asg.set(gd, id);
}

fn post_dist(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId, centre: V3) -> f64 {
    let a = *seat.orders.asg.get(gd).unwrap();
    v3::dist_xz(g.tab.orders_assignment[a].post.unwrap(), centre)
}

/// Rear security at a door: the last of the file have their eyes spare
pub fn squad_orders_spare_rear(g: &mut crate::game::Game, men: &[GuardId], door: DoorId, lead: Option<GuardId>, from: Option<&JsMap<GuardId, V3>>) -> Vec<GuardId> {
    let live: Vec<GuardId> = men.iter().copied().filter(|&gd| !crate::condition::down(g, g.gd[gd].ch) && g.orders.asg.get(gd).map_or(false, |&a| g.tab.orders_assignment[a].post.is_some())).collect();
    for &gd in &live {
        let a = *g.orders.asg.get(gd).unwrap();
        g.tab.orders_assignment[a].spare = Some(false);
    }
    if live.len() < 2 { return Vec::new(); }
    let centre = g.dr[door].frame_centre;
    let first = match lead.filter(|l| live.contains(l)) {
        Some(l) => l,
        None => {
            let mut best = live[0];
            for &b in &live[1..] { if post_dist(g, &crate::seats::here(g), b, centre) < post_dist(g, &crate::seats::here(g), best, centre) { best = b; } }
            best
        }
    };
    let mut rear: Vec<GuardId> = live.iter().copied().filter(|&gd| gd != first).collect();
    let place = |gd: GuardId| live.iter().position(|&x| x == gd).unwrap() as f64;
    {
        let g2: &Game = &*g;
        js::sort_by(&mut rear, |&a, &b| js::or(post_dist(g2, &crate::seats::here(g2), b, centre) - post_dist(g2, &crate::seats::here(g2), a, centre), place(b) - place(a)));
    }
    rear.truncate(if live.len() >= 4 { 2 } else { 1 });
    for &gd in &rear {
        let a = *g.orders.asg.get(gd).unwrap();
        let row = &mut g.tab.orders_assignment[a];
        row.spare = Some(true);
        if row.from.is_none() {
            row.from = Some(match from.and_then(|f| f.get(gd)) { Some(&q) => q, None => [row.anchor[0], 0.0, row.anchor[2]] });
        }
    }
    rear
}

/// A memory save put back: a ring and a press half made are the hand's, not the saved moment's
pub fn squad_orders_loaded(g: &mut crate::game::Game) {
    if g.orders.cmd_ring.is_some() { g.orders.menu = None; g.orders.cmd_ring = None; }
    g.orders.cmd_press = None;
    g.orders.tap_was = None;
}

/// Esc: the menu steps back a page, or is dismissed
pub fn squad_orders_escape(g: &mut crate::game::Game) -> bool {
    if g.orders.menu.is_none() { return false; }
    if g.orders.cmd_ring.is_some() {
        g.orders.menu = None;
        g.orders.cmd_ring = None;
        g.stats.cmdring.cancelled += 1;
    } else if !squad_orders_page_back(g) {
        g.orders.menu = None;
    }
    g.orders.esc_used = true;
    true
}

/// Open the menu on a target
pub fn squad_orders_open(g: &mut crate::game::Game, target: Option<&crate::orders::OrderTarget>, flat: Option<bool>, part: Option<&[GuardId]>) -> bool {
    let target: OrderTarget = match target { Some(x) => x.clone(), None => { let a = g.aim_point; squad_orders_target_at(g, a) } };
    let flat = flat.unwrap_or(false);
    if !squad_orders_active(g, &crate::seats::here(g)) || crate::condition::down(g, g.pl[g.player].ch) { return false; }
    let rows = if flat { squad_orders_flat_rows(g, &crate::seats::here(g), &target, part) } else { squad_orders_rows_for(g, &crate::seats::here(g), &target, None, part) };
    let word_row = squad_orders_word_row_of(g, &target, &rows);
    // (THE MENU'S OWN TARGET IS KNOWN BY IDENTITY: wallcharge.ts keptFor asks `M.target === t` before it keeps a walk for the open menu. A floor target
    //  carries a serial for that, and this is where the one a menu keeps is dealt; the rows above were asked before there was this menu, as in the
    //  TypeScript, so they are not kept. Without it no walk of an open menu was ever kept: the same answers, worked again at every re-read.)
    let mut target = target;
    if let OrderTarget::Floor(f) = &mut target { if f.serial == 0 { f.serial = g.serial(); } }
    g.orders.menu = Some(Menu {
        target,
        men: Vec::new(),
        rows,
        why: Vec::new(),
        sel: 0.0,
        wheel_acc: 0.0,
        read_t: -1.0,
        part: part.map(|x| x.to_vec()),
        split: None,
        page: None,
        word_row,
        on_word: false,
        men_side: None,
        side_key: String::new(),
        flat: if flat { Some(true) } else { None },
        words: Vec::new(),
        quick: None,
    });
    squad_orders_refresh(g);
    g.orders.menu.is_some()
}

pub fn squad_orders_close(g: &mut crate::game::Game) {
    g.orders.menu = None;
}

/// The open menu turned to a page of it
pub fn squad_orders_turn(g: &mut crate::game::Game, page: Option<&str>, on_word: Option<bool>) {
    let on_word = on_word.unwrap_or(false);
    let Some(mut m) = g.orders.menu.clone() else { return };
    let from = m.page.clone();
    m.page = page.map(String::from);
    if let Some(up) = g.orders.menu.as_mut() { up.page = m.page.clone(); }
    let rows = squad_orders_rows_for(g, &crate::seats::here(g), &m.target, page, Some(&m.men));
    let word_row = squad_orders_word_row_of(g, &m.target, &rows);
    let opener = from.as_ref().and_then(|f| g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].opens.as_ref() == Some(f)));
    let mut sel: Option<usize> = rows.iter().position(|&d| { let o = &g.tab.orders_order_def[d].opens; word(o) && o.as_ref() == from.as_ref() });
    if sel.is_none() {
        if let Some(op) = opener {
            let (key, op_page) = (g.tab.orders_order_def[op].key, g.tab.orders_order_def[op].page.clone());
            if key.is_some() { sel = rows.iter().position(|&d| g.tab.orders_order_def[d].key == key && g.tab.orders_order_def[d].page == op_page); }
        }
    }
    if let Some(up) = g.orders.menu.as_mut() {
        up.rows = rows;
        up.word_row = word_row;
        up.on_word = up.on_word || (word_row && on_word);
        up.sel = sel.map_or(0.0, |i| i as f64);
        up.wheel_acc = 0.0;
    }
    if let Some(r) = g.orders.cmd_ring.as_mut() { r.lit = 0.0; r.armed = false; r.more = Some(false); }
    squad_orders_refresh(g);
}

/// A page of the open menu → the page it was opened from
pub(super) fn squad_orders_page_back(g: &mut crate::game::Game) -> bool {
    let Some(page) = g.orders.menu.as_ref().and_then(|m| m.page.clone()) else { return false };
    let back = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].opens.as_ref() == Some(&page)).and_then(|d| g.tab.orders_order_def[d].page.clone());
    squad_orders_turn(g, back.as_deref(), None);
    true
}

/// Give the open menu's row `i` and take the menu down
pub fn squad_orders_pick(g: &mut crate::game::Game, i: Option<f64>) -> Option<String> {
    let i = match i { Some(x) => x, None => g.orders.menu.as_ref().map_or(-1.0, |m| m.sel) };
    let Some(m) = g.orders.menu.as_mut() else { return Some(String::from("no menu")); };
    if m.word_row && i == m.rows.len() as f64 {
        m.on_word = !m.on_word;
        return Some(String::from(if m.on_word { "on the word" } else { "at once" }));
    }
    let row = row_at(&m.rows, i);
    let page = row.and_then(|d| g.tab.orders_order_def[d].opens.clone()).filter(|p| !p.is_empty());
    if let Some(p) = page {
        squad_orders_turn(g, Some(&p), None);
        return Some(String::from(if g.orders.menu.is_some() { "opened" } else { "nobody to take it" }));
    }
    let there = squad_orders_reread(g);
    let m = g.orders.menu.take();
    if !there { return Some(String::from("nobody to take it")); }
    let Some(m) = m else { return Some(String::from("nobody to take it")); };
    if i < 0.0 || i >= m.rows.len() as f64 { return Some(String::from("cancelled")); }
    let d = m.rows[i as usize];
    let id = g.tab.orders_order_def[d].id.clone();
    let on_word = on(g.tab.orders_order_def[d].on_word) && m.on_word;
    if m.part.is_some() {
        let opts = GiveOpts { men: Some(m.men.clone()), on_word: if on_word { Some(true) } else { None }, ack: None };
        let target = m.target.clone();
        let mut give = |g: &mut Game| squad_orders_give(g, &id, Some(&target), None, Some(&opts));
        return squad_orders_part_give(g, &m, &mut give);
    }
    let opts = GiveOpts { ack: None, on_word: Some(true), men: None };
    let no = squad_orders_give(g, &id, Some(&m.target), None, if on_word { Some(&opts) } else { None });
    no
}

/// An order of the ring that adds a goal, given by hand to its men alone
pub(super) fn squad_orders_part_give(g: &mut crate::game::Game, m: &crate::orders::Menu, give: &mut dyn FnMut(&mut crate::game::Game) -> Option<String>) -> Option<String> {
    let n = g.orders.group;
    let no = give(g);
    if g.orders.group != n {
        g.stats.planmode.goals += 1;
        if on(m.split) { g.stats.planmode.split += 1; }
    }
    no
}

/// The open menu's target as its men will work it
pub(super) fn squad_orders_menu_target(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::orders::OrderTarget> {
    seat.orders.menu.as_ref().map(menu_target_of)
}

/// The open menu's cheap half, re-read against the floor as it now stands
fn squad_orders_reread(g: &mut crate::game::Game) -> bool {
    let Some(t) = g.orders.menu.as_ref().map(|m| m.target.clone()) else { return false };
    if let OrderTarget::None(x) = &t {
        if let Some(man) = x.man {
            if crate::condition::down(g, g.gd[man].ch) { g.orders.menu = None; return false; }
        }
    }
    let mut t = t;
    if let OrderTarget::Door(x) = &mut t {
        let q = g.ch[player_ch(g, &crate::seats::here(g))].pos;
        x.side = crate::orders::side_of(g, x.door, q);
        if let Some(m) = g.orders.menu.as_mut() { m.target = OrderTarget::Door(*x); }
    }
    if let Some(part) = g.orders.menu.as_ref().and_then(|m| m.part.clone()) {
        let mut men = Vec::with_capacity(part.len());
        for gd in part { if !crate::condition::down(g, g.gd[gd].ch) && squad_orders_mine(g, &crate::seats::here(g), gd) { men.push(gd); } }
        let none = men.is_empty();
        if let Some(m) = g.orders.menu.as_mut() { m.men = men; }
        if none { g.orders.menu = None; return false; }
        return true;
    }
    let men = recipients_of(g, &crate::seats::here(g), &t);
    if let Some(m) = g.orders.menu.as_mut() { m.men = men; }
    true
}

/// … and the whole of it: each row's why-not as well
pub(super) fn squad_orders_refresh(g: &mut crate::game::Game) {
    if g.orders.menu.is_none() || !squad_orders_reread(g) { return; }
    crate::squadthrow::plan::forget(g);
    let Some(m) = g.orders.menu.clone() else { return };
    let door_up = match &m.target { OrderTarget::Door(x) => if squad_orders_big(g, &crate::seats::here(g)) && !m.men.is_empty() { Some(*x) } else { None }, _ => None };
    if let Some(x) = door_up {
        let mut key = format!("{}|", x.side);
        for (n, &gd) in m.men.iter().enumerate() {
            let c = g.gd[gd].ch;
            let (id, q) = (g.ch[c].id, g.ch[c].pos);
            let side = crate::orders::side_of(g, x.door, q);
            if n > 0 { key.push(','); }
            key.push_str(&format!("{}{}", id, if side > 0.0 { '+' } else { '-' }));
        }
        if key != m.side_key {
            if let Some(up) = g.orders.menu.as_mut() { up.side_key = key; }
            let side = squad_orders_side_for(g, x.door, &m.men, x.side);
            if let Some(up) = g.orders.menu.as_mut() { up.men_side = Some(side); }
        }
    } else if let Some(up) = g.orders.menu.as_mut() {
        up.men_side = None;
        up.side_key = String::new();
    }
    let Some(m) = g.orders.menu.clone() else { return };
    let mt = menu_target_of(&m);
    let rows = squad_orders_menu_rows(g, &crate::seats::here(g), &m, &mt);
    if rows != m.rows {
        let was = row_at(&m.rows, m.sel);
        if let Some(up) = g.orders.menu.as_mut() { up.rows = rows.clone(); }
        let word_row = squad_orders_word_row_of(g, &m.target, &rows);
        let i: Option<usize> = match was {
            None => None,
            Some(w) if on(m.flat) => rows.iter().position(|&d| d == w),
            Some(w) => {
                let wk = g.tab.orders_order_def[w].key;
                rows.iter().position(|&d| d == w).or_else(|| rows.iter().position(|&d| wk.is_some() && g.tab.orders_order_def[d].key == wk))
            }
        };
        let sel = match i { Some(x) => x as f64, None => m::min(m.sel, rows.len() as f64 + if word_row { 1.0 } else { 0.0 }) };
        if let Some(up) = g.orders.menu.as_mut() { up.word_row = word_row; up.sel = sel; }
    }
    let Some(m) = g.orders.menu.clone() else { return };
    let later: Option<Vec<f64>> = if on(m.flat) && !g.orders.cmd_ring.as_ref().map_or(false, |r| on(r.more)) {
        Some(squad_orders_seats(g, &m, None).over)
    } else { None };
    let mut why: Vec<Option<String>> = Vec::with_capacity(m.rows.len());
    for (i, &d) in m.rows.iter().enumerate() {
        if later.as_ref().map_or(false, |l| l.contains(&(i as f64))) { why.push(None); continue; }
        let o = if g.tab.orders_order_def[d].row.only.is_some() { order_only(g, d, &m.men, &mt) } else { m.men.clone() };
        let to: &[GuardId] = if o.is_empty() { &m.men } else { &o };
        why.push(squad_orders_why_not(g, d, &mt, Some(to)));
    }
    let now = g.time;
    if let Some(up) = g.orders.menu.as_mut() { up.why = why; up.read_t = now; }
    super::read_words(g);
}

/// Is the open menu's list stale now
pub(super) fn squad_orders_due(g: &crate::game::Game, m: &crate::orders::Menu) -> bool {
    if let Some(p) = &m.page {
        if g.orders.defs.iter().any(|&d| { let r = &g.tab.orders_order_def[d]; r.opens.as_ref() == Some(p) && r.stands_in.is_some() }) { return true; }
    }
    let t = menu_target_of(m);
    squad_orders_menu_rows(g, &crate::seats::here(g), m, &t) != m.rows
}

/// What the cursor is on, for a HUD (None: no squad answers to this seat)
pub fn hover(g: &Game, seat: &crate::seats::SeatRef, now: &crate::view::HostNow) -> Option<crate::orders::OrderTarget> {
    if !squad_orders_active(g, seat) { return None; }
    Some(target_under(g, seat, now, &mut Vec::new()))
}
/// The order the quick key gives on a target: its first row
pub fn squad_orders_default_for(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget) -> Option<OrdersOrderDefId> {
    squad_orders_rows_for(g, seat, t, None, None).into_iter().find(|&d| !word(&g.tab.orders_order_def[d].opens))
}

/// What T would make of `who` with the cursor on `point`
fn squad_orders_who_after_t(g: &mut crate::game::Game, point: Option<&crate::orders::OrderTarget>, dir: Option<f64>) -> crate::orders::Who {
    let dir = dir.unwrap_or(1.0);
    let s = squad_orders_squad(g, &crate::seats::here(g));
    let man = match point { Some(OrderTarget::None(x)) => x.man, _ => None };
    let k: Option<usize> = match man { Some(gd) if !crate::condition::down(g, g.gd[gd].ch) => s.iter().position(|&x| x == gd), _ => None };
    let menu_man = match g.orders.menu.as_ref().map(|m| &m.target) { Some(OrderTarget::None(x)) => x.man, _ => None };
    if let Some(k) = k {
        let same = matches!(g.orders.who, Who::Num(n) if n == k as f64);
        return if (same && menu_man.is_none()) || man == menu_man { Who::All } else { Who::Num(k as f64) };
    }
    if menu_man.is_some() { return Who::All; }
    let was = g.orders.who;
    let next = squad_orders_cycle_who(g, Some(dir));
    g.orders.who = was;
    next
}

/// Who the next order goes to is stepped, and a menu that is up is theirs from here on
pub fn squad_orders_step_who(g: &mut crate::game::Game, dir: f64, point: Option<&crate::orders::OrderTarget>) {
    g.orders.who = squad_orders_who_after_t(g, point, Some(dir));
    let Some(m) = g.orders.menu.as_mut() else { return };
    if let OrderTarget::None(x) = &m.target {
        if x.man.is_some() { m.target = OrderTarget::None(OrderTargetNone { at: x.at, man: None }); }
    }
    squad_orders_refresh(g);
}

/// After a tick: whoever asks between ticks is answered off the live input and camera
pub fn squad_orders_host_view(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    if g.orders.view.input.is_some() { set_view(g, input, cam, canvas); }
}

/// Once a frame right after the aim is known: T, the press that opens or closes the menu, and the open menu's keys, each taken out of the frame's input
pub fn squad_orders_input(g: &mut crate::game::Game, _dt: f64, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    squad_orders_sync(g);
    if g.orders.esc_used { g.orders.esc_used = false; press_delete(g, input, "Escape"); }
    if !squad_orders_active(g, &crate::seats::here(g)) || crate::condition::down(g, g.pl[g.player].ch) {
        g.orders.menu = None;
        g.orders.cmd_ring = None;
        g.orders.cmd_press = None;
        g.orders.tap_was = None;
        g.orders.view.input = None;
        g.orders.view.cam = None;
        g.orders.view.canvas = None;
        return;
    }
    if g.orders.tap_was.is_some() && (press_has(g, input, "Blur") || press_has(g, input, "Thawed")) { g.orders.tap_was = None; }
    set_view(g, input, cam, canvas);
    if squad_orders_whole(g, &crate::seats::here(g)) {
        g.orders.who = Who::All;
    } else {
        let who_key = g.k.orders.who_key.clone();
        if crate::input::input_hit(g, input, &who_key) {
            let t = squad_orders_target_under(g, input, cam, canvas);
            squad_orders_step_who(g, 1.0, Some(&t));
        }
    }
    crate::planmode::goal_input(g, input, cam, canvas);
    if squad_orders_ring_input(g, input, cam, canvas) { return; }
    let Some(m) = g.orders.menu.clone() else {
        let quick = g.k.orders.quick_key.clone();
        if crate::input::input_hit(g, input, &quick) {
            let t = squad_orders_target_under(g, input, cam, canvas);
            squad_orders_tap_give(g, &t, input, None);
        }
        return;
    };
    let esc = crate::backkey::back_take(g, input);
    let blur = press_has(g, input, "Blur");
    let space = press_delete(g, input, crate::consts::INTERACT_KEY);
    let rmb = crate::input::input_rmb_hit(g, input);
    if rmb { unclick(g, input, 4); }
    let lmb = crate::input::input_lmb_hit(g, input);
    if g.time - m.read_t >= g.k.orders.refresh_secs || squad_orders_due(g, &m) {
        squad_orders_refresh(g);
        if g.orders.menu.is_none() { return; }
    }
    let Some(m) = g.orders.menu.clone() else { return };
    let wait_d = squad_orders_wait_digit(g, &m);
    let word_page = squad_orders_word_page(g, &m);
    let (mut digit, mut wait) = (0.0, false);
    for d in 1..=9 {
        if press_delete(g, input, &format!("Digit{}", d)) {
            let d = d as f64;
            if d == wait_d { wait = true; } else if digit == 0.0 { digit = d; }
        }
    }
    let wait_key = g.k.orders.wait_key.clone();
    if !js::truthy(crate::orders::digit_of(&wait_key)) && press_delete(g, input, &wait_key) && m.word_row { wait = true; }
    let wait_alt = g.k.orders.wait_alt.clone();
    let alt = (m.word_row || word_page.is_some()) && !wait_alt.is_empty() && press_delete(g, input, &wait_alt);
    let mut d_sel = 0.0;
    if press_delete(g, input, "ArrowDown") { d_sel += 1.0; }
    if press_delete(g, input, "ArrowUp") { d_sel -= 1.0; }
    let mut wheel = 0.0;
    let turned = g.tab.input[input].wheel;
    if js::truthy(turned) {
        let mut acc = m::max(-2.0 * WHEEL_STEP, m::min(2.0 * WHEEL_STEP, m.wheel_acc + turned));
        g.tab.input[input].wheel = 0.0;
        if m::abs(acc) >= WHEEL_STEP {
            wheel = m::sign(acc);
            acc -= wheel * WHEEL_STEP;
        }
        if let Some(up) = g.orders.menu.as_mut() { up.wheel_acc = acc; }
    }
    if lmb && matches!(g.pl[g.player].holding.as_ref().map(|h| h.variant), Some(crate::character::HoldVariant::Arm)) { unclick(g, input, 1); }
    if blur || lmb { g.orders.menu = None; return; }
    if esc || rmb {
        if !squad_orders_page_back(g) { g.orders.menu = None; }
        return;
    }
    if alt {
        if let Some(p) = &word_page { squad_orders_turn(g, Some(p), Some(true)); return; }
    }
    if wait || alt { squad_orders_pick(g, Some(m.rows.len() as f64)); }
    if digit != 0.0 {
        let found = m.rows.iter().position(|&r| g.tab.orders_order_def[r].key == Some(digit));
        if let Some(i) = found { squad_orders_pick(g, Some(i as f64)); return; }
    }
    d_sel += wheel;
    let Some(up) = g.orders.menu.as_mut() else { return };
    let n = up.rows.len() as f64 + if up.word_row { 1.0 } else { 0.0 };
    if js::truthy(d_sel) { up.sel = js::rem(js::rem(up.sel + d_sel, n + 1.0) + (n + 1.0), n + 1.0); }
    let sel = up.sel;
    if space { squad_orders_pick(g, Some(sel)); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed_as_the_language_has_it() {
        // bun: x.toFixed(1) for each
        for (x, s) in [(0.25, "0.3"), (-0.25, "-0.3"), (0.75, "0.8"), (1.25, "1.3"), (0.35, "0.3"), (1.45, "1.4"), (-0.04, "-0.0"), (0.0, "0.0"), (-0.0, "0.0"), (2.5, "2.5"), (12.05, "12.1"), (-12.15, "-12.2"), (100.125, "100.1"), (1e-9, "0.0")] {
            assert_eq!(js::to_fixed(x, 1), s, "{}", x);
        }
    }

    #[test]
    fn a_door_menu_works_from_the_mens_face() {
        let d = OrderTargetDoor { at: [1.0, 0.0, 2.0], door: DoorId(3), side: 1.0 };
        let mut m = Menu::default();
        m.target = OrderTarget::Door(d);
        m.men_side = Some(-1.0);
        match menu_target_of(&m) { OrderTarget::Door(x) => assert_eq!(x.side, -1.0), _ => panic!() }
        m.men_side = None;
        match menu_target_of(&m) { OrderTarget::Door(x) => assert_eq!(x.side, 1.0), _ => panic!() }
    }

    #[test]
    fn a_row_by_a_number() {
        let rows = [OrdersOrderDefId(0), OrdersOrderDefId(1)];
        assert!(row_at(&rows, -1.0).is_none());
        assert!(row_at(&rows, 2.0).is_none());
        assert!(row_at(&rows, 1.0) == Some(OrdersOrderDefId(1)));
    }
}
