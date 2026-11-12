//! FALL IN: "fall in". The men take places in the file behind Sam.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use crate::{js, m, v3};
use alloc::{string::String, vec::Vec};

pub const ORDER: Row = Row { is: &[Is::Follows, Is::LooksAbout, Is::Resumed], ..Row::new("fallIn", label, issue) };

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("fall in") }

/// 'fallIn'.issue: a man already in the file keeps his place; three or more file by pairs, the pair with the nearer man first
fn issue(g: &mut Game, men: &[GuardId], _: &OrderTarget, _: Option<&GiveOpts>) -> Option<String> {
    let mut taken: Vec<f64> = Vec::new();
    for (gd, a) in g.groups[g.group].orders.asg.iter() {
        let row = &g.tab.task[*a];
        if row.post.is_none() && !men.contains(&gd) { taken.push(row.follow); }
    }
    let sam = g.player_pos(g.player);
    let nears: Vec<f64> = men.iter().map(|&m| v3::dist_xz(g.pos_of(m), sam)).collect();
    let pair_ids: Vec<Option<crate::game::PairId>> = men.iter().map(|&m| squad_orders_pair_of(g, m)).collect();
    let many = men.len() > 2;
    let mut sorted: Vec<(GuardId, (f64, f64, Option<crate::game::PairId>))> = Vec::with_capacity(men.len());
    for (i, &gd) in men.iter().enumerate() {
        let id = pair_ids[i];
        let pair_near = if many && id.is_some() {
            let of_pair: Vec<f64> = (0..men.len()).filter(|&j| pair_ids[j] == id).map(|j| nears[j]).collect();
            m::min_n(&of_pair)
        } else {
            nears[i]
        };
        sorted.push((gd, (pair_near, nears[i], id)));
    }
    js::sort_by(&mut sorted, |a, b| fall_in_cmp(many, a.1, b.1));
    let mut k = 0.0;
    for (gd, _) in sorted {
        while taken.contains(&k) { k += 1.0; }
        taken.push(k);
        let a = crate::duty::TaskGiven {
            order: String::from(ORDER.id), post: None, follow: Some(k), face: None,
            stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
            going: "falling in".into(), there: "with you".into(),
            ..Default::default()
        };
        squad_orders_assign(g, gd, &a);
    }
    None
}
