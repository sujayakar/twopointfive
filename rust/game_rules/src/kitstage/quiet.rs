//! THE QUIET PICTURE'S OWN ROWS: his suppressed gun puts the lamp out, the lean, his light's one blink; LOUD KIT'S COST (a loud row on a quiet job is answered from inside the room); LIFE AT REST (off as shipped).
use super::acts::{gas_out, off_rest, press, to};
use super::aim::{aim_legs, clamp, quiet_tick, Step, R};
use super::light::{KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK};
use super::plan::KIT_PLAN;
use super::types::*;
use super::walk::{along, at, away, flat, free, out_of, sam_goes, sam_stand, v2, yaw_to};
use crate::doors::door_off;
use crate::game::{Game, GamePuppet, SlotKind};
use crate::ids::*;
use crate::m;
use crate::sim_lightshome::{self as home, light_in};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

fn smooth(u: f64) -> f64 { let x = clamp(u, 0.0, 1.0); x * x * (3.0 - 2.0 * x) }
fn lamp_up(g: &Game, st: &KitStage) -> bool {
    st.pool.map_or(false, |p| home::fixture_on(g, p) && !home::fixture_broken(g, p) && home::fixture_disabled_until(g, p) < g.time && g.tab.sim_lightshome_fixture[p].light.is_some())
}
fn gun_def(g: &Game, seat: &crate::seats::SeatRef) -> crate::weapons::WeaponId { let pl = seat.player; g.fa[if g.pl[pl].slot == SlotKind::Rifle { g.pl[pl].rifle } else { g.pl[pl].pistol }].def }
fn in_hand(g: &Game, seat: &crate::seats::SeatRef) -> bool { matches!(g.pl[seat.player].slot, SlotKind::Pistol | SlotKind::Rifle) }
fn draw_key(g: &Game, seat: &crate::seats::SeatRef) -> &'static str { if g.pl[seat.player].slot == SlotKind::Rifle { "Digit2" } else { "Digit1" } }

/// HIS SUPPRESSED GUN PUTS THE LAMP OUT: from the crouch the gun comes out, he rises on the lamp, one round (the game's own: the lamp is a target of its list), and the pool dies. The dark is held: the fill lifts and he walks through where the pool was. Then the room is staged again. Refused where there is no lamp burning or the gun is not suppressed.
pub(super) fn gun_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if b.man.is_some() { return Err(String::from("the lamp is Sam's to shoot")); }
    if !st.quiet || !lamp_up(g, st) { return Err(String::from("no lamp burning to put out")); }
    if !in_hand(g, &crate::seats::here(g)) { return Err(String::from("no gun in his hand")); }
    if !g.m.weapons.weapons.get(gun_def(g, &crate::seats::here(g))).map_or(false, |d| d.suppressed) { return Err(String::from("not a suppressed gun")); }
    to(g, b, "draw");
    b.h_to = 1.1;
    sam_stand(g, st);
    if g.pl[g.player].holstered { press(st, draw_key(g, &crate::seats::here(g))); }
    if QK.cut && st.a.lead_was.is_none() { st.a.lead_was = Some(g.m.fixture_break.fixlook.on); g.m.fixture_break.fixlook.on = false; }
    Ok(())
}
pub(super) fn gun_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let now = g.time;
    let el = now - b.pt;
    let p = g.ch[c].pos;
    let Some(lamp) = st.pool else { return Step::Done };
    let lamp_pos = g.tab.sim_lightshome_fixture[lamp].pos;
    if b.phase == "draw" {
        let busy = crate::character::anim_kit_busy(g, c);
        let at_home = flat(p, st.sam) <= K.on_slot;
        if ((g.pl[pl].holstered || busy || g.pl[pl].equip_t > 0.0 || el < 0.15) && el < 2.5) || (!at_home && el < 4.0) { return Step::On; }
        if g.pl[pl].holstered { b.marks.push("kept"); st.a.refused = Some(String::from("weapon: the gun never came out")); return Step::Done; }
        let d = m::max(0.5, flat(lamp_pos, [p[0], p[2]]));
        let h0 = 1.15;
        b.quiet = true;
        b.h_to = h0 + (lamp_pos[1] - h0) * 3.0 / d;
        b.legs = vec![Leg { yaw: yaw_to(p, lamp_pos), cap: K.gun_rate * R, min_t: QK.rise, hold: 0.1 }];
        b.leg = 0;
        b.leg_t = f64::NAN;
        to(g, b, "rise");
        return Step::On;
    }
    if b.phase == "rise" {
        st.a.rise = smooth(el / QK.rise);
        if !aim_legs(g, st, b) || el < QK.rise { return Step::On; }
        super::acts::fire_at(g, None, lamp_pos);
        st.a.shots += 1;
        b.shots += 1;
        b.shot_t = now;
        b.marks.push("shot");
        to(g, b, "shot");
        if QK.flick.intensity > 0.0 {
            let y = g.ch[c].aim_yaw;
            crate::render_lights::light_set_flash(g, [p[0] + m::sin(y) * 0.5, 1.25, p[2] + m::cos(y) * 0.5], [0.7, 0.8, 1.0], QK.flick.intensity, QK.flick.range, QK.flick.secs, 0.0);
        }
        return Step::On;
    }
    if b.phase == "shot" {
        st.a.rise = 1.0;
        if home::fixture_broken(g, lamp) {
            b.marks.push("dark");
            if b.mark_t.is_nan() { b.mark_t = now; }
            st.a.lift_at = now;
            if QK.cut { gas_out(g, st, true); }
            to(g, b, "dark");
            b.legs = vec![Leg { yaw: yaw_to(p, at(st, [0.0, -1.5, 0.0])), cap: K.gun_rate * R, min_t: -1.0, hold: 0.0 }];
            b.leg = 0;
            b.leg_t = f64::NAN;
            b.h_to = 1.0;
            return Step::On;
        }
        if el < 0.4 { return Step::On; }
        b.marks.push("missed");
        st.a.refused = Some(String::from("weapon: the round did not put the lamp out"));
        b.legs = Vec::new();
        return quiet_tick(g, st, b);
    }
    if b.phase == "dark" {
        let t = now - b.mark_t;
        if QK.cut && t < 1.5 { gas_out(g, st, false); }
        st.a.rise = 1.0 - smooth((t - QK.walk_at) / 0.4);
        aim_legs(g, st, b);
        if t >= QK.walk_at && b.post.is_none() {
            let plan_face = st.plan.as_ref().map_or(0.0, |x| x.wall.face);
            let sg = if along(st, st.sam) > 0.0 { 1.0 } else { -1.0 };
            let jamb = |k: f64| at(st, [k * (st.width / 2.0 + QK.jamb_along), plan_face + QK.jamb_out, 0.0]);
            let jf = jamb(-sg);
            let walk_free = |g: &Game, a: V2, c: V2| { let n = m::ceil(m::hyp2(c[0] - a[0], c[1] - a[1]) / 0.25); let mut i = 1.0; while i <= n { if !free(g, [a[0] + (c[0] - a[0]) * i / n, a[1] + (c[1] - a[1]) * i / n]) { return false; } i += 1.0; } true };
            let j = if QK.jamb_far && free(g, v2(jf)) && walk_free(g, v2(p), v2(jf)) { jf } else { jamb(sg) };
            let post = if free(g, v2(j)) { v2(j) } else { v2(p) };
            b.post = Some(post);
            if flat(p, post) > 0.1 {
                b.marks.push("walk");
                let aim = g.puppet.as_ref().and_then(|q| q.aim);
                sam_goes(g, post);
                g.puppet.as_mut().unwrap().aim = aim;
            }
        }
        return if t >= QK.hold { Step::Done } else { Step::On };
    }
    quiet_tick(g, st, b)
}
/// THE SUIT, THE TRIM: no walk, no turn: he leans half a step toward the pool so that a shoulder catches it, holds, and leans back (down throughout)
pub(super) fn lean_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if b.man.is_some() { return Err(String::from("Sam's")); }
    let face = st.plan.as_ref().map_or(0.0, |x| x.wall.face);
    let t = at(st, [0.0, face + KIT_PLAN.off_face, 0.0]);
    let (dx, dz) = (t[0] - st.sam[0], t[2] - st.sam[1]);
    let len = m::hyp2(dx, dz);
    if !st.quiet || len < QK.lean_m + 0.2 { return Err(String::from("no pool to lean into")); }
    let p: V2 = [st.sam[0] + dx / len * QK.lean_m, st.sam[1] + dz / len * QK.lean_m];
    if !free(g, p) { return Err(String::from("no floor to lean onto")); }
    b.post = Some(p);
    to(g, b, "home");
    sam_stand(g, st);
    Ok(())
}
pub(super) fn lean_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let el = g.time - b.pt;
    match b.phase {
        "home" => { if flat(p, st.sam) > K.on_slot && el < 3.0 { return Step::On; } to(g, b, "out"); sam_goes(g, b.post.unwrap()); Step::On }
        "out" => {
            if flat(p, b.post.unwrap()) > 0.12 && el < 2.0 { return Step::On; }
            b.marks.push("lean");
            if b.mark_t.is_nan() { b.mark_t = g.time; }
            to(g, b, "hold");
            g.puppet = Some(GamePuppet::zeroed());
            Step::On
        }
        "hold" => { if el < QK.lean_secs { return Step::On; } to(g, b, "back"); sam_goes(g, st.sam); Step::On }
        _ => if flat(p, st.sam) <= K.on_slot || el >= 2.5 { Step::Done } else { Step::On },
    }
}
/// SAM'S WEAPON LIGHT on the quiet picture: one blink: the gun out, the beam on as his aim reaches the door's far jamb, across to the near one, held and off. No step, no hand on the leaf, no cost.
pub(super) fn light_start(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Result<(), String> {
    if b.man.is_some() { return Err(String::from("Sam's")); }
    if !st.quiet || (st.walled && !st.hole) { return Err(String::from("no door to cross")); }
    if !in_hand(g, &crate::seats::here(g)) { return Err(String::from("no gun in his hand")); }
    to(g, b, "draw");
    b.h_to = QK.light.aim_y;
    sam_stand(g, st);
    if g.pl[g.player].holstered { press(st, draw_key(g, &crate::seats::here(g))); }
    Ok(())
}
pub(super) fn light_tick(g: &mut Game, st: &mut KitStage, b: &mut Beat) -> Step {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let el = g.time - b.pt;
    let p = g.ch[c].pos;
    if b.phase == "draw" {
        let busy = crate::character::anim_kit_busy(g, c);
        if (g.pl[pl].holstered || busy || g.pl[pl].equip_t > 0.0 || el < 0.15) && el < 2.5 { return Step::On; }
        if g.pl[pl].holstered { b.marks.push("kept"); st.a.refused = Some(String::from("light: the gun never came out")); return Step::Done; }
        let sg = if along(st, st.sam) >= 0.0 { 1.0 } else { -1.0 };
        let (w2, o) = (st.width / 2.0, st.plan.as_ref().map_or(0.0, |x| x.wall.face));
        b.quiet = true;
        b.legs = vec![Leg { yaw: yaw_to(p, at(st, [-sg * w2, o, 0.0])), cap: K.gun_rate * R, min_t: -1.0, hold: 0.0 }, Leg { yaw: yaw_to(p, at(st, [sg * w2, o, 0.0])), cap: K.gun_rate * R, min_t: QK.light.sweep, hold: QK.light.hold }];
        b.leg = 0;
        b.leg_t = f64::NAN;
        to(g, b, "act");
        return Step::On;
    }
    if b.phase == "act" && b.leg >= 1 && !b.lit { b.lit = true; b.marks.push("beam"); }
    quiet_tick(g, st, b)
}
// ------------------------------------------------------------ loud kit's cost
/// does the row that just ended have one: only on the quiet picture as staged, a row loud by nature that ran to its own end, and only if it went off
pub(super) fn costly(quiet: bool, loud: bool, prelude: bool, how: &str, gun: bool, shots: u32, suppressed: bool, marked: bool) -> bool {
    if !quiet || !loud || prelude || (how != "done" && how != "cap") { return false; }
    if gun { shots > 0 && !suppressed } else { marked }
}
struct CostAt { share: f64, pool: f64, open: bool, done: bool }
fn cost_at(t: f64) -> CostAt {
    let c = &QK.cost;
    CostAt { share: if t >= c.slit { c.room } else { 1.0 }, pool: if t >= c.slit { c.pool } else { 1.0 }, open: t >= c.slit, done: t >= c.slit + c.wide_deg * R / m::max(0.1, c.swing) + c.hold }
}
/// THE COST'S LAMP: where it hangs so that the slab through the opened doorway falls along the wall over the man beside it
fn cost_lamp(g: &Game, st: &KitStage, hinge: Option<V2>) -> V3 {
    let c = &QK.cost.lamp;
    let (mid, s, n) = (st.mid, st.s, st.n);
    let (face, wall_back) = st.plan.as_ref().map_or((0.0, 0.0), |x| (x.wall.face, x.wall.back));
    let w2 = st.width / 2.0;
    let s_al = along(st, st.sam);
    let sg = if s_al >= 0.0 { 1.0 } else { -1.0 };
    let o = m::max(0.2, out_of(st, st.sam) - face);
    let th = face - wall_back;
    let leaf_far = hinge.map_or(false, |h| along(st, h) * sg < 0.0);
    let (lo, hi) = (if leaf_far { -(w2 - c.clear) } else { -(w2 + c.side) }, w2 - c.clear);
    let need = m::abs(s_al) + c.past - w2;
    let mut back = c.back;
    let mut a = m::max(lo, m::min(hi, w2 - need * (th + back) / o));
    let reach = |a: f64, back: f64| w2 + (w2 - a) * o / (th + back);
    if reach(a, back) < m::abs(s_al) + c.past - 1e-6 && need > 0.0 { back = m::max(c.back_min, m::min(c.back, (w2 - a) * o / need - th)); }
    let pos = |a: f64, back: f64| -> V3 { [mid[0] + s[0] * sg * a + n[0] * (wall_back - back), c.up, mid[1] + s[1] * sg * a + n[1] * (wall_back - back)] };
    for k in 0..12 {
        let p = pos(a, back);
        if (m::abs(a) < w2 && back < st.width) || free(g, [p[0], p[2]]) { break; }
        a *= 0.85;
        if k == 11 { a = 0.0; }
    }
    pos(a, back)
}
pub(super) fn cost_off(g: &mut Game, st: &mut KitStage) {
    st.a.cost_t = f64::NAN;
    if let Some(l) = st.a.cost_lamp.take() { crate::render_lights::light_set_remove(g, l); }
    if let (Some((i, e)), Some(p)) = (st.a.cost_pool.take(), st.pool) {
        let li = light_in();
        home::light_home_in(g, li.base, Some(p), Some(i), None, None, None);
        g.tab.sim_lightshome_fixture[p].base_emissive = e;
    }
}
pub(super) fn cost_begin(g: &mut Game, st: &mut KitStage, b: &Beat) {
    life_off(g, st, true);
    let hinge = st.door.filter(|&d| !door_off(g, d)).map(|d| g.dr[d].def.hinge);
    let p = cost_lamp(g, st, hinge);
    st.a.cost_t = g.time;
    st.a.cost_n += 1;
    st.a.cost_last = Some(CostSeen { field: b.field, i: b.i, door: hinge.is_some(), leaf: 0.0, up: false, s: 0.0, how: "", moved: 0.0, pool: false });
    st.a.cost_feet = v2(g.ch[g.pl[g.player].ch].pos);
    super::stage::quiet_inside(g, st, 1.0, QK.cost.blade);
    let c = &QK.cost.lamp;
    st.a.cost_lamp = Some(super::stage::hang_lamp(g, "kitview_cost", 0.0, p, [0.0, -1.0, 0.0], c.color, c.intensity, c.range, 0.0, 180.0, c.radius));
}
/// each tick with no row up: true while the cost plays (nothing else begins under it)
pub(super) fn cost_tick(g: &mut Game, st: &mut KitStage) -> bool {
    if st.a.cost_t.is_nan() { return false; }
    let c = &QK.cost;
    let now = g.time;
    let el = now - st.a.cost_t;
    let a = cost_at(el);
    let pl = g.player;
    let feet = g.ch[g.pl[pl].ch].pos;
    st.a.stow_t = now;
    let mut last = st.a.cost_last.unwrap_or_default();
    last.s = el;
    last.moved = m::max(last.moved, flat(feet, st.a.cost_feet));
    if st.a.asked.is_none() && !a.done {
        g.pl[pl].crouch = false;
        if a.open {
            if let Some(d) = st.door.filter(|&d| !door_off(g, d)) {
                crate::doors::door_set_locked(g, d, false);
                let f = at(st, [0.0, 1.0, 0.0]);
                let id = g.ch[g.pl[pl].ch].id;
                crate::doors::doors_crack(g, d, f, id, st.a.step_dt, None, Some(c.swing), Some(c.wide_deg * R));
            }
            if !last.up {
                super::stage::quiet_inside(g, st, a.share, 0.0);
                if let Some(p) = st.pool { if !home::fixture_broken(g, p) && a.pool < 1.0 {
                    let (i, e) = (home::fixture_base_intensity(g, p), g.tab.sim_lightshome_fixture[p].base_emissive);
                    st.a.cost_pool = Some((i, e));
                    let li = light_in();
                    home::light_home_in(g, li.base, Some(p), Some(i * a.pool), None, None, None);
                    g.tab.sim_lightshome_fixture[p].base_emissive = [e[0] * a.pool, e[1] * a.pool, e[2] * a.pool];
                    last.pool = true;
                } }
            }
            last.up = true;
        }
        if let Some(d) = st.door.filter(|&d| !door_off(g, d)) { last.leaf = m::max(last.leaf, m::abs(crate::doors::door_angle(g, d) - st.leaf_shut)); }
        st.a.cost_last = Some(last);
        return true;
    }
    if !a.done { st.a.cost_cuts += 1; }
    last.how = if a.done { "done" } else { "cut" };
    st.a.cost_last = Some(last);
    cost_off(g, st);
    super::stage::quiet_inside(g, st, QK.inside, 1.0);
    st.a.dirty = true;
    st.a.apart = true;
    st.a.rest_until = now;
    st.a.rest_next = if st.a.asked.is_some() { f64::NAN } else { c.rest };
    false
}
// ------------------------------------------------------------ life at rest
struct IdleAt { x: f64, lit: f64, env: f64, done: bool }
fn idle_at(t: f64) -> IdleAt {
    let i = &QK.idle;
    let secs = i.out * 2.0 + i.stay;
    let x = if t <= i.out { smooth(t / i.out) } else if t <= i.out + i.stay { 1.0 } else { 1.0 - smooth((t - i.out - i.stay) / i.out) };
    IdleAt { x: i.slide * x, lit: if t >= i.blink_at && t < i.blink_at + i.blink { i.blink_to } else { 1.0 }, env: m::max(0.0, m::min(m::min(1.0, t / i.fade), (secs - t) / i.fade)), done: t >= secs }
}
pub(super) fn life_off(g: &mut Game, st: &mut KitStage, cut_it: bool) {
    if st.a.life_t.is_nan() { return; }
    if cut_it { st.a.life_cuts += 1; }
    st.a.life_t = f64::NAN;
    st.a.life_still = false;
    st.a.life_next = f64::NAN;
    let Some(l) = st.a.life_lamp.take() else { return };
    if st.a.life_hung { crate::render_lights::light_set_remove(g, l); }
    else {
        let r = &mut g.tab.light[l];
        r.pos = st.a.life_home;
        r.intensity = QK.own.intensity * st.a.blade;
        r.peak_intensity = r.intensity;
    }
}
/// LIFE AT REST, with no guard: after `every` seconds of rest the warm lamp behind the door slides along the wall and back, dark for a blink, and nobody's gun wanders. Off as shipped (`idle.on`): a check raises `life_on` for itself.
pub(super) fn life(g: &mut Game, st: &mut KitStage) {
    let now = g.time;
    let busy = !st.quiet || st.a.beat.is_some() || st.a.asked.is_some() || !st.a.cost_t.is_nan() || st.veil_asked.is_some() || st.a.dirty || g.puppet.is_some() || (st.walled && !st.hole);
    if st.a.life_t.is_nan() {
        if !st.a.life_on.unwrap_or(QK.idle.on) { st.a.life_next = f64::NAN; return; }
        if busy || st.a.life_next.is_nan() { st.a.life_next = now + QK.idle.every[st.a.life_n as usize % QK.idle.every.len()]; return; }
        if now < st.a.life_next || (st.a.hi.is_some() && st.a.rest_until - now < QK.idle.out * 2.0 + QK.idle.stay + 0.5) { return; }
        let sam_p = g.ch[g.pl[g.player].ch].pos;
        if off_rest(g, st) || flat(sam_p, st.sam) > K.on_slot || away(g, st) { return; }
        let hung = st.inside.filter(|l| g.engine.lights.lights.contains(l));
        let o = &QK.own;
        st.a.life_t = now;
        st.a.life_n += 1;
        st.a.life_seen += 1;
        st.a.life_hung = hung.is_none();
        let wall_back = st.plan.as_ref().map_or(0.0, |x| x.wall.back);
        st.a.life_home = match hung { Some(l) => g.tab.light[l].pos, None => at(st, [if st.door.is_some() { st.latch * (st.width / 2.0 + o.side) } else { 0.0 }, wall_back - o.back, o.up]) };
        st.a.life_lamp = Some(match hung { Some(l) => l, None => super::stage::hang_lamp(g, "kitview_idle", 0.0, st.a.life_home, [0.0, -1.0, 0.0], o.color, 0.0, o.range, 0.0, 180.0, o.radius) });
    } else if busy { life_off(g, st, true); return; }
    let a = idle_at(now - st.a.life_t);
    let Some(l) = st.a.life_lamp.filter(|_| !a.done) else { life_off(g, st, false); return };
    let sg = if along(st, st.sam) >= 0.0 { 1.0 } else { -1.0 };
    let way = if st.door.map_or(false, |d| !door_off(g, d)) { st.latch } else { -sg };
    let h = st.a.life_home;
    let i = (if st.a.life_hung { QK.own.intensity * QK.idle.gain * a.env } else { QK.own.intensity * st.a.blade }) * a.lit;
    let r = &mut g.tab.light[l];
    r.pos = [h[0] + st.s[0] * way * a.x, h[1], h[2] + st.s[1] * way * a.x];
    r.intensity = i;
    r.peak_intensity = i;
    st.a.life_still = true;
}
