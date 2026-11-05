//! src/game/friendlyfire.ts lines 1 to 116
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Is the rule running: nobody's show on the stage; a show's rounds are its script's. (friendlyfire.ts ffLive)
pub fn ff_live(g: &mut crate::game::Game) -> bool {
    !g.touring && !crate::director::director_showing(g) && !g.puppet.is_some()
}

/// A man of Sam's own side whose body a round of that side can find: living, on his feet, not a show's cast. (friendlyfire.ts ffBody)
pub fn ff_body(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    if !(g.gd[gd].state != crate::game::GuardState::Dead && g.ch[c].alive && !g.gd[gd].god && !g.gd[gd].civ.is_some()) {
        return false;
    }
    let team = g.gd[gd].team;
    !crate::teams::is_hostile(g, crate::teams::Team::Player, team)
}

/// Is this shooter's side Sam's: an operative, never the night staff or a defender. (friendlyfire.ts ffSide)
pub fn ff_side(g: &mut crate::game::Game, sg: Option<GuardId>) -> bool {
    let Some(sg) = sg else { return false };
    if g.gd[sg].civ.is_some() {
        return false;
    }
    let team = g.gd[sg].team;
    !crate::teams::is_hostile(g, team, crate::teams::Team::Player) && !crate::teams::is_hostile(g, crate::teams::Team::Player, team)
}

/// THE CARE: the friend nearest the muzzle that a round from `muzzle` at `to`, thrown as wide as `spread` can throw it, could reach. (friendlyfire.ts friendInCone)
pub fn friend_in_cone(g: &mut crate::game::Game, self_: GuardId, muzzle: V3, to: V3, spread: f64, def: Option<&crate::weapons::WeaponDef>, on: Option<CharId>) -> Option<crate::friendlyfire::FriendInCone> {
    if !ff_live(g) || !ff_side(g, Some(self_)) {
        return None;
    }
    let mut dx = to[0] - muzzle[0];
    let mut dz = to[2] - muzzle[2];
    let l = m::hyp2(dx, dz);
    if l < 0.05 {
        return None;
    }
    dx /= l;
    dz /= l;
    let rise = (to[1] - muzzle[1]) / l;
    let k = g.k.friendlyfire.cone_k * m::max(0.0, spread);
    let own_def; // his own gun's row, copied out of g: the calls below take g
    let w: &crate::weapons::WeaponDef = match def {
        Some(d) => d,
        None => {
            own_def = g.m.weapons.weapons.get(g.fa[g.gd[self_].weapon].def).unwrap().clone();
            &own_def
        }
    };
    let self_ch = g.gd[self_].ch;
    let energy = crate::glasspen::shot_energy(g, self_ch, Some(w));
    let thin = if crate::wallpen::wall_pen_live(g) { crate::glasspen::round_energy(g, w) } else { -1.0 };
    let mut past = f64::INFINITY; // how far along the line a friend still counts: all the way, unless the round is sure to stop in the man it is for
    if let Some(on) = on {
        if g.ch[on].alive {
            let p = g.ch[on].pos;
            let ch = g.ch[on].bones.get(crate::character::Bone::Chest).copied().unwrap_or([p[0], p[1] + 1.2, p[2]]);
            if m::hyp3(to[0] - ch[0], to[1] - ch[1], to[2] - ch[2]) < 0.35 && k * l <= g.k.friendlyfire.sure_m {
                past = l + 0.3;
            }
        }
    }
    let mut best: Option<crate::friendlyfire::FriendInCone> = None;
    let mut bt = f64::INFINITY;
    for pl in men_of(g) {
        if !g.pl[pl].down {
            let c = g.pl[pl].ch;
            ask(g, c, crate::friendlyfire::FriendInCone::Player, muzzle, dx, dz, rise, k, past, energy, thin, &mut best, &mut bt);
        }
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o != self_ && ff_body(g, o) {
            let c = g.gd[o].ch;
            ask(g, c, crate::friendlyfire::FriendInCone::Guard(o), muzzle, dx, dz, rise, k, past, energy, thin, &mut best, &mut bt);
        }
    }
    best
}

/// The closure `ask` of friendInCone: is this man nearer along the line than the best so far, in the cone and not behind anything solid. (friendlyfire.ts friendInCone)
#[allow(clippy::too_many_arguments)]
fn ask(g: &mut Game, c: CharId, who: crate::friendlyfire::FriendInCone, muzzle: V3, dx: f64, dz: f64, rise: f64, k: f64, past: f64, energy: Option<f64>, thin: f64, best: &mut Option<crate::friendlyfire::FriendInCone>, bt: &mut f64) {
    let pos = g.ch[c].pos;
    let hips = g.ch[c].bones.get(crate::character::Bone::Hips).copied().unwrap_or(pos);
    let chest_bone = g.ch[c].bones.get(crate::character::Bone::Chest).copied();
    let chest = chest_bone.unwrap_or(hips);
    let mx = hips[0] - muzzle[0];
    let mz = hips[2] - muzzle[2];
    if mx * mx + mz * mz < 0.28 * 0.28 {
        return; // the muzzle is already inside his column: past him
    }
    let body_m = g.k.friendlyfire.body_m;
    let mut t = f64::INFINITY; // how far along the line his hips or his chest come inside the cone
    for p in [hips, chest] {
        let rx = p[0] - muzzle[0];
        let rz = p[2] - muzzle[2];
        let a = rx * dx + rz * dz;
        if a > 0.0 && a < t && m::abs(rx * dz - rz * dx) <= body_m + a * k {
            t = a;
        }
    }
    if t >= *bt || t > 60.0 || t > past {
        return;
    }
    let y = muzzle[1] + rise * t;
    let top = (match g.ch[c].bones.get(crate::character::Bone::HeadTop) { Some(h) => h[1], None => pos[1] + 1.8 }) + 0.1;
    if y - t * k > top || y + t * k < pos[1] {
        return;
    }
    let cy = if chest_bone.is_some() { chest[1] } else { pos[1] + 1.2 };
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied().unwrap_or([pos[0], pos[1] + 1.6, pos[2]]);
    // something solid between them that his round does not go through: asked of his middle, his head and the two edges of his column
    if !reaches(g, muzzle, [chest[0], cy, chest[2]], energy, thin)
        && !reaches(g, muzzle, head, energy, thin)
        && !reaches(g, muzzle, [hips[0] + dz * 0.25, cy, hips[2] - dx * 0.25], energy, thin)
        && !reaches(g, muzzle, [hips[0] - dz * 0.25, cy, hips[2] + dx * 0.25], energy, thin)
    {
        return;
    }
    *bt = t;
    *best = Some(who);
}

/// The closure `reaches` of friendInCone: level geometry only, less what the round goes through. (friendlyfire.ts friendInCone)
fn reaches(g: &mut Game, muzzle: V3, p: V3, energy: Option<f64>, thin: f64) -> bool {
    !crate::scene_collision::shot_blocked(g, muzzle, p, Some(0.02), energy) || (thin >= 0.0 && crate::wallpen::thin_line(g, muzzle, p, thin, None, None).is_some())
}

/// A teammate's round about to be born with a friend in its cone: not pressed. (friendlyfire.ts ffUnpressed)
pub fn ff_unpressed(g: &mut crate::game::Game, sg: Option<GuardId>, muzzle: V3, to: V3, spread: f64, def: &crate::weapons::WeaponDef) -> bool {
    let Some(sg) = sg else { return false };
    let b = def.pellets.as_ref().and_then(|p| p.breach);
    if let Some(b) = b {
        if m::hyp3(to[0] - muzzle[0], to[1] - muzzle[1], to[2] - muzzle[2]) <= b.reach_m + 0.2 {
            return false; // a shell put into a lock at arm's length is the door tool's
        }
    }
    let foe = crate::teams::foe_char(g, sg);
    if friend_in_cone(g, sg, muzzle, to, spread, Some(def), foe).is_none() {
        return false;
    }
    g.stats.friendlyfire.unpressed += 1;
    true
}

/// The check's and the measure's environment word; never set on the page. (friendlyfire.ts ffBroken)
pub fn ff_broken() -> bool {
    false
}

/// The PLAYER's round flown from `from` to where it stopped: every standing teammate it went close past says so. (friendlyfire.ts ffPast)
pub fn ff_past(g: &mut crate::game::Game, from: V3, to: V3, into: Option<CharId>) {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let dz = to[2] - from[2];
    let l2 = dx * dx + dy * dy + dz * dz;
    if l2 < 1.0 {
        return;
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        let c = g.gd[o].ch;
        if !ff_body(g, o) || Some(c) == into || g.gd[o].held.is_some() {
            continue;
        }
        let near_m = g.k.friendlyfire.near_m;
        let bones = [g.ch[c].bones.get(crate::character::Bone::Head).copied(), g.ch[c].bones.get(crate::character::Bone::Chest).copied()];
        for p in bones {
            let Some(p) = p else { continue };
            let t = ((p[0] - from[0]) * dx + (p[1] - from[1]) * dy + (p[2] - from[2]) * dz) / l2;
            if t < 0.02 || t > 1.0 {
                continue; // beside or behind the muzzle: not past him
            }
            if m::hyp3(from[0] + dx * t - p[0], from[1] + dy * t - p[1], from[2] + dz * t - p[2]) > near_m {
                continue;
            }
            if crate::suppression::watch_your_fire(g, o) {
                g.stats.friendlyfire.said += 1;
            }
            break;
        }
    }
}

/// The room fight's gate held a round for a friend in the cone that its own lane did not hold. (friendlyfire.ts ffHeld)
pub fn ff_held(g: &mut crate::game::Game) {
    g.stats.friendlyfire.held += 1;
}

/// A round of Sam's side has gone into a man of Sam's side: counted. (friendlyfire.ts ffNote)
pub fn ff_note(g: &mut crate::game::Game, sg: Option<GuardId>, is_player: bool, into_sam: bool) {
    let s = &mut g.stats.friendlyfire;
    if is_player {
        s.sam_on_mate += 1;
    } else if into_sam {
        s.mate_on_sam += 1;
    } else if sg.is_some() {
        s.mate_on_mate += 1;
    }
}

/// `men(g)`: the stub also asks for the `Pick<Game, 'seats' | 'player'>` as a value, filled here from `g`.
fn men_of(g: &mut Game) -> Vec<PlayerId> {
    let view = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    crate::seats::men(g, &view)
}
