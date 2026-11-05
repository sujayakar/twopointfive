//! src/game/pressure.ts lines 931 to 1112
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// how long after the call a hunter may still stand and fight   (pressure.ts CALLED_SECS)
const CALLED_SECS: f64 = 30.0;

/// `x.toFixed(d)`: a tie goes to the larger number, where Rust's formatter goes to the even one. A tie has a finite binary form: `|x| * 2^(d+1)` is an odd integer.
fn to_fixed(x: f64, d: usize) -> String {
    if x.is_nan() { return String::from("NaN"); }
    let sign = if x < 0.0 { "-" } else { "" };
    let a = m::abs(x);
    if !a.is_finite() { return alloc::format!("{}Infinity", sign); }
    let mut two = 2.0;
    let mut unit = 0.25;
    for _ in 0..d { two *= 2.0; unit /= 10.0; }
    let v = if js::rem(a * two, 2.0) == 1.0 { a + unit } else { a };
    alloc::format!("{}{:.*}", sign, d, v)
}
/// `+x.toFixed(2)`
fn fixed2(x: f64) -> f64 { to_fixed(x, 2).parse::<f64>().unwrap_or(f64::NAN) }

fn say_radio(g: &mut Game, gd: GuardId, text: &str) { crate::game::game_say(g, gd, text, Some(true), None, None); }
/// `alive(X) && X.mode === 'hold'`   (standTo's holds)
fn holds(g: &mut Game, x: RoomfightDefId) -> bool { alive(g, x) && g.tab.roomfight_def[x].mode == crate::roomfight::DefMode::Hold }
/// `F.defs.find(holds)`
fn first_holding(g: &mut Game) -> Option<RoomfightDefId> {
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        if holds(g, d) { return Some(d); }
    }
    None
}
fn word_t(st: &State) -> f64 { st.word.map(|w| w.t).unwrap_or(f64::NEG_INFINITY) }

/// the fight a man's ears give him to go to: stood to, a report is nobody's room being hit   (pressure.ts reportStands)
pub fn report_stands(g: &mut crate::game::Game, d: RoomfightDefId) -> bool {
    let w = if g.k.pressure.warned { state_of(g).warn } else { None };
    match w {
        None => false,
        Some(w) => w.phase != WarnPhase::Over || (w.call_t >= 0.0 && g.tab.roomfight_def[d].noise_t < w.hit_t.unwrap_or(f64::NAN) - 0.5),
    }
}

/// the room a man's ears send him to, unless a report stands   (pressure.ts aim)
pub fn aim(g: &mut crate::game::Game, d: RoomfightDefId) -> Option<crate::pressure::Aim> {
    if report_stands(g, d) { return None; }
    target(g, d).map(|x| Aim { t: x.t, at: x.at })
}

/// the held room of theirs the noise points at: the one with a door the shortest way on foot from it   (pressure.ts roomFor)
pub fn room_for(g: &mut crate::game::Game, at: V3) -> Option<RoomfightRoomId> {
    #[derive(Clone, Copy)]
    struct Way { r: RoomfightRoomId, o: V3, d: f64 }
    let from = crate::roomfight::room_fight_free_near(g, at).unwrap_or(at);
    let mut ways: Vec<Way> = Vec::new();
    let mut done: JsSet<RoomfightRoomId> = JsSet::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if !alive(g, x) || g.tab.roomfight_def[x].mode != crate::roomfight::DefMode::Hold { continue; }
        let r = match g.tab.roomfight_def[x].room { Some(r) => r, None => continue };
        if done.has(r) || g.tab.roomfight_room[r].cells.len() as f64 > ROOM_CELLS { continue; }
        done.add(r);
        let entries = g.tab.roomfight_room[r].entries.clone();
        for e in entries.iter() {
            if let Some(o) = crate::roomfight::room_fight_free_near(g, [e.at[0] + e.out[0] * 1.2, 0.0, e.at[2] + e.out[1] * 1.2]) {
                ways.push(Way { r, o, d: flat(from, o) });
            }
        }
    }
    js::sort_by(&mut ways, |a, b| a.d - b.d);
    let mut best: Option<RoomfightRoomId> = None;
    let mut bl = f64::INFINITY;
    for way in ways.iter() {
        if way.d >= bl { break; }   // (no way on foot is shorter than the straight line)
        let r = way.r;
        let mut dear = |g: &mut Game, ix: f64, iz: f64| -> f64 {
            let cell = crate::scene_collision::nav_grid_idx(g, ix, iz);
            if g.tab.roomfight_room[r].cells.has(crate::Num(cell)) { 30.0 } else { 1.0 }
        };
        let path = match crate::scene_collision::nav_grid_find_path(g, from, way.o, Some(6000.0), Some(&mut dear)) { Some(p) => p, None => continue };
        let mut l = 0.0;
        let mut q = from;
        for p in path.iter() { l += flat(q, *p); q = *p; }
        if l < bl { bl = l; best = Some(way.r); }
    }
    best
}

/// STAND TO, once a frame before anybody is considered for a press: the report heard, called, stood down, or overtaken by a room being hit   (pressure.ts standTo)
pub fn stand_to(g: &mut crate::game::Game) {
    let warn = state_of(g).warn;
    let w = match warn {
        Some(w) => w,
        None => {   // the first loud thing any of them hears, as the man best placed heard it
            let mut by: Option<RoomfightDefId> = None;
            let mut i = 0;
            while i < g.fight.defs.len() {
                let d = g.fight.defs[i];
                i += 1;
                if !holds(g, d) { continue; }
                let def = &g.tab.roomfight_def[d];
                let at = match def.noise_at { Some(at) => at, None => continue };
                if !(g.time - def.noise_t < 1.0) { continue; }
                let nearer = match by {
                    None => true,
                    Some(b) => { let bd = &g.tab.roomfight_def[b]; flat(def.station, at) < flat(bd.station, bd.noise_at.unwrap()) }
                };
                if nearer { by = Some(d); }
            }
            let on = !state_of(g).presses.is_empty() || fight_on(g);
            if by.is_none() && !on { return; }
            let room = match by {
                Some(b) if !on => { let at = g.tab.roomfight_def[b].noise_at.unwrap(); room_for(g, at) }   // (no held room it could be for: nothing to stand to)
                _ => None,
            };
            let made = Warn {
                phase: if room.is_some() { WarnPhase::Heard } else { WarnPhase::Over },
                t0: g.time,
                by: by.unwrap_or_else(|| g.fight.defs[0]),
                call_t: -1.0,
                last_t: g.time,
                room,
                hit_t: None,
            };
            state_of(g).warn = Some(made);
            return;
        }
    };
    if w.phase == WarnPhase::Over { return; }
    if fight_on(g) {   // a room is being hit: the stock press's from here, with every hand in it if the report was called
        let now = g.time;
        let st = state_of(g);
        let o = st.warn.as_mut().unwrap();
        o.phase = WarnPhase::Over;
        o.hit_t = Some(now);
        if w.call_t < 0.0 { return; }
        st.stats.warn_hits += 1.0;
        st.stats.warn_lead += now - w.call_t;
        let mut pick: Option<RoomfightDefId> = None;
        let mut i = 0;
        while i < g.fight.defs.len() {
            let x = g.fight.defs[i];
            i += 1;
            if holds(g, x) && g.tab.roomfight_def[x].room != w.room && g.time >= g.gd[g.tab.roomfight_def[x].gd].dazzled_until { pick = Some(x); break; }
        }
        if pick.is_none() { pick = first_holding(g); }
        if let Some(x) = pick {
            let gd = g.tab.roomfight_def[x].gd;
            say_radio(g, gd, "contact! that's them \u{2014} everybody, go!");
            let text = alloc::format!("stand to: a room is hit {} s after the call, all hands", to_fixed(g.time - w.call_t, 1));
            note(g, gd, &text);
        }
        return;
    }
    if w.phase == WarnPhase::Called {
        let mut last = w.last_t;
        let mut i = 0;
        while i < g.fight.defs.len() {
            let d = g.fight.defs[i];
            i += 1;
            if holds(g, d) && g.tab.roomfight_def[d].noise_at.is_some() && g.tab.roomfight_def[d].noise_t > last { last = g.tab.roomfight_def[d].noise_t; }   // (more of it outside: they are still out there)
        }
        state_of(g).warn.as_mut().unwrap().last_t = last;
        if g.time - last <= g.k.pressure.stand_secs { return; }
        let by = first_holding(g);
        let st = state_of(g);
        st.warn = None;   // nobody came (the next report is a new one)
        st.stats.warn_stood_down += 1.0;
        if let Some(x) = by {
            let gd = g.tab.roomfight_def[x].gd;
            if !g.k.pressure.sweep { say_radio(g, gd, "nothing. stand down \u{2014} hold where you are."); }
            note(g, gd, "stand to: nobody came, stands down");
        }
        return;
    }
    if g.time - w.t0 < g.k.pressure.report_secs { return; }
    let now = g.time;
    let o = state_of(g).warn.as_mut().unwrap();
    o.phase = WarnPhase::Called;
    o.call_t = now;
    o.last_t = now;
    let x = if holds(g, w.by) { w.by } else { first_holding(g).unwrap_or(w.by) };
    let gd = g.tab.roomfight_def[x].gd;
    state_of(g).stats.warn_calls += 1.0;
    let line: &str = crate::words::wg(g, &"that was outside \u{2014} they're coming. get ready!", &"that was outside \u{2014} they're coming. stand to!");
    say_radio(g, gd, line);
    let text = alloc::format!("stand to: calls the report, room {} is the nearest of theirs to it", g.tab.roomfight_room[w.room.unwrap()].id);
    note(g, gd, &text);
}

/// STAND AND FIGHT, once a frame: a hunter the press called, with eyes and a clear shot on one of them out of the room he was called to, stops where he stands   (pressure.ts standAndFight)
pub fn stand_and_fight(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        let gd = g.tab.roomfight_def[d].gd;
        let c = g.gd[gd].ch;
        let st = state_of(g);
        let m = st.of.get(d).copied();
        let k = match st.called.get(d).copied() { Some(k) => k, None => continue };
        if g.time - k.t_2 > CALLED_SECS || !alive(g, d) || g.gd[gd].held.is_some() || g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Free
            || m.map_or(false, |m| g.tab.pressure_man[m].fell) || g.time - g.gd[gd].last_seen_t >= 0.1 || g.time < g.gd[gd].dazzled_until || g.time < g.gd[gd].stagger_until { continue; }
        let dc = crate::guards::door_cells(g);
        let sight = g.gd[gd].sight_pos;
        let at = cell_of(g, sight);
        let pos = g.ch[c].pos;
        let mine = cell_of(g, pos);
        let d_at = dc.get(crate::Num(at)).copied();
        let fc = match g.gd[gd].foe {
            Some(crate::game::Foe::Player) => Some(g.pl[g.player].ch),
            Some(crate::game::Foe::Guard(f)) => Some(g.gd[f].ch),
            None => None,
        };
        let fc = match fc { Some(fc) => fc, None => continue };
        let t = k.t;
        if g.tab.roomfight_room[t].cells.has(crate::Num(at)) || g.tab.roomfight_room[t].cells.has(crate::Num(mine)) || dc.has(crate::Num(mine)) { continue; }   // (never in a door frame himself: his mates need it)
        if let Some(door) = d_at {
            if g.tab.roomfight_room[t].entries.iter().any(|e| e.door == door) { continue; }   // one of them in that room's own door frame is not out of it
        }
        let fp = g.ch[fc].pos;
        if !crate::guards::clear_shot(g, c, [fp[0], fp[1] + 1.2, fp[2]]) { continue; }
        if crate::lanes::lane_exposed(g, gd) { continue; }   // (floor his side knows a gun covers: no place to plant himself)
        g.gd[gd].hold = true;
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_goal = None;
        crate::roomfight::room_fight_stand_on(g, d, true);
        let now = g.time;
        if let Some(m) = m { drop(g, m); } else { state_of(g).done_at.set(d, now); }
        let st = state_of(g);
        st.stats.second_stood += 1.0;
        if now - st.stood_t > 6.0 {
            st.stood_t = now;
            st.stats.second_calls += 1.0;
            say_radio(g, gd, "contact \u{2014} they're out in the open! stand and fight!");
        }
        note(g, gd, "meets the team in the open, stands and fights");
    }
}

/// goLook's `free`: a man at his post with nothing else on, for the word at `p`
fn look_free(g: &mut Game, d: RoomfightDefId, p: V3) -> bool {
    if !alive(g, d) || g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Hold || state_of(g).of.has(d) { return false; }
    let gd = g.tab.roomfight_def[d].gd;
    if g.gd[gd].held.is_some() || g.tab.roomfight_def[d].rally_at == f64::INFINITY || !(g.time >= g.gd[gd].dazzled_until) || !(g.time >= g.gd[gd].stagger_until) { return false; }
    if state_of(g).outs.iter().any(|o| o.def == d) || crate::laststand::last_stand_man(g, gd) || crate::enemysquads::squad_owns(g, gd) { return false; }
    let ph = crate::digin::dig_in_phase(g, gd);   // (on his way to cover or at a door with a wedge: busy. Set on his door, he has done getting ready)
    if ph != "idle" && ph != "set" { return false; }
    !crate::lanes::lane_holds(g, gd, Some(p)) && !crate::lanes::lane_warns(g, gd, Some(p))
}

/// GO AND LOOK, once a frame after the press's own rules: the word is kept every frame; a look is called when it has gone quiet   (pressure.ts goLook)
pub fn go_look(g: &mut crate::game::Game) {
    if g.rescue.active { return; }   // (a hostage rescue's men are pinned to their room a frame at a time)
    let mut up = 0.0;
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if !alive(g, x) { continue; }
        up += 1.0;
        let gd = g.tab.roomfight_def[x].gd;
        let (seen, held, sight) = (g.gd[gd].last_seen_t, g.gd[gd].held.is_some(), g.gd[gd].sight_pos);
        let (noise_at, noise_t) = (g.tab.roomfight_def[x].noise_at, g.tab.roomfight_def[x].noise_t);
        let st = state_of(g);
        if !held && seen >= 0.0 && seen > word_t(st) { st.word = Some(StateWord { t: seen, p: sight }); }
        if let Some(at) = noise_at {
            if noise_t > word_t(st) { st.word = Some(StateWord { t: noise_t, p: at }); }
        }
    }
    // the men of the last look: one of them down while he was out, and nobody else walks out this fight; all of them holding again, and that look is over
    if let Some(out) = state_of(g).out.clone() {
        let mut down = false;
        for &d in out.iter() { if !alive(g, d) { down = true; break; } }
        if down {
            let st = state_of(g);
            st.look_lost = true;
            st.out = None;
            let mut by = None;
            let mut i = 0;
            while i < g.fight.defs.len() {
                let x = g.fight.defs[i];
                i += 1;
                if alive(g, x) { by = Some(x); break; }
            }
            if let Some(by) = by { let gd = g.tab.roomfight_def[by].gd; note(g, gd, "go and look: a man of the look is down \u{2014} nobody else walks out"); }
        } else if out.iter().all(|&d| g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Free) {
            state_of(g).out = None;
        }
    }
    let st = state_of(g);
    let w = match st.word { Some(w) if !st.look_lost => w, _ => return };
    if let Some(l) = st.look.clone() {
        let mut off = w.t > l.w;   // (word of them in the beat, or a man of the pair taken: nobody goes on the old call)
        if !off { for &d in l.men.iter() { if !look_free(g, d, w.p) { off = true; break; } } }
        if off {
            let st = state_of(g);
            st.look = None;
            st.stats.look_off += 1.0;
            if w.t <= l.w { st.swept = f64::NEG_INFINITY; }
            return;
        }
        if g.time < l.go_t { return; }
        let st = state_of(g);
        st.look = None;
        st.out = Some(l.men.clone());
        for &d in l.men.iter() {
            loose(g, d, l.at);
            state_of(g).stats.look_men += 1.0;
            let gd = g.tab.roomfight_def[d].gd;
            let c = g.gd[gd].ch;
            let text = alloc::format!("goes to look where the team was last put ({} m)", to_fixed(flat(g.ch[c].pos, l.at), 1));
            note(g, gd, &text);
        }
        return;
    }
    let (swept, look_t, busy) = { let st = state_of(g); (st.swept, st.look_t, !st.presses.is_empty() || !st.outs.is_empty()) };
    if swept >= w.t || g.time - w.t < g.k.pressure.sweep_secs || g.time - look_t < g.k.pressure.sweep_secs { return; }
    if busy || awaiting(g) || crate::enemysquads::squads_live(g) { return; }
    let n = m::min(m::max(2.0, g.k.pressure.sweep_men), up - 1.0);
    if n < 2.0 { return; }   // (never fewer than two, and somebody always stays)
    let mut i = 0;
    while i < g.fight.defs.len() {   // (a man is out already: his look is the look)
        let x = g.fight.defs[i];
        i += 1;
        if alive(g, x) && g.tab.roomfight_def[x].mode == crate::roomfight::DefMode::Free { return; }
    }
    if let Some(r) = crate::roomfight::room_fight_room_at(g, w.p) {   // (a man of theirs holds the room that word is of: he is looking at it)
        let mut i = 0;
        while i < g.fight.defs.len() {
            let x = g.fight.defs[i];
            i += 1;
            if holds(g, x) && g.tab.roomfight_def[x].room == Some(r) { return; }
        }
    }
    let mut c: Vec<RoomfightDefId> = Vec::new();
    let mut i = 0;
    while i < g.fight.defs.len() {
        let x = g.fight.defs[i];
        i += 1;
        if look_free(g, x, w.p) { c.push(x); }
    }
    {
        let defs = &g.tab.roomfight_def;
        js::sort_by(&mut c, |a, b| flat(defs[*a].station, w.p) - flat(defs[*b].station, w.p));
    }
    for &lead in c.iter() {   // the nearest man who has his mates within a call of him: the pair leaves as a pair
        let at_lead = g.tab.roomfight_def[lead].station;
        let pair_m = g.k.pressure.sweep_pair_m;
        let mut mates: Vec<RoomfightDefId> = Vec::new();
        for &d in c.iter() { if d != lead && flat(g.tab.roomfight_def[d].station, at_lead) <= pair_m { mates.push(d); } }
        {
            let defs = &g.tab.roomfight_def;
            js::sort_by(&mut mates, |a, b| flat(defs[*a].station, at_lead) - flat(defs[*b].station, at_lead));
        }
        mates.truncate((n - 1.0) as usize);
        if (mates.len() as f64) < n - 1.0 { continue; }
        let now = g.time;
        let go_t = now + g.k.pressure.sweep_beat;
        let mut men = vec![lead];
        men.extend_from_slice(&mates);
        let st = state_of(g);
        st.swept = w.t;
        st.look_t = now;
        st.look = Some(StateLook { at: w.p, go_t, w: w.t, men });
        st.stats.look_called += 1.0;
        let gd = g.tab.roomfight_def[lead].gd;
        say_radio(g, gd, "it's gone quiet. with me \u{2014} let's go look.");
        let text = alloc::format!("go and look: nothing of the team for {} s, calls {} to where they were last put", to_fixed(g.time - w.t, 0), n);
        note(g, gd, &text);
        return;
    }
}

/// `const st = stateOf(g); if (st.stale) { ... }`: a new fight's counts
fn fresh_state(g: &mut Game) {
    let st = state_of(g);
    if st.stale { st.stale = false; st.stats = new_stats(); }
}

/// once a frame from RoomFight.tick, after the defenders' and the teammates' own ticks   (pressure.ts pressTick)
pub fn press_tick(g: &mut crate::game::Game, _dt: f64) {
    // the kit is its own lever: what a defender carries is handed out whether or not the press is on
    let mut i = 0;
    if g.k.pressure.stuns > 0.0 {
        while i < g.fight.defs.len() {
            let gd = g.tab.roomfight_def[g.fight.defs[i]].gd;
            i += 1;
            if !g.m.pressure.kitted.has(gd) { g.m.pressure.kitted.add(gd); let n = g.k.pressure.stuns; crate::tactics::give_stuns(g, gd, n); }
        }
    } else {
        while i < g.fight.defs.len() {
            let gd = g.tab.roomfight_def[g.fight.defs[i]].gd;
            i += 1;
            if g.m.pressure.kitted.delete(gd) { crate::tactics::give_stuns(g, gd, 0.0); }   // (the row put back to 0 mid-fight: the charges go back with it)
        }
    }
    if g.k.pressure.stuns > 0.0 { fresh_state(g); learn(g); }
    if !g.k.pressure.on { return; }
    fresh_state(g);
    if g.k.pressure.warned && !crate::enemysquads::squads_live(g) { stand_to(g); }   // (on a squads level a shot heard before the first entry is each squad's own to answer)
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        if alive(g, d) { consider(g, d); }
    }
    if !state_of(g).presses.is_empty() {   // `st.presses = st.presses.filter(...)`: walks the list as it stood, and what it kept replaces the list
        let walked = state_of(g).presses.clone();
        let mut kept = Vec::with_capacity(walked.len());
        for mut p in walked { if !tick_press(g, &mut p) { kept.push(p); } }
        state_of(g).presses = kept;
    }
    if g.k.pressure.second { stand_and_fight(g); }
    let mut i = 0;
    while i < g.fight.defs.len() {
        let d = g.fight.defs[i];
        i += 1;
        if alive(g, d) { last_man(g, d); }
    }
    if !state_of(g).outs.is_empty() {   // the same for the men fallen back
        let walked = state_of(g).outs.clone();
        let mut kept = Vec::with_capacity(walked.len());
        for mut o in walked { if !tick_out(g, &mut o) { kept.push(o); } }
        state_of(g).outs = kept;
    }
    if g.k.pressure.sweep { go_look(g); }
}

/// the fight is over: every press forgotten; the counts stay for whoever reads them after the run   (pressure.ts pressStop)
pub fn press_stop(g: &mut crate::game::Game) {
    let st = match g.m.pressure.states.as_mut() { Some(st) => st, None => return };
    st.presses = Vec::new();
    st.of.clear();
    st.outs = Vec::new();
    st.think.clear();
    st.stay.clear();
    st.called.clear();
    st.stood_t = -100.0;
    st.sent.clear();
    st.done_at.clear();
    st.last.clear();
    st.tried.clear();
    st.ear.clear();
    st.warn = None;
    st.told.clear();
    st.heard = Vec::new();
    st.marks.clear();
    st.last_held.clear();
    st.word = None;
    st.swept = f64::NEG_INFINITY;
    st.look = None;
    st.out = None;
    st.look_lost = false;
    st.look_t = f64::NEG_INFINITY;
    st.stale = true;
}

/// when the look this man is on was called (called and not yet gone, or still out), or none. Words only   (pressure.ts lookSince)
pub fn look_since(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> {
    let st = g.m.pressure.states.as_ref()?;
    let is_his = |d: &RoomfightDefId| g.tab.roomfight_def[*d].gd == gd;
    let on_look = st.look.as_ref().map_or(false, |l| l.men.iter().any(is_his));
    if on_look || st.out.as_ref().map_or(false, |o| o.iter().any(is_his)) { Some(st.look_t) } else { None }
}

/// the counts of this run, with the stock room fight's own beside them   (pressure.ts pressureStats)
pub fn pressure_stats(g: &mut crate::game::Game) -> JsMap<String, f64> {
    let s = match g.m.pressure.states.as_ref() { Some(st) => st.stats, None => new_stats() };
    let f = &g.fight.stats;
    let rows: [(&str, f64); 49] = [
        ("pressFormed", s.press_formed), ("pressMen", s.press_men), ("pressSieges", s.press_sieges), ("pressConverge", s.press_converge),
        ("pressWords", s.press_words), ("pressLone", s.press_lone), ("pressArrived", s.press_arrived), ("pressLate", s.press_late),
        ("pressLanded", s.press_landed), ("pressCueFirst", s.press_cue_first), ("pressHeard", s.press_heard), ("pressLead", fixed2(s.press_lead)),
        ("flankSent", s.flank_sent), ("flankSet", s.flank_set), ("flankArrived", s.flank_arrived), ("flankLate", s.flank_late),
        ("flankLanded", s.flank_landed), ("flankCueFirst", s.flank_cue_first), ("flankHeard", s.flank_heard), ("flankLead", fixed2(s.flank_lead)),
        ("fbPush", s.fb_push), ("fbLast", s.fb_last), ("fbEarly", s.fb_early), ("fbPlanted", s.fb_planted), ("fbFought", s.fb_fought), ("fbTold", s.fb_told),
        ("warnCalls", s.warn_calls), ("warnStoodDown", s.warn_stood_down), ("warnHits", s.warn_hits), ("warnLead", fixed2(s.warn_lead)),
        ("secondStood", s.second_stood), ("secondCalls", s.second_calls),
        ("pairsTold", s.pairs_told), ("pairsWaited", s.pairs_waited), ("pairsWent", s.pairs_went),
        ("stunMarked", s.stun_marked), ("stunHeld", s.stun_held), ("stunHeldIn", s.stun_held_in), ("stunLate", s.stun_late),
        ("stunOff", s.stun_off), ("stunBangs", s.stun_bangs), ("stunEmpty", s.stun_empty), ("lastHeld", s.last_held),
        ("lookCalled", s.look_called), ("lookMen", s.look_men), ("lookOff", s.look_off),
        ("stockRallied", f.rallied), ("stockFellBack", f.fell_back), ("stockTookCover", f.took_cover),
    ];
    let mut out = JsMap::new();
    for (k, v) in rows.iter() { out.set(String::from(*k), *v); }
    out
}

/// a line for the debugger's panel. PRESSURE.press, flank and fallBack are settled on   (pressure.ts pressureSummary)
pub fn pressure_summary(g: &mut crate::game::Game) -> String {
    if !g.k.pressure.on { return String::from("off"); }
    let second = g.k.pressure.second;
    let st = match g.m.pressure.states.as_ref() { Some(st) => st, None => return String::from("on \u{b7} nothing yet") };
    let s = st.stats;
    alloc::format!(
        "press + flank + fallBack{} \u{b7} {} live \u{b7} {} presses, {} words, {} round, {} fell back{}",
        if second { " + second" } else { "" }, st.presses.len(), s.press_formed, s.press_words, s.flank_sent, s.fb_push + s.fb_last,
        if second { alloc::format!(", {} stood", s.second_stood) } else { String::new() },
    )
}

/// the host's stored copy of the table: the page hands the Rust game its values, so nothing here   (pressure.ts loadPressure)
pub fn load_pressure(g: &mut crate::game::Game) {}

/// the shipped values back; dropping the stored copy is the host's   (pressure.ts resetPressure)
pub fn reset_pressure(g: &mut crate::game::Game) {
    g.k.pressure = pressure_defaults();
}

/// the host's stored copy of the table: nothing here   (pressure.ts savePressure)
pub fn save_pressure(g: &mut crate::game::Game) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// node's own x.toFixed(d) for each
    #[test]
    fn to_fixed_is_the_languages() {
        for (x, d, want) in [
            (0.125, 2, "0.13"), (0.375, 2, "0.38"), (1.005, 2, "1.00"), (2.675, 2, "2.67"), (0.25, 1, "0.3"), (0.75, 1, "0.8"), (0.5, 0, "1"),
            (2.5, 0, "3"), (1.45, 1, "1.4"), (-0.125, 2, "-0.13"), (3.14159, 2, "3.14"), (1234.5, 0, "1235"), (0.0, 2, "0.00"),
        ] {
            assert_eq!(to_fixed(x, d), want);
        }
        assert_eq!(fixed2(0.125).to_bits(), 0.13f64.to_bits());
        assert_eq!(fixed2(1.005).to_bits(), 1.0f64.to_bits());
    }
}
