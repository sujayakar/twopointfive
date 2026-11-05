//! src/game/dialogue.ts lines 1767 to 2092
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Every line of the needs talk: the QA soak's pool, radio lines expanded per callsign given. (dialogue.ts needsLines)
pub fn needs_lines(g: &mut crate::game::Game, callsigns: Option<&[String]>) -> Vec<String> {
    needs_lines_of(callsigns.unwrap_or(&[]))
}

fn needs_lines_of(callsigns: &[String]) -> Vec<String> {
    let l = crate::dialogue::needs_lines_table();
    let mut out: Vec<String> = Vec::new();
    for (_, pool) in l.announce.iter() { out.extend(pool.iter().cloned()); }
    for (_, pool) in l.announce_own.iter() { out.extend(pool.iter().cloned()); }
    out.extend(l.now.values().cloned());
    out.extend(l.reply.iter().cloned());
    out.extend(l.back.iter().cloned());
    for o in callsigns {
        for r in &l.reply_other { out.push(r.replace("{other}", o)); }
    }
    for w in callsigns {
        for s in l.short.values().chain(l.short_own.values()) {
            out.push(l.radio.replacen("{who}", w, 1).replacen("{short}", s, 1));
        }
    }
    out
}

/// Every civilian line and every line said about one: the QA soak's pool. (dialogue.ts civilianLines)
pub fn civilian_lines(g: &mut crate::game::Game) -> Vec<String> {
    civilian_lines_all()
}

fn civilian_lines_all() -> Vec<String> {
    let l = crate::dialogue::civ_lines();
    let count = crate::dialogue::civ_count();
    let flat = |t: &[String]| -> Vec<String> { t.iter().flat_map(|t| count.iter().map(move |n| t.replacen("{n}", n, 1))).collect() };
    let mut out: Vec<String> = Vec::new();
    out.extend(l.glimpse.iter().cloned());
    out.extend(l.shrug.iter().cloned());
    for (_, pool) in l.gasp.iter() { out.extend(pool.iter().cloned()); }
    out.extend(l.flee.iter().cloned());
    for (_, pool) in l.report.iter() { out.extend(pool.iter().cloned()); }
    out.extend(l.guard_ack.someone.iter().cloned());
    out.extend(l.guard_ack.armed.iter().cloned());
    out.extend(l.guard_radio.values().cloned());
    out.extend(l.phone.iter().cloned());
    out.extend(l.phone_armed.iter().cloned());
    out.extend(l.control.values().cloned());
    out.extend(l.cower.iter().cloned());
    out.extend(l.held_fear.iter().cloned());
    out.extend(l.held_fear_worker.iter().cloned());
    for (_, pool) in l.intel.iter() { out.extend(pool.iter().cloned()); }
    out.extend(l.dry.iter().cloned());
    out.extend(l.comply.sam.iter().cloned());
    out.extend(l.comply.civ.iter().cloned());
    out.extend(l.sam_ask.iter().cloned());
    out.extend(l.found.iter().cloned());
    out.push(l.found_down.cold.clone());
    out.push(l.found_down.dead.clone());
    out.push(l.outrage.clone());
    out.extend(l.greet.civ.iter().cloned());
    out.extend(l.greet.guard.iter().cloned());
    out.extend(l.settle.iter().cloned());
    out.push(l.qrf.clone());
    out.extend(l.evac.iter().cloned());
    let o = &l.own;
    out.push(o.phone.clone());
    out.push(o.phone_armed.clone());
    out.extend(o.control.values().cloned());
    out.push(o.held_fear.clone());
    out.push(o.intel_pad.clone());
    out.extend(flat(&o.intel));
    out.push(o.sam_ask.clone());
    out.push(o.found_cold.clone());
    out.push(l.now.guard_ack.clone());
    out.push(l.now.qrf.clone());
    // the guide's variants: all worked out against `out` as it stands, then added (a repeat among them stays)
    let g2 = &l.guide;
    let mut variants: Vec<String> = Vec::new();
    for (_, pool) in g2.report.iter() { variants.extend(pool.iter().cloned()); }
    variants.extend(g2.control.values().cloned());
    variants.extend(g2.greet_civ.iter().cloned());
    variants.extend(g2.greet_guard.iter().cloned());
    variants.extend(g2.own.control.values().cloned());
    variants.extend(flat(&g2.own.intel));
    variants.push(g2.own.found_cold.clone());
    variants.retain(|t| !out.contains(t));
    out.extend(variants);
    out
}

/// Every hostage line and the team's word to them. (dialogue.ts hostageLines)
pub fn hostage_lines() -> Vec<String> {
    let l = crate::dialogue::hostage_lines_table();
    let mut out = l.wait;
    out.extend(l.threat);
    out.extend(l.aimed);
    out.extend(l.shot);
    out.extend(l.entry);
    out.extend(l.alone);
    out.extend(l.secured);
    out.extend(l.team);
    out
}

/// Every line of the man a snatch is for; snatch.ts asks it of a line about to be said. (dialogue.ts snatchLines)
pub fn snatch_lines() -> Vec<String> {
    let l = crate::dialogue::snatch_lines_table();
    let mut out = l.running;
    out.extend(l.cornered);
    out.extend(l.hands_up);
    out.extend(l.tied);
    out.extend(l.held);
    out.extend(l.dry);
    out
}

/// What a new row holds: next is -1, which AmbientState::zeroed() is not. (dialogue.ts:2002)
fn ambient_fresh() -> AmbientState {
    AmbientState { next: -1.0, run: None, played: JsMap::new(), count: 0.0 }
}

/// The game's ambient state, made if none: the row itself. (dialogue.ts ambientState)
pub fn ambient_state<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::dialogue::AmbientState {
    g.m.dialogue.ambient_by_game.get_or_insert_with(ambient_fresh)
}

/// Is the floor one the pairs may talk on? (dialogue.ts ambientLive)
pub fn ambient_live(g: &mut crate::game::Game) -> bool {
    if !g.ai_enabled || g.quiet_utility || g.puppet.is_some() || g.touring { return false; }
    if crate::guards::escalation_of(g) != 0.0 { return false; }
    if g.alarm.episode { return false; }
    if crate::sim_lightshome::mains_active(g) { return false; }
    let seats = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    let men = crate::seats::men(g, &seats);
    !men.iter().any(|&p| g.pl[p].holding.is_some() || g.pl[p].down)
}

/// A calm patrolling staff man with nothing on his hands or mind, not talking, not held by a script. (dialogue.ts chatty)
pub fn chatty(g: &mut crate::game::Game, x: GuardId) -> bool {
    let time = g.time;
    let m = &g.gd[x];
    let calm = m.team == crate::teams::Team::Guard && m.unit.is_none() && m.state == crate::game::GuardState::Patrol
        && m.held.is_none() && m.script.is_none() && m.task.is_none() && m.body_duty.is_none() && !m.pinned && !m.hold
        && m.leader.is_none() && m.post.is_none() && m.awareness < 0.15
        && !m.bubble.as_ref().map_or(false, |b| time - b.t < b.dur + 1.0);
    if !calm { return false; }
    if crate::needs::needs_busy(g, x) { return false; }
    g.guards.contains(&x)
}

/// Still fit to keep talking (the hold flag is ours by now). (dialogue.ts stillChatty)
pub fn still_chatty(g: &mut crate::game::Game, x: GuardId) -> bool {
    let m = &g.gd[x];
    m.state == crate::game::GuardState::Patrol && m.held.is_none() && m.script.is_none() && m.task.is_none()
        && m.body_duty.is_none() && m.awareness < 0.3 && g.guards.contains(&x)
}

/// A conversation over: the men's holds back as they were, the cooldown from now. (dialogue.ts endAmbient)
pub fn end_ambient(g: &mut crate::game::Game) {
    let time = g.time;
    let s = ambient_state(g);
    let Some(r) = s.run.take() else { return; };
    s.next = time + crate::dialogue::ambient().cooldown;
    if !r.held[0] { g.gd[r.a].hold = false; }
    if !r.held[1] { g.gd[r.b].hold = false; }
}

/// A restart: the pairs' clock starts over; off the office the ledger is the run's, so it clears. The men's holds stay as they are, as in the TypeScript. (dialogue.ts ambientRestart)
pub fn ambient_restart(g: &mut crate::game::Game) {
    let Some(s) = g.m.dialogue.ambient_by_game.as_mut() else { return; };
    s.run = None;
    s.next = -1.0;
    if !is_office(g) { ambient_state(g).played.clear(); }
}

/// The line of a pair as it will be said: its `own` wording off the office, else its text. (dialogue.ts pairSaid)
pub fn pair_said(l: &crate::dialogue::PairTalkLinesItem, own: bool) -> String {
    if own { l.own.as_ref().unwrap_or(&l.text).clone() } else { l.text.clone() }
}

/// The door's name with its first letter up. (dialogue.ts:2031)
fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// A pair's line with its tokens filled. (dialogue.ts:2031)
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

/// May this pair's talk be dealt, given a third man, a pad, the wording and the ledger? (dialogue.ts:2049)
fn pair_fits(p: &PairTalk, third: bool, pad: bool, own: bool, played: &JsMap<String, f64>) -> bool {
    (third || !p.lines.iter().any(|l| names_other(&pair_said(l, own))))
        && (pad || !p.lines.iter().any(|l| names_pad(&pair_said(l, own))))
        && !(own && (p.office.unwrap_or(false) || played_of(played, &p.id) >= 1.0))
}

/// S.played.get(id) ?? 0
fn played_of(played: &JsMap<String, f64>, id: &str) -> f64 {
    played.get(String::from(id)).copied().unwrap_or(0.0)
}

/// The one per-frame hook: the handler's radio, then the pairs: advance a conversation under way (or drop it the frame either man stops being a calm patrol), else look for a pair to start one. (dialogue.ts updateAmbient)
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
        if r.i >= r.talk.lines.len() as f64 { end_ambient(g); return; }
        let line = r.talk.lines[r.i as usize].clone();
        r.i += 1.0;
        let who = if line.who == 0.0 { r.a } else { r.b };
        let (other, pad) = (r.other.clone(), r.pad.clone());
        let own = !is_office(g);
        let raw = pair_said(&line, own);
        let text = said_text(&raw, other.as_deref(), pad.as_ref());
        crate::game::game_say(g, who, &text, None, None, None);
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
                if x == a || x == b || g.gd[x].unit.is_some() || g.gd[x].state == crate::game::GuardState::Dead { continue; }
                let callsign = g.gd[x].callsign.clone();
                if !known_down(g, a).has(callsign) { third = Some(x); break; }
            }
            let pad = match crate::devices::devices_pad_near(g, pa) {
                Some(k) => {
                    let written = g.dr[k.door].def.name.clone();
                    Some(AmbientRunPad { code: k.def.code.clone(), door: door_name(g, &written), lock_id: k.def.name.clone() })
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
                crate::math_dice::hash_str(&format!("{}|{}|{}|{}", csa, csb, count, p.id)) - crate::math_dice::hash_str(&format!("{}|{}|{}|{}", csa, csb, count, q.id))
            });
            let talk = fits.swap_remove(0);
            let n = played_of(&s.played, &talk.id) + 1.0;
            s.played.set(talk.id.clone(), n);
            s.count += 1.0;
            let a_opens = js::to_i32(crate::math_dice::hash_str(&format!("{}{}{}", csa, csb, s.count))) & 1 != 0;
            let (opener, other2, held) = if a_opens { (a, b, [hold_a, hold_b]) } else { (b, a, [hold_b, hold_a]) };
            s.run = Some(AmbientRun { a: opener, b: other2, talk, i: 0.0, at: time + 0.3, other, pad, held });
            g.gd[opener].hold = true;
            g.gd[other2].hold = true;
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fnv(lines: &[String]) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let mut first = true;
        for l in lines {
            if !first { h ^= b'\n' as u64; h = h.wrapping_mul(0x100000001b3); }
            first = false;
            for b in l.as_bytes() { h ^= *b as u64; h = h.wrapping_mul(0x100000001b3); }
        }
        h
    }

    // each pair is the TypeScript's own: its count and the FNV-1a of its lines joined by \n, under bun with WORDS.on and WORDS.guide on
    #[test]
    fn line_pools_are_the_typescripts() {
        let got = needs_lines_of(&[]);
        assert_eq!((got.len(), fnv(&got)), (33, 0xa3cad49332398a49));
        let got = needs_lines_of(&[String::from("Novak"), String::from("Reyes")]);
        assert_eq!((got.len(), fnv(&got)), (47, 0x60856ff288239cd4));
        let got = civilian_lines_all();
        assert_eq!((got.len(), fnv(&got)), (171, 0xafb2f0666b466e78));
        let got = hostage_lines();
        assert_eq!((got.len(), fnv(&got)), (32, 0x4f3202ed4c2bf9b2));
        let got = snatch_lines();
        assert_eq!((got.len(), fnv(&got)), (16, 0x2939817da6025187));
    }

    #[test]
    fn pairs_are_the_typescripts() {
        let rows: Vec<String> = crate::dialogue::ambient_pairs().iter().map(|p| {
            let lines: Vec<String> = p.lines.iter().map(|l| format!("{}~{}~{}", l.who, l.text, l.own.as_deref().unwrap_or("-"))).collect();
            format!("{}|{}|{}", p.id, if p.office.unwrap_or(false) { 1 } else { 0 }, lines.join("^"))
        }).collect();
        assert_eq!((rows.len(), fnv(&rows)), (9, 0x385b0d49c82d7aaa));
    }

    #[test]
    fn fits_are_the_typescripts() {
        // (third, pad, own, pencil played once, the ids that fit), from the TypeScript's own filter
        let cases: &[(bool, bool, bool, bool, &str)] = &[
            (false, false, false, false, "sensor,thunk,overtime,poster,jug,photos,costcentre"),
            (false, false, true, false, "jug"),
            (false, true, false, false, "sensor,thunk,pin,overtime,poster,jug,photos,costcentre"),
            (false, true, true, false, "pin,jug"),
            (true, false, false, true, "sensor,thunk,pencil,overtime,poster,jug,photos,costcentre"),
            (true, false, true, false, "pencil,jug"),
            (true, false, true, true, "jug"),
            (true, true, false, false, "sensor,thunk,pencil,pin,overtime,poster,jug,photos,costcentre"),
            (true, true, true, false, "pencil,pin,jug"),
            (true, true, true, true, "pin,jug"),
        ];
        for &(third, pad, own, pencil, want) in cases {
            let mut played = JsMap::new();
            if pencil { played.set(String::from("pencil"), 1.0); }
            let ids: Vec<String> = crate::dialogue::ambient_pairs().into_iter().filter(|p| pair_fits(p, third, pad, own, &played)).map(|p| p.id).collect();
            assert_eq!(ids.join(","), want);
        }
    }

    #[test]
    fn tokens_are_the_typescripts() {
        let pad = AmbientRunPad { code: String::from("7391"), door: String::from("the side door"), lock_id: String::from("lock") };
        assert_eq!(said_text("{Other} did the sheet in pencil again.", Some("Novak"), Some(&pad)), "Novak did the sheet in pencil again.");
        assert_eq!(said_text("{Other} did the sheet in pencil again.", None, None), " did the sheet in pencil again.");
        assert_eq!(said_text("They redid the pad on {padDoor}. {Code} now.", Some("Novak"), Some(&pad)), "They redid the pad on the side door. 7391 now.");
        assert_eq!(said_text("They redid the pad on {padDoor}. {Code} now.", None, None), "They redid the pad on .  now.");
        assert_eq!(said_text("{PadDoor}.", None, Some(&pad)), "The side door.");
    }
}
