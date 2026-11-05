//! src/game/squad40.ts lines 1 to 259
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// a row of this id is on the menu's list (`S.defs.some(d => d.id === id)`)
fn def_listed(g: &Game, id: &str) -> bool {
    g.orders.defs.iter().any(|d| g.tab.orders_order_def[*d].id == id)
}

/// the same 47 fields, from `LentBack` to `TurnBack` and back (a `Turn` extends a `Lent`: see notes, Skeleton)
macro_rules! same_back {
    ($to:path, $b:expr) => {{
        let b = $b;
        $to {
            post: b.post, going: b.going, order: b.order, face: b.face, there: b.there, t0: b.t0, follow: b.follow, watch: b.watch, door: b.door,
            strafe: b.strafe, stance: b.stance, upper: b.upper, stack_side: b.stack_side, move_: b.move_, set_line: b.set_line, group: b.group,
            set_said: b.set_said, goal: b.goal, arrived: b.arrived, held_set: b.held_set, held_fire: b.held_fire, arr_t: b.arr_t, moved_t: b.moved_t,
            light: b.light, low: b.low, yield_t: b.yield_t, face_now: b.face_now, search: b.search, search_t: b.search_t, via: b.via, anchor: b.anchor,
            mark: b.mark, mark_t: b.mark_t, had: b.had, back_n: b.back_n, shy: b.shy, thru: b.thru, via_for: b.via_for, via_t: b.via_t, pace: b.pace,
            spare: b.spare, from: b.from, foe: b.foe, crouch: b.crouch, lean: b.lean, fit: b.fit, tries: b.tries,
        }
    }};
}

/// the `Lent` a `Turn` extends, for `squadHandBack`
fn lent_of(w: Turn) -> crate::squadthrow::Lent {
    crate::squadthrow::Lent { mates: w.mates, gd: w.gd, t: w.t, a: w.a, home: w.home, home_yaw: w.home_yaw, back: w.back.map(|b| same_back!(crate::squadthrow::LentBack, b)) }
}

/// `{ ...squadLend(g, S, gd, t), at, t0, firedT, shot }`
fn turn_of(l: crate::squadthrow::Lent, at: V3, t0: f64, fired_t: f64, shot: Shot) -> Turn {
    Turn { at, t0, fired_t, shot, mates: l.mates, gd: l.gd, t: l.t, a: l.a, home: l.home, home_yaw: l.home_yaw, back: l.back.map(|b| same_back!(TurnBack, b)) }
}

/// the `at` every kind of target carries
fn at_of(t: &crate::orders::OrderTarget) -> V3 {
    match t {
        crate::orders::OrderTarget::Floor(f) => f.at,
        crate::orders::OrderTarget::Door(f) => f.at,
        crate::orders::OrderTarget::None(f) => f.at,
        crate::orders::OrderTarget::Foe(f) => f.at,
    }
}

/// A TEAMMATE'S 40 MM WORKS IN THIS BUILD: settled on the page. (squad40.ts squad40Here)
pub fn squad40_here(g: &mut crate::game::Game) -> bool { true }

/// the launcher is under the gun in his hands: a long gun whose row has one. (squad40.ts tubeUp)
pub fn tube_up(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let d = g.fa[g.gd[gd].weapon].def;
    let r = g.m.weapons.weapons.get(d).unwrap();
    r.cls != crate::weapons::WeaponClass::Pistol && r.pellets.is_none() && r.launcher != Some(false)
}

/// the rounds counted over a state in hand (`squad40Left`'s walk)
fn left_in(g: &Game, s: &State) -> f64 {
    let mut n = 0.0;
    for (gd, k) in s.rounds.iter() {
        if g.gd[gd].state != crate::game::GuardState::Dead { n += *k; }
    }
    n
}

/// the rounds the squad's men on their feet have between them. (squad40.ts squad40Left)
pub fn squad40_left(g: &mut crate::game::Game) -> f64 {
    let key = crate::seats::view_key(g);
    let g: &Game = g;
    match g.m.squad40.states.get(key) { Some(s) => left_in(g, s), None => 0.0 }
}

/// what this staging brought. (squad40.ts squad40Brought)
pub fn squad40_brought(g: &mut crate::game::Game) -> f64 {
    let key = crate::seats::view_key(g);
    g.m.squad40.states.get(key).map_or(0.0, |s| s.brought)
}

/// one man's rounds left. (squad40.ts squad40Of)
pub fn squad40_of(g: &mut crate::game::Game, gd: GuardId) -> f64 {
    let key = crate::seats::view_key(g);
    g.m.squad40.states.get(key).and_then(|s| s.rounds.get(gd).copied()).unwrap_or(0.0)
}

/// the words the throws' row adds while the order is on the page. (squad40.ts squad40Word)
pub fn squad40_word(g: &mut crate::game::Game) -> String {
    let key = crate::seats::view_key(g);
    let brought = g.m.squad40.states.get(key).map(|s| s.brought);
    match brought {
        Some(b) if b > 0.0 => format!(" · 40 mm {}", squad40_left(g)),
        _ => String::new(),
    }
}

/// A scenario has just staged its team: each man's rounds are what his card carries. (squad40.ts squad40Stage)
pub fn squad40_stage(g: &mut crate::game::Game, carried: &[(GuardId, f64)]) {
    let mut rounds: JsMap<GuardId, f64> = JsMap::new();
    let mut n = 0.0;
    for &(gd, c) in carried {
        let k = if tube_up(g, gd) { m::max(0.0, js::or(m::floor(c), 0.0)) } else { 0.0 };
        if js::truthy(k) { rounds.set(gd, k); n += k; }
    }
    let key = crate::seats::view_key(g);
    if g.m.squad40.states.get(key).is_none() {
        if !js::truthy(n) { return; }
        g.m.squad40.states.set(key, State { rounds: JsMap::new(), brought: 0.0, last: JsMap::new(), turns: JsMap::new(), noted: false, shown: None, kept: JsMap::new() });
    }
    let s = g.m.squad40.states.get_mut(key).unwrap();
    s.rounds = rounds;
    s.brought = n;
    s.last = JsMap::new();
    s.turns = JsMap::new();
    s.shown = None;
    s.kept.clear();
    g.stats.squad40.staged += 1;
}

/// the bodies a round stops on, as far as the player can see them. (squad40.ts bodies)
pub fn bodies(g: &mut crate::game::Game) -> crate::squad40::Bodies {
    let (mut friends, mut foes, mut mates): (Vec<CharId>, Vec<CharId>, Vec<CharId>) = (Vec::new(), Vec::new(), Vec::new());
    let wanted = g.snatch.man;
    let hostages = g.rescue.hostages.clone();
    if !g.pl[g.player].down { friends.push(g.pl[g.player].ch); }
    for gd in g.guards.clone() {
        let c = g.gd[gd].ch;
        let up = g.gd[gd].state != crate::game::GuardState::Dead && g.ch[c].alive;
        if g.gd[gd].team == crate::teams::Team::Operative {
            if up { mates.push(c); }
        } else if up && !crate::fogofwar::fog_of_war_hidden(g, gd) {
            let team = g.gd[gd].team;
            if crate::teams::is_hostile(g, crate::teams::Team::Operative, team) && g.gd[gd].civ.is_none() && Some(gd) != wanted && !hostages.contains(&gd) {
                foes.push(c);
            } else {
                friends.push(c);
            }
        }
    }
    for cv in crate::civilians::civilian_bodies(g) {
        let c = g.gd[cv].ch;
        if g.ch[c].alive && !crate::fogofwar::fog_of_war_hidden(g, cv) { friends.push(c); }
    }
    Bodies { friends, foes, mates }
}

/// he faces the spot. (squad40.ts onIt)
pub fn on_it(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool {
    let c = g.gd[gd].ch;
    m::abs(v3::wrap_angle(g.ch[c].aim_yaw - v3::yaw_to(g.ch[c].pos, at))) < g.k.squad40.aim_rad
}

/// his gun is UP and on the spot: in a fight, or under an order that has it at his shoulder. (squad40.ts gunOn)
pub fn gun_on(g: &mut crate::game::Game, gd: GuardId, at: V3) -> bool {
    if !on_it(g, gd, at) { return false; }
    if g.gd[gd].state == crate::game::GuardState::Alert { return true; }
    if let Some(o) = crate::roomfight::room_fight_op_of(g, gd) {
        if g.tab.roomfight_op[o].tgt.is_some() { return true; }
    }
    match crate::orders::squad_orders_of(g, gd) {
        Some(a) => g.tab.orders_assignment[a].upper == crate::character::UpperMode::Aim,
        None => false,
    }
}

/// ONE MAN'S SHOT to a spot, from where he stands with his gun turned to it (`Close`: not armed there; None: he has none). (squad40.ts shotOf)
pub fn shot_of(g: &mut crate::game::Game, gd: GuardId, at: V3, b: Option<&crate::squad40::Bodies>) -> Option<crate::squad40::ShotOf> {
    let made;
    let bb: &Bodies = match b { Some(x) => x, None => { made = bodies(g); &made } };
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let d = v3::dist_xz(p, at);
    if d < 1e-3 { return Some(ShotOf::Close); }
    let (muzzle_up, head_drop, muzzle_ahead, near_m, arm_slack) = (g.k.squad40.muzzle_up, g.k.squad40.head_drop, g.k.squad40.muzzle_ahead, g.k.squad40.near_m, g.k.squad40.arm_slack);
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied();
    let up = m::max(0.6, m::min(muzzle_up, match head { Some(h) => h[1] - p[1] - head_drop, None => muzzle_up }));
    let k = muzzle_ahead / d;
    let from: V3 = [p[0] + (at[0] - p[0]) * k, p[1] + up, p[2] + (at[2] - p[2]) * k];
    let to: V3 = [at[0], at[1] + 0.03, at[2]];
    if crate::scene_collision::solid_blocked(g, [p[0], p[1] + up, p[2]], from, None) { return None; }
    let short = if d < g.k.he40.arm_m { Some(ShotOf::Close) } else { None };
    let mut friends: Vec<CharId> = bb.friends.clone();
    friends.extend(bb.mates.iter().copied().filter(|m| *m != c));
    let mut all = friends.clone();
    all.extend(bb.foes.iter().copied());
    let sol = crate::he40::shell_solve(g, from, to, Some(&all[..]));
    if v3::dist(sol.land, to) > near_m {
        let clear = !bb.foes.is_empty() && v3::dist(crate::he40::shell_solve(g, from, to, Some(&friends[..])).land, to) <= near_m;
        if !clear { return short; }
    }
    if crate::he40::arc_length(&sol) < g.k.he40.arm_m + arm_slack { return Some(ShotOf::Close); }
    let burst = v3::copy(sol.land);
    let aimed = gun_on(g, gd, at);
    Some(ShotOf::Shot(Shot { gd, from, to, sol, burst, aimed }))
}

/// who of ours the burst would have in it (None: nobody), asked of where it goes off AND of the spot. (squad40.ts caught)
pub fn caught(g: &mut crate::game::Game, a: &crate::squad40::Shot, at: V3) -> Option<String> {
    let c = g.gd[a.gd].ch;
    let moved = crate::squadthrow::SquadFragCatchesMoved { gd: a.gd, at: g.ch[c].pos };
    let burst = crate::squadthrow::SquadFragCatchesBurst { reach: g.k.frag.wound_m + g.k.squad40.reach_slack, no: String::from("no 40 mm.") };
    crate::squadthrow::squad_frag_catches(g, a.burst, at, Some(moved), Some(true), Some(false), Some(&burst))
}

/// `q` of plan's key: a place on a quarter-metre grid (`${n}` of a rounded number prints -0 as 0)
fn grid_key(p: V3) -> String {
    format!("{},{}", m::round(p[0] * 4.0) as i64, m::round(p[2] * 4.0) as i64)
}

/// `toFixed(2)` of plan's key (-0 prints as 0; see notes, Decisions)
fn fixed2(x: f64) -> String { format!("{:.2}", x + 0.0) }

/// `ready` of plan: a launcher in a free hand, not turning already, the tube closed on his last round
fn ready(g: &mut Game, gd: GuardId) -> bool {
    if !tube_up(g, gd) || !crate::squadsmoke::squad_hand_free(g, gd) || crate::squadthrow::hands_busy(g, gd) { return false; }
    let key = crate::seats::view_key(g);
    let (turning, t) = match g.m.squad40.states.get(key) {
        Some(s) => (s.turns.has(gd), s.last.get(gd).copied().unwrap_or(-1e9)),
        None => (false, -1e9),
    };
    !turning && !(g.time >= t && g.time - t < g.k.launcher40.cd)
}

/// the key plan's kept answers are good for: the spot, the men asked, and everybody of ours
fn plan_key(g: &mut Game, men: &[GuardId], at: V3) -> String {
    let mut ms: Vec<String> = Vec::with_capacity(men.len());
    for &man in men {
        let c = g.gd[man].ch;
        let (id, pos) = (g.ch[c].id, g.ch[c].pos);
        let on = if gun_on(g, man, at) { 1 } else { 0 };
        let rd = if ready(g, man) { 1 } else { 0 };
        let key = crate::seats::view_key(g);
        let rounds = g.m.squad40.states.get(key).and_then(|s| s.rounds.get(man).copied()).unwrap_or(0.0);
        ms.push(format!("{}:{},{}{}{}", id, grid_key(pos), on, rd, rounds));
    }
    let me = if g.pl[g.player].down { String::new() } else { grid_key(g.ch[g.pl[g.player].ch].pos) };
    let ours: Vec<String> = g.guards.iter().map(|&m| if g.gd[m].team == crate::teams::Team::Operative { grid_key(g.ch[g.gd[m].ch].pos) } else { String::new() }).collect();
    format!("{},{}|{}|{}|{}", fixed2(at[0]), fixed2(at[2]), ms.join(";"), me, ours.join(";"))
}

/// `keep` of plan: an asker's answer kept with its key, and handed on
fn keep(g: &mut Game, asker: Option<Asker>, key: String, a: StateKeptValueA, who: Option<String>) -> Plan {
    if let Some(k) = asker {
        let (t, row) = (g.time, crate::seats::view_key(g));
        if let Some(s) = g.m.squad40.states.get_mut(row) {
            s.kept.set(k, StateKeptValue { key, t, a: a.clone(), who: who.clone() });
        }
    }
    match a {
        StateKeptValueA::Text(x) => Plan::Text(x),
        StateKeptValueA::Shot(x) => Plan::Arm1(PlanArm1 { a: x, who }),
    }
}

/// THE SHOT AS IT WOULD BE ORDERED NOW: the man of `men` who would fire it, one whose gun is on the spot first, then the nearest; else why nobody has one. (squad40.ts plan)
pub fn plan(g: &mut crate::game::Game, men: &[GuardId], at: V3, asker: Option<crate::squad40::Asker>) -> crate::squad40::Plan {
    let row = crate::seats::view_key(g);
    if g.m.squad40.states.get(row).is_none() || squad40_left(g) <= 0.0 { return Plan::Text(String::from(none)); }
    let key = plan_key(g, men, at);
    if let Some(k) = asker {
        let now = g.time;
        if let Some(c) = g.m.squad40.states.get(row).and_then(|s| s.kept.get(k)) {
            if c.key == key && now >= c.t && now - c.t < 0.3 {
                return match &c.a {
                    StateKeptValueA::Text(x) => Plan::Text(x.clone()),
                    StateKeptValueA::Shot(x) => Plan::Arm1(PlanArm1 { a: x.clone(), who: c.who.clone() }),
                };
            }
        }
    }
    let bb = bodies(g);
    let (mut any, mut asked, mut close) = (false, false, false);
    let mut dirty: Option<(Shot, String)> = None;
    let mut order: Vec<GuardId> = men.to_vec();
    js::sort_by(&mut order, |x, y| {
        let (x, y) = (*x, *y);
        let by_gun = (if gun_on(g, y, at) { 1.0 } else { 0.0 }) - (if gun_on(g, x, at) { 1.0 } else { 0.0 });
        js::or(by_gun, v3::dist_xz(g.ch[g.gd[x].ch].pos, at) - v3::dist_xz(g.ch[g.gd[y].ch].pos, at))
    });
    for gd in order {
        let has = g.m.squad40.states.get(row).and_then(|s| s.rounds.get(gd).copied()).map_or(false, |k| k > 0.0);
        if g.gd[gd].state == crate::game::GuardState::Dead || !has { continue; }
        any = true;
        if !ready(g, gd) { continue; }
        if !gun_on(g, gd, at) && !crate::squadthrow::may_walk(g, gd) { continue; }
        asked = true;
        let a = match shot_of(g, gd, at, Some(&bb)) {
            Some(ShotOf::Close) => { close = true; continue; }
            None => continue,
            Some(ShotOf::Shot(a)) => a,
        };
        if let Some(who) = caught(g, &a, at) {
            if dirty.is_none() { dirty = Some((a, who)); }
            continue;
        }
        return keep(g, asker, key, StateKeptValueA::Shot(a), None);
    }
    if let Some((a, who)) = dirty { return keep(g, asker, key, StateKeptValueA::Shot(a), Some(who)); }
    let text: &str = if !any { crate::words::wg(g, &"nobody's carrying one.", &"none with them.") } else if !asked { "not ready." } else if close { "too close to arm." } else { "no shot from here." };
    keep(g, asker, key, StateKeptValueA::Text(String::from(text)), None)
}

/// the round leaves his tube; `pl` is the row of `states` it is counted on. (squad40.ts fire)
pub fn fire(g: &mut crate::game::Game, pl: PlayerId, a: &mut crate::squad40::Shot, at: V3, turned: bool) {
    let gd = a.gd;
    let now = g.time;
    let s = g.m.squad40.states.get_mut(pl).unwrap();
    let left = s.rounds.get(gd).copied().unwrap_or(0.0);
    s.rounds.set(gd, m::max(0.0, left - 1.0));
    s.last.set(gd, now);
    s.kept.clear();
    let c = g.gd[gd].ch;
    crate::he40::fire_shell(g, c, a.from, a.to);
    crate::character::anim_fire(g, c);
    crate::game::game_say(g, gd, "forty out!", None, None, None);
    let callsign = g.gd[gd].callsign.clone();
    let t = g.time;
    g.m.squad40.states.get_mut(pl).unwrap().shown = Some(StateShown { t, text: format!("{} · 40 mm out", callsign) });
    g.stats.squad40.fired += 1;
    if turned { g.stats.squad40.turned += 1; }
    let left = squad40_left(g);
    let log = &mut g.m.squad40.squad40_log;
    log.push(Squad40ModSQUAD40LOGItem { t, who: callsign, from: a.from, at: v3::copy(at), burst: a.burst, left, turned });
    if log.len() > 100 { log.drain(0..50); }
}

/// `end` of turnTick: he gives up (`line`) or is done; his turn is over and he goes back
fn end_turn(g: &mut Game, gd: GuardId, line: Option<&str>) {
    if let Some(l) = line {
        crate::game::game_say(g, gd, l, None, None, None);
        g.stats.squad40.gave_up += 1;
    }
    let key = crate::seats::view_key(g);
    let w = g.m.squad40.states.get_mut(key).and_then(|s| s.turns.delete(gd)).unwrap();
    crate::squadthrow::squad_hand_back(g, &mut lent_of(w), Some(true));
}

/// A MAN TURNING TO HIS SHOT, each frame (the row's `tick`). (squad40.ts turnTick)
pub fn turn_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let key = crate::seats::view_key(g);
    let now = g.time;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    let w = g.m.squad40.states.get(key).and_then(|s| s.turns.get(gd)).map(|w| (w.a, w.fired_t, w.t0, w.at));
    let (fired_t, t0, at) = match w {
        Some((wa, fired_t, t0, at)) if wa == Some(a) => (fired_t, t0, at),
        _ => {
            // (under the row's id with no turn of ours behind it: he holds where he is)
            if w.is_some() { g.m.squad40.states.get_mut(key).unwrap().turns.delete(gd); }
            let face = g.ch[c].body_yaw;
            crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
                order: String::from("hold"),
                post: Some([p[0], 0.0, p[2]]),
                face: Some(face),
                stance: Some(crate::character::Stance::LowReady),
                upper: Some(crate::character::UpperMode::None),
                going: String::from("holding"),
                there: String::from("holding"),
                set_line: None,
                ..Default::default()
            });
            return;
        }
    };
    if fired_t >= 0.0 {
        if now < fired_t || now - fired_t >= g.k.squad40.after_secs { end_turn(g, gd, None); }
        return;
    }
    if now < t0 || now - t0 > g.k.squad40.aim_secs { return end_turn(g, gd, Some("no shot from here.")); }
    if !crate::squadsmoke::squad_hand_free(g, gd) || crate::squadthrow::hands_busy(g, gd) || !tube_up(g, gd) { return; }
    if !on_it(g, gd, at) || now - t0 < g.k.squad40.raise_secs { return; }
    // the order's own sense, asked of the floor as it stands NOW: nothing is spent on a refusal
    let has = g.m.squad40.states.get(key).and_then(|s| s.rounds.get(gd).copied()).map_or(false, |k| k > 0.0);
    if !has { return end_turn(g, gd, Some(none)); }
    let mut shot = match shot_of(g, gd, at, None) {
        None => return end_turn(g, gd, Some("no shot from here.")),
        Some(ShotOf::Close) => return end_turn(g, gd, Some("too close to arm.")),
        Some(ShotOf::Shot(x)) => x,
    };
    if let Some(who) = caught(g, &shot, at) {
        g.stats.squad40.refused += 1;
        return end_turn(g, gd, Some(&who));
    }
    fire(g, key, &mut shot, at, true);
    if let Some(w) = g.m.squad40.states.get_mut(key).and_then(|s| s.turns.get_mut(gd)) { w.fired_t = now; }
}

/// the arm `OrderLabel::Shell40`: the row's words
pub fn row_label(g: &mut Game) -> String { format!("40 mm there · {} left", squad40_left(g)) }

/// the arm `OrderOnly::Shell40`: the order is one man's, the man who fires
pub fn row_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    match plan(g, men, at_of(t), Some(Asker::Only)) {
        Plan::Text(_) => Vec::new(),
        Plan::Arm1(a) => vec![a.a.gd],
    }
}

/// the arm `OrderValid::Shell40`: None is the TypeScript's `true`, else the refusal's words
pub fn row_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Option<String> {
    match plan(g, men, at_of(t), Some(Asker::Valid)) {
        Plan::Text(x) => Some(x),
        Plan::Arm1(a) => a.who,
    }
}

/// the arm `OrderIssue::Shell40`: a refusal's words, or None (the floor as it stands NOW: nothing is spent on a refusal)
pub fn row_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let at = at_of(t);
    let mut a = match plan(g, men, at, None) {
        Plan::Text(x) => { g.stats.squad40.refused += 1; return Some(x); }
        Plan::Arm1(PlanArm1 { who: Some(who), .. }) => { g.stats.squad40.refused += 1; return Some(who); }
        Plan::Arm1(PlanArm1 { a, who: None }) => a,
    };
    let gd = a.gd;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    crate::squadthrow::squad_log_row(g, gd, SHELL_THERE, at);
    if a.aimed { let key = crate::seats::view_key(g); fire(g, key, &mut a, at, false); return None; }
    let to = v3::copy(a.to);
    let now = g.time;
    let lent = crate::squadthrow::squad_lend(g, gd, t);
    let mut w = turn_of(lent, v3::copy(at), now, -1.0, a);
    crate::orders::squad_orders_assign(g, gd, &crate::orders::SquadOrdersAssignA {
        order: String::from(SHELL_THERE),
        post: Some([p[0], 0.0, p[2]]),
        face: Some(v3::yaw_to(p, at)),
        watch: Some(to),
        stance: Some(crate::character::Stance::None),
        upper: Some(crate::character::UpperMode::Aim),
        going: String::from("turning to fire · 40 mm"),
        there: String::from("aiming · 40 mm"),
        set_line: None,
        ..Default::default()
    });
    w.a = crate::orders::squad_orders_of(g, gd);
    let key = crate::seats::view_key(g);
    let s = g.m.squad40.states.get_mut(key).unwrap();
    s.turns.set(gd, w);
    s.kept.clear();
    None
}

/// the order's row on the throws' page. (squad40.ts row)
pub fn row(g: &mut crate::game::Game) -> OrdersOrderDefId {
    g.tab.orders_order_def.push(crate::orders::OrderDef {
        id: String::from(SHELL_THERE),
        page: Some(String::from(crate::squadthrow::THROW_PAGE)),
        targets: vec![crate::orders::TargetKind::Floor],
        free: Some(true),
        label: crate::orders::OrderLabel::Shell40,
        only: Some(crate::orders::OrderOnly::Shell40),
        valid: Some(crate::orders::OrderValid::Shell40),
        issue: crate::orders::OrderIssue::Shell40,
        tick: Some(crate::orders::OrderTick::Shell40),
        ..Default::default()
    })
}

/// the squad's HUD block: what is left while this staging brought any, and the shot just made. (squad40.ts notes)
pub fn notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> {
    let key = crate::seats::view_key(g);
    let (brought, shown) = match g.m.squad40.states.get(key) { Some(s) => (s.brought, s.shown.clone()), None => return Vec::new() };
    if !js::truthy(brought) || !def_listed(g, SHELL_THERE) { return Vec::new(); }
    let left = crate::orders::OrderNote::Text(format!("40 mm {}", squad40_left(g)));
    match shown {
        Some(w) if g.time >= w.t && g.time - w.t < g.k.squad40.shown_secs => vec![crate::orders::OrderNote::Text(w.text), left],
        _ => vec![left],
    }
}

/// Once a frame straight after the menu's own input: the row kept in step with what this staging brought and with the throws' page, a turn whose man is down or under another order ended. (squad40.ts squad40Tick)
pub fn squad40_tick(g: &mut crate::game::Game) {
    let key = crate::seats::view_key(g);
    if g.m.squad40.states.get(key).is_none() { return; }
    let mut at = 0;
    while let Some(gd) = g.m.squad40.states.get(key).unwrap().turns.next_key(&mut at) {
        let (wa, t0) = { let w = g.m.squad40.states.get(key).unwrap().turns.get(gd).unwrap(); (w.a, w.t0) };
        if g.gd[gd].state == crate::game::GuardState::Dead || crate::orders::squad_orders_of(g, gd) != wa {
            g.m.squad40.states.get_mut(key).unwrap().turns.delete(gd);
        } else if g.time < t0 || g.time - t0 > g.k.squad40.aim_secs + 1.0 {
            let w = g.m.squad40.states.get_mut(key).unwrap().turns.delete(gd).unwrap();
            crate::squadthrow::squad_hand_back(g, &mut lent_of(w), Some(true));
        }
    }
    let brought = g.m.squad40.states.get(key).unwrap().brought;
    let has = def_listed(g, SHELL_THERE);
    let want = brought > 0.0 && def_listed(g, crate::squadthrow::THROW_OPEN);
    if has == want { return; }
    if g.orders.menu.is_some() { crate::orders::squad_orders_close(g); }
    if !want { crate::orders::squad_orders_unregister(g, SHELL_THERE); return; }
    let def = row(g);
    crate::orders::squad_orders_register(g, def, None);
    // (`s.noted !== S`: a row is one view's and a view's SquadOrders is made once, as the game stands: game.ts:1373, squadcast.ts:25)
    let s = g.m.squad40.states.get_mut(key).unwrap();
    if !s.noted {
        s.noted = true;
        g.orders.notes.push(crate::orders::OrderNoteFn::Squad40);
    }
}

/// `of` of squad40Plans
fn plan_of(g: &Game, a: &Shot, preview: Option<bool>) -> Squad40Plan {
    let p = g.ch[g.gd[a.gd].ch].pos;
    Squad40Plan { gd: a.gd, from: [p[0], 0.0, p[2]], points: a.sol.points.clone(), land: a.burst, kind: Squad40PlanKind::Shell, preview, walk: None }
}

/// WHAT THE PICTURE DRAWS: for a man turning to his shot until it goes, and a preview for the highlighted row while its page is up. Reads only. (squad40.ts squad40Plans)
pub fn squad40_plans(g: &mut crate::game::Game) -> Vec<crate::squad40::Squad40Plan> {
    let mut out: Vec<Squad40Plan> = Vec::new();
    let key = crate::seats::view_key(g);
    let keys = match g.m.squad40.states.get(key) { Some(s) => s.turns.keys(), None => return out };
    for k in keys {
        let (w_gd, wa, fired_t, shot) = { let w = g.m.squad40.states.get(key).unwrap().turns.get(k).unwrap(); (w.gd, w.a, w.fired_t, w.shot.clone()) };
        if fired_t < 0.0 && g.gd[w_gd].state != crate::game::GuardState::Dead && crate::orders::squad_orders_of(g, w_gd) == wa {
            out.push(plan_of(g, &shot, None));
        }
    }
    let asked = match &g.orders.menu {
        Some(m) => match &m.target {
            crate::orders::OrderTarget::Floor(f) if m.sel >= 0.0 && m.rows.get(m.sel as usize).map_or(false, |d| g.tab.orders_order_def[*d].id == SHELL_THERE) => Some((m.men.clone(), f.at)),
            _ => None,
        },
        None => None,
    };
    if let Some((men, at)) = asked {
        if let Plan::Arm1(a) = plan(g, &men, at, None) {
            if a.who.is_none() { out.push(plan_of(g, &a.a, Some(true))); }
        }
    }
    out
}

/// the line for the tools. (squad40.ts squad40Summary)
pub fn squad40_summary(g: &mut crate::game::Game) -> String {
    let key = crate::seats::view_key(g);
    let brought = g.m.squad40.states.get(key).map(|s| s.brought);
    match brought {
        Some(b) if js::truthy(b) => format!("{} of {} left · X on a spot of floor, G, then 3 · fired {}", squad40_left(g), b, g.stats.squad40.fired),
        _ => String::from("none brought (no teammate's card carries a 40 mm grenade)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grid_key_prints_as_the_typescripts_template() {
        // (`${Math.round(x)}` prints -0 as "0", and Math.round takes a half up)
        assert_eq!(grid_key([-0.1, 0.0, 0.125]), "0,1");
        assert_eq!(grid_key([-0.125, 0.0, -0.13]), "0,-1");
        assert_eq!(grid_key([2.5, 0.0, -3.0]), "10,-12");
    }

    #[test]
    fn the_spot_in_the_key_never_says_minus_zero() {
        assert_eq!(fixed2(-0.0), "0.00");
        assert_eq!(fixed2(1.5), "1.50");
        assert_eq!(fixed2(-2.25), "-2.25");
    }
}
