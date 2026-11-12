use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ------------------------------------------------------------ small words

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
fn sel_of_who(w: Who) -> SquadOrdersMenSel {
    match w {
        Who::Num(k) => SquadOrdersMenSel::Num(k),
        Who::All => SquadOrdersMenSel::All,
        Who::Red => SquadOrdersMenSel::Red,
        Who::Blue => SquadOrdersMenSel::Blue,
    }
}
/// `a === b` of two marks of one deal: pair() hands the spots back unchanged, so their bits are the same, a NaN's too
pub(crate) fn same_bits(a: V3, b: V3) -> bool {
    a[0].to_bits() == b[0].to_bits() && a[1].to_bits() == b[1].to_bits() && a[2].to_bits() == b[2].to_bits()
}
/// a string as a condition
pub(crate) fn truthy_str(s: &Option<String>) -> bool {
    s.as_deref().map_or(false, |x| !x.is_empty())
}

// (the order's own functions are its file in orders/rows/. 'fallIn' sorts by this, which the tests below read too)

/// 'fallIn'.issue's sort of one man against another: pair_near, near, pair
pub(crate) fn fall_in_cmp(many: bool, a: (f64, f64, Option<crate::game::PairId>), b: (f64, f64, Option<crate::game::PairId>)) -> f64 {
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

// ------------------------------------------------------------ the class

/// A module that changed these men's plan with nobody asking says so: drawn for a moment.
pub fn squad_orders_show(g: &mut crate::game::Game, men: &[GuardId]) {
    let t = g.time;
    let who: Vec<String> = men.iter().filter(|&&m| !crate::condition::down(g, g.gd[m].ch)).map(|&m| g.gd[m].callsign.clone()).collect();
    g.groups[g.group].orders.shows.push(SquadOrdersShowsItem { t, who });
    if g.groups[g.group].orders.shows.len() > 16 { g.groups[g.group].orders.shows.drain(0..8); }
}

/// The built-in rows of the menu and the night breach's owner.
pub fn squad_orders_new(g: &mut crate::game::Game) -> crate::orders::SquadOrders {
    let mut s = SquadOrders::blank();
    let words = |w: &[&str]| -> Option<Vec<String>> { Some(w.iter().map(|x| String::from(*x)).collect()) };
    let rows = [
        rows::def(&rows::movethere::ORDER, OrderDef {
            targets: vec![TargetKind::Floor],
            ack: words(&["moving.", "on it."]),
            ..Default::default()
        }),
        rows::def(&rows::stack::ORDER, OrderDef {
            targets: vec![TargetKind::Door],
            ack: words(&["stacking up.", "on the door."]),
            ..Default::default()
        }),
        rows::def(&rows::cover::ORDER, OrderDef {
            targets: vec![TargetKind::Floor, TargetKind::Door],
            ack: words(&["covering.", "i've got it."]),
            ..Default::default()
        }),
        rows::def(&rows::fallin::ORDER, OrderDef {
            targets: vec![TargetKind::Floor, TargetKind::Door, TargetKind::None],
            ack: words(&["on you.", "with you."]),
            ..Default::default()
        }),
        rows::def(&rows::hold::ORDER, OrderDef {
            targets: vec![TargetKind::Floor, TargetKind::Door, TargetKind::None],
            ack: words(&["holding."]),
            ..Default::default()
        }),
    ];
    for row in rows {   // (rows of its own: a menu newly stood has none yet, whichever seat the pass is about while it is made)
        let (id, d) = (row.id.clone(), new_row(g, row));
        s.made.set(id, d);
        s.defs.push(d);
    }
    // the night breach's drill owns the pair from Sam's 'stack up' (F) until the room is called or lost (mission::has, lets_go, doing)
    s
}

/// Is there a squad to order at all (a living operative unit, outside any show).
pub fn squad_orders_active(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if squad_orders_mine(g, seat, gd) && !crate::condition::down(g, g.gd[gd].ch) { return true; }
    }
    false
}

/// The roster: every operative unit on the guard list, in its order, down or up.
pub fn squad_orders_squad(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if squad_orders_mine(g, seat, gd) { out.push(gd); }
    }
    out
}

/// Is he one of the men these orders move: the one roster test.
pub fn squad_orders_mine(g: &crate::game::Game, seat_ref: &crate::seats::SeatRef, gd: GuardId) -> bool {
    crate::group::has(g, seat_ref.group_id, gd)
}

/// The player is one of these men's own.
pub fn squad_orders_sams(seat: &crate::seats::SeatRef) -> bool {
    seat.group.by_a_player()
}

/// Whose side a can of theirs is.
pub fn squad_orders_team(seat: &crate::seats::SeatRef) -> crate::teams::Team {
    seat.group.side
}

/// The keys a locked door is tried with for them: Sam's ring and the codes he has learned.
pub fn squad_orders_ring(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> JsSet<String> {
    if !seat.group.by_a_player() { return seat.group.ring.clone(); }
    player_ring(g, seat.player)
}
/// A player's own ring: his keys, and the codes he has learned.
pub fn player_ring(g: &Game, p: PlayerId) -> JsSet<String> {
    let mut ring = g.pl[p].keys.clone();
    for code in g.pl[p].codes.keys() { ring.add(code); }
    ring
}

/// The living men an order would go to: the teammate under the reticle, else `who`.
pub fn squad_orders_recipients(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: Option<&crate::orders::OrderTarget>, who: Option<crate::orders::Who>) -> Vec<GuardId> {
    if let Some(OrderTarget::None(x)) = t {
        if let Some(man) = x.man {
            return if !crate::condition::down(g, g.gd[man].ch) && squad_orders_squad(g, seat).contains(&man) { vec![man] } else { Vec::new() };
        }
    }
    let who = who.unwrap_or(seat.orders.who);
    squad_orders_men(g, seat, Some(sel_of_who(who)), None)
}

/// The one place to ask for men: the squad, a pair, or a man; the living unless `down`.
pub fn squad_orders_men(g: &crate::game::Game, seat: &crate::seats::SeatRef, sel: Option<crate::orders::SquadOrdersMenSel>, opts: Option<crate::orders::SquadOrdersMenOpts>) -> Vec<GuardId> {
    let s = squad_orders_squad(g, seat);
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
    list.into_iter().filter(|&gd| down || !crate::condition::down(g, g.gd[gd].ch)).collect()
}

/// The squad of four (any roster of more than two).
pub fn squad_orders_big(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    squad_orders_men(g, seat, Some(SquadOrdersMenSel::All), Some(SquadOrdersMenOpts { down: Some(true) })).len() > 2
}

/// The pair a man was dealt into.
pub fn squad_orders_pair_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::game::PairId> {
    g.gd[gd].unit.as_ref().and_then(|u| u.pair)
}

/// The pairs on the roster, in its order.
pub fn squad_orders_pairs(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<crate::game::PairId> {
    let mut out = Vec::new();
    for gd in squad_orders_squad(g, seat) {
        if let Some(id) = g.gd[gd].unit.as_ref().and_then(|u| u.pair) {
            if !out.contains(&id) { out.push(id); }
        }
    }
    out
}

/// Is the squad dealt into more than one pair.
pub fn squad_orders_paired(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    squad_orders_pairs(g, seat).len() > 1
}

/// The next element: gold, red, blue for a squad in pairs; all, then man by man for the one pair.
pub fn squad_orders_cycle_who(g: &mut crate::game::Game, dir: Option<f64>) -> crate::orders::Who {
    let dir = dir.unwrap_or(1.0);
    let s = squad_orders_squad(g, &crate::seats::here(g));
    let mut ring: Vec<Who> = vec![Who::All];
    if squad_orders_paired(g, &crate::seats::here(g)) {
        for id in squad_orders_pairs(g, &crate::seats::here(g)) {
            if !squad_orders_men(g, &crate::seats::here(g), Some(sel_of_pair(id)), None).is_empty() { ring.push(who_of_pair(id)); }
        }
    } else {
        for (k, &gd) in s.iter().enumerate() {
            if !crate::condition::down(g, g.gd[gd].ch) { ring.push(Who::Num(k as f64)); }
        }
    }
    let was = g.groups[g.group].orders.who;
    let now = match ring.iter().position(|&w| same_who(w, was)) {   // (a man of a squad in pairs is not on the ring: gold, either way)
        None => Who::All,
        Some(i) => ring[js::rem(i as f64 + dir + ring.len() as f64, ring.len() as f64) as usize],
    };
    g.groups[g.group].orders.who = now;
    now
}

/// Who these men are, in a word or two.
pub fn squad_orders_who_words(g: &crate::game::Game, seat: &crate::seats::SeatRef, men: Option<&[GuardId]>) -> String {
    let own;
    let men: &[GuardId] = match men {
        Some(m) => m,
        None => { own = squad_orders_recipients(g, seat, None, None); &own }
    };
    let alive = squad_orders_squad(g, seat).into_iter().filter(|&gd| !crate::condition::down(g, g.gd[gd].ch)).count();
    if men.len() == alive && men.len() > 1 {
        return String::from(if squad_orders_paired(g, seat) { "gold" } else { "all" });
    }
    let id = if men.len() > 1 { squad_orders_pair_of(g, men[0]) } else { None };
    if let Some(id) = id {
        let all_of = men.iter().all(|&gd| g.gd[gd].unit.as_ref().and_then(|u| u.pair) == Some(id));
        if all_of && men.len() == squad_orders_men(g, seat, Some(sel_of_pair(id)), None).len() { return String::from(id.as_str()); }   // (a whole pair of a bigger squad)
    }
    let names: Vec<&str> = men.iter().map(|&gd| g.gd[gd].callsign.as_str()).collect();
    let joined = names.join(", ");
    if joined.is_empty() { String::from("nobody") } else { joined }
}

/// The element's colour for the panel.
pub fn squad_orders_tone_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, men: &[GuardId]) -> Option<crate::orders::Tone> {
    if !squad_orders_paired(g, seat) || men.is_empty() { return None; }
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

pub fn squad_orders_has(seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    seat.orders.asg.has(gd)
}

pub fn squad_orders_of(seat: &crate::seats::SeatRef, gd: GuardId) -> Option<TaskId> {
    seat.orders.asg.get(gd).copied()
}

/// The tick's one write of a follower's latched word: `d` is how far he is off his place.
pub(super) fn squad_orders_follow_word(g: &mut crate::game::Game, a: TaskId, d: f64) {
    let t = g.time;
    let arrived = g.tab.task[a].arrived;
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

/// Is he shown as there: a follower's is the latch, every other order's is `arrived`.
pub(super) fn squad_orders_shown_there(g: &crate::game::Game, a: TaskId) -> bool {
    let row = &g.tab.task[a];
    if row.post.is_some() { return row.arrived; }
    match g.m.orders.follow_word.get(a) {
        Some(w) => w.on,
        None => row.arrived,
    }
}

/// Add an order to the menu: at the foot or before `before`.
pub fn squad_orders_register(g: &mut crate::game::Game, def: OrdersOrderDefId, before: Option<&str>) {
    let id = g.tab.orders_order_def[def].id.clone();
    squad_orders_unregister(g, &id);
    if let Some(si) = &g.tab.orders_order_def[def].stands_in {   // a stand-in: the other row's page and place, the two never listed together
        let wants = si.for_.clone();
        let stood = g.groups[g.group].orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == wants);
        if let Some(stood) = stood {
            squad_orders_seat(g, def, stood);
            return;
        }
        g.tab.orders_order_def[def].stands_in = None;   // (not registered: listed as a row of its own)
    }
    let i = match before {
        Some(b) if !b.is_empty() => g.groups[g.group].orders.defs.iter().position(|&d| g.tab.orders_order_def[d].id == b),
        _ => None,
    };
    let at = i.unwrap_or(g.groups[g.group].orders.defs.len());
    g.groups[g.group].orders.defs.insert(at, def);
    let stand_ins: Vec<OrdersOrderDefId> = g.groups[g.group].orders.defs.iter().copied()
        .filter(|&d| g.tab.orders_order_def[d].stands_in.as_ref().map_or(false, |x| x.for_ == id))
        .collect();
    for s in stand_ins {   // (registered again: its stand-ins move with it)
        if let Some(j) = g.groups[g.group].orders.defs.iter().position(|&d| d == s) { g.groups[g.group].orders.defs.remove(j); }
        squad_orders_seat(g, s, def);
    }
}

/// Open a fresh group (the men whose arrival one 'set.' waits on).
pub fn squad_orders_new_group(g: &mut crate::game::Game) -> f64 {
    g.groups[g.group].orders.group += 1.0;
    g.groups[g.group].orders.group
}

/// A ROW OF A SEAT'S MENU IS ONE ROW OF THE TABLE, HOWEVER OFTEN IT IS LISTED. The row this seat's menu has for the id is written in place, on the
/// menu or off it; an id this menu has never had is a new row. Every module that lists an order makes its row through here.
///
/// Before, each listing pushed a new row and the one before it was left in the table for good: a row re-listed when the squad's roster changed,
/// when a held door's page was rebuilt or when the man with the hammer fell left an orphan each time, every later row stood one place further
/// on, and two machines of one game that had seen a different number of re-listings numbered their rows differently.
///
/// THE ROW IS THE MENU'S OWN (`SquadOrders.made`): each group has its own. Rows are written in place (a page, a target that
/// comes and goes with the pairs), so two menus that shared one would change each other's.
pub fn squad_orders_row(g: &mut crate::game::Game, def: crate::orders::OrderDef) -> OrdersOrderDefId {
    if let Some(d) = g.groups[g.group].orders.made.get(def.id.clone()).copied() {
        g.tab.orders_order_def[d] = def;
        return d;
    }
    let (id, d) = (def.id.clone(), new_row(g, def));
    g.groups[g.group].orders.made.set(id, d);
    d
}

/// a row the table has not had
fn new_row(g: &mut Game, def: OrderDef) -> OrdersOrderDefId { g.tab.orders_order_def.push(def) }

pub fn squad_orders_unregister(g: &mut crate::game::Game, id_2: &str) {
    if let Some(i) = g.groups[g.group].orders.defs.iter().position(|&d| g.tab.orders_order_def[d].id == id_2) {
        g.groups[g.group].orders.defs.remove(i);
    }
}

/// Take the men off their orders (all of them by default): they stop where they are, still on the runner's script.
pub fn squad_orders_clear(g: &mut crate::game::Game, men: Option<&[GuardId]>) {
    let own;
    let men: &[GuardId] = match men {
        Some(m) => m,
        None => { own = g.groups[g.group].orders.asg.keys(); &own }
    };
    for &gd in men {
        if g.groups[g.group].orders.asg.has(gd) && g.gd[gd].script.is_some() && !crate::condition::down(g, g.gd[gd].ch) { crate::squad::order(g, gd, crate::squad::OrderPatch { goal: Some(None), ..Default::default() }); }
        squad_orders_drop(g, gd);
    }
    if !g.groups[g.group].orders.asg.values().any(|&a| g.tab.task[a].post.is_none()) { g.groups[g.group].orders.trail.clear(); }
}

/// `this.people?.near(at) ?? null`: the man of the other side nearest the point, with how near; None while g.orders.people is None.
pub fn squad_orders_people_near(g: &crate::game::Game, seat: &crate::seats::SeatRef, at: V3) -> Option<(GuardId, f64)> {
    if seat.orders.people.is_none() { return None; }
    if !crate::pairorders::pair_orders_up(g, seat) { return None; }
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.pair.pick_m;
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        if !crate::pairorders::pair_orders_is_foe(g, h) { continue; }
        let d = v3::dist_xz(g.pos_of(h), at);
        if d < bd { bd = d; best = Some(h); }
    }
    best.map(|gd| (gd, bd))
}

/// What a point in the world is, as a target: the nearest of a door, a teammate and Sam himself; else the free floor at or nearest it; else nothing.
pub fn squad_orders_target_at(g: &mut crate::game::Game, p: V3) -> crate::orders::OrderTarget {
    pointed(g);
    let mut asked = Vec::new();
    let t = target_at(g, &crate::seats::here(g), p, &mut asked);
    crate::wallcharge::sky_kept(g, asked);
    t
}
/// what a question of a point wants standing (before the first tick: the seat's list of things to work, which a tick keeps)
fn pointed(g: &mut Game) {
    if g.sets[g.seat].work.cache.is_none() { crate::workthings::work_things_keep(g); }
}
/// …asked of the game shared (`asked` = the walls whose sky was asked, for a rule to keep)
pub(super) fn target_at(g: &Game, seat: &crate::seats::SeatRef, p: V3, asked: &mut Vec<(f64, f64)>) -> crate::orders::OrderTarget {
    let (door_pick_m, man_m, self_m) = (g.k.orders.door_pick_m, g.k.orders.man_m, g.k.orders.self_m);
    let sam = g.seat_pos(seat);
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
    for gd in squad_orders_squad(g, seat) {
        let x = v3::dist_xz(g.pos_of(gd), at);
        if !crate::condition::down(g, g.gd[gd].ch) && x < dm { dm = x; man = Some(gd); }
    }
    let ds = v3::dist_xz(sam, at);
    let fo = squad_orders_people_near(g, seat, at);   // (a man of the other side nearer the point than a door, a teammate or Sam himself)
    if let Some((foe, fd)) = fo {
        if !(door.is_some() && dd < fd) && !(man.is_some() && dm < fd) && !(ds < self_m && ds < fd) {
            let fp = g.pos_of(foe);
            return OrderTarget::Foe(OrderTargetFoe { at: [fp[0], 0.0, fp[2]], foe });
        }
    }
    let wk = crate::workthings::work_near(g, seat, at);   // (a thing a teammate can work, nearer the point than a door, a teammate or Sam himself)
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
    let base = match squad_orders_free_spot(g, seat, at) {
        Some(spot) => OrderTarget::Floor(OrderTargetFloor { at: spot, wall: None, thing: None, serial: 0 }),   // (serial 0: no menu's yet; opening a menu stamps the one it keeps)
        None => OrderTarget::None(OrderTargetNone { at, man: None }),
    };
    crate::wallcharge::wall_target(g, seat, at, &base, asked)   // (a floor target under a wall carries the wall)
}

/// How far along the ray it crosses one door's doorway rectangle, nearer than `bt` and with nothing of the scene short of the frame line
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

/// The door a ray is on as drawn: the nearest doorway it crosses with nothing of the scene nearer.
pub(super) fn squad_orders_door_on_ray(g: &crate::game::Game, ro: V3, rd: V3, t_scene: Option<f64>) -> Option<DoorId> {
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

/// What the cursor is on, for the menu; with CMDRING.wholeSquad a teammate under it is the squad itself.
pub fn squad_orders_target_under(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> crate::orders::OrderTarget {
    pointed(g);
    let now = crate::view::host_now(g, input, cam, canvas);
    let mut asked = Vec::new();
    let t = target_under(g, &crate::seats::here(g), &now, &mut asked);
    crate::wallcharge::sky_kept(g, asked);
    t
}
/// …asked of the game shared, by a cursor and a camera as they stand
pub fn target_under(g: &Game, seat: &crate::seats::SeatRef, now: &crate::view::HostNow, asked: &mut Vec<(f64, f64)>) -> crate::orders::OrderTarget {
    let t = target_under_of(g, seat, now, asked);
    if let OrderTarget::None(x) = &t {
        if x.man.is_some() && squad_orders_sams(seat) { return OrderTarget::None(OrderTargetNone { at: x.at, man: None }); }
    }
    t
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    /// A row listed again is the same row of the table, with what it was last listed as; only a new id is a new row.
    #[test]
    fn a_row_listed_again_is_the_same_row() {
        let mut g = alloc::boxed::Box::new(crate::game::Game::blank());
        let was = g.tab.orders_order_def.len();
        let row = |id: &str, page: &str| crate::orders::OrderDef { id: String::from(id), page: Some(String::from(page)), ..Default::default() };
        let a = squad_orders_row(&mut g, row("first", "1"));
        let b = squad_orders_row(&mut g, row("second", "2"));
        assert_ne!(a, b);
        for k in 0..5 {
            assert_eq!(squad_orders_row(&mut g, row("first", ["3", "4", "5", "6", "7"][k])), a, "listing {k}: the row moved");
            assert_eq!(squad_orders_row(&mut g, row("second", "2")), b);
        }
        assert_eq!(g.tab.orders_order_def.len(), was + 2, "ten more listings made no row");
        assert_eq!(g.tab.orders_order_def[a].page.as_deref(), Some("7"), "the row holds what it was last listed as");
        // (on the menu or off it: a row taken off the menu is still the table's row of its id)
        squad_orders_register(&mut g, a, None);
        squad_orders_unregister(&mut g, "first");
        assert_eq!(squad_orders_row(&mut g, row("first", "1")), a);
        assert_eq!(g.tab.orders_order_def.len(), was + 2);
    }
    use crate::game::PairId;

    /// A door along +x at (2, 1), 2 m wide and 2.1 m high; the cursor's ray comes from (2, 1, -3) along +z and crosses it at 4 m. A failure means a menu opens on a door the cursor is not on.
    #[test]
    fn a_ray_is_on_a_door_where_it_crosses_the_doorway_with_nothing_drawn_nearer() {
        let (c, pad, far): (V3, f64, f64) = ([2.0, 0.0, 1.0], 0.08, f64::INFINITY);
        let ray = |x: f64, y: f64, t_scene: f64, bt: f64| ray_on_door([x, y, -3.0], [0.0, 0.0, 1.0], c, 0.0, 2.0, 2.1, pad, t_scene, bt);
        assert_eq!(ray(2.0, 1.0, far, far), Some(4.0));
        assert_eq!(ray(2.0, 1.0, 3.95, far), Some(4.0));    // the scene a hand short of the frame line is the frame
        assert_eq!(ray(2.0, 1.0, 3.8, far), None);          // more than that is something in front of it
        assert_eq!(ray(2.0, 1.0, far, 3.0), None);          // a nearer door already found wins
        assert_eq!(ray(3.05, 1.0, far, far), Some(4.0));    // a little past the jamb still picks it
        assert_eq!(ray(3.1, 1.0, far, far), None);
        assert_eq!(ray(2.0, 2.2, far, far), None);          // over its top
        assert_eq!(ray_on_door([2.0, 1.0, -3.0], [0.0, 0.0, -1.0], c, 0.0, 2.0, 2.1, pad, far, far), None);    // behind the eye
        assert_eq!(ray_on_door([2.0, 1.0, 1.0], [1.0, 0.0, 0.0], c, 0.0, 2.0, 2.1, pad, far, far), None);      // along the wall
    }

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
