//! src/game/roomfight.ts lines 1 to 433
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Hard's drilled defenders: only the night breach by itself, under the press's own switch.   (roomfight.ts drilledOn)
pub fn drilled_on(g: &mut crate::game::Game) -> bool {
    g.k.pressure.on && g.k.pressure.drilled && g.night_breach.active && g.night_breach.layout.is_none()
}

/// × the seconds a stun canister doubles this man over: DRILLED.shaken for a drilled defender, 1 for anybody else.   (roomfight.ts drilledK)
pub fn drilled_k(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    if g.gd[gd].team == crate::teams::Team::Hostile && drilled_on(g) { g.k.drilled.shaken } else { 1.0 }
}

/// Distance on the floor.   (roomfight.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// A draw of the simulation's die in a range.   (roomfight.ts pick)
pub fn pick(g: &mut crate::game::Game, r: [f64; 2]) -> f64 {
    r[0] + crate::math_dice::sim_random(g) * (r[1] - r[0])
}

/// His head bone, or where it would be.   (roomfight.ts eyeOf)
pub fn eye_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    match g.ch[c_2].bones.get(crate::character::Bone::Head) {
        Some(p) => *p,
        None => v3::add(g.ch[c_2].pos, [0.0, 1.65, 0.0]),
    }
}

/// His chest bone, or where it would be.   (roomfight.ts chestOf)
pub fn chest_of(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    match g.ch[c_2].bones.get(crate::character::Bone::Chest) {
        Some(p) => *p,
        None => v3::add(g.ch[c_2].pos, [0.0, 1.1, 0.0]),
    }
}

/// The sensor key of a pair of bodies, whichever way round it is asked.   (roomfight.ts pairKey)
pub fn pair_key(a: f64, b: f64) -> String {
    if a < b { alloc::format!("fightS{}_{}", a, b) } else { alloc::format!("fightS{}_{}", b, a) }
}

/// The smoke between two AI men as last answered (1 = clear air), a picked weather's fog on top.   (roomfight.ts pairSmoke)
pub fn pair_smoke(g: &mut crate::game::Game, a: CharId, b: CharId) -> f64 {
    let key = pair_key(g.ch[a].id, g.ch[b].id);
    let t = match g.fight.trans.get(key) { Some(v) => *v, None => 1.0 };
    if g.weather.fog.is_some() {
        let (ca, cb) = (chest_of(g, a), chest_of(g, b));
        t * crate::weather::air_trans(g, ca, cb)
    } else {
        t
    }
}

/// The light to read for `o` when `eye` stands inside the cone of his burning weapon light, else 0.   (roomfight.ts lampGlare)
pub fn lamp_glare(g: &mut crate::game::Game, eye: CharId, o: GuardId) -> f64 {
    if !g.fight.live { return 0.0; }
    let l = g.gd[o].flashlight;
    let light = &g.tab.light[l];
    if !light.enabled || !(light.intensity > 0.0) || g.gd[o].state == crate::game::GuardState::Dead { return 0.0; }
    let (pos, dir, range, outer_deg) = (light.pos, light.dir, light.range, light.outer_deg);
    let e = eye_of(g, eye);
    let (dx, dy, dz) = (e[0] - pos[0], e[1] - pos[1], e[2] - pos[2]);
    let len = m::hyp3(dx, dy, dz);
    if len > range || len < 1e-3 { return 0.0; }
    if (dx * dir[0] + dy * dir[1] + dz * dir[2]) / len >= m::cos(outer_deg * v3::DEG) { g.k.roomfight.lamp } else { 0.0 }
}

/// Is the room fight running this frame?   (roomfight.ts fightLive)
pub fn fight_live(g: &mut crate::game::Game) -> bool {
    g.fight.live
}

/// The clearance a friend is owed against a line of fire `l` long, he `t` of the way along it.   (roomfight.ts closeLane)
pub fn close_lane(g: &mut crate::game::Game, clear: f64, t: f64, l: f64) -> f64 {
    lane(clear, t, l, g.k.roomfight.point_blank)
}

/// closeLane's sum, with the knob it reads handed in.
fn lane(clear: f64, t: f64, l: f64, point_blank: f64) -> f64 {
    m::min(clear, if t > 1.0 && l < point_blank { 0.3 } else { 0.3 + 0.25 * m::max(0.0, t) * l })
}

/// How fast the player on this body is really getting anywhere; Infinity while not known yet.   (roomfight.ts RoomFight.samSpeedOf)
pub fn room_fight_sam_speed_of(g: &mut crate::game::Game, ch: CharId) -> f64 {
    let mut i = 0;
    while i < g.seats.len() {
        let seat = g.seats[i];
        let man = crate::cmd::seat_man(g, seat);
        if g.pl[man].ch == ch {
            return match g.fight.sams.get(i) {
                Some(t) if !(t.known < 0.25) => m::hyp2(t.v[0], t.v[1]),
                _ => m::INF,
            };
        }
        i += 1;
    }
    m::INF
}

/// A new room fight; `knobs` is the game's own table as it stands, not the module's defaults.   (roomfight.ts RoomFight.constructor)
pub fn room_fight_new(g: &mut crate::game::Game) -> crate::roomfight::RoomFight {
    let mut f = crate::roomfight::RoomFight::blank();
    f.knobs = g.k.roomfight;
    f
}

/// May the module run at all: the AI, sides, and no show holding the floor (a tutorial's own program does not hold it).   (roomfight.ts RoomFight.on)
pub fn room_fight_on(g: &mut crate::game::Game) -> bool {
    if !(g.ai_enabled && g.tune.teams && !g.touring && !g.quiet_utility && g.puppet.is_none()) { return false; }
    let show_runs = match g.director.run { Some(r) => !g.tab.director_run[r].ended, None => false };
    !(show_runs && !crate::rails::rails_live(g))
}

/// A line of the fight's log, kept to its last few hundred.   (roomfight.ts RoomFight.note)
pub fn room_fight_note(g: &mut crate::game::Game, gd: Option<GuardId>, what: &str) {
    let who = match gd { Some(x) => g.gd[x].callsign.clone(), None => String::from("-") };
    let t = g.time;
    g.fight.log.push(crate::roomfight::FightNote { t, who, what: String::from(what) });
    if g.fight.log.len() > 600 { g.fight.log.drain(0..200); }
}

/// `this.ops.find(o => o.gd === gd)`: the first operative's row for this man.
fn op_of(g: &mut crate::game::Game, gd: GuardId) -> Option<RoomfightOpId> {
    g.fight.ops.iter().copied().find(|&o| g.tab.roomfight_op[o].gd == gd)
}

/// An operative's muzzle is on a man, or coming onto him.   (roomfight.ts RoomFight.engaged)
pub fn room_fight_engaged(g: &mut crate::game::Game, gd: GuardId) -> bool {
    match op_of(g, gd) { Some(o) => g.tab.roomfight_op[o].tgt.is_some(), None => false }
}

/// Seconds he has been on his man with no round at him from where he stands.   (roomfight.ts RoomFight.noShot)
pub fn room_fight_no_shot(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    match op_of(g, gd) {
        Some(o) if g.tab.roomfight_op[o].tgt.is_some() => g.tab.roomfight_op[o].block_t,
        _ => 0.0,
    }
}

/// An operative is on a man, turning to one, or lost one inside the last `secs`.   (roomfight.ts RoomFight.busy)
pub fn room_fight_busy(g: &mut crate::game::Game, gd: GuardId, secs: f64) -> bool {
    let now = g.time;
    match op_of(g, gd) {
        Some(o) => {
            let o = &g.tab.roomfight_op[o];
            o.tgt.is_some() || o.turn.is_some() || now - o.lost_at < secs
        }
        None => false,
    }
}

/// An operative has a living hostile in his sight right now, target or not.   (roomfight.ts RoomFight.contact)
pub fn room_fight_contact(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let now = g.time;
    let Some(o) = op_of(g, gd) else { return false; };
    for (h, meter) in g.tab.roomfight_op[o].meters.iter() {
        if g.gd[h].state != crate::game::GuardState::Dead && now - meter.seen_t < 0.3 { return true; }
    }
    false
}

/// Does any living teammate see this man right now?   (roomfight.ts RoomFight.teamSees)
pub fn room_fight_team_sees(g: &mut crate::game::Game, h: GuardId) -> bool {
    room_fight_seen_by(g, h).len() > 0
}

/// The living teammates who see this man: the scripted by this module's meters, one on the fight AI by its own eyes.   (roomfight.ts RoomFight.seenBy)
pub fn room_fight_seen_by(g: &mut crate::game::Game, h: GuardId) -> Vec<GuardId> {
    let t = g.time;
    let mut out = Vec::new();
    for &id in g.fight.ops.iter() {
        let o = &g.tab.roomfight_op[id];
        let man = &g.gd[o.gd];
        if man.state == crate::game::GuardState::Dead { continue; }
        let by_meter = match o.meters.get(h) { Some(meter) => t - meter.seen_t < 0.3, None => false };
        let by_own_eyes = man.script.is_none()
            && matches!(man.foe, Some(crate::game::Foe::Guard(f)) if f == h)
            && t - man.last_seen_t < 0.3;
        if by_meter || by_own_eyes { out.push(o.gd); }
    }
    out
}

/// The sight test between two AI men in one call; stateless, anybody may ask it of any pair.   (roomfight.ts RoomFight.sightOn)
pub fn room_fight_sight_on(g: &mut crate::game::Game, viewer: GuardId, target: GuardId) -> crate::roomfight::RoomFightSightOn {
    let target_ch = g.gd[target].ch;
    let vis = crate::guards::can_see_char(g, viewer, target_ch, false);
    let meter = op_of(g, viewer)
        .and_then(|o| g.tab.roomfight_op[o].meters.get(target))
        .map_or(0.0, |meter| meter.m);
    let known = meter >= g.fight.skill.get(crate::roomfight::RoomFightSkillKey::Operative).unwrap().notice;
    let rate = if vis.visible { room_fight_rate_on(g, viewer, target, vis.dist, Some(known)) } else { 0.0 };
    crate::roomfight::RoomFightSightOn { visible: vis.visible, rate, dist: vis.dist, sighting: rate > 0.25 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pair_has_one_key_whichever_way_it_is_asked() {
        assert_eq!(pair_key(3.0, 12.0), "fightS3_12");
        assert_eq!(pair_key(12.0, 3.0), "fightS3_12");
        assert_eq!(pair_key(5.0, 5.0), "fightS5_5");
    }

    #[test]
    fn flat_ignores_height() {
        assert_eq!(flat([0.0, 9.0, 0.0], [3.0, -4.0, 4.0]).to_bits(), 5.0f64.to_bits());
    }

    fn man(g: &mut Game, state: crate::game::GuardState) -> GuardId {
        let mut m = crate::game::Guard::new(CharId(0), String::new(), Some(crate::teams::Team::Operative), RouteId(0), 0, LightId(0), JsSet::new(), FirearmId(0), [0.0; 3], 0.0, GameTalkId(0));
        m.state = state;
        g.gd.push(m)
    }

    fn op(g: &mut Game, gd: GuardId) -> RoomfightOpId {
        let mut o = crate::roomfight::Op::zeroed();
        o.gd = gd;
        let id = g.tab.roomfight_op.push(o);
        g.fight.ops.push(id);
        id
    }

    /// A row of `ops` is an ID now: each question reads the row of the man asked about, and a man with no row answers no.
    #[test]
    fn the_questions_find_the_mans_own_row() {
        let mut g = Game::blank();
        g.time = 100.0;
        let (a, b, h, none) = (man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol));
        let (oa, ob) = (op(&mut g, a), op(&mut g, b));
        g.tab.roomfight_op[ob].tgt = Some(h);
        g.tab.roomfight_op[ob].block_t = 1.5;
        g.tab.roomfight_op[oa].block_t = 9.0;
        g.tab.roomfight_op[oa].lost_at = 99.5;
        assert!(!room_fight_engaged(&mut g, a) && room_fight_engaged(&mut g, b) && !room_fight_engaged(&mut g, none));
        // (no man, no count: his blockT is not read once he has no target)
        assert_eq!((room_fight_no_shot(&mut g, a), room_fight_no_shot(&mut g, b), room_fight_no_shot(&mut g, none)), (0.0, 1.5, 0.0));
        assert!(room_fight_busy(&mut g, a, 1.0) && !room_fight_busy(&mut g, a, 0.5) && !room_fight_busy(&mut g, none, 1e9));
    }

    /// Seen by a fresh meter or, off his script, by his own eyes on that very man; never by a dead teammate; a dead man is no contact.
    #[test]
    fn who_sees_a_man() {
        let mut g = Game::blank();
        g.time = 100.0;
        let (by_meter, stale, by_eyes, dead) = (man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Dead));
        let (h, gone) = (man(&mut g, crate::game::GuardState::Patrol), man(&mut g, crate::game::GuardState::Dead));
        let (om, os, _oe, od) = (op(&mut g, by_meter), op(&mut g, stale), op(&mut g, by_eyes), op(&mut g, dead));
        let fresh = crate::roomfight::Meter { m: 1.0, seen_t: 99.9, glass_t: None };
        g.tab.roomfight_op[om].meters.set(h, fresh);
        g.tab.roomfight_op[os].meters.set(h, crate::roomfight::Meter { seen_t: 99.0, ..fresh });
        g.tab.roomfight_op[od].meters.set(h, fresh);
        g.gd[by_eyes].foe = Some(crate::game::Foe::Guard(h));
        g.gd[by_eyes].last_seen_t = 99.9;
        g.gd[stale].foe = Some(crate::game::Foe::Player);
        g.gd[stale].last_seen_t = 99.9;
        assert_eq!(room_fight_seen_by(&mut g, h), vec![by_meter, by_eyes]);
        assert!(room_fight_team_sees(&mut g, h) && !room_fight_team_sees(&mut g, gone));
        assert!(room_fight_contact(&mut g, by_meter) && !room_fight_contact(&mut g, stale));
        g.gd[h].state = crate::game::GuardState::Dead;
        assert!(!room_fight_contact(&mut g, by_meter));
    }

    /// The lamp gives him away only burning, in range, inside its cone, with the fight live, and never a dead man's.
    #[test]
    fn a_lamp_gives_its_carrier_away_inside_its_cone() {
        let mut g = Game::blank();
        g.fight.live = true;
        let mut lamp = crate::render_lights::RtLight::zeroed();
        lamp.enabled = true; lamp.intensity = 1.0; lamp.pos = [0.0, 0.0, 0.0]; lamp.dir = [0.0, 0.0, 1.0]; lamp.range = 10.0; lamp.outer_deg = 30.0;
        let l = g.tab.light.push(lamp);
        let eye = g.ch.push(crate::character::Character::blank());
        g.ch[eye].bones.set(crate::character::Bone::Head, [0.0, 0.0, 5.0]);
        let o = man(&mut g, crate::game::GuardState::Patrol);
        g.gd[o].flashlight = l;
        let lit = g.k.roomfight.lamp;
        assert_eq!(lamp_glare(&mut g, eye, o), lit);
        g.ch[eye].bones.set(crate::character::Bone::Head, [5.0, 0.0, 1.0]);   // (outside the cone)
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.ch[eye].bones.set(crate::character::Bone::Head, [0.0, 0.0, 11.0]);   // (past its range)
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.ch[eye].bones.set(crate::character::Bone::Head, [0.0, 0.0, 5.0]);
        g.tab.light[l].enabled = false;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.tab.light[l].enabled = true;
        g.tab.light[l].intensity = 0.0;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.tab.light[l].intensity = 1.0;
        g.gd[o].state = crate::game::GuardState::Dead;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
        g.gd[o].state = crate::game::GuardState::Patrol;
        g.fight.live = false;
        assert_eq!(lamp_glare(&mut g, eye, o), 0.0);
    }

    /// The doubles are the TypeScript's own: closeLane under bun with EXACT on, point blank 2.
    #[test]
    fn the_lane_at_arms_length_has_the_typescripts_bits() {
        let rows: [(f64, f64, f64, u64); 5] = [
            (0.95, 0.6, 3.3, 0x3fe970a3d70a3d70),
            (0.45, 1.2, 1.5, 0x3fd3333333333333),
            (0.45, 1.2, 2.5, 0x3fdccccccccccccd),
            (0.95, -0.2, 7.1, 0x3fd3333333333333),
            (0.2, 0.37, 11.3, 0x3fc999999999999a),
        ];
        for (clear, t, l, bits) in rows {
            assert_eq!(lane(clear, t, l, 2.0).to_bits(), bits);
        }
    }
}
