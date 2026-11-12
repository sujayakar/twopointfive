//! HOLD POSITION: "hold position". A running man plants a stride on; the squad of four take an approach each.
use crate::game::Game;
use crate::ids::*;
use crate::orders::rows::{Is, Row};
use crate::orders::*;
use crate::m;
use crate::v3::V3;
use alloc::{format, string::String, vec, vec::Vec};

pub const ORDER: Row = Row {
    is: &[Is::Holds, Is::LooksAbout, Is::Resumed, Is::KeepsLane],
    targets: &[TargetKind::Floor, TargetKind::Door, TargetKind::None],
    ack: &["holding."],
    ..Row::new("hold", label, issue)
};

fn label(_g: &mut Game, _t: &OrderTarget) -> String { String::from("hold position") }

/// 'hold'.issue: a running man plants a stride on; the squad of four take an approach each
fn issue(g: &mut Game, men: &[GuardId], t: &OrderTarget, o: Option<&GiveOpts>) -> Option<String> {
    let mut posts: Vec<V3> = Vec::with_capacity(men.len());
    for &gd in men {
        let c = g.gd[gd].ch;
        let (p, v, k) = (g.ch[c].pos, g.ch[c].vel, g.k.orders.hold_stride);
        let on: V3 = [p[0] + v[0] * k, 0.0, p[2] + v[2] * k];
        let ok = !crate::scene_collision::nav_grid_is_blocked(g, on[0], on[2]) && !crate::scene_collision::low_blocked(g, p, on);
        posts.push(if ok { on } else { [p[0], 0.0, p[2]] });
    }
    let arcs = if squad_orders_big(g, &crate::seats::here(g)) && men.len() > 1 { Some(squad_orders_hold_arcs(g, &posts)) } else { None };
    for (i, &gd) in men.iter().enumerate() {
        let arc = arcs.as_ref().and_then(|a| a.get(i).copied().flatten());
        let a = match arc {
            Some(SquadOrdersHoldArcsItem { yaw, door: Some(door), .. }) => {   // (a door close by in his arc: the gun, and the light on it, is ON it)
                let fc = g.dr[door].frame_centre;
                let there = format!("holding · {}", door_words(g, door));
                crate::duty::TaskGiven {
                    order: String::from(ORDER.id), post: Some(posts[i]), face: Some(yaw), watch: Some([fc[0], 1.1, fc[2]]), door: Some(door),
                    stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true),
                    going: "holding".into(), there: there.into(),
                    ..Default::default()
                }
            }
            Some(arc) => {   // (… else out along his arc)
                let watch: V3 = [posts[i][0] + m::sin(arc.yaw) * arc.reach, 1.1, posts[i][2] + m::cos(arc.yaw) * arc.reach];
                let pl = crate::dialogue::watched_place(g, posts[i], [posts[i][0] + m::sin(arc.yaw) * arc.reach, 0.0, posts[i][2] + m::cos(arc.yaw) * arc.reach]);
                let there = if truthy_str(&pl) { format!("holding · {}", pl.unwrap()) } else { String::from("holding") };
                crate::duty::TaskGiven {
                    order: String::from(ORDER.id), post: Some(posts[i]), face: Some(arc.yaw), watch: Some(watch),
                    stance: Some(crate::character::Stance::None), upper: Some(crate::character::UpperMode::Aim), light: Some(true),
                    going: "holding".into(), there: there.into(),
                    ..Default::default()
                }
            }
            None => crate::duty::TaskGiven {
                order: String::from(ORDER.id), post: Some(posts[i]), face: Some(g.ch[g.gd[gd].ch].body_yaw),
                stance: Some(crate::character::Stance::LowReady), upper: Some(crate::character::UpperMode::None),
                going: "holding".into(), there: "holding".into(),
                ..Default::default()
            },
        };
        squad_orders_assign(g, gd, &a);
    }
    None
}
