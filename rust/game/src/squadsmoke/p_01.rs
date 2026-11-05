//! src/game/squadsmoke.ts lines 1 to 160
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `x.toFixed(1)`: on an exact tie (only an odd multiple of a quarter can be one) the larger digit, where `{:.1}` takes the even; −0 has no sign, a small negative keeps it.
fn to_fixed_1(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    let a = m::abs(x);
    if a == f64::INFINITY { return String::from(if x < 0.0 { "-Infinity" } else { "Infinity" }); }
    let sign = if x < 0.0 { "-" } else { "" };
    let q = a * 4.0;
    if q == m::floor(q) && js::rem(q, 2.0) == 1.0 {
        let n = m::floor(a * 10.0) + 1.0;   // (a tie's tenths end in .5, exactly: this is the digit above)
        let tenths = js::rem(n, 10.0);
        return format!("{}{}.{}", sign, (n - tenths) / 10.0, tenths);
    }
    format!("{}{:.1}", sign, a)
}

/// The spot of any target: every arm of the union has `at`.
fn target_at(t: &crate::orders::OrderTarget) -> V3 {
    match t {
        crate::orders::OrderTarget::Floor(x) => x.at,
        crate::orders::OrderTarget::Door(x) => x.at,
        crate::orders::OrderTarget::None(x) => x.at,
        crate::orders::OrderTarget::Foe(x) => x.at,
    }
}

/// The squad's smoke canisters left this run. (squadsmoke.ts squadSmokeLeft)
pub fn squad_smoke_left(g: &mut crate::game::Game) -> f64 {
    let key = crate::seats::view_key(g);
    match g.m.squadsmoke.states.get(key) { Some(s) => s.cans, None => 0.0 }
}

/// What this staging brought. (squadsmoke.ts squadSmokeBrought)
pub fn squad_smoke_brought(g: &mut crate::game::Game) -> f64 {
    let key = crate::seats::view_key(g);
    match g.m.squadsmoke.states.get(key) { Some(s) => s.brought, None => 0.0 }
}

/// A scenario has just staged its team: the squad's smoke is `cans`, and the row is on the menu only if that is one or more. (squadsmoke.ts squadSmokeStage)
pub fn squad_smoke_stage(g: &mut crate::game::Game, cans: f64) {
    let n = m::max(0.0, js::or(m::floor(cans), 0.0));
    let key = crate::seats::view_key(g);
    if !g.m.squadsmoke.states.has(key) {
        if !js::truthy(n) { return; }
        g.m.squadsmoke.states.set(key, State { brought: 0.0, cans: 0.0, noted: false, last: JsMap::new() });
    }
    {
        let s = g.m.squadsmoke.states.get_mut(key).unwrap();
        s.cans = n;
        s.brought = n;
        s.last = JsMap::new();
    }
    g.stats.squadsmoke.staged += 1;
    crate::orders::squad_orders_unregister(g, SMOKE_ORDER);
    if !js::truthy(n) { return; }
    let after = g.orders.defs.iter().position(|d| g.tab.orders_order_def[*d].id == "hold");
    let def = row(g);
    let before = match after {
        Some(a) => g.orders.defs.get(a + 1).map(|d| g.tab.orders_order_def[*d].id.clone()),
        None => None,
    };
    crate::orders::squad_orders_register(g, def, before.as_deref());
    let s = g.m.squadsmoke.states.get_mut(key).unwrap();
    if !s.noted {
        s.noted = true;
        g.orders.notes.push(crate::orders::OrderNoteFn::SquadSmoke);
    }
}

/// Where a can thrown from `from` on `sol` comes to rest on open floor: its landing and the run of its bounces. (squadsmoke.ts squadSmokeRest)
pub fn squad_smoke_rest(from: V3, sol: &crate::throwables::ThrowSolution, roll_k: Option<f64>) -> V3 {
    let roll_k = match roll_k { Some(r) => r, None => squadsmoke().roll_k };
    let v = sol.v0;
    let l = sol.land;
    let t = roll_k * m::sqrt(m::max(0.0, v[1] * v[1] + 2.0 * crate::throwables::GRAVITY * (from[1] - l[1])));
    [l[0] + v[0] * t, l[1], l[2] + v[2] * t]
}

/// Nothing between a can down at `a` and the spot `b` for its run along the floor. (squadsmoke.ts runClear)
pub fn run_clear(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    let d = v3::dist_xz(a, b);
    if d < 0.05 { return true; }
    let rd: V3 = [(b[0] - a[0]) / d, 0.0, (b[2] - a[2]) / d];
    let back = 0.12;
    for h in squadsmoke().clear_h {
        let ro: V3 = [a[0] - rd[0] * back, a[1] + h, a[2] - rd[2] * back];
        if crate::scene_collision::solid_blocked(g, ro, [b[0], a[1] + h, b[2]], None)
            || crate::scene_collision::raycast(g, ro, rd, d + back).is_some()
            || crate::props::prop_system_raycast(g, ro, rd, d + back).is_some()
        {
            return false;
        }
    }
    true
}

/// Has this man a hand for a throw right now. (squadsmoke.ts squadHandFree)
pub fn squad_hand_free(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let c = g.gd[gd].ch;
    !(g.gd[gd].state == crate::game::GuardState::Dead
        || g.ch[c].anim.lockpick
        || g.time < g.gd[gd].stagger_until
        || g.time < g.gd[gd].dazzled_until)
}

/// Where a throw on `s` rests, the ground under its landing shortening the run. (squadsmoke.ts squadArcTo, its `restOf`)
fn rest_of(g: &mut Game, from: V3, s: &crate::throwables::ThrowSolution, roll_k: f64) -> V3 {
    let bog = crate::snow_bog::snow_bog_at(g, s.land[0], 0.0, s.land[2]);
    let run = crate::snow_bog::bog_run(g, bog);
    squad_smoke_rest(from, s, Some(roll_k * run))
}

/// One man's throw to a spot: the hand it leaves, the arc that brings the thing to REST on the spot, where it rests. None: he has none. (squadsmoke.ts squadArcTo)
pub fn squad_arc_to(g: &mut crate::game::Game, gd: GuardId, at: V3, roll_k: Option<f64>, stood: Option<V3>) -> Option<crate::squadsmoke::SquadArcTo> {
    let t = squadsmoke();
    let roll_k = match roll_k { Some(r) => r, None => t.roll_k };
    let to: V3 = [at[0], at[1] + 0.05, at[2]];
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    let p = match stood { Some(s) => s, None => pos };
    let d = v3::dist_xz(p, at);
    let a = if d > 1e-3 { t.hand_ahead / d } else { 0.0 };
    let head = g.ch[c].bones.get(crate::character::Bone::Head).copied();
    let up = m::max(0.6, m::min(t.hand_up, match head { Some(h) => h[1] - pos[1] - t.head_drop, None => t.hand_up }));
    let from: V3 = [p[0] + (at[0] - p[0]) * a, p[1] + up, p[2] + (at[2] - p[2]) * a];
    if crate::scene_collision::solid_blocked(g, [p[0], p[1] + up, p[2]], from, None) { return None; }
    let mut aim = to;
    let mut sol = crate::throwables::throwables_solve(g, from, aim, Some(t.speed));
    let mut rest = rest_of(g, from, &sol, roll_k);
    for _ in 0..2 {
        aim = [aim[0] + at[0] - rest[0], to[1], aim[2] + at[2] - rest[2]];
        sol = crate::throwables::throwables_solve(g, from, aim, Some(t.speed));
        rest = rest_of(g, from, &sol, roll_k);
    }
    let l = sol.land;
    if l[1] - at[1] > t.floor_m || v3::dist_xz(rest, at) > t.near_m { return None; }
    if !run_clear(g, l, at) { return None; }
    Some(SquadArcTo { from, sol, rest })
}

/// The man of `men` who would throw it, with his throw: the nearest to the spot with an arc to rest on it. (squadsmoke.ts squadSmokeThrower)
pub fn squad_smoke_thrower(g: &mut crate::game::Game, men: &[GuardId], at: V3) -> Option<crate::squadsmoke::SquadSmokeThrower> {
    let again = squadsmoke().again_secs;
    let key = crate::seats::view_key(g);
    let mut order: Vec<GuardId> = men.to_vec();
    js::sort_by(&mut order, |a, b| {
        v3::dist_xz(g.ch[g.gd[*a].ch].pos, at) - v3::dist_xz(g.ch[g.gd[*b].ch].pos, at)
    });
    for gd in order {
        if !squad_hand_free(g, gd) { continue; }
        if let Some(s) = g.m.squadsmoke.states.get(key) {
            let last = match s.last.get(gd) { Some(t) => *t, None => -1e9 };
            if g.time - last < again { continue; }
        }
        if let Some(arc) = squad_arc_to(g, gd, at, None, None) {
            return Some(SquadSmokeThrower { gd, from: arc.from, sol: arc.sol, rest: arc.rest });
        }
    }
    None
}

/// The order's row on the menu: its four functions are the arms `Smoke` of orders' enums, whose bodies are `row_smoke_*` below. (squadsmoke.ts row)
pub fn row(g: &mut crate::game::Game) -> OrdersOrderDefId {
    let def = crate::orders::OrderDef {
        id: String::from(SMOKE_ORDER),
        hot_key: Some(squadsmoke().order_key),
        label: crate::orders::OrderLabel::Smoke,
        targets: vec![crate::orders::TargetKind::Floor],
        free: Some(true),
        only: Some(crate::orders::OrderOnly::Smoke),
        valid: Some(crate::orders::OrderValid::Smoke),
        issue: crate::orders::OrderIssue::Smoke,
        ..crate::orders::OrderDef::zeroed()
    };
    g.tab.orders_order_def.push(def)
}

/// The row's words: the arm `OrderLabel::Smoke`. (squadsmoke.ts:131 label)
pub(crate) fn row_smoke_label(g: &mut Game) -> String {
    format!("smoke there · {} left", squad_smoke_left(g))
}

/// The order is one man's, the thrower's: the arm `OrderOnly::Smoke`. (squadsmoke.ts:133 only)
pub(crate) fn row_smoke_only(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> Vec<GuardId> {
    match squad_smoke_thrower(g, men, target_at(t)) { Some(th) => vec![th.gd], None => Vec::new() }
}

/// Ok is true, Err(Some) the words: the arm `OrderValid::Smoke`. (squadsmoke.ts:134 valid)
pub(crate) fn row_smoke_valid(g: &mut Game, t: &crate::orders::OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    if squad_smoke_left(g) <= 0.0 { return Err(Some(String::from("no smoke left."))); }
    if squad_smoke_thrower(g, men, target_at(t)).is_some() { Ok(()) } else { Err(Some(String::from("no throw from here."))) }
}

/// The throw: one can off the count, on its arc from the nearest man's hand, and a row in the squad's log. None: given. The arm `OrderIssue::Smoke`. (squadsmoke.ts:135 issue)
pub(crate) fn row_smoke_issue(g: &mut Game, men: &[GuardId], t: &crate::orders::OrderTarget, o: Option<&crate::orders::GiveOpts>) -> Option<String> {
    let key = crate::seats::view_key(g);
    let at = target_at(t);
    let has_cans = match g.m.squadsmoke.states.get(key) { Some(s) => s.cans > 0.0, None => false };
    let th = if has_cans { squad_smoke_thrower(g, men, at) } else { None };
    let th = match th { Some(th) => th, None => return Some(String::from("no throw from here.")) };
    let now = g.time;
    {
        let s = g.m.squadsmoke.states.get_mut(key).unwrap();
        s.cans -= 1.0;
        s.last.set(th.gd, now);
    }
    let c = g.gd[th.gd].ch;
    let opts = crate::throwables::ThrowablesSpawnOpts {
        who: Some(g.ch[c].id),
        heading: Some(v3::yaw_of(th.sol.v0)),
        ..Default::default()
    };
    let it = crate::throwables::throwables_spawn(g, crate::throwables::ItemKind::Smoke, th.from, th.sol.v0, Some(opts));
    g.tab.item[it].on_bounce = Some(crate::throwables::ItemBounce::Can);
    crate::character::anim_toss_item(g, c, Some(crate::character::TOSS_QUICK));
    crate::game::game_say(g, th.gd, "smoke out.", None, None, None);
    let callsign = g.gd[th.gd].callsign.clone();
    let serial = g.serial();
    g.orders.log.push(crate::orders::SquadOrdersLogItem {
        t: g.time,
        order: String::from(SMOKE_ORDER),
        who: vec![callsign.clone()],
        target: format!("floor:{},{}", to_fixed_1(at[0]), to_fixed_1(at[2])),
        auto: None,
        laid: None,
        serial,
    });
    g.stats.squadsmoke.thrown += 1;
    let left = g.m.squadsmoke.states.get(key).unwrap().cans;
    let log = &mut g.m.squadsmoke.squadsmoke_log;
    log.push(SquadsmokeModSQUADSMOKELOGItem { t: g.time, who: callsign, from: th.from, at: v3::copy(at), land: v3::copy(th.sol.land), rest: th.rest, left });
    if log.len() > 100 { log.drain(0..50); }
    None
}

/// The squad's HUD block: what is left, while this staging brought any. (squadsmoke.ts notes)
pub fn notes(g: &mut crate::game::Game) -> Vec<crate::orders::OrderNote> {
    let key = crate::seats::view_key(g);
    match g.m.squadsmoke.states.get(key) {
        Some(s) if s.brought > 0.0 => vec![crate::orders::OrderNote::Text(format!("smoke {}", s.cans))],
        _ => Vec::new(),
    }
}

/// The row's key with a menu up that lists it: the press is that row's. True: the press was the row's. (squadsmoke.ts squadSmokeKey)
pub fn squad_smoke_key(g: &mut crate::game::Game, input: InputId) -> bool {
    let key = squadsmoke().order_key;
    let view = crate::seats::view_key(g);
    if g.orders.menu.is_none() || !g.m.squadsmoke.states.has(view) || !crate::input::input_hit(g, input, &key) { return false; }
    let id = SMOKE_ORDER;
    let i = match &g.orders.menu {
        Some(menu) => menu.rows.iter().position(|d| g.tab.orders_order_def[*d].id == id),
        None => None,
    };
    let i = match i { Some(i) => i, None => return false };
    g.tab.input[input].pressed.delete(key);
    crate::orders::squad_orders_pick(g, Some(i as f64));
    true
}

/// A line of what the squad's smoke stands at. (squadsmoke.ts squadSmokeSummary)
pub fn squad_smoke_summary(g: &mut crate::game::Game) -> String {
    let key = crate::seats::view_key(g);
    match g.m.squadsmoke.states.get(key) {
        Some(s) if js::truthy(s.brought) => format!("{} of {} left · {} with the menu up on a spot of floor", s.cans, s.brought, squadsmoke().order_key.replacen("Key", "", 1)),
        _ => String::from("none brought (no card carries smoke)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sol(v0: V3, land: V3) -> crate::throwables::ThrowSolution {
        crate::throwables::ThrowSolution { v0, land, ..crate::throwables::ThrowSolution::zeroed() }
    }

    #[test]
    fn rest_has_the_typescripts_bits() {
        for (from, v0, land, roll, want) in REST {
            let r = squad_smoke_rest(*from, &sol(*v0, *land), *roll);
            for i in 0..3 { assert_eq!(r[i].to_bits(), want[i], "{:?} {}", from, i); }
        }
    }

    #[test]
    fn to_fixed_is_the_languages() {
        for (x, want) in FIXED { assert_eq!(to_fixed_1(*x), *want, "{}", x); }
    }

    const REST: &[(V3, V3, V3, Option<f64>, [u64; 3])] = &[
        ([1.3, 1.6, -2.7], [3.1, 4.2, -5.5], [6.2, 0.02, -9.9], None, [0x401e134658ef2aa8, 0x3f947ae147ae147b, 0xc0287acd44fbcd7a]),
        ([0.0, 1.3, 0.0], [8.9, 1.1, 0.3], [7.77, 0.0, 0.33], None, [0x402526f89febef44, 0x0000000000000000, 0x3fdb2c73d8560c8e]),
        ([-4.25, 0.9, 11.1], [-1.7, -6.3, 8.8], [-5.5, 0.1, 12.34], Some(0.04), [0xc0180636b376779e, 0x3fb999999999999a, 0x402deb56c3d7aa74]),
        ([2.0, 0.5, 2.0], [0.1, -0.2, 0.3], [2.1, 3.0, 2.2], None, [0x4000cccccccccccd, 0x4008000000000000, 0x400199999999999a]),
        ([10.1, 1.25, 3.3], [2.2, 5.5, -1.1], [13.3, 0.035, 1.9], Some(0.02989), [0x402b91360ee13e27, 0x3fa1eb851eb851ec, 0x3ffa87f49147d430]),
    ];
    const FIXED: &[(f64, &str)] = &[
        (0.25, "0.3"),
        (-0.25, "-0.3"),
        (0.75, "0.8"),
        (0.35, "0.3"),
        (1.05, "1.1"),
        (-0.04, "-0.0"),
        (-0.0, "0.0"),
        (0.0, "0.0"),
        (12.75, "12.8"),
        (3.45, "3.5"),
        (1234.56, "1234.6"),
        (-7.25, "-7.3"),
        (0.05, "0.1"),
        (99.95, "100.0"),
        (2.5, "2.5"),
        (-1.75, "-1.8"),
        (100.25, "100.3"),
    ];
}
