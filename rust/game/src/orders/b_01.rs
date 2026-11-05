//! THE BRING-UP'S OWN PART of orders: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// BRING-UP, ROUND TWO: AN ORDER'S CLOSURES AFTER AN IMPORT. The bridge cannot carry OrderDef.label / can / only / valid / issue / tick / gone (closures made enums:
// not-carried.json), so a row the import brought has `issue: Nothing` and an order given does nothing (the two sides first parted there, at tick 21).
// BRIDGE-ASK: on import, deal each row its enums by `def.id` (the tables are static), as this does by hand from the parts' own registrations.
/// Every imported order row dealt its enums again by its id, as the part that registers it writes them (orders p_02 squad_orders_new; doortools p_02 door_tools_new /
/// thr_rows / sync; shutdoor p_01; workthings p_01; wallcharge p_01). A row registered by the Rust game itself is whole already and is written the same again.
/// Not dealt (no variant in the skeleton): 'held' beyond its label, 'thrWord'. Answers the ids it did not know.
pub fn orders_closures_again(g: &mut Game) -> Vec<String> {
    let mut unknown = Vec::new();
    let text = |s: &str| OrderLabel::Text { text: String::from(s) };
    for id in g.tab.orders_order_def.ids().collect::<Vec<_>>() {
        let d = &mut g.tab.orders_order_def[id];
        match d.id.as_str() {
            "move" => { d.label = text("move there"); d.can = Some(OrderCan::Move); d.valid = Some(OrderValid::Move); d.issue = OrderIssue::Move; d.tick = Some(OrderTick::Move); }
            "stack" => { d.label = text("stack up"); d.valid = Some(OrderValid::Stack); d.issue = OrderIssue::Stack; }
            "cover" => { d.label = OrderLabel::Cover; d.can = Some(OrderCan::Cover); d.issue = OrderIssue::Cover; }
            "fallIn" => { d.label = text("fall in"); d.issue = OrderIssue::FallIn; }
            "hold" => { d.label = text("hold position"); d.issue = OrderIssue::Hold; }
            "tools" => { d.label = text("door tools"); d.issue = OrderIssue::Nothing; }
            "held" => { d.label = text("threshold held"); d.issue = OrderIssue::Nothing; }
            "openClear" => { d.label = OrderLabel::DoorOpenClear; d.only = Some(OrderOnly::DoorHeldOrAll); d.valid = Some(OrderValid::DoorOpenClear); d.issue = OrderIssue::DoorOpenClear; }
            "bangClear" => { d.label = OrderLabel::DoorBangClear; d.only = Some(OrderOnly::DoorHeldOrAll); d.valid = Some(OrderValid::DoorBangClear); d.issue = OrderIssue::DoorBangClear; }
            "charge" => { d.label = OrderLabel::DoorCharge; d.valid = Some(OrderValid::DoorCharge); d.issue = OrderIssue::DoorCharge; d.gone = Some(OrderGone::DoorCharge); }
            "fragClear" => { d.label = OrderLabel::DoorFragClear; d.valid = Some(OrderValid::DoorFragClear); d.issue = OrderIssue::DoorFragClear; }
            "thresholdHold" => { d.label = OrderLabel::DoorThresholdHold; d.valid = Some(OrderValid::DoorThresholdHold); d.issue = OrderIssue::DoorThresholdHold; }
            "bigClear" => { d.label = OrderLabel::DoorBigClear; d.valid = Some(OrderValid::DoorBigClear); d.issue = OrderIssue::DoorBigClear; d.gone = Some(OrderGone::DoorBigClear); }
            "shutDoor" => { d.label = text("shut that door"); d.only = Some(OrderOnly::ShutDoor); d.valid = Some(OrderValid::ShutDoor); d.issue = OrderIssue::ShutDoor; d.tick = Some(OrderTick::ShutDoor);
                if let Some(s) = d.stands_in.as_mut() { s.when = OrderWhen::ShutDoor; } }
            "work" => { d.label = OrderLabel::Work; d.can = Some(OrderCan::Work); d.only = Some(OrderOnly::Work); d.valid = Some(OrderValid::Work); d.issue = OrderIssue::Work; d.tick = Some(OrderTick::Work); }
            "walltools" => { d.label = text("wall tools"); d.issue = OrderIssue::Nothing; }
            "wallCharge" => { d.label = OrderLabel::WallRow; d.can = Some(OrderCan::WallRow); d.valid = Some(OrderValid::WallRow); d.issue = OrderIssue::WallRow; d.gone = Some(OrderGone::WallRow); }
            other => unknown.push(String::from(other)),
        }
    }
    unknown
}
