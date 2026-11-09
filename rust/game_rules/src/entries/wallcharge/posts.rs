//! src/game/wallcharge.ts lines 421 to 786
use super::*;                      // your own module: its types, its other parts' functions
use crate::ask::Over;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

fn flat(a: V3, b: V3) -> f64 { v3::dist_xz(a, b) }

/// A box's row (`Box.*`). Skeleton 5 keeps `g.col.boxes: Vec<BoxId>` and no table of Box rows; on skeleton 5.1, which has the table `g.tab.boxes`, the body is `&g.tab.boxes[b]`.
fn seam_box(g: &Game, b: BoxId) -> &crate::scene_boxes::Box { &g.tab.boxes[b] }

fn sam_pos(g: &Game, seat: &crate::seats::SeatRef) -> V3 { g.ch[g.pl[seat.player].ch].pos }
fn man_pos(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }
fn free(g: &mut Game, q: V3) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) }
fn clear(g: &mut Game, a: V3, b: V3) -> bool { !crate::scene_collision::solid_blocked(g, [a[0], 0.45, a[2]], [b[0], 0.45, b[2]], None) }
fn box_ch(g: &Game, b: BoxId) -> (V3, V3) { { let r = seam_box(g, b); (r.c, r.h) } }
/// the box of the wall as authored, by the engine's id, else the spot's own (`g.col.boxes[wall] ?? W.box`)
fn box_of_wall(g: &Game, w: &WallSpot, wall: f64) -> BoxId { if wall >= 0.0 { g.col.boxes.get(wall as usize).copied().unwrap_or(w.box_) } else { w.box_ } }
fn wall_of(g: &Game, c: WallChargeId) -> WallSpot { g.tab.wallcharge_set_wall_charge[c].wall.clone() }

// ---------------------------------------------------------------- the run, the switch, the rows
/// (The roster is this book's own men, by the one roster test: an operative unit of this seat. Another seat's teammate coming or going is not this
/// seat's run starting over, and a seat with no teammates of its own is a man alone whatever the other seat has.)
/// A new Player or a new cast is a fresh run; the rows are on the menu only while the module is live. Returns live. (wallcharge.ts sync)
pub(super) fn wall_charge_sync(g: &mut crate::game::Game) -> bool {
    let (mut i, mut same) = (0usize, g.wall_charge.player == Some(g.player));
    for k in 0..g.guards.len() {
        let gd = g.guards[k];
        if crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) {
            if g.wall_charge.roster.get(i) != Some(&gd) { same = false; }
            i += 1;
        }
    }
    if !same || i != g.wall_charge.roster.len() {
        let (first, was) = (g.wall_charge.player.is_none(), g.wall_charge.roster.len());
        let mut roster = Vec::new();
        for k in 0..g.guards.len() {
            let gd = g.guards[k];
            if crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) { roster.push(gd); }
        }
        g.wall_charge.roster = roster;
        g.wall_charge.player = Some(g.player);
        if !first && was == 0 && g.wall_charge.roster.is_empty() && g.wall_charge.alone > 0.0 {
            crate::doortools::door_tools_sync(g);
            loop {
                let a = g.wall_charge.alone;
                g.wall_charge.alone = a - 1.0;
                if !(a > 0.0 && g.door_tools.charges < g.k.doortools.charges) { break; }
                crate::doortools::door_tools_refund(g, crate::doortools::DoorToolsRefundWhat::Charge);
            }
        }
        wall_charge_reset(g);
    }
    let live = wall_charge_on(g);
    let mode = if live { g.k.wallcharge.menu.as_str() } else { "" };
    if mode != g.wall_charge.mode {
        g.wall_charge.mode = String::from(mode);
        wall_charge_seat(g);
        if !live { wall_charge_stop(g); }
    }
    live
}

/// Puts the rows on the menu as the mode says. (wallcharge.ts seat)
fn wall_charge_seat(g: &mut crate::game::Game) {
    crate::orders::squad_orders_unregister(g, WALL_PAGE);
    crate::orders::squad_orders_unregister(g, WALL_ORDER);
    g.wall_charge.seated = false;
    if g.wall_charge.mode.is_empty() { return; }
    let page = g.wall_charge.mode == "page";
    let (opener, row) = (g.wall_charge.opener, g.wall_charge.row);
    g.tab.orders_order_def[opener].key = Some(5.0);
    if page { crate::orders::squad_orders_register(g, opener, None); }
    g.tab.orders_order_def[row].page = if page { Some(String::from(WALL_PAGE)) } else { None };
    g.tab.orders_order_def[row].key = Some(if page { 3.0 } else { 5.0 });
    crate::orders::squad_orders_register(g, row, None);
    g.wall_charge.seated = true;
}

/// A fresh run: nothing set, nothing under way. (wallcharge.ts reset)
pub fn wall_charge_reset(g: &mut crate::game::Game) {
    g.wall_charge.ops = Vec::new();
    g.wall_charge.done = Vec::new();
    g.wall_charge.set = Vec::new();
    g.wall_charge.sam = None;
    g.wall_charge.offered = None;
    g.wall_charge.alone = 0.0;
    let n = match g.k.wallcharge.carried { None => None, Some(c) => Some(crate::seats::seat_share(g, c)) };
    g.wall_charge.carried = n;
    g.wall_charge.brought = n;
    g.wall_charge.log = Vec::new();
    g.wall_charge.stats = WallChargeStats::default();
    g.wall_charge.last_blow = None;
    g.wall_charge.walks = None;
    g.wall_charge.keep.asks = 0.0;
    g.wall_charge.keep.hits = 0.0;
    g.wall_charge.away.clear();
    g.wall_charge.away_marks.clear();
    g.wall_charge.away_none.clear();
}

/// The switch went off mid-run: what is set goes back in the bag, what is under way ends where it stands. (wallcharge.ts stop)
fn wall_charge_stop(g: &mut crate::game::Game) {
    for op in g.wall_charge.ops.clone() { wall_charge_end(g, op, "switched off"); }
    for c in core::mem::take(&mut g.wall_charge.set) { wall_charge_give_back(g, c, "switched off"); }
    g.wall_charge.sam = None;
    g.wall_charge.marker = None;
    g.wall_charge.walks = None;
}

/// (wallcharge.ts note)
pub fn wall_charge_note(g: &mut crate::game::Game, what: &str) {
    g.wall_charge.log.push(WallChargeLogItem {  });
    if g.wall_charge.log.len() > 300 { g.wall_charge.log.drain(0..100); }
}

/// (wallcharge.ts beat)
pub(super) fn wall_charge_beat(g: &mut crate::game::Game, op: WallOpId, what: &str) {
    let line = format!("wall {}: {}", g.tab.wallcharge_wall_op[op].wall.index, what);
    wall_charge_note(g, &line);
}

/// The kit's hook: this run carries `n` wall charges of its own; none: back to the cost rule. (wallcharge.ts setCarried)
pub fn wall_charge_set_carried(g: &mut crate::game::Game, n: Option<f64>) {
    wall_charge_sync(g);
    let v = n.map(|n| m::max(0.0, m::floor(n)));
    g.wall_charge.carried = v;
    g.wall_charge.brought = v;
}

/// A mission's own issue: this run carries AT LEAST `n`. (wallcharge.ts raiseCarried)
pub fn wall_charge_raise_carried(g: &mut crate::game::Game, n: f64) {
    wall_charge_sync(g);
    let k = m::max(0.0, m::floor(n));
    if g.wall_charge.carried.unwrap_or(0.0) < k {
        g.wall_charge.carried = Some(k);
        g.wall_charge.brought = Some(k);
    }
}

/// What a wall takes out of the bag now. (wallcharge.ts price)
fn wall_charge_price(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> f64 {
    if seat.wall_charge.carried.is_some() { 0.0 } else { m::max(1.0, m::floor(g.k.wallcharge.cost)) }
}

/// Can one be set. (wallcharge.ts affordable)
pub(super) fn wall_charge_affordable(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    match seat.wall_charge.carried { Some(c) => c > 0.0, None => seat.door_tools.charges >= wall_charge_price(g, seat) }
}

/// The row's words. (wallcharge.ts rowWords)
pub fn wall_charge_row_words(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> String {
    let (n, p) = (seat.door_tools.charges, wall_charge_price(g, seat));
    if let Some(c) = seat.wall_charge.carried { return format!("breach with a wall charge · {} left", c); }
    if p <= 1.0 { format!("breach with a charge · {} left", n) }
    else if p == 2.0 && n == 2.0 { String::from("breach the wall · uses both charges") }
    else { format!("breach the wall · uses {} of {} charges", p, n) }
}

/// The refusal's words, with none to set. (wallcharge.ts noneWords)
pub(super) fn wall_charge_none_words(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> String {
    if seat.wall_charge.carried.is_some() {
        if seat.wall_charge.brought == Some(0.0) { return String::from(crate::words::wg(g, &"nobody brought a wall charge.", &"no wall charge carried.")); }
        return String::from("no wall charge left.");
    }
    if wall_charge_price(g, seat) > 1.0 && seat.door_tools.charges > 0.0 { String::from("not enough charge for a wall.") } else { String::from("no charges left.") }
}

/// One set on a wall: the price paid, or -1 = there was none to set. (wallcharge.ts take)
pub fn wall_charge_take(g: &mut crate::game::Game) -> f64 {
    g.wall_charge.free_set = false;
    if g.infinite_ammo && wall_charge_affordable(g, &crate::seats::here(g)) {
        g.wall_charge.free_set = true;
        g.wall_charge.stats.set += 1.0;
        wall_charge_note(g, "wall charge set · unlimited ammo: none spent");
        return 0.0;
    }
    if let Some(c) = g.wall_charge.carried {
        if c <= 0.0 { return -1.0; }
        g.wall_charge.carried = Some(c - 1.0);
        g.wall_charge.stats.set += 1.0;
        let line = format!("wall charge set · {} carried left", c - 1.0);
        wall_charge_note(g, &line);
        return 0.0;
    }
    let p = wall_charge_price(g, &crate::seats::here(g));
    if g.door_tools.charges < p { return -1.0; }
    let mut i = 0.0;
    while i < p { crate::doortools::door_tools_take(g, crate::doortools::DoorToolsTakeWhat::Charge); i += 1.0; }
    if g.wall_charge.roster.is_empty() { g.wall_charge.alone += p; }
    g.wall_charge.stats.set += 1.0;
    p
}

/// A charge off the wall unblown: what it cost goes back. (wallcharge.ts giveBack)
pub(super) fn wall_charge_give_back(g: &mut crate::game::Game, c_2: WallChargeId, why: &str) {
    let (free_, paid, index) = { let c = &g.tab.wallcharge_set_wall_charge[c_2]; (c.free.unwrap_or(false), c.paid, c.wall.index) };
    if free_ {
    } else if paid <= 0.0 {
        if let Some(n) = g.wall_charge.carried { g.wall_charge.carried = Some(n + 1.0); }
    } else {
        let mut i = 0.0;
        while i < paid { crate::doortools::door_tools_refund(g, crate::doortools::DoorToolsRefundWhat::Charge); i += 1.0; }
        if g.wall_charge.roster.is_empty() { g.wall_charge.alone = m::max(0.0, g.wall_charge.alone - paid); }
    }
    g.wall_charge.stats.back += 1.0;
    let line = format!("charge off wall {}: {}", index, why);
    wall_charge_note(g, &line);
}

// ---------------------------------------------------------------- the place
/// (wallcharge.ts chargeOn)
pub(super) fn wall_charge_charge_on(g: &crate::game::Game, seat: &crate::seats::SeatRef, w: &crate::wallcharge::WallSpot) -> Option<WallChargeId> {
    seat.wall_charge.set.iter().copied().find(|c| same_wall(&g.tab.wallcharge_set_wall_charge[*c].wall, w))
}

/// (wallcharge.ts chargeOnTarget)
pub fn wall_charge_charge_on_target(g: &crate::game::Game, seat: &crate::seats::SeatRef, t: &crate::orders::OrderTarget) -> Option<WallChargeId> {
    match t {
        crate::orders::OrderTarget::Floor(f) => match &f.wall { Some(w) => wall_charge_charge_on(g, seat, w), None => None },
        _ => None,
    }
}

/// The target's wall as these men would work it now: on Sam's side, slid to the nearest place that will do. (wallcharge.ts spot)
pub fn wall_charge_spot(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::wallcharge::WallSpot> {
    let w = match t {
        crate::orders::OrderTarget::Floor(f) => match &f.wall { Some(w) => w, None => return None },
        _ => return None,
    };
    let sam = sam_pos(g, &crate::seats::here(g));
    let c = seam_box(g, w.box_).c;
    let perp = if w.along_x { sam[2] - c[2] } else { sam[0] - c[0] };
    let mut q = w.clone();
    q.side = if perp >= 0.0 { 1.0 } else { -1.0 };
    Some(wall_charge_settle(g, &q))
}

/// Slides the charge along the wall to the nearest place with floor both sides. (wallcharge.ts settle)
pub fn wall_charge_settle(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> crate::wallcharge::WallSpot {
    if !w.ok || wall_charge_charge_on(g, &crate::seats::here(g), w).is_some() || wall_charge_open_at(g, w) { return w.clone(); }
    let (bc, bh) = box_ch(g, w.box_);
    let (c, l) = if w.along_x { (bc[0], bh[0]) } else { (bc[2], bh[2]) };
    let mm = m::min(g.k.wallcharge.end_m, l);
    let slide = g.k.wallcharge.slide.clone();
    for furn in [false, true] {
        for &s in &slide {
            let u = w.u + s;
            if u < c - l + mm - 1e-6 || u > c + l - mm + 1e-6 { continue; }
            let mut q = w.clone();
            q.u = u;
            if (s == 0.0 || wall_charge_charge_on(g, &crate::seats::here(g), &q).is_none()) && !wall_charge_open_at(g, &q) && wall_charge_set_spot(g, &q).is_some() && wall_charge_far_floor(g, &q, Some(furn)).is_some() { return q; }
        }
    }
    w.clone()
}

/// Nothing stands at the charge's place any more. (wallcharge.ts openAt)
pub fn wall_charge_open_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool {
    let t = thick(g, w);
    let (a, b) = (wall_pt(g, w, 0.0, FACE, Some(0.35)), wall_pt(g, w, 0.0, -(t + FACE), Some(0.35)));
    !crate::scene_collision::solid_blocked(g, a, b, None)
}

/// Where the man who sets it stands (None: no floor at the wall's foot there). (wallcharge.ts setSpot)
pub fn wall_charge_set_spot(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<V3> {
    let set_off = g.k.wallcharge.set_off;
    for strict in [true, false] {
        for du in [0.0, 0.3, -0.3] {
            let q = wall_pt(g, w, du, set_off, None);
            if free(g, q) && !(strict && wall_charge_in_frame(g, q, None)) { return Some(q); }
        }
    }
    None
}

/// Floor past the far face; `furn`: failing that, floor as the charge will leave it. (wallcharge.ts farFloor)
pub fn wall_charge_far_floor(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, furn: Option<bool>) -> Option<V3> {
    let furn = furn.unwrap_or(true);
    let t = thick(g, w);
    for d in [0.5, 0.8] {
        for du in [0.0, 0.4, -0.4] {
            let q = wall_pt(g, w, du, -(t + d), None);
            if free(g, q) { return Some(q); }
        }
    }
    if furn { wall_charge_mouth_after(g, w) } else { None }
}

/// What one middle of the hole asks of a place along the wall (the closure `through` of mouthAfter).
struct Mouth { m: f64, half: f64, wall: f64, first: Option<crate::furnblast::BlastFirst>, mouth: [V3; 1], zone: [f64; 4], t: f64, mouth_in: f64 }

fn through(g: &mut Game, w: &WallSpot, s: &Mouth, du: f64) -> Option<V3> {
    let (t, m_, half, wall, zone) = (s.t, s.m, s.half, s.wall, s.zone);
    let far = wall_pt(g, w, du, -(t + s.mouth_in), None);
    let cell = g.col.nav.cell;
    let (n0, n1) = (wall_pt(g, w, du, 0.35, None), wall_pt(g, w, du, 0.35 + cell, None));
    let (mid, behind) = (wall_pt(g, w, du, -t / 2.0, None), wall_pt(g, w, du, -(t + m::max(0.05, s.mouth_in - cell)), None));
    let ok = wall_charge_floor_after(g, w, m_, half, n0, wall, None, &s.mouth, None)
        && (free(g, n0) || free(g, n1))
        && wall_charge_floor_after(g, w, m_, half, mid, wall, s.first.clone(), &s.mouth, Some(&mut |g: &mut Game, i: f64| crate::furnblast::furniture_mouth_would_take(g, i, zone, None)))
        && wall_charge_floor_after(g, w, m_, half, behind, wall, s.first.clone(), &s.mouth, Some(&mut |g: &mut Game, i: f64| crate::furnblast::furniture_mouth_would_take(g, i, zone, None)))
        && wall_charge_floor_after(g, w, m_, half, far, wall, s.first.clone(), &s.mouth, Some(&mut |g: &mut Game, i: f64| crate::furnblast::furniture_mouth_would_take(g, i, zone, None)));
    if ok { Some(far) } else { None }
}

/// The hole's mouth once the charge has gone: for every middle the hole could have, a way through within reach of it. None = no promise. (wallcharge.ts mouthAfter)
fn wall_charge_mouth_after(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<V3> {
    let t = thick(g, w);
    let step = 0.125;
    let at = charge_at(g, w);
    let src = crate::furnblast::charge_source(g.k.walls.blast.radius);
    let first = crate::furnblast::furniture_first_to_go(g, at, src);
    let wall = match crate::wallbreak::wall_break_breakable_at(g, w.index) { Some(b) => b.id, None => w.index };
    let b = box_of_wall(g, w, wall);
    let (bc, bh) = box_ch(g, b);
    let (c, l) = if w.along_x { (bc[0], bh[0]) } else { (bc[2], bh[2]) };
    let half = g.k.walls.blast.min_hole * g.k.walls.col / 2.0;
    let (far_slack, far_reach, mouth_in) = (g.k.wallcharge.far_slack, g.k.wallcharge.far_reach, g.k.wallcharge.mouth_in);
    let (lo, hi) = (m::max(-far_slack, c - l + half - w.u), m::min(far_slack, c + l - half - w.u));
    if lo > hi + 1e-6 { return None; }
    let (mut any, mut any_at): (Option<V3>, f64) = (None, f64::INFINITY);
    let (mut mm, mut last) = (lo, false);
    while !last {
        if mm >= hi - 1e-9 { mm = hi; last = true; }
        let mouth_m = g.k.furn.mouth_m;
        let mouth = [wall_pt(g, w, mm, -(t + mouth_m), Some(1.0))];
        let (a0, a1) = (w.u + mm - half + step / 2.0, w.u + mm + half - step / 2.0);
        let tc = if w.along_x { bc[2] } else { bc[0] };
        let td = (if w.along_x { bh[2] } else { bh[0] }) + g.k.furn.mouth.depth;
        let zone = if w.along_x { [a0, tc - td, a1, tc + td] } else { [tc - td, a0, tc + td, a1] };
        let s = Mouth { m: mm, half, wall, first: first.clone(), mouth, zone, t, mouth_in };
        let mut got = through(g, w, &s, mm);
        let mut k = step;
        while k <= far_reach + 1e-6 && got.is_none() {
            got = through(g, w, &s, mm - k);
            if got.is_none() { got = through(g, w, &s, mm + k); }
            k += step;
        }
        if got.is_none() { return None; }
        if m::abs(mm) < any_at { any = got; any_at = m::abs(mm); }
        mm += step;
    }
    any
}

/// The nav cell at `q` will be open once a hole `half` wide either way of `m` is in this wall. (wallcharge.ts floorAfter)
fn wall_charge_floor_after(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, m: f64, half: f64, q: V3, wall: f64, first: Option<crate::furnblast::BlastFirst>, mouth: &[V3], goes: Option<&mut dyn FnMut(&mut crate::game::Game, f64) -> bool>) -> bool {
    use crate::m as mm;
    let mut goes = goes;
    if free(g, q) { return true; }
    let mut near: Vec<i32> = vec![0; 48];
    let n = crate::scene_collision::nav_grid_cell_blockers(g, q[0], q[2], &mut near);
    if n <= 0.0 { return false; }
    let mut ours = false;
    let mut k = 0usize;
    while (k as f64) < n {
        let i = near[k] as f64;
        k += 1;
        if i == w.index || crate::wallbreak::wall_break_breakable_at(g, i).map_or(false, |b| b.id == wall) { ours = true; continue; }
        let taken = match &first { Some(f) => crate::furnblast::furniture_would_take(g, f.clone(), i, mouth), None => false };
        if taken { continue; }
        let gone = match goes.as_mut() { Some(f) => f(g, i), None => false };
        if !gone { return false; }
    }
    if !ours { return true; }
    let cell = g.col.nav.cell;
    let (cx, cz) = ((mm::floor(q[0] / cell) + 0.5) * cell, (mm::floor(q[2] / cell) + 0.5) * cell);
    let b = box_of_wall(g, w, wall);
    let bc = seam_box(g, b).c;
    let r = 0.36;
    let off = mm::max(0.0, mm::abs(if w.along_x { cz - bc[2] } else { cx - bc[0] }) - thick(g, w) / 2.0);
    if off >= r { return true; }
    half - mm::abs((if w.along_x { cx } else { cz }) - w.u - m) >= mm::sqrt(r * r - off * off)
}

/// In a doorway: no place for a man to wait or to hold from. (wallcharge.ts inFrame)
pub(super) fn wall_charge_in_frame(g: &crate::game::Game, q: V3, m: Option<f64>) -> bool {
    let m = m.unwrap_or(g.k.wallcharge.off_frame);
    m > 0.0 && g.doors.list.iter().any(|d| flat(q, g.dr[*d].frame_centre) < m)
}

/// The wall is already open at the foot of a post `du` along it from the charge. (wallcharge.ts openFoot)
fn wall_charge_open_foot(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64) -> bool {
    let (bc, bh) = box_ch(g, w.box_);
    let (c, l) = if w.along_x { (bc[0], bh[0]) } else { (bc[2], bh[2]) };
    for e in [0.0, 0.35, -0.35] {
        let u = w.u + du + e;
        if u <= c - l || u >= c + l { continue; }
        let mut q = w.clone();
        q.u = u;
        if wall_charge_open_at(g, &q) { return true; }
        let t = thick(g, &q);
        let (a, b) = (wall_pt(g, &q, 0.0, FACE, Some(1.2)), wall_pt(g, &q, 0.0, -(t + FACE), Some(1.2)));
        if !crate::scene_collision::solid_blocked(g, a, b, None) { return true; }
    }
    false
}

/// n stand-off marks, the nearest the setter's spot first: the safe ones, else along the wall, else back across the floor. (wallcharge.ts standoffPosts)
fn wall_charge_standoff_posts(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, n: f64, spot: Option<V3>, old: Option<bool>) -> Option<Vec<V3>> {
    let spot = match spot { Some(s) => s, None => { let o = g.k.wallcharge.set_off; wall_pt(g, w, 0.0, o, None) } };
    let old = old.unwrap_or(false);
    let (sam, at) = (sam_pos(g, &crate::seats::here(g)), wall_pt(g, w, 0.0, 0.0, None));
    if !old {
        if let Some(out) = wall_charge_safe_posts(g, w, n, spot, None, None) { return Some(out); }
    }
    let (post_off, file_gap, clear_m) = (g.k.wallcharge.post_off, g.k.wallcharge.file_gap, g.k.wallcharge.clear_m);
    let (standoff, back_m) = (g.k.wallcharge.standoff.clone(), g.k.wallcharge.back_m.clone());
    let reach = if clear_m > 0.0 { g.k.walls.blast.radius * g.k.walls.blast.port_k + 0.25 + clear_m } else { 0.0 };
    let room = |out: &Vec<V3>, q: V3| !out.iter().any(|o| flat(*o, q) < 0.8) && flat(q, sam) >= 0.7;
    for &min in &standoff {
        for strict in [true, false] {
            let mut out: Vec<V3> = Vec::new();
            let mut runs: [Vec<V3>; 2] = [Vec::new(), Vec::new()];
            for (ri, dir) in [1.0, -1.0].into_iter().enumerate() {
                let lo = m::max(min, reach);
                let (mut prev, mut next, mut s) = (spot, lo, 0.45);
                let top = lo + n * file_gap + 0.5;
                while s <= top {
                    let q = wall_pt(g, w, dir * s, post_off, None);
                    if !free(g, q) || !clear(g, prev, q) { break; }
                    prev = q;
                    if s + 1e-6 >= next && !(strict && (wall_charge_in_frame(g, q, None) || wall_charge_open_foot(g, w, dir * s))) { runs[ri].push(q); next = s + file_gap; }
                    s += 0.45;
                }
            }
            let mut k = 0;
            while (out.len() as f64) < n && (k < runs[0].len() || k < runs[1].len()) {
                for r in &runs {
                    if let Some(q) = r.get(k) { if (out.len() as f64) < n && room(&out, *q) { out.push(*q); } }
                }
                k += 1;
            }
            if (out.len() as f64) < n {
                let (nx, nz) = (spot[0] - at[0], spot[2] - at[2]);
                let l = js::or(m::hyp2(nx, nz), 1.0);
                for ang in [0.0, 0.5, -0.5, 1.0, -1.0] {
                    let (cs, sn) = (m::cos(ang), m::sin(ang));
                    let (dx, dz) = ((nx * cs - nz * sn) / l, (nx * sn + nz * cs) / l);
                    for &d in &back_m {
                        if out.len() as f64 >= n { break; }
                        let q: V3 = [at[0] + dx * d, 0.0, at[2] + dz * d];
                        if free(g, q) && clear(g, spot, q) && flat(q, at) >= min && wall_off(g, w, q) > 0.3 && room(&out, q) && !(strict && wall_charge_in_frame(g, q, None)) { out.push(q); }
                    }
                }
            }
            if out.len() as f64 >= n {
                js::sort_by(&mut out, |a, b| flat(*a, spot) - flat(*b, spot));
                return Some(out);
            }
        }
    }
    None
}

/// A mark is only ever on floor the squad holds (the closure `ours` of safePosts).
fn ours(g: &mut Game, q: V3, room0: f64) -> bool {
    let r = crate::cleared::cleared_rooms_room_at(g, q);
    !js::truthy(r) || r == room0 || crate::cleared::cleared_rooms_is_known(g, r)
}

/// The walk to a place, on floor that is ours, no longer than `cap`; -1: none (the closure `way` of safePosts).
fn way(g: &mut Game, spot: V3, q: V3, cap: f64, room0: f64) -> f64 {
    let path = match crate::scene_collision::nav_grid_find_path(g, spot, q, Some(4000.0), None) { Some(p) => p, None => return -1.0 };
    let (mut len, mut a, mut held) = (0.0, spot, true);
    for b in path {
        let l = flat(a, b);
        let n = m::max(1.0, m::ceil(l / 0.5));
        let mut i = 1.0;
        while i <= n && held {
            held = ours(g, v3::lerp3(a, b, i / n), room0);
            i += 1.0;
        }
        len += l;
        a = b;
    }
    if held && len <= cap { len } else { -1.0 }
}

/// A walk's answer through the store it is kept in, else worked out now (the `kept(…, way)` of safePosts).
fn walk_kept(g: &mut Game, spot: V3, q: V3, cap: f64, room0: f64, kept: Option<&mut WallChargeKept>) -> f64 {
    let kept = match kept { Some(k) => k, None => return way(g, spot, q, cap, room0) };
    let key = format!("w|{},{}|{},{}|{}", spot[0], spot[2], q[0], q[2], cap);
    match kept {
        WallChargeKept::Menu => {
            g.wall_charge.keep.asks += 1.0;
            let have = g.wall_charge.walks.as_ref().and_then(|k| k.map.get(key.clone()).cloned());
            if let Some(WallChargeWalkAnswer::Walk { len }) = have { g.wall_charge.keep.hits += 1.0; return len; }
            let len = way(g, spot, q, cap, room0);
            if let Some(k) = g.wall_charge.walks.as_mut() { k.map.set(key, WallChargeWalkAnswer::Walk { len }); }
            len
        }
        WallChargeKept::Repick { op } => {
            let op = *op;
            let have = g.tab.wallcharge_wall_op[op].repick_kept.as_ref().and_then(|c| c.get(key.clone()).copied());
            if let Some(v) = have { return v; }
            let v = way(g, spot, q, cap, room0);
            g.tab.wallcharge_wall_op[op].repick_kept.get_or_insert_with(JsMap::new).set(key, v);
            v
        }
        WallChargeKept::Local { memo } => {
            if let Some(v) = memo.get(key.clone()) { return *v; }
            let v = way(g, spot, q, cap, room0);
            memo.set(key, v);
            v
        }
    }
}

/// S.reach, through the open menu's store when `menu` (the closure `reach` of plan: only keptFor's store holds refusals).
fn reach_kept(g: &mut Game, gd: GuardId, to: V3, menu: bool) -> Option<String> {
    if !menu { return crate::orders::squad_orders_reach(g, gd, to); }
    let cs = g.col.nav.cell;
    let (id, p) = (g.ch[g.gd[gd].ch].id, man_pos(g, gd));
    let key = format!("r|{}|{},{}|{},{}", id, m::floor(p[0] / cs), m::floor(p[2] / cs), to[0], to[2]);
    g.wall_charge.keep.asks += 1.0;
    let have = g.wall_charge.walks.as_ref().and_then(|k| k.map.get(key.clone()).cloned());
    if let Some(WallChargeWalkAnswer::Reach { why }) = have { g.wall_charge.keep.hits += 1.0; return why; }
    let why = crate::orders::squad_orders_reach(g, gd, to);
    if let Some(k) = g.wall_charge.walks.as_mut() { k.map.set(key, WallChargeWalkAnswer::Reach { why: why.clone() }); }
    why
}

/// Something solid holds between the blast's reach and a chest at `p`, of the world the caller asks it of (the closure `covered` of safePosts).
fn covered(g: &mut Game, over: &Over, src: V3, open: f64, p: V3) -> bool {
    let chest: V3 = [p[0], 1.1, p[2]];
    let from = v3::lerp3(src, chest, open / v3::dist(src, chest));
    crate::scene_collision::solid_blocked_over(g, over, from, chest, None) && crate::scene_collision::segment_blocked_over(g, over, from, chest, None, None)
}

/// The safe stand-off: n marks out of the blast, the shortest walk back first; None: the floor has not n of them. (wallcharge.ts safePosts)
pub fn wall_charge_safe_posts(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, n: f64, spot: V3, can: Option<&mut dyn FnMut(&mut crate::game::Game, V3) -> bool>, kept: Option<&mut crate::wallcharge::WallChargeKept>) -> Option<Vec<V3>> {
    let (mut can, mut kept) = (can, kept);
    let (sam, at) = (sam_pos(g, &crate::seats::here(g)), wall_pt(g, w, 0.0, 0.0, None));
    let r = safe_radius(g, w);
    let (post_off, file_gap, hide) = (g.k.wallcharge.post_off, g.k.wallcharge.file_gap, g.k.wallcharge.safe_hide_m);
    let cap = m::min(g.k.wallcharge.safe_walk_m, g.k.wallcharge.safe_back_m);
    // a place and the walk measured to it (the Map `walk`, keyed by the array's identity in the TypeScript)
    let mut pool: Vec<(V3, Option<f64>)> = Vec::new();
    let mut fan: Vec<V3> = Vec::new();
    let mut fire = fire_new(g, Some(w), None);
    let room0 = crate::cleared::cleared_rooms_room_at(g, spot);
    for dir in [1.0, -1.0] {
        let (mut prev, mut next, mut s) = (spot, r.standoff, 0.45);
        let top = r.standoff + n * file_gap + 0.5;
        while s <= top {
            let q = wall_pt(g, w, dir * s, post_off, None);
            if !free(g, q) || !clear(g, prev, q) { break; }
            prev = q;
            if s + 1e-6 >= next && !wall_charge_in_frame(g, q, None) && !wall_charge_open_foot(g, w, dir * s) && ours(g, q, room0) { pool.push((q, None)); next = s + file_gap; }
            s += 0.45;
        }
    }
    {
        let (nx, nz) = (spot[0] - at[0], spot[2] - at[2]);
        let l = js::or(m::hyp2(nx, nz), 1.0);
        for ang in [0.0, 0.5, -0.5, 1.0, -1.0] {
            let (cs, sn) = (m::cos(ang), m::sin(ang));
            let (dx, dz) = ((nx * cs - nz * sn) / l, (nx * sn + nz * cs) / l);
            for d in [0.1, 1.0, 1.9] {
                let q: V3 = [at[0] + dx * (r.standoff + d), 0.0, at[2] + dz * (r.standoff + d)];
                if free(g, q) && clear(g, spot, q) && wall_off(g, w, q) > 0.3 && !wall_charge_in_frame(g, q, None) && ours(g, q, room0) { fan.push(q); }
            }
        }
    }
    {
        let mut hid: Vec<V3> = Vec::new();
        let open = g.k.walls.blast.radius * g.k.walls.blast.port_k + 0.25;
        let lo = m::max(g.k.wallcharge.standoff[0], open + 0.6);
        let src = wall_pt(g, w, 0.0, 0.5, Some(1.1));
        let statics = Over::statics();   // (before the blast a door leaf is no cover)
        let mut x = -hide;
        while x <= hide + 1e-6 {
            let mut z = -hide;
            while z <= hide + 1e-6 {
                let q: V3 = [at[0] + x, 0.0, at[2] + z];
                z += 0.9;
                let d = flat(q, at);
                if d < lo || d > hide || !free(g, q) || wall_off(g, w, q) <= 0.3 || wall_charge_in_frame(g, q, None) || !ours(g, q, room0) { continue; }
                // (a place is good if the charge's own line to it is cut, and to a step off it every way there is floor)
                let mut good = covered(g, &statics, src, open, q);
                for (dx, dz) in [(0.4, 0.0), (-0.4, 0.0), (0.0, 0.4), (0.0, -0.4)] {
                    if !good { break; }
                    let p: V3 = [q[0] + dx, 0.0, q[2] + dz];
                    good = !free(g, p) || covered(g, &statics, src, open, p);
                }
                if good { hid.push(q); }
            }
            x += 0.9;
        }
        js::sort_by(&mut hid, |a, b| flat(*a, spot) - flat(*b, spot));
        let mut late: Vec<V3> = Vec::new();
        for pass in [0, 1] {
            let (mut walks, mut tries) = (0.0, 0.0);
            let list = if pass == 1 { core::mem::take(&mut late) } else { core::mem::take(&mut hid) };
            for k in 0..list.len() {
                let q = list[k];
                if walks >= n + 8.0 || tries >= 3.0 * (n + 8.0) {
                    if pass == 0 { late.extend_from_slice(&list[k..]); }
                    break;
                }
                if pool.iter().any(|o| flat(o.0, q) < 0.8) { continue; }
                if pass == 0 && wall_charge_fire_any(g, &mut fire, q) { late.push(q); continue; }
                tries += 1.0;
                let len = walk_kept(g, spot, q, cap, room0, kept.as_deref_mut());
                if len >= 0.0 { pool.push((q, Some(len))); walks += 1.0; }
            }
        }
    }
    js::sort_by(&mut pool, |a, b| a.1.unwrap_or_else(|| flat(a.0, spot)) - b.1.unwrap_or_else(|| flat(b.0, spot)));
    js::sort_by(&mut fan, |a, b| flat(*a, spot) - flat(*b, spot));
    let mut all: Vec<(V3, f64)> = Vec::with_capacity(pool.len() + fan.len());
    for q in pool.iter().map(|o| o.0).chain(fan.iter().copied()) {
        let s = wall_charge_fire_score(g, &mut fire, q);
        all.push((q, s));
    }
    js::sort_by(&mut all, |a, b| a.1 - b.1);
    let mut out: Vec<V3> = Vec::new();
    for (q, _) in all {
        if out.len() as f64 >= n { break; }
        if !out.iter().any(|o| flat(*o, q) < 0.8) && flat(q, sam) >= 0.7 {
            let may = match can.as_mut() { Some(f) => f(g, q), None => true };
            if may { out.push(q); }
        }
    }
    if out.len() as f64 >= n { Some(out) } else { None }
}

/// Each living enemy in the team's picture now, where he stands or was last seen. (wallcharge.ts knownFoes)
pub fn wall_charge_known_foes(g: &mut crate::game::Game) -> Vec<V3> {
    let mut out = Vec::new();
    let held = g.pl[g.player].holding.as_ref().map(|h| h.g);
    let known_s = g.k.wallcharge.fire_known_s;
    if crate::fogofwar::fog_of_war_active(g) {
        let men: Vec<GuardId> = g.guards.iter().copied().filter(|m| g.gd[*m].team == crate::teams::Team::Operative).collect();
        for k in crate::picture::known_places(g, &men, known_s) {
            if let crate::picture::Who::Guard(h) = k.who { if counts(g, h, held) { out.push(k.at); } }
        }
        return out;
    }
    for h in g.guards.clone() {
        if !counts(g, h, held) { continue; }
        if let Some(s) = crate::fogofwar::fog_of_war_sighting(g, h) {
            if s.age >= 0.0 && s.age <= known_s { out.push(s.at); }
        }
    }
    out
}

fn counts(g: &mut Game, h: GuardId, held: Option<GuardId>) -> bool {
    let team = g.gd[h].team;
    Some(h) != held && g.gd[h].state != crate::game::GuardState::Dead && g.ch[g.gd[h].ch].alive && crate::teams::is_hostile(g, team, crate::teams::Team::Player)
}

fn fire_new(g: &mut Game, w: Option<&WallSpot>, all: Option<&[V3]>) -> WallChargeFireAsk {
    let all = match all { Some(a) => a.to_vec(), None => wall_charge_known_foes(g) };
    WallChargeFireAsk::Lines { wall: w.cloned(), all, memo: JsMap::new(), hit: JsMap::new() }
}

/// How much under the known enemies' fire a place is: what its two answers close over. (wallcharge.ts underFire)
pub fn wall_charge_under_fire(g: &mut crate::game::Game, w: Option<&crate::wallcharge::WallSpot>, all: Option<&[V3]>) -> crate::wallcharge::WallChargeUnderFire {
    WallChargeUnderFire { score: fire_new(g, w, all) }
}

/// How much of the blast a place on the charge's own side takes: 0 = none, up to 1 at the wall. (wallcharge.ts blastShare)
fn wall_charge_blast_share_of(g: &crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3, r: Option<crate::wallcharge::WallChargeAwayMarksValueR>, leaves: Option<bool>) -> f64 {
    let r = match r { Some(r) => r, None => safe_radius_of(g, w) };
    let leaves = leaves.unwrap_or(true);
    let at = wall_pt(g, w, 0.0, 0.0, None);
    let d = flat(p, at);
    let t = thick(g, w);
    if d >= r.reach || wall_off(g, w, p) < -t / 2.0 { return 0.0; }
    let (a, b): (V3, V3) = (wall_pt(g, w, 0.0, 0.5, Some(1.1)), [p[0], 1.1, p[2]]);
    let cut = crate::ask::WorldAsk::of(g).over(if leaves { Over::OWN } else { Over::statics() }).line(a, b, crate::ask::Line::Solid);
    if cut { 0.0 } else { 1.0 - d / r.reach }
}
/// …for a rule that holds the game: the copy of the boxes stood first
pub fn wall_charge_blast_share(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3, r: Option<crate::wallcharge::WallChargeAwayMarksValueR>, leaves: Option<bool>) -> f64 {
    crate::scene_collision::stands(g);
    wall_charge_blast_share_of(g, w, p, r, leaves)
}

/// The charge's radius, read once while it is on the wall, and the same with the stand-off distance for its reach (the closure `rOf` of clearOf).
fn r_of(g: &mut Game, c: WallChargeId) -> (WallChargeAwayMarksValueR, WallChargeAwayMarksValueR) {
    if let Some(m_) = g.wall_charge.away_marks.get(c) { return (m_.r, m_.pad); }
    let w = wall_of(g, c);
    let r = safe_radius(g, &w);
    let mut pad = r;
    pad.reach = m::max(r.reach, r.standoff - g.k.wallcharge.men_clear_in);
    g.wall_charge.away_marks.set(c, WallChargeAwayMarksValue { n: 0.0, marks: Vec::new(), t: -1e9, sets: 0.0, r, pad });
    (r, pad)
}

fn hit(g: &mut Game, c: WallChargeId, q: V3) -> bool {
    let w = wall_of(g, c);
    let pad = r_of(g, c).1;
    wall_charge_blast_share(g, &w, q, Some(pad), Some(false)) > 0.0
}

/// Out of EVERY charge on a wall.
fn out_of_all(g: &mut Game, q: V3) -> bool {
    for o in g.wall_charge.set.clone() { if hit(g, o, q) { return false; } }
    true
}

fn free_mark(g: &mut Game, q: V3, taken: &[V3], others: &[GuardId]) -> bool {
    !taken.iter().any(|o| flat(*o, q) < 0.8) && !others.iter().any(|m_| flat(man_pos(g, *m_), q) < 0.6) && out_of_all(g, q)
}

/// A follower's place is in the blast of a set charge: the stand-off mark he walks to instead. None: clear, no mark, or nothing set. (wallcharge.ts clearOf)
pub fn wall_charge_clear_of(g: &mut crate::game::Game, gd: GuardId, where_: &mut dyn FnMut(&mut crate::game::Game) -> V3) -> Option<V3> {
    let t = g.time;
    if g.wall_charge.set.is_empty() {
        g.wall_charge.away.clear();
        g.wall_charge.away_marks.clear();
        g.wall_charge.away_none.clear();
        return None;
    }
    for c in g.wall_charge.away_marks.keys() { if !g.wall_charge.set.contains(&c) { g.wall_charge.away_marks.delete(c); } }
    for m_ in g.wall_charge.away.keys() {
        let a = *g.wall_charge.away.get(m_).unwrap();
        if g.gd[m_].state == crate::game::GuardState::Dead || !g.wall_charge.set.contains(&a.c) || (m_ != gd && m::abs(t - a.t) > 0.5) { g.wall_charge.away.delete(m_); }
    }
    let mut mine = g.wall_charge.away.get(gd).copied();
    if let Some(a) = mine {
        if !out_of_all(g, a.at) { g.wall_charge.away.delete(gd); mine = None; }
    }
    let place = where_(g);
    let mut c = None;
    for o in g.wall_charge.set.clone() { if hit(g, o, place) { c = Some(o); break; } }
    if let Some(a) = mine {
        let mut hit_t = a.hit_t;
        if c.is_some() || t < hit_t { hit_t = t; }
        else if t - hit_t >= g.k.wallcharge.men_clear_s { g.wall_charge.away.delete(gd); return None; }
        if let Some(row) = g.wall_charge.away.get_mut(gd) { row.t = t; row.hit_t = hit_t; }
        return Some(a.at);
    }
    let c = c?;
    if let Some(none) = g.wall_charge.away_none.get(gd).copied() { if t >= none && t - none < 1.0 { return None; } }
    g.wall_charge.away_none.delete(gd);
    let p = man_pos(g, gd);
    let mut taken: Vec<V3> = Vec::new();
    for (m_, a) in g.wall_charge.away.iter() { if m_ != gd { taken.push(a.at); } }
    for op in g.wall_charge.ops.iter() { for q in g.tab.wallcharge_wall_op[*op].post.values() { taken.push(*q); } }
    let others: Vec<GuardId> = crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None).into_iter().filter(|m_| *m_ != gd).collect();
    let n = m::max(1.0, (others.len() + 1) as f64);
    r_of(g, c);
    let (mn, msets, mt) = { let row = g.wall_charge.away_marks.get(c).unwrap(); (row.n, row.sets, row.t) };
    let sets = g.wall_charge.set.len() as f64;
    if mn < n || msets != sets || m::abs(t - mt) > 2.0 {
        let w = wall_of(g, c);
        let spot = match wall_charge_set_spot(g, &w) { Some(s) => s, None => { let o = g.k.wallcharge.set_off; wall_pt(g, &w, 0.0, o, None) } };
        let mut kept = WallChargeKept::Local { memo: JsMap::new() };
        let (mut marks, mut k): (Option<Vec<V3>>, f64) = (None, n + 2.0);
        while k >= 1.0 && marks.is_none() {
            marks = wall_charge_safe_posts(g, &w, k, spot, Some(&mut |g: &mut Game, q: V3| out_of_all(g, q)), Some(&mut kept));
            k -= 1.0;
        }
        if let Some(row) = g.wall_charge.away_marks.get_mut(c) { row.n = n; row.marks = marks.unwrap_or_default(); row.t = t; row.sets = sets; }
    }
    let mut at: Option<V3> = None;
    let mut all: Vec<V3> = Vec::new();
    for o in g.wall_charge.set.clone() { r_of(g, o); all.extend_from_slice(&g.wall_charge.away_marks.get(o).unwrap().marks); }
    js::sort_by(&mut all, |a, b| flat(p, *a) - flat(p, *b));
    for q in all {
        if free_mark(g, q, &taken, &others) && crate::orders::squad_orders_reach(g, gd, q).is_none() { at = Some(q); break; }
    }
    if at.is_none() {
        let mut order = vec![c];
        for k in g.wall_charge.set.clone() { if k != c { order.push(k); } }
        for o in order {
            let wo = wall_of(g, o);
            let w_ = wall_pt(g, &wo, 0.0, 0.0, None);
            let (dx, dz) = (p[0] - w_[0], p[2] - w_[2]);
            let l = m::hyp2(dx, dz);
            let r = r_of(g, o).0;
            if at.is_some() || l <= 0.2 { continue; }
            for d in [0.0, 0.9, 1.8] {
                let q: V3 = [w_[0] + dx / l * (r.standoff + d), 0.0, w_[2] + dz / l * (r.standoff + d)];
                if free(g, q) && crate::scene_collision::nav_grid_walkable(g, p, q) && !wall_charge_in_frame(g, q, None) && free_mark(g, q, &taken, &others) { at = Some(q); break; }
            }
        }
    }
    let at = match at { Some(q) => q, None => { g.wall_charge.away_none.set(gd, t); return None; } };
    let mark: V3 = [at[0], 0.0, at[2]];
    g.wall_charge.away.set(gd, WallChargeAwayValue { c, at: mark, hit_t: t, t });
    g.wall_charge.stats.cleared += 1.0;
    let line = format!("{} steps back from the charge on wall {}", g.gd[gd].callsign, g.tab.wallcharge_set_wall_charge[c].wall.index);
    wall_charge_note(g, &line);
    Some(mark)
}

/// Who is inside the radius of a set charge right now: 'you' and the seat's men by callsign. (wallcharge.ts tooClose)
pub fn wall_charge_too_close_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: Option<WallChargeId>) -> Vec<String> {
    let c = match c_2 { Some(c) => c, None => return Vec::new() };
    let w = wall_of(g, c);
    let r = safe_radius_of(g, &w);
    let mut out = Vec::new();
    if !g.pl[seat.player].down {
        let p = sam_pos(g, seat);
        if wall_charge_blast_share_of(g, &w, p, Some(r), Some(false)) > 0.0 { out.push(String::from("you")); }
    }
    for gd in crate::orders::squad_orders_men(g, seat, None, None) {
        let p = man_pos(g, gd);
        if wall_charge_blast_share_of(g, &w, p, Some(r), Some(false)) > 0.0 { out.push(g.gd[gd].callsign.clone()); }
    }
    out
}
/// …for a rule that holds the game: the copy of the boxes stood first
pub fn wall_charge_too_close(g: &mut crate::game::Game, c_2: Option<WallChargeId>) -> Vec<String> {
    crate::scene_collision::stands(g);
    wall_charge_too_close_of(g, &crate::seats::here(g), c_2)
}

/// A clear line to `p` from the far face at the charge or a step either side, door leaves out (the closure `lined` of behind).
fn lined(g: &Game, w: &WallSpot, t: f64, p: V3) -> bool {
    let world = crate::ask::WorldAsk::of(g).over(Over::statics());
    for du in [0.0, 0.4, -0.4] {
        let a = wall_pt(g, w, du, -(t + 0.45), Some(1.1));
        if !world.line(a, [p[0], 1.1, p[2]], crate::ask::Line::Solid) { return true; }
    }
    false
}

/// Who the blast would catch beyond the wall right now. (wallcharge.ts behind)
fn wall_charge_behind_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: Option<WallChargeId>) -> Vec<String> {
    let c = match c_2 { Some(c) => c, None => return Vec::new() };
    let w = wall_of(g, c);
    let (t, at) = (thick(g, &w), wall_pt(g, &w, 0.0, 0.0, None));
    let stun_m = g.k.doortools.stun_m;
    let mut out = Vec::new();
    {
        let p = sam_pos(g, seat);
        if !g.pl[seat.player].down && g.puppet.is_none() && wall_off(g, &w, p) < -t / 2.0 && flat(p, at) < stun_m && lined(g, &w, t, p) { out.push(String::from("you")); }
    }
    for gd in crate::orders::squad_orders_men(g, seat, None, None) {
        let p = man_pos(g, gd);
        if wall_off(g, &w, p) < -t / 2.0 && flat(p, at) <= stun_m && lined(g, &w, t, p) { out.push(g.gd[gd].callsign.clone()); }
    }
    out
}

/// The trigger's warning in words ('' = nobody). (wallcharge.ts closeWords)
pub(super) fn wall_charge_close_words_of(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: Option<WallChargeId>, caps: Option<bool>) -> String {
    let caps = caps.unwrap_or(false);
    let (near, far) = (wall_charge_too_close_of(g, seat, c_2), wall_charge_behind_of(g, seat, c_2));
    let mut s = String::new();
    if !near.is_empty() { s.push_str(if caps { " · TOO CLOSE: " } else { " · too close: " }); s.push_str(&near.join(", ")); }
    if !far.is_empty() { s.push_str(if caps { " · BEHIND THE WALL: " } else { " · behind the wall: " }); s.push_str(&far.join(", ")); }
    s
}
/// …for a rule that holds the game: the copy of the boxes stood first
pub fn wall_charge_close_words(g: &mut crate::game::Game, c_2: Option<WallChargeId>, caps: Option<bool>) -> String {
    crate::scene_collision::stands(g);
    wall_charge_close_words_of(g, &crate::seats::here(g), c_2, caps)
}

/// A teammate outside the order in the blast: the word for it. (wallcharge.ts inBlast)
fn wall_charge_in_blast(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, men: &[GuardId]) -> Option<String> {
    let t = thick(g, w);
    let (at, out, inn) = (wall_pt(g, w, 0.0, 0.0, None), wall_pt(g, w, 0.0, 0.5, Some(1.1)), wall_pt(g, w, 0.0, -(t + 0.45), Some(1.1)));
    let (stun_m, mate_m) = (g.k.doortools.stun_m, g.k.wallcharge.mate_m);
    for gd in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
        if men.contains(&gd) { continue; }
        let p = man_pos(g, gd);
        let (d, chest): (f64, V3) = (flat(p, at), [p[0], 1.1, p[2]]);
        if wall_off(g, w, p) < -t {
            if d < stun_m && !crate::scene_collision::solid_blocked(g, inn, chest, None) { return Some(format!("{} is behind that wall.", g.gd[gd].callsign)); }
        } else if d < mate_m && !crate::scene_collision::solid_blocked(g, out, chest, None) {
            return Some(format!("{} is too close to that wall.", g.gd[gd].callsign));
        }
    }
    None
}

fn deal_of(g: &mut Game, lead: GuardId, rest: &[GuardId], posts: &[V3]) -> Vec<(GuardId, V3)> {
    let mut d = vec![(lead, posts[0])];
    if !rest.is_empty() { d.extend(crate::orders::squad_orders_pair(g, rest, &posts[1..], None, None)); }
    d
}

fn walk_of(g: &mut Game, d: &[(GuardId, V3)], menu: bool) -> Option<String> {
    for (gd, p) in d {
        let w = reach_kept(g, *gd, *p, menu);
        if w.is_some() { return w; }
    }
    None
}

/// Who sets it, from where, and where everybody stands off, or the word for why not. (wallcharge.ts plan)
pub(super) fn wall_charge_plan(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, men: &[GuardId], kept: Option<&crate::wallcharge::WallChargeKept>) -> crate::wallcharge::WallChargePlan {
    let text = |s: &str| WallChargePlan::Text(String::from(s));
    if men.is_empty() { return text("nobody to take it."); }
    let mut kept = kept.cloned();
    let menu = matches!(kept, Some(WallChargeKept::Menu));
    let spot = match wall_charge_set_spot(g, w) { Some(s) => s, None => return text("no room at that wall.") };
    if wall_charge_far_floor(g, w, None).is_none() { return text("no floor behind it."); }
    let mut sorted = men.to_vec();
    { let by = |a: &GuardId, b: &GuardId| flat(man_pos(g, *a), spot) - flat(man_pos(g, *b), spot); js::sort_by(&mut sorted, by); }
    let lead = sorted[0];
    let rest: Vec<GuardId> = men.iter().copied().filter(|m_| *m_ != lead).collect();
    let n = men.len() as f64;
    let posts = wall_charge_safe_posts(g, w, n, spot, Some(&mut |g: &mut Game, q: V3| reach_kept(g, lead, q, menu).is_none()), kept.as_mut());
    if let Some(posts) = posts {
        if wall_charge_in_blast(g, w, men).is_none() && reach_kept(g, lead, spot, menu).is_none() {
            let d = deal_of(g, lead, &rest, &posts);
            if walk_of(g, &d, menu).is_none() { return WallChargePlan::Arm1(WallChargePlanArm1 { lead, spot, deal: d, close: false }); }
        }
    }
    let posts = match wall_charge_standoff_posts(g, w, n, Some(spot), Some(true)) { Some(p) => p, None => return text("no room to stand off.") };
    if let Some(mate) = wall_charge_in_blast(g, w, men) { return WallChargePlan::Text(mate); }
    if let Some(w0) = reach_kept(g, lead, spot, menu) { return WallChargePlan::Text(w0); }
    let d = deal_of(g, lead, &rest, &posts);
    if let Some(why) = walk_of(g, &d, menu) { return WallChargePlan::Text(why); }
    WallChargePlan::Arm1(WallChargePlanArm1 { lead, spot, deal: d, close: true })
}

/// The walks of the open menu, kept between its re-reads, when `t` is that menu's own target. (wallcharge.ts keptFor)
pub(super) fn wall_charge_kept_for(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) -> Option<crate::wallcharge::WallChargeKept> {
    use crate::orders::OrderTarget;
    if !g.wall_charge.keep.on { return None; }
    // `M.target === t`: the serial the menu's own target was stamped with as it opened; a target made by anyone else has serial 0
    let own = match (g.orders.menu.as_ref().map(|m_| &m_.target), t) {
        (Some(OrderTarget::Floor(a)), OrderTarget::Floor(b)) if b.serial != 0 && a.serial == b.serial => b.serial,
        _ => return None,
    };
    Some(wall_charge_kept_walks(g, own))
}
