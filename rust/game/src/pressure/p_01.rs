//! src/game/pressure.ts lines 1 to 434
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The answer to where a man who gives ground goes: PRESSURE_HOOKS.fallBackTo is never set, so a room his side dug in, or nothing. (pressure.ts hooked)
pub fn hooked(g: &mut crate::game::Game, d: RoomfightDefId, why: crate::pressure::HookedWhy, threat: V3) -> Option<crate::pressure::Hooked> {
    if g.k.digin.on {
        crate::digin::dig_in_refuge(g, d, threat).map(|r| crate::pressure::Hooked { spot: r.spot, door: r.door })
    } else {
        None
    }
}

/// All the counts at nought. (pressure.ts newStats)
pub fn new_stats() -> crate::pressure::StateStats {
    crate::pressure::StateStats { press_formed: 0.0, press_men: 0.0, press_sieges: 0.0, press_converge: 0.0, press_words: 0.0, press_lone: 0.0, press_arrived: 0.0, press_late: 0.0, press_landed: 0.0, press_cue_first: 0.0, press_heard: 0.0, press_lead: 0.0, flank_sent: 0.0, flank_set: 0.0, flank_arrived: 0.0, flank_late: 0.0, flank_landed: 0.0, flank_cue_first: 0.0, flank_heard: 0.0, flank_lead: 0.0, fb_push: 0.0, fb_last: 0.0, fb_early: 0.0, fb_planted: 0.0, fb_fought: 0.0, fb_told: 0.0, warn_calls: 0.0, warn_stood_down: 0.0, warn_hits: 0.0, warn_lead: 0.0, second_stood: 0.0, second_calls: 0.0, pairs_told: 0.0, pairs_waited: 0.0, pairs_went: 0.0, stun_marked: 0.0, stun_held: 0.0, stun_held_in: 0.0, stun_late: 0.0, stun_off: 0.0, stun_bangs: 0.0, stun_empty: 0.0, last_held: 0.0, look_called: 0.0, look_men: 0.0, look_off: 0.0, }
}

/// This game's press state, made the first time it is asked for: the row itself. (pressure.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::pressure::State {
    g.m.pressure.states.get_or_insert_with(|| crate::pressure::State {
        presses: Vec::new(),
        of: JsMap::new(),
        outs: Vec::new(),
        serial: 0.0,
        stale: false,
        think: JsMap::new(),
        stay: JsMap::new(),
        called: JsMap::new(),
        stood_t: -100.0,
        sent: JsSet::new(),
        done_at: JsMap::new(),
        last: JsMap::new(),
        tried: JsMap::new(),
        ear: JsMap::new(),
        warn: None,
        told: JsMap::new(),
        heard: Vec::new(),
        ev_seen: JsSet::new(),
        marks: JsMap::new(),
        bangs: JsMap::new(),
        last_held: JsSet::new(),
        word: None,
        swept: f64::NEG_INFINITY,
        look: None,
        out: None,
        look_lost: false,
        look_t: f64::NEG_INFINITY,
        stats: new_stats(),
    })
}

/// PRESSURE.wayOut on a floor whose structure asks for it. (pressure.ts wayOut)
pub fn way_out(g: &mut crate::game::Game) -> bool {
    g.k.pressure.way_out && g.level.scenarios.as_ref().and_then(|s| s.room_clear.as_ref()).and_then(|r| r.way_out) == Some(true)
}

/// The press forms up outside the doors before it goes; `gather` is off on the page and under hard. (pressure.ts forms)
pub fn forms(g: &mut crate::game::Game) -> bool {
    way_out(g)
}

/// Distance on the floor. (pressure.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A number from a range, on the game's die. (pressure.ts pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 {
    let u = crate::math_dice::sim_random(g);
    r[0] + u * (r[1] - r[0])
}

/// (pressure.ts alive)
pub fn alive(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let gd = g.tab.roomfight_def[d].gd;
    g.gd[gd].state != crate::game::GuardState::Dead
}

/// A line in the fight's log, which keeps its last 400 to 600. (pressure.ts note)
pub fn note(g: &mut crate::game::Game, gd: GuardId, what: &str) {
    let note = crate::roomfight::FightNote { t: g.time, who: g.gd[gd].callsign.clone(), what: alloc::format!("[pressure] {}", what) };
    g.fight.log.push(note);
    if g.fight.log.len() > 600 {
        g.fight.log.drain(0..200);
    }
}

/// The nav grid's cell a point falls in. (pressure.ts cellOf)
pub fn cell_of(g: &mut crate::game::Game, p: V3) -> f64 {
    let cell = g.col.nav.cell;
    crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell))
}

/// Where his eyes are: the head bone, or a standing man's height. (pressure.ts eyeOf)
pub fn eye_of(g: &mut crate::game::Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(h) => [h[0], h[1], h[2]],
        None => {
            let p = g.ch[c].pos;
            [p[0], 1.6, p[2]]
        }
    }
}

/// The room of his side's he hears fought over: the nearest post of a mate's in another room to where his ears put the noise. (pressure.ts target)
pub fn target(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::pressure::Target> {
    let noise_at = g.tab.roomfight_def[d].noise_at?;
    let d_room = g.tab.roomfight_def[d].room;
    let mut best = None;
    let mut bd = 9.0;
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if x == d || g.tab.roomfight_def[x].room == d_room || g.tab.roomfight_def[x].mode == crate::roomfight::DefMode::Free {
            continue;
        }
        let station = g.tab.roomfight_def[x].station;
        let dist = flat(station, noise_at);
        if dist >= bd {
            continue;
        }
        let Some(t) = crate::roomfight::room_fight_room_at(g, station) else { continue };
        if Some(t) == d_room || g.tab.roomfight_room[t].cells.len() as f64 > ROOM_CELLS || g.tab.roomfight_room[t].entries.is_empty() {
            continue;
        }
        bd = dist;
        best = Some(crate::pressure::Target { t, at: station });
    }
    best
}

/// The room a picket hears fought over: PRESSURE.picket is off on the page and under hard, so no man is a picket. (pressure.ts picketHears)
pub fn picket_hears(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<RoomfightRoomId> {
    None
}

/// The staging's marks as they are: PRESSURE.picket is off on the page and under hard. (pressure.ts picketMarks)
pub fn picket_marks(g: &mut crate::game::Game, marks: &[crate::nightbreach::NightBreachMark], team_at: V3) -> Vec<crate::nightbreach::NightBreachMark> {
    marks.to_vec()
}

/// Nothing to do: picket_marks never posts a man. (pressure.ts postPicket)
pub fn post_picket(g: &mut crate::game::Game, gd: GuardId, room: RoomfightRoomId) {}

/// Asked first by the stock rally: true = this module has the man, so the walk-over-by-himself does not start. (pressure.ts pressOwns)
pub fn press_owns(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    if !g.k.pressure.on {
        return false;
    }
    let st = state_of(g);
    if st.of.has(d) || st.think.has(d) {
        return true;
    }
    if last_alone(g, d) {
        return true;
    }
    if awaiting(g) {
        return true;
    }
    let Some(keep) = state_of(g).stay.get(d).copied() else { return false };
    aim(g, d).map(|a| a.t) == Some(keep) && stays_home(g, d)
}

/// digin.ts asks this before a man starts on his room: true = he is in a press or on his beat toward one. (pressure.ts pressWants)
pub fn press_wants(g: &mut crate::game::Game, def: RoomfightDefId) -> bool {
    if !g.k.pressure.on {
        return false;
    }
    if g.m.pressure.states.is_none() {
        return false;
    }
    let now = g.time;
    if ready_man(g, def, now) {
        return true;
    }
    let Some(st) = g.m.pressure.states.as_ref() else { return false };
    st.think.has(def) || st.of.get(def).map_or(false, |m| !g.tab.pressure_man[*m].fell)
}

/// digin.ts asks this too: true = a press has left him to keep his room, and a mate of that room has gone. (pressure.ts pressKeeps)
pub fn press_keeps(g: &mut crate::game::Game, def: RoomfightDefId) -> bool {
    if !g.k.pressure.on {
        return false;
    }
    let Some(st) = g.m.pressure.states.as_ref() else { return false };
    if !st.stay.has(def) {
        return false;
    }
    match g.tab.roomfight_def[def].room {
        Some(r) => st.sent.has(r),
        None => false,
    }
}

/// The man left to keep his room still has it to keep: a mate of that room is up. (pressure.ts staysHome)
pub fn stays_home(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let d_room = g.tab.roomfight_def[d].room;
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if x == d || !alive(g, x) {
            continue;
        }
        let from_home = state_of(g).of.get(x).copied().map_or(false, |m| g.tab.pressure_man[m].from == d_room);
        if g.tab.roomfight_def[x].room == d_room || from_home {
            return true;
        }
    }
    false
}

/// What a way on foot pays for a cell of room T (gatherSpot's `dear`).
fn dear(g: &mut Game, t: RoomfightRoomId, ix: f64, iz: f64) -> f64 {
    let i = crate::scene_collision::nav_grid_idx(g, ix, iz);
    if g.tab.roomfight_room[t].cells.has(crate::Num(i)) { 40.0 } else { 1.0 }
}

/// The stock fall-back's spots outside a door: depth, then sideways.
const SPOTS: [[f64; 2]; 9] = [[3.0, 1.6], [3.0, -1.6], [2.4, 1.2], [2.4, -1.2], [1.2, 2.4], [1.2, -2.4], [1.2, 1.7], [1.2, -1.7], [2.6, 0.0]];

/// A spot outside entry `e` of room T, off the door's line, that he can walk to without crossing T. (pressure.ts gatherSpot)
pub fn gather_spot(g: &mut crate::game::Game, d: RoomfightDefId, t: RoomfightRoomId, e: crate::roomfight::Entry) -> Option<crate::pressure::GatherSpot> {
    let c = g.gd[g.tab.roomfight_def[d].gd].ch;
    let fr = crate::guards::frame_of(g, e.door);
    if way_out(g) {
        // (no floor that far back with a line to the door: not his door, never the stock spots, which are inside the flash)
        return stand_off_spot(g, d, t, e, &mut |g: &mut Game, ix: f64, iz: f64| dear(g, t, ix, iz)).map(|s| crate::pressure::GatherSpot { spot: s.spot, l: s.l });
    }
    for [depth, lat] in SPOTS {
        let p: V3 = [
            e.at[0] + e.out[0] * depth + fr.ux * lat * (1.0 + fr.w / 3.2),
            0.0,
            e.at[2] + e.out[1] * depth + fr.uz * lat * (1.0 + fr.w / 3.2),
        ];
        if crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
            continue;
        }
        let cell = cell_of(g, p);
        if g.tab.roomfight_room[t].cells.has(crate::Num(cell)) {
            continue;
        }
        if mate_stands_on(g, d, p) {
            continue;
        }
        let st = g.m.pressure.states.as_ref();
        if st.map_or(false, |st| st.of.values().any(|m| g.tab.pressure_man[*m].d != d && flat(g.tab.pressure_man[*m].spot, p) < 0.9)) {
            continue;
        }
        let from = g.ch[c].pos;
        let Some(path) = crate::scene_collision::nav_grid_find_path(g, from, p, Some(6000.0), Some(&mut |g: &mut Game, ix: f64, iz: f64| dear(g, t, ix, iz))) else { continue };
        if path.is_empty() {
            continue;
        }
        let mut l = 0.0;
        let mut q = from;
        let mut through = false;
        for w in path {
            l += flat(q, w);
            q = w;
            let cell = cell_of(g, w);
            if g.tab.roomfight_room[t].cells.has(crate::Num(cell)) {
                through = true;
            }
        }
        if through || flat(q, p) > 0.8 {
            continue;
        }
        return Some(crate::pressure::GatherSpot { spot: p, l });
    }
    None
}

/// A living mate stands within a body's width of p (gatherSpot's `F.defs.some`).
fn mate_stands_on(g: &mut Game, d: RoomfightDefId, p: V3) -> bool {
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if x != d && alive(g, x) {
            let c = g.gd[g.tab.roomfight_def[x].gd].ch;
            if flat(g.ch[c].pos, p) < 0.9 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The TypeScript's own bits: bun with EXACT.on, `Math.hypot(a[0] - b[0], a[2] - b[2])`.
    #[test]
    fn flat_matches_the_typescript() {
        let cases: [(V3, V3, u64); 4] = [
            ([1.1, 0.0, 2.2], [-3.3, 9.0, 4.7], 0x40143e166993943a),
            ([0.1, 0.0, 0.2], [0.3, 0.0, 0.7], 0x3fe13b86ea20ae5c),
            ([12.345, 1.0, -6.789], [12.345, 2.0, -6.789], 0x0000000000000000),
            ([1e-3, 0.0, 7.25], [46.125, 0.0, -3.5], 0x4047ae1a1c50acc9),
        ];
        for (a, b, bits) in cases {
            assert_eq!(flat(a, b).to_bits(), bits);
        }
    }
}
