//! src/game/silent.ts lines 1 to 270
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// `LOUD_ROWS.includes(id)`   (silent.ts:244, :245)
fn is_loud_row(id: &str) -> bool { loud_rows().iter().any(|r| r == id) }

/// a state as the TypeScript's literal makes it   (silent.ts:121, :255)
fn new_state(cast: Vec<GuardId>, since: f64) -> State {
    State { cast, silent: false, since, hot: false, was: None, booted: None }
}

/// the roster's own test for one man   (silent.ts mate)
pub fn mate(g: &mut crate::game::Game, gd: GuardId) -> bool { g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some() }

/// the squad of this seat   (silent.ts castOf)
pub fn cast_of(g: &mut crate::game::Game) -> Vec<GuardId> { crate::orders::squad_orders_squad(g) }

/// is the squad silent right now   (silent.ts squadSilent)
pub fn squad_silent(g: &mut crate::game::Game) -> bool {
    if !g.k.silent.on { return false; }
    let key = crate::seats::view_key(g);
    match g.m.silent.states.get(key) {
        Some(s) if s.silent => crate::orders::squad_orders_active(g),
        _ => false,
    }
}

/// put the squad in silent or take it out   (silent.ts setSilent)
pub fn set_silent(g: &mut crate::game::Game, on: bool, by: Option<crate::silent::By>) {
    let by = by.unwrap_or(By::Api);
    let key = crate::seats::view_key(g);
    if !g.m.silent.states.has(key) {
        let cast = cast_of(g);
        let since = g.time;
        g.m.silent.states.set(key, new_state(cast, since));
    }
    let now = g.time;
    let s = g.m.silent.states.get_mut(key).unwrap();
    if s.silent == on { return; }
    s.silent = on; s.since = now; s.hot = false; s.was = Some(false); s.booted = None;
    if on { g.stats.silent.silent += 1; } else { g.stats.silent.loud += 1; }
    let to = if on { SilentModSILENTLOGItemTo::Silent } else { SilentModSILENTLOGItemTo::Loud };
    g.m.silent.silent_log.push(SilentModSILENTLOGItem { t: now, to, by });
    if g.m.silent.silent_log.len() > 100 { g.m.silent.silent_log.drain(0..50); }
}

/// this teammate is under the silent posture this frame   (silent.ts silentMan)
pub fn silent_man(g: &mut crate::game::Game, gd: GuardId) -> bool { squad_silent_of_view(g, gd) && mate(g, gd) }

/// squadSilent(viewOf(g, gd)): one book is the game in hand, as viewOf says; else asked inside his seat   (silent.ts:127, seats.ts:244)
fn squad_silent_of_view(g: &mut Game, gd: GuardId) -> bool {
    if crate::seats::one_book(g) { return squad_silent(g); }
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| squad_silent(g))
}

/// the speed his legs may be asked for   (silent.ts silentPace)
pub fn silent_pace(g: &mut crate::game::Game, gd: GuardId, speed: f64) -> f64 {
    if !silent_man(g, gd) {
        let cap = crate::unseen::unseen_pace(g, gd);
        return match cap {
            Some(c) if speed > c && !crate::frag::frag_running(g, gd) => c,
            _ => speed,
        };
    }
    let pace = g.k.silent.pace;
    if !(pace > 0.0) || speed <= pace || crate::frag::frag_running(g, gd) { return speed; }
    pace
}

/// loud has reached this man since the order   (silent.ts reached)
pub fn reached(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let key = crate::seats::view_key(g);
    let since = match g.m.silent.states.get(key) { Some(s) => s.since, None => return false };
    crate::roomfight::room_fight_hot_for(g, gd) && crate::roomfight::room_fight_hot_of(g, gd) >= since
}

/// a hostage's life is on it: a captor of a live rescue at his last warning   (silent.ts urgent)
pub fn urgent(g: &mut crate::game::Game) -> bool {
    let mut i = 0;
    while i < g.doors.list.len() {
        let d = g.doors.list[i];
        if crate::rescue::rescue_last_warning(g, d) { return true; }
        i += 1;
    }
    false
}

/// a walking footfall's base level; None: his usual   (silent.ts silentStep)
pub fn silent_step(g: &mut crate::game::Game, gd: GuardId) -> Option<f64> {
    if !silent_man(g, gd) { return crate::unseen::unseen_step(g, gd); }
    let own = if g.k.silent.step > 0.0 {
        match crate::sneak::sneak_step(g, gd) { Some(x) => Some(x), None => Some(g.k.silent.step) }
    } else { None };
    let c = g.gd[gd].ch;
    let low = if g.ch[c].anim.crouch > 0.5 { crate::unseen::unseen_step(g, gd) } else { None };
    match (low, own) {
        (None, _) => own,
        (Some(_), None) => low,
        (Some(l), Some(o)) => Some(m::min(o, l)),
    }
}

/// his weapon light or torch is held off   (silent.ts silentDark)
pub fn silent_dark(g: &mut crate::game::Game, gd: GuardId) -> bool { if silent_man(g, gd) { true } else { crate::unseen::unseen_dark(g, gd) } }

/// his line goes over the net   (silent.ts silentHush)
pub fn silent_hush(g: &mut crate::game::Game, gd: GuardId) -> bool { if silent_man(g, gd) { true } else { crate::unseen::unseen_hush(g, gd) } }

/// he eases a door   (silent.ts silentEase)
pub fn silent_ease(g: &mut crate::game::Game, gd: GuardId) -> bool { if silent_man(g, gd) { true } else { crate::unseen::unseen_ease(g, gd) } }

/// the boot is held back; counted   (silent.ts silentNoBoot)
pub fn silent_no_boot(g: &mut crate::game::Game) -> bool {
    if !squad_silent(g) || urgent(g) { return false; }
    g.stats.silent.locked += 1;
    true
}

/// the lock is worked instead of the boot   (silent.ts silentPick)
pub fn silent_pick(g: &mut crate::game::Game) -> bool { squad_silent(g) && !urgent(g) }

/// a door no pick opens, before a squad compromised since the order: the boot goes   (silent.ts silentBoot)
pub fn silent_boot(g: &mut crate::game::Game, d: DoorId) -> Option<crate::silent::SilentBoot> {
    if !squad_silent(g) { return None; }
    let key = crate::seats::view_key(g);
    let was = match g.m.silent.states.get(key) { Some(s) => s.was == Some(true), None => false };
    if !was || urgent(g) { return None; }
    let s = g.m.silent.states.get_mut(key).unwrap();
    let said = s.booted.get_or_insert_with(JsSet::new);
    if said.has(d) { return Some(SilentBoot::Again); }
    said.add(d);
    g.stats.silent.booted += 1;
    Some(SilentBoot::First)
}

/// the canisters stay in the kit   (silent.ts silentNoCans)
pub fn silent_no_cans(g: &mut crate::game::Game) -> bool { squad_silent(g) }

/// is his gun quiet: by the weapon's own report   (silent.ts quietGun)
pub fn quiet_gun(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let def = g.fa[g.gd[gd].weapon].def;
    let r = &g.m.weapons.weapons.get(def).unwrap().report;
    !r.loud && r.level < 1.0
}

/// the order's own hold on a silent man's round at `h`, nothing counted   (silent.ts ownHold)
pub fn own_hold(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if reached(g, gd) || quiet_gun(g, gd) { return false; }
    let st = g.gd[h].state;
    !(st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search || g.gd[h].awareness >= g.k.silent.noticed || urgent(g))
}

/// his round at `h` is held   (silent.ts silentHolds)
pub fn silent_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId) -> bool {
    if !silent_man(g, gd) {
        if !crate::holdfire::hold_fire_loud(g) {
            let (noticed, quiet) = (g.k.silent.noticed, quiet_gun(g, gd));
            if crate::unseen::unseen_holds(g, gd, h, noticed, quiet, &mut urgent) { return true; }
        }
        let (noticed, quiet) = (g.k.silent.noticed, quiet_gun(g, gd));
        return crate::holdfire::hold_fire_holds(g, gd, h, noticed, quiet, &mut urgent);
    }
    if !own_hold(g, gd, h) { return false; }
    g.stats.silent.held += 1;
    true
}

/// the man the room fight has him on is one the order will not let him shoot   (silent.ts silentOnMan)
pub fn silent_on_man(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !silent_man(g, gd) { return false; }
    let op = match crate::roomfight::room_fight_op_of(g, gd) { Some(op) => op, None => return false };
    let h = match g.tab.roomfight_op[op].tgt { Some(h) => h, None => return false };
    g.gd[h].state != crate::game::GuardState::Dead && own_hold(g, gd, h)
}

/// his round at `h` would have to go through standing glass to reach `to`: held   (silent.ts silentGlassHolds)
pub fn silent_glass_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId, c_2: CharId, to: V3) -> bool {
    let sil = silent_man(g, gd);
    let uns = !sil && !crate::holdfire::hold_fire_loud(g) && crate::unseen::unseen_glass(g, gd);
    let out = if sil { reached(g, gd) } else { !uns && !crate::holdfire::hold_fire_glass(g, gd) };
    if out { return false; }
    let st = g.gd[h].state;
    if st == crate::game::GuardState::Alert || st == crate::game::GuardState::Search || urgent(g) { return false; }
    let from = crate::guards::shot_origin(g, c_2);
    if !crate::glasspen::glass_on_line(g, from, to) { return false; }
    if sil { g.stats.silent.held_glass += 1; }
    else if uns { crate::unseen::unseen_glass_held(g); }
    else { crate::holdfire::hold_fire_glass_held(g); }
    true
}

/// a loud tool is being used: a silent squad is loud from here, with one line   (silent.ts silentTool)
pub fn silent_tool(g: &mut crate::game::Game, loud: bool) {
    if !loud || !squad_silent(g) { return; }
    set_silent(g, false, Some(By::Tool));
    g.stats.silent.tools += 1;
    crate::game::game_msg(g, "squad: going loud on that order.");
}

/// is the silent squad compromised right now   (silent.ts silentCompromised)
pub fn silent_compromised(g: &mut crate::game::Game) -> bool {
    if !squad_silent(g) { return false; }
    let key = crate::seats::view_key(g);
    match g.m.silent.states.get(key) { Some(s) => s.hot, None => false }
}

fn row_targets() -> Vec<crate::orders::TargetKind> { vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::Door, crate::orders::TargetKind::None] }
fn row_listed() -> Vec<crate::orders::TargetKind> { vec![crate::orders::TargetKind::Floor, crate::orders::TargetKind::None] }

/// the posture's two rows on the menu: made anew at each call, as the TypeScript's literals are   (silent.ts rows)
pub fn rows(g: &mut crate::game::Game) -> [OrdersOrderDefId; 2] {
    let silent = g.tab.orders_order_def.push(crate::orders::OrderDef {
        id: String::from("silent"),
        head: Some(String::from(HEAD)),
        label: crate::orders::OrderLabel::Text { text: String::from("go silent") },
        targets: row_targets(),
        listed: Some(row_listed()),
        free: Some(true),
        only: Some(crate::orders::OrderOnly::SilentAll),
        ack: Some(vec![String::from("going quiet."), String::from("copy. silent.")]),
        issue: crate::orders::OrderIssue::Silent,
        ..Default::default()
    });
    let loud = g.tab.orders_order_def.push(crate::orders::OrderDef {
        id: String::from("loud"),
        head: Some(String::from(HEAD)),
        label: crate::orders::OrderLabel::Text { text: String::from("go loud") },
        targets: row_targets(),
        listed: Some(row_listed()),
        free: Some(true),
        only: Some(crate::orders::OrderOnly::SilentAll),
        ack: Some(vec![String::from("going loud."), String::from("copy. loud.")]),
        stands_in: Some(crate::orders::OrderDefStandsIn { for_: String::from("silent"), when: crate::orders::OrderWhen::Loud }),
        issue: crate::orders::OrderIssue::Loud,
        ..Default::default()
    });
    [silent, loud]
}

/// the word that follows 'loud' on the HUD   (silent.ts loudWord)
pub fn loud_word(w: Option<&str>) -> String {
    match w {
        Some(w) if !w.is_empty() => alloc::format!("loud · {}", w),
        _ => String::from("loud"),
    }
}

/// the rows on the menu, off it with the switch off; work only when that changes   (silent.ts seat)
pub fn seat(g: &mut crate::game::Game) {
    let go = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == "go");
    let go_digit = match go {
        Some(d) => match &g.tab.orders_order_def[d].hot_key { Some(k) => k.is_empty(), None => true },
        None => false,
    };
    let on = g.k.silent.on;
    let key = crate::seats::view_key(g);
    if let Some(was) = g.m.silent.seats.get(key) {
        if was.on == on && was.go_digit == go_digit { return; }
    }
    g.m.silent.seats.set(key, SilentModSeatsValue { on, go_digit });
    crate::orders::squad_orders_unregister(g, "loud");
    crate::orders::squad_orders_unregister(g, "silent");
    if !on { g.orders.posture = None; return; }
    if let Some(go) = go {
        if go_digit { crate::orders::squad_orders_register(g, go, None); }
    }
    let [silent, loud] = rows(g);
    crate::orders::squad_orders_register(g, silent, Some("go"));
    crate::orders::squad_orders_register(g, loud, None);
    g.orders.posture = Some(seam_posture(g));
}

/// `posture.word`   (silent.ts:243): what the posture's seam should call
fn posture_word(g: &mut Game) -> Option<String> {
    if !crate::orders::squad_orders_active(g) { return None; }
    if squad_silent(g) {
        return Some(String::from(if silent_compromised(g) { "SILENT · compromised" } else { "SILENT" }));
    }
    let w = crate::unseen::unseen_word(g);
    Some(loud_word(w.as_deref()))
}
/// `posture.mark`   (silent.ts:244): what the posture's seam should call
fn posture_mark(g: &mut Game, def: OrdersOrderDefId) -> &'static str {
    if squad_silent(g) && is_loud_row(&g.tab.orders_order_def[def].id) { " · LOUD" } else { "" }
}
/// `posture.given`   (silent.ts:245): what the posture's seam should call
fn posture_given(g: &mut Game, def: OrdersOrderDefId) {
    let loud = is_loud_row(&g.tab.orders_order_def[def].id);
    silent_tool(g, loud);
}
fn seam_posture(g: &mut Game) -> crate::orders::SquadOrdersPosture { todo!("needs: SquadOrdersPosture able to call posture_word, posture_mark, posture_given (silent.ts:242)") }

/// once a frame, before the orders pose the men: the rows, a new cast, the one line when loud reaches the squad   (silent.ts silentTick)
pub fn silent_tick(g: &mut crate::game::Game) {
    seat(g);
    let key = crate::seats::view_key(g);
    if !g.m.silent.states.has(key) {
        if !g.k.silent.on || !g.k.silent.start { return; }
        let cast = cast_of(g);
        if cast.is_empty() { return; }
        let since = g.time;
        g.m.silent.states.set(key, new_state(cast, since));
        set_silent(g, true, Some(By::Start));
        return;
    }
    let cast = cast_of(g);
    let now = g.time;
    let s = g.m.silent.states.get_mut(key).unwrap();
    let fresh = !cast.is_empty() && !cast.iter().any(|gd| s.cast.contains(gd));
    s.cast = cast.clone();
    if fresh {
        s.since = now; s.hot = false; s.was = Some(false); s.booted = None;
        let on = g.k.silent.on && g.k.silent.start;
        set_silent(g, on, Some(By::Start));
    }
    if !squad_silent(g) {
        g.m.silent.states.get_mut(key).unwrap().hot = false;
        return;
    }
    let mut hot = None;
    for &gd in cast.iter() {
        if g.gd[gd].state != crate::game::GuardState::Dead && reached(g, gd) { hot = Some(gd); break; }
    }
    let s = g.m.silent.states.get(key).unwrap();
    let (was_hot, was) = (s.hot, s.was == Some(true));
    if let Some(man) = hot {
        if !was_hot {
            g.stats.silent.compromised += 1;
            if !was { crate::game::game_say(g, man, "compromised.", None, None, None); }
        }
    }
    let s = g.m.silent.states.get_mut(key).unwrap();
    s.hot = hot.is_some();
    if hot.is_some() { s.was = Some(true); }
}

/// the panel's one line   (silent.ts silentSummary)
pub fn silent_summary(g: &mut crate::game::Game) -> String {
    if !g.k.silent.on { return String::from("off"); }
    let n = crate::orders::squad_orders_men(g, None, None).len();
    let start = if g.k.silent.start { "silent" } else { "loud" };
    if n == 0 { return alloc::format!("no squad · a new one starts {}", start); }
    let key = crate::seats::view_key(g);
    let word = if squad_silent(g) {
        match g.m.silent.states.get(key) { Some(s) if s.hot => "SILENT · compromised", _ => "SILENT" }
    } else { "loud" };
    let lasted = match g.m.silent.states.get(key) {
        Some(s) if s.since > 0.0 => alloc::format!(" for {} s", to_fixed_0(g.time - s.since)),
        _ => String::new(),
    };
    alloc::format!("{}{} · {} men · a new squad starts {}", word, lasted, n, start)
}

/// `x.toFixed(0)`: a tie goes up, which Rust's `{:.0}` does not do
fn to_fixed_0(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    if x < 0.0 { return alloc::format!("-{}", to_fixed_0(-x)); }
    let f = m::floor(x);
    let n = if x - f >= 0.5 { f + 1.0 } else { f };
    if n == 0.0 { return String::from("0"); }
    alloc::format!("{:.0}", n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_hud_word_after_loud() {
        assert_eq!(loud_word(None), "loud");
        assert_eq!(loud_word(Some("")), "loud");
        assert_eq!(loud_word(Some("sneaking")), "loud · sneaking");
    }
    #[test]
    fn to_fixed_as_the_language() {
        // the right-hand strings are what bun printed for x.toFixed(0)
        for (x, s) in [(0.0, "0"), (0.4, "0"), (0.5, "1"), (1.5, "2"), (2.5, "3"), (0.49999999999999994, "0"), (-0.4, "-0"), (-2.5, "-3"), (44.999, "45"), (1234.5, "1235")] {
            assert_eq!(to_fixed_0(x), s, "{}", x);
        }
    }
}
