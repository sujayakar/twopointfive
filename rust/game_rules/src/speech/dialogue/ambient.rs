use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// What a new row holds: next is -1, which AmbientState::default() is not.
fn ambient_fresh() -> AmbientState {
    AmbientState { next: -1.0, run: None, played: JsMap::new(), count: 0.0 }
}

/// The game's ambient state, made if none: the row itself.
fn ambient_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::AmbientState {
    g.m.dialogue.ambient_by_game.get_or_insert_with(ambient_fresh)
}

/// Is the floor one the pairs may talk on?
fn ambient_live(g: &crate::game::Game) -> bool {
    if !g.ai_enabled || g.aside { return false; }
    if crate::guards::escalation_of(g) != 0.0 { return false; }
    if g.alarm.episode { return false; }
    if g.light_home.mains.active { return false; }
    let men = crate::seats::men(g);
    !men.iter().any(|&p| g.pl[p].holding().is_some() || crate::condition::down(g, g.pl[p].ch))
}

/// A calm patrolling staff man with nothing on his hands or mind, not talking, not held by a script.
fn chatty(g: &crate::game::Game, x: GuardId) -> bool {
    let time = g.time;
    let m = &g.gd[x];
    let calm = g.ch[m.ch].team == crate::teams::Team::Guard && m.unit.is_none() && m.state == crate::game::GuardState::Patrol
        && m.held.is_none() && m.script.is_none() && m.task.is_none() && m.body_duty.is_none() && !m.pinned && !m.hold
        && m.leader.is_none() && m.post.is_none() && m.awareness < 0.15
        && !m.bubble.as_ref().map_or(false, |b| time - b.t < b.dur + 1.0);
    if !calm { return false; }
    if crate::owner::has(g, x, crate::owner::by::NEEDS) { return false; }
    g.guards.contains(&x)
}

/// Still fit to keep talking (the hold flag is ours by now).
fn still_chatty(g: &crate::game::Game, x: GuardId) -> bool {
    let m = &g.gd[x];
    m.state == crate::game::GuardState::Patrol && m.held.is_none() && m.script.is_none() && m.task.is_none()
        && m.body_duty.is_none() && m.awareness < 0.3 && g.guards.contains(&x)
}

/// A conversation over: the men's holds back as they were, the cooldown from now.
fn end_ambient(g: &mut crate::game::Game) {
    let time = g.time;
    let s = ambient_state(g);
    let Some(r) = s.run.take() else { return; };
    s.next = time + crate::dialogue::ambient().cooldown;
    if !r.held[0] { crate::control::set_hold(g, r.a, false, "dialogue"); }
    if !r.held[1] { crate::control::set_hold(g, r.b, false, "dialogue"); }
}

/// The line of a pair as it will be said: its `own` wording off the office, else its text.
fn pair_said(l: &crate::dialogue::PairTalkLinesItem, own: bool) -> String {
    if own { l.own.as_ref().unwrap_or(&l.text).clone() } else { l.text.clone() }
}

/// The door's name with its first letter up.
fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// A pair's line with its tokens filled.
fn said_text(raw: &str, other: Option<&str>, pad: Option<&AmbientRunPad>) -> String {
    let o = other.unwrap_or("");
    let code = pad.map_or("", |p| p.code.as_str());
    let door = pad.map_or("", |p| p.door.as_str());
    let cap = pad.map_or(String::new(), |p| cap_first(&p.door));
    raw.replace("{Other}", o).replace("{other}", o)
        .replace("{Code}", code).replace("{code}", code)
        .replace("{PadDoor}", &cap).replace("{padDoor}", door)
}

/// /\{other\}/i
fn names_other(line: &str) -> bool { line.to_ascii_lowercase().contains("{other}") }
/// /\{code\}/i
fn names_code(line: &str) -> bool { line.to_ascii_lowercase().contains("{code}") }
/// /\{code\}|\{padDoor\}/i
fn names_pad(line: &str) -> bool { let l = line.to_ascii_lowercase(); l.contains("{code}") || l.contains("{paddoor}") }

/// May this pair's talk be dealt, given a third man, a pad, the wording and the ledger?
fn pair_fits(p: &PairTalk, third: bool, pad: bool, own: bool, played: &JsMap<String, f64>) -> bool {
    (third || !p.lines.iter().any(|l| names_other(&pair_said(l, own))))
        && (pad || !p.lines.iter().any(|l| names_pad(&pair_said(l, own))))
        && !(own && (p.office.unwrap_or(false) || played_of(played, &p.id) >= 1.0))
}

/// S.played.get(id) ?? 0
fn played_of(played: &JsMap<String, f64>, id: &str) -> f64 {
    played.get(String::from(id)).copied().unwrap_or(0.0)
}

/// The one per-frame hook: the handler's radio, then the pairs: advance a conversation under way (or drop it the frame either man stops being a calm patrol), else look for a pair to start one.
pub fn update_ambient(g: &mut crate::game::Game, dt: f64) {
    tick_radio(g);
    let table = crate::dialogue::ambient();
    if let Some(r) = ambient_state(g).run.as_ref() {
        let (a, b) = (r.a, r.b);
        if !ambient_live(g) || !still_chatty(g, a) || !still_chatty(g, b) { end_ambient(g); return; }
        for (me, him) in [(a, b), (b, a)] {
            let (c, h) = (g.gd[me].ch, g.gd[him].ch);
            let want = crate::character::character_want_body(g, c);
            let (cp, hp) = (g.ch[c].pos, g.ch[h].pos);
            g.ch[c].body_yaw = v3::approach_angle(want, m::atan2(hp[0] - cp[0], hp[2] - cp[2]), table.face_rate * dt);
        }
        let time = g.time;
        let Some(r) = ambient_state(g).run.as_mut() else { return; };
        if time < r.at { return; }
        if r.i >= r.talk.lines.len() { end_ambient(g); return; }
        let line = r.talk.lines[r.i].clone();
        r.i += 1;
        let who = if line.who == 0 { r.a } else { r.b };
        let (other, pad) = (r.other.clone(), r.pad.clone());
        let own = !is_office(g);
        let raw = pair_said(&line, own);
        let text = said_text(&raw, other.as_deref(), pad.as_ref());
        crate::game::game_say(g, who, &text, None, None);
        if let Some(pad) = pad.as_ref() {
            if names_code(&raw) { crate::devices::devices_overhear_said(g, who, &pad.lock_id); }
        }
        let at = g.time + crate::guards::bubble_secs(&text) + table.gap;
        if let Some(r) = ambient_state(g).run.as_mut() { r.at = at; }
        return;
    }
    if !ambient_live(g) { ambient_state(g).next = -1.0; return; }
    let time = g.time;
    let s = ambient_state(g);
    if s.next < 0.0 { s.next = time + table.first_after; return; }
    if time < s.next { return; }
    s.next = time + 1.0;
    let mut men: Vec<GuardId> = Vec::new();
    for x in g.guards.clone() { if chatty(g, x) { men.push(x); } }
    for i in 0..men.len() {
        for j in (i + 1)..men.len() {
            let (a, b) = (men[i], men[j]);
            let (pa, pb) = (g.ch[g.gd[a].ch].pos, g.ch[g.gd[b].ch].pos);
            if v3::dist_xz(pa, pb) > table.pair_m { continue; }
            if crate::scene_collision::solid_blocked(g, [pa[0], 1.4, pa[2]], [pb[0], 1.4, pb[2]], None) { continue; }
            let mut third: Option<GuardId> = None;
            for x in g.guards.clone() {
                if x == a || x == b || g.gd[x].unit.is_some() || crate::condition::down(g, g.gd[x].ch) { continue; }
                let callsign = g.gd[x].callsign.clone();
                if !known_down(g, a).has(callsign) { third = Some(x); break; }
            }
            let pad = match crate::devices::devices_pad_near(g, pa) {
                Some(k) => {
                    let written = g.dr[k.door].def.name.clone();
                    Some(AmbientRunPad { code: k.def.code.clone(), door: door_name(&written), lock_id: k.def.name.clone() })
                }
                None => None,
            };
            let own = !is_office(g);
            let (csa, csb) = (g.gd[a].callsign.clone(), g.gd[b].callsign.clone());
            let other = third.map(|t| g.gd[t].callsign.clone());
            let (hold_a, hold_b) = (g.gd[a].hold, g.gd[b].hold);
            let s = ambient_state(g);
            let played = &s.played;
            let mut fits: Vec<PairTalk> = crate::dialogue::ambient_pairs().into_iter().filter(|p| pair_fits(p, third.is_some(), pad.is_some(), own, played)).collect();
            if fits.is_empty() { return; }
            let count = s.count;
            js::sort_by(&mut fits, |p, q| {
                let d = played_of(played, &p.id) - played_of(played, &q.id);
                if js::truthy(d) { return d; }
                crate::dice::hash_str(&format!("{}|{}|{}|{}", csa, csb, count, p.id)) - crate::dice::hash_str(&format!("{}|{}|{}|{}", csa, csb, count, q.id))
            });
            let talk = fits.swap_remove(0);
            let n = played_of(&s.played, &talk.id) + 1.0;
            s.played.set(talk.id.clone(), n);
            s.count += 1.0;
            let a_opens = js::to_i32(crate::dice::hash_str(&format!("{}{}{}", csa, csb, s.count))) & 1 != 0;
            let (opener, other2, held) = if a_opens { (a, b, [hold_a, hold_b]) } else { (b, a, [hold_b, hold_a]) };
            s.run = Some(AmbientRun { a: opener, b: other2, talk, i: 0, at: time + 0.3, other, pad, held });
            crate::control::set_hold(g, opener, true, "dialogue");
            crate::control::set_hold(g, other2, true, "dialogue");
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn talk(id: &str, office: bool, lines: &[(&str, Option<&str>)]) -> PairTalk {
        PairTalk {
            id: String::from(id),
            lines: lines.iter().enumerate().map(|(i, (text, own))| crate::dialogue::PairTalkLinesItem { who: (i % 2) as _, text: String::from(*text), own: own.map(String::from) }).collect(),
            office: Some(office),
        }
    }

    /// A FAILURE MEANS: two men talk of "" because no third man stands to be named or no pad to give away.
    #[test]
    fn a_talk_that_names_a_third_man_or_a_pad_needs_one() {
        let none = JsMap::new();
        let plain = talk("a", false, &[("It clicked at me.", None)]);
        assert!(pair_fits(&plain, false, false, false, &none));
        let other = talk("b", false, &[("{Other} did the sheet again.", None)]);
        assert!(!pair_fits(&other, false, true, false, &none));
        assert!(pair_fits(&other, true, false, false, &none));
        for line in ["{Code} now.", "They redid {padDoor}."] {
            let pad = talk("c", false, &[(line, None)]);
            assert!(!pair_fits(&pad, true, false, false, &none), "{line}");
            assert!(pair_fits(&pad, false, true, false, &none), "{line}");
        }
    }

    /// A FAILURE MEANS: the office's talk is heard on another level, or a level's men tell the same story twice in a night.
    #[test]
    fn on_a_levels_own_words_the_office_talk_is_out_and_a_talk_is_told_once() {
        let mut played = JsMap::new();
        let office = talk("a", true, &[("It clicked at me.", None)]);
        assert!(pair_fits(&office, false, false, false, &played));
        assert!(!pair_fits(&office, false, false, true, &played));
        let mine = talk("b", false, &[("It clicked at me.", None)]);
        assert!(pair_fits(&mine, false, false, true, &played));
        played.set(String::from("b"), 1.0);
        assert!(!pair_fits(&mine, false, false, true, &played));
        // what a line needs is asked of the words it will be said in
        let worded = talk("c", false, &[("Plain.", Some("{Other} again."))]);
        assert!(pair_fits(&worded, false, false, false, &played));
        assert!(!pair_fits(&worded, false, false, true, &played));
    }

    /// A FAILURE MEANS: a player overhears "{Code}", or a door's name in lower case at the head of a line.
    #[test]
    fn a_line_says_the_other_man_the_code_and_the_door() {
        let pad = AmbientRunPad { code: String::from("7391"), door: String::from("the side door"), lock_id: String::from("lock") };
        assert_eq!(said_text("{Other} did the sheet in pencil again.", Some("Novak"), Some(&pad)), "Novak did the sheet in pencil again.");
        assert_eq!(said_text("They redid the pad on {padDoor}. {Code} now.", Some("Novak"), Some(&pad)), "They redid the pad on the side door. 7391 now.");
        assert_eq!(said_text("{PadDoor}.", None, Some(&pad)), "The side door.");
    }
}
