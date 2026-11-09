//! src/game/laststand.ts lines 410 to 618
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- small private helpers

/// the module's state: every caller here runs with it made
fn st(g: &mut Game) -> &mut State { g.m.laststand.states.as_mut().unwrap() }
/// the die's seed off the room clear's seed and attempt
fn fresh_seed(seed: f64, attempt: f64) -> f64 {
    ((m::imul(js::to_i32(seed), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(attempt), 0x85EBCA6Bu32 as i32) ^ 0x1A575A4D) as i32) as f64
}

// ---------------------------------------------------------------- the stand

// ---------------------------------------------------------------- the seams with the rest of the game

/// A new state for the endgame: its own die off the room clear's seed and attempt, four draws spent; the posts as staged, kept if it is the same staging.   (laststand.ts fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::laststand::State {
    let (seed, attempt) = (g.room_clear.seed, g.room_clear.attempt);
    let mut die = crate::math_dice::die_of(fresh_seed(seed, attempt));
    for _ in 0..4 { crate::math_dice::roll(&mut die); }
    let key = alloc::format!("{}:{}", seed, attempt);
    let kept = match &g.m.laststand.states { Some(was) if was.key == key => Some(was.post.clone()), _ => None };
    let post = match kept {
        Some(p) => p,
        None => {
            let mut p: JsMap<GuardId, String> = JsMap::new();
            for r in &g.staged.rooms { for &gd in &r.defenders { p.set(gd, r.def.id.clone()); } }
            p
        }
    };
    let mut rot: JsMap<LineKind, f64> = JsMap::new();
    rot.set(LineKind::MovedAt, 0.0);
    rot.set(LineKind::MovedFree, 0.0);
    State {
        post, key, die, next_at: 0.0, check: None, known: JsSet::new(), named: JsMap::new(), spots: JsMap::new(), at: JsMap::new(), 
        lone_said: false, few_n: 0.0, said: Vec::new(), stats: new_stats(), stale: false, rot, cls_at: Vec::new(), later: Vec::new(),
    }
}

/// Once a frame from RoomFight.tick, after the defenders', the press's and the buddy call's own ticks.   (laststand.ts lastStandTick)
pub fn last_stand_tick(g: &mut crate::game::Game) {
    step_stand(g);
}

/// The frame of the endgame: 'most are down' starts it; then the lines waiting, the check, who has moved, the stand.   (laststand.ts stepStand)
pub fn step_stand(g: &mut crate::game::Game) {
    if g.staged.rooms.is_empty() || g.staged.over { return; }
    if g.m.laststand.states.as_ref().map_or(true, |st| st.stale) {
        let all = g.staged.foes.clone();
        let few = m::min(g.k.laststand.few, m::floor(all.len() as f64 / 3.0));
        let mut up = 0;
        for &h in &all { if g.gd[h].state != crate::game::GuardState::Dead && !g.gd[h].fled { up += 1; } }
        if up as f64 > few || up == 0 { return; }
        let row = fresh(g);
        g.m.laststand.states = Some(row);
        let t = g.time;
        st(g).stats.t0 = t;
        st(g).stats.up0 = up as f64;
        let first = g.k.laststand.first;
        let at = t + pick(g, first);
        st(g).next_at = at;
        note(g, None, &alloc::format!("most are down: {} of {} up", up, all.len()));
    }
    let defs = g.fight.defs.clone();
    if !defs.iter().any(|&d| alive(g, d)) { return; }
    let mut names = st(g).named.keys();
    names.extend(st(g).spots.keys());
    for d in names { if !alive(g, d) { name(g, d, None); } }
    tick_later(g);
    if st(g).check.is_some() {
        tick_check(g);
    } else if g.time >= st(g).next_at {
        start_check(g);
    }
    if !st(g).at.is_empty() { tick_moved(g); }
}

/// The fight is over (RoomFight.stop): everything forgotten but the counts and the lines.   (laststand.ts lastStandStop)
pub fn last_stand_stop(g: &mut crate::game::Game) {
    let Some(st) = g.m.laststand.states.as_mut() else { return };
    st.check = None;
    st.known.clear();
    st.named.clear();
    st.spots.clear();
    st.at.clear();
    st.later.clear();
    st.stale = true;
}

/// roomclear.ts objective(): the rooms their net has named and the named places that are no room of it, by the men still up who said so
pub fn last_stand_hint(g: &crate::game::Game) -> Option<crate::laststand::Hint> {
    let st = g.m.laststand.states.as_ref()?;
    if st.stale { return None; }
    let mut rooms: JsSet<String> = JsSet::new();
    for (d, id) in st.named.iter() { if alive(g, d) { rooms.add(id.clone()); } }
    let mut spots: Vec<Spot> = Vec::new();
    for (d, x) in st.spots.iter() { if alive(g, d) && !spots.iter().any(|y| y.name == x.name) { spots.push(x.clone()); } }
    if rooms.is_empty() && spots.is_empty() { None } else { Some(Hint { rooms, spots }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_die_is_seeded_as_the_language_multiplies() {
        for &(seed, attempt, want) in SEEDS { assert_eq!(fresh_seed(seed, attempt).to_bits(), want.to_bits(), "{} {}", seed, attempt); }
    }

    const SEEDS: &[(f64, f64, f64)] = &[(0.0, 0.0, 441932365.0), (1.0, 1.0, 25946519.0), (12345.0, 2.0, -1330936078.0), (4294967295.0, 3.0, -363035837.0), (-7.0, 9.0, 57076647.0), (2147483648.0, 1.0, 532451366.0), (3.9, 2.9, -886660216.0), (987654321.0, 1000.0, -1273962284.0)];
}
