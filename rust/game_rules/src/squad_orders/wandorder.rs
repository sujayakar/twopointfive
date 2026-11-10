//! src/game/wandorder.ts lines 1 to 185: a teammate looks under a shut door with the optical wand, on the player's order only.

pub use ::game_types::wandorder::types;
pub use types::*;
/// this module's own order
pub const WAND_ORDER: &str = crate::orders::rows::wand::ORDER.id;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{string::String, vec, vec::Vec};

/// This view's row for `gd`, a copy (`states.get(g)?.jobs.get(gd)`). The TypeScript's `J` is the row itself: `put_job` after a write, `lens_out` included.
fn job(g: &Game, seat: &crate::seats::SeatRef, gd: GuardId) -> Option<Job> {
    let k = crate::seats::view_key(g, seat);
    g.m.wandorder.states.get(k).and_then(|st| st.jobs.get(gd).copied())
}

/// Writes a copy back over the row, if the row is still there (never sets a new one: that would move it in the walk's order).
fn put_job(g: &mut Game, gd: GuardId, j: Job) {
    let k = crate::seats::view_key(g, &crate::seats::here(g));
    if let Some(row) = g.m.wandorder.states.get_mut(k).and_then(|st| st.jobs.get_mut(gd)) {
        *row = j;
    }
}

/// Distance on the floor.
fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// Is there a fog to look into. (wandorder.ts live; WAND.on and WAND.mate are settled on)
fn live(g: &mut crate::game::Game) -> bool {
    crate::fogofwar::fog_of_war_active(g)
}

/// Is a wand in his kit.
pub fn carries(g: &crate::game::Game, gd: GuardId) -> bool {
    crate::kit::kit_of(g, gd).map_or(false, |k| k.wand)
}

/// The fight has him: its lease, blind, folded, or the room fight's aim over his order.
fn fought(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool {
    g.tab.orders_assignment[a].yield_t >= 0.0
        || crate::control::man::blind(g, gd)
        || crate::control::man::staggered(g, gd)
        || g.gd[gd].script.as_ref().map_or(false, |s| s.over.is_some())
}

/// A point half a metre off the frame on face `side`: what Doors.workSpot is asked from.
fn on_face(g: &crate::game::Game, d: DoorId, side: f64) -> V3 {
    let u = crate::doors::door_dir(g, d);
    let c = g.dr[d].frame_centre;
    [c[0] - u[1] * side * 0.5, 0.0, c[2] + u[0] * side * 0.5]
}

/// His kneeling spot on that face of the leaf.
fn spot_on(g: &crate::game::Game, d: DoorId, side: f64) -> crate::doors::DoorsWorkSpot {
    let from = on_face(g, d, side);
    let along = g.dr[d].def.width * 0.5;
    crate::doors::doors_work_spot(g, d, from, Some(0.5), Some(along))
}

/// Who of `men` takes it and where he kneels: the nearest carrier who can walk to the door's face; else why not.
pub(super) fn pick(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::wandorder::Pick {
    let crate::orders::OrderTarget::Door(t) = t else { return Pick::Text(String::from("not a door.")) };
    let d = t.door;
    if g.dr[d].leaf.off {
        return Pick::Text(String::from("no door left."));
    }
    if g.dr[d].def.steel.unwrap_or(false) {
        return Pick::Text(String::from("steel: no gap under it."));
    }
    if !crate::wand::wand_fits(g, d) {
        return Pick::Text(String::from(crate::words::wg(g, &"it's not shut.", &"it is not shut.")));
    }
    let mut has: Vec<GuardId> = Vec::new();
    for &gd in men {
        if carries(g, gd) {
            has.push(gd);
        }
    }
    if has.is_empty() {
        return Pick::Text(String::from(crate::words::wg(g, &"nobody's carrying a wand.", &"no wand on them.")));
    }
    let spot = spot_on(g, d, t.side);
    let mut best: Option<GuardId> = None;
    let mut bd = m::INF;
    for &gd in &has {
        if crate::orders::squad_orders_reach(g, gd, spot.pos).is_some() {
            continue;
        }
        let k = flat(g.ch[g.gd[gd].ch].pos, spot.pos);
        if k < bd {
            bd = k;
            best = Some(gd);
        }
    }
    match best {
        Some(gd) => Pick::Arm1(PickArm1 { gd, side: t.side, spot: PickArm1Spot { pos: spot.pos, yaw: spot.yaw } }),
        None => Pick::Text(String::from("can't get to it.")),
    }
}

/// The lens comes out and he gets up; the next feed is a full one if he had been on his knee. `j` is the caller's copy: the caller puts it back.
fn lens_out(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::wandorder::Job) {
    if j.knelt {
        let c = g.gd[gd].ch;
        g.ch[c].anim.lockpick = false;
        j.knelt = false;
        j.again = true;
    }
    j.t0 = -1.0;
    j.live = false;
    g.tab.orders_assignment[j.a].crouch = Some(false);
}

/// Done with it: the lens out, a word, a plain hold: beside the latch jamb with his gun on the doorway when the leaf moved, else where he stands.
fn settle(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::wandorder::Job, line: Option<&str>, moved: bool) {
    let k = crate::seats::view_key(g, &crate::seats::here(g));
    let c = g.gd[gd].ch;
    let d = j.door;
    lens_out(g, gd, j);
    g.m.wandorder.states.get_mut(k).unwrap().jobs.delete(gd);
    if let Some(l) = line.filter(|l| !l.is_empty()) {
        crate::game::game_say(g, gd, l, None, None);
    }
    let to = match j.aside {
        Some(a) if moved && crate::orders::squad_orders_reach(g, gd, a.pos).is_none() => Some(a),
        _ => None,
    };
    if let Some(to) = to {
        let fc = g.dr[d].frame_centre;
        let there = alloc::format!("holding · {}", crate::orders::door_words(g, d));
        crate::orders::squad_orders_assign(
            g,
            gd,
            &crate::orders::SquadOrdersAssignA {
                order: String::from(crate::orders::rows::hold::ORDER.id),
                post: Some(to.pos),
                face: Some(to.yaw),
                watch: Some([fc[0], 1.1, fc[2]]),
                door: Some(d),
                stance: Some(crate::character::Stance::None),
                upper: Some(crate::character::UpperMode::Aim),
                going: String::from("off the door"),
                there,
                set_line: None,
                ..Default::default()
            },
        );
    } else {
        // (`p` and `face` are read here: in the TypeScript `p` is his live position, and the calls above do not move him)
        let p = g.ch[c].pos;
        let face = g.ch[c].body_yaw;
        crate::orders::squad_orders_assign(
            g,
            gd,
            &crate::orders::SquadOrdersAssignA {
                order: String::from(crate::orders::rows::hold::ORDER.id),
                post: Some([p[0], 0.0, p[2]]),
                face: Some(face),
                stance: Some(crate::character::Stance::LowReady),
                upper: Some(crate::character::UpperMode::None),
                going: String::from("holding"),
                there: String::from("holding"),
                set_line: None,
                ..Default::default()
            },
        );
    }
}

/// Gives the one man the look, and opens his job. `Issue::Arm1(())` is the TypeScript's `void`; it never returns words.
pub(super) fn issue(g: &mut crate::game::Game, men: &[GuardId], t: &crate::orders::OrderTarget) -> crate::wandorder::Issue {
    let p = pick(g, men, t);
    let (Pick::Arm1(p), crate::orders::OrderTarget::Door(t)) = (p, t) else { return Issue::Arm1(()) };
    let st_key = crate::seats::view_key(g, &crate::seats::here(g));
    if !g.m.wandorder.states.has(st_key) {
        return Issue::Arm1(());
    }
    if let Some(mut old) = job(g, &crate::seats::here(g), p.gd) {
        lens_out(g, p.gd, &mut old); // (sent from one door to another on his knees: he gets up first)
        put_job(g, p.gd, old);
    }
    let d = t.door;
    let gd = p.gd;
    let words = crate::orders::door_words(g, d);
    // (beside the latch jamb, found while the leaf is shut: its direction is the frame's only then)
    let from = on_face(g, d, p.side);
    let k = crate::doors::doors_kick_spot_side(g, d, from);
    let aside = if crate::scene_collision::nav_grid_is_blocked(g, k.pos[0], k.pos[2]) {
        None
    } else {
        Some(JobAside { pos: [k.pos[0], 0.0, k.pos[2]], yaw: k.yaw })
    };
    crate::orders::squad_orders_assign(
        g,
        gd,
        &crate::orders::SquadOrdersAssignA {
            order: String::from(WAND_ORDER),
            post: Some([p.spot.pos[0], 0.0, p.spot.pos[2]]),
            face: Some(p.spot.yaw),
            door: Some(d),
            stance: Some(crate::character::Stance::LowReady),
            upper: Some(crate::character::UpperMode::None),
            move_: Some(crate::doortools::DoorToolsDoneItemLaneBackMove {
                stance: crate::character::Stance::None,
                upper: crate::character::UpperMode::Aim,
            }),
            going: alloc::format!("wand · {}", words),
            there: alloc::format!("feeding the wand · {}", words),
            set_line: None,
            ..Default::default()
        },
    );
    let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd).unwrap();
    let new_job = Job {
        door: d,
        side: p.side,
        stand: [p.spot.pos[0], 0.0, p.spot.pos[2]],
        yaw: p.spot.yaw,
        aside,
        a,
        t0: -1.0,
        live_t: -1.0,
        last_t: g.time,
        live: false,
        knelt: false,
        again: false,
    };
    g.m.wandorder.states.get_mut(st_key).unwrap().jobs.set(gd, new_job);
    Issue::Arm1(())
}

/// The order's beat for one man: walk, kneel, feed, look; ends when the leaf moves, the switch goes, or his mark is moved.
pub fn tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let Some(mut j) = job(g, &crate::seats::here(g), gd) else { return };
    if j.a != a {
        return;
    }
    let now = g.time;
    let d = j.door;
    let c = g.gd[gd].ch;
    if !live(g) || !carries(g, gd) {
        settle(g, gd, &mut j, None, false);
        return;
    }
    if !crate::wand::wand_fits(g, d) {
        let line = if j.t0 >= 0.0 { "door's moving. wand's out." } else { "door's open." };
        settle(g, gd, &mut j, Some(line), true);
        return;
    }
    // (the fight had him, or he was blinded or folded: the lens is out, and it is fed again from the start)
    if now - j.last_t > 0.25 || fought(g, gd, a) {
        lens_out(g, gd, &mut j);
    }
    j.last_t = now;
    put_job(g, gd, j);
    // (the orders stopped him short: his mark moved to his feet, and he never looks from there)
    let short = match g.tab.orders_assignment[a].post {
        None => true,
        Some(post) => flat(post, j.stand) > 0.3,
    };
    if short {
        let line = if g.tab.orders_assignment[a].there == "stopped short" { None } else { Some("can't get on the door.") };
        settle(g, gd, &mut j, line, false);
        return;
    }
    let pos = g.ch[c].pos;
    let on = g.tab.orders_assignment[a].arrived
        && flat(pos, j.stand) < 0.45
        && m::abs(v3::wrap_angle(g.ch[c].body_yaw - j.yaw)) < 0.6
        && crate::doors::doors_side(g, d, pos) == j.side
        && !fought(g, gd, a);
    if !on {
        lens_out(g, gd, &mut j);
        put_job(g, gd, j);
        return;
    }
    if j.t0 < 0.0 {
        j.t0 = now;
    }
    g.ch[c].anim.lockpick = true;
    j.knelt = true;
    g.tab.orders_assignment[a].crouch = Some(true);
    // (WAND.quick settled on: only a second feed at this door waits WAND.feedSecs)
    let through = !j.live
        && if j.again {
            now - j.t0 >= g.k.wand.feed_secs
        } else {
            let crouch = g.ch[c].anim.crouch;
            crate::wand::wand_through(g, now - j.t0, crouch, None)
        };
    if through {
        j.live = true;
        j.live_t = now;
    }
    put_job(g, gd, j);
    if through {
        g.stats.wandorder.live += 1;
        crate::game::game_say(g, gd, "wand's in.", None, None);
    }
    let what = if j.live { "looking under" } else { "feeding the wand" };
    let there = alloc::format!("{} · {}", what, crate::orders::door_words(g, d));
    g.tab.orders_assignment[a].there = there;
}

/// The order's row on the menus.
fn row(g: &mut crate::game::Game) -> OrdersOrderDefId {
    let def = crate::orders::rows::def(&crate::orders::rows::wand::ORDER, crate::orders::OrderDef {
        targets: vec![crate::orders::TargetKind::Door],
        ack: Some(vec![String::from("on it."), String::from("moving.")]),
        ..Default::default()
    });
    crate::orders::squad_orders_row(g, def)
}

/// Once a frame per seat: keeps the row on the menus only while a living squad man carries a wand and there is a fog; takes the lens out of a man the
/// fight or another order has.
pub fn wand_order_frame(g: &mut crate::game::Game) {
    let k = crate::seats::view_key(g, &crate::seats::here(g));
    let mut want = false;
    if live(g) {
        for gd in crate::orders::squad_orders_squad(g, &crate::seats::here(g)) {
            if !crate::condition::down(g, g.gd[gd].ch) && carries(g, gd) {
                want = true;
                break;
            }
        }
    }
    if !g.m.wandorder.states.has(k) {
        if !want {
            return;
        }
        g.m.wandorder.states.set(k, State { jobs: JsMap::new() });
    }
    let listed = g.sets[g.seat].orders.defs.iter().any(|&d| g.tab.orders_order_def[d].id == WAND_ORDER);
    if want && !listed {
        let after = g.sets[g.seat].orders.defs.iter().position(|&d| g.tab.orders_order_def[d].id == crate::orders::rows::hold::ORDER.id);
        let r = row(g);
        let before = after.and_then(|i| g.sets[g.seat].orders.defs.get(i + 1)).map(|&d| g.tab.orders_order_def[d].id.clone());
        crate::orders::squad_orders_register(g, r, before.as_deref());
    } else if !want && listed {
        crate::orders::squad_orders_unregister(g, WAND_ORDER);
    }
    if !g.m.wandorder.states.get(k).unwrap().jobs.is_empty() {
        let squad = crate::orders::squad_orders_squad(g, &crate::seats::here(g));
        let now = g.time;
        // (a live walk: settle deletes the row it stands on and calls out to the orders)
        let mut at = 0usize;
        while let Some(gd) = g.m.wandorder.states.get(k).unwrap().jobs.next_key(&mut at) {
            let mut j = job(g, &crate::seats::here(g), gd).unwrap();
            let a = crate::orders::squad_orders_of(&crate::seats::here(g), gd);
            if crate::condition::down(g, g.gd[gd].ch) || !squad.contains(&gd) || a != Some(j.a) {
                if j.knelt {
                    let c = g.gd[gd].ch;
                    g.ch[c].anim.lockpick = false;
                }
                g.m.wandorder.states.get_mut(k).unwrap().jobs.delete(gd);
                continue;
            }
            // (the fog or the switch went off under him: the row is off the list, so its tick never runs again: a plain hold where he is)
            if !want || !carries(g, gd) {
                settle(g, gd, &mut j, None, false);
                continue;
            }
            if (j.knelt || j.live) && (now - j.last_t > 0.25 || fought(g, gd, j.a)) {
                lens_out(g, gd, &mut j);
                put_job(g, gd, j);
            }
        }
    }
}

/// His lens as his eye among the fog's, or None while he is not looking: the player's lens by every number, swept over its arc on the game's clock.
pub fn mate_wand_eye(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogEye> {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| eye_in_view(g, gd))
}

/// `mateWandEye` from its second line on, in his seat's view.
fn eye_in_view(g: &mut Game, gd: GuardId) -> Option<crate::fogofwar::FogEye> {
    let k = crate::seats::view_key(g, &crate::seats::here(g));
    let st = g.m.wandorder.states.get(k)?;
    if st.jobs.is_empty() {
        return None;
    }
    let j = *st.jobs.get(gd)?;
    if !j.live || !live(g) {
        return None;
    }
    let now = g.time;
    if crate::condition::down(g, g.gd[gd].ch)
        || now - j.last_t > 0.25
        || crate::condition::is_blind(g, g.gd[gd].ch)
        || g.gd[gd].script.as_ref().map_or(false, |s| s.over.is_some())
        || !crate::wand::wand_fits(g, j.door)
    {
        return None;
    }
    let l = crate::wand::wand_lens(g, j.door, j.side);
    let ph = js::rem((now - j.live_t) / m::max(0.5, g.k.wand.sweep_secs) + 0.25, 1.0);
    let a = l.yaw + crate::wand::wand_pan_max(g) * (1.0 - 4.0 * m::abs(ph - 0.5));
    let fov = crate::wand::wand_fov(g);
    let id = g.ch[g.gd[gd].ch].id;
    Some(crate::fogofwar::FogEye {
        id,
        pos: l.pos,
        look: [m::sin(a), 0.0, m::cos(a)],
        at: l.pos,
        fov: Some(fov),
        near: Some(0.0),
        wand: Some(true),
    })
}

/// Every teammate's live lens: a device feed under 'only my own eyes'.
pub fn mate_wand_eyes(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogEye> {
    let k = crate::seats::view_key(g, &crate::seats::here(g));
    let Some(st) = g.m.wandorder.states.get(k) else { return Vec::new() };
    if st.jobs.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for gd in st.jobs.keys() {
        if let Some(e) = mate_wand_eye(g, gd) {
            out.push(e);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Off a knee he was on, the next feed is a full one; never on his knee, `again` stays as it was.
    #[test]
    fn lens_out_marks_a_second_feed_only_if_he_knelt() {
        let mut g = Game::blank();
        let c = crate::character::man_new(&mut g, crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let gd = g.gd.push(man);
        let a = g.tab.orders_assignment.push(crate::orders::Assignment::default());
        g.tab.orders_assignment[a].crouch = Some(true);
        g.ch[c].anim.lockpick = true;

        let mut j = Job { a, t0: 3.0, live: true, knelt: false, ..Job::default() };
        lens_out(&mut g, gd, &mut j);
        assert!(!j.again && !j.live && j.t0 == -1.0);
        assert_eq!(g.tab.orders_assignment[a].crouch, Some(false));
        assert!(g.ch[c].anim.lockpick, "this file did not put the pose on him, so it does not take it off");

        j.knelt = true;
        lens_out(&mut g, gd, &mut j);
        assert!(j.again && !j.knelt);
        assert!(!g.ch[c].anim.lockpick);
    }
}
