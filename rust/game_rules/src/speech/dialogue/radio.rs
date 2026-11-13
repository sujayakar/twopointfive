                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::js::{self, JsSet};
use crate::m;
use alloc::{string::String, vec::Vec};

/// The game's radio row, with no restart: `radio_state` has been called, this is for going back to it after a call that takes `g`.
fn row(g: &mut Game) -> &mut crate::dialogue::RadioState {
    g.m.dialogue.radio_by_game.as_mut().unwrap()
}

/// `g.guards.filter(x => x.found).length`
fn found_count(g: &Game) -> f64 {
    let mut n = 0.0;
    for &x in g.guards.iter() { if g.gd[x].found { n += 1.0; } }
    n
}

/// The radio's row for this game, restarted on a fresh run: every beat may sound again, and the edges start from where the floor stands.
fn radio_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::RadioState {
    if g.m.dialogue.radio_by_game.is_none() {
        g.m.dialogue.radio_by_game = Some(crate::dialogue::RadioState { run: None, done: JsSet::new(), queue: Vec::new(), reply: None, last_at: -100.0, esc: 0.0, blackout: false, found: 0.0 });
    }
    let now = Some(g.mission.serial);
    if row(g).run != now {   // a Mission by identity: restartEncounter and a load deal it a new serial
        let r = row(g);
        r.run = now;
        r.done.clear();
        r.queue.clear();
        r.reply = None;
        r.last_at = -100.0;
        let esc = crate::guards::escalation_of(g);
        let blackout = g.light_home.mains.active;
        let found = found_count(g);
        let r = row(g);
        r.esc = esc;
        r.blackout = blackout;
        r.found = found;
    }
    row(g)
}

/// The run is really the player's and still going, Sam is on his feet, and it is the night she talks him through.
fn radio_live(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let s = g.mission.stage;
    !g.aside && g.ai_enabled
        && s != crate::game::MissionStage::Done && s != crate::game::MissionStage::Failed
        && !crate::condition::down(g, g.pl[seat.player].ch)
        && !g.staged.facts.fight
        && crate::dialogue::is_office(g)
}

/// Nobody is fighting, and Sam's mouth and hands are free.
fn radio_clear(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    !g.guards.iter().any(|&x| g.gd[x].state == crate::game::GuardState::Alert)
        && !g.pl[seat.player].on_a_man()
}

/// Sound a beat now (no delay) or queue it `delay` seconds out. False if it will never sound. The variant is dealt off the beat and the run's start.
pub fn handler_beat(g: &mut crate::game::Game, beat: crate::dialogue::HandlerBeat, delay: Option<f64>) -> bool {
    let delay = delay.unwrap_or(0.0);
    radio_state(g);
    if !radio_live(g, &crate::seats::here(g)) || row(g).done.has(beat) { return false; }
    if delay > 0.0 {
        let at = g.time + delay;
        let r = row(g);
        if !r.queue.iter().any(|q| q.beat == beat) { r.queue.push(crate::dialogue::RadioStateQueueItem { at, beat }); }
        return true;
    }
    if !radio_clear(g, &crate::seats::here(g)) { return false; }
    let now = g.time;
    let r = row(g);
    if now - r.last_at < 5.0 || r.reply.is_some() { return false; }
    let table = crate::dialogue::handler_lines();
    let Some(pool) = table.of(beat.as_str()) else { return false };
    if pool.is_empty() { return false; }
    let dealt = crate::dice::hash_str(&alloc::format!("{}|{}", beat.as_str(), js::num_str(m::round(g.mission.t0 * 10.0))));
    let l = &pool[js::rem(dealt, pool.len() as f64) as usize];
    let own = !crate::dialogue::is_office(g);
    let h_now = l.h_now.as_deref().unwrap_or(&l.h);
    let h = if own { l.h_own.as_deref().unwrap_or(h_now) } else { h_now };
    let s = if own { l.s_own.as_deref().unwrap_or(&l.s) } else { &l.s };
    let r = row(g);
    r.done.add(beat);
    r.last_at = now;
    crate::cues::cues_radio(g, &alloc::format!("{} \u{25B8} {}", crate::dialogue::HANDLER_TAG, h));
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Radio, None, Some(0.3), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.12 }));
    let at = g.time + crate::dialogue::RADIO_REPLY_SECS;
    row(g).reply = Some(crate::dialogue::RadioStateReply { at, text: String::from(s) });
    true
}

/// Once a frame: the edges she reacts to, queued beats coming due, and his reply when its moment comes.
pub(super) fn tick_radio(g: &mut crate::game::Game) {
    radio_state(g);
    let esc = crate::guards::escalation_of(g);
    if esc > row(g).esc {
        handler_beat(g, if esc >= 2.0 { crate::dialogue::HandlerBeat::Lockdown } else { crate::dialogue::HandlerBeat::Alarm }, Some(crate::dialogue::RADIO_EDGE_DELAY));
    }
    row(g).esc = esc;
    let bo = g.light_home.mains.active;
    if bo && !row(g).blackout {
        handler_beat(g, crate::dialogue::HandlerBeat::Blackout, Some(crate::dialogue::RADIO_EDGE_DELAY));
    }
    row(g).blackout = bo;
    let (mut found, mut ko) = (0.0, false);
    let known = row(g).found;
    let mut i = 0;
    while i < g.guards.len() {
        let x = g.guards[i];
        if g.gd[x].found {
            found += 1.0;
            if found > known && crate::game::is_breathing(g, x) { ko = true; }
        }
        i += 1;
    }
    if found > known {
        handler_beat(g, if ko { crate::dialogue::HandlerBeat::BodyKo } else { crate::dialogue::HandlerBeat::BodyDead }, Some(crate::dialogue::RADIO_EDGE_DELAY + 0.6));
    }
    row(g).found = found;
    let mut i = row(g).queue.len();
    while i > 0 {   // backward, because a row may be taken out on the way
        i -= 1;
        let q = row(g).queue[i];
        if g.time < q.at { continue; }
        if !radio_live(g, &crate::seats::here(g)) || row(g).done.has(q.beat) {
            row(g).queue.remove(i);
            continue;
        }
        if handler_beat(g, q.beat, None) {
            row(g).queue.remove(i);
        } else if g.time - q.at > 20.0 {   // a beat that cannot find a quiet moment inside twenty seconds has missed it
            row(g).queue.remove(i);
        }
    }
    let now = g.time;
    let due = match &row(g).reply { Some(p) => now >= p.at, None => false };
    if due {
        let p = row(g).reply.take().unwrap();
        if radio_live(g, &crate::seats::here(g)) && radio_clear(g, &crate::seats::here(g)) {
            let now = g.time;
            let pl = g.player;
            let talking = match &g.pl[pl].bubble { Some(b) => now - b.t < b.dur, None => false };
            if !talking {
                let dur = m::min(2.4, m::max(1.3, 0.9 + 0.025 * p.text.encode_utf16().count() as f64));   // .length: UTF-16 units
                let said = alloc::format!("you \u{25B8} {}", p.text);
                g.pl[pl].bubble = Some(crate::game::PlayerBubble { text: p.text, t: now, dur, radio: true });
                crate::cues::cues_radio(g, &said);
            }
        }
    }
}

