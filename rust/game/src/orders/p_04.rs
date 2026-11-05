//! src/game/orders.ts lines 1098 to 1354
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- the frame's input: what the order input takes out of it

fn pressed_has(g: &Game, input: InputId, code: &str) -> bool {
    g.tab.input[input].pressed.has(String::from(code))
}
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
fn shift_down(g: &mut Game, input: InputId) -> bool {
    crate::input::input_down(g, input, "ShiftLeft") || crate::input::input_down(g, input, "ShiftRight")
}
/// backTake(input): `Keys` holds `pressed` by value, so it is lent and put back
fn back_take_input(g: &mut Game, input: InputId) -> bool {
    let mut keys = crate::backkey::Keys { pressed: core::mem::take(&mut g.tab.input[input].pressed) };
    let esc = crate::backkey::back_take(g, &mut keys);
    g.tab.input[input].pressed = keys.pressed;
    esc
}
/// `if (input === g.seat.input) g.seat.swallow(1)`
fn swallow_if_seat(g: &mut Game, input: InputId) {
    let s = g.seat;
    if g.tab.cmd_seat[s].input == input { crate::cmd::seat_swallow(g, s, 1.0); }
}

// ---------------------------------------------------------------- small readings

/// a string as a condition: `!!s` of `string | null | undefined`
fn text(s: &Option<String>) -> bool {
    s.as_ref().is_some_and(|x| !x.is_empty())
}
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
fn target_at(t: &OrderTarget) -> V3 {
    match t {
        OrderTarget::Floor(x) => x.at,
        OrderTarget::Door(x) => x.at,
        OrderTarget::None(x) => x.at,
        OrderTarget::Foe(x) => x.at,
    }
}
fn from_menu_target(t: SquadOrdersMenuTarget) -> OrderTarget {
    match t {
        SquadOrdersMenuTarget::Floor(x) => OrderTarget::Floor(x),
        SquadOrdersMenuTarget::Door(x) => OrderTarget::Door(x),
        SquadOrdersMenuTarget::None(x) => OrderTarget::None(x),
        SquadOrdersMenuTarget::Foe(x) => OrderTarget::Foe(x),
    }
}
fn hover_target(t: &OrderTarget) -> SquadOrdersHoverWordsT {
    match t {
        OrderTarget::Floor(x) => SquadOrdersHoverWordsT::Floor(x.clone()),
        OrderTarget::Door(x) => SquadOrdersHoverWordsT::Door(x.clone()),
        OrderTarget::None(x) => SquadOrdersHoverWordsT::None(x.clone()),
        OrderTarget::Foe(x) => SquadOrdersHoverWordsT::Foe(x.clone()),
    }
}
/// `{ kind: 'none', at: [p[0], 0, p[2]] }`, p where Sam stands
fn at_sam(g: &Game) -> OrderTarget {
    let p = g.ch[g.pl[g.player].ch].pos;
    OrderTarget::None(OrderTargetNone { at: [p[0], 0.0, p[2]], man: None })
}
fn opts_men(men: Vec<GuardId>, ack: Option<String>) -> GiveOpts {
    GiveOpts { ack, on_word: None, men: Some(men) }
}
/// `R.lit = 0; R.armed = false`, and `R.more` as given
fn unlight(g: &mut Game, more: Option<bool>) {
    if let Some(r) = g.orders.cmd_ring.as_mut() {
        if more.is_some() { r.more = more; }
        r.lit = 0.0;
        r.armed = false;
    }
}
fn ring_more(r: Option<crate::cmdring::RingState>) -> bool {
    r.is_some_and(|r| r.more.unwrap_or(false))
}
/// `this.seats(this.menu!, R)`: the menu is lent as a copy, since `seats` takes `g` too
fn seats_now(g: &mut Game, r: Option<crate::cmdring::RingState>) -> SquadOrdersSeats {
    let m = g.orders.menu.clone().unwrap();
    squad_orders_seats(g, &m, r)
}
fn away(g: &mut Game) {
    g.orders.menu = None;
    g.orders.cmd_ring = None;
}

/// THE RING'S INPUT, ahead of the menu's own: the frame's order input is the ring's if true. (orders.ts ringInput)
pub fn squad_orders_ring_input(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> bool {
    if g.orders.cmd_ring.is_some() && g.orders.menu.is_none() { g.orders.cmd_ring = None; }
    let key = g.k.cmdring.key.clone();
    if g.orders.cmd_ring.is_none() {
        // the key down and no ring yet: let go, it was a tap; still down when the hold is over, the ring goes up
        if let Some(p) = g.orders.cmd_press.clone() {
            if pressed_has(g, input, "Blur") || pressed_has(g, input, "Paused") { g.orders.cmd_press = None; return false; }
            if !crate::input::input_down(g, input, &key) {
                g.orders.cmd_press = None;
                g.stats.cmdring.quick += 1;
                let goal = (p.add.unwrap_or(false) || shift_down(g, input)) && crate::planmode::goals_on(g);
                squad_orders_tap_give(g, &p.t, input, Some(goal));
                return true;
            }
            let digit = squad_orders_ring_digit(g, input);
            if !digit && g.time - p.t0 < crate::cmdring::ring_hold_secs(g) - 1e-6 { return false; }
            g.orders.cmd_press = None;
            if !squad_orders_back_free(g) { return false; }
            let add = p.add.unwrap_or(false) || shift_down(g, input);
            let part = match goal_part_or_nobody(g, &p.t, add) { Some(part) => part, None => return true };
            let t0 = g.time - g.k.cmdring.tap_secs;
            if !squad_orders_ring_up(g, &p.t, input, canvas, t0, Some(cam), part.as_ref()) { return false; }
            return if digit { squad_orders_ring_input(g, input, cam, canvas) } else { true };
        }
        if !crate::input::input_hit(g, input, &key) || !squad_orders_ring_free(g, input) { return false; }
        pressed_delete(g, input, &key);
        let add = crate::planmode::goals_on(g) && shift_down(g, input);
        if g.k.cmdring.hold_secs > 0.0 && !crate::input::input_down(g, input, &key) {
            g.stats.cmdring.quick += 1;
            let t = squad_orders_target_under(g, input, cam, canvas);
            squad_orders_tap_give(g, &t, input, Some(add));
            return true;
        }
        if g.k.cmdring.hold_secs > 0.0 {
            let t0 = g.time;
            let t = squad_orders_target_under(g, input, cam, canvas);
            g.orders.cmd_press = Some(SquadOrdersCmdPress { t0, t, add: if add { Some(true) } else { None } });
            if squad_orders_ring_digit(g, input) { squad_orders_ring_input(g, input, cam, canvas); }
            return true;
        }
        let t = squad_orders_target_under(g, input, cam, canvas);
        return match goal_part_or_nobody(g, &t, add) {
            None => true,
            Some(part) => { let t0 = g.time; squad_orders_ring_up(g, &t, input, canvas, t0, Some(cam), part.as_ref()) }
        };
    }
    // it stays on its point: its middle put back over the place it was opened on; a cursor the hand has not moved lights no other wedge
    let (w, h) = (canvas.client_width, canvas.client_height);
    let mut still = false;
    if let Some(at) = g.orders.cmd_ring.unwrap().at {
        let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, at, w, h, None, None);
        if front {
            // ringPin reads only the table of numbers on g: R goes back as it comes out
            let mut r = g.orders.cmd_ring.unwrap();
            crate::cmdring::ring_pin(g, &mut r, sx, sy, w, h);
            g.orders.cmd_ring = Some(r);
        }
        let (mx, my) = mouse(g, input);
        if let Some(r) = g.orders.cmd_ring.as_mut() {
            still = r.mx == Some(mx) && r.my == Some(my);
            r.mx = Some(mx);
            r.my = Some(my);
        }
    }
    pressed_delete(g, input, crate::consts::INTERACT_KEY);
    let esc = if key != "Tab" { back_take_input(g, input) } else { pressed_delete(g, input, "Escape") };
    let blur = pressed_has(g, input, "Blur") || pressed_has(g, input, "Paused");
    let rmb = crate::input::input_rmb_hit(g, input);
    if rmb { clicked_off(g, input, 4); }
    let lmb = crate::input::input_lmb_hit(g, input);
    let mut click = false;
    let mut digit = 0.0;
    for d in 1..=9 {
        if pressed_delete(g, input, &format!("Digit{}", d)) && !js::truthy(digit) { digit = d as f64; }
    }
    // the squad's only keys: a row's own letter is nobody's while the ring is up; the word's key stays the word's
    for id in g.orders.defs.clone() {
        if let Some(hot) = g.tab.orders_order_def[id].hot_key.clone() {
            if !hot.is_empty() && hot != g.k.orders.wait_alt { pressed_delete(g, input, &hot); }
        }
    }
    if js::truthy(g.tab.input[input].wheel) {
        let turned = g.tab.input[input].wheel;
        g.tab.input[input].wheel = 0.0;
        let menu = g.orders.menu.as_mut().unwrap();
        menu.wheel_acc = m::max(-2.0 * WHEEL_STEP, m::min(2.0 * WHEEL_STEP, menu.wheel_acc + turned));
        if m::abs(menu.wheel_acc) >= WHEEL_STEP {
            let wheel = m::sign(menu.wheel_acc);
            menu.wheel_acc -= wheel * WHEEL_STEP;
        }
    }
    let stale = {
        let read_t = g.orders.menu.as_ref().unwrap().read_t;
        g.time - read_t >= g.k.orders.refresh_secs || {
            let menu = g.orders.menu.clone().unwrap();
            squad_orders_due(g, &menu)
        }
    };
    if stale {
        let r = g.orders.cmd_ring;
        let was = seats_now(g, r).row;
        squad_orders_refresh(g);
        if g.orders.menu.is_none() { g.orders.cmd_ring = None; return true; }
        let r = g.orders.cmd_ring;
        // a row came or went under the ring: the wedges are other rows now, none is lit until the hand lights one
        if seats_now(g, r).row != was { unlight(g, None); }
    }
    if blur || esc { away(g); g.stats.cmdring.cancelled += 1; return true; }
    if ring_more(g.orders.cmd_ring) && seats_now(g, None).over.is_empty() { unlight(g, Some(false)); }
    if rmb {
        if ring_more(g.orders.cmd_ring) { unlight(g, Some(false)); }
        else if !squad_orders_page_back(g) { away(g); g.stats.cmdring.cancelled += 1; }
        return true;
    }
    if lmb {
        // on a man: the orders are his, and the ring starts again for him · anywhere else the click gives the lit wedge, below
        let part = g.orders.menu.as_ref().unwrap().part.is_some();
        let k = if squad_orders_whole(g) || part { -1.0 } else { squad_orders_man_at(g, input, cam, canvas) };
        clicked_off(g, input, 1);
        buttons_off(g, input, 1);
        swallow_if_seat(g, input);
        if k < 0.0 {
            click = true;
        } else {
            g.orders.who = match g.orders.who { Who::Num(w) if w == k => Who::All, _ => Who::Num(k) };
            g.stats.cmdring.who += 1;
            let menu = g.orders.menu.as_mut().unwrap();
            if let OrderTarget::None(n) = &menu.target {
                if n.man.is_some() {
                    let at = n.at;
                    menu.target = OrderTarget::None(OrderTargetNone { at, man: None });
                }
            }
            if menu.page.is_some() { squad_orders_turn(g, None, None); } else { squad_orders_refresh(g); }
            if g.orders.menu.is_none() { g.orders.cmd_ring = None; return true; }
            if let Some(r) = g.orders.cmd_ring.as_mut() {
                r.more = Some(false);
                r.lit = 1.0;
                r.armed = false;
            }
            squad_orders_ring_sel(g);
        }
    }
    // a digit, or the cursor gone out of the ring's middle one way: that wedge is lit; a wedge that opens a page turns the ring to it and lights nothing
    let lone = crate::ringlone::lone_key(g, &mut |g: &mut Game| squad_orders_ring_keys(g));
    let time = g.time;
    // ringWant reads only the table of numbers on g: R goes back as it comes out
    let mut r = g.orders.cmd_ring.unwrap();
    let want0 = crate::cmdring::ring_want(g, &mut r, input, digit, time, Some(lone), None);
    g.orders.cmd_ring = Some(r);
    let lit_now = g.orders.cmd_ring.unwrap().lit;
    let want = if still && !js::truthy(digit) && want0 != lit_now { 0.0 } else { want0 };
    let mut turned = false;
    if js::truthy(want) && want != lit_now {
        let r = g.orders.cmd_ring;
        let (rows, on_first) = {
            let menu = g.orders.menu.as_ref().unwrap();
            (menu.rows.clone(), menu.page.is_none())
        };
        let seat = seats_now(g, r);
        let i = seat_at(&seat.row, want);
        let first = on_first && !ring_more(r);
        let opens = if i >= 0.0 { g.tab.orders_order_def[rows[i as usize]].opens.clone() } else { None };
        if first && (want == 1.0 || (js::truthy(digit) && seat.same.get(want as usize).is_some_and(|&x| x >= 0.0))) {
            g.orders.cmd_ring.as_mut().unwrap().lit = 1.0;
        } else if want == seat.more {
            unlight(g, Some(true));
            g.stats.cmdring.pages += 1;
            turned = true;
            if g.orders.menu.as_ref().unwrap().flat.unwrap_or(false) {
                squad_orders_refresh(g);
                if g.orders.menu.is_none() { g.orders.cmd_ring = None; return true; }
            }
        } else if text(&opens) {
            squad_orders_turn(g, opens.as_deref(), None);
            g.stats.cmdring.pages += 1;
            turned = true;
            if g.orders.menu.is_none() { g.orders.cmd_ring = None; return true; }
        } else if i >= 0.0 {
            g.orders.cmd_ring.as_mut().unwrap().lit = want;
        }
        squad_orders_ring_sel(g);
    }
    // it stays up without its key: a digit's own wedge is given at once, a left click gives the lit one, and so does the key again once the press that
    // opened it has come up; that press coming up gives only a wedge the cursor is out on
    let lit_now = g.orders.cmd_ring.unwrap().lit;
    let again = g.orders.cmd_ring.unwrap().free.unwrap_or(false) && pressed_delete(g, input, &key);
    let key_down = crate::input::input_down(g, input, &key);
    let latch = crate::cmdring::ring_latch(g.orders.cmd_ring.as_mut().unwrap(), key_down, again, !js::truthy(digit) && want > 0.0 && want == lit_now);
    let by_digit = digit > 0.0 && !turned && lit_now == digit;
    if latch != crate::cmdring::RingLatch::Give && !by_digit && !(click && !turned) { return true; }
    if click && !js::truthy(lit_now) { return true; }
    // the lit wedge is given: the first page's first wedge is the quick key's own order, every other one its row's
    let r = g.orders.cmd_ring;
    let n = g.orders.menu.clone().unwrap();
    let lit = r.unwrap().lit;
    let i = seat_at(&squad_orders_seats(g, &n, r).row, lit);
    g.orders.cmd_ring = None;
    if n.page.is_none() && !ring_more(r) && lit == 1.0 {
        g.orders.menu = None;
        g.stats.cmdring.quick += 1;
        if n.part.is_some() {
            squad_orders_part_give(g, &n, &mut |g: &mut Game| {
                if squad_orders_quick_for(g, &n.men, &n.target).is_some() { None } else { Some(String::from("refused")) }
            });
        } else {
            squad_orders_quick_give(g, &n.target);
        }
    } else if js::truthy(lit) && i >= 0.0 {
        squad_orders_pick(g, Some(i));
        g.orders.menu = None;
        g.stats.cmdring.given += 1;
    } else {
        g.orders.menu = None;
        g.stats.cmdring.cancelled += 1;
    }
    true
}

/// A digit a ring carries went down this frame. (orders.ts ringDigit)
pub fn squad_orders_ring_digit(g: &mut crate::game::Game, input: InputId) -> bool {
    let mut d = 1.0;
    while d <= g.k.cmdring.wedges {
        if pressed_has(g, input, &format!("Digit{}", d as i32)) { return true; }
        d += 1.0;
    }
    false
}

/// The order menu put up on `t` as a ring at the cursor (false: no menu opened there). (orders.ts ringUp)
pub fn squad_orders_ring_up(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, input: InputId, canvas: crate::host_hud::View, t0: f64, cam: Option<CameraId>, g_2: Option<&crate::orders::SquadOrdersRingUpG>) -> bool {
    let flat = squad_orders_flat_ring(g);
    if !squad_orders_open(g, Some(t), Some(flat), g_2.map(|p| p.men.as_slice())) { return false; }
    if let Some(p) = g_2 {
        g.orders.menu.as_mut().unwrap().split = Some(p.split);
        g.stats.planmode.rings += 1;
    }
    g.orders.menu.as_mut().unwrap().on_word = false;
    let (w, h) = (canvas.client_width, canvas.client_height);
    let r = crate::cmdring::ring_open(g, input, w, h, t0, None);
    g.orders.cmd_ring = Some(r);
    squad_orders_ring_sel(g);
    g.stats.cmdring.opened += 1;
    // it stays on its point: the place, and where its middle stands from it (from the cursor, not from where the view's edge stood it)
    if let Some(cam) = cam {
        let at = target_at(t);
        let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, at, w, h, None, None);
        if front {
            let (mx, my) = mouse(g, input);
            if let Some(r) = g.orders.cmd_ring.as_mut() {
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

/// `goalPart` with its three answers: `None` = null (nobody is free: said, no ring) · `Some(None)` = undefined (the ring it always was) ·
/// `Some(Some(_))` = the men the ring is for. The skeleton's signature below has lost null from undefined.
fn goal_part_three(g: &mut Game, t: &OrderTarget, add: bool) -> Option<Option<SquadOrdersGoalPart>> {
    if !add || !crate::planmode::goal_ring_on(g) { return Some(None); }
    let men = match crate::planmode::goal_men(g, t) {
        Some(men) => men,
        None => {
            crate::game::game_msg(g, crate::planmode::GOAL_NONE);
            g.stats.planmode.none += 1;
            return None;
        }
    };
    use crate::planmode::GoalMenHow as H;
    Some(if men.how == H::Same || men.how == H::Idle || men.how == H::Split {
        Some(SquadOrdersGoalPart { split: men.how == H::Split, men: men.men })
    } else {
        None
    })
}
/// goal_part_three for ring_up's argument: the outer None is still null
fn goal_part_or_nobody(g: &mut Game, t: &OrderTarget, add: bool) -> Option<Option<SquadOrdersRingUpG>> {
    goal_part_three(g, t, add).map(|p| p.map(|p| SquadOrdersRingUpG { men: p.men, split: p.split }))
}

/// Shift with the key: the men a tap of it on `t` would have sent; the ring is theirs. (orders.ts goalPart)
pub fn squad_orders_goal_part(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, add: bool) -> Option<crate::orders::SquadOrdersGoalPart> {
    goal_part_three(g, t, add).flatten()
}

/// The ring's key would open a ring now. (orders.ts ringFree)
pub fn squad_orders_ring_free(g: &mut crate::game::Game, input: InputId) -> bool {
    !crate::planmode::goal_shift(g, input) && squad_orders_back_free(g)
}

/// The ring's key is free: no menu up and nothing the key backs out of instead. (orders.ts backFree)
pub fn squad_orders_back_free(g: &mut crate::game::Game) -> bool {
    g.orders.menu.is_none() && g.pl[g.player].launch_held.is_none() && !crate::interact::hands_busy(g)
}

/// What the key would open on, for a hint: CMDRING.bareHint draws none. (orders.ts hintView)
pub fn squad_orders_hint_view(g: &mut crate::game::Game, kept: Option<&mut crate::orders::HoverKept>) -> Option<crate::orders::SquadOrdersHintView> {
    None
}

/// The menu's own highlight follows the lit wedge. (orders.ts ringSel)
pub fn squad_orders_ring_sel(g: &mut crate::game::Game) {
    let r = match (&g.orders.menu, g.orders.cmd_ring) {
        (Some(_), Some(r)) if js::truthy(r.lit) => r,
        _ => return,
    };
    let i = seat_at(&seats_now(g, Some(r)).row, r.lit);
    if i >= 0.0 { g.orders.menu.as_mut().unwrap().sel = i; }
}

/// The open menu's rows by wedge: its page's own, or on 'more' the rest of them from wedge 1. (orders.ts seats)
pub fn squad_orders_seats(g: &mut crate::game::Game, m: &crate::orders::Menu, r: Option<crate::cmdring::RingState>) -> crate::orders::SquadOrdersSeats {
    let word_key = g.k.orders.wait_alt.clone();
    let flat = m.flat.unwrap_or(false);
    let (mut s_row, s_more, s_over) = if flat {
        let rows: Vec<crate::cmdring::FlatRow> = m.rows.iter().map(|&id| flat_row(g, id)).collect();
        let lead = if rows.is_empty() { -1.0 } else { 0.0 };
        let s = crate::cmdring::flat_seats(g, &rows, target_kind(&m.target), &word_key, lead);
        (s.row, s.more, s.over)
    } else {
        let rows: Vec<crate::cmdring::SeatRow> = m.rows.iter().map(|&id| seat_row(g, id)).collect();
        let s = crate::cmdring::ring_seats(g, &rows, target_kind(&m.target) != "none", &word_key, Some(m.page.is_none()));
        (s.row, s.more, s.over)
    };
    let wedges = g.k.cmdring.wedges as usize;
    let mut same = vec![-1.0; wedges + 1];
    if ring_more(r) {
        let mut row = vec![-1.0; wedges + 1];
        for (n, &i) in s_over.iter().enumerate() { if n + 1 < row.len() { row[n + 1] = i; } else { row.push(i); } }   // (an array in JavaScript: it grows)
        return SquadOrdersSeats { row, more: 0.0, over: s_over, same };
    }
    // the flat tree's first ring: a row that says what wedge 1 says has no wedge of its own, and its digit is wedge 1's
    let q = if flat { squad_orders_menu_quick(g, m) } else { None };
    let mt = if text(&q) { squad_orders_menu_target(g).map(from_menu_target) } else { None };
    if let (true, Some(mt)) = (text(&q), mt) {
        for d in 2..=wedges {
            let i = s_row[d];
            if i >= 0.0 && d as f64 != s_more && Some(squad_orders_label_of(g, m.rows[i as usize], &mt)) == q {
                same[d] = i;
                s_row[d] = -1.0;
            }
        }
    }
    SquadOrdersSeats { row: s_row, more: s_more, over: s_over, same }
}
fn seat_row(g: &Game, id: OrdersOrderDefId) -> crate::cmdring::SeatRow {
    let d = &g.tab.orders_order_def[id];
    crate::cmdring::SeatRow {
        key: d.key,
        hot_key: d.hot_key.clone(),
        opens: d.opens.clone(),
        free: d.free,
        targets: d.targets.iter().map(|k| String::from(k.as_str())).collect(),
        listed: d.listed.as_ref().map(|l| l.iter().map(|k| String::from(k.as_str())).collect()),
    }
}
fn flat_row(g: &Game, id: OrdersOrderDefId) -> crate::cmdring::FlatRow {
    let s = seat_row(g, id);
    crate::cmdring::FlatRow { id: g.tab.orders_order_def[id].id.clone(), key: s.key, hot_key: s.hot_key, opens: s.opens, free: s.free, targets: s.targets, listed: s.listed }
}

/// A tap's order on `t`: nothing where a tutorial step teaches the key held; a second tap in the same place is 'fall in'. (orders.ts tapGive)
pub fn squad_orders_tap_give(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, input: InputId, goal: Option<bool>) {
    if crate::cmdring::ring_hold_secs(g) > 0.0 && crate::rails::rails_tap_off(g) { return; }
    if goal.unwrap_or(false) {
        g.orders.tap_was = None;
        crate::planmode::plan_goal(g, t);
        return;
    }
    let was = g.orders.tap_was.take();
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
    g.orders.tap_was = Some(SquadOrdersTapWas { at: now, t: t.clone(), x, y });
    squad_orders_quick_give(g, t);
}

/// CMDRING.twice is asked here: the ring's key, and on a tutorial mission only from a step that lists it. (orders.ts twiceOn)
pub fn squad_orders_twice_on(g: &mut crate::game::Game) -> bool {
    g.k.cmdring.twice && !crate::rails::rails_locks(g, "twice")
}

/// Every teammate falls in; not said twice over a 'fall in' the first tap gave them all. (orders.ts allIn)
pub fn squad_orders_all_in(g: &mut crate::game::Game) {
    let men = squad_orders_men(g, Some(SquadOrdersMenSel::All), None);
    let t = at_sam(g);
    g.stats.cmdring.twice += 1;
    let mut every = true;
    for &gd in &men {
        let fell_in = match squad_orders_of(g, gd) {
            Some(a) => { let a = &g.tab.orders_assignment[a]; a.order == "fallIn" && a.post.is_none() }
            None => false,
        };
        if !fell_in { every = false; break; }
    }
    if every { return; }
    g.orders.by_hand = true;
    squad_orders_give(g, "fallIn", Some(&t), None, Some(&opts_men(men, None)));
    g.orders.by_hand = false;
}

/// The quick key's own order on `t`, given by hand: the entry the men pick, else the target's first row. (orders.ts quickGive)
pub fn squad_orders_quick_give(g: &mut crate::game::Game, t: &crate::orders::OrderTarget) {
    g.orders.by_hand = true;
    if !crate::autopilot::auto_quick(g, t) {
        if let Some(d) = squad_orders_default_for(g, t) {
            let id = g.tab.orders_order_def[d].id.clone();
            squad_orders_give(g, &id, Some(t), None, None);
        }
    }
    g.orders.by_hand = false;
}

/// The quick key's order on `t` for exactly these men. (orders.ts quickFor)
pub fn squad_orders_quick_for(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<crate::orders::SquadOrdersQuickFor> {
    use crate::autopilot::AutoPickWhy as Why;
    let n = g.orders.group;
    let tail = g.orders.log.last().map(|row| row.serial);
    let a = crate::autopilot::auto_pick(g, t, Some(true), Some(men));
    if a.why == Why::Pending { return None; }
    let to: Vec<GuardId> = a.men.clone().unwrap_or_else(|| men.to_vec());
    if a.why == Why::Set {
        let lines = crate::autopilot::restack_lines();
        squad_orders_again(g, "stack", t, &to, &lines);
        g.stats.autopilot.again += 1;
        return None;
    }
    let here = at_sam(g);
    g.orders.by_hand = true;
    if a.word {
        squad_orders_give(g, "go", Some(&here), None, Some(&opts_men(to, None)));
    } else if text(&a.id) {
        let ack = crate::squadnet::why_words(g, &a).map(|w| w.ack);
        squad_orders_give(g, a.id.as_deref().unwrap(), Some(t), None, Some(&opts_men(to, ack)));
    }
    g.orders.by_hand = false;
    if let Some(row) = g.orders.log.last_mut() {
        if Some(row.serial) != tail { row.auto = Some(true); }
    }
    if g.orders.group == n { None }
    else if a.why == Why::First { Some(SquadOrdersQuickFor::First) }
    else { Some(SquadOrdersQuickFor::Entry) }
}

/// Those men cover toward `at` from where they stand (true: somebody took it). (orders.ts coverFor)
pub fn squad_orders_cover_for(g: &mut crate::game::Game, men: &[GuardId], at: V3) -> bool {
    let n = g.orders.group;
    g.orders.by_hand = true;
    let t = OrderTarget::Floor(OrderTargetFloor { at, wall: None, thing: None, serial: 0 });
    squad_orders_give(g, "cover", Some(&t), None, Some(&opts_men(men.to_vec(), None)));
    g.orders.by_hand = false;
    g.orders.group != n
}

/// The quick key's words for the HUD and the ring's first wedge. (orders.ts quickWords)
pub fn squad_orders_quick_words(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, p: Option<&crate::autopilot::AutoPick>) -> Option<String> {
    crate::autopilot::auto_hint(g, t, p)
}

/// … for the open menu's first wedge; for a ring of a part of the squad, what those men would do. (orders.ts menuQuick)
pub fn squad_orders_menu_quick(g: &mut crate::game::Game, m: &crate::orders::Menu) -> Option<String> {
    if m.part.is_some() {
        let p = crate::autopilot::auto_pick(g, &m.target, Some(false), Some(&m.men));
        crate::autopilot::auto_hint(g, &m.target, Some(&p))
    } else {
        squad_orders_quick_words(g, &m.target, None)
    }
}

/// The man of the squad the cursor is on, or whose feet are nearest it on the screen (his place on the roster; -1: nobody). (orders.ts manAt)
pub fn squad_orders_man_at(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> f64 {
    let squad = squad_orders_squad(g);
    let t = squad_orders_target_under(g, input, cam, canvas);
    if let OrderTarget::None(OrderTargetNone { man: Some(man), .. }) = t {
        if let Some(k) = squad.iter().position(|&gd| gd == man) {
            if g.gd[man].state != crate::game::GuardState::Dead { return k as f64; }
        }
    }
    let (w, h) = (canvas.client_width, canvas.client_height);
    let mut best = -1.0;
    let mut bd = g.k.cmdring.man_px * crate::cmdring::ring_zoom(g, w, h);
    let (mx, my) = mouse(g, input);
    for (k, &gd) in squad.iter().enumerate() {
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let pos = g.ch[g.gd[gd].ch].pos;
        let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, pos, w, h, None, None);
        if !front { continue; }
        let d = m::hyp2(sx - mx, sy - my);
        if d < bd { bd = d; best = k as f64; }
    }
    best
}

/// The digits the ring has a wedge on, a bit each (0: no ring). All the game reads of a ring's picture. (orders.ts ringKeys)
pub fn squad_orders_ring_keys(g: &mut crate::game::Game) -> f64 {
    let (r, menu) = match (g.orders.cmd_ring, g.orders.menu.clone()) {
        (Some(r), Some(menu)) => (r, menu),
        _ => return 0.0,
    };
    let seat = squad_orders_seats(g, &menu, Some(r));
    let mut keys: i32 = 0;
    let wedges = g.k.cmdring.wedges as i32;
    for d in 1..=wedges {
        let at = d as usize;
        let on = d as f64 == seat.more || if menu.page.is_none() && !ring_more(Some(r)) && d == 1 {
            text(&squad_orders_menu_quick(g, &menu))
        } else {
            seat.row[at] >= 0.0 && (seat.row[at] as usize) < menu.rows.len()
        };
        if on { keys |= 1 << d; }
    }
    keys as f64
}

/// What the ring shows this frame: where it is, the lit wedge, whose colour it wears, the wedge on each digit. For the page: the simulation never reads it. (orders.ts ringView)
pub fn squad_orders_ring_view(g: &mut crate::game::Game) -> Option<crate::cmdring::RingView> {
    let (r, menu) = match (g.orders.cmd_ring, g.orders.menu.clone()) {
        (Some(r), Some(menu)) => (r, menu),
        _ => return None,
    };
    let mt = from_menu_target(squad_orders_menu_target(g).unwrap());
    let mut wedges = Vec::new();
    let seat = squad_orders_seats(g, &menu, Some(r));
    let n = g.k.cmdring.wedges as usize;
    for d in 1..=n {
        let i = seat.row[d];
        let row = if i >= 0.0 { menu.rows.get(i as usize).copied() } else { None };
        if d as f64 == seat.more {
            wedges.push(crate::cmdring::RingViewWedgesItem { key: d as f64, text: String::from("more"), off: false, opens: true });
        } else if menu.page.is_none() && !ring_more(Some(r)) && d == 1 {
            let quick = squad_orders_menu_quick(g, &menu);
            if text(&quick) { wedges.push(crate::cmdring::RingViewWedgesItem { key: 1.0, text: quick.unwrap(), off: false, opens: false }); }
        } else if let Some(row) = row {
            let words = squad_orders_label_of(g, row, &mt);
            let off = menu.why.get(i as usize).is_some_and(text);
            let opens = text(&g.tab.orders_order_def[row].opens);
            wedges.push(crate::cmdring::RingViewWedgesItem { key: d as f64, text: words, off, opens });
        }
    }
    let what = match &menu.target {
        OrderTarget::None(n) => n.man.map_or(String::new(), |man| g.gd[man].callsign.clone()),
        t => { let h = hover_target(t); squad_orders_hover_words(g, Some(&h)) }
    };
    let who = squad_orders_who_tag(g).tag;
    let tone = squad_orders_tone_of(g, &menu.men).map(|t| String::from(t.as_str()));
    let caption = if !what.is_empty() && what != who { format!("{} \u{2192} {}", who, what) } else { who };
    Some(crate::cmdring::RingView { x: r.ax, y: r.ay, zoom: r.z, lit: r.lit, tone, caption, wedges })
}

/// The roster as it stands; a new cast starts clean. (orders.ts sync)
pub fn squad_orders_sync(g: &mut crate::game::Game) -> Vec<GuardId> {
    let mut i = 0;
    let mut same = true;
    let mut at = 0;
    while at < g.guards.len() {
        let gd = g.guards[at];
        at += 1;
        if squad_orders_mine(g, gd) {
            if g.orders.roster.get(i) != Some(&gd) { same = false; }
            i += 1;
        }
    }
    if !same || i != g.orders.roster.len() {
        let squad = squad_orders_squad(g);
        g.orders.roster = squad.clone();
        g.orders.who = Who::All;
        g.orders.menu = None;
        g.orders.trail.clear();
        for gd in g.orders.asg.keys() {
            if !squad.contains(&gd) { squad_orders_drop(g, gd); }
        }
        crate::planlay::lay_fresh(g);
    }
    let stands = match g.orders.who {
        Who::Num(k) => g.orders.roster.get(k as usize).map_or(true, |&gd| g.gd[gd].state != crate::game::GuardState::Dead),
        Who::All => true,
        Who::Red => !squad_orders_men(g, Some(SquadOrdersMenSel::Red), None).is_empty(),
        Who::Blue => !squad_orders_men(g, Some(SquadOrdersMenSel::Blue), None).is_empty(),
    };
    if !stands {
        // the man the orders went to is down: a menu that was theirs goes with them, never on to everybody else
        g.orders.who = Who::All;
        g.orders.menu = None;
    }
    g.orders.roster.clone()
}

/// Sam is standing. (orders.ts samStopped)
pub fn squad_orders_sam_stopped(g: &mut crate::game::Game) -> bool {
    g.orders.sam_v <= 0.3
}

/// A man's order goes, and with it the pin the fight seam put on him. (orders.ts drop)
pub fn squad_orders_drop(g: &mut crate::game::Game, gd: GuardId) {
    g.orders.asg.delete(gd);
    if g.orders.ours.delete(gd) { g.gd[gd].pinned = false; }
}
