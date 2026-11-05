//! src/game/flinch.ts lines 1 to 125
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A man of the defenders or the night staff, no civilian. (flinch.ts staff)
pub fn staff(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].civ.is_none() && (g.gd[gd].team == crate::teams::Team::Hostile || g.gd[gd].team == crate::teams::Team::Guard)
}

/// A man of the room fight's on its own walk carries a script that is not choreography. (flinch.ts fightMan)
pub fn fight_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.fight.defs.iter().any(|&d| g.tab.roomfight_def[d].gd == gd)
}

/// Is the run asked of anybody on this floor now. (flinch.ts runOn)
pub fn run_on(g: &mut crate::game::Game) -> bool {
    g.tune.suppression && g.ai_enabled && !g.touring && !g.quiet_utility
}

/// Does a hostile round that came only `s` m from its muzzle still count here. (flinch.ts flinchInside)
pub fn flinch_inside(g: &mut crate::game::Game, s: f64) -> bool {
    s >= g.k.flinch.min_travel
}

/// One round went close to `c`: duck him off it, and count a green man's fire. `gd` = the man, None for the player. (flinch.ts flinchRound)
pub fn flinch_round(g: &mut crate::game::Game, c_2: CharId, gd: Option<GuardId>, near: f64, from: V3, hostile: bool) {
    let c = c_2;
    if near < g.k.flinch.least || !hostile || g.quiet_utility { return; }
    match gd {
        Some(man) => { if crate::runner::runner_owns(g, man) || !free(g, man) { return; } }
        None => { if g.touring || g.pl[g.player].down { return; } }
    }
    let share = duck_share(g.k.flinch.far, g.k.flinch.least, g.k.flinch.full, near);
    let k = share * flinch_scale(g, gd);
    let toward = v3::sub(from, g.ch[c].pos);
    crate::character::anim_flinch_from(g, c, Some(toward), Some(k), Some(true));
    g.stats.flinch.ducks += 1;
    let id = g.ch[c].id;
    if let Some(last) = g.stats.flinch.last.as_mut() { last.push(id); }
    if let Some(man) = gd {
        if run_on(g) && green(g, man) {
            let now = g.time;
            if let Some(t) = g.m.flinch.state_of.get_mut(man) { t.at = now; }
            else { g.m.flinch.state_of.set(man, FlinchModStateOfValue { under: 0.0, at: now }); }
        }
    }
}

/// How much of the duck a round of weight `near` is worth, before his training. (flinch.ts flinchRound, its `lerp`)
fn duck_share(far: f64, least: f64, full: f64, near: f64) -> f64 {
    v3::lerp(far, 1.0, v3::clamp((near - least) / m::max(1e-6, full - least), 0.0, 1.0))
}

/// Is his body his own to duck with. (flinch.ts free)
pub fn free(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let sc = g.gd[gd].script.as_ref().map(|s| s.flinch);
    if g.touring { return sc == Some(Some(true)); }
    let flinch = match sc {
        None | Some(Some(true)) => return true,
        Some(f) => f,
    };
    flinch != Some(false) && (g.gd[gd].team == crate::teams::Team::Operative || fight_man(g, gd))
}

/// How much of the duck his training leaves him: a steady defender or night-staff man `steady_k`, anybody else 1. (flinch.ts flinchScale)
pub fn flinch_scale(g: &mut crate::game::Game, gd: Option<GuardId>) -> f64 {
    if let Some(man) = gd {
        if staff(g, man) && !g.gd[man].trained && crate::morale::training_of(g, man) == crate::morale::TrainingOf::Steady {
            return g.k.flinch.steady_k;
        }
    }
    1.0
}

/// Is he a man the run is asked of at all: a green defender or night-staff man, free. (flinch.ts green)
pub fn green(g: &mut crate::game::Game, gd: GuardId) -> bool {
    staff(g, gd) && g.gd[gd].held.is_none() && crate::morale::training_of(g, gd) == crate::morale::TrainingOf::Green
}

/// Has he a man of his side by him who still has his gun. (flinch.ts mateBy)
pub fn mate_by(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let eye = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or_else(|| v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]));
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || g.gd[o].state == crate::game::GuardState::Dead || !staff(g, o) || g.gd[o].team != g.gd[gd].team || g.gd[o].held.is_some() || crate::runner::runner_owns(g, o) { continue; }
        let oc = g.gd[o].ch;
        if v3::dist_xz(g.ch[oc].pos, g.ch[c].pos) <= g.k.morale.alone_m {
            let chest = g.ch[oc].bones.get(crate::character::Bone::Chest).copied().unwrap_or_else(|| v3::add(g.ch[oc].pos, [0.0, 1.2, 0.0]));
            if !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) { return true; }
        }
    }
    false
}

/// Once a frame: each green man's seconds under close hostile fire counted up or eased back; the man over the count who is alone goes to the runner. (flinch.ts flinchTick)
pub fn flinch_tick(g: &mut crate::game::Game, dt: f64) {
    let on = run_on(g);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let Some(t) = g.m.flinch.state_of.get(gd).copied() else { continue };
        if !on || g.gd[gd].state == crate::game::GuardState::Dead || crate::runner::runner_owns(g, gd) || !green(g, gd) {
            g.m.flinch.state_of.delete(gd);
            continue;
        }
        if g.time - t.at > g.k.flinch.hot_secs {
            let under = t.under - dt * g.k.flinch.ease_k;
            if let Some(r) = g.m.flinch.state_of.get_mut(gd) { r.under = under; }
            if under <= 0.0 { g.m.flinch.state_of.delete(gd); }
            continue;
        }
        let under = t.under + dt;
        if let Some(r) = g.m.flinch.state_of.get_mut(gd) { r.under = under; }
        if under < g.k.flinch.break_secs { continue; }
        if !mate_by(g, gd) && crate::runner::runner_break(g, gd) {
            g.stats.flinch.broke += 1;
            g.m.flinch.state_of.delete(gd);
        } else {
            let again = g.k.flinch.break_secs - g.k.flinch.ask_secs;
            if let Some(r) = g.m.flinch.state_of.get_mut(gd) { r.under = again; }
        }
    }
}

/// QA: a man's seconds under close fire (0 = none on record). (flinch.ts flinchUnder)
pub fn flinch_under(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match g.m.flinch.state_of.get(gd) { Some(t) => t.under, None => 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The duck's share at the page's numbers, against the TypeScript's own bits (lerp, clamp and Math.max of vec.ts under EXACT).
    #[test]
    fn duck_share_matches_the_typescript_bits() {
        let rows: [(u64, u64); 8] = [
            (0x3faeb851eb851eb8, 0x3fdccccccccccccd),
            (0x3fb1eb851eb851ec, 0x3fde0387f1e03880),
            (0x3fbf9adbb8f8da72, 0x3fe2404c5d819db8),
            (0x3fc999999999999a, 0x3fe6e58469ee5848),
            (0x3fd3333333333333, 0x3fecf72c234f72c2),
            (0x3fd6666666666666, 0x3ff0000000000000),
            (0x3fe0000000000000, 0x3ff0000000000000),
            (0x3ffb333333333333, 0x3ff0000000000000),
        ];
        for (near, want) in rows {
            assert_eq!(duck_share(0.45, 0.06, 0.35, f64::from_bits(near)).to_bits(), want);
        }
    }

    fn script(flinch: Option<bool>) -> Option<crate::game::GuardScript> {
        Some(crate::game::GuardScript { flinch, ..crate::game::GuardScript::zeroed() })
    }

    /// A game with one man in the roster, on the blank's own team.
    fn one_man() -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        let gd = g.gd.push(crate::game::Guard::blank());
        g.guards.push(gd);
        (g, gd)
    }

    /// flinch.ts free: every row of its table, on a show and off one.
    #[test]
    fn free_follows_the_script() {
        let (mut g, gd) = one_man();
        g.gd[gd].team = crate::teams::Team::Hostile;
        for (touring, flinch, has_script, operative, want) in [
            (true, None, false, false, false),
            (true, Some(true), true, false, true),
            (true, Some(false), true, false, false),
            (true, None, true, true, false),
            (false, None, false, false, true),
            (false, Some(true), true, false, true),
            (false, Some(false), true, true, false),
            (false, None, true, false, false),
            (false, None, true, true, true),
        ] {
            g.touring = touring;
            g.gd[gd].script = if has_script { script(flinch) } else { None };
            g.gd[gd].team = if operative { crate::teams::Team::Operative } else { crate::teams::Team::Hostile };
            assert_eq!(free(&mut g, gd), want, "{touring} {flinch:?} {has_script} {operative}");
        }
    }

    /// flinch.ts:102: with the run's gates down nobody keeps a count, and a man with none is passed over.
    #[test]
    fn the_tick_with_the_run_off_clears_the_count() {
        let (mut g, gd) = one_man();
        let other = g.gd.push(crate::game::Guard::blank());
        g.guards.push(other);
        g.tune.suppression = true; g.ai_enabled = true; g.quiet_utility = true;
        g.m.flinch.state_of.set(gd, FlinchModStateOfValue { under: 1.5, at: 0.0 });
        assert_eq!(flinch_under(&mut g, gd), 1.5);
        flinch_tick(&mut g, 0.016);
        assert!(g.m.flinch.state_of.get(gd).is_none());
        assert_eq!(flinch_under(&mut g, gd), 0.0);
    }

    /// flinch.ts:59: the page's least travel is inclusive.
    #[test]
    fn inside_counts_from_the_least_travel() {
        let mut g = Box::new(Game::blank());
        let least = g.k.flinch.min_travel;
        assert!(flinch_inside(&mut g, least));
        assert!(!flinch_inside(&mut g, least - 0.001));
    }
}
