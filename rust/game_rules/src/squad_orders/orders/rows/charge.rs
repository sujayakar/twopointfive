//! BREACH WITH A CHARGE: a charge on the door, blown from a stand-off; the charge already on it is blown.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::LiftsFire, Is::Blows, Is::Loud, Is::Enters],
    valid: Some(valid),
    gone: Some(gone),
    ..Row::new("charge", label, issue)
};

fn label(g: &mut Game, t: &OrderTarget) -> String {
    if let OrderTarget::Door(x) = t {
        if crate::doortools::door_tools_loose_on(g, &crate::seats::here(g), t) { return String::from("stand off · in behind the charge"); }
        if let Some(c) = crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), x.door) {
            return format!("blow the charge{}", crate::doortools::door_tools_close_words(g, c));
        }
    }
    format!("breach with a charge · {} left", g.sets[g.seat].door_tools.charges)
}

fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let OrderTarget::Door(x) = t else { return Err(None) };
    let fits = |r: crate::doortools::DoorToolsStandoffFits| match r { crate::doortools::DoorToolsStandoffFits::Text(w) => Err(Some(w)), crate::doortools::DoorToolsStandoffFits::Bool(true) => Ok(()), crate::doortools::DoorToolsStandoffFits::Bool(false) => Err(None) };
    if crate::doortools::door_tools_loose_on(g, &crate::seats::here(g), t) { return fits(crate::doortools::door_tools_standoff_fits(g, men, t, Some(true))); }
    if crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), x.door).is_some() { return Ok(()); }
    if on(g.dr[x.door].def.steel) { return Err(Some(String::from("steel. a charge won't cut it."))); }
    let open = crate::doortools::door_tools_known_open(g, x.door);
    if open { return Err(Some(String::from("it's open."))); }
    if g.sets[g.seat].door_tools.charges <= 0.0 { return Err(Some(String::from("no charges left."))); }
    let ops = g.sets[g.seat].door_tools.ops.clone();
    if ops.iter().any(|&o| { let op = &g.tab.doortools_door_op[o]; op.kind == crate::doortools::DoorToolKind::Charge && op.door == x.door && op.phase == crate::doortools::Phase::Place }) { return Err(Some(String::from("already on it."))); }
    fits(crate::doortools::door_tools_standoff_fits(g, men, t, None))
}

fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let OrderTarget::Door(x) = t else { return Some(String::from("no door")) };
    let started = |r: crate::doortools::DoorToolsStartCharge| match r { crate::doortools::DoorToolsStartCharge::Text(w) => Some(w), crate::doortools::DoorToolsStartCharge::Arm1(()) => None };
    if crate::doortools::door_tools_loose_on(g, &crate::seats::here(g), t) { return started(crate::doortools::door_tools_start_charge(g, men, t, Some(true))); }
    if let Some(c) = crate::doortools::door_tools_charge_on(g, &crate::seats::here(g), x.door) { crate::doortools::door_tools_trigger(g, Some(c)); return None; }
    started(crate::doortools::door_tools_start_charge(g, men, t, None))
}

/// a steel door takes none, but for the one that is on it
fn gone(g: &Game, seat: &crate::seats::SeatRef, t: &OrderTarget, _men: &[GuardId]) -> bool {
    let OrderTarget::Door(x) = t else { return false };
    on(g.dr[x.door].def.steel) && crate::doortools::door_tools_charge_on(g, seat, x.door).is_none()
}
