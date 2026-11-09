//! src/game/quietfog.ts lines 1 to 185
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The rung a word names, if it names one: the narrowing `isRung` gives the TypeScript. (quietfog.ts:64, 143)
fn rung_of(v: Option<&str>) -> Option<QuietRung> {
    [QuietRung::Off, QuietRung::Team, QuietRung::Cones, QuietRung::Own].into_iter().find(|r| Some(r.as_str()) == v)
}

/// A link is the solo night's own, to be read as a quiet row's: `solo` on it, nothing else staged by it. (quietfog.ts soloFogLink)
pub fn solo_fog_link(g: &crate::game::Game, q: &crate::query::Query) -> bool {
    q.has("solo") && !not_solo().iter().any(|k| q.has(k))
}

/// A quiet mission that hands out its cameras hands the player a wand as well. (quietfog.ts quietWand)
pub fn quiet_wand(g: &crate::game::Game) -> bool {
    crate::stickycam::sticky_visit(g) > 0.0
}

/// Is this word one of the lever's rungs? (quietfog.ts isRung)
pub fn is_rung(v: Option<&str>) -> bool {
    quiet_rungs().iter().any(|r| Some(r.id.as_str()) == v)
}

/// The line beside the lever for a rung ('' = none). (quietfog.ts quietFogWords)
pub fn quiet_fog_words(rung: Option<&str>) -> String {
    match quiet_rungs().into_iter().find(|r| Some(r.id.as_str()) == rung) {
        Some(r) => r.words,
        None => String::new(),
    }
}

/// Where a quiet mission starts with nothing kept and nothing on its link; the enum is always a rung. (quietfog.ts quietFogDefault)
pub fn quiet_fog_default(g: &crate::game::Game) -> crate::quietfog::QuietRung {
    g.k.quietfog.def
}

/// The rung this machine keeps for its quiet missions: the default with nothing kept. The store's read is the host's. (quietfog.ts quietFog)
pub fn quiet_fog(g: &mut crate::game::Game) -> crate::quietfog::QuietRung {
    let def = quiet_fog_default(g);
    let qf = &mut g.m.quietfog;
    match qf.pick {
        Some(pick) if qf.pick_def == def || qf.kept => pick,
        _ => {
            qf.pick = Some(def);
            qf.pick_def = def;
            qf.kept = false;
            def
        }
    }
}

/// The cameras come with the fog unasked where a quiet mission starts under the fog. (quietfog.ts quietCamsDefault)
pub fn quiet_cams_default(g: &crate::game::Game) -> bool {
    quiet_fog_default(g) != QuietRung::Off
}

/// Does this machine's quiet mission come with its cameras? Only ever with a rung up. The store's read is the host's. (quietfog.ts quietCams)
pub fn quiet_cams(g: &mut crate::game::Game) -> bool {
    g.m.quietfog.cams_read = true;
    let cams = match g.m.quietfog.cams {
        Some(c) => c,
        None => quiet_cams_default(g),
    };
    cams && quiet_fog(g) != QuietRung::Off
}

/// The cameras' button: pressed with the lever on 'off' it moves the lever up. The store's write is the host's. (quietfog.ts setQuietCams)
pub fn set_quiet_cams(g: &mut crate::game::Game, on: bool, keep: Option<bool>) {
    g.m.quietfog.cams = Some(on);
    g.m.quietfog.cams_read = true;
    if on && quiet_fog(g) == QuietRung::Off {
        let rung = if quiet_cams_default(g) { quiet_fog_default(g) } else { QuietRung::Team };
        set_quiet_fog(g, rung, keep);
    }
}

/// The cameras a quiet mission's query asks to be handed: none named is the default's, `cams=N` is N within what can be carried. (quietfog.ts quietCamsOfQuery)
pub fn quiet_cams_of_query(g: &crate::game::Game, q: &crate::query::Query, lesson: Option<bool>) -> f64 {
    if lesson.unwrap_or(false) || !(q.has("quiet") || solo_fog_link(g, q)) { return 0.0; }
    if quiet_fog_of_query(g, q, lesson) == QuietRung::Off { return 0.0; }
    if !q.has(PARAM_CAMS) {
        return if quiet_cams_default(g) { g.k.sticky.issue } else { 0.0 };
    }
    let n = q.number(PARAM_CAMS).unwrap_or(0.0);   // (`Number(null)` is 0: the name is there, asked above)
    if n.is_finite() { m::max(0.0, m::min(g.k.sticky.carry, m::floor(n))) } else { 0.0 }
}

/// Move the lever on a quiet row. On 'off', where the cameras are a button, the press goes too. The store's writes are the host's. (quietfog.ts setQuietFog)
pub fn set_quiet_fog(g: &mut crate::game::Game, rung: crate::quietfog::QuietRung, keep: Option<bool>) {
    let def = quiet_fog_default(g);
    g.m.quietfog.pick = Some(rung);
    g.m.quietfog.pick_def = def;
    g.m.quietfog.kept = rung != def;
    if rung == QuietRung::Off && !quiet_cams_default(g) {
        g.m.quietfog.cams = None;
        g.m.quietfog.cams_read = true;
    }
}

/// What this machine keeps of the rung, as the store's read takes it: a word that is no rung is nothing. (quietfog.ts quietFog)
pub fn quiet_fog_kept(g: &mut crate::game::Game, word: &str) {
    let Some(rung) = rung_of(Some(word)) else { return };
    g.m.quietfog.pick = Some(rung);
    g.m.quietfog.pick_def = quiet_fog_default(g);
    g.m.quietfog.kept = true;
}

/// What this machine keeps of the cameras, as the store's read takes it: no lever moves. (quietfog.ts quietCams)
pub fn quiet_cams_kept(g: &mut crate::game::Game, on: bool) {
    g.m.quietfog.cams = Some(on);
    g.m.quietfog.cams_read = true;
}

/// The text this machine keeps of the rung ('' = none: the default's own stores nothing). (quietfog.ts setQuietFog)
pub fn quiet_fog_text(g: &mut crate::game::Game) -> String {
    let rung = quiet_fog(g);
    if rung == quiet_fog_default(g) { String::new() } else { String::from(rung.as_str()) }
}

/// The text this machine keeps of the cameras just pressed ('' = none: the default's own stores nothing). (quietfog.ts setQuietCams)
pub fn quiet_cams_text(g: &crate::game::Game, on: bool) -> String {
    if on == quiet_cams_default(g) { String::new() } else { String::from(if on { "1" } else { "0" }) }
}

/// The pick forgotten, to be read again. (quietfog.ts quietFogForget)
pub fn quiet_fog_forget(g: &mut crate::game::Game) {
    g.m.quietfog.pick = None;
    g.m.quietfog.kept = false;
    g.m.quietfog.cams = None;
    g.m.quietfog.cams_read = false;
}

/// A mission link with the quiet pick put right on it: a rung it carried is dropped, and on a quiet mission's link this machine's pick goes on where it is not the default's. (quietfog.ts quietFogHref)
pub fn quiet_fog_href(g: &mut crate::game::Game, href: &str, quiet: bool) -> String {
    let mut split = href.split('?');
    let path = split.next().unwrap_or("");
    let query = split.next().unwrap_or("");
    let all: Vec<&str> = query.split('&').filter(|k| !k.is_empty()).collect();
    let mut keys: Vec<String> = Vec::new();
    for k in all.iter() {
        let mut word = k.split('=');
        let n = word.next().unwrap_or("");
        let v = word.next();
        if n != PARAM_CAMS && (n != PARAM || v == Some("0")) { keys.push(String::from(*k)); }
    }
    let kept = keys.len();
    if quiet {
        let rung = quiet_fog(g);
        let cams = quiet_cams(g);
        if rung != quiet_fog_default(g) { keys.push(alloc::format!("{}={}", PARAM, rung.as_str())); }
        if rung != QuietRung::Off && cams != quiet_cams_default(g) {
            let n = if cams { g.k.sticky.issue } else { 0.0 };
            keys.push(alloc::format!("{}={}", PARAM_CAMS, js::num_str(n)));
        }
    }
    if keys.len() == kept && kept == all.len() { return String::from(href); }
    if keys.is_empty() { String::from(path) } else { alloc::format!("{}?{}", path, keys.join("&")) }
}

/// The rung a query asks its quiet mission to be played under: none named is the default's; 'off' for what is not a quiet mission, or a lesson. (quietfog.ts quietFogOfQuery)
pub fn quiet_fog_of_query(g: &crate::game::Game, q: &crate::query::Query, lesson: Option<bool>) -> crate::quietfog::QuietRung {
    if lesson.unwrap_or(false) || !(q.has("quiet") || solo_fog_link(g, q)) { return QuietRung::Off; }
    match rung_of(q.get(PARAM)) {
        Some(rung) => rung,
        None => quiet_fog_default(g),
    }
}

/// The rung's hand off the rows: each as it stood, unless moved by hand since, which stays and is the rung's no more. (quietfog.ts lift)
pub fn lift(g: &mut crate::game::Game, h: &mut crate::quietfog::Held) {
    if let Some(cone) = h.cone {
        if g.k.fog.cone == cone.put && crate::difficulty::cone_visit_now(g) == Some(cone.put) {
            crate::difficulty::set_cone_visit(g, cone.was);
        } else {
            h.cone = None;
        }
    }
    if let Some(own) = h.own {
        if g.k.fog.own == own.put {
            g.k.fog.own = own.was;
        } else {
            h.own = None;
        }
    }
}

/// The mission's start: the rung on the fog's two rows for the visit; 'off' lets go of an earlier start's. (quietfog.ts quietFogStart)
pub fn quiet_fog_start(g: &mut crate::game::Game, q: &crate::query::Query, rung: crate::quietfog::QuietRung) {
    quiet_fog_end(g);
    if rung == QuietRung::Off { return; }
    let serial = g.serial();
    g.m.quietfog.held = Some(Held { cone: None, own: None });
    if !q.has("fogcone") {
        let put = rung != QuietRung::Team;
        let was = crate::difficulty::cone_visit_now(g);
        if let Some(held) = g.m.quietfog.held.as_mut() { held.cone = Some(HeldCone { was, put }); }
        crate::difficulty::set_cone_visit(g, Some(put));
    }
    if !q.has("fogown") {
        let put = rung == QuietRung::Own;
        let was = g.k.fog.own;
        if let Some(held) = g.m.quietfog.held.as_mut() { held.own = Some(HeldOwn { was, put }); }
        g.k.fog.own = put;
    }
}

/// The scenario quit, or another started over it: the rows as they stood. (quietfog.ts quietFogEnd)
pub fn quiet_fog_end(g: &mut crate::game::Game) {
    let Some(mut h) = g.m.quietfog.held else { return };
    lift(g, &mut h);
    g.m.quietfog.held = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_link_says_only_what_differs_from_the_default() {
        let mut g = Game::blank();
        assert_eq!(quiet_fog_href(&mut g, "/?quiet=a", true), "/?quiet=a");
        assert_eq!(quiet_fog_href(&mut g, "/?quiet=a&quietfog=own&cams=1&x", true), "/?quiet=a&x");
        assert_eq!(quiet_fog_href(&mut g, "/?quietfog=0&quietfog=own", false), "/?quietfog=0");
        assert_eq!(quiet_fog_href(&mut g, "/?cams=2", false), "/");
        assert_eq!(quiet_fog_href(&mut g, "/?a&&b", false), "/?a&&b");
        set_quiet_fog(&mut g, QuietRung::Own, Some(false));
        assert_eq!(quiet_fog_href(&mut g, "/?quiet=a", true), "/?quiet=a&quietfog=own");
        set_quiet_cams(&mut g, false, Some(false));
        assert_eq!(quiet_fog_href(&mut g, "/", true), "/?quietfog=own&cams=0");
        set_quiet_fog(&mut g, QuietRung::Off, Some(false));
        assert_eq!(quiet_fog_href(&mut g, "/", true), "/?quietfog=off");
    }

    #[test]
    fn the_pick_follows_a_moved_default_unless_somebody_made_it() {
        let mut g = Game::blank();
        assert_eq!(quiet_fog(&mut g), QuietRung::Team);
        assert!(!g.m.quietfog.kept);
        g.k.quietfog.def = QuietRung::Off;
        assert_eq!(quiet_fog(&mut g), QuietRung::Off);
        set_quiet_fog(&mut g, QuietRung::Cones, None);
        assert!(g.m.quietfog.kept);
        g.k.quietfog.def = QuietRung::Team;
        assert_eq!(quiet_fog(&mut g), QuietRung::Cones);
    }

    #[test]
    fn the_cameras_only_ever_come_with_a_rung_up() {
        let mut g = Game::blank();
        assert!(quiet_cams(&mut g));
        set_quiet_fog(&mut g, QuietRung::Off, None);
        assert!(!quiet_cams(&mut g));
        set_quiet_cams(&mut g, true, None);
        assert_eq!(quiet_fog(&mut g), QuietRung::Team);
        g.k.quietfog.def = QuietRung::Off;
        quiet_fog_forget(&mut g);
        set_quiet_cams(&mut g, true, None);
        assert_eq!(quiet_fog(&mut g), QuietRung::Team);
        set_quiet_fog(&mut g, QuietRung::Off, None);
        assert_eq!(g.m.quietfog.cams, None);
    }

    #[test]
    fn a_row_moved_by_hand_stays_and_is_the_rungs_no_more() {
        let mut g = Game::blank();
        g.k.fog.own = true;
        let mut h = Held { cone: None, own: Some(HeldOwn { was: false, put: true }) };
        lift(&mut g, &mut h);
        assert!(!g.k.fog.own);
        assert!(h.own.is_some());
        let mut h = Held { cone: None, own: Some(HeldOwn { was: true, put: true }) };
        lift(&mut g, &mut h);
        assert!(!g.k.fog.own);
        assert!(h.own.is_none());
    }

    #[test]
    fn the_rungs_words() {
        assert!(is_rung(Some("own")) && is_rung(Some("off")));
        assert!(!is_rung(Some("0")) && !is_rung(None));
        assert_eq!(rung_of(Some("cones")), Some(QuietRung::Cones));
        assert_eq!(rung_of(Some("x")), None);
        assert_eq!(quiet_fog_words(None), "");
        assert!(quiet_fog_words(Some("team")).starts_with("you see all around"));
    }
}
