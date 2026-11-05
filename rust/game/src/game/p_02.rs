//! src/game/game.ts lines 300 to 588
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The suppression scalar, 0..1. (game.ts Player.suppression)
pub fn player_suppression(g: &mut crate::game::Game, id: PlayerId) -> f64 {
    g.pl[id].sup.v
}

/// The selector open as a list, not up as a ring. (game.ts Player.listOpen)
pub fn player_list_open(g: &mut crate::game::Game, id: PlayerId) -> bool {
    let sel = &g.pl[id].sel;
    sel.open && sel.ring.is_none()
}

/// The selector open with his hand on it: a list, or a ring whose key is still down. (game.ts Player.choosing)
pub fn player_choosing(g: &mut crate::game::Game, id: PlayerId) -> bool {
    let sel = &g.pl[id].sel;
    sel.open && !sel.ring.as_ref().and_then(|r| r.free).unwrap_or(false)
}

/// Makes a player: his guns first (the field initialisers), then his body. (game.ts Player.constructor)
pub fn player_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, pos: V3, has_rifle: Option<bool>, seat: Option<f64>) -> PlayerId {
    let has_rifle = has_rifle.unwrap_or(true);
    let seat = seat.unwrap_or(0.0);
    let pistol = seam_pistol_new(g);
    let rifle = seam_rifle_new(g);
    let frags = g.k.frag.issue;
    let desc = crate::character::CharacterDesc {
        id: crate::seats::seat_man_id(seat),
        tint: [0.05, 0.06, 0.07],
        tint2: if js::truthy(seat) { [0.1, 0.2, 0.36] } else { [0.1, 0.32, 0.18] },
        pos,
        yaw: m::PI,
        men: Some(men),
    };
    let c = crate::character::character_new(g, ch, desc);
    let mut p = crate::game::Player::new(c, has_rifle, pistol, rifle);
    p.frags = frags;
    let id = g.pl.push(p);
    seam_same_man(g, id, c);
    let def = g.fa[pistol].def;
    g.ch[c].muzzle_ahead = g.m.weapons.weapons.get(def).unwrap().muzzle_ahead;
    id
}

/// `new Pistol()`: the fiveseven row. pistol_new (weapons.ts:511) takes no `g` and returns Opaque.
fn seam_pistol_new(g: &mut crate::game::Game) -> FirearmId {
    todo!("needs: pistol_new(g, def) -> FirearmId (game.ts:346)")
}

/// `new Rifle()`: the fn2000 row. rifle_new (weapons.ts:531) takes no `g` and returns Opaque.
fn seam_rifle_new(g: &mut crate::game::Game) -> FirearmId {
    todo!("needs: rifle_new(g, def) -> FirearmId (game.ts:351)")
}

/// `sameMan(this, this.char)`: a Player and a Character hold no row or ManCols to share, and ManCols has no fields.
fn seam_same_man(g: &mut crate::game::Game, id: PlayerId, c: CharId) {
    todo!("needs: where a Player's men row lives, or sameMan out of the game (game.ts:456)")
}

/// The firearm the kit has selected. (game.ts Player.firearm)
pub fn player_firearm(g: &mut crate::game::Game, id: PlayerId) -> FirearmId {
    if player_rifle_sel(g, id) { g.pl[id].rifle } else { g.pl[id].pistol }
}

/// What is physically in the gun hand right now. (game.ts Player.gunInHand)
pub fn player_gun_in_hand(g: &mut crate::game::Game, id: PlayerId) -> Option<FirearmId> {
    if g.pl[id].holstered {
        None
    } else if g.pl[id].slot == crate::game::SlotKind::Pistol {
        Some(g.pl[id].pistol)
    } else if player_rifle_sel(g, id) && g.pl[id].dragging.is_none() {
        Some(g.pl[id].rifle)
    } else {
        None
    }
}

/// The Five-seveN is the selected item and drawn. (game.ts Player.pistolUp)
pub fn player_pistol_up(g: &mut crate::game::Game, id: PlayerId) -> bool {
    g.pl[id].slot == crate::game::SlotKind::Pistol && !g.pl[id].holstered
}

/// The FN2000 is the selected item and the kit has it. (game.ts Player.rifleSel)
pub fn player_rifle_sel(g: &mut crate::game::Game, id: PlayerId) -> bool {
    g.pl[id].slot == crate::game::SlotKind::Rifle && g.pl[id].has_rifle
}

/// The FN2000 is in both hands. (game.ts Player.rifleUp)
pub fn player_rifle_up(g: &mut crate::game::Game, id: PlayerId) -> bool {
    player_rifle_sel(g, id) && !g.pl[id].holstered && g.pl[id].dragging.is_none() && !g.pl[id].down
}

/// Braced on the foregrip, on his own legs this tick. (game.ts Player.braced)
pub fn player_braced(g: &mut crate::game::Game, id: PlayerId) -> bool {
    let sprinting = g.pl[id].sprinting;
    player_braced_at(g, id, sprinting)
}

/// Braced, asked of legs that are or are not at a run. (game.ts Player.bracedAt)
pub fn player_braced_at(g: &mut crate::game::Game, id: PlayerId, sprinting: bool) -> bool {
    player_rifle_up(g, id) && !sprinting && g.pl[id].kick.is_none()
}

/// The canister the selected slot puts in the fist, None with a gun selected. (game.ts Player.canisterSel)
pub fn player_canister_sel(g: &mut crate::game::Game, id: PlayerId) -> Option<crate::game::ThrowKind> {
    match g.pl[id].slot {
        crate::game::SlotKind::Smoke => Some(crate::game::ThrowKind::Smoke),
        crate::game::SlotKind::Flash => Some(crate::game::ThrowKind::Flash),
        crate::game::SlotKind::Frag => Some(crate::game::ThrowKind::Frag),
        crate::game::SlotKind::Pistol | crate::game::SlotKind::Rifle => None,
    }
}

/// A living friend stands in the lane his gun points down. Geometry only. (game.ts friendBeforeMuzzle)
pub fn friend_before_muzzle(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    let y = crate::character::character_gun_yaw(g, c);
    let at = g.ch[c].pos;
    let team = g.gd[gd].team;
    for &o in &g.guards {
        if o != gd && g.gd[o].state != crate::game::GuardState::Dead && g.gd[o].team == team && in_lane(g.ch[g.gd[o].ch].pos, at, y) {
            return true;
        }
    }
    let pl = g.player;
    if g.pl[pl].down || crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player) {
        return false;
    }
    in_lane(g.ch[g.pl[pl].ch].pos, at, y)
}

/// The `lane` closure of friendBeforeMuzzle.
fn in_lane(p: V3, at: V3, y: f64) -> bool {
    crate::character::in_dip_lane(p[0] - at[0], p[2] - at[2], y)
}

/// Down but breathing: put out cold, not shot. False for a living man and for a corpse. (game.ts isBreathing)
pub fn is_breathing(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].state == crate::game::GuardState::Dead
        && g.gd[gd].down_kind.is_some()
        && g.gd[gd].down_kind != Some(crate::game::DownKind::Shot)
}

/// What a man has personally perceived, as an encounter starts. (game.ts newWitness)
pub fn new_witness() -> crate::game::Witness {
    crate::game::Witness {
        heard: crate::game::WitnessHeard { shots: 0.0, last_shot_t: -1.0, kick: 0.0, bang: 0.0, small: 0.0, small_where: None, last_t: -1.0 },
        peak_awareness: 0.0,
        alerted_by: None,
        alert_t: -1.0,
        saw_act: None,
        saw_body: None,
        called_to_body: None,
        saw_held: None,
        was_held: 0.0,
        dazzled: 0.0,
    }
}

/// A man's interrogation progress, as an encounter starts. (game.ts newTalk)
pub fn new_talk(g: &mut crate::game::Game) -> GameTalkId {
    g.tab.game_talk.push(fresh_talk())
}

/// The object literal of newTalk.
fn fresh_talk() -> crate::game::Talk {
    crate::game::Talk {
        stage: 0.0,
        presses: 0.0,
        said: JsSet::new(),
        said_n: JsMap::new(),
        given: JsSet::new(),
        last_t: -1.0,
        held_since: -1.0,
        abort_t: -1.0,
        last: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_makers_start_their_clocks_at_minus_one() {
        let w = new_witness();
        assert!(w.heard.last_shot_t == -1.0 && w.heard.last_t == -1.0 && w.alert_t == -1.0);
    }

    #[test]
    fn a_new_talk_is_a_row_of_the_game() {
        let mut g = Game::blank();
        let a = new_talk(&mut g);
        let b = new_talk(&mut g);
        assert!(a != b);
        let t = &g.tab.game_talk[b];
        assert!(t.stage == 0.0 && t.last.is_none());
        assert!(t.last_t == -1.0 && t.held_since == -1.0 && t.abort_t == -1.0);
    }
}
