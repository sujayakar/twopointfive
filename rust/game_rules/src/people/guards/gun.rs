use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ small private helpers

/// Lends a skip_owner on for one call, keeping it for the next.
fn lend<'a, 'b>(s: &'a mut Option<&'b mut (dyn FnMut(&Game, f64) -> bool + 'b)>) -> Option<&'a mut (dyn FnMut(&Game, f64) -> bool + 'a)> {
    match s {
        Some(f) => Some(&mut **f),
        None => None,
    }
}

// ------------------------------------------------------------ the guards' collision

/// g.col.collideCircle(c.pos, c.radius, 0.2, 1.5, iterations): moves c.pos in place.
fn push_out(g: &mut Game, c: CharId, iterations: Option<f64>) {
    let (mut p, r) = (g.ch[c].pos, g.ch[c].radius);
    crate::scene_collision::collide_circle(g, &mut p, r, 0.2, 1.5, iterations);
    g.ch[c].pos = p;
}

/// The bottom of updateGuard: the statics swept and pushed out, the colleagues' capped shove, the statics again for the shove.
pub(super) fn collide_guard(g: &mut crate::game::Game, gd: GuardId, p0x: f64, p0z: f64) {
    let c = g.gd[gd].ch;
    // (1) statics, swept from the frame's start (or the snap's spot)
    let snapped = g.m.guards.unstick.last_snap_who == Some(gd) && g.m.guards.unstick.last_snap_t == g.time;
    let (fx, fz) = if snapped { (g.gd[gd].last_pos[0], g.gd[gd].last_pos[2]) } else { (p0x, p0z) };
    sweep_statics(g, c, fx, fz);
    push_out(g, c, None);
    // (2) colleagues, each in turn off where the last left him, then the whole of it capped
    let (ax, az) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
        let oc = g.gd[o].ch;
        let mut d = v3::sub(g.ch[c].pos, g.ch[oc].pos);
        d[1] = 0.0;
        let l = v3::len(d);
        if l < 0.6 && l > 1e-4 { { let to = v3::mad(g.ch[c].pos, d, (0.6 - l) / l * 0.5); crate::character::pos_replace(g, c, to); } }
    }
    let sl = m::hyp2(g.ch[c].pos[0] - ax, g.ch[c].pos[2] - az);
    if sl <= SEP_SETTLE_MIN { return; }
    if sl > SEP_MAX_M {
        g.ch[c].pos[0] = ax + (g.ch[c].pos[0] - ax) * (SEP_MAX_M / sl);
        g.ch[c].pos[2] = az + (g.ch[c].pos[2] - az) * (SEP_MAX_M / sl);
    }
    // (3) the statics again for the shove
    sweep_statics(g, c, ax, az);
    push_out(g, c, None);
}

/// Walks a body from (fx, fz) to where its pos says in strides with a two-pass push-out after each, when that is more than one stride.
fn sweep_statics(g: &mut crate::game::Game, c_2: CharId, fx: f64, fz: f64) {
    let c = c_2;
    let (tx, tz) = (g.ch[c].pos[0], g.ch[c].pos[2]);
    let dd = m::hyp2(tx - fx, tz - fz);
    let n = m::min(16.0, m::ceil(dd / GUARD_SWEEP_STEP));
    if n <= 1.0 { return; }
    g.ch[c].pos[0] = fx;
    g.ch[c].pos[2] = fz;
    let mut i = 1.0;
    while i < n {
        g.ch[c].pos[0] += (tx - fx) / n;
        g.ch[c].pos[2] += (tz - fz) / n;
        push_out(g, c, Some(2.0));
        i += 1.0;
    }
    g.ch[c].pos[0] += (tx - fx) / n;
    g.ch[c].pos[2] += (tz - fz) / n;
}

// ------------------------------------------------------------ the light meter

/// THE LIGHT ON A POINT, by which every eye judges how well it is seen: what stands on it (`light_at`), as eyes used to the place see it (`eyes_at`).
pub fn light_estimate_at(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&Game, f64) -> bool>, steady_only: Option<bool>, leave: Option<f64>) -> f64 {
    let (e, place) = light_at(g, p, skip_owner, steady_only, leave);
    e * eyes_at(g, place)
}
/// How many times brighter than it stands the light of the place about a point is seen by whoever is there (`eyes_at`).
pub fn eyes_in(g: &crate::game::Game, p: V3) -> f64 {
    if crate::hours::exposure(g) >= 1.0 { return 1.0; }
    let field = crate::sense::under_roof(g, p);
    eyes_at(g, field.map(|f| f.place))
}
/// EYES GET USED TO A PLACE: how many times brighter a place under a roof that holds `place` of light is seen than an hour's light stands at, by whoever is in it: up
/// to as bright as the open air, and never past a night's own exposure, at which a lamp is as it is at night. 1 at night and in the open air. (The picture's one
/// exposure model, render/shaders/place_common.wgsl lookExpoFor, for eyes that are in the place, under one open sky: its low level and its middle are one. The panel's
/// rows over it, Room brightness and Eyes open in the dark, are a taste of the picture's.)
const EXPO_BAND: f64 = 1.25;
fn eyes_at(g: &crate::game::Game, place: Option<f64>) -> f64 {
    let exposure = crate::hours::exposure(g);
    match place {
        Some(here) if exposure < 1.0 => {
            let darker = m::max(1.0, sim::glow::open_air(&crate::hours::sky_now(g)) / (EXPO_BAND * m::max(here, 1e-9)));
            m::min(darker * m::min(m::sqrt(darker), EXPO_BAND), 1.0 / exposure)
        }
        _ => 1.0,
    }
}
/// WHAT LIGHT STANDS ON A POINT, at the hour's exposure: the best of the six axis faces under every lamp that reaches it (each dimmed by the smoke on its way),
/// the sun's or the moon's beam and the sky, a stroke of lightning, and what comes round; and, under a roof, how light the place is. `skip_owner` = whose carried lamps are
/// left out; `steady_only` = what the place lights, with nothing that passes; `leave` = lamps of the level's own left out, by what feeds them.
pub fn light_at(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&Game, f64) -> bool>, steady_only: Option<bool>, leave: Option<f64>) -> (f64, Option<f64>) {
    light_in(g, p, skip_owner, steady_only.unwrap_or(false), leave.unwrap_or(0.0))
}

/// The lamps, the sun's beam or the moon's, the stroke, onto six faces, with the sky's under the open sky; the best of them. Under a roof the sky is what the
/// openings show of it, and with all that comes round it is the field's (`sim::glow`): its mean with the best face of the beams, or its own best face.
fn light_in(g: &mut crate::game::Game, p: V3, skip_owner: Option<&mut dyn FnMut(&Game, f64) -> bool>, steady_only: bool, leave: f64) -> (f64, Option<f64>) {
    let mut acc = lamps_on(g, p, skip_owner, steady_only, leave);
    crate::hours::sun_light(g, p, &mut acc);
    let field = crate::sense::under_roof(g, p);
    if field.is_none() || leave != 0.0 { crate::hours::dome_light(g, p, &mut acc); }
    if !steady_only { crate::weather::stroke_light(g, p, &mut acc); }
    let direct = m::max_n(&[acc[0], acc[1], acc[2], acc[3], acc[4], acc[5]]);
    let Some(f) = field else { return (direct, None) };
    // (what comes round does not say whose it is: a question that leaves lamps of the level out asks of the direct light, the sky's by lines of its own)
    (if leave == 0.0 { m::max(direct + f.mean, f.best) } else { direct }, Some(f.place))
}

/// Is this lamp, one no fixture feeds, one of a light question's. The level's own lamps have these gates in the module (light.rs `row`).
fn lamp_asked(g: &crate::game::Game, l: LightId, skip_owner: Option<&mut dyn FnMut(&Game, f64) -> bool>, steady_only: bool) -> bool {
    if !g.tab.light[l].enabled || g.tab.light[l].intensity <= 0.0 || g.tab.light[l].kind == 2.0 { return false; }
    if let Some(f) = skip_owner {
        let owner = g.tab.light[l].owner;
        if js::truthy(owner) && f(g, owner) { return false; }
    }
    !(steady_only && (g.tab.light[l].ttl >= 0.0 || g.tab.light[l].transient.unwrap_or(false)))
}

/// What a light question says of the level's own lamps: this game's rows, less the passing ones and each as its steady self to who asks what the place lights, less what the asker leaves out.
fn rows_of(steady_only: bool, leave: f64) -> crate::scene_collision::LampRows {
    let leave = if steady_only { (js::to_i32(leave) | ::sim::lights::R_PASSING as i32) as f64 } else { leave };
    crate::scene_collision::LampRows { leave, steady: steady_only }
}

/// A light question's own lamps, in the list's order, each with how many of the level's rows stand before it; the rows themselves are read where they stand.
fn own_lamps(g: &Game, p: Option<V3>, skip_owner: Option<&mut dyn FnMut(&Game, f64) -> bool>, steady_only: bool) -> Vec<[f64; sim::light::LAMP]> {
    let mut skip_owner = skip_owner;
    let exposure = crate::hours::exposure(g);
    let mut lamps = Vec::new();
    let mut rows = 0.0;
    for &l in &g.engine.lights.lights {
        if let Some(r) = g.tab.light[l].row {
            if r != rows { panic!("light: the lamp '{}' is row {} of the level's lights and stands after {} of them in the engine's list: the rows are that list's order", g.tab.light[l].name, r, rows); }
            rows += 1.0;
            continue;
        }
        if !lamp_asked(g, l, lend(&mut skip_owner), steady_only) { continue; }
        let row = &g.tab.light[l];
        let sp = if steady_only { row.steady } else { None };
        let (lp, radius, intensity) = match sp { Some(s) => (s.pos, s.radius, s.intensity), None => (row.pos, row.radius, row.intensity) };
        let (range, kind, dir, outer, inner, color) = (row.range, row.kind, row.dir, row.outer_deg, row.inner_deg, row.color);
        let intensity = if exposure < 1.0 { sim::light::by_day(crate::render_lights::light_by_day(row), intensity, row.peak_intensity, exposure) } else { intensity };
        if let Some(q) = p {
            let (dx, dy, dz) = (lp[0] - q[0], lp[1] - q[1], lp[2] - q[2]);
            if dx > range || -dx > range || dy > range || -dy > range || dz > range || -dz > range { continue; }
        }
        lamps.push([lp[0], lp[1], lp[2], range, radius, kind, dir[0], dir[1], dir[2], outer, inner, color[0], color[1], color[2], intensity, rows]);
    }
    lamps
}

/// The lamps the light that comes round is swept from: the few of nobody's fixture, and how the rows are read.
pub fn steady_lamps(g: &Game) -> (Vec<[f64; sim::light::LAMP]>, crate::scene_collision::LampRows) {
    (own_lamps(g, None, Some(&mut |_: &Game, _: f64| true), true), rows_of(true, 0.0))
}

/// The lamps' part of the light on a point: the question's own and the level's, summed onto the six faces as one list in the engine's order, each dimmed by the smoke on its way.
fn lamps_on(g: &Game, p: V3, skip_owner: Option<&mut dyn FnMut(&Game, f64) -> bool>, steady_only: bool, leave: f64) -> [f64; 6] {
    let lamps = own_lamps(g, Some(p), skip_owner, steady_only);
    crate::scene_collision::lamps_on(g, p, &lamps, crate::sense::smoky(g), rows_of(steady_only, leave))
}

/// The light on one body, the brighter of his chest and his head: what stands on him, and what eyes used to the place see him by.
pub fn light_on(g: &mut crate::game::Game, c: CharId) -> [f64; 2] {
    let pos = g.ch[c].pos;
    let chest = g.ch[c].bone(crate::character::Bone::Chest).unwrap_or(v3::add(pos, [0.0, 1.2, 0.0]));
    let head = g.ch[c].bone(crate::character::Bone::Head).unwrap_or(v3::add(pos, [0.0, 1.6, 0.0]));
    let (a, pa) = light_at(g, chest, None, None, None);
    let (b, pb) = light_at(g, head, None, None, None);
    [m::max(a, b), m::max(a * eyes_at(g, pa), b * eyes_at(g, pb))]
}
/// The light meter of a man, whoever he is: what stands on him, and how visible it makes him, eased.
pub fn light_meter(g: &mut crate::game::Game, c: CharId, dt: f64) {
    let [stands, e] = light_on(g, c);
    if !e.is_finite() { return; }
    let to = crate::game::game_visibility_of(g, e);
    let s = &mut g.shown[c];
    s.irradiance = stands;
    s.visibility = if s.visibility.is_finite() { v3::damp(s.visibility, to, 6.0, dt) } else { to };
}

// ------------------------------------------------------------ a unit's foe

/// The closure of acquireFoe: one enemy looked at; the most salient is kept.
fn consider(g: &mut Game, gd: GuardId, f: CharId, best: &mut Option<CharId>, best_rate: &mut f64, cur_visible: &mut bool) {
    let rate = match crate::character::guard_of(g, f) {
        None => {
            let vis = can_see(g, gd);
            if !vis.visible { return; }
            sight_gain(g, gd, vis.dist)
        }
        Some(o) => {
            let vis = can_see_char(g, gd, f);
            if !vis.visible { return; }
            sight_gain_on(g, gd, o, vis.dist)
        }
    };
    if g.gd[gd].foe == Some(f) && rate > 0.05 { *cur_visible = true; }
    if rate > *best_rate {
        *best_rate = rate;
        *best = Some(f);
    }
}

/// A unit's foe this frame: off a man who is gone, then on the most salient enemy he can see; a fight his eyes have won ends here.
pub(super) fn acquire_foe(g: &mut crate::game::Game, gd: GuardId) {
    use crate::game::GuardState;
    use crate::teams::Team;
    let cur_man = crate::teams::foe_guard(g, gd);
    let (mut lost_man, mut mine) = (false, false);
    let knee = match cur_man {
        Some(o) => crate::rails::rails_down(g, o),
        None => false,
    };
    if let Some(cur) = cur_man {
        if crate::condition::down(g, g.gd[cur].ch) || !g.guards.contains(&cur) || g.gd[cur].fled || knee {
            let cur_pos = g.pos_of(cur);
            if let Some(lk) = g.gd[gd].last_known {
                if v3::dist_xz(lk, cur_pos) < 1.5 { crate::mind::fix(g, gd, None, "guards"); }
            }
            mine = crate::condition::down(g, g.gd[cur].ch) && g.gd[cur].killer_id == g.char_id(gd) && g.gd[gd].state == GuardState::Alert;
            crate::mind::foe(g, gd, None, "guards");
            crate::aim::aim_state_clear(&mut g.arms[g.gd[gd].ch].aim);
            lost_man = true;
        }
    }
    let (mut best, mut best_rate, mut cur_visible): (Option<CharId>, f64, bool) = (None, 0.05, false);
    let team = g.team(gd);
    if team.theirs() && !crate::condition::down(g, g.pl[g.player].ch) && !g.player_invisible {
        let ch = g.pl[g.player].ch;
        consider(g, gd, ch, &mut best, &mut best_rate, &mut cur_visible);
    }
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == gd || crate::condition::down(g, g.gd[o].ch) { continue; }
        let och = g.gd[o].ch;
        if !g.ch[och].alive { continue; }
        if team.foe_of(g.team(o)) && !g.gd[o].fled && !crate::rails::rails_down(g, o) {
            consider(g, gd, och, &mut best, &mut best_rate, &mut cur_visible);
        }
    }
    let op = team.ours();
    if cur_visible { return; }
    if let Some(b) = best {
        if g.gd[gd].foe != Some(b) {
            crate::mind::foe(g, gd, Some(b), "guards");
            crate::aim::aim_state_clear(&mut g.arms[g.gd[gd].ch].aim);
            if mine { crate::game::game_say(g, gd, if op { "he's down — next." } else { "got him!" }, None, None); }
            return;
        }
    }
    if lost_man && best.is_none() && g.gd[gd].unit.is_none() && team == Team::Guard && !crate::condition::down(g, g.pl[g.player].ch) {
        // a staff man: the man he dropped is one of two intruders, and the other is still on his floor
        let c = g.pl[g.player].ch;
        crate::mind::foe(g, gd, Some(c), "guards");
        g.gd[gd].last_seen_t = m::min(g.gd[gd].last_seen_t, g.time - 0.5);
        if mine { crate::game::game_say(g, gd, "got him!", None, None); }
    } else if lost_man && best.is_none() && (g.gd[gd].state == GuardState::Alert || g.gd[gd].state == GuardState::Search) {
        // the man he was fighting is down and no other enemy is in his sight: he comes off the alert to a wary look round
        crate::mind::set_state(g, gd, GuardState::Suspicious, "guards");
        crate::mind::set(g, gd, 0.45, "guards");
        g.gd[gd].react_t = 1.2;
        g.gd[gd].search_t = 0.0;
        g.gd[gd].drop_path();
        g.gd[gd].search_plan = None;
        g.gd[gd].pieing = None;
        let left = enemy_up_on_floor(g, &crate::seats::here(g), team);
        let said = left || floor_called_clear(g, gd, team);
        if mine {
            let text = if op { if said { "he's down." } else { "he's down — clear." } } else if said { "got him!" } else { "got him — that's all of them." };
            crate::game::game_say(g, gd, text, None, None);
        } else if !said && !knee {
            crate::game::game_say(g, gd, if op { "clear." } else { "that's all of them." }, None, None);
        }
    } else if mine {
        crate::game::game_say(g, gd, if op { "he's down." } else { "got him!" }, None, None);
    }
}

/// Nobody calls a floor clear with an enemy still up on it.
fn enemy_up_on_floor(g: &Game, seat: &crate::seats::SeatRef, team: crate::teams::Team) -> bool {
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if crate::condition::down(g, g.gd[o].ch) { continue; }
        if team.foe_of(g.team(o)) { return true; }
    }
    team.theirs() && !crate::condition::down(g, g.pl[seat.player].ch)
}

/// One of his side said it in the last three seconds.
fn floor_called_clear(g: &Game, gd: GuardId, team: crate::teams::Team) -> bool {
    g.guards.iter().any(|&o| {
        if o == gd || g.team(o) != team { return false; }
        match g.gd[o].bubble.as_ref() {
            Some(b) => g.time - b.t < 3.0 && (b.text.contains("clear.") || b.text.contains("all of them")),
            None => false,
        }
    })
}

// ------------------------------------------------------------ the right hand

/// Which rig is in his hands.
pub fn rig_of(g: &crate::game::Game, gd: GuardId) -> crate::rig_props::RigWeapon {
    use crate::weapons::WeaponDefRig;
    let def = g.fa[g.gd[gd].weapon].def;
    let rig = g.weapon(def).rig;
    if g.gd[gd].carbine || rig == WeaponDefRig::Carbine {
        crate::rig_props::RigWeapon::Carbine
    } else if rig == WeaponDefRig::Sc20k {
        crate::rig_props::RigWeapon::Sc20k
    } else {
        crate::rig_props::RigWeapon::Pistol
    }
}

/// Which long gun that is; None = the sidearm.
pub fn long_rig_of(g: &crate::game::Game, gd: GuardId) -> Option<crate::rig_props::LongRig> {
    match rig_of(g, gd) {
        crate::rig_props::RigWeapon::Pistol => None,
        crate::rig_props::RigWeapon::Carbine => Some(crate::rig_props::LongRig::Carbine),
        crate::rig_props::RigWeapon::Sc20k => Some(crate::rig_props::LongRig::Sc20k),
    }
}

/// What his state has him carry in the gun hand.
pub fn hand_carry(g: &crate::game::Game, gd: GuardId) -> crate::guards::InHand {
    use crate::game::GuardState;
    let torch = crate::startle::startle_torch(g, gd)
        || ((g.gd[gd].state == GuardState::Patrol || (g.gd[gd].state == GuardState::Suspicious && crate::whistle::whistle_soft(g, gd)))
            && !g.gd[gd].drawn
            && rig_of(g, gd) == crate::rig_props::RigWeapon::Pistol
            && !crate::kit::kit_pistol(g, gd));
    if torch { crate::guards::InHand::Torch } else { crate::guards::InHand::Weapon }
}

/// A living man has the gun knocked out of his hand at the grab again: off the floor and in his fist.
pub fn gun_back(g: &mut crate::game::Game, gd: GuardId) {
    let dropped = g.gd[gd].dropped;
    if let Some(d) = dropped {
        if g.tab.item[d].kind != crate::throwables::ItemKind::Torch {
            if let Some(i) = g.items.items.iter().position(|&x| x == d) {
                g.items.items.remove(i);
                g.gd[gd].dropped = None;
            }
        }
    }
    g.gd[gd].gun_lost_at = -1.0;
}

/// What is physically in the hand.
pub fn in_hand(g: &crate::game::Game, gd: GuardId) -> Option<crate::guards::InHand> {
    if g.gd[gd].fled { return None; }
    if g.gd[gd].gun_lost_at >= 0.0 { return None; }
    if crate::condition::down(g, g.gd[gd].ch) || g.gd[gd].civ.is_some() { return None; }
    let carry = hand_carry(g, gd);
    if carry == crate::guards::InHand::Torch && (g.gd[gd].dropped.is_some() || g.gd[gd].torch_lost_at >= 0.0) { None } else { Some(carry) }
}

/// The magazine is empty: he stands there changing it, says so, and the clack plays at him.
pub fn start_reload_beat(g: &mut crate::game::Game, gd: GuardId) {
    if g.gd[gd].second.is_some() && crate::gunslots::side_out_alert(g, gd) { return; }
    begin_reload(g, gd);
    crate::game::game_say(g, gd, "reloading!", None, None);
}
/// HIS MAGAZINE COMES OUT: the one way a man begins a change of magazine, whichever side he is on. His hands go on with it from here (`frame.rs hands`); he says his own word.
pub fn begin_reload(g: &mut crate::game::Game, gd: GuardId) {
    let c = g.gd[gd].ch;
    let w = g.gd[gd].weapon;
    g.gd[gd].reload_t = crate::gunreload::feed_begin(g, w).dur;
    g.fa[w].burst = None;
    crate::character::anim_reload(g, c, None, None);
    let gun = if g.ch[c].anim.long_gun { Some(w) } else { None };
    crate::gunreload::feed_deal(g, c, gun, Some(true));
    let def = g.weapon(g.fa[w].def).clone();
    if !crate::gunreload::feed_starts(&def) {
        let at = g.ch[c].pos;
        crate::audio_iface::game_audio_like_play(g, "magOut", Some(at), Some(0.5), None);
    }
}

/// Counted down each frame he could otherwise fire; a staged row lets the empty go as the beat crosses `out`.
pub(super) fn tick_reload_beat(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    let w = g.gd[gd].weapon;
    let rr = crate::weapons::firearm_reload_row(g, w);
    let before = rr.dur - g.gd[gd].reload_t;
    g.gd[gd].reload_t -= dt;
    let def = g.weapon(g.fa[w].def).clone();
    let drop = crate::gunreload::feed_drop(&def);
    let c = g.gd[gd].ch;
    let gun = if g.ch[c].anim.long_gun { Some(w) } else { None };
    crate::gunreload::feed_deal(g, c, gun, Some(true));
    let now = rr.dur - g.gd[gd].reload_t;
    crate::gunreload::feed_tick(g, c, &def, before, now, None, Some(w));
    if let Some(out) = rr.out {
        if drop != Some(None) && before < out && rr.dur - g.gd[gd].reload_t >= out {
            let pos = g.ch[c].pos;
            let h = g.ch[c].bone(crate::character::Bone::HandR).unwrap_or(v3::add(pos, [0.0, 1.1, 0.0]));
            let d = g.ch[c].gun_dir;
            let vel = g.ch[c].vel;
            let (id, yaw) = (g.ch[c].id, g.ch[c].aim_yaw);
            let opts = crate::throwables::ThrowablesSpawnOpts { fill: Some(drop.flatten().unwrap_or(2.0)), who: Some(id), heading: Some(yaw), yaw: Some(yaw), ..Default::default() };
            let it = crate::throwables::throwables_spawn(
                g,
                crate::throwables::ItemKind::Mag,
                [h[0] + d[0] * 0.12, m::max(0.3, h[1] - 0.1), h[2] + d[2] * 0.12],
                [vel[0] * 0.5 + d[0] * 0.3, -0.4, vel[2] * 0.5 + d[2] * 0.3],
                Some(opts),
            );
            // it.onBounce: rate 0.6 where drop === 5, else 0.8
            g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::MagRate { belt: drop == Some(Some(5.0)) });
        }
    }
    if g.gd[gd].reload_t < 0.0 { crate::weapons::firearm_rearm(g, w); }
}

// ------------------------------------------------------------ rifles: the burst trigger

/// The closure of burstTrigger: the burst ends, and the beat to the next runs from its last serviced frame.
/// The burst he is firing, if he is firing one, is broken off: the next waits its moment.
pub(in crate::people) fn burst_broken(g: &mut Game, gd: GuardId) {
    let w = g.gd[gd].weapon;
    if g.fa[w].burst.is_some() { burst_end(g, gd, w, BURST_BROKEN_CD); }
}
fn burst_end(g: &mut Game, gd: GuardId, w: FirearmId, cd: f64) {
    let b = g.fa[w].burst;
    g.fa[w].burst = None;
    if let Some(b) = b {
        if b.idx > 0.0 {
            g.fa[w].bursts += 1.0;
            g.fa[w].last_burst_t = b.t;
        }
    }
    let since = match b {
        Some(b) => g.time - b.t,
        None => 0.0,
    };
    g.arms[g.gd[gd].ch].fire_cd = m::max(g.arms[g.gd[gd].ch].fire_cd, cd - since);
}

/// The trigger of a row with a BurstSpec, asked once a frame at the alert branch's fire site.
pub fn burst_trigger(g: &mut crate::game::Game, gd: GuardId, dt: f64, aim_at: V3, start_ok: bool, round_ok: bool, except: Option<GuardId>) {
    let w = g.gd[gd].weapon;
    let def = g.fa[w].def;
    let spec = match g.weapon(def).burst {
        Some(s) => s,
        None => return,
    };
    let c = g.gd[gd].ch;
    if let Some(b) = g.fa[w].burst {
        if g.time - b.t > dt * 1.5 + 1e-6 { burst_end(g, gd, w, BURST_BROKEN_CD); }
    }
    if g.fa[w].burst.is_none() {
        if !start_ok || g.arms[g.gd[gd].ch].fire_cd > 0.0 || round_past_colleague(g, gd, aim_at, except, None) { return; }
        let n = v3::clamp(spec.min + m::floor(crate::dice::draw_of(g, "guards::burst_trigger#1", gd) * (spec.max - spec.min + 1.0)), spec.min, spec.max);
        let mut off: Option<V3> = None;
        let opening = g.fa[w].bursts == 0.0 || g.time - g.fa[w].last_burst_t > SUPPRESS_FIRST_LAPSE;
        let wide = g.tune.suppress_first && opening && {
            let foe = match g.gd[gd].foe {
                Some(f) => f,
                None => g.pl[g.player].ch,
            };
            v3::dist_xz(g.ch[c].pos, g.ch[foe].pos) >= SUPPRESS_FIRST_MIN_M
        };
        if wide {
            let pos = g.ch[c].pos;
            let (dx, dz) = (aim_at[0] - pos[0], aim_at[2] - pos[2]);
            let ll = js::or(m::hyp2(dx, dz), 1.0);
            let (px, pz) = (-dz / ll, dx / ll);
            let chest_p: V3 = [aim_at[0], aim_at[1], aim_at[2]];
            let l = crate::scene_collision::raycast(g, chest_p, [px, 0.0, pz], SUPPRESS_FIRST_COVER_M);
            let r = crate::scene_collision::raycast(g, chest_p, [-px, 0.0, -pz], SUPPRESS_FIRST_COVER_M);
            let side: f64 = match (&l, &r) {
                (Some(a), Some(b)) => if a.t <= b.t { 1.0 } else { -1.0 },
                (Some(_), None) => 1.0,
                (None, Some(_)) => -1.0,
                (None, None) => if crate::dice::draw_of(g, "guards::burst_trigger#2", gd) < 0.5 { 1.0 } else { -1.0 },
            };
            let out = SUPPRESS_FIRST_OFF_M + crate::dice::draw_of(g, "guards::burst_trigger#3", gd) * SUPPRESS_FIRST_JITTER_M;
            off = Some([px * side * out, SUPPRESS_FIRST_UP_M, pz * side * out]);
        }
        g.fa[w].burst = Some(crate::weapons::BurstState { left: n, next_t: 0.0, idx: 0.0, t: g.time, off });
    }
    // `b` is a local: a burst something else ends under the calls below is written no more, and b is still read
    let mut b = g.fa[w].burst.unwrap();
    b.t = g.time;
    g.fa[w].burst = Some(b);
    if !round_ok {
        burst_end(g, gd, w, BURST_BROKEN_CD);
        return;
    }
    b.next_t -= dt;
    g.fa[w].burst = Some(b);
    if b.next_t > 1e-4 { return; }
    let aim: V3 = match b.off {
        Some(o) => [aim_at[0] + o[0], aim_at[1] + o[1] + SUPPRESS_FIRST_WALK_M * b.idx, aim_at[2] + o[2]],
        None => aim_at,
    };
    let grow = m::min(spec.spread_cap, spec.spread_per_round * b.idx);
    let mut cone = 0.0;
    if b.off.is_none() && b.idx > 0.0 { cone = crate::aim::hands_cone(g, gd, grow) / 2.0; }
    if round_past_colleague(g, gd, aim, except, Some(cone)) {
        burst_end(g, gd, w, BURST_BROKEN_CD);
        return;
    }
    if !crate::weapons::firearm_fire(g, w) {
        g.fa[w].burst = None;
        if crate::weapons::firearm_rounds_ready(g, w) == 0.0 && g.gd[gd].reload_t < 0.0 { start_reload_beat(g, gd); }
        return;
    }
    let mods = crate::suppression::guard_fire_mods(g, gd);
    let opts = if b.off.is_some() {
        crate::combat::FireOpts { spread: Some(0.0), steady: Some(true), ..Default::default() }
    } else {
        crate::aim::guard_round(g, gd, grow + mods.spread)
    };
    crate::combat::fire_weapon(g, c, aim, Some(&opts));
    if b.off.is_none() { crate::aim::note_bore(g, gd, aim); }
    b.idx += 1.0;
    b.left -= 1.0;
    b.next_t = spec.interval;
    if let Some(live) = g.fa[w].burst.as_mut() { *live = b; }
    if b.left <= 0.0 || crate::weapons::firearm_rounds_ready(g, w) == 0.0 {
        let gap = (spec.gap_min + crate::dice::draw_of(g, "guards::burst_trigger#4", gd) * (spec.gap_max - spec.gap_min)) * mods.gap;
        burst_end(g, gd, w, gap);
        if crate::weapons::firearm_rounds_ready(g, w) == 0.0 { start_reload_beat(g, gd); }
    }
}

/// His light posed for the frame; a burning lens carried through a wall lights from this side of it.
pub fn attach_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: Option<f64>) {
    let dt = dt.unwrap_or(1.0 / 60.0);
    let held = pose_flashlight(g, gd, dt);
    let fl = g.gd[gd].flashlight;
    if let Some(from) = held {
        if g.tab.light[fl].enabled {
            let lens = g.tab.light[fl].pos;
            let radius = FLASHLIGHT_RADIUS;
            let to = crate::lighting::lens_clear(g, from, lens, radius, Some(fl));
            g.tab.light[fl].pos = to;
        }
    }
}

/// `gd.weapon.def.rig === 'pistol'`
fn weapon_rig_is_pistol(g: &Game, gd: GuardId) -> bool {
    let def = g.fa[g.gd[gd].weapon].def;
    matches!(g.m.weapons.weapons.get(def), Some(d) if d.rig == crate::weapons::WeaponDefRig::Pistol)
}

/// The light at its lens, its beam and whether it burns; returns what the lens hangs off.
fn pose_flashlight(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<V3> {
    use crate::character::Bone;
    use crate::game::GuardState;
    let c = g.gd[gd].ch;
    let fl = g.gd[gd].flashlight;
    if g.ch[c].bones.get(Bone::HandR).is_none() { return None; }
    let mut dir = g.ch[c].gun_dir;
    let rk = rig_of(g, gd);
    let carbine = rk == crate::rig_props::RigWeapon::Carbine;
    let torch = hand_carry(g, gd) == crate::guards::InHand::Torch;
    let hangs = torch && g.ch[c].alive && torch_low(g, gd);
    if torch && g.ch[c].alive {
        let low = if hangs { Some(crate::rig_props::TorchCarryDirLow { tan: g.k.torch.low_tan, rate: g.k.torch.low_rate }) } else { None };
        let prev = g.gd[gd].beam_dir;
        let beam = crate::rig_props::torch_carry_dir(g, c, prev, Some(dt), low);
        g.gd[gd].beam_dir = beam;
        dir = beam;
    } else {
        g.gd[gd].beam_dir = v3::copy(dir);
    }
    let lens = if rk == crate::rig_props::RigWeapon::Sc20k {
        crate::rig_props::sc20k_light_pose(g, c).pos
    } else if crate::kit::kit_pistol(g, gd) && weapon_rig_is_pistol(g, gd) {
        crate::rig_props::rig_light_pose(g, c).pos
    } else {
        let want = if torch { crate::guards::InHand::Torch } else { crate::guards::InHand::Weapon };
        let in_torch_hand = crate::handwork::swap_held(g, gd, Some(want)) == Some(crate::guards::InHand::Torch);
        let weapon = if carbine { crate::rig_props::RigWeapon::Carbine } else { crate::rig_props::RigWeapon::Pistol };
        crate::rig_props::rig_torch_pos(g, c, dir, in_torch_hand, Some(weapon))
    };
    g.tab.light[fl].pos = lens;
    g.tab.light[fl].dir = dir;
    if g.gd[gd].fled {
        g.tab.light[fl].enabled = false;
        return g.ch[c].bone(Bone::UpperArmR);
    }
    let mut held: Option<V3> = g.ch[c].bone(Bone::UpperArmR);
    let lost_live = if torch && g.gd[gd].torch_lost_at >= 0.0 { g.gd[gd].torch_lost_at } else { g.gd[gd].gun_lost_at };
    if crate::condition::down(g, g.gd[gd].ch) || lost_live >= 0.0 {
        if let Some(it) = g.gd[gd].dropped {
            let (yaw, tumble) = (g.tab.item[it].yaw, g.tab.item[it].tumble);
            let hd: V3 = [m::sin(yaw) * m::cos(tumble), -m::sin(tumble), m::cos(yaw) * m::cos(tumble)];
            let at = g.tab.item[it].pos;
            g.tab.light[fl].pos = v3::mad(at, hd, 0.1);
            g.tab.light[fl].dir = hd;
            held = Some(at);
        }
    }
    if !crate::condition::down(g, g.gd[gd].ch) && lost_live >= 0.0 {
        if !g.gd[gd].light_lit_at_loss { g.tab.light[fl].enabled = false; return held; }
        sputter(g, fl, g.time - lost_live);
        return held;
    }
    if crate::condition::down(g, g.gd[gd].ch) && g.time >= g.gd[gd].light_dead_until {
        if !g.gd[gd].light_lit_at_loss { g.tab.light[fl].enabled = false; return held; }
        let (tl, gl) = (g.gd[gd].torch_lost_at, g.gd[gd].gun_lost_at);
        let lost_at = m::min(m::min(if tl >= 0.0 { tl } else { f64::INFINITY }, if gl >= 0.0 { gl } else { f64::INFINITY }), g.gd[gd].died_at);
        sputter(g, fl, g.time - lost_at);
        return held;
    }
    let until = g.gd[gd].light_dead_until;
    if g.time < until {
        g.tab.light[fl].enabled = false;
    } else if until > 0.0 && g.time < until + 1.0 {
        g.tab.light[fl].enabled = m::sin(g.time * 50.0) > 0.2;
    } else {
        let unit_light = g.gd[gd].unit.as_ref().map(|u| u.light);
        let on = (match unit_light {
            None => true,
            Some(lit) => (lit || crate::darkfoe::dark_foe_lit(g, gd) || crate::powerout::power_out_lit(g, gd))
                && (!light_ruled(g, gd) || team_lit(g, gd))
                && !crate::nvg::nvg_dark(g, gd)
                && !crate::silent::silent_dark(g, gd),
        }) && !(torch && crate::calmtorch::calm_torch_dark(g, gd, dt));
        g.tab.light[fl].enabled = on;
    }
    if wall_dark(g, gd, held) && g.tab.light[fl].enabled {
        g.tab.light[fl].enabled = false;
    }
    if g.tab.light[fl].enabled && crate::doze::doze_dark(g, gd) { g.tab.light[fl].enabled = false; }
    let (peak, inner, outer) = (g.tune.flash_intensity, g.tune.flash_inner, g.tune.flash_outer);
    let radius = FLASHLIGHT_RADIUS;
    let row = &mut g.tab.light[fl];
    row.peak_intensity = peak;
    row.intensity = peak;
    row.inner_deg = inner;
    row.outer_deg = outer;
    row.radius = radius;
    held
}

/// A light on the floor `t` seconds after it fell: a couple of sputters, then dark.
fn sputter(g: &mut Game, fl: LightId, t: f64) {
    let row = &mut g.tab.light[fl];
    row.enabled = t < 0.9 && (t < 0.35 || m::sin(t * 60.0) > 0.3);
    row.intensity = row.peak_intensity * v3::clamp(1.0 - t / 0.9, 0.0, 1.0);
}

// ------------------------------------------------------------ on a screen, and a man's line

/// On a player's screen: with two, on either's.
pub fn on_screen(g: &mut crate::game::Game, p: V3) -> bool {
    if g.seats.len() > 1 {
        let seats = g.seats.clone();
        for s in seats {
            if crate::seats::with_seat(g, s, &mut |g: &mut Game| in_game_view(&crate::seats::here(g), p)) { return true; }
        }
        return false;
    }
    in_game_view(&crate::seats::here(g), p)
}

/// inView: of the seat this pass is about.
fn in_game_view(seat: &crate::seats::SeatRef, p: V3) -> bool {
    match seat.view_proj {
        None => false,
        Some(crate::game::GameViewProj::Array(v)) => in_view_of(|i| v[i], p),
        Some(crate::game::GameViewProj::Float32Array(v)) => in_view_of(|i| v[i] as f64, p),
    }
}

fn in_view_of(vp: impl Fn(usize) -> f64, p: V3) -> bool {
    let w = vp(3) * p[0] + vp(7) * p[1] + vp(11) * p[2] + vp(15);
    if w <= 0.0 { return false; }
    let x = (vp(0) * p[0] + vp(4) * p[1] + vp(8) * p[2] + vp(12)) / w;
    let y = (vp(1) * p[0] + vp(5) * p[1] + vp(9) * p[2] + vp(13)) / w;
    x > -0.92 && x < 0.92 && y > -0.9 && y < 0.9
}

/// How long a line stands over a man's head, by its length (the language's: UTF-16 units).
pub fn bubble_secs(text: &str) -> f64 {
    m::min(4.0, m::max(2.0, 1.5 + text.encode_utf16().count() as f64 * 0.055))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `bubble_secs` against pinned bits; an em dash is one unit, a clef two.
    #[test]
    fn bubble_secs_bits() {
        let rows: [(&str, u64); 5] = [
            ("", 0x4000000000000000),
            ("reloading!", 0x4000666666666666),
            ("he's down — clear.", 0x4003eb851eb851ec),
            ("𝄞 reloading!", 0x4001b851eb851eb8),
            ("a very long line that stands for four seconds whatever it says, and then some more", 0x4010000000000000),
        ];
        for (text, bits) in rows {
            assert_eq!(bubble_secs(text).to_bits(), bits, "{}", text);
        }
    }

}
