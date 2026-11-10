pub use ::game_types::unseen::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String};

// ------------------------------------------------------------ the state

/// `g = viewOf(g, gd)`: run `f` in the game as this teammate's seat has it. The seat the pass is about is that view: re-entrant by this test
fn in_view<R>(g: &mut Game, gd: GuardId, f: &mut dyn FnMut(&mut Game) -> R) -> R {
    let seat = crate::seats::view_of(g, gd);
    if seat == g.seat { f(g) } else { crate::seats::with_seat(g, seat, f) }
}

fn clamp(x: f64, a: f64, b: f64) -> f64 {
    m::max(a, m::min(b, x))
}

/// what stateOf makes for a view it has not seen
fn new_state() -> State {
    State { t: -1e9, live: false, sneak_t: -1e9, cue_t: -1e9, hold: None, sgn: 1.0, why: String::from("never asked") }
}

/// this view's row, made if it is not there
fn state_of<'g>(g: &'g mut Game) -> &'g mut State {
    let k = crate::seats::view_key(g, &crate::seats::here(g));
    g.m.unseen.states.get_or_insert_with(k, new_state)
}

/// which player the squad reads
fn lead(g: &mut Game) -> PlayerId {
    g.player
}

fn mate(g: &Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some()
}

/// on a wall, as the sneak counts it: his back on it `wall_secs`, where the hold keeps that clock
fn on_wall(g: &Game, pl: PlayerId) -> bool {
    let Some(w) = &g.pl[pl].wall else { return false };
    match w.hit_t {
        None => true,
        Some(hit_t) => hit_t >= g.k.unseen.wall_secs,
    }
}

/// Sam is low, as his squad counts it: by his own key. A knee the work of his hands put him on (a lock, the wand, a body) leaves it as it was before
pub(in crate) fn sam_low(g: &Game, pl: PlayerId) -> bool { g.pl[pl].crouch && g.pl[pl].knee.map_or(true, |k| k.low) }

/// he goes down on a knee for his work; one already low stays as he is. The squad keeps what it was doing: low with him, sneaking with him, or neither
pub(in crate) fn kneel(g: &mut Game, pl: PlayerId) {
    if !g.pl[pl].crouch {
        let sneaks = sam_sneaks(g, pl) || g.time - state_of(g).sneak_t <= g.k.unseen.linger;
        g.pl[pl].knee = Some(crate::player::Knee { low: on_wall(g, pl), sneaks });
    }
    g.pl[pl].crouch = true;
}

/// Sam is sneaking: low, on a wall, or with the wheel right down, and not sprinting
pub fn sam_sneaks(g: &Game, pl: PlayerId) -> bool {
    !crate::condition::down(g, g.pl[pl].ch) && !g.pl[pl].sprinting && !g.pl[pl].dash && ((g.pl[pl].crouch && g.pl[pl].knee.map_or(true, |k| k.sneaks)) || on_wall(g, pl) || g.pl[pl].pace <= g.k.unseen.pace_at + 1e-6)
}

/// what has given the squad away (None: nothing)
fn squad_found(g: &Game, men: &[GuardId]) -> Option<String> {
    for &gd in men {
        if crate::roomfight::room_fight_hot_for(g, gd) {
            return Some(String::from("loud"));
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        let state = g.gd[h].state;
        if (state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search) && crate::teams::hostile_to(g, h, g.gd[men[0]].team) {
            return Some(String::from(if state == crate::game::GuardState::Alert { "a defender alert" } else { "a defender searching" }));
        }
    }
    None
}

/// The squad is unseen this frame, read once a frame by whoever asks first
fn unseen_live(g: &mut Game) -> bool {
    let now = g.time;
    let s = state_of(g);
    if s.t == now {
        return s.live;
    }
    let was = s.live;
    s.t = now;
    if now < s.sneak_t { s.sneak_t = -1e9; }
    if now < s.cue_t { s.cue_t = -1e9; }
    let pl = lead(g);
    let why;
    if !crate::orders::squad_orders_active(g, &crate::seats::here(g)) {
        why = String::from("no squad under orders");
    } else {
        let mut men = crate::orders::squad_orders_squad(g, &crate::seats::here(g));
        men.retain(|&m| !crate::condition::down(g, g.gd[m].ch));
        if men.is_empty() {
            why = String::from("no squad under orders");
        } else {
            if g.pl[pl].sprinting || crate::condition::down(g, g.pl[pl].ch) {
                state_of(g).sneak_t = -1e9;
            } else if sam_sneaks(g, pl) {
                state_of(g).sneak_t = now;
            }
            if now - state_of(g).sneak_t > g.k.unseen.linger {
                why = String::from("not sneaking");
            } else {
                why = squad_found(g, &men).unwrap_or_default();
            }
        }
    }
    let live = why.is_empty();
    let s = state_of(g);
    s.live = live;
    s.why = why.clone();
    if live != was {
        if live {
            let k = crate::seats::view_key(g, &crate::seats::here(g));
            cue(g, k);
        } else {
            state_of(g).hold = None;
        }
    }
    live
}

/// said once as a spell starts, by the first man following; `pl` is the key of the spell's row in `states`, not always a man (a cast's is CAST_KEY)
fn cue(g: &mut Game, pl: PlayerId) {
    let now = g.time;
    if now - g.m.unseen.states.get(pl).unwrap().cue_t < g.k.unseen.cue_gap {
        return;
    }
    let mut who = None;
    for m in crate::orders::squad_orders_squad(g, &crate::seats::here(g)) {
        let state = g.gd[m].state;
        if state != crate::game::GuardState::Dead && state != crate::game::GuardState::Alert && g.gd[m].held.is_none() {
            who = Some(m);
            break;
        }
    }
    let Some(gd) = who else { return };
    g.m.unseen.states.get_mut(pl).unwrap().cue_t = now;   // before the line is said: a read of the row from under `say` must see it, as the TypeScript's would
    g.stats.unseen.cues += 1;
    let sam = lead(g);
    let low = sam_low(g, sam) || on_wall(g, sam);
    crate::game::game_say(g, gd, if low { "low and quiet." } else { "slow and quiet." }, None, None);
}

/// this teammate is unseen this frame
pub fn under_unseen(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| mate(g, gd) && unseen_live(g) && !entering(g, &crate::seats::here(g), gd))
}

/// he is one of a door tool's men right now: that work is the door's own
fn entering(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    seat.door_tools.ops.iter().any(|&op| g.tab.doortools_door_op[op].men.contains(&gd))
}

/// Sam on a wall, and no place on it for this man right now
fn off_wall(g: &mut Game, gd: GuardId) -> bool {
    let pl = lead(g);
    on_wall(g, pl) && (g.m.unseen.on_wall_at.get(gd).copied() != Some(g.time) || g.m.unseen.kneel_at.get(gd).copied() == Some(g.time))
}

fn low_now(g: &mut Game, gd: GuardId) -> bool {
    let pl = lead(g);
    sam_low(g, pl) || off_wall(g, gd)
}

/// he is low because Sam is
pub fn unseen_low(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| under_unseen(g, gd) && low_now(g, gd))
}

// ------------------------------------------------------------ the follower's seams

/// A follower's legs and knee this frame; None: the file's own
pub fn unseen_follow(g: &mut Game, gd: GuardId, sam_v: f64, d: f64, walk: f64) -> Option<UnseenFollow> {
    in_view(g, gd, &mut |g: &mut Game| {
        if !under_unseen(g, gd) {
            return None;
        }
        let low = low_now(g, gd);
        let k = &g.k.unseen;
        let top = if low { m::min(k.low_pace, walk) } else { walk };
        Some(UnseenFollow { speed: clamp(sam_v + k.gain * (d - 0.3), m::min(k.min_pace, top), top), crouch: low })
    })
}

/// a walk to a mark an order gave him: the pace he keeps to, low; None: the order's own carriage
pub fn unseen_post(g: &mut Game, gd: GuardId, order: &str) -> Option<f64> {
    in_view(g, gd, &mut |g: &mut Game| if crate::orders::rows::id_is(order, crate::orders::Is::WalksLow) && unseen_low(g, gd) { Some(g.k.unseen.low_pace) } else { None })
}

/// A follower's place while Sam is on a wall; None: his place in the file as ever
pub fn unseen_wall_slot(g: &mut Game, gd: GuardId, back: f64, file: &[GuardId]) -> Option<V3> {
    in_view(g, gd, &mut |g: &mut Game| {
        let q = wall_slot(g, gd, back, file);
        if q.is_some() {
            let now = g.time;
            g.m.unseen.on_wall_at.set(gd, now);
            let pl = lead(g);
            let wall = g.pl[pl].wall.as_ref().unwrap().serial;
            g.m.unseen.placed.set(gd, wall);
        } else {
            g.m.unseen.placed.delete(gd);
        }
        q
    })
}

/// the place just dealt was not taken after all: he is in the open, so on a knee
pub fn unseen_wall_lost(g: &mut Game, gd: GuardId) {
    g.m.unseen.on_wall_at.delete(gd);
    g.m.unseen.placed.delete(gd);
}

/// the place itself: along Sam's face, on the file's side, a body's width off it
fn wall_slot(g: &mut Game, gd: GuardId, back: f64, file: &[GuardId]) -> Option<V3> {
    if !under_unseen(g, gd) {
        return None;
    }
    let pl = lead(g);
    if g.pl[pl].wall.is_none() || !on_wall(g, pl) {
        return None;
    }
    let (t, n, w_d, w_lo, w_hi, w_side, lean_off, wall) = {
        let w = g.pl[pl].wall.as_ref().unwrap();
        (w.along, w.n, w.d, w.lo, w.hi, w.side, w.lean_off, w.serial)
    };
    let sam = g.ch[g.pl[pl].ch].pos;
    let su = sam[0] * t[0] + sam[2] * t[2];
    if state_of(g).hold != Some(wall) {
        let mut sum = 0.0;
        for &o in file {
            let p = g.ch[g.gd[o].ch].pos;
            sum += p[0] * t[0] + p[2] * t[2] - su;
        }
        let lo = su - w_lo;
        let hi = w_hi - su;
        let wall_end = g.k.unseen.wall_end;
        let sgn = if m::min(lo, hi) < wall_end && m::abs(lo - hi) > 0.5 {
            if lo > hi { -1.0 } else { 1.0 }
        } else if m::abs(sum) > 0.3 * m::max(1.0, file.len() as f64) {
            if sum > 0.0 { 1.0 } else { -1.0 }
        } else if w_side > 0.0 {
            -1.0
        } else {
            1.0
        };
        let s = state_of(g);
        s.hold = Some(wall);
        s.sgn = sgn;
    }
    let c = g.ch[g.gd[gd].ch].pos;
    let sgn = state_of(g).sgn;
    let u = su - lean_off.unwrap_or(0.0) + sgn * back;
    let r = 0.34;
    if u < w_lo + r || u > w_hi - r {
        return None;
    }
    if g.m.unseen.placed.get(gd).copied() != Some(wall) && (c[0] * t[0] + c[2] * t[2] - su) * sgn < -g.k.unseen.wall_cross {
        return None;
    }
    let (wall_off, wall_out) = (g.k.unseen.wall_off, g.k.unseen.wall_out);
    for out in [0.0, wall_out * 0.5, wall_out] {
        let off = w_d + wall_off + out;
        let q: V3 = [t[0] * u + n[0] * off, 0.0, t[2] * u + n[2] * off];
        if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) {
            continue;
        }
        if crate::scene_collision::solid_blocked(g, [sam[0], 0.45, sam[2]], [q[0], 0.45, q[2]], None) {
            break;
        }
        if !crate::scene_collision::segment_blocked(g, [q[0], 1.35, q[2]], [q[0] - n[0] * (off - w_d + 0.5), 1.35, q[2] - n[2] * (off - w_d + 0.5)], None, None, None) {
            let now = g.time;
            g.m.unseen.kneel_at.set(gd, now);
        }
        return Some(q);
    }
    None
}

// ------------------------------------------------------------ the posture's seams (silent.ts asks each beside its own rule)

/// the most his legs may be asked for; None: no cap of this file's
pub(super) fn unseen_pace(g: &mut Game, gd: GuardId) -> Option<f64> {
    in_view(g, gd, &mut |g: &mut Game| if g.k.unseen.cap > 0.0 && under_unseen(g, gd) { Some(g.k.unseen.cap) } else { None })
}

/// a footfall's level; None: his usual
pub(super) fn unseen_step(g: &mut Game, gd: GuardId) -> Option<f64> {
    in_view(g, gd, &mut |g: &mut Game| {
        if !under_unseen(g, gd) {
            return None;
        }
        let c = g.gd[gd].ch;
        Some(if g.ch[c].anim.crouch > 0.5 { g.k.unseen.low_step } else { g.k.unseen.walk_step })
    })
}

pub(super) fn unseen_hush(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| under_unseen(g, gd))
}

pub(super) fn unseen_ease(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| under_unseen(g, gd))
}

/// his round at `h` is held: nobody has found the squad, and `h` has not noticed anybody. `urgent` runs in the caller's game, not the view, as its closure does
pub(super) fn unseen_holds(g: &mut Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut Game) -> bool) -> bool {
    if quiet || !in_view(g, gd, &mut |g: &mut Game| under_unseen(g, gd) && !crate::roomfight::room_fight_hot_for(g, gd)) {
        return false;
    }
    let state = g.gd[h].state;
    if state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search || g.gd[h].awareness >= noticed || urgent(g) {
        return false;
    }
    true
}

/// through standing glass the hold is the silent order's own: is that rule his this frame
pub(super) fn unseen_glass(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| under_unseen(g, gd) && !crate::roomfight::room_fight_hot_for(g, gd))
}

// ------------------------------------------------------------ the HUD's word, QA

/// the squad line's word beside the posture (None: nothing to say)
pub(super) fn unseen_word(g: &Game, seat: &crate::seats::SeatRef) -> Option<String> {
    let live = g.m.unseen.states.get(crate::seats::view_key(g, seat)).is_some_and(|s| s.live);   // (as the tick last read it: `unseen_live` is a clock of its own, kept by the tick)
    if live { Some(String::from("sneaking")) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_has_the_languages_edges() {
        assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(-2.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(0.5, 1.0, 0.0), 1.0);   // a above b: the max wins, as Math.max(a, Math.min(b, x))
        assert!(clamp(f64::NAN, 0.0, 1.0).is_nan());
    }

    fn game_with_sam() -> (alloc::boxed::Box<Game>, PlayerId) {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let ch = crate::character::man_new(&mut g, crate::character::Character::blank());
        let pl = g.pl.push(crate::game::Player { ch, ..Default::default() });
        g.player = pl;
        (g, pl)
    }

    #[test]
    fn a_wall_counts_once_his_back_has_been_on_it_a_beat() {
        let (mut g, pl) = game_with_sam();
        assert!(!on_wall(&mut g, pl));
        g.pl[pl].wall = Some(crate::player::WallHold::default());
        assert!(on_wall(&mut g, pl));   // a hold with no clock counts at once
        let secs = g.k.unseen.wall_secs;
        g.pl[pl].wall.as_mut().unwrap().hit_t = Some(secs - 0.01);
        assert!(!on_wall(&mut g, pl));
        g.pl[pl].wall.as_mut().unwrap().hit_t = Some(secs);
        assert!(on_wall(&mut g, pl));
    }

    #[test]
    fn sneaking_is_low_or_walled_or_slow_and_never_sprinting() {
        let (mut g, pl) = game_with_sam();
        g.pl[pl].pace = 1.0;
        assert!(!sam_sneaks(&mut g, pl));
        g.pl[pl].pace = g.k.unseen.pace_at;
        assert!(sam_sneaks(&mut g, pl));
        g.pl[pl].pace = 1.0;
        g.pl[pl].crouch = true;
        assert!(sam_sneaks(&mut g, pl));
        g.pl[pl].sprinting = true;
        assert!(!sam_sneaks(&mut g, pl));
    }

    #[test]
    fn a_knee_for_work_leaves_him_as_he_was() {
        // (on his own key, with the wheel right down, off a wall, or just up off it: what he was is what he is counted as)
        let counted = |f: &dyn Fn(&mut Game, PlayerId)| {
            let (mut g, pl) = game_with_sam();
            g.pl[pl].pace = 1.0;
            g.time = 5.0;
            f(&mut g, pl);
            kneel(&mut g, pl);
            g.pl[pl].pace = 1.0;   // (at his work, and off the wall for it)
            g.pl[pl].wall = None;
            assert!(g.pl[pl].crouch);
            (sam_low(&g, pl), sam_sneaks(&g, pl))
        };
        assert_eq!(counted(&|_, _| {}), (false, false), "up before it");
        assert_eq!(counted(&|g, pl| g.pl[pl].crouch = true), (true, true), "low by his key");
        assert_eq!(counted(&|g, pl| g.pl[pl].pace = g.k.unseen.pace_at), (false, true), "slow and upright");
        assert_eq!(counted(&|g, pl| g.pl[pl].wall = Some(crate::player::WallHold::default())), (true, true), "on a wall");
        assert_eq!(counted(&|g, _| state_of(g).sneak_t = 5.0), (false, true), "a moment ago");
    }

    #[test]
    fn beside_a_wall_a_man_with_no_place_this_frame_is_off_it() {
        let (mut g, pl) = game_with_sam();
        let gd = GuardId(0);
        g.time = 5.0;
        assert!(!off_wall(&mut g, gd));   // Sam on no wall
        g.pl[pl].wall = Some(crate::player::WallHold::default());
        assert!(off_wall(&mut g, gd));    // no place dealt
        g.m.unseen.on_wall_at.set(gd, 5.0);
        assert!(!off_wall(&mut g, gd));   // a place dealt this frame
        g.m.unseen.kneel_at.set(gd, 5.0);
        assert!(off_wall(&mut g, gd));    // under a window: his place, on a knee
        g.m.unseen.kneel_at.set(gd, 4.0);
        assert!(!off_wall(&mut g, gd));   // an old kneel does not count
    }

    #[test]
    fn a_new_state_is_the_typescripts_not_zeroes() {
        let s = new_state();
        assert_eq!((s.t, s.sneak_t, s.cue_t, s.sgn), (-1e9, -1e9, -1e9, 1.0));
        assert_eq!(s.why, "never asked");
    }
}
