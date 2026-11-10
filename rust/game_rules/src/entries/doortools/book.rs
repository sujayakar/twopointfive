use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- where a door is

/// doorPt's arithmetic: `th` the door's closedDir, `h` its hinge
fn frame_pt(th: f64, h: [f64; 2], a: f64, b: f64, side: f64, y: f64) -> V3 {
    let (ux, uz) = (m::cos(th), m::sin(th));
    [h[0] + ux * a - uz * side * b, y, h[1] + uz * a + ux * side * b]
}
/// alongOf's arithmetic
fn frame_along(th: f64, h: [f64; 2], p: V3) -> f64 {
    (p[0] - h[0]) * m::cos(th) + (p[2] - h[1]) * m::sin(th)
}
/// offOf's arithmetic
fn frame_off(th: f64, h: [f64; 2], p: V3, side: f64) -> f64 {
    (m::cos(th) * (p[2] - h[1]) - m::sin(th) * (p[0] - h[0])) * side
}
/// a point in a door's frame: `a` m along hinge → latch, `b` out from the frame line on face `side`
pub fn door_pt(g: &crate::game::Game, d: DoorId, a: f64, b: f64, side: f64, y: Option<f64>) -> V3 {
    frame_pt(g.dr[d].def.closed_dir, g.dr[d].def.hinge, a, b, side, y.unwrap_or(0.0))
}
/// metres along the frame, hinge → latch
pub fn along_of(g: &crate::game::Game, d: DoorId, p: V3) -> f64 {
    frame_along(g.dr[d].def.closed_dir, g.dr[d].def.hinge, p)
}
/// metres off the frame line on face `side` (negative: the other face)
pub fn off_of(g: &crate::game::Game, d: DoorId, p: V3, side: f64) -> f64 {
    frame_off(g.dr[d].def.closed_dir, g.dr[d].def.hinge, p, side)
}
/// unlatched and swung past OPEN_RAD
pub fn is_open(g: &crate::game::Game, d: DoorId) -> bool {
    !crate::doors::door_latched(g, d) && m::abs(crate::doors::door_angle(g, d)) >= OPEN_RAD
}
/// is most of a room's floor under the sky (asked once per room, of every cell or about 64 of them)
fn under_sky(g: &mut crate::game::Game, room: RoomfightRoomId) -> bool {
    let size = g.tab.roomfight_room[room].cells.len() as f64;
    if let Some(hit) = g.m.doortools.sky.get(room) {
        if hit.n == size { return hit.open; }
    }
    let (w, cs) = (g.col.nav.w, g.col.nav.cell);
    let every = m::max(1.0, m::floor(size / 64.0));
    let (mut k, mut of, mut sky) = (0.0, 0.0, 0.0);
    // the walk asks only the walls: it cannot change the room's cells
    for c in g.tab.roomfight_room[room].cells.keys() {
        let ci = c.0;
        let at = k;
        k += 1.0;
        if js::truthy(js::rem(at, every)) { continue; }
        let x = (js::rem(ci, w) + 0.5) * cs;
        let z = (js::to_i32(ci / w) as f64 + 0.5) * cs;
        of += 1.0;
        if !crate::scene_collision::solid_blocked(g, [x, 2.0, z], [x, 9.0, z], None) { sky += 1.0; }
    }
    let open = sky * 2.0 > of;
    g.m.doortools.sky.set(room, crate::doortools::DoortoolsModSKYValue { n: size, open });
    open
}
/// open floor, not a room: a flood past `searchCells`, or floor under the sky
pub(super) fn open_floor(g: &mut crate::game::Game, room: RoomfightRoomId) -> bool {
    g.tab.roomfight_room[room].cells.len() as f64 > g.k.doortools.search_cells || under_sky(g, room)
}

// ---------------------------------------------------------------- what the rows' functions share

/// `t` as a door: the door and the face
fn door_of(t: &crate::orders::OrderTarget) -> Option<(DoorId, f64)> {
    match t {
        crate::orders::OrderTarget::Door(x) => Some((x.door, x.side)),
        _ => None,
    }
}
/// a string list of a row
fn strs(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|x| String::from(*x)).collect()
}
/// S.register(def, before): a row is a thing of the orders' table
fn register(g: &mut Game, def: crate::orders::OrderDef, before: Option<&str>) {
    let id = crate::orders::squad_orders_row(g, def);
    crate::orders::squad_orders_register(g, id, before);
}

// ---------------------------------------------------------------- what more than one door order asks (each order's own functions are its file in orders/rows/)

fn tt_of(t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsThresholdThenT {
    use crate::doortools::DoorToolsThresholdThenT as T;
    match t { crate::orders::OrderTarget::Floor(x) => T::Floor(x.clone()), crate::orders::OrderTarget::Door(x) => T::Door(x.clone()), crate::orders::OrderTarget::None(x) => T::None(x.clone()), crate::orders::OrderTarget::Foe(x) => T::Foe(x.clone()) }
}
fn ct_of(t: &crate::orders::OrderTarget) -> crate::doortools::DoorToolsCanThresholdThenT {
    use crate::doortools::DoorToolsCanThresholdThenT as T;
    match t { crate::orders::OrderTarget::Floor(x) => T::Floor(x.clone()), crate::orders::OrderTarget::Door(x) => T::Door(x.clone()), crate::orders::OrderTarget::None(x) => T::None(x.clone()), crate::orders::OrderTarget::Foe(x) => T::Foe(x.clone()) }
}
/// `stackable(t, men)` as a row's `valid` answers it
pub fn row_stackable(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match crate::doortools::door_tools_stackable(g, t, men) {
        crate::doortools::DoorToolsStackable::Text(w) => Err(Some(w)),
        crate::doortools::DoorToolsStackable::Bool(true) => Ok(()),
        crate::doortools::DoorToolsStackable::Bool(false) => Err(None),
    }
}
/// `canThresholdThen(what, t, men)` as a row's `valid` answers it
pub fn row_can_then(g: &mut Game, what: crate::doortools::DoorToolsCanThresholdThenWhat, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    match crate::doortools::door_tools_can_threshold_then(g, what, Some(&ct_of(t)), men) {
        crate::doortools::DoorToolsCanThresholdThen::Text(w) => Err(Some(w)),
        crate::doortools::DoorToolsCanThresholdThen::Bool(true) => Ok(()),
        crate::doortools::DoorToolsCanThresholdThen::Bool(false) => Err(None),
    }
}
/// `who === 'you' ? wg(…) : `${who} is behind that door.``
pub fn row_behind_words(g: &Game, who: String) -> String {
    if who == "you" { let (now, was) = (String::from("you're behind that door."), String::from("you are behind that door.")); crate::words::wg(g, &now, &was) } else { format!("{} is behind that door.", who) }
}
/// an entry's row: of a door whose jambs are held, what the men on them do; of any other, its own order
pub fn row_entry(g: &mut Game, bang: bool, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    use crate::doortools::{DoorToolKind as Kind, DoorToolsThresholdThen, DoorToolsThresholdThenWhat};
    if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, men) {
        return match crate::doortools::door_tools_threshold_then(g, if bang { DoorToolsThresholdThenWhat::Bang } else { DoorToolsThresholdThenWhat::Step }, men, Some(&tt_of(t))) { DoorToolsThresholdThen::Text(w) => Some(w), DoorToolsThresholdThen::Arm1(()) => None };
    }
    match crate::doortools::door_tools_issue(g, if bang { Kind::BangClear } else { Kind::OpenClear }, men, t) { DoorToolsIssue::Text(w) => Some(w), DoorToolsIssue::Arm1(()) => None }
}
/// `only` of the two entries: the men holding the threshold, while it is held
pub fn row_held_or_all(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> { if crate::doortools::door_tools_held_plain(g, &crate::seats::here(g), t, men) { crate::doortools::door_tools_held_men(g, &crate::seats::here(g), men, t) } else { men.to_vec() } }

// ---------------------------------------------------------------- the class

/// the menu's switches as one number, to see one move: on the page all four are settled on
fn door_tools_thr_sig_now() -> f64 {
    (1 | 2 | 4 | 8) as f64
}

/// the door tools, and their rows on the menu
pub fn door_tools_new(g: &mut crate::game::Game) -> crate::doortools::DoorTools {
    use crate::orders::{rows, OrderDef, TargetKind as K};
    use crate::doortools::TOOLS_PAGE;
    let mut me = crate::doortools::DoorTools::blank();
    me.charges = g.k.doortools.charges;
    me.bangs = g.k.doortools.bangs;
    me.frags = g.k.frag.squad;
    register(g, rows::def(&rows::tools::ORDER, OrderDef { targets: vec![K::Door], opens: Some(String::from(TOOLS_PAGE)), ..OrderDef::default() }), None);
    register(g, rows::def(&rows::openclear::ORDER, OrderDef { page: Some(String::from(TOOLS_PAGE)), targets: vec![K::Door], ack: Some(strs(&["on it.", "moving."])), ..OrderDef::default() }), None);
    register(g, rows::def(&rows::bangclear::ORDER, OrderDef { page: Some(String::from(TOOLS_PAGE)), targets: vec![K::Door], ack: Some(strs(&["bang and clear.", "copy, banging it."])), ..OrderDef::default() }), None);
    register(g, rows::def(&rows::charge::ORDER, OrderDef { page: Some(String::from(TOOLS_PAGE)), targets: vec![K::Door], ack: Some(strs(&["copy.", "on it."])), ..OrderDef::default() }), None);
    register(g, rows::def(&rows::fragclear::ORDER, OrderDef { page: Some(String::from(TOOLS_PAGE)), targets: vec![K::Door], ack: Some(strs(&["frag and clear.", "copy, fragging it."])), ..OrderDef::default() }), None);
    // the row that stands in for the tools' while the jambs are held. no part registered it (CHANGES-6)
    register(g, rows::def(&rows::held::ORDER, OrderDef { targets: vec![K::Door], opens: Some(String::from(crate::doortools::HELD_PAGE)),
        stands_in: Some(crate::orders::OrderDefStandsIn { for_: String::from(TOOLS_PAGE) }), ..OrderDef::default() }), None);
    // thrRows writes `this`, and this is not on g yet: it stands there for the call
    core::mem::swap(&mut g.sets[g.seat].door_tools, &mut me);
    door_tools_thr_rows(g);
    core::mem::swap(&mut g.sets[g.seat].door_tools, &mut me);
    me
}

/// the row of the door held from outside, the one order of that page on the page's switches
pub fn door_tools_thr_rows(g: &mut crate::game::Game) {
    use crate::orders::{rows, OrderDef, TargetKind as K};
    g.sets[g.seat].door_tools.thr_sig = door_tools_thr_sig_now();
    for id in [crate::orders::rows::threshold::ORDER.id, crate::doortools::THR_PAGE] {
        crate::orders::squad_orders_unregister(g, id);
    }
    register(g, rows::def(&rows::threshold::ORDER, OrderDef { page: Some(String::from(crate::doortools::TOOLS_PAGE)), targets: vec![K::Door],
        ack: Some(strs(&["on the jambs. your call after.", "copy, holding it for you."])), ack_now: Some(strs(&["on the jambs. your call.", "copy, holding it for you."])), ..OrderDef::default() }), Some(crate::orders::rows::fragclear::ORDER.id));
    g.sets[g.seat].door_tools.thr_row = false;
}

/// spend one (false: none left)
pub fn door_tools_take(g: &mut crate::game::Game, what: crate::doortools::DoorToolsTakeWhat) -> bool {
    use crate::doortools::DoorToolsTakeWhat as W;
    door_tools_sync(g);
    if what == W::Frag {
        if g.sets[g.seat].door_tools.frags <= 0.0 { return false; }
        g.sets[g.seat].door_tools.frags -= 1.0;
        return true;
    }
    if what == W::Charge {
        if g.sets[g.seat].door_tools.charges <= 0.0 { return false; }
        g.sets[g.seat].door_tools.charges -= 1.0;
    } else {
        if g.sets[g.seat].door_tools.bangs <= 0.0 { return false; }
        g.sets[g.seat].door_tools.bangs -= 1.0;
    }
    true
}

/// one back in the bag
pub fn door_tools_refund(g: &mut crate::game::Game, what: crate::doortools::DoorToolsRefundWhat) {
    use crate::doortools::DoorToolsRefundWhat as W;
    if what == W::Frag {
        g.sets[g.seat].door_tools.frags += 1.0;
        return;
    }
    if what == W::Charge { g.sets[g.seat].door_tools.charges += 1.0; } else { g.sets[g.seat].door_tools.bangs += 1.0; }
}

/// a new cast is a fresh run; the rows that hang on the cast dealt again
pub fn door_tools_sync(g: &mut crate::game::Game) {
    use crate::orders::{rows, OrderDef, TargetKind as K};
    use crate::doortools::{BIG_ORDER, GoRow, TOOLS_PAGE};
    let (mut i, mut same, mut at) = (0usize, true, 0usize);
    while at < g.guards.len() {
        let gd = g.guards[at];
        at += 1;
        if crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) {
            if g.sets[g.seat].door_tools.roster.get(i) != Some(&gd) { same = false; }
            i += 1;
        }
    }
    if !same || i != g.sets[g.seat].door_tools.roster.len() {
        g.sets[g.seat].door_tools.roster = crate::orders::squad_orders_squad(g, &crate::seats::here(g));
        door_tools_reset(g);
        g.sets[g.seat].door_tools.go_row = None;
        g.sets[g.seat].door_tools.thr_row = false;
        g.sets[g.seat].door_tools.big_row = None;
    }
    // a man alone: a fresh Player is his new run
    let player = g.player;
    if g.sets[g.seat].door_tools.sam_of != Some(player) {
        let first = g.sets[g.seat].door_tools.sam_of.is_none();
        g.sets[g.seat].door_tools.sam_of = Some(player);
        if !first && g.sets[g.seat].door_tools.roster.is_empty() {
            g.sets[g.seat].door_tools.set.clear();
            g.sets[g.seat].door_tools.charges += g.sets[g.seat].door_tools.sam_spent;
        }
        if !first { g.sets[g.seat].door_tools.sam_spent = 0.0; }
    }
    let n = g.sets[g.seat].door_tools.roster.len();
    let go = if n == 0 { GoRow::None } else if n <= 2 { GoRow::Small } else { GoRow::Big };
    if g.sets[g.seat].door_tools.go_row != Some(go) {
        g.sets[g.seat].door_tools.go_row = Some(go);
        if go == GoRow::None {
            crate::orders::squad_orders_unregister(g, crate::orders::rows::go::ORDER.id);
        } else {
            let listed = if go == GoRow::Small { Vec::new() } else { vec![K::Floor, K::None] };
            register(g, rows::def(&rows::go::ORDER, OrderDef { word: true, targets: vec![K::Floor, K::Door, K::None], listed: Some(listed), ack: Some(strs(&["going.", "moving."])), ..OrderDef::default() }), None);
        }
    }
    if g.sets[g.seat].door_tools.thr_sig != door_tools_thr_sig_now() {
        door_tools_thr_rows(g);
        g.sets[g.seat].door_tools.big_row = None;
    }
    // the big slot's own row, while a man of the squad carries the hammer or the breaching shotgun
    let mut big = false;
    let mut k = 0;
    while k < g.sets[g.seat].door_tools.roster.len() {
        let m = g.sets[g.seat].door_tools.roster[k];
        k += 1;
        if crate::bigkit::carries_hammer(g, m) || (!crate::condition::down(g, g.gd[m].ch) && crate::kit::kit_big(g, m) == crate::kit::BigKit::Shotgun) {
            big = true;
            break;
        }
    }
    if Some(big) != g.sets[g.seat].door_tools.big_row {
        g.sets[g.seat].door_tools.big_row = Some(big);
        crate::orders::squad_orders_unregister(g, BIG_ORDER);
        if big {
            register(g, rows::def(&rows::bigclear::ORDER, OrderDef { page: Some(String::from(TOOLS_PAGE)), targets: vec![K::Door], ack: Some(strs(&["on it.", "breaching."])), ..OrderDef::default() }), Some(crate::orders::rows::fragclear::ORDER.id));
        }
    }
    if !g.sets[g.seat].door_tools.thr_row {
        g.sets[g.seat].door_tools.thr_row = true;
        register(g, rows::def(&rows::thrword::ORDER, OrderDef { page: Some(String::from(crate::doortools::HELD_PAGE)), word: true, targets: vec![K::Door], ..OrderDef::default() }), None);
    }
}

/// a fresh run: the counts back, nothing set, nothing under way
pub fn door_tools_reset(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.sets[g.seat].door_tools.ops.len() {
        let op = g.sets[g.seat].door_tools.ops[i];
        i += 1;
        door_tools_defuse(g, op);
        door_tools_off_lock(g, op);
    }
    g.sets[g.seat].door_tools.ops.clear();
    g.sets[g.seat].door_tools.done.clear();
    g.sets[g.seat].door_tools.set.clear();
    g.sets[g.seat].door_tools.words.clear();
    g.sets[g.seat].door_tools.held.clear();
    g.sets[g.seat].door_tools.watches.clear();
    g.sets[g.seat].door_tools.tried.clear();
    g.sets[g.seat].door_tools.sam_spent = 0.0;
    let n = g.k.doortools.charges;
    g.sets[g.seat].door_tools.charges = crate::seats::seat_share(g, n);
    let n = g.k.doortools.bangs;
    g.sets[g.seat].door_tools.bangs = crate::seats::seat_share(g, n);
    let n = g.k.frag.squad;
    g.sets[g.seat].door_tools.frags = crate::seats::seat_share(g, n);
    g.sets[g.seat].door_tools.stats = crate::doortools::DoorToolsStats::default();
    g.sets[g.seat].door_tools.seen = None;
}

/// nobody came with him: the count is his card's, nothing set, nothing spent
pub fn door_tools_alone(g: &mut crate::game::Game, n: f64) {
    door_tools_sync(g);
    if !g.sets[g.seat].door_tools.roster.is_empty() { return; }
    g.sets[g.seat].door_tools.set.clear();
    g.sets[g.seat].door_tools.sam_spent = 0.0;
    g.sets[g.seat].door_tools.charges = m::max(0.0, m::floor(n));
}

/// the charge set on this door, if any
pub fn door_tools_charge_on(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: DoorId) -> Option<ChargeId> {
    seat.door_tools.set.iter().copied().find(|&c| g.tab.doortools_set_charge[c].door == d)
}

/// can these men stack on that face of that door (the entry orders' menu test)
pub(super) fn door_tools_stackable(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> crate::doortools::DoorToolsStackable {
    use crate::doortools::DoorToolsStackable as V;
    let Some((d, side)) = door_of(t) else { return V::Bool(false) };
    let Some(deal) = door_tools_deal_stack(g, d, side, men) else {
        return V::Text(crate::orders::squad_orders_no_room_words(g, "no room on that door.", &mut |g| door_tools_deal_stack(g, d, side, men).is_some()));
    };
    let posts = deal.posts;
    let spots: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|x| x.group.unwrap_or(0.0)).collect();
    let front = g.dr[d].frame_centre;
    for (gd, p) in crate::orders::squad_orders_pair(g, men, &spots, Some(&groups), Some(front)) {
        if let Some(w) = crate::orders::squad_orders_reach(g, gd, p) {
            if !w.is_empty() { return V::Text(w); }
        }
    }
    if !door_tools_known_shut(g, d, side) { return V::Bool(true); }
    if g.dr[d].def.steel.unwrap_or(false) && !is_open(g, d) && door_tools_locked_out(g, d, side) {
        return V::Text(String::from("steel, and locked."));
    }
    if door_tools_frame_held(g, d, side) && !door_tools_pickable(g, d) && !door_tools_big_opens(g, d, men) {
        return V::Text(door_tools_no_boot(g, &crate::seats::here(g), d));
    }
    V::Bool(true)
}

/// is the door locked against the team from that face
pub(super) fn door_tools_locked_out(g: &crate::game::Game, d: DoorId, side: f64) -> bool {
    let w = g.dr[d].def.width;
    let from = door_pt(g, d, w / 2.0, 0.6, side, None);
    let ring = crate::orders::squad_orders_ring(g, &crate::seats::here(g));
    crate::doors::doors_locked_out(g, d, from, Some(&ring))
}

/// locked against the team on the face it opens toward: a boot only drives it onto its frame
pub(super) fn door_tools_frame_held(g: &crate::game::Game, d: DoorId, side: f64) -> bool {
    if g.dr[d].def.steel.unwrap_or(false) || is_open(g, d) || !door_tools_locked_out(g, d, side) { return false; }
    let w = g.dr[d].def.width;
    let from = door_pt(g, d, w / 2.0, 0.6, side, None);
    crate::doors::doors_boot_holds(g, d, from)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A door's frame runs hinge to latch, and `side` picks the face. A failure means a man's post comes out on the wrong face of a door, or past the wrong jamb.
    #[test]
    fn a_place_in_a_door_s_frame_reads_back_on_either_face() {
        assert_eq!(frame_pt(0.0, [1.0, 2.0], 0.5, 0.25, 1.0, 0.45), [1.5, 0.45, 2.25]);   // a door along +x: its +1 face is +z
        let (th, h) = (0.7, [3.25, -1.5]);
        let near = |a: f64, b: f64| assert!((a - b).abs() < 1e-12, "{a} {b}");
        for side in [1.0, -1.0] {
            let p = frame_pt(th, h, 1.1, 0.6, side, 0.0);
            near(frame_along(th, h, p), 1.1);
            near(frame_off(th, h, p, side), 0.6);
            near(frame_off(th, h, p, -side), -0.6);
        }
    }
}
