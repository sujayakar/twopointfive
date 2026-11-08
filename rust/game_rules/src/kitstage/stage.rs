//! THE KIT PICTURE'S STAGE: the Team & kit screen's picture of the team at the mission's own way in. The mission is started as Deploy would start it; then this stands the team on the plan's places, parks
//! everybody else far off and calm, hangs the lamps and pins the camera. NO RULE OF THE GAME READS THE STAGE: it writes only what any host could write (men, lights, boxes, a pin) round the tick.
use super::acts;
use super::aim::R;
use super::light::{KIT_VIEW_LIGHT as T, KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK, KIT_VIEW_STAGE as ST};
use super::plan::{porch_lit, KIT_PLAN};
use super::types::*;
use crate::cmd::PlayerCmd;
use crate::game::{Game, GuardState};
use crate::ids::*;
use crate::scene_boxes::{box_flag, MakeBoxP};
use crate::sim_lightshome::{self as home, light_in, FixtureDef, FixtureDefKind, FixtureKind};
use crate::{js, m};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

/// a man no event can reach: past any number an event can get
const DEAF: f64 = 9007199254740991.0;

fn unit(a: V3, b: V3) -> V3 {
    let v = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len = m::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    let len = if len != 0.0 { len } else { 1.0 };
    [v[0] / len, v[1] / len, v[2] / len]
}
fn free(g: &Game, x: f64, z: f64) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, x, z) }
fn take(g: &mut Game) -> Option<Box<KitStage>> { g.kit_stage.take() }

/// A point by [along the wall toward the stack, out on the team's side, up] from the anchor's middle.
fn at_of(p: &Planned, o: V3) -> V3 {
    let (mid, s, n) = (p.anchor.mid, p.s, p.anchor.n);
    [mid[0] + s[0] * o[0] + n[0] * o[1], o[2], mid[1] + s[1] * o[0] + n[1] * o[1]]
}

/// THE WAY THE MISSION THAT IS UP STAGED ITS TEAM ON, whatever its kind, and whether it has ended. A quiet mission and the solo night have way 0 and their own men.
struct Way { active: bool, way: usize, ended: bool, fixed: bool }
fn staged_way_of(g: &Game, asks_none: bool) -> Way {
    if crate::roomclear::room_clear_scenario_active(g) { return Way { active: true, way: g.room_clear.way as usize, ended: g.room_clear.end.is_some(), fixed: false }; }
    if g.rescue.active { return Way { active: true, way: 0, ended: g.rescue.result.is_some(), fixed: false }; }
    if g.night_breach.active { return Way { active: true, way: 0, ended: false, fixed: false }; }
    if g.quiet.active { return Way { active: true, way: 0, ended: false, fixed: true }; }
    Way { active: asks_none, way: 0, ended: false, fixed: true }
}
/// the way a game stood from a link stages: its index, or none (no mission started)
pub fn staged_way(g: &Game, asks_none: bool) -> Option<usize> { let w = staged_way_of(g, asks_none); if w.active { Some(w.way) } else { None } }

fn ops(g: &Game) -> Vec<GuardId> { crate::scenario_run::team_of(g).into_iter().filter(|&x| g.gd[x].state != GuardState::Dead).collect() }

/// EVERYBODY WHO IS NOT THE TEAM, far off, dark, held, calm and deaf, the level's devices inert, and the alarm a stray noise placed dropped while the floor is at 0. A blast is made inside the step, so the mark
/// on a man's ears is set past any number an event can get, and renewed here. Answers what it had to put right: men found roused, the alarm's marks dropped.
pub fn park(g: &mut Game) -> (u32, u32) {
    g.tune.devices = false;
    g.tune.lasers = false;
    for i in 0..g.events.len() { if g.events[i].id.is_none() { g.event_seq += 1.0; g.events[i].id = Some(g.event_seq); } }
    let (mut k, mut men, mut alarms) = (0.0, 0, 0);
    let all: Vec<GuardId> = g.guards.iter().chain(g.civilians.iter()).copied().collect();
    for x in all {
        if g.gd[x].team == crate::teams::Team::Operative { continue; }
        let (c, light) = (g.gd[x].ch, g.gd[x].flashlight);
        crate::character::pos_replace(g, c, [-40.0 - k * 3.0, 0.0, -40.0]);
        k += 1.0;
        g.tab.light[light].enabled = false;
        crate::control::set_hold(g, x, true, "kitstage");
        crate::control::set_pinned(g, x, true, "kitstage");
        g.gd[x].heard_up_to = DEAF;
        crate::mind::set(g, x, 0.0, "kitstage");
        crate::mind::fix_keep(g, x, None, "kitstage");
        if g.gd[x].state != GuardState::Dead {
            if g.gd[x].state != GuardState::Patrol { men += 1; }
            crate::mind::set_state(g, x, GuardState::Patrol, "kitstage");
        }
    }
    if g.escalation == 0.0 {
        if g.alarm.episode { alarms += 1; }
        if g.alarm.placed { alarms += 1; }
        g.alarm.episode = false;
        g.alarm.placed = false;
    }
    (men, alarms)
}

/// THE BOXES OUT OF THE PICTURE: sunk 30 m under the ground with the trim they say (a box is not taken out of the world, whose authored count a restart cuts back to), and off the floor: the nav grid asks its
/// bake's question again where they stood, and the sound's paths are thrown away. `idx` = places in the level's list.
pub fn sunk(g: &mut Game, idx: &[usize]) {
    let list: Vec<BoxId> = g.col.boxes.clone();
    let mut all: Vec<usize> = idx.to_vec();
    if let Some(first) = list.iter().position(|&b| g.tab.boxes[b].piece == Some(true)) {
        for i in first..list.len() {
            let b = &g.tab.boxes[list[i]];
            if b.piece == Some(true) { if let Some(of) = b.of { if idx.iter().any(|&o| o as f64 == of) { all.push(i); } } }
        }
    }
    let Some(mut st) = take(g) else { return };
    let mut n = 0;
    let (mut x0, mut z0, mut x1, mut z1) = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for i in all {
        let Some(&b) = list.get(i) else { continue };
        // (index for index the level's boxes head the world's: one that is not is left alone)
        if st.sunk.contains(&i) || g.engine.world.statics.get(i) != Some(&b) { continue; }
        let row = &mut g.tab.boxes[b];
        let r = m::hyp2(row.h[0], row.h[2]);
        x0 = m::min(x0, row.c[0] - r); x1 = m::max(x1, row.c[0] + r); z0 = m::min(z0, row.c[2] - r); z1 = m::max(z1, row.c[2] + r);
        row.c[1] -= 30.0;
        st.sunk.push(i);
        n += 1;
    }
    g.kit_stage = Some(st);
    if n > 0 {
        box_changed(g);
        let _ = crate::scene_collision::nav_grid_rebake(g, x0, z0, x1, z1);
        crate::scene_collision::sound_paths_invalidate(g);
    }
}
/// the game's own word that a static was written: the mirror compares them again, and what is drawn off the list sees it
fn box_changed(g: &mut Game) {
    g.engine.world.geom_serial += 1.0;
    crate::scene_collision::wrote(g);
}
pub(super) fn drop_props(g: &mut Game, hidden: &[usize]) {
    for &i in hidden { if let Some(&p) = g.props.all.get(i) { crate::props::prop_system_remove(g, p); } }
}

/// the room behind the door at this share of itself: each lamp's own base, which the game multiplies each step, and its fixture's glow
pub(super) fn quiet_inside(g: &mut Game, st: &mut KitStage, k: f64, blade: f64) {
    st.share = k;
    st.a.blade = blade;
    let li = light_in();
    for a in &st.lit_in {
        home::light_home_in(g, li.on, Some(a.t), Some(1.0), None, None, None);
        home::light_home_in(g, li.base, Some(a.t), Some(a.intensity * k), None, None, None);
        g.tab.sim_lightshome_fixture[a.t].base_emissive = [a.emissive[0] * k, a.emissive[1] * k, a.emissive[2] * k];
    }
    if let Some(o) = st.inside { let l = &mut g.tab.light[o]; l.intensity = QK.own.intensity * blade; l.peak_intensity = l.intensity; }
}
pub(super) fn pool_sync(g: &mut Game, st: &KitStage) {
    let Some(p) = st.pool else { return };
    let i = st.fixture.and_then(|f| g.engine.world.statics.iter().position(|&b| b == f));
    let row = &mut g.tab.sim_lightshome_fixture[p];
    row.boxes = i.map_or(Vec::new(), |i| alloc::vec![i as f64]);
    if st.porch.is_some() { row.light = st.porch; }
}
/// THE ROOF STAYS ON: the level's boxes, by place, that the quiet picture draws though the game's camera looks through them: a cover flagged NoPrimary, no light's panel, that lies wholly behind the wall. None
/// where the team itself stands under a roof.
fn quiet_roofs(g: &Game, p: &Planned) -> Vec<usize> {
    if !QK.roofs || p.indoors { return Vec::new(); }
    let (mid, n, lim) = (p.anchor.mid, p.anchor.n, p.wall.face + QK.roof_eave);
    let out_of = |x: f64, z: f64| (x - mid[0]) * n[0] + (z - mid[1]) * n[1];
    let f = box_flag();
    let (np, not) = (js::to_i32(f.no_primary), js::to_i32(f.no_shadow) | js::to_i32(f.emissive) | js::to_i32(f.area_lit));
    let mut got = Vec::new();
    for (i, &id) in g.col.boxes.iter().enumerate() {
        let b = &g.tab.boxes[id];
        let fl = js::to_i32(b.flags);
        if fl & np == 0 || fl & not != 0 || b.c[1] - b.h[1] < KIT_PLAN.cover_y || m::min(b.h[0], b.h[2]) < KIT_PLAN.cover_half { continue; }
        let far = if b.yaw != 0.0 { out_of(b.c[0], b.c[2]) + m::hyp2(b.h[0], b.h[2]) <= lim }
            else { [[1.0, 1.0], [1.0, -1.0], [-1.0, 1.0], [-1.0, -1.0]].iter().all(|o: &[f64; 2]| out_of(b.c[0] + o[0] * b.h[0], b.c[2] + o[1] * b.h[2]) <= lim) };
        if far { got.push(i); }
    }
    got
}

/// THE REST PLACES: Sam crouched by the pool on the side the plan stands him, at the wall men's distance, a partner behind him. A place the level wrote wins, and so does the plan's where the worked-out one is not free floor.
struct QuietPlaces { sam: Option<V3>, mate: Option<V3>, back: Option<V3> }
fn quiet_places(g: &Game, p: &Planned, mates: usize) -> QuietPlaces {
    let (a, s) = (&p.anchor, p.s);
    let (n, mid, out) = (a.n, a.mid, p.wall.face + KIT_PLAN.off_face);
    let al0 = (p.slots[0][0] - mid[0]) * s[0] + (p.slots[0][1] - mid[1]) * s[1];
    let sg = if al0 > 0.0 { 1.0 } else { -1.0 };
    let at = |al: f64| -> V3 { [mid[0] + s[0] * sg * al + n[0] * out, mid[1] + s[1] * sg * al + n[1] * out, m::atan2(-s[0] * sg, -s[1] * sg) / R] };
    let ok = |q: &V3| free(g, q[0], q[1]);
    let authored = |i: usize| p.slot_src.get(i).copied() == Some(Src::Authored);
    let sam = if !authored(0) { Some(at(QK.sam_along)) } else { None };
    let mate = if mates >= 1 && !authored(1) { Some(at(QK.sam_along + QK.mate_gap)) } else { None };
    let sam_ok = sam.filter(|q| ok(q));
    let back = if sam_ok.is_some() && mates >= 1 && a.kind != "wall" { Some(at(QK.sam_along + QK.step_back)) } else { None };
    QuietPlaces { sam: sam_ok, mate: if sam_ok.is_some() { mate.filter(|q| ok(q)) } else { None }, back: back.filter(|q| ok(q)) }
}

fn lamps_out(g: &Game) -> bool {
    let h = crate::hours::hour_of(g);
    h != crate::hours::HourId::Night && !crate::hours::hours().values().any(|d| d.id == h && d.lamps)
}
/// the hour has the level's outside lamps out: the porch lamp is on the same photocell
pub fn hour_has_lamps_out(g: &Game) -> bool { lamps_out(g) }

fn or<T: Copy>(a: Option<T>, b: T) -> T { a.unwrap_or(b) }

/// a hung lamp as the picture has it
struct Hung { kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner: f64, outer: f64, radius: f64, volumetric: f64, broad: bool }
fn hang(g: &mut Game, name: &str, h: &Hung) -> LightId {
    let mut l = crate::render_lights::RtLight::zeroed();
    l.name = String::from(name); l.group = String::from("kitview"); l.fixture_box = -1.0; l.ttl = -1.0;
    let id = g.tab.light.push(l);
    crate::render_lights::light_set_add(g, id);
    let l = &mut g.tab.light[id];
    l.kind = h.kind; l.pos = h.pos; l.dir = h.dir; l.color = h.color; l.intensity = h.intensity; l.peak_intensity = h.intensity; l.range = h.range; l.inner_deg = h.inner; l.outer_deg = h.outer; l.radius = h.radius; l.volumetric = h.volumetric; l.broad = h.broad; l.enabled = h.intensity > 0.0;
    id
}

/// a lamp of the picture's own, for a row: a point, hung until it is taken off the list
pub(super) fn hang_lamp(g: &mut Game, name: &str, kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner: f64, outer: f64, radius: f64) -> LightId {
    hang(g, name, &Hung { kind, pos, dir, color, intensity, range, inner, outer, radius, volumetric: 0.0, broad: false })
}

/// the team's lights off, Sam's dim and tight at the door's foot (the game writes each weapon light in each step)
fn torches(g: &mut Game, st: &mut KitStage) {
    let (lit, lit_man) = st.a.beat.as_ref().map_or((false, None), |b| (b.lit, b.man));
    let beam = st.a.beam;
    for &x in &st.order {
        let on = lit && lit_man == Some(x);
        let l = g.gd[x].flashlight;
        let r = &mut g.tab.light[l];
        r.enabled = on;
        if on { r.intensity = K.beam; r.peak_intensity = K.beam; if let Some(d) = beam { r.dir = d; } }
    }
    let pl = g.player;
    let id = g.pl[pl].light;
    let pos = g.tab.light[id].pos;
    let l = &mut g.tab.light[id];
    let cone = *st.a.sam_cone.get_or_insert((l.inner_deg, l.outer_deg, l.volumetric));
    if lit && lit_man.is_none() {
        l.enabled = true;
        l.intensity = K.beam; l.peak_intensity = K.beam; l.inner_deg = cone.0; l.outer_deg = cone.1; l.volumetric = cone.2;
        if let Some(d) = beam { l.dir = d; }
        return;
    }
    if !st.sam_on { l.enabled = false; return; }
    l.dir = unit(pos, st.sam_aim);
    l.intensity = st.sam_intensity; l.peak_intensity = st.sam_intensity; l.inner_deg = st.sam_inner; l.outer_deg = st.sam_outer; l.volumetric = st.sam_volumetric;
}

/// every man on his place, as the squad's own hand-over has it (a staged man's `hold` is only "nothing moves until the first order", and with no order of his own he drifts after Sam): sent once, and again whenever he is found with no such order (a fresh man has the scenario's own until his first)
fn home_men(g: &mut Game, st: &mut KitStage) {
    for k in 0..st.order.len() {
        let x = st.order[k];
        let is_open = acts::actor(g, st) == Some(x);
        let (p, yaw, low) = match st.mate { Some((p, y)) if k == 0 && !is_open => (p, y, st.quiet), _ => (st.spots[k], st.yaws[k], false) };
        let mine = crate::orders::squad_orders_of(g, &crate::seats::here(g), x).map_or(false, |a| g.tab.orders_assignment[a].order == crate::orders::rows::cover::ORDER.id);
        if mine && st.sent.contains(&x) { continue; }
        super::walk::send(g, x, p, yaw, None, K.pace, true, low);
        if !st.sent.contains(&x) { st.sent.push(x); }
    }
}
fn hands_off(g: &mut Game, i: InputId) { let r = &mut g.tab.input[i]; r.keys.clear(); r.pressed.clear(); r.buttons = 0.0; r.clicked = 0.0; }

/// THE STAGING, off the stage's plan. On a failure the stage says `none`, never a panic.
pub fn stage(g: &mut Game) {
    let Some(mut st) = take(g) else { return };
    place(g, &mut st);
    g.kit_stage = Some(st);
}
fn set_none(st: &mut KitStage) { st.none = true; st.order.clear(); st.pinned = None; }
fn place(g: &mut Game, st: &mut KitStage) {
    st.none = false;
    st.sent.clear();
    let sw = staged_way_of(g, st.asks_none);
    if !sw.active { return set_none(st); }
    let Some(plan) = st.plan.clone() else { return set_none(st) };
    drop_props(g, &plan.hidden_props);
    let a = &plan.anchor;
    let door = a.door.as_deref().and_then(|d| crate::doors::doors_by_name(g, d));
    let team = ops(g);
    // (a door the level does not have, a span too short to stand at, a team the game did not make the size asked: no picture)
    let fit = if a.kind == "door" { door.is_some() } else { a.width >= 0.4 };
    if !fit || st.size.map_or(false, |n| team.len() != n as usize) { return set_none(st); }

    // THE LIGHT: the table, the way's own keys over it, and the lamps where the plan hangs them. The quiet picture is staged where the link's kind is quiet and its pool lamp burns.
    let out_now = lamps_out(g);
    let o = st.over.clone();
    let quiet_lit = porch_lit(or(o.porch.intensity, QK.lamp.intensity), o.porch.day, plan.indoors, out_now);
    let q = st.quiet_kind != 0 && QK.on && quiet_lit > 0.0;
    st.quiet = q;
    st.out.clear();
    st.lit_in.clear();
    st.share = 1.0;
    let (lamp0, offset_q, aim_q) = if q {
        let po = plan.lamps.porch.offset;
        (&QK.lamp, po, if o.porch.aim { plan.lamps.porch.aim } else { [po[0], po[1] + QK.tip, 0.0] })
    } else { (&T.porch, plan.lamps.porch.offset, plan.lamps.porch.aim) };
    let porch_color = or(o.porch.color, lamp0.color);
    let porch_i = porch_lit(or(o.porch.intensity, lamp0.intensity), o.porch.day, plan.indoors, out_now);
    let porch_glow = or(o.porch.fix_glow, T.fixture.glow);
    let fill_base = (if q { QK.fill_color } else { T.fill.color }, if q { QK.fill_intensity } else { T.fill.intensity });
    let mut fill_i = or(o.fill.intensity, fill_base.1);
    if o.fill.intensity.is_none() && plan.src.iter().any(|(k, s)| *k == "fill.offset" && *s == Src::Derived) {
        let k = (plan.lamps.fill.offset[1] - plan.wall.face) / KIT_PLAN.fill_out;
        if k < 0.999 { fill_i *= m::max(0.0, k) * m::max(0.0, k); }
    }
    let moon_scale = or(o.moon_scale, if q { QK.moon_scale } else { T.moon_scale });
    let sky_scale = or(o.sky_scale, if q { QK.sky_scale } else { T.sky_scale });
    st.sam_on = or(o.sam.on, if q { false } else { T.sam.on });
    st.sam_intensity = or(o.sam.intensity, T.sam.intensity);
    st.sam_inner = or(o.sam.inner_deg, T.sam.inner_deg);
    st.sam_outer = or(o.sam.outer_deg, T.sam.outer_deg);
    st.sam_volumetric = or(o.sam.volumetric, T.sam.volumetric);
    let room = st.way_room_lights.or(o.room_lights).unwrap_or(if plan.set.is_some() { 1.0 } else { T.room_lights });
    let mut kept: Vec<&str> = alloc::vec!["exterior", "player", "flashlights", "transient", "weather", "kitview"];
    kept.extend(plan.keep_lights.iter().map(|s| s.as_str()));
    let own = |g: &Game, l: LightId| { let r = &g.tab.light[l]; r.ttl < 0.0 && r.kind != 2.0 && !kept.iter().any(|k| *k == r.group) };

    g.fog_of_war.on = false;
    g.fog_picture.viewer = false;
    crate::fogofwar::fog_of_war_reset(g);
    let _ = park(g);
    let targets: Vec<FixtureId> = g.targets.clone();
    let lit0: Vec<FixtureId> = if q { targets.iter().copied().filter(|&t| home::fixture_on(g, t)).collect() } else { Vec::new() };
    let li = light_in();
    if room <= 0.0 {
        let list: Vec<LightId> = g.engine.lights.lights.clone();
        let mut groups: Vec<String> = Vec::new();
        for l in list { if own(g, l) { let n = &g.tab.light[l].group; if !groups.contains(n) { groups.push(n.clone()); } } }
        for grp in groups { crate::lighting::set_group(g, &grp, false); }
    }
    // (by day the way's own lamps are out too, each by its own switch: a restart puts the level's switches back as authored, so every staging)
    if !st.day_out.is_empty() {
        for &t in &targets { let r = &g.tab.sim_lightshome_fixture[t]; if r.kind == FixtureKind::Analytic && st.day_out.iter().any(|n| *n == r.name) { home::light_home_in(g, li.on, Some(t), Some(0.0), None, None, None); } }
    }
    // (the quiet picture's own lamps: one pool and no other light on the team's side; the room behind the door burns low)
    if q {
        let (mid, n) = (a.mid, a.n);
        let out_of = |p: V3| (p[0] - mid[0]) * n[0] + (p[2] - mid[1]) * n[1];
        let dist = |p: V3, q: V3| m::sqrt((p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]) + (p[2] - q[2]) * (p[2] - q[2]));
        let foot_m = plan.wall.face + KIT_PLAN.off_face;
        let foot: V3 = [mid[0] + n[0] * foot_m, 1.0, mid[1] + n[1] * foot_m];
        let wm = plan.wall.back - KIT_PLAN.lit_in;
        let wedge: V3 = [mid[0] + n[0] * wm, KIT_PLAN.lit_y, mid[1] + n[1] * wm];
        for &t in &targets {
            let r = &g.tab.sim_lightshome_fixture[t];
            let Some(l) = r.light else { continue };
            if r.kind != FixtureKind::Analytic || g.tab.light[l].kind == 2.0 { continue; }
            let (pos, range, name) = (r.pos, g.tab.light[l].range, r.name.clone());
            if out_of(pos) >= plan.wall.back {
                if home::fixture_on(g, t) && dist(pos, foot) <= range { home::light_home_in(g, li.on, Some(t), Some(0.0), None, None, None); st.out.push(name); }
            } else if lit0.contains(&t) && dist(pos, wedge) <= m::min(range, KIT_PLAN.lit_reach) {
let (intensity, emissive) = (home::fixture_base_intensity(g, t), g.tab.sim_lightshome_fixture[t].base_emissive);
                st.lit_in.push(LampAs { t, intensity, emissive });
            }
        }
        quiet_inside(g, st, QK.inside, 1.0);
        let roofs = quiet_roofs(g, &plan);
        let np = js::to_i32(box_flag().no_primary);
        let any = !roofs.is_empty();
        for i in roofs {
            let b = g.col.boxes[i];
            if g.engine.world.statics.get(i) != Some(&b) { continue; }
            let f = &mut g.tab.boxes[b].flags;
            *f = (js::to_i32(*f) & !np) as f64;
        }
        if any { box_changed(g); }
    }

    // THE MEN: the open card's man first, on the plan's places
    st.who = if st.want >= 1 && (st.want as usize) <= team.len() { st.want } else { 0 };
    let head = acts::actor(g, st).filter(|h| team.contains(h));
    // (a restage keeps the file's order, the men's places by team index: a restart makes new men. The open card's man goes to the head of it by the card switch's own walk, the fourth man round Sam: `live`. Only a first staging and the still picture, which has no walks, stand him first)
    let all0 = crate::scenario_run::team_of(g);
    let keep = st.keep.take();
    let kept_order = keep.is_some() && !st.still;
    let kept_men: Vec<GuardId> = keep.map_or(Vec::new(), |perm| perm.iter().filter_map(|&i| all0.get(i).copied()).filter(|x| team.contains(x)).collect());
    let file: Vec<GuardId> = kept_men.iter().copied().chain(team.iter().copied().filter(|x| !kept_men.contains(x))).collect();
    let order: Vec<GuardId> = match head { Some(h) if !kept_order => core::iter::once(h).chain(file.iter().copied().filter(|&x| x != h)).collect(), _ => file };
    st.doorway = alloc::vec![a.a, a.b];
    st.stand = plan.slots.iter().map(|p| [p[0], p[1]]).collect();
    st.hand_right = plan.hand == "right";
    st.top_y = if plan.wall.top > 0.0 { m::min(KIT_PLAN.porch_up, plan.wall.top) } else { 1.9 };
    let mut slots: Vec<V3> = plan.slots.iter().take(order.len() + 1).copied().collect();
    let places = if q { Some(quiet_places(g, &plan, order.len())) } else { None };
    let mate0 = places.as_ref().and_then(|x| x.mate).filter(|_| !order.is_empty() && Some(order[0]) != head);
    let sam_rest: Option<V2> = places.as_ref().and_then(|x| x.sam).map(|s| [s[0], s[1]]);
    if let Some(x) = places.as_ref() { if let Some(s) = x.sam {
        slots[0] = if head.is_some() { x.back.unwrap_or(s) } else { s };
        st.stand.push([s[0], s[1]]);
        if mate0.is_some() || (x.mate.is_some() && !order.is_empty()) { let m0 = x.mate.unwrap(); st.stand.push([m0[0], m0[1]]); }
    } }
    let pl = g.player;
    let sam_ch = g.pl[pl].ch;
    crate::game::game_teleport_player(g, [slots[0][0], 0.0, slots[0][1]]);
    g.ch[sam_ch].aim_pitch = 0.0;
    let mut chars: Vec<(CharId, f64)> = alloc::vec![(sam_ch, slots[0][2] * R)];
    for (i, &x) in order.iter().enumerate() {
        let p = if i == 0 { mate0.unwrap_or(slots[i + 1]) } else { slots[i + 1] };
        let c = g.gd[x].ch;
        crate::character::pos_replace(g, c, [p[0], 0.0, p[1]]);
        g.ch[c].aim_pitch = 0.0;
        crate::control::set_hold(g, x, true, "the kit picture");
        crate::control::set_pinned(g, x, true, "the kit picture");
        chars.push((c, p[2] * R));
    }
    let shut0 = door.map_or(0.0, |d| crate::doors::door_angle(g, d));
    let (rifle, pistol) = (g.pl[pl].rifle, g.pl[pl].pistol);
    g.fa[rifle].light_on = st.sam_on;
    g.fa[pistol].light_on = st.sam_on;
    // (the quiet picture: he is down, his gun put away, the leaf a hand ajar by the game's own hand on it; a way-in door found locked is simply unlocked)
    if q {
        g.pl[pl].crouch = true;
        g.pl[pl].holstered = true;
        g.ch[sam_ch].anim.crouch = 1.0;
        g.ch[sam_ch].anim.crouch_target = 1.0;
        if let Some(d) = door { if !crate::doors::door_off(g, d) {
            crate::doors::door_set_locked(g, d, false);
            let id = g.ch[sam_ch].id;
            crate::doors::doors_crack(g, d, [slots[0][0], 0.0, slots[0][1]], id, 1.0, None, Some(QK.ajar_deg * R), Some(QK.ajar_deg * R));
        } }
    }
    // ONE ZERO-LENGTH STEP, hands off the controls: nothing advances, the men are posed where and how they now stand. The yaws written before and after it.
    for &(c, y) in &chars { g.ch[c].body_yaw = y; g.ch[c].aim_yaw = y; }
    if let (Some(input), Some(cam)) = (g.host_input, g.host_cam) {
        hands_off(g, input);
        crate::game::game_update(g, 0.0, input, cam, crate::host_hud::View { client_width: 1600.0, client_height: 1000.0, width: 1600.0, height: 1000.0 });
    }
    for &(c, y) in &chars { g.ch[c].body_yaw = y; g.ch[c].aim_yaw = y; }

    // THE LIGHTS, after the step (the game writes each weapon light in it)
    st.order = order.clone();
    st.sam_aim = at_of(&plan, or(o.sam.aim, T.sam.aim));
    st.a.beat = None;
    torches(g, st);
    let list: Vec<LightId> = g.engine.lights.lights.clone();
    for &l in &list {
        if !own(g, l) { continue; }
        g.tab.light[l].intensity *= room;
    }
    if q {
        for k in 0..st.lit_in.len() {
            let t = st.lit_in[k].t;
            let Some(l) = g.tab.sim_lightshome_fixture[t].light else { continue };
            let (on, broken, base) = (home::fixture_on(g, t), home::fixture_broken(g, t), home::fixture_base_intensity(g, t));
            g.tab.light[l].enabled = on && !broken;
            g.tab.light[l].intensity = base;
            home::light_home_feed(g, l);
        }
    }
    {
        let moon = list.iter().copied().find(|&l| g.tab.light[l].name == "moon" && g.tab.light[l].kind == 2.0);
        st.moon_at = None;
        st.sky_at = None;
        if let Some(l) = moon {
            let t = if q { QK.moon_tint } else { [1.0, 1.0, 1.0] };
            let row = &mut g.tab.light[l];
            row.color = [row.color[0] * t[0], row.color[1] * t[1], row.color[2] * t[2]];
            if row.enabled && row.intensity > 0.0 {
                row.intensity *= moon_scale;
                st.moon_at = Some(LightAs { light: l, intensity: row.intensity });
                let (z, h) = (g.engine.settings.sky_zenith, g.engine.settings.sky_horizon);
                let sky = ([z[0] * sky_scale, z[1] * sky_scale, z[2] * sky_scale], [h[0] * sky_scale, h[1] * sky_scale, h[2] * sky_scale]);
                g.engine.settings.sky_zenith = sky.0;
                g.engine.settings.sky_horizon = sky.1;
                st.sky_at = Some(sky);
            }
        }
    }
    {
        let (p0, p1) = (at_of(&plan, offset_q), at_of(&plan, aim_q));
        st.porch = Some(hang(g, "kitview_porch", &Hung { kind: or(o.porch.kind, lamp0.kind), pos: p0, dir: unit(p0, p1), color: porch_color, intensity: porch_i, range: or(o.porch.range, lamp0.range), inner: or(o.porch.inner_deg, lamp0.inner_deg), outer: or(o.porch.outer_deg, lamp0.outer_deg), radius: or(o.porch.radius, lamp0.radius), volumetric: or(o.porch.volumetric, lamp0.volumetric), broad: or(o.porch.broad, lamp0.broad) }));
        let (f0, f1) = (at_of(&plan, plan.lamps.fill.offset), at_of(&plan, plan.lamps.fill.aim));
        let fl = &T.fill;
        st.fill = Some(hang(g, "kitview_fill", &Hung { kind: or(o.fill.kind, fl.kind), pos: f0, dir: unit(f0, f1), color: or(o.fill.color, fill_base.0), intensity: fill_i, range: or(o.fill.range, fl.range), inner: or(o.fill.inner_deg, fl.inner_deg), outer: or(o.fill.outer_deg, fl.outer_deg), radius: or(o.fill.radius, fl.radius), volumetric: or(o.fill.volumetric, fl.volumetric), broad: or(o.fill.broad, fl.broad) }));
        st.fill_at = fill_i;
    }
    // the porch lamp's glowing box on the wall (a restart cuts the world back to its authored boxes: put again)
    {
        if let (Some(fx), true) = (plan.lamps.porch.fixture.as_ref(), porch_i > 0.0) {
            let (c, half) = (porch_color, or(o.porch.fix_half, T.fixture.half));
            let mut p = MakeBoxP::zeroed();
            p.c = at_of(&plan, fx.at);
            p.h = half;
            p.yaw = Some(m::atan2(plan.s[1], plan.s[0]));
            p.albedo = Some([0.04, 0.04, 0.04]);
            p.emissive = Some([c[0] * porch_glow, c[1] * porch_glow, c[2] * porch_glow]);
            let f = box_flag();
            p.flags = Some((js::to_i32(f.emissive) | js::to_i32(f.area_lit)) as f64);
            let b = crate::scene_boxes::make_box(g, &p);
            crate::scene_boxes::box_world_add_static(g, b);
            st.fixture = Some(b);
        }
    }
    // (the quiet picture: the pool lamp is a lamp of the game's, so that a round puts it out as it puts out any fixture and a restart's repair mends it; and behind the door a small warm lamp of the picture's own)
    {
        if q {
            let c = porch_color;
            let porch = st.porch.unwrap();
            let mut d = FixtureDef::zeroed();
            d.name = String::from("kitview_porch"); d.group = String::from("kitview"); d.kind = FixtureDefKind::Analytic; d.pos = g.tab.light[porch].pos; d.light = Some(porch);
            d.base_emissive = [c[0] * porch_glow, c[1] * porch_glow, c[2] * porch_glow];
            d.base_intensity = porch_i;
            d.on = true; d.disabled_until = -1.0; d.factor = 1.0; d.interactive = true;
            let p = home::light_home_add(g, &d);
            home::light_home_in(g, li.authored, Some(p), Some(1.0), None, None, None);
            st.pool = Some(p);
            g.targets.push(p);
            crate::game::game_stand_lights(g);
            pool_sync(g, st);
        }
        if q && a.kind != "wall" {
            // (which way along the wall the leaf's free edge is; a gap: its middle)
            let lt = match door { Some(d) => { let h = g.dr[d].def.hinge; if (a.mid[0] - h[0]) * plan.s[0] + (a.mid[1] - h[1]) * plan.s[1] >= 0.0 { 1.0 } else { -1.0 } } None => 0.0 };
            let oo = &QK.own;
            let p = at_of(&plan, [lt * (a.width / 2.0 + oo.side), plan.wall.back - oo.back, oo.up]);
            st.inside = Some(hang(g, "kitview_inside", &Hung { kind: 0.0, pos: p, dir: [0.0, -1.0, 0.0], color: oo.color, intensity: oo.intensity, range: oo.range, inner: 0.0, outer: 180.0, radius: oo.radius, volumetric: 0.0, broad: false }));
        }
    }
    // THE CAMERA: the plan's pin, framed
    st.look = plan.cam.look;
    st.yaw = plan.cam.yaw;
    st.pitch = plan.cam.pitch;
    st.dist = plan.cam.dist;
    st.fit = if q { let lamp = match st.fixture { Some(f) => g.tab.boxes[f].c, None => g.tab.light[st.porch.unwrap()].pos }; Some((lamp, sam_rest.unwrap_or([slots[0][0], slots[0][1]]))) } else { None };
    st.foot = at_of(&plan, [0.0, 0.0, 0.0]);
    st.s = plan.s; st.n = a.n; st.mid = a.mid;
    st.sam = [slots[0][0], slots[0][1]];
    st.sam_yaw = slots[0][2] * R;
    st.spots = (0..order.len()).map(|i| [slots[i + 1][0], slots[i + 1][1]]).collect();
    st.yaws = (0..order.len()).map(|i| slots[i + 1][2] * R).collect();
    st.mate = match places.as_ref().and_then(|x| x.mate) { Some(p) if !order.is_empty() => Some(([p[0], p[1]], p[2] * R)), _ => None };
    aim_stage(g, st);
    {
        // (the way in as the acts read it. How far in a can or a round may go: the plan's free floor behind the anchor's line; a blank wall: what a hole opens onto)
        let wall = a.kind == "wall";
        let foot = st.foot;
        let side = match door { Some(d) => crate::doors::doors_side(g, d, [slots[0][0], 0.0, slots[0][1]]), None => 1.0 };
        let latch = match door { Some(d) => { let h = g.dr[d].def.hinge; if (foot[0] - h[0]) * plan.s[0] + (foot[2] - h[1]) * plan.s[1] >= 0.0 { 1.0 } else { -1.0 } } None => 1.0 };
        let reach_in = if wall { plan.beyond - plan.wall.back } else { plan.inside };
        let fits = |x: f64| x <= reach_in - 0.3;
        let spots = |g: &Game, list: &[f64]| -> Vec<V3> {
            let mut ms: Vec<f64> = list.iter().copied().filter(|&x| fits(x)).collect();
            if !list.iter().any(|&x| fits(x)) && reach_in >= 0.8 { ms.push(reach_in / 2.0); }
            let mut o = Vec::new();
            for x in ms { for k in K.in_along { let p = at_of(&plan, [k * latch, -x, 0.0]); if free(g, p[0], p[2]) { o.push(p); } } }
            o
        };
        st.spots_in = spots(g, &K.in_m);
        st.spots_smoke = spots(g, &K.smoke_m);
        let free_at = |x: f64, z: f64| free(g, x, z);
        st.act4 = super::plan::act4(a.mid, plan.s, a.n, a.width, &plan.slots, Some(&free_at), None);
        st.frag_m = m::max(0.6, m::min(K.sam_frag_m, reach_in - 0.5));
        st.frag_land = m::min(K.frag_land, m::max(0.3, st.frag_m - 0.5));
        st.door = door; st.walled = wall; st.width = a.width; st.reach_in = reach_in; st.hole = false; st.side = side; st.latch = latch;
        st.leaf_shut = shut0; st.leaf_rest = door.map_or(0.0, |d| crate::doors::door_angle(g, d));
        st.sam_smoke = at_of(&plan, [0.0, -K.sam_smoke_m, 0.0]);
        st.sam_stun = at_of(&plan, [0.0, -K.sam_stun_m, 0.0]);
        st.sam_frag = at_of(&plan, [0.0, -st.frag_m, 0.0]);
        st.sam_rest = sam_rest.unwrap_or([slots[0][0], slots[0][1]]);
        st.sam_back = places.as_ref().and_then(|x| x.back).map(|q| [q[0], q[1]]);
        st.four = false;
    }
    // LIVE: Sam cannot be killed and his belt refills, no autosave, no response team, the game's turn weight on the picture's numbers, every teammate unhurt, each sent to his place
    g.god_mode = true;
    g.infinite_ammo = true;
    g.saves.enabled = false;
    g.tune.qrf = false;
    let t = &mut g.k.turn;
    t.on = 1.0;
    t.aim = m::round(K.gun_rate * R * 1000.0) / 1000.0;
    t.aim_acc = m::round(K.gun_rate * R / K.ramp * 100.0) / 100.0;
    t.body = m::round(K.body_rate * R * 1000.0) / 1000.0;
    t.body_acc = m::round(K.body_rate * R / K.ramp * 100.0) / 100.0;
    for x in crate::scenario_run::team_of(g) { g.gd[x].god = true; }
    // (the posing step walked the parked men back inside the world's edge)
    let _ = park(g);
    if !st.still { acts::live(g, st); }
}

/// WHAT EVERY STAGING STARTS FROM: the mission as its start left it, and the stage as it then is. A staged game is never mended and never grows: it is thrown away whole.
pub fn keep_base(g: &mut Game) {
    let Some(mut st) = take(g) else { return };
    st.base = Some(Base(alloc::sync::Arc::new((Box::new(g.clone()), block_of(g.sim), (*st).clone()))));
    g.kit_stage = Some(st);
}
fn block_of(slot: usize) -> Vec<u8> { unsafe { core::slice::from_raw_parts(::sim::state::addr(slot) as *const u8, core::mem::size_of::<::sim::state::State>()).to_vec() } }

/// THE MISSION FROM ITS START and the team staged again, as the cards, the size and the open card now stand.
pub fn restage(g: &mut Game) {
    let Some(mut was) = take(g) else { return };
    let Some(base) = was.base.clone() else { was.due = false; g.kit_stage = Some(was); return };
    // (the file's order, as team indexes: every row id the stage holds is dead after this)
    let team = crate::scenario_run::team_of(g);
    let keep: Vec<usize> = was.order.iter().filter_map(|x| team.iter().position(|t| t == x)).collect();
    let mut n = base.0.0.clone();
    // (what the host holds of the game goes on: its count of serials, what the steps told it and the record of its hand; and what the cards and the size are)
    n.serial_next = n.serial_next.max(g.serial_next);
    n.out = core::mem::take(&mut g.out);
    n.cmd_buf = g.cmd_buf.take();
    // (the cards, the notes, the words on them; what is keyed by a man or a player is of the game thrown away, and a restart's new men would answer to it)
    let of_base = core::mem::replace(&mut n.m.kit, g.m.kit.clone());
    (n.m.kit.man_of, n.m.kit.bare_eyed, n.m.kit.kit_staged, n.m.kit.issued_player, n.m.kit.you_bare) = (of_base.man_of, of_base.bare_eyed, of_base.kit_staged, of_base.issued_player, of_base.you_bare);
    (n.k.nightbreach.squad, n.k.rescue.squad) = (g.k.nightbreach.squad, g.k.rescue.squad);
    crate::scene_collision::unstand(&mut n);
    ::sim::state::put(g.sim, &base.0.1);
    crate::scene_collision::stands(&mut n);
    *g = *n;
    let mut st = Box::new(base.0.2.clone());
    (st.base, st.want, st.field, st.size, st.frame, st.aspect) = (Some(base), was.want, was.field.clone(), was.size, was.frame, was.aspect);
    (st.roused_men, st.roused_alarms) = (was.roused_men, was.roused_alarms);
    (st.veil, st.still, st.shown, st.keep) = (was.veil, was.still, was.shown, Some(keep));
    acts::carry(&was.a, &mut st.a);
    crate::game::game_restart_encounter(g);
    place(g, &mut st);
    // (another picture than the last: nothing of it is blended into this one)
    g.restart_cut = true;
    g.kit_stage = Some(st);
}

/// the camera on the men, for the stage in hand: the plan's pin, moved and scaled to where the page's frame wants them. Both framings: the quiet one, for one subject, and the one hung on the door.
fn aim_stage(g: &mut Game, st: &mut KitStage) {
    let Some(cam) = g.host_cam else { return };
    if st.aspect > 0.0 { g.tab.camera[cam].aspect = st.aspect; }
    let (mut l, mut d) = (st.look, st.dist);
    let (yaw, pitch) = (st.yaw, st.pitch);
    let f = st.frame;
    let fov = g.tab.camera[cam].fov;
    let th = m::tan(fov / 2.0);
    let aspect = g.tab.camera[cam].aspect;
    let put = |g: &mut Game, l: V3, d: f64| {
        let c = &mut g.tab.camera[cam];
        c.target = l; c.yaw = yaw; c.pitch = pitch; c.distance = d; c.desired_distance = d;
        crate::camera::follow_camera_rebuild(g, cam);
    };
    let pr = |g: &Game, p: V3| { let (x, y, _) = crate::camera::follow_camera_project(g, cam, p, 2.0, 2.0, None, None); (x, y) };
    if let (true, Some((lamp, sam))) = (st.quiet, st.fit) {
        let qf = &QK.frame;
        let column = f.and_then(|x| x.column);
        let hung = f.and_then(|x| x.hung);
        let (top, bot) = column.map_or((1.0, -1.0), |c| (c[0], c[1]));
        let (lf, rt) = hung.map_or((-1.0, 1.0), |h| (h[1], h[2]));
        let y_at = |t: f64| top + (bot - top) * t;
        let x_at = lf + (rt - lf) * (if st.hand_right { 1.0 - qf.spine } else { qf.spine });
        l = [sam[0], 1.0, sam[1]];
        for _ in 0..12 {
            put(g, l, d);
            let (pl, pb, ps) = (pr(g, lamp), pr(g, [sam[0], 0.0, sam[1]]), pr(g, [sam[0], 0.5, sam[1]]));
            let b = crate::camera::follow_camera_planar_basis(g, cam);
            let k = (pb.1 - pl.1) / m::max(0.05, y_at(qf.lamp) - y_at(qf.boots));
            let (ey, ex) = (1.0 - pb.1 - y_at(qf.boots), ps.0 - 1.0 - x_at);
            let (mx, mz) = (ex * d * th * aspect, ey * d * th / m::max(0.2, m::sin(pitch)));
            l = [l[0] + b.right[0] * mx + b.fwd[0] * mz, l[1], l[2] + b.right[2] * mx + b.fwd[2] * mz];
            d = m::min(60.0, m::max(4.0, d * m::min(2.0, m::max(0.5, k))));
        }
    } else if let (Some(fa), false) = (f, st.stand.is_empty()) {
        let (rb, rx) = (ST.body, ST.reach);
        let hung = fa.hung.filter(|_| st.doorway.len() == 2);
        let dm: V3 = if hung.is_some() { [(st.doorway[0][0] + st.doorway[1][0]) / 2.0, 1.0, (st.doorway[0][1] + st.doorway[1][1]) / 2.0] } else { [0.0; 3] };
        for _ in 0..(if hung.is_some() { 12 } else { 6 }) {
            put(g, l, d);
            let (mut x0, mut x1, mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
            for q in &st.stand { for y in [0.0, 1.9] { for (ox, oz) in [(-rb, 0.0), (rb, 0.0), (0.0, -rb), (0.0, rb)] {
                let p = pr(g, [q[0] + ox, y, q[1] + oz]);
                x0 = m::min(x0, p.0 - 1.0); x1 = m::max(x1, p.0 - 1.0); y0 = m::min(y0, 1.0 - p.1); y1 = m::max(y1, 1.0 - p.1);
            } } }
            for j in &st.doorway { for y in [0.0, st.top_y] {
                let p = pr(g, [j[0], y, j[1]]);
                x0 = m::min(x0, p.0 - 1.0); x1 = m::max(x1, p.0 - 1.0); y0 = m::min(y0, 1.0 - p.1); y1 = m::max(y1, 1.0 - p.1);
            } }
            let (a0, a1) = (pr(g, [l[0], 0.0, l[2]]), pr(g, [l[0], 1.9, l[2]]));
            let tall = m::abs(a1.1 - a0.1) / 2.0;
            let (h, w, ey) = ((y1 - y0) / 2.0, (x1 - x0) / 2.0, (y0 + y1) / 2.0 - fa.cy);
            let b = crate::camera::follow_camera_planar_basis(g, cam);
            let mut ex = (x0 + x1) / 2.0 - fa.cx;
            let mut k = m::max(m::max(h / m::max(0.05, fa.h), w / m::max(0.05, fa.w)), tall / fa.man);
            if let Some([door, left, right, keep]) = hung {
                // (the door's line at `door`, the places' reach inside `left` to `right`; the men no bigger than the fit above and no smaller than `keep` of it; then the places are held at `left`)
                let (mut r0, mut r1) = (f64::INFINITY, f64::NEG_INFINITY);
                for q in &st.stand { for y in [0.0, 1.9] { for (ox, oz) in [(-rx, 0.0), (rx, 0.0), (0.0, -rx), (0.0, rx)] {
                    let p = pr(g, [q[0] + ox, y, q[1] + oz]);
                    r0 = m::min(r0, p.0 - 1.0); r1 = m::max(r1, p.0 - 1.0);
                } } }
                let dx = pr(g, dm).0 - 1.0;
                let room = m::max(0.02, right - left);
                let fd = if st.hand_right { left + right - door } else { door };
                let for_door = m::max((dx - r0) / m::max(0.02, fd - left), (r1 - dx) / m::max(0.02, right - fd));
                k = m::max(m::max(k, (r1 - r0) / room), m::min(for_door, k / m::min(1.0, m::max(0.5, keep))));
                let kk = m::min(2.0, m::max(0.5, k));
                ex = dx - (if st.hand_right { m::min(fd, right - (r1 - dx) / kk) } else { m::max(fd, left + (dx - r0) / kk) });
            }
            let (mx, mz) = (ex * d * th * aspect, ey * d * th / m::max(0.2, m::sin(pitch)));
            l = [l[0] + b.right[0] * mx + b.fwd[0] * mz, l[1], l[2] + b.right[2] * mx + b.fwd[2] * mz];
            d = m::min(60.0, m::max(4.0, d * m::min(2.0, m::max(0.5, k))));
        }
    }
    st.pinned = Some((l, yaw, pitch, d));
    write_pin(g, st);
}
/// the worked-out pin on the host's camera: the page's own half of a tick may have handed the row over
fn write_pin(g: &mut Game, st: &KitStage) {
    let (Some(cam), Some((look, yaw, pitch, dist))) = (g.host_cam, st.pinned) else { return };
    crate::camera::follow_camera_set_pin(g, cam, Some(crate::camera::FollowCameraSetPinP { look, yaw, pitch, dist }));
}
/// THE CAMERA ON THE MEN, into the host camera's pin.
pub fn aim(g: &mut Game) {
    let Some(mut st) = take(g) else { return };
    if !st.none { aim_stage(g, &mut st); }
    g.kit_stage = Some(st);
}

/// the cursor on a point of the world, as the view a tick is handed sees it (None: behind the lens)
pub(super) fn cursor_of(cmd: &PlayerCmd, p: V3) -> Option<(f64, f64)> {
    let vp = &cmd.view.vp;
    if vp.len() != 16 { return None; }
    let cw = vp[3] * p[0] + vp[7] * p[1] + vp[11] * p[2] + vp[15];
    if cw <= 0.0 { return None; }
    Some((((vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12]) / cw * 0.5 + 0.5) * cmd.w, (1.0 - ((vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13]) / cw * 0.5 + 0.5)) * cmd.h))
}
/// the mission the stage has up has ended
pub(super) fn staged_ended(g: &Game, asks_none: bool) -> bool { staged_way_of(g, asks_none).ended }

/// ROUND THE TICK, BEFORE IT: the park; the seat's hand says nothing of its own: the keys and the cursor are the director's.
pub fn before(g: &mut Game, cmd: &mut PlayerCmd) {
    let Some(mut st) = take(g) else { return };
    cmd.keys.clear();
    cmd.pressed.clear();
    cmd.buttons = 0.0;
    cmd.clicked = 0.0;
    cmd.wheel = 0.0;
    // (the page has laid its copy over the picture, or has been given half a second to: the room staged again, now)
    if let Some(t0) = st.veil_asked { if st.veil_laid || g.time - t0 >= 0.5 { st.due = true; } }
    let pl = g.player;
    if st.still && !st.none && (g.pl[pl].down || st.order.iter().any(|&x| g.gd[x].state == GuardState::Dead) || staged_way_of(g, st.asks_none).ended) { st.due = true; }
    if st.due { g.kit_stage = Some(st); restage(g); st = match take(g) { Some(s) => s, None => return }; }
    if st.none { g.kit_stage = Some(st); return; }
    let (men, alarms) = park(g);
    st.roused_men += men;
    st.roused_alarms += alarms;
    if let Some((x, y)) = cursor_of(cmd, st.foot) { cmd.mx = x; cmd.my = y; }
    st.a.cursor = None;
    if st.still { home_men(g, &mut st); }
    else {
        acts::before_step(g, &mut st, cmd);
        // (Sam's own keys, for this tick; the cursor where the director asked for it)
        for k in st.a.press.drain(..) { cmd.pressed.push(String::from(k)); }
        cmd.clicked = st.a.clicked;
        st.a.clicked = 0.0;
        if let Some(q) = st.a.cursor { if let Some((x, y)) = cursor_of(cmd, q) { cmd.mx = x; cmd.my = y; } }
    }
    pool_sync(g, &st);
    g.kit_stage = Some(st);
}

/// ROUND THE TICK, AFTER IT: what the step made of anything is not carried into the next, and what the game writes each step goes back as staged.
pub fn after(g: &mut Game) {
    let Some(mut st) = take(g) else { return };
    if st.none { g.kit_stage = Some(st); return; }
    if !st.still { acts::after_step(g, &mut st); }
    torches(g, &mut st);
    let (men, alarms) = park(g);
    st.roused_men += men;
    st.roused_alarms += alarms;
    // (the dark after the lamp: the fill and the moon lifted, eyes adjusting; the half crouch he shoots from, over the game's own knee)
    let e = if st.a.lift_at.is_nan() { 0.0 } else { let u = m::max(0.0, m::min(1.0, (g.time - st.a.lift_at) / QK.lift_secs)); u * u * (3.0 - 2.0 * u) };
    if let Some(l) = st.fill { if st.fill_at > 0.0 { let r = &mut g.tab.light[l]; r.intensity = st.fill_at * (1.0 + (QK.lift_fill - 1.0) * e); r.peak_intensity = r.intensity; } }
    if let Some(a) = st.moon_at { g.tab.light[a.light].intensity = a.intensity * (1.0 + (QK.lift - 1.0) * e); }
    if st.quiet {
        let up = st.a.beat.as_ref().map_or(false, |b| b.row == Act::QuietGun) && st.a.rise > 0.0;
        if up { let c = g.pl[g.player].ch; let a = &mut g.ch[c].anim; a.crouch = m::min(a.crouch, 1.0 - QK.half * st.a.rise); } else { st.a.rise = 0.0; }
    }
    if let Some((z, h)) = st.sky_at { g.engine.settings.sky_zenith = z; g.engine.settings.sky_horizon = h; }
    // (the team stands under a roof: no weather on the men, whatever the level's own roof test makes of a ceiling the camera never draws)
    if st.plan.as_ref().map_or(false, |p| p.indoors) {
        let pl = g.player;
        let mut chars: Vec<CharId> = st.order.iter().map(|&x| g.gd[x].ch).collect();
        chars.push(g.pl[pl].ch);
        for c in chars { let r = &mut g.ch[c]; r.wet = 0.0; r.beads = 0.0; r.wet_rain = 0.0; }
    }
    let pl = g.player;
    g.pl[pl].hit_flash = 0.0;
    let now = g.time;
    if g.pl[pl].dazzled_until > now { g.pl[pl].dazzled_until = -1.0; }
    if g.pl[pl].stagger_until > now { g.pl[pl].stagger_until = -1.0; }
    if let Some(c) = g.host_cam { g.tab.camera[c].kick = None; }
    let seat = crate::game::game_host_seat(g);
    let seat_cam = g.tab.cmd_seat[seat].cam;
    g.tab.camera[seat_cam].kick = None;
    write_pin(g, &st);
    // (a flash, a blast: no brighter than the porch lamp's own, so nothing whites the picture out. The lights' own update writes the envelope from its peak)
    let list: Vec<LightId> = g.engine.lights.lights.clone();
    for &l in &list {
        let r = &mut g.tab.light[l];
        if r.ttl >= 0.0 {
            if r.intensity > K.flash_max { r.intensity = K.flash_max; }
            if r.peak_intensity > K.flash_max { r.peak_intensity = K.flash_max; }
        }
    }
    stroke(g, &list);
    // (the wand's row: the fog is the game's for its length and never the picture's: no lamp's face as the team last saw it, and the frame's boxes as they stood before the fog cut them)
    if st.a.fog_up && crate::fogofwar::fog_of_war_active(g) {
        crate::fogofwar::fog_of_war_faces_lift(g);
        if g.fog_picture.raw_at == g.fog_of_war.frame_no { g.engine.world.dynamics = g.fog_picture.raw_list.clone(); }
    }
    g.kit_stage = Some(st);
}

/// LIGHTNING TAMED: the weather's stroke is a directional light with no lifetime, flagged transient: cut to a share of the night's own directional light, and the sky's flash by as much.
fn stroke(g: &mut Game, list: &[LightId]) {
    let mut cap = -1.0;
    for &l in list {
        let r = &g.tab.light[l];
        if !(r.transient == Some(true) && r.ttl < 0.0 && r.enabled) { continue; }
        let was = r.intensity;
        if cap < 0.0 {
            let mut night: f64 = 0.0;
            for &n in list { let x = &g.tab.light[n]; if x.kind == 2.0 && x.enabled && x.transient != Some(true) && x.ttl < 0.0 { night = m::max(night, x.intensity); } }
            cap = if night > 0.0 { night * K.stroke_k } else { K.stroke_min };
        }
        if was > cap {
            let k = cap / was;
            g.tab.light[l].intensity = cap;
            if let Some(w) = g.engine.weather.as_mut() { w.flash = [w.flash[0] * k, w.flash[1] * k, w.flash[2] * k]; }
        }
    }
}

/// a frame message's numbers, NaN where one was not said: what is out of its range falls back, a hung door or a column out of order is not asked for
pub fn frame_ask(cx: f64, cy: f64, h: f64, w: f64, man: f64, hung: [f64; 4], column: [f64; 2]) -> Option<FrameAsk> {
    if !(cx.is_finite() && cy.is_finite() && h > 0.02 && h <= 1.0) { return None; }
    let ([door, left, right, keep], [top, bottom]) = (hung, column);
    let share = |x: f64| if x > 0.02 && x <= 1.0 { x } else { 1.0 };
    let edge = |x: f64| m::max(-1.0, m::min(1.0, x));
    let hung = if left >= -1.0 && right <= 1.0 && left < door && door < right { Some([door, left, right, if keep > 0.0 && keep <= 1.0 { keep } else { 1.0 }]) } else { None };
    let column = if top <= 1.5 && bottom >= -1.5 && top - bottom > 0.2 { Some([top, bottom]) } else { None };
    Some(FrameAsk { cx: edge(cx), cy: edge(cy), h, w: share(w), man: share(man), hung, column })
}
/// THE COMMANDS. Where the page wants the men in its view: `ask` none leaves it as it was.
pub fn frame(g: &mut Game, aspect: f64, ask: Option<FrameAsk>) {
    let Some(st) = g.kit_stage.as_mut() else { return };
    if aspect > 0.0 && aspect.is_finite() { st.aspect = aspect; }
    if ask.is_some() { st.frame = ask; }
    aim(g);
}
/// whose card is open (0 the player) and the row under its highlight: another card's man walks to the head of the file, another tile plays at once. The card may be asked for before its man has come
pub fn open(g: &mut Game, who: u32, field: &str) {
    let Some(mut st) = take(g) else { return };
    if st.still {
        st.field = String::from(field);
        if st.want != who { st.want = who; st.due = true; }
    } else { super::cards::opened(g, &mut st, who, field); }
    g.kit_stage = Some(st);
}
/// the cards are written (the caller's) and how many come (under 0: as it stands; a quiet or a solo mission's men are the mission's own). What moved is written live; what cannot be asks for the page's copy, and is staged again under it
pub fn cards(g: &mut Game, size: i32) {
    let Some(mut st) = take(g) else { return };
    let size = if (0..=4).contains(&size) && !staged_way_of(g, st.asks_none).fixed { Some(size as u32) } else { None };
    if let Some(n) = size { if g.rescue.active { g.k.rescue.squad = n as f64; } else { g.k.nightbreach.squad = n as f64; } }
    if size.is_some() { st.size = size; }
    if st.still { st.due = true; } else { super::cards::told(g, &mut st, size); }
    g.kit_stage = Some(st);
}
/// THE PAGE HAS LAID ITS COPY over the picture: what it was asked for is staged again in the next tick
pub fn under(g: &mut Game) {
    if let Some(st) = g.kit_stage.as_mut() { if st.veil_asked.is_some() { st.veil_laid = true; } }
}
/// THE PAGE HAS SHOWN ITS FIRST PICTURE: from now the acts may begin
pub fn shown(g: &mut Game) {
    let now = g.time;
    let Some(st) = g.kit_stage.as_mut() else { return };
    // (the first row's rest is counted from the picture the eye has)
    if !st.shown { st.a.rest_until = m::max(st.a.rest_until, now + K.rest_stage); }
    st.shown = true;
}
