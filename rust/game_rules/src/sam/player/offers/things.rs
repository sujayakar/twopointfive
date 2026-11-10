//! A THING'S ROWS: the lamps, what the errand is for, the breaker.
use super::*;

/// Every lamp that can be worked is marked; none has a row.
pub(super) fn lamps(g: &Game, o: &mut Offers) {
    for i in 0..g.targets.len() {
        let id = g.targets[i];
        let f = &g.tab.fixture[id];
        if f.broken || !f.interactive { continue; }
        let dead = crate::carlamps::dead_lamp(g, id);
        let f = &g.tab.fixture[id];
        let lit = !dead && f.factor > 0.0;
        let line1 = if lit || dead { f.name.clone() } else { format!("{} (off)", f.name) };
        let line2 = if dead { "click  shoot" } else { "right-click  OCP (the pistol's)  ·  click  shoot" };
        o.list.push(Interactable { target: Some(id), off: !lit, ..mark(InteractableKind::Light, f.pos, line1, line2, true) });
    }
}

/// What the errand is for, while it is still in what holds it.
pub(super) fn errand(g: &Game, o: &mut Offers, h: Him, strikes: bool) {
    if g.mission.stage != crate::game::MissionStage::Drive { return; }
    let reach = crate::mission::rack_dist(g, h.p) < crate::consts::DRIVE_REACH && !h.drags && !h.sprints && !strikes;
    let e = &g.level.errand;
    let mut it = mark(InteractableKind::Objective, e.front, format!("{} · {}", e.holder, e.thing), if reach { "" } else if h.drags { "hands full" } else { "step closer" }, reach);
    if let Some(t) = g.pl[h.pl].taking() {
        let part = t / crate::consts::DRIVE_SECS;
        it.progress = Some(m::min(1.0, part));
        it.line2 = format!("taking… {}%  ·  stay on it", pct(part));
    }
    let take = if reach && !h.busy { Some(row(&it, Verb::Drive, &format!("take {} — stay on it", e.thing), "")) } else { None };
    o.add(it, take.into_iter().collect());
}

/// The mains' breaker, in sight of its face.
pub(super) fn breaker(g: &Game, o: &mut Offers, h: Him) {
    let at = g.level.breaker.pos;
    let (thrown, for_good) = (g.light_home.mains.active, g.light_home.mains.permanent);
    let far = v3::dist_xz(at, h.p);
    if (thrown && for_good) || far >= NEARBY_M || !sees_panel(g, h, at) { return; }
    let reach = far < 1.15 && !h.drags && !h.sprints;
    let it = Interactable { off: thrown, ..mark(InteractableKind::Breaker, [at[0], at[1] + 0.35, at[2]], if thrown { "breaker · thrown" } else { "breaker · power on" }, if reach { "" } else { "step closer" }, reach) };
    let throw = if !reach || h.busy { None }
        else if thrown { Some(row(&it, Verb::BreakerOn, "turn the power back on", "")) }
        else { Some(row(&it, Verb::BreakerOff, "throw it — kill the lights", "")) };
    o.add(it, throw.into_iter().collect());
}
fn sees_panel(g: &Game, h: Him, at: V3) -> bool {
    let eye: V3 = [h.p[0], if h.low { 0.9 } else { 1.3 }, h.p[2]];
    if !crate::scene_collision::solid_blocked(g, eye, [at[0], at[1], at[2] - 0.18], None) { return true; }
    crate::workthings::sam_panel_point(g, h.p).is_some_and(|p| !crate::scene_collision::solid_blocked(g, eye, p, None))
}
