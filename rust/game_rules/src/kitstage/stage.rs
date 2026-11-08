//! THE KIT PICTURE'S STAGE: the Team & kit screen's picture of the team at the mission's own way in. The mission is started as Deploy would start it; then this stands the team on the plan's places, parks
//! everybody else far off and calm, hangs the lamps and pins the camera. NO RULE OF THE GAME READS THE STAGE: it writes only what any host could write (men, lights, boxes, a pin) round the tick.
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

const R: f64 = m::PI / 180.0;
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
        let r = &mut g.gd[x];
        r.hold = true;
        r.pinned = true;
        r.heard_up_to = DEAF;
        r.awareness = 0.0;
        r.last_known = None;
        if r.state != GuardState::Dead {
            if r.state != GuardState::Patrol { men += 1; }
            r.state = GuardState::Patrol;
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
    g.engine.world.statics_dirty = true;
    g.engine.world.geom_serial += 1.0;
    crate::scene_collision::wrote(g);
}
fn drop_props(g: &mut Game, hidden: &[usize]) {
    for &i in hidden { if let Some(&p) = g.props.all.get(i) { crate::props::prop_system_remove(g, p); } }
}

/// the room behind the door at this share of itself: each lamp's own base, which the game multiplies each step, and its fixture's glow
fn quiet_inside(g: &mut Game, st: &mut KitStage, k: f64) {
    st.share = k;
    let li = light_in();
    for i in 0..st.lit_in.len() {
        let t = st.lit_in[i];
        let Some(a) = st.lamp_as.iter().find(|a| a.t == t).copied() else { continue };
        home::light_home_in(g, li.on, Some(t), Some(1.0), None, None, None);
        home::light_home_in(g, li.base, Some(t), Some(a.intensity * k), None, None, None);
        g.tab.sim_lightshome_fixture[t].base_emissive = [a.emissive[0] * k, a.emissive[1] * k, a.emissive[2] * k];
    }
    if let Some(o) = st.inside { let l = &mut g.tab.light[o]; l.intensity = QK.own.intensity; l.peak_intensity = QK.own.intensity; }
}
/// …and as the mission had them, before a restart: a base is the lamp's own, and no restart puts it back
fn quiet_undo(g: &mut Game, st: &mut KitStage) {
    let li = light_in();
    for t in core::mem::take(&mut st.lit_in) {
        let Some(a) = st.lamp_as.iter().find(|a| a.t == t).copied() else { continue };
        home::light_home_in(g, li.base, Some(t), Some(a.intensity), None, None, None);
        g.tab.sim_lightshome_fixture[t].base_emissive = a.emissive;
    }
    if let Some(p) = st.pool { g.tab.sim_lightshome_fixture[p].boxes = Vec::new(); }
    let roofs = core::mem::take(&mut st.roofs);
    if !roofs.is_empty() {
        let np = js::to_i32(box_flag().no_primary);
        for b in roofs { let f = &mut g.tab.boxes[b].flags; *f = (js::to_i32(*f) | np) as f64; }
        box_changed(g);
    }
}
fn pool_sync(g: &mut Game, st: &KitStage) {
    let Some(p) = st.pool else { return };
    let i = st.fixture.and_then(|f| g.engine.world.statics.iter().position(|&b| b == f));
    let row = &mut g.tab.sim_lightshome_fixture[p];
    row.boxes = i.map_or(Vec::new(), |i| alloc::vec![i as f64]);
    if st.porch.is_some() { row.light = st.porch; }
}
/// the lamps off the list: the game counts steady lights as it stands each man up, and a level near the light buffer's top refuses a man with them on it
fn unhang(g: &mut Game, st: &mut KitStage) {
    for l in [st.porch, st.fill, st.inside].into_iter().flatten() { crate::render_lights::light_set_remove(g, l); }
    quiet_undo(g, st);
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
fn hang(g: &mut Game, name: &str, h: &Hung, had: Option<LightId>) -> LightId {
    let on_list = had.filter(|l| g.engine.lights.lights.contains(l));
    let id = match on_list {
        Some(l) => l,
        None => {
            let mut l = crate::render_lights::RtLight::zeroed();
            l.name = String::from(name); l.group = String::from("kitview"); l.fixture_box = -1.0; l.ttl = -1.0;
            let id = g.tab.light.push(l);
            crate::render_lights::light_set_add(g, id)
        }
    };
    let l = &mut g.tab.light[id];
    l.kind = h.kind; l.pos = h.pos; l.dir = h.dir; l.color = h.color; l.intensity = h.intensity; l.peak_intensity = h.intensity; l.range = h.range; l.inner_deg = h.inner; l.outer_deg = h.outer; l.radius = h.radius; l.volumetric = h.volumetric; l.broad = h.broad; l.enabled = h.intensity > 0.0;
    id
}

/// the team's lights off, Sam's dim and tight at the door's foot (the game writes each weapon light in each step)
fn torches(g: &mut Game, st: &KitStage) {
    for &x in &st.order { let l = g.gd[x].flashlight; g.tab.light[l].enabled = false; }
    let pl = g.player;
    let id = g.pl[pl].light;
    let pos = g.tab.light[id].pos;
    let l = &mut g.tab.light[id];
    if !st.sam_on { l.enabled = false; return; }
    l.dir = unit(pos, st.sam_aim);
    l.intensity = st.sam_intensity; l.peak_intensity = st.sam_intensity; l.inner_deg = st.sam_inner; l.outer_deg = st.sam_outer; l.volumetric = st.sam_volumetric;
}

/// the man at his place, turned his way, as the squad's own hand-over has it: it takes him off whatever had him. A staged man's `hold` is only "nothing moves until the first order", and with no order of his own he drifts after Sam.
fn send(g: &mut Game, x: GuardId, p: V2, face: f64, low: bool) {
    let mut a = crate::orders::SquadOrdersAssignA::zeroed();
    a.order = String::from("cover");
    a.post = Some([p[0], 0.0, p[1]]);
    a.face = Some(face);
    a.strafe = Some(true);
    a.stance = Some(crate::character::Stance::LowReady);
    a.upper = Some(crate::character::UpperMode::None);
    a.pace = Some(K.pace);
    a.crouch = Some(low);
    crate::orders::squad_orders_assign(g, x, &a);
}
/// every man on his place: sent once, and again whenever he is found with no such order (a fresh man has the scenario's own until his first)
fn home_men(g: &mut Game, st: &mut KitStage) {
    for k in 0..st.order.len() {
        let x = st.order[k];
        let actor = open_man(g, st) == Some(x);
        let (p, yaw, low) = match st.mate { Some((p, y)) if k == 0 && !actor => (p, y, st.quiet), _ => (st.spots[k], st.yaws[k], false) };
        let mine = crate::orders::squad_orders_of(g, &crate::seats::here(g), x).map_or(false, |a| g.tab.orders_assignment[a].order == "cover");
        if mine && st.sent.contains(&x) { continue; }
        send(g, x, p, yaw, low);
        if !st.sent.contains(&x) { st.sent.push(x); }
    }
}
/// the man of the open card (member i is the team's [i - 1])
fn open_man(g: &Game, st: &KitStage) -> Option<GuardId> { if st.who > 0 { crate::scenario_run::team_of(g).get(st.who as usize - 1).copied() } else { None } }

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
            if Some(t) == st.pool || r.kind != FixtureKind::Analytic || g.tab.light[l].kind == 2.0 { continue; }
            let (pos, range, name) = (r.pos, g.tab.light[l].range, r.name.clone());
            if out_of(pos) >= plan.wall.back {
                if home::fixture_on(g, t) && dist(pos, foot) <= range { home::light_home_in(g, li.on, Some(t), Some(0.0), None, None, None); st.out.push(name); }
            } else if lit0.contains(&t) && dist(pos, wedge) <= m::min(range, KIT_PLAN.lit_reach) {
                if !st.lamp_as.iter().any(|x| x.t == t) { let (i, e) = (home::fixture_base_intensity(g, t), g.tab.sim_lightshome_fixture[t].base_emissive); st.lamp_as.push(LampAs { t, intensity: i, emissive: e }); }
                st.lit_in.push(t);
            }
        }
        quiet_inside(g, st, QK.inside);
        // (the slab's flag is the box's own and no restart writes it: put back with the dimmed lamps)
        let roofs = quiet_roofs(g, &plan);
        let np = js::to_i32(box_flag().no_primary);
        for i in roofs {
            let b = g.col.boxes[i];
            if g.engine.world.statics.get(i) != Some(&b) { continue; }
            let f = &mut g.tab.boxes[b].flags;
            *f = (js::to_i32(*f) & !np) as f64;
            st.roofs.push(b);
        }
        if !st.roofs.is_empty() { box_changed(g); }
    }

    // THE MEN: the open card's man first, on the plan's places
    let head = open_man(g, st).filter(|h| team.contains(h));
    let order: Vec<GuardId> = match head { Some(h) => core::iter::once(h).chain(team.iter().copied().filter(|&x| x != h)).collect(), None => team.clone() };
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
    torches(g, st);
    let list: Vec<LightId> = g.engine.lights.lights.clone();
    for &l in &list {
        if !own(g, l) { continue; }
        let base = match st.dimmed.iter().find(|x| x.light == l) { Some(x) => x.intensity, None => { let i = g.tab.light[l].intensity; st.dimmed.push(LightAs { light: l, intensity: i }); i } };
        g.tab.light[l].intensity = base * room;
    }
    if q {
        for k in 0..st.lit_in.len() {
            let t = st.lit_in[k];
            let Some(l) = g.tab.sim_lightshome_fixture[t].light else { continue };
            let (on, broken, base) = (home::fixture_on(g, t), home::fixture_broken(g, t), home::fixture_base_intensity(g, t));
            g.tab.light[l].enabled = on && !broken;
            g.tab.light[l].intensity = base;
            home::light_home_feed(g, l);
        }
    }
    {
        let moon = list.iter().copied().find(|&l| g.tab.light[l].name == "moon" && g.tab.light[l].kind == 2.0);
        if let Some(l) = moon { if st.moon.is_none() { st.moon = Some(LightAs { light: l, intensity: g.tab.light[l].intensity }); } }
        if st.sky.is_none() { st.sky = Some((g.engine.settings.sky_zenith, g.engine.settings.sky_horizon)); }
        st.moon_at = None;
        st.sky_at = None;
        if let Some(l) = moon {
            let was = *st.moon_color.get_or_insert(g.tab.light[l].color);
            let t = if q { QK.moon_tint } else { [1.0, 1.0, 1.0] };
            g.tab.light[l].color = [was[0] * t[0], was[1] * t[1], was[2] * t[2]];
            let base = st.moon.map_or(0.0, |x| x.intensity);
            if g.tab.light[l].enabled && base > 0.0 {
                let at = base * moon_scale;
                g.tab.light[l].intensity = at;
                st.moon_at = Some(at);
                let (z, h) = st.sky.unwrap();
                let sky = ([z[0] * sky_scale, z[1] * sky_scale, z[2] * sky_scale], [h[0] * sky_scale, h[1] * sky_scale, h[2] * sky_scale]);
                g.engine.settings.sky_zenith = sky.0;
                g.engine.settings.sky_horizon = sky.1;
                st.sky_at = Some(sky);
            }
        }
    }
    {
        let (p0, p1) = (at_of(&plan, offset_q), at_of(&plan, aim_q));
        st.porch = Some(hang(g, "kitview_porch", &Hung { kind: or(o.porch.kind, lamp0.kind), pos: p0, dir: unit(p0, p1), color: porch_color, intensity: porch_i, range: or(o.porch.range, lamp0.range), inner: or(o.porch.inner_deg, lamp0.inner_deg), outer: or(o.porch.outer_deg, lamp0.outer_deg), radius: or(o.porch.radius, lamp0.radius), volumetric: or(o.porch.volumetric, lamp0.volumetric), broad: or(o.porch.broad, lamp0.broad) }, st.porch));
        let (f0, f1) = (at_of(&plan, plan.lamps.fill.offset), at_of(&plan, plan.lamps.fill.aim));
        let fl = &T.fill;
        st.fill = Some(hang(g, "kitview_fill", &Hung { kind: or(o.fill.kind, fl.kind), pos: f0, dir: unit(f0, f1), color: or(o.fill.color, fill_base.0), intensity: fill_i, range: or(o.fill.range, fl.range), inner: or(o.fill.inner_deg, fl.inner_deg), outer: or(o.fill.outer_deg, fl.outer_deg), radius: or(o.fill.radius, fl.radius), volumetric: or(o.fill.volumetric, fl.volumetric), broad: or(o.fill.broad, fl.broad) }, st.fill));
        st.fill_at = fill_i;
    }
    // the porch lamp's glowing box on the wall (a restart cuts the world back to its authored boxes: put again)
    {
        if let Some(f) = st.fixture.take() { if let Some(i) = g.engine.world.statics.iter().position(|&b| b == f) { g.engine.world.statics.remove(i); g.engine.world.statics_dirty = true; } }
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
        let had = st.pool;
        let i = had.and_then(|p| g.targets.iter().position(|&t| t == p));
        if q {
            let c = porch_color;
            let emissive = [c[0] * porch_glow, c[1] * porch_glow, c[2] * porch_glow];
            let porch = st.porch.unwrap();
            let pos = g.tab.light[porch].pos;
            let p2 = match st.pool_kept {
                None => {
                    let mut d = FixtureDef::zeroed();
                    d.name = String::from("kitview_porch"); d.group = String::from("kitview"); d.kind = FixtureDefKind::Analytic; d.pos = pos; d.light = Some(porch); d.base_emissive = emissive; d.base_intensity = porch_i;
                    d.on = true; d.disabled_until = -1.0; d.factor = 1.0; d.interactive = true;
                    home::light_home_add(g, &d)
                }
                Some(p2) => {
                    let r = &mut g.tab.sim_lightshome_fixture[p2];
                    r.pos = pos; r.light = Some(porch); r.base_emissive = emissive;
                    home::light_home_in(g, li.base, Some(p2), Some(porch_i), None, None, None);
                    home::light_home_in(g, li.on, Some(p2), Some(1.0), None, None, None);
                    if i.is_none() { home::light_home_in(g, li.mend, Some(p2), None, None, None, None); home::light_home_in(g, li.blind, Some(p2), Some(-1.0), None, None, None); }
                    p2
                }
            };
            st.pool_kept = Some(p2);
            home::light_home_in(g, li.authored, Some(p2), Some(1.0), None, None, None);
            st.pool = Some(p2);
            if i.is_none() { g.targets.push(p2); }
            crate::game::game_stand_lights(g);
            pool_sync(g, st);
        } else {
            if let Some(i) = i { g.targets.remove(i); }
            st.pool = None;
        }
        let want_own = q && a.kind != "wall";
        let hung = st.inside.filter(|l| g.engine.lights.lights.contains(l));
        if !want_own {
            if let Some(l) = hung { crate::render_lights::light_set_remove(g, l); }
            st.inside = None;
        } else {
            // (which way along the wall the leaf's free edge is; a gap: its middle)
            let lt = match door { Some(d) => { let h = g.dr[d].def.hinge; if (a.mid[0] - h[0]) * plan.s[0] + (a.mid[1] - h[1]) * plan.s[1] >= 0.0 { 1.0 } else { -1.0 } } None => 0.0 };
            let oo = &QK.own;
            let p = at_of(&plan, [lt * (a.width / 2.0 + oo.side), plan.wall.back - oo.back, oo.up]);
            st.inside = Some(hang(g, "kitview_inside", &Hung { kind: 0.0, pos: p, dir: [0.0, -1.0, 0.0], color: oo.color, intensity: oo.intensity, range: oo.range, inner: 0.0, outer: 180.0, radius: oo.radius, volumetric: 0.0, broad: false }, hung));
        }
    }
    // THE CAMERA: the plan's pin, framed
    st.look = plan.cam.look;
    st.yaw = plan.cam.yaw;
    st.pitch = plan.cam.pitch;
    st.dist = plan.cam.dist;
    st.fit = if q { let lamp = match st.fixture { Some(f) => g.tab.boxes[f].c, None => g.tab.light[st.porch.unwrap()].pos }; Some((lamp, sam_rest.unwrap_or([slots[0][0], slots[0][1]]))) } else { None };
    st.foot = at_of(&plan, [0.0, 0.0, 0.0]);
    st.sam = [slots[0][0], slots[0][1]];
    st.sam_yaw = slots[0][2] * R;
    st.spots = (0..order.len()).map(|i| [slots[i + 1][0], slots[i + 1][1]]).collect();
    st.yaws = (0..order.len()).map(|i| slots[i + 1][2] * R).collect();
    st.mate = match places.as_ref().and_then(|x| x.mate) { Some(p) if !order.is_empty() => Some(([p[0], p[1]], p[2] * R)), _ => None };
    aim_stage(g, st);
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
}

/// THE ENCOUNTER RESTARTED and the team staged again, as the table and the size now stand.
pub fn restage(g: &mut Game) {
    let Some(mut st) = take(g) else { return };
    unhang(g, &mut st);
    st.porch = None;
    st.fill = None;
    st.inside = None;
    let (input, cam) = (g.host_input, g.host_cam);
    if let Some(i) = input { hands_off(g, i); }
    crate::game::game_restart_encounter(g);
    if let Some(c) = cam { g.tab.camera[c].kick = None; }
    // (the card asked for, if its man is on the team as it now stands)
    if st.who > 0 && crate::scenario_run::team_of(g).get(st.who as usize - 1).is_none() { st.who = 0; }
    place(g, &mut st);
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

/// ROUND THE TICK, BEFORE IT: the park; the seat's hand says nothing but the cursor on the door's foot.
pub fn before(g: &mut Game, cmd: &mut PlayerCmd) {
    let Some(mut st) = take(g) else { return };
    cmd.keys.clear();
    cmd.pressed.clear();
    cmd.buttons = 0.0;
    cmd.clicked = 0.0;
    cmd.wheel = 0.0;
    if st.none { g.kit_stage = Some(st); return; }
    let pl = g.player;
    // (a man down, a mission ended: nothing the picture shows, so it is staged again)
    let down = g.pl[pl].down || st.order.iter().any(|&x| g.gd[x].state == GuardState::Dead) || staged_way_of(g, st.asks_none).ended;
    if down { g.kit_stage = Some(st); restage(g); st = match take(g) { Some(s) => s, None => return }; if st.none { g.kit_stage = Some(st); return; } }
    let (men, alarms) = park(g);
    st.roused_men += men;
    st.roused_alarms += alarms;
    let vp = &cmd.view.vp;
    if vp.len() == 16 {
        let p = st.foot;
        let cw = vp[3] * p[0] + vp[7] * p[1] + vp[11] * p[2] + vp[15];
        if cw > 0.0 {
            cmd.mx = ((vp[0] * p[0] + vp[4] * p[1] + vp[8] * p[2] + vp[12]) / cw * 0.5 + 0.5) * cmd.w;
            cmd.my = (1.0 - ((vp[1] * p[0] + vp[5] * p[1] + vp[9] * p[2] + vp[13]) / cw * 0.5 + 0.5)) * cmd.h;
        }
    }
    home_men(g, &mut st);
    pool_sync(g, &st);
    g.kit_stage = Some(st);
}

/// ROUND THE TICK, AFTER IT: what the step made of anything is not carried into the next, and what the game writes each step goes back as staged.
pub fn after(g: &mut Game, _dt: f64) {
    let Some(mut st) = take(g) else { return };
    if st.none { g.kit_stage = Some(st); return; }
    torches(g, &st);
    let (men, alarms) = park(g);
    st.roused_men += men;
    st.roused_alarms += alarms;
    if let Some(l) = st.fill { if st.fill_at > 0.0 { let r = &mut g.tab.light[l]; r.intensity = st.fill_at; r.peak_intensity = st.fill_at; } }
    if let (Some(a), Some(at)) = (st.moon, st.moon_at) { g.tab.light[a.light].intensity = at; }
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

/// a frame message's numbers, as the page may send them: what is out of its range falls back, a hung door or a column out of order is not asked for
pub fn frame_ask(cx: f64, cy: f64, h: f64, w: f64, man: f64, hung: Option<[f64; 4]>, column: Option<[f64; 2]>) -> Option<FrameAsk> {
    if !(cx.is_finite() && cy.is_finite() && h.is_finite()) || !(h > 0.02 && h <= 1.0) { return None; }
    let hung = hung.filter(|x| { let [dr, l, r, _] = *x; dr.is_finite() && l.is_finite() && r.is_finite() && l >= -1.0 && r <= 1.0 && l < dr && dr < r }).map(|[dr, l, r, k]| [dr, l, r, if k > 0.0 && k <= 1.0 { k } else { 1.0 }]);
    let column = column.filter(|x| { let [t, b] = *x; t.is_finite() && b.is_finite() && t <= 1.5 && b >= -1.5 && t - b > 0.2 });
    Some(FrameAsk { cx: m::max(-1.0, m::min(1.0, cx)), cy: m::max(-1.0, m::min(1.0, cy)), h, w: if w > 0.02 && w <= 1.0 { w } else { 1.0 }, man: if man > 0.02 && man <= 1.0 { man } else { 1.0 }, hung, column })
}
/// THE COMMANDS. Where the page wants the men in its view: `ask` none leaves it as it was.
pub fn frame(g: &mut Game, aspect: f64, ask: Option<FrameAsk>) {
    let Some(st) = g.kit_stage.as_mut() else { return };
    if aspect > 0.0 && aspect.is_finite() { st.aspect = aspect; }
    if ask.is_some() { st.frame = ask; }
    aim(g);
}
/// whose card is open (0 the player) and the row under its highlight: the men stand again with that man first at the door
pub fn open(g: &mut Game, who: u32, field: &str) {
    let team = crate::scenario_run::team_of(g).len();
    let Some(st) = g.kit_stage.as_mut() else { return };
    let i = if who >= 1 && who as usize <= team { who } else { 0 };
    st.field = String::from(field);
    if i == st.who { return; }
    st.who = i;
    restage(g);
}
/// the cards are written (the caller's) and how many come (under 0: as it stands; a quiet or a solo mission's men are the mission's own): staged again
pub fn cards(g: &mut Game, size: i32) {
    let Some(st) = g.kit_stage.as_ref() else { return };
    if size >= 0 && size <= 4 && !staged_way_of(g, st.asks_none).fixed {
        if g.rescue.active { g.k.rescue.squad = size as f64; } else { g.k.nightbreach.squad = size as f64; }
        g.kit_stage.as_mut().unwrap().size = Some(size as u32);
    }
    restage(g);
}

/// ONE TICK OF A HOST THAT SAMPLES ITS OWN HAND (the host with no picture): `game_update`'s lines with the stage round the step.
pub fn update(g: &mut Game, dt: f64, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    let me = crate::game::game_host_seat(g);
    crate::math_dice::use_dice(g);
    crate::director::director_inject_input(g, input);
    let into = g.cmd_buf.take();
    let c = crate::seats::with_seat(g, me, &mut |g| crate::cmd::sample_cmd(g, input, cam, canvas, into.as_ref()));
    g.cmd_buf = Some(c.clone());
    let mut staged = c;
    before(g, &mut staged);
    crate::game::game_step(g, dt, &staged);
    crate::game::game_host_after(g, input, cam, canvas);
    after(g, dt);
}
