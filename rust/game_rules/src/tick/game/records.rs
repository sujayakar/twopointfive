#![allow(unused_variables)]
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// INVENTED: THE whole write of a man's path. Every `gd.path = x` of the TypeScript (a new array, even `[]`) is this call: identity readers compare path_serial. An in-place edit (splice, push, index, `length = n`) is not a whole write and does not call it. ONE exception: runner.ts:294 puts the same array back, so it restores path and path_serial together and does not call this
pub fn guard_set_path(g: &mut crate::game::Game, gd: GuardId, path: Vec<V3>) { let o = &mut g.gd[gd]; o.path = path; o.path_serial = o.path_serial.wrapping_add(1); }

/// A game stands in this slot of rust/sim's.
pub fn stand_in(g: &mut crate::game::Game, slot: usize) { g.sim = slot; g.tab.boxes.stand_in(slot); }

/// A man easing past furniture: a player crouched and not dashing; anyone else low.
pub(super) fn quiet_man(g: &crate::game::Game, pl: Option<PlayerId>, c: CharId) -> bool {
    match pl {
        Some(p) => g.pl[p].crouch && !g.pl[p].dash,
        None => g.ch[c].anim.crouch > 0.5,
    }
}

/// `m:ss` of a count of seconds.
pub fn fmt_clock(sec: f64) -> String {
    let s = m::max(0.0, m::floor(sec));
    let tail = js::num_str(js::rem(s, 60.0));
    let pad = if tail.len() < 2 { "0" } else { "" };
    alloc::format!("{}:{}{}", js::num_str(m::floor(s / 60.0)), pad, tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The debrief's clock.
    #[test]
    fn a_clock_reads_whole_minutes_and_seconds_and_nothing_under_nought() {
        let cases: &[(f64, &str)] = &[(0.0, "0:00"), (-5.0, "0:00"), (0.5, "0:00"), (9.99, "0:09"), (59.9, "0:59"), (60.0, "1:00"), (61.0, "1:01"), (3725.5, "62:05")];
        for (x, want) in cases {
            assert_eq!(fmt_clock(*x), *want, "fmt_clock({})", x);
        }
    }
}

/// The selector open with his hand on it: a list, or a ring whose key is still down.
pub fn player_choosing(g: &crate::game::Game, id: PlayerId) -> bool {
    let sel = &g.sets[crate::seats::seat_of_man(g, id)].hand.sel;
    sel.open && !sel.ring.as_ref().and_then(|r| r.free).unwrap_or(false)
}

/// Makes a player: his guns first (the field initialisers), then his body.
pub(in crate::tick) fn player_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, pos: V3, has_rifle: Option<bool>, seat: Option<f64>) -> PlayerId {
    let has_rifle = has_rifle.unwrap_or(true);
    let seat = seat.unwrap_or(0.0);
    let pistol = crate::weapons::pistol_new(g, None);
    let rifle = crate::weapons::rifle_new(g, None);
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
    g.ch[c].who = crate::character::Who::Player(id);
    g.ch[c].team = crate::teams::Team::Ours;
    let def = g.fa[pistol].def;
    g.ch[c].muzzle_ahead = g.m.weapons.weapons.get(def).unwrap().muzzle_ahead;
    id
}

/// What is physically in the gun hand right now.
pub fn player_gun_in_hand(g: &crate::game::Game, id: PlayerId) -> Option<FirearmId> {
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

/// The Five-seveN is the selected item and drawn.
pub fn player_pistol_up(g: &crate::game::Game, id: PlayerId) -> bool {
    g.pl[id].slot == crate::game::SlotKind::Pistol && !g.pl[id].holstered
}

/// The FN2000 is the selected item and the kit has it.
pub fn player_rifle_sel(g: &crate::game::Game, id: PlayerId) -> bool {
    g.pl[id].slot == crate::game::SlotKind::Rifle && g.pl[id].has_rifle
}

/// The FN2000 is in both hands.
pub fn player_rifle_up(g: &crate::game::Game, id: PlayerId) -> bool {
    player_rifle_sel(g, id) && !g.pl[id].holstered && g.pl[id].dragging.is_none() && !crate::condition::down(g, g.pl[id].ch)
}

/// Braced on the foregrip, on his own legs this tick.
pub fn player_braced(g: &crate::game::Game, id: PlayerId) -> bool {
    let sprinting = g.pl[id].sprinting;
    player_braced_at(g, id, sprinting)
}

/// Braced, asked of legs that are or are not at a run.
pub fn player_braced_at(g: &crate::game::Game, id: PlayerId, sprinting: bool) -> bool {
    player_rifle_up(g, id) && !sprinting && g.pl[id].kick.is_none()
}

/// The canister the selected slot puts in the fist, None with a gun selected.
pub fn player_canister_sel(g: &crate::game::Game, id: PlayerId) -> Option<crate::game::ThrowKind> {
    match g.pl[id].slot {
        crate::game::SlotKind::Smoke => Some(crate::game::ThrowKind::Smoke),
        crate::game::SlotKind::Flash => Some(crate::game::ThrowKind::Flash),
        crate::game::SlotKind::Frag => Some(crate::game::ThrowKind::Frag),
        crate::game::SlotKind::Pistol | crate::game::SlotKind::Rifle => None,
    }
}

/// Down but breathing: put out cold, not shot. False for a living man and for a corpse.
pub fn is_breathing(g: &crate::game::Game, gd: GuardId) -> bool {
    crate::condition::down(g, g.gd[gd].ch)
        && g.gd[gd].down_kind.is_some()
        && g.gd[gd].down_kind != Some(crate::game::DownKind::Shot)
}

/// What a man has personally perceived, as an encounter starts.
fn new_witness() -> crate::game::Witness {
    crate::game::Witness {
        heard: crate::game::WitnessHeard { shots: 0.0, last_shot_t: -1.0, kick: 0.0, bang: 0.0, small: 0.0, small_where: None, loud_at: None, last_t: -1.0 },
        peak_awareness: 0.0,
        alerted_by: None,
        alert_t: -1.0,
        saw_act: None,
        saw_body: None,
        called_to_body: None,
        saw_held: None,
        dazzled: 0.0,
    }
}

/// A man's interrogation progress, as an encounter starts.
fn new_talk(g: &mut crate::game::Game) -> GameTalkId {
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

/// Stamp fix_t if the fix has moved since last looked at; called once a frame by update_guard.
pub fn guard_track_fix(g: &mut crate::game::Game, id: GuardId, now: f64) {
    let lk = match g.gd[id].last_known {
        Some(lk) => lk,
        None => { g.gd[id].fix_ref = None; return; }
    };
    let moved = match g.gd[id].fix_ref {
        None => true,
        Some(r) => m::hyp2(lk[0] - r[0], lk[2] - r[2]) > 0.5,
    };
    if moved { g.gd[id].fix_ref = Some(v3::copy(lk)); g.gd[id].fix_t = now; }
}

/// A man: his record, his body facing down the first leg, his keys, his gun. Field initialisers first (the look phase's draw comes BEFORE the body's own).
pub fn guard_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, id_2: f64, route: RouteId, route_i: i32, flashlight: LightId, tint: V3, look: Option<&crate::game::GuardNewLook>) -> GuardId {
    let id = g.gd.push(Guard::blank());
    g.gd[id].witness = new_witness();
    g.gd[id].talk = new_talk(g);
    g.gd[id].look_phase = crate::dice::draw_world(g, "game::guard_new#1") * 10.0;

    let (p0, p1, weapon, keys) = {
        let r = &g.tab.route[route];
        (r.points[0], r.points[1 % r.points.len()], r.weapon, r.keys.clone())
    };
    let pos = v3::copy(p0);
    let desc = crate::character::CharacterDesc {
        id: id_2,
        tint,
        tint2: look.and_then(|l| l.tint2).unwrap_or([0.02, 0.02, 0.025]),
        pos,
        yaw: match look.and_then(|l| l.yaw) { Some(y) => y, None => v3::yaw_to(pos, p1) },
        men: Some(men),
    };
    let c = crate::character::character_new(g, ch, desc);
    g.gd[id].ch = c;
    g.ch[c].who = crate::character::Who::Guard(id);
    g.arms[c].fire_cd = 1.0;   // (a second before his first round)

    g.gd[id].last_pos = v3::copy(pos);
    g.gd[id].route_i = route_i;
    g.gd[id].route = route;
    g.gd[id].flashlight = flashlight;
    g.gd[id].callsign = match look.and_then(|l| l.name.clone()) {
        Some(name) => name,
        None => {
            let cs = callsigns();
            cs[js::rem(m::max(0.0, id_2 - 10.0), cs.len() as f64) as usize].clone()
        }
    };
    if let Some(team) = look.and_then(|l| l.team) { g.ch[c].team = team; }
    let team = g.team(id);
    // (an enemy of the player is born with the first of them in mind)
    g.gd[id].foe = if team.theirs() { Some(g.pl[g.player].ch) } else { None };
    let keys = match keys {
        Some(k) => k,
        None => vec![String::from(crate::doors::FLOOR_KEY)],
    };
    let mut held: JsSet<String> = JsSet::new();
    for k in keys { held.add(k); }
    g.gd[id].keys = held;
    // a row the table does not hold panics here, as the TypeScript's throws
    let row = g.m.weapons.weapons.get(weapon.unwrap_or(crate::weapons::WeaponId::Guard9mm)).unwrap().clone();
    let w = crate::weapons::firearm_new(g, &row);
    g.gd[id].weapon = w;
    g.ch[c].muzzle_ahead = row.muzzle_ahead;
    id
}
