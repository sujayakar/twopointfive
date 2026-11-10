pub use ::game_types::squadnet::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec::Vec};

/// A fresh set of the net's counters.
fn new_stats() -> crate::squadnet::SquadNetStats {
    crate::squadnet::SquadNetStats { lines: 0.0, downs: 0.0, whys: 0.0, quiets: 0.0, clears: 0.0, ends: 0.0 }
}

/// The net as a game starts it; the class's constructor adds nothing to its initialisers.
pub fn squad_net_new(g: &crate::game::Game) -> crate::squadnet::SquadNet {
    crate::squadnet::SquadNet::blank()
}

/// A fresh encounter: everything but `last_t`, which the TypeScript leaves standing.
pub fn squad_net_reset(g: &mut crate::game::Game) {
    let n = &mut g.squad_net;
    n.rows = Vec::new();
    n.down = Vec::new();
    n.seeded = false;
    n.click_due = false;
    n.down_due = false;
    n.stats = new_stats();
}

/// A teammate said a line with his head off your screen: the net clicks.
pub fn squad_net_line(g: &mut crate::game::Game) {
    g.squad_net.stats.lines += 1.0;
    g.squad_net.click_due = true;
}

/// A line that tells the player something, by the man it is about if he can speak, else by the nearest of the seat's men who can. Whether anybody said it.
pub fn team_says(g: &mut crate::game::Game, about: Option<GuardId>, text: &str) -> bool {
    let can = |g: &Game, gd: GuardId| !crate::condition::down(g, g.gd[gd].ch) && g.gd[gd].held.is_none() && !crate::runner::runner_mute(g, gd);
    let by = about.filter(|&gd| can(g, gd)).or_else(|| {
        let at = g.ch[g.pl[g.player].ch].pos;
        let mut best: Option<(GuardId, f64)> = None;
        for gd in crate::orders::squad_orders_men(g, &crate::seats::here(g), None, None) {
            if !can(g, gd) { continue; }
            let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, at);
            if best.map_or(true, |b| d < b.1) { best = Some((gd, d)); }
        }
        best.map(|b| b.0)
    });
    let Some(by) = by else { return false };
    crate::game::game_say(g, by, text, None, None);
    true
}

/// Once a frame, after everybody has moved: who went down this frame, rows past their time, the one sound owed.
pub fn squad_net_tick(g: &mut crate::game::Game) {
    let t = g.time;
    if t < g.squad_net.last_t { squad_net_reset(g); }
    g.squad_net.last_t = t;
    let squad = crate::seats::team_squad(g, &crate::seats::here(g));
    if !g.squad_net.down.is_empty() {
        let kept: Vec<String> = g.squad_net.down.iter()
            .filter(|c| squad.iter().any(|&gd| g.gd[gd].callsign == **c && crate::condition::down(g, g.gd[gd].ch)))
            .cloned().collect();
        g.squad_net.down = kept;
    }
    for &gd in &squad {
        if !crate::condition::down(g, g.gd[gd].ch) || g.squad_net.down.contains(&g.gd[gd].callsign) { continue; }
        let callsign = g.gd[gd].callsign.clone();
        g.squad_net.down.push(callsign.clone());
        if !g.squad_net.seeded { continue; }
        let p = g.ch[g.gd[gd].ch].pos;
        let dur = g.k.squadnet.down_secs;
        g.squad_net.rows.push(crate::squadnet::NetRow { text: alloc::format!("{callsign} is down."), at: [p[0], p[1], p[2]], t, dur });
        g.squad_net.stats.downs += 1.0;
        g.squad_net.down_due = true;
    }
    g.squad_net.seeded = true;
    if !g.squad_net.rows.is_empty() && t - g.squad_net.rows[0].t > g.squad_net.rows[0].dur {
        g.squad_net.rows.retain(|r| t - r.t <= r.dur);
    }
    if g.squad_net.down_due {
        if g.k.squadnet.down_gain > 0.0 {
            let (gain, rate) = (g.k.squadnet.down_gain, g.k.squadnet.down_rate);
            crate::audio_iface::game_audio_like_play(g, "radio", None, Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
        }
    } else if g.squad_net.click_due && g.k.squadnet.click > 0.0 {
        let (gain, rate) = (g.k.squadnet.click, g.k.squadnet.click_rate);
        crate::audio_iface::game_audio_like_play(g, "radio", None, Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
    }
    g.squad_net.click_due = false;
    g.squad_net.down_due = false;
}

/// What a quick-key pick that is not a plain open adds: the end of its row and the answer.
pub fn why_words(g: &crate::game::Game, p: &crate::autopilot::AutoPick) -> Option<crate::squadnet::WhyWords> {
    use crate::autopilot::AutoPickWhy;
    if p.word || !matches!(p.t, crate::orders::OrderTarget::Door(_)) { return None; }
    let id = p.id.as_deref();
    let (row, ack) = if id == Some(crate::orders::rows::bangclear::ORDER.id) && p.why == AutoPickWhy::Contact {
        ("they see one inside", "banging it. i see one in there.")
    } else if id == Some(crate::orders::rows::bangclear::ORDER.id) && p.why == AutoPickWhy::Unknown {
        ("nobody's been in there", "banging it. nobody's been in there.")
    } else if id == Some(crate::orders::rows::charge::ORDER.id) && p.why == AutoPickWhy::Shut {
        ("they can't open it", "we can't open it. setting a charge.")
    } else if id == Some(crate::orders::rows::openclear::ORDER.id) && p.kept == Some(true) {
        ("they're saving the last bang", "going in. i'm saving the last bang.")
    } else {
        return None;
    };
    Some(crate::squadnet::WhyWords { ack: String::from(ack) })
}

/// The regex `/[^s]s$/`: ends in one s, not two, with something before it ('bunks are', 'boss's').
fn ends_plural(name: &str) -> bool {
    let mut back = name.chars().rev();
    back.next() == Some('s') && matches!(back.next(), Some(c) if c != 's')
}

/// The call a lead makes when an entry is over. None: nothing is said.
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

/// The call at the end of an entry into a room the mission has no name for; `open` = open floor.
pub fn plain_call(g: &mut crate::game::Game, open: bool, left: f64, inside: bool, seen: bool) -> Option<String> {
    let call = if open {
        if seen { None } else { Some(String::from("clear so far.")) }
    } else {
        room_call(g, "room", left, inside, seen)
    };
    if call.is_some() { g.squad_net.stats.ends += 1.0; }
    call
}

/// Does a man at p stand in that room's flood, a doorway of the room's own counted.
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

    /// A FAILURE MEANS: a lead calls a room clear with a man still in it, says nothing of a room he could not see all of, or "boss are quiet".
    #[test]
    fn a_lead_calls_clear_only_with_nobody_left_and_quiet_only_of_what_he_did_not_see() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            assert_eq!(room_call(g, "mess", 0.0, false, false).as_deref(), Some("mess clear."));
            assert_eq!(room_call(g, "bunks", 3.0, false, false).as_deref(), Some("bunks are quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "server room", 3.0, true, false).as_deref(), Some("server room's quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "boss", 3.0, true, false).as_deref(), Some("boss's quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "mess", 0.0, true, false), None);
            assert_eq!(room_call(g, "mess", 3.0, true, true), None);
            assert_eq!(room_call(g, "mess", 3.0, false, true), None);
        });
    }

    /// A FAILURE MEANS: an entry onto open floor is called as if it had been a room, or a call is made when nothing can be said.
    #[test]
    fn open_floor_is_clear_so_far_and_a_room_nobody_named_is_called_as_room() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            assert_eq!(plain_call(g, true, 0.0, true, false).as_deref(), Some("clear so far."));
            assert_eq!(plain_call(g, true, 0.0, false, true), None);
            assert_eq!(plain_call(g, false, 0.0, false, false).as_deref(), Some("room clear."));
            assert_eq!(plain_call(g, false, 3.0, false, false).as_deref(), Some("room's quiet. we didn't see all of it."));
            assert_eq!(plain_call(g, false, 3.0, true, true), None);
        });
    }

    /// A FAILURE MEANS: a quick-key pick gives a reason it does not have, or keeps one it has to itself.
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

    /// A FAILURE MEANS: a restart still says "Hicks is down." from the last try, or tells of men already down when the net comes back.
    #[test]
    fn a_restart_clears_the_nets_rows_and_what_it_owes() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            g.squad_net.seeded = true;
            g.squad_net.down.push(String::from("Hicks"));
            g.squad_net.rows.push(crate::squadnet::NetRow { text: String::from("Hicks is down."), at: [0.0; 3], t: 1.0, dur: 3.0 });
            squad_net_line(g);
            assert!(g.squad_net.click_due, "a line off your screen owes a click");
            squad_net_reset(g);
            assert!(g.squad_net.rows.is_empty() && g.squad_net.down.is_empty());
            assert!(!g.squad_net.click_due && !g.squad_net.seeded);
        });
    }
}
