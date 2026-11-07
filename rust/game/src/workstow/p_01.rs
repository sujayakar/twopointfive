//! src/game/workstow.ts lines 1 to 96
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The rule is live: settled on the page (WORKSTOW.on, HANDWORK.on), so always.   (workstow.ts live)
pub fn live(g: &mut crate::game::Game) -> bool { true }

/// A charge is going on by your own hands.   (workstow.ts samJob)
pub fn sam_job(g: &mut crate::game::Game) -> bool {
    crate::doortools::door_tools_sam_job(g).is_some() || crate::wallcharge::wall_charge_sam_job(g).is_some()
}

/// Head of the player's frame: a job begun puts away what is in his hand, a job over brings it back.   (workstow.ts samStowTick)
pub fn sam_stow_tick(g: &mut crate::game::Game, holster: &mut dyn FnMut(&mut crate::game::Game, bool)) {
    let pl = g.player;
    let job = g.puppet.is_none() && sam_job(g);
    let Some(s) = g.pl[pl].work_stow else {
        if !job { return; }
        let mut owed = false;
        if let Some(w) = g.pl[pl].wall.as_mut() { owed = w.drawn; w.drawn = false; }
        let drew = !g.pl[pl].holstered;
        let t = g.time;
        g.pl[pl].work_stow = Some(crate::game::PlayerWorkStow { drew: drew || owed, t, down: !drew });
        if drew {
            holster(g, true);
            crate::audio_iface::game_audio_like_play(g, "equip", None, Some(0.35), None);
            g.stats.workstow.sam_down += 1;
        }
        return;
    };
    if job {
        let c = g.pl[pl].ch;
        if !s.down && g.time > s.t && (!crate::character::anim_kit_busy(g, c) || g.time - s.t > g.k.workstow.down_max) {
            if let Some(w) = g.pl[pl].work_stow.as_mut() { w.down = true; }
        }
        return;
    }
    g.pl[pl].work_stow = None;
    if !s.drew || !g.pl[pl].holstered { return; }
    let p = &g.pl[pl];
    if p.down || p.holding.is_some() || p.takedown.is_some() || p.dragging.is_some() || p.picking.is_some() || p.wanding.is_some() {
        g.stats.workstow.sam_kept += 1;
        return;
    }
    if let Some(w) = g.pl[pl].wall.as_mut() {
        if w.kind != Some(crate::player::WallHoldKind::Stack) {
            w.drawn = true;
            g.stats.workstow.sam_kept += 1;
            return;
        }
    }
    holster(g, false);
    g.pl[pl].equip_t = m::max(g.pl[pl].equip_t, g.k.workstow.sam_up);
    crate::audio_iface::game_audio_like_play(g, "equip", None, Some(0.35), None);
    g.stats.workstow.sam_up += 1;
}

/// The thing is still on its way out of his hand: the job's clock has not started.   (workstow.ts samStowing)
pub fn sam_stowing(g: &mut crate::game::Game) -> bool {
    let pl = g.player;
    match g.pl[pl].work_stow { Some(s) => !s.down, None => false }
}

/// Seconds a teammate's job waits, from the frame he is on it, for his hands to be free of his gun.   (workstow.ts mateStowLead)
pub fn mate_stow_lead(g: &mut crate::game::Game) -> f64 { g.k.workstow.mate_down }

/// Once a frame of a teammate's two-handed job, by its owner: his gun is out of his hands.   (workstow.ts mateStow)
pub fn mate_stow(g: &mut crate::game::Game, gd: GuardId) {
    if g.gd[gd].state == crate::game::GuardState::Dead { return; }
    match g.gd[gd].work_stow.as_mut() {
        Some(s) if s.up_t < 0.0 => s.fed = true,
        _ => {
            g.gd[gd].work_stow = Some(crate::game::GuardWorkStow { up_t: -1.0, fed: true });
            g.stats.workstow.mate_down += 1;
        }
    }
}

/// Once a frame for a man with one: a job no longer fed starts the gun back up; his fire clock is held meanwhile.   (workstow.ts mateStowTick)
pub fn mate_stow_tick(g: &mut crate::game::Game, gd: GuardId) {
    if g.gd[gd].work_stow.is_none() { return; }
    let c = g.gd[gd].ch;
    if g.gd[gd].state == crate::game::GuardState::Dead {
        g.gd[gd].work_stow = None;
        g.ch[c].anim.stowed = false;
        return;
    }
    let (now, mate_up) = (g.time, g.k.workstow.mate_up);
    let s = g.gd[gd].work_stow.as_mut().unwrap();
    if s.up_t < 0.0 && !s.fed {
        s.up_t = now + mate_up;
        g.stats.workstow.mate_up += 1;
    }
    s.fed = false;
    let up_t = s.up_t;
    if up_t >= 0.0 && now >= up_t {
        g.gd[gd].work_stow = None;
        g.ch[c].anim.stowed = false;
        return;
    }
    let hang = stow_hang(g, gd);
    g.ch[c].anim.stowed = hang;
    g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, if up_t < 0.0 { mate_up } else { up_t - now });
}

/// His gun is drawn out of his hands this frame (hung on its sling, or on his thigh).   (workstow.ts stowHang)
pub fn stow_hang(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match g.gd[gd].work_stow {
        Some(s) => g.gd[gd].state != crate::game::GuardState::Dead
            && (s.up_t < 0.0 || s.up_t - g.time > g.k.workstow.mate_up * (1.0 - g.k.workstow.show)),
        None => false,
    }
}

/// A round that leaves a gun this module has out of his hands puts it in them at once.   (workstow.ts stowFired)
pub fn stow_fired(g: &mut crate::game::Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    g.gd[gd].work_stow = None;
    g.ch[c].anim.stowed = false;
    g.stats.workstow.mate_fired += 1;
}

/// The fight has him: the job is dropped and waits, for `fightCap` at most at a stretch.   (workstow.ts stowFight)
pub fn stow_fight(g: &mut crate::game::Game, gd: GuardId, a: Option<crate::workstow::StowFightA>, op: &mut crate::workstow::StowFightOp) -> bool {
    let has = a.map_or(false, |a| a.yield_t >= 0.0)
        || g.time < g.gd[gd].dazzled_until
        || g.time < g.gd[gd].stagger_until
        || g.gd[gd].script.as_ref().map_or(false, |s| s.over.is_some());
    if !has {
        op.stow_off_t = None;
        return false;
    }
    let off = *op.stow_off_t.get_or_insert(g.time);
    g.time - off < g.k.workstow.fight_cap
}

#[cfg(test)]
mod tests {
    use super::*;

    fn man() -> (Box<Game>, GuardId, CharId) {
        let mut g = Box::new(Game::blank());
        let c = g.ch.push(crate::character::Character::blank());
        let mut guard = crate::game::Guard::blank();
        guard.ch = c;
        let gd = g.gd.push(guard);
        (g, gd, c)
    }

    /// The whole beat of a teammate's job: stowed while fed, a clock once it stops, the gun drawn back at `show`, his again at `mateUp`.
    #[test]
    fn a_mates_job_runs_its_beats() {
        let (mut g, gd, c) = man();
        g.time = 10.0;
        g.gd[gd].fire_cd = 0.0;
        mate_stow(&mut g, gd);
        assert_eq!(g.stats.workstow.mate_down, 1);
        mate_stow_tick(&mut g, gd);
        assert!(g.ch[c].anim.stowed);
        assert_eq!(g.gd[gd].work_stow.unwrap().fed, false);
        assert_eq!(g.gd[gd].fire_cd, g.k.workstow.mate_up);

        // fed again: the same stow, no second count
        mate_stow(&mut g, gd);
        assert_eq!(g.stats.workstow.mate_down, 1);
        assert!(g.gd[gd].work_stow.unwrap().fed);
        mate_stow_tick(&mut g, gd);

        // not fed: the clock starts
        let up = g.time + g.k.workstow.mate_up;
        mate_stow_tick(&mut g, gd);
        assert_eq!(g.gd[gd].work_stow.unwrap().up_t, up);
        assert_eq!(g.stats.workstow.mate_up, 1);
        assert!(g.ch[c].anim.stowed);

        // past `show` of the way up it is drawn in his hands, with the fire clock still held
        g.time = up - g.k.workstow.mate_up * (1.0 - g.k.workstow.show);
        g.gd[gd].fire_cd = 0.0;
        mate_stow_tick(&mut g, gd);
        assert!(!g.ch[c].anim.stowed);
        assert_eq!(g.gd[gd].fire_cd, up - g.time);

        // up
        g.time = up;
        mate_stow_tick(&mut g, gd);
        assert!(g.gd[gd].work_stow.is_none());
        assert!(!g.ch[c].anim.stowed);
    }

    /// A job begun during the gun's way back up starts a new stow (and counts one); a dead man's is dropped.
    #[test]
    fn restow_and_death() {
        let (mut g, gd, c) = man();
        g.time = 1.0;
        g.gd[gd].work_stow = Some(crate::game::GuardWorkStow { up_t: 1.2, fed: false });
        mate_stow(&mut g, gd);
        let s = g.gd[gd].work_stow.unwrap();
        assert_eq!((s.up_t, s.fed), (-1.0, true));
        assert_eq!(g.stats.workstow.mate_down, 1);

        g.ch[c].anim.stowed = true;
        g.gd[gd].state = crate::game::GuardState::Dead;
        assert!(!stow_hang(&mut g, gd));
        mate_stow(&mut g, gd);
        assert_eq!(g.stats.workstow.mate_down, 1);
        mate_stow_tick(&mut g, gd);
        assert!(g.gd[gd].work_stow.is_none());
        assert!(!g.ch[c].anim.stowed);
    }

    /// The cap on a stretch of the fight, with the stretch already begun; no fight clears the stretch; a fight with none begun starts one in the op.
    #[test]
    fn the_fight_holds_him_up_to_its_cap() {
        let (mut g, gd, _) = man();
        g.time = 100.0;
        let y = Some(crate::workstow::StowFightA { yield_t: 0.0 });
        let on = |t| crate::workstow::StowFightOp { stow_off_t: Some(t) };
        let cap = g.k.workstow.fight_cap;
        assert!(stow_fight(&mut g, gd, y, &mut on(100.0 - cap + 1.0)));
        assert!(!stow_fight(&mut g, gd, y, &mut on(100.0 - cap)));

        let mut op = on(90.0);
        assert!(!stow_fight(&mut g, gd, None, &mut op));
        assert_eq!(op.stow_off_t, None);
        assert!(!stow_fight(&mut g, gd, Some(crate::workstow::StowFightA { yield_t: -1.0 }), &mut op));
        assert_eq!(op.stow_off_t, None);

        g.gd[gd].dazzled_until = 101.0;
        assert_eq!(stow_fight(&mut g, gd, None, &mut op), 0.0 < cap);
        assert_eq!(op.stow_off_t, Some(100.0));
    }
}
