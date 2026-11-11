#![allow(unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{format, string::String, vec, vec::Vec};

/// The context's tokens as the table `expand` reads, keyed by each token's word (`sawWhere`), an absent token None.
pub(super) fn ctx_tokens_map(t: &crate::dialogue::DialogueCtxTokens) -> JsMap<String, Option<String>> {
    let mut o = JsMap::new();
    let rows: [(&str, &Option<String>); 23] = [("partner", &t.partner), ("victim", &t.victim), ("body", &t.body), ("missing", &t.missing), ("room", &t.room), ("sawWhere", &t.saw_where), ("heardWhere", &t.heard_where), ("route", &t.route), ("partnerRoute", &t.partner_route), ("post", &t.post), ("door", &t.door), ("caller", &t.caller), ("checker", &t.checker), ("checked", &t.checked), ("clearRoom", &t.clear_room), ("breakerRoom", &t.breaker_room), ("checkEta", &t.check_eta), ("minutes", &t.minutes), ("n", &t.n), ("up", &t.up), ("badge", &t.badge), ("code", &t.code), ("padDoor", &t.pad_door)];
    for (k, v) in rows { o.set(String::from(k), v.clone()); }
    o
}

// ---------------------------------------------------------------- axes

/// Agitation to its band.
pub(super) fn band_of(a: f64) -> crate::dialogue::Band {
    if a >= BAND_HIGH { Band::High } else if a >= BAND_MID { Band::Mid } else { Band::Low }
}

/// What this man knows of Sam, the highest level that holds, from his own record only.
pub(super) fn knowledge_of(g: &crate::game::Game, gd: GuardId) -> crate::dialogue::Knowledge {
    if g.gd[gd].witness.saw_act.is_some() || g.gd[gd].witness.saw_held.is_some() { return Knowledge::SawHurt; }
    if g.gd[gd].last_seen_t >= 0.0 || g.gd[gd].witness.alerted_by == Some(crate::game::WitnessAlertedBy::Sight) || {
        let weapon = g.gd[gd].weapon;
        crate::weapons::firearm_spent(g, weapon) > 0.0
    } { return Knowledge::Saw; }
    if g.gd[gd].sight_t >= 0.0 { return Knowledge::Glimpsed; }
    let w = &g.gd[gd].witness;
    let h = &w.heard;
    if h.shots + h.kick + h.bang + h.small > 0.0
        || w.alerted_by == Some(crate::game::WitnessAlertedBy::Sound)
        || w.alerted_by == Some(crate::game::WitnessAlertedBy::Radio)
        || w.alerted_by == Some(crate::game::WitnessAlertedBy::Body)
        || w.saw_body.is_some()
        || w.called_to_body.is_some()
        || w.dazzled > 0.0
    { return Knowledge::Heard; }
    Knowledge::None
}

/// How rattled this man is, 0 to 1.
pub fn agitation_of(g: &crate::game::Game, gd: GuardId) -> f64 { agitation_terms(g, gd).a }

/// One term of the sum, and its label for the panel.
fn add(a: &mut f64, why: &mut Vec<String>, v: f64, label: &str) {
    if m::abs(v) < 1e-6 { return; }
    *a += v;
    if m::abs(v) >= 0.005 { why.push(format!("{} {}{}", label, if v > 0.0 { "+" } else { "" }, js::to_fixed(v, 2))); }
}

/// The sum of what has rattled this man, clamped, with its terms for the panel and the probes.
pub(super) fn agitation_terms(g: &crate::game::Game, gd: GuardId) -> crate::dialogue::AgitationTerms {
    use crate::game::{GuardState, Temperament};
    let t = temperament_of(g, gd);
    let talk = g.gd[gd].talk;
    let mut why: Vec<String> = Vec::new();
    let mut a = 0.0;
    let esc = crate::guards::escalation_of(g);
    add(&mut a, &mut why, 0.22 * esc, "floor");
    if g.alarm.episode { add(&mut a, &mut why, 0.15, "episode"); }
    let state = g.gd[gd].state;
    let alert_t = g.gd[gd].witness.alert_t;
    if state == GuardState::Alert || (alert_t >= 0.0 && g.time - alert_t < 60.0) {
        add(&mut a, &mut why, 0.2, "ownAlert");
    } else if state == GuardState::Suspicious || state == GuardState::Search {
        add(&mut a, &mut why, 0.1, "ownEdge");
    }
    if g.gd[gd].witness.saw_act.is_some() { add(&mut a, &mut why, 0.3, "sawAct"); }
    if g.gd[gd].witness.saw_body.is_some() {
        add(&mut a, &mut why, 0.2, "sawBody");
    } else if g.gd[gd].witness.called_to_body.is_some() {
        add(&mut a, &mut why, 0.12, "calledToBody");
    }
    if g.gd[gd].witness.heard.shots > 0.0 { add(&mut a, &mut why, 0.15, "shots"); }
    if g.gd[gd].witness.heard.kick + g.gd[gd].witness.heard.bang > 0.0 { add(&mut a, &mut why, 0.1, "bangs"); }
    if g.gd[gd].witness.dazzled > 0.0 { add(&mut a, &mut why, 0.15, "dazzled"); }
    if missing_known_to(g, gd).is_some() { add(&mut a, &mut why, 0.1, "missing"); }
    if g.light_home.mains.active { add(&mut a, &mut why, 0.08, "blackout"); }
    let hs = held_since_of(g, gd);
    let held = if hs >= 0.0 { m::max(0.0, g.time - hs) } else { 0.0 };
    if held > 0.0 {
        add(&mut a, &mut why, if t == Temperament::Nervy { m::min(0.15, 0.004 * held) } else { -m::min(0.12, 0.003 * held) }, "held");
    }
    if matches!(&g.gd[gd].held, Some(h) if h.variant == crate::character::HoldVariant::Gun) { add(&mut a, &mut why, 0.1, "gunToHead"); }
    let abort_t = g.tab.game_talk[talk].abort_t;
    if abort_t >= 0.0 && (g.gd[gd].held.is_some() || g.time - abort_t < ABORT_RATTLE_SECS) { add(&mut a, &mut why, 0.2, "aborted"); }
    let presses = g.tab.game_talk[talk].presses;
    if presses > 3.0 { add(&mut a, &mut why, m::min(0.15, 0.03 * (presses - 3.0)), "pressed"); }
    add(&mut a, &mut why, if t == Temperament::Nervy { 0.15 } else if t == Temperament::Hard { -0.1 } else { 0.0 }, "bias");
    let shaken = crate::morale::shaken_of(g, gd);
    let dread = m::max(0.0, shaken - m::max(0.0, a));
    add(&mut a, &mut why, dread, "dread");
    AgitationTerms { a: m::max(0.0, m::min(1.0, a)) }
}

/// By callsign; a spawned unit, whatever it is called, is a fighting man: hard.
pub fn temperament_of(g: &crate::game::Game, gd: GuardId) -> crate::game::Temperament {
    if g.gd[gd].unit.is_some() { return crate::game::Temperament::Hard; }
    let callsign = g.gd[gd].callsign.clone();
    crate::game::temperament().iter().find(|(k, _)| *k == callsign.as_str()).map(|(_, t)| *t).unwrap_or(crate::game::Temperament::Steady)
}

/// The stages at which this temperament gives the useful thing.
pub(super) fn gives_at(t: crate::game::Temperament, band: crate::dialogue::Band) -> Vec<crate::dialogue::StageKey> {
    match t {
        crate::game::Temperament::Nervy => vec![StageKey::Num(2.0), StageKey::Num(3.0)],
        crate::game::Temperament::Hard => if band == Band::Low { vec![] } else { vec![StageKey::Num(3.0)] },
        _ => vec![StageKey::Num(3.0)],
    }
}

// ---------------------------------------------------------------- tokens, the line table

/// A string that counts as said: present and not empty.
fn said(s: &Option<String>) -> Option<String> { s.as_ref().filter(|x| !x.is_empty()).cloned() }

/// The words of a row as they are said to this man and by him.
pub(super) fn said_text(l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String { said_of(l, ctx) }

fn said_of(l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String {
    if ctx.nameless == Some(true) { if let Some(x) = said(&l.bare) { return x; } }
    if ctx.facts.office != Some(true) { if let Some(x) = said(&l.away) { return x; } }
    l.text.clone()
}

// ---------------------------------------------------------------- intel

/// What each kind of fact is given in, by who gives it: anybody ("*"), a temper ("hard"), a temper and what he knows ("nervy:sawHurt"), a voice ("fan"),
/// a voice and a temper ("selfown:nervy"), or one script ("sc:selfown-s1").
static INTEL: &[(IntelKind, &str, &str)] = &[
    (IntelKind::CheckOn, "nervy", "{Checker}'s out looking for {checked}— he went quiet— he'll walk right into us—"),
    (IntelKind::CheckOn, "hard", "{Checker}'s walking {checked}'s route. This way. Soon."),
    (IntelKind::CheckOn, "*", "{Checker}'s walking {checked}'s route — he went quiet. He'll come past here."),
    (IntelKind::Clearing, "hard", "Rooms are being cleared. {ClearRoom} first. Then this one."),
    (IntelKind::Clearing, "*", "They're clearing rooms off your last position. {ClearRoom} first."),
    (IntelKind::RollcallLive, "hard", "That's the check. If I don't answer, {caller} sends somebody here."),
    (IntelKind::RollcallLive, "*", "That's a check now. If one of us doesn't answer, {caller} sends someone over."),
    (IntelKind::Blackout, "hard", "If the lights go out, somebody goes to the breaker in {breakerRoom}. Armed."),
    (IntelKind::Blackout, "*", "Lights die, one of us goes to the breaker in {breakerRoom} — armed."),
    (IntelKind::Door, "hard", "{Door} is keyed. We all carry one. Take mine and get out."),
    (IntelKind::Door, "*", "{Door} is on a key. We all carry one. Mine's on my belt."),
    (IntelKind::Door, "fan", "{Door}'s keyed— key's on my belt. Do yours open anything? No?"),
    (IntelKind::Door, "con", "{Door}'s keyed; every belt carries one. Here— inspect mine."),
    (IntelKind::Door, "selfown:steady", "…Belt. The key's on my belt. Well played, I suppose."),
    (IntelKind::Door, "selfown:nervy", "…belt. It's on my belt. Please don't make it a whole thing."),
    (IntelKind::Door, "sc:selfown-s1", "…On my belt. The key's on my belt. Eleven years, for this."),
    (IntelKind::Door, "sc:selfown-s2", "…Belt. Left side. Take it— you were going to anyway."),
    (IntelKind::Door, "sc:selfown-s3", "…Belt clip. Under the hi-vis. Nicely done, I suppose."),
    (IntelKind::Door, "sc:selfown-n2", "…This one. On my belt. I heard it as I said it. I HEARD it."),
    (IntelKind::Door, "sc:selfown-n3", "…key. Belt, left side. I don't feel better. You said I would."),
    (IntelKind::Code, "nervy", "The pad by {padDoor}— it's {code}— {code}— don't make me say it twice—"),
    (IntelKind::Code, "hard", "{PadDoor}: {code} on the pad. Now you know what I know."),
    (IntelKind::Code, "*", "{PadDoor} is on a pad — {code}. Same four digits every night."),
    (IntelKind::Code, "fan", "The pad on {padDoor} is {code}— do YOU get codes, or is it all wire?"),
    (IntelKind::Code, "con", "{PadDoor} takes {code} on the pad, sir. Logged monthly, changed never."),
    (IntelKind::Rollcall, "nervy", "A check— {checkEta}— I don't answer, somebody comes looking."),
    (IntelKind::Rollcall, "hard", "Radio check {checkEta}. No answer, they send somebody. Do the math."),
    (IntelKind::Rollcall, "*", "Radio check {checkEta}. Miss it, they send a man down your route."),
    (IntelKind::Rollcall, "fan", "Radio check's {checkEta}— miss it, they send someone. Do YOU have checks?"),
    (IntelKind::Rollcall, "con", "Radio check {checkEta}, sir. No answer, a man walks that route. By the book."),
    (IntelKind::Rollcall, "nervy:sawHurt", "The check's {checkEta}— {victim} won't answer it— then they ALL come—"),
    (IntelKind::LockdownPosts, "hard", "Pairs. A man on {post}. Nobody alone — except you."),
    (IntelKind::LockdownPosts, "*", "We're paired now. One man holds {post}. Nobody walks alone."),
    (IntelKind::Routes, "nervy", "It's {n} of us— me on {route}, {partner} on {partnerRoute}— that's it—"),
    (IntelKind::Routes, "hard", "{N}. Me, {route}. {Partner}, {partnerRoute}. Happy?"),
    (IntelKind::Routes, "*", "{N} of us tonight. I've got {route}, {partner} has {partnerRoute}."),
    (IntelKind::Routes, "fan", "{N} of us— I do {route}, {partner} {partnerRoute}. What's YOUR route like?"),
    (IntelKind::Routes, "con", "{N} on shift: I take {route}, {partner} {partnerRoute}. All logged, sir."),
    (IntelKind::Missing, "hard", "{Missing} is off the air. His route comes up empty, we lock down."),
    (IntelKind::Missing, "*", "Nobody can raise {missing}. His route turns up empty, this all locks down."),
    (IntelKind::Drive, "hard", "Your rack went dark on the board. Somebody's on his way to it."),
    (IntelKind::Drive, "*", "Your rack dropped off the board. Somebody's already been sent to look at it."),
];

/// The words written for `kind` as `by` gives it.
fn written(kind: IntelKind, by: &str) -> Option<&'static str> {
    INTEL.iter().find(|(k, b, _)| *k == kind && *b == by).map(|(_, _, text)| *text)
}

/// The text a man of this temperament and knowledge gives for `kind`, the most specific variant written.
pub(super) fn intel_text(kind: IntelKind, t: crate::game::Temperament, k: Knowledge) -> String {
    let (t, k) = (t.as_str(), k.as_str());
    let of = |by: &str| written(kind, by);
    String::from(of(&format!("{t}:{k}")).or_else(|| of(t)).or_else(|| of(&format!("*:{k}"))).or_else(|| of("*")).unwrap_or(""))
}

/// The script's own or the shape-voiced variant of `kind`, or None when none is written.
pub(super) fn shaped_intel_text(kind: IntelKind, sc: &crate::dialogue::ShapedIntelTextSc, t: crate::game::Temperament) -> Option<String> {
    let shape = sc.shape.as_str();
    let of = |by: &str| written(kind, by);
    of(&format!("sc:{}", sc.id)).or_else(|| of(&format!("{shape}:{}", t.as_str()))).or_else(|| of(shape)).map(String::from)
}

/// The checker's own words when he is the man walking the silent colleague's route.
pub(super) fn check_on_self() -> String {
    String::from(crate::dialogue::CHECKON_SELF_NOW)
}

/// What goes in front of a given fact.
pub(super) fn intel_lead(t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String {
    match t {
        crate::game::Temperament::Hard => String::from("…Fine. "),
        crate::game::Temperament::Steady if k == Knowledge::Saw || k == Knowledge::SawHurt => String::from("…All right. "),
        _ => String::new(),
    }
}

/// A fact behind a lead that runs on into it starts in lower case, unless it starts with 'I' or a man's name. By hand, the regexes of TS 366:
/// `/[a-z—] $/` on the lead, `/^(?!I\b|\{(?:Checker|Checked|Missing|Partner|Victim|Caller|Body)\})(\{?)([A-Z])/` on the fact.
fn run_on_text(lead: &str, fact: &str) -> String {
    let mut back = lead.chars().rev();
    let runs = back.next() == Some(' ') && matches!(back.next(), Some('a'..='z') | Some('—'));
    if !runs { return String::from(fact); }
    if let Some(after) = fact.strip_prefix('I') {
        // \b after the I: JavaScript's \w is ASCII only, with no `u` flag.
        if !matches!(after.chars().next(), Some(c) if c.is_ascii_alphanumeric() || c == '_') { return String::from(fact); }
    }
    for name in ["{Checker}", "{Checked}", "{Missing}", "{Partner}", "{Victim}", "{Caller}", "{Body}"] {
        if fact.starts_with(name) { return String::from(fact); }
    }
    let (brace, rest) = match fact.strip_prefix('{') { Some(r) => ("{", r), None => ("", fact) };
    match rest.chars().next() {
        Some(c) if c.is_ascii_uppercase() => format!("{}{}{}", brace, c.to_ascii_lowercase(), &rest[1..]),
        _ => String::from(fact),
    }
}

pub(super) fn run_on(lead: &str, fact: &str) -> String { run_on_text(lead, fact) }

/// Four digits off a callsign, the same every night for the same man.
pub(super) fn badge_of(callsign: &str) -> String {
    let n = 1000.0 + js::rem(crate::dice::hash_str(&format!("badge:{}", callsign)), 9000.0);
    format!("{}", n as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Temperament;

    const TS: [Temperament; 3] = [Temperament::Steady, Temperament::Nervy, Temperament::Hard];
    const KS: [Knowledge; 5] = [Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw, Knowledge::SawHurt];

    fn script(id: &str, shape: ShapeKey) -> crate::dialogue::ShapedIntelTextSc { crate::dialogue::ShapedIntelTextSc { id: String::from(id), shape } }

    /// A FAILURE MEANS: a man gives a fact in silence: some temper or knowledge has no words for it.
    #[test]
    fn every_man_has_words_for_every_fact() {
        for kind in crate::dialogue::intel_kinds() {
            for t in TS { for k in KS { assert!(!intel_text(kind, t, k).is_empty(), "{} {} {}", kind.as_str(), t.as_str(), k.as_str()); } }
        }
    }

    /// A FAILURE MEANS: words are written for a man nobody is, so he talks like everybody else.
    #[test]
    fn every_row_is_written_for_somebody() {
        let sh = [ShapeKey::Pedant, ShapeKey::Selfown, ShapeKey::Folder, ShapeKey::Ramble, ShapeKey::Fan, ShapeKey::Stoic, ShapeKey::Con, ShapeKey::Game, ShapeKey::Menace];
        let scripts = crate::dialogue::scripts();
        for (kind, by, _) in INTEL {
            let known = *by == "*"
                || TS.iter().any(|t| *by == t.as_str() || KS.iter().any(|k| *by == format!("{}:{}", t.as_str(), k.as_str())))
                || KS.iter().any(|k| *by == format!("*:{}", k.as_str()))
                || sh.iter().any(|s| *by == s.as_str() || TS.iter().any(|t| *by == format!("{}:{}", s.as_str(), t.as_str())))
                || scripts.iter().any(|s| *by == format!("sc:{}", s.id));
            assert!(known, "{} {by}", kind.as_str());
        }
    }

    /// A FAILURE MEANS: a nervy man who saw it talks like a man who knows nothing: the most specific words written are the ones said.
    #[test]
    fn a_fact_is_said_in_the_most_specific_words_written_for_the_man() {
        for (kind, by, text) in INTEL {
            for t in TS { for k in KS {
                let own = written(*kind, &format!("{}:{}", t.as_str(), k.as_str()));
                if own == Some(*text) || own.is_none() && *by == t.as_str() { assert_eq!(intel_text(*kind, t, k), *text); }
            } }
        }
    }

    /// A FAILURE MEANS: a script's man says the common line where his own was written, or a voice's line for another temper.
    #[test]
    fn a_shaped_fact_takes_the_scripts_own_then_its_temper_then_its_shape() {
        let door = |id: &str, sh| shaped_intel_text(IntelKind::Door, &script(id, sh), Temperament::Steady);
        assert_eq!(door("selfown-s1", ShapeKey::Selfown).as_deref(), written(IntelKind::Door, "sc:selfown-s1"));
        assert_eq!(door("nope", ShapeKey::Selfown).as_deref(), written(IntelKind::Door, "selfown:steady"));
        assert!(written(IntelKind::Door, "sc:selfown-s1").is_some() && written(IntelKind::Door, "selfown:steady").is_some());
        for (kind, _, text) in INTEL.iter().filter(|(_, by, _)| *by == "fan") {
            for t in TS { assert_eq!(shaped_intel_text(*kind, &script("nope", ShapeKey::Fan), t).as_deref(), Some(*text)); }
        }
        assert_eq!(shaped_intel_text(IntelKind::Door, &script("nope", ShapeKey::Game), Temperament::Steady), None);
    }

    /// A FAILURE MEANS: "Oh— and Radio check", or "oh— and {checker}'s walking" with a man's name in lower case, or "…Fine. radio check" behind a lead that ends its sentence.
    #[test]
    fn a_fact_behind_a_lead_that_runs_on_starts_low_but_for_i_and_a_mans_name() {
        for (fact, want) in [("Radio check", "radio check"), ("It's", "it's"), ("{N} of us", "{n} of us"), ("{Door} is", "{door} is"), ("{Bodyguard} y", "{bodyguard} y"), ("I was sent", "I was sent"), ("I'm out", "I'm out"), ("{Checker}'s walking", "{Checker}'s walking"), ("{Body} x", "{Body} x"), ("…belt", "…belt")] {
            assert_eq!(run_on_text("Oh— and ", fact), want);
        }
        assert_eq!(run_on_text("—okay— ", "Radio check"), "radio check");
        for lead in ["…Fine. ", "Sir: ", ""] { assert_eq!(run_on_text(lead, "Radio check"), "Radio check", "{:?}", lead); }
    }

    /// A FAILURE MEANS: a nameless man speaks as a named one, or the office's words are heard away from the office.
    #[test]
    fn said_text_takes_the_first_that_applies() {
        let l = SaidTextL { text: String::from("t"), away: Some(String::from("a")), bare: Some(String::from("b")) };
        let mut ctx = DialogueCtx::default();
        assert_eq!(said_of(&l, &ctx), "a");
        ctx.facts.office = Some(false);
        assert_eq!(said_of(&l, &ctx), "a");
        ctx.facts.office = Some(true);
        assert_eq!(said_of(&l, &ctx), "t");
        ctx.nameless = Some(true);
        assert_eq!(said_of(&l, &ctx), "b");
        let l2 = SaidTextL { text: String::from("t"), away: Some(String::from("")), bare: Some(String::from("")) };
        ctx.facts.office = None;
        assert_eq!(said_of(&l2, &ctx), "t");
    }

    /// A FAILURE MEANS: the panel lists what hardly moved a man, or something that moved him goes unlisted.
    #[test]
    fn agitation_add_drops_the_negligible_and_labels_what_shows() {
        let (mut a, mut why) = (0.0, Vec::new());
        add(&mut a, &mut why, 5e-7, "tiny");
        assert_eq!(a, 0.0);
        add(&mut a, &mut why, 0.004, "small");
        assert_eq!(a, 0.004);
        assert!(why.is_empty());
        add(&mut a, &mut why, 0.22, "floor");
        add(&mut a, &mut why, -0.12, "held");
        assert_eq!(why, vec![String::from("floor +0.22"), String::from("held -0.12")]);
    }
}
