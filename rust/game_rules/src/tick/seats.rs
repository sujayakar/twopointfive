#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::seats::types;
pub use types::*;
pub use ::game_types::seats::seat_ref::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// the row a second cast of the squad's code keeps in a table kept per view
pub const CAST_KEY: PlayerId = PlayerId(u32::MAX);

/// is this call inside a seat's view, and not on the bare tick. with_seat raises Game.view_depth for the length of its call
fn in_view(g: &crate::game::Game) -> bool { g.view_depth > 0 }

/// INVENTED BY THE SKELETON: which row of a table kept per view this pass reads: the man of the seat the pass is about, or CAST_KEY inside a second cast.
/// (seats.ts SEAT_TABLES key by the view; the bridge writes a view's key as `view.player`, export.ts:83)
pub fn view_key(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> PlayerId {
    if g.in_cast { CAST_KEY } else { seat.player }
}

/// The pass is about this seat from here on. Its man is on `g.player` for the pass; the man of the seat it was about goes back to its row.
fn stand_on(g: &mut Game, s: SeatId) {
    g.sets[g.seat].player = g.player;
    g.seat = s;
    g.player = g.sets[s].player;
}

/// A PASS ABOUT ANOTHER SEAT. The seat it was about comes back after; the seat already about runs as it stands (so a nesting is fine). For the length of the call the
/// game is "a seat's view" (`in_view`), the first seat's too.
pub fn with_seat<R>(g: &mut crate::game::Game, seat: SeatId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    let from = g.seat;
    stand_on(g, seat);
    g.view_depth += 1;
    let r = f(g);
    g.view_depth -= 1;
    stand_on(g, from);
    r
}

/// WHICH SEAT THIS MACHINE SHOWS, from now on: a host's own fact, told after a start and whenever it changes. The rows of a game are numbered the same on every
/// machine (a start stands with the first seat), and this only says which seat `g.seat` names between ticks, for the page to read. It changes nothing of the play:
/// a tick is about the first seat whichever seat is shown (`on_first_bare`).
pub fn show_seat(g: &mut crate::game::Game, seat: SeatId) {
    assert!(g.seats.contains(&seat), "a host asks to show a seat this game has not got: {:?}", seat);
    g.local_seat = Some(seat);
    stand_on(g, seat);
}

/// THE BARE GAME FOR THE LENGTH OF A TICK: the first seat and no view open (seats.ts seatsStand: inside a tick `g.seat` answers the FIRST seat). Where another seat was
/// shown (a host that shows the second player), it comes back after.
pub(super) fn on_first_bare<R>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    let from = g.seat;
    stand_on(g, g.seats[0]);
    let r = f(g);
    stand_on(g, from);
    r
}

/// HOW MANY PLAYERS' MEN STAND ON THE FLOOR: one a seat. Every budget of characters counts this, never "1" (a floor holds at most
/// guards::MAX_SKINNED characters with every seat counted: the seats' men, every teammate, every other man stood).
pub fn seat_count(g: &crate::game::Game) -> f64 { g.seats.len() as f64 }

/// A BUDGET OF CHARACTERS WRITTEN FOR ONE PLAYER, with every seat counted: one more for each further seat. The ONE rule every staging's budget goes
/// through (the eight a caller gets without asking, the four's ten, a structure's, a rescue's cast); a game of one reads the number as written.
/// spawn_unit holds any budget to guards::MAX_SKINNED; past that a staging cuts its defenders (floor_room).
pub fn budget_with_seats(g: &crate::game::Game, for_one: f64) -> f64 { for_one - 1.0 + seat_count(g) }

/// How many more men the floor can be given as it stands: the picture's sixteen less a man a seat and less every guard, teammate and unit.
/// A staging stands its team first and then as many of its defenders, in the order of their marks, as this leaves (a game of one: all of them).
pub fn floor_room(g: &crate::game::Game) -> f64 { crate::guards::MAX_SKINNED - seat_count(g) - g.guards.len() as f64 }

// ---------------------------------------------------------------- the seams: what skeleton 5 still does not have

/// seats.ts:213 `rawGame(g) !== g` with the pass about the first seat: a pass over his view and a pass over the bare Game look alike here
fn seam_in_first_seat_view(g: &Game) -> bool { crate::seats::in_view(g) }

// ---------------------------------------------------------------- small helpers

/// a later seat's record as it is stood: what the first seat's holds now, with his seat
fn set_of_game(g: &Game, s: SeatId) -> crate::seats::SeatSet {
    crate::seats::SeatSet { seat: s, player: g.player, ..g.sets[g.seat].clone() }
}
/// `S[i ?? 0] ?? S[0]`
fn seat_at(g: &Game, i: Option<f64>) -> SeatId {
    g.seats.get(i.unwrap_or(0.0) as usize).copied().unwrap_or(g.seats[0])
}
/// `!!gd.unit && gd.team === 'operative'`
fn on_team(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].unit.is_some() && g.gd[gd].team == crate::teams::Team::Operative
}
/// `gd.unit?.seat ?? 0`
fn unit_seat(g: &Game, gd: GuardId) -> f64 {
    g.gd[gd].unit.as_ref().and_then(|u| u.seat).unwrap_or(0.0)
}
/// the character id a player's man carries
fn man_char_id(g: &Game, p: PlayerId) -> f64 { g.ch[g.pl[p].ch].id }

/// a seat's part of the squad's `n`: dealt in roster order so the shares add up to the count
fn share_of(n: f64, lo: f64, mine: f64, all: f64) -> f64 {
    m::floor(n * (lo + mine) / all) - m::floor(n * lo / all)
}

// ---------------------------------------------------------------- the seats

/// the game's seats, made as its constructor begins, each with its row of `g.sets`
pub(super) fn seats_stand(g: &mut crate::game::Game, n: f64) -> Vec<SeatId> {
    let max = g.k.seats.max;
    if !(n >= 1.0 && n <= max) || n != m::floor(n) { panic!("a game is stood up with 1 to {} seats, not {}", max, n); }
    if n == 1.0 {
        let s = crate::cmd::seat_new(g, Some(0.0));
        g.seat = s;
        return vec![s];
    }
    let mut seats: Vec<SeatId> = Vec::new();
    let mut i = 0.0;
    while i < n {
        let s = crate::cmd::seat_new(g, Some(i));
        if i != 0.0 {
            let own = set_of_game(g, s);
            assert_eq!(g.sets.push(own), s, "a seat's set stands at its seat's id");
        }
        seats.push(s);
        i += 1.0;
    }
    g.seat = seats[0];
    seats
}

// ---------------------------------------------------------------- the men

/// the character id of seat `i`'s man
pub(super) fn seat_man_id(i: f64) -> f64 {
    if i == 0.0 { crate::consts::PLAYER_ID } else { crate::consts::NOBODY_ID + i }
}

/// is `id` a player's man's
pub fn is_man_id(g: &crate::game::Game, id_2: f64) -> bool {
    id_2 == crate::consts::PLAYER_ID || (id_2 > crate::consts::NOBODY_ID && id_2 < crate::consts::NOBODY_ID + g.k.seats.max)
}

/// every seat's man, in seat order
pub fn men(g: &crate::game::Game) -> Vec<PlayerId> {
    if g.seats.len() == 1 { return vec![g.player]; }
    let mut out = Vec::new();
    for i in 0..g.seats.len() { let s = g.seats[i]; out.push(crate::cmd::seat_man(g, s)); }
    out
}

/// the men still up
pub fn men_up(g: &crate::game::Game) -> Vec<PlayerId> {
    let mut all = men(g);
    all.retain(|p| !crate::condition::down(g, g.pl[*p].ch));
    all
}

/// is every player down
pub fn all_down(g: &crate::game::Game) -> bool {
    if g.seats.len() == 1 { return crate::condition::down(g, g.pl[g.player].ch); }
    for i in 0..g.seats.len() {
        let p = crate::cmd::seat_man(g, g.seats[i]);
        if !crate::condition::down(g, g.pl[p].ch) { return false; }
    }
    true
}

/// the men a rule about the team goes by: those still up, or every one if none is (a game of one asks about its one man, down or up)
pub fn men_left(g: &crate::game::Game) -> Vec<PlayerId> {
    let all = men(g);
    let up: Vec<PlayerId> = all.iter().copied().filter(|p| !crate::condition::down(g, g.pl[*p].ch)).collect();
    if up.is_empty() { all } else { up }
}

/// the man up nearest a place, or the first if every one is down
pub fn nearest_or_first(g: &crate::game::Game, at: &[f64]) -> PlayerId {
    nearest_man(g, at).unwrap_or_else(|| crate::cmd::seat_man(g, g.seats[0]))
}

/// `f` standing on the seat this man's senses and doings are about: his own, or the one he faces (update_guard's own way, for whoever else runs a man)
pub fn standing_for<R>(g: &mut crate::game::Game, gd: GuardId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    if g.seats.len() == 1 { return f(g); }
    let v = view_for(g, gd);
    with_seat(g, v, f)
}

/// `f` with this seat standing; in a game of one, or for no seat, as things stand
pub fn on_seat<R>(g: &mut crate::game::Game, s: Option<SeatId>, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    match s {
        Some(s) if g.seats.len() > 1 => with_seat(g, s, f),
        _ => f(g),
    }
}

/// can a mission of this kind be played by two
pub fn seats_kind(kind: &str) -> bool { crate::seats::seat_kinds().iter().any(|k| k == kind) }

/// why this game cannot start a mission of this kind
pub fn seats_refuse(g: &crate::game::Game, kind: &str) -> Option<String> {
    if g.seats.len() > 1 && !seats_kind(kind) { Some(String::from("it takes one player today")) } else { None }
}

/// the man whose eyes and ears this host's picture and speakers are
pub fn host_man(g: &crate::game::Game) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    let s = crate::game::game_host_seat(g);
    crate::cmd::seat_man(g, s)
}

/// the man a writer with no man of its own names: the view's; bare, the pass is about the first seat, so both arms are g.player
pub fn man_here(seat: &crate::seats::SeatRef) -> PlayerId { seat.player }

/// `f` for every seat, over its view
pub fn each_seat(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) {
    if g.seats.len() == 1 { f(g); return; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        crate::seats::with_seat(g, s, &mut *f);
    }
}

/// the first answer any seat's view gives `f`
pub fn first_of<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> Option<T>) -> Option<T> {
    if g.seats.len() == 1 { return f(g); }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let r = crate::seats::with_seat(g, s, &mut *f);
        if r.is_some() { return r; }
    }
    None
}

/// the seat whose man this character is
pub fn seat_of_char(g: &crate::game::Game, c_2: Option<CharId>) -> Option<SeatId> {
    let c = c_2?;
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if g.pl[p].ch == c { return Some(s); }
    }
    None
}

/// … and the man himself
pub fn man_of(g: &mut crate::game::Game, c_2: Option<CharId>) -> Option<PlayerId> {
    if g.seats.len() == 1 {
        return match c_2 { Some(c) if g.pl[g.player].ch == c => Some(g.player), _ => None };
    }
    let s = seat_of_char(g, c_2)?;
    Some(crate::cmd::seat_man(g, s))
}

/// the seat whose man carries character id `id`
pub fn seat_of_id(g: &crate::game::Game, id_2: f64) -> Option<SeatId> {
    if !is_man_id(g, id_2) { return None; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if man_char_id(g, p) == id_2 { return Some(s); }
    }
    None
}

/// the man who carries character id `id`
pub fn man_by_id(g: &crate::game::Game, id_2: f64) -> Option<PlayerId> {
    if g.seats.len() == 1 {
        return if man_char_id(g, g.player) == id_2 { Some(g.player) } else { None };
    }
    let s = seat_of_id(g, id_2)?;
    Some(crate::cmd::seat_man(g, s))
}

/// the game as the player with this id sees it, None for no player's id
pub fn view_of_id(g: &crate::game::Game, id_2: f64) -> Option<SeatId> {
    if g.seats.len() == 1 {
        return if man_char_id(g, g.player) == id_2 { Some(g.seat) } else { None };
    }
    seat_of_id(g, id_2)
}

/// the seat of a man
pub fn seat_of_man(g: &crate::game::Game, pl: PlayerId) -> SeatId {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if crate::cmd::seat_man(g, s) == pl { return s; }
    }
    g.seats[0]
}

/// what a player is called
pub fn player_name(g: &crate::game::Game, pl: PlayerId) -> String {
    if g.seats.len() == 1 { return String::from("Player"); }
    let s = seat_of_man(g, pl);
    alloc::format!("Player {}", g.tab.cmd_seat[s].index + 1.0)
}

/// the living man nearest a place
pub fn nearest_man(g: &crate::game::Game, at: &[f64]) -> Option<PlayerId> {
    let (mut best, mut bd) = (None, f64::INFINITY);
    for i in 0..g.seats.len() {
        let p = crate::cmd::seat_man(g, g.seats[i]);
        if crate::condition::down(g, g.pl[p].ch) { continue; }
        let pos = g.ch[g.pl[p].ch].pos;
        let d = m::hyp2(pos[0] - at[0], pos[2] - at[2]);
        if d < bd { bd = d; best = Some(p); }
    }
    best
}

/// a later seat's man and his light, made as the Game's constructor ends
pub(super) fn stand_men(g: &mut crate::game::Game) {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let index = g.tab.cmd_seat[s].index;
        if !js::truthy(index) { continue; }
        let spawn = v3::copy(g.level.player_spawn);
        let at = spot_beside(g, spawn, index);
        let fn2000 = g.fn2000;
        let (men, rig) = (g.men, g.rig);
        let p = crate::game::player_new(g, men, rig, at, Some(fn2000), Some(index));
        crate::seats::with_seat(g, s, &mut |g: &mut Game| g.player = p);
        let (owner, intensity) = (man_char_id(g, p), g.tune.player_light);
        let def = crate::rig_props::rail_light_def(g, owner, intensity);
        g.pl[p].light = crate::render_lights::light_set_add(g, def);
    }
}

/// where seat `i`'s man stands when the first stands on `at`
pub(super) fn spot_beside(g: &mut crate::game::Game, at: V3, i: f64) -> V3 {
    if !js::truthy(i) { return at; }
    let apart = g.k.seats.apart;
    for k in 0..8 {
        let a = m::PI / 2.0 + (k as f64) * m::PI / 4.0;
        let p: V3 = [at[0] + m::cos(a) * apart * i, at[1], at[2] + m::sin(a) * apart * i];
        let mut q = v3::copy(p);
        crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, Some(1.0));
        if m::hyp2(q[0] - p[0], q[2] - p[2]) < 0.02
            && !crate::scene_collision::clamp_to_world(&mut q, 0.3)
            && !crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [p[0], 0.45, p[2]], None) { return p; }
    }
    v3::copy(at)
}

/// a staging puts the players on its start
pub fn stage_men(g: &mut crate::game::Game, at: V3, yaw: f64, each: Option<&mut dyn FnMut(&mut crate::game::Game, PlayerId)>) {
    let mut each = each;
    deal_seats(g);
    let mut taken: Vec<V3> = vec![at];
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !crate::condition::down(g, g.gd[gd].ch) { taken.push(g.ch[g.gd[gd].ch].pos); }
    }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let index = g.tab.cmd_seat[s].index;
        let mut p = at;
        if js::truthy(index) {
            let mut lead = None;
            for j in 0..g.guards.len() {
                let gd = g.guards[j];
                if on_team(g, gd) && g.gd[gd].unit.as_ref().and_then(|u| u.seat) == Some(index) { lead = Some(gd); break; }
            }
            p = match lead {
                Some(gd) => {
                    let c = g.gd[gd].ch;
                    let (pos, body_yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
                    crate::nightbreach::night_breach_stand_by(g, pos, &taken, Some(body_yaw))
                }
                None => spot_beside(g, at, index),
            };
            taken.push(p);
        }
        crate::seats::with_seat(g, s, &mut |g: &mut Game| {
            crate::game::game_teleport_player(g, v3::copy(p));
            let man = g.player;
            let c = g.pl[man].ch;
            g.ch[c].aim_yaw = yaw;
            g.ch[c].body_yaw = yaw;
            if let Some(f) = each.as_mut() { f(g, man); }
        });
    }
}

/// a scenario's opening cut: every seat's lens snapped to its own man
pub fn cut_to_men(g: &mut crate::game::Game) {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let at = crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::game::game_follow_pos(g, None));
        let cam = g.tab.cmd_seat[s].cam;
        let c = &mut g.tab.camera[cam];
        c.target = v3::copy(at);
        c.desired = v3::copy(c.target);
        c.desired_serial = c.desired_serial.wrapping_add(1);
    }
}

// ---------------------------------------------------------------- who an enemy is up against

/// the man an enemy has in mind
fn faced_seat(g: &mut crate::game::Game, gd: GuardId) -> SeatId {
    let cur = seat_at(g, g.gd[gd].faces);
    let cur_man = crate::cmd::seat_man(g, cur);
    if crate::teams::foe_is_player(g, gd) && !crate::condition::down(g, g.pl[cur_man].ch) { return cur; }
    let (mut best, mut bd) = (cur, f64::INFINITY);
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if crate::condition::down(g, g.pl[p].ch) { continue; }
        let keep = if s == cur { g.k.seats.keep } else { 0.0 };
        let d = v3::dist_xz(g.ch[g.pl[p].ch].pos, g.ch[g.gd[gd].ch].pos) - keep;
        if d < bd { bd = d; best = s; }
    }
    if best != cur { g.gd[gd].faces = Some(g.tab.cmd_seat[best].index); }
    // (the player he fought is down: his eyes go to the one they are on now)
    if crate::teams::foe_is_player(g, gd) {
        let c = g.pl[crate::cmd::seat_man(g, best)].ch;
        crate::mind::foe(g, gd, Some(c), "seats");
    }
    best
}

/// that player's man, by his body
pub fn faced(g: &crate::game::Game, gd: GuardId) -> CharId { g.pl[faced_man(g, gd)].ch }

/// the player an enemy has in mind as the tick stands
pub fn faced_man(g: &crate::game::Game, gd: GuardId) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    let s = seat_at(g, g.gd[gd].faces);
    crate::cmd::seat_man(g, s)
}

/// the game as that player sees it
pub fn faced_view(g: &crate::game::Game, gd: GuardId) -> SeatId {
    if g.seats.len() == 1 { return g.seat; }
    seat_at(g, g.gd[gd].faces)
}

/// the game as the man behind this body has it; never moves `faces`
pub fn view_of_body(g: &crate::game::Game, c_2: crate::seats::ViewOfBodyC) -> SeatId {
    if g.seats.len() == 1 || g.seat != g.seats[0] || seam_in_first_seat_view(g) { return g.seat; }
    if let Some(seat) = view_of_id(g, c_2.id) { return seat; }
    let mut found = None;
    for i in 0..g.guards.len() {
        let x = g.guards[i];
        if g.ch[g.gd[x].ch].id == c_2.id { found = Some(x); break; }
    }
    match found {
        None => g.seats[0],
        Some(gd) if on_team(g, gd) => seat_of(g, gd),
        Some(gd) => faced_view(g, gd),
    }
}

/// the view a man of the floor is stepped over: his seat's for a teammate, the faced man's for anybody else
pub fn view_for(g: &mut crate::game::Game, gd: GuardId) -> SeatId {
    if on_team(g, gd) { seat_of(g, gd) } else { faced_seat(g, gd) }
}

// ---------------------------------------------------------------- whose teammate

/// the deal: the team's roster is cut into as many runs as there are seats; returns whether it dealt
pub fn deal_seats(g: &mut crate::game::Game) -> bool {
    let n = g.seats.len();
    if n == 1 { return false; }
    let last = (n - 1) as f64;
    let mut fresh = 0usize;
    let mut roster: Vec<GuardId> = Vec::new();
    let mut has: Vec<i32> = vec![0; n];
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !on_team(g, gd) { continue; }
        roster.push(gd);
        match g.gd[gd].unit.as_ref().and_then(|u| u.seat) {
            None => fresh += 1,
            Some(seat) => has[m::min(last, seat) as usize] += 1,
        }
    }
    if fresh == 0 { return false; }
    let k = roster.len();
    let per = m::ceil(k as f64 / n as f64);
    if fresh == k {
        for (i, gd) in roster.iter().enumerate() {
            let seat = m::min(last, m::floor(i as f64 / per));
            g.gd[*gd].unit.as_mut().unwrap().seat = Some(seat);
        }
    } else {
        for gd in roster.iter() {
            if g.gd[*gd].unit.as_ref().unwrap().seat.is_some() { continue; }
            let mut to = 0;
            for i in 1..n { if has[i] < has[to] { to = i; } }
            g.gd[*gd].unit.as_mut().unwrap().seat = Some(to as f64);
            has[to] += 1;
        }
    }
    true
}

/// the seat a teammate takes his orders from
pub fn seat_of(g: &crate::game::Game, gd: GuardId) -> SeatId {
    let at = g.gd[gd].unit.as_ref().and_then(|u| u.seat);
    seat_at(g, at)
}

/// is this teammate `s`'s to order
pub fn in_seat(g: &crate::game::Game, s: SeatId, gd: GuardId) -> bool {
    unit_seat(g, gd) == g.tab.cmd_seat[s].index
}

/// the game handed in has one squad's orders, the seat's
pub fn one_book(g: &crate::game::Game) -> bool {
    g.seats.len() == 1 || g.in_cast
}

/// the game as a teammate's own player sees it
pub fn view_of(g: &crate::game::Game, gd: GuardId) -> SeatId {
    if one_book(g) { return g.seat; }
    seat_of(g, gd)
}

/// the orders that move a teammate: his seat's book, named by its man
pub fn orders_of(g: &crate::game::Game, gd: GuardId) -> PlayerId {
    if one_book(g) { return g.player; }
    let s = seat_of(g, gd);
    crate::cmd::seat_man(g, s)
}

/// the order he is under, in whichever seat's book
pub fn order_of(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> {
    if one_book(g) { return crate::orders::squad_orders_of(&crate::seats::here(g), gd); }
    let s = seat_of(g, gd);
    crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_of(&crate::seats::here(g), gd))
}

/// is there a squad to order at all, any seat's
pub fn team_active(g: &mut crate::game::Game) -> bool {
    if one_book(g) { return crate::orders::squad_orders_active(g, &crate::seats::here(g)); }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_active(g, &crate::seats::here(g))) { return true; }
    }
    false
}

/// every seat's teammates, in the guard list's order
pub fn team_squad(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    if one_book(g) { return crate::orders::squad_orders_squad(g, seat); }
    let mut out = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if on_team(g, gd) { out.push(gd); }
    }
    out
}

/// … and the living of them
pub fn team_men(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    if one_book(g) { return crate::orders::squad_orders_men(g, seat, None, None); }
    let mut out = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if on_team(g, gd) && !crate::condition::down(g, g.gd[gd].ch) { out.push(gd); }
    }
    out
}

/// a seat's share of what the squad brings, for the seat the pass is about
pub fn seat_share(g: &mut crate::game::Game, n: f64) -> f64 {
    if one_book(g) { return n; }
    let me = g.tab.cmd_seat[g.seat].index;
    let (mut lo, mut mine, mut all) = (0.0, 0.0, 0.0);
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !on_team(g, gd) { continue; }
        let s = unit_seat(g, gd);
        all += 1.0;
        if s < me { lo += 1.0; } else if s == me { mine += 1.0; }
    }
    if all == 0.0 { return if me == 0.0 { n } else { 0.0 }; }
    share_of(n, lo, mine, all)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_man_s_ids() {
        assert_eq!(seat_man_id(0.0), crate::consts::PLAYER_ID);
        assert_eq!(seat_man_id(1.0), crate::consts::NOBODY_ID + 1.0);
    }

    #[test]
    fn only_the_two_kinds_seat_two() {
        assert!(seats_kind("roomclear") && seats_kind("nightbreach") && !seats_kind("rescue"));
    }

    #[test]
    fn the_shares_add_up_to_the_count() {
        for n in 0..12 {
            let n = n as f64;
            // a squad of four cut in two pairs, then three cut two and one
            assert_eq!(share_of(n, 0.0, 2.0, 4.0) + share_of(n, 2.0, 2.0, 4.0), n);
            assert_eq!(share_of(n, 0.0, 2.0, 3.0) + share_of(n, 2.0, 1.0, 3.0), n);
        }
        assert_eq!(share_of(3.0, 0.0, 2.0, 4.0), 1.0);
        assert_eq!(share_of(3.0, 2.0, 2.0, 4.0), 2.0);
    }

    #[test]
    fn a_seat_number_prints_as_the_typescript_s() {
        assert_eq!(alloc::format!("Player {}", 1.0_f64 + 1.0), "Player 2");
    }
}
