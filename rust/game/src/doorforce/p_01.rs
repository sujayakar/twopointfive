//! src/game/doorforce.ts lines 1 to 369
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the game's State, which is there (`states` holds it once a hook made it); never kept across a call that takes `g`
macro_rules! state { ($g:expr) => { $g.m.doorforce.states.as_mut().unwrap() } }
/// a man's row, which is there
macro_rules! man { ($g:expr, $gd:expr) => { $g.m.doorforce.states.as_mut().unwrap().men.get_mut($gd).unwrap() } }
/// the stand he is on, which is there
macro_rules! stand { ($g:expr, $gd:expr) => { $g.m.doorforce.states.as_mut().unwrap().men.get_mut($gd).unwrap().on.as_mut().unwrap() } }

/// The rule is up: the AI runs and no tour has the stage. (doorforce.ts live)
pub fn live(g: &mut crate::game::Game) -> bool { g.ai_enabled && !g.touring }

/// The game's state, made if absent, every belief forgotten if the clock went back (a load). (doorforce.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::doorforce::State {
    let now = g.time;
    let s = g.m.doorforce.states.get_or_insert_with(|| State { last_t: now, men: JsMap::new(), kickers: JsMap::new(), boots: JsMap::new() });
    if now < s.last_t - 1e-6 { s.men.clear(); s.kickers.clear(); s.boots.clear(); s.last_t = now; }
    s
}

/// A new man's row, with the literal's numbers (not `Man::zeroed`: served and cfT start long ago). (doorforce.ts manOf)
fn new_man() -> Man {
    Man { beliefs: JsMap::new(), saw: None, on: None, kick: None, rest: JsMap::new(), served: -1e9, cf_t: -1e9 }
}

/// A man's row, made if absent. (doorforce.ts manOf)
pub fn man_of<'g>(g: &'g mut crate::game::Game, gd: GuardId) -> &'g mut crate::doorforce::Man {
    state_of(g).men.get_or_insert_with(gd, new_man)
}

/// Distance on the floor. (doorforce.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// A line for the panel's log. (doorforce.ts log)
pub fn log(g: &mut crate::game::Game, s: &str) {
    let line = alloc::format!("{:.2} {}", g.time, s);
    let l = &mut g.m.doorforce.doorforce_log;
    l.push(line);
    if l.len() > 400 { l.drain(0..200); }
}

/// A man this rule may have: a defender, up, on his own senses. (doorforce.ts defender)
pub fn defender(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    g.gd[gd].team != crate::teams::Team::Operative && g.gd[gd].state != crate::game::GuardState::Dead && g.ch[c].alive
}

/// A leaf that is a shut door to a man: hung, home in its frame. (doorforce.ts shut)
pub fn shut(g: &mut crate::game::Game, d: DoorId) -> bool {
    !crate::doors::door_off(g, d) && crate::doors::door_is_closed(g, d)
}

/// `at` is close behind the shut leaf for a man at `from`, and it is not a leaf he is set on behind a wedge. (doorforce.ts behind)
pub fn behind(g: &mut crate::game::Game, gd: GuardId, d: DoorId, from: V3, at: V3) -> bool {
    let k = g.k.doorforce;
    if !shut(g, d) || flat(g.dr[d].frame_centre, at) > k.behind_m || flat(g.dr[d].frame_centre, from) > k.reach_m { return false; }
    if crate::doors::doors_side(g, d, from) == crate::doors::doors_side(g, d, at) { return false; }
    let l = crate::doors::doors_frame_coords(g, d, at);
    if l.along < -k.behind_m || l.along > g.dr[d].def.width + k.behind_m { return false; }
    !crate::digin::dig_in_set_on(g, gd, d)
}

/// HEARD: the shut door `at` is close behind for a man at `from`, the nearest such to `at`. (doorforce.ts doorBetween)
pub fn door_between(g: &mut crate::game::Game, gd: GuardId, from: V3, at: V3) -> Option<DoorId> {
    let mut best: Option<DoorId> = None;
    let mut bd = m::INF;
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        i += 1;
        if !behind(g, gd, d, from, at) { continue; }
        let da = flat(g.dr[d].frame_centre, at);
        if da < bd { bd = da; best = Some(d); }
    }
    best
}

/// SEEN: the shut door that is now the first thing in his eyes' line to where he last saw the man. (doorforce.ts doorHiding)
pub fn door_hiding(g: &mut crate::game::Game, gd: GuardId, at: V3) -> Option<DoorId> {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let from: V3 = [pos[0], pos[1] + 1.5, pos[2]];
    let dd = v3::sub([at[0], pos[1] + g.k.doorforce.chest_y, at[2]], from);
    let l = v3::len(dd);
    if l < 0.3 { return None; }
    let h = crate::scene_collision::raycast(g, from, v3::scale(dd, 1.0 / l), l)?;
    if h.index < 1e6 { return None; }
    let bx = g.col.dynamic_boxes[(h.index - 1e6) as usize];
    let found = g.doors.list.iter().copied().find(|&x| g.dr[x].box_ == bx);
    let d = found?;
    if behind(g, gd, d, pos, at) { Some(d) } else { None }
}

/// He has word of an enemy at the far face of `d`: sureness adds (a sighting sets a floor), the place and the time are the newest. (doorforce.ts learn)
pub fn learn(g: &mut crate::game::Game, gd: GuardId, d: DoorId, at: V3, add: f64, seen: bool) {
    let now = g.time;
    let man = man_of(g, gd);
    if let Some(&rest) = man.rest.get(d) { if rest > now { return; } }
    let b = man.beliefs.get_or_insert_with(d, || Belief { door: d, sure: 0.0, at: [at[0], 0.0, at[2]], t: now, seen });
    b.sure = m::min(3.0, if seen { m::max(b.sure, add) } else { b.sure + add });
    b.at = [at[0], 0.0, at[2]];
    b.t = now;
    b.seen = seen;
}

/// His ears: a sound that came to him across a shut leaf, or a door's own noise, is word of that door. (doorforce.ts doorForceHear)
pub fn door_force_hear(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent, prop: crate::doorforce::DoorForceHearProp) {
    use crate::director::EventKind;
    if !live(g) || g.gd[gd].state != crate::game::GuardState::Alert || !defender(g, gd) || g.gd[gd].script.is_some() { return; }
    let k = g.k.doorforce;
    let who: f64;
    let horizon: f64;
    let mut loud = false;
    match e.kind {
        EventKind::Step => { who = e.who.unwrap_or(crate::consts::PLAYER_ID); horizon = 9.0; }
        EventKind::Door => {
            match e.who { None => return, Some(w) => { if w < 0.0 { return; } who = w; } }
            horizon = 12.0;
        }
        EventKind::Shot => { who = crate::consts::PLAYER_ID; horizon = 22.0; loud = e.level.unwrap_or(0.0) >= 1.0; }
        EventKind::GuardShot => {
            match e.who { None => return, Some(w) => who = w }
            horizon = 22.0;
            loud = e.level.unwrap_or(0.0) >= 1.0;
        }
        _ => return,
    }
    let c = g.gd[gd].ch;
    let (me, team) = (g.ch[c].id, g.gd[gd].team);
    let player_ch = g.pl[g.player].ch;
    let player_id = g.ch[player_ch].id;
    if who == me { return; }
    let their = crate::teams::team_of_id(g, who);
    if !crate::teams::is_hostile(g, team, their) || (g.player_invisible && who == player_id) { return; }
    let a = e.level.unwrap_or(0.3) * v3::clamp(1.0 - prop.path_len / horizon, 0.0, 1.0) * prop.carry;
    if a < k.hear_min && !loud { return; }
    let mut add = if loud { k.shot } else { a * k.gain };
    let d: DoorId;
    let src: V3;
    if e.kind == EventKind::Door {
        // a door's own noise is word of THAT door, only while the man whose hand was on it stands at its far face; the place is the far face, never his feet
        let maker = if who == player_id {
            Some(player_ch)
        } else {
            let mut found = None;
            for &x in g.guards.iter() { let xc = g.gd[x].ch; if g.ch[xc].id == who { found = Some(xc); break; } }
            found
        };
        let maker = match maker { Some(x) => x, None => return };
        let maker_pos = g.ch[maker].pos;
        let my_pos = g.ch[c].pos;
        let mut found: Option<DoorId> = None;
        let mut i = 0;
        while i < g.doors.list.len() {
            let x = g.doors.list[i];
            i += 1;
            if shut(g, x) && flat(g.dr[x].frame_centre, e.pos) < g.dr[x].def.width + 0.3 && behind(g, gd, x, my_pos, maker_pos) { found = Some(x); break; }
        }
        d = match found { Some(x) => x, None => return };
        let u = crate::doors::door_dir(g, d);
        let s = crate::doors::doors_side(g, d, maker_pos);
        let (hinge, width) = (g.dr[d].def.hinge, g.dr[d].def.width);
        let mx = hinge[0] + u[0] * width * 0.5;
        let mz = hinge[1] + u[1] * width * 0.5;
        let mut far: V3 = [mx - u[1] * s * k.past_leaf_m, 0.0, mz + u[0] * s * k.past_leaf_m];
        if crate::doors::doors_side(g, d, far) != s { far = [mx + u[1] * s * k.past_leaf_m, 0.0, mz - u[0] * s * k.past_leaf_m]; }
        src = far;
        add = m::max(add, k.handle);
    } else {
        // anything else must have come to him ACROSS a shut leaf (the hearing model's word)
        if prop.direct || !prop.routed || prop.carry > k.shut_carry { return; }
        src = e.pos;
        let my_pos = g.ch[c].pos;
        d = match door_between(g, gd, my_pos, src) { Some(x) => x, None => return };
    }
    state_of(g);
    let h = e.id.unwrap_or(0.0) * 31.0 + me;
    let ang = crate::debris::hash01(h) * m::PI * 2.0;
    let r = k.heard_err_m * crate::debris::hash01(h + 7919.0);
    let mut at: V3 = [src[0] + m::cos(ang) * r, 0.0, src[2] + m::sin(ang) * r];
    if crate::doors::doors_side(g, d, at) != crate::doors::doors_side(g, d, src) { at = [src[0], 0.0, src[2]]; }   // (never put through the leaf onto his own face of it)
    learn(g, gd, d, at, add, false);
    g.stats.doorforce.heard += 1;
}

/// This 'kick' is a boot this rule had a man of the listener's own side (or the listener) throw: no intruder. (doorforce.ts doorForceOwnBoot)
pub fn door_force_own_boot(g: &mut crate::game::Game, gd: GuardId, e: crate::game::GameEvent) -> bool {
    if e.kind != crate::director::EventKind::Kick { return false; }
    let who = match e.who { Some(w) => w, None => return false };
    if !live(g) { return false; }
    let t = match g.m.doorforce.states.as_ref().and_then(|s| s.boots.get(crate::Num(who))) { Some(&t) => t, None => return false };
    if g.time - t > 1.0 { return false; }
    let c = g.gd[gd].ch;
    if who == g.ch[c].id { return true; }
    let team = g.gd[gd].team;
    let their = crate::teams::team_of_id(g, who);
    !crate::teams::is_hostile(g, team, their)
}

/// Once a frame after the guards: what each man saw and heard fades, a sighting just lost is put behind the leaf it went behind. (doorforce.ts doorForceTick)
pub fn door_force_tick(g: &mut crate::game::Game, dt: f64) {
    if !live(g) { return; }
    let (kf, time) = (g.k.doorforce, g.time);
    state_of(g).last_t = time;
    let k = m::exp(-dt / kf.fade_secs * m::log(3.0));
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if !defender(g, gd) {
            if state!(g).men.has(gd) { drop(g, gd); }
            continue;
        }
        let now = time - g.gd[gd].last_seen_t < dt * 0.5 + 1e-6;
        let pc = crate::teams::foe_char(g, gd);
        let alert = g.gd[gd].state == crate::game::GuardState::Alert;
        if alert {
            if let Some(pc) = pc {
                if now && !crate::teams::foe_down(g, gd) {
                    let p = g.ch[pc].pos;
                    man_of(g, gd).saw = Some(ManSaw { at: [p[0], 0.0, p[2]], t: time, used: false });
                }
            }
        }
        if !state!(g).men.has(gd) { continue; }
        if !alert { drop(g, gd); continue; }
        // his eyes: the man he had in sight a moment ago is now at the far face of a shut leaf
        if let Some(sw) = man!(g, gd).saw {
            if !now && !sw.used && time - sw.t <= kf.seen_secs {
                if let Some(d) = door_hiding(g, gd, sw.at) {
                    man!(g, gd).saw.as_mut().unwrap().used = true;
                    learn(g, gd, d, sw.at, kf.seen, true);
                    g.stats.doorforce.seen += 1;
                }
            }
        }
        let cf = crate::coverfire::coverfire_on(g, gd);
        let theirs = match &cf { Some(x) => x.kind == "thru", None => false };
        if theirs && cf.as_ref().unwrap().fired > 0.0 { man!(g, gd).cf_t = time; }
        // a stand or a boot the alert branch has stopped asking about is over: taken up again it starts again
        {
            let s = state!(g);
            let row = s.men.get_mut(gd).unwrap();
            if let Some(kick) = row.kick {
                if time - row.served > 0.2 {
                    if s.kickers.get(kick.door) == Some(&gd) { s.kickers.delete(kick.door); }
                    row.kick = None;
                    row.on = None;
                }
            }
            if row.on.is_some() && time - row.served > 0.5 { row.on = None; }
        }
        let mut at = 0;
        while let Some(d) = man!(g, gd).beliefs.next_key(&mut at) {
            let b = man!(g, gd).beliefs.get_mut(d).unwrap();
            if !(theirs && b.seen) { b.sure *= k; }   // (what he saw does not fade while coverfire.ts's burst on that sighting is going through the leaf)
            let (sure, t) = (b.sure, b.t);
            if !shut(g, d) || sure < 0.02 || time - t > kf.stale_secs + kf.fade_secs {
                let row = man!(g, gd);
                row.beliefs.delete(d);
                if row.on.map(|o| o.door) == Some(d) && row.kick.is_none() { row.on = None; }
            }
        }
        {
            let s = state!(g);
            let row = s.men.get_mut(gd).unwrap();
            let mut at = 0;
            while let Some(d) = row.rest.next_key(&mut at) {
                if *row.rest.get(d).unwrap() <= time { row.rest.delete(d); }
            }
            if let Some(kick) = row.kick {
                if s.kickers.get(kick.door) != Some(&gd) { row.kick = None; }
            }
        }
    }
    let mut at = 0;
    while let Some(d) = state!(g).kickers.next_key(&mut at) {
        let gd = *state!(g).kickers.get(d).unwrap();
        if !defender(g, gd) || state!(g).men.get(gd).and_then(|x| x.kick).map(|x| x.door) != Some(d) { state!(g).kickers.delete(d); }
    }
    let mut at = 0;
    while let Some(gd) = state!(g).men.next_key(&mut at) {
        if !g.guards.contains(&gd) { drop(g, gd); }
    }
    let s = state!(g);
    let mut at = 0;
    while let Some(id) = s.boots.next_key(&mut at) {
        if time - *s.boots.get(id.clone()).unwrap() > 2.0 { s.boots.delete(id); }
    }
}

/// A man's row goes, and the door he had the boot of is free. (doorforce.ts drop)
pub fn drop(g: &mut crate::game::Game, gd: GuardId) {
    let s = state_of(g);
    if let Some(kick) = s.men.get(gd).and_then(|x| x.kick) {
        if s.kickers.get(kick.door) == Some(&gd) { s.kickers.delete(kick.door); }
    }
    s.men.delete(gd);
}

/// roundFor's `through`: the round reaches `to`, the leaf the first thing in the line, not with his muzzle against it, and not laid through standing glass.
fn round_through(g: &mut Game, gd: GuardId, def: &crate::weapons::WeaponDef, from: V3, to: V3, e: f64, index: f64) -> bool {
    let t = match crate::wallpen::thin_line(g, from, to, e, Some(0.3), None) {
        Some(crate::wallpen::ThinLine2::ThinLine(t)) => t,
        _ => return false,
    };
    if t.walls[0].index != index || t.walls[0].t < 0.5 { return false; }
    if !js::truthy(t.panes) { return true; }
    let c = g.gd[gd].ch;
    crate::glasspen::shot_energy(g, c, Some(def)) != Some(0.0)
}

/// Where his rounds go for this belief: the place when the leaf is first in the line and the round crosses it, or the stretch of leaf nearest a man right at it. (doorforce.ts roundFor)
pub fn round_for(g: &mut crate::game::Game, gd: GuardId, b: &mut crate::doorforce::Belief) -> Option<V3> {
    let kf = g.k.doorforce;
    let d = b.door;
    if g.dr[d].def.steel == Some(true) || !crate::wallpen::wall_men_live(g) { return None; }
    let c = g.gd[gd].ch;
    let from = crate::guards::shot_origin(g, c);
    let def = g.m.weapons.weapons.get(g.fa[g.gd[gd].weapon].def).unwrap().clone();
    let e = crate::glasspen::round_energy(g, &def);
    let bx = g.dr[d].box_;
    let index = 1e6 + match g.col.dynamic_boxes.iter().position(|&x| x == bx) { Some(i) => i as f64, None => -1.0 };
    let y = g.ch[c].pos[1] + kf.chest_y;
    let at: V3 = [b.at[0], y, b.at[2]];
    if round_through(g, gd, &def, from, at, e, index) { return Some(at); }
    let l = crate::doors::doors_frame_coords(g, d, b.at);
    let (closed, width, hinge) = (g.dr[d].def.closed_dir, g.dr[d].def.width, g.dr[d].def.hinge);
    if l.off > kf.near_leaf_m || l.along < -kf.near_leaf_m || l.along > width + kf.near_leaf_m { return None; }
    let ux = m::cos(closed);
    let uz = m::sin(closed);
    let al = v3::clamp(l.along, 0.15, width - 0.15);
    let s = crate::doors::doors_side(g, d, b.at);
    let px = hinge[0] + ux * al;
    let pz = hinge[1] + uz * al;   // on the leaf's line, the stretch nearest the man
    let mut to: V3 = [px - uz * s * kf.past_leaf_m, y, pz + ux * s * kf.past_leaf_m];
    if crate::doors::doors_side(g, d, to) != s { to[0] = px + uz * s * kf.past_leaf_m; to[2] = pz - ux * s * kf.past_leaf_m; }
    if round_through(g, gd, &def, from, to, e, index) { Some(to) } else { None }
}

/// `kick_step` on the man's own row: the signature still lends `m` beside `g`, so it works on a copy that goes back by key. The calls out in it (the walk, the boot, the scuff, the animator) never reach this module (notes, Traps).
fn kick_step_lent(g: &mut Game, gd: GuardId, dt: f64) -> Option<KickStep> {
    let mut row = man!(g, gd).clone();
    let r = kick_step(g, gd, &mut row, dt);
    if let Some(home) = state!(g).men.get_mut(gd) { *home = row; }
    r
}

/// The alert branch's hook: the door this man is on, where his gun points and how fast his legs go. None = nothing of the kind. (doorforce.ts doorForce)
pub fn door_force(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::doorforce::DoorForce> {
    if !live(g) || !defender(g, gd) || g.gd[gd].script.is_some() || g.gd[gd].held.is_some() || g.time < g.gd[gd].dazzled_until || g.time < g.gd[gd].stagger_until { return None; }
    g.m.doorforce.states.as_ref()?.men.get(gd)?;
    let (kf, time, c, w) = (g.k.doorforce, g.time, g.gd[gd].ch, g.gd[gd].weapon);
    man!(g, gd).served = time;
    if man!(g, gd).kick.is_some() {
        if let Some(r) = kick_step_lent(g, gd, dt) { return Some(DoorForce { aim: r.aim, move_: r.move_ }); }
    }
    // the door he is on, or the one he is surest of
    let on_door = man!(g, gd).on.map(|o| o.door);
    let mut b: Option<Belief> = match on_door { Some(d) => man!(g, gd).beliefs.get(d).copied(), None => None };
    if let Some(x) = b {
        if x.sure < kf.keep || time - x.t > kf.stale_secs || !shut(g, x.door) { b = None; }
    }
    if b.is_none() {
        man!(g, gd).on = None;
        let mut at = 0;
        while let Some(key) = man!(g, gd).beliefs.next_key(&mut at) {
            let x = *man!(g, gd).beliefs.get(key).unwrap();
            if x.sure >= kf.sure && time - x.t <= kf.stale_secs && shut(g, x.door) && b.map_or(true, |y| x.sure > y.sure) { b = Some(x); }
        }
    }
    let mut b = b?;
    let d = b.door;
    if man!(g, gd).on.is_none() {
        // (a burst coverfire.ts has just put through a thin thing on this sighting is the first: this one's rounds then want a newer word)
        let theirs = b.seen && time - man!(g, gd).cf_t < 2.0;
        man!(g, gd).on = Some(Stand { door: d, t0: time, n: 0.0, fired_t: if theirs { time } else { -1e9 }, next: 0.0, lay: 0.0, fired: 0.0, no_shot_t: 0.0, to: None, to_key: [m::NAN, m::NAN, m::NAN] });
        g.stats.doorforce.stands += 1;
        let line = alloc::format!("{} on {}: {} (sure {:.2})", g.gd[gd].callsign, g.dr[d].def.name, if b.seen { "saw him go behind it" } else { "hears them behind it" }, b.sure);
        log(g, &line);
    }
    let free = !g.gd[gd].pinned && !g.gd[gd].hold;
    // (what his round would do changes only with a new word, or when he moves: asked again then, not every frame)
    let pos = g.ch[c].pos;
    let key = stand!(g, gd).to_key;
    if key[0] != b.t || key[1] != pos[0] || key[2] != pos[2] {
        let to = round_for(g, gd, &mut b);
        let st = stand!(g, gd);
        st.to = to;
        st.to_key = [b.t, pos[0], pos[2]];
    }
    let to = stand!(g, gd).to;
    let st = stand!(g, gd);
    st.no_shot_t = if to.is_some() { 0.0 } else { st.no_shot_t + dt };
    let (t0, no_shot_t) = (st.t0, st.no_shot_t);
    // THE BOOT: a free man who has been on it long enough and is still sure
    if free && !state!(g).kickers.has(d) && (time - t0 >= kf.kick_secs || no_shot_t >= kf.kick_secs_no_shot) && g.gd[gd].reload_t < 0.0 {
        let spot = crate::doors::doors_kick_spot(g, d, pos);
        let steel = g.dr[d].def.steel == Some(true);
        if steel || crate::doors::doors_boot_holds(g, d, pos) || crate::scene_collision::nav_grid_is_blocked(g, spot.pos[0], spot.pos[2]) {
            // no boot opens it from here: through it by the handle, as he always went; the place he believes in is his fix
            g.stats.doorforce.no_kick += 1;
            let why = if steel { "steel" } else if crate::doors::doors_boot_holds(g, d, pos) { "the frame would hold a boot" } else { "no floor to kick from" };
            let line = alloc::format!("{} leaves {} to his hand ({})", g.gd[gd].callsign, g.dr[d].def.name, why);
            log(g, &line);
            g.gd[gd].last_known = Some([b.at[0], 0.0, b.at[2]]);
            let row = man!(g, gd);
            row.rest.set(d, time + kf.rest_secs);
            row.beliefs.delete(d);
            row.on = None;
            return None;
        }
        man!(g, gd).kick = Some(Kick { door: d, spot: spot.pos, yaw: spot.yaw, phase: KickPhase::Walk, t0: time, set_t: 0.0 });
        state!(g).kickers.set(d, gd);
        let man = &mut g.gd[gd];
        man.path = Vec::new();
        man.path_goal = None;
        man.stall_ref = None;
        let line = alloc::format!("{} goes to boot {}", g.gd[gd].callsign, g.dr[d].def.name);
        log(g, &line);
        return kick_step_lent(g, gd, dt).map(|r| DoorForce { aim: r.aim, move_: r.move_ });
    }
    // THE ROUNDS: stood still, laid on the spot, by the aim law and the cadence every round of his obeys
    let aim: V3 = match to { Some(x) => x, None => [b.at[0], pos[1] + kf.chest_y, b.at[2]] };
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 6.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.gd[gd].fire_cd -= dt;
    let held = DoorForce { aim, move_: 0.0 };
    if g.gd[gd].reload_t >= 0.0 {
        crate::guards::tick_reload_beat(g, gd, dt);
        return Some(held);
    }
    if to.is_none() { return Some(held); }
    let st = stand!(g, gd);
    if st.n >= kf.rounds { st.n = 0.0; st.fired_t = time; st.next = time + kf.gap_secs; }   // these rounds are through: he listens
    let p = g.k.aim.guard;
    let dist = flat(pos, aim);
    let on = m::abs(v3::wrap_angle(m::atan2(aim[0] - pos[0], aim[2] - pos[2]) - g.ch[c].aim_yaw)) < 0.25 && !g.gd[gd].muzzle_down;
    let settle = m::max(p.settle_secs, 1e-3) * v3::clamp(dist / m::max(p.range_ref, 0.1), p.near_k, 1.0) * (match g.gd[gd].skill { Some(sk) => sk.settle_k, None => 1.0 });
    let aim_model = g.tune.aim_model;
    let st = stand!(g, gd);
    st.lay = if !aim_model { if on { 1.0 } else { 0.0 } } else if on { m::min(1.0, st.lay + dt / settle) } else { m::max(0.0, st.lay - p.lose_rate * dt) };   // (his lay on the spot, kept while he listens too)
    let (lay, fired_t, next) = (st.lay, st.fired_t, st.next);
    let armed = b.t > fired_t && time >= next;
    if !armed && g.fa[w].burst.is_none() { return Some(held); }
    let muzzle = g.ch[c].muzzle;
    if !crate::coverfire::cover_lane(g, gd, muzzle, aim) {
        g.stats.doorforce.held_friend += 1;
        return Some(held);
    }
    let ok = on && lay >= p.acquire_min - 1e-9;
    let had = crate::weapons::firearm_rounds_ready(g, w);
    let def = g.fa[w].def;
    if g.m.weapons.weapons.get(def).unwrap().burst.is_some() {
        let tr = g.gd[gd].aim.track;
        g.gd[gd].aim.track = lay;   // (his lay on the SPOT for the burst's own cone, and for nothing else: his track on the man is put back)
        crate::guards::burst_trigger(g, gd, dt, aim, ok && armed, ok, None);
        g.gd[gd].aim.track = tr;
    } else if ok && armed && g.gd[gd].fire_cd <= 0.0 && !crate::guards::round_past_colleague(g, gd, aim, None, None) && crate::weapons::firearm_fire(g, w) {
        let mo = crate::suppression::guard_fire_mods(g, gd);
        let row = g.m.weapons.weapons.get(def).unwrap();
        let (r, cd, jitter) = (row.ai_spread, row.cd, row.cd_jitter);
        let die = crate::math_dice::sim_random(g);
        g.gd[gd].fire_cd = (cd + die * jitter) * mo.gap;
        let hands = (r.base + m::max(0.0, dist - r.range_from) * r.range_k) * (match g.gd[gd].skill { Some(sk) => sk.spread_k, None => 1.0 });
        let opts = if g.tune.aim_model {
            crate::combat::FireOpts { steady: Some(true), spread: Some(hands * v3::lerp(p.unsettled_k, 1.0, v3::clamp(lay, 0.0, 1.0)) + mo.spread), ..Default::default() }
        } else {
            crate::combat::FireOpts { spread: Some(mo.spread), ..Default::default() }
        };
        crate::combat::fire_weapon(g, c, aim, false, Some(&opts));
        if crate::weapons::firearm_rounds_ready(g, w) == 0.0 { crate::guards::start_reload_beat(g, gd); }
    }
    let out = had - crate::weapons::firearm_rounds_ready(g, w);
    if out > 0.0 {
        if !js::truthy(stand!(g, gd).fired) {
            let line = alloc::format!("{} fires through {}", g.gd[gd].callsign, g.dr[d].def.name);
            log(g, &line);
        }
        let st = stand!(g, gd);
        st.n += out;
        st.fired += out;
        g.stats.doorforce.shots += out as i32;
    }
    Some(held)
}

/// kickStep's `end`: the boot is over, thrown or given up, and the door is left alone a while.
fn kick_end(g: &mut Game, gd: GuardId, man: &mut Man, d: DoorId, b: Option<Belief>, why: Option<&str>) -> Option<KickStep> {
    if let Some(why) = why {
        g.stats.doorforce.no_kick += 1;
        let line = alloc::format!("{} gives the boot at {} up: {}", g.gd[gd].callsign, g.dr[d].def.name, why);
        log(g, &line);
    }
    let s = state!(g);
    if s.kickers.get(d) == Some(&gd) { s.kickers.delete(d); }
    if let Some(b) = b { g.gd[gd].last_known = Some([b.at[0], 0.0, b.at[2]]); }
    man.kick = None;
    man.on = None;
    man.beliefs.delete(d);
    man.rest.set(d, g.time + g.k.doorforce.rest_secs);
    let row = &mut g.gd[gd];
    row.path = Vec::new();
    row.path_goal = None;
    row.stall_ref = None;
    None
}

/// The walk to the boot's spot, the set, the boot. None = it is over: the caller's own branch goes on. (doorforce.ts kickStep)
pub fn kick_step(g: &mut crate::game::Game, gd: GuardId, m: &mut crate::doorforce::Man, dt: f64) -> Option<crate::doorforce::KickStep> {
    let kk = m.kick.unwrap();
    let d = kk.door;
    let c = g.gd[gd].ch;
    let b = m.beliefs.get(d).copied();
    let (hinge, closed, width) = (g.dr[d].def.hinge, g.dr[d].def.closed_dir, g.dr[d].def.width);
    let lock: V3 = [hinge[0] + crate::m::cos(closed) * (width - 0.2), 0.9, hinge[1] + crate::m::sin(closed) * (width - 0.2)];
    if kk.phase != KickPhase::Kick && !shut(g, d) { return kick_end(g, gd, m, d, b, Some("it stands open")); }
    if kk.phase == KickPhase::Walk {
        if g.time - kk.t0 > g.k.doorforce.walk_secs { return kick_end(g, gd, m, d, b, Some("the walk took too long")); }
        crate::guards::go_to(g, gd, kk.spot, None);
        let speed = g.tune.guard_investigate;
        let arrived = crate::guards::follow_path(g, gd, dt, speed);
        let dist = flat(g.ch[c].pos, kk.spot);
        if dist < 0.3 || (arrived && dist < 0.6) {
            let k = m.kick.as_mut().unwrap();
            k.phase = KickPhase::Set;
            k.set_t = g.time;
            g.gd[gd].path = Vec::new();
            g.gd[gd].path_goal = None;
        } else if arrived {
            return kick_end(g, gd, m, d, b, Some("no way to the spot"));
        }
        return Some(KickStep { aim: lock, move_: g.gd[gd].speed });
    }
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    // on the spot: square to the leaf as the spot was dealt, his eyes on the lock
    let on_it = flat(g.ch[c].pos, kk.spot) < 0.6;
    if kk.phase == KickPhase::Set {
        if !on_it { return kick_end(g, gd, m, d, b, Some("he was moved off the spot")); }
        let pos = g.ch[c].pos;
        if crate::doors::doors_boot_holds(g, d, pos) { return kick_end(g, gd, m, d, b, Some("the frame would hold it from here")); }
        let yawed = v3::approach_angle(g.ch[c].body_yaw, kk.yaw, 6.0 * dt);
        g.ch[c].body_yaw = yawed;
        let square = crate::m::abs(v3::wrap_angle(kk.yaw - yawed)) < 0.3;
        if square && g.gd[gd].speed < 0.3 {
            crate::character::anim_kick_door(g, c, None);
            m.kick.as_mut().unwrap().phase = KickPhase::Kick;
        } else if g.time - kk.set_t > 1.5 {
            return kick_end(g, gd, m, d, b, Some("never got square to it"));
        }
        return Some(KickStep { aim: lock, move_: 0.0 });
    }
    // the sole meets the leaf at KICK_IMPACT, and only a man still on his spot has a leaf under it
    if !(crate::character::anim_kick_impact(g, c) || crate::character::anim_kick_time(g, c) >= crate::character::KICK_IMPACT || !crate::character::anim_kicking(g, c)) {
        return Some(KickStep { aim: lock, move_: 0.0 });
    }
    if !on_it { return kick_end(g, gd, m, d, b, Some("he was moved off the spot")); }
    let (id, pos, time) = (g.ch[c].id, g.ch[c].pos, g.time);
    g.stats.doorforce.kicks += 1;
    state!(g).boots.set(crate::Num(id), time);
    let gave = shut(g, d) && crate::doors::doors_kick_in(g, d, pos, id, None);
    if gave {
        g.stats.doorforce.gave += 1;
        let pos = g.ch[c].pos;
        crate::marks::kick_scuff(g, d, pos);
        crate::doors::door_set_noticed(g, d, true);   // (the keep his own side tore out is no puzzle to a colleague on a calm pass later)
    }
    let line = alloc::format!("{} boots {}: {}", g.gd[gd].callsign, g.dr[d].def.name, if gave { "it goes in" } else { "it holds" });
    log(g, &line);
    kick_end(g, gd, m, d, b, None)
}

/// QA: what `gd` believes of `d`. (doorforce.ts doorForceBelief)
pub fn door_force_belief(g: &mut crate::game::Game, gd: GuardId, d: DoorId) -> Option<crate::doorforce::DoorForceBelief> {
    let time = g.time;
    let b = g.m.doorforce.states.as_ref()?.men.get(gd)?.beliefs.get(d)?;
    Some(DoorForceBelief { sure: b.sure, at: b.at, seen: b.seen, age: time - b.t })
}

/// QA: what `gd` is doing about a door this frame. (doorforce.ts doorForceOn)
pub fn door_force_on(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::doorforce::DoorForceOn> {
    let row = g.m.doorforce.states.as_ref()?.men.get(gd)?;
    if let Some(k) = row.kick {
        let kick = match k.phase { KickPhase::Walk => DoorForceOnKick::Walk, KickPhase::Set => DoorForceOnKick::Set, KickPhase::Kick => DoorForceOnKick::Kick };
        return Some(DoorForceOn { door: k.door, fired: row.on.map_or(0.0, |o| o.fired), kick: Some(kick) });
    }
    row.on.map(|o| DoorForceOn { door: o.door, fired: o.fired, kick: None })
}

/// QA: hand `gd` the record a real sighting of his foe at `at` leaves, `age` seconds old. (doorforce.ts doorForceSaw)
pub fn door_force_saw(g: &mut crate::game::Game, gd: GuardId, at: V3, age: Option<f64>) {
    let age = age.unwrap_or(0.0);
    let t = g.time - age;
    man_of(g, gd).saw = Some(ManSaw { at: [at[0], 0.0, at[2]], t, used: false });
}

/// A restart or a checkpoint load: forget everything about this game. (doorforce.ts doorForceReset)
pub fn door_force_reset(g: &mut crate::game::Game) { g.m.doorforce.states = None; }

/// Panel and console line. (doorforce.ts doorForceSummary)
pub fn door_force_summary(g: &mut crate::game::Game) -> String {
    let s = &g.stats.doorforce;
    let last = match g.m.doorforce.doorforce_log.last() { Some(l) => alloc::format!(" · last: {}", l), None => String::new() };
    alloc::format!("an alerted man at a shut door ON ▸ stands {} (heard {}, seen {}) · rounds through {} (held for a friend {}) · boots {} (gave {}; not thrown {}){}", s.stands, s.heard, s.seen, s.shots, s.held_friend, s.kicks, s.gave, s.no_kick, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Leaked: a test's game lives as long as the test binary.
    fn game() -> &'static mut Game {
        let g = Box::leak(Box::new(Game::blank()));
        g.ai_enabled = true;
        g.touring = false;
        g
    }

    #[test]
    fn a_new_man_keeps_the_literals_long_ago() {
        let g = game();
        let row = man_of(g, GuardId(3));
        assert_eq!(row.served, -1e9);
        assert_eq!(row.cf_t, -1e9);
        row.served = 4.0;   // (the row itself: a write through the answer stays)
        assert_eq!(state_of(g).men.get(GuardId(3)).unwrap().served, 4.0);
    }

    #[test]
    fn word_adds_up_to_three_a_sighting_is_a_floor_and_a_rested_door_hears_nothing() {
        let g = game();
        g.time = 10.0;
        let (gd, d) = (GuardId(1), DoorId(2));
        learn(g, gd, d, [1.0, 5.0, 2.0], 1.25, false);
        learn(g, gd, d, [1.5, 5.0, 2.5], 1.25, false);
        let b = *man_of(g, gd).beliefs.get(d).unwrap();
        assert_eq!((b.sure, b.at, b.t, b.seen), (2.5, [1.5, 0.0, 2.5], 10.0, false));
        learn(g, gd, d, [1.5, 0.0, 2.5], 1.25, false);
        assert_eq!(man_of(g, gd).beliefs.get(d).unwrap().sure, 3.0);
        learn(g, gd, d, [1.5, 0.0, 2.5], 1.5, true);
        let b = *man_of(g, gd).beliefs.get(d).unwrap();
        assert_eq!((b.sure, b.seen), (3.0, true));

        let rested = DoorId(4);
        man_of(g, gd).rest.set(rested, 10.5);
        learn(g, gd, rested, [0.0; 3], 1.0, false);
        assert!(man_of(g, gd).beliefs.get(rested).is_none());
        g.time = 10.5;   // (rest > now is what stops it: at its last moment he hears again)
        learn(g, gd, rested, [0.0; 3], 1.0, false);
        assert!(man_of(g, gd).beliefs.get(rested).is_some());
    }

    #[test]
    fn dropping_a_man_frees_only_the_door_he_holds() {
        let g = game();
        let (a, b, d) = (GuardId(1), GuardId(2), DoorId(7));
        man_of(g, a).kick = Some(Kick { door: d, spot: [0.0; 3], yaw: 0.0, phase: KickPhase::Walk, t0: 0.0, set_t: 0.0 });
        state_of(g).kickers.set(d, b);
        drop(g, a);
        assert!(!state_of(g).men.has(a));
        assert_eq!(state_of(g).kickers.get(d), Some(&b));
        man_of(g, b).kick = Some(Kick { door: d, spot: [0.0; 3], yaw: 0.0, phase: KickPhase::Walk, t0: 0.0, set_t: 0.0 });
        drop(g, b);
        assert!(state_of(g).kickers.get(d).is_none());
    }

    #[test]
    fn a_clock_gone_back_forgets_every_belief() {
        let g = game();
        g.time = 20.0;
        door_force_saw(g, GuardId(1), [1.0, 9.0, 2.0], Some(0.5));
        let saw = man_of(g, GuardId(1)).saw.unwrap();
        assert_eq!((saw.at, saw.t, saw.used), ([1.0, 0.0, 2.0], 19.5, false));
        state_of(g).last_t = 20.0;
        g.time = 20.0 - 1e-7;   // (a step inside the tolerance is no load)
        assert!(state_of(g).men.has(GuardId(1)));
        g.time = 5.0;
        assert!(state_of(g).men.is_empty());
        assert_eq!(state_of(g).last_t, 5.0);
    }

    #[test]
    fn the_tick_lets_old_boots_go_and_drops_men_off_the_roster() {
        let g = game();
        g.time = 10.0;
        man_of(g, GuardId(9));
        state_of(g).boots.set(crate::Num(1.0), 7.5);
        state_of(g).boots.set(crate::Num(2.0), 8.0);
        door_force_tick(g, 1.0 / 60.0);
        assert!(!state_of(g).men.has(GuardId(9)));
        assert!(!state_of(g).boots.has(crate::Num(1.0)));
        assert!(state_of(g).boots.has(crate::Num(2.0)));   // (g.time - t > 2: exactly two seconds stays)
        assert_eq!(state_of(g).last_t, 10.0);
    }

    #[test]
    fn the_log_halves_past_four_hundred() {
        let g = game();
        for _ in 0..401 { log(g, "x"); }
        assert_eq!(g.m.doorforce.doorforce_log.len(), 201);
        assert_eq!(g.m.doorforce.doorforce_log[0], "0.00 x");
    }
}
