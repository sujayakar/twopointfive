use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use alloc::string::String;

/// Is this man's down one the card counts as the partner's: the tick's test, asked of him later.
pub fn team_tallied(g: &crate::game::Game, gd: GuardId) -> bool {
    if !g.staged.facts.unseen
        || !crate::condition::down(g, g.gd[gd].ch)
        || g.gd[gd].down_by != Some(crate::game::DownBy::Guard)
        || !g.team(gd).theirs()
    {
        return false;
    }
    let how = g.gd[gd].down_kind;
    if how != Some(crate::game::DownKind::Shot) {
        return true;
    }
    let killer = g.gd[gd].killer_id;
    let mates = g.staged.team.clone();
    mates.iter().any(|&m| g.char_id(m) == killer)
}

/// At the end of a step: tally the defenders the partner put down in it.
pub fn team_tally_tick(g: &mut crate::game::Game) {
    if !g.staged.facts.unseen || !crate::mission::mission_live(g) {
        return;
    }
    let mates = g.staged.team.clone();
    if mates.is_empty() {
        return;
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !crate::condition::down(g, g.gd[gd].ch)
            || g.gd[gd].died_at != g.time
            || g.gd[gd].down_by != Some(crate::game::DownBy::Guard)
            || !g.team(gd).theirs()
        {
            continue;
        }
        let how = g.gd[gd].down_kind;
        let shot = how == Some(crate::game::DownKind::Shot);
        let round = shot;
        let killer = g.gd[gd].killer_id;
        let mate = mates.iter().copied().find(|&m| g.char_id(m) == killer);
        if round && mate.is_none() {
            continue; // a round none of the team fired: his own side's
        }
        crate::mission::tally(g, if shot { crate::mission::TallyK::Kills } else { crate::mission::TallyK::Knockouts });
        if round {
            g.chronicle.counts.shots_near_men += 1.0;
        }
        let who = g.gd[gd].callsign.clone();
        let pos = g.pos_of(gd);
        let mut data: JsMap<String, Option<crate::chronicle::ChronicleDataValue>> = JsMap::new();
        data.set(String::from("who"), Some(crate::chronicle::ChronicleDataValue::Text(who)));
        if shot {
            // `round` holds here, so the walk above found his mate
            let weapon = alloc::format!("{}'s round", g.gd[mate.unwrap()].callsign);
            data.set(String::from("weapon"), Some(crate::chronicle::ChronicleDataValue::Text(weapon)));
            crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Kill, Some(&data), Some(pos));
        } else {
            let method = if how == Some(crate::game::DownKind::Choked) {
                "choke"
            } else {
                "takedown"
            };
            data.set(String::from("method"), Some(crate::chronicle::ChronicleDataValue::Text(String::from(method))));
            crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Ko, Some(&data), Some(pos));
        }
    }
}
