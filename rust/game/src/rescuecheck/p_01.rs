//! src/game/rescuecheck.ts lines 1 to 244
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

const NOISE: &str = "heard that — moving to cover it!";
const DOOR: &str = "on my way — moving to the hostage door!";

/// The row, where it lives. Released at the end of each use: everything that takes `g` needs it back.
macro_rules! st {
    ($g:ident) => { $g.m.rescuecheck.states.as_mut().unwrap() };
}

/// The flat distance between two points.   (rescuecheck.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// The fractional part.   (rescuecheck.ts frac)
pub fn frac(x: f64) -> f64 { x - m::floor(x) }

/// Up, on his feet and still in the level.   (rescuecheck.ts alive)
pub fn alive(g: &mut Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    g.gd[gd].state != crate::game::GuardState::Dead && g.ch[c].alive && !crate::runner::runner_fled(g, gd)
}

/// Alive, not held, not dazzled, not staggered: he can speak on the net.   (rescuecheck.ts fit)
pub fn fit(g: &mut Game, gd: GuardId) -> bool {
    alive(g, gd) && g.gd[gd].held.is_none() && g.time >= g.gd[gd].dazzled_until && g.time >= g.gd[gd].stagger_until
}

/// Alert or searching.   (rescuecheck.ts roused)
pub fn roused(g: &mut Game, gd: GuardId) -> bool {
    let s = g.gd[gd].state;
    s == crate::game::GuardState::Alert || s == crate::game::GuardState::Search
}

/// The loud things he has heard, as a running count.   (rescuecheck.ts loudHeard)
pub fn loud_heard(g: &mut Game, gd: GuardId) -> f64 {
    let h = &g.gd[gd].witness.heard;
    h.shots + h.kick + h.bang
}

/// His row on the room fight's roster.   (rescuecheck.ts defOf)
pub fn def_of(g: &mut Game, gd: GuardId) -> Option<RoomfightDefId> {
    for i in 0..g.fight.defs.len() {
        let d = g.fight.defs[i];
        if g.tab.roomfight_def[d].gd == gd { return Some(d); }
    }
    None
}

/// A line on their net: its class on the team's strip, a row of the debrief unless it is the routine check.   (rescuecheck.ts net)
pub fn net(g: &mut Game, gd: GuardId, text: &str, cls: Option<f64>, chron: Option<bool>) {
    let cls = match cls { Some(c) => c, None => crate::cues::cue_class(text) };
    let chron = chron.unwrap_or(true);
    if g.gd[gd].held.is_some() { return; }
    crate::cues::cues_net(g, gd, text, cls);
    if chron { crate::chronicle::chron_radio(g, gd, text); }
    let line = format!("{} (radio): {}", g.gd[gd].callsign, text);
    crate::game::game_msg(g, &line);
}

/// Where in a band of seconds, by a key: the cast's ids and the check's number, no dice.   (rescuecheck.ts band)
pub fn band(b: [f64; 2], k: f64) -> f64 { b[0] + (b[1] - b[0]) * frac(k * 0.6180339887) }

/// A row of the timeline and of the room fight's log.   (rescuecheck.ts event)
pub fn event(g: &mut Game, what: EventWhat, gd: Option<GuardId>, why: &str) {
    let who = match gd { Some(id) => g.gd[id].callsign.clone(), None => String::from("-") };
    let what = what_row(what);
    let t = g.time;
    let st = st!(g);
    st.events.push(RescueCheckEvent { t, what, who: who.clone(), why: String::from(why) });
    if st.events.len() > 120 { st.events.drain(0..40); }
    let note = crate::roomfight::FightNote { t: g.time, who, what: format!("[rescuecheck] {}: {}", what.as_str(), why) };
    g.fight.log.push(note);
    if g.fight.log.len() > 600 { g.fight.log.drain(0..200); }
}

/// The skeleton has two enums of the one union: the parameter's and the row's.
fn what_row(w: EventWhat) -> RescueCheckEventWhat {
    match w {
        EventWhat::Lost => RescueCheckEventWhat::Lost,
        EventWhat::Home => RescueCheckEventWhat::Home,
        EventWhat::There => RescueCheckEventWhat::There,
        EventWhat::Word => RescueCheckEventWhat::Word,
        EventWhat::Found => RescueCheckEventWhat::Found,
        EventWhat::Kept => RescueCheckEventWhat::Kept,
        EventWhat::Nobody => RescueCheckEventWhat::Nobody,
        EventWhat::Empty => RescueCheckEventWhat::Empty,
        EventWhat::Check => RescueCheckEventWhat::Check,
        EventWhat::Asks => RescueCheckEventWhat::Asks,
        EventWhat::Abort => RescueCheckEventWhat::Abort,
        EventWhat::Goes => RescueCheckEventWhat::Goes,
    }
}

/// The rule's floor: a rescue staged or running, in live play.   (rescuecheck.ts ruled)
pub fn ruled(g: &mut Game) -> bool {
    if !g.rescue.active || g.touring || crate::director::director_showing(g) { return false; }
    let s = crate::rescue::rescue_state(g);
    s == crate::rescue::RescueState::Staged || s == crate::rescue::RescueState::Running
}

/// This man is out on the check's errand and has noticed nothing: his walk is not word of the team.   (rescuecheck.ts rescueCheckOwns)
pub fn rescue_check_owns(g: &mut Game, gd: GuardId) -> bool {
    let out = match &g.m.rescuecheck.states { Some(st) => st.out.as_ref().map(|o| o.gd), None => None };
    out == Some(gd) && !roused(g, gd) && g.gd[gd].state != crate::game::GuardState::Suspicious && g.gd[gd].awareness < 0.3
}

/// The check's and the debugger's timeline.   (rescuecheck.ts rescueCheckTimeline)
pub fn rescue_check_timeline(g: &mut Game) -> Vec<RescueCheckEvent> {
    match &g.m.rescuecheck.states { Some(st) => st.events.clone(), None => Vec::new() }
}

/// A staging, a restart.   (rescuecheck.ts rescueCheckReset)
pub fn rescue_check_reset(g: &mut Game) { g.m.rescuecheck.states = None; }

/// Who goes: a roamer on his own post first, then the door man or a guard posted outside the hostage room, the nearest by the straight line;
/// a man any other rule has is not free.   (rescuecheck.ts goer)
pub fn goer(g: &mut Game, cast: &[CastItem], about: GuardId, post: V3) -> Option<Goer> {
    let mut first: Vec<CastItem> = Vec::new();
    for c in cast.iter().copied() {
        if c.gd != about && !c.in_room && c.role != crate::rescue::Role::Captor && c.learned_at.is_nan() && !c.released && !c.crewed
            && g.gd[c.gd].state != crate::game::GuardState::Suspicious { first.push(c); }
    }
    let mut with_def: Vec<(CastItem, Option<RoomfightDefId>)> = Vec::new();
    for c in first { let d = def_of(g, c.gd); with_def.push((c, d)); }
    let mut free: Vec<(CastItem, RoomfightDefId)> = Vec::new();
    for (c, d) in with_def {
        if let Some(d) = d {
            if crate::powerout::power_out_kept(g, d).is_none() { free.push((c, d)); }
        }
    }
    let rank = |c: &CastItem| if c.role == crate::rescue::Role::Roamer { 0.0 } else { 1.0 };
    let read: &Game = &*g;
    let at = |c: &CastItem| read.ch[read.gd[c.gd].ch].pos;
    js::sort_by(&mut free, |a, b| js::or(rank(&a.0) - rank(&b.0), flat(at(&a.0), post) - flat(at(&b.0), post)));
    free.first().map(|p| Goer { d: p.1, gd: p.0.gd })
}

/// Where a man is looked for: where he was going, else his post, else where he stands.   (rescuecheck.ts postOf)
pub fn post_of(g: &mut Game, cast: &[CastItem], gd: GuardId) -> V3 {
    if let Some(p) = st!(g).where_.get(gd) { return *p; }
    if let Some(c) = cast.iter().find(|c| c.gd == gd) { return c.post; }
    g.ch[g.gd[gd].ch].pos
}

/// Nothing of the team in his eye since he learned of them (no learned time: in the last half minute).   (rescuecheck.ts unseen)
pub fn unseen(g: &mut Game, c_2: CastItem) -> bool {
    let since = if c_2.learned_at.is_nan() { g.time - 30.0 } else { c_2.learned_at - 1.0 };
    !(g.gd[c_2.gd].last_seen_t > since)
}

/// A leg of the walk at the pace of a man going to look.   (rescuecheck.ts walk)
pub fn walk(g: &mut Game, d: RoomfightDefId, to: V3) {
    crate::pressure::run(g, d, to, None, None);
    let gd = g.tab.roomfight_def[d].gd;
    let speed = g.tune.guard_investigate;
    crate::squad::order(g, gd, crate::squad::OrderPatch { speed: Some(Some(speed)), ..Default::default() });
}

/// Once a frame, before the room fight's own tick.   (rescuecheck.ts rescueCheckTick)
pub fn rescue_check_tick(g: &mut Game, _dt: f64) {
    if !ruled(g) {
        if g.m.rescuecheck.states.is_some() { let_go(g); g.m.rescuecheck.states = None; }
        return;
    }
    let cast_owned = crate::rescue::rescue_cast(g);
    let cast: &[CastItem] = &cast_owned;
    if cast.len() < 2 { return; }
    if g.m.rescuecheck.states.is_none() {
        let lead = match cast.iter().find(|c| c.role == crate::rescue::Role::Captor) { Some(c) => *c, None => cast[0] };
        let id = g.ch[g.gd[lead.gd].ch].id;
        g.m.rescuecheck.states = Some(State {
            next_at: g.time + band(g.k.rescuecheck.first, id),
            n: 0.0,
            stage: StateStage::Idle,
            t0: 0.0,
            caller: None,
            about: None,
            known: JsSet::new(),
            where_: JsMap::new(),
            worded: JsMap::new(),
            kept: JsSet::new(),
            out: None,
            events: Vec::new(),
        });
    }
    let k = g.k.rescuecheck;
    // a roamer about to be let off his post says where he is going (not a man with the team in his eye)
    for c in cast.iter().copied() {
        if c.role == crate::rescue::Role::Roamer && !c.released && !c.crewed && !c.learned_at.is_nan() && !st!(g).worded.has(c.gd) && fit(g, c.gd) && unseen(g, c)
            && g.time >= c.learned_at + g.k.rescue.set_secs - k.word_secs {
            let at = crate::rescue::rescue_noise_at(g);
            st!(g).worded.set(c.gd, if at.is_some() { StateWordedValue::Noise } else { StateWordedValue::Door });
            net(g, c.gd, if at.is_some() { NOISE } else { DOOR }, Some(1.0), None);
            event(g, EventWhat::Word, Some(c.gd), &format!("{} s before the rescue lets him off his post", k.word_secs));
        }
    }
    // HIS WORD IS KEPT: let off his post with a noise known and nothing of the team seen since he learned, his fix is that noise
    for c in cast.iter().copied() {
        if c.role == crate::rescue::Role::Roamer && c.released && !c.crewed && !st!(g).kept.has(c.gd) {
            st!(g).kept.add(c.gd);
            let at = crate::rescue::rescue_noise_at(g);
            let gd = c.gd;
            if !fit(g, gd) || g.gd[gd].state != crate::game::GuardState::Alert || !unseen(g, c) { continue; }
            if let Some(at) = at {
                if st!(g).worded.get(gd).copied() != Some(StateWordedValue::Noise) {
                    st!(g).worded.set(gd, StateWordedValue::Noise);
                    net(g, gd, NOISE, Some(1.0), None);
                }
                g.gd[gd].last_known = Some(v3::copy(at));
                crate::guards::reset_chase(g, gd);
                event(g, EventWhat::Kept, Some(gd), "his fix is the noise");
            } else if !st!(g).worded.has(gd) {
                st!(g).worded.set(gd, StateWordedValue::Door);
                net(g, gd, DOOR, Some(1.0), None);
            }
        }
    }
    let mut fight = false;
    for c in cast.iter() {
        if alive(g, c.gd) && roused(g, c.gd) { fight = true; break; }
    }
    if fight {   // their side is in it: who is down is known, nobody is asked for, the man out runs home
        for c in cast.iter() {
            if !alive(g, c.gd) { st!(g).known.add(c.gd); }
        }
        if st!(g).stage != StateStage::Idle { st!(g).stage = StateStage::Idle; st!(g).about = None; }
        st!(g).next_at = m::max(st!(g).next_at, g.time + k.every[0]);
    }
    if st!(g).out.is_some() { tick_out(g, cast, fight); }
    if fight || st!(g).out.is_some() { return; }
    if st!(g).stage == StateStage::Idle {
        if g.time < st!(g).next_at { return; }
        let by = caller(g, cast);
        st!(g).n += 1.0;
        let id = match by { Some(b) => g.ch[g.gd[b].ch].id, None => 1.0 };
        st!(g).next_at = g.time + band(k.every, id + st!(g).n * 7.0);
        let by = match by { Some(b) => b, None => return };
        st!(g).caller = Some(by);
        st!(g).stage = StateStage::Called;
        st!(g).t0 = g.time;
        net(g, by, "radio check — sound off", Some(6.0), Some(false));
        event(g, EventWhat::Check, Some(by), "calls the radio check");
        return;
    }
    let was = st!(g).caller;
    let by = match was {
        Some(c) if fit(g, c) => Some(c),
        _ => caller(g, cast),
    };
    let by = match by { Some(b) => b, None => { st!(g).stage = StateStage::Idle; return; } };
    if st!(g).stage == StateStage::Called {
        if g.time < st!(g).t0 + k.ask_secs { return; }
        // a man down in the hostage room is never a question on the net: that room is the rescue's own
        for c in cast.iter() {
            if c.in_room && !alive(g, c.gd) { st!(g).known.add(c.gd); }
        }
        let mut miss = None;
        for c in cast.iter() {
            if !alive(g, c.gd) && !st!(g).known.has(c.gd) && c.gd != by { miss = Some(c.gd); break; }
        }
        let miss = match miss { Some(x) => x, None => { st!(g).stage = StateStage::Idle; return; } };
        st!(g).about = Some(miss);
        st!(g).stage = StateStage::Asked;
        st!(g).t0 = g.time;
        let post = post_of(g, cast, miss);
        let place = crate::dialogue::place_name(g, post);
        net(g, by, &format!("{}, sound off … {}?", place, place), Some(2.0), None);
        let why = format!("no answer from {}: asked for by his post, {}", g.gd[miss].callsign, place);
        event(g, EventWhat::Asks, Some(by), &why);
        return;
    }
    if g.time < st!(g).t0 + k.go_secs { return; }
    let about = st!(g).about.unwrap();
    let post = post_of(g, cast, about);
    let place = crate::dialogue::place_name(g, post);
    st!(g).stage = StateStage::Idle;
    st!(g).about = None;
    st!(g).known.add(about);
    let go = match goer(g, cast, about, post) {
        Some(x) => x,
        None => {
            net(g, by, &format!("nothing from {} — nobody to spare. Stay on your doors!", place), Some(3.0), None);
            let why = format!("nobody outside the hostage room is free to walk to {}'s post", g.gd[about].callsign);
            event(g, EventWhat::Nobody, Some(by), &why);
            return;
        }
    };
    let from = v3::copy(g.tab.roomfight_def[go.d].station);
    let heard0 = loud_heard(g, go.gd);
    let calm = crate::powerout::calm_of(g, go.d);
    st!(g).out = Some(Out { gd: go.gd, about, goal: v3::copy(post), from, phase: Phase::Out, there_t: -1.0, tries: 0.0, step_t: -100.0, heard0, calm });
    let line = crate::words::wg(g, &format!("nothing from {} — I'm heading over!", place), &format!("nothing from {} — one moving there!", place));
    net(g, go.gd, &line, Some(1.0), None);
    walk(g, go.d, post);
    let why = format!("walks to where {} was posted ({}), {} m off", g.gd[about].callsign, place, to_fixed(flat(g.ch[g.gd[go.gd].ch].pos, post), 0));
    event(g, EventWhat::Goes, Some(go.gd), &why);
}

/// Who calls: the captor if he can, else the first man who can.   (the closure at rescuecheck.ts:153)
fn caller(g: &mut Game, cast: &[CastItem]) -> Option<GuardId> {
    for c in cast.iter() {
        if c.role == crate::rescue::Role::Captor && fit(g, c.gd) { return Some(c.gd); }
    }
    for c in cast.iter() {
        if fit(g, c.gd) { return Some(c.gd); }
    }
    None
}

/// The man who went, a step on.   (rescuecheck.ts tickOut)
pub fn tick_out(g: &mut Game, cast: &[CastItem], fight: bool) {
    let k = g.k.rescuecheck;
    let (gd, about, m_goal, m_from, phase, there_t, tries, step_t, heard0) = {
        let o = st!(g).out.as_ref().unwrap();
        (o.gd, o.about, o.goal, o.from, o.phase, o.there_t, o.tries, o.step_t, o.heard0)
    };
    let d = def_of(g, gd);
    // down or taken on his way: he is missed at the next check like any man, where he was going; and the post he never reached is a question again
    if !alive(g, gd) || d.is_none() || g.gd[gd].held.is_some() {
        st!(g).where_.set(gd, v3::copy(m_goal));
        if phase != Phase::Back { st!(g).known.delete(about); }
        let why = if !alive(g, gd) { "down on his way" } else if g.gd[gd].held.is_some() { "taken on his way" } else { "no man of the fight's any more" };
        event(g, EventWhat::Lost, Some(gd), why);
        st!(g).out = None;
        return;
    }
    let d = d.unwrap();
    if roused(g, gd) {
        event(g, EventWhat::Abort, Some(gd), "he is roused (a body, or the team, in his eye): his own from here");
        st!(g).out = None;
        return;
    }
    let step = crate::pressure::boots(g, gd, step_t);
    st!(g).out.as_mut().unwrap().step_t = step;
    if g.tab.roomfight_def[d].mode != crate::roomfight::DefMode::Move {
        event(g, EventWhat::Abort, Some(gd), "the room fight has him where he stands");
        st!(g).out = None;
        return;
    }
    let learned = match cast.iter().find(|c| c.gd == gd) { Some(c) => !c.learned_at.is_nan(), None => false };
    if fight || learned || loud_heard(g, gd) > heard0 {
        crate::pressure::run(g, d, m_from, None, None);
        let why = format!("{}: back to his post at a run", if fight { "a fight on their side" } else if learned { "something of the team in his ears: the rescue has him" } else { "a shot, a boot or a charge in his ears" });
        event(g, EventWhat::Abort, Some(gd), &why);
        st!(g).out = None;
        return;
    }
    // a man of theirs on the floor, not yet found: his to call, and he is his own from here
    let mut down = None;
    if phase != Phase::Back {
        for c in cast.iter() {
            if !alive(g, c.gd) && g.gd[c.gd].state == crate::game::GuardState::Dead && !g.gd[c.gd].found
                && crate::guards::body_seen(g, gd, c.gd, if phase == Phase::There && c.gd == about { Some(k.feet_m) } else { None }) {
                down = Some(c.gd);
                break;
            }
        }
    }
    if let Some(down) = down {
        let dc = g.gd[down].ch;
        let body = match g.ch[dc].bones.get(crate::character::Bone::Hips) { Some(p) => *p, None => g.ch[dc].pos };
        let place = crate::dialogue::place_name(g, body);
        g.gd[down].found = true;
        crate::mission::tally(g, crate::mission::TallyK::Bodies);
        crate::chronicle::chron_body_found(g, down, gd);
        st!(g).known.add(down);
        g.gd[gd].saw_body_t = g.time;
        let victim = g.gd[down].callsign.clone();
        let breathing = crate::game::is_breathing(g, down);
        g.gd[gd].witness.saw_body = Some(crate::game::WitnessSawBody { victim, t: g.time, breathing });
        if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Alert)); }
        {
            let man = &mut g.gd[gd];
            man.state = crate::game::GuardState::Alert;
            man.awareness = 1.0;
            man.react_t = 0.0;
            man.hold = false;
            man.pinned = false;
            man.last_known = Some([body[0], 0.0, body[2]]);
        }
        crate::guards::reset_chase(g, gd);
        // (the way the rescue lets a roamer go: he stays on the room fight's roster and holds again where he ends up)
        let now = g.time;
        {
            let def = &mut g.tab.roomfight_def[d];
            def.mode = crate::roomfight::DefMode::Free;
            def.free_at = now;
            def.rally_at = f64::NAN;
        }
        net(g, gd, &format!("man down, {} — they're inside!", place), Some(1.0), None);
        let why = format!("{} on the floor in his eye, {} m off: roused, and his own from here", g.gd[down].callsign, to_fixed(flat(g.ch[g.gd[gd].ch].pos, body), 1));
        event(g, EventWhat::Found, Some(gd), &why);
        st!(g).out = None;
        return;
    }
    let goal = if phase == Phase::Back { m_from } else { m_goal };
    let arrived = match &g.gd[gd].script { Some(s) => s.arrived.unwrap_or(false), None => false };
    let reached = arrived || flat(g.ch[g.gd[gd].ch].pos, goal) < 0.6;
    if phase == Phase::There {
        // (he stands where he was sent: still his leg)
        if let Some(s) = g.gd[gd].script.as_mut() { s.arrived = Some(false); }
        g.tab.roomfight_def[d].stall_t = 0.0;
        g.tab.roomfight_def[d].move_t = 0.0;
        if g.time - there_t < k.wait_secs { return; }
        let place = crate::dialogue::place_name(g, m_goal);
        net(g, gd, &format!("nothing at {} — he's gone!", place), Some(2.0), None);
        let why = format!("{} s at {}'s post and nothing in his eye: back to his own", k.wait_secs, g.gd[about].callsign);
        event(g, EventWhat::Empty, Some(gd), &why);
        let o = st!(g).out.as_mut().unwrap();
        o.phase = Phase::Back;
        o.tries = 0.0;
        walk(g, d, m_from);
        return;
    }
    if !reached {
        if g.tab.roomfight_def[d].stall_t <= 1.3 && g.tab.roomfight_def[d].move_t <= 8.0 { return; }   // on his way
        if tries < k.tries {
            st!(g).out.as_mut().unwrap().tries += 1.0;
            walk(g, d, goal);
            return;
        }
        if phase == Phase::Back {
            event(g, EventWhat::Abort, Some(gd), "no way back to his post: the room fight plants him where he is");
            st!(g).out = None;
            return;
        }
        let o = st!(g).out.as_mut().unwrap();
        o.phase = Phase::Back;
        o.tries = 0.0;
        walk(g, d, m_from);
        event(g, EventWhat::Abort, Some(gd), "no way to the post: back to his own");
        return;
    }
    if phase == Phase::Back {
        let calm = st!(g).out.as_ref().unwrap().calm.clone();
        crate::powerout::calm_back(g, d, &calm, false);
        event(g, EventWhat::Home, Some(gd), "back on his post");
        st!(g).out = None;
        return;
    }
    let now = g.time;
    let o = st!(g).out.as_mut().unwrap();
    o.phase = Phase::There;
    o.there_t = now;
    if let Some(s) = g.gd[gd].script.as_mut() { s.arrived = Some(false); }
    g.tab.roomfight_def[d].stall_t = 0.0;
    g.tab.roomfight_def[d].move_t = 0.0;
    let why = format!("at {}'s post", g.gd[about].callsign);
    event(g, EventWhat::There, Some(gd), &why);
}

/// The rule let go of with a man out: he runs back to his post and is the room fight's from there.   (rescuecheck.ts letGo)
pub fn let_go(g: &mut Game) {
    let o = match st!(g).out.take() { Some(o) => o, None => return };
    let d = def_of(g, o.gd);
    if let Some(d) = d {
        if g.tab.roomfight_def[d].mode == crate::roomfight::DefMode::Move && alive(g, o.gd) && g.gd[o.gd].held.is_none() && !roused(g, o.gd) {
            crate::pressure::run(g, d, o.from, None, None);
        }
    }
}

/// `x.toFixed(digits)` of a length, 0 or more, for the log's text: the language's tie goes up, Rust's format goes to even. A tie needs `x` to be
/// an odd multiple of 2^-(digits + 1).
fn to_fixed(x: f64, digits: usize) -> String {
    let (scale, up) = if digits == 0 { (2.0, 0.25) } else { (4.0, 0.025) };
    let y = x * scale;
    let tie = y == m::floor(y) && js::rem(y, 2.0) != 0.0;
    format!("{:.*}", digits, if tie { x + up } else { x })
}

#[cfg(test)]
mod tests {
    use super::*;

    // The bits are the TypeScript's own, printed under bun with EXACT on.
    #[test]
    fn flat_has_the_typescripts_bits() {
        assert_eq!(flat([1.3, 9.0, -7.7], [4.25, 2.0, 11.1]).to_bits(), 0x403307b0d5c6522b);
        assert_eq!(flat([0.1, 0.0, 0.2], [0.1, 5.0, 0.2]).to_bits(), 0);
        assert_eq!(flat([-12.625, 0.0, 33.3], [17.77, 0.0, -0.01]).to_bits(), 0x40468bf3964912ae);
    }
    #[test]
    fn frac_has_the_typescripts_bits() {
        assert_eq!(frac(3.7).to_bits(), 0x3fe6666666666668);
        assert_eq!(frac(-1.25).to_bits(), 0x3fe8000000000000);
        assert_eq!(frac(6.180339887).to_bits(), 0x3fc715609e6a27a0);
    }
    #[test]
    fn band_has_the_typescripts_bits() {
        assert_eq!(band([14.0, 22.0], 7.0).to_bits(), 0x40309c22a087d824);
        assert_eq!(band([24.0, 36.0], 8.0).to_bits(), 0x4041aa66d2bd94b0);
        assert_eq!(band([24.0, 36.0], 134.0).to_bits(), 0x4040e63a49e77a98);
        assert_eq!(band([14.0, 22.0], 0.0).to_bits(), 0x402c000000000000);
    }
    #[test]
    fn to_fixed_says_what_the_language_says() {
        let cases: [(f64, usize, &str); 9] = [(0.5, 0, "1"), (2.5, 0, "3"), (3.5, 0, "4"), (0.25, 1, "0.3"), (0.75, 1, "0.8"), (1.45, 1, "1.4"), (12.34, 0, "12"), (12.34, 1, "12.3"), (1.05, 1, "1.1")];
        for (x, f, want) in cases { assert_eq!(to_fixed(x, f), want); }
    }
}
