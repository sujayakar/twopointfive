//! src/game/seats.ts lines 1 to 273
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the tones in seat order (seats.ts SEAT_TONES; the generated `seat_tones()` holds them as words, and this answers the enum)
const SEAT_TONES: [SeatTone; 2] = [SeatTone::You, SeatTone::Purple];

// ---------------------------------------------------------------- the seams: what skeleton 3 still does not have

/// seats.ts:213 `rawGame(g) !== g` with the first seat on Game: a pass over his view and a pass over the bare Game look alike here
fn seam_in_first_seat_view(g: &mut Game) -> bool { todo!("needs: a word on Game set by with_seat, for 'inside a seat's view'") }
/// seats.ts:129: `f(view)`; the stub's closure takes `g` twice, which cannot be lent
fn seam_call_view(g: &mut Game, s: Option<SeatId>, f: &mut dyn FnMut(&mut Game, &mut Game)) { todo!("needs: each_seat's f taking one &mut Game") }
/// seats.ts:132: the same for first_of
fn seam_ask_view<T>(g: &mut Game, s: Option<SeatId>, f: &mut dyn FnMut(&mut Game, &mut Game) -> Option<T>) -> Option<T> { todo!("needs: first_of's f taking one &mut Game") }

// ---------------------------------------------------------------- small helpers

/// `f` over seat `s`'s view. The seat on Game needs no swap: a game of one's, and the first between passes
fn over_seat<R>(g: &mut Game, s: SeatId, f: &mut dyn FnMut(&mut Game) -> R) -> R {
    if g.seat == s { return f(g); }
    let man = crate::cmd::seat_man(g, s);
    crate::seats::with_seat(g, man, f)
}
/// a later seat's record as it is stood: what Game's fields of those names hold now, with his seat (seats.ts:86-87)
fn set_of_game(g: &Game, s: SeatId) -> crate::seats::SeatSet {
    crate::seats::SeatSet {
        seat: s,
        player: g.player,
        orders: g.orders.clone(),
        door_tools: g.door_tools.clone(),
        work: g.work.clone(),
        wall_charge: g.wall_charge.clone(),
        aim_point: g.aim_point,
        aim_target: g.aim_target,
        aim_guard: g.aim_guard,
        aim_friend: g.aim_friend,
        aim_snap: g.aim_snap,
        hover: g.hover.clone(),
        interactables: g.interactables.clone(),
        nearby: g.nearby.clone(),
        act: g.act.clone(),
        use_door: g.use_door,
        view_proj: g.view_proj.clone(),
        threat_proximity: g.threat_proximity,
        torch_on_t: g.torch_on_t,
    }
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

/// the count of `seats=<digits>` in an address, by the regex `(?:^|[?&])seats=(\d+)(?:&|$)`; 1 with no such word
fn seats_word(search: &str) -> f64 {
    let b = search.as_bytes();
    const WORD: &[u8] = b"seats=";
    let mut i = 0;
    while i + WORD.len() <= b.len() {
        if &b[i..i + WORD.len()] == WORD && (i == 0 || b[i - 1] == b'?' || b[i - 1] == b'&') {
            let (mut j, mut n) = (i + WORD.len(), 0.0);
            while j < b.len() && b[j].is_ascii_digit() { n = n * 10.0 + (b[j] - b'0') as f64; j += 1; }
            if j > i + WORD.len() && (j == b.len() || b[j] == b'&') { return n; }
        }
        i += 1;
    }
    1.0
}
/// a seat's part of the squad's `n`: dealt in roster order so the shares add up to the count
fn share_of(n: f64, lo: f64, mine: f64, all: f64) -> f64 {
    m::floor(n * (lo + mine) / all) - m::floor(n * lo / all)
}

// ---------------------------------------------------------------- the seats

/// how many seats an address asks for (seats.ts seatsOfSearch)
pub fn seats_of_search(g: &mut crate::game::Game, search: Option<&str>) -> f64 {
    let n = seats_word(search.unwrap_or(""));
    m::max(1.0, m::min(g.k.seats.max, js::or(n, 1.0)))
}

/// a seat's view of the game (seats.ts seatView). The Proxy has no Rust form: with_seat swaps the record onto Game, and a view is named by its man
pub fn seat_view(g: &mut crate::game::Game, own: &mut crate::seats::SeatSet) -> PlayerId { own.player }

/// the game's seats, made as its constructor begins (seats.ts seatsStand). More than one: the first seat's set is Game's own fields, a later seat's is in `Seat.own`
pub fn seats_stand(g: &mut crate::game::Game, n: f64) -> Vec<SeatId> {
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
            g.tab.cmd_seat[s].own = Some(own);
        }
        seats.push(s);
        i += 1.0;
    }
    g.seat = seats[0];
    seats
}

/// the Game's own field of a seat's name was asked in a game of more than one seat (seats.ts miss)
pub fn miss(g: &mut crate::game::Game, k: &str) {
    if g.m.seats.seat_asks.strict {
        panic!("seats: the Game's own '{}' was asked in a game of more than one seat (a player's code is handed his seat's view; the world asks men(g) / manOf / ordersOf)", k);
    }
    g.m.seats.seat_asks.misses += 1.0;
    g.m.seats.seat_asks.missed.add(String::from(k));
}

/// the Game a view looks at (seats.ts rawGame): there is one Game, and bare it holds the first seat's set, so it is named by his man
pub fn raw_game(g: &mut crate::game::Game) -> PlayerId {
    let first = g.seats[0];
    crate::cmd::seat_man(g, first)
}

// ---------------------------------------------------------------- the men

/// the character id of seat `i`'s man (seats.ts seatManId)
pub fn seat_man_id(i: f64) -> f64 {
    if i == 0.0 { crate::consts::PLAYER_ID } else { crate::consts::NOBODY_ID + i }
}

/// is `id` a player's man's (seats.ts isManId)
pub fn is_man_id(g: &mut crate::game::Game, id_2: f64) -> bool {
    id_2 == crate::consts::PLAYER_ID || (id_2 > crate::consts::NOBODY_ID && id_2 < crate::consts::NOBODY_ID + g.k.seats.max)
}

/// every seat's man, in seat order (seats.ts men)
pub fn men(g: &mut crate::game::Game, g_2: &crate::seats::MenG) -> Vec<PlayerId> {
    if g_2.seats.len() == 1 { return vec![g_2.player]; }
    let mut out = Vec::new();
    for i in 0..g_2.seats.len() { out.push(crate::cmd::seat_man(g, g_2.seats[i])); }
    out
}

/// the men still up (seats.ts menUp)
pub fn men_up(g: &mut crate::game::Game) -> Vec<PlayerId> {
    let of = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let mut all = men(g, &of);
    all.retain(|p| !g.pl[*p].down);
    all
}

/// is every player down (seats.ts allDown)
pub fn all_down(g: &mut crate::game::Game) -> bool {
    if g.seats.len() == 1 { return g.pl[g.player].down; }
    for i in 0..g.seats.len() {
        let p = crate::cmd::seat_man(g, g.seats[i]);
        if !g.pl[p].down { return false; }
    }
    true
}

/// is any player down (seats.ts anyDown)
pub fn any_down(g: &mut crate::game::Game) -> bool {
    if g.seats.len() == 1 { return g.pl[g.player].down; }
    for i in 0..g.seats.len() {
        let p = crate::cmd::seat_man(g, g.seats[i]);
        if g.pl[p].down { return true; }
    }
    false
}

/// can a mission of this kind be played by two (seats.ts seatsKind)
pub fn seats_kind(kind: &str) -> bool { crate::seats::seat_kinds().iter().any(|k| k == kind) }

/// why this game cannot start a mission of this kind (seats.ts seatsRefuse)
pub fn seats_refuse(g: &mut crate::game::Game, kind: &str) -> Option<String> {
    if g.seats.len() > 1 && !seats_kind(kind) { Some(String::from("it takes one player today")) } else { None }
}

/// the man whose eyes and ears this host's picture and speakers are (seats.ts hostMan)
pub fn host_man(g: &mut crate::game::Game) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    let s = crate::game::game_host_seat(g);
    crate::cmd::seat_man(g, s)
}

/// the man a writer with no man of its own names (seats.ts manHere): the view's; bare, Game holds the first seat's set, so both arms are g.player
pub fn man_here(g: &mut crate::game::Game) -> PlayerId { g.player }

/// `f` for every seat, over its view (seats.ts eachSeat)
pub fn each_seat(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game, &mut crate::game::Game)) {
    if g.seats.len() == 1 { seam_call_view(g, None, f); return; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        seam_call_view(g, Some(s), f);
    }
}

/// the first answer any seat's view gives `f` (seats.ts firstOf)
pub fn first_of<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game, &mut crate::game::Game) -> Option<T>) -> Option<T> {
    if g.seats.len() == 1 { return seam_ask_view(g, None, f); }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let r = seam_ask_view(g, Some(s), f);
        if r.is_some() { return r; }
    }
    None
}

/// the seat whose man this character is (seats.ts seatOfChar)
pub fn seat_of_char(g: &mut crate::game::Game, c_2: Option<CharId>) -> Option<SeatId> {
    let c = c_2?;
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if g.pl[p].ch == c { return Some(s); }
    }
    None
}

/// … and the man himself (seats.ts manOf)
pub fn man_of(g: &mut crate::game::Game, c_2: Option<CharId>) -> Option<PlayerId> {
    if g.seats.len() == 1 {
        return match c_2 { Some(c) if g.pl[g.player].ch == c => Some(g.player), _ => None };
    }
    let s = seat_of_char(g, c_2)?;
    Some(crate::cmd::seat_man(g, s))
}

/// the seat whose man carries character id `id` (seats.ts seatOfId)
pub fn seat_of_id(g: &mut crate::game::Game, id_2: f64) -> Option<SeatId> {
    if !is_man_id(g, id_2) { return None; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if man_char_id(g, p) == id_2 { return Some(s); }
    }
    None
}

/// the man who carries character id `id` (seats.ts manById)
pub fn man_by_id(g: &mut crate::game::Game, id_2: f64) -> Option<PlayerId> {
    if g.seats.len() == 1 {
        return if man_char_id(g, g.player) == id_2 { Some(g.player) } else { None };
    }
    let s = seat_of_id(g, id_2)?;
    Some(crate::cmd::seat_man(g, s))
}

/// the game as the player with this id sees it, None for no player's id (seats.ts viewOfId)
pub fn view_of_id(g: &mut crate::game::Game, id_2: f64) -> Option<PlayerId> { man_by_id(g, id_2) }

/// the seat of a man (seats.ts seatOfMan)
pub fn seat_of_man(g: &mut crate::game::Game, pl: PlayerId) -> SeatId {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if crate::cmd::seat_man(g, s) == pl { return s; }
    }
    g.seats[0]
}

/// which tone a seat's man has (seats.ts seatTone)
pub fn seat_tone(seat: f64) -> crate::seats::SeatTone {
    SEAT_TONES[js::rem(seat, SEAT_TONES.len() as f64) as usize]
}

/// what a player is called (seats.ts playerName)
pub fn player_name(g: &mut crate::game::Game, pl: PlayerId) -> String {
    if g.seats.len() == 1 { return String::from("Player"); }
    let s = seat_of_man(g, pl);
    alloc::format!("Player {}", g.tab.cmd_seat[s].index + 1.0)
}

/// the living man nearest a place (seats.ts nearestMan)
pub fn nearest_man(g: &mut crate::game::Game, at: &[f64]) -> Option<PlayerId> {
    let (mut best, mut bd) = (None, f64::INFINITY);
    for i in 0..g.seats.len() {
        let p = crate::cmd::seat_man(g, g.seats[i]);
        if g.pl[p].down { continue; }
        let pos = g.ch[g.pl[p].ch].pos;
        let d = m::hyp2(pos[0] - at[0], pos[2] - at[2]);
        if d < bd { bd = d; best = Some(p); }
    }
    best
}

/// a later seat's man and his light, made as the Game's constructor ends (seats.ts standMen)
pub fn stand_men(g: &mut crate::game::Game) {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let index = g.tab.cmd_seat[s].index;
        if !js::truthy(index) { continue; }
        let spawn = v3::copy(g.level.player_spawn);
        let at = spot_beside(g, spawn, index);
        let fn2000 = g.fn2000;
        let (men, rig) = (g.men, g.rig);
        let p = crate::game::player_new(g, men, rig, at, Some(fn2000), Some(index));
        // (written by his seat: with_seat is keyed by a man and he had none till now)
        if g.seat == s { g.player = p; } else { g.tab.cmd_seat[s].own.as_mut().unwrap().player = p; }
        let def = crate::rig_props::rail_light_def(man_char_id(g, p), g.tune.player_light);
        g.pl[p].light = crate::render_lights::light_set_add(&mut g.engine.lights, def);
    }
}

/// where seat `i`'s man stands when the first stands on `at` (seats.ts spotBeside)
pub fn spot_beside(g: &mut crate::game::Game, at: V3, i: f64) -> V3 {
    if !js::truthy(i) { return at; }
    let apart = g.k.seats.apart;
    for k in 0..8 {
        let a = m::PI / 2.0 + (k as f64) * m::PI / 4.0;
        let p: V3 = [at[0] + m::cos(a) * apart * i, at[1], at[2] + m::sin(a) * apart * i];
        let mut q = v3::copy(p);
        crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, Some(1.0));
        if m::hyp2(q[0] - p[0], q[2] - p[2]) < 0.02
            && !crate::scene_collision::clamp_to_world(g, &mut q, 0.3)
            && !crate::scene_collision::solid_blocked(g, [at[0], 0.45, at[2]], [p[0], 0.45, p[2]], None) { return p; }
    }
    v3::copy(at)
}

/// a staging puts the players on its start (seats.ts stageMen)
pub fn stage_men(g: &mut crate::game::Game, at: V3, yaw: f64, each: Option<&mut dyn FnMut(&mut crate::game::Game, PlayerId)>) {
    let mut each = each;
    deal_seats(g);
    let mut taken: Vec<V3> = vec![at];
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].state != crate::game::GuardState::Dead { taken.push(g.ch[g.gd[gd].ch].pos); }
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
        over_seat(g, s, &mut |g: &mut Game| {
            crate::game::game_teleport_player(g, v3::copy(p));
            let man = g.player;
            let c = g.pl[man].ch;
            g.ch[c].aim_yaw = yaw;
            g.ch[c].body_yaw = yaw;
            if let Some(f) = each.as_mut() { f(g, man); }
        });
    }
}

/// a scenario's opening cut: every seat's lens snapped to its own man (seats.ts cutToMen)
pub fn cut_to_men(g: &mut crate::game::Game) {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let at = over_seat(g, s, &mut |g: &mut Game| crate::game::game_follow_pos(g, None));
        let c = &mut g.tab.cmd_seat[s].cam;
        c.target = v3::copy(at);
        c.desired = v3::copy(c.target);
        c.desired_serial = c.desired_serial.wrapping_add(1);   // (a whole write of `desired`: Seat.takeCut, cmd.ts:191)
    }
}

// ---------------------------------------------------------------- who an enemy is up against

/// the man an enemy has in mind (seats.ts facedSeat)
pub fn faced_seat(g: &mut crate::game::Game, gd: GuardId) -> SeatId {
    let cur = seat_at(g, g.gd[gd].faces);
    let cur_man = crate::cmd::seat_man(g, cur);
    if matches!(g.gd[gd].foe, Some(crate::game::Foe::Player)) && !g.pl[cur_man].down { return cur; }
    let (mut best, mut bd) = (cur, f64::INFINITY);
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if g.pl[p].down { continue; }
        let keep = if s == cur { g.k.seats.keep } else { 0.0 };
        let d = v3::dist_xz(g.ch[g.pl[p].ch].pos, g.ch[g.gd[gd].ch].pos) - keep;
        if d < bd { bd = d; best = s; }
    }
    if best != cur { g.gd[gd].faces = Some(g.tab.cmd_seat[best].index); }
    best
}

/// the player an enemy has in mind as the tick stands (seats.ts facedMan)
pub fn faced_man(g: &mut crate::game::Game, gd: GuardId) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    let s = seat_at(g, g.gd[gd].faces);
    crate::cmd::seat_man(g, s)
}

/// the game as that player sees it (seats.ts facedView)
pub fn faced_view(g: &mut crate::game::Game, gd: GuardId) -> PlayerId { faced_man(g, gd) }

/// the game as the man behind this body has it; never moves `faces` (seats.ts viewOfBody)
pub fn view_of_body(g: &mut crate::game::Game, c_2: crate::seats::ViewOfBodyC) -> PlayerId {
    if g.seats.len() == 1 || g.seat != g.seats[0] || seam_in_first_seat_view(g) { return g.player; }
    if let Some(man) = view_of_id(g, c_2.id) { return man; }
    let mut found = None;
    for i in 0..g.guards.len() {
        let x = g.guards[i];
        if g.ch[g.gd[x].ch].id == c_2.id { found = Some(x); break; }
    }
    match found {
        None => { let s = g.seats[0]; crate::cmd::seat_man(g, s) }
        Some(gd) if on_team(g, gd) => { let s = seat_of(g, gd); crate::cmd::seat_man(g, s) }
        Some(gd) => faced_view(g, gd),
    }
}

/// the view a man of the floor is stepped over: his seat's for a teammate, the faced man's for anybody else (seats.ts viewFor)
pub fn view_for(g: &mut crate::game::Game, gd: GuardId) -> PlayerId {
    let s = if on_team(g, gd) { seat_of(g, gd) } else { faced_seat(g, gd) };
    crate::cmd::seat_man(g, s)
}

// ---------------------------------------------------------------- whose teammate

/// the deal: the team's roster is cut into as many runs as there are seats; returns whether it dealt (seats.ts dealSeats)
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

/// the seat a teammate takes his orders from (seats.ts seatOf)
pub fn seat_of(g: &mut crate::game::Game, gd: GuardId) -> SeatId {
    let at = g.gd[gd].unit.as_ref().and_then(|u| u.seat);
    seat_at(g, at)
}

/// is this teammate `s`'s to order (seats.ts seatHas)
pub fn seat_has(g: &mut crate::game::Game, s: SeatId, gd: GuardId) -> bool {
    unit_seat(g, gd) == g.tab.cmd_seat[s].index
}

/// the game handed in has one squad's orders and they are `g.orders` (seats.ts oneBook)
pub fn one_book(g: &mut crate::game::Game) -> bool {
    g.seats.len() == 1 || g.in_cast
}

/// the man a teammate follows: his seat's (seats.ts leaderOf)
pub fn leader_of(g: &mut crate::game::Game, gd: GuardId) -> PlayerId {
    if one_book(g) { return g.player; }
    let s = seat_of(g, gd);
    crate::cmd::seat_man(g, s)
}

/// the game as a teammate's own player sees it (seats.ts viewOf)
pub fn view_of(g: &mut crate::game::Game, gd: GuardId) -> PlayerId {
    if one_book(g) { return g.player; }
    let s = seat_of(g, gd);
    crate::cmd::seat_man(g, s)
}

/// the orders that move a teammate: his seat's book (seats.ts ordersOf). sig.rs returns nothing, so it answers nothing: a caller runs view_of inside with_seat and reads g.orders
pub fn orders_of(g: &mut crate::game::Game, gd: GuardId) {}

/// the order he is under, in whichever seat's book (seats.ts orderOf)
pub fn order_of(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> {
    if one_book(g) { return crate::orders::squad_orders_of(g, gd); }
    let s = seat_of(g, gd);
    over_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_of(g, gd))
}

/// is there a squad to order at all, any seat's (seats.ts teamActive)
pub fn team_active(g: &mut crate::game::Game) -> bool {
    if one_book(g) { return crate::orders::squad_orders_active(g); }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if over_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_active(g)) { return true; }
    }
    false
}

/// every seat's teammates, in the guard list's order (seats.ts teamSquad)
pub fn team_squad(g: &mut crate::game::Game) -> Vec<GuardId> {
    if one_book(g) { return crate::orders::squad_orders_squad(g); }
    let mut out = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if on_team(g, gd) { out.push(gd); }
    }
    out
}

/// … and the living of them (seats.ts teamMen)
pub fn team_men(g: &mut crate::game::Game) -> Vec<GuardId> {
    if one_book(g) { return crate::orders::squad_orders_men(g, None, None); }
    let mut out = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if on_team(g, gd) && g.gd[gd].state != crate::game::GuardState::Dead { out.push(gd); }
    }
    out
}

/// is the team in more than one pair (seats.ts teamPaired)
pub fn team_paired(g: &mut crate::game::Game) -> bool {
    if one_book(g) { return crate::orders::squad_orders_paired(g); }
    let mut ids: JsSet<crate::game::PairId> = JsSet::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].team != crate::teams::Team::Operative { continue; }
        if let Some(pair) = g.gd[gd].unit.as_ref().and_then(|u| u.pair) { ids.add(pair); }
    }
    ids.len() > 1
}

/// a seat's share of what the squad brings; `g` holds the seat's set (seats.ts seatShare)
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

/// the last order anybody gave, off the books' logs (seats.ts lastOrder)
pub fn last_order(g: &mut crate::game::Game) -> Option<crate::orders::SquadOrdersLogItem> {
    if one_book(g) { return g.orders.log.last().cloned(); }
    let mut best: Option<crate::orders::SquadOrdersLogItem> = None;
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if let Some(e) = over_seat(g, s, &mut |g: &mut Game| g.orders.log.last().cloned()) {
            if best.as_ref().map_or(true, |b| e.t > b.t) { best = Some(e); }
        }
    }
    best
}

/// every seat's book, in seat order: copies, read only (seats.ts teamBooks)
pub fn team_books(g: &mut crate::game::Game) -> Vec<crate::orders::SquadOrders> {
    if one_book(g) { return vec![g.orders.clone()]; }
    let mut out = Vec::new();
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        out.push(over_seat(g, s, &mut |g: &mut Game| g.orders.clone()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_address_word_reads_as_the_regex_does() {
        assert_eq!(seats_word(""), 1.0);
        assert_eq!(seats_word("?seats=2"), 2.0);
        assert_eq!(seats_word("seats=2"), 2.0);
        assert_eq!(seats_word("?a=1&seats=007&b=2"), 7.0);
        assert_eq!(seats_word("?seats=0"), 0.0);
        // not a whole word, no count, or a count with something after it
        assert_eq!(seats_word("?xseats=2"), 1.0);
        assert_eq!(seats_word("?seats="), 1.0);
        assert_eq!(seats_word("?seats=2x"), 1.0);
        // the first that matches, past one that does not
        assert_eq!(seats_word("?seats=x&seats=2"), 2.0);
    }

    #[test]
    fn a_man_s_ids() {
        assert_eq!(seat_man_id(0.0), crate::consts::PLAYER_ID);
        assert_eq!(seat_man_id(1.0), crate::consts::NOBODY_ID + 1.0);
    }

    #[test]
    fn tones_go_round() {
        assert_eq!(seat_tone(0.0), SeatTone::You);
        assert_eq!(seat_tone(1.0), SeatTone::Purple);
        assert_eq!(seat_tone(2.0), SeatTone::You);
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
