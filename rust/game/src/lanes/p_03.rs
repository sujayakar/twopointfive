//! src/game/lanes.ts lines 872 to 1001
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// No state yet, or no lane in it (`!st || !st.lanes.length`).
fn no_lanes(g: &Game) -> bool { g.m.lanes.states.as_ref().map_or(true, |st| st.lanes.is_empty()) }

/// The lanes lent out of the state for a read: `knows` takes `g` and a Lane, and the lanes live in `g`. `f` may call nothing that walks them.
fn with_lanes<R>(g: &mut Game, f: impl FnOnce(&mut Game, &[Lane]) -> R) -> R {
    let lanes = core::mem::take(&mut s(g).lanes);
    let r = f(g, &lanes);
    s(g).lanes = lanes;
    r
}

/// A key for a `Set<number>` of cells: JavaScript's Set counts -0 as +0, `Num` goes by the bits.
fn cell_key(i: f64) -> crate::Num { crate::Num(i + 0.0) }

/// `x.toFixed(1)`: Rust's `{:.1}` agrees but for an exact tie (only x.25 can be one, x.75 rounds up in both) and for -0.
fn to_fixed_1(x: f64) -> String {
    if x == 0.0 { return String::from("0.0"); }
    if js::rem(m::abs(x), 1.0) == 0.25 { return format!("{}{}.3", if x < 0.0 { "-" } else { "" }, m::floor(m::abs(x))); }
    format!("{:.1}", x)
}

fn pos_of(g: &Game, gd: GuardId) -> V3 { g.ch[g.gd[gd].ch].pos }

/// True = this rule has his feet this frame: his way lies down a lane his side knows is covered, and he has been stopped short of it. (lanes.ts laneGate)
pub fn lane_gate(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.gd[gd].script.is_some() && crate::director::director_lesson_runs(g) { return false; }
    if no_lanes(g) || !live(g) || !mine(g, gd) || g.gd[gd].path.is_empty() || pushing(g, gd) { return false; }
    let Some(m) = mask_of(g, gd) else { return false };
    if flanking(g, gd) || s(g).peek.has(gd) { return false; }
    let sg = g.gd[gd].script.as_ref().and_then(|sc| sc.goal);
    if let (Some(h), Some(sg)) = (s(g).held.get(gd), sg) {
        if !h.aimed && flat(sg, h.at) < 0.8 { return false; }
    }
    let last_look = s(g).look.get(gd).copied().unwrap_or(-100.0);
    if g.time - last_look < 0.2 { return false; }
    let now = g.time;
    s(g).look.set(gd, now);
    if close_fight(g, gd) || g.time < g.gd[gd].dazzled_until || crate::frag::frag_flee(g, gd).is_some() { return false; }
    let here = in_mask(g, Some(&m), pos_of(g, gd));
    let Some(mut first) = ahead(g, &m, gd, here) else { return false };
    if !here {
        if let Some(goal) = g.gd[gd].path_goal {
            crate::guards::go_to(g, gd, goal, Some(true));
            let Some(again) = ahead(g, &m, gd, false) else {
                s(g).stats.rerouted += 1.0;
                note(g, Some(gd), "a covered lane on his way: goes round it");
                return false;
            };
            first = again;
        }
    }
    let Some(l) = lane_at(g, first, gd) else { return false };
    if here {
        if g.time - l.last_t > g.k.lanes.hot_secs {
            let on = along(g, gd, g.k.lanes.ahead_m);
            if flat(on, l.from) > flat(pos_of(g, gd), l.from) - 0.5 { return false; }
        }
        if back_out(g, gd, &l) { return true; }
    } else {
        let p = pos_of(g, gd);
        let shift = g.k.lanes.shift_m;
        if (gun_on(g, &l, p, Some(shift)) || crate::doorway::doorway_in_way(g, p)) && back_out(g, gd, &l) { return true; }
    }
    if def_of(g, gd).is_none() { return false; }
    if !s(g).due.iter().any(|d| d.gd == gd) {
        let mut look = first;
        let (p, look_m) = (pos_of(g, gd), g.k.lanes.look_m);
        if flat(p, first) < look_m {
            let far = along(g, gd, 3.0);
            look = if flat(p, far) >= look_m { far } else if flat(p, l.from) >= look_m { l.from } else { first };
        }
        s(g).due.push(StateDueItem { gd, lane: l.id, look });
    }
    g.gd[gd].speed = 0.0;
    let c = g.gd[gd].ch;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    true
}

/// The point `m` metres on along his way. (lanes.ts along)
pub fn along(g: &mut crate::game::Game, gd: GuardId, m: f64) -> V3 {
    let mut a = pos_of(g, gd);
    let mut left = m;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() {
        let b = g.gd[gd].path[i];
        let d = flat(a, b);
        if d >= left { return [a[0] + (b[0] - a[0]) * left / js::or(d, 1.0), 0.0, a[2] + (b[2] - a[2]) * left / js::or(d, 1.0)]; }
        left -= d;
        a = b;
        i += 1;
    }
    a
}

/// He stops where he stands, short of a lane, his gun on `look`. (lanes.ts plantShort)
pub fn plant_short(g: &mut crate::game::Game, gd: GuardId, lane: f64, look: V3) {
    if !mine(g, gd) { return; }
    if !with_lanes(g, |g, lanes| lanes.iter().any(|l| l.id == lane && knows(g, l, gd))) { return; }
    if !crate::roomfight::room_fight_lane_stop(g, gd, look) { return; }
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    let (p, now) = (pos_of(g, gd), g.time);
    let st = s(g);
    st.held.set(gd, StateHeldValue { lane, t: now, look, aimed: true, at: p });
    st.stats.held += 1.0;
    if now - st.said_t > 5.0 {
        st.said_t = now;
        crate::game::game_say(g, gd, "holding here — I'm not walking into that!", Some(true), None, None);
    }
    let p = pos_of(g, gd);
    note(g, Some(gd), &format!("lane {}: stops short of it at ({}, {})", lane, to_fixed_1(p[0]), to_fixed_1(p[2])));
}

/// The first point of a known lane on the next `aheadM` of his way; `inside`: he stands in one, and None means his way leaves it for good within reach. (lanes.ts ahead)
pub fn ahead(g: &mut crate::game::Game, m: &[u8], gd: GuardId, inside: bool) -> Option<V3> {
    let mut a = pos_of(g, gd);
    let mut left = g.k.lanes.ahead_m;
    let mut was = inside;
    let mut hit: Option<V3> = None;
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() && left > 0.0 {
        let b = g.gd[gd].path[i];
        let d = flat(a, b);
        let k = m::max(1.0, m::ceil(m::min(d, left) / 0.35));
        let mut j = 1.0;
        while j <= k {
            let t = m::min(1.0, j * 0.35 / js::or(d, 1.0));
            let p: V3 = [a[0] + (b[0] - a[0]) * t, 0.0, a[2] + (b[2] - a[2]) * t];
            let on = in_mask(g, Some(m), p);
            if on && hit.is_none() { hit = Some(p); }
            if inside && was && !on { return None; }
            was = on;
            j += 1.0;
        }
        left -= d;
        a = b;
        i += 1;
    }
    if hit.is_some() { return hit; }
    if inside { Some(pos_of(g, gd)) } else { None }
}

/// He is held short of a lane his side knows is covered, and what wants him (`to`; none = anything) lies at its guns or down it. (lanes.ts laneHolds)
pub fn lane_holds(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool {
    if crate::gunroles::gun_keeps_post(g, gd) { return true; }
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    if st.held.is_empty() && st.peek.is_empty() { return false; }
    if !live(g) { return false; }
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    let lane = match (st.held.get(gd), st.peek.get(gd)) {
        (Some(h), _) => h.lane,
        (None, Some(p)) => p.lane,
        (None, None) => return false,
    };
    let Some(at) = st.lanes.iter().position(|x| x.id == lane) else { return false };
    let Some(to) = to else { return true };
    let field = g.k.lanes.field_m;
    if st.lanes[at].guns.iter().any(|m| flat(*m, to) < field + 4.0) { return true; }
    let i = cell_at(g, to);
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    st.lanes[at].cells.as_ref().map_or(false, |c| c.has(cell_key(i)))
}

/// He has shouted for a lane and the wind-up still has his arm: the dig-in does not adopt him. (lanes.ts laneThrows)
pub fn lane_throws(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    if st.peek.is_empty() { return false; }
    if !live(g) { return false; }
    let Some(st) = g.m.lanes.states.as_ref() else { return false };
    let Some(f) = st.peek.get(gd).and_then(|p| p.frag) else { return false };
    winding(g, gd, f)
}

/// The noise he would walk over to by himself is at the guns of a lane he knows is covered, or down it. (lanes.ts laneWarns)
pub fn lane_warns(g: &mut crate::game::Game, gd: GuardId, to: Option<V3>) -> bool {
    let Some(to) = to else { return false };
    if no_lanes(g) || !live(g) || !mine(g, gd) { return false; }
    let i = cell_at(g, to);
    let field = g.k.lanes.field_m;
    with_lanes(g, |g, lanes| {
        for l in lanes {
            if !knows(g, l, gd) { continue; }
            let Some(cells) = l.cells.as_ref() else { continue };
            if cells.has(cell_key(i)) || l.guns.iter().any(|m| flat(*m, to) < field + 4.0) { return true; }
        }
        false
    })
}

/// The far side of this doorway is in the line of guns his side knows of: no drill through it, unless his charge will burst on those guns. (lanes.ts laneDoor)
pub fn lane_door(g: &mut crate::game::Game, gd: GuardId, far: V3, centre: V3, can: bool) -> bool {
    if no_lanes(g) || !live(g) || !mine(g, gd) || pushing(g, gd) || flanking(g, gd) { return false; }
    let Some(l) = lane_on(g, far, gd) else { return false };
    let bang = g.k.lanes.bang_m;
    !(can && l.guns.iter().any(|m| flat(*m, centre) < bang + 2.5))
}

/// The two questions tryEngage asked, asked again while the drill has him this side of the doorway. True = the drill lets him go. (lanes.ts laneDrill)
pub fn lane_drill(g: &mut crate::game::Game, gd: GuardId, at: V3, far: V3, centre: V3, can: bool) -> bool {
    if !lane_covers(g, gd, at) && !lane_door(g, gd, far, centre, can) { return false; }
    note(g, Some(gd), "the doorway drill has him at a doorway in a lane his side knows of: it lets him go");
    true
}

/// This spot is in the line of guns his side knows of: no place to stack on a doorway. (lanes.ts laneCovers)
pub fn lane_covers(g: &mut crate::game::Game, gd: GuardId, p: V3) -> bool {
    if no_lanes(g) || !live(g) || !mine(g, gd) { return false; }
    !pushing(g, gd) && !flanking(g, gd) && lane_on(g, p, gd).is_some()
}

/// He stands in a lane his side knows is covered: no place to plant himself. (lanes.ts laneExposed)
pub fn lane_exposed(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if g.m.lanes.states.is_none() || !live(g) || !mine(g, gd) { return false; }
    if pushing(g, gd) || flanking(g, gd) { return false; }
    if no_lanes(g) || close_fight(g, gd) { return false; }
    let p = pos_of(g, gd);
    lane_on(g, p, gd).is_some()
}

/// A magazine change this rule starts for a man behind his corner and counts down itself. (lanes.ts laneReload)
pub fn lane_reload(g: &mut crate::game::Game, gd: GuardId) {
    crate::guards::start_reload_beat(g, gd);
    let t = g.gd[gd].reload_t;
    s(g).reloads.set(gd, t);
}

/// A new state, the counters kept. (lanes.ts lanesReset)
pub fn lanes_reset(g: &mut crate::game::Game) {
    let Some(stats) = g.m.lanes.states.as_ref().map(|st| st.stats) else { return };
    let mut st = fresh(g);
    st.stats = stats;
    g.m.lanes.states = Some(st);
}

/// The counters, and how many lanes the side knows. (lanes.ts lanesStats)
pub fn lanes_stats(g: &mut crate::game::Game) -> JsMap<String, f64> {
    let s = match g.m.lanes.states.as_ref() { Some(st) => st.stats, None => fresh(g).stats };
    let known = g.m.lanes.states.as_ref().map_or(0, |st| st.lanes.iter().filter(|l| l.known).count());
    let rows: [(&str, f64); 23] = [
        ("lanes", s.lanes), ("known", s.known), ("unknown", s.unknown), ("held", s.held), ("rerouted", s.rerouted), ("backedOut", s.backed_out),
        ("round", s.round), ("stuns", s.stuns), ("pushes", s.pushes), ("lapsed", s.lapsed), ("peeks", s.peeks), ("peekThere", s.peek_there),
        ("peekClear", s.peek_clear), ("peekLost", s.peek_lost), ("pins", s.pins), ("pinRounds", s.pin_rounds), ("pinLost", s.pin_lost), ("sets", s.sets),
        ("goes", s.goes), ("frags", s.frags), ("fragLost", s.frag_lost), ("fragKept", s.frag_kept), ("live", known as f64),
    ];
    let mut out = JsMap::new();
    for (k, v) in rows { out.set(String::from(k), v); }
    out
}

/// One line for the debugger. (lanes.ts lanesSummary)
pub fn lanes_summary(g: &mut crate::game::Game) -> String {
    let Some(st) = g.m.lanes.states.as_ref() else { return String::from("none yet") };
    let k = st.lanes.iter().filter(|l| l.known).count();
    let s = &st.stats;
    let frags = if js::truthy(s.frags) { format!(" · grenades down it {}", s.frags) } else { String::new() };
    format!(
        "{} known of {} · held short {} · round {} · backed out {} · charges {} · looks {} (still held {}, clear {}) · set on its other side {}, bursts from the corners {}{}",
        k, st.lanes.len(), s.held, s.rerouted + s.round, s.backed_out, s.stuns, s.peeks, s.peek_there, s.peek_clear, s.sets, s.pins, frags
    )
}

/// For a check or the debugger: every lane as plain data. (lanes.ts lanesList)
pub fn lanes_list(g: &mut crate::game::Game) -> Vec<crate::lanes::LanesListItem> {
    let Some(st) = g.m.lanes.states.as_ref() else { return Vec::new() };
    st.lanes.iter().map(|l| LanesListItem {
        id: l.id,
        from: l.from,
        at: l.at,
        known: l.known,
        weight: l.weight,
        downs: l.downs,
        cells: l.cells.as_ref().map_or(0.0, |c| c.len() as f64),
        until: l.until,
        push_until: l.push_until,
    }).collect()
}

/// This point is on the floor of a lane the whole side knows. (lanes.ts lanesKnows)
pub fn lanes_knows(g: &mut crate::game::Game, p: V3) -> bool {
    let Some(st) = g.m.lanes.states.as_mut() else { return false };
    let mask = st.mask.take();
    let r = in_mask(g, mask.as_deref(), p);
    s(g).mask = mask;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed_1_is_javascripts() {
        // printed by bun: [0.25, 1.25, -1.25, 0.75, 2.35, -0.04, -0, 0, 12.34, -3.96].map(x => x.toFixed(1))
        let cases: [(f64, &str); 10] = [(0.25, "0.3"), (1.25, "1.3"), (-1.25, "-1.3"), (0.75, "0.8"), (2.35, "2.4"), (-0.04, "-0.0"), (-0.0, "0.0"), (0.0, "0.0"), (12.34, "12.3"), (-3.96, "-4.0")];
        for (x, want) in cases { assert_eq!(to_fixed_1(x), want, "{}", x); }
    }

    #[test]
    fn a_negative_zero_cell_is_cell_zero() {
        assert!(cell_key(-0.0) == cell_key(0.0));
    }
}
