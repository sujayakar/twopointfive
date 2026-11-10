#![allow(unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{format, string::String, vec, vec::Vec};

/// INVENTED: DialogueCtx.tokens as the table `expand` reads, keyed by the words of TOKENS (`sawWhere`, as Token::as_str), a null token None. The TypeScript hands the one object to expand; Rust has a struct there and a map here.
pub(super) fn ctx_tokens_map(t: &crate::dialogue::DialogueCtxTokens) -> JsMap<String, Option<String>> {
    let mut o = JsMap::new();
    let rows: [(&str, &Option<String>); 23] = [("partner", &t.partner), ("victim", &t.victim), ("body", &t.body), ("missing", &t.missing), ("room", &t.room), ("sawWhere", &t.saw_where), ("heardWhere", &t.heard_where), ("route", &t.route), ("partnerRoute", &t.partner_route), ("post", &t.post), ("door", &t.door), ("caller", &t.caller), ("checker", &t.checker), ("checked", &t.checked), ("clearRoom", &t.clear_room), ("breakerRoom", &t.breaker_room), ("checkEta", &t.check_eta), ("minutes", &t.minutes), ("n", &t.n), ("up", &t.up), ("badge", &t.badge), ("code", &t.code), ("padDoor", &t.pad_door)];
    for (k, v) in rows { o.set(String::from(k), v.clone()); }
    o
}

fn st(s: &str) -> String { String::from(s) }

// ---------------------------------------------------------------- axes

/// Agitation to its band.
pub(super) fn band_of(a: f64) -> crate::dialogue::Band {
    if a >= BAND_HIGH { Band::High } else if a >= BAND_MID { Band::Mid } else { Band::Low }
}

/// What this man knows of Sam, the highest level that holds, from his own record only.
pub(super) fn knowledge_of(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::Knowledge {
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
pub fn agitation_of(g: &mut crate::game::Game, gd: GuardId) -> f64 { agitation_terms(g, gd).a }

/// One term of the sum, and its label for the panel.
fn add(a: &mut f64, why: &mut Vec<String>, v: f64, label: &str) {
    if m::abs(v) < 1e-6 { return; }
    *a += v;
    if m::abs(v) >= 0.005 { why.push(format!("{} {}{}", label, if v > 0.0 { "+" } else { "" }, js::to_fixed(v, 2))); }
}

/// The sum of what has rattled this man, clamped, with its terms for the panel and the probes.
pub(super) fn agitation_terms(g: &mut crate::game::Game, gd: GuardId) -> crate::dialogue::AgitationTerms {
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

/// A string the TypeScript's `||` would take: present and not empty.
fn said(s: &Option<String>) -> Option<String> { s.as_ref().filter(|x| !x.is_empty()).cloned() }

/// The words of a row as they are said to this man and by him.
pub(super) fn said_text(g: &crate::game::Game, l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String { said_of(l, ctx) }

fn said_of(l: &crate::dialogue::SaidTextL, ctx: &crate::dialogue::DialogueCtx) -> String {
    if ctx.nameless == Some(true) { if let Some(x) = said(&l.bare) { return x; } }
    if ctx.facts.office != Some(true) { if let Some(x) = said(&l.away) { return x; } }
    if let Some(x) = said(&l.guide) { return x; }
    if let Some(x) = said(&l.now) { return x; }
    l.text.clone()
}

// ---------------------------------------------------------------- intel

/// Calls a macro with every field a kind's row can hold, but the script's own (`sc`).
macro_rules! intel_fields {
    ($m:ident; $($a:tt)*) => { $m!($($a)*; steady nervy hard under pedant selfown folder ramble fan stoic con game menace steady_none steady_heard steady_glimpsed steady_saw steady_saw_hurt nervy_none nervy_heard nervy_glimpsed nervy_saw nervy_saw_hurt hard_none hard_heard hard_glimpsed hard_saw hard_saw_hurt __none __heard __glimpsed __saw __saw_hurt pedant_steady pedant_nervy pedant_hard selfown_steady selfown_nervy selfown_hard folder_steady folder_nervy folder_hard ramble_steady ramble_nervy ramble_hard fan_steady fan_nervy fan_hard stoic_steady stoic_nervy stoic_hard con_steady con_nervy con_hard game_steady game_nervy game_hard menace_steady menace_nervy menace_hard) };
}

/// A row of INTEL_TEXT as the struct of a kind's table.
macro_rules! text_row {
    ($row:ident; $($f:ident)*) => { crate::dialogue::IntelTable { $($f: $row.$f,)* sc: $row.sc } };
}

/// A row of INTEL_NOW or INTEL_GUIDE laid over: what it writes wins, as a spread.
macro_rules! lay_over {
    ($t:ident, $row:ident; $($f:ident)*) => { $(if $row.$f.is_some() { $t.$f = $row.$f; })* };
}

/// A kind's texts as the page has them: INTEL_TEXT with INTEL_NOW and INTEL_GUIDE laid over.
fn table_of(kind: IntelKind) -> crate::dialogue::IntelTable {
    let row = crate::dialogue::intel_text_table().get(st(kind.as_str())).cloned().unwrap_or_default();
    let mut t = intel_fields!(text_row; row);
    let (now, guide) = (crate::dialogue::intel_now(), crate::dialogue::intel_guide());
    macro_rules! over {
        ($row:expr) => { if let Some(r) = $row { intel_fields!(lay_over; t, r); } };
    }
    match kind {
        IntelKind::CheckOn => { over!(now.check_on); over!(guide.check_on); }
        IntelKind::Clearing => { over!(now.clearing); over!(guide.clearing); }
        IntelKind::RollcallLive => { over!(now.rollcall_live); over!(guide.rollcall_live); }
        IntelKind::Blackout => { over!(now.blackout); over!(guide.blackout); }
        IntelKind::Door => { over!(now.door); over!(guide.door); }
        IntelKind::Code => { over!(now.code); over!(guide.code); }
        IntelKind::Rollcall => { over!(now.rollcall); over!(guide.rollcall); }
        IntelKind::LockdownPosts => { over!(now.lockdown_posts); over!(guide.lockdown_posts); }
        IntelKind::Routes => { over!(now.routes); over!(guide.routes); }
        IntelKind::Missing => { over!(now.missing); over!(guide.missing); }
        IntelKind::Drive => { over!(now.drive); over!(guide.drive); }
    }
    t
}

/// T[`${t}:${k}`]
fn of_t_k(x: &crate::dialogue::IntelTable, t: crate::game::Temperament, k: Knowledge) -> Option<&String> {
    use crate::game::Temperament as T;
    match (t, k) {
        (T::Steady, Knowledge::None) => x.steady_none.as_ref(),
        (T::Steady, Knowledge::Heard) => x.steady_heard.as_ref(),
        (T::Steady, Knowledge::Glimpsed) => x.steady_glimpsed.as_ref(),
        (T::Steady, Knowledge::Saw) => x.steady_saw.as_ref(),
        (T::Steady, Knowledge::SawHurt) => x.steady_saw_hurt.as_ref(),
        (T::Nervy, Knowledge::None) => x.nervy_none.as_ref(),
        (T::Nervy, Knowledge::Heard) => x.nervy_heard.as_ref(),
        (T::Nervy, Knowledge::Glimpsed) => x.nervy_glimpsed.as_ref(),
        (T::Nervy, Knowledge::Saw) => x.nervy_saw.as_ref(),
        (T::Nervy, Knowledge::SawHurt) => x.nervy_saw_hurt.as_ref(),
        (T::Hard, Knowledge::None) => x.hard_none.as_ref(),
        (T::Hard, Knowledge::Heard) => x.hard_heard.as_ref(),
        (T::Hard, Knowledge::Glimpsed) => x.hard_glimpsed.as_ref(),
        (T::Hard, Knowledge::Saw) => x.hard_saw.as_ref(),
        (T::Hard, Knowledge::SawHurt) => x.hard_saw_hurt.as_ref(),
    }
}

/// T[t]
fn of_t(x: &crate::dialogue::IntelTable, t: crate::game::Temperament) -> Option<&String> {
    use crate::game::Temperament as T;
    match t { T::Steady => x.steady.as_ref(), T::Nervy => x.nervy.as_ref(), T::Hard => x.hard.as_ref() }
}

/// T[`*:${k}`]
fn of_k(x: &crate::dialogue::IntelTable, k: Knowledge) -> Option<&String> {
    match k {
        Knowledge::None => x.__none.as_ref(),
        Knowledge::Heard => x.__heard.as_ref(),
        Knowledge::Glimpsed => x.__glimpsed.as_ref(),
        Knowledge::Saw => x.__saw.as_ref(),
        Knowledge::SawHurt => x.__saw_hurt.as_ref(),
    }
}

/// T[shape]
fn of_shape(x: &crate::dialogue::IntelTable, shape: ShapeKey) -> Option<&String> {
    match shape {
        ShapeKey::Pedant => x.pedant.as_ref(),
        ShapeKey::Selfown => x.selfown.as_ref(),
        ShapeKey::Folder => x.folder.as_ref(),
        ShapeKey::Ramble => x.ramble.as_ref(),
        ShapeKey::Fan => x.fan.as_ref(),
        ShapeKey::Stoic => x.stoic.as_ref(),
        ShapeKey::Con => x.con.as_ref(),
        ShapeKey::Game => x.game.as_ref(),
        ShapeKey::Menace => x.menace.as_ref(),
    }
}

/// T[`${shape}:${t}`]
fn of_shape_t(x: &crate::dialogue::IntelTable, shape: ShapeKey, t: crate::game::Temperament) -> Option<&String> {
    use crate::game::Temperament as T;
    match (shape, t) {
        (ShapeKey::Pedant, T::Steady) => x.pedant_steady.as_ref(),
        (ShapeKey::Pedant, T::Nervy) => x.pedant_nervy.as_ref(),
        (ShapeKey::Pedant, T::Hard) => x.pedant_hard.as_ref(),
        (ShapeKey::Selfown, T::Steady) => x.selfown_steady.as_ref(),
        (ShapeKey::Selfown, T::Nervy) => x.selfown_nervy.as_ref(),
        (ShapeKey::Selfown, T::Hard) => x.selfown_hard.as_ref(),
        (ShapeKey::Folder, T::Steady) => x.folder_steady.as_ref(),
        (ShapeKey::Folder, T::Nervy) => x.folder_nervy.as_ref(),
        (ShapeKey::Folder, T::Hard) => x.folder_hard.as_ref(),
        (ShapeKey::Ramble, T::Steady) => x.ramble_steady.as_ref(),
        (ShapeKey::Ramble, T::Nervy) => x.ramble_nervy.as_ref(),
        (ShapeKey::Ramble, T::Hard) => x.ramble_hard.as_ref(),
        (ShapeKey::Fan, T::Steady) => x.fan_steady.as_ref(),
        (ShapeKey::Fan, T::Nervy) => x.fan_nervy.as_ref(),
        (ShapeKey::Fan, T::Hard) => x.fan_hard.as_ref(),
        (ShapeKey::Stoic, T::Steady) => x.stoic_steady.as_ref(),
        (ShapeKey::Stoic, T::Nervy) => x.stoic_nervy.as_ref(),
        (ShapeKey::Stoic, T::Hard) => x.stoic_hard.as_ref(),
        (ShapeKey::Con, T::Steady) => x.con_steady.as_ref(),
        (ShapeKey::Con, T::Nervy) => x.con_nervy.as_ref(),
        (ShapeKey::Con, T::Hard) => x.con_hard.as_ref(),
        (ShapeKey::Game, T::Steady) => x.game_steady.as_ref(),
        (ShapeKey::Game, T::Nervy) => x.game_nervy.as_ref(),
        (ShapeKey::Game, T::Hard) => x.game_hard.as_ref(),
        (ShapeKey::Menace, T::Steady) => x.menace_steady.as_ref(),
        (ShapeKey::Menace, T::Nervy) => x.menace_nervy.as_ref(),
        (ShapeKey::Menace, T::Hard) => x.menace_hard.as_ref(),
    }
}

/// The words for what was taken, off the office: DRIVE_AWAY with DRIVE_AWAY_GUIDE laid over.
pub(super) fn drive_away(g: &crate::game::Game) -> crate::dialogue::DriveAway { drive_away_of() }

fn drive_away_of() -> crate::dialogue::DriveAway {
    let (was, now) = (crate::dialogue::drive_away_table(), crate::dialogue::drive_away_guide());
    crate::dialogue::DriveAway {
        steady: now.steady.or(was.steady),
        nervy: now.nervy.or(was.nervy),
        hard: now.hard.or(was.hard),
        under: now.under.or(was.under),
    }
}

fn intel_text_of(kind: IntelKind, t: crate::game::Temperament, k: Knowledge) -> String {
    let x = table_of(kind);
    of_t_k(&x, t, k).or_else(|| of_t(&x, t)).or_else(|| of_k(&x, k)).or(x.under.as_ref()).cloned().unwrap_or_default()
}

/// The text a man of this temperament and knowledge gives for `kind`, the most specific variant written.
pub(super) fn intel_text(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind, t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String {
    intel_text_of(kind, t, k)
}

fn shaped_of(kind: IntelKind, id: &str, shape: ShapeKey, t: crate::game::Temperament) -> Option<String> {
    let x = table_of(kind);
    x.sc.get(st(id)).or_else(|| of_shape_t(&x, shape, t)).or_else(|| of_shape(&x, shape)).cloned()
}

/// The script's own or the shape-voiced variant of `kind`, or None when none is written.
pub(super) fn shaped_intel_text(g: &mut crate::game::Game, kind: crate::dialogue::IntelKind, sc: &crate::dialogue::ShapedIntelTextSc, t: crate::game::Temperament) -> Option<String> {
    shaped_of(kind, &sc.id, sc.shape, t)
}

/// The checker's own words when he is the man walking the silent colleague's route.
pub(super) fn check_on_self(g: &crate::game::Game) -> String {
    st(crate::dialogue::CHECKON_SELF_NOW)
}

/// What goes in front of a given fact.
pub(super) fn intel_lead(t: crate::game::Temperament, k: crate::dialogue::Knowledge) -> String {
    match t {
        crate::game::Temperament::Hard => st("…Fine. "),
        crate::game::Temperament::Steady if k == Knowledge::Saw || k == Knowledge::SawHurt => st("…All right. "),
        _ => String::new(),
    }
}

/// A fact behind a lead that runs on into it starts in lower case, unless it starts with 'I' or a man's name. By hand, the regexes of TS 366:
/// `/[a-z—] $/` on the lead, `/^(?!I\b|\{(?:Checker|Checked|Missing|Partner|Victim|Caller|Body)\})(\{?)([A-Z])/` on the fact.
fn run_on_text(lead: &str, fact: &str) -> String {
    let mut back = lead.chars().rev();
    let runs = back.next() == Some(' ') && matches!(back.next(), Some('a'..='z') | Some('—'));
    if !runs { return st(fact); }
    if let Some(after) = fact.strip_prefix('I') {
        // \b after the I: JavaScript's \w is ASCII only, with no `u` flag.
        if !matches!(after.chars().next(), Some(c) if c.is_ascii_alphanumeric() || c == '_') { return st(fact); }
    }
    for name in ["{Checker}", "{Checked}", "{Missing}", "{Partner}", "{Victim}", "{Caller}", "{Body}"] {
        if fact.starts_with(name) { return st(fact); }
    }
    let (brace, rest) = match fact.strip_prefix('{') { Some(r) => ("{", r), None => ("", fact) };
    match rest.chars().next() {
        Some(c) if c.is_ascii_uppercase() => format!("{}{}{}", brace, c.to_ascii_lowercase(), &rest[1..]),
        _ => st(fact),
    }
}

pub(super) fn run_on(g: &mut crate::game::Game, lead: &str, fact: &str) -> String { run_on_text(lead, fact) }

/// Four digits off a callsign, the same every night for the same man.
pub(super) fn badge_of(callsign: &str) -> String {
    let n = 1000.0 + js::rem(crate::math_dice::hash_str(&format!("badge:{}", callsign)), 9000.0);
    format!("{}", n as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Temperament;

    const TS: [Temperament; 3] = [Temperament::Steady, Temperament::Nervy, Temperament::Hard];
    const KS: [Knowledge; 5] = [Knowledge::None, Knowledge::Heard, Knowledge::Glimpsed, Knowledge::Saw, Knowledge::SawHurt];
    const SH: [ShapeKey; 9] = [ShapeKey::Pedant, ShapeKey::Selfown, ShapeKey::Folder, ShapeKey::Ramble, ShapeKey::Fan, ShapeKey::Stoic, ShapeKey::Con, ShapeKey::Game, ShapeKey::Menace];

    /// A FAILURE MEANS: a man gives a fact in silence: some temper or knowledge has no words for it.
    #[test]
    fn every_man_has_words_for_every_fact() {
        for kind in crate::dialogue::intel_kinds() {
            for t in TS { for k in KS { assert!(!intel_text_of(kind, t, k).is_empty(), "{} {} {}", kind.as_str(), t.as_str(), k.as_str()); } }
        }
    }

    /// A FAILURE MEANS: a nervy man who saw it talks like a man who knows nothing: the most specific words written are the ones said.
    #[test]
    fn a_fact_is_said_in_the_most_specific_words_written_for_the_man() {
        let (mut both, mut temper) = (0, 0);
        for kind in crate::dialogue::intel_kinds() {
            let x = table_of(kind);
            for t in TS { for k in KS {
                let said = intel_text_of(kind, t, k);
                if let Some(w) = of_t_k(&x, t, k) { assert_eq!(&said, w); both += 1; }
                else if let Some(w) = of_t(&x, t) { assert_eq!(&said, w); temper += 1; }
                else if let (None, Some(w)) = (of_k(&x, k), x.under.as_ref()) { assert_eq!(&said, w); }
            } }
        }
        assert!(both > 0 && temper > 0, "the table has lost what this test reads");
    }

    /// A FAILURE MEANS: a script's man says the common line where his own was written, or a voice's line for another temper.
    #[test]
    fn a_shaped_fact_takes_the_scripts_own_then_its_temper_then_its_shape() {
        let door = table_of(IntelKind::Door);
        let own = door.sc.get(st("selfown-s1")).cloned();
        assert!(own.is_some());
        assert_eq!(shaped_of(IntelKind::Door, "selfown-s1", ShapeKey::Selfown, Temperament::Steady), own);
        assert_eq!(shaped_of(IntelKind::Door, "nope", ShapeKey::Selfown, Temperament::Steady), door.selfown_steady);
        let (mut temper, mut shape) = (0, 0);
        for kind in crate::dialogue::intel_kinds() {
            let x = table_of(kind);
            for sh in SH { for t in TS {
                let said = shaped_of(kind, "nope", sh, t);
                if let Some(w) = of_shape_t(&x, sh, t) { assert_eq!(said.as_ref(), Some(w)); temper += 1; }
                else if let Some(w) = of_shape(&x, sh) { assert_eq!(said.as_ref(), Some(w)); shape += 1; }
                else { assert_eq!(said, None); }
            } }
        }
        assert!(temper > 0 && shape > 0, "the table has lost what this test reads");
    }

    /// A FAILURE MEANS: the words of an older pass come back: what the guide wrote lies over what came before it.
    #[test]
    fn the_guides_words_lie_over_the_old() {
        let guide = crate::dialogue::intel_guide();
        let live = guide.rollcall_live.unwrap();
        assert!(live.under.is_some() && live.under != crate::dialogue::intel_text_table().get(st("rollcallLive")).unwrap().under);
        assert_eq!(table_of(IntelKind::RollcallLive).under, live.under);
        let (was, now, d) = (crate::dialogue::drive_away_table(), crate::dialogue::drive_away_guide(), drive_away_of());
        assert!(now.under.is_some() && now.under != was.under);
        assert_eq!((d.under, d.hard, d.steady), (now.under, now.hard, None));
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
        let l = SaidTextL { text: st("t"), now: Some(st("n")), away: Some(st("a")), bare: Some(st("b")), guide: Some(st("g")) };
        let mut ctx = DialogueCtx::default();
        assert_eq!(said_of(&l, &ctx), "a");
        ctx.facts.office = Some(false);
        assert_eq!(said_of(&l, &ctx), "a");
        ctx.facts.office = Some(true);
        assert_eq!(said_of(&l, &ctx), "g");
        ctx.nameless = Some(true);
        assert_eq!(said_of(&l, &ctx), "b");
        let l2 = SaidTextL { text: st("t"), now: None, away: Some(st("")), bare: Some(st("")), guide: Some(st("")) };
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
        assert_eq!(why, vec![st("floor +0.22"), st("held -0.12")]);
    }
}
