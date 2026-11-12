use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the frame's input: what the order input takes out of it

fn pressed_delete(g: &mut Game, input: InputId, code: &str) -> bool {
    g.tab.input[input].pressed.delete(String::from(code))
}
fn mouse(g: &Game, input: InputId) -> (f64, f64) {
    let i = &g.tab.input[input];
    (i.mouse_x, i.mouse_y)
}
/// `input.clicked &= ~bits`
fn clicked_off(g: &mut Game, input: InputId, bits: i32) {
    let i = &mut g.tab.input[input];
    i.clicked = (js::to_i32(i.clicked) & !bits) as f64;
}
/// `input.buttons &= ~bits`
fn buttons_off(g: &mut Game, input: InputId, bits: i32) {
    let i = &mut g.tab.input[input];
    i.buttons = (js::to_i32(i.buttons) & !bits) as f64;
}
/// `input.down('ShiftLeft') || input.down('ShiftRight')`
fn shift_down(g: &Game, input: InputId) -> bool {
    crate::input::input_down(g, input, "ShiftLeft") || crate::input::input_down(g, input, "ShiftRight")
}
/// `if (input === g.seat.input) g.seat.swallow(1)`
fn swallow_if_seat(g: &mut Game, input: InputId) {
    let s = g.seat;
    if g.tab.cmd_seat[s].input == input { crate::cmd::seat_swallow(g, s, 1.0); }
}

// ---------------------------------------------------------------- small readings

/// `v[i] ?? -1`
fn seat_at(v: &[f64], i: f64) -> f64 {
    v.get(i as usize).copied().unwrap_or(-1.0)
}
fn target_kind(t: &OrderTarget) -> &'static str {
    match t {
        OrderTarget::Floor(_) => "floor",
        OrderTarget::Door(_) => "door",
        OrderTarget::None(_) => "none",
        OrderTarget::Foe(_) => "foe",
    }
}
/// `{ kind: 'none', at: [p[0], 0, p[2]] }`, p where Sam stands
fn at_sam(g: &Game, seat: &crate::seats::SeatRef) -> OrderTarget {
    let p = g.seat_pos(seat);
    OrderTarget::None(OrderTargetNone { at: [p[0], 0.0, p[2]], man: None })
}
fn opts_men(men: Vec<GuardId>, ack: Option<String>) -> GiveOpts {
    GiveOpts { ack, men: Some(men) }
}
/// `R.lit = 0; R.armed = false`, and `R.more` as given
fn unlight(g: &mut Game, more: Option<bool>) {
    if let Some(r) = g.groups[g.group].orders.cmd_ring.as_mut() {
        if more.is_some() { r.more = more; }
        r.lit = 0.0;
        r.armed = false;
    }
}
fn ring_more(r: Option<crate::cmdring::RingState>) -> bool {
    r.is_some_and(|r| r.more.unwrap_or(false))
}
/// `this.seats(this.menu!, R)`
fn seats_now(g: &Game, r: Option<crate::cmdring::RingState>) -> SquadOrdersSeats {
    let m = g.groups[g.group].orders.menu.clone().unwrap();
    squad_orders_seats(g, &m, r)
}
fn away(g: &mut Game) {
    g.groups[g.group].orders.menu = None;
    g.groups[g.group].orders.cmd_ring = None;
}

/// THE RING'S INPUT, ahead of the menu's own: the frame's order input is the ring's if true.
pub(super) fn squad_orders_ring_input(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> bool {
    if g.groups[g.group].orders.cmd_ring.is_some() && g.groups[g.group].orders.menu.is_none() { g.groups[g.group].orders.cmd_ring = None; }
    if g.groups[g.group].orders.cmd_ring.is_none() { return no_ring_yet(g, input, cam, canvas); }
    // what its rows say, all at one moment: every wedge of this frame is laid out over these words, the tick's and a HUD's
    super::read_words(g);
    let still = ring_stays_on_its_point(g, input, cam, canvas);
    pressed_delete(g, input, crate::consts::INTERACT_KEY);
    let hand = hand_this_frame(g, input);
    if !rows_fresh(g) { return true; }
    if hand.blur || hand.back { away(g); return true; }
    if ring_more(g.groups[g.group].orders.cmd_ring) && seats_now(g, None).over.is_empty() { unlight(g, Some(false)); }
    if hand.rmb {
        if ring_more(g.groups[g.group].orders.cmd_ring) { unlight(g, Some(false)); }
        else { away(g); }
        return true;
    }
    let click = if hand.lmb {
        let Some(click) = left_click(g, input, cam, canvas) else { return true };
        click
    } else { false };
    let Some(lit) = wedge_lit(g, input, hand.digit, still) else { return true };
    if lit_wedge_is_given(g, input, hand.digit, click, lit) { give_the_lit_wedge(g); }
    true
}

/// With no ring up: the key down and let go is a tap; still down when the hold is over, the ring goes up.
fn no_ring_yet(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> bool {
    if let Some(p) = g.groups[g.group].orders.cmd_press.clone() { return key_still_held(g, input, cam, canvas, &p); }
    let key = g.k.cmdring.key.clone();
    if !crate::input::input_hit(g, input, &key) || !squad_orders_ring_free(g, &crate::seats::here(g), input) { return false; }
    pressed_delete(g, input, &key);
    let add = crate::planmode::goals_on(g, &crate::seats::here(g)) && shift_down(g, input);
    if g.k.cmdring.hold_secs > 0.0 && !crate::input::input_down(g, input, &key) {
        let t = squad_orders_target_under(g, input, cam, canvas);
        squad_orders_tap_give(g, &t, input, Some(add));
        return true;
    }
    if g.k.cmdring.hold_secs > 0.0 {
        let t0 = g.time;
        let t = squad_orders_target_under(g, input, cam, canvas);
        g.groups[g.group].orders.cmd_press = Some(SquadOrdersCmdPress { t0, t, add: if add { Some(true) } else { None } });
        if squad_orders_ring_digit(g, input) { squad_orders_ring_input(g, input, cam, canvas); }
        return true;
    }
    let t = squad_orders_target_under(g, input, cam, canvas);
    match goal_part_or_nobody(g, &t, add) {
        None => true,
        Some(part) => { let t0 = g.time; squad_orders_ring_up(g, &t, input, canvas, t0, Some(cam), part.as_ref()) }
    }
}

/// The press goes on: let go it is a tap; a digit, or the hold over, puts the ring up on what the press was on.
fn key_still_held(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View, p: &SquadOrdersCmdPress) -> bool {
    let key = g.k.cmdring.key.clone();
    if crate::input::input_hit(g, input, "Blur") || crate::input::input_hit(g, input, "Paused") { g.groups[g.group].orders.cmd_press = None; return false; }
    if !crate::input::input_down(g, input, &key) {
        g.groups[g.group].orders.cmd_press = None;
        let goal = (p.add.unwrap_or(false) || shift_down(g, input)) && crate::planmode::goals_on(g, &crate::seats::here(g));
        squad_orders_tap_give(g, &p.t, input, Some(goal));
        return true;
    }
    let digit = squad_orders_ring_digit(g, input);
    if !digit && g.time - p.t0 < crate::cmdring::ring_hold_secs(g) - 1e-6 { return false; }
    g.groups[g.group].orders.cmd_press = None;
    if !squad_orders_back_free(g, &crate::seats::here(g)) { return false; }
    let add = p.add.unwrap_or(false) || shift_down(g, input);
    let Some(part) = goal_part_or_nobody(g, &p.t, add) else { return true };
    let t0 = g.time - g.k.cmdring.tap_secs;
    if !squad_orders_ring_up(g, &p.t, input, canvas, t0, Some(cam), part.as_ref()) { return false; }
    if digit { squad_orders_ring_input(g, input, cam, canvas) } else { true }
}

/// It stays on its point: its middle put back over the place it was opened on. True: the hand has not moved, and a cursor that has not moved lights no other wedge.
fn ring_stays_on_its_point(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> bool {
    let (w, h) = (canvas.client_width, canvas.client_height);
    let Some(at) = g.groups[g.group].orders.cmd_ring.unwrap().at else { return false };
    let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, at, w, h, None, None);
    if front {
        // ring_pin reads only the table of numbers on g: R goes back as it comes out
        let mut r = g.groups[g.group].orders.cmd_ring.unwrap();
        crate::cmdring::ring_pin(g, &mut r, sx, sy, w, h);
        g.groups[g.group].orders.cmd_ring = Some(r);
    }
    let (mx, my) = mouse(g, input);
    let Some(r) = g.groups[g.group].orders.cmd_ring.as_mut() else { return false };
    let still = r.mx == Some(mx) && r.my == Some(my);
    r.mx = Some(mx);
    r.my = Some(my);
    still
}

/// What the hand did this frame.
#[derive(Clone, Copy)]
struct RingHand {
    back: bool,
    blur: bool,
    rmb: bool,
    lmb: bool,
    /// the first digit down, or 0
    digit: f64,
}

/// The frame's keys are the ring's while it is up: taken out of the input, the digits and the wheel all of them.
fn hand_this_frame(g: &mut Game, input: InputId) -> RingHand {
    let back = crate::backkey::back_take(g, input);
    let blur = crate::input::input_hit(g, input, "Blur") || crate::input::input_hit(g, input, "Paused");
    let rmb = crate::input::input_rmb_hit(g, input);
    if rmb { clicked_off(g, input, 4); }
    let lmb = crate::input::input_lmb_hit(g, input);
    let mut digit = 0.0;
    for d in 1..=9 {
        if pressed_delete(g, input, &format!("Digit{}", d)) && !js::truthy(digit) { digit = d as f64; }
    }
    g.tab.input[input].wheel = 0.0;
    RingHand { back, blur, rmb, lmb, digit }
}

/// The menu's rows read again. False: the menu has gone, and the ring with it.
fn menu_survives_a_read(g: &mut Game) -> bool {
    squad_orders_refresh(g);
    if g.groups[g.group].orders.menu.is_some() { return true; }
    g.groups[g.group].orders.cmd_ring = None;
    false
}

/// Rows gone stale are read again; a row that came or went under the ring makes the wedges other rows, and none is lit until the hand lights one. False: the menu has gone.
fn rows_fresh(g: &mut Game) -> bool {
    let read_t = g.groups[g.group].orders.menu.as_ref().unwrap().read_t;
    let stale = g.time - read_t >= g.k.orders.refresh_secs || {
        let menu = g.groups[g.group].orders.menu.clone().unwrap();
        squad_orders_due(g, &menu)
    };
    if !stale { return true; }
    let r = g.groups[g.group].orders.cmd_ring;
    let was = seats_now(g, r).row;
    if !menu_survives_a_read(g) { return false; }
    let r = g.groups[g.group].orders.cmd_ring;
    if seats_now(g, r).row != was { unlight(g, None); }
    true
}

/// On a man the orders are his, and the ring starts again for him; anywhere else the click gives the lit wedge (Some(true)). None: the menu has gone.
fn left_click(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> Option<bool> {
    let part = g.groups[g.group].orders.menu.as_ref().unwrap().part.is_some();
    let k = if squad_orders_sams(&crate::seats::here(g)) || part { -1.0 } else { squad_orders_man_at(g, input, cam, canvas) };
    clicked_off(g, input, 1);
    buttons_off(g, input, 1);
    swallow_if_seat(g, input);
    if k < 0.0 { return Some(true); }
    g.groups[g.group].orders.who = match g.groups[g.group].orders.who { Who::Num(w) if w == k => Who::All, _ => Who::Num(k) };
    let menu = g.groups[g.group].orders.menu.as_mut().unwrap();
    if let OrderTarget::None(n) = &menu.target {
        if n.man.is_some() {
            let at = n.at;
            menu.target = OrderTarget::None(OrderTargetNone { at, man: None });
        }
    }
    if !menu_survives_a_read(g) { return None; }
    if let Some(r) = g.groups[g.group].orders.cmd_ring.as_mut() {
        r.more = Some(false);
        r.lit = 1.0;
        r.armed = false;
    }
    squad_orders_ring_sel(g);
    Some(false)
}

/// What the hand asked of the wedges this frame.
#[derive(Clone, Copy)]
struct WedgeLit {
    want: f64,
    /// 'more' turned the ring
    turned: bool,
}

/// A digit, or the cursor gone out of the ring's middle one way: that wedge is lit; 'more' turns the ring to the rest of them and lights nothing. None: the menu has gone.
fn wedge_lit(g: &mut Game, input: InputId, digit: f64, still: bool) -> Option<WedgeLit> {
    let lone = crate::ringlone::lone_key(g, &mut |g: &mut Game| squad_orders_ring_keys(g));
    let time = g.time;
    // ring_want reads only the table of numbers on g: R goes back as it comes out
    let mut r = g.groups[g.group].orders.cmd_ring.unwrap();
    let want0 = crate::cmdring::ring_want(g, &mut r, input, digit, time, Some(lone), None);
    g.groups[g.group].orders.cmd_ring = Some(r);
    let lit_now = g.groups[g.group].orders.cmd_ring.unwrap().lit;
    let want = if still && !js::truthy(digit) && want0 != lit_now { 0.0 } else { want0 };
    if !js::truthy(want) || want == lit_now { return Some(WedgeLit { want, turned: false }); }
    let mut turned = false;
    let r = g.groups[g.group].orders.cmd_ring;
    let seat = seats_now(g, r);
    let i = seat_at(&seat.row, want);
    let first = !ring_more(r);
    if first && (want == 1.0 || (js::truthy(digit) && seat.same.get(want as usize).is_some_and(|&x| x >= 0.0))) {
        g.groups[g.group].orders.cmd_ring.as_mut().unwrap().lit = 1.0;
    } else if want == seat.more {
        unlight(g, Some(true));
        turned = true;
        if !menu_survives_a_read(g) { return None; }
    } else if i >= 0.0 {
        g.groups[g.group].orders.cmd_ring.as_mut().unwrap().lit = want;
    }
    squad_orders_ring_sel(g);
    Some(WedgeLit { want, turned })
}

/// It stays up without its key: a digit's own wedge is given at once, a left click gives the lit one, and so does the key again once the press that opened it has come up;
/// that press coming up gives only a wedge the cursor is out on.
fn lit_wedge_is_given(g: &mut Game, input: InputId, digit: f64, click: bool, lit: WedgeLit) -> bool {
    let key = g.k.cmdring.key.clone();
    let lit_now = g.groups[g.group].orders.cmd_ring.unwrap().lit;
    let again = g.groups[g.group].orders.cmd_ring.unwrap().free.unwrap_or(false) && pressed_delete(g, input, &key);
    let key_down = crate::input::input_down(g, input, &key);
    let latch = crate::cmdring::ring_latch(g.groups[g.group].orders.cmd_ring.as_mut().unwrap(), key_down, again, !js::truthy(digit) && lit.want > 0.0 && lit.want == lit_now);
    let by_digit = digit > 0.0 && !lit.turned && lit_now == digit;
    if latch != crate::cmdring::RingLatch::Give && !by_digit && !(click && !lit.turned) { return false; }
    !(click && !js::truthy(lit_now))
}

/// The lit wedge is given: the first wedge is the quick key's own order, every other one its row's.
fn give_the_lit_wedge(g: &mut Game) {
    let r = g.groups[g.group].orders.cmd_ring;
    let n = g.groups[g.group].orders.menu.clone().unwrap();
    let lit = r.unwrap().lit;
    let i = seat_at(&squad_orders_seats(g, &n, r).row, lit);
    g.groups[g.group].orders.cmd_ring = None;
    if !ring_more(r) && lit == 1.0 {
        g.groups[g.group].orders.menu = None;
        if n.part.is_some() {
            squad_orders_part_give(g, &mut |g: &mut Game| {
                if crate::autopilot::quick(g, &n.target, Some(&n.men)) { None } else { Some(String::from("refused")) }
            });
        } else {
            crate::autopilot::auto_quick(g, &n.target);
        }
    } else if js::truthy(lit) && i >= 0.0 {
        squad_orders_pick(g, Some(i));
        g.groups[g.group].orders.menu = None;
    } else {
        g.groups[g.group].orders.menu = None;
    }
}

/// A digit a ring carries went down this frame.
fn squad_orders_ring_digit(g: &crate::game::Game, input: InputId) -> bool {
    let mut d = 1.0;
    while d <= g.k.cmdring.wedges {
        if crate::input::input_hit(g, input, &format!("Digit{}", d as i32)) { return true; }
        d += 1.0;
    }
    false
}

/// The order menu put up on `t` as a ring at the cursor (false: no menu opened there).
fn squad_orders_ring_up(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, input: InputId, canvas: crate::host_hud::View, t0: f64, cam: Option<CameraId>, g_2: Option<&crate::orders::SquadOrdersRingUpG>) -> bool {
    if !squad_orders_open(g, Some(t), g_2.map(|p| p.men.as_slice())) { return false; }
    let (w, h) = (canvas.client_width, canvas.client_height);
    let r = crate::cmdring::ring_open(g, input, w, h, t0, None);
    g.groups[g.group].orders.cmd_ring = Some(r);
    squad_orders_ring_sel(g);
    // it stays on its point: the place, and where its middle stands from it (from the cursor, not from where the view's edge stood it)
    if let Some(cam) = cam {
        let at = t.at();
        let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, at, w, h, None, None);
        if front {
            let (mx, my) = mouse(g, input);
            if let Some(r) = g.groups[g.group].orders.cmd_ring.as_mut() {
                r.at = Some(at);
                r.ox = Some(mx - sx);
                r.oy = Some(my - sy);
                r.mx = Some(mx);
                r.my = Some(my);
            }
        }
    }
    true
}

/// `squad_orders_goal_part` as ring_up's argument, in that call's own type: the outer None is still null
fn goal_part_or_nobody(g: &mut Game, t: &OrderTarget, add: bool) -> Option<Option<SquadOrdersRingUpG>> {
    squad_orders_goal_part(g, t, add).map(|p| p.map(|p| SquadOrdersRingUpG { men: p.men }))
}

/// Shift with the key: the men a tap of it on `t` would have sent; the ring is theirs. `None` = null (nobody is free: said, no ring) ·
/// `Some(None)` = undefined (the ring it always was) · `Some(Some(_))` = the men the ring is for.
fn squad_orders_goal_part(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, add: bool) -> Option<Option<crate::orders::SquadOrdersGoalPart>> {
    if !add || !crate::planmode::goal_ring_on(g, &crate::seats::here(g)) { return Some(None); }
    let men = match crate::planmode::goal_men(g, t) {
        Some(men) => men,
        None => {
            crate::interact::say_why(g, None, crate::view::ThingKind::Wall, crate::planmode::GOAL_NONE, crate::planmode::GOAL_NONE.trim_end_matches('.'));
            return None;
        }
    };
    use crate::planmode::GoalMenHow as H;
    Some(if men.how == H::Same || men.how == H::Idle || men.how == H::Split {
        Some(SquadOrdersGoalPart { men: men.men })
    } else {
        None
    })
}

/// The ring's key would open a ring now.
fn squad_orders_ring_free(g: &crate::game::Game, seat: &crate::seats::SeatRef, input: InputId) -> bool {
    !crate::planmode::goal_shift(g, seat, input) && squad_orders_back_free(g, seat)
}

/// The ring's key is free: no menu up and nothing the key backs out of instead.
pub(in crate::squad_orders) fn squad_orders_back_free(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    seat.orders.menu.is_none() && g.pl[seat.player].launch_held.is_none() && !crate::interact::hands_busy(g, seat)
}

/// The menu's own highlight follows the lit wedge.
fn squad_orders_ring_sel(g: &mut crate::game::Game) {
    let r = match (&g.groups[g.group].orders.menu, g.groups[g.group].orders.cmd_ring) {
        (Some(_), Some(r)) if js::truthy(r.lit) => r,
        _ => return,
    };
    let i = seat_at(&seats_now(g, Some(r)).row, r.lit);
    if i >= 0.0 { g.groups[g.group].orders.menu.as_mut().unwrap().sel = i; }
}

/// The open menu's rows by wedge, or on 'more' the rest of them from wedge 1. A function of the menu's record: the tick's wedges and a HUD's are the same over the
/// same words (`menu.words`, `menu.quick`: orders/words.rs).
pub fn squad_orders_seats(g: &crate::game::Game, m: &crate::orders::Menu, r: Option<crate::cmdring::RingState>) -> crate::orders::SquadOrdersSeats {
    let rows: Vec<crate::cmdring::FlatRow> = m.rows.iter().map(|&id| flat_row(g, id)).collect();
    let lead = if rows.is_empty() { -1.0 } else { 0.0 };
    let s = crate::cmdring::flat_seats(g, &rows, target_kind(&m.target), lead);
    let (mut s_row, s_more, s_over) = (s.row, s.more, s.over);
    let wedges = g.k.cmdring.wedges as usize;
    let mut same = vec![-1.0; wedges + 1];
    if ring_more(r) {
        let mut row = vec![-1.0; wedges + 1];
        for (n, &i) in s_over.iter().enumerate() { if n + 1 < row.len() { row[n + 1] = i; } else { row.push(i); } }   // (an array in JavaScript: it grows)
        return SquadOrdersSeats { row, more: 0.0, over: s_over, same };
    }
    // a row that says what wedge 1 says has no wedge of its own, and its digit is wedge 1's
    if super::truthy_str(&m.quick) {
        for d in 2..=wedges {
            let i = s_row[d];
            if i >= 0.0 && d as f64 != s_more && m.words.get(i as usize) == m.quick.as_ref() {
                same[d] = i;
                s_row[d] = -1.0;
            }
        }
    }
    SquadOrdersSeats { row: s_row, more: s_more, over: s_over, same }
}
fn flat_row(g: &Game, id: OrdersOrderDefId) -> crate::cmdring::FlatRow {
    let d = &g.tab.orders_order_def[id];
    crate::cmdring::FlatRow {
        id: d.id.clone(),
        word: d.word,
        free: d.free,
        targets: d.targets.iter().map(|k| String::from(k.as_str())).collect(),
        listed: d.listed.as_ref().map(|l| l.iter().map(|k| String::from(k.as_str())).collect()),
    }
}

/// A tap's order on `t`: nothing where a tutorial step teaches the key held; a second tap in the same place is 'fall in'.
pub(super) fn squad_orders_tap_give(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, input: InputId, goal: Option<bool>) {
    // (a tap ON HIMSELF is never stopped: the key on himself always calls them back, and it is the way out when a step that teaches the hold finds nobody free)
    let on_himself = matches!(t, OrderTarget::None(OrderTargetNone { man: None, .. }));
    if !on_himself && crate::cmdring::ring_hold_secs(g) > 0.0 && crate::rails::rails_tap_off(g) { return; }
    if goal.unwrap_or(false) {
        g.groups[g.group].orders.tap_was = None;
        crate::planmode::plan_goal(g, t);
        return;
    }
    let was = g.groups[g.group].orders.tap_was.take();
    let now = g.time;
    let (x, y) = mouse(g, input);
    let same = match &was {
        None => false,
        Some(w) => m::hyp2(x - w.x, y - w.y) <= g.k.cmdring.twice_px || match (&w.t, t) {
            (OrderTarget::Door(a), OrderTarget::Door(b)) => a.door == b.door,
            _ => false,
        },
    };
    if squad_orders_twice_on(g) {
        if let (Some(w), true) = (&was, same) {
            if now >= w.at && now - w.at <= g.k.cmdring.twice_secs + 1e-6 {
                squad_orders_all_in(g);
                return;
            }
        }
    }
    g.groups[g.group].orders.tap_was = Some(SquadOrdersTapWas { at: now, t: t.clone(), x, y });
    crate::autopilot::auto_quick(g, t);
}

/// CMDRING.twice is asked here: the ring's key, and on a tutorial mission only from a step that lists it.
pub fn squad_orders_twice_on(g: &crate::game::Game) -> bool {
    g.k.cmdring.twice && !crate::rails::rails_locks(g, "twice")
}

/// Every teammate falls in; not said twice over a 'fall in' the first tap gave them all.
fn squad_orders_all_in(g: &mut crate::game::Game) {
    let men = squad_orders_men(g, &crate::seats::here(g), Some(SquadOrdersMenSel::All), None);
    let t = at_sam(g, &crate::seats::here(g));
    let mut every = true;
    for &gd in &men {
        let fell_in = match squad_orders_of(&crate::seats::here(g), gd) {
            Some(a) => { let a = &g.tab.task[a]; crate::orders::rows::id_is(&a.order, crate::orders::Is::Follows) && a.post.is_none() }
            None => false,
        };
        if !fell_in { every = false; break; }
    }
    if every { return; }
    squad_orders_give(g, crate::orders::rows::fallin::ORDER.id, Some(&t), None, Some(&opts_men(men, None)));
}

/// The quick key's words for the HUD and the ring's first wedge.
fn squad_orders_quick_words(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, p: Option<&crate::autopilot::AutoPick>) -> Option<String> {
    crate::autopilot::auto_hint(g, t, p)
}

/// … for the open menu's first wedge; for a ring of a part of the squad, what those men would do.
pub(super) fn squad_orders_menu_quick(g: &mut crate::game::Game, m: &crate::orders::Menu) -> Option<String> {
    if m.part.is_some() {
        let p = crate::autopilot::auto_pick(g, &crate::seats::here(g), &m.target, Some(false), Some(&m.men));
        crate::autopilot::auto_hint(g, &m.target, Some(&p))
    } else {
        squad_orders_quick_words(g, &m.target, None)
    }
}

/// The man of the squad the cursor is on, or whose feet are nearest it on the screen (his place on the roster; -1: nobody).
fn squad_orders_man_at(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> f64 {
    let squad = squad_orders_squad(g, &crate::seats::here(g));
    let t = squad_orders_target_under(g, input, cam, canvas);
    if let OrderTarget::None(OrderTargetNone { man: Some(man), .. }) = t {
        if let Some(k) = squad.iter().position(|&gd| gd == man) {
            if !crate::condition::down(g, g.gd[man].ch) { return k as f64; }
        }
    }
    let (w, h) = (canvas.client_width, canvas.client_height);
    let mut best = -1.0;
    let mut bd = g.k.cmdring.man_px * crate::cmdring::ring_zoom(g, w, h);
    let (mx, my) = mouse(g, input);
    for (k, &gd) in squad.iter().enumerate() {
        if crate::condition::down(g, g.gd[gd].ch) { continue; }
        let pos = g.pos_of(gd);
        let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, pos, w, h, None, None);
        if !front { continue; }
        let d = m::hyp2(sx - mx, sy - my);
        if d < bd { bd = d; best = k as f64; }
    }
    best
}

/// The digits the ring has a wedge on, a bit each (0: no ring). All the game reads of a ring's picture.
fn squad_orders_ring_keys(g: &crate::game::Game) -> f64 {
    let (r, menu) = match (g.groups[g.group].orders.cmd_ring, g.groups[g.group].orders.menu.clone()) {
        (Some(r), Some(menu)) => (r, menu),
        _ => return 0.0,
    };
    let seat = squad_orders_seats(g, &menu, Some(r));
    let mut keys: i32 = 0;
    let wedges = g.k.cmdring.wedges as i32;
    for d in 1..=wedges {
        let at = d as usize;
        let on = d as f64 == seat.more || if !ring_more(Some(r)) && d == 1 {
            super::truthy_str(&menu.quick)
        } else {
            seat.row[at] >= 0.0 && (seat.row[at] as usize) < menu.rows.len()
        };
        if on { keys |= 1 << d; }
    }
    keys as f64
}

/// The roster as it stands; a new cast starts clean.
pub(super) fn squad_orders_sync(g: &mut crate::game::Game) -> Vec<GuardId> {
    let mut i = 0;
    let mut same = true;
    let mut at = 0;
    while at < g.guards.len() {
        let gd = g.guards[at];
        at += 1;
        if squad_orders_mine(g, &crate::seats::here(g), gd) {
            if g.groups[g.group].orders.roster.get(i) != Some(&gd) { same = false; }
            i += 1;
        }
    }
    if !same || i != g.groups[g.group].orders.roster.len() {
        let squad = squad_orders_squad(g, &crate::seats::here(g));
        g.groups[g.group].orders.roster = squad.clone();
        g.groups[g.group].orders.who = Who::All;
        g.groups[g.group].orders.menu = None;
        g.groups[g.group].orders.trail.clear();
        for gd in g.groups[g.group].orders.asg.keys() {
            if !squad.contains(&gd) { squad_orders_drop(g, gd); }
        }
    }
    let stands = match g.groups[g.group].orders.who {
        Who::Num(k) => g.groups[g.group].orders.roster.get(k as usize).map_or(true, |&gd| !crate::condition::down(g, g.gd[gd].ch)),
        Who::All => true,
        Who::Red => !squad_orders_men(g, &crate::seats::here(g), Some(SquadOrdersMenSel::Red), None).is_empty(),
        Who::Blue => !squad_orders_men(g, &crate::seats::here(g), Some(SquadOrdersMenSel::Blue), None).is_empty(),
    };
    if !stands {
        // the man the orders went to is down: a menu that was theirs goes with them, never on to everybody else
        g.groups[g.group].orders.who = Who::All;
        g.groups[g.group].orders.menu = None;
    }
    g.groups[g.group].orders.roster.clone()
}

/// Sam is standing.
pub fn squad_orders_sam_stopped(seat: &crate::seats::SeatRef) -> bool {
    seat.orders.sam_v <= 0.3
}

/// A man's order goes, and with it the pin the fight seam put on him.
pub fn squad_orders_drop(g: &mut crate::game::Game, gd: GuardId) {
    let his = g.groups[g.group].orders.asg.get(gd).copied();
    g.groups[g.group].orders.asg.delete(gd);
    crate::duty::done(g, gd, his);
    if g.groups[g.group].orders.ours.delete(gd) { crate::duty::pin(g, gd, false, "orders"); }
}
