//! src/game/dialogue.ts lines 1392 to 1767
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- seams (the skeleton lacks these; notes.md, ## Skeleton)

/// `g.audio.play(name, pos, gain, { rate })` (dialogue.ts:1738): `game_audio_like_play`'s opts hold `crate::Opaque`, which cannot carry a rate.
fn seam_audio_play(g: &mut Game, name: &str, pos: Option<V3>, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

// ---------------------------------------------------------------- the functions

/// Every line of Sam's the scripts and the rooms can put in his mouth, into SAM_LINES as `script` entries.   (dialogue.ts registerScriptLines)
pub fn register_script_lines(g: &mut crate::game::Game) {
    let mut lines = script_sam_lines();
    g.m.dialogue.sam_lines.append(&mut lines);
    for (room, list) in crate::dialogue::room_menace().iter() {
        for (i, r) in list.iter().enumerate() {
            g.m.dialogue.sam_lines.push(script_line(format!("S:room:{}:{}", room, i), None, r.sam.clone(), None));
            if let Some(after) = r.after.as_deref().filter(|a| !a.is_empty()) {
                g.m.dialogue.sam_lines.push(script_line(format!("S:room:{}:{}+", room, i), None, String::from(after), None));
            }
        }
    }
}

/// `sc.beats[s]`. ScriptBeats holds all four, as `Record<StageKey, Beat>` says.
fn beat_at(beats: &crate::dialogue::ScriptBeats, s: crate::dialogue::StageKey) -> &crate::dialogue::Beat {
    match s {
        crate::dialogue::StageKey::X => &beats.x,
        crate::dialogue::StageKey::Num(n) if n == 1.0 => &beats._1,
        crate::dialogue::StageKey::Num(n) if n == 2.0 => &beats._2,
        crate::dialogue::StageKey::Num(_) => &beats._3,
    }
}

/// `String(s)` of a stage.
fn stage_word(s: crate::dialogue::StageKey) -> String {
    match s { crate::dialogue::StageKey::X => String::from("x"), crate::dialogue::StageKey::Num(n) => js_int(n) }
}

/// The scripts' half of registerScriptLines: every script x STAGES, in that order. An empty ask, rejoinder or take is falsy, as `if (b.ask)` has it.
fn script_sam_lines() -> Vec<crate::dialogue::SamLine> {
    let mut out = Vec::new();
    for sc in crate::dialogue::scripts().iter() {
        for s in crate::dialogue::stages() {
            let b = beat_at(&sc.beats, s);
            let base = format!("S:{}:{}", sc.id, stage_word(s));
            out.push(script_line(base.clone(), Some(sc.t), b.sam.clone(), b.sam_now.clone()));
            if let Some(x) = b.ask.as_deref().filter(|x| !x.is_empty()) { out.push(script_line(format!("{}?", base), Some(sc.t), String::from(x), None)); }
            if let Some(x) = b.after.as_deref().filter(|x| !x.is_empty()) { out.push(script_line(format!("{}+", base), Some(sc.t), String::from(x), None)); }
            if let Some(x) = b.take.as_deref().filter(|x| !x.is_empty()) { out.push(script_line(format!("{}+?", base), Some(sc.t), String::from(x), None)); }
        }
    }
    out
}

/// One `{ id, s: 'script', t, text, guide }` of registerScriptLines.
fn script_line(id: String, t: Option<crate::game::Temperament>, text: String, guide: Option<String>) -> crate::dialogue::SamLine {
    crate::dialogue::SamLine { id, s: crate::dialogue::SamLineS::Script, t, text, guide, ..Default::default() }
}

/// The row the TypeScript's `R` holds, with no restart: `radio_state` has been called, this is for going back to it after a call that takes `g`.
fn row(g: &mut Game) -> &mut crate::dialogue::RadioState {
    g.m.dialogue.radio_by_game.as_mut().unwrap()
}

/// `g.guards.filter(x => x.found).length`
fn found_count(g: &mut Game) -> f64 {
    let mut n = 0.0;
    for &x in g.guards.iter() { if g.gd[x].found { n += 1.0; } }
    n
}

/// The radio's row for this game, restarted on a fresh run: every beat may sound again, and the edges start from where the floor stands.   (dialogue.ts radioState)
pub fn radio_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::RadioState {
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
        let blackout = crate::sim_lightshome::mains_active(g);
        let found = found_count(g);
        let r = row(g);
        r.esc = esc;
        r.blackout = blackout;
        r.found = found;
    }
    row(g)
}

/// The run is really the player's and still going, Sam is on his feet, and it is the night she talks him through.   (dialogue.ts radioLive)
pub fn radio_live(g: &mut crate::game::Game) -> bool {
    let s = g.mission.stage;
    !g.quiet_utility && g.puppet.is_none() && !g.touring && g.ai_enabled
        && s != crate::game::MissionStage::Done && s != crate::game::MissionStage::Failed
        && !g.pl[g.player].down
        && !(g.night_breach.active || g.rescue.active)
        && crate::dialogue::is_office(g)
}

/// Nobody is fighting, and Sam's mouth and hands are free.   (dialogue.ts radioClear)
pub fn radio_clear(g: &mut crate::game::Game) -> bool {
    !g.guards.iter().any(|&x| g.gd[x].state == crate::game::GuardState::Alert)
        && g.pl[g.player].holding.is_none()
        && g.pl[g.player].takedown.is_none()
}

/// Sound a beat now (no delay) or queue it `delay` seconds out. False if it will never sound. The variant is dealt off the beat and the run's start.   (dialogue.ts handlerBeat)
pub fn handler_beat(g: &mut crate::game::Game, beat: crate::dialogue::HandlerBeat, delay: Option<f64>) -> bool {
    let delay = delay.unwrap_or(0.0);
    radio_state(g);
    if !radio_live(g) || row(g).done.has(beat) { return false; }
    if delay > 0.0 {
        let at = g.time + delay;
        let r = row(g);
        if !r.queue.iter().any(|q| q.beat == beat) { r.queue.push(crate::dialogue::RadioStateQueueItem { at, beat }); }
        return true;
    }
    if !radio_clear(g) { return false; }
    let now = g.time;
    let r = row(g);
    if now - r.last_at < 5.0 || r.reply.is_some() { return false; }
    let table = crate::dialogue::handler_lines();
    let Some(pool) = table.get(String::from(beat.as_str())) else { return false };
    if pool.is_empty() { return false; }
    let dealt = crate::math_dice::hash_str(&format!("{}|{}", beat.as_str(), js_int(m::round(g.mission.t0 * 10.0))));
    let l = &pool[js::rem(dealt, pool.len() as f64) as usize];
    let rack = js_int(g.level.mission.rack.index + 1.0);
    let own = !crate::dialogue::is_office(g);
    let h_now = l.h_now.as_deref().unwrap_or(&l.h);
    let h = if own { l.h_own.as_deref().unwrap_or(h_now) } else { h_now };
    let s = if own { l.s_own.as_deref().unwrap_or(&l.s) } else { &l.s };
    let r = row(g);
    r.done.add(beat);
    r.last_at = now;
    crate::game::game_msg(g, &format!("radio: {} \u{25B8} {}", crate::dialogue::HANDLER_TAG, h.replace("{rack}", &rack)));
    seam_audio_play(g, "radio", None, 0.3, 1.12);
    let at = g.time + crate::dialogue::RADIO_REPLY_SECS;
    row(g).reply = Some(crate::dialogue::RadioStateReply { at, text: s.replace("{rack}", &rack) });
    true
}

/// Once a frame: the edges she reacts to, queued beats coming due, and his reply when its moment comes.   (dialogue.ts tickRadio)
pub fn tick_radio(g: &mut crate::game::Game) {
    radio_state(g);
    let esc = crate::guards::escalation_of(g);
    if esc > row(g).esc {
        handler_beat(g, if esc >= 2.0 { crate::dialogue::HandlerBeat::Lockdown } else { crate::dialogue::HandlerBeat::Alarm }, Some(crate::dialogue::RADIO_EDGE_DELAY));
    }
    row(g).esc = esc;
    let bo = crate::sim_lightshome::mains_active(g);
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
    while i > 0 {   // backward, with splice, as the TypeScript walks it
        i -= 1;
        let q = row(g).queue[i];
        if g.time < q.at { continue; }
        if !radio_live(g) || row(g).done.has(q.beat) {
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
        if radio_live(g) && radio_clear(g) {
            let now = g.time;
            let pl = g.player;
            let talking = match &g.pl[pl].bubble { Some(b) => now - b.t < b.dur, None => false };
            if !talking {
                let dur = m::min(2.4, m::max(1.3, 0.9 + 0.025 * p.text.encode_utf16().count() as f64));   // .length: UTF-16 units
                let said = format!("radio: you \u{25B8} {}", p.text);
                g.pl[pl].bubble = Some(crate::game::PlayerBubble { text: p.text, t: now, dur, radio: true });
                crate::game::game_msg(g, &said);
            }
        }
    }
}

/// A pair line with its tokens at their longest, for the lint's length check and the length report.   (dialogue.ts pairWorst)
pub fn pair_worst(text: &str) -> String {
    let table = crate::dialogue::token_worst_table();
    let pad = table.get(String::from("padDoor")).unwrap();
    let code = table.get(String::from("code")).unwrap();
    let t = replace_ascii_ci(text, "{other}", "Lindqvist");
    let t = replace_ascii_ci(&t, "{code}", code);
    let t = t.replace("{padDoor}", pad);
    let mut chars = pad.chars();
    let mut cap = String::new();
    if let Some(first) = chars.next() { cap.extend(first.to_uppercase()); cap.push_str(chars.as_str()); }
    t.replace("{PadDoor}", &cap)
}

/// `text.replace(/<pat>/gi, by)` for an ASCII `pat` (a non-unicode `i` regex folds no other letter onto an ASCII one here).
fn replace_ascii_ci(text: &str, pat: &str, by: &str) -> String {
    let (t, p) = (text.as_bytes(), pat.as_bytes());
    let mut out = String::with_capacity(text.len());
    let (mut i, mut from) = (0, 0);
    while i + p.len() <= t.len() {
        if t[i..i + p.len()].eq_ignore_ascii_case(p) {
            out.push_str(&text[from..i]);
            out.push_str(by);
            i += p.len();
            from = i;
        } else {
            i += 1;
        }
    }
    out.push_str(&text[from..]);
    out
}

/// `String(x)` of a whole number (a rounded time, a rack's number). −0 says "0", as JavaScript does.
fn js_int(x: f64) -> String { format!("{}", x as i64) }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogue::{Him, StageKey};

    /// FNV-1a over a string's UTF-8, the digest the TypeScript's side was worked out with.
    fn fnv(mut d: u64, s: &str) -> u64 { for b in s.bytes() { d ^= b as u64; d = d.wrapping_mul(0x100000001b3); } d }
    const SEED: u64 = 0xcbf29ce484222325;

    const TS_SCRIPT_LINES: usize = 253;
    const TS_SCRIPT_DIGEST: u64 = 0x37ad7f6c3d9dba9d;
    const TS_ROWS_DIGEST: u64 = 0xf4724108b7433e7f;
    const TS_HANDLER_DIGEST: u64 = 0x9dc9478aef436ee5;

    #[test]
    fn script_lines_are_the_typescripts() {
        let lines = script_sam_lines();
        assert_eq!(lines.len(), TS_SCRIPT_LINES);
        let mut d = SEED;
        for l in &lines {
            d = fnv(d, &l.id); d = fnv(d, "\0"); d = fnv(d, &l.text); d = fnv(d, "\0"); d = fnv(d, l.guide.as_deref().unwrap_or(""));
            d = fnv(d, "\0"); d = fnv(d, l.t.unwrap().as_str()); d = fnv(d, "\n");
            assert!(l.s == crate::dialogue::SamLineS::Script);
        }
        assert_eq!(d, TS_SCRIPT_DIGEST);
    }

    #[test]
    fn beats_rows_are_the_typescripts() {
        let mut d = SEED;
        for sc in crate::dialogue::scripts().iter() {
            d = fnv(d, &sc.id); d = fnv(d, sc.shape.as_str()); d = fnv(d, sc.t.as_str());
            for k in sc.k.iter() { d = fnv(d, k.as_str()); }
            d = fnv(d, "|");
            if let Some(x) = &sc.band { for b in x { d = fnv(d, b.as_str()); } }
            d = fnv(d, "|");
            if let Some(x) = &sc.needs { for b in x { d = fnv(d, b.as_str()); } }
            d = fnv(d, "|");
            if let Some(x) = &sc.not { for b in x { d = fnv(d, b.as_str()); } }
            d = fnv(d, "|");
            if let Some(x) = sc.pin { d = fnv(d, x.as_str()); }
            for s in crate::dialogue::stages() {
                let b = beat_at(&sc.beats, s);
                match &b.him {
                    Him::Text(t) => { d = fnv(d, t); }
                    Him::Arm1(p) => { for (k, v) in p.iter() { d = fnv(d, &k); d = fnv(d, "="); d = fnv(d, v); d = fnv(d, ";"); } }
                }
                d = fnv(d, "\0"); d = fnv(d, b.mild.as_deref().unwrap_or("")); d = fnv(d, if b.room == Some(true) { "R" } else { "-" });
            }
            d = fnv(d, "\n");
        }
        assert_eq!(d, TS_ROWS_DIGEST);
    }

    #[test]
    fn handler_pools_are_the_typescripts() {
        use crate::dialogue::HandlerBeat as Hb;
        let table = crate::dialogue::handler_lines();
        let mut d = SEED;
        for beat in [Hb::Server, Hb::Drive, Hb::Rack, Hb::Alarm, Hb::Lockdown, Hb::Blackout, Hb::BodyKo, Hb::BodyDead] {
            d = fnv(d, beat.as_str());
            for l in table.get(String::from(beat.as_str())).unwrap() {
                for x in [Some(l.h.as_str()), Some(l.s.as_str()), l.h_own.as_deref(), l.s_own.as_deref(), l.h_now.as_deref()] { d = fnv(d, "\0"); d = fnv(d, x.unwrap_or("")); }
            }
        }
        assert_eq!(d, TS_HANDLER_DIGEST);
    }

    #[test]
    fn stages_read_their_own_beat() {
        let mut beats = crate::dialogue::ScriptBeats::zeroed();
        beats._1.sam = String::from("a"); beats._2.sam = String::from("b"); beats._3.sam = String::from("c"); beats.x.sam = String::from("d");
        let said: Vec<String> = crate::dialogue::stages().into_iter().map(|s| format!("{}{}", stage_word(s), beat_at(&beats, s).sam)).collect();
        assert_eq!(said, ["1a", "2b", "3c", "xd"]);
        assert!(matches!(crate::dialogue::stages()[3], StageKey::X));
    }

    #[test]
    fn worst_pair_line_has_the_longest_tokens() {
        assert_eq!(pair_worst("{OTHER} {Code} {padDoor}, {PadDoor} {PADDOOR}"), "Lindqvist 7391 the office side door, The office side door {PADDOOR}");
    }

    #[test]
    fn regex_i_replace_matches_javascript() {
        assert_eq!(replace_ascii_ci("{other} {OTHER} {Other}! {othe} \u{2026}{oThEr}", "{other}", "Lindqvist"), "Lindqvist Lindqvist Lindqvist! {othe} \u{2026}Lindqvist");
        assert_eq!(replace_ascii_ci("{cod", "{code}", "7391"), "{cod");
        assert_eq!(replace_ascii_ci("", "{code}", "7391"), "");
    }

    #[test]
    fn whole_numbers_say_what_string_says() {
        assert_eq!(js_int(-0.0), "0");
        assert_eq!(js_int(12.0), "12");
        assert_eq!(js_int(-3.0), "-3");
        assert_eq!(js_int(1234567890123.0), "1234567890123");
    }
}
