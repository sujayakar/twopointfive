//! COVER: "cover this door", "cover that way". Each man stays where he is and watches it.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::Row;
use crate::orders::*;
use crate::v3::{self, V3};
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    can: Some(can),
    ..Row::new("cover", label, issue)
};

/// 'cover'.label (orders.ts:605)
fn label(_g: &mut Game, t: &OrderTarget) -> String {
    String::from(if matches!(t, OrderTarget::Door(_)) { "cover this door" } else { "cover that way" })
}
/// 'cover'.can (orders.ts:606)
fn can(g: &mut Game, gd: GuardId, t: &OrderTarget) -> Option<String> {
    if squad_orders_has_line(g, gd, t) { None } else { Some(String::from("no line from here.")) }
}
/// 'cover'.issue (orders.ts:606)
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let t_at = rows::at(t);
    let watch: V3 = [t_at[0], 1.1, t_at[2]];
    for &gd in men {
        let p = g.ch[g.gd[gd].ch].pos;
        let yaw = v3::yaw_to(p, watch);
        let (place, door) = match t {
            OrderTarget::Door(x) => (door_words(g, x.door), Some(x.door)),
            _ => (crate::dialogue::watched_place(g, p, watch).unwrap_or_else(|| String::from("that way")), None),
        };
        let words = format!("covering {}", place);
        let a = SquadOrdersAssignA {
            order: String::from("cover"), post: Some([p[0], 0.0, p[2]]), face: Some(yaw), watch: Some(watch), door,
            stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim),
            going: words.clone(), there: words,
            ..Default::default()
        };
        squad_orders_assign(g, gd, &a);
    }
    None
}
