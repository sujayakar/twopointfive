pub use ::game_types::doorthrow::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::vec::Vec;

/// Whose man: Sam and his squad 1, the defenders 2, a civilian 3.
fn side_of(t: crate::teams::Team) -> f64 {
    if t.ours() { 1.0 } else if t.theirs() { 2.0 } else { 3.0 }
}

/// The guard or civilian whose body has this character id.
fn man_of(g: &Game, id_2: f64) -> Option<GuardId> {
    for i in 0..g.guards.len() {
        let x = g.guards[i];
        if g.char_id(x) == id_2 {
            return Some(x);
        }
    }
    for i in 0..g.civilians.len() {
        let x = g.civilians[i];
        if g.char_id(x) == id_2 {
            return Some(x);
        }
    }
    None
}

/// Doors.man: side | MAN_FOLDED | MAN_BRACED | MAN_CALM | MAN_HELD for this id; 0 = nobody the game knows.
pub(super) fn door_man(g: &mut Game, who: f64) -> f64 {
    let (again, now) = (g.k.doorthrow.again, g.time);
    let folded_bit = js::to_i32(crate::doors::MAN_FOLDED);
    let braced_bit = js::to_i32(crate::doors::MAN_BRACED);
    let held_bit = js::to_i32(crate::doors::MAN_HELD);
    let calm_bit = js::to_i32(crate::doors::MAN_CALM);
    if let Some(seat) = crate::seats::view_of_id(g, who) {
        // a player's man: over his own seat's view
        let (folded, until) = crate::seats::with_seat(g, seat, &mut |g: &mut Game| {
            (crate::samstun::sam_staggered(g, &crate::seats::here(g), None), g.cond[g.pl[g.player].ch].fold_until)
        });
        let braced = if until >= 0.0 && now < until + again {
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
    let until = m::max(g.cond[g.gd[gd].ch].fold_until, g.cond[g.gd[gd].ch].blind_until);
    let side = side_of(g.team(gd));
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

/// Whose leaf it was, for the teams' own table (nobody the game knows: no foe's).
fn team_of(g: &Game, id_2: f64) -> Option<crate::teams::Team> {
    if crate::seats::is_man_id(g, id_2) {
        return Some(crate::teams::Team::Ours);
    }
    man_of(g, id_2).map(|gd| g.team(gd))
}

/// After the leaves are stepped: a carried man is settled against the walls, a struck man startled or thrown.
pub fn door_throw_after(g: &mut Game, contacts: &[crate::doors::DoorContact]) {
    for c in contacts {
        let moved = c.moved == Some(true);
        if !moved && c.struck.is_none() {
            continue;
        }
        if moved {
            g.stats.doorthrow.carried += 1;
        }
        if let Some(seat) = crate::seats::view_of_id(g, c.who) {
            if let Some(s) = c.struck {
                if s.tier != crate::doors::DoorStrikeTier::Open {
                    crate::seats::with_seat(g, seat, &mut |g: &mut Game| throw_sam(g, s));
                }
            }
            continue;
        }
        let gd = match man_of(g, c.who) {
            Some(gd) if !crate::condition::down(g, g.gd[gd].ch) => gd,
            _ => continue,
        };
        if moved {
            // no walk this frame keeps him out of the wall the leaf carried him at
            let ch = g.gd[gd].ch;
            let mut p = g.ch[ch].pos;
            crate::scene_collision::collide_circle(g, &mut p, 0.3, 0.2, 1.5, None);
            crate::scene_collision::clamp_to_world(&mut p, 0.3);
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

/// The counters of one throw.
fn count(g: &mut Game, s: crate::doors::DoorStrike) {
    if s.tier == crate::doors::DoorStrikeTier::Boot {
        g.stats.doorthrow.kicked += 1;
    } else {
        g.stats.doorthrow.pushed += 1;
    }
}

/// A leaf merely opened into him.
fn startle_man(g: &mut Game, gd: GuardId, s: crate::doors::DoorStrike) {
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
    let team = g.team(gd);
    if !by.foe_of(team) || !team.theirs() {
        return; // Sam's men know whose door it is
    }
    if g.gd[gd].state == crate::game::GuardState::Patrol {
        if g.gd[gd].script.is_some() {
            crate::squad::end_guard_script(g, gd, Some(crate::game::GuardState::Suspicious));
        }
        let startle = g.k.doorthrow.startle;
        crate::mind::set_state(g, gd, crate::game::GuardState::Suspicious, "doorthrow");
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, startle);
        g.gd[gd].search_t = 0.0;
        g.gd[gd].drop_path();
        g.gd[gd].stall_ref = None;
        let weapon = g.gd[gd].weapon;
        g.fa[weapon].burst = None;
        if !crate::startle::give_startle(g, gd, door, Some(startle), None) {
            let ch = g.gd[gd].ch;
            crate::character::anim_flinch(g, ch);
        }
        g.stats.doorthrow.startled += 1;
    }
    if g.gd[gd].state == crate::game::GuardState::Suspicious {
        let wary = g.k.doorthrow.wary;
        crate::mind::fix(g, gd, Some(door), "doorthrow");
        crate::mind::raise(g, gd, wary, "doorthrow");
    }
}

/// A leaf slammed or booted into him: folded and blind for the seconds, by the stun charge's two stamps.
fn throw_man(g: &mut Game, gd: GuardId, s: crate::doors::DoorStrike) {
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
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::BodyHit, Some(heard), Some(if hard { 0.8 } else { 0.55 }), Some(&crate::audio_iface::PlayOpts::Rate { rate: if hard { 0.9 } else { 1.05 } }));
    crate::condition::fold(g, c, secs);
    count(g, s);
    if g.gd[gd].civ.is_some() {
        crate::civilians::to_alarmed(g, gd, crate::civilians::CivCause::Noise, [at[0], 0.0, at[2]], false);
        return;
    }
    if g.gd[gd].script.is_some() {
        let then = match g.gd[gd].state {
            s @ (crate::game::GuardState::Search | crate::game::GuardState::Alert) => s,
            _ => crate::game::GuardState::Suspicious,
        };
        crate::squad::end_guard_script(g, gd, Some(then));
    }
    crate::condition::blind(g, c, secs, Some(at));
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, secs);
    let weapon = g.gd[gd].weapon;
    g.fa[weapon].burst = None;
    crate::condition::stopped_dead(g, gd);
    if s.floor {
        g.gd[gd].door_down_until = m::max(g.gd[gd].door_down_until, until);
    }
    let by = team_of(g, s.by);
    if let Some(by) = by {
        if by.foe_of(g.team(gd)) {
            let aware = g.k.doorthrow.aware;
            crate::mind::raise(g, gd, aware, "doorthrow");
            if g.gd[gd].state != crate::game::GuardState::Alert || g.gd[gd].last_known.is_none() {
                crate::belief::word(g, gd, [at[0], 0.0, at[2]]);
            }
        }
    }
}

/// A leaf slammed or booted into the seat's man.
fn throw_sam(g: &mut Game, s: crate::doors::DoorStrike) {
    let pl = g.player;
    let hard = s.tier == crate::doors::DoorStrikeTier::Boot;
    let secs = if hard { g.k.doorthrow.kick_secs } else { g.k.doorthrow.push_secs };
    if crate::condition::down(g, g.pl[pl].ch) {
        return;
    }
    crate::samstun::stagger_player(g, secs, [s.dir[0], 0.0, s.dir[1]]);
    let heard = v3::add(g.player_pos(pl), [0.0, 1.1, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::BodyHit, Some(heard), Some(if hard { 0.8 } else { 0.55 }), Some(&crate::audio_iface::PlayOpts::Rate { rate: if hard { 0.9 } else { 1.05 } }));
    if s.floor {
        crate::unseen::kneel(g, pl);
    }
    count(g, s);
}
