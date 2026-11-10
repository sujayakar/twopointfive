pub use ::game_types::interact::types;
pub use types::*;
pub mod hud;
pub use hud::*;
use crate::game::{Deed, Game};
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String};

/// every row a door can have, and those of them an open leaf can
static OPEN_ROWS: &[&str] = &["pull", "ease", "charge", "breach", "hammer", "wand"];
static SHUT_ROWS: &[&str] = &["pull", "ease", "charge", "breach", "hammer", "wand", "push", "pick", "kick", "rattle", "unlock", "code", "nocode"];

/// a level line from his eye to a hand's breadth short of the thing
fn walled(g: &Game, eye: V3, to: V3) -> bool {
    let dx = to[0] - eye[0];
    let dz = to[2] - eye[2];
    let k = m::max(0.0, 1.0 - crate::handring::EMPTY_SHY_M / m::max(1e-6, m::hyp2(dx, dz)));
    crate::scene_collision::solid_blocked(g, eye, [eye[0] + dx * k, eye[1], eye[2] + dz * k], None)
}

/// THE THING AN EMPTY TAP IS BESIDE, and why the key did nothing on it, about the seat it is asked of.
pub fn empty_thing(g: &Game, seat: &crate::seats::SeatRef) -> Option<crate::interact::EmptyThing> {
    let pl = seat.player;
    if g.pl[pl].takedown().is_some() { return None; }
    let p = g.ch[g.pl[pl].ch].pos;
    let eye: V3 = [p[0], if g.pl[pl].crouch { 0.9 } else { 1.3 }, p[2]];
    let mut best: Option<crate::interact::EmptyThing> = None;
    let mut rank = 3;
    let mut man = m::INF;
    let mut other = m::INF;
    for it in seat.interactables.iter() {
        let d = crate::player::thing_dist(g, it, p);
        if d > crate::handring::EMPTY_NEAR_M { continue; }
        if it.kind == crate::game::InteractableKind::Guard {
            if d < man && !walled(g, eye, it.pos) { man = d; }
            continue;
        }
        if !g.m.player.press_things.has(String::from(it.kind.as_str())) { continue; }
        let door = it.door;
        // (a door's reach is its distance and his feet: its `inReach` is also the big tool's, from where a stack stands)
        let r = if door.is_some() {
            if d >= crate::doors::DOORS_USE_REACH { 2 } else if g.pl[pl].sprinting { 1 } else { 0 }
        } else if it.in_reach { 0 } else { 3 };
        if r == 3 { other = m::min(other, d); continue; }
        let why_of = crate::handring::empty_why();
        let why: &str = if r == 0 {
            if it.framed.unwrap_or(false) { why_of.held.as_str() } else if door.is_some() { why_of.no.as_str() } else { "" }
        } else {
            let d0 = door.unwrap();
            let rows = if crate::doors::door_is_closed(g, d0) { SHUT_ROWS } else { OPEN_ROWS };
            let mut offered = false;
            for v in rows { if crate::rails::rails_offers(g, v) { offered = true; break; } }
            if !offered { why_of.held.as_str() } else if r == 1 { why_of.run.as_str() } else { why_of.far.as_str() }
        };
        if why.is_empty() { other = m::min(other, d); continue; }   // (in reach with no row and no frame to blame)
        if r > rank || (r == rank && best.as_ref().map_or(false, |b| d >= b.d)) { continue; }
        let skip = if r == 2 {
            let frame = g.dr[door.unwrap()].frame_centre;
            walled(g, eye, it.pos) && walled(g, eye, frame)
        } else if r == 1 {
            crate::doors::doors_locked_out(g, door.unwrap(), p, Some(&g.pl[pl].keys))
        } else { false };
        if skip { continue; }
        best = Some(crate::interact::EmptyThing { it: it.clone(), d, why: String::from(why) });
        rank = r;
    }
    match best {
        Some(b) if b.d <= man && (rank == 0 || b.d <= other) => Some(b),
        _ => None,
    }
}

/// what a tap that found no row says
pub fn empty_word(t: Option<&crate::interact::EmptyThing>) -> String {
    match t {
        Some(t) => {
            let head = t.it.line1.split(" · ").next().unwrap_or("");
            format!("{} · {}", if head.is_empty() { t.it.kind.as_str() } else { head }, t.why)
        }
        None => String::from(crate::handring::EMPTY_WORD),
    }
}

/// the movement keys down, a bit each
fn move_mask(g: &Game, input: InputId) -> f64 {
    let mut mask = 0i32;
    for (i, k) in crate::player::WALK_KEYS.iter().enumerate() {
        if crate::input::input_down(g, input, k) { mask |= 1 << i; }
    }
    mask as f64
}

/// Which sustained verb is running right now, as an identity ('' = none).
pub fn running_verb(g: &Game, seat: &crate::seats::SeatRef) -> String {
    let pl = seat.player;
    let (verb, d) = match &g.pl[pl].deed {
        Some(Deed::Ease(e)) => ("ease", e.door),
        Some(Deed::Pick(d)) => ("pick", *d),
        Some(Deed::Wand(w)) => ("wand", w.door),
        Some(Deed::Code(c)) => ("code", c.door),
        Some(Deed::Charge(c)) => ("charge", c.door),
        Some(Deed::Hold(h)) if crate::player::hold_works(h) => return format!("hold:{}", g.gd[h.g].callsign),
        Some(Deed::Breaker(_)) => return String::from("breaker"),
        Some(Deed::Take(_)) => return String::from("drive"),
        Some(Deed::Wall(_)) => return String::from("charge:wall"),
        _ => return String::new(),
    };
    format!("{verb}:{}", g.dr[d].def.name)
}

/// Stop the running sustained verb where it is. True if there was anything to stop.
pub(super) fn cancel_sustained(g: &mut Game) -> bool {
    let pl = g.player;
    match &g.pl[pl].deed {
        Some(Deed::Pick(_)) => { crate::player::stop_picking(g); true }
        Some(Deed::Wand(_)) => { crate::player::stop_wand(g); true }
        Some(Deed::Ease(_) | Deed::Code(_) | Deed::Charge(_) | Deed::Wall(_) | Deed::Take(_)) => { crate::deed::put_down(g, pl); true }
        Some(Deed::Hold(h)) if crate::player::hold_works(h) => { crate::player::ease_off_hold(g); true }
        _ => false,
    }
}

/// a row the ring has a wedge for: one with something to do
pub fn wedged(r: &crate::game::Interactable) -> bool {
    r.verb.is_some() && !r.running.unwrap_or(false)
}

/// the whistle has a wedge of the ring this frame
pub fn whistle_wedge(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    let pl = seat.player;
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].takedown().is_some() || crate::rails::rails_locks(g, "whistle") { return false; }
    let mut n = 0.0;
    for r in seat.nearby { { n += 1.0; } }
    n < g.k.cmdring.wedges
}

/// a wedge's digit pressed this frame
fn wedge_digit(g: &Game, input: InputId) -> bool {
    let mut d = 1;
    while d as f64 <= g.k.cmdring.wedges {
        if crate::input::input_hit(g, input, &format!("Digit{}", d)) { return true; }
        d += 1;
    }
    false
}

/// a row done by the key, a digit or a click: what updateInteract hands on as Game.act
fn give(g: &mut Game, r: Option<&crate::game::Interactable>) {
    let r = match r { Some(r) => r, None => return };
    if !r.in_reach || r.verb.is_none() || r.running.unwrap_or(false) || crate::samstun::sam_staggered(g, &crate::seats::here(g), None) { return; }
    crate::lightswitch::switch_let_go(g);
    g.sets[g.seat].act = Some(r.clone());
}

/// the digits his ring has a wedge on, a bit each, bit 1 the first
fn hand_ring_keys(g: &Game, seat: &crate::seats::SeatRef) -> f64 {
    let mut keys = 0i32;
    let n = m::min(seat.nearby.len() as f64, g.k.cmdring.wedges) as usize;
    for i in 0..n { if wedged(&seat.nearby[i]) { keys |= 1 << (i + 1); } }
    if whistle_wedge(g, seat) { (keys | 1 << (js::to_i32(g.k.cmdring.wedges) & 31)) as f64 } else { keys as f64 }
}

/// His ring is put away with its press spent, if it is up or its key is down.
pub fn interact_let_go(g: &mut Game) {
    if g.sets[g.seat].hand.sel.open || g.sets[g.seat].hand.sel.down_t >= 0.0 { close_selector(g, true); }
}

/// a standing ring's `free` (`!!S.ring?.free`)
fn ring_free(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    g.sets[seat.id].hand.sel.ring.as_ref().is_some_and(|r| r.state.free == Some(true))
}

/// his hands are on a choice or a verb
pub fn hands_busy(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    (g.sets[seat.id].hand.sel.open && !ring_free(g, seat)) || g.sets[seat.id].hand.sel.down_t >= 0.0 || !running_verb(g, seat).is_empty()
}

/// is there anything here the back key would back out of
pub fn backable_interact(g: &Game, seat: &crate::seats::SeatRef) -> bool {
    g.sets[seat.id].hand.sel.open || g.sets[seat.id].hand.sel.down_t >= 0.0 || !running_verb(g, seat).is_empty()
}

/// anything to CHOOSE in this list
pub fn choosable(rows: &[crate::game::Interactable]) -> bool {
    rows.iter().any(|r| r.verb.is_some() && !r.running.unwrap_or(false))
}

/// the list shut, the press itself untouched
fn shut_list(g: &mut Game) {
    let ring = g.sets[g.seat].hand.sel.ring.is_some();
    g.sets[g.seat].hand.sel.open = false;
    g.sets[g.seat].hand.sel.ring = None;
    if ring && g.sets[g.seat].hand.near_sel != 0 { g.sets[g.seat].hand.near_sel = 0; crate::player::pick_hover(g); }
}

/// shut the list AND end the press (`spent`: it must also come up before Space arms again)
fn close_selector(g: &mut Game, spent: bool) {
    shut_list(g);
    g.sets[g.seat].hand.sel.down_t = -1.0;
    if spent { g.sets[g.seat].hand.f_held = true; }
}

/// the point his ring stands on: the thing its first row is about where that is near him; else his own body
fn ring_spot(g: &Game, seat: &crate::seats::SeatRef) -> V3 {
    let p = g.ch[g.pl[seat.player].ch].pos;
    match seat.nearby.iter().find(|r| wedged(r)) {
        Some(h) if v3::dist_xz(h.pos, p) <= RING_THING_M => h.pos,
        _ => [p[0], p[1] + RING_BODY_M, p[2]],
    }
}

/// the ring put on its thing (`open`) and kept there. True: the hand has not moved the cursor since the last look.
fn ring_on_thing(g: &Game, seat: &crate::seats::SeatRef, r: &mut crate::cmdring::RingState, input: InputId, cam: CameraId, view: crate::host_hud::View, open: Option<bool>) -> bool {
    let open = open.unwrap_or(false);
    let (mouse_x, mouse_y) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    if open {
        r.ox = Some(0.0); r.oy = Some(0.0); r.mx = Some(mouse_x); r.my = Some(mouse_y); r.clear = Some(false); r.pt = Some(0.0);
    } else if r.at.is_none() { return false; }
    let at = ring_spot(g, seat);
    r.at = Some(at);
    let (sx, sy, front) = crate::camera::follow_camera_project(g, cam, at, view.client_width, view.client_height, None, None);
    if front { crate::cmdring::ring_pin(g, r, sx, sy, view.client_width, view.client_height); }
    if open { r.armed = m::hyp2(mouse_x - r.ax, mouse_y - r.ay) < g.k.cmdring.flick_px * r.z; }
    let still = r.mx == Some(mouse_x) && r.my == Some(mouse_y);
    r.mx = Some(mouse_x);
    r.my = Some(mouse_y);
    still
}

/// The selector's ring lent to a call that also takes `g`: neither of those that borrow it (where the ring is pinned, and where it wants to be) reads the selector.
fn lend_ring<T>(g: &mut Game, f: impl FnOnce(&mut Game, &mut crate::cmdring::RingState) -> T) -> T {
    let mut r = ring(g).state;
    let out = f(g, &mut r);
    ring(g).state = r;
    out
}
/// the ring that is up
fn ring(g: &mut Game) -> &mut crate::game::PlayerSelRing { g.sets[g.seat].hand.sel.ring.as_mut().unwrap() }

/// His interact key, the back key and what they are read by, this tick.
#[derive(Clone, Copy)]
struct Press { dt: f64, input: InputId, view: Option<crate::host_hud::View>, cam: Option<CameraId>, down: bool, hit: bool, back: bool }
impl Press {
    /// the ring put on its thing (`open`), or kept there: has the hand not moved the cursor since the last look?
    fn ring_on_thing(&self, g: &mut Game, open: bool) -> bool {
        let (Some(cam), Some(view)) = (self.cam, self.view) else { return false };
        lend_ring(g, |g, r| ring_on_thing(g, &crate::seats::here(g), r, self.input, cam, view, Some(open)))
    }
    /// a mouse button pressed this tick is the ring's and nothing else's
    fn swallow(&self, g: &mut Game, bit: i32) {
        let i = &mut g.tab.input[self.input];
        i.clicked = (js::to_i32(i.clicked) & !bit) as f64;
    }
}

/// Once a tick, straight after the rows are built: the arrows, the back key and the interact key; what is done goes out as the seat's `act`.
pub fn update_interact(g: &mut Game, dt: f64, input: InputId, view: Option<crate::host_hud::View>, cam: Option<CameraId>) {
    let key = crate::consts::INTERACT_KEY;
    let (down, hit) = (crate::input::input_down(g, input, key), crate::input::input_hit(g, input, key));
    let k = Press { dt, input, view, cam, down, hit, back: crate::backkey::back_hit(g, input) };
    g.sets[g.seat].act = None;
    if !down && !hit { g.sets[g.seat].hand.f_held = false; }   // (a spent press has come up: the key is live again)
    if crate::input::input_hit(g, input, "Paused") { interact_let_go(g); }
    if verb_runs(g, k) { return; }
    if crate::condition::down(g, g.pl[g.player].ch) { close_selector(g, false); return; }
    let free = g.sets[g.seat].hand.sel.open && ring_free(g, &crate::seats::here(g));
    let s = &mut g.sets[g.seat].hand;
    if hit && !s.f_held && !free { s.sel.down_t = 0.0; s.sel.open = false; s.sel.ring = None; s.sel.word = String::new(); }
    let armed = s.sel.down_t >= 0.0;
    let live = choosable(&g.sets[g.seat].nearby) || whistle_wedge(g, &crate::seats::here(g));
    if g.sets[g.seat].hand.sel.open && !live { shut_list(g); }
    let s = &g.sets[g.seat];
    let ring_up = s.hand.sel.open && s.hand.sel.ring.is_some();
    if ring_up && (s.orders.menu.is_some() || s.orders.cmd_ring.is_some()) { close_selector(g, down); return; }   // (one list at a time)
    if armed && live && !s.hand.sel.open && s.orders.cmd_ring.is_none() && down && (wedge_digit(g, input) || s.hand.sel.down_t >= g.k.handring.hold_secs - 1e-6) { ring_opens(g, k); }
    if g.sets[g.seat].hand.sel.open && g.sets[g.seat].hand.sel.ring.is_some() { ring_tick(g, k); return; }
    light_moves(g, input);
    if k.back && armed { close_selector(g, true); return; }   // (the press is spent: nothing happens on its release)
    if !armed { if g.sets[g.seat].hand.sel.open { close_selector(g, false); } return; }
    if down { held_on(g, dt); return; }
    released(g);
}
/// The key is kept down; a hitch of the host does not turn a tap into a hold.
fn held_on(g: &mut Game, dt: f64) { g.sets[g.seat].hand.sel.down_t += m::min(dt, 0.05); }

/// What he is in the middle of: the key again, the back key or a step INTO it stops it, and nothing else is on offer. True: there is such a thing.
fn verb_runs(g: &mut Game, k: Press) -> bool {
    let run = running_verb(g, &crate::seats::here(g));
    let mask = if run.is_empty() { 0.0 } else { move_mask(g, k.input) };
    let hand = &mut g.sets[g.seat].hand;
    if run.is_empty() { hand.sel.run_key = run; hand.sel.move_latch = 0.0; return false; }
    if run != hand.sel.run_key { hand.sel.run_key = run; hand.sel.move_latch = mask; }
    else { hand.sel.move_latch = (js::to_i32(hand.sel.move_latch) & js::to_i32(mask)) as f64; }
    let steps = js::to_i32(mask) & !js::to_i32(hand.sel.move_latch) != 0;
    if hand.sel.open || hand.sel.down_t >= 0.0 { close_selector(g, k.down); }
    let hand = &mut g.sets[g.seat].hand;
    let again = k.hit && !hand.f_held;
    if again { hand.f_held = true; }
    if again || k.back || steps { cancel_sustained(g); }
    true
}

/// The key held: his rows go up as a ring, on the first of their things where it is near.
fn ring_opens(g: &mut Game, k: Press) {
    let (w, h) = k.view.map_or((0.0, 0.0), |v| (v.client_width, v.client_height));
    let (time, lit) = (g.time - g.k.cmdring.tap_secs, (g.sets[g.seat].hand.near_sel + 1) as f64);
    g.sets[g.seat].hand.sel.open = true;
    let state = crate::cmdring::ring_open(g, k.input, w, h, time, Some(lit));
    let hand = &mut g.sets[g.seat].hand;
    hand.sel.ring = Some(crate::game::PlayerSelRing { state, key: hand.near_key.clone(), whistle: None });
    crate::player::pick_hover(g);
    k.ring_on_thing(g, true);
}

/// The ring is up: what the hand points it at is lit, and a click, a digit or the key let go of does it.
fn ring_tick(g: &mut Game, k: Press) {
    if crate::input::input_hit(g, k.input, "Blur") { close_selector(g, true); return; }
    if crate::input::input_rmb_hit(g, k.input) { k.swallow(g, 4); close_selector(g, true); return; }
    let lmb = crate::input::input_lmb_hit(g, k.input);
    let digit = digit_taken(g, k.input);
    rows_changed(g, k);
    let still = ring(g).state.at.is_some() && k.ring_on_thing(g, false);
    let (at, to_whistle) = ring_points(g, k, digit, still);
    let whistles = ring(g).whistle == Some(true);
    let near_sel = g.sets[g.seat].hand.near_sel;
    ring(g).state.lit = if whistles { g.k.cmdring.wedges } else { (near_sel + 1) as f64 };
    if lmb {
        k.swallow(g, 1);
        let i = &mut g.tab.input[k.input];
        i.buttons = (js::to_i32(i.buttons) & !1) as f64;
        if k.input == g.tab.cmd_seat[g.seat].input { crate::cmd::seat_swallow(g, g.seat, 1.0); }
    }
    let lit = if whistles { None } else { g.sets[g.seat].hover.clone().filter(wedged) };
    let free = ring(g).state.free == Some(true);
    let pointed = to_whistle || at.is_some();
    let asked = lmb || (digit > 0 && pointed) || (if free { k.hit } else { !k.down && digit == 0 && pointed });
    let can = if whistles { crate::whistle::whistle_ready_in(g) <= 0.0 } else { lit.as_ref().is_some_and(|r| r.in_reach) };
    if asked && can {
        close_selector(g, k.down);
        if !whistles { give(g, lit.as_ref()); }
        else if !crate::samstun::sam_staggered(g, &crate::seats::here(g), None) { crate::whistle::try_whistle(g); }
        return;
    }
    if !free && !k.down {   // (the key let go of on nothing: the ring stays up)
        ring(g).state.free = Some(true);
        g.sets[g.seat].hand.sel.down_t = -1.0;
    }
    if k.back { close_selector(g, true); }
    else if k.down && !free { held_on(g, k.dt); }
}
/// The digit pressed this tick, 0 for none: the ring's while it is up, not his slots'.
fn digit_taken(g: &mut Game, input: InputId) -> i32 {
    let pressed = &mut g.tab.input[input].pressed;
    (1..=9).rev().fold(0, |digit, d| if pressed.delete(format!("Digit{}", d)) { d } else { digit })
}
/// The rows changed under it: nothing is pointed at until the hand moves again, and it goes to its thing.
fn rows_changed(g: &mut Game, k: Press) {
    let key = g.sets[g.seat].hand.near_key.clone();
    let r = ring(g);
    if r.key == key { return; }
    r.key = key;
    r.whistle = Some(false);
    r.state.armed = false;
    r.state.clear = Some(false);
    if r.state.at.is_some() { k.ring_on_thing(g, true); }
}
/// Which row the hand or a digit points at, and whether it is the whistle's wedge instead. That one is lit.
fn ring_points(g: &mut Game, k: Press, digit: i32, still: bool) -> (Option<i32>, bool) {
    let keys = hand_ring_keys(g, &crate::seats::here(g));
    let lone = crate::ringlone::lone_key(g, &mut |_| keys);
    let stands = ring(g).state.at.is_some();
    let time = g.time;
    let way = lend_ring(g, |g, r| crate::cmdring::ring_want(g, r, k.input, digit as f64, time, Some(lone), Some(if stands { keys } else { 0.0 })));
    let want = if still && digit == 0 && way != ring(g).state.pt.unwrap_or(0.0) { 0.0 } else { way };   // (a ring that moved under a still hand was not pointed)
    if stands && (digit != 0 || !still) { ring(g).state.pt = Some(if digit != 0 { 0.0 } else { way }); }
    let whistle = whistle_wedge(g, &crate::seats::here(g));
    let to_whistle = whistle && want == g.k.cmdring.wedges;
    let nearby = &g.sets[g.seat].nearby;
    let at = if js::truthy(want) && !to_whistle && nearby.get((want - 1.0) as usize).is_some_and(wedged) { Some((want - 1.0) as i32) } else { None };
    let only = whistle && !choosable(nearby);
    if let Some(at) = at {
        ring(g).whistle = Some(false);
        if at != g.sets[g.seat].hand.near_sel { g.sets[g.seat].hand.near_sel = at; crate::player::pick_hover(g); }
    }
    if to_whistle || only { ring(g).whistle = Some(true); } else if !whistle { ring(g).whistle = Some(false); }
    (at, to_whistle)
}

/// The arrows move the light over his rows.
fn light_moves(g: &mut Game, input: InputId) {
    let by = crate::input::input_hit(g, input, "ArrowDown") as i32 - crate::input::input_hit(g, input, "ArrowUp") as i32;
    if by != 0 && g.sets[g.seat].nearby.len() > 1 { g.sets[g.seat].hand.near_sel += by; crate::player::pick_hover(g); }
    let hand = &mut g.sets[g.seat].hand;
    if let Some(r) = hand.sel.ring.as_mut() { r.state.lit = (hand.near_sel + 1) as f64; }
}

/// The key let go of, or tapped: the row that is lit is done; with none, he is told why nothing is.
fn released(g: &mut Game) {
    let lit = g.sets[g.seat].hover.clone();
    let sel = &g.sets[g.seat].hand.sel;
    let (listed, quick) = (sel.open, sel.down_t < g.k.handring.hold_secs);
    close_selector(g, false);
    if !listed && lit.is_none() && !crate::samstun::sam_staggered(g, &crate::seats::here(g), None) {
        let w = empty_word(empty_thing(g, &crate::seats::here(g)).as_ref());
        refuse(g, g.player, None, &w);
        return;
    }
    if quick && crate::ownblow::own_blow_held(g, lit.as_ref()) { return; }
    give(g, lit.as_ref());
}

/// A press of his own that did nothing says why: a greyed row of its own after his rows, for `EMPTY_SHOW_SECS` from now (a button held on it comes back here each frame).
/// `kind`: what the word is about. None is an empty tap, the one word the view may take down early.
pub fn refuse(g: &mut Game, pl: PlayerId, kind: Option<crate::view::ThingKind>, words: &str) {
    let time = g.time;
    let his = crate::seats::seat_of_man(g, pl);
    let s = &mut g.sets[his].hand.sel;
    s.word = String::from(words);
    s.word_t = time;
    s.word_kind = kind;
}

/// The answer to a press that a teammate gives: `line` by the man it is about or the nearest who can speak, and with nobody there, `alone` as his own press's word.
pub fn say_why(g: &mut Game, about: Option<GuardId>, kind: crate::view::ThingKind, line: &str, alone: &str) {
    if !crate::squadnet::team_says(g, about, line) {
        let pl = g.player;
        refuse(g, pl, Some(kind), alone);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Interactable;

    #[test]
    fn what_has_a_wedge() {
        let mut r = Interactable::default();
        assert!(!wedged(&r));   // a status line
        r.verb = Some(crate::game::Verb::Push);
        assert!(wedged(&r) && choosable(core::slice::from_ref(&r)));
        r.running = Some(true);
        assert!(!wedged(&r) && !choosable(core::slice::from_ref(&r)));
    }
}
