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

/// is this call inside a seat's view (seats.ts `rawGame(g) !== g`), and not on the bare Game. with_seat raises Game.view_depth for the length of its call
pub fn in_view(g: &crate::game::Game) -> bool { g.view_depth > 0 }

/// INVENTED BY THE SKELETON: which row of a table kept per view this pass reads: the man of the seat on Game, or CAST_KEY inside a second cast.
/// (seats.ts SEAT_TABLES key by the view; the bridge writes a view's key as `view.player`, export.ts:83)
pub fn view_key(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> PlayerId {
    if g.in_cast { CAST_KEY } else { seat.player }
}

/// A SEAT'S MAN IS MADE ANEW (a restart): every table kept per view moves its row from the man who left to the man who stands in his place.
/// The TypeScript keys these tables by the VIEW (seats.ts SEAT_TABLES, the eighteen named there), which a restart does not replace, so what a module
/// kept for the seat is kept across it (the rear watch's book and its log, the squad's throws). Here a view's key is its seat's man (`view_key`), and a
/// new man is a new key: without this every such module began again at a restart. The rows keep their order in each table; a row already under the
/// new man (none is made before this is called) gives way to the seat's own.
pub fn view_rekey(g: &mut crate::game::Game, old: PlayerId, new: PlayerId) {
    fn rekey<V>(m: &mut crate::js::JsMap<PlayerId, V>, old: PlayerId, new: PlayerId) {
        if old == new || !m.has(old) { return; }
        m.delete(new);
        let mut rows: Vec<(PlayerId, V)> = Vec::new();
        for k in m.keys() { if let Some(v) = m.delete(k) { rows.push((if k == old { new } else { k }, v)); } }
        m.clear();
        for (k, v) in rows { m.set(k, v); }
    }
    let m = &mut g.m;
    rekey(&mut m.blast_feel.state, old, new); rekey(&mut m.blast_look.state, old, new); rekey(&mut m.close.state, old, new);
    rekey(&mut m.guards.ground_track, old, new); rekey(&mut m.holdfire.states, old, new); rekey(&mut m.player.light_at, old, new);
    rekey(&mut m.rearwatch.books, old, new); rekey(&mut m.samlook.state, old, new); rekey(&mut m.showwhy.states, old, new);
    rekey(&mut m.silent.seats, old, new); rekey(&mut m.silent.states, old, new); rekey(&mut m.squad40.states, old, new);
    rekey(&mut m.squadsmoke.states, old, new); rekey(&mut m.squadthrow.states, old, new); rekey(&mut m.stalk.state, old, new);
    rekey(&mut m.unseen.states, old, new); rekey(&mut m.wandorder.states, old, new);
}

/// one seat's SEAT_KEYS fields and Game's, changed over
fn swap_set(g: &mut Game, o: &mut SeatSet) {
    use core::mem::swap;
    swap(&mut g.seat, &mut o.seat); swap(&mut g.player, &mut o.player); swap(&mut g.orders, &mut o.orders); swap(&mut g.door_tools, &mut o.door_tools);
    swap(&mut g.work, &mut o.work); swap(&mut g.wall_charge, &mut o.wall_charge); swap(&mut g.aim_point, &mut o.aim_point); swap(&mut g.aim_target, &mut o.aim_target);
    swap(&mut g.aim_guard, &mut o.aim_guard); swap(&mut g.aim_friend, &mut o.aim_friend); swap(&mut g.aim_snap, &mut o.aim_snap); swap(&mut g.hover, &mut o.hover);
    swap(&mut g.interactables, &mut o.interactables); swap(&mut g.nearby, &mut o.nearby); swap(&mut g.act, &mut o.act); swap(&mut g.use_door, &mut o.use_door);
    swap(&mut g.view_proj, &mut o.view_proj); swap(&mut g.threat_proximity, &mut o.threat_proximity); swap(&mut g.torch_on_t, &mut o.torch_on_t);
}

/// INVENTED BY THE SKELETON (conventions, SEATS): a pass about another seat. That seat's SEAT_KEYS fields are put on Game for the call and
/// taken back after; the seat that stood on Game keeps its own meanwhile. The seat already on Game runs as it stands (so a nesting is fine).
/// For the length of the call the game is "a seat's view" (`in_view`), the first seat's too.
pub fn with_seat<R>(g: &mut crate::game::Game, seat: SeatId, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    if g.seat == seat { g.view_depth += 1; let r = f(g); g.view_depth -= 1; return r; }
    let from = g.seat;
    let mut theirs = match g.tab.cmd_seat[seat].own.take() { Some(o) => o, None => panic!("with_seat: that seat keeps no set") };
    swap_set(g, &mut theirs);                 // Game now stands on `seat`; `theirs` holds what `from` had
    g.tab.cmd_seat[from].own = Some(theirs);
    g.view_depth += 1;
    let r = f(g);
    g.view_depth -= 1;
    let mut back = g.tab.cmd_seat[from].own.take().unwrap();
    swap_set(g, &mut back);
    g.tab.cmd_seat[seat].own = Some(back);
    r
}

/// WHICH SEAT THIS MACHINE SHOWS, from now on: a host's own fact, told after a start and whenever it changes. The rows of a game are numbered the same on every
/// machine (a start stands with the first seat), and this only says whose set stands on Game between ticks, for the page to read. It changes nothing of the play:
/// a tick stands the first seat's set whichever seat is shown (`on_first_bare`).
pub fn show_seat(g: &mut crate::game::Game, seat: SeatId) {
    assert!(g.seats.contains(&seat), "a host asks to show a seat this game has not got: {:?}", seat);
    g.local_seat = Some(seat);
    if g.seat == seat { return; }
    let from = g.seat;
    let mut theirs = match g.tab.cmd_seat[seat].own.take() { Some(o) => o, None => panic!("show_seat: that seat keeps no set") };
    swap_set(g, &mut theirs);
    g.tab.cmd_seat[from].own = Some(theirs);
}

/// THE BARE GAME FOR THE LENGTH OF A TICK: the first seat's set on Game and no view open (seats.ts seatsStand: inside a tick the Game's own fields of a
/// seat's name answer the FIRST seat's). Where another seat stood on Game (a host that shows the second player), the sets are changed over and back.
pub fn on_first_bare<R>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    let first = g.seats[0];
    if g.seat == first { return f(g); }
    let from = g.seat;
    let mut theirs = match g.tab.cmd_seat[first].own.take() { Some(o) => o, None => panic!("on_first_bare: the first seat keeps no set") };
    swap_set(g, &mut theirs);
    g.tab.cmd_seat[from].own = Some(theirs);
    let r = f(g);
    let mut back = g.tab.cmd_seat[from].own.take().unwrap();
    swap_set(g, &mut back);
    g.tab.cmd_seat[first].own = Some(back);
    r
}

/// EVERY SEAT'S SET IN ITS OWN ROW for the length of the call, that of the seat that stands on Game too (Game's fields of it are blank meanwhile): for who walks the sets.
pub fn parked<R>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R {
    let on = g.seat;
    let mut set = SeatSet::default();
    swap_set(g, &mut set);
    g.tab.cmd_seat[on].own = Some(set);
    let r = f(g);
    let mut back = g.tab.cmd_seat[on].own.take().unwrap();
    swap_set(g, &mut back);
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

/// seats.ts:213 `rawGame(g) !== g` with the first seat on Game: a pass over his view and a pass over the bare Game look alike here
fn seam_in_first_seat_view(g: &Game) -> bool { crate::seats::in_view(g) }

// ---------------------------------------------------------------- small helpers

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

/// a seat's part of the squad's `n`: dealt in roster order so the shares add up to the count
fn share_of(n: f64, lo: f64, mine: f64, all: f64) -> f64 {
    m::floor(n * (lo + mine) / all) - m::floor(n * lo / all)
}

// ---------------------------------------------------------------- the seats

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

// ---------------------------------------------------------------- the men

/// the character id of seat `i`'s man (seats.ts seatManId)
pub fn seat_man_id(i: f64) -> f64 {
    if i == 0.0 { crate::consts::PLAYER_ID } else { crate::consts::NOBODY_ID + i }
}

/// is `id` a player's man's (seats.ts isManId)
pub fn is_man_id(g: &crate::game::Game, id_2: f64) -> bool {
    id_2 == crate::consts::PLAYER_ID || (id_2 > crate::consts::NOBODY_ID && id_2 < crate::consts::NOBODY_ID + g.k.seats.max)
}

/// every seat's man, in seat order (seats.ts men)
pub fn men(g: &crate::game::Game) -> Vec<PlayerId> {
    if g.seats.len() == 1 { return vec![g.player]; }
    let mut out = Vec::new();
    for i in 0..g.seats.len() { let s = g.seats[i]; out.push(crate::cmd::seat_man(g, s)); }
    out
}

/// the men still up (seats.ts menUp)
pub fn men_up(g: &mut crate::game::Game) -> Vec<PlayerId> {
    let mut all = men(g);
    all.retain(|p| !g.pl[*p].down);
    all
}

/// is every player down (seats.ts allDown)
pub fn all_down(g: &crate::game::Game) -> bool {
    if g.seats.len() == 1 { return g.pl[g.player].down; }
    for i in 0..g.seats.len() {
        let p = crate::cmd::seat_man(g, g.seats[i]);
        if !g.pl[p].down { return false; }
    }
    true
}

/// the men a rule about the team goes by: those still up, or every one if none is (a game of one asks about its one man, down or up)
pub fn men_left(g: &crate::game::Game) -> Vec<PlayerId> {
    let all = men(g);
    let up: Vec<PlayerId> = all.iter().copied().filter(|p| !g.pl[*p].down).collect();
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

/// can a mission of this kind be played by two (seats.ts seatsKind)
pub fn seats_kind(kind: &str) -> bool { crate::seats::seat_kinds().iter().any(|k| k == kind) }

/// why this game cannot start a mission of this kind (seats.ts seatsRefuse)
pub fn seats_refuse(g: &crate::game::Game, kind: &str) -> Option<String> {
    if g.seats.len() > 1 && !seats_kind(kind) { Some(String::from("it takes one player today")) } else { None }
}

/// the man whose eyes and ears this host's picture and speakers are (seats.ts hostMan)
pub fn host_man(g: &crate::game::Game) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    let s = crate::game::game_host_seat(g);
    crate::cmd::seat_man(g, s)
}

/// the man a writer with no man of its own names (seats.ts manHere): the view's; bare, Game holds the first seat's set, so both arms are g.player
pub fn man_here(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> PlayerId { seat.player }

/// `f` for every seat, over its view (seats.ts eachSeat)
pub fn each_seat(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game)) {
    if g.seats.len() == 1 { f(g); return; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        crate::seats::with_seat(g, s, &mut *f);
    }
}

/// the first answer any seat's view gives `f` (seats.ts firstOf)
pub fn first_of<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, f: &mut dyn FnMut(&mut crate::game::Game) -> Option<T>) -> Option<T> {
    if g.seats.len() == 1 { return f(g); }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let r = crate::seats::with_seat(g, s, &mut *f);
        if r.is_some() { return r; }
    }
    None
}

/// the seat whose man this character is (seats.ts seatOfChar)
pub fn seat_of_char(g: &crate::game::Game, c_2: Option<CharId>) -> Option<SeatId> {
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
pub fn seat_of_id(g: &crate::game::Game, id_2: f64) -> Option<SeatId> {
    if !is_man_id(g, id_2) { return None; }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let p = crate::cmd::seat_man(g, s);
        if man_char_id(g, p) == id_2 { return Some(s); }
    }
    None
}

/// the man who carries character id `id` (seats.ts manById)
pub fn man_by_id(g: &crate::game::Game, id_2: f64) -> Option<PlayerId> {
    if g.seats.len() == 1 {
        return if man_char_id(g, g.player) == id_2 { Some(g.player) } else { None };
    }
    let s = seat_of_id(g, id_2)?;
    Some(crate::cmd::seat_man(g, s))
}

/// the game as the player with this id sees it, None for no player's id (seats.ts viewOfId)
pub fn view_of_id(g: &crate::game::Game, id_2: f64) -> Option<SeatId> {
    if g.seats.len() == 1 {
        return if man_char_id(g, g.player) == id_2 { Some(g.seat) } else { None };
    }
    seat_of_id(g, id_2)
}

/// the seat of a man (seats.ts seatOfMan)
pub fn seat_of_man(g: &crate::game::Game, pl: PlayerId) -> SeatId {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if crate::cmd::seat_man(g, s) == pl { return s; }
    }
    g.seats[0]
}

/// what a player is called (seats.ts playerName)
pub fn player_name(g: &crate::game::Game, pl: PlayerId) -> String {
    if g.seats.len() == 1 { return String::from("Player"); }
    let s = seat_of_man(g, pl);
    alloc::format!("Player {}", g.tab.cmd_seat[s].index + 1.0)
}

/// the living man nearest a place (seats.ts nearestMan)
pub fn nearest_man(g: &crate::game::Game, at: &[f64]) -> Option<PlayerId> {
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
        crate::seats::with_seat(g, s, &mut |g: &mut Game| g.player = p);
        let (owner, intensity) = (man_char_id(g, p), g.tune.player_light);
        let def = crate::rig_props::rail_light_def(g, owner, intensity);
        g.pl[p].light = crate::render_lights::light_set_add(g, def);
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

/// a scenario's opening cut: every seat's lens snapped to its own man (seats.ts cutToMen)
pub fn cut_to_men(g: &mut crate::game::Game) {
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        let at = crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::game::game_follow_pos(g, None));
        let cam = g.tab.cmd_seat[s].cam;
        let c = &mut g.tab.camera[cam];
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
pub fn faced_man(g: &crate::game::Game, gd: GuardId) -> PlayerId {
    if g.seats.len() == 1 { return g.player; }
    let s = seat_at(g, g.gd[gd].faces);
    crate::cmd::seat_man(g, s)
}

/// the game as that player sees it (seats.ts facedView)
pub fn faced_view(g: &crate::game::Game, gd: GuardId) -> SeatId {
    if g.seats.len() == 1 { return g.seat; }
    seat_at(g, g.gd[gd].faces)
}

/// the game as the man behind this body has it; never moves `faces` (seats.ts viewOfBody)
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

/// the view a man of the floor is stepped over: his seat's for a teammate, the faced man's for anybody else (seats.ts viewFor)
pub fn view_for(g: &mut crate::game::Game, gd: GuardId) -> SeatId {
    if on_team(g, gd) { seat_of(g, gd) } else { faced_seat(g, gd) }
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
pub fn seat_of(g: &crate::game::Game, gd: GuardId) -> SeatId {
    let at = g.gd[gd].unit.as_ref().and_then(|u| u.seat);
    seat_at(g, at)
}

/// is this teammate `s`'s to order (seats.ts inSeat)
pub fn in_seat(g: &crate::game::Game, s: SeatId, gd: GuardId) -> bool {
    unit_seat(g, gd) == g.tab.cmd_seat[s].index
}

/// the game handed in has one squad's orders and they are `g.orders` (seats.ts oneBook)
pub fn one_book(g: &crate::game::Game) -> bool {
    g.seats.len() == 1 || g.in_cast
}

/// the game as a teammate's own player sees it (seats.ts viewOf)
pub fn view_of(g: &crate::game::Game, gd: GuardId) -> SeatId {
    if one_book(g) { return g.seat; }
    seat_of(g, gd)
}

/// the orders that move a teammate: his seat's book, named by the man it is on Game with (seats.ts ordersOf)
pub fn orders_of(g: &crate::game::Game, gd: GuardId) -> PlayerId {
    if one_book(g) { return g.player; }
    let s = seat_of(g, gd);
    crate::cmd::seat_man(g, s)
}

/// the order he is under, in whichever seat's book (seats.ts orderOf)
pub fn order_of(g: &mut crate::game::Game, gd: GuardId) -> Option<OrdersAssignmentId> {
    if one_book(g) { return crate::orders::squad_orders_of(g, &crate::seats::here(g), gd); }
    let s = seat_of(g, gd);
    crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_of(g, &crate::seats::here(g), gd))
}

/// is there a squad to order at all, any seat's (seats.ts teamActive)
pub fn team_active(g: &mut crate::game::Game) -> bool {
    if one_book(g) { return crate::orders::squad_orders_active(g, &crate::seats::here(g)); }
    for i in 0..g.seats.len() {
        let s = g.seats[i];
        if crate::seats::with_seat(g, s, &mut |g: &mut Game| crate::orders::squad_orders_active(g, &crate::seats::here(g))) { return true; }
    }
    false
}

/// every seat's teammates, in the guard list's order (seats.ts teamSquad)
pub fn team_squad(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    if one_book(g) { return crate::orders::squad_orders_squad(g, seat); }
    let mut out = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if on_team(g, gd) { out.push(gd); }
    }
    out
}

/// … and the living of them (seats.ts teamMen)
pub fn team_men(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    if one_book(g) { return crate::orders::squad_orders_men(g, seat, None, None); }
    let mut out = Vec::new();
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if on_team(g, gd) && g.gd[gd].state != crate::game::GuardState::Dead { out.push(gd); }
    }
    out
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
