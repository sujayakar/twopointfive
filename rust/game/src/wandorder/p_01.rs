//! src/game/wandorder.ts lines 1 to 185: a teammate looks under a shut door with the optical wand, on the player's order only.
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// This view's row for `gd`, a copy (`states.get(g)?.jobs.get(gd)`). The TypeScript's `J` is the row itself: `put_job` after a write, `lens_out` included.
fn job(g: &mut Game, gd: GuardId) -> Option<Job> {
    let k = crate::seats::view_key(g);
    g.m.wandorder.states.get(k).and_then(|st| st.jobs.get(gd).copied())
}

/// Writes a copy back over the row, if the row is still there (never sets a new one: that would move it in the walk's order).
fn put_job(g: &mut Game, gd: GuardId, j: Job) {
    let k = crate::seats::view_key(g);
    if let Some(row) = g.m.wandorder.states.get_mut(k).and_then(|st| st.jobs.get_mut(gd)) {
        *row = j;
    }
}

/// Distance on the floor. (wandorder.ts flat)
pub fn flat(a: V3, b: V3) -> f64 {
    m::hyp2(a[0] - b[0], a[2] - b[2])
}

/// Is there a fog to look into. (wandorder.ts live; WAND.on and WAND.mate are settled on)
pub fn live(g: &mut crate::game::Game) -> bool {
    crate::fogofwar::fog_of_war_active(g)
}

/// Is a wand in his kit. (wandorder.ts carries)
pub fn carries(g: &mut crate::game::Game, gd: GuardId) -> bool {
    crate::kit::kit_of(g, gd).map_or(false, |k| k.wand)
}

/// The fight has him: its lease, blind, folded, or the room fight's aim over his order. (wandorder.ts fought)
pub fn fought(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) -> bool {
    g.tab.orders_assignment[a].yield_t >= 0.0
        || g.time < g.gd[gd].dazzled_until
        || g.time < g.gd[gd].stagger_until
        || g.gd[gd].script.as_ref().map_or(false, |s| s.over.is_some())
}

/// A point half a metre off the frame on face `side`: what Doors.workSpot is asked from. (wandorder.ts onFace)
pub fn on_face(g: &mut crate::game::Game, d: DoorId, side: f64) -> V3 {
    let u = crate::doors::door_dir(g, d);
    let c = g.dr[d].frame_centre;
    [c[0] - u[1] * side * 0.5, 0.0, c[2] + u[0] * side * 0.5]
}

/// His kneeling spot on that face of the leaf. (wandorder.ts spotOn)
pub fn spot_on(g: &mut crate::game::Game, d: DoorId, side: f64) -> crate::doors::DoorsWorkSpot {
    let from = on_face(g, d, side);
    let along = g.dr[d].def.width * 0.5;
    crate::doors::doors_work_spot(g, d, from, Some(0.5), Some(along))
}

/// Who of `men` takes it and where he kneels: the nearest carrier who can walk to the door's face; else why not. (wandorder.ts pick)
pub fn pick(g: &mut crate::game::Game, men: &[GuardId], t: &mut crate::orders::OrderTarget) -> crate::wandorder::Pick {
    let crate::orders::OrderTarget::Door(t) = t else { return Pick::Text(String::from("not a door.")) };
    let d = t.door;
    if crate::doors::door_off(g, d) {
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

/// The lens comes out and he gets up; the next feed is a full one if he had been on his knee. `j` is the caller's copy: the caller puts it back. (wandorder.ts lensOut)
pub fn lens_out(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::wandorder::Job) {
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
/// (wandorder.ts settle)
pub fn settle(g: &mut crate::game::Game, gd: GuardId, j: &mut crate::wandorder::Job, line: Option<&str>, moved: bool) {
    let k = crate::seats::view_key(g);
    let c = g.gd[gd].ch;
    let d = j.door;
    lens_out(g, gd, j);
    g.m.wandorder.states.get_mut(k).unwrap().jobs.delete(gd);
    if let Some(l) = line.filter(|l| !l.is_empty()) {
        crate::game::game_say(g, gd, l, None, None, None);
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
                order: String::from("hold"),
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
                order: String::from("hold"),
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

/// Gives the one man the look, and opens his job. `Issue::Arm1(())` is the TypeScript's `void`; it never returns words. (wandorder.ts issue)
pub fn issue(g: &mut crate::game::Game, men: &[GuardId], t: &mut crate::orders::OrderTarget) -> crate::wandorder::Issue {
    let p = pick(g, men, t);
    let (Pick::Arm1(p), crate::orders::OrderTarget::Door(t)) = (p, &*t) else { return Issue::Arm1(()) };
    let st_key = crate::seats::view_key(g);
    if !g.m.wandorder.states.has(st_key) {
        return Issue::Arm1(());
    }
    if let Some(mut old) = job(g, p.gd) {
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
    let a = crate::orders::squad_orders_of(g, gd).unwrap();
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
    g.stats.wandorder.ordered += 1;
    Issue::Arm1(())
}

/// The order's beat for one man: walk, kneel, feed, look; ends when the leaf moves, the switch goes, or his mark is moved. (wandorder.ts tick)
pub fn tick(g: &mut crate::game::Game, gd: GuardId, a: OrdersAssignmentId) {
    let Some(mut j) = job(g, gd) else { return };
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
        g.stats.wandorder.moved += 1;
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
            crate::wand::wand_through(g, 0.0, now - j.t0, crouch, None)
        };
    if through {
        j.live = true;
        j.live_t = now;
    }
    put_job(g, gd, j);
    if through {
        g.stats.wandorder.live += 1;
        crate::game::game_say(g, gd, "wand's in.", None, None, None);
    }
    let what = if j.live { "looking under" } else { "feeding the wand" };
    let there = alloc::format!("{} · {}", what, crate::orders::door_words(g, d));
    g.tab.orders_assignment[a].there = there;
}

/// The order's row on the menus. (wandorder.ts row)
pub fn row(g: &mut crate::game::Game) -> OrdersOrderDefId {
    let def = crate::orders::OrderDef {
        id: String::from(WAND_ORDER),
        hot_key: Some(g.k.wand.order_key.clone()),
        label: crate::orders::OrderLabel::Text { text: String::from("look under it · wand") },
        targets: vec![crate::orders::TargetKind::Door],
        ack: Some(vec![String::from("on it."), String::from("moving.")]),
        only: Some(crate::orders::OrderOnly::Wand),
        valid: Some(crate::orders::OrderValid::Wand),
        issue: crate::orders::OrderIssue::Wand,
        tick: Some(crate::orders::OrderTick::Wand),
        ..Default::default()
    };
    g.tab.orders_order_def.push(def)
}

/// Once a frame per seat: keeps the row on the menus only while a living squad man carries a wand and there is a fog; takes the lens out of a man the
/// fight or another order has; reads the row's key while a menu lists it. True = the key was the order's (and is eaten). (wandorder.ts wandOrderKey)
pub fn wand_order_key(g: &mut crate::game::Game, input: InputId) -> bool {
    let k = crate::seats::view_key(g);
    let mut want = false;
    if live(g) {
        for gd in crate::orders::squad_orders_squad(g) {
            if g.gd[gd].state != crate::game::GuardState::Dead && carries(g, gd) {
                want = true;
                break;
            }
        }
    }
    if !g.m.wandorder.states.has(k) {
        if !want {
            return false;
        }
        g.m.wandorder.states.set(k, State { jobs: JsMap::new() });
    }
    let listed = g.orders.defs.iter().any(|&d| g.tab.orders_order_def[d].id == WAND_ORDER);
    if want && !listed {
        let after = g.orders.defs.iter().position(|&d| g.tab.orders_order_def[d].id == "hold");
        let r = row(g);
        let before = after.and_then(|i| g.orders.defs.get(i + 1)).map(|&d| g.tab.orders_order_def[d].id.clone());
        crate::orders::squad_orders_register(g, r, before.as_deref());
    } else if !want && listed {
        crate::orders::squad_orders_unregister(g, WAND_ORDER);
    }
    if !g.m.wandorder.states.get(k).unwrap().jobs.is_empty() {
        let squad = crate::orders::squad_orders_squad(g);
        let now = g.time;
        // (a live walk: settle deletes the row it stands on and calls out to the orders)
        let mut at = 0usize;
        while let Some(gd) = g.m.wandorder.states.get(k).unwrap().jobs.next_key(&mut at) {
            let mut j = job(g, gd).unwrap();
            let a = crate::orders::squad_orders_of(g, gd);
            if g.gd[gd].state == crate::game::GuardState::Dead || !squad.contains(&gd) || a != Some(j.a) {
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
    if !want {
        return false;
    }
    if g.orders.menu.is_none() {
        return false;
    }
    let key = g.k.wand.order_key.clone();
    if !crate::input::input_hit(g, input, &key) {
        return false;
    }
    let i = g.orders.menu.as_ref().unwrap().rows.iter().position(|&d| g.tab.orders_order_def[d].id == WAND_ORDER);
    let Some(i) = i else { return false };
    g.tab.input[input].pressed.delete(key);
    crate::orders::squad_orders_pick(g, Some(i as f64));
    true
}

/// His lens as his eye among the fog's, or None while he is not looking: the player's lens by every number, swept over its arc on the game's clock.
/// (wandorder.ts mateWandEye)
pub fn mate_wand_eye(g: &mut crate::game::Game, gd: GuardId) -> Option<crate::fogofwar::FogEye> {
    let seat = crate::seats::view_of(g, gd);
    crate::seats::with_seat(g, seat, &mut |g: &mut Game| eye_in_view(g, gd))
}

/// `mateWandEye` from its second line on, in his seat's view.
fn eye_in_view(g: &mut Game, gd: GuardId) -> Option<crate::fogofwar::FogEye> {
    let k = crate::seats::view_key(g);
    let st = g.m.wandorder.states.get(k)?;
    if st.jobs.is_empty() {
        return None;
    }
    let j = *st.jobs.get(gd)?;
    if !j.live || !live(g) {
        return None;
    }
    let now = g.time;
    if g.gd[gd].state == crate::game::GuardState::Dead
        || now - j.last_t > 0.25
        || now < g.gd[gd].dazzled_until
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

/// Every teammate's live lens: a device feed under 'only my own eyes'. (wandorder.ts mateWandEyes)
pub fn mate_wand_eyes(g: &mut crate::game::Game) -> Vec<crate::fogofwar::FogEye> {
    let k = crate::seats::view_key(g);
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

/// Who is looking under what, for the checks and the debugger. (wandorder.ts wandOrderJobs)
pub fn wand_order_jobs(g: &mut crate::game::Game) -> Vec<crate::wandorder::WandOrderJobsItem> {
    let k = crate::seats::view_key(g);
    let Some(st) = g.m.wandorder.states.get(k) else { return Vec::new() };
    st.jobs.iter().map(|(gd, j)| WandOrderJobsItem { gd, door: j.door, side: j.side, live: j.live, knelt: j.knelt }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_has_the_typescripts_bits() {
        assert_eq!(flat([1.25, 9.0, 7.7], [-3.1, 0.0, 2.2]).to_bits(), 0x401c0c9b269dc1c4);
        assert_eq!(flat([0.1, 0.0, 0.2], [0.3, 5.0, 0.7]).to_bits(), 0x3fe13b86ea20ae5c);
    }

    /// wandorder.ts:82-83: off a knee he was on, the next feed is a full one; never on his knee, `again` stays as it was.
    #[test]
    fn lens_out_marks_a_second_feed_only_if_he_knelt() {
        let mut g = Game::blank();
        let c = g.ch.push(crate::character::Character::blank());
        let mut man = crate::game::Guard::blank();
        man.ch = c;
        let gd = g.gd.push(man);
        let a = g.tab.orders_assignment.push(crate::orders::Assignment::zeroed());
        g.tab.orders_assignment[a].crouch = Some(true);
        g.ch[c].anim.lockpick = true;

        let mut j = Job { a, t0: 3.0, live: true, knelt: false, ..Job::zeroed() };
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
