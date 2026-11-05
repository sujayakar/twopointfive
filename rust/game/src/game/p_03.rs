//! src/game/game.ts lines 588 to 974
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Stamp fix_t if the fix has moved since last looked at; called once a frame by update_guard. (game.ts Guard.trackFix)
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

/// A man: his record, his body facing down the first leg, his keys, his gun. Field initialisers first (the look phase's draw comes BEFORE the body's own). (game.ts Guard.constructor)
pub fn guard_new(g: &mut crate::game::Game, men: crate::Opaque, ch: RigId, id_2: f64, route: RouteId, route_i: i32, flashlight: LightId, tint: V3, look: Option<&crate::game::GuardNewLook>) -> GuardId {
    let id = g.gd.push(Guard::blank());
    g.gd[id].witness = new_witness();
    g.gd[id].talk = new_talk(g);
    g.gd[id].look_phase = crate::math_dice::sim_random(g) * 10.0;

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
    if let Some(team) = look.and_then(|l| l.team) { g.gd[id].team = team; }
    let team = g.gd[id].team;
    if team != crate::teams::Team::Guard && team != crate::teams::Team::Hostile { g.gd[id].foe = None; }
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

/// The snow's field and what the wind and the dust hold are handed to the GPU whole again next frame. Host's call (putback.ts). (game.ts Game.rehand)
pub fn game_rehand(g: &mut crate::game::Game) {
    g.snow_look_was = -1.0;
    crate::air_sim::air_resend(g);
    crate::air_dust::air_dust_resend(g);
}

/// PICTURE ONLY, not translated: the snow's field and prints follow their switches for the g-buffer. Empty, as nothing in a rule reads them and the game's constructor calls this unguarded (game.ts:1102). (game.ts Game.syncSnowLook)
pub fn game_sync_snow_look(g: &mut crate::game::Game) {}

/// PICTURE ONLY, not translated: this frame's footfalls pressed into the snow for the g-buffer. Empty, as nothing in a rule reads them. (game.ts Game.stepSnowTracks)
pub fn game_step_snow_tracks(g: &mut crate::game::Game, dt: f64) {}

/// Asks the renderer's irradiance at a point under `key`. (game.ts Game.lightQuery; the tools' swap of this field is not carried)
pub fn game_light_query(g: &mut crate::game::Game, key: &str, pos: V3, skip_owner: Option<f64>) {
    let skip_owner = skip_owner.unwrap_or(0.0);
    crate::sensors::sensors_ask(g, key, pos, Some(skip_owner));
}

/// The latest answer for `key`, or None while there is none. (game.ts Game.lightAt)
pub fn game_light_at(g: &mut crate::game::Game, key: &str) -> Option<f64> {
    crate::sensors::sensors_read(g, key)
}

/// Drops a stale answer for `key`. (game.ts Game.lightForget)
pub fn game_light_forget(g: &mut crate::game::Game, key: &str) {
    crate::sensors::sensors_forget(g, key);
}

/// Which side each door opens into, as the deals stand now. (game.ts Game.doorSwings)
pub fn game_door_swings(g: &mut crate::game::Game) -> Vec<crate::doorswing::SwingDeal> {
    let deals = g.door_deals.clone();
    crate::doorswing::swings_now(g, &deals)
}

/// How long a man of theirs has been at the panel, read. (game.ts Game.breakerFixT)
pub fn game_breaker_fix_t(g: &mut crate::game::Game) -> f64 {
    crate::sim_lightshome::mains_fix_t(g)
}
