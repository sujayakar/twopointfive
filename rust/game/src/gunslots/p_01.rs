//! src/game/gunslots.ts lines 1 to 118
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// WEAPONS[w]: every WeaponId has its row.
fn weapon_row(g: &Game, w: crate::weapons::WeaponId) -> &crate::weapons::WeaponDef {
    g.m.weapons.weapons.get(w).unwrap()
}

/// The check's break path is tooling: BREAK is false in the game. (gunslots.ts gunsBroken)
pub fn guns_broken() -> bool {
    BREAK
}

/// hearingCheck: a teammate's quiet report counts, as yours does. GUNSLOTS.on and .heard are settled true. (gunslots.ts quietHeard)
pub fn quiet_heard(g: &mut crate::game::Game) -> bool {
    true
}

/// The pick of that id, or None. (gunslots.ts gunPick)
pub fn gun_pick(id_2: &str) -> Option<crate::gunslots::GunPick> {
    picks().into_iter().find(|p| p.id == id_2)
}

/// What a slot's row steps through, in order; a primary may be left at home. (gunslots.ts gunPicks)
pub fn gun_picks(g: &mut crate::game::Game, slot: crate::gunslots::GunSlot) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for p in picks() {
        if p.slot == slot && [p.can, p.bare].iter().any(|w| w.map_or(false, |w| weapon_row(g, w).enabled)) {
            ids.push(p.id);
        }
    }
    if slot == GunSlot::Primary { ids.push(String::from(NO_PRIMARY)); }
    ids
}

/// The weapons.ts row a pick is on a card with / without the suppressor; the other one where it cannot be had that way or is not enabled. GUNSLOTS.bare is settled true. (gunslots.ts pickRow)
pub fn pick_row(g: &mut crate::game::Game, id_2: &str, suppressed: bool) -> Option<crate::weapons::WeaponId> {
    let p = gun_pick(id_2)?;
    let order = if suppressed { [p.can, p.bare] } else { [p.bare, p.can] };
    order.into_iter().flatten().find(|&w| weapon_row(g, w).enabled)
}

/// The pick's name on its tile: its row's short name. (gunslots.ts pickWord)
pub fn pick_word(g: &mut crate::game::Game, id_2: &str, suppressed: Option<bool>) -> String {
    let suppressed = suppressed.unwrap_or(true);
    if id_2 == NO_PRIMARY { return String::from("none"); }
    match pick_row(g, id_2, suppressed) {
        Some(w) => weapon_row(g, w).short.clone(),
        None => String::from(id_2),
    }
}

/// Which of the card's gun drawings stands for a pick, by its row's class; None: none carried. (gunslots.ts pickIcon)
pub fn pick_icon(g: &mut crate::game::Game, id_2: &str) -> Option<crate::kit::KitWeapon> {
    let w = match pick_row(g, id_2, true) {
        Some(w) => w,
        None => pick_row(g, id_2, false)?,
    };
    Some(match weapon_row(g, w).cls {
        crate::weapons::WeaponClass::Pistol => crate::kit::KitWeapon::Pistol,
        crate::weapons::WeaponClass::Shotgun => crate::kit::KitWeapon::Shotgun,
        _ => crate::kit::KitWeapon::Rifle,
    })
}

/// kit.ts kitStage: he is handed the gun that is not in his hands (None: the pistol alone is his whole kit). GUNSLOTS.on is settled true. (gunslots.ts sideStage)
pub fn side_stage(g: &mut crate::game::Game, gd: GuardId, row: Option<crate::weapons::WeaponId>) {
    g.gd[gd].second = None;
    let Some(row) = row else { return };
    if !weapon_row(g, row).enabled { return; }
    let in_hand = g.gd[gd].weapon;
    if row == g.fa[in_hand].def { return; }
    let def = weapon_row(g, row).clone();
    let gun = crate::weapons::firearm_new(g, &def);
    g.gd[gd].second = Some(crate::game::GuardSecond { gun, id: row, ahead: def.muzzle_ahead, out: false, t: -100.0, lock: -100.0, shots: 0.0 });
    g.stats.gunslots.staged += 1;
}

/// The gun in his hands and the one that is not change places; the row is dealt off his route every frame, so the route is written. (gunslots.ts swap)
pub fn swap(g: &mut crate::game::Game, gd: GuardId) {
    let s = g.gd[gd].second.unwrap();
    let was = g.gd[gd].weapon;
    let c = g.gd[gd].ch;
    let ahead = g.ch[c].muzzle_ahead;
    let route = g.gd[gd].route;
    g.tab.route[route].weapon = Some(s.id);
    g.gd[gd].weapon = s.gun;
    g.gd[gd].reload_t = -1.0;
    g.ch[c].muzzle_ahead = s.ahead;
    let was_id = g.fa[was].def;
    let s = g.gd[gd].second.as_mut().unwrap();
    s.gun = was;
    s.id = was_id;
    s.ahead = ahead;
}

/// … on the fight AI's path: his man is where he last had him, while he is on him. (gunslots.ts sideOutAlert)
pub fn side_out_alert(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let k = g.gd[gd].last_known;
    if g.gd[gd].second.is_none() || g.gd[gd].state != crate::game::GuardState::Alert { return false; }
    let Some(k) = k else { return false };
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    side_out(g, gd, m::hyp2(k[0] - p[0], k[2] - p[2]))
}

/// DRY ON A MAN: the sidearm comes out instead of the reload; true, and no round goes for `out` seconds. GUNSLOTS.on and .dry are settled true. (gunslots.ts sideOut)
pub fn side_out(g: &mut crate::game::Game, gd: GuardId, dist: f64) -> bool {
    let Some(s) = g.gd[gd].second else { return false };
    if s.out || !js::truthy(crate::weapons::firearm_rounds_ready(g, s.gun)) { return false; }
    if g.gd[gd].slung.as_ref().map_or(false, |l| l.out) || g.gd[gd].big_out.is_some() || g.gd[gd].state == crate::game::GuardState::Dead || !(dist <= g.k.gunslots.near) { return false; }
    swap(g, gd);
    let (now, out) = (g.time, g.k.gunslots.out);
    let w = g.gd[gd].weapon;
    let shots = g.fa[w].shots_fired;
    let s = g.gd[gd].second.as_mut().unwrap();
    s.out = true;
    s.t = now;
    s.lock = now + out;
    s.shots = shots;
    g.stats.gunslots.out += 1;
    g.gd[gd].fire_cd = 0.0;
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    crate::audio_iface::game_audio_like_play(g, "equip", Some(pos), Some(0.45), None);
    true
}

/// He has a man in front of him this frame: the sidearm stays out. (gunslots.ts sideSeen)
pub fn side_seen(g: &mut crate::game::Game, gd: GuardId) {
    let now = g.time;
    if let Some(s) = g.gd[gd].second.as_mut() {
        if s.out { s.t = now; }
    }
}

/// His hands are between guns, or off his gun for a two-handed job: no round. (gunslots.ts sideLocked)
pub fn side_locked(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].second.map_or(false, |s| g.time < s.lock) || g.gd[gd].work_stow.is_some()
}

/// The long gun he let go of hangs down his front: its rig, while the sidearm is out. (gunslots.ts sideHung)
pub fn side_hung(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::gunslots::SideHung> {
    let s = g.gd[gd].second?;
    if !s.out || g.gd[gd].state == crate::game::GuardState::Dead { return None; }
    let def = g.fa[s.gun].def;
    match weapon_row(g, def).rig {
        crate::weapons::WeaponDefRig::Carbine => Some(SideHung::Carbine),
        crate::weapons::WeaponDefRig::Sc20k => Some(SideHung::Sc20k),
        _ => None,
    }
}

/// Every frame, each man: nobody in front of him for `lull` seconds and the sidearm goes back, the primary up reloaded, his hands busy for its reload and `back` more. (gunslots.ts sideTick)
pub fn side_tick(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) {
    let dt = dt.unwrap_or(0.0);
    let Some(s) = g.gd[gd].second else { return };
    if g.gd[gd].state == crate::game::GuardState::Dead { return; }
    if !s.out {
        if dt > 0.0 && g.time < s.lock {
            let w = g.gd[gd].weapon;
            let def = weapon_row(g, g.fa[w].def).clone();
            let now = g.time - (s.lock - def.reload.dur - g.k.gunslots.back);
            let c = g.gd[gd].ch;
            crate::gunreload::feed_tick(g, c, &def, now - dt, now, None, None);
        }
        return;
    }
    if g.gd[gd].slung.as_ref().map_or(false, |l| l.out) { return; }
    let now = g.time;
    let w = g.gd[gd].weapon;
    let fired = g.fa[w].shots_fired;
    if fired != s.shots || (g.gd[gd].state == crate::game::GuardState::Alert && now - g.gd[gd].last_seen_t < SEEN_SECS) {
        let s = g.gd[gd].second.as_mut().unwrap();
        s.shots = fired;
        s.t = now;
    }
    let s = g.gd[gd].second.unwrap();
    if now - s.t < g.k.gunslots.lull || now < s.lock { return; }
    swap(g, gd);
    g.gd[gd].second.as_mut().unwrap().out = false;
    let w = g.gd[gd].weapon;
    crate::weapons::firearm_rearm(g, w);
    let side = g.gd[gd].second.unwrap().gun;
    crate::weapons::firearm_rearm(g, side);
    let w = g.gd[gd].weapon;
    let def = weapon_row(g, g.fa[w].def).clone();
    let row = def.reload;
    let lock = g.time + row.dur + g.k.gunslots.back;
    g.gd[gd].second.as_mut().unwrap().lock = lock;
    g.stats.gunslots.back += 1;
    let f = crate::gunreload::feed_of(g, &def);
    let c = g.gd[gd].ch;
    let long = if def.cls == crate::weapons::WeaponClass::Pistol { None } else { Some(w) };
    crate::gunreload::feed_deal(g, c, long, Some(true));
    let beat = f.as_ref().map(|_| crate::character::ReloadBeat { dur: row.dur, out: row.out, in_: row.in_, rack: row.rack, keep: None, hand: None, exact: None });
    crate::character::anim_reload(g, c, Some(row.dur), beat);
    let pos = g.ch[c].pos;
    crate::audio_iface::game_audio_like_play(g, "equip", Some(pos), Some(0.45), None);
    if f.as_ref().map_or(true, |f| f.kind == "mag") {
        let pos = g.ch[c].pos;
        crate::audio_iface::game_audio_like_play(g, "magOut", Some(pos), Some(0.5), None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::WeaponId as W;

    /// One man with a pair of guns, on a blank game, the page's knobs and rows. Nothing here reaches a stub.
    fn stand(in_hand: W, second: W) -> (Box<Game>, GuardId, FirearmId, FirearmId) {
        let mut g = Box::new(Game::blank());
        let a = g.fa.push(crate::weapons::Firearm { def: in_hand, ..crate::weapons::Firearm::blank() });
        let b = g.fa.push(crate::weapons::Firearm { def: second, ..crate::weapons::Firearm::blank() });
        let route = g.tab.route.push(crate::scene_level::PatrolRoute::zeroed());
        let c = g.ch.push(crate::character::Character::blank());
        g.ch[c].muzzle_ahead = 0.5;
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        man.route = route;
        man.weapon = a;
        man.state = crate::game::GuardState::Patrol;
        man.second = Some(crate::game::GuardSecond { gun: b, id: second, ahead: 0.25, out: false, t: -100.0, lock: -100.0, shots: 0.0 });
        let gd = g.gd.push(man);
        (g, gd, a, b)
    }

    #[test]
    fn a_pick_is_found_by_its_id_alone() {
        assert!(gun_pick(NO_PRIMARY).is_none());
        let p = gun_pick("g19").unwrap();
        assert_eq!(p.slot, GunSlot::Secondary);
        assert_eq!(p.can, Some(W::Fiveseven));
        assert_eq!(p.bare, Some(W::G19bare));
    }

    #[test]
    fn a_pick_falls_back_to_the_way_it_can_be_had() {
        let (mut g, ..) = stand(W::Mk12, W::Hk45);
        assert_eq!(pick_row(&mut g, "mk18", true), Some(W::Fn2000));
        assert_eq!(pick_row(&mut g, "mk18", false), Some(W::Mk18bare));
        assert_eq!(pick_row(&mut g, "mk46", true), Some(W::Mk46));
        assert_eq!(pick_row(&mut g, "mk12", false), Some(W::Mk12));
        assert_eq!(pick_row(&mut g, NO_PRIMARY, true), None);
        assert_eq!(pick_icon(&mut g, NO_PRIMARY), None);
        assert_eq!(pick_icon(&mut g, "g18"), Some(crate::kit::KitWeapon::Pistol));
        assert_eq!(pick_icon(&mut g, "m1014"), Some(crate::kit::KitWeapon::Shotgun));
        assert_eq!(pick_icon(&mut g, "mk46"), Some(crate::kit::KitWeapon::Rifle));
        assert_eq!(pick_word(&mut g, NO_PRIMARY, None), "none");
    }

    #[test]
    fn the_row_of_a_primary_ends_in_none() {
        let (mut g, ..) = stand(W::Mk12, W::Hk45);
        let ids = gun_picks(&mut g, GunSlot::Primary);
        assert_eq!(ids.last().map(String::as_str), Some(NO_PRIMARY));
        assert!(!gun_picks(&mut g, GunSlot::Secondary).iter().any(|i| i == NO_PRIMARY));
    }

    #[test]
    fn a_swap_trades_guns_and_each_keeps_its_own_state() {
        let (mut g, gd, a, b) = stand(W::Mk12, W::Hk45);
        g.gd[gd].reload_t = 1.0;
        swap(&mut g, gd);
        let s = g.gd[gd].second.unwrap();
        assert_eq!(g.gd[gd].weapon, b);
        assert_eq!((s.gun, s.id, s.ahead), (a, W::Mk12, 0.5));
        let (c, route) = (g.gd[gd].ch, g.gd[gd].route);
        assert_eq!(g.ch[c].muzzle_ahead, 0.25);
        assert_eq!(g.tab.route[route].weapon, Some(W::Hk45));
        assert_eq!(g.gd[gd].reload_t, -1.0);
    }

    #[test]
    fn the_sidearm_stays_out_through_the_lull_and_a_round_restarts_it() {
        let (mut g, gd, a, b) = stand(W::Hk45, W::Mk12);
        g.time = 10.0;
        let s = g.gd[gd].second.as_mut().unwrap();
        s.out = true;
        s.t = 9.0;
        s.lock = 0.0;
        s.shots = 2.0;
        g.fa[a].shots_fired = 2.0;
        side_tick(&mut g, gd, Some(0.016));
        assert_eq!(g.gd[gd].weapon, a);
        assert_eq!(g.gd[gd].second.unwrap().t, 9.0);
        g.fa[a].shots_fired = 3.0;
        side_tick(&mut g, gd, Some(0.016));
        let s = g.gd[gd].second.unwrap();
        assert_eq!((s.t, s.shots, s.gun), (10.0, 3.0, b));
    }

    #[test]
    fn he_is_locked_while_his_hands_are_between_guns() {
        let (mut g, gd, ..) = stand(W::Mk12, W::Hk45);
        g.time = 1.0;
        assert!(!side_locked(&mut g, gd));
        g.gd[gd].second.as_mut().unwrap().lock = 2.0;
        assert!(side_locked(&mut g, gd));
    }
}
