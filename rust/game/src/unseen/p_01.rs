//! src/game/unseen.ts lines 1 to 240
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the orders whose walk to a mark goes low while Sam is low (unseen.ts POSTED)
static POSTED: [&str; 2] = ["move", "cover"];

// ------------------------------------------------------------ seams: what skeleton 3 cannot say yet

/// the identity of the wall hold Sam is on now, as `State.hold` and `placed` keep it (unseen.ts:174, 184, 187, 195)
fn seam_wall_serial(g: &mut Game, pl: PlayerId) -> u32 {
    todo!("needs: a serial on WallHold, dealt where a hold is made (player.ts:1081, 1255)")
}
/// `UNSEEN_STATS.seconds += dt` (unseen.ts:135): a sum of seconds, in a counter typed i32
fn seam_add_seconds(g: &mut Game, dt: f64) {
    todo!("needs: UnseenStats.seconds as f64")
}

// ------------------------------------------------------------ the state

/// `g = viewOf(g, gd)`: run `f` in the game as this teammate's seat has it. The seat already on Game is that view: re-entrant by this test (seats.ts viewOf)
fn in_view<R>(g: &mut Game, gd: GuardId, f: &mut dyn FnMut(&mut Game) -> R) -> R {
    let seat = crate::seats::view_of(g, gd);
    if seat == g.player { f(g) } else { crate::seats::with_seat(g, seat, f) }
}

/// (unseen.ts clamp)
pub fn clamp(x: f64, a: f64, b: f64) -> f64 {
    m::max(a, m::min(b, x))
}

/// what stateOf makes for a view it has not seen (unseen.ts:103)
fn new_state() -> State {
    State { t: -1e9, live: false, sneak_t: -1e9, cue_t: -1e9, hold: None, sgn: 1.0, why: String::from("never asked") }
}

/// this view's row, made if it is not there (unseen.ts stateOf)
pub fn state_of<'g>(g: &'g mut Game) -> &'g mut State {
    let k = crate::seats::view_key(g);
    g.m.unseen.states.get_or_insert_with(k, new_state)
}

/// which player the squad reads (unseen.ts lead)
pub fn lead(g: &mut Game) -> PlayerId {
    g.player
}

/// (unseen.ts mate)
pub fn mate(g: &mut Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some()
}

/// on a wall, as the sneak counts it: his back on it `wall_secs`, where the hold keeps that clock (unseen.ts onWall)
pub fn on_wall(g: &mut Game, pl: PlayerId) -> bool {
    let Some(w) = &g.pl[pl].wall else { return false };
    match w.hit_t {
        None => true,
        Some(hit_t) => hit_t >= g.k.unseen.wall_secs,
    }
}

/// Sam is sneaking: low, on a wall, or with the wheel right down, and not sprinting (unseen.ts samSneaks)
pub fn sam_sneaks(g: &mut Game, pl: PlayerId) -> bool {
    !g.pl[pl].down && !g.pl[pl].sprinting && !g.pl[pl].dash && (g.pl[pl].crouch || on_wall(g, pl) || g.pl[pl].pace <= g.k.unseen.pace_at + 1e-6)
}

/// what has given the squad away (None: nothing) (unseen.ts squadFound)
pub fn squad_found(g: &mut Game, men: &[GuardId]) -> Option<String> {
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
        if (state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search) && crate::teams::hostile_to(g, h, crate::teams::HostileToO::Guard(men[0])) {
            return Some(String::from(if state == crate::game::GuardState::Alert { "a defender alert" } else { "a defender searching" }));
        }
    }
    None
}

/// The squad is unseen this frame, read once a frame by whoever asks first (unseen.ts unseenLive)
pub fn unseen_live(g: &mut Game) -> bool {
    let now = g.time;
    let s = state_of(g);
    if s.t == now {
        return s.live;
    }
    let was = s.live;
    let dt = now - s.t;
    s.t = now;
    if now < s.sneak_t { s.sneak_t = -1e9; }
    if now < s.cue_t { s.cue_t = -1e9; }
    let pl = lead(g);
    let why;
    if !crate::orders::squad_orders_active(g) {
        why = String::from("no squad under orders");
    } else {
        let mut men = crate::orders::squad_orders_squad(g);
        men.retain(|&m| g.gd[m].state != crate::game::GuardState::Dead);
        if men.is_empty() {
            why = String::from("no squad under orders");
        } else {
            if g.pl[pl].sprinting || g.pl[pl].down {
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
    if live && was && dt > 0.0 && dt < 1.0 {
        seam_add_seconds(g, dt);
    }
    if live != was {
        let what = if live { UnseenModUNSEENLOGItemWhat::On } else { UnseenModUNSEENLOGItemWhat::Off };
        g.m.unseen.unseen_log.push(UnseenModUNSEENLOGItem { t: now, what, why });
        if g.m.unseen.unseen_log.len() > 200 {
            g.m.unseen.unseen_log.drain(0..100);
        }
        if live {
            g.stats.unseen.spells += 1;
            let k = crate::seats::view_key(g);
            cue(g, k);
        } else {
            state_of(g).hold = None;
        }
    }
    live
}

/// said once as a spell starts, by the first man following; `pl` is the key of the spell's row in `states`, not always a man (a cast's is CAST_KEY) (unseen.ts cue)
pub fn cue(g: &mut Game, pl: PlayerId) {
    let now = g.time;
    if now - g.m.unseen.states.get(pl).unwrap().cue_t < g.k.unseen.cue_gap {
        return;
    }
    let mut who = None;
    for m in crate::orders::squad_orders_squad(g) {
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
    let low = g.pl[sam].crouch || on_wall(g, sam);
    crate::game::game_say(g, gd, if low { "low and quiet." } else { "slow and quiet." }, None, None, None);
}

/// this teammate is unseen this frame (unseen.ts unseenMan)
pub fn unseen_man(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| mate(g, gd) && unseen_live(g) && !entering(g, gd))
}

/// he is one of a door tool's men right now: that work is the door's own (unseen.ts entering)
pub fn entering(g: &mut Game, gd: GuardId) -> bool {
    g.door_tools.ops.iter().any(|&op| g.tab.doortools_door_op[op].men.contains(&gd))
}

/// Sam on a wall, and no place on it for this man right now (unseen.ts offWall)
pub fn off_wall(g: &mut Game, gd: GuardId) -> bool {
    let pl = lead(g);
    on_wall(g, pl) && (g.m.unseen.on_wall_at.get(gd).copied() != Some(g.time) || g.m.unseen.kneel_at.get(gd).copied() == Some(g.time))
}

/// (unseen.ts lowNow)
pub fn low_now(g: &mut Game, gd: GuardId) -> bool {
    let pl = lead(g);
    g.pl[pl].crouch || off_wall(g, gd)
}

/// he is low because Sam is (unseen.ts unseenLow)
pub fn unseen_low(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| unseen_man(g, gd) && low_now(g, gd))
}

// ------------------------------------------------------------ the follower's seams (orders.ts tick)

/// A follower's legs and knee this frame; None: the file's own (unseen.ts unseenFollow)
pub fn unseen_follow(g: &mut Game, gd: GuardId, sam_v: f64, d: f64, walk: f64, speed0: f64) -> Option<UnseenFollow> {
    in_view(g, gd, &mut |g: &mut Game| {
        if !unseen_man(g, gd) {
            return None;
        }
        let low = low_now(g, gd);
        let k = &g.k.unseen;
        let top = if low { m::min(k.low_pace, walk) } else { walk };
        Some(UnseenFollow { speed: clamp(sam_v + k.gain * (d - 0.3), m::min(k.min_pace, top), top), crouch: low })
    })
}

/// a walk to a mark an order gave him: the pace he keeps to, low; None: the order's own carriage (unseen.ts unseenPost)
pub fn unseen_post(g: &mut Game, gd: GuardId, order: &str) -> Option<f64> {
    in_view(g, gd, &mut |g: &mut Game| if POSTED.contains(&order) && unseen_low(g, gd) { Some(g.k.unseen.low_pace) } else { None })
}

/// A follower's place while Sam is on a wall; None: his place in the file as ever (unseen.ts unseenWallSlot)
pub fn unseen_wall_slot(g: &mut Game, gd: GuardId, back: f64, file: &[GuardId]) -> Option<V3> {
    in_view(g, gd, &mut |g: &mut Game| {
        let q = wall_slot(g, gd, back, file);
        if q.is_some() {
            g.stats.unseen.wall_places += 1;
            let now = g.time;
            g.m.unseen.on_wall_at.set(gd, now);
            let pl = lead(g);
            let wall = seam_wall_serial(g, pl);
            g.m.unseen.placed.set(gd, wall);
        } else {
            g.m.unseen.placed.delete(gd);
        }
        q
    })
}

/// the place just dealt was not taken after all: he is in the open, so on a knee (unseen.ts unseenWallLost)
pub fn unseen_wall_lost(g: &mut Game, gd: GuardId) {
    g.m.unseen.on_wall_at.delete(gd);
    g.m.unseen.placed.delete(gd);
    g.stats.unseen.wall_lost += 1;
}

/// the place itself: along Sam's face, on the file's side, a body's width off it (unseen.ts wallSlot)
pub fn wall_slot(g: &mut Game, gd: GuardId, back: f64, file: &[GuardId]) -> Option<V3> {
    if !unseen_man(g, gd) {
        return None;
    }
    let pl = lead(g);
    if g.pl[pl].wall.is_none() || !on_wall(g, pl) {
        return None;
    }
    let (t, n, w_d, w_lo, w_hi, w_side, lean_off) = {
        let w = g.pl[pl].wall.as_ref().unwrap();
        (w.along, w.n, w.d, w.lo, w.hi, w.side, w.lean_off)
    };
    let sam = g.ch[g.pl[pl].ch].pos;
    let su = sam[0] * t[0] + sam[2] * t[2];
    let wall = seam_wall_serial(g, pl);
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
        g.stats.unseen.wall_past += 1;
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

/// the most his legs may be asked for; None: no cap of this file's (unseen.ts unseenPace)
pub fn unseen_pace(g: &mut Game, gd: GuardId) -> Option<f64> {
    in_view(g, gd, &mut |g: &mut Game| if g.k.unseen.cap > 0.0 && unseen_man(g, gd) { Some(g.k.unseen.cap) } else { None })
}

/// a footfall's level; None: his usual (unseen.ts unseenStep)
pub fn unseen_step(g: &mut Game, gd: GuardId) -> Option<f64> {
    in_view(g, gd, &mut |g: &mut Game| {
        if !unseen_man(g, gd) {
            return None;
        }
        let c = g.gd[gd].ch;
        Some(if g.ch[c].anim.crouch > 0.5 { g.k.unseen.low_step } else { g.k.unseen.walk_step })
    })
}

/// his light is held off because Sam's is: UNSEEN.lights is off on the page, so never (unseen.ts unseenDark)
pub fn unseen_dark(g: &mut Game, gd: GuardId) -> bool {
    false
}

/// (unseen.ts unseenHush)
pub fn unseen_hush(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| unseen_man(g, gd))
}

/// (unseen.ts unseenEase)
pub fn unseen_ease(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| unseen_man(g, gd))
}

/// his round at `h` is held: nobody has found the squad, and `h` has not noticed anybody. `urgent` runs in the caller's game, not the view, as its closure does (unseen.ts unseenHolds)
pub fn unseen_holds(g: &mut Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut Game) -> bool) -> bool {
    if quiet || !in_view(g, gd, &mut |g: &mut Game| unseen_man(g, gd) && !crate::roomfight::room_fight_hot_for(g, gd)) {
        return false;
    }
    let state = g.gd[h].state;
    if state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search || g.gd[h].awareness >= noticed || urgent(g) {
        return false;
    }
    g.stats.unseen.held += 1;
    true
}

/// through standing glass the hold is the silent order's own: is that rule his this frame (unseen.ts unseenGlass)
pub fn unseen_glass(g: &mut Game, gd: GuardId) -> bool {
    in_view(g, gd, &mut |g: &mut Game| unseen_man(g, gd) && !crate::roomfight::room_fight_hot_for(g, gd))
}

/// silent.ts's glass hold held a round under this rule (unseen.ts unseenGlassHeld)
pub fn unseen_glass_held(g: &mut Game) {
    g.stats.unseen.held_glass += 1;
}

// ------------------------------------------------------------ the HUD's word, QA

/// the squad line's word beside the posture (None: nothing to say) (unseen.ts unseenWord)
pub fn unseen_word(g: &mut Game) -> Option<String> {
    if unseen_live(g) { Some(String::from("sneaking")) } else { None }
}

/// (unseen.ts unseenSummary)
pub fn unseen_summary(g: &mut Game) -> String {
    let first = if unseen_live(g) {
        String::from("sneaking with the player")
    } else {
        alloc::format!("not sneaking ({})", state_of(g).why)
    };
    let st = &g.stats.unseen;
    alloc::format!(
        "{} · spells {} · held {} · held for glass {} · wall places {} ({} handed back to a door's lane)",
        first, st.spells, st.held, st.held_glass, st.wall_places, st.wall_lost
    )
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
        let pl = g.pl.push(crate::game::Player::blank());
        g.player = pl;
        (g, pl)
    }

    #[test]
    fn a_wall_counts_once_his_back_has_been_on_it_a_beat() {
        let (mut g, pl) = game_with_sam();
        assert!(!on_wall(&mut g, pl));
        g.pl[pl].wall = Some(crate::player::WallHold::zeroed());
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
    fn beside_a_wall_a_man_with_no_place_this_frame_is_off_it() {
        let (mut g, pl) = game_with_sam();
        let gd = GuardId(0);
        g.time = 5.0;
        assert!(!off_wall(&mut g, gd));   // Sam on no wall
        g.pl[pl].wall = Some(crate::player::WallHold::zeroed());
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
