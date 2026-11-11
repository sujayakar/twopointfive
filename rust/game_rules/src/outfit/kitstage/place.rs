//! THE STAGING: the mission's way in as a picture. Everybody else off the floor, the light, the men on their places, the lamps, the camera, and the way in as the acts read it.
use super::acts;
use super::aim::R;
use super::light::{Lamp, KIT_VIEW_LIGHT as T, KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK};
use super::plan::{porch_lit, KIT_PLAN};
use super::stage::*;
use super::types::*;
use crate::fixtures::{self as home, Fixture, FixtureKind};
use crate::game::Game;
use crate::ids::*;
use crate::scene_boxes::{box_flag, MakeBoxP};
use crate::{js, m};
use alloc::string::String;
use alloc::vec::Vec;

/// What is staged: the way's plan, its door, the hour's light as the picture has it, and whether it is the quiet picture (one subject by one pool of light).
struct Set { plan: Planned, door: Option<DoorId>, quiet: bool, lit: Lit }

/// The light of it: the lamp over the way in (where it hangs, where it points), the fill, the moon and the sky, and the share the building's own lamps burn at.
struct Lit { lamp: &'static Lamp, at: V3, aim: V3, color: V3, intensity: f64, glow: f64, fill_color: V3, fill: f64, moon: f64, sky: f64, room: f64 }

pub(super) fn place(g: &mut Game, st: &mut KitStage) {
    st.none = false;
    st.sent.clear();
    let Some(set) = set_of(g, st) else { return set_none(st) };
    sam_mark(st);
    no_fog(g);
    let _ = park(g);   // (everybody who is not of the team far off)
    let lamps = building_dark(g, st, &set);
    if set.quiet { quiet_lamps(g, st, &set, &lamps); }
    let men = stand_men(g, st, &set);
    pose(g, &men.faces);
    light_it(g, st, &set, &men);
    frame_it(g, st, &set, &men);
    way_in(g, st, &set, &men);
    live_rules(g);
    let _ = park(g);   // (the posing step walked the parked men back inside the world's edge)
    if !st.still { acts::live(g, st); }
}
fn set_none(st: &mut KitStage) { st.none = true; st.order.clear(); st.pinned = None; }

/// The set, or None: no way in, a door the level does not have, a span too short to stand at, a team the game did not make the size asked.
fn set_of(g: &mut Game, st: &mut KitStage) -> Option<Set> {
    staged_way(g, st.asks_none)?;
    let plan = st.plan.clone()?;
    drop_props(g, &plan.hidden_props);
    let a = &plan.anchor;
    let door = a.door.as_deref().and_then(|d| crate::doors::doors_by_name(g, d));
    let fit = if a.kind == "door" { door.is_some() } else { a.width >= 0.4 };
    if !fit || st.size.is_some_and(|n| ops(g).len() != n as usize) { return None; }
    // (the quiet picture is staged where the link's kind is quiet and its pool lamp burns)
    let out = lamps_out(g);
    let quiet = st.quiet_kind != 0 && QK.on && porch_lit(or(st.over.porch.intensity, QK.lamp.intensity), st.over.porch.day, plan.indoors, out) > 0.0;
    st.quiet = quiet;
    st.lit_in.clear();
    let lit = lit_of(st, &plan, quiet, out);
    Some(Set { plan, door, quiet, lit })
}

/// The light: the table, the way's own words over it, and the lamps where the plan hangs them.
fn lit_of(st: &KitStage, plan: &Planned, quiet: bool, out: bool) -> Lit {
    let o = &st.over;
    let lamp: &'static Lamp = if quiet { &QK.lamp } else { &T.porch };
    let at = plan.lamps.porch.offset;
    let aim = if quiet && !o.porch.aim { [at[0], at[1] + QK.tip, 0.0] } else { plan.lamps.porch.aim };
    let (fill_color, fill0) = if quiet { (QK.fill_color, QK.fill_intensity) } else { (T.fill.color, T.fill.intensity) };
    let mut fill = or(o.fill.intensity, fill0);
    if o.fill.intensity.is_none() && plan.src.iter().any(|(k, s)| *k == "fill.offset" && *s == Src::Derived) {
        let k = (plan.lamps.fill.offset[1] - plan.wall.face) / KIT_PLAN.fill_out;
        if k < 0.999 { fill *= m::max(0.0, k) * m::max(0.0, k); }
    }
    Lit {
        lamp, at, aim,
        color: or(o.porch.color, lamp.color),
        intensity: porch_lit(or(o.porch.intensity, lamp.intensity), o.porch.day, plan.indoors, out),
        glow: or(o.porch.fix_glow, T.fixture.glow),
        fill_color, fill,
        moon: or(o.moon_scale, if quiet { QK.moon_scale } else { T.moon_scale }),
        sky: or(o.sky_scale, if quiet { QK.sky_scale } else { T.sky_scale }),
        room: st.way_room_lights.or(o.room_lights).unwrap_or(if plan.set.is_some() { 1.0 } else { T.room_lights }),
    }
}
/// The mark of Sam's weapon light at the door's foot.
fn sam_mark(st: &mut KitStage) {
    let o = &st.over.sam;
    st.sam_on = or(o.on, !st.quiet && T.sam.on);
    st.sam_intensity = or(o.intensity, T.sam.intensity);
    st.sam_inner = or(o.inner_deg, T.sam.inner_deg);
    st.sam_outer = or(o.outer_deg, T.sam.outer_deg);
    st.sam_volumetric = or(o.volumetric, T.sam.volumetric);
}

pub(super) fn no_fog(g: &mut Game) {
    g.fog_of_war.on = false;
    g.fog_picture.viewer = false;
    crate::fogofwar::fog_of_war_reset(g);
}

/// a lamp of the building's own, that the picture darkens: not one of the groups it keeps
fn own_lamp(g: &Game, set: &Set, l: LightId) -> bool {
    const KEPT: [&str; 6] = ["exterior", "player", "flashlights", "transient", "weather", "kitview"];
    let r = &g.tab.light[l];
    r.ttl < 0.0 && r.kind != 2.0 && !KEPT.contains(&r.group.as_str()) && !set.plan.keep_lights.contains(&r.group)
}
/// The building's own lamps out, and by day the way's own, each by its own switch: a restart puts the level's switches back, so every staging. Returns, for the quiet picture, the lamps that burned.
fn building_dark(g: &mut Game, st: &KitStage, set: &Set) -> Vec<FixtureId> {
    let lit: Vec<FixtureId> = if set.quiet { g.targets.iter().copied().filter(|&t| g.tab.fixture[t].on).collect() } else { Vec::new() };
    if set.lit.room <= 0.0 {
        let mut groups: Vec<String> = Vec::new();
        for &l in &g.engine.lights.lights { if own_lamp(g, set, l) && !groups.contains(&g.tab.light[l].group) { groups.push(g.tab.light[l].group.clone()); } }
        for grp in groups { crate::lighting::set_group(g, &grp, false); }
    }
    for &t in &g.targets {
        let f = &mut g.tab.fixture[t];
        if f.kind == FixtureKind::Analytic && st.day_out.contains(&f.name) { f.on = false; }
    }
    lit
}
/// The quiet picture's lamps: one pool and no other light on the team's side; the room behind the door burns low, under its own roof.
fn quiet_lamps(g: &mut Game, st: &mut KitStage, set: &Set, lit: &[FixtureId]) {
    let (plan, mid, n) = (&set.plan, set.plan.anchor.mid, set.plan.anchor.n);
    let out_of = |p: V3| (p[0] - mid[0]) * n[0] + (p[2] - mid[1]) * n[1];
    let at = |out: f64, y: f64| -> V3 { [mid[0] + n[0] * out, y, mid[1] + n[1] * out] };
    let dist = |p: V3, q: V3| m::sqrt((p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]) + (p[2] - q[2]) * (p[2] - q[2]));
    let foot = at(plan.wall.face + KIT_PLAN.off_face, 1.0);
    let wedge = at(plan.wall.back - KIT_PLAN.lit_in, KIT_PLAN.lit_y);
    for &t in &g.targets {
        let f = &mut g.tab.fixture[t];
        let Some(l) = f.light else { continue };
        if f.kind != FixtureKind::Analytic || g.tab.light[l].kind == 2.0 { continue; }
        let range = g.tab.light[l].range;
        if out_of(f.pos) >= plan.wall.back {
            if f.on && dist(f.pos, foot) <= range { f.on = false; }
        } else if lit.contains(&t) && dist(f.pos, wedge) <= m::min(range, KIT_PLAN.lit_reach) {
            st.lit_in.push(LampAs { t, intensity: f.base_intensity, emissive: f.base_emissive });
        }
    }
    quiet_inside(g, st, QK.inside, 1.0);
    let roofs = quiet_roofs(g, plan);
    let np = js::to_i32(box_flag().no_primary);
    for &i in &roofs {
        let b = g.col.boxes[i];
        if g.engine.world.statics.get(i) != Some(&b) { continue; }
        let mut b = g.tab.boxes.edit(b);
        b.flags = (js::to_i32(b.flags) & !np) as f64;
    }
    if !roofs.is_empty() { g.boxes_changed(); }
}

/// The men as they are stood: the file, its places (Sam's first), the quiet picture's rest places, each body's way to face, and the leaf as it was.
struct Men { order: Vec<GuardId>, slots: Vec<V3>, rest: QuietPlaces, faces: Vec<(CharId, f64)>, shut: f64 }

/// THE MEN on the plan's places, the open card's man first.
fn stand_men(g: &mut Game, st: &mut KitStage, set: &Set) -> Men {
    let plan = &set.plan;
    let (order, head) = file_of(g, st);
    st.doorway = alloc::vec![plan.anchor.a, plan.anchor.b];
    st.stand = plan.slots.iter().map(|p| [p[0], p[1]]).collect();
    st.hand_right = plan.hand == "right";
    st.top_y = if plan.wall.top > 0.0 { m::min(KIT_PLAN.porch_up, plan.wall.top) } else { 1.9 };
    let mut slots: Vec<V3> = plan.slots.iter().take(order.len() + 1).copied().collect();
    let rest = if set.quiet { quiet_places(g, plan, order.len()) } else { QuietPlaces { sam: None, mate: None, back: None } };
    let mate = rest.mate.filter(|_| !order.is_empty() && Some(order[0]) != head);
    if let Some(s) = rest.sam {
        slots[0] = if head.is_some() { rest.back.unwrap_or(s) } else { s };
        st.stand.push([s[0], s[1]]);
        if let Some(q) = rest.mate.filter(|_| !order.is_empty()) { st.stand.push([q[0], q[1]]); }
    }
    let sam = g.pl[g.player].ch;
    crate::game::game_teleport_player(g, [slots[0][0], 0.0, slots[0][1]]);
    g.ch[sam].aim_pitch = 0.0;
    st.foot = at_of(plan, [0.0, 0.0, 0.0]);
    let mut faces = alloc::vec![(sam, super::walk::faces(st, [slots[0][0], slots[0][1]]))];
    for (i, &x) in order.iter().enumerate() {
        let p = if i == 0 { mate.unwrap_or(slots[i + 1]) } else { slots[i + 1] };
        let c = g.gd[x].ch;
        crate::character::pos_replace(g, c, [p[0], 0.0, p[1]]);
        g.ch[c].aim_pitch = 0.0;
        crate::duty::hold(g, x, true, "the kit picture");
        crate::duty::pin(g, x, true, "the kit picture");
        faces.push((c, super::walk::faces(st, [p[0], p[1]])));
    }
    let shut = set.door.map_or(0.0, |d| g.dr[d].leaf.angle);
    let (rifle, pistol) = (g.pl[g.player].rifle, g.pl[g.player].pistol);
    g.fa[rifle].light_on = st.sam_on;
    g.fa[pistol].light_on = st.sam_on;
    if set.quiet { sam_low(g, set, slots[0]); }
    Men { order, slots, rest, faces, shut }
}
/// The file, and the open card's man where he is of it. A restage keeps the file's order, the men's places by team index: a restart makes new men. The open card's man
/// goes to the head of it by the card switch's own walk, the fourth man round Sam (`live`). Only a first staging, and the still picture, which has no walks, stand him first.
fn file_of(g: &Game, st: &mut KitStage) -> (Vec<GuardId>, Option<GuardId>) {
    let team = ops(g);
    st.who = if st.want >= 1 && st.want as usize <= team.len() { st.want } else { 0 };
    let head = acts::actor(g, st).filter(|h| team.contains(h));
    let keep = st.keep.take();
    let kept_order = keep.is_some() && !st.still;
    let kept: Vec<GuardId> = keep.unwrap_or_default().iter().filter_map(|&i| g.staged.team.get(i).copied()).filter(|x| team.contains(x)).collect();
    let mut file: Vec<GuardId> = kept.iter().copied().chain(team.iter().copied().filter(|x| !kept.contains(x))).collect();
    if let Some(h) = head.filter(|_| !kept_order) { file.retain(|&x| x != h); file.insert(0, h); }
    (file, head)
}
/// The quiet picture: Sam is down, his gun put away, the leaf a hand ajar by the game's own hand on it; a way-in door found locked is simply unlocked.
fn sam_low(g: &mut Game, set: &Set, at: V3) {
    let pl = g.player;
    let sam = g.pl[pl].ch;
    g.pl[pl].crouch = true;
    g.pl[pl].holstered = true;
    g.ch[sam].anim.crouch = 1.0;
    g.ch[sam].anim.crouch_target = 1.0;
    let Some(d) = set.door.filter(|&d| !g.dr[d].leaf.off) else { return };
    g.dr[d].leaf.locked = false;
    let id = g.ch[sam].id;
    crate::doors::doors_crack(g, d, [at[0], 0.0, at[1]], id, 1.0, None, Some(QK.ajar_deg * R), Some(QK.ajar_deg * R));
}
/// ONE ZERO-LENGTH STEP, hands off the controls: nothing advances, the men are posed where and how they now stand. The yaws written before and after it.
fn pose(g: &mut Game, faces: &[(CharId, f64)]) {
    let turn = |g: &mut Game| for &(c, y) in faces { g.ch[c].body_yaw = y; g.ch[c].aim_yaw = y; };
    turn(g);
    if let (Some(input), Some(cam)) = (g.host_input, g.host_cam) {
        hands_off(g, input);
        crate::game::game_update(g, 0.0, input, cam, crate::host_hud::View { client_width: 1600.0, client_height: 1000.0, width: 1600.0, height: 1000.0 });
    }
    turn(g);
}

/// THE LIGHTS, after the step: the game writes each weapon light in it.
fn light_it(g: &mut Game, st: &mut KitStage, set: &Set, men: &Men) {
    st.order = men.order.clone();
    st.sam_aim = at_of(&set.plan, or(st.over.sam.aim, T.sam.aim));
    st.a.beat = None;
    torches(g, st);
    for l in g.engine.lights.lights.clone() { if own_lamp(g, set, l) { g.tab.light[l].intensity *= set.lit.room; } }
    if set.quiet { room_behind(g, st); }
    moon_and_sky(g, st, set);
    hang_porch_and_fill(g, st, set);
    porch_box(g, st, set);
    if set.quiet { pool_lamp(g, st, set); warm_inside(g, st, set); }
}
fn room_behind(g: &mut Game, st: &KitStage) {
    for a in &st.lit_in {
        let f = &g.tab.fixture[a.t];
        let Some(l) = f.light else { continue };
        g.tab.light[l].enabled = f.on && !f.broken;
        g.tab.light[l].intensity = f.base_intensity;
        home::light_home_feed(g, l);
    }
}
fn moon_and_sky(g: &mut Game, st: &mut KitStage, set: &Set) {
    st.moon_at = None;
    st.sky_at = None;
    let Some(light) = g.engine.lights.lights.iter().copied().find(|&l| g.tab.light[l].name == "moon" && g.tab.light[l].kind == 2.0) else { return };
    let t = if set.quiet { QK.moon_tint } else { [1.0, 1.0, 1.0] };
    let row = &mut g.tab.light[light];
    row.color = [row.color[0] * t[0], row.color[1] * t[1], row.color[2] * t[2]];
    if !row.enabled || row.intensity <= 0.0 { return; }
    row.intensity *= set.lit.moon;
    st.moon_at = Some(LightAs { light, intensity: row.intensity });
    let (z, h) = crate::hours::told::sky_now(g);
    let k = set.lit.sky;
    st.sky_at = Some(([z[0] * k, z[1] * k, z[2] * k], [h[0] * k, h[1] * k, h[2] * k]));
}
fn hang_porch_and_fill(g: &mut Game, st: &mut KitStage, set: &Set) {
    let (plan, lit) = (&set.plan, &set.lit);
    let (o, base) = (&st.over.porch, lit.lamp);
    let (at, to) = (at_of(plan, lit.at), at_of(plan, lit.aim));
    let porch = Hung { kind: or(o.kind, base.kind), pos: at, dir: unit(at, to), color: lit.color, intensity: lit.intensity, range: or(o.range, base.range), inner: or(o.inner_deg, base.inner_deg), outer: or(o.outer_deg, base.outer_deg), radius: or(o.radius, base.radius), volumetric: or(o.volumetric, base.volumetric), broad: or(o.broad, base.broad) };
    let (o, base) = (&st.over.fill, &T.fill);
    let (at, to) = (at_of(plan, plan.lamps.fill.offset), at_of(plan, plan.lamps.fill.aim));
    let fill = Hung { kind: or(o.kind, base.kind), pos: at, dir: unit(at, to), color: or(o.color, lit.fill_color), intensity: lit.fill, range: or(o.range, base.range), inner: or(o.inner_deg, base.inner_deg), outer: or(o.outer_deg, base.outer_deg), radius: or(o.radius, base.radius), volumetric: or(o.volumetric, base.volumetric), broad: or(o.broad, base.broad) };
    st.porch = Some(hang(g, "kitview_porch", &porch));
    st.fill = Some(hang(g, "kitview_fill", &fill));
    st.fill_at = lit.fill;
}
fn glow(lit: &Lit) -> V3 { [lit.color[0] * lit.glow, lit.color[1] * lit.glow, lit.color[2] * lit.glow] }
/// The porch lamp's glowing box on the wall (a restart cuts the world back to its authored boxes: put again).
fn porch_box(g: &mut Game, st: &mut KitStage, set: &Set) {
    let plan = &set.plan;
    let Some(fx) = plan.lamps.porch.fixture.as_ref().filter(|_| set.lit.intensity > 0.0) else { return };
    let mut p = MakeBoxP::default();
    p.c = at_of(plan, fx.at);
    p.h = or(st.over.porch.fix_half, T.fixture.half);
    p.yaw = Some(m::atan2(plan.s[1], plan.s[0]));
    p.albedo = Some([0.04, 0.04, 0.04]);
    p.emissive = Some(glow(&set.lit));
    let f = box_flag();
    p.flags = Some((js::to_i32(f.emissive) | js::to_i32(f.area_lit)) as f64);
    let b = crate::scene_boxes::make_box(g, &p);
    crate::scene_boxes::box_world_add_static(g, b);
    st.fixture = Some(b);
}
/// The quiet picture's pool lamp is a lamp of the game's, so that a round puts it out as it puts out any fixture and a restart's repair mends it.
fn pool_lamp(g: &mut Game, st: &mut KitStage, set: &Set) {
    let porch = st.porch.unwrap();
    let mut d = Fixture::blank();
    d.name = String::from("kitview_porch"); d.group = String::from("kitview"); d.kind = FixtureKind::Analytic; d.pos = g.tab.light[porch].pos; d.light = Some(porch);
    d.base_emissive = glow(&set.lit);
    d.base_intensity = set.lit.intensity;
    d.on = true; d.factor = 1.0; d.interactive = true;
    let p = home::fixture_add(g, d);
    st.pool = Some(p);
    g.targets.push(p);
    crate::game::game_stand_lights(g);
    pool_sync(g, st);
}
/// And behind the door, by the leaf's free edge (a gap: its middle), a small warm lamp of the picture's own.
fn warm_inside(g: &mut Game, st: &mut KitStage, set: &Set) {
    let plan = &set.plan;
    if plan.anchor.kind == "wall" { return; }
    let edge = set.door.map_or(0.0, |d| free_edge(g, plan, d, plan.anchor.mid));
    let o = &QK.own;
    let pos = at_of(plan, [edge * (plan.anchor.width / 2.0 + o.side), plan.wall.back - o.back, o.up]);
    st.inside = Some(hang(g, "kitview_inside", &Hung { kind: 0.0, pos, dir: [0.0, -1.0, 0.0], color: o.color, intensity: o.intensity, range: o.range, inner: 0.0, outer: 180.0, radius: o.radius, volumetric: 0.0, broad: false }));
}
/// which way along the wall, toward the stack or from it, the leaf's free edge is from a point of it
fn free_edge(g: &Game, plan: &Planned, d: DoorId, at: V2) -> f64 {
    let h = g.dr[d].def.hinge;
    if (at[0] - h[0]) * plan.s[0] + (at[1] - h[1]) * plan.s[1] >= 0.0 { 1.0 } else { -1.0 }
}

/// THE CAMERA: the plan's pin, framed.
fn frame_it(g: &mut Game, st: &mut KitStage, set: &Set, men: &Men) {
    let plan = &set.plan;
    let sam: V2 = [men.slots[0][0], men.slots[0][1]];
    st.look = plan.cam.look;
    st.yaw = plan.cam.yaw;
    st.pitch = plan.cam.pitch;
    st.dist = plan.cam.dist;
    st.sam_rest = men.rest.sam.map_or(sam, |s| [s[0], s[1]]);
    st.fit = if set.quiet { let lamp = match st.fixture { Some(f) => g.tab.boxes[f].c, None => g.tab.light[st.porch.unwrap()].pos }; Some((lamp, st.sam_rest)) } else { None };
    st.s = plan.s;
    st.n = plan.anchor.n;
    st.mid = plan.anchor.mid;
    st.sam = sam;
    st.spots = men.slots[1..].iter().map(|p| [p[0], p[1]]).collect();
    st.mate = men.rest.mate.filter(|_| !men.order.is_empty()).map(|p| [p[0], p[1]]);
    aim_stage(g, st);
}

/// THE WAY IN as the acts read it. How far in a can or a round may go: the plan's free floor behind the anchor's line; a blank wall: what a hole opens onto.
fn way_in(g: &Game, st: &mut KitStage, set: &Set, men: &Men) {
    let (plan, a) = (&set.plan, &set.plan.anchor);
    let (foot, sam) = (st.foot, men.slots[0]);
    st.walled = a.kind == "wall";
    st.door = set.door;
    st.width = a.width;
    st.hole = false;
    st.side = set.door.map_or(1.0, |d| crate::doors::doors_side(g, d, [sam[0], 0.0, sam[1]]));
    st.latch = set.door.map_or(1.0, |d| free_edge(g, plan, d, [foot[0], foot[2]]));
    st.reach_in = if st.walled { plan.beyond - plan.wall.back } else { plan.inside };
    st.spots_in = floor_in(g, st, plan, &K.in_m);
    st.spots_smoke = floor_in(g, st, plan, &K.smoke_m);
    st.act4 = super::plan::act4(a.mid, plan.s, a.n, a.width, &plan.slots, Some(&|x: f64, z: f64| free(g, x, z)), None);
    st.frag_m = m::max(0.6, m::min(K.sam_frag_m, st.reach_in - 0.5));
    st.frag_land = m::min(K.frag_land, m::max(0.3, st.frag_m - 0.5));
    st.leaf_shut = men.shut;
    st.leaf_rest = set.door.map_or(0.0, |d| g.dr[d].leaf.angle);
    st.sam_smoke = at_of(plan, [0.0, -K.sam_smoke_m, 0.0]);
    st.sam_stun = at_of(plan, [0.0, -K.sam_stun_m, 0.0]);
    st.sam_frag = at_of(plan, [0.0, -st.frag_m, 0.0]);
    st.sam_back = men.rest.back.map(|q| [q[0], q[1]]);
    st.four = false;
}
/// The floor spots inside, of those asked that fit, at each step along; where none fits and there is any floor at all, half way in.
fn floor_in(g: &Game, st: &KitStage, plan: &Planned, asked: &[f64]) -> Vec<V3> {
    let mut ins: Vec<f64> = asked.iter().copied().filter(|&x| x <= st.reach_in - 0.3).collect();
    if ins.is_empty() && st.reach_in >= 0.8 { ins.push(st.reach_in / 2.0); }
    let spots = ins.iter().flat_map(|&x| K.in_along.iter().map(move |&k| (k, x)));
    spots.map(|(k, x)| at_of(plan, [k * st.latch, -x, 0.0])).filter(|p| free(g, p[0], p[2])).collect()
}

/// LIVE: Sam cannot be killed and his belt refills, no autosave, no response team, the game's turn weight on the picture's numbers, every teammate unhurt; a flash at the
/// door is there to be seen, so the team's own, Sam's too, finds none of them.
fn live_rules(g: &mut Game) {
    g.god_mode = true;
    g.infinite_ammo = true;
    g.saves.enabled = false;
    g.tune.qrf = false;
    let t = &mut g.k.turn;
    t.aim = m::round(K.gun_rate * R * 1000.0) / 1000.0;
    t.aim_acc = m::round(K.gun_rate * R / K.ramp * 100.0) / 100.0;
    t.body = m::round(K.body_rate * R * 1000.0) / 1000.0;
    t.body_acc = m::round(K.body_rate * R / K.ramp * 100.0) / 100.0;
    for &x in &g.staged.team { g.gd[x].god = true; }
    g.k.samstun.own_seen = false;
    g.k.flash.own_m = 0.0;
}
