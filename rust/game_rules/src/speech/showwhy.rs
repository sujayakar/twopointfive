pub use ::game_types::showwhy::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec::Vec};

// SHOWWHY_LINES: the lines, one place.
const LINE_LINE: &str = "you're in my line.";
const LINE_DOOR: &str = "after you.";
const LINE_HURT: &str = "I'm hit.";
const LINE_LAG: &str = "I can't keep up.";
const LINE_KEEP: &str = "I've got this door.";

/// A man's memory, all void.
fn blank(t: f64) -> crate::showwhy::Mem {
    Mem {
        t,
        line_t0: -100.0,
        line_seen_t: -100.0,
        line_on: false,
        line_end_t: -100.0,
        wave_t: -100.0,
        waves: 0,
        line_b: f64::NAN,
        door_t0: -100.0,
        door_seen_t: -100.0,
        door_shown: false,
        door_t: -100.0,
        hurt_said: -1.0,
        lag_t: -100.0,
        lag_n: 0,
        keep_t0: f64::NAN,
        keep_said: false,
        keep_said_t: -100.0,
        owed: Vec::new(),
    }
}

/// Nothing shown yet.
fn fresh() -> crate::showwhy::ShowWhyStats {
    ShowWhyStats { waves: 0, squeezes: 0, fists: 0, ways: 0, lines: 0, holds: 0 }
}

/// This seat's state, made if there is none: the row itself.
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::showwhy::State {
    let (key, t) = (crate::seats::view_key(g, &crate::seats::here(g)), g.time);
    g.m.showwhy.states.get_or_insert_with(key, || State { stats: fresh(), after_you_t: -100.0, keep_say_t: -100.0, line_say_t: -100.0, t })
}

/// The row `st` of the TypeScript, by its key; the tick made it before anything here runs.
fn st_row(g: &mut Game, pl: PlayerId) -> &mut crate::showwhy::State {
    g.m.showwhy.states.get_mut(pl).unwrap()
}

/// Distance on the floor.
fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// One row of the log, which keeps its last 120 to 200.
fn note(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, why: crate::showwhy::Why, signal: Option<crate::character::SignalKind>, line: Option<&str>) {
    let st = st_row(g, pl);
}

/// His words are over him this frame, asked of the bubble's clock.
fn speaking(g: &crate::game::Game, gd: GuardId) -> bool {
    match &g.gd[gd].bubble {
        Some(b) => g.time >= b.t && g.time - b.t <= b.dur,
        None => false,
    }
}

/// His line through the game's own say; false where it did not land (a man with no voice).
fn say(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, why: crate::showwhy::Why, text: &str) -> bool {
    crate::game::game_say(g, gd, text, None, None);
    if g.gd[gd].bubble.as_ref().map(|b| b.text.as_str()) != Some(text) {
        return false;
    }
    st_row(g, pl).stats.lines += 1;
    note(g, pl, gd, why, None, Some(text));
    true
}

/// His free hand's signal, and its row in the log.
fn sign(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, why: crate::showwhy::Why, kind: crate::character::SignalKind, side: Option<f64>) {
    let side = side.unwrap_or(1.0);
    let c = g.gd[gd].ch;
    crate::character::anim_signal(g, c, kind, Some(side));
    note(g, pl, gd, why, Some(kind), None);
}

/// `g.fight.opOf(gd)?.tgt`: the man the room fight has him on.
fn op_tgt(g: &Game, gd: GuardId) -> Option<GuardId> {
    let o = crate::roomfight::room_fight_op_of(g, gd)?;
    g.tab.roomfight_op[o].tgt
}

/// The player is in this man's line: the room fight held his round this frame for the player, no mate is nearer in it, and no fire hold has the round anyway.
fn masked(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if crate::condition::down(g, g.pl[g.player].ch) {
        return false;
    }
    let o = match crate::roomfight::room_fight_op_of(g, gd) {
        Some(o) => o,
        None => return false,
    };
    let h = match g.tab.roomfight_op[o].tgt {
        Some(h) => h,
        None => return false,
    };
    let r = &g.tab.roomfight_op[o];
    if crate::condition::down(g, g.gd[h].ch)
        || !(r.line_t > 0.0)
        || !r.line_by.is_some_and(|f| crate::character::is_player(g, f))
        || g.time - r.line_at > 0.1
    {
        return false;
    }
    let to = g.ch[g.gd[h].ch].pos;
    if mate_nearer(g, &crate::seats::here(g), gd, to) {
        return false;
    }
    let hs = g.gd[h].state;
    if hs == crate::game::GuardState::Alert || hs == crate::game::GuardState::Search || crate::roomfight::room_fight_hot_for(g, gd) {
        return true;
    }
    !(crate::silent::silent_holds_target(g, gd) || crate::holdfire::hold_fire_holds_target(g, gd) || crate::unseen::under_unseen(g, gd))
}

/// A mate stands in this man's line to `to` nearer to him than the player does.
fn mate_nearer(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, to: V3) -> bool {
    let c = g.ch[g.gd[gd].ch].pos;
    let pl = g.ch[g.pl[seat.player].ch].pos;
    let dx = to[0] - c[0];
    let dz = to[2] - c[2];
    let l = m::hyp2(dx, dz);
    if l < 1e-3 {
        return false;
    }
    let ux = dx / l;
    let uz = dz / l;
    let ap = (pl[0] - c[0]) * ux + (pl[2] - c[2]) * uz;
    let team = g.gd[gd].team;
    let half = g.k.showwhy.mate_half;
    for &o in &g.guards {
        if o == gd || crate::condition::down(g, g.gd[o].ch) || g.gd[o].team != team {
            continue;
        }
        let op = g.ch[g.gd[o].ch].pos;
        let x = op[0] - c[0];
        let z = op[2] - c[2];
        let a = x * ux + z * uz;
        if a > 0.2 && a < ap && m::abs(x * uz - z * ux) < half {
            return true;
        }
    }
    false
}

/// The side of his line to his man the player stands on: 1 his left, -1 his right.
fn line_side(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> f64 {
    let h = op_tgt(g, gd);
    let c = g.ch[g.gd[gd].ch].pos;
    let pl = g.ch[g.pl[seat.player].ch].pos;
    let h = match h {
        Some(h) => h,
        None => return 1.0,
    };
    let hp = g.ch[g.gd[h].ch].pos;
    let dx = hp[0] - c[0];
    let dz = hp[2] - c[2];
    if ((pl[0] - c[0]) * dz - (pl[2] - c[2]) * dx) / m::max(1e-3, m::hyp2(dx, dz)) >= -0.15 {
        1.0
    } else {
        -1.0
    }
}

/// A mate has "you're in my line." up as this rule's right now.
fn line_said_by(g: &crate::game::Game, but: GuardId) -> bool {
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == but || crate::condition::down(g, g.gd[o].ch) {
            continue;
        }
        let up = match (g.m.showwhy.mem.get(o), &g.gd[o].bubble) {
            (Some(mm), Some(b)) => mm.line_on && b.t == mm.line_b && b.text == LINE_LINE,
            _ => false,
        };
        if up && speaking(g, o) {
            return true;
        }
    }
    false
}

/// A door order that is made and waiting on something that is not its own next step.
fn waits(g: &mut crate::game::Game, op: DoorOpId) -> bool {
    let r = &g.tab.doortools_door_op[op];
    if r.word.is_some() || r.hold == 2.0 {
        return false;
    }
    match r.phase {
        crate::doortools::Phase::Ready | crate::doortools::Phase::Armed => true,
        crate::doortools::Phase::Stack => !r.ready_t.is_nan(),
        crate::doortools::Phase::Thr => r.thr.as_ref().map_or(false, |x| x.told),
        _ => false,
    }
}

/// This man of the order is on his mark with his hands his own.
fn stands_free(g: &crate::game::Game, seat: &crate::seats::SeatRef, op: DoorOpId, gd: GuardId) -> bool {
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].script.is_none() || crate::control::man::blind(g, gd) {
        return false;
    }
    let a = match crate::orders::squad_orders_of(g, seat, gd) {
        Some(a) => a,
        None => return false,
    };
    if g.tab.doortools_door_op[op].mine.get(gd).copied() != Some(a) || !g.tab.orders_assignment[a].arrived {
        return false;
    }
    if op_tgt(g, gd).is_some() {
        return false;
    }
    let c = g.gd[gd].ch;
    crate::character::anim_signal_free(g, c)
}

/// A mate of the order in reach of his hand ahead of him, nearer the door and in front of his chest.
fn man_ahead(g: &crate::game::Game, op: DoorOpId, gd: GuardId) -> bool {
    let reach = g.k.showwhy.squeeze_reach;
    let c = g.gd[gd].ch;
    let (pos, yaw) = (g.ch[c].pos, g.ch[c].body_yaw);
    let door = g.dr[g.tab.doortools_door_op[op].door].frame_centre;
    let d0 = flat(pos, door);
    for &o in &g.tab.doortools_door_op[op].men {
        if o == gd || crate::condition::down(g, g.gd[o].ch) {
            continue;
        }
        let o_pos = g.ch[g.gd[o].ch].pos;
        let d = flat(pos, o_pos);
        if d < 0.3 || d > reach || flat(o_pos, door) > d0 - 0.1 {
            continue;
        }
        if m::abs(v3::wrap_angle(m::atan2(o_pos[0] - pos[0], o_pos[2] - pos[2]) - yaw)) < 1.0 {
            return true;
        }
    }
    false
}

/// STACK: the squeezes up the file of each order made and waiting, then the lead's fist.
fn stacks(g: &mut crate::game::Game, pl: PlayerId) {
    let k = g.k.showwhy;
    let t = g.time;
    let mut i = 0;
    while i < g.door_tools.ops.len() {
        let op = g.door_tools.ops[i];
        i += 1;
        if g.tab.doortools_door_op[op].men.is_empty() {
            continue;
        }
        let mut all = true;
        let mut j = 0;
        while j < g.tab.doortools_door_op[op].men.len() {
            let man = g.tab.doortools_door_op[op].men[j];
            j += 1;
            if !crate::orders::squad_orders_mine(g, &crate::seats::here(g), man) {
                all = false;
                break;
            }
        }
        if !all {
            continue;
        }
        if !waits(g, op) {
            g.m.showwhy.op_mem.delete(op);
            continue;
        }
        let begins = match g.m.showwhy.op_mem.get(op) {
            Some(r) => t < r.t,
            None => true,
        };
        if begins {
            // it waits from now (or the clock went back under it)
            g.m.showwhy.op_mem.set(op, OpMem { t, turns: Vec::new(), fist_t: 0.0, fists: 0 });
            st_row(g, pl).stats.holds += 1;
            let door = g.dr[g.tab.doortools_door_op[op].door].frame_centre;
            let mut at = t + 0.15;
            let lead = g.tab.doortools_door_op[op].lead;
            // the last man first (the distance is read once a man: the comparer's own numbers, nothing moves during the sort)
            let mut file: Vec<(GuardId, f64)> = Vec::new();
            for &man in &g.tab.doortools_door_op[op].men {
                if man != lead && !crate::condition::down(g, g.gd[man].ch) {
                    file.push((man, flat(g.ch[g.gd[man].ch].pos, door)));
                }
            }
            js::sort_by(&mut file, |a, b| b.1 - a.1);
            if js::truthy(g.k.moves.squeeze) {
                for &(man, _) in &file {
                    if man_ahead(g, op, man) {
                        g.m.showwhy.op_mem.get_mut(op).unwrap().turns.push(OpMemTurnsItem { gd: man, at });
                        at += k.squeeze_gap;
                    }
                }
            }
            g.m.showwhy.op_mem.get_mut(op).unwrap().fist_t = at + k.fist_after;
        }
        g.m.showwhy.op_mem.get_mut(op).unwrap().t = t;
        loop {
            let due = {
                let r = g.m.showwhy.op_mem.get_mut(op).unwrap();
                if !r.turns.is_empty() && t >= r.turns[0].at { Some(r.turns.remove(0)) } else { None }
            };
            let turn = match due {
                Some(x) => x,
                None => break,
            };
            if stands_free(g, &crate::seats::here(g), op, turn.gd) && man_ahead(g, op, turn.gd) {
                sign(g, pl, turn.gd, Why::Stack, crate::character::SignalKind::Squeeze, None);
                st_row(g, pl).stats.squeezes += 1;
            }
        }
        let (none_left, fists, fist_t) = {
            let r = g.m.showwhy.op_mem.get(op).unwrap();
            (r.turns.is_empty(), r.fists, r.fist_t)
        };
        if none_left && (fists as f64) < k.hold_max && t >= fist_t {
            let lead = g.tab.doortools_door_op[op].lead;
            if stands_free(g, &crate::seats::here(g), op, lead) {
                sign(g, pl, lead, Why::Stack, crate::character::SignalKind::Hold, None);
                st_row(g, pl).stats.fists += 1;
                let r = g.m.showwhy.op_mem.get_mut(op).unwrap();
                r.fists += 1;
                r.fist_t = t + k.hold_every * fist_doubling(r.fists);
            } else {
                // his hands are busy, or the fight has him: looked at again shortly
                g.m.showwhy.op_mem.get_mut(op).unwrap().fist_t = t + 0.5;
            }
        }
    }
}

/// `1 << Math.min(fists - 1, 30)`: JavaScript's shift of a 32-bit one by the count's low five bits.
fn fist_doubling(fists: i32) -> f64 {
    (1i32 << (core::cmp::min(fists - 1, 30) & 31)) as f64
}

/// Once a frame, after the orders, the door tools and the room fight have had theirs.
pub fn show_why_tick(g: &mut crate::game::Game) {
    let k = g.k.showwhy;
    if !crate::orders::squad_orders_sams(g, &crate::seats::here(g)) || g.puppet.is_some() {
        return;
    }
    let (t, pl) = (g.time, g.player);
    let st = state_of(g);
    if t < st.t {
        // the clock went back: a restart, a load
        st.after_you_t = -100.0;
        st.keep_say_t = -100.0;
        st.line_say_t = -100.0;
    }
    st.t = t;
    stacks(g, pl);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) || crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].script.is_none() || g.gd[gd].held.is_some() {
            continue;
        }
        // a man no order of the player's poses is his drill's
        let a = match crate::orders::squad_orders_of(g, &crate::seats::here(g), gd) {
            Some(a) => a,
            None => continue,
        };
        let void = match g.m.showwhy.mem.get(gd) {
            Some(r) => t < r.t,
            None => true,
        };
        if void {
            g.m.showwhy.mem.set(gd, blank(t));
        }
        g.m.showwhy.mem.get_mut(gd).unwrap().t = t;
        let c = g.gd[gd].ch;
        let quiet = !speaking(g, gd);

        // ---- LINE
        let line_now = masked(g, gd);
        {
            let r = g.m.showwhy.mem.get_mut(gd).unwrap();
            if line_now {
                if t - r.line_seen_t > k.line_gap {
                    r.line_t0 = t;
                }
                r.line_seen_t = t;
            }
            if !r.line_on && line_now && t - r.line_t0 >= k.line_after {
                r.line_on = true;
                if t - r.line_end_t > k.wave_later + 2.0 {
                    r.waves = 0;
                }
                r.wave_t = m::max(t, r.wave_t);
                r.line_b = f64::NAN;
            }
        }
        if g.m.showwhy.mem.get(gd).unwrap().line_on {
            let (line_b, line_seen_t) = {
                let r = g.m.showwhy.mem.get(gd).unwrap();
                (r.line_b, r.line_seen_t)
            };
            let mine = g.gd[gd].bubble.as_ref().map_or(false, |b| b.t == line_b && b.text == LINE_LINE);
            if t - line_seen_t > k.line_gap {
                // the line is his again: the words go with it (having stood long enough to be read)
                let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                r.line_on = false;
                r.line_end_t = t;
                r.line_b = f64::NAN;
                if mine {
                    let b = g.gd[gd].bubble.as_mut().unwrap();
                    b.dur = m::max(k.line_min, t - b.t + 0.35);
                }
            } else {
                if t >= g.m.showwhy.mem.get(gd).unwrap().wave_t && crate::character::anim_signal_free(g, c) {
                    let side = line_side(g, &crate::seats::here(g), gd);
                    sign(g, pl, gd, Why::Line, crate::character::SignalKind::Aside, Some(side));
                    st_row(g, pl).stats.waves += 1;
                    let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                    r.waves += 1;
                    r.wave_t = t + if r.waves < 2 { k.wave_every } else { k.wave_later };
                }
                if mine {
                    // it stands while it is true
                    let b = g.gd[gd].bubble.as_mut().unwrap();
                    b.dur = m::max(b.dur, t - b.t + 0.6);
                } else if quiet && t >= st_row(g, pl).line_say_t && !line_said_by(g, gd) && say(g, pl, gd, Why::Line, LINE_LINE) {
                    // one voice: a mate who has it up says it for all of them
                    let bt = g.gd[gd].bubble.as_ref().unwrap().t;
                    g.m.showwhy.mem.get_mut(gd).unwrap().line_b = bt;
                    st_row(g, pl).line_say_t = t + k.line_say_gap;
                }
            }
        }

        // ---- DOOR (never in a fight)
        let pc = g.pl[pl].ch;
        let held_now = !g.tab.orders_assignment[a].arrived
            && !crate::condition::down(g, g.pl[pl].ch)
            && crate::doorway::doorway_held_by(g, gd).as_deref() == Some("Player")
            && m::hyp2(g.ch[pc].vel[0], g.ch[pc].vel[2]) < 0.6
            && op_tgt(g, gd).is_none()
            && !crate::roomfight::room_fight_hot_for(g, gd);
        if held_now {
            let r = g.m.showwhy.mem.get_mut(gd).unwrap();
            if t - r.door_seen_t > 0.3 {
                r.door_t0 = t;
                r.door_shown = false;
            }
            r.door_seen_t = t;
        }
        let may_show = {
            let r = g.m.showwhy.mem.get(gd).unwrap();
            held_now && !r.door_shown && t - r.door_t0 >= k.door_after && t >= r.door_t
        };
        if may_show && crate::character::anim_signal_free(g, c) {
            {
                let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                r.door_shown = true;
                r.door_t = t + k.door_every;
            }
            sign(g, pl, gd, Why::Door, crate::character::SignalKind::Go, None);
            st_row(g, pl).stats.ways += 1;
            if t >= st_row(g, pl).after_you_t {
                owe(g.m.showwhy.mem.get_mut(gd).unwrap(), LINE_DOOR, Why::Door, t + 1.5);
            }
        }

        // ---- HURT (a line is spent when it is SAID, never when it is queued)
        if crate::condition::is_hurt(g, g.gd[gd].ch, None) {
            let hurt_at = g.cond[g.gd[gd].ch].hurt_at;
            let stagger_until = g.cond[g.gd[gd].ch].fold_until;
            let (hurt_said, lag_n, lag_t) = {
                let r = g.m.showwhy.mem.get(gd).unwrap();
                (r.hurt_said, r.lag_n, r.lag_t)
            };
            if hurt_said != hurt_at {
                let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                if t - hurt_at > k.hurt_late {
                    // too long ago to say now
                    r.hurt_said = hurt_at;
                    r.lag_t = t;
                    r.lag_n = 0;
                } else if t >= stagger_until {
                    owe(r, LINE_HURT, Why::Hurt, t + k.owed_for);
                }
            } else if crate::orders::rows::is(g, a, crate::orders::Is::Follows)
                && (lag_n as f64) < k.lag_max
                && t >= lag_t
                && g.pl[pl].sprinting
                && !crate::condition::down(g, g.pl[pl].ch)
                && flat(g.ch[c].pos, g.ch[pc].pos) > k.lag_m
            {
                owe(g.m.showwhy.mem.get_mut(gd).unwrap(), LINE_LAG, Why::Hurt, t + 3.0);
            }
        }

        // ---- KEEP
        if crate::rearwatch::room_kept(g, &crate::seats::here(g), gd) {
            let r = g.m.showwhy.mem.get_mut(gd).unwrap();
            if r.keep_t0.is_nan() {
                r.keep_t0 = t;
                r.keep_said = false;
            }
            if !r.keep_said && t - r.keep_t0 >= k.keep_after && t - r.keep_said_t >= k.keep_every {
                owe(r, LINE_KEEP, Why::Keep, t + k.owed_for);
            }
        } else {
            g.m.showwhy.mem.get_mut(gd).unwrap().keep_t0 = f64::NAN;
        }

        // ---- the lines owed, the oldest first: said once his own words are spent, each only while its cause still stands
        loop {
            let (text, why, by, line_on, keep_said, door_seen_t) = {
                let r = g.m.showwhy.mem.get(gd).unwrap();
                if r.owed.is_empty() {
                    break;
                }
                (r.owed[0].text.clone(), r.owed[0].why, r.owed[0].by, r.line_on, r.keep_said, r.door_seen_t)
            };
            let late = text == LINE_HURT && t - g.cond[g.gd[gd].ch].hurt_at > k.hurt_late;
            if line_on && !late {
                let o = &mut g.m.showwhy.mem.get_mut(gd).unwrap().owed[0];
                o.by = m::max(o.by, t + 1.0);
                break;
            }
            let stands = match why {
                Why::Keep => crate::rearwatch::room_kept(g, &crate::seats::here(g), gd) && !keep_said,
                Why::Door => t - door_seen_t <= 0.5 && t >= st_row(g, pl).after_you_t,
                _ => crate::condition::is_hurt(g, g.gd[gd].ch, None) && !late,
            };
            if t > by || !stands {
                g.m.showwhy.mem.get_mut(gd).unwrap().owed.remove(0);
                continue;
            }
            if !quiet || (why == Why::Keep && t < st_row(g, pl).keep_say_t) {
                break;
            }
            g.m.showwhy.mem.get_mut(gd).unwrap().owed.remove(0);
            if say(g, pl, gd, why, &text) {
                if why == Why::Door {
                    st_row(g, pl).after_you_t = t + k.after_you_gap;
                } else if why == Why::Keep {
                    st_row(g, pl).keep_say_t = t + k.keep_gap;
                    let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                    r.keep_said_t = t;
                    r.keep_said = true;
                } else if text == LINE_HURT {
                    let hurt_at = g.cond[g.gd[gd].ch].hurt_at;
                    let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                    r.hurt_said = hurt_at;
                    r.lag_t = t + 3.0;
                    r.lag_n = 0;
                } else {
                    let r = g.m.showwhy.mem.get_mut(gd).unwrap();
                    r.lag_t = t + k.lag_every;
                    r.lag_n += 1;
                }
            }
            break;
        }
    }
}

/// A line for this man to say when he can; two at the most wait.
fn owe(m: &mut crate::showwhy::Mem, text: &str, why: crate::showwhy::Why, by: f64) {
    if m.owed.len() < 2 && !m.owed.iter().any(|o| o.text == text) {
        m.owed.push(Owed { text: String::from(text), why, by });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A FAILURE MEANS: a man nags: the same line twice, or more waiting than he will ever get to say.
    #[test]
    fn two_lines_at_most_wait_and_never_the_same_twice() {
        let mut r = blank(0.0);
        owe(&mut r, LINE_DOOR, Why::Door, 1.0);
        owe(&mut r, LINE_DOOR, Why::Door, 2.0);
        assert_eq!(r.owed.len(), 1);
        assert_eq!(r.owed[0].by, 1.0);
        owe(&mut r, LINE_HURT, Why::Hurt, 3.0);
        owe(&mut r, LINE_KEEP, Why::Keep, 4.0);
        assert_eq!(r.owed.len(), 2);
        assert_eq!(r.owed[1].text, LINE_HURT);
    }

    /// A FAILURE MEANS: a man waves at you without end: each raised fist waits twice the last, up to a cap that cannot overflow.
    #[test]
    fn the_fists_space_out_by_doubling() {
        assert_eq!(fist_doubling(1), 1.0);
        assert_eq!(fist_doubling(2), 2.0);
        assert_eq!(fist_doubling(3), 4.0);
        assert_eq!(fist_doubling(100), 1073741824.0);
    }

    /// A FAILURE MEANS: a man on the floor above counts as far off: every range here is on the floor.
    #[test]
    fn a_range_is_on_the_floor() {
        assert_eq!(flat([0.0, 0.0, 0.0], [3.0, 9.0, 4.0]), 5.0);
    }
}
