//! The blind pockets: a can for the floor nobody has had a muzzle across, then the last burst's own count, and in.
use super::*;

/// What is thrown and by whom: his pocket (or the middle of a room with none), whether it is the can that goes before the first step, and whether the
/// middle will do.
struct Can { th: GuardId, pick: DoorToolsPocketOf, first: bool, blank: bool }

pub(super) fn step_pocket(g: &mut Game, op: DoorOpId, cx: &Cx) {
    keep_open(g, cx.d);
    door_tools_thr_men(g, op);
    door_tools_thr_watch(g, op);
    if thr!(g, op).first == Some(ThresholdFirst::Burst) {
        if door_tools_burst_dry(g, op) { door_tools_thr_out(g, op, None); }
        return;
    }
    if cx.t < thr!(g, op).next_t || men_waited_for(g, op, cx.t) { return; }
    let th = match thr!(g, op).thrower { Some(x) => x, None => { let x = door_tools_thr_thrower(g, op); thr!(g, op).thrower = Some(x); x } };
    let Some(can) = can_for_pocket(g, op, cx, th) else { return };
    if leaf_shut(g, op, cx, &can) || hand_waited_for(g, op, cx.t, &can) { return; }
    if !door_tools_take(g, crate::doortools::DoorToolsCan::Bang) { return; }
    thrown(g, op, cx, &can);
}

fn hand_w(g: &Game, op: DoorOpId) -> f64 { o!(g, op).thr.as_ref().unwrap().hand_w.unwrap_or(0.0) }
/// The can is this man's to throw, who takes a moment over it; the man it was his before looks to the rear, unless he is at a jamb.
fn thrower_is(g: &mut Game, op: DoorOpId, was: Option<GuardId>, now: GuardId, t: f64) {
    if let Some(w) = was { if !thr!(g, op).jamb.contains(&w) { o!(g, op).rear.add(w); } }
    o!(g, op).rear.delete(now);
    let turn = g.k.doortools.thr_turn;
    let x = thr!(g, op);
    x.thrower = Some(now);
    x.next_t = t + turn;
}

/// Men who ran from a grenade are waited for, unless a can is fresh in there. Back, the thrower is asked for again.
fn men_waited_for(g: &mut Game, op: DoorOpId, t: f64) -> bool {
    let out = thr!(g, op).fled.as_ref().map_or(0, |f| f.len());
    let fresh = js::truthy(thr!(g, op).tossed) && t - o!(g, op).bang_t < g.k.doortools.entry_cap;
    if out > 0 && !fresh { return men_out(g, op, t); }
    let x = thr!(g, op);
    if !tr(x.frag_said) { return false; }
    x.frag_said = Some(false);
    x.frag_t = None;
    if out > 0 { return false; }
    if let Some(was) = x.thrower.filter(|w| !x.jamb.contains(w)) { o!(g, op).rear.add(was); }
    let now = door_tools_thr_thrower(g, op);
    thrower_is(g, op, None, now, t);
    true
}
fn men_out(g: &mut Game, op: DoorOpId, t: f64) -> bool {
    if door_tools_thr_frag_wait(g, op) {
        let x = thr!(g, op);
        if let Some(since) = x.frag_t { x.hand_w = Some(x.hand_w.unwrap_or(0.0) + m::min(t - since, 0.1)); }
        x.frag_t = Some(t);
        x.hand_t = None;
        if !tr(x.frag_said) {
            x.frag_said = Some(true);
            let fled = x.fled.clone();
            fled_said(g, op, fled);
        }
        if door_tools_thr_frag_wait(g, op) { return true; }
    }
    let spent = hand_w(g, op) >= g.k.doortools.thr_hand_wait;
    let x = thr!(g, op);
    x.frag_t = None;
    if !tr(x.frag_said) { return false; }
    x.frag_said = Some(false);
    let why = if tr(x.hand_done) || spent { "the order's wait is spent" } else { "not waited for" };
    let fled = x.fled.clone();
    door_tools_beat(g, op, &format!("on without {} · {}", fled_names(g, &fled), why));
    false
}

/// The can he throws, if one is owed: cans enough, none of theirs in there, and a pocket worth it. Without one they go on as they are.
fn can_for_pocket(g: &mut Game, op: DoorOpId, cx: &Cx, th: GuardId) -> Option<Can> {
    let men = o!(g, op).men.clone();
    let mate = door_tools_can_behind(g, cx.d, o!(g, op).side, &men).map(|w| if w == "you" { String::from("you are in there") } else { format!("{w} is in there") });
    let none = g.sets[g.seat].door_tools.bangs <= 0.0 || mate.is_some();
    o!(g, op).rear.delete(th);
    let most = cans_most(g, op, cx.t);
    let (tossed, force, first) = { let x = thr!(g, op); (x.tossed, x.force, x.first == Some(ThresholdFirst::Can)) };
    let blank = (force || first) && !js::truthy(tossed);
    if tossed < most && !none {
        let pick = door_tools_pocket_of(g, op, th).or(if blank { Some(DoorToolsPocketOf { at: None, n: 0.0 }) } else { None });
        if let Some(pick) = pick { return Some(Can { th, pick, first, blank }); }
    }
    if first { door_tools_thr_out(g, op, Some(mate.as_deref().unwrap_or("none left"))); return None; }
    if js::truthy(tossed) { o!(g, op).phase = Phase::Wait; return None; }
    let why = if most <= 0.0 { String::from("none to spend on a threshold") } else if let Some(mate) = mate { mate } else if none { String::from("none left") } else { String::from("no blind pocket") };
    door_tools_beat(g, op, &format!("no bang · {why}"));
    door_tools_to_step(g, op);
    None
}
/// How many cans the threshold may spend: on a last warning one more than it had, and none on top of one that is fresh.
fn cans_most(g: &mut Game, op: DoorOpId, t: f64) -> f64 {
    let x = thr!(g, op).clone();
    let r = &o!(g, op);
    if !tr(r.last_hold) { return m::max(door_tools_thr_cans(g, &crate::seats::here(g), &x), if x.force { 1.0 } else { 0.0 }); }
    if x.tossed > 0.0 && !tr(x.wide) && t - r.bang_t < g.k.doortools.thr_last_fresh { x.tossed } else { r.last_tossed.unwrap_or(0.0) + 1.0 }
}

/// The door has come shut on them since: shoved, and waited for a moment; then in without.
fn leaf_shut(g: &mut Game, op: DoorOpId, cx: &Cx, can: &Can) -> bool {
    let (d, t, side) = (cx.d, cx.t, o!(g, op).side);
    if can.first || g.dr[d].leaf.off || (thr!(g, op).shut_t.is_none() && is_open(g, d)) { return false; }
    if way_for_can(g, d, side, g.dr[d].leaf.vel) { thr!(g, op).shut_t = None; return false; }
    let since = match thr!(g, op).shut_t {
        Some(x) => x,
        None => {
            let locked = door_tools_locked_out(g, d, side);
            let since = if locked { -1e9 } else { t };
            thr!(g, op).shut_t = Some(since);
            let said = format!("can waits: the door has come shut ({} rad)", js::to_fixed(g.dr[d].leaf.angle, 2));
            door_tools_beat(g, op, &said);
            if !locked { door_tools_shove(g, op); }
            since
        }
    };
    if t - since < g.k.doortools.frag_way_wait { return true; }
    say(g, can.th, "the door's in the way — no bang. going in.");
    door_tools_beat(g, op, "can held");
    thr!(g, op).shut_t = None;
    door_tools_to_step(g, op);
    true
}

/// A can leaves a hand at the door, and not the lead's while another's will do: waited for in the order's own time, and given to who is there. True: not this tick.
fn hand_waited_for(g: &mut Game, op: DoorOpId, t: f64, can: &Can) -> bool {
    if tr(thr!(g, op).hand_done) || !door_tools_own_time(g, op) { thr!(g, op).hand_t = None; return false; }
    let wait = g.k.doortools.thr_hand_wait;
    let hand = hand_at_door(g, op, can).filter(|&h| h == can.th || hand_w(g, op) < wait);
    let Some(h) = hand else { no_hand(g, op, t, can); return true };
    thr!(g, op).hand_t = None;
    if h == can.th { return false; }
    thrower_is(g, op, Some(can.th), h, t);
    thr!(g, op).hand_w = Some(hand_w(g, op) + g.k.doortools.thr_turn);
    door_tools_beat(g, op, &format!("can to {} · at the door", g.gd[h].callsign.clone()));
    true
}
fn hand_at_door(g: &mut Game, op: DoorOpId, can: &Can) -> Option<GuardId> {
    let th = can.th;
    if th != o!(g, op).lead && door_tools_thr_at(g, op, th) { return Some(th); }
    let mut fit = |g: &mut Game, m_: GuardId| -> bool { m_ == th || can.blank || door_tools_pocket_of(g, op, m_).is_some() };
    door_tools_thr_pick(g, op, true, Some(&mut fit))
}
/// No hand at the door: the order waits for one, as long as it has left to; then they go in without.
fn no_hand(g: &mut Game, op: DoorOpId, t: f64, can: &Can) {
    let lead = o!(g, op).lead;
    let wait = g.k.doortools.thr_hand_wait;
    if hand_w(g, op) < wait {
        let every = m::max(0.05, g.k.doortools.thr_hand_every);
        match thr!(g, op).hand_t {
            None => {
                let why = why_no_hand(g, op, lead, can.th);
                door_tools_beat(g, op, &format!("can waits: no hand at the door {why}"));
                say(g, lead, "hold the bang — back on the door.");
            }
            Some(since) => thr!(g, op).hand_w = Some(hand_w(g, op) + m::min(t - since, 2.0 * every)),
        }
        thr!(g, op).hand_t = Some(t);
        if hand_w(g, op) < wait { thr!(g, op).next_t = t + every; return; }
    }
    thr!(g, op).hand_t = None;
    say(g, lead, if can.first { "nobody on the door — no bang." } else { "nobody on the door — no bang. going in." });
    if can.first { door_tools_thr_out(g, op, Some("nobody at the door")); return; }
    door_tools_beat(g, op, "no bang · nobody at the door");
    if js::truthy(thr!(g, op).tossed) { o!(g, op).phase = Phase::Wait; } else { door_tools_to_step(g, op); }
}
fn why_no_hand(g: &Game, op: DoorOpId, lead: GuardId, th: GuardId) -> String {
    if !door_tools_thr_at(g, op, th) { return format!("({} {} m off)", g.gd[th].callsign, js::to_fixed(v3::dist_xz(pos_of(g, th), g.dr[o!(g, op).door].frame_centre), 1)); }
    let mut off: Vec<String> = Vec::new();
    for m_ in o!(g, op).men.clone() { if m_ != lead && !door_tools_thr_at(g, op, m_) { off.push(g.gd[m_].callsign.clone()); } }
    format!("but the lead's own ({} not back)", if off.is_empty() { String::from("nobody") } else { off.join(", ") })
}

/// It goes, from his hand at the frame to the pocket, or the middle; the one before the first step is cooked to go off a little way in.
fn thrown(g: &mut Game, op: DoorOpId, cx: &Cx, can: &Can) {
    let (d, side) = (cx.d, o!(g, op).side);
    let from = door_tools_hand_at(g, op, can.th);
    let land = match can.pick.at { Some(a) => a, None => door_tools_land_at(g, op, None) };
    let v = crate::throwables::throwables_solve(g, from, land, Some(9.0)).v0;
    let it = leaves_hand(g, can.th, crate::throwables::ItemKind::Flash, from, v, 99.0);
    let first_in = g.k.doortools.thr_first_in;
    let short = can.pick.at.filter(|_| can.first && first_in > 0.0).and_then(|at| {
        let o0 = off_of(g, d, from, side);
        let k = (o0 + first_in) / (o0 - off_of(g, d, at, side));
        if k > 0.0 && k < 1.0 { Some([from[0] + (at[0] - from[0]) * k, at[1], from[2] + (at[2] - from[2]) * k]) } else { None }
    });
    let r = &mut o!(g, op);
    r.can = Some(it);
    r.toss_t = cx.t;
    r.land_t = f64::NAN;
    r.threw = Some(can.th);
    r.phase = Phase::Fuse;
    let x = thr!(g, op);
    x.aim = short.or(can.pick.at);
    x.from = Some(from);
    x.tossed += 1.0;
    let what = match can.pick.at {
        Some(at) => {
            let (inside, along) = (-off_of(g, d, at, side), along_of(g, d, at) - g.dr[d].def.width / 2.0);
            let cooked = if short.is_some() { format!(" · cooked short, {} m in", first_in) } else { String::new() };
            format!("pocket · {} cells unseen · {} m in, {} along{}", can.pick.n, js::to_fixed(inside, 1), js::to_fixed(along, 1), cooked)
        }
        None => String::from(if o!(g, op).search.flatten().is_some() { "no blind pocket: the middle" } else { "no model of the room: the middle" }),
    };
    let said = format!("toss:{} · {}{}", g.gd[can.th].callsign, if can.first { "first · " } else { "" }, what);
    door_tools_beat_is(g, op, crate::doortools::Beat::Toss, &said);
}
