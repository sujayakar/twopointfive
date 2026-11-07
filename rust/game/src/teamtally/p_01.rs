//! src/game/teamtally.ts lines 1 to 39
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Is this man's down one the card counts as the partner's: the tick's test, asked of him later.   (teamtally.ts teamTallied)
pub fn team_tallied(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.quiet.active
        || g.gd[gd].state != crate::game::GuardState::Dead
        || g.gd[gd].down_by != Some(crate::game::DownBy::Guard)
        || g.gd[gd].civ.is_some()
        || g.gd[gd].team == crate::teams::Team::Operative
        || !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player)
    {
        return false;
    }
    let how = g.gd[gd].down_kind;
    if !(how == Some(crate::game::DownKind::Shot) || how == Some(crate::game::DownKind::Wounded)) {
        return true;
    }
    let killer = g.gd[gd].killer_id;
    let mates = crate::scenario_run::team_of(g);
    mates.iter().any(|&m| g.ch[g.gd[m].ch].id == killer)
}

/// At the end of a step: tally the defenders the partner put down in it.   (teamtally.ts teamTallyTick)
pub fn team_tally_tick(g: &mut crate::game::Game) {
    if !g.quiet.active || !crate::mission::mission_live(g) {
        return;
    }
    let mates = crate::scenario_run::team_of(g);
    if mates.is_empty() {
        return;
    }
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].state != crate::game::GuardState::Dead
            || g.gd[gd].died_at != g.time
            || g.gd[gd].down_by != Some(crate::game::DownBy::Guard)
            || g.gd[gd].civ.is_some()
            || g.gd[gd].team == crate::teams::Team::Operative
            || !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player)
        {
            continue;
        }
        let how = g.gd[gd].down_kind;
        let shot = how == Some(crate::game::DownKind::Shot);
        let wounded = how == Some(crate::game::DownKind::Wounded);
        let round = shot || wounded;
        let killer = g.gd[gd].killer_id;
        let mate = mates.iter().copied().find(|&m| g.ch[g.gd[m].ch].id == killer);
        if round && mate.is_none() {
            continue; // a round none of the team fired: his own side's
        }
        crate::mission::tally(g, if shot { crate::mission::TallyK::Kills } else { crate::mission::TallyK::Knockouts });
        if round {
            g.chronicle.counts.shots_near_men += 1.0;
        }
        let who = g.gd[gd].callsign.clone();
        let pos = g.ch[g.gd[gd].ch].pos;
        let mut data: JsMap<String, Option<crate::chronicle::ChronicleDataValue>> = JsMap::new();
        data.set(String::from("who"), Some(crate::chronicle::ChronicleDataValue::Text(who)));
        if shot {
            // `round` holds here, so the walk above found his mate
            let weapon = alloc::format!("{}'s round", g.gd[mate.unwrap()].callsign);
            data.set(String::from("weapon"), Some(crate::chronicle::ChronicleDataValue::Text(weapon)));
            crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Kill, Some(&data), Some(pos));
        } else {
            let method = if wounded {
                "wounded"
            } else if how == Some(crate::game::DownKind::Choked) {
                "choke"
            } else {
                "takedown"
            };
            data.set(String::from("method"), Some(crate::chronicle::ChronicleDataValue::Text(String::from(method))));
            crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Ko, Some(&data), Some(pos));
        }
    }
}
