//! src/game/holdfire.ts lines 1 to 187
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A new row: never lifted (the module's own literal, not `State::blank()`, whose stamps are 0).   (holdfire.ts stateOf)
fn fresh() -> State {
    State { lift_t: f64::NEG_INFINITY, why: None, cast: Vec::new(), by: None, loud_t: f64::NEG_INFINITY }
}

/// This view's row, made if missing.   (holdfire.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::holdfire::State {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    g.m.holdfire.states.get_or_insert_with(key, fresh)
}

/// Which player's squad this is.   (holdfire.ts leadOf)
pub fn lead_of(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> PlayerId {
    seat.player
}

/// A teammate under orders.   (holdfire.ts mate)
pub fn mate(g: &crate::game::Game, gd: GuardId) -> bool {
    g.gd[gd].team == crate::teams::Team::Operative && g.gd[gd].unit.is_some()
}

/// The squad's living men.   (holdfire.ts living)
pub fn living(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<GuardId> {
    let mut men = crate::orders::squad_orders_squad(g, seat);
    men.retain(|&gd| g.gd[gd].state != crate::game::GuardState::Dead);
    men
}

/// The squad has been found: something loud in a man's ears, or a defender roused who has really seen them.   (holdfire.ts holdFireFound)
pub fn hold_fire_found(g: &crate::game::Game, men: &[GuardId]) -> Option<String> {
    for &gd in men {
        if crate::roomfight::room_fight_hot_for(g, gd) {
            return Some(String::from("loud"));
        }
    }
    let m = *men.first()?;
    let mut i = 0;
    while i < g.guards.len() {
        let h = g.guards[i];
        i += 1;
        let state = g.gd[h].state;
        if (state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search)
            && g.gd[h].last_seen_t >= 0.0
            && crate::teams::hostile_to(g, h, crate::teams::HostileToO::Guard(m))
        {
            return Some(String::from("a defender has seen them"));
        }
    }
    None
}

/// A door order of the squad is being worked, or a rescue's clock is running. Waiting is not working.   (holdfire.ts working)
pub fn working(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> bool {
    let dead = crate::game::GuardState::Dead;
    if seat.door_tools.ops.iter().any(|&id| {
        let op = &g.tab.doortools_door_op[id];
        let waits = matches!(op.phase, crate::doortools::Phase::Place | crate::doortools::Phase::Standoff | crate::doortools::Phase::Armed);
        !op.on_word && !(op.kind == crate::doortools::DoorToolKind::Charge && waits) && op.men.iter().any(|&m| g.gd[m].state != dead)
    }) {
        return true;
    }
    if seat.wall_charge.ops.iter().any(|&id| {
        let op = &g.tab.wallcharge_wall_op[id];
        let waits = matches!(op.phase, crate::wallcharge::Phase::Place | crate::wallcharge::Phase::Standoff | crate::wallcharge::Phase::Armed);
        !waits && op.men.iter().any(|&m| g.gd[m].state != dead)
    }) {
        return true;
    }
    if g.night_breach.active {
        let st = crate::nightbreach::night_breach_state(g);
        if st == crate::nightbreach::NightBreachState::Going || st == crate::nightbreach::NightBreachState::Inside {
            return true;
        }
    }
    if !g.rescue.active || !crate::rescue::rescue_dynamic(g) {
        return false;
    }
    let st = crate::rescue::rescue_state(g);
    st != crate::rescue::RescueState::Rescued && st != crate::rescue::RescueState::Lost
}

/// His shot or his order has started the fight. A stamp from a roster that is gone, or from ahead of the clock, is no lift.   (holdfire.ts holdFireLifted)
pub fn hold_fire_lifted(g: &mut crate::game::Game) -> bool {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    if let Some(lift_t) = g.m.holdfire.states.get(key).map(|s| s.lift_t) {
        if lift_t > f64::NEG_INFINITY {
            let gone = g.time < lift_t || {
                let squad = crate::orders::squad_orders_squad(g, &crate::seats::here(g));
                match g.m.holdfire.states.get(key) {
                    Some(s) => !squad.iter().any(|gd| s.cast.contains(gd)),
                    None => true,
                }
            };
            if gone {
                if let Some(s) = g.m.holdfire.states.get_mut(key) {
                    s.lift_t = f64::NEG_INFINITY;
                    s.loud_t = f64::NEG_INFINITY;
                    s.why = None;
                    s.cast = Vec::new();
                    s.by = None;
                }
            } else if g.time - lift_t < g.k.holdfire.lift_secs {
                return true;
            }
        }
    }
    working(g, &crate::seats::here(g))
}

/// The hold comes off: a stamp still running is extended, one fight, one count.   (holdfire.ts lift)
pub fn lift(g: &mut crate::game::Game, why: crate::holdfire::LiftWhy, what: &str, by: PlayerId, loud: Option<bool>) {
    let loud = loud.unwrap_or(false);
    if !crate::orders::squad_orders_active(g, &crate::seats::here(g)) {
        return;
    }
    hold_fire_lifted(g);
    let cast = crate::orders::squad_orders_squad(g, &crate::seats::here(g));
    let (now, secs) = (g.time, g.k.holdfire.lift_secs);
    let s = state_of(g);
    let was = s.lift_t > f64::NEG_INFINITY && now - s.lift_t < secs;
    s.lift_t = now;
    s.why = Some(why);
    s.cast = cast;
    s.by = Some(by);
    if loud {
        s.loud_t = now;
    }
    if !was {
        g.stats.holdfire.lifts[why as usize] += 1;
        g.m.holdfire.holdfire_log.push(HoldfireModHOLDFIRELOGItem { t: now, why, what: String::from(what) });
        if g.m.holdfire.holdfire_log.len() > 200 {
            g.m.holdfire.holdfire_log.drain(0..100);
        }
    }
}

/// A round has left a gun: a player's starts the fight for his squad.   (holdfire.ts holdFireShot)
pub fn hold_fire_shot(g: &mut crate::game::Game, shooter: CharId) {
    let lead = lead_of(g, &crate::seats::here(g));
    if shooter != g.pl[lead].ch {
        return;
    }
    let by = lead_of(g, &crate::seats::here(g));
    lift(g, LiftWhy::Shot, "a round of his own", by, None);
}

/// An order has been given: one that means fire starts the fight, a waiting one when it goes, 'go loud' only to a squad that was silent.   (holdfire.ts holdFireOrder)
pub fn hold_fire_order(g: &mut crate::game::Game, id_2: &str, o: Option<crate::holdfire::HoldFireOrderO>) {
    let o = o.unwrap_or_default();
    if o.silent == Some(true) && id_2 == crate::holdfire::LOUD_ORDER && !crate::silent::squad_silent(g, &crate::seats::here(g)) {
        let by = match o.by {
            Some(by) => by,
            None => lead_of(g, &crate::seats::here(g)),
        };
        lift(g, o.why.unwrap_or(LiftWhy::Order), id_2, by, Some(true));
        return;
    }
    if !crate::orders::rows::id_is(id_2, crate::orders::Is::LiftsFire) || o.waits == Some(true) || (crate::orders::rows::id_is(id_2, crate::orders::Is::Blows) && o.blown != Some(true)) {
        return;
    }
    let by = match o.by {
        Some(by) => by,
        None => lead_of(g, &crate::seats::here(g)),
    };
    lift(g, o.why.unwrap_or(LiftWhy::Order), id_2, by, None);
}

/// Asked before an order's row is issued: is this 'go loud' told to a squad that is silent right now.   (holdfire.ts holdFireSilent)
pub fn hold_fire_silent(g: &crate::game::Game, seat: &crate::seats::SeatRef, id_2: &str) -> bool {
    id_2 == crate::holdfire::LOUD_ORDER && crate::silent::squad_silent(g, seat)
}

/// The lift that stands is his 'go loud'; the sneak's hold holds nobody either.   (holdfire.ts holdFireLoud)
pub fn hold_fire_loud(g: &mut crate::game::Game) -> bool {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    let loud_t = match g.m.holdfire.states.get(key) {
        Some(s) => s.loud_t,
        None => return false,
    };
    if !(loud_t > f64::NEG_INFINITY) || !hold_fire_lifted(g) {
        return false;
    }
    // Read again: hold_fire_lifted can clear the stamp on the row the TypeScript goes on holding.
    let loud_t = match g.m.holdfire.states.get(key) {
        Some(s) => s.loud_t,
        None => return false,
    };
    g.time - loud_t < g.k.holdfire.lift_secs
}

/// The F drill's 'go'.   (holdfire.ts holdFireDrill)
pub fn hold_fire_drill(g: &mut crate::game::Game, by: Option<PlayerId>) {
    let by = match by {
        Some(by) => by,
        None => lead_of(g, &crate::seats::here(g)),
    };
    lift(g, LiftWhy::Drill, crate::orders::rows::go::ORDER.id, by, None);
}

/// Is this loud teammate under the hold this frame.   (holdfire.ts held)
pub fn held(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !mate(g, gd) || !crate::orders::squad_orders_active(g, &crate::seats::here(g)) || hold_fire_lifted(g) {
        return false;
    }
    let men = living(g, &crate::seats::here(g));
    !men.is_empty() && hold_fire_found(g, &men).is_none()
}

/// This rule's own answer for one man's round at `h`, nothing counted.   (holdfire.ts holds)
pub fn holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut crate::game::Game) -> bool) -> bool {
    let state = g.gd[h].state;
    if state == crate::game::GuardState::Alert || state == crate::game::GuardState::Search || g.gd[h].awareness >= noticed {
        return false;
    }
    held(g, gd) && !urgent(g)
}

/// His round at `h` is held; the count.   (holdfire.ts holdFireHolds)
pub fn hold_fire_holds(g: &mut crate::game::Game, gd: GuardId, h: GuardId, noticed: f64, quiet: bool, urgent: &mut dyn FnMut(&mut crate::game::Game) -> bool) -> bool {
    if !holds(g, gd, h, noticed, quiet, urgent) {
        return false;
    }
    g.stats.holdfire.held += 1;
    true
}

/// The man the room fight has him on is one this rule will not let him shoot; asked afresh, nothing counted.   (holdfire.ts holdFireHoldsTarget)
pub fn hold_fire_holds_target(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if crate::silent::under_silence(g, gd) {
        return false;
    }
    let h = match crate::roomfight::room_fight_op_of(g, gd).and_then(|op| g.tab.roomfight_op[op].tgt) {
        Some(h) => h,
        None => return false,
    };
    if g.gd[h].state == crate::game::GuardState::Dead {
        return false;
    }
    let noticed = g.k.silent.noticed;
    let quiet = crate::silent::quiet_gun(g, gd);
    holds(g, gd, h, noticed, quiet, &mut |g: &mut Game| crate::silent::urgent(g))
}

/// Through standing glass the hold is the silent order's own: is that rule his this frame.   (holdfire.ts holdFireGlass)
pub fn hold_fire_glass(g: &mut crate::game::Game, gd: GuardId) -> bool {
    held(g, gd)
}

/// Silent.ts's glass hold held a round under this rule: the count.   (holdfire.ts holdFireGlassHeld)
pub fn hold_fire_glass_held(g: &mut crate::game::Game) {
    g.stats.holdfire.held_glass += 1;
}

/// A fresh encounter: nothing lifted.   (holdfire.ts holdFireReset)
pub fn hold_fire_reset(g: &mut crate::game::Game) {
    let key = crate::seats::view_key(g, &crate::seats::here(g));
    g.m.holdfire.states.delete(key);
}

