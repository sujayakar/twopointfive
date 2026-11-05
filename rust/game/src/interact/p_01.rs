//! src/game/interact.ts lines 1 to 295
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// every row a door can have, and those of them an open leaf can (interact.ts OPEN_ROWS)
static OPEN_ROWS: &[&str] = &["pull", "ease", "charge", "breach", "hammer", "wand"];
/// (interact.ts SHUT_ROWS: OPEN_ROWS first)
static SHUT_ROWS: &[&str] = &["pull", "ease", "charge", "breach", "hammer", "wand", "push", "pick", "kick", "rattle", "unlock", "code", "nocode"];
/// (interact.ts MOVE_KEYS)
static MOVE_KEYS: &[&str] = &["KeyW", "KeyA", "KeyS", "KeyD"];

/// a level line from his eye to a hand's breadth short of the thing (interact.ts emptyThing `walled`)
fn walled(g: &mut Game, eye: V3, to: V3) -> bool {
    let dx = to[0] - eye[0];
    let dz = to[2] - eye[2];
    let k = m::max(0.0, 1.0 - crate::handring::EMPTY_SHY_M / m::max(1e-6, m::hyp2(dx, dz)));
    crate::scene_collision::solid_blocked(g, eye, [eye[0] + dx * k, eye[1], eye[2] + dz * k], None)
}

/// THE THING AN EMPTY TAP IS BESIDE, and why the key did nothing on it. (interact.ts emptyThing)
pub fn empty_thing(g: &mut Game) -> Option<crate::interact::EmptyThing> {
    let pl = g.player;
    if g.pl[pl].takedown.is_some() { return None; }
    let p = g.ch[g.pl[pl].ch].pos;
    let eye: V3 = [p[0], if g.pl[pl].crouch { 0.9 } else { 1.3 }, p[2]];
    let mut best: Option<crate::interact::EmptyThing> = None;
    let mut rank = 3;
    let mut man = m::INF;
    let mut other = m::INF;
    // (nothing below changes the list: each row is copied so that the calls may take `g`)
    for i in 0..g.interactables.len() {
        let it = g.interactables[i].clone();
        let d = crate::player::thing_dist(g, &it, p);
        if d > crate::handring::EMPTY_NEAR_M { continue; }
        if it.kind == crate::game::InteractableKind::Guard {
            if d < man && !walled(g, eye, it.pos) { man = d; }
            continue;
        }
        if !g.m.player.press_things.has(String::from(it.kind.as_str())) { continue; }
        let door = it.door;
        // (a door's reach is its distance and his feet: its `inReach` is also the big tool's, from where a stack stands)
        let r = if door.is_some() {
            if d >= seam_doors_use_reach() { 2 } else if g.pl[pl].sprinting { 1 } else { 0 }
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
        if why.is_empty() { other = m::min(other, d); continue; }   // (in reach with no row and no frame to blame: a wounded man)
        if r > rank || (r == rank && best.as_ref().map_or(false, |b| d >= b.d)) { continue; }
        let skip = if r == 2 {
            let frame = g.dr[door.unwrap()].frame_centre;
            walled(g, eye, it.pos) && walled(g, eye, frame)
        } else if r == 1 {
            let keys = g.pl[pl].keys.clone();
            crate::doors::doors_locked_out(g, door.unwrap(), p, Some(&keys))
        } else { false };
        if skip { continue; }
        best = Some(crate::interact::EmptyThing { it, d, why: String::from(why) });
        rank = r;
    }
    match best {
        Some(b) if b.d <= man && (rank == 0 || b.d <= other) => Some(b),
        _ => None,
    }
}

/// what a tap that found no row says (interact.ts emptyWord; `t` None = its default, the thing as of now)
pub fn empty_word(g: &mut Game, t: Option<&crate::interact::EmptyThing>) -> String {
    let own;
    let t = match t {
        Some(t) => Some(t),
        None => { own = empty_thing(g); own.as_ref() }
    };
    match t {
        Some(t) => {
            let head = t.it.line1.split(" · ").next().unwrap_or("");
            format!("{} · {}", if head.is_empty() { t.it.kind.as_str() } else { head }, t.why)
        }
        None => String::from(crate::handring::EMPTY_WORD),
    }
}

/// the movement keys down, a bit each (interact.ts moveMask)
pub fn move_mask(g: &mut Game, input: InputId) -> f64 {
    let mut mask = 0i32;
    for (i, k) in MOVE_KEYS.iter().enumerate() {
        if crate::input::input_down(g, input, k) { mask |= 1 << i; }
    }
    mask as f64
}

/// Which sustained verb is running right now, as an identity ('' = none). (interact.ts runningVerb)
pub fn running_verb(g: &mut Game) -> String {
    let pl = g.player;
    if let Some(d) = g.pl[pl].easing { return format!("ease:{}", g.dr[d].def.name); }
    if let Some(d) = g.pl[pl].picking { return format!("pick:{}", g.dr[d].def.name); }
    if let Some(w) = &g.pl[pl].wanding { let d = w.door; return format!("wand:{}", g.dr[d].def.name); }
    if let Some(d) = g.pl[pl].coding { return format!("code:{}", g.dr[d].def.name); }
    if let Some(h) = &g.pl[pl].holding {
        if h.wind_t >= 0.0 || h.phase == crate::character::HoldPhase::Choke || h.phase == crate::character::HoldPhase::Whip || crate::player::hold_killing(h) {
            let who = h.g;
            return format!("hold:{}", g.gd[who].callsign);
        }
    }
    if g.mission.drive_t >= 0.0 { return String::from("drive"); }
    if g.pl[pl].breaker_move.is_some() { return String::from("breaker"); }
    if let Some(c) = &g.pl[pl].charging { let d = c.door; return format!("charge:{}", g.dr[d].def.name); }
    if crate::wallcharge::wall_charge_setting(g) { return String::from("charge:wall"); }
    if crate::defuse::defuse_working(g) { return String::from("defuse"); }
    String::new()
}

/// Stop the running sustained verb where it is. True if there was anything to stop. (interact.ts cancelSustained)
pub fn cancel_sustained(g: &mut Game) -> bool {
    let pl = g.player;
    if g.pl[pl].easing.is_some() { g.pl[pl].easing = None; return true; }
    if g.pl[pl].picking.is_some() { crate::player::stop_picking(g, None); return true; }
    if g.pl[pl].wanding.is_some() { crate::player::stop_wand(g, None); return true; }
    if g.pl[pl].coding.is_some() { crate::devices::stop_coding(g); return true; }
    let (held, killing, working) = match &g.pl[pl].holding {
        Some(h) => (true, crate::player::hold_killing(h), h.wind_t >= 0.0 || h.phase == crate::character::HoldPhase::Choke || h.phase == crate::character::HoldPhase::Whip),
        None => (false, false, false),
    };
    if held && killing { return false; }   // the knife in the hold is COMMITTED
    if held && working { crate::player::ease_off_hold(g); return true; }
    if g.mission.drive_t >= 0.0 { crate::mission::cancel_drive(g); return true; }
    if crate::doortools::door_tools_cancel_sam(g) { return true; }
    if crate::wallcharge::wall_charge_cancel(g) { return true; }
    if crate::defuse::defuse_cancel(g) { return true; }
    false
}

/// a row the ring has a wedge for: one with something to do (interact.ts wedged)
pub fn wedged(r: &crate::game::Interactable) -> bool {
    r.verb.is_some() && !r.running.unwrap_or(false) && r.kind != crate::game::InteractableKind::Cancel
}

/// `\s` of a JavaScript pattern, and what its `trim()` takes away
fn js_space(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\u{b}' | '\u{c}' | '\r' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
}

/// what stands before the first `\s+<sep>\s+` of `s`: `s.split(/\s+<sep>\s+/)[0]`
fn before_dash(s: &str, sep: fn(char) -> bool) -> &str {
    for (i, c) in s.char_indices() {
        if !js_space(c) { continue; }
        let rest = s[i..].trim_start_matches(js_space);
        let mut it = rest.chars();
        if it.next().map_or(false, sep) && it.next().map_or(false, js_space) { return &s[..i]; }
    }
    s
}

/// a row's words as the panel has them, less its key and less what follows a dot (interact.ts rowWords)
pub fn row_words(r: &crate::game::Interactable) -> String {
    let s = r.line2.as_str();
    let s = match s.strip_prefix("Space") {
        Some(t) if t.starts_with(js_space) => t.trim_start_matches(js_space),
        _ => s,
    };
    String::from(before_dash(s, |c| c == '·').trim_matches(js_space))
}

/// …and up to a dash: what its wedge says (interact.ts wedgeWords)
pub fn wedge_words(r: &crate::game::Interactable) -> String {
    String::from(before_dash(&row_words(r), |c| matches!(c, '—' | '–' | '-')))
}

/// the whistle has a wedge of the ring this frame (interact.ts whistleWedge)
pub fn whistle_wedge(g: &mut Game) -> bool {
    let pl = g.player;
    if !ring_stays(g) || g.pl[pl].down || g.pl[pl].takedown.is_some() || crate::rails::rails_locks(g, "whistle") { return false; }
    let mut n = 0.0;
    for r in &g.nearby { if r.kind != crate::game::InteractableKind::Cancel { n += 1.0; } }
    n < g.k.cmdring.wedges
}

/// Sam's own ring is on (interact.ts ringMode)
pub fn ring_mode(g: &mut Game) -> bool { g.puppet.is_none() }

/// it is the ring that stays (interact.ts ringStays)
pub fn ring_stays(g: &mut Game) -> bool { g.puppet.is_none() }

/// a wedge's digit pressed this frame (interact.ts wedgeDigit)
pub fn wedge_digit(g: &mut Game, input: InputId) -> bool {
    let mut d = 1;
    while d as f64 <= g.k.cmdring.wedges {
        if crate::input::input_hit(g, input, &format!("Digit{}", d)) { return true; }
        d += 1;
    }
    false
}

/// a row done by the key, a digit or a click: what updateInteract hands on as Game.act (interact.ts give)
pub fn give(g: &mut Game, r: Option<&crate::game::Interactable>) {
    let r = match r { Some(r) => r, None => return };
    if r.kind == crate::game::InteractableKind::Cancel || !r.in_reach || r.verb.is_none() || r.running.unwrap_or(false) || crate::samstun::sam_staggered(g, None) { return; }
    crate::lightswitch::switch_let_go(g);
    g.act = Some(r.clone());
}

/// What his ring shows this frame; ui/cmdring.ts draws it, no rule reads it. (interact.ts handRingView)
pub fn hand_ring_view(g: &mut Game) -> Option<crate::cmdring::RingView> {
    let pl = g.player;
    if !g.pl[pl].sel.open { return None; }
    let (ax, ay, z, whistle) = match &g.pl[pl].sel.ring {
        Some(r) => (r.ax, r.ay, r.z, r.whistle.unwrap_or(false)),
        None => return None,
    };
    let count = g.k.cmdring.wedges;
    let off_stands = crate::interact::state_off();
    let mut wedges = Vec::new();
    for (i, r) in g.nearby.iter().enumerate() {
        if i as f64 >= count { break; }
        if !wedged(r) { continue; }
        let stands = off_stands.iter().any(|k| k == r.kind.as_str());
        wedges.push(crate::cmdring::RingViewWedgesItem { key: (i + 1) as f64, text: wedge_words(r), off: (r.off && !stands) || !r.in_reach, opens: false });
    }
    if whistle_wedge(g) {
        let off = crate::whistle::whistle_ready_in(g) > 0.0;
        wedges.push(crate::cmdring::RingViewWedgesItem { key: count, text: String::from(WHISTLE_WEDGE), off, opens: false });
    }
    let near_sel = g.pl[pl].near_sel;
    let lit = if whistle { count } else if g.hover.as_ref().map_or(false, wedged) && (near_sel as f64) < count { (near_sel + 1) as f64 } else { 0.0 };
    Some(crate::cmdring::RingView { x: ax, y: ay, zoom: z, lit, tone: Some(String::from("hands")), caption: String::new(), wedges })
}

/// the digits his ring has a wedge on, a bit each, bit 1 the first (interact.ts handRingKeys)
pub fn hand_ring_keys(g: &mut Game) -> f64 {
    let mut keys = 0i32;
    let n = m::min(g.nearby.len() as f64, g.k.cmdring.wedges) as usize;
    for i in 0..n { if wedged(&g.nearby[i]) { keys |= 1 << (i + 1); } }
    if whistle_wedge(g) { (keys | 1 << (js::to_i32(g.k.cmdring.wedges) & 31)) as f64 } else { keys as f64 }
}

/// Esc, ahead of the pause menu: the open selector backs out, else a running sustained verb stops. (interact.ts escapeInteract)
pub fn escape_interact(g: &mut Game) -> bool {
    let pl = g.player;
    if g.pl[pl].sel.open || g.pl[pl].sel.down_t >= 0.0 { close_selector(g, true); return true; }
    cancel_sustained(g)
}

/// a standing ring's `free` (`!!S.ring?.free`)
fn ring_free(g: &Game) -> bool {
    g.pl[g.player].sel.ring.as_ref().map_or(false, |r| r.free.unwrap_or(false))
}

/// his hands are on a choice or a verb (interact.ts handsBusy)
pub fn hands_busy(g: &mut Game) -> bool {
    let pl = g.player;
    (g.pl[pl].sel.open && !ring_free(g)) || g.pl[pl].sel.down_t >= 0.0 || !running_verb(g).is_empty()
}

/// is there anything here the back key would back out of (interact.ts backableInteract)
pub fn backable_interact(g: &mut Game) -> bool {
    let pl = g.player;
    g.pl[pl].sel.open || g.pl[pl].sel.down_t >= 0.0 || !running_verb(g).is_empty()
}

/// anything to CHOOSE in this list (interact.ts choosable)
pub fn choosable(rows: &[crate::game::Interactable]) -> bool {
    rows.iter().any(|r| r.kind != crate::game::InteractableKind::Cancel && r.verb.is_some() && !r.running.unwrap_or(false))
}

/// the list opened, its cancel row at its foot at once (interact.ts openSelector)
pub fn open_selector(g: &mut Game) {
    let pl = g.player;
    if g.pl[pl].sel.open { return; }
    g.pl[pl].sel.open = true;
    if g.nearby.last().map_or(false, |r| r.kind != crate::game::InteractableKind::Cancel) {
        let row = crate::player::cancel_row(g);
        g.nearby.push(row);
    }
    crate::player::pick_hover(g);
}

/// the list shut, the press itself untouched (interact.ts shutList)
pub fn shut_list(g: &mut Game) {
    let pl = g.player;
    let ring = g.pl[pl].sel.ring.is_some();
    g.pl[pl].sel.open = false;
    g.pl[pl].sel.ring = None;
    if ring && ring_stays(g) && g.pl[pl].near_sel != 0 { g.pl[pl].near_sel = 0; crate::player::pick_hover(g); }
    if g.nearby.iter().any(|r| r.kind == crate::game::InteractableKind::Cancel) {
        let hv = g.hover.as_ref().map(|r| r.kind);
        g.nearby.retain(|r| r.kind != crate::game::InteractableKind::Cancel);
        if hv == Some(crate::game::InteractableKind::Cancel) { g.pl[pl].near_sel = 0; }
        crate::player::pick_hover(g);
    }
}

/// shut the list AND end the press (`spent`: it must also come up before Space arms again) (interact.ts closeSelector)
pub fn close_selector(g: &mut Game, spent: bool) {
    let pl = g.player;
    shut_list(g);
    g.pl[pl].sel.down_t = -1.0;
    g.pl[pl].sel.moved = false;
    if spent { g.pl[pl].f_held = true; }
}

/// the point his ring stands on: the thing its first row is about where that is near him; else his own body (interact.ts ringSpot)
pub fn ring_spot(g: &mut Game) -> V3 {
    let p = g.ch[g.pl[g.player].ch].pos;
    match g.nearby.iter().find(|r| wedged(r)) {
        Some(h) if v3::dist_xz(h.pos, p) <= RING_THING_M => h.pos,
        _ => [p[0], p[1] + RING_BODY_M, p[2]],
    }
}

/// the ring put on its thing (`open`) and kept there. True: the hand has not moved the cursor since the last look. (interact.ts ringOnThing)
pub fn ring_on_thing(g: &mut Game, r: &mut crate::cmdring::RingState, input: InputId, view: crate::host_hud::View, open: Option<bool>) -> bool {
    let open = open.unwrap_or(false);
    let (mouse_x, mouse_y) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    if open {
        r.ox = Some(0.0); r.oy = Some(0.0); r.mx = Some(mouse_x); r.my = Some(mouse_y); r.clear = Some(false); r.pt = Some(0.0);
    } else if r.at.is_none() { return false; }
    let at = ring_spot(g);
    r.at = Some(at);
    let (sx, sy, front) = crate::camera::follow_camera_project(g, at, view.client_width, view.client_height, None, None);
    if front { crate::cmdring::ring_pin(g, r, sx, sy, view.client_width, view.client_height); }
    if open { r.armed = m::hyp2(mouse_x - r.ax, mouse_y - r.ay) < g.k.cmdring.flick_px * r.z; }
    let still = r.mx == Some(mouse_x) && r.my == Some(mouse_y);
    r.mx = Some(mouse_x);
    r.my = Some(mouse_y);
    still
}

/// the selector's ring as the plain RingState it is, less its own two words
fn ring_state(ring: &crate::game::PlayerSelRing) -> crate::cmdring::RingState {
    crate::cmdring::RingState { ax: ring.ax, ay: ring.ay, z: ring.z, t0: ring.t0, lit: ring.lit, armed: ring.armed, free: ring.free, more: ring.more, at: ring.at, ox: ring.ox, oy: ring.oy, mx: ring.mx, my: ring.my, clear: ring.clear, pt: ring.pt }
}

/// the selector's ring lent as a RingState to a call that also takes `g`, and what the call wrote put back. Its callees here (ringSpot, project, ringPin, ringWant) do not read the selector.
fn lend_ring<T>(g: &mut Game, f: impl FnOnce(&mut Game, &mut crate::cmdring::RingState) -> T) -> T {
    let pl = g.player;
    let mut r = ring_state(g.pl[pl].sel.ring.as_ref().unwrap());
    let out = f(g, &mut r);
    let ring = g.pl[pl].sel.ring.as_mut().unwrap();
    ring.ax = r.ax; ring.ay = r.ay; ring.z = r.z; ring.t0 = r.t0; ring.lit = r.lit; ring.armed = r.armed; ring.free = r.free; ring.more = r.more; ring.at = r.at;
    ring.ox = r.ox; ring.oy = r.oy; ring.mx = r.mx; ring.my = r.my; ring.clear = r.clear; ring.pt = r.pt;
    out
}

/// `ringOnThing(g, S.ring, input, cam, view, open)`
fn ring_on_own(g: &mut Game, input: InputId, view: crate::host_hud::View, open: Option<bool>) -> bool {
    lend_ring(g, |g, r| ring_on_thing(g, r, input, view, open))
}

/// `input.backHit`: read, not taken; `Keys` is its own type, so it is handed a copy of the frame's presses
fn input_back_hit(g: &mut Game, input: InputId) -> bool {
    let keys = crate::backkey::Keys { pressed: g.tab.input[input].pressed.clone() };
    crate::backkey::back_hit(g, &keys)
}

/// `x &= ~bits` on one of the Input's words of bits
fn off_bits(x: f64, bits: i32) -> f64 { (js::to_i32(x) & !bits) as f64 }

/// Once a frame, straight after the rows are built: the wheel, ↑ / ↓, Esc and the Space press; what is performed goes out as Game.act. (interact.ts updateInteract)
pub fn update_interact(g: &mut Game, dt: f64, input: InputId, view: Option<crate::host_hud::View>) {
    let pl = g.player;
    g.act = None;
    let key = crate::consts::INTERACT_KEY;
    let down = crate::input::input_down(g, input, key);
    let hit = crate::input::input_hit(g, input, key);
    let esc = input_back_hit(g, input);
    let wheel = g.tab.input[input].wheel;
    let wheeled = js::truthy(wheel);
    if g.pl[pl].f_held && !down && !hit { g.pl[pl].f_held = false; }   // a spent press has come up: Space is live again
    if crate::input::input_hit(g, input, "Paused") && (g.pl[pl].sel.open || g.pl[pl].sel.down_t >= 0.0) { close_selector(g, true); }
    // ---- a sustained verb is running: Space (a fresh press), Esc, or a step INTO it stops it; nothing else is on offer meanwhile
    let run = running_verb(g);
    if !run.is_empty() {
        let mask = move_mask(g, input);
        if run != g.pl[pl].sel.run_key {
            g.pl[pl].sel.run_key = run;
            g.pl[pl].sel.move_latch = mask;
        } else {
            g.pl[pl].sel.move_latch = (js::to_i32(g.pl[pl].sel.move_latch) & js::to_i32(mask)) as f64;
        }
        if g.pl[pl].sel.open || g.pl[pl].sel.down_t >= 0.0 { close_selector(g, down); }
        if hit && !g.pl[pl].f_held { g.pl[pl].f_held = true; cancel_sustained(g); }
        else if esc { cancel_sustained(g); }
        else if (js::to_i32(mask) & !js::to_i32(g.pl[pl].sel.move_latch)) != 0 && g.puppet.is_none() { cancel_sustained(g); }
        if wheeled && !down { crate::player::wheel_pace(g, wheel); }
        return;
    }
    g.pl[pl].sel.run_key = String::new();
    g.pl[pl].sel.move_latch = 0.0;
    if g.pl[pl].down {
        close_selector(g, false);
        if wheeled && !down { crate::player::wheel_pace(g, wheel); }
        return;
    }
    // ---- the press edge: armed unless this press is already spent
    let stays = ring_stays(g);
    let free = g.pl[pl].sel.open && ring_free(g);
    if hit && !g.pl[pl].f_held && !free {
        let s = &mut g.pl[pl].sel;
        s.down_t = 0.0; s.open = false; s.moved = false; s.ring = None; s.word = String::new();
    }
    let armed = g.pl[pl].sel.down_t >= 0.0;
    let live = choosable(&g.nearby) || (stays && whistle_wedge(g));
    if g.pl[pl].sel.open && !live { shut_list(g); }
    // ---- THE LIST AS A RING (with HANDRING.on it is the ring that stays: `stays` holds throughout this block)
    if ring_mode(g) {
        if g.pl[pl].sel.open && g.pl[pl].sel.ring.is_some() && (g.orders.menu.is_some() || g.orders.cmd_ring.is_some()) { close_selector(g, down); return; }   // (one list at a time)
        let early = down && armed && !g.pl[pl].sel.open && wedge_digit(g, input);
        if armed && live && !g.pl[pl].sel.open && g.orders.cmd_ring.is_none() && down && (early || g.pl[pl].sel.down_t >= g.k.handring.hold_secs - 1e-6) {
            g.pl[pl].sel.open = true;
            let w = view.map_or(0.0, |v| v.client_width);
            let h = view.map_or(0.0, |v| v.client_height);
            let time = g.time - g.k.cmdring.tap_secs;
            let lit = (g.pl[pl].near_sel + 1) as f64;
            let o = crate::cmdring::ring_open(g, input, w, h, time, Some(lit));
            g.pl[pl].sel.ring = Some(crate::game::PlayerSelRing { ax: o.ax, ay: o.ay, z: o.z, t0: o.t0, lit: o.lit, armed: o.armed, free: o.free, more: o.more, at: o.at, ox: o.ox, oy: o.oy, mx: o.mx, my: o.my, clear: o.clear, pt: o.pt, key: g.pl[pl].near_key.clone(), whistle: None });
            crate::player::pick_hover(g);
            if let Some(v) = view { ring_on_own(g, input, v, Some(true)); }
        }
        if g.pl[pl].sel.ring.is_some() && g.pl[pl].sel.open {
            if crate::input::input_hit(g, input, "Blur") { close_selector(g, true); return; }
            if crate::input::input_rmb_hit(g, input) {
                let i = &mut g.tab.input[input];
                i.clicked = off_bits(i.clicked, 4);
                close_selector(g, true);
                return;
            }
            let lmb = crate::input::input_lmb_hit(g, input);
            let mut digit = 0;
            for d in 1..=9 {
                if g.tab.input[input].pressed.delete(format!("Digit{}", d)) && digit == 0 { digit = d; }   // (not his weapon slots while it is up)
            }
            if g.pl[pl].sel.ring.as_ref().unwrap().key != g.pl[pl].near_key {
                let near_key = g.pl[pl].near_key.clone();
                let ring = g.pl[pl].sel.ring.as_mut().unwrap();
                ring.key = near_key; ring.armed = false; ring.clear = Some(false); ring.whistle = Some(false);
                if ring.at.is_some() {
                    if let Some(v) = view { ring_on_own(g, input, v, Some(true)); }
                }
            }
            let still = match view {
                Some(v) if g.pl[pl].sel.ring.as_ref().unwrap().at.is_some() => ring_on_own(g, input, v, None),
                _ => false,
            };
            let wedges = g.k.cmdring.wedges;
            let keys = hand_ring_keys(g);
            let lone = crate::ringlone::lone_key(g, &mut |_| keys);
            let at_set = g.pl[pl].sel.ring.as_ref().unwrap().at.is_some();
            let time = g.time;
            let shapes = if at_set { keys } else { 0.0 };
            let want0 = lend_ring(g, |g, r| crate::cmdring::ring_want(g, r, input, digit as f64, time, Some(lone), Some(shapes)));
            let pt = g.pl[pl].sel.ring.as_ref().unwrap().pt.unwrap_or(0.0);
            let want = if still && digit == 0 && want0 != pt { 0.0 } else { want0 };
            let wh = whistle_wedge(g);
            let to_wh = wh && want == wedges;
            // (the row: `want && !toWh ? g.nearby[want - 1] : null`, off the list's end = none)
            let at: i32 = if js::truthy(want) && !to_wh && g.nearby.get((want - 1.0) as usize).map_or(false, wedged) { (want - 1.0) as i32 } else { -1 };
            if at_set && (digit != 0 || !still) {
                g.pl[pl].sel.ring.as_mut().unwrap().pt = Some(if digit != 0 { 0.0 } else { want0 });
            }
            if at >= 0 {
                g.pl[pl].sel.ring.as_mut().unwrap().whistle = Some(false);
                if at != g.pl[pl].near_sel { g.pl[pl].near_sel = at; crate::player::pick_hover(g); }
            }
            if to_wh || (wh && !choosable(&g.nearby)) { g.pl[pl].sel.ring.as_mut().unwrap().whistle = Some(true); }
            else if !wh { g.pl[pl].sel.ring.as_mut().unwrap().whistle = Some(false); }
            let whistle = g.pl[pl].sel.ring.as_ref().unwrap().whistle.unwrap_or(false);
            let near_sel = g.pl[pl].near_sel;
            g.pl[pl].sel.ring.as_mut().unwrap().lit = if whistle { wedges } else { (near_sel + 1) as f64 };
            let lit: Option<crate::game::Interactable> = if whistle { None } else { g.hover.clone().filter(wedged) };
            let pointed = digit == 0 && (at >= 0 || to_wh);
            if lmb {   // (the click is the ring's: he does not fire)
                let i = &mut g.tab.input[input];
                i.clicked = off_bits(i.clicked, 1);
                i.buttons = off_bits(i.buttons, 1);
                let seat = g.seat;
                if input == g.tab.cmd_seat[seat].input { crate::cmd::seat_swallow(g, seat, 1.0); }
            }
            let r_free = g.pl[pl].sel.ring.as_ref().unwrap().free.unwrap_or(false);
            let ask = lmb || (digit > 0 && (to_wh || at >= 0)) || (if r_free { hit } else { !down && pointed });
            let can = if whistle { crate::whistle::whistle_ready_in(g) <= 0.0 } else { lit.as_ref().map_or(false, |r| r.in_reach) };
            if ask && can {
                close_selector(g, down);
                if whistle {
                    if !crate::samstun::sam_staggered(g, None) { crate::whistle::try_whistle(g); }
                } else { give(g, lit.as_ref()); }
                return;
            }
            if wheeled && !down { crate::player::wheel_pace(g, wheel); }
            if !r_free && !down {
                g.pl[pl].sel.ring.as_mut().unwrap().free = Some(true);
                g.pl[pl].sel.down_t = -1.0;
            }
            if esc { close_selector(g, true); }
            else if down && !g.pl[pl].sel.ring.as_ref().unwrap().free.unwrap_or(false) { g.pl[pl].sel.down_t += m::min(dt, 0.05); }
            return;
        }
    }
    // ---- the highlight: ↑ / ↓ any time, the wheel while an armed press is down; else the wheel is his pace
    let mut d_sel = 0;
    if g.puppet.is_none() {
        if crate::input::input_hit(g, input, "ArrowDown") { d_sel += 1; }
        if crate::input::input_hit(g, input, "ArrowUp") { d_sel -= 1; }
    }
    if wheeled {
        if stays {
            if !down { crate::player::wheel_pace(g, wheel); }
        } else if armed && live {
            let s = &mut g.pl[pl].sel;
            s.wheel_acc = m::max(-2.0 * WHEEL_STEP, m::min(2.0 * WHEEL_STEP, s.wheel_acc + wheel));
            if m::abs(s.wheel_acc) >= WHEEL_STEP {
                let sg = m::sign(s.wheel_acc);
                d_sel += sg as i32;
                s.wheel_acc -= sg * WHEEL_STEP;
                s.moved = true;
            }
        } else if !down { crate::player::wheel_pace(g, wheel); }
    }
    if !armed { g.pl[pl].sel.wheel_acc = 0.0; }
    if !stays && armed && live && !g.pl[pl].sel.open && (g.pl[pl].sel.moved || (down && g.pl[pl].sel.down_t >= TAP_SECS - 1e-6)) { open_selector(g); }
    if d_sel != 0 && g.nearby.len() > 1 { g.pl[pl].near_sel += d_sel; crate::player::pick_hover(g); }
    let near_sel = g.pl[pl].near_sel;
    if let Some(ring) = g.pl[pl].sel.ring.as_mut() { ring.lit = (near_sel + 1) as f64; }
    // ---- Esc: back out of an open list (or a press not yet a hold): the press is spent, nothing happens on its release
    if esc && armed { close_selector(g, true); return; }
    if !armed {
        if g.pl[pl].sel.open { close_selector(g, false); }
        return;
    }
    if down {
        g.pl[pl].sel.down_t += m::min(dt, 0.05);   // (a frame hitch must not turn a tap into a hold)
        if !stays && live && !g.pl[pl].sel.open && g.pl[pl].sel.down_t >= TAP_SECS - 1e-6 { open_selector(g); }
        return;
    }
    // ---- released this frame (or begun and ended inside it): the highlighted row is what Space does
    let row_now = g.hover.clone();
    let listed = g.pl[pl].sel.open;
    let quick = !g.pl[pl].sel.moved && g.pl[pl].sel.down_t < (if stays { g.k.handring.hold_secs } else { TAP_SECS });
    close_selector(g, false);
    if !listed && row_now.is_none() && g.puppet.is_none() && !crate::samstun::sam_staggered(g, None) {
        let w = empty_word(g, None);
        let time = g.time;
        g.pl[pl].sel.word = w.clone();
        g.pl[pl].sel.word_t = time;
        let file_t = g.pl[pl].sel.file_t;
        if time < file_t || time - file_t > crate::handring::EMPTY_AGAIN_SECS {
            g.pl[pl].sel.file_t = time;
            crate::game::game_msg(g, &w);
        }
        return;
    }
    if quick && crate::ownblow::own_blow_held(g, row_now.as_ref()) { return; }
    give(g, row_now.as_ref());
}

/// (interact.ts:54 `Doors.USE_REACH`, doors.ts:1183) a class's static the skeleton has no entry for
fn seam_doors_use_reach() -> f64 { todo!("needs: Doors.USE_REACH as a const") }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Interactable, InteractableKind};

    fn row(line2: &str) -> Interactable { let mut r = Interactable::zeroed(); r.line2 = String::from(line2); r }

    // (expected strings: what the TypeScript's own rowWords and wedgeWords give, run under bun)
    #[test]
    fn words_of_a_row() {
        for (line2, rw, ww) in [
            ("Space push", "push", "push"),
            ("Space   grab · 2 m", "grab", "grab"),
            ("Spacer push", "Spacer push", "Spacer push"),
            ("Space\tcharge — too close: you · x", "charge — too close: you", "charge"),
            ("  pick lock - slow · 3 ", "pick lock - slow", "pick lock"),
            ("well-made · dash", "well-made", "well-made"),
            ("a ·b · c", "a ·b", "a ·b"),
            ("a –  b", "a –  b", "a"),
            ("", "", ""),
        ] {
            assert_eq!(row_words(&row(line2)), rw, "{:?}", line2);
            assert_eq!(wedge_words(&row(line2)), ww, "{:?}", line2);
        }
    }

    #[test]
    fn what_has_a_wedge() {
        let mut r = Interactable::zeroed();
        assert!(!wedged(&r));   // a status line
        r.verb = Some(crate::game::Verb::Push);
        assert!(wedged(&r) && choosable(core::slice::from_ref(&r)));
        r.running = Some(true);
        assert!(!wedged(&r) && !choosable(core::slice::from_ref(&r)));
        r.running = Some(false);
        r.kind = InteractableKind::Cancel;
        assert!(!wedged(&r) && !choosable(core::slice::from_ref(&r)));
    }
}
