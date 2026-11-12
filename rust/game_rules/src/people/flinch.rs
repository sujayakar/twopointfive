pub use ::game_types::flinch::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, vec};

/// A man of the room fight's on its own walk carries a script that is not choreography.
fn fight_man(g: &crate::game::Game, gd: GuardId) -> bool {
    g.fight.defs.iter().any(|&d| g.tab.roomfight_def[d].gd == gd)
}

/// Is the run asked of anybody on this floor now.
fn run_on(g: &crate::game::Game) -> bool {
    g.k.tune.suppression && g.ai_enabled
}

/// Does a hostile round that came only `s` m from its muzzle still count here.
pub(super) fn flinch_inside(g: &crate::game::Game, s: f64) -> bool {
    s >= g.k.flinch.min_travel
}

/// One round went close to `c`: duck him off it, and count a green man's fire. `gd` = the man, None for the player.
pub(super) fn flinch_round(g: &mut crate::game::Game, c_2: CharId, gd: Option<GuardId>, near: f64, from: V3, hostile: bool) {
    let c = c_2;
    if near < g.k.flinch.least || !hostile { return; }
    match gd {
        Some(man) => { if crate::owner::has(g, man, crate::owner::by::RUNNER) || !free(g, man) { return; } }
        None => { if crate::condition::down(g, g.pl[g.player].ch) { return; } }
    }
    let share = duck_share(g.k.flinch.far, g.k.flinch.least, g.k.flinch.full, near);
    let k = share * flinch_scale(g, gd);
    let toward = v3::sub(from, g.ch[c].pos);
    crate::character::anim_flinch_from(g, c, Some(toward), Some(k), Some(true));
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

/// How much of the duck a round of weight `near` is worth, before his training.
fn duck_share(far: f64, least: f64, full: f64, near: f64) -> f64 {
    v3::lerp(far, 1.0, v3::clamp((near - least) / m::max(1e-6, full - least), 0.0, 1.0))
}

/// Is his body his own to duck with.
fn free(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let sc = g.gd[gd].script.as_ref().map(|s| s.flinch);
    let flinch = match sc {
        None | Some(Some(true)) => return true,
        Some(f) => f,
    };
    flinch != Some(false) && (g.team(gd).ours() || fight_man(g, gd))
}

/// How much of the duck his training leaves him: a steady defender or night-staff man `steady_k`, anybody else 1.
pub(super) fn flinch_scale(g: &mut crate::game::Game, gd: Option<GuardId>) -> f64 {
    if let Some(man) = gd {
        if g.team(man).theirs() && !g.gd[man].trained && crate::morale::training_of(g, man) == crate::morale::TrainingOf::Steady {
            return g.k.flinch.steady_k;
        }
    }
    1.0
}

/// Is he a man the run is asked of at all: a green defender or night-staff man, free.
fn green(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.team(gd).theirs() && g.gd[gd].held.is_none() && crate::morale::training_of(g, gd) == crate::morale::TrainingOf::Green
}

/// Has he a man of his side by him who still has his gun.
fn mate_by(g: &crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let eye = g.ch[c].bone_or(crate::character::Bone::Head, 1.6);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || crate::condition::down(g, g.gd[o].ch) || !g.team(o).theirs() || g.team(o) != g.team(gd) || g.gd[o].held.is_some() || crate::owner::has(g, o, crate::owner::by::RUNNER) { continue; }
        let oc = g.gd[o].ch;
        if v3::dist_xz(g.ch[oc].pos, g.ch[c].pos) <= g.k.morale.alone_m {
            let chest = g.ch[oc].bone_or(crate::character::Bone::Chest, 1.2);
            if !crate::scene_collision::segment_blocked(g, eye, chest, None, None, None) { return true; }
        }
    }
    false
}

/// Once a frame: each green man's seconds under close hostile fire counted up or eased back; the man over the count who is alone goes to the runner.
pub fn flinch_tick(g: &mut crate::game::Game, dt: f64) {
    let on = run_on(g);
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let Some(t) = g.m.flinch.state_of.get(gd).copied() else { continue };
        if !on || crate::condition::down(g, g.gd[gd].ch) || crate::owner::has(g, gd, crate::owner::by::RUNNER) || !green(g, gd) {
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
            g.m.flinch.state_of.delete(gd);
        } else {
            let again = g.k.flinch.break_secs - g.k.flinch.ask_secs;
            if let Some(r) = g.m.flinch.state_of.get_mut(gd) { r.under = again; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The duck's share at the page's numbers, against pinned bits.
    #[test]
    fn duck_share_keeps_its_bits() {
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
        Some(crate::game::GuardScript { flinch, ..crate::game::GuardScript::default() })
    }

    /// A game with one man in the roster, on the blank's own team.
    fn one_man() -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        let ch = crate::character::man_new(&mut g, crate::character::Character::blank());
        let gd = g.gd.push(crate::game::Guard { ch, ..crate::game::Guard::blank() });
        g.guards.push(gd);
        (g, gd)
    }

    /// every row of `free`'s table.
    #[test]
    fn free_follows_the_script() {
        let (mut g, gd) = one_man();
        g.ch[g.gd[gd].ch].team = crate::teams::Team::Hostile;
        for (flinch, has_script, operative, want) in [
            (None, false, false, true),
            (Some(true), true, false, true),
            (Some(false), true, true, false),
            (None, true, false, false),
            (None, true, true, true),
        ] {
            g.gd[gd].script = if has_script { script(flinch) } else { None };
            g.ch[g.gd[gd].ch].team = if operative { crate::teams::Team::Ours } else { crate::teams::Team::Hostile };
            assert_eq!(free(&mut g, gd), want, "{flinch:?} {has_script} {operative}");
        }
    }

    /// With the run's gates down nobody keeps a count, and a man with none is passed over.
    #[test]
    fn the_tick_with_the_run_off_clears_the_count() {
        let (mut g, gd) = one_man();
        let other = g.gd.push(crate::game::Guard::blank());
        g.guards.push(other);
        g.k.tune.suppression = true; g.ai_enabled = false;
        g.m.flinch.state_of.set(gd, FlinchModStateOfValue { under: 1.5, at: 0.0 });
        flinch_tick(&mut g, 0.016);
        assert!(g.m.flinch.state_of.get(gd).is_none());
    }

    /// The page's least travel is inclusive.
    #[test]
    fn inside_counts_from_the_least_travel() {
        let mut g = Box::new(Game::blank());
        let least = g.k.flinch.min_travel;
        assert!(flinch_inside(&mut g, least));
        assert!(!flinch_inside(&mut g, least - 0.001));
    }
}
