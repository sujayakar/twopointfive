//! src/game/picture.ts lines 452 to 494
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The panel's one line: the squad's men and their ideas, then how many of the other side know of anybody.   (picture.ts pictureSummary)
pub fn picture_summary(g: &mut crate::game::Game) -> String {
    let men = match g.m.picture.states.as_ref() {
        Some(s) => s.men.keys(),
        None => return String::from("picture ▸ nobody yet"),
    };
    let mut parts: Vec<String> = Vec::new();
    let (mut foes, mut know) = (0, 0);
    for gd in men {
        if !fighter(g, gd) { continue; }
        // copied out: `name_of` and `g.time` are read while the ideas are walked, and nothing here changes them
        let ideas: Vec<Idea> = g.m.picture.states.as_ref()
            .and_then(|s| s.men.get(gd))
            .map_or_else(Vec::new, |p| p.ideas.values().copied().collect());
        if g.gd[gd].team != crate::teams::Team::Operative {
            foes += 1;
            if !ideas.is_empty() { know += 1; }
            continue;
        }
        let mut lines: Vec<String> = Vec::new();
        for i in &ideas {
            let age = g.time - i.t;
            let name = name_of(g, i.who);
            let lost = if i.lost { " (lost)" } else { "" };
            lines.push(format!("{} {} {} s{}", name, i.how.as_str(), js::to_fixed(age, if age < 10.0 { 1 } else { 0 }), lost));
        }
        let said = if lines.is_empty() { String::from("nobody") } else { lines.join(", ") };
        parts.push(format!("{}: {}", g.gd[gd].callsign, said));
    }
    if foes != 0 { parts.push(format!("{} of {} of the other side know of somebody", know, foes)); }
    let t = &g.stats.picture;
    parts.push(format!("{} seen, {} heard, {} told, {} lost, {} asks", t.seen, t.heard, t.told, t.lost, t.asks));
    format!("picture ▸ {}", parts.join(" · "))
}

/// For the check and a colleague's shots: one man's idea of one enemy (a copy: no caller writes to it).   (picture.ts ideaOf)
pub fn idea_of(g: &mut crate::game::Game, gd: GuardId, who: crate::picture::Who) -> Option<crate::picture::Idea> {
    g.m.picture.states.as_ref()?.men.get(gd)?.ideas.get(who).copied()
}
