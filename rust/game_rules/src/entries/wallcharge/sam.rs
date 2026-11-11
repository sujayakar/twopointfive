use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// holeAt's `open`: no solid thing on the rays through the wall at a man's shin and chest
fn hole_open(g: &Game, w: &WallSpot, t: f64, du: f64) -> bool {
    let (a, b) = (wall_pt(g, w, du, FACE, Some(0.35)), wall_pt(g, w, du, -(t + FACE), Some(0.35)));
    if crate::scene_collision::solid_blocked(g, a, b, None) { return false; }
    let (a, b) = (wall_pt(g, w, du, FACE, Some(1.3)), wall_pt(g, w, du, -(t + FACE), Some(1.3)));
    !crate::scene_collision::solid_blocked(g, a, b, None)
}
/// holeAt's `through`: floor at both mouths
fn hole_through(g: &Game, w: &WallSpot, t: f64, du: f64) -> bool {
    let mouth_in = g.k.wallcharge.mouth_in;
    let far = xz(wall_pt(g, w, du, -(t + mouth_in), None));
    if crate::scene_collision::nav_grid_is_blocked(g, far[0], far[1]) { return false; }
    let near = xz(wall_pt(g, w, du, 0.35, None));
    !crate::scene_collision::nav_grid_is_blocked(g, near[0], near[1])
}
/// holeAt's `shut`: an open run ends at `to`; if a man fits, the way through nearest its middle
fn hole_shut(g: &Game, w: &WallSpot, t: f64, step: f64, run: &mut f64, best: &mut Option<WallChargeHoleAt>, to: f64) {
    if run.is_nan() { return; }
    let a = *run - step / 2.0;
    let b = to - step / 2.0;
    let mid = (a + b) / 2.0;
    *run = f64::NAN;
    if b - a < 0.7 { return; }
    let mut k = 0.0;
    while mid - k >= a + 0.35 - 1e-6 {
        let (dus, n) = if js::truthy(k) { ([mid - k, mid + k], 2) } else { ([mid, mid], 1) };
        for &du in &dus[..n] {
            if hole_through(g, w, t, du) {
                let h = WallChargeHoleAt { du, half: m::min(du - a, b - du) };
                let better = match best { None => true, Some(was) => m::abs(h.du) < m::abs(was.du) };
                if better { *best = Some(h); }
                return;
            }
        }
        k += step;
    }
}

/// The gap a man fits through nearest the charge, read off the wall as it stands
pub fn wall_charge_hole_at(g: &crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<crate::wallcharge::WallChargeHoleAt> {
    let t = thick(g, w);
    let (step, r) = (0.125, 2.5);
    let mut best: Option<WallChargeHoleAt> = None;
    let mut run = f64::NAN;
    let (bc, bh) = { let b = &g.tab.boxes[w.box_]; (b.c, b.h) };
    let c = if w.along_x { bc[0] } else { bc[2] };
    let l = if w.along_x { bh[0] } else { bh[2] };
    let lo = m::max(-r, c - l - w.u);
    let hi = m::min(r, c + l - w.u);
    let mut du = lo + step / 2.0;
    while du <= hi + 1e-6 {
        if hole_open(g, w, t, du) {
            if run.is_nan() { run = du; }
        } else {
            hole_shut(g, w, t, step, &mut run, &mut best, du);
        }
        du += step;
    }
    hole_shut(g, w, t, step, &mut run, &mut best, du);
    best
}

/// WALLSHOT: the shock with no flash
pub(super) fn wall_charge_shock(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) {
    let (c, fa) = (g.gd[gd].ch, g.gd[gd].weapon);
    crate::condition::fold(g, c, secs);
    g.fa[fa].burst = None;
    crate::condition::dazed(g, gd, secs, at, 0.6, "wallcharge");
    crate::character::anim_hit(g, c);
}

/// Flash-blind
pub(super) fn wall_charge_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) {
    let c = g.gd[gd].ch;
    crate::condition::dazzled(g, gd, secs, at, None, "wallcharge");
    crate::character::anim_hit(g, c);
}

/// The charge on the wall: a dark slab and a lit tell, one-frame boxes
pub fn wall_charge_draw(g: &mut crate::game::Game, c_2: WallChargeId) {
    let height = g.k.wallcharge.height;
    let w = g.tab.wallcharge_set_wall_charge[c_2].wall.clone();
    let p = wall_pt(g, &w, 0.0, 0.022, Some(height));
    let yaw = if w.along_x { 0.0 } else { m::PI / 2.0 };
    let n = wall_pt(g, &w, 0.0, 1.0, None);
    let o = wall_pt(g, &w, 0.0, 0.0, None);
    let flag = crate::scene_boxes::box_flag();
    let slab = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP {
        c: p,
        h: [0.07, 0.11, 0.02],
        yaw: Some(yaw),
        albedo: Some([0.09, 0.1, 0.08]),
        flags: Some(flag.dynamic),
        ..Default::default()
    });
    g.fx.push(crate::game::GameFxItem { box_: slab, ttl: 1e-3 });
    let tell = crate::scene_boxes::make_box_kept(g, &crate::scene_boxes::MakeBoxP {
        c: [p[0] + (n[0] - o[0]) * 0.022, p[1] + 0.07, p[2] + (n[2] - o[2]) * 0.022],
        h: [0.012, 0.012, 0.004],
        yaw: Some(yaw),
        albedo: Some([0.9, 0.12, 0.08]),
        emissive: Some([2.2, 0.2, 0.1]),
        flags: Some((js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64),
        ..Default::default()
    });
    g.fx.push(crate::game::GameFxItem { box_: tell, ttl: 1e-3 });
}

/// Is this set charge the squad's key's to blow: one the squad set, or his own while an order of theirs is on that wall
pub fn wall_charge_key_is(g: &crate::game::Game, seat: &crate::seats::SeatRef, c_2: WallChargeId) -> bool {
    let c = &g.tab.wallcharge_set_wall_charge[c_2];
    c.by != crate::ownblow::SAM_BY
        || seat.wall_charge.ops.iter().any(|&o| same_wall(&g.tab.wallcharge_wall_op[o].wall, &c.wall))
}

/// The nearest charge his own hands set
pub fn wall_charge_sam_set(g: &mut crate::game::Game) -> Option<WallChargeId> {
    if !g.sets[g.seat].wall_charge.seated { return None; }
    wall_charge_nearest(g, Some(&mut |g: &mut Game, c: WallChargeId| g.tab.wallcharge_set_wall_charge[c].by == crate::ownblow::SAM_BY))
}

/// Where a set charge sits
pub fn wall_charge_sam_at(g: &crate::game::Game, c_2: WallChargeId) -> V3 {
    let w = g.tab.wallcharge_set_wall_charge[c_2].wall.clone();
    let height = g.k.wallcharge.height;
    wall_pt(g, &w, 0.0, 0.0, Some(height))
}

/// Who stands inside its radius, in words
pub fn wall_charge_sam_words(g: &crate::game::Game, c_2: WallChargeId) -> String {
    wall_charge_close_words(g, Some(c_2), None)
}

/// Would it put HIM down where he stands
pub fn wall_charge_sam_fatal(g: &crate::game::Game, c_2: WallChargeId) -> bool {
    if g.god_mode { return false; }
    let p = g.player_pos(g.player);
    let w = g.tab.wallcharge_set_wall_charge[c_2].wall.clone();
    let at = wall_pt(g, &w, 0.0, 0.0, None);
    let off = wall_off(g, &w, p);
    let t = thick(g, &w);
    v3::dist_xz(p, at) < (if off < -t / 2.0 { g.k.doortools.knock_m } else { g.k.wallkill.near_m })
}

/// The squad block's lines: what is carried, while this run brought wall charges of its own, and a charge waiting for the trigger: the squad's key for those that are its to blow, the interaction's for his own, a note each, counted apart.
pub fn wall_charge_hud_notes(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Vec<crate::view::Note> {
    use crate::view::Note;
    if !wall_charge_on(g) { return Vec::new(); }
    let W = seat.wall_charge;
    let mut out = Vec::new();
    if let Some(n) = W.carried { if W.brought.unwrap_or(0.0) > 0.0 { out.push(Note { key: String::new(), text: alloc::format!("wall charges {}", n) }); } }
    if W.set.is_empty() { return out; }
    let mark = !crate::pairorders::pair_orders_waiting(g, seat).is_empty();   // (while his partner waits on a mark, the key is the mark first: the note says so)
    let f = crate::doortools::blow_letter(g);
    let p = g.seat_pos(seat);
    for own in [false, true] {
        let of: Vec<WallChargeId> = W.set.iter().copied().filter(|&c| wall_charge_key_is(g, seat, c) != own).collect();
        if of.is_empty() { continue; }
        let mut by: Vec<(WallChargeId, f64)> = of.iter().map(|&c| (c, v3::dist_xz(wall_pt(g, &g.tab.wallcharge_set_wall_charge[c].wall, 0.0, 0.0, None), p))).collect();
        js::sort_by(&mut by, |a, b| a.1 - b.1);
        let many = if of.len() > 1 { alloc::format!(" ({})", of.len()) } else { String::new() };
        let after = if mark && !own { " · after the mark" } else { "" };
        out.push(Note { key: crate::ownblow::blow_key_word(!own, &f), text: alloc::format!("blow the charge · the wall{}{}{}", many, after, wall_charge_close_words_of(g, seat, by.first().map(|x| x.0), Some(true))) });
    }
    out
}

/// With the cursor on a partition and Sam at its foot, the marker and its row
pub fn wall_charge_rows(g: &mut crate::game::Game, p: V3, busy: bool, o: &mut crate::player::Offers) {
    g.sets[g.seat].wall_charge.offered = None;
    g.sets[g.seat].wall_charge.marker = None;
    if !wall_charge_on(g) || !g.sets[g.seat].wall_charge.seated { return; }
    let pl = g.player;
    let sam_wall = g.pl[pl].wall_work().map(|s| s.wall.clone());
    let wall = match sam_wall {
        Some(w) => Some(w),
        None => {
            let aim = g.sets[g.seat].aim_point;
            match wall_at(g, aim, Some(p)) {
                Some(w) => Some(wall_charge_settle(g, &w)),
                None => None,
            }
        }
    };
    let w = match wall { Some(w) if w.ok => w, _ => return };
    let c = wall_charge_charge_on(g, &crate::seats::here(g), &w);
    let set_off = g.k.wallcharge.set_off;
    let foot = wall_pt(g, &w, 0.0, set_off, None);
    let d = v3::dist_xz(p, foot);
    let setting = g.pl[pl].wall_work().is_some();
    if d > g.k.wallcharge.sam_marker_m || (c.is_none() && !setting && wall_charge_open_at(g, &w)) { return; }
    if c.is_none() && !setting && !wall_charge_affordable(g, &crate::seats::here(g)) { return; }
    let near = d < g.k.wallcharge.sam_reach && wall_off(g, &w, p) > 0.0 && {
        let to = wall_pt(g, &w, 0.0, 0.1, Some(0.9));
        !crate::scene_collision::solid_blocked(g, [p[0], 0.9, p[2]], to, None)
    };
    let free = !g.pl[pl].sprinting && g.pl[pl].tow.is_none() && !g.pl[pl].on_a_man();
    let reach = near && free;
    let f = crate::doortools::blow_letter(g);
    let key = crate::ownblow::blow_key_word(false, &f);
    let line1 = if c.is_some() { "wall · charge set" } else if w.outer == Some(true) { "wall · outer wall" } else { "wall · partition" };
    let line2 = if reach { "" } else if near { "hands full" } else { "step closer" };
    let height = g.k.wallcharge.height;
    let pos = wall_pt(g, &w, 0.0, 0.05, Some(height));
    let mut it = crate::player::mark(crate::game::InteractableKind::Wall, pos, line1, line2, reach);
    if setting {
        it.in_reach = true;
        g.sets[g.seat].wall_charge.marker = Some(it.clone());
        o.add(it, Vec::new());
        return;
    }
    if let Some(c) = c {
        let key_is = wall_charge_key_is(g, &crate::seats::here(g), c);
        let word = crate::ownblow::blow_key_word(key_is, &f);
        let close = wall_charge_close_words(g, Some(c), None);
        it.line2 = alloc::format!("{}  blow the charge{}", word, close);
        o.add(it, Vec::new());
        return;
    }
    let mut verbs: Vec<crate::game::Interactable> = Vec::new();
    let going_on = reach && g.sets[g.seat].wall_charge.ops.iter().any(|&o| {
        let op = &g.tab.wallcharge_wall_op[o];
        op.phase == Phase::Place && same_wall(&op.wall, &w)
    });
    if going_on {
        it.line2 = String::from("a charge is going on it");
    } else if reach && !busy {
        let words = wall_charge_row_words(g, &crate::seats::here(g));
        verbs.push(crate::player::row(&it, crate::game::Verb::Charge, &words, &alloc::format!("{}  blows it once set", key)));
        g.sets[g.seat].wall_charge.offered = Some(w);
    }
    o.add(it, verbs);
}

/// The row performed starts it; he stands on it setSecs; a step, a sprint, full hands or another verb breaks it off
pub(super) fn wall_charge_sam_tick(g: &mut crate::game::Game) {
    let pl = g.player;
    let ch = g.pl[pl].ch;
    let p = g.ch[ch].pos;
    let height = g.k.wallcharge.height;
    let has_act = g.sets[g.seat].act.is_some();
    let mine = match &g.sets[g.seat].act {
        Some(a) => a.kind == crate::game::InteractableKind::Wall && a.verb == Some(crate::game::Verb::Charge),
        None => false,
    };
    if mine && g.pl[pl].wall_work().is_none() {
        if let Some(wall) = g.sets[g.seat].wall_charge.offered.clone() {
            let t0 = g.time;
            crate::deed::begin(g, crate::game::Deed::Wall(WallChargeSam { wall, t0, at: [p[0], 0.0, p[2]] }));
            return;
        }
    }
    let Some(s) = g.pl[pl].wall_work().cloned() else { return };
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].sprinting
        || v3::dist_xz(p, s.at) > g.k.wallcharge.sam_shift || (has_act && !mine) {
        g.pl[pl].wall_work_over();
        return;
    }
    if crate::workstow::sam_stowing(g, &crate::seats::here(g)) {
        let now = g.time;
        if let Some(w) = g.pl[pl].wall_work_mut() { w.t0 = now; }
        return;
    }
    let set_secs = g.k.doortools.set_secs;
    let at = wall_pt(g, &s.wall, 0.0, 0.0, Some(height));
    let hand = if crate::game::player_pistol_up(g, pl) { Some(0.0) } else { None };
    let el = g.time - s.t0;
    crate::handwork::charge_hands(g, ch, at, wall_n(&s.wall), el, set_secs, hand);
    if g.time - s.t0 < set_secs { return; }
    g.pl[pl].wall_work_over();
    if wall_charge_open_at(g, &s.wall) || wall_charge_charge_on(g, &crate::seats::here(g), &s.wall).is_some() { return; }
    let paid = wall_charge_take(g);
    if paid < 0.0 {
        return;
    }
    let free = if g.sets[g.seat].wall_charge.free_set { Some(true) } else { None };
    let row = SetWallCharge { wall: s.wall, t: g.time, by: String::from(crate::ownblow::SAM_BY), paid, free };
    let id = g.tab.wallcharge_set_wall_charge.push(row);
    g.sets[g.seat].wall_charge.set.push(id);
    crate::audio_iface::game_audio_like_play(g, "lockPick", Some(at), Some(0.6), None);
    crate::furnblast::furniture_warm(g);
}

/// The charge Sam is setting by hand, for whoever stands him at it.
pub fn wall_charge_sam_job(g: &crate::game::Game) -> Option<crate::wallcharge::WallChargeSamJob> {
    let height = g.k.wallcharge.height;
    let (wall, at) = g.pl[g.player].wall_work().map(|s| (s.wall.clone(), s.at))?;
    Some(WallChargeSamJob { p: wall_pt(g, &wall, 0.0, 0.0, Some(height)), n: wall_n(&wall), at })
}

/// The one row while he is at it
pub fn wall_charge_running_row(g: &crate::game::Game) -> crate::game::Interactable {
    let s = g.pl[g.player].wall_work().unwrap();
    let it = match &g.sets[g.seat].wall_charge.marker {
        Some(it) => it.clone(),
        None => crate::player::mark(crate::game::InteractableKind::Wall, wall_pt(g, &s.wall, 0.0, 0.05, Some(g.k.wallcharge.height)), if s.wall.outer == Some(true) { "wall · outer wall" } else { "wall · partition" }, "", true),
    };
    crate::player::running(&it, crate::game::Verb::Charge, "setting the charge… — Space or a step breaks it off", Some((g.time - s.t0) / g.k.doortools.set_secs))
}

/// A point's x and z
fn xz(p: V3) -> [f64; 2] {
    [p[0], p[2]]
}
