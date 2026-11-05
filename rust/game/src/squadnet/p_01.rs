//! src/game/squadnet.ts lines 1 to 165
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A fresh set of the net's counters. (squadnet.ts newStats)
pub fn new_stats() -> crate::squadnet::SquadNetStats {
    crate::squadnet::SquadNetStats { lines: 0.0, downs: 0.0, whys: 0.0, quiets: 0.0, clears: 0.0, ends: 0.0 }
}

/// The net as a game starts it; the class's constructor adds nothing to its initialisers. (squadnet.ts SquadNet.constructor)
pub fn squad_net_new(g: &mut crate::game::Game) -> crate::squadnet::SquadNet {
    crate::squadnet::SquadNet::blank()
}

/// A fresh encounter: everything but `last_t`, which the TypeScript leaves standing. (squadnet.ts SquadNet.reset)
pub fn squad_net_reset(g: &mut crate::game::Game) {
    let n = &mut g.squad_net;
    n.rows = Vec::new();
    n.down = Vec::new();
    n.seeded = false;
    n.click_due = false;
    n.down_due = false;
    n.stats = new_stats();
}

/// A teammate said a line with his head off your screen: the net clicks. (squadnet.ts SquadNet.line)
pub fn squad_net_line(g: &mut crate::game::Game) {
    g.squad_net.stats.lines += 1.0;
    g.squad_net.click_due = true;
}

/// What a quick-key pick that is not a plain open adds: the end of its row and the answer. (squadnet.ts whyWords)
pub fn why_words(g: &mut crate::game::Game, p: &crate::autopilot::AutoPick) -> Option<crate::squadnet::WhyWords> {
    use crate::autopilot::AutoPickWhy;
    if p.word || !matches!(p.t, crate::orders::OrderTarget::Door(_)) { return None; }
    let id = p.id.as_deref();
    let (row, ack) = if id == Some("bangClear") && p.why == AutoPickWhy::Contact {
        ("they see one inside", "banging it. i see one in there.")
    } else if id == Some("bangClear") && p.why == AutoPickWhy::Unknown {
        ("nobody's been in there", "banging it. nobody's been in there.")
    } else if id == Some("charge") && p.why == AutoPickWhy::Shut {
        ("they can't open it", "we can't open it. setting a charge.")
    } else if id == Some("openClear") && p.kept == Some(true) {
        ("they're saving the last bang", "going in. i'm saving the last bang.")
    } else {
        return None;
    };
    Some(crate::squadnet::WhyWords { row: String::from(row), ack: String::from(ack) })
}

/// The regex `/[^s]s$/`: ends in one s, not two, with something before it ('bunks are', 'boss's').
fn ends_plural(name: &str) -> bool {
    let mut back = name.chars().rev();
    back.next() == Some('s') && matches!(back.next(), Some(c) if c != 's')
}

/// The call a lead makes when an entry is over. None: nothing is said. (squadnet.ts roomCall)
pub fn room_call(g: &mut crate::game::Game, name: &str, left: f64, inside: bool, seen: bool) -> Option<String> {
    if !inside && left <= 0.0 {
        g.squad_net.stats.clears += 1.0;
        return Some(alloc::format!("{name} clear."));
    }
    if left > 0.0 && !seen {
        g.squad_net.stats.quiets += 1.0;
        let verb = if ends_plural(name) { " are" } else { "'s" };
        return Some(alloc::format!("{name}{verb} quiet. we didn't see all of it."));
    }
    None
}

/// The call at the end of an entry into a room the mission has no name for; `open` = open floor. (squadnet.ts plainCall)
pub fn plain_call(g: &mut crate::game::Game, open: bool, left: f64, inside: bool, seen: bool) -> Option<String> {
    let call = if open {
        if seen { None } else { Some(String::from("clear so far.")) }
    } else {
        room_call(g, "room", left, inside, seen)
    };
    if call.is_some() { g.squad_net.stats.ends += 1.0; }
    call
}

/// Does a man at p stand in that room's flood, a doorway of the room's own counted. (squadnet.ts inFlood)
pub fn in_flood(g: &mut crate::game::Game, r: &crate::squadnet::InFloodR, p: V3) -> bool {
    let cell = g.col.nav.cell;
    let c = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
    if r.cells.has(crate::Num(c)) { return true; }
    match crate::guards::door_cells(g).get(crate::Num(c)) {
        Some(d) => r.entries.iter().any(|e| e.door == *d),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autopilot::{AutoPick, AutoPickWhy};
    use crate::orders::{OrderTarget, OrderTargetDoor};

    fn on_big_stack(f: fn()) {
        extern crate std;
        std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
    }

    fn pick(id: &str, why: AutoPickWhy, kept: Option<bool>) -> AutoPick {
        AutoPick {
            id: Some(String::from(id)),
            why,
            kept,
            t: OrderTarget::Door(OrderTargetDoor { at: [0.0; 3], door: DoorId(0), side: 1.0 }),
            ..Default::default()
        }
    }

    #[test]
    fn the_quiet_call_agrees_in_number_as_the_regex_does() {
        for (name, plural) in [("bunks", true), ("server room", false), ("boss", false), ("s", false), ("", false), ("as", true)] {
            assert_eq!(ends_plural(name), plural, "{name:?}");
        }
    }

    #[test]
    fn room_call_and_its_counters() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            assert_eq!(room_call(g, "mess", 0.0, false, false).as_deref(), Some("mess clear."));
            assert_eq!(room_call(g, "bunks", 3.0, false, false).as_deref(), Some("bunks are quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "server room", 3.0, true, false).as_deref(), Some("server room's quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "mess", 0.0, true, false), None);
            assert_eq!(room_call(g, "mess", 3.0, true, true), None);
            assert_eq!(room_call(g, "mess", 3.0, false, true), None);
            assert_eq!((g.squad_net.stats.clears, g.squad_net.stats.quiets), (1.0, 2.0));
        });
    }

    #[test]
    fn plain_call_counts_only_what_it_says() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            assert_eq!(plain_call(g, true, 0.0, true, false).as_deref(), Some("clear so far."));
            assert_eq!(plain_call(g, true, 0.0, false, true), None);
            assert_eq!(plain_call(g, false, 0.0, false, false).as_deref(), Some("room clear."));
            assert_eq!(plain_call(g, false, 3.0, false, false).as_deref(), Some("room's quiet. we didn't see all of it."));
            assert_eq!(plain_call(g, false, 3.0, true, true), None);
            assert_eq!(g.squad_net.stats.ends, 3.0);
        });
    }

    #[test]
    fn why_words_says_only_the_reasons() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            let ack = |g: &mut Game, p: &AutoPick| why_words(g, p).map(|w| w.ack);
            assert_eq!(ack(g, &pick("bangClear", AutoPickWhy::Contact, None)).as_deref(), Some("banging it. i see one in there."));
            assert_eq!(ack(g, &pick("bangClear", AutoPickWhy::Unknown, None)).as_deref(), Some("banging it. nobody's been in there."));
            assert_eq!(ack(g, &pick("charge", AutoPickWhy::Shut, None)).as_deref(), Some("we can't open it. setting a charge."));
            assert_eq!(ack(g, &pick("openClear", AutoPickWhy::Known, Some(true))).as_deref(), Some("going in. i'm saving the last bang."));
            assert_eq!(ack(g, &pick("openClear", AutoPickWhy::Known, Some(false))), None);
            assert_eq!(ack(g, &pick("bangClear", AutoPickWhy::Known, None)), None);
            let mut said = pick("bangClear", AutoPickWhy::Contact, None);
            said.word = true;
            assert_eq!(ack(g, &said), None);
            let mut floor = pick("bangClear", AutoPickWhy::Contact, None);
            floor.t = OrderTarget::Floor(Default::default());
            assert_eq!(ack(g, &floor), None);
        });
    }

    #[test]
    fn reset_keeps_last_t() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            g.squad_net.last_t = 7.0;
            g.squad_net.seeded = true;
            squad_net_line(g);
            assert!(g.squad_net.click_due);
            squad_net_reset(g);
            assert!(!g.squad_net.click_due && !g.squad_net.seeded);
            assert_eq!((g.squad_net.stats.lines, g.squad_net.last_t), (0.0, 7.0));
        });
    }
}
