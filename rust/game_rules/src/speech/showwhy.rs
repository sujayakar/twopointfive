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

/// This seat's state, made if there is none: the row itself.
fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::showwhy::State {
    let (key, t) = (crate::seats::view_key(g, &crate::seats::here(g)), g.time);
    g.m.showwhy.states.get_or_insert_with(key, || State { after_you_t: -100.0, keep_say_t: -100.0, line_say_t: -100.0, t })
}

/// The row `st` of the TypeScript, by its key; the tick made it before anything here runs.
fn st_row(g: &mut Game, pl: PlayerId) -> &mut crate::showwhy::State {
    g.m.showwhy.states.get_mut(pl).unwrap()
}

/// Distance on the floor.
fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// His words are over him this frame, asked of the bubble's clock.
fn speaking(g: &crate::game::Game, gd: GuardId) -> bool {
    match &g.gd[gd].bubble {
        Some(b) => g.time >= b.t && g.time - b.t <= b.dur,
        None => false,
    }
}

/// His line through the game's own say; false where it did not land (a man with no voice).
fn say(g: &mut crate::game::Game, gd: GuardId, text: &str) -> bool {
    crate::game::game_say(g, gd, text, None, None);
    if g.gd[gd].bubble.as_ref().map(|b| b.text.as_str()) != Some(text) {
        return false;
    }
    true
}

/// His free hand's signal, and its row in the log.
fn sign(g: &mut crate::game::Game, gd: GuardId, kind: crate::character::SignalKind, side: Option<f64>) {
    let side = side.unwrap_or(1.0);
    let c = g.gd[gd].ch;
    crate::character::anim_signal(g, c, kind, Some(side));
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
    let team = g.team(gd);
    let half = g.k.showwhy.mate_half;
    for &o in &g.guards {
        if o == gd || crate::condition::down(g, g.gd[o].ch) || g.team(o) != team {
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
fn waits(g: &crate::game::Game, op: DoorOpId) -> bool {
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
    let a = match crate::orders::squad_orders_of(seat, gd) {
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
fn stacks(g: &mut crate::game::Game) {
    for i in 0..g.sets[g.seat].door_tools.ops.len() {
        let op = g.sets[g.seat].door_tools.ops[i];
        let men = g.tab.doortools_door_op[op].men.clone();
        if men.is_empty() || !men.iter().all(|&gd| crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd)) { continue; }
        if !waits(g, op) { g.m.showwhy.op_mem.delete(op); continue; }
        let t = g.time;
        if g.m.showwhy.op_mem.get(op).is_none_or(|r| t < r.t) { stack_begins(g, op, &men); }   // (it waits from now, or the clock went back under it)
        op_mem(g, op).t = t;
        squeezes(g, op);
        fist(g, op);
    }
}
fn op_mem(g: &mut Game, op: DoorOpId) -> &mut OpMem { g.m.showwhy.op_mem.get_mut(op).unwrap() }
/// When each of the file will squeeze the man ahead of him, the last man first, and when the lead's fist goes up after them.
fn stack_begins(g: &mut Game, op: DoorOpId, men: &[GuardId]) {
    let (k, t) = (g.k.showwhy, g.time);
    let (lead, door) = { let o = &g.tab.doortools_door_op[op]; (o.lead, g.dr[o.door].frame_centre) };
    let mut file: Vec<(GuardId, f64)> = men.iter().filter(|&&gd| gd != lead && !crate::condition::down(g, g.gd[gd].ch)).map(|&gd| (gd, flat(g.ch[g.gd[gd].ch].pos, door))).collect();
    js::sort_by(&mut file, |a, b| b.1 - a.1);
    let mut r = OpMem { t, turns: Vec::new(), fist_t: 0.0, fists: 0 };
    let mut at = t + 0.15;
    for (gd, _) in file { if man_ahead(g, op, gd) { r.turns.push(OpMemTurnsItem { gd, at }); at += k.squeeze_gap; } }
    r.fist_t = at + k.fist_after;
    g.m.showwhy.op_mem.set(op, r);
}
fn squeezes(g: &mut Game, op: DoorOpId) {
    let t = g.time;
    while op_mem(g, op).turns.first().is_some_and(|x| t >= x.at) {
        let gd = op_mem(g, op).turns.remove(0).gd;
        if stands_free(g, &crate::seats::here(g), op, gd) && man_ahead(g, op, gd) { sign(g, gd, crate::character::SignalKind::Squeeze, None); }
    }
}
fn fist(g: &mut Game, op: DoorOpId) {
    let (k, t, lead) = (g.k.showwhy, g.time, g.tab.doortools_door_op[op].lead);
    let r = op_mem(g, op);
    if !r.turns.is_empty() || r.fists as f64 >= k.hold_max || t < r.fist_t { return; }
    if !stands_free(g, &crate::seats::here(g), op, lead) { op_mem(g, op).fist_t = t + 0.5; return; }   // (his hands are busy, or the fight has him: looked at again shortly)
    sign(g, lead, crate::character::SignalKind::Hold, None);
    let r = op_mem(g, op);
    r.fists += 1;
    r.fist_t = t + k.hold_every * fist_doubling(r.fists);
}

/// `1 << Math.min(fists - 1, 30)`: JavaScript's shift of a 32-bit one by the count's low five bits.
fn fist_doubling(fists: i32) -> f64 {
    (1i32 << (core::cmp::min(fists - 1, 30) & 31)) as f64
}

/// The memory of a man, his void one where he has none or the clock has gone back under it.
fn mem_from(g: &mut Game, gd: GuardId) {
    let t = g.time;
    if g.m.showwhy.mem.get(gd).is_none_or(|r| t < r.t) { g.m.showwhy.mem.set(gd, blank(t)); }
    mem(g, gd).t = t;
}
fn mem(g: &mut Game, gd: GuardId) -> &mut crate::showwhy::Mem { g.m.showwhy.mem.get_mut(gd).unwrap() }

/// Once a tick, after the orders, the door tools and the room fight have had theirs: what each man of his shows or says of why he does what he does.
pub fn show_why_tick(g: &mut crate::game::Game) {
    if !crate::orders::squad_orders_sams(&crate::seats::here(g)) { return; }
    let t = g.time;
    let st = state_of(g);
    if t < st.t { st.after_you_t = -100.0; st.keep_say_t = -100.0; st.line_say_t = -100.0; }   // (the clock went back: a restart, a load)
    st.t = t;
    stacks(g);
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if !crate::orders::squad_orders_mine(g, &crate::seats::here(g), gd) || crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].script.is_none() || g.gd[gd].held.is_some() { continue; }
        // (a man no order of the player's poses is his drill's)
        let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), gd) else { continue };
        mem_from(g, gd);
        let quiet = !speaking(g, gd);
        line_shown(g, gd, quiet);
        door_shown(g, gd, a);
        hurt_owed(g, gd, a);
        keep_owed(g, gd);
        owed_said(g, gd, quiet);
    }
}

/// LINE: the player has stood in his line for a while. He waves him aside, and says so; both end with it.
fn line_shown(g: &mut Game, gd: GuardId, quiet: bool) {
    let (k, t, pl, c) = (g.k.showwhy, g.time, g.player, g.gd[gd].ch);
    let now = masked(g, gd);
    let r = mem(g, gd);
    if now {
        if t - r.line_seen_t > k.line_gap { r.line_t0 = t; }
        r.line_seen_t = t;
        if !r.line_on && t - r.line_t0 >= k.line_after {
            r.line_on = true;
            if t - r.line_end_t > k.wave_later + 2.0 { r.waves = 0; }
            r.wave_t = m::max(t, r.wave_t);
            r.line_b = f64::NAN;
        }
    }
    if !r.line_on { return; }
    let (said_at, seen_t, wave_t) = (r.line_b, r.line_seen_t, r.wave_t);
    let up = g.gd[gd].bubble.as_ref().is_some_and(|b| b.t == said_at && b.text == LINE_LINE);
    if t - seen_t > k.line_gap {
        // (the line is his again: the words go with it, having stood long enough to be read)
        let r = mem(g, gd);
        r.line_on = false;
        r.line_end_t = t;
        r.line_b = f64::NAN;
        if let Some(b) = g.gd[gd].bubble.as_mut().filter(|_| up) { b.dur = m::max(k.line_min, t - b.t + 0.35); }
        return;
    }
    if t >= wave_t && crate::character::anim_signal_free(g, c) {
        let side = line_side(g, &crate::seats::here(g), gd);
        sign(g, gd, crate::character::SignalKind::Aside, Some(side));
        let r = mem(g, gd);
        r.waves += 1;
        r.wave_t = t + if r.waves < 2 { k.wave_every } else { k.wave_later };
    }
    if let Some(b) = g.gd[gd].bubble.as_mut().filter(|_| up) { b.dur = m::max(b.dur, t - b.t + 0.6); return; }   // (it stands while it is true)
    // (one voice: a mate who has it up says it for all of them)
    if quiet && t >= st_row(g, pl).line_say_t && !line_said_by(g, gd) && say(g, gd, LINE_LINE) {
        mem(g, gd).line_b = g.gd[gd].bubble.as_ref().unwrap().t;
        st_row(g, pl).line_say_t = t + k.line_say_gap;
    }
}

/// DOOR, never in a fight: the player stands in the doorway he waits at. He is waved through, and asked to go.
fn door_shown(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    let (k, t, pl) = (g.k.showwhy, g.time, g.player);
    let pc = g.pl[pl].ch;
    let held = !g.tab.orders_assignment[a].arrived
        && !crate::condition::down(g, pc)
        && crate::doorway::doorway_held_by(g, gd).as_deref() == Some("Player")
        && m::hyp2(g.ch[pc].vel[0], g.ch[pc].vel[2]) < 0.6
        && op_tgt(g, gd).is_none()
        && !crate::roomfight::room_fight_hot_for(g, gd);
    if !held { return; }
    let free = crate::character::anim_signal_free(g, g.gd[gd].ch);
    let r = mem(g, gd);
    if t - r.door_seen_t > 0.3 { r.door_t0 = t; r.door_shown = false; }
    r.door_seen_t = t;
    if r.door_shown || t - r.door_t0 < k.door_after || t < r.door_t || !free { return; }
    r.door_shown = true;
    r.door_t = t + k.door_every;
    sign(g, gd, crate::character::SignalKind::Go, None);
    if t >= st_row(g, pl).after_you_t { owe(mem(g, gd), LINE_DOOR, Why::Door, t + 1.5); }
}

/// HURT: that he is hit, once a wound and not long after it; then that he cannot keep up, when the player runs on from him.
fn hurt_owed(g: &mut Game, gd: GuardId, a: OrdersAssignmentId) {
    let (k, t, c) = (g.k.showwhy, g.time, g.gd[gd].ch);
    if !crate::condition::is_hurt(g, c, None) { return; }
    let (hurt_at, up_at) = (g.cond[c].hurt_at, g.cond[c].fold_until);
    let r = mem(g, gd);
    if r.hurt_said != hurt_at {
        // (a line is spent when it is SAID, never when it is queued; or when it is too late to say it)
        if t - hurt_at > k.hurt_late { r.hurt_said = hurt_at; r.lag_t = t; r.lag_n = 0; }
        else if t >= up_at { owe(r, LINE_HURT, Why::Hurt, t + k.owed_for); }
        return;
    }
    if r.lag_n as f64 >= k.lag_max || t < r.lag_t { return; }
    let pc = g.pl[g.player].ch;
    if crate::orders::rows::is(g, a, crate::orders::Is::Follows) && g.pl[g.player].sprinting && !crate::condition::down(g, pc) && flat(g.ch[c].pos, g.ch[pc].pos) > k.lag_m {
        owe(mem(g, gd), LINE_LAG, Why::Hurt, t + 3.0);
    }
}

/// KEEP: he has kept a door behind them for a while.
fn keep_owed(g: &mut Game, gd: GuardId) {
    let (k, t) = (g.k.showwhy, g.time);
    let kept = crate::rearwatch::room_kept(g, &crate::seats::here(g), gd);
    let r = mem(g, gd);
    if !kept { r.keep_t0 = f64::NAN; return; }
    if r.keep_t0.is_nan() { r.keep_t0 = t; r.keep_said = false; }
    if !r.keep_said && t - r.keep_t0 >= k.keep_after && t - r.keep_said_t >= k.keep_every { owe(r, LINE_KEEP, Why::Keep, t + k.owed_for); }
}

/// The lines owed, the oldest first: one is said once his own words are spent, each only while its cause still stands.
fn owed_said(g: &mut Game, gd: GuardId, quiet: bool) {
    let (k, t, pl) = (g.k.showwhy, g.time, g.player);
    while let Some(o) = mem(g, gd).owed.first().cloned() {
        let late = o.text == LINE_HURT && t - g.cond[g.gd[gd].ch].hurt_at > k.hurt_late;
        if mem(g, gd).line_on && !late { mem(g, gd).owed[0].by = m::max(o.by, t + 1.0); return; }
        let stands = match o.why {
            Why::Keep => crate::rearwatch::room_kept(g, &crate::seats::here(g), gd) && !mem(g, gd).keep_said,
            Why::Door => t - mem(g, gd).door_seen_t <= 0.5 && t >= st_row(g, pl).after_you_t,
            _ => crate::condition::is_hurt(g, g.gd[gd].ch, None) && !late,
        };
        if t > o.by || !stands { mem(g, gd).owed.remove(0); continue; }
        if !quiet || (o.why == Why::Keep && t < st_row(g, pl).keep_say_t) { return; }
        mem(g, gd).owed.remove(0);
        if say(g, gd, &o.text) { spent(g, gd, &o); }
        return;
    }
}
/// It is said: when it, or another of its kind, may be said again.
fn spent(g: &mut Game, gd: GuardId, o: &Owed) {
    let (k, t, pl) = (g.k.showwhy, g.time, g.player);
    let hurt_at = g.cond[g.gd[gd].ch].hurt_at;
    match o.why {
        Why::Door => st_row(g, pl).after_you_t = t + k.after_you_gap,
        Why::Keep => {
            st_row(g, pl).keep_say_t = t + k.keep_gap;
            let r = mem(g, gd);
            r.keep_said_t = t;
            r.keep_said = true;
        }
        _ if o.text == LINE_HURT => { let r = mem(g, gd); r.hurt_said = hurt_at; r.lag_t = t + 3.0; r.lag_n = 0; }
        _ => { let r = mem(g, gd); r.lag_t = t + k.lag_every; r.lag_n += 1; }
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
