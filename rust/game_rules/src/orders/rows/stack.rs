//! STACK UP: "stack up". The men take the marks of a stack on the door, the lead nearest the frame.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use crate::v3::V3;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    valid: Some(valid),
    ..Row::new("stack", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("stack up") }

/// 'stack'.valid (orders.ts:592)
fn valid(g: &mut Game, t: &OrderTarget, men: &[GuardId]) -> Result<(), Option<String>> {
    let (door, side) = match t { OrderTarget::Door(x) => (x.door, x.side), _ => return Err(None) };
    let posts = match squad_orders_stack_posts_wait(g, door, side, men) {
        Some(p) => p,
        None => return Err(Some(squad_orders_no_room_words(g, "no room on that door.", &mut |g| squad_orders_stack_posts_wait(g, door, side, men).is_some()))),   // (ORDERS.inWay: … or that it is Sam who is in the way, where it is)
    };
    let spots: Vec<V3> = posts.iter().map(|x| x.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|x| x.group.unwrap_or(0.0)).collect();
    let front = g.dr[door].frame_centre;
    for (gd, p) in squad_orders_pair(g, men, &spots, Some(&groups), Some(front)) {
        if let Some(w) = squad_orders_reach(g, gd, p) { if !w.is_empty() { return Err(Some(w)); } }
    }
    Ok(())
}
/// 'stack'.issue (orders.ts:597)
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let (door, side) = match t { OrderTarget::Door(x) => (x.door, x.side), _ => return Some(String::from("no door")) };
    let posts = match squad_orders_stack_posts_wait(g, door, side, men) {
        Some(p) => p,
        None => return Some(squad_orders_no_room_words(g, "no room to stack on that door", &mut |g| squad_orders_stack_posts_wait(g, door, side, men).is_some())),
    };
    let name = door_words(g, door);
    let spots: Vec<V3> = posts.iter().map(|p| p.pos).collect();
    let groups: Vec<f64> = posts.iter().map(|p| p.group.unwrap_or(0.0)).collect();
    let fc = g.dr[door].frame_centre;
    for (gd, p) in squad_orders_pair(g, men, &spots, Some(&groups), Some(fc)) {
        // (`x.pos === p` in the TypeScript: the very mark dealt. Marks of one stack are never at one spot, so the first of the same bits is that one)
        let post = *posts.iter().find(|x| same_bits(x.pos, p)).unwrap();
        let a = SquadOrdersAssignA {
            order: String::from("stack"), post: Some(post.pos), face: Some(post.face),
            watch: Some([fc[0], 1.05, fc[2]]), door: Some(door), shy: Some(door),
            stance: Some(if post.lead { crate::character::Stance::Stack } else { crate::character::Stance::HighReady }),
            upper: Some(crate::character::UpperMode::None),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove { stance: crate::character::Stance::None, upper: crate::character::UpperMode::Aim }),
            stack_side: Some(post.stack_side),
            going: format!("stacking · {}", name), there: format!("stacked · {}", name), set_line: Some(String::from("set.")),
            ..Default::default()
        };
        squad_orders_assign(g, gd, &a);
    }
    squad_orders_spare_rear(g, men, door, None, None);
    None
}
