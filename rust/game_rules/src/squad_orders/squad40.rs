pub use ::game_types::squad40::types;
pub use types::*;
/// this module's own order
pub const SHELL_THERE: &str = crate::orders::rows::shell40::ORDER.id;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// a row of this id is on the menu's list (`S.defs.some(d => d.id === id)`)
fn def_listed(g: &Game, seat: &crate::seats::SeatRef, id: &str) -> bool {
    seat.orders.defs.iter().any(|d| g.tab.orders_order_def[*d].id == id)
}

/// the `Lent` a `Turn` extends, for `squadHandBack`
fn lent_of(w: Turn) -> crate::squadthrow::Lent {
    crate::squadthrow::Lent { mates: w.mates, gd: w.gd, t: w.t, a: w.a, home: w.home, home_yaw: w.home_yaw, back: w.back }
}

/// `{ ...squadLend(g, S, gd, t), at, t0, firedT, shot }`
fn turn_of(l: crate::squadthrow::Lent, at: V3, t0: f64, fired_t: f64, shot: Shot) -> Turn {
    Turn { at, t0, fired_t, shot, mates: l.mates, gd: l.gd, t: l.t, a: l.a, home: l.home, home_yaw: l.home_yaw, back: l.back }
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

/// the launcher is under the gun in his hands: a long gun whose row has one. (squad40.ts tubeUp)
fn tube_up(g: &crate::game::Game, gd: GuardId) -> bool {
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
pub fn squad40_left(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> f64 {
    let key = crate::seats::view_key(g, seat);
    let g: &Game = g;
    match g.m.squad40.states.get(key) { Some(s) => left_in(g, s), None => 0.0 }
}

/// the words the throws' row adds while the order is on the page. (squad40.ts squad40Word)
pub(super) fn squad40_word(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> String {
    let key = crate::seats::view_key(g, seat);
    let brought = g.m.squad40.states.get(key).map(|s| s.brought);
    match brought {
        Some(b) if b > 0.0 => format!(" · 40 mm {}", js::num_str(squad40_left(g, seat))),
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
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    if g.m.squad40.states.get(key).is_none() {
        if !js::truthy(n) { return; }
        g.m.squad40.states.set(key, State { rounds: JsMap::new(), brought: 0.0, last: JsMap::new(), turns: JsMap::new(), shown: None, plan: None });
    }
    let s = g.m.squad40.states.get_mut(key).unwrap();
    s.rounds = rounds;
    s.brought = n;
    s.last = JsMap::new();
    s.turns = JsMap::new();
    s.shown = None;
    g.stats.squad40.staged += 1;
}

/// the bodies a round stops on, as far as the player can see them. (squad40.ts bodies)
fn bodies(g: &mut crate::game::Game) -> crate::squad40::Bodies {
    let (mut friends, mut foes, mut mates): (Vec<CharId>, Vec<CharId>, Vec<CharId>) = (Vec::new(), Vec::new(), Vec::new());
    let hostages = g.rescue.hostages.clone();
    if !g.pl[g.player].down { friends.push(g.pl[g.player].ch); }
    for gd in g.guards.clone() {
        let c = g.gd[gd].ch;
        let up = g.gd[gd].state != crate::game::GuardState::Dead && g.ch[c].alive;
        if g.gd[gd].team == crate::teams::Team::Operative {
            if up { mates.push(c); }
        } else if up && !crate::fogofwar::fog_of_war_hidden(g, gd) {
            let team = g.gd[gd].team;
            if crate::teams::is_hostile(g, crate::teams::Team::Operative, team) && g.gd[gd].civ.is_none() && !hostages.contains(&gd) {
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
fn on_it(g: &crate::game::Game, gd: GuardId, at: V3) -> bool {
    let c = g.gd[gd].ch;
    m::abs(v3::wrap_angle(g.ch[c].aim_yaw - v3::yaw_to(g.ch[c].pos, at))) < g.k.squad40.aim_rad
}

/// his gun is UP and on the spot: in a fight, or under an order that has it at his shoulder. (squad40.ts gunOn)
fn gun_on(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId, at: V3) -> bool {
    if !on_it(g, gd, at) { return false; }
    if g.gd[gd].state == crate::game::GuardState::Alert { return true; }
    if let Some(o) = crate::roomfight::room_fight_op_of(g, gd) {
        if g.tab.roomfight_op[o].tgt.is_some() { return true; }
    }
    match crate::orders::squad_orders_of(g, seat, gd) {
        Some(a) => g.tab.orders_assignment[a].upper == crate::character::UpperMode::Aim,
        None => false,
    }
}

/// ONE MAN'S SHOT to a spot, from where he stands with his gun turned to it (`Close`: not armed there; None: he has none). (squad40.ts shotOf)
fn shot_of(g: &mut crate::game::Game, gd: GuardId, at: V3, b: Option<&crate::squad40::Bodies>) -> Option<crate::squad40::ShotOf> {
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
    let aimed = gun_on(g, &crate::seats::here(g), gd, at);
    Some(ShotOf::Shot(Shot { gd, from, to, sol, burst, aimed }))
}

/// who of ours the burst would have in it (None: nobody), asked of where it goes off AND of the spot. (squad40.ts caught)
fn caught(g: &mut crate::game::Game, a: &crate::squad40::Shot, at: V3) -> Option<String> {
    let c = g.gd[a.gd].ch;
    let moved = crate::squadthrow::SquadFragCatchesMoved { gd: a.gd, at: g.ch[c].pos };
    let burst = crate::squadthrow::SquadFragCatchesBurst { reach: g.k.frag.wound_m + g.k.squad40.reach_slack, no: String::from("no 40 mm.") };
    crate::squadthrow::squad_frag_catches(g, a.burst, at, Some(moved), Some(true), Some(false), Some(&burst))
}

/// `ready` of plan: a launcher in a free hand, not turning already, the tube closed on his last round
fn ready(g: &mut Game, gd: GuardId) -> bool {
    if !tube_up(g, gd) || !crate::squadsmoke::squad_hand_free(g, gd) || crate::squadthrow::hands_at_work(g, gd) { return false; }
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let (turning, t) = match g.m.squad40.states.get(key) {
        Some(s) => (s.turns.has(gd), s.last.get(gd).copied().unwrap_or(-1e9)),
        None => (false, -1e9),
    };
    !turning && !(g.time >= t && g.time - t < g.k.launcher40.cd)
}

/// THE SHOT AS IT WOULD BE ORDERED NOW: the man of `men` who would fire it, one whose gun is on the spot first, then the nearest; else why nobody has one. Asked each time,
/// by the row, the picture (`follow`) and the order. (squad40.ts plan)
pub(super) fn plan(g: &mut crate::game::Game, men: &[GuardId], at: V3) -> crate::squad40::ShellPlan {
    let of = |shot: Option<Shot>, no: Option<String>| ShellPlan { at, men: men.to_vec(), shot, no };
    let row = crate::seats::view_key(g, &crate::seats::here(g));
    if g.m.squad40.states.get(row).is_none() || squad40_left(g, &crate::seats::here(g)) <= 0.0 { return of(None, Some(String::from(none))); }
    let bb = bodies(g);
    let (mut any, mut asked, mut close) = (false, false, false);
    let mut dirty: Option<(Shot, String)> = None;
    let mut order: Vec<GuardId> = men.to_vec();
    js::sort_by(&mut order, |x, y| {
        let (x, y) = (*x, *y);
        let by_gun = (if gun_on(g, &crate::seats::here(g), y, at) { 1.0 } else { 0.0 }) - (if gun_on(g, &crate::seats::here(g), x, at) { 1.0 } else { 0.0 });
        js::or(by_gun, v3::dist_xz(g.ch[g.gd[x].ch].pos, at) - v3::dist_xz(g.ch[g.gd[y].ch].pos, at))
    });
    for gd in order {
        let has = g.m.squad40.states.get(row).and_then(|s| s.rounds.get(gd).copied()).map_or(false, |k| k > 0.0);
        if g.gd[gd].state == crate::game::GuardState::Dead || !has { continue; }
        any = true;
        if !ready(g, gd) { continue; }
        if !gun_on(g, &crate::seats::here(g), gd, at) && !crate::squadthrow::may_walk(g, &crate::seats::here(g), gd) { continue; }
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
        return of(Some(a), None);
    }
    if let Some((a, who)) = dirty { return of(Some(a), Some(who)); }
    let text: &str = if !any { crate::words::wg(g, &"nobody's carrying one.", &"none with them.") } else if !asked { "not ready." } else if close { "too close to arm." } else { "no shot from here." };
    of(None, Some(String::from(text)))
}

/// the shell's row lit on the open menu: the men it is asked of, and the spot
pub fn lit(g: &Game, seat: &crate::seats::SeatRef) -> Option<(Vec<GuardId>, V3)> {
    let mn = seat.orders.menu.as_ref()?;
    let crate::orders::OrderTarget::Floor(t) = &mn.target else { return None };
    if !(mn.sel >= 0.0) || g.tab.orders_order_def[*mn.rows.get(mn.sel as usize)?].id != SHELL_THERE { return None; }
    Some((mn.men.clone(), t.at))
}

/// once a frame with a menu up: the lit row's shot as it is now, for whoever draws it
fn follow(g: &mut Game) {
    let seat = crate::seats::here(g);
    let (key, asked) = (crate::seats::view_key(g, &seat), lit(g, &seat));
    let now = asked.map(|(men, at)| plan(g, &men, at));
    if let Some(s) = g.m.squad40.states.get_mut(key) { s.plan = now; }
}

/// the round leaves his tube; `pl` is the row of `states` it is counted on. (squad40.ts fire)
fn fire(g: &mut crate::game::Game, pl: PlayerId, a: &crate::squad40::Shot, at: V3, turned: bool) {
    let gd = a.gd;
    let now = g.time;
    let s = g.m.squad40.states.get_mut(pl).unwrap();
    let left = s.rounds.get(gd).copied().unwrap_or(0.0);
    s.rounds.set(gd, m::max(0.0, left - 1.0));
    s.last.set(gd, now);
    let c = g.gd[gd].ch;
    crate::he40::fire_shell(g, c, a.from, a.to);
    crate::character::anim_fire(g, c);
    crate::game::game_say(g, gd, "forty out!", None, None);
    let callsign = g.gd[gd].callsign.clone();
    let t = g.time;
    g.m.squad40.states.get_mut(pl).unwrap().shown = Some(StateShown { t, text: format!("{} · 40 mm out", callsign) });
    g.stats.squad40.fired += 1;
    if turned { g.stats.squad40.turned += 1; }
    let left = squad40_left(g, &crate::seats::here(g));
    let log = &mut g.m.squad40.squad40_log;
    log.push(Squad40ModSQUAD40LOGItem { burst: a.burst });
    if log.len() > 100 { log.drain(0..50); }
}

/// `end` of turnTick: he gives up (`line`) or is done; his turn is over and he goes back
fn end_turn(g: &mut Game, gd: GuardId, line: Option<&str>) {
    if let Some(l) = line {
        crate::game::game_say(g, gd, l, None, None);
        g.stats.squad40.gave_up += 1;
    }
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let w = g.m.squad40.states.get_mut(key).and_then(|s| s.turns.delete(gd)).unwrap();
    crate::squadthrow::squad_hand_back(g, &mut lent_of(w), Some(true));
}

/// A MAN TURNING TO HIS SHOT, each frame (the row's `tick`). (squad40.ts turnTick)
pub fn turn_tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
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
                order: String::from(crate::orders::rows::hold::ORDER.id),
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
    if !crate::squadsmoke::squad_hand_free(g, gd) || crate::squadthrow::hands_at_work(g, gd) || !tube_up(g, gd) { return; }
    if !on_it(g, gd, at) || now - t0 < g.k.squad40.raise_secs { return; }
    // the order's own sense, asked of the floor as it stands NOW: nothing is spent on a refusal
    let has = g.m.squad40.states.get(key).and_then(|s| s.rounds.get(gd).copied()).map_or(false, |k| k > 0.0);
    if !has { return end_turn(g, gd, Some(none)); }
    let shot = match shot_of(g, gd, at, None) {
        None => return end_turn(g, gd, Some("no shot from here.")),
        Some(ShotOf::Close) => return end_turn(g, gd, Some("too close to arm.")),
        Some(ShotOf::Shot(x)) => x,
    };
    if let Some(who) = caught(g, &shot, at) {
        g.stats.squad40.refused += 1;
        return end_turn(g, gd, Some(&who));
    }
    fire(g, key, &shot, at, true);
    if let Some(w) = g.m.squad40.states.get_mut(key).and_then(|s| s.turns.get_mut(gd)) { w.fired_t = now; }
}

/// the order's `issue` (orders/rows/shell40.rs): a refusal's words, or None (the floor as it stands NOW: nothing is spent on a refusal)
pub(super) fn row_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Option<String> {
    let at = at_of(t);
    let a = match plan(g, men, at) {
        ShellPlan { no: Some(why), .. } => { g.stats.squad40.refused += 1; return Some(why); }
        ShellPlan { shot: Some(a), .. } => a,
        ShellPlan { shot: None, no: None, .. } => return None,
    };
    let gd = a.gd;
    let c = g.gd[gd].ch;
    let p = g.ch[c].pos;
    crate::squadthrow::squad_log_row(g, gd, SHELL_THERE, at);
    if a.aimed { let key = crate::seats::view_key(g, &crate::seats::here(g)); fire(g, key, &a, at, false); return None; }
    let to = v3::copy(a.to);
    let now = g.time;
    let lent = crate::squadthrow::squad_lend(g, &crate::seats::here(g), gd, t);
    let mut w = turn_of(lent, v3::copy(at), now, -1.0, a);
    crate::orders::rows::assign(g, &crate::orders::rows::shell40::ORDER, gd, crate::orders::SquadOrdersAssignA {
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
    w.a = crate::orders::squad_orders_of(g, &crate::seats::here(g), gd);
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let s = g.m.squad40.states.get_mut(key).unwrap();
    s.turns.set(gd, w);
    None
}

/// the order's row, put on the throws' page. (squad40.ts row)
fn list(g: &mut crate::game::Game) {
    crate::orders::rows::list(g, &crate::orders::rows::shell40::ORDER, crate::orders::OrderDef {
        page: Some(String::from(crate::squadthrow::THROW_PAGE)),
        targets: vec![crate::orders::TargetKind::Floor],
        free: Some(true),
        ..Default::default()
    });
}

/// the squad's HUD block: what is left while this staging brought any, and the shot just made. (squad40.ts notes)
pub fn notes(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    let Some(st) = g.m.squad40.states.get(crate::seats::view_key(g, seat)) else { return Vec::new() };
    if !js::truthy(st.brought) || !def_listed(g, seat, SHELL_THERE) { return Vec::new(); }
    let plain = |text: String| crate::view::Note { key: String::new(), text };
    let left = plain(format!("40 mm {}", js::num_str(squad40_left(g, seat))));
    match &st.shown {
        Some(w) if g.time >= w.t && g.time - w.t < g.k.squad40.shown_secs => vec![plain(w.text.clone()), left],
        _ => vec![left],
    }
}

/// Once a frame straight after the menu's own input: the row kept in step with what this staging brought and with the throws' page, a turn whose man is down or under another order ended. (squad40.ts squad40Tick)
pub fn squad40_tick(g: &mut crate::game::Game) {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    if g.m.squad40.states.get(key).is_none() { return; }
    let mut at = 0;
    while let Some(gd) = g.m.squad40.states.get(key).unwrap().turns.next_key(&mut at) {
        let (wa, t0) = { let w = g.m.squad40.states.get(key).unwrap().turns.get(gd).unwrap(); (w.a, w.t0) };
        if g.gd[gd].state == crate::game::GuardState::Dead || crate::orders::squad_orders_of(g, &crate::seats::here(g), gd) != wa {
            g.m.squad40.states.get_mut(key).unwrap().turns.delete(gd);
        } else if g.time < t0 || g.time - t0 > g.k.squad40.aim_secs + 1.0 {
            let w = g.m.squad40.states.get_mut(key).unwrap().turns.delete(gd).unwrap();
            crate::squadthrow::squad_hand_back(g, &mut lent_of(w), Some(true));
        }
    }
    let brought = g.m.squad40.states.get(key).unwrap().brought;
    let has = def_listed(g, &crate::seats::here(g), SHELL_THERE);
    let want = brought > 0.0 && def_listed(g, &crate::seats::here(g), crate::squadthrow::THROW_OPEN);
    if has == want { follow(g); return; }
    if g.orders.menu.is_some() { crate::orders::squad_orders_close(g); }
    if !want { crate::orders::rows::unlist(g, &crate::orders::rows::shell40::ORDER); return; }
    list(g);
}
