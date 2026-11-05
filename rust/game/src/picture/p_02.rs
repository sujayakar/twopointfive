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
            lines.push(format!("{} {} {} s{}", name, i.how.as_str(), to_fixed(age, if age < 10.0 { 1 } else { 0 }), lost));
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

/// `Number.prototype.toFixed` for a time on the panel: Rust's formatter takes a tie of the exact binary value to even, JavaScript away from zero.
/// A tie at `digits` is exactly a value that 2^(digits + 1) makes an odd whole number.
fn to_fixed(x: f64, digits: usize) -> String {
    let scaled = m::abs(x) * (1u64 << (digits + 1)) as f64;
    let tie = scaled == m::floor(scaled) && js::rem(scaled, 2.0) == 1.0;
    if !tie { return format!("{:.*}", digits, x); }
    // one step out from the tie, away from zero: rounds as JavaScript's does, and prints the same digits
    format!("{:.*}", digits, f64::from_bits(x.to_bits() + 1))
}

#[cfg(test)]
mod tests {
    use super::to_fixed;

    /// each expected string is what bun printed for `x.toFixed(d)`
    #[test]
    fn to_fixed_is_the_language_s() {
        let cases: [(f64, usize, &str); 18] = [
            (0.25, 1, "0.3"), (0.75, 1, "0.8"), (1.25, 1, "1.3"), (2.5, 0, "3"), (3.5, 0, "4"), (10.5, 0, "11"),
            (11.5, 0, "12"), (0.05, 1, "0.1"), (0.15, 1, "0.1"), (9.95, 1, "9.9"), (9.96, 1, "10.0"), (-0.25, 1, "-0.3"),
            (-0.04, 1, "-0.0"), (-2.5, 0, "-3"), (0.0, 1, "0.0"), (1234.5, 0, "1235"), (0.2, 1, "0.2"), (1000000.5, 0, "1000001"),
        ];
        for (x, d, want) in cases {
            assert_eq!(to_fixed(x, d), want, "{}.toFixed({})", x, d);
        }
    }
}
