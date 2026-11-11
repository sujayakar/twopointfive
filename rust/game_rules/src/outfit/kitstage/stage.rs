//! THE KIT PICTURE'S STAGE: the Team & kit screen's picture of the team at the mission's own way in. The mission is started as Deploy would start it; then this stands the team on the plan's places, parks
//! everybody else far off and calm, hangs the lamps and pins the camera. NO RULE OF THE GAME READS THE STAGE: it writes only what any host could write (men, lights, boxes, a pin) round the tick.
use super::acts;
use super::place::place;
use super::aim::R;
use super::light::{KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK, KIT_VIEW_STAGE as ST};
use super::plan::KIT_PLAN;
use super::types::*;
use crate::cmd::PlayerCmd;
use crate::game::{Game, GuardState};
use crate::ids::*;
use crate::scene_boxes::box_flag;
use crate::{js, m};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

/// a man no event can reach: past any number an event can get
const DEAF: f64 = 9007199254740991.0;

pub(super) fn unit(a: V3, b: V3) -> V3 {
    let v = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let len = m::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    let len = if len != 0.0 { len } else { 1.0 };
    [v[0] / len, v[1] / len, v[2] / len]
}
pub(super) fn free(g: &Game, x: f64, z: f64) -> bool { !crate::scene_collision::nav_grid_is_blocked(g, x, z) }
fn take(g: &mut Game) -> Option<Box<KitStage>> { g.kit_stage.take() }

/// A point by [along the wall toward the stack, out on the team's side, up] from the anchor's middle.
pub(super) fn at_of(p: &Planned, o: V3) -> V3 {
    let (mid, s, n) = (p.anchor.mid, p.s, p.anchor.n);
    [mid[0] + s[0] * o[0] + n[0] * o[1], o[2], mid[1] + s[1] * o[0] + n[1] * o[1]]
}

/// THE WAY THE MISSION THAT IS UP STAGED ITS TEAM ON, whatever its kind, and whether it has ended. A quiet mission and the solo night have way 0 and their own men.
struct Way { active: bool, way: usize, ended: bool, fixed: bool }
fn staged_way_of(g: &Game, asks_none: bool) -> Way {
    let s = &g.staged;
    Way { active: s.up.is_some() || asks_none, way: s.way as usize, ended: s.over, fixed: !s.facts.fight }
}
/// the way a game stood from a link stages: its index, or none (no mission started)
pub fn staged_way(g: &Game, asks_none: bool) -> Option<usize> { let w = staged_way_of(g, asks_none); if w.active { Some(w.way) } else { None } }

pub(super) fn ops(g: &Game) -> Vec<GuardId> { g.staged.team.iter().copied().filter(|&x| !crate::condition::down(g, g.gd[x].ch)).collect() }

/// EVERYBODY WHO IS NOT THE TEAM, far off, dark, held, calm and deaf, the level's devices inert, and the alarm a stray noise placed dropped while the floor is at 0. A blast is made inside the step, so the mark
/// on a man's ears is set past any number an event can get, and renewed here. Answers what it had to put right: men found roused, the alarm's marks dropped.
pub(super) fn park(g: &mut Game) -> (u32, u32) {
    g.tune.devices = false;
    g.tune.lasers = false;
    for i in 0..g.events.len() { if g.events[i].id.is_none() { g.event_seq += 1.0; g.events[i].id = Some(g.event_seq); } }
    let (mut k, mut men, mut alarms) = (0.0, 0, 0);
    let all: Vec<GuardId> = g.guards.iter().chain(g.civilians.iter()).copied().collect();
    for x in all {
        if g.team(x).ours() { continue; }
        let (c, light) = (g.gd[x].ch, g.gd[x].flashlight);
        crate::character::pos_replace(g, c, [-40.0 - k * 3.0, 0.0, -40.0]);
        k += 1.0;
        g.tab.light[light].enabled = false;
        crate::duty::hold(g, x, true, "kitstage");
        crate::duty::pin(g, x, true, "kitstage");
        g.gd[x].heard_up_to = DEAF;
        crate::mind::set(g, x, 0.0, "kitstage");
        crate::mind::fix_keep(g, x, None, "kitstage");
        if !crate::condition::down(g, g.gd[x].ch) {
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
        let mut row = g.tab.boxes.edit(b);
        let r = m::hyp2(row.h[0], row.h[2]);
        x0 = m::min(x0, row.c[0] - r); x1 = m::max(x1, row.c[0] + r); z0 = m::min(z0, row.c[2] - r); z1 = m::max(z1, row.c[2] + r);
        row.c[1] -= 30.0;
        st.sunk.push(i);
        n += 1;
    }
    g.kit_stage = Some(st);
    if n > 0 {
        g.boxes_changed();
        let _ = crate::scene_collision::nav_grid_rebake(g, x0, z0, x1, z1);
        crate::scene_collision::sound_paths_invalidate(g);
    }
}
pub(super) fn drop_props(g: &mut Game, hidden: &[usize]) {
    for &i in hidden { if let Some(&p) = g.props.all.get(i) { crate::props::prop_system_remove(g, p); } }
}

/// the room behind the door at this share of itself: each lamp's own base, which the game multiplies each step, and its fixture's glow
pub(super) fn quiet_inside(g: &mut Game, st: &mut KitStage, k: f64, blade: f64) {
    st.a.blade = blade;
    for a in &st.lit_in {
        g.tab.fixture[a.t].on = true;
        g.tab.fixture[a.t].base_intensity = a.intensity * k;
        g.tab.fixture[a.t].base_emissive = [a.emissive[0] * k, a.emissive[1] * k, a.emissive[2] * k];
    }
    if let Some(o) = st.inside { let l = &mut g.tab.light[o]; l.intensity = QK.own.intensity * blade; l.peak_intensity = l.intensity; }
}
pub(super) fn pool_sync(g: &mut Game, st: &KitStage) {
    let Some(p) = st.pool else { return };
    let i = st.fixture.and_then(|f| g.engine.world.statics.iter().position(|&b| b == f));
    let row = &mut g.tab.fixture[p];
    row.boxes = i.map_or(Vec::new(), |i| alloc::vec![i as f64]);
    if st.porch.is_some() { row.light = st.porch; }
}
/// THE ROOF STAYS ON: the level's boxes, by place, that the quiet picture draws though the game's camera looks through them: a cover flagged NoPrimary, no light's panel, that lies wholly behind the wall. None
/// where the team itself stands under a roof.
pub(super) fn quiet_roofs(g: &Game, p: &Planned) -> Vec<usize> {
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
pub(super) struct QuietPlaces { pub sam: Option<V3>, pub mate: Option<V3>, pub back: Option<V3> }
pub(super) fn quiet_places(g: &Game, p: &Planned, mates: usize) -> QuietPlaces {
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

pub(super) fn lamps_out(g: &Game) -> bool {
    let h = crate::hours::hour_of(g);
    h != crate::hours::HourId::Night && !crate::hours::hours().values().any(|d| d.id == h && d.lamps)
}
/// the hour has the level's outside lamps out: the porch lamp is on the same photocell
pub fn hour_has_lamps_out(g: &Game) -> bool { lamps_out(g) }

pub(super) fn or<T: Copy>(a: Option<T>, b: T) -> T { a.unwrap_or(b) }

/// a hung lamp as the picture has it
pub(super) struct Hung { pub kind: f64, pub pos: V3, pub dir: V3, pub color: V3, pub intensity: f64, pub range: f64, pub inner: f64, pub outer: f64, pub radius: f64, pub volumetric: f64, pub broad: bool }
pub(super) fn hang(g: &mut Game, name: &str, h: &Hung) -> LightId {
    let mut l = crate::render_lights::RtLight::default();
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
pub(super) fn torches(g: &mut Game, st: &mut KitStage) {
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
        let (p, low) = match st.mate { Some(p) if k == 0 && !is_open => (p, st.quiet), _ => (st.spots[k], false) };
        let mine = crate::orders::squad_orders_of(&crate::seats::here(g), x).map_or(false, |a| g.tab.task[a].order == crate::orders::rows::cover::ORDER.id);
        if mine && st.sent.contains(&x) { continue; }
        super::walk::send(g, x, p, super::walk::faces(st, p), None, K.pace, true, low);
        if !st.sent.contains(&x) { st.sent.push(x); }
    }
}
pub(super) fn hands_off(g: &mut Game, i: InputId) { let r = &mut g.tab.input[i]; r.keys.clear(); r.pressed.clear(); r.buttons = 0.0; r.clicked = 0.0; }

/// THE STAGING, off the stage's plan. On a failure the stage says `none`, never a panic.
pub fn stage(g: &mut Game) {
    let Some(mut st) = take(g) else { return };
    place(g, &mut st);
    g.kit_stage = Some(st);
}
/// WHAT EVERY STAGING STARTS FROM: the mission from its stand, and the stage as it is before its first staging. A staged game is never mended and never grows: it is thrown away whole.
pub fn keep_base(g: &mut Game) { crate::savestate::control::keep_first(g); }

/// THE MISSION FROM ITS STAND, with new men, and the team staged again, as the cards, the size and the open card now stand.
pub fn restage(g: &mut Game) {
    let Some(mut was) = take(g) else { return };
    // (the file's order, as team indexes: every row id the stage holds is dead after this)
    let team = g.staged.team.clone();
    let keep: Vec<usize> = was.order.iter().filter_map(|x| team.iter().position(|t| t == x)).collect();
    // (the tables go on as they are, so what a row wrote for its length is put back first)
    acts::tables_back(g, &mut was);
    if !crate::savestate::control::again(g, was.size) { was.due = false; g.kit_stage = Some(was); return; }
    let Some(mut st) = take(g) else { return };
    (st.want, st.field, st.size, st.frame, st.aspect) = (was.want, was.field.clone(), was.size, was.frame, was.aspect);
    (st.roused_men, st.roused_alarms) = (was.roused_men, was.roused_alarms);
    (st.veil, st.still, st.shown, st.keep) = (was.veil, was.still, was.shown, Some(keep));
    acts::carry(&was.a, &mut st.a);
    place(g, &mut st);
    // (another picture than the last: nothing of it is blended into this one)
    g.restart_cut = true;
    g.kit_stage = Some(st);
}

/// the camera on the men, for the stage in hand: the plan's pin, moved and scaled to where the page's frame wants them. Both framings: the quiet one, for one subject, and the one hung on the door.
pub(super) fn aim_stage(g: &mut Game, st: &mut KitStage) {
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
fn aim(g: &mut Game) {
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
    if st.still && !st.none && (crate::condition::down(g, g.pl[pl].ch) || st.order.iter().any(|&x| crate::condition::down(g, g.gd[x].ch)) || staged_way_of(g, st.asks_none).ended) { st.due = true; }
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
        super::walk::sam_hand(g, &mut st, cmd);
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
    if g.cond[g.pl[pl].ch].blind_until > now { g.cond[g.pl[pl].ch].blind_until = -1.0; }
    if g.cond[g.pl[pl].ch].fold_until > now { g.cond[g.pl[pl].ch].fold_until = -1.0; }
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
    if let Some(n) = size { (g.k.rescue.squad, g.k.nightbreach.squad) = (n as f64, n as f64); }   // (whichever is up stages so many)
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
