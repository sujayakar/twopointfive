//! src/game/doorthrow.ts lines 1 to 113
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Whose man: Sam and his squad 1, the defenders 2, a civilian 3.   (doorthrow.ts sideOf)
pub fn side_of(t: crate::teams::Team) -> f64 {
    if t == crate::teams::Team::Player || t == crate::teams::Team::Operative {
        1.0
    } else if t == crate::teams::Team::Civilian {
        3.0
    } else {
        2.0
    }
}

/// The guard or civilian whose body has this character id.   (doorthrow.ts manOf)
pub fn man_of(g: &mut Game, id_2: f64) -> Option<GuardId> {
    for i in 0..g.guards.len() {
        let x = g.guards[i];
        if g.ch[g.gd[x].ch].id == id_2 {
            return Some(x);
        }
    }
    for i in 0..g.civilians.len() {
        let x = g.civilians[i];
        if g.ch[g.gd[x].ch].id == id_2 {
            return Some(x);
        }
    }
    None
}

/// Doors.man: side | MAN_FOLDED | MAN_BRACED | MAN_CALM | MAN_HELD for this id; 0 = nobody the game knows.   (doorthrow.ts doorMan)
pub fn door_man(g: &mut Game, who: f64) -> f64 {
    let (again, now) = (g.k.doorthrow.again, g.time);
    let folded_bit = js::to_i32(crate::doors::MAN_FOLDED);
    let braced_bit = js::to_i32(crate::doors::MAN_BRACED);
    let held_bit = js::to_i32(crate::doors::MAN_HELD);
    let calm_bit = js::to_i32(crate::doors::MAN_CALM);
    if let Some(pl) = crate::seats::view_of_id(g, who) {
        // a player's man: over his own seat's view
        let folded = crate::seats::with_seat(g, pl, &mut |g: &mut Game| crate::samstun::sam_staggered(g, None));
        let until = g.pl[pl].stagger_until;
        let braced = if g.puppet.is_some() {
            braced_bit | held_bit
        } else if until >= 0.0 && now < until + again {
            braced_bit
        } else {
            0
        };
        return (1 | (if folded { folded_bit } else { 0 }) | braced) as f64;
    }
    let gd = match man_of(g, who) {
        Some(gd) => gd,
        None => return 0.0,
    };
    let until = m::max(g.gd[gd].stagger_until, g.gd[gd].dazzled_until);
    let side = if g.gd[gd].civ.is_some() { 3.0 } else { side_of(g.gd[gd].team) };
    let state = g.gd[gd].state;
    let braced = if g.gd[gd].held.is_some() {
        braced_bit | held_bit
    } else if until >= 0.0 && now < until + again {
        braced_bit
    } else {
        0
    };
    let calm = side == 3.0 || (side == 2.0 && state != crate::game::GuardState::Alert && state != crate::game::GuardState::Search);
    (js::to_i32(side) | (if now < until { folded_bit } else { 0 }) | braced | (if calm { calm_bit } else { 0 })) as f64
}

/// Whose leaf it was, for the teams' own table (nobody the game knows: no foe's).   (doorthrow.ts teamOf)
pub fn team_of(g: &mut Game, id_2: f64) -> Option<crate::teams::Team> {
    if crate::seats::is_man_id(g, id_2) {
        return Some(crate::teams::Team::Player);
    }
    man_of(g, id_2).map(|gd| g.gd[gd].team)
}

/// After the leaves are stepped: a carried man is settled against the walls, a struck man startled or thrown.   (doorthrow.ts doorThrowAfter)
pub fn door_throw_after(g: &mut Game, contacts: &[crate::doors::DoorContact]) {
    for c in contacts {
        let moved = c.moved == Some(true);
        if !moved && c.struck.is_none() {
            continue;
        }
        if moved {
            g.stats.doorthrow.carried += 1;
        }
        if let Some(pl) = crate::seats::view_of_id(g, c.who) {
            if let Some(s) = c.struck {
                if s.tier != crate::doors::DoorStrikeTier::Open {
                    crate::seats::with_seat(g, pl, &mut |g: &mut Game| throw_sam(g, s));
                }
            }
            continue;
        }
        let gd = match man_of(g, c.who) {
            Some(gd) if g.gd[gd].state != crate::game::GuardState::Dead => gd,
            _ => continue,
        };
        if moved {
            // no walk this frame keeps him out of the wall the leaf carried him at
            let ch = g.gd[gd].ch;
            let mut p = g.ch[ch].pos;
            crate::scene_collision::collide_circle(g, &mut p, 0.3, 0.2, 1.5, None);
            crate::scene_collision::clamp_to_world(g, &mut p, 0.3);
            g.ch[ch].pos = p;
            crate::character::character_update(g, ch, 0.0);
        }
        if let Some(s) = c.struck {
            if s.tier == crate::doors::DoorStrikeTier::Open {
                startle_man(g, gd, s);
            } else {
                throw_man(g, gd, s);
            }
        }
    }
}

/// The counters of one throw.   (doorthrow.ts count)
pub fn count(g: &mut Game, s: crate::doors::DoorStrike) {
    if s.tier == crate::doors::DoorStrikeTier::Boot {
        g.stats.doorthrow.kicked += 1;
    } else {
        g.stats.doorthrow.pushed += 1;
    }
    if s.floor {
        g.stats.doorthrow.floored += 1;
    }
}

/// A leaf merely opened into him.   (doorthrow.ts startleMan)
pub fn startle_man(g: &mut Game, gd: GuardId, s: crate::doors::DoorStrike) {
    if g.gd[gd].held.is_some() {
        return;
    }
    let at = g.dr[s.door].frame_centre;
    let door: V3 = [at[0], 0.0, at[2]];
    if g.gd[gd].civ.is_some() {
        crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Noise, door, false);
        return;
    }
    let by = match team_of(g, s.by) {
        Some(by) => by,
        None => return,
    };
    let team = g.gd[gd].team;
    if !crate::teams::is_hostile(g, by, team) || (team != crate::teams::Team::Guard && team != crate::teams::Team::Hostile) {
        return; // Sam's men know whose door it is
    }
    if g.gd[gd].state == crate::game::GuardState::Patrol {
        if g.gd[gd].script.is_some() {
            crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Suspicious));
        }
        let startle = g.k.doorthrow.startle;
        g.gd[gd].state = crate::game::GuardState::Suspicious;
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, startle);
        g.gd[gd].search_t = 0.0;
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_goal = None;
        g.gd[gd].stall_ref = None;
        let weapon = g.gd[gd].weapon;
        g.fa[weapon].burst = None;
        if !crate::startle::startle_man(g, gd, door, Some(startle), None) {
            let ch = g.gd[gd].ch;
            crate::character::anim_flinch(g, ch);
        }
        g.stats.doorthrow.startled += 1;
    }
    if g.gd[gd].state == crate::game::GuardState::Suspicious {
        let wary = g.k.doorthrow.wary;
        g.gd[gd].last_known = Some(door);
        g.gd[gd].awareness = m::max(g.gd[gd].awareness, wary);
    }
}

/// A leaf slammed or booted into him: folded and blind for the seconds, by the stun charge's two stamps.   (doorthrow.ts throwMan)
pub fn throw_man(g: &mut Game, gd: GuardId, s: crate::doors::DoorStrike) {
    if g.gd[gd].held.is_some() {
        return;
    }
    let hard = s.tier == crate::doors::DoorStrikeTier::Boot;
    let secs = if hard { g.k.doorthrow.kick_secs } else { g.k.doorthrow.push_secs };
    let c = g.gd[gd].ch;
    let away: V3 = [s.dir[0], 0.0, s.dir[1]];
    let at = g.dr[s.door].frame_centre;
    let until = g.time + secs;
    crate::character::anim_hit_from(g, c, Some(away));
    let heard = v3::add(g.ch[c].pos, [0.0, 1.1, 0.0]);
    seam_audio_play(g, "bodyHit", heard, if hard { 0.8 } else { 0.55 }, if hard { 0.9 } else { 1.05 });
    g.gd[gd].stagger_until = m::max(g.gd[gd].stagger_until, until);
    count(g, s);
    if g.gd[gd].civ.is_some() {
        crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Noise, [at[0], 0.0, at[2]], false);
        return;
    }
    if g.gd[gd].script.is_some() {
        let then = match g.gd[gd].state {
            crate::game::GuardState::Patrol => crate::squad::EndGuardScriptState::Suspicious,
            crate::game::GuardState::Search => crate::squad::EndGuardScriptState::Search,
            crate::game::GuardState::Alert => crate::squad::EndGuardScriptState::Alert,
            _ => crate::squad::EndGuardScriptState::Suspicious,
        };
        crate::squad::end_guard_script(g, gd, Some(then));
    }
    g.gd[gd].dazzled_until = m::max(g.gd[gd].dazzled_until, until);
    g.gd[gd].dazzled_at = Some(v3::copy(at));
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, secs);
    let weapon = g.gd[gd].weapon;
    g.fa[weapon].burst = None;
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    g.gd[gd].stall_ref = None;
    g.gd[gd].speed = 0.0;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    if s.floor {
        g.gd[gd].door_down_until = m::max(g.gd[gd].door_down_until, until);
    }
    let by = team_of(g, s.by);
    if let Some(by) = by {
        let team = g.gd[gd].team;
        if crate::teams::is_hostile(g, by, team) {
            let aware = g.k.doorthrow.aware;
            g.gd[gd].awareness = m::max(g.gd[gd].awareness, aware);
            if g.gd[gd].state != crate::game::GuardState::Alert || g.gd[gd].last_known.is_none() {
                g.gd[gd].last_known = Some([at[0], 0.0, at[2]]);
            }
        }
    }
}

/// A leaf slammed or booted into the seat's man.   (doorthrow.ts throwSam)
pub fn throw_sam(g: &mut Game, s: crate::doors::DoorStrike) {
    let pl = g.player;
    let hard = s.tier == crate::doors::DoorStrikeTier::Boot;
    let secs = if hard { g.k.doorthrow.kick_secs } else { g.k.doorthrow.push_secs };
    if g.pl[pl].down || g.puppet.is_some() {
        return;
    }
    crate::samstun::stagger_player(g, secs, [s.dir[0], 0.0, s.dir[1]]);
    let heard = v3::add(g.ch[g.pl[pl].ch].pos, [0.0, 1.1, 0.0]);
    seam_audio_play(g, "bodyHit", heard, if hard { 0.8 } else { 0.55 }, if hard { 0.9 } else { 1.05 });
    if s.floor {
        g.pl[pl].crouch = true;
    }
    count(g, s);
}

/// g.audio.play(name, at, volume, { rate }).
fn seam_audio_play(g: &mut Game, name: &str, at: V3, volume: f64, rate: f64) {
    todo!("needs: game_audio_like_play's opts to carry the number `rate` (crate::Opaque holds nothing)")
}
